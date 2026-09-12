# -*- coding: utf-8 -*-
u"""fixneedles — `closelist.py` 의 **죽은 needle 10건**을 실제 문면으로 교정. (2026-09-11, 4차 반영)

4차 배치 C 가 전수 확인해 적발했다 — index 오기 4 + 문면 불일치 6.
문면 불일치의 원인은 대부분 **백틱·어휘 차이**였다:
  `Blackboard 배열을`  vs  실제 `` `Blackboard` 배열을 ``   (백틱)
  `별개 클로저`        vs  실제 `두 개의 별도 클로저인지`   (별개/별도)
⟹ 교훈: **needle 은 원문에서 복사해 오고, 감사기로 매번 확인한다.**
"""
import io, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
P = "_spec/closelist.py"
s = io.open(P, encoding="utf-8").read()

# (옛 (index, needle) → 새 (index, needle))  — 새 needle 은 v3 원문에서 복사했다
FIX = [
    # ── index 오기 4건 ─────────────────────────────────────────────
    ((13, u"bushes"), (14, u"MapDef.bushes 배열의 값 사전"),
     u"index 오기 13→14. ★이것이 /specs[14]/open[0] 이 4차까지 열려 있던 직접 원인"),
    ((18, u"Prepare"), (17, u"Prepare"), u"index 오기 18→17"),
    ((8, u"is_recent_visible 본체"), (8, u"is_recent_visible 의 판정 조건"),
     u"문면 — 08 은 '본체'가 아니라 '판정 조건'으로 적혀 있다"),
    ((8, u"15"), (8, u"constants 에 넣지 않고 knobs 에만"),
     u"★2글자 needle 은 잠복 오닫힘 지뢰였다(다른 6개 index 에서 매칭). 원문 문구로 교체"),
    # ── 문면 불일치 6건 ────────────────────────────────────────────
    ((2, u"buf.a"), (2, u"앞 24B(ptr/ptr/cap)"), u"문면"),
    ((3, u"check_kill_die_tick 반환값"), (3, u"반환값 die 가"), u"문면"),
    ((4, u"Blackboard 배열을"), (4, u"배열을 **적팀 인덱스"), u"문면 — 백틱 때문에 안 맞았다"),
    ((6, u"별개 클로저"), (6, u"두 개의 별도 클로저인지"), u"문면 — 별개/별도"),
    ((12, u"position_exists"), (12, u"chat_allowed(GameContext"),
     u"대상이 없어졌다 — 3차가 297칸 전수 진리표로 닫은 chat_allowed 항목으로 재조준"),
    ((19, u"SliceRandom"), (19, u"map 클로저(s_0)"),
     u"대상이 없어졌다 — 19 의 남은 '사실 서술' 항목으로 재조준"),
]

n = 0
for (oi, on), (ni, nn), why in FIX:
    # (index, u"needle" 형태를 찾아 교체 — 여러 줄에 걸친 항목도 있으므로 needle 문자열로 찾는다
    pat = u'(%d, u"%s"' % (oi, on)
    if pat in s:
        s = s.replace(pat, u'(%d, u"%s"' % (ni, nn), 1)
        n += 1
        print(u"  OK (%d,'%s') -> (%d,'%s')   %s" % (oi, on[:26], ni, nn[:34], why))
    else:
        print(u"  ★못 찾음: %s" % pat[:60])

io.open(P, "w", encoding="utf-8").write(s)
print(u"\n%d/%d 교정 -> %s" % (n, len(FIX), P))
