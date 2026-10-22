#!/usr/bin/env python3
"""Combine original fixed and manifest resource stages before native postload."""
import argparse
import hashlib
import json
from pathlib import Path
from native_engine_index import ROOT, EXPECTED


def sha(data):return hashlib.sha256(data).hexdigest()


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--fixed',type=Path,default=ROOT/'analysis/native-focused/resources/bulk-native-assets-plan.json')
    parser.add_argument('--dynamic',type=Path,default=ROOT/'analysis/native-context/dynamic-v1/dynamic-native-assets-plan.json')
    parser.add_argument('--out',type=Path,default=ROOT/'analysis/native-context/complete-v2/complete-native-assets-plan.json')
    args=parser.parse_args()
    if args.out.exists():raise ValueError('Refusing to overwrite a retained native execution plan')
    fixed_bytes=args.fixed.read_bytes();dynamic_bytes=args.dynamic.read_bytes()
    fixed=json.loads(fixed_bytes);dynamic=json.loads(dynamic_bytes)
    if fixed['source_sha256']!=EXPECTED or dynamic['source_sha256']!=EXPECTED:raise ValueError('Resource source identity mismatch')
    if dynamic['inherit_plan_sha256']!=sha(fixed_bytes):raise ValueError('Dynamic stage does not inherit this exact fixed catalog')
    cutoff=next(i for i,step in enumerate(fixed['steps']) if step.get('rva')=='0xe6e3ec')
    steps=fixed['steps'][:cutoff]+dynamic['steps']
    names=[step['name'] for step in steps]
    if len(names)!=len(set(names)):raise ValueError('Combined plan has ambiguous step names')
    if not any(step.get('rva')=='0xe16984' for step in steps):raise ValueError('Combined plan omits mandatory original typed postload')
    proof={item['rva']:item for item in fixed['native_api_evidence']+dynamic['native_api_evidence']}
    plan={'source_sha256':EXPECTED,'purpose':'Original fixed catalog, original data_manifest TOML entries, dependency construction and complete typed postload in original loader order',
        'generator_sha256':sha(Path(__file__).read_bytes()),
        'input_stages':[{'path':str(args.fixed.resolve()),'sha256':sha(fixed_bytes),'included_step_count':cutoff},{'path':str(args.dynamic.resolve()),'sha256':sha(dynamic_bytes),'included_step_count':len(dynamic['steps'])}],
        'ordering_evidence':'Original e69930 appends manifest-derived flag-4 resources through e69758; e6df50 feeds all their parsed roots to e6e280 before calling e6e3ec, e16858, e12548 and e16984. The preliminary postload from the fixed-only probe is excluded.',
        'catalog':fixed['catalog'],'assets':fixed['assets'],'input_manifest':dynamic['input_manifest'],'dynamic_assets':dynamic['dynamic_assets'],
        'input_adapter':dynamic['input_adapter'],'native_api_evidence':list(proof.values()),'steps':steps,'inspect':dynamic['inspect'],
        'limitations':['Native execution and validation must succeed before this context can be used for a battle.','The two localization descriptors use separate client language APIs.','No native rows, card rules, map grids or battle state are synthesized.']}
    args.out.parent.mkdir(parents=True,exist_ok=True)
    data=(json.dumps(plan,indent=2)+'\n').encode();args.out.write_bytes(data)
    print(json.dumps({'plan':str(args.out.resolve()),'sha256':sha(data),'steps':len(steps),'fixed_files':len(fixed['assets']),'dynamic_entries':len(dynamic['dynamic_assets']),'native_execution_performed':False},indent=2))


if __name__=='__main__':main()
