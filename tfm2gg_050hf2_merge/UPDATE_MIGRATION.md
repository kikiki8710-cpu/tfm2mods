# 게임 업데이트 오면 — 마이그레이션 대응 가이드

> 이번 세션(~2026-07-15)에 만진 것 = **① TFM2.gg 대시보드(0.5.0 병합본)** + **② 인게임 오버레이 모드 tfm2_draft_overlay**.
> 업데이트가 오면 이 문서 순서대로. 현행 버전·SDK 단일 출처 = `MEM\CURRENT.md` (아래 값은 기준일 2026-07-15 = **게임 0.5.0_3 / SDK sdk_050_hotfix2 / mod toolchain nightly-2026-06-16 / probe toolchain nightly-2026-05-24**).

---

## 0. 먼저 판정: 핫픽스인가 메이저인가

`MEM\CURRENT.md` 의 buildid 와 새 exe buildid 비교. 판정은 **version-migrator 에이전트**에 맡기는 게 정석(exe↔exe 스켈레톤 해시 비교로 함수별 RVA-only/로직변경/구조변경 판정).

- **핫픽스급(RVA-only 재링크)** = 0.5.0_2→0.5.0_3 이 그랬음. 함수 주소만 이동, 구조체 오프셋·로직 불변. → **상수 교체 + 재빌드**로 끝.
- **메이저(구조 변경)** = 0.4.14→0.5.0 류. 세이브 포맷/struct/SDK 변경. → 아래 각 항목의 "메이저" 경로.

---

## 1. 대시보드 (TFM2.gg 0.5.0 병합본)

파이프라인 = **save_probe.exe**(세이브 바이너리→debug.txt, ★버전 민감) → build_meta_data.py(debug.txt→meta-data.js) → app.js. **버전 임계점은 save_probe 하나뿐.**

### 항상 (핫픽스든 메이저든)
1. **새 SDK 확인** = `MEM\CURRENT.md` 갱신값(예: `sdk_050_hotfix3`). 없으면 정식 Releases에서 받아 `C:\tfm2mods\sdk_XXX\` 로.
2. **save_probe 재빌드**:
   ```
   powershell -File C:\tfm2mods\tfm2gg_050hf2_merge\build_probe.ps1 -Sdk "C:\tfm2mods\sdk_<새버전>\mod-sdk"
   ```
   - toolchain 은 SDK `toolchain_version.txt` 에서 자동 도출. **★probe 는 game_core rlib 링크라, mod DLL 빌드용 toolchain 과 다를 수 있음**(hotfix2 기준 probe=nightly-2026-05-24 / mod=06-16).
   - 산출물 = `payload\tfm2_meta_dashboard\tools\tfm2_save_probe.exe`.
3. **배포**: `powershell -File apply.ps1` (워크샵 대시보드 자동탐지→미러복사→BOM검증).
4. **검증**: 대시보드에서 세이브 선택 → `_last_refresh_log.txt` 에
   `Save probe: received` + `Banpick agent: … meta_weights` + `Candidate map: N/N` + `Unavailable champions: …` 확인.

### save_probe 가 컴파일/로드 실패하면 (메이저 신호)
- **컴파일 실패**(Database 필드명 변경): `tfm2_save_probe_047_050hf2.rs` 의 `database.<field>` 이름을 새 game_core 에 맞춰 수정. (대개 teams/athletes/match_replays/base_network/matches 등은 안정.)
- **`full load failed … salvage`**: SDK 불일치. 다른 0.5.x/새 SDK 로 재빌드. 그래도 안 되면 세이브 후미 필드 드리프트 → salvage 경로가 부분복구(정상 작동, banpickAgent 만 빠질 수 있음).

### 빌더/프론트 (build_meta_data.py / app.js / main.cjs)
- **대개 안 건드림** — debug.txt 텍스트 파싱이라 버전 무관.
- 단 debug.txt 의 struct Debug 포맷(필드명)이 바뀌면 빌더 파서가 헛읽을 수 있음. `champions=` 수·`Patch impact`·`Banpick agent` 로그가 비정상이면 그 파서만 점검.
- **candidateIndex**(게임학습점수)·**인게임 미추가 챔프 제외**·**활성 모드 필터**는 champion_info_sheet/mods.json 기반 = **자동, 패치 무관.**

---

## 2. 인게임 오버레이 모드 (tfm2_draft_overlay)

소스 = `C:\tfm2mods\tfm2_draft_overlay\` (Steam 관리 밖). UI = `gen_popup.py`→`ui_inject\draft_popup.ui`(`include_str!`). 챔프UV = `gen_champ_uv.py`→`champ_uv.rs`(`include!`).

### 핫픽스 (RVA-only)
1. **version-migrator 로 RVA 판정** → 이동한 것만 `lib.rs` 상수 교체(정정형, 버전태그):
   - `LOADER_RVA`(L359, 에셋게터), `PARSER_RVA`(L360, .ui→NodeTemplate), `ALLOC_RVA`(L361, 게임 alloc), `ANIM_GET_RVA`(L142, #anim map). ← **4개가 마이그 대상.** (현재 0.5.0_3: 0x51cd40 / 0x2499f30 / 0x25ab3d0 / 0x51bbc0)
   - ANIM_GET 는 MULTI-family(string-xref 불가)라 상대유도(LOADER−0x1180) — 판정 시 주의.
2. **DBG=false 확인**(`lib.rs` L449) 후 재빌드:
   ```
   powershell -File C:\tfm2mods\build_inj.ps1 -Src C:\tfm2mods\tfm2_draft_overlay\src\lib.rs -ModId tfm2_draft_overlay
   ```
3. **인게임 검증**: 밴픽 화면 팝업 표시 + 아래 §2.5 동작 전부.

### 메이저 (구조 변경) — ★UI 오프셋·히트/z 체계가 바뀌면 여기가 위험
이번에 Ghidra 로 확정한 **UI 엔진 오프셋/RVA 가 struct 변경 시 이동**할 수 있음. 재RE 대상(ghidra-re 에이전트):
- **히트테스트**: `build_hit_tester` 0x247ae70 / `HitTester::build` 0x247a2e0 / `test` 0x247a1b0 / `is_blocked` 0x247a140 / 이벤트 디스패처 0x60ea90.
- **렌더 z**: RenderCommand.z(기본 100), z머지 0x9de080 / dispatcher 0x9d3470. 러너 block_event = vtable+0x130(ColorIconButton=0x541890 무조건 true).
- **엔진 Node 레이아웃**: stride 0x268, children ptr@+0x20/len@+0x28, rect@+0x240, visible@+0x260, unk@+0x261, runner@+0x230/vtable@+0x238.
- **호버**: `MatchUIRunner::update`(occlusion 없음, tooltip.visible 직접세팅). 툴팁 id = player_tooltip/champion_position_tooltip/champion_tooltip/fearless_tooltip.
- **DraggablePopup**: 드래그밴드 +0x1c0, ignore_event +0x1d0. **ImageRunner** source(cap@+0x00/ptr@+0x08/len@+0x10), 슬롯 stride 0xd0. (상세 정본 = `ANA\tfm2-draft-overlay-mod.md`.)
- 이 값들이 바뀌면 lib.rs 의 raw 오프셋(hide_hover_tooltips 는 SDK 필드라 안전 / raise_to_top 는 child Vec 이라 안전 / 하지만 z/히트 로직 자체는 위 규명에 의존)을 새 값으로.

### §2.5 인게임 검증 체크리스트 (재빌드 후 반드시)
1. 밴픽 진입 → 팝업 뜸 + **5개 탭**(메타통계/챔피언정보/메타해석/밴픽코치/모의밴픽)
2. **빈 영역 클릭이 뒤 게임 UI로 안 샘** (투명 차단막 #ov_blocker, 명시 px)
3. **헤더 잡고 드래그** 됨
4. **선수 칸 호버 후에도 클릭·드래그 유지** (raise_to_top) + 호버 시 선수 툴팁 안 뜸
5. **자체 스크롤바**(우측 청록) 표시·스크롤 따라감
6. 모의밴픽 탭: 컨트롤 버튼·가중치 ±·추천 클릭 픽·완료 시 분석
7. 대시보드 토글 버튼(우상단, 톱니바퀴 옆) 정상

### champ_uv.rs / 이미지 (모드챔프)
- 새 모드챔프/신규 챔프 추가 시: `gen_champ_uv.py`(시스템 python+PIL) 재실행 → champ_uv.rs 갱신 후 재빌드. 대시보드용은 `TFM2_DASH_ROOT` env 로 워크샵 대시보드 지정.

---

## 3. 요약 체크리스트

| 상황 | 대시보드 | 오버레이 모드 |
|---|---|---|
| **Steam 워크샵 재동기화**(파일 되돌림, 게임 동일) | `apply.ps1` | (영향 없음, 소스 별도) |
| **핫픽스**(buildid만 변경) | `build_probe.ps1`(새 SDK) → `apply.ps1` | version-migrator → 상수4 교체 → `build_inj.ps1` |
| **메이저**(세이브/struct 변경) | 위 + 빌더 파서 점검 + probe 소스 필드명 수정 | 위 + §2 메이저 UI 오프셋 재RE(ghidra-re) |

- 배포 전 필수: **probe·mod dll 은 Length+LastWriteTime 검증**(build 스크립트가 함) / **mod DBG=false** / **텍스트 파일 UTF-8 no-BOM**.
- 매 마이그 후 `MEM\CURRENT.md`·`MODS\MIGRATION.md §7` 갱신(record-keeper).
- 상세 정본: 대시보드=`MEM\tfm2gg-dashboard-save-probe.md` / 오버레이=`ANA\tfm2-draft-overlay-mod.md` / 이 키트=`README.md`.
