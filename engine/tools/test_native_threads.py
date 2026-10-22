"""Focused Unicorn execution tests for the cooperative ARM64 thread model.

Run with an environment that has Unicorn installed:
    python tools/test_native_threads.py
"""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

try:
    from unicorn import Uc, UC_ARCH_ARM64, UC_MODE_ARM
    from unicorn.arm64_const import UC_ARM64_REG_PC, UC_ARM64_REG_X0, UC_ARM64_REG_X1
    from unicorn.arm64_const import UC_ARM64_REG_X16, UC_ARM64_REG_X17, UC_ARM64_REG_X18
    from unicorn.arm64_const import UC_ARM64_REG_X19, UC_ARM64_REG_X20, UC_ARM64_REG_X21
    from unicorn.arm64_const import UC_ARM64_REG_X22
    from unicorn import UC_HOOK_CODE
except ImportError as exc:  # Keep the dependency failure explicit and local.
    raise SystemExit("This test needs the 'unicorn' Python package") from exc

from native_threads import NativeThreads


CODE = 0x1000
MAIN = 0x1000
WORKER = 0x1100
RETURN = 0x1200
LOCK = 0x8000
WAIT = 0x8010
SIGNAL = 0x8020
UNLOCK = 0x8030
EXIT = 0x8040
MAIN_COND = 0x9000
MAIN_MUTEX = 0x9010
JOB_COND = 0x9020
JOB_MUTEX = 0x9030
SHARED = 0x9040
RESULT = 0x9050
DONE_COND = 0x9060
DONE_MUTEX = 0x9070


def movz_x(rd, value):
    return 0xD2800000 | ((value & 0xFFFF) << 5) | rd


def movz_w(rd, value):
    return 0x52800000 | ((value & 0xFFFF) << 5) | rd


def str_x(rt, rn, offset=0):
    assert offset % 8 == 0 and 0 <= offset // 8 < 4096
    return 0xF9000000 | ((offset // 8) << 10) | (rn << 5) | rt


def ldr_x(rt, rn, offset=0):
    assert offset % 8 == 0 and 0 <= offset // 8 < 4096
    return 0xF9400000 | ((offset // 8) << 10) | (rn << 5) | rt


def blr(rn):
    return 0xD63F0000 | (rn << 5)


def code(*words):
    return b"".join(word.to_bytes(4, "little") for word in words)


def make_program(uc):
    uc.mem_map(CODE, 0x4000)
    uc.mem_map(0x8000, 0x1000)
    uc.mem_map(0x9000, 0x1000)

    # Main sleeps until externally released, then publishes a value while
    # holding the job mutex, signals the worker, and exits.
    main = code(
        movz_x(0, MAIN_COND), movz_x(1, MAIN_MUTEX), blr(17),
        movz_x(0, JOB_MUTEX), blr(16),
        movz_x(20, SHARED), movz_x(9, 42), str_x(9, 20),
        movz_x(0, JOB_COND), blr(18),
        movz_x(0, JOB_MUTEX), blr(19),
        movz_x(0, DONE_COND), movz_x(1, DONE_MUTEX), blr(17),
        movz_x(9, RETURN), 0xD61F0000 | (9 << 5),
    )

    # The worker really executes lock -> condition wait -> load -> unlock.
    # The post-wait value is recorded only after mutex reacquisition.
    worker = code(
        movz_x(16, LOCK), movz_x(17, WAIT), movz_x(18, SIGNAL),
        movz_x(19, UNLOCK), movz_x(21, EXIT),
        movz_x(20, SHARED), movz_x(22, RESULT),
        movz_x(0, JOB_MUTEX), blr(16),
        movz_x(0, JOB_COND), movz_x(1, JOB_MUTEX), blr(17),
        ldr_x(9, 20), str_x(9, 22),
        movz_x(0, JOB_MUTEX), blr(19),
        movz_x(0, DONE_COND), blr(18),
        movz_w(0, 0), blr(21),
    )
    uc.mem_write(MAIN, main)
    uc.mem_write(WORKER, worker)
    uc.mem_write(RETURN, code(0xD65F03C0))
    uc.mem_write(SHARED, (0).to_bytes(8, "little"))
    uc.mem_write(RESULT, (0).to_bytes(8, "little"))


def make_scheduler(uc):
    scheduler = NativeThreads(uc, main_tls=0xA000, return_address=RETURN)
    scheduler.mutex_init(MAIN_MUTEX)
    scheduler.mutex_init(JOB_MUTEX)
    scheduler.mutex_init(DONE_MUTEX)
    scheduler.create(WORKER, 0)

    def on_import(emu, address, _size, _data):
        if address == LOCK:
            result = scheduler.mutex_lock(emu.reg_read(UC_ARM64_REG_X0))
            if result is not None:
                scheduler.return_from_import(result)
        elif address == UNLOCK:
            scheduler.return_from_import(scheduler.mutex_unlock(emu.reg_read(UC_ARM64_REG_X0)))
        elif address == WAIT:
            scheduler.cond_wait(emu.reg_read(UC_ARM64_REG_X0), emu.reg_read(UC_ARM64_REG_X1))
        elif address == SIGNAL:
            scheduler.cond_signal(emu.reg_read(UC_ARM64_REG_X0))
            scheduler.return_from_import(0)
        elif address == EXIT:
            scheduler.exit(emu.reg_read(UC_ARM64_REG_X0))

    uc.hook_add(UC_HOOK_CODE, on_import, begin=LOCK, end=LOCK)
    uc.hook_add(UC_HOOK_CODE, on_import, begin=WAIT, end=WAIT)
    uc.hook_add(UC_HOOK_CODE, on_import, begin=SIGNAL, end=SIGNAL)
    uc.hook_add(UC_HOOK_CODE, on_import, begin=UNLOCK, end=UNLOCK)
    uc.hook_add(UC_HOOK_CODE, on_import, begin=EXIT, end=EXIT)

    scheduler.worker_mutex_owners = []

    def observe_worker_after_wait(emu, _address, _size, _data):
        scheduler.worker_mutex_owners.append(scheduler.mutexes[JOB_MUTEX]["owner"])

    uc.hook_add(UC_HOOK_CODE, observe_worker_after_wait,
                begin=WORKER + 12 * 4, end=WORKER + 12 * 4)
    uc.reg_write(UC_ARM64_REG_X16, LOCK)
    uc.reg_write(UC_ARM64_REG_X17, WAIT)
    uc.reg_write(UC_ARM64_REG_X18, SIGNAL)
    uc.reg_write(UC_ARM64_REG_X19, UNLOCK)
    uc.reg_write(UC_ARM64_REG_X20, SHARED)
    uc.reg_write(UC_ARM64_REG_X21, EXIT)
    return scheduler


class NativeThreadsExecutionTests(unittest.TestCase):
    def test_checkpoint_restore_wakes_and_runs_worker_after_mutex_reacquire(self):
        uc = Uc(UC_ARCH_ARM64, UC_MODE_ARM)
        make_program(uc)
        scheduler = make_scheduler(uc)

        # Both genuine ARM64 routines block in their respective condition
        # waits. This also leaves their post-BLR PCs in saved CPU contexts.
        with self.assertRaisesRegex(RuntimeError, "deadlocked at OS waits"):
            scheduler.emulate(MAIN, RETURN, timeout=1_000_000)
        self.assertEqual(scheduler.threads[0]["state"], "condition")
        self.assertEqual(scheduler.threads[1]["state"], "condition")

        checkpoint = scheduler.snapshot()
        scheduler.restore(checkpoint)
        resumed = scheduler

        # An external condition notification makes main runnable and gives it
        # its mutex. Main then signals the worker while holding JOB_MUTEX;
        # the worker must remain parked until the main unlock transfers it.
        resumed.cond_signal(MAIN_COND)
        self.assertEqual(resumed.threads[0]["state"], "runnable")
        self.assertEqual(resumed.mutexes[MAIN_MUTEX]["owner"], 0)

        resumed.current = 0
        uc.context_restore(resumed.threads[0]["context"])
        try:
            resumed.emulate(uc.reg_read(UC_ARM64_REG_PC), RETURN, timeout=1_000_000)
        except RuntimeError as exc:
            self.fail(f"scheduler stopped early: {exc}; {resumed.summary()}; pc={hex(uc.reg_read(UC_ARM64_REG_PC))}")

        actual = int.from_bytes(uc.mem_read(RESULT, 8), "little")
        self.assertEqual(actual, 42, (resumed.summary(), hex(uc.reg_read(UC_ARM64_REG_PC))))
        self.assertEqual(resumed.threads[1]["state"], "exited")
        self.assertEqual(resumed.mutexes[JOB_MUTEX]["owner"], None)
        self.assertTrue(any(event.get("wake") == 1 for event in resumed.events))
        self.assertEqual(resumed.worker_mutex_owners, [1])


if __name__ == "__main__":
    unittest.main(verbosity=2)
