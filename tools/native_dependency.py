"""Map a dependent ARM64 ELF and apply its dynamic relocations in Unicorn."""

import hashlib
from pathlib import Path

from elftools.elf.elffile import ELFFile
from elftools.elf.relocation import RelocationTable


PAGE = 0x1000
RELOCATION_TYPES = {257, 1025, 1026}


def _align(value):
    return (value + PAGE - 1) & -PAGE


def map_elf_dependency(uc, source: Path, base: int, external_names: dict[int, str], known_symbols: dict[str, int]):
    source = Path(source).resolve()
    blob = source.read_bytes()
    source_sha256 = hashlib.sha256(blob).hexdigest()
    with source.open("rb") as stream:
        elf = ELFFile(stream)
        if elf.elfclass != 64 or not elf.little_endian or elf["e_machine"] != "EM_AARCH64":
            raise ValueError(f"Expected ARM64 ELF64 little-endian dependency: {source}")
        if base % PAGE:
            raise ValueError("Dependency load base must be page-aligned")
        segments = [s for s in elf.iter_segments() if s["p_type"] == "PT_LOAD"]
        if not segments:
            raise ValueError("Dependency has no PT_LOAD segments")
        min_vaddr = min(s["p_vaddr"] for s in segments) & -PAGE
        max_vaddr = max(s["p_vaddr"] + s["p_memsz"] for s in segments)
        size = _align(max_vaddr - min_vaddr)
        uc.mem_map(base + min_vaddr, size)
        for segment in segments:
            if segment["p_filesz"]:
                start = segment["p_offset"]
                uc.mem_write(base + segment["p_vaddr"], blob[start:start + segment["p_filesz"]])

        dynamic = next((s for s in elf.iter_segments() if s["p_type"] == "PT_DYNAMIC"), None)
        if dynamic is None:
            raise ValueError("Dependency has no PT_DYNAMIC segment")
        tags = {tag.entry.d_tag: tag.entry.get("d_val", tag.entry.get("d_ptr")) for tag in dynamic.iter_tags()}
        dynamic_symbols = list(dynamic.iter_symbols())
        symbols = {
            symbol.name: (symbol["st_value"] if symbol["st_shndx"] == "SHN_ABS" else base + symbol["st_value"])
            for symbol in dynamic_symbols
            if symbol.name and symbol["st_shndx"] != "SHN_UNDEF"
        }

        def file_offset(address, length):
            for segment in segments:
                start, end = segment["p_vaddr"], segment["p_vaddr"] + segment["p_filesz"]
                if start <= address and address + length <= end:
                    return segment["p_offset"] + address - start
            raise ValueError(f"Dynamic relocation table is not file-backed: {address:#x}")

        relocations = []
        for address_tag, size_tag in (("DT_RELA", "DT_RELASZ"), ("DT_JMPREL", "DT_PLTRELSZ")):
            address, table_size = tags.get(address_tag), tags.get(size_tag, 0)
            if address is not None and table_size:
                table_offset = file_offset(address, table_size)
                relocations.extend(dict(r.entry) for r in RelocationTable(elf, table_offset, table_size, True).iter_relocations())

        for relocation in relocations:
            kind = relocation["r_info_type"]
            if kind == 0:
                continue
            if kind == 1027:
                value = base + relocation["r_addend"]
            elif kind in RELOCATION_TYPES:
                symbol = dynamic_symbols[relocation["r_info_sym"]]
                name = symbol.name
                addend = relocation["r_addend"]
                if symbol["st_shndx"] != "SHN_UNDEF":
                    symbol_value = symbol["st_value"] if symbol["st_shndx"] == "SHN_ABS" else base + symbol["st_value"]
                    value = symbol_value + addend
                elif name in known_symbols:
                    value = known_symbols[name] + addend
                elif name in symbols:
                    value = symbols[name] + addend
                elif symbol["st_info"]["bind"] == "STB_WEAK":
                    value = addend
                else:
                    matches = [address for address, existing_name in external_names.items() if existing_name == name]
                    if matches:
                        value = matches[0] + addend
                    else:
                        if not external_names:
                            raise ValueError(f"Cannot allocate external stub without an external address anchor: {name}")
                        address = max(external_names) + 16
                        external_names[address] = name
                        value = address + addend
            else:
                raise ValueError(f"Unsupported dependency relocation type {kind}")
            target = relocation["r_offset"]
            if not any(s["p_vaddr"] <= target and target + 8 <= s["p_vaddr"] + s["p_memsz"] for s in segments):
                raise ValueError(f"Dependency relocation target outside mapped span: {target:#x}")
            uc.mem_write(base + target, int(value & ((1 << 64) - 1)).to_bytes(8, "little"))

        info = {
            "path": str(source),
            "source_sha256": source_sha256,
            "base": hex(base),
            "load_bias": hex(base),
            "min_vaddr": hex(min_vaddr),
            "map_base": hex(base + min_vaddr),
            "size": size,
            "init_rva": hex(tags.get("DT_INIT", 0)),
            "init_array_rva": hex(tags.get("DT_INIT_ARRAY", 0)),
            "init_array_count": tags.get("DT_INIT_ARRAYSZ", 0) // 8,
            "init_array_completed": 0,
            "in_progress": None,
            "program_header_offset": elf["e_phoff"],
            "program_header_count": elf["e_phnum"],
        }
        return {"info": info, "symbols": symbols, "tags": tags}
