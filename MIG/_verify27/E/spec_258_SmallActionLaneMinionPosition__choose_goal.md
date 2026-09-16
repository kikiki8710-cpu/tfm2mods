---

### `258` SmallActionLaneMinionPosition::choose_goal — 라인 미니언 공격 위치 후보(현재 위치·목표 주변 16방향 링·7×7 격자)를 push_candidate 로 채점해 최고 점수 (x,y,score) 를 고른다

| 항목 | 값 |
|---|---|
| id | `SmallActionLaneMinionPosition__choose_goal` |
| 심볼 | `_RNvMNtNtCshdEBA0ozCnw_7game_ai12small_action11lane_minionNtB2_29SmallActionLaneMinionPosition11choose_goal` |
| 소스 | `game-ai\src\small_action\lane_minion.rs:493` |
| IR | `m11.ll` 43376~43887행 |
| 경로·가시성 | `game_ai::SmallActionLaneMinionPosition::choose_goal` · **in:game_ai** |
| 계층 | 기타 |
| exe | `e248f0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::SmallActionLaneMinionPosition, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PositioningScoreData, game_core::LineType) -> std::option::Option<(u64, u64, i64)>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[258]/sig/tls/<키>`)**

- `name`: 없음
- `role`: 없음 — 본문(43376~43887)에 `LocalKey`/`call_once`/`llvm.threadlocal.address`/`@anon.* = constant ptr @…call_once` 참조 0. @anon 참조 자체가 없다
- `key`: 해당 없음
- `layout`: 해당 없음
- `invalidation`: 해당 없음
- `call_conditions`: 콜리 push_candidate → position_score_at_position(→ position_eval_at 계약만) 쪽에 TLS 메모가 있을 수 있으나 이 함수 범위 밖(미열람)

<details><summary>인자 10개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<(u64,u64,i64)> (32B) %0 | define 줄 속성: dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(32). +0 태그(i64: 0=None · 1=Some) · +8 x(u64) · +0x10 y(u64) · +0x18 score(i64). DI 반환 타입 !5887 = enum2$<Option<tuple$<u64,u64,i64>>> | 4 |
| 1 | 1 | self.goal_score (ArgumentPromotion 조각 1/2) | i64 %1 | IR 에 `&self` 는 없다(DI self = ptr poison). 호출자 get_input(m11.ll:46257~46260) 이 `self+0x20`(=SmallActionLaneMinionPosition.goal_score i64, tcxdict) 을 load 해 넘긴다. 본문에서는 push_candidate 의 13번째 인자 `target_score` 로만 3회 전달(43538·43869·43878) — 분기 없음. IR 속성 없음 | 3 |
| 2 | 1 | self.position_eval_purpose (ArgumentPromotion 조각 2/2) | i8 %2 | 호출자 get_input 이 `self+0x78`(position_eval_purpose: PositionEvalPurpose 1B) 을 load 해 넘김(m11.ll:46259~46260). push_candidate 10번째 인자(range(i8 0,13)) 로만 3회 전달. IR 속성 없음 | 4 |
| 3 | 2 | version | usize (i64 %3) | define 줄 속성: noundef. 본문 분기 없음 — push_candidate 2번째 인자로 3회 전달만 | 4 |
| 4 | 3 | player | &PlayerState(2528B) %4 | define 줄 속성: noalias noundef nonnull readonly captures(address, read_provenance) dereferenceable(2528). 본문 필드 읽기 0 — push_candidate 3번째 인자로 전달만 | 4 |
| 5 | 4 | data | &OperationData(24B) %5 | define 줄 속성: noalias noundef nonnull readonly captures(address, read_provenance) dereferenceable(24). +8 context 만 읽음(→ GameContext +0 pool(bump) · +8 setting · +0x20 map). push_candidate 4번째 인자로도 전달 | 4 |
| 6 | 5 | champ | &Entity(1728B) %6 | define 줄 속성: noalias noundef nonnull readonly captures(address, read_provenance) dereferenceable(1728). 읽는 필드: +0x4c0 attack_effect 니치태그 · +0x4a0/+0x4a8 attack_effect.range/growth_range · +0x438 stat_buff_cached.range · +0x470 radius_mult · +0x5c8 level · +0x660 x · +0x668 y · +0x680 radius. range_adjust(caster)·Entity::distance(self)·push_candidate 5번째 인자 | 4 |
| 7 | 6 | target | &Entity(1728B) %7 | define 줄 속성: noalias noundef nonnull readonly captures(address, read_provenance) dereferenceable(1728). 읽는 필드: +0x470 radius_mult · +0x680 radius · +0x660 x · +0x668 y. push_candidate 에는 &Entity 가 아니라 target.x/target.y 두 i64(%91/%93) 로 전달(push_candidate 쪽 ArgumentPromotion) | 4 |
| 8 | 7 | positioning_score | &PositioningScoreData(2760B) %8 | define 줄 속성: noalias noundef nonnull readonly captures(address, read_provenance) dereferenceable(2760). +0xab8 cx · +0xac0 cy 만 직접 읽음(격자 중심). push_candidate 7번째 인자로 전달(value[7][7] 은 콜리에서 소비) | 4 |
| 9 | 8 | line | LineType (i8 %9, range(i8 0,3)) | define 줄 속성: noundef range(i8 0, 3). 본문 분기 없음 — push_candidate 14번째 인자로 전달만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn choose_goal(&self, version, player, data, champ, target, positioning_score, line) -> Option<(u64,u64,i64)>   // lane_minion.rs:493
// 494  (helper Self::attack_range(champ,target)->Option<u64>, lane_minion.rs:354~357 인라인)
let atk = champ.attack_effect.as_ref()?;                     // 355: +0x4c0 == -1 → return None
attack_range = atk.range(champ)                              // 356: atk.range + champ.stat_buff_cached.range + (champ.level-1)*atk.growth_range (effect.rs:26 인라인)
             + atk.range_adjust(champ, target)               //      계약만(gc)
             + champ.radius() + target.radius();             //      entity.rs:1511~1515 인라인
// 495~498
preferred_range = if attack_range > 69999 { attack_range.saturating_sub(14000) }   // 496
                  else { (attack_range as u32 * 65 / 100) as u64 };                  // 498
// 500
let mut candidates: bumpalo::Vec<(u64,u64,i64)> = Vec::new_in(data.context.pool);
// 502~503  후보 1: 현재 위치
Self::push_candidate(&mut candidates, version, player, data, champ, target.x, target.y, positioning_score,
                     champ.x, champ.y, self.position_eval_purpose, attack_range, preferred_range, self.goal_score, line, /*source_bonus*/ 35);
// 505~517  후보 2: 목표 주변 16방향 링(선호 사거리 반지름) — 목표가 사거리+1.5셀 안일 때만
if champ.distance(target) <= attack_range + 48000 {          // 505 (ugt 이면 건너뜀)
    for (dx,dy) in DIRS16 {                                   // 512 (1000,0)(-1000,0)(0,1000)(0,-1000)(±707,±707)(±923,±382)(±382,±923)
        x = target.x + dx*preferred_range/1000;               // 513 (i64 sdiv)
        y = target.y + dy*preferred_range/1000;               // 514
        (x,y) = Game::adjust_position(data.context.map, data.context.setting, x, y);   // 515 계약만 → (u64,u64)
        Self::push_candidate(…, x, y, …, /*source_bonus*/ 0); // 516
    }
}
// 521~534  후보 3: positioning_score 중심 7×7 셀 격자
cx = positioning_score.cx as i32; cy = positioning_score.cy as i32;   // 521~522
for dx in 0..7 { for dy in 0..7 {                            // 523~524
    xi = cx - 3 + dx; yi = cy - 3 + dy;                       // 524~526
    if xi < 0 || yi < 0 || xi > 29 || yi > 29 { continue }    // 527 (한 줄 안 순서 표기 불가·전부 순수)
    x = xi*32000 + 16000; y = yi*32000 + 16000;               // 531 셀 중심
    Self::push_candidate(…, x, y, …, /*source_bonus*/ 10);   // 532
} }
// 537~538
candidates.into_iter().max_by_key(|c| c.2)                  // 537: key 클로저 choose_goal::{closure#0} = 튜플 3번째(score) · fold 는 m12.ll:34778 별도 define
// max_by 접기: compare(acc,new)==Greater 이면 acc 유지, 아니면(Less·Equal) new 채택(m12.ll fold %26 `sgt 0` select) ⟹ **동점이면 나중에 push 된 후보가 이긴다**(현재위치 < 링 < 격자 순으로 뒤가 우선)
// 후보 0개(모두 push_candidate 에서 거절) → None

※ 후보 채점(위치 점수·위험·source_bonus 합산·거절 조건)은 전부 push_candidate(m11.ll:44238~44467) 안 — 이 함수는 좌표 생성과 최댓값 선택만.
※ 판 8 리턴주소 실측이 잡은 exe 0xe25450 은 이 함수가 아니라 push_candidate 다(note/unknown 첫 항목).
```

**`mem` 메모리 접근 22건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | Entity | 0x4c0 | attack_effect@tag (니치 = Some.0.casting 4B, champ) | r | m11.ll:43420~43422 (%17/%18 · i32 == -1 → None). tcxdict Entity 0x4c0: attack_effect@tag(Niche). 인라인 helper attack_range(lane_minion.rs:355 `champ.attack_effect.as_ref()?` · option.rs:742) | 3 | OK |  |
| 1 | Entity | 0x490 | attack_effect@Some.0 (&Effect 56B 시작, champ) | r | m11.ll:43426 (%21 = champ+1168). range_adjust 의 self 로 전달 | 4 | OK |  |
| 2 | Entity | 0x4a0 | attack_effect.range (champ) | r | m11.ll:43429~43430 (%22/%23). Effect::range(caster) 인라인(effect.rs:26) — Effect+0x10 | 4 | OK |  |
| 3 | Entity | 0x4a8 | attack_effect.growth_range (champ) | r | m11.ll:43431~43432 (%24/%25). `(level-1)*growth_range` — Effect+0x18 | 4 | OK |  |
| 4 | Entity | 0x5c8 | level (champ) | r | m11.ll:43433~43435 (%26/%27/%28 add -1) | 4 | OK |  |
| 5 | Entity | 0x438 | stat_buff_cached.range (champ) | r | m11.ll:43437~43438 (%30/%31) | 4 | OK |  |
| 6 | Entity | 0x470 | stat_buff_cached.radius_mult (champ 43440 / target 43463) | r | i32. Entity::radius() 인라인(entity.rs:1511~1515) | 4 | OK |  |
| 7 | Entity | 0x680 | radius (champ 43447·43454 / target 43470·43477) | r | attack_range 에 양쪽 반경 가산(356) | 4 | OK |  |
| 8 | Entity | 0x660 | x (champ 43529·43531 / target 43534~43535) | r | champ.x → 첫 후보 좌표(503) · target.x → 링 후보 중심(513) 및 push_candidate 6번째 인자 | 4 | OK |  |
| 9 | Entity | 0x668 | y (champ 43532~43533 / target 43536~43537) | r | 503·514 | 4 | OK |  |
| 10 | OperationData | 0x8 | context (&GameContext) | r | m11.ll:43520~43521 (%80/%81) | 4 | OK |  |
| 11 | GameContext | 0x0 | pool (&Bump) | r | m11.ll:43522 (%82). bumpalo Vec::new_in(500) | 4 | OK |  |
| 12 | GameContext | 0x20 | map (&MapDef 28112B) | r | m11.ll:43651·43717 (%134/%160). Game::adjust_position 1번째 인자(515) | 4 | OK |  |
| 13 | GameContext | 0x8 | setting (&GameSetting 5432B) | r | m11.ll:43652·43718 (%135/%161). Game::adjust_position 2번째 인자(515) | 4 | OK |  |
| 14 | PositioningScoreData | 0xab8 | cx | r | m11.ll:43656~43658 (%137/%138 · trunc i32). 격자 중심 x(521) | 4 | OK |  |
| 15 | PositioningScoreData | 0xac0 | cy | r | m11.ll:43660~43662 (%140/%141). 격자 중심 y(522) | 4 | OK |  |
| 16 | bumpalo::Vec<(u64,u64,i64)> (지역 %16 32B) | 0x0 | ptr (원소 배열 시작 · 초기 dangling 8) | r | m11.ll:43524(store) · 43754(load %175). 원소 24B stride(43760 `mul %176, 24`): +0 x · +8 y · +0x10 score | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 17 | bumpalo::Vec<(u64,u64,i64)> (지역 %16) | 0x18 | len | r | m11.ll:43528·43755~43756 (%85/%176 count). 0 이면 None(43784) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 18 | sret Option<(u64,u64,i64)> | 0x0 | 태그 | w | 유일한 반환 쓰기 표면. define 속성 writeonly captures(none) | 4 | 확인불가(tcx 사전에 타입 없음) | 0(None: 43497 attack_effect None · 43826~43828 phi [1,%188]/[0,%192] 후보 없음) / 1(Some) |
| 19 | sret Option<(u64,u64,i64)> | 0x8 | (x,y,score) 24B | w | None 경로에서는 미기록(살아있지 않음) | 4 | 확인불가(tcx 사전에 타입 없음) | max_by_key 결과의 +8..+0x20 memcpy(m11.ll:43810) — Some 경로에서만 |
| 20 | 지역 bumpalo::Vec<(u64,u64,i64)> %16 (32B alloca) | 0x0..0x20 | ptr=8(dangling) · +8 bump · +0x10 cap=0 · +0x18 len=0 | w | 이후 push 는 push_candidate 안에서(콜리 부작용). 함수 끝에 drop_glue(43885, unwind 경로) / IntoIter drop(43816 · 후보 0개일 때) — bumpalo 라 메모리 해제는 없음 | 4 | 확인불가(tcx 사전에 타입 없음) | Vec::new_in(data.context.pool) 초기화(43524~43530) |
| 21 | 지역 IntoIter<(i64,i64),16> %15 (272B alloca) | 0x10..0x110 | dirs 16쌍 상수 채우기 | w | m11.ll:43590~43650. 배열 into_iter 의 alive 범위는 +0..0x10(SSA %146 로 접힘) | 4 | 확인불가(tcx 사전에 타입 없음) | (1000,0)(-1000,0)(0,1000)(0,-1000)(707,707)(707,-707)(-707,707)(-707,-707)(923,382)(923,-382)(-923,382)(-923,-382)(382,923)(382,-923)(-382,923)(-382,-923) |

**`consts` 상수 20건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -1 | 494 | 센티널 | Option<Effect> 니치 None 센티널 — champ+0x4c0(casting 태그 i32) == -1 이면 attack_effect None → 즉시 None 반환(m11.ll:43422 · helper attack_range 355줄 `?`) | 4 |
| 1 | 100 | 356 | 계수 | Entity::radius() 인라인(entity.rs:1515) `radius*(radius_mult+100)/100`(43456·43458·43479·43481) | 4 |
| 2 | 69999 | 495 | 임계 | `attack_range > 69999`(= >= 70000 · m11.ll:43493 icmp ugt) → 장거리 분기(496). 70000 = 2.1875셀(셀 32000) | 4 |
| 3 | -14000 | 496 | 계수 | `attack_range.saturating_sub(14000)`(uint_macros 2472 · DI rhs=14000)가 `add %67, -14000` 로 접힘(43512 · %67>69999 이므로 포화 불필요) — 장거리 챔프의 선호 사거리 = 사거리 - 14000 | 4 |
| 4 | 65 | 498 | 미상 | 단거리 챔프 선호 사거리 = attack_range*65/100(43505 mul i32 · 43506 udiv 100). i32 로 trunc 되는 것은 <=69999 가 보장돼서 | 4 |
| 5 | 35 | 502 | 미상 | 첫 후보(champ 현재 위치)의 source_bonus(push_candidate 16번째 인자, 43538) — 제자리 유지 가산 | 4 |
| 6 | 48000 | 505 | 계수 | `champ.distance(target) > attack_range + 48000`(43551~43552) 이면 링 후보(512~516) 생략. 48000 = 1.5셀 | 4 |
| 7 | 1000 | 512 | 계수 | 방향 벡터 스케일(dirs 성분 ±1000/±707/±923/±382 = cos/sin×1000, 16방향 22.5° 간격) · 513~514 `dx*preferred_range/1000`(43710·43714 sdiv) | 4 |
| 8 | 707 | 512 | 산출값 | 45° 방향 성분(cos45°×1000) — dirs[4..8] | 4 |
| 9 | 923 | 512 | 산출값 | 22.5° 방향 성분(cos22.5°×1000 ≈ 924 → 923 사용) — dirs[8..16] | 4 |
| 10 | 382 | 512 | 산출값 | 67.5° 방향 성분(sin22.5°×1000 ≈ 383 → 382 사용) — dirs[8..16] | 4 |
| 11 | 16 | 512 | 임계 | dirs 원소 수(IntoIter<_,16> · 43678 `icmp eq %146, 16` 루프 종료) | 4 |
| 12 | 0 | 516 | 태그 | 링 후보의 source_bonus = 0(43878 마지막 인자) · 537 후보 없음 → None 태그 0(43826) | 4 |
| 13 | 7 | 523 | 임계 | 격자 루프 `0..7` 상한(43733·43839 `icmp ult, 7`) — 7×7 셀 | 4 |
| 14 | -3 | 524 | 계수 | xi = cx - 3 + dx · yi = cy - 3 + dy(43666~43667 add -3) — 중심 ±3셀 | 4 |
| 15 | 29 | 527 | 임계 | `xi > 29 \|\| yi > 29`(43747·43856) → 건너뜀 — 격자 30×30(0..=29). `(xi\|yi) < 0`(43853~43854) 도 건너뜀 | 4 |
| 16 | 32000 | 531 | 미상 | 셀 크기(좌표 변환): x = xi*32000 + 16000(43749·43866) | 4 |
| 17 | 16000 | 531 | 미상 | 셀 중심 오프셋(좌표 변환)(43750·43867) | 4 |
| 18 | 10 | 532 | 미상 | 격자 후보의 source_bonus = 10(43869 마지막 인자) | 4 |
| 19 | 8 | 500 | 미상 | bumpalo Vec 빈 상태의 dangling ptr(= align, 43524 `inttoptr 8`) — 판정값 아님·라이브러리 상수 | 4 |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 장거리 판정 임계 | lane_minion.rs:495 (m11.ll:43493) | 69999 | 이 값 초과 사거리 챔프는 선호 사거리 = 사거리-14000(거의 최대 사거리 유지), 이하이면 65%. 내리면 더 많은 챔프가 '최대 사거리 근처' 포지셔닝 | 4 | 기존 |
| 1 | 장거리 선호 사거리 여유 | lane_minion.rs:496 (m11.ll:43512 add -14000) | 14000 | 올리면 장거리 챔프가 목표에 더 가까이 서는 링 후보를 만든다(≈0.44셀 당 14000) | 4 | 기존 |
| 2 | 단거리 선호 사거리 비율(%) | lane_minion.rs:498 (m11.ll:43505) | 65 | 올리면(예 80) 미니언에서 더 멀리, 내리면 더 붙어서 링 후보를 만든다 | 4 | 기존 |
| 3 | 현재 위치 유지 보너스 | lane_minion.rs:502 (m11.ll:43538 마지막 인자) | 35 | 올리면 제자리에서 계속 치는 경향↑(이동 감소). 격자(10)·링(0)보다 커서 동점 규칙(뒤가 우선)을 이 가산이 상쇄 | 4 | 기존 |
| 4 | 링 후보 생성 거리 한계 여유 | lane_minion.rs:505 (m11.ll:43551) | 48000 | 올리면 더 먼 목표에도 목표 주변 링 후보를 만든다(접근 포지셔닝↑) | 4 | 기존 |
| 5 | 격자 후보 보너스 | lane_minion.rs:532 (m11.ll:43869 마지막 인자) | 10 | 올리면 positioning_score 격자 셀(포지션 평가 중심 주변)이 링 후보보다 우대 → 링(목표 주변 정밀 좌표)보다 셀 중심 이동이 늘어난다 | 4 | 기존 |
| 6 | 격자 반경(±3 · 0..7) | lane_minion.rs:523~524 (m11.ll:43666~43667·43733) | 7 | 루프 상한 7 과 오프셋 -3 을 같이 바꿔야 한다. 넓히면 후보 수(최대 49→N²)·push_candidate 호출 수가 제곱으로 증가 | 4 | 기존 |

<details><summary>`callees` 피호출자 8건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | attack_range | game_ai::SmallActionLaneMinionPosition::attack_range | in:game_ai | fn(&game_core::Entity, &game_core::Entity) -> std::option::Option<u64> | game-ai\src\small_action\lane_minion.rs:354 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 2 | choose_goal | game_ai::SmallActionLaneMinionPosition::choose_goal | in:game_ai | fn(&game_ai::SmallActionLaneMinionPosition, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PositioningScoreData, game_core::LineType) -> std::option::Option<(u64, u64, i64)> | game-ai\src\small_action\lane_minion.rs:493 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 3 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | push_candidate | game_ai::SmallActionLaneMinionPosition::push_candidate | in:game_ai | fn(&mut bumpalo::collections::vec::Vec< (u64, u64, i64)>, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose, u64, u64, i64, game_core::LineType, i64) | game-ai\src\small_action\lane_minion.rs:439 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 6 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 7 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 5개**: `compare`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `llvm.assume`, `llvm.memcpy.p0.p0.i64`, `llvm.memset.p0.i64`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m11.ll:46260, m11.ll:54494) · **형제 20개** (SmallActionLaneMinionPosition)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionLaneMinionPosition as std::clone::Clone>::clone | pub | game-ai\src\small_action\lane_minion.rs:8 | True | fn(&game_ai::SmallActionLaneMinionPosition) -> game_ai::SmallActionLaneMinionPosition |
| 1 | <game_ai::SmallActionLaneMinionPosition as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\lane_minion.rs:8 | True | fn(&game_ai::SmallActionLaneMinionPosition, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionLaneMinionPosition::new | pub | game-ai\src\small_action\lane_minion.rs:139 | False | fn(&game_core::OperationData, usize, usize, i64, game_ai::PositionEvalPurpose) -> game_ai::SmallActionLaneMinionPosition |
| 3 | game_ai::SmallActionLaneMinionPosition::should_suppress_direct_minion_attack | pub | game-ai\src\small_action\lane_minion.rs:152 | False | fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::Entity) -> bool |
| 4 | game_ai::SmallActionLaneMinionPosition::has_current_explicit_minion_action | in:game_ai | game-ai\src\small_action\lane_minion.rs:187 | False | fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::Entity) -> bool |
| 5 | game_ai::SmallActionLaneMinionPosition::can_use_minion_action_now | in:game_ai | game-ai\src\small_action\lane_minion.rs:208 | False | fn(bool, std::option::Option<&game_core::Effect>, &game_core::Entity, &game_core::Entity) -> bool |
| 6 | game_ai::SmallActionLaneMinionPosition::direct_attack_stance | in:game_ai | game-ai\src\small_action\lane_minion.rs:212 | False | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity, u64) -> std::option::Option<(u64, u64)> |
| 7 | game_ai::SmallActionLaneMinionPosition::has_safe_minion_attack_stance | in:game_ai | game-ai\src\small_action\lane_minion.rs:226 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, game_core::LineType, u64) -> bool |
| 8 | game_ai::SmallActionLaneMinionPosition::is_safe_minion_attack_stance | in:game_ai | game-ai\src\small_action\lane_minion.rs:272 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, game_core::LineType, u64, u64, u64) -> bool |
| 9 | game_ai::SmallActionLaneMinionPosition::target_score | in:game_ai | game-ai\src\small_action\lane_minion.rs:294 | False | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Effect, &game_core::Entity, std::option::Option<&game_ai::MinionWaveSnapshot>, &mut rand::rngs::std::StdRng) -> std::option::Option<i64> |
| 10 | game_ai::SmallActionLaneMinionPosition::attack_range | in:game_ai | game-ai\src\small_action\lane_minion.rs:354 | False | fn(&game_core::Entity, &game_core::Entity) -> std::option::Option<u64> |
| 11 | game_ai::SmallActionLaneMinionPosition::local_position_score | in:game_ai | game-ai\src\small_action\lane_minion.rs:359 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> i64 |
| 12 | game_ai::SmallActionLaneMinionPosition::lane_stance_risk | in:game_ai | game-ai\src\small_action\lane_minion.rs:380 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64, u64) -> std::option::Option<i64> |
| 13 | game_ai::SmallActionLaneMinionPosition::push_candidate | in:game_ai | game-ai\src\small_action\lane_minion.rs:439 | False | fn(&mut bumpalo::collections::vec::Vec< (u64, u64, i64)>, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose, u64, u64, i64, game_core::LineType, i64) |
| 14 | game_ai::SmallActionLaneMinionPosition::choose_goal | in:game_ai | game-ai\src\small_action\lane_minion.rs:493 | False | fn(&game_ai::SmallActionLaneMinionPosition, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PositioningScoreData, game_core::LineType) -> std::option::Option<(u64, u64, i64)> |
| 15 | game_ai::SmallActionLaneMinionPosition::get_input | in:game_ai | game-ai\src\small_action\lane_minion.rs:540 | False | fn(&mut game_ai::SmallActionLaneMinionPosition, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> |
| 16 | game_ai::SmallActionLaneMinionPosition::merge | in:game_ai | game-ai\src\small_action\lane_minion.rs:635 | False | fn(&mut game_ai::SmallActionLaneMinionPosition, usize, game_ai::SmallActionLaneMinionPosition) |
| 17 | game_ai::SmallActionLaneMinionPosition::get_action | in:game_ai | game-ai\src\small_action\lane_minion.rs:652 | True | fn(&game_ai::SmallActionLaneMinionPosition) -> game_core::SmallAction |
| 18 | game_ai::SmallActionLaneMinionPosition::is_end | in:game_ai | game-ai\src\small_action\lane_minion.rs:656 | False | fn(&game_ai::SmallActionLaneMinionPosition, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 19 | game_ai::SmallActionLaneMinionPosition::near_move_complete | in:game_ai | game-ai\src\small_action\lane_minion.rs:685 | False | fn(&game_ai::SmallActionLaneMinionPosition, &game_core::Entity) -> bool |

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | ★exe 정체(지시 ⚠ 항목): 0xe25450(952B) 은 choose_goal 본체가 **아니다**. 근거 ①argscan 0xe25450 = 레지스터 4 + 스택 13 = **17 인자, i8 두 개가 12·16번째**(movzx byte) ↔ IR push_candidate 는 16 인자에 i8 이 11·15번째(%10 purpose · %14 line)로 위치 일치(exe 쪽 promotion 1개 추가로 +1 밀림) · choose_goal 은 10 인자(i8 는 3·10번째)라 불일치 ②fnprobe 0xe25450 의 호출자 = 0xe248f0(1,731B) 안 **3곳** ↔ IR choose_goal 이 push_candidate 를 **3회** invoke(43538·43869·43878) ③0xe248f0 의 호출자 2곳(0xe26c40·0xe2a830) ↔ IR choose_goal 호출 사이트 2곳(get_input m11.ll:46260 · lane_minion_position_action 54494) ④0xe248f0 상수 0x3e8(1000)·0x3e80(16000)·0x7d00(32000)·0xbb80(48000)·0x1116f(69999)·0x17e(382)·0x2c3(707)·0x39b(923) = choose_goal 상수 전부. ⟹ **0xe25450 = push_candidate**(추정 강 · Ghidra 미사용) · **0xe248f0 = choose_goal**(추정 강). '후보 클로저' 가설은 기각: max_by_key 의 key 클로저(choose_goal::{closure#0})는 별도 define 없이 fold(m12.ll:34778, 89줄)에 인라인돼 있고 fold 는 인자 4개라 17-인자 exe 와 무관. 검증 방법: 0xe25450 진입부 프로브의 리턴 주소가 0xe248f0 안 3곳인지 / 0xe248f0 에 판 프로브를 달아 get_input 경로에서 발화하는지 | 4 |  |
| 1 | 미탐색 | push_candidate(m11.ll:44238~44467) 본문은 이 명세 범위 밖 — 후보 채점식(position_score_at_position · is_enemy_well_danger · is_unnecessary_enemy_tower_position · is_near_line · lane_stance_risk · positioning_effective/skill_avoid_effective · source_bonus 합산 · 거절 조건)은 별도 명세 필요. 계약: (candidates:&mut bumpalo Vec<(u64,u64,i64)>(32B), version, player, data, champ, target.x:i64, target.y:i64, positioning_score, x, y, position_eval_purpose:i8(0..13), attack_range, preferred_range, target_score:i64, line:i8, source_bonus:i64(0..36)) -> () · Vec 에 push 하는 것이 유일한 출력 | 4 |  |
| 2 | 미탐색 | IR `&self` 가 ArgumentPromotion 으로 (goal_score i64 %1, position_eval_purpose i8 %2) 두 값으로 쪼개졌다 — 호출자 get_input(46257~46260 `self+32`·`self+120`) 으로 확인. 두 번째 호출자 lane_minion_position_action(54494) 은 `%14+16` 등 다른 지역 구조체에서 넘기며 미열람(범위 밖). exe 에서는 다르게 promotion 될 수 있어(0xe248f0 argscan 은 스택 인자 0·xmm0 읽음) sweep 편입 전 argscan 대조 필요 | 4 |  |
| 3 | 표기 불가 | 527 줄 `xi<0\|\|yi<0\|\|xi>29\|\|yi>29` 의 한 줄 안 순서는 column 부재로 표기 불가(IR 은 (xi\|yi)<0 · xi>29 · yi>29 를 or 로 합침) — 전부 순수 비교라 동작 무관 | 4 |  |
| 4 | 미탐색 | dirs 순서(1000,0)→(-1000,0)→(0,1000)→(0,-1000)→… 는 IR store 순서(43590~43650)에서 복원 — 동점 시 '뒤가 우선' 규칙 때문에 링 안에서는 마지막 방향(-382,-923) 쪽이 유리하나, 실제로 동점이 나는지는 push_candidate 채점(미열람)에 달림 | 4 |  |
| 5 | 미탐색 | 콜리 계약만: Effect::range_adjust(&Effect,&Entity caster,&Entity target)->u64 · Entity::distance(&,&)->u64 · Game::adjust_position(&MapDef(28112B), &GameSetting(5432B), x:u64, y:u64)->(u64,u64)(m11.ll:60774 declare · define _gcbc g15.ll:72245) — 맵 경계/장애물 보정으로 추정(미열람) | 4 |  |
| 6 | 미탐색 | bumpalo Vec 레이아웃(+0 ptr · +8 bump · +0x10 cap · +0x18 len)은 IR 사용 패턴(43524~43530 초기화 · 43754~43756 len 읽기)에서 읽은 것 — tcxdict 는 bumpalo 내부를 안 준다 | 3 |  |
| 7 | 미탐색 | `_docs\game_ai.txt` 에 lane_minion/choose_goal 관련 개발자 주석 0건 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

