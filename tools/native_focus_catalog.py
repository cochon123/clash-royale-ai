#!/usr/bin/env python3
"""Index direct assembly calls and printable constant candidates from bounded decompilations."""
import argparse, hashlib, json, pathlib, re, struct

ROOT = pathlib.Path(__file__).resolve().parents[1]

def digest(data): return hashlib.sha256(data).hexdigest()

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=pathlib.Path)
    args = parser.parse_args()
    manifest_path = args.directory / "manifest.json"
    manifest = json.loads(manifest_path.read_text())
    data = pathlib.Path(manifest["source_elf"]).read_bytes()
    assert digest(data) == manifest["source_sha256"]
    phoff, = struct.unpack_from("<Q", data, 32)
    phentsize, phnum = struct.unpack_from("<HH", data, 54)
    segments = []
    for index in range(phnum):
        kind, _, off, virtual, _, size, _, _ = struct.unpack_from("<IIQQQQQQ", data, phoff+index*phentsize)
        if kind == 1: segments.append((virtual, virtual+size, off))
    def file_offset(rva):
        for lower, upper, offset in segments:
            if lower <= rva < upper: return offset+rva-lower
        return None
    image_base = int(manifest["image_base"],16)
    records = []
    for function in manifest["functions"]:
        stem = f"rva_{int(function['rva'],16):x}"
        code = (args.directory / (stem+".c")).read_text()
        assembly = (args.directory / (stem+".asm")).read_text()
        calls = []
        for instruction, operation, target in re.findall(r"(?m)^([0-9a-f]+)\s+(bl|b)\s+0x([0-9a-f]+)$",assembly):
            calls.append({"instruction_rva":hex(int(instruction,16)-image_base),"kind":operation,"target_rva":hex(int(target,16)-image_base)})
        constants = []
        for address in sorted({int(value,16) for value in re.findall(r"\bUNK_([0-9a-fA-F]+)\b",code)}):
            rva=address-image_base; offset=file_offset(rva)
            if offset is None: continue
            stop=data.find(b"\0",offset,min(offset+4096,len(data)))
            if stop < 0: continue
            raw=data[offset:stop]
            if len(raw)<2: continue
            try: value=raw.decode("utf-8")
            except UnicodeDecodeError: continue
            if not all(character.isprintable() or character in "\n\r\t" for character in value): continue
            constants.append({"program_address":hex(address),"rva":hex(rva),"file_offset":hex(offset),"candidate_utf8":value,"bytes_sha256":digest(raw)})
        records.append({"rva":function["rva"],"direct_branches":calls,"printable_constant_candidates":constants})
    (args.directory / "catalog.json").write_text(json.dumps({"source_sha256":manifest["source_sha256"],"manifest_sha256":digest(manifest_path.read_bytes()),"catalog_script_sha256":digest(pathlib.Path(__file__).read_bytes()),"limits":["Direct calls/jumps come from the original bounded assembly; indirect calls are not resolved.","Printable bytes referenced through Ghidra UNK labels are candidates, not proven string types or semantics.","Program addresses include Ghidra's image base; RVA fields do not."],"functions":records},indent=2)+"\n")

if __name__ == "__main__": main()
