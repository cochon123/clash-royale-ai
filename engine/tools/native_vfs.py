#!/usr/bin/env python3
"""Bounded read-only guest file VFS for explicitly mapped files.

The VFS stores bytes read only from caller-supplied host paths. Guest paths are
exact keys; it does not resolve directories, follow symlinks, or fall back to
the host filesystem. File metadata is synthetic and deterministic.
"""
from __future__ import annotations

import argparse
import errno
import hashlib
import json
import math
import os
import pathlib
import struct
import stat
from typing import Any


STAT_LAYOUT_SOURCE = pathlib.Path("/usr/include/asm-generic/stat.h")
STAT_LAYOUT_SHA256 = "8c8b66ab62a09ed42e8793c82edbf4a18ba4362891f1ed91026933e1a6c8cedf"
_STAT_LAYOUT_FORMAT = "<QQIIIIQQqiiqqQqQqQII"
_STAT_SIZE = struct.calcsize(_STAT_LAYOUT_FORMAT)
_FIXED_TIME = 1_700_000_000


def _read_layout_digest() -> str:
    """Return the digest of the local authoritative asm-generic header."""
    try:
        return hashlib.sha256(STAT_LAYOUT_SOURCE.read_bytes()).hexdigest()
    except OSError:
        return ""


STAT_LAYOUT_OBSERVED_SHA256 = _read_layout_digest()


class ReadOnlyVfs:
    """A tiny positional/read-offset VFS for a closed set of regular files.

    ``bindings`` maps exact guest paths to ``{"path": host_path,
    "sha256": expected_digest}``. Each mapped host file is read once during
    construction and its contents are digest-checked before use.
    """

    _ALLOWED_EXTRA_FLAGS = 0x00000100 | 0x00000800 | 0x00008000 | 0x00020000 | 0x00080000
    _WRITE_FLAGS = 0x00000001 | 0x00000002 | 0x00000040 | 0x00000200 | 0x00000400 | 0x00001000
    _O_DIRECTORY = 0x00010000

    def __init__(self, bindings: dict[str, dict[str, str]], *, next_fd: int = 100):
        if next_fd < 3:
            raise ValueError("next_fd must be at least 3")
        self._files: dict[str, bytes] = {}
        self._sources: dict[str, str] = {}
        self._digests: dict[str, str] = {}
        self._inodes: dict[str, int] = {}
        self._fds: dict[int, dict[str, Any]] = {}
        self._next_fd = next_fd

        for guest_path, spec in bindings.items():
            if not isinstance(guest_path, str) or not guest_path or "\x00" in guest_path:
                raise ValueError("guest path must be a nonempty string without NUL")
            if not isinstance(spec, dict) or set(spec) != {"path", "sha256"}:
                raise ValueError(f"{guest_path!r}: binding must contain exactly path and sha256")
            source = os.fspath(spec["path"])
            expected = spec["sha256"].lower()
            if len(expected) != 64 or any(c not in "0123456789abcdef" for c in expected):
                raise ValueError(f"{guest_path!r}: sha256 must be 64 hexadecimal characters")
            # This is the only host-file read: source is an explicit binding.
            contents = pathlib.Path(source).read_bytes()
            actual = hashlib.sha256(contents).hexdigest()
            if actual != expected:
                raise ValueError(f"{guest_path!r}: source digest mismatch ({actual})")
            self._files[guest_path] = contents
            self._sources[guest_path] = source
            self._digests[guest_path] = actual
            self._inodes[guest_path] = int.from_bytes(
                hashlib.sha256(guest_path.encode("utf-8")).digest()[:8], "little"
            ) or 1

    @property
    def identity(self) -> dict[str, dict[str, Any]]:
        """Serializable input identity used to validate restored checkpoints."""
        return {
            guest_path: {
                "path": self._sources[guest_path],
                "sha256": self._digests[guest_path],
                "bytes": len(self._files[guest_path]),
            }
            for guest_path in sorted(self._files)
        }

    def _path(self, path: str | bytes | os.PathLike[str]) -> str:
        if isinstance(path, bytes):
            try:
                path = path.decode("utf-8")
            except UnicodeDecodeError as exc:
                raise OSError(errno.ENOENT, os.strerror(errno.ENOENT)) from exc
        path = os.fspath(path)
        if not isinstance(path, str) or "\x00" in path:
            raise OSError(errno.EINVAL, os.strerror(errno.EINVAL))
        if path not in self._files:
            raise OSError(errno.ENOENT, os.strerror(errno.ENOENT), path)
        return path

    def open(self, path: str | bytes | os.PathLike[str], flags: int) -> int:
        """Open an exact mapped path read-only and return a synthetic fd."""
        if not isinstance(flags, int) or flags < 0:
            raise OSError(errno.EINVAL, os.strerror(errno.EINVAL))
        if flags & self._O_DIRECTORY:
            raise OSError(errno.ENOTDIR, os.strerror(errno.ENOTDIR), os.fspath(path))
        if flags & self._WRITE_FLAGS:
            raise OSError(errno.EROFS, os.strerror(errno.EROFS), os.fspath(path))
        if flags & ~(self._ALLOWED_EXTRA_FLAGS):
            raise OSError(errno.EINVAL, os.strerror(errno.EINVAL), os.fspath(path))
        guest_path = self._path(path)
        fd = self._next_fd
        self._next_fd += 1
        self._fds[fd] = {"path": guest_path, "offset": 0}
        return fd

    def close(self, fd: int) -> int:
        if fd not in self._fds:
            raise OSError(errno.EBADF, os.strerror(errno.EBADF))
        del self._fds[fd]
        return 0

    def read(self, fd: int, count: int) -> bytes:
        state = self._fd(fd)
        self._count(count)
        content = self._files[state["path"]]
        offset = state["offset"]
        result = content[offset:offset + count]
        state["offset"] = offset + len(result)
        return result

    def pread(self, fd: int, count: int, signed_offset: int) -> bytes:
        state = self._fd(fd)
        self._count(count)
        if not isinstance(signed_offset, int):
            raise OSError(errno.EINVAL, os.strerror(errno.EINVAL))
        if signed_offset < 0:
            raise OSError(errno.EINVAL, os.strerror(errno.EINVAL))
        if signed_offset > (1 << 63) - 1:
            raise OSError(errno.EOVERFLOW, os.strerror(errno.EOVERFLOW))
        content = self._files[state["path"]]
        return content[signed_offset:signed_offset + count]

    def lseek(self, fd: int, signed_offset: int, whence: int) -> int:
        state = self._fd(fd)
        if not isinstance(signed_offset, int):
            raise OSError(errno.EINVAL, os.strerror(errno.EINVAL))
        if signed_offset < -(1 << 63) or signed_offset > (1 << 63) - 1:
            raise OSError(errno.EOVERFLOW, os.strerror(errno.EOVERFLOW))
        if whence == os.SEEK_SET:
            new_offset = signed_offset
        elif whence == os.SEEK_CUR:
            new_offset = state["offset"] + signed_offset
        elif whence == os.SEEK_END:
            new_offset = len(self._files[state["path"]]) + signed_offset
        else:
            raise OSError(errno.EINVAL, os.strerror(errno.EINVAL))
        if new_offset < 0:
            raise OSError(errno.EINVAL, os.strerror(errno.EINVAL))
        if new_offset > (1 << 63) - 1:
            raise OSError(errno.EOVERFLOW, os.strerror(errno.EOVERFLOW))
        state["offset"] = new_offset
        return new_offset

    def access(self, path: str | bytes | os.PathLike[str], mode: int) -> int:
        self._path(path)
        if not isinstance(mode, int) or mode & ~(os.R_OK | os.W_OK | os.X_OK | os.F_OK):
            raise OSError(errno.EINVAL, os.strerror(errno.EINVAL))
        if mode & (os.W_OK | os.X_OK):
            raise OSError(errno.EACCES, os.strerror(errno.EACCES), os.fspath(path))
        return 0

    def stat(self, path: str | bytes | os.PathLike[str]) -> bytes:
        guest_path = self._path(path)
        return self._stat_bytes(guest_path)

    def fstat(self, fd: int) -> bytes:
        state = self._fd(fd)
        return self._stat_bytes(state["path"])

    def stat_bytes128(self, path: str | bytes | os.PathLike[str]) -> bytes:
        """Alias spelling emphasizing the guest ABI's fixed 128-byte result."""
        return self.stat(path)

    def _stat_bytes(self, guest_path: str) -> bytes:
        size = len(self._files[guest_path])
        values = (
            1, self._inodes[guest_path], stat.S_IFREG | 0o444, 1, 0, 0,
            0, 0, size, 4096, 0, math.ceil(size / 512),
            _FIXED_TIME, 0, _FIXED_TIME, 0, _FIXED_TIME, 0, 0, 0,
        )
        result = struct.pack(_STAT_LAYOUT_FORMAT, *values)
        if len(result) != 128:
            raise AssertionError(f"asm-generic 64-bit stat layout is {_STAT_SIZE}, not 128 bytes")
        return result

    def snapshot(self) -> dict[str, Any]:
        return {
            "version": 1,
            "next_fd": self._next_fd,
            "identity": self.identity,
            "open_fds": {
                str(fd): {"path": state["path"], "offset": state["offset"]}
                for fd, state in sorted(self._fds.items())
            },
        }

    def restore(self, snapshot: dict[str, Any]) -> None:
        if not isinstance(snapshot, dict) or snapshot.get("version") != 1:
            raise ValueError("unsupported VFS snapshot")
        if snapshot.get("identity") != self.identity:
            raise ValueError("VFS snapshot input identity does not match current bindings")
        next_fd = snapshot.get("next_fd")
        open_fds = snapshot.get("open_fds")
        if not isinstance(next_fd, int) or next_fd < 3 or not isinstance(open_fds, dict):
            raise ValueError("malformed VFS snapshot")
        restored: dict[int, dict[str, Any]] = {}
        for raw_fd, state in open_fds.items():
            try:
                fd = int(raw_fd)
            except (TypeError, ValueError) as exc:
                raise ValueError("malformed fd in VFS snapshot") from exc
            if fd < 3 or str(fd) != str(raw_fd) or not isinstance(state, dict):
                raise ValueError("malformed fd entry in VFS snapshot")
            path = state.get("path")
            offset = state.get("offset")
            if path not in self._files or not isinstance(offset, int) or offset < 0 or offset > (1 << 63) - 1:
                raise ValueError("malformed open file state in VFS snapshot")
            restored[fd] = {"path": path, "offset": offset}
        if restored and next_fd <= max(restored):
            raise ValueError("snapshot next_fd must exceed all open descriptors")
        self._fds = restored
        self._next_fd = next_fd

    def _fd(self, fd: int) -> dict[str, Any]:
        try:
            return self._fds[fd]
        except (KeyError, TypeError) as exc:
            raise OSError(errno.EBADF, os.strerror(errno.EBADF)) from exc

    @staticmethod
    def _count(count: int) -> None:
        if not isinstance(count, int) or count < 0 or count > (1 << 63) - 1:
            raise OSError(errno.EINVAL, os.strerror(errno.EINVAL))


class ReadOnlyStdio:
    """Opaque guest ``FILE *`` state backed by a :class:`ReadOnlyVfs`.

    Handles are integer tokens only; no host ``FILE`` or guest libc layout is
    created. Callers may provide a guest token to ``fopen`` or let this helper
    assign one. The VFS owns descriptor offsets and must be snapshotted/restored
    separately from the stream EOF/error indicators here.
    """

    _READ_MODES = {"r", "rb", "rt", "re", "rbe", "rte"}

    def __init__(self, vfs: ReadOnlyVfs, *, next_handle: int = 0x7FFF0000):
        if next_handle <= 0:
            raise ValueError("next_handle must be positive")
        self.vfs = vfs
        self._next_handle = next_handle
        self._streams: dict[int, dict[str, Any]] = {}

    def fopen(self, path: str | bytes | os.PathLike[str], mode: str, handle: int | None = None) -> int:
        """Open a mapped file read-only and associate an opaque stream token."""
        if mode not in self._READ_MODES:
            if isinstance(mode, str) and mode[:1] in {"w", "a"} or "+" in str(mode):
                raise OSError(errno.EROFS, os.strerror(errno.EROFS), os.fspath(path))
            raise OSError(errno.EINVAL, os.strerror(errno.EINVAL), os.fspath(path))
        if handle is not None:
            if not isinstance(handle, int) or handle <= 0:
                raise OSError(errno.EINVAL, os.strerror(errno.EINVAL))
            if handle in self._streams:
                raise OSError(errno.EBUSY, os.strerror(errno.EBUSY))
        vfs_fd = self.vfs.open(path, os.O_RDONLY | 0x00008000 | 0x00080000)
        if handle is None:
            while self._next_handle in self._streams:
                self._next_handle += 1
            handle = self._next_handle
            self._next_handle += 1
        self._streams[handle] = {"fd": vfs_fd, "eof": False, "error": False}
        return handle

    def fclose(self, handle: int) -> int:
        state = self._stream(handle)
        self.vfs.close(state["fd"])
        del self._streams[handle]
        return 0

    def fread(self, handle: int, size: int, count: int) -> tuple[bytes, int]:
        """Read up to ``size * count`` bytes; return bytes and complete items."""
        state = self._stream(handle)
        if not isinstance(size, int) or not isinstance(count, int) or size < 0 or count < 0:
            raise OSError(errno.EINVAL, os.strerror(errno.EINVAL))
        if size == 0 or count == 0:
            return b"", 0
        if size > ((1 << 63) - 1) // count:
            raise OSError(errno.EOVERFLOW, os.strerror(errno.EOVERFLOW))
        try:
            data = self.vfs.read(state["fd"], size * count)
        except OSError:
            state["error"] = True
            return b"", 0
        if len(data) < size * count:
            state["eof"] = True
        return data, len(data) // size

    def fseek(self, handle: int, signed_offset: int, whence: int) -> int:
        state = self._stream(handle)
        try:
            self.vfs.lseek(state["fd"], signed_offset, whence)
        except OSError:
            state["error"] = True
            return -1
        state["eof"] = False
        return 0

    def ftell(self, handle: int) -> int:
        state = self._stream(handle)
        try:
            return self.vfs.lseek(state["fd"], 0, os.SEEK_CUR)
        except OSError:
            state["error"] = True
            return -1

    def feof(self, handle: int) -> int:
        return int(self._stream(handle)["eof"])

    def ferror(self, handle: int) -> int:
        return int(self._stream(handle)["error"])

    def clearerr(self, handle: int) -> int:
        state = self._stream(handle)
        state["eof"] = False
        state["error"] = False
        return 0

    def fileno(self, handle: int) -> int:
        return self._stream(handle)["fd"]

    def fflush(self, handle: int | None) -> int:
        if handle is not None:
            self._stream(handle)
        return 0

    def snapshot(self) -> dict[str, Any]:
        return {
            "version": 1,
            "next_handle": self._next_handle,
            "streams": {
                str(handle): {
                    "fd": state["fd"],
                    "eof": state["eof"],
                    "error": state["error"],
                }
                for handle, state in sorted(self._streams.items())
            },
        }

    def restore(self, snapshot: dict[str, Any]) -> None:
        if not isinstance(snapshot, dict) or snapshot.get("version") != 1:
            raise ValueError("unsupported stdio snapshot")
        next_handle = snapshot.get("next_handle")
        streams = snapshot.get("streams")
        if not isinstance(next_handle, int) or next_handle <= 0 or not isinstance(streams, dict):
            raise ValueError("malformed stdio snapshot")
        restored: dict[int, dict[str, Any]] = {}
        for raw_handle, state in streams.items():
            try:
                handle = int(raw_handle)
            except (TypeError, ValueError) as exc:
                raise ValueError("malformed FILE handle in stdio snapshot") from exc
            if handle <= 0 or str(handle) != str(raw_handle) or not isinstance(state, dict):
                raise ValueError("malformed FILE handle entry in stdio snapshot")
            fd = state.get("fd")
            eof = state.get("eof")
            error = state.get("error")
            if not isinstance(fd, int) or not isinstance(eof, bool) or not isinstance(error, bool):
                raise ValueError("malformed stream state in stdio snapshot")
            # The paired ReadOnlyVfs snapshot is restored first and owns offsets.
            self.vfs.fstat(fd)
            restored[handle] = {"fd": fd, "eof": eof, "error": error}
        self._streams = restored
        self._next_handle = next_handle

    def _stream(self, handle: int) -> dict[str, Any]:
        try:
            return self._streams[handle]
        except (KeyError, TypeError) as exc:
            raise OSError(errno.EBADF, os.strerror(errno.EBADF)) from exc


def _self_check(guest_path: str, host_path: pathlib.Path) -> None:
    """Compare key VFS operations with an explicitly supplied host file."""
    host_path = host_path.resolve()
    source_bytes = host_path.read_bytes()
    expected_digest = hashlib.sha256(source_bytes).hexdigest()
    vfs = ReadOnlyVfs({guest_path: {"path": str(host_path), "sha256": expected_digest}})
    fd = vfs.open(guest_path, os.O_RDONLY | 0x00008000 | 0x00080000)
    with host_path.open("rb") as reference:
        for count in (1, 7, 4093, 8192):
            got = vfs.read(fd, count)
            want = reference.read(count)
            assert got == want
        saved = reference.tell()
        for offset, count in ((0, 16), (511, 4096), (4097, 31), (max(0, len(source_bytes) - 9), 32), (len(source_bytes), 1), (len(source_bytes) + 5, 4)):
            reference.seek(offset)
            want = reference.read(count)
            assert vfs.pread(fd, count, offset) == want
            assert vfs.lseek(fd, 0, os.SEEK_CUR) == saved
        reference.seek(saved)
        assert vfs.lseek(fd, 0, os.SEEK_SET) == 0
        reference.seek(0)
        assert vfs.read(fd, 37) == reference.read(37)
        reference.seek(-23, os.SEEK_END)
        assert vfs.lseek(fd, -23, os.SEEK_END) == reference.tell()
        assert vfs.read(fd, 128) == reference.read(128)
        assert vfs.read(fd, 1) == reference.read(1) == b""
        vstat = vfs.fstat(fd)
        assert len(vstat) == 128
        assert struct.unpack_from("<q", vstat, 48)[0] == host_path.stat().st_size
        assert struct.unpack_from("<I", vstat, 16)[0] == (stat.S_IFREG | 0o444)
    snapshot = vfs.snapshot()
    restored = ReadOnlyVfs({guest_path: {"path": str(host_path), "sha256": expected_digest}})
    restored.restore(snapshot)
    assert restored.snapshot() == snapshot
    assert vfs.close(fd) == 0

    stdio_vfs = ReadOnlyVfs({guest_path: {"path": str(host_path), "sha256": expected_digest}})
    stdio = ReadOnlyStdio(stdio_vfs)
    stream = stdio.fopen(guest_path, "rbe")
    with host_path.open("rb") as reference:
        header, items = stdio.fread(stream, 1, 100)
        assert (header, items) == (reference.read(100), 100)
        page_size = int.from_bytes(source_bytes[16:18], "big")
        assert page_size >= 512 and page_size & (page_size - 1) == 0
        assert stdio.fseek(stream, 0, os.SEEK_SET) == 0
        page, items = stdio.fread(stream, page_size, 1)
        assert (page, items) == (source_bytes[:page_size], 1)
        reference.seek(0)
        assert page == reference.read(page_size)
        assert stdio.fseek(stream, -1, os.SEEK_END) == 0
        reference.seek(-1, os.SEEK_END)
        partial, items = stdio.fread(stream, 2, 1)
        assert (partial, items) == (reference.read(2), 0)
        assert stdio.feof(stream) == 1 and stdio.ferror(stream) == 0
        assert stdio.fseek(stream, -len(source_bytes) - 1, os.SEEK_SET) == -1
        assert stdio.feof(stream) == 1 and stdio.ferror(stream) == 1
        assert stdio.ftell(stream) == len(source_bytes)
        assert stdio.fflush(stream) == 0
        assert stdio.fflush(None) == 0
    vfs_snapshot = stdio_vfs.snapshot()
    stdio_snapshot = stdio.snapshot()
    restored_stdio_vfs = ReadOnlyVfs({guest_path: {"path": str(host_path), "sha256": expected_digest}})
    restored_stdio_vfs.restore(vfs_snapshot)
    restored_stdio = ReadOnlyStdio(restored_stdio_vfs)
    restored_stdio.restore(stdio_snapshot)
    assert restored_stdio.fileno(stream) == stdio.fileno(stream)
    assert restored_stdio.ftell(stream) == stdio.ftell(stream)
    assert restored_stdio.feof(stream) == 1 and restored_stdio.ferror(stream) == 1
    restored_stdio.clearerr(stream)
    assert restored_stdio.feof(stream) == restored_stdio.ferror(stream) == 0
    assert restored_stdio.fclose(stream) == 0


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", nargs=2, metavar=("GUEST_PATH", "HOST_PATH"), help="run focused VFS checks")
    args = parser.parse_args()
    if args.check:
        _self_check(args.check[0], pathlib.Path(args.check[1]))
        print(json.dumps({"ok": True, "stat_layout_source": str(STAT_LAYOUT_SOURCE), "stat_layout_sha256": STAT_LAYOUT_SHA256}))


if __name__ == "__main__":
    main()
