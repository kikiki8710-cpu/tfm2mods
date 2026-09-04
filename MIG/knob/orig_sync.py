# -*- coding: utf-8 -*-
"""★orig_table.rs 이관 v2 — 재핀 RVA 를 표에 옮기고, 표에 없던 것은 **추가**한다.

배경(이 세션 최대 발견): `patch_imm_bytes` 의 모든 쓰기는 `orig_guard_ok` 를 통과해야 하고
그 가드는 2026-09-02 부터 **fail-CLOSED** — 표에 없는 RVA 는 **거부**한다.
⟹ 사이트만 재핀하고 표를 안 고치면 **바이트는 맞는데 패치가 하나도 안 나간다**(무증상).

v1 의 한계: 표에 이미 있던 구주소만 옮겨 122행. 구주소가 표에 **없던 121개**는 여전히 차단된다.
v2 는 그것들을 **추가**한다. 추가 행의 `expect_orig` 는 다음 우선순위로 정한다.
  ① 소스가 ORIG 를 명시하면(`b4(k,ORIG)`·`sq/sqp/dsh(k,ORIG,..)`) 그 값을 쓰되
     **exe 실측과 일치할 때만** 추가한다(진짜 교차검증).
  ② 명시가 없으면(`gh`·`v` 처럼 값이 변수) exe 실측값을 쓰고 `[exe유래]` 로 표시한다.
     이 경우 가드는 "이 자리가 아직 원본인가"만 확인하게 되지만, 재핀 때 prefix 로 확인했고
     표에 없으면 **아예 못 쓰므로** 추가하는 편이 낫다.
"""
import io
import os
import re
import subprocess
import sys
import struct
from collections import Counter

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


def imm_at(rva, off, w):
    b = rd(rva, off + w)
    if b is None or len(b) < off + w:
        return None
    if w == 1:
        return b[off]
    if w == 2:
        return struct.unpack('<H', b[off:off + 2])[0]
    if w == 4:
        return struct.unpack('<I', b[off:off + 4])[0]
    if w == 8:
        return struct.unpack('<Q', b[off:off + 8])[0]
    return None


# ── 1. 이번 세션 구→신 쌍 ──
BASE = 'a87d00d~1'   # ★09-04 아침 19건 재핀(a87d00d)도 표 이관이 안 돼 있었다
HEX = re.compile(r'0x0*([0-9a-fA-F]{5,8})')


def rvas(line):
    return [int(m.group(1), 16) for m in HEX.finditer(line.split('//')[0])
            if 0xc00000 <= int(m.group(1), 16) <= 0x3000000]


pairs = {}
for f in ('tfm2_ai_adjust/src/detour.rs', 'tfm2_ai_adjust/src/disc19_repro.rs'):
    d = subprocess.run(['git', '-C', r'C:\tfm2mods', 'diff', '-U0', BASE, '--', f],
                       capture_output=True, text=True, encoding='utf-8', errors='replace').stdout
    ob, nb = [], []

    def flush():
        o = [v for l in ob for v in rvas(l)]
        n = [v for l in nb for v in rvas(l)]
        if o and n and len(o) == len(n):
            for a, b in zip(o, n):
                if a != b:
                    pairs.setdefault(a, b)
        ob.clear(); nb.clear()

    for line in d.split('\n'):
        if line.startswith('@@'):
            flush()
        elif line.startswith('-') and not line.startswith('---'):
            if nb:
                flush()
            ob.append(line[1:])
        elif line.startswith('+') and not line.startswith('+++'):
            nb.append(line[1:])
        else:
            flush()
    flush()
print("구→신 쌍 %d개" % len(pairs))

# ── 2. 현행 소스에서 (rva, off, w, 명시ORIG) 수집 ──
exec(open('_audit_parse.py', encoding='utf-8').read())   # sites: list of dict
SITE = {}
for s in sites:
    SITE.setdefault(s['rva'], s)
print("소스 사이트 %d개 (그중 ORIG 명시 %d)"
      % (len(SITE), sum(1 for s in SITE.values() if s['orig'] is not None)))

# ── 3. 표 로드 ──
P = r'C:\tfm2mods\tfm2_ai_adjust\src\orig_table.rs'
src = io.open(P, encoding='utf-8').read()
ROW = re.compile(r'^(\s*)\((0x[0-9a-fA-F]+),\s*(\d+),\s*(\d+),\s*(\d+)\),(.*)$', re.M)
ms = list(ROW.finditer(src))
rows = [dict(rva=int(m.group(2), 16), off=int(m.group(3)), w=int(m.group(4)),
             orig=int(m.group(5)), tail=m.group(6).rstrip()) for m in ms]
head, tail_txt = src[:ms[0].start()], src[ms[-1].end():]
byrva = {}
for r in rows:
    byrva.setdefault(r['rva'], []).append(r)
print("표 %d행" % len(rows))

# ── 4. 이관 ──
moved, hold = 0, []
for old, new in pairs.items():
    for r in byrva.get(old, []):
        cur = imm_at(new, r['off'], r['w'])
        if cur is None or cur != r['orig']:
            hold.append((old, new, 'exe %s != expect %s' % (cur, r['orig']))); continue
        r['rva'] = new
        r['tail'] += '   // ★0.5.8 이관(구 0x%x)' % old
        moved += 1

# ── 5. 표에 없던 신주소 추가 ──
have = {r['rva'] for r in rows}
added_x, added_e, miss = 0, 0, []
for old, new in pairs.items():
    if new in have:
        continue
    s = SITE.get(new)
    if s is None:
        miss.append((new, '소스에서 (off,w) 미확보')); continue
    cur = imm_at(new, s['off'], s['w'])
    if cur is None:
        miss.append((new, '섹션 밖')); continue
    if s['orig'] is not None:
        if cur != s['orig']:
            miss.append((new, 'exe %s != 소스ORIG %s' % (cur, s['orig']))); continue
        rows.append(dict(rva=new, off=s['off'], w=s['w'], orig=cur,
                         tail='   // ★0.5.8 신규(구 0x%x·소스ORIG 교차확인)' % old))
        added_x += 1
    else:
        rows.append(dict(rva=new, off=s['off'], w=s['w'], orig=cur,
                         tail='   // ★0.5.8 신규(구 0x%x·[exe유래] 소스에 ORIG 표기 없음)' % old))
        added_e += 1
    have.add(new)

print("\n이관 %d행 / 신규추가 %d행(소스ORIG 교차확인) + %d행([exe유래])"
      % (moved, added_x, added_e))
print("보류 %d / 추가불가 %d" % (len(hold), len(miss)))
for a, b, w in hold[:6]:
    print("   보류 0x%x→0x%x %s" % (a, b, w))
for a, w in miss[:8]:
    print("   불가 0x%x %s" % (a, w))

# ── 6. 정렬·중복 검사 ──
rows.sort(key=lambda r: (r['rva'], r['off'], r['w']))
ded, seen = [], set()
for r in rows:
    k = (r['rva'], r['off'], r['w'], r['orig'])
    if k in seen:
        continue
    seen.add(k); ded.append(r)
print("\n완전중복 제거 %d행 → %d행" % (len(rows) - len(ded), len(ded)))
conf = [ded[i]['rva'] for i in range(1, len(ded)) if ded[i]['rva'] == ded[i - 1]['rva']]
if conf:
    print("⚠같은 RVA 에 서로 다른 (off,w) %d건 — 이분탐색은 첫 행만 보므로 확인 필요:" % len(conf))
    for k in conf[:6]:
        print("   0x%x → %s" % (k, [(r['off'], r['w'], r['orig']) for r in ded if r['rva'] == k]))

body = '\n'.join("    (0x%x, %d, %d, %d),%s" % (r['rva'], r['off'], r['w'], r['orig'], r['tail'])
                 for r in ded)
io.open(P + '.bak_migrate', 'w', encoding='utf-8', newline='').write(src)
io.open(P, 'w', encoding='utf-8', newline='').write(head + body + tail_txt)
ok = all(ded[i]['rva'] >= ded[i - 1]['rva'] for i in range(1, len(ded)))
print("\n표 재작성 %d행 / 오름차순: %s" % (len(ded), 'OK' if ok else '★깨짐'))
