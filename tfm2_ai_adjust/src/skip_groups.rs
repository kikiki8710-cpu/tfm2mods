// ★[08-07 자동생성 — v54/fix_class_gate.py] skip_untuned 판정 그룹(g(&[...]))에 등장하는 노브 전부.
//   클래스 오버라이드가 이 목록 안의 노브에 걸리면 "그 판단만" 재구현을 유지하면 된다.
//   목록 밖이면 어느 판단이 그 값을 읽는지 알 수 없으므로 보수적으로 전체 skip 을 해제한다.
pub static SKIP_GROUP_KEYS: [&str; 136] = [
    "bt_home_hi", "bt_home_lo", "bt_home_x1", "bt_home_y1", "bt_hp_retreat", "d13_engage_hp_pct",
    "d15_engage_hp_pct", "d4_close_hp", "d4_coef_clamp", "d4_coef_min", "d4_coef_scale", "d4_coord_dist",
    "d4_div_base", "d4_dmg_scale", "d4_engage_r2", "d4_pathlen_thr", "d4_ref_dist2", "d4_threat_min",
    "d4_ttd_scale", "d4_ward_dist2", "d4_wcast_thr", "d8_slot_thr", "dd_cover_count", "dd_cover_p3_thr",
    "dd_early_p3_thr", "dd_f22e80_margin", "dd_facet_thr", "dd_frontier_mult", "dd_gatee_dist", "dd_ivar2_thr",
    "dd_lane_margin", "dd_main_near_dist", "dd_n_thr", "dd_near_dist", "dd_ratio_thr", "dd_survivor_thr",
    "disc16_home_hp", "ec_commit_hp", "ec_count_hp", "ec_count_radius", "ec_engage_dist2", "ec_gate_tick",
    "ec_iz_hp", "ec_oz_hp", "ec_self_hp_low", "ec_valid_hp", "ec_vision_ticks", "eng_role2",
    "eng_role3", "eng_role4", "eng_role_def", "engage_base", "engage_thr_mult", "gb_cnt_move",
    "gb_cnt_skip", "gb_da_thr", "gb_db_engage", "gb_r14_num", "gb_r15_div", "gb_rbx_div",
    "gb_score_mult", "nxd_near_dist", "nxd_p3_gate", "nxd_pred_dist", "nxd_prog_crit", "nxd_prog_low",
    "nxd_ref_hp", "pf_band_width", "pf_center_band", "pf_diag_far", "pf_diag_near", "pf_edge_margin",
    "pk_engage_dist", "pk_home_hi", "pk_home_lo", "pk_home_x1", "pk_home_y1", "pk_hp_main",
    "pk_hp_retreat", "pk_obj_hp", "pk_smallact_split", "pk_threat_mult", "pk_zone_hp", "poke_active_min",
    "poke_phase_gate", "poke_reach_bonus", "poke_serpen_slot", "rc_ad_far_pen", "rc_ad_mid", "rc_ad_near",
    "rc_ad_near_bonus", "rc_ahp2_pen", "rc_ahp_t1", "rc_ahp_t2", "rc_ally_hp_min", "rc_ed_far",
    "rc_ed_far_bonus", "rc_ed_mid", "rc_ed_near", "rc_ed_near_pen", "rc_ed_vfar_bonus", "rc_ehp_t1",
    "rc_ehp_t2", "rc_ehp_t3", "rc_ehp_v1", "rc_ehp_v2", "rc_join_adv", "rc_join_dmid",
    "rc_join_dnear", "rc_join_obj_mult", "rc_join_rescue", "rc_join_weight", "rc_mult_bonus", "rc_norp_bonus",
    "rc_rng_a_base", "rc_rng_center", "rc_rng_spread_div", "rc_score_div", "rc_u13_bonus", "rc_u21_init",
    "sn_home_hi", "sn_home_lo", "sn_home_x1", "sn_home_y1", "sn_hp_crit", "sn_self_hp",
    "stat_judg_ref", "stat_neutral", "stat_noise_shift", "stat_pos_div", "t_engage", "t_gb",
    "t_recall", "t_ttd", "tower_dps", "vis_window",
];
