# 18차 배치 A 보고 — #57~#61 (게임 0.5.8)

기계 적용분 = `patch.json`(정정 44 · ev상향 55 · 브리핑오류 4 · `applypatch.py 18 --only A --dry` = 44/44·55/55·실패 0).
오라클 = `oracle/v18A_o1.rs`(#57) · `v18A_o2.rs`(#60) · `v18A_o3.rs`(#59) + 각 `.tsv` · 패치 생성 스크립트 `oracle/mk18A.py`.

## 0. 신선도
- 착수 시 `dossierfresh.py 18 A` = FRESH(18:15 도장). 제출 직전 재실행 = STALE — 바뀐 것은 `specs20_v3.json` 해시뿐(18:42, 다른 배치 적용분). `DOSSIER_A.md`(mtime 18:15)·`spec_*.md` 는 그대로, 77개 spec 인덱스 불변(57~61 이름 동일), `old` 44/44 가 현재 정본과 일치, `ev_up` 55행 전부 존재 → 패치 유효. 메인이 적용 전 `--dry` 한 번 더.

## 1. 실행한 것(명령 그대로)
```
python -X utf8 dossierfresh.py 18 A
python -X utf8 dloc.py /c/tfm2mods/_gaibc/m09.ll 13447 13449 13451 13452 13453 / 13683 13730 13659
python -X utf8 srclinecheck.py --only 57 ; python -X utf8 specgate.py --gate G15
python -X utf8 tcxdict.py LineGankerPlan / BrainMinionParameter / Blackboard / AbstractGameWithCache 0x28·0x68·0xa8 / Entity 0x670·0x628·0x660 / TeamPlan 0xc8·0xd0 / MobaMode 0x1b0·0x1e0 / --enum MorgardUseStrategy / --enum Chat
python -X utf8 divtable.py AbstractGame 0xf8 / 0x150 / 0x28 / 0xe8 / 0x108
python -X utf8 tcxq.py grep game_ai|game_core "<경로>"  (handle_press_epic pub · TeamPlan Default pub · DebugFrameData Default pub · World::strategy pub · Game::mode pub · ready_damage_to_target in:game_ai)
sh _verify3/build.sh …/v18A_o1.rs ; for c in 0..6: v18A_o1.exe $c   (케이스당 프로세스 1개)
sh _verify3/build.sh …/v18A_o2.rs ; v18A_o2.exe
sh _verify3/build.sh …/v18A_o3.rs ; v18A_o3.exe
python -X utf8 oracle/mk18A.py ; PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 18 --only A --dry
```
IR 원문 직접 열람: m09.ll 5284·5340~5372·5430~5470·5488~5525·5570~5578 / m08.ll 94136·94228~94300·94950~94962 / m07.ll 50443·50575~50611·54011·54039·54079·210 / m10.ll 49147·56717~56720·56889~56892 / _gcbc g07.ll 157005~157045 / g11.ll 49926~50030 / g15.ll 130480~130505.

## 2. §4 게이트 12건 — 판정
| 게이트 | 함수 | 판정 | 근거 |
|---|---|---|---|
| G16 P3 | #57 | 실오류 → i 1..6 | m09.ll:5284 `define void …(ptr %0 … ptr %5)` sret 없음 |
| G16 P3 + G5 | #58 | 실오류 → 3행(self/player/data, i 1..3) + 승격행 2개 delete | m08.ll:94136 `define internal fastcc … (i8 %0, i64 %1, i32 %2, ptr %3, ptr %4)` = 인자 승격. 호출자 94953~94960 이 self.line·team·position·cache·context 로 풀어 넘김 — 승격 사실은 각 행 role 에 |
| G16 P3 | #59 | 실오류 → i 1..5 | m10.ll:49147 인자 5·sret 없음 |
| G16 P3 | #60 | 실오류 → i 1..6 | m07.ll:50443 인자 6·sret 없음 |
| G12 | #57 consts[3]=512 | **오탐** | m09.ll:5368 `%24 = add nsw i32 %23, -5, !dbg !13453`(=epic.rs:512, inlinedAt 없음). `litpat(5)` 가 `, -5` 를 못 본다 — 17차 배치D `, -1` 과 같은 구멍. 게이트 수정: `litpat` 에 `-?` 허용. 후보 [518..591,615] 는 역할 ②④ 줄이라 ①(512)을 기각할 근거가 아니다 |
| G15 NEG | #59 consts[10] | 보강(낱말) | 「임계 아님」 부정문 ↔ CMP_ORD(`icmp ult %team, 2`) 충돌. 관례 「판정값 아님」으로. 구조적으로 `길이` 로는 못 내려간다(kindchk `길이` 지지관측 전부 WEAK) |
| G15 NEG | #61 consts[1] | 보강(낱말) | 동일 |
| G19 | #59 knobs[6] | **오탐** + where 보강 | 리터럴은 aux(m10.ll:56719·56891 `icmp ult i64 …, 19600000001`). `knobval.src_anchors` 가 `ir.aux[]` 를 안 훑는다(srclinecheck 는 15차에 넣음). 게이트 수정: src_anchors 에 aux 세그먼트 추가 |
| G19 | #61 knobs[0] | **오탐** + where 보강 | 71 은 본체 m07.ll:54039 `store i64 71` · 54079 `icmp eq i64 %20, 71`. 옛 앵커 m07.ll:210 은 `@anon…188 = constant [1136 x i8]` 정의 줄 |
| G7 | #59 open[1] | 닫음(사실 확정) — §4 | _gcbc/g07.ll:157005~157045 전문 독해 |
| G10 | #57 open[0] | 닫음(사실 확정) — §4 | BrainMinionParameter::update(_gcbc/g11.ll:49926) 독해 |

## 3. 오라클 결과(전부 setting_ok true)
### #57 handle_press_epic — v18A_o1: 7케이스 × 6,000호출 = 42,000회 · gate_mism 0 · pred_mism 0
- 21% 게이트: 시드별 `StdRng::seed_from_u64(s).gen_range(0..100)` 독립 계산 `v < 21` ↔ 실제 발령(chats.len==1) 1:1 일치(발령률 21.63%). `<= 20` 은 외연 동일 = 표기 불가.
- is_top_side 극성: `x <= height − y`(17차 배치C)로 세운 예측 전건 일치. IR `(h−y) <u x` true = !is_top_side = Bottom 분기(m09.ll:5447~5449 → %70 = +0x1c0 bottom_tower).
- 3 arm 전부 진입: `game.world.strategy[t].morgard_use`(pub) 로 Gather / Split14{Jungle|Mid} / Split131{Top,Support}. `PlayerState::strategy`(g15.ll:130480)는 is_solorank 아니면 vtable+0x108 `AbstractGame::strategy(team)` 복사뿐 — rnd 를 안 쓴다.
- far 라인: `game.mode.jungle_runner.{epic,serpen}.next_respawn_tick`(pub) 5000/100 ↔ 100/5000 반전 확인. Split131 은 `cache.player_champion[t][Top]=None` 으로 position2→Split(Top) 경로까지.
- 튜플 3순위: blackboard[team]+0x20/+0x48/+0x70 에 (5,2,−3)/(−1,0,9)/(0,0,0) 을 써 부호 비교·동률→Mid 확인. 범위: 1·2순위(타워 생존)는 전 타워 생존만 재어 미검증.
- Chat 24B(byte0 태그 21/20, byte1 line, +0x8 usize 0)·objective +0x41f 태그 5/6·+0x420 line 을 9,086건 전부 읽어 일치.

### #60 can_trace_without_tower — v18A_o2: 2,184케이스 · mism 0(true 2,094 / false 90)
- 재구현: 12점 표 + `(dx*range)/1000`(sdiv) + `Game::adjust_position`(pub) + `game_ai::can_tower_focused`(pub) any-부정. 중심 = 적 타워 12 + 격자 144 · range {0,999,1000,25000,60000,130000,400000} · 2팀.
- 앵커: 적 미드타워 정중앙 range 0 → false, 400000 → true. 999 vs 1000 결과 동일(내림).
- 부수 관찰(기록만): default 세팅에서 타워 `attack_effect.range`(+0x4a0)=0 이라 can_tower_focused 반경이 타워 중심 25000~45000 사이(o3 스캔: 45000 링은 289/289 탈출).

### #59 is_unreasonable_tower_dive_enemy — v18A_o3: 4,032케이스 · mism 0(무리 3,392 / 합리 640)
- 축: my_r{0,24,25,34,35,44,45,100} × t_r{0,1,25,26,50,51,100} × wdd{F,T} × 아군근접{0,1,2} × 적근접(시야){0,1,3} × 거리{20000,300000} × target{타워권,맵중앙}. 커버리지: A_true 864 · B_true 576 · C_true 80 · in_range 2,688 · ally_adv 896.
- 경계 대표: (my100,t25)→합리(B) / (34,25)→무리 / (35,25)→합리 / (100,26)→무리 / wdd (45,50) ally_adv 없으면 무리.
- pub 콜리 직접 호출: max_range_cached·can_tower_focused_when_battle·can_trace_without_tower·Blackboard::is_recent_visible. `ready_damage_to_target` 은 in:game_ai → 0 가정(default 챔프), t_r=1 에서 A 가 안 열려 자기검증.
- 두 번 실패 후 해결: ①타워 정중앙 → needs 전건 false(45000 링 탈출) ②적 타워 attack_effect.range=300000 으로 확장 → 진입 ③hp/좌표 write 전건 무효 — 원인 `fn set_pos(e: &Entity, ..)` 의 `&Entity` 인자가 readonly noalias 라 LLVM 이 raw store 를 UB 로 제거(volatile read 도 옛 값). `*const Entity` 인자로 바꾸자 정상 → TEMPLATE 수법 ⑦ 각주 요청.
- 범위: 140000 경계 자체 미검증(안 30000/밖 300000). TLS MAX_RANGE_CACHE 는 양쪽 동일 값 → 무해.

## 4. open/notes/closed 로 메인이 넣을 문면(경로 패치 불가 항목)
- #57 open[0] → closed(사실 확정): `Blackboard.<line>_minion_state.minion_count(i32) = cache.<line>_minions[team].len() − cache.<line>_minions[1−team].len()` — `BrainMinionParameter::update(&mut self, &mut StdRng, &GameContext, &AbstractGameWithCache, team, line)` _gcbc/g11.ll:49926, store 50024~50027(`%52 = sub i32 %47, %50` → +0x20), `!91843` = ai_interface.rs:47. 라인별 Vec = AbstractGameWithCache +0x10/+0x50/+0x90(32B/팀, +0x18=len). nexus[team]·nexus[1−team](+0x170) 중 하나라도 None 이면 store 없이 반환(g11.ll:50007~50009 → %454). 3순위 = 「그 라인에서 내 미니언이 적보다 얼마나 많은가」. (front_minion·from_mid·minion_power 는 미독해 — 범위)
- #57 notes[0]: patch 로 문면 정정(추정→확정).
- #58 open[2] → closed: `is_top_side(x,y) = x <= height − y`(pub map_regions.rs:21; 17차 배치C v17C_o1.tsv 9/9 · g09.ll:158986 `icmp uge`). `%65 = icmp ult i64 %64(h−y), %57(x)` true = !is_top_side → 21/14/9 = 대각선 아래쪽.
- #59 open[1] → closed: `Blackboard::is_recent_visible(&self, game, player, e)`(blackboard.rs:346, g07.ll:157005) = `game.is_visible(player.info.team, e.id)`(vtable+0xf8) `|| match game.get_player_by_champion_id(e.id)`(vtable+0x150) `{ None → false, Some(p) → self.last_visible[p.info.position] + 120 >= game.tick() }`(+0x1e0, g07.ll:157036~157041, blackboard.rs:348·350). 「지금 보이거나 마지막 목격 후 120틱(2초) 안」. `data.blackboard[enemy]` 를 넘기는 것은 오라클로 확인.
- #59 open[5](`_version`) 미탐색 유지. #60 open[0] can_tower_focused 부분 독해(tick<tower_attack_disable_tick 게이트 · 적 타워 순회 · nearest_enemy==내 id → true · 거리² vs attack_effect.range(+0x4a0) · 미니언 fold) — 사거리 식·시즈 예외 미확정, open 유지.

## 5. 무검사 축 표본
- one_line/layer 5개 전부 본문과 일치.
- callees[]: #57 `kind`(game_view DbEditAppearanceTarget)·`pick`(MatchUIRunner)·`push`(Staff/Athlete/Contract serialize) 는 logic 산문에서 긁힌 잡음 — 크레이트가 game_ai/game_core 가 아니면 후보 제외로 기계 필터 가능. #61 `check` 도 동형. 판정 술어 중 미확정은 #57 is_tower2·morgard_exists·tower(인라인, define 없음 — 정상)뿐.
- sig.params.role: #59 params[4] 오라클 일치(C_true 80 전부 wdd=true). #57 params[2] rnd 「strategy 에 전달」 — 전달은 맞으나 strategy 가 rnd 를 안 씀(g15.ll:130480 `%2` 미참조).
- mem.dir(#57 31r/4w): w 4행 오라클 관측, 불일치 0.

## 6. 판정 어휘
- 사실 확정: #57 minion_count 의미 · #57/#58 is_top_side 극성 · #59 is_recent_visible 본문 · #57 21% 게이트·3 arm · #59 HP 경계 5쌍 · #60 any-부정.
- 표기 불가: `< 21` vs `<= 20` · #60 all/any · #58 select 중첩 순서.
- 재료 부재 → 해소: #59 needs_tower_entry 진입은 default 타워 range 0 탓 — attack_effect.range 직접 쓰기로 열림(실전 타워 range 값은 미조사).
- 미탐색 유지: #59 open[0]·open[5] · #60 open[0] 잔여 · #58 open[0]·[1]·[4] · #61 open[0].

## 7. 내 지시(도시에)의 오류 — patch.json.brief_errors 4건
1. §4 G12 #57 후보 목록이 현재 게이트 출력(…,591,615)보다 낡음.
2. TEMPLATE.rs 에 `&Entity` 인자 헬퍼 raw-store 소거 함정 없음(이번 실측).
3. G19 두 건은 게이트 앵커 구멍 — 문구가 값 재검증으로 유도.
4. 「pub 이면 오라클」만 있고 세팅 경로(world.strategy · mode.jungle_runner · blackboard raw write · player_champion None)가 없다 — 수법 승격 제안.