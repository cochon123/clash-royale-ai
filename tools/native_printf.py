"""Adapt ARM64 printf arguments to host libc without dereferencing guest pointers."""
import ctypes
import re
import struct

CONVERSION=re.compile(rb'%[-+ #0\x27]*(\*|[0-9]+)?(?:\.(\*|[0-9]*))?(hh|ll|h|l|j|z|t|L)?([diuoxXfFeEgGaAcspn%])')
LIBC=ctypes.CDLL(None)
SNPRINTF=LIBC.snprintf
SNPRINTF.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_char_p]
SNPRINTF.restype=ctypes.c_int


def vsnprintf_arm64(emu,destination,capacity,format_address,va_address,cstring):
    """Read an AAPCS64 va_list and format C-locale bytes with checked pointers.

    Integer widths, pointers, strings, doubles and %n are marshalled into host
    values. Positional formats and long-double/wide conversions stop explicitly.
    The target machine code is never executed on the host.
    """
    if capacity>0x100000:raise ValueError('printf output exceeds analysis bound')
    fmt=cstring(format_address)
    stack,gr_top,vr_top,gr_offset,vr_offset=struct.unpack('<QQQii',emu.mem_read(va_address,32))
    def next_argument(floating=False):
        nonlocal stack,gr_offset,vr_offset
        if floating and vr_offset<0:
            address=vr_top+vr_offset;vr_offset+=16
        elif not floating and gr_offset<0:
            address=gr_top+gr_offset;gr_offset+=8
        else:address=stack;stack+=8
        return struct.unpack('<Q',emu.mem_read(address,8))[0]
    arguments=[];writebacks=[];position=0
    while True:
        position=fmt.find(b'%',position)
        if position<0:break
        match=CONVERSION.match(fmt,position)
        if not match:raise ValueError(f'Unsupported printf conversion: {fmt[position:position+32]!r}')
        width,precision,length,kind=match.groups();position=match.end()
        if kind==b'%':continue
        for field in (width,precision):
            if field==b'*':arguments.append(ctypes.c_int(next_argument()&0xffffffff))
        floating=kind in b'fFeEgGaA'
        if length==b'L' or (length==b'l' and kind in (b's',b'c')):raise ValueError('Wide or long-double printf conversion needs an explicit ABI adapter')
        raw=next_argument(floating)
        if kind==b's':arguments.append(ctypes.c_char_p(cstring(raw) if raw else None))
        elif kind==b'p':arguments.append(ctypes.c_void_p(raw))
        elif floating:arguments.append(ctypes.c_double(struct.unpack('<d',struct.pack('<Q',raw))[0]))
        elif kind==b'n':
            integer=ctypes.c_int8 if length==b'hh' else ctypes.c_int16 if length==b'h' else ctypes.c_int64 if length in (b'l',b'll',b'j',b'z',b't') else ctypes.c_int32
            slot=integer();arguments.append(ctypes.pointer(slot));writebacks.append((raw,slot))
        elif kind in (b'd',b'i'):
            integer=ctypes.c_int64 if length in (b'l',b'll',b'j',b'z',b't') else ctypes.c_int32
            arguments.append(integer(raw))
        else:
            integer=ctypes.c_uint64 if length in (b'l',b'll',b'j',b'z',b't') else ctypes.c_uint32
            arguments.append(integer(raw))
    buffer=ctypes.create_string_buffer(max(capacity,1))
    result=SNPRINTF(buffer,capacity,fmt,*arguments)
    if capacity and destination:emu.mem_write(destination,buffer.raw[:min(capacity,max(result,0)+1)])
    for address,slot in writebacks:emu.mem_write(address,bytes(slot))
    return result,fmt
