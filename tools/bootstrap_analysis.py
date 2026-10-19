#!/usr/bin/env python3
"""Install pinned REA/JADX/Ghidra tools locally without changing agent configuration."""
import hashlib
from pathlib import Path
import shutil
import subprocess
import urllib.request
import zipfile

ROOT = Path(__file__).resolve().parents[1]
TOOLS = ROOT / "tools"
DOWNLOADS = [
    ("jadx-headless-mcp-0.7.1-all.jar",
     "https://github.com/1013503897/jadx-headless-mcp/releases/download/v0.7.1/jadx-headless-mcp-0.7.1-all.jar",
     "6e5eacf500b64292bfb73c49797c1958f6ee44646e43e868039ae7feb573ff75"),
    ("ghidra.zip",
     "https://github.com/NationalSecurityAgency/ghidra/releases/download/Ghidra_12.1.4_build/ghidra_12.1.4_PUBLIC_20260921.zip",
     "ddac49f903da9d5bac833e5cc79395098b9c33cfd3279be5f31bd00387d2d4db"),
]


def main():
    for filename, url, expected in DOWNLOADS:
        path = TOOLS / filename
        if not path.exists():
            pending = path.with_suffix(path.suffix + ".partial")
            print(f"Downloading {filename}", flush=True)
            with urllib.request.urlopen(url) as response, pending.open("wb") as sink:
                shutil.copyfileobj(response, sink)
            pending.rename(path)
        with path.open("rb") as source:
            actual = hashlib.file_digest(source, "sha256").hexdigest()
        if actual != expected:
            raise SystemExit(f"Digest mismatch for {filename}; refusing to execute it")
        print(f"Verified {filename}: {actual}", flush=True)
    if not (TOOLS / "ghidra_12.1.4_PUBLIC").exists():
        with zipfile.ZipFile(TOOLS / "ghidra.zip") as archive:
            for member in archive.infolist():
                if Path(member.filename).is_absolute() or ".." in Path(member.filename).parts:
                    raise SystemExit("Unsafe release archive member")
            archive.extractall(TOOLS)
            for member in archive.infolist():
                mode = member.external_attr >> 16
                if mode & 0o111:
                    (TOOLS / member.filename).chmod(mode & 0o777)
    subprocess.run(["npm", "install", "--prefix", str(TOOLS / "rea"), "rea-agents@6.5.0", "--omit=optional"], check=True)
    print("Analysis tools ready. A full JDK 21+ and compatible Node are also required.")


if __name__ == "__main__":
    main()
