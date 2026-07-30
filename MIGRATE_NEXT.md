# 다음 패치 마이그 대상 사전 정리 (베이스 = 0.5.2)

> 작성 2026-07-29. **베이스 = 0.5.2 (buildid 24310934, exe 69,209,088B, sha256[:16] `40b55c1b819dff50`)** — 릴리스 zip = `<게임설치>\mods\release\0.5.2\`.
> 이 문서 = **"어디를 고쳐야 하나"의 사전 지도**(패치 오면 이 순서대로). RVA **값**의 정본은 `MIGRATION.md §7.2`, 현행 상태 정본은 `MEM\CURRENT.md`. 여기엔 값을 적지 않고 **위치(파일:라인)와 개수·함정만** 적는다.

---

## 0. 패치 도착 시 공통 선행 작업 (모드 무관)

1. **현 exe 백업** → `_N\` (migrate_rva.py 입력). 신 exe buildid·크기·sha256 기록.
2. **패치 성격 판정** = version-migrator 에이전트(exe↔exe 스켈레톤 해시). 0.5.1→0.5.2는 "전역 델타 없는 함수 재정렬" = **델타 더하기 금지**가 확정 규칙.
3. **SDK 확인**: GitHub 릴리스 zip → `C:\tfm2mods\sdk_<ver>\` 전개 → `build_inj.ps1` L29 `$SDK` 전환 + `base_version.txt`.
   - ★**rlib 내용이 바뀌면 하드코딩 RVA 0인 모드도 전부 재빌드 필수**(0.5.2가 그랬음: mod_api·engine·engine_asset·engine_core·engine_ui 5종 DIFF, engine_network만 동일).
   - toolchain(rust-toolchain.toml)이 바뀌면 rustup 설치 먼저. 0.5.2 기준 = nightly-2026-05-24.
4. **빌드 플래그**: rustc 명령줄에 직접 `-C opt-level=1 -C overflow-checks=off` (opt-level 2/3 = 스택오버플로 크래시 확정).
5. **panicmap 재생성**: `C:\tfm2mods\panicmap\` — `panic_sites_<ver>.csv`는 **버전 종속**. 크래시 진단 전제물.
6. **`.ui` 번들 재추출·diff**: `tools\bundle\extract_bundle_ui.py` → base `ingame.ui` 등이 바뀌었으면 override 재머지(Spectator_Chat).
7. **asset-get `.ui` 로더 copy 재확인**(string-xref): 0.5.1은 copy 2개 분화(main·strategy), 0.5.2는 단일 `0x5ac950`으로 수렴. **분화 여부가 버전마다 바뀐다** → UI 주입 모드 전부 영향.

---

## 1. T1 = 마이그 대상 8종 (유저 지시 2026-07-22)

### ①`tfm2_ai_adjust` — 최대 물량. 항상 여기부터.

| 위치 | 대상 | 개수 |
|---|---|---|
| `src\rva_052.rs` | **RVA 단일 수정점**(다음 버전엔 `rva_053.rs`로 복사 후 갱신 + `tfm2_ai_adjust.rs` L25 include 갱신) | 19 |
| `src\tfm2_ai_adjust.rs` L2524·2534-2535·2545-2548 | LANE_GATE / T3_GATE_A·B / CALL_PUSH_A·B / CALL_JOIN_A·B | 7 |
| `src\tfm2_ai_adjust.rs` L2728·2734·2745·2751 | D19_SLOT2_EMPTY / D19_STATIC_TEMPLATE / D19_STATIC2_TEMPLATE(구값 유지 중) / D19_TV7 | 4 |
| `src\detour.rs` L880 | SIMUNCHUNK (rayon bridge 사이트) | 1 |
| `src\mem_safety.rs` L310 | TEXT_END (PE .text vsz_end 실측값) | 1 |
| `src\genbuild_repro.rs` L698-699 | GB_ATKCTX 2종 (`&PTR_…` 데이터) | 2 |
| `src\ui_inject_embed.rs` L24-26 | uinj LOADER / PARSER / ALLOC | 3 |
| `src\detour.rs` `apply_objective_imm()` L650~ | **byte-patch oi_\* 12사이트** + 컨테이너 4 | 12 |
| `src\detour.rs` `apply_gb_imm()` L742~ | **byte-patch gb_\* 10사이트** + 컨테이너 6 (`gb_join_phase` 2사이트는 0.5.2에서 삭제=死) | 10 |
| `src\detour.rs` `apply_sev_imm()` L809~ | **byte-patch sv_\* 29사이트**(severity 4사본) + 컨테이너 4 | 29 |
| `src\disc19_repro.rs` `apply_disc19_imm()` L23~ | **byte-patch d19 10사이트**(전부 disc19 본체 내부) | 10 |
| `src\tfm2_ai_adjust.rs` (구조체 오프셋) | ROSTER_BASE/STRIDE·E_POSX/HP/SPEED·O_BLUE/RED_STRAT·**O_ATHLETE_ID** | 오프셋 |

**함정**
- ★byte-patch는 **주소만 갈고 prefix 배열(레지스터 인코딩)을 안 고치면 조용히 skip** → `applied=0/N` = 노브 사망(0.5.2에서 실제로 40/41이 죽어 있었음). 로그의 `applied` 카운트를 반드시 확인.
- ★**부분 재핀 금지** — 임계 일부만 사용자값이면 판단 일관성 파괴. 전량 확정 or 전량 유지.
- 대형 AI 함수(1300~4800 instr)는 스켈레톤 md5(L1)가 거의 실패 → **니모닉 멀티셋 코사인(L4)** 사용.
- 判斷 imm(tr/hp/ally/reach/lane_margin/vis) 자체는 0.5.2에서 전부 불변이었음 → 재현식 상수 재도출은 보통 불요, **위치만** 이동.
- shadow-CALL 대상 RVA를 미마이그 상태로 두면 **this 포인터 오염 = 크래시**(disc14 진범). 게이트 OFF라도 상수는 갱신할 것.
- 설정편집기(`C:\tfm2mods\ai_adjust_editor\`)는 별도 exe — 레버 목록이 바뀌면 같이 재빌드.
- **사이즈가드 초과** → build_inj.ps1 대신 rustc 직접 + `Copy-Item`(이때도 opt 플래그 필수).

**검증**: 로그의 `[patch] N/N patched+VERIFIED` + `d19_imm`·`oi`·`gb`·`sv` 카운트 전량 + 크래시 0 + disc0/1/3 400/400 DIFF 0 재측정.

---

### ②`tfm2_item_tactics`

| 위치 | 대상 | 개수 |
|---|---|---|
| `src\lib.rs` L32·1179·1772·1813·1928·1976·2102·2143·2658·2706·3953·3975 | FN_DD_SETOPT / SETTER_NOP(OFF) / REALLOC / CL_LAUNCHER / SEEDCTOR / SPAWN / SIM(OFF) / VIEW / BUY_ITEM / ITEMNET_FORWARD / CAND_GATE(OFF) / SLOT_HELPER | 12 |
| `src\ui_inject.rs` L20-24 | LOADER / STRAT_LOADER / PARSER / ALLOC / DEALLOC | 5 |
| `src\lib.rs` (시그·사이트) | SLOT_BOUNDS ×4 · owned_cap sig+imm · gate3 sig+jbe · launcher retaddr A/B/comptest | ~9 |

**함정**
- ★**AUTO4 shadow-CALL이 살아 있는 모드** = 최대 리스크. RVA 하나라도 stale이면 AV.
- `LOADER_RVA == STRAT_LOADER_RVA`면 세컨드 훅 스킵 가드가 발동해야 함(같은 주소 이중 설치 = 자기체인/본문 2회 실행). **패치로 다시 분화되면 가드 조건 재검토**.
- SPAWN은 0.5.2에서 push 8→7로 로직 변경돼 ORIG_LEN 12→15·`install_detour_r11` 사용 중. 프롤로그 길이 재확인 필수.
- retaddr류는 **컨테이너 델타·콜 서수 둘 다 오답** → `mig4.py`(명령어 정렬) 방식만 신뢰.

---

### ③`community_reaction_mod` (crm)

- **하드코딩 RVA 0개.** 재핀 대상 = `src\lib.rs` L34, L401-406의 **ClientDatabase raw 오프셋**(scene +0x1338 / MatchType +0x1818 / match id +0x1820 / match_info.id +0x17F8 / events Vec +0x1670·1678·1680).
- **작업 = sdk 재빌드 + 오프셋 불변 검증**뿐.
- ★**검증법**: mem-operand disp **센서스 금지**(ClientDatabase 계열은 값이 흔해 판정 불가). **마스크 시그로 witness 함수 A/B/C를 재핀 → 창 바이트 diff 0** 방식. 보조 = PDB TPI(아래 Spectator_Chat 절).
- ⚠**배포처 2곳**: 로컬 `mods\community_reaction_mod\`(mod_info 없는 단순 미러) + ★**authoritative = 워크샵 `steamapps\workshop\content\3009300\3738958482\`**(수동 복사).

---

### ④`tfm2_banpick_illust` (쇼케이스 통합판 v1.3.2)

| 위치 | 대상 | 개수 |
|---|---|---|
| `src\showcase.rs` L19-21 | **훅 3**: FX_SET / CARD_DRAW / ILLUST_GET | 3 |
| `src\showcase.rs` L22-35 | **FFI 함수 14종** | 14 |
| `src\showcase.rs` L57-69 | **기하 패치 사이트**: .rdata 상수 6 + 코드 즉치 5 + 슬롯 1 | 12 |

- 상세 시그니처·cmd 필드맵·기하 패치 바이트 = `C:\tfm2mods\tfm2_banpick_showcase\FFI_CONTRACT.md`(정본, 소스 폴더는 계약 보존용).
- 기하 패치는 **12/12 사전검증 통과 시에만 적용**(실패 = 밴 360 폴백) → 부분 실패해도 크래시는 안 나지만 연출이 원본으로 돌아감.
- ⚠공유 상수 `0x1436e8e98`(0.82) = 폴백 일러 스케일과 공유 → **패치 금지** 사이트. 재핀 시 동일 판정 유지.
- ★**사이즈가드 초과**(ChampionInfoSheet 정적 링크 2.8MB) → rustc 직접 빌드 경로.
- 아트팩/모드챔프 3단계 폴백은 데이터 경로라 RVA 무관.

---

### ⑤`tfm2_draft_overlay` — ⚠**0.5.2 마이그가 실제로 안 돼 있음 (이번에 발견)**

| 위치 | 상수 | 현재 값의 출처 |
|---|---|---|
| `src\lib.rs` L142 | ANIM_GET | **0.5.1** |
| `src\lib.rs` L359 | LOADER | **0.5.1** (0.5.2 = uinj LOADER 신값) |
| `src\lib.rs` L365 | BANPICK_LOADER | **0.5.1** (0.5.2는 copy 병합돼 별도 주소 없음) |
| `src\lib.rs` L366 | PARSER | **0.5.1** |
| `src\lib.rs` L367 | ALLOC | **0.5.1** |

- 근거: 소스 mtime **2026-07-19 00:11**, 배포 dll mtime **07-19 00:13**, 소스 내 "0.5.2" 문자열 **0건**, `release\0.5.2\`에 zip 없음(0.5.1엔 있음). ⇒ **0.5.2 도착(07-22) 이후 손댄 적 없음 = sdk_052 재빌드도 미실행.**
- `MEM\CURRENT.md` L38의 "배포완"은 이 실측과 어긋남 → **다음 패치 작업 전에 먼저 0.5.2 기준으로 정정 마이그**하거나, 어차피 새 버전으로 갈 거면 **신 버전 기준으로 5개 상수를 신규 재핀**하면 됨(0.5.2를 경유할 필요 없음).
- 함정: BANPICK_LOADER는 **밴픽 화면 전용 copy** — 0.5.2에서 단일 copy로 병합됐으므로 신 버전에서도 **밴픽 경로의 asset-get copy를 string-xref로 재확인**해야 한다(분화/병합 여부가 버전마다 다름). 병합이면 세컨드 훅 스킵, 분화면 item_tactics와 **체인 후킹**(덮어쓰기 금지).

---

### ⑥`tfm2_elemental_serpen` (v0.4.1)

| 위치 | 대상 | 개수 |
|---|---|---|
| `src\lib.rs` L34·350·405·414·420-425 | SERPEN / MOBATICK / SPAWN_HOOKS[2] / LAUNCHER / **LAUNCHER_RET_A·B·C** | 8 |
| `src\lib.rs` L513-515 | UILOADER / UIPARSER / UIALLOC | 3 |
| `src\lib.rs` L717·744·1707·1710·1902·2427 | RENDER_STEP / RUNNER_CTOR / DMGA / DMGB / **KEYRES** / ARG_STR | 6 |
| `src\lib.rs` (기타) | BUILD / PUSH / BUFFAPPLY / ENTBUILD / DISP | 5 |
| `src\lib.rs` (보류 4종) | ANIM_LOOKUP·SHEET_LOOKUP·RENDER·SHEET = **死상수·inert, 건드리지 말 것** | — |

**함정**
- ★★**`KEYRES_PROLOGUE`도 반드시 동시 갱신**. 0.5.2에서 프롤로그 바이트가 바뀐 **유일한** 훅이었다(`sub rsp,0x70`→`0x60`). RVA만 갈면 프롤로그 검증 실패 → **훅 조용히 미설치 → 스프라이트 교체 전멸**(폴백 없음).
- LAUNCHER_RET_A/B/C = retaddr → `mig4.py` 명령어 정렬만 신뢰(단순 델타·콜 서수 오답).
- `LAUNCHER_RET_C`는 **리플레이(다시보기) 게이트** — v0.4.1 신규분이라 이전 표에 없을 수 있음. 누락 주의.
- ClientDatabase 계열 오프셋은 disp 센서스로 판정 불가 → crm/Spectator_Chat 세션 결과와 교차확인.

---

### ⑦`tfm2_fog_damage_fix`

- 재핀 대상 = `src\lib.rs`의 **byte-patch 5사이트**(L52 A / L57 B / L66 native / L71 data / L75 v3) + 참조 착탄 함수.
- 함정: ★**"로직 동일 ≠ 인코딩 동일"** — 0.5.2에서 사이트 B가 `sete r8b`→`sete dil`로 **레지스터만 바뀌어 orig/fixed 바이트를 둘 다 고쳐야 했다**. 주소만 갈면 byte mismatch로 조용히 skip.
- ★**코어 native↔data 게이트는 바이트 쌍둥이** → 시그 대조로 구분 불가. **panic-Location(소스경로:행번호) 추출로 판정**(도구 `scratchpad\locs.py`).
- ★**신설 4번째 시야 게이트(0.5.2 `0x2367c3f`)는 의도적 미패치** — AI 교전 타겟 필터라 무력화하면 전지적 AI가 됨. 신 버전에서도 **"새 게이트 발견 = 패치 대상"으로 착각 금지**.
- ⬜기존 잔여: 배포본 오염 의혹(`ANA\100퍼-잔여-트래커.md` #0c) → 어차피 재빌드하면 해소.

---

### ⑧`Spectator_Chat`

- **하드코딩 RVA 0개.** 재핀 대상 = `src\lib.rs` L584-585 raw 오프셋 2개(`LIVE_PLAYED_OFF` / `LIVE_EVENTS_OFF`).
- ★**검증 1순위 = PDB TPI 정적 재도출**: `-C debuginfo=2` **별도 빌드**(build_inj.ps1은 pdb를 안 만듦) → `tools\pdb\tpi_dump.py`. 산식 = `ClientDatabase.scene + 8 + GameView.played_tick` / `+ GameClient.events`.
- 같은 TPI 덤프로 **UI Runner 오프셋**(DraggablePopupRunner 0x1c0~0x1cc, LabelRunner text +0x160 = `ui_kit.rs off::LABEL_TEXT`)도 한 번에 검증 → **ui_kit 쓰는 전 모드가 이 결과에 의존**.
- `mod.override_info` + `ui\layout\ingame.ui` override → **base `ingame.ui`가 패치로 바뀌면 재머지 필수**(0.5.2는 diff 0이라 불요였음).
- ⚠**`chat_lines.txt`는 load-bearing**(런타임 로드) — 릴리스 zip에서 "런타임 txt"라고 제외하면 사고.

---

## 2. T1 외 — 재빌드/재핀이 필요한 것 (카운트에 안 잡혀 누락 주의)

| 모드 | 하드코딩 RVA | 위치 | 비고 |
|---|---|---|---|
| `tfm2_comptest_unlock` | **바이트패치 14 + 훅 16 + 死상수 10** | `src\tfm2_comptest_unlock.rs`(28 선언) + `src\ui_inject.rs`(3) | 유저 명시 요청분. ★`server_dedup_real`은 0.5.2에서 **jne rel32 변위가 바뀜** = 시그도 갱신 필요. 금지 사이트(라인업 10슬롯 상한)와 같은 컨테이너 → **패턴 검색 금지, 명령어 인덱스 정렬 방식만** |
| `tfm2_banpick_order` | **17** (훅 5 + 적용기/전이 5 + AI 파리티 4 + 배너 1 + 패닉훅 1) | `src\hooks.rs` L23-25·275-279·299-302·310·337·355·369 / `src\diag.rs` L636 | ★훅 D(`RVA_TURN`)는 **13B 특수 트램폴린**(조건분기 재배치) — 일반 12B 루틴 재사용 금지. 훅 E(`RVA_LINEUP`)는 크래시 회피 핵심 |
| `tfm2_level_cap` | **2** | `src\lib.rs` L82(RVA_LEN_LOAD)·L88(RVA_UI_CMP) | 데이터 merge가 아니라 런타임 트램폴린 2사이트 |
| `tfm2_transfer_tweak` | **2** | `src\lib.rs` L43(RVA_GATE)·L44(RVA_TBL) | ⬜인게임 미검증 상태 |
| `ui_kit`(공용) | **1** | `ui_kit.rs` L911 `DD_SETOPT_RVA` | ⚠**현재 0.4.x 구값으로 보임**(item_tactics·comptest의 동명 상수와 값이 다름). ui_kit 드롭다운 경로를 쓰는 모드가 생기면 즉시 문제 → 다음 마이그 때 **동기화 여부 판정** |
| `tfm2_meta_item_delegate` | 0 | — | **SDK 재빌드만**. 카운트에 안 잡히니 누락 주의 |
| 대시보드 `save_probe` | 0 | — | 〃 |
| daram2 뷰플러스 **9종** | 0 | — | 〃 (coaching_staff/custom_tier/facility/finance/legacy_save_patcher/recruitment/roster/statistics/training). banpick_view_plus는 유저 지시 SKIP |
| `tfm2_mod_order` | 0 | — | 〃 |

---

## 3. 재핀 방법 선택 (대상 성격별 — 0.5.2에서 검증된 것만)

| 대상 성격 | 1순위 방법 | 도구 |
|---|---|---|
| 작은 함수 | 스켈레톤 md5 L1 (UNIQUE면 즉시 확정) | `migrate_rva.py`, `mig_serpen.py` |
| 대형 AI 함수 | 니모닉 멀티셋 코사인 L4 | `mig2.py` |
| L1~L3 전부 NO-MATCH | **콜그래프 앵커링**(구 콜사이트 전수→콜러 매핑→정렬) | `mig4.py` |
| retaddr(콜사이트 복귀주소) | **명령어열 difflib 정렬** — 컨테이너 델타·콜 서수는 오답 | `mig4.py` |
| `.ui` 로더 등 문자열 참조 | string-xref | `mig_xref.py` |
| 바이트쌍둥이 함수 판별 | **panic-Location(소스경로:행)** | `locs.py`, `panicmap\` |
| ClientDatabase raw 오프셋 | **PDB TPI**(1순위) / 마스크시그 witness 재핀(2순위). **disp 센서스 금지** | `tools\pdb\tpi_dump.py` |
| struct 시프트 탐지 | .text 실 disp 센서스 | `mig6.py` |

---

## 4. 마이그 후 필수 검증

1. **프롤로그 전수 확인** — 신 RVA의 선두 12B(또는 선언 ORIG_LEN)가 모드 선언 상수와 일치하는지. 불일치 = 훅 미설치(조용히).
2. ★**착수 시 역방향 점검**: "구 RVA의 프롤로그가 모드 선언 상수와 실제로 맞았는지" 먼저 확인 → 안 맞으면 그 상수는 **이전 버전부터 이미 죽어 있던 것**(comptest에서 3건, serpen에서 2건 실증). 마이그가 아니라 **회귀 수정** 대상.
3. **byte-patch는 orig 바이트 대조 PASS**(`verify052.py` 계열) + 런타임 `applied=N/N`.
4. **배포 dll 바이트 스캔**: 신 주소 전량 존재 + 구 주소 0건.
5. **stale dll 판정은 SHA256으로** — Length가 우연히 같은 사례 실측됨(serpen 425,984B 2회).
6. **dll 크기 급감은 기능 누락이 아닐 수 있음** — opt-level 0→1 전환 계단(−40~48%). 07-18 이전 빌드본과 크기 비교로 판정 금지.
7. `mod.mod_info` **BOM 없는 UTF-8**(첫 바이트 `7b`) + `dependencies` 버전 범위 검토.
8. deploy-verify 에이전트 → 릴리스 zip = `<게임설치>\mods\release\<신버전>\<MOD_ID>.zip`(zip 루트에 `<MOD_ID>\` 한 겹, 개인·런타임 파일 제외).

---

## 5. 권장 작업 순서

```
0. 백업 → 성격 판정(version-migrator) → SDK/toolchain 전환
1. ai_adjust            (물량 최대·단일 수정점 rva_053.rs)
2. item_tactics         (shadow-CALL 리스크)
3. elemental_serpen     (KEYRES 프롤로그 동시 갱신)
4. banpick_illust       (훅3+FFI14+기하12)
5. draft_overlay        (★0.5.2 미마이그 상태 — 신 버전 기준으로 신규 재핀)
6. fog_damage_fix       (5사이트, 인코딩 변경 주의)
7. comptest_unlock      (바이트패치 14 + 훅 16)
8. banpick_order        (17개, 훅 D 13B 특수)
9. level_cap / transfer_tweak (각 2)
10. RVA 0 재빌드군: crm(워크샵 포함)·Spectator_Chat·meta_item_delegate·save_probe·daram2 9종·mod_order
11. 기록: MIGRATION §7.3 신설 + CURRENT.md 갱신(record-keeper)
```
