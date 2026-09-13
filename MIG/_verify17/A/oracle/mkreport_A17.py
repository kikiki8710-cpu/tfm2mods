# -*- coding: utf-8 -*-
"""17차 배치A REPORT.md 생성기 — 도시에 §6 이 요구하는 파이프라인 산출물(메인이 open/notes 반영에 읽는다)."""
import io
R = u"""# 17차 배치 A 보고 — specs[40]~[44] (게임 0.5.8)

기계 적용분 = `_verify17/A/patch.json` (정정 31 · ev상향 62 · 지시오류 5). `applypatch.py 17 --only A --dry` = **31/31 · 62/62 성공, 동작 변경 0건**.
신선도: `dossierfresh.py 17 A` = FRESH (착수 시·제출 직전 2회).

## 0. 실제로 실행한 것 (명령줄)
```
python -X utf8 dossierfresh.py 17 A                                   # FRESH ×2
python -X utf8 llann.py /c/tfm2mods/_gaibc/m13.ll 10721 10921 --root   # #40 본문
python -X utf8 llann.py /c/tfm2mods/_gaibc/m10.ll 39411 39912 --root   # #41 본문
python -X utf8 llann.py /c/tfm2mods/_gaibc/m05.ll 26839 26894 --root   # #42
python -X utf8 llann.py /c/tfm2mods/_gaibc/m10.ll 22813 22868 --root   # #43
python -X utf8 dloc.py m10.ll 45851 45852 45857 45953 45954 45505 45503  # #41 631/632 사슬
python -X utf8 llann.py _gcbc/g06.ll 84197 84260 / 87697~87860           # Entity::distance → utils::distance
python -X utf8 llann.py _gaibc/m12.ll 32239 32300                        # #41 min_by fold 본체
python -X utf8 llann.py _gcbc/g07.ll 152570 152700 / _gcbc/g02.ll 6914 7160   # MapDef::camp_pos + 클로저
python -X utf8 llann.py _gaibc/m04.ll 28343 28375 / 28110 28342          # PassiveJunglePlan::new / new_counter_jungle
python -X utf8 tcxdict.py {BattlePlan, SinglePlanBattle, TeamPlan 0x0, Blackboard 0xf8, Entity 0x438/0x4a0/0x4c0/0x5c0/0x5c8/0x640/0x660/0x668, LegacyPlanHandler 0x1808/0x5e8/0x706/0x618/0x650/0x638/0x7b0/0x7b8/0x7c0, GameContext 0x0/0x8/0x20, OperationData 0x0/0x8/0x10, PlayerState 0x930, GameSetting 0x12f8, AbstractGameWithCache 0x1e0, MapDef 0x68, CampDef, PassiveJunglePlan}
python -X utf8 tcxdict.py --enum {BattlePlanGoal, MainObjective, game_core::BigGoal, game_core::Chat}
python -X utf8 divtable.py AbstractGame 0x28                             # → tick (98%, vtable 2개)
python -X utf8 fnprobe.py 0xe4b8c0 ; python -X utf8 disrva.py 0xe4b8c0 +0x160   # #44 exe 재확인
sh _verify3/build.sh C:/tfm2mods/MIG/_verify17/A/oracle/v17A_o1.rs ; %TEMP%/tfm2_spanprobe/v17A_o1.exe > _verify17/A/oracle/v17A_o1.tsv
python -X utf8 _verify17/A/oracle/mkpatch_A17.py ; python -X utf8 applypatch.py 17 --only A --dry
```

## 1. ★내 지시(도시에)의 오류 — 먼저
`patch.json.brief_errors` 5건. 요지:
1. **G10 어휘 불일치**: §4 는 [42]/[43] `open[2]`(「미확정」)만 잡았는데 같은 함수의 `open[3]`/`[4]`(「…실었다」 = 사실 서술)는 못 잡았다. `specgate.gate10.FACT_TAIL` 이 `mkspec3.FACT_TAIL`(넣었다·적었다…)보다 좁다 → 두 도구가 같은 문장을 다르게 분류한다. **게이트가 mkspec3 의 정규식을 import 해서 써야 한다.**
2. **spec_44 자기모순**: 머리표 `exe=e4b8c0`(09-13 확정)인데 `open[0]` 은 「0xe70c70 은 이 함수가 아닐 가능성」을 그대로 싣고 있었다. 주소를 고칠 때 open 이 안 닫혔다(G7 의 exe 축 판) — 이번 patch 로 닫았다(**판정 반전 1건**으로 센다).
3. 라운드 프롬프트의 「G4(callees_unmatched ≥8)」: 배치A 5함수의 unmatched = 6/3/4/4/4 로 해당 0건, 그리고 RUNBOOK §S5-b 의 G4 는 「술어 시그니처」다 — 지시가 표에 없는 이름으로 게이트를 가리켰다.
4. §1 「ev≥4(미실행)」에 vis 가 `pub` 인 #42/#43 이 있으면 「오라클 우선」 표식을 달아 달라 — 순수·TLS 없음이라 진리표 520건이 30분이었다.
5. `_verify3/build.sh` 출력 폴더(`%TEMP%\\tfm2_spanprobe`)를 4배치가 공유한다 — `o1.rs` 같은 이름은 충돌한다(이번엔 `v17A_` 접두로 회피). 지시문에 파일명 접두 규칙을 넣어라.

## 2. 함수별 결과

### #42 SinglePlanBattle::with_runaway · #43 BattlePlan::with_runaway — ★오라클 520/520 MATCH (각각)
- `_verify17/A/oracle/v17A_o1.rs` → `v17A_o1.tsv`. 축 = main_goal 태그{0..3} × elapsed{0, tps−1, tps, tps+1, 10tps} × main_objective{None(−1), 0..11} × with_dive{F,T} = 520 케이스/함수. `set_tick(pub)` 으로 tick 을 옮기고, 필드는 tcx 오프셋(0x40/0x80·0xc0/0x88·0xf6/0x8d·0xff)에 `ptr::write` 해 두 함수를 **한 케이스에 동시** 호출, 명세 logic 을 독립 재구현한 `spec_with_runaway` 와 대조 → **MISMATCH 0**. 경계 실측: elapsed=59·60 → false, 61 → true / Morgard·Serpen·Defense·Nexus → false / PressEpic = !with_dive / 나머지(None·3·6~11) → true. `setting_ok=true, tps=60`.
- 오라클 자체 검증: TLS 메모 없음(IR 에 LocalKey 호출 0, 순수 읽기) → 한 프로세스 반복 측정 OK. `data.cache.game.tick()` 이 `set_tick` 값과 일치 = 구현체는 `Game`.
- ev 상향: consts 6+6 · knobs 3+3 → **ev2**, mem 7+7 → **ev3**(tcxdict + 오라클이 그 오프셋에 쓴 값으로 결과가 갈렸으니 오프셋도 실행으로 교차확인됨).
- G10 `open[2]`(tick 구현체): divtable 0x28→tick(98%) + tcx impl 4종(Game/SingleLaneGame/DeathMatchGame/ExpectedGame) + ExpectedGame 은 game_ai 에서 dodge 계열(m04.ll:35693/40503/47229)에만 vtable 이 넘어감 → **사실 서술**로 문면 정정. `open[3]/[4]`(xor bool) 도 IR+오라클로 확정 → 사실 서술.
- 미해소(그대로 둠): `open[0]`·`[1]` 표기 불가(외연 동일 — 오라클로도 못 가른다, METHOD_MAP ⑥-3) · #43 `open[3]` 호출자 소비처 = 미탐색(범위 밖).
- 참고(패치 안 함): #42 exe `d51fb0` 은 메인이 「SingleLane 전용 → MOBA NA」로 확정했는데 명세 `one_line`/notes 엔 그 사실이 없다 — 메인이 notes 에 넣을지 판단.

### #41 fight_participants — G12 2건 = **실오류 2**(둘 다 값 정정) · G10 2건 해소 · open 4건 해소
- **consts[8] src_line 26 → 631** (실오류): 리터럴 −1 은 m10.ll:39800 `add i64 %154, -1 !dbg !45851`, 사슬 = effect.rs:26(range) ← fight_model.rs:631. `src_line` 규약은 자기 파일 줄(`srclinebase.check_spec: fn == own`)이라 26 은 effect.rs 줄. meaning 에 「effect.rs:26 인라인 · 오프셋가감」 명시(kind 태그→오프셋가감 유도). behavior_change=false.
- **consts[10] src_line 623 → 624** (실오류): `phi i8 [1,%118],[1,%113]`(39897) 의 인입 블록 !dbg 가 :624(push). logic 도 `[L624]` 줄 분리.
- `open[3]` Effect::range: tcx 시그니처 `range(&Effect,&Entity)` + 합계식 전부가 `scope=range(effect.rs:26)` 안(closure$1 은 :631 호출만) → **사실 서술(★해소)**.
- `open[5]` min fold: m12.ll:32239~32320 본체 = `acc = if Ord::cmp(acc,d) ≤ Equal {acc} else {d}` 순수 최소값(동률 앞 원소) → **★해소**.
- `open[1]` Entity::distance: g06.ll:84197 → utils::distance(87697) = isqrt(dx²+dy²)(≤1e6 usqrt 표 / 초과 이분탐색) → **실거리, ★해소**.
- `open[4]` unwrap 형태: `!45505 = unwrap_or<u64> inlinedAt 632`(631 의 !45503 과 별개) → **`.min().unwrap_or(_)` 로 형태 확정**, 기본값 리터럴만 표기 불가(정정형 문면).
- logic 필드 표기 `f(+0x…)` → `f[+0x…]` 5곳: `harvest_callees` 의 `name(` 정규식이 필드를 호출로 긁어 callees 에 `game_view::ClientData::team`·`nightmare::tick_per_second`·`WindowStatView::level` 잡음이 생기고 있었다(§4-b callees 축). `range`/`attack_effect`/`distance`/`fold` 는 실제 인라인·호출이라 유지. 미매칭 `move_speed`·`player_champion` = 필드, `reserve_internal_or_panic` = bumpalo — 술어 아님.
- mem 19행 → ev3(tcxdict 전수 대조, 불일치 0).
- **오라클 = 재료 부재(직접 진입)**: tcx `v=in:game_ai`(pub(crate)) 확인 — 호출자 2종(`BattleSubPlan::action_candidates` m02.ll:27592 · `BattlePlan::update_v32` m10.ll:13739) 경유는 **미탐색**. fight_model 에 TLS `RESOLVE_FIGHT_CACHE`(m10.ll:141) 가 있어 ally_is_bound 가 그걸 쓰면 케이스당 프로세스 1개 규칙 적용(다음 라운드 주의).
- `open[0]` ally_is_bound 내용·`open[2]` 호출측 near_allies 자기포함 = **미탐색** 유지.

### #40 v3_assign_anchor — open 2건 ★해소 · consts[8] ev5→4 · callees 잡음 정리
- `open[0]` camp_pos bool: g07.ll:152585 `team_idx = !bool as usize`(map_def.rs:213) → 클로저(g02.ll:7066~7133)가 `camps`(MapDef+0x68, CampDef 40B = pos[(u64,u64);2]+0x0 / ty+0x20)에서 `ty==jungle` 을 find 해 **`pos[team_idx]`** 반환(못 찾으면 (0,0), TLS CAMP_POS_MEMO). ⟹ true(팀0)→pos[0], false→pos[1]. **★해소**.
- `open[2]` PassiveJunglePlan.team vs player_team: `new`(m04.ll:28355~28358)는 둘 다 인자 team, `new_counter_jungle`(28208 `sub i64 1,%2` → 28316 +72)은 **team = 1−player_team**(상대 정글 진영), player_team = 내 팀. ⟹ 카운터정글 중엔 상대 진영 캠프 좌표가 앵커. **★해소**(재구현엔 영향 없음 — 명세는 이미 +0x638 을 읽고 있었다).
- consts[8] meaning 의 「추정」 제거(ev5→4) + IR `icmp eq i64 %2, 0`(10872) 인용.
- callees 판정(§4-b): `as_index` = **Position::as_index**(entity.rs:581, `as_index:581 <=v3_assign_anchor:1774` 인라인) — 후보표의 BigGoal::as_index 는 오답. `line`·`team`·`jungle`·`plan`·`v3_armed`·`player_champion` = 필드(logic 표기 수정으로 잡음 제거 예정), `line_anchor` = 지역 클로저, `anchor` = 반환값 이름. 실제 호출은 `first_tower_position`·`camp_pos`(둘 다 이미 ev3) 뿐.
- mem 10행 → ev3(tcxdict 전수 일치). 오라클: `internal fastcc`(in:handler) → 직접 진입 재료 부재, 호출자(m13.ll 7002/7166/8193) 경유 미탐색.

### #44 take_misunderstood_received_chat — open[0] **판정 반전(exe)** · consts kind 낱말 2건
- `open[0]`: exe **0xe4b8c0** 을 disrva 로 재확인 — `[rcx+0x7c0]` len · `[rcx+0x7b8]` ptr · ×40 stride · chat 7바이트 선로드 · tick/from/tag 비교 → 점프테이블 · 일치 시 `dec rax` + 40B 복사 + `[rcx+0x7c0]=rax` + `al=1`. IR m13.ll:12483~12489·13867~13870 과 일대일. 호출자 2곳(0xe49a50·0xe4c5c0). 옛 문면(0xe70c70 의심)은 **판정 반전 = 오류 1건**.
- consts[2] −1: 「오프셋가감」 · consts[3] 40: 「길이」로 meaning 낱말 정정(kind 파생 교정 · 미상/태그 → 오프셋가감/길이).
- switch 57 케이스(tag 40 은 페이로드 없이 즉시 일치) · default `unreachable` 1건 · Chat variant 57(tcx) — 명세대로.
- mem[0]/[1]/[11] → ev3(tcxdict + exe 오퍼랜드). 원소 튜플 행(2~10)은 tcx 에 튜플 타입이 없어 IR `{ i64, i32, [1 x i32], { i8, [23 x i8] } }` 로만 지지 — ev4 유지.

## 3. 판정 어휘 정리
| 항목 | 판정 |
|---|---|
| #41 직접 오라클 | **재료 부재**(tcx `v=in:game_ai`=pub(crate), 루트 재수출 없음) — 시도 범위: tcx `v`·`p` 조회. 우회(`action_candidates` pub 호출자 경유) = **미탐색** |
| #40·#44 직접 오라클 | **재료 부재**(`internal fastcc`, in:handler) — 호출자 경유 미탐색 |
| #42/#43 `open[0]`·`[1]` | **표기 불가**(외연 동일, 오라클 520/520 로 동작만 확정) |
| #41 `open[4]` 기본값 리터럴 | **표기 불가**(형태 unwrap_or 는 dbg 로 확정) |
| #41 `open[0]`·`[2]`, #43 `open[3]`, #40 `open[3]`·`[4]`, #44 `open[1]`·`[2]` | **미탐색**(범위 밖 유지) |

## 4. 검사기 제안(다음 라운드 게이트)
- **G10 어휘 공유**: `specgate.gate10` 이 `mkspec3.FACT_TAIL`/`DECLARED` 를 import 해서 같은 판정을 내게 하라(이번 [42]/[43] open[3]/[4] 누락의 원인).
- **G7-exe**: `exe.addr` 을 바꾸는 patch 가 들어오면 `open`/`notes` 에 옛 주소 문자열(`0x[0-9a-f]{6}`)이 남아 있는지 grep 하는 게이트(spec_44 자기모순 재발 방지).
- **callees 잡음 검사**: `harvest_callees` 가 logic 에서 긁은 이름 중 **같은 logic 에 `name(+0x…)`/`name[+0x…]` 꼴로 오프셋이 붙은 것**은 필드 표기이므로 후보에서 제외하라 — 5함수에서 잡음 9행(team×2·line·jungle·plan·v3_armed·player_champion·move_speed·tick_per_second·level)이 전부 이 형태였다.
"""
io.open(r"C:\tfm2mods\MIG\_verify17\A\REPORT.md", "w", encoding="utf-8").write(R)
print("written", len(R))
