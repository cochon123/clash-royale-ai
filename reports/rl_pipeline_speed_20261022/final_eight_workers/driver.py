import os,sys,json,time,subprocess,shlex,shutil,datetime,hashlib,traceback
from pathlib import Path
R=Path('/tmp/cr-prof/bench-safe')
PYTHON='/home/cochon/Documents/mini-project/clash-royale-ai/.venv/bin/python'
BASE='/home/cochon/Documents/mini-project/clash-royale-ai/engine'
NEW='/home/cochon/Documents/mini-project/clash-royale-ai-worktrees/rl-speed-integration/engine'
GIB=1024**3
STAMP=datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
start=time.monotonic(); summary=[]; samples=[]; current_unit=None
R.joinpath('session.txt').write_text(STAMP+'\n')
def command(cmd,cwd=None):
 p=subprocess.run(cmd,cwd=cwd,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=30)
 return {'command':shlex.join(cmd),'returncode':p.returncode,'output':p.stdout}
def write(path,obj):
 path=Path(path); tmp=path.with_suffix(path.suffix+'.tmp');tmp.write_text(json.dumps(obj,indent=2));tmp.replace(path)
def mem():
 return {line.split(':')[0]:int(line.split()[1])*1024 for line in Path('/proc/meminfo').read_text().splitlines() if line.startswith(('MemAvailable:','MemTotal:','SwapFree:','SwapTotal:'))}
def snapshot():
 return {'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'memory_bytes':mem(),'gpu':command(['nvidia-smi','--query-gpu=temperature.gpu,utilization.gpu,memory.used,power.draw','--format=csv,noheader'])['output'].strip()}
def props(unit):
 out=command(['systemctl','--user','show',unit,'--property=ActiveState,SubState,Result,ExecMainStatus,MemoryCurrent,MemoryPeak,ControlGroup,MemoryMax,MemorySwapMax,OOMPolicy'])['output']
 return dict(line.split('=',1) for line in out.splitlines() if '=' in line)
def readnum(path):
 try:return int(path.read_text())
 except (FileNotFoundError,ValueError):return None
def telemetry(unit):
 p=props(unit); cg=Path('/sys/fs/cgroup'+p.get('ControlGroup',''))
 d={'elapsed_s':time.monotonic()-start,'host':mem(),'unit':p,'cgroup_memory_current':readnum(cg/'memory.current'),'cgroup_memory_peak':readnum(cg/'memory.peak')}
 try:d['memory_events']={x.split()[0]:int(x.split()[1]) for x in (cg/'memory.events').read_text().splitlines()}
 except FileNotFoundError:pass
 return d
def parse_result(raw):
 result=None
 for i,c in enumerate(raw):
  if c=='{':
   try:
    o,_=json.JSONDecoder().raw_decode(raw[i:])
    if isinstance(o,dict) and 'code_root' in o and 'total_s' in o:result=o
   except json.JSONDecodeError:pass
 return result
try:
 pre=snapshot();pre['repos']={n:{'head':command(['git','rev-parse','HEAD'],p),'status':command(['git','status','--short'],p)} for n,p in [('baseline',BASE),('new',NEW)]}
 pre['inputs']={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [Path(BASE,'runs/banana-hog-2/bin/rust_league_rl'),Path(BASE,'runs/banana-hog-2/pool.json'),Path(BASE,'runs/banana-hog-1/models/final.pt')]}
 pre['limits']={'memory_max_bytes':8*GIB,'swap_max_bytes':0,'abort_mem_available_bytes':6*GIB,'required_launch_available_bytes':12*GIB,'workers':8}
 write(R/'preflight.json',pre)
 configs=[('BASELINE8',BASE,None),('NEW8g1',NEW,1),('NEW8g2',NEW,2)]
 schedule=[(777,c) for c in configs]+[(2026,c) for c in reversed(configs)]
 for index,(seed,(name,code,groups)) in enumerate(schedule):
  label=f'{STAMP}-{index+1:02d}-{name}-s{seed}';scratch=R/(label+'-scratch');log=R/(label+'.stdout.log')
  before=snapshot();write(R/(label+'-before.json'),before)
  if before['memory_bytes']['MemAvailable'] < 12*GIB:raise RuntimeError('Pre-launch MemAvailable below 12GiB; no trainer launched')
  cmd=[PYTHON,str(R/'bench_wave_measured.py'),'--code-root',code,'--workers','8','--waves','3','--seed',str(seed),'--run-dir',str(scratch)]
  if groups is not None:cmd+=['--extra',f'--step-groups {groups}']
  unit=f'cr-bench-safe-{STAMP.lower()}-{index+1}.service';current_unit=unit
  launch=['systemd-run','--user','--unit='+unit,'--remain-after-exit','--working-directory='+code,'--property=MemoryMax=8G','--property=MemorySwapMax=0','--property=OOMPolicy=kill','--property=KillMode=control-group','--property=TimeoutStopSec=10','--property=RuntimeMaxSec=1200','--property=StandardOutput=append:'+str(log),'--property=StandardError=append:'+str(log),'--setenv=PYTHONDONTWRITEBYTECODE=1','--setenv=PYTHONUNBUFFERED=1']+cmd
  with (R/'commands.txt').open('a') as f:f.write(shlex.join(launch)+'\n')
  launch_result=command(launch);write(R/(label+'-launch.json'),launch_result)
  if launch_result['returncode']:raise RuntimeError('systemd-run failed '+launch_result['output'])
  run_start=time.monotonic();sample_next=0;run_samples=[];aborted=None;peak=0;waves=[]
  while True:
   t=telemetry(unit);run_samples.append(t)
   current=t['cgroup_memory_current'] or 0;peak=max(peak,current,t['cgroup_memory_peak'] or 0)
   try:waves=json.loads((scratch/'bench_waves.json').read_text())
   except (FileNotFoundError,json.JSONDecodeError):pass
   done=index*3+len(waves);elapsed=time.monotonic()-start;eta=elapsed/done*(18-done) if done else None
   progress={'status':'running','configuration':name,'seed':seed,'unit':unit,'completed_runs':index,'total_runs':6,'completed_waves':done,'total_waves':18,'work_done':f'{done}/18 waves','eta_seconds':round(eta) if eta else None,'elapsed_seconds':round(elapsed,1),'memory_current_gib':round(current/GIB,3),'memory_peak_gib':round(peak/GIB,3),'available_gib':round(t['host']['MemAvailable']/GIB,3)}
   write(R/'progress.json',progress)
   if t['host']['MemAvailable']<6*GIB:
    aborted='Host MemAvailable below 6GiB watchdog threshold';command(['systemctl','--user','stop',unit]);break
   if t.get('memory_events',{}).get('max',0)>0 or t.get('memory_events',{}).get('oom',0)>0:
    aborted='Trainer memory cap hit';command(['systemctl','--user','stop',unit]);break
   if t['unit'].get('SubState')=='exited' or t['unit'].get('ActiveState') in ('failed','inactive'):break
   time.sleep(.5)
  final=telemetry(unit);run_samples.append(final);after=snapshot()
  peaks=[int(x['unit'].get('MemoryPeak',0)) for x in run_samples if x['unit'].get('MemoryPeak','').isdigit()]
  peak=max([peak]+peaks)
  write(R/(label+'-telemetry.json'),{'samples':run_samples,'peak_bytes':peak,'aborted':aborted,'before':before,'after':after,'final_unit':final['unit']})
  write(R/(label+'-after.json'),after)
  result=parse_result(log.read_text()) if log.exists() else None
  # Always stop this trainer cgroup, including any lingering native children.
  command(['systemctl','--user','stop',unit]);current_unit=None
  if aborted or result is None or final['unit'].get('Result') not in ('success',):
   raise RuntimeError(f'{label} failed: {aborted or final["unit"]}; inspect {log}')
  if result['updates']!=3 or len(result['wave_records'])!=3:raise RuntimeError('Unequal learner wave count')
  result.update(label=label,configuration=name,groups=groups,subprocess_wall_s=time.monotonic()-run_start,command=shlex.join(launch),memory_peak_bytes=peak,unit=unit)
  path=R/(label+'.json');write(path,result)
  for fn in ['matches.jsonl','train.log','bench_waves.json']:
   shutil.copy2(scratch/fn,R/(label+'-'+fn))
  summary.append({'label':label,'configuration':name,'seed':seed,'result':str(path),'total_s':result['total_s'],'games_per_min':result['games_per_min'],'decisions_per_s':result['decisions_per_s'],'memory_peak_bytes':peak})
  write(R/'summary.json',summary)
  shutil.rmtree(scratch)
  print(json.dumps(summary[-1]),flush=True)
 write(R/'postflight.json',snapshot())
 write(R/'progress.json',{'status':'completed','completed_runs':6,'total_runs':6,'completed_waves':18,'total_waves':18,'work_done':'18/18 waves','eta_seconds':0,'elapsed_seconds':round(time.monotonic()-start,1)})
except BaseException as e:
 if current_unit:
  try:command(['systemctl','--user','stop',current_unit])
  except Exception:pass
 write(R/'progress.json',{'status':'aborted','error':str(e),'completed_runs':len(summary),'completed_waves':len(summary)*3,'total_waves':18,'eta_seconds':None})
 write(R/'error.json',{'error':str(e),'traceback':traceback.format_exc()})
 traceback.print_exc();sys.exit(1)
