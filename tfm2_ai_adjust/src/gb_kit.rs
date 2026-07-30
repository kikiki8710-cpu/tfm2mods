// gb_kit.rs — generic_build(영역D) 캡처/검증 하네스. (스코어러 재현 본체 = genbuild_repro.rs)
// 들어있는 것: gbrd_capture(영역D mid-func 캡처), genbuild_body_capture(kind14 리턴검증). GB statics/cfg키·gb_region_d_050=genbuild_repro.rs.
// 언제 손대나: MIG_GB_CHANGED 재검증·영역D 캡처 재활성 시(현재 미설치 보류). ⚠다음세션 재검증 예정=폐기 아님.



// ★반환 i64(install_detour_d_skip용): RAX_SENT=passthrough(게임 region D 실행) / out ptr=HANDLED(skip, 우리출력 기록후 funnel jump).
//   verify(gbrd)/overwrite(gbrepl/chk)는 항상 SENT(passthrough+capture). skip(gbskip)만 Some&&push==0시 out 반환(진짜 계산대체).
unsafe extern "C" fn gbrd_capture(saved: usize, _entry_rsp: usize) -> i64 {
    if GBRD.load(Ordering::Relaxed) || GBREPL.load(Ordering::Relaxed) || GBREPLCHK.load(Ordering::Relaxed) { GBRD_RAW.fetch_add(1, Ordering::Relaxed); }   // ★성능: 진단캡처 켜졌을때만(프로덕션 캐시라인 바운싱 제거)
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return RAX_SENT; }
    if !GBRD.load(Ordering::Relaxed) && !GBREPL.load(Ordering::Relaxed) && !GBREPLCHK.load(Ordering::Relaxed) && !GBSKIP.load(Ordering::Relaxed) { return RAX_SENT; }
    // ★대체/체크/skip은 store 캡 없음(매 호출). verify(GBRD-only)만 4000 캡.
    if !GBREPL.load(Ordering::Relaxed) && !GBREPLCHK.load(Ordering::Relaxed) && !GBSKIP.load(Ordering::Relaxed) && GBRD_ARMED.load(Ordering::Relaxed) >= GBRD_ARM_MAX { return RAX_SENT; }
    let mbase = exe_base();
    if mbase == 0 { return RAX_SENT; }
    let rbp = rd_u64(saved + 0x38).unwrap_or(0) as usize;   // 게임 rbp(프레임베이스)
    if !ptr_ok(rbp) || !readable(rbp + 0x2b8, 8) { GBRD_BADPTR.fetch_add(1, Ordering::Relaxed); return RAX_SENT; }
    let out = rd_u64(rbp + 0x2b8).unwrap_or(0) as usize;    // ★0.5.0_3 out@rbp+0x2b8(was 0x290, ghidra-re 확정) = kind14 대조키
    if !ptr_ok(out) || !readable(out + 0x80, 8) { GBRD_BADPTR.fetch_add(1, Ordering::Relaxed); return RAX_SENT; }
    // ★0.5.0_3 gb_region_d_050 7입력(ghidra-re 규명): cnt@0x110(스코어러#1 TTD) da@0x100(sqrt거리) db@0x108(스코어러#2 TTD) d2@0x170(후보Vec cnt) l2@0x1d0
    let cnt = rd_i64(rbp + 0x110).unwrap_or(0);
    let da = rd_i64(rbp + 0x100).unwrap_or(0);
    let db = rd_i64(rbp + 0x108).unwrap_or(0);
    let d2 = rd_u64(rbp + 0x170).unwrap_or(0);
    let l2 = rd_u64(rbp + 0x1d0).unwrap_or(0);
    // ★score/sim_scale = dedc0 인라인(캡처점서 미산출→직접계산): score=max(0, *(refA+0x20)−vt0x28(ent)), sim_scale=*(*([rbp+0x180]+8)+0x12f8)
    let sim = rd_u64(rbp + 0x180).unwrap_or(0) as usize;
    let sim_scale = if ptr_ok(sim) { rd_i64(rd_u64(sim + 8).unwrap_or(0) as usize + 0x12f8).unwrap_or(0) } else { 0 };
    let refa = rd_u64(rbp + 0x2a0).unwrap_or(0) as usize;
    let refa20 = if ptr_ok(refa) { rd_i64(refa + 0x20).unwrap_or(0) } else { 0 };
    let vt28ret: i64 = if GBDEDC0.load(Ordering::Relaxed) {   // ★vt0x28 shadow-call(게임상태 walk)=cfg gbdedc0 게이트 격리(기본off=score≈refA[0x20])
        let ent = rd_u64(rbp + 0x2d8).unwrap_or(0) as usize;
        let vt = rd_u64(rbp + 0x1a0).unwrap_or(0) as usize;
        if ptr_ok(ent) && ptr_ok(vt) {
            let slot = rd_u64(vt + 0x28).unwrap_or(0) as usize;
            if ptr_ok(slot) { let f: unsafe extern "C" fn(usize) -> i64 = core::mem::transmute(slot);
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(ent))).unwrap_or(0) } else { 0 }
        } else { 0 }
    } else { 0 };
    let score = (refa20 - vt28ret).max(0);
    let pred_code = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| gb_region_d_050(cnt, da, db, d2, l2, score, sim_scale))) {
        Ok(v) => v, _ => { GBRD_PANIC.fetch_add(1, Ordering::Relaxed); return RAX_SENT; }
    };
    let entry_vlen = rd_u64(out + 0x78).unwrap_or(0);   // 영역 D 진입시 action Vec len(A/B/C 누적). 리턴서 delta=D push.
    if GBRD.load(Ordering::Relaxed) || GBREPL.load(Ordering::Relaxed) || GBREPLCHK.load(Ordering::Relaxed) {
        // ★0.5.0_3 관측(1단계): pred_code(0=SKIP/3=MOVE/0xb=ENGAGE_B/-1=ABORT) ↔ game out기록(kind14 대조부서 push/kind 병치)로 매핑 파악. pred=(k=pred_code, a=sim_scale, push=0).
        let dump = format!("cnt={} da={} db={} d2={:#x} l2={:#x} score={} ss={} pred={}", cnt, da, db, d2, l2, score, sim_scale, pred_code);
        if let Ok(mut m) = GBRD_MAP.lock() {
            let e = Some((pred_code, sim_scale as u64, 0u16));
            if let Some(x) = m.iter_mut().find(|x| x.0 == out) { *x = (out, e, dump, entry_vlen); }
            else if m.len() < 256 { m.push((out, e, dump, entry_vlen)); }
        }
    }
    GBRD_ARMED.fetch_add(1, Ordering::Relaxed);
    // ★1단계=관측 전용: GBSKIP/대체는 pred_code↔game out기록 매핑 확정 후 재활성(pred 형식이 0.4.13 kind/arg와 달라 지금 덮어쓰면 게임 손상). 지금은 passthrough.
    RAX_SENT
}

// ★generic_build 본체(0x20def90, task#23) 출력 캡처: 진입서 (disc,param2,team) 스냅 + out포인터 저장 → 리턴훅(kind14)서
//   out struct kind@+0x58/arg@+0x60/action Vec 읽기. 매프레임 수백만콜 → unique (disc,param2) 키별 GBB_PER_KEY개만 arm.
//   순수 read(게임호출 제로)라 안전. install_detour saved: rcx@+0x28(out), rdx@+0x20(param2), r9@+0x10(athlete). arg7=S champion@entry_rsp+0x38.
unsafe extern "C" fn genbuild_body_capture(saved: usize, entry_rsp: usize) {
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    GBB_RAW.fetch_add(1, Ordering::Relaxed);
    if !GBBODY.load(Ordering::Relaxed) && !GBRD.load(Ordering::Relaxed) { return; }  // gbrd=verify(kind14). 대체(gbrepl)는 에필로그 hook이 처리하므로 여기 무장 불요.
    if GBB_ARMED.load(Ordering::Relaxed) >= GBB_ARM_MAX { return; }
    let thunk = RET_THUNK.load(Ordering::Relaxed);
    if thunk == 0 { return; }
    let out     = rd_u64(saved + 0x28).unwrap_or(0) as usize;        // rcx = out(0x90B sret)
    let param2  = rd_u64(saved + 0x20).unwrap_or(0);                 // rdx = param2
    let athlete = rd_u64(saved + 0x10).unwrap_or(0) as usize;        // r9  = athlete A
    let s_champ = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize;    // arg7 = S champion
    if !ptr_ok(out) || !ptr_ok(s_champ) || !ptr_ok(athlete) { return; }
    let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    if !ptr_ok(orig_ret) || !readable(entry_rsp, 8) { return; }
    let team = rd_u64(athlete + 0x6a8).unwrap_or(99) as i64;
    // disc-2D = (byte[S+0x3e8]<<16) | word[S+0x3e6]
    let dword = rd_u8(s_champ + 0x3e8) as u32;
    let dlo   = (rd_u8(s_champ + 0x3e6) as u32) | ((rd_u8(s_champ + 0x3e7) as u32) << 8);
    let disc  = (dword << 16) | dlo;
    // unique (disc,param2) 키별 상한 → 분포 골고루
    let key = ((disc as u64) << 20) | (param2 & 0xfffff);
    // ★gbrd 페어링: GBRD on이면 throttle 우회(모든 리턴 무장) → 0x42a3 store를 그 invocation 리턴이 1:1 consume
    //   = out-key 충돌(다른 invocation 출력이 재사용 out슬롯에 lingering) 제거. gbbody-only면 기존 (disc,p2) throttle.
    let ok = if GBRD.load(Ordering::Relaxed) { true } else if let Ok(mut sv) = GBB_SEEN.lock() {
        if let Some(e) = sv.iter_mut().find(|x| x.0 == key) {
            if e.1 >= GBB_PER_KEY { false } else { e.1 += 1; true }
        } else if sv.len() < 8192 { sv.push((key, 1)); true } else { false }
    } else { false };
    if !ok { return; }
    let (sil, mid, hi) = (disc & 0xff, (disc >> 8) & 0xff, (disc >> 16) & 0xff);
    // ★my_generic_build 예측(영역 A early-exit). rh_chain=arg5 value(*=rhd). panic-safe(catch_unwind).
    let gctx = GBCtx {
        param2, athlete, rh_chain: rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize, s_champ,
        o_30: rd_i64(out + 0x30).unwrap_or(0), o_38: rd_i64(out + 0x38).unwrap_or(0), o_60: rd_u64(out + 0x60).unwrap_or(0),
    };
    let (mine, parg): (i64, u64) = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_generic_build(&gctx, 0))) {
        Ok(Some((k, a))) => (k, a), _ => (-99, 0),
    };
    let pre = format!("[gbb #{}] disc={:#x}(lo={} mid={} hi={}) p2={} team={}",
        GBB_ARMED.load(Ordering::Relaxed), disc, sil, mid, hi, param2 as i64, team);
    let pushed = if let Ok(mut st) = RET_STACK.lock() {
        if st.len() < 64 { st.push(RetFrame{ key: entry_rsp, orig_ret, mine, kind: 14, pre, p5: out, p6: parg as usize, disp_pred: 0 }); true } else { false }
    } else { false };
    if pushed {
        core::ptr::write_unaligned(entry_rsp as *mut usize, thunk);
        GBB_ARMED.fetch_add(1, Ordering::Relaxed);
    }
}
