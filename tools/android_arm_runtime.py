#!/usr/bin/env python3
"""Run an isolated Android 10 ARM64 guest with full CPU emulation on x86 Linux.

Google's current launcher rejects this host/guest combination. Its direct ARM64
QEMU core can boot it after disabling PCI-only devices in the legacy ARM ranchu
machine. The local compatibility directory changes frontend metadata only; all
actual kernel, ramdisk, system, and vendor bytes remain the official API29 image.
"""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import subprocess
import time

import android_runtime as runtime

runtime.RUNTIME = runtime.ROOT / "analysis/android-arm-runtime"
runtime.AVD_NAME = "ClashNativeARM"
runtime.PORT = 5582
runtime.SERIAL = "emulator-5582"


def setup() -> None:
    directory = runtime.RUNTIME
    original = runtime.SDK / "system-images/android-29/default/arm64-v8a"
    if not (original / "system.img").exists():
        raise RuntimeError("Install system-images;android-29;default;arm64-v8a with sdkmanager first")
    for name in ("avd", "android-home", "compat-systemdir"):
        (directory / name).mkdir(parents=True, exist_ok=True)
    config = directory / f"avd/{runtime.AVD_NAME}.avd/config.ini"
    if not config.exists():
        runtime.run([str(runtime.SDK / "cmdline-tools/latest/bin/avdmanager"), "create", "avd",
                     "--name", runtime.AVD_NAME, "--package", "system-images;android-29;default;arm64-v8a",
                     "--device", "pixel_7", "--path", str(config.parent)], input="no\n")
    compat = directory / "compat-systemdir"
    for path in original.iterdir():
        destination = compat / path.name
        if path.name in ("build.prop", "source.properties", "package.xml"):
            text = path.read_text().replace("version.sdk=29", "version.sdk=25")
            text = text.replace("AndroidVersion.ApiLevel=29", "AndroidVersion.ApiLevel=25")
            text = text.replace("<api-level>29</api-level>", "<api-level>25</api-level>")
            text = text.replace("system-images;android-29;", "system-images;android-25;")
            if destination.is_symlink():
                destination.unlink()
            destination.write_text(text)
        elif not destination.exists():
            destination.symlink_to(path)
    values = dict(line.split("=", 1) for line in config.read_text().splitlines() if "=" in line)
    values.update({"target": "android-25", "image.sysdir.1": str(compat) + "/", "hw.ramSize": "2048",
                   "hw.cpu.ncore": "2", "hw.lcd.width": "720", "hw.lcd.height": "1280",
                   "hw.lcd.density": "280", "hw.gpu.enabled": "yes", "hw.gpu.mode": "swiftshader_indirect",
                   "hw.audioInput": "no", "hw.audioOutput": "no", "hw.camera.back": "none",
                   "hw.camera.front": "none", "hw.keyboard": "yes"})
    config.write_text("".join(f"{key}={value}\n" for key, value in values.items()))
    ini = directory / f"avd/{runtime.AVD_NAME}.ini"
    ini.write_text(f"avd.ini.encoding=UTF-8\npath={config.parent}\ntarget=android-25\n")
    print(f"Prepared ARM64 AVD {config.parent}; actual guest image remains Android API29")


def start(window: bool) -> None:
    state = runtime.adb("get-state", capture=True, check=False).stdout.strip()
    if state == "device":
        runtime.assert_ours()
        print(f"Already running: {runtime.SERIAL}")
        return
    # Check the recorded process too: first TCG boot can take several minutes
    # while ADB reports 'offline'. Do not launch two writers for the same AVD.
    pidfile = runtime.RUNTIME / "emulator.pid"
    if pidfile.exists():
        pid = int(pidfile.read_text().strip())
        try:
            command = Path(f"/proc/{pid}/cmdline").read_bytes()
        except FileNotFoundError:
            command = b""
        if b"qemu-system-aarch64" in command and b"ClashNativeARM" in command:
            print(f"ARM64 guest is still booting: PID {pid}, ADB state {state or 'absent'}")
            return
    setup()
    executable = runtime.SDK / "emulator/qemu/linux-x86_64/qemu-system-aarch64-headless"
    if window:
        executable = runtime.SDK / "emulator/qemu/linux-x86_64/qemu-system-aarch64"
    command = [str(executable), "-avd", runtime.AVD_NAME, "-port", str(runtime.PORT), "-accel", "off",
               "-no-snapshot", "-no-audio", "-no-boot-anim", "-gpu", "swiftshader_indirect",
               "-feature", "-Vulkan,-VirtioInput", "-http-proxy", "http://127.0.0.1:9"]
    if not window:
        command.append("-no-window")
    environment = runtime.env()
    environment["LD_LIBRARY_PATH"] = str(runtime.SDK / "emulator/lib64")
    with (runtime.RUNTIME / "emulator.log").open("ab") as log:
        process = subprocess.Popen(command, env=environment, stdout=log, stderr=subprocess.STDOUT,
                                   start_new_session=True)
    pidfile.write_text(f"{process.pid}\n")
    (runtime.RUNTIME / "launch-command.json").write_text(json.dumps(command, indent=2) + "\n")
    print(f"Started ARM64 TCG guest: PID {process.pid}. First boot is slow; inspect status/logcat.")


def ready() -> None:
    runtime.adb("wait-for-device")
    runtime.assert_ours()
    runtime.adb("root")
    runtime.adb("wait-for-device")
    # Current PCI input devices cannot attach to legacy ARM ranchu. Without a
    # directory at all, API29's EventHub aborts system_server during first boot.
    runtime.adb("shell", "mkdir", "-p", "/dev/input")
    runtime.adb("shell", "chmod", "755", "/dev/input")
    runtime.adb("shell", "restorecon", "/dev/input")
    deadline = time.monotonic() + 600
    while time.monotonic() < deadline:
        result = runtime.adb("shell", "getprop", "sys.boot_completed", capture=True, check=False)
        if result.stdout.strip() == "1":
            print("ARM64 Android API29 booted")
            runtime.offline()
            return
        time.sleep(3)
    raise RuntimeError("ARM64 framework has not completed boot; inspect logcat and emulator.log")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["setup", "start", "ready", "offline", "install", "launch", "capture", "status", "stop"])
    parser.add_argument("--window", action="store_true")
    args = parser.parse_args()
    if args.command == "start":
        start(args.window)
    elif args.command == "setup":
        setup()
    elif args.command == "ready":
        ready()
    elif args.command == "status":
        runtime.adb("get-state")
        runtime.assert_ours()
        runtime.adb("shell", "getprop", "ro.product.cpu.abilist")
        runtime.adb("shell", "getprop", "ro.build.version.sdk")
        runtime.adb("shell", "getprop", "sys.boot_completed")
    elif args.command == "stop":
        runtime.assert_ours()
        runtime.adb("emu", "kill")
    else:
        getattr(runtime, args.command)()


if __name__ == "__main__":
    try:
        main()
    except (RuntimeError, subprocess.SubprocessError) as exc:
        raise SystemExit(str(exc))
