# -*- coding: utf-8 -*-
"""감사 3차 — **파싱 실패 63건의 사각지대를 닫는다**.

`imm_audit2.py` 는 `if len(a) < 4 or '&[' not in a[1]` 로 걸러서
**`pany!(base+a, [[..],[..],[..]], off, w, v)`** 형태(= prefix 후보 여러 개)를 통째로 건너뛰었다.
즉 그 사이트들은 **죽었는지 살았는지 판정 자체가 없었다** — 감사가 "정상"이라고 말해 온 것도 아니고
그냥 안 본 것이다. 이 사각지대를 닫는다.

판정 규칙
  · `pany!` : prefix 후보 중 **하나라도** 일치하면 살아 있음(그게 매크로의 의미).
  · `patch_imm_bytes(..) || patch_imm_bytes(..)` : 같은 이유로 하나라도 맞으면 살아 있음.
  · 그 외 파싱 실패는 **"판정 불가"로 따로 세어** 숨기지 않는다.
"""
import io
import os
import re
import sys
import struct
from collections import Counter, defaultdict

sys.path.insert(0, r'C:\tfm2mods\MIG')
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
import pefile
import mig_verify as MV

pe = pefile.PE(MV.GAME_EXE, fast_load=True)
SEC = [(s.VirtualAddress, s.VirtualAddress + s.Misc_VirtualSize, s.get_data())
       for s in pe.sections]


def rd(r, n):
    for a, b, d in SEC:
        if a <= r < b:
            return d[r - a:r - a + n]
    return None


SRC = r'C:\tfm2mods\tfm2_ai_adjust\src'
CALL = re.compile(r'\b(p|pany)!\(|(?<![A-Za-z_])patch_imm_bytes\(')
NUM = re.compile(r'0x[0-9a-fA-F_]+|\d[\d_]*')


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


def prefixes(expr):
    """`&[0x..,0x..]` → [[..]] / `[[0x..],[0x..]]` → 여러 후보. 없으면 None."""
    e = expr.strip()
    if e.startswith('&['):
        h = re.findall(r'0x[0-9a-fA-F]+', e)
        return [[int(x, 16) for x in h]] if h else None
    if e.startswith('[['):
        outs = []
        for grp in re.findall(r'\[([^\[\]]*)\]', e[1:]):
            h = re.findall(r'0x[0-9a-fA-F]+', grp)
            if h:
                outs.append([int(x, 16) for x in h])
        return outs or None
    return None


def static_prefix_tables(txt):
    """`static NAME: [[u8;N];M] = [[0x..],[0x..]];` → {NAME: [[..],[..]]}.
    이걸 안 하면 `pany!(base+a, CMP3, ..)` 같은 호출이 통째로 판정 불가로 빠진다."""
    out = {}
    for m in re.finditer(r'(?:static|const)\s+([A-Z_][A-Z0-9_]*)\s*:\s*\[\[u8;\s*\d+\];\s*\d+\]\s*=\s*(\[.*?\]);',
                         txt, re.S):
        grps = []
        for g in re.findall(r'\[([^\[\]]*)\]', m.group(2)[1:]):
            h = re.findall(r'0x[0-9a-fA-F]+', g)
            if h:
                grps.append([int(x, 16) for x in h])
        if grps:
            out[m.group(1)] = grps
    return out


rows, unparsed = [], []
for f in sorted(os.listdir(SRC)):
    if not f.endswith('.rs'):
        continue
    txt = io.open(os.path.join(SRC, f), encoding='utf-8', errors='replace').read()
    STAT = static_prefix_tables(txt)
    # ⚠튜플 루프 `for (a, v, o) in [(0x.., x, y), ..]` 를 지원하지 않으면
    #   같은 이름을 쓴 **앞선 다른 루프**의 주소가 잘못 붙어 **가짜 사이트**가 생긴다
    #   (2026-09-04 실측: L1487 에 무관한 주소 5개가 붙어 "죽은 사이트"로 오탐).
    loops = []
    for m in re.finditer(r'for\s+([A-Za-z_]\w*)\s+in\s*\[([^\]]*)\]', txt):
        v = [int(x.replace('_', ''), 0) for x in NUM.findall(m.group(2))]
        if v:
            loops.append((m.start(), m.group(1), v))
    for m in re.finditer(r'for\s*\(([^)]*)\)\s+in\s*\[(.*?)\]\s*\{', txt, re.S):
        # ★튜플의 **모든 원소**를 각각 바인딩한다. 첫 원소만 읽으면 `(c, m)` 의 `m`(=mov 짝)이
        #   통째로 빠져 사이트 목록에서 사라진다 — 2026-09-04 인게임 검증에서 이 누락 8건이
        #   `imm_unknown.txt` 로 드러났다(가드가 거부 = 패치 안 나감).
        names = [x.strip() for x in m.group(1).split(',')]
        cols = []
        for t in re.findall(r'\(([^()]*)\)', m.group(2)):
            parts = [x.strip() for x in t.split(',')]
            cols.append(parts)
        for idx, nm in enumerate(names):
            vals = []
            for parts in cols:
                if idx < len(parts):
                    h = re.match(r'(0x[0-9a-fA-F]+)', parts[idx])
                    if h:
                        vals.append(int(h.group(1), 16))
            if vals and len(vals) == len(cols):
                loops.append((m.start(), nm, vals))
        continue
    for m in re.finditer(r'(?!)x', txt):
        pass
    loops.sort()
    for m in CALL.finditer(txt):
        ln = txt[:m.start()].count('\n') + 1
        if 'pskip' in txt[max(0, m.start() - 8):m.start() + 6]:
            continue
        inner = cbody(txt, txt.index('(', m.start()))
        if not inner:
            unparsed.append((f, ln, '괄호 매칭 실패')); continue
        a = split_args(inner)
        if len(a) < 4:
            unparsed.append((f, ln, '인자 %d개' % len(a))); continue
        pres = prefixes(a[1]) or STAT.get(a[1].strip().lstrip('&'))
        if not pres:
            unparsed.append((f, ln, 'prefix 형태 미지원: %s' % a[1][:28])); continue
        try:
            off = int(NUM.search(a[2]).group(0).replace('_', ''), 0)
            w = int(NUM.search(a[3]).group(0).replace('_', ''), 0)
        except Exception:
            unparsed.append((f, ln, 'off/w 파싱 실패')); continue
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
            unparsed.append((f, ln, '주소식: %s' % a[0].strip()[:26])); continue
        multi = len(pres) > 1
        for x in ad:
            rows.append(dict(fn=f, line=ln, rva=x, pres=pres, off=off, w=w, multi=multi,
                             val=(a[4].strip()[:30] if len(a) > 4 else '')))


ORIGRE = re.compile(r"^[A-Za-z_]\w*\(\s*\w+\s*,\s*(0x[0-9a-fA-F_]+|\d[\d_]*)\s*[,)]")
sites = []
for r in rows:
    m = ORIGRE.match(r['val'].strip())
    sites.append(dict(rva=r['rva'], off=r['off'], w=r['w'],
                      orig=(int(m.group(1).replace('_',''), 0) if m else None),
                      val=r['val']))
