# 9차 배치 B — 축 ``sig.params[].role` — **G16 승격**` · 함수 `00`~`04` (34행)

> `mkdossier.py --axis` 생성물. **칸을 하나도 줄이지 않았다** — 7차까지 지시 오류는 전부 「줄여 적은 자리」에서 났다.

| # | 함수 | idx | i | name | ty | role | ev |
|---|---|---|---|---|---|---|---|
| `00` | ult | 0 | 0 | (sret) |  | 반환값 출력 슬롯. +0x0=태그(-1=None, 5=Input::Ult), +0x8=InputTarget(24B) | 4 |
| `00` | ult | 1 | 1 | version |  | AI 버전 게이트. 이 함수 본문에선 분기 없음 — get_input_target / safe_move_avoiding_enemy_well 로 그대로 전달만 한다 | 4 |
| `00` | ult | 2 | 2 | rnd |  | 본문에서 직접 안 씀. get_input_target 으로 전달만 | 4 |
| `00` | ult | 3 | 3 | player |  | info.team(0x930) / info.position(0x9c0) 을 읽어 자기 챔피언을 찾는 데 씀 | 4 |
| `00` | ult | 4 | 4 | data |  | cache(0x0)=AbstractGameWithCache, context(0x8)=GameContext | 4 |
| `00` | ult | 5 | 5 | positioning_score |  | 본문에서 직접 안 씀. get_input_target 으로 전달만 | 4 |
| `00` | ult | 6 | 6 | target |  | 궁극기를 쓸 상대(적) 엔티티. 거리·가시성·반경 계산의 기준 | 4 |
| `01` | calculate_jungle_action_score | 0 | 1 | _rnd |  | 본문에서 전혀 안 씀(소스에서도 `_` 접두 = 미사용 확정) | 4 |
| `01` | calculate_jungle_action_score | 1 | 2 | player |  | info.team, info.position 두 필드만 읽어 챔피언 엔티티를 찾는 용도 | 4 |
| `01` | calculate_jungle_action_score | 2 | 3 | data |  | cache(+0x0), context(+0x8) 사용. blackboard(+0x10)는 안 씀 | 4 |
| `01` | calculate_jungle_action_score | 3 | 4 | _parameter |  | 본문에서 전혀 안 씀 — 이 함수의 계수는 전부 하드코딩 리터럴 | 4 |
| `01` | calculate_jungle_action_score | 4 | 5 | _action |  | 본문에서 전혀 안 씀 | 4 |
| `01` | calculate_jungle_action_score | 5 | 6 | effect |  | expected_damage_target 의 self 로 통째로 전달. 필드 접근 없음 | 4 |
| `01` | calculate_jungle_action_score | 6 | 7 | t |  | 평가 대상 엔티티. ty(+0x68) / ty.Jungle.info.camp_type.0(+0x98) / hp(+0x670) 를 읽음 | 4 |
| `02` | sub_plan | 0 | 0 | (sret) |  | 반환값 out-ptr. m12.ll:34867 ptr dead_on_unwind noalias noundef writable writeonly sret([72 x i8]) align 8 captures(none) dereferenceable(72) %0. 본문이 태그·페이로드를 여기에 store 한다 | 4 |
| `02` | sub_plan | 1 | 1 | self |  | ★IR %1 (=%0 은 반환 out-ptr). 필드 team:usize@0x0 · line:LineType@0x8. 본문은 line 만 읽는다(team 은 안 씀 — team 은 player.info.team 에서 가져옴). ⚠**이 params 표는 sret out-ptr 을 빠뜨렸다** — m12.ll:34867 `define void @…AttackNexusPlan8sub_plan(ptr dead_on_unwind noalias noundef writable writeonly sret([72 x i8]) align 8 captures(none) dereferenceable(72) %0, ptr … %1, i64 noundef %2, …)` 로 IR 인자가 8개인데 표는 7행뿐이라 **표의 자리 번호가 IR 보다 한 칸 앞선다.** 8차 확정 규약은 07·15 처럼 `(sret)`(i=0) 행을 params[0] 로 싣는 것인데 `applypatch` 에 행 추가가 없어 기계로 못 넣었다(8차 배치C) | 4 |
| `02` | sub_plan | 2 | 2 | version |  | AI 버전 게이트. ★이 함수에선 한 번도 참조되지 않음(분기 없음) | 4 |
| `02` | sub_plan | 3 | 3 | rnd |  | readnone — 난수 미사용 | 4 |
| `02` | sub_plan | 4 | 4 | player |  | info.team(0x930) · info.position(0x9c0) 만 읽음 | 4 |
| `02` | sub_plan | 5 | 5 | data |  | cache(0x0) · context(0x8) 사용. blackboard(0x10) 미사용 | 4 |
| `02` | sub_plan | 6 | 6 | _team_plan |  | readnone — 미사용(이름의 _ 접두와 일치) | 4 |
| `02` | sub_plan | 7 | 7 | _debug |  | readnone — 미사용 | 4 |
| `03` | defensive_crisis | 0 | 1 | version |  | AI 버전. 이 함수 본문에선 분기에 안 쓰인다 — IR 은 `%0` 을 8B alloca `%11` 에 한 번 spill 하고(m10.ll:33874 `store i64 %0, ptr %11` + `#dbg_declare`) 그 포인터를 클로저 환경·피호출자에 넘길 뿐이다(값 비교 0건). 그대로 effect_cc_time / check_kill_die_tick / is_ignored_well_enemy 로 그대로 전달만 된다 | 4 |
| `03` | defensive_crisis | 1 | 2 | rnd |  | 본문에서 직접 안 씀. check_kill_die_tick 에만 넘김 | 4 |
| `03` | defensive_crisis | 2 | 3 | player |  | 판단 주체(아군) 플레이어. info.team 만 읽는다 | 4 |
| `03` | defensive_crisis | 3 | 4 | data |  | {0x0 cache: &AbstractGameWithCache(8840B), 0x8 context: &GameContext(64B), 0x10 blackboard: &[Blackboard;2]} | 4 |
| `03` | defensive_crisis | 4 | 5 | target |  | 위기 판정 대상 챔피언(보통 아군 자신). ★이 본문이 %4 에서 **직접 읽는 것은 +0x5c0 한 곳뿐**이다 — m10.ll:33981 `%46 = getelementptr inbounds nuw i8, ptr %4, i64 1472` → m10.ll:33982 `%47 = load i64, ptr %46` → m10.ll:33983 `player_by_champion_id(%18, i64 noundef %47)` ⟹ +0x5c0 = 챔피언 id. ~~좌표~~ 는 거짓: m10.ll:33864~34294 범위에 `ptr %4, i64 1632`(x=0x660)·`i64 1640`(y=0x668) gep 가 **0건**이다. %4 는 m10.ll:33929 `store ptr %4, ptr %33` 으로 클로저 환경에 담겨 넘어가므로 좌표 사용이 있다면 그 클로저 쪽이고 이 본문의 '직접'이 아니다 | 4 |
| `03` | defensive_crisis | 5 | 6 | debug |  | 본문에서 직접 안 씀. check_kill_die_tick 에만 넘김 | 4 |
| `04` | handle_line_defense | 0 | 1 | _version |  | 본문에서 전혀 안 쓰임(이름 앞 _). AI 버전 게이트 분기 없음 | 4 |
| `04` | handle_line_defense | 1 | 2 | rnd |  | 이 함수가 직접 소비하지 않는다. PlayerState::strategy(575줄)에 그대로 넘김 | 4 |
| `04` | handle_line_defense | 2 | 3 | player |  | info.team(+0x930)만 직접 읽고, 나머지는 피호출 함수로 전달 | 4 |
| `04` | handle_line_defense | 3 | 4 | data |  | cache(+0x0)/context(+0x8)/blackboard(+0x10) 세 참조의 묶음 | 4 |
| `04` | handle_line_defense | 4 | 5 | line |  | 0=Top 1=Mid 2=Bottom (dienum LineType). alloca %10에 담겨 &line 으로도 전달됨 | 3 |
| `04` | handle_line_defense | 5 | 6 | _debug |  | readnone — 본문에서 안 씀 | 4 |
