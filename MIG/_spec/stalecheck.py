# -*- coding: utf-8 -*-
u"""stalecheck — 2026-09-11 반증검증 13건이 **본 표에 반영됐는지** 확인.

배경: 정정을 `resolved[]` 의 was/now 로만 append 하면, `reads`/`writes`/`signature`/
`constants`/`knobs`/`unknown` 을 **기계로 소비하는 쪽은 옛 값을 계속 읽는다**(§7 정정형 기록 위반).
tcxaudit 은 base+offset 만 보므로 이 오염을 못 잡는다 — 그래서 이 검사가 따로 필요하다.

판정: STALE = 반증된 문자열이 본 표에 아직 살아 있다 / OK = 없다(또는 취소선 처리됨)
"""
import io, json, sys, re

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
D = json.load(io.open("_spec/specs20.json", encoding="utf-8"))
S = D["specs"]

# 본 표 = resolved(정정 로그) 를 뺀 나머지. still_unknown 도 서술이라 포함한다.
LIVE = ("signature", "logic", "reads", "writes", "constants", "calls",
        "knobs", "new_knobs", "unknown", "still_unknown", "one_line", "exe")
STRIKE = re.compile(r"~~(?!~).+?~~")


def live_text(sp):
    t = json.dumps({k: sp.get(k) for k in LIVE}, ensure_ascii=False)
    return STRIKE.sub(u" ", t)          # 취소선 = 이미 정정 표기된 것 → 오탐 제외


# (함수 index, 라벨, 본 표에 남아 있으면 안 되는 문자열들, 정정 후 기대값)
CHECKS = [
    (1,  u"01 is_jungle 티어컷 노브(존재하지 않음)",
     [u"정글 캠프 티어 컷", u"camp_type.__0 < 2", u"camp_type.0<2"], u"3항목 삭제"),
    (1,  u"01 −10 경로 술어 이름",
     [u"entity.rs:1256"], u"is_any_type_minion(entity.rs:1260)"),
    (3,  u"03 judge_accuracy 오귀속",
     [u"judge_accuracy`(PlayerState+0x180)", u"judge_accuracy(PlayerState+0x180)"],
     u"+0x180 = info.parameter:AthleteParameter, judge_accuracy 는 그 위 pub 메서드"),
    (3,  u"03 judge_accuracy 치역",
     [u'"value": "0~1000"', u'"value": "0~1000 '], u"100~1000(9 간격 이산)"),
    (8,  u"08 live_list 접근자",
     [u"live_list.first()"], u"get(0)"),
    (9,  u"09 유일 호출처(거짓)",
     [u"유일 호출처"], u"호출부 8곳 · 7곳이 런타임 version"),
    (13, u"13 is_top_side 극성",
     [u"본문식 = (ctx.setting.height"], u"IR 술어 = !is_top_side"),
    (14, u"14 is_top_side 극성",
     [u"결과 = (ry < champ.x)", u"결과 = ry < champ.x", u"if is_top_side {"],
     u"is_top_side = !(ry<x) = (x+y<=height)"),
    (14, u"14 target_bush_v41 누락",
     [], u"new_knobs 에 v41 전표 + *_lead 3필드 추가됨(수동 확인)"),
    (15, u"15 team_plan &mut 전달(거짓)",
     [u"&mut self.team_plan /*", u"만 만짐. team_plan 은 &mut"],
     u"전 계열이 공유 &TeamPlan"),
    (17, u"17 main_objective 타입",
     [u"Option<Option<MainObjective>>"], u"Option<MainObjective>(3B) 단일"),
    (17, u"17 flee_die 센티널 이름",
     [u"usize::MAX"], u"i64::MAX"),
    (18, u"18 repair_need 시그니처(argpromotion)",
     [u"v3_epicops_repair_need(team", u"fn(team, cache, plan)", u"v3_group_press_line(team"],
     u"fn(&PlayerState,&OperationData,&BigPlan)"),
    (18, u"18 Chat::Press 소스 줄",
     [u'"src_line": 682', u"682줄의 is_none"], u"674/676 · is_none=673"),
]

bad = 0
print(u"\n" + u"=" * 100)
print(u"## 반증검증 13건 — 본 표 반영 여부 (게임 0.5.8 / 2026-09-11)")
print(u"=" * 100)
for idx, label, pats, want in CHECKS:
    sp = S[idx]
    t = live_text(sp)
    hits = [p for p in pats if p in t]
    if not pats:
        print(u"\n[수동] %-44s  %s" % (label, want))
        continue
    if hits:
        bad += 1
        print(u"\n★STALE  %-44s" % label)
        print(u"        본 표에 살아 있음: %s" % u" / ".join(hits))
        print(u"        정정 후 기대값   : %s" % want)
    else:
        print(u"\n  OK    %-44s" % label)
print(u"\n" + u"=" * 100)
print(u"STALE %d건 / 자동검사 %d건" % (bad, len([c for c in CHECKS if c[2]])))
