# -*- coding: utf-8 -*-
"""점프테이블 전수 스캐너 (capstone, Ghidra 불필요).

LLVM/rustc x64 점프테이블 코다:
    lea  Rb, [rip+d32]                 ; JT 베이스
    movsxd Rd, dword ptr [Rb + Ri*4]   ; 엔트리 = JT 기준 rel32
    add  Rd, Rb
    jmp  Rd
바이트 스캔으로 movsxd 를 먼저 찾고, 뒤(add/jmp)·앞(lea) 을 검증한다.

  python jtscan.py all 054            # 전수 스캔 → tsv
  python jtscan.py one 054 <rva>      # 특정 디스패처 상세(엔트리·타깃)
"""
import io, os, struct, sys, collections
if __name__ == '__main__':
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE

REGN = ['rax','rcx','rdx','rbx','rsp','rbp','rsi','rdi',
        'r8','r9','r10','r11','r12','r13','r14','r15']


def scan(e):
    """[(jmp_rva, jt_rva, base_reg, idx_reg, lea_rva)] 전수."""
    txt = [s for s in e.sections if s[0] == '.text'][0]
    _, va, vsz, ra, rsz = txt
    data = e.raw[ra:ra + rsz]
    out = []
    n = len(data)
    i = 0
    while True:
        i = data.find(b'\x63', i + 1)
        if i < 1 or i + 12 > n:
            break
        rex = data[i - 1]
        if not (0x48 <= rex <= 0x4f):
            continue
        modrm = data[i + 1]
        if (modrm & 0xC7) != 0x04:       # mod=00, rm=100(SIB)
            continue
        sib = data[i + 2]
        if (sib >> 6) != 2:              # scale must be *4
            continue
        idx = ((sib >> 3) & 7) | ((rex & 2) << 2)
        bas = (sib & 7) | ((rex & 1) << 3)
        if (sib & 7) == 5:               # base=101 → disp32, JT 형태 아님
            continue
        dst = ((modrm >> 3) & 7) | ((rex & 4) << 1)
        p = i + 3                        # movsxd 길이 = 4 (rex+63+modrm+sib)
        # add dst, bas  :  REX 01 /r  (modrm mod=11)
        if not (0x48 <= data[p] <= 0x4f and data[p + 1] == 0x01):
            continue
        m2 = data[p + 2]
        if (m2 >> 6) != 3:
            continue
        r2src = ((m2 >> 3) & 7) | ((data[p] & 4) << 1)
        r2dst = (m2 & 7) | ((data[p] & 1) << 3)
        if not (r2dst == dst and r2src == bas):
            continue
        p += 3
        # jmp dst : [41] FF /4
        q = p
        rex2 = 0
        if 0x40 <= data[q] <= 0x4f:
            rex2 = data[q]; q += 1
        if data[q] != 0xFF:
            continue
        m3 = data[q + 1]
        if (m3 >> 6) != 3 or ((m3 >> 3) & 7) != 4:
            continue
        jr = (m3 & 7) | ((rex2 & 1) << 3)
        if jr != dst:
            continue
        # 앞으로 최대 80B 안에서 lea bas,[rip+d32] 찾기
        lea_rva = jt = None
        want_m = 0x05 | ((bas & 7) << 3)
        want_rexbit = 4 if bas >= 8 else 0
        for k in range(i - 8, max(0, i - 90), -1):
            if data[k + 1] == 0x8D and 0x48 <= data[k] <= 0x4f \
               and data[k + 2] == want_m and ((data[k] & 4) == want_rexbit):
                d = struct.unpack_from('<i', data, k + 3)[0]
                lea_rva = va + k
                jt = va + k + 7 + d
                break
        if jt is None:
            continue
        out.append((va + q - (1 if rex2 else 0), jt, REGN[bas], REGN[idx], lea_rva, va + i - 1))
    return out


_FS = {}


def fast_func_of(e, rva):
    import bisect
    key = id(e)
    if key not in _FS:
        fs = e.funcs()
        _FS[key] = (fs, [x[0] for x in fs])
    fs, starts = _FS[key]
    i = bisect.bisect_right(starts, rva) - 1
    if i >= 0 and fs[i][0] <= rva < fs[i][1]:
        return fs[i]
    return None


def entries(e, jt, fn=None, maxn=64):
    """JT 엔트리(rel32→RVA) 나열. fn=(s,en) 주면 그 범위 밖에서 중단."""
    out = []
    for k in range(maxn):
        v = e.u32(jt + 4 * k)
        if v is None:
            break
        d = struct.unpack('<i', struct.pack('<I', v))[0]
        t = jt + d
        if t < 0x1000 or t >= 0x30a7000:
            break
        if fn and not (fn[0] <= t < fn[1]):
            break
        out.append(t)
    return out


def srcmap(ver):
    rows = []
    for ln in io.open(os.path.join(r'C:\tfm2mods\v54', '%s_srcmap2.tsv' % ver), encoding='utf-8'):
        s, en, src, lines = ln.rstrip('\n').split('\t')
        rows.append((int(s, 16), int(en, 16), src, lines))
    return rows


_SMK = {}


def src_of(sm, rva):
    import bisect
    key = id(sm)
    if key not in _SMK:
        _SMK[key] = [r[0] for r in sm]
    ks = _SMK[key]
    i = bisect.bisect_right(ks, rva) - 1
    if i >= 0 and sm[i][0] <= rva < sm[i][1]:
        return sm[i][2], sm[i][3]
    return '', ''


if __name__ == '__main__':
    a = sys.argv[1:]
    ver = a[1]
    e = load(ver)
    fns = e.funcs()
    sm = srcmap(ver)
    if a[0] == 'all':
        res = scan(e)
        print('점프테이블 사이트 %d개' % len(res))
        with io.open(r'C:\tfm2mods\v54\%s_jt.tsv' % ver, 'w', encoding='utf-8') as f:
            for jmp, jt, br, ir, lea, mv in res:
                fn = fast_func_of(e, jmp)
                ent = entries(e, jt, fn)
                src, lines = src_of(sm, fn[0]) if fn else ('', '')
                f.write('%06x\t%06x\t%06x\t%s\t%s\t%d\t%s\t%s\n' % (
                    jmp, jt, fn[0] if fn else 0, br, ir, len(ent), src, lines))
    elif a[0] == 'one':
        rva = int(a[2], 16)
        fn = fast_func_of(e, rva)
        print('함수 %06x-%06x  소스=%s' % (fn[0], fn[1], src_of(sm, fn[0])[0]))
        for jmp, jt, br, ir, lea, mv in scan(e):
            if fn[0] <= jmp < fn[1]:
                ent = entries(e, jt, fn)
                print('  jmp %06x  JT %06x  base=%s idx=%s  엔트리 %d' % (jmp, jt, br, ir, len(ent)))
                seen = {}
                for k, t in enumerate(ent):
                    seen.setdefault(t, k)
                    print('    [%2d] -> %06x %s' % (k, t, '(=arm%d 중복)' % seen[t] if seen[t] != k else ''))
