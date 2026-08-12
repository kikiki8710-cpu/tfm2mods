# -*- coding: utf-8 -*-
"""갱신된 소스 상수를 0.5.5 exe 로 전수 검증(시그<->emit 짝 포함). bo_verify54 이식."""
import io, re
from bo_055 import N
S = io.open(r'C:\tfm2mods\tfm2_banpick_order\src\hooks.rs', encoding='utf-8').read()
D = io.open(r'C:\tfm2mods\tfm2_banpick_order\src\diag.rs', encoding='utf-8').read()
fail=0
def chk(tag, rva, want):
    global fail
    got=N.read(rva, len(want)); ok=got==bytes(want)
    print(f"{'OK ' if ok else 'FAIL'} {tag:22s} {rva:#9x}  want={bytes(want).hex()}  got={got.hex() if got else None}")
    if not ok: fail+=1
def const(name, src=None):
    m=re.search(r'const %s: usize = (0x[0-9a-f]+)'%name, src or S); return int(m.group(1),16)
def arr(name, src=None):
    m=re.search(r'const %s: &\[u8\] = &\[(.*?)\];'%name, src or S, re.S)
    return [int(x,16) for x in re.findall(r'0x([0-9a-fA-F]{2})', m.group(1))]
def arrN(name, src=None):
    m=re.search(r'const %s: \[u8; \d+\] = \[(.*?)\];'%name, src or S, re.S)
    return [int(x,16) for x in re.findall(r'0x([0-9a-fA-F]{2})', m.group(1))]

print("== 함수시작 프롤로그 ==")
chk('A PHASE_SCENE', const('RVA_PHASE_SCENE'), arr('PROLOGUE_SCENE'))
chk('B PHASE_SCALAR', const('RVA_PHASE_SCALAR'), arr('PROLOGUE_SCALAR'))
chk('C APPLIER', const('RVA_APPLIER'), arr('PROLOGUE_APPLIER'))
chk('O SLOTUPD', const('RVA_SLOTUPD'), arr('PROLOGUE_SLOTUPD'))
chk('N PHASE_RAW', const('RVA_PHASE_RAW'), arr('PROLOGUE_PHASE_RAW'))
chk('E LINEUP', const('RVA_LINEUP'), arr('PROLOGUE_LINEUP'))
chk('F COMMIT', const('RVA_COMMIT'), arr('PROLOGUE_COMMIT'))
chk('TRIGGER', const('RVA_TRIGGER'), arrN('PROLOGUE_TRIGGER'))
m=re.search(r'const RVA_PANIC_HOOK: usize = (0x[0-9a-f]+)', D)
chk('PANIC(13B)', int(m.group(1),16), bytes.fromhex('5541565657534881ec80000000'))
chk('D TURN(13B)', const('RVA_TURN'), bytes.fromhex('488b41104885c00f84d2010000'))

print("\n== 함수시작 호출주소 검증(단순 존재) ==")
for nm in ['RVA_APP_PICK_T1','RVA_APP_PICK_T2','RVA_APP_BAN_T1','RVA_APP_BAN_T2','RVA_TRANSITION','RVA_BANNER']:
    r=const(nm); b=N.read(r,1)
    print(f"{'OK ' if b else 'FAIL'} {nm:22s} {r:#9x}  first={b.hex() if b else None}")
    if not b: fail+=1

print("\n== 바이트패치 사이트 SIG ==")
chk('AI_SIG1@SITE1', const('RVA_AI_SITE1'), arr('AI_SIG1'))
chk('AI_SIG2@SITE2', const('RVA_AI_SITE2'), arr('AI_SIG2'))
chk('AITURN_SIG', const('RVA_AITURN_SITE'), arr('AITURN_SIG'))
chk('SFX_SIG', const('RVA_SFX_SITE'), arr('SFX_SIG'))
# STR
chk('STR_BAN', const('RVA_STR_BAN'), b'asset/base/sound/sfx/ban_sfx')
chk('STR_PICK', const('RVA_STR_PICK'), b'asset/base/sound/sfx/pick_sfx')
# 테이블 SIG
body=re.search(r'const AI6: .*?\n(.*?)\n\];', S, re.S).group(1)
for line in body.splitlines():
    mm=re.match(r'\s*\((0x[0-9a-f]+), (0x[0-9a-f]+), &\[([^\]]*)\]', line)
    if mm:
        site=int(mm.group(1),16); sig=[int(x,16) for x in re.findall(r'0x([0-9a-fA-F]{2})', mm.group(3))]
        chk('AI6 site', site, sig)
hl=re.search(r'const HL: .*?=\s*\((.*?)\n\);', S, re.S).group(1)
mm=re.search(r'(0x[0-9a-f]+), (0x[0-9a-f]+),\s*&\[([^\]]*)\]', hl, re.S)
chk('HL site', int(mm.group(1),16), [int(x,16) for x in re.findall(r'0x([0-9a-fA-F]{2})', mm.group(3))])
body=re.search(r'const DRAIN_HL: .*?\n(.*?)\n\];', S, re.S).group(1)
for line in body.splitlines():
    mm=re.match(r'\s*\((0x[0-9a-f]+), (0x[0-9a-f]+), &\[([^\]]*)\]', line)
    if mm: chk('DRAIN_HL site', int(mm.group(1),16), [int(x,16) for x in re.findall(r'0x([0-9a-fA-F]{2})', mm.group(3))])
body=re.search(r'const DRAIN_HL2: .*?\n(.*?)\n\];', S, re.S).group(1)
for line in body.splitlines():
    mm=re.match(r'\s*\((0x[0-9a-f]+), (0x[0-9a-f]+), (0x[0-9a-f]+), &\[([^\]]*)\]', line)
    if mm: chk('DRAIN_HL2 site', int(mm.group(1),16), [int(x,16) for x in re.findall(r'0x([0-9a-fA-F]{2})', mm.group(4))])
sl=re.search(r'const SLOTSEL: .*?=\s*\((.*?)\n\);', S, re.S).group(1)
mm=re.search(r'(0x[0-9a-f]+), (0x[0-9a-f]+), (0x[0-9a-f]+), (0x[0-9a-f]+),\s*&\[([^\]]*)\]', sl, re.S)
chk('SLOTSEL site', int(mm.group(1),16), [int(x,16) for x in re.findall(r'0x([0-9a-fA-F]{2})', mm.group(5))])
hc=re.search(r'const HL_COUNT: .*?=\s*\((.*?)\n\);', S, re.S).group(1)
mm=re.search(r'(0x[0-9a-f]+), (0x[0-9a-f]+),\s*&\[([^\]]*)\]', hc, re.S)
chk('HL_COUNT site', int(mm.group(1),16), [int(x,16) for x in re.findall(r'0x([0-9a-fA-F]{2})', mm.group(3))])

print("\n== emit 슬롯 <-> exe 짝 ==")
em=re.search(r'emit!\(\[0x48u8, 0x8b, 0x8d, (0x[0-9a-f]{2}), (0x[0-9a-f]{2})', S)
ed=int(em.group(2),16)*256+int(em.group(1),16); sd=arr('SFX_SIG')[3]|(arr('SFX_SIG')[4]<<8)
print(f"{'OK ' if ed==sd else 'FAIL'} sfx scene slot emit={ed:#x} sig={sd:#x}"); fail+= (ed!=sd)
for tag,pat,want in [
  ('aiturn total', r'p\[0\.\.7\]\.copy_from_slice\(&\[0x48, 0x8b, 0x8d, (0x[0-9a-f]{2}), (0x[0-9a-f]{2})', 0x6040),
  ('aiturn rule',  r'p\[7\.\.14\]\.copy_from_slice\(&\[0x0f, 0xb6, 0x95, (0x[0-9a-f]{2}), (0x[0-9a-f]{2})', 0x5ef1),
  ('aiturn ban',   r'p\[14\.\.21\]\.copy_from_slice\(&\[0x4c, 0x8b, 0x85, (0x[0-9a-f]{2}), (0x[0-9a-f]{2})', 0x5ee8)]:
    mm=re.search(pat,S); v=int(mm.group(2),16)*256+int(mm.group(1),16)
    print(f"{'OK ' if v==want else 'FAIL'} {tag} emit={v:#x} exe={want:#x}"); fail+=(v!=want)
# AITURN join calls TURN oracle (이중확증)
jt=const('RVA_TURN'); site=const('RVA_AITURN_JOIN')
# join+? call rel32 -> compute
b=N.read(site,16)
print("\nFAILS =", fail)
