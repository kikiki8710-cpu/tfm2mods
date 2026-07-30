import os
path = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\bundle.game_data"
anchor = 886365277           # 'game_time:color' 히트
START = b'ingame:ingame_ui {'
lo = max(0, anchor - 3_000_000)
hi = anchor + 3_000_000
with open(path,'rb') as f:
    f.seek(lo); blob = f.read(hi-lo)
i = blob.rfind(START, 0, anchor-lo)
print("start marker rel:", i, "abs:", lo+i if i>=0 else None)
if i < 0: raise SystemExit("start not found")

# .ui 는 중괄호 균형으로 끝을 찾는다 (문자열/주석 없다고 가정 — 이 파일엔 // 주석 0개)
depth = 0
end = None
for j in range(i, len(blob)):
    c = blob[j]
    if c == 0x7b: depth += 1
    elif c == 0x7d:
        depth -= 1
        if depth == 0:
            end = j+1
            break
    elif c > 0x7e and c < 0xc0 and depth == 0:
        break
print("end rel:", end, "len:", (end-i) if end else None)
text = blob[i:end]
out = r"C:\Users\dev\AppData\Local\Temp\claude\C--Users-dev-Desktop-claude-tfm2\45bdc6c4-7896-4de1-9dfa-643d6e6b96e0\scratchpad\base_ingame_050.ui"
open(out,'wb').write(text)
print("wrote", out, len(text), "bytes,", text.count(b'\n')+1, "lines")
print("--- tail 200B ---")
print(text[-200:].decode('utf-8','replace'))
