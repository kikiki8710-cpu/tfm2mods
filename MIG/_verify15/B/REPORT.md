# 15차 배치 B 보고 — #25~#29 (게임 0.5.8, 2026-09-13)

기계 적용분 = `patch.json` (**정정 36 · ev상향 90 · brief_errors 6** · `applypatch.py 15 --only B --dry` → 36/36 · 90/90 · 실패 0 · **동작 변경 0건**).
⚠제출 시점에 `dossierfresh 15 B` = **STALE**(v3 도장 `03d55ff8…` → `14948de7…`). 원인은 ①v3 가 도시에 생성 12초 뒤(08:02:40)에 재생성됐고 ②메인이 08:31:23 에 B·C·M 패치를 **이미 적용**했기 때문(`_verify15/B/applied.json`: errors 36 · ev 90 · failed 0). `DOSSIER_B.md`·`spec_*.md` 는 08:00:48 그대로라 지시문 자체는 안 바뀌었고, 내 `old` 36건은 생성 시점(08:31)의 v3 에서 전부 대조됐다.

## 0. 요약 — 함수별

| # | 함수 | 정정 | ev상향 | 오라클 | 핵심 |
|---|---|---|---|---|---|
| 25 | v22_visible_enemy_is_runaway_threat | 6 | 12 | ✗(pub(crate)) | knobs[2] g07 줄 오기 · **소스 표기 전 줄 복원**(`||` 한 식) · open 0/1/2/3 전부 해소 |
| 26 | v23_objective_setup_pressure_line | 12 | 25 | ✅ 14/14 | line_lead 인자 순서 뒤집힘 · `>=` 표기 확정 · G16/G20/kind 미상 정리 · open 2/3/4 해소 |
| 27 | nexus_under_direct_attack | 3 | 21 | ✅ 13/13 | knobs[1] g07 · champions/minions **.ll 줄 뒤바뀜** · 필터 없음 확정 · open 0 해소 |
| 28 | v21_should_defer_support_target | 11 | 19 | ✗(pub(crate)) | knobs[6] g07 · **support_range 제곱 전(a) 확정** · else 구조·꼬리식 표기 · consts[2] src_line 1089→1090 · open 1/2/3 해소 |
| 29 | v23_recent_visible_enemies_near_point | 4 | 13 | ✅ 8/8 + 창 3/3 | knobs[2] g07 · **distance_sq 는 utils 4인자 자유함수**(Entity 메서드 아님) · open 3 해소 |

behavior_change = **0/36**. 이유: 옛 문면들이 전부 **외연은 맞고 표기·귀속·줄번호가 틀린** 부류였다(재구현하면 같은 동작). 단 #26 `line_lead(line, team)` 은 시그니처대로 부르면 컴파일이 안 되는 오기라 재구현자가 멈추는 자리였다.

## 1. 실제로 실행한 것 (명령줄)

```
python -X utf8 dossierfresh.py 15 B                      # FRESH(시작) / STALE(제출 — 위 원인)
sed -n '49400,49500p' m10.ll · sed -n '55350,55545p' m15.ll · sed -n '61399,61658p' m04.ll · sed -n '48962,49144p' m10.ll · sed -n '55548,55800p;56080,56099p' m15.ll
grep -n '^!NNNNN = ' m*.ll                                # !dbg → inlinedAt 루트 (5함수 전 상수 src_line 대조)
grep -n "^define.*Blackboard17is_recent_visible" /c/tfm2mods/_gcbc/g*.ll → g07.ll:157005 ; sed -n '157005,157055p' g07.ll
python -X utf8 divtable.py AbstractGame 0x28/0xf8/0x150/0x20 ; g15.ll:44 Game 구현 vtable 전역 직접 파싱(0x28 tick / 0xf8 is_visible / 0x150 get_player_by_champion_id 동일)
sed -n '50689,51083p' m10.ll (max_range_cached) · m00.ll:79611~79995 (LocalKey::with 클로저) · m10.ll:52940~53308 (battle::max_range) · m03.ll:144500~144563 (is_enemy_well_danger)
g15.ll:109546~109886 (minions) · 109887~110016 (champions) — Entity 필드 gep 0건 확인
python -X utf8 rmeta_srcmap.py game_ai fight_model.rs 1166 1178 / 1077 1121 · objective_helpers.rs 10 40 / 70 95 · "old\defense_nexus.rs" 119 136   # ★줄 길이 산술
python -X utf8 tcxdict.py <Entity|AbstractGameWithCache|Blackboard|BrainMinionParameter|GameContext|GameSetting|PlayerState|OperationData> <offset> · --enum TeamType/LineType/EntityType
sh _verify3/build.sh C:/tfm2mods/MIG/_verify15/B/oracle/v15B_o1.rs ; v15B_o1.exe > oracle/v15B_o1_tick0.out ; v15B_o1.exe 1000 > oracle/v15B_o1_tick1000.out
python -X utf8 _verify15/B/oracle/mk15B.py ; python -X utf8 applypatch.py 15 --only B --dry
```

오라클(`oracle/v15B_o1.rs`, TEMPLATE 기반 · `setting_ok true · towers 16 · twin 2/2`): 적 챔프 5명을 바이트 복제본(`MaybeUninit<Entity>`, 절대 drop 안 함)으로 갈아 끼워 좌표·hp 를 오프셋으로 조작, `Blackboard.last_visible[pos]` 로 최근가시 제어, 미니언은 복제본의 `0x68/0x88/0x90` 을 써서 위장. 결과 **tick0 32/32 · tick1000 35/35 MATCH, MISMATCH 0**. ⚠첫 판의 MISMATCH 1건(min_hp 50)은 **default 챔프 `stat_cached.hp == 1`** 이라 50% 가 0 이 되는 입력 판별력 문제였다(TEMPLATE 함정⑥ 계열 — hp 도 같다). `maxhp=1000` 고정으로 해소. 실행 결과가 **처음부터 틀린 게 아니라 입력이 틀렸다**는 것을 `enemyN maxhp=… hp=… ratio=…` 줄로 찍어 확인했다(오라클 자신을 검증).

## 2. 함수별 판정 (판정 어휘 · 적용 범위)

### #25 v22_visible_enemy_is_runaway_threat
- **실오류** knobs[2].where `g07.ll:157005~157049` → `157039` (`%25 = add i64 %24, 120, !dbg !187710` = blackboard.rs:350; 157005 는 define 줄). 같은 오기가 #27/#28/#29 에 복제.
- **판정 반전(open[3] 표기 불가 → 확정)**: 줄 길이 산술(`rmeta_srcmap`, 내용길이=chars−1, 들여쓰기 2, tcx `sp.c2` 로 교차검증)로 L1168~1176 **전 줄 ±0 복원**:
  `L1169(96) if !data.blackboard[1 - player.info.team].is_recent_visible(data.cache.game, player, enemy) ||` / `L1170(51)     is_ignored_well_enemy(version, player, enemy) {` / `L1171 return false;` / `L1172 }` / `L1174(80) let threat_range = (max_range_cached(data, enemy, champ) + 40000).min(150000);` / `L1175(57) enemy.distance_sq(champ) <= threat_range * threat_range`. ⟹ **하나의 `||` 식**(별도 두 if 아님). 「column 부재 → 표기 불가」는 **IR 한정** 판정이었다.
- **사실 서술 → notes 로 이동 요청(open 에는 patch 불가)**:
  - open[0] max_range_cached = `battle::max_range(champ,target)`(4슬롯 CastingTarget::check → Effect::range_adjust umax) 의 TLS `MaxRangeCache` 메모(키 seed·tick·슬롯 idx; 슬롯에 없으면 직접 호출). ⚠TLS 메모 → 오라클 시 케이스당 프로세스.
  - open[1] is_enemy_well_danger = 적 샘 코너 L자 영역(team 1: x≤64000∧800000≤y≤960000 ∨ x≤160000∧896000≤y≤960000 / team 0: 대칭). version 미사용.
  - open[2] vtable 슬롯: `Game as AbstractGame` 구현 vtable(g15.ll:44)에서도 0x28/0xf8/0x150 = tick/is_visible/get_player_by_champion_id — dyn 트레이트 슬롯은 트레이트가 정하므로 구현체 무관(호출부 is_recent_visible 이 고정 오프셋으로 부르는 것 자체가 증거).
- G10([25] class 미탐색인데 사실 서술) — 위 open[0] 처리로 해소. G7 과열림 2건 — open[2]/[3] 해소로 닫힘.
- 오라클: **재료 부재** — `v`=in:game_ai(pub(crate)) · 루트 재수출 없음(`_tcx/game_ai.json` p/v 조회) · 형제 0 · 호출자 18곳 전부 큰 플랜 함수. 시도 범위 = tcx 가시성 조회 + 호출자 목록. 대신 is_recent_visible(공유 콜리)은 #29 경유로 실행 확인.
- 잔여(미해결): consts[4] 150000 의 kind `인덱스` — **도구 결함**(§4 참조), meaning 으로 못 고침.

### #26 v23_objective_setup_pressure_line
- **실오류** logic L81 `line_lead(line, team)` → `line_lead(team, line)`(tcx sig `fn(&self, usize, LineType)`; 60자 검산).
- **실오류(G20 R1)** mem[9].name → `top_lead/mid_lead/bottom_lead [line][team]`(#36 과 통일; tcxdict 0x21c0 = top_lead[0]).
- **판정 반전(open[3])** L82 = `if lead >= 3 && minion_state.from_mid >= 2000 && minion_state.minion_count >= 2 {`(85자, `>` 3개는 82 로 불일치, 중첩 if 는 L83/84 와 불일치) → 한 `&&` 식 · 소스는 `>=`, IR 이 `>` 로 정규화. consts[4..6] meaning 에 병기(값은 IR 그대로 둠).
- **판정 반전(open[2])** L92 = `best.map(|(line, _)| line)`(28자) · L87 = `if best.is_none_or(|(_, best_score)| score < best_score) {`(62자) · L86 = `let score = minion_state.from_mid + lead as i64 * 10000 + minion_state.minion_count as i64 * 1000;`(102자).
- **사실 서술 → notes**: open[4](team≥2 는 line_exists 참인 첫 라인에서 panic) — IR 블록 %70~%79 로 확정, 실전 도달 불가.
- G16 P1: params role 에 `%0/%1/%2,%3` 명기(슬라이스 팻포인터 SROA 승격). kind `미상`(consts[8]) → meaning 에 `계수` 낱말.
- 오라클 14/14: 동점→먼저 온 라인 · 제외 경계 lead 2/3 · from_mid 1999/2000 · minion_count 1/2 · 가중치 10000/1000 등가 · 음수 minion_count · 전부 제외/빈 슬라이스 → None · 중복 라인.
- 미탐색 유지: open[0] line_lead 의 의미(region_point 검사), open[1] from_mid 부호 — `g11.ll:49926 BrainMinionParameter::update` 를 80줄 스캔했으나 (…)/2 분기 4개의 의미까지는 안 읽었다(**미탐색**, 재료는 있음: g11.ll:49926~ + ai_interface.rs:117~162 줄 길이 산술).

### #27 nexus_under_direct_attack
- **실오류** knobs[1].where g07 줄(위와 동일).
- **실오류** logic 의 `_gcbc g15.ll:109546`(champions) / `109887`(minions) 가 **서로 뒤바뀜**(grep define: 109546 = minions, 109887 = champions).
- **보강(open[0] 해소)**: champions() = player_champion[team][0..5] Some 전부, minions() = top/mid/bottom_minions[team] 세 Vec 이어붙임 — 둘 다 **Entity 필드 gep 0건 = 필터 없음**(죽음·Illusion 검사 없음). 캐시 Vec 자체의 구성은 `AbstractGameWithCache::new`(g15.ll:109064) 소관 = 미탐색.
- open[2](L127 `&&` 표기): **재료 부재** — 줄 길이 산술 시도: L126=87·L127=99·L128=47·L129=17·L130=16·L131=3 인데 변수명(`enemies`/`enemy_champions`…)·명명 상수(`240000 * 240000` vs 상수명) 자유도가 커서 ±0 후보가 유일하지 않다. 확정된 것: 조건 순서(is_recent_visible → 거리)는 IR, L129 는 17자(`return …;` 꼴). 미탐색 = MIR 없음(mir=0) 확인 안 함.
- open[1](nearest_enemy 갱신 규칙)·open[3](샘 제외 없음 = 사실, ev5 추정 문구는 notes 로) — 유지/이동.
- 오라클 13/13: d=240000 true / 240001 false · 비가시(fog+last_visible 옛날) false · 미니언 Some(nexus.id) true / Some(other)·None·Tower 태그 false · bottom_minions 도 true · 아군 목록은 무시.

### #28 v21_should_defer_support_target
- **실오류** knobs[6].where g07 줄.
- **판정 반전(open[1] → (a))**: L1089(52) `let support_range = if target.team == champ.team {` / L1090(10) `120000` / L1092(49) `max_range_cached(data, champ, target) + 25000` / L1094(65) `if target.distance_sq(champ) <= support_range * support_range {` — support_range 는 **제곱 전**, 14400000000 은 접힘. consts[2].src_line 1089→**1090**, knobs[1].where 1090.
- **표기 정정(외연 동일)**: L1106(73) `if local_outnumbered && (can_near_enemies > 0 || die_tick <= tps * 3) {`(`!= 0` 은 74 불일치 → `> 0`) · L1110(50) `if can_near_enemies > 1 && die_tick <= tps * 4 {` **else 없는 독립 if**(열세 경로에선 can_near==0 이라 항상 거짓 → LLVM 이 else 로 접음) · L1118~1119 꼬리식 `target.team != champ.team && !data.blackboard[…].is_recent_visible(data.cache.game, player, target)`(30/93자; `if … {` 는 32 불일치 + 닫는 줄 없음).
- **사실 서술 → notes**: open[2](L1100 let 은 순수 계산, IR 재배치는 부작용 없음 — 1100=61자 `let local_outnumbered = …;` 확정) · open[3](max_range_cached 내부 = #25 와 동일 결론).
- 잔여: L1099 `hp_ratio` 줄 61 vs 후보 `champ.hp * 100 / champ.stat_cached.hp.max(1)` 62 — **잔차 1 미해결**(#29 L12 도 같은 식이 −1: 공통 토큰이 1자 짧다는 뜻이나 후보를 못 찾음). 동작은 IR(umax 1, mul 100, udiv)로 확정.
- 오라클: **재료 부재**(pub(crate), 루트 재수출 없음, 호출자 3곳). knobs[6](120틱)만 #29 경유 실행 확인.

### #29 v23_recent_visible_enemies_near_point
- **실오류** knobs[2].where g07 줄.
- **판정 반전(open[3])**: L35 의 거리식은 `game_core::utils::distance_sq(e.x, e.y, x, y)`(4인자 자유함수, DISubprogram !61866 이 클로저 L35 에서 직접 inlinedAt — 중간 Entity 스코프 없음; 인자 순서는 `icmp ult %37(e.x), %2(x)`) — 튜플도 Entity 메서드도 아님. callees[1] Entity::distance_sq 는 이 함수의 콜리가 아니다(logic 정정으로 다음 빌드에서 떨어짐).
- 소스 전 줄 복원: L31(148 시그니처) / L32(49) `data.cache.iter_champions(1 - player.info.team)` / L33 `.filter(|e| {` / L34(37) `v23_healthy(e, min_hp_ratio) &&` / L35(55, 들여쓰기 8) `distance_sq(e.x, e.y, x, y) <= range * range &&` / L36(91) `data.blackboard[1 - player.info.team].is_recent_visible(data.cache.game, player, e)` / L37 `})` / L38 `.count()`. 변수 `enemy_ix`·`range_sq` 는 소스에 없음(LLVM 호이스팅).
- open[1](`>=` vs `!(<)`): **표기 불가 유지, 근거 갱신** — 외연은 오라클로 확정(ratio==min 포함) · 줄 길이 `>=` 후보 57 vs 실제 56(잔차 1), `!(<)` 는 59(잔차 3) → `>=` 가 유력하나 ±0 아님.
- open[2](죽은 챔프 Some 여부): 미탐색 유지(game_core 캐시 구성 소관) — champions() 자체는 필터 없음(#27).
- 오라클 8/8 + 창 3/3: range 경계 포함(199999/200000/200001) · min_hp 경계 `>=`(50%/49%) · **120틱 창 경계(last_visible=tick−120 가시 / tick−121 비가시)**.

## 3. 게이트가 안 보는 축에서 나온 것 (§4-b 표본 대조)

| 축 | 표본 | 결과 |
|---|---|---|
| `callees[]` 경로 | #29 distance_sq 2후보 | **[1] Entity::distance_sq 가 오답**(실제 [0] utils::distance_sq). 검사기 제안: `logic` 이 `X::f(` 로 적은 이름을 IR `DISubprogram(name: f, linkageName)` 의 **inlinedAt 사슬이 담당 함수 줄에 직접 닿는 것**과 대조(중간 스코프 없이 닿으면 그 정의가 콜리) |
| `logic` 인자 순서 | #26 line_lead | **뒤집힘**. 검사기 제안: `logic` 의 `f(a, b)` 를 tcx sig 의 인자 타입 순서와 대조(인자 이름에 `team`/`line` 같은 타입 힌트가 있을 때만) |
| `logic` 의 `.ll` 줄 참조 | #27 champions/minions | **서로 뒤바뀜**. 검사기 제안: `logic` 안 `gNN.ll:LINE` 은 그 줄이 `^define.*<이름>` 인지 대조(G13 은 knobs 만 본다) |
| `one_line` | #25 | 두 번째 게이트(샘 제외) 누락 → 보강. 나머지 4개는 정확 |
| `notes[]`/`closed[]` | 5함수 전부 0건 | 대조 대상 없음 |
| `mem.dir` | 67행 전부 r | IR load 와 모순 0(불일치 0 = 결과) |
| `consts.src_line` | 34행 | 불일치 1(#28 consts[2] 1089→1090, phi 접힘이라 G12 강한 후보 0) |
| `sig.params` | 26행 | #25 params[1]·#26 params 0~2 보강 외 이상 없음(#28 `version` = IR `i64 %0` noundef 없음·dbg poison = 미사용 확인) |

## 4. 내 지시(도시에)의 오류 — patch.json `brief_errors` 6건 요지
1. G13 4건은 오탐이 아니라 **한 오기의 4중 복제**(157005=define 줄) — cross-spec 복제인데 G20 은 knobs.where 를 안 본다.
2. G20 [27] Entity 0x90 은 **#27 이 tcxdict 표기와 일치**, 어긋난 쪽은 #34(다른 배치) — 게이트가 「어느 쪽이 정본인가」를 안 적는다.
3. §1 의 「pub 아니면 상위 래퍼/형제」 안내는 #25/#28 에 재료가 없고, 실제 돌파구는 **줄 길이 산술**이었다(§1 이 안 가리킴).
4. `consts.kind`: llvm.umin 클램프 상한(#25 150000)은 `상한` 을 써도 `인덱스` 로 파생 — `kindchk.SUPPORT[임계]` 에 `MINMAX` 없음(도구 결함).
5. **`mkpatch.Patch.ev()` 가 2단 배열 경로를 전부 거부**(`m.group(3) is None` 검사 오류; idx 는 group(4)) — 참조구현이 ev_up 을 한 건도 못 만든다. 우회 = `oracle/mk15B.py` 머리의 대체 함수.
6. §5 예시의 `/specs[25]/consts[3]` 이 이 배치의 실제 #25 와 겹쳐 헷갈림(가상 예시 인덱스는 배치 밖 번호로).

## 5. 막힌 지점 · 소요
- 막힘: ① #27 L127 표기(변수명·명명상수 자유도) ② `X.hp * 100 / X.stat_cached.hp` 계열 2줄의 **잔차 −1**(#28 L1099·#29 L12 공통) — 같은 가설(필드명·공백) 2회 실패로 중단. 남은 재료: rmeta `SourceFile` 에 원문 없음, `mir=0`(v23_healthy 는 xinl 확인 안 함 — **미탐색**: `_tcx` 의 `xinl` 이 1 이면 `mirdump` 로 span 을 얻을 수 있다).
- 소요 체감: 정본 4종 읽기 ~15분 · 5함수 IR/dbg 대조 ~35분 · game_core 콜리(is_recent_visible/max_range_cached/is_enemy_well_danger/champions/minions) ~20분 · 줄 길이 산술 ~25분 · 오라클 작성·빌드 3회·실행 ~25분 · patch/보고 ~20분. 가장 비쌌던 것은 오라클 첫 MISMATCH 의 원인 확인이 아니라 **오라클 borrow 구조**(복제본 포인터를 `*mut` 로 두는 것)였다.
