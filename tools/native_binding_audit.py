#!/usr/bin/env python3
"""Cross-check retained decoded bindings against original wire records and native execution reports."""
import argparse, collections, hashlib, json, pathlib

ROOT=pathlib.Path(__file__).resolve().parents[1]
ASSET='834217feba3588c1f4d28e8dcf2a309c9baa1903eb5c08f535a628b50751d60b'
COMPANION='714b5e3979b263917a1b85c029b0a84d07c515404f1085b1c27f41bed55d4b56'

def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--directory',type=pathlib.Path,default=ROOT/'analysis/native-init-array/protection-investigation')
    parser.add_argument('--require-all',action='store_true')
    args=parser.parse_args();catalog_path=args.directory/'catalog.json';index_path=args.directory/'binding-index.json'
    catalog=json.loads(catalog_path.read_text());index=json.loads(index_path.read_text())
    if catalog['asset_sha256']!=ASSET or index['source_asset_sha256']!=ASSET:raise ValueError('Source asset identity mismatch')
    originals={hex(int(entry['id'],16)&0xffffffff):entry for entry in catalog['records']}
    if len(originals)!=len(catalog['records']):raise ValueError('Original normalized IDs are ambiguous')
    failures=[];sizes=collections.Counter();verified=[]
    for identifier,entry in index['bindings'].items():
        try:
            original=originals[identifier]['artifacts']['ciphertext'];path=pathlib.Path(entry['path']);report_path=pathlib.Path(entry['report']);report=json.loads(report_path.read_text())
            if sha(path)!=entry['sha256'] or path.stat().st_size!=entry['size']:raise ValueError('Decoded bytes identity mismatch')
            if entry['ciphertext_sha256']!=original['sha256'] or sha(pathlib.Path(original['path']))!=original['sha256']:raise ValueError('Original ciphertext identity mismatch')
            decode=report['binding_decode']
            if report['failure'] is not None or not decode['returned']:raise ValueError('Native decoder did not return successfully')
            if decode['source_sha256']!=original['sha256']:raise ValueError('Native report input differs from original ciphertext')
            if decode['decoded_sha256']!=entry['sha256'] or decode['decoded_size']!=entry['size']:raise ValueError('Native report output differs from retained bytes')
            if decode['entry_rva']!='0x2170b0' or report['companion']['sha256']!=COMPANION:raise ValueError('Wrong original native codec or companion snapshot')
            if not any(stage.get('entry')=='0x2b95dc' and stage.get('returned') for stage in report['stages']):raise ValueError('Original codec constructor did not return')
            if 'provenance' in entry:
                proof=json.loads(pathlib.Path(entry['provenance']).read_text())
                if proof['report_sha256']!=sha(report_path):raise ValueError('Retained report identity mismatch')
                if proof.get('native_unpack_snapshot') and sha(pathlib.Path(proof['native_unpack_snapshot']))!=proof['native_unpack_sha256']:raise ValueError('Retained decoder source identity mismatch')
                if proof.get('decoder_bundle_manifest'):
                    bundle_path=pathlib.Path(proof['decoder_bundle_manifest'])
                    if sha(bundle_path)!=proof['decoder_bundle_manifest_sha256']:raise ValueError('Frozen decoder bundle identity mismatch')
                    bundle=json.loads(bundle_path.read_text())
                    if bundle['runner_sha256']!=proof['native_unpack_sha256']:raise ValueError('Frozen bundle runner identity mismatch')
                    for helper in bundle['helpers'].values():
                        if sha(pathlib.Path(helper['path']))!=helper['sha256']:raise ValueError('Frozen helper identity mismatch')
                if (int(proof.get('wire_id',identifier),16)&0xffffffff)!=int(identifier,16):raise ValueError('Wire/native ID normalization mismatch')
            sizes[entry['size']]+=1;verified.append(identifier)
        except Exception as error:failures.append({'id':identifier,'error':str(error)})
    missing=sorted(set(originals)-set(index['bindings']),key=lambda value:int(value,16))
    result={'source_asset_sha256':ASSET,'catalog_sha256':sha(catalog_path),'binding_index_sha256':sha(index_path),'audit_script_sha256':sha(pathlib.Path(__file__)),'original_bindings':len(originals),'indexed_bindings':len(index['bindings']),'verified_bindings':len(verified),'all_original_bindings_verified':not missing and not failures,'decoded_size_distribution':dict(sorted(sizes.items())),'failures':failures,'missing_ids':missing,'limits':['This audit ties original encrypted records to retained outputs from the original companion decoder.','It does not verify the signed asset envelope signature or establish that native game initialization or a match completed.']}
    (args.directory/'binding-audit.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({key:value for key,value in result.items() if key not in ('missing_ids','limits')},indent=2))
    if failures or (args.require_all and missing):raise SystemExit(1)

if __name__=='__main__':main()
