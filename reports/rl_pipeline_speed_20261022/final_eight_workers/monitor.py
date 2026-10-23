import json,time,argparse
from pathlib import Path
ap=argparse.ArgumentParser();ap.add_argument('--watch',type=float,default=0);args=ap.parse_args()
def show():
 try:p=json.loads(Path('/tmp/cr-prof/bench-safe/progress.json').read_text())
 except (FileNotFoundError,json.JSONDecodeError):return
 
 if p.get('status')=='completed':
  p.setdefault('configuration','ALL DONE')
  summary=json.loads(Path('/tmp/cr-prof/bench-safe/summary.json').read_text())
  p.setdefault('memory_peak_gib',max(x['memory_peak_bytes'] for x in summary)/1024**3)
  p.setdefault('available_gib',int(next(l for l in Path('/proc/meminfo').read_text().splitlines() if l.startswith('MemAvailable:')).split()[1])/1024**2)
 eta=p.get('eta_seconds');eta='estimating' if eta is None else f'{eta//60:.0f}m{eta%60:02.0f}s'
 print(f"{'CONFIG':<12} {'WORK_DONE':<12} {'ETA':<11} {'RAM_CURRENT/PEAK':<20} {'AVAILABLE':<12} STATUS")
 print(f"{p.get('configuration','—'):<12} {p.get('work_done',str(p.get('completed_waves',0))+'/18 waves'):<12} {eta:<11} {p.get('memory_current_gib',0):.2f}/{p.get('memory_peak_gib',0):.2f} GiB{'':<6} {p.get('available_gib',0):.2f} GiB{'':<4} {p.get('status','?')}",flush=True)
 if p.get('error'):print(p['error'])
while True:
 show()
 if not args.watch:break
 time.sleep(args.watch)
