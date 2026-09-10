# -*- coding: utf-8 -*-
u"""patch2b — 2026-09-11 **2차** 반증검증 정정을 본 표에 반영 (배치 C·D = 10~19). 규칙은 patch2a 와 같다."""
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


# ══════════ 10 should_end_object_finish_kill_priority_battle ══════════
print(u"\n[10] should_end_object_finish_kill_priority_battle")
add(S[10]["knobs"], {
    "what": u"\"최근 목격\" 시간창",
    "where": u"game_core blackboard.rs:346 `Blackboard::is_recent_visible` "
             u"(`_gcbc/g07.ll:157005~157050`) — fight_model.rs:1231 호출",
    "value": 120,
    "effect": u"`last_visible[pos] + 120 >= tick` 이면 '최근에 봤다'. tps=60 이라 **2.0초 고정이며 "
              u"`tick_per_second` 로 스케일되지 않는다.** 올리면 적이 시야에서 사라져도 한동안 '아직 근처'로 "
              u"쳐서 오브젝트 마무리 전투를 안 끝내고, 내리면 시야 끊기는 즉시 접는다. "
              u"형제 4개 중 big_action 계열은 같은 배열에 600틱을 쓴다(`_shared.is_recent_visible_4형제`). "
              u"⚠1차가 '노브인데 표에 없다'고 지적한 것이 `resolved` 에만 있었다(2차배치C)"},
    u"10 knobs += 120틱")
rep(10, "logic", u"(★인덱스가 e_team 인 것은 관측 사실 — unknown 참조)",
    u"// `Blackboard[T].last_visible[pos]` = '팀 (1−T) 가 팀 T 의 pos 챔프를 마지막으로 본 틱' "
    u"⟹ T=e_team 이므로 1−T=내 팀 = **「내 팀이 본 적인가」**(`_shared.is_recent_visible` 확정)",
    u"10 logic blackboard 인덱스 의미")
r3 = S[10]["reads"][3]
setk(r3, "note", (r3.get("note") or u"") +
     u" ★실행 확인(2차배치C): `Strategy` 는 24B 이고 raw 덤프의 `+0xf` 바이트가 Debug 의 "
     u"`object_finish` 와 1:1(기본값 1=BattlePriority). `_verify2/C/C_o10b.tsv`",
     u"10 reads[3].note 실행확인")
add(S[10]["resolved"], {
    "was": u"p2 rnd — `strategy` 가 난수를 소비하는지(RNG 부작용) 확인 못 했다",
    "now": u"★**소비하지 않는다**(실행 확정, 2026-09-11 2차배치C, `C_o10b.rs`): "
           u"`PlayerState::strategy` 호출 전후로 같은 시드의 난수열이 **완전 동일(5/5)**, 반환 `Strategy` 도 "
           u"RNG 상태와 무관 ⟹ 이 함수는 **RNG 부작용이 없고 `writes: []` 가 옳다**. "
           u"적용 범위 = Moba·tick 0·`TeamColorStrategy` 12필드 전부 None(기본 팀). "
           u"랜덤 전략은 `thread_rng()` 를 쓰므로(`_shared.TeamColorStrategy_random`) 인자 rnd 와 무관."},
    u"10 resolved RNG 무영향")

# ══════════ 11 v3_fall_back_to_passive ══════════
print(u"\n[11] v3_fall_back_to_passive")
add(S[11]["resolved"], {
    "was": u"1차(REPORT_C §2)의 「&PlayerState·&OperationData·&mut DebugFrameData 는 오라클 구성 불가」",
    "now": u"★**거짓이었다 — 실행으로 반증**(2026-09-11 2차배치C). `C_o11.rs`·`C_o11b.rs` 가 컴파일·링크·실행 "
           u"전부 성공(패닉 0). 관측은 ★**`LegacyPlanHandler` 6,168B 스냅샷 바이트 diff**"
           u"(필드가 pub 이 아니어도, 반환형이 `()` 여도 된다). 실행 일치: "
           u"①`version<2` 게이트(ver 0/1 → **diff 0바이트**) ②DeathMatch 즉시 return(diff 0바이트) "
           u"③SingleLane → `BigPlan` tag 4 + `+0x8=0/+0x10=inttoptr 8/+0x18=0/+0x20=0/+0x21=1` "
           u"**1바이트도 안 어긋남** ④`mf_swap=(29,tick)` ⑤`v3_lapse_passive_fallbacks=1` "
           u"⑥writes 5구역(0x5e8·0x768·0x1610·0x1618·0x1628) **밖 변화 0건** "
           u"⑦튜토리얼 화이트리스트 9×5 = **45칸 전수 일치**. "
           u"⟹ 이 명세의 신뢰도는 「IR 추론」이 아니라 **「오라클 실행」**이다. 정정 0건."},
    u"11 resolved 오라클 전수 검증")

# ══════════ 12 handle_chat ══════════
print(u"\n[12] handle_chat")
pp = S[12]["signature"]["params"]
tgt = [p for p in pp if u"Chat" in str(p.get("type"))]
for p in tgt:
    setk(p, "type", u"Chat(24B, enum 57 variant) — ★**값 전달(by value)**. ~~&Chat~~ 은 오기",
         u"12 signature Chat 값 전달")
    setk(p, "note", (p.get("note") or u"") +
         u" ★근거: tcx sig `(…, game_core::Position, game_core::Chat, bool, &mut DebugFrameData)` + "
         u"**rustc 실컴파일 거부**(expected `Chat`, found `&Chat`; `_verify2/C/C_o12.rs` 1차 시도). "
         u"24B 라 Win64 ABI 가 간접전달해 IR 에 포인터로 보인 것. `handle_chat_inner` 도 값 전달(2차배치C)",
         u"12 signature Chat note")
if u"chat: &Chat" in S[12]["logic"]:
    rep(12, "logic", u"chat: &Chat", u"chat: Chat", u"12 logic chat 값 전달")
for it in [
    {"what": u"★ff_call_* 8칸 계측표 — 「콜을 받고도 왜 안 갔는가」 분해기",
     "where": u"LegacyPlanHandler +0x15b8 recv / +0x15c0 ignored / +0x15c8 in_battle / +0x15d0 no_help / "
              u"+0x15d8 too_far / +0x15e0 low_hp / +0x15e8 bail / +0x15f0 join "
              u"(chat.rs:141/144/151/204/205/206/239/236)",
     "value": u"usize × 8",
     "effect": u"`handle_chat` 이 게이트를 통과시킨 뒤 inner 가 콜을 어떻게 처리했는지가 이 8칸에 전부 남는다 — "
               u"**인게임 검증 지표로 즉시 쓸 수 있다.** 실행 확인(2차배치C, `C_o12.tsv`): Battle 계열 콜이 "
               u"inner 에 닿을 때마다 `recv` +1, 동반해 `too_far` 또는 `bail` 이 +1 로 실제 증가"},
    {"what": u"★합류 거절 후 「글로벌 궁 예약」 (명세 본문에 통째로 없던 구간)",
     "where": u"chat.rs:245~290 (`_gaibc/m13.ll:30495~30620`) → "
              u"LegacyPlanHandler+0x530 `pending_global_ult_target: Option<(usize,usize)>`(24B, tcx 확정)",
     "value": u"대상 `Entity.id`(+0x538) + 만료틱 `tick + tps*3`(+0x540)",
     "effect": u"콜을 거절한다고 아무것도 안 하는 게 아니라 **거절 대신 글로벌 궁을 3초짜리로 예약한다** "
               u"(조건: chats 포화 ∧ `Entity::can_ult` ∧ `+0x530==None` ∧ 거리>199999 ∧ "
               u"`iter_champions(..).count()==0`). 소비처 = `auction::get_small_action` 1곳. "
               u"재구현에서 빠지면 궁 사용 타이밍이 통째로 달라진다. "
               u"⚠오라클 미검증 — 기본 챔피언(이펙트 공백)은 `can_ult` 가 안 서서 14,950행 전수에서 "
               u"한 번도 안 바뀌었다(입력 판별력 부재)"},
]:
    add(S[12]["new_knobs"], it, u"12 new_knobs += %s" % it["what"][:26])
for it in [
    {"base": "LegacyPlanHandler", "offset": "0x858", "name": "pending_trace_events.cap",
     "value": u"RawVec::grow_one 결과",
     "note": u"트레이스 ON 첫 push 에서 재할당. logic 에는 적혀 있었으나 writes 표에 빠져 있었다. "
             u"실측 diff 0x858..0x859 (2차배치C)"},
    {"base": "LegacyPlanHandler", "offset": "0x860", "name": "pending_trace_events.ptr",
     "value": u"새 힙 주소", "note": u"동상. 실측 diff 0x860..0x866 (2차배치C)"},
]:
    add(S[12]["writes"], it, u"12 writes += %s" % it["name"])
add(S[12]["resolved"], {
    "was": u"1차의 「오라클 인자 구성 불가」 + chat_allowed/position_exists 표의 신뢰도",
    "now": u"★**실행으로 전수 검증**(2026-09-11 2차배치C, `C_o12.rs` → **14,950행**). "
           u"①자기발화 게이트: `recv==from` 인 **전 행에서 diff 0바이트**(위반 0건) "
           u"②`position_exists`: tutorial 9종 × from 5종 **45칸 전수 일치**"
           u"(Top{0,2,7,8}/Jungle{0,6,8}/Mid{0,4,5,7,8}/Bottom·Support{0,1,3,5,7,8}) "
           u"③트레이스 게이트: Off 는 push 0, **Summary·Detailed 둘 다** len +1 ⟹ "
           u"`is_enabled() = (trace_level != Off)` 확정 ④`misunderstood` 는 이 본문에서 분기하지 않는다 "
           u"⑤stake 줄번호 = chat.rs:212/215/216. "
           u"⚠부수 함정 재확인: m13.ll:33188 의 `store i8 21` 은 `ff_battle_exit` 가 아니라 "
           u"**`mf_swap.0`(+0x1610)** 이다(tcx 레이아웃으로 재확인)."},
    u"12 resolved 14,950행 전수")

# ══════════ 13 target_bush_v30 ══════════
print(u"\n[13] target_bush_v30")
nk = S[13]["new_knobs"][0]
setk(nk, "effect",
     u"★이 함수(`LineGankCoverPlan::target_bush_v30`)는 2~21 만 낸다. "
     u"⚠**「1/22/23/24 영구 제외」는 이 함수 한정**이고 갱커 쪽 형제 "
     u"`LineGankerPlan::target_bush_v41` 이 **7·23 을 실제로 반환**한다(2026-09-11 2차배치C 실행 확인, "
     u"`C_o1314_t0.tsv` Bottom lead6). 두 함수를 합친 실제 미사용 = **1, 5, 10, 19, 22, 24** "
     u"(적용 범위: v30 + v41 기준. 좌표를 돌려주는 `target_bush`(→(u64,u64))·`near_jungle_bush` 는 미탐색). "
     u"여기 ID 를 바꾸면 새 은신처가 살아난다",
     u"13 new_knobs[0].effect 범위정정")
add(S[13]["resolved"], {
    "was": u"명세 13 의 반환표가 IR 추론뿐이었다",
    "now": u"★**실행 확인**(2026-09-11 2차배치C, `C_o1314.rs` — `LineGankCoverPlan::sub_plan` 이 pub 이라 "
           u"`SubPlan::Hide{bush}` 를 그대로 읽었다). tick 0(타워 전부 생존·1차타워·`nearest_enemy=None`·"
           u"챔프 `is_top_side=true`): **Top 3/6 · Mid 11(team 무관) · Bottom 15/20** — L156/L171/L186 예측과 일치. "
           u"★특히 **Mid=11 은 `is_top_side` 극성 정정이 옳다는 실행 증거**다(정정 전 표대로면 14 가 나왔어야 한다). "
           u"나머지 분기(타워 전멸 2/16·4/17·9/21, 2차타워, `nearest_enemy` 스왑, Mid 봇사이드 8/12·13/18·14)는 "
           u"**미탐색** — 범위: 기본 챔피언으로는 타워를 못 부수고 챔피언 10명이 전부 `is_top_side=true`."},
    u"13 resolved 실행 확인")

# ══════════ 14 LineGankerPlan::update ══════════
print(u"\n[14] LineGankerPlan::update")
nk1 = findk(14, "new_knobs", "target_bush_v41 전표")
setk(nk1, "value", nk1["value"] +
     u"\n  ★**실행 재현**(2026-09-11 2차배치C): Top·Bottom 14칸 전부 일치, Mid 는 s=false 가지 7칸 일치"
     u"(team0 [17,11,11,11,11,4,4] / team1 [4,11,11,11,11,17,17]). **lead=7 은 실제로 패닉**(`lead<7` 확인). "
     u"s=true(봇 사이드) 가지는 **재료 부재** — `start_game` 직후 양 팀 챔프 10명이 전부 `is_top_side=true`"
     u"(team0 (15000,913000)…, team1 (913000,15000)…). 미탐색 = 챔프를 봇 사이드로 옮긴 상태. "
     u"프로브 `_verify2/C/C_o1314.rs`",
     u"14 new_knobs v41 실행 재현")
add(S[14]["new_knobs"], {
    "what": u"★갱커의 부시 선택기는 **셋**이다 (1차는 둘로 적었다)",
    "where": u"`target_bush_v30`(ganker.rs:248, →usize) = `update`(:54) 인라인 / "
             u"`target_bush_v41`(ganker.rs:310, →usize) = `sub_plan`(:380, m08.ll:94960) "
             u"**+ `next_plan`(:144, m08.ll:95509)** / "
             u"★**`target_bush`(ganker.rs:351, →(u64,u64) 좌표쌍)** = `next_plan`(:142) 인라인",
    "value": u"—",
    "effect": u"1차는 v41 호출부를 `sub_plan` 1곳으로 적었으나 **실제 2곳**이고, `next_plan` 은 좌표를 "
              u"돌려주는 **세 번째 선택기**(`min_by_key` + `map_regions::near_jungle_bush`)를 함께 쓴다 ⟹ "
              u"「이 플랜의 목표 부시」를 재구현할 때 `update`/`sub_plan`/`next_plan` 이 **서로 다른 세 규칙**을 "
              u"쓴다는 것을 놓치면 안 된다. 부수: `LineGankCoverPlan` 에는 v41 이 없다(v30 + `target_bush` 둘뿐) "
              u"= 커버 플랜은 항상 v30 (2026-09-11 2차배치C)"},
    u"14 new_knobs += 선택기 셋")
add(S[14]["resolved"], {
    "was": u"`GoalData::has_near_line_enemy` 내부 — 1차는 `region_dist < 2` 구조로 추정만",
    "now": u"★**확정**(2026-09-11 2차배치C, IR 전수독해 + 오라클 3/3 MATCH):\n"
           u"  ∃i∈0..5: enemy_region[i]=Some(info) ∧ ∃k∈{2,3,4}: "
           u"map.region_dist(map.line_region(line, 0, k), info.region) < 2\n"
           u"근거 `_gaibc/m09.ll:3948~4260`(5슬롯 × k 3개 = `line_region` 15회 완전 언롤, "
           u"`region_dist` 표 = `MapDef+21720(0x54d8)` `[[usize;27];27]`, 경계검사 <27 2개, 조기탈출 `ult ..,2`). "
           u"실행 부수확정: `last_known` 은 **판정에 미사용** · 5슬롯 대등 · 빈 GoalData → false · "
           u"**`side` 인자 리터럴 0 은 무해**(`lane_seq(line,1)` 이 역순이라 가운데 3칸 k=2,3,4 집합이 동일: "
           u"Top{22,17,18}/Mid{4,6,3}/Bottom{8,16,23}). "
           u"실측 진리표 = Top{5,7,10,13,17,18,20,22} / Mid{1,2,3,4,6,7,10,11,12,13,14,15,19} / "
           u"Bottom{2,8,9,14,15,16,23,25}. 프로브 `_verify2/C/C_o_near.rs`."},
    u"14 resolved has_near_line_enemy 확정")
add(S[14]["resolved"], {
    "was": u"`*_lead` 를 누가 갱신하는지",
    "now": u"★**갱신 주체 = `AbstractGameWithCache::new_with_prev_cache`**(game-core/simulation.rs:1780) — "
           u"`_gcbc/g15.ll:104397~104406` 에 6개 store(+8640/8648 top, +8656/8664 mid, +8672/8680 bottom) ⟹ "
           u"**캐시 생성마다 재계산되는 파생값**이지 누적 상태가 아니다. 읽기 접근자 = "
           u"`line_lead(team, line)`(simulation.rs:1928, mir=1). 재료 = "
           u"`region_point: [i32;27] @ +0x2218(8728)`(tcx + `memcpy(dst=+8728,108B,align 4)` 일치). "
           u"⚠**산출식은 못 닫았다** — 2차배치C 가 IR(`g15.ll:104280~104360`)에서 읽은 규칙"
           u"(`region_point[r] < -2` 인 동안 전진)이 **실행과 6/6 불일치**했다(예측 0/3 vs 실측 전부 2). "
           u"tick 0 실측: 전 라인·전 팀 `lead=2`, "
           u"`region_point=[8,3,0,-6,6,7,0,0,6,7,-3,-3,-7,3,2,-2,0,0,-6,7,-7,-3,6,-6,3,-7,-8]`. "
           u"미탐색 범위 = `_gcbc/g15.ll:104100~104410` 전수 독해. "
           u"★교훈: 오라클과 IR 독해가 어긋나면 **IR 독해 쪽을 먼저 의심하라**(이번엔 IR 독해가 틀렸다)."},
    u"14 resolved *_lead 갱신자")
add(S[14]["resolved"], {
    "was": u"형제 `is_cancel`(ganker.rs:69) 의 의미 — 명세는 phase=Cancel(8) 만 서술",
    "now": u"★**`is_cancel()` ≠ `phase == Cancel`**(2026-09-11 2차배치C). 실측 = "
           u"WaitResponse→**true** / Setup→false / Cancel→true / ChangeJungle→false. "
           u"MIR 확증(`mirdump_game_ai.txt:12918`, ganker.rs:70) = discriminant 를 **2(Cancel)** 와 "
           u"**0(WaitResponse)** 두 값에 비교하는 `||` 식. "
           u"`LineGankerPhase` = {WaitResponse 0, Setup 1, Cancel 2, ChangeJungle(JungleType) 3}(선언 인덱스) / "
           u"메모리 태그는 니치로 ChangeJungle 0..5, WaitResponse 6, Setup 7, **Cancel 8**(명세의 8 은 맞다)."},
    u"14 resolved is_cancel")
add(S[14]["resolved"], {
    "was": u"`update` 의 취소 경로(LowHpSelf / TargetMissing)를 실행으로 확인했는가",
    "now": u"★**미검증(재료 부재)**. `C_o1314.rs` 로 phase 4종 × 라인 3 × 팀 2 × lead 7 = **168회 호출**, "
           u"phase 변화 0 · 패닉 0. tick 0 은 HP 만복이라 `hp_ratio<41` 이 안 서고, 챔피언이 목표 부시 밖이라 "
           u"도착 판정도 안 선다 ⟹ 1차의 「v30≠v41 이라 TargetMissing 이 원리적으로 발화하지 않는다」는 "
           u"**실행으로 확증도 반박도 못 했고 논거는 여전히 IR 뿐이다.** "
           u"미탐색 = Entity 좌표를 목표 부시 셀로 옮긴 상태 / HP 조작."},
    u"14 resolved 취소경로 미검증")

# ══════════ 15 single_try_engage ══════════
print(u"\n[15] single_try_engage")
rep(15, "logic",
    u"앞 6칸 = 이름있는 타워 슬롯(Option), 뒤 = 나머지 타워 슬라이스. 넥서스 제외.",
    u"앞 6칸 = [top_tower, mid_tower, bottom_tower, top_tower2, mid_tower2, bottom_tower2][team]\n"
    u"          //     (+0x180/0x1a0/0x1c0/0x190/0x1b0/0x1d0, 이 순서 그대로) .flatten()\n"
    u"          //   뒤 = twin_towers[team](+0x130, bumpalo Vec<&Entity>).iter().copied()  "
    u"— ~~나머지 타워 슬라이스~~ 아님\n"
    u"          //   nexus(+0x170)는 별도 필드라 실제 제외. 실측 팀당 10개(6+4, twin 2쌍은 좌표 중복)",
    u"15 logic 249행 iter_towers 6칸")
add(S[15]["resolved"], {
    "was": u"`engage_requires_dive` 의 실제 거리식(`Effect::is_in_range` 안) — 미탐색",
    "now": u"★**확정**(2026-09-11 2차배치D). `_gcbc/g06.ll:51634~51782`(is_in_range) + "
           u"`51807~51942`(is_in_range_ex) + MIR + DWARF `!63248~!63260`(인자 이름 원문):\n"
           u"  // effect.rs:63~65\n"
           u"  pub fn is_in_range(&self, caster: &Entity, target: &Entity) -> bool {\n"
           u"    self.is_in_range_ex(caster, target, caster.x, caster.y, target.x, target.y, 0)\n"
           u"  }\n"
           u"  // effect.rs:78~84\n"
           u"  pub fn is_in_range_ex(&self, caster, target, cx, cy, tx, ty, offset: u64) -> bool {\n"
           u"    let r        = self.range(caster) + offset + self.range_adjust(caster, target);\n"
           u"    let caster_r = if self.casting == CastingType::Targeting { caster.radius() } else { 0 };\n"
           u"    let total    = r + caster_r + target.radius();\n"
           u"    distance_sq(cx, cy, tx, ty) <= total * total\n"
           u"  }\n"
           u"구성요소(전부 MIR 정본): `Effect::range(&self,e)` = `range(+0x10) + growth_range(+0x18) * "
           u"(e.level(+0x5c8) − 1) + e.stat_buff_cached.range(+0x438)`(effect.rs:26) / "
           u"`Effect::range_adjust` = `Arc<dyn EffectType>` **vtable +0xe8** 위임(effect.rs:30, 반환 u64) / "
           u"`Entity::radius()` = `if radius_mult(+0x470)==0 { radius(+0x680) } else "
           u"{ radius*(100+radius_mult)/100 }`(entity.rs:1511~1515) / "
           u"`utils::distance_sq` = `abs_diff²+abs_diff²`(utils.rs:6~10, sqrt 없음) / "
           u"`CastingType` = 0 Targeting/1 Position/2 Direction/3 None(4B Direct).\n"
           u"⟹ `engage_requires_dive` = '적 타워 중 하나라도 `attack_effect` 가 Some 이고 "
           u"`dist_sq(tower,e) <= (eff.range(tower) + eff.range_adjust(tower,e) + tower.radius() + e.radius())²`'. "
           u"★**오라클 game==mine 900/900 MATCH**(`D2_oracle5.exe`, 챔프10+타워20 전 조합). "
           u"진리표 300쌍 중 100쌍 true = 'target 이 적 타워 자신'과 정확히 일치 ⟹ "
           u"`1 − player.info.team` 극성도 실행 확정. `iter_towers_without_nexus(team)` 는 **팀당 정확히 10개**. "
           u"적용 범위 = 0.5.8 SDK. `offset != 0` 로 부르는 호출부는 미탐색(노브 후보)."},
    u"15 resolved Effect::is_in_range 900/900")
add(S[15]["resolved"], {
    "was": u"`SinglePlanBattle::update` 가 `sub_goal` 을 KitingBack/RunAway/End 로 떨구는 조건",
    "now": u"★**부분 확정 — 52사이트 전량 지도 + 게이트 13개**(2026-09-11 2차배치D). "
           u"구조: `update`(single_battle.rs:121~343)는 **얇은 전처리 + 조기탈출 게이트**이고 본 판단은 "
           u"`SinglePlanBattle::update_v32`(single_battle.rs:345~870, `vis=in:single_battle`)가 rs:342 에서 "
           u"통째로 인라인된 것(IR `_gaibc/m05.ll:27043~34389`). `sub_goal`(+0x58) 스토어 52사이트 분포 = "
           u"End(7) 14 · RunAway(4) 10 · Kiting(2) 10 · KitingBack(3) 9 · Trace(0) 5 · 계산값 4. "
           u"★**`update` 본체(rs<345)가 내는 값은 RunAway·End 뿐** — `single_try_engage` 의 폐기 결정 절반이 "
           u"여기서 난다. 주요 게이트: "
           u"128→132/133 `is_enemy_well_danger` → RunAway + `well_runaway = tick + tps*5` / "
           u"140→141 `well_runaway` 유효 → RunAway / 157 `tick−start_tick < 30 || help_called` / "
           u"199 `select(<술어2개>, RunAway, End)`(술어는 미탐색) / "
           u"211 `should_end_object_finish_kill_priority_battle`(=명세 #10) → End / "
           u"222·234 objective Hunt 분기 + 거리 ≥200000 → End / 245 거리 ≥300000 → End / "
           u"282 region 밖 → End / 316→317 sub_goal==RunAway → End 승격 / 333 `tick−120 > start_tick` → End / "
           u"484(v32) 스택 로컬 +24==0 → End / 505(v32) `dist_sq > max_range_cached²` → RunAway / "
           u"558(v32) `!is_unreasonable_tower_dive_enemy || <플래그>` → End. "
           u"전량 좌표 = `_verify2\\D\\subgoal_map.txt`. 미탐색 = rs:296·301 및 v32 636~861 의 40여 사이트 개별 술어."},
    u"15 resolved sub_goal 52사이트")
for it in [
    {"what": u"적 우물 도피 지속", "where": u"single_battle.rs:133 (m05.ll:27343 `mul i64 %149, 5`)",
     "value": u"tps × 5 = 5초", "effect": u"우물 근처에서 강제 후퇴를 유지하는 시간 (2차배치D)"},
    {"what": u"교전 초반 유예", "where": u"single_battle.rs:157 (m05.ll:27390 `icmp ult .., 30`)",
     "value": u"30틱(0.5초)", "effect": u"붙자마자 후퇴/종료로 빠지는 것을 막는 창 (2차배치D)"},
    {"what": u"오브젝트 이탈 종료 거리",
     "where": u"rs:222 (m05.ll:29114 `ugt .., 39999999999`) / rs:245 (m05.ll:29219 `ugt .., 89999999999`)",
     "value": u"200000 / 300000", "effect": u"사냥 중 이 거리 넘게 벌어지면 종료 (2차배치D)"},
    {"what": u"교전 최대 지속", "where": u"rs:333 (m05.ll:34365/34375 `usub.sat(tick,120)`)",
     "value": u"120틱(2초)", "effect": u"넘으면 End (2차배치D)"},
]:
    add(S[15]["new_knobs"], it, u"15 new_knobs += %s" % it["what"])

# ══════════ 16 max_range_nearly_can_use ══════════
print(u"\n[16] max_range_nearly_can_use")
for old, new, lab in [
    (u"champ.level > 2", u"champ.level >= 3   // ~~> 2~~ 는 IR 접힘 표기(MIR 정본 = Ge(level, 3))",
     u"16 logic level>=3"),
    (u"champ.level > 4", u"champ.level >= 5   // ~~> 4~~ 는 IR 접힘 표기(MIR 정본 = Ge(level, 5))",
     u"16 logic level>=5"),
    (u"champ.ty.attack_cooldown()",
     u"champ.attack_cooldown()   // ★`Entity::attack_cooldown(&self)` 가 self.ty 를 match — "
     u"~~champ.ty.…~~ 는 IR 표기(MIR entity.rs:1747)",
     u"16 logic attack_cooldown 수신자"),
]:
    if old in S[16]["logic"]:
        rep(16, "logic", old, new, lab)
    else:
        print(u"  -- 건너뜀: %s" % lab)
add(S[16]["resolved"], {
    "was": u"14-way switch 에서 태그 3(Nexus)·0(None)에 게이트가 없는 이유",
    "now": u"★**확정**(2026-09-11 2차배치D). `Entity::attack_cooldown`(entity.rs:1747~1764) 14-arm 전량이 "
           u"MIR 에 있다 — **태그 3(Nexus)은 1761 에서, 태그 0(None)은 1762 에서 `const 0_usize` 를 반환**한다 ⟹ "
           u"「게이트가 없다」가 아니라 **「그 팔이 0 을 반환해 `0 <= tick` 이 항상 참이라 접힌 것」**. "
           u"덤으로 `EntityType` 태그 전표가 독립 재확인됐다(0 None·1 Minion·2 Tower·3 Nexus·4 Jungle·5 Epic·"
           u"6 Serpen·7 Ghoul·8 SmallJiangshi·9 Bear·10 Eagle·11 Revenant·12 Illusion·13 Champion) — "
           u"명세의 14-way switch 표와 **완전 일치**."},
    u"16 resolved attack_cooldown 14-arm")
add(S[16]["resolved"], {
    "was": u"이 함수의 사거리식이 엔진의 실제 판정식과 같은가",
    "now": u"★**다르다**(2026-09-11 2차배치D). 이 함수(m10.ll 블록 %34~%74)는 **`champ.radius()` 를 무조건** "
           u"더하는데, 엔진의 실판정 `Effect::is_in_range_ex` 는 **`casting == Targeting` 일 때만** "
           u"더한다(g06.ll:51836 `icmp eq i32 .., 0`) ⟹ `Position`/`Direction`/`None` 시전 이펙트에 대해 "
           u"**AI 의 '거의 닿는다' 추정치가 엔진 판정보다 `champ.radius()` 만큼 후하다(과대추정)**. "
           u"적용 범위: IR 대조로 확정. 오라클은 판별력 0(디폴트 데이터의 이펙트가 전부 Targeting 이라 gatediff=0)."},
    u"16 resolved 과대추정")

# ══════════ 18 v3_epicops_buff_window ══════════
print(u"\n[18] v3_epicops_buff_window")
if u"/*p5=*/true" in S[18]["logic"]:
    rep(18, "logic", u"/*p5=*/true",
        u"WavePriorityObject::Serpen   /* ★~~/*p5=*/true~~ 는 오독 — i1 은 1B 열거형의 ABI 표현. "
        u"tcx sig = fn(&PlayerState,&OperationData,&GoalData,&TeamPlan,WavePriorityObject) -> bool, "
        u"0=Morgard/1=Serpen */",
        u"18 logic WavePriorityObject")
else:
    add(S[18]["resolved"], {
        "was": u"`is_object_being_taken_by_enemy` 의 5번째 인자 리터럴 true 의 의미",
        "now": u"★**확정**(2026-09-11 2차배치D): 5번 인자는 `bool` 이 아니라 "
               u"**`WavePriorityObject`**(1B enum, 0=Morgard/1=Serpen). IR 의 `i1 true` 는 그 ABI 표현이므로 "
               u"`true` = **`Serpen`**. tcx sig(objective_helpers.rs:313) + `tcxdict --enum WavePriorityObject`. "
               u"세르펜 분기에서 세르펜을 묻는 것이라 자기정합적."},
        u"18 resolved WavePriorityObject")

# ══════════ 19 best_jungle_goal ══════════
print(u"\n[19] best_jungle_goal")
add(S[19]["resolved"], {
    "was": u"`tick_per_second=0` 한계가 1차 결과를 오염시켰는가",
    "now": u"★**오염 없음**(2026-09-11 2차배치D). `GameSetting::tick_per_second` 는 pub 필드라 "
           u"`MapDef::moba` 호출 **이전에** 60 을 대입하면 된다. 재실행 결과 담당 5개 전부 1차와 동일: "
           u"19 **10/10 MATCH**(Bee), `is_cleared`/`is_side_cleared` 전부 false. "
           u"⟹ tps=0 한계는 **실재하지만 이 함수의 1차 결과를 바꾸지 않았다**."},
    u"19 resolved tps=60 재실행")

D["meta"]["corrections"].append(
    u"2026-09-11 **2차 반증검증** 배치 C·D(10~19) 정정을 본 표에 반영(patch2b.py). "
    u"실오류 = 12 p7 `&Chat` → `Chat`(값 전달, rustc 실컴파일 거부) / 18 5번 인자 bool → WavePriorityObject / "
    u"16 level>2·>4 → MIR 정본 >=3·>=5. "
    u"신규 확정 = 15 Effect::is_in_range 완전식(900/900)·sub_goal 52사이트+노브4 / "
    u"14 has_near_line_enemy(3/3)·*_lead 갱신자·부시 선택기 셋·is_cancel / 12 14,950행 전수+노브2 / "
    u"11 오라클 전수(정정 0) / 10 120틱 노브+RNG 무영향 / 16 AI 과대추정. "
    u"★1차의 「오라클 구성 불가」가 2세션 연속 거짓이었다.")
with io.open(P, "w", encoding="utf-8", newline="\r\n") as f:
    f.write(json.dumps(D, ensure_ascii=False, indent=1))
print(u"\n총 %d곳 반영 -> %s" % (N[0], P))
