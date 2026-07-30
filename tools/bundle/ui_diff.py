import difflib, io, re, sys
sp = r"C:\Users\dev\AppData\Local\Temp\claude\C--Users-dev-Desktop-claude-tfm2\45bdc6c4-7896-4de1-9dfa-643d6e6b96e0\scratchpad"
ovr_p = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\Spectator_Chat\ui\layout\ingame.ui"
base_p = sp + r"\base_ingame_050.ui"

ovr = open(ovr_p, encoding='utf-8').read().splitlines()
base = open(base_p, encoding='utf-8').read().splitlines()

# override 에서 우리가 덧붙인 #spectator_chat 블록을 중괄호 균형으로 잘라낸다
start = next(i for i,l in enumerate(ovr) if '#spectator_chat' in l)
depth = 0; end = None
for j in range(start, len(ovr)):
    depth += ovr[j].count('{') - ovr[j].count('}')
    if depth == 0 and j > start:
        end = j; break
print(f"[*] override 의 #spectator_chat 블록 = L{start+1}..L{end+1} ({end-start+1} 줄)")
ovr_stripped = ovr[:start] + ovr[end+1:]
# 빈 줄 정리해서 base 와 비교
def norm(ls): return [l.rstrip() for l in ls if l.strip()]
a, b = norm(ovr_stripped), norm(base)
print(f"[*] override(패널제거)={len(a)}줄  base0.5.0={len(b)}줄")

sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
adds = dels = 0
print("\n=== base 0.5.0 에만 있는 것 (override 가 덮어써서 사라질 내용) ===")
for tag,i1,i2,j1,j2 in sm.get_opcodes():
    if tag in ('insert','replace'):
        for l in b[j1:j2]:
            adds += 1
            if adds <= 60: print("  + " + l)
print(f"  ... 총 {adds} 줄")
print("\n=== override 에만 있는 것 (0.4.14 잔재) ===")
for tag,i1,i2,j1,j2 in sm.get_opcodes():
    if tag in ('delete','replace'):
        for l in a[i1:i2]:
            dels += 1
            if dels <= 40: print("  - " + l)
print(f"  ... 총 {dels} 줄")
