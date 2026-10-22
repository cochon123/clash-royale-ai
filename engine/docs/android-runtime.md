# Original Android client runtime investigation

The original supplied Clash Royale APK can be installed and its ARM64 startup
code executes on this desktop. Native battle UI and a playable match have **not**
been reached: the protected startup deliberately dispatches an obfuscated Java
exception before `libg.so` appears in the observed process maps.

## Isolated runtime

The existing SDK at `/home/cochon/Android/Sdk` supplies Android Emulator
37.1.11.0 and `system-images;android-35;google_apis;x86_64` revision 9. KVM is
usable. The Google image advertises `x86_64,arm64-v8a` and loads ARM64 code through
`libndk_translation.so`. No SDK image download or system package installation was
necessary. Existing Waydroid and the user's `Luna_Music_Phone` AVD were preserved.

All new writable Android data is under `analysis/android-runtime/avd/` and
`analysis/android-runtime/android-home/`. The new AVD is `ClashNativeRE`, ADB
serial `emulator-5580`, with a 720×1600 display, software graphics, and 3 GiB RAM.
The scripts verify the AVD name before changing its state.

```sh
python3 tools/android_runtime.py start --window
python3 tools/android_runtime.py ready
python3 tools/android_runtime.py install
python3 tools/android_runtime.py launch
python3 tools/android_runtime.py capture
python3 tools/android_runtime.py stop
```

Omit `--window` for a headless emulator. Set `CLASH_ANDROID_SDK` to use another
existing SDK with the same image. Original APK splits are installed unchanged.
The base manifest requires both ABI and density splits; installing only the
three previously extracted APKs fails with `INSTALL_FAILED_MISSING_SPLIT`.
`install` also extracts `config.xxhdpi.apk` and `config.en.apk` from the supplied
XAPK and installs all five required/selected splits.

The lab remains offline: airplane mode is enabled, Wi-Fi and cellular data are
disabled, and an unreachable HTTP proxy is set. After package installation,
IPv4 and IPv6 output rules reject the app UID, including direct native sockets.
No Google, Supercell, or user account was used; no live match was requested.

## Runtime observations

`analysis/android-runtime/logcat.txt` records loading
`lib/arm64/libsupercell_clashroyale.so` successfully. The original startup then
reports `lskumcyej.aI: !...` from `lskumcyej.W.a`, `W.b`, and `q.run`.
Retrying after unrooting adbd produced the same fault. The exact reason encoded
in that status is still unknown; this does not prove that root, translation,
emulator detection, missing services, or any particular condition caused it.

REA/JADX evidence:

| Evidence | Observation |
| --- | --- |
| `ev_42f07697e8770197f4907296e390f151e3d6d3c99c6eeee15fa5cbcb49b7df15` | `lskumcyej.W` method and field inventory. |
| `ev_8ebef73fdbc99aa494d1341d4317963aceb30af9bd02c4aab262ebf105385082` | `q.run` waits for native status and calls `W.b(String)`. |
| `ev_26d40a9b0b7d0834939c80a39975c0b47bff2a77b5fa4dbd1c3a618ac30fcf6d` | `W.a(String)` interprets native status and constructs exception types. |
| `ev_8d7e72845d4fa8d64e004f977b714944cec7fffeaba109ca702d9afc756b3bf7` | `W.b(String)` explicitly invokes the default uncaught exception handler. |

The complete REA records are retained as `analysis/android-runtime/rea-startup-*.json`.
`native-ui.png` is the observed Android startup/crash screen, not a successful
render of the original battle UI.

## Memory evidence

Frida 17.24.0 and frida-tools 14.11.0 are installed in `tools/android-env`.
The x86_64 server archive was verified against GitHub's release digest
`922f8cc93a86977b05fc2b8a6e47a473367d6e367b9a3a7232d2962e7cff04f1`;
identity is recorded in `analysis/android-runtime/frida-server-identity.json`.
The running process is x86_64; its ARM64 libraries are mapped by NativeBridge.
Their presence is observed through `/proc/self/maps`, independently of Frida's
native-module enumeration.

`tools/android_memory_probe.js` records maps and selected native ranges during
startup. `tools/android_startup_probe.js` is a separate active experiment that
suppresses the Java error dispatcher. That suppression did not reach game UI
or establish working battle execution. The passive memory probe did reach a
state with parent PID 4591 blocked on a pipe and child PID 4613 running, both
with the same package name. Neither observed map contained `libg.so`.

The live companion memory image is
`analysis/android-runtime/memory/companion-live-parent-memory.bin`, 0x6d3000
bytes, base `0x7fda04465000`, SHA-256
`747a1754c91ace13e3f2216638b187c2f66d2084897ee5b239fcd00e83cd4424`.
Its JSON manifest records readable mapping ranges and hashes; holes in the
image are zero-filled. This is a virtual-memory image, **not** a reconstructed
ELF file. The observed initialized callback slots are:

| Companion slot | Observed value | Relative to module base |
| --- | --- | --- |
| `0x6c6468` | `0x7fda04aa949c` | `0x64449c` |
| `0x6c6470` | `0x7fda04604ee0` | `0x19fee0` |

To repeat an exact live mapping dump after finding its current PID:

```sh
adb -s emulator-5580 shell pidof com.supercell.clashroyale
python3 tools/android_dump_memory.py --pid 4591 --module libsupercell_clashroyale.so
```

The ARM64 guest Bionic libc was pulled to
`analysis/android-runtime/libc-arm64.so`, SHA-256
`2ff4bf0e733a1103333d0742890d66e88d7aabe05c0f977f3b2d1ae470269b4c`.
Its observed runtime base was `0x7fda15ac7000`; live range snapshots and the
process maps are retained in `analysis/android-runtime/memory/`.

Official references: [Android emulator command line](https://developer.android.com/studio/run/emulator-commandline),
[ARM application translation in Google's emulator images](https://android-developers.googleblog.com/2020/03/run-arm-apps-on-android-emulator.html),
[Frida Android instrumentation setup](https://frida.re/docs/android/).
