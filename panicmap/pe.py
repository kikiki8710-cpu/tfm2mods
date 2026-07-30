import struct, sys

EXE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"

def load():
    data = open(EXE,'rb').read()
    e_lfanew = struct.unpack_from('<I', data, 0x3c)[0]
    assert data[e_lfanew:e_lfanew+4] == b'PE\0\0'
    fh = e_lfanew+4
    nsec = struct.unpack_from('<H', data, fh+2)[0]
    sizeopt = struct.unpack_from('<H', data, fh+16)[0]
    oh = fh+20
    magic = struct.unpack_from('<H', data, oh)[0]
    imagebase = struct.unpack_from('<Q', data, oh+24)[0]
    sects=[]
    st = oh+sizeopt
    for i in range(nsec):
        o = st+i*40
        name = data[o:o+8].rstrip(b'\0').decode('latin1')
        vsize, vaddr, rawsize, rawptr = struct.unpack_from('<IIII', data, o+8)
        chars = struct.unpack_from('<I', data, o+36)[0]
        sects.append(dict(name=name,vsize=vsize,rva=vaddr,rawsize=rawsize,rawptr=rawptr,chars=chars))
    return data, imagebase, sects

def rva2off(sects, rva):
    for s in sects:
        if s['rva'] <= rva < s['rva']+max(s['vsize'],s['rawsize']):
            d = rva - s['rva']
            if d < s['rawsize']:
                return s['rawptr']+d
    return None

def off2rva(sects, off):
    for s in sects:
        if s['rawptr'] <= off < s['rawptr']+s['rawsize']:
            return s['rva'] + (off - s['rawptr'])
    return None

if __name__=='__main__':
    data, ib, sects = load()
    print('imagebase 0x%x  len %d' % (ib, len(data)))
    for s in sects:
        print('%-10s rva=0x%08x vsize=0x%08x raw=0x%08x rawsize=0x%08x chars=0x%08x' % (s['name'],s['rva'],s['vsize'],s['rawptr'],s['rawsize'],s['chars']))
