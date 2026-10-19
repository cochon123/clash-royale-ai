#!/usr/bin/env python3
"""Source-bound native class/data/UI anchor index, without a Ghidra import.

Uses exact clear strings, original ELF RELA addends, the shipped unwind table,
and validated linear ADRP/ADD or ADRP/LDR instruction sequences. These anchors
are evidence for focused decompilation, not recovered C++ source or behavior.
"""
import argparse,array,bisect,collections,hashlib,json,re,struct
from pathlib import Path
from capstone import Cs,CS_ARCH_ARM64,CS_MODE_LITTLE_ENDIAN
from capstone.arm64_const import ARM64_OP_IMM
from elftools.elf.elffile import ELFFile
from elftools.elf.relocation import RelocationTable
ROOT=Path(__file__).resolve().parents[1]
EXPECTED='79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6'
UI_NAMES={'HUD_player','HUD_topRight','ability_ingame_button','NextSpellTimer','elixir_bar','nextCard1','ability_location','cardArea'}
SEARCHES=['LogicBattle','LogicCharacter','LogicGameObject','LogicProjectile','LogicLevel','LogicGameMode','BattleView','BattleState','battle_controls','LogicBattleDeck::',*sorted(UI_NAMES)]

def run(source,out):
 b=source.read_bytes();digest=hashlib.sha256(b).hexdigest()
 if digest!=EXPECTED:raise ValueError('Derived ELF SHA256 mismatch')
 out.mkdir(exist_ok=True,parents=True)
 with source.open('rb') as f:
  e=ELFFile(f);segs=[dict(s.header) for s in e.iter_segments() if s['p_type']=='PT_LOAD'];eh=next(dict(s.header) for s in e.iter_segments() if s['p_type']=='PT_GNU_EH_FRAME');d=next(s for s in e.iter_segments() if s['p_type']=='PT_DYNAMIC');tags={t.entry.d_tag:t.entry.get('d_val',t.entry.get('d_ptr')) for t in d.iter_tags()}
  def off(va):
   for s in segs:
    if s['p_vaddr']<=va<s['p_vaddr']+s['p_filesz']:return s['p_offset']+va-s['p_vaddr']
   raise ValueError(f'Address not file-backed {va:x}')
  relocs=[]
  for addr,sz in [('DT_RELA','DT_RELASZ'),('DT_JMPREL','DT_PLTRELSZ')]:
   if addr in tags:
    for r in RelocationTable(e,off(tags[addr]),tags[sz],True).iter_relocations():relocs.append(dict(r.entry))
 # Original unwind records establish source-level compilation boundaries.
 hdr=eh['p_vaddr'];ho=eh['p_offset'];assert b[ho:ho+4]==bytes([1,0x1b,3,0x3b]);count=struct.unpack_from('<I',b,ho+8)[0];funcs=[];cies=collections.Counter()
 for i in range(count):
  rp,rf=struct.unpack_from('<ii',b,ho+12+i*8);start=hdr+rp;fde=hdr+rf;fo=off(fde);length,cieback,begin,size=struct.unpack_from('<IIiI',b,fo);cie=fde+4-cieback
  if start!=fde+8+begin:raise ValueError('FDE and search table disagree')
  if b[off(cie)+8:off(cie)+20]!=bytes.fromhex('017a5200017c1e011b0c1f00'):raise ValueError('Unrecognized CIE encoding')
  cies[cie]+=1;funcs.append({'start':start,'end':start+size,'size':size,'fde':fde,'cie':cie})
 starts=[x['start'] for x in funcs];start_set=set(starts)
 def owner(va):
  i=bisect.bisect_right(starts,va)-1
  return funcs[i] if i>=0 and va<funcs[i]['end'] else None
 (out/'unwind-functions.tsv').write_text('start\tend_exclusive\tsize\tfde\tcie\n'+''.join(f'0x{x["start"]:x}\t0x{x["end"]:x}\t{x["size"]}\t0x{x["fde"]:x}\t0x{x["cie"]:x}\n' for x in funcs))
 strings=[]
 for m in re.finditer(rb'[\x20-\x7e]{4,}\0',b):
  # Selected anchors must belong to the recovered string range, avoiding code coincidences.
  if not 0x2b1680<=m.start()<0x3cc210:continue
  text=m.group()[:-1].decode();cat=None
  if text in UI_NAMES:cat='exact_recovered_native_ui_instance'
  elif re.search(r'LogicBattleDeck::|LogicBattleBoostMode|battle end hud:|cannot find.*HUD_topRight',text):cat='explicit_method_or_subsystem_assertion'
  elif text.startswith('logic.data.') and re.search(r'Logic(Character|GameObject|Projectile|Battle|GameMode)|\.actions\.',text):cat='logic_data_reflection_type'
  elif re.search(r'Logic(Character|GameObject|Projectile|Battle|GameMode)',text):cat='named_logic_component_or_asset'
  elif text.startswith('HUD_'):cat='battle_ui_or_flow_literal'
  elif re.search(r'^(pvp_battle|battle_end|replay|spectate|elixir|card_slot|card_row|hand_card)|^Battle(Page|State|EndScreen)$|battle.*hud|hud.*battle',text,re.I) and not text.startswith(('TID_','soundbank','ui/autochess')):cat='battle_ui_or_flow_literal'
  if cat:strings.append({'virtual_address':m.start(),'file_offset':m.start(),'size':len(m.group()),'text':text,'category':cat})
 string_starts=[x['virtual_address'] for x in strings]
 def anchor(va):
  n=bisect.bisect_right(string_starts,va)-1
  if n>=0 and va<strings[n]['virtual_address']+strings[n]['size']-1:return n
  return None
 refs=[];by_pointer={};function_pointer_relocs=[];relative_count=0
 for r in relocs:
  if r['r_info_type']!=1027:continue
  relative_count+=1;target=r['r_addend'];idx=anchor(target)
  if idx is not None:
   record={'kind':'ELF_R_AARCH64_RELATIVE_addend','location':r['r_offset'],'target':target,'anchor_index':idx,'interior_offset':target-strings[idx]['virtual_address']};refs.append(record);by_pointer[r['r_offset']]=idx
  if target in start_set:function_pointer_relocs.append((r['r_offset'],target))
 # vtable candidates: contiguous relative pointer slots pointing at unwind starts.
 groups=[]
 for loc,target in sorted(function_pointer_relocs):
  if not groups or loc!=groups[-1][-1][0]+8:groups.append([])
  groups[-1].append((loc,target))
 groups=[g for g in groups if len(g)>=2];group_by_func=collections.defaultdict(list)
 for gi,g in enumerate(groups):
  for slot,(loc,target) in enumerate(g):group_by_func[target].append((gi,slot,loc))
 run_addresses={g[0][0]:gi for gi,g in enumerate(groups)};run_code_refs=[]
 md=Cs(CS_ARCH_ARM64,CS_MODE_LITTLE_ENDIAN);md.detail=True
 def sign(n,bits):return n-(1<<bits) if n&(1<<(bits-1)) else n
 def valid_interval(pc,at,regnum):
  instructions=list(md.disasm(b[off(pc):off(at)+4],pc));rn=f'x{regnum}'
  if len(instructions)!=(at-pc)//4+1:return None
  for i in instructions[1:-1]:
   if i.mnemonic in ('b','bl','br','blr','ret') or i.mnemonic.startswith('b.'):return None
   _,writes=i.regs_access()
   if any(md.reg_name(r) in (rn,f'w{regnum}') for r in writes):return None
  return [f'0x{i.address:x} {i.mnemonic} {i.op_str}' for i in instructions]
 code_start=starts[0];code_end=max(x['end'] for x in funcs);assert code_start==0x7706b0
 words=array.array('I');words.frombytes(b[off(code_start):off(code_end)])
 for wi,w in enumerate(words):
  pc=code_start+wi*4
  if w&0x9f000000==0x10000000:
   target=pc+sign(((w>>5)&0x7ffff)<<2|((w>>29)&3),21);idx=anchor(target)
   if idx is not None:refs.append({'kind':'ARM64_ADR','location':pc,'target':target,'anchor_index':idx,'instructions':[f'0x{i.address:x} {i.mnemonic} {i.op_str}' for i in md.disasm(b[off(pc):off(pc)+4],pc)]})
  if w&0x9f000000!=0x90000000:continue
  rd=w&31;page=(pc&~4095)+(sign(((w>>5)&0x7ffff)<<2|((w>>29)&3),21)<<12)
  for distance in range(1,9):
   if wi+distance>=len(words):break
   v=words[wi+distance];rn=(v>>5)&31
   if rn!=rd:continue
   at=pc+distance*4;target=None;kind=None;idx=None
   if v&0xff000000==0x91000000:
    target=page+((v>>10)&0xfff)*(4096 if v&(1<<22) else 1);idx=anchor(target);kind='ARM64_ADRP_ADD_linear'
   elif v&0xffc00000==0xf9400000:
    target=page+((v>>10)&0xfff)*8
    if target in by_pointer:idx=by_pointer[target];kind='ARM64_ADRP_LDR_relocated_pointer_linear'
   run_idx=run_addresses.get(target) if kind=='ARM64_ADRP_ADD_linear' else None
   if idx is None and run_idx is None:continue
   evidence=valid_interval(pc,at,rd)
   if evidence is None:continue
   function=owner(at)
   record={'kind':kind,'location':at,'page_instruction':pc,'target':target,'function':function,'instructions':evidence}
   if idx is not None:refs.append({**record,'anchor_index':idx})
   if run_idx is not None:run_code_refs.append({**record,'pointer_run_index':run_idx})
 # Resolve original vtable-style pointer runs only for literal-referencing functions.
 code_functions={r['function']['start'] for r in refs if r.get('function')}
 candidate_groups=set();function_links=[]
 for function in sorted(code_functions):
  for gi,slot,location in group_by_func.get(function,[]):
   candidate_groups.add(gi);function_links.append({'function':function,'pointer_run_index':gi,'slot':slot,'relocation_location':location})
 candidates=[]
 for gi in sorted(candidate_groups):
  g=groups[gi];candidates.append({'index':gi,'start':g[0][0],'end_exclusive':g[-1][0]+8,'slot_count':len(g),'slots':[{'location':a,'target':t} for a,t in g],'interpretation':'contiguous function-pointer run; vtable/dispatch candidate, not yet a proven C++ class vtable'})
 for i,s in enumerate(strings):s['reference_count']=sum(r['anchor_index']==i for r in refs)
 searches=[]
 for p in SEARCHES:
  matches=[s for s in strings if p in s['text']]
  raw_count=len(list(re.finditer(re.escape(p.encode()),b)))
  searches.append({'query':p,'raw_substring_occurrences_in_entire_ELF':raw_count,'selected_complete_string_count':len(matches),'selected_string_addresses':[hex(s['virtual_address']) for s in matches],'scope':'entire derived ELF rawsubstring scan; selected printable NUL-terminated strings restricted to recovered data range'})
 summary={'source':str(source),'sha256':digest,'address_coordinates':'ELF virtual addresses (RVA), no Ghidra image base added','methods':['Exact recovered ASCII literals','Original R_AARCH64_RELATIVE addends','Original GNU EH frame header plus FDE function intervals','Validated linear ADRP/ADD and ADRP/LDR code references'],'unwind':{'header':hex(hdr),'function_count':len(funcs),'cie_counts':{hex(k):v for k,v in cies.items()},'code_start':hex(code_start),'code_end_exclusive':hex(code_end)},'selected_anchor_count':len(strings),'reference_count':len(refs),'relative_relocation_count':relative_count,'function_pointer_runs':len(groups),'linked_candidate_pointer_runs':len(candidates),'searches':searches,'limitations':['Addresses identify shipped native evidence; names from literals do not prove reconstructed class layouts or behavior.','ADRP scans cover at most eight subsequent linear instructions with register writes and branch boundaries checked; indirect/distant computations are not exhausted.','RELATIVE pointer runs can be dispatch arrays or vtables; this index does not assert the distinction.','LogicGameObjectFlowAsset and autochess-named literals can describe Merge Tactics rather than ordinary Royale simulation.','Function intervals are unwind metadata observations, not Ghidra decompiler boundaries.']}
 (out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');(out/'anchors.json').write_text(json.dumps(strings,indent=2)+'\n');(out/'references.json').write_text(json.dumps(refs,indent=2)+'\n');(out/'pointer-runs.json').write_text(json.dumps({'runs':candidates,'function_links':function_links,'code_references':[r for r in run_code_refs if r['pointer_run_index'] in candidate_groups]},indent=2)+'\n')
 # Named UI instance index retains local call instructions after address construction.
 ui_bindings=[]
 for i,s in enumerate(strings):
  if s['category']!='exact_recovered_native_ui_instance':continue
  sites=[]
  for r in refs:
   if r['anchor_index']!=i:continue
   site=dict(r)
   if r.get('function'):
    pc=r['location'];end=min(pc+52,r['function']['end'])
    site['following_instructions']=[f'0x{x.address:x} {x.mnemonic} {x.op_str}' for x in md.disasm(b[off(pc):off(end)],pc)]
   sites.append(site)
  ui_bindings.append({'literal':s,'reference_sites':sites})
 (out/'native-ui-instance-bindings.json').write_text(json.dumps(ui_bindings,indent=2)+'\n')
 # One pointer run is tied to five explicit LogicBattleDeck method assertions.
 deck_groups={link['pointer_run_index'] for r in refs if r.get('function') and 'LogicBattleDeck::' in strings[r['anchor_index']]['text'] for link in function_links if link['function']==r['function']['start']}
 for gi in sorted(deck_groups):
  g=next(x for x in candidates if x['index']==gi);slots=[]
  for sn,slot in enumerate(g['slots']):
   f=owner(slot['target']);pc=f['start'];n=f['size'];instructions=[f'0x{x.address:x} {x.mnemonic} {x.op_str}' for x in md.disasm(b[off(pc):off(pc)+min(n,512)],pc)]
   slots.append({**slot,'slot_index':sn,'function_start':pc,'function_end_exclusive':f['end'],'function_size':n,'source_bytes':b[off(pc):off(pc)+min(n,512)].hex(),'instructions':instructions,'truncated':n>512})
  (out/'battle-deck-pointer-run.json').write_text(json.dumps({'source_sha256':digest,'pointer_run':{**g,'slots':slots},'code_references':[r for r in run_code_refs if r['pointer_run_index']==gi]},indent=2)+'\n')
 # Focused functions carrying explicit method assertions.
 focused=[]
 for r in refs:
  if r.get('function') and strings[r['anchor_index']]['category']=='explicit_method_or_subsystem_assertion':focused.append({'literal':strings[r['anchor_index']],'reference':r,'pointer_run_links':[x for x in function_links if x['function']==r['function']['start']]})
 (out/'focused-method-anchors.json').write_text(json.dumps(focused,indent=2)+'\n');print(json.dumps(summary,indent=2))
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--source',type=Path,default=ROOT/'analysis/libg-cipher/libg.decrypted.so');p.add_argument('--out',type=Path,default=ROOT/'analysis/native-engine-index');a=p.parse_args();run(a.source,a.out)
