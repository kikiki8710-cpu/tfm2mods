# 패치날 요약 — 0.7.0

> 생성 2026-10-28 19:34:01 · `MIG\patchday\run_patchday.py` · 전체 소요 467s · 출력 = `F:\claude\tfm2mods\MIG\_patchday\0.7.0`

## 0. 버전

| | 기준(구) | 신 |
|---|---|---|
| 라벨 | 0.6.5 | **0.7.0** |
| sha256 | `DAF040DB8A3A3AE136F3EF8CE704E2718F049656B6B5E4872EACB599811D30B5` | `D678E01285B7A9EDBA2C7EE1067CEA93A54F979D769EAAE9F0D8309D18FF7D30` |
| 크기 | 87009792 B | 93235712 B |
| exe | `F:\claude\tfm2\tfm2_0.6.5\TeamfightManager2.exe` | `F:\claude\tfm2\tfm2_0.7.0\TeamfightManager2.exe` |
| base_version.txt | — | `0.7.0` (C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mod-sdk-stable\base_version.txt) |

## 1. 단계 결과

| # | 단계 | 결과 | 종료코드 | 소요 | 메모 | 로그 |
|---|---|---|---|---|---|---|
| 0 | detect | 정상 | 0 | 1.3s | 0.6.5 → 0.7.0 · sha D678E01285B7A9ED · 93235712 B | [log](logs/00_detect.log) |
| 1 | backup | 정상 | 0 | 3.2s | TeamfightManager2.exe: 복사 완료 93235712 B · sha D678E01285B7A9ED / bundle.game_data: 복사 완료 1156602422 B · sha FF41053C02223FDB | [log](logs/01_backup.log) |
| 2 | fnindex | 정상 | 0 | 47.3s | 기준 _fnidx_065.pkl: 검증 통과 — 메타 경로 sha 일치 · 경계 154316/154316 일치 · skel 표본 48 일치 / 생성 후 검증: 메타 경로 sha 일치 · 경계 165552/165552 일치 · skel 표본 48 일치 | [log](logs/02_fnindex.log) |
| 3 | callgraph | 정상 | 0 | 28.9s | 기준 _cg_065.pkl: 검증 통과 — callee 136827개 · 표본 40 일치 / 생성 후 검증: callee 146469개 · 표본 40 일치 | [log](logs/03_callgraph.log) |
| 4 | aidiff | 정상·검토 필요 | 0 | 176.2s | EDITED 85 · MERGED 0 · 사라짐 17 · 새로 생김 25 | [log](logs/04_aidiff.log) |
| 5 | check | 정상·검토 필요 | 1 | 0.3s | STALE·INVALID 566 (모드 13개 중 10개) | [log](logs/05_check.log) |
| 6 | repin | 정상·검토 필요 | 0 | 149.9s | 대상 566 · 엔진 해결 463 · 실패 103 · 사람 판정 필요 213(apply 대상 34 + 생성물 모드 179) · 동형 클론 주의 9 | [log](logs/06_repin.log) |
| 7 | offsets | 정상·검토 필요 | 1 | 1.3s | 변화 6개 모드 · 33개 함수 (기준 = repin_map.aligned.json) | [log](logs/07_offsets.log) |
| 8 | detail_tactics | 정상·검토 필요 | 1 | 58.4s | 검토 필요 항목 있음(정상) · 전술 AUTO 25 · REVIEW 75 · 창 218(자동 52) · 기본값 변경 전술 0 | [log](logs/08_detail_tactics.log) |
| 9 | addrbook | 건너뜀 |  | 0.0s | --skip |  |
| 10 | summary | 정상 | 0 | 0s | SUMMARY.md |  |

## 2. AI 판단계층 (aidiff)

- ★**EDITED 85개** · MERGED 0 · SHIFTED 353 · IDENTICAL 292 · 사라짐 17 · 새로 생김 25 → 전문 [aidiff.md](aidiff.md)
  - `game-ai\src\action_score.rs` `0xe5fb50`→`0x105a9f0`
  - `game-ai\src\action_score.rs` `0xe5a690`→`0x1055690`
  - `game-ai\src\ai_utils.rs` `0xec5fa0`→`0xeaefb0`
  - `game-ai\src\finish_race.rs` `0xe40bd0`→`0xf9dc40`
  - `game-ai\src\finish_race.rs` `0xf309e0`→`0x1094260`
  - `game-ai\src\free_dist.rs` `0xe82df0`→`0x10c31d0`
  - `game-ai\src\item_purchase.rs` `0xf0b9f0`→`0xffd5a0`
  - `game-ai\src\observe.rs` `0xfcb590`→`0xf19810`
  - `game-ai\src\path_field.rs` `0xe88ee0`→`0x10c8d00`
  - `game-ai\src\path_field.rs` `0xe7ad60`→`0x10ba4d0`
  - `game-ai\src\path_finder.rs` `0xf5e6f0`→`0xf766c0`
  - `game-ai\src\plan\big_plan\battle.rs` `0xef7860`→`0xf0ac40`
  - `game-ai\src\plan\big_plan\battle.rs` `0xee2120`→`0xefbfe0`
  - `game-ai\src\plan\big_plan\battle.rs` `0xedf560`→`0xef8eb0`
  - `game-ai\src\plan\big_plan\battle.rs` `0xe30bb0`→`0xf0a4d0`
  - `game-ai\src\plan\big_plan\battle.rs` `0xee6490`→`0xf02990`
  - `game-ai\src\plan\big_plan\battle.rs` `0xef79e0`→`0x10eccd0`
  - `game-ai\src\plan\big_plan\battle.rs` `0xef83e0`→`0xf92f40`
  - `game-ai\src\plan\big_plan\battle.rs` `0xef81a0`→`0xf93140`
  - `game-ai\src\plan\big_plan\battle.rs` `0xef7ca0`→`0xf023c0`
  - `game-ai\src\plan\big_plan\battle.rs` `0xef4ca0`→`0xeff890`
  - `game-ai\src\plan\big_plan\death_battle.rs` `0xeebd10`→`0x108d750`
  - `game-ai\src\plan\big_plan\defense_nexus.rs` `0xf16380`→`0xe9c9d0`
  - `game-ai\src\plan\big_plan\epic.rs` `0xdcb2c0`→`0xecfd40`
  - `game-ai\src\plan\big_plan\epic\hunt_and_poke.rs` `0xde7950`→`0xee8750`
  - `game-ai\src\plan\big_plan\fight_model.rs` `0xf56060`→`0x10b2e40`
  - `game-ai\src\plan\big_plan\line_gank\ganker.rs` `0xfc42b0`→`0x10d56f0`
  - `game-ai\src\plan\big_plan\passive_jungle.rs` `0x1016020`→`0x10e1970`
  - `game-ai\src\plan\big_plan\passive_jungle.rs` `0x100c870`→`0x10d1180`
  - `game-ai\src\plan\big_plan\passive_jungle.rs` `0xd95980`→`0x10ee5a0`
  - `game-ai\src\plan\big_plan\passive_jungle.rs` `0xd57940`→`0x10ced80`
  - `game-ai\src\plan\big_plan\passive_line.rs` `0xf94700`→`0x1103560`
  - `game-ai\src\plan\big_plan\passive_line.rs` `0xf98cf0`→`0x1105f80`
  - `game-ai\src\plan\big_plan\serpen.rs` `0xdce140`→`0xed2c50`
  - `game-ai\src\plan\big_plan\serpen\hunt_and_poke.rs` `0xe8f590`→`0xee9a30`
  - `game-ai\src\plan\big_plan\single_battle.rs` `0xe4a890`→`0xee4e60`
  - `game-ai\src\plan\big_plan\single_battle.rs` `0xe67f40`→`0xf01d40`
  - `game-ai\src\plan\handler.rs` `0xd40740`→`0xe381b0`
  - `game-ai\src\plan\handler.rs` `0xd568a0`→`0xe4dc10`
  - `game-ai\src\plan\handler\assign.rs` `0xd5b2e0`→`0xe52ab0`
  - … 외 45개

## 3. 매니페스트 STALE (mig_verify check)

| 모드 | PASS | STALE·INVALID | (INVALID) |
|---|---|---|---|
| tfm2_level_cap | 0 | **2** |  |
| tfm2_champion_exclude | 0 | **1** |  |
| banpick_view_plus | 0 | **28** |  |
| tfm2_comptest_unlock | 0 | **72** |  |
| tfm2_elemental_serpen | 0 | **18** |  |
| tfm2_champ_pos_lock | 0 | **4** |  |
| training_view_plus | 0 | 0 |  |
| facility_view_plus | 0 | 0 |  |
| custom_tier_assignment | 0 | 0 |  |
| tfm2_tactics_verify | 0 | **108** |  |
| tfm2_detail_tactics | 0 | **321** |  |
| tfm2_lane_tactics | 0 | **1** |  |
| ui_kit | 0 | **11** |  |

합계 STALE·INVALID = **566** · 전문 [log](logs/05_check.log)

## 4. 재핀 계획 (repin plan + sitealign) — ★apply 안 함

- 대상 566 · 엔진 해결 463 · 실패 103 · 사람 판정 필요 213 (실패 + sitealign AGREE 아님 + 함수시작 WEAK) · 동형 클론 주의 9 · 파일 [repin_map.aligned.json](repin_map.aligned.json)
  - ★**repin apply 대상 사람 판정 = 34건** (아래 표 전부) · 생성물 모드 179건(tfm2_detail_tactics·tfm2_tactics_verify)은 **repin 판정 불필요** — sites.rs 가 생성물이라 6절 상세전술(③ track_update)·④ migrate_targets 가 따로 추적한다(참고용으로만 repin_review.md 뒤쪽에 둔다)
- 엔진 판별 종류: BYTES_UNIQUE=274, FAIL=103, MID_OWNER_CG_NONE=92, MID_OWNER_HEAD_UNIQUE=47, MID_OWNER_UNIQUE=19, FN_UNIQUE=13, FN_HEAD_UNIQUE=7, MID_OWNER_CG_MULTI_caller=6, MID_OWNER_CG_NONE_caller=4, MID_OWNER_CG_MULTI=1
- sitealign(함수 중간 사이트): AGREE=284, DISAGREE_EXACT=31, DISAGREE_LOOSE=2, DISAGREE_LOOSE_GAP=1, DISAGREE_MNEM=10, DISAGREE_WLOOSE=8, DISAGREE_WLOOSE_GAP=7, DISAGREE_WMNEM=16, FAIL_ALIGNED_EXACT=39, FAIL_ALIGNED_LOOSE_GAP=2, FAIL_ALIGNED_MNEM=4, FAIL_ALIGNED_WLOOSE_GAP=1, FAIL_ALIGNED_WMNEM=8, FAIL_NO_ALIGN=4, FAIL_OWNER=34, NO_ALIGN=32
- 함수시작 교차검증(콜러 수·callee 사영·size 비 2/3, [repin_fncheck.json](repin_fncheck.json)): CLONE=9, STRONG=50, WEAK=3
- ⚠「사람 판정 필요」 밖(AGREE·STRONG)도 **후보**다 — apply 전 MIG\README 0.6.0 절 규칙으로 최종 확정.

| 모드 | 대상 | 해결 | 실패 | 정렬 AGREE | 함수 WEAK | 클론 | 사람 판정 |
|---|---|---|---|---|---|---|---|
| tfm2_level_cap | 2 | 1 | 1 | 1 | 0 | 0 | **1** |
| tfm2_champion_exclude | 1 | 1 | 0 | 0 | 0 | 0 | 0 |
| banpick_view_plus | 28 | 23 | 5 | 2 | 1 | 2 | **6** |
| tfm2_comptest_unlock | 72 | 56 | 16 | 30 | 0 | 2 | **17** |
| tfm2_elemental_serpen | 18 | 12 | 6 | 1 | 2 | 0 | **8** |
| tfm2_champ_pos_lock | 4 | 4 | 0 | 0 | 0 | 0 | 0 |
| training_view_plus | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| facility_view_plus | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| custom_tier_assignment | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| tfm2_tactics_verify (생성물 — ③④, apply 안 함) | 108 | 91 | 17 | 75 | 0 | 0 | 33(참고) |
| tfm2_detail_tactics (생성물 — ③④, apply 안 함) | 321 | 265 | 56 | 175 | 0 | 0 | 146(참고) |
| tfm2_lane_tactics | 1 | 1 | 0 | 0 | 0 | 0 | 0 |
| ui_kit | 11 | 9 | 2 | 0 | 0 | 5 | **2** |

<details><summary>동형 클론 주의 9건 (같은 skel 의 함수가 신 exe 에 여럿 — 콜러 문맥으로 확인)</summary>

| 모드 | 이름 | 구 | 후보 | 비고 |
|---|---|---|---|---|
| banpick_view_plus | RVA_ASSET_GET | `0x16cca0` | 0x159060 | MID_OWNER_CG_NONE · 같은 skel 29개 |
| banpick_view_plus | RVA_ANIM_GET | `0x20f5f0` | 0x261f50 | MID_OWNER_CG_NONE · 같은 skel 29개 |
| tfm2_comptest_unlock | CTX_DROP_RVA | `0x294b3c0` | 0x257fa80 | MID_OWNER_CG_MULTI_caller · 같은 skel 6개 |
| tfm2_comptest_unlock | CLONE_CHAMP_RVA | `0x1d32ca0` | 0x220f510 | MID_OWNER_CG_MULTI_caller · 같은 skel 5개 |
| ui_kit | RVA_DROP_MAP | `0x2f60f0` | 0x351e10 | MID_OWNER_CG_MULTI_caller · 같은 skel 2개 |
| ui_kit | RVA_INSERT | `0xa30c90` | 0xa315c0 | MID_OWNER_CG_MULTI · 같은 skel 5개 |
| ui_kit | RVA_CLONE | `0x29305b0` | 0x28f2de0 | MID_OWNER_CG_MULTI_caller · 같은 skel 2개 |
| ui_kit | RVA_SEND | `0x29c8000` | 0x2802060 | BYTES_UNIQUE · 같은 skel 5개 |
| ui_kit | RVA_RESPONSE | `0x1db7810` | 0x2803cb0 | BYTES_UNIQUE · 같은 skel 2개 |

</details>
- 전 목록 = [repin_review.md](repin_review.md)

<details open><summary>★repin apply 대상 사람 판정 34건</summary>

| 모드 | 이름 | 구 | repin | aligned | 판정 |
|---|---|---|---|---|---|
| tfm2_level_cap | RVA_UI_CMP | `0x8742ce` | 실패 |  | FAIL_NO_ALIGN |
| banpick_view_plus | RVA_TEXT_BUILD | `0x1bde50` | 0x13194e0 |  | FN_WEAK(FN_HEAD_UNIQUE · 콜러 [6, 1] · 사영 None · size 5.855) |
| banpick_view_plus | RVA_CARD_DRAW | `0x22cb030` | 실패 |  | FAIL |
| banpick_view_plus | RVA_I_SNAP_H | `0x235ee00` | 실패 | 0x2fcff5a | FAIL_ALIGNED_EXACT |
| banpick_view_plus | RVA_D_SNAP_W | `0x235ee16` | 실패 | 0x2fcff70 | FAIL_ALIGNED_MNEM |
| banpick_view_plus | RVA_D_ZIG_X1 | `0x235fc62` | 실패 | 0x2fd1002 | FAIL_ALIGNED_LOOSE_GAP |
| banpick_view_plus | RVA_D_ZIG_X2 | `0x2360340` | 실패 | 0x2fd16e0 | FAIL_ALIGNED_LOOSE_GAP |
| tfm2_comptest_unlock | RPLY_RVA | `0xd0af9` | 실패 | 0xe7329 | FAIL_ALIGNED_EXACT |
| tfm2_comptest_unlock | RPLY_RESUME_RVA | `0xd0b04` | 실패 | 0xe7334 | FAIL_ALIGNED_EXACT |
| tfm2_comptest_unlock | LOADER_RVA | `0x337890` | 실패 |  | FAIL |
| tfm2_comptest_unlock | A15E20_RVA | `0x79ad90` | 실패 |  | FAIL_OWNER |
| tfm2_comptest_unlock | DRIVE_RVA | `0xce4eca` | 실패 | 0xc1e042 | FAIL_ALIGNED_WMNEM |
| tfm2_comptest_unlock | DRIVE_RESUME_RVA | `0xce4edd` | 실패 | 0xc1e055 | FAIL_ALIGNED_WMNEM |
| tfm2_comptest_unlock | SRV_RVA | `0x12fb277` | 실패 | 0x19b7ad6 | FAIL_ALIGNED_EXACT |
| tfm2_comptest_unlock | INLINE@src/tfm2_comptest_unlock.rs:192 | `0x1533c6e` | 실패 |  | FAIL_OWNER |
| tfm2_comptest_unlock | INLINE@src/tfm2_comptest_unlock.rs:110 | `0x2a52ca0` | 0x2fbdc00 | 0x2fbdf37 | DISAGREE_EXACT |
| tfm2_comptest_unlock | CT_REGION_LO | `0x289fbd0` | 실패 |  | FAIL |
| tfm2_comptest_unlock | INLINE@src/tfm2_comptest_unlock.rs:147 | `0x28b19a3` | 실패 | 0x2860cdf | FAIL_ALIGNED_WMNEM |
| tfm2_comptest_unlock | INLINE@src/tfm2_comptest_unlock.rs:137 | `0x28b5371` | 실패 | 0x2864e56 | FAIL_ALIGNED_WMNEM |
| tfm2_comptest_unlock | CT_REGION_HI | `0x28cd18e` | 실패 |  | FAIL |
| tfm2_comptest_unlock | SREG_RVA | `0x28fb410` | 실패 |  | FAIL |
| tfm2_comptest_unlock | INLINE@src/tfm2_comptest_unlock.rs:171 | `0x28fb8a0` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_comptest_unlock | CT_CLIENT_LO | `0x241bc00` | 실패 |  | FAIL_OWNER |
| tfm2_comptest_unlock | RUNNER_VT_RVA | `0x3ce5040` | 실패 |  | FAIL |
| tfm2_elemental_serpen | UILOADER_RVA | `0x337890` | 실패 |  | FAIL |
| tfm2_elemental_serpen | LAUNCHER_RET_A | `0xc848b5` | 실패 | 0xbc1695 | FAIL_ALIGNED_WMNEM |
| tfm2_elemental_serpen | LAUNCHER_RET_B | `0xc88baf` | 실패 | 0xbc5a04 | FAIL_ALIGNED_WLOOSE_GAP |
| tfm2_elemental_serpen | RENDER_STEP_RVA | `0x87abe0` | 0x87bf10 |  | FN_WEAK(BYTES_UNIQUE · 콜러 [9, 1] · 사영 0.1 · size 0.236) |
| tfm2_elemental_serpen | SERPEN_RVA | `0x186f1e0` | 실패 |  | FAIL |
| tfm2_elemental_serpen | DMGA_RVA | `0x176a6c0` | 0x167a711 |  | FN_WEAK(후보가 신 함수시작 아님) |
| tfm2_elemental_serpen | MOBATICK_RVA | `0x15fc1d0` | 실패 |  | FAIL |
| tfm2_elemental_serpen | LAUNCHER_RET_C | `0x21203d4` | 실패 |  | FAIL_OWNER |
| ui_kit | RVA_DROP | `0x2859590` | 실패 |  | FAIL |
| ui_kit | RVA_UPDATE | `0x2360f30` | 실패 |  | FAIL |

</details>

<details><summary>(참고) 생성물 모드 179건 — repin 판정 불필요(③④ 도구가 따로 판정) · 앞 30</summary>

| 모드 | 이름 | 구 | repin | aligned | 판정 |
|---|---|---|---|---|---|
| tfm2_tactics_verify | BASELINE_EXE_SHA256 | `0xd4b2ca` | 0xe42d3a | 0xe42f1a | DISAGREE_WLOOSE |
| tfm2_tactics_verify | BASELINE_EXE_SHA256 | `0xd5200d` | 0xe49a7d |  | NO_ALIGN |
| tfm2_tactics_verify | INLINE@src/sites.rs:11 | `0xd6c77a` | 실패 | 0xe637c5 | FAIL_ALIGNED_EXACT |
| tfm2_tactics_verify | INLINE@src/sites.rs:12 | `0xd6c953` | 실패 | 0xe638dd | FAIL_ALIGNED_EXACT |
| tfm2_tactics_verify | INLINE@src/sites.rs:13 | `0xd6ca19` | 실패 | 0xe639a3 | FAIL_ALIGNED_EXACT |
| tfm2_tactics_verify | INLINE@src/sites.rs:14 | `0xd6cadf` | 실패 | 0xe63a69 | FAIL_ALIGNED_EXACT |
| tfm2_tactics_verify | INLINE@src/sites.rs:15 | `0xd6ccc0` | 실패 | 0xe63b49 | FAIL_ALIGNED_EXACT |
| tfm2_tactics_verify | INLINE@src/sites.rs:16 | `0xd6f935` | 0xe66795 |  | NO_ALIGN |
| tfm2_tactics_verify | INLINE@src/sites.rs:17 | `0xd6fcc7` | 0xe66b27 | 0xe66ad2 | DISAGREE_MNEM |
| tfm2_tactics_verify | INLINE@src/sites.rs:18 | `0xd6ffc0` | 0xe66e20 | 0xe66d4f | DISAGREE_MNEM |
| tfm2_tactics_verify | INLINE@src/sites.rs:32 | `0xdbd25a` | 0xec1d7a | 0xec1d37 | DISAGREE_EXACT |
| tfm2_tactics_verify | INLINE@src/sites.rs:33 | `0xdbd26b` | 0xec1d8b | 0xec1d48 | DISAGREE_EXACT |
| tfm2_tactics_verify | INLINE@src/sites.rs:40 | `0xe00987` | 0xf2c6c7 | 0xf2c0ae | DISAGREE_EXACT |
| tfm2_tactics_verify | INLINE@src/sites.rs:46 | `0xe52923` | 0x1018273 | 0x1017856 | DISAGREE_WMNEM |
| tfm2_tactics_verify | INLINE@src/sites.rs:115 | `0xe52929` | 0x1018279 | 0x1017860 | DISAGREE_EXACT |
| tfm2_tactics_verify | INLINE@src/sites.rs:47 | `0xe5292d` | 0x101827d | 0x1017864 | DISAGREE_EXACT |
| tfm2_tactics_verify | INLINE@src/sites.rs:48 | `0xe5b546` | 0x1056546 | 0x1056830 | DISAGREE_EXACT |
| tfm2_tactics_verify | INLINE@src/sites.rs:109 | `0xe63da9` | 실패 | 0x105ed2f | FAIL_ALIGNED_EXACT |
| tfm2_tactics_verify | INLINE@src/sites.rs:49 | `0xe63db5` | 실패 | 0x105ed3b | FAIL_ALIGNED_EXACT |
| tfm2_tactics_verify | INLINE@src/sites.rs:50 | `0xe922df` | 실패 |  | FAIL_OWNER |
| tfm2_tactics_verify | INLINE@src/sites.rs:51 | `0xe998a2` | 0xf43092 | 0xf43085 | DISAGREE_MNEM |
| tfm2_tactics_verify | INLINE@src/sites.rs:52 | `0xe9ce82` | 0xf43f92 | 0xf43f85 | DISAGREE_MNEM |
| tfm2_tactics_verify | INLINE@src/sites.rs:73 | `0xf7d2a4` | 0x1063314 | 0x1063357 | DISAGREE_EXACT |
| tfm2_tactics_verify | INLINE@src/sites.rs:74 | `0xf9603d` | 0x1104e9d |  | NO_ALIGN |
| tfm2_tactics_verify | INLINE@src/sites.rs:75 | `0xfb3c2f` | 실패 |  | FAIL_OWNER |
| tfm2_tactics_verify | INLINE@src/sites.rs:76 | `0xfb3c89` | 실패 |  | FAIL_OWNER |
| tfm2_tactics_verify | INLINE@src/sites.rs:143 | `0xfcc584` | 실패 | 0xf1a33d | FAIL_ALIGNED_EXACT |
| tfm2_tactics_verify | INLINE@src/sites.rs:85 | `0x100935b` | 실패 | 0x10cd5a2 | FAIL_ALIGNED_EXACT |
| tfm2_tactics_verify | INLINE@src/sites.rs:86 | `0x1009475` | 실패 | 0x10cd6c5 | FAIL_ALIGNED_EXACT |
| tfm2_tactics_verify | INLINE@src/sites.rs:87 | `0x1009e2c` | 실패 | 0x10ce001 | FAIL_ALIGNED_EXACT |

</details>

## 5. 구조체 오프셋 (재핀 후보값 기준 지문 대조)

- tfm2_level_cap — 기준선 지문 없음
- tfm2_champion_exclude — 일치
- ★**banpick_view_plus — 변화 6개 함수**
  - `RVA_ASSET_GET` 0x16cca0→0x159060 Δ+0x338 ×4 ⚠재핀 후보가 CLONE — 이 행은 엉뚱한 함수를 본 것일 수 있다(재핀 판정 후 다시)
  - `RVA_TEXT_BUILD` 0x1bde50→0x13194e0 - 사라짐 - / 생김 0x11,0x14,0x18,0x1c,0x20,0x30,0x40,0x50,0x58,0x60,0x68,0x70 ⚠재핀 후보가 WEAK — 이 행은 엉뚱한 함수를 본 것일 수 있다(재핀 판정 후 다시)
  - `RVA_IMG_BUILD` 0x1be5d0→0x1c97d0 Δ-0x18 ×2 
  - `RVA_ANIM_GET` 0x20f5f0→0x261f50 Δ+0x338 ×4 ⚠재핀 후보가 CLONE — 이 행은 엉뚱한 함수를 본 것일 수 있다(재핀 판정 후 다시)
  - `RVA_FX_SET` 0x22b56b0→0x2ef8990 Δ+0x40 ×9 
  - `RVA_CARD_DRAW` 0x22cb030→ - 재핀 실패 — 지문 대조 불가
- ★**tfm2_comptest_unlock — 변화 13개 함수**
  - `LOADER_RVA` 0x337890→ - 재핀 실패 — 지문 대조 불가
  - `ORACLE_RVA` 0x15a6410→0x1896cf0 Δ+0x780 ×4 
  - `REFRESH_RVA` 0x29751d0→0x2e9e260 Δ+0x2e0 ×9 
  - `CSEND_RVA` 0x297bac0→0x2ea4f60 Δ+0x3 ×1 
  - `CGATE_RVA` 0x297fd00→0x2ea8d10 Δ+0x228 ×2 
  - `RUN_RVA` 0x298d090→0x2eb9920 Δ+0x2e0 ×4 
  - `COLLECT_RVA` 0x298ec00→0x2ebb490 Δ+0x80 ×1 
  - `RPLY3_RVA` 0x2992ce0→0x2ebf6d0 Δ+0x1a8 ×173 
  - `RPLY2_RVA` 0x29a3a90→0x2ed0480 Δ+0x4 ×1 
  - `ARRIVE_RVA` 0x29a4300→0x2ed0cf0 Δ+0x4 ×1 
  - `LIVEB_RVA` 0x248fc90→0x213a680 Δ+0x1a8 ×6 
  - `CT_REGION_LO` 0x289fbd0→ - 재핀 실패 — 지문 대조 불가
- ★**tfm2_elemental_serpen — 변화 8개 함수**
  - `UILOADER_RVA` 0x337890→ - 재핀 실패 — 지문 대조 불가
  - `RENDER_STEP_RVA` 0x87abe0→0x87bf10 Δ+0x1e8 ×5 ⚠재핀 후보가 WEAK — 이 행은 엉뚱한 함수를 본 것일 수 있다(재핀 판정 후 다시)
  - `SERPEN_RVA` 0x186f1e0→ - 재핀 실패 — 지문 대조 불가
  - `DMGB_RVA` 0x130c680→0x18c0480 Δ+0x8 ×1 
  - `DMGA_RVA` 0x176a6c0→0x167a711 - 재핀 후보값이 함수시작 아님
  - `LAUNCHER_RVA` 0x15ad190→0x189e2a0 - 사라짐 0x478,0x588,0x698,0x7a8,0x8b8,0x9c8 / 생김 -
  - `RUNNER_CTOR_RVA` 0x15aedd0→0x189fae0 - 사라짐 0x478,0x588,0x698,0x7a8,0x8b8,0x9c8 / 생김 -
  - `MOBATICK_RVA` 0x15fc1d0→ - 재핀 실패 — 지문 대조 불가
- ★**tfm2_champ_pos_lock — 변화 3개 함수**
  - `RVA_SELECT_SWAP` 0x22a0bc0→0x2ee35e0 Δ+0x18 ×6 
  - `RVA_PICK_CLICK` 0x20e36f0→0x2dae960 Δ+0x18 ×2 
  - `RVA_SWAP_CONFIRM` 0x20e3fb0→0x2daf220 Δ+0x18 ×4 
- training_view_plus — 기준선 지문 없음
- facility_view_plus — 기준선 지문 없음
- custom_tier_assignment — 기준선 지문 없음
- tfm2_tactics_verify — 기준선 지문 없음
- tfm2_detail_tactics — 기준선 지문 없음
- ★**tfm2_lane_tactics — 변화 1개 함수**
  - `RVA_PASSIVE_PLAN` 0xd5b2e0→0xe52ab0 Δ+0x30 ×17 
- ★**ui_kit — 변화 2개 함수**
  - `DD_SETOPT_RVA` 0x1e79d0→0x216d50 Δ+0x68 ×3 
  - `RVA_UPDATE` 0x2360f30→ - 재핀 실패 — 지문 대조 불가

> apply 뒤 정식 `offsets.py check` → `sources` 는 따로 돈다(이 표는 apply 전 예보). [json](offsets_shifts.json)

## 6. ★상세전술 (tfm2_detail_tactics — 전술 100 · 패치창 218)

- 전술 100개 → **자동확정(AUTO) 25 · 검토 필요(REVIEW) 75** · 창 218 (자동 52)

| 전술의 가장 나쁜 창 판정 | 뜻 | 전술 수 | 창 수 |
|---|---|---|---|
| `SAME` | 이동만(자동확정) | 25 | 52 |
| `REG_RENAME` | 레지스터·스택 재배치 | **22** | 57 |
| `VALUE_CHANGED` | 게임 기본값 변경 | 0 | 0 |
| `COPIES` | 사본 분기 | 0 | 0 |
| `LOGIC_CHANGED` | 로직 변경 | **43** | 72 |
| `SITE_MISSING` | 창 소실 | **8** | 35 |
| `OWNER_MISSING` | 함수 소실 | **2** | 2 |

- 전문: [track.md](detail_tactics/track.md) · [review_queue.json](detail_tactics/review_queue.json) · 초안 `detail_tactics\draft_targets\`

### ★게임이 기본값을 바꾼 전술 (0)

- 없음

### 검토 필요 전술 (75 · 나쁜 판정 순 앞 30)

- [34] **구조물 공격 중 후퇴 유지 체력(%)** (`structure_retreat_keep_hp`, 숨김) — `OWNER_MISSING` 함수 소실 · extra_c003@0xe922df: OWNER_MISSING: owner 0xe90e90 대응 없음 (Location LOC_GONE · repin NONE skel 후보0 / head후보 0 | midpin A:ALIGN_AMBIG 0.10/0.10)
- [91] **같은라인 동료 근접반경** (`same_lane_partner_radius`, 라인·이동) — `OWNER_MISSING` 함수 소실 · extra_n004@0xd3dae1: OWNER_MISSING: owner 0xd3d9f0 대응 없음 (Location LOC_GONE · repin NONE skel 후보0 / head후보 0 | midpin A:ALIGN_AMBIG 0.23/0.23)
- [3] **라인 후퇴 점수 문턱** (`lane_retreat_final`, 라인·이동) — `SITE_MISSING` 창 소실 · lane_retreat_final_50@0xf9603d: SITE_MISSING: owner 0xf94700→0x1103560 안에서 창 대응 실패 (꼴 후보 0개 [])
- [5] **교전 회피 체력(%)** (`combat_avoid_hp`, 전투·생존) — `SITE_MISSING` 창 소실 · combat_avoid_hp_41@0xe52923: SITE_MISSING: owner 0xe4f200→0x1014b50 안에서 창 대응 실패 (꼴 후보 4개 ['0x1017b01', '0x10182f5', '0x1019b88', '0x101a13c'])
- [14] **정글링 중단 체력(%)** (`jungle_stop_hp`, 정글) — `SITE_MISSING` 창 소실 · jungle_stop_hp_41@0x1018efd: SITE_MISSING: Caller 반환 콜사이트 0x100d2b0 소실 — call 정렬 실패 · 같은 꼴 후보 84 → 호출 대상 0x10e4a30 로 거르면 3
- [38] **오브젝트 전력 보완값** (`objective_power_allowance`, 숨김) — `SITE_MISSING` 창 소실 · extra_c007@0xdabbe7: SITE_MISSING: owner 0xdab3e0→0xeb9e60 안에서 창 대응 실패 (꼴 후보 102개 ['0xeba1c5', '0xeba40c', '0xeba7fd', '0xeba8dc', '0xeba8fb', '0xebaaaa', '0xeb
- [52] **다른 라인 붕괴 경계** (`other_lane_collapse_profile`, 숨김) — `SITE_MISSING` 창 소실 · extra_c023@0xfb2510: SITE_MISSING: Caller 반환 콜사이트 0xdbd5fc 소실 — call 정렬 실패 · 같은 꼴 후보 175 → 호출 대상 0x103b4c0 로 거르면 4
- [63] **보스 견제 예상피해 가중치** (`boss_poke_damage_factor`, 숨김) — `SITE_MISSING` 창 소실 · extra_c035@0xe96af2: SITE_MISSING: owner 0xe954f0→0xf3d620 안에서 창 대응 실패 (꼴 후보 0개 []) · 참고: 값 조각 `add rax, rax` 와 같은 꼴·즉치 위치 9개 ['0xf3da03', '0xf3dae6', '0xf3db90
- [69] **이동기 절약 보행 여유(업데이트)** (`movement_skill_period_margin`, 위치·기술) — `SITE_MISSING` 창 소실 · extra_c042@0xf59d0f: SITE_MISSING: Caller 반환 콜사이트 0xe922b9 소실 — caller 함수 대응 없음(NONE skel 후보0 / head후보 0 | midpin A:ALIGN_AMBIG 0.10/0.10)
- [87] **서폿 로밍 원딜 최소 체력** (`support_roam_adc_min_hp`, 숨김) — `SITE_MISSING` 창 소실 · extra_d004@0xd3fa45: SITE_MISSING: owner 0xd3f9f0→0xe73770 안에서 창 대응 실패 (꼴 후보 0개 [])
- [0] **전투 중 후퇴 체력(%)** (`combat_retreat_hp`, 전투·생존) — `LOGIC_CHANGED` 로직 변경 · combat_retreat_hp_35@0xe5b546: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['action_score.rs:1156:26', 'action_score.rs:1185:125'] · 생긴 행 ['actio
- [1] **라인·추적 중 즉시 후퇴 체력(%)** (`line_chase_retreat_hp`, 전투·생존) — `LOGIC_CHANGED` 로직 변경 · line_chase_retreat_hp_45@0xf7ce6d: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED · 행 시프트 갈림: ['small_action.rs:140:21'] 등은 그대로, 나머지 행만 밀림 = 함수 중간 편집 의심) + 창 주변 변화
- [2] **추적 표적 유지 체력(%)** (`line_chase_target_guard_hp`, 전투·생존) — `LOGIC_CHANGED` 로직 변경 · line_chase_target_guard_hp_65@0xf7d2a4: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED · 행 시프트 갈림: ['small_action.rs:140:21'] 등은 그대로, 나머지 행만 밀림 = 함수 중간 편집 의심) + 창 주변 
- [4] **수적 열세 후퇴 가중치** (`crowd_retreat_weight`, 전투·생존) — `LOGIC_CHANGED` 로직 변경 · crowd_retreat_weight_plus2_300@0xe5c97e: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['action_score.rs:1156:26', 'action_score.rs:1185:125'] · 생긴
- [6] **교전 회피 인원 여유(명)** (`combat_avoid_count_margin`, 전투·생존) — `LOGIC_CHANGED` 로직 변경 · combat_avoid_count_margin_2@0xe52929: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['battle.rs:149:55', 'battle.rs:179:46', 'battle.rs:212:97', 'b
- [8] **갱 시작 체력(%)** (`jg_gank_hp_start`, 정글) — `LOGIC_CHANGED` 로직 변경 · jg_gank_hp_start_50@0x1009e2c: LOGIC_CHANGED: 창 1번째 명령 꼴 변화 `mov rdx, qword ptr [rbp + 0x138]` → `jb 0x10cd945`
- [12] **갱 콜 수락 최소 체력(%)** (`gank_call_accept_hp`, 갱·로밍) — `LOGIC_CHANGED` 로직 변경 · gank_call_accept_hp_60@0xd7079f: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['signal.rs:229:23', 'signal.rs:234:77'] · 생긴 행 ['signal.rs:239:23',
- [17] **라인 기회 — 건강 아군 최소(명)** (`lane_opportunity_own_min`, 라인·이동) — `LOGIC_CHANGED` 로직 변경 · lane_opportunity_own_min_2@0xfb3c2f: LOGIC_CHANGED: 창 안 분기 대상 0xfb3e1d → 신 0x103d202 ≠ 기대 ? (대상 명령 정렬 안 됨)
- [20] **로밍 최소 자기 체력(%)** (`liner_roam_self_hp`, 갱·로밍) — `LOGIC_CHANGED` 로직 변경 · liner_roam_self_hp_41@0xd4b2ca: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['handler.rs:437:10', 'handler.rs:762:22', 'handler.rs:944:14', 'hand
- [22] **로밍 팀 콜 확률(/1000)** (`liner_roam_call_chance`, 갱·로밍) — `LOGIC_CHANGED` 로직 변경 · liner_roam_call_chance_700@0xd5200d: LOGIC_CHANGED: 소비 분기 `jae 0xd52074` 대상 0xd52074 → 신 0xe4a1b3 ≠ 기대 ? (대상 명령 정렬 안 됨)
- [24] **푸시 유지 최소 인원(명)** (`push_sustain_min_healthy`, 푸시·타워) — `LOGIC_CHANGED` 로직 변경 · push_sustain_min_healthy_3@0xdbd25a: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['objective_handlers.rs:48:8', 'objective_handlers.rs:58:8', 'ob
- [25] **푸시 유지 인원 여유(명)** (`push_sustain_margin`, 푸시·타워) — `LOGIC_CHANGED` 로직 변경 · push_sustain_margin_2@0xdbd220: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['objective_handlers.rs:48:8', 'objective_handlers.rs:58:8', 'objecti
- [32] **갱 콜 수락 난수** (`gank_call_rng`, 숨김) — `LOGIC_CHANGED` 로직 변경 · extra_c001@0xd707db: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['signal.rs:229:23', 'signal.rs:234:77'] · 생긴 행 ['signal.rs:239:23', 'signal.rs:
- [33] **픽오프 저체력 기준(%)** (`pickoff_hp_bands`, 숨김) — `LOGIC_CHANGED` 로직 변경 · extra_c002@0xfb51f3: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['opportunities.rs:423:7', 'opportunities.rs:479:26'] · 생긴 행 ['opportunities.rs:
- [35] **갱 도착 전 라인 선공** (`pre_gank_engage_profile`, 숨김) — `LOGIC_CHANGED` 로직 변경 · extra_c004@0xd6c77a: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['vec.rs:2710:37', 'vec.rs:2716:44', 'engage.rs:275:103', 'engage.rs:286:15', 'e
- [37] **교전 콜 응답 시간** (`battle_call_response_seconds`, 숨김) — `LOGIC_CHANGED` 로직 변경 · extra_c006@0xd6f931: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['signal.rs:65:25', 'signal.rs:109:23', 'signal.rs:171:22'] · 생긴 행 ['engage.rs:9
- [39] **같은 라인 갱 재시도** (`same_lane_gank_retry_seconds`, 숨김) — `LOGIC_CHANGED` 로직 변경 · extra_c008@0xd47d42: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['handler.rs:437:10', 'handler.rs:762:22', 'handler.rs:944:14', 'handler.rs:948:
- [40] **추격 속도비 기준(10분율)** (`chase_speed_ratio`, 숨김) — `LOGIC_CHANGED` 로직 변경 · extra_c010@0xe4d559: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['single_battle.rs:323:55', 'single_battle.rs:377:42', 'single_battle.rs:457:22'
- [45] **라인 방어 승인 신중도** (`line_defense_approval_profile`, 라인·이동) — `LOGIC_CHANGED` 로직 변경 · extra_c016@0xde7412: LOGIC_CHANGED: 창 ±8 명령의 비교·분기 구조 변화(정렬 기준)
- [46] **기지 교전 콜 허용 폭** (`base_battle_call_range`, 숨김) — `LOGIC_CHANGED` 로직 변경 · extra_c017@0xd6c872: LOGIC_CHANGED: 소비 분기 `ja 0xd6c8da` 대상 0xd6c8da → 신 0xe62ecf ≠ 기대 ? (대상 명령 정렬 안 됨)
- … 외 45 (track.md)
- 검토 대기열 파일: [review_queue.json](detail_tactics/review_queue.json)

## 7. 주소록 (addrbook — KB 에 적힌 함수·지점)

- (주소록 산출물 없음 — 단계 상태: 건너뜀 --skip)

## 8. 핫픽스 노출 표 (게임을 켜기 전에 볼 것)

- 이번 판정: **정식 버전 변경(또는 --label 지정)** — 「정식패치」 열을 볼 것
- 정식 패치(0.6.6): deps 상한 10종은 게임이 자동 비활성, 상한 없는 5종은 stable 전용(RVA 0)이라 무관 = 안전
- 핫픽스(버전 문자열 0.6.5 유지 — 과거 7/7회): 15종 전부 새 exe 에서 로드됨
- 핫픽스 크래시 위험 높음 2종 = tfm2_comptest_unlock(UI 로더 훅 무검증) · tfm2_elemental_serpen(arg_str 훅 무검증·init 즉시)
- 중간 1종 = banpick_view_plus(훅은 대조하나 훅 안 무검증 헬퍼 13개 — 부분 이동 핫픽스에서 위험)
- 나머지는 안전하게 꺼지거나 기능만 죽음 · tfm2_detail_tactics 는 EXE SHA256 가드로 100전술 전부 미설치
- 권고: 게임 켜기 전 run_patchday.py → 핫픽스면 comptest·serpen(필수)·banpick_view_plus(권장) 를 enabled_mods 에서 빼고 마이그 후 복귀

| 모드 | 활성 | deps | 정식패치(0.6.6) | 핫픽스(0.6.5 유지) | 자체 확인 | 권고 |
|---|---|---|---|---|---|---|
| tfm2_comptest_unlock | ON | >=0.6.5, <0.6.6 | 자동 비활성(deps 상한) | 로드됨 · UI로더 훅 무검증 덮어쓰기 → 크래시 위험 높음 | 혼합 — 패치·훅 대부분 바이트 대조, 로더 훅 확인 없음 | 핫픽스면 게임 켜기 전 끄기(필수) |
| tfm2_elemental_serpen | ON | >=0.6.5, <0.6.6 | 자동 비활성(deps 상한) | 로드됨 · arg_str 훅 무검증(init 즉시) → 크래시 위험 높음 | 훅 12B 대조(대부분 범용 push8) · arg_str 확인 없음 | 핫픽스면 게임 켜기 전 끄기(필수) |
| banpick_view_plus | ON | >=0.6.5, <0.6.6 | 자동 비활성(deps 상한) | 전면이동=훅 미설치 / 부분이동=무검증 헬퍼 호출 크래시 | 훅3 진입 12·12·13B + 기하 13항 사전검사 · 헬퍼13 무검증 | 핫픽스면 끄기 권장(밴픽 연출만 사라짐) |
| tfm2_champ_pos_lock | ON | >=0.6.5, <0.6.6 | 자동 비활성(deps 상한) | 대개 훅 미설치 → 포지션 제한 무시(크래시 위험 낮음) | 훅 4종 진입 12B 대조(범용 push8) · 자동스왑 고유 12B | 켜 둬도 됨 · 제한이 안 걸리는 것 주의 |
| tfm2_champion_exclude | ON | >=0.6.5, <0.6.6 | 자동 비활성(deps 상한) | 훅 미설치 → 제외 목록 무시(바닐라대로 신챔프 추가) | 진입 17B 정확 대조 · 다른 모드 훅이면 체인 | 켜 둬도 됨 · 시즌 패치 진행은 마이그 후 |
| tfm2_level_cap | ON | >=0.6.5, <0.6.6 | 자동 비활성(deps 상한) | 2곳 독립 → 미설치/한쪽만 → 상한 바닐라·경험치 바 깨짐 | 사이트 2곳 원본 7B 정확 대조 | 켜 둬도 됨 · 레벨업 진행은 마이그 후 |
| training_view_plus | ON | >=0.6.5, <0.6.6 | 자동 비활성(deps 상한) | UI 는 뜨나 서버 적용이 오류로 끝남(아무것도 안 씀) | 구조 가드①② + 함수 7종 진입 8~12B 정확 대조 | 켜 둬도 됨 |
| facility_view_plus | ON | >=0.6.5, <0.6.6 | 자동 비활성(deps 상한) | 생산은 서버 반영 · 즉시 화면 동기만 실패 | 구조 가드①② + 함수 3종 첫 1B 느슨(team_sync) | 켜 둬도 됨 |
| custom_tier_assignment | ON | >=0.6.5, <0.6.6 | 자동 비활성(deps 상한) | 티어 저장은 반영 · 즉시 동기만 실패 · raw 읽기 자가검증 | team_sync 가드①②③(느슨) + client_db 팀ID 자가검증 | 켜 둬도 됨 |
| tfm2_detail_tactics | ON | >=0.6.5, <0.6.6 | 자동 비활성(deps 상한) | SHA 불일치 → 100전술 전부 FAILED·원본 유지(안전) | EXE SHA256 전체 대조 + 창 218 바이트 대조 | 켜 둬도 안전 · track_update 후 재생성·재배포 |
| tfm2_mod_order | ON | >=0.6.0 | 계속 로드(상한 없음 · stable 전용) | 무관(RVA·코드쓰기 없음) | 해당 없음(RVA 0) | 조치 불요 |
| Spectator_Chat | ON | >=0.6.0 | 계속 로드(상한 없음 · stable 전용) | 무관(stable 훅·UI 만) | 해당 없음(RVA 0) | 조치 불요 |
| roster_view_plus | ON | >=0.6.0 | 계속 로드(상한 없음 · stable 전용) | 무관(RVA·코드쓰기 없음) | 해당 없음(RVA 0) | 조치 불요 |
| recruitment_view_plus | ON | >=0.6.0 | 계속 로드(상한 없음 · stable 전용) | 무관(RVA·코드쓰기 없음) | 해당 없음(RVA 0) | 조치 불요 |
| coaching_staff_view_plus | ON | >=0.6.0 | 계속 로드(상한 없음 · stable 전용) | 무관(RVA·코드쓰기 없음) | 해당 없음(RVA 0) | 조치 불요 |
| leefs_variety | ON(워크샵 3738384222) | >=0.4.0 | 계속 로드(콘텐츠 · 코드 없음) | 무관(dll 없음) | 해당 없음(dll 없음) | 조치 불요 |
| leefs_variety2 | ON(워크샵 3743143109) | >=0.4.0 | 계속 로드(콘텐츠 · 코드 없음) | 무관(dll 없음) | 해당 없음(dll 없음) | 조치 불요 |
| leefs_variety3 | ON(워크샵 3748325543) | >=0.4.0 | 계속 로드(콘텐츠 · 코드 없음) | 무관(dll 없음) | 해당 없음(dll 없음) | 조치 불요 |
| leefs_variety4 | ON(워크샵 3755409850) | >=0.4.0 | 계속 로드(콘텐츠 · 코드 없음) | 무관(dll 없음) | 해당 없음(dll 없음) | 조치 불요 |
| leefs_variety5 | ON(워크샵 3767908673) | >=0.4.0 | 계속 로드(콘텐츠 · 코드 없음) | 무관(dll 없음) | 해당 없음(dll 없음) | 조치 불요 |
| touhou_project | ON(워크샵 3738888753) | >=0.4.0 | 계속 로드(콘텐츠 · 코드 없음) | 무관(dll 없음) | 해당 없음(dll 없음) | 조치 불요 |
| tfm2_lane_tactics | OFF | >=0.6.5, <0.6.6 | 자동 비활성(deps 상한) | 지금 OFF=로드 안 됨 · 켜면 훅1 대개 미설치 | passive_plan 진입 12B 대조(범용 push8) | OFF 유지(상세전술 검수로 꺼 둔 상태) |
| tfm2_tactics_verify | OFF | >=0.6.5, <0.6.6 | 자동 비활성(deps 상한) | OFF · 켜도 SHA 불일치로 설치 거절 | EXE SHA256 대조 | OFF 유지 |
| tfm2_ai_analysis_verify | OFF | >=0.6.5, <0.6.6 | 자동 비활성(deps 상한) | OFF · 기준 SHA=0.6.4 라 무장 안 됨 | EXE SHA256 대조 | OFF 유지 |
| tfm2_flow_capture | OFF | >=0.6.0 | 켜면 로드(상한 없음 · stable) | OFF · 켜도 무관(RVA 0) | 해당 없음(RVA 0) | 조치 불요 |
| tfm2_meta_item_delegate | OFF | >=0.6.0 | 켜면 로드(상한 없음 · stable) | OFF · 켜도 무관(RVA 0) | 해당 없음(RVA 0) | 조치 불요 |
| legacy_save_patcher | OFF | >=0.5.0 | 켜면 로드(상한 없음 · stable) | OFF · 켜도 무관(소스에 RVA 없음) | 해당 없음(RVA 0 · 소스 기준) | 조치 불요 |
| tfm2_ibhook_probe | OFF | >=0.1.0 | 켜면 로드(상한 없음 · stable) | OFF · 켜도 무관(stable 훅 프로브) | 해당 없음(RVA 0) | OFF 유지(진단용) |

원문 `F:\claude\tfm2mods\MIG\patchday\hotfix_exposure_0.6.5.json` · 사람용 `F:\claude\tfm2mods\MIG\patchday\hotfix_exposure_0.6.5.md`

## 9. 다음에 사람이 할 일 (순서대로 — 정확한 명령 = MIG\patchday\README.md)

1. **게임은 아직 켜지 말 것** — 8절 노출 표에서 낡은 주소로 패치하는 모드가 있는지 확인
2. 6절 상세전술: `detail_tactics\track.md`·review_queue → 사람 판정 → draft_targets 승격 → `gen_sites.py --ver 0.7.0` → `build_local.ps1 -RunTests` → 인게임 검증
3. tfm2_tactics_verify: `tools\migrate_targets.py` (sites.rs 생성물 — repin apply 대상 아님)
4. 4절 apply 대상 잔여 판정 34건(+동형 클론) → `MIG\_mkfinal070.py` 로 final 맵 → `repin.py apply <MOD…> --write`(생성물 모드 tfm2_detail_tactics·tfm2_tactics_verify 제외) → `rebase` → `coverage` → `offsets.py check`·`sources` → `bump_deps.py` → `env.py` → `run.py` 0
5. 2절 EDITED 함수 = 재현코드(ai_adjust·judge 계층)에 닿는지 확인
6. 7절 `addrmap.md` 구→신 표로 KB 갱신(record-keeper 경유)
7. REPORT `_0.7.0_마이그.md`·모드별 03_시행착오·DONE.md 기록
