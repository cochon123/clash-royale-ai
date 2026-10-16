"""Bounded local restart protection for the already authorized overnight queue."""
from datetime import datetime,timezone
import json
from pathlib import Path
import subprocess
import time
from .controlled_experiment import atomic_json


def check(output="models/policy_v9",now=None):
    now=time.time() if now is None else now
    end=datetime.fromisoformat("2026-10-12T06:00:00-06:00").timestamp()
    root=Path(output)
    if now>=end-120:
        subprocess.run(["systemctl","--user","stop","cr-v9-recovery.timer"],check=False)
        return "deadline"
    active=subprocess.run(["systemctl","--user","is-active","cr-v9-suite.service"],capture_output=True,text=True).stdout.strip()
    if active!="failed":return active
    # At the training cutoff only a frozen final evaluation may be resumed.
    if now>=end-2700 and not (root/"final_selection.json").exists():return "training_cutoff"
    path=root/"recovery_attempts.json"
    attempts=json.loads(path.read_text()) if path.exists() else []
    if len(attempts)>=2:return "restart_limit"
    attempts.append(dict(at=datetime.fromtimestamp(now,timezone.utc).isoformat(),reason="suite service failed"))
    atomic_json(path,attempts)
    subprocess.run(["systemctl","--user","stop","lettera-qwen-asr.service","comfyui.service"],check=True)
    subprocess.run(["systemctl","--user","mask","--runtime","lettera-qwen-asr.service"],check=True)
    subprocess.run(["systemctl","--user","restart","cr-v9-suite.service"],check=True)
    print(f"watchdog resumed saved queue; work={len(attempts)}/2 restart attempts; ETA={max(0,end-now):.0f}s",flush=True)
    return "restarted"


if __name__=="__main__":check()
