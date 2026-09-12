# 12차 전수 감사 — 배치 C (담당 `10`~`14`) 보고서

> 게임 0.5.8 · 2026-09-11 · 산출물 = **`MIG\_verify12\C\patch.json`**(제출 완료) + 보조 스크립트 3개
> 착수·제출 직전 `dossierfresh.py 12 C` 두 번 실행 → **둘 다 `FRESH`**
> ⚠**`REPORT.md` 는 하네스가 파일 쓰기를 거부**해서 배치의 응답 전문을 **메인이 그대로 옮겨 적었다**(가공·요약 없음).

---

## 1. 정정 건수

| 분류 | 건수 | 내역 |
|---|---|---|
| **실오류** | **15** | 필드명 오귀속 6 · IR 줄 인용 오기 6 · 값/문면 3 |
| **보강** | **5** | 복제본과 규격 통일 2 · `logic` 구조 정정 1 · `chk=오귀속` 해소 1 · 복제본 간 `src_line` 차이 기록 1 |
| **행추가** | **4** | `11 mem` +2(BigPlan chats.ptr/len) · `14 mem` +2(Chat 힙 원소 tag/reason) |
| **합계** | **24** | `applypatch.py 12 --only C --dry` = **정정 24 성공 / 0 실패** |
| `behavior_change` | **1** | `13 mem[11]` `Entity+0x70 = ty payload = **&Tower**` — 포인터가 아니라 **인라인 값**이다. 이대로 재구현하면 `+0x70` 을 로드해 역참조하게 되어 **엉뚱한 주소를 읽는다** |
| `found_by` | new 19 / reused 5 | |
| `ev_up` | **0** | 잔여 `ev4` 는 ①`callees` 의 **vtable 간접호출**(심볼이 원래 없다 — §4-④) ②인라인된 다른 함수 줄을 가리키는 `knobs` 뿐이라 내릴 수 있는 행이 없었다. 판정 = **재료 부재**(범위: IR 호출 심볼·v2 `calls[]`·tcx leaf 검색 3종 시도. 미탐색 = `divtable` 슬롯→트레이트 메서드 자동 앵커) |

나머지 23건을 `bc=false` 로 본 근거: 줄번호·이름표기·문서 규격이라 값·분기·쓰기 대상이 안 바뀐다. `11 mem[22]/value` 의 `&새 SubPlan → 새 SubPlan` 은 소유권이 바뀌는 경계선이지만, 11차가 같은 문장을 `logic` 에서 이미 `bc=false` 로 고쳤으므로 그 판정을 승계했다.

---

## 2. G20 4건 — 각각의 판정

### ⓐ `Entity+0x128` → **`15`(`ty.Tower.info.ty`) 가 맞다. `13`·`14` 둘 다 틀렸다**

```
$ python -X utf8 tcxdict.py Entity 0x128
  ★0x128   ty@Tower.info.ty@tag   tag:1B  판별자 (Direct)
$ python -X utf8 tcxdict.py --enum EntityType
  판별자 enum+0x0(8B Direct) · 태그 2 = Tower · 페이로드 Tower — enum+0x8  info : game_core::Tower (192B)
$ python -X utf8 tcxdict.py Tower
  === game_core::Tower (192B · struct · 필드 9) ===   0xb8  ty : game_core::TowerType (1B)
```
산술이 닫힌다: `Entity.ty@0x68` + 페이로드 `0x8` + `Tower.ty@0xb8` = **`0x128`**.
`.info.` 는 실재하는 필드 단계(`EntityType::Tower(Tower)` 의 필드명이 `info`)다. `13` 은 뿌리(`ty.`)와 `.info.` 를 둘 다, `14` 는 `.info.` 를 빠뜨렸다 → **둘 다 `ty.Tower.info.ty` 로 정정**.

### ⓑ `Entity+0x70` → **`14`(인라인) 가 맞다. `13`(`&Tower` 포인터) 이 틀렸다**

```
$ python -X utf8 tcxdict.py Entity 0x70
  ★0x70   ty@Tower.info.state@tag   tag:8B  (외 11개 variant 동거)
```
`EntityType` 은 480B enum 이고 **페이로드가 enum+0x8 에 인라인**된다(Tower 192B). `Entity.ty@0x68` 이므로 `+0x70` 은 **Tower 값의 선두**다. 인라인이 아니면 `0x70+0xb8=0x128`(ⓐ)도 `0x70+0x18=0x88`(nearest_enemy)도 성립하지 않는다.
`13` 의 「`&Tower`」는 **소스 바인딩 타입**(`match &tower.ty { EntityType::Tower(info) => …` 의 `info: &Tower`)을 메모리 레이아웃 칸에 적은 것 — 소스로는 맞지만 `mem` 표는 메모리를 말하는 칸이다. 둘을 `ty.Tower.info` 로 통일. **이 1건만 `behavior_change: true`.**

### ⓒ·ⓓ `13` 의 `PlayerState+0x930`·`+0x9c0` `dir` 미기재 → **★오탐. 게이트(G20 R3)를 고쳐야 한다**

지시문은 「다른 14개/10개 명세가 `r` 이니 **기계적으로 확실한 누락**」이라 단정했지만, **`dir` 은 명세 사이에 공유되는 사실이 아니라 함수마다 다른 사실**이다.

- `13 target_bush_v30` 은 fastcc 인자승격이라 본문에 gep 가 **0건**: `awk 'NR>=11483&&NR<=11731' m10.ll | grep -c "i64 2352\|i64 2496"` → **0**
- 로드는 **호출부** `LineGankCoverPlan::sub_plan` 에 있다(11차 배치C 가 적은 줄번호 **8개 전부 정확**): `m10.ll:11762 gep 2352` → `11763 load` → `11764 icmp ult %9, 2` → `11768 panic_bounds_check` → `11773 gep 2496` → `11774 load i32` → `11787 gep 32` → `11791 tail call fastcc … target_bush_v30(i8 %24, i64 %9, i32 %14, ptr %16, ptr %26)`
- 복제본 `14 update` 는 본문에 gep 가 있어 `r` 이 맞다. **두 값이 다른 건 모순이 아니라 두 함수가 실제로 다르기 때문**이다.
- 이 프로젝트에서 `dir="-"` = 「이 함수 본문은 읽지도 쓰지도 않는다(참조용 행)」이고 `10 mem[12]/[15]`·`14 mem[16]/[26]/[27]` 도 같은 용법. **G14 는 `dir=-` 를 의도적으로 스킵**한다(`memdir.py` 판정식 표 「`dir=-` → 스킵」, 「부재는 결함이 아니다」).

⟹ 지시대로 `r` 을 채웠으면 **IR 에 없는 로드가 있다고 주장**하게 된다. **패치하지 않았다.** 고치는 법 = `sharedchk.check` 의 R3 블록(L162~L170) 삭제, 또는 「`-` 쪽 `note` 에 `참조용`/`인자승격`/`읽지 않는다` 가 있으면 억제」.

### ⓔ (지시에 없던 몫) `R2 AbstractGameWithCache.game` — **`12` 도 기여자였다**

게이트 머리줄이 `[03]` 으로 찍혀 「배치C 몫 4건」에 안 들어갔지만 기여자는 `03/04/06/07/12/18` 이고 그중 **`12 mem[6]` 이 `game`(0x8)** 이었다(형제는 이미 `game.vtable` — `10 mem[8]`·`11 mem[3]`). → `game.vtable (&dyn AbstractGame 팻포인터 뒤 절반)` 로 정정.

**사후 시뮬레이션**(정본 **사본**에 6건 모의 적용 후 `sharedchk.check`): **G20 8 → 6**. 남은 6 = `[03]/[16]` 의 Entity 0xb8/0xc0/0xc8 3건(배치A 몫) · `[03,04,06,07,18,18]` R2 1건 · **내가 오탐 판정한 R3 2건**. ⟹ **내 담당 R1 2건은 완전히 닫힌다.**

---

## 3. 11차가 고친 자리가 다시 틀렸는가

| 11차 정정 | 지금 상태 | 판정 |
|---|---|---|
| `10 logic` — `is_recent_visible` **즉시통과 2단** | **살아 있다**(logic + `knobs[4].effect`). IR 재확인: `g07.ll:157015` vtable+0xf8 `is_visible` → 157017 호출 → **즉시 true** · `157021` vtable+0x150 `get_player_by_champion_id` → null 이면 **즉시 false** · `157036` Blackboard+0x1e0 → `add 120` → `157040` vtable+0x28 `tick` | ✅ |
| `13 mem` — 입력 3칸 | **정본에 실제로 들어가 있다**(v2 `reads[0..2]` 직접 확인). 인용 IR 줄 8개 전부 정확 | ✅ |
| `12 closed[1].why` — 45칸 → **297칸 전수 진리표** | 바뀌어 있다 | ✅ |
| `11 logic` — `&mut self.plan`/`&self.team_plan`/`merge(.., sp)` | 3곳 전부 유지 | ✅ |
| `12 knobs[3]` — `knobs[7]`→`[8]` | 두 곳 다 `[8]`. 내부 인덱스 참조 25종 전수 → **범위밖 0** | ✅ |

### ★그러나 — **정정이 옮겨 간 자리가 있다**

1. **`11 mem[22]/value` 가 `&새 SubPlan` 인 채로 남았다.** 11차는 `logic` 의 같은 문장만 고치고 `mem` 표의 `value` 칸은 안 고쳤다 — 같은 사실이 두 칸에 있고 한 칸만 고쳐진 전형. → 이번에 정정.
2. **`closelist` 의 「needle 공유」 버그는 안 고쳐졌다.** 11차가 지적한 두 형태 중 **중복 키 쪽만** 고쳐졌다:
   - ✅ `(12, "chat_allowed(GameContext")` → 297칸 쪽이 이긴다.
   - ❌ `(11, "passive_plan")` 하나가 **`11 closed[4]/[5]/[6]` 세 물음에 같은 why**(「뒤 8B 는 별도 반환값」)를 그대로 달고 있다. `[5]` 의 진짜 답은 `history[5]`, `[6]` 은 `history[4]/[6]`.
   - ❌ `(14, "_lead")` 가 **`14 closed[7]`**(「`_version` 미사용 일반화 금지」)에 엉뚱한 why(`*_lead` 산출식)를 달고 있다.
   - ⚠`closed` 는 v2 에 없는 파생 필드(`_spec/closelist.py`)라 **`applypatch` 경로로 주소지정이 안 된다** → 산문 보고만 가능. 고칠 법 = 「복수 매치 시 **가장 긴 needle**」 + `(i,needle)` 유일키.
3. **`10` 의 「완료 보고 ≠ 정본 반영」 재발 여부**: `specs20.json` 을 직접 열어 확인 — `10 knobs[6]`(level 게이트)·`knobs[7]`(objective→id 표)·`consts[5]`(19600000000) 전부 **실재**한다. 이번 내 patch 도 `--dry` 의 삽입 자가검증(「★삽입이 정본에 반영되지 않았다」 미발생)까지 통과시켰다.

---

## 4. 도구 결함 · 내 지시 오류 (`brief_errors` 7건)

1. **★G20 R3 은 축 자체가 잘못 놓였다** — `dir` 은 cross-spec 사실이 아니다(§2 ⓒⓓ). **지시문이 이걸 「기계적으로 확실한 누락」으로 단정한 것이 지시 오류다.**
2. **★G20 R1 의 억제 규칙 `prefix_of` 가 같은 계열 결함을 절반만 잡는다.** `Entity+0x128`(2성분 vs 4성분)은 발화했는데 **같은 함수의 `Entity+0x88`**(`Tower.nearest_enemy` 2 vs `ty.Tower.nearest_enemy` 3)은 `pa[-2:]==pb[-2:]` 로 「정밀도 차이」 억제돼 **조용히 통과**했다 — 둘 다 `.info.` 누락이라는 똑같은 결함(tcx: `ty@Tower.info.nearest_enemy@tag`). ⟹ **사전이 있으니 「어느 쪽이 틀렸는지」까지 말할 수 있다** — `tcxdict <base> <offset>` 과 대조해 「어느 쪽도 정본과 불일치면 둘 다 결함」으로. 이번 4건 전부 그렇게 갈렸다.
3. **`consts.kind` 확장 어휘가 반쪽만 적용됐다.** 지시가 확인하라 한 `11 c17`(-8)은 **맞다**(`m13.ll:12415 %66 = add nsw i8 %65, -8` = `add` 피연산자 ⟹ `오프셋가감`). 그런데 **완전히 같은 역할의 `c15`(-7)는 `임계` 로 남아 있다**(`m13.ll:12382 %53 = add nsw i8 %52, -7`). `-6` 과 Jungle 의 `-7` 은 `icmp` 경계라 `임계` 가 맞다 ⟹ **`-7` 이 두 역할을 겸하는데 칸이 하나다**. 판정 = **표기 불가**(범위: `kind` 가 `meaning` 낱말에서 파생되는 현행 설계. 미탐색 = `kind` 다중값화). 어휘를 넓혔으면 **기존 행 문면도 같이 훑어야** 같은 표 안에서 같은 게 다르게 분류되지 않는다.
4. **`callees` 의 `ev4` 는 「후보 중 못 골랐다」가 아니라 「간접호출이라 심볼이 원래 없다」인 경우가 많은데 표기가 구분 못 한다.** 지시가 지목한 `10 is_visible(7)`·`tick(7)`·`get_player_by_champion_id(5)`, `11 get_game_mode(5)`·`tick(7)` 은 **전부 `&dyn AbstractGame` vtable 간접호출** — `_rank_callees` 의 `_anchor` 가 **영원히 false** 다. 슬롯으로는 확정된다: `g07.ll:157015` +0xf8 → `AbstractGame::is_visible`(호출 157017) · `157021` +0x150 → `…get_player_by_champion_id`(157023) · `157040` +0x28 → `…tick`(157042) · `m13.ll:12259` +0x40 → `…get_game_mode`. ⟹ **정답은 언제나 트레이트 메서드 `game_core::AbstractGame::<method>`** 이고 `World::is_visible`·`VisibleState::is_visible`·`CCState::tick`·`PlayerAiContext::tick` 은 **잡음**이다. 지금은 `_anchor` 전부 false 일 때 **경로 길이 순** 상위 3개를 실어서 `World::is_visible`(28자)이 `AbstractGame::is_visible`(33자)을 **이긴다** — 임의 선택. 고칠 법 = ①`vtable+0xNN` 을 `divtable` 로 풀어 `_anchor` 를 세운다 ②길이 정렬을 버리고 전 후보를 싣는다.
   ✅한편 **직접호출 쪽에서는 11차 후속 수정이 실제로 작동했다** — `11` 의 `BigPlan::goal`·`SubPlan::merge`·`BigPlan::sub_plan`·`passive_plan` 이 `ev3`(IR 호출 심볼 일치)로 앵커돼 있다.
5. **★IR 줄 인용(`mNN.ll:NNNN`)을 보는 게이트가 없다 — 내 담당 5함수에서 6건이 틀렸다.** G13 은 `knobs[].where` 의 **소스 줄**만 본다. 검사기는 40줄이면 되고, 「그 줄이 `#dbg_value`/빈 줄/블록 라벨이 아닌가」만 봐도 6건 중 5건이 잡힌다(참조구현 = `_verify12/C/citechk.py`).
6. **도시에 §1 표의 `ev≥4(미실행)` 이 `knobs` 만 센다.** `10` 은 `1` 로 적혀 있지만 `callees` 16행 중 **12행이 `ev4`**, `sig.params` 도 4행이 `ev4` 다.
7. **§6 이 `_verify12/C/` 를 「출력 전용」이라 적는데 실제로는 분책 `spec_*.md` 5개가 같은 폴더에 있다.** 덮어쓸 위험을 한 번 확인해야 했다(파일명이 달라 실제 충돌은 없었다).

---

## 5. 전수 대조 — **불일치 0 인 축도 결과다**

| 축 | 대조 대상 | 방법 | 결과 |
|---|---|---|---|
| `mem` 오프셋↔필드명 | **117행**(10:22·11:23·12:26·13:18·14:28) | `tcxdict.pick()`+`walk(want=off)` 로 **행별 재조회**(`chk` 컬럼 불신). 조회 가능 **110행**(나머지 7 = `TraceEventType::CallHandled` variant 경로) | **실오류 6 · 표기정정 1 · `chk` 오귀속 1** · 나머지 일치. 매처가 든 41개 후보 중 **33개는 오탐**(첨자 `[team]`↔`[0]` · std 내부 경로 `buf.inner.*` · vtable 슬롯) |
| `mem` **역방향**(IR→명세) | 5함수 IR 범위 전량의 gep 상수 오프셋 | `irscan.py` | **미등재 5종** 중 진짜 누락은 **1종**(Chat 힙 원소 `+0x1`) + Vec 워드 2종 → **행추가 4로 해소**. 나머지 2종(`+0x18/+0x20` = player_champion 4·5번 슬롯)은 `SPEC_GUIDE §3`(stride 는 안 적는다) 규칙대로 정상 |
| `consts` **역방향** | 5함수 IR 범위의 산술·비교 리터럴 전량 | 같은 스크립트 | **누락 0.** 걸린 2개는 규칙상 제외 — `10` 의 `2`(배열 길이) · `14` 의 `5`(`shl … 5` = stride 32 의 log2) |
| `consts.src_line` | 표본 5건 | `dloc.py` 로 inlinedAt 루트까지 전개 | **전건 일치**. `10 c5`=1190(`!53163`) · `10 c4`=1233(`!55515`, 루트 1230) · `13` 의 `4`(`!23180` = tower.rs:100→is_tower2 1309→cover.rs:150) · `14 c1`=47(`m08.ll:94618 icmp ult %32, 41`) · `10` 마스크(`49632 and i24 %4, 65534` / `49633 icmp eq i24 %8, 768`, **리터럴 3 부재 = `notes[0]` 확인**) |
| `callers` | 5함수 전량 | `_gaibc/m*.ll` 24개 심볼+`call`/`invoke` 전수 grep | **불일치 0.** 초과 5건은 전부 ThinLTO 요약 인덱스(`^N = gv: … calls:`) — 명세 목록이 정확하다 |
| 명세 **내부 인덱스 참조** | 5함수 전문 | 정규식 스캔 + 배열 길이 대조 | **25종 · 범위밖 0** |
| `notes` | 6행 | IR 원문 반증 시도 | **반증 0** |
| `one_line`·`layer` | 5행 | `logic`·`sig`·경로 대조 | **불일치 0** (`13` 의 「부시 인덱스(2~21)」는 실제 반환집합 최소~최대라 타당) |
| `closed[].why` | 37행 | 물음↔근거 의미 대조 | **오귀속 3건**(§3-2, 파생 필드라 patch 불가) |
| IR 줄 인용 | 5함수 전문의 `[mgv]NN.ll:NNNN` 약 120개소 | `citechk.py` 로 그 줄의 IR 원문 전량 출력·대조 | **오기 6건**(§4-5) |

### 새로 확정한 구조 사실

- **`10` 의 적 챔피언 주사는 `for` 루프가 아니라 `iter_champions(e_team).any(|enemy| …)` 이고 반환은 `!any(..)`** 다. 근거 = `dloc m10.ll 55515` 사슬: `1233 closure$1 → 2893 iterator.rs → 50 filter_map.rs(…iter_champions::closure_env$0, Iterator::any::check…) → 2494 → 138 → 2897 any<FilterMap<slice::Iter<Option<&Entity>>, iter_champions::closure_env$0>,…> → **1230 should_end_object_finish_kill_priority_battle**`. `!55306`(blackboard gep)=1230 · `!55339`(e_team)=1229 · `!55489`(is_recent_visible)=1231 ⟹ **1230 한 줄 = `blackboard[e_team]` + `iter_champions(..).any(`**. 부수로 `iter_champions`(`AbstractGameWithCache::iter_champions`, simulation.rs:1904, pub/mir=1)가 **`callees` 에서 통째로 빠져 있었다** — `logic` 산문이 배열 직접순회로 적혀 있어 `harvest_callees` 가 못 긁었다. `logic` 정정으로 다음 빌드부터 잡힌다.
- **`Entity+0x5c0` 의 필드명은 `champion_id` 가 아니라 `id`** 다(`tcxdict Entity`). 「champion_id」는 받는 쪽 인자 이름이다.
- **`is_in_range_ex` 는 `g06.ll:51807~51942`** 이고 `25000`(`range_margin`)의 선형 가산 지점은 **`51928 %73 = add i64 %10, %7`**, 최종 판정은 `51939 mul` → `51940 icmp ule %81, %82`(dx²+dy² ≤ total²). `10 history[0]` 이 인용한 `51807~51890` 은 **그 근거를 범위 밖에 두고 있었다.**
  ⟹ 지시문이 의심한 「`10 c4`(25000)가 콜리 안에서 임계일 수 있다」는 **아니다 — `오프셋가감` 이 맞다.** 임계인 것은 `c5`(`m10.ll:47496 icmp ugt i64 %22, 19600000000`)이고 그 `kind` 변경도 **맞다**.
- **`BigPlan` 의 `SinglePlanLine.chats` 3워드**: `tcxdict BigPlan 0x8/0x10/0x18` = `cap`/`ptr(NonNull<u8>)`/`len`. IR `m13.ll:12281~12291` 이 `+8=0`, `+16=inttoptr(8)`, `+24=0`, `+0=4(tag)`, `+32=0`, `+33=1` 로 찍는다. `inttoptr(8)` = `Chat`(24B align 8) dangling ⟹ **할당 0**.

---

## 6. 실제로 실행한 것

```bash
cd /c/tfm2mods/MIG
python -X utf8 dossierfresh.py 12 C                    # 착수/제출 직전 2회 — 둘 다 FRESH
PYTHONIOENCODING=utf-8 python -X utf8 specgate.py      # G20 8건 중 내 몫 4건 확인
python -X utf8 tcxdict.py Entity 0x128 | Entity 0x70 | Entity | Tower | BigPlan 0x8|0x10|0x18
python -X utf8 tcxdict.py --enum EntityType
python -X utf8 tcxq.py grep game_core iter_champions
python -X utf8 dloc.py m10.ll 53163 53157 55515 55306 55339 55489 23139 23095 23118 23180 23190
python -X utf8 dloc.py m13.ll 38511 38480
python -X utf8 _verify12/C/irscan.py    # ★IR→명세 역방향(gep·리터럴)
python -X utf8 _verify12/C/memtcx.py    # mem 117행 tcx 독립 재조회
python -X utf8 _verify12/C/citechk.py   # ★IR 줄 인용 전수 대조(신규 — 다음 라운드 게이트 후보)
PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 12 --only C --dry   # 24/24 성공 · 0 실패
# + sharedchk.check 를 정본 **사본**에 모의 적용해 G20 8→6 확인(정본 미변경)
# + awk/sed/grep 로 m10/m13/m08/g06/g07.ll 원문 직접 확인(줄번호는 본문에 전부 인용)
```
산출물 = `_verify12/C/` 의 `patch.json` · `irscan.py` · `memtcx.py` · `citechk.py`.
**`_spec/specs20*.json` · `distruct.json` · `dienum.json` 은 한 번도 쓰지 않았다**(읽기만).

---

## 7. 판정 어휘

| 항목 | 어휘 | 범위·전제 |
|---|---|---|
| `10 c4`(25000) = `오프셋가감` | **사실 서술** | IR 확정 — 콜러는 `%2` 를 `is_in_range_ex` 8번째 인자로 그대로 넘기고(`m10.ll:47527` 외 3곳), 콜리는 `g06.ll:51928 add i64 %10, %7` 로 **합산**한다. 임계로 쓰이는 자리 없음 |
| `11 c15`(-7)의 `kind` | **표기 불가** | **현행 `kind` 설계(단일 문자열, `meaning` 낱말 파생)로는** 불가. Epic 에서 `add` 피연산자(오프셋가감)·Jungle 에서 `icmp` 경계(임계) **둘 다**다. 미탐색 = `kind` 다중값화 |
| `callees` 의 vtable 간접호출 5계열 | **재료 부재** | 시도: ①IR 호출 심볼 — 간접호출이라 **원리적으로 없다** ②v2 `calls[]` — IR 스캔 산물이라 역시 없다 ③tcx leaf 검색 — 동명 후보만. **전제** = 「IR 심볼로 앵커한다」는 현행 `_rank_callees` 설계. 미탐색 = `divtable` 슬롯→트레이트 메서드 자동 앵커(슬롯 번호는 이미 `mem` 표에 있다) |
| `closed[].why` 오귀속 3건 | **사실 서술** + **표기 불가(제출 경로)** | 오귀속 자체는 확정. `closed` 는 v2 에 없어 `applypatch` 계약이 **주소지정을 못 한다** — 고칠 곳은 `_spec/closelist.py` |
| G20 R3 2건 | **오탐** | 게이트 결함. `dir` 은 cross-spec 사실이 아니다 |
| `13` vs `14` 의 같은 리터럴 `2` 에 다른 `src_line`(138 vs 261) | **표기 불가** | `icmp eq i64 %38, 2`(m10.ll:11569)의 `!dbg !23139` 가 **`line 0`(병합 위치)** — 어느 역할의 줄을 대표로 삼느냐의 문제이고 둘 다 자기 역할로는 옳다(138 = Top 부시 ID · 261 = `match line` switch). **어떤 게이트도 「복제본끼리의 `src_line` 일관성」을 안 본다 → 다음 라운드 게이트 후보.** 이번엔 `13 consts[0]` 에 사실만 기록 |
