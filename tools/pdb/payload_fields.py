"""GameEvent variant payload struct(__0)의 필드명을 재귀로 뽑아 두 PDB 비교.
usage: python payload_fields.py <pdbA> <pdbB>
"""
import sys, struct, re, pdbparse

LF_FIELDLIST=0x1203; LF_MEMBER=0x150d; LF_STRUCTURE=0x1505; LF_CLASS=0x1504; LF_UNION=0x1506

def numeric(buf, off):
    (v,) = struct.unpack_from('<H', buf, off)
    if v < 0x8000: return v, off+2
    off += 2
    m = {0x8000:('<b',1),0x8001:('<h',2),0x8002:('<H',2),0x8003:('<i',4),0x8004:('<I',4),0x8009:('<q',8),0x800a:('<Q',8)}[v]
    return struct.unpack_from(m[0], buf, off)[0], off+m[1]

def cstr(buf, off):
    e = buf.index(b'\0', off); return buf[off:e].decode('utf-8','replace'), e+1

def load(p):
    pd = pdbparse.parse(p, fast_load=True)
    raw = pd.streams[2].data
    ver, hdr, tib, tie, rb = struct.unpack_from('<IIIII', raw, 0)
    recs={}; off=hdr; ti=tib
    while ti < tie:
        (rlen,)=struct.unpack_from('<H',raw,off); (leaf,)=struct.unpack_from('<H',raw,off+2)
        recs[ti]=(leaf, raw[off+4:off+2+rlen]); off+=2+rlen; ti+=1
    return recs

_DEFS = {}   # pdb-local: name -> (nm, size, field) for non-fwdref defs

def _index_defs(recs):
    d = {}
    for ti in recs:
        leaf, b = recs[ti]
        if leaf not in (LF_STRUCTURE, LF_CLASS): continue
        try:
            cnt, prop, field, der, vs = struct.unpack_from('<HHIII', b, 0)
            size, o = numeric(b, 16); nm, _ = cstr(b, o)
        except Exception: continue
        if prop & 0x80: continue          # fwdref 는 스킵
        d[nm] = (nm, size, field)
    return d

def struct_info(recs, ti):
    if ti not in recs: return None
    leaf, b = recs[ti]
    if leaf not in (LF_STRUCTURE, LF_CLASS): return None
    try:
        cnt, prop, field, der, vs = struct.unpack_from('<HHIII', b, 0)
        size, o = numeric(b, 16); nm, _ = cstr(b, o)
    except Exception: return None
    if prop & 0x80:                       # fwdref → 이름으로 실제 정의 조회
        return _DEFS.get(nm)
    return nm, size, field

def members(recs, field_ti):
    out=[]
    if field_ti not in recs: return out
    fl, fb = recs[field_ti]
    if fl != LF_FIELDLIST: return out
    o=0
    while o < len(fb):
        (ml,)=struct.unpack_from('<H', fb, o)
        if ml != LF_MEMBER: break
        attr, idx = struct.unpack_from('<HI', fb, o+2)
        moff, o = numeric(fb, o+8); mname, o = cstr(fb, o)
        out.append((mname, idx))
        while o < len(fb) and fb[o] >= 0xf0: o += 1
    return out

def collect(pdb):
    global _DEFS
    recs = load(pdb)
    _DEFS = _index_defs(recs)
    # GameEvent variant struct 찾기
    res = {}
    for ti in recs:
        si = struct_info(recs, ti)
        if not si: continue
        nm, size, field = si
        m = re.match(r'^enum2\$<game_core::simulation::game::frame::GameEvent>::(\w+)$', nm)
        if not m: continue
        vname = m.group(1)
        if vname.startswith('Variant') or vname == 'GameEvent': continue
        for mname, idx in members(recs, field):
            if mname != '__0': continue
            psi = struct_info(recs, idx)
            if not psi:
                res[vname] = ['(non-struct payload)']
                continue
            pnm, psz, pfield = psi
            fields = [f'{f}' for f,_ in members(recs, pfield)]
            res[vname] = [f'{pnm.split("::")[-1]}({psz}B)'] + sorted(fields)
    return res

a, b = collect(sys.argv[1]), collect(sys.argv[2])
keys = sorted(set(a) | set(b))
bad = 0
for k in keys:
    x, y = a.get(k), b.get(k)
    if x == y:
        print(f"[SAME] {k:<24} {', '.join(x or [])}")
    else:
        bad += 1
        print(f"[DIFF] {k}")
        print(f"   A: {x}")
        print(f"   B: {y}")
print(f"\n총 {len(keys)} variant, 불일치 {bad}")
