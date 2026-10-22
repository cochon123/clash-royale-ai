#!/usr/bin/env python3
"""Extract original protected bindings and decode selected IDs with the original native codec.

No game guard is patched. Wire extraction does not verify the envelope signature.
The native decoder executes in the existing bounded Unicorn analysis environment.
"""
import argparse, ast, concurrent.futures, hashlib, json, pathlib, re, shutil, struct, subprocess, sys, tempfile, time, zipfile
from native_recover_metadata import BASE_SHA256, ASSET_SHA256, fields, unique

ROOT=pathlib.Path(__file__).resolve().parents[1]

def sha(data):return hashlib.sha256(data).hexdigest()

def extract(apk, output):
    package=apk.read_bytes()
    if sha(package)!=BASE_SHA256:raise ValueError('Supplied base APK identity mismatch')
    member='assets/anjeamelnchn.dat'
    with zipfile.ZipFile(apk) as archive:asset=archive.read(member)
    if sha(asset)!=ASSET_SHA256 or asset[:4]!=b'lbts':raise ValueError('Protected asset identity mismatch')
    blocks={};position=4
    while position<len(asset):
        tag,length=struct.unpack_from('<II',asset,position);position+=8
        if tag in blocks or position+length>len(asset):raise ValueError('Invalid protected envelope')
        blocks[tag]=(position,asset[position:position+length]);position+=length
    origin,signed=blocks[0x30]
    unsigned=unique(fields(signed),1,2)
    entries=[];ids=set()
    for outer in fields(signed,unsigned['data'],unsigned['end']):
        if outer['field']!=10 or outer['wire']!=2:continue
        inner=fields(signed,outer['data'],outer['end'])
        identifier=unique(inner,1,0)['value']
        if identifier in ids:raise ValueError(f'Duplicate binding ID {identifier:x}')
        ids.add(identifier)
        binding=unique(inner,2,2)
        ciphertext=unique(fields(signed,binding['data'],binding['end']),1,2)
        directory=output/'bindings'/f'{identifier:08x}';directory.mkdir(parents=True,exist_ok=True)
        artifacts={}
        for name,start,end in [('record',outer['data'],outer['end']),('binding',binding['data'],binding['end']),('ciphertext',ciphertext['data'],ciphertext['end'])]:
            content=signed[start:end];path=directory/(name+'.bin');path.write_bytes(content)
            artifacts[name]={'path':str(path.resolve()),'size':len(content),'sha256':sha(content),'block30_offset':start,'asset_offset':origin+start}
        entries.append({'id':hex(identifier),'artifacts':artifacts})
    catalog={'source_apk':str(apk.resolve()),'source_apk_sha256':sha(package),'member':member,'asset_sha256':sha(asset),'script_sha256':sha(pathlib.Path(__file__).read_bytes()),'wire_parser_sha256':sha((ROOT/'tools/native_recover_metadata.py').read_bytes()),'binding_records':len(entries),'records':entries,'limits':['Wire extraction preserves original offsets and bytes; it does not validate the signed envelope signature.','IDs and ciphertext are original records; the higher-level meaning of decoded fields is not inferred here.']}
    output.mkdir(parents=True,exist_ok=True)
    (output/'catalog.json').write_text(json.dumps(catalog,indent=2)+'\n')
    return catalog

def freeze_decoder(output):
    unpack=ROOT/'tools/native_unpack.py';content=unpack.read_bytes();identity=sha(content)
    snapshots=output/'decoder-sources';snapshots.mkdir(exist_ok=True)
    snapshot=snapshots/(identity+'.py')
    if snapshot.exists():
        if sha(snapshot.read_bytes())!=identity:raise ValueError('Frozen decoder identity mismatch')
    else:snapshot.write_bytes(content)
    helpers={};pending=[content]
    while pending:
        tree=ast.parse(pending.pop())
        modules=[]
        for node in ast.walk(tree):
            if isinstance(node,ast.Import):modules.extend(alias.name.split('.')[0] for alias in node.names)
            elif isinstance(node,ast.ImportFrom) and node.module:modules.append(node.module.split('.')[0])
        for module in modules:
            local=ROOT/'tools'/(module+'.py')
            if not local.is_file() or module in helpers:continue
            value=local.read_bytes();helpers[module]={'source':str(local),'sha256':sha(value),'content':value};pending.append(value)
    identifiers={module:entry['sha256'] for module,entry in sorted(helpers.items())}
    bundle_identity=sha(json.dumps({'main':identity,'helpers':identifiers},sort_keys=True).encode())
    bundle=snapshots/'bundles'/bundle_identity;bundle.mkdir(parents=True,exist_ok=True)
    records={}
    for module,entry in helpers.items():
        frozen=bundle/(module+'.py')
        if frozen.exists() and sha(frozen.read_bytes())!=entry['sha256']:raise ValueError('Frozen helper identity mismatch')
        if not frozen.exists():frozen.write_bytes(entry['content'])
        records[module]={'source':entry['source'],'path':str(frozen.resolve()),'sha256':entry['sha256']}
    manifest={'runner':str(snapshot.resolve()),'runner_sha256':identity,'helpers':records}
    manifest_path=bundle/'manifest.json';manifest_path.write_text(json.dumps(manifest,indent=2)+'\n')
    return snapshot,identity,manifest_path

def decode(entry,output,companion,python,snapshot,unpack_sha,bundle_manifest):
    identifier=int(entry['id'],16)&0xffffffff;decoded_dir=output/'decoded';reports=output/'decoder-reports'
    decoded_dir.mkdir(exist_ok=True);reports.mkdir(exist_ok=True)
    unpack=ROOT/'tools/native_unpack.py'
    with tempfile.TemporaryDirectory(prefix=f'clash-binding-{identifier:08x}-') as temporary:
        # Execute the retained bytes with the original __file__ so project-local
        # source resolution stays identical while the root agent edits its runner.
        wrapper=f"import pathlib,sys;sys.path.insert(0,{str(ROOT/'tools')!r});sys.path.insert(0,{str(bundle_manifest.parent.resolve())!r});sys.argv=[{str(unpack)!r}]+sys.argv[1:];exec(compile(pathlib.Path({str(snapshot.resolve())!r}).read_bytes(),{str(unpack)!r},'exec'),{{'__name__':'__main__','__file__':{str(unpack)!r}}})"
        command=[str(python),'-c',wrapper,str(ROOT/'analysis/native/lib/arm64-v8a/libg.so'),'--out',temporary,
            '--companion-memory',str(companion),'--companion-init','0x2b95dc','--companion-init-only','--companion-sret',
            '--decode-binding',entry['artifacts']['ciphertext']['path'],'--timeout','30','--instructions','50000000']
        process=subprocess.run(command,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True)
        report_path=pathlib.Path(temporary)/'report.json'
        if not report_path.exists():raise RuntimeError('Native decoder did not produce report: '+process.stdout[-2000:])
        report=json.loads(report_path.read_text())
        report_destination=reports/f'{identifier:08x}.json'
        shutil.copy2(report_path,report_destination)
        if report.get('failure') or not report.get('binding_decode',{}).get('returned'):
            raise RuntimeError(f"Native decode for {identifier:x} failed: {report.get('failure')}")
        decoded=pathlib.Path(temporary)/'binding-decoded.bin'
        destination=decoded_dir/f'{identifier:08x}.bin';shutil.copy2(decoded,destination)
        result={'id':hex(identifier),'wire_id':entry['id'],'path':str(destination.resolve()),'size':destination.stat().st_size,'sha256':sha(destination.read_bytes()),
            'ciphertext':entry['artifacts']['ciphertext'],'decoder':'Original companion codec RVA2170b0 after original codec constructor RVA2b95dc',
            'native_unpack_sha256':unpack_sha,'native_unpack_snapshot':str(snapshot.resolve()),'decoder_bundle_manifest':str(bundle_manifest.resolve()),'decoder_bundle_manifest_sha256':sha(bundle_manifest.read_bytes()),'companion_snapshot_sha256':sha(companion.read_bytes()),'report':str(report_destination.resolve()),'report_sha256':sha(report_destination.read_bytes()),
            'command':command,'process_exitcode':process.returncode,'native_decoder_returned':True,
            'limits':['The isolated native codec executes with recorded imports/syscalls; Android startup and binding lookup registration are not run.','Large emulator memory dumps are temporary; the original decode report, input bytes and decoded result are retained.']}
        if sha(snapshot.read_bytes())!=unpack_sha:raise RuntimeError('Frozen native decoder script changed during this decode')
        proof=reports/f'{identifier:08x}-provenance.json';temporary_proof=proof.with_suffix('.tmp')
        temporary_proof.write_text(json.dumps(result,indent=2)+'\n');temporary_proof.replace(proof)
        return result

def write_index(output,catalog):
    bindings={}
    for path in sorted((output/'decoder-reports').glob('*-provenance.json')):
        report=json.loads(path.read_text());decoded=pathlib.Path(report['path'])
        if decoded.stat().st_size!=report['size'] or sha(decoded.read_bytes())!=report['sha256']:
            raise ValueError('Retained decoded binding identity mismatch')
        bindings[report['id']]={key:report[key] for key in ('path','size','sha256','report')}
        bindings[report['id']]['ciphertext_sha256']=report['ciphertext']['sha256']
        bindings[report['id']]['provenance']=str(path.resolve())
    known=ROOT/'analysis/native-unpack/companion-binding-decoded/report.json'
    if known.exists():
        report=json.loads(known.read_text());decoded=ROOT/'analysis/native-unpack/companion-binding-decoded/binding-decoded.bin'
        entry=next(entry for entry in catalog['records'] if entry['id']=='0x7e33eb5e')
        if report.get('failure') is not None or not report['binding_decode']['returned']:
            raise ValueError('Existing known native binding decoder did not succeed')
        if sha(decoded.read_bytes())!=report['binding_decode']['decoded_sha256'] or sha(decoded.read_bytes())!='29f73c4325b635d46233ca040c0048cad83570b7cad0fb456099785140f1bb83':
            raise ValueError('Existing known native binding identity mismatch')
        if entry['artifacts']['ciphertext']['sha256']!=report['binding_decode']['source_sha256']:
            raise ValueError('Known decoded binding is not tied to this original catalog ciphertext')
        bindings['0x7e33eb5e']={'path':str(decoded.resolve()),'size':decoded.stat().st_size,'sha256':sha(decoded.read_bytes()),'ciphertext_sha256':entry['artifacts']['ciphertext']['sha256'],'report':str(known.resolve()),'report_sha256':sha(known.read_bytes())}
    result={'source_asset_sha256':catalog['asset_sha256'],'source_apk_sha256':catalog['source_apk_sha256'],'catalog':str((output/'catalog.json').resolve()),'bindings':bindings}
    with tempfile.NamedTemporaryFile(mode='w',prefix='binding-index-',suffix='.tmp',dir=output,delete=False) as file:
        file.write(json.dumps(result,indent=2)+'\n');temporary=pathlib.Path(file.name)
    temporary.replace(output/'binding-index.json')
    return result

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--apk',type=pathlib.Path,default=ROOT/'analysis/apks/com.supercell.clashroyale.apk')
    parser.add_argument('--output',type=pathlib.Path,default=ROOT/'analysis/native-init-array/protection-investigation')
    parser.add_argument('--decode-id',type=lambda value:int(value,16),action='append',default=[])
    parser.add_argument('--decode-all',action='store_true')
    parser.add_argument('--reextract',action='store_true',help='Regenerate the byte-for-byte extraction; default reuses a verified existing catalog')
    parser.add_argument('--jobs',type=int,default=4,help='Independent native decoder subprocesses, default 4')
    parser.add_argument('--companion-memory',type=pathlib.Path,default=ROOT/'analysis/native-entry/companion-text-decrypt/companion-decrypted-memory.bin')
    parser.add_argument('--python',type=pathlib.Path,default=ROOT/'tools/reverse-env/bin/python')
    args=parser.parse_args();args.output.mkdir(parents=True,exist_ok=True)
    catalog_path=args.output/'catalog.json'
    if catalog_path.exists() and not args.reextract:
        catalog=json.loads(catalog_path.read_text())
        if catalog['source_apk_sha256']!=BASE_SHA256 or catalog['asset_sha256']!=ASSET_SHA256:
            raise ValueError('Existing catalog source identities differ')
    else:catalog=extract(args.apk,args.output)
    # The original API consumes W0/uint32. Negative protobuf int32 IDs use
    # sign-extended uint64 varints; preserve wire IDs but normalize lookup keys.
    available={int(entry['id'],16)&0xffffffff:entry for entry in catalog['records']}
    if len(available)!=len(catalog['records']):raise ValueError('Ambiguous uint32 binding ID normalization')
    requested=set(available) if args.decode_all else {identifier&0xffffffff for identifier in args.decode_id}
    if requested-set(available):raise ValueError(f'Absent original binding IDs: {requested-set(available)}')
    if not 1<=args.jobs<=16:raise ValueError('Use between 1 and 16 decoder jobs')
    known=write_index(args.output,catalog)['bindings'];pending=[]
    for identifier in sorted(requested):
        ciphertext=pathlib.Path(available[identifier]['artifacts']['ciphertext']['path'])
        if sha(ciphertext.read_bytes())!=available[identifier]['artifacts']['ciphertext']['sha256']:
            raise ValueError('Catalog ciphertext identity mismatch')
        if hex(identifier) not in known:pending.append(identifier)
    snapshot,identity,bundle_manifest=freeze_decoder(args.output)
    progress={'requested':len(requested),'cached':len(requested)-len(pending),'pending':len(pending),'completed_now':0,'failures':[],'decoder_snapshot':str(snapshot.resolve()),'decoder_sha256':identity,'decoder_bundle_manifest':str(bundle_manifest.resolve()),'jobs':args.jobs}
    started=time.monotonic()
    def record_progress():
        progress['elapsed_seconds']=time.monotonic()-started
        (args.output/'batch-progress.json').write_text(json.dumps(progress,indent=2)+'\n')
    record_progress()
    def next_pending():
        # Prioritize only lookup IDs actually observed in original execution.
        reports=sorted((ROOT/'analysis/native-init-array').glob('with-companion-*/report.json'),key=lambda path:path.stat().st_mtime_ns,reverse=True)
        for report_path in reports:
            report=json.loads(report_path.read_text())
            match=re.search(r'Unexpected recovered metadata lookup key\s+(?:0x)?([0-9a-fA-F]+)',report.get('failure') or '')
            if match:
                identifier=int(match.group(1),16)&0xffffffff
                if identifier in pending:
                    pending.remove(identifier)
                    print(json.dumps({'prioritized_observed_id':hex(identifier),'report':str(report_path)}),flush=True)
                    return identifier
        return pending.pop(0)
    with concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as pool:
        futures={}
        while pending or futures:
            while pending and len(futures)<args.jobs:
                identifier=next_pending()
                futures[pool.submit(decode,available[identifier],args.output,args.companion_memory,args.python,snapshot,identity,bundle_manifest)]=identifier
            completed,_=concurrent.futures.wait(futures,return_when=concurrent.futures.FIRST_COMPLETED)
            for future in completed:
                identifier=futures.pop(future)
                try:
                    result=future.result();progress['completed_now']+=1
                    write_index(args.output,catalog)
                    print(json.dumps({key:result[key] for key in ('id','path','size','sha256')}),flush=True)
                except Exception as error:
                    progress['failures'].append({'id':hex(identifier),'error':str(error)})
                    print(json.dumps({'id':hex(identifier),'error':str(error)}),flush=True)
                record_progress()
    write_index(args.output,catalog)
    print(json.dumps({'catalog':str(args.output/'catalog.json'),'original_binding_records':catalog['binding_records'],'decoded_now':progress['completed_now'],'cached':progress['cached'],'failures':len(progress['failures'])}))
    if progress['failures']:raise SystemExit(1)

if __name__=='__main__':main()
