// ★★[08-04 2차] 라인 총력전(`0xc57580` line_total.rs) — 노브가 0개였던 핸들러
//   근거 = RE6-08-04_실행층4핸들러-line_total-hide-nexus-노브라벨오류-0.5.3.md
//   전 사이트 정적 검증 통과(`gen_wave2.py` 205/205). 기본 -1 = 원본 유지.
unsafe fn apply_lt_imm() {
    let lt_ally_join = tune("lt_ally_join", -1);   // 아군에게 붙기 시작하는 거리(원본 50000). ↑=잘 안 모임
    let lt_around_radius = tune("lt_around_radius", -1);   // 라인 총력전 배회 반경(원본 80000, 3곳)
    let lt_phase_mask = tune("lt_phase_mask", -1);   // 발화 페이즈 비트마스크(원본 0x1a1)
    let mut sig = 0u64;
    for v in [lt_ally_join, lt_around_radius, lt_phase_mask] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == LT_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32; }}; }
    let v_lt_ally_join: u64 = { if lt_ally_join < 0 { 9765625u64 } else { let x = lt_ally_join.max(0) as u64; x.wrapping_mul(x) >> 8 } };
    p!(base + 0xc57dcf, &[0x49,0x81,0xf8], 3, 4, v_lt_ally_join);
    let v_lt_around_radius: u64 = { if lt_around_radius < 0 { 80000u64 } else { lt_around_radius.max(0) as u64 } };
    p!(base + 0xc57ed3, &[0x48,0xc7,0x85,0xf0,0x00,0x00,0x00], 7, 4, v_lt_around_radius);
    p!(base + 0xc5816c, &[0x48,0xc7,0x85,0xf0,0x00,0x00,0x00], 7, 4, v_lt_around_radius);
    p!(base + 0xc58296, &[0x48,0xc7,0x85,0xf0,0x00,0x00,0x00], 7, 4, v_lt_around_radius);
    let v_lt_phase_mask: u64 = { if lt_phase_mask < 0 { 417u64 } else { lt_phase_mask.max(0) as u64 } };
    p!(base + 0xc5763d, &[0xba], 1, 4, v_lt_phase_mask);
    LT_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("lt_imm.txt") {
        let _ = fs::write(pp, format!("applied={}/{} lt_ally_join={} lt_around_radius={} lt_phase_mask={} @base{:#x}\n",
            ok, tot, lt_ally_join, lt_around_radius, lt_phase_mask, base));
    }
}

// ★★[08-04 2차] 넥서스 공격/방어(disc18·19)의 미배선 사이트 — disc18에만 걸려 있던 교전 컷을 disc19에도
//   근거 = RE6-08-04_실행층4핸들러-line_total-hide-nexus-노브라벨오류-0.5.3.md
//   전 사이트 정적 검증 통과(`gen_wave2.py` 205/205). 기본 -1 = 원본 유지.
unsafe fn apply_nx_imm() {
    let nx_cull_dist19 = tune("nx_cull_dist19", -1);   // 넥서스 방어 교전후보 컷 거리(원본 80000, 3곳). disc18 쌍둥이인데 미배선이었다
    let nx_around_atk = tune("nx_around_atk", -1);   // 넥서스 공격 시 배회 반경(원본 80000)
    let nx_around_def = tune("nx_around_def", -1);   // 넥서스 방어 시 배회 반경(원본 80000, 4곳)
    let mut sig = 0u64;
    for v in [nx_cull_dist19, nx_around_atk, nx_around_def] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == NX_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32; }}; }
    let v_nx_cull_dist19: u64 = { if nx_cull_dist19 < 0 { 390625u64 } else { let x = nx_cull_dist19.max(0) as u64; x.wrapping_mul(x) >> 14 } };
    p!(base + 0xdee222, &[0x49,0x81,0xf8], 3, 4, v_nx_cull_dist19);
    p!(base + 0xdee2b1, &[0x49,0x81,0xf8], 3, 4, v_nx_cull_dist19);
    p!(base + 0xdee335, &[0x49,0x81,0xf8], 3, 4, v_nx_cull_dist19);
    let v_nx_around_atk: u64 = { if nx_around_atk < 0 { 80000u64 } else { nx_around_atk.max(0) as u64 } };
    p!(base + 0xd95316, &[0x48,0xc7,0x85,0xa8,0x00,0x00,0x00], 7, 4, v_nx_around_atk);
    let v_nx_around_def: u64 = { if nx_around_def < 0 { 80000u64 } else { nx_around_def.max(0) as u64 } };
    p!(base + 0xdedabf, &[0x48,0xc7,0x85,0x08,0x01,0x00,0x00], 7, 4, v_nx_around_def);
    p!(base + 0xdedd0a, &[0x48,0xc7,0x85,0x08,0x01,0x00,0x00], 7, 4, v_nx_around_def);
    p!(base + 0xdeddf7, &[0x48,0xc7,0x85,0x08,0x01,0x00,0x00], 7, 4, v_nx_around_def);
    p!(base + 0xdede9f, &[0x48,0xc7,0x85,0x08,0x01,0x00,0x00], 7, 4, v_nx_around_def);
    NX_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("nx_imm.txt") {
        let _ = fs::write(pp, format!("applied={}/{} nx_cull_dist19={} nx_around_atk={} nx_around_def={} @base{:#x}\n",
            ok, tot, nx_cull_dist19, nx_around_atk, nx_around_def, base));
    }
}

// ★★[08-04 2차] 숨기(`0xca43c0` hide.rs) — 노브가 0개였던 핸들러. 후보 선별 30사이트는 prefix 4종
//   근거 = RE6-08-04_실행층4핸들러-line_total-hide-nexus-노브라벨오류-0.5.3.md
//   전 사이트 정적 검증 통과(`gen_wave2.py` 205/205). 기본 -1 = 원본 유지.
unsafe fn apply_hd_imm() {
    let hd_bush_near = tune("hd_bush_near", -1);   // 이미 이만큼 가까우면 부시로 안 움직임(원본 100000, 2곳)
    let hd_path_radius = tune("hd_path_radius", -1);   // 부시 경로탐색 반경(원본 60000, 2곳)
    let hd_around_radius = tune("hd_around_radius", -1);   // 숨은 뒤 배회 반경(원본 80000, 2곳)
    let hd_detect_max = tune("hd_detect_max", -1);   // 적 후보 최대 탐지거리(원본 250000)
    let hd_fight_cut = tune("hd_fight_cut", -1);   // 교전 후보 컷 거리(원본 150000)
    let hd_cand_select = tune("hd_cand_select", -1);   // 부시·후퇴지점 후보 선별 거리(원본 150000, 30곳)
    let hd_trace_leash = tune("hd_trace_leash", -1);   // 추적 시 붙는 거리(원본 15000)
    let hd_vision_mem = tune("hd_vision_mem", -1);   // 적 목격 정보 유효 틱(원본 120)
    let mut sig = 0u64;
    for v in [hd_bush_near, hd_path_radius, hd_around_radius, hd_detect_max, hd_fight_cut, hd_cand_select, hd_trace_leash, hd_vision_mem] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == HD_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32; }}; }
    let v_hd_bush_near: u64 = { if hd_bush_near < 0 { 10000000000u64 } else { let x = hd_bush_near.max(0) as u64; x.wrapping_mul(x) } };
    p!(base + 0xca46b7, &[0x48,0xb8], 2, 8, v_hd_bush_near);
    p!(base + 0xca4812, &[0x48,0xb8], 2, 8, v_hd_bush_near);
    let v_hd_path_radius: u64 = { if hd_path_radius < 0 { 60000u64 } else { hd_path_radius.max(0) as u64 } };
    p!(base + 0xca46e3, &[0x48,0xc7,0x44,0x24,0x20], 5, 4, v_hd_path_radius);
    p!(base + 0xca483e, &[0x48,0xc7,0x44,0x24,0x20], 5, 4, v_hd_path_radius);
    let v_hd_around_radius: u64 = { if hd_around_radius < 0 { 80000u64 } else { hd_around_radius.max(0) as u64 } };
    p!(base + 0xca471f, &[0x48,0xc7,0x45,0x08], 4, 4, v_hd_around_radius);
    p!(base + 0xca487a, &[0x48,0xc7,0x45,0x08], 4, 4, v_hd_around_radius);
    let v_hd_detect_max: u64 = { if hd_detect_max < 0 { 62500000001u64 } else { let x = hd_detect_max.max(0) as u64; x.wrapping_mul(x).wrapping_add(1) } };
    p!(base + 0xca4ae1, &[0x48,0xba], 2, 8, v_hd_detect_max);
    let v_hd_fight_cut: u64 = { if hd_fight_cut < 0 { 22500000000u64 } else { let x = hd_fight_cut.max(0) as u64; x.wrapping_mul(x) } };
    p!(base + 0xca50fa, &[0x48,0xb8], 2, 8, v_hd_fight_cut);
    let v_hd_cand_select: u64 = { if hd_cand_select < 0 { 22500000001u64 } else { let x = hd_cand_select.max(0) as u64; x.wrapping_mul(x).wrapping_add(1) } };
    p!(base + 0xca598f, &[0x49,0xb8], 2, 8, v_hd_cand_select);
    p!(base + 0xca59f2, &[0x49,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca5a59, &[0x49,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca5ac0, &[0x49,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca5b26, &[0x49,0xb8], 2, 8, v_hd_cand_select);
    p!(base + 0xca5b8f, &[0x48,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca6080, &[0x49,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca60e2, &[0x49,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca6147, &[0x49,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca61ac, &[0x49,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca6211, &[0x49,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca6275, &[0x48,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca62c9, &[0x48,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca631c, &[0x48,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca636f, &[0x48,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca63c0, &[0x48,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca64a7, &[0x49,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca650c, &[0x49,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca6576, &[0x49,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca65e0, &[0x49,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca66ce, &[0x48,0xba], 2, 8, v_hd_cand_select);
    p!(base + 0xca68a0, &[0x48,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca68f3, &[0x48,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca6946, &[0x48,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca6997, &[0x48,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca6a1e, &[0x49,0xb8], 2, 8, v_hd_cand_select);
    p!(base + 0xca6a85, &[0x49,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca6aef, &[0x49,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca6b59, &[0x49,0xb9], 2, 8, v_hd_cand_select);
    p!(base + 0xca6bbc, &[0x48,0xba], 2, 8, v_hd_cand_select);
    let v_hd_trace_leash: u64 = { if hd_trace_leash < 0 { 15000u64 } else { hd_trace_leash.max(0) as u64 } };
    p!(base + 0xca6df5, &[0x48,0xc7,0x45,0x18], 4, 4, v_hd_trace_leash);
    let v_hd_vision_mem: u64 = { if hd_vision_mem < 0 { 120u64 } else { hd_vision_mem.max(0) as u64 } };
    p!(base + 0xca4b65, &[0x49,0x83,0xc7], 3, 1, v_hd_vision_mem);
    HD_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("hd_imm.txt") {
        let _ = fs::write(pp, format!("applied={}/{} hd_bush_near={} hd_path_radius={} hd_around_radius={} hd_detect_max={} hd_fight_cut={} hd_cand_select={} hd_trace_leash={} hd_vision_mem={} @base{:#x}\n",
            ok, tot, hd_bush_near, hd_path_radius, hd_around_radius, hd_detect_max, hd_fight_cut, hd_cand_select, hd_trace_leash, hd_vision_mem, base));
    }
}

// ★★[08-04 2차] plan 4 매퍼(`0xd71630` single_line.rs) — 레인 인덱스 2 = 바텀(봇 듀오 특수 블록)
//   근거 = RE6-08-04_Blackboard전필드-층2매퍼-소함수5종-0.5.3.md
//   전 사이트 정적 검증 통과(`gen_wave2.py` 205/205). 기본 -1 = 원본 유지.
unsafe fn apply_d4_imm() {
    let d4_ally_radius_a = tune("d4_ally_radius_a", -1);   // 아군 인정 반경(원본 150000, 5곳)
    let d4_ally_radius_b = tune("d4_ally_radius_b", -1);   // 아군 인정 반경(원본 150000, 10곳)
    let d4_early_leave = tune("d4_early_leave", -1);   // 조기 이탈 거리(원본 170000)
    let d4_partner_dist = tune("d4_partner_dist", -1);   // 봇 듀오 파트너 인지 반경(원본 200000)
    let d4_hp_safe = tune("d4_hp_safe", -1);   // HP% 이 값 미만이면 안전/귀환 쪽(원본 51)
    let d4_from_mid = tune("d4_from_mid", -1);   // 미드 기준 거리 하한 — 넘으면 귀환 허용(원본 1000)
    let d4_from_mid_mode = tune("d4_from_mid_mode", -1);   // 미드 기준 거리 상한 — 이하면 mode 1(원본 2001)
    let d4_ally_cnt = tune("d4_ally_cnt", -1);   // 아군 수 게이트(원본 3, 2곳)
    let d4_minion_cnt = tune("d4_minion_cnt", -1);   // 미니언 수 게이트(원본 2)
    let d4_gather_radius = tune("d4_gather_radius", -1);   // 주변 수집 반경(원본 150000)
    let mut sig = 0u64;
    for v in [d4_ally_radius_a, d4_ally_radius_b, d4_early_leave, d4_partner_dist, d4_hp_safe, d4_from_mid, d4_from_mid_mode, d4_ally_cnt, d4_minion_cnt, d4_gather_radius] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == D4_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32; }}; }
    let v_d4_ally_radius_a: u64 = { if d4_ally_radius_a < 0 { 87890624u64 } else { let x = d4_ally_radius_a.max(0) as u64; (x.wrapping_mul(x) >> 8).wrapping_sub(1) } };
    p!(base + 0xd71cd3, &[0x49,0x81,0xf9], 3, 4, v_d4_ally_radius_a);
    p!(base + 0xd71d98, &[0x49,0x81,0xf9], 3, 4, v_d4_ally_radius_a);
    p!(base + 0xd71e58, &[0x49,0x81,0xf9], 3, 4, v_d4_ally_radius_a);
    p!(base + 0xd71f18, &[0x49,0x81,0xf9], 3, 4, v_d4_ally_radius_a);
    p!(base + 0xd71fdb, &[0x49,0x81,0xf9], 3, 4, v_d4_ally_radius_a);
    let v_d4_ally_radius_b: u64 = { if d4_ally_radius_b < 0 { 87890625u64 } else { let x = d4_ally_radius_b.max(0) as u64; x.wrapping_mul(x) >> 8 } };
    p!(base + 0xd72296, &[0x49,0x81,0xfc], 3, 4, v_d4_ally_radius_b);
    p!(base + 0xd722d0, &[0x49,0x81,0xfb], 3, 4, v_d4_ally_radius_b);
    p!(base + 0xd72332, &[0x49,0x81,0xfd], 3, 4, v_d4_ally_radius_b);
    p!(base + 0xd7236d, &[0x49,0x81,0xfd], 3, 4, v_d4_ally_radius_b);
    p!(base + 0xd723ce, &[0x49,0x81,0xfd], 3, 4, v_d4_ally_radius_b);
    p!(base + 0xd72409, &[0x49,0x81,0xfd], 3, 4, v_d4_ally_radius_b);
    p!(base + 0xd7246a, &[0x49,0x81,0xfd], 3, 4, v_d4_ally_radius_b);
    p!(base + 0xd724a5, &[0x49,0x81,0xfd], 3, 4, v_d4_ally_radius_b);
    p!(base + 0xd72500, &[0x48,0x81,0xfa], 3, 4, v_d4_ally_radius_b);
    p!(base + 0xd72542, &[0x48,0x3d], 2, 4, v_d4_ally_radius_b);
    let v_d4_early_leave: u64 = { if d4_early_leave < 0 { 112890625u64 } else { let x = d4_early_leave.max(0) as u64; x.wrapping_mul(x) >> 8 } };
    p!(base + 0xd72791, &[0x48,0x3d], 2, 4, v_d4_early_leave);
    let v_d4_partner_dist: u64 = { if d4_partner_dist < 0 { 40000000000u64 } else { let x = d4_partner_dist.max(0) as u64; x.wrapping_mul(x) } };
    p!(base + 0xd71bbf, &[0x48,0xb9], 2, 8, v_d4_partner_dist);
    let v_d4_hp_safe: u64 = { if d4_hp_safe < 0 { 51u64 } else { d4_hp_safe.max(0) as u64 } };
    p!(base + 0xd71a64, &[0x48,0x83,0xf8], 3, 1, v_d4_hp_safe);
    let v_d4_from_mid: u64 = { if d4_from_mid < 0 { 1000u64 } else { d4_from_mid.max(0) as u64 } };
    p!(base + 0xd71a58, &[0x49,0x81,0x7c,0x08,0x60], 5, 4, v_d4_from_mid);
    let v_d4_from_mid_mode: u64 = { if d4_from_mid_mode < 0 { 2001u64 } else { d4_from_mid_mode.max(0) as u64 } };
    p!(base + 0xd720d5, &[0x48,0x81,0x78,0x10], 4, 4, v_d4_from_mid_mode);
    let v_d4_ally_cnt: u64 = { if d4_ally_cnt < 0 { 3u64 } else { d4_ally_cnt.max(0) as u64 } };
    p!(base + 0xd728f6, &[0x48,0x83,0xbc,0x24,0x88,0x00,0x00,0x00], 8, 1, v_d4_ally_cnt);
    p!(base + 0xd72934, &[0x48,0x83,0xbc,0x24,0x88,0x00,0x00,0x00], 8, 1, v_d4_ally_cnt);
    let v_d4_minion_cnt: u64 = { if d4_minion_cnt < 0 { 2u64 } else { d4_minion_cnt.max(0) as u64 } };
    p!(base + 0xd7290a, &[0x83,0xfe], 2, 1, v_d4_minion_cnt);
    let v_d4_gather_radius: u64 = { if d4_gather_radius < 0 { 150000u64 } else { d4_gather_radius.max(0) as u64 } };
    p!(base + 0xd721c7, &[0x48,0xc7,0x44,0x24,0x40], 5, 4, v_d4_gather_radius);
    D4_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("d4_imm.txt") {
        let _ = fs::write(pp, format!("applied={}/{} d4_ally_radius_a={} d4_ally_radius_b={} d4_early_leave={} d4_partner_dist={} d4_hp_safe={} d4_from_mid={} d4_from_mid_mode={} d4_ally_cnt={} d4_minion_cnt={} d4_gather_radius={} @base{:#x}\n",
            ok, tot, d4_ally_radius_a, d4_ally_radius_b, d4_early_leave, d4_partner_dist, d4_hp_safe, d4_from_mid, d4_from_mid_mode, d4_ally_cnt, d4_minion_cnt, d4_gather_radius, base));
    }
}

// ★★[08-04 2차] 아군 지원스킬 낭비방지 필터(`0xc3a3a0` line_defense.rs:595~735) + 콜리 3종
//   근거 = RE6-08-04_c3a3a0-생성기아니라retain-c72ae0-d90ab0-c365a0-0.5.3.md
//   전 사이트 정적 검증 통과(`gen_wave2.py` 205/205). 기본 -1 = 원본 유지.
unsafe fn apply_c3_imm() {
    let c3_enemy_near_a = tune("c3_enemy_near_a", -1);   // 지원스킬 유지 조건 — 아군 주변 적 탐지 반경(원본 120000, 33곳)
    let c3_enemy_near_b = tune("c3_enemy_near_b", -1);   // 동상(+1 형태, 15곳)
    let c3_minion_near = tune("c3_minion_near", -1);   // 미니언 근접 판정 반경(원본 120000, 3곳)
    let c3_ally_hp = tune("c3_ally_hp", -1);   // 지원스킬을 쓸 아군의 HP% 상한(원본 79, 6곳). ↓=진짜 위급할 때만 씀
    let c3_minion_margin = tune("c3_minion_margin", -1);   // 미니언 탐색 사거리 여유(원본 64000). ↑=먼 미니언도 후보
    let c3_hurt_scale = tune("c3_hurt_scale", -1);   // 아군 부상 판정 스케일(원본 100). 임계 = 80×100÷값 %
    let mut sig = 0u64;
    for v in [c3_enemy_near_a, c3_enemy_near_b, c3_minion_near, c3_ally_hp, c3_minion_margin, c3_hurt_scale] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == C3_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32; }}; }
    let v_c3_enemy_near_a: u64 = { if c3_enemy_near_a < 0 { 14400000000u64 } else { let x = c3_enemy_near_a.max(0) as u64; x.wrapping_mul(x) } };
    p!(base + 0xc3b6f5, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3b790, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3b820, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3b8b0, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3b937, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3bc41, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3bcdc, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3bd6c, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3bdfc, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3be83, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3c192, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3c22d, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3c2bd, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3c34d, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3c3d4, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3c9c6, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3aa44, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3aade, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3ab6e, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3abfe, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3ac85, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3ae8e, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3af28, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3afb8, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3b048, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3b0cf, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3b2d5, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3b36f, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3b3ff, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3b48f, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3b516, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3c794, &[0x49,0xb9], 2, 8, v_c3_enemy_near_a);
    p!(base + 0xc3c92d, &[0x49,0xb9], 2, 8, v_c3_enemy_near_a);
    let v_c3_enemy_near_b: u64 = { if c3_enemy_near_b < 0 { 14400000001u64 } else { let x = c3_enemy_near_b.max(0) as u64; x.wrapping_mul(x).wrapping_add(1) } };
    p!(base + 0xc3c5a3, &[0x48,0xb8], 2, 8, v_c3_enemy_near_b);
    p!(base + 0xc3c611, &[0x48,0xb8], 2, 8, v_c3_enemy_near_b);
    p!(base + 0xc3ca96, &[0x48,0xb8], 2, 8, v_c3_enemy_near_b);
    p!(base + 0xc3baf5, &[0x48,0xb9], 2, 8, v_c3_enemy_near_b);
    p!(base + 0xc3c041, &[0x48,0xb9], 2, 8, v_c3_enemy_near_b);
    p!(base + 0xc3ad8e, &[0x48,0xbb], 2, 8, v_c3_enemy_near_b);
    p!(base + 0xc3b1d8, &[0x48,0xbb], 2, 8, v_c3_enemy_near_b);
    p!(base + 0xc3b61a, &[0x49,0xb9], 2, 8, v_c3_enemy_near_b);
    p!(base + 0xc3bb61, &[0x49,0xb9], 2, 8, v_c3_enemy_near_b);
    p!(base + 0xc3c0ad, &[0x49,0xb9], 2, 8, v_c3_enemy_near_b);
    p!(base + 0xc3c678, &[0x49,0xb9], 2, 8, v_c3_enemy_near_b);
    p!(base + 0xc3c811, &[0x49,0xb9], 2, 8, v_c3_enemy_near_b);
    p!(base + 0xc3c736, &[0x49,0xba], 2, 8, v_c3_enemy_near_b);
    p!(base + 0xc3c8cf, &[0x49,0xba], 2, 8, v_c3_enemy_near_b);
    p!(base + 0xc3cb55, &[0x49,0xba], 2, 8, v_c3_enemy_near_b);
    let v_c3_minion_near: u64 = { if c3_minion_near < 0 { 14400000001u64 } else { let x = c3_minion_near.max(0) as u64; x.wrapping_mul(x).wrapping_add(1) } };
    p!(base + 0xd90bfd, &[0x49,0xba], 2, 8, v_c3_minion_near);
    p!(base + 0xd90c86, &[0x49,0xba], 2, 8, v_c3_minion_near);
    p!(base + 0xd90d0e, &[0x49,0xba], 2, 8, v_c3_minion_near);
    let v_c3_ally_hp: u64 = { if c3_ally_hp < 0 { 79u64 } else { c3_ally_hp.max(0) as u64 } };
    p!(base + 0xc3a629, &[0x48,0x83,0xf8], 3, 1, v_c3_ally_hp);
    p!(base + 0xc3a77f, &[0x48,0x83,0xf8], 3, 1, v_c3_ally_hp);
    p!(base + 0xc3a8d5, &[0x48,0x83,0xf8], 3, 1, v_c3_ally_hp);
    p!(base + 0xc3b637, &[0x48,0x83,0xf8], 3, 1, v_c3_ally_hp);
    p!(base + 0xc3bb83, &[0x48,0x83,0xf8], 3, 1, v_c3_ally_hp);
    p!(base + 0xc3c0cf, &[0x48,0x83,0xf8], 3, 1, v_c3_ally_hp);
    let v_c3_minion_margin: u64 = { if c3_minion_margin < 0 { 64000u64 } else { c3_minion_margin.max(0) as u64 } };
    p!(base + 0xe2321a, &[0x48,0x05], 2, 4, v_c3_minion_margin);
    let v_c3_hurt_scale: u64 = { if c3_hurt_scale < 0 { 100u64 } else { c3_hurt_scale.max(0) as u64 } };
    p!(base + 0xcc7d38, &[0x48,0x6b,0x97,0x58,0x06,0x00,0x00], 7, 1, v_c3_hurt_scale);
    p!(base + 0xcc7dfa, &[0x48,0x6b,0x96,0x58,0x06,0x00,0x00], 7, 1, v_c3_hurt_scale);
    p!(base + 0xcc7eb1, &[0x48,0x6b,0x96,0x58,0x06,0x00,0x00], 7, 1, v_c3_hurt_scale);
    p!(base + 0xcc7f68, &[0x48,0x6b,0x96,0x58,0x06,0x00,0x00], 7, 1, v_c3_hurt_scale);
    p!(base + 0xcc8019, &[0x49,0x6b,0x8b,0x58,0x06,0x00,0x00], 7, 1, v_c3_hurt_scale);
    C3_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("c3_imm.txt") {
        let _ = fs::write(pp, format!("applied={}/{} c3_enemy_near_a={} c3_enemy_near_b={} c3_minion_near={} c3_ally_hp={} c3_minion_margin={} c3_hurt_scale={} @base{:#x}\n",
            ok, tot, c3_enemy_near_a, c3_enemy_near_b, c3_minion_near, c3_ally_hp, c3_minion_margin, c3_hurt_scale, base));
    }
}

// ★★[08-04 2차] 스킬2/궁 해금 레벨의 **추가 사이트** — 기존 노브는 2곳만 잡고 있었다
//   근거 = RE6-08-04_Blackboard전필드-층2매퍼-소함수5종-0.5.3.md
//   전 사이트 정적 검증 통과(`gen_wave2.py` 205/205). 기본 -1 = 원본 유지.
unsafe fn apply_lv_imm() {
    let ex_ult_level_x = tune("ex_ult_level_x", -1);   // 궁 해금 레벨 추가 사이트(원본 5, 5곳). ex_ult_level과 값을 맞출 것
    let ex_skill2_level_x = tune("ex_skill2_level_x", -1);   // 스킬2 해금 레벨 추가 사이트(원본 3, 4곳). ex_skill2_level과 맞출 것
    let mut sig = 0u64;
    for v in [ex_ult_level_x, ex_skill2_level_x] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == LV_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32; }}; }
    let v_ex_ult_level_x: u64 = { if ex_ult_level_x < 0 { 5u64 } else { ex_ult_level_x.max(0) as u64 } };
    p!(base + 0xfdb9ed, &[0x49,0x83,0xff], 3, 1, v_ex_ult_level_x);
    p!(base + 0xfdba95, &[0x49,0x83,0xff], 3, 1, v_ex_ult_level_x);
    p!(base + 0xfdbb6d, &[0x49,0x83,0xff], 3, 1, v_ex_ult_level_x);
    p!(base + 0xc8ab0a, &[0x49,0x83,0xbe,0xb0,0x05,0x00,0x00], 7, 1, v_ex_ult_level_x);
    p!(base + 0xc3a7db, &[0x49,0x83,0xbd,0xb0,0x05,0x00,0x00], 7, 1, v_ex_ult_level_x);
    let v_ex_skill2_level_x: u64 = { if ex_skill2_level_x < 0 { 3u64 } else { ex_skill2_level_x.max(0) as u64 } };
    p!(base + 0xfcb9ad, &[0x49,0x83,0xff], 3, 1, v_ex_skill2_level_x);
    p!(base + 0xfcba4e, &[0x49,0x83,0xff], 3, 1, v_ex_skill2_level_x);
    p!(base + 0xfcbb1d, &[0x49,0x83,0xff], 3, 1, v_ex_skill2_level_x);
    p!(base + 0xc8aae4, &[0x49,0x83,0xbe,0xb0,0x05,0x00,0x00], 7, 1, v_ex_skill2_level_x);
    LV_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("lv_imm.txt") {
        let _ = fs::write(pp, format!("applied={}/{} ex_ult_level_x={} ex_skill2_level_x={} @base{:#x}\n",
            ok, tot, ex_ult_level_x, ex_skill2_level_x, base));
    }
}

// ★★[08-04 2차] 에픽/세르펜 사냥(`0xc688c0`·`0xda0750`) — 유일한 전술 리더 2핸들러. EPIC/SERP 쌍으로 배선
//   근거 = RE6-08-04_전술리더-epic_hunt-serpen_hunt-fin한필드-0.5.3.md
//   전 사이트 정적 검증 통과(`gen_wave2.py` 205/205). 기본 -1 = 원본 유지.
unsafe fn apply_eh_imm() {
    let eh_flee_clear_hp = tune("eh_flee_clear_hp", -1);   // 도주 상태 해제 HP%(원본 29, 4곳). ↑=더 오래 도망
    let eh_reach_margin = tune("eh_reach_margin", -1);   // 처치우선 전술 전용 교전 도달 마진(원본 25000, 4곳)
    let eh_recall_radius = tune("eh_recall_radius", -1);   // 귀환 국면 목표 반경(원본 60000, 2곳)
    let eh_around_radius = tune("eh_around_radius", -1);   // 사냥 중 배회 반경(원본 80000, 2곳)
    let eh_trace_arrive = tune("eh_trace_arrive", -1);   // 추적 도착 허용 반경(원본 15000, 10곳)
    let eh_band_low = tune("eh_band_low", -1);   // 접근 밴드 하한(원본 12000, 비교값 포함 8곳)
    let eh_band_high = tune("eh_band_high", -1);   // 접근 밴드 상한(원본 45000, 4곳)
    let eh_commit_hp = tune("eh_commit_hp", -1);   // 몬스터 HP% 이 값 미만이면 넓게 모임(원본 50, 2곳)
    let eh_commit_r_low = tune("eh_commit_r_low", -1);   // HP% 낮을 때 모이는 반경(원본 70000, 2곳)
    let eh_commit_r_high = tune("eh_commit_r_high", -1);   // HP% 높을 때 모이는 반경(원본 40000, 2곳)
    let eh_abort_hp = tune("eh_abort_hp", -1);   // 몬스터 HP% 이 값 초과면 사냥 포기(원본 44, 2곳)
    let eh_abort_dist = tune("eh_abort_dist", -1);   // 거리 상한 — 넘으면 포기(원본 220000, 2곳)
    let eh_score_norm = tune("eh_score_norm", -1);   // 거리→점수 정규화 상한(원본 320000, 12곳)
    let mut sig = 0u64;
    for v in [eh_flee_clear_hp, eh_reach_margin, eh_recall_radius, eh_around_radius, eh_trace_arrive, eh_band_low, eh_band_high, eh_commit_hp, eh_commit_r_low, eh_commit_r_high, eh_abort_hp, eh_abort_dist, eh_score_norm] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == EH_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32; }}; }
    let v_eh_flee_clear_hp: u64 = { if eh_flee_clear_hp < 0 { 29u64 } else { eh_flee_clear_hp.max(0) as u64 } };
    p!(base + 0xc68992, &[0x48,0x83,0xf8], 3, 1, v_eh_flee_clear_hp);
    p!(base + 0xc68a7f, &[0x48,0x83,0xf8], 3, 1, v_eh_flee_clear_hp);
    p!(base + 0xda0825, &[0x48,0x83,0xf8], 3, 1, v_eh_flee_clear_hp);
    p!(base + 0xda08c5, &[0x48,0x83,0xf8], 3, 1, v_eh_flee_clear_hp);
    let v_eh_reach_margin: u64 = { if eh_reach_margin < 0 { 25000u64 } else { eh_reach_margin.max(0) as u64 } };
    p!(base + 0xc6a83d, &[0x41,0xb8], 2, 4, v_eh_reach_margin);
    p!(base + 0xda253e, &[0x41,0xb8], 2, 4, v_eh_reach_margin);
    p!(base + 0xc6d3b5, &[0x48,0xc7,0x44,0x24,0x30], 5, 4, v_eh_reach_margin);
    p!(base + 0xda4fc3, &[0x48,0xc7,0x44,0x24,0x30], 5, 4, v_eh_reach_margin);
    let v_eh_recall_radius: u64 = { if eh_recall_radius < 0 { 60000u64 } else { eh_recall_radius.max(0) as u64 } };
    p!(base + 0xc69fd5, &[0x48,0xc7,0x44,0x24,0x20], 5, 4, v_eh_recall_radius);
    p!(base + 0xda1ce2, &[0x48,0xc7,0x44,0x24,0x20], 5, 4, v_eh_recall_radius);
    let v_eh_around_radius: u64 = { if eh_around_radius < 0 { 80000u64 } else { eh_around_radius.max(0) as u64 } };
    p!(base + 0xc6a026, &[0x48,0xc7,0x85,0xf8,0x04,0x00,0x00], 7, 4, v_eh_around_radius);
    p!(base + 0xda1d33, &[0x48,0xc7,0x85,0x08,0x05,0x00,0x00], 7, 4, v_eh_around_radius);
    let v_eh_trace_arrive: u64 = { if eh_trace_arrive < 0 { 15000u64 } else { eh_trace_arrive.max(0) as u64 } };
    p!(base + 0xc6a349, &[0x48,0xc7,0x85,0x08,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);
    p!(base + 0xc6aaff, &[0x48,0xc7,0x85,0x08,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);
    p!(base + 0xc6ac5d, &[0x48,0xc7,0x85,0x08,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);
    p!(base + 0xc6acf9, &[0x48,0xc7,0x85,0x08,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);
    p!(base + 0xc6ba21, &[0x48,0xc7,0x85,0x08,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);
    p!(base + 0xda2049, &[0x48,0xc7,0x85,0x18,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);
    p!(base + 0xda2837, &[0x48,0xc7,0x85,0x18,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);
    p!(base + 0xda28cf, &[0x48,0xc7,0x85,0x18,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);
    p!(base + 0xda2967, &[0x48,0xc7,0x85,0x18,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);
    p!(base + 0xda366b, &[0x48,0xc7,0x85,0x18,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);
    let v_eh_band_low: u64 = { if eh_band_low < 0 { 12000u64 } else { eh_band_low.max(0) as u64 } };
    p!(base + 0xc6b3a1, &[0xb9], 1, 4, v_eh_band_low);
    p!(base + 0xda300a, &[0xb9], 1, 4, v_eh_band_low);
    p!(base + 0xc6b3eb, &[0xbe], 1, 4, v_eh_band_low);
    p!(base + 0xda305b, &[0xbe], 1, 4, v_eh_band_low);
    p!(base + 0xc6b3f5, &[0x48,0xc7,0x85,0xa0,0x04,0x00,0x00], 7, 4, v_eh_band_low);
    p!(base + 0xda3065, &[0x48,0xc7,0x85,0xb0,0x04,0x00,0x00], 7, 4, v_eh_band_low);
    p!(base + 0xc6b39b, &[0x48,0x3d], 2, 4, v_eh_band_low.wrapping_add(1));
    p!(base + 0xda3004, &[0x48,0x3d], 2, 4, v_eh_band_low.wrapping_add(1));
    let v_eh_band_high: u64 = { if eh_band_high < 0 { 45000u64 } else { eh_band_high.max(0) as u64 } };
    p!(base + 0xc6b3aa, &[0x48,0x81,0xf9], 3, 4, v_eh_band_high);
    p!(base + 0xda3013, &[0x48,0x81,0xf9], 3, 4, v_eh_band_high);
    p!(base + 0xc6b3b1, &[0xbb], 1, 4, v_eh_band_high);
    p!(base + 0xda301a, &[0xbb], 1, 4, v_eh_band_high);
    let v_eh_commit_hp: u64 = { if eh_commit_hp < 0 { 50u64 } else { eh_commit_hp.max(0) as u64 } };
    p!(base + 0xc6bcb8, &[0x48,0x83,0xbd,0x98,0x05,0x00,0x00], 7, 1, v_eh_commit_hp);
    p!(base + 0xda3924, &[0x48,0x83,0xbd,0x90,0x05,0x00,0x00], 7, 1, v_eh_commit_hp);
    let v_eh_commit_r_low: u64 = { if eh_commit_r_low < 0 { 70000u64 } else { eh_commit_r_low.max(0) as u64 } };
    p!(base + 0xc6bcc0, &[0xb8], 1, 4, v_eh_commit_r_low);
    p!(base + 0xda392c, &[0xb8], 1, 4, v_eh_commit_r_low);
    let v_eh_commit_r_high: u64 = { if eh_commit_r_high < 0 { 40000u64 } else { eh_commit_r_high.max(0) as u64 } };
    p!(base + 0xc6bcc5, &[0x41,0xbc], 2, 4, v_eh_commit_r_high);
    p!(base + 0xda3931, &[0x41,0xbe], 2, 4, v_eh_commit_r_high);
    let v_eh_abort_hp: u64 = { if eh_abort_hp < 0 { 44u64 } else { eh_abort_hp.max(0) as u64 } };
    p!(base + 0xc6bd5d, &[0x48,0x83,0xbd,0x98,0x05,0x00,0x00], 7, 1, v_eh_abort_hp);
    p!(base + 0xda39c2, &[0x48,0x83,0xbd,0x90,0x05,0x00,0x00], 7, 1, v_eh_abort_hp);
    let v_eh_abort_dist: u64 = { if eh_abort_dist < 0 { 220000u64 } else { eh_abort_dist.max(0) as u64 } };
    p!(base + 0xc6bd6b, &[0x48,0x81,0xbd,0x38,0x03,0x00,0x00], 7, 4, v_eh_abort_dist);
    p!(base + 0xda39d0, &[0x48,0x81,0xbd,0x48,0x03,0x00,0x00], 7, 4, v_eh_abort_dist);
    let v_eh_score_norm: u64 = { if eh_score_norm < 0 { 320000u64 } else { eh_score_norm.max(0) as u64 } };
    p!(base + 0xc6c145, &[0x48,0x3d], 2, 4, v_eh_score_norm);
    p!(base + 0xc6c14b, &[0x41,0xb8], 2, 4, v_eh_score_norm);
    p!(base + 0xc6c191, &[0xba], 1, 4, v_eh_score_norm);
    p!(base + 0xc6c45e, &[0x48,0x3d], 2, 4, v_eh_score_norm);
    p!(base + 0xc6c464, &[0xb9], 1, 4, v_eh_score_norm);
    p!(base + 0xc6c4ea, &[0xb9], 1, 4, v_eh_score_norm);
    p!(base + 0xda3d96, &[0x48,0x3d], 2, 4, v_eh_score_norm);
    p!(base + 0xda3d9c, &[0x41,0xb8], 2, 4, v_eh_score_norm);
    p!(base + 0xda3de2, &[0xba], 1, 4, v_eh_score_norm);
    p!(base + 0xda409e, &[0x48,0x3d], 2, 4, v_eh_score_norm);
    p!(base + 0xda40a4, &[0xb9], 1, 4, v_eh_score_norm);
    p!(base + 0xda412a, &[0xb9], 1, 4, v_eh_score_norm);
    EH_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("eh_imm.txt") {
        let _ = fs::write(pp, format!("applied={}/{} eh_flee_clear_hp={} eh_reach_margin={} eh_recall_radius={} eh_around_radius={} eh_trace_arrive={} eh_band_low={} eh_band_high={} eh_commit_hp={} eh_commit_r_low={} eh_commit_r_high={} eh_abort_hp={} eh_abort_dist={} eh_score_norm={} @base{:#x}\n",
            ok, tot, eh_flee_clear_hp, eh_reach_margin, eh_recall_radius, eh_around_radius, eh_trace_arrive, eh_band_low, eh_band_high, eh_commit_hp, eh_commit_r_low, eh_commit_r_high, eh_abort_hp, eh_abort_dist, eh_score_norm, base));
    }
}