# -*- coding: utf-8 -*-
"""앵커 없는 죽은 사이트 — **prefix + 원본 즉치 서명**으로 0.5.8 exe 에서 직접 찾는다.

왜 이게 되는가: 소스의 `sq(pcol, 0x9_502F_9000)` / `b4(prea, 80_000)` 에서 두 번째 인자는
**게임 원본 즉치(ORIG)** 다. 8바이트 상수 `0x9_502F_9000`(=40009302016) 같은 값은
.text 전체에서 극히 드물다 ⟹ **prefix 바이트열 + 그 즉치**를 함께 찾으면 앵커(구버전 exe) 없이도
사이트를 특정할 수 있다. 유사도·정렬이 전혀 필요 없다.

자기검증: 같은 소스행의 **살아있는 형제 주소가 검색 결과에 전부 포함**돼야 한다.
포함되지 않으면 서명이 틀린 것이므로 그 행은 폐기한다.

채택 조건: |검색결과| == |소스가 가진 주소 수| 이고 살아있는 형제가 전부 결과에 있을 것.
그러면 **결과 중 소스에 없는 주소들**이 곧 죽은 사이트의 새 주소다(순서 무관 — 어차피 전부 패치한다).
"""
import io
import os
import re
import sys
import struct
import pickle
from collections import defaultdict, Counter

sys.stdout.reconfigure(encoding='utf-8', errors='replace')
sys.path.insert(0, r'C:\tfm2mods\MIG')
import pefile
import mig_verify as MV

pe = pefile.PE(MV.GAME_EXE, fast_load=True)
SECS = [(s.VirtualAddress, s.VirtualAddress + s.Misc_VirtualSize, s.get_data(),
         s.Name.rstrip(b'\x00').decode('ascii', 'replace')) for s in pe.sections]
TEXT = [x for x in SECS if x[3] == '.text'][0]


def rd(rva, n):
    for va, ve, d, _ in SECS:
        if va <= rva < ve:
            return d[rva - va:rva - va + n]
    return None


SRC = r'C:\tfm2mods\tfm2_ai_adjust\src'
CALL = re.compile(r'\b(p|pany)!\(|(?<![A-Za-z_])patch_imm_bytes\(')
NUM = re.compile(r'0x[0-9a-fA-F_]+|\d[\d_]*')
# sq(knob, ORIG) / sqp / b1 / b4 / dsh(knob, ORIG, ..) — 두 번째 인자가 원본 즉치
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


groups = defaultdict(lambda: dict(addrs=[], pre=None, off=0, w=0, orig=None, val=''))
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
            continue                      # 원본 즉치를 모르는 형태 → 이 방법 불가
        orig = int(om.group(1).replace('_', ''), 0)
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
        g = groups[(f, ln)]
        g['addrs'] += ad; g['pre'] = [int(y, 16) for y in pm]
        g['off'], g['w'], g['orig'] = off, w, orig
        g['val'] = a[4].strip()[:26]


def alive(g, a):
    b = rd(a, g['off'] + g['w'] + 4)
    return bool(b) and list(b[:len(g['pre'])]) == g['pre']


dead_groups = {k: g for k, g in groups.items() if any(not alive(g, a) for a in g['addrs'])}
ndead = sum(sum(1 for a in g['addrs'] if not alive(g, a)) for g in dead_groups.values())
print("원본즉치를 아는 소스행 %d개 중 죽은 사이트를 가진 행 %d개 / 죽은 사이트 %d건\n"
      % (len(groups), len(dead_groups), ndead))

TVA, TVE, TD, _ = TEXT
PACK = {1: 'B', 4: '<I', 8: '<Q'}


def sigscan(pre, off, w, orig):
    """.text 전체에서 prefix + 그 위치의 즉치 == orig 인 주소를 전부 찾는다."""
    need = struct.pack(PACK[w], orig) if w in PACK else None
    if need is None:
        return None
    pb = bytes(pre)
    out, i = [], 0
    n = len(TD)
    while True:
        i = TD.find(pb, i)
        if i < 0 or i + off + w > n:
            break
        if TD[i + off:i + off + w] == need:
            out.append(TVA + i)
        i += 1
        if len(out) > 400:
            break
    return out


ok, rej = [], []
for (f, ln), g in sorted(dead_groups.items()):
    dead = [a for a in g['addrs'] if not alive(g, a)]
    live = [a for a in g['addrs'] if alive(g, a)]
    hits = sigscan(g['pre'], g['off'], g['w'], g['orig'])
    if hits is None:
        rej.append((f, ln, g, '폭 %d 미지원' % g['w'])); continue
    miss = [a for a in live if a not in hits]
    if miss:
        rej.append((f, ln, g, '자기검증 실패 — 살아있는 형제 %d개가 결과에 없음' % len(miss)))
        continue
    extra = [h for h in hits if h not in g['addrs']]
    if len(hits) == len(g['addrs']) and len(extra) == len(dead):
        ok.append(dict(fn=f, line=ln, dead=dead, new=sorted(extra), live=live,
                       pre=g['pre'], off=g['off'], w=g['w'], orig=g['orig'], val=g['val'],
                       ev='서명 %d개 == 소스 %d개, 산 형제 %d개 전부 포함'
                          % (len(hits), len(g['addrs']), len(live))))
    else:
        rej.append((f, ln, g,
                    '수 불일치 서명=%d 소스=%d 죽음=%d 여분=%d'
                    % (len(hits), len(g['addrs']), len(dead), len(extra))))

nfix = sum(len(o['dead']) for o in ok)
print("★서명 일치 → 채택 : %d 소스행 / %d 사이트\n" % (len(ok), nfix))
for o in ok:
    print("  %-11s:%-5d 죽음%2d → %s"
          % (o['fn'], o['line'], len(o['dead']), ' '.join('0x%x' % a for a in o['new'][:6])))
    print("      %-26s w=%d ORIG=%s  %s" % (o['val'], o['w'], o['orig'], o['ev']))

print("\n폐기 %d 소스행" % len(rej))
for k, n in Counter(w.split('—')[0].split('서명=')[0].strip() for _, _, _, w in rej).most_common(8):
    print("   %-40s %d행" % (k, n))
print("\n--- 폐기 상세(상위 14) ---")
for f, ln, g, w in rej[:14]:
    d = sum(1 for a in g['addrs'] if not alive(g, a))
    print("   %-11s:%-5d 죽음%2d/%2d  %-24s %s" % (f, ln, d, len(g['addrs']), g['val'], w))

pickle.dump(ok, open('sig_ok.pkl', 'wb'))
print("\n→ sig_ok.pkl 저장")
