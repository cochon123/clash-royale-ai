#!/usr/bin/env python3
"""Extract the selected APKs and native libraries without executing them."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import zipfile

ROOT = Path(__file__).resolve().parents[1]
EXPECTED = "3dc0085605d495ee40b443084f7767ecc547d329be30b5f11884b0d345643f13"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("xapk", nargs="?", type=Path,
                        default=ROOT / "Clash+Royale_160402020_APKPure.xapk")
    args = parser.parse_args()
    with args.xapk.open("rb") as source:
        digest = hashlib.file_digest(source, "sha256").hexdigest()
    if digest != EXPECTED:
        raise SystemExit("Source differs from the analyzed build; select and analyze that version explicitly.")
    output = ROOT / "analysis"
    selected = ["com.supercell.clashroyale.apk", "config.arm64_v8a.apk", "install_time_asset_pack.apk"]
    with zipfile.ZipFile(args.xapk) as archive:
        for name in selected:
            destination = output / "apks" / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            with archive.open(name) as source, destination.open("wb") as sink:
                shutil.copyfileobj(source, sink)
            with zipfile.ZipFile(destination) as package:
                inventory = [{"path": i.filename, "size": i.file_size, "compressed": i.compress_size}
                             for i in package.infolist()]
                (output / (destination.stem + ".inventory.json")).write_text(json.dumps(inventory, indent=2) + "\n")
                for member in package.namelist():
                    path = Path(member)
                    if member.startswith("lib/arm64-v8a/") and path.name.endswith(".so") and len(path.parts) == 3:
                        native = output / "native" / path
                        native.parent.mkdir(parents=True, exist_ok=True)
                        native.write_bytes(package.read(member))
    print(json.dumps({"source_sha256": digest, "apks": selected, "executed_target": False}))


if __name__ == "__main__":
    main()
