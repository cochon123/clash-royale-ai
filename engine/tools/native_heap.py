"""Checkpointable guest allocator for native ARM64 execution."""


class GuestHeap:
    EXTRA_BASE = 0x200000000
    EXTRA_STEP = 0x20000000
    EXTRA_CAPACITY = 0x10000000
    MAX_EXTRAS = 4

    def __init__(self, uc, base=0x50000000, capacity=0x10000000):
        if base < 0 or capacity <= 0:
            raise ValueError("heap base and capacity must be positive guest ranges")
        if base + capacity > 1 << 64:
            raise ValueError("primary heap range exceeds guest address space")
        self.uc = uc
        self.base = base
        self.capacity = capacity
        self.primary_pointer = base
        self.allocations = {}
        self.free_buckets = {}
        self.extra_heaps = []

    @staticmethod
    def _size_class(size):
        if not isinstance(size, int) or size < 0:
            raise ValueError("allocation size must be a nonnegative integer")
        return max(16, (size + 15) & ~15)

    @staticmethod
    def _validate_alignment(alignment):
        if not isinstance(alignment, int) or alignment < 8 or alignment & (alignment - 1):
            raise ValueError("alignment must be a power of two and at least 8")

    @staticmethod
    def _align(value, alignment):
        return (value + alignment - 1) & -alignment

    @staticmethod
    def _int(value):
        return int(value, 0) if isinstance(value, str) else int(value)

    @property
    def used_bytes(self):
        return self.primary_pointer - self.base + sum(
            arena["pointer"] - arena["base"] for arena in self.extra_heaps
        )

    def _arena_for(self, pointer):
        if self.base <= pointer < self.base + self.capacity:
            return self.base, self.capacity, None
        for arena in self.extra_heaps:
            if arena["base"] <= pointer < arena["base"] + arena["capacity"]:
                return arena["base"], arena["capacity"], arena
        return None

    def _allocate_bump(self, size_class, alignment):
        arenas = [(self.base, self.capacity, self.primary_pointer, None)]
        arenas.extend((a["base"], a["capacity"], a["pointer"], a)
                      for a in self.extra_heaps)
        for base, capacity, pointer, arena in arenas:
            aligned = self._align(pointer, alignment)
            end = aligned + size_class
            if end > base + capacity:
                continue
            if arena is None:
                self.primary_pointer = end
            else:
                arena["pointer"] = end
            return aligned

        if len(self.extra_heaps) >= self.MAX_EXTRAS:
            raise MemoryError("guest heap exhausted its primary and four extra arenas")
        index = len(self.extra_heaps)
        extra_base = self.EXTRA_BASE + index * self.EXTRA_STEP
        if extra_base + self.EXTRA_CAPACITY > 1 << 64:
            raise MemoryError("guest extra heap address range overflow")
        aligned = self._align(extra_base, alignment)
        end = aligned + size_class
        if end > extra_base + self.EXTRA_CAPACITY:
            raise MemoryError("allocation does not fit in an extra heap arena")
        self.uc.mem_map(extra_base, self.EXTRA_CAPACITY)
        arena = {"base": extra_base, "capacity": self.EXTRA_CAPACITY, "pointer": end}
        self.extra_heaps.append(arena)
        return aligned

    def allocate(self, size, alignment=16):
        self._validate_alignment(alignment)
        size_class = self._size_class(size)
        bucket = self.free_buckets.get(size_class, [])
        for index in range(len(bucket)-1,-1,-1):
            pointer=bucket[index]
            if pointer % alignment == 0:
                del bucket[index]
                if not bucket:
                    del self.free_buckets[size_class]
                self.allocations[pointer] = size
                return pointer

        pointer = self._allocate_bump(size_class, alignment)
        self.allocations[pointer] = size
        return pointer

    def free(self, pointer):
        if pointer == 0:
            return
        if pointer not in self.allocations:
            raise ValueError(f"free of unknown or already freed guest pointer 0x{pointer:x}")
        size = self.allocations.pop(pointer)
        size_class = self._size_class(size)
        self.free_buckets.setdefault(size_class, []).append(pointer)

    def regions(self):
        return [(f"heap_extra_{index:02d}", arena["base"],
                 arena["pointer"] - arena["base"])
                for index, arena in enumerate(self.extra_heaps)]

    def snapshot(self):
        return {
            "version": 1,
            "base": self.base,
            "capacity": self.capacity,
            "primary_pointer": self.primary_pointer,
            "allocations": dict(self.allocations),
            "free_buckets": {str(size): list(pointers)
                             for size, pointers in self.free_buckets.items()},
            "extra_heaps": [dict(arena) for arena in self.extra_heaps],
        }

    def restore(self, state):
        """Restore allocator metadata; caller maps/restores extra arena bytes first."""
        self.base = self._int(state.get("base", self.base))
        self.capacity = self._int(state.get("capacity", self.capacity))
        self.primary_pointer = self._int(state["primary_pointer"])
        if not self.base <= self.primary_pointer <= self.base + self.capacity:
            raise ValueError("checkpoint primary heap pointer is outside its arena")
        self.allocations = {self._int(pointer): self._int(size)
                            for pointer, size in state.get("allocations", {}).items()}
        self.free_buckets = {self._int(size): [self._int(pointer) for pointer in pointers]
                             for size, pointers in state.get("free_buckets", {}).items()}
        self.extra_heaps = []
        for row in state.get("extra_heaps", []):
            arena = {key: self._int(row[key]) for key in ("base", "capacity", "pointer")}
            if not arena["base"] <= arena["pointer"] <= arena["base"] + arena["capacity"]:
                raise ValueError("checkpoint extra heap pointer is outside its arena")
            self.extra_heaps.append(arena)
        if len(self.extra_heaps) > self.MAX_EXTRAS:
            raise ValueError("checkpoint contains too many extra heap arenas")

    def summary(self):
        return {
            "base": hex(self.base),
            "capacity": self.capacity,
            "primary_pointer": hex(self.primary_pointer),
            "used_bytes": self.used_bytes,
            "allocation_count": len(self.allocations),
            "free_block_count": sum(len(items) for items in self.free_buckets.values()),
            "extra_heaps": [{**arena, "base": hex(arena["base"]),
                             "pointer": hex(arena["pointer"])}
                            for arena in self.extra_heaps],
        }
