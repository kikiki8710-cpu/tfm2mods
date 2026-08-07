import io,sys
sys.stdout.reconfigure(encoding="utf-8")
P="src/main.rs"; s=io.open(P,encoding="utf-8").read()
R=[
# ★RE로 의미가 바뀐 키 — 설명이 옛것 그대로였다
('"oi_dn_lane_margin" => "[oi] 레인 진척 허용 마진. ↑=더 밀려도 수비 유지. 원본 120",',
 '"oi_dn_lane_margin" => "적을 본 기억이 남는 시간(틱, 약 4초). ↑하면 오래된 목격도 위협으로 쳐서 넥서스 방어가 더 자주 걸립니다. 이름과 달리 레인 진척과는 무관합니다",'),
]
n=0
for a,b in R:
    if a in s: s=s.replace(a,b,1); n+=1
    else: print("못찾음:", a[:50])
# 원본값이 비어 있던 실제 노브 4종 (근거: RE 표 / detour 주석의 근사값)
ANCH = '    // ── detour.rs 에서 자동 회수(2026-08-04 감사) ──\n'
ADD = ('    "d19_retreat_hp" => "46",\n'
       '    "gb_close_radius" => "387",\n'
       '    "gb_line_range" => "500",\n'
       '    "gb_reach_cap" => "140052",\n')
assert ANCH in s
s = s.replace(ANCH, ANCH + ADD, 1)
io.open(P,"w",encoding="utf-8").write(s)
print("설명 %d건 정정 + 원본값 4키 추가" % n)
