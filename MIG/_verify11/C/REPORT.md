> ⚠**이 파일은 배치 C 가 직접 못 남겼다.** 그 하네스가 보고서 `.md` 작성을 막아서,
> 배치가 최종 응답으로 돌려준 본문을 **메인이 원문 그대로** 옮겼다(2026-09-11).
> `patch.json` 은 배치가 직접 디스크에 썼다.

---

# 11차 배치 C — 전수 감사 (함수 `10`~`14`) · 게임 0.5.8

## 0. 한 줄 결론

**10차가 고친 자리에서는 새 오류가 0건이었다.** 새로 나온 것은 전부 **10차가 「보이게 만든 축」의 뒤쪽** — `closed[].why` 를 **생성하는 기계**, `callees[]` 를 **생성하는 기계**, 그리고 **삽입이 깨뜨린 산문 안의 인덱스 참조** 다. 10차는 「칸이 비어 있었다」를 고쳤고, 11차는 「그 칸을 채우는 규칙이 틀렸다」를 찾았다.

## 1. 10차 후속 확인 — 지시받은 5건, 전부 제대로 실렸고 전부 재검증 통과

| 10차 항목 | 11차 재검증 | 판정 |
|---|---|---|
| `12` 두 번째 게이트 = 술어(`morgard_exists`/`serpen_exists`/`line_exists`) | `knobs[3]` 에 그대로 실림. `logic` L60~62 도 동기됨 | **OK** |
| `10` 스킬2 `level>2` · 궁 `level>4` (`knobs[6]` 행 추가) | `_gaibc/m10.ll:47564` `%47 = icmp ugt i64 %46, 2` · `47594` `%59 = icmp ugt i64 %46, 4` · `%46 = load (ptr %0 + 1480 = 0x5c8)` — **줄번호까지 정확** | **OK** |
| `10 closed[3]` 판정반전 | 문면에 정정 표기 그대로 | **OK** |
| `mem` 행 추가 `12 mem[5]`·`14 mem[1]` | `m13.ll:29570·29575` / `m08.ll:94879·94880·94882·94921~94922` — 인용 줄 전부 정확 | **OK** |
| 10차 tcx 오프셋 3건 | `tcxdict` → `0x1802 v2_assign@Some.0.0:u8` · `0x180a v3_epicops_armed:bool` · `0x1658 mf_ret25_src[0]:usize` | **OK** |

⟹ **10차 정정분에서 새로 발견된 오류 = 0건.**

## 2. 함수별 발견

### `10` should_end_object_finish_kill_priority_battle
- ★**`Blackboard::is_recent_visible` 가 「120틱 기억」만이 아니다 — 앞에 즉시통과 2단이 있다.** `_gcbc/g07.ll:157005~157049`: ① `game.is_visible(player.info.team /*+0x930*/, enemy.champion_id /*Entity+0x5c0*/)`(vtable **+0xf8**, divtable 확인)가 참이면 **즉시 true** → ② `get_player_by_champion_id`(vtable **+0x150**)가 null 이면 **즉시 false** → ③ `last_visible[p.info.position] + 120 >= tick`. phi(157047) `[true,%5] [%29,%18] [false,%13]` 가 못박는다. 명세는 `logic`·`knobs[4]` 둘 다 **③만** 서술 ⟹ ③만 재구현하면 **지금 눈앞의 적**을 놓친다. → patch 2건, `behavior_change=true` **1건**(이 라운드 유일).
- `logic` 전 분기 재검증: 진입마스크(`and i24 %4, 65534`/`icmp eq 768`)·전략게이트(`+0xf`==0)·비-Moba→true·live_list len==0→true·objective null→true·`can_enemy_hit_objective`→false·폴스루→true. `m10.ll:50098` 13-인입 phi 와 **전건 일치**. 정정 없음.
- callers 3곳 grep 재확인 일치. `consts` 6 · `knobs` 8 · `sig.params` 5 불일치 0.

### `11` v3_fall_back_to_passive
- ★★**`logic` 의 시그니처 3곳이 tcx 정본과 어긋난다**(G5 는 `sig` 필드만 보고 `logic` 은 안 본다):

| 자리 | 명세 | tcx 정본 | IR 증거 |
|---|---|---|---|
| `BigPlan::sub_plan` 수신자 | `&self.plan` | **`&mut BigPlan`** | m13.ll:12461 의 `%81` 에만 `readonly` 없음 |
| 7번째 인자 | `&mut self.team_plan(0xf8)` | **`&TeamPlan`**(불변) | `mem[10]` 도 `dir=r` |
| `SubPlan::merge` 2번째 인자 | `&sp` | **값 전달** `SubPlan` | m13.ll:12466 `readonly captures(none) dereferenceable(72)` = 간접 by-value |

- ★**`history[11]` 의 `mf_note_obj_clear` 위치 오기.** `handler.rs:197` → 실제 **`TeamPlan::mf_note_obj_clear` @ `team_plan.rs:196`**, 소유 타입도 `LegacyPlanHandler` 가 아니다. 결정적 근거: `LegacyPlanHandler` AssocFn 전량이 **정확히 41개**(= `siblings` 41행과 동수)인데 그 안에 없다. `mf_obj_clear` 도 `TeamPlan` 필드(team_plan.rs:52)고, `LegacyPlanHandler` 필드는 `mf_ret25_src`(handler.rs:159)뿐.
- ★★`callees[]` 에서 실제 피호출자 3개가 전부 빠졌다(§3).
- `closed[4]/[5]/[6]` 이 **같은 닫은 근거를 공유**(§3) — [5]·[6] 은 그 근거로 안 닫힌다.
- 나머지 `logic`(버전<2, DeathMatch==2, SingleLane 하드코딩, goal switch 0~6+default unreachable, 화이트리스트 6표, mf_swap=(29,tick), +1, 384B memcpy) 전부 일치. `consts` 19·`mem` 23·`sig.params` 6 불일치 0.

### `12` handle_chat
- ★**행 삽입이 산문 안의 인덱스 참조를 깨뜨렸다.** `knobs[3]`(10차에 `new_knobs[0]` 으로 삽입된 그 행)이 `push_line_code_allowed` 를 **`knobs[7]`** 로 두 번 가리키는데, 삽입으로 밀려 지금 그것은 **`knobs[8]`**(`knobs[7]` = 「chat_allowed default=true」).
- `closed[1]`(「`chat_allowed` 내부는 안 봄」)의 닫은 근거가 **「2차 배치C: 45칸 전수 일치」** — 45칸은 `position_exists` 결과지 `chat_allowed` 의 답이 아니다. `closelist.py:290` 에 **「3차 배치C: 297칸 전수 진리표」가 실제로 있는데 첫 매치 우선 때문에 못 뜬다**(§3).
- 10차 술어 정정 제대로 실림. `mem` 26·`consts` 12·`sig.params` 9 불일치 0.

### `13` target_bush_v30
- ★★**`mem` 에서 이 함수의 입력 3칸이 통째로 빠져 있었다** — `LineGankCoverPlan+0x20 line` · `PlayerState+0x930 info.team` · `PlayerState+0x9c0 info.position`. `sig.params` 와 `logic` 은 세 오프셋을 다 적는데 오프셋 정본인 `mem` 에만 없다. 게다가 **`closed[2]` 가 4차에 「간접증거→직접증거, 호출부 m10.ll:11778~11791 에 gep 그대로」로 이미 닫혔는데 그 결론이 표에 전파되지 않았다**(G17 형). 호출부 원문: `11762` `%8 = gep %4, i64 2352` → team(+`11764` `icmp ult %9,2`/`11768` `panic_bounds_check` — **경계검사도 호출부가 한다**) · `11773` `%13 = gep %4, i64 2496` → position · `11787` `%23 = gep %1, i64 32` → line.
  ★**결정적 비대칭**: `notes[0]` 이 「cover ↔ ganker 는 **문자 단위 동일 복제본**」이라 확정했는데 **복제본 명세 `14` 는 같은 세 필드를 `mem[3]/[4]/[5]` 로 싣는다.** → **행 추가 3건**(`dir:"-"` = 기존 참조용 행 관례 준수).
- `callees[2]` 가 **형제**(`LineGankerPlan::target_bush_v30`), `callees[3]` 이 **자기 자신** — §3 부작용.
- `knobs` 11·`consts` 16·`sig.params` 3·`siblings` 10 불일치 0. `target_bush_v41`(ganker.rs:310) 존재 tcx 재확인.

### `14` update
- IR 전수 재검증 통과: `%24 = gep %21, i64 1576`(0x628) · `%29 = gep %21, i64 1648`(0x670) · `mul 100`/`udiv` · **`94618` `icmp ult i64 %32, 41`**(knobs[0] 인용 정확) · `%36 = gep %0, i64 40`(0x28). `consts` 21·`mem` 28·`knobs` 7·`sig.params` 8·`siblings` 14 불일치 0.
- `closed[7]` 말미의 **잔여 미탐색 2건이 stale**: 「`target_bush_v41` Mid 분기」는 같은 파일 `knobs[5]` 가 84/84 로, 「`*_lead` 갱신 주체」는 `history[5]` 가 `new_with_prev_cache`(simulation.rs:1780)로 **이미 닫았다**. G7 은 `open[]` 만 보고 `closed[]` **안에 적힌** 미탐색 목록은 안 본다.
- `closed[7]` 의 닫은 근거(「`*_lead` 산출식 확정 6/6」)가 그 물음(「`_version` 미사용 일반화 금지」)의 답이 아니다(§3).
- `notes[0]` 말미 「같은 오류 문면이 `SPEC_GUIDE.md` §1 fnparts 에도 있다」는 **stale**: 현행 L87~L94 는 정반대로 「★★소유 타입이 다르다고 조각을 버리지 마라 — 먼저 복제본인지 확인하라」 + cover↔ganker 53줄 일치 근거를 싣고 있다.

## 3. ★★두 개의 **생성 규칙** 결함 — 이번 라운드의 본체

둘 다 명세가 아니라 **명세를 만드는 코드**라 `errors[]` 로 못 고친다(`_spec/` 쓰기 금지 · 파생 필드).

### (a) `callees[]` — v2 정본의 **완전 경로를 버리고 leaf 이름으로 재검색**한다
`mkspec3.harvest_callees`(L272~274)가 `c.split("::")[-1]` 로 깎고, L429 가 `L.fnlookup(n)` 결과를 **`rs[:3]`** 로 잘라 싣는다. v2 `calls[]` 에는 정확한 전체 경로가 **이미 들어 있다.**

`11` 실측 — v2 `calls` = `passive_plan` / **`BigPlan::goal`** / **`BigPlan::sub_plan`** / **`SubPlan::merge`** / `drop_glue`. v3 `callees` 26행에 굵은 셋이 **하나도 없다**:

| leaf | tcx 동명 후보 | 실린 3개 | 실제 피호출자 |
|---|---|---|---|
| `goal` | 16 | PassiveLinePlan / SinglePlanLine / SinglePlanBattle | **`BigPlan::goal`**(types.rs:132) — m13.ll:12308 |
| `sub_plan` | 16 | 같은 3판 | **`BigPlan::sub_plan`**(types.rs:230) — m13.ll:12461 |
| `merge` | 19 | SmallActionPlay/RunAway/Recall | **`SubPlan::merge`**(sub_plan/mod.rs:186) — m13.ll:12466 |

부수: 같은 이유로 `11 callees[25]`·`12 callees[4]`·`13 callees[3]` 이 **자기 자신**을 피호출자로 싣는다.

> ★**반증 통과**: 기계 대조로는 「v2 경로가 v3 에 없다」가 **117 중 106건**이었으나, 끝 2성분 재대조하니 대부분 **경로 표기 차이**(`game_ai::path_finder::is_enemy_well_danger` ↔ 루트 재수출 `game_ai::is_enemy_well_danger`)였다. 진짜 결손은 **`11` 의 3건**이고 나머지는 `panic_bounds_check`·`unwrap_failed`·`grow_one`·`drop_glue` 같은 패닉/alloc 헬퍼(CLAUDE.md §3 상 재현 대상 아님). ⟹ **106 → 3.**

**고치는 법**: `c` 에 `::` 가 있으면 **경로로 먼저 조회**하고 실패 시에만 leaf 폴백.

### (b) `closed[].why` — **첫 매치 우선**이라 늘 *가장 약한* 근거가 이긴다
`_spec/closelist.py:closed_reason(i, txt)` 는 `(i, needle, why)` 를 선언 순서대로 훑어 첫 매치를 반환한다. `CLOSE` 는 라운드순 `+=` 이므로 **언제나 가장 이른 라운드의 why 가 이긴다.**

1. **중복 키**: `(12, "chat_allowed(GameContext")` 가 87행(2차 「45칸」)·290행(3차 「297칸 전수 진리표」)에 둘 다 있고 2차 것이 뜬다. 중복 `(i,needle)` 쌍 = **6개**(`3/반환값 die 가`·`12/chat_allowed(GameContext`·`14/has_near_line_enemy`·`17/Prepare`·`18/v3_epic_group_line`·`19/map 클로저(s_0)`).
2. **짧은 needle 이 여러 물음을 삼킨다**: `(11,"passive_plan")` **하나**가 `11 closed[4]/[5]/[6]` 세 물음에 같은 why 를 달았다(진짜 답은 각각 `history[5]`·`history[4]/[6]`). `(14,"_lead")` 가 `14 closed[7]` 을 집었다.

**20함수 전량: `closed` 151행 중 26행**이 같은 명세 안에서 why 를 공유한다(「본문에 해소 표기가 있다」 제외).
**고치는 법**: ①중복 키 제거(또는 뒤엣것 우선) ②needle 길게/`(i,needle)` 유일키 강제 ③복수 매치 시 **최장 needle** 채택.

> ★이건 **10차가 열어 준 축**이다 — 그 전까지 `why` 컬럼은 렌더러가 `a`/`ev` 키를 찾느라 전 함수에서 공백이었다.

## 4. 「불일치 0」도 결과다

| 축 | 대조 범위 | 결과 |
|---|---|---|
| `sig.params[].type`(10차에 처음 보임) | 5함수 **31 인자 전량** ↔ tcx sig | **불일치 0** |
| `siblings` 전 행(10차에 처음 보임) | `11`/`12` 41행 ↔ tcx AssocFn 41개 · `13` 10 · `14` 14 | **불일치 0**(이 동수 검산이 `11 history[11]` 오기를 잡았다) |
| `knobs.where` IR 줄 인용 | `10 knobs[5]/[6]`·`14 knobs[0]`·`11 knobs[5]~[7]`·`12 mem[5]`·`14 mem[1]` | **전건 정확(±0줄)** |
| `one_line`·`layer` | 5함수 | **불일치 0** |
| `logic` 제어흐름 | `10` 13-인입 phi 전수 · `11` goal switch 7-case · `14` HP 게이트 | **불일치 0** |
| `consts` 값·의미 | 6+19+12+16+21 | **불일치 0** |

## 5. 실제로 돌린 것

```bash
cd /c/tfm2mods/MIG
python -X utf8 dossierfresh.py 11 C                      # 착수·제출 직전 (둘 다 FRESH)
PYTHONIOENCODING=utf-8 python -X utf8 tcxdict.py LegacyPlanHandler 0x1802 / 0x180a / 0x1658
PYTHONIOENCODING=utf-8 python -X utf8 divtable.py AbstractGame 0xf8    # → is_visible
PYTHONIOENCODING=utf-8 python -X utf8 divtable.py AbstractGame 0x150   # → get_player_by_champion_id
PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 11 --only C --dry  # 11/11 성공
```
IR 원문(`awk 'NR>=a && NR<=b'`): `_gaibc/m10.ll` 47490~47600·49611~50100·11745~11795 / `_gaibc/m13.ll` 12238~12479 / `_gaibc/m08.ll` 94590~94640·94879~94922 / `_gcbc/g07.ll` 157000~157060.
tcx = `_tcx/game_ai.json`(items 13,459) 직접 파싱. 생성 규칙 대조 = `mkspec3.py` L270~284·L415~440 · `applypatch.py` L60~330 · `_spec/closelist.py` 전량(읽기만).

## 6. patch 요약

| | 건수 |
|---|---|
| `errors[]` **11건**(실오류 7 · 보강 4) | `10` 2 · `11` 4 · `12` 2 · `13` 3 |
| **행 추가**(`op:insert`) | **3건**(`/specs[13]/mem` at 0/1/2) |
| `behavior_change=true` | **1건**(`10 logic`) |
| `ev_up[]` | 0 |
| `found_by` | `new` 11 / `reused` 0 |
| 오탐 반증 | **4건** |
| `brief_errors[]` | 7건 |

⚠**삽입 3건 때문에 적용 후 `--restamp` 필수**(`/specs[13]/mem` 이후 15행이 밀린다).

### 내가 스스로 반증해서 뺀 것 (오탐 4건)
1. `callees` 결손 「117 중 106」 → 재대조로 **3건**으로 축소.
2. 「`12 callees` 에 `handle_chat_inner` 가 없다」 → 실제로 `callees[5]` 에 있다(`<impl ...>` 표기 때문에 내 스크립트가 놓쳤다). **기각.**
3. 「`10 mem` 에 `Entity+0x5c8 level` 이 없다」 → `mem` 은 선언 `ir` 범위 안만 싣고 `consts`/`knobs` 만 아웃오브라인 콜리로 나가는 **일관된 관례**(`13`·`14` 동일). **결함이 아니라 규약** ⟹ 「부재를 결함으로 세지 마라」 적용, patch 에서 뺐다.
4. 「`10 knobs[6]` 의 47564 가 한 줄 어긋난다」(손으로 세어 47563으로 봄) → `awk` 로 찍으니 **47564 가 정확**. **기각.** (교훈: 줄번호를 손으로 세지 마라 — 필터링된 `#dbg_value` 가 계수를 망친다.)

## 7. 판정 어휘로 남기는 것
- **사실 서술** — `mem` 은 선언 `ir` 범위 안만, `consts`/`knobs` 는 아웃오브라인 콜리까지. 5함수 일관. 바꾸려면 20함수 동시에.
- **미탐색** — `13`↔`14` 가 문자단위 복제본이라는 `notes[0]` 결론을 `mem`·`consts`·`knobs` **전 행 1:1 diff 로 검산하지 않았다**(이번엔 입력 3칸 결손만). **다음 라운드의 값싼 표적.**
- **재료 부재 아님** — `closed.why`·`callees` 는 `_spec/` 쓰기 금지라 배치가 못 고칠 뿐, 재료는 전부 있다. 메인이 `mkspec3.py`·`_spec/closelist.py` 두 파일을 고치면 끝.

## 8. ★10차 대비 — 무엇이 줄고 무엇이 남았는가

| | 10차(배치 4개 합) | 11차 배치 C(함수 5개) |
|---|---|---|
| 정정 | 82 | 11 |
| 행 추가 | 17 | 3 |
| 판정반전 | 2 | 0 |
| `behavior_change` | 5 | 1 |
| 지시 오류 | 30 | 7 |

**줄어든 것 — 「칸이 비어 있다」류가 사실상 고갈됐다.** 10차의 수확은 ①`closed` 화석 22건 ②`mem` 누락 ③렌더러가 숨긴 3축이었다. 이번에 같은 5함수를 같은 방법으로 훑으니: `closed` **38행 전수 재검토 → 화석 0건**(도구가 바뀌어 무효가 된 근거 없음) · `mem` 누락은 **`13` 3건뿐**이고 그것도 10차가 열어 준 「복제본 명세 `14` 와 대조」로 나왔다 · `sig.params` 타입과 `siblings` 전 행은 **불일치 0 ⟹ 이 두 축은 닫아도 된다.**

**남은 것 — 축이 「데이터」에서 「데이터를 만드는 규칙」으로 한 단 올라갔다.** 새로 나온 7건 중 5건(`callees` 3 · `closed.why` 오귀속 · `knobs[N]` 참조 깨짐)이 명세 문면이 아니라 **생성기·인덱스 규약**의 결함이다. 10차가 「보게 만든 것」을 대조하자마자 **그 칸을 채운 규칙이 틀렸다**가 드러났다 — `S5-c` 의 「오류는 검사받지 않는 축에 고인다」가 한 계층 위에서 재현됐다.

**10차 배치C 의 관측도 그대로 성립**: 게이트 19개는 전부 0이었고, 이번 11건은 **전부 IR·tcx 원문에서 역으로 찾은 것**이다 — 게이트가 준 후보는 0개.

### 다음 라운드 권장 표적 (값싼 순서)
1. `mkspec3.harvest_callees` 경로 우선 조회 + `rs[:3]` 제거 → **20함수 `callees` 가 한 번에 정확해진다**.
2. `_spec/closelist.py` 중복 키 6개 정리 + 최장 needle 채택 → **closed 151행 중 26행의 why 가 바로잡힌다**.
3. `specgate` 신설 게이트 = **「같은 명세 안의 `knobs[N]`/`mem[N]`/`consts[N]` 산문 참조가 실제 그 행을 가리키나」**(순수 기계 검사, 삽입마다 깨진다).
4. `13` ↔ `14` 행 단위 diff.

---

## 내 지시(도시에)의 오류 — `brief_errors[]` 7건 요지
1. ★★`callees[]` leaf 재검색 + `rs[:3]` (§3a).
2. ★★`closed[].why` 첫 매치 우선 + 중복 키 6개 (§3b).
3. ★삽입은 **행 본문에 적힌 인덱스 참조도 깨뜨린다** — §5 경고는 `ev_up` 경로만 말한다.
4. ★`op:insert` 의 `at` 은 v3 인덱스지만 `applypatch.resolve` 가 v2 **두 배열 경계**로 되돌린다 — `13` 은 `writes` 가 0행이라 `at=15` 로 쓰면 조용히 `writes[0]` 에 꽂힌다. §5 에 한 줄 추가 요망(그래서 삽입 3건을 전부 at=0/1/2 로 넣었다).
5. `14 notes[0]` 의 SPEC_GUIDE 경고가 stale(현행은 이미 정반대로 고쳐져 있다) — `notes` 는 patch 불가라 메인이 정정 요망.
6. `14 closed[7]` 의 잔여 미탐색 2건이 stale(둘 다 같은 파일에서 이미 닫힘).
7. 도시에 §1 이 `10` 의 `vis=in:game_ai` 에 「오라클 직접 진입이 막힌다」를 붙이는데, 이 함수는 이미 5차부터 pub 래퍼로 전수 진리표가 나와 있다 — **이미 해결된 함수에도 경고가 그대로 붙어** 새 배치가 「막혔다」고 오해할 여지가 있다. `ev≥4` 처럼 해결분은 빼고 표시하면 좋겠다.
