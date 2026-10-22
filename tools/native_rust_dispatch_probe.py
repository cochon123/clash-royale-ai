#!/usr/bin/env python3
"""Check dense-dispatch output by reversing its ID renaming byte-for-byte.

Unlike replay coverage, this checks every generated body and every constant
transfer. It complements execution tests; it is not a compiler correctness test.
"""
import argparse
import json
from pathlib import Path
import re


def functions(path):
    text = path.read_text()
    starts = list(re.finditer(r"(?m)^#\[inline\]\npub\(super\) unsafe fn (f_[0-9a-f]+)\(", text))
    return {m[1]: text[m.start():starts[i+1].start() if i+1 < len(starts) else len(text)]
            for i, m in enumerate(starts)}


def normalize_error_carrier(text):
    text = text.replace("Result<(),crate::fault::Fault>", "Result<(),String>")
    return re.sub(r'Err\((format!\("(?:[^"\\]|\\.)*"\))\.into\(\)\)', r'Err(\1)', text)


def check(before, after):
    manifest = json.loads((after / "manifest.json").read_text())
    checked = changed = blocks = transfers = 0
    for path in sorted(before.glob("chunk*.rs")):
        original, candidate = functions(path), functions(after / path.name)
        assert original.keys() == candidate.keys(), (path.name, "function set changed")
        for name, body in original.items():
            checked += 1
            current = normalize_error_carrier(candidate[name])
            if current == body:
                continue
            changed += 1
            old, new = body.splitlines(), current.splitlines()
            addresses = re.findall(r"(?m)^(0x[0-9a-f]+)=>\{$", body)
            entry = re.search(r"let mut block:u32=if pc==(0x[0-9a-f]+)\{0\}else\{match pc\{(.*?)_=>", new[1])
            assert entry and entry[1] == addresses[0], (name, "initial block changed")
            mapping = re.findall(r"(0x[0-9a-f]+)=>([0-9]+),", entry[2])
            assert mapping == [(address, str(i)) for i, address in enumerate(addresses)], (name, "entry mapping changed")
            inverse = {index: address for address, index in mapping}
            ids = re.findall(r"(?m)^([0-9]+)=>\{$", current)
            assert ids == list(inverse), (name, "block entries changed")
            prefix = old[1].split(">{loop{match pc{")[0]
            assert new[1].startswith(prefix + ">{let mut block:u32="), (name, "function signature changed")
            assert new[1].endswith("}};loop{match block{"), (name, "dispatch shape changed")
            # The entry error must retain the original ARM address and wording.
            error = old[-1][:-3]
            assert new[1].endswith(error + "}};loop{match block{"), (name, "entry error changed")
            assert new[-1] == old[-1].replace("untranslated block {pc:#x}", "invalid internal block {block}"), name
            restored = [old[0], old[1]]
            for line in new[2:-1]:
                if m := re.fullmatch(r"([0-9]+)=>\{", line):
                    line = inverse[m[1]] + "=>{"
                transfers += len(re.findall(r"(?<![\w.])block=([0-9]+);continue;", line))
                line = re.sub(r"(?<![\w.])block=([0-9]+);continue;",
                              lambda m: "pc=" + inverse[m[1]] + ";continue;", line)
                restored.append(line)
            restored.append(old[-1])
            assert "\n".join(restored) == "\n".join(old), (name, "instruction/control body changed")
            blocks += len(addresses)
    expected = sum(v["applied"] for v in manifest["dense_dispatch"].values())
    assert changed == expected, (changed, expected)
    return dict(matched=True, functions_checked=checked, functions_renamed=changed,
                dense_blocks=blocks, transfers_checked=transfers,
                compact_fault=manifest.get("compact_fault", False),
                scope="Bijective entry mapping and byte-exact inverse renaming of all generated instruction/control bodies, normalizing the error carrier type and message conversion.")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--before", type=Path, required=True)
    parser.add_argument("--after", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    report = check(args.before, args.after)
    args.out.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
