# -*- coding: utf-8 -*-
"""죽은 사이트 각각이 **마지막으로 유효했던 게임 버전**을 찾는다.

왜 필요한가: 재핀의 출발점(앵커)은 "그 주소가 실제로 맞던 exe"다.
2026-09-04 실측으로 20건 중 18건이 **0.5.4 가 마지막**이었다(0.5.5부터 3회 이월된 스테일).
나머지 335건도 제각각일 수 있으므로 먼저 버전을 갈라야 owner 를 제대로 잡는다.

판정: 그 RVA 에서 소스가 기대하는 prefix 바이트가 **그대로 일치**하면 그 버전에서 유효.
"""
import io
import os
import re
import sys
import struct
from collections import Counter

sys.stdout.reconfigure(encoding='utf-8', errors='replace')

SRC = r'C:\tfm2mods\tfm2_ai_adjust\src'
ANA = r'C:\Users\jungs\Desktop\claude\tfm2'
CUR = (r'C:\Program Files (x86)\Steam\steamapps\common'
       r'\Teamfight Manager2\TeamfightManager2.exe')

VERS = ['0.5.7', '0.5.6', '0.5.5', '0.5.4', '0.5.3', '0.5.2', '0.5.1', '0.5.0']


class Exe:
    def __init__(s, p):
        d = s.d = open(p, 'rb').read()
        e = struct.unpack_from('<I', d, 0x3c)[0]
        n = struct.unpack_from('<H', d, e + 6)[0]
        ss = e + 24 + struct.unpack_from('<H', d, e + 20)[0]
        s.secs = []
        for i in range(n):
            o = ss + i * 40
            va = struct.unpack_from('<I', d, o + 12)[0]
            vsz = struct.unpack_from('<I', d, o + 8)[0]
            rsz = struct.unpack_from('<I', d, o + 16)[0]
            pr = struct.unpack_from('<I', d, o + 20)[0]
            s.secs.append((va, max(vsz, rsz), pr))

    def rd(s, r, n):
        for va, sz, pr in s.secs:
            if va <= r < va + sz:
                o = pr + (r - va)
                return s.d[o:o + n]
        return None


exes = {}
for v in VERS:
    p = os.path.join(ANA, 'tfm2_' + v, 'TeamfightManager2.exe')
    if os.path.isfile(p):
        exes[v] = Exe(p)
exes['0.5.8'] = Exe(CUR)
print("로드된 exe: %s\n" % ', '.join(sorted(exes)))

# --- 죽은 사이트 수집 (imm_audit2 와 같은 파서) ---
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


def call_body(txt, i):
    d = 0
    for j in range(i, min(len(txt), i + 4000)):
        if txt[j] == '(':
            d += 1
        elif txt[j] == ')':
            d -= 1
            if d == 0:
                return txt[i + 1:j], j
    return None, i


rows = []
for fn in sorted(os.listdir(SRC)):
    if not fn.endswith('.rs'):
        continue
    txt = io.open(os.path.join(SRC, fn), encoding='utf-8', errors='replace').read()
    loops = []
    for m in re.finditer(r'for\s+([A-Za-z_]\w*)\s+in\s*\[([^\]]*)\]', txt):
        vals = [int(x, 0) for x in NUM.findall(m.group(2))]
        if vals:
            loops.append((m.start(), m.group(1), vals))
    for m in CALL.finditer(txt):
        if 'pskip' in txt[max(0, m.start() - 8):m.start() + 6]:
            continue
        op = txt.index('(', m.start())
        inner, _ = call_body(txt, op)
        if inner is None:
            continue
        a = split_args(inner)
        if len(a) < 4 or '&[' not in a[1]:
            continue
        pm = re.findall(r'0x[0-9a-fA-F]+', a[1])
        if not pm:
            continue
        pre = [int(x, 16) for x in pm]
        try:
            off = int(NUM.search(a[2]).group(0), 0)
            w = int(NUM.search(a[3]).group(0), 0)
        except Exception:
            continue
        addrs = []
        hm = re.fullmatch(r'(?:base\s*\+\s*)?(0x[0-9a-fA-F]+)', a[0].strip())
        if hm:
            addrs = [int(hm.group(1), 16)]
        else:
            vm = re.fullmatch(r'base\s*\+\s*([A-Za-z_]\w*)', a[0].strip())
            if vm:
                c = [v for pos, nm, v in loops if nm == vm.group(1) and pos < m.start()]
                if c:
                    addrs = c[-1]
        if not addrs:
            continue
        val = (a[4].strip()[:26] if len(a) > 4 else '')
        line = txt[:m.start()].count('\n') + 1
        for ad in addrs:
            rows.append((fn, line, ad, pre, off, w, val))

cur = exes['0.5.8']
dead = [r for r in rows
        if (lambda b: b is None or list(b[:len(r[3])]) != r[3])(cur.rd(r[2], r[4] + r[5] + 4))]
print("전체 %d / 죽은 사이트 %d\n" % (len(rows), len(dead)))

hit = Counter()
per = {}
for fn, line, rva, pre, off, w, val in dead:
    where = None
    for v in VERS:
        e = exes.get(v)
        if not e:
            continue
        b = e.rd(rva, off + w + 4)
        if b is not None and list(b[:len(pre)]) == pre:
            where = v
            break
    hit[where or '(어느 버전에도 없음)'] += 1
    per.setdefault(where or 'NONE', []).append((fn, line, rva, val))

print("=== 마지막 유효 버전 분포 ===")
for k, n in hit.most_common():
    print("   %-22s %4d건" % (k, n))

print("\n=== 버전별 소스행 (상위) ===")
for v, lst in sorted(per.items()):
    c = Counter((f, l, val) for f, l, _, val in lst)
    print("  [%s] %d사이트 / %d행" % (v, len(lst), len(c)))
    for (f, l, val), n in c.most_common(6):
        print("      %-16s:%-5d %2d개  %s" % (f, l, n, val))
