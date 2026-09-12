# -*- coding: utf-8 -*-
u"""patch4 — 2026-09-11 **4차** 반증검증 정정을 `_spec/specs20.json`(v2 정본)에 반영.

4차 집계(판정반전을 오류로 세면): **실오류 12건** = 스펙 8 + 판정반전 3 + 근거 오귀속 1.
배치 A·C 는 실오류 0 / B 2+1 / D 6+3.

## 이 라운드의 구조적 소득 — G6 사각지대가 세 방향으로 드러났다
 ① **표에 남은 옛 값(역방향)**: 3차 P-4 가 `logic` 만 고쳐 `reads[8].name` 에 `ty.Tower.0` 이 남았다(D-E3).
 ② **`knobs[].effect`**: G6 는 `logic` 만 보므로 노브의 분기 극성 오류를 못 잡는다(B-E2).
 ③ **`logic` 오류가 `callees` 자동생성을 오염시킨다**: `logic` 이 메서드를 필드로 적어
    `harvest_callees` 가 4개를 못 잡았다(D-E2). ⟹ `callees` 는 `logic` 의 오류를 **상속**한다.
⟹ 게이트 G7/G8/G9 로 막는다(specgate.py).

## 대상 필드 주의
v2 스키마는 `unknown[](문자열)` · `resolved[]{was,now}` · `reads/writes[]{base,offset,name,note}` ·
`knobs/new_knobs[]{what,where,value,effect}` 다. v3 의 `open/closed/mem` 는 mkspec3 산출물이므로
**여기를 고쳐야** 다음 빌드에 반영된다. `closed[].why` 는 `closelist.py` 소관이라 거기서 고친다.
"""
import io, json, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
P = "_spec/specs20.json"
D = json.load(io.open(P, encoding="utf-8"))
S = D["specs"]
N = [0]


def ok(label):
    N[0] += 1
    print(u"  OK %s" % label)


def rep(i, key, old, new, label):
    assert old in S[i][key], u"못 찾음: %s\n    %r" % (label, old[:70])
    S[i][key] = S[i][key].replace(old, new)
    ok(label)


def setf(obj, key, val, label):
    obj[key] = val
    ok(label)


def add(i, field, item, label):
    S[i].setdefault(field, []).append(item)
    ok(label)


# ══════════════════════ 배치 B (05~09) ══════════════════════
print(u"\n[배치 B] 05~09 — 실오류 2 + 근거 오귀속 1(closelist) + 새 발견 7")

# ── B-E1 : 3차 정정이 항목 이동만 되고 본문은 옛 결론 그대로였다 ──
u0 = S[9]["unknown"][0]
OLD = (u"나머지 7곳(plan_legacy::handler)은 **런타임 version SSA 값**을 넘긴다 "
       u"⟹ 주 경로에서는 **살아 있는 버전 게이트**다. 재구현 시 0 하드코딩 금지. "
       u"미탐색 = 피호출자 내부의 version 분기")
assert OLD in u0, u0[-200:]
S[9]["unknown"][0] = u0.replace(OLD, (
    u"나머지 7곳(plan_legacy::handler)은 **런타임 version SSA 값**을 넘긴다. "
    u"단 ~~⟹ 주 경로에서는 살아 있는 버전 게이트다. 재구현 시 0 하드코딩 금지~~ 는 "
    u"**결론이 틀렸다**(2026-09-11 4차 배치B 재실측: version 12종 diff=0). "
    u"관측(호출부 8곳·리터럴 0 은 1곳)은 맞지만 **피호출자 2단이 모두 `i64 poison`** 이라 "
    u"**이 체인 전체에서 죽은 인자**다 ⟹ 같은 스펙 `signature.params[0].note`(ev2)와 일치한다. "
    u"⚠3차 정정이 **항목 이동만 되고 이 문장은 옛 결론 그대로 남아 있었다** — 4차 배치B 적발"
    u"(정정이 `resolved`/`params` 에만 들어가고 원문에 안 닿는 누출의 재발). "
    u"미탐색 = 피호출자 내부의 version 분기(범위: `minion_wave_risk.rs` 하위)"))
ok(u"B-E1 09 unknown[0] 옛 결론 문장 정정")

# ── B-E2 : 분기 극성 오류 (G6 가 못 보는 knobs[].effect) ──
k = S[9]["new_knobs"][3]
assert k["what"] == u"에픽버프 위험표 게이트", k
setf(k, "value",
     u"≠0 이면 엄격표 — **단 09 경로에서는 이 값이 판정을 못 바꾼다**(아래)",
     u"B-E2 09 new_knobs[3].value")
setf(k, "effect",
     u"★★**극성 정정(4차 배치B)**: ~~강제로 0 을 읽게 하면 항상 완화표 / 강제 ≠0 이면 항상 엄격표~~ 는 "
     u"**09 에서 성립하지 않는다.** 09 호출부는 `champion_action` 에 **리터럴 `true`**(m15.ll:35472)를 넘기고 "
     u"표 선택이 `or(ca, buff≠0)`(m07.ll:48291 `%36 = or i1 %6, %35`) 이므로 "
     u"**09 경로는 buff 값과 무관하게 항상 엄격표**다.\n"
     u"buff 의 실제 개입 지점은 한 단 아래 `enemy_minion_line_action_damage_at`(minion_wave_risk.rs:132)이 "
     u"모델을 **`enemy_minion_wave_risk_damage_at` 으로 통째 위임**하는 것이다 — "
     u"오라클 `buff=0 → 73` / `buff≠0 → 128`(=wave_risk), `OR` 8/8 MATCH. "
     u"⟹ **이 노브를 돌리려면 09 가 아니라 그 아래를 건드려야 한다.**\n"
     u"⚠같은 사실이 `resolved` 에는 이미 옳게 적혀 있었다 = **G6 가 `logic` 만 봐서 "
     u"`knobs[].effect` 를 못 잡는 사각지대**(이 라운드의 구조적 소득 → 게이트 G8)",
     u"B-E2 09 new_knobs[3].effect 분기 극성")

# ── B-N1/N2/N3/N4/N6/N7 : 새 발견 ──
add(9, "resolved", {
    "was": u"`enemy_minion_line_action_damage_at` 본문 — 미니언이 필요해 오라클이 막혀 미탐색",
    "now": u"★**전문 복원 + 미니언 생성 우회 확립**(4차 배치B). minion_wave_risk.rs:130~231, "
           u"DWARF 지역변수명·줄번호 실측. 반환 상한 = `damage.min(hp.saturating_mul(2).max(1))` "
           u"**오라클 10/10**, `window_tick` 하한 tps/2, 가중 3단(100/70/55) × 거리가중(100/100/70/50), "
           u"`range_offset` 상한 24000/16000/+8000.\n"
           u"신규 오프셋: `Entity+0x11a`(Minion.line) · `+0x4c0`(attack_effect 판별자, −1=None) · "
           u"`+0x88/+0x90`(nearest_enemy) · `+0x640`(move_speed) · `GameSetting+0x1410`.\n"
           u"★**우회 레시피**: `minion_wave_setting` 16필드 + `melee/range_minion` 실전값을 주입하고 "
           u"`run_tick` 600틱 → **팀당 9마리**. 이 프로젝트에서 `damage_at > 0` 을 처음 얻었다 "
           u"(정본 = `_shared.오라클_레시피_함정`)."}, u"B-N1/N2 09 resolved 미니언 전문+우회")
add(9, "resolved", {
    "was": u"`logic` 이 IR 과 일치하는지 — 2차에 오라클 400/400(재현→명세 방향)까지만 확인",
    "now": u"★**독립 재구현 대조 1800/1800 MATCH**(true 1337 / false 463, 4차 배치B). "
           u"2차보다 강한 방향(**명세 → 재구현**)이다.\n"
           u"⚠부수 정정: 3차가 `_shared` 에 적은 `champion_action || has_epic_buff` 는 **IR 과 좌우가 반대**다"
           u"(간접 vtable 호출 + 패닉 경로가 있어 투기 불가 ⟹ buff 가 좌항). 논리값 동일·재현 무영향."},
    u"B-N4/N7 09 resolved 1800/1800 + 좌우 정정")
add(7, "resolved", {
    "was": u"07 `Hide`(태그 9) 경로는 `target_bush` 가 private 이라 오라클 미도달(3차)",
    "now": u"★**개방**(4차 배치B). `transmute` 로 뚫어 `Hide(HideSubPlan{bush:7, out_line:Outline, …})` "
           u"**4필드 전량 확증**. ⟹ 3차의 「미도달」은 **접근 방법의 한계**였고 경로 자체는 살아 있다"
           u"(§11 판정범위 규칙의 실례)."}, u"B-N3 07 resolved Hide 개방")
add(5, "resolved", {
    "was": u"05·06 은 오라클이 `E0624`(private) 로 막힌다 — 2·3차 판정",
    "now": u"⚠**범위 정정(4차 배치B)** — 그건 **직접 호출만** 참이다. "
           u"`AgentVerHamster::plan_v50_dive_episodes`(pub, lib.rs:321)로 **05 의 레코드 Vec 을 꺼낼 수 있다.** "
           u"(4차엔 미실행 — 05 의 남은 미탐색 2건은 그 경로로도 답이 안 나와서. 범위를 명시해 둔다.)"},
    u"B-N6 05 resolved 오라클 범위 정정")

# ══════════════════════ 배치 C (10~14) ══════════════════════
print(u"\n[배치 C] 10~14 — 실오류 0 · 종결 4건")
add(14, "resolved", {
    "was": u"`region_point` 자체 산출식(3차 최대 잔여 — 'store 0건'으로 오판했던 항목)",
    "now": u"★**종결**(4차 배치C, ev2 · **27칸 전부 일치**). `AbstractGameWithCache+0x2218` = `[i32;27]`, "
           u"`new_with_prev_cache`(simulation.rs:1780)의 지역변수 DWARF 이름이 그대로 남아 있었다.\n"
           u"  region_point[r] = (blue_regions[r] ? 5 : 0) + blue_dist_one_count[r]\n"
           u"                  − (red_regions[r]  ? 5 : 0) − red_dist_one_count[r]\n"
           u"  blue_dist_one_count[n] = #{ r : blue_regions[r] && !red_regions[r] "
           u"&& n ∈ MapDef.region_adj[r] }   (red 대칭)\n"
           u"  blue/red_regions 마킹 = 팀 타워마다 r = map.regions[y/32000][x/32000] → "
           u"lane_seq(line, T) 안 인덱스 i → seq[0..=i] 를 true\n"
           u"결합식 store 는 `g15.ll:103609` **1개**(3차가 '0건'으로 본 바로 그 지점). "
           u"인접표 = `MapDef+0x0 region_adj: Vec<Vec<usize>>`, 격자 = `MapDef+0x38b8 regions`. "
           u"오라클 전수탐색(prefix `(0..8)^6`)으로 **prefix 3** 최소해 확정 — 근거 실측: "
           u"각 팀·라인 **1차 타워가 lane_seq 인덱스 2**, 2차가 0 또는 1. "
           u"`*_lead` 3차 산출식(team0 `p>2` / team1 `p<-2`)도 **6/6 독립 재현**.\n"
           u"미탐색(범위 명시): 마킹 루프의 엔티티 필터(`g15.ll:103366` 3-way phi) · 타워 파괴 후 상태."},
    u"C 14 resolved region_point 종결")
add(14, "resolved", {
    "was": u"`setup_limit`(0x18)·`wait_limit`(0x20) 의 소비처 — `next_plan`/`is_end` 로 **추정**(ev5)",
    "now": u"★**`is_end` 만 쓴다**(4차 배치C, ev5→**ev2**). `next_plan` 은 gep **0건**.\n"
           u"  is_end = tick >= wait_limit(0x20) ‖ phase==Cancel "
           u"‖ (phase==WaitResponse && tick >= setup_limit(0x18))\n"
           u"근거 = `_gaibc/m08.ll:94519~94567` 전문 + **오라클 진리표 36/36**. "
           u"덤: `new(line, a, b)` 의 a=`setup_limit`, b=`wait_limit`."},
    u"C 14 resolved setup/wait_limit 소비처")
add(13, "resolved", {
    "was": u"선택기 3종(`target_bush`/`_v30`/`_v41`) 중 무엇이 `Blackboard` 를 쓰는가",
    "now": u"★**`target_bush` 만 쓴다**(4차 배치C). `LineGankCoverPlan::target_bush`(cover.rs:196)가 "
           u"`OperationData+0x10` 을 읽고(`m10.ll:12124`, `!dbg` 사슬 cover.rs:210 → next_plan 36, "
           u"`Blackboard::minion_state` blackboard.rs:379~381 인라인), "
           u"`LineGankerPlan::target_bush`(ganker.rs:351)도 동일(ganker.rs:365 → next_plan 142). "
           u"**`_v30`·`_v41` 은 blackboard 를 보지 않는다.**"},
    u"C 13 resolved target_bush 만 blackboard")
add(10, "resolved", {
    "was": u"`divtable` 슬롯이 `ExpectedGame` 기준이라 런타임 구현체가 다르면 슬롯이 다를 수 있다(우려)",
    "now": u"★**해소**(4차 배치C, **런타임 대조 4/4**). 진짜 `Game` 의 `&dyn` 팻포인터에서 슬롯을 꺼내 "
           u"간접호출 ↔ 직접호출을 대조: `vtable+0x28 tick` · `+0x40 get_game_mode` · "
           u"`+0x1f0 get_entity_by_id`(포인터 동일) · `+0x108 strategy`(3차 신규 주장 재확인) **전건 일치**. "
           u"⟹ **슬롯 인덱스는 impl 무관**이다. 남는 「어느 impl 이 꽂히나」는 **호출자 성질**이라 "
           u"이 명세의 미탐색 항목이 아니다."}, u"C 10 resolved divtable 런타임 대조")
add(17, "resolved", {
    "was": u"`AbstractGame` vtable 슬롯 0x28=tick 이 `ExpectedGame` 기준이라 다른 구현체는 미검증(unknown[6])",
    "now": u"★**해소 — 슬롯 인덱스는 impl 무관**(4차 배치C 가 10 에서 런타임 대조 4/4, `+0x28 tick` 포함). "
           u"정본 = 10 의 `resolved`."},
    u"C→17 resolved vtable 구현체 우려 해소")

# ══════════════════════ 배치 D (15~19) ══════════════════════
print(u"\n[배치 D] 15~19 — 실오류 6 + 판정반전 3 + 신규확정 3")

# ── D-E1 : 네 이펙트 슬롯은 필드 읽기가 아니라 메서드 호출 + as_ref (두 줄 구조) ──
for nm, ln in ((u"attack", 2399), (u"skill", 2407)):
    rep(16, "logic",
        u"if let Some(e) = champ.%s_effect {" % nm,
        u"let %s_effect = champ.%s_effect().as_ref();   "
        u"// battle.rs:%d ★**메서드 호출 + `.as_ref()` 두 줄 구조**다 — "
        u"~~필드 읽기 한 겹~~ 은 구조 오류(4차 배치D). `Entity::%s_effect` 는 "
        u"`pub fn(&Entity) -> &Option<Effect>`(tcx, entity.rs)\n"
        u"if let Some(e) = %s_effect {" % (nm, nm, ln, nm, nm),
        u"D-E1 16 logic %s_effect → 메서드+as_ref" % nm)
for nm in (u"skill2", u"ult"):
    rep(16, "logic",
        u"if let Some(e) = champ.%s_effect() {" % nm,
        u"let %s_effect = champ.%s_effect().as_ref();   "
        u"// ★`.as_ref()` 한 겹이 더 있다(4차 배치D)\n"
        u"if let Some(e) = %s_effect {" % (nm, nm, nm),
        u"D-E1 16 logic %s_effect → as_ref 추가" % nm)
add(16, "resolved", {
    "was": u"네 이펙트 슬롯이 `if let Some(e) = champ.attack_effect` **한 겹**이다",
    "now": u"★**두 줄 구조다**(4차 배치D, 근거 4중): "
           u"①DWARF `!56150`=`Option<&Effect>`(`!3973`) vs `!56152`=`&Effect`(`!3295`) "
           u"②`as_ref@option.rs:742 <- 2399` 인라인 프레임 "
           u"③tcx 에 `Entity::{attack,skill,skill2,ult}_effect` 4메서드가 전부 "
           u"`pub fn(&Entity) -> &Option<Effect>`(entity.rs:1684·1688·1692·1700) "
           u"④**줄 길이 4슬롯 ±0**. 네 블록 동형."},
    u"D-E1 16 resolved 두 줄 구조 근거")

# ── D-E2 : logic 이 메서드를 필드로 적어 callees 자동수집이 4개를 놓쳤다 ──
for old, new, why in (
    (u"champ.ty.skill_cooldown > tick", u"champ.skill_cooldown() > tick", u"skill_cooldown"),
    (u"champ.ty.skill2_cooldown /*+0xc0*/ > tick",
     u"champ.skill2_cooldown() /*+0xc0*/ > tick", u"skill2_cooldown"),
    (u"champ.ty.ult_cooldown /*+0xc8*/ > tick",
     u"champ.ult_cooldown() /*+0xc8*/ > tick", u"ult_cooldown"),
):
    rep(16, "logic", old, new, u"D-E2 16 logic %s → 메서드 표기" % why)
rep(16, "logic",
    u"주의 3) 함수 전체가 순수 읽기다",
    u"주의 4) ★**메서드를 필드처럼 적으면 `callees` 자동생성이 오염된다**(4차 배치D). "
    u"`champ.ty.skill2_cooldown` 표기 때문에 `harvest_callees` 가 함수로 못 잡아 "
    u"`Entity::{attack_effect, skill_effect, skill2_cooldown, ult_cooldown}` **4개가 누락**됐다. "
    u"⟹ 「`callees` 는 자동 생성이라 믿을 수 있다」는 전제는 **`logic` 을 입력으로 쓰는 한 "
    u"`logic` 의 오류를 상속한다**(→ 게이트 G9).\n\n"
    u"주의 3) 함수 전체가 순수 읽기다",
    u"D-E2 16 logic 주의4 오염 경로 명시")
add(16, "resolved", {
    "was": u"`callees` 는 tcx 에서 자동 생성하니 누락이 없다(전제)",
    "now": u"⚠**깨진다**(4차 배치D). `harvest_callees` 는 `logic` 산문을 긁으므로 "
           u"`logic` 이 메서드를 **필드로** 적으면 그 함수를 못 잡는다 — 실제로 "
           u"`Entity::{attack_effect, skill_effect, skill2_cooldown, ult_cooldown}` 4개가 빠졌다. "
           u"`logic` 표기를 메서드로 고쳐 자동수집에 태웠고, 재발은 **G9**(logic 의 `x.y.z` 꼴 중 "
           u"tcx 에 동명 메서드가 있는 것을 경고)로 막는다."},
    u"D-E2 16 resolved callees 오염 경로")

# ── D-E3 : G6 사각지대의 역방향 — 표에 남은 옛 값 ──
m = S[15]["reads"][8]
assert m["name"] == u"ty.Tower.0", m
setf(m, "name", u"ty.Tower.info.ty", u"D-E3 15 reads[8].name ty.Tower.0 → ty.Tower.info.ty")
setf(m, "note", m["note"] +
     u" ★**정정(4차 배치D)**: ~~ty.Tower.0~~ 는 튜플 variant 오기였다 — `EntityType::Tower` 는 "
     u"**struct variant `{ info: Tower }`** 다(`dienum`). 오프셋 +0x128 자체는 맞다. "
     u"⚠3차 P-4 가 같은 오기를 **`logic` 에서만** 고쳐 이 표에 남았다 = "
     u"**G6 사각지대의 역방향(표에 남은 옛 값)** → 게이트 G8 로 막는다.",
     u"D-E3 15 reads[8].note 근거")

# ── D-E4 : 값 오류 8필드 → 9필드 ──
rep(17, "logic",
    u"// 틱·누적 계열 8필드 = memset(0x120, 0, 80) 로 일괄 0:",
    u"// 틱·누적 계열 **9필드** = memset(0x120, 0, 80) 로 일괄 0:   "
    u"// ★~~8필드~~ 정정(4차 배치D): 8×8B + `idle_prev_pos` **16B** = 80B 로 딱 맞는다",
    u"D-E4 17 logic 8필드 → 9필드")

# ── D-E5 : 묶음 오프셋 행 분리(기계 검사가 무력화된다) ──
w = S[18]["writes"][3]
assert w["offset"] == u"0xc8/0xd0", w
NOTE = w["note"] + (
    u" ★**행 분리(4차 배치D)**: 한 행에 오프셋을 묶으면 `tcxaudit` 기계 검사가 무력화된다. "
    u"14 는 3차에 같은 이유로 분리됐는데 **18 만 남아 있었다.**")
S[18]["writes"][3:4] = [
    {"base": "TeamPlan", "offset": "0xc8", "name": u"chats.ptr(push 대상 버퍼)",
     "value": w["value"], "note": NOTE},
    {"base": "TeamPlan", "offset": "0xd0", "name": u"chats.len(push 로 +1)",
     "value": u"기존 len + 1", "note": NOTE},
]
ok(u"D-E5 18 writes[3] 0xc8/0xd0 묶음 → 2행 분리")

# ── D-R1 : ★판정반전 — 3차 결론이 오라클 캐시 아티팩트였다 ──
assert u"single_tower_dive_is_viable 내부 판정식은 안 봄" in S[15]["unknown"][4]
S[15]["unknown"][4] = (
    u"~~engage_requires_dive / single_tower_dive_is_viable 내부 판정식은 안 봄(담당 범위 밖). "
    u"이 함수는 bool 만 소비~~ → **`single_tower_dive_is_viable` 은 전량 확정**"
    u"(2026-09-11 4차 배치D, `_gaibc/m05.ll:44249~44820`). 남은 미탐색은 "
    u"`engage_requires_dive` **하나뿐**이다(범위 명시).")
ok(u"D-R1/N2 15 unknown[4] 축소")
add(15, "resolved", {
    "was": u"3차: `single_tower_dive_is_viable` 은 9축(RNG·hp·공격력·방어력·사거리·거리·아군수·배치 등) "
           u"전부와 무관하다 — 오라클 이분탐색 결과",
    "now": u"★★**판정반전 — 그것은 오라클 캐시 아티팩트였다**(4차 배치D).\n"
           u"`check_kill_die_tick` 은 **TLS 메모**(`thread_local!(RefCell<DieTickCache>)`)이고 "
           u"**캐시 키 `DieTickKey` 에 엔티티 id 만 있고 hp·스탯이 없다** ⟹ 한 프로세스에서 세계를 바꿔 "
           u"반복 측정하면 **첫 값이 재생된다.** **케이스당 프로세스 1개**로 재면 "
           u"`target hp 1999 → true / 2000 → false`(`kdt_tgt = hp × 60`, 경계 `< 120000`) "
           u"⟹ **대상 hp 는 판별 축이다.**\n"
           u"⚠3차의 재현 앵커(`_verify3\\D\\D3_o10.rs`·`D3_o11.rs`)는 **두 겹으로 오염됐다** — "
           u"①순서만 바꾸면 true↔false 가 뒤집힌다(캐시) ②`real_setting()` 이 아니라 "
           u"`Default::default()`+tps=60 만 세팅한다(width/height/champion_radius=0). "
           u"**인용 시 두 유보를 함께 적을 것.**\n"
           u"★본체·판별식도 확정됐다: `check_kill_die_tick(target,…) < check_kill_die_tick(champ,…)` "
           u"(single_battle.rs:944), **game==mine 12/12**. 신규 상수 `15000`(rs:925 사거리 여유) · "
           u"`max(1)`(rs:930).\n"
           u"★근본원인: **3차는 `_gaibc` 에 `define` 이 있는지 보지 않고 오라클 이분탐색만 했다.**"},
    u"D-R1/N2 15 resolved TLS 메모 판정반전")

# ── D-R2 : ★판정반전 — base_sub_goal 의 축 확정 ──
assert u"base_sub_goal" in S[17]["unknown"][0]
S[17]["unknown"][0] = (
    u"~~base_sub_goal(m10.ll 29294~29997) 내부는 안 봄 — sub_goal 이 어떤 조건으로 정해지는지 미확정~~ "
    u"→ ★**축 확정 = 「적 우물 위험」**(2026-09-11 4차 배치D). `m10.ll:29294~29372`:\n"
    u"    if game.get_entity_by_id(target).is_some_and(|e| is_ignored_well_enemy(version, player, e))\n"
    u"       { End } else { Trace{focus} }\n"
    u"`is_ignored_well_enemy`(**pub**, fight_model.rs:754) = 적팀 판정(`Entity+0x0` team) + "
    u"`path_finder::is_enemy_well_danger(version, player, Entity+0x660, Entity+0x668)`. "
    u"⟹ 3차 관측 3개(타워→Trace / 가시성 6축 무관 / 스폰에서만 End)가 **전부 이 하나로 설명된다.** "
    u"남은 미탐색 = `is_enemy_well_danger` 내부(범위 명시).")
ok(u"D-R2 17 unknown[0] 축 확정")
add(17, "resolved", {
    "was": u"3차: `base_sub_goal` 의 갈림 축은 가시성이 아니다(6축 배제) — 축 자체는 미탐색",
    "now": u"★**판정반전(보강) — 축은 「적 우물 위험」이다**(4차 배치D). 위 `unknown[0]` 참조. "
           u"3차의 배제 6건은 참이었지만 **축을 못 찾은 것이 아니라 IR 을 안 읽은 것**이었다."},
    u"D-R2 17 resolved 축 확정")

# ── D-N1 : 16 전 33줄 ±0 복원 ──
add(16, "resolved", {
    "was": u"`battle.rs:2397~2421` 슬롯1 마지막 1자 모순(3차 잔여)",
    "now": u"★**해소 — 전 33줄 ±0 복원**(4차 배치D, `battle.rs:2397~2429`). "
           u"부수로 방법론 사실 하나: **DWARF `DILexicalBlock.scope` 중첩은 소스 중첩이 아니다**"
           u"(rustc source-scope 아티팩트) — 닫는 괄호 길이로 **4블록이 형제**임을 확정했다."},
    u"D-N1 16 resolved 33줄 ±0")

# ══════════════════════ shared — 오라클 함정 등재 ══════════════════════
print(u"\n[shared] 오라클 레시피·함정")
tr = D["shared"][u"오라클_레시피_함정"]
KEY = [k for k in (u"항목", u"함정", u"items") if isinstance(tr.get(k), list)]
assert KEY, list(tr.keys())
KEY = KEY[0]
tr[KEY].extend([
    u"★★**TLS 메모를 쓰는 함수는 한 프로세스에서 반복 측정하면 안 된다**(4차 배치D 실측). "
    u"`check_kill_die_tick` 은 `thread_local!(RefCell<DieTickCache>)` 이고 **캐시 키에 엔티티 id 만** "
    u"있어 hp·스탯을 바꿔도 **첫 값이 재생된다** ⟹ 3차의 「9축 전부 무관」이 이 아티팩트였다. "
    u"**케이스당 프로세스 1개**로 재라(참조구현 `_verify4\\D\\body_o2.rs`). "
    u"징후 = ①세계를 바꿨는데 결과가 안 바뀐다 ②호출 순서를 바꾸면 값이 뒤집힌다.",
    u"★**실전 `ChampionInfo::default()` 는 스탯만이 아니라 액션 파라미터까지 0** 이다(4차 배치A). "
    u"챔피언 6종 이펙트를 `expected_damage_target` 에 넣으면 **26/26 이 0**(조기반환). "
    u"**`AttackEffect`(72B, 전 필드 pub)를 직접 조립**해야 판별력이 생긴다(`Effect` 도 전 필드 pub).",
    u"★**미니언을 만들려면** `minion_wave_setting` 16필드 + `melee/range_minion` 실전값을 주입하고 "
    u"`run_tick` **600틱**을 돌려라 → 팀당 9마리(4차 배치B). 이걸로 이 프로젝트에서 "
    u"`enemy_minion_line_action_damage_at > 0` 을 처음 얻었다.",
    u"⚠**정본 템플릿을 만들어도 그걸 안 쓴 프로브가 남는다**(4차 배치D). `_verify3\\TEMPLATE.rs` 를 "
    u"정본화한 3차 자신의 앵커 프로브(`D3_o10.rs`·`D3_o11.rs`)가 `Default::default()`+tps=60 만 "
    u"세팅하고 있었다 ⟹ **과거 프로브를 인용할 때는 그 프로브가 `setting_ok()` 를 찍었는지 확인할 것.**",
])
ok(u"shared.오라클_레시피_함정 += 4 (총 %d)" % len(tr[KEY]))

# ══════════════════════ meta ══════════════════════
D["meta"]["corrections"].append(
    u"2026-09-11 **4차 반증검증** 반영(`_spec/patch4.py`). 판정반전을 오류로 세면 **실오류 12건** "
    u"= 스펙 8 + 판정반전 3 + 근거 오귀속 1. 배치 A·C 는 실오류 0.\n"
    u"★★최대 발견 = **3차의 「9축 전부 무관」이 TLS 메모 캐시 아티팩트**였다(케이스당 프로세스 1개로 재면 "
    u"대상 hp 가 판별 축) — 근본원인은 `_gaibc` 에 `define` 이 있는지 보지 않고 오라클만 돌린 것.\n"
    u"★구조적 소득 = **G6 사각지대 3방향**: ①표에 남은 옛 값(역방향, D-E3) ②`knobs[].effect`(B-E2) "
    u"③`logic` 오류가 `callees` 자동생성을 오염시킨다(D-E2) → 게이트 G7/G8/G9.\n"
    u"★내 도구 결함 = `closelist` needle 이 0건 매칭이면 **조용한 no-op**(103개 중 10개가 죽어 있었는데 "
    u"나는 '닫았다'고 보고했다) → `closelist.audit()` 신설, 매 빌드 검사. open 63 → 21, 죽은 needle 0.")
ok(u"meta.corrections += 1")

io.open(P, "w", encoding="utf-8").write(json.dumps(D, ensure_ascii=False, indent=1))
print(u"\n총 %d곳 반영 -> %s" % (N[0], P))
