# -*- coding: utf-8 -*-
"""정적 테이블 8개(262 사이트) — **0.5.6 앵커 스냅샷(git 55cc7fc)** 에서 0.5.8 로 통째 마이그.

왜 현행이 아니라 git 스냅샷에서 출발하나:
  · 현행 HEAD 의 테이블은 0.5.8 적중 106/262(=156 죽음). 중간 리비전(6d780d9·c09385c)의
    **부분 마이그가 오히려 섞어 놓은 상태**라 앵커로 못 쓴다.
  · `55cc7fc` 스냅샷은 **262/262 전부 0.5.6 에서 prefix+즉치 정확 일치** = 완전한 앵커.
    (3ca527b=0.5.5 262/262, 5e653c3=0.5.4 262/262 로도 확인 — 이 계열은 버전마다 통째 이동했다.)

절차: 0.5.6 owner → 유사도 여유로 0.5.8 owner 확정 → 명령 정렬로 사이트 사상
      → **0.5.8 바이트에서 prefix + ORIG 재검증** → 통과분만 채택.
"""
import io
import os
import re
import sys
import struct
import pickle
import difflib
import subprocess
from collections import defaultdict, Counter

sys.stdout.reconfigure(encoding='utf-8', errors='replace')
K = r'C:\tfm2mods\MIG\knob'
SP = os.path.dirname(os.path.abspath(__file__))
os.chdir(K)
exec(open(os.path.join(K, 'align.py'), encoding='utf-8').read())

i6 = Img(r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.6\TeamfightManager2.exe"); i6.prep()
i8 = Img(r"C:\Program Files (x86)\Steam\steamapps\common"
         r"\Teamfight Manager2\TeamfightManager2.exe"); i8.prep()

WID = {'PE_CAP': (4, 150), 'PE_STG': (4, 180), 'TH_LEA': (4, 32000), 'TH_CAP': (4, 150),
       'PATH_STEP640': (4, 640), 'PATH_STEP896': (4, 896),
       'PATH_RISK1281': (4, 1281), 'PATH_HEUR': (1, 7)}
ENT = re.compile(r'\(\s*(0x[0-9a-fA-F]+)\s*,\s*&\[([^\]]*)\]\s*,\s*(\d+)\s*\)')
txt = subprocess.run(['git', '-C', r'C:\tfm2mods', 'show',
                      '55cc7fc:tfm2_ai_adjust/src/detour.rs'],
                     capture_output=True, text=True, encoding='utf-8', errors='replace').stdout
TB = {}
for m in re.finditer(r'static\s+(\w+)\s*:\s*\[\(usize,\s*&\[u8\],\s*usize\);\s*(\d+)\]\s*=\s*\[(.*?)\n\s*\];',
                     txt, re.S):
    if m.group(1) in WID:
        TB[m.group(1)] = [(int(a, 16), [int(x, 16) for x in re.findall(r'0x[0-9a-fA-F]+', p)], int(o))
                          for a, p, o in ENT.findall(m.group(3))]
print("앵커 스냅샷 테이블 %d개 / 사이트 %d개" % (len(TB), sum(len(v) for v in TB.values())))


def chk(img, rva, pre, off, w, orig):
    b = img.code(rva, off + w + 4)
    if not b or len(b) < off + w or list(b[:len(pre)]) != pre:
        return False
    cur = b[off] if w == 1 else struct.unpack('<I', b[off:off + 4])[0]
    return cur == orig


bad = [(nm, a) for nm, v in TB.items() for a, p, o in v
       if not chk(i6, a, p, o, WID[nm][0], WID[nm][1])]
print("0.5.6 앵커 자기검증 실패 %d건 %s\n" % (len(bad), bad[:4]))

# owner 별 묶기
byown = defaultdict(list)
for nm, v in TB.items():
    w, orig = WID[nm]
    for a, pre, off in v:
        fr = i6.frange(a)
        byown[fr[0] if fr else 0].append(dict(tbl=nm, rva=a, pre=pre, off=off, w=w, orig=orig))
print("0.5.6 owner %d개" % len(byown))

TC = {}
def tk(img, fn, tag):
    k = (tag, fn)
    if k not in TC:
        ins = stream(img, fn)
        TC[k] = (ins, toks(ins)) if ins else (None, None)
    return TC[k]

# 0.5.8 후보: .pdata 전 함수 중 크기가 ±40% 인 것만 1차 필터 → 상위 유사도
fns8 = sorted({b for b, e, u in i8.rawfuncs()})
sz8 = {}
for f in fns8:
    fr = i8.frange(f)
    if fr:
        sz8[f] = fr[1] - fr[0]

ok, rej = [], []
for o6, lst in sorted(byown.items(), key=lambda x: -len(x[1])):
    fr6 = i6.frange(o6)
    if not fr6:
        rej += [(s, 'owner 경계 미상') for s in lst]; continue
    s6 = fr6[1] - fr6[0]
    ins6, t6 = tk(i6, o6, '6')
    if not t6:
        rej += [(s, '0.5.6 디스어셈 실패') for s in lst]; continue
    cands = [f for f, z in sz8.items() if 0.6 * s6 <= z <= 1.6 * s6]
    sc = []
    for c in cands:
        i8i, t8 = tk(i8, c, '8')
        if not t8:
            continue
        sm = difflib.SequenceMatcher(None, t6, t8, autojunk=False)
        if sm.real_quick_ratio() < 0.55 or sm.quick_ratio() < 0.55:
            continue
        sc.append((sm.ratio(), c))
    sc.sort(reverse=True)
    if not sc:
        rej += [(s, 'owner 0x%x 후보 없음(크기 0x%x, 후보군 %d)' % (o6, s6, len(cands))) for s in lst]
        continue
    best = sc[0]; second = sc[1] if len(sc) > 1 else (0.0, 0)
    marg = best[0] - second[0]
    # ★동률(r=1.000 클론 다수)이면 **가르지 않고 전부** 취한다 — 소스가 클론을 전부 패치하므로
    #   어느 쪽이 "그" 짝인지 가릴 필요가 없다(entity 2벌 사례와 같은 구조).
    ties = [c for r, c in sc if best[0] - r <= 0.001]
    if best[0] >= 0.70 and marg >= 0.03:
        targets, mode = [best[1]], 'UNIQ'
    elif len(ties) > 1:
        targets, mode = ties, 'TIE:%d' % len(ties)
    elif len(sc) == 1:
        targets, mode = [best[1]], 'ONLY1(r=%.3f)' % best[0]   # 경쟁자 없음 = 그 자체가 근거
    else:
        rej += [(s_, 'r=%.3f 여유=%.3f' % (best[0], marg)) for s_ in lst]; continue
    print("  0x%-8x(%2d사이트, 0x%x) r=%.3f 2위 %.3f → %s %s"
          % (o6, len(lst), s6, best[0], second[0], mode,
             ' '.join('0x%x' % t for t in targets[:4])))
    for s_ in lst:
        ia = idx_of(ins6, s_['rva'])
        if ia is None:
            rej.append((s_, '0.5.6 인덱스 없음')); continue
        got = []
        for o8 in targets:
            ins8, _ = tk(i8, o8, '8')
            key = (o6, o8)
            if key not in TC:
                TC[key] = align(ins6, ins8)[0]
            sm = TC[key]
            tag, jb = map_idx(sm, ia)
            if tag != 'equal' or jb is None or jb >= len(ins8):
                continue
            na = ins8[jb].address - BASE
            if chk(i8, na, s_['pre'], s_['off'], s_['w'], s_['orig']):
                got.append(na)
        if not got:
            rej.append((s_, '전 후보에서 검증 실패')); continue
        for na in got:
            d = dict(s_); d['new'] = na; d['mode'] = mode
            ok.append(d)

print("\n★검증 통과 %d 주소 (앵커 사이트 %d개 기준)"
      % (len(ok), sum(len(v) for v in TB.values())))
uniq = len({(x['tbl'], x['new']) for x in ok})
print("   테이블·주소 중복 제거 후 %d개" % uniq)
c = Counter(s_['tbl'] for s_ in ok)
print("   테이블별: %s" % dict(c))
print("\n폐기 %d건" % len(rej))
for k, n in Counter(w.split('0x')[0].strip() for _, w in rej).most_common(8):
    print("   %-34s %d" % (k, n))
pickle.dump(ok, open(os.path.join(SP, 'tbl_new.pkl'), 'wb'))
print("\n→ tbl_new.pkl 저장")
