---

### `261` kite_reposition_point — 가장 가까운 적(nearest) 주위 16방향 × (attack_range−8000) 거리의 후보점 중 position risk 최소(동률이면 '적에서 멀어지는 방향' 정렬 최대)를 고르되, v48 캐스트 라인 위가 아닌 후보가 하나라도 있으면 그쪽 최선을 우선 반환

| 항목 | 값 |
|---|---|
| id | `battle__kite_reposition_point` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6battle21kite_reposition_point` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\battle.rs:1217` |
| IR | `m02.ll` 74099~74527행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::battle::kite_reposition_point` · **in:game_ai::plan_legacy::sub_plan::battle** |
| 계층 | 기타 |
| exe | `cd5ee0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_core::Entity, &game_core::Entity, u64) -> (u64, u64)
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[261]/sig/tls/<키>`)**

없음 — 본문에 LocalKey/call_once fn-포인터 상수 참조 0. ⚠콜리 position_score_at_position → position_eval_at(exe 0xd84db0 · rvaname 인스턴스 = LocalKey<RefCell<…>> 즉 TLS 메모)이 TLS 를 소비/작성하지만 그것은 콜리 계약(이 함수 자신은 TLS 접점 없음)

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize (i64 %0) | 본문 분기 없음. position_score_at_position 의 1번째 인자로만 전달(m02.ll:74230·74362) | 4 |
| 1 | 2 | player | &PlayerState(2528B) %1 | define 줄 속성: readonly · captures(address, read_provenance). 본문 직접 필드 읽기 없음 — position_score_at_position·v48_on_cast_line 에 그대로 전달 | 4 |
| 2 | 3 | data | &OperationData(24B) %2 | define 줄 속성: readonly. +0x8 context(&GameContext) 만 읽음(m02.ll:74172) → context+0x20 map · +0x8 setting 을 adjust_position 인자로. 콜리 둘에도 그대로 전달 | 4 |
| 3 | 4 | parameter | &ScoreParameter(5384B) %3 | define 줄 속성: readonly. +0x9f0 positioning_score(PositioningScoreData 2760B) 의 주소만 만들어(m02.ll:74176) position_score_at_position 4번째 인자로 전달. 다른 필드 읽기 없음 | 4 |
| 4 | 5 | champ | &Entity(1728B) %4 | define 줄 속성: readonly · captures(none). 읽는 필드 = x(0x660)·y(0x668)·stat_buff_cached.radius_mult(0x470)·radius(0x680) | 4 |
| 5 | 6 | nearest | &Entity → 스칼라 승격 i64 %5(=nearest.x, Entity+0x660) · i64 %6(=nearest.y, Entity+0x668) | DI arg 6 이름 nearest(!77530, battle.rs:1218). LTO ArgumentPromotion 으로 IR define 엔 &Entity 대신 x·y 두 i64 가 들어온다 — 호출자 m02.ll:29703/29707 `gep %688, 1632/1640` → 29948/29949 load → 29950 invoke 로 확인. exe 호출부 0xcbe020 도 `movups xmm0,[rbx+0x660]` → `[rsp+0x20]` 로 두 i64 를 스택에 놓는다(argscan --caller) | 4 |
| 6 | 7 | attack_range | u64 (i64 %7) | dist = attack_range.saturating_sub(8000) (L1219, m02.ll:74117 usub.sat). 8000 미만이면 dist=0 → 모든 후보점이 nearest 위치(adjust_position 보정 후)가 된다 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// battle.rs:1217 kite_reposition_point(version, player, data, parameter, champ, nearest, attack_range) -> (u64,u64)
dist = attack_range.saturating_sub(8000)                                   // L1219 (74117)
bdx = champ.x − nearest.x ; bdy = champ.y − nearest.y                       // L1227~1228 (i64 wrapping sub)
(away_x, away_y) = if bdx²+bdy² < 4000000 (=2000², 부호 있는 비교) { (1, 0) } else { (bdx, bdy) }   // L1229 (74132~74134)
best: Option<(px,py,risk,align)> = None ; best_clear: Option<(px,py,risk,align)> = None            // L1230 · L1233
DIRS16 = 스택 복사된 방향표(m02.ll:275 @anon.…267, 256B) = [(1000,0),(924,383),(707,707),(383,924),(0,1000),(−383,924),(−707,707),(−924,383),(−1000,0),(−924,−383),(−707,−707),(−383,−924),(0,−1000),(383,−924),(707,−707),(924,−383)]  // 22.5° 단계, 크기 ≈1000
for (dx,dy) in DIRS16 {                                                    // L1234 (IR: 방향 0 은 루프 밖으로 벗겨짐 74212~74294, 방향 1..15 는 블록 %56 루프)
  len = max(isqrt(dx²+dy²), 1)                                             // L1235 (isqrt → smax 1)
  px = nearest.x + (dx*dist)/len ; py = nearest.y + (dy*dist)/len          // L1236~1237 (sdiv, 부호 있음)
  (px,py) = Game::adjust_position(data.context.map, data.context.setting, px, py)   // L1238 — 맵 경계/이동가능 보정(콜리 계약)
  risk = position_score_at_position(version, player, data, &parameter.positioning_score, px, py, General).risk   // L1239 (sret 56B 중 +0 만)
  align = dx*away_x + dy*away_y                                            // L1241 (적에서 멀어지는 방향과의 내적 · i64)
  better = best.is_none_or(|(_,_,r,a)| risk < r || (risk == r && align > a))     // L1242 (closure$0 인라인 · slt/sgt 부호 있음)
  if better { best = Some((px,py,risk,align)) }                            // L1243
  if !v48_on_cast_line(player, data, champ.radius(), px, py) {             // L1246 — champ.radius() = radius_mult==0 ? radius : radius*(100+radius_mult)/100 (entity.rs:1512 인라인)
    better_c = best_clear.is_none_or(|(_,_,r,a)| risk < r || (risk == r && align > a))   // L1247 (closure$1)
    if better_c { best_clear = Some((px,py,risk,align)) }                  // L1248
  }
}
(bx,by) = best_clear.unwrap_or(best).(px,py)  // L1253 (best 는 방향 0 에서 무조건 Some 이므로 unwrap 실패 경로 없음)
return (bx,by)                                                             // L1255
// 극성 메모: 후보 우선순위 = ①캐스트 라인 밖(v48 false) ②risk 최소 ③align 최대(적에게서 멀어지는 방향). 방향 0 (dx=1000,dy=0) 이 첫 best 이므로 전 후보 risk·align 동률이면 (nearest.x+dist, nearest.y) 가 반환된다.
// rnd(StdRng) 인자 없음 · gen_range 호출 0.
```

**`mem` 메모리 접근 11건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | Entity(champ %4) | 0x660 | x | r | L1227 bdx = champ.x − nearest.x (m02.ll:74119~74121) | 4 | OK |
| 1 | Entity(champ %4) | 0x668 | y | r | L1228 bdy = champ.y − nearest.y (74124~74126) | 4 | OK |
| 2 | Entity(champ %4) | 0x470 | stat_buff_cached.radius_mult | r | i32. L1246 인라인 Entity::radius(entity.rs:1509, 본문 1512): mult==0 → radius 그대로, 아니면 radius*(100+mult)/100 (74177~74185·74263 select) | 4 | OK |
| 3 | Entity(champ %4) | 0x680 | radius | r | usize. 위 radius() 의 밑값 → v48_on_cast_line 3번째 인자 champ_r (74264·74448) | 4 | OK |
| 4 | Entity(nearest) | 0x660 | x → 승격 %5 | r | 호출자 base_positioning 이 load 해 i64 로 전달(m02.ll:29703·29948). 후보점 원점 · L1236 px = nearest.x + … | 4 | OK |
| 5 | Entity(nearest) | 0x668 | y → 승격 %6 | r | 29707·29949. L1237 py = nearest.y + … | 4 | OK |
| 6 | OperationData(data %2) | 0x8 | context | r | &GameContext (74172~74173) | 4 | OK |
| 7 | GameContext | 0x20 | map | r | &MapDef → adjust_position 1번째 인자 (74174·74222·74354) | 4 | OK |
| 8 | GameContext | 0x8 | setting | r | &GameSetting → adjust_position 2번째 인자 (74175·74223·74355) | 4 | OK |
| 9 | ScoreParameter(parameter %3) | 0x9f0 | positioning_score | r | PositioningScoreData(2760B) 의 주소만 취해 position_score_at_position 4번째 인자로 (74176) | 4 | OK |
| 10 | PositioningScore(sret 지역 %9, 56B) | 0x0 | risk | r | i64. 콜리 sret 56B 중 이 8B 만 읽는다(74231·74363). tower_risk(0x8)·gain(0x10) 등 나머지 48B 는 미사용 | 4 | OK |

**`consts` 상수 9건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 8000 | 1219 | 계수 | dist = attack_range.saturating_sub(8000) — 후보점의 nearest 로부터의 거리(사거리에서 8000 안쪽). usub.sat 이라 8000 미만이면 0 | 4 |
| 1 | 4000000 | 1229 | 임계 | = 2000². bdx²+bdy² < 4000000 (icmp slt, 부호 있음) 이면 nearest 와 사실상 겹친 것으로 보고 away 벡터를 (1,0) 으로 고정. 아니면 away=(bdx,bdy) | 4 |
| 2 | 1 | 1229 | 인덱스 | 겹침 시 away_x=1 (select %23) / L1235 len=max(isqrt,1) 0-나눗셈 방지 바닥(smax 1) | 4 |
| 3 | 0 | 1229 | 태그 | 겹침 시 away_y=0 (select %22) | 4 |
| 4 | 1000 | 1236 | 계수 | 방향 0 벗김: 방향표 첫 원소 (dx,dy)=(1000,0) 이 상수 전파돼 `mul %11, 1000`(px) · L1241 `mul %23, 1000`(align). 방향표 원소 크기 = 1000(단위벡터×1000) | 4 |
| 5 | 1000000 | 1235 | 미상 | 방향 0 의 dx²+dy² = 1000² 을 isqrt 에 상수로 전달(74212). 결과 1000 | 4 |
| 6 | 16 | 1234 | 태그 | 방향표 원소 수 16 (루프 종료 `icmp eq %67, 16` 74525; 스택 %10+16 에 256B memcpy = 16×{i64,i64}). 22.5° 간격 전방위 | 4 |
| 7 | 100 | 1246 | 계수 | 인라인 Entity::radius: radius*(100+radius_mult)/100 (%36 add 100 · %38 udiv 100) — 퍼센트 배율 | 4 |
| 8 | 2 | 1239 | 센티널 | position_score_at_position 의 purpose 인자 i8 2 = PositionEvalPurpose 메모리태그 2 = General(idx 0, 니치 niche_start=2 · tcxdict --enum) | 3 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 카이팅 후보 반경 마진 | battle.rs:1219 (m02.ll:74117) | 8000 | dist = attack_range−8000. 올리면 nearest 에 더 가까운 지점(공격적)으로, 내리면 사거리 끝에 가까운 지점(안전)으로 재배치. attack_range 이상이면 dist=0 → nearest 위치 자체 | 4 | 기존 |
| 1 | away 벡터 겹침 판정 반경² | battle.rs:1229 (74132) | 4000000 | champ↔nearest 거리 2000 미만이면 '멀어지는 방향' 정렬이 +x 축으로 고정된다. 올리면 더 넓은 범위에서 정렬 기준이 무의미해짐 | 4 | 기존 |
| 2 | 방향 후보 수·간격 | battle.rs:1234 방향표 = m02.ll:275 @anon.94acafa22d01e083ca1cc62f01598c8f.267 (256B 상수) | 16 | 16방향 22.5°. 표를 바꾸면 후보 위치가 바뀜(표 원소 크기 1000 은 len 으로 나눠져 정규화되므로 스케일 자체는 결과에 무관, 방향만 의미) | 4 | 기존 |
| 3 | position 평가 목적 태그 | battle.rs:1239 (74230·74362 i8 2) | 2 | PositionEvalPurpose::General. 다른 태그(Trace 7·Around 5 등)로 바꾸면 콜리 position_eval_at 의 목적별 가중이 달라진다(콜리 계약) | 4 | 기존 |
| 4 | v48 캐스트 라인 회피 반경 | battle.rs:1246 (74263 champ.radius()) | champ.radius×(100+radius_mult)/100 | v48_on_cast_line 에 넘기는 반경. 키우면 더 많은 후보가 '라인 위'로 판정돼 best_clear 후보가 줄고 best(라인 무시) 로 폴백하는 빈도가 는다 | 4 | 기존 |

<details><summary>`callees` 피호출자 6건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | isqrt | game_core::utils::isqrt | pub | fn(i64) -> i64 | game-core\src\utils.rs:85 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | kite_reposition_point | game_ai::plan_legacy::sub_plan::battle::kite_reposition_point | in:game_ai::plan_legacy::sub_plan::battle | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_core::Entity, &game_core::Entity, u64) -> (u64, u64) | game-ai\src\plan_legacy\sub_plan\battle.rs:1217 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 3 | position_score_at_position | game_ai::position_score_at_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1192 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 5 | v48_on_cast_line | game_ai::plan_legacy::sub_plan::battle::v48_on_cast_line | in:game_ai::plan_legacy::sub_plan::battle | fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64) -> bool | game-ai\src\plan_legacy\sub_plan\battle.rs:1057 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 4개**: `is_none_or`, `llvm.memcpy.p0.p0.i64`, `llvm.smax.i64`, `llvm.usub.sat.i64`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m02.ll:29950) · **형제 0개** 

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | exe ABI: exe 0xcd5ee0 는 인자 7(레지스터 rcx·rdx·r8·r9 + 스택 3 = nearest.x·nearest.y·attack_range) 인데 IR define 은 8 — version/player/data/parameter/champ 중 어느 하나가 exe 에서 소거됐는지(DeadArgElim/IPSCCP) 미확정. 호출부 0xcbe020 은 rcx=[rbp+0x538]·rdx=[rbp+0x620]·r8=[rbp+0x628]·r9=[rbp+0x588] 을 넘김. 확정하려면 ghidra-re 로 호출자 0xcbbdb0 의 4 레지스터 출처 대응(sweep 편입 전 필수, EXE_ABI 판정) | 4 |  |
| 1 | 미탐색 | 방향표 원소값(924·707·383)은 body 명령이 아니라 전역 상수 @anon.…267(m02.ll:275)에 있어 constants 에 넣지 않았다(C1 대조 불가) — knobs 에 위치·값 기재 | 4 |  |
| 2 | 표기 불가 | L1229 의 `bdx²+bdy² < 4000000` 은 부호 있는 비교(slt)다 — u64 좌표차 제곱이 2^63 을 넘는 경우는 실전 좌표 범위(≤ 30셀×32000 ≈ 960000)에서 발생하지 않으므로 동작상 무해하나 소스가 i64 캐스팅인지 usize 인지는 표기 불가(외연 동일) | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | adjust_position(map, setting, x, y) 의 보정 규칙(경계 클램프인지 이동가능 셀 스냅인지)은 game_core 콜리 계약(g15.ll:72245 define)이라 본 명세 범위 밖 — 반환 (x,y) 를 그대로 후보점으로 쓴다는 사실만 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

