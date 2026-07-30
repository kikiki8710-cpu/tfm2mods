import io
sp = r"C:\Users\dev\AppData\Local\Temp\claude\C--Users-dev-Desktop-claude-tfm2\45bdc6c4-7896-4de1-9dfa-643d6e6b96e0\scratchpad"
ovr_p = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\Spectator_Chat\ui\layout\ingame.ui"
base_p = sp + r"\base_ingame_050.ui"
out_p  = sp + r"\ingame_050_merged.ui"

ovr = open(ovr_p, encoding='utf-8').read().split('\n')
base = open(base_p, encoding='utf-8').read()

# 1) override 에서 #spectator_chat 블록 추출 (중괄호 균형)
start = next(i for i,l in enumerate(ovr) if '#spectator_chat' in l)
depth = 0; end = None
for j in range(start, len(ovr)):
    depth += ovr[j].count('{') - ovr[j].count('}')
    if depth == 0 and j > start:
        end = j; break
panel = ovr[start:end+1]
assert '#spectator_chat' in panel[0] and panel[-1].strip() == '}'
print(f"[*] 패널 {len(panel)}줄 추출 (override L{start+1}..L{end+1})")

# 2) base 0.5.0 의 마지막 루트 '}' 를 찾아 그 앞에 삽입
k = base.rstrip().rfind('}')
assert base.rstrip()[k:].strip() == '}', "루트 닫는 중괄호 탐색 실패"
head = base.rstrip()[:k].rstrip('\n')
merged = head + '\n\n' + '\n'.join(panel) + '\n}\n'

# 3) 무결성 체크
assert merged.count('{') == merged.count('}'), "중괄호 불균형"
assert '//' not in merged, "// 주석 발견 (파서 크래시 유발)"
assert '#spectator_chat' in merged and '#dm_center' in merged and '#game_time' in merged

# 4) UTF-8 no BOM 으로 기록
with open(out_p, 'wb') as f:
    f.write(merged.encode('utf-8'))
raw = open(out_p,'rb').read()
print(f"[*] wrote {out_p}: {len(raw)}B, {merged.count(chr(10))} lines")
print(f"[*] 첫 3바이트 = {raw[:3].hex()}  (efbbbf 면 BOM=불량)")
print(f"[*] 중괄호 {merged.count('{')} 쌍 / '//' 개수 {merged.count('//')}")
for tok in ['#dm_center','#overtime','#count','#game_time','#spectator_chat','draggable_popup']:
    print(f"    {tok:<18} {merged.count(tok)}")
