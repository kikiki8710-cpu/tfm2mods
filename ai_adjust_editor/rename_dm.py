# "죽음 교전" → "데스매치" 로 통일 + 일반 경기에서 안 뜬다는 사실 명시.
#  근거: 게임에 '데스매치' 모드가 실제로 있고(유저 확인 2026-08-04), 이 판단은 그 모드 전용이다.
#  판단 이름표도 DeathMatchBattle → 소스 파일 death_battle.rs 로 이어진다.
import io, re, sys
sys.stdout.reconfigure(encoding="utf-8")

P = "src/main.rs"
s = io.open(P, encoding="utf-8").read()
n = 0

# ── ① 섹션 제목·안내문의 이름 교체 ──
for a, b in [
    ("죽음 교전 — 언제 닿는다고 볼지", "데스매치 — 언제 닿는다고 볼지"),
    ("죽음 교전 — 무엇을 노릴지",       "데스매치 — 무엇을 노릴지"),
    ("죽음 교전 — 궁 사용 조건",        "데스매치 — 궁 사용 조건"),
    ("죽음 교전 — 해금 레벨",           "데스매치 — 해금 레벨"),
    ("죽음 교전의 전투 행동 만들기",     "데스매치의 전투 행동 만들기"),
    ("죽음 교전", "데스매치"), ("죽음교전", "데스매치"),
]:
    if a in s: c = s.count(a); s = s.replace(a, b); n += c

# ── ② db_* 설명 앞에 "일반 경기에서는 안 뜬다"를 박아 넣는다 ──
#   (개별 설명마다 붙이면 장황하니, 설명 앞머리에 한 줄 표식만)
MARK = "【데스매치 모드 전용 — 일반 경기에서는 발동하지 않습니다】 "
i = s.index("fn desc_static("); j = s.index("\n}", i)
body = s[i:j]
cnt = 0
for m in list(re.finditer(r'("db_[a-z0-9_]+" => ")', body)):
    pass
def add_mark(mo):
    global cnt
    head = mo.group(0)
    if body[mo.end():mo.end() + 1] == "【": return head
    cnt += 1
    return head + MARK
body = re.sub(r'"db_[a-z0-9_]+" => "', add_mark, body)
s = s[:i] + body + s[j:]

io.open(P, "w", encoding="utf-8").write(s)
print("이름 교체 %d곳 · db_* 설명에 모드 표식 %d개" % (n, cnt))
