# 패치날 요약 — 0.7.2

> 생성 2026-11-11 14:42:26 · `MIG\patchday\run_patchday.py` · 전체 소요 1114s · 출력 = `F:\claude\tfm2mods\MIG\_patchday\0.7.2`

## 0. 버전

| | 기준(구) | 신 |
|---|---|---|
| 라벨 | 0.7.1 | **0.7.2** |
| sha256 | `BD011C426A3F5ED4C411C7D9379448D306E091A9BD83E5E201C81D3EBFDD1D6B` | `4B324F3127953A67EE1972F0D8CEB0673BB4FACCAAB530F0538DA64DD70319FB` |
| 크기 | 94043136 B | 94603776 B |
| exe | `F:\claude\tfm2\tfm2_0.7.1\TeamfightManager2.exe` | `F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe` |
| base_version.txt | — | `0.7.2` (C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mod-sdk-stable\base_version.txt) |

## 1. 단계 결과

| # | 단계 | 결과 | 종료코드 | 소요 | 메모 | 로그 |
|---|---|---|---|---|---|---|
| 0 | detect | 정상 | 0 | 0.6s | 0.7.1 → 0.7.2 · sha 4B324F3127953A67 · 94603776 B | [log](logs/00_detect.log) |
| 1 | backup | 정상 | 0 | 6.5s | TeamfightManager2.exe: 복사 완료 94603776 B · sha 4B324F3127953A67 / bundle.game_data: 복사 완료 1156635523 B · sha BD9FF71E1E1A5280 | [log](logs/01_backup.log) |
| 2 | fnindex | 정상 | 0 | 145.0s | 기준 _fnidx_071.pkl: 검증 통과 — 메타 경로 sha 일치 · 경계 166780/166780 일치 · skel 표본 48 일치 / 생성 후 검증: 메타 경로 sha 일치 · 경계 167313/167313 일치 · skel 표본 48 일치 | [log](logs/02_fnindex.log) |
| 3 | callgraph | 정상 | 0 | 95.5s | 기준 _cg_071.pkl: 검증 통과 — callee 147375개 · 표본 40 일치 / 생성 후 검증: callee 147611개 · 표본 40 일치 | [log](logs/03_callgraph.log) |
| 4 | aidiff | 정상·검토 필요 | 0 | 417.5s | EDITED 157 · MERGED 0 · 사라짐 10 · 새로 생김 159 | [log](logs/04_aidiff.log) |
| 5 | check | 정상·검토 필요 | 1 | 0.6s | STALE·INVALID 454 (모드 12개 중 9개) | [log](logs/05_check.log) |
| 6 | repin | 정상·검토 필요 | 0 | 274.2s | 대상 454 · 엔진 해결 329 · 실패 125 · 사람 판정 필요 208(apply 대상 22 + 생성물 모드 186) · 동형 클론 주의 9 | [log](logs/06_repin.log) |
| 7 | offsets | 정상·검토 필요 | 1 | 2.9s | 변화 3개 모드 · 10개 함수 (기준 = repin_map.aligned.json) | [log](logs/07_offsets.log) |
| 8 | detail_tactics | 정상·검토 필요 | 1 | 169.0s | 검토 필요 항목 있음(정상) · 전술 AUTO 18 · REVIEW 82 · 창 208(자동 41) · 기본값 변경 전술 1 | [log](logs/08_detail_tactics.log) |
| 9 | addrbook | 건너뜀 |  | 0.0s | --skip |  |
| 10 | summary | 정상 | 0 | 0s | SUMMARY.md |  |

## 2. AI 판단계층 (aidiff)

- ★**EDITED 157개** · MERGED 0 · SHIFTED 393 · IDENTICAL 222 · 사라짐 10 · 새로 생김 159 → 전문 [aidiff.md](aidiff.md)
  - `game-ai\src\action_score.rs` `0x108fa10`→`0x11525a0`
  - `game-ai\src\action_score.rs` `0x1086710`→`0x1148f50`
  - `game-ai\src\action_score.rs` `0x1084b20`→`0x11565f0`
  - `game-ai\src\action_score.rs` `0x1021050`→`0xf20c80`
  - `game-ai\src\action_score.rs` `0x1155100`→`0xf93af0`
  - `game-ai\src\action_score.rs` `0x1155320`→`0xf93d10`
  - `game-ai\src\action_score.rs` `0x1093f10`→`0x1147850`
  - `game-ai\src\ai_utils.rs` `0xf54700`→`0x10356a0`
  - `game-ai\src\ai_utils.rs` `0x1128f50`→`0x10afa10`
  - `game-ai\src\buff_value.rs` `0x10fb9f0`→`0x1114520`
  - `game-ai\src\fight_check.rs` `0xf93e00`→`0xf12440`
  - `game-ai\src\fight_check.rs` `0xf91c70`→`0xf101e0`
  - `game-ai\src\fight_check.rs` `0xf8e820`→`0xf0c8a0`
  - `game-ai\src\fight_check.rs` `0xf990e0`→`0xf18680`
  - `game-ai\src\fight_check.rs` `0xf887f0`→`0xf0e5f0`
  - `game-ai\src\fight_check.rs` `0xf93140`→`0xf11700`
  - `game-ai\src\fight_check.rs` `0xf8af90`→`0xf07fa0`
  - `game-ai\src\fight_check.rs` `0xf8cef0`→`0xf0ac60`
  - `game-ai\src\finish_race.rs` `0xf15990`→`0xee61b0`
  - `game-ai\src\finish_race.rs` `0xf16310`→`0xee7410`
  - `game-ai\src\finish_race.rs` `0xf7c420`→`0x1016620`
  - `game-ai\src\finish_race.rs` `0xf9cbf0`→`0x101e020`
  - `game-ai\src\finish_race.rs` `0xf16070`→`0xee7140`
  - `game-ai\src\finish_race.rs` `0x1122fc0`→`0x10a8ca0`
  - `game-ai\src\finish_race.rs` `0xf9c1b0`→`0x101d560`
  - `game-ai\src\finish_race.rs` `0x1122bd0`→`0x10a89d0`
  - `game-ai\src\finish_race.rs` `0x111ee00`→`0x108c840`
  - `game-ai\src\finish_race.rs` `0xf7c5a0`→`0x10168e0`
  - `game-ai\src\finish_race.rs` `0xf7ca20`→`0xeed250`
  - `game-ai\src\geometry.rs` `0x11152e0`→`0x1132770`
  - `game-ai\src\goal_data.rs` `0xf589d0`→`0xfc4820`
  - `game-ai\src\goal_data.rs` `0xf59590`→`0xfc51e0`
  - `game-ai\src\lib.rs` `0x1024470`→`0x1170c60`
  - `game-ai\src\observe.rs` `0x1026e90`→`0x1176960`
  - `game-ai\src\path_field.rs` `0x1133960`→`0x1095f70`
  - `game-ai\src\path_field.rs` `0x112fd00`→`0x10972c0`
  - `game-ai\src\path_field.rs` `0x1139000`→`0x109b5e0`
  - `game-ai\src\path_field.rs` `0x113a960`→`0x109c6c0`
  - `game-ai\src\path_field.rs` `0x113c2c0`→`0x1098610`
  - `game-ai\src\path_field.rs` `0x113dc20`→`0x1099600`
  - … 외 117개

## 3. 매니페스트 STALE (mig_verify check)

| 모드 | PASS | STALE·INVALID | (INVALID) |
|---|---|---|---|
| tfm2_level_cap | 0 | **2** |  |
| tfm2_champion_exclude | 0 | **1** |  |
| banpick_view_plus | 9 | **19** |  |
| tfm2_comptest_unlock | 4 | **68** |  |
| tfm2_elemental_serpen | 2 | **16** |  |
| tfm2_champ_pos_lock | 0 | **4** |  |
| training_view_plus | 0 | 0 |  |
| facility_view_plus | 0 | 0 |  |
| custom_tier_assignment | 0 | 0 |  |
| tfm2_detail_tactics | 0 | **324** |  |
| tfm2_transfer_fix | 0 | **10** |  |
| ui_kit | 1 | **10** |  |

합계 STALE·INVALID = **454** · 전문 [log](logs/05_check.log)

## 4. 재핀 계획 (repin plan + sitealign) — ★apply 안 함

- 대상 454 · 엔진 해결 329 · 실패 125 · 사람 판정 필요 208 (실패 + sitealign AGREE 아님 + 함수시작 WEAK) · 동형 클론 주의 9 · 파일 [repin_map.aligned.json](repin_map.aligned.json)
  - ★**repin apply 대상 사람 판정 = 22건** (아래 표 전부) · 생성물 모드 186건(tfm2_detail_tactics)은 **repin 판정 불필요** — sites.rs 가 생성물이라 6절 상세전술(③ track_update)·④ migrate_targets 가 따로 추적한다(참고용으로만 repin_review.md 뒤쪽에 둔다)
- 엔진 판별 종류: BYTES_UNIQUE=185, FAIL=125, MID_OWNER_CG_NONE=47, MID_OWNER_HEAD_UNIQUE=47, FN_UNIQUE=20, MID_OWNER_UNIQUE=14, MID_OWNER_CG_NONE_caller=7, MID_OWNER_CG_MULTI_caller=5, MID_OWNER_CG_MULTI=3, FN_HEAD_UNIQUE=1
- sitealign(함수 중간 사이트): AGREE=185, DISAGREE_EXACT=47, DISAGREE_LOOSE_GAP=1, DISAGREE_MNEM=9, DISAGREE_WLOOSE=2, DISAGREE_WLOOSE_GAP=2, DISAGREE_WMNEM=3, FAIL_ALIGNED_EXACT=26, FAIL_ALIGNED_LOOSE=1, FAIL_ALIGNED_LOOSE_GAP=2, FAIL_ALIGNED_MNEM=1, FAIL_ALIGNED_WLOOSE=1, FAIL_ALIGNED_WMNEM=12, FAIL_NO_ALIGN=15, FAIL_OWNER=62, NO_ALIGN=13
- 함수시작 교차검증(콜러 수·callee 사영·size 비 2/3, [repin_fncheck.json](repin_fncheck.json)): CLONE=9, STRONG=41, WEAK=2
- ⚠「사람 판정 필요」 밖(AGREE·STRONG)도 **후보**다 — apply 전 MIG\README 0.6.0 절 규칙으로 최종 확정.

| 모드 | 대상 | 해결 | 실패 | 정렬 AGREE | 함수 WEAK | 클론 | 사람 판정 |
|---|---|---|---|---|---|---|---|
| tfm2_level_cap | 2 | 2 | 0 | 2 | 0 | 0 | 0 |
| tfm2_champion_exclude | 1 | 1 | 0 | 0 | 0 | 0 | 0 |
| banpick_view_plus | 19 | 19 | 0 | 6 | 0 | 0 | 0 |
| tfm2_comptest_unlock | 68 | 64 | 4 | 37 | 0 | 3 | **6** |
| tfm2_elemental_serpen | 16 | 14 | 2 | 2 | 1 | 1 | **5** |
| tfm2_champ_pos_lock | 4 | 4 | 0 | 0 | 0 | 0 | 0 |
| training_view_plus | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| facility_view_plus | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| custom_tier_assignment | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| tfm2_detail_tactics (생성물 — ③④, apply 안 함) | 324 | 215 | 109 | 138 | 0 | 0 | 186(참고) |
| tfm2_transfer_fix | 10 | 2 | 8 | 0 | 0 | 0 | **8** |
| ui_kit | 10 | 8 | 2 | 0 | 1 | 5 | **3** |

<details><summary>동형 클론 주의 9건 (같은 skel 의 함수가 신 exe 에 여럿 — 콜러 문맥으로 확인)</summary>

| 모드 | 이름 | 구 | 후보 | 비고 |
|---|---|---|---|---|
| tfm2_comptest_unlock | LOADER_RVA | `0x3a93f0` | 0x3a92f0 | MID_OWNER_CG_MULTI · 같은 skel 28개 |
| tfm2_comptest_unlock | CTX_DROP_RVA | `0x2806360` | 0x23b1bb0 | MID_OWNER_CG_MULTI_caller · 같은 skel 6개 |
| tfm2_comptest_unlock | CLONE_CHAMP_RVA | `0x2125af0` | 0x219b7e0 | MID_OWNER_CG_MULTI_caller · 같은 skel 6개 |
| tfm2_elemental_serpen | UILOADER_RVA | `0x3a93f0` | 0x3a92f0 | MID_OWNER_CG_MULTI · 같은 skel 28개 |
| ui_kit | RVA_DROP_MAP | `0x351d10` | 0x351c10 | MID_OWNER_CG_MULTI · 같은 skel 2개 |
| ui_kit | RVA_INSERT | `0x9520f0` | 0xd12ac0 | MID_OWNER_CG_MULTI_caller · 같은 skel 5개 |
| ui_kit | RVA_CLONE | `0x213cff0` | 0x21b2ce0 | MID_OWNER_CG_MULTI_caller · 같은 skel 2개 |
| ui_kit | RVA_SEND | `0x21ecc70` | 0x225b090 | BYTES_UNIQUE · 같은 skel 5개 |
| ui_kit | RVA_RESPONSE | `0x202ee60` | 0x20a1910 | BYTES_UNIQUE · 같은 skel 2개 |

</details>
- 전 목록 = [repin_review.md](repin_review.md)

<details open><summary>★repin apply 대상 사람 판정 22건</summary>

| 모드 | 이름 | 구 | repin | aligned | 판정 |
|---|---|---|---|---|---|
| tfm2_comptest_unlock | A15E20_RVA | `0x8543f0` | 실패 |  | FAIL_OWNER |
| tfm2_comptest_unlock | HPUSH_RVA | `0x17cdfa0` | 실패 |  | FAIL |
| tfm2_comptest_unlock | INLINE@src/tfm2_comptest_unlock.rs:137 | `0x2089c3f` | 0x20fd62f | 0x20fd5a8 | DISAGREE_WLOOSE_GAP |
| tfm2_comptest_unlock | CT_REGION_HI | `0x20a2080` | 실패 |  | FAIL |
| tfm2_comptest_unlock | CT_CLIENT_LO | `0x201eddc` | 0x2090fcc | 0x2090f9f | DISAGREE_MNEM |
| tfm2_comptest_unlock | RUNNER_VT_RVA | `0x42134e0` | 실패 |  | FAIL |
| tfm2_elemental_serpen | LAUNCHER_RET_A | `0xa90240` | 0x18cefe0 |  | NO_ALIGN |
| tfm2_elemental_serpen | LAUNCHER_RET_B | `0xa945af` | 0x18d334f |  | NO_ALIGN |
| tfm2_elemental_serpen | SERPEN_RVA | `0x1aa38e0` | 실패 |  | FAIL |
| tfm2_elemental_serpen | DMGA_RVA | `0x149ad70` | 0x18675c1 |  | FN_WEAK(후보가 신 함수시작 아님) |
| tfm2_elemental_serpen | MOBATICK_RVA | `0x193fa40` | 실패 |  | FAIL |
| tfm2_transfer_fix | RIP_SITES | `0x140ddda` | 실패 |  | FAIL_OWNER |
| tfm2_transfer_fix | RIP_SITES | `0x140de09` | 실패 |  | FAIL_OWNER |
| tfm2_transfer_fix | RIP_SITES | `0x140de73` | 실패 |  | FAIL_OWNER |
| tfm2_transfer_fix | RVA_S_MAX | `0x140de7b` | 실패 |  | FAIL_OWNER |
| tfm2_transfer_fix | RIP_SITES | `0x140de7f` | 실패 |  | FAIL_OWNER |
| tfm2_transfer_fix | INLINE@src/lib.rs:62 | `0x140dfdb` | 실패 |  | FAIL_OWNER |
| tfm2_transfer_fix | RIP_SITES | `0x4104d90` | 실패 |  | FAIL |
| tfm2_transfer_fix | RIP_SITES | `0x4104d98` | 실패 |  | FAIL |
| ui_kit | RVA_DROP | `0x1fb4cf0` | 실패 |  | FAIL |
| ui_kit | RVA_BUILD | `0x20cfbe0` | 0x2897a40 |  | FN_WEAK(BYTES_UNIQUE · 콜러 [2, 1] · 사영 0.5 · size 1.118) |
| ui_kit | RVA_UPDATE | `0x2957ac0` | 실패 |  | FAIL |

</details>

<details><summary>(참고) 생성물 모드 186건 — repin 판정 불필요(③④ 도구가 따로 판정) · 앞 30</summary>

| 모드 | 이름 | 구 | repin | aligned | 판정 |
|---|---|---|---|---|---|
| tfm2_detail_tactics | CALLER_RETURN_0xe3aca4 | `0xe3aca4` | 실패 | 0xe3ccd5 | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xe3cc60 | `0xe3cc60` | 실패 | 0xe3eeef | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xe3e91c | `0xe3e91c` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | CALLER_RETURN_0xe3f2b1 | `0xe3f2b1` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xe3f2db | `0xe3f2db` | 실패 | 0xe41ecc | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xe3fc54 | `0xe3fc54` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | liner_roam_self_hp_41@0xe40164 | `0xe40164` | 실패 | 0xe424c9 | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xe40194 | `0xe40194` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | CALLER_RETURN_0xe4020f | `0xe4020f` | 0xe430a2 |  | NO_ALIGN |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xe404b4 | `0xe404b4` | 실패 | 0xe42623 | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xe4067c | `0xe4067c` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | CALLER_RETURN_0xe4073d | `0xe4073d` | 실패 | 0xe42844 | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | extra_c008@0xe413e8 | `0xe413e8` | 실패 | 0xe4342e | FAIL_ALIGNED_WLOOSE |
| tfm2_detail_tactics | CALLER_RETURN_0xe42a34 | `0xe42a34` | 실패 | 0xe46563 | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xe45105 | `0xe45105` | 실패 | 0xe48e5a | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xe4b3d9 | `0xe4b3d9` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | extra_c029@0xe4b490 | `0xe4b490` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | liner_roam_self_hp_41@0xe4b56a | `0xe4b56a` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | CALLER_RETURN_0xe4b5ef | `0xe4b5ef` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | liner_roam_call_chance_700@0xe4cb1d | `0xe4cb1d` | 실패 | 0xe50a81 | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xe54814 | `0xe54814` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | CALLER_RETURN_0xe58367 | `0xe58367` | 0xe5d217 | 0xe5d264 | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0xe59d34 | `0xe59d34` | 0xe5ebe4 | 0xe5ecf0 | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c017@0xe66a2d | `0xe66a2d` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xe66c68 | `0xe66c68` | 실패 | 0xe6b8c1 | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | extra_c004@0xe67372 | `0xe67372` | 실패 | 0xe6c7d8 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c004@0xe674b3 | `0xe674b3` | 실패 | 0xe6c8a9 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c004@0xe67587 | `0xe67587` | 실패 | 0xe6c97d | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c004@0xe6765b | `0xe6765b` | 실패 | 0xe6ca43 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c004@0xe6772f | `0xe6772f` | 실패 | 0xe6cb09 | FAIL_ALIGNED_EXACT |

</details>

## 5. 구조체 오프셋 (재핀 후보값 기준 지문 대조)

- tfm2_level_cap — 기준선 지문 없음
- tfm2_champion_exclude — 일치
- banpick_view_plus — 일치
- ★**tfm2_comptest_unlock — 변화 4개 함수**
  - `HPUSH_RVA` 0x17cdfa0→ - 재핀 실패 — 지문 대조 불가
  - `ORACLE_RVA` 0x1474070→0x145aa60 Δ+0x58 ×5 
  - `CT_REGION_LO` 0x2073b70→0x20e7560 Δ+0x1 ×1 
  - `CT_REGION_HI` 0x20a2080→ - 재핀 실패 — 지문 대조 불가
- ★**tfm2_elemental_serpen — 변화 5개 함수**
  - `SERPEN_RVA` 0x1aa38e0→ - 재핀 실패 — 지문 대조 불가
  - `DMGB_RVA` 0x14a4a20→0x149fea0 Δ+0x8 ×4 
  - `DMGA_RVA` 0x149ad70→0x18675c1 - 재핀 후보값이 함수시작 아님
  - `LAUNCHER_RVA` 0x147baa0→0x147baa0 Δ-0x1ad0 ×7 
  - `MOBATICK_RVA` 0x193fa40→ - 재핀 실패 — 지문 대조 불가
- tfm2_champ_pos_lock — 일치
- training_view_plus — 기준선 지문 없음
- facility_view_plus — 기준선 지문 없음
- custom_tier_assignment — 기준선 지문 없음
- tfm2_detail_tactics — 기준선 지문 없음
- tfm2_transfer_fix — 일치
- ★**ui_kit — 변화 1개 함수**
  - `RVA_UPDATE` 0x2957ac0→ - 재핀 실패 — 지문 대조 불가

> apply 뒤 정식 `offsets.py check` → `sources` 는 따로 돈다(이 표는 apply 전 예보). [json](offsets_shifts.json)

## 6. ★상세전술 (tfm2_detail_tactics — 전술 100 · 패치창 218)

- 전술 100개 → **자동확정(AUTO) 18 · 검토 필요(REVIEW) 82** · 창 208 (자동 41)

| 전술의 가장 나쁜 창 판정 | 뜻 | 전술 수 | 창 수 |
|---|---|---|---|
| `SAME` | 이동만(자동확정) | 18 | 42 |
| `REG_RENAME` | 레지스터·스택 재배치 | **27** | 71 |
| `VALUE_CHANGED` | 게임 기본값 변경 | 0 | 0 |
| `COPIES` | 사본 분기 | 0 | 0 |
| `LOGIC_CHANGED` | 로직 변경 | **39** | 67 |
| `SITE_MISSING` | 창 소실 | **15** | 27 |
| `OWNER_MISSING` | 함수 소실 | **1** | 1 |

- 전문: [track.md](detail_tactics/track.md) · [review_queue.json](detail_tactics/review_queue.json) · 초안 `detail_tactics\draft_targets\`

### ★게임이 기본값을 바꾼 전술 (1)

| # | 탭 | 전술 | 구 기본값 → 신 | 창 |
|---|---|---|---|---|
| 27 | 기지 방어 | 기지 방어 출동 기준 적 수 | 6 → **2** (즉치 5→1) | `base_defense_min_enemies_6@0xf3b00c` |

### 검토 필요 전술 (82 · 나쁜 판정 순 앞 30)

- [34] **구조물 공격 중 후퇴 유지 체력(%)** (`structure_retreat_keep_hp`, 숨김) — `OWNER_MISSING` 함수 소실 · extra_c003@0x10827b5: OWNER_MISSING: owner 0x1081390 대응 없음 (Location LOC_GONE · repin NONE skel 후보0 / head후보 0 | midpin A:ALIGN_AMBIG 0.06/0.06)
- [7] **위험 무시 체력(%)** (`self_risk_ignore_hp`, 전투·생존) — `SITE_MISSING` 창 소실 · self_risk_ignore_hp_35@0x10940dc: SITE_MISSING: owner 0x1093f10→0x1147850 안에서 창 대응 실패 (꼴 후보 0개 []) · 참고: 값 조각 `imul rdx, rcx, 0x23` 와 같은 꼴·즉치 위치 1개 ['0x1147abd'
- [14] **정글링 중단 체력(%)** (`jungle_stop_hp`, 정글) — `SITE_MISSING` 창 소실 · jungle_stop_hp_41@0x10e5021: SITE_MISSING: Caller 반환 콜사이트 0xe4067c 소실 — call 정렬 실패 · 같은 꼴 후보 611 → 호출 대상 0xf864a0 로 거르면 0
- [27] **기지 방어 출동 기준 적 수** (`base_defense_min_enemies`, 기지 방어) — `SITE_MISSING` 창 소실 · base_defense_min_enemies_6@0xf3af6c: SITE_MISSING: owner 0xf3ae10→0xfc79f0 안에서 창 대응 실패 (꼴 후보 0개 [])
- [33] **픽오프 저체력 기준(%)** (`pickoff_hp_bands`, 숨김) — `SITE_MISSING` 창 소실 · extra_c002@0xeaf6e6: SITE_MISSING: owner 0xeaacf0→0xeb3ee0 안에서 창 대응 실패 (꼴 후보 0개 [])
- [37] **교전 콜 응답 시간** (`battle_call_response_seconds`, 숨김) — `SITE_MISSING` 창 소실 · extra_c006@0xe6a420: SITE_MISSING: Caller 반환 콜사이트 0xe3fc54 소실 — call 정렬 실패 · 같은 꼴 후보 611 → 호출 대상 0xe70590 로 거르면 0
- [61] **보스 리스폰 대기(초)** (`boss_respawn_wait_seconds`, 숨김) — `SITE_MISSING` 창 소실 · extra_c033@0x10f8a73: SITE_MISSING: Caller 반환 콜사이트 0xec8a9c 소실 — caller 함수 대응 없음(NONE skel 후보0 / head후보 0 | midpin A:POOL_SMALL)
- [64] **마무리 전략의 귀환 종료 여유(초)** (`active_recall_slack_profile`, 숨김) — `SITE_MISSING` 창 소실 · extra_c037@0x116065b: SITE_MISSING: Caller 반환 콜사이트 0xec8afe 소실 — caller 함수 대응 없음(NONE skel 후보0 / head후보 0 | midpin A:POOL_SMALL)
- [69] **이동기 절약 보행 여유(업데이트)** (`movement_skill_period_margin`, 위치·기술) — `SITE_MISSING` 창 소실 · extra_c042@0x10b388b: SITE_MISSING: Caller 반환 콜사이트 0x108278f 소실 — caller 함수 대응 없음(NONE skel 후보0 / head후보 0 | midpin A:ALIGN_AMBIG 0.06/0.06)
- [73] **시야 밖 자기 위험 할인(분모)** (`unseen_self_risk_divisor`, 숨김) — `SITE_MISSING` 창 소실 · extra_c049@0x1094dc8: SITE_MISSING: owner 0x1093f10→0x1147850 안에서 창 대응 실패 (꼴 후보 0개 [])
- [76] **전진 후 피해 상쇄 비율(%)** (`forward_damage_credit_percent`, 숨김) — `SITE_MISSING` 창 소실 · extra_c052@0x10959df: SITE_MISSING: owner 0x1093f10→0x1147850 안에서 창 대응 실패 (꼴 후보 0개 [])
- [84] **운영 신호 불사 잔여 허용** (`relay_undying_limit_centiseconds`, 숨김) — `SITE_MISSING` 창 소실 · extra_c061@0xe68def: SITE_MISSING: Caller 반환 콜사이트 0xe3f2b1 소실 — call 정렬 실패 · 같은 꼴 후보 611 → 호출 대상 0xe6e5a0 로 거르면 2
- [87] **서폿 로밍 원딜 최소 체력** (`support_roam_adc_min_hp`, 숨김) — `SITE_MISSING` 창 소실 · extra_d004@0xe77e82: SITE_MISSING: Caller 반환 콜사이트 0xe4b5ef 소실 — call 정렬 실패 · 같은 꼴 후보 611 → 호출 대상 0xe7e620 로 거르면 2
- [88] **구매 귀환 도보시간** (`purchase_recall_walk_ticks`, 라인·이동) — `SITE_MISSING` 창 소실 · extra_n001@0xf5086f: SITE_MISSING: Caller 반환 콜사이트 0x116225b 소실 — caller 함수 대응 없음(NONE skel 후보0 / head후보 0 | cg 판별실패 (1위 4.00 2위 3.83, 후보 18) (caller제한 18->18) |
- [89] **구매 귀환 위협 반경** (`purchase_recall_threat_radius`, 라인·이동) — `SITE_MISSING` 창 소실 · extra_n002@0xf50928: SITE_MISSING: Caller 반환 콜사이트 0x116225b 소실 — caller 함수 대응 없음(NONE skel 후보0 / head후보 0 | cg 판별실패 (1위 4.00 2위 3.83, 후보 18) (caller제한 18->18) |
- [90] **구매 귀환 위협 기억** (`purchase_recall_threat_memory`, 라인·이동) — `SITE_MISSING` 창 소실 · extra_n003@0xf509f3: SITE_MISSING: Caller 반환 콜사이트 0x116225b 소실 — caller 함수 대응 없음(NONE skel 후보0 / head후보 0 | cg 판별실패 (1위 4.00 2위 3.83, 후보 18) (caller제한 18->18) |
- [0] **전투 중 후퇴 체력(%)** (`combat_retreat_hp`, 전투·생존) — `LOGIC_CHANGED` 로직 변경 · combat_retreat_hp_35@0x1087596: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['action_score.rs:878:15', 'action_score.rs:878:93', 'action_score.rs
- [1] **라인·추적 중 즉시 후퇴 체력(%)** (`line_chase_retreat_hp`, 전투·생존) — `LOGIC_CHANGED` 로직 변경 · line_chase_retreat_hp_45@0xfa5f82: LOGIC_CHANGED: 창 안 분기 대상 0xfa614a → 신 0x102b91b ≠ 기대 ? (대상 명령 정렬 안 됨)
- [4] **수적 열세 후퇴 가중치** (`crowd_retreat_weight`, 전투·생존) — `LOGIC_CHANGED` 로직 변경 · crowd_retreat_weight_plus2_300@0x10889f2: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['action_score.rs:878:15', 'action_score.rs:878:93', 'actio
- [8] **갱 시작 체력(%)** (`jg_gank_hp_start`, 정글) — `LOGIC_CHANGED` 로직 변경 · jg_gank_hp_start_50@0x10cff15: LOGIC_CHANGED: 창 0번째 명령 꼴 변화 `cmp qword ptr [rbp + 0x108], 0x32` → `cmp r14, 0x32`
- [9] **갱 최소 체력(%)** (`jg_gank_hp_floor`, 정글) — `LOGIC_CHANGED` 로직 변경 · jg_gank_hp_floor_40@0x10cfdd6: LOGIC_CHANGED: 창 안 분기 대상 0x10cfe01 → 신 0xf6d655 ≠ 기대 0xf6d65c (owner 내부)
- [20] **로밍 최소 자기 체력(%)** (`liner_roam_self_hp`, 숨김) — `LOGIC_CHANGED` 로직 변경 · liner_roam_self_hp_41@0xe4b56a: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['handler.rs:455:10', 'handler.rs:796:22', 'handler.rs:1282:12', 'han
- [22] **로밍 팀 콜 확률(/1000)** (`liner_roam_call_chance`, 숨김) — `LOGIC_CHANGED` 로직 변경 · liner_roam_call_chance_700@0xe4cb1d: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['handler.rs:455:10', 'handler.rs:796:22', 'handler.rs:1282:12',
- [24] **푸시 유지 최소 인원(명)** (`push_sustain_min_healthy`, 푸시·타워) — `LOGIC_CHANGED` 로직 변경 · push_sustain_min_healthy_3@0xeadb7e: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['objective_handlers.rs:48:8', 'objective_handlers.rs:58:8', 'ob
- [25] **푸시 유지 인원 여유(명)** (`push_sustain_margin`, 푸시·타워) — `LOGIC_CHANGED` 로직 변경 · push_sustain_margin_2@0xeadb44: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['objective_handlers.rs:48:8', 'objective_handlers.rs:58:8', 'objecti
- [26] **타워 아래 추격 내 체력(%)** (`tower_chase_own_hp`, 푸시·타워) — `LOGIC_CHANGED` 로직 변경 · tower_chase_own_hp_35@0xf76445: LOGIC_CHANGED: 창 ±8 명령의 비교·분기 구조 변화(정렬 기준)
- [28] **기지 방어 중 후퇴 체력(%)** (`base_defense_hold_retreat_hp`, 기지 방어) — `LOGIC_CHANGED` 로직 변경 · base_defense_hold_retreat_hp_45@0xfa90bc: LOGIC_CHANGED: 창 1번째 명령 꼴 변화 `ja 0xfa91aa` → `jbe 0x11ca241`
- [29] **넥서스 위험 체력(%)** (`base_defense_hold_nexus_hp`, 기지 방어) — `LOGIC_CHANGED` 로직 변경 · base_defense_hold_nexus_hp_50@0xfa907e: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED · 행 시프트 갈림: ['defense_nexus.rs:30:8'] 등은 그대로, 나머지 행만 밀림 = 함수 중간 편집 의심) + 창 주변 변
- [30] **기지 방어 피해율 문턱(%)** (`base_defense_hold_dmg_ratio`, 기지 방어) — `LOGIC_CHANGED` 로직 변경 · base_defense_hold_dmg_ratio_49@0xfa9000: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED · 행 시프트 갈림: ['defense_nexus.rs:30:8'] 등은 그대로, 나머지 행만 밀림 = 함수 중간 편집 의심) + 창 주변 
- [35] **갱 도착 전 라인 선공** (`pre_gank_engage_profile`, 숨김) — `LOGIC_CHANGED` 로직 변경 · extra_c004@0xe67372: LOGIC_CHANGED: owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 ['engage.rs:263:103', 'engage.rs:274:15', 'engage.rs:283:95', 'engage.rs:286:58'
- … 외 52 (track.md)
- 검토 대기열 파일: [review_queue.json](detail_tactics/review_queue.json)

## 7. 주소록 (addrbook — KB 에 적힌 함수·지점)

- (주소록 산출물 없음 — 단계 상태: 건너뜀 --skip)

## 8. 핫픽스 노출 표 (게임을 켜기 전에 볼 것)

- (`F:\claude\tfm2mods\MIG\patchday\hotfix_exposure_0.7.1.json` 없음)

## 9. 다음에 사람이 할 일 (순서대로 — 정확한 명령 = MIG\patchday\README.md)

1. **게임은 아직 켜지 말 것** — 8절 노출 표에서 낡은 주소로 패치하는 모드가 있는지 확인
2. 6절 상세전술: `detail_tactics\track.md`·review_queue → 사람 판정 → draft_targets 승격 → `gen_sites.py --ver 0.7.2` → `build_local.ps1 -RunTests` → 인게임 검증
3. tfm2_tactics_verify: `tools\migrate_targets.py` (sites.rs 생성물 — repin apply 대상 아님)
4. 4절 apply 대상 잔여 판정 22건(+동형 클론) → `MIG\_mkfinal072.py` 로 final 맵 → `repin.py apply <MOD…> --write`(생성물 모드 tfm2_detail_tactics·tfm2_tactics_verify 제외) → `rebase` → `coverage` → `offsets.py check`·`sources` → `bump_deps.py` → `env.py` → `run.py` 0
5. 2절 EDITED 함수 = 재현코드(ai_adjust·judge 계층)에 닿는지 확인
6. 7절 `addrmap.md` 구→신 표로 KB 갱신(record-keeper 경유)
7. REPORT `_0.7.2_마이그.md`·모드별 03_시행착오·DONE.md 기록
