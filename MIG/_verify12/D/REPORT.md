# 12차 전수 감사 — 배치 D (함수 `15`~`19`) 보고서

> 게임 0.5.8 · 2026-09-11 · 도시에 도장 `2026-09-11 23:46:22` · `dossierfresh 12 D` = **FRESH**(착수·제출 직전 2회)
> 기계 적용분 = `_verify12/D/patch.json` — `--dry` 결과 **정정 8/8 · ev상향 10/10 · 실패 0 · 동작변경 0**
> ⚠하네스가 배치의 파일 쓰기를 막아 **메인이 반환 원문을 그대로 옮겨 적었다**(가공·요약 없음).

## 0. 실제로 실행한 것 (명령줄 그대로)

```bash
cd /c/tfm2mods/MIG
python -X utf8 dossierfresh.py 12 D                      # 2회, 둘 다 FRESH
PYTHONIOENCODING=utf-8 python -X utf8 sharedchk.py       # G20 전역 8건 + 억제 9건

sed -n '33374,33739p' /c/tfm2mods/_gaibc/m13.ll > _verify12/D/work/f15.ll   # 16~19 동일
# 16: m10.ll 51913~52374 / 17: m05.ll 17695~17825 / 18: m09.ll 6879~7264 / 19: m04.ll 62570~62978

# ★IR→명세 방향 — 직접 만든 도구 (전부 _verify12/D/work/)
python -X utf8 work/irsy.py    # 5함수 call/invoke 심볼 + 간접호출 전수
python -X utf8 work/geps.py    # gep 사슬 → (root, offset) 누적 집계
python -X utf8 work/anch.py    # `_rank_callees` 앵커를 길이접두 경계로 재판정
python -X utf8 work/ce.py      # callees pick/ev 전역 통계
python -X utf8 work/lk.py      # spec3lib.fnlookup leaf 후보 전량
python -X utf8 work/r5probe.py # G20 추가규칙 후보 R5/R6 전역 시험
python -X utf8 work/r7probe.py # 〃 R7(판정 전파 누락)

python -X utf8 dloc.py m04.ll 72085 72082 71892 72192 72211 71630
python -X utf8 tcxdict.py --enum MainObjective ; --enum Chat ; --enum JungleType
python -X utf8 -c "…" _gcbc/g15.ll 130480          # G16 인용처 실물

sed -n '27343p;27390p;29114p;29219p;34365p;34375p' _gaibc/m05.ll             # 15 knobs 5~8
sed -n '82163p;87035,87042p;66660,66676p;79434p'   _gcbc/g06.ll              # 16 knobs 5~8
sed -n '62483p;64612,64660p' _gaibc/m09.ll ; sed -n '176782p;5094p' _gcbc/g15.ll   # 18
sed -n '62501,62505p;62526,62568p' _gaibc/m04.ll ; sed -n '59281,59282p' _gcbc/g09.ll  # 19
grep -rn "24max_range_nearly_can_use(ptr" _gaibc/*.ll                        # 16 호출부 13곳 + tick 인자
diff _verify11/D/spec_1[5-9]*.md _verify12/D/spec_1[5-9]*.md                 # consts.kind 변화 확인

PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 12 --only D --dry
```

오라클은 **쓰지 않았다.** 이번 표적 셋(`consts.kind` · `callees` 앵커 · G20)은 전부 문면·IR·tcx 문제라 실행으로 갈리지 않는다(`METHOD_MAP ⑥` 한계 3). `18`(`in:game_ai`)은 오라클 직접 진입이 막히는 함수이기도 하다.

---

## 1. 정정 건수

| 분류 | 건수 | 내역 |
|---|---|---|
| **실오류** | **1** | `17 mem` 지역 `Vec<Chat>` 의 **cap(+0x0) 행 결손** (= 행 추가 1건) |
| **오탐** | **1** | `18 sig.params[2]` G16 (§2) |
| **보강** | **6** | `18 mem[3]`·`mem[4]` 이름 통일(G20 R2) · `18 mem[16]` 세 번째 쓰기 사이트 · `18 knobs[13]` 구간 · `19 knobs[6]` 줄 · `19 one_line` 출구 3개 |
| **행 추가** | **1** | `op:insert /specs[17]/mem at=23` → v2 `writes[16]` |
| **ev 상향** | **10** | `mem` ev4→ev3 (tcx `--enum Chat`/`MainObjective`/`JungleType` 실행 근거) |
| **`behavior_change`** | **0** | ↓ |

### `behavior_change` 가 0 인 이유 — 한 건씩 따로 판정했다

7차에 이 배치가 전건 `false` 로 내서 지적받았으므로 이번엔 개별로 따졌다.

* `17` cap 행 결손 → **false**. 11차가 **같은 `Vec` 의 `ptr`·`len` 행**을 추가하며 `false` 로 판정한 선례를 따른다. 재구현이 Rust `Vec<Chat>` 을 그대로 쓰면 cap 은 자동으로 맞고, **24B 를 손으로 조립할 때만** 어긋난다.
* `18 mem[16]` → **false**. note 가 진술하는 *규칙*(「`load(+0xd0)` → `add 1` → 되쓴다」)은 경로와 무관하게 옳다. 틀린 것은 **사이트 열거**뿐이고, 같은 표 `mem[15]` 가 Press/PressChange 태그 쓰기를 이미 싣고 있어 재구현자가 그 경로의 push 를 빠뜨릴 수 없다.
* `19 one_line` → **false**. `logic`·`knobs[2]`·`knobs[4]` 가 세 출구를 다 적고, 명세 자신이 「재구현 정본은 `mem`/`consts`/`callees`/`sig.tcx`」라고 못 박는다.

⟹ **0 은 이번 라운드의 결과다.** 11차 결론 1(「IR 값이 틀렸나는 사실상 닫혔다」)의 예측과 일치한다. 값 축에서 새로 나온 실오류는 **행 결손 1건**뿐이다.

---

## 2. G16 1건의 판정 — **오탐**

**지적**: `18 sig.params[2] P4 role` 이 주장한 `readnone` 이 `define` 에 없다.

```
# ① 18 자신의 define (m09.ll:6879) — readnone 이 **없는 것이 맞다**
define internal fastcc noundef zeroext i1 @…TeamPlan22v3_epicops_buff_window(
  ptr noalias noundef nonnull align 8 dereferenceable(1064) %0, i64 noundef range(i64 2, 0) %1,
  ptr noalias noundef nonnull align 16 dereferenceable(320) %2, …)

# ② 명세가 **인용한 곳** (_gcbc/g15.ll:130480) — 여기엔 있다
define void @…game_core…PlayerState8strategy(
  ptr … sret([24 x i8]) … %0, ptr noalias noundef readonly … dereferenceable(2528) %1,
  ptr noalias noundef readnone align 16 captures(none) dereferenceable(320) %2, …)
```

명세 문면은 「**본체** `_gcbc/g15.ll:130480` 의 `rnd` 인자에 `readnone` 이 붙어 있어」이고 `_gcbc` = **game_core 크레이트**다. 즉 **콜리의 `define` 인용**이지 18 자신의 속성 주장이 아니다.

**사실 주장 자체도 참**이다: 18 범위(6879~7264)에서 `%2` 출현은 `define` 줄 · `#dbg_value`(6886) · **6964 의 `PlayerState::strategy` 호출 인자** 딱 3회 ⟹ RNG 무전진. `closed[5]`·`history[5]` 와 정합.

### 게이트를 어떻게 고치면 되는가 (`paramrole.py`)

1. **P4 에 외부 IR 인용 회피 한 줄.** 속성어가 든 절(`CLAUSE.split` 조각)에 `sp["ir"]["file"]` 이 아닌 `[gm]\d+\.ll` 인용이나 `_gcbc`/`_gvbc` 경로가 있으면 **그 절의 속성 주장은 콜리의 것**이므로 건너뛴다. 기존 `META` 필터와 같은 자리에 조건 하나면 된다.
2. **덤 — `CITE` 정규식이 `m\d+\.ll` 만 받는다.** 그래서 `_gcbc`(`gNN.ll`) 인용은 **P6 이 검증조차 못 한다**(내 담당 `16 knobs[5..8]`·`18 knobs[19..21]` 이 전부 그 형태 — 이번에 **손으로** 7/7 일치 확인). `([gm]\d+\.ll)` 로 넓히고 접두로 디렉터리를 고르면 cross-crate 인용도 기계 검증된다.

`patch.json` 은 `kind:"오탐"` 으로 내되, **귀속을 기계가 읽도록** role 문면에 「피호출자 `PlayerState::strategy`(game_core 크레이트, 이 명세의 `m09.ll` 이 아니다)」를 명시하는 편집을 함께 냈다.

---

## 3. ★`sharedchk.py`(G20) 리뷰 — 내가 제안한 게이트의 구현 결과

**총평: 설계는 옳다. 구현은 「억제가 모자란 곳 1 · 문면이 과장된 곳 1 · 존재하지도 않는 억제 1」이다.** 49→8 로 줄인 것은 억제가 제대로 작동했다는 뜻이고, 남은 8건 중 5건이 실제로 뭔가를 말한다(내 담당 4건).

### 3-1. 억제 4종 개별 판정

| 억제 | 판정 | 근거 |
|---|---|---|
| **중첩**(enum variant) | ✅**정확. 과하지 않다** | `tcxdict --enum MainObjective` 실행: 3B enum, 판별자 enum+0x0, **`페이로드 Morgard` = `enum+0x1 phase` · `페이로드 Serpen` = `enum+0x1 phase`** — 글자 그대로 같은 자리. 태그가 `+0x41f` 이므로 `08`(Morgard)·`18`(Serpen) **둘 다 맞다**. `Chat+0x8`·`SubPlan+0x8`·`Entity+0x88` 동형 |
| **정밀도** | ✅타당 | `Entity+0x0` `team` vs `team.tag` 류 |
| **표기**(`_ptr`·`[]`) | ✅타당 | `game`/`game.data`/`game.data_ptr` |
| **`r` vs `w`** | ⚠**그런 억제는 존재하지 않는다** (§3-4) |

### 3-2. ★억제가 **모자란** 곳 — 튜플 variant 첨자 `.0` (R1 오탐 3건)

남은 R1 5건 중 **3건**(`Entity+0xb8`·`+0xc0`·`+0xc8`)이 전부 이 형태다.

```
[16] ty.Champion.0.skill_cooldown / ty.Minion.info.attack_cooldown
[03] ty.Champion.skill_cooldown
```

`16` 이 맞다 — 같은 행 근거가 **`tcxdict --enum EntityType`(필드명 = `0`) + rustc `E0164`(`Champion(ch)` 통과 / `Tower(i)` 반려)**. `03` 은 첨자를 생략했을 뿐 **모순이 아니다**. 그런데 `variant_split` 은 갈리는 성분(`0` vs `skill_cooldown`)이 대문자가 아니라 False, `prefix_of` 는 `pa[:3] != pb[:3]` 이라 False, `skel` 은 `_ptr|_id|[]` 만 벗기므로 False → 발화.

> **제안**: `skel()` 에 **숫자 단독 성분 제거** 한 줄 — `".".join(s for s in n.split(".") if not s.isdigit())`. 그러면 `spelling_of` 가 `R1표기` 로 억제한다. **R1 5건 → 2건.**
> (`03` 쪽은 `.0` 을 넣는 편이 정보량이 크다. 내 배치가 아니라 손대지 않았다.)

### 3-3. R2 — 억제가 과한 게 아니라 **사유 문면이 거짓**이다

```
[R2] AbstractGameWithCache game — "표기로 설명되지 않는다 — 반드시 한쪽이 틀렸다"
   [18] 0x0 game(데이터 포인터) / [18] 0x8 game(vtable 포인터) / [12] 0x8 game(vtable 절반)
```

**「반드시 한쪽이 틀렸다」는 이 건에서 거짓이다.** `18`·`12` 는 괄호로 두 절반을 정확히 갈랐고 둘 다 맞다. 발화 원인은 `norm_name` 이 **`(` 부터 통째로 버리는 것**(주석 제거 규칙)인데 여기선 **괄호가 유일한 판별자**다.

다만 결과적으로 쓸모는 있었다: 같은 사실을 `15 mem[1]/[2]`·`17 mem[3]/[4]`·`19 mem[4]/[5]` 는 `game.data_ptr`/`game.vtable_ptr` 로 적는다 — **한 팻포인터를 부르는 이름이 20명세 안에서 5가지**였다. `18` 을 그 표기로 통일하는 정정 2건을 냈다.

> **제안 2가지**
> 1. R2 사유를 「반드시 한쪽이 틀렸다」 → 「**같은 이름이 두 자리에 있다 — 값이 틀렸거나 이름이 판별력을 잃었다**」로. 지금 문면은 정정자를 「값을 고쳐라」로 오도한다(내가 실제로 거기서 출발했다).
> 2. R2 키에만 **괄호를 살리는** 정규화를 쓴다(`norm_name` 은 R1 에 필요하니 그대로). 그러면 이 건은 `12`↔`18` 의 명명 규약 불일치로 `soft: R2표기` 에 떨어진다.

### 3-4. 지시 ⓑ 의 「`15` w ↔ `17` r 을 G20 이 정상 판정했다」 — **검사 자체가 없다**

`R3` 은 **한쪽 `dir` 이 `-`(미기재)일 때만** 발화한다(`check` 의 `if u"-" in ds and set(ds) - {u"-"}`). `w` vs `r` 은 애초에 비교 대상이 아니다. **사실 판정은 「둘 다 맞다」** — `15` 는 호출 직전 스택에 `BattlePlanGoal::TryKill` 을 **조립**(w)하고 `17` 은 by-value 로 받은 그 값을 **읽는다**(r). 방향은 함수마다 다른 것이 정상이고, **cross-spec 에서 `dir` 을 비교하는 규칙은 넣으면 안 된다**(넣으면 이 쌍 전부 오탐). R3 의 현재 설계가 옳다.

### 3-5. R1~R4 말고 더 넣을 규칙 — **제안만 하지 않고 20명세 전역에 돌려 봤다**

| 후보 | 무엇 | 실측 |
|---|---|---|
| **R5 타입 크기** | `Ty(NNNB)` 표기가 명세마다 다르면 한쪽이 틀렸다(`PlayerState(2528B)`·`StdRng(320B)`·`TeamPlan(1064B)`) | **0건** — 축이 이미 깨끗하다 |
| **R6 열거형 태그값** | `Ty::Variant` ↔ `consts.value` 짝 대조 | **0건**. ⚠단 **`kind=태그` 행으로 한정**해야 한다 — 무제한이면 `12 consts[9]`(값 `-8`, `kind=계수`, 설명이 `TutorialType::Total(8)` 을 언급)가 오탐으로 뜬다 |
| **R7 판정 전파 누락** | 같은 (base,offset) 중 **한쪽만** 「소비처 0건/죽은 슬롯/노브 아님」 확정을 달면 나머지는 안 따라간 것 | **0건** — 11차가 지적한 전파 문제(`17 knobs[1]` vs `15 knobs[4]`)는 실제로 닫혔다 |

⟹ **넣을 값이 있는 새 규칙은 없다.** R5·R6 은 상수 비용이라 회귀 방지로 붙일 만하지만 **지금 수확은 0** 이라고 정직하게 적는다. 실제 수익은 §3-2 의 `skel()` 한 줄이다.

### 3-6. R1 이 이미 공짜로 덮고 있는 것

`AbstractGame::vtable` 을 base 로 쓰는 행(`15` +0x1f0 `get_entity_by_id` · `17` +0x28 `tick` · `19` +0x40 `get_game_mode` · `12`…)은 **R1 이 자동 cross-spec 대조**한다. vtable 슬롯은 `divtable` 말고 검증 수단이 없는 축인데 덮였다. 이번에 불일치 0.

---

## 4. ★11차가 고친 자리가 다시 틀렸는지 — **재정정 0건**

11차 `applied.json` 의 13개 도장 자리를 전부 IR 로 다시 쟀다.

| 11차 정정 | 12차 재확인 | 판정 |
|---|---|---|
| `17` Chat 페이로드 3행 + Vec ptr/len 2행 **추가** | `mem[5]·[20]·[21]·[22]·[23]` 존재. IR: 17744 `%19 = load ptr, ptr %7` / 17745 `store i8 3` / 17746~7 `%20 = gep %19,8`→`store i64 %13` / 17748~9 `%21 = gep %19,16`→`store i64 0` / 17750 `store i64 1, ptr %8` — **5/5 일치** | ✅**빈 Chat 문제 해소 확인** |
| `18 sig.params[2].role` readnone | G16 재지적 = **오탐**(§2). 인용처 실물 확인 | ✅유지 |
| `18 mem[16].note` | 규칙은 옳고 **사이트가 2/3** — 7218~7219(Press 경로) 누락 | ⚠**부분 잔여** → 보강 |
| `18 mem[21].note`(`__1` 과일반화 경고) | `tcxdict --enum Chat`: `Press/PressChange` = `enum+0x8 "1" : usize` · `Repair/SerpenSetup` = `enum+0x8 "0" : usize` — **경고가 tcx 로 확증** | ✅유지 +ev3 |
| `18 logic` 658 | `!14843` 사슬 그대로 | ✅유지 |
| `19 consts[4].meaning` 821/836 + 829 | `dloc 71892`→`option.rs:1011`←**821** · `72192`←**836** · `72211`←**829**. **3/3** | ✅유지 |
| `19 logic` `is_empty` | `dloc 71630`→`len@vec.rs:1617`←`is_empty@vec.rs:1636`←**817** | ✅유지 |
| `19 logic` 818/819/821 호이스트 | `!72085`/`!72082` 사슬 = `min_by_key`←**833**(본선), 폴백은 821 | ✅유지 |
| `19 mem[15]` 원소 역참조 **추가** | 62852 `%100 = load i8, ptr %91, !range !41907` · 62850 `%99 = gep %91, i64 1`. `--enum JungleType` = **1B**, variant 6 ⟹ `!41907={i8 0,i8 6}` 정합 | ✅유지 +ev3 |
| `16 knobs[3].where` 1775/1790/1805 · `logic` 1748/1775 | 같은 표 `knobs[5..8]` 은 `_gcbc` 인용이라 G13 이 못 본다 — **손으로 7/7 확인** | ✅유지 |

**⟹ 재정정 0. 11차의 12건은 전부 살아 있다.** 유일한 잔여는 `18 mem[16]` 의 **열거 불완전**이고, 이건 「정정이 오류를 옮겼다」가 아니라 「새 문면이 처음부터 2/3만 적었다」이다. 형태는 같다 — **정정할 때 그 축을 전수로 세지 않으면 새 문면이 새 결손을 만든다.**

11차 §4-2 의 `closelist.closed_reason` 그림자(`19 closed[5]`)는 **고쳐져 있다**(현재 근거 = 「4차 배치A/C: 항등 사상 확인 기록(확정 서술)」). §4-1·§4-3·§4-4 는 **미반영**.

---

## 5. ★이번 라운드 최대 수확 — `callees` 축 (도시에 ⓑ 의 답)

도시에는 「262행 중 56행만 `ev3`」이라 했다. **그 수치가 틀렸고, `ev4` 의 뜻도 한 가지가 아니다.**

### 5-1. `ev3` 도장 중 **3건이 거짓 앵커** — 내 담당 1건

`_rank_callees` 의 앵커는 `all(s in sym for s in path.split("::"))` 인 **날 부분문자열**이다. v0 망글링은 `<길이><이름>` 이라 **경계를 세야** 한다.

```
16 callees[5]  game_core::Effect::range   ← ev3 "IR 호출 심볼 일치"
   16 범위(m10.ll 51913~52374)의 call 심볼은 **딱 2개**:
     51944  …6effect4typeNtB4_13CastingTarget5check
     52030  …NtB2_6Effect12range_adjust    ← 'game_core'✓ 'Effect'✓ 'range'✓ (range_adjust 안!)
   ⟹ `Effect::range` 는 **호출되지 않았는데** 다른 심볼의 부분문자열로 도장을 받았다.
```

전역 3건 = `00 callees[20]`·`16 callees[5]`(둘 다 `Effect::range`) · `08 callees[11]`(`AbstractGame::is_end`).
**고침**: 같은 파일에 이미 있는 `mkspec3.mangled_names(sym)` 로 토큰 집합을 만들어 `all(s in toks)` 로 바꾼다.

### 5-2. `ev4`(⚠미확정) 중 **9건은 실제로 IR 에 있다** — 내 담당 1건

경로에 **비식별자 성분**(`<'a, 'b>`·`<impl game_ai…>`)이 있으면 `all(seg in sym)` 은 **원리적으로 통과 불가**다(망글링에 그 문자열이 없다).

```
15 callees[7]  game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus → ev4 ⚠미확정
   실물: m13.ll:33468 @_RNvMs3_…9game_core10simulation…21AbstractGameWithCache25iter_towers_without_nexus
   막은 seg = "<'a, 'b>"
```

전역 9건: `03`×2 · `04` · `06`×2 · `08` · `12` · `15`×2.
**고침**: `segs = [s for s in path.split("::") if re.match(r"^[A-Za-z_]\w*$", s)]`.

⟹ **§4-b 의 「56행만 ev3」는 정확히는 「53행이 참 ev3 + 9행이 잘못 ev4」.**

### 5-3. `pick` 문면이 **세 가지 다른 사태를 뭉친다**

| 사태 | 전역 | 내 담당 예 |
|---|---|---|
| (a) 후보 여럿이라 못 고름 | 123행 | `15 distance_sq`(2) · `18 as_index`(2) |
| (b) **후보가 1개**(고를 것이 없다) | **83행 = ev4 의 40%** | `16` 의 10행 · `15 iter_towers_without_nexus` · `19 is_cleared` |
| (c) 호출은 IR 에 **있는데 간접(vtable)** | — | `15 get_entity_by_id`(33398·33433 `%22 = tail call … %21(…)`) · `17 tick`(17760) · `19 get_game_mode`(62708) |

(c) 는 `pick` 의 「IR 범위에 이 호출이 없다」가 **명백히 거짓**이다.

### 5-4. ★(b)「후보 1개」는 **정답의 보증이 아니다** — 내 담당 16행이 틀렸다

`fnlookup` 은 **tcx 3크레이트**만 본다. std/bumpalo/rand 구현은 사전에 없으므로, 유일한 후보가 **전혀 무관한 게임 함수**여도 그것이 실린다.

| 명세 | callees 가 말하는 것 | IR 이 말하는 것 |
|---|---|---|
| `15 callees[2]` `drop`(후보 **1**) | `<game_core::prof::ProfTimer as Drop>::drop` | **m13.ll:33726 `<Vec<Chat> as Drop>::drop`** · 33732 `<RawVec<Chat> as Drop>::drop` |
| `19 callees[2]` `drop`(후보 **1**) | 같음 | **m04.ll:62767/62773/62914 bumpalo `Vec`/`RawVec`/`IntoIter` 의 `Drop::drop`** |
| `15 callees[9..11]` `next`(9중 3) | `game_view::StatMode`/`FlowPeriod`/`ResultsPeriod::next` — **전부 game_view UI 열거형** | **m13.ll:33598 `<Copied<slice::Iter<&Entity>> as Iterator>::next`** |
| `18 callees[6..8]` `push`(6중 3) | `<Staff/Athlete/Contract as SpitzDatable>::serialize::push` — **직렬화 클로저** | **m09.ll:6929 `RawVec<Chat>::grow_one`**(=`Vec::push` 느린 경로) |
| `19 callees[8..10]` `is_empty`(3중 3) | `PatchSetting`/`ModAiRegistry`/`PublicChampionEvidence` | **bumpalo `Vec::is_empty`**(62643 `len@vec.rs:1617←is_empty@vec.rs:1636←817`) |
| `18 callees[4..5]` `position`(2중 2) | `PlayerAiContext`/`TeamRosterPlanningContext::position` | **`Iterator::position`**(`(0..5).position(closure)`, logic 667) — 게임 함수가 아니다 |

**내 담당 58 callee 행 중 16행(28%)이 「그 함수가 아님이 IR 로 증명되는」 경로를 전체 경로·시그니처·정의처까지 달고 실려 있다.** `16` 만 0건(13/13 정확).

> **고침 제안**: 앵커가 없을 때 **IR 심볼의 망글링 토큰에 그 leaf 가 있는지**를 먼저 본다.
> * leaf 가 IR 토큰에 **있는데** 어떤 후보도 안 맞으면 → 「**IR 에 이 leaf 의 호출이 있으나 tcx 3크레이트에 그 구현이 없다(std/bumpalo/rand). 후보를 싣지 않는다**」로 `callees_unmatched` 행. 위 6건 중 5건이 사라진다.
> * leaf 가 IR 토큰에 **없고** 간접호출이 있으면 → 「**간접호출(vtable) — `divtable` 로 슬롯 확인**」.
> * 후보 1개는 「후보 1개 중 1개」가 아니라 「**tcx 에 동명이 1개뿐(정답이라는 뜻은 아니다)**」.

### 5-5. 부수 — `rs[:3]` 절단이 **같은 명세의 `mem` 이 지목한 구현을 가린다** (11차 후속의 회귀)

`15 mem[3]` 은 「`divtable AbstractGame 0x1f0` → **`ExpectedGame::AbstractGame::get_entity_by_id`**」라고 적는데, `callees[4..6]` 은 `AbstractGame`(트레이트)·`Game`·`SingleLaneGame` 이고 **`ExpectedGame` 이 잘려 나갔다**(정렬 키가 경로 길이). **11차 판에는 있었다.** `19 get_game_mode` 도 동일.
> **고침**: 잘라야 할 때 **같은 명세의 `mem`/`logic` 에 이름이 등장하는 후보를 우선**(한 줄).

### 5-6. `harvest_callees` 의 길이 필터가 `new` 를 통째로 버린다

`CALLNAME = [a-z_][a-z0-9_]{3,45}` + `len(n) > 3` ⟹ **`new`(3자)는 절대 수확되지 않는다.** `15` 는 **비다이브 경로 전체**가 `SinglePlanBattle::new`(m13.ll:33428, IR 앵커 실물)인데 `callees` 에 없다(`new_dive` 만 있다). 생성자가 판정 분기의 절반인 코퍼스에서 비싼 필터다. (`choose` 는 `SKIP` 명시지만 `19` 의 `jungle_camps.choose(rnd)`(m04.ll:62941 `SliceRandom::choose`)는 **실제 판정 경로**다.)

---

## 6. `consts.kind` 어휘 확장 — **내 담당은 변화 0**

11차↔12차 분책 5파일 `diff`: `consts` 블록의 차이는 **한 줄도 없다**. 담당 32행의 `kind` 분포 = `태그` 23 · `센티널` 5 · `임계` 2 · `계수` 2. 신설 어휘(`오프셋가감`·`길이`)와 산술 intrinsic `ARITH` 인식이 건드릴 행이 없다(오프셋 가감 상수도, 길이 상수도 없다 — 배열 길이 4·6·2 는 `SPEC_GUIDE §3` 로 이미 제외: `15 closed[6]`·`19 closed[6]`).

⟹ 「`kind=미상` 7→0」의 7건은 전부 다른 배치 몫. **내 담당에서 확인할 것은 없었다**(미탐색 아님 — 전수 대조해서 변화 0).

---

## 7. 전수 대조해서 **불일치 0** 인 축 (수치)

| 축 | 행수 | 방법 | 결과 |
|---|---|---|---|
| `mem` 오프셋·방향 (5함수 전량) | **137** | IR gep 사슬을 (root,offset) 으로 누적 집계 → 표와 1:1. `15` 의 `%9/%10`(128B alloca)은 **이터레이터 내부 상태**라 대상 밖. `16` 의 9개는 `%30 = phi i64 [232,208,216,240,200,176,272,184,496]`(switch 병합)로 확인 | **불일치 0**(결손 1 = `17` Vec cap) |
| `consts.src_line` | **32** | 11차 전수분 + 12차 diff 변화 0 | **불일치 0** |
| `knobs` IR 인용(신규 포함) | **35** | `sed -n 'Lp'` 실물 대조. `_gcbc` 인용(16 knobs 5~8 · 18 knobs 19~21)도 손으로 — G13·P6 이 `gNN.ll` 을 못 본다 | **2건 어긋남**(`19 knobs[6]` 62553→62554 · `18 knobs[13]` 구간 절단) |
| `16 knobs[0]` 호출부 분포 | **13** | `_gaibc` 전량 grep = 정확히 13곳(m02×5·m14×5·m15×3), 3번째 인자 실측 = **40×5 / 60×5 / 50×2 / `%33`×1** | **주장과 완전 일치** |
| `callers` 자동열거 | **21** | 심볼별 `_gaibc` 전량 call/invoke grep = 2 / 13 / 2 / 1 / 3 | **불일치 0** |
| `17 mem` ↔ `%0` store 전수 | **41** | `%0` 상대 store 오프셋 37 + Chat 3 + Vec len 1, memset(0x120,80)이 9행을 덮는다 | **불일치 0** |
| `sig.params` 정렬·`i` 규약 | **30** | `define` 헤더 인자 수 ↔ 표 행수, sret=0 / 소스 1..n | **불일치 0** |
| `one_line`·`layer` | **10** | 산문 ↔ `logic` 대조 | **1건 보강**(`19 one_line`) · 9 일치 |
| G20 추가규칙 후보 R5/R6/R7(20명세 전역) | — | `r5probe.py`·`r7probe.py` 실행 | **전부 0건** |

---

## 8. 판정 어휘

* **사실 서술** — `mem` 137행 · `callers` 21행 · `consts.src_line` 32행 · `17` 의 DeathMatchBattle store 41행은 IR 과 **전수 일치**(0.5.8 `_gaibc`). `MainObjective`/`Chat`/`JungleType` 의 페이로드 오프셋은 **tcx 정본 확정**(ev_up 10건). 다시 파지 말 것.
* **사실 서술** — `TeamPlan+0x420` 의 `Morgard.phase`/`Serpen.phase` 중첩은 **tcx 로 확증된 양립**. G20 중첩 억제는 옳다.
* **재료 부재(범위 = tcx 3크레이트 사전)** — `15`/`19` 의 `drop`, `15` 의 `next`, `18` 의 `push`, `19` 의 `is_empty`, `18` 의 `position` 의 **진짜 콜리는 `_tcx\{game_ai,game_core,game_view}.json` 에 없다**(std·bumpalo·core::iter). 시도한 재료 = ①`fnlookup` 전 후보 열거(각 1·9·6·3·2개) ②IR 범위 call/invoke 심볼 전수 디망글 ③`mangled_names` 토큰 대조. ⟹ **이 세 재료로는 `callees` 행을 채울 수 없다**; 채우려면 std/bumpalo rlib 을 코퍼스에 넣거나(미탐색) 행을 안 싣고 `callees_unmatched` 로 보내야 한다. 내 판정이 기대는 전제 = 「IR 의 망글링 심볼이 실제 콜리의 정본이다」.
* **표기 불가** — `15 callees` 에 `SinglePlanBattle::new` 를 되살리는 것. `callees` 는 v2 에 없는 **파생 필드**라 `errors[]`·`ev_up[]` 어느 쪽으로도 주소 지정이 안 된다(`applypatch.resolve` 매핑 부재, `ev_up` 도 `arr is None` 실패). 값은 확정됐는데(m13.ll:33428) 담을 칸이 없다.
* **미탐색** — `18 open[0]`(version 축) · `15 open[0]`(판별력 있는 세계의 `update`/`dive_is_viable` version 축). 오라클 세계 구성 필요, 이번 표적 아님.
* **미탐색** — `19` 폴백 체인 **820줄**의 내용(11차가 「표기 불가」로 남긴 것). `rmeta_srcmap` 줄 길이 산술은 이번에도 안 했다.

---

## 9. 내 지시(도시에)·도구의 오류 — 8건 (`patch.json` `brief_errors`)

1. **G16 1건은 오탐이고, 그 오탐 모드를 도시에 자신이 예고했다.** 예고할 수 있으면 게이트가 걸러야 한다 — `paramrole.py` P4 에 외부 IR 인용 회피 한 줄. 덤으로 `CITE` 가 `m\d+\.ll` 만 받아 **`_gcbc` 인용은 P6 이 검증조차 못 한다**.
2. **§4-b 의 「262행 중 56행만 ev3」는 수치 자체가 틀렸다** — 거짓 앵커 3 + 놓친 앵커 9(§5-1·5-2). 11차 후속이 없애려던 「거짓 확증」이 형태만 바꿔 남았다.
3. **「`ev4` = 판정 보류」도 과소 진술** — `pick` 이 (a)후보 다수 (b)후보 1개(206 중 **83행**) (c)간접호출 셋을 한 문장으로 뭉치고, (b) 는 **정답도 아니다**(§5-4).
4. **§5 가 여전히 `callees`·`closed[].why` 를 기계로 못 고친다는 사실을 안 적었다**(11차 §4-1 재발). 이번 표적이 `callees` 인데 내 발견 12건을 전부 산문으로만 낼 수 있다 — 「기계가 적용할 수 있는 형식으로만 받는다」(§5)와 정면 배치.
5. **`errors[].kind` 어휘가 도구와 다르다** — 도시에 `실오류/오탐/보강` vs `applypatch.py` docstring `실오류|판정반전|분류오류|보강`. §5-b 4 는 「판정 반전도 오류로 세라」는데 그걸 적을 값이 §5 어휘에 없다(11차 §4-4 미반영).
6. **§1 표에 `closed` 개수 열이 아직 없다**(11차 §4-3 미반영). 담당 = 15:7 / 16:7 / 17:8 / 18:10 / 19:7 = **39건**, 10·11차의 최대 수확처인데 규모를 모르고 들어간다.
7. **지시 ⓑ 의 「`15` w ↔ `17` r 을 G20 이 정상으로 판정했다」는 전제가 틀렸다** — G20 은 이 쌍을 **검사하지 않는다**(§3-4). 「게이트가 정상 판정」과 「게이트가 안 본다」는 전혀 다른 상태이고, 후자를 전자로 적으면 다음 라운드가 그 축을 닫힌 것으로 오해한다(도시에 §1 이 스스로 경고한 형태의 변형).
8. **읽기 부담을 파일 크기로만 경고한다**(「`18`(72KB)은 특히 크다」). 실제 최대 덩어리는 **`siblings` 표**(15: 41행 · 18: 55행 · 17: 22행 = 118행)이고 어느 게이트도 안 보며 이번 감사에도 안 쓰였다. `<details>` 로 접거나 개수만 적으면 비용이 크게 준다 — 7차에 배치 둘이 죽은 원인이 크기였다면 줄일 곳은 여기다.

---

## 10. 다음 라운드 인계

1. `applypatch.py --restamp` **필수**(`op:insert` 1건 — `/specs[17]/mem at=23` → v2 `writes[16]`, 이후 25행 이동).
2. **`mkspec3._rank_callees` 3줄 수정이 최대 수익**: ①`mangled_names` 토큰 경계(거짓 ev3 3건 제거) ②비식별자 seg 제거(누락 ev3 9건 회수) ③`mem`/`logic` 언급 후보 우선(절단 회귀 복구). 그 다음이 §5-4 의 「IR 에 leaf 는 있는데 구현이 tcx 밖」 분기(잡음 16행 제거).
3. `sharedchk.skel()` 에 **숫자 단독 성분 제거** 한 줄 → R1 5건 → 2건.
4. `paramrole.py` P4 외부 IR 인용 회피 + `CITE` 를 `[gm]\d+\.ll` 로 확장.
5. `applypatch.resolve()` 에 `callees`(읽기 전용이라도 `ev_up` 가능하게)·`closed` 매핑 — 11차·12차가 연속으로 막힌 자리다.
