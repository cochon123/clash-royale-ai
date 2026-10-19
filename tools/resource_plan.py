#!/usr/bin/env python3
"""Build a source-bound native CSV parse/attach/lookup call plan.

All CSV rows and native table objects are constructed by original game code.
A one-pointer input adapter binds the parsed CSV root into the original resource
file entry, replacing only the external file/resource-manager delivery step.
"""
import argparse,csv,hashlib,json,struct
from pathlib import Path
from native_engine_index import ROOT,EXPECTED

def make(out,table_id,filename,row,context):
 source=ROOT/'analysis/libg-cipher/libg.decrypted.so';b=source.read_bytes()
 if hashlib.sha256(b).hexdigest()!=EXPECTED:raise ValueError('Native source SHA256 mismatch')
 catalog=json.loads((ROOT/'analysis/native-focused/resources/catalog-descriptors.json').read_text())
 if not any(r['table_id']==table_id and any(p.get('path')==filename for p in r['paths']) for r in catalog['records']):raise ValueError('Filename/tableID pair not observed in original native catalog')
 asset=(ROOT/'analysis/decoded/assets'/filename).resolve();data=asset.read_bytes();digest=hashlib.sha256(data).hexdigest()
 if not filename.endswith('.csv'):raise ValueError('This generator currently accepts the recovered native CSV parser only')
 parsed=list(csv.reader(data.decode('utf-8-sig').splitlines()))
 names=[x[0] for x in parsed[2:] if x and x[0]]
 if row not in names:raise ValueError('Requested lookup name is absent from original CSV')
 # Exact one-store leaf instruction, used only as a transparent input-pointer adapter.
 if b[0xd19b78:0xd19b80]!=bytes.fromhex('010000f9c0035fd6'):raise ValueError('Native pointer adapter instruction mismatch')
 def ref(name,offset=0):return {'ref':name,**({'offset':hex(offset)} if offset else {})}
 steps=[]
 if context is None:
  steps.append({'name':'context','rva':'0xe160c0','args':[],'expect_nonzero':True});ctx=ref('context')
 else:ctx=hex(context)
 def buffer(name,payload=None,size=None):
  steps.append({'name':name,**({'data_hex':payload.hex()} if payload is not None else {}),**({'size':hex(size)} if size is not None else {})})
 def call(name,rva,args,nonnull=False):steps.append({'name':name,'rva':hex(rva),'args':args,**({'expect_nonzero':True} if nonnull else {})})
 buffer('filename_bytes',filename.encode()+b'\0');buffer('filename_string',size=16)
 call('construct_filename_string',0x142fde4,[ref('filename_string'),ref('filename_bytes')])
 steps.append({'name':'csv_bytes','file':str(asset),'sha256':digest,'nul_terminate':True})
 buffer('csv_node',size=0x68)
 call('parse_original_csv',0x1288f2c,[ref('csv_node'),ref('csv_bytes'),len(data),ref('filename_string')])
 call('parsed_csv_table',0x1289a04,[ref('csv_node')],True)
 buffer('descriptor',size=0x20)
 call('construct_native_descriptor',0xe14440,[ref('descriptor'),ref('filename_string'),table_id,0])
 steps.append({'name':'file_entries','read_u64':ref('descriptor',8)})
 steps.append({'name':'first_file_entry','read_u64':ref('file_entries')})
 call('bind_parsed_csv_input_pointer',0xd19b78,[ref('first_file_entry',0x10),ref('csv_node')])
 call('attach_original_native_table',0xe15e64,[ctx,ref('descriptor'),table_id])
 if context is None:tableaddress=ref('context',table_id*8)
 else:tableaddress=hex(context+table_id*8)
 steps.append({'name':'context_table','read_u64':tableaddress})
 buffer('lookup_name_bytes',row.encode()+b'\0');buffer('lookup_name_string',size=16)
 call('construct_lookup_name_string',0x142fde4,[ref('lookup_name_string'),ref('lookup_name_bytes')])
 # Source-bound original wrappers for Testing and GameModes retain their exact table offsets.
 if table_id==121:lookup=0xe1b7e8
 elif table_id==72:lookup=0xe191c4
 else:raise ValueError('No verified exact native name-lookup wrapper registered for this tableID')
 call('lookup_original_row',lookup,[ctx,ref('lookup_name_string'),0],True)
 apis=[(0xe160c0,'context factory'),(0x142fde4,'native String constructor'),(0x1288f2c,'original CSV root-node parser constructor'),(0x1289a04,'CSV root-node table accessor'),(0xe14440,'native resource descriptor constructor'),(0xd19b78,'one-pointer input adapter'),(0xe15e64,'context native table attach'),(lookup,'native table name lookup wrapper')]
 size_index={int(x['start'],16):int(x['size']) for x in csv.DictReader((ROOT/'analysis/native-engine-index/unwind-functions.tsv').open(),delimiter='\t')}
 evidence=[{'rva':hex(a),'purpose':purpose,'exact_FDE_size':size_index[a],'source_file_bytes_sha256':hashlib.sha256(b[a:a+size_index[a]]).hexdigest()} for a,purpose in apis]
 plan={'source_sha256':EXPECTED,'purpose':'Parse original supplied CSV bytes into an original native CSV node and attach original typed table objects; look up an original named row','input_binding':'The only resource-delivery adapter stores the original parser result pointer into the original descriptor file entry at+0x10 using source-verified STRX1,[X0];RET. No row fields or typed engine objects are synthesized.','asset':{'path':filename,'decoded_file':str(asset),'sha256':digest,'bytes':len(data),'table_id':table_id,'row_name':row,'original_csv_row_names':names},'native_api_evidence':evidence,'steps':steps,'inspect':[{'address':ref('csv_node'),'size':'0x68'},{'address':ref('parsed_csv_table'),'size':'0x68'},{'address':ref('descriptor'),'size':'0x20'},{'address':ref('first_file_entry'),'size':'0x20'},{'address':ref('context_table'),'size':'0x98'},{'address':ref('lookup_original_row'),'size':'0x90'}]}
 if context is None:plan['inspect'].append({'address':ctx,'size':'0x10a8'})
 out.parent.mkdir(parents=True,exist_ok=True);out.write_text(json.dumps(plan,indent=2)+'\n');print(json.dumps({'plan':str(out),'sha256':hashlib.sha256(out.read_bytes()).hexdigest(),'steps':len(steps),'asset':plan['asset'],'context_factory_included':context is None},indent=2))
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--out',type=Path,default=ROOT/'analysis/native-focused/resources/testing-lookup-plan.json');p.add_argument('--table-id',type=lambda x:int(x,0),default=121);p.add_argument('--filename',default='csv_logic/testing.csv');p.add_argument('--row',default='Normal');p.add_argument('--context',type=lambda x:int(x,0));a=p.parse_args();make(a.out,a.table_id,a.filename,a.row,a.context)
