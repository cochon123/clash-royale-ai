#!/usr/bin/env python3
"""Import an unchanged recovered ELF without autoanalysis and bound selected functions by original FDEs."""
import argparse, csv, hashlib, json, os, pathlib, struct, subprocess, tempfile, time

ROOT = pathlib.Path(__file__).resolve().parents[1]
DEFAULT_RVAS = "fec180 fec49c fec3c0 8da088 8dddb8 8dc37c 1242f34 1243194 8cf3b8 8b1430 146096c 14616d4".split()

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def import_thunks(data):
    """Resolve exact AArch64 PLT stubs through the original ELF64 JMPREL table."""
    phoff, = struct.unpack_from("<Q", data, 32)
    phentsize, phnum = struct.unpack_from("<HH", data, 54)
    segments = [struct.unpack_from("<IIQQQQQQ", data, phoff+i*phentsize) for i in range(phnum)]
    def offset(address):
        for kind, _, file_offset, virtual, _, size, _, _ in segments:
            if kind == 1 and virtual <= address < virtual+size: return file_offset+address-virtual
        raise ValueError(f"Not file-backed: {address:x}")
    def virtual(file_offset):
        for kind, _, start, address, _, size, _, _ in segments:
            if kind == 1 and start <= file_offset < start+size: return address+file_offset-start
        raise ValueError("PLT candidate not in a LOAD segment")
    dynamic = next(segment for segment in segments if segment[0] == 2)
    tags = {}
    for position in range(dynamic[2],dynamic[2]+dynamic[5],16):
        tag,value=struct.unpack_from("<qQ",data,position)
        if not tag: break
        tags[tag]=value
    imports = {}
    for position in range(offset(tags[23]),offset(tags[23])+tags[2],24):
        got,info,_=struct.unpack_from("<QQq",data,position)
        nameoff,=struct.unpack_from("<I",data,offset(tags[6])+(info>>32)*tags[11])
        start=offset(tags[5])+nameoff
        imports[got]=data[start:data.index(0,start)].decode()
    thunks=[];position=0
    while True:
        end=data.find(b"\x20\x02\x1f\xd6",position)
        if end<0: break
        position=end+4
        if end<12: continue
        first,load,add,_=struct.unpack_from("<IIII",data,end-12)
        if first&0x9f00001f!=0x90000010 or load&0xffc003ff!=0xf9400211 or add&0xffc003ff!=0x91000210: continue
        address=virtual(end-12)
        immediate=((first>>5)&0x7ffff)<<2|((first>>29)&3)
        if immediate&(1<<20):immediate-=1<<21
        got=(address&~4095)+(immediate<<12)+((load>>10)&4095)*8
        if got in imports:thunks.append((address,imports[got],got))
    return thunks

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--elf", type=pathlib.Path, default=ROOT / "analysis/libg-cipher/libg.decrypted.so")
    parser.add_argument("--fde", type=pathlib.Path, default=ROOT / "analysis/native-engine-index/unwind-functions.tsv")
    parser.add_argument("--out", type=pathlib.Path, default=ROOT / "analysis/native-focused")
    parser.add_argument("--rva", nargs="+", default=DEFAULT_RVAS)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    selected = {int(value, 16) for value in args.rva}
    rows = list(csv.DictReader(args.fde.open(), delimiter="\t"))
    matches = [row for row in rows if int(row["start"], 16) in selected]
    if len(matches) != len(selected):
        raise SystemExit("Every target must be an exact original FDE start")
    targets = args.out.resolve() / "targets.tsv"
    with targets.open("w") as file:
        writer = csv.DictWriter(file, fieldnames=list(matches[0]), delimiter="\t")
        writer.writeheader(); writer.writerows(matches)
    script = ROOT / "tools/native_focus_decompile.java"
    imports_path = args.out.resolve() / "import-thunks.tsv"
    imports_path.write_text("rva\tname\tgot_rva\n"+"".join(f"0x{address:x}\t{name}\t0x{got:x}\n" for address,name,got in import_thunks(args.elf.read_bytes())))
    headless = ROOT / "tools/ghidra_12.1.4_PUBLIC/support/analyzeHeadless"
    temporary = tempfile.TemporaryDirectory(prefix="clash-focused-ghidra-")
    home = pathlib.Path(temporary.name)
    environment = os.environ.copy()
    environment["JAVA_HOME"] = str(pathlib.Path.home() / ".sdkman/candidates/java/21.0.3-tem")
    environment["GHIDRA_JAVA_HOME"] = environment["JAVA_HOME"]
    command = [str(headless), str(home), "focused", "-import", str(args.elf.resolve()),
        "-noanalysis", "-readOnly", "-deleteProject", "-max-cpu", "2", "-scriptPath", str(ROOT / "tools"),
        "-postScript", script.name, str(targets), str(args.out.resolve()), str(args.elf.resolve()), str(script), str(args.fde.resolve()), str(imports_path),
        "-log", str(args.out.resolve()/"ghidra.log"), "-scriptlog", str(args.out.resolve()/"script.log")]
    before = sha(args.elf)
    run_record={"command": command, "source_sha256": before,
        "fde_sha256": sha(args.fde), "runner_sha256": sha(pathlib.Path(__file__)), "script_sha256": sha(script),
        "headless_launcher_sha256": sha(headless),
        "ghidra_application_properties_sha256": sha(ROOT / "tools/ghidra_12.1.4_PUBLIC/Ghidra/application.properties")}
    (args.out / "run.json").write_text(json.dumps(run_record,indent=2)+"\n")
    (args.out / "manifest.json").unlink(missing_ok=True)
    started=time.monotonic()
    result = subprocess.run(command, env=environment)
    run_record.update(returncode=result.returncode,elapsed_seconds=time.monotonic()-started)
    (args.out / "run.json").write_text(json.dumps(run_record,indent=2)+"\n")
    if sha(args.elf) != before:
        raise SystemExit("Source ELF unexpectedly changed")
    manifest_path = args.out / "manifest.json"
    if manifest_path.exists():
        manifest = json.loads(manifest_path.read_text())
        data = args.elf.read_bytes()
        if data[:6] != b"\x7fELF\x02\x01":
            raise SystemExit("Expected ELF64 little-endian input")
        phoff, = struct.unpack_from("<Q", data, 32)
        phentsize, phnum = struct.unpack_from("<HH", data, 54)
        loads = []
        for index in range(phnum):
            ptype, _, offset, virtual, _, file_size, _, _ = struct.unpack_from("<IIQQQQQQ", data, phoff+index*phentsize)
            if ptype == 1: loads.append((virtual, virtual+file_size, offset))
        for function in manifest["functions"]:
            start, end = int(function["rva"],16), int(function["end_rva_exclusive"],16)
            for lower, upper, offset in loads:
                if lower <= start < end <= upper:
                    position = offset+start-lower
                    function["source_file_offset"] = hex(position)
                    function["source_file_bytes_sha256"] = hashlib.sha256(data[position:position+end-start]).hexdigest()
                    break
            else: raise SystemExit("Function does not fit a file-backed LOAD segment")
        manifest["source_unchanged_after_run"] = True
        manifest_path.write_text(json.dumps(manifest, indent=2)+"\n")
    temporary.cleanup()
    raise SystemExit(result.returncode)

if __name__ == "__main__": main()
