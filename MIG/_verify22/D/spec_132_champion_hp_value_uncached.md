---

### `132` champion_hp_value_uncached — 대상 챔피언의 'HP 가치' = (attack_value·공격력 순위배율 + util_value·유틸력 순위배율)/100 — 근처 챔피언(≤10명) 안에서의 상대 순위로 스케일

| 항목 | 값 |
|---|---|
| id | `utils__champion_hp_value_uncached` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai5utils26champion_hp_value_uncached` |
| 소스 | `game-ai\src\utils.rs:929` |
| IR | `m04.ll` 52225~53105행 |
| 경로·가시성 | `game_ai::utils::champion_hp_value_uncached` · **in:game_ai::utils** |
| 계층 | 기타 |
| exe | `d390a0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::ScoreParameter, &game_ai::ChampionScoreParameter) -> i64
```

<details><summary>인자 2개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | parameter | &ScoreParameter (5384B) noalias readonly | player(0x918: 자기 ChampionScoreParameter)·near_allies(0x14b8)·near_enemies(0x14d8) 만 읽음 | 4 |
| 1 | 2 | target | &ChampionScoreParameter (216B) noalias readonly | 가치를 매길 대상. dbg 이름 target/p 둘 다 %1 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn champion_hp_value_uncached(parameter: &ScoreParameter, target: &ChampionScoreParameter) -> i64 {   // utils.rs:929~932
  let mut atk_list = [0i64; 10]; let mut util_list = [0i64; 10];                                    // :934~935 L52242~52246 memset
  let (mut n, mut same_dps, mut opp_dps) = (0, 0, 0);                                                 // :938~941 (same_dps 의 dbg 별칭 = ally_team_dps_sum)
  let me = &parameter.player;                                                                          // +0x918
  // :943~944  자기 자신
  atk_list[0] = me.attack_power; n = 1;                                                                // L52252~52254
  if me.team == target.team { same_dps += me.attack_power } else { opp_dps += me.attack_power }        // L52255~52261 (select 쌍)
  // :947~951  근처 아군
  for p in parameter.near_allies.iter() { if n >= 10 { break; }                                        // L52296~52336 (가드는 `n_old > 8` 로 접힘)
    atk_list[n] = p.attack_power;                                                                       // :949
    if p.team == target.team { same_dps += p.attack_power } else { opp_dps += p.attack_power }          // :950
    n += 1; }                                                                                           // :951
  // :953~957  근처 적군 (같은 처리)
  for p in parameter.near_enemies.iter() { if n >= 10 { break; } atk_list[n] = p.attack_power; …team 합산…; n += 1; }   // L52376~52418
  // :964~965  자기 유틸력 (자기 '편' DPS 합 = target 과 같은 팀이면 same_dps, 아니면 opp_dps)
  let ally_dps = if me.team == target.team { same_dps } else { opp_dps };                               // L52427
  util_list[0] = util_power_combined(me, ally_dps);                                                     // L52430~52442
  // :968~971  근처 아군 유틸력 (idx 1.., idx>=n 이면 중단 — 완전 언롤 L52459~52814)
  let mut idx = 1;
  for p in near_allies { if idx >= n { break; } let d = if p.team == target.team {same_dps} else {opp_dps}; util_list[idx] = util_power_combined(p, d); idx += 1; }
  // :974~978  근처 적군 유틸력 (idx>=n 중단, idx>=10 이면 bounds panic — 도달 불가)
  for p in near_enemies { if idx >= n { break; } … util_list[idx] = util_power_combined(p, d); idx += 1; }   // L52832~52880
  // :982~983  대상 자신의 공격력·유틸력 (대상 팀 DPS 합 = same_dps)
  let target_atk = target.attack_power;                                                                 // L52887~52888
  let target_util = util_power_combined(target, same_dps);                                              // L52890~52901
  // :985~986  근처 집단 안에서의 순위 배율
  let atk_scale = rank_scale(&atk_list[..n], target_atk);                                               // L52923~53067
  let util_scale = rank_scale(&util_list[..n], target_util);                                            // L53018~53088
  // :988  최종
  (target.attack_value * atk_scale + target.util_value * util_scale) / 100                              // L53094~53104
}   // :989

// util_power_combined(p, dps) -> i64   utils.rs:873~875 (인라인 4곳)
  p.util_power_base + p.cc_time_x_inv_cd * dps / 1000 + p.buff_inv_cd_count * dps / 10000

// rank_scale(values: &[i64], target: i64) -> i64   utils.rs:882~894 (인라인 2곳)
  let n = values.len(); if n < 2 { return 100; }                                                        // :882
  let (mut lower, mut equal) = (0, 0);
  for v in values { if v < target { lower += 1 } ; if v == target { equal += 1 } }                        // :885~886 (부호 있는 비교 slt/eq)
  let middle = lower + equal / 2;                                                                       // :889 (동률 절반을 아래로)
  let rank_x100 = (middle * 100 / (n - 1)).clamp(0, 100);                                               // :890
  if middle*100/(n-1) < 51 { rank_x100 + 50 } else { rank_x100 * 2 }                                    // :891~894 → 50..=100 | 102..=200

※ values 에는 target 자신도 들어 있을 수 있어(근처 목록에 target 포함 시) equal>=1 이 보통이고, middle 로 절반만 센다.
```

**`mem` 메모리 접근 16건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | ScoreParameter | 0x9d0 | player.attack_power (=0x918+0xb8) | r | L52252~52254 atk_list[0] (:943) | 4 | OK |
| 1 | ScoreParameter | 0x978 | player.team (=0x918+0x60) | r | L52255~52259 `== target.team` (:944, :964) | 4 | OK |
| 2 | ScoreParameter | 0x9d8 | player.util_power_base (=0x918+0xc0) | r | L52430~52431 util_power_combined 인라인 utils.rs:873 (:965) | 4 | OK |
| 3 | ScoreParameter | 0x9e0 | player.cc_time_x_inv_cd (=0x918+0xc8) | r | L52432~52436 `*ally_dps/1000` (utils.rs:874) | 4 | OK |
| 4 | ScoreParameter | 0x9e8 | player.buff_inv_cd_count (=0x918+0xd0) | r | L52437~52441 `*ally_dps/10000` (utils.rs:875) | 4 | OK |
| 5 | ScoreParameter | 0x14b8 | near_allies.buf.ptr (bumpalo Vec<ChampionScoreParameter>) | r | L52268~52269 (:947, :968) | 4 | OK |
| 6 | ScoreParameter | 0x14d0 | near_allies.len (=0x14b8+0x18) | r | L52271~52272 · 원소 stride 216B (L52286) | 4 | OK |
| 7 | ScoreParameter | 0x14d8 | near_enemies.buf.ptr | r | L52344~52345 (:953, :974) | 4 | OK |
| 8 | ScoreParameter | 0x14f0 | near_enemies.len (=0x14d8+0x18) | r | L52347~52348 | 4 | OK |
| 9 | ChampionScoreParameter | 0x60 | team | r | target: L52257~52258. 각 p: L52313~52315 / L52395~52397 / L52466~52468 … / L52850~52852 | 4 | OK |
| 10 | ChampionScoreParameter | 0xb8 | attack_power | r | 각 p: L52309~52310 / L52391~52392 → atk_list[n] ; target: L52887~52888 target_atk (:982) | 4 | OK |
| 11 | ChampionScoreParameter | 0xc0 | util_power_base | r | util_power_combined 항1 (utils.rs:873) L52472~52473 등 · target L52890~52891 | 4 | OK |
| 12 | ChampionScoreParameter | 0xc8 | cc_time_x_inv_cd | r | 항2 `*dps/1000` L52474~52477 · target L52892~52895 | 4 | OK |
| 13 | ChampionScoreParameter | 0xd0 | buff_inv_cd_count | r | 항3 `*dps/10000` L52479~52482 · target L52897~52900 | 4 | OK |
| 14 | ChampionScoreParameter | 0xa8 | attack_value (target) | r | L53094~53096 `* atk_scale` (:988) | 4 | OK |
| 15 | ChampionScoreParameter | 0xb0 | util_value (target) | r | L53097~53099 `* util_scale` (:988) | 4 | OK |

**`consts` 상수 13건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 1000 | 874 | 계수 | util_power_combined 항2 분모: cc_time_x_inv_cd * ally_dps / 1000 (L52435 등 — :965/:971/:977/:983 네 호출 지점에 인라인) | 4 |  |
| 1 | 10000 | 875 | 계수 | util_power_combined 항3 분모: buff_inv_cd_count * ally_dps / 10000 (L52440 등) | 4 |  |
| 2 | 10 | 934 | 길이 | atk_list/util_list 길이 10 (=자기 1 + 근처 최대 9) — L52883 panic_bounds_check len 10 (:977) · L52954 slice_index_fail len 10 (:985) · dbg values[8..+8]=10 | 4 |  |
| 3 | 8 | 947 | 임계 | 루프 상한 접힘: 저장 후 `n_old > 8`(L52334/L52416 `icmp samesign ugt %24, 8`) 이면 중단 = 소스 `n >= 10` 가드가 회전된 형태(folded). n 은 1에서 시작하므로 최대 index 9 | 4 |  |
| 4 | 9 | 953 | 임계 | 적 루프 진입 가드 `n > 9`(L52377) = 같은 `n >= 10` 가드(빈 루프 진입 전 검사). 판정값 아님(용량) | 4 |  |
| 5 | 2 | 882 | 임계 | rank_scale: `n < 2` 이면 순위 계산 없이 100 (L52923 icmp ult %79, 2 → L53091/53092 phi 100). 즉 비교 대상이 없으면 배율 1.0 | 4 |  |
| 6 | 11 | 985 | 임계 | `atk_list[..n]` 슬라이스 범위검사 n<=10 (L52929 `icmp ult %79, 11`). 도달 불가 panic | 4 |  |
| 7 | 100 | 890 | 계수 | rank_x100 = middle*100/(n−1) (L53059/L53071) · clamp 상한 100 (L53063/L53077) · :988 최종 /100 (L53101) · n<2 기본 배율 100 | 4 |  |
| 8 | 51 | 891 | 임계 | `rank_x100 < 51`(하위·중위) 이면 배율 = rank_x100+50 (50..100), 아니면 rank_x100*2 (102..200). L53064/L53079 — ⚠비교는 clamp 전 값(%366/%376)이지만 결과는 동일 | 4 |  |
| 9 | 50 | 892 | 계수 | 하위 배율 오프셋 `rank_x100 + 50` (L53065/L53087) | 4 |  |
| 10 | 1 | 894 | 계수 | 상위 배율 `rank_x100 * 2` 가 `shl nuw nsw i64 %368, 1`(L53066/L53083) 로 접힘. 또 `equal/2` 가 `lshr 1`(L53057/L53068), `n−1` 이 `add −1`(L53060) | 4 | 2 |
| 11 | -1 | 890 | 미상 | `n - 1` (L53060 add nsw −1) — 순위 분모(자기 제외 인원) | 4 |  |
| 12 | 0 | 938 | 임계 | 누적 초기값 n·same_dps·opp_dps=0 (dbg) · select 분기의 0 (팀 불일치면 상대 합에 0 가산, L52260~52261) · clamp 하한 0 | 4 |  |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 순위→배율 곡선 분기점 | utils.rs:891 (L53064/L53079) | 51 | rank 51% 이상이면 ×2 곡선(102..200), 미만이면 +50(50..100). 올리면 '상위' 취급이 어려워져 HP 가치가 전반적으로 낮아짐 | 4 | 기존 |
| 1 | 하위 배율 바닥 | utils.rs:892 (L53065/L53087) | 50 | 꼴찌도 가치 50%. 내리면 약한 챔피언 HP 가치가 더 무시됨 | 4 | 기존 |
| 2 | 상위 배율 기울기(×2) | utils.rs:894 (L53066/L53083 shl 1) | 2 | 1위 = 200%. 올리면 캐리 챔피언 HP 가 훨씬 비싸짐(보호·집중 대상) | 4 | 기존 |
| 3 | 유틸력 DPS 결합 계수 | utils.rs:874~875 (L52435/L52440 등) | cc_time_x_inv_cd/1000 · buff_inv_cd_count/10000 | CC·버프 챔피언의 유틸력이 편 DPS 합에 비례해 커짐. 분모를 줄이면 서포터 HP 가치 상승 | 4 | 기존 |
| 4 | 근처 집단 최대 인원 | utils.rs:934 (배열 10) | 10 | 자기+9. 양 팀 5:5 전원이 들어감. 줄이면 뒤쪽 원소(적군 뒤쪽)가 순위 모집단에서 빠짐 | 4 | 기존 |

<details><summary>`callees` 피호출자 3건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | champion_hp_value_uncached | game_ai::utils::champion_hp_value_uncached | in:game_ai::utils | fn(&game_ai::ScoreParameter, &game_ai::ChampionScoreParameter) -> i64 | game-ai\src\utils.rs:929 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 1 | rank_scale | game_ai::utils::rank_scale | in:game_ai::utils | fn(&[i64], usize, i64) -> i64 | game-ai\src\utils.rs:881 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 2 | util_power_combined | game_ai::utils::util_power_combined | in:game_ai::utils | fn(&game_ai::ChampionScoreParameter, i64) -> i64 | game-ai\src\utils.rs:868 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 5개**: `clamp`, `llvm.memset.p0.i64`, `llvm.smax.i64`, `llvm.umin.i64`, `slice_index_fail`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m00.ll:91173) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | 루프 가드 `n >= 10`/`idx >= n` 의 소스 표기(if-break 인지 take() 인지) — IR 은 `n_old > 8`·`n > 9`·`idx < n` 으로 회전돼 있어 표기 불가. 동작(최대 10원소, 아군 먼저)은 확정 | 4 |  |
| 1 | 미탐색 | :968~971 아군 유틸 루프가 완전 언롤(9회, L52459~52814)된 것은 컴파일러 판단 — 소스는 :974 적군 루프(L52832~)와 같은 형태로 추정 | 5 |  |
| 2 | 표기 불가 | rank_scale 에서 `< 51` 비교가 clamp 전 값(%366/%376)에 걸리는 것이 소스 그대로인지(`let r = …; let rank_x100 = r.clamp(0,100); if r < 51`) — 표기 불가, 외연 동일 | 4 |  |
| 3 | 미탐색 | near_allies/near_enemies 에 target 자신이 포함되는지는 ScoreParameter 생성 지점(이 명세 밖)의 몫 — 포함되면 equal 에 1이 들어가 middle 에 0.5 기여 | 4 |  |
| 4 | 미탐색 | attack_value/util_value/attack_power/util_power_base 의 산출식은 ChampionScoreParameter 생성부(score_parameter.rs:1379~) 몫 — 여기서는 소비만 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

