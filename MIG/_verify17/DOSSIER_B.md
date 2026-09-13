<!-- ★`mkdossier.py` 가 생성한다. 손으로 고치지 마라 — 다음 생성에 날아가고, 손으로 쓴 수치가 5차까지 반복된 지시 오류의 원인이었다. -->
# 17차 배치 B 도시에 — 담당 `45`~`49` (게임 0.5.8)

## §0 정본 스탬프 — **읽기 전에 스스로 확인하라**

| 정본 | sha256[:16] | mtime |
|---|---|---|
| `_spec/specs20_v3.json` | `68b8f9e4edee76d2` | 2026-09-13 12:48:39 |
| `_spec/specs20.json` (손이 닿는 정본) | `a7ec76fe38944d6f` | 2026-09-13 12:48:24 |

| `mkdossier.py`(이 파일을 만든 도구) | `81d9294dd98fa641` | 2026-09-13 12:49:42 |

이 파일 생성 = `2026-09-13 12:50:06`

### ★★신선도 확인 — **정본만이 아니라 「이 지시문 자신」도 확인하라**

7차에 배치 C 가 이걸 적발했다: §0 이 `_spec` 해시만 확인시키는데 **지시문 본문은 라운드 중에 바뀐다**
(다른 배치가 내 지시 오류를 보고하면 내가 그 자리에서 고치기 때문). C 가 받은 판은 17:41,
디스크 현재는 17:58 이었고 **`_spec` 해시는 일치하는데 지시문 4곳이 달랐다** — 
그중 하나가 `patch.json` 스키마라 **옛 판대로 냈으면 0건 적용**이었다.

```bash
cd /c/tfm2mods/MIG
python -X utf8 dossierfresh.py 17 B      # ← 이것만 돌리면 된다
```
`STALE` 이 뜨면 **작업을 멈추고 이 파일과 `spec_*.md` 를 다시 읽어라.**
네 patch 를 낼 때 한 번 더 돌려라 — 그 사이에 또 바뀌었을 수 있다.

> ★**v3 를 직접 고치지 마라.** 정본은 `specs20.json`(v2)이고 v3 는 `mkspec3.py` 의 생성물이다.
> 네가 할 일은 파일 수정이 아니라 **`patch.json` 제출**이다(§5).

## §1 담당 함수와 이번 라운드의 표적

| # | 이름 | vis | mem | consts | knobs | open | ev≥4(미실행) |
|---|---|---|---|---|---|---|---|
| `45` | can_recall | pub | 23 | 12 | 6 | 7 | 40 |
| `46` | need_defense_nexus | pub | 23 | 15 | 4 | 6 | 42 |
| `47` | TeamPlan::handle_epic_line_change | pub | 23 | 13 | 5 | 7 | 40 |
| `48` | v25_scoped_battle_objective | in:game_ai | 8 | 10 | 2 | 4 | 18 |
| `49` | resolve_join_stake | in:game_ai | 39 | 11 | 5 | 9 | 55 |

★**`vis` 가 `pub` 이 아니면 오라클 직접 진입이 막힌다.** 5차에 `18`(`in:game_ai`)이
그걸 모르고 들어갔다가 47행을 그대로 남겼다 — `pub` 이 아니면 처음부터 **상위 `pub` 래퍼나**
**형제 복제본**을 노려라.

★**`ev≥4`** = IR 독해(4)·추론(5)뿐이라 **실행으로 확인되지 않은 행**이다. 오라클로 내려라.
⚠단 이 수는 **실행 대상 수가 아니다** — `knobs` 상당수가 인라인된 **다른 함수의 줄**을 가리켜
네 함수에 진입해도 안 닿는다(7차 배치A 적발). 숫자를 목표로 삼지 말고 **닿는 것부터** 내려라.
`ev` 는 손으로 매기는 값이 아니라 **근거 문면에서 파생**된다(`mkspec3.evtier`) — 
숫자를 고치려 하지 말고 **근거를 바꿔라**(예: 「오라클 실행 확인: …」을 `note` 에 쓰면 2로 내려간다).

### ★★게이트가 **전부 0** 이어도 그게 「명세가 맞다」는 뜻이 아니다

게이트 20개는 **각자 한 축만** 본다. 아래는 **어떤 게이트도 보지 않는 칸**이다:

| 무검사 칸 | 왜 위험한가 |
|---|---|
| `one_line` · `layer` | 함수를 한 줄로 요약한 것. 틀리면 **읽는 사람이 통째로 오해**한다 |
| `callees[]` 의 내용 | tcx 에서 자동 생성되지만 **경로·시그니처가 맞는지는 무검사** |
| `closed[]` | 「이미 닫혔다」고 적힌 것들. **닫은 근거가 지금도 유효한지** 아무도 안 본다 |
| `notes[]` | 「확정된 사실 서술」. 그 확정이 맞는지 무검사 |
| `siblings` · `callers` | 자동 열거물. 빠진 게 있어도 모른다 |
| **아예 빠진 것** | ★9차에 `09 knobs[7]`(미니언 위험 임계표)이 **명세 어디에도 없었다**. 게이트는 *있는 칸이 틀렸나*만 본다 |

⟹ **게이트 0 = 「검사한 축이 깨끗하다」** 이지 **「명세가 완전하다」가 아니다.**

## §2 읽어야 할 정본 — **요약본이 아니라 원문을 읽어라**

여기 요약은 없다. 아래 파일을 **직접 Read** 하라. 이 표는 「무엇이 어디 있고 지금 내용이 무엇인지」만 준다.

| 파일 | 줄 | sha256[:16] | 왜 |
|---|---|---|---|
| `MIG\METHOD_MAP.md` | 214 | `5771da41d40372a2` | ★**「불가」를 쓰기 전에 §0 라우팅표를 보라.** 09-11 에 「원리적 불가」가 7건 뒤집혔고 원인은 매번 「가진 재료의 한계를 문제의 한계로 착각」이었다. |
| `MIG\SPEC_RUNBOOK.md` | 1052 | `c83594f5c4dc18fd` | ★라운드 절차. **§S5-b 게이트표 · §S5-c 「검사받지 않는 축」 · §S5-d 파이프라인 4계약**. |
| `MIG\SPEC_GUIDE.md` | 364 | `0b45fe38ad5ebf34` | 명세 필드의 뜻과 채우는 법. |
| `MIG\TOOLS.md` | 268 | `d6b6f65e638fd729` | 도구 인벤토리(자동 생성). **무엇이 있는가**만 센다 — 무엇을 집는가는 `METHOD_MAP §0`. |
| `MIG\_verify3\TEMPLATE.rs` | 226 | `c7f1a3efd83d7087` | ★오라클 작성 템플릿. **함정 ①~⑧ · 수법 ⓐ~ⓕ** — 오라클을 쓸 거면 먼저 읽어라. |

## §3 담당 함수 명세 **전문** (무손실 — 잘린 칸이 없다)

> 표의 어떤 칸도 `...` 로 줄이지 않았다. 길어 보이는 게 정상이고, **줄인 자리가 곧 지시 오류가 난 자리**였다.

★**함수마다 파일이 따로 있다. 한 번에 하나씩 Read 하라.**
(7차 첫 시도에서 도시에를 통째로 읽으려던 배치 둘이 600초 무진전으로 죽었다. 내용은 그대로고 파일만 나눴다.)

| # | 이름 | 파일 |
|---|---|---|
| `45` | can_recall | `MIG\_verify17\B\spec_45_can_recall.md` |
| `46` | need_defense_nexus | `MIG\_verify17\B\spec_46_need_defense_nexus.md` |
| `47` | TeamPlan::handle_epic_line_change | `MIG\_verify17\B\spec_47_TeamPlan__handle_epic_line_change.md` |
| `48` | v25_scoped_battle_objective | `MIG\_verify17\B\spec_48_v25_scoped_battle_objective.md` |
| `49` | resolve_join_stake | `MIG\_verify17\B\spec_49_resolve_join_stake.md` |

## §4 게이트 미해소 — **이 배치 몫만 추렸다**

★**이번 라운드의 주 표적이다.** 6차 통제실험 결론 = 「오류는 **검사받지 않는 축**에 고인다」 — 
`consts.src_line` 은 5라운드 동안 아무도 안 봤고 G12 를 붙이자 **즉시 56건**이 나왔다.
그 오류들은 **1차부터 그대로 있었다.** 아래는 아직 안 닫힌 것들이다.

**이 배치 몫 = 15건** (게이트 머리줄의 건수는 **20함수 전역 합계**다 — 혼동 말 것)

⚠★**게이트의 「실제 후보」를 그대로 믿지 마라.** 7차 배치A 가 G12 8건을 전수 검증했더니 **전부 오탐**이었다 — `srclinecheck.py` 가 ①`switch` case 라벨 줄에 `!dbg` 가 없어 못 보고 ②`phi` 상수를 `line 0` 으로 버리고 ③리터럴 정규식이 `%2`·`%16` 같은 SSA 레지스터와 gep 오프셋까지 잡는다. **지적된 줄의 IR 원문을 직접 열어 확인**하고, 명세가 옳으면 `kind: "오탐"` 으로 보고하라(고치는 게 아니라 게이트를 고쳐야 한다).

```

**[G10 class 오분류] 17건**
  [46] need_defense_nexus                              class=미탐색 인데 문면이 **사실 서술**로 끝난다 — 다음 라운드가 이걸 또 판다
  twin_towers[team][0] 만 쓰고 [1] 은 안 본다(첫 원소 = first/twin dbg). 두 쌍둥이 중 어느 쪽이 [0]인지 미확정
  [47] TeamPlan::handle_epic_line_change               class=미탐색 인데 문면이 **사실 서술**로 끝난다 — 다음 라운드가 이걸 또 판다
  ckboard.top_minion_state.minion_count 가 '우리 미니언 수'인지 '적 미니언 수'인지 — 이름만 근거. blackboard[team](우리 팀 판) 인 것은 IR 확정
  [49] resolve_join_stake                              class=미탐색 인데 문면이 **사실 서술**로 끝난다 — 다음 라운드가 이걸 또 판다
  의 조건항'인지 'L729 의 명시 가산'인지 확정 못 함(Effect::range 는 _gaibc/_gcbc 어디에도 define 없음 — 전부 인라인). 동작(타워 경로에만 +15000)은 확정
  [49] resolve_join_stake                              class=미탐색 인데 문면이 **사실 서술**로 끝난다 — 다음 라운드가 이걸 또 판다
  y_team], game data, game vtable, player, e) 의 의미('적팀 판'이 무엇을 기록하는지)는 game_core 본문 미열람. 인덱스가 적팀(1-team) 인 것만 확정
  [49] resolve_join_stake                              class=미탐색 인데 문면이 **사실 서술**로 끝난다 — 다음 라운드가 이걸 또 판다
  사용 패턴으로 읽은 것 — 타워 6개 배열 + 추가 슬라이스라는 구조 자체는 IR 타입(`IntoIter<Option<&Entity>,6>` + `Copied<slice::Iter>`) 으로 확정

**[G12 src_line 대조] 7건**
  [47] TeamPlan::handle_epic_line_change               consts[10] src_line=719 인데 IR 사슬에 그 줄이 없다
  실제 후보 = [697, 715, 716, 725, 730, 742, 745, 749]
  [47] TeamPlan::handle_epic_line_change               consts[11] src_line=721 인데 IR 사슬에 그 줄이 없다
  실제 후보 = [691, 693, 694, 706, 707, 708, 709, 711]
  [47] TeamPlan::handle_epic_line_change               consts[12] src_line=697 인데 IR 사슬에 그 줄이 없다
  실제 후보 = [691, 693, 694, 702, 706, 707, 711, 712]

**[G15 consts.kind 관측] 1건**
  [47] TeamPlan::handle_epic_line_change               consts[3] kind=임계 NEG
  `meaning` 이 「) ⚠`shl … , 5` 는 포탑 배열 stride(line*32) 접힘이지 이 임계가 아님」라 적었는데 kind=임계

**[G16 params.role 대조] 1건**
  [48] v25_scoped_battle_objective                     sig.params[0] P1 정렬 불변식 깨짐(SROA 인자 승격 미기재)
  IR 인자 6개(sret보정 -0) > params 5개인데 role 이 `%k` 로 승격 대응을 전부 적지 않았다(적은 것 p[4], 덮은 레지스터 ['%4', '%5']). 자리 매핑 보류

**[G4 술어 시그니처] 6건**
  [45] can_recall                                      callees_unmatched 8개 — 판정에 쓰이는 술어가 섞였는지 손으로 확인
  cache, focused, fountains, invisible_tick, move_speed, pool, return_tick, setting
  [47] TeamPlan::handle_epic_line_change               callees_unmatched 10개 — 판정에 쓰이는 술어가 섞였는지 손으로 확인
  bottom_wave, change, first, grow_one, minion_count, morgard_use, next_respawn_tick, outer, second, tower2
  [49] resolve_join_stake                              callees_unmatched 11개 — 판정에 쓰이는 술어가 섞였는지 손으로 확인
  block_target_tick, drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다, line_absolute, move_speed, my_team,

**[G7 과열림] 3건**
  [46] need_defense_nexus                              ★open 이 `shared.is_recent_visible` 를 부르는데 그 항목은 **확정**이다 — 과열림
  Blackboard::is_recent_visible(self=blackboard[1-team], game, vtable, player, e) 내부(_gcbc) 안 봄 — '적팀 판'을 self 로 넘기는 것만 확인
  [49] resolve_join_stake                              ★open 이 `shared.is_recent_visible` 를 부르는데 그 항목은 **확정**이다 — 과열림
  Blackboard::is_recent_visible(&Blackboard[enemy_team], game data, game vtable, player, e) 의 의미('적팀 판'이 무엇을 기록하는지)는 game_
```

### §4-b 아직 **게이트가 없는 축** — 여기를 의심하라

| 축 | 전체 건수 | 상태 |
|---|---|---|
| `mem.dir`(읽기/쓰기 방향) | 1242 | ✅**G14** — IR load/store 대조 |
| `consts.kind`(상수 종류) | 591 | ✅**G15** — IR 소비 오프코드 반증식 |
| `sig.params.role`(인자 역할) | 325 | ✅**G16** — `define` 속성 대조 |
| `callees[]` 의 **경로·시그니처** | 813 | ⚠**138행만 IR 앵커**(`ev3`). 나머지 675행은 `ev4` + 「미확정」 — 같은 leaf 이름 후보가 여럿이라 **하나를 고른 것이 아니라 못 고른 것**이다 |
| 명세 **사이**의 같은 사실 | — | ✅**G20**(11차 후속 신설) — 유일한 cross-spec 게이트 |

⚠`consts.kind` 는 **v2 에 없는 파생 필드**다(`mkspec3.py` 가 `meaning` 의 낱말 + IR 관측으로 정한다) — 
`errors[]` 로 직접 못 쓰니 **`meaning` 쪽 낱말**을 고쳐야 한다(7차 배치A 가 3건 거부당했다).
어휘 = `임계`·`태그`·`센티널`·`인덱스`·`계수`·`산출값`·`오프셋가감`·`길이`(뒤 넷은 확장).

★**`callees` 의 `ev4` 행을 「틀렸다」로 읽지 마라.** 그건 **판정 보류**(재료 부재)다 — 
11차 후속 이전에는 이 행들이 전부 `ev3`(tcx 정본) 도장을 받고 있었고 그게 **거짓 확증**이었다.
담당 함수의 `ev4` 행 중 **판정에 실제로 쓰이는 술어**가 있으면 IR 호출 심볼로 확정해 달라.

⟹ 담당 함수에서 **표본을 떠서 직접 대조하라.** 틀린 게 나오면 그건 「그 축 전체가 무검사였다」는 뜻이고,
보고에 **검사기를 어떻게 만들면 되는지**까지 적어라(그게 다음 라운드의 게이트가 된다).

★**불일치 0 도 결과다.** 7차 배치A 가 `mem.dir` 89행을 대조해 **불일치 0** 을 냈다 — 
「계측기를 붙이면 오류가 나온다」가 법칙이 아니라는 반대 사례이고, 그 축은 닫아도 된다는 뜻이다.

## §5 보고 형식 — **`patch.json` 하나. 산문 보고서는 부수적이다**

★5차에 ev 상향 **492행 중 317행이 유실**됐다. 원인은 보고를 **집계표로 받은 것**이다.
기계가 적용할 수 있는 형식으로만 받는다 — 사람이 옮겨 적는 경계를 없앤다.

★**이 스키마는 `applypatch.py` 의 실제 계약이다.** 다른 최상위 키를 쓰면 도구가 **적용을 거부**한다
(7차 배치A 적발 — 이 자리에 없는 계약 `entries` 가 예시로 실려 있었고, 그대로 냈으면 **0건 적용**이었다).

```json
{
  "round": 17, "batch": "B",
  "errors": [
    {"path": "/specs[45]/consts[3]/src_line",
     "kind": "실오류",
     "old": 293, "new": 310,
     "evidence": "m04.ll:44120 `store i64 2, !dbg !56400` (!56400 = abstract_input.rs:310).
                   293 은 함수 머리줄이다",
     "behavior_change": false,
     "found_by": "new"}
  ],
  "ev_up": [
    {"path": "/specs[45]/mem[0]", "from": 4, "to": 3,
     "evidence": "tcx 정본 대조: offset_of!(PlayerState, info.team)==0x930",
     "found_by": "new"}
  ],
  "brief_errors": ["이 도시에에서 발견한 내 지시의 오류 — 문장으로"]
}
```

| 키 | 뜻 |
|---|---|
| `errors[]` | 값·문면 정정. `kind` = `실오류` / `오탐`(게이트가 틀렸다) / `보강` |
| `errors[].evidence` | ★**IR 줄 원문을 인용하라.** 「확인했다」는 근거가 아니다 |
| `errors[].behavior_change` | ★**이 명세대로 재구현하면 틀린 동작이 나오는가**(고친 뒤엔 안 나오는가). 「지금 재현 코드가 바뀌나」가 **아니다** — 7차에 네 배치가 그렇게 읽어 전건 `false` 로 냈는데, 실제로는 노브 arm 방향 뒤집힘·쓰기 대상 오기·존재하지 않는 노브 등 **8건이 재구현을 틀리게** 했다 |
| `errors[].found_by` | `new`(이번에 처음) / `reused`(전 라운드 기법 재사용) — **집계에 쓴다** |
| `ev_up[]` | 증거등급 **이동**. `from`/`to` 는 숫자지만 **`ev` 필드를 직접 쓰는 게 아니라** 근거 갱신 요청이다 |
| `brief_errors[]` | ★**내 지시(이 도시에)의 오류.** 매 라운드 나왔다 — 비워 두지 마라 |

제출 전 **반드시** 사전 검증하라(적용 없이 성공/실패만 본다):
```bash
cd /c/tfm2mods/MIG && PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 17 --only B --dry
```

경로 문법(`applypatch.py` 가 파싱한다):

```
/specs[i]/<필드>                     예: /specs[3]/one_line
/specs[i]/<배열>[n]/<키>             예: /specs[3]/consts[2]/src_line
/specs[i]/<바깥>/<필드>              예: /specs[3]/sig/vis
/specs[i]/<바깥>/<배열>[n]/<키>      예: /specs[3]/sig/params[1]/role
```

규칙:
- `old` 는 **현재 값과 정확히 일치**해야 한다(불일치 = 그 항목만 거부 · 나머지는 적용).
  정수·불리언 필드(`src_line` 등)는 **문자열로 써도 된다** — 도구가 현재 필드 타입으로 되돌려 넣는다.
  ⟹ 네가 본 값이 정본과 다르면 **네 도시에가 낡은 것**이다. §0 해시부터 다시 확인하라.
- ★**행 추가·삭제가 된다** — `{"op":"insert", "path":"/specs[i]/mem", "at":N, "guard":"<식별 문자열>", "guard_key":"name", "new":{...}}` · `{"op":"delete", ...}`.
  ⚠**삽입은 뒤 인덱스를 전부 민다** — 같은 배열의 `ev_up` 경로는 **삽입 후 인덱스**로 써라. 적용 뒤 메인이 `--restamp` 를 돌린다.
  ⛔단 `open`/`notes` 에는 **못 쓴다**(v3 가 v2 의 두 배열을 걸러 만든 것이라 인덱스가 안 맞는다). 그쪽은 산문으로 보고하면 메인이 넣는다.
- ★**`ev` 를 `errors[]` 로 직접 쓰지 마라** — `ev` 는 근거 문면에서 파생되는 값이다(`mkspec3.evtier`). 등급을 옮기려면 **`ev_up[]`** 을 쓰고 `evidence` 에 새 근거를 대라.
  (`mem` 의 `ev` 는 근거가 무엇이든 **상한 3**이다. 오프셋 주장의 최강 근거가 tcx 이기 때문.)
- ⚠**파생 필드는 `errors[]` 로 못 쓴다.** `consts.kind` 는 v2 에 없고 `mkspec3.py` 가 `meaning` 의 낱말에서 정한다 — 분류를 고치려면 **`meaning` 쪽 낱말**을 고쳐라(7차 배치A 적발).
- ⛔`_spec/specs20*.json` 을 **직접 쓰지 마라**(읽기 전용). 반영은 메인이 `applypatch.py` 로 한다.

생성 보조: `python -X utf8 mkpatch.py --help` (참조구현 — 손으로 JSON 을 쓰다 오타를 내지 마라)

### §5-b 산문 보고서에 **반드시** 적을 것

1. ★**판정 어휘를 골라 써라** — 「불가」 한 단어로 뭉치면 다음 라운드가 시도조차 안 한다:
   - `미탐색` = 아직 안 해봤다
   - `재료 부재` = 해봤는데 재료가 없다 → **어떤 재료를 어떻게 시도했는지 범위를 열거**하라
   - `표기 불가` = **동작은 확정됐는데** 명세 칸에 담을 형식이 없다
   - `사실 서술` = 물음이 아니다 → `notes[]` 로 보내라(`open` 에 두지 마라)
2. **무엇을 실제로 실행했는지** — 명령줄 그대로. 「확인했다」만 쓰면 재현이 안 된다.
3. **내 지시(이 도시에)의 오류**를 찾으면 그것부터 보고하라. 6차까지 매 라운드 나왔다.
4. **판정 반전도 오류로 센다**(유저 확정). 「전에 A 라 했는데 B 였다」면 그건 오류 1건이다.

## §6 출력 디렉터리 — **_verify17/B 만 써라**

```
MIG/_verify17/B/patch.json      ← 기계 적용분(필수)
MIG/_verify17/B/REPORT.md       ← 산문 보고서
MIG/_verify17/B/oracle/         ← 오라클 .rs · 출력 로그
```

⚠**다른 배치 폴더에 쓰지 마라.** 네 배치가 동시에 돌아 경합한다.
⚠`_spec/` 아래 어떤 파일도 쓰지 마라(읽기 전용).
⚠`distruct.json`·`dienum.json` 은 **절대 수정 금지**.

