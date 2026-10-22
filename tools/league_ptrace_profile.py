#!/usr/bin/env python3
"""Sample host CPU time of a league engine without perf, using ptrace.

perf_event_paranoid blocks perf for unprivileged users here, but Yama still
lets a parent trace its own child. A sampler thread spawns the engine, seizes
it, and periodically interrupts it to read RIP (and, for binaries built with
-C force-frame-pointers=yes, the RBP frame chain). The main thread replays
recorded development traces over the normal JSON protocol, so the sampled
workload is exactly the benchmark workload. Every reply is still checked.
"""
from __future__ import annotations
import argparse
import bisect
import collections
import ctypes
import ctypes.util
import gzip
import json
import os
from pathlib import Path
import selectors
import signal
import subprocess
import threading
import sys
import time

from league_validation import CHECKPOINT, ROOT, semantic

PTRACE_CONT, PTRACE_GETREGS, PTRACE_SEIZE, PTRACE_INTERRUPT = 7, 12, 0x4206, 0x4207
PTRACE_EVENT_STOP = 128
libc = ctypes.CDLL(ctypes.util.find_library("c"), use_errno=True)
libc.ptrace.restype = ctypes.c_long
libc.ptrace.argtypes = [ctypes.c_long, ctypes.c_int, ctypes.c_void_p, ctypes.c_void_p]


class Regs(ctypes.Structure):
    _fields_ = [(n, ctypes.c_ulonglong) for n in (
        "r15 r14 r13 r12 rbp rbx r11 r10 r9 r8 rax rcx rdx rsi rdi orig_rax rip cs eflags rsp ss "
        "fs_base gs_base ds es fs gs").split()]


class Sampler(threading.Thread):
    def __init__(self, binary, log, hz, depth):
        super().__init__(daemon=True)
        self.binary, self.log, self.period, self.depth = binary, log, 1 / hz, depth
        self.ready = threading.Event()
        self.stop = threading.Event()
        self.samples = []
        self.active = threading.Event()
        self.error = None

    def run(self):
        try:
            self._run()
        except BaseException as error:  # surfaced by the driver
            self.error = error
            self.ready.set()

    def _run(self):
        self.err = open(self.log, "ab")
        self.proc = subprocess.Popen([str(self.binary), "--checkpoint", str(CHECKPOINT)], stdin=subprocess.PIPE,
                                     stdout=subprocess.PIPE, stderr=self.err, bufsize=0, cwd=ROOT)
        pid = self.proc.pid
        if libc.ptrace(PTRACE_SEIZE, pid, None, None) != 0:
            raise OSError(ctypes.get_errno(), "PTRACE_SEIZE failed")
        mem = open(f"/proc/{pid}/mem", "rb", buffering=0)
        self.ready.set()
        regs = Regs()
        status = ctypes.c_int()
        while not self.stop.is_set():
            time.sleep(self.period)
            if not self.active.is_set():
                # A seized tracee stops on every signal (e.g. SIGCHLD from its
                # startup hash subprocesses); keep re-injecting until sampling.
                while libc.waitpid(pid, ctypes.byref(status), 0x40000000 | 1) > 0:  # __WALL|WNOHANG
                    s = status.value
                    if os.WIFEXITED(s) or os.WIFSIGNALED(s):
                        return
                    if os.WIFSTOPPED(s):
                        sig = os.WSTOPSIG(s)
                        libc.ptrace(PTRACE_CONT, pid, None, ctypes.c_void_p(0 if sig == signal.SIGTRAP else sig))
                continue
            if libc.ptrace(PTRACE_INTERRUPT, pid, None, None) != 0:
                break
            while True:
                if libc.waitpid(pid, ctypes.byref(status), 0x40000000) < 0:  # __WALL
                    return
                s = status.value
                if os.WIFEXITED(s) or os.WIFSIGNALED(s):
                    return
                sig = os.WSTOPSIG(s)
                if sig == signal.SIGTRAP and (s >> 16) == PTRACE_EVENT_STOP:
                    break
                libc.ptrace(PTRACE_CONT, pid, None, ctypes.c_void_p(sig))  # re-inject real signals
            libc.ptrace(PTRACE_GETREGS, pid, None, ctypes.byref(regs))
            if regs.orig_rax != 2**64 - 1:  # blocked in (or entering) a syscall, e.g. waiting on stdin
                self.samples.append([f"syscall:{regs.orig_rax}"])
                libc.ptrace(PTRACE_CONT, pid, None, None)
                continue
            stack = [regs.rip]
            fp = regs.rbp
            for _ in range(self.depth):
                if not fp or fp & 7:
                    break
                try:
                    mem.seek(fp)
                    frame = mem.read(16)
                except OSError:
                    break
                if len(frame) != 16:
                    break
                next_fp, ret = int.from_bytes(frame[:8], "little"), int.from_bytes(frame[8:], "little")
                if not ret or next_fp <= fp:
                    stack.append(ret)
                    break
                stack.append(ret)
                fp = next_fp
            self.samples.append(stack)
            libc.ptrace(PTRACE_CONT, pid, None, None)


def load_symbols(binary, pid):
    base = None
    maps = []
    for line in open(f"/proc/{pid}/maps"):
        parts = line.split()
        lo, hi = (int(x, 16) for x in parts[0].split("-"))
        maps.append((lo, hi, parts[5] if len(parts) >= 6 else "[anon]"))
        if base is None and len(parts) >= 6 and os.path.realpath(parts[5]) == os.path.realpath(binary) and parts[2] == "00000000":
            base = lo
    nm = Path(subprocess.check_output(["rustc", "--print", "sysroot"], text=True).strip()) / \
        "lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-nm"
    rows = []
    for line in subprocess.check_output([str(nm), "--defined-only", "--numeric-sort", "--demangle", str(binary)],
                                        text=True).splitlines():
        parts = line.split(None, 2)
        if len(parts) == 3 and parts[1] in "tTwW":
            rows.append((int(parts[0], 16) + base, parts[2]))
    text_end = max(hi for lo, hi, path in maps if os.path.realpath(path) == os.path.realpath(binary))
    return [a for a, _ in rows], [n for _, n in rows], maps, text_end


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--binary", type=Path, required=True)
    p.add_argument("--traces", type=Path, default=ROOT / "analysis/fast-league-worktree-20261022/development")
    p.add_argument("--ids", type=int, nargs="+", default=[5, 11, 17, 23, 29, 30])
    p.add_argument("--rounds", type=int, default=2)
    p.add_argument("--hz", type=float, default=1500)
    p.add_argument("--depth", type=int, default=64)
    p.add_argument("--top", type=int, default=40)
    p.add_argument("--out", type=Path, required=True)
    a = p.parse_args()
    a.out.parent.mkdir(parents=True, exist_ok=True)
    sys.setswitchinterval(0.0002)
    traces = [json.load(gzip.open(a.traces / f"{i:04d}.json.gz", "rt")) for i in a.ids]
    sampler = Sampler(a.binary.resolve(), a.out.with_suffix(".engine.log"), a.hz, a.depth)
    sampler.start()
    sampler.ready.wait()
    if sampler.error:
        raise sampler.error
    proc = sampler.proc
    sel = selectors.DefaultSelector()
    sel.register(proc.stdout, selectors.EVENT_READ)
    pending = b""

    def request(obj):
        nonlocal pending
        proc.stdin.write(json.dumps(obj, separators=(",", ":")).encode() + b"\n")
        while b"\n" not in pending:
            sel.select(120)
            data = os.read(proc.stdout.fileno(), 65536)
            if not data:
                raise RuntimeError("engine exited")
            pending += data
        line, pending = pending.split(b"\n", 1)
        reply = json.loads(line)
        if reply.get("ok") is not True:
            raise RuntimeError(str(reply))
        return reply["result"]

    symbols = load_symbols(a.binary.resolve(), proc.pid)
    request(traces[0]["records"][0]["request"])  # warm reset; startup excluded
    sampler.active.set()
    started = time.perf_counter()
    for _ in range(a.rounds):
        for trace in traces:
            for record in trace["records"]:
                if semantic(request(record["request"])) != record["expected"]:
                    raise RuntimeError(f"replay differs in game {trace['job']['id']}")
    wall = time.perf_counter() - started
    sampler.stop.set()
    request({"op": "close"})
    sampler.join(timeout=30)
    addrs, names, maps, text_end = symbols

    def name(ip):
        if isinstance(ip, str):
            return ip
        if ip >= text_end or ip < addrs[0]:
            return next((Path(path).name for lo, hi, path in maps if lo <= ip < hi), f"?{ip:#x}")
        i = bisect.bisect_right(addrs, ip) - 1
        return names[i] if i >= 0 else f"?{ip:#x}"

    self_counts = collections.Counter()
    incl_counts = collections.Counter()
    for stack in sampler.samples:
        frames = [name(ip) for ip in stack]
        self_counts[frames[0]] += 1
        for f in set(frames):
            incl_counts[f] += 1
    n = len(sampler.samples)
    busy = sum(not isinstance(s[0], str) for s in sampler.samples)
    report = dict(binary=str(a.binary), samples=n, busy_samples=busy, wall_seconds=wall, hz=a.hz,
                  games=a.rounds * len(traces),
                  self=[dict(symbol=s, samples=c, share=c / n) for s, c in self_counts.most_common(a.top)],
                  inclusive=[dict(symbol=s, samples=c, share=c / n) for s, c in incl_counts.most_common(a.top)])
    a.out.write_text(json.dumps(report, indent=2) + "\n")
    print(f"{n} samples ({busy} on-CPU) over {wall:.1f}s, {report['games']} games")
    for row in report["self"][:a.top]:
        print(f"self {row['share']*100:6.2f}%  {row['symbol'][:110]}")
    if any(len(s) > 1 for s in sampler.samples):
        for row in report["inclusive"][:a.top]:
            print(f"incl {row['share']*100:6.2f}%  {row['symbol'][:110]}")


if __name__ == "__main__":
    main()
