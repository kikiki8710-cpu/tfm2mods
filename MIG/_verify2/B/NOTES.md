# 배치 B 2차 반증검증 작업 노트 (2026-09-11 / 게임 0.5.8 / SDK sdk_058 / nightly-2026-05-24)

## 만든 것
| 파일 | 무엇 |
|---|---|
| `B_o1.rs` / `B_o1.tsv` | 도달성 스모크 — fountains 실측 · `BigPlan::get_name` 8종 실행 · SubPlan/BattleSubPlanGoal 태그 · 07/08/09 호출 가능 확인 |
| `B_o2.rs` / `B_o2.tsv` | **09 `check_favorable_engage_formation` 전수 진리표 400시나리오 = 400/400 MATCH** (+ version 12값 스윕 diff 0) |
| `B_o3.rs` | 08 `is_end` objective 진리표 (Morgard/Serpen/Defense/Nexus × ObjectPhase 4종) + `15*tps` 임계 스윕 |
| `B_o4_blocked.rs` | 05·06 오라클 차단 실측(E0624 private method) |
| `s05..s09.json`, `shared.json` | `_spec\specs20.json` 에서 뜬 읽기 사본 |

## 컴파일/실행 레시피 (재현용)
```
powershell -ExecutionPolicy Bypass -File C:/tfm2mods/MIG/spanprobe.ps1 \
  -Src C:/tfm2mods/MIG/_verify2/B/B_o2.rs \
  -Extra "--extern bumpalo=C:/tfm2mods/sdk_058/mod-sdk/deps/libbumpalo-dafef1f270bdb02f.rlib --extern rand=C:/tfm2mods/sdk_058/mod-sdk/deps/librand-e2a5dd20f067a3a7.rlib -C lto=fat -C codegen-units=1 -C opt-level=1"
%TEMP%\tfm2_spanprobe\B_o2.exe
```

## ★새로 뚫린 오라클 레시피 (다음 배치용)
1. **모듈이 `pub(crate)` 여도 아이템이 크레이트 루트로 재수출돼 있으면 호출된다.**
   판정은 모듈 가시성이 아니라 `_tcx\pubapi_game_ai.txt` / `_tcx\game_ai.json` 의 `p`(재수출 해석 경로) + `v`.
   실측: `game_ai::check_favorable_engage_formation`(fight_check 소속) · `game_ai::enemy_minion_line_action_danger_damage_at` 모두 **pub**.
2. **`AbstractGameWithCache` 는 전 필드 pub** → `Entity`(전 필드 pub, `Clone`)를 복제해 좌표·HP 를 바꾼 뒤
   `cache.player_champion[t][p] = Some(&my_entity)` 로 **입력을 완전히 통제**할 수 있다. 09 진리표가 이걸로 나왔다.
3. `GameSetting::default().tick_per_second == 0` 실측 확인 → 60 으로 세팅해야 한다(BRIEF §1① 경고 확인).
4. `MapDef.fountains` pub → 실측 `[(0,896000,64000,960000), (892000,0,960000,64000)]`.

## ★함정 추가 (1차 부록 B 에 붙일 것)
- **인라인 루트 줄 ≠ 선언 줄.** tcx `sp` 로 확인해야 한다.
  `iter_champions` 선언 1904 / 클로저 1905 · `Position::as_index` 선언 580 / 본문 581 · `Entity::distance_sq` 선언 2157 / 본문 2158.
  ⟹ 1차의 "08 명세 1904 는 1줄 틀렸다(1905)" 는 **오히려 오독**이다.
- **`_tcx\mirdump_game_core.txt` 는 span 에 칸(column)까지 있다** → game_core 헬퍼는 소스 ±0 복원이 가능.
  실증: `simulation.rs:1905` = `    self.player_champion[team].iter().filter_map(|c| *c)` (56자, srcmap 57−1 ±0).
- `hunt_and_poke.rs` 쌍둥이는 **줄번호가 1 밀려 있다**(epic fn sig L163 / serpen L162). `hunt_and_battle.rs` 쌍둥이는 동일.

## ★06 `engage.rs` 줄 길이 산술 복원 (잔차 0, L13~L32)
`rmeta_srcmap`(값−1) + tcx 클로저 `sp` + 패닉 Location 칼럼(21:19 / 28:21 / 17:23) 3중 교차.

| 줄 | 실측 길이 | 복원 | 잔차 |
|---|---|---|---|
| 13 | 178 | `  pub fn v2_response_retreat_stance(...) -> BattleSubPlanGoal {` (tcx sp 13:3-13:177 + ` {`) | 0 |
| 14 | 20 | `    if version < 2 {` | 0 |
| 15 | 40 | `      return BattleSubPlanGoal::RunAway;` | 0 |
| 16 | 5 | `    }` | 0 |
| 17 | 106 | `    let Some(champ) = data.cache.player_champion[player.info.team][player.info.position.as_index()] else {` | 0 (칼럼 17:23 = `data` ✔) |
| 18 | 40 | `      return BattleSubPlanGoal::RunAway;` | 0 |
| 19 | 6 | `    };` | 0 |
| 20 | 65 | `    let nearest = data.cache.iter_champions(1 - player.info.team)` | 0 |
| 21 | 101 | `      .filter(\|c\| data.blackboard[1 - player.info.team].is_recent_visible(data.cache.game, player, c)` | 0 (`\|c\|` 15-17 ✔ · 칼럼 21:19 ✔) |
| 22 | 53 | `        && !is_ignored_well_enemy(version, player, c)` | 0 (**`fight_model::` 접두 없음** — 있으면 66자) |
| 23 | 51 | `        && champ.distance_sq(c) <= 200000 * 200000)` | 0 (**소스는 `<=`+곱셈**, `< 40000000001` 은 접힘 결과) |
| 24 | 44 | `      .min_by_key(\|c\| champ.distance_sq(c));` | 0 (`\|c\|` 19-21 = tcx closure#1 ✔) |
| 26 | 63 | `    let near_enemies = bumpalo::collections::Vec::from_iter_in(` | 0 |
| 27 | 53 | `      data.cache.iter_champions(1 - player.info.team)` | 0 |
| 28 | 103 | (L21 과 동일 + 들여쓰기 +2) | 0 (`\|c\|` 17-19 ✔ · 칼럼 28:21 ✔) |
| 29 | 55 | (L22 + 2) | 0 |
| 30 | 74 | `          && champ.distance_sq(c) <= 150000 * 150000), data.context.pool);` | **0 — 1차의 「잔차 +23」이 여기서 닫힌다** |
| 31 | 82 | `    let die = check_kill_die_tick(version, rnd, data, player, champ, near_enemies,` | 0 |
| 32 | 67 | `      bumpalo::collections::Vec::new_in(data.context.pool), debug);` | 0 |
| 33 | 51 | `    if die > data.context.setting.tick_per_second {` | **+1 잔차(미확정)** |

⟹ 잔차 +23 의 정체 = **`.collect_in(pool)` 이 아니라 `Vec::from_iter_in(iter, data.context.pool)` 의 두 번째 인자 + 닫는 괄호 + `;`** = `, data.context.pool);` **정확히 21자**(43+21=64, 들여쓰기 10 → 74).
독립 확증: IR `!dbg engage.rs:30` 이 `data.context`(+8) → `pool`(+0) 로드를 물고 있다(m13.ll:45586~45588) · 호출 심볼이 `bumpalo::collections::vec::Vec<&Entity>::from_iter_in`(m13.ll:45590).
⚠**후보 유일성은 미보장**(같은 길이의 다른 표기 가능) — 잔차 0 이라는 사실만 주장한다.

