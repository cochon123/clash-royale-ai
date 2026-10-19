#!/usr/bin/env python3
"""Recover the exact protected libg metadata binding from the supplied base APK.

This only parses the original asset envelope and protobuf wire records. It does
not decrypt the binding, validate the PBI signature or invent native semantics.
Run the recovered companion decoder separately with tools/native_unpack.py.
"""
import argparse
import hashlib
import json
from pathlib import Path
import struct
import zipfile

ROOT = Path(__file__).resolve().parents[1]
BASE_SHA256 = '0d0dfed74fdb755b71f5424e70af9234ed25471c029d09863b657fed9057b13e'
ASSET_SHA256 = '834217feba3588c1f4d28e8dcf2a309c9baa1903eb5c08f535a628b50751d60b'
LOOKUP_ID = 0x7e33eb5e


def digest(data):
    return hashlib.sha256(data).hexdigest()


def varint(data, offset, end):
    value = 0
    for shift in range(0, 70, 7):
        if offset >= end:
            raise ValueError('Truncated protobuf varint')
        byte = data[offset]
        offset += 1
        if shift == 63 and byte > 1:
            raise ValueError('Protobuf varint overflows uint64')
        value |= (byte & 127) << shift
        if byte < 128:
            return value, offset
    raise ValueError('Oversized protobuf varint')


def fields(data, start=0, end=None):
    end = len(data) if end is None else end
    offset = start
    result = []
    while offset < end:
        first = offset
        tag, offset = varint(data, offset, end)
        field, wire = tag >> 3, tag & 7
        if not 0 < field < 1 << 29:
            raise ValueError('Invalid protobuf field number')
        row = {'field': field, 'wire': wire, 'start': first}
        if wire == 0:
            row['value'], offset = varint(data, offset, end)
        elif wire in (1, 5):
            count = 8 if wire == 1 else 4
            row.update(data=offset, size=count)
            offset += count
        elif wire == 2:
            count, offset = varint(data, offset, end)
            row.update(data=offset, size=count)
            offset += count
        else:
            raise ValueError(f'Unsupported protobuf wire type {wire}')
        if offset > end:
            raise ValueError('Protobuf field exceeds its enclosing record')
        row['end'] = offset
        result.append(row)
    return result


def unique(rows, number, wire):
    matches = [row for row in rows if row['field'] == number and row['wire'] == wire]
    if len(matches) != 1:
        raise ValueError(f'Expected one protobuf field {number} with wire {wire}')
    return matches[0]


def recover(apk, output):
    package = apk.read_bytes()
    if digest(package) != BASE_SHA256:
        raise ValueError('This recovery recipe requires the verified supplied base APK')
    member = 'assets/anjeamelnchn.dat'
    with zipfile.ZipFile(apk) as archive:
        asset = archive.read(member)
    if digest(asset) != ASSET_SHA256 or asset[:4] != b'lbts':
        raise ValueError('Protected asset identity mismatch')
    output.mkdir(parents=True, exist_ok=True)
    (output/'protection-config.dat').write_bytes(asset)
    blocks = {}
    offsets = {}
    position = 4
    while position < len(asset):
        if position + 8 > len(asset):
            raise ValueError('Truncated protected asset envelope')
        tag, length = struct.unpack_from('<II', asset, position)
        position += 8
        if tag in blocks or position + length > len(asset):
            raise ValueError('Duplicate block tag or invalid envelope size')
        blocks[tag] = asset[position:position+length]
        offsets[tag] = position
        (output/f'protection-config-block-{tag:02x}.bin').write_bytes(blocks[tag])
        position += length
    if set(blocks) != {0x10, 0x20, 0x30}:
        raise ValueError('Unexpected protected asset block inventory')
    signed = blocks[0x30]
    unsigned = unique(fields(signed), 1, 2)
    records = []
    selected = []
    for record in fields(signed, unsigned['data'], unsigned['end']):
        if record['field'] != 10 or record['wire'] != 2:
            continue
        contents = fields(signed, record['data'], record['end'])
        key = unique(contents, 1, 0)
        records.append(key['value'])
        if key['value'] == LOOKUP_ID:
            selected.append((record, contents))
    if len(selected) != 1:
        raise ValueError('Requested native lookup ID is absent or ambiguous')
    record, contents = selected[0]
    binding = unique(contents, 2, 2)
    cipher = unique(fields(signed, binding['data'], binding['end']), 1, 2)
    artifacts = {}
    for name, first, end in [('record', record['data'], record['end']),
                             ('binding', binding['data'], binding['end']),
                             ('ciphertext', cipher['data'], cipher['end'])]:
        data = signed[first:end]
        path = output/f'protection-config-requested-{name}.bin'
        path.write_bytes(data)
        artifacts[name] = {'path': str(path), 'size': len(data), 'sha256': digest(data),
                           'block30_offset': first, 'original_asset_offset': offsets[0x30]+first}
    report = {'source_apk': str(apk), 'source_apk_sha256': digest(package),
              'source_member': member, 'asset_sha256': digest(asset),
              'envelope': [{'tag': hex(tag), 'size': len(data), 'asset_offset': offsets[tag], 'sha256': digest(data)} for tag, data in blocks.items()],
              'lookup_id': hex(LOOKUP_ID), 'binding_records': len(records), 'artifacts': artifacts,
              'limitations': ['Protobuf wire extraction only; the signed envelope signature is not verified here.',
                              'The ciphertext still requires the original native decoder.']}
    (output/'metadata-recovery.json').write_text(json.dumps(report, indent=2)+'\n')
    print(json.dumps(report, indent=2))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--apk', type=Path, default=ROOT/'analysis/apks/com.supercell.clashroyale.apk')
    parser.add_argument('--output', type=Path, default=ROOT/'analysis/native-entry')
    arguments = parser.parse_args()
    recover(arguments.apk, arguments.output)
