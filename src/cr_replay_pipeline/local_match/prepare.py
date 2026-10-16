"""Locate a local Clash Royale APK and record what REA can inspect.

The APK stays under ``var/clash-royale/`` (gitignored). Nothing here contacts
Supercell. REA's documented intake is a caller-supplied file or
``rea pull-adb-package`` from a device you control.
"""

from __future__ import annotations

import hashlib
import json
import os
import shutil
import subprocess
import zipfile
from pathlib import Path

from .catalog import repo_root
from .engine import REAL_ENGINE_BLOCKER

PACKAGE = "com.supercell.clashroyale"


def default_var_dir() -> Path:
    return repo_root() / "var" / "clash-royale"


def inspect_apk_zip(path: Path) -> dict:
    """Hash the archive and record native library names. Do not extract it."""
    path = Path(path)
    if not path.is_file():
        raise FileNotFoundError(path)
    if not zipfile.is_zipfile(path):
        raise ValueError(f"{path} is not a zip/APK")
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    with zipfile.ZipFile(path) as archive:
        names = archive.namelist()
    native = sorted(
        {
            name.rsplit("/", 1)[-1]
            for name in names
            if name.startswith("lib/") and name.endswith(".so")
        }
    )
    return {
        "path": str(path),
        "sha256": digest.hexdigest(),
        "bytes": path.stat().st_size,
        "entries": len(names),
        "has_android_manifest": "AndroidManifest.xml" in names,
        "has_classes_dex": any(
            name.startswith("classes") and name.endswith(".dex") for name in names
        ),
        "native_libraries": native,
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


def rea_inspect(apk: Path) -> dict:
    command = ["rea", "inspect-android-package", str(apk), "--json"]
    rea = shutil.which("rea")
    if not rea:
        return {
            "ran": False,
            "command": command,
            "error": (
                "rea is not on PATH. Install Node.js 22+ and run "
                "`npm install --global rea-agents`. Download the pinned "
                "jadx-headless-mcp 0.7.1 jar, verify its SHA-256 "
                "6e5eacf500b64292bfb73c49797c1958f6ee44646e43e868039ae7feb573ff75, "
                "and set REA_JADX_MCP_JAR. See rea docs/android-analysis.md."
            ),
        }
    try:
        proc = subprocess.run(
            [rea, *command[1:]],
            capture_output=True,
            text=True,
            timeout=180,
            check=False,
        )
    except (OSError, subprocess.TimeoutExpired) as exc:
        return {"ran": True, "command": command, "error": str(exc)}
    # Keep the raw JADX dump out of git. Callers write it under var/ if they want it.
    return {
        "ran": True,
        "command": command,
        "returncode": proc.returncode,
        "stdout_chars": len(proc.stdout),
        "stderr_tail": proc.stderr[-800:],
    }


def prepare(
    *,
    apk: str | Path | None = None,
    from_device: str | None = None,
    out_dir: str | Path | None = None,
) -> dict:
    out = Path(out_dir) if out_dir else default_var_dir()
    out.mkdir(parents=True, exist_ok=True)
    blockers: list[str] = []
    pull = None
    if from_device:
        try:
            pull = pull_from_device(from_device, out)
        except (RuntimeError, OSError, subprocess.TimeoutExpired) as exc:
            blockers.append(str(exc))
    apk_path = Path(apk) if apk else None
    if apk_path is None:
        for candidate in (out / "base.apk", out / "clash-royale.apk"):
            if candidate.is_file():
                apk_path = candidate
                break
    inspected = None
    rea_result = None
    if apk_path is None or not apk_path.is_file():
        blockers.append(
            "No Clash Royale APK is on disk. REA expects a caller-supplied file "
            f"or `rea pull-adb-package <serial> {PACKAGE}`. Put the APK at "
            f"{out / 'clash-royale.apk'}. This tool will not download one and "
            "will not contact Supercell. On 2026-10-16 a HEAD to APKMirror's "
            "public Clash Royale page returned HTTP 403 (Cloudflare challenge)."
        )
    else:
        try:
            inspected = inspect_apk_zip(apk_path)
        except (OSError, ValueError, zipfile.BadZipFile) as exc:
            blockers.append(f"APK could not be read: {exc}")
        else:
            rea_result = rea_inspect(apk_path)
            if not rea_result.get("ran"):
                blockers.append(str(rea_result.get("error")))
    if not shutil.which("adb") and not os.environ.get("REA_ADB_PATH"):
        blockers.append("adb is not installed, so a device pull cannot run here.")
    if not shutil.which("java"):
        blockers.append("A full JDK 17+ is required before `rea inspect-android-package`.")
    elif not _java_has_compiler():
        blockers.append("java is present but jdk.compiler is missing. Point JAVA_HOME at a full JDK.")
    blockers.append(REAL_ENGINE_BLOCKER)
    report = {
        "package": PACKAGE,
        "out_dir": str(out),
        "apk": inspected,
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
            "On the 3060 machine, install Android platform-tools and Node.js 22.",
            "Install Clash Royale from the Play Store on a phone you control.",
            f"rea pull-adb-package <serial> {PACKAGE} --output-directory {out}",
            "export REA_JADX_MCP_JAR=$HOME/.local/share/rea/android/jadx-headless-mcp-0.7.1-all.jar",
            f"cr-replays local-match-prepare --apk {out / 'base.apk'}",
            "cr-replays local-match",
            "Open http://127.0.0.1:8770 and pick both AIs and both decks.",
        ],
    }
    target = out / "prepare-status.json"
    target.write_text(json.dumps(report, indent=2), encoding="utf-8")
    report["status_file"] = str(target)
    return report


def main_prepare(apk: str | None, from_device: str | None, out_dir: str | None) -> dict:
    report = prepare(apk=apk, from_device=from_device, out_dir=out_dir)
    print(json.dumps({k: report[k] for k in ("apk", "tools", "blockers", "status_file")}, indent=2))
    return report
