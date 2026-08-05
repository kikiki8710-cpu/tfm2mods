# -*- coding: utf-8 -*-
"""갱신된 소스 상수를 0.5.4 exe 로 전수 검증(시그니처<->emit 짝 포함)."""
import io, re, sys
from bo_054 import N
S = io.open(r'C:\tfm2mods\tfm2_banpick_order\src\hooks.rs', encoding='utf-8').read()
D = io.open(r'C:\tfm2mods\tfm2_banpick_order\src\diag.rs', encoding='utf-8').read()
fail = 0
def chk(tag, rva, want):
    global fail
    got = N.read(rva, len(want))
    ok = got == bytes(want)
    print(f"{'OK ' if ok else 'FAIL'} {tag:22s} {rva:#9x}  want={bytes(want).hex()}  got={got.hex() if got else None}")
    if not ok: fail += 1

def const(name, src=None):
    m = re.search(r'const %s: usize = (0x[0-9a-f]+)' % name, src or S)
    return int(m.group(1), 16)
def arr(name, src=None):
    m = re.search(r'const %s: &\[u8\] = &\[(.*?)\];' % name, src or S, re.S)
    return [int(x, 16) for x in re.findall(r'0x([0-9a-fA-F]{2})', m.group(1))]
def arrN(name, src=None):
    m = re.search(r'const %s: \[u8; \d+\] = \[(.*?)\];' % name, src or S, re.S)
    return [int(x, 16) for x in re.findall(r'0x([0-9a-fA-F]{2})', m.group(1))]

print("== 함수시작 프롤로그 ==")
chk('A PHASE_SCENE', const('RVA_PHASE_SCENE'), arr('PROLOGUE_SCENE'))
chk('B PHASE_SCALAR', const('RVA_PHASE_SCALAR'), arr('PROLOGUE_SCALAR'))
chk('C APPLIER', const('RVA_APPLIER'), arr('PROLOGUE_APPLIER'))
chk('O SLOTUPD', const('RVA_SLOTUPD'), arr('PROLOGUE_SLOTUPD'))
chk('N PHASE_RAW', const('RVA_PHASE_RAW'), arr('PROLOGUE_PHASE_RAW'))
chk('E LINEUP', const('RVA_LINEUP'), arr('PROLOGUE_LINEUP'))
chk('F COMMIT', const('RVA_COMMIT'), arr('PROLOGUE_COMMIT'))
chk('TRIGGER', const('RVA_TRIGGER'), arrN('PROLOGUE_TRIGGER'))
m = re.search(r'const RVA_PANIC_HOOK: usize = (0x[0-9a-f]+)', D)
chk('PANIC(13B)', int(m.group(1),16), bytes.fromhex('55415656575348 81ec80000000'.replace(' ','')))
# D' TURN: install_turn 이 13B 훔침 (진입 `mov rax,[rcx+0x10]; test; jz rel32`)
chk('D TURN(13B)', const('RVA_TURN'), bytes.fromhex('488b41104885c00f84d2010000'))

print("\n== 바이트패치 사이트 SIG ==")
chk('AI_SIG1@SITE1', const('RVA_AI_SITE1'), arr('AI_SIG1'))
chk('AI_SIG2@SITE2', const('RVA_AI_SITE2'), arr('AI_SIG2'))
chk('AITURN_SIG', const('RVA_AITURN_SITE'), arr('AITURN_SIG'))
chk('SFX_SIG', const('RVA_SFX_SITE'), arr('SFX_SIG'))
# 테이블
def tuples(name, pat):
    m = re.search(pat, S, re.S); return m
for tag, rx in [
  ('AI6', r'const AI6: .*?\n(.*?)\n\];'),
]:
    body = re.search(rx, S, re.S).group(1)
    for line in body.splitlines():
        mm = re.match(r'\s*\((0x[0-9a-f]+), (0x[0-9a-f]+), &\[([^\]]*)\]', line)
        if mm:
            site = int(mm.group(1),16); sig=[int(x,16) for x in re.findall(r'0x([0-9a-fA-F]{2})', mm.group(3))]
            chk('AI6 site', site, sig)
hl = re.search(r'const HL: .*?=\s*\((.*?)\n\);', S, re.S).group(1)
mm = re.search(r'(0x[0-9a-f]+), (0x[0-9a-f]+),\s*&\[([^\]]*)\]', hl, re.S)
chk('HL site', int(mm.group(1),16), [int(x,16) for x in re.findall(r'0x([0-9a-fA-F]{2})', mm.group(3))])
body = re.search(r'const DRAIN_HL: .*?\n(.*?)\n\];', S, re.S).group(1)
for line in body.splitlines():
    mm = re.match(r'\s*\((0x[0-9a-f]+), (0x[0-9a-f]+), &\[([^\]]*)\]', line)
    if mm: chk('DRAIN_HL site', int(mm.group(1),16), [int(x,16) for x in re.findall(r'0x([0-9a-fA-F]{2})', mm.group(3))])
body = re.search(r'const DRAIN_HL2: .*?\n(.*?)\n\];', S, re.S).group(1)
for line in body.splitlines():
    mm = re.match(r'\s*\((0x[0-9a-f]+), (0x[0-9a-f]+), (0x[0-9a-f]+), &\[([^\]]*)\]', line)
    if mm: chk('DRAIN_HL2 site', int(mm.group(1),16), [int(x,16) for x in re.findall(r'0x([0-9a-fA-F]{2})', mm.group(4))])
sl = re.search(r'const SLOTSEL: .*?=\s*\((.*?)\n\);', S, re.S).group(1)
mm = re.search(r'(0x[0-9a-f]+), (0x[0-9a-f]+), (0x[0-9a-f]+), (0x[0-9a-f]+),\s*&\[([^\]]*)\]', sl, re.S)
chk('SLOTSEL site', int(mm.group(1),16), [int(x,16) for x in re.findall(r'0x([0-9a-fA-F]{2})', mm.group(5))])
hc = re.search(r'const HL_COUNT: .*?=\s*\((.*?)\n\);', S, re.S).group(1)
mm = re.search(r'(0x[0-9a-f]+), (0x[0-9a-f]+),\s*&\[([^\]]*)\]', hc, re.S)
chk('HL_COUNT site', int(mm.group(1),16), [int(x,16) for x in re.findall(r'0x([0-9a-fA-F]{2})', mm.group(3))])

print("\n== emit 본문 리터럴 <-> exe 짝 점검 ==")
# sfx emit: mov rcx,[rbp+0x12f0]
em = re.search(r'emit!\(\[0x48u8, 0x8b, 0x8d, (0x[0-9a-f]{2}), (0x[0-9a-f]{2})', S)
emit_disp = int(em.group(2),16)*256 + int(em.group(1),16)
sig_disp = arr('SFX_SIG')[3] | (arr('SFX_SIG')[4]<<8)
print(f"{'OK ' if emit_disp==sig_disp else 'FAIL'} sfx scene slot emit={emit_disp:#x} sig={sig_disp:#x}")
if emit_disp!=sig_disp: fail+=1
# aiturn emit: total/rule/ban 슬롯
for tag, pat, want in [
  ('aiturn total', r'p\[0\.\.7\]\.copy_from_slice\(&\[0x48, 0x8b, 0x8d, (0x[0-9a-f]{2}), (0x[0-9a-f]{2})', 0x5eb0),
  ('aiturn rule',  r'p\[7\.\.14\]\.copy_from_slice\(&\[0x0f, 0xb6, 0x95, (0x[0-9a-f]{2}), (0x[0-9a-f]{2})', 0x5d61),
  ('aiturn ban',   r'p\[14\.\.21\]\.copy_from_slice\(&\[0x4c, 0x8b, 0x85, (0x[0-9a-f]{2}), (0x[0-9a-f]{2})', 0x5d58)]:
    mm = re.search(pat, S)
    v = int(mm.group(2),16)*256+int(mm.group(1),16)
    print(f"{'OK ' if v==want else 'FAIL'} {tag} emit={v:#x} exe={want:#x}")
    if v!=want: fail+=1
print("\nFAILS =", fail)
