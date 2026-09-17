# ec9de0→f88880 wave_priority_clearer_position

## logic_060
```
fn wave_priority_clearer_position(player, data, line: LineType(u8), exclude_jungler: bool) -> Option<usize>   // f88880 · ★0.6.0 4번째 인자 신설
let team = player.info.team(+0xa00);   // ★0.6.0 +0x930→+0xa00 · team>=2 → panic_bounds_check
// L246  target = wave_priority_clear_target(player, data, line)  — objective_helpers.rs:258~266 전부 인라인 (0.5.8 동일)
//   L258  let nexus = cache.nexus[team]                         // +0x170 · None → 폴백
//   L259  let minions = cache.line_minions(line)[1 - team]      // simulation.rs:1807 · 0x10 + line*0x40 + enemy*0x20 (bumpalo Vec<&Entity>) · len==0 → 폴백
//   L260  minions.iter().min_by_key(|m| dist_sq(m, nexus))      // 첫 원소 + 루프(dist² 작으면 교체 · 동률 유지) · 인라인
//   L261  Some(m) → target = (m.x, m.y)
//   L265  None(넥서스 없음 | 적 미니언 없음) → target = first_tower_position(team, line):
//         Top: team0 (48000,272000) / team1 (272000,48000)   Mid: team0 (368000,592000)/team1 (592000,368000)   Bottom: team0 (688000,912000)/team1 (912000,688000)
// L249~255 본체 = 클로저 f5d920(env = {&flag(bool), start=0, end=5, cache, player, &tx, &ty}) · 폴드 db73c0(구 e2f230)
let pick = |excl: bool| -> Option<usize> {                        // ★0.6.0 신규: 정글(pos 4) 제외 플래그
  (0..5).filter(|&pos| !(excl && pos == 4))                        // ★0.6.0 `if pos==4 && *flag → skip`
  .filter_map(|pos| {
    let c = cache.player_champion[team][pos]?;                     // L249 (+0x1e0 + team*0x28 + pos*8)
    if !(c.hp(+0x670) * 100 / c.stat_cached.hp(+0x628) > 29) { return None; }   // L250 · max_hp==0 이면 div_by_zero 패닉
    Some((pos, dist_sq(c, target)))                                // L251
  })
  .min_by_key(|&(pos, d)| (d, pos))                                // L253 · 키 = (거리², 포지션) — 동거리면 낮은 포지션
  .map(|(pos, _)| pos)                                             // L254~255
};
// ★0.6.0 신규: 호출 순서
if exclude_jungler {
  if let Some(p) = pick(true) { return Some(p); }                   // flag=1 로 1차 탐색
}
return pick(false);                                                 // flag=0 (구 동작 그대로)
```
// 콜러(참고): passive_plan d38180 4곳 = 인자 0 / LPH+0x24d5 / *rsi / 1 · fb8da0 1곳 (브리핑 「콜러」 줄).

## changes
- 시그니처: 4번째 인자 `exclude_jungler: bool` 신설(0.5.8 3인자).
- L249: 후보 필터에 `excl && pos==4 → 제외` 추가(폴드 클로저 f5d920/db73c0 · 구 e2f230).
- 반환: `exclude_jungler ? pick(true).or_else(|| pick(false)) : pick(false)`.
- 오프셋: PlayerState team +0x930→+0xa00. 그 외(nexus +0x170 · line_minions 산식 · 폴백 좌표 · HP 29 · 키 (d,pos)) 동일.

## verified
- Ghidra 0.6.0 f88880 디컴: nexus +0x170 · line*0x40+(1-team)*0x20 슬라이스 min 루프 · 폴백 좌표 6종(0xdea80/0xa7f80/0x90880/0x59d80/0x42680/48000) · `if (param_4) { flag=1; f5d920; if Some → return } flag=0; f5d920`.
- Ghidra 0.6.0 f5d920 디컴: `if (*flag == 1) { for pos { if pos != 4 { … } } }` 분기 · HP `>0x1d` · 폴드 db73c0 호출 확인.
- 미확인: 폴드 db73c0 내부(키 (d,pos) 동률 규칙)는 RE 결론 인용.

## confidence
A — 본체·클로저 둘 다 디컴 대조.
