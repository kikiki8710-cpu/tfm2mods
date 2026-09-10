# 검증 리포트 — 배치 B (05~09)

> 2026-09-11 / 게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24`
> 대상 = `_verify\05..09.json` · 정본 반영처 = `_spec\specs20.json`
> §11 원문 = `REPORT\tfm2_ai_adjust\RE\2026-09-11_20함수-반증검증-4갈래-정정13건.md`

> 2026-09-11 / 게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24`

## 0. 판정 요약

| # | 함수 | 판정 |
|---|---|---|
| 05 | `v50_fold_dive_episode` | **➕보강** — 판정 내용 오류 없음. `unknown` 3건이 새 재료로 닫힘(L132~137 = 한국어 주석 확정) |
| 06 | `v2_response_retreat_stance` | **➕보강** — "두 클로저 동일" ✅확인(명령 단위). 미확정 2건 확정 |
| 07 | `EpicHuntAndBattlePlan::sub_plan` | **🔁범위정정 + ⚠정정 2건** — L36 순서의 **추정 근거가 같은 식 안에서 반증됨** |
| 08 | `EpicHuntAndPokePlan::is_end` | **⚠정정 1건 + ✅확인** — 필터 주장 ✅. `first()` → 실제는 **`get(0)`** |
| 09 | `check_favorable_engage_formation` | **⚠정정 2건(중대)** — **「유일 호출처」가 거짓(8곳)·version 항상 0 아님**. 인원 임계 knob 표기 오류 |

**기계 검증(오프셋)**: 5건 `reads`/`writes` 전수 재확인(vtable 슬롯 6건은 구조체 아님 → SKIP) → **오귀속·밀림 0**. 도구 = `_verify\B_audit.py`(tcxdict 직결). `tcxaudit --prose` 는 이 JSON 들이 `base`/`offset` 을 분리 필드로 갖고 있어 산문 스캔이 안 걸리므로 구조화 대조로 대체했다. `★불일치?` 2건은 전부 **내 매처의 이름표기 차이**(`v50_dive_episodes.ptr` ↔ `…buf.inner.ptr.pointer.pointer`) = **오탐**. `AbstractGameWithCache+0x8`(`~덮음`)은 `&dyn` 팻포인터 뒤 절반이라 사전이 관통을 멈춘 것 — 정상.

**만든 도구**(전부 `_verify\B_*`): `B_audit.py` · **`B_ploc.py`/`B_plocfull.py`(패닉 Location 의 파일:줄:칸 추출 — 기존 `panicloc.py` 는 칼럼을 버린다)** · `B_ann.py`(IR→소스줄 inlinedAt 체인 주석) · `B_calls09.py` · `B_07_ir.txt`.

## 05 `v50_fold_dive_episode` — ➕보강 (오류 없음)

### ✅확인
**`end_reason` 0~8 표를 독립 재독해했고 전 항목 일치.** 명세가 인용한 줄이 아니라 phi 를 직접 읽었다:
- `m13.ll:23538` `%3508 = phi i8 [3,%3380],[1,%3372],[%3506,%3505],[2,%3376],[8,%3369]`
- `m13.ll:23531` `%3506 = phi i8 [6,%3401],[5,%3395],[4,%3384],[6,%3405],[7,%3416],[4,%3384],[6,%3397],[7,%3418]`
- 각 전임 블록 가드의 `!dbg` 루트: 38(entity null→8) · 39(plan tag≠9→1) · 40(`+1766`=BattlePlan+0xf6 with_dive→2) · 41(`+1774`=+0xfe dive_tower≠0xff→3) · **battle.rs:30 switch case{4,7}**(focus None→4) · 43(get_entity_by_id null→5) · 44 태그·값 2단계 + 45(→6) · 47·option.rs:742(→7). **전건 일치.**

A 블록이 B/C 를 게이트하지 않는다는 주장도 확인(`br i1 %11, %19, %12`, `%19`→`%12` 합류). `end_reason==7` 게이트(`icmp eq i8 %2, 7` @dbg 139 + `or` team_holder_ticks≠0)도 확인.

### ➕보강 ① — `unknown` #1(2회 시도 후 중단)이 닫혔다
명세: *"`in_range_ticks==0` 체크가 131줄인지 132~137 어딘가의 중첩 if 인지 확정 못 함"*. **닫힌다.**
1. `rmeta_srcmap` — **L132~137 이 전부 다바이트 헤비**: 문자수 70/67/74/68/65/64 에 **한글 37/37/31/30/36/36자**. 코드 줄이 이런 비율일 수 없다 = **한국어 주석 6줄**.
2. 함수 IR 전 구간(m13.ll 28946~29380)이 참조하는 `dive_episode.rs` 줄 전수 census = `{0,125,131,138,139,140,145~153,155,156,157,167,170}` — **132~137 은 한 번도 안 나온다.**

⟹ **132~137 에 코드 없음 ⟹ 중첩 if 가설 배제. L131 확정**(IR 도 `%11 = select i1 %7(is_some), i1 %10(in_range==0), i1 false` @dbg 131 = 단일 단축평가 `&&`).
참고: `dive_episode.rs` 는 패닉 Location **0건**이라 칼럼 재료가 이 파일엔 없다.

### ➕보강 ② — take 렌더 정밀화
명세는 `let taken = …take(); if let Some(done) = taken` 2단계. DWARF 지역변수는 `self/_version/_tps/aborted/end_reason`(L125)·`live`(L131)·`no_contact`(L138)·`end_plan`(L144)·`n`(L145)·`done`(**L156**) **뿐이고 `taken` 이 없다** ⟹ **L156 한 줄**(`if let Some(done) = self.v50_dive_ep_live.take() {`).

### ➕보강 ③ — end_plan 사슬 독립 교차검증
- L151 에 `starts_with` **두 개**(블록 `%52`,`%57`, 둘 다 dbg 151, 같은 성공 라벨 `%73`) ✓
- **줄 길이가 뒷받침**: L146~153 = 43/50/45/38/40/**76**/43/39자 — **151행만 이웃의 1.6~2배** = 술어 2개.
- "`starts_with("Recall")` 죽은 가지" — `tcxdict --enum BigPlan` 의 variant 16개가 명세 get_name 표와 **정확히 1:1** ⟹ **tcx 로 독립 교차검증**(종전엔 IR 표가 유일 근거). variant 는 `ForcePassive`(정상 철자)이므로 원문 오타는 **문자열 리터럴 쪽**이라는 명세 기술이 맞다.

### 남은 재료
`end_reason` 표의 **제3 독립 출처**: rmeta 주석 0건 · 이 파일 패닉 Location 0건 · exe 디스어셈 **미탐색**. 현 근거 = IR 2회 독립 독해(전건 일치).

## 06 `v2_response_retreat_stance` — ➕보강 (오류 없음)

### ✅확인 — "두 클로저가 반경만 다르다"는 **완전히 맞다**
`m12.ll:41916~41998`(closure0) ↔ `42001~42083`(closure2) 를 SSA·메타ID 정규화 후 diff: **83줄 vs 83줄, 차이 3곳** — ①심볼명 ②패닉 Location 상수(`.49`↔`.50`) ③`icmp ult … 40000000001` ↔ `22500000001`. 명령 시퀀스는 **한 줄도 다르지 않다**.

### ➕보강 ① — "별개 클로저 vs 헬퍼 인라인" 확정
mangled 심볼이 `…v2_response_retreat_stance**0**I…` / `…v2_response_retreat_stance**s0_0**I…` = **같은 함수 DefId 아래 클로저 #0 / #2**. 임계만 받는 헬퍼였다면 클로저 DefId 의 부모가 그 헬퍼가 된다. ⟹ **소스에 두 클로저 리터럴이 이 함수 본문 안에 직접 있다 — 확정**(DWARF `closure_env$0/$2` 보다 강한 근거).

### ➕보강 ② — `unknown` #5(min_by_key 동점) 확정
m12.ll:12613~12615 (08 판 = 14034~14036):
```
%92 = icmp slt i8 <compare(acc,new)>, 1     ; = is_le()
%93 = select i1 %92, ptr <acc>, ptr <new>
```
`%5`=acc(phi 로 실려온 현재 최소), `%4`=new 를 그 순서로 store 후 `compare(&acc,&new)`; `is_le` 에서 **acc 유지** ⟹ `cmp::min_by`(`Less|Equal => v1`)와 일치 = **동점이면 먼저 나온 원소**. 순회가 슬롯 0→4 이므로 **동점 시 낮은 포지션 슬롯이 이긴다.** (06·08 동시 해소.)

### ➕보강 ③ — 소스 줄/칼럼
`!DILocation` 루트 = closure0 engage.rs **21/22/23**, closure2 **28/29/30** (명세대로). 패닉 칼럼 = closure0 bounds_check **21:19**, closure2 **28:21**. 줄 길이 101/53/51 ↔ 103/55/74 — 앞 두 쌍 **정확히 +2**(들여쓰기)로 자기정합, **셋째만 +23**(closure2 30행에 추가 텍스트) → **미탐색**(판정 무영향).

## 07 `EpicHuntAndBattlePlan::sub_plan` — 🔁범위정정 + ⚠정정 2건

### 🔁범위정정 ★ — L36 두 OR 항 순서: **명세의 "추정" 근거가 같은 식 안에서 반증된다**

명세 `still_unknown`: *"**추정**: 피연산자 순서 보존 경향상 `hp_ratio < 51` 이 좌항일 가능성이 높다(검증법 없음)."*

실측 IR (m02.ll:49087~49091):
```
%94 = icmp ult i64 %33, 51        ; hp_ratio < 51          (L36 에서 생성)
%95 = or  i1 %94, %85             ; %85 = can_upgrade_item (L35 호출 결과)
%96 = icmp ult i64 %26, %28       ; champ.hp < champ.stat_cached.hp
%97 = and i1 %96, %83             ; && is_in_heal_area
%98 = or  i1 %97, %95
```
명세 자신이 (옳게) **"내부 or 가 {hp_ratio<51, can_upgrade_item} 을 묶으므로 세 번째 항 = and-항"** 으로 확정했다. 그러면 소스는 좌결합 `(A||B)||C` 이고 바깥 or 는 `or(%95, %97)` 이어야 한다. **실제는 `or(%97, %95)` — 역순이다.**
⟹ **"피연산자 순서 ≈ 소스 순서" 휴리스틱은 이 표현식 안에서 이미 깨졌다. 명세의 두 진술(세 번째 항 확정 / 좌항 추정)은 동시에 성립할 수 없다.**

또 관측 순서는 **정의 순서(rank)** 로 완전히 설명된다 — 두 or 모두 나중에 정의된 쪽이 앞(`%94`>`%85`, `%97`>`%95`). 결정적으로 **`can_upgrade_item`(%85)은 L35 호출 결과라 소스에서 좌항이든 우항이든 반드시 `%94`(L36 생성)보다 먼저 정의된다** ⟹ **관측된 `or(%94,%85)` 는 어느 소스 순서에서도 동일하게 나오며 정보량 0.** 방향 추정은 **삭제하거나 "근거 무효"로 강등**해야 한다(판정 결과에는 여전히 무영향).

#### ➕보강 — 새 무기 2종을 실제로 대 봤고, 둘 다 이 줄엔 **원리적으로 못 닿는다**
1. **패닉 Location 칼럼** — `old\epic\hunt_and_battle.rs` 의 Location 은 `21:17 · 21:95 · 23:93 · 29:17 · 29:95 · 31:20 · 32:58 · 56:58` **8개뿐이고 36행이 없다.** L36 에 패닉 가능 연산이 하나도 없다(비교 2개·bool 로드·필드 로드뿐 — 인덱싱·나눗셈·unwrap 없음) ⟹ **재료 부재**(칼럼 무기의 사각지대).
2. **줄 길이 산술** — `A || B` 와 `B || A` 는 **같은 길이**. L36 이 149자임을 알아도 순서는 **원리적으로 못 가른다**. "안 해봤다"가 아니라 **방법의 불변량**.

★ 칼럼 무기가 이 파일에서 **실제로 작동**하는 것은 검증했다 — L29 ±0 복원:
```
    let champ = data.cache.player_champion[player.info.team][player.info.position.as_index()].unwrap();
```
길이 103자(srcmap 104−1) · `data` 가 **col 17**(bounds_check Location) · `unwrap` 이 **col 95**(unwrap_failed Location) — **3중 동시 일치**. 보정 규칙 확정: **bounds_check 칼럼 = 인덱싱식 시작**, **`.unwrap()` 칼럼 = 메서드 이름 시작**.

⟹ 권장 최종 문구:
> `hp_ratio < 51` vs `upgrade_item(..).is_some()` 의 소스 순서 = **재료 부재로 종결**. ①MIR `mir=0 xinl=0`(rmeta 미인코딩) ②`DILocation.column` 전 모듈 0 ③**패닉 Location: L36 에 패닉 가능 연산이 없어 상수 자체가 없음(실측)** ④**줄 길이 산술: 교환 시 길이 불변 = 원리적 불가** ⑤**IR 피연산자 순서: rank 정렬로 설명되며 정보량 0(같은 식의 바깥 or 가 소스 역순인 것으로 반증)**. 미탐색 = 게임 exe 디스어셈(같은 IR 산물이라 기대치 낮음)·개발사 소스.

### ⚠정정 ① — `live_list.get(0)` 은 맞고, **08 쪽이 틀렸다**
인라인 체인 `!54410`→`!54400` = `DISubprogram(name: "get<usize,usize>", file: slice\mod.rs, line: 572)` ⟹ **`<[T]>::get`** 확정. **07 이 맞다.**

### ⚠정정 ② — Hide 구성 렌더 (경미)
`logic` 은 `bush: self.target_bush.unwrap()` 을 인라인으로 썼으나, DWARF `!54396 = DILocalVariable(name:"target_bush", line: 42)` + `!54542 = DILocation(line: 42)` 가 `self+0x8` 로드에 붙는다 ⟹ **L42 에서 지역변수로 묶고 L43(`!54544`)에서 `SubPlan::Hide{…}` 4필드 store.** (42행에는 패닉 Location 이 없다 — L41 `is_some()` 로 unwrap 분기가 죽어 제거된 것과 정합.)

### ➕보강 — `unknown` 2건 해소 + **쌍둥이 파일 경고**
- *"Vec 필드 순서를 DWARF 로 직접 확인 못 했다"* → **tcxdict 확정**: `MobaMode+0x198 = …live_list.buf.inner.cap` / `+0x1a0 = ptr` / `+0x1a8 = len`. (08 의 동일 `unknown` 도 해소.)
- `MapDef+0x6d70 fountains` 원소 = **이름 없는 `(u64,u64,u64,u64)` 튜플**(tcx).
- ★**`old\epic\hunt_and_battle.rs` 와 `old\serpen\hunt_and_battle.rs` 는 줄번호까지 동일한 쌍둥이다.** 패닉 Location 이 21:17/21:95/23:93/29:17/29:95/31:20 까지 **완전히 같고**, 32·56행만 칼럼 58↔60(=`epic`↔`serpen` 2자 차). **줄번호만 보고 파일을 판별하면 반드시 섞인다.**

### ✅확인 (그 외)
분수대 사각형 판정(`x∈[f.0,f.2] && y∈[f.1,f.3]`), hp_ratio=hp*100/max, **L41 게이트가 `select i1 %109(tps+ally_tick>ally_killed_tick), i1 %111(target_bush.is_some), false` — 단축평가 보존이라 tick 비교가 좌항임 확정**, 태그 phi `{5,9,11}`, `store i8 1`(out_line), 오프셋 전건.

## 08 `EpicHuntAndPokePlan::is_end` — ⚠정정 1건

### ✅확인 ★ — "필터는 `is_recent_visible` 단 하나" 는 **맞다**
- 모노모픽 min_by_key 본체 `m10.ll:6028~6196`: 루프 안 술어 호출이 `Blackboard::is_recent_visible` **1개**(`%33` @6130)뿐이고 곧바로 키(`|dx|²+|dy|²`). 추가 술어 0.
- fold `m12.ll:13847~14059`: 동일(`%48` @13974). **판정 임계 상수 0건** 재확인. `14062~14274` = Serpen 판이라는 경고도 유효.

### ⚠정정 — `live_list.first()` → 실제는 **`.get(0)`**
`!19322` 체인 실측:
```
index.rs:219   fn=get<usize>
 └ mod.rs:576  fn=get<usize,usize>          ← <[T]>::get (slice\mod.rs:572 선언)
    └ hunt_and_poke.rs:193 fn=is_end
```
`slice::first()` 는 `get` 을 부르지 않고 슬라이스 패턴으로 구현돼 있어, 소스가 `first()` 였다면 `fn=first` 프레임이 나와야 하는데 **없다.** ⟹ 소스는 `live_list.get(0)`(`and_then` 은 `!19367` 에서 `closure_env$2` + `option.rs:1543` 확인). **두 명세가 서로 어긋나 있었고, 틀린 쪽이 08.**

### ✅확인 (전건 IR 재확인)
L164 = `TeamPlan+0x41f == 0`(체인 `team_plan.rs:231 ← :244(take_active)`) · L172 = `+0x41f==0 && +0x420==1`(`team_plan.rs:258`) · L179 `is_visible_cell` = vt **+256(0x100)**, 인자 `(team, x/32000, y/32000)` · L184 `icmp ugt … 22500000000` · L194 `mul tps, 15` + `usub.sat(next_respawn(+0x1b0), tick(vt+0x28))` · ★**`get_game_mode` 실제 2회 호출**(`%40` @7719, `%106` @7863). `iter_champions` 인라인 루트는 `simulation.rs:**1905**`(명세 1904 — 1줄 차, 경미).

### ➕보강
Vec 배치 tcx 확정(cap@0x198/ptr@0x1a0/len@0x1a8) — `unknown` 해소. min_by_key **동점 = 첫 최소 유지** IR 확정(06 항목).

## 09 `check_favorable_engage_formation` — ⚠정정 2건 (중대 1)

### ⚠정정 ★★ — **「유일 호출처」가 거짓. 호출부 8곳이고, version 이 리터럴 0 인 곳은 1곳뿐**

명세 두 곳(`signature` 노트 · `unknown`)이 같은 전제를 쓴다: *"유일 호출처(`should_disengage_object_hunt`, m15.ll:33271)는 200000 을 넘긴다 / **리터럴 0** 을 넘긴다."*

전수 실측(`_verify\B_calls09.py`):

| IR 위치 | version | engage_range | 감싼 함수 |
|---|---|---|---|
| m13.ll:18805 | `%1561` | 200000 | `plan_legacy::handler::LegacyPlanHandler::update` |
| m13.ll:35679 | `%484` | 200000 | `handler::engage::LegacyPlanHandler::handle_interact_battle` |
| m13.ll:37141 | `%1007` | 200000 | 〃 |
| m13.ll:39946 | `%2204` | 200000 | 〃 |
| m13.ll:40936 | `%2579` | 200000 | 〃 |
| m13.ll:41820 | `%2959` | 200000 | 〃 |
| m13.ll:42834 | `%3337` | 200000 | 〃 |
| m15.ll:33271 | **0(리터럴)** | 200000 | `fight_check::should_disengage_object_hunt` |

⟹ ①**호출부 8곳**(7곳이 `plan_legacy::handler`) ②**7곳은 런타임 `version` SSA 값**.
**영향**: 이 함수는 `version` 을 `enemy_minion_line_action_danger_damage_at` 로 그대로 흘린다. 「version 은 늘 0 이라 사실상 무의미」로 읽히던 항목이 **주 경로에서는 살아 있는 버전 게이트**다. 재구현 시 0 하드코딩 금지.
- `engage_range = 200000` 은 **8곳 전부 맞다**(✅).
- 부수: 명세 `exe.callers: []` 도 실체와 어긋남 — **exe 조인 실패**라는 사실을 적어 둘 것.
- 개발자 주석 근거 신규 확보: `_docs\game_ai.txt:281` — *"다이브의 두 판단축(콜: `check_favorable_engage_formation`=진형/숫자우위, 실제 교전 판단: HP 임계값 기반 `is_unreasonable_tower_dive_enemy`)"*. 이 명세엔 rmeta 주석 근거가 0건이었다.

### ⚠정정 — knob 「대형 성립 인원 조건」의 `value: 1`
실제 비교 상수는 **0 과 1 이 섞여 있다**(m15.ll:35578·35598~35601·35606):
```
1316  %86  = icmp sgt i32 rear , 0
1321  %95  = icmp sgt i32 flank, 0
      %96  = icmp sgt i32 front, 0
      %97  = select i1 %95, i1 %96, false     ; flank>0 && front>0  ← 단축평가 보존
      %98  = icmp sgt i32 flank, 1
      %99  = or i1 %98, %97
1331  %101 = icmp sgt i32 front, 1
```
단일 스칼라 `1` 로 적으면 **"임계가 전부 1"** 로 오독된다. `{1316:0, 1321:(1,0,0), 1331:1}` 로 적을 것. (`logic` 블록 자체는 정확 — 오류는 knobs 표에만.)

### ➕보강 — 1321 의 **안쪽 `&&` 순서는 확정된다**
명세 `unknown` 의 "순서 확정 불가"는 **바깥 `||` 에만 해당**한다. 안쪽 `&&` 는 `select i1 %95, i1 %96, i1 false` 로 단축평가가 남아 **`flank>0` 이 좌항임이 확정**. 적용 범위를 붙일 것.
(대비: 07 L41 `&&` 도 `select` 로 남아 순서 확정 / 07 L36 은 `and`·`or` 로 평탄화돼 소실.)

### ➕보강 — fountains 원소 타입
`tcxdict MapDef 0x6d70` → `fountains[0].0 : u64` ⟹ **원소는 명명 구조체가 아니라 4-튜플 `(u64,u64,u64,u64)`**. "원소 구조체명 확정 못함"이라는 질문 자체가 성립하지 않는다.

### ✅확인 (본문 전건)
소스 줄 배정 **전건 일치**(1203/1204/1208/1209/1219/1221/1223/1234/1235/1240/1247/1252/1254/1256/1258/1264/1267/1270/1278/1279/1280/1283/1286/1288/1289/1290/1296/1298/1300/1302/1304/1307/1316/1321/1331/1340/1348, `!DILocation` 실측) · 각도 임계(`shl i128 dot_sq,2` vs lps / `dot_sq*100 > lps*9` / `cross_sq*100 > lps*9`) · 분기 순서(front → rear방향 → flank → front) · `hp*100/max < 40` · `engage_range+100000` 제곱(루프 밖 호이스트) · `champ_to_base*5 <= enemy_to_base*6` · 오프셋 전건.

## 부록 B — 이번에 검증된 방법론 사실 (다음 배치용)

1. **패닉 Location 칼럼은 살아 있고 실제로 쓸 수 있다.** 보정: `bounds_check` 칼럼 = **인덱싱식 시작**, `.unwrap()` 칼럼 = **메서드 이름 시작**. `panicloc.py` 는 칼럼을 버리므로 `_verify\B_plocfull.py` 를 쓸 것. ⚠**한계(실측)**: 패닉 가능 연산이 없는 줄엔 상수가 아예 없다(07 L36).
2. **줄 길이 산술은 `A||B` 순서 문제에 원리적으로 못 쓴다**(교환 시 길이 불변). "못 한다"로 적을 것.
3. **IR `or`/`and` 피연산자 순서로 소스 순서를 추정하지 마라.** 07 에서 같은 식의 바깥 `or` 가 소스 역순임이 드러났고 두 or 모두 rank 내림차순으로 설명된다. 반면 **`select i1 A, i1 B, i1 false/true` 로 남았으면 단축평가가 보존된 것이라 순서 확정**(07 L41 · 09 L1321 안쪽).
4. **`rmeta_srcmap` 의 다바이트 문자수(mb)는 "주석 줄" 판별기다.** 한 줄에 한글 30자 이상이면 코드가 아니다. + "그 줄이 IR 어디에도 안 나온다"를 겹치면 빈 구간의 정체를 확정할 수 있다(05 L132~137).
5. **인라인 체인의 `DISubprogram(name:)` 을 봐라.** `first()` vs `get(0)` 처럼 파일:줄만으로는 못 가르는 것이 이름으로 즉시 갈린다(08 정정의 근거).
6. **`_docs` grep 은 함수명만으로 약하다** — 09 는 *다른 함수의* 주석 본문에 이름이 등장해서 잡혔다. 주변 개념어로도 grep 할 것.

