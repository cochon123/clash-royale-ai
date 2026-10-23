import json,shutil,time
from pathlib import Path
r=Path('/tmp/cr-prof/bench-safe');seen={}
while True:
 for d in r.glob('*-scratch'):
  for p in d.iterdir():
   if p.is_file() and (p.suffix in ('.json','.jsonl','.log')):
    try:
     sig=(p.stat().st_mtime_ns,p.stat().st_size)
     if seen.get(str(p))!=sig:
      shutil.copy2(p,r/(d.name.removesuffix('-scratch')+'-'+p.name));seen[str(p)]=sig
    except FileNotFoundError:pass
 try:
  if json.loads((r/'progress.json').read_text()).get('status') in ('completed','aborted'):break
 except (FileNotFoundError,json.JSONDecodeError):pass
 time.sleep(.1)
