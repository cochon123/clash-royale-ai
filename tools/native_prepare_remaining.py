#!/usr/bin/env python3
"""Prepare remaining original tables discovered in a verified native checkpoint."""
import argparse,csv,hashlib,json,struct
from pathlib import Path
from native_engine_index import ROOT,EXPECTED


def sha(b):return hashlib.sha256(b).hexdigest()
def ref(n,off=0):return {'ref':n,**({'offset':hex(off)} if off else {})}
def write(p,v):
 b=(json.dumps(v,indent=2)+'\n').encode()
 with p.open('xb') as f:f.write(b)
 return sha(b)


def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--checkpoint',required=True,type=Path);p.add_argument('--typed-plan',required=True,type=Path);p.add_argument('--out',required=True,type=Path);a=p.parse_args()
 if a.out.exists():raise ValueError('Refusing to overwrite retained native plans')
 source=(ROOT/'analysis/libg-cipher/libg.decrypted.so').read_bytes()
 if sha(source)!=EXPECTED:raise ValueError('Native source mismatch')
 raw=(a.checkpoint/'checkpoint.json').read_bytes();state=json.loads(raw);current=state['call_plan']
 parent_bytes=Path(current['source']).read_bytes();parent=json.loads(parent_bytes)
 if state['source_sha256']!=EXPECTED or current['sha256']!=sha(parent_bytes) or state['failure'] or not state['returned'] or current['in_progress'] is not None or current['completed']!=len(parent['steps']):raise ValueError('Need an exact successful completed native checkpoint')
 context=current['values']['context'];region=next(r for r in state['regions'] if int(r['base'],16)<=context and context+223*8<=int(r['base'],16)+r['size'])
 memory=(a.checkpoint/region['file']).read_bytes()
 if sha(memory)!=region['sha256']:raise ValueError('Context memory hash mismatch')
 slots=struct.unpack_from('<223Q',memory,context-int(region['base'],16))
 prepared={int(name.removeprefix('native_reprepare_dynamic_table_')) for name in current['values'] if name.startswith('native_reprepare_dynamic_table_')}
 remaining=[i for i,t in enumerate(slots) if t and i not in prepared]
 if remaining!=[212,213,214,221,222]:raise ValueError(f'Unexpected unprepared original table set: {remaining}')
 fdes={int(r['start'],16):int(r['end_exclusive'],16) for r in csv.DictReader((ROOT/'analysis/native-engine-index/unwind-functions.tsv').open(),delimiter='\t')}
 steps=[]
 for i in remaining:
  name=f'remaining_prepare_table_{i}'
  steps += [{'name':name,'read_u64':ref('context',i*8)},{'name':f'native_reprepare_dynamic_table_{i}','rva':'0xe12548','args':[ref(name)]}]
 plan={'source_sha256':EXPECTED,'inherit_plan_sha256':current['sha256'],'purpose':'Complete original e6df50 per-table preparation for all five non-catalog tables before typed postload',
       'generator_sha256':sha(Path(__file__).read_bytes()),'checkpoint_evidence':{'path':str(a.checkpoint.resolve()),'checkpoint_sha256':sha(raw),'context':hex(context),'region_sha256':region['sha256'],'present_table_count':sum(bool(x) for x in slots),'previously_prepared_table_count':len(prepared),'remaining':remaining},
       'native_api_evidence':[{'rva':hex(r),'end_rva_exclusive':hex(fdes[r]),'source_bytes_sha256':sha(source[r:fdes[r]])} for r in (0xe12548,0xe6df50)],'steps':steps,'inspect':[{'address':ref('context'),'size':'0x10a8'}],
       'source_contract':'e6df50 iterates every nonnull context table and calls e12548 before e16984. The prefix prepared 159 catalog/dependency tables; the original dynamic/post-attachment functions created these five further tables. e12548 resolves names within its own table; this continuation runs it only for the unprepared tables.'}
 a.out.mkdir(parents=True);path=a.out/'remaining-prepare-plan.json';identity=write(path,plan)
 typed_bytes=a.typed_plan.read_bytes();typed=json.loads(typed_bytes)
 if typed['source_sha256']!=EXPECTED or typed.get('inherit_plan_sha256')!=current['sha256'] or typed['steps'][0].get('rva')!='0xe16984':raise ValueError('Typed continuation does not follow this checkpoint')
 typed['inherit_plan_sha256']=identity;typed['inserted_preparation']={'path':str(path.resolve()),'sha256':identity,'original_typed_plan':str(a.typed_plan.resolve()),'original_typed_plan_sha256':sha(typed_bytes)}
 typed_path=a.out/'resource-typed-plan.json';typed_identity=write(typed_path,typed)
 result={'source_sha256':EXPECTED,'remaining_table_ids':remaining,'prepare_plan':{'path':str(path.resolve()),'sha256':identity},'typed_plan':{'path':str(typed_path.resolve()),'sha256':typed_identity},'native_execution_performed':False};write(a.out/'generation-report.json',result);print(json.dumps(result,indent=2))

if __name__=='__main__':main()
