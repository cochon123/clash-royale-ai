#!/usr/bin/env python3
"""Prepare original battle construction and tick calls after a fully loaded native context.

This generates source-bound plans; it never executes or fabricates game state.
Each plan inherits only from the exact SHA256 of its completed predecessor.
"""
import argparse, csv, hashlib, json, pathlib, struct

ROOT=pathlib.Path(__file__).resolve().parents[1]
SOURCE=ROOT/'analysis/libg-cipher/libg.decrypted.so'
EXPECTED='79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6'

def digest(data):return hashlib.sha256(data).hexdigest()
def ref(name,offset=0):
    result={'ref':name}
    if offset:result['offset']=hex(offset)
    return result

def construction_steps(seed,extra):
    return [
        {'name':'battle_name_bytes','data_hex':b'Normal\0'.hex()},
        {'name':'battle_name_string','size':'0x10'},
        {'name':'construct_native_battle_name','rva':'0x142fde4','args':[ref('battle_name_string'),ref('battle_name_bytes')]},
        {'name':'confirm_original_Normal','rva':'0xe1b7e8','args':[ref('context'),ref('battle_name_string'),0],'expect_nonzero':True},
        {'name':'battle_input_sret','data_hex':'00'*8},
        {'name':'construct_native_battle_input','rva':'0x11d7968','args':[ref('context'),ref('battle_name_string'),seed,extra],'sret':ref('battle_input_sret')},
        {'name':'battle_input','read_u64':ref('battle_input_sret')},
        {'name':'battle_harness','size':'0x78'},
        {'name':'construct_native_battle_harness','rva':'0x11d58e0','args':[ref('battle_harness'),ref('context'),ref('battle_input_sret')]},
        {'name':'battle_world','read_u64':ref('battle_harness',0x20)},
        {'name':'battle_state','read_u64':ref('battle_world',0xa8)},
        {'name':'battle_entities','read_u64':ref('battle_world',0x40)},
        {'name':'battle_commands','read_u64':ref('battle_world',0x38)},
        {'name':'battle_config','read_u64':ref('battle_state',0x1b0)},
        {'name':'battle_arena','read_u64':ref('battle_state',0xb8)},
        {'name':'battle_tilemap','read_u64':ref('battle_state',0xc0)},
        {'name':'battle_world_counter_before','read_u32':ref('battle_world',0x60)},
        {'name':'battle_world_other_counter_before','read_u32':ref('battle_world',0x70)},
    ]

def inspections():
    return [
        {'address':ref('battle_harness'),'size':'0x78'},
        {'address':ref('battle_world'),'size':'0x238'},
        {'address':ref('battle_state'),'size':'0x378'},
        {'address':ref('battle_entities'),'size':'0xf0'},
        {'address':ref('battle_commands'),'size':'0x30'},
        {'address':ref('battle_config'),'size':'0xd8'},
        {'address':ref('battle_arena'),'size':'0x80'},
        {'address':ref('battle_tilemap'),'size':'0x80'},
        {'address':ref('battle_input'),'size':'0x28'},
        {'address':ref('confirm_original_Normal'),'size':'0xa0'},
    ]

def validate_checkpoint(path,plan,identity):
    state=json.loads(path.read_text());saved=state.get('call_plan') or {};initializers=state.get('init_array') or {}
    if state['source_sha256']!=EXPECTED or saved.get('sha256')!=identity:raise ValueError('Predecessor checkpoint identity mismatch')
    if state.get('failure') is not None or saved.get('in_progress') is not None or saved.get('completed')!=len(plan['steps']):raise ValueError('Predecessor plan has not completed successfully')
    if initializers.get('count')!=763 or initializers.get('completed')!=763:raise ValueError('Original 763 native constructors have not all returned')
    if not saved.get('values',{}).get('context'):raise ValueError('Native context pointer is absent')

def has_source_proven_context_route(steps):
    # The inner factory route remains valid for plans that call e160c0 directly.
    if any(step.get('name')=='context' and step.get('rva')=='0xe160c0' for step in steps):
        return True
    # The outer factory publishes its manager globally, then exposes the
    # original context at manager+0x20. Keep the exact dataflow explicit.
    for index in range(len(steps)-2):
        factory,manager,context=steps[index:index+3]
        if (factory.get('name')=='construct_original_logic_data_manager' and
            factory.get('rva')=='0x8264f0' and factory.get('args',[])==[] and
            manager.get('name')=='original_logic_data_manager' and
            manager.get('read_u64')=='0x11a46a80' and
            context.get('name')=='context' and
            context.get('read_u64')=={'ref':'original_logic_data_manager','offset':'0x20'}):
            return True
    return False

def build(args):
    source=SOURCE.read_bytes()
    if digest(source)!=EXPECTED:raise ValueError('Recovered source ELF identity mismatch')
    previous_bytes=args.bulk_plan.read_bytes();previous=json.loads(previous_bytes);previous_sha=digest(previous_bytes)
    if previous['source_sha256']!=EXPECTED:raise ValueError('Bulk plan is bound to another ELF')
    if not has_source_proven_context_route(previous['steps']):raise ValueError('Bulk plan lacks a source-proven original context factory route (e160c0 or outer 8264f0/global-manager/context+0x20)')
    def completes_typed(plan):
        return any(step.get('rva')=='0xe16984' or
                   (step.get('rva')=='0xe1698c' and len(step.get('args',[]))>1 and
                    isinstance(step['args'][1],(str,int)) and int(str(step['args'][1]),0)&1)
                   for step in plan['steps'])
    typed_postload=completes_typed(previous)
    resource_chain=[{'path':str(args.bulk_plan.resolve()),'sha256':previous_sha}]
    for continuation in args.after_plan:
        contents=continuation.read_bytes();following=json.loads(contents)
        if following['source_sha256']!=EXPECTED or following.get('inherit_plan_sha256')!=previous_sha:raise ValueError('Resource continuation plan identity mismatch')
        typed_postload=typed_postload or completes_typed(following)
        previous_bytes=contents;previous=following;previous_sha=digest(contents)
        resource_chain.append({'path':str(continuation.resolve()),'sha256':previous_sha})
    if not typed_postload:raise ValueError('Resource chain lacks original complete typed context postload (flag 1)')
    if args.checkpoint:validate_checkpoint(args.checkpoint,previous,previous_sha)
    args.output.mkdir(parents=True,exist_ok=True)
    records=[]
    stages=[
        ('construct',construction_steps(args.seed,args.extra)),
        ('world-step',[
            {'name':'execute_original_world_step','rva':'0x10ca81c','args':[ref('battle_world')]},
            {'name':'battle_world_counter_after_world_step','read_u32':ref('battle_world',0x60)},
            {'name':'battle_world_other_counter_after_world_step','read_u32':ref('battle_world',0x70)},
        ]),
        ('harness-step',[
            {'name':'battle_step_sret','data_hex':'00'*8},
            {'name':'execute_original_harness_step','rva':'0x11d6398','args':[ref('battle_harness')],'sret':ref('battle_step_sret')},
            {'name':'battle_step_result','read_u64':ref('battle_step_sret')},
            {'name':'battle_world_counter_after_harness_step','read_u32':ref('battle_world',0x60)},
            {'name':'battle_world_other_counter_after_harness_step','read_u32':ref('battle_world',0x70)},
        ]),
    ]
    for stage,steps in stages:
        plan={'source_sha256':EXPECTED,'inherit_plan_sha256':previous_sha,
            'purpose':f'Original native Normal scenario: {stage}; no replacement game logic',
            'requirements':['Run only after the exact predecessor plan completed successfully, including all original context postload and resource loading.','All 763 original native initializers must have returned before executing the first construction plan.','The game itself must create its world, decks, tilemap, and initial objects; no synthesized context, rows, vtables or grid are supplied.'],
            'steps':steps,'inspect':inspections()}
        path=args.output/f'battle-plan-{stage}.json';data=(json.dumps(plan,indent=2)+'\n').encode();path.write_bytes(data)
        records.append({'stage':stage,'path':str(path.resolve()),'sha256':digest(data),'inherit_plan_sha256':previous_sha,'steps':len(steps)})
        previous_sha=digest(data)
    fdes={int(row['start'],16):row for row in csv.DictReader((ROOT/'analysis/native-engine-index/unwind-functions.tsv').open(),delimiter='\t')}
    functions=[]
    for rva in [0x142fde4,0xe1b7e8,0x11d7968,0x11d58e0,0x10ca81c,0x11d6398]:
        row=fdes[rva];size=int(row['size'])
        # These selected functions all reside in the first original file-backed LOAD,
        # whose virtual address and file offset are both zero.
        functions.append({'rva':hex(rva),'fde':row,'bytes_sha256':digest(source[rva:rva+size])})
    manifest={'source_elf':str(SOURCE),'source_sha256':EXPECTED,'generator_sha256':digest(pathlib.Path(__file__).read_bytes()),'bulk_plan':str(args.bulk_plan.resolve()),'bulk_plan_sha256':resource_chain[0]['sha256'],'resource_plan_chain':resource_chain,'predecessor_checkpoint_verified':str(args.checkpoint.resolve()) if args.checkpoint else None,'scenario':'Normal','seed':args.seed,'extra':args.extra,'plans':records,'native_functions':functions,'abi':['11d7968: X0=context,X1=&nativeString,W2=seed,W3=extra,X8=&inputPointerResult','11d58e0: X0=harnessStorage(0x78),X1=context,X2=&inputPointerResult','10ca81c: X0=world','11d6398: X0=harness,X8=&stepResult'],'status':'Plans generated only; native battle execution has not been performed by this generator.'}
    (args.output/'battle-plan-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    print(json.dumps({'bulk_plan_sha256':manifest['bulk_plan_sha256'],'plans':records,'execution_performed':False},indent=2))

def audit(args):
    report=json.loads(args.audit_report.read_text());state=report.get('call_plan') or {};values=state.get('values') or {};completed=state.get('completed',0)
    pointer_names=('battle_input','battle_harness','battle_world','battle_state','battle_entities','battle_commands','battle_config','battle_arena','battle_tilemap')
    pointers={name:hex(values.get(name,0)) for name in pointer_names}
    inspection_data={int(stage['address'],16):(args.audit_report.parent/stage['file']).read_bytes() for stage in report.get('stages',[]) if stage.get('phase')=='inspect'}
    manager=inspection_data.get(values.get('battle_entities'))
    world=inspection_data.get(values.get('battle_world'))
    native_count=struct.unpack_from('<i',manager,0x14)[0] if manager and len(manager)>=0x18 else None
    world_counter_after=struct.unpack_from('<I',world,0x60)[0] if world and len(world)>=0x64 else None
    world_other_counter_after=struct.unpack_from('<I',world,0x70)[0] if world and len(world)>=0x74 else None
    world_counter_before=values.get('battle_world_counter_before')
    world_other_counter_before=values.get('battle_world_other_counter_before')
    world_counter_delta=((world_counter_after-world_counter_before)&0xffffffff) if world_counter_after is not None and world_counter_before is not None else None
    world_other_counter_delta=((world_other_counter_after-world_other_counter_before)&0xffffffff) if world_other_counter_after is not None and world_other_counter_before is not None else None
    result={'source_sha256':report['source_sha256'],'report_sha256':digest(args.audit_report.read_bytes()),'failure':report.get('failure'),'plan_completed_steps':completed,'plan_in_progress':state.get('in_progress'),'pointers':pointers,'nonzero_pointers':all(values.get(name,0) for name in pointer_names),'native_entity_count':native_count,'native_world_counter_before':world_counter_before,'native_world_other_counter_before':world_other_counter_before,'native_world_counter_after':world_counter_after,'native_world_other_counter_after':world_other_counter_after,'native_world_counter_delta':world_counter_delta,'native_world_other_counter_delta':world_other_counter_delta,'both_native_world_counters_advanced':world_counter_delta is not None and world_other_counter_delta is not None and world_counter_delta>0 and world_other_counter_delta>0,'original_step_calls':[step for step in state.get('steps',[]) if step['name'] in ('execute_original_world_step','execute_original_harness_step')],'limits':['A returned constructor alone is insufficient: nonzero config/tilemap and spawned native entities must be inspected.','A successful original tick is not evidence of a complete Rust engine or a desktop UI.']}
    output=args.audit_report.parent/'native-battle-audit.json';output.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--bulk-plan',type=pathlib.Path,default=ROOT/'analysis/native-focused/resources/bulk-native-assets-plan.json')
    parser.add_argument('--after-plan',type=pathlib.Path,action='append',default=[],help='Completed resource continuation in dependency order, e.g. typed postload then original map-cache load; each exact parent hash is verified')
    parser.add_argument('--output',type=pathlib.Path,default=ROOT/'analysis/native-context')
    parser.add_argument('--checkpoint',type=pathlib.Path,help='Optional successful FULLY loaded predecessor checkpoint to validate before generation')
    parser.add_argument('--seed',type=lambda value:int(value,0),default=160402020)
    parser.add_argument('--extra',type=lambda value:int(value,0),default=-1)
    parser.add_argument('--audit-report',type=pathlib.Path,help='Audit a completed/failed native plan report; does not execute native code')
    args=parser.parse_args()
    if args.audit_report:audit(args)
    else:build(args)

if __name__=='__main__':main()
