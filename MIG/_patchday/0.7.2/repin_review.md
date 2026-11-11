# 재핀 사람 판정 대기열 — 0.7.2 (구 0.7.1 → 신 0.7.2)

> run_patchday 생성. 판정 결과는 `MIG\_mkfinal072.py`(= `_mkfinal065.py` 복제) 의 FIX 표로 → `MIG\repin_map_072.final.json`.
> 근거 원문은 REPORT `<MOD>\RE\YYYY-MM-DD_0.7.2-재핀판정.md` 로 저장(§11).

## ★repin apply 대상 — 사람 판정 필요 22

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

## 동형 클론 주의 9

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

## (참고) 생성물 모드 186 — repin 판정 불필요

> tfm2_detail_tactics·tfm2_tactics_verify 는 sites.rs 생성물이라 `repin apply` 대상이 아니다. 상세전술 = `detail_tactics\track.md`(README ③), tactics_verify = `tools\migrate_targets.py`(README ④) 가 따로 판정한다. 이 표는 교차 확인용.

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
| tfm2_detail_tactics | extra_c061@0xe68def | `0xe68def` | 실패 | 0xe6e7c2 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c006@0xe6a420 | `0xe6a420` | 0xe712eb |  | NO_ALIGN |
| tfm2_detail_tactics | extra_c006@0xe6a6b0 | `0xe6a6b0` | 0xe70db0 |  | NO_ALIGN |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xe6b421 | `0xe6b421` | 0xe71ae1 | 0xe71b0b | DISAGREE_EXACT |
| tfm2_detail_tactics | liner_roam_lane_healthy_hp_41@0xe789b0 | `0xe789b0` | 실패 | 0xe7faec | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | liner_roam_lane_healthy_hp_41@0xe78a16 | `0xe78a16` | 실패 | 0xe7fb52 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | liner_roam_lane_healthy_hp_41@0xe78a79 | `0xe78a79` | 실패 | 0xe7fbb5 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | liner_roam_lane_healthy_hp_41@0xe78adc | `0xe78adc` | 실패 | 0xe7fc18 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0xe97bb6 | `0xe97bb6` | 0xea0ed6 | 0xea1048 | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0xe9cab7 | `0xe9cab7` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0xe9d6bb | `0xe9d6bb` | 0xfd661b | 0xfd6681 | DISAGREE_MNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xe9da3c | `0xe9da3c` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0xe9e280 | `0xe9e280` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0xea07c0 | `0xea07c0` | 0xea97a0 | 0xea97a7 | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0xea3682 | `0xea3682` | 0xeac662 | 0xeac665 | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0xea8c39 | `0xea8c39` | 0xeb1cf9 | 0xeb1dbe | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c057@0xea9386 | `0xea9386` | 0xeb2446 | 0xeb23d8 | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c057@0xea9562 | `0xea9562` | 0xeb2622 | 0xeb255c | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c057@0xea97fc | `0xea97fc` | 0xeb28bc | 0xeb2818 | DISAGREE_WMNEM |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xea989e | `0xea989e` | 0xeb0fcb | 0xeb28b9 | DISAGREE_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xeab41d | `0xeab41d` | 0xeb460d | 0xeb45bd | DISAGREE_EXACT |
| tfm2_detail_tactics | push_sustain_min_healthy_3@0xeadb7e | `0xeadb7e` | 0xeb6d6e | 0xeb6ef8 | DISAGREE_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xeadcd9 | `0xeadcd9` | 0xeb6ec9 | 0xeb7094 | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c002@0xeaf6e6 | `0xeaf6e6` | 0x104777b |  | NO_ALIGN |
| tfm2_detail_tactics | extra_c002@0xeaf6f2 | `0xeaf6f2` | 0x1047787 |  | NO_ALIGN |
| tfm2_detail_tactics | extra_c060@0xeb5a7f | `0xeb5a7f` | 실패 | 0xebe0a0 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c060@0xeb5aaa | `0xeb5aaa` | 실패 | 0xebe0cc | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c060@0xeb5abe | `0xeb5abe` | 실패 | 0xebe0e0 | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | extra_c060@0xeb5ae0 | `0xeb5ae0` | 실패 | 0xebe102 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c032@0xebb41c | `0xebb41c` | 0xfd2434 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0xebbf91 | `0xebbf91` | 0xfd2ebf |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0xebe640 | `0xebe640` | 0xecd610 | 0xecd61c | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c023@0xebfcc2 | `0xebfcc2` | 0x1044722 | 0x1044725 | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c023@0xebfccc | `0xebfccc` | 0x104472c | 0x104472f | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c023@0xebfcf6 | `0xebfcf6` | 0x1044756 | 0x1044759 | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c023@0xebfd00 | `0xebfd00` | 0x1044760 | 0x1044763 | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c023@0xebfd34 | `0xebfd34` | 0x1044794 | 0x1044797 | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c023@0xebfd3e | `0xebfd3e` | 0x104479e | 0x10447a1 | DISAGREE_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xebfd44 | `0xebfd44` | 0x10447a4 | 0x10447a7 | DISAGREE_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xebfd4d | `0xebfd4d` | 0x10447ad | 0x10447b0 | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c022@0xec0fb5 | `0xec0fb5` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0xec8a9c | `0xec8a9c` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0xec8ab6 | `0xec8ab6` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0xec8afe | `0xec8afe` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0xf2c30d | `0xf2c30d` | 실패 | 0xfbb36a | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | base_defense_min_enemies_6@0xf3af6c | `0xf3af6c` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | base_defense_min_enemies_6@0xf3b00c | `0xf3b00c` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | base_defense_min_enemies_6@0xf3b0cc | `0xf3b0cc` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xf3b0d2 | `0xf3b0d2` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xf3b0d6 | `0xf3b0d6` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c010@0xf4e7a0 | `0xf4e7a0` | 0xec80c0 | 0xec8282 | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_n001@0xf5086f | `0xf5086f` | 실패 | 0x10310b6 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0xf5c351 | `0xf5c351` | 실패 | 0x10c6800 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0xf5c39b | `0xf5c39b` | 실패 | 0x10c684a | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0xf5c3e1 | `0xf5c3e1` | 실패 | 0x10c6890 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | tower_chase_own_hp_35@0xf76445 | `0xf76445` | 0x10eebb5 | 0x10eeb38 | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0xf810ba | `0xf810ba` | 0x101b3aa |  | NO_ALIGN |
| tfm2_detail_tactics | CALLER_RETURN_0xf8627b | `0xf8627b` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c051@0xf8972e | `0xf8972e` | 0xf052fe |  | NO_ALIGN |
| tfm2_detail_tactics | extra_c051@0xf89cc2 | `0xf89cc2` | 0xf844b9 | 0xf0557c | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c051@0xf89e6c | `0xf89e6c` | 0xf844b9 | 0xf0570c | DISAGREE_EXACT |
| tfm2_detail_tactics | line_chase_retreat_hp_45@0xfa5f6d | `0xfa5f6d` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | line_chase_retreat_hp_45@0xfa5f82 | `0xfa5f82` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xfa5f8c | `0xfa5f8c` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xfa614a | `0xfa614a` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | line_chase_target_guard_hp_65@0xfa63e7 | `0xfa63e7` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | base_defense_hold_dmg_ratio_49@0xfa9000 | `0xfa9000` | 0x11c9ca0 | 0x11c9d56 | DISAGREE_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xfa9045 | `0xfa9045` | 0x11c9ce5 | 0x11c9da9 | DISAGREE_MNEM |
| tfm2_detail_tactics | base_defense_hold_nexus_hp_50@0xfa907e | `0xfa907e` | 0x11c9d1e | 0x11c9de2 | DISAGREE_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xfa9084 | `0xfa9084` | 0x11c9d24 | 0x11c9de8 | DISAGREE_LOOSE_GAP |
| tfm2_detail_tactics | base_defense_hold_nexus_hp_50@0xfa90a6 | `0xfa90a6` | 0x11c9d46 | 0x11c9e07 | DISAGREE_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xfa90ac | `0xfa90ac` | 0x11c9d4c | 0x11c9e0d | DISAGREE_EXACT |
| tfm2_detail_tactics | base_defense_hold_retreat_hp_45@0xfa90bc | `0xfa90bc` | 0x11c9d5c | 0x11c9e1d | DISAGREE_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xfa91aa | `0xfa91aa` | 0x11c9e4a | 0x11c9e2e | DISAGREE_MNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xfb5076 | `0xfb5076` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0xfbc342 | `0xfbc342` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | CALLER_RETURN_0xfbc7bf | `0xfbc7bf` | 0xfe505f | 0xfe5041 | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0xfbd1e0 | `0xfbd1e0` | 실패 | 0xfe5a59 | FAIL_ALIGNED_LOOSE_GAP |
| tfm2_detail_tactics | CALLER_RETURN_0xfbe45a | `0xfbe45a` | 0x1363708 | 0xfe6cd2 | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c053@0xfe0630 | `0xfe0630` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c035@0xfefa7d | `0xfefa7d` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c035@0xfeff6e | `0xfeff6e` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c034@0xff4a07 | `0xff4a07` | 0x10086f7 | 0x1008977 | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c035@0x10001fc | `0x10001fc` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c035@0x1001181 | `0x1001181` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0x101f9b0 | `0x101f9b0` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | support_minion_block_role_4@0x102b8ea | `0x102b8ea` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | RELOCATED_BRANCH_0x102bc0d | `0x102bc0d` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c054@0x103029b | `0x103029b` | 0x11b936b | 0x11b9281 | DISAGREE_MNEM |
| tfm2_detail_tactics | CALLER_RETURN_0x10547a5 | `0x10547a5` | 0xf37385 | 0xf3683a | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c054@0x1054cba | `0x1054cba` | 0xf3789a | 0xf36c5f | DISAGREE_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0x1056246 | `0x1056246` | 0xf38e26 | 0xf381ac | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_d002@0x105b725 | `0x105b725` | 0x11ce9a5 | 0x11cebf8 | DISAGREE_MNEM |
| tfm2_detail_tactics | CALLER_RETURN_0x106e460 | `0x106e460` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0x106e6ec | `0x106e6ec` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c019@0x106e99a | `0x106e99a` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c048@0x107ae1e | `0x107ae1e` | 0x116a94e | 0x116a9b6 | DISAGREE_WLOOSE |
| tfm2_detail_tactics | extra_c048@0x107afd2 | `0x107afd2` | 0x116ab02 | 0x116ab62 | DISAGREE_WLOOSE |
| tfm2_detail_tactics | extra_c050@0x107b792 | `0x107b792` | 0x116b2c2 | 0x116b337 | DISAGREE_WLOOSE_GAP |
| tfm2_detail_tactics | extra_c050@0x107b7b9 | `0x107b7b9` | 0x116b2e9 | 0x116b35e | DISAGREE_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0x107d8dd | `0x107d8dd` | 0x116d40d | 0x116d5cb | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c050@0x107e1ce | `0x107e1ce` | 0x116dcfe |  | NO_ALIGN |
| tfm2_detail_tactics | CALLER_RETURN_0x108278f | `0x108278f` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c003@0x10827b5 | `0x10827b5` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c020@0x1084e1a | `0x1084e1a` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c020@0x1084e2c | `0x1084e2c` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c021@0x1085466 | `0x1085466` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_n005@0x1085c53 | `0x1085c53` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | combat_retreat_hp_35@0x1086962 | `0x1086962` | 0x11491a2 | 0x114924c | DISAGREE_EXACT |
| tfm2_detail_tactics | combat_retreat_hp_35@0x1087596 | `0x1087596` | 0x1149dd6 |  | NO_ALIGN |
| tfm2_detail_tactics | RELOCATED_BRANCH_0x10875d3 | `0x10875d3` | 0x1149e13 |  | NO_ALIGN |
| tfm2_detail_tactics | extra_c020@0x1089788 | `0x1089788` | 실패 | 0x114bccd | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c020@0x108979a | `0x108979a` | 실패 | 0x114bcdf | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c013@0x108af43 | `0x108af43` | 실패 | 0x114d45e | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c013@0x108affa | `0x108affa` | 실패 | 0x114d50b | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c013@0x108b0ec | `0x108b0ec` | 실패 | 0x114d604 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c013@0x108b21d | `0x108b21d` | 실패 | 0x114d7b5 | FAIL_ALIGNED_MNEM |
| tfm2_detail_tactics | self_risk_ignore_hp_35@0x10940dc | `0x10940dc` | 실패 | 0x11567db | FAIL_ALIGNED_LOOSE |
| tfm2_detail_tactics | extra_c049@0x1094dc8 | `0x1094dc8` | 0x11aaecf | 0x1156e2f | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c052@0x10959df | `0x10959df` | 실패 | 0x1157891 | FAIL_ALIGNED_LOOSE_GAP |
| tfm2_detail_tactics | extra_c028@0x10a8603 | `0x10a8603` | 0xed0283 | 0xed02c6 | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c028@0x10a860c | `0x10a860c` | 0xed02cc | 0xed02cf | DISAGREE_MNEM |
| tfm2_detail_tactics | RELOCATED_BRANCH_0x10a861e | `0x10a861e` | 0xed02de | 0xed02e4 | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c053@0x10b17cb | `0x10b17cb` | 실패 | 0x11a9ff3 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_n010@0x10b2b61 | `0x10b2b61` | 실패 | 0x11ab397 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_n010@0x10b2b6e | `0x10b2b6e` | 실패 | 0x11ab3ab | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c025@0x10cfcd5 | `0x10cfcd5` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0x10cfda9 | `0x10cfda9` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | jg_gank_hp_floor_40@0x10cfdd6 | `0x10cfdd6` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | RELOCATED_BRANCH_0x10cfddc | `0x10cfddc` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | jg_gank_hp_floor_40@0x10cfdfb | `0x10cfdfb` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | RELOCATED_BRANCH_0x10cfe01 | `0x10cfe01` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | jg_gank_hp_start_50@0x10cff15 | `0x10cff15` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | RELOCATED_BRANCH_0x10d0192 | `0x10d0192` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | jg_gank_hp_start_50@0x10d084e | `0x10d084e` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c026@0x10d0959 | `0x10d0959` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c005@0x10d0a7b | `0x10d0a7b` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_d001@0x10d1337 | `0x10d1337` | 0xf6ede7 | 0xf6edfa | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_d001@0x10d13e6 | `0x10d13e6` | 0xf6ee96 | 0xf6eea9 | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0x10d1acd | `0x10d1acd` | 0xf6f58d | 0xf6f59b | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c045@0x10d2e65 | `0x10d2e65` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0x10d30be | `0x10d30be` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c045@0x10d33fd | `0x10d33fd` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c045@0x10d36bf | `0x10d36bf` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0x10d3c22 | `0x10d3c22` | 0xf716f2 |  | NO_ALIGN |
| tfm2_detail_tactics | CALLER_RETURN_0x10d3d4c | `0x10d3d4c` | 0xf7181c | 0xf7181f | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0x10d4a3a | `0x10d4a3a` | 0xf7250a | 0xf7256a | DISAGREE_MNEM |
| tfm2_detail_tactics | CALLER_RETURN_0x10e4527 | `0x10e4527` | 0xf85bfa |  | FAIL_OWNER |
| tfm2_detail_tactics | lane_retreat_final_50@0x10efcdd | `0x10efcdd` | 0x10b959d | 0x10b949f | DISAGREE_MNEM |
| tfm2_detail_tactics | lane_retreat_final_50@0x10f25ba | `0x10f25ba` | 0x10bbd7a | 0x10bbc62 | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c041@0x110836f | `0x110836f` | 0x112bfcf | 0x112c3eb | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c040@0x1108ff1 | `0x1108ff1` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | extra_c040@0x1109343 | `0x1109343` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | extra_c040@0x110962c | `0x110962c` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | CALLER_RETURN_0x110de11 | `0x110de11` | 실패 | 0x11223f2 | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0x116225b | `0x116225b` | 0x1074b19 |  | FAIL_OWNER |
