#!/usr/bin/env python3
"""Partition native resources and insert original text-table bootstrap before typed postload."""
import argparse
import csv
import hashlib
import json
from pathlib import Path
from native_engine_index import ROOT, EXPECTED


def sha(data):return hashlib.sha256(data).hexdigest()
def ref(name, offset=0):return {'ref':name, **({'offset':hex(offset)} if offset else {})}
def write(path, plan):
    data=(json.dumps(plan,indent=2)+'\n').encode()
    with path.open('xb') as stream:stream.write(data)
    return sha(data)


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--resource-plan',required=True,type=Path)
    parser.add_argument('--out',required=True,type=Path)
    args=parser.parse_args()
    if args.out.exists():raise ValueError('Refusing to overwrite retained plans')
    source=(ROOT/'analysis/libg-cipher/libg.decrypted.so').read_bytes()
    if sha(source)!=EXPECTED:raise ValueError('Native source identity mismatch')
    raw=args.resource_plan.read_bytes();original=json.loads(raw)
    if original['source_sha256']!=EXPECTED:raise ValueError('Resource plan source mismatch')
    starts=[i for i,s in enumerate(original['steps']) if s.get('rva')=='0xe16984']
    if len(starts)!=1:raise ValueError('Expected one complete typed postload boundary')
    boundary=starts[0]
    origin={'path':str(args.resource_plan.resolve()),'sha256':sha(raw)}
    prefix={**original,'purpose':'Original fixed and manifest resource loading and table preparation; stops BEFORE typed postload',
            'partition_of':origin,'generator_sha256':sha(Path(__file__).read_bytes()),
            'steps':original['steps'][:boundary],'inspect':[{'address':ref('context'),'size':'0x10a8'}]}
    asset=ROOT/'analysis/decoded/assets/csv_client/texts.csv';data=asset.read_bytes()
    catalog_path=ROOT/'analysis/native-focused/resources/catalog-descriptors.json'
    catalog=json.loads(catalog_path.read_text())
    entry=next(r for r in catalog['records'] if r['flags']==1 and r['table_id']==0)
    if len(entry['paths'])!=1 or entry['paths'][0]['decoded_sha256']!=sha(data):raise ValueError('Original localization catalog/input mismatch')
    fdes={int(r['start'],16):int(r['end_exclusive'],16) for r in csv.DictReader((ROOT/'analysis/native-engine-index/unwind-functions.tsv').open(),delimiter='\t')}
    apis=[0x142fde4,0x1288f2c,0x1289a04,0x12ad530,0x12ad7e8]
    evidence=[{'rva':hex(a),'end_rva_exclusive':hex(fdes[a]),'source_bytes_sha256':sha(source[a:fdes[a]])} for a in apis]
    steps=[
        {'name':'texts_filename_bytes','data_hex':b'csv_client/texts.csv\0'.hex()},
        {'name':'texts_filename_string','size':'0x10'},
        {'name':'texts_construct_filename','rva':'0x142fde4','args':[ref('texts_filename_string'),ref('texts_filename_bytes')]},
        {'name':'texts_original_bytes','file':str(asset.resolve()),'sha256':sha(data),'nul_terminate':True},
        {'name':'texts_native_csv_root','size':'0x68'},
        {'name':'texts_parse_original_csv','rva':'0x1288f2c','args':[ref('texts_native_csv_root'),ref('texts_original_bytes'),len(data),ref('texts_filename_string')]},
        {'name':'texts_native_csv_table','rva':'0x1289a04','args':[ref('texts_native_csv_root')],'expect_nonzero':True},
        {'name':'texts_initialize_original_singleton','rva':'0x12ad530','args':[ref('texts_native_csv_table')]},
        {'name':'texts_original_singleton','rva':'0x12ad7e8','args':[],'expect_nonzero':True},
    ]
    args.out.mkdir(parents=True)
    prefix_path=args.out/'resource-pretyped-plan.json';prefix_sha=write(prefix_path,prefix)
    language={'source_sha256':EXPECTED,'inherit_plan_sha256':prefix_sha,'purpose':'Original native text CSV parser and localization singleton required by native Language typed rows',
              'generator_sha256':sha(Path(__file__).read_bytes()),'catalog_entry':entry,'catalog_sha256':sha(catalog_path.read_bytes()),'native_api_evidence':evidence,
              'source_contract':'Original e6df50 flag-1/table-0 branch passes e149d4(descriptor) CSV table to 12ad530. Language loader e476cc calls 12ad7e8 then 12ad7f4 to retrieve that CSV table.',
              'steps':steps,'inspect':[{'address':ref('texts_original_singleton'),'size':'0x30'},{'address':ref('texts_native_csv_table'),'size':'0x68'}],
              'limitations':['Raw original multilingual texts.csv is parsed without filtering rows or languages.','The client resource-provider I/O is replaced by original asset byte delivery; the native parser and singleton initialization remain original.']}
    language_path=args.out/'language-plan.json';language_sha=write(language_path,language)
    typed={'source_sha256':EXPECTED,'inherit_plan_sha256':language_sha,'purpose':'Complete original typed resource postload after original localization initialization',
           'partition_of':origin,'generator_sha256':sha(Path(__file__).read_bytes()),'native_api_evidence':original['native_api_evidence'],
           'steps':original['steps'][boundary:],'inspect':original['inspect']}
    typed_path=args.out/'resource-typed-plan.json';typed_sha=write(typed_path,typed)
    result={'source_sha256':EXPECTED,'partition_of':origin,'plans':[{'path':str(p.resolve()),'sha256':h,'steps':n} for p,h,n in ((prefix_path,prefix_sha,len(prefix['steps'])),(language_path,language_sha,len(steps)),(typed_path,typed_sha,len(typed['steps'])))],'texts_bytes':len(data),'texts_sha256':sha(data),'native_execution_performed':False}
    write(args.out/'generation-report.json',result);print(json.dumps(result,indent=2))

if __name__=='__main__':main()
