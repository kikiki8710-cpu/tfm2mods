# -*- coding: utf-8 -*-
"""★노브 재배치 엔진 — orig_table.rs 의 전 사이트를 0.5.3 → 0.5.4 로 옮긴다.

**절대 하지 않는 것**: 상수값만 보고 짝짓기. (그게 "설정값이 죽었다" 오판의 원인)

하는 것 — 4단계로 좁힌다:
  ① 사이트가 속한 **0.5.3 함수**를 찾는다 (.pdata 경계)
  ② 그 함수의 **0.5.4 짝**을 찾는다 — 소스 파일명(패닉 Location = 버전무관 앵커)이
     겹치는 후보만 놓고, 명령 골격 정렬로 고른다
  ③ 짝 함수 안에서 **명령 시그니처**(즉치 바이트만 0으로 가린 실바이트)가 같고
     **원본값도 같은** 명령을 모은다
  ④ 0.5.3 쪽에서 그 사이트가 **몇 번째**였는지 세어 같은 순번을 고른다
     (개수가 다르면 확정하지 않고 **미확정으로 남긴다** — 추측으로 채우지 않는다)

시그니처를 쓰는 이유: 레지스터가 바뀌면(r14→r13 등) 바이트가 달라진다. 그걸 "같다"고
넘기면 엉뚱한 자리를 패치한다. 반대로 시그니처가 안 맞는데 값·순번이 맞는 자리는
**"레지스터 변경 후보"로 따로 보고**해서 사람이 확인한다.

산출: reloc_054.tsv  (053rva / 054rva / off / width / orig / 판정 / 근거)
"""
import io, os, re, sys, difflib, collections

sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load

# ⚠stdout 재래핑은 **import 시에 하면 안 된다.** 두 모듈이 각각 감싸면 앞 래퍼가
#   GC 될 때 바탕 buffer 를 닫아 "closed file" 로 터진다(실측 3회). __main__ 에서만.
if __name__ == '__main__':
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

D = r'C:\tfm2mods\v54'
SRC = r'C:\tfm2mods\tfm2_ai_adjust\src'
E3, E4 = load('053'), load('054')

ENT = re.compile(r'\(\s*0x([0-9a-fA-F]+)\s*,\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)')


def read_orig():
    txt = io.open(os.path.join(SRC, 'orig_table.rs'), encoding='utf-8').read()
    return [(int(a, 16), int(b), int(c), int(d)) for a, b, c, d in ENT.findall(txt)]


def srcmap(ver):
    out = []
    for ln in io.open(os.path.join(D, '%s_srcmap2.tsv' % ver), encoding='utf-8'):
        p = ln.rstrip('\n').split('\t')
        out.append((int(p[0], 16), int(p[1], 16), p[2]))
    return out


def parts(s):
    return set(x.strip() for x in s.split('|') if x.strip())


S3, S4 = srcmap('053'), srcmap('054')
SRC3 = {s: src for s, e, src in S3}
BY4 = collections.defaultdict(list)
for s, e, src in S4:
    for p in parts(src):
        BY4[p].append((s, e, src))

_ins = {}


def insns(E, s, e):
    """함수 전체를 선형 디스어셈해 캐시한다."""
    k = (id(E), s)
    if k not in _ins:
        _ins[k] = list(E.dis(s, e - s))
    return _ins[k]


def sig(ins, off, w):
    """즉치 바이트만 0으로 가린 실바이트 = '이 명령이 무슨 모양인가'."""
    b = bytearray(ins.bytes)
    if off + w > len(b):
        return None
    for i in range(off, off + w):
        b[i] = 0
    return bytes(b)


def val(ins, off, w):
    return int.from_bytes(ins.bytes[off:off + w], 'little')


_pair = {}

# ★손으로 확정한 함수 짝 — 자동 짝짓기가 못 가르거나 틀린 것만.
#   근거를 반드시 같이 적는다. 근거 없는 항목은 넣지 말 것.
FN_OVERRIDE = {
    # 쌍둥이(548B 동일 제네릭 인스턴스). 골격 100%라 자동으로는 못 가른다.
    # 근거 = 함수 안의 `cmp [r14+0x5b0], N` 값(스킬2=3 / 궁=5)이 양 판 그대로이고,
    #        호출자 순서도 보존(053 d94974<d94a8b ↔ 054 d9ee70<d9ef6f).
    #        자동 짝짓기는 둘 다 ca6700 으로 보내 ex_ult_level 을 스킬2 함수에 붙일 뻔했다.
    0xcc3340: 0xca6700,   # ex_skill2_level (값 3)
    0xcc3570: 0xca6930,   # ex_ult_level    (값 5)
    # 패닉 Location 이 없어 소스 앵커가 안 붙는 함수들.
    0xd90bd0: 0xe4f3d0,   # 호출자(entity.rs c3a3a0) 같은 순번 call 3표 + 크기 430B 동일
    0xd0cab0: 0xe76c10,   # 판단주기 스로틀. 명령열 지문으로 확인:
                          #   mov ecx,0x64 / sub / cmovae / lea+0x190 / mov eax,eax / lea rax*4+0x320
}


def pair_fn(fs, fe):
    """0.5.3 함수 → 0.5.4 짝. 소스명이 겹치는 후보만, 골격 정렬로 선택."""
    if fs in FN_OVERRIDE:
        g = E4.func_of(FN_OVERRIDE[fs])     # 끝 주소는 .pdata 에서 — 손으로 적지 않는다
        return (g[0], g[1], 1.0) if g else None
    if fs in _pair:
        return _pair[fs]
    src = SRC3.get(fs)
    best = None
    if src:
        sz = fe - fs
        cand = {}
        for p in parts(src):
            for c in BY4.get(p, []):
                cand[c[0]] = c
        cand = [c for c in cand.values() if 0.3 < (c[1] - c[0]) / sz < 3.0]
        a = [i.mnemonic for i in insns(E3, fs, fe)]
        # 싸구려 히스토그램으로 상위만 추린 뒤 정밀 정렬
        def h(x):
            return collections.Counter(x)
        ha = h(a)
        def cheap(c):
            hb = h([i.mnemonic for i in insns(E4, c[0], c[1])])
            inter = sum(min(v, hb.get(k, 0)) for k, v in ha.items())
            return -inter / max(1, max(sum(ha.values()), sum(hb.values())))
        cand.sort(key=cheap)
        for c in cand[:4]:
            b = [i.mnemonic for i in insns(E4, c[0], c[1])]
            r = difflib.SequenceMatcher(None, a, b, autojunk=False).ratio()
            if best is None or r > best[2]:
                best = (c[0], c[1], r)
    _pair[fs] = best
    return best


def main():
    rows = []
    ent = read_orig()
    print('orig_table 사이트 %d개 재배치 시도\n' % len(ent))

    # 같은 함수의 사이트를 묶어 순번을 센다
    byfn = collections.defaultdict(list)
    noffn = []
    for rva, off, w, orig in ent:
        f = E3.func_of(rva)
        if not f:
            noffn.append((rva, off, w, orig))
            continue
        byfn[f].append((rva, off, w, orig))

    for (fs, fe), sites in sorted(byfn.items()):
        i3 = {i.address - 0x140000000: i for i in insns(E3, fs, fe)}
        pr = pair_fn(fs, fe)
        if not pr:
            for rva, off, w, orig in sites:
                rows.append((rva, 0, off, w, orig, '짝없음', 'src=%s' % (SRC3.get(fs, '?'))))
            continue
        bs, be, ratio = pr
        i4 = insns(E4, bs, be)

        # 0.5.3 함수 안의 (시그니처, 값) → 주소 목록
        g3 = collections.defaultdict(list)
        for a, ins in sorted(i3.items()):
            for _, off, w, orig in sites:
                s = sig(ins, off, w)
                if s is not None and val(ins, off, w) == orig:
                    g3[(s, orig, off, w)].append(a)
        g4 = collections.defaultdict(list)
        for ins in i4:
            for _, off, w, orig in sites:
                s = sig(ins, off, w)
                if s is not None and val(ins, off, w) == orig:
                    g4[(s, orig, off, w)].append(ins.address - 0x140000000)

        for rva, off, w, orig in sites:
            ins = i3.get(rva)
            if ins is None:
                rows.append((rva, 0, off, w, orig, '명령경계불일치', '선형디스어셈에서 %06x가 명령 시작이 아님' % rva))
                continue
            if val(ins, off, w) != orig:
                rows.append((rva, 0, off, w, orig, '053원본불일치', '실제 %d ≠ 표 %d' % (val(ins, off, w), orig)))
                continue
            key = (sig(ins, off, w), orig, off, w)
            a3, a4 = g3[key], g4[key]
            if not a4:
                rows.append((rva, 0, off, w, orig, '시그니처소멸',
                             '054 %06x-%06x 에 같은 모양 0개 (골격 %.0f%%)' % (bs, be, ratio * 100)))
            elif len(a3) == len(a4):
                k = a3.index(rva)
                rows.append((rva, a4[k], off, w, orig, '확정',
                             '%d/%d번째, 시그니처 동일, 골격 %.0f%%' % (k + 1, len(a3), ratio * 100)))
            else:
                k = a3.index(rva)
                rows.append((rva, a4[k] if k < len(a4) else 0, off, w, orig, '개수불일치',
                             '053 %d곳 → 054 %d곳 (%d번째)' % (len(a3), len(a4), k + 1)))

    for rva, off, w, orig in noffn:
        rows.append((rva, 0, off, w, orig, '함수미상', '.pdata 에 없음'))

    rows.sort()
    with io.open(os.path.join(D, 'reloc_054.tsv'), 'w', encoding='utf-8', newline='') as fo:
        fo.write('rva053\trva054\timm_off\twidth\torig\t판정\t근거\n')
        for r in rows:
            fo.write('%06x\t%s\t%d\t%d\t%d\t%s\t%s\n'
                     % (r[0], ('%06x' % r[1]) if r[1] else '-', r[2], r[3], r[4], r[5], r[6]))

    c = collections.Counter(r[5] for r in rows)
    print('판정')
    for k in ('확정', '개수불일치', '시그니처소멸', '짝없음', '명령경계불일치', '053원본불일치', '함수미상'):
        if c[k]:
            print('  %-12s %4d' % (k, c[k]))
    print('\n→ reloc_054.tsv')


if __name__ == '__main__':
    main()
