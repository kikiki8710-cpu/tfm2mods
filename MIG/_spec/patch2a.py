# -*- coding: utf-8 -*-
u"""patch2a — 2026-09-11 **2차** 반증검증 정정을 specs20.json 본 표에 반영 (배치 A·B = 00~09).

원칙(1차의 실패에서 배운 것): 정정은 `resolved[]` 로그가 아니라 **본 표에** 넣는다.
표기 = `~~구~~ → 신`. 값 필드는 신값으로 교체하고 옛 값은 note 에 취소선으로 남긴다.
`unknown[]` 의 해소분은 여기서 손대지 않는다 — v3 재구성(mkspec3.py)이 `open[]` 을
처음부터 다시 짜면서 해소분을 자동으로 떨군다.
"""
import io, json, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
P = "_spec/specs20.json"
D = json.load(io.open(P, encoding="utf-8"))
S = D["specs"]
N = [0]


def rep(i, key, old, new, label):
    o = S[i]
    assert old in o[key], u"못 찾음: %s" % label
    o[key] = o[key].replace(old, new)
    N[0] += 1
    print(u"  OK %s" % label)


def setk(obj, key, val, label):
    obj[key] = val
    N[0] += 1
    print(u"  OK %s" % label)


def add(lst, item, label):
    lst.append(item)
    N[0] += 1
    print(u"  OK %s" % label)


def findk(i, field, needle):
    u"""배열에서 needle 을 담은 첫 항목."""
    for x in S[i].get(field) or []:
        if needle in json.dumps(x, ensure_ascii=False):
            return x
    raise AssertionError(u"%s[%s] 에 %s 없음" % (i, field, needle))


# ══════════ 00 abstract_input::ult ══════════
print(u"\n[00] ult")
rep(0, "logic",
    u"//       m = e.stat_buff_cached.radius_mult(0x470);",
    u"//       let mult = <stat_buff_cached.radius_mult(0x470)> as usize;   "
    u"// ★i32 -> usize 부호확장(sext). ~~m = ...~~ 은 부호확장 누락 표기",
    u"00 logic radius as usize")
rep(0, "logic",
    u"//       if m == 0 { e.radius(0x680) } else { e.radius * (m + 100) / 100 }",
    u"//       if mult == 0 { e.radius(0x680) } else { e.radius * (100 + mult) / 100 }   "
    u"// wrapping usize 산술\n"
    u"//       ⚠`as usize` 를 빼면 **음수 radius_mult 에서 동작이 갈린다**(거대 usize 로 접혀 반경 오버플로)",
    u"00 logic radius 식")
c = findk(0, "reads", "0x470")
setk(c, "note", (c.get("note") or u"") +
     u" ★`as usize` 로 **부호확장**돼 쓰인다(MIR entity.rs:1511 `_2 = move _3 as usize (IntToInt)`, "
     u"IR `sext i32 %9 to i64` @m04.ll:43306 · `%33` @43339). 이후 `100 + mult` 는 wrapping usize 산술",
     u"00 reads[0x470].note sext")

# ══════════ 01 calculate_jungle_action_score ══════════
print(u"\n[01] calculate_jungle_action_score")
rep(1, "logic",
    u"else if t.ty.is_jungle() {",
    u"else if t.ty.is_jungle(0) || t.ty.is_jungle(1) {",
    u"01 logic is_jungle 인자")
rep(1, "logic",
    u"ty.Jungle.info.camp_type.0 < 2",
    u"camp_type.0 in {0,1} = 양 팀 캠프 전부 (~~< 2 임계~~ 는 LLVM 접힘)",
    u"01 logic 유령 임계 1")
rep(1, "logic",
    u"else if t.ty.is_minion() {",
    u"else if t.ty.is_any_type_minion() {",
    u"01 logic is_any_type_minion")
lg = S[1]["logic"]
for old, new, lab in [
    (u"camp_type.0 >= 2 인 대상은 default 로 떨어져 coef = 0",
     u"~~camp_type.0 >= 2 면 coef 0~~ 은 **거짓** — camp_type.0 은 팀 인덱스라 {0,1} 뿐이고 "
     u"IR 의 `ult 2` 는 `x==0||x==1` 의 접힘이다(2차 배치A)", u"01 logic 유령 임계 2"),
    (u"위와 같은 is_jungle() 술어를 한 번 더 평가",
     u"같은 `is_jungle(0)||is_jungle(1)` 를 한 번 더 평가", u"01 logic 542 술어"),
    (u"based = if t.ty.is_jungle() { 5 } else { 0 };",
     u"based = if t.ty.is_jungle(0) || t.ty.is_jungle(1) { 5 } else { 0 };   // L542=57자 ±0",
     u"01 logic based 식"),
    (u"if hp > value { 20 } else { 40 }",
     u"if hp <= value { 40 } else { 20 }            // ★then/else 반전형 확정 — "
     u"L531=20자라 `if hp > value {`(19자)는 길이가 안 맞는다(2차 배치A)",
     u"01 logic L531 arm 반전"),
]:
    if old in S[1]["logic"]:
        rep(1, "logic", old, new, lab)
    else:
        print(u"  -- 건너뜀(이미 반영): %s" % lab)
c = findk(1, "reads", "0x98")
setk(c, "note",
     u"tuple(usize, JungleType) 의 usize 쪽 = **팀 인덱스(0 블루 / 1 레드)**. 소스는 "
     u"`is_jungle(0) || is_jungle(1)`(인자와 **동등 비교**, entity.rs:1377 `fn(&EntityType, usize) -> bool`)이고 "
     u"~~`< 2` 요구~~ 는 LLVM 접힘이다(2차 배치A)",
     u"01 reads[0x98].note")
for cc in S[1]["constants"]:
    if cc.get("src_line") == 531:
        setk(cc, "meaning", cc["meaning"] +
             u" ★소스 L531 은 **`if hp <= value { 40 } else { 20 }` 형태**(20 이 else-arm) — "
             u"rmeta_srcmap L531=20자. 어느 표기인지(`hp <= value` / `value >= hp`, 둘 다 11자)는 "
             u"**표기 불가**이나 **arm 반전 구조는 확정**(2차 배치A)",
             u"01 constants L531 arm")

# ══════════ 02 AttackNexusPlan::sub_plan ══════════
print(u"\n[02] AttackNexusPlan::sub_plan")
add(S[2]["resolved"], {
    "was": u"3분기(Recall / LineDefense / AttackNexus)가 실행으로 확인된 적이 없다",
    "now": u"★오라클 실행 확증(2026-09-11 2차배치A, `_verify2\\A\\A2_oracle3.tsv`, tps=60): "
           u"①샘 안(15000,913000)+hp 500/999 → **Recall** ②샘 밖(500000,500000)+hp 500/999 → "
           u"**LineDefense{line=Mid(=self.line), Push, Aggressive}** ③적팀 타워 전량 제거"
           u"(twin_towers[1].len()==0) → **AttackNexus**. "
           u"`AttackNexusPlan::new(1, Mid)` 로 team=1 을 줬는데 결과 line 이 Mid ⟹ "
           u"**`+0x0 team` 미참조 / `+0x8 line` 만 사용** 재확증. "
           u"'샘 안이지만 HP 꽉 참'은 ②와 같은 경로 = 분기 순서 확증."},
    u"02 resolved 3분기 실행확증")
add(S[2]["resolved"], {
    "was": u"L38 `is_in_heal_area` 표기 잔차 2자(1차)",
    "now": u"★해소 ±0(2026-09-11 2차배치A). L38 = "
           u"`    let is_in_heal_area = champ.x >= lx && champ.x <= rx && champ.y >= ly && champ.y <= ry;` "
           u"(실측 91자 = indent 4 + 87). IR 의 `!(y<ly || y>ry)` 는 이것의 De Morgan 변형. "
           u"⚠1차의 잔차 2자는 **들여쓰기를 2로 가정**해 생긴 것 — tcx sp 10:3·15:3 ⟹ impl 멤버 indent 2, "
           u"**fn 본문 indent 4**. L41 은 `if is_in_heal_area && champ.hp < champ.stat_cached.hp {`(55자) "
           u"대비 실측 54자 = **잔차 1자**. "
           u"⚠쌍둥이: `plan_legacy\\sub_plan\\attack_nexus.rs` 는 다른 파일(L38=1자) — `old\\` 한정 조회 필수."},
    u"02 resolved L38 ±0")

# ══════════ 03 defensive_crisis ══════════
print(u"\n[03] defensive_crisis")
jmr = findk(3, "new_knobs", "judgement_mental_ratio")
setk(jmr, "where",
     u"`AthleteParameter.judgement_mental_ratio` = param+0x2d0 (= PlayerState+0x450). "
     u"★**갱신 주체 확정 = `AthleteParameter::update`**(pub, player.rs:135, 본체 `_gcbc/g15.ll:126497`, "
     u"갱신부 player.rs:180~186 = g15.ll:126767~126798). 시그니처(DWARF !142612~!142620) = "
     u"`update(&mut self, rnd, _team, tick, statistics: &PlayerStatistics, team_gold_diff: i32, "
     u"dm_score_gap: usize, team_death_delta: usize, lane_cs_behind: bool)`",
     u"03 jmr.where 갱신주체")
setk(jmr, "value",
     u"초기값 **1000**(`AthleteParameter::new`, g15.ll:126475 `store i64 1000`). 갱신식:\n"
     u"  if dm_score_gap > 0 {\n"
     u"    gap_norm    = min(dm_score_gap, 3) * 1000 / 3;\n"
     u"    degradation = (100 - min(stat.mental, 100)) * gap_norm / 100 * 110 / 100;\n"
     u"    target      = max(1000.saturating_sub(degradation), 10);\n"
     u"    judgement_mental_ratio = min(judgement_mental_ratio, target);   // ★min = 단조 감소(래칫)\n"
     u"  }\n"
     u"★`_gcbc` 전 16파일 스캔 결과 `+0x2d0` 필드 store 는 **`new`(1000)와 이 한 줄 둘뿐** ⟹ "
     u"한 번 내려간 값은 update 로 절대 안 올라온다",
     u"03 jmr.value 공식")
setk(jmr, "effect",
     u"**멘탈 × 팀 점수차(dm_score_gap)** 두 축으로 판단 정확도를 깎는다 — "
     u"'멘탈 나간 선수가 오판한다'의 구현 경로가 이 한 쌍이다. "
     u"오라클 진리표 **48/48 MATCH**(`_verify2\\A\\A2_oracle1.tsv`): "
     u"mental>=100 → 어떤 gap 에서도 1000(무손상) / mental=80 → 928/865/802 / mental=50 → 829/667/505 / "
     u"mental=0 → 667/334/**109**(사실상 최저). `min(gap,3)` 이라 **gap 3에서 포화**. "
     u"반복 호출 [1000,505,505,…] 로 되돌아오지 않는다(래칫 확인). "
     u"★**개입 지점 3개**: `min(gap,3)` 의 3 / `110/100` 의 110 / `max(..,10)` 의 하한 10",
     u"03 jmr.effect 진리표")
ja = findk(3, "new_knobs", "judge_accuracy 기반")
setk(ja, "effect", ja["effect"] +
     u" ★오라클 전수 진리표(2차배치A): judgement 0/1/10/11/50/99/100/101/111/200/1000 → "
     u"acc 100/109/190/199/550/991/1000×4. `judgement` 는 **100 에서 포화**, acc 는 **9 간격 이산**. "
     u"손계산 `min(judgement*jmr/1000,100)*9+100` 과 11/11 일치",
     u"03 judge_accuracy 진리표")
jb = findk(3, "new_knobs", "judgement`")
setk(jb, "where",
     u"`AthleteStat.judgement` = AthleteParameter+0x98 (= PlayerState+0x218) — tcx 확정"
     u"(AthleteStat 232B, judgement +0x98; AthleteParameter.stat 는 +0x0). "
     u"`judgement_base()` = `min(stat.judgement, 100)` 이라 **실효 상한 100**",
     u"03 judgement.where")
k0 = S[3]["knobs"][0]
setk(k0, "effect", k0["effect"] +
     u" ★**실행 확증(tps 를 실제로 60 으로 세팅, 2차배치A)**: 같은 판(die_tick=60 고정)에서 "
     u"tps=60 → `60<120` **true** / tps=6 → `60<12` **false** / tps=1 → **false** "
     u"(`A2_oracle3.tsv` dcrisisB) ⟹ 임계 `die < tps*2` 실행 확정. "
     u"⚠1차는 `GameSetting::default()` 의 `tick_per_second=0` 때문에 이 확인을 못 했다",
     u"03 knobs[0] tps 실행확증")
k30 = findk(3, "knobs", "30000")
setk(k30, "effect", k30["effect"] +
     u" ★실행 확증(2차배치A): `game_ai::max_range(foe,me)=0` 인데 제곱거리 1,000,000 이 "
     u"`(0+30000)^2 = 9e8` 이하라 **필터 통과** → die_imminent 계산 진입. "
     u"`game_ai::max_range` 는 크레이트 루트 재수출로 **직접 호출 가능**",
     u"03 knobs 30000 실행확증")

# ══════════ 04 handle_line_defense ══════════
print(u"\n[04] handle_line_defense")
nk0 = S[4]["new_knobs"][0]
setk(nk0, "value",
     u"**team0** Top (80000,820000) · Mid (144000,817000) · Bottom (144000,880000) / "
     u"**team1** Top (820000,80000) · Mid (817000,144000) · Bottom (880000,144000) "
     u"(~~팀 표기 없이 team1 값만 적혀 있었다~~ — 오라클 실측, 2차배치A)",
     u"04 new_knobs[0].value 팀 표기")
add(S[4]["resolved"], {
    "was": u"`has_line_defense_threat(player, data, line, tower.id)` 는 _gaibc 에 define 이 없는 외부 선언",
    "now": u"★**define 이 있다**(2026-09-11 2차배치A): `_gaibc/m04.ll:60400`(defense_nexus.rs:587~604), pub.\n"
           u"  let bm = data.blackboard[player.info.team].<line>_minion_state;            // :588\n"
           u"       //  Blackboard +0x0 top / +0x28 mid / +0x50 bottom (각 BrainMinionParameter 40B)\n"
           u"       //  IR: data+0x10 -> gep Blackboard, team -> switch line {0:+0, 1:+40, 2:+80}\n"
           u"  if bm.from_mid(+0x10,i64) < -3000 || bm.minion_count(+0x20,i32) < -2 {     // :589~590\n"
           u"    data.cache.minions(1 - team, pool).any(|m|                               // :594 ★적팀 미니언\n"
           u"         map_regions::is_near_line(ctx.map, m.x(+0x660), m.y(+0x668), line)  // :595\n"
           u"      && m.ty(+0x68) == 1 /*Minion*/                                         // :599\n"
           u"      && m.ty.Minion.<target: Option<usize>>(tag +0x88 / val +0x90) == Some(tower_id))  // :600\n"
           u"  } else { false }\n"
           u"⟹ **'라인이 밀렸다(from_mid<-3000) 또는 미니언 수가 밀린다(minion_count<-2)' AND "
           u"'그 타워를 노리는 적 미니언이 있다'**. "
           u"★오라클 항별 분해(`A2_oracle4.tsv`): line_exists=true·morgard=true·epic1=600·visible=5/5·near=0~3 인데도 "
           u"**threat=false** 라 handle_line_defense 가 false 였다 ⟹ 실패 원인을 이 게이트로 실행 특정."},
    u"04 resolved has_line_defense_threat 본문")
for it in [
    {"what": u"라인 방어 위협 게이트 — 라인 밀림 임계",
     "where": u"defense_nexus.rs:589 (m04.ll:60446 `icmp slt i64 %22, -3000`)",
     "value": -3000,
     "effect": u"`blackboard[내팀].<line>_minion_state.from_mid`(BrainMinionParameter+0x10)가 이보다 "
               u"작아야(우리 쪽으로 밀렸어야) 위협 판정에 진입한다. 0 쪽으로 올리면 조금만 밀려도 라인 방어를 검토"},
    {"what": u"라인 방어 위협 게이트 — 미니언 수 열세 임계",
     "where": u"defense_nexus.rs:589~590 (m04.ll:60451 `icmp slt i32 %26, -2`)",
     "value": -2,
     "effect": u"`minion_count`(+0x20)가 이보다 작아야 위협. 두 조건은 **OR** 이라 하나만 만족해도 "
               u"다음 단계(적 미니언 타깃 확인)로 간다"},
]:
    add(S[4]["new_knobs"], it, u"04 new_knobs += %s" % it["what"][:22])
for it in [
    {"base": "Blackboard", "offset": "0x0/0x28/0x50", "name": u"top/mid/bottom_minion_state",
     "note": u"BrainMinionParameter(40B). `has_line_defense_threat`(:588)가 line 으로 switch 해 고른다 (2차배치A)"},
    {"base": "BrainMinionParameter", "offset": "0x10", "name": "from_mid",
     "note": u"i64. `< -3000` 이면 라인 밀림 위협 (2차배치A)"},
    {"base": "BrainMinionParameter", "offset": "0x20", "name": "minion_count",
     "note": u"i32. `< -2` 이면 미니언 수 열세 위협 (2차배치A)"},
]:
    add(S[4]["reads"], it, u"04 reads += %s" % it["name"])

# ══════════ 06 v2_response_retreat_stance ══════════
print(u"\n[06] v2_response_retreat_stance")
rep(6, "logic", u".collect_in(data.context.pool)",
    u".pipe(|it| bumpalo::collections::Vec::from_iter_in(it, data.context.pool))   "
    u"// ★~~collect_in~~ 이 아니라 **2인자 `from_iter_in`** (심볼 실측 m13.ll:45590 + L26/27/30 줄길이 ±0)",
    u"06 logic from_iter_in")
for old, new, lab in [
    (u"filter_map(|slot| slot)", u"filter_map(|c| *c)", u"06 logic filter_map"),
    (u"!fight_model::is_ignored_well_enemy(version, player, e)",
     u"!is_ignored_well_enemy(version, player, c)   // ★호출부에 `fight_model::` 접두 없음(L22=53자 ±0)",
     u"06 logic 접두 없음"),
]:
    if old in S[6]["logic"]:
        rep(6, "logic", old, new, lab)
    else:
        print(u"  -- 건너뜀: %s" % lab)
for kk in S[6]["knobs"]:
    v = str(kk.get("value"))
    if v in ("40000000001", "22500000001"):
        base = "200000" if v.startswith("4") else "150000"
        setk(kk, "note", (kk.get("note") or u"") +
             u" ★소스 표기는 `champ.distance_sq(c) <= %s * %s` 다 — IR 의 `icmp ult .. %s` 는 "
             u"`<=` + 상수접힘 결과이고 **소스에 `+1` 리터럴은 없다**(2차배치B, 줄길이 ±0)" % (base, base, v),
             u"06 knobs %s 소스표기" % v)
add(S[6]["resolved"], {
    "was": u"closure2 L30 의 잔차 +23자(1차 미탐색)",
    "now": u"★해소 — `, data.context.pool);` 가 **정확히 21자** + 들여쓰기 2 = 23. "
           u"`from_iter_in` 2인자 형태였기 때문이다(2026-09-11 2차배치B). L13~L32 전 구간 잔차 0."},
    u"06 resolved 잔차 0")
add(S[6]["resolved"], {
    "was": u"오라클로 실행 검증이 가능한가",
    "now": u"★**차단(실측)**. `v: in:game_ai` 라 `error[E0624]: method ... is private` "
           u"(`_verify2\\B\\B_o4_blocked.rs`, 2026-09-11). `LegacyPlanHandler` 자체는 pub 이나 "
           u"pub 메서드는 `handle_chat`/`get_small_action` 2개뿐이라 이 함수로 가는 pub 경로가 없다. "
           u"미탐색 = `AiAgent::update` 로 전체 시뮬을 돌려 간접 관측."},
    u"06 resolved 오라클 차단 실측")

# ══════════ 05 v50_fold_dive_episode ══════════
print(u"\n[05] v50_fold_dive_episode")
add(S[5]["resolved"], {
    "was": u"end_reason 0~8 표의 제3 독립 출처",
    "now": u"★부분 확보(2026-09-11 2차배치B). ①**카디널리티 독립 확인**: "
           u"`game_core::GankStatistics.dive_ep_end_reason` 이 **`[usize; 9]`** "
           u"(game_view DWARF `_gvbc/v00.ll:!16729` size 576bit + `!16731` DISubrange count 9) ⟹ "
           u"코드 개수가 정확히 9(0~8). ②`V50DiveEpisode` 22필드 오프셋 0x00~0x61 = tcxdict 전건 일치, "
           u"선언줄 = `ai_interface.rs:444~465`. ③rmeta 주석 `_docs\\game_ai.txt:510`. "
           u"④MIR = **재료 부재**(`mirdump_game_ai` 에 fold/track 둘 다 없다 — 실측) "
           u"⑤오라클 = **E0624 차단**(실측) ⟹ 각 코드의 *의미* 제3출처는 **exe 디스어셈만 남음(미탐색)**."},
    u"05 resolved end_reason 제3출처")
add(S[5]["resolved"], {
    "was": u"BigPlan::get_name 표가 IR 근거뿐",
    "now": u"★오라클 실행 검증 8종(2차배치B, `B_o1.tsv`): ForcePassive→\"ForcePasive\""
           u"(**원문 오타 실행 확인**)→9 / ActiveRecall→6 / EpicHuntAndPoke·EpicHuntAndBattle→4 / "
           u"SerpenHuntAndPoke·SerpenHuntAndBattle→5 / AttackNexus·DefenseNexus→8. "
           u"미탐색 = 나머지 8 variant(private 필드라 구성 불가)."},
    u"05 resolved get_name 실행")
k1 = S[5]["knobs"][1]
setk(k1, "where", u"dive_episode.rs:131 (확정 — 132~137 은 한국어 주석 6줄이라 코드가 없다)",
     u"05 knobs[1].where 확정")

# ══════════ 07 EpicHuntAndBattlePlan::sub_plan ══════════
print(u"\n[07] EpicHuntAndBattlePlan::sub_plan")
k = findk(7, "knobs", "4856")
setk(k, "value",
     u"`tick_per_second`(GameSetting+0x12f8) × 계수 1 — ★**리터럴이 아니다.** "
     u"~~4856~~ 은 10진 **오프셋**(0x12f8)이었다(2차배치B가 본표 미반영 적발)",
     u"07 knobs 4856 → tps")
r4 = S[7]["resolved"][4] if len(S[7]["resolved"]) > 4 else None
if r4 and u"원리적 불가" in json.dumps(r4, ensure_ascii=False):
    setk(r4, "now", r4["now"].replace(
        u"원리적 불가 확정",
        u"**재료 부재로 종결**(범위 = still_unknown[0] 의 5경로). ~~원리적 불가~~ 는 "
        u"디버그정보 경로 한 가지에만 해당하는데 전 범위 표현을 썼다(판정어휘 위반, 2차배치B)"),
        u"07 resolved[4] 판정어휘 범위정정")
f = findk(7, "reads", "0x6d70")
setk(f, "note", (f.get("note") or u"") +
     u" ★오라클 실측(0.5.8 moba, 2차배치B) = `[(0, 896000, 64000, 960000), (892000, 0, 960000, 64000)]`. "
     u"명명 구조체가 아니라 **4-튜플 (u64,u64,u64,u64)** 이고 `f.0<=f.2 · f.1<=f.3`(min/max 코너). "
     u"`fountains[i]` = 팀 i **자기** 분수대",
     u"07 reads fountains 실측")
add(S[7]["resolved"], {
    "was": u"오라클로 실행 검증이 가능한가",
    "now": u"★가능(2026-09-11 2차배치B). `EpicHuntAndBattlePlan: Default` + `sub_plan` pub. "
           u"실측(`B_o1.tsv`): version ∈ {0,1,2,3,40,50,60} 전부 "
           u"**tag=11 EpicHunt(EpicHuntSubPlan{need_recall:false})** ⟹ "
           u"①태그 11=EpicHunt ②need_recall=false ③**version 게이트 없음**을 실행 확인. "
           u"한계(입력 판별력): 에픽 엔티티 부재로 Recall(5) 미판별, target_bush=None 이라 Hide(9) 미판별. "
           u"미탐색 = `MobaMode.live_list` 주입."},
    u"07 resolved 오라클 tag11")

# ══════════ 08 EpicHuntAndPokePlan::is_end ══════════
print(u"\n[08] is_end")
if u"filter_map(|slot| slot)" in S[8]["logic"]:
    rep(8, "logic", u"filter_map(|slot| slot)", u"filter_map(|c| *c)", u"08 logic filter_map")
c = findk(8, "reads", "iter_champions")
setk(c, "note", c["note"].replace(u"simulation.rs:1904", u"simulation.rs:1904(=선언줄)") +
     u" ⚠1차의 「인라인 루트가 1905 이므로 1904 는 1줄 차」는 **오독**이다(2차배치B): tcx 실측 "
     u"`iter_champions` 선언 = 1904:3-1904:85, 그 안의 `{closure#0}`(filter_map 술어) = 1905:50-1905:53. "
     u"루트가 1905 인 것은 클로저 줄이기 때문이고 **1904 가 맞다**. "
     u"같은 함정 2건: `Position::as_index` 580/581 · `Entity::distance_sq` 2157/2158",
     u"08 reads iter_champions 1904 확정")
add(S[8]["resolved"], {
    "was": u"L164 objective 게이트 / L172 setup_like 가 실제로 그렇게 갈리는가",
    "now": u"★실행 검증(2026-09-11 2차배치B, `B_o3.rs`). TeamPlan.objective 를 바꿔 `is_end(3,…)` 실측: "
           u"None(default) → **true**(L164) / Morgard{None|Assemble|Hunt, wb=any} → **false** / "
           u"Morgard{**Setup**, wb=any} → **true**(setup 전용 종료경로 진입) / "
           u"Serpen{4 phase 전부}·Defense·Nexus(Mid) → **true**(태그≠0 즉시 종료) ⟹ "
           u"①`TeamPlan+0x41f==0`(Morgard) 만 통과 ②`+0x420==Setup` 만 추가 종료경로를 켠다 "
           u"③**`with_battle` 은 이 함수 판정에 무영향** — 전부 실행 확인. "
           u"`ObjectPhase` 태그 tcxdict = None 0 / Setup 1 / Assemble 2 / Hunt 3."},
    u"08 resolved objective 진리표")
k15 = findk(8, "knobs", "15")
setk(k15, "note", (k15.get("note") or u"") +
     u" ★오라클로 `15 * tick_per_second` 임계를 **측정하지 못했다**(입력 판별력 부재, 2차배치B): "
     u"tps 를 1~400 스윕해도 결과가 안 뒤집힌다 — 시작 직후 `next_respawn_tick=0, tick=0` 이라 "
     u"`usub.sat(0,0)=0` 이고 `0 > 15*tps` 가 항상 false. "
     u"미탐색 = `MobaMode.next_respawn_tick` 주입 / 틱 진행",
     u"08 knobs 15 오라클 한계")

# ══════════ 09 check_favorable_engage_formation ══════════
print(u"\n[09] check_favorable_engage_formation")
setk(S[9]["signature"]["params"][0], "note",
     u"★AI 버전 게이트가 **아니다**(이름만 version). 본문 분기 0 + **피호출자 2단이 모두 `i64 poison`** "
     u"(LLVM 이 미사용을 증명) + 오라클 version 12종 × 400 시나리오 **결과 차이 0건** ⟹ "
     u"**이 체인 전체에서 죽은 인자다**(2026-09-11 2차배치B). "
     u"~~1차의 '주 경로에서는 살아 있는 버전 게이트'~~ 는 관측은 맞고 **결론이 틀렸다**. "
     u"재구현 지침: 받아서 흘리기만 하면 되고 0 하드코딩도 결과 불변(단 시그니처 호환을 위해 인자는 유지)",
     u"09 signature.params[0] version 죽은 인자")
add(S[9]["resolved"], {
    "was": u"version(p1) 이 무엇을 가르는가 — 1차는 '살아 있는 버전 게이트'로 결론",
    "now": u"★★**죽은 인자로 확정**(2026-09-11 2차배치B). "
           u"①본문에서 `%0` 의 용도는 피호출자에 넘기는 것뿐(m15.ll:35410~35850, 비교 0건) "
           u"②`enemy_minion_line_action_danger_damage_at`(m07.ll:48226) 진입부가 "
           u"`#dbg_value(i64 poison, !60288)` 이고 다시 `enemy_minion_line_action_damage_at` 에 "
           u"**`i64 poison` 을 넘긴다**(m07.ll:48235) ③최종 피호출자(m07.ll:47470)도 poison + 미사용 "
           u"④오라클 version ∈ {0,1,2,3,10,20,30,40,46,50,54,60} × 400 시나리오 → **diff 0건**. "
           u"미탐색 = 개발사 소스의 원래 의도(IR 로는 소거돼 복원 불가)."},
    u"09 resolved version 죽은 인자")
add(S[9]["resolved"], {
    "was": u"본문 판정 로직(각도 분류·인원 조건·거리비율)의 실행 검증 없음",
    "now": u"★★**오라클 400/400 MATCH**(2026-09-11 2차배치B, `_verify2\\B\\B_o2.rs`). "
           u"`game_ai::check_favorable_engage_formation` 은 크레이트 루트 재수출 pub 이라 직접 호출된다. "
           u"`AbstractGameWithCache` 전 필드 pub + `Entity` 전 필드 pub·Clone 으로 아군 5칸·타깃의 "
           u"좌표/HP/생존을 완전히 통제: engage_range ∈ {50000,100000,200000,400000}, 아군 좌표 ±350000 난수, "
           u"HP% ∈ {20,39,40,41,80,100}, 슬롯 결손 10% → **400 시나리오 불일치 0**. "
           u"분기 커버리지 = rear>0 150 / flank&front 26 / flank>1 10 / front>1 true 6·false 4 / 종료 false 204 "
           u"⟹ **모든 종료 경로가 실제로 밟혔다**. 실행 확정 항목: HP 임계 40 · 여유 100000 · "
           u"`dot>0 && dot²*4>lp` · `dot<0 && dot²*100>lp*9` · `cross²*100>lp*9` · "
           u"분기 순서(front→rear→flank→else front) · rear 의 base 거리 타이브레이크 · "
           u"`rear>0` / `flank>1||(flank>0&&front>0)` / `front>1→5:6` · fountain 중심 공식."},
    u"09 resolved 400/400")
ex = S[9].get("exe")
if isinstance(ex, dict):
    setk(ex, "callers_note",
         u"★`callers: []` 는 '호출자 없음'이 아니라 **exe 조인 실패**다. IR 실측 **8곳** = "
         u"m13.ll:18805/35679/37141/39946/40936/41820/42834 + m15.ll:33271 "
         u"(`spec3lib.callsites` 로 0.7초에 전수 확인 — 2026-09-11)",
         u"09 exe.callers_note")
u5 = [i for i, x in enumerate(S[9]["unknown"]) if u"1321" in x]
if u5:
    S[9]["unknown"][u5[0]] = (
        u"1321행 `flank>1 || (flank>0 && front>0)` 의 **바깥 || 두 항 순서만** 확정 불가"
        u"(비단축 or 평탄화 = 정보량 0). **안쪽 && 는 확정** — `select i1 %95, i1 %96, i1 false` 로 "
        u"단축평가가 보존돼 `flank>0` 이 좌항(m15.ll:35598~35601). "
        u"재료 부재 범위 = ①DILocation.column 전 모듈 0 ②mir=0 ③줄 길이 산술은 교환 불변 "
        u"④IR 피연산자 순서 정보량 0. 미탐색 = exe 디스어셈·개발사 소스 (2차배치B)")
    N[0] += 1
    print(u"  OK 09 unknown[1321] 범위 표기")

D["meta"].setdefault("corrections", []).append(
    u"2026-09-11 **2차 반증검증** 배치 A·B(00~09) 정정을 본 표에 반영(patch2a.py). "
    u"실오류 = 09 version 죽은 인자(1차 결론 반증) / 06 from_iter_in·filter_map(|c| *c) / 07 knobs 4856=오프셋. "
    u"신규 확정 = 03 judgement_mental_ratio 갱신주체·공식·래칫(48/48) · 04 has_line_defense_threat 본문+노브2 · "
    u"09 오라클 400/400 · 02 3분기 실행확증 · 05 end_reason 카디널리티 독립출처. "
    u"1차 지적 되돌림 = 08 iter_champions 1904 가 맞다(선언줄 vs 클로저줄).")
with io.open(P, "w", encoding="utf-8", newline="\r\n") as f:
    f.write(json.dumps(D, ensure_ascii=False, indent=1))
print(u"\n총 %d곳 반영 -> %s" % (N[0], P))
