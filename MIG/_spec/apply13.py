# -*- coding: utf-8 -*-
u"""apply13 — 2026-09-11 반증검증 정정을 specs20.json **본 표에 §7 정정형으로 반영**.

배경: 정정이 resolved[] 의 was/now 로그에만 들어가 있고 reads/writes/signature/
constants/knobs/unknown 은 반증된 옛 값 그대로였다(stalecheck.py 로 10건 적발).
로그만 고치면 표를 기계로 소비하는 쪽이 계속 옛 값을 읽는다.
⚠tcxaudit 은 base+offset 만 보므로 이 오염을 못 잡는다 — 그래서 이 반영이 따로 필요하다.

원칙: 지우지 않고 `~~구~~ → 신` 으로 남긴다(§7). 값 필드는 신값으로 교체하되
      note/effect 에 옛 값을 취소선으로 보존한다.
"""
import io, json, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
P = "_spec/specs20.json"
D = json.load(io.open(P, encoding="utf-8"))
S = D["specs"]
N = [0]


def setv(obj, key, val, label):
    assert obj.get(key) != val, label
    obj[key] = val
    N[0] += 1
    print(u"  OK %s" % label)


def setl(lst, i, val, label):
    assert lst[i] != val, label
    lst[i] = val
    N[0] += 1
    print(u"  OK %s" % label)


def rep(obj, key, old, new, label):
    s = obj[key]
    assert old in s, u"못 찾음: %s" % label
    obj[key] = s.replace(old, new)
    N[0] += 1
    print(u"  OK %s" % label)


# -- 01 calculate_jungle_action_score --------------------------------
print(u"\n[01] calculate_jungle_action_score")
nk = S[1]["new_knobs"][0]
setv(nk, "what", u"정글 캠프의 **소유 팀** 게이트", u"01 new_knobs[0].what")
setv(nk, "where",
     u"action_score.rs:530 · 542 — 소스는 `t.ty.is_jungle(0) || t.ty.is_jungle(1)`"
     u"(줄길이 52자/57자 ±0 검산). IR `icmp ult camp_type.__0, 2`(m05.ll:40059~40063)는 "
     u"**그 OR 의 LLVM 접힘**이지 소스 임계가 아니다",
     u"01 new_knobs[0].where")
setv(nk, "effect",
     u"`is_jungle(player.info.team)` 으로 바꾸면 **아군 진영 캠프만**, `is_jungle(1 - team)` 이면 "
     u"**카운터정글 전용**이 된다(camp_type.__0 = 팀 인덱스). 정글 스코어 가산 +5(m05.ll:40092)도 같이 따라간다. "
     u"WARN ~~「camp_type.__0 < 2 = 정글 캠프 티어 컷」~~ 은 **거짓**(2026-09-11 검증배치 A) — "
     u"`EntityType::is_jungle(&self, usize)`(entity.rs:1377)는 **인자와의 동등 비교**이고 상수 2 는 소스에 없다. "
     u"「죽은 가드」도 아니다",
     u"01 new_knobs[0].effect")

# -- 03 defensive_crisis ---------------------------------------------
print(u"\n[03] defensive_crisis")
nk = S[3]["new_knobs"][1]
setv(nk, "where",
     u"`PlayerState+0x180` = `info.parameter : AthleteParameter`(744B) 의 **시작 주소**이고, "
     u"`judge_accuracy` 는 그 위의 **pub 메서드**(player.rs:337, 본체 `_gcbc/g15.ll:125794`)다 — "
     u"WARN ~~`judge_accuracy(PlayerState+0x180)` 필드~~ 가 아니다"
     u"(+0x180 에서 u64 를 읽으면 `Vec<String>` 포인터를 읽는다). "
     u"호출 = m15.ll:31277~31279; range_min=1000-(1000-acc)/2 @m15.ll:31295~31297; 폭=(1000-acc)|1 @m15.ll:31344",
     u"03 new_knobs[1].where")
setv(nk, "value", u"100~1000 (9 간격 이산 — ~~0~1000~~ 은 도달 불가)", u"03 new_knobs[1].value")
setv(nk, "effect",
     u"1000 이면 데미지 추정 오차 0(항상 x1.000). **최악값이 100** 이라 x0.550~1.450 까지만 벌어진다 "
     u"(~~0 이면 x0.500~1.500~~ 은 **도달 불가**). 실제 계산 = "
     u"`x = umin(judgement_mental_ratio * judgement / 1000, 100); acc = x*9 + 100`. "
     u"오라클 실측: `AthleteParameter::new(&AthleteStat::default()).judge_accuracy() = 100`",
     u"03 new_knobs[1].effect")
S[3]["new_knobs"].extend([
    {"what": u"판단 능력치 원본",
     "where": u"`AthleteStat.judgement` = AthleteParameter+0x98 (= PlayerState+0x218)",
     "value": u"선수 능력치",
     "effect": u"judge_accuracy 의 입력. 올리면 위협 추정이 정확해진다 — "
               u"2026-09-11 검증배치 A 신규 발굴(명세 어디에도 없었다)"},
    {"what": u"**멘탈 -> 판단 곱수**",
     "where": u"`AthleteParameter.judgement_mental_ratio` = param+0x2d0 (= PlayerState+0x450)",
     "value": u"곱한 뒤 `min(.., 100)` 포화",
     "effect": u"**멘탈 상태에 따라 판단 정확도를 흔드는 곱수.** 이것 하나로 '멘탈 나간 선수가 오판한다'가 "
               u"구현돼 있다. 고정하면 멘탈이 판단에 미치는 영향이 사라진다 — 2026-09-11 검증배치 A 신규 발굴. "
               u"WARN 갱신 주체(`AthleteParameter::update`)는 **미탐색**"},
])
N[0] += 1
print(u"  OK 03 new_knobs += 2 (judgement / judgement_mental_ratio)")

# -- 08 is_end -------------------------------------------------------
print(u"\n[08] is_end")
rep(S[8], "logic",
    u"let epic = moba.jungle_runner.epic.live_list.first()  // MobaMode+0x198 (len==0 → None)",
    u"let epic = moba.jungle_runner.epic.live_list.get(0)   // MobaMode+0x198 (len==0 → None)",
    u"08 logic first() -> get(0)")
setv(S[8]["reads"][17], "note",
     u"==0 이면 `get(0)` = None 으로 접힘 (~~first()~~ — 2026-09-11 검증배치 B: "
     u"`first()` 는 `get` 을 부르지 않고 슬라이스 패턴으로 구현돼 있어 인라인 체인에 `fn=first` 프레임이 "
     u"있어야 하는데 없고, 실제 체인은 `slice/mod.rs:572 get<usize,usize>`)",
     u"08 reads[17].note")

# -- 09 check_favorable_engage_formation -----------------------------
print(u"\n[09] check_favorable_engage_formation")
setv(S[9]["signature"]["params"][4], "note",
     u"교전 사거리. +100000 한 뒤 제곱해 아군 참가 반경으로 씀. **호출부 8곳 전수 200000**"
     u"(m13.ll:18805 / 35679 / 37141 / 39946 / 40936 / 41820 / 42834 · m15.ll:33271). "
     u"~~「유일 호출처 = should_disengage_object_hunt」~~ 는 거짓(2026-09-11 검증배치 B)",
     u"09 signature.params[4].note")
setl(S[9]["unknown"], 0,
     u"version(p1) 이 실제로 무엇을 가르는지 — 이 함수 본문엔 분기가 없고 "
     u"`enemy_minion_line_action_danger_damage_at` 로 그대로 전달만 된다. "
     u"WARN ~~「유일 호출처가 리터럴 0 을 넘기므로 사실상 무의미」~~ 는 **거짓**(2026-09-11 검증배치 B): "
     u"호출부 **8곳**이고 **리터럴 0 은 1곳뿐**(m15.ll:33271 should_disengage_object_hunt), "
     u"나머지 7곳(plan_legacy::handler)은 **런타임 version SSA 값**을 넘긴다 ⟹ 주 경로에서는 "
     u"**살아 있는 버전 게이트**다. 재구현 시 0 하드코딩 금지. 미탐색 = 피호출자 내부의 version 분기",
     u"09 unknown[0]")

# -- 15 single_try_engage --------------------------------------------
print(u"\n[15] single_try_engage")
setv(S[15]["signature"]["params"][1], "note",
     u"team_plan(+0xf8)·positioning_score(+0x990) 만 만짐. "
     u"WARN ~~team_plan 은 &mut 로 하위에 전달~~ 은 **거짓**(2026-09-11 검증배치 D): tcx 정본상 "
     u"`self` 부터가 **공유 `&LegacyPlanHandler`** 라 `&mut self.team_plan` 이 성립 불가이고, "
     u"`single_tower_dive_is_viable`·`SinglePlanBattle::update` 둘 다 **`&TeamPlan`(공유)** 를 받는다. "
     u"이 계열에 team_plan 부작용은 **없다**(&mut 인 것은 rnd·debug·battle 뿐)",
     u"15 signature.params[1].note")
setv(S[15]["reads"][5], "note",
     u"`single_tower_dive_is_viable`(245행)·`SinglePlanBattle::update`(255행)에 "
     u"**`&TeamPlan`(공유)** 로 전달 — ~~&mut 로 전달 · 피호출자가 변경할 수 있음~~ 은 "
     u"거짓(2026-09-11 검증배치 D, tcx sig)",
     u"15 reads[5].note")
rep(S[15], "logic",
    u"&mut self.team_plan /*+0xf8*/, target, debug) {",
    u"&self.team_plan /*+0xf8, 공유 - ~~&mut~~ 정정*/, target, debug) {",
    u"15 logic 245행 &mut -> &")
rep(S[15], "logic",
    u"&mut self.team_plan /*+0xf8*/, debug);",
    u"&self.team_plan /*+0xf8, 공유 - ~~&mut~~ 정정*/, debug);",
    u"15 logic 255행 &mut -> &")

# -- 17 DeathMatchBattle::new ----------------------------------------
print(u"\n[17] DeathMatchBattle::new")
setv(S[17]["writes"][14], "note",
     u"**`i64::MAX`** 센티널 = '아직 도주사망 판정 없음' "
     u"(저장값 9223372036854775807 = 0x7FFF...FF. ~~usize::MAX~~ 는 오기 — 같은 명세의 logic 블록은 "
     u"i64::MAX 로 맞게 적혀 두 곳이 모순이었다. 2026-09-11 검증배치 D)",
     u"17 writes[14].note")
setv(S[17]["writes"][35], "note",
     u"**`Option<MainObjective>`(3B) 단일 Option** — ~~Option<Option<MainObjective>>~~ 은 **타입 오류**"
     u"(2026-09-11 검증배치 D, tcx 정본). 이중으로 읽으면 **니치가 한 겹 어긋나 재구현이 실제로 틀린다**. "
     u"태그 바이트(+0x17b)만 0xff 로 쓰고 0x17c/0x17d 는 안 씀",
     u"17 writes[35].note")
setl(S[17]["unknown"], 4,
     u"~~0x17c/0x17d 의 정체 미확인~~ -> **확정**(2026-09-11 검증배치 D): "
     u"`tcxdict --enum MainObjective` = 3B, 태그 +0x0(0..11), `Morgard`/`Serpen` 페이로드가 "
     u"**`phase: ObjectPhase @+0x1` · `with_battle: bool @+0x2`**. "
     u"None 니치는 태그바이트 0xff 하나 ⟹ **0x17c/0x17d 는 그 phase/with_battle 자리**",
     u"17 unknown[4]")

# -- 18 v3_epicops_buff_window ---------------------------------------
print(u"\n[18] v3_epicops_buff_window")
rep(S[18], "logic",
    u"match v3_epicops_repair_need(team, data.cache, plan) {   // i8 반환",
    u"match v3_epicops_repair_need(player, data, plan) {       // i8 반환. "
    u"~~(team, data.cache, plan)~~ 은 **ArgumentPromotion 아티팩트**(IR 인자 != 소스 인자). "
    u"tcx sig = fn(&PlayerState, &OperationData, &BigPlan). "
    u"promotion 이 일어났다는 것 자체가 'player 에서 info.team 만, data 에서 cache 만 읽는다'의 증명",
    u"18 logic repair_need 시그니처")
for j, ln in ((5, 674), (6, 676)):
    c = S[18]["constants"][j]
    setv(c, "src_line", ln, u"18 constants[%d].src_line 682 -> %d" % (j, ln))
    setv(c, "meaning", c["meaning"] +
         u" WARN 줄번호 정정(2026-09-11 검증배치 D): ~~682~~ 는 **`core/src/option.rs` 줄번호**가 "
         u"인라인 체인에 섞여 든 것(숫자 우연 일치). 실제 = is_none 판정 **epic.rs:673**, "
         u"Chat 구성 **674/676**. epic.rs:682 는 실재하지만 함수 꼬리(lifetime.end + 공통 출구)다",
         u"18 constants[%d].meaning" % j)
setv(S[18]["knobs"][5], "where",
     u"epic.rs:673 (`Option::is_none` 인라인) -> 674/676 에서 Chat 구성. "
     u"~~epic.rs:682~~ 는 core/src/option.rs 줄번호 혼입(2026-09-11 검증배치 D)",
     u"18 knobs[5].where")

# -- 저장 -------------------------------------------------------------
D["meta"].setdefault("corrections", [])
D["meta"]["corrections"].append(
    u"2026-09-11 **정정이 resolved[] 로그에만 있고 본 표는 옛 값이었다**(stalecheck.py 로 10건 적발). "
    u"§7 정정형 기록은 '옛 줄을 그 자리에서 고치라'는 뜻인데 changelog append 로 끝냈던 것. "
    u"tcxaudit 은 base+offset 만 보므로 이 오염을 못 잡는다 — apply13.py 로 본 표에 반영 완료.")
with io.open(P, "w", encoding="utf-8", newline="\r\n") as f:
    f.write(json.dumps(D, ensure_ascii=False, indent=1))
print(u"\n총 %d곳 반영 -> %s" % (N[0], P))
