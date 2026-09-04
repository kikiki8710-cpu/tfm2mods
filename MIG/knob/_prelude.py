# -*- coding: utf-8 -*-
"""죽은 사이트 재핀 — **증거 = 단형화 집합의 전단사(bijection)**.

왜 이 모델인가
  1차: owner 를 "모듈+크기"로 골랐다 → 147건 전부 크기추정 = 형제 오선택 위험.
  2차: 즉치까지 고정한 명령창 유일성 → 145건 0매치(0.5.8은 구조체 오프셋 자체가 밀림).
  ★관찰: 소스가 **한 줄에서 주소를 K개** 패치한다(`for a in [0x.., 0x..]`).
     = 그 줄은 제네릭 단형화 형제를 **전부** 패치하고 있다.
     ⟹ 형제 A/B 중 어느 게 어느 것인지 가릴 필요가 없다. **집합이 빠짐없이 1:1로 덮이면 충분**.

판정
  · 소스행의 사이트들이 사는 0.5.7 owner 집합 S, 같은 모듈의 0.5.8 함수 집합 C.
  · 모든 (o7, c) 쌍을 명령스트림 정렬 → 사이트 인덱스 사상 → **prefix 검증**.
  · 각 c 가 정확히 한 주소를 받고, |덮인 c| == |S| 이면 **전단사 성립 → 채택**.
  · 아니면 폐기(부분 채택 금지 — 한 형제만 패치하면 AI가 반쪽만 바뀐다).
"""
import io
import os
import re
import sys
import struct
import pickle
from collections import Counter, defaultdict

sys.stdout.reconfigure(encoding='utf-8', errors='replace')
K = r'C:\tfm2mods\MIG\knob'
os.chdir(K)
exec(open(os.path.join(K, 'align.py'), encoding='utf-8').read())

img7 = Img(r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"); img7.prep()
img8 = Img(r"C:\Program Files (x86)\Steam\steamapps\common"
           r"\Teamfight Manager2\TeamfightManager2.exe"); img8.prep()
s7, p7 = build_strs(img7, os.path.join(K, '_s57k.pkl'))
s8, p8 = build_strs(img8, os.path.join(K, '_s58k.pkl'))
om7 = ownmap(img7, s7, p7, os.path.join(K, '_om57.pkl'))
om8 = ownmap(img8, s8, p8, os.path.join(K, '_om58.pkl'))

mod2fn8 = defaultdict(list)
for fn, cnt in om8.items():
    if cnt:
        mod2fn8[cnt.most_common(1)[0][0]].append(fn)
mod2fn7 = defaultdict(list)
for fn, cnt in om7.items():
    if cnt:
        mod2fn7[cnt.most_common(1)[0][0]].append(fn)

SRC = r'C:\tfm2mods\tfm2_ai_adjust\src'
CALL = re.compile(r'\b(p|pany)!\(|(?<![A-Za-z_])patch_imm_bytes\(')
NUM = re.compile(r'0x[0-9a-fA-F]+|\d+')


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


rows = []
for f in sorted(os.listdir(SRC)):
    if not f.endswith('.rs'):
        continue
    txt = io.open(os.path.join(SRC, f), encoding='utf-8', errors='replace').read()
    loops = []
    for m in re.finditer(r'for\s+([A-Za-z_]\w*)\s+in\s*\[([^\]]*)\]', txt):
        v = [int(x, 0) for x in NUM.findall(m.group(2))]
        if v:
            loops.append((m.start(), m.group(1), v))
    for m in CALL.finditer(txt):
        if 'pskip' in txt[max(0, m.start() - 8):m.start() + 6]:
            continue
        inner = cbody(txt, txt.index('(', m.start()))
        if not inner:
            continue
        a = split_args(inner)
        if len(a) < 4 or '&[' not in a[1]:
            continue
        pm = re.findall(r'0x[0-9a-fA-F]+', a[1])
        if not pm:
            continue
        try:
            off = int(NUM.search(a[2]).group(0), 0); w = int(NUM.search(a[3]).group(0), 0)
        except Exception:
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
        ln = txt[:m.start()].count('\n') + 1
        for x in ad:
            rows.append(dict(fn=f, line=ln, rva=x, pre=[int(y, 16) for y in pm],
                             off=off, w=w, val=(a[4].strip()[:26] if len(a) > 4 else ''),
                             loop=(not hm)))


def alive(img, r):
    b = img.code(r['rva'], r['off'] + r['w'] + 4)
    return b and list(b[:len(r['pre'])]) == r['pre']


dead = [r for r in rows if not alive(img8, r) and alive(img7, r)]
grp = defaultdict(list)
for r in dead:
    grp[(r['fn'], r['line'])].append(r)
print("0.5.7 유효 + 0.5.8 사망 = %d건 / 소스행 %d개\n" % (len(dead), len(grp)))

SC, AC = {}, {}


def st(img, fn, tag):
    k = (tag, fn)
    if k not in SC:
        SC[k] = stream(img, fn)
    return SC[k]


def al(o7, c8):
    k = (o7, c8)
    if k not in AC:
        i7, i8 = st(img7, o7, '7'), st(img8, c8, '8')
        AC[k] = None if not i7 or not i8 else (i7, i8, align(i7, i8)[0])
    return AC[k]


