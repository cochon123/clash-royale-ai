#!/usr/bin/env python3
"""Run one bounded instrumentation experiment in the project Android AVD."""
import argparse
import json
import re
from pathlib import Path
import time

import frida
import android_runtime as runtime

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('script',type=Path)
    parser.add_argument('--seconds',type=float,default=15)
    parser.add_argument('--realm',choices=['native','emulated'],default='native')
    parser.add_argument('--pid',type=int)
    parser.add_argument('--arm',action='store_true',help='use the genuine ARM64 AVD on port 5582')
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args()
    if args.arm:
        import android_arm_runtime  # Selects the isolated ARM64 profile.
    runtime.assert_ours()
    args.output.parent.mkdir(parents=True,exist_ok=True)
    device=frida.get_device(runtime.SERIAL,timeout=10)
    pid=args.pid or device.spawn([runtime.PACKAGE])
    with args.output.open('w',buffering=1) as output:
        def record(event):
            line=json.dumps({'wall_time':time.time(),**event},default=str)
            output.write(line+'\n');print(line,flush=True)
        record({'event':'attach','pid':pid,'realm':args.realm,'script':str(args.script)})
        session=device.attach(pid,realm=args.realm)
        session.on('detached',lambda reason,crash:record({'event':'detached','reason':reason,'crash':str(crash)}))
        source=args.script.read_text()
        if int(frida.__version__.split('.')[0]) >= 17 and re.search(r'\bJava\s*[.\[]', source):
            # Frida 17 keeps the Java bridge in frida-tools rather than in Gum.
            import frida_tools
            bridge=Path(frida_tools.__file__).parent/'bridges/java.js'
            source=bridge.read_text()+'\nconst Java=bridge;\n'+source
        script=session.create_script(source)
        script.on('message',lambda message,data:record({'event':'message','message':message,'bytes':len(data) if data else 0}))
        script.load()
        if not args.pid:device.resume(pid)
        deadline=time.monotonic()+args.seconds
        while time.monotonic()<deadline and not session.is_detached:time.sleep(.1)
        if not session.is_detached:session.detach()

if __name__=='__main__':main()
