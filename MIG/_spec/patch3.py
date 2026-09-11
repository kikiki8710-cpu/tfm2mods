# -*- coding: utf-8 -*-
u"""patch3 — 2026-09-11 **3차** 반증검증 정정을 specs20.json 에 반영 (4갈래 전체).

3차의 잔존 오염원은 사실상 하나였다: **`logic` 산문이 표/`history` 의 정정을 못 받는다.**
(배치 D 실오류 5건 중 3건 · 배치 A E1/E2 · 배치 C E8 · 배치 B E2 — 전부 같은 형태)
⟹ 여기서 고치고, 재발은 `specgate` G5/G6 가 막는다.

`open`/`closed` 이동은 `closelist.py` 가 담당한다(이 파일에서 하지 않는다).
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
    for x in S[i].get(field) or []:
        if needle in json.dumps(x, ensure_ascii=False):
            return x
    raise AssertionError(u"%s[%s] 에 %s 없음" % (i, field, needle))


# ══════════ 배치 A ══════════
print(u"\n[배치 A] 00~04")

# A-E2 (구조) — 이 오류가 A-E1(src_line)을 만들었다
rep(2, "logic",
    u"if !has_enemy_twin_tower {\n"
    u"    return SubPlan::AttackNexus               // tag 16 — 적 쌍둥이탑 전멸 = 넥서스 직행\n"
    u"}\n"
    u"return SubPlan::LineDefense(LineDefenseSubPlan {",
    u"// ★L47~51 = **return 없는 꼬리표현식 + else**, 조건은 부정형이 아니다\n"
    u"//   (~~if !has_enemy_twin_tower { return AttackNexus }~~ 는 구조 오류 — 3차 배치A,\n"
    u"//    줄길이 ±0 4줄 + IR m12.ll:34955~34971 분기방향. 이 오류가 consts src_line 46 오기를 만들었다)\n"
    u"if has_enemy_twin_tower {                     // L47\n"
    u"    SubPlan::LineDefense(LineDefenseSubPlan {  // L48",
    u"A-E2 02 logic arm 반전(앞)")
rep(2, "logic",
    u"    minion_action_type: MinionActionType::Push,  // 2\n"
    u"})                                            // tag 2",
    u"        minion_action_type: MinionActionType::Push,  // 2\n"
    u"    })                                        // tag 2\n"
    u"} else {                                      // L49\n"
    u"    SubPlan::AttackNexus                      // L50, tag 16 — 적 쌍둥이탑 전멸 = 넥서스 직행\n"
    u"}                                             // L51",
    u"A-E2 02 logic arm 반전(뒤)")
c = S[2]["constants"][4]
assert c.get("value") == 16, c
setk(c, "src_line", 50, u"A-E1 02 consts[4].src_line 46 -> 50")
setk(c, "meaning", c["meaning"] +
     u" ★줄 정정(3차 배치A): ~~46~~ 은 빈 줄(1자)이다. L50 = 58자 → 내용 57 = indent 6 + "
     u"`SubPlan::AttackNexus(AttackNexusSubPlan::default())`(51자) ±0",
     u"A-E1 02 consts[4].meaning")

# A-E3
h = findk(4, "resolved", "is_near_line")
setk(h, "now", h["now"] +
     u"\n★정정(3차 배치A): `is_near_line` 의 1번 인자는 `ctx.map` 이 아니라 **`&GameContext`** 다 — "
     u"tcx `game_core::is_near_line(&GameContext, u64, u64, LineType)`(map_regions.rs:139) + "
     u"IR m04.ll:60492 가 넘기는 `%32` = `data.context`(deref 64B)",
     u"A-E3 04 history is_near_line 인자")

# A-E4 (shared)
ss = D["shared"].get(u"set_strategy_호출자")
if isinstance(ss, dict):
    for k in list(ss.keys()):
        if u"TeamStrategy" in str(ss[k]):
            setk(ss, k, str(ss[k]) +
                 u" ★정정(3차 배치A): 실제는 `fn(&mut Self, usize, game_core::Strategy)` — "
                 u"**`Strategy`(24B·13필드, strategy.rs:45)를 값 전달**한다(24B라 Win64 가 간접전달 = "
                 u"IR 에 포인터로 보인다). ~~&TeamStrategy(24B)~~ 는 오기이고 `TeamStrategy`(12필드)는 다른 타입",
                 u"A-E4 shared.set_strategy_호출자")
            break

# A 신규 확정
add(S[3]["resolved"], {
    "was": u"`effect_cc_time` 내부와 version 의 역할",
    "now": u"★확정(3차 배치A). `effect_cc_time` = `e.ty.<vt+0x88 = expected_cc_time_deep>()` "
           u"**단일 vtable 디스패치**이고 **version 인자를 완전히 쓰지 않는다**"
           u"(IR 등장 0건 + 오라클 1,150 호출 불일치 0). "
           u"★`cc_threat` 오라클 **44/44 실행 확증**: `level>2`(skill2)·`level>4`(ult)·`cool<=tps`"
           u"(60 통과 / 61 탈락) ⟹ consts[2]·consts[3]·knobs[1] **ev 4→2**. "
           u"CC 후보 = 실전 챔피언 60종 × 4슬롯 중 **41슬롯**(`Some(0)` 도 `is_some()` 이라 위협으로 센다)."},
    u"A 03 resolved effect_cc_time + cc_threat 44/44")
add(S[4]["resolved"], {
    "was": u"`has_line_defense_threat` 를 실제로 true 로 만들 수 있는가",
    "now": u"★**점등 성공**(3차 배치A). `(from_mid < -3000 || minion_count < -2) && "
           u"∃ 적 미니언(nearest_enemy == Some(tower_id))` 의 OR·AND 구조를 실행 확증. "
           u"★미확정이던 `<target>` 필드 이름 확정 = **`Minion::nearest_enemy`**"
           u"(Entity +0x88 태그 / +0x90 값). `Blackboard::minion_state(line)` 는 필드가 아니라 "
           u"**메서드**(blackboard.rs:378). "
           u"★인원 구간 20/20 + 반경 경계(d2 = 40000000000 통과 · 40000400001 탈락): "
           u"Gather `>1` / Battle `{1,2}` ⟹ knobs[0]·[1]·[2]·consts[2] **ev 4→2**."},
    u"A 04 resolved has_line_defense_threat 점등")
add(S[0]["resolved"], {
    "was": u"`get_input_target` 내부(2,049줄) — 미탐색",
    "now": u"★확정(3차 배치A). abstract_input.rs **345~655**, **None 반환 정확히 3곳**: "
           u"L346 `?` / L349 `!Effect::is_in_range` / L354 `!is_visible_from && !casting.is_nontarget()`. "
           u"`CastingType::is_nontarget()`(type.rs:148) = {Position, Direction} 이고 "
           u"IR 의 `(casting - 1) <u 2` 가 그 접힘이다. "
           u"격자 채점 = `position_score_at_cell(..., PositionEvalPurpose::AttackStance)`(태그 12, 니치 밀림), "
           u"best_pos = `Option<(i64,(i32,i32))>` + `Option::is_none_or`, 인덱스 `clamp(0,29)`, "
           u"`gen_range(0..=1000)`. 부수: PlayerState +0x180 = `AthleteParameter`(기존 확정 재확인)."},
    u"A 00 resolved get_input_target 확정")
add(S[0]["resolved"], {
    "was": u"`entity.rs:1511` 잔차 4자 — 들여쓰기 문제인가",
    "now": u"★**들여쓰기 문제가 아니다 → 표기 불가(4자)**(3차 배치A). **MIR 칸**으로 `Entity::radius` "
           u"나머지 6줄은 전부 ±0 복원됐다(`if mult == 0` / `self.radius * (100 + mult) / 100` 등). "
           u"L1511 은 `let <4자> = <29자> as usize;` 로 칸까지 고정되는데 의미상 "
           u"`self.stat_buff_cached.radius_mult` 는 33자다 ⟹ 표기 불가. "
           u"남은 미탐색 = 매크로 전개 여부. **동작·sext 는 확정.**"},
    u"A 00 resolved 1511 표기 불가")
add(S[2]["resolved"], {
    "was": u"`attack_nexus.rs:41` 잔차 1자",
    "now": u"★**표기 불가(1자)**(3차 배치A). 같은 함수 8줄이 indent 4/6 으로 ±0 이라 들여쓰기는 이미 맞다. "
           u"측정 54자 vs 후보 55자이고 `mir=False` 라 MIR 칸도 없다. "
           u"신규 정황: 이 58자 줄이 `passive_line.rs:1073`·`single_line.rs:286` 에 **바이트까지 동일** "
           u"= 공용 이디엄."},
    u"A 02 resolved 41행 표기 불가")

# ══════════ 배치 B ══════════
print(u"\n[배치 B] 05~09")
p = S[8]["signature"]["params"]
tgt = [x for x in p if u"TeamPlan" in str(x.get("type"))]
assert tgt, u"08 team_plan 인자 못 찾음"
setk(tgt[0], "type",
     u"&TeamPlan(1064B) — ★공유 참조. ~~&mut TeamPlan~~ 은 오기(3차 배치B, specgate G5 가 잡는다)",
     u"B-E1 08 params team_plan &mut -> &")
setk(tgt[0], "note", (tgt[0].get("note") or u"") +
     u" 근거: tcx 정본 sig 에 `mut` 없음 + 같은 명세의 `sig.tcx` 문자열과 자기모순이었다 + "
     u"IR m10.ll:7655 의 `%5` 에 store 0건(gep 2곳 모두 load)",
     u"B-E1 08 params note")
for it in [
    {"was": u"05 `end_reason` 0~8 의 *의미* — 제3출처가 exe 디스어셈만 남았다고 판단했다",
     "now": u"★**거짓이었다 — 호출자 IR 에 전부 있었다**(3차 배치B). `m13.ll` 의 `update` 에 인라인된 "
            u"`v50_track_dive_episode` 에 코드 산출 지점이 다 있다. 코드표:\n"
            u"  0 = 다이브 타워 변경 / 1 = BigPlan≠Battle / 2 = with_dive false / 3 = dive_tower None /\n"
            u"  4 = sub_goal 이 RunAway·End / 5 = 대상 엔티티 소멸 / 6 = 대상이 적팀 챔피언 아님 /\n"
            u"  7 = **다이브할 타워가 없음** / 8 = 내 챔피언 캐시 없음\n"
            u"⟹ 「end_reason==7 = 포기 집계 제외」가 **의미까지** 확정. "
            u"⚠이건 메인 세션 브리핑의 범위 판정 오류였다 — 「가진 재료의 한계를 문제의 한계로 착각」 재발."},
    {"was": u"05 `last_dive_abandon_tick` 의 소비 게이트",
     "now": u"★확정(3차 배치B). 읽는 곳 **8군데 전부** `tick > 값 + 1 + 4×tick_per_second` "
            u"(≈4초 재시도 쿨다운). 두 번째 writer 도 발견(handler.rs:1205)."},
    {"was": u"`V50DiveEpisode` 에 `prev_holder_hp`/`gap_ticks` 가 있는가",
     "now": u"★**필드가 아예 없다**(tcx 확정, 3차 배치B)."},
]:
    add(S[5]["resolved"], it, u"B 05 resolved += %s" % it["was"][:26])
add(S[7]["resolved"], {
    "was": u"07 `hp_ratio < 51` 경계와 세 번째 OR 항",
    "now": u"★**오라클 8/8**(3차 배치B, ev2). `hp_ratio < 51` 경계가 **정확히 50/51** 에서 갈린다. "
           u"세 번째 OR 항이 `hp < max && in_heal_area` 인 것도 확증. "
           u"태그 5=Recall · 11=EpicHunt 런타임 확인. `target_bush` 가 private 이라 Hide(9) 경로만 미도달. "
           u"⚠지시문의 「L36 **두** OR 항」은 오기 — 실제 **세 항**이고 명세가 맞다."},
    u"B 07 resolved 51 경계 8/8")
add(S[8]["resolved"], {
    "was": u"08 `15 × tick_per_second` 임계 — 오라클로 못 켰다(입력 판별력 부재)",
    "now": u"★**열렸다. 임계가 정확히 900**(3차 배치B, 오라클 13/13, ev2): `MobaMode.next_respawn_tick` 을 "
           u"직접 써 넣으면 된다(`Game::mode` 가 pub) — nrt 900→false, 901→true. "
           u"L164 objective 게이트·L193 에픽 생존·phase 판정·version 무영향(9종)도 동시 확증. "
           u"setup 경로는 `v24_...=true` 가 먼저 발화해 (c)(d) 는 오라클 미도달(미탐색)."},
    u"B 08 resolved 900 임계")
add(S[8]["resolved"], {
    "was": u"`AbstractGame` vtable 슬롯 번호의 근거",
    "now": u"★**공식 확정**(3차 배치B, ev3): `슬롯 = 0x20 + 8 × (트레이트 선언 순서)`, `0x18` = `Debug::fmt`. "
           u"**구현체와 무관**하며 `divtable` 표와 6/6 일치 ⟹ vtable 행 전부 ev 4→3. "
           u"덤: `<Game as AbstractGame>::get_game_mode` MIR = `GameMode::Moba(&self.mode)` **순수** ⟹ "
           u"`as_moba()` 2회 호출 동일성 확정(이 플랜이 Moba 전용임도 증명)."},
    u"B 08 resolved vtable 공식")
add(S[9]["resolved"], {
    "was": u"09 미니언 게이트 `enemy_minion_line_action_danger_damage_at` 내부",
    "now": u"★확정(3차 배치B). `damage` 계산 후 "
           u"`champion_action || has_epic_buff ? is_dangerous : is_critical` → 참이면 damage, 아니면 0. "
           u"`is_dangerous`/`is_critical` 임계표 전량(49/29/17/9 · 34/19) + "
           u"`has_epic_buff` = `MobaMode+0x240 epic_minion_buff_time[1−team] != 0`. "
           u"★09 는 `champion_action = true` 라 **항상 is_dangerous 표**를 쓰고 HP 비교 대상은 "
           u"**자기 챔피언**이다. 남은 미탐색 = `enemy_minion_line_action_damage_at` 본문(pub, 미니언 필요)."},
    u"B 09 resolved 미니언 게이트")

# ══════════ 배치 C ══════════
print(u"\n[배치 C] 10~14")
k = findk(14, "new_knobs", "target_bush_v41")
setk(k, "effect",
     u"★★**판정 반전 — `TargetMissing` 은 발화한다**(3차 배치C, ev2). "
     u"~~「v30≠v41 이라 도착 판정이 성립할 수 없고 TargetMissing 이 원리적으로 발화하지 않는다」~~ 는 **거짓**이다. "
     u"자연상태(lead=2 · 1차타워 생존 · nearest_enemy=None)에서 **v30 == v41 12/12** "
     u"(Top t0 3=3 · t1 6=6 / Mid 11·14 양팀 / Bottom t0 15 · t1 20). "
     u"실행 증거: 챔프를 v30 부시 셀에 놓고 `update` 를 부르면 "
     u"`chats:[Cancel(TargetMissing)] phase:Cancel` 이 실제로 나온다(4건, 나머지 부시 21개에서는 미발화). "
     u"불일치는 **lead=0 · 타워 파괴 · 타워가 적 인지 구간 한정**(72칸 중 32칸 일치)이라 "
     u"그 구간에서만 취소가 막힌다. "
     u"⟹ v30/v41 이 호출자별 하드와이어라는 구조는 그대로 맞지만, **결론이 과했다.**",
     u"C-E3 14 knobs v41 판정 반전")
setk(S[10]["signature"]["params"][0], "note",
     u"★AI 버전 게이트가 **아니다** — **죽은 인자**(3차 배치C, ev2). `is_enemy_well_danger` 는 "
     u"`%0`(version)을 한 번도 로드하지 않는다(`_gaibc/m03.ll:144500~144535` 전문) + "
     u"오라클 ver 0~5 × team 0/1 × 17×17 격자 전수 동일. "
     u"~~「버전 분기는 전적으로 이 함수 몫」~~ 은 거짓 ⟹ **10 전체에 버전 분기가 없다.**",
     u"C-E4 10 params[0] 죽은 인자")
for it in [
    {"value": 19600000000, "src_line": 1188,
     "meaning": u"= 140000² . `can_enemy_hit_objective` 진입 **하드컷** — `dx²+dy² > 19.6e9` 이면 즉시 false. "
                u"실측 경계 140000 true / 140001 false(3차 배치C, ev2). **신규 발굴**"},
]:
    add(S[10]["constants"], it, u"C-E6 10 constants += 19.6e9")
add(S[10]["knobs"], {
    "what": u"오브젝트 타격 가능 거리 하드컷",
    "where": u"fight_model.rs:1188 (= 140000²)",
    "value": 19600000000,
    "effect": u"`can_enemy_hit_objective` 가 이 거리를 넘으면 이펙트·레벨을 보지도 않고 false 를 낸다. "
              u"줄이면 원거리 적을 아예 위협으로 안 세고, 늘리면 맵 절반 밖 적까지 계산에 들어온다. "
              u"실측 경계 140000/140001 (3차 배치C)"},
    u"C-E6 10 knobs += 하드컷")
add(S[10]["resolved"], {
    "was": u"`can_enemy_hit_objective` 의 25000 단위 / `is_enemy_well_danger` 내부",
    "now": u"★둘 다 확정(3차 배치C). **25000 = 선형 거리 여유치**"
           u"(`Effect::is_in_range_ex` 의 8번째 인자로 들어간다), skill2 는 `level>2` · ult 는 `level>4` 게이트. "
           u"`is_enemy_well_danger` = 적 진영 우물 **L자 2사각형**(team 별 대칭) 판정이고 version 미사용. "
           u"vtable 슬롯 실측 4종: `0x28 tick`(5) / `0x40 get_game_mode`(8) / `0x1f0 get_entity_by_id`(62) / "
           u"**신규 `0x108 strategy`**(33). "
           u"`objective_entity_id_for_main_objective` 전수: 태그 0→`epic.live_list[0]`, "
           u"1→`serpen.live_list[0]`, **2~11→None**(ev2)."},
    u"C 10 resolved 25000 + 우물 + vtable")
add(S[11]["resolved"], {
    "was": u"`passive_plan` 반환 392B 중 뒤 8B 가 패딩인지 반환값인지",
    "now": u"★**별도 반환값**(3차 배치C). tcx: `passive_plan(...) -> (BigPlan, u8)`. "
           u"392 = 384 + u8 + 패딩 7 이고 **이 호출자는 u8 을 버린다**. "
           u"★근거가 이미 같은 명세의 `callees[]` 안에 있었다 — 자동 생성 필드를 안 본 사례. ev 4→3. "
           u"남은 미탐색 = 그 u8 의 의미(private 이라 오라클 불가)."},
    u"C-E5 11 resolved passive_plan 반환")
add(S[12]["resolved"], {
    "was": u"12 포맷 템플릿의 `{:?}` 추정(ev5)",
    "now": u"★확정(3차 배치C). `shared.포맷템플릿_문법.결론` 에 이미 답이 있었고"
           u"(정정이 shared 에만 반영된 사례), IR 직접 근거도 있다 — 인자 배열에 "
           u"`<Option<MainObjective> as Debug>::fmt` ×2, `<Chat as Debug>::fmt` ×1 store"
           u"(m13.ll rel 104/137/171). ev 5→4."},
    u"C-E8 12 resolved 포맷 확정")
add(S[14]["resolved"], {
    "was": u"`*_lead` 산출식 — 2차 IR 독해가 실행과 6/6 불일치했다",
    "now": u"★**확정(3차 배치C, ev2, 6/6 일치)**:\n"
           u"  lead = { let mut l = 0;\n"
           u"           for (i, r) in map.lane_seq(line, team).iter().enumerate() {\n"
           u"             let p = region_point[*r];\n"
           u"             if (team == 0 && p > 2) || (team == 1 && p < -2) { l = i } else { break }\n"
           u"           } l }                      // 0..=6\n"
           u"★**2차가 6/6 틀린 원인은 규칙이 아니라 team 부호를 놓친 것**이다(team0 `>2`, team1 `<-2`). "
           u"IR = `g15.ll` `new_with_prev_cache`, `lane_seq` 6회 호출 + 저장 104398~104410. "
           u"⚠배치 C 가 기록한 자기 함정: `awk` 범위를 다음 `define` 까지 안 잡아 "
           u"「store 0건 ⟹ region_point ≡ 0」이라는 틀린 결론을 냈고 **오라클이 즉시 반증**했다. "
           u"남은 미탐색 = `region_point` 자체 산출식(game_core 의 blue/red_regions 채우기 루프)."},
    u"C 14 resolved *_lead 산출식")
add(S[13]["resolved"], {
    "was": u"13 의 미검증 분기(Mid 봇사이드 등)가 '재료 부재' 인가",
    "now": u"★**재료 부재가 아니라 설정 데이터 누락(미탐색)이었다**(3차 배치C). "
           u"`GameSetting::default()` 는 `height == 0` 이라 `height − y` 가 u64 **언더플로**해 "
           u"`is_top_side` 가 상수 true 가 된다 — 2차의 「챔프 10명 전부 탑사이드」가 그 오진이다. "
           u"실전 height(960000)를 넣으면 같은 초기 좌표에서 **Support 2명이 false** 로 갈리고 "
           u"**13/14 의 26칸을 전부 밟았다**. 정본 템플릿 = `MIG\\_verify3\\TEMPLATE.rs`."},
    u"C 13 resolved 설정 누락 진단")

# ══════════ 배치 D ══════════
print(u"\n[배치 D] 15~19")
rep(15, "logic",
    u"실측 팀당 10개(6+4, twin 2쌍은 좌표 중복)",
    u"실측 **팀당 8개**(이름있는 6칸 + twin_towers 2개), **좌표 중복 0** "
    u"(~~팀당 10개(6+4, twin 2쌍 좌표 중복)~~ 은 `init_tower` 재호출로 타워가 2배가 된 오염값이었다 "
    u"— 3차 배치D 가 깨끗한 프로브로 재측정, 포인터 동일성까지 확인)",
    u"D-P1 15 logic 팀당 8")
rep(15, "logic",
    u"SinglePlanBattle::new_dive(version, &BattlePlanGoal::TryKill(target_id, 60), data, player)",
    u"SinglePlanBattle::new_dive(version, BattlePlanGoal::TryKill(target_id, 60), data, player)"
    u"  // ★`&` 없음(by-value) — tcx `fn(usize, BattlePlanGoal, &OperationData, &PlayerState)`",
    u"D-P3 15 logic 248 by-value")
rep(15, "logic",
    u"SinglePlanBattle::new(version, &BattlePlanGoal::TryKill(target_id, 60), data, player)",
    u"SinglePlanBattle::new(version, BattlePlanGoal::TryKill(target_id, 60), data, player)"
    u"  // ★`&` 없음(by-value)",
    u"D-P3 15 logic 253 by-value")
rep(15, "logic",
    u"if let EntityType::Tower(tt) = e.ty /*+0x68 == 2*/ { Some(tt) /*+0x128*/ }",
    u"if let EntityType::Tower { info } = &e.ty /*+0x68 == 2*/ { Some(info.ty) /*+0x128*/ }"
    u"  /* ★**struct variant** `{ info: Tower }` — ~~Tower(tt)~~ 는 튜플 variant 오기(rustc 진단). "
    u"오프셋 +0x128 은 맞다: Entity+0x68 → 페이로드+0x8 → Tower+0xb8 */",
    u"D-P4 15 logic 250 struct variant")
add(S[15]["resolved"], {
    "was": u"2차의 `Effect::is_in_range` 900/900 · `engage_requires_dive` 300쌍 — 오염 여부",
    "now": u"★**깨끗한 프로브로 재측정 완료**(3차 배치D). `start_game` 만 부르면 "
           u"`tower_ids 16 · 팀당 8 · twin 2/2 · 좌표중복 0`.\n"
           u"  `Effect::is_in_range` game==mine : 2차 900/900(2배 세계) → **3차 676/676 (mismatch 0)**\n"
           u"  `engage_requires_dive` 진리표     : 2차 300쌍/true 100 → **3차 260쌍/true 80**(재현 260/260)\n"
           u"  `best_jungle_goal`                : 10/10 유지\n"
           u"★**검증 강도 대폭 상향**: 실물 26개는 true 26(자기쌍)뿐이라 판별력이 약해서 `Entity` 를 복제해 "
           u"`casting`(4종)·range·growth_range·level·radius·radius_mult·거리를 변주한 "
           u"**합성 엔티티 1,440개**로 전수 대조 → **2,073,600쌍 · mismatch 0 · game_true 40%**. "
           u"`Effect::range`·`Entity::radius` 재현식도 각각 1440/0.\n"
           u"➕`iter_towers_without_nexus` 의 6칸 순서 + `twin_towers` 꼬리가 "
           u"**포인터 동일성(PTR_IDENTICAL)** 으로 확정 ⟹ 해당 항목 ev 4→2."},
    u"D 15 resolved 오염 재측정")
add(S[15]["resolved"], {
    "was": u"`SinglePlanBattle::update` 의 sub_goal 게이트 — 개별 술어 미탐색",
    "now": u"★**1단위 이분탐색으로 거리·틱 게이트 확정**(3차 배치D, ev2). "
           u"`new/new_dive/update`·`BattlePlanGoal`·`BattleSubPlanGoal` 이 전부 pub 이라 본문을 그대로 돌렸다:\n"
           u"  거리 `<= 100000` → `Kiting{focus}` / `100001~250000` → `Trace{focus}` / "
           u"`>= 250001` → `RunAway`\n"
           u"  `tick <= 120` vs `>= 121` → `RunAway`\n"
           u"**축 비의존**(x/y/대각 동일) · **tps 비의존**(30/60/90 모두 120/121) · "
           u"**version 비의존**(0~100 전부 동일). "
           u"디폴트 데이터는 `stat_cached.hp == 1` 이라 판별력 0(전부 RunAway)이고 `stat`/`world.tick`(pub)을 "
           u"열면 갈린다. `history` 의 200000/300000→End 와는 **다른 게이트**(objective Hunt 분기)라 모순 아님. "
           u"부수: `BattleSubPlanGoal` 페이로드는 **struct variant `{ focus }`**."},
    u"D 15 resolved sub_goal 이분탐색")
add(S[15]["resolved"], {
    "was": u"`single_tower_dive_is_viable` — 담당 범위 밖",
    "now": u"★**pub 이라 오라클 호출 가능**(3차 배치D). 쌍둥이 `death_battle::` 쪽은 `in:death_battle` 이라 "
           u"호출 불가이고 15 가 부르는 것은 `single_battle::` 쪽이다. "
           u"내부 사슬 = `single_battle.rs:938 → fight_check.rs:960 → 979 → "
           u"979:59 Option<&PlayerState>::unwrap()` ⟹ **target 이 챔피언이어야 하고 타워를 주면 패닉**. "
           u"**RNG 비의존**(40회). true·false 둘 다 재현했고 판별 축은 미탐색이나 "
           u"9축(RNG·타워hp 1~2M·대상hp·공격력·방어력·사거리·거리·아군수·배치)을 실측 배제했다. "
           u"재현 앵커 = `_verify3\\D\\D3_o11`(true 6/6) ↔ `D3_o10`(false 29/29)."},
    u"D 15 resolved single_tower_dive_is_viable")
add(S[16]["resolved"], {
    "was": u"16 `battle.rs:2397~2421` 원문 — 「재료 부재(전 범위)」로 종결했다",
    "now": u"★★**거짓이었다 — 줄 길이 산술로 뚫렸다**(3차 배치D). "
           u"⚠메인 세션 브리핑이 「재료 부재(전 범위), 다시 파지 마라」로 못박은 항목인데 "
           u"v3 의 실제 `class` 는 `미탐색` 이었고 재료는 존재했다. 복원 결과:\n"
           u"  L2397(86자) = `pub fn max_range_nearly_can_use(champ: &Entity, target: &Entity, "
           u"tick: usize) -> u64 {` — **±0 정확 일치**\n"
           u"  **본문 들여쓰기 = 2칸** 확정, L2398(20자) = `  let mut range = 0;`(변수명은 DWARF `!56148` 정본)\n"
           u"  네 블록 구조 동일·내용 6줄씩, 슬롯별 모델이 **4블록 동시 일치**: "
           u"슬롯1 `27+2n` / 슬롯2 `26+n` / 슬롯3 `72+len(접두)` / 슬롯4 **131(4줄 완전 동일)** / "
           u"슬롯5 `    }` / 슬롯6 `  }`\n"
           u"  슬롯1 유일 정합 표기 = `  if let Some(<name>) = &champ.<name> {`\n"
           u"  DWARF 인라인 접근자 정본 9건(attack_cooldown 1747 · skill_cooldown 1774 · "
           u"skill2_effect 1692 · skill2_cooldown 1789 · ult_effect 1700 · ult_cooldown 1804 · "
           u"Effect::range 25 · Entity::radius 1509 · Option::as_ref<Effect> 741)\n"
           u"⚠남은 모순(**미탐색**): 슬롯1 정합은 필드 접근을 가리키는데 IR 은 메서드 인라인을 말한다(1자 차). "
           u"다음 수단 = `dloc.py` 로 `inlinedAt` 줄 확정."},
    u"D-P5 16 resolved 줄길이로 뚫림")
add(S[17]["resolved"], {
    "was": u"`BattlePlanGoal::base_sub_goal` 내부 — 미탐색",
    "now": u"★**전수 진리표 확보**(3차 배치D, pub). `Response`/`Avoid` → `RunAway` / "
           u"`TryKill`·`Support` 는 산출 동일 / 타워·없는 id → **거리 무관 `Trace{focus:id}`** / "
           u"챔피언 → 거리에 따라 `Trace`/`End`. version·`__1` 무관. "
           u"★갈림은 **가시성이 아니다** — `visible_state`·`can_target`·`invisible_tick`·`hp=0`·"
           u"`visible_map`·`exist_map` **6축을 실측 배제**했다. 남은 미탐색 = 그 판별 축."},
    u"D-P9 17 resolved base_sub_goal 진리표")
add(S[19]["resolved"], {
    "was": u"`get_camp_state` 의 team/캠프 매핑",
    "now": u"★**포인터 동일성 전수표**(3차 배치D). 4캠프는 team 0→`blue_*` / 1→`red_*`, "
           u"**team>=2 는 패닉**(`unreachable!(\"team value must be 0 or 1\")` @jungle.rs:713). "
           u"특례 2건: **Morgard → `epic`(team 무관)** · **Serpen → 전 team 패닉** ⟹ "
           u"`serpen` 필드는 이 접근자로 **도달 불가**. v3 의 추정이 맞았고 특례만 추가된다."},
    u"D-P10 19 resolved get_camp_state")
add(S[18]["resolved"], {
    "was": u"18 epic 헬퍼 3종의 규칙표",
    "now": u"★**진리표 확보**(3차 배치D, pub). `v3_epic_formation_role` **80칸 전수 확정**"
           u"(Gather→전원 Mid/false · Split14{P}→P만 Bottom/true · Split131→p1 Top/true·p2 Bottom/true). "
           u"★`v3_epic_group_line` 은 **TutorialType 의존**이고 `JungleOnly`·`First/Bottom+Split14` 에서 "
           u"**None** 을 낸다 ⟹ 「group_line==None → false」 경로가 **도달 가능함이 실행 확인**됐다. "
           u"부수: `PlayerState::strategy` 실제 시그니처는 "
           u"**`(&self, &mut StdRng, &dyn AbstractGame)`**(인자 3개, player.rs:1576)."},
    u"D-P11 18 resolved epic 헬퍼 진리표")

# ── 오라클 레시피 함정 갱신 ─────────────────────────────────────────
tr = D["shared"].get(u"오라클_레시피_함정")
if tr:
    tr[u"항목"] = [
        u"★★**`GameSetting::default()` 는 거의 전부 0 이다** — 실전 설정 파일과 비교하면 "
        u"**숫자 필드 44개가 0 이 아니다**(width/height 960000 · tick_per_second 60 · champion_radius 10000 · "
        u"visible_distance 130000 · respawn_tick 300 · well_damage 600 …). "
        u"`height==0` 이면 `height − y` 가 u64 **언더플로**해 `is_top_side` 가 상수 true 가 되고, "
        u"`champion_radius==0` 이면 사거리 계열이 전 구간 0 이 된다. "
        u"⟹ 2차의 「챔프 10명 전부 탑사이드 = 재료 부재」·「default 챔피언은 이펙트가 비었다」가 **이 오진**이었다. "
        u"실전값 = `<게임설치>\\bundle_unpacked_full\\setting\\game_setting.game_setting`(JSON, 65키). "
        u"**정본 템플릿 `MIG\\_verify3\\TEMPLATE.rs` 를 복사해서 시작하라**(44줄 대입 + 무결성 출력 포함). "
        u"⚠`serde_json::from_str` 로 직접 읽을 수는 없다 — SDK deps 에 serde rlib 이 둘이라 "
        u"`Deserialize` 바운드가 안 맞는다(실측 E0277).",
        u"★`start_game()` 이 이미 타워·넥서스를 만든다. 그 뒤 `init_tower()`/`init_nexus()` 를 또 부르면 "
        u"전부 2배가 된다 — 실측(`_verify2\\towerchk.rs`): `start_game` 만 = tower_ids 16 · twin 2/2 · "
        u"팀당 8 · 좌표중복 0 / 재호출 = 32 · 4/4 · 16 · 8. "
        u"2차 배치 D 의 「타워 팀당 10개, twin 2쌍 좌표 중복」이 이 오염이었고 3차가 깨끗한 값으로 재측정했다.",
        u"★`GameSetting::default().minion_wave_setting` 이 전부 0이라 `run_tick` 1,200틱에도 "
        u"**미니언 0마리**다(손으로 넣으면 6틱에 팀당 6마리) — 3차 배치A.",
        u"★`world.strategy[team]` 직접 대입은 `PlayerState::strategy()` 에 **반영되지 않는다** → "
        u"`AbstractGame::set_strategy(team, Strategy)` 를 써라 — 3차 배치A.",
        u"★모듈 가시성으로 「오라클 불가」를 판정하지 마라 — 모듈이 `in:game_ai` 라도 아이템이 크레이트 루트로 "
        u"재수출되면 호출된다(`game_ai::defensive_crisis`·`check_kill_die_tick`·`effect_cc_time`·`max_range`·"
        u"`check_favorable_engage_formation` 실측 `v: pub`). 판정은 `_tcx\\pubapi_game_ai.txt` + tcx `v` 로 하고, "
        u"의심되면 **한 번 컴파일해 본다**(`error[E0624]` 가 나오면 진짜 막힌 것).",
        u"★**실전 챔피언 데이터가 열렸다**(3차 배치A): **ChampionInfo 61종 전부 pub + `Action::effect()` pub** ⟹ "
        u"`Game` 없이 실전 챔피언 이펙트 진리표를 짤 수 있다. 단 `::default()` 는 스탯 0(range 0), "
        u"`DataChampionInfo` 만 Default 없음, 5슬롯은 0除算 패닉.",
        u"★입력 완전 통제: `AbstractGameWithCache` 전 필드 pub · `Entity` 전 필드 pub+Clone · "
        u"`Game::world.entity.get_mut`/`values_mut` · `Blackboard` 필드 pub · "
        u"`Game::mode.epic_minion_buff_time`/`next_respawn_tick` pub · `world.tick` pub. "
        u"Entity 를 복제해 좌표·HP·이펙트를 바꾸고 `cache.player_champion[t][p] = Some(&내엔티티)` 로 꽂으면 "
        u"진리표를 자유롭게 짤 수 있다(`Vec<Entity>` 를 cache 보다 먼저 선언해야 수명이 맞는다).",
        u"★`&mut Self` 를 받고 `()` 를 돌려주는 함수는 **구조체 스냅샷 바이트 diff** 로 `writes` 표를 "
        u"기계 대조할 수 있다(필드 pub 불필요).",
        u"⚠**오라클과 IR 독해가 어긋나면 IR 독해를 먼저 의심하라.** 2차 `*_lead`(6/6 불일치)·"
        u"3차 배치C `region_point`(store 0건 오판) 둘 다 IR 독해가 틀렸다. "
        u"`awk`/`sed` 로 IR 구간을 자를 때는 **반드시 다음 `define` 까지** 잡아라.",
        u"⚠`tcxaudit --prose` 스캐너는 `Type+0xNNN` **형태만** 잡는다. 오프셋을 표 형식으로만 적으면 "
        u"**한 건도 스캔되지 않고 감사가 무의미해진다**(3차 배치D 실측 — 총계가 안 변하는 것으로 겨우 발각). "
        u"오프셋을 발표할 땐 `Type+0xNNN` 형태를 최소 한 번 쓸 것. 그리고 **한 행에 오프셋을 묶지 마라.**",
    ]
    N[0] += 1
    print(u"  OK shared.오라클_레시피_함정 갱신(항목 %d)" % len(tr[u"항목"]))

D["meta"]["corrections"].append(
    u"2026-09-11 **3차 반증검증** 4갈래 정정 반영(patch3.py). "
    u"★3차의 잔존 오염원은 사실상 하나였다 — **`logic` 산문이 표/history 의 정정을 못 받는다**"
    u"(배치 D 실오류 5건 중 3건 · A E1/E2 · C E8 · B E2). 재발은 specgate G5/G6 가 막는다. "
    u"판정 반전 1건 = 14 의 TargetMissing 은 **발화한다**(자연상태 v30==v41 12/12). "
    u"★내 도구 결함 2건 = siblings 망글링 파싱이 20개 전부 실패 + specgate G3 가 plan==null 을 통과. "
    u"★내 브리핑 오류 3건 = 05 end_reason 범위 오판 · 07 「두 OR 항」 오기 · 16 「재료 부재」 오판. "
    u"★오라클 오염 원인 확정 = GameSetting::default() 의 44개 필드가 0(정본 템플릿 _verify3\\TEMPLATE.rs 신설).")
with io.open(P, "w", encoding="utf-8", newline="\r\n") as f:
    f.write(json.dumps(D, ensure_ascii=False, indent=1))
print(u"\n총 %d곳 반영 -> %s" % (N[0], P))
