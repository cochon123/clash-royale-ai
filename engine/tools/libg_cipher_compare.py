#!/usr/bin/env python3
"""Compare every direct-decoded protected range with retained native execution."""
import argparse,hashlib,json
from pathlib import Path
from libg_cipher_cfg import ROOT
from libg_cipher_verify import load_checkpoint

def compare(checkpoint,manifest,out):
 j,regions=load_checkpoint(checkpoint)
 m=json.loads(manifest.read_text());elf=Path(m['derived']).read_bytes()
 if hashlib.sha256(elf).hexdigest()!=m['derived_sha256']:raise ValueError('Derived ELF digest mismatch')
 game=next(raw for base,raw in regions if base==0x10000000)
 records=[]
 for r in m['ranges']:
  va=int(r['virtual_address'],16);off=int(r['file_offset'],16);n=r['size'];a=elf[off:off+n];b=game[va:va+n]
  if len(a)!=n or len(b)!=n:raise ValueError('Range bounds mismatch')
  diffs=[i for i,(x,y) in enumerate(zip(a,b)) if x!=y]
  records.append({**r,'all_bytes_equal':not diffs,'differences':len(diffs),'first_difference':hex(va+diffs[0]) if diffs else None,'direct_sha256':hashlib.sha256(a).hexdigest(),'native_sha256':hashlib.sha256(b).hexdigest()})
 result={'native_checkpoint':str(checkpoint.resolve()),'native_checkpoint_sha256':hashlib.sha256(checkpoint.read_bytes()).hexdigest(),'native_memory_sha256':hashlib.sha256(game).hexdigest(),'direct_elf_sha256':m['derived_sha256'],'all_ranges_equal':all(r['all_bytes_equal'] for r in records),'ranges':records}
 out.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('checkpoint',type=Path);p.add_argument('--manifest',type=Path,default=ROOT/'analysis/libg-cipher/direct-decode.json');p.add_argument('--out',type=Path,required=True);a=p.parse_args();compare(a.checkpoint,a.manifest,a.out)
