# 8차 배치 C — 축 ``sig.params[].role` — 인자의 역할` · 함수 `05`~`09` (31행)

> `mkdossier.py --axis` 생성물. **칸을 하나도 줄이지 않았다** — 7차까지 지시 오류는 전부 「줄여 적은 자리」에서 났다.

| # | 함수 | idx | i | name | ty | role | ev |
|---|---|---|---|---|---|---|---|
| `05` | v50_fold_dive_episode | 0 | 1 | self |  | 상태변경 대상 | 4 |
| `05` | v50_fold_dive_episode | 1 | 2 | _version |  | DWARF !35908 에만 존재. 최적화로 인자에서 제거됨(#dbg_value(i64 poison)) — 본문에서 안 씀 | 3 |
| `05` | v50_fold_dive_episode | 2 | 3 | _tps |  | DWARF !35909 에만 존재. 마찬가지로 poison — 본문에서 안 씀 | 3 |
| `05` | v50_fold_dive_episode | 3 | 4 | aborted |  | IR %1. 레코드 aborted 필드로 그대로 저장되고, abort_src 기록 여부를 게이트 | 4 |
| `05` | v50_fold_dive_episode | 4 | 5 | end_reason |  | IR %2. 레코드 end_reason 으로 저장. 값 7 은 '포기 집계 제외' 특례 | 4 |
| `06` | v2_response_retreat_stance | 0 | 1 | self |  | IR 인자로 안 넘어옴(#dbg_value(ptr poison)) — 본문에서 self 필드를 하나도 안 읽어 최적화로 소거됨 | 4 |
| `06` | v2_response_retreat_stance | 1 | 2 | version |  | IR %0. AI 버전 게이트. version<2면 즉시 RunAway. 그대로 클로저 캡처(&version)와 check_kill_die_tick 으로 전달 | 4 |
| `06` | v2_response_retreat_stance | 2 | 3 | rnd |  | IR %1. 이 본문에선 안 씀 — check_kill_die_tick 에 그대로 넘김 | 4 |
| `06` | v2_response_retreat_stance | 3 | 4 | player |  | IR %2. team/position 을 읽고, 클로저 캡처 + check_kill_die_tick 의 judger 로 전달 | 4 |
| `06` | v2_response_retreat_stance | 4 | 5 | data |  | IR %3. {cache, context, blackboard} | 4 |
| `06` | v2_response_retreat_stance | 5 | 6 | debug |  | IR %4. 이 본문에선 안 씀 — check_kill_die_tick 에 그대로 넘김 | 4 |
| `07` | sub_plan | 0 | 0 | (sret) |  | 반환값 out-ptr. 태그 i64 @+0x0 | 4 |
| `07` | sub_plan | 1 | 1 | self |  | 필드 1개: target_bush: Option<usize> (태그 @0x0, 값 @0x8) | 4 |
| `07` | sub_plan | 2 | 2 | version |  | 이 함수에선 분기에 안 쓰임. upgrade_item 으로 그대로 전달만 | 4 |
| `07` | sub_plan | 3 | 3 | rnd |  | 이 함수에선 직접 안 씀. upgrade_item 으로 전달만 | 4 |
| `07` | sub_plan | 4 | 4 | player |  | info.team(0x930), info.position(0x9c0) 만 읽음 | 4 |
| `07` | sub_plan | 5 | 5 | data |  | cache(0x0)=&AbstractGameWithCache, context(0x8)=&GameContext | 4 |
| `07` | sub_plan | 6 | 6 | goal_data |  | epic(0x78) 스탠스의 tick 두 개만 읽음 | 4 |
| `07` | sub_plan | 7 | 7 | _debug |  | readnone — 본문에서 전혀 안 씀 | 4 |
| `08` | is_end | 0 | 1 | self |  | readonly captures(none) — 본문에서 단 한 번도 역참조하지 않는다(gep 없음). focus_epic_only/vision_only/v46_flee 모두 미사용 | 4 |
| `08` | is_end | 1 | 2 | version |  | AI 버전 게이트. 이 함수 자체엔 분기 없고 v24_objective_setup_should_release_to_passive 로 그대로 전달만 됨 | 4 |
| `08` | is_end | 2 | 3 | _rnd |  | readnone — 미사용 | 4 |
| `08` | is_end | 3 | 4 | player |  | info.team(+0x930)만 읽고, 나머지는 필터 클로저로 전달 | 4 |
| `08` | is_end | 4 | 5 | data |  | cache(+0x0)/context(+0x8)/blackboard(+0x10) 세 필드 전부 사용 | 4 |
| `08` | is_end | 5 | 6 | team_plan |  | objective(+0x41f, 3B) 만 읽음. 이 함수에서 쓰기는 없다(이름이 take_* 지만 실제 mutate 없음) 근거: tcx 정본 sig 에 `mut` 없음 + 같은 명세의 `sig.tcx` 문자열과 자기모순이었다 + IR m10.ll:7655 의 `%5` 에 store 0건(gep 2곳 모두 load) | 3 |
| `08` | is_end | 6 | 7 | _debug |  | readnone — 미사용 | 4 |
| `09` | check_favorable_engage_formation | 0 | 1 | version |  | ★AI 버전 게이트가 **아니다**(이름만 version). 본문 분기 0 + **피호출자 2단이 모두 `i64 poison`** (LLVM 이 미사용을 증명) + 오라클 version 12종 × 400 시나리오 **결과 차이 0건** ⟹ **이 체인 전체에서 죽은 인자다**(2026-09-11 2차배치B). ~~1차의 '주 경로에서는 살아 있는 버전 게이트'~~ 는 관측은 맞고 **결론이 틀렸다**. 재구현 지침: 받아서 흘리기만 하면 되고 0 하드코딩도 결과 불변(단 시그니처 호환을 위해 인자는 유지) | 2 |
| `09` | check_favorable_engage_formation | 1 | 2 | player |  | info.team(0x930)·info.position(0x9c0) 만 읽는다 | 4 |
| `09` | check_favorable_engage_formation | 2 | 3 | data |  | cache(0x0)=&AbstractGameWithCache, context(0x8)=&GameContext. blackboard(0x10)은 안 읽음 | 4 |
| `09` | check_favorable_engage_formation | 3 | 4 | target_enemy |  | 교전 대상 적. x(0x660)/y(0x668)만 읽는다 | 4 |
| `09` | check_favorable_engage_formation | 4 | 5 | engage_range |  | 교전 사거리. +100000 한 뒤 제곱해 아군 참가 반경으로 씀. **호출부 8곳 전수 200000**(m13.ll:18805 / 35679 / 37141 / 39946 / 40936 / 41820 / 42834 · m15.ll:33271). ~~「유일 호출처 = should_disengage_object_hunt」~~ 는 거짓(2026-09-11 검증배치 B) | 4 |
