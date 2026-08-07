# -*- coding: utf-8 -*-
"""문자열/xref/상수 조회 보조 도구"""
import io, os, re, sys, struct, bisect, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE

_cache = {}
def E(v):
    if v not in _cache: _cache[v] = load(v)
    return _cache[v]

def sec(e, name):
    for s in e.sections:
        if s[0] == name: return s
    return None

def rdata_strings(ver, minlen=3):
    """(rva, bytes) 전 .rdata ASCII 런"""
    e = E(ver)
    _, va, vsz, ra, rsz = sec(e, '.rdata')
    blob = e.raw[ra:ra+rsz]
    out = []
    for m in re.finditer(rb'[\x20-\x7e]{%d,}' % minlen, blob):
        out.append((va + m.start(), m.group()))
    return out

def find_str(ver, pat, minlen=3):
    p = pat.encode() if isinstance(pat, str) else pat
    return [(r, s) for r, s in rdata_strings(ver, minlen) if p in s]

def lea_index(ver):
    """{target_rva: [site_rva,...]} for lea reg,[rip+d]"""
    key = ('lea', ver)
    if key in _cache: return _cache[key]
    e = E(ver)
    _, va, vsz, ra, rsz = sec(e, '.text')
    body = e.raw[ra:ra+rsz]
    idx = collections.defaultdict(list)
    for m in re.finditer(rb'[\x48\x4c\x44\x00]?[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]', body):
        o = m.end()-3
        if o+7 > len(body): continue
        disp = int.from_bytes(body[o+3:o+7], 'little', signed=True)
        idx[va+o+7+disp].append(va+o)
    _cache[key] = idx
    return idx

def xref_lea(ver, target):
    return lea_index(ver).get(target, [])

def data_refs(ver, target):
    """.rdata/.data 안에서 이 rva를 절대주소(u64)로 담고 있는 위치"""
    e = E(ver); out=[]
    want = struct.pack('<Q', BASE+target)
    for nm in ('.rdata','.data'):
        s = sec(e, nm)
        if not s: continue
        _, va, vsz, ra, rsz = s
        blob = e.raw[ra:ra+rsz]
        i = 0
        while True:
            i = blob.find(want, i)
            if i < 0: break
            out.append((nm, va+i))
            i += 1
    return out

def calls_to(ver, target):
    """e8 rel32 call / e9 jmp -> target"""
    e = E(ver)
    _, va, vsz, ra, rsz = sec(e, '.text')
    body = e.raw[ra:ra+rsz]
    out=[]
    for m in re.finditer(rb'[\xe8\xe9]', body):
        o = m.start()
        if o+5 > len(body): continue
        d = int.from_bytes(body[o+1:o+5],'little',signed=True)
        if va+o+5+d == target: out.append((va+o, body[o]))
    return out

def fsrc(ver):
    rows=[]
    for ln in io.open(os.path.join(r'C:\tfm2mods\v54','%s_srcmap.tsv'%ver), encoding='utf-8'):
        s,en,src,lines = ln.rstrip('\n').split('\t')
        rows.append((int(s,16),int(en,16),src,lines))
    return rows

import bisect as _bi
def _wcache(ver):
    k=('w',ver)
    if k not in _cache:
        e=E(ver); fl=e.funcs(); st=[f[0] for f in fl]
        sm={s:(src,l) for s,en,src,l in fsrc(ver)}
        _cache[k]=(fl,st,sm)
    return _cache[k]
def whose(ver, rva):
    fl,st,sm=_wcache(ver)
    i=_bi.bisect_right(st,rva)-1
    if i<0 or not (fl[i][0]<=rva<fl[i][1]): return None
    src,l=sm.get(fl[i][0],('?','?'))
    return (fl[i],src,l)

def dis(ver, rva, n):
    for i in E(ver).dis(rva, n):
        print('%06x  %-22s %s %s' % (i.address-BASE, i.bytes.hex(), i.mnemonic, i.op_str))
