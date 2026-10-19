#!/usr/bin/env python3
"""Resolve the entity-manager vtable from original relocations, preserving byte/FDE identities."""
import csv, hashlib, json, pathlib
from elftools.elf.elffile import ELFFile
from elftools.elf.relocation import RelocationTable

ROOT=pathlib.Path(__file__).resolve().parents[1]
SOURCE=ROOT/'analysis/libg-cipher/libg.decrypted.so'
EXPECTED='79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6'

def main():
    data=SOURCE.read_bytes();assert hashlib.sha256(data).hexdigest()==EXPECTED
    table=ROOT/'analysis/native-engine-index/unwind-functions.tsv'
    bounds={int(row['start'],16):row for row in csv.DictReader(table.open(),delimiter='\t')}
    with SOURCE.open('rb') as file:
        elf=ELFFile(file);segments=list(elf.iter_segments())
        dynamic=next(segment for segment in segments if segment['p_type']=='PT_DYNAMIC')
        tags={tag.entry.d_tag:tag.entry.get('d_val',tag.entry.get('d_ptr')) for tag in dynamic.iter_tags()}
        def offset(address):
            segment=next(segment for segment in segments if segment['p_type']=='PT_LOAD' and segment['p_vaddr']<=address<segment['p_vaddr']+segment['p_filesz'])
            return segment['p_offset']+address-segment['p_vaddr']
        relocations={r['r_offset']:r['r_addend'] for r in RelocationTable(elf,offset(tags['DT_RELA']),tags['DT_RELASZ'],True).iter_relocations() if r['r_info_type']==1027}
        vtable=0x1936450
        slots=[]
        for index in range(19):
            at=vtable+index*8;target=relocations[at];fde=bounds.get(target)
            assert fde is not None
            size=int(fde['size']);position=offset(target)
            slots.append({'slot_index':index,'slot_offset':hex(index*8),'slot_rva':hex(at),'target_rva':hex(target),'fde':fde,'target_bytes_sha256':hashlib.sha256(data[position:position+size]).hexdigest()})
    result={'source':str(SOURCE),'source_sha256':EXPECTED,'script_sha256':hashlib.sha256(pathlib.Path(__file__).read_bytes()).hexdigest(),'fde_table_sha256':hashlib.sha256(table.read_bytes()).hexdigest(),'method':'Constructor RVAf961f0 stores vtable RVA1936450; resolve original R_AARCH64_RELATIVE addends at that table, and require every selected target to be an exact original unwind start.','constructor_rva':'0xf961f0','vtable_rva':hex(vtable),'slots':slots,'limits':['The table assignment is established by the constructor assembly; higher-level class names are inferred separately.','Virtual-slot mapping establishes dispatch targets for this concrete manager class, not for arbitrary subclasses or components.','No game or native tick was executed by this script.']}
    output=ROOT/'analysis/native-focused/engine-step/entity-vtable.json'
    output.write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({slot['slot_offset']:slot['target_rva'] for slot in slots},indent=2))

if __name__=='__main__':main()
