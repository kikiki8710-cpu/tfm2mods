"""rustc/lld PDB의 TPI 스트림을 직접 파싱해 구조체 필드 오프셋 추출.
pdbparse는 MSF 컨테이너에서 raw stream 을 얻는 용도로만 사용(타입파서는 깨짐).
usage: python tpi_dump.py <pdb> <substr> [<substr> ...]
"""
import sys, struct, pdbparse

LF_MODIFIER=0x1001; LF_POINTER=0x1002
LF_FIELDLIST=0x1203
LF_MEMBER=0x150d; LF_STMEMBER=0x150e; LF_NESTTYPE=0x1510
LF_ENUMERATE=0x1502; LF_INDEX=0x1404
LF_STRUCTURE=0x1505; LF_CLASS=0x1504; LF_UNION=0x1506; LF_ENUM=0x1507
LF_ARRAY=0x1503

def numeric(buf, off):
    (v,) = struct.unpack_from('<H', buf, off)
    if v < 0x8000:
        return v, off+2
    off += 2
    if v == 0x8000: return struct.unpack_from('<b', buf, off)[0], off+1
    if v == 0x8001: return struct.unpack_from('<h', buf, off)[0], off+2
    if v == 0x8002: return struct.unpack_from('<H', buf, off)[0], off+2
    if v == 0x8003: return struct.unpack_from('<i', buf, off)[0], off+4
    if v == 0x8004: return struct.unpack_from('<I', buf, off)[0], off+4
    if v == 0x8009: return struct.unpack_from('<q', buf, off)[0], off+8
    if v == 0x800a: return struct.unpack_from('<Q', buf, off)[0], off+8
    raise ValueError(f'numeric leaf {v:#x}')

def cstr(buf, off):
    e = buf.index(b'\0', off)
    return buf[off:e].decode('utf-8', 'replace'), e+1

def main():
    pdb_path, wanted = sys.argv[1], sys.argv[2:]
    p = pdbparse.parse(pdb_path, fast_load=True)
    raw = p.streams[2].data
    ver, hdr_size, ti_begin, ti_end, rec_bytes = struct.unpack_from('<IIIII', raw, 0)
    print(f"[*] TPI ver={ver} ti=[{ti_begin:#x},{ti_end:#x}) recbytes={rec_bytes}", flush=True)

    records = {}          # ti -> (leaf, body_bytes)
    off = hdr_size
    ti = ti_begin
    while ti < ti_end:
        (rlen,) = struct.unpack_from('<H', raw, off)
        (leaf,) = struct.unpack_from('<H', raw, off+2)
        records[ti] = (leaf, raw[off+4: off+2+rlen])
        off += 2 + rlen
        ti += 1
    print(f"[*] parsed {len(records)} type records", flush=True)

    # 1st pass: struct name -> ti (skip fwdrefs)
    named = []
    for ti, (leaf, b) in records.items():
        if leaf not in (LF_STRUCTURE, LF_CLASS, LF_UNION):
            continue
        try:
            if leaf == LF_UNION:
                cnt, prop, field = struct.unpack_from('<HHI', b, 0)
                size, o = numeric(b, 8)
            else:
                cnt, prop, field, derived, vshape = struct.unpack_from('<HHIII', b, 0)
                size, o = numeric(b, 16)
            name, _ = cstr(b, o)
        except Exception:
            continue
        if prop & 0x80:   # fwdref
            continue
        named.append((name, ti, size, field, leaf))

    # ti -> (name, size, field) 역맵 (타입 인덱스 직접 조회용)
    by_ti = {ti: (nm, size, field) for nm, ti, size, field, leaf in named}

    for w in wanted:
        if w.startswith('#'):          # 타입 인덱스 직접 조회: #0xed1c
            t = int(w[1:], 16)
            if t not in by_ti:
                print(f"\n### ti={w[1:]}: NOT a named struct"); continue
            nm, size, field = by_ti[t]
            hits = [(nm, t, size, field, LF_STRUCTURE)]
        elif w.startswith('='):        # 정확 일치(끝부분) 모드
            t = w[1:]
            hits = [x for x in named if x[0] == t or x[0].endswith('::' + t)]
        else:
            hits = [x for x in named if w in x[0]]
        if not hits:
            print(f"\n### {w}: NOT FOUND"); continue
        # 가장 필드 많은/가장 큰 것을 대표로 (중복 정의 방지)
        hits.sort(key=lambda x: -x[2])
        for name, ti, size, field, leaf in hits[:24]:
            print(f"\n### {name}  ti={ti:#x} size={size} (0x{size:x})")
            if field not in records:
                print("   (no fieldlist)"); continue
            fleaf, fb = records[field]
            if fleaf != LF_FIELDLIST:
                print(f"   (field ti {field:#x} leaf {fleaf:#x})"); continue
            o = 0
            while o < len(fb):
                (mleaf,) = struct.unpack_from('<H', fb, o)
                if mleaf == LF_MEMBER:
                    attr, idx = struct.unpack_from('<HI', fb, o+2)
                    moff, o2 = numeric(fb, o+8)
                    mname, o2 = cstr(fb, o2)
                    tn = ''
                    if idx in records:
                        tl, tb = records[idx]
                        if tl in (LF_STRUCTURE, LF_CLASS, LF_UNION, LF_ENUM):
                            try:
                                if tl == LF_ENUM:
                                    _c,_p,_u,_f = struct.unpack_from('<HHII', tb, 0); tn,_ = cstr(tb, 12)
                                elif tl == LF_UNION:
                                    _c,_p,_f = struct.unpack_from('<HHI', tb, 0); _s,_o = numeric(tb,8); tn,_ = cstr(tb,_o)
                                else:
                                    _c,_p,_f,_d,_v = struct.unpack_from('<HHIII', tb, 0); _s,_o = numeric(tb,16); tn,_ = cstr(tb,_o)
                            except Exception: tn = f'ti{idx:#x}'
                        else:
                            tn = f'ti{idx:#x}'
                    else:
                        tn = f'prim{idx:#x}'
                    print(f"   +0x{moff:<5x} ({moff:>6})  {mname:<34} {tn[:70]}")
                    o = o2
                elif mleaf == LF_ENUMERATE:
                    attr, = struct.unpack_from('<H', fb, o+2)
                    val, o2 = numeric(fb, o+4); nm, o2 = cstr(fb, o2)
                    print(f"   ={val:<6}  {nm}")
                    o = o2
                elif mleaf in (LF_STMEMBER, LF_NESTTYPE):
                    # skip: attr/index + name
                    if mleaf == LF_NESTTYPE:
                        _, idx = struct.unpack_from('<HI', fb, o); nm, o = cstr(fb, o+6)
                    else:
                        idx, attr = struct.unpack_from('<IH', fb, o+2); nm, o = cstr(fb, o+8)
                else:
                    break
                while o < len(fb) and fb[o] >= 0xf0:   # padding
                    o += 1

if __name__ == '__main__':
    main()
