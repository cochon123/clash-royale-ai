#!/usr/bin/env python3
"""Create an immutable continuation plan for original fully typed context postload."""
import argparse,hashlib,json
from pathlib import Path
from native_engine_index import ROOT,EXPECTED

def make(prefix,out):
 if out.exists():raise ValueError('Choose an absent output path; active plans remain immutable')
 p=json.loads(prefix.read_text())
 if p['source_sha256']!=EXPECTED:raise ValueError('Inherited plan source mismatch')
 source=ROOT/'analysis/libg-cipher/libg.decrypted.so';b=source.read_bytes()
 if hashlib.sha256(b).hexdigest()!=EXPECTED:raise ValueError('Native ELF digest mismatch')
 # This original wrapper explicitly supplies the flag tested by e1698c.
 expected=bytes.fromhex('2100805201000014')
 if b[0xe16984:0xe1698c]!=expected:raise ValueError('Native typed-postload wrapper instruction mismatch')
 def ref(name,offset=0):return {'ref':name,**({'offset':hex(offset)} if offset else {})}
 result={'source_sha256':EXPECTED,'inherit_plan_sha256':hashlib.sha256(prefix.read_bytes()).hexdigest(),'purpose':'Original native typed postload after the immutable bulk asset plan','native_api_evidence':[{'rva':'0xe16984','bytes':expected.hex(),'instructions':['MOV W1,#1','B 0xe1698c'],'meaning':'Sets the original postload flag required for table virtual+58/+68/+70 typed phases.'}],'steps':[{'name':'complete_typed_context_postload','rva':'0xe16984','args':[ref('context')]},{'name':'typed_testing_table','read_u64':ref('context',0x3c8)},{'name':'typed_lookup_Normal','rva':'0xe1b7e8','args':[ref('context'),ref('lookup_Normal_string'),0],'expect_nonzero':True},{'name':'typed_lookup_Friendly','rva':'0xe191c4','args':[ref('context'),ref('lookup_Friendly_string'),0],'expect_nonzero':True}],'inspect':[{'address':ref('context'),'size':'0x10a8'},{'address':ref('typed_testing_table'),'size':'0x98'},{'address':ref('typed_lookup_Normal'),'size':'0xa0'},{'address':ref('typed_lookup_Friendly'),'size':'0x100'}],'limitations':['Run only after the inherited bulk plan has completed with no in-progress step.','Complete native postload success is required; lookup alone does not prove typed fields are populated.','Arena tilemap cache population remains separate from this fixed CSV/TOML/JSON catalog.']}
 out.parent.mkdir(exist_ok=True,parents=True);out.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'plan':str(out),'sha256':hashlib.sha256(out.read_bytes()).hexdigest(),'inherits':result['inherit_plan_sha256'],'context_key':'context'},indent=2))
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--prefix',type=Path,default=ROOT/'analysis/native-focused/resources/bulk-native-assets-plan.json');p.add_argument('--out',type=Path,default=ROOT/'analysis/native-focused/resources/bulk-native-assets-typed-tail.json');a=p.parse_args();make(a.prefix,a.out)
