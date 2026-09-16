---

### `263` get_small_action_score_closure — 각 후보 SmallActionPlay 에 SubPlan::score 기본점수를 매기고, |점수|≥6 이면 액션 분류별 judge_noise_ratio[cat]/1000 배율을 곱해 (점수, 액션) 벡터로 수집

| 항목 | 값 |
|---|---|
| id | `get_small_action__score_closure` |
| 심볼 | `_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecTxNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEE12from_iter_inINtNtNtNtCsjihNppCmMEE_4core4iter8adapters3map3MapINtB3_8IntoIterBW_ENCNvMNtNtNtB10_11plan_legacy7handler7auctionNtB3l_17LegacyPlanHandler16get_small_actions1_0EEB10_` |
| 소스 | `game-ai\src\plan_legacy\handler\auction.rs:177` |
| IR | `m01.ll` 48954~49327행 |
| 경로·가시성 | `None` · **None** |
| 계층 | 플랜 핸들러 |
| exe | `ca6700` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev5)**
```rust
(없음)
```

<details><summary>인자 3개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::collections::vec::Vec<(i64, SmallActionPlay), &Bump> (32B) %0 | define 줄 속성: dead_on_unwind noalias writable sret([32 x i8]) captures(none) dereferenceable(32). 레이아웃 +0 buf.ptr · +8 buf.a(&Bump) · +16 cap · +24 len (m01.ll:48966~48979 초기화 · 49324 memcpy 32B) | 4 |
| 1 | 1 | iter | Map<bumpalo IntoIter<SmallActionPlay>, closure_env$3> (80B, by-value · dead_on_return) %1 | define 줄 속성: dead_on_return align 8 dereferenceable(80). [0..64) = closure env 8 ptr(부모 m13.ll:47126~47140 store 순): env[0]=&self.sub_plan(LegacyPlanHandler+0x768 · SubPlan 72B) · env[8]=&version(i64) · env[16]=&parameter(ScoreParameter 5384B 지역) · env[24]=rnd(&mut StdRng 320B) · env[32]=player(&PlayerState) · env[40]=data(&OperationData) · env[48]=debug(&mut DebugFrameData 224B) · env[56]=&judge_noise_ratio([i64;11] 88B 지역 = self+0x17a0 복사, auction.rs:175) · +64 IntoIter.ptr(원소 184B stride) · +72 IntoIter.end | 4 |
| 2 | 2 | bump | &Bump %2 | define 줄 속성 없음(plain ptr). 결과 Vec 의 allocator(부모: data.context.pool · m13.ll:47141~47143). sret+8 에 저장(48968) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// Vec<(i64, SmallActionPlay), &Bump>::from_iter_in(candidates.into_iter().map(closure$3), bump)   [bumpalo vec.rs:605~609 · 클로저 auction.rs:177~184]
// 부모(get_small_action · 열람 금지 · IR 만 참조)에서 env 구성: sub_plan=&self.sub_plan(+0x768) · version · parameter(지역 5384B) · rnd · player · data · debug · judge_noise_ratio(=self.judge_noise_ratio 88B 복사, auction.rs:175)
let mut v = Vec::new_in(bump);                                   // 48966~48979 {ptr=8, bump, cap=0, len=0}
let n = (iter.end − iter.ptr) / 184;                             // 48995~49001 (size_hint)
if n >= 1 { v.reserve_internal_or_panic(0, n, exact=true) }      // 49010~49025 ((bytes+183) < 367 ⟺ n==0 이면 생략)
while iter.ptr != iter.end {                                     // 49034 · 49310
    let c: SmallActionPlay = *iter.ptr; iter.ptr += 184;         // 49067 · 49079~49082 (177B + tag + 6B 복사)
    if c.tag == 0xFF { break }                                   // 49073 — Option 니치 None(실제 도달 불가)
178:    let base: i64 = sub_plan.score(version, parameter, rnd, player, data, &c, debug);   // 49092~49093 (디스패처 e388c0 · 계약만: (&SubPlan 72B, usize, &ScoreParameter 5384B, &mut StdRng, &PlayerState, &OperationData, &SmallActionPlay 184B, &mut DebugFrameData) -> i64)
179:    let cat: usize = c.get_action() as usize;                // small_action.rs:308~326 인라인 · SmallAction 판별자(game_core blackboard.rs:82 · Direct 0..10)
           // 니치 복원(49106~49110): idx = (tag > 2) ? tag−3 : 7(AroundPosition)
           // match idx(SmallActionPlay 논리 idx) → SmallAction(49111~49162):
           //   0 RunAway·1 Recall·5 AroundRunAway            → 0 RunAway        (L310)
           //   6 Positioning                                 → 1 Positioning    (L312)
           //   2 Around·3 AroundHide·10 LaneMinionPosition    → 2 Around         (L316)
           //   4 AroundRegion·7 AroundPosition·8 AroundPositionBush·9 AroundBush → 3 AroundPosition (L314)
           //   11 Trace → 4 (L321) · 12 Attack → 6 (L322) · 13 Skill → 7 (L323) · 14 Skill2 → 8 (L324) · 15 Ult → 9 (L325) · 16 Stop → 10 (L326)
           //   (5 Dodge 는 미매핑 · 그 외 idx = unreachable 49131)
180:    let score = if base.abs() < 6 { base }                   // 49164~49166 · llvm.abs(poison=false)
183:                else { base * judge_noise_ratio[cat] / 1000 };   // 49169~49173 · mul 순서 IR = ratio*base(표기 불가) · sdiv 절삭
184:    let t = (score, c);                                       // 49188~49192 → 192B 지역 %6
    if v.len == v.cap { v.reserve_internal_or_panic(v.len, 1, true) }   // 49201~49218
    v.ptr[v.len] = t (memcpy 192); v.len += 1;                   // 49292~49300
}
// 루프 종료 후 남은 원소 drop(49249~49287 · 정상 경로에선 ptr==end 라 0회) → sret = v (49324 memcpy 32B)

요지(한 줄 판정식): out[i] = ( |s|<6 ? s : s·W[cat(c_i)]/1000 , c_i )  where s = SubPlan::score(sub_plan, …, &c_i) · W = judge_noise_ratio[0..11] · cat = SmallActionPlay→SmallAction 판별자(위 표). 필터 없음 · 순서 보존 · len(out) = len(in).

&mut 표면: iter(by-value 소비 · ptr 전진) · sret 32B + 원소 192B×len (bump 할당) · rnd·debug 는 콜리 score 에만 전달(이 본문 직접 쓰기 0).
rnd: 이 본문 gen_range 사이트 0.
```

**`mem` 메모리 접근 18건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | Map<IntoIter,closure>(iter %1) | 0x40 | IntoIter.ptr (현재 원소 · 184B stride) | r | m01.ll:48983~48984 (%14 origin) · 49033 (%31) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 1 | Map<IntoIter,closure>(iter %1) | 0x48 | IntoIter.end | r | m01.ll:48985~48986 (%16) · 49032 (%30). end-ptr 로 size_hint(49001) · 루프 종료(49034·49310) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 2 | closure_env$3 | 0x0 | sub_plan (&SubPlan 72B = LegacyPlanHandler+0x768) | r | m01.ll:49047 (%43) → SubPlan::score 의 self | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 3 | closure_env$3 | 0x8 | &version → *ptr (i64) | r | m01.ll:49048 (%44) · 49092 load i64 (%62) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 4 | closure_env$3 | 0x10 | parameter (&ScoreParameter 5384B) | r | m01.ll:49049 (%45) → score 3번째 인자 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 5 | closure_env$3 | 0x18 | rnd (&mut StdRng 320B) | r | m01.ll:49050 (%46) → score 4번째 인자. 이 클로저 자체는 gen_range 호출 0 — rnd 소비는 콜리 score 내부(계약만) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 6 | closure_env$3 | 0x20 | player (&PlayerState) | r | m01.ll:49051 (%47) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 7 | closure_env$3 | 0x28 | data (&OperationData) | r | m01.ll:49052 (%48) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 8 | closure_env$3 | 0x30 | debug (&mut DebugFrameData 224B) | r | m01.ll:49053 (%49) → score 8번째 인자 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 9 | closure_env$3 | 0x38 | judge_noise_ratio (&[i64;11]) | r | m01.ll:49054 (%50) · 49169~49170 [cat] 로드 (%87 ratio). 원본 = LegacyPlanHandler+0x17a0 judge_noise_ratio(tcxdict) 의 88B 지역 복사(부모 auction.rs:175) | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 10 | SmallActionPlay(원소 c) | 0xb1 | 니치 태그 (i8 · gep 177) | r | m01.ll:49068~49069 (%58). -1(0xFF) 비교 49073 = IntoIter::next 의 Option 니치 None(실제로는 ptr==end 가 종료 조건). 49106~49110 로 SmallAction 분류 인덱스 복원 | 4 | OK |  |
| 11 | (sret) | 0x0 | buf.ptr | w | m01.ll:48966 · 49024 reserve(0, count) · 49213 reserve(len,1) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | inttoptr 8(dangling) → reserve 후 bump 할당 ptr |
| 12 | (sret) | 0x8 | buf.a | w | m01.ll:48968 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | bump(%2) |
| 13 | (sret) | 0x10 | buf.cap | w | m01.ll:48979 memset 16B | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 0 → reserve 결과 |
| 14 | (sret) | 0x18 | len | w | m01.ll:49299~49300 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 0 → 원소마다 +1 |
| 15 | (sret) buf.ptr[len] (192B stride) | 0x0 | score (i64) | w | m01.ll:49191 store → %6 지역 · 49298 memcpy 192B 로 벡터에 복사 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | %93 = \|base\|<6 ? base : base*ratio/1000 |
| 16 | (sret) buf.ptr[len] | 0x8 | SmallActionPlay 184B (payload 177 + tag@+0xb9 + 6B) | w | m01.ll:49188~49192 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 입력 원소 c 를 그대로 이동(memcpy 177 + tag store + memcpy 6) |
| 17 | Map<IntoIter,closure>(iter 지역 복사 %7) | 0x40 | IntoIter.ptr 전진 | w | m01.ll:49098·49222 (언와인드 cleanup 에서만 store · 정상 경로는 phi %55 로 레지스터 유지) · 49067 gep 184 | 4 | 확인불가(tcx 사전에 타입 없음) | old+184 |

**`consts` 상수 14건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -1 | 177 | 센티널 | 0xFF = Option<SmallActionPlay> 니치 None 센티널(IntoIter::next 인라인 · bumpalo vec.rs:1161). 태그 바이트 +0xb1 == 255 면 루프 종료. 실제 원소의 태그는 0..=19 라 항상 거짓 — 실질 종료 조건은 ptr==end(49034·49310). m01.ll:49073 · 49264 | 4 |
| 1 | 10 | 309 | 센티널 | llvm.assume(tag != 10): 니치 3+7=10 은 AroundPosition(untagged) 자리라 실제로 나올 수 없는 값. m01.ll:49106~49107 | 4 |
| 2 | -3 | 309 | 센티널 | 니치 복원 idx = tag − 3 (niche_start=3 · tcxdict --enum SmallActionPlay). m01.ll:49108 add nsw -3 | 3 |
| 3 | 2 | 309 | 임계 | tag > 2 (unsigned) 이면 tag−3 이 논리 idx, 아니면(0..2 = AroundPosition.outline_type 값) idx 7 = AroundPosition. m01.ll:49109~49110 select | 4 |
| 4 | 7 | 309 | 태그 | untagged variant AroundPosition 의 논리 idx 7(select 의 기본값). m01.ll:49110 | 4 |
| 5 | 6 | 180 | 센티널 | \|base_score\| < 6 이면 노이즈 배율 면제(점수 그대로). llvm.abs(i64, poison=false) → icmp slt 6. m01.ll:49164~49166. 5 이하의 작은 점수(고정 센티널류)는 노이즈로 흔들지 않는 의도(추정 — _docs 에 직접 주석 없음) | 4 |
| 6 | 1000 | 183 | 계수 | score = base * ratio / 1000 — judge_noise_ratio 는 천분율(‰) 배율. sdiv(부호 있는 0 방향 절삭). m01.ll:49172~49173 | 4 |
| 7 | 0 | 310 | 태그 | SmallAction::RunAway 인덱스 — SmallActionPlay idx 0 RunAway · 1 Recall · 5 AroundRunAway → 0. phi 49162 ([0,%66]×3) — ⚠IR 에 L310 마커 없음(switch 에서 phi 로 직행) · 310~311 arm 줄은 rmeta 줄 길이(63·62자) 기반 추정 | 3 |
| 8 | 1 | 312 | 태그 | SmallAction::Positioning — SmallActionPlay idx 6 Positioning → 1. m01.ll:49140~49141 · phi 49162 | 4 |
| 9 | 3 | 314 | 태그 | SmallAction::AroundPosition — idx 4 AroundRegion · 7 AroundPosition · 8 AroundPositionBush · 9 AroundBush → 3. m01.ll:49137~49138 | 4 |
| 10 | 4 | 321 | 태그 | SmallAction::Trace — idx 11 Trace → 4. m01.ll:49143~49144 | 4 |
| 11 | 8 | 324 | 태그 | SmallAction::Skill2 — idx 14 Skill2 → 8. m01.ll:49152~49153 (2=Around: idx 2 Around·3 AroundHide·10 LaneMinionPosition, 49134 / 6=Attack idx 12, 49146 / 7=Skill idx 13, 49149 / 9=Ult idx 15, 49155 / 10=Stop idx 16, 49158 — 값 5 Dodge 는 어느 variant 도 안 매핑) | 4 |
| 12 | 183 | 176 | 계수 | size_hint 예약: (end−ptr)+183 < 367 ⟺ 원소 수 0 이면 reserve 생략, 아니면 reserve(0, (end−ptr)/184). 184B stride 산술(원소 크기) — 판정 상수 아님. m01.ll:49010~49011 · 49020 sdiv 184 | 4 |
| 13 | 367 | 176 | 임계 | = 2*184−1. 위 size_hint 예약 판정 짝. m01.ll:49011 | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 노이즈 면제 임계(\|기본점수\| < 6) | auction.rs:180 (m01.ll:49165) | 6 | 올리면 더 큰 점수까지 judge_noise_ratio 배율을 안 받아 '원점수 그대로' 후보가 늘어난다(노이즈 영향↓). 내리면(예: 1) 거의 모든 후보가 분류별 배율을 받는다. 0 이면 score==0 도 배율(0 유지)이라 사실상 전부 적용 | 4 | 기존 |
| 1 | 천분율 분모 | auction.rs:183 (m01.ll:49173) | 1000 | judge_noise_ratio 의 스케일. 분모를 바꾸면 W 전체의 실효 배율이 비례해 변한다(W 와 같이 바꿔야 함). 부호 있는 sdiv 라 음수 점수는 0 쪽으로 절삭 | 4 | 기존 |
| 2 | 분류별 배율표 judge_noise_ratio[11] (SmallAction 판별자 인덱스 0 RunAway·1 Positioning·2 Around·3 AroundPosition·4 Trace·5 Dodge(미사용)·6 Attack·7 Skill·8 Skill2·9 Ult·10 Stop) | LegacyPlanHandler+0x17a0 (부모 get_small_action auction.rs:168~169 가 rnd 로 채움 · 175 복사) — 이 클로저는 읽기만(m01.ll:49169) | 런타임 값(부모 소관) | 특정 분류의 값을 1000 보다 키우면 그 분류 후보의 점수가 부풀어 선택 확률↑(음수 점수는 더 음수), 줄이면 억제. 분류 병합 규칙 자체(위 표)는 get_action 코드 — 예: Recall 은 RunAway 배율을 같이 받는다 | 4 | 기존 |

<details><summary>`callees` 피호출자 5건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | drop | <game_core::prof::ProfTimer as std::ops::Drop>::drop | pub | fn(&mut game_core::prof::ProfTimer) | game-core\src\simulation\prof.rs:184 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 1 | get_action | game_ai::SmallActionUlt::get_action | in:game_ai | fn(&game_ai::SmallActionUlt) -> game_core::SmallAction | game-ai\src\small_action\cast.rs:286 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 16개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 2 | get_action | game_ai::SmallActionPlay::get_action | pub | fn(&game_ai::SmallActionPlay) -> game_core::SmallAction | game-ai\src\small_action.rs:308 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 16개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 3 | get_action | game_ai::SmallActionTrace::get_action | in:game_ai | fn(&game_ai::SmallActionTrace) -> game_core::SmallAction | game-ai\src\small_action\trace.rs:403 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 16개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 4 | score | game_ai::plan_legacy::sub_plan::SubPlan::score | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\mod.rs:164 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 9개**: `base`, `judge_noise_ratio`, `llvm.abs.i64`, `llvm.assume`, `llvm.memcpy.p0.p0.i64`, `llvm.memset.p0.i64`, `parameter`, `reserve_internal_or_panic`, `sub_plan  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 16개는 **전부 다른 함수**라 싣지 않는다`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m13.ll:47144) · **형제 41개** (LegacyPlanHandler)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::handler::LegacyPlanHandler as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\handler.rs:13 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> game_ai::plan_legacy::handler::LegacyPlanHandler |
| 1 | <game_ai::plan_legacy::handler::LegacyPlanHandler as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\handler.rs:13 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::handler::LegacyPlanHandler::new | pub | game-ai\src\plan_legacy\handler.rs:228 | False | fn(usize, &mut rand::rngs::std::StdRng, usize, game_core::Position) -> game_ai::plan_legacy::handler::LegacyPlanHandler |
| 3 | game_ai::plan_legacy::handler::LegacyPlanHandler::r2_fight_protected | in:game_ai | game-ai\src\plan_legacy\handler.rs:362 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 4 | game_ai::plan_legacy::handler::LegacyPlanHandler::ff_note_battle_swap | in:game_ai | game-ai\src\plan_legacy\handler.rs:400 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) |
| 5 | game_ai::plan_legacy::handler::LegacyPlanHandler::mf_note_swap | in:game_ai | game-ai\src\plan_legacy\handler.rs:409 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) |
| 6 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_fall_back_to_passive | pub | game-ai\src\plan_legacy\handler.rs:416 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 7 | game_ai::plan_legacy::handler::LegacyPlanHandler::eo_cover_picks | pub | game-ai\src\plan_legacy\handler.rs:440 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> usize |
| 8 | game_ai::plan_legacy::handler::LegacyPlanHandler::eo_serpen_punish_issues | pub | game-ai\src\plan_legacy\handler.rs:445 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> usize |
| 9 | game_ai::plan_legacy::handler::LegacyPlanHandler::subplan_is_recall | pub | game-ai\src\plan_legacy\handler.rs:450 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> bool |
| 10 | game_ai::plan_legacy::handler::LegacyPlanHandler::team_objective_code | pub | game-ai\src\plan_legacy\handler.rs:456 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> u8 |
| 11 | game_ai::plan_legacy::handler::LegacyPlanHandler::update_v2_egowave | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:473 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 12 | game_ai::plan_legacy::handler::LegacyPlanHandler::v2_obj_restore_safe | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:501 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 13 | game_ai::plan_legacy::handler::LegacyPlanHandler::v2_apply_assign_commit | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:518 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut u8, &mut game_core::DebugFrameData) |
| 14 | game_ai::plan_legacy::handler::LegacyPlanHandler::sanitize_rule_scope | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:571 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::OperationData) |
| 15 | game_ai::plan_legacy::handler::LegacyPlanHandler::take_misunderstood_received_chat | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:588 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, game_core::Position, game_core::Chat) -> bool |
| 16 | game_ai::plan_legacy::handler::LegacyPlanHandler::enter_line_backfight_support | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:598 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 17 | game_ai::plan_legacy::handler::LegacyPlanHandler::update_on_dead | pub | game-ai\src\plan_legacy\handler.rs:635 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 18 | game_ai::plan_legacy::handler::LegacyPlanHandler::update | pub | game-ai\src\plan_legacy\handler.rs:685 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool) |
| 19 | game_ai::plan_legacy::handler::LegacyPlanHandler::determine_transition_reason | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1675 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &str) -> game_core::PlanTransitionReason |
| 20 | game_ai::plan_legacy::handler::LegacyPlanHandler::force_plan_update | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1709 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool) |
| 21 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_assign_anchor | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1773 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 22 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_depart_anchor | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1806 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 23 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_plan_dest | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1824 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::types::BigPlan) -> std::option::Option<(u64, u64)> |
| 24 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_repair_done | in:game_ai | game-ai\src\plan_legacy\handler.rs:1847 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 25 | game_ai::plan_legacy::handler::LegacyPlanHandler::passive_plan | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1855 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> (game_ai::plan_legacy::types::BigPlan, u8) |
| 26 | game_ai::plan_legacy::handler::auction::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::get_small_action | pub | game-ai\src\plan_legacy\handler\auction.rs:8 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &std::vec::Vec<(usize, game_core::SmallAction), std::alloc::Global>, &mut game_core::DebugFrameData) -> (game_ai::ScoreParameter, i64, game_ai::SmallActionPlay) |
| 27 | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat | pub | game-ai\src\plan_legacy\handler\chat.rs:8 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) |
| 28 | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat_inner | in:game_ai | game-ai\src\plan_legacy\handler\chat.rs:41 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) |
| 29 | game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_track_dive_episode | in:game_ai | game-ai\src\plan_legacy\handler\dive_episode.rs:35 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &game_core::PlayerState, &game_core::OperationData) |
| 30 | game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_fold_dive_episode | in:game_ai | game-ai\src\plan_legacy\handler\dive_episode.rs:125 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, usize, bool, u8) |
| 31 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v2_response_retreat_stance | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:13 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::BattleSubPlanGoal |
| 32 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:40 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> |
| 33 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage_dive | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:109 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, std::option::Option<game_core::TowerType>, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> |
| 34 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_solokill | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:136 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 35 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_interact_battle | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:233 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 36 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::update_single_lane | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:13 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 37 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::update_deathmatch | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:56 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 38 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_lane_initiate | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:196 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 39 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_try_engage | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:241 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::SinglePlanBattle> |
| 40 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_handle_solokill | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:261 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | judge_noise_ratio 원소의 실제 값 범위·생성 규칙(부모 get_small_action auction.rs:168~169 의 gen_range(RangeInclusive) 루프 — 부모 명세 열람 금지·이 배치 범위 밖). 이 클로저는 [i64;11] 을 읽기만 함(m01.ll:49169) | 4 |  |
| 1 | 미탐색 | SubPlan::score 내부(디스패처 e388c0 = m12.ll:37190 · sub_plan.rs:164 · 각 SubPlan 별 score 로 분기) — 계약만. rnd 소비(gen_range) 는 그 안에서 일어날 수 있으나 이 본문에는 0 | 4 |  |
| 2 | 표기 불가 | auction.rs:183 의 곱셈 피연산자 순서(`score * ratio` vs `ratio * score`) — IR mul %87(ratio), %63(score) · column 정보 없음 → 표기 불가(외연 동일) | 4 |  |
| 3 | 미탐색 | 노이즈 면제 임계 6 의 설계 의도 — _docs(game_ai.txt) 에 judge_noise / 면제 관련 주석 0건. '작은 고정 점수(센티널)를 흔들지 않기' 는 추정 | 5 |  |
| 4 | 미탐색 | 값 5(SmallAction::Dodge) 가 어느 SmallActionPlay 에서도 안 나오므로 judge_noise_ratio[5] 는 이 경로에서 사장(다른 소비처는 미탐색) | 4 |  |
| 5 | 미탐색 | IntoIter 남은 원소 drop 루프(49249~49287)와 언와인드 cleanup(49096~49100 · 49178~49181 · 49220~49225 · 49313~49319)은 예외 경로 — reach 가 L209 Map drop_glue 를 사장으로 판정. 정상 경로 재현엔 불필요 | 4 |  |
| 6 | 미탐색 | src_line 177 은 `.map(\|c\| {` 시작 줄(Map::new 의 inlinedAt 루트 L177 · m13.ll:47122~47140 ;L69<836<177) 이고 define 의 DI 루트는 bumpalo vec.rs:605 — 클로저 인스턴스라 둘을 병기 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

