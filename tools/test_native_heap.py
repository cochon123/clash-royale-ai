"""Focused allocator tests using real Unicorn guest memory."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

try:
    from unicorn import Uc, UC_ARCH_ARM64, UC_MODE_ARM
except ImportError as exc:
    raise SystemExit("This test needs the 'unicorn' Python package") from exc

from native_heap import GuestHeap


BASE = 0x50000000


def mapped_uc(base=BASE, map_size=0x1000):
    uc = Uc(UC_ARCH_ARM64, UC_MODE_ARM)
    uc.mem_map(base, map_size)
    return uc


class GuestHeapTests(unittest.TestCase):
    def test_reuse_size_classes_and_honor_alignment(self):
        uc = mapped_uc()
        heap = GuestHeap(uc, base=BASE, capacity=0x1000)
        filler = heap.allocate(8)
        first = heap.allocate(17)
        second = heap.allocate(17, alignment=32)
        self.assertEqual(first % 16, 0)
        self.assertEqual(second % 32, 0)
        uc.mem_write(first, b"first-block")
        uc.mem_write(second, b"second-block")
        heap.free(first)
        heap.free(second)

        aligned = heap.allocate(17, alignment=64)
        self.assertEqual(aligned, second)
        self.assertEqual(aligned % 64, 0)
        self.assertEqual(bytes(uc.mem_read(aligned, 12)), b"second-block")
        different_class = heap.allocate(33)
        self.assertNotIn(different_class, (first, second))
        self.assertEqual(heap.allocations[aligned], 17)
        self.assertIn(filler, heap.allocations)

        heap.free(0)
        with self.assertRaisesRegex(ValueError, "unknown or already freed"):
            heap.free(first)
        with self.assertRaisesRegex(ValueError, "unknown or already freed"):
            heap.free(first)

    def test_primary_bounds_are_checked_before_pointer_moves(self):
        uc = mapped_uc()
        heap = GuestHeap(uc, base=BASE, capacity=16)
        before = heap.primary_pointer
        with self.assertRaises(MemoryError):
            heap.allocate(GuestHeap.EXTRA_CAPACITY + 1)
        self.assertEqual(heap.primary_pointer, before)
        self.assertEqual(heap.extra_heaps, [])
        with self.assertRaises(ValueError):
            heap.allocate(8, alignment=12)

    def test_extra_arena_and_snapshot_restore_continue_allocation(self):
        uc1 = mapped_uc()
        heap1 = GuestHeap(uc1, base=BASE, capacity=64)
        primary = heap1.allocate(64)
        extra = heap1.allocate(24)
        self.assertEqual(primary, BASE)
        self.assertEqual(extra, GuestHeap.EXTRA_BASE)
        uc1.mem_write(extra, b"checkpoint-data")
        name, region_base, used = heap1.regions()[0]
        self.assertEqual(name, "heap_extra_00")
        self.assertEqual(region_base, GuestHeap.EXTRA_BASE)
        self.assertEqual(used, 32)

        state = heap1.snapshot()
        self.assertEqual(heap1.used_bytes, 96)
        uc2 = mapped_uc()
        for _name, base, _size in heap1.regions():
            uc2.mem_map(base, GuestHeap.EXTRA_CAPACITY)
        uc2.mem_write(extra, bytes(uc1.mem_read(extra, used)))
        heap2 = GuestHeap(uc2, base=BASE, capacity=64)
        heap2.restore(state)
        self.assertEqual(bytes(uc2.mem_read(extra, 15)), b"checkpoint-data")

        continued = heap2.allocate(16)
        self.assertEqual(continued, extra + 32)
        uc2.mem_write(continued, b"after-restore")
        self.assertEqual(bytes(uc2.mem_read(continued, 13)), b"after-restore")
        self.assertEqual(heap2.summary()["allocation_count"], 3)

    def test_legacy_restore_state_without_free_buckets_or_extras(self):
        uc = mapped_uc()
        heap = GuestHeap(uc, base=BASE, capacity=0x1000)
        heap.restore({"primary_pointer": hex(BASE + 32),
                      "allocations": {str(BASE): 24}})
        self.assertEqual(heap.allocate(8), BASE + 32)
        self.assertEqual(heap.allocations[BASE], 24)


if __name__ == "__main__":
    unittest.main(verbosity=2)
