# -*- coding: utf-8 -*-
"""서명 검색 2차 — **살아있는 형제의 소속 함수로 범위를 좁힌다**.

1차(`sigsearch.py`)는 .text 전역을 훑어 "서명 17개 vs 소스 8개" 같은 수 불일치로 32행을 폐기했다.
그런데 같은 소스행의 **살아있는 형제**는 그 사이트들이 어느 함수에 있는지 알려주는 **하드 증거**다
(유사도 추정이 아니라 실제로 지금 살아 있는 주소). 그 함수 안으로 좁혀 다시 센다.

채택 조건(1차와 동일 + 범위 축소)
  · 살아있는 형제 ≥ 1 (없으면 소속 함수를 모르니 폐기)
  · 그 형제들이 속한 함수들 안에서만 서명을 세고, **개수 == 소스 주소 수**
  · 산 형제가 전부 결과에 포함
  · 신주소 전역 유일(다른 행이 주장하지 않고, 어느 행의 산 사이트와도 겹치지 않음)
"""
import io
import os
import re
import sys
import struct
import pickle
from collections import defaultdict, Counter

sys.stdout.reconfigure(encoding='utf-8', errors='replace')
K = r'C:\tfm2mods\MIG\knob'
os.chdir(K)
exec(open(os.path.join(K, 'lib.py'), encoding='utf-8').read())

img8 = Img(r"C:\Program Files (x86)\Steam\steamapps\common"
           r"\Teamfight Manager2\TeamfightManager2.exe")
img8.prep()

SRC = r'C:\tfm2mods\tfm2_ai_adjust\src'
CALL = re.compile(r'\b(p|pany)!\(|(?<![A-Za-z_])patch_imm_bytes\(')
NUM = re.compile(r'0x[0-9a-fA-F_]+|\d[\d_]*')
ORIGRE = re.compile(r'^[A-Za-z_]\w*\(\s*\w+\s*,\s*(0x[0-9a-fA-F_]+|\d[\d_]*)\s*[,)]')


def split_args(s):
    out, d, cur = [], 0, ''
    for ch in s:
        if ch in '([{':
            d += 1
        elif ch in ')]}':
            d -= 1
        if ch == ',' and d == 0:
            out.append(cur); cur = ''
        else:
            cur += ch
    if cur.strip():
        out.append(cur)
    return [x.strip() for x in out]


def cbody(t, i):
    d = 0
    for j in range(i, min(len(t), i + 4000)):
        if t[j] == '(':
            d += 1
        elif t[j] == ')':
            d -= 1
            if d == 0:
                return t[i + 1:j]
    return None


groups = {}
for f in sorted(os.listdir(SRC)):
    if not f.endswith('.rs'):
        continue
    txt = io.open(os.path.join(SRC, f), encoding='utf-8', errors='replace').read()
    loops = []
    for m in re.finditer(r'for\s+([A-Za-z_]\w*)\s+in\s*\[([^\]]*)\]', txt):
        v = [int(x.replace('_', ''), 0) for x in NUM.findall(m.group(2))]
        if v:
            loops.append((m.start(), m.group(1), v))
    for m in CALL.finditer(txt):
        if 'pskip' in txt[max(0, m.start() - 8):m.start() + 6]:
            continue
        inner = cbody(txt, txt.index('(', m.start()))
        if not inner:
            continue
        a = split_args(inner)
        if len(a) < 5 or '&[' not in a[1]:
            continue
        pm = re.findall(r'0x[0-9a-fA-F]+', a[1])
        if not pm:
            continue
        try:
            off = int(NUM.search(a[2]).group(0).replace('_', ''), 0)
            w = int(NUM.search(a[3]).group(0).replace('_', ''), 0)
        except Exception:
            continue
        om = ORIGRE.match(a[4].strip())
        if not om:
            continue
        ad = []
        hm = re.fullmatch(r'(?:base\s*\+\s*)?(0x[0-9a-fA-F]+)', a[0].strip())
        if hm:
            ad = [int(hm.group(1), 16)]
        else:
            vm = re.fullmatch(r'base\s*\+\s*([A-Za-z_]\w*)', a[0].strip())
            if vm:
                c = [v for pos, nm, v in loops if nm == vm.group(1) and pos < m.start()]
                if c:
                    ad = c[-1]
        if not ad:
            continue
        ln = txt[:m.start()].count('\n') + 1
        g = groups.setdefault((f, ln), dict(addrs=[], pre=[int(y, 16) for y in pm],
                                            off=off, w=w,
                                            orig=int(om.group(1).replace('_', ''), 0),
                                            val=a[4].strip()[:26]))
        g['addrs'] += ad


def alive(g, a):
    b = img8.code(a, g['off'] + g['w'] + 4)
    return bool(b) and list(b[:len(g['pre'])]) == g['pre']


PACK = {1: 'B', 4: '<I', 8: '<Q'}


def scan_fn(fn, pre, off, w, orig):
    """함수 fn 의 모든 조각에서 prefix+즉치 서명 위치를 찾는다."""
    pb, parts = img8.fparts(fn)
    if pb is None:
        return []
    need = struct.pack(PACK[w], orig)
    out = []
    for b, e in parts:
        d = img8.code(b, e - b)
        if not d:
            continue
        i = 0
        while True:
            i = d.find(bytes(pre), i)
            if i < 0 or i + off + w > len(d):
                break
            if d[i + off:i + off + w] == need:
                out.append(b + i)
            i += 1
    return out


cand, rej = [], []
for (f, ln), g in sorted(groups.items()):
    dead = [a for a in g['addrs'] if not alive(g, a)]
    if not dead:
        continue
    live = [a for a in g['addrs'] if alive(g, a)]
    if not live:
        rej.append((f, ln, g, dead, '산 형제 0 — 소속 함수 미상')); continue
    owners = []
    for a in live:
        fr = img8.frange(a)
        if fr and fr[0] not in owners:
            owners.append(fr[0])
    if not owners:
        rej.append((f, ln, g, dead, '산 형제의 함수 경계 미상')); continue
    hits = []
    for o in owners:
        hits += scan_fn(o, g['pre'], g['off'], g['w'], g['orig'])
    hits = sorted(set(hits))
    miss = [a for a in live if a not in hits]
    if miss:
        rej.append((f, ln, g, dead, '자기검증 실패(산 형제 %d개 누락)' % len(miss))); continue
    extra = [h for h in hits if h not in g['addrs']]
    if len(hits) == len(g['addrs']) and len(extra) == len(dead):
        cand.append(dict(fn=f, line=ln, dead=dead, new=sorted(extra), live=live,
                         pre=g['pre'], off=g['off'], w=g['w'], orig=g['orig'], val=g['val'],
                         ev='함수%d개 내 서명 %d == 소스 %d, 산형제 %d 포함'
                            % (len(owners), len(hits), len(g['addrs']), len(live))))
    else:
        rej.append((f, ln, g, dead, '수 불일치 함수내서명=%d 소스=%d 여분=%d'
                    % (len(hits), len(g['addrs']), len(extra))))

# 전역 검사
claim = Counter()
for c in cand:
    for a in c['new']:
        claim[a] += 1
livall = set()
for g in groups.values():
    livall |= {a for a in g['addrs'] if alive(g, a)}
good, drop = [], []
for c in cand:
    why = []
    if any(claim[a] > 1 for a in c['new']):
        why.append('주소 충돌')
    if any(a in livall for a in c['new']):
        why.append('산 사이트와 겹침')
    (drop if why else good).append((c, ' / '.join(why)))

n = sum(len(c['dead']) for c, _ in good)
print("★함수범위 서명 통과 : %d 소스행 / %d 사이트\n" % (len(good), n))
for c, _ in good:
    print("  %-11s:%-5d 죽음%2d → %s" %
          (c['fn'], c['line'], len(c['dead']), ' '.join('0x%x' % a for a in c['new'][:8])))
    print("      %-26s ORIG=%-12s %s" % (c['val'], c['orig'], c['ev']))
print("\n전역검사 탈락 %d행" % len(drop))
for c, w in drop:
    print("   %-11s:%-5d %-26s %s" % (c['fn'], c['line'], c['val'], w))
print("\n폐기 %d행 / %d 사이트" % (len(rej), sum(len(d) for _, _, _, d, _ in rej)))
for k, v in Counter(w.split('함수내서명')[0].strip() for _, _, _, _, w in rej).most_common(8):
    print("   %-40s %d행" % (k, v))
pickle.dump([c for c, _ in good], open('sig2_final.pkl', 'wb'))
print("\n→ sig2_final.pkl 저장")
