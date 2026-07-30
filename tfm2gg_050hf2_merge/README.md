# TFM2.gg 메타 대시보드 — 0.4.14 기능 병합 키트 (기준일 2026-07-15)

워크샵 대시보드(0.5.0 기반, 세이브 읽기)에 0.4.14 대시보드의 기능을 얹은 **재적용 키트**.
Steam 재동기화나 게임 패치로 원상복구돼도 이 폴더로 **한 번에 다시 적용**한다.

> ★**게임 업데이트가 오면 → `UPDATE_MIGRATION.md`** (핫픽스/메이저 판정 트리 + 대시보드·오버레이 모드 컴포넌트별 절차·검증 체크리스트). 이 README 는 재적용 상세, 그 문서는 마이그 진입점.

## 무엇이 담겼나 (07-14 초기 병합)
1. **프론트 3파일**(app.js/index.html/styles.css = 0.4.14): 밴픽 코치·모의 밴픽·티어 필터·정렬 컬럼·게임학습 슬라이더.
2. **빌더**(build_meta_data.py = 0.4.14 + 패치): `candidateIndex` 부여(champion_info_sheet 등장순=base_network 인덱스) → 모의밴픽 게임학습점수 / `--collect-assets-only`(세이브 없이 `#sheet.png` 모드챔프 수집).
3. **save_probe**(tfm2_save_probe.exe = 047 소스 재빌드): 0.5.0 세이브 full load + `base_network.debug.txt` + `save_file` manifest.
4. **main.cjs**(0.5.0 + `collectChampionAssets()`): 앱 시작 시 세이브 무관 챔프 이미지 수집.
5. **gen_champ_uv.py**(env `TFM2_DASH_ROOT`): raw `.aseprite` 모드챔프(leefs 8종) → PNG 렌더 + `mod_champ_assets.json`.

## 07-15 추가 수정 (대시보드 = app.js/build_meta_data.py/main.cjs)
6. **인게임 오버레이에 모의밴픽 탭 추가**(app.js `overlayBanpickSection`/`applyOverlayControl` bp_* / 모드 5번째 탭 — C절): 양팀 밴/픽 보드·추천·가중치·세트/룰·완료 시 라인업·팀 분석. exportOverlay 순서 = **추천 → 블루 → 레드**.
7. **overlay IPC 복구**(preload.cjs 신규 + main.cjs `ipcMain.handle("tfm2gg-overlay:write")` + overlay_control 워처): 07-14 병합이 preload 누락으로 끊었던 것. ★워처는 **시작 시점의 stale overlay_control.txt 무시**(안 그러면 `refresh=1` 잔재가 startup 에 `reload()` 호출 → **ERR_ABORTED(-3) 로 대시보드 안 뜸**) + 세이브 미선택 시 reload 안 함.
8. **밴픽 챔프 목록 정확화**(build_meta_data.py 2건 + app.js 그리드 필터 제거):
   - `configured_mod_ids()` = **`enabled_mods` 만**(구독만 한 `known_workshop_mods` 제외 → 꺼둔 모드 챔프 안 뜸).
   - **인게임 미추가 챔프 제외**: 번들(banpick-data.js)에만 있고 세이브 `champion_info_sheet` 에 없는 base 챔프(alchemist/crossbowman/nightmare/sand_mage) 제거 → **초상화 공백 사라짐**. (`champion_info_sheet` = 지금 게임에 실재하는 챔프 목록.)
   - 그리드(모의밴픽·밴픽코치)에서 **통계 유무 필터(`bpHasData`) 제거** → 방금 켠 모드 챔프도 통계 없이 즉시 표시(추천 계산엔 필터 유지).

## 폴더 구조
```
tfm2gg_050hf2_merge\
  apply.ps1            ← ★재적용(자동탐지→미러복사→BOM검증→이미지 재수집)
  build_probe.ps1      ← 게임 패치 시 save_probe 재빌드
  tfm2_save_probe_047_050hf2.rs   probe 소스(base_network export + save_file)
  payload\             타겟(resources\app) 구조 미러 — apply 가 이걸 복사
    main.cjs
    tfm2_meta_dashboard\ app.js, index.html, styles.css, mod_champ_assets.json,
                         tools\{build_meta_data.py, tfm2_save_probe.exe},
                         assets\mod-champions\*.png (leefs 8종 baked)
```

## A. Steam 재동기화로 되돌려졌을 때 (게임 버전 그대로)
```
powershell -ExecutionPolicy Bypass -File C:\tfm2mods\tfm2gg_050hf2_merge\apply.ps1
```
→ 워크샵 대시보드 자동탐지 후 payload 복사 + 검증 + 이미지 수집. 대시보드 재시작하면 끝.
(대상 못 찾으면 `-DashApp "<...\DashboardApp\resources\app>"` 로 지정.)

## B. 게임이 패치됐을 때 (세이브 포맷/SDK 변경)
1. 현행 SDK 확인 = `MEM\CURRENT.md` (예: `sdk_050_hotfix2`).
2. save_probe 재빌드:
   ```
   powershell -ExecutionPolicy Bypass -File build_probe.ps1 -Sdk "C:\tfm2mods\sdk_<현행>\mod-sdk"
   ```
   (툴체인은 SDK toolchain_version.txt 에서 자동 도출. ★rlib 링크라 모드빌드용 06-16 아님.)
3. `apply.ps1` 실행 → 배포.
4. 검증: 대시보드에서 세이브 선택 → `_last_refresh_log.txt` 에 `Save probe: received` + `Banpick agent: … meta_weights` +
   `Candidate map: N/N` 확인. 게임학습점수/밴픽코치/이미지 정상인지 눈으로 확인.
   - save_probe 가 `full load failed … salvage` 로 뜨면 SDK 불일치 → 다른 0.5.x SDK 로 재빌드.
   - Database 필드명이 바뀌어 컴파일 실패하면 소스(.rs)의 field 이름을 새 game_core 에 맞춰 수정(대개 base_network/matches 등은 안정).

## C. 인게임 오버레이 모드 (tfm2_draft_overlay) — 대시보드와 연동
밴픽 화면에 대시보드 메타분석을 띄우는 별개 네이티브 모드. 대시보드와 파일로 연동:
- 대시보드 `main.cjs`(+`preload.cjs`)가 `overlay_data.txt` 쓰기 / `overlay_control.txt` 읽기(양방향).
  ★이 병합의 `payload\main.cjs`+`preload.cjs`에 그 IPC 포함(app.js `window.tfm2ggOverlay`).
- 모드는 `champ_uv.rs`(gen_champ_uv.py 생성)를 `include!` → 챔프 스프라이트.
- 모드 소스 = `C:\tfm2mods\tfm2_draft_overlay\` (Steam 관리 밖이라 재동기화 영향 없음). UI = `gen_popup.py` → `ui_inject\draft_popup.ui`(모드가 `include_str!`).

### 07-15 오버레이 기능·UI (모두 배포 완료, 소스에 반영)
- **모의밴픽 5번째 탭**: `gen_popup.py` NUM_TABS=5 + 컨트롤 패널(#bppanel: 세트/룰/밴수/진영 토글버튼·가중치 ±버튼·패치보정), lib.rs 탭4 렌더/라우팅.
- **가로 1.2배(816px)** + 그리드 10열(NGRID 120). 값 필드 폭 확대(개행 방지).
- ★**밴픽 화면 z/히트/호버 함정 (ghidra 확정 — 재빌드 시 반드시 유지)**:
  - **z 는 히트테스트와 무관**. 히트 승자 = children **DFS 순서상 가장 나중** 노드 1개(소비, 전파 없음). `build_hit_tester` 0x247ae70 / `HitTester::test` 0x247a1b0.
  - **빈 영역 클릭이 뒤 게임 UI로 새는 것** 차단 = 투명 `color_icon_button`(`#ov_blocker`)를 팝업 첫 자식으로. **반드시 명시 px(816×824)** — `width:100%` 는 rect 0 으로 접혀 히트 후보 등록 실패. **헤더(상단 44px)는 제외**(draggable_popup 드래그 밴드 보존).
  - **호버는 클릭과 다른 경로**(MatchUIRunner::update, occlusion 없음) → 차단막으로 못 막음. `post_update` 에서 `hide_hover_tooltips`(player_tooltip 등 재귀 visible=false).
  - **★선수 칸 호버 후 클릭/드래그 뺏김** = 게임이 매 프레임 `move_child_after` 로 자기 노드를 children 뒤로 재배치(히트=DFS 최후 승자). 해결 = `post_update` 에서 **`raise_to_top`**(우리 `draft_root` 를 부모 children 맨 끝으로 remove+push). ⚠**순서 재배열(길이 불변)은 반영됨** — "런타임 트리 push 금지"는 *새 노드 추가*만 해당.
  - **네이티브 스크롤바 안 보임** = 스크롤바 렌더 커맨드가 노드 z 무시 → 콘텐츠(z=300)에 덮임. 해결 = **자체 스크롤바**(#sbar_track/#sbar_thumb `:color`, lib.rs 가 content_h·scroll_get 로 thumb 갱신).
- **게임 패치 시 이 모드도 재빌드 필요**(RVA 이동):
  1. RVA 마이그 판정 = version-migrator 에이전트(또는 이미 소스가 갱신됐는지 확인 — 상수는 lib.rs 상단 LOADER/PARSER/ALLOC/ANIM_GET).
  2. `const DBG: bool = false`(배포시 필수) 확인 후:
     `powershell -File C:\tfm2mods\build_inj.ps1 -Src C:\tfm2mods\tfm2_draft_overlay\src\lib.rs -ModId tfm2_draft_overlay`
  3. 인게임 밴픽 화면에서 팝업 표시 + 위 z/히트/호버 동작(클릭 차단·드래그·스크롤바·모의밴픽 탭) 확인.
- 상세 정본 = `ANA\tfm2-draft-overlay-mod.md`.

## 데이터 계약 (왜 이렇게 나뉘나)
파이프라인 = save_probe.exe(세이브 바이너리→debug.txt, **버전 민감**) → build_meta_data.py(debug.txt→meta-data.js) → app.js.
세이브 읽기의 유일한 버전 임계점 = **save_probe** 뿐. 나머지(빌더·프론트)는 debug.txt 포맷이 안정한 한 버전 무관.
그래서 "데이터=0.5.0(save_probe), 기능=0.4.14(빌더+프론트)".

## 함정 정리
- **UTF-8 no-BOM** 필수(app.js/index.html/styles.css/build_meta_data.py/main.cjs). apply.ps1 이 검증.
- **save_probe 툴체인** = SDK rlib 빌드 툴체인(hotfix2=nightly-2026-05-24). 모드 DLL 빌드용과 다름.
- **candidate 순서** = champion_info_sheet 등장순서 자동추출 → 패치로 챔프 추가돼도 코드 수정 불필요.
- **게임학습 synergy/counter** = base_network 에서 원래 전부 0(0.4.14도 동일·정상). 게임학습점수는 global+pos 로 계산.
- **.aseprite 전용 모드챔프** = PIL 렌더 필요(시스템 python). 워크샵 배포엔 baked png 포함(payload).
