#!/usr/bin/env python3
"""Run the supplied original APK in a fresh, offline, project-owned Android AVD.

This is a reverse engineering runtime. It does not implement a battle server or
claim the original battle engine has been ported.
"""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import time
import zipfile

ROOT = Path(__file__).resolve().parents[1]
RUNTIME = ROOT / "analysis" / "android-runtime"
AVD_NAME = "ClashNativeRE"
PORT = 5580
SERIAL = f"emulator-{PORT}"
PACKAGE = "com.supercell.clashroyale"
SDK = Path(os.environ.get("CLASH_ANDROID_SDK", str(Path.home() / "Android/Sdk")))


def env() -> dict[str, str]:
    result = os.environ.copy()
    result.update(ANDROID_HOME=str(SDK), ANDROID_AVD_HOME=str(RUNTIME / "avd"),
                  ANDROID_USER_HOME=str(RUNTIME / "android-home"))
    return result


def run(args: list[str], *, capture: bool = False, timeout: int = 60,
        check: bool = True, input: str | None = None) -> subprocess.CompletedProcess:
    return subprocess.run(args, text=True, capture_output=capture, check=check,
                          timeout=timeout, env=env(), input=input)


def adb(*args: str, capture: bool = False, timeout: int = 60,
        check: bool = True) -> subprocess.CompletedProcess:
    executable = SDK / "platform-tools/adb"
    return run([str(executable), "-s", SERIAL, *args], capture=capture,
               timeout=timeout, check=check)


def assert_ours() -> None:
    result = adb("emu", "avd", "name", capture=True).stdout.splitlines()
    if AVD_NAME not in result:
        raise RuntimeError(f"{SERIAL} is not the project AVD {AVD_NAME}; refusing to change it")


def setup() -> None:
    if not (SDK / "emulator/emulator").is_file():
        raise RuntimeError(f"Android SDK emulator not found in {SDK}; set CLASH_ANDROID_SDK")
    (RUNTIME / "avd").mkdir(parents=True, exist_ok=True)
    (RUNTIME / "android-home").mkdir(parents=True, exist_ok=True)
    config = RUNTIME / "avd" / f"{AVD_NAME}.avd" / "config.ini"
    if not config.exists():
        run([str(SDK / "cmdline-tools/latest/bin/avdmanager"), "create", "avd",
             "--name", AVD_NAME, "--package", "system-images;android-35;google_apis;x86_64",
             "--device", "pixel_7", "--path", str(config.parent)], input="no\n")
    values = {line.split("=", 1)[0]: line.split("=", 1)[1]
              for line in config.read_text().splitlines() if "=" in line}
    values.update({"hw.ramSize": "3072", "hw.lcd.width": "720", "hw.lcd.height": "1600",
                   "hw.lcd.density": "280", "hw.gpu.enabled": "yes",
                   "hw.gpu.mode": "swiftshader_indirect", "hw.camera.back": "none",
                   "hw.camera.front": "none", "hw.audioInput": "no",
                   "hw.audioOutput": "no", "hw.keyboard": "yes"})
    config.write_text("".join(f"{key}={value}\n" for key, value in values.items()))
    print(f"Project AVD: {config.parent}")


def start(window: bool) -> None:
    setup()
    state = adb("get-state", capture=True, check=False).stdout.strip()
    if state == "device":
        assert_ours()
        print(f"Already running: {SERIAL}")
        return
    command = [str(SDK / "emulator/emulator"), "-avd", AVD_NAME, "-port", str(PORT),
               "-no-snapshot", "-no-boot-anim", "-no-audio", "-gpu", "swiftshader_indirect",
               "-memory", "3072", "-cores", "4", "-http-proxy", "http://127.0.0.1:9"]
    if not window:
        command.append("-no-window")
    with (RUNTIME / "emulator.log").open("ab") as log:
        process = subprocess.Popen(command, env=env(), stdout=log, stderr=subprocess.STDOUT,
                                   start_new_session=True)
    (RUNTIME / "emulator.pid").write_text(f"{process.pid}\n")
    (RUNTIME / "launch-command.json").write_text(json.dumps(command, indent=2) + "\n")
    print(f"Started {SERIAL}, PID {process.pid}. Run 'ready' after boot.")


def ready() -> None:
    adb("wait-for-device", timeout=60)
    assert_ours()
    deadline = time.monotonic() + 180
    while time.monotonic() < deadline:
        if adb("shell", "getprop", "sys.boot_completed", capture=True).stdout.strip() == "1":
            print("Android booted")
            offline()
            return
        time.sleep(2)
    raise RuntimeError("Android has not completed boot; inspect emulator.log")


def offline() -> None:
    assert_ours()
    adb("root")
    adb("wait-for-device")
    airplane = adb("shell", "cmd", "connectivity", "airplane-mode", "enable",
                   capture=True, check=False)
    if airplane.returncode or "Unknown command" in airplane.stdout or "No shell command" in airplane.stdout:
        adb("shell", "settings", "put", "global", "airplane_mode_on", "1")
        adb("shell", "am", "broadcast", "-a", "android.intent.action.AIRPLANE_MODE", "--ez", "state", "true")
    adb("shell", "settings", "put", "global", "http_proxy", "127.0.0.1:9")
    packages = adb("shell", "cmd", "package", "list", "packages", "-U", PACKAGE,
                   capture=True).stdout
    match = re.search(r"uid:(\d+)", packages)
    if match:
        uid = match.group(1)
        for binary in ("iptables", "ip6tables"):
            # The package gets no network, including any direct native sockets.
            check = adb("shell", binary, "-C", "OUTPUT", "-m", "owner", "--uid-owner", uid,
                        "-j", "REJECT", capture=True, check=False)
            if check.returncode:
                adb("shell", binary, "-I", "OUTPUT", "-m", "owner", "--uid-owner", uid,
                    "-j", "REJECT")
    for service in ("wifi", "data"):
        try:
            adb("shell", "svc", service, "disable", capture=True, timeout=15, check=False)
        except subprocess.TimeoutExpired:
            print(f"Android {service} service did not return; package firewall remains enforced")
    print("Airplane mode requested, dead HTTP proxy; package network rejected when installed")


def install() -> None:
    assert_ours()
    offline()
    names = ["com.supercell.clashroyale.apk", "config.arm64_v8a.apk", "config.xxhdpi.apk",
             "config.en.apk", "install_time_asset_pack.apk"]
    paths = [ROOT / "analysis/apks" / name for name in names]
    xapk = ROOT / "Clash+Royale_160402020_APKPure.xapk"
    if xapk.is_file():
        with zipfile.ZipFile(xapk) as archive:
            for path in paths:
                if not path.exists():
                    path.write_bytes(archive.read(path.name))
    for path in paths:
        if not path.is_file():
            raise RuntimeError(f"Missing supplied split: {path}")
    adb("install-multiple", "-r", *map(str, paths), timeout=240)
    offline()
    print("Original, unmodified supplied APK splits installed")


def launch() -> None:
    assert_ours()
    offline()
    adb("shell", "input", "keyevent", "KEYCODE_WAKEUP")
    adb("shell", "wm", "dismiss-keyguard")
    adb("logcat", "-c")
    adb("shell", "am", "start", "-n", f"{PACKAGE}/com.supercell.titan.GameApp")


def capture() -> None:
    assert_ours()
    result = subprocess.run([str(SDK / "platform-tools/adb"), "-s", SERIAL,
                             "exec-out", "screencap", "-p"], capture_output=True,
                            check=True, timeout=30, env=env())
    (RUNTIME / "native-ui.png").write_bytes(result.stdout)
    (RUNTIME / "logcat.txt").write_text(adb("logcat", "-d", "-v", "threadtime", capture=True).stdout)
    for name, args in {"properties.txt": ["getprop"], "connectivity.txt": ["dumpsys", "connectivity"],
                       "package.txt": ["dumpsys", "package", PACKAGE],
                       "activities.txt": ["dumpsys", "activity", "activities"]}.items():
        (RUNTIME / name).write_text(adb("shell", *args, capture=True).stdout)
    print(f"Saved {RUNTIME / 'native-ui.png'} and runtime evidence")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["setup", "start", "ready", "status", "offline", "install", "launch", "capture", "stop"])
    parser.add_argument("--window", action="store_true", help="show emulator window when starting")
    args = parser.parse_args()
    if args.command == "start":
        start(args.window)
    elif args.command == "status":
        adb("get-state")
        assert_ours()
        adb("shell", "getprop", "ro.product.cpu.abilist")
        adb("shell", "getprop", "ro.dalvik.vm.native.bridge")
    elif args.command == "stop":
        assert_ours()
        adb("emu", "kill")
    else:
        globals()[args.command]()


if __name__ == "__main__":
    try:
        main()
    except (RuntimeError, subprocess.SubprocessError) as exc:
        print(str(exc), file=sys.stderr)
        sys.exit(1)
