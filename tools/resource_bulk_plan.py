#!/usr/bin/env python3
"""Generate native asset parsing, typed table attachment and original postload calls.

Original CSV/TOML/JSON contents go through the recovered game parsers. Only the
resource manager delivery of parsed roots is adapted by one exact native store.
The two language resources do not attach to the engine context and are recorded
as excluded; no catalog descriptor is silently omitted.
"""
import argparse,collections,csv,hashlib,json,zipfile
from pathlib import Path
from native_engine_index import ROOT,EXPECTED
ASSET_PACK_SHA='3e62afcf73786919be08b0a48f1744d82b618642fed8e8db7d1a76b5b2eb3fd3'

def make(out,postload=True,limit=None):
 elf=ROOT/'analysis/libg-cipher/libg.decrypted.so';b=elf.read_bytes()
 if hashlib.sha256(b).hexdigest()!=EXPECTED:raise ValueError('Recovered ELF digest mismatch')
 catalog_path=ROOT/'analysis/native-focused/resources/catalog-descriptors.json';catalog=json.loads(catalog_path.read_text());records=catalog['records']
 if any(r['table_id'] is None or r['flags'] is None for r in records):raise ValueError('Unknown native catalog scalar')
 if b[0xd19b78:0xd19b80]!=bytes.fromhex('010000f9c0035fd6'):raise ValueError('Input pointer-store native instructions mismatch')
 steps=[];assets=[];loaded=[];excluded=[];helpers=set();primary=set();special_counts=collections.Counter()
 def ref(name,offset=0):return {'ref':name,**({'offset':hex(offset)} if offset else {})}
 def buf(name,size=None,data=None,file=None):
  step={'name':name}
  if size is not None:step['size']=hex(size)
  if data is not None:step['data_hex']=data.hex()
  if file is not None:step.update(file=str(file.resolve()),sha256=hashlib.sha256(file.read_bytes()).hexdigest(),nul_terminate=True)
  steps.append(step);return ref(name)
 def call(name,rva,args,nonnull=False):
  helpers.add(rva);steps.append({'name':name,'rva':hex(rva),'args':args,**({'expect_nonzero':True} if nonnull else {})});return ref(name)
 def read(name,address):steps.append({'name':name,'read_u64':address});return ref(name)
 ctx=call('context',0xe160c0,[],True)
 call('prepare_native_context',0xe15dd0,[ctx]);call('clear_native_context_tables',0xe158e0,[ctx])
 pack=None
 def asset_path(path):
  nonlocal pack
  p=ROOT/'analysis/decoded/assets'/path
  if p.exists():return p,None
  # These plain UTF8 JSON resources were not part of the earlier CSV/TOML decoder.
  if path not in ('legacy_vfx/effects.json','legacy_vfx/emitters.json'):raise ValueError('Unrecovered asset '+path)
  archive=ROOT/'analysis/apks/install_time_asset_pack.apk'
  if pack is None:
   digest=hashlib.sha256(archive.read_bytes()).hexdigest()
   if digest!=ASSET_PACK_SHA:raise ValueError('Original asset pack SHA mismatch')
   pack=zipfile.ZipFile(archive)
  data=pack.read('assets/'+path);json.loads(data)
  p=ROOT/'analysis/native-focused/resources/inputs'/path;p.parent.mkdir(exist_ok=True,parents=True)
  if p.exists() and p.read_bytes()!=data:raise ValueError('Existing extracted JSON differs')
  if not p.exists():p.write_bytes(data)
  return p,{'archive':str(archive),'archive_sha256':ASSET_PACK_SHA,'member':'assets/'+path,'member_sha256':hashlib.sha256(data).hexdigest(),'encoding':'original plain UTF8 JSON, unmodified'}
 for index,record in enumerate(records):
  tid,flag=record['table_id'],record['flags'];prefix=f'asset_{index:03d}'
  if flag==1:
   excluded.append({'catalog_index':index,'table_id':tid,'flags':flag,'paths':record['paths'],'reason':'Native localization resources follow separate global UI language APIs; they do not attach to native battle context.'});continue
  if limit is not None and len(loaded)>=limit:
   excluded.append({'catalog_index':index,'table_id':tid,'flags':flag,'reason':'Explicit bounded probe limit'});continue
  if flag not in (0,2,3,5):raise ValueError('Unimplemented observed native descriptor flag '+str(flag))
  paths=record['paths']
  if flag!=5 and (not paths or any(not p.get('path') for p in paths)):raise ValueError('Unresolved native resource paths')
  if len(paths)>2:raise ValueError('Unobserved descriptor path count')
  names=[];parsed=[]
  for j,path_record in enumerate(paths):
   path=path_record['path'];f,extraction=asset_path(path);data=f.read_bytes();kind=Path(path).suffix.lower();part=prefix+f'_file{j}'
   if kind not in ('.csv','.toml','.json'):raise ValueError('Unmapped asset parser format '+kind)
   names.append(buf(part+'_name_bytes',data=path.encode()+b'\0'));name=buf(part+'_name_string',size=16)
   call(part+'_construct_name',0x142fde4,[name,names[-1]]);names[-1]=name
   raw=buf(part+'_bytes',file=f)
   if kind=='.csv':
    node=buf(part+'_csv_root',size=0x68);call(part+'_parse_csv',0x1288f2c,[node,raw,len(data),name]);call(part+'_csv_table',0x1289a04,[node],True);parsed.append((node,0x10))
   else:
    text=buf(part+'_text_string',size=16);call(part+'_construct_text',0x142fdc4,[text,raw,len(data)])
    if kind=='.toml':root=call(part+'_parse_toml',0x12a6e64,[text,0],True)
    else:root=call(part+'_parse_json',0x1292230,[text,0,100],True)
    root=call(part+'_require_json_object',0x128f5c0,[root,1],True);parsed.append((root,0x18))
   assets.append({'catalog_index':index,'table_id':tid,'flags':flag,'path':path,'decoded_file':str(f.resolve()),'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest(),**({'extraction':extraction} if extraction else {})})
  descriptor=buf(prefix+'_descriptor',size=0x20)
  if flag==5:call(prefix+'_construct_empty_descriptor',0xe14450,[descriptor,tid,flag])
  elif len(names)==1:call(prefix+'_construct_descriptor',0xe14440,[descriptor,names[0],tid,flag])
  else:call(prefix+'_construct_paired_descriptor',0xe14408,[descriptor,names[0],names[1],tid,flag])
  if paths:
   entries=read(prefix+'_entries',ref(prefix+'_descriptor',8))
   for j,(root,offset) in enumerate(parsed):
    entry=read(prefix+f'_entry{j}',ref(prefix+'_entries',j*8))
    call(prefix+f'_deliver_parsed_input{j}',0xd19b78,[ref(prefix+f'_entry{j}',offset),root])
  if flag in (0,5):
   call(prefix+'_attach_original_table',0xe15e64,[ctx,descriptor,tid]);primary.add(tid)
  elif flag==2:
   # Exact native e6df50 branch: first type2 resource initializes, subsequent extends.
   api=0xe16468 if special_counts[flag]==0 else 0xe16478;call(prefix+'_load_global_ids',api,[ctx,descriptor]);call(prefix+'_release_resource_root',0xe146fc,[descriptor])
  elif flag==3:
   call(prefix+'_load_game_tags',0xe16234,[ctx,descriptor]);call(prefix+'_release_resource_root',0xe146fc,[descriptor])
  special_counts[flag]+=1;loaded.append({'catalog_index':index,'table_id':tid,'flags':flag,'path_count':len(paths)})
 if pack is not None:pack.close()
 if postload and limit is not None:raise ValueError('Full native postload is not authorized for an explicitly partial catalog probe')
 if postload:
  call('native_post_attach_stage',0xe6e3ec,[ctx])
  call('native_build_dependency_tables',0xe16858,[ctx])
  # Source e15180/e16890 constructs these exact six dependency tables.
  for tid in sorted(primary|{6,7,49,97,107,169}):
   table=read(f'pre_finalize_table_{tid}',ref('context',tid*8));call(f'native_table_prepare_{tid}',0xe12548,[table])
  call('native_complete_context_postload',0xe1698c,[ctx])
  for name,lookup,tid in [('Normal',0xe1b7e8,121),('Friendly',0xe191c4,72)]:
   if tid not in primary:continue
   raw=buf('lookup_'+name+'_bytes',data=name.encode()+b'\0');string=buf('lookup_'+name+'_string',size=16);call('construct_lookup_'+name,0x142fde4,[string,raw]);call('lookup_'+name,lookup,[ctx,string,0],True)
 sizes={int(x['start'],16):int(x['size']) for x in csv.DictReader((ROOT/'analysis/native-engine-index/unwind-functions.tsv').open(),delimiter='\t')}
 for a in helpers:
  if a not in sizes:raise ValueError('API not exact original FDE start '+hex(a))
 proof=[{'rva':hex(a),'exact_FDE_size':sizes[a],'source_file_bytes_sha256':hashlib.sha256(b[a:a+sizes[a]]).hexdigest()} for a in sorted(helpers)]
 plan={'source_sha256':EXPECTED,'purpose':'Original native CSV/TOML/JSON parsers, native typed table construction, native dependency tables and full original context postload','catalog':{'source':str(catalog_path),'sha256':hashlib.sha256(catalog_path.read_bytes()).hexdigest(),'original_descriptor_count':len(records),'loaded_engine_descriptor_count':len(loaded),'loaded_descriptors':loaded,'excluded_descriptors':excluded},'input_adapter':'Only parsed-root delivery is adapted by source-verified d19b78 STRX1,[X0];RET into the original native resource-file entry at+10(CSV) or+18(TOML/JSON). No rows or typed engine objects are synthesized.','assets':assets,'native_api_evidence':proof,'steps':steps,'inspect':[{'address':ctx,'size':'0x10a8'}], 'limitations':['Execution and complete typed postload must succeed before claiming the native context is ready.','Two native language/localization resources are explicitly excluded from this battle-context plan.','SCDB dynamic extra resources beyond the two fixed native catalog builders are outside this plan.']}
 if postload:plan['inspect'] += [{'address':{'ref':'lookup_Normal'},'size':'0xa0'},{'address':{'ref':'lookup_Friendly'},'size':'0x100'}]
 out.parent.mkdir(exist_ok=True,parents=True);out.write_text(json.dumps(plan,indent=2)+'\n');print(json.dumps({'plan':str(out),'sha256':hashlib.sha256(out.read_bytes()).hexdigest(),'steps':len(steps),'assets':len(assets),'asset_bytes':sum(a['bytes'] for a in assets),'engine_descriptors':len(loaded),'excluded':len(excluded),'full_postload':postload},indent=2))
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--out',type=Path,default=ROOT/'analysis/native-focused/resources/bulk-native-assets-plan.json');p.add_argument('--no-postload',action='store_true');p.add_argument('--limit',type=int);a=p.parse_args();make(a.out,not a.no_postload,a.limit)
