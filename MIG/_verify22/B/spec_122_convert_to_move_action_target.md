---

### `122` convert_to_move_action_target — 이동형 스킬의 목표 (x,y) 를 캐스팅 타입에 맞는 InputTarget 으로 변환 — Position 이면 사거리 안으로 클램프한 Pos, Direction 이면 방향 벡터 Dir(×부호)

| 항목 | 값 |
|---|---|
| id | `abstract_input__convert_to_move_action_target` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai14abstract_input29convert_to_move_action_target` |
| 소스 | `game-ai\src\abstract_input.rs:1440` |
| IR | `m04.ll` 43140~43269행 |
| 경로·가시성 | `game_ai::convert_to_move_action_target` · **pub** |
| 계층 | 입력 생성 |
| exe | `d34470` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::GameSetting, &game_core::MapDef, &game_core::Entity, &game_core::Effect, u64, u64, bool) -> game_core::InputTarget
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut InputTarget (24B) noalias writable writeonly | 반환 슬롯. 아래 returns 참조 | 4 |
| 1 | 1 | setting | &GameSetting (5432B) noalias readonly | adjust_position 에 전달만(L43252). 본문 직접 read 없음 | 4 |
| 2 | 2 | map | &MapDef (28112B) noalias readonly | adjust_position 에 전달만 | 4 |
| 3 | 3 | caster | &Entity (1728B) noalias readonly | x/y(0x660/0x668)·level(0x5c8)·stat_buff_cached.range(0x438) | 4 |
| 4 | 4 | effect | &Effect (56B) noalias readonly | casting(0x30)·range(0x10)·growth_range(0x18)·ty(Arc<dyn EffectType>, 0x0/0x8) | 4 |
| 5 | 5 | x | u64 (i64 %5) | 목표 x | 4 |
| 6 | 6 | y | u64 (i64 %6) | 목표 y | 4 |
| 7 | 7 | max_range | bool (i1 zeroext %7) | true 면 거리와 무관하게 항상 최대 사거리 지점으로 투사 (Position 분기에서만 의미) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn convert_to_move_action_target(setting, map, caster, effect, x, y, max_range) -> InputTarget {   // abstract_input.rs:1440
  match effect.casting {                                                                          // :1441 L43154~43159 (Effect+0x30 i32)
    CastingType::Position /*1*/ => {
      // :1443  실효 사거리 = caster.stat_buff_cached.range + effect.range + effect.growth_range*(level-1)   (Effect::range 인라인 effect.rs:26)
      let range = caster.stat_buff_cached.range + effect.range + (caster.level - 1) * effect.growth_range;   // L43166~43177
      // :1444  목표가 사거리 밖이거나 max_range 강제면 사거리 끝점으로 투사
      if max_range || distance_sq(x, y, caster.x, caster.y) > range*range {                          // L43179~43203 (`or i1 %7, %41` 비단락 — 둘 다 계산됨; 소스 순서 표기 불가)
        let dx = x - caster.x; let dy = y - caster.y;                                                // :1449 (wrapping sub, 부호 있음)
        let sz = isqrt(dx*dx + dy*dy);                                                              // :1449 L43234~43237
        let nx = caster.x + range*dx / max(sz,1);                                                    // :1451 L43239~43244
        let ny = caster.y + dy*range / max(sz,1);                                                    // :1452 L43246~43250
        let (nx, ny) = Game::adjust_position(map, setting, nx, ny);                                 // :1453 L43252 (맵 경계 클램프·통행 보정, g15.ll:72245)
        InputTarget::Pos { x: nx, y: ny }                                                            // tag 2
      } else {
        InputTarget::Pos { x, y }                                                                    // :1444 else — 그대로 (L43260~43262 phi [%5,%12],[%6,%12],[2,%12])
      }
    }
    CastingType::Direction /*2*/ => {
      let sign = effect.ty.expected_move_input_dir_sign();                                          // :1459 L43208~43219 vtable+0xf0 (i64, 돌진형=+1 / 후퇴형=−1 로 추정)
      InputTarget::Dir { dir_x: (x - caster.x) * sign, dir_y: (y - caster.y) * sign }               // :1461~1462 L43221~43228, tag 1
    }
    _ => unreachable!()                                                                             // :1465 L43162 core::panicking::panic
  }
}   // :1467
```

**`mem` 메모리 접근 12건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | Effect | 0x30 | casting (CastingType, i32) | r | L43154~43159 switch 1=Position 2=Direction (tcxdict --enum CastingType), 그 외(0 Targeting/3 None) → panic | 3 | OK |  |
| 1 | Effect | 0x10 | range | r | L43166~43167 Effect::range 인라인(effect.rs:26) | 4 | OK |  |
| 2 | Effect | 0x18 | growth_range | r | L43168~43169 ×(level−1) | 4 | OK |  |
| 3 | Effect | 0x0 | ty (Arc<dyn EffectType>) data_ptr → ArcInner | r | L43208 Direction 분기. ArcInner 내부 오프셋은 vtable align 필드(+16)로 계산(L43211~43216) | 4 | OK |  |
| 4 | Effect | 0x8 | ty vtable_ptr | r | L43209~43210; 슬롯 +0xf0(240) = EffectType::expected_move_input_dir_sign (divtable EffectType 0xf0, g02.ll) L43217~43219 | 3 | OK |  |
| 5 | Entity | 0x5c8 | level | r | L43170~43172 `level - 1` (effect.rs:26) | 4 | OK |  |
| 6 | Entity | 0x438 | stat_buff_cached.range | r | L43174~43175 사거리 기본항 (effect.rs:26) | 4 | OK |  |
| 7 | Entity | 0x660 | x | r | L43179~43180 (Position) · L43221~43222 (Direction) | 4 | OK |  |
| 8 | Entity | 0x668 | y | r | L43183~43184 · L43225~43226 | 4 | OK |  |
| 9 | InputTarget(sret) | 0x0 | tag (i32) | w | L43262 phi + L43267 store i32 | 4 | OK | 1=Dir (Direction 분기) / 2=Pos (Position 분기 두 경로) |
| 10 | InputTarget(sret) | 0x8 | Pos.x \| Dir.dir_x | w | L43260 phi + L43263~43264 | 4 | OK | Pos: 클램프된 nx 또는 원래 x / Dir: (x − caster.x)·sign |
| 11 | InputTarget(sret) | 0x10 | Pos.y \| Dir.dir_y | w | L43261 phi + L43265~43266 | 4 | OK | Pos: ny 또는 y / Dir: (y − caster.y)·sign |

**`consts` 상수 3건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 1 | 1441 | 태그 | CastingType::Position 태그(switch L43157) — ★같은 값 1 이 L43262 phi 에서는 InputTarget::Dir 태그, L43172 에서는 `level - 1`(add −1), L43242 `max(sz,1)` 0나눗셈 방지 | 4 |
| 1 | 2 | 1441 | 태그 | CastingType::Direction 태그(switch L43158) — ★L43262 phi 에서는 InputTarget::Pos 태그 2 | 4 |
| 2 | -1 | 1443 | 계수 | `level - 1` 의 add −1 (L43172, effect.rs:26 성장 사거리 = growth_range × (level−1)) | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 사거리 밖 판정 = 제곱거리 > range² | abstract_input.rs:1444 (L43200~43201) | range² (동적) | '>' 를 '>=' 로 바꿔도 외연 차이는 정확히 경계 1점뿐. 사거리 자체는 Effect::range 항(range+growth·(lv−1)+stat range)이라 데이터 값 | 4 | 기존 |
| 1 | max_range 인자 | abstract_input.rs:1444 (%7) | bool | true 면 가까운 목표도 항상 사거리 끝까지 투사(돌진기 최대거리 사용) | 4 | 기존 |

<details><summary>`callees` 피호출자 8건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | convert_to_move_action_target | game_ai::convert_to_move_action_target | pub | fn(&game_core::GameSetting, &game_core::MapDef, &game_core::Entity, &game_core::Effect, u64, u64, bool) -> game_core::InputTarget | game-ai\src\abstract_input.rs:1440 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | expected_move_input_dir_sign | game_core::EffectType::expected_move_input_dir_sign | pub | fn(&Self/#0) -> i64 | game-core\src\simulation\effect\type.rs:343 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 5 | expected_move_input_dir_sign | <game_core::CombineEffect as game_core::EffectType>::expected_move_input_dir_sign | pub | fn(&game_core::CombineEffect) -> i64 | game-core\src\simulation\effect\type\combine.rs:112 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 6 | expected_move_input_dir_sign | <game_core::setting::champion::crossbowman::CrossbowmanFireEffect as game_core::EffectType>::expected_move_input_dir_sign | pub | fn(&game_core::setting::champion::crossbowman::CrossbowmanFireEffect) -> i64 | game-core\src\setting\champion\crossbowman.rs:369 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 7 | isqrt | game_core::utils::isqrt | pub | fn(i64) -> i64 | game-core\src\utils.rs:85 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 2개**: `llvm.smax.i64`, `panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 6곳** (m03.ll:138028, m03.ll:138235, m03.ll:138628, m04.ll:45974, m04.ll:46181, m04.ll:46576) · **형제 0개** 

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | `max_range \|\| dist_sq > range²` 의 소스 순서 — IR 은 `or i1 %7, %41` 로 비단락 평가(column 0 · MIR 없음) → 표기 불가, 동작은 동일 | 3 |  |
| 1 | 미탐색 | expected_move_input_dir_sign 의 반환 의미(+1/−1 추정) — EffectType 은 Arc<dyn> 런타임 vtable 이라 구현체별 값은 이 명세 범위 밖(divtable 은 슬롯 이름만 확정) | 3 |  |
| 2 | 미탐색 | adjust_position 의 정확한 보정 규칙(30×30 셀 통행 검사 + [0, setting.width/height] 클램프로 보임, g15.ll:72245~) — 콜리 명세 범위 밖 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | :1449 의 dx/dy 가 부호 있는 wrapping sub(L43189/L43193 `sub i64 %5, %26`)인데 :1444 의 distance_sq 는 abs_diff — 같은 줄 재사용이 아니라 별도 계산이라는 것만 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

