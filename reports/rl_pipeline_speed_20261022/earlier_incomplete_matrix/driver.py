import os,sys,json,time,subprocess,shlex,shutil,datetime,hashlib
from pathlib import Path
ROOT=Path('/tmp/cr-prof/bench')
PYTHON='/home/cochon/Documents/mini-project/clash-royale-ai/.venv/bin/python'
BASE='/home/cochon/Documents/mini-project/clash-royale-ai/engine'
NEW='/home/cochon/Documents/mini-project/clash-royale-ai-worktrees/rl-speed-integration/engine'
STAMP=datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
ROOT.joinpath('session.txt').write_text(STAMP+'\n')
def command(cmd,cwd=None):
    r=subprocess.run(cmd,cwd=cwd,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
    return {'command':shlex.join(cmd),'returncode':r.returncode,'output':r.stdout}
def snapshot():
    cmds=[['pgrep','-fa','rust_league_rl|bench_wave|banana_'],['uptime'],['free','-g'],['nvidia-smi'],['ps','-p','2578,2613','-o','pid,stat,pcpu,etime,args']]
    return {'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'checks':[command(x) for x in cmds], 'power':{str(p):p.read_text().strip() for p in Path('/sys/class/power_supply').glob('*/online')}}
pre=snapshot()
pre['repos']={name:{'head':command(['git','rev-parse','HEAD'],path),'status':command(['git','status','--short'],path)} for name,path in [('baseline',BASE),('new',NEW)]}
pre['inputs']={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [Path(BASE,'runs/banana-hog-2/bin/rust_league_rl'),Path(BASE,'runs/banana-hog-2/pool.json'),Path(BASE,'runs/banana-hog-1/models/final.pt')]}
ROOT.joinpath(f'{STAMP}-preflight.json').write_text(json.dumps(pre,indent=2))
configs=[('baseline12',BASE,12,None),('new12g1',NEW,12,1),('new12g2',NEW,12,2),('baseline16',BASE,16,None),('new16g1',NEW,16,1),('new16g2',NEW,16,2)]
# Reverse round two to counterbalance ordering/thermal drift as well as changing the seed.
schedule=[(777,c) for c in configs]+[(2026,c) for c in reversed(configs)]
summary=[]
start=time.monotonic()
for index,(seed,(name,code,workers,groups)) in enumerate(schedule):
    label=f'{STAMP}-{index+1:02d}-{name}-s{seed}'
    mem=command(['free','-g'])
    available=int(next(l for l in mem['output'].splitlines() if l.startswith('Mem:')).split()[-1])
    before=snapshot()
    ROOT.joinpath(label+'-before.json').write_text(json.dumps(before,indent=2))
    if workers==16 and available<11:
        summary.append({'label':label,'skipped':True,'available_gib':available})
        ROOT.joinpath('summary.json').write_text(json.dumps(summary,indent=2))
        continue
    scratch=ROOT/(label+'-scratch')
    cmd=[PYTHON,str(ROOT/'bench_wave_measured.py'),'--code-root',code,'--workers',str(workers),'--waves','3','--seed',str(seed),'--run-dir',str(scratch)]
    if groups is not None: cmd+=['--extra',f'--step-groups {groups}']
    shellcmd='cd '+shlex.quote(code)+' && PYTHONDONTWRITEBYTECODE=1 '+shlex.join(cmd)
    with ROOT.joinpath('commands.txt').open('a') as f: f.write(shellcmd+'\n')
    env=dict(os.environ,PYTHONDONTWRITEBYTECODE='1',PYTHONUNBUFFERED='1')
    t=time.monotonic(); logpath=ROOT/(label+'.stdout.log')
    telemetry=[]; nextsample=0
    with logpath.open('w') as out:
        proc=subprocess.Popen(cmd,cwd=code,env=env,stdout=out,stderr=subprocess.STDOUT)
        while proc.poll() is None:
            elapsed=time.monotonic()-t
            waves=[]
            try: waves=json.loads((scratch/'bench_waves.json').read_text())
            except (FileNotFoundError,json.JSONDecodeError): pass
            estimate=(time.monotonic()-start)/max(1,index+len(waves)/3)*(len(schedule)-index-len(waves)/3) if index or waves else None
            status={'run':index+1,'total_runs':len(schedule),'configuration':name,'seed':seed,'pid':proc.pid,'waves_done':len(waves),'waves_total':3,'elapsed_run_s':round(elapsed,1),'eta_total_s':round(estimate) if estimate else None,'completed_runs':index,'work_done':f'{index*3+len(waves)}/36 waves'}
            ROOT.joinpath('progress.json').write_text(json.dumps(status,indent=2))
            if elapsed>=nextsample:
                telemetry.append({'elapsed_s':elapsed,'gpu':command(['nvidia-smi','--query-gpu=timestamp,temperature.gpu,utilization.gpu,memory.used,power.draw,clocks.sm','--format=csv,noheader'])['output'].strip(),'memory':command(['free','-g'])['output']})
                nextsample=elapsed+10
            time.sleep(1)
    wall=time.monotonic()-t
    raw=logpath.read_text(); result=None
    for pos,ch in enumerate(raw):
        if ch=='{':
            try:
                obj,end=json.JSONDecoder().raw_decode(raw[pos:])
                if isinstance(obj,dict) and 'code_root' in obj and 'total_s' in obj: result=obj
            except json.JSONDecodeError: pass
    ROOT.joinpath(label+'-telemetry.json').write_text(json.dumps(telemetry,indent=2))
    if result is None or proc.returncode:
        summary.append({'label':label,'failed':True,'exit_code':proc.returncode,'wall_s':wall,'log':str(logpath)})
        ROOT.joinpath('summary.json').write_text(json.dumps(summary,indent=2))
        raise RuntimeError(f'Failed {label}; inspect {logpath}')
    result.update(label=label,configuration=name,groups=groups,subprocess_wall_s=wall,command=shellcmd,available_before_gib=available)
    resultpath=ROOT/(label+'.json');resultpath.write_text(json.dumps(result,indent=2))
    outcomes=[json.loads(line) for line in (scratch/'matches.jsonl').read_text().splitlines() if line]
    ROOT.joinpath(label+'-outcomes.json').write_text(json.dumps(outcomes,indent=2))
    shutil.copy2(scratch/'train.log',ROOT/(label+'-train.log'))
    summary.append({'label':label,'configuration':name,'seed':seed,'result':str(resultpath),'total_s':result['total_s'],'games_per_min':result['games_per_min'],'decisions_per_s':result['decisions_per_s']})
    ROOT.joinpath('summary.json').write_text(json.dumps(summary,indent=2))
    shutil.rmtree(scratch)
    print(json.dumps(summary[-1]),flush=True)
post=snapshot(); post['repos']={name:command(['git','status','--short'],path) for name,path in [('baseline',BASE),('new',NEW)]}
ROOT.joinpath(f'{STAMP}-postflight.json').write_text(json.dumps(post,indent=2))
ROOT.joinpath('progress.json').write_text(json.dumps({'status':'completed','completed_runs':len(summary),'work_done':'36/36 waves','elapsed_s':time.monotonic()-start},indent=2))
