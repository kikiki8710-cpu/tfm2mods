# 패치날 요약 — 0.7.1

> 생성 2026-11-04 17:02:42 · `MIG\patchday\run_patchday.py` · 전체 소요 2s · 출력 = `F:\claude\tfm2mods\MIG\_patchday\0.7.1`

## 0. 버전

| | 기준(구) | 신 |
|---|---|---|
| 라벨 | 0.7.0 | **0.7.1** |
| sha256 | `D678E01285B7A9EDBA2C7EE1067CEA93A54F979D769EAAE9F0D8309D18FF7D30` | `BD011C426A3F5ED4C411C7D9379448D306E091A9BD83E5E201C81D3EBFDD1D6B` |
| 크기 | 93235712 B | 94043136 B |
| exe | `F:\claude\tfm2\tfm2_0.7.0\TeamfightManager2.exe` | `F:\claude\tfm2\tfm2_0.7.1\TeamfightManager2.exe` |
| base_version.txt | — | `0.7.1` (C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mod-sdk-stable\base_version.txt) |

## 1. 단계 결과

| # | 단계 | 결과 | 종료코드 | 소요 | 메모 | 로그 |
|---|---|---|---|---|---|---|
| 0 | detect | 정상 | 0 | 0.4s | 0.7.0 → 0.7.1 · sha BD011C426A3F5ED4 · 94043136 B | [log](logs/00_detect.log) |
| 1 | backup | 정상 (이전 실행 2026-11-04 16:56:15) | 0 | 3.2s | TeamfightManager2.exe: 복사 완료 94043136 B · sha BD011C426A3F5ED4 / bundle.game_data: 복사 완료 1156606899 B · sha A005B7789CE61A7F | [log](logs/01_backup.log) |
| 2 | fnindex | 정상 (이전 실행 2026-11-04 16:57:01) | 0 | 45.8s | 기준 _fnidx_070.pkl: 검증 통과 — 메타 경로 sha 일치 · 경계 165552/165552 일치 · skel 표본 48 일치 / 생성 후 검증: 메타 경로 sha 일치 · 경계 166780/166780 일치 · skel 표본 48 일치 | [log](logs/02_fnindex.log) |
| 3 | callgraph | 정상 (이전 실행 2026-11-04 16:57:29) | 0 | 28.4s | 기준 _cg_070.pkl: 검증 통과 — callee 146469개 · 표본 40 일치 / 생성 후 검증: callee 147375개 · 표본 40 일치 | [log](logs/03_callgraph.log) |
| 4 | aidiff | 정상·검토 필요 (이전 실행 2026-11-04 16:59:57) | 0 | 147.5s | EDITED 98 · MERGED 0 · 사라짐 38 · 새로 생김 65 | [log](logs/04_aidiff.log) |
| 5 | check | 정상·검토 필요 (이전 실행 2026-11-04 16:59:57) | 1 | 0.2s | STALE·INVALID 469 (모드 12개 중 9개) | [log](logs/05_check.log) |
| 6 | repin | 정상·검토 필요 (이전 실행 2026-11-04 17:01:52) | 0 | 114.6s | 대상 469 · 엔진 해결 363 · 실패 106 · 사람 판정 필요 169(apply 대상 22 + 생성물 모드 147) · 동형 클론 주의 11 | [log](logs/06_repin.log) |
| 7 | offsets | 정상·검토 필요 | 1 | 1.2s | 변화 3개 모드 · 19개 함수 (기준 = repin_map.aligned.json) | [log](logs/07_offsets.log) |
| 8 | detail_tactics | 건너뜀 |  | 0.0s | --only |  |
| 9 | addrbook | 건너뜀 |  | 0.0s | --only |  |
| 10 | summary | 정상 | 0 | 0s | SUMMARY.md |  |

## 2. AI 판단계층 (aidiff)

- ★**EDITED 98개** · MERGED 0 · SHIFTED 382 · IDENTICAL 237 · 사라짐 38 · 새로 생김 65 → 전문 [aidiff.md](aidiff.md)
  - `game-ai\src\action_score.rs` `0x105a9f0`→`0x108fa10`
  - `game-ai\src\action_score.rs` `0x1055690`→`0x1086710`
  - `game-ai\src\action_score.rs` `0x105ec00`→`0x1093f10`
  - `game-ai\src\action_score.rs` `0x10582c0`→`0x108a900`
  - `game-ai\src\ai_utils.rs` `0xe8e410`→`0x1128f50`
  - `game-ai\src\buff_value.rs` `0xff62a0`→`0x10fb9f0`
  - `game-ai\src\buff_value.rs` `0xffa720`→`0x10fa3f0`
  - `game-ai\src\buff_value.rs` `0xe8eed0`→`0x1083b70`
  - `game-ai\src\buff_value.rs` `0xff32f0`→`0x11023e0`
  - `game-ai\src\fight_check.rs` `0x1027ce0`→`0xf93e00`
  - `game-ai\src\fight_check.rs` `0x1023350`→`0xf887f0`
  - `game-ai\src\fight_check.rs` `0x1021c40`→`0xf990e0`
  - `game-ai\src\finish_race.rs` `0x1094260`→`0xf9c1b0`
  - `game-ai\src\free_dist.rs` `0x10bff40`→`0xf9bde0`
  - `game-ai\src\observe.rs` `0xf19810`→`0x1026e90`
  - `game-ai\src\path_field.rs` `0xfa4c70`→`0x1133960`
  - `game-ai\src\path_field.rs` `0xfa8e90`→`0x112fd00`
  - `game-ai\src\path_field.rs` `0xfa9f20`→`0x1139000`
  - `game-ai\src\path_field.rs` `0xfa5ec0`→`0x113a960`
  - `game-ai\src\path_field.rs` `0xfa6eb0`→`0x113c2c0`
  - `game-ai\src\path_field.rs` `0xfa7ea0`→`0x113dc20`
  - `game-ai\src\path_field.rs` `0xfaf8f0`→`0x11411c0`
  - `game-ai\src\path_field.rs` `0xfb1730`→`0x11445f0`
  - `game-ai\src\path_field.rs` `0xfaaff0`→`0x1142b20`
  - `game-ai\src\path_field.rs` `0xfabf70`→`0x1137490`
  - `game-ai\src\path_field.rs` `0xfacdd0`→`0x113f580`
  - `game-ai\src\path_field.rs` `0xfadc30`→`0x1131c40`
  - `game-ai\src\path_field.rs` `0xfaea90`→`0x11354d0`
  - `game-ai\src\path_field.rs` `0x10b80c0`→`0x112d040`
  - `game-ai\src\path_field.rs` `0x10b9fd0`→`0x112bba0`
  - `game-ai\src\path_field.rs` `0x10b75a0`→`0x112ab40`
  - `game-ai\src\path_field.rs` `0x10b7b30`→`0x112b640`
  - `game-ai\src\path_field.rs` `0x10b85d0`→`0x112c080`
  - `game-ai\src\path_field.rs` `0x10b8a00`→`0x112c470`
  - `game-ai\src\path_field.rs` `0x10b8e30`→`0x112c860`
  - `game-ai\src\path_field.rs` `0x10b9260`→`0x112cc50`
  - `game-ai\src\path_field.rs` `0x10b9690`→`0x112d930`
  - `game-ai\src\path_field.rs` `0x10b9ba0`→`0x112d540`
  - `game-ai\src\path_field.rs` `0x10ba4d0`→`0x112de00`
  - `game-ai\src\path_finder.rs` `0xf766c0`→`0x10b8370`
  - … 외 58개

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
| tfm2_detail_tactics | 0 | **322** |  |
| tfm2_transfer_fix | 0 | **11** |  |
| ui_kit | 0 | **11** |  |

합계 STALE·INVALID = **469** · 전문 [log](logs/05_check.log)

## 4. 재핀 계획 (repin plan + sitealign) — ★apply 안 함

- 대상 469 · 엔진 해결 363 · 실패 106 · 사람 판정 필요 169 (실패 + sitealign AGREE 아님 + 함수시작 WEAK) · 동형 클론 주의 11 · 파일 [repin_map.aligned.json](repin_map.aligned.json)
  - ★**repin apply 대상 사람 판정 = 22건** (아래 표 전부) · 생성물 모드 147건(tfm2_detail_tactics)은 **repin 판정 불필요** — sites.rs 가 생성물이라 6절 상세전술(③ track_update)·④ migrate_targets 가 따로 추적한다(참고용으로만 repin_review.md 뒤쪽에 둔다)
- 엔진 판별 종류: BYTES_UNIQUE=225, FAIL=106, MID_OWNER_CG_NONE=42, MID_OWNER_UNIQUE=35, FN_UNIQUE=18, MID_OWNER_HEAD_UNIQUE=18, MID_OWNER_CG_NONE_caller=10, MID_OWNER_CG_MULTI=6, FN_HEAD_UNIQUE=5, MID_OWNER_CG_MULTI_caller=4
- sitealign(함수 중간 사이트): AGREE=220, DISAGREE_EXACT=20, DISAGREE_LOOSE=3, DISAGREE_LOOSE_GAP=2, DISAGREE_MNEM=5, DISAGREE_WLOOSE=1, DISAGREE_WLOOSE_GAP=3, DISAGREE_WMNEM=10, FAIL_ALIGNED_EXACT=39, FAIL_ALIGNED_LOOSE_GAP=7, FAIL_ALIGNED_MNEM=5, FAIL_ALIGNED_WLOOSE=3, FAIL_ALIGNED_WLOOSE_GAP=1, FAIL_ALIGNED_WMNEM=12, FAIL_NO_ALIGN=6, FAIL_OWNER=32, NO_ALIGN=14
- 함수시작 교차검증(콜러 수·callee 사영·size 비 2/3, [repin_fncheck.json](repin_fncheck.json)): CLONE=11, STRONG=57
- ⚠「사람 판정 필요」 밖(AGREE·STRONG)도 **후보**다 — apply 전 MIG\README 0.6.0 절 규칙으로 최종 확정.

| 모드 | 대상 | 해결 | 실패 | 정렬 AGREE | 함수 WEAK | 클론 | 사람 판정 |
|---|---|---|---|---|---|---|---|
| tfm2_level_cap | 2 | 2 | 0 | 2 | 0 | 0 | 0 |
| tfm2_champion_exclude | 1 | 1 | 0 | 0 | 0 | 0 | 0 |
| banpick_view_plus | 28 | 28 | 0 | 6 | 0 | 2 | 0 |
| tfm2_comptest_unlock | 72 | 66 | 6 | 35 | 0 | 3 | **9** |
| tfm2_elemental_serpen | 18 | 15 | 3 | 1 | 0 | 1 | **4** |
| tfm2_champ_pos_lock | 4 | 4 | 0 | 0 | 0 | 0 | 0 |
| training_view_plus | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| facility_view_plus | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| custom_tier_assignment | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| tfm2_detail_tactics (생성물 — ③④, apply 안 함) | 322 | 234 | 88 | 175 | 0 | 0 | 147(참고) |
| tfm2_transfer_fix | 11 | 4 | 7 | 1 | 0 | 0 | **7** |
| ui_kit | 11 | 9 | 2 | 0 | 0 | 5 | **2** |

<details><summary>동형 클론 주의 11건 (같은 skel 의 함수가 신 exe 에 여럿 — 콜러 문맥으로 확인)</summary>

| 모드 | 이름 | 구 | 후보 | 비고 |
|---|---|---|---|---|
| banpick_view_plus | RVA_ASSET_GET | `0x159060` | 0x158fa0 | MID_OWNER_CG_MULTI · 같은 skel 35개 |
| banpick_view_plus | RVA_ANIM_GET | `0x261f50` | 0x261e90 | MID_OWNER_CG_MULTI · 같은 skel 35개 |
| tfm2_comptest_unlock | LOADER_RVA | `0x3a94f0` | 0x3a93f0 | MID_OWNER_CG_MULTI · 같은 skel 35개 |
| tfm2_comptest_unlock | CTX_DROP_RVA | `0x257fa80` | 0x2806360 | MID_OWNER_CG_MULTI_caller · 같은 skel 7개 |
| tfm2_comptest_unlock | CLONE_CHAMP_RVA | `0x220f510` | 0x2125af0 | MID_OWNER_CG_MULTI · 같은 skel 6개 |
| tfm2_elemental_serpen | UILOADER_RVA | `0x3a94f0` | 0x3a93f0 | MID_OWNER_CG_MULTI · 같은 skel 35개 |
| ui_kit | RVA_DROP_MAP | `0x351e10` | 0x351d10 | MID_OWNER_CG_MULTI · 같은 skel 2개 |
| ui_kit | RVA_INSERT | `0xa315c0` | 0x9520f0 | MID_OWNER_CG_MULTI_caller · 같은 skel 5개 |
| ui_kit | RVA_CLONE | `0x28f2de0` | 0x213cff0 | MID_OWNER_CG_MULTI_caller · 같은 skel 2개 |
| ui_kit | RVA_SEND | `0x2802060` | 0x21ecc70 | BYTES_UNIQUE · 같은 skel 5개 |
| ui_kit | RVA_RESPONSE | `0x2803cb0` | 0x202ee60 | BYTES_UNIQUE · 같은 skel 2개 |

</details>
- 전 목록 = [repin_review.md](repin_review.md)

<details open><summary>★repin apply 대상 사람 판정 22건</summary>

| 모드 | 이름 | 구 | repin | aligned | 판정 |
|---|---|---|---|---|---|
| tfm2_comptest_unlock | RPLY_RVA | `0xe7329` | 실패 | 0xe7269 | FAIL_ALIGNED_EXACT |
| tfm2_comptest_unlock | RPLY_RESUME_RVA | `0xe7334` | 실패 | 0xe7274 | FAIL_ALIGNED_EXACT |
| tfm2_comptest_unlock | A15E20_RVA | `0x854050` | 0x8543f0 |  | FAIL_OWNER |
| tfm2_comptest_unlock | DRIVE_RVA | `0xc1e037` | 실패 | 0xaecd37 | FAIL_ALIGNED_WMNEM |
| tfm2_comptest_unlock | DRIVE_RESUME_RVA | `0xc1e04a` | 실패 | 0xaecd45 | FAIL_ALIGNED_WMNEM |
| tfm2_comptest_unlock | INLINE@src/tfm2_comptest_unlock.rs:147 | `0x2860cdf` | 0x208597f | 0x2085f67 | DISAGREE_WLOOSE_GAP |
| tfm2_comptest_unlock | INLINE@src/tfm2_comptest_unlock.rs:137 | `0x2864e56` | 0x2089af6 | 0x2089c3f | DISAGREE_WLOOSE_GAP |
| tfm2_comptest_unlock | CT_REGION_HI | `0x287d008` | 실패 |  | FAIL |
| tfm2_comptest_unlock | RUNNER_VT_RVA | `0x419b3b0` | 실패 |  | FAIL |
| tfm2_elemental_serpen | LAUNCHER_RET_A | `0xbc1695` | 실패 | 0xa90240 | FAIL_ALIGNED_WLOOSE |
| tfm2_elemental_serpen | LAUNCHER_RET_B | `0xbc5a04` | 실패 | 0xa945af | FAIL_ALIGNED_WLOOSE_GAP |
| tfm2_elemental_serpen | MOBATICK_RVA | `0x16b4050` | 실패 |  | FAIL |
| tfm2_elemental_serpen | LAUNCHER_RET_C | `0x2de7693` | 0x23da7a3 | 0x23da78f | DISAGREE_EXACT |
| tfm2_transfer_fix | RIP_SITES | `0x1bdc25a` | 실패 | 0x140ddda | FAIL_ALIGNED_LOOSE_GAP |
| tfm2_transfer_fix | RIP_SITES | `0x1bdc28a` | 실패 | 0x140de09 | FAIL_ALIGNED_LOOSE_GAP |
| tfm2_transfer_fix | RIP_SITES | `0x1bdc2f1` | 실패 | 0x140de73 | FAIL_ALIGNED_LOOSE_GAP |
| tfm2_transfer_fix | RVA_S_MAX | `0x1bdc2f9` | 실패 | 0x140de7b | FAIL_ALIGNED_EXACT |
| tfm2_transfer_fix | RIP_SITES | `0x1bdc2fd` | 실패 | 0x140de7f | FAIL_ALIGNED_LOOSE_GAP |
| tfm2_transfer_fix | INLINE@src/lib.rs:65 | `0x1bdc45b` | 실패 | 0x140dfdb | FAIL_ALIGNED_LOOSE_GAP |
| tfm2_transfer_fix | RIP_SITES | `0x40820b8` | 실패 |  | FAIL |
| ui_kit | RVA_DROP | `0x27a29e0` | 실패 |  | FAIL |
| ui_kit | RVA_UPDATE | `0x2fd22e0` | 실패 |  | FAIL |

</details>

<details><summary>(참고) 생성물 모드 147건 — repin 판정 불필요(③④ 도구가 따로 판정) · 앞 30</summary>

| 모드 | 이름 | 구 | repin | aligned | 판정 |
|---|---|---|---|---|---|
| tfm2_detail_tactics | CALLER_RETURN_0xe3aa2a | `0xe3aa2a` | 0xe3c8ba |  | NO_ALIGN |
| tfm2_detail_tactics | CALLER_RETURN_0xe3b256 | `0xe3b256` | 0xe3d0e6 | 0xe3cc60 | DISAGREE_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xe3d913 | `0xe3d913` | 0xe3f7a3 | 0xe3f2b1 | DISAGREE_WMNEM |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xe3d93d | `0xe3d93d` | 0xe3f7cd | 0xe3f2db | DISAGREE_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xe3e246 | `0xe3e246` | 0xe400d6 | 0xe3fc54 | DISAGREE_WMNEM |
| tfm2_detail_tactics | liner_roam_self_hp_41@0xe3e78c | `0xe3e78c` | 0xe4061c | 0xe40164 | DISAGREE_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xe3e7bc | `0xe3e7bc` | 0xe4064c | 0xe40194 | DISAGREE_WMNEM |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xe3eafb | `0xe3eafb` | 0xe4098b | 0xe404bb | DISAGREE_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xe3ecb9 | `0xe3ecb9` | 0xe40b49 | 0xe4067c | DISAGREE_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xe3f3ae | `0xe3f3ae` | 0xe4123e |  | NO_ALIGN |
| tfm2_detail_tactics | extra_c008@0xe3f6d0 | `0xe3f6d0` | 0xe41560 | 0xe413e8 | DISAGREE_WLOOSE |
| tfm2_detail_tactics | CALLER_RETURN_0xe40c9e | `0xe40c9e` | 0xe42b2e | 0xe42a34 | DISAGREE_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xe42d67 | `0xe42d67` | 0xe44bf7 |  | NO_ALIGN |
| tfm2_detail_tactics | extra_c029@0xe42e1e | `0xe42e1e` | 0xe44cae |  | NO_ALIGN |
| tfm2_detail_tactics | liner_roam_self_hp_41@0xe42f1a | `0xe42f1a` | 0xe44daa |  | NO_ALIGN |
| tfm2_detail_tactics | CALLER_RETURN_0xe42f9f | `0xe42f9f` | 0xe44e2f |  | NO_ALIGN |
| tfm2_detail_tactics | CALLER_RETURN_0xe4331b | `0xe4331b` | 0xe451ab | 0xe45105 | DISAGREE_WMNEM |
| tfm2_detail_tactics | liner_roam_call_chance_700@0xe4a14b | `0xe4a14b` | 0xe4bfdb | 0xe4cb1d | DISAGREE_WLOOSE_GAP |
| tfm2_detail_tactics | CALLER_RETURN_0xe54328 | `0xe54328` | 0xe58068 | 0xe58367 | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0xe55c86 | `0xe55c86` | 0xe599c6 | 0xe59d34 | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0xe56339 | `0xe56339` | 0xe5a459 | 0xe5a473 | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0xe56379 | `0xe56379` | 0xe5a499 | 0xe5a4b7 | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c017@0xe62e7c | `0xe62e7c` | 실패 | 0xe66a2d | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xe63090 | `0xe63090` | 실패 | 0xe66c68 | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | extra_c004@0xe637c5 | `0xe637c5` | 실패 | 0xe67372 | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | extra_c004@0xe638dd | `0xe638dd` | 실패 | 0xe674b3 | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | extra_c004@0xe639a3 | `0xe639a3` | 실패 | 0xe67372 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c004@0xe63a69 | `0xe63a69` | 실패 | 0xe674b3 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c004@0xe63b49 | `0xe63b49` | 실패 | 0xe6772f | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c006@0xe6694b | `0xe6694b` | 0xe6a4eb |  | NO_ALIGN |

</details>

## 5. 구조체 오프셋 (재핀 후보값 기준 지문 대조)

- tfm2_level_cap — 기준선 지문 없음
- tfm2_champion_exclude — 일치
- banpick_view_plus — 일치
- ★**tfm2_comptest_unlock — 변화 11개 함수**
  - `ORACLE_RVA` 0x1896cf0→0x1474070 Δ-0x60 ×38 
  - `CTX_DROP_RVA` 0x257fa80→0x2806360 Δ-0x60 ×17 ⚠재핀 후보가 CLONE — 이 행은 엉뚱한 함수를 본 것일 수 있다(재핀 판정 후 다시)
  - `REFRESH_RVA` 0x2e9e260→0x2831650 Δ-0x60 ×4 
  - `CSEND_RVA` 0x2ea4f60→0x2838340 Δ-0x60 ×20 
  - `CGATE_RVA` 0x2ea8d10→0x283c0f0 Δ-0x60 ×3 
  - `RUN_RVA` 0x2eb9920→0x284cd00 Δ-0x60 ×15 
  - `RPLY3_RVA` 0x2ebf6d0→0x2852ab0 Δ-0x60 ×1 
  - `RPLY2_RVA` 0x2ed0480→0x2863860 Δ-0x60 ×7 
  - `ARRIVE_RVA` 0x2ed0cf0→0x28640d0 Δ-0x60 ×10 
  - `CT_REGION_LO` 0x284eed0→0x2073b70 Δ+0x30 ×18 
  - `CTX_CLONE_RVA` 0x26760c0→0x2d3acf0 Δ-0x60 ×21 
- ★**tfm2_elemental_serpen — 변화 7개 함수**
  - `SPAWN_HOOKS` 0xa00e70→0x921fc0 Δ-0x60 ×3 
  - `SPAWN_HOOKS` 0xa01ba0→0x922cf0 Δ-0x60 ×3 
  - `SERPEN_RVA` 0x1d49aa0→0x1aa38e0 - 사라짐 0x118,0x120 / 생김 -
  - `DMGA_RVA` 0x167a710→0x149ad70 - 사라짐 0x11 / 생김 -
  - `LAUNCHER_RVA` 0x189e2a0→0x147baa0 Δ-0x60 ×14 
  - `RUNNER_CTOR_RVA` 0x189fae0→0x147d2f0 Δ-0x60 ×14 
  - `MOBATICK_RVA` 0x16b4050→ - 재핀 실패 — 지문 대조 불가
- tfm2_champ_pos_lock — 일치
- training_view_plus — 기준선 지문 없음
- facility_view_plus — 기준선 지문 없음
- custom_tier_assignment — 기준선 지문 없음
- tfm2_detail_tactics — 기준선 지문 없음
- tfm2_transfer_fix — 일치
- ★**ui_kit — 변화 1개 함수**
  - `RVA_UPDATE` 0x2fd22e0→ - 재핀 실패 — 지문 대조 불가

> apply 뒤 정식 `offsets.py check` → `sources` 는 따로 돈다(이 표는 apply 전 예보). [json](offsets_shifts.json)

## 6. ★상세전술 (tfm2_detail_tactics — 전술 100 · 패치창 218)

- (track.json 없음 — 단계 상태: 건너뜀 --only)

## 7. 주소록 (addrbook — KB 에 적힌 함수·지점)

- (주소록 산출물 없음 — 단계 상태: 건너뜀 --only)

## 8. 핫픽스 노출 표 (게임을 켜기 전에 볼 것)

- (`F:\claude\tfm2mods\MIG\patchday\hotfix_exposure_0.7.0.json` 없음)

## 9. 다음에 사람이 할 일 (순서대로 — 정확한 명령 = MIG\patchday\README.md)

1. **게임은 아직 켜지 말 것** — 8절 노출 표에서 낡은 주소로 패치하는 모드가 있는지 확인
2. 6절 상세전술: `detail_tactics\track.md`·review_queue → 사람 판정 → draft_targets 승격 → `gen_sites.py --ver 0.7.1` → `build_local.ps1 -RunTests` → 인게임 검증
3. tfm2_tactics_verify: `tools\migrate_targets.py` (sites.rs 생성물 — repin apply 대상 아님)
4. 4절 apply 대상 잔여 판정 22건(+동형 클론) → `MIG\_mkfinal071.py` 로 final 맵 → `repin.py apply <MOD…> --write`(생성물 모드 tfm2_detail_tactics·tfm2_tactics_verify 제외) → `rebase` → `coverage` → `offsets.py check`·`sources` → `bump_deps.py` → `env.py` → `run.py` 0
5. 2절 EDITED 함수 = 재현코드(ai_adjust·judge 계층)에 닿는지 확인
6. 7절 `addrmap.md` 구→신 표로 KB 갱신(record-keeper 경유)
7. REPORT `_0.7.1_마이그.md`·모드별 03_시행착오·DONE.md 기록
