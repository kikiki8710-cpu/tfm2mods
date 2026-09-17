# e8aeb0→f34d10 LineDefenseSubPlan::calculate_score_parameter_value

## logic_060
```
fn calculate_score_parameter_value(&self, _rnd, player, data, parameter: &mut ScoreParameter)   [line_defense.rs:396]
  line_style: bool = (self.style == LineStyle::Defensive)        // 태그 1 → true          [L397] · self+0
  champ = data.cache.player_champion[player.info.team(+0xa00)][player.info.position(+0xa90)].unwrap()      [L399 · team<2 체크 · cache+0x1e0+team*0x28+pos*8]
  hp_ratio = champ.hp(+0x670) * 100 / champ.stat_cached.hp(+0x628)   // usize udiv · max_hp==0 이면 패닉   [L400]
  value = if line_style /*Defensive*/ { if hp_ratio > 50 { 50 } else { 70 } }
          else          /*Aggressive*/ { if hp_ratio < 50 { 50 } else { 30 } }             [L401] (동일)
  aggr = player.traits.aggressive_play(+0x49d) != 0; def = player.traits.defensive_play(+0x49e) != 0   // ★0.6.0 특성 플래그
  my = if aggr { value } else if def { 90 } else { value }                                 // ★0.6.0 (DefensivePlay 이고 AggressivePlay 아니면 90 고정)
  parameter.line_subplan(+0x1501) = true                                                   // ★0.6.0 신설 플래그(SubPlan::score 후처리가 읽음)
  parameter.player.attack_value(+0x9c0) = my                                               [L418] · ★0.6.0 (구 value)
  parameter.player.util_value(+0x9c8)   = my                                               [L419]
  for p in parameter.near_allies(+0x14b8 ptr / +0x14d0 len · stride 0xd8).iter_mut()  { p.attack_value(+0xa8) = 50; p.util_value(+0xb0) = 50; }   [L421~423] (동일)
  ev = if aggr { 150 } else if def { 35 } else if line_style { 50 } else { 100 }           // ★0.6.0 (구: line_style ? 50 : 100)
  for p in parameter.near_enemies(+0x14d8 ptr / +0x14f0 len · stride 0xd8).iter_mut() { p.attack_value = ev; p.util_value = ev; }   [L426, 435~436]
  return                                                                                    [L438]

해석: 구 규칙 위에 특성이 얹힌다. AggressivePlay 는 자기 가치 그대로·적 가치 150(적을 매우 높게 봐 공격 선호) · DefensivePlay(비공격) 는 자기 가치 90(자기 보호)·적 35 · 특성 없으면 구 규칙. +0x1501 은 LineDefense/LineSafe/LineWait 의 calc 만 1 로 세워 SubPlan::score 공통 후처리(교환비 거부)를 Line* 서브플랜에 한정한다. gen_range 사이트 0.
```

## changes
- L401 뒤: `my = aggr ? value : def ? 90 : value` (자기 가치).
- 신설: `parameter.+0x1501 = 1`.
- L426: 적 가치 `ev = aggr ? 150 : def ? 35 : (line_style ? 50 : 100)`.
- 오프셋: PS +0x930/+0x9c0→+0xa00/+0xa90 · ScoreParameter player attack/util +0x9c0/+0x9c8 · near_allies +0x14b8/+0x14d0 · near_enemies +0x14d8/+0x14f0(stride 0xd8 · +0xa8/+0xb0).

## verified
- Ghidra 0.6.0 f34d10 디컴 전문: value 표(0x46/0x32 · 0x1e/0x32 · self+0 스타일) · `mode = 2 - +0x49e` · `my = +0x49e==0 ? value : 0x5a` · `+0x1501 = 1` · `if +0x49d { my = value; mode = 0 }` · +0x9c0/+0x9c8 · 아군 루프 0x32 · 적 루프 mode 0 → 0x96(150) / mode 1 → 0x23(35) / mode 2 → line_style ? 0x32 : 0x64.
- 미확인 없음(함수 전체가 디컴 한 화면).

## confidence
A — 전 항 0.6.0 디컴으로 확인.