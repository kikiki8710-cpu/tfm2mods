"""두 PDB에서 지정 enum들의 VariantN -> 이름 매핑을 뽑아 diff.
usage: python variant_diff.py <pdbA> <pdbB> <enum_substr> [...]
"""
import sys, struct, re, pdbparse

LF_FIELDLIST=0x1203; LF_MEMBER=0x150d; LF_STRUCTURE=0x1505; LF_CLASS=0x1504; LF_UNION=0x1506

def numeric(buf, off):
    (v,) = struct.unpack_from('<H', buf, off)
    if v < 0x8000: return v, off+2
    off += 2
    sz = {0x8000:('<b',1),0x8001:('<h',2),0x8002:('<H',2),0x8003:('<i',4),0x8004:('<I',4),0x8009:('<q',8),0x800a:('<Q',8)}[v]
    return struct.unpack_from(sz[0], buf, off)[0], off+sz[1]

def cstr(buf, off):
    e = buf.index(b'\0', off); return buf[off:e].decode('utf-8','replace'), e+1

def load(pdb_path):
    p = pdbparse.parse(pdb_path, fast_load=True)
    raw = p.streams[2].data
    ver, hdr, tib, tie, rb = struct.unpack_from('<IIIII', raw, 0)
    recs = {}; off = hdr; ti = tib
    while ti < tie:
        (rlen,) = struct.unpack_from('<H', raw, off)
        (leaf,) = struct.unpack_from('<H', raw, off+2)
        recs[ti] = (leaf, raw[off+4:off+2+rlen]); off += 2+rlen; ti += 1
    names = {}
    for ti,(leaf,b) in recs.items():
        if leaf not in (LF_STRUCTURE, LF_CLASS): continue
        try:
            cnt, prop, field, der, vs = struct.unpack_from('<HHIII', b, 0)
            size, o = numeric(b, 16); nm, _ = cstr(b, o)
        except Exception: continue
        if prop & 0x80: continue
        names[nm] = (ti, size, field)
    return recs, names

VAR_RE = re.compile(r'^(.*)::Variant(\d+)$')

def variants(recs, names, substr):
    """enum2$<X>::VariantN 구조체의 value 필드 타입명 -> variant 이름"""
    out = {}
    for nm,(ti,size,field) in names.items():
        m = VAR_RE.match(nm)
        if not m: continue
        base, idx = m.group(1), int(m.group(2))
        if substr not in base: continue
        if field not in recs: continue
        fleaf, fb = recs[field]
        if fleaf != LF_FIELDLIST: continue
        o = 0
        while o < len(fb):
            (ml,) = struct.unpack_from('<H', fb, o)
            if ml != LF_MEMBER: break
            attr, tidx = struct.unpack_from('<HI', fb, o+2)
            moff, o2 = numeric(fb, o+8); mname, o2 = cstr(fb, o2)
            if mname == 'value' and tidx in recs:
                tl, tb = recs[tidx]
                try:
                    if tl in (LF_STRUCTURE, LF_CLASS):
                        _c,_p,_f,_d,_v = struct.unpack_from('<HHIII', tb, 0)
                        _s,_o = numeric(tb,16); vn,_ = cstr(tb,_o)
                        out.setdefault(base, {})[idx] = vn.split('::')[-1]
                except Exception: pass
            break
        o = o2
    return out

def main():
    a, b = sys.argv[1], sys.argv[2]
    wanted = sys.argv[3:]
    print('[*] loading A', flush=True); ra, na = load(a)
    print('[*] loading B', flush=True); rb, nb = load(b)
    for w in wanted:
        va, vb = variants(ra, na, w), variants(rb, nb, w)
        keys = sorted(set(va) | set(vb))
        if not keys:
            print(f"\n### {w}: NOT FOUND"); continue
        for k in keys:
            ma, mb = va.get(k, {}), vb.get(k, {})
            allv = sorted(set(ma) | set(mb))
            same = all(ma.get(i) == mb.get(i) for i in allv)
            print(f"\n### {k}   -> {'IDENTICAL' if same else '*** CHANGED ***'}")
            for i in allv:
                x, y = ma.get(i, '(없음)'), mb.get(i, '(없음)')
                mark = '   ' if x == y else ' <<'
                print(f"   {i:>2}: {x:<28} | {y:<28}{mark}")

if __name__ == '__main__':
    main()
