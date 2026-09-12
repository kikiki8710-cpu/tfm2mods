# -*- coding: utf-8 -*-
u"""patch5 — 2026-09-11 **5차** 반증검증 정정을 `_spec/specs20.json`(v2 정본)에 반영.

5차 집계: **실오류 14 · 판정반전 4 · 새 발견 44 · `ev4→ev2` 상향 492행.**
배치별 = A 1 / B 7+3 / C 3+1 / D 3+0. **값·오프셋·상수·분기 오류는 네 배치 전부 0.**

## 5차의 성격
`ev2`(오라클 실행) 커버리지를 **1.5%(13행) → 약 50%** 로 올린 라운드다. 그래서 나온 14건은
「명세가 나빠졌다」가 아니라 **처음 실행해 본 곳에서 나온 것**이다. 실제로 동작을 바꾸는 것은
**A-E1(`caster_r` 배치) · D-E2(`best_jungle_goal` 반환 범위)** 정도이고 나머지는 문면·표기·노브 라벨이다.

## ⚠ev 상향 492행은 이 패치에 없다
배치들이 **집계표로만** 보고하고 행 목록을 JSON 경로로 안 남겨서 175개만 추출됐다.
그건 내 브리핑이 "행 목록"이라고만 하고 **기계 판독 형식을 지정 안 한 탓**이다(6차 브리핑에서 교정).
⟹ 지금은 정정만 반영하고, ev 는 6차가 경로와 함께 다시 보고하게 한다.

## 대상 필드 주의
v2 는 `unknown[](문자열)` · `resolved[]{was,now}` · `reads/writes[]` · `knobs/new_knobs[]` · `constants[]`.
v3 의 `mem`=reads+writes · `open`/`notes`=unknown · `history`=resolved · `knobs`=knobs+new_knobs · `consts`=constants.
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
    assert old in S[i][key], u"못 찾음: %s\n    %r" % (label, old[:80])
    S[i][key] = S[i][key].replace(old, new)
    ok(label)


def find(i, field, needle):
    u"""⚠`json.dumps` 는 따옴표를 `\\\"` 로 이스케이프한다 — 값 안에 `"` 가 있는 항목
    (`"최근 가시" 창`)은 dumps 문자열로 찾으면 **안 걸린다.** 원문 값에서 찾는다."""
    for j, x in enumerate(S[i].get(field) or []):
        vals = list(x.values()) if isinstance(x, dict) else [x]
        if any(isinstance(v, str) and needle in v for v in vals):
            return j, x
        if needle in json.dumps(x, ensure_ascii=False):
            return j, x
    raise AssertionError(u"못 찾음: specs[%d].%s ~ %s" % (i, field, needle))


def add(i, item, label):
    S[i].setdefault("resolved", []).append(item)
    ok(label)


# ══════════════ 배치 A — 실오류 1 ══════════════
print(u"\n[배치 A] 00~04 — 실오류 1 (동작을 바꾸는 건)")

# A-E1: caster_r 가 Effect::range 안이 아니라 밖(effect_range_with_radii)에서 더해진다
rep(0, "logic",
    u"//   Effect::range(effect, champ) = effect.range(0x10) + caster_r",
    u"//   Effect::range(effect, champ) = effect.range(0x10)   "
    u"// ★**caster_r 는 여기 없다**(5차 배치A 정정). ~~+ caster_r~~ 는 구조 오류였다",
    u"A-E1 00 logic caster_r 를 Effect::range 밖으로")
rep(0, "logic",
    u"//   total = Effect::range(..) + Effect::range_adjust(effect, champ, target)",
    u"//   total = Effect::range(..) + **caster_r**   "
    u"// ★caster_r 는 `effect_range_with_radii`(abstract_input.rs:191) 에서 **밖에서** 더해진다\n"
    u"//           + Effect::range_adjust(effect, champ, target)",
    u"A-E1 00 logic total 에 caster_r 추가")
add(0, {
    "was": u"`caster_r`(시전자 반경)가 `Effect::range` **안**에서 더해진다",
    "now": u"★**틀렸다 — 밖에서 더해진다**(5차 배치A, 오라클). "
           u"`Effect::range(&ef, champ)` 은 casting 4값(Targeting/Position/Direction/None) "
           u"**전부 200000**(champ.radius=10000) 인데, 같은 실행의 실제 total 은 "
           u"Targeting **220000** / 나머지 **210000** ⟹ 여분 10000 = `champ.radius` 가 "
           u"`effect_range_with_radii`(abstract_input.rs:191)에서 **밖에서** 더해진다. "
           u"IR `m04.ll:44245~44255` 의 caster_r phi 도 `effect+16` 과 나란한 **별개 항**이다.\n"
           u"⚠같은 파일 `constants`(CastingType::Targeting 태그)는 **이미 올바르게** 적혀 있었다 = "
           u"**자기모순**인데 `specgate G1` 이 못 잡았다(G1 은 `logic` 산문의 **함수 경계**를 안 본다)."},
    u"A-E1 00 resolved 근거")

# ══════════════ 배치 B — 실오류 7 + 판정반전 3 ══════════════
print(u"\n[배치 B] 05~09 — 실오류 7 + 판정반전 3")

# B-E1/E2: "최근 가시" 창 40틱 → min_hp_ratio = 40
j, k = find(8, "new_knobs", u'"최근 가시" 창')
k["what"] = u"적 HP 하한 `min_hp_ratio`"
k["value"] = 40
k["effect"] = (
    u"★★**정정(5차 배치B) — ~~\"최근 가시\" 창 = 40틱~~ 은 오독이었다.** 실제는 "
    u"**`min_hp_ratio = 40`(적 HP 백분율 하한)**: `hp*100/max < 40` 인 적은 **건너뛴다**. "
    u"근거 3중 = DWARF `!61645 name:\"min_hp_ratio\" arg:6` + IR `hp*100/max < %5 → skip` + "
    u"**오라클 39/40 경계 반전**.\n"
    u"올리면 빈사 적을 무시해 셋업 해제가 둔해지고, 내리면 거의 모든 적을 세어 민감해진다.\n"
    u"⚠**같은 오독이 `resolved` 에 「★확정」 표시로 한 번 더** 있었다(B-E2). "
    u"`knobs` 와 `resolved` 가 같은 거짓을 서로 보강하고 있었고, `logic`↔표만 보는 게이트(G5·G6·G8)는 "
    u"**`knobs` ↔ `resolved` 모순을 못 잡는다**(배치B 지적, 게이트 사각지대).")
ok(u"B-E1 08 new_knobs[%d] 최근가시창 → min_hp_ratio" % j)
add(8, {
    "was": u"`objective_discipline.rs:187` 의 40 = **\"최근 가시\" 창(40틱)** — 「★확정」으로 표기돼 있었다",
    "now": u"★**거짓이었다**(5차 배치B, 판정반전 R1). 실제 = **`min_hp_ratio = 40`**(적 HP% 하한). "
           u"DWARF 인자명 + IR 비교식 + 오라클 39/40 반전으로 확정. "
           u"⟹ 「★확정」 표기가 붙어 있어도 **근거가 인자명·실행이 아니면 믿지 마라.**"},
    u"B-E2 08 resolved 오독 정정")

# B-E3/E4/E5: knobs[0] note 오귀속 + ev 근거 / knobs[1] ev
j0, k0 = find(8, "knobs", u"22500000000")
k0["effect"] = k0.get("effect", u"") + (
    u"\n⚠**근거 정정(5차 배치B)**: 이 노브는 **(d) 경로 전용**이고 3차가 「(c)(d) 오라클 미도달」로 "
    u"명시했으므로 실행 근거가 없었다. 5차가 **(d) 경로에 최초 도달**해 경계를 "
    u"**정확히 150000/150001** 로 격리했다(제곱값 22500000000). "
    u"⚠기존 `note` 에 **15×tps 노브의 설명이 오귀속**돼 있던 것도 함께 제거했다.")
if "note" in k0:
    k0["note"] = u"~~(15×tps 노브의 note 가 오귀속돼 있었다 — 5차 배치B 제거)~~"
ok(u"B-E3/E4 08 knobs[%d] 근거 정정 + note 오귀속 제거" % j0)
j1, k1 = find(8, "knobs", u"에픽 리젠 대기 허용 시간")
k1["effect"] = k1.get("effect", u"") + (
    u"\n★**실행 확증(5차 배치B)**: v23 반경 180000 도 **180000/180001** 로 격리. "
    u"같은 사실을 `constants` 는 이미 실행 근거로 갖고 있었는데 이 행만 빠져 있었다.")
ok(u"B-E5 08 knobs[%d] 실행 근거 추가" % j1)

# B-E6: 09 knobs[3] — sin² 는 무효 노브
j, k = find(9, "knobs", u"후퇴로/측면 판정 임계")
k["what"] = u"후퇴로/측면 판정 임계(cos² 실효 · **sin² 는 무효**)"
k["effect"] = (
    u"★**정정(5차 배치B)** — ~~cos²·sin² **공통** 임계~~ 는 절반이 틀렸다. "
    u"**`1286` 의 sin² 항은 그 지점에 도달하면 항상 참**이라 **0~74 구간에서 무효 노브**다. "
    u"실효는 **`1283`(cos²) 하나뿐**이다.\n"
    + (k.get("effect") or u"") +
    u"\n⟹ 재구현·설정 UI 에서 이 노브를 **두 개처럼 노출하면 안 된다.**")
ok(u"B-E6 09 knobs[%d] sin² 무효 노브 정정" % j)

# B-E7: 09 open class 재료 부재 → 미탐색 (rmeta SourceMap 에 원본 줄 정보가 있다)
u0 = [x for x in S[9]["unknown"] if u"원본 소스는 이 환경에 없어" in x]
assert u0, u"09 unknown src_line 항목 못 찾음"
S[9]["unknown"][S[9]["unknown"].index(u0[0])] = u0[0].replace(
    u"원본 소스는 이 환경에 없어 대조하지 못했다",
    u"원본 `.rs` 파일 자체는 이 환경에 없다. ⚠단 **「재료 부재」가 아니라 「미탐색」이다**"
    u"(5차 배치B 정정): **rmeta SourceMap 에 `fight_check.rs` 가 있다**(1417줄, 파싱 0에러) ⟹ "
    u"줄 길이 산술로 표현식 문구를 좁힐 수 있다(4차 배치D 가 `16` 에서 33줄 ±0 복원한 그 수법). "
    u"현재까지 대조하지 못했다")
ok(u"B-E7 09 unknown src_line 재료부재 → 미탐색")

add(8, {
    "was": u"`08` 경로 (c) `L183` = 「오라클 미도달 = 미탐색」",
    "now": u"★**판정반전 R2 — 구조적 도달 불가**(5차 배치B). v24 exit phi + v23 술어가 "
           u"`is_end` 필터의 **진부분집합**임을 증명했고, 3종 구성 시도가 전부 (b)로 빠졌다. "
           u"⟹ 「아직 못 갔다」가 아니라 **갈 수 없다**. 재구현에서 이 가지는 죽은 코드로 취급해도 된다."},
    u"B-R2 08 resolved (c) 구조적 도달 불가")
add(9, {
    "was": u"`09` `L1307 else → front` 가지",
    "now": u"★**판정반전 R3 — 죽은 가지다**(5차 배치B). 라그랑주 항등식 기반 **대수 증명** + "
           u"난수 **200만** · 전수 격자에서 **0회** 도달. 재구현에서 생략해도 `game==mine` 이 깨지지 않는다."},
    u"B-R3 09 resolved L1307 죽은 가지")

# ══════════════ 배치 C — 실오류 3 + 판정반전 1 ══════════════
print(u"\n[배치 C] 10~14 — 실오류 3 + 판정반전 1 (값 오류 0 유지)")

assert u"v3" in S[11]["one_line"], S[11]["one_line"][:120]
S[11]["one_line"] = S[11]["one_line"].replace(u"v3", u"v2")
S[11]["one_line"] += (u"  ※~~v3~~ → **v2** 정정(5차 배치C, 오라클: version 0·1 은 6168B diff 전무, "
                      u"**2부터** `fallbacks=1`). `constants`·`logic` 은 처음부터 v2 로 맞았고 "
                      u"`one_line` 만 어긋나 있었다 — `specgate G1` 은 `one_line` 을 보지만 규칙이 3종뿐이라 못 잡는다.")
ok(u"C-E1 11 one_line v3 → v2")

rep(12, "logic", u"position_exists(from, tutorial)",
    u"position_exists(&GameContext, Position)   "
    u"// ★시그니처 정정(5차 배치C, rustc 실컴파일). ~~(from, tutorial)~~ 은 오기",
    u"C-E2 12 logic position_exists 시그니처")

rs = D["shared"].get(u"rule_scope_게이트")
assert rs, list(D["shared"].keys())
key = [k for k, v in rs.items() if isinstance(v, str) and u"goal_allowed" in v]
assert key, list(rs.keys())
rs[key[0]] = rs[key[0]].replace(u"goal: &BigGoal", u"goal: BigGoal") + (
    u"  ※★정정(5차 배치C): `goal_allowed` 의 goal 은 **값 전달 `BigGoal`** 이다 — "
    u"`&BigGoal` 은 **컴파일 거부**된다. 형제 `plan_allowed` 는 `&BigPlan` 이 맞아서 헷갈리기 쉽다.")
ok(u"C-E3 shared.rule_scope_게이트 goal 값 전달")

# C-E4 판정반전: cover 판 target_bush_v30 은 무관 → 문자 단위 동일 복제본
u14 = [x for x in S[14]["unknown"] if u"담당 함수와 무관" in x]
assert u14, u"14 unknown cover 판 항목 못 찾음"
S[14]["unknown"][S[14]["unknown"].index(u14[0])] = u14[0].replace(
    u"담당 함수와 무관",
    u"~~담당 함수와 무관~~ → ★**판정반전(5차 배치C): 문자 단위 동일 복제본**이다. "
    u"rmeta SourceMap 줄 길이가 `cover.rs:134~186` ↔ `ganker.rs:248~300` **53줄 전부 일치**"
    u"(오프셋 차 +114)이고 다른 것은 `self` 타입뿐(`LineGankCoverPlan+0x20` ↔ `LineGankerPlan+0x28`). "
    u"⟹ cover 판 `define`(m10.ll:11483)은 **유효한 대리 관측점**이고 실제로 5차 상향 60행을 만들었다. "
    u"⚠**같은 오류 문면이 `MIG\\SPEC_GUIDE.md` §1 fnparts 항목에도 있다** — 「≠ 이니 조각을 버려라」가 "
    u"관측 경로를 닫는다")
ok(u"C-E4 14 unknown cover 판 = 동일 복제본(판정반전)")

j, k = find(14, "knobs", u"Mid")
if u"재료 부재" in json.dumps(k, ensure_ascii=False):
    k["effect"] = (k.get("effect") or u"") + (
        u"\n★**해소(5차 배치C, 84/84)** — ~~Mid `s=true` 가지는 재료 부재~~. "
        u"`Entity` 복제 + 좌표 변경 + `cache.player_champion[t][p]` 주입으로 도달했다. "
        u"실측 `team0 [21,14,14,14,14,9,9]` / `team1 [9,14,14,14,14,21,21]`.")
    ok(u"C-E5 14 knobs[%d] Mid 가지 해소" % j)
else:
    print(u"  -- C-E5 대상 문면 다름(수동 확인 필요)")

u13 = [x for x in S[13]["unknown"] if u"IR 만으로는 알 수 없다" in x]
assert u13, u"13 unknown L151/L156 항목 못 찾음"
S[13]["unknown"][S[13]["unknown"].index(u13[0])] = u13[0].replace(
    u"IR 만으로는 알 수 없다",
    u"IR 만으로는 알 수 없다. ⚠**단 「재료 부재」가 아니라 「미탐색」이다**(5차 배치C): "
    u"rmeta SourceMap 줄 길이가 **L151(54B) ≠ L156(56B)** · L181(56B) ≠ L186(58B) ⟹ "
    u"**같은 값을 내는 다른 표기**이지 중복이 아니다. 표기 차이의 내용은 줄 길이 산술로 더 좁힐 수 있다")
ok(u"C-E6 13 unknown L151/L156 재료부재 → 미탐색")

# ══════════════ 배치 D — 실오류 3 ══════════════
print(u"\n[배치 D] 15~19 — 실오류 3 (ev2 0행 → 128행)")

rep(17, "logic", u"cap 0 -> 1",
    u"cap 0 -> **4**   // ★정정(5차 배치D): `RawVec::MIN_NON_ZERO_CAP`, `size_of::<Chat>()`=24 "
    u"⟹ 첫 push 에서 cap 이 **4**가 된다(실측 `vec_push1 w0=4 cap_api=4`). ~~1~~ 은 오기",
    u"D-E1 17 logic Vec cap 0->4")

L19 = S[19]["logic"]
old19 = [ln for ln in L19.split(u"\n") if u"일반 캠프" in ln and (u"전용" in ln or u"후보에 없다" in ln)]
assert old19, u"19 logic 일반캠프 전용 문장 못 찾음"
S[19]["logic"] = L19.replace(old19[0], (
    u"// ~~%s~~\n"
    u"// ★**정정(5차 배치D)**: 반환값은 **일반 캠프로 한정되지 않는다.** "
    u"824~829 폴백이 `now_camp` 를 **그대로 돌려주므로** Morgard/Serpen 도 나올 수 있다 "
    u"(실측 `champNone_nowcamp Some(Morgard) game=Morgard`). "
    u"⟹ 재구현에서 반환값을 일반 캠프로 필터링하면 **동작이 달라진다**."
) % old19[0].strip().lstrip(u"/ "))
ok(u"D-E2 19 logic 일반캠프 한정 아님")

cnt = 0
for i in (16,):
    for x in (S[i].get("reads") or []) + (S[i].get("writes") or []):
        n = x.get("name") or u""
        if n.startswith(u"ty.Champion.") and not n.startswith(u"ty.Champion.0."):
            x["name"] = n.replace(u"ty.Champion.", u"ty.Champion.0.", 1)
            x["note"] = (x.get("note") or u"") + (
                u" ★표기 정정(5차 배치D): `EntityType::Champion` 은 **튜플 variant** 라 "
                u"`ty.Champion.0.<필드>` 다. 근거 = `tcxdict --enum EntityType`(필드명 = `0`) + "
                u"rustc **E0164**(`Tower(i)` 반려 / `Champion(ch)` 통과). "
                u"⚠4차 D-E3 가 `Tower` 를 **struct variant**(`{info}`)로 정정했는데 "
                u"**`Champion` 은 튜플이다 — variant 마다 다르다.** 오프셋은 맞다.")
            cnt += 1
if cnt:
    ok(u"D-E3 16 reads/writes ty.Champion.0.* 표기 정정 %d행" % cnt)
else:
    print(u"  -- D-E3 16 대상 없음(이미 정정됐거나 표기 다름)")

sh = D["shared"].get(u"Entity_공통_오프셋")
if isinstance(sh, dict):
    c2 = 0
    for k2 in list(sh.keys()):
        v2 = sh[k2]
        if isinstance(v2, str) and u"ty.Champion." in v2 and u"ty.Champion.0." not in v2:
            sh[k2] = v2.replace(u"ty.Champion.", u"ty.Champion.0.") + \
                u"  ※튜플 variant 표기(5차 배치D 정정)"
            c2 += 1
    if c2:
        ok(u"D-E3 shared.Entity_공통_오프셋 %d행 표기 정정" % c2)

j, k = find(16, "knobs", u"HARD_CC") if any(u"HARD_CC" in json.dumps(x, ensure_ascii=False)
                                            for x in (S[16].get("knobs") or [])) else (None, None)
if k:
    k["effect"] = (k.get("effect") or u"") + (
        u"\n⚠**보강(5차 배치D)**: 제외 목록이 2개만 적혀 있는데 **실제 5개**다.")
    ok(u"D 보강 16 knobs[%d] HARD_CC 제외 목록" % j)

D["meta"]["corrections"].append(
    u"2026-09-11 **5차 반증검증** 반영(`_spec/patch5.py`). **실오류 14 · 판정반전 4 · 새 발견 44 · "
    u"`ev4→ev2` 상향 492행**(A 146 / B 46 / C 172 / D 128). "
    u"★**값·오프셋·상수·분기 오류는 네 배치 전부 0** — 5차의 14건은 문면·시그니처·노브 라벨 쪽이고, "
    u"**실제로 재구현 동작을 바꾸는 것은 A-E1(`caster_r` 배치)·D-E2(`best_jungle_goal` 반환 범위) 2건**이다.\n"
    u"★5차의 성격 = **`ev2` 커버리지를 1.5%(13행) → 약 50% 로 올린 계측기 라운드**다. "
    u"14건은 「명세가 나빠졌다」가 아니라 **처음 실행해 본 곳에서 나온 것**이다.\n"
    u"⚠**ev 상향 492행은 미반영**: 배치들이 집계표로만 보고하고 행 목록을 JSON 경로로 안 남겨 "
    u"175개만 추출됐다. 원인은 내 브리핑이 기계 판독 형식을 지정 안 한 것 — 6차가 경로와 함께 재보고한다.\n"
    u"★게이트 사각지대 2개 신규 확인: ①`specgate G1` 이 `logic` 산문의 **함수 경계**를 안 봐서 "
    u"A-E1 자기모순을 놓쳤다 ②G5·G6·G8 은 `logic`↔표만 봐서 **`knobs` ↔ `resolved` 모순**을 못 잡는다"
    u"(B-E1/E2 가 서로를 보강하고 있었다).")
ok(u"meta.corrections += 1")

io.open(P, "w", encoding="utf-8").write(json.dumps(D, ensure_ascii=False, indent=1))
print(u"\n총 %d곳 반영 -> %s" % (N[0], P))
