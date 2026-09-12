# -*- coding: utf-8 -*-
u"""제출 예정 변경을 v3 사본에 얹어 G13/G19/G15 가 어떻게 반응하는지 미리 본다."""
import io, json, copy, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import knobval as KV
import kindchk as KC

D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
S = D["specs"] if isinstance(D, dict) else D

NEW_WHERE_08_4 = (u"★담당 범위 밖 — 술어 본체 = `game_core::Blackboard::is_recent_visible`"
                  u"(blackboard.rs:346 · `define` = `_gcbc/g07.ll:157005`). 08 에서의 호출 지점은 "
                  u"m10.ll:6130(모노모피된 min_by_key 본체 m10.ll 6028~6196). "
                  u"시야 기억창 리터럴은 그 본체 안 `%25 = add i64 %24, 120` (g07.ll:157041)")


def run(i, sp, tag):
    print(u"--- %s ---" % tag)
    rows = KV.check_spec(sp)
    for j, why, det in rows or []:
        print(u"  G19 knobs[%s] %s | %s" % (j, why, str(det)[:90]))
    rows = KC.check_spec(sp)
    for r in rows or []:
        print(u"  G15 %s" % (str(r)[:130],))
    if not (KV.check_spec(sp) or KC.check_spec(sp)):
        print(u"  (깨끗)")


for i in (6, 8):
    run(i, S[i], u"현행 %02d" % i)

# 시험 ①: 08 knobs[4] value 0 -> 120 + where 에 인용 추가
t8 = copy.deepcopy(S[8])
t8["knobs"][4]["value"] = 120
t8["knobs"][4]["where"] = NEW_WHERE_08_4
run(8, t8, u"시험 08 knobs[4] = 120")

# 시험 ②: 06 에 knobs 행 추가(시야 기억창)
t6 = copy.deepcopy(S[6])
t6["knobs"].append({
    "what": u"시야 기억창(최근 목격으로 치는 틱 수)",
    "where": NEW_WHERE_08_4.replace(u"08 에서의 호출 지점은 m10.ll:6130"
                                    u"(모노모피된 min_by_key 본체 m10.ll 6028~6196). ", u""),
    "value": 120,
    "effect": u"테스트",
    "ev": 4, "src": u"신규"})
run(6, t6, u"시험 06 knobs 추가")
