"""Locate a local Clash Royale APK and record what REA can inspect.

The APK stays under ``var/clash-royale/`` (gitignored). The optional download
talks only to an allowlisted APKPure CDN. Nothing here contacts Supercell, and
nothing executes the APK.
"""

from __future__ import annotations

import hashlib
import io
import json
import os
import shutil
import struct
import subprocess
import zipfile
from pathlib import Path
from urllib.error import HTTPError, URLError
from urllib.parse import urlparse
from urllib.request import HTTPRedirectHandler, Request, build_opener

from .catalog import repo_root
from .engine import REAL_ENGINE_BLOCKER

PACKAGE = "com.supercell.clashroyale"
APKPURE_PAGE = "https://apkpure.com/clash-royale/com.supercell.clashroyale"
APKPURE_XAPK_URL = "https://d.apkpure.net/b/APK/com.supercell.clashroyale?version=latest"
ALLOWED_DOWNLOAD_HOSTS = frozenset(
    {
        "d.apkpure.net",
        "d.cdnpure.com",
        "data.winudf.com",
    }
)
FORBIDDEN_HOST_SUFFIXES = (
    "supercell.com",
    "clashroyale.com",
    "clashroyale.onelink.me",
)
MIN_FETCHED_BYTES = 1_000_000
_DOWNLOAD_UA = (
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 "
    "(KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"
)
_LOOPBACK_NEEDLES = (b"127.0.0.1",)
_NATIVE_SCAN_LIBS = ("libsupercell_clashroyale.so", "libg.so")


def _battle_blocker(*, dex_loopback: bool, native_scanned: bool, native_loopback: bool) -> str:
    if dex_loopback or native_loopback:
        return (
            "A static scan found 127.0.0.1 in the client, but no local process "
            "that resolves combat is running. real_engine stays false. Do not "
            "dial Supercell. Smallest next step: a program on "
            "http://127.0.0.1:8790 that speaks cr-local-match/1 and actually "
            "resolves a battle."
        )
    native = (
        "A static scan of libsupercell_clashroyale.so and libg.so in the arm64 "
        "split found no 127.0.0.1."
        if native_scanned
        else "No arm64 split was available to scan."
    )
    return (
        "Verified package com.supercell.clashroyale still cannot run a battle "
        "on this machine. The base DEX files do not contain 127.0.0.1. "
        f"{native} The control harness is not the combat engine. Smallest next "
        "step: a process on http://127.0.0.1:8790 that speaks cr-local-match/1, "
        "sets real_engine true, and actually resolves combat. Do not patch this "
        "client to dial Supercell."
    )


def default_var_dir() -> Path:
    return repo_root() / "var" / "clash-royale"


def _u16(data: bytes, offset: int) -> int:
    return struct.unpack_from("<H", data, offset)[0]


def _u32(data: bytes, offset: int) -> int:
    return struct.unpack_from("<I", data, offset)[0]


def _forbidden_host(host: str) -> bool:
    return any(host == suffix or host.endswith("." + suffix) for suffix in FORBIDDEN_HOST_SUFFIXES)


def assert_download_host(url: str) -> str:
    """Allow only the APKPure CDN hosts that actually returned the XAPK.

    The HTML page on apkpure.com is not a download host: curl receives a
    Cloudflare challenge there. Supercell hosts are refused before any socket
    to them.
    """
    host = (urlparse(url).hostname or "").lower().rstrip(".")
    if host in ALLOWED_DOWNLOAD_HOSTS and not _forbidden_host(host):
        return host
    raise ValueError(
        f"refusing download host {host or url!r}. Allowed CDN hosts are "
        "d.apkpure.net, d.cdnpure.com, and data.winudf.com. "
        "Supercell hosts are never contacted."
    )


def _redact_url(url: str) -> str:
    parsed = urlparse(url)
    return f"{parsed.scheme}://{parsed.hostname}{parsed.path}"


def _looks_like_html(head: bytes) -> bool:
    stripped = head.lstrip().lower()
    return stripped.startswith(b"<") or stripped.startswith(b"<!doctype") or b"<html" in stripped[:200]


def _read_head(path: Path, count: int = 512) -> bytes:
    with path.open("rb") as handle:
        return handle.read(count)


def _sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def _string_pool(data: bytes, offset: int) -> tuple[list[str], int] | None:
    if offset + 28 > len(data) or _u16(data, offset) != 0x0001:
        return None
    header_size = _u16(data, offset + 2)
    chunk_size = _u32(data, offset + 4)
    if header_size < 28 or chunk_size < header_size or offset + chunk_size > len(data):
        return None
    string_count = _u32(data, offset + 8)
    flags = _u32(data, offset + 16)
    strings_start = _u32(data, offset + 20)
    if string_count > 100_000:
        return None
    utf8 = bool(flags & (1 << 8))
    pool = offset + strings_start
    strings: list[str] = []
    for index in range(string_count):
        rel = _u32(data, offset + header_size + 4 * index)
        cursor = pool + rel
        if utf8:
            if cursor >= len(data):
                return None
            char_len = data[cursor]
            cursor += 1
            if char_len & 0x80:
                if cursor >= len(data):
                    return None
                char_len = ((char_len & 0x7F) << 8) | data[cursor]
                cursor += 1
            if cursor >= len(data):
                return None
            byte_len = data[cursor]
            cursor += 1
            if byte_len & 0x80:
                if cursor >= len(data):
                    return None
                byte_len = ((byte_len & 0x7F) << 8) | data[cursor]
                cursor += 1
            strings.append(data[cursor : cursor + byte_len].decode("utf-8", "replace"))
        else:
            if cursor + 2 > len(data):
                return None
            char_len = _u16(data, cursor)
            cursor += 2
            if char_len & 0x8000:
                if cursor + 2 > len(data):
                    return None
                char_len = ((char_len & 0x7FFF) << 16) | _u16(data, cursor)
                cursor += 2
            strings.append(data[cursor : cursor + char_len * 2].decode("utf-16-le", "replace"))
    return strings, chunk_size


def _pool_string(strings: list[str], index: int) -> str | None:
    if index == 0xFFFFFFFF or index >= len(strings):
        return None
    return strings[index]


def read_manifest_identity(data: bytes) -> dict | None:
    """Read package, versionName, and versionCode from a binary AndroidManifest."""
    if len(data) < 8 or _u32(data, 0) != 0x00080003:
        return None
    parsed = _string_pool(data, 8)
    if parsed is None:
        return None
    strings, pool_size = parsed
    position = 8 + pool_size
    identity: dict | None = None
    while position + 8 <= len(data):
        chunk_type = _u16(data, position)
        chunk_size = _u32(data, position + 4)
        if chunk_size < 8 or position + chunk_size > len(data):
            break
        if chunk_type == 0x0102 and identity is None:
            ext = position + 16
            if ext + 20 <= position + chunk_size:
                name = _pool_string(strings, _u32(data, ext + 4))
                if name == "manifest":
                    attr_start = _u16(data, ext + 8)
                    attr_size = _u16(data, ext + 10) or 20
                    attr_count = _u16(data, ext + 12)
                    cursor = ext + attr_start
                    values: dict[str, str | int | None] = {}
                    for _ in range(attr_count):
                        if cursor + 20 > position + chunk_size:
                            break
                        attr_name = _pool_string(strings, _u32(data, cursor + 4))
                        value_type = data[cursor + 15]
                        raw = _u32(data, cursor + 16)
                        if value_type == 0x03:
                            values[attr_name or ""] = _pool_string(strings, raw)
                        elif value_type in {0x10, 0x11}:
                            values[attr_name or ""] = raw
                        cursor += attr_size
                    identity = {
                        "package": values.get("package"),
                        "version_name": values.get("versionName"),
                        "version_code": values.get("versionCode"),
                    }
        position += chunk_size
    return identity


def inspect_apk_zip(path: Path) -> dict:
    """Hash an APK and read its manifest identity. Do not extract it."""
    path = Path(path)
    if not path.is_file():
        raise FileNotFoundError(path)
    size = path.stat().st_size
    head = _read_head(path)
    if _looks_like_html(head):
        raise ValueError(f"{path} is an HTML page, not an APK")
    if not head.startswith(b"PK"):
        raise ValueError(f"{path} is not a zip/APK (magic {head[:4]!r})")
    if size < 64:
        raise ValueError(f"{path} is too small to be an APK")
    if not zipfile.is_zipfile(path):
        raise ValueError(f"{path} is not a zip/APK")
    digest = _sha256(path)
    with zipfile.ZipFile(path) as archive:
        names = archive.namelist()
        identity = None
        if "AndroidManifest.xml" in names:
            identity = read_manifest_identity(archive.read("AndroidManifest.xml"))
    native = sorted(
        {
            name.rsplit("/", 1)[-1]
            for name in names
            if name.startswith("lib/") and name.endswith(".so")
        }
    )
    package = None if identity is None else identity.get("package")
    version_name = None if identity is None else identity.get("version_name")
    version_code = None if identity is None else identity.get("version_code")
    accepted = package == PACKAGE
    if accepted:
        reject_reason = None
    elif package:
        reject_reason = f"package is {package!r}, expected {PACKAGE}"
    else:
        reject_reason = "AndroidManifest package name could not be read"
    return {
        "path": str(path),
        "sha256": digest,
        "bytes": size,
        "entries": len(names),
        "has_android_manifest": "AndroidManifest.xml" in names,
        "has_classes_dex": any(
            name.startswith("classes") and name.endswith(".dex") for name in names
        ),
        "native_libraries": native,
        "package": package,
        "version_name": None if version_name is None else str(version_name),
        "version_code": version_code,
        "accepted": accepted,
        "reject_reason": reject_reason,
    }


def _zip_entries(archive: zipfile.ZipFile) -> list[dict]:
    return [
        {"name": info.filename, "bytes": info.file_size}
        for info in archive.infolist()
        if not info.filename.endswith("/")
    ]


def _base_member(meta: dict, names: set[str]) -> str:
    for split in meta.get("split_apks") or []:
        if split.get("id") == "base" and split.get("file") in names:
            return str(split["file"])
    fallback = f"{PACKAGE}.apk"
    if fallback in names:
        return fallback
    raise ValueError("XAPK has no base split")


def _nested_native_names(archive: zipfile.ZipFile, member: str | None) -> list[str]:
    if not member:
        return []
    try:
        info = archive.getinfo(member)
    except KeyError:
        return []
    if info.file_size > 80_000_000:
        return []
    payload = archive.read(member)
    if not payload.startswith(b"PK"):
        return []
    with zipfile.ZipFile(io.BytesIO(payload)) as nested:
        return sorted(
            {
                name.rsplit("/", 1)[-1]
                for name in nested.namelist()
                if name.endswith(".so")
            }
        )


def _scan_native_loopback(archive: zipfile.ZipFile, member: str | None) -> dict:
    """Search two native libraries for 127.0.0.1. Do not extract them."""
    found = {name: False for name in _NATIVE_SCAN_LIBS}
    scanned: list[str] = []
    if not member:
        return {"member": None, "loopback": found, "scanned": scanned}
    try:
        info = archive.getinfo(member)
    except KeyError:
        return {"member": member, "loopback": found, "scanned": scanned}
    if info.file_size > 80_000_000:
        return {
            "member": member,
            "loopback": found,
            "scanned": scanned,
            "skipped": "split too large",
        }
    payload = archive.read(member)
    if not payload.startswith(b"PK"):
        return {"member": member, "loopback": found, "scanned": scanned}
    with zipfile.ZipFile(io.BytesIO(payload)) as nested:
        by_base = {name.rsplit("/", 1)[-1]: name for name in nested.namelist()}
        for base in _NATIVE_SCAN_LIBS:
            inner = by_base.get(base)
            if inner is None:
                continue
            blob = nested.read(inner)
            scanned.append(base)
            found[base] = any(needle in blob for needle in _LOOPBACK_NEEDLES)
    return {"member": member, "loopback": found, "scanned": scanned}


def describe_xapk(path: Path) -> dict:
    """Record an XAPK layout. Only the base APK is ever written to disk later."""
    path = Path(path)
    head = _read_head(path)
    if _looks_like_html(head):
        raise ValueError(f"{path} is an HTML page, not an XAPK")
    if not head.startswith(b"PK") or not zipfile.is_zipfile(path):
        raise ValueError(f"{path} is not a zip/XAPK")
    with zipfile.ZipFile(path) as archive:
        names = set(archive.namelist())
        if "manifest.json" not in names:
            raise ValueError(f"{path} is not an XAPK (no manifest.json)")
        meta = json.loads(archive.read("manifest.json"))
        base = _base_member(meta, names)
        arm = None
        for split in meta.get("split_apks") or []:
            if split.get("id") == "config.arm64_v8a":
                arm = split.get("file")
        native = _nested_native_names(archive, arm)
        loopback = _scan_native_loopback(archive, arm)
        entries = _zip_entries(archive)
    package_name = meta.get("package_name")
    return {
        "path": str(path),
        "sha256": _sha256(path),
        "bytes": path.stat().st_size,
        "package_name": package_name,
        "version_name": meta.get("version_name"),
        "version_code": meta.get("version_code"),
        "xapk_version": meta.get("xapk_version"),
        "base_member": base,
        "splits": [split.get("id") for split in meta.get("split_apks") or []],
        "entries": entries,
        "native_libraries": native,
        "native_loopback": loopback,
        "accepted_package": package_name == PACKAGE,
    }


def extract_base_apk(xapk: Path, dest: Path) -> Path:
    """Copy only the base split out of an XAPK. Leave every other member packed."""
    dest = Path(dest)
    dest.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(xapk) as archive:
        meta = json.loads(archive.read("manifest.json"))
        member = _base_member(meta, set(archive.namelist()))
        info = archive.getinfo(member)
        if dest.is_file() and dest.stat().st_size == info.file_size:
            return dest
        temporary = dest.with_suffix(dest.suffix + ".partial")
        with archive.open(member) as source, temporary.open("wb") as target:
            shutil.copyfileobj(source, target, 1024 * 1024)
        temporary.replace(dest)
    return dest


def _dex_has_loopback(apk: Path) -> bool:
    with zipfile.ZipFile(apk) as archive:
        for name in archive.namelist():
            if name.startswith("classes") and name.endswith(".dex"):
                if any(needle in archive.read(name) for needle in _LOOPBACK_NEEDLES):
                    return True
    return False


class _GuardedRedirects(HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        assert_download_host(newurl)
        return super().redirect_request(req, fp, code, msg, headers, newurl)


def download_apkpure(dest_dir: Path, url: str = APKPURE_XAPK_URL) -> dict:
    """Download the XAPK from the allowlisted CDN. Refuse every other host."""
    assert_download_host(url)
    dest_dir = Path(dest_dir)
    dest_dir.mkdir(parents=True, exist_ok=True)
    target = dest_dir / "clash-royale.xapk"
    partial = dest_dir / ".clash-royale.xapk.partial"
    opener = build_opener(_GuardedRedirects)
    request = Request(url, headers={"User-Agent": _DOWNLOAD_UA})
    final_url = url
    try:
        with opener.open(request, timeout=120) as response, partial.open("wb") as handle:
            final_url = response.geturl()
            assert_download_host(final_url)
            shutil.copyfileobj(response, handle, 1024 * 1024)
    except (HTTPError, URLError, TimeoutError, OSError, ValueError):
        partial.unlink(missing_ok=True)
        raise
    head = _read_head(partial)
    size = partial.stat().st_size
    if _looks_like_html(head) or not head.startswith(b"PK") or size < MIN_FETCHED_BYTES:
        partial.unlink(missing_ok=True)
        raise ValueError(
            f"APKPure download was rejected (bytes={size}, magic={head[:4]!r}). "
            "Expected an XAPK zip, not an HTML challenge page."
        )
    partial.replace(target)
    return {
        "page": APKPURE_PAGE,
        "url": _redact_url(url),
        "final_url": _redact_url(final_url),
        "path": str(target),
        "bytes": target.stat().st_size,
    }


def _java_has_compiler() -> bool:
    java = shutil.which("java")
    if not java:
        return False
    try:
        proc = subprocess.run(
            [java, "--list-modules"],
            capture_output=True,
            text=True,
            timeout=20,
            check=False,
        )
    except (OSError, subprocess.TimeoutExpired):
        return False
    return "jdk.compiler@" in proc.stdout


def pull_from_device(serial: str, dest: Path) -> dict:
    """Pull the installed package via REA's adb flow, or adb itself.

    This talks only to a local adb server. It does not open a Supercell socket.
    """
    dest.mkdir(parents=True, exist_ok=True)
    rea = shutil.which("rea")
    if rea:
        command = [
            rea,
            "pull-adb-package",
            serial,
            PACKAGE,
            "--output-directory",
            str(dest),
        ]
        proc = subprocess.run(command, capture_output=True, text=True, timeout=300, check=False)
        return {
            "tool": "rea",
            "command": command,
            "returncode": proc.returncode,
            "stderr": proc.stderr[-1000:],
            "stdout": proc.stdout[-1000:],
        }
    adb = os.environ.get("REA_ADB_PATH") or shutil.which("adb")
    if not adb:
        raise RuntimeError(
            "adb is not installed and `rea` is not on PATH. REA does not install "
            "platform-tools. Install Android platform-tools, connect a device you "
            "control that already has Clash Royale from the Play Store, then rerun "
            f"with --from-device {serial}."
        )
    listed = subprocess.run(
        [adb, "-s", serial, "shell", "pm", "path", PACKAGE],
        capture_output=True,
        text=True,
        timeout=30,
        check=False,
    )
    if listed.returncode != 0:
        raise RuntimeError(listed.stderr.strip() or listed.stdout.strip() or "adb pm path failed")
    device_paths = []
    for line in listed.stdout.splitlines():
        line = line.strip()
        if line.startswith("package:"):
            device_paths.append(line.split(":", 1)[1].strip())
    if not device_paths:
        raise RuntimeError(
            f"{PACKAGE} is not installed on {serial}. Install it from the Play Store "
            "on that device, then pull it. Do not use a third-party repack."
        )
    saved = []
    for device_path in device_paths:
        name = device_path.rsplit("/", 1)[-1]
        if name in {"", ".", ".."} or "/" in name:
            raise RuntimeError(f"refusing device path {device_path!r}")
        target = dest / name
        pulled = subprocess.run(
            [adb, "-s", serial, "pull", device_path, str(target)],
            capture_output=True,
            text=True,
            timeout=300,
            check=False,
        )
        if pulled.returncode != 0:
            raise RuntimeError(pulled.stderr.strip() or "adb pull failed")
        saved.append(str(target))
    return {"tool": "adb", "serial": serial, "files": saved}


def rea_inspect(apk: Path, log_dir: Path | None = None) -> dict:
    command = ["rea", "inspect-android-package", str(apk), "--format", "json"]
    rea = shutil.which("rea")
    if not rea:
        return {
            "ran": False,
            "command": command,
            "error": (
                "rea is not on PATH. Install Node.js 22.19+ and run "
                "`npm install --global rea-agents`. Download the pinned "
                "jadx-headless-mcp 0.7.1 jar, verify its SHA-256 "
                "6e5eacf500b64292bfb73c49797c1958f6ee44646e43e868039ae7feb573ff75, "
                "and set REA_JADX_MCP_JAR. See rea docs/android-analysis.md."
            ),
        }
    env = os.environ.copy()
    try:
        proc = subprocess.run(
            [rea, *command[1:]],
            capture_output=True,
            text=True,
            timeout=900,
            check=False,
            env=env,
        )
    except (OSError, subprocess.TimeoutExpired) as exc:
        return {"ran": True, "command": command, "error": str(exc)}
    stdout_path = None
    if log_dir is not None and proc.stdout:
        log_dir.mkdir(parents=True, exist_ok=True)
        target = log_dir / "rea-inspect.stdout"
        target.write_text(proc.stdout, encoding="utf-8")
        stdout_path = str(target)
    return {
        "ran": True,
        "command": command,
        "returncode": proc.returncode,
        "stdout_chars": len(proc.stdout),
        "stdout_path": stdout_path,
        "stderr_tail": proc.stderr[-800:],
    }


def _is_xapk(path: Path) -> bool:
    if path.suffix.lower() not in {".xapk", ".apks", ".zip"} and path.name != "clash-royale.xapk":
        return False
    if not zipfile.is_zipfile(path):
        return False
    with zipfile.ZipFile(path) as archive:
        return "manifest.json" in archive.namelist()


def prepare(
    *,
    apk: str | Path | None = None,
    from_device: str | None = None,
    from_apkpure: bool = False,
    out_dir: str | Path | None = None,
    run_rea: bool = True,
) -> dict:
    out = Path(out_dir) if out_dir else default_var_dir()
    out.mkdir(parents=True, exist_ok=True)
    blockers: list[str] = []
    pull = None
    obtain = None
    if from_device:
        try:
            pull = pull_from_device(from_device, out)
        except (RuntimeError, OSError, subprocess.TimeoutExpired) as exc:
            blockers.append(str(exc))
    if from_apkpure:
        try:
            obtain = download_apkpure(out)
        except (HTTPError, URLError, TimeoutError, OSError, ValueError, json.JSONDecodeError) as exc:
            blockers.append(f"APKPure download failed: {exc}")
    apk_path = Path(apk) if apk else None
    if apk_path is None and obtain is not None:
        apk_path = Path(obtain["path"])
    if apk_path is None:
        for candidate in (out / "base.apk", out / "clash-royale.apk", out / "clash-royale.xapk"):
            if candidate.is_file():
                apk_path = candidate
                break
    inspected = None
    xapk_info = None
    rea_result = None
    local_battle = None
    if apk_path is None or not apk_path.is_file():
        blockers.append(
            "No Clash Royale APK is on disk. The APKPure page is "
            f"{APKPURE_PAGE}. That HTML page returned HTTP 403 to curl on "
            "2026-10-16. The working download is "
            f"`cr-replays local-match-prepare --from-apkpure`, which fetches "
            f"{APKPURE_XAPK_URL} and follows redirects only to "
            "d.apkpure.net, d.cdnpure.com, and data.winudf.com. "
            "This tool never contacts Supercell."
        )
    else:
        try:
            source = apk_path
            if _is_xapk(source):
                xapk_info = describe_xapk(source)
                if not xapk_info["accepted_package"]:
                    blockers.append(
                        "XAPK package is "
                        f"{xapk_info['package_name']!r}, expected {PACKAGE}. "
                        "The base split was not extracted."
                    )
                else:
                    base = extract_base_apk(source, out / "base.apk")
                    inspected = inspect_apk_zip(base)
            else:
                inspected = inspect_apk_zip(source)
                sibling = source.parent / "clash-royale.xapk"
                if sibling.is_file() and sibling.resolve() != source.resolve():
                    try:
                        xapk_info = describe_xapk(sibling)
                    except (OSError, ValueError, zipfile.BadZipFile, json.JSONDecodeError) as exc:
                        blockers.append(f"Sibling XAPK could not be read: {exc}")
        except (OSError, ValueError, zipfile.BadZipFile, json.JSONDecodeError) as exc:
            blockers.append(f"APK could not be read: {exc}")
        if inspected is not None and not inspected["accepted"]:
            blockers.append(str(inspected["reject_reason"]))
        elif inspected is not None and inspected["accepted"]:
            loopback = _dex_has_loopback(Path(inspected["path"]))
            probe = xapk_info.get("native_loopback") if xapk_info else None
            probe = probe or {}
            flags = probe.get("loopback") or {}
            scanned = list(probe.get("scanned") or [])
            native_loopback = any(flags.get(name) for name in scanned)
            local_battle = {
                "executed_apk": False,
                "supercell_sockets": False,
                "dex_loopback": loopback,
                "native_loopback": native_loopback,
                "native_libraries_scanned": scanned,
                "real_engine": False,
            }
            blockers.append(
                _battle_blocker(
                    dex_loopback=loopback,
                    native_scanned=bool(scanned),
                    native_loopback=native_loopback,
                )
            )
            if run_rea:
                rea_result = rea_inspect(Path(inspected["path"]), out)
                if not rea_result.get("ran"):
                    blockers.append(str(rea_result.get("error")))
                elif rea_result.get("error"):
                    blockers.append(f"rea inspect-android-package failed: {rea_result['error']}")
                elif rea_result.get("returncode") not in (0, None):
                    tail = rea_result.get("stderr_tail") or ""
                    blockers.append(
                        "rea inspect-android-package exited "
                        f"{rea_result['returncode']}. {tail}".strip()
                    )
    if not shutil.which("adb") and not os.environ.get("REA_ADB_PATH"):
        blockers.append("adb is not installed, so a device pull cannot run here.")
    if not shutil.which("java"):
        blockers.append("A full JDK 17+ is required before `rea inspect-android-package`.")
    elif not _java_has_compiler():
        blockers.append("java is present but jdk.compiler is missing. Point JAVA_HOME at a full JDK.")
    blockers.append(REAL_ENGINE_BLOCKER)
    report = {
        "package": PACKAGE,
        "source": {
            "page": APKPURE_PAGE,
            "download_url": APKPURE_XAPK_URL,
            "allowed_hosts": sorted(ALLOWED_DOWNLOAD_HOSTS),
            "page_note": (
                "curl to the HTML page returned HTTP 403 (Cloudflare challenge) "
                "on 2026-10-16. The CDN URL is the transfer that returned the XAPK."
            ),
        },
        "out_dir": str(out),
        "obtain": obtain,
        "xapk": xapk_info,
        "apk": inspected,
        "local_battle": local_battle,
        "pull": pull,
        "rea": rea_result,
        "tools": {
            "java": shutil.which("java"),
            "jdk_compiler": _java_has_compiler(),
            "node": shutil.which("node"),
            "npm": shutil.which("npm"),
            "adb": shutil.which("adb") or os.environ.get("REA_ADB_PATH"),
            "rea": shutil.which("rea"),
            "jadx_jar": os.environ.get("REA_JADX_MCP_JAR"),
            "kvm": Path("/dev/kvm").exists(),
        },
        "blockers": blockers,
        "next_steps": [
            f"Page (often HTTP 403 to curl): {APKPURE_PAGE}",
            "cr-replays local-match-prepare --from-apkpure",
            f"Or: curl -fL -A 'Mozilla/5.0' -o var/clash-royale/clash-royale.xapk '{APKPURE_XAPK_URL}'",
            f"cr-replays local-match-prepare --apk {out / 'base.apk'}",
            "export REA_JADX_MCP_JAR=$HOME/.local/share/rea/android/jadx-headless-mcp-0.7.1-all.jar",
            "rea inspect-android-package var/clash-royale/base.apk --format json",
            "A combat resolver must listen on http://127.0.0.1:8790 and set real_engine true.",
            "cr-replays local-match",
            "Open http://127.0.0.1:8770 and pick both AIs and both decks.",
        ],
    }
    target = out / "prepare-status.json"
    target.write_text(json.dumps(report, indent=2), encoding="utf-8")
    report["status_file"] = str(target)
    return report


def main_prepare(
    apk: str | None,
    from_device: str | None,
    out_dir: str | None,
    from_apkpure: bool = False,
) -> dict:
    report = prepare(
        apk=apk,
        from_device=from_device,
        from_apkpure=from_apkpure,
        out_dir=out_dir,
    )
    shown = {
        key: report[key]
        for key in ("apk", "xapk", "obtain", "local_battle", "tools", "rea", "blockers", "status_file")
        if key in report
    }
    print(json.dumps(shown, indent=2))
    return report
