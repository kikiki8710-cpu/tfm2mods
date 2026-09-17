# apply060 batch_09.md — 반영(logic_060) 브리핑 · 함수 1

★**version = 3 런타임 확정(09-17)** — v<3/v2 경로는 명세에서 제외(사장). 태그/오프셋/vt 슬롯 = `spec_patch_060.md` §A(dispcheck 추가 행 포함). 출력 형식·절차 = `mkapply060.py` 도크스트링. 출력 = `_next/apply060/out/<old>.md`.

RE 정본 파일: 2026-09-16_r19_실변경17_배치B_SmallAction_get_input계열6_디컴대조_원문.md

### `dc2960` → `ed0400` SmallActionAroundBush::get_input (i=168 · 변경(r19: ★로직 변경: path_finder 이름≠around_bush 면 폐기·재생성))
- 0.5.8 src: `game-ai\src\small_action\around.rs:1195` · one_line: 수풀 목표 셀까지 check_cell(지역 규칙+타워 회피) 술어로 PathFinder 경로를 만들고/갱신해 이동 입력 생성, 디버그면 경로선·지역 로그 기록
- 0.6.0 판정: **한 줄** · 패치 요지: `if self.path_finder.as_ref().is_some_and(|pf| pf.name != "around_bush") { self.path_finder = None; }` 를 `if is_none { new_target }` 앞에 삽입
- RE 정본: `2026-09-16_r19_실변경17_배치B_SmallAction_get_input계열6_디컴대조_원문.md` · 절: `2026-09-16_r19_실변경17_배치B_SmallAction_get_input계열6_디컴대조_원문.md` §AroundBush::get_input 변경 구간
- dispcheck 변위 짝(구→신 · 규칙): 0x18→0x50(AroundBush(r19 로직 ) · 0x20→0x18(AroundBush(r19 로직 ) · 0x50→0x58(AroundBush(r19 로직 ) · 0x930→0xa00(PlayerState) · 0x9c0→0xa90(PlayerState)
- sig: `fn(&mut game_ai::SmallActionAroundBush, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input>`
- consts: [{"value": 32000, "src_line": 1197, "meaning": "월드→셀: target_x/32000, target_y/32000 (105453·105462) · 디버그 ex/32000, ey/32000 (105734·105739)", "kind": "계수", "ev": 4}, {"value": 29, "src_line": 1197, "meaning": "셀 인덱스 상한 clamp: min(x/32000, 29) (105457·105466, `llvm.umin`) — 30×30 격자", "kind": "인덱스", "ev": 4}, {"value": 960000, "src_line": 1213, "meaning": "30×32000 — 디버그 regions 인덱스 범위 검사(ey<960000, ex<960000 아니면 panic_bounds_check 30) (105735·105740)", "kind": "임계", "ev": 4}, {"value": 2, "src_line": 1200, "meaning": "Option<PathFinder>::None 니치 태그 비교(105475 등 4곳). player_champion 팀 인덱스 상한 2
- 0.5.8 logic 전문:
```
SmallActionAroundBush::get_input(&mut self, version, rnd, player, data, ps, debug) -> Option<Input>   [around.rs:1195]
 entity = data.cache.player_champion[player.team][player.position].unwrap()          (L1196)
 tcx = min(self.target_x/32000, 29) ; tcy = min(self.target_y/32000, 29)             (L1197~1198; 지역 변수, 클로저에 &참조)
 tower_dodge = TowerDodgeContext::new_tower_avoid_v3(version, player, data, self.target_x, self.target_y)   (L1199)
 if self.path_finder.is_none():                                                        (L1200)
     self.path_finder = PathFinder::new_target(rnd, data.context, version, "around_bush", entity.x, entity.y, self.target_x, self.target_y,
         |sx,sy,nx,ny| check_cell(version, player, ps, data, &tower_dodge, self.out_line, sx,sy,nx,ny, tcx, tcy))   (L1201; 클로저 환경 8칸: &version, player, ps, data, &tower_dodge, &self.out_line, &tcx, &tcy)
     if self.path_finder.is_none(): return None                                        (L1206 — new_target 결과 None 가능, 105492)
 pf = self.path_finder.as_mut()  (Some 확정)
 pf.update_path(rnd, data.context, entity.x, entity.y, self.target_x, self.target_y, 같은 클로저(s_0))   (L1206)
 if self.path_finder.is_none(): return None                                            (L1210, 105613 — update_path 뒤 재검사)
 input = pf.get_input(player, data, SafeMoveWithSkill::Safe, false)                    (L1210)
 if data.context.debug && input is Move{x:mx, y:my}:                                    (L1211~1212)
     debug.infos.entry(entity.id).or_insert(Vec::new()).push(format!("region: {}", map.regions[entity.y/32000][entity.x/32000]))   (L1213; ex,ey ≥ 960000 이면 panic)
     debug.add_line(entity.x, entity.y, mx, my, Color(1,0,1,1))                          (L1214)
 return Some(input)                                                                    (L1218)

클로저 계약(관측, PathFinder 인스턴스 m03.ll 17819~21356 / 32437~33531 안 인라인): check_cell 12인자를 위 순서로 호출(m03 21150·32711), 반환 PathVerdict 태그로 switch(0 Allow → 계속 / 1 Soft / 2·3·4 → 위반 처리 — path_finder 계층). out_line 은 self.out_line 을 매 호출 로드.
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
