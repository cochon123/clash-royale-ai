"""Cooperative POSIX boundary for executing original ARM64 worker routines.

One virtual CPU switches only at blocking OS calls. Native jobs, queue code,
and atomics run unchanged. Busy-wait scheduling and timed waits are unsupported.
"""
import base64
import ctypes
import hashlib
import struct
import time

from unicorn.arm64_const import *


class NativeThreads:
    STACK_SIZE = 0x400000

    def __init__(self, uc, main_tls, return_address):
        self.uc = uc
        self.return_address = return_address
        self.current = 0
        self.next_id = 1
        self.threads = {0: {'state': 'runnable', 'tls': main_tls, 'context': None}}
        self.mutexes = {}
        self.events = []
        self.switch_requested = False

    @property
    def tls(self):
        return self.threads[self.current]['tls']

    def create(self, start, argument):
        if self.next_id > 32:
            raise RuntimeError('Virtual worker limit reached')
        tid = self.next_id
        self.next_id += 1
        base = 0xc0000000 + tid * 0x800000
        self.uc.mem_map(base, self.STACK_SIZE)
        tls = base + self.STACK_SIZE
        self.uc.mem_map(tls, 0x1000)
        parent = self.uc.context_save()
        for reg in range(31):
            self.uc.reg_write(globals()[f'UC_ARM64_REG_X{reg}'], 0)
        for reg in range(32):
            self.uc.reg_write(globals()[f'UC_ARM64_REG_Q{reg}'], 0)
        self.uc.reg_write(UC_ARM64_REG_PC, start)
        self.uc.reg_write(UC_ARM64_REG_SP, base + self.STACK_SIZE - 0x1000)
        self.uc.reg_write(UC_ARM64_REG_TPIDR_EL0, tls)
        self.uc.reg_write(UC_ARM64_REG_X0, argument)
        self.uc.reg_write(UC_ARM64_REG_X30, self.return_address)
        self.threads[tid] = {'state': 'runnable', 'tls': tls, 'stack': base,
                             'start': start, 'argument': argument,
                             'context': self.uc.context_save()}
        self.uc.context_restore(parent)
        self.events.append({'create': tid, 'start': hex(start), 'argument': hex(argument)})
        return tls + 0x100

    def return_from_import(self, result=0):
        self.uc.reg_write(UC_ARM64_REG_X0, result & ((1 << 64) - 1))
        self.uc.reg_write(UC_ARM64_REG_PC, self.uc.reg_read(UC_ARM64_REG_X30))

    def block(self, state, **fields):
        self.threads[self.current].update(state=state, **fields)
        self.switch_requested = True
        self.uc.emu_stop()

    def mutex_init(self, address, kind=0):
        self.mutexes[address] = {'owner': None, 'depth': 0, 'kind': kind}

    def mutex_lock(self, address):
        mutex = self.mutexes.setdefault(address, {'owner': None, 'depth': 0, 'kind': 0})
        if mutex['owner'] is None:
            mutex.update(owner=self.current, depth=1)
            return 0
        if mutex['owner'] == self.current:
            if mutex['kind'] == 1:
                mutex['depth'] += 1
                return 0
            if mutex['kind'] == 2:
                return 35
        # The import is retried after the owning thread releases the mutex.
        self.block('mutex', mutex=address)
        return None

    def mutex_unlock(self, address):
        mutex = self.mutexes.setdefault(address, {'owner': None, 'depth': 0, 'kind': 0})
        if mutex['owner'] not in (None, self.current):
            raise RuntimeError('Virtual mutex unlocked by a different thread')
        if mutex['depth'] > 1:
            mutex['depth'] -= 1
            return 0
        mutex.update(owner=None, depth=0)
        for tid, thread in self.threads.items():
            if thread['state'] in ('mutex', 'reacquire') and thread.get('mutex') == address:
                if thread['state'] == 'reacquire':
                    mutex.update(owner=tid, depth=1)
                thread['state'] = 'runnable'
                break
        return 0

    def cond_wait(self, condition, mutex):
        self.mutex_unlock(mutex)
        self.return_from_import()
        self.events.append({'wait': self.current, 'condition': hex(condition), 'mutex': hex(mutex)})
        self.block('condition', condition=condition, mutex=mutex)

    def cond_signal(self, condition, broadcast=False):
        for tid, thread in self.threads.items():
            if thread['state'] == 'condition' and thread.get('condition') == condition:
                mutex = self.mutexes[thread['mutex']]
                if mutex['owner'] is None:
                    mutex.update(owner=tid, depth=1)
                    thread['state'] = 'runnable'
                else:
                    thread['state'] = 'reacquire'
                self.events.append({'wake': tid, 'condition': hex(condition)})
                if not broadcast:
                    break

    def exit(self, value):
        self.threads[self.current].update(state='exited', exit_value=value)
        self.events.append({'exit': self.current, 'value': hex(value)})
        for thread in self.threads.values():
            if thread['state'] == 'join' and thread['join'] == self.current:
                if thread['result_pointer']:
                    self.uc.mem_write(thread['result_pointer'], struct.pack('<Q', value))
                thread['state'] = 'runnable'
        self.switch_requested = True
        self.uc.emu_stop()

    def join(self, identity, result_pointer):
        target = next((tid for tid, t in self.threads.items() if t['tls'] + 0x100 == identity), None)
        if target is None:
            return 3
        if target == self.current:
            return 35
        thread = self.threads[target]
        if thread['state'] == 'exited':
            if result_pointer:
                self.uc.mem_write(result_pointer, struct.pack('<Q', thread['exit_value']))
            return 0
        self.return_from_import()
        self.block('join', join=target, result_pointer=result_pointer)
        return None

    def emulate(self, begin, until, timeout, count=0):
        deadline = time.monotonic() + timeout / 1_000_000
        self.uc.reg_write(UC_ARM64_REG_PC, begin)
        while True:
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                return
            self.switch_requested = False
            self.uc.emu_start(self.uc.reg_read(UC_ARM64_REG_PC), until,
                              timeout=max(1, int(remaining * 1_000_000)), count=count)
            if not self.switch_requested:
                if self.current and self.uc.reg_read(UC_ARM64_REG_PC) == until:
                    raise RuntimeError('Native worker reached the main call return sentinel')
                return
            self.threads[self.current]['context'] = self.uc.context_save()
            candidates = [tid for tid, t in self.threads.items() if t['state'] == 'runnable']
            if not candidates:
                raise RuntimeError('Native virtual threads deadlocked at OS waits')
            # Round robin among runnable threads at each blocking boundary.
            following = [tid for tid in candidates if tid > self.current]
            selected = min(following or candidates)
            self.events.append({'switch': [self.current, selected]})
            self.current = selected
            self.uc.context_restore(self.threads[selected]['context'])

    def regions(self):
        return [(f'thread_{tid:03d}_stack', t['stack'], self.STACK_SIZE)
                for tid, t in self.threads.items() if tid] + [
                    (f'thread_{tid:03d}_tls', t['tls'], 0x1000)
                    for tid, t in self.threads.items() if tid]

    def snapshot(self):
        self.threads[self.current]['context'] = self.uc.context_save()
        rows = {}
        for tid, thread in self.threads.items():
            raw = bytes(thread['context'])
            rows[str(tid)] = {**{k: v for k, v in thread.items() if k != 'context'},
                              'context_b64': base64.b64encode(raw).decode(),
                              'context_sha256': hashlib.sha256(raw).hexdigest()}
        return {'current': self.current, 'next_id': self.next_id, 'threads': rows,
                'mutexes': self.mutexes, 'events': self.events}

    def restore(self, state):
        self.current = state['current']
        self.next_id = state['next_id']
        self.events = state['events']
        self.mutexes = {int(k): v for k, v in state['mutexes'].items()}
        self.threads = {}
        for tid, row in state['threads'].items():
            thread = {k: v for k, v in row.items() if not k.startswith('context_')}
            raw = base64.b64decode(row['context_b64'])
            context = self.uc.context_save()
            if len(raw) != context.size or hashlib.sha256(raw).hexdigest() != row['context_sha256']:
                raise ValueError('Worker CPU context identity mismatch')
            ctypes.memmove(context.context, raw, len(raw))
            thread['context'] = context
            self.threads[int(tid)] = thread

    def summary(self):
        return {'model': 'one virtual CPU, cooperative switching at blocking OS imports; native callbacks/jobs execute unchanged; busy waits and timed waits unsupported',
                'current': self.current,
                'threads': {str(k): {n: v for n, v in t.items() if n != 'context'}
                            for k, t in self.threads.items()}, 'events': self.events}
