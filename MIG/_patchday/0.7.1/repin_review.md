# 재핀 사람 판정 대기열 — 0.7.1 (구 0.7.0 → 신 0.7.1)

> run_patchday 생성. 판정 결과는 `MIG\_mkfinal071.py`(= `_mkfinal065.py` 복제) 의 FIX 표로 → `MIG\repin_map_071.final.json`.
> 근거 원문은 REPORT `<MOD>\RE\YYYY-MM-DD_0.7.1-재핀판정.md` 로 저장(§11).

## ★repin apply 대상 — 사람 판정 필요 22

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

## 동형 클론 주의 11

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

## (참고) 생성물 모드 147 — repin 판정 불필요

> tfm2_detail_tactics·tfm2_tactics_verify 는 sites.rs 생성물이라 `repin apply` 대상이 아니다. 상세전술 = `detail_tactics\track.md`(README ③), tactics_verify = `tools\migrate_targets.py`(README ④) 가 따로 판정한다. 이 표는 교차 확인용.

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
| tfm2_detail_tactics | extra_c006@0xe67037 | `0xe67037` | 0xe6a420 |  | NO_ALIGN |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xe6782d | `0xe6782d` | 0xe6b41d | 0xe6b421 | DISAGREE_LOOSE_GAP |
| tfm2_detail_tactics | liner_roam_lane_healthy_hp_41@0xe74299 | `0xe74299` | 0xe78a79 | 0xe78adc | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_n012@0xe90433 | `0xe90433` | 0x106efb3 | 0x106ef93 | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c007@0xeaf05c | `0xeaf05c` | 0xe9466c | 0xe946b2 | DISAGREE_MNEM |
| tfm2_detail_tactics | extra_c007@0xeaf150 | `0xeaf150` | 0xe94760 | 0xe947b0 | DISAGREE_MNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xeb204a | `0xeb204a` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0xeb65cf | `0xeb65cf` | 0xe9c9ef | 0xe9cab7 | DISAGREE_MNEM |
| tfm2_detail_tactics | CALLER_RETURN_0xebcc19 | `0xebcc19` | 실패 | 0xea3682 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0xebfbdb | `0xebfbdb` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | push_sustain_margin_2@0xec1cfd | `0xec1cfd` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | push_sustain_min_healthy_3@0xec1d37 | `0xec1d37` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xec1ea9 | `0xec1ea9` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0xec6f34 | `0xec6f34` | 실패 | 0xea8c39 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c057@0xec74a5 | `0xec74a5` | 실패 | 0xea9386 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c057@0xec7650 | `0xec7650` | 실패 | 0xea9562 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xec76d0 | `0xec76d0` | 실패 | 0xea9612 | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | extra_c057@0xec78d0 | `0xec78d0` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xec7945 | `0xec7945` | 실패 | 0xea989e | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | extra_c060@0xecc174 | `0xecc174` | 실패 | 0xeb5aaa | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c060@0xecc188 | `0xecc188` | 실패 | 0xeb5abe | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | extra_c060@0xecc1aa | `0xecc1aa` | 실패 | 0xeb5ae0 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c032@0xed16d0 | `0xed16d0` | 0xebb410 | 0xebb41c | DISAGREE_LOOSE_GAP |
| tfm2_detail_tactics | CALLER_RETURN_0xed2418 | `0xed2418` | 실패 | 0xebbf91 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0xed4aa6 | `0xed4aa6` | 실패 | 0xebe640 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0xf1590c | `0xf1590c` | 실패 | 0xec8a9c | FAIL_ALIGNED_LOOSE_GAP |
| tfm2_detail_tactics | extra_n008@0xf56025 | `0xf56025` | 실패 | 0x100b5d5 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c027@0xf567f1 | `0xf567f1` | 실패 | 0x100bda3 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xf788a7 | `0xf788a7` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0xf78e7d | `0xf78e7d` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c003@0xf78eb4 | `0xf78eb4` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0xf7e6ce | `0xf7e6ce` | 실패 | 0x110de11 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c038@0xf819bb | `0xf819bb` | 실패 | 0x1111ce7 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c039@0xf821f4 | `0xf821f4` | 실패 | 0x11134f4 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c041@0xf82292 | `0xf82292` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | extra_c040@0xf83af2 | `0xf83af2` | 0x1108e59 | 0x1108ff1 | DISAGREE_LOOSE |
| tfm2_detail_tactics | extra_c040@0xf83d63 | `0xf83d63` | 0x1108e59 | 0x1109343 | DISAGREE_LOOSE |
| tfm2_detail_tactics | extra_c040@0xf83f5e | `0xf83f5e` | 0x1108e59 | 0x110962c | DISAGREE_LOOSE |
| tfm2_detail_tactics | base_defense_hold_dmg_ratio_49@0xfe3353 | `0xfe3353` | 실패 | 0xfa9000 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xfe3399 | `0xfe3399` | 실패 | 0xfa9045 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xfe33d8 | `0xfe33d8` | 실패 | 0xfa9084 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | base_defense_hold_nexus_hp_50@0xfe33fa | `0xfe33fa` | 실패 | 0xfa90a6 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xfe3400 | `0xfe3400` | 실패 | 0xfa90ac | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | base_defense_hold_retreat_hp_45@0xfe3410 | `0xfe3410` | 실패 | 0xfa90bc | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xfe34fe | `0xfe34fe` | 실패 | 0xfa91aa | FAIL_ALIGNED_MNEM |
| tfm2_detail_tactics | extra_c037@0xff61be | `0xff61be` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c037@0xff61c3 | `0xff61c3` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | RELOCATED_BRANCH_0xff61d1 | `0xff61d1` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c048@0x101519e | `0x101519e` | 실패 | 0x107ae1e | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c048@0x10152c4 | `0x10152c4` | 실패 | 0x107afd2 | FAIL_ALIGNED_WLOOSE |
| tfm2_detail_tactics | extra_c048@0x1015310 | `0x1015310` | 실패 | 0x107afff | FAIL_ALIGNED_WLOOSE |
| tfm2_detail_tactics | RELOCATED_BRANCH_0x10153d2 | `0x10153d2` | 실패 | 0x107b12b | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | extra_c050@0x101705c | `0x101705c` | 실패 | 0x107d10b | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | combat_avoid_hp_41@0x1017856 | `0x1017856` | 실패 | 0x107d8ca | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | combat_avoid_count_margin_2@0x1017860 | `0x1017860` | 실패 | 0x107d8d0 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c050@0x1019551 | `0x1019551` | 0x107b6a8 |  | NO_ALIGN |
| tfm2_detail_tactics | extra_c050@0x1019569 | `0x1019569` | 0x107b6c0 |  | NO_ALIGN |
| tfm2_detail_tactics | extra_c050@0x1019571 | `0x1019571` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | extra_c050@0x1019634 | `0x1019634` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | extra_c050@0x101965b | `0x101965b` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | CALLER_RETURN_0x101d076 | `0x101d076` | 실패 | 0xf8627b | FAIL_ALIGNED_LOOSE_GAP |
| tfm2_detail_tactics | CALLER_RETURN_0x102151c | `0x102151c` | 실패 | 0xf8265f | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c051@0x1022f1c | `0x1022f1c` | 0xf8972e | 0xf89e6c | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c022@0x103c0b9 | `0x103c0b9` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c002@0x103e65c | `0x103e65c` | 0xeaf6e6 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c002@0x103e668 | `0x103e668` | 0xeaf6f2 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c053@0x1044cf9 | `0x1044cf9` | 실패 | 0x10b17cb | FAIL_ALIGNED_MNEM |
| tfm2_detail_tactics | extra_n009@0x1045604 | `0x1045604` | 실패 |  | FAIL_NO_ALIGN |
| tfm2_detail_tactics | RELOCATED_BRANCH_0x104613e | `0x104613e` | 0x10b2a60 |  | NO_ALIGN |
| tfm2_detail_tactics | extra_n010@0x10463e1 | `0x10463e1` | 실패 | 0x10b2b61 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_n010@0x10463f1 | `0x10463f1` | 실패 | 0x10b2b6e | FAIL_ALIGNED_MNEM |
| tfm2_detail_tactics | CALLER_RETURN_0x104baf3 | `0x104baf3` | 0xf80b03 | 0xf8265f | DISAGREE_EXACT |
| tfm2_detail_tactics | combat_retreat_hp_35@0x1056830 | `0x1056830` | 0x10878b0 |  | NO_ALIGN |
| tfm2_detail_tactics | RELOCATED_BRANCH_0x1056871 | `0x1056871` | 0x10878f1 |  | NO_ALIGN |
| tfm2_detail_tactics | extra_c013@0x1058955 | `0x1058955` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c013@0x10589f3 | `0x10589f3` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c013@0x1058e59 | `0x1058e59` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c013@0x1058f20 | `0x1058f20` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c013@0x10590af | `0x10590af` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c013@0x1059160 | `0x1059160` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | self_risk_ignore_hp_35@0x105ed2f | `0x105ed2f` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c049@0x105f9bd | `0x105f9bd` | 0x10988c5 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c052@0x106058e | `0x106058e` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c020@0x1061197 | `0x1061197` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c020@0x10611b7 | `0x10611b7` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c020@0x1061763 | `0x1061763` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c020@0x1061775 | `0x1061775` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_c021@0x1061eae | `0x1061eae` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_n005@0x10624d7 | `0x10624d7` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | extra_n005@0x1062a64 | `0x1062a64` | 실패 |  | FAIL_OWNER |
| tfm2_detail_tactics | CALLER_RETURN_0x107cffc | `0x107cffc` | 실패 | 0x10547a5 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c054@0x107d52e | `0x107d52e` | 실패 | 0x1054cba | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0x107edd4 | `0x107edd4` | 실패 | 0x1056246 | FAIL_ALIGNED_WMNEM |
| tfm2_detail_tactics | CALLER_RETURN_0x10800c0 | `0x10800c0` | 실패 | 0x105791f | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | jg_gank_hp_floor_40@0x10cd576 | `0x10cd576` | 실패 | 0x10cfdd6 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0x10cd57c | `0x10cd57c` | 실패 | 0x10cfddc | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | jg_gank_hp_floor_40@0x10cd5a2 | `0x10cd5a2` | 실패 | 0x10cfdfb | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0x10cd5a8 | `0x10cd5a8` | 실패 | 0x10cfe01 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | jg_gank_hp_start_50@0x10cd6c5 | `0x10cd6c5` | 실패 | 0x10cff15 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | RELOCATED_BRANCH_0x10cd945 | `0x10cd945` | 실패 | 0x10d0192 | FAIL_ALIGNED_EXACT |
| tfm2_detail_tactics | extra_c045@0x10d0625 | `0x10d0625` | 0x10d2e55 | 0x10d2e65 | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0x10d087b | `0x10d087b` | 0x10d30ab | 0x10d30be | DISAGREE_EXACT |
| tfm2_detail_tactics | extra_c045@0x10d0b8e | `0x10d0b8e` | 0x10d33be | 0x10d33fd | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0x10d13a7 | `0x10d13a7` | 0x10d3c17 | 0x10d3c22 | DISAGREE_MNEM |
| tfm2_detail_tactics | CALLER_RETURN_0x10d151a | `0x10d151a` | 0x10d3d8a | 0x10d3d4c | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0x10d1d84 | `0x10d1d84` | 0x10d45f4 | 0x10d456d | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0x10d2073 | `0x10d2073` | 0x10d48e3 | 0x10d4884 | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0x10d22dc | `0x10d22dc` | 0x10d4b4c | 0x10d4a3a | DISAGREE_MNEM |
| tfm2_detail_tactics | CALLER_RETURN_0x10de732 | `0x10de732` | 0x106e482 | 0x106e460 | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0x10e4188 | `0x10e4188` | 0x10e4527 |  | FAIL_OWNER |
| tfm2_detail_tactics | RELOCATED_BRANCH_0x10e4bcc | `0x10e4bcc` | 0x10e4f6c | 0x10e4f6e | DISAGREE_EXACT |
| tfm2_detail_tactics | jungle_stop_hp_41@0x10e4c71 | `0x10e4c71` | 0x10e5011 | 0x10e5015 | DISAGREE_EXACT |
| tfm2_detail_tactics | jungle_stop_hp_41@0x10e4c7d | `0x10e4c7d` | 0x10e501d | 0x10e5021 | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0x10ff072 | `0x10ff072` | 실패 | 0xfb5076 | FAIL_ALIGNED_MNEM |
| tfm2_detail_tactics | lane_retreat_final_50@0x1100e9e | `0x1100e9e` | 0x10f25ce |  | NO_ALIGN |
| tfm2_detail_tactics | CALLER_RETURN_0x11012f2 | `0x11012f2` | 0x10f2aa2 | 0x10f2a9c | DISAGREE_EXACT |
| tfm2_detail_tactics | CALLER_RETURN_0x110d8f1 | `0x110d8f1` | 실패 | 0xfbc342 | FAIL_ALIGNED_MNEM |
