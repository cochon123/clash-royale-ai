#!/usr/bin/env python3
"""Repeat the static REA queries; --native also decompiles the initializer."""
import argparse
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--native", action="store_true")
    args = parser.parse_args()
    rea = ROOT / "tools/rea/node_modules/.bin/rea"
    if not rea.exists():
        raise SystemExit("Run python3 tools/bootstrap_analysis.py first")
    env = os.environ | {
        "REA_JADX_MCP_JAR": str(ROOT / "tools/jadx-headless-mcp-0.7.1-all.jar"),
        "GHIDRA_INSTALL_DIR": str(ROOT / "tools/ghidra_12.1.4_PUBLIC"),
        "REA_GHIDRA_STARTUP_TIMEOUT_MS": "900000", "GHIDRA_HEADLESS_MAXMEM": "4G",
    }
    base = str(ROOT / "analysis/apks/com.supercell.clashroyale.apk")
    queries = [
        ("rea-xapk.json", ["inspect-artifact", str(ROOT / "Clash+Royale_160402020_APKPure.xapk")]),
        ("rea-android-package.json", ["inspect-android-package", base]),
        ("rea-gameapp.json", ["inspect-android-class", base, "com.supercell.titan.GameApp"]),
    ]
    if args.native:
        queries.append(("rea-libg-entry-decompile.json", ["decompile",
            str(ROOT / "analysis/native/lib/arm64-v8a/libg.so"), "--file-offset", "0x1adbc20",
            "--provider", "ghidra", "--snapshot", str(ROOT / "analysis/libg.snapshot.json")]))
    for filename, command in queries:
        print("REA:", " ".join(command), flush=True)
        with (ROOT / "analysis" / filename).open("w") as output:
            subprocess.run([str(rea), *command, "--format", "json"], env=env, stdout=output, check=True)


if __name__ == "__main__":
    main()
