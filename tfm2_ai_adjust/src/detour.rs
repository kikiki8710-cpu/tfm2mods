// detour.rs — 트램폴린/훅 설치 인프라 + 정적 바이트패치.
// 들어있는 것: install_* 전 디투어 설치기, build_ret_thunk/hook_return/RET 하네스, build_shim_*, alloc_near,
//   move_override/commit_dump/threatgate_adjust/df0c10_post/itemnet_guard_ok(훅 콜백), patch6/14/patch_imm_bytes,
//   apply_engage_base/thr_mult/lane_gate/type3_ablate/call_ablate/objective_imm/numbers_sp(정적패치).
// 언제 손대나: 훅 설치 방식·트램폴린 ABI·바이트패치 변경 시. detour 안전수칙=[[tfm2-mod-safety]].

// ══════ ★[07-30] 스텁 인벤토리 — 크래시의 `module=unknown` 을 훅 이름으로 역해석하기 위한 표 ══════
//   왜 필요한가: 우리 트램폴린/스텁은 VirtualAlloc(RWX)로 잡은 **어느 모듈에도 속하지 않는** 메모리다.
//     그래서 여기서 폴트가 나면 Windows 이벤트로그(WER)가 `Faulting module name: unknown` + 절대주소만 남기고,
//     crash_log.txt 도 `exe+`/`MOD+` 둘 다 매칭 실패해 **아무 단서를 못 남긴다**(2026-07-30 실측: 3건 중 2건이 이 상태).
//   ⟹ 스텁을 잡을 때마다 (주소, 크기, 타깃 RVA)를 여기 등록해 두고, 크래시 시 RIP 를 이 표와 대조해
//      `STUB(rva=0x…)+off` 로 찍는다. 어느 훅의 트램폴린에서 죽었는지 즉시 특정된다.
//   ⚠크래시 문맥(VEH/UEF)에서 읽으므로 **고정 배열 + 원자 카운터만** 쓴다(alloc/lock/format! 금지 — §3).
const STUB_MAX: usize = 24;
static STUB_N: AtomicUsize = AtomicUsize::new(0);
static mut STUB_TBL: [(usize, usize, usize); STUB_MAX] = [(0, 0, 0); STUB_MAX];   // (addr, size, tag=타깃RVA/식별자)
/// VirtualAlloc 결과를 그대로 통과시키며 표에 등록한다(할당 실패=0이면 무등록). tag = 타깃 RVA, 없으면 식별상수.
#[inline] pub(crate) unsafe fn stub_reg(addr: usize, size: usize, tag: usize) -> usize {
    if addr != 0 {
        let i = STUB_N.fetch_add(1, Ordering::Relaxed);
        if i < STUB_MAX { STUB_TBL[i] = (addr, size, tag); }
    }
    addr
}
/// 크래시 문맥 전용: 주소가 등록된 스텁 안이면 (tag, offset) 반환. 순수 읽기라 어느 스레드/문맥에서도 안전.
#[inline] pub(crate) unsafe fn stub_lookup(a: usize) -> Option<(usize, usize)> {
    let n = STUB_N.load(Ordering::Relaxed).min(STUB_MAX);
    for i in 0..n {
        let (base, sz, tag) = STUB_TBL[i];
        if base != 0 && a >= base && a < base + sz { return Some((tag, a - base)); }
    }
    None
}
/// 설치 결과를 LOG_ON 무관으로 파일에 확증(진단 로그가 꺼져 있어도 남아야 하는 정보).
unsafe fn stub_dump() {
    let n = STUB_N.load(Ordering::Relaxed).min(STUB_MAX);
    let mut s = format!("=== 훅 스텁 인벤토리 (n={}) exe_base={:#x} ===\n\
                         # 크래시 이벤트로그가 module=unknown 이면 그 offset 을 아래 [addr, addr+size) 와 대조하라.\n", n, exe_base());
    for i in 0..n {
        let (a, sz, tag) = STUB_TBL[i];
        s.push_str(&format!("stub[{:2}] addr={:#x} size={:#x} end={:#x} tag={:#x}{}\n",
            i, a, sz, a + sz, tag, stub_tag_name(tag)));
    }
    if let Some(p) = pth("hooks.txt") { let _ = fs::write(p, s); }
}
fn stub_tag_name(t: usize) -> &'static str {
    match t {
        RVA_RETREAT => "  (retreat)", RVA_FC59A0 => "  (fc59a0/recall)",
        RVA_CONDGATE => "  (condgate)", RVA_MOVEPRI => "  (movepri)",
        RVA_DISC18_HANDLER => "  (disc18)", RVA_DISC19_HANDLER => "  (disc19)",
        RVA_GENERIC_BUILD => "  (generic_build)", RVA_ITEMNET_SCORER => "  (itemnet guard)",
        RVA_GB_REGIOND_HOOK => "  (gb region-D)",
        0xF001 => "  (ret_thunk)", 0xF002 => "  (shim_both)", 0xF003 => "  (shim_rdx)",
        0xF004 => "  (call_stub)", 0xF005 => "  (alloc_near)",
        _ => "",
    }
}

// kind: 0=TTD(game=retval vs mine), 1=RE(retreat_engage: 결정=*retval). pre=로그프리픽스.
struct RetFrame { key: usize, orig_ret: usize, mine: i64, kind: u8, pre: String, p5: usize, p6: usize, disp_pred: i64 }


// thunk: rax=retval로 진입, rsp=key+8. hook_return(retval, key)→orig_ret 호출 후 rax복원·orig_ret 점프.
unsafe fn build_ret_thunk() {
    let handler = hook_return as *const () as usize;
    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(&[0x50]);                 // push rax            (retval 보존; rsp=key=ESP0)
    code.extend_from_slice(&[0x48,0x89,0xC1]);       // mov rcx, rax        (arg1=retval)
    code.extend_from_slice(&[0x48,0x89,0xE2]);       // mov rdx, rsp        (arg2=key=ESP0)
    code.extend_from_slice(&[0x4C,0x8B,0x84,0x24,0x50,0xFF,0xFF,0xFF]); // mov r8,[rsp-0xb0]  (arg3=e1; RE=local_b0 게임임계값@ESP0-0xb0)
    code.extend_from_slice(&[0x4C,0x8B,0x8C,0x24,0xD8,0xFD,0xFF,0xFF]); // mov r9,[rsp-0x228] (arg4=e2; RE=local_228 셀렉터@ESP0-0x228)
    code.extend_from_slice(&[0x48,0x8B,0x84,0x24,0xE8,0xFD,0xFF,0xFF]); // mov rax,[rsp-0x218] (tmp=local_218 idx)
    code.extend_from_slice(&[0x4C,0x8B,0x94,0x24,0xB0,0xFF,0xFF,0xFF]); // mov r10,[rsp-0x50]  (tmp=local_50 df1da0반환)
    code.extend_from_slice(&[0x48,0x83,0xEC,0x38]);  // sub rsp,0x38        (16정렬 + shadow + 2 stack args)
    code.extend_from_slice(&[0x48,0x89,0x44,0x24,0x20]); // mov [rsp+0x20],rax  (arg5=local_218)
    code.extend_from_slice(&[0x4C,0x89,0x54,0x24,0x28]); // mov [rsp+0x28],r10  (arg6=local_50)
    code.extend_from_slice(&[0x48,0xB8]); code.extend_from_slice(&handler.to_le_bytes()); // movabs rax,handler
    code.extend_from_slice(&[0xFF,0xD0]);            // call rax
    code.extend_from_slice(&[0x48,0x83,0xC4,0x38]);  // add rsp,0x38
    code.extend_from_slice(&[0x49,0x89,0xC2]);       // mov r10, rax        (orig_ret)
    code.extend_from_slice(&[0x58]);                 // pop rax             (retval 복원; rsp=key+8)
    code.extend_from_slice(&[0x41,0xFF,0xE2]);       // jmp r10
    let m = stub_reg(VirtualAlloc(0, 64, 0x1000|0x2000, 0x40), 64, 0xF001);
    if m != 0 { core::ptr::copy_nonoverlapping(code.as_ptr(), m as *mut u8, code.len()); RET_THUNK.store(m, Ordering::Relaxed); }
}

// 공용 리턴 thunk 핸들러: retval=반환값, key=ESP0, e1=local_b0(게임임계값), e2=local_228(셀렉터), e3=local_218(idx), e4=local_50(df1da0반환).
unsafe extern "C" fn hook_return(retval: i64, key: usize, e1: i64, e2: i64, e3: i64, e4: i64) -> usize {
    let frame = if let Ok(mut st) = RET_STACK.lock() {
        st.iter().rposition(|f| f.key == key).map(|p| st.remove(p))
    } else { None };
    match frame {
        Some(f) => {
            // ★panic-safe(mod-safety): 리턴훅 verify/logging panic이 FFI UB로 게임 크래시 → catch_unwind 차단.
            //   orig_ret은 먼저 추출해 패닉시에도 정상 복귀(게임 흐름 유지). 전 kind(0/1/2/3/5/9/11) 공통보호.
            let ret = f.orig_ret;
            let hr = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if f.kind == 5 {
                // fc59a0 recall score: 게임출력 *p5 = score@+0(i32), bool@+4(u8), mult@+8(i32). f.mine=내 RNG배율 m, f.disp_pred=threshold.
                let op = f.p5;
                let score = rd_i32(op).unwrap_or(-999) as i64;
                let gbool = if readable(op+4,1) { std::ptr::read_unaligned((op+4) as *const u8) as i64 } else { -1 };
                let mult  = rd_i32(op+8).unwrap_or(-999) as i64;
                let my_m = f.mine; let thr = f.disp_pred;
                let my_mult = f.p6 as i64;                     // 내 base-score 재현 mult
                let mok = my_mult != RECALL_MULT_NONE && my_mult == mult;   // ★base-score 검증(게임 mult 대조)
                let mtag = if my_mult == RECALL_MULT_NONE { "mult:N/A".to_string() }
                           else if mok { "mult:OK".to_string() } else { format!("★mult-DIFF(my={} game={})", my_mult, mult) };
                let n = RECALL_ARMED.load(Ordering::Relaxed);
                if n <= RECALL_ARM_MAX {
                    let (verdict, detail) = if mult == 0 && score == 0 {
                        ("early-out".to_string(), "(후보없음/조기반환, RNG미소비)".to_string())
                    } else {
                        let pred_score = (my_m * mult) / 100;          // score = (m*mult)/100 검증
                        let pred_bool = (thr <= pred_score) as i64;
                        let sok = pred_score == score;
                        let bok = pred_bool == gbool;
                        let v = if sok && bok { "OK" } else if sok { "bool-DIFF" } else { "★score-DIFF" };
                        (v.to_string(), format!("game_score={} pred={}({}*{}/100) | game_bool={} pred_bool={}", score, pred_score, my_m, mult, gbool, pred_bool))
                    };
                    let s = format!("{} → mult={} [{}] {} [{}]\n", f.pre, mult, mtag, detail, verdict);
                    if !RECALL_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("recallcmp.txt", "=== fc59a0 recall RNG score 검증: score=(m*mult)/100, bool=(thr<=score). m=내 read-only RNG draw 재현 ===\n"); }
                    append_named("recallcmp.txt", &s);
                }
            } else if f.kind == 6 {
                // facet#1 condgate: 게임 al = retval&0xff. f.mine=my_condgate(-99=pending poke/gank).
                let game_al = (retval & 0xff) as i64;
                let my = f.mine;
                // ★in-scope RNG draw 측정(cond_repl 안전 재확인): condgate가 RNG 소비했나? replaced disc(my≠-99)가 0이어야 skip 안전.
                COND_INSCOPE.store(false, Ordering::Relaxed);
                let draws = COND_IS_DRAWS.load(Ordering::Relaxed);
                let di6 = (f.p5).min(15);
                if draws > COND_DISC_MAXDRAW[di6].load(Ordering::Relaxed) { COND_DISC_MAXDRAW[di6].store(draws, Ordering::Relaxed); }
                let def = COND_IS_DEF.load(Ordering::Relaxed);   // fcd980+fcdaf0=항상 실제 draw
                let e88 = COND_IS_E88.load(Ordering::Relaxed);   // e88a0 실제 draw(count>0)
                let e9 = COND_IS_E9.load(Ordering::Relaxed);     // e9a30 호출
                let real = def + e88;   // 확실한 실제 draw(e9는 count불명이라 별도)
                if draws > 0 {  // RNG 함수 호출한 condgate 케이스. real>0=확실히 desync위험.
                    if !CONDRNG_INIT.swap(true, Ordering::Relaxed) { write_named("condrng.txt", "=== facet#1 condgate in-scope RNG: def(fcd980/af0=실제) e88(count>0) e9(호출). replaced(my≠-99)+real>0=desync위험 ===\n"); }
                    append_named("condrng.txt", &format!("disc={} my={} def={} e88={} e9={} real={} | LEAK누적={} [{}]\n", f.p5, my, def, e88, e9, real, COND_LEAK.load(Ordering::Relaxed),
                        if my != -99 && real > 0 {"★REPLACED+REAL_RNG=desync확실(or누수)"} else if my != -99 && e9 > 0 {"replaced+e9호출(count확인필요)"} else if my == -99 {"passthrough(안전)"} else {"replaced(RNG=0)"}));
                }
                if my == -99 { COND_PEND.fetch_add(1, Ordering::Relaxed); }
                else if my == game_al { COND_OK.fetch_add(1, Ordering::Relaxed); }
                else { COND_DIFF.fetch_add(1, Ordering::Relaxed); }
                let n = COND_ARMED.load(Ordering::Relaxed);
                if n <= COND_ARM_MAX {
                    let verdict = if my == -99 { "pending" } else if my == game_al { "OK" } else { "★DIFF" };
                    let s = format!("{} → game={} [{}] | OK={} DIFF={} PEND={}\n", f.pre, game_al, verdict, COND_OK.load(Ordering::Relaxed), COND_DIFF.load(Ordering::Relaxed), COND_PEND.load(Ordering::Relaxed));
                    if !COND_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("condcmp.txt", "=== facet#1 condgate: my_condgate vs 게임 al (subplan별, -99=pending) ===\n"); }
                    append_named("condcmp.txt", &s);
                }
            } else if f.kind == 7 {
                // facet#4 movepriority: 출력구조체 *p5. code@+0, 필드 +8/+0x10/+0x20/+0x21.
                let op = f.p5;
                let code = rd_i64(op).unwrap_or(-999);
                let my = f.mine;
                // ★④ 출력계약 덤프: disc별 게임 출력구조 head(6 qword + key byte). aux 비-0 오프셋 식별 → replace 재현범위.
                {
                    let di = (f.p6 as usize).min(15);
                    if MPOUT_CNT[di].fetch_add(1, Ordering::Relaxed) < 6 && readable(op, 0x30) {
                        let q: [i64;6] = core::array::from_fn(|k| rd_i64(op + k*8).unwrap_or(0));
                        let b12 = rd_u8(op + 0x12); let b21 = rd_u8(op + 0x21);
                        if !MPOUT_INIT.swap(true, Ordering::Relaxed) { write_named("mpout.txt", "=== ④ movepriority 출력계약: disc별 게임 출력구조 (code@+0, aux 비0 오프셋 식별) ===\n"); }
                        append_named("mpout.txt", &format!("[disc={} code={}] +8={:#x} +0x10={:#x} +0x18={:#x} +0x20={:#x} +0x28={:#x} | b+0x12={} b+0x21={}\n",
                            f.p6, code, q[1], q[2], q[3], q[4], q[5], b12, b21));
                    }
                }
                // ★출력계약 write-set: 진입스냅(MP_ENTRY) vs 현재 *op = sub-judge가 쓴 qword오프셋 비트마스크. code-only(=0b1)/aux 판별.
                if MP_ENTRY_PTR.load(Ordering::Relaxed) == op && readable(op, 0x40) {
                    let mut ws = 0u64;
                    for k in 0..8usize { if rd_u64(op + k*8).unwrap_or(0) != MP_ENTRY[k].load(Ordering::Relaxed) { ws |= 1 << k; } }
                    let di = (f.p6 as usize).min(15);
                    let prev = MP_WS[di].fetch_or(ws, Ordering::Relaxed);
                    if (prev | ws) != prev {   // 새 비트 발견 → 로그
                        if !MP_WS_INIT.swap(true, Ordering::Relaxed) { write_named("mpws.txt", "=== movepriority sub-judge write-set (qword offset 비트: bit0=+0(code) bit1=+8 bit2=+0x10 ...) code-only=0b1 ===\n"); }
                        append_named("mpws.txt", &format!("[disc={}] write-set=0b{:08b} (오프셋: {})\n", f.p6, prev|ws,
                            (0..8).filter(|k| (prev|ws)>>k & 1 == 1).map(|k| format!("+0x{:x}", k*8)).collect::<Vec<_>>().join(",")));
                    }
                }
                // ★★[07-31] disc10·11 은 **code 대조 대상에서 제외**한다(N/A).
                //   근거: 게임 핸들러의 공통 에필로그(0.5.3 `0xc55d37`)가 `mov qword [out], 0xb` 로 **tag 를 상수 11 로 고정**하고,
                //         실제 판단결과는 payload `out+8` 에 넣는다 ⟹ `my`(payload 계열)와 `code`(=tag)는 애초에 다른 량이다.
                //   ⚠이건 2026-07-23 에 이미 규명돼 소스 주석(tfm2_ai_adjust.rs L6502~6504)에 적혀 있었는데도
                //     **로그가 계속 ★DIFF 를 찍는 바람에** 07-30·07-31 세션이 "재현 결함 2,591건"으로 오독했다.
                //     ⟹ 주석만으로는 재발을 못 막는다는 게 실증됐으므로 **집계에서 아예 뺀다**.
                //   정본 지표 = `pokecmp` 바이트대조(write-set 전량 비교).
                let tag_fixed = f.p6 == 10 || f.p6 == 11;
                if tag_fixed { MP_NA.fetch_add(1, Ordering::Relaxed); }
                else if my == -99 { MP_PEND.fetch_add(1, Ordering::Relaxed); }
                else if my == code { MP_OK.fetch_add(1, Ordering::Relaxed); }
                else { MP_DIFF.fetch_add(1, Ordering::Relaxed); }
                // ★★[07-23] disc0/1/3 tail 오판(my∈{6,7} → game=2) 종단 추격: DIFF시 STAGE6 종단태그+진단 덤프.
                if my != code && my != -99 && !tag_fixed && (f.p6 == 0 || f.p6 == 1 || f.p6 == 3) {
                    let dn = DD0_DIFF_N.fetch_add(1, Ordering::Relaxed) + 1;
                    if dn <= 300 {
                        append_named("dd0diff.txt", &format!(
                            "[#{}] disc={} my={} game={} path={} TERM={} | ivar2={} plan={} bl={} route={} t86dd={} t872d={} || count={} near={} n={} efield={} kind={}\n",
                            dn, f.p6, my, code, DD7_PATH.load(Ordering::Relaxed), DD7_TERM.load(Ordering::Relaxed),
                            DD7_DBG[0].load(Ordering::Relaxed), DD7_DBG[1].load(Ordering::Relaxed), DD7_DBG[2].load(Ordering::Relaxed),
                            DD7_DBG[3].load(Ordering::Relaxed), DD7_DBG[4].load(Ordering::Relaxed), DD7_DBG[5].load(Ordering::Relaxed),
                            DD7_DBG[6].load(Ordering::Relaxed), DD7_DBG[7].load(Ordering::Relaxed),
                            DD7_DBG[8].load(Ordering::Relaxed), DD7_DBG[9].load(Ordering::Relaxed),
                            DD7_DBG[10].load(Ordering::Relaxed)));
                    }
                }
                let n = MP_ARMED.load(Ordering::Relaxed);
                if n <= 30000 {
                    let verdict = if tag_fixed { "N/A:tag고정" } else if my == -99 { "pending" } else if my == code { "OK" } else { "★DIFF" };
                    let s = format!("{} → game_code={} [{}] | OK={} DIFF={} PEND={} NA={}\n", f.pre, code, verdict, MP_OK.load(Ordering::Relaxed), MP_DIFF.load(Ordering::Relaxed), MP_PEND.load(Ordering::Relaxed), MP_NA.load(Ordering::Relaxed));
                    if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt",
                        "=== facet#4 movepriority: my_movepriority vs 게임코드 (subplan별, -99=pending) ===\n\
                         # ★disc10·11 = [N/A:tag고정] — 게임 out+0 이 enum tag 상수 0xb 라 code 비교가 성립하지 않는다(정본 지표=pokecmp).\n\
                         #   이 둘을 DIFF 로 세면 매 경기 2,500여 건의 가짜 불일치가 잡힌다(2026-07-23 규명 / 07-31 집계 제외).\n"); }
                    append_named("mpcmp.txt", &s);
                }
                // ★0.5.0 disc9/11 full-output 검증(재작성): 게임출력 op에 내 재현writes 적용 후 byte대조. epic=out-struct(0x2d), serpen=0xb래핑(0x14), battle=기존. 진입스냅(MP_ENTRY)기반.
                if (f.p6 == 9 || f.p6 == 11 || f.p6 == 10 || f.p6 == 12 || f.p6 == 4) && MP_AUX_OP.load(Ordering::Relaxed) == op && (my != -99 || (f.p6 == 12 && SERPEN_VERIFY.load(Ordering::Relaxed))) && readable(op, 0x30) {
                    let mut buf = [0u8; 0x30];
                    for i in 0..0x30usize { buf[i] = rd_u8(op + i); }
                    let p2sj = MP_AUX_P2.load(Ordering::Relaxed);
                    let bufp = buf.as_mut_ptr() as usize;
                    // 재현: disc9=epic_poke_write · disc11=write_serpen_out(char) · disc4=disc4_aux · disc10=EpicBattle(weight) · disc12=SerpenBattle(미포팅→skip).
                    // ★[07-23] `disp_my` = 로그에 찍을 재현 code. disc12는 f.mine이 **항상 -99**(my_movepriority가 12=>-99 하드코딩)라
                    //   구 `my={f.mine}` 표시가 무의미했다(미해결 ① "my=-99 아티팩트"). disc12는 my_serpen_battle의 실제 반환 c를 찍는다.
                    let (wrote, win, disp_my) = if f.p6 == 4 { (write_disc4_aux(bufp, my, p2sj), 0x22usize, my) }
                        else if f.p6 == 9 { (epic_poke_write(bufp, p2sj, MP_AUX_P3.load(Ordering::Relaxed), MP_AUX_P5.load(Ordering::Relaxed), MP_AUX_P6.load(Ordering::Relaxed)), 0x2dusize, my) }
                        else if f.p6 == 11 { (write_serpen_out(bufp, my), 0x14usize, my) }
                        else if f.p6 == 10 {   // EpicBattle: out+0=0xb/+8=weight(my)/+0x10=1/+0x12=0
                            std::ptr::write_unaligned(bufp as *mut u64, 0xbu64);
                            std::ptr::write_unaligned((bufp + 8) as *mut u64, my as u64);
                            std::ptr::write_unaligned((bufp + 0x10) as *mut u16, 1u16);
                            std::ptr::write_unaligned((bufp + 0x12) as *mut u8, 0u8);
                            (true, 0x14usize, my)
                        }
                        else if f.p6 == 12 {   // ★SerpenBattle body 재현(out-writer, SERPEN_VERIFY 게이트). bufp에 직접기록 후 게임출력 대조.
                            let c = my_serpen_battle(bufp, p2sj, MP_AUX_P3.load(Ordering::Relaxed) as i64, MP_AUX_RNG.load(Ordering::Relaxed),
                                MP_AUX_P5.load(Ordering::Relaxed), MP_AUX_P6.load(Ordering::Relaxed), MP_AUX_TP.load(Ordering::Relaxed), MP_AUX_SF.load(Ordering::Relaxed), false, None);   // ★live=false: 검증 리턴훅은 게임이 이미 RNG 소비 후 → 재드로우 금지 / ★aux=None: 이 경로는 캡처가 채운 MP_AUX_* 전역 스냅샷 사용(검증 전용·단일 경로)
                            (c != -99, 0x14usize, c)   // ★disp_my=c(실제 재현 code) — -99(진입가드 튕김)면 wrote=false라 아래 미기록
                        }
                        else { (false, 0usize, my) };
                    if wrote {
                        let mut diff_at: i64 = -1;
                        for i in 0..win { if buf[i] != rd_u8(op + i) { diff_at = i as i64; break; } }
                        let ok = diff_at < 0;
                        if ok { POKE_OK.fetch_add(1, Ordering::Relaxed); } else { POKE_DIFF.fetch_add(1, Ordering::Relaxed); }
                        let n2 = POKE_OK.load(Ordering::Relaxed) + POKE_DIFF.load(Ordering::Relaxed);
                        if !ok || n2 % 500 == 0 || n2 <= 5 {
                            if !POKE_INIT.swap(true, Ordering::Relaxed) { write_named("pokecmp.txt", "=== 0.5.0 disc9/11 EpicPoke/SerpenPoke 재작성 full-output 검증: 내재현 vs 게임출력 byte대조 ===\n"); }
                            let verdict = if ok { "OK".to_string() } else { format!("★DIFF@+0x{:x}", diff_at) };
                            let mb: [u8;0x2d] = core::array::from_fn(|i| buf[i]);
                            let gb: [u8;0x2d] = core::array::from_fn(|i| rd_u8(op + i));
                            append_named("pokecmp.txt", &format!("[poke {}] disc={} my={} win=0x{:x} [{}] OK={} DIFF={}\n  my  ={:02x?}\n  game={:02x?}\n",
                                n2, f.p6, disp_my, win, verdict, POKE_OK.load(Ordering::Relaxed), POKE_DIFF.load(Ordering::Relaxed), mb, gb));
                        }
                        // ★disc12 0xe/0xc divergence 특정 덤프(2026-07-10): 미스매치시 SGT 경로 상태 한 줄(sgate.txt).
                        //   stage=engage_gate 마지막 도달 지점(0진입~10true), aux7/aux8=stage6(cnt/lane)·stage9(best_idx/sim lane).
                        if !ok && f.p6 == 12 {
                            let sgn = SGT_N.fetch_add(1, Ordering::Relaxed);
                            append_named("sgate.txt", &format!(
                                "[sg {}] my={} game_q0={} gate={} stage={} aux7={} aux8={} d2>>8={} thr>>8={} ord={} tgt={} m18={} m19={} tick={} tag={} zenc={:#x} op=0x{:x} sim=0x{:x}\n",
                                sgn, SGT[0].load(Ordering::Relaxed), rd_u64(op).unwrap_or(0),
                                SGT[5].load(Ordering::Relaxed), SGT[6].load(Ordering::Relaxed),
                                SGT[7].load(Ordering::Relaxed), SGT[8].load(Ordering::Relaxed),
                                SGT[9].load(Ordering::Relaxed), SGT[10].load(Ordering::Relaxed),
                                SGT[1].load(Ordering::Relaxed), SGT[2].load(Ordering::Relaxed),
                                SGT[3].load(Ordering::Relaxed), SGT[4].load(Ordering::Relaxed),
                                SGT[11].load(Ordering::Relaxed), SGT[12].load(Ordering::Relaxed), SGT[13].load(Ordering::Relaxed),
                                op, MP_AUX_P5.load(Ordering::Relaxed)));
                        }
                        // ★disc9 EpicPoke 미스매치 원인 특정 덤프(egate.txt): epic_poke_compute 게이트별 상태 한 줄.
                        //   game_q1(op+8)이 힙포인터면 game이 타겟 리졸브함. my=f.mine(0=후보없음 의심). cand/pgpass/contested로 갈림지점 판별.
                        if !ok && f.p6 == 9 {
                            let egn = EGT_N.fetch_add(1, Ordering::Relaxed);
                            append_named("egate.txt", &format!(
                                "[eg {}] my={} game_q0={} game_q1={} active={} pgpass={} p3={} bvar1={} disc={} clane={} cand={} lane={} eside={} nlist={} contested={} reach={} op=0x{:x}\n",
                                egn, my, rd_u64(op).unwrap_or(0), rd_u64(op + 8).unwrap_or(0),
                                EGT[0].load(Ordering::Relaxed), EGT[10].load(Ordering::Relaxed),
                                EGT[1].load(Ordering::Relaxed), EGT[2].load(Ordering::Relaxed),
                                EGT[3].load(Ordering::Relaxed), EGT[4].load(Ordering::Relaxed),
                                EGT[5].load(Ordering::Relaxed), EGT[6].load(Ordering::Relaxed),
                                EGT[7].load(Ordering::Relaxed), EGT[8].load(Ordering::Relaxed),
                                EGT[9].load(Ordering::Relaxed), EGT[11].load(Ordering::Relaxed), op));
                        }
                    }
                }
                // ★disc9/11 RNG footprint 측정: 진입 스냅(POKE_RNG_*) vs exit p4 → 실제 RNG 소비(words/refills). draw 분포·early-guard 상관 파악 → my_poke_rng_final 모델링용.
                if (f.p6 == 9 || f.p6 == 11) {
                    let p4 = POKE_RNG_P4.load(Ordering::Relaxed);
                    if p4 != 0 && readable(p4 + 0x138, 8) {
                        let i0 = POKE_RNG_I0.load(Ordering::Relaxed);
                        let c0 = POKE_RNG_C0.load(Ordering::Relaxed);
                        let i1 = rd_u64(p4 + 0x100).unwrap_or(0);
                        let c1 = rd_u64(p4 + 0x130).unwrap_or(0);
                        let refills = c1.wrapping_sub(c0) / 4;
                        let words = (i1 as i64 + 64 * refills as i64) - i0 as i64;   // 소비 u32 워드수(refill 보정)
                        let n2 = POKE_RNG_N_CTR.fetch_add(1, Ordering::Relaxed);
                        // ★재구성 검증: 예측 exit(POKE_PIDX/PCTR, reconstructed args) vs 실제 p4 exit(i1,c1).
                        let pcount = POKE_PCOUNT.load(Ordering::Relaxed);
                        let (pidx, pctr) = (POKE_PIDX.load(Ordering::Relaxed), POKE_PCTR.load(Ordering::Relaxed));
                        let e_ok = pcount >= 0 && i1 == pidx && c1 == pctr;
                        if pcount >= 0 { if e_ok { POKE_E88_OK.fetch_add(1, Ordering::Relaxed); } else { POKE_E88_DIFF.fetch_add(1, Ordering::Relaxed); } }
                        if !POKERNG_INIT.swap(true, Ordering::Relaxed) { write_named("pokerng.txt", "=== disc9/11 RNG: p4 delta + e88a0 재구성 검증(예측 exit vs 실제). eOK=재구성정확 ===\n"); }
                        if n2 < 4000 || !e_ok {
                            append_named("pokerng.txt", &format!("[pokerng {}] disc={} code={} plan={} | i0={} i1={} refills={} words={} | myCount={} pred(idx={} ctr={}) e88[{}] eOK={} eDIFF={}\n",
                                n2, f.p6, code, POKE_RNG_PLAN.load(Ordering::Relaxed), i0, i1, refills, words,
                                pcount, pidx, pctr, if pcount<0 {"n/a"} else if e_ok {"OK"} else {"★DIFF"},
                                POKE_E88_OK.load(Ordering::Relaxed), POKE_E88_DIFF.load(Ordering::Relaxed)));
                        }
                        POKE_RNG_P4.store(0, Ordering::Relaxed);
                    }
                }
                if f.p6 == 9 || f.p6 == 11 { POKE_INSCOPE.store(false, Ordering::Relaxed); }   // ★RNG caller 추적 윈도우 종료(p4 무관 항상 해제)
                if f.p6 == 14 && code != 18 { defwatch_log(code, my, f.disp_pred); }   // 캡내 disc14의 7-케이스도 watcher 기록
                if f.p6 == 9 && my == 7 && code != 7 {   // ★epic 7-DIFF 진단: 어느 7-출구가 과발동했나
                    let dn = EPICDIAG_N.fetch_add(1, Ordering::Relaxed);
                    if dn < 200 {
                        let d = EPIC_DIAG.load(Ordering::Relaxed);
                        let s = format!("[epicdiff #{}] game={} my=7 | reason={} hp%={} obj_full={} not_home={} side={} self_z7={} other_z7={} obj_hp={} thr_lt={}\n",
                            dn, code, d & 0xf, (d>>4)&0xff, (d>>12)&1, (d>>13)&1, (d>>14)&1, (d>>16)&0xf, (d>>20)&0xf, (d>>24)&0xff, (d>>32)&1);
                        if !EPICDIAG_INIT.swap(true, Ordering::Relaxed) { write_named("epicdiff.txt", "=== EpicPoke 7-DIFF 진단 (reason 1~5 = 어느 return-7) ===\n"); }
                        append_named("epicdiff.txt", &s);
                    }
                }
                if f.p6 == 9 && my == 13 {   // ★engage(13) 진단: 2 DIFF가 champ999==1로 갈리나
                    let en = ENGDIAG_N.fetch_add(1, Ordering::Relaxed);
                    if en < 300 {
                        let d = ENG_DIAG.load(Ordering::Relaxed);
                        let dsq = ENG_DIST.load(Ordering::Relaxed);
                        let thr = 0x53d1ac101u64;   // dist²<thr% : 100=임계바로아래, 작을수록 멀리(=fdae40 의심)
                        let verdict = if code == 13 { "OK" } else { "★DIFF" };
                        let s = format!("[eng #{}] game={} my=13 [{}] champ999={} champ3e6={} side={} dist²={} ({}%of임계)\n",
                            en, code, verdict, (d>>16)&0xff, (d>>24)&0xff, (d>>14)&1, dsq, dsq.saturating_mul(100)/thr);
                        if !ENGDIAG_INIT.swap(true, Ordering::Relaxed) { write_named("epiceng.txt", "=== EpicPoke engage(13) 진단: champ999/champ3e6 (fdae40 게이트) ===\n"); }
                        append_named("epiceng.txt", &s);
                    }
                }
                if f.p6 == 9 && my == 11 && code != 11 {   // ★epic my=11 DIFF 진단: 어느 게이트서 갈렸나
                    let d = EPIC11_DIAG.load(Ordering::Relaxed);
                    let s = format!("[epic11] game={} my=11 | reason={} fdae40={} node2[c1={} c2={} c3={} c4={} c5={} heq={}] zone_app={} side={} flag={} champ999={} champ3e6={} zsf={} zot={} zhp={}\n",
                        code, d&7, (d>>3)&1, (d>>4)&1, (d>>5)&1, (d>>6)&1, (d>>7)&1, (d>>8)&1, (d>>9)&1, (d>>10)&1, (d>>11)&1, (d>>12)&1, (d>>16)&0xff, (d>>24)&0xff, (d>>32)&0xff, (d>>40)&0xff, (d>>48)&0xff);
                    append_named("epic11.txt", &s);
                }
            } else if f.kind == 8 {
                // DefenseNexus 7-watcher(무제한): game!=18(=7) 케이스만 기록
                let code = rd_i64(f.p5).unwrap_or(-999);
                if code != 18 { defwatch_log(code, f.mine, f.disp_pred); }
            } else if f.kind == 14 {
                // ★generic_build 본체(0x20def90) 출력: out struct kind@+0x58 / arg@+0x60 / action sub-Vec(+0x70 ptr, +0x78 len, entry stride 0x18, word=code).
                let out = f.p5;
                let mbase = exe_base();
                let kind = rd_i64(out + 0x58).unwrap_or(-99);
                let arg = rd_u64(out + 0x60).unwrap_or(0) as usize;
                let argr = if arg > mbase && arg < mbase + 0x10000000 { format!("rva+{:#x}", arg - mbase) } else { format!("{:#x}", arg) };
                if GBBODY.load(Ordering::Relaxed) {
                    let sentinel = rd_i64(out).unwrap_or(-99);
                    let hdr8d = (rd_u8(out + 0x8d) as u32) | ((rd_u8(out + 0x8e) as u32) << 8);
                    let (h89, h8a, h8b, h8f) = (rd_u8(out + 0x89), rd_u8(out + 0x8a), rd_u8(out + 0x8b), rd_u8(out + 0x8f));
                    let vlen = rd_u64(out + 0x78).unwrap_or(0);
                    let vptr = rd_u64(out + 0x70).unwrap_or(0) as usize;
                    let mut vcodes = String::new();
                    for i in 0..vlen.min(8) {
                        let p = vptr + (i as usize) * 0x18;
                        let code = (rd_u8(p) as u16) | ((rd_u8(p + 1) as u16) << 8);
                        vcodes.push_str(&format!("{:#x},", code));
                    }
                    let (mk, ma) = (f.mine, f.p6 as u64);   // my_generic_build 예측 (kind, arg)
                    let verdict = if mk == -99 { GBB_NOPRED.fetch_add(1, Ordering::Relaxed); "미예측".to_string() }
                        else if mk == kind && ma == (arg as u64) { GBB_OK.fetch_add(1, Ordering::Relaxed); "OK".to_string() }
                        else { GBB_DIFF.fetch_add(1, Ordering::Relaxed); format!("★DIFF(my k={} a={:#x})", mk, ma) };
                    let s = format!("{} → kind={} arg={} [{}] (OK={} DIFF={} NP={}) sent={} hdr8d={:#x} h89/8a/8b/8f={}/{}/{}/{} vlen={} v=[{}]\n",
                        f.pre, kind, argr, verdict, GBB_OK.load(Ordering::Relaxed), GBB_DIFF.load(Ordering::Relaxed), GBB_NOPRED.load(Ordering::Relaxed),
                        sentinel, hdr8d, h89, h8a, h8b, h8f, vlen, vcodes);
                    if !GBB_FILE_INIT.swap(true, Ordering::Relaxed) {
                        write_named("gbbody.txt", "=== generic_build 본체(0x20def90) 출력 캡처: (disc,p2,team) → (kind@+0x58, arg@+0x60, action Vec) ===\n");
                    }
                    append_named("gbbody.txt", &s);
                }
                // ★gbrd: 0x20e42a3 mid-func 캡처가 저장한 gb_region_d 예측을 out ptr로 조회 → game kind/arg 대조 → gbrdcmp.txt.
                //   같은 invocation서 0x42a3(store) → 함수리턴(여기서 consume). 0x42a3 미도달 invocation은 맵에 없음(=영역D 깊은분기 우회, 1차 무방).
                let gbrd_ent = if let Ok(mut m) = GBRD_MAP.lock() {
                    m.iter().position(|x| x.0 == out).map(|p| m.remove(p))
                } else { None };
                if let Some((_, pred, dump, entry_vlen)) = gbrd_ent {
                    // ★action Vec 검증: 영역 D delta = 최종 len − entry_vlen = 영역 D가 push한 코드. (out+0x78 len, out+0x70 ptr, stride 0x18, word=code)
                    let fvlen = rd_u64(out + 0x78).unwrap_or(0);
                    let dn = fvlen.saturating_sub(entry_vlen);   // 영역 D push 개수
                    if dn > 0 { GBRD_VPUSH.fetch_add(1, Ordering::Relaxed); }
                    // game 영역 D push: dn==0→0 / dn==1→그 코드 / dn>1→0xffff(예상밖)
                    let vptr = rd_u64(out + 0x70).unwrap_or(0) as usize;
                    let game_push: u16 = if dn == 0 { 0 } else if dn == 1 && ptr_ok(vptr) {
                        (rd_u8(vptr + (entry_vlen as usize) * 0x18) as u16) | ((rd_u8(vptr + (entry_vlen as usize) * 0x18 + 1) as u16) << 8)
                    } else { 0xffff };
                    if GBRD.load(Ordering::Relaxed) {   // verify 로깅은 gbrd일 때만(gbrepl 단독시 로그폭증 방지)
                        let ga = arg as u64;
                        let verdict = match pred {
                            Some((pk, pa, ppush)) => if pk == kind && pa == ga && ppush == game_push {
                                GBRD_OK.fetch_add(1, Ordering::Relaxed); "OK".to_string()
                            } else {
                                GBRD_DIFF.fetch_add(1, Ordering::Relaxed); format!("★DIFF(my k={} a={:#x} push={:#x})", pk, pa, ppush)
                            },
                            None => { GBRD_NP.fetch_add(1, Ordering::Relaxed); "미예측(영역D 분기 TODO)".to_string() }
                        };
                        let mut dcodes = String::new();
                        if dn > 0 && ptr_ok(vptr) {
                            for i in entry_vlen..fvlen.min(entry_vlen + 8) {
                                let p = vptr + (i as usize) * 0x18;
                                let code = (rd_u8(p) as u16) | ((rd_u8(p + 1) as u16) << 8);
                                dcodes.push_str(&format!("{:#x},", code));
                            }
                        }
                        let s = format!("[gbrd] game kind={} arg={} push={:#x} [{}] (OK={} DIFF={} NP={}) Dvec(d={} ev={} [{}]) | {}\n",
                            kind, argr, game_push, verdict, GBRD_OK.load(Ordering::Relaxed), GBRD_DIFF.load(Ordering::Relaxed), GBRD_NP.load(Ordering::Relaxed),
                            dn, entry_vlen, dcodes, dump);
                        if !GBRD_FILE_INIT.swap(true, Ordering::Relaxed) {
                            write_named("gbrdcmp.txt", "=== 영역 D gb_region_d 검증: 캡처 locals → 예측 vs game out (kind/arg) + Dvec(영역D push delta) ===\n");
                        }
                        append_named("gbrdcmp.txt", &s);
                    }
                    // ★대체(gbrepl)는 에필로그 hook(gbrd_epilogue_apply)이 100% inline 처리 → kind14서 제거.
                    let _ = pred;
                }
            } else {
                // RE: retval=puVar3(출력ptr) → 결정=*retval. e1=game임계값(local_b0), e2=셀렉터, e3=idx, e4=df1da0반환.
                let decision = if ptr_ok(retval as usize) { rd_i64(retval as usize).unwrap_or(0) } else { 0 };
                let pred = f.mine;                          // -1=retreat, 0=none, 9999=proceed(예측없음)
                let has_pred = pred != 9999;
                let violation = has_pred && decision != pred;
                // ★2단계 검증: 임계값 충실재현(결정론적, RNG無). 디컴 332-392:
                //   local_228==1: arr=[[[p6+8]+0x20]+8]; data=[arr+idx*0x10]; vt=[arr+8+idx*0x10]; role=(*[vt+0x68])(data); {4:100,3:70,2:50,_:30}
                //   else: (local_50==1)?(p5[0x7a]?0:10):0
                let sel: i64 = -777;   // (df0c10 셀렉터 훅 제거 — 항상 미신선. 0.5.1 정리)
                let my_thr: i64 = if false && (sel as i32) == 1 {   // ★0.5.0: role getter vt+0x68 shadow-call = AV 크래시(vtable slot 0.5.0 stale). thr검증은 어차피 bypass(game garbage)라 무시됨 → shadow-call 비활성(false 게이트). role threshold 재검증시 vt+0x68 slot 0.5.0 확정 후 복원.
                    let a = rd_u64(f.p6 + 8).unwrap_or(0) as usize;
                    let b = rd_u64(a + 0x20).unwrap_or(0) as usize;
                    let arr = rd_u64(b + 8).unwrap_or(0) as usize;
                    let idx = e3 as usize;
                    if ptr_ok(arr) && idx < 64 && readable(arr + idx*0x10, 0x10) {
                        let data = rd_u64(arr + idx*0x10).unwrap_or(0) as usize;
                        let vt = rd_u64(arr + 8 + idx*0x10).unwrap_or(0) as usize;
                        if ptr_ok(data) && ptr_ok(vt) && readable(vt + 0x68, 8) {
                            let g = rd_u64(vt + 0x68).unwrap_or(0) as usize;
                            if ptr_ok(g) {
                                let gf: Getter1 = core::mem::transmute(g);
                                match gf(data) { 4 => 100, 3 => 70, 2 => 50, _ => 30 }
                            } else { -777 }
                        } else { -777 }
                    } else { -777 }
                } else {
                    if e4 == 1 { if rd_i64(f.p5 + 0x7a*8).unwrap_or(0) != 0 { 0 } else { 10 } } else { 0 }
                };
                // e1(게임 local_b0)은 2차게이트 우회시 garbage. 유효 임계값(0/10/30/50/70/100)일때만 비교.
                let e1_valid = matches!(e1, 0|10|30|50|70|100);
                let thr_v = if my_thr == -777 { "thr:가드스킵" }
                    else if !e1_valid { "thr:bypass(game garbage)" }
                    else if e1 == my_thr { "thr:OK✓" } else { "thr:★DIFF" };
                let n = RE_LOGGED.load(Ordering::Relaxed);
                let roll_v = String::new();   // (fcd980 롤예측 훅 제거 — ROLL 대조 비활성)
                let _ = violation;
                let dtag = if decision == 5 { "ENGAGE(5)" } else if decision == 7 { "RECALL(7)" } else if decision == -1 { "RETREAT(-1)" } else if decision == 0 { "NONE(0)" } else if decision == 3 { "ZONE(3)" } else if decision == 8 { "STAND(8)" } else { "OTHER" };
                let pv = if has_pred { if decision == pred { "PRED-OK" } else { "★PRED-VIOLATION" } } else { "proceed" };
                // ★my_dispatch_code 라이브 검증: f.disp_pred(진입시 예측 7/8/3) vs 실제 decision
                // ★완전정복 갭측정: my_dispatch_code 예측(7/8/3)이 실제 디스패치 출력과 일치? 아니면 roll/none으로 빠짐(mispredict)?
                // ★2026-06-19 수정: disp_pred는 "dispatch 도달시" 조건부 예측 → proceed(lp_pred=9999) 케이스만 MISPREDICT 집계.
                //   (lane_pred=0 퇴각 케이스는 my_full이 lp_pred로 -1 산출=정답이므로 disp_pred 무관 → 오집계 방지.)
                let is_misp = f.mine == 9999 && matches!(f.disp_pred, 3|7|8) && !matches!(decision, 3|7|8);
                let disp_v = if matches!(decision, 3|7|8) {
                    let md = f.disp_pred;
                    if md == decision { DISP_OK.fetch_add(1, Ordering::Relaxed); format!(" | ★DISP mydisp={} [DISP-OK✓]", md) }
                    else { DISP_DIFF.fetch_add(1, Ordering::Relaxed); format!(" | ★DISP mydisp={} [DISP-★DIFF]", md) }
                } else if is_misp {
                    DISP_DIFF.fetch_add(1, Ordering::Relaxed);
                    format!(" | ★DISP mydisp={} actual={} [★MISPREDICT(예측디스패치→실제roll/none)]", f.disp_pred, decision)
                } else { String::new() };
                // ★통합 출력 예측 my_full = lp_pred(lane/none) + dispatch + roll → 전 출력(-1/0/3/5/7/8) game==mine 측정.
                //   f.mine=lp_pred(0=none/-1=lane퇴각/9999=proceed). proceed면 disp_pred(3/7/8) or roll(5/-1).
                let roll_out: i64 = -777;   // (fcd980 롤예측 훅 제거)
                let my_full: i64 =
                    if f.mine == 0 { 0 } else if f.mine == -1 { -1 }
                    else if matches!(f.disp_pred, 3|7|8) { f.disp_pred } else { roll_out };
                let full_v = if my_full == -777 { String::new() }
                    else if my_full == decision { FULL_OK.fetch_add(1, Ordering::Relaxed); String::new() }
                    else { FULL_DIFF.fetch_add(1, Ordering::Relaxed); format!(" [★FULL-DIFF myfull={} act={}]", my_full, decision) };
                let is_full_diff = !full_v.is_empty();
                let s = format!("{} → out={} [{} {}] | sel_fresh={} idx={} game_thr={} my_thr={} [{}]{}{}{}\n", f.pre, decision, dtag, pv, sel, e3, e1, my_thr, thr_v, roll_v, disp_v, full_v);
                // ★디스패치(3/7/8) + cVar6==2 + ★FULL-DIFF(통합예측 틀린것=진짜갭) → dispcmp.txt 고캡(2000).
                let is_cv2 = f.pre.contains("cVar6=2 ");
                if matches!(decision, 3|7|8) || is_cv2 || is_full_diff {
                    if DISP_LOGGED.fetch_add(1, Ordering::Relaxed) < 2000 { append_named("dispcmp.txt", &s); }
                } else if n < 60 {
                    RE_LOGGED.fetch_add(1, Ordering::Relaxed);
                    append_named("recmp.txt", &s);
                }
                // ★engage footprint 측정: 진입 스냅 → 출력별 총 RNG delta(words = refills*64 + i1 - i0). engfoot.txt.
                if let Ok(mut sn) = RE_SNAP.lock() {
                    if let Some(pos) = sn.iter().rposition(|x| x.0 == f.key) {
                        let (_, state, i0, c0, pred_out, pred_words, pca, pcb) = sn.remove(pos);
                        if readable(state + 0x138, 8) {
                            let i1 = rd_u64(state + 0x100).unwrap_or(0);
                            let c1 = rd_u64(state + 0x130).unwrap_or(0);
                            let refills = c1.wrapping_sub(c0) / 4;
                            let words = refills.wrapping_mul(64).wrapping_add(i1).wrapping_sub(i0) as i64;
                            // ★engage 예측 검증: pred_out/pred_words(my_engage_predict) vs 실제 (decision, words). pred=-777=비engage(skip).
                            if pred_out != -777 {
                                let ok = pred_out == decision && pred_words == words;
                                if ok { EP_OK.fetch_add(1, Ordering::Relaxed); } else { EP_DIFF.fetch_add(1, Ordering::Relaxed); }
                                let pn = EP_OK.load(Ordering::Relaxed) + EP_DIFF.load(Ordering::Relaxed);
                                if !ok || pn % 500 == 0 || pn <= 40 {
                                    if !EFOOT_INIT.swap(true, Ordering::Relaxed) { write_named("engfoot.txt", "=== engage 예측검증: my_engage_predict(out,words) vs 실제(decision,words). gate early-exit은 DIFF로 노출 ===\n"); }
                                    let roll_fired = false;   // (fcd980 롤예측 훅 제거)
                                    append_named("engfoot.txt", &format!("[ep {}] my(out={} words={}) game(out={} words={}) [{}] | i0={} i1={} refills={} ca={} cb={} roll_fired={} EP_OK={} EP_DIFF={}\n",
                                        pn, pred_out, pred_words, decision, words, if ok {"OK"} else {"★DIFF"}, i0, i1, refills, pca, pcb, roll_fired, EP_OK.load(Ordering::Relaxed), EP_DIFF.load(Ordering::Relaxed)));
                                }
                            }
                        }
                    }
                }
            }
            }));
            if hr.is_err() {
                let c = HR_PANIC.fetch_add(1, Ordering::Relaxed);
                if c < 30 { append_named("recmp.txt", &format!("[★PANIC caught] hook_return kind={} — verify 건너뜀(orig_ret 정상복귀)\n", f.kind)); }
            }
            ret
        }
        None => 0,
    }
}

type Shim5 = unsafe extern "C" fn(usize, usize, usize, usize, usize) -> i64;

type Getter4 = unsafe extern "C" fn(usize, usize, usize, usize) -> i64;

type ShimBoth = unsafe extern "C" fn(usize, usize, usize, usize, usize, usize);  // (out[2], getter, a,b,c,d)

// ★게터를 1회만 호출 → (rax,rdx) 둘 다 out[0],out[1]에 기록. 비멱등 게터(소환수)도 정확.
unsafe fn build_shim_both() {
    let code: [u8; 43] = [
        0x53,                       // push rbx
        0x48,0x89,0xCB,             // mov rbx, rcx        (out)
        0x49,0x89,0xD2,             // mov r10, rdx        (getter)
        0x4C,0x89,0xC1,             // mov rcx, r8         (getter arg1=a)
        0x4C,0x89,0xCA,             // mov rdx, r9         (getter arg2=b)
        0x4C,0x8B,0x44,0x24,0x30,   // mov r8, [rsp+0x30]  (arg3=c, +8 for pushed rbx)
        0x4C,0x8B,0x4C,0x24,0x38,   // mov r9, [rsp+0x38]  (arg4=d)
        0x48,0x83,0xEC,0x20,        // sub rsp,0x20        (shadow, 16-align 유지)
        0x41,0xFF,0xD2,             // call r10            (getter 1회)
        0x48,0x83,0xC4,0x20,        // add rsp,0x20
        0x48,0x89,0x03,             // mov [rbx], rax      (out[0]=base1)
        0x48,0x89,0x53,0x08,        // mov [rbx+8], rdx    (out[1]=base2)
        0x5B,                       // pop rbx
        0xC3,                       // ret
    ];
    let m = stub_reg(VirtualAlloc(0, 64, 0x1000|0x2000, 0x40), 64, 0xF002);
    if m != 0 { core::ptr::copy_nonoverlapping(code.as_ptr(), m as *mut u8, code.len()); SHIM_BOTH.store(m, Ordering::Relaxed); }
}

unsafe fn build_shim_rdx() {
    let code: [u8; 32] = [
        0x49,0x89,0xCA,             // mov r10, rcx (target)
        0x48,0x89,0xD1,             // mov rcx, rdx (a)
        0x4C,0x89,0xC2,             // mov rdx, r8  (b)
        0x4D,0x89,0xC8,             // mov r8, r9   (c)
        0x4C,0x8B,0x4C,0x24,0x28,   // mov r9, [rsp+0x28] (d)
        0x48,0x83,0xEC,0x28,        // sub rsp,0x28
        0x41,0xFF,0xD2,             // call r10
        0x48,0x83,0xC4,0x28,        // add rsp,0x28
        0x48,0x89,0xD0,             // mov rax, rdx
        0xC3,                       // ret
    ];
    let m = stub_reg(VirtualAlloc(0, 64, 0x1000|0x2000, 0x40), 64, 0xF003);
    if m != 0 { core::ptr::copy_nonoverlapping(code.as_ptr(), m as *mut u8, code.len()); SHIM_RDX.store(m, Ordering::Relaxed); }
}

unsafe fn patch6(addr: usize, bytes: &[u8; 6]) {
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(addr, 6, RWX, &mut old) == 0 { return; }
    core::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, 6);
    VirtualProtect(addr, 6, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, 6);
}

unsafe fn apply_lane_gate() {
    let want = LANE_GATE.load(Ordering::Relaxed);
    if want == LANE_GATE_APPLIED.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 { return; }
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let addr = base + LANE_GATE_RVA;
    if !readable(addr, 6) { return; }
    let cur: [u8; 6] = [rd_u8(addr),rd_u8(addr+1),rd_u8(addr+2),rd_u8(addr+3),rd_u8(addr+4),rd_u8(addr+5)];
    if !(cur == LANE_GATE_ORIG || cur == LANE_GATE_OFF || cur == LANE_GATE_ALL) {
        write_named("lane_gate.txt", &format!("ABORT cur={:02x?} (RVA mismatch?)\n", cur));
        return;
    }
    let target: &[u8; 6] = match want { 1 => &LANE_GATE_OFF, 2 => &LANE_GATE_ALL, _ => &LANE_GATE_ORIG };
    patch6(addr, target);
    LANE_GATE_APPLIED.store(want, Ordering::Relaxed);
    write_named("lane_gate.txt", &format!("lane_gate={} APPLIED @ {:#x} bytes={:02x?}\n", want, addr, target));
}

unsafe fn apply_type3_ablate() {
    let want = TYPE3_ABLATE.load(Ordering::Relaxed);
    if want == TYPE3_APPLIED.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 { return; }
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let sites = [base + T3_GATE_A_RVA, base + T3_GATE_B_RVA];
    // 안전검증: 둘째 바이트 0x5f, 첫바이트 want면 0x73(원본)·아니면 0xEB(패치)
    for &addr in sites.iter() {
        if !readable(addr, 2) { return; }
        let (b0, b1) = (rd_u8(addr), rd_u8(addr + 1));
        let ok = b1 == 0x5f && (if want { b0 == 0x73 } else { b0 == 0xEB });
        if !ok { write_named("type3_ablate.txt", &format!("ABORT @{:#x} {:02x}{:02x} want={} (RVA mismatch?)\n", addr, b0, b1, want)); return; }
    }
    let newb: u8 = if want { 0xEB } else { 0x73 };
    for &addr in sites.iter() {
        let mut old: u32 = 0;
        if VirtualProtect(addr, 1, 0x40, &mut old) == 0 { continue; }
        core::ptr::write_unaligned(addr as *mut u8, newb);
        VirtualProtect(addr, 1, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), addr, 1);
    }
    TYPE3_APPLIED.store(want, Ordering::Relaxed);
    write_named("type3_ablate.txt", &format!("type3_ablate={} APPLIED @ {:#x}/{:#x} (jae→jmp 차단)\n", want, sites[0], sites[1]));
}

unsafe fn build_call_stub(counter_addr: usize, join_addr: usize) -> usize {
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = stub_reg(VirtualAlloc(0, 64, MEM_CR, RWX), 64, 0xF004);
    if stub == 0 { return 0; }
    let mut s: Vec<u8> = Vec::new();
    s.push(0x51);                                          // push rcx
    s.extend_from_slice(&[0x48,0xb9]); s.extend_from_slice(&counter_addr.to_le_bytes());  // movabs rcx, &counter
    s.extend_from_slice(&[0xf0,0x48,0xff,0x01]);           // lock inc qword [rcx]
    s.push(0x59);                                          // pop rcx
    s.extend_from_slice(&[0xff,0x25,0x00,0x00,0x00,0x00]); // jmp qword [rip+0]
    s.extend_from_slice(&join_addr.to_le_bytes());         // 합류점 절대주소
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    stub
}

unsafe fn patch14(addr: usize, bytes: &[u8; 14]) {
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(addr, 14, RWX, &mut old) == 0 { return; }
    core::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, 14);
    VirtualProtect(addr, 14, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, 14);
}

unsafe fn apply_call_ablate() {
    let want = CALL_ABLATE.load(Ordering::Relaxed);
    if want == CALL_ABLATE_APPLIED.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 { return; }
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }  // 게임 안정 후에만
    let a_addr = base + CALL_PUSH_A_RVA;
    let b_addr = base + CALL_PUSH_B_RVA;
    if !readable(a_addr, 14) || !readable(b_addr, 14) { return; }
    // 안전검증: 켤때 원본(C6 04..) / 끌때 패치(FF 25..) 상태인지 확인 후에만 적용 (RVA 오류 크래시 방지)
    let (a0, a1, b0, b1) = (rd_u8(a_addr), rd_u8(a_addr + 1), rd_u8(b_addr), rd_u8(b_addr + 1));
    let chk = if want { a0==0xC6 && a1==0x04 && b0==0xC6 && b1==0x04 } else { a0==0xFF && a1==0x25 && b0==0xFF && b1==0x25 };
    if !chk { write_named("call_ablate.txt", &format!("ABORT A={:02x}{:02x} B={:02x}{:02x} want={} (RVA mismatch?)\n", a0,a1,b0,b1,want)); return; }
    if want {
        let sa = build_call_stub(&CALL_BLOCKED_A as *const _ as usize, base + CALL_JOIN_A_RVA);
        let sb = build_call_stub(&CALL_BLOCKED_B as *const _ as usize, base + CALL_JOIN_B_RVA);
        if sa == 0 || sb == 0 { write_named("call_ablate.txt", "ABORT stub alloc fail\n"); return; }
        let mut pa = [0u8; 14]; pa[0]=0xff; pa[1]=0x25; pa[6..14].copy_from_slice(&sa.to_le_bytes());
        let mut pb = [0u8; 14]; pb[0]=0xff; pb[1]=0x25; pb[6..14].copy_from_slice(&sb.to_le_bytes());
        patch14(a_addr, &pa); patch14(b_addr, &pb);
        write_named("call_ablate.txt", &format!("call_ablate=ON (콜차단+카운트) @ {:#x}/{:#x} stubs {:#x}/{:#x}\n", a_addr, b_addr, sa, sb));
    } else {
        patch14(a_addr, &CALL_ORIG_A); patch14(b_addr, &CALL_ORIG_B);
        write_named("call_ablate.txt", &format!("call_ablate=OFF (원본복원) @ {:#x}/{:#x}\n", a_addr, b_addr));
    }
    CALL_ABLATE_APPLIED.store(want, Ordering::Relaxed);
}

// ★★[#26] '원본값 일치' 가드 — 2026-08-04 신설.
//   prefix 만 보던 구조의 결함: **주소가 어긋났는데 prefix 가 우연히 같으면 패치가 "성공"으로 계상**된다.
//   실사고(08-03) = `ld_chase_stop` 2주소가 +0xb 어긋났는데 `48 c7 85` 가 우연히 일치해,
//   원본이 5인 슬롯에 15000 을 써 넣으면서도 applied 는 정상으로 나왔다. 기본 설정에서도 동작이 바뀌던 실버그.
//   막는 방법: 검증기에서 뽑은 (rva, imm_off, width, 원본값) 표를 들고, **그 사이트를 처음 건드릴 때**
//   현재 immediate 가 원본과 같은지 확인한다. 다르면 = 내가 아는 그 자리가 아니다 → 쓰지 않는다.
//   두 번째부터는 우리가 쓴 값이 들어 있는 게 정상이므로 검사하지 않는다(SEEN 비트).
#[path = "orig_table.rs"] mod orig_table;
static GUARD_SEEN: [AtomicU64; (orig_table::EXPECT_ORIG.len() + 63) / 64] =
    [const { AtomicU64::new(0) }; (orig_table::EXPECT_ORIG.len() + 63) / 64];
static GUARD_BLOCKED: AtomicU32 = AtomicU32::new(0);
static GUARD_CHECKED: AtomicU32 = AtomicU32::new(0);

/// 사이트를 처음 건드릴 때만 원본값을 대조한다. `true` = 써도 된다.
#[inline] unsafe fn orig_guard_ok(rva: u32, imm_off: usize, width: usize, site: usize) -> bool {
    let t = orig_table::EXPECT_ORIG;
    // rva 오름차순 → 이분탐색. 같은 rva 가 여러 폭으로 등록될 일은 없다.
    let mut lo = 0usize; let mut hi = t.len();
    while lo < hi {
        let m = (lo + hi) / 2;
        if t[m].0 < rva { lo = m + 1; } else { hi = m; }
    }
    if lo >= t.len() || t[lo].0 != rva { return true; }          // 표에 없는 사이트 = 기존 동작 유지
    let (_, off, w, orig) = t[lo];
    if off as usize != imm_off || w as usize != width { return true; }   // 다른 배선 = 판단 보류
    let (wi, bi) = (lo / 64, lo % 64);
    if GUARD_SEEN[wi].load(Ordering::Relaxed) & (1u64 << bi) != 0 { return true; }  // 이미 우리가 쓴 자리
    GUARD_CHECKED.fetch_add(1, Ordering::Relaxed);
    let mut cur = 0u64;
    core::ptr::copy_nonoverlapping(site as *const u8, &mut cur as *mut u64 as *mut u8, width);
    if cur != orig {
        GUARD_BLOCKED.fetch_add(1, Ordering::Relaxed);
        // ★`write_named` 를 쓰지 않는다 — 그건 cfg `log=1` 에서만 기록되는데,
        //   이건 진단이 아니라 **안전 경보**라 배포 상태(log=0)에서도 반드시 남아야 한다.
        if let Some(p) = pth("imm_guard.txt") {
            use std::io::Write as _;
            if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) {
                let _ = write!(f, "BLOCK rva={:#x} off={} w={} 기대원본={} 실제={}  ← 이 주소는 내가 아는 그 자리가 아니다\n",
                               rva, imm_off, width, orig, cur);
            }
        }
        return false;
    }
    GUARD_SEEN[wi].fetch_or(1u64 << bi, Ordering::Relaxed);
    true
}

/// 가드 결과 요약. `blocked>0` = **배선 주소가 틀렸다**는 뜻이므로 그 자리를 반드시 다시 확인할 것.
/// `checked` 는 "표에 등록된 사이트 중 처음 건드려 원본을 확인한 수"라 총 사이트 수와 같아야 정상.
unsafe fn write_guard_summary() {
    let ck = GUARD_CHECKED.load(Ordering::Relaxed);
    let bl = GUARD_BLOCKED.load(Ordering::Relaxed);
    if ck == 0 { return; }
    if let Some(p) = pth("imm_guard_summary.txt") {   // log 플래그와 무관하게 기록(안전 보고)
        let _ = fs::write(p, format!(
            "checked={}/{} blocked={}\n\
             (등록 사이트 전수 = {}. checked 가 이보다 작으면 그 배선이 아직 안 돌았거나 prefix 단계에서 걸린 것)\n\
             blocked>0 이면 그 배선 주소가 틀린 것이므로 반드시 확인할 것.\n",
            ck, orig_table::EXPECT_ORIG.len(), bl, orig_table::EXPECT_ORIG.len()));
    }
}

#[inline] unsafe fn patch_imm_bytes(addr: usize, prefix: &[u8], imm_off: usize, width: usize, val: u64) -> bool {
    if !readable(addr, imm_off + width) { return false; }
    for (i, &b) in prefix.iter().enumerate() {
        if rd_u8(addr + i) != b { return false; }   // opcode 불일치 = RVA 어긋남 → skip(크래시 방지)
    }
    let site = addr + imm_off;
    {   // ★원본값 가드(#26): prefix 우연 일치로 엉뚱한 자리를 덮어쓰는 것을 막는다.
        let base = exe_base();
        if base != 0 && addr >= base && addr - base <= u32::MAX as usize {
            if !orig_guard_ok((addr - base) as u32, imm_off, width, site) { return false; }
        }
    }
    let mut old: u32 = 0;
    if VirtualProtect(site, width, 0x40, &mut old) == 0 { return false; }
    let vb = val.to_le_bytes();
    core::ptr::copy_nonoverlapping(vb.as_ptr(), site as *mut u8, width);
    VirtualProtect(site, width, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), site, width);
    true
}

// ✅★[0.5.2 재핀 완료 = 아래 12사이트 전부 0.5.2 주소 (ghidra-re 2026-07-23 · exe 실바이트 대조 확인)]
//   ~~구 13사이트(0.5.1 주소·applied=0/13 무개입)~~ → **12사이트**. 13→12 내역:
//     · 신규 발견 3 = `dn_near_dist #3`(후보 루프 컬링) · `dn_nexus_hp #2`(32bit div 경로) · `an_finish_hp #2`(32bit div 경로)
//       ★#2들은 0.5.1에서도 존재했을 가능성이 크며 **한쪽만 패치하면 div 경로에 따라 임계가 갈린다** = 구 배선의 잠재 결함.
//     · ⛔0.5.2 삭제 2 = `dn_count_gate`(해당 위치가 `cmp rdx,[rbp-0x50]` 레지스터 비교로 대체=상수 소멸) ·
//       `dn_hp_crit #2`(두 사이트가 하나로 병합, 0x15 사이트는 컨테이너 내 1곳뿐)
//     · ⛔오식별 1 = **`an_count_gate`는 튜닝 레버가 아니라 컴파일러 bounds-check 관용구**였다 → **사이트 삭제**(아래 ★).
//   컨테이너 3종: dn-A = **`0x1b92e40`**(0x1b92e40~0x1b93569) / dn-B = **`0x1bdaaa0`**(0x1b934bc·0x1b934d6에서 호출) /
//     an = **`0x2376320`**(0x2376320~0x2377af1, 0.5.1 disc18 `0x1c7ca20`의 유일 후계).
//   컨테이너 확증 = imm 지문 유일성: dn-A는 `cmp rax,0x32`+`cmp qword[rbp±],0x1f`+`cmp qword[rbp±],0x15`를 **동시 만족하는 유일 함수** /
//     dn-B는 `0xd693a4001`이 **exe 전역 1건** / an은 컬링 `0x5f5e0` 2곳 중 `cmp rax,0x38`을 함께 가진 유일 함수.
//   프레임(dn-A): `[rbp-0x50]`=넥서스 maxHP(`[obj+0x610]`) · `[rbp-0x48]`=curHP(`[obj+0x658]`) · `[rbp-0x28]`=HP%(cur*100/max).
// ⛔★`an_count_gate` 영구 폐기(재핀 금지 · cfg `nx_an_count_gate` = DEAD): 구 사이트 `cmp qword[rbx+0x5b0],5`는
//   **배열 bounds-check 관용구**다(0.5.2에 동일 패턴 37곳, 예외없이 `cmp [X+0x5b0],N` → `lea 정적더미` → `cmovae 실원소` →
//   `cmp [reg+0x30],-1` 형태이며 imm=3/5가 항상 짝으로 등장 — 0.5.1에서 "짝구조 일치"를 강후보 근거로 삼았던 게 바로 이 지문이었다).
//   N을 바꾸면 없는 원소를 실포인터로 읽어 **OOB → 크래시/미정의**. 게다가 an 컨테이너(0x2376320) 안엔 이 패턴이 아예 없다.
// ⬜미해결(패치와 무관): 서브플랜 디스패처가 0.5.2에 최소 2개(`0x2134240` JT `0x38ae274` / `0x1dabcc0` JT `0x3842688`)이고
//   둘이 같은 idx에 다른 타깃을 준다 ⟹ **disc 번호 정본 재확인 필요**. 단 위 배선은 disc 번호가 아니라 **imm 지문 기반 함수 동정**이라 무영향.
unsafe fn apply_objective_imm() {
    // 게임 원본 imm값(0.5.0_2 실측): count 0x26 / nexus_hp 0x32 / hp_crit 0x15 / hp_low 0x1f
    //   / lane_margin 0x78 / an_gate 5 / near_dist 120000 / pred_dist 240000 (dist는 거리, 코드가 제곱).
    let enable = tune("nx_enable", 0) != 0;
    let cg = tune("nx_dn_count_gate", 0x26);
    let nh = tune("nx_dn_nexus_hp",   0x32);
    let hc = tune("nx_dn_hp_crit",    0x15);
    let hl = tune("nx_dn_hp_low",     0x1f);
    let lm = tune("nx_dn_vision_mem",0x78);
    let ag = tune("nx_an_count_gate",  5);
    let nd = tune("nx_dn_near_dist",  120000);
    let pd = tune("nx_dn_pred_dist",  240000);
    // ★[07-16 확증배선] disc18 핸들러 0x1c7ca20 내부 2사이트(ghidra-re DIFF 확정):
    //   fh = ★[08-05 감사 정정] **내 챔피언 HP%** 게이트(구 주석 "적 넥서스 HP%"는 오기 — `0xd95ddd`에서 r14=champions[myside][myrole]=나 확정)(0x1c7df47 cmp rax,0x38=56). ≥이값이면 아군 무관 즉시 마무리오더, 미만이면 아군2+도달 필요.
    //   cd = 넥서스공격 후보 거리컬링(0x1c7d5f9 cmp r8,0x5f5e0). (dist²>>14)>이값이면 그 아군 후보 스킵.
    let fh = tune("nx_an_finish_hp",  0x38);      // 56(%)
    let cd = tune("nx_an_cull_dist",  0x5f5e0);   // 390624 (dist²>>14 스케일 임계, ≈넥서스 2.5셀 반경)
    // 서명(enable+전값) — 변화 없으면 재패치 skip(핫패스 무부담)
    let mut sig = enable as u64;
    for v in [cg, nh, hc, hl, lm, ag, nd, pd, fh, cd] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == OBJIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    // 유효값: enable이면 cfg, 아니면 게임 원본(복원)
    let (cg, nh, hc, hl, lm, ag, nd, pd) = if enable { (cg, nh, hc, hl, lm, ag, nd, pd) }
        else { (0x26, 0x32, 0x15, 0x1f, 0x78, 5, 120000, 240000) };
    let (fh, cd) = if enable { (fh, cd) } else { (0x38, 0x5f5e0) };   // disc18 사이트 원본 복원값
    let b1 = |v: i64| (v.max(0).min(0x7f)) as u64;             // imm8 sign-safe clamp
    let u32c = |v: i64| (v.max(0) as u64) & 0xffff_ffff;       // imm32 클램프(부호·오버플로 안전)
    let sq  = |d: i64| { let d = d.max(0) as u64; d.wrapping_mul(d).wrapping_add(1) };   // dist²+1 (게임 인코딩)
    let sq0 = |d: i64| { let d = d.max(0) as u64; d.wrapping_mul(d) };                   // ★dist² (+1 없음 — near_dist #3 전용)
    let _ = (cg, ag);   // ⛔dn_count_gate=0.5.2 삭제 / an_count_gate=오식별 폐기 → 패치 사이트 없음(로그 표시용으로만 유지)
    let mut ok = 0u32;
    // ── dn 클러스터 A: 컨테이너 0x1b92e40 ──
    ok += patch_imm_bytes(base + 0xcd7054, &[0x48,0x83,0xf8], 3, 1, b1(nh)) as u32;              // dn_nexus_hp #1 orig 0x32 (64bit div 경로)
    ok += patch_imm_bytes(base + 0xcd7063, &[0x48,0x83,0xf8], 3, 1, b1(nh)) as u32;
    // ★0.5.4: nx_dn_nexus_hp 가 **2곳 → 4곳**으로 늘었다(신설 블록A 64/32bit).
    ok += patch_imm_bytes(base + 0xcd6ff1, &[0x48,0x83,0xf8], 3, 1, b1(nh)) as u32;
    ok += patch_imm_bytes(base + 0xcd7089, &[0x48,0x83,0xf8], 3, 1, b1(nh)) as u32;              // dn_nexus_hp #2 orig 0x32 (32bit div 경로) ★둘 다 필수
    ok += patch_imm_bytes(base + 0xcd712c, &[0x48,0x83,0x7d,0xe8], 4, 1, b1(hl)) as u32;         // dn_hp_low  orig 0x1f  (★0.5.3: 스택변위 [rbp-0x28]→**[rbp-0x30]** = prefix 마지막 d8→d0. 주소는 맞는데 이 prefix를 안 고쳐 8/12로 skip됐던 자리)
    ok += patch_imm_bytes(base + 0xcd713a, &[0x48,0x83,0x7d,0xe8], 4, 1, b1(hc)) as u32;         // dn_hp_crit orig 0x15  (★0.5.3: 변위 d8→d0, 위와 동일 사유. 0.5.2엔 1곳뿐=병합)
    ok += patch_imm_bytes(base + 0xcd6b74, &[0x48,0xb8], 2, 8, sq(nd)) as u32;                   // dn_near_dist #1 orig 0x35a4e9001 (=120000²+1)
    ok += patch_imm_bytes(base + 0xcd6ca0, &[0x48,0xb8], 2, 8, sq(nd)) as u32;                   // dn_near_dist #2 orig 0x35a4e9001
    ok += patch_imm_bytes(base + 0xcd6f08, &[0x49,0xba], 2, 8, sq0(nd)) as u32;                  // ★dn_near_dist #3 orig 0x35a4e9000 (**+1 없음** = sq0 / movabs r10 / 후보 루프 컬링)
    // ── dn 클러스터 B: 컨테이너 0x1bdaaa0 ──
    ok += patch_imm_bytes(base + 0xce3995, &[0x48,0xb8], 2, 8, sq(pd)) as u32;                   // dn_pred_dist  orig 0xd693a4001 (=240000²+1, exe 전역 유일)
    ok += patch_imm_bytes(base + 0xce3a03, &[0x49,0x83,0xc5], 3, 1, b1(lm)) as u32;              // dn_lane_margin orig 0x78 (★0.5.3: `add r14`→**`add r13`** = prefix c6→c5. 명령·의미 동일, 레지스터 배정만 바뀜)
    // ── an 클러스터: 컨테이너 0x2376320 (0.5.1 disc18 핸들러 0x1c7ca20 후계) ──
    // ★0.5.3 재구성(ghidra-re 07-29 + 실측): 0.5.2의 **단일 루프 1사이트**가 **3연속 루프 3사이트**로 분열했다.
    //   0.5.2 = 헬퍼(0x22c8a70)가 만든 Vec 1개를 1회 스캔 / 0.5.3 = [rbp+0x198]+idx*32 의 (ptr,len) **3쌍**을 인라인 체이닝,
    //   세 루프가 같은 본문(cmp byte[r12],0 → call 0xfcb660)으로 합류 = 논리적으로 같은 한 스캔 ⟹ **3사이트 전부 패치해야 커버리지 동일**.
    // ★★극성 반전: 0.5.2 `cmp r10,0x5f5e0; jbe(수락)` ⟺ 0.5.3 `cmp r8,0x5f5e1; jae(스킵)` — 같은 판정을 반대로 인코딩.
    //   ⟹ 임계값에 **+1** 을 실어야 의미가 같다(원본 복원값 0x5f5e0 → write 0x5f5e1 로 자동 정합).
    //   prefix 도 `49 81 fa`(cmp r10) → **`49 81 f8`(cmp r8)** 로 바뀌었다.
    let cd1 = u32c(cd).saturating_add(1);
    ok += patch_imm_bytes(base + 0xda2143, &[0x49,0x81,0xf8], 3, 4, cd1) as u32;                  // an_cull_dist #1 (리스트A)
    ok += patch_imm_bytes(base + 0xda21d3, &[0x49,0x81,0xf8], 3, 4, cd1) as u32;                  // an_cull_dist #2 (리스트B)
    ok += patch_imm_bytes(base + 0xda2257, &[0x49,0x81,0xf8], 3, 4, cd1) as u32;                  // an_cull_dist #3 (리스트C)
    ok += patch_imm_bytes(base + 0xda2c22, &[0x48,0x83,0xf8], 3, 1, b1(fh)) as u32;              // an_finish_hp #1 orig 0x38 (64bit div 경로)
    ok += patch_imm_bytes(base + 0xda2c2e, &[0x48,0x83,0xf8], 3, 1, b1(fh)) as u32;              // an_finish_hp #2 orig 0x38 (32bit div 경로) ★0.5.1은 #1만 패치=결함이었음
    OBJIMM_SIG.store(sig, Ordering::Relaxed);
    // ★LOG_ON 무관 직접 write(설치확증 — d19_imm.txt·itemnet_guard와 동일). write_named은 LOG_ON 게이트라 프로덕션서 미확인됐음.
    if let Some(p) = pth("obj_imm.txt") {
        let _ = fs::write(p, format!("nx_enable={} applied={}/16 cg={}=DEAD nh={} hc={} hl={} lm={} an={}=DEAD near={} pred={} fh={} cull={} @base{:#x}\n",
            enable, ok, cg, nh, hc, hl, lm, ag, nd, pd, fh, cd, base));
    }
}

// ★[07-16] vis_window 부활 byte-patch. 비-라인 상황(교전/운영/합류 전반) 적 시야기억창.
//   0.4.14 cand_filter의 단일 600창이 0.5.0서 판단핸들러들에 인라인 복제됐으나 window는 여전히 하드코딩 imm.
//   broad 600창 직계 후계 = 0x1caedd3 `add rsi, imm32`(imm@+3, 4B). 기본 600=원본 무변화.
//   (120창 imm8 8곳은 별건 — dd_lane_margin류 핸들러별 단창, 여기선 미개입.)
unsafe fn apply_vis_imm() {
    let vw = tune("vis_window", 600);
    let sig = vw as u64;
    if sig == VISIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let v = (vw.max(0) as u64) & 0xffff_ffff;   // imm32 클램프(0=즉시망각, 유저책임)
    // ★0.5.2(was 0.5.1 0x1caedd3). version-migrator: 컨테이너 한정 마스크시그 유일 + 사이트 12B 바이트 완전동일
    //   (`48 81 c6 58 02 00 00 48 39 c6 0f 93`). 신뢰=중상(컨테이너 매칭은 cos 기반 L4). prefix 3B 검증이 있어 어긋나면 skip.
    let ok = patch_imm_bytes(base + 0xc8c4e3, &[0x48,0x81,0xc6], 3, 4, v);   // add rsi, imm32
    VISIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(p) = pth("vis_imm.txt") {
        let _ = fs::write(p, format!("vis_window={} applied={}/1 @0xc8c4e3(0.5.3) @base{:#x}\n", vw, ok as u32, base));
    }
}

// ★★[08-03 신설] subplan별 개별 단기 시야창(120틱, imm8) byte-patch — vis_window(전역 600틱 공유 1사이트)와 별개 축.
//   ghidra-re 08-03 전수 스캔(0.5.3 exe 파일 디스어셈·.pdata 소속검증). 판별 기준 = `[ctx+idx*8+0x1e0]`(lastSeen 배열)
//   + 0x78 → `cmp vs curtick(vt+0x28)` 패턴만(오프셋/스트라이드/스택프레임 0x78은 전부 제외·제외표는 RE 결과 보존).
//   0.5.3 클론 분열로 **25사이트**(구 0.5.1 "imm8 8곳" 카운트는 무효 — 그 8은 sev[B] 스킬타이머 게이트였을 가능성).
//   그룹 6키(전부 기본 -1=원본 120 유지=무개입 / 단위 틱 / imm8 클램프 0~127 — 127틱≈2초가 상한).
//   ⚠재현(repl) 짝 동기 주의: vw_lane ↔ 재현측 dd_lane_margin / vw_check ↔ ec_vision_ticks — byte-patch는 게임측만
//     바꾸므로 대체 ON이면 재현측 키도 같은 값으로 맞춰야 game==mine 유지(대체 OFF·passthrough 경로는 게임측만 유효).
//   ⚠vw_check(공용 bool 헬퍼 0x12b6e20)는 disc0/1/3·disc4·disc12·disc14 공유 = subplan 단독 조절 불가(문서 명시).
//   ⚠disc17 시야창은 기배선 nx_dn_vision_mem(0xce3a03·disc17/19 공용 헬퍼) 소관 — 이중패치 금지로 여기서 제외.
//     ★그 사이트의 바이트 문맥이 본 패턴과 100% 동일 = "레인 마진" 라벨 오라벨 의혹(08-03 RE) — 시맨틱 재확정 전 라벨 유지.
unsafe fn apply_visshort_imm() {
    let lane   = tune("vw_lane", -1);    // disc0/1/3 라인전 컨테이너 인라인 5클론
    let jungle = tune("vw_jungle", -1);  // disc4 컨테이너 5사이트 (⚠disc4는 정규전 미발화 관측 — 참고용)
    let check  = tune("vw_check", -1);   // 공용 bool 헬퍼(라인·정글·모르가드12/14) 1사이트
    let nexus  = tune("vw_nexus", -1);   // disc18/19 공용 헬퍼 2사이트
    let threat = tune("vw_threat", -1);  // 위협평가 정본[A]·위협 컨텍스트 빌더[B] 헬퍼 2사이트
    let score  = tune("vw_score", -1);   // 후보 스코어링[E] 본체 5클론 + 헬퍼 5클론 = 10사이트
    let mut sig = 0u64;
    for v in [lane, jungle, check, nexus, threat, score] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == VISSHORT_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let enc = |v: i64| if v < 0 { 0x78u64 } else { (v.min(0x7f)) as u64 };   // -1=원본(120) 복원 / imm8 클램프
    let mut ok = 0u32;
    // ── lane: disc0/1/3 컨테이너(0xd803f0) — add rdi,imm8 ×5 ──
    for rva in [0xcd5c7eusize, 0xcd5d45, 0xcd5e07, 0xcd5ec9, 0xcd5f92] {
        ok += patch_imm_bytes(base + rva, &[0x48,0x83,0xc7], 3, 1, enc(lane)) as u32;
    }
    // ── jungle: disc4 컨테이너(0xd71630) — add rdi ×4 + add rsi ×1 ──
    for rva in [0xd67247usize, 0xd6730c, 0xd673cc, 0xd6748c] {
        ok += patch_imm_bytes(base + rva, &[0x48,0x83,0xc7], 3, 1, enc(jungle)) as u32;
    }
    ok += patch_imm_bytes(base + 0xd6754e, &[0x48,0x83,0xc6], 3, 1, enc(jungle)) as u32;
    // ── check: 공용 bool 헬퍼 0x12b6e20 — add rbx ──
    ok += patch_imm_bytes(base + 0xf3d658, &[0x48,0x83,0xc3], 3, 1, enc(check)) as u32;
    // ── nexus: 헬퍼 0xc8e4e0(add r13 — ⚠disc19 관찰 shadow-CALL 대상과 동일 함수=값 일관 자동 유지)·0xd10d00(add r14) ──
    ok += patch_imm_bytes(base + 0xdb6ade, &[0x49,0x83,0xc5], 3, 1, enc(nexus)) as u32;
    ok += patch_imm_bytes(base + 0xe23c83, &[0x49,0x83,0xc6], 3, 1, enc(nexus)) as u32;
    // ── threat: sev[A] 헬퍼 0xd36b00(add r12, 비트마스크형)·sev[B] 헬퍼 0xc4d6f0(add r15) ──
    ok += patch_imm_bytes(base + 0xc70ce2, &[0x49,0x83,0xc4], 3, 1, enc(threat)) as u32;
    ok += patch_imm_bytes(base + 0xc985d1, &[0x49,0x83,0xc7], 3, 1, enc(threat)) as u32;
    // ── score: sev[E] 본체(0xc7f640) add rsi ×5 + 헬퍼 0xcc8060 add rbx ×1·add rsi ×4 ──
    for rva in [0xd93b33usize, 0xd93c04, 0xd93cd1, 0xd93d9e, 0xd93e67] {
        ok += patch_imm_bytes(base + rva, &[0x48,0x83,0xc6], 3, 1, enc(score)) as u32;
    }
    ok += patch_imm_bytes(base + 0xcdd4ec, &[0x48,0x83,0xc3], 3, 1, enc(score)) as u32;
    for rva in [0xcdd635usize, 0xcdd76e, 0xcdd8c0, 0xcdd9e5] {
        ok += patch_imm_bytes(base + rva, &[0x48,0x83,0xc6], 3, 1, enc(score)) as u32;
    }
    VISSHORT_SIG.store(sig, Ordering::Relaxed);
    if let Some(p) = pth("visshort_imm.txt") {
        let _ = fs::write(p, format!("applied={}/25 lane={} jungle={} check={} nexus={} threat={} score={} (-1=원본120틱·상한127) @base{:#x}\n",
            ok, lane, jungle, check, nexus, threat, score, base));
    }
}

// ★★[08-03 신설] 라인개입(팀전술 jng=1) 갱 셋업 타이밍/게이트 byte-patch — "정글 부쉬 왕복" 대응 레버.
//   근거 = ghidra-re 08-03 2건(REPORT RE\2026-08-03_Strategy-소비처*·갱셋업-타이밍상수*): jng 유일 소비처 =
//   passive_jungle 업데이트 0xe00350, 왕복 기전(추정) = LineGankerPlan wait_limit 만료→취소→재시도 루프.
//   ⚠A(wait)는 imm이 아니라 **lea SIB 바이트의 스케일비트(상위 2비트)만 교체** — 초값은 F1×F2 조합 근사(2~72초).
//     F1(곱형) ∈ {2,3,5,9} / F2 = 사이트별 곱형 {2,3,5,9} 또는 덧셈형 {1,2,4,8}. 하위 6비트(레지스터)는 보존.
//   B(HP 게이트 base)는 jng 분기별 독립 3카피 중 **jng=1(라인개입) 카피만** 패치 = 라인개입 전용 조정.
//   전 키 기본 -1 = 원본 복원(무개입). ⚠전 사이트 인게임 미검증(정적 확정만) — 적용확인 = gank_imm.txt.
unsafe fn apply_gank_imm() {
    let wait = tune("gk_wait", -1);           // 부쉬 대기 timeout(초). -1=원본(사이트별 10/12/15/15/10초). 2~72초 조합 근사
    let hpb  = tune("gk_hp_base_gank", -1);   // 라인개입 리드액션 HP 게이트 base(원본 70, 실효임계=base−스탯/5). ↑=발동 억제
    let wm   = tune("gk_window_margin", -1);  // 갱 윈도우 최소 여유 배수(원본 5·허용 2/3/5/9). ↑=재시도 억제
    let mut sig = 0u64;
    for v in [wait, hpb, wm] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == GANKIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let mut ok = 0u32;
    // ── A. wait_limit 5사이트 (rva1=F1곱형, rva2=F2, form2: true=곱형/false=덧셈형) ──
    const F1: [i64; 4] = [2, 3, 5, 9];
    const F2M: [i64; 4] = [2, 3, 5, 9];
    const F2A: [i64; 4] = [1, 2, 4, 8];
    const A_SITES: [(usize, u8, [u8; 3], usize, u8, [u8; 3], bool); 5] = [
        (0xe642a5, 0x89, [0x48,0x8d,0x0c], 0xe642a9, 0x48, [0x48,0x8d,0x34], false), // A1 passive_jungle 10초
        (0xe64d88, 0x49, [0x48,0x8d,0x0c], 0xe64d8c, 0x88, [0x48,0x8d,0x34], false), // A2 passive_jungle 12초
        (0xe8e89e, 0x89, [0x48,0x8d,0x0c], 0xe8e8a2, 0x49, [0x48,0x8d,0x0c], true),  // A3 GankPlan 수락 15초
        (0xea1a10, 0x89, [0x48,0x8d,0x0c], 0xea1a14, 0x49, [0x48,0x8d,0x3c], true),  // A4 핸들러 15초
        (0xea20d5, 0x89, [0x48,0x8d,0x0c], 0xea20d9, 0x48, [0x48,0x8d,0x1c], false), // A5 핸들러 10초
    ];
    for (r1, o1, p1, r2, o2, p2, mul2) in A_SITES {
        let (b1v, b2v) = if wait >= 2 {
            // want에 가장 가까운 F1×F2 조합 선택(동률=작은 곱). 스케일비트만 교체.
            let f2set = if mul2 { F2M } else { F2A };
            let (mut bi, mut bj, mut bd) = (2usize, 1usize, i64::MAX);
            for (i, f1) in F1.iter().enumerate() {
                for (j, f2) in f2set.iter().enumerate() {
                    let d = (f1 * f2 - wait).abs();
                    if d < bd || (d == bd && f1 * f2 < F1[bi] * f2set[bj]) { bd = d; bi = i; bj = j; }
                }
            }
            (((o1 & 0x3f) | ((bi as u8) << 6)) as u64, ((o2 & 0x3f) | ((bj as u8) << 6)) as u64)
        } else { (o1 as u64, o2 as u64) };   // -1 = 원본 복원
        ok += patch_imm_bytes(base + r1, &p1, 3, 1, b1v) as u32;
        ok += patch_imm_bytes(base + r2, &p2, 3, 1, b2v) as u32;
    }
    // ── B. 라인개입(jng=1) 리드액션 HP 게이트 base (mov dl,imm8 @0xe63d92, 원본 0x46=70) ──
    let bv = if hpb >= 0 { hpb.min(100) as u64 } else { 0x46 };
    ok += patch_imm_bytes(base + 0xe63d92, &[0xb2], 1, 1, bv) as u32;
    // ── C. 갱 윈도우 여유 배수 3사이트 (lea SIB, 원본 0x80=×5 / 허용 2/3/5/9 → 최근접 매핑) ──
    let ci = if wm >= 0 { let mut bi = 2usize; let mut bd = i64::MAX;
        for (i, f) in F2M.iter().enumerate() { let d = (f - wm).abs(); if d < bd { bd = d; bi = i; } } bi } else { 2 };
    let cv = ((ci as u8) << 6) as u64;   // 하위 6비트 = 0x00 (orig 0x80 & 0x3f)
    for rva in [0xe63e26usize, 0xe63ecc, 0xe63ff5] {
        ok += patch_imm_bytes(base + rva, &[0x48,0x8d,0x04], 3, 1, cv) as u32;
    }
    GANKIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(p) = pth("gank_imm.txt") {
        let _ = fs::write(p, format!("applied={}/14 gk_wait={} hp_base={} window_margin={} (-1=원본: wait 10/12/15/15/10s·base70·margin x5) @base{:#x}\n",
            ok, wait, hpb, wm, base));
    }
}

// ★★[08-03 신설] sub_plan **실행층** byte-patch — 게임이 "실제 움직임"을 만드는 층.
//   근거 = RE\2026-08-03_라인전hot3핸들러-*.md + RE\2026-08-03_Order소비자-실행기규명-auction층-*.md.
//   ★이 층은 모드가 대체하지 않는다(모드 훅 = movepri `0xc559e0` arm) ⟹ **byte-patch가 그대로 유효**.
//
//   ① 판단력 오판 게이트 (`line_defense 0xc5e160` P10) — 게임의 "판단력 스탯" 구현 실체:
//        thr  = min(v,100) * 85 * 6554 >> 16 + 150     (≈ min(v,100)*8.5 + 150, 범위 150~1000)
//        roll = rng(0..1000)
//        if thr < roll && 후보수 >= 2 → **최선 후보 대신 무작위 1개로 교체**
//      ⟹ 판단력 100 → thr 1000 → 오판 0% / 판단력 0 → thr 150 → **오판 85%**.
//      cap(100)·slope(85)·floor(150)를 노출: floor↑ = 전반적 오판 감소, slope↑ = 판단력 영향력 증가.
//   ② 대기 위치 (`line_wait 0xd96a40`) — "밀고 나간 아군이 앵커에서 N 이상 멀면, 그 아군 경로의
//      **끝에서 M 뒤** 지점에서 대기". N(전환 임계, d²+1 인코딩)·M(뒤로 물러날 거리) 둘 다 노출.
//   ③ 오더 유지 최소 경과 (`lib.rs 0xd0a4fc`) — ↑ = 오더 재선정 억제 = "고집" 증가(우왕좌왕 완화 후보).
//   전 키 기본 -1 = 원본 복원(무개입). 적용확인 = exec_imm.txt.
unsafe fn apply_exec_imm() {
    let jcap = tune("ex_judge_cap", -1);     // 판단력 상한(원본 100)
    let jslp = tune("ex_judge_slope", -1);   // 판단력→문턱 기울기(원본 85 ≈ ×8.5)
    let jflr = tune("ex_judge_floor", -1);   // 문턱 하한(원본 150 = 최대 오판율 85%)
    // ⛔[08-07] 아래 둘은 lw_wait_dist / lw_back 의 **알리아스**가 됐다(중복 사이트 제거).
    //    tune() 호출을 남겨두면 sig 에 섞여 무의미한 재적용을 유발하므로 읽지 않는다.
    let hold = tune("ex_order_hold", -1);    // 오더 유지 최소 경과 틱(원본 10)
    let mut sig = 0u64;
    for v in [jcap, jslp, jflr, hold] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == EXECIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let sq1 = |d: i64| { let d = d.max(0) as u64; d.wrapping_mul(d).wrapping_add(1) };
    let mut ok = 0u32;
    // ── ① 판단력 오판 게이트 (line_defense) ──
    ok += patch_imm_bytes(base + 0xd792e7, &[0x48,0x83,0xfa], 3, 1, b1(jcap, 100)) as u32;   // cmp rdx,100
    ok += patch_imm_bytes(base + 0xd792f4, &[0x6b,0xc0], 2, 1, b1(jslp, 85)) as u32;         // imul eax,eax,85
    ok += patch_imm_bytes(base + 0xd79303, &[0x05], 1, 4, b4(jflr, 150)) as u32;             // add eax,150
    // ── ② 대기 위치 (line_wait) ──
    // ⛔[08-07 중복 제거] 이 두 사이트(0xe721d3 · 0xe727c4)는 **apply_score_imm 의 lw_wait_dist / lw_back
    //    과 같은 주소**였다. 두 묶음이 같은 바이트를 각각 패치해 **나중 것이 이기는** 상태였고,
    //    한쪽 값을 바꿔도 다른 쪽이 덮으면 조용히 무효가 됐다(applied=N/N 이라 지표로 안 드러남).
    //    ⟹ lw_* 를 정본으로 두고 여기선 패치하지 않는다. ex_wait_* 는 알리아스로 계속 동작한다.
    // ── ③ 오더 유지 최소 경과 (lib.rs 유닛 AI 틱) ──
    // ★[08-07] 마이크로 디투어와 상호배타.
    if !micro_taken("ex_order_hold") {
    ok += patch_imm_bytes(base + 0xe747e3, &[0x48,0x83,0xc0], 3, 1, b1(hold, 10)) as u32;    // add rax,10
    }
    EXECIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(p) = pth("exec_imm.txt") {
        // ⛔[08-07] wait=[dist back] 제거 — 그 두 사이트(0xe721d3·0xe727c4)는 lw_wait_dist / lw_back
        //   소관으로 일원화됐다(중복 패치였다). 적용 수도 6 → 4.
        let _ = fs::write(p, format!("applied={}/4 judge=[cap{} slope{} floor{}] order_hold={} (-1=원본: 100/85/150/10) @base{:#x}\n",
            ok, jcap, jslp, jflr, hold, base));
    }
}

// ★[08-03] 시전 후보 생성기(`0xcb3ab0` = battle.rs:1223) + 행동 실행층(`0xd945a0` 계열) byte-patch.
//   근거 = RE\2026-08-03_0xcb3ab0-공격스킬후보생성기-0.5.3.md §④
//        + RE\2026-08-03_SmallAction-소비자-태그20종-실명확정-0.5.3.md §5
//   ★유효성: 두 층 모두 모드가 대체하지 않음 ⟹ byte-patch가 실제로 먹는다.
//   ⚠전 키 기본 -1 = 원본(무개입). 적용 결과는 cast_imm.txt.
//
//   ① 시전 후보(cs_*) — "무엇을 향해 평타/스킬을 쓸 후보로 올릴지".
//      사거리 판정 = dist² <= (사거리 + **선행예측틱 × 접근속도** + 보정)² 이므로,
//      선행예측 틱을 올리면 "곧 닿을 것"으로 보고 더 먼 거리에서 공격을 시작한다.
//   ② 실행층(ex_*) — 고른 행동을 실제 입력으로 바꾸는 단계.
//      ★★skill2/ult **해금 레벨이 데이터가 아니라 코드 하드코딩**이라 여기서만 바꿀 수 있다.
//      ⚠낮추면 슬롯이 비어 있는 챔프(스킬 1개짜리 등)에서 id==-1 → **게임 패닉**. 반드시 원본(-1) 유지 권장.
unsafe fn apply_cast_imm() {
    let la   = tune("cs_lead_attack", -1);    // 평타 선행예측 틱(원본 30, 2곳)
    let ls   = tune("cs_lead_skill", -1);     // 스킬 선행예측 틱(원본 30)
    let ls2  = tune("cs_lead_skill2", -1);    // 스킬2 선행예측 틱(원본 30)
    let lst  = tune("cs_lead_steal", -1);     // 막타/스틸 선행예측 틱(원본 30)
    let lu   = tune("cs_lead_ult", -1);       // 궁 선행예측 틱(원본 60, 2곳)
    let ur   = tune("cs_ult_range", -1);      // 궁 사용 허용 반경(원본 6000, d² 인코딩, 2곳)
    let urg  = tune("cs_ult_range_global", -1); // 글로벌 궁 반경(원본 90000, d²)
    let umk  = tune("cs_ult_mode_mask", -1);  // 궁 근접요구를 적용할 팀모드 비트마스크(원본 0x6f)
    let shp  = tune("cs_steal_hp", -1);       // 중립몹 막타 시도 HP%(원본 20, 2곳)
    let uh   = tune("cs_unit_hits", -1);      // 적 유닛 공격 허용: hp/dmg <= N (원본 2 = 3방컷, 2곳)
    let ahp  = tune("cs_ally_hp", -1);        // 아군 지원 스킬 HP% 경계(원본 79, 2곳)
    let arad = tune("cs_ally_radius", -1);    // 아군 지원 밀집 판정 반경(원본 120000, d² 8곳)
    let mvis = tune("cs_minion_vision", -1);  // 미니언 시야 기억 틱(원본 120)
    let ccm  = tune("cs_cc_mask", -1);        // "이동 가능" 상태이상 마스크(원본 0x3B8, 4곳)
    let sk2l = tune("ex_skill2_level", -1);   // ★스킬2 해금 레벨(원본 3)
    let ultl = tune("ex_ult_level", -1);      // ★궁 해금 레벨(원본 5)
    let am   = tune("ex_attack_margin", -1);  // 평타 접근 여유(원본 15000)
    let ams  = tune("ex_attack_margin_sp", -1); // 특수 대상용 접근 여유(원본 2000)
    let asek = tune("ex_attack_seek", -1);    // 평타 대상 탐색 사거리 배율 base %(원본 100)
    let fmt  = tune("ex_fail_min_ticks", -1); // 실패 오더 기록 최소 지속틱(원본 119)
    let tmin = tune("ex_think_min", -1);      // 재판단 간격 하한 base(원본 400)
    let tmax = tune("ex_think_max", -1);      // 재판단 간격 상한 base(원본 800)
    let mut sig = 0u64;
    for v in [la, ls, ls2, lst, lu, ur, urg, umk, shp, uh, ahp, arad, mvis, ccm,
              sk2l, ultl, am, ams, asek, fmt, tmin, tmax] {
        sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64);
    }
    if sig == CASTIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let sq  = |v: i64, orig: u64| if v < 0 { orig } else { let d = v.max(0) as u64; d.wrapping_mul(d) };
    let sqp = |v: i64, orig: u64| if v < 0 { orig } else { let d = v.max(0) as u64; d.wrapping_mul(d).wrapping_add(1) };
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }
    macro_rules! pskip { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        // ⛔0.5.4 재배치 미확정 — 세기만 하고 **패치하지 않는다**.
        //    (orig_guard 는 표에 없는 RVA 를 통과시키므로, 옛 주소를 남기면 오패치 위험)
        tot += 1; let _ = ($a, $pre, $off, $w, $v);
    }}; }
    // prefix가 사이트마다 다른 경우(레지스터만 다른 같은 명령) — 후보를 순서대로 시도.
    macro_rules! pany { ($a:expr, $pres:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1;
        let mut done = false;
        for pre in $pres.iter() { if !done && patch_imm_bytes($a, pre, $off, $w, $v) { done = true; } }
        ok += done as u32;
    }}; }

    // ── ① 시전 후보 생성기 (0xcb3ab0) ──
    // ★[08-07] 마이크로 디투어가 이 자리를 가져갔으면 건드리지 않는다(상호배타 — 창 5B 를 통째로 바꿨다).
    if !micro_taken("cs_lead_attack") {
    p!(base + 0xdb869a, &[0xb8], 1, 4, b4(la, 30));   // ←0.5.3 cb3efd  ★재조사로 복구: lead 30 (053 5곳→054 2곳 통합, 나머지 4키는 보류)  ★41bc(6B)→b8(5B) 로 인코딩 축소 ⟹ imm_off 2→1
    }
    pskip!(base + 0xcb440a, &[0xb9],      1, 4, b4(la, 30));   // ⛔0.5.4 미확정: 시그 2→0 / 완화 4→2 (골격 99%)
    pskip!(base + 0xcb4781, &[0xb9],      1, 4, b4(ls, 30));   // ⛔0.5.4 미확정: 시그 2→0 / 완화 4→2 (골격 99%)
    pskip!(base + 0xcb5a7d, &[0xbb],      1, 4, b4(ls2, 30));   // ⛔0.5.4 미확정: 시그 1→0 / 완화 4→2 (골격 99%)
    pskip!(base + 0xcb6936, &[0xba],      1, 4, b4(lst, 30));   // ⛔0.5.4 미확정: 시그 1→0 / 완화 4→2 (골격 99%)
    p!(base + 0xdbcbba, &[0x48,0x6b,0x85,0x60,0x02,0x00,0x00], 7, 1, b1(lu, 60));   // ←0.5.3 cb8036
    p!(base + 0xdbcd67, &[0x48,0x6b,0x86,0x28,0x06,0x00,0x00], 7, 1, b1(lu, 60));   // ←0.5.3 cb81d7
    p!(base + 0xdbc645, &[0xb9],      1, 4, sq(ur, 36_000_000));          // 6000²   // ←0.5.3 cb7ac9
    p!(base + 0xdbc65d, &[0xb8],      1, 4, sq(ur, 36_000_000));   // ←0.5.3 cb7ae1
    p!(base + 0xdbc662, &[0x48,0xb9], 2, 8, sq(urg, 8_100_000_000));      // 90000²   // ←0.5.3 cb7ae6
    p!(base + 0xdbc77a, &[0xb9],      1, 4, b4(umk, 0x6f));   // ←0.5.3 cb7bfa
    for a in [0xdbb5d7usize, 0xdbb5e7] { p!(base + a, &[0x48,0x83,0xf8], 3, 1, b1(shp, 20)); }
    // ↓0.5.4: prefix 가 사이트마다 달라져 루프를 펼침(원래 `for a in [..]`)
    p!(base + 0xdb86b7, &[0x48,0x83,0xfa], 3, 1, b1(uh, 2));   // ←0.5.3 cb4317
    p!(base + 0xdb8f07, &[0x48,0x83,0xf8], 3, 1, b1(uh, 2));   // ←0.5.3 cb4331
    for a in [0xdb9ab6usize, 0xdbad76] { p!(base + a, &[0x48,0x83,0xf8], 3, 1, b1(ahp, 79)); }
    // ↓0.5.4: prefix 가 사이트마다 달라져 루프를 펼침(원래 `for a in [..]`)
    p!(base + 0xdb996f, &[0x48,0xb9], 2, 8, sqp(arad, 14_400_000_001));   // ←0.5.3 cb4d9f
    p!(base + 0xdb99e2, &[0x49,0xb9], 2, 8, sqp(arad, 14_400_000_001));   // ←0.5.3 cb4e12
    p!(base + 0xdb9ba2, &[0x49,0xba], 2, 8, sqp(arad, 14_400_000_001));   // ←0.5.3 cb4fd2
    p!(base + 0xdbac40, &[0x48,0xb9], 2, 8, sqp(arad, 14_400_000_001));   // ←0.5.3 cb6080
    p!(base + 0xdbacab, &[0x49,0xb9], 2, 8, sqp(arad, 14_400_000_001));   // ←0.5.3 cb60eb
    p!(base + 0xdbae5b, &[0x49,0xba], 2, 8, sqp(arad, 14_400_000_001));   // ←0.5.3 cb629b
    for a in [0xdb9a6ausize, 0xdbad2a] {                                           // 120000²
        pany!(base + a, [[0x49,0xbb],[0x48,0xbb],[0x49,0xb9],[0x48,0xb9]], 2, 8, sq(arad, 14_400_000_000));
    }
    p!(base + 0xdbc0ea, &[0x49,0x83,0xc4], 3, 1, b1(mvis, 120));   // ←0.5.3 cb756b
    // ↓0.5.4: prefix 가 사이트마다 달라져 루프를 펼침(원래 `for a in [..]`)
    p!(base + 0xdb8af7, &[0x41,0xbd], 2, 4, b4(ccm, 0x3B8));   // ←0.5.3 cb3f07
    p!(base + 0xdb9444, &[0x41,0xb9], 2, 4, b4(ccm, 0x3B8));   // ←0.5.3 cb4874
    p!(base + 0xdba640, &[0x41,0xbe], 2, 4, b4(ccm, 0x3B8));   // ←0.5.3 cb5a86
    p!(base + 0xdbc834, &[0x41,0xb9], 2, 4, b4(ccm, 0x3B8));   // ←0.5.3 cb7cb4
    // ── ② 행동 실행층 ──
    p!(base + 0xca6849, &[0x49,0x83,0xbe,0xb0,0x05,0x00,0x00], 7, 1, b1(sk2l, 3));   // ←0.5.3 cc3489
    p!(base + 0xca6a79, &[0x49,0x83,0xbe,0xb0,0x05,0x00,0x00], 7, 1, b1(ultl, 5));   // ←0.5.3 cc36b9
    p!(base + 0xe5a1c2, &[0xba], 1, 4, b4(am, 15000));   // ←0.5.3 c87c76
    p!(base + 0xe5a1bd, &[0xb8], 1, 4, b4(ams, 2000));   // ←0.5.3 c87c71
    p!(base + 0xe59fc1, &[0x83,0xc2], 2, 1, b1(asek, 100));   // ←0.5.3 c87a81
    p!(base + 0xe753af, &[0x48,0x83,0xf9], 3, 1, b1(fmt, 119));   // ←0.5.3 d0a85b
    // ★0.5.4: 명령 모양이 바뀌었다. 053 `add rcx,0x190`(7B, imm off 3)
    //   → 054 `lea rcx,[rax+rax*2+0x190]`(8B, imm off **4**). 앞 명령의 ×3 SIB 가 합쳐진 것.
    if !micro_taken("ex_think_min") {   // ★[08-07] 마이크로 디투어와 상호배타
    p!(base + 0xe76cb0, &[0x48,0x8d,0x8c,0x40], 4, 4, b4(tmin, 400));   // ←0.5.3 d0cb6b
    }
    p!(base + 0xe76cba, &[0x48,0x8d,0x04,0x85], 4, 4, b4(tmax, 800));   // ←0.5.3 d0cb74

    CASTIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("cast_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} lead[atk{} skill{} skill2{} steal{} ult{}] ult[r{} rg{} mask{}] \
             steal_hp{} unit_hits{} ally[hp{} rad{}] minion_vis{} cc_mask{} | \
             exec[sk2lv{} ultlv{} atk_margin{} atk_margin_sp{} atk_seek{} fail_min{} think{}~{}]\n\
             (-1=원본: 30/30/30/30/60 · 6000/90000/0x6f · 20 · 2 · 79/120000 · 120 · 0x3B8 | 3/5/15000/2000/100/119/400/800) @base{:#x}\n",
            ok, tot, la, ls, ls2, lst, lu, ur, urg, umk, shp, uh, ahp, arad, mvis, ccm,
            sk2l, ultl, am, ams, asek, fmt, tmin, tmax, base));
    }
}

// ★★[08-03] 행동 점수 엔진(`action_score.rs`) + 이동 실행층 byte-patch.
//   근거 = RE\2026-08-03_action_score-점수엔진-수적우세배율-0.5.3.md
//        + RE\2026-08-03_이동오더생산자-line_wait-line_safe-0.5.3.md
//        + RE\2026-08-03_SmallAction-variant-페이로드-필드명-전수-0.5.3.md
//   ★유효성: 경매·점수·이동 실행층 모두 모드 미대체 구간 ⟹ byte-patch 유효.
//
//   ★★핵심 = **국지 수적 배율**. 점수에 마지막으로 곱하는 배율이 주변 머릿수 차 n으로 정해진다.
//   ★★[08-03 정정] n의 **부호가 반대**였다 — 실측 `n = 적 수(반경 150,000, 시야조건 有)
//      − 아군 수(반경 100,000, 시야조건 無)`. 근거 = 적 루프가 `1−myTeam` 배열(`c7ccb6`)·반경
//      `0x53d1ac100`(150,000²), 아군 루프가 `myTeam` 배열(`c7d142`)·반경 `0x2540be401`(100,000²),
//      합산이 `c7d2fd sub rbx, r8`(적−아군). ⟹ **n이 클수록 "적이 많다"**는 뜻이다.
//      (RE\2026-08-03_이동계점수-c7b730-cat0-cat2-cat4-0.5.3.md §0 정정①)
//      cat4(Trace)에선 적이 많을수록 추적 가치↑, cat0(도주)에선 적이 많을수록 도주 가치↑로 읽힌다.
//   ★★[08-03 정정②] 배율 테이블이 **cat별로 2종**이다(하나가 아님):
//      cat4(Trace)   = 30 / 60 / 80 / 150 / 200  ← 아래 키들이 건드리는 것
//      cat0(도주·귀환) = 40 / 75 / 100 / 200 / 300 (`c7c7be`·`c7c7ae`·`.rdata 0x31AA4E8/4F0/4F8`)
//      cat2(접근)    = 배율 없음
//      ⟹ 아래 `sc_adv_*`는 **cat4 전용**이다. cat0 테이블은 아직 미노출(TODO).
//   ⚠−1/0/+1 세 값은 코드가 아니라 **.rdata 테이블**(`0x31AA500/508/510`)이라 prefix 검증이 불가능하다.
//     → 최초 1회 "정말 60/80/150인지" 확인하고, 아니면 **세 개 다 건드리지 않는다**(RVA 어긋남 방어).
unsafe fn apply_score_imm() {
    // ⚠키 이름의 hi/lo는 **n의 부호** 기준이며, n = 적−아군이다(위 정정① 참조).
    let ahi  = tune("sc_adv_hi", -1);       // n≥+2 = 적이 2명 이상 많을 때 배율%(원본 200)
    let alo  = tune("sc_adv_lo", -1);       // n≤−2 = 아군이 2명 이상 많을 때 배율%(원본 30)
    let am1  = tune("sc_adv_m1", -1);       // n=−1 배율%(원본 60)
    let a0   = tune("sc_adv_0", -1);        // n=0  배율%(원본 80)
    let ap1  = tune("sc_adv_p1", -1);       // n=+1 배율%(원본 150)
    // ⚠키 이름이 실제와 **반대**다(호환 위해 이름은 유지). 150,000 = **적** 세는 반경,
    //   100,000 = **아군** 세는 반경. 편집기 라벨은 실제 의미로 표기한다.
    let arad = tune("sc_ally_radius", -1);  // 실제로는 **적**을 세는 반경(원본 150000)
    let erad = tune("sc_enemy_radius", -1); // 실제로는 **아군**을 세는 반경(원본 100000)
    let nbon = tune("sc_near_bonus", -1);   // 근접 보너스(원본 10)
    let obon = tune("sc_obj_bonus", -1);    // 오브젝트 확인 판단 보너스(원본 10)
    // ⚠이 키만 **원본이 음수**(−30)라, 다른 키처럼 "음수 = 원본"으로 볼 수 없다.
    //   → 전용 센티널 −9999 이하만 "원본", 그 외는 값 그대로 적용한다.
    let rthr = tune("sc_keep_thr", -9999);  // 라인 수비 후보 유지 점수 하한(원본 −30)
    let rthr_orig = rthr <= -1000;
    let lwd  = tune("lw_wait_dist", -1);    // 대기↔전진 전환 거리(원본 180000)
    let lwb  = tune("lw_back", -1);         // 경로 끝에서 물러날 거리(원본 180000)
    let lwr  = tune("lw_radius", -1);       // 대기 중 배회 반경(원본 80000, 3곳)
    let lsr  = tune("ls_radius", -1);       // 라인 안전 배회 반경(원본 80000, 2곳)
    let mvb  = tune("mv_bush_arrive", -1);  // 수풀 도착 판정 반경(원본 16000)
    let mvh  = tune("mv_hide_near", -1);    // 은신 근접 판정 거리(원본 12000)
    let mvt  = tune("mv_trace_dist", -1);   // 추격 거리 임계(원본 120000)
    let mut sig = 0u64;
    for v in [ahi, alo, am1, a0, ap1, arad, erad, nbon, obon, rthr,
              lwd, lwb, lwr, lsr, mvb, mvh, mvt] {
        sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64);
    }
    if sig == SCOREIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b4  = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let sq  = |v: i64, orig: u64| if v < 0 { orig } else { let d = v.max(0) as u64; d.wrapping_mul(d) };
    let sqp = |v: i64, orig: u64| if v < 0 { orig } else { let d = v.max(0) as u64; d.wrapping_mul(d).wrapping_add(1) };
    // 음수 i8 (점수 하한). -128..=127 로 클램프 후 2의 보수 바이트.
    let s1  = |x: i64| ((x.max(-128).min(127)) as i8) as u8 as u64;
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }
    macro_rules! pskip { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        // ⛔0.5.4 재배치 미확정 — 세기만 하고 **패치하지 않는다**.
        //    (orig_guard 는 표에 없는 RVA 를 통과시키므로, 옛 주소를 남기면 오패치 위험)
        tot += 1; let _ = ($a, $pre, $off, $w, $v);
    }}; }

    // ── ① 수적우세 배율 (코드 즉치 2개) ──
    p!(base + 0xd8fd14, &[0xb9], 1, 4, b4(ahi, 200));   // ←0.5.3 c7d300
    p!(base + 0xd8fd23, &[0xb9], 1, 4, b4(alo, 30));   // ←0.5.3 c7d30f
    // ── ①-b 수적우세 배율 (.rdata 테이블 3개) ──
    //    prefix 검증이 불가능한 데이터 영역이라, 원본 3값이 그대로인지 최초 확인 후에만 건드린다.
    {
        let t = base + 0x31AA500;
        // rd_u64 는 Option<u64> 를 돌려준다(읽기 실패 = None).
        let cur = (readable(t, 24) && rd_u64(t) == Some(60)
                   && rd_u64(t + 8) == Some(80) && rd_u64(t + 16) == Some(150))
                  || RDATA_ADV_OK.load(Ordering::Relaxed);
        if cur {
            RDATA_ADV_OK.store(true, Ordering::Relaxed);
            for (off, v, orig) in [(0usize, am1, 60u64), (8, a0, 80), (16, ap1, 150)] {
                tot += 1;
                let want = if v < 0 { orig } else { v.max(0) as u64 };
                let mut old: u32 = 0;
                if VirtualProtect(t + off, 8, 0x04, &mut old) != 0 {
                    core::ptr::write_unaligned((t + off) as *mut u64, want);
                    VirtualProtect(t + off, 8, old, &mut old);
                    ok += 1;
                }
            }
        } else {
            tot += 3;   // 주소 어긋남 → 3개 모두 미적용으로 계상(로그에 드러나게)
        }
    }
    // ── ② 인식 반경·보너스 ──
    p!(base + 0xd8f8d3, &[0x48,0xbe], 2, 8, sq(arad, 22_500_000_000));      // 150000²   // ←0.5.3 c7ccde
    for a in [0xd8ee73usize, 0xd8fb96] {
        p!(base + a, &[0x48,0xba], 2, 8, sqp(erad, 10_000_000_000));        // 100000²+1
    }
    p!(base + 0xd8ee80, &[0xbb], 1, 4, b4(nbon, 10));   // ←0.5.3 c7d5a6
    p!(base + 0xe55d65, &[0xb8], 1, 4, b4(obon, 10));   // ←0.5.3 d9bac1
    // ── ③ 라인 수비 후보 유지 하한 (두 인코딩이 한 임계를 이룸: N, N−1) ──
    p!(base + 0xc865aa, &[0x48,0x83,0xf8], 3, 1, s1(if rthr_orig { -30 } else { rthr }));   // ←0.5.3 c3cf8d
    p!(base + 0xc865b9, &[0x48,0x83,0xf8], 3, 1, s1(if rthr_orig { -31 } else { rthr - 1 }));   // ←0.5.3 c3cf9e
    // ── ④ 대기(line_wait) / 라인 안전(line_safe) ──
    p!(base + 0xe721d3, &[0x48,0xb9], 2, 8, sqp(lwd, 32_400_000_001));      // 180000²+1   // ←0.5.3 d971b6
    p!(base + 0xe727c4, &[0x48,0x2d], 2, 4, b4(lwb, 180_000));   // ←0.5.3 d974c9
    for a in [0xe72141usize, 0xe72358, 0xe72db4] {
        p!(base + a, &[0x48,0xc7,0x85], 7, 4, b4(lwr, 80_000));
    }
    for a in [0xe70a25usize, 0xe70b05] {
        p!(base + a, &[0x48,0xc7,0x85], 7, 4, b4(lsr, 80_000));
    }
    // ── ⑤ 이동 실행층 ──
    p!(base + 0xe077c3, &[0x48,0x81,0xfa], 3, 4, sqp(mvb, 256_000_001));    // 16000²+1   // ←0.5.3 d87f22
    p!(base + 0xe0d089, &[0x48,0x81,0xfa], 3, 4, sq(mvh, 144_000_000));     // 12000²   // ←0.5.3 d89ad0
    p!(base + 0xca8132, &[0x48,0xb9], 2, 8, sqp(mvt, 14_400_000_001));   // ←0.5.3 c7b4a5  ★재조사로 복구: mvt ★자동짝 de0680은 오답이었다

    SCOREIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("score_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} adv[{} {} {} {} {}] radius[ally{} enemy{}] bonus[near{} obj{}] keep_thr{} | \
             wait[dist{} back{} r{}] safe[r{}] move[bush{} hide{} trace{}]\n\
             (-1=원본: 200/30/60/80/150 · 150000/100000 · 10/10 · -30 · 180000/180000/80000 · 80000 · 16000/12000/120000) @base{:#x}\n",
            ok, tot, ahi, alo, am1, a0, ap1, arad, erad, nbon, obon, rthr,
            lwd, lwb, lwr, lsr, mvb, mvh, mvt, base));
    }
}

// ★★[08-03] 전투행동 점수 공식(`0xc7f640` = action_score.rs 974~1452) byte-patch.
//   근거 = RE\2026-08-03_전투행동-점수공식-c7f640-c7d7e0-0.5.3.md
//   점수 = ①아군포탑지원(+, 최대100) + ②자기위험비용(−) + ③셀위협비용(−) + ④본체가치
//   ★유효성: 모드 미대체 구간 ⟹ byte-patch 유효.
unsafe fn apply_score2_imm() {
    let trad = tune("sc_turret_radius", -1);  // 아군/적 구조물 인식 반경(원본 150000, 13곳)
    let erad2= tune("sc_engage_radius", -1);  // 적 챔피언 "근접" 인식 반경(원본 약 122474, 5곳)
    let cdst = tune("sc_cell_dist", -1);      // 이 거리 초과일 때만 셀 위협 페널티(원본 35000)
    let dvm  = tune("sc_dive_margin", -1);    // 적 포탑 위협 사거리 여유분(원본 15000)
    let rd0  = tune("sc_risk_dmg", -1);       // 위험판정 기본 피해%(원본 49)
    let rh1  = tune("sc_risk_hp1", -1);       // 1단 체력 경계(원본 65 → HP%<66)
    let rd1  = tune("sc_risk_dmg1", -1);      // 1단 피해%(원본 29)
    let rh2  = tune("sc_risk_hp2", -1);       // 2단 체력 경계(원본 40)
    let rd2  = tune("sc_risk_dmg2", -1);      // 2단 피해%(원본 17)
    let rh3  = tune("sc_risk_hp3", -1);       // 3단 체력 경계(원본 25)
    let rd3  = tune("sc_risk_dmg3", -1);      // 3단 피해%(원본 10)
    let fcap = tune("sc_focus_cap", -1);      // 집중포화 보너스 상한(원본 80)
    let kcap = tune("sc_kill_cap", -1);       // 처치각 보너스 상한(원본 80)
    let kpct = tune("sc_kill_pct", -1);       // 총딜 대비 부분 처치각 기준%(원본 60)
    let svis = tune("sc_score_vision", -1);   // 점수 계산의 적 "최근 목격" 유효틱(원본 120, 5곳)
    // ⚠원본이 음수(−10) — 전용 센티널
    let nul  = tune("sc_null_score", -9999);
    let nul_orig = nul <= -1000;
    let mut sig = 0u64;
    for v in [trad, erad2, cdst, dvm, rd0, rh1, rd1, rh2, rd2, rh3, rd3,
              fcap, kcap, kpct, svis, nul] {
        sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64);
    }
    if sig == SCORE2_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    // 거리 → (d²>>shift) 인코딩
    let dsh = |v: i64, orig: u64, sh: u32| if v < 0 { orig } else {
        let d = v.max(0) as u64; (d.wrapping_mul(d)) >> sh
    };
    let sqp = |v: i64, orig: u64| if v < 0 { orig } else { let d = v.max(0) as u64; d.wrapping_mul(d).wrapping_add(1) };
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! pany { ($a:expr, $pres:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1;
        let mut done = false;
        for pre in $pres.iter() { if !done && patch_imm_bytes($a, pre, $off, $w, $v) { done = true; } }
        ok += done as u32;
    }}; }
    // ★0.5.4 재작성으로 이 함수에도 단일 prefix 사이트가 생겨 `p!` 가 필요해졌다
    //   (루프를 펼치면서 사이트별 prefix 가 갈렸다).
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }
    static CMP3: [[u8;3];4] = [[0x48,0x81,0xfe],[0x49,0x81,0xfa],[0x48,0x81,0xfa],[0x49,0x81,0xf9]];
    static CMP1: [[u8;3];5] = [[0x48,0x83,0xf8],[0x49,0x83,0xf8],[0x48,0x83,0xff],[0x49,0x83,0xff],
                              [0x49,0x83,0xfb]];   // ★0.5.4: kcap 2사이트가 cmp r11 로 바뀜

    // ── ① 구조물 인식 반경 (d²>>8). 두 그룹이 1 차이(부등호 방향) ──
    let tr_a = dsh(trad, 0x53D1AC1, 8);
    // ★[08-03 주소 정정] 뒤 3개(`c8037c`·`c803ef`·`c80470`)는 어긋난 주소였고 한 번도 안 걸렸다.
    //   구간 전수 스캔으로 실제 사이트 확정 → `c80397`·`c80418`·`c8048b`, 그리고 **목록에 없던 `c80513` 추가**.
    // ↓0.5.4: prefix 가 사이트마다 달라져 루프를 펼침(원래 `for a in [..]`)
    p!(base + 0xd92e97, &[0x48,0x81,0xfb], 3, 4, tr_a);   // ←0.5.3 c7f8b9
    p!(base + 0xd92f3b, &[0x48,0x81,0xfb], 3, 4, tr_a);   // ←0.5.3 c7f939
    p!(base + 0xd92fc6, &[0x48,0x81,0xfb], 3, 4, tr_a);   // ←0.5.3 c7f9ac
    p!(base + 0xd93044, &[0x48,0x81,0xfb], 3, 4, tr_a);   // ←0.5.3 c7fa1f
    p!(base + 0xd932a6, &[0x48,0x81,0xfb], 3, 4, tr_a);   // ←0.5.3 c7faa0
    p!(base + 0xd935e7, &[0x48,0x81,0xfe], 3, 4, tr_a);   // ←0.5.3 c7fcaf
    p!(base + 0xd93689, &[0x48,0x81,0xfe], 3, 4, tr_a);   // ←0.5.3 c80289
    p!(base + 0xd9372b, &[0x48,0x81,0xfe], 3, 4, tr_a);   // ←0.5.3 c80309
    p!(base + 0xd937c0, &[0x48,0x81,0xfe], 3, 4, tr_a);   // ←0.5.3 c80397
    p!(base + 0xd93848, &[0x48,0x81,0xfe], 3, 4, tr_a);   // ←0.5.3 c80418
    p!(base + 0xd938de, &[0x48,0x81,0xfb], 3, 4, tr_a);   // ←0.5.3 c8048b
    p!(base + 0xd9397b, &[0x48,0x81,0xfe], 3, 4, tr_a);   // ←0.5.3 c80513
    // ↓0.5.4: prefix 가 사이트마다 달라져 루프를 펼침(원래 `for a in [..]`)
    p!(base + 0xd940e7, &[0x49,0x81,0xfa], 3, 4, tr_a.wrapping_sub(1));   // ←0.5.3 c7fd67
    p!(base + 0xd94197, &[0x49,0x81,0xfc], 3, 4, tr_a.wrapping_sub(1));   // ←0.5.3 c80be7
    // ── ② 적 챔피언 근접 인식 (d²>>9) ──
    for a in [0xd93ad3usize, 0xd93ba4, 0xd93c71, 0xd93d3e, 0xd93e0b] {
        pany!(base + a, CMP3, 3, 4, dsh(erad2, 0x1BF08EA, 9));
    }
    // ── ③ 셀 위협 진입 거리 (d²+1) ──
    pany!(base + 0xd943a4, CMP3, 3, 4, sqp(cdst, 0x49040441));   // ←0.5.3 c80e02
    // ── ④ 다이브 여유분 ──
    pany!(base + 0xd9533e, [[0x48,0x05]], 2, 4, b4(dvm, 15000));   // ←0.5.3 c7fe82
    // ── ⑤ 위험 판정 사다리 ──
    for (a, v, o) in [(0xc82224usize, rd0, 49u64), (0xd958c9, rh1, 65), (0xd958cf, rd1, 29),
                      (0xd958d5, rh2, 40), (0xd958db, rd2, 17), (0xd958e3, rh3, 25),
                      (0xd958e9, rd3, 10)] {
        pany!(base + a, CMP1, 3, 1, b1(v, o));
    }
    // ── ⑥ 보너스 상한·기준 ──
    pany!(base + 0xd95e74, CMP1, 3, 1, b1(fcap, 80));   // ←0.5.3 c827c4
    pany!(base + 0xd95e78, [[0x41,0xbf],[0x41,0xbe]], 2, 4, b4(fcap, 80));   // ★0.5.4: r14d→r15d   // ←0.5.3 c827c8
    // ★[08-05 감사] 이 상한은 `cmp / mov / cmovl` 3종 세트다. 예전엔 3쌍 중 2쌍이 **cmp만** 패치돼
    //   200으로 올리면 `cmp rdi,200 / mov eax,80 / cmovl` ⟹ 200 미만은 무제한 통과, 200 이상은 80으로 추락 =
    //   **상한을 올렸는데 큰 값만 잘리는 역전**이 났다. mov 쪽 2사이트를 짝으로 추가한다.
    pany!(base + 0xd96649, CMP1, 3, 1, b1(kcap, 80));   // ←0.5.3 c82f27
    pany!(base + 0xd9664d, [[0xb8]], 1, 4, b4(kcap, 80));   // ←0.5.3 c82f2b
    pany!(base + 0xd96698, CMP1, 3, 1, b1(kcap, 80));   // ←0.5.3 c82f76
    pany!(base + 0xd9669c, [[0xb8]], 1, 4, b4(kcap, 80));   // ←0.5.3 c82f7a
    // ⚠`c83391`은 `mov`가 아니라 **`cmp rax,80`** 이다(짝이 되는 `mov ecx,80`이 c83395). 08-03 정정.
    pany!(base + 0xd96a37, CMP1, 3, 1, b1(kcap, 80));   // ←0.5.3 c83391
    pany!(base + 0xd96a3b, [[0xb8],[0xb9],[0xbb]], 1, 4, b4(kcap, 80));   // ←0.5.3 c83395
    pany!(base + 0xd9668b, CMP1, 3, 1, b1(kpct, 60));   // ←0.5.3 c82f69
    // ── ⑦ 점수 계산의 시야 기억 ──
    // ⚠[08-05 감사] 같은 축이 5곳인데 여기서 잡는 건 1곳뿐이다. 나머지 4곳(`0xd93c04`·`0xd93cd1`·
    //   `0xd93d9e`·`0xd93e67`)은 **`vw_score`가 10사이트로 이미 소유**한다 — 여기서 또 잡으면 두 노브가
    //   같은 바이트를 다투게 되므로(마지막에 쓴 쪽이 이김) **의도적으로 1곳만 유지**한다.
    //   사용자에겐 `vw_score`를 쓰라고 안내한다(편집기 설명 반영).
    for a in [0xd93b33usize] { pany!(base + a, [[0x48,0x83,0xc6],[0x49,0x83,0xc6],[0x48,0x83,0xc5]], 3, 1, b1(svis, 120)); }
    // ── ⑧ 본체 0일 때 대체 점수(음수) ──
    let nv = if nul_orig { -10i64 } else { nul };
    for a in [0xd96c90usize, 0xd96f06] {
        pany!(base + a, [[0x48,0xc7,0xc1],[0x48,0xc7,0xc0],[0x49,0xc7,0xc1]], 3, 4,
              (nv as i32) as u32 as u64);
    }

    SCORE2_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("score2_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} turret_r{} engage_r{} cell_d{} dive{} risk[{} {}/{} {}/{} {}/{}] \
             cap[focus{} kill{} pct{}] vis{} null{}\n\
             (-1=원본: 150000 · 122474 · 35000 · 15000 · 49 65/29 40/17 25/10 · 80 80 60 · 120 · -10) @base{:#x}\n",
            ok, tot, trad, erad2, cdst, dvm, rd0, rh1, rd1, rh2, rd2, rh3, rd3,
            fcap, kcap, kpct, svis, nul, base));
    }
}

// ★★[08-03 신설] 이동 계열 점수(`0xc7b730` cat 0 도주 / cat 2 접근 / cat 4 추적) byte-patch.
//   근거 = RE\2026-08-03_이동계점수-c7b730-cat0-cat2-cat4-0.5.3.md
//   ★유효성: 점수 엔진은 모드 미대체 구간 ⟹ byte-patch 유효.
//
//   공식 요약
//     cat0 도주 = riskDrop/4 + (배율 × (towerDrop+engage))/800 − 2 + bonus10
//     cat2 접근 = 아군포탑 지원(0~100) − 현재셀 risk 비용 + 태그별 goal_gain
//     cat4 추적 = 배율×record 가치/100 + 포탑지원 − 포탑위협 − 목적지 risk − 교전 risk
//   ⚠cat0은 배율 테이블이 **cat4와 다른 별도 표**(40/75/100/200/300)를 쓴다 — 여기서 처음 노출한다.
unsafe fn apply_move_imm() {
    // ── cat0(도주·귀환) 전용 배율표. n = 적−아군 이므로 hi = "적이 2명 이상 많을 때" ──
    let m0hi = tune("mv0_adv_hi", -1);      // n≥+2 (원본 300)
    let m0lo = tune("mv0_adv_lo", -1);      // n≤−2 (원본 40)
    let m0m1 = tune("mv0_adv_m1", -1);      // n=−1 (원본 75, .rdata)
    let m0z  = tune("mv0_adv_0", -1);       // n=0  (원본 100, .rdata)
    let m0p1 = tune("mv0_adv_p1", -1);      // n=+1 (원본 200, .rdata)
    // ── cat0 가중치·보너스 ──
    let m0rs = tune("mv0_risk_shift", -1);  // 위험감소 항 shift(원본 2 = ÷4). ↓=도주 성향↑
    let m0es = tune("mv0_engage_shift", -1);// 피격·포탑 항 shift(원본 9 = ÷800). ↓=도주 성향↑
    let m0nb = tune("mv0_near_bonus", -1);  // 근접 보너스(원본 10)
    let m0ng = tune("mv0_near_gate", -1);   // 이 값 이상이면 근접 보너스 무효(원본 950)
    // ⚠원본이 음수(−2)라 "음수 = 원본" 규약을 쓸 수 없다 → 전용 센티널.
    let m0bp = tune("mv0_base_penalty", -9999);
    let m0bp_orig = m0bp <= -1000;
    // ── cat2/cat4 공통 ──
    let mtm  = tune("mv_tower_margin", -1); // 포탑 사거리 감산(원본 30000). ↑=포탑 지원·위협 판정이 좁아짐
    let mtc  = tune("mv_tower_cap", -1);    // 포탑 지원/위협 상한(원본 100, 3쌍 6곳)
    let m2gs = tune("mv2_gain_shift", -1);  // cat2 Around goal_gain shift(원본 7 = ÷200)
    let met  = tune("mv_engage_thr", -1);   // 예상 피격 합산 필터 임계(원본 9999 = 사실상 전부)
    let vmg  = tune("vis_mem_global", -1);  // ★0.5.4 신설: **전역** 적 "최근 목격" 유효틱(원본 120, 1곳=143 호출자 공용)
    //   구 `mv_vision_mem`(이동 점수 한정)은 0.5.4에서 코드가 흡수돼 그 범위로는 불가능해졌다.
    let mut sig = 0u64;
    for v in [m0hi, m0lo, m0m1, m0z, m0p1, m0rs, m0es, m0nb, m0ng, m0bp,
              mtm, mtc, m2gs, met, vmg] {
        sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64);
    }
    if sig == MOVEIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }
    macro_rules! pskip { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        // ⛔0.5.4 재배치 미확정 — 세기만 하고 **패치하지 않는다**.
        //    (orig_guard 는 표에 없는 RVA 를 통과시키므로, 옛 주소를 남기면 오패치 위험)
        tot += 1; let _ = ($a, $pre, $off, $w, $v);
    }}; }

    // ── ① cat0 배율 (코드 즉치 2개) ──
    p!(base + 0xd8eafe, &[0x41,0xb8], 2, 4, b4(m0hi, 300));   // ←0.5.3 c7c7ae
    p!(base + 0xd8eb0e, &[0x41,0xb8], 2, 4, b4(m0lo, 40));   // ←0.5.3 c7c7be
    // ── ①-b cat0 배율 (.rdata 3개) — prefix 검증이 불가능해 최초 1회 원본 확인 후에만 건드린다 ──
    {
        let t = base + 0x31AA4E8;
        let cur = (readable(t, 24) && rd_u64(t) == Some(75)
                   && rd_u64(t + 8) == Some(100) && rd_u64(t + 16) == Some(200))
                  || RDATA_ADV0_OK.load(Ordering::Relaxed);
        if cur {
            RDATA_ADV0_OK.store(true, Ordering::Relaxed);
            for (off, v, orig) in [(0usize, m0m1, 75u64), (8, m0z, 100), (16, m0p1, 200)] {
                tot += 1;
                let want = if v < 0 { orig } else { v.max(0) as u64 };
                let mut old: u32 = 0;
                if VirtualProtect(t + off, 8, 0x04, &mut old) != 0 {
                    core::ptr::write_unaligned((t + off) as *mut u64, want);
                    VirtualProtect(t + off, 8, old, &mut old);
                    ok += 1;
                }
            }
        } else {
            tot += 3;   // 주소 어긋남 → 미적용으로 계상(로그에 드러나게)
        }
    }
    // ── ② cat0 가중치·기본 페널티·근접 보너스 ──
    p!(base + 0xd8eb45, &[0x49,0xc1,0xf9], 3, 1, b1(m0rs, 2));    // sar r9, 2   (÷4)   // ←0.5.3 c7c7f5
    p!(base + 0xd8eb53, &[0x48,0xc1,0xfa], 3, 1, b1(m0es, 9));    // sar rdx, 9  (÷800)   // ←0.5.3 c7c803
    {   // add r12, −2  → imm8 그대로. 센티널 이하면 원본 유지.
        tot += 1;
        let want = if m0bp_orig { 0xfeu64 } else { (m0bp as i8) as u8 as u64 };
        ok += patch_imm_bytes(base + 0xd8eb5e, &[0x49,0x83,0xc5], 3, 1, want) as u32;
    }
    p!(base + 0xd8ee80, &[0xbb], 1, 4, b4(m0nb, 10));   // ←0.5.3 c7d5a6
    p!(base + 0xd8edbb, &[0x48,0x81,0xbd,0xb8,0x00,0x00,0x00], 7, 4, b4(m0ng, 950));   // ←0.5.3 c7d4f0  ★재조사로 복구: 950 (rbp+0x68→+0xb8)  ★★imm_off 4→7 — 0.5.4에서 REX 접두가 붙어 즉치가 1B 밀렸다(크래시 원인)
    // ── ③ 포탑 사거리 감산 / 상한 3쌍 ──
    for a in [0xd8f3dcusize, 0xd8f60d] {
        p!(base + a, &[0x48,0x81,0xee], 3, 4, b4(mtm, 30000));
    }
    //   상한은 `cmp rax,100` 과 `mov reg,100` **두 곳을 같이** 고쳐야 의미가 맞는다.
    for (ca, mv, mpre) in [(0xd8ec43usize, 0xd8ec47usize, &[0x41,0xbd][..]),
                           (0xd8f4db,      0xd8f4df,      &[0xb9][..]),
                           (0xd8fea6,      0xd8feaa,      &[0xb9][..])] {
        p!(base + ca, &[0x48,0x83,0xf8], 3, 1, b1(mtc, 100));
        p!(base + mv, mpre, mpre.len(), 4, b4(mtc, 100));
    }
    // ── ④ cat2 goal_gain 가중 / 예상 피격 필터 / 시야 기억 ──
    p!(base + 0xd8ec84, &[0x48,0xc1,0xfa], 3, 1, b1(m2gs, 7));    // sar rdx, 7 (÷200)   // ←0.5.3 c7d3d0
    for a in [0xd8e8dfusize, 0xd8f0f6, 0xd8f74f] {
        p!(base + a, &[0x41,0xb8], 2, 4, b4(met, 9999));
    }
    // ★[08-05 감사] 동일 패턴이 5곳인데 1곳만 잡고 있었다(다른 키가 커버하지도 않음).
    //   3곳은 `add rdi,0x78`, 마지막 1곳만 `add rsi,0x78` — 레지스터가 달라 prefix가 갈린다.
    // ↓0.5.4: prefix 가 사이트마다 달라져 루프를 펼침(원래 `for a in [..]`)
    // ★0.5.4: action_score 안에 인라인이던 "최근 목격 120틱" 판정 5곳이
    //   **공용 아웃라인 `f3d600` 으로 흡수**됐다(054 action_score 안엔 `add r,0x78` 이 0곳).
    //   여기를 패치하면 xref **143곳 전부**(death_battle·team_plan·epic·serpen·ganker…)가 바뀐다.
    //   ⟹ 노브 의미가 "이동 점수 한정" → **"전역 시야 기억"** 으로 달라져 키를 새로 뒀다.
    //   ⚠옛 키(mv_vision_mem)의 알리아스를 두지 않는다 — 옛 값이 조용히 전역 적용되면 더 위험하다.
    p!(base + 0xf3d658, &[0x48,0x83,0xc3], 3, 1, b1(vmg, 120));

    MOVEIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("move_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} cat0adv[{} {} {} {} {}] cat0[rs{} es{} bp{} nb{} ng{}] \
             tower[margin{} cap{}] cat2gain{} engage_thr{} vis{}\n\
             (-1=원본: 300 40 75 100 200 / 2 9 -2 10 950 / 30000 100 / 7 / 9999 / 120) @base{:#x}\n",
            ok, tot, m0hi, m0lo, m0m1, m0z, m0p1, m0rs, m0es, m0bp, m0nb, m0ng,
            mtm, mtc, m2gs, met, vmg, base));
    }
}

// ★★[08-03 신설] 숨은 행동 생산자 층 byte-patch — `0xdaf780`(death_battle 전투 후보 생성기)과
//   그 게이트 함수들(`0xc8b560` 안전판정 등).
//   근거 = RE\2026-08-03_숨은생산자5함수-daf780-fight_check-0.5.3.md
//   ★유효성: sub_plan 실행층 하위 = 모드 미대체 ⟹ byte-patch 유효.
//   ⚠거리 상수는 **d² 형태로 저장**돼 있어 그대로 못 쓴다 — `sq`(제곱)/`sqp`(제곱+1)로 변환한다.
unsafe fn apply_db_imm() {
    let dna  = tune("dm_near_ally", -1);    // "곁에 아군 있음" 판정 거리(원본 150000, 5곳)
    let dne  = tune("dm_near_enemy", -1);   // "교전 중" 판정 거리(원본 150000, 5곳)
    let dla  = tune("dm_lookahead", -1);    // 사거리 판정 선행 틱수(원본 30, 5곳). ↑=훨씬 멀리서도 "닿는다"고 판단
    let dul  = tune("dm_ult_lookahead", -1);// 궁 경로 선행 틱수(원본 60, 2곳)
    let dex  = tune("dm_execute_hp", -1);   // 처형 사정권 HP%(원본 20)
    let dlh  = tune("dm_lasthit", -1);      // 미니언 막타 인정 타격수(원본 2 = 3방 이내)
    let dsh_ = tune("dm_skill_hp", -1);     // 스킬을 무조건 허용하는 대상 HP% 상한(원본 79, 2곳)
    let dur  = tune("dm_ult_rally", -1);    // 궁 대상 랠리포인트 근접 게이트(원본 6000, 2곳)
    let dur2 = tune("dm_ult_rally2", -1);   // 특정 궁 타입의 완화 게이트(원본 90000)
    let durg = tune("dm_ult_range", -1);    // 궁 총사거리 임계(원본 150000). ⚠넘으면 대상집합 자체가 바뀜
    let dm1  = tune("dm_ult_mask_rally", -1);// 랠리 거리 게이트를 적용할 팀모드 마스크(원본 0x6f)
    let dm2  = tune("dm_ult_mask_focus", -1);// 지정 타깃 예외 경로 마스크(원본 0x4e)
    let dm3  = tune("dm_ult_mask_safe", -1); // 안전판정 강제 마스크(원본 0x21)
    let ds2l = tune("dm_skill2_level", -1); // 스킬2 해금 레벨(원본 3)
    let dulv = tune("dm_ult_level", -1);    // 궁 해금 레벨(원본 5)
    let dsm  = tune("sf_margin", -1);  // 안전판정의 적 사거리 여유분(원본 15000). ↓=겁 덜 냄
    let dsr  = tune("sf_radius", -1);  // 안전판정 머릿수 반경(원본 120000, 3곳)
    let dsv  = tune("sf_mem", -1);     // 안전판정의 적 위치 기억 틱(원본 120, 5곳)
    let mut sig = 0u64;
    for v in [dna, dne, dla, dul, dex, dlh, dsh_, dur, dur2, durg,
              dm1, dm2, dm3, ds2l, dulv, dsm, dsr, dsv] {
        sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64);
    }
    if sig == DBIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let sq  = |v: i64, orig: u64| if v < 0 { orig } else { let d = v.max(0) as u64; d.wrapping_mul(d) };
    let sqp = |v: i64, orig: u64| if v < 0 { orig } else { let d = v.max(0) as u64; d.wrapping_mul(d).wrapping_add(1) };
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }
    macro_rules! pskip { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        // ⛔0.5.4 재배치 미확정 — 세기만 하고 **패치하지 않는다**.
        //    (orig_guard 는 표에 없는 RVA 를 통과시키므로, 옛 주소를 남기면 오패치 위험)
        tot += 1; let _ = ($a, $pre, $off, $w, $v);
    }}; }

    // ── ① 아군/적 근접 판정 반경 (둘 다 원본 150000, d²+1 = 0x53D1AC101) ──
    for a in [0xe29d40usize, 0xe29db6, 0xe29e2c, 0xe29ea2, 0xe29f14] {
        p!(base + a, &[0x48,0xb8], 2, 8, sqp(dna, 0x5_3D1A_C101));
    }
    for a in [0xe29fe1usize, 0xe2a068, 0xe2a0ef, 0xe2a176, 0xe2a1fd] {
        p!(base + a, &[0x48,0xb8], 2, 8, sqp(dne, 0x5_3D1A_C101));
    }
    // ── ② 사거리 판정 선행 틱수 (speed × N) ──
    for a in [0xe2a272usize, 0xe2acf5, 0xe2c0e3, 0xe2d0ef] {
        p!(base + a, &[0xb9], 1, 4, b4(dla, 30));
    }
    p!(base + 0xe2a8b0, &[0x41,0xb9], 2, 4, b4(dla, 30));   // ←0.5.3 db05b0
    p!(base + 0xe2edac, &[0x48,0x6b,0x8d], 7, 1, b1(dul, 60));   // ←0.5.3 db4aac
    p!(base + 0xe2ef5a, &[0x49,0x6b,0x83], 7, 1, b1(dul, 60));   // ←0.5.3 db4c5a
    // ── ③ 처형·막타·스킬 허용 HP% ──
    // ★[08-05 감사] 이 둘은 64bit/32bit div 쌍인데 **64bit 쪽만** 잡고 있었다.
    //   `or rax,rcx; shr rax,0x20; je`로 갈리므로 HP·피해가 32비트에 들어가는 **통상 상황에선 32bit 경로가 돈다**
    //   ⟹ 사실상 무효였다. 짝을 추가한다. (같은 함수의 cs_steal_hp·cs_unit_hits는 원래 쌍을 다 잡고 있었다.)
    p!(base + 0xe2d1d6, &[0x48,0x83,0xf8], 3, 1, b1(dex, 20));   // ←0.5.3 db2ed6
    p!(base + 0xe2d1e6, &[0x48,0x83,0xf8], 3, 1, b1(dex, 20));   // ←0.5.3 db2ee6
    p!(base + 0xe2a7be, &[0x48,0x83,0xf8], 3, 1, b1(dlh, 2));   // ←0.5.3 db04be
    p!(base + 0xe2a7d0, &[0x48,0x83,0xf8], 3, 1, b1(dlh, 2));   // ←0.5.3 db04d0
    for a in [0xe2b4e6usize, 0xe2c8a6] {
        p!(base + a, &[0x48,0x83,0xf8], 3, 1, b1(dsh_, 79));
    }
    // ── ④ 궁 게이트 ──
    p!(base + 0xe2e6bd, &[0xb9], 1, 4, sq(dur, 36_000_000));      // 6000²   // ←0.5.3 db43bd
    p!(base + 0xe2e6d5, &[0xb8], 1, 4, sq(dur, 36_000_000));   // ←0.5.3 db43d5
    p!(base + 0xe2e6da, &[0x48,0xb9], 2, 8, sqp(dur2, 0x1_E2CC_3100)); // 90000²+1   // ←0.5.3 db43da
    p!(base + 0xe2e71e, &[0x48,0x3d], 2, 4, b4(durg, 150_000));   // ←0.5.3 db441e
    p!(base + 0xe2e999, &[0xb9], 1, 4, b4(dm1, 0x6f));   // ←0.5.3 db4699
    p!(base + 0xe2ec2c, &[0xb9], 1, 4, b4(dm2, 0x4e));   // ←0.5.3 db492c
    p!(base + 0xe2ec53, &[0xb9], 1, 4, b4(dm3, 0x21));   // ←0.5.3 db4953
    // ── ⑤ 스킬2·궁 해금 레벨 ──
    p!(base + 0xe29cab, &[0x48,0x83,0xfa], 3, 1, b1(ds2l, 3));   // ←0.5.3 daf9ab
    p!(base + 0xe29ccf, &[0x48,0x83,0xfa], 3, 1, b1(dulv, 5));   // ←0.5.3 daf9cf
    // ── ⑥ 안전판정 `0xc8b560` — 위 값들과 같이 안 바꾸면 효과가 상쇄된다 ──
    if !micro_taken("sf_margin") {   // ★[08-07] 마이크로 디투어와 상호배타
    p!(base + 0xdb3f1b, &[0x48,0x05], 2, 4, b4(dsm, 15_000));   // ←0.5.3 c8b99b
    }
    // ★[08-05 감사] 3곳 → **5곳**(같은 함수의 5슬롯 언롤). 짝인 `sf_mem`이 5/5인 것과 대조해 확정.
    for a in [0xdb4079usize, 0xdb4166, 0xdb4242, 0xdb431e, 0xdb43fa] {
        p!(base + a, &[0x48,0xb8], 2, 8, sq(dsr, 0x3_5A4E_9000));  // 120000²
    }
    for a in [0xdb40deusize, 0xdb41cb, 0xdb42a7, 0xdb4383, 0xdb444c] {
        p!(base + a, &[0x48,0x83,0xc6], 3, 1, b1(dsv, 120));
    }

    DBIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("db_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} near[ally{} enemy{}] look[{} ult{}] exec{} lasthit{} skillhp{} \
             ult[rally{} rally2{} range{} mask {:#x}/{:#x}/{:#x}] lv[s2 {} ult {}] \
             safe[margin{} radius{} mem{}]\n\
             (-1=원본: 150000 150000 / 30 60 / 20 / 2 / 79 / 6000 90000 150000 0x6f 0x4e 0x21 / 3 5 / 15000 120000 120)\
             @base{:#x}\n",
            ok, tot, dna, dne, dla, dul, dex, dlh, dsh_, dur, dur2, durg,
            dm1, dm2, dm3, ds2l, dulv, dsm, dsr, dsv, base));
    }
}

// ★★[08-03 신설] 자리 평가 엔진(`position_eval.rs`, 본문 `0xcc9d70`) byte-patch.
//   근거 = RE\2026-08-03_position_eval-본문-cc9d70-risk생성-0.5.3.md
//   ★유효성: 모드 미대체 구간 ⟹ byte-patch 유효.
//
//   ★핵심 사실
//   - `risk`/`tower_risk`/`gain`/`gain_me`의 단위 = **"내 현재 HP의 몇 %"**. 소스당 150 캡.
//   - `tower_risk`는 `risk`의 **부분집합**(SSE 한 줄로 동시 가산) = 판단력 노이즈가 안 걸리는 결정론적 하한.
//   - `0xcc9960`은 진입점이 아니라 **512버킷 메모이제이션 래퍼** — 같은 틱·셀·kind면 본문이 안 돈다.
//   ⚠**`×1968`·`×25`는 immediate가 아니라 `lea` 조합**이라 byte-patch로 못 건드린다(노브 없음).
unsafe fn apply_pe_imm() {
    let pcol = tune("pe_collect_radius", -1); // 적·아군 위협 수집 반경(원본 200000, 12곳)
    let pflt = tune("pe_filter_radius", -1);  // 구조물·미니언 후보 필터(원본 150000, 10곳)
    let pnea = tune("pe_near_cut", -1);       // 병합 이터 근접컷(원본 70000, 4곳)
    let pmin = tune("pe_minion_add", -1);     // ★미니언·중립을 위험으로 **가산**하는 거리(원본 64000)
    let pcha = tune("pe_champ_threat", -1);   // ★적 챔피언 위협 평가 컷(원본 100000)
    let pfld = tune("pe_field_radius", -1);   // 장판 맵 순회 반경(원본 250000)
    let pcnt = tune("pe_count_radius", -1);   // ★고립 판정용 아군·적 카운트 반경(원본 120000, 2곳)
    let prea = tune("pe_reach_bonus", -1);    // gain·risk 슬롯 "접근 허용 거리"(원본 80000, 21곳)
    let pband= tune("pe_outer_band", -1);     // 투사체·중립 외곽 감쇠 띠(원본 32000, 2곳)
    let pshot= tune("pe_skillshot_width", -1);// ★스킬샷 궤적 허용폭(원본 20000)
    let pblk = tune("pe_bodyblock_width", -1);// 아군 몸빵 판정폭(원본 28000, 3곳)
    let ptwr = tune("pe_tower_margin", -1);   // 타워·미니언 사거리 여유(원본 18000, 2곳)
    let pcap = tune("pe_source_cap", -1);     // ★소스당 %HP 캡(원본 150, **11쌍 22곳**)
    let ppcap= tune("pe_predict_cap", -1);    // 예측피해 캡(원본 140, 1쌍)
    let pfar = tune("pe_tower_far", -1);      // 원거리 타워 기여 계수(원본 656, 2곳)
    let pna2 = tune("pe_noise_amp_mode2", -1);// 노이즈 진폭 상한 mode==2(원본 1000)
    let pna  = tune("pe_noise_amp", -1);      // 노이즈 진폭 상한 그 외(원본 2000)
    let pnex = tune("pe_noise_exempt", -1);   // ★노이즈 면제 판단력 임계(원본 100000). ↓면 노이즈 잦아짐
    let pks  = tune("pe_kind_scale", -1);     // kind 스케일 ×1.2(원본 120, 2곳)
    let pmsk = tune("pe_mode_mask", -1);      // 미니언 위협을 켜는 게임모드 마스크(원본 0x1a1, 2곳)
    let pkm  = tune("pe_kind_mask", -1);      // 예측피해 감산 kind 마스크(원본 0x303)
    let pwall= tune("pe_wall_risk", -1);      // 지형 벽 셀 risk(원본 9999)
    let pwell= tune("pe_well_risk", -1);      // 적 우물 risk·tower_risk·base(원본 9999, 3곳)
    let pagc = tune("pe_ally_gain_cut", -1);  // 아군 스킬 gain 컷(원본 1200)
    let pst  = tune("pe_state_gate", -1);     // 상태 게이트(원본 180, 3곳)
    let mut sig = 0u64;
    for v in [pcol, pflt, pnea, pmin, pcha, pfld, pcnt, prea, pband, pshot, pblk, ptwr,
              pcap, ppcap, pfar, pna2, pna, pnex, pks, pmsk, pkm, pwall, pwell, pagc, pst] {
        sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64);
    }
    if sig == PEIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let sq  = |v: i64, orig: u64| if v < 0 { orig } else { let d = v.max(0) as u64; d.wrapping_mul(d) };
    let sqp = |v: i64, orig: u64| if v < 0 { orig } else { let d = v.max(0) as u64; d.wrapping_mul(d).wrapping_add(1) };
    // 거리 → (d² >> shift). 원본이 짝수/홀수(±1)인 사이트가 섞여 있어 오프셋 인자를 둔다.
    let dsh = |v: i64, orig: u64, sh: u32, adj: u64| if v < 0 { orig } else {
        let d = v.max(0) as u64; ((d.wrapping_mul(d)) >> sh).wrapping_add(adj)
    };
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }
    macro_rules! pskip { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        // ⛔0.5.4 재배치 미확정 — 세기만 하고 **패치하지 않는다**.
        //    (orig_guard 는 표에 없는 RVA 를 통과시키므로, 옛 주소를 남기면 오패치 위험)
        tot += 1; let _ = ($a, $pre, $off, $w, $v);
    }}; }
    // ⚠같은 상수라도 사이트마다 **레지스터와 명령 형태가 다르다**(`cmp rcx`/`cmp r12`/`cmp rax` 등).
    //   prefix 길이가 다르면 immediate 오프셋도 달라지므로 **(prefix, off) 쌍**으로 후보를 든다.
    macro_rules! pmulti { ($a:expr, $cands:expr, $w:expr, $v:expr) => {{
        tot += 1;
        let mut done = false;
        for (pre, off) in $cands.iter() {
            if !done && patch_imm_bytes($a, pre, *off, $w, $v) { done = true; }
        }
        ok += done as u32;
    }}; }

    // ── ① 거리 임계 (전부 d² 또는 d²>>shift 인코딩) ──
    for a in [0xca8e05usize, 0xca8e40, 0xca8ed4, 0xca8f10, 0xca8fa4, 0xca8fe0, 0xca9077, 0xca90ad] {
        p!(base + a, &[0x48,0xb8], 2, 8, sqp(pcol, 0x9_502F_9001));
    }
    for a in [0xca9143usize, 0xca9178, 0xca94b0, 0xca94e0] {
        p!(base + a, &[0x48,0xb8], 2, 8, sq(pcol, 0x9_502F_9000));
    }
    for a in [0xcaa4a1usize, 0xcaa648, 0xcaa807, 0xcaafd8] {
        p!(base + a, &[0x48,0x81,0xf9], 3, 4, dsh(pflt, 87_890_625, 8, 1));
    }
    for a in [0xcab8c8usize, 0xcab968] {
        p!(base + a, &[0x48,0x81,0xfa], 3, 4, dsh(pflt, 87_890_625, 8, 1));
    }
    p!(base + 0xca98eb, &[0x48,0x3d], 2, 4, dsh(pflt, 87_890_624, 8, 0));   // ←0.5.3 ccaeef
    p!(base + 0xcabc48, &[0x48,0x81,0xfa], 3, 4, dsh(pflt, 87_890_624, 8, 0));   // ←0.5.3 ccd108
    // ↓0.5.4: prefix 가 사이트마다 달라져 루프를 펼침(원래 `for a in [..]`)
    p!(base + 0xcac08c, &[0x49,0xbe], 2, 8, sq(pflt, 0x5_3D1A_C100));   // ←0.5.3 ccd76e  ★재조사로 복구: pflt (053 3곳=재로드→054 1곳)
    pskip!(base + 0xccd78e, &[0x49,0xbb], 2, 8, sq(pflt, 0x5_3D1A_C100));   // ⛔0.5.4 미확정: 시그 3→0 / 완화 3→1 (골격 86%)
    pskip!(base + 0xccdad8, &[0x49,0xbb], 2, 8, sq(pflt, 0x5_3D1A_C100));   // ⛔0.5.4 미확정: 시그 3→0 / 완화 3→1 (골격 86%)
    for a in [0xcaa6c1usize, 0xcab04a] {
        p!(base + a, &[0x49,0x81,0xfe], 3, 4, dsh(pnea, 19_140_625, 8, 1));
    }
    for a in [0xcaa51dusize, 0xcaa876] {
        p!(base + a, &[0x49,0x81,0xfe], 3, 4, dsh(pnea, 19_140_624, 8, 0));
    }
    // ★0.5.4: **원본값이 +1 됐다**(0xf4240000→0xf4240001) = 비교 부등호가 뒤집혔다.
    //   그래서 sq → sqp 로 바꾼다. 값만 옮기면 가드에 막혀 조용히 죽는다.
    p!(base + 0xcabc0a, &[0xb8], 1, 4, sqp(pmin, 4_096_000_001));   // ←0.5.3 ccd3f3
    p!(base + 0xcad98a, &[0x48,0x81,0xf9], 3, 4, dsh(pcha, 9_765_625, 10, 0));   // ←0.5.3 ccefaa
    p!(base + 0xcac640, &[0x49,0x81,0xf8], 3, 4, dsh(pfld, 244_140_624, 8, 0));   // ←0.5.3 ccdcf1
    p!(base + 0xcaeb9b, &[0x48,0xb8], 2, 8, sqp(pcnt, 0x3_5A4E_9001));   // ←0.5.3 cd01f5
    p!(base + 0xcaf40a, &[0x48,0xb8], 2, 8, sq(pcnt, 0x3_5A4E_9000));   // ←0.5.3 cd0aab
    // ── ② 선형 반경·여유 ──
    for a in [0xcaced5usize, 0xcacf06, 0xcad085, 0xcad0b6, 0xcad265, 0xcad296, 0xcad415,
              0xcad446, 0xcad5f5, 0xcad626, 0xcad7a5, 0xcad7d6, 0xcaef34, 0xcaef61] {
        p!(base + a, &[0x49,0x81,0xc0], 3, 4, b4(prea, 80_000));
    }
    for a in [0xcadea1usize, 0xcae120, 0xcae399] {
        p!(base + a, &[0x48,0x81,0xc1], 3, 4, b4(prea, 80_000));
    }
    for a in [0xcaf0a9usize, 0xcaf0d6, 0xcaf209, 0xcaf23a] {
        p!(base + a, &[0x49,0x81,0xc1], 3, 4, b4(prea, 80_000));
    }
    for a in [0xca9eceusize, 0xcac415] { p!(base + a, &[0x48,0x05], 2, 4, b4(pband, 32_000)); }
    p!(base + 0xcae7a8, &[0x48,0x05], 2, 4, b4(pshot, 20_000));   // ←0.5.3 ccfd38
    for a in [0xcaea4ausize, 0xcaeb1c, 0xcaf47c] {
        p!(base + a, &[0x48,0x81,0xc2], 3, 4, b4(pblk, 28_000));
    }
    p!(base + 0xcab59d, &[0x48,0x05], 2, 4, b4(ptwr, 18_000));   // ←0.5.3 ccccc4
    p!(base + 0xcac7cd, &[0x49,0x81,0xc1], 3, 4, b4(ptwr, 18_000));   // ←0.5.3 ccde6f
    // ── ③ 캡·비율 ──
    //   ⚠150 캡은 `cmp`와 `mov`가 **쌍**이라 둘 다 안 고치면 의미가 어긋난다.
    //   실측된 cmp 인코딩 5종·mov 인코딩 2종. `48 3d`(cmp rax)는 prefix가 2바이트라 imm 오프셋도 2다.
    static CAPCMP: [(&[u8], usize); 5] = [(&[0x48,0x81,0xf9], 3), (&[0x49,0x81,0xfc], 3),
                                         (&[0x48,0x81,0xff], 3), (&[0x48,0x81,0xfb], 3),
                                         (&[0x48,0x3d], 2)];
    static CAPMOV: [(&[u8], usize); 3] = [(&[0xb8], 1), (&[0xb9], 1), (&[0xbb], 1)];
    // ★스택 오버플로 방지: 호출부를 펼치지 말고 **표+루프 1개**로 유지할 것.
    //   (펼치면 opt-level=1 에서 프레임이 선형으로 커져 rayon 워커 스택을 넘긴다 — 실사고)
    static PE_CAP: [(usize, &[u8], usize); 22] = [
        (0xca9a6a, &[0x49,0x81,0xfc], 3),
        (0xca9a71, &[0xb8], 1),
        (0xca9c3e, &[0x49,0x81,0xfc], 3),
        (0xca9c45, &[0xb8], 1),
        (0xcaadc5, &[0x49,0x81,0xff], 3),
        (0xcaadcc, &[0xb8], 1),
        (0xcab2d7, &[0x48,0x3d], 2),
        (0xcab2dd, &[0xb9], 1),
        (0xcabb35, &[0x48,0x81,0xf9], 3),
        (0xcabb3c, &[0xb8], 1),
        (0xcabe4d, &[0x48,0x81,0xf9], 3),
        (0xcabe54, &[0xb8], 1),
        (0xcac1ec, &[0x49,0x81,0xfd], 3),
        (0xcac1f3, &[0xb8], 1),
        (0xcac843, &[0x48,0x3d], 2),
        (0xcac849, &[0xb9], 1),
        (0xcacaec, &[0x48,0x3d], 2),
        (0xcacaf2, &[0xb9], 1),
        (0xcacc77, &[0x48,0x3d], 2),
        (0xcacc7d, &[0xb9], 1),
        (0xcae928, &[0x49,0x81,0xff], 3),
        (0xcae92f, &[0xb8], 1),
    ];
    for &(a, pre, off) in PE_CAP.iter() { p!(base + a, pre, off, 4, b4(pcap, 150)); }
    p!(base + 0xcabedd, &[0x48,0x3d], 2, 4, b4(ppcap, 140));   // ←0.5.3 ccd5c5
    p!(base + 0xcabee3, &[0xbe], 1, 4, b4(ppcap, 140));   // ←0.5.3 ccd5cb
    // ↓0.5.4: prefix 가 사이트마다 달라져 루프를 펼침(원래 `for a in [..]`)
    p!(base + 0xcab773, &[0x44,0x69,0xc0], 3, 4, b4(pfar, 656));   // ←0.5.3 ccce81  ★재조사로 복구: pfar 656 #1  ★★imm_off 2→3 — 0.5.4에서 REX 접두가 붙어 즉치가 1B 밀렸다(크래시 원인)
    p!(base + 0xcab817, &[0x44,0x69,0xc0], 3, 4, b4(pfar, 656));   // ←0.5.3 cccef2  ★재조사로 복구: pfar 656 #2  ★★imm_off 2→3 — 0.5.4에서 REX 접두가 붙어 즉치가 1B 밀렸다(크래시 원인)
    p!(base + 0xcaf8a0, &[0xbe], 1, 4, b4(pna2, 1000));   // ←0.5.3 cd0e8e  ★재조사로 복구: pna2 1000  ★41be(6B)→be(5B) 로 인코딩 축소 ⟹ imm_off 2→1
    p!(base + 0xcaf8d1, &[0xb9], 1, 4, b4(pna, 2000));   // ←0.5.3 cd0e9e
    // ★0.5.4 재규명: 이 노브는 **포지셔닝 스탯 기반 위치 노이즈의 면제선**이다.
    //   (판단력이 아니다 — `[unit+0x1f8]`=포지셔닝. 구 RE 의 "판단력" 표기는 오류였다.)
    //   0.5.3 = `[+0x400](실력스탯 보정계수) × [+0x1f8] >= 100000` 로 **보정된** 값을 봤다.
    //   0.5.4 = `[+0x1f8] >= 100` — 보정 곱셈이 사라져 **원본(표시) 스탯**으로 본다.
    //     ⟹ **패치되는 값**이 ÷1000 이다. ⚠cfg 단위는 옛날 그대로(기본 100000) — 아래 코드가 나눠준다.
    //       cfg 에 100 을 넣으면 v=0 이 되어 술어가 항상 참 = 전원 면제 = 노이즈가 꺼진다(08-06 실사고).
    //   ⚠**width 4→1(imm8 부호확장)** 이라 0~127 만 인코딩된다. 128↑은 명령이 4B라 in-place 불가.
    //   ⚠**두 곳 다 패치해야 한다** — 0.5.4가 신규 분기(`param_2>=2`)로 같은 술어를 복제했다.
    //   ⛔`caf823`/`caf827` 은 바이트가 같지만 **진폭 캡**이다(곱셈으로 흘러감). 건드리지 말 것.
    {
        let v = if pnex < 0 { 100 } else { (pnex / 1000).max(0).min(127) as u64 };
        p!(base + 0xcaf845, &[0x48,0x83,0xf8], 3, 1, v);
        p!(base + 0xcaf855, &[0x48,0x83,0xf8], 3, 1, v);
    }
    p!(base + 0xcaf6f7, &[0x48,0x6b,0x8d,0xa0,0x06,0x00,0x00], 3, 1, b1(pks, 120));   // ←0.5.3 cd0db9  ★재조사로 복구: pks 120 #1
    p!(base + 0xcaf78f, &[0x48,0x6b,0x8d,0x30,0x06,0x00,0x00], 7, 1, b1(pks, 120));   // ←0.5.3 cd0ddc  ★재조사로 복구: pks 120 #2  ★★imm_off 4→7 — 0.5.4에서 REX 접두가 붙어 즉치가 1B 밀렸다(크래시 원인)
    for a in [0xca9f79usize, 0xcabf21] { p!(base + a, &[0xba], 1, 4, b4(pmsk, 0x1a1)); }
    p!(base + 0xcabf6c, &[0xb9], 1, 4, b4(pkm, 0x303));   // ←0.5.3 ccd654
    // ★0.5.4: 대상 레지스터가 rax→r15 (`48 c7 00` → `49 c7 07`).
    p!(base + 0xca8936, &[0x49,0xc7,0x07], 3, 4, b4(pwall, 9999));   // ←0.5.3 cc9eaf
    pskip!(base + 0xcca0a6, &[0x48,0xc7,0x02], 3, 4, b4(pwell, 9999));   // ⛔0.5.4 미확정: 시그 1→0 / 완화 2→1 (골격 86%)
    pskip!(base + 0xcca0ad, &[0x48,0xc7,0x42,0x08], 4, 4, b4(pwell, 9999));   // ⛔0.5.4 미확정: 시그 1→0 / 완화 1→0 (골격 86%)
    p!(base + 0xca8b27, &[0xb8], 1, 4, b4(pwell, 9999));   // ←0.5.3 cca0b5
    p!(base + 0xcaf390, &[0x48,0x81,0xbd,0x10,0x04,0x00,0x00], 7, 4, b4(pagc, 1200));   // ←0.5.3 cd0a31
    //   세 사이트가 **읽는 구조체 오프셋도 레지스터도 다르다**(+0xb8/rdi, +0xc0/rdi, +0xc8/rcx).
    // ★스택 오버플로 방지: 호출부를 펼치지 말고 **표+루프 1개**로 유지할 것.
    //   (펼치면 opt-level=1 에서 프레임이 선형으로 커져 rayon 워커 스택을 넘긴다 — 실사고)
    static PE_STG: [(usize, &[u8], usize); 3] = [
        (0xcacd94, &[0x48,0x81,0xbe,0xb8,0x00,0x00,0x00], 7),
        (0xcad12d, &[0x48,0x81,0xbe,0xc0,0x00,0x00,0x00], 7),
        (0xcad4b6, &[0x48,0x81,0xbe,0xc8,0x00,0x00,0x00], 7),
    ];
    for &(a, pre, off) in PE_STG.iter() { p!(base + a, pre, off, 4, b4(pst, 180)); }
    PEIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("pe_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} dist[collect{} filter{} near{} minion{} champ{} field{} count{}] \
             lin[reach{} band{} shot{} block{} tower{}] cap[src{} pred{}] \
             misc[far{} amp2 {} amp{} exempt{} kscale{} modemask{} kindmask{} wall{} well{} allygain{} state{}]\n\
             (-1=원본: 200000 150000 70000 64000 100000 250000 120000 / 80000 32000 20000 28000 18000 / 150 140 / 656 1000 2000 100000 120 0x1a1 0x303 9999 9999 1200 180) @base{:#x}\n",
            ok, tot, pcol, pflt, pnea, pmin, pcha, pfld, pcnt,
            prea, pband, pshot, pblk, ptwr, pcap, ppcap,
            pfar, pna2, pna, pnex, pks, pmsk, pkm, pwall, pwell, pagc, pst, base));
    }
}

// ★★[08-04 신설] 이동 입력 생성기 `0xc86560` + 우물 탈출 오버라이드 byte-patch.
//   근거 = RE\2026-08-04_d945a0-행동실행층-is_act발견-우물탈출-0.5.3.md
//   ★레버리지 최대 — **모든 이동 행동의 최종 입력이 `0xc86560` 한 곳을 통과**한다.
//   ⚠우물 탈출은 `0xd945a0`(태그 디스패치보다 먼저)과 `0xc86560` **두 곳에 미러**돼 있어 세트로 고쳐야 한다.
unsafe fn apply_move2_imm() {
    let snap = tune("mv2_arrive_snap", -1);   // 이 거리 안이면 회피 계산 생략하고 목적지 직행(원본 2000)
    let acf  = tune("mv2_avoid_coef", -1);    // 회피 반경 계수 = 상대 반경 × 이값(원본 400)
    let amg  = tune("mv2_avoid_margin", -1);  // 회피 여유 상수(원본 6000). ↑=뭉침↓·경로↑
    let abi  = tune("mv2_avoid_bias", -1);    // 편향 확정 임계(원본 1500, 의미 추정)
    let wr   = tune("mv2_well_radius", -1);   // 우물 탈출 발동 반경(원본 260000, 2곳)
    let wd   = tune("mv2_well_dist", -1);     // 우물 탈출 목적지 거리(원본 260000, 4곳)
    let apm  = tune("mv2_pos_mode_thr", -1);  // AroundPosition 이동 모드 전환 임계(원본 10)
    let mut sig = 0u64;
    for v in [snap, acf, amg, abi, wr, wd, apm] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == MOVE2_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let sq = |v: i64, orig: u64| if v < 0 { orig } else { let d = v.max(0) as u64; d.wrapping_mul(d) };
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }
    macro_rules! pskip { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        // ⛔0.5.4 재배치 미확정 — 세기만 하고 **패치하지 않는다**.
        //    (orig_guard 는 표에 없는 RVA 를 통과시키므로, 옛 주소를 남기면 오패치 위험)
        tot += 1; let _ = ($a, $pre, $off, $w, $v);
    }}; }
    p!(base + 0xe58be8, &[0x48,0x3d], 2, 4, b4(snap, 2000));   // ←0.5.3 c8694b
    if !micro_taken("mv2_avoid_coef") {   // ★[08-07] 마이크로 디투어와 상호배타
    p!(base + 0xe58cf1, &[0x48,0x69,0xc1], 3, 4, b4(acf, 400));   // ←0.5.3 c86a36
    }
    p!(base + 0xe58d39, &[0x48,0x83,0xc1], 3, 1, b1(50, 50));   // 위 계수 입력 보정(고정)   // ←0.5.3 c86a77
    if !micro_taken("mv2_avoid_margin") {   // ★[08-07] 마이크로 디투어와 상호배타
    p!(base + 0xe58d45, &[0x48,0x05], 2, 4, b4(amg, 6000));   // ←0.5.3 c86a86
    }
    if !micro_taken("mv2_avoid_bias") {   // ★[08-07] 마이크로 디투어와 상호배타
    p!(base + 0xe5919f, &[0x48,0x3d], 2, 4, b4(abi, 1500));   // ←0.5.3 c86f23
    }
    // ── 우물 탈출(두 함수 미러) ──
    for a in [0xd9ec15usize, 0xe58a77] {
        p!(base + a, &[0x48,0xb8], 2, 8, sq(wr, 67_600_000_000));
    }
    p!(base + 0xd9ecab, &[0x49,0x69,0xc7], 3, 4, b4(wd, 260_000));   // ←0.5.3 d94863
    p!(base + 0xd9ecb2, &[0x4c,0x69,0xcf], 3, 4, b4(wd, 260_000));   // ←0.5.3 d9486a
    p!(base + 0xe58b3a, &[0x49,0x69,0xc5], 3, 4, b4(wd, 260_000));   // ←0.5.3 c8689a  ★재조사로 복구: wd 260000 #1
    p!(base + 0xe58b41, &[0x4d,0x69,0xcc], 3, 4, b4(wd, 260_000));   // ←0.5.3 c868a1  ★재조사로 복구: wd 260000 #2
    p!(base + 0xe07475, &[0x48,0x83,0x7b,0x18], 4, 1, b1(apm, 10));   // ←0.5.3 d87c92
    MOVE2_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("move2_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} snap{} coef{} margin{} bias{} well[r{} d{}] posmode{}\n\
             (-1=원본: 2000 400 6000 1500 / 260000 260000 / 10) @base{:#x}\n",
            ok, tot, snap, acf, amg, abi, wr, wd, apm, base));
    }
}

// ★★[08-04 신설] `buff_value.rs` 9함수(`0xcc4740`~`0xcc94c0`) byte-patch — 전투 실익의 마지막 20%.
//   근거 = RE\2026-08-04_buff_value-9함수-cc9100은전면대체-0.5.3.md
//   ★`0xcc9100`은 4종 특수 효과의 **하드코딩 점수표**이고, Some이면 **점수 전체를 대체**한다.
unsafe fn apply_bv_imm() {
    let cap160 = tune("bv_cap_main", -1);     // 본체 가치 상한(원본 160, 3쌍)
    let cap80  = tune("bv_cap_half", -1);     // DoT·오버킬 페널티 상한(원본 80, 2쌍)
    let focus  = tune("bv_focus_max", -1);    // 집중포화 인원 상한(원본 3, 2쌍). ↑=4명 이상도 배율↑
    let frad   = tune("bv_focus_radius", -1); // 집중포화 반경(원본 60000, 10사이트)
    let aflat  = tune("bv_ally_flat", -1);    // 평타 없는 아군 버프 고정값(원본 10)
    let acap   = tune("bv_ally_cap", -1);     // 아군 버프 DPS 상한(원본 90, 1쌍)
    let aout   = tune("bv_out_of_fight", -1); // 교전권 밖 버프 가치(원본 5)
    let bin    = tune("bv_b_in", -1);         // B형 교전권 안(원본 25)
    let bout   = tune("bv_b_out", -1);        // B형 교전권 밖(원본 8)
    let din    = tune("bv_d_in", -1);         // D형 교전권 안(원본 90)
    let dout   = tune("bv_d_out", -1);        // D형 교전권 밖(원본 30)
    let ccap   = tune("bv_c_cap", -1);        // C형 상한(원본 60, 1쌍)
    // ⚠원본이 음수(−100) → 전용 센티널
    let cnone  = tune("bv_c_none", -9999);
    let cnone_orig = cnone <= -1000;
    let mut sig = 0u64;
    for v in [cap160, cap80, focus, frad, aflat, acap, aout, bin, bout, din, dout, ccap, cnone] {
        sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64);
    }
    if sig == BV_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let sqp = |v: i64, orig: u64| if v < 0 { orig } else { let d = v.max(0) as u64; d.wrapping_mul(d).wrapping_add(1) };
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }
    macro_rules! pskip { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        // ⛔0.5.4 재배치 미확정 — 세기만 하고 **패치하지 않는다**.
        //    (orig_guard 는 표에 없는 RVA 를 통과시키므로, 옛 주소를 남기면 오패치 위험)
        tot += 1; let _ = ($a, $pre, $off, $w, $v);
    }}; }
    // ── 캡(전부 cmp+mov 쌍이라 둘 다 고쳐야 의미가 맞는다) ──
    for (c, m) in [(0xcc5fcfusize, 0xcc5fd6usize), (0xcdb738, 0xcdb73f), (0xcde1a4, 0xcde1ab)] {
        p!(base + c, &[0x48,0x81,0xf9], 3, 4, b4(cap160, 160));
        p!(base + m, &[0xb8], 1, 4, b4(cap160, 160));
    }
    for (c, m) in [(0xcc690busize, 0xcc690fusize), (0xcdc123, 0xcdc127)] {
        p!(base + c, &[0x48,0x83,0xf9], 3, 1, b1(cap80, 80));
        p!(base + m, &[0xb8], 1, 4, b4(cap80, 80));
    }
    // ── 집중포화 ──
    p!(base + 0xcdc37d, &[0x48,0x83,0xfa], 3, 1, b1(focus, 3));   // ←0.5.3 cc71dd
    p!(base + 0xcdc381, &[0xb8], 1, 4, b4(focus, 3));   // ←0.5.3 cc71e1
    p!(base + 0xcda68e, &[0x49,0x83,0xf8], 3, 1, b1(focus, 3));   // ←0.5.3 cc54ee
    p!(base + 0xcda692, &[0xb8], 1, 4, b4(focus, 3));   // ←0.5.3 cc54f2
    //   ⚠반경 10사이트 중 2곳(`cc71c9`·`cc5b78`)은 **REX가 없어 imm 오프셋이 1**이다.
    for a in [0xcdc232usize, 0xcdc282, 0xcdc2d1, 0xcdc320] {
        p!(base + a, &[0x41,0xb9], 2, 4, sqp(frad, 3_600_000_000));
    }
    p!(base + 0xcdc369, &[0xb9], 1, 4, sqp(frad, 3_600_000_000));   // ←0.5.3 cc71c9
    for a in [0xcdabd2usize, 0xcdac24, 0xcdac73, 0xcdacc2] {
        p!(base + a, &[0x41,0xbb], 2, 4, sqp(frad, 3_600_000_000));
    }
    p!(base + 0xcdad18, &[0xba], 1, 4, sqp(frad, 3_600_000_000));   // ←0.5.3 cc5b78
    // ── cc9100 하드코딩 점수표 ──
    p!(base + 0xcde4e1, &[0xbf], 1, 4, b4(aflat, 10));   // ←0.5.3 cc9341
    p!(base + 0xcde53c, &[0x48,0x83,0xf8], 3, 1, b1(acap, 90));   // ←0.5.3 cc939c
    p!(base + 0xcde540, &[0xbf], 1, 4, b4(acap, 90));   // ←0.5.3 cc93a0
    p!(base + 0xcde563, &[0xba], 1, 4, b4(aout, 5));   // ←0.5.3 cc93c3
    p!(base + 0xcde3f4, &[0xb8], 1, 4, b4(bin, 25));   // ←0.5.3 cc9254
    p!(base + 0xcde3f9, &[0xba], 1, 4, b4(bout, 8));   // ←0.5.3 cc9259
    p!(base + 0xcde4d2, &[0xb8], 1, 4, b4(din, 90));   // ←0.5.3 cc9332
    p!(base + 0xcde4d7, &[0xba], 1, 4, b4(dout, 30));   // ←0.5.3 cc9337
    p!(base + 0xcde63e, &[0x48,0x83,0xf9], 3, 1, b1(ccap, 60));   // ←0.5.3 cc949e
    p!(base + 0xcde642, &[0xba], 1, 4, b4(ccap, 60));   // ←0.5.3 cc94a2
    {   // mov rdx, −100
        tot += 1;
        let want = if cnone_orig { (-100i32) as u32 as u64 } else { (cnone as i32) as u32 as u64 };
        ok += patch_imm_bytes(base + 0xcde457, &[0x48,0xc7,0xc2], 3, 4, want) as u32;
    }
    BV_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("bv_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} cap[main{} half{}] focus[max{} radius{}] \
             buff[allyflat{} allycap{} out{} Bin{} Bout{} Din{} Dout{} Ccap{} Cnone{}]\n\
             (-1=원본: 160 80 / 3 60000 / 10 90 5 25 8 90 30 60 / -100은 -9999로 복원) @base{:#x}\n",
            ok, tot, cap160, cap80, focus, frad,
            aflat, acap, aout, bin, bout, din, dout, ccap, cnone, base));
    }
}

// ★★[08-04 신설] `action_eval.rs`(`0xdf5880`) byte-patch — 라인 수비 후보 점수의 **절반**.
//   근거 = RE\2026-08-04_df5880-action_eval-Around와LaneMinion동일-0.5.3.md
//   ★`Around`(5)·`LaneMinionPosition`(13)의 점수가 통째로 이 함수에서 나온다.
unsafe fn apply_ae_imm() {
    let msk  = tune("ae_none_mask", -1);      // 이 마스크에 든 태그는 None → 상위가 자체 점수 사용(원본 0x1F863)
    let rsh  = tune("ae_risk_shift", -1);     // risk ÷100 shift(원본 6). 7=위험 절반 / 5=2배
    let tsh  = tune("ae_tower_shift", -1);    // tower_risk ÷100 shift(원본 6)
    let gsh  = tune("ae_gain_shift", -1);     // GAIN ÷200 shift(원본 7). 6=이득 2배
    let bsoon= tune("ae_bonus_soon", -1);     // "곧 죽는 대상" 보너스(원본 25, 2곳)
    let bkill= tune("ae_bonus_kill", -1);     // ★"확살" 보너스(원본 140, 2곳)
    let bnear= tune("ae_bonus_near", -1);     // "거의 죽임" 보너스(원본 70, 2곳)
    let bstru= tune("ae_bonus_struct", -1);   // 특수대상 보너스(원본 80)
    let thr  = tune("ae_threat_limit", -1);   // 예상 피격 합산 필터 임계(원본 9999, 2곳)
    let mut sig = 0u64;
    for v in [msk, rsh, tsh, gsh, bsoon, bkill, bnear, bstru, thr] {
        sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64);
    }
    if sig == AE_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }
    macro_rules! pskip { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        // ⛔0.5.4 재배치 미확정 — 세기만 하고 **패치하지 않는다**.
        //    (orig_guard 는 표에 없는 RVA 를 통과시키므로, 옛 주소를 남기면 오패치 위험)
        tot += 1; let _ = ($a, $pre, $off, $w, $v);
    }}; }
    p!(base + 0xcba42e, &[0x41,0xb8], 2, 4, b4(msk, 0x1F863));   // ←0.5.3 df58de
    p!(base + 0xcba6e6, &[0x49,0xc1,0xfe], 3, 1, b1(rsh, 6));   // ←0.5.3 df5b96
    p!(base + 0xcba71f, &[0x48,0xc1,0xf8], 3, 1, b1(tsh, 6));   // ←0.5.3 df5bcf
    p!(base + 0xcba84e, &[0x48,0xc1,0xfa], 3, 1, b1(gsh, 7));   // ←0.5.3 df5cfe
    // ⚠루프 A와 B는 **같은 값인데 인코딩이 다르다**(A=`41 bc` off 2 / B=`ba` off 1).
    p!(base + 0xcbaa51, &[0x41,0xbc], 2, 4, b4(bsoon, 25));   // ←0.5.3 df5f01
    p!(base + 0xcbaf07, &[0xba], 1, 4, b4(bsoon, 25));   // ←0.5.3 df63b7
    p!(base + 0xcbab0c, &[0x41,0xbc], 2, 4, b4(bkill, 140));   // ←0.5.3 df5fbc
    p!(base + 0xcbaebe, &[0xba], 1, 4, b4(bkill, 140));   // ←0.5.3 df636e
    p!(base + 0xcbab2c, &[0x41,0xbc], 2, 4, b4(bnear, 70));   // ←0.5.3 df5fdc
    p!(base + 0xcbaeda, &[0xba], 1, 4, b4(bnear, 70));   // ←0.5.3 df638a
    p!(base + 0xcbaf80, &[0x48,0x8d,0x4a], 3, 1, b1(bstru, 80));   // ←0.5.3 df6430
    for a in [0xcba6a1usize, 0xcba6ed] {
        p!(base + a, &[0x41,0xb8], 2, 4, b4(thr, 9999));
    }
    AE_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("ae_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} mask{:#x} shift[risk{} tower{} gain{}] \
             bonus[soon{} kill{} near{} struct{}] threat_limit{}\n\
             (-1=원본: 0x1f863 / 6 6 7 / 25 140 70 80 / 9999) @base{:#x}\n",
            ok, tot, msk, rsh, tsh, gsh, bsoon, bkill, bnear, bstru, thr, base));
    }
}

// ★★[08-04 신설] 위협 디스크립터 생산자(`0xd07a60` = score_parameter.rs) byte-patch.
//   근거 = RE\2026-08-04_d07a60-위협디스크립터-생산자-스킬2버그-0.5.3.md
//   ★자리 평가가 소비하는 **위협/이득 수치의 직접 산출지**. 값은 이미 "%HP + 150 캡"이 적용된 상태.
unsafe fn apply_th_imm() {
    let smg  = tune("th_skill_margin", -1);   // 상대 스킬 사거리 가산(원본 18000, 4곳)
    let amg  = tune("th_atk_margin", -1);     // 상대 평타 확장 임계(원본 50000)
    let band = tune("th_band_margin", -1);    // 거리 3단의 중간 여유(원본 32000, **13곳**)
    let cap  = tune("th_cap", -1);            // 슬롯당 %HP 캡(원본 150, **12쌍 24곳**)
    let coll = tune("th_collect_radius", -1); // 위협 디스크립터 생성 반경(원본 200000, 12곳)
    let mut sig = 0u64;
    for v in [smg, amg, band, cap, coll] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == TH_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let sq  = |v: i64, orig: u64| if v < 0 { orig } else { let d = v.max(0) as u64; d.wrapping_mul(d) };
    let sqp = |v: i64, orig: u64| if v < 0 { orig } else { let d = v.max(0) as u64; d.wrapping_mul(d).wrapping_add(1) };
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }
    macro_rules! pskip { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        // ⛔0.5.4 재배치 미확정 — 세기만 하고 **패치하지 않는다**.
        //    (orig_guard 는 표에 없는 RVA 를 통과시키므로, 옛 주소를 남기면 오패치 위험)
        tot += 1; let _ = ($a, $pre, $off, $w, $v);
    }}; }
    macro_rules! pm { ($a:expr, $cands:expr, $w:expr, $v:expr) => {{
        tot += 1;
        let mut done = false;
        for (pre, off) in $cands.iter() {
            if !done && patch_imm_bytes($a, pre, *off, $w, $v) { done = true; }
        }
        ok += done as u32;
    }}; }
    // ── 사거리 마진 ──
    //   ⚠`d07d1a`·`d07e06`·`d07ee6`은 **바이트열이 완전히 동일** — 시그니처가 아니라 주소로 구분한다.
    for a in [0xc9ff17usize, 0xca0006, 0xca00ea] {
        p!(base + a, &[0x48,0x05], 2, 4, b4(smg, 18_000));
    }
    p!(base + 0xca070a, &[0x48,0x81,0xc1], 3, 4, b4(smg, 18_000));   // ←0.5.3 d08501
    p!(base + 0xca0716, &[0x48,0x81,0xc3], 3, 4, b4(amg, 50_000));   // ←0.5.3 d0850d
    // ── +32000 여유 13사이트 (레지스터가 6종으로 갈리지만 prefix 길이는 전부 3) ──
    // ★스택 오버플로 방지: 호출부를 펼치지 말고 **표+루프 1개**로 유지할 것.
    //   (펼치면 opt-level=1 에서 프레임이 선형으로 커져 rayon 워커 스택을 넘긴다 — 실사고)
    static TH_LEA: [(usize, &[u8], usize); 13] = [
        (0xca0725, &[0x48,0x8d,0x8d], 3),
        (0xca073e, &[0x49,0x8d,0x8e], 3),
        (0xca074d, &[0x49,0x8d,0x8b], 3),
        (0xca079f, &[0x49,0x8d,0x88], 3),
        (0xca07c6, &[0x48,0x8d,0x8f], 3),
        (0xca08da, &[0x4d,0x8d,0x91], 3),
        (0xca08ef, &[0x4d,0x8d,0xb0], 3),
        (0xca1432, &[0x49,0x8d,0x80], 3),
        (0xca1448, &[0x49,0x8d,0x8c,0x24], 4),
        (0xca1466, &[0x49,0x8d,0x8a], 3),
        (0xca1535, &[0x48,0x8d,0x8a], 3),
        (0xca154f, &[0x4d,0x8d,0x9e], 3),
        (0xca1567, &[0x4d,0x8d,0xb4,0x24], 4),
    ];
    for &(a, pre, off) in TH_LEA.iter() { p!(base + a, pre, off, 4, b4(band, 32_000)); }
    // ── 150 캡 12쌍 (cmp + mov, mov는 레지스터별 4종) ──
    // ★스택 오버플로 방지: 호출부를 펼치지 말고 **표+루프 1개**로 유지할 것.
    //   (펼치면 opt-level=1 에서 프레임이 선형으로 커져 rayon 워커 스택을 넘긴다 — 실사고)
    static TH_CAP: [(usize, &[u8], usize); 24] = [
        (0xca04fb, &[0x48,0x3d], 2),
        (0xca0501, &[0xb9], 1),
        (0xca0543, &[0x48,0x3d], 2),
        (0xca0549, &[0xb9], 1),
        (0xca0648, &[0x48,0x3d], 2),
        (0xca064e, &[0xb9], 1),
        (0xca076a, &[0x48,0x3d], 2),
        (0xca0770, &[0xb9], 1),
        (0xca08bb, &[0x48,0x3d], 2),
        (0xca08c1, &[0xbb], 1),
        (0xca0947, &[0x48,0x3d], 2),
        (0xca094d, &[0x41,0xbf], 2),
        (0xca1221, &[0x48,0x3d], 2),
        (0xca1227, &[0xb9], 1),
        (0xca12a2, &[0x48,0x3d], 2),
        (0xca12a8, &[0xb9], 1),
        (0xca133a, &[0x48,0x3d], 2),
        (0xca1340, &[0x41,0xb8], 2),
        (0xca1416, &[0x48,0x3d], 2),
        (0xca141c, &[0xba], 1),
        (0xca151d, &[0x48,0x3d], 2),
        (0xca1523, &[0x41,0xba], 2),
        (0xca15bb, &[0x48,0x3d], 2),
        (0xca15c1, &[0xbb], 1),
    ];
    for &(a, pre, off) in TH_CAP.iter() { p!(base + a, pre, off, 4, b4(cap, 150)); }
    // ── 디스크립터 생성 반경 200000 (12곳, +1 유무 2종) ──
    for a in [0xca8e05usize, 0xca8e40, 0xca8ed4, 0xca8f10, 0xca8fa4, 0xca8fe0, 0xca9077, 0xca90ad] {
        p!(base + a, &[0x48,0xb8], 2, 8, sqp(coll, 0x9_502F_9001));
    }
    for a in [0xca9143usize, 0xca9178, 0xca94b0, 0xca94e0] {
        p!(base + a, &[0x48,0xb8], 2, 8, sq(coll, 0x9_502F_9000));
    }
    TH_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("th_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} skill_margin{} atk_margin{} band{} cap{} collect{}\n\
             (-1=원본: 18000 50000 32000 150 200000) @base{:#x}\n",
            ok, tot, smg, amg, band, cap, coll, base));
    }
}

// ★★[08-04 신설] 위협 감지/후퇴 트리거(`0xd63d60` = handler.rs:1620~1737) + 정글 진행 게이트(`0xdff660`).
//   근거 = RE\2026-08-04_층1플랜결정기-d63d60은위협감지-plan0이주출력-0.5.3.md
//           RE\2026-08-04_Blackboard전필드-층2매퍼-소함수5종-0.5.3.md
//   ★`d63d60` = "내가 곧 죽는데 그 원인이 되는 적"을 찾는 함수 = **후퇴/귀환 판단의 입력기**.
//     임계 3종이 스탯 s로 정해진다: A = 160 − 0.8s / B = 45 + 0.45s / C = 15 + 0.35s
unsafe fn apply_rt_imm() {
    let ka = tune("rt_a_slope", -1);      // A식 기울기(원본 −800). ⚠음수 원본 — 아래 센티널 처리
    let ia = tune("rt_a_base", -1);       // A식 절편(원본 80000, ÷1000)
    let oa = tune("rt_a_offset", -1);     // A 하한 오프셋(원본 80). ↑=후퇴 판정 **덜** 남
    let kb = tune("rt_b_slope", -1);      // B식 기울기(원본 450). ↑=후퇴 **더** 자주
    let ib = tune("rt_b_base", -1);       // B식 절편(원본 45)
    let kc = tune("rt_c_slope", -1);      // C식 기울기(원본 350). ↑=후퇴 **덜**
    let ic = tune("rt_c_base", -1);       // C식 절편(원본 15)
    let dl = tune("rt_deadline_min", -1); // 위협 유효기간 하한(원본 60, 1쌍)
    let jf = tune("jg_hp_fight", -1);     // 전투 가능할 때 정글 진행 HP%(원본 21)
    let jn = tune("jg_hp_nofight", -1);   // 전투 불가일 때 정글 진행 HP%(원본 41)
    let mut sig = 0u64;
    for v in [ka, ia, oa, kb, ib, kc, ic, dl, jf, jn] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == RT_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }
    macro_rules! pskip { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        // ⛔0.5.4 재배치 미확정 — 세기만 하고 **패치하지 않는다**.
        //    (orig_guard 는 표에 없는 RVA 를 통과시키므로, 옛 주소를 남기면 오패치 위험)
        tot += 1; let _ = ($a, $pre, $off, $w, $v);
    }}; }
    {   // A식 기울기만 원본이 음수(−800)라 "음수 = 원본" 규약을 못 쓴다 → 값 그대로 적용.
        tot += 1;
        let want = if ka == -1 { (-800i32) as u32 as u64 } else { (ka as i32) as u32 as u64 };
        ok += patch_imm_bytes(base + 0xeb3118, &[0x69,0xc1], 2, 4, want) as u32;
    }
    p!(base + 0xeb311e, &[0x05], 1, 4, b4(ia, 80_000));   // ←0.5.3 d6431e
    p!(base + 0xeb312e, &[0x83,0xc0], 2, 1, b1(oa, 80));   // ←0.5.3 d6432e
    p!(base + 0xeb3135, &[0x69,0xc1], 2, 4, b4(kb, 450));   // ←0.5.3 d64335
    p!(base + 0xeb314a, &[0x83,0xc0], 2, 1, b1(ib, 45));   // ←0.5.3 d6434a
    p!(base + 0xeb3151, &[0x69,0xc1], 2, 4, b4(kc, 350));   // ←0.5.3 d64351
    p!(base + 0xeb3166, &[0x83,0xc0], 2, 1, b1(ic, 15));   // ←0.5.3 d64366
    //   deadline 하한은 `cmp 61` + `mov 60` 쌍이라 비교값을 +1로 맞춰야 한다.
    p!(base + 0xeb42d5, &[0x48,0x83,0xf9], 3, 1, if dl < 0 { 61 } else { (dl.max(0).min(0x7e) + 1) as u64 });   // ←0.5.3 d654d5
    p!(base + 0xeb42d9, &[0xba], 1, 4, b4(dl, 60));   // ←0.5.3 d654d9
    // ── 정글 진행 HP% ──
    p!(base + 0xe621d4, &[0x48,0x83,0xf8], 3, 1, b1(jf, 21));   // ←0.5.3 dffebc
    p!(base + 0xe621f1, &[0x48,0x83,0xf8], 3, 1, b1(jn, 41));   // ←0.5.3 dfff00
    RT_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("rt_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} A[slope{} base{} off{}] B[slope{} base{}] C[slope{} base{}] \
             deadline{} jungle[fight{} nofight{}]\n\
             (-1=원본: -800 80000 80 / 450 45 / 350 15 / 60 / 21 41) @base{:#x}\n",
            ok, tot, ka, ia, oa, kb, ib, kc, ic, dl, jf, jn, base));
    }
}

// ★★[08-03 신설] `line_defense` 후보 점수 함수(`0xc66800` = line_defense.rs:969~1020) byte-patch.
//   근거 = RE\2026-08-03_c66800-line_defense-후보점수함수-0.5.3.md
//   ★구조: 태그 5·13은 `df5880` 조기반환이 가로채 이 함수 로직을 안 탄다.
//          태그 3·8·14는 `c7b730` 출력 단독. 태그 15만 `c7b730 + lane_economy + 스킬가치`.
unsafe fn apply_ldsc_imm() {
    let lsv  = tune("ldsc_vision_mem", -1);   // 이 함수 안의 적 최근목격 유효틱(원본 120, 3곳)
                                              // ⚠공유 헬퍼(12b6e78·dc5090)는 **전역 영향**이라 제외했다.
    let lfac = tune("ldsc_skill_factor", -1); // 전투태그 스킬가치 계수 기저(원본 100)
    let lem  = tune("ldsc_early_mask", -1);   // ★`df5880` 조기반환 태그 마스크(원본 0x1F863).
                                              //   비트를 세우면 그 태그가 A+B+C 경로로 돌아온다.
    // ⚠원본이 음수(−99999) → 전용 센티널
    let lnul = tune("ldsc_lost_target", -9999);
    let lnul_orig = lnul <= -1000;
    let mut sig = 0u64;
    for v in [lsv, lfac, lem, lnul] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == LDSC_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }
    macro_rules! pskip { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        // ⛔0.5.4 재배치 미확정 — 세기만 하고 **패치하지 않는다**.
        //    (orig_guard 는 표에 없는 RVA 를 통과시키므로, 옛 주소를 남기면 오패치 위험)
        tot += 1; let _ = ($a, $pre, $off, $w, $v);
    }}; }
    for a in [0xd7fa62usize, 0xd7fad2, 0xd7fb3d] {
        p!(base + a, &[0x49,0x83,0xc6], 3, 1, b1(lsv, 120));
    }
    p!(base + 0xd7fe07, &[0x83,0xc2], 2, 1, b1(lfac, 100));   // ←0.5.3 c66b95
    p!(base + 0xcba42e, &[0x41,0xb8], 2, 4, b4(lem, 0x1F863));   // ←0.5.3 df58de
    {   // mov rcx, −99999 (48 c7 c1 imm32)
        tot += 1;
        let want = if lnul_orig { (-99999i32) as u32 as u64 } else { (lnul as i32) as u32 as u64 };
        ok += patch_imm_bytes(base + 0xd7fe56, &[0x48,0xc7,0xc1], 3, 4, want) as u32;
    }
    LDSC_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("ldsc_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} vision{} skill_factor{} early_mask{:#x} lost_target{}\n\
             (-1=원본: 120 100 0x1f863 / -99999는 -9999로 복원) @base{:#x}\n",
            ok, tot, lsv, lfac, lem, lnul, base));
    }
}

/// 원본 바이트열 ↔ 대체 바이트열을 **양방향으로** 뒤집는 패치(길이 동일).
/// `patch_imm_bytes`는 prefix가 원본일 때만 통과하므로, 되돌리기(대체→원본)가 불가능하다.
/// 이 헬퍼는 현재 바이트가 둘 중 무엇이든 목표 상태로 맞춘다(= 재적용·복원 모두 안전).
#[inline] unsafe fn patch_toggle_bytes(addr: usize, orig: &[u8], alt: &[u8], want_alt: bool) -> bool {
    let n = orig.len();
    if n == 0 || n != alt.len() || !readable(addr, n) { return false; }
    let want: &[u8] = if want_alt { alt } else { orig };
    let cur_is_orig = (0..n).all(|i| rd_u8(addr + i) == orig[i]);
    let cur_is_alt  = (0..n).all(|i| rd_u8(addr + i) == alt[i]);
    if !cur_is_orig && !cur_is_alt { return false; }   // 제3의 바이트열 = RVA 어긋남 → 건드리지 않음
    if (0..n).all(|i| rd_u8(addr + i) == want[i]) { return true; }   // 이미 목표 상태
    let mut old: u32 = 0;
    if VirtualProtect(addr, n, 0x40, &mut old) == 0 { return false; }
    core::ptr::copy_nonoverlapping(want.as_ptr(), addr as *mut u8, n);
    VirtualProtect(addr, n, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, n);
    true
}

// ★★[08-03 신설] 08-03 RE 3건에서 새로 확보한 레버 묶음 byte-patch.
//   근거 = REPORT\tfm2_ai_adjust\RE\2026-08-03_judge_noise_ratio-오더가중치-출처규명-0.5.3.md §D
//        + …_line_defense-2회차평가구간-및-battle.rs-0.5.3.md §3·§5
//        + …_팀모드-0x4de-결정로직-채팅프로토콜-0.5.3.md §E-2
//   ★유효성: 세 층 모두 **모드가 대체하지 않는 구간**(auction 0xd5f500 / battle.rs 0xca8a10 /
//     line_defense 2회차 c60c00~ / chat.rs 0xd53f40) ⟹ byte-patch가 실제로 먹는다(§12.21(3) 규칙 통과).
//   ⚠전 키 기본 -1 = 원본(무개입). 다중 사이트 키는 "N곳 중 몇 곳 성공"을 auction_imm.txt에 기록.
//
//   ① 판단력 노이즈(`judge_noise_ratio`, unit_ai+0x1340[11]) — ★행동 성향의 최상위 레버.
//      원본: 플랜이 바뀔 때마다 11개 카테고리 가중치를 각각 uniform[1000−noise, 1000+noise]로 재롤.
//      noise = (900 − 9×판단력)/2  ⟹ 판단력 100이면 0(왜곡 없음), 0이면 ±450(0.55×~1.45×).
//      · au_noise_off=1 → 롤 자체를 무력화(전 카테고리 정확히 1000) = "판단력과 무관하게 일관된 성향".
//      · au_noise_amp  → 진폭 상수(원본 900). ⚠**900 미만 금지**(9×판단력보다 작으면 부호없이 언더플로
//        → noise가 거대해져 점수 체계가 붕괴). 그래서 900으로 하한 클램프한다. 줄이려면 au_noise_off를 쓸 것.
//      · au_score_center → lo/hi의 중심(원본 1000). 올리면 **모든 오더 점수가 일괄 증폭**된다.
//   ② battle.rs(교전 판단) HP·거리 임계. ③ line_defense 2회차 추격·아군근접 임계. ④ 팀모드 자동취소 마스크.
// ════════════════════════════════════════════════════════════════════════════════
// ★[08-05 신설] 2026-08-05 조사 4건에서 새로 배선 가능해진 노브들.
//   근거 RE = `mods_report\tfm2_ai_adjust\RE\2026-08-05_*` 4건.
//   ⚠**사이트 18곳 전부 정적 바이트 대조 통과분만** 넣었다(prefix·imm 오프셋·원본값 3중 확인).
//   ⚠상수값이 같아도 의미가 다르면 절대 묶지 않았다 — 오늘 `add rax,0x29`(포인터 산술)를
//     HP 임계 41로 오인해 배선돼 있던 사고가 있었다(03_시행착오 참조).
// ════════════════════════════════════════════════════════════════════════════════
unsafe fn apply_new_imm() {
    // ── ① 적 위치 추정 모델 (0xcef270 → 0xc42db0). 지금까지 반경(dd_f22e80_margin) 하나만 노출돼 있었다 ──
    //    ★이 셋은 **실전 경로(게임모드 ≠ 2)** 상수다. 08-05 프로브에서 `G.vt[0x30]`이 100% 0으로 측정됐으므로
    //      일반 경기에서 실제로 도는 쪽이 여기다. (모드 2 전용 상수 900/1000/600틱은 실측상 死라 노출하지 않았다.)
    let epsb = tune("eg_spread_base", -1);   // 확산 기본항(원본 3000). ↑=적 위치를 더 넓게 의심
    let epdr = tune("eg_disk_radius", -1);   // 추정 원판 기본 반경(원본 40000, 2곳 = 값·값+1)
    let eprc = tune("eg_radius_cap",  -1);   // 추정 반경 상한(원본 300000). 넘으면 그 적을 아예 후보에서 제외
    // ── ② 시전 후보 2차 검열 (0xc65710). line_defense(sub_plan 2) 전용 ──
    let cfrn = tune("cf_risk_near", -1);     // 가까운 경로(≤2초)의 목적지 위험 임계(원본 9). ★가장 자주 걸리는 컷
    let cfrf = tune("cf_risk_far",  -1);     // 먼 경로(>2초)의 목적지 위험 임계(원본 25)
    let cfdp = tune("cf_dmg_pct",   -1);     // 예상 피격이 내 HP의 몇 %면 시전 포기(원본 35)
    let cfrp = tune("cf_reach_pad", -1);     // 사거리 여유 — 평타·스킬·스킬2(원본 15000)
    let cfru = tune("cf_reach_pad_ult", -1); // 사거리 여유 — 궁 전용(원본 150000)
    let cfoff= tune("cf_filter_off", -1);    // 1 = 2차 검열 전면 무효화(12→17). ⚠12 미만 금지(JT 음수 = 크래시)
    let cffk = tune("cf_flee_kill_off", -1); // 1 = "후퇴 의사가 있으면 시전 후보 몰살"을 끈다(원본 = 몰살함). au_noise_off·cf_filter_off와 같은 1=끔 규약
    // ── ③ 1차 점수컷 (0xc3cd60). 두 사이트가 값·값−1 쌍이라 반드시 같이 움직인다 ──
    let cssf = tune("ld_score_floor", -1);   // 후보 점수 하한의 절댓값(원본 30 = score ≥ −30이면 통과)
    // ── ④ 경매 재선택 (0xd5f500 Stage-2/3) ──
    let recp = tune("re_cast_promote", -1);  // Stage-2 승격 대상 태그 개수(원본 2 = 15·16·17). 3이면 **궁(18)도 승격**
    let retp = tune("re_trace_pad", -1);     // Stage-3 추격 포기 여유거리(원본 25000)
    let regs = tune("re_gate_subplan", -1);  // Stage-3 사전게이트 `sub_plan <= N`(원본 1)
    // ── ⑤ 전역 궁 요청(아군 채팅) 오버라이드 (0xd5f500 진입부) ──
    let gulv = tune("gu_level", -1);         // 궁 해금 레벨 전제(원본 5)
    let gumem= tune("gu_enemy_mem", -1);     // 억제 판정의 적 목격 기억(원본 120틱)
    let gusr = tune("gu_suppress_r", -1);    // 억제 반경(원본 150000). 이 안에 적이 보이면 전역 궁 발동 안 함
    let mut sig = 0u64;
    for v in [epsb, epdr, eprc, cfrn, cfrf, cfdp, cfrp, cfru, cfoff, cffk,
              cssf, recp, retp, regs, gulv, gumem, gusr] {
        sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64);
    }
    if sig == NEWIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }
    macro_rules! pskip { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        // ⛔0.5.4 재배치 미확정 — 세기만 하고 **패치하지 않는다**.
        //    (orig_guard 는 표에 없는 RVA 를 통과시키므로, 옛 주소를 남기면 오패치 위험)
        tot += 1; let _ = ($a, $pre, $off, $w, $v);
    }}; }
    // ① 적 위치 추정
    p!(base + 0xc8c561, &[0x81,0xc2], 2, 4, b4(epsb, 3000));   // ←0.5.3 c43101
    p!(base + 0xc8c586, &[0x48,0x8d,0x9e], 3, 4, b4(epdr, 40000));   // ←0.5.3 c43126
    // ⚠짝 사이트 — 여기는 **반경+1**(gen_range 상한). 하나만 바꾸면 난수 범위가 어긋난다.
    p!(base + 0xc8c7d9, &[0x48,0x81,0xc1], 3, 4, if epdr < 0 { 40001 } else { b4(epdr, 40000) + 1 });   // ←0.5.3 c43379
    p!(base + 0xc8c58d, &[0x48,0x81,0xfb], 3, 4, b4(eprc, 300000));   // ←0.5.3 c4312d
    // ② 시전 후보 2차 검열
    p!(base + 0xd7ed2c, &[0x48,0x83,0xf9], 3, 1, b1(cfrn, 9));   // ←0.5.3 c65f3e
    p!(base + 0xd7ecdf, &[0x48,0x83,0xf9], 3, 1, b1(cfrf, 25));   // ←0.5.3 c65ef1
    p!(base + 0xd7ecb7, &[0x4c,0x6b,0xc0], 3, 1, b1(cfdp, 35));   // ←0.5.3 c65ec9
    p!(base + 0xd7e573, &[0xb9], 1, 4, b4(cfrp, 15000));   // ←0.5.3 c65793
    p!(base + 0xd7e59e, &[0xb8], 1, 4, b4(cfru, 150000));   // ←0.5.3 c657be
    // ⚠12 미만은 JT 인덱스가 음수가 되어 크래시한다 — 켜기(17)/원본(12) 두 값만 허용한다.
    p!(base + 0xd7e568, &[0x3c], 1, 1, if cfoff == 1 { 17 } else { 12 });   // ←0.5.3 c65788
    // `jne +0x0d`(= 도주중이면 이 후보 제거) → NOP 2바이트면 그 조건만 꺼진다.
    let flee_ok = patch_toggle_bytes(base + 0xd7ed2a, &[0x75,0x0d], &[0x90,0x90], cffk == 1);
    // ③ 1차 점수컷 — 두 사이트가 (−v, −v−1) 쌍
    let (f0, f1) = if cssf < 0 { (0xe2u64, 0xe1u64) } else {
        let v = cssf.clamp(0, 120) as u64; ((256 - v) & 0xff, (256 - v - 1) & 0xff)
    };
    p!(base + 0xc865aa, &[0x48,0x83,0xf8], 3, 1, f0);   // ←0.5.3 c3cf8d
    p!(base + 0xc865b9, &[0x48,0x83,0xf8], 3, 1, f1);   // ←0.5.3 c3cf9e
    // ④ 경매 재선택
    p!(base + 0xeb00ba, &[0x3c], 1, 1, b1(recp, 2));   // ←0.5.3 d61e4b
    p!(base + 0xeb0778, &[0x48,0x05], 2, 4, b4(retp, 25000));   // ←0.5.3 d6220d
    p!(base + 0xeb073e, &[0x83,0x38], 2, 1, b1(regs, 1));   // ←0.5.3 d621d3
    // ⑤ 전역 궁 오버라이드
    p!(base + 0xead84d, &[0x48,0x83,0xb9,0xb0,0x05,0x00,0x00], 7, 1, b1(gulv, 5));   // ←0.5.3 d5f93b
    p!(base + 0xead9a7, &[0x48,0x83,0xc3], 3, 1, b1(gumem, 120));   // ←0.5.3 d5faa7
    // ⚠거리 그대로가 아니라 **d²+1**로 인코딩된 자리다(원본 150000 → 22,500,000,001).
    pskip!(base + 0xd5f9fa, &[0x48,0xb9], 2, 8,   // ⛔0.5.4 미확정: 시그 2→1 / 완화 7→2 (골격 84%)
       if gusr < 0 { 22_500_000_001u64 } else { let d = gusr.max(0) as u64; d.wrapping_mul(d).wrapping_add(1) });
    NEWIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("new_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} flee_toggle={} @base{:#x}\n\
             ep[spread{} disk{} cap{}] | cf[near{} far{} dmg{} pad{} padult{} off{} fleekill{}] | cs[floor{}] | re[promote{} pad{} gate{}] | gu[lv{} mem{} r{}]\n\
             (-1=원본: 3000 40000 300000 / 9 25 35 15000 150000 / 30 / 2 25000 1 / 5 120 150000)\n",
            ok, tot, flee_ok, base,
            epsb, epdr, eprc, cfrn, cfrf, cfdp, cfrp, cfru, cfoff, cffk, cssf, recp, retp, regs, gulv, gumem, gusr));
    }
}

unsafe fn apply_auction_imm() {
    let noff = tune("au_noise_off", -1);      // 1 = 판단력 노이즈 롤 무력화(원본 = 롤 수행)
    let namp = tune("au_noise_amp", -1);      // 노이즈 진폭 상수(원본 900, 하한 900)
    let ctr  = tune("au_score_center", -1);   // 가중치 중심(원본 1000)
    let bhpf = tune("bt_hp_flee", -1);        // battle tag8(후퇴/추격) HP% 임계(원본 21)
    let bhpg = tune("bt_hp_gate", -1);        // battle HP% 2차 임계(원본 41, 2곳)
    let bstp = tune("bt_chase_stop", -1);     // battle 접근 정지 반경(원본 15000, 5곳)
    let bkep = tune("bt_chase_keep", -1);     // battle 추격 유지 거리(원본 80000)
    let bvis = tune("bt_vision_mem", -1);     // battle 마지막 목격 유효 틱(원본 120, ★0.5.4=11곳)
    let lstp = tune("ld_chase_stop", -1);     // line_defense 접근 정지 반경(원본 15000, 4곳)
    let lnear= tune("ld_ally_near", -1);      // 아군 "근접" 판정 거리(원본 160000, 5곳)
    let livn = tune("ld_intervene", -1);      // 개입 최소 거리(원본 50000)
    let lvis = tune("ld_vision_mem", -1);     // line_defense 마지막 목격 유효 틱(원본 120)
    let lest = tune("ld_est_base", -1);       // AI 추정 오차 하한(원본 10, ↑ = 추정이 정확해짐)
    let tcnc = tune("tm_cancel_mask", -1);    // 자동취소 대상 팀모드 비트마스크(원본 0xb00 = mode 8·9·11)
    // ── 08-03 신규: line_defense 1회차 구간에서 새로 노출한 값들 ──
    let larn = tune("ld_around_range", -1);   // 목표물 접근 사거리(원본 80000, 1·2회차 합쳐 7곳)
    let lard = tune("ld_around_delay", -1);   // 접근 액션 유지 시간(원본 5, 3곳)
    let lmsk = tune("ld_mode_mask", -1);      // position_eval purpose=8을 주는 게임모드 마스크(원본 0x1a1, 3곳)
    let lmvp = tune("ld_move_pct", -1);       // 이동거리 예측식의 기준 퍼센트(원본 100, 4곳). ↑=위협 반경 확대
    let lthr = tune("ld_threat_state", -1);   // 적 위협 스캔이 반응하는 행동 enum(원본 13)
    let lrnd = tune("ld_rand_min", -1);       // 무작위 대체가 발동할 최소 후보 수(원본 2). ↑=랜덤 발동 감소
    let mut sig = 0u64;
    for v in [noff, namp, ctr, bhpf, bhpg, bstp, bkep, bvis, lstp, lnear, livn, lvis, lest, tcnc,
              larn, lard, lmsk, lmvp, lthr, lrnd] {
        sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64);
    }
    if sig == AUCTIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    // 거리 d → (d²)>>shift 인코딩(원본 상수와 같은 형식). v<0이면 원본 유지.
    let dsh = |v: i64, orig: u64, shift: u32| if v < 0 { orig } else {
        let d = v.max(0) as u64; (d.wrapping_mul(d)) >> shift
    };
    let mut ok = 0u32; let mut tot = 0u32;
    // 클로저로 만들면 ok/tot를 가변 차용한 채 아래에서 직접 대입까지 해 차용 검사에 걸린다 → 매크로로 처리.
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }
    macro_rules! pskip { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        // ⛔0.5.4 재배치 미확정 — 세기만 하고 **패치하지 않는다**.
        //    (orig_guard 는 표에 없는 RVA 를 통과시키므로, 옛 주소를 남기면 오패치 위험)
        tot += 1; let _ = ($a, $pre, $off, $w, $v);
    }}; }
    // ── ① 판단력 노이즈 (auction 0xd5f500) ──
    //    cmovne rsi,rdx (48 0f 45 f2) → nop dword ptr [rax] (0f 1f 40 00) 이면 noise=0 고정.
    let noise_ok = patch_toggle_bytes(base + 0xeadfff,
                                      &[0x48,0x0f,0x45,0xf2], &[0x0f,0x1f,0x40,0x00], noff == 1);
    p!(base + 0xeadfee, &[0xba], 1, 4, if namp < 0 { 900 } else { (namp.max(900) as u64) & 0xffff_ffff });   // ←0.5.3 d5febc
    p!(base + 0xeae027, &[0xbb], 1, 4, b4(ctr, 1000));               // mov ebx,1000  (lo 중심)   // ←0.5.3 d5fefc
    p!(base + 0xeae02f, &[0x48,0x81,0xc6], 3, 4, b4(ctr, 1000));     // add rsi,1000  (hi 중심)   // ←0.5.3 d5ff04
    // ── ② battle.rs (0xca8a10) ──
    p!(base + 0xda6514, &[0x48,0x83,0xf8], 3, 1, b1(bhpf, 21));   // ←0.5.3 cab663
    // ⛔[08-05 정정] 예전엔 `0xca920b`도 같이 패치했는데 **거긴 HP 임계가 아니다.**
    //   `ca920b: add rax,0x29` = `&sub_plan + 0x29` = **`&BattleSubPlan.with_dive` 포인터 산술**(`0xc497b0` 인자 `+0x30`).
    //   41 이외의 값을 넣으면 BattleSubPlan의 엉뚱한 필드를 가리키게 된다(0x10~0x17이면 `goal` 내부).
    //   상수 41이 우연히 겹쳐 HP 임계처럼 보였을 뿐. 진짜 HP 41% 비교는 `0xcab1ef` 한 곳뿐이다.
    //   근거 = RE\2026-08-05_battle.rs-JT3개-goal8분기-크래시원인확정-모드실버그2건-0.5.3.md §2-3 Q5b / §③B
    p!(base + 0xda608d, &[0x48,0x83,0xf8], 3, 1, b1(bhpg, 41));     // cmp rax,41 — HP%≥41이면 아군수 +2   // ←0.5.3 cab1ef
    // ★[08-05] emit 사이트 전수(RE\2026-08-05_battle.rs-JT3개…) 대조 결과 **4곳이 아니라 6곳**이었다.
    //   빠져 있던 `0xcab77e`(emit#13) · `0xcabb22`(emit#17)는 Q14/15 사거리 판정에서 갈라지는 경로라
    //   일반 경기에선 잘 안 보이지만 같은 tag 0xE 접근정지 반경이다 = 반쪽만 먹던 노브.
    for a in [0xda49ddusize, 0xda4a86, 0xda4bf2, 0xda4f67, 0xda6742, 0xda6898] {
        p!(base + a, &[0x48,0xc7,0x85], 7, 4, b4(bstp, 15000));
    }
    // ⚠이 사이트만 명령 형태가 다르다 — `add rcx,15000`(48 81 c1) 이라 prefix·imm 오프셋이 위와 다름.
    //   08-03 정적 검증 전까지 위 루프에 섞여 있어 **한 번도 안 걸리고 있었다**.
    p!(base + 0xda7417, &[0x48,0x81,0xc1], 3, 4, b4(bstp, 15000));   // ←0.5.3 cac3be
    // ★[08-05] 여기도 1곳이 아니라 2곳 — `0xcac136`(emit#18, arm4 근접 처리)이 빠져 있었다.
    for a in [0xda8e06usize, 0xda8f8a] {
        p!(base + a, &[0x48,0xc7,0x85], 7, 4, b4(bkep, 80000));
    }
    // ⚠7사이트가 **레지스터가 제각각**이라 prefix가 4종이다(`add r13/r14/rsi/r15, 0x78`).
    //   08-03 정적 검증 전까지 2종만 시도해 **7곳 중 4곳이 안 걸리고 있었다**.
    // ★0.5.4: **7곳 → 11곳**으로 늘었다(반쪽 노브). 신규 4곳 = da7a2b·da8c8e·da81f7·da8d32.
    // ★★[08-07 결함 수정] prefix 목록이 실제 사이트와 어긋나 **11곳 중 1곳(0xda5234)이 안 걸리고 있었다**.
    //   exe 전수 추출(`v54\micro_win.py`) 실측 = r15×5(`49 83 c7`) · rsi×4(`48 83 c6`) · r14×1(`49 83 c6`) ·
    //   **rdi×1(`48 83 c7`)**. 목록엔 rdi 가 없고 대신 **아무 사이트도 안 쓰는 r13(`49 83 c5`)** 이 들어 있었다.
    //   ⟹ `applied=10/11` 이었는데 `ok += done` 집계만 보면 알 수 없다(같은 계열 사고 3번째 — 08-03 "7곳 중 4곳",
    //   08-07 "알리아스 중복 사이트"). ★교훈 = **prefix 목록은 짐작이 아니라 exe 실측으로 채운다.**
    // ★[08-07] 마이크로 디투어가 11곳을 통째로 가져갔으면 여기선 건드리지 않는다(상호배타).
    //   그쪽은 **키 단위 all-or-nothing** 이라 "일부만 가져간" 중간 상태가 없다.
    if !micro_taken("bt_vision_mem") {
    for a in [0xda4625usize, 0xda5234, 0xda5dae, 0xda791b, 0xda79a3, 0xda7a2b, 0xda8c8e,
              0xda80e7, 0xda816f, 0xda81f7, 0xda8d32] {
        tot += 1;
        let mut done = false;
        for pre in [[0x49u8,0x83,0xc7], [0x48,0x83,0xc6], [0x49,0x83,0xc6], [0x48,0x83,0xc7]].iter() {
            if !done && patch_imm_bytes(base + a, pre, 3, 1, b1(bvis, 120)) { done = true; }
        }
        ok += done as u32;
    }
    }   // ← micro_taken("bt_vision_mem") 가드 끝
    // ── ③ line_defense 2회차 평가 구간 ──
    // ★★[08-03 버그 수정] `0xc62377`·`0xc6242d`는 **+0xb 어긋난 주소**였다. 그 자리는
    //   `mov [rbp+0x4a0], 5`(접근 유지 시간)이고, prefix(48 c7 85)가 우연히 같아 **패치는 성공했다** —
    //   즉 원본이 5인 슬롯에 15000을 써 넣고 있었다(기본 설정에서도 동작이 바뀌던 실버그).
    //   올바른 주소 = `0xc6236c`·`0xc62422` (`mov [rbp+0x498], 15000`).
    for a in [0xd7b47cusize, 0xd7b210, 0xd7b314, 0xd7b3d1] {
        p!(base + a, &[0x48,0xc7,0x85], 7, 4, b4(lstp, 15000));
    }
    for a in [0xd7a5c8usize, 0xd7a631, 0xd7a69d, 0xd7a709, 0xd7a76d] {
        tot += 1;
        let v = dsh(lnear, 390625, 16);                             // (160000²)>>16 = 390625
        ok += (patch_imm_bytes(base + a, &[0x49,0x81,0xfa], 3, 4, v)
            || patch_imm_bytes(base + a, &[0x48,0x3d], 2, 4, v)) as u32;
    }
    // ★★[08-03 정정] `line_defense`의 "1회차/2회차"는 순차 단계가 아니라 **소스상 별개의 두 헬퍼가
    //   나란히 인라인된 것**이고(패닉 라인 `:168~172` vs `:220~254`), 1회차는 **무조건 진입**한다.
    //   그런데 아래 두 노브는 지금까지 **2회차 사이트만** 패치하고 있었다 = 효과가 반쪽이었다.
    //   → 1회차 동일 상수 사이트를 같이 패치한다. (RE\2026-08-03_line_defense-1회차구간-c5e160)
    // ↓0.5.4: prefix 가 사이트마다 달라져 루프를 펼침(원래 `for a in [..]`)
    p!(base + 0xd77a06, &[0x48,0x81,0xf9], 3, 4, dsh(livn, 9765625, 8));   // ←0.5.3 c5f059
    p!(base + 0xd7a254, &[0x48,0x81,0xfa], 3, 4, dsh(livn, 9765625, 8));   // ←0.5.3 c61784
    p!(base + 0xd778b8, &[0x49,0x83,0xc5], 3, 1, b1(lvis, 120));           // ← 1회차(신규)   // ←0.5.3 c5eee7
    p!(base + 0xd7c9a1, &[0x49,0x83,0xc4], 3, 1, b1(lvis, 120));   // ←0.5.3 c63b87
    // c61667 = `add rdi,0x78` — 08-03 exe 정적 검증으로 바이트열 확정(같은 120틱 사이트, 세 번째).
    p!(base + 0xd7a14a, &[0x48,0x83,0xc3], 3, 1, b1(lvis, 120));   // ←0.5.3 c61667
    p!(base + 0xd7a8aa, &[0x83,0xc1], 2, 1, b1(lest, 10));                 // add ecx,10   // ←0.5.3 c61cb4
    // ── ③-b line_defense 1회차 전용 상수 (08-03 신규 노출) ──
    //   ★`ld_around_range`는 1·2회차 합쳐 **7사이트** — 지금까지 완전 미노출이던 값이다.
    for a in [0xd77ab2usize, 0xd77d2c, 0xd780f2, 0xd786a2, 0xd7a3c3, 0xd7be13, 0xd7bec7] {
        p!(base + a, &[0x48,0xc7,0x85], 7, 4, b4(larn, 80000));
    }
    // ★[08-05 감사] 3곳만 잡고 있었다 — 실제는 **7곳**이고, 바로 위 `ld_around_range` 7사이트의 **+11B 짝**이다.
    for a in [0xd77abdusize, 0xd78025, 0xd780fd, 0xd7b487, 0xd7b6f8, 0xd7bed2, 0xd7c1e6] {
        p!(base + a, &[0x48,0xc7,0x85], 7, 4, b4(lard, 5));
    }
    // ★[08-05 감사] 3곳 → **6곳**. 그리고 이 마스크가 고르는 건 게임 모드가 아니라 **경기 페이즈**(`u8[S+0x38]`, 0~8)다
    //   — 게임 모드는 0/1/2 셋뿐인데 마스크가 bit 8까지 쓴다. `lt_phase_mask`·`pl_serpen_phase_mask`와 **같은 필드**.
    p!(base + 0xd76fbc, &[0xba], 1, 4, b4(lmsk, 0x1a1));   // ←0.5.3 c5e61a
    p!(base + 0xd77370, &[0x41,0xb8], 2, 4, b4(lmsk, 0x1a1));   // ←0.5.3 c5e9b3
    p!(base + 0xd78133, &[0xba], 1, 4, b4(lmsk, 0x1a1));   // ←0.5.3 c5f664
    for a in [0xd796c0usize, 0xd79a56, 0xd7d29c] {
        p!(base + a, &[0xba], 1, 4, b4(lmsk, 0x1a1));
    }
    // ★[08-05 감사] 4곳 → **10곳**. 전부 동일 관용구 `movsxd rax,[r+0x458]; add rax,0x64; imul [r+0x668]`.
    for a in [0xd76cdausize, 0xd76f00, 0xd788db, 0xd78929,
              0xd78ef7, 0xd78f2a, 0xd7973b, 0xd7999c, 0xd7b55e, 0xd7b65d] {
        p!(base + a, &[0x48,0x83,0xc0], 3, 1, b1(lmvp, 100));
    }
    // ★[08-05 감사] `0xc60d6c`는 `0xc5e3cc`의 완전 클론(`cmp qword[r?+0x68],13; sete`)인데 미배선이었다.
    p!(base + 0xd76d9c, &[0x49,0x83,0x7f,0x68], 4, 1, b1(lthr, 13));   // ←0.5.3 c5e3cc
    p!(base + 0xd797ec, &[0x48,0x83,0x7e,0x68], 4, 1, b1(lthr, 13));   // ←0.5.3 c60d6c
    p!(base + 0xd794bc, &[0x48,0x83,0xfa], 3, 1, b1(lrnd, 2));   // ←0.5.3 c60a0c
    // ── ④ 팀모드 자동취소 마스크 (chat.rs / modes.rs) ──
    for a in [0xea1b91usize, 0xe9683a] {
        p!(base + a, &[0xb9], 1, 4, b4(tcnc, 0x0b00));
    }
    AUCTIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("auction_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} noise_toggle={} (off={}) amp={} center={} | battle[hpflee{} hpgate{} stop{} keep{} vis{}] | ld[stop{} near{} interv{} vis{} est{}] | tm_cancel_mask={} | ld1[around{} delay{} mask{} movepct{} threat{} randmin{}]\n\
             (-1=원본: amp900 center1000 / 21 41 15000 80000 120 / 15000 160000 50000 120 10 / 0xb00 / 80000 5 0x1a1 100 13 2) @base{:#x}\n",
            ok, tot, noise_ok, noff, namp, ctr, bhpf, bhpg, bstp, bkep, bvis,
            lstp, lnear, livn, lvis, lest, tcnc,
            larn, lard, lmsk, lmvp, lthr, lrnd, base));
    }
}

// ★★[08-03 신설] plan 결정기(`0xd452e0` = plan_legacy\handler.rs) 생성 게이트 byte-patch.
//   근거 = RE\2026-08-03_plan-vs-subplan-두enum-분리-발화조건-비트단위.md §D(imm 22종 표).
//   ★왜 여기인가: 이 함수는 **어떤 판단(plan)을 만들지 고르는 최상위 단계**이고, 모드가 대체하지 않는다
//     (모드 훅은 하류의 movepri `0xc559e0`) ⟹ **byte-patch가 유효**. 반면 `0xd803f0`·`0xc6e080`·`0xcb2340` 등
//     대체 대상 함수 내부의 상수는 패치해도 무효(vw_lane 사례 = §12.21(3) 규칙).
//   ⚠전 키 기본 -1 = 원본 복원(무개입). 값 지정 시 **게임이 원래 만들지 않던 조합의 plan이 생성될 수 있다** —
//     모드 재현이 처리하지 못하는 disc면 passthrough(원본 실행)로 떨어지므로 크래시는 아니나 AI 성향이 크게 바뀐다.
//   ⚠팀모드 JT 범위(`d45454 cmp ecx,7`)는 **의도적 제외** — 늘리면 점프테이블 OOB 위험.
/// ★[0.5.4 신설] 판단 14 = AttackNexus(넥서스 공격) — **0.5.4까지 노브가 하나도 없던 유일한 판단**.
///   Plan 디스패처 `0xe145b0` 안 202B 인라인 조각이라 함수 경계가 없어 후킹 대상이 못 됐다.
///   하지만 상수 5개가 전부 **exe 전역 유일 사이트**라 바이트패치는 안전하다(반쪽 노브 위험 0).
///
/// 원본 판정식(쉬운 말):
///   ① 내가 우리 분수 안이고 체력이 꽉 안 찼으면 → 귀환(SubPlan 7) 유지 = **풀피 될 때까지 집에서 대기**
///   ② 아니면 적 타워가 **하나도 안 남았을 때만** 진짜 넥서스 공격(SubPlan 18)
///   ③ 남아 있으면 라인방어(SubPlan 2)로 복귀
///   ⚠HP 조건은 `hp < max_hp` = **1이라도 깎였으면 대기**(퍼센트 문턱이 아니다).
///
/// ⚠로직은 0.5.3과 **완전 동일**하다(202B 중 다른 바이트 6개 = 오프셋 이동 + 점프거리).
unsafe fn apply_an_imm() {
    let wait   = tune("an_home_wait", -1);      // 분수 대기 시 SubPlan (원본 7=Recall)
    let towers = tune("an_tower_gate", -1);     // 넥서스 공격 허용 적 타워 수 (원본 0 = 전멸해야)
    let fb     = tune("an_fallback", -1);       // 타워가 남았을 때 폴백 SubPlan (원본 2=라인방어)
    let atk    = tune("an_attack_sub", -1);     // 넥서스 공격 SubPlan (원본 18)
    let wave   = tune("an_fallback_wave", -1);  // 폴백 시 웨이브 처리 성향 (원본 2, 추정=Push)
    let style  = tune("an_fallback_style", -1); // 폴백 스타일 바이트 (원본 0)

    let mut sig = 0u64;
    for v in [wait, towers, fb, atk, wave, style] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == ANIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }

    p!(base + 0xe14685, &[0x41,0xb9], 2, 4, b4(wait, 7));
    // ⚠`cmp qword[rax+r8+0x148], 0` — imm8 슬롯(부호확장)이라 0~127.
    //   값만 올리면 "정확히 N개일 때"가 되므로 아래 je→jbe 를 같이 봐야 한다(현재는 값만 노출).
    p!(base + 0xe146a1, &[0x4a,0x83,0xbc,0x00,0x48,0x01,0x00,0x00], 8, 1, b1(towers, 0));
    p!(base + 0xe146bf, &[0x41,0xb9], 2, 4, b4(fb, 2));
    p!(base + 0xe14929, &[0x41,0xb9], 2, 4, b4(atk, 18));
    p!(base + 0xe146bb, &[0xc6,0x46,0x0a], 3, 1, b1(wave, 2));
    p!(base + 0xe146b4, &[0xc6,0x46,0x08], 3, 1, b1(style, 0));

    ANIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("an_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} home_wait={} tower_gate={} fallback={} attack_sub={} wave={} style={}              (-1=원본: 7/0/2/18/2/0) @base{:#x}
", ok, tot, wait, towers, fb, atk, wave, style, base));
    }
}

/// ★[0.5.4 신설] 경매(전술 입찰) 중 **강제 귀환** 오버라이드 — 12노브.
///   0.5.4에서 새로 생긴 판단으로, 경매가 진행되는 동안 "지금 도망칠 피해를 견딜 수 없다"고
///   보이면 다른 모든 플랜을 제치고(점수 99999) 기지 코너로 물러나게 만든다.
///   ⚠전체가 `team_plan.version >= 2` 게이트(`auc_flee_version_gate`) 아래에 있다.
///     런타임 version 값은 **아직 미확정**(정적 호출 0건) — 게이트를 0으로 낮추면 항상 켤 수 있다.
///   전 키 기본 -1 = 원본값 그대로 = 무변화. 검증 = v54\aucknobs.py (12/12 exe 대조 통과).
unsafe fn apply_auc_imm() {
    let gate  = tune("auc_flee_version_gate", -1); // AI 사양 버전 게이트 (원본 1 → version>=2 에서 발동)
    let undy  = tune("auc_flee_undying_gate", -1); // 불사 특례 판정값 (원본 0) — 발동여부엔 무영향
    let hpf   = tune("auc_flee_hp_field", -1);     // 피해와 비교할 대상 (원본 0x658=현재HP, 0x610=최대HP)
    let nmask = tune("auc_flee_nexus_mask", -1);   // "넥서스 피격 중이면 취소" 비트마스크 (원본 0x100)
    let gfar  = tune("auc_flee_goal_far", -1);     // 도망 목적지 먼쪽 축 (원본 928000)
    let gna   = tune("auc_flee_goal_near_a", -1);  // 〃 가까운쪽 축 (원본 32000)
    let gnb   = tune("auc_flee_goal_near_b", -1);  // 〃 반대 사이드 사본 (원본 32000) — a와 같이 바꿀 것
    let dly   = tune("auc_flee_end_delay", -1);    // 도착 후 붙잡는 틱 (원본 5)
    let pf    = tune("auc_flee_pathfinder", -1);   // 경로탐색 사용 (원본 2=None) ⚠2 외 금지
    let wsk   = tune("auc_flee_with_skill", -1);   // 도망 중 허용 행동 비트 (원본 1=스킬만)
    let sc    = tune("auc_flee_score", -1);        // 이 플랜의 점수 (원본 99999 = 사실상 무조건 1위)
    let tag   = tune("auc_flee_action_tag", -1);   // 실제 행동 태그 (원본 3=RunAway) ⚠3 외 권장 안 함

    let mut sig = 0u64;
    for v in [gate, undy, hpf, nmask, gfar, gna, gnb, dly, pf, wsk, sc, tag] {
        sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64);
    }
    if sig == AUCIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }

    // ⚠`cmp rax, imm8`(부호확장) — 0~127 범위. 0 으로 낮추면 version 과 무관하게 항상 켜진다.
    p!(base + 0xead271, &[0x48,0x83,0xf8], 3, 1, b1(gate, 1));
    p!(base + 0xead285, &[0x41,0x80,0xb9,0x70,0x04,0x00,0x00], 7, 1, b1(undy, 0));
    // disp32 교체 — 값이 아니라 **비교할 필드**를 바꾼다(0x658 현재HP / 0x610 최대HP).
    p!(base + 0xead5e1, &[0x48,0x3b,0x81], 3, 4, b4(hpf, 0x658));
    p!(base + 0xead68d, &[0xa9], 1, 4, b4(nmask, 0x100));
    p!(base + 0xead6c8, &[0xb9], 1, 4, b4(gfar, 928_000));
    p!(base + 0xead6cd, &[0xba], 1, 4, b4(gna, 32_000));
    p!(base + 0xead6d6, &[0x41,0xb8], 2, 4, b4(gnb, 32_000));
    p!(base + 0xead6ff, &[0x49,0xc7,0x84,0x24,0x28,0x15,0x00,0x00], 8, 4, b4(dly, 5));
    p!(base + 0xead72f, &[0x41,0xc6,0x84,0x24,0x8d,0x15,0x00,0x00], 8, 1, b1(pf, 2));
    p!(base + 0xead738, &[0x41,0xc7,0x84,0x24,0x90,0x15,0x00,0x00], 8, 4, b4(wsk, 1));
    p!(base + 0xead759, &[0x49,0xc7,0x84,0x24,0x08,0x15,0x00,0x00], 8, 4, b4(sc, 99_999));
    p!(base + 0xead765, &[0x41,0xc6,0x84,0x24,0xc1,0x15,0x00,0x00], 8, 1, b1(tag, 3));

    AUCIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("auc_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} gate={} undying={} hp_field={} nexus_mask={} goal={}/{}/{} delay={} pf={} skill={} score={} tag={}\n\
             (-1=원본: 1/0/0x658/0x100/928000/32000/32000/5/2/1/99999/3) @base{:#x}\n",
            ok, tot, gate, undy, hpf, nmask, gfar, gna, gnb, dly, pf, wsk, sc, tag, base));
    }
}

// ★[0.5.4 신설] 경로/거리 시스템 노브 — 0.5.4 게임-ai 증가분(+464KB) 중 ~447KB가 이 신규 서브시스템이다.
//   ⚠⚠**표+루프로 유지할 것.** 208개 사이트를 p! 인라인으로 펼치면 opt-level=1 에서 호출부마다 스택 슬롯이
//     생겨 rayon 워커 스택이 터진다(2026-08-05 STATUS_STACK_OVERFLOW 실사고 = 크래시2).
//   전 사이트 exe 대조 완료(prefix/imm 실제위치/원본값/명령경계) = v54\wire_path.py, 208/208 통과.
static PATH_STEP640: [(usize, &[u8], usize); 76] = [
    (0xc4b941, &[0xc7,0x85,0x60,0x01,0x00,0x00], 6),
    (0xc4ba16, &[0xc7,0x85,0x60,0x01,0x00,0x00], 6),
    (0xc4baf0, &[0xc7,0x85,0x60,0x01,0x00,0x00], 6),
    (0xc4cc71, &[0xc7,0x85,0xa8,0x01,0x00,0x00], 6),
    (0xc4cd3d, &[0xc7,0x85,0xa8,0x01,0x00,0x00], 6),
    (0xc4ce08, &[0xc7,0x85,0xa8,0x01,0x00,0x00], 6),
    (0xc4dfc1, &[0xc7,0x85,0x60,0x01,0x00,0x00], 6),
    (0xc4e096, &[0xc7,0x85,0x60,0x01,0x00,0x00], 6),
    (0xc4e170, &[0xc7,0x85,0x60,0x01,0x00,0x00], 6),
    (0xc51630, &[0x41,0xb8], 2),
    (0xc5171b, &[0xc7,0x85,0xcc,0x01,0x00,0x00], 6),
    (0xc517c4, &[0xc7,0x85,0xcc,0x01,0x00,0x00], 6),
    (0xc517f1, &[0xc7,0x85,0xcc,0x01,0x00,0x00], 6),
    (0xc52c60, &[0x41,0xb8], 2),
    (0xc52d4b, &[0xc7,0x85,0xcc,0x01,0x00,0x00], 6),
    (0xc52df4, &[0xc7,0x85,0xcc,0x01,0x00,0x00], 6),
    (0xc52e21, &[0xc7,0x85,0xcc,0x01,0x00,0x00], 6),
    (0xc552e1, &[0xc7,0x85,0x80,0x01,0x00,0x00], 6),
    (0xc553b6, &[0xc7,0x85,0x80,0x01,0x00,0x00], 6),
    (0xc554b0, &[0xc7,0x85,0x80,0x01,0x00,0x00], 6),
    (0xc566a1, &[0xc7,0x85,0x80,0x01,0x00,0x00], 6),
    (0xc56776, &[0xc7,0x85,0x80,0x01,0x00,0x00], 6),
    (0xc56870, &[0xc7,0x85,0x80,0x01,0x00,0x00], 6),
    (0xc5ad91, &[0xc7,0x85,0x98,0x01,0x00,0x00], 6),
    (0xc5ae66, &[0xc7,0x85,0x98,0x01,0x00,0x00], 6),
    (0xc5af59, &[0xc7,0x85,0x98,0x01,0x00,0x00], 6),
    (0xc5d0a1, &[0xc7,0x85,0xa8,0x01,0x00,0x00], 6),
    (0xc5d166, &[0xc7,0x85,0xa8,0x01,0x00,0x00], 6),
    (0xc5d238, &[0xc7,0x85,0xa8,0x01,0x00,0x00], 6),
    (0xc5e34a, &[0xc7,0x85,0xa0,0x01,0x00,0x00], 6),
    (0xc5e426, &[0xc7,0x85,0xa0,0x01,0x00,0x00], 6),
    (0xc5e4ea, &[0xc7,0x85,0xa0,0x01,0x00,0x00], 6),
    (0xc4a889, &[0x41,0xb8], 2),
    (0xc4a970, &[0xc7,0x85,0x94,0x01,0x00,0x00], 6),
    (0xc4a996, &[0xc7,0x85,0x94,0x01,0x00,0x00], 6),
    (0xc4f384, &[0x41,0xbd], 2),
    (0xc4f450, &[0x41,0xbd], 2),
    (0xc4f46f, &[0x41,0xbd], 2),
    (0xc50464, &[0x41,0xbd], 2),
    (0xc50530, &[0x41,0xbd], 2),
    (0xc50602, &[0x41,0xbd], 2),
    (0xc542b1, &[0xc7,0x85,0x70,0x01,0x00,0x00], 6),
    (0xc54394, &[0xc7,0x85,0x70,0x01,0x00,0x00], 6),
    (0xc5445f, &[0xc7,0x85,0x70,0x01,0x00,0x00], 6),
    (0xc57a8a, &[0x41,0xbd], 2),
    (0xc57b6d, &[0x41,0xbd], 2),
    (0xc57c4f, &[0x41,0xbd], 2),
    (0xc58bda, &[0x41,0xbd], 2),
    (0xc58cbd, &[0x41,0xbd], 2),
    (0xc58d9f, &[0x41,0xbd], 2),
    (0xc59d11, &[0xc7,0x85,0xc8,0x01,0x00,0x00], 6),
    (0xc59dfb, &[0xc7,0x85,0xc8,0x01,0x00,0x00], 6),
    (0xc59ecf, &[0xc7,0x85,0xc8,0x01,0x00,0x00], 6),
    (0xc5c001, &[0xc7,0x85,0xc8,0x01,0x00,0x00], 6),
    (0xc5c0eb, &[0xc7,0x85,0xc8,0x01,0x00,0x00], 6),
    (0xc5c1bf, &[0xc7,0x85,0xc8,0x01,0x00,0x00], 6),
    (0xc5f6b1, &[0xc7,0x85,0xc8,0x01,0x00,0x00], 6),
    (0xc5f79b, &[0xc7,0x85,0xc8,0x01,0x00,0x00], 6),
    (0xc5f876, &[0xc7,0x85,0xc8,0x01,0x00,0x00], 6),
    (0xc607c1, &[0xc7,0x85,0xc8,0x01,0x00,0x00], 6),
    (0xc608ab, &[0xc7,0x85,0xc8,0x01,0x00,0x00], 6),
    (0xc60986, &[0xc7,0x85,0xc8,0x01,0x00,0x00], 6),
    (0xde5d4e, &[0x81,0xc2], 2),
    (0xde624a, &[0x41,0x81,0xc6], 3),
    (0xde6e10, &[0x41,0x81,0xc2], 3),
    (0xde736b, &[0x81,0xc3], 2),
    (0xde775a, &[0x41,0x81,0xc6], 3),
    (0xde7c3f, &[0x81,0xc7], 2),
    (0xde80aa, &[0x41,0x81,0xc6], 3),
    (0xde8582, &[0x81,0xc7], 2),
    (0xde8a7a, &[0x41,0x81,0xc6], 3),
    (0xde94fa, &[0x41,0x81,0xc6], 3),
    (0xde991a, &[0x41,0x81,0xc6], 3),
    (0xde9df2, &[0x81,0xc7], 2),
    (0xde6753, &[0x81,0xc1], 2),
    (0xde8f7a, &[0x81,0xc1], 2),
];

static PATH_STEP896: [(usize, &[u8], usize); 20] = [
    (0xc4a8b4, &[0x41,0xb8], 2),
    (0xc4f3ab, &[0x41,0xbd], 2),
    (0xc50492, &[0x41,0xbd], 2),
    (0xc542dc, &[0xc7,0x85,0x70,0x01,0x00,0x00], 6),
    (0xc57ab1, &[0x41,0xbd], 2),
    (0xc58c01, &[0x41,0xbd], 2),
    (0xc59d3c, &[0xc7,0x85,0xc8,0x01,0x00,0x00], 6),
    (0xc5c02c, &[0xc7,0x85,0xc8,0x01,0x00,0x00], 6),
    (0xc5f6dc, &[0xc7,0x85,0xc8,0x01,0x00,0x00], 6),
    (0xc607ec, &[0xc7,0x85,0xc8,0x01,0x00,0x00], 6),
    (0xc4b96d, &[0xc7,0x85,0x60,0x01,0x00,0x00], 6),
    (0xc4cc9c, &[0xc7,0x85,0xa8,0x01,0x00,0x00], 6),
    (0xc4dfed, &[0xc7,0x85,0x60,0x01,0x00,0x00], 6),
    (0xc51657, &[0x41,0xb8], 2),
    (0xc52c87, &[0x41,0xb8], 2),
    (0xc5530d, &[0xc7,0x85,0x80,0x01,0x00,0x00], 6),
    (0xc566cd, &[0xc7,0x85,0x80,0x01,0x00,0x00], 6),
    (0xc5adbd, &[0xc7,0x85,0x98,0x01,0x00,0x00], 6),
    (0xc5d0cc, &[0xc7,0x85,0xa8,0x01,0x00,0x00], 6),
    (0xc5e379, &[0xc7,0x85,0xa0,0x01,0x00,0x00], 6),
];

static PATH_RISK1281: [(usize, &[u8], usize); 50] = [
    (0xc4bc95, &[0xb8], 1),
    (0xc4bee7, &[0xb8], 1),
    (0xc4cf91, &[0xba], 1),
    (0xc4d007, &[0xba], 1),
    (0xc4d0f1, &[0xba], 1),
    (0xc4e315, &[0xb8], 1),
    (0xc4e567, &[0xb8], 1),
    (0xc51f0c, &[0xb8], 1),
    (0xc5353c, &[0xb8], 1),
    (0xc556b1, &[0xb8], 1),
    (0xc5591d, &[0xb8], 1),
    (0xc56a71, &[0xb8], 1),
    (0xc56cdd, &[0xb8], 1),
    (0xc5b0ea, &[0xb9], 1),
    (0xc5d40b, &[0xb9], 1),
    (0xc5d4a7, &[0xb9], 1),
    (0xc5e65f, &[0xb9], 1),
    (0xc5e743, &[0xb9], 1),
    (0xc5e77d, &[0xb9], 1),
    (0xc5e79f, &[0xb9], 1),
    (0xc5e7f1, &[0xb9], 1),
    (0xde5e45, &[0xba], 1),
    (0xde628e, &[0x41,0xb8], 2),
    (0xde6f47, &[0x41,0xb8], 2),
    (0xde6f82, &[0x41,0xb8], 2),
    (0xde739d, &[0xb8], 1),
    (0xde779e, &[0x41,0xb8], 2),
    (0xde7cdf, &[0x41,0xb8], 2),
    (0xde80e3, &[0xba], 1),
    (0xde863e, &[0x41,0xb8], 2),
    (0xde8abe, &[0x41,0xb8], 2),
    (0xde9533, &[0xba], 1),
    (0xde9953, &[0xba], 1),
    (0xde9e8e, &[0x41,0xb8], 2),
    (0xc4ab50, &[0xb8], 1),
    (0xc4f5ad, &[0xb8], 1),
    (0xc4f696, &[0xb8], 1),
    (0xc50750, &[0xb8], 1),
    (0xc50961, &[0xb8], 1),
    (0xc545bf, &[0xb8], 1),
    (0xc57d29, &[0xb8], 1),
    (0xc58e79, &[0xb8], 1),
    (0xc59f3d, &[0xb8], 1),
    (0xc5c22d, &[0xb8], 1),
    (0xc5f923, &[0xb8], 1),
    (0xc60a33, &[0xb8], 1),
    (0xde68ba, &[0x41,0xb9], 2),
    (0xde68fa, &[0x41,0xb9], 2),
    (0xde695c, &[0x41,0xb9], 2),
    (0xde90ab, &[0x41,0xb8], 2),
];

static PATH_HEUR: [(usize, &[u8], usize); 54] = [
    (0xc4a32b, &[0xc1,0xe1], 2),
    (0xc4add1, &[0xc1,0xe1], 2),
    (0xc4b48e, &[0xc1,0xe1], 2),
    (0xc4c127, &[0xc1,0xe1], 2),
    (0xc4c7ae, &[0xc1,0xe1], 2),
    (0xc4d307, &[0xc1,0xe1], 2),
    (0xc4db0e, &[0xc1,0xe1], 2),
    (0xc4e7a7, &[0xc1,0xe1], 2),
    (0xc4ee35, &[0xc1,0xe1], 2),
    (0xc4f887, &[0xc1,0xe1], 2),
    (0xc4ffbe, &[0xc1,0xe1], 2),
    (0xc50b37, &[0xc1,0xe1], 2),
    (0xc5117e, &[0xc1,0xe1], 2),
    (0xc520e7, &[0xc1,0xe1], 2),
    (0xc527ae, &[0xc1,0xe1], 2),
    (0xc53717, &[0xc1,0xe1], 2),
    (0xc53de5, &[0xc1,0xe1], 2),
    (0xc547e1, &[0xc1,0xe1], 2),
    (0xc54e3e, &[0xc1,0xe1], 2),
    (0xc55b77, &[0xc1,0xe1], 2),
    (0xc561fe, &[0xc1,0xe1], 2),
    (0xc56f37, &[0xc1,0xe1], 2),
    (0xc575c5, &[0xc1,0xe1], 2),
    (0xc58071, &[0xc1,0xe1], 2),
    (0xc58715, &[0xc1,0xe1], 2),
    (0xc591c1, &[0xc1,0xe1], 2),
    (0xc5985b, &[0xc1,0xe1], 2),
    (0xc5a281, &[0xc1,0xe1], 2),
    (0xc5a8ce, &[0xc1,0xe1], 2),
    (0xc5b417, &[0xc1,0xe1], 2),
    (0xc5bb4b, &[0xc1,0xe1], 2),
    (0xc5c571, &[0xc1,0xe1], 2),
    (0xc5cbbe, &[0xc1,0xe1], 2),
    (0xc5d6b7, &[0xc1,0xe1], 2),
    (0xc5de4b, &[0xc1,0xe1], 2),
    (0xc5ea55, &[0xc1,0xe1], 2),
    (0xc5f1f5, &[0xc1,0xe1], 2),
    (0xc5fc71, &[0xc1,0xe1], 2),
    (0xc60305, &[0xc1,0xe1], 2),
    (0xc60d81, &[0xc1,0xe1], 2),
    (0xde5ea7, &[0x41,0xc1,0xe6], 3),
    (0xde632b, &[0xc1,0xe2], 2),
    (0xde69e4, &[0xc1,0xe7], 2),
    (0xde6fd8, &[0xc1,0xe2], 2),
    (0xde73ee, &[0x41,0xc1,0xe1], 3),
    (0xde783b, &[0xc1,0xe2], 2),
    (0xde7d37, &[0xc1,0xe2], 2),
    (0xde8170, &[0x41,0xc1,0xe1], 3),
    (0xde869c, &[0xc1,0xe2], 2),
    (0xde8b5b, &[0xc1,0xe2], 2),
    (0xde9157, &[0xc1,0xe2], 2),
    (0xde95c0, &[0x41,0xc1,0xe1], 3),
    (0xde99e0, &[0x41,0xc1,0xe1], 3),
    (0xde9eec, &[0xc1,0xe2], 2),
];

/// ★[0.5.4 신설] 경로탐색 비용·위험 회피 노브. 전 키 기본 -1 = 원본값 = 무변화.
///   ⚠**A\* 허용성(admissibility)**: 휴리스틱이 `2^greedy × free_dist` 라, 간선비용을 휴리스틱보다
///     **낮추면** 최단경로 보장이 깨진다(크래시는 아니고 경로가 나빠질 뿐).
///     안전선 = 직교 >= 640, 대각 >= 896. **올리는 방향은 언제나 안전**하다.
unsafe fn apply_path_imm() {
    let orth  = tune("path_orth_cost", -1);     // 직교 1칸 비용 (원본 640) — 위험 페널티와의 상대 크기를 정한다
    let diag  = tune("path_diag_cost", -1);     // 대각 1칸 비용 (원본 896) — 올리면 계단식(맨해튼) 이동
    let dang  = tune("path_danger_cost", -1);   // 미니언 웨이브가 죽는 자리 회피 비용 (원본 1281 = 2칸우회×640+1)
    let greedy= tune("path_greedy", -1);        // A* 탐욕도 shl 자릿수 (원본 7 = ×128). 0=완전탐색(CPU↑)
    let tfloor= tune("path_threat_floor", -1);  // 위험지대 최소 우회 칸 (원본 2)
    let tcap  = tune("path_threat_cap", -1);    // 위험지대 최대 우회 칸 (원본 60)
    let tscale= tune("path_threat_scale", -1);  // 체력대비 피해 민감도 (원본 30)
    let tdef  = tune("path_threat_default", -1);// 위험원 못 찾았을 때 기본 우회 칸 (원본 2)
    let wave  = tune("path_wave_risk_ret", -1); // 위협원 안에 있을 때 위험등급 (원본 3)

    let mut sig = 0u64;
    for v in [orth, diag, dang, greedy, tfloor, tcap, tscale, tdef, wave] {
        sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64);
    }
    if sig == PATHIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let mut ok = 0u32; let mut tot = 0u32;

    // ── 다중 사이트 4종: 표 1개당 루프 1개 (인라인 전개 금지) ──
    let v = b4(orth, 640);
    for &(a, pre, off) in PATH_STEP640.iter()  { tot += 1; ok += patch_imm_bytes(base + a, pre, off, 4, v) as u32; }
    let v = b4(diag, 896);
    for &(a, pre, off) in PATH_STEP896.iter()  { tot += 1; ok += patch_imm_bytes(base + a, pre, off, 4, v) as u32; }
    let v = b4(dang, 1281);
    for &(a, pre, off) in PATH_RISK1281.iter() { tot += 1; ok += patch_imm_bytes(base + a, pre, off, 4, v) as u32; }
    // ⚠`shl r32, imm8` 의 자릿수 — 9를 넘기면 휴리스틱이 폭주해 경로가 직선으로 뭉개진다. 0~9 로 조인다.
    let v = if greedy < 0 { 7 } else { greedy.max(0).min(9) as u64 };
    for &(a, pre, off) in PATH_HEUR.iter()     { tot += 1; ok += patch_imm_bytes(base + a, pre, off, 1, v) as u32; }

    // ── 위협원 산식: clamp(floor + scale*(1칸에 받을피해)/HP, floor, cap) 칸 ──
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }
    p!(base + 0xdb07cb, &[0x83,0xc1], 2, 1, b1(tfloor, 2));
    // ⚠상한은 **2곳 동시**(cmp 는 imm8, 뒤이은 mov 는 imm32) — 한쪽만 바꾸면 반쪽 노브가 된다.
    p!(base + 0xdb07ce, &[0x83,0xf9], 2, 1, b1(tcap, 60));
    p!(base + 0xdb07d1, &[0xb8], 1, 4, b4(tcap, 60));
    p!(base + 0xdb077e, &[0xb9], 1, 4, b4(tscale, 30));
    p!(base + 0xdb07b2, &[0xb9], 1, 4, b4(tscale, 30));
    p!(base + 0xdb05fc, &[0xb8], 1, 4, b4(tdef, 2));
    p!(base + 0xdb0745, &[0xb8], 1, 4, b4(tdef, 2));
    p!(base + 0xd3101c, &[0xb8], 1, 4, b4(wave, 3));

    PATHIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("path_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} orth={} diag={} danger={} greedy={} | threat[floor={} cap={} scale={} default={}] wave={}\n\
             (-1=원본: 640/896/1281/7 | 2/60/30/2 | 3) @base{:#x}\n",
            ok, tot, orth, diag, dang, greedy, tfloor, tcap, tscale, tdef, wave, base));
    }
}

unsafe fn apply_plan_imm() {
    let role = tune("pl_obj_role", -1);       // 에픽/세르펜·정글 계열 plan을 가질 역할 슬롯(원본 1=정글러)
    let smask = tune("pl_serpen_phase_mask", -1); // 세르펜 plan 허용 페이즈 비트마스크(원본 0x1a1 = bit0,5,7,8)
    let ephase = tune("pl_epic_phase_min", -1);   // 에픽 plan 허용 페이즈 경계(원본 0xf9 — 내부 인코딩·의미 ⬜미확정)
    let gank = tune("pl_ganker_gate", -1);    // 갱(LineGanker) plan 게이트(원본 0x0b)
    let mut sig = 0u64;
    for v in [role, smask, ephase, gank] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == PLANIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let mut ok = 0u32;
    // ── 역할 게이트 2사이트 (plan12/14 생성 / 일반분기 plan7·8·10) ──
    ok += patch_imm_bytes(base + 0xe8e070, &[0x49,0x83,0xfd], 3, 1, b1(role, 1)) as u32;
    ok += patch_imm_bytes(base + 0xe8e433, &[0x41,0x83,0xfd], 3, 1, b1(role, 1)) as u32;
    // ── 세르펜 허용 페이즈 비트마스크 2사이트 ──
    ok += patch_imm_bytes(base + 0xe8e0ab, &[0xb9], 1, 4, b4(smask, 0x1a1)) as u32;
    ok += patch_imm_bytes(base + 0xe8e0e4, &[0xb9], 1, 4, b4(smask, 0x1a1)) as u32;
    // ★0.5.4: pl_serpen_phase_mask 가 **2곳 → 4곳**. `byte[+0x38]`(맵 레이아웃 9종) → `bt` 게이트.
    ok += patch_imm_bytes(base + 0xe8eae9, &[0xb9], 1, 4, b4(smask, 0x1a1)) as u32;
    ok += patch_imm_bytes(base + 0xe8f00c, &[0xb9], 1, 4, b4(smask, 0x1a1)) as u32;
    // ── 에픽 허용 페이즈 경계 2사이트 (⬜의미 미확정·실험용) ──
    ok += patch_imm_bytes(base + 0xe8e0f9, &[0x3c], 1, 1, if ephase < 0 { 0xf9 } else { (ephase & 0xff) as u64 }) as u32;
    ok += patch_imm_bytes(base + 0xe8e126, &[0x3c], 1, 1, if ephase < 0 { 0xf9 } else { (ephase & 0xff) as u64 }) as u32;
    // ★0.5.4: pl_epic_phase_min 이 **2곳 → 3곳**.
    ok += patch_imm_bytes(base + 0xe8f876, &[0x3c], 1, 1, if ephase < 0 { 0xf9 } else { (ephase & 0xff) as u64 }) as u32;
    // ── 갱 plan 게이트 1사이트 ──
    ok += patch_imm_bytes(base + 0xe8e86c, &[0x3c], 1, 1, b1(gank, 0x0b)) as u32;
    PLANIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(p) = pth("plan_imm.txt") {
        let _ = fs::write(p, format!("applied={}/10 obj_role={} serpen_mask={} epic_phase={} ganker_gate={} (-1=원본: 1/0x1a1/0xf9/0x0b) @base{:#x}\n",
            ok, role, smask, ephase, gank, base));
    }
}

// ★[07-16] GenericBuild(운영전환, FUN_141e1ebb0) 로밍/운영 상수 byte-patch. 경로A(전체재현 폐기 후 실속).
//   ghidra-re 8080/8081 병렬 정확핀(0.5.1). gb_enable=0(기본)이면 전량 원본복원(무개입).
//   값 규약: 각 튜닝키 기본 -1 = 그 사이트만 원본유지. gb_enable=1 & 값≥0일 때만 패치.
//   거리계열은 "유닛"을 입력받아 내부 인코딩(d²/d²+1)한다. imm 폭·부호 제약은 clamp로 방어.
//   ⚠ reach(cap/margin)는 GenericBuild 전용이 아닌 전역공유 헬퍼(FUN_141e30c00, 10콜사이트) → 전 AI 사거리에 영향.
//     기본 -1로 두면 무개입. 명시 입력 시에만 패치(편집기 경고).
//   ⚠ scout_radius(거점반경) 헬퍼(0x1e29xxx)의 GenericBuild 콜엣지는 미확정 → 로그 ok카운트로 sig매칭부터 확인.
// ✅★[0.5.2 재핀 완료 = 10사이트 (ghidra-re 2026-07-23, pdata 소속검증 + imm 지문 대조)] — gb_* 레버 부활
//   ~~구 12사이트(0.5.1 주소·applied=0/12)~~ → **10사이트**. 컨테이너 4종:
//     본체 GenericBuild = **`0x22b2280`**(0.5.1 0x1e1ebb0↔, close·line·join·push·margin) / 거점헬퍼 = **`0x2398240`**(op·scout×2)
//     / reach 공유헬퍼 = **`0x23ad980`**(cap#1) / reach#2 = **`0x23ba8d0`**(cap#2, 별도 함수).
//   ⛔**gb_join_phase 2사이트 = 0.5.2 삭제 = 死레버**: 본체 전 영역에서 `cmp r/m,0xc` 전 인코딩 변형 스캔 0건 —
//     join dist cmp 직후의 phase 서브게이트가 리팩터링으로 소멸(disc19 phase 게이트 삭제와 같은 0.5.2 패턴). 값 무반영.
//   ⚠prefix 변경 5곳(0.5.1→0.5.2): line(disp 0x180→0x1b0) / join(`49 01 c9 b9`→**`41 b8`**=mov r8d, off 4→2)
//     / scout1(disp 0x10→0x18) / scout2(`4c 8b 75 70`→`4c 8b ad 80 00 00 00`, off 6→9) / op(rbx→r14: `48 83 bb`→`49 83 be`).
//   ⚠scout(0x35a4e9001)은 objective dn_near_dist(0x1b930xx)와 **같은 값·다른 함수** — 혼동 금지(pdata로 소속 재확인됨).
unsafe fn apply_gb_imm() {
    let enable = tune("gb_enable", 0) != 0;
    // 튜닝키(전부 기본 -1 = 원본유지). 거리계열은 유닛, 게이트는 phase/퍼센트 raw.
    let cr = tune("gb_close_radius", -1);   // 근접반경(유닛, 원본≈387 / raw 150000)
    let lr = tune("gb_line_range",   -1);   // 라인range(유닛, 원본≈500 / raw 250000)
    let jd = tune("gb_join_dist",    -1);   // 합류/근접 전환거리(유닛, 원본 60000) — 지배 게이트
    let sr = tune("gb_scout_radius", -1);   // 거점반경(유닛, 원본 120000) — 로밍 후보수집 범위
    let op = tune("gb_op_phase",     -1);   // 운영진입 phase 임계(원본 31, =>30). 낮추면 이른 운영
    let jp = tune("gb_join_phase",   -1);   // 합류 phase 임계(원본 12). 낮추면 이른 합류
    let ph = tune("gb_push_hp",      -1);   // 라인압박 HP% 임계(원본 30). <이값이면 압박오더
    let rc = tune("gb_reach_cap",    -1);   // ⚠전역 reach 반경(유닛, 원본≈140052)
    let rm = tune("gb_reach_margin", -1);   // ⚠전역 reach 여유(유닛, 원본 25000)
    // 서명(enable+전 raw값) — 변화 없으면 재패치 skip
    let mut sig = enable as u64;
    for v in [cr, lr, jd, sr, op, jp, ph, rc, rm] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == GBIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    // 인코더/클램프
    let b1  = |v: i64| (v.max(0).min(0x7f)) as u64;                            // imm8 sign-safe(cmp qword 부호확장)
    let u32c= |v: i64| (v.max(0) as u64) & 0xffff_ffff;                        // imm32 raw
    let sqd = |d: i64| { let d = d.max(0) as u64; d.wrapping_mul(d) };         // d² (부호확장 imm32는 <2^31 필요)
    let sq1 = |d: i64| { let d = d.max(0) as u64; d.wrapping_mul(d).wrapping_add(1) };  // d²+1 (게임 인코딩)
    // 유효값: enable & 값≥0이면 인코딩, 아니면 게임 원본 raw 복원
    let on = |v: i64| enable && v >= 0;
    let e_cr = if on(cr) { sqd(cr) & 0x7fff_ffff } else { 150000 };            // imm32 부호확장 방어
    let e_lr = if on(lr) { sqd(lr) & 0x7fff_ffff } else { 250000 };
    let e_jd = if on(jd) { sq1(jd) & 0xffff_ffff } else { 0xd693a401 };        // MOV ECX zero-ext
    let _ = sq1;   // ★0.5.3: 거점반경²가 극성 반전으로 sqd 인코딩이 됨 → 구 e_sr(sq1) 폐기. sq1 은 다른 사이트가 쓸 수 있어 클로저는 유지.
    let e_op = if on(op) { b1(op) }                else { 0x1f };
    let _ = jp;   // ⛔gb_join_phase = 0.5.2 死레버(게이트 삭제) — 패치 사이트 없음(로그 표시용으로만 유지)
    let e_ph = if on(ph) { b1(ph) }                else { 0x1e };
    let e_rc = if on(rc) { sqd(rc) }               else { 0x490404400 };      // imm64 (전역 reach)
    let e_rm = if on(rm) { u32c(rm) }              else { 0x61a8 };
    let mut ok = 0u32;
    // ── 본체 0x22b2280: 거리/반경·HP (0.5.2 재핀 07-23) ──
    ok += patch_imm_bytes(base + 0xdca53f, &[0x48,0xc7,0x44,0x24,0x40], 5, 4, e_cr) as u32;
    // ★0.5.4: gb 근접반경² 가 **1곳 → 2곳**(같은 인자슬롯 복제).
    ok += patch_imm_bytes(base + 0xdca6b0, &[0x48,0xc7,0x44,0x24,0x40], 5, 4, e_cr) as u32;                  // 근접반경²  orig 0x249f0
    ok += patch_imm_bytes(base + 0xdcb1f0, &[0x48,0xc7,0x85,0x30,0x02,0x00,0x00], 7, 4, e_lr) as u32;        // 라인range² orig 0x3d090 (★0.5.3: rbp 변위 0x1b0→**0x270** = prefix 4~7B 교체. 0.5.1→0.5.2땐 0x180→0x1b0였음)
    ok += patch_imm_bytes(base + 0xdcb115, &[0xb8], 1, 4, e_jd) as u32;                                 // 합류max거리²(지배) orig 0xd693a401 (★0.5.3: `41 b8`(mov r8d)→**`b8`(mov eax)** 로 인코딩 축소 ⟹ 사이트가 +1(0xe075c9→**0xdcb115**), prefix 1B, off 2→1. 뒤 비교도 cmp r9,r8→cmp r8,rax 로 대응. 구 인코딩 mov r8d로 변경)
    ok += patch_imm_bytes(base + 0xdce2d5, &[0x48,0x83,0xf8], 3, 1, e_ph) as u32;                            // 라인압박 HP%<30
    // ── 거점헬퍼 0x2398240: op·scout ──
    ok += patch_imm_bytes(base + 0xdd512d, &[0x48,0x83,0xb9,0xb8,0x00,0x00,0x00], 7, 1, e_op) as u32;   // 운영진입 phase>30 (★0.5.3: 컨테이너 0x2398240→**0xcc3960**, `[r14+0xb8]`→**`[rcx+0xb8]`** = prefix 49 83 be→48 83 b9)
    // ★0.5.3: 0.5.2 는 같은 5슬롯 루프의 임계값을 **프리헤더+latch 2곳**에 호이스트했었는데(그래서 2사이트),
    //   0.5.3 은 호이스트 없이 **루프 본문 1곳**만 둔다 ⟹ 2사이트 → **1사이트 병합**. `movabs r9`→**`movabs rax`**.
    // ★★극성 반전: 0.5.2 `cmp rdx,r9; jae(스킵)`(임계=d²+1) ⟺ 0.5.3 `cmp rdx,rax; ja(스킵)`(임계=d²)
    //   ⟹ 인코딩을 **sq1(d²+1) → sqd(d²)** 로 바꿔야 한다. 여기서 sq1 을 쓰면 반경이 1 어긋난다.
    // ⚠같은 함수의 `movabs r13, 0x53d1ac101`(=150000²+1 @0xcc4399)은 **다른 반경**이고 원래도 미패치 — 값만 보고 잡지 말 것.
    let e_sr2 = if on(sr) { sqd(sr) } else { 0x35a4e9000 };   // ⚠sq1 아님(극성 반전)
    ok += patch_imm_bytes(base + 0xdd5656, &[0x48,0xb8], 2, 8, e_sr2) as u32;   // 거점반경² (0.5.2 #1+#2 통합)
    // ⛔합류 phase≥12 2사이트(구 0x1e1f4ea/0x1e1fa74) = 0.5.2 게이트 삭제 → 제거(상단 주석)
    // ── reach (전역공유 ⚠): 0x23ad980 / 0x23ba8d0 ──
    ok += patch_imm_bytes(base + 0xddc5d7, &[0x48,0xb8], 2, 8, e_rc) as u32;                                 // reach cap² #1(≤)
    ok += patch_imm_bytes(base + 0xde338d, &[0x49,0xba], 2, 8, e_rc.wrapping_add(1)) as u32;                 // reach cap² #2(<, +1경계)
    ok += patch_imm_bytes(base + 0xdcd2d7, &[0x41,0xb8], 2, 4, e_rm) as u32;                                 // reach margin
    GBIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(p) = pth("gb_imm.txt") {
        let _ = fs::write(p, format!("gb_enable={} applied={}/10 close={} line={} join={} scout={} op_ph={} join_ph={}=DEAD push_hp={} reach_cap={} reach_mgn={} @base{:#x}\n",
            enable, ok, cr, lr, jd, sr, op, jp, ph, rc, rm, base));
    }
}

// ★★[07-23 신설] 공유 위협 severity 사다리 byte-patch — **전 서브플랜 공통의 "이 위협이 유의미한가" 필터**.
//   ghidra-re 규명(0.5.2): disc19 인라인 사다리(d19_sev_* 소관)와 **같은 모양의 사본이 exe에 5곳 더** 있고, 각각:
//     [A] `0x22dd9a0` = **위협 평가 정본 본체**(TLS memo 래퍼 `0x22dd690` 경유·전 핸들러 ~60콜사이트가 공유) — 사다리 7 + **할인율 레버**
//         ('사소' 판정 위협은 `min(cap, threat>>shift)`로 축소 누산; 원본 shift=2(1/4)·cap=0x12=18).
//     [B] `0x22e6460` = 드라이버B(JT2 `0x1dabcc0` 계열 = 넥서스 공방 포함) 디스패치 직전 공통 위협 컨텍스트 빌더 — 축약 사다리 5.
//     [C] `0x22efed0` = 위협 유의성 필터 leaf(disc0/1/3·disc4 경로) — 사다리 7 (+branch A 별도 4임계 = ✅08-03 배선완, 아래 sv_pa_*).
//     [E] `0x23a04d0` = 공유 후보-스코어링 평가자(JT2 다수 래퍼 15종 경유) — 사다리 7(tr3만 jb 인코딩 = **+1**).
//     [D] `0x22f8a90`(disc5/6 후퇴판정 leaf) = ⬜**미배선**(disc5/6 라이브 발화 미확정 + 트레일러 매핑 신뢰도 중).
//   ★의미: `tr = threat*100/hp_cur` 사다리 — **tr 임계↓ = 더 겁쟁이(위협을 더 심각하게), ↑ = 더 대담**. hp 경계 = 단계 전환점.
//   ★설계: 4사본 33사이트(사다리 26 + [A]할인 3 + [C]branch A 4)를 **같은 값으로 일괄 패치**(사본별 개별화 금지 — 판단 일관성. disc19 사본만 d19_sev_*로 별도인 것은
//     기존 체계 유지). sv_enable=0(기본)=원본 복원. ⚠disc19 사다리와 값을 다르게 주면 넥서스방어만 다른 기준이 됨(의도적 허용).
unsafe fn apply_sev_imm() {
    let enable = tune("sv_enable", 0) != 0;
    let tr0 = tune("sv_tr0", 49);            // HP무관 기본 문턱 (원본 0x31)
    let tr1 = tune("sv_tr1", 29);            // HP<66 구간 (0x1d)
    let tr2 = tune("sv_tr2", 17);            // HP<41 구간 (0x11)
    let tr3 = tune("sv_tr3", 9);             // HP<26 구간 (0x09; E 사본은 +1 인코딩)
    let hp1 = tune("sv_hp1", 65);            // 1단계 HP% 경계 (0x41 = "hp%>65")
    let hp2 = tune("sv_hp2", 40);            // 2단계 (0x28)
    let hp3 = tune("sv_hp3", 25);            // 3단계 (0x19)
    let dsh = tune("sv_discount_shift", 2);  // [A] '사소' 위협 할인 shift (>>2 = 1/4). 0=할인없음(전액), 클수록 무시
    let dcp = tune("sv_discount_cap", 18);   // [A] 할인 후 상한 (0x12)
    // ★[08-03 신설] [C] branch A "소극 경로" 별도 4임계 배선 (구 ⬜미배선 후보 = MIGRATION §7.2-A14 §7 / 0.5.2 0x22f0067~79 → 0.5.3 0xcba183~e9, ghidra-re 08-03 확정·함수 내부 오프셋 보존·바이트 지문 검증).
    //   진입 = 콜사이트 arg7 flag==0 경로(6콜사이트 중 4곳 고정 + 동적 1곳). 원본 사다리(branch B보다 훨씬 엄격):
    //   hp%<=25 & tr>34 → 통과 / hp%>15 → 차단 / hp%<=15 & tr>=20 → 통과. tr = threat*100/hp_cur (branch B와 동일 축).
    //   ⚠sv_pa_tr_lo만 jb 인코딩 = 의미가 "tr >= 이값 통과"(초과 아님) — imm 그대로 노출(±1 보정 없음, 편집기 desc에 명시).
    let pa_hh = tune("sv_pa_hp_hi", 25);     // A1 hp% 상단 게이트: 이하일 때만 1단(tr_hi) 검사. ↑=1단 검사 hp 구간 확대(완화)
    let pa_th = tune("sv_pa_tr_hi", 34);     // A2 tr 문턱 1단: 초과=통과. ↓=소극 경로가 위협을 더 잘 인정(겁쟁이)
    let pa_hl = tune("sv_pa_hp_lo", 15);     // A3 hp% 하단 게이트: 초과=차단. ↑=차단 hp 구간 축소(완화)
    let pa_tl = tune("sv_pa_tr_lo", 20);     // A4 tr 문턱 2단(빈사 구간): 이상=통과(jb). ↓=빈사 시 작은 위협도 인정
    let mut sig = enable as u64;
    for v in [tr0, tr1, tr2, tr3, hp1, hp2, hp3, dsh, dcp, pa_hh, pa_th, pa_hl, pa_tl] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == SEVIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64| (v.max(0).min(0x7f)) as u64;
    let (p_t0, p_t1, p_t2, p_t3, p_h1, p_h2, p_h3, p_ds, p_dc) = if enable {
        (b1(tr0), b1(tr1), b1(tr2), b1(tr3), b1(hp1), b1(hp2), b1(hp3), b1(dsh.min(63)), b1(dcp))
    } else {
        (0x31, 0x1d, 0x11, 0x09, 0x41, 0x28, 0x19, 0x02, 0x12)
    };
    let (q_hh, q_th, q_hl, q_tl) = if enable {
        (b1(pa_hh), b1(pa_th), b1(pa_hl), b1(pa_tl))
    } else {
        (0x19, 0x22, 0x0f, 0x14)   // branch A 원본 복원값 = 25/34/15/20
    };
    let mut ok = 0u32;
    // ── [A] 위협 평가 정본 본체 0x22dd9a0 (사다리 7 + 할인 3) ──
    ok += patch_imm_bytes(base + 0xcaf964, &[0x48,0x83,0xf8], 3, 1, p_t0) as u32;   // tr>49
    ok += patch_imm_bytes(base + 0xcaf974, &[0x48,0x83,0xf9], 3, 1, p_h1) as u32;   // hp%>65 (rcx)
    ok += patch_imm_bytes(base + 0xcaf97a, &[0x48,0x83,0xf8], 3, 1, p_t1) as u32;   // tr>29
    ok += patch_imm_bytes(base + 0xcaf984, &[0x48,0x83,0xf9], 3, 1, p_h2) as u32;   // hp%>40
    ok += patch_imm_bytes(base + 0xcaf98a, &[0x48,0x83,0xf8], 3, 1, p_t2) as u32;   // tr>17
    ok += patch_imm_bytes(base + 0xcaf994, &[0x48,0x83,0xf9], 3, 1, p_h3) as u32;   // hp%>25
    ok += patch_imm_bytes(base + 0xcaf99a, &[0x48,0x83,0xf8], 3, 1, p_t3) as u32;   // tr>9
    ok += patch_imm_bytes(base + 0xcaf9af, &[0x48,0xc1,0xf8], 3, 1, p_ds) as u32;   // 할인 shift (sar rax,imm8)
    ok += patch_imm_bytes(base + 0xcaf9b3, &[0x48,0x83,0xf8], 3, 1, p_dc) as u32;   // 할인 cap 비교
    ok += patch_imm_bytes(base + 0xcaf9b7, &[0xbe], 1, 4, p_dc) as u32;             // 할인 cap 값 (mov ebx,imm32)
    // ── [B] 드라이버B 공통 위협 빌더 0x22e6460 (축약 5) ──
    ok += patch_imm_bytes(base + 0xcb7d5d, &[0x48,0x83,0xf8], 3, 1, p_t0) as u32;   // tr>49
    ok += patch_imm_bytes(base + 0xcb7d63, &[0x49,0x83,0xf8], 3, 1, p_h1) as u32;   // hp%>65 (r8)
    ok += patch_imm_bytes(base + 0xcb7d69, &[0x48,0x83,0xf8], 3, 1, p_t1) as u32;   // tr>29
    ok += patch_imm_bytes(base + 0xcb7d6f, &[0x49,0x83,0xf8], 3, 1, p_h2) as u32;   // hp%>40
    ok += patch_imm_bytes(base + 0xcb7d75, &[0x48,0x83,0xf8], 3, 1, p_t2) as u32;   // tr>17
    // ── [C] 위협 유의성 필터 leaf 0x22efed0 branch B (7) ──
    ok += patch_imm_bytes(base + 0xcba11b, &[0x48,0x83,0xf8], 3, 1, p_t0) as u32;   // tr>49
    ok += patch_imm_bytes(base + 0xcba121, &[0x48,0x83,0xf9], 3, 1, p_h1) as u32;   // hp%>65 (rcx)
    ok += patch_imm_bytes(base + 0xcba127, &[0x48,0x83,0xf8], 3, 1, p_t1) as u32;   // tr>29
    ok += patch_imm_bytes(base + 0xcba12d, &[0x48,0x83,0xf9], 3, 1, p_h2) as u32;   // hp%>40
    ok += patch_imm_bytes(base + 0xcba133, &[0x48,0x83,0xf8], 3, 1, p_t2) as u32;   // tr>17
    ok += patch_imm_bytes(base + 0xcba139, &[0x48,0x83,0xf9], 3, 1, p_h3) as u32;   // hp%>25
    ok += patch_imm_bytes(base + 0xcba13f, &[0x48,0x83,0xf8], 3, 1, p_t3) as u32;   // tr>9
    // ── [C] branch A "소극 경로" 4임계 (0.5.3 실바이트 지문: 48 83 f9 19 / 48 83 f8 22 / 48 83 f9 0f / 48 83 f8 14) ──
    ok += patch_imm_bytes(base + 0xcba183, &[0x48,0x83,0xf9], 3, 1, q_hh) as u32;   // A1 hp%>25 → 1단 스킵
    ok += patch_imm_bytes(base + 0xcba189, &[0x48,0x83,0xf8], 3, 1, q_th) as u32;   // A2 tr>34 → 통과
    ok += patch_imm_bytes(base + 0xcba18f, &[0x48,0x83,0xf9], 3, 1, q_hl) as u32;   // A3 hp%>15 → 차단
    ok += patch_imm_bytes(base + 0xcba195, &[0x48,0x83,0xf8], 3, 1, q_tl) as u32;   // A4 tr>=20 → 통과 (jb 인코딩)
    // ── [E] 공유 후보-스코어링 평가자 0x23a04d0 (7, tr3만 +1 인코딩) ──
    ok += patch_imm_bytes(base + 0xd958c3, &[0x48,0x83,0xf8], 3, 1, p_t0) as u32;   // tr>49
    ok += patch_imm_bytes(base + 0xd958c9, &[0x49,0x83,0xf8], 3, 1, p_h1) as u32;   // hp%>65 (r8)
    ok += patch_imm_bytes(base + 0xd958cf, &[0x48,0x83,0xf8], 3, 1, p_t1) as u32;   // tr>29
    ok += patch_imm_bytes(base + 0xd958d5, &[0x49,0x83,0xf8], 3, 1, p_h2) as u32;   // hp%>40
    ok += patch_imm_bytes(base + 0xd958db, &[0x48,0x83,0xf8], 3, 1, p_t2) as u32;   // tr>17
    ok += patch_imm_bytes(base + 0xd958e3, &[0x49,0x83,0xf8], 3, 1, p_h3) as u32;   // hp%>25
    ok += patch_imm_bytes(base + 0xd958e9, &[0x48,0x83,0xf8], 3, 1, p_t3 + 1) as u32; // tr>=10 (jb = tr3+1 인코딩)
    SEVIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(p) = pth("sev_imm.txt") {
        let _ = fs::write(p, format!("sv_enable={} applied={}/33 tr=[{} {} {} {}] hp=[{} {} {}] discount=shift{} cap{} pa=[hh{} th{} hl{} tl{}] @base{:#x}\n",
            enable, ok, tr0, tr1, tr2, tr3, hp1, hp2, hp3, dsh, dcp, pa_hh, pa_th, pa_hl, pa_tl, base));
    }
}

// ★[07-16] 백그라운드 경기 sim 병렬도 개선(경로: rayon split budget 게이트 무력화).
//   ghidra-re 8080/8081 확정: 일정넘김 배치 sim은 rayon bridge_producer_consumer(0x19ada40)가 매치묶음을
//   split budget(`splits==0`)로 조기 leaf종료 → 1 job이 여러 매치 순차처리(가동률 ~60%, 놀던 코어).
//   개입: split-budget-소진 게이트 `0x19adc93 je leaf(74 a0)` → nop(90 90). min=1 하한이 있어 len==1까지만
//   분할 = 매 leaf=1매치(과분할 아님·크래시 위험 없음·정적 thread-safe, 8081 판정). 결과 불변, 가동률만↑.
//   ⚠이 브릿지가 매치-sim 전용 모노모프인지 100% 미확증 → 공유 시 파급=그 iter 과분할(오버헤드뿐, 크래시 아님).
//   sim_unchunk=1로 켜고 일정넘김 시간을 직접 A/B 측정. 이상하면 0(원본복원).
static SIMUNCHUNK_APPLIED: AtomicU8 = AtomicU8::new(0xff);
const SIMUNCHUNK_RVA: usize = 0x19b40c3;   // *** 0.5.3 미해결 = 0.5.2값 유지(2026-07-29): 사이트 12B `74 a0 48 d1 eb 48 89 5d c0 48 89 f0` 가 0.5.3 exe 전역 0건(앞 8B로 줄여도 0건) = rayon 브리지 코드 자체가 바뀜. 코드가 원본바이트(74 a0) 재검증 후에만 패치하므로 ABORT = fail-safe(1매치/job 분할 노브만 죽음). // ★0.5.2(was 0.5.1 0x19adc93). version-migrator 확정: 컨테이너(rayon bridge)가 L3-UNIQUE 매칭 + 사이트 12B 바이트 완전동일(`74 a0 48 d1 eb 48 89 5d c0 48 89 f0`). 코드가 원본바이트(74 a0) 재검증 후에만 패치=어긋나면 ABORT(안전).
unsafe fn apply_sim_unchunk() {
    let want = tune("sim_unchunk", 0) != 0;
    let w = want as u8;
    if SIMUNCHUNK_APPLIED.load(Ordering::Relaxed) == w { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let addr = base + SIMUNCHUNK_RVA;
    if !readable(addr, 2) { return; }
    let (b0, b1) = (rd_u8(addr), rd_u8(addr + 1));
    let is_orig = b0 == 0x74 && b1 == 0xa0;   // je leaf(rel8=0xa0 → 0x19adc35)
    let is_nop  = b0 == 0x90 && b1 == 0x90;
    if !is_orig && !is_nop {                    // RVA 어긋남/패치판 → 중단(크래시 방지)
        if let Some(p) = pth("sim_unchunk.txt") { let _ = fs::write(p, format!("ABORT bytes={:02x}{:02x} (RVA 불일치?) @0x{:x}\n", b0, b1, addr)); }
        SIMUNCHUNK_APPLIED.store(w, Ordering::Relaxed);   // 재시도 안 함(스팸 방지)
        return;
    }
    let target: [u8; 2] = if want { [0x90, 0x90] } else { [0x74, 0xa0] };
    let mut old: u32 = 0;
    if VirtualProtect(addr, 2, 0x40, &mut old) == 0 { return; }
    core::ptr::write_unaligned(addr as *mut u8, target[0]);
    core::ptr::write_unaligned((addr + 1) as *mut u8, target[1]);
    VirtualProtect(addr, 2, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, 2);
    SIMUNCHUNK_APPLIED.store(w, Ordering::Relaxed);
    if let Some(p) = pth("sim_unchunk.txt") {
        let _ = fs::write(p, format!("sim_unchunk={} APPLIED @0x{:x} bytes={:02x}{:02x} (1매치/job 분할 {})\n",
            want, addr, target[0], target[1], if want { "ON" } else { "OFF=원본" }));
    }
}

// ★install_wrap: 순수 트램폴린(orig N바이트 + jmp fn+N) 생성 후, fn 프롤로그를 jmp cap_fn으로 패치.
//   반환=트램폴린 주소(cap_fn이 game 원본으로 호출). install_detour와 달리 cap_fn이 원본 실행 후 sret 캡처 가능.
unsafe fn install_wrap(rva: usize, orig_len: usize, cap_fn: usize) -> Result<usize, &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, orig_len+4) { return Err("fn unreadable"); }
    // 신원검증: push rbp/r15/r14/r13/r12/rsi/rdi/rbx (12B) — disc18/19 공통 프롤로그
    let prol = [0x55u8,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];
    for i in 0..12 { if *((fn_addr+i) as *const u8) != prol[i] { return Err("프롤로그 불일치(패치판?)"); } }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = stub_reg(VirtualAlloc(0, 128, MEM_CR, RWX), 128, rva);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;   // fn+12 (원본 sub rsp,...)
    let mut s: Vec<u8> = Vec::new();
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);                                              // orig 12B(push8)
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes()); s.extend_from_slice(&[0xff,0xe0]); // movabs rax,fn+12; jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    // 패치: fn 프롤로그 12B → movabs rax,cap_fn; jmp rax
    let mut patch = vec![0x90u8; orig_len];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&cap_fn.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(stub)
}


unsafe fn install_detour(rva: usize, orig_len: usize, cap_fn: usize) -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, orig_len+4) { return Err("fn unreadable"); }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = stub_reg(VirtualAlloc(0, 256, MEM_CR, RWX), 256, rva);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);        // mov r10, rsp
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]); // push rcx rdx r8 r9 r10 r11
    s.extend_from_slice(&[0x48,0x89,0xe1]);        // mov rcx, rsp (saved)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);        // mov rdx, r10 (entry_rsp)
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);   // sub rsp,0x28
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff,0xd0]);             // call rax
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);   // add rsp,0x28
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]); // pop r11 r10 r9 r8 rdx rcx
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes());
    s.extend_from_slice(&[0xff,0xe0]);             // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(())
}

unsafe fn install_replace_detour_rax(rva: usize, orig_len: usize, cap_fn: usize) -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, orig_len+4) { return Err("fn unreadable"); }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = stub_reg(VirtualAlloc(0, 256, MEM_CR, RWX), 256, rva);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);        // mov r10, rsp
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]); // push rcx rdx r8 r9 r10 r11
    s.extend_from_slice(&[0x48,0x89,0xe1]);        // mov rcx, rsp (saved)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);        // mov rdx, r10 (entry_rsp)
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);   // sub rsp,0x28
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes()); // movabs rax, cap_fn
    s.extend_from_slice(&[0xff,0xd0]);             // call rax  (→ rax: RAX_SENT=passthrough / else=반환값)
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);   // add rsp,0x28
    s.extend_from_slice(&[0x49,0xbb]); s.extend_from_slice(&(RAX_SENT as u64).to_le_bytes()); // movabs r11, sentinel
    s.extend_from_slice(&[0x4c,0x39,0xd8]);        // cmp rax, r11
    s.extend_from_slice(&[0x74,0x0b]);             // je +0x0b → fallthrough (HANDLED 11B 스킵)
    // ── HANDLED (11B): pop regs(rax=반환값 보존) → ret (caller로 복귀) ──
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]); // pop r11 r10 r9 r8 rdx rcx (10B)
    s.extend_from_slice(&[0xc3]);                  // ret (1B)
    // ── FALLTHROUGH: regs복원 → 원본 prologue → fn+len ──
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]); // pop r11 r10 r9 r8 rdx rcx
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes()); // movabs rax, fn+len
    s.extend_from_slice(&[0xff,0xe0]);             // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(())
}


// ── 영역 D 캡처 전용 디투어(mid-func 0x20e42a3): rbp/r12/r13 캡처 + ★강제 16정렬보정(and rsp,-16) ──
//   mid-func는 rsp 16-정렬(함수진입 16k-8과 다름) → push개수만으론 call서 8B 어긋남 → cap_fn movaps 폴트(genbuild_body_D.md "유일 crash지점").
//   해결: 전 reg save 후 rbx에 rsp백업 → and rsp,-16(어느쪽 정렬이든 robust) → call → mov rsp,rbx 복원(rbx=non-vol, cap_fn 보존).
//   saved 레이아웃: +0=rcx +8=rdx +0x10=r8 +0x18=r9 +0x20=r10(entry_rsp) +0x28=r11 +0x30=rbx +0x38=rbp +0x40=r12 +0x48=r13.
//   ★r14는 save안함(cap_fn=Win64가 non-vol 보존) → 원본 shr r14,2 정상. rbp/rbx도 pop으로 복원 후 원본 mov rcx,[rbp+0x108] 정상.
// ── ★영역 D 진짜 skip 디투어(mid-func 0x42a3): cap_fn(saved,entry_rsp)->i64. SENT=passthrough(원본 region D 실행, fn+orig_len 복귀) / else=out ptr=HANDLED.
//    HANDLED: rax=cap반환(out=sret 반환값) 유지하고 funnel(funnel_rva)로 jmp = 게임 region D 미실행. region D=RNG-free라 skip 무desync.
//    saved 레이아웃 = install_detour_d 동일(+0x38=rbp/+0x40=r12/+0x48=r13). 정렬보정 and rsp,-16. rax: passthrough=복원 / handled=cap반환(out) 유지.
unsafe fn install_detour_d_skip(rva: usize, orig_len: usize, cap_fn: usize, funnel_rva: usize) -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, orig_len+4) { return Err("fn unreadable"); }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = stub_reg(VirtualAlloc(0, 256, MEM_CR, RWX), 256, rva);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;          // passthrough 복귀 (0x42b2)
    let funnel_addr = mbase + funnel_rva;        // handled jump (0x20e4a1a)
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);   // mov r10, rsp (entry_rsp)
    // push rax r13 r12 rbp rbx r11 r10 r9 r8 rdx rcx  (rax=highest/마지막pop ; rcx=saved+0)
    s.extend_from_slice(&[0x50, 0x41,0x55, 0x41,0x54, 0x55, 0x53, 0x41,0x53, 0x41,0x52, 0x41,0x51, 0x41,0x50, 0x52, 0x51]);
    s.extend_from_slice(&[0x48,0x89,0xe1]);   // mov rcx, rsp (saved=arg1)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);   // mov rdx, r10 (entry_rsp=arg2)
    s.extend_from_slice(&[0x48,0x89,0xe3]);   // mov rbx, rsp (정렬복원 홀더)
    s.extend_from_slice(&[0x48,0x83,0xe4,0xf0]); // and rsp,-16
    s.extend_from_slice(&[0x48,0x83,0xec,0x20]); // sub rsp,0x20
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff,0xd0]);        // call rax  (rax=SENT / out)
    s.extend_from_slice(&[0x48,0x89,0xdc]);   // mov rsp, rbx
    s.extend_from_slice(&[0x49,0xbb]); s.extend_from_slice(&(RAX_SENT as u64).to_le_bytes()); // movabs r11, SENT
    s.extend_from_slice(&[0x4c,0x39,0xd8]);   // cmp rax, r11
    s.extend_from_slice(&[0x74, 0x22]);       // je +0x22 → PASSTHROUGH (HANDLED 블록 34B 스킵)
    // ── HANDLED (34B): rax=out(cap반환) 유지. pop rcx..r13(10, rax제외) → add rsp,8(rax슬롯 폐기) → jmp funnel ──
    s.extend_from_slice(&[0x59, 0x5a, 0x41,0x58, 0x41,0x59, 0x41,0x5a, 0x41,0x5b, 0x5b, 0x5d, 0x41,0x5c, 0x41,0x5d]); // pop rcx rdx r8 r9 r10 r11 rbx rbp r12 r13 (16B)
    s.extend_from_slice(&[0x48,0x83,0xc4,0x08]); // add rsp,8 (saved rax슬롯 폐기, rax=out 유지) (4B)
    s.extend_from_slice(&[0xff,0x25,0x00,0x00,0x00,0x00]); s.extend_from_slice(&funnel_addr.to_le_bytes()); // jmp [rip+0]; .quad funnel (14B)
    // ── PASSTHROUGH: pop rcx..r13 rax(11, rax복원) → 원본 15B → jmp fn+orig_len ──
    s.extend_from_slice(&[0x59, 0x5a, 0x41,0x58, 0x41,0x59, 0x41,0x5a, 0x41,0x5b, 0x5b, 0x5d, 0x41,0x5c, 0x41,0x5d, 0x58]); // pop ...r13 rax (17B)
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0xff,0x25,0x00,0x00,0x00,0x00]); s.extend_from_slice(&ret_addr.to_le_bytes()); // jmp [rip+0]; .quad fn+orig_len
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(())
}

// ── 조건부 디투어: cap_fn 반환값 rax==0(handled)→*p1 이미씀, rax=p1로 caller에 즉시 RET(원본 스킵).
//    rax==1(fall-through)→원본 prologue 실행 후 fn+12로(원본 정상실행). 출력 sret=rcx=param_1. ──
unsafe fn install_replace_detour(rva: usize, orig_len: usize, cap_fn: usize) -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, orig_len+4) { return Err("fn unreadable"); }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = stub_reg(VirtualAlloc(0, 256, MEM_CR, RWX), 256, rva);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);        // mov r10, rsp  (r10=ESP0=retaddr슬롯)
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]); // push rcx rdx r8 r9 r10 r11
    s.extend_from_slice(&[0x48,0x89,0xe1]);        // mov rcx, rsp (saved)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);        // mov rdx, r10 (entry_rsp)
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);   // sub rsp,0x28
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes()); // movabs rax, cap_fn
    s.extend_from_slice(&[0xff,0xd0]);             // call rax  (→ rax: 0=handled / 1=fallthrough)
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);   // add rsp,0x28
    s.extend_from_slice(&[0x48,0x85,0xc0]);        // test rax, rax
    s.extend_from_slice(&[0x75,0x0e]);             // jnz +0x0e → fallthrough (handled블록 14B 스킵)
    // ── HANDLED (14B): regs복원 → rax=rcx(=p1) → ret (caller로 복귀, *p1 이미 씀) ──
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]); // pop r11 r10 r9 r8 rdx rcx (rsp=ESP0)
    s.extend_from_slice(&[0x48,0x89,0xc8]);        // mov rax, rcx  (반환값 = param_1 sret)
    s.extend_from_slice(&[0xc3]);                  // ret  ([ESP0]=caller retaddr pop, 복귀)
    // ── FALLTHROUGH: regs복원 → 원본 prologue → fn+12 ──
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]); // pop r11 r10 r9 r8 rdx rcx (rsp=ESP0)
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes()); // movabs rax, fn+12
    s.extend_from_slice(&[0xff,0xe0]);             // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(())
}

// ★07-11 강화(재현검증 후): itemnet 가드 판정. GOOD(원본실행)=true / BAD(차단·score 0.0)=false.
//   ⚠구 범위체크(저주소만 차단)로는 owner "부분초기화"(model=owner+0x1558이 큰 주소지만 unmapped) 케이스를 못 막아
//     0x1b784a1 `MOV RAX,[RBX+0x10]`서 AV 재현됨(d4_repl=1 리그 실측 07-11). → readable()로 실제 커밋 페이지 확인.
//   순수 검사(VirtualQuery만·패닉0·락0) → rayon 워커 포함 전 스레드 안전(§6). readable=TOCTOU이나 여기선 즉시 deref로 창 최소.
unsafe extern "C" fn itemnet_guard_ok(model: usize) -> bool {
    if model < 0x10000 || model >= (1usize << 48) { return false; }   // 저주소/비정상(owner NULL→~0x1558)
    if !readable(model, 0x20) { return false; }                        // 크래시 [model+0x10]/[+0x18] deref 대상 커밋 확인
    let w = std::ptr::read_unaligned((model + 8) as *const usize);     // weights ptr(model readable 확인 후 안전)
    if w != 0 && !readable(w, 8) { return false; }                     // 희소 내적 대상도 2차 방어
    true
}

unsafe fn install_itemnet_guard() -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + RVA_ITEMNET_SCORER;
    if !readable(fn_addr, 27) { return Err("fn unreadable"); }
    // 신원검증①: 프롤로그 push8(rbp,r15,r14,r13,r12,rsi,rdi,rbx) — scrim expect와 동일
    let prol = [0x55u8,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];
    for i in 0..12 { if *((fn_addr+i) as *const u8) != prol[i] { return Err("프롤로그 불일치(게임 업데이트?)"); } }
    // 신원검증②: fn+12 변위대상 15B = sub rsp,0xd8 / lea rbp,[rsp+0x80] (byte-exact, rip-rel無 확인됨)
    let disp = [0x48u8,0x81,0xec,0xd8,0x00,0x00,0x00, 0x48,0x8d,0xac,0x24,0x80,0x00,0x00,0x00];
    let site = fn_addr + 12;
    for i in 0..15 { if *((site+i) as *const u8) != disp[i] { return Err("fn+12 바이트 불일치(게임 업데이트?)"); } }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = stub_reg(VirtualAlloc(0, 128, MEM_CR, RWX), 128, RVA_ITEMNET_SCORER);
    if stub == 0 { return Err("VirtualAlloc"); }
    let hits = core::ptr::addr_of!(ITEMNET_GUARD_HITS) as usize;
    let ret_addr = site + 15;   // GOOD 복귀 = fn+27
    let check = itemnet_guard_ok as *const () as usize;
    let mut s: Vec<u8> = Vec::new();
    // ── 프리앰블: model(rcx) 보존 + Rust 가드판정(readable 검사) 호출. rbp프레임+and로 16정렬(call 안전). ──
    s.extend_from_slice(&[0x55]);                        // push rbp
    s.extend_from_slice(&[0x48,0x89,0xe5]);              // mov rbp, rsp
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]); // push rcx,rdx,r8,r9,r10,r11 (volatile 보존)
    s.extend_from_slice(&[0x48,0x83,0xec,0x20]);         // sub rsp,0x20 (shadow space)
    s.extend_from_slice(&[0x48,0x83,0xe4,0xf0]);         // and rsp,-16 (16정렬)
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&check.to_le_bytes()); // movabs rax, itemnet_guard_ok
    s.extend_from_slice(&[0xff,0xd0]);                   // call rax  (rcx=model=arg1 진입값 그대로) → al=GOOD?
    s.extend_from_slice(&[0x48,0x8d,0x65,0xd0]);         // lea rsp,[rbp-0x30] (6 push 위치로 복원)
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59,0x5d]); // pop r11,r10,r9,r8,rdx,rcx,rbp
    s.extend_from_slice(&[0x84,0xc0]);                   // test al,al
    s.extend_from_slice(&[0x74,0x1d]);                   // jz BAD (+0x1d=29 → GOOD블록 건너뜀)
    // ── GOOD: 변위 15B 실행 → fn+27 복귀 (rsp=fn+12 진입상태로 복원됨) ──
    s.extend_from_slice(&disp);
    s.extend_from_slice(&[0xff,0x25,0x00,0x00,0x00,0x00]); s.extend_from_slice(&ret_addr.to_le_bytes()); // jmp [rip+0]; .quad fn+27
    // ── BAD(82): 카운트 → push8 되감기 → xmm0/xmm1=0 → ret ──
    debug_assert!(s.len() == 82);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&hits.to_le_bytes()); // movabs rax, &HITS
    s.extend_from_slice(&[0xf0,0x48,0xff,0x00]);                              // lock inc qword [rax]
    s.extend_from_slice(&[0x5b,0x5f,0x5e,0x41,0x5c,0x41,0x5d,0x41,0x5e,0x41,0x5f,0x5d]); // pop rbx,rdi,rsi,r12,r13,r14,r15,rbp (프롤로그 역순)
    s.extend_from_slice(&[0x0f,0x57,0xc0]);                                   // xorps xmm0,xmm0 (score=0.0)
    s.extend_from_slice(&[0x0f,0x57,0xc9]);                                   // xorps xmm1,xmm1 (미분 레인 보험)
    s.push(0xc3);                                                             // ret
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    // 패치(fn+12, 15B): movabs rax,stub(10B) + jmp rax(2B) + nop*3
    let mut patch = vec![0x90u8; 15];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(site, 15, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), site as *mut u8, 15);
    VirtualProtect(site, 15, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), site, 15);
    Ok(())
}


// ── facet#2 이동 override 핸들러: driver memcpy 직전 Input(src=rdx) 가로채기 ──
// tag@+0==1(Move)이면 x@+8/y@+0x10를 cfg값으로 덮어씀. tag별 카운트(훅 발동확인).
unsafe extern "C" fn move_override(src: usize) {
    if !ptr_ok(src) || !readable(src, 0x90) { return; }
    let tag = std::ptr::read_unaligned(src as *const i64);
    let b = if (0..16).contains(&tag) { tag as usize } else { 15 };
    TAG_COUNTS[b].fetch_add(1, Ordering::Relaxed);
    // tag별 첫 샘플: struct 머리 9 qword 덤프 (좌표 위치 찾기)
    if TAG_SAMP[b][0].load(Ordering::Relaxed) == i64::MIN {
        for k in 0..18usize { TAG_SAMP[b][k].store(std::ptr::read_unaligned((src + k*8) as *const i64), Ordering::Relaxed); }
    }
    if MOVE_ON.load(Ordering::Relaxed) && tag == MOVE_TAG.load(Ordering::Relaxed) {
        let off = MOVE_OFF.load(Ordering::Relaxed) as usize;
        if off + 16 <= 0x90 {
            std::ptr::write_unaligned((src + off) as *mut i64, MOVE_X.load(Ordering::Relaxed));
            std::ptr::write_unaligned((src + off + 8) as *mut i64, MOVE_Y.load(Ordering::Relaxed));
            MOVE_HANDLED.fetch_add(1, Ordering::Relaxed);
        }
    }
}

// rel32 도달범위(±2GB) 내 target 근처에 RWX 할당 (CALL rel32 재지정용)
unsafe fn alloc_near(target: usize, size: usize) -> usize {
    let base = target & !0xffff;
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let mut step = 1usize;
    while step < 0x7000 {  // ~1.75GB 까지 64KB 스텝
        for dir in [1isize, -1isize] {
            let addr = base.wrapping_add((dir * (step as isize) * 0x10000) as usize);
            if addr >= 0x10000 {
                let p = stub_reg(VirtualAlloc(addr, size, MEM_CR, RWX), size, 0xF005);
                if p != 0 { return p; }
            }
        }
        step += 1;
    }
    0
}


// ── facet#2 진짜 이동훅: driver의 FUN_141917430(이동좌표 최종화) 호출지점(0x1d4fecf)을 POST-래퍼로 ──
// 래퍼 = 원본 FUN_141917430을 (인자 그대로 복제) 호출 → 직후 outptr(=rcx, [RBP+0xd0])이 최종 Move{tag,x,y}
//        → move_override(outptr) (9 qword 덤프 + cfg move=1이면 x/y 강제). 호출지점 한정이라 다른 caller엔 무영향.
unsafe fn install_move_post_hook() -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let call_site = mbase + RVA_F2_BUILD_CALL;   // CALL FUN_141917430
    if !readable(call_site, 5) { return Err("call_site unreadable"); }
    if std::ptr::read_unaligned(call_site as *const u8) != 0xE8 { return Err("not a CALL(E8)"); }
    let next = call_site + 5;                      // 0x141d4fed4 (driver 복귀지점)
    let rel0 = std::ptr::read_unaligned((call_site + 1) as *const i32);
    let target = (next as i64 + rel0 as i64) as usize;  // 실제 FUN_141917430 주소
    if target != mbase + RVA_GENERIC_BUILD { return Err("target mismatch (not generic_build)"); }
    let stub = alloc_near(next, 160);
    if stub == 0 { return Err("alloc_near"); }
    let new_rel = stub as i64 - next as i64;
    if new_rel > 0x7f00_0000 || new_rel < -0x7f00_0000 { return Err("stub too far"); }
    // 래퍼 스텁. 진입 rsp=S(%16==8), [S]=복귀주소, rcx=outptr, rdx/r8/r9=arg2~4, stack arg5~8=[S+0x28..0x40].
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x55]);                          // push rbp
    s.extend_from_slice(&[0x48, 0x89, 0xE5]);             // mov rbp, rsp        (rbp=S-8)
    s.extend_from_slice(&[0x53]);                          // push rbx
    s.extend_from_slice(&[0x48, 0x89, 0xCB]);             // mov rbx, rcx        (rbx=outptr, call 넘어 보존)
    s.extend_from_slice(&[0x48, 0x83, 0xEC, 0x48]);       // sub rsp, 0x48       (shadow0x20+arg0x20+align8)
    // stack arg5~8 복제: [rbp+0x30..0x48] → [rsp+0x20..0x38] (원래 호출이 넘기던 값 그대로)
    s.extend_from_slice(&[0x48, 0x8B, 0x45, 0x30, 0x48, 0x89, 0x44, 0x24, 0x20]); // mov rax,[rbp+0x30]; mov [rsp+0x20],rax
    s.extend_from_slice(&[0x48, 0x8B, 0x45, 0x38, 0x48, 0x89, 0x44, 0x24, 0x28]); // arg6
    s.extend_from_slice(&[0x48, 0x8B, 0x45, 0x40, 0x48, 0x89, 0x44, 0x24, 0x30]); // arg7
    s.extend_from_slice(&[0x48, 0x8B, 0x45, 0x48, 0x48, 0x89, 0x44, 0x24, 0x38]); // arg8
    // rcx/rdx/r8/r9 그대로(arg1~4). 원본 호출.
    s.extend_from_slice(&[0x48, 0xB8]); s.extend_from_slice(&target.to_le_bytes());  // movabs rax, FUN_141917430
    s.extend_from_slice(&[0xFF, 0xD0]);                   // call rax
    // 복귀: rax=리턴값(sret→outptr). move_override(outptr) 실행 (rax 보존).
    s.extend_from_slice(&[0x50]);                          // push rax           (리턴값 보존)
    s.extend_from_slice(&[0x48, 0x89, 0xD9]);             // mov rcx, rbx        (arg=outptr)
    s.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]);       // sub rsp, 0x28       (shadow+align)
    s.extend_from_slice(&[0x48, 0xB8]); s.extend_from_slice(&(move_override as usize).to_le_bytes()); // movabs rax,move_override
    s.extend_from_slice(&[0xFF, 0xD0]);                   // call rax
    s.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28]);       // add rsp, 0x28
    s.extend_from_slice(&[0x58]);                          // pop rax            (리턴값 복원)
    s.extend_from_slice(&[0x48, 0x8D, 0x65, 0xF8]);       // lea rsp, [rbp-8]    (저장된 rbx 위치)
    s.extend_from_slice(&[0x5B]);                          // pop rbx
    s.extend_from_slice(&[0x5D]);                          // pop rbp
    s.extend_from_slice(&[0xC3]);                          // ret                (→ 복귀주소 next)
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(call_site, 5, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    std::ptr::write_unaligned((call_site + 1) as *mut i32, new_rel as i32);
    VirtualProtect(call_site, 5, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), call_site, 5);
    Ok(())
}

// stub이 p2(rcx) 넘겨 호출. 반환 = 적용할 p2(배율). 게임 원본 FUN_1420a8680은 stub이 호출.
unsafe extern "C" fn threatgate_adjust(p2: usize) -> usize {
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mult = TG_MULT.load(Ordering::Relaxed);
        if TG_CAP.load(Ordering::Relaxed) {
            let n = TG_N.fetch_add(1, Ordering::Relaxed);
            if n < 300 { append_named("tgcap.txt", &format!("[tg #{}] p2={} (0x{:x}) gate_skip(<12)={} mult={}\n", n, p2 as i64, p2, (p2 as i64) < 0xc, mult)); }
        }
        if mult == 100 { p2 } else {
            let v = (p2 as i64).wrapping_mul(mult) / 100;
            if v < 0 { 0 } else { v as usize }
        }
    }));
    r.unwrap_or(p2)   // 패닉시 원본 p2 passthrough(게임 계속)
}

unsafe fn install_threatgate_hook() -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let call_site = mbase + RVA_TG_CALL;            // 0x1feca43 CALL FUN_1420a8680 (E8 rel32)
    if !readable(call_site, 5) { return Err("call_site unreadable"); }
    if std::ptr::read_unaligned(call_site as *const u8) != 0xE8 { return Err("not a CALL(E8)"); }
    let next = call_site + 5;
    let rel0 = std::ptr::read_unaligned((call_site + 1) as *const i32);
    let target = (next as i64 + rel0 as i64) as usize;
    if target != mbase + RVA_THREATGATE_FN { return Err("target mismatch (not threatgate)"); }
    let stub = alloc_near(next, 160);
    if stub == 0 { return Err("alloc_near"); }
    let new_rel = stub as i64 - next as i64;
    if new_rel > 0x7f00_0000 || new_rel < -0x7f00_0000 { return Err("stub too far"); }
    // 진입 rsp=S(%16==8), [S]=복귀(0x1feca48). rcx/rdx/r8/r9=인자(스택인자 없음). 반환 rax(결과)+rdx(타깃) 둘 다 보존.
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x55]);                          // push rbp
    s.extend_from_slice(&[0x48, 0x89, 0xE5]);              // mov rbp, rsp
    s.extend_from_slice(&[0x48, 0x83, 0xEC, 0x40]);        // sub rsp, 0x40 (인자저장 + shadow)
    s.extend_from_slice(&[0x48, 0x89, 0x4D, 0xF8]);        // mov [rbp-0x08], rcx
    s.extend_from_slice(&[0x48, 0x89, 0x55, 0xF0]);        // mov [rbp-0x10], rdx
    s.extend_from_slice(&[0x4C, 0x89, 0x45, 0xE8]);        // mov [rbp-0x18], r8
    s.extend_from_slice(&[0x4C, 0x89, 0x4D, 0xE0]);        // mov [rbp-0x20], r9
    s.extend_from_slice(&[0x48, 0x89, 0xD1]);              // mov rcx, rdx (=p2)
    s.extend_from_slice(&[0x48, 0xB8]); s.extend_from_slice(&(threatgate_adjust as usize).to_le_bytes()); // movabs rax, threatgate_adjust
    s.extend_from_slice(&[0xFF, 0xD0]);                    // call rax → rax=조정된 p2
    s.extend_from_slice(&[0x48, 0x89, 0xC2]);              // mov rdx, rax (p2 갱신)
    s.extend_from_slice(&[0x48, 0x8B, 0x4D, 0xF8]);        // mov rcx, [rbp-0x08]
    s.extend_from_slice(&[0x4C, 0x8B, 0x45, 0xE8]);        // mov r8, [rbp-0x18]
    s.extend_from_slice(&[0x4C, 0x8B, 0x4D, 0xE0]);        // mov r9, [rbp-0x20]
    s.extend_from_slice(&[0x48, 0xB8]); s.extend_from_slice(&target.to_le_bytes());  // movabs rax, FUN_1420a8680
    s.extend_from_slice(&[0xFF, 0xD0]);                    // call rax → rax/rdx 반환
    s.extend_from_slice(&[0x48, 0x89, 0xEC]);              // mov rsp, rbp
    s.extend_from_slice(&[0x5D]);                          // pop rbp
    s.extend_from_slice(&[0xC3]);                          // ret (→ next 0x1feca48, rax/rdx 유지)
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());
    const RWX: u32 = 0x40; let mut old: u32 = 0;
    if VirtualProtect(call_site, 5, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    std::ptr::write_unaligned((call_site + 1) as *mut i32, new_rel as i32);
    VirtualProtect(call_site, 5, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), call_site, 5);
    Ok(())
}


// 광범위 커밋 dump: 매프레임 최종 Input(rdx) tag별 첫샘플 18 qword. (override 없음, 관측전용)
unsafe extern "C" fn commit_dump(src: usize) {
    if !ptr_ok(src) || !readable(src, 0x90) { return; }
    COMMIT_TOTAL.fetch_add(1, Ordering::Relaxed);
    let tag = std::ptr::read_unaligned(src as *const i64);
    let b = if (0..16).contains(&tag) { tag as usize } else { 15 };
    COMMIT_TAGCOUNT[b].fetch_add(1, Ordering::Relaxed);
    if COMMIT_SAMP[b][0].load(Ordering::Relaxed) == i64::MIN {
        for k in 0..18usize { COMMIT_SAMP[b][k].store(std::ptr::read_unaligned((src + k*8) as *const i64), Ordering::Relaxed); }
    }
}

// 페이즈 게이트 threshold 베이스(imm8) 패치. cfg engage_base>=0이면 적용(핫리로드). -1이면 원본 복원.
unsafe fn apply_engage_base() {
    let mbase = exe_base();
    if mbase == 0 { return; }
    let site = mbase + RVA_ENGAGE_GATE;
    if !readable(site, 3) { return; }
    // sanity: 83 C0 ?? (ADD EAX, imm8)
    if std::ptr::read_unaligned(site as *const u8) != 0x83 || std::ptr::read_unaligned((site+1) as *const u8) != 0xC0 { return; }
    let imm_site = site + 2;
    // 최초 1회 원본 백업
    if ENGAGE_ORIG.load(Ordering::Relaxed) < 0 {
        ENGAGE_ORIG.store(std::ptr::read_unaligned(imm_site as *const u8) as i64, Ordering::Relaxed);
    }
    let want = ENGAGE_BASE.load(Ordering::Relaxed);
    let new_imm: u8 = if want < 0 { ENGAGE_ORIG.load(Ordering::Relaxed) as u8 }  // -1=원본 복원
                      else { want.clamp(0, 127) as u8 };
    if std::ptr::read_unaligned(imm_site as *const u8) == new_imm { return; }  // 변화없으면 skip
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(imm_site, 1, RWX, &mut old) == 0 { return; }
    std::ptr::write_unaligned(imm_site as *mut u8, new_imm);
    VirtualProtect(imm_site, 1, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), imm_site, 1);
}

// ── 광범위 커밋 훅: CALL FUN_141a49fa0(0x1d5035d) rel32 재지정 → commit_dump(rdx) 후 jmp 원본 ──
unsafe fn install_commit_hook() -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let call_site = mbase + RVA_COMMIT_CALL;
    if !readable(call_site, 5) { return Err("call_site unreadable"); }
    if std::ptr::read_unaligned(call_site as *const u8) != 0xE8 { return Err("not a CALL(E8)"); }
    let next = call_site + 5;
    let rel0 = std::ptr::read_unaligned((call_site + 1) as *const i32);
    let target = (next as i64 + rel0 as i64) as usize;
    if target != mbase + RVA_COMMIT_FN { return Err("target mismatch (not commit fn)"); }
    let stub = alloc_near(next, 128);
    if stub == 0 { return Err("alloc_near"); }
    let new_rel = stub as i64 - next as i64;
    if new_rel > 0x7f00_0000 || new_rel < -0x7f00_0000 { return Err("stub too far"); }
    // 스텁: rcx=champ+0x590, rdx=&Input. commit_dump(rdx) 후 jmp FUN_141a49fa0(→ret시 0x141d50362 복귀).
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x51, 0x52, 0x41, 0x50, 0x41, 0x51]);  // push rcx; push rdx; push r8; push r9
    s.extend_from_slice(&[0x48, 0x89, 0xD1]);                    // mov rcx, rdx (arg=Input)
    s.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]);              // sub rsp,0x28 (shadow+align)
    s.extend_from_slice(&[0x48, 0xB8]); s.extend_from_slice(&(commit_dump as *const () as usize).to_le_bytes());
    s.extend_from_slice(&[0xFF, 0xD0]);                          // call rax
    s.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28]);              // add rsp,0x28
    s.extend_from_slice(&[0x41, 0x59, 0x41, 0x58, 0x5A, 0x59]);  // pop r9; pop r8; pop rdx; pop rcx
    s.extend_from_slice(&[0x48, 0xB8]); s.extend_from_slice(&target.to_le_bytes());  // movabs rax, FUN_141a49fa0
    s.extend_from_slice(&[0xFF, 0xE0]);                          // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(call_site, 5, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    std::ptr::write_unaligned((call_site + 1) as *mut i32, new_rel as i32);
    VirtualProtect(call_site, 5, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), call_site, 5);
    Ok(())
}


// ★facet#5 역할 교전임계값 스케일 패치(cfg engage_thr_mult). mult=100→원본 복원. 각 immediate low byte 1개.
unsafe fn apply_engage_thr_mult() {
    let mult = ENGAGE_THR_MULT.load(Ordering::Relaxed);
    if mult < 0 { return; }
    let mbase = exe_base();
    if mbase == 0 { return; }
    // 오프셋 sanity: 각 site의 imm32 상위3바이트가 0이어야(작은값). 아니면 잘못된 오프셋→중단.
    for &(rva, _) in &ROLE_THR {
        let s = mbase + rva;
        if !readable(s, 4) { return; }
        if std::ptr::read_unaligned((s+1) as *const u8) != 0
            || std::ptr::read_unaligned((s+2) as *const u8) != 0
            || std::ptr::read_unaligned((s+3) as *const u8) != 0 { return; }
    }
    const RWX: u32 = 0x40;
    for &(rva, orig) in &ROLE_THR {
        let s = mbase + rva;
        let new = ((orig as i64) * mult / 100).clamp(0, 255) as u8;
        if std::ptr::read_unaligned(s as *const u8) == new { continue; }
        let mut old: u32 = 0;
        if VirtualProtect(s, 1, RWX, &mut old) == 0 { continue; }
        std::ptr::write_unaligned(s as *mut u8, new);
        VirtualProtect(s, 1, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), s, 1);
    }
}

// ★numbers 전력회피를 movepri 각 disc(subplan) 분기에 공통 적용(2026-07-02). disc가 곧 subplan → NUMBERS_THREAT_SP[disc] 우선(-1=공통 폴백).
//   r14=athlete(p5)/r15=view(p6)/p1=출력ptr(대체 후 code)은 모든 disc 분기 공통(entry_rsp+0x28/+0x30, saved+0x28). 후퇴 판정시 code7 override.
//   게이트: numbers류 아무것도 설정 안 하면 skip(기본=무동작=원본보존). catch_unwind+fault-safe read라 detour 안전.
// ★★[08-03 원본 순수화 — 유저 지시] 이 함수 전체 = **게임 원본에 없는 모드 신규 판단층**
//   (전력 회피 numbers_* · 포탑 회피 tower_*/ally_tower_* · 성향 보정 stat_* · subplan별 개별 임계 numbers_threat_spN).
//   방침: "원본 판단함수에 없는 판단식은 일단 전부 제거하고, 원본 레버를 모두 노출한 뒤 다시 추가한다."
//   ⟹ **함수 본문은 보존하고 진입부에서만 차단**(재추가 시 이 한 줄만 되돌리면 복구 = 저비용).
//   재추가 시점에는 §분기조건_정본에 정리된 실행층(auction·Order 태그)과 겹치지 않게 설계할 것.
const NUMBERS_LAYER_ENABLED: bool = false;   // ★false = 모드 신규 후퇴층 전면 비활성(원본 동작)
unsafe fn apply_numbers_sp(disc: i64, entry_rsp: usize, p1: usize) {
    if !NUMBERS_LAYER_ENABLED { return; }   // ★[08-03] 원본 순수화
    if !(NUMBERS_THREAT_SP_ANY.load(Ordering::Relaxed) || TOWER_THREAT.load(Ordering::Relaxed) > 0 || NUMBERS_THREAT.load(Ordering::Relaxed) > 0 || NUMBERS_THREAT_MOVE.load(Ordering::Relaxed) > 0 || NUMBERS_MARGIN.load(Ordering::Relaxed) > 0) { return; }
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if !ptr_ok(p1) { return; }
        let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;   // athlete(p5)
        let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;   // view(p6)
        if !ptr_ok(r14) || !ptr_ok(r15) { return; }
        let base_code = rd_u8(p1);
        let l80 = rd_u64(r15).unwrap_or(0) as usize;
        let sim = rd_u64(l80).unwrap_or(0) as usize;
        // ★★[07-23 stale 수정] ~~`r14+0x6a8`(side)·`r14+0x6a0`(self handle)~~ = **0.4.x 잔재** → **`+0x820`·`+0x818`**(0.5.x).
        //   모드 전역이 이미 `p5+0x820`/`p5+0x818`을 쓰는데(dd7700·serpen·f22e80 등) **이 함수만 구 오프셋**이었다.
        //   ⚠**라이브 경로**: cfg `tower_threat=200`·`numbers_threat=49`·`numbers_threat_move=2`가 켜져 있어 실제 실행 중이었고,
        //   side가 쓰레기값·selfe가 잘못된 핸들로 해석돼 **numbers 후퇴 판정이 전 disc에서 오작동**하고 있었다(감사 07-23).
        let side = rd_i64(r14 + 0x810).unwrap_or(-1);//  ★0.5.4 오프셋 이동 반영
        let selfe = dd7_slot128(sim, rd_u64(r14 + 0x818).unwrap_or(0));
        // ★[07-29 detlog v2] numbers: IN(site12)=판단입력 / OUT(site28)=후퇴 발동여부. 둘 다 **항상** 기록해야
        //   "같은 입력인데 발동만 갈림"(=laner_should_retreat 비결정)을 잡을 수 있다.
        let fired = ptr_ok(selfe) && laner_should_retreat(r15, side, selfe, r14, base_code, disc);
        // ★[07-29 detlog ch4] numbers 후퇴 판정(발동여부). ch0/ch1 일치인데 여기만 갈리면 laner_should_retreat이 비결정.
        if DL_ON.load(Ordering::Relaxed) {
            dl_rec(sim, 4, (fired as u64) ^ ((disc as u64) << 1) ^ rd_u64(r14 + 0x810).unwrap_or(0).rotate_left(7));
        }
        if fired {
            std::ptr::write_unaligned(p1 as *mut u64, 7u64);
            // ★[수정 07-31] `.min(15)` → `.min(17)`: 구 클램프는 disc15/16/17을 슬롯15 하나에 합산해
            //   "세르펜 사냥/견제에서 실제로 후퇴가 걸렸는지"를 볼 수 없게 만들고 있었다(numbers_threat_sp16/17 死 오판정의 배경).
            // ★★[수정 07-31] **덤프를 여기서 제거** — 구 코드는 20발동마다 `write_named`(동기 fs::write)를 **detour 안에서** 했다.
            //   disc3만 실측 88,393회 발동 ⟹ `log=1` 이면 rayon 워커들이 수천 번 동기 파일IO = ⛔DONE.md 등재 크래시 원인
            //   ("detour 내 동기 파일IO 금지 — rayon 폭주→크래시"). 여기선 **원자 카운터만** 올리고,
            //   실제 파일 쓰기는 메인스레드 `post_update`의 `sp_seen_flush()`가 프레임 스로틀로 담당한다.
            SP_SEEN[(disc as usize).min(17)].fetch_add(1, Ordering::Relaxed);   // ★진단: 어느 subplan서 numbers 후퇴 발동
        }
    }));
}

// ════════════════════════════════════════════════════════════════════════════════
// ★★★게임 결함 수정 ③: 「아군에게 붙기」 판단이 **영원히 실행되지 않는다** (`lt_revive_join`)
// ════════════════════════════════════════════════════════════════════════════════
// RE = `RE\2026-08-04_line_total-전수해독-purpose정체-죽은분기-0.5.3.md`
//
// 라인 총력전(`line_total`)에는 "가장 가까운 아군이 나보다 적에게 더 가까우면 그 아군에게 붙는다"는
// 판단이 있는데, **한 번도 실행되지 않는다.**
//   원인: "가장 가까운 아군"을 고르는 루프(`0xc578c6`~`0xc57d11`, 5슬롯 완전 언롤)가
//        **자기 자신을 후보에서 빼지 않는다** ⟹ 언제나 자기(거리 0)가 뽑힌다.
//        그러면 게이트가 `d²(아군,적) < d²(아군,나) = 0` 이 되어 **unsigned 비교라 영구 false**이고,
//        뒤따르는 `d²(아군,나) >= 50000²` 게이트도 0이라 함께 막힌다(`0xc57db1`·`0xc57dcf`).
//
// [고치는 법] 바이트패치로는 불가능하다 — 루프에 "자기 제외" 비교를 **끼워 넣을 자리가 없다**.
//   대신 argmin이 끝나고 게이트가 시작되기 직전(`0xc57d18`)에 트램폴린을 걸어,
//   **결과가 자기 자신이면 자기를 뺀 최근접 아군으로 다시 고른다.**
//   그 지점에서 `rax` = 고른 아군 · `rbp+0xb8` = 나 · `rbp+0x1d8` = 아군 5슬롯 배열이 모두 살아 있다.
//   ⚠재진입 경로(`0xc57f8d → 0xc57d18`)에서도 `rax`는 같은 의미(아군 argmin 결과)다 — 확인함.
//
// ⚠**기본 OFF.** 켜면 개발사가 한 번도 테스트하지 않은 판단이 살아나므로 밸런스 영향이 미지수다.
// ⚠살아나면 후보가 하나 늘어 경매 경쟁이 바뀐다 ⟹ 리플레이 재현이 깨진다.
// ════════════════════════════════════════════════════════════════════════════════
const LT_JOIN_RVA: usize = 0xc57d18;
const LT_JOIN_LEN: usize = 14;
const LT_JOIN_ORIG: [u8; 14] = [0x31,0xff, 0xba,0x08,0x00,0x00,0x00, 0x48,0x89,0x95,0xe0,0x01,0x00,0x00];
static LT_JOIN_APPLIED: AtomicI64 = AtomicI64::new(-1);
static LT_JOIN_STUB: AtomicUsize = AtomicUsize::new(0);

/// 「자기를 뺀 최근접 아군」재선택 스텁. 변위는 **전부 길이에서 계산**한다(손으로 세지 않는다).
unsafe fn lt_build_join_stub(ret_addr: usize) -> usize {
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = stub_reg(VirtualAlloc(0, 256, MEM_CR, RWX), 256, 0xF40);
    if stub == 0 { return 0; }

    // ── 루프 1회분(후보 하나 평가). 앞의 두 조기탈출은 이 블록의 "남은 길이"로 점프한다 ──
    let mut score: Vec<u8> = Vec::new();
    score.extend_from_slice(&[0x48,0x8b,0x81,0x48,0x06,0x00,0x00]);   // mov rax,[rcx+0x648]
    score.extend_from_slice(&[0x49,0x2b,0x82,0x48,0x06,0x00,0x00]);   // sub rax,[r10+0x648]
    score.extend_from_slice(&[0x48,0x0f,0xaf,0xc0]);                  // imul rax,rax
    score.extend_from_slice(&[0x48,0x8b,0x99,0x50,0x06,0x00,0x00]);   // mov rbx,[rcx+0x650]
    score.extend_from_slice(&[0x49,0x2b,0x9a,0x50,0x06,0x00,0x00]);   // sub rbx,[r10+0x650]
    score.extend_from_slice(&[0x48,0x0f,0xaf,0xdb]);                  // imul rbx,rbx
    score.extend_from_slice(&[0x48,0x01,0xd8]);                       // add rax,rbx
    score.extend_from_slice(&[0x49,0x39,0xc1]);                       // cmp r9,rax   (best vs cand)
    score.extend_from_slice(&[0x76,0x06]);                            // jbe +6 → 갱신 건너뜀
    score.extend_from_slice(&[0x49,0x89,0xc1]);                       // mov r9,rax
    score.extend_from_slice(&[0x49,0x89,0xc8]);                       // mov r8,rcx

    let mut body: Vec<u8> = Vec::new();
    body.extend_from_slice(&[0x49,0x8b,0x0c,0x13]);                   // mov rcx,[r11+rdx]
    body.extend_from_slice(&[0x48,0x85,0xc9]);                        // test rcx,rcx
    // ⚠변위는 "뒤에 올 바이트 수" — `cmp`(3) + **`je` 자신(2)** + score. je 2바이트를 빠뜨리면
    //   명령 중간에 착지한다(2026-08-04에 실제로 그랬고 역어셈 검증에서 잡았다).
    body.extend_from_slice(&[0x74, (3 + 2 + score.len()) as u8]);     // jz → NEXT
    body.extend_from_slice(&[0x4c,0x39,0xd1]);                        // cmp rcx,r10  (자기?)
    body.extend_from_slice(&[0x74, score.len() as u8]);               // je → NEXT
    body.extend_from_slice(&score);
    // NEXT:
    let back = body.len() + 4 + 4 + 2;                                // 루프 선두까지 되돌아갈 거리
    body.extend_from_slice(&[0x48,0x83,0xc2,0x08]);                   // add rdx,8
    body.extend_from_slice(&[0x48,0x83,0xfa,0x28]);                   // cmp rdx,0x28 (5슬롯)
    body.push(0x72); body.push((256 - back) as u8);                   // jb → LOOP (음수 rel8)

    // 결과 반영: 찾았으면 스택에 저장된 원래 rax 자리를 덮어쓴다
    let mut commit: Vec<u8> = Vec::new();
    commit.extend_from_slice(&[0x4d,0x85,0xc0]);                      // test r8,r8
    commit.extend_from_slice(&[0x74,0x05]);                           // jz → DONE
    commit.extend_from_slice(&[0x4c,0x89,0x44,0x24,0x40]);            // mov [rsp+0x40],r8

    // 누산기 초기화
    let mut init: Vec<u8> = Vec::new();
    init.extend_from_slice(&[0x4d,0x31,0xc0]);                        // xor r8,r8    (best=null)
    init.extend_from_slice(&[0x49,0xc7,0xc1,0xff,0xff,0xff,0xff]);    // mov r9,-1    (best=∞)
    init.extend_from_slice(&[0x31,0xd2]);                             // xor edx,edx

    // 아군 배열 로드 블록 (조기탈출 변위 계산용으로 따로 만든다)
    let after_arr = init.len() + body.len() + commit.len();           // "배열 null" 탈출이 건너뛸 길이
    let mut arr: Vec<u8> = Vec::new();
    arr.extend_from_slice(&[0x4c,0x8b,0x9d,0xd8,0x01,0x00,0x00]);     // mov r11,[rbp+0x1d8] (아군 배열)
    arr.extend_from_slice(&[0x4d,0x85,0xdb]);                         // test r11,r11
    arr.extend_from_slice(&[0x74, after_arr as u8]);                  // jz → DONE
    let tail = arr.len() + after_arr;                                 // "결과≠나" 탈출이 건너뛸 길이

    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x50,0x53,0x51,0x52]);                      // push rax,rbx,rcx,rdx
    s.extend_from_slice(&[0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]);  // push r8,r9,r10,r11
    s.push(0x9c);                                                     // pushfq   → [rsp+0x40]=원래 rax
    s.extend_from_slice(&[0x4c,0x8b,0x95,0xb8,0x00,0x00,0x00]);       // mov r10,[rbp+0xb8]  (나)
    s.extend_from_slice(&[0x4d,0x85,0xd2]);                           // test r10,r10
    s.extend_from_slice(&[0x74, (5 + 2 + tail) as u8]);               // jz → DONE (cmp+jne 까지 포함해 건너뜀)
    s.extend_from_slice(&[0x4c,0x39,0x54,0x24,0x40]);                 // cmp [rsp+0x40],r10  (결과==나?)
    s.extend_from_slice(&[0x75, tail as u8]);                         // jne → DONE (정상이면 손대지 않음)
    s.extend_from_slice(&arr);
    s.extend_from_slice(&init);
    s.extend_from_slice(&body);
    s.extend_from_slice(&commit);
    // DONE:
    s.push(0x9d);                                                     // popfq
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58]);  // pop r11,r10,r9,r8
    s.extend_from_slice(&[0x5a,0x59,0x5b,0x58]);                      // pop rdx,rcx,rbx,rax
    s.extend_from_slice(&LT_JOIN_ORIG);                               // 훔친 원본 재실행
    s.extend_from_slice(&[0xff,0x25,0x00,0x00,0x00,0x00]);            // jmp qword [rip+0]
    s.extend_from_slice(&ret_addr.to_le_bytes());
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());
    stub
}

/// `lt_revive_join` 토글. 켜면 트램폴린 설치, 끄면 원본 복원.
unsafe fn apply_lt_revive_join() {
    let want = if tune("lt_revive_join", 0) != 0 { 1i64 } else { 0i64 };
    if LT_JOIN_APPLIED.load(Ordering::Relaxed) == want { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let addr = base + LT_JOIN_RVA;

    if want == 0 {
        if LT_JOIN_APPLIED.load(Ordering::Relaxed) == 1 { fs2_write(addr, &LT_JOIN_ORIG); }
        LT_JOIN_APPLIED.store(0, Ordering::Relaxed);
        if let Some(p) = pth("lt_revive.txt") { let _ = fs::write(p, "lt_revive_join=0 (원본 — 이 판단은 실행되지 않습니다)\n"); }
        return;
    }
    if !fs2_bytes_eq(addr, &LT_JOIN_ORIG) {
        LT_JOIN_APPLIED.store(0, Ordering::Relaxed);
        if let Some(p) = pth("lt_revive.txt") {
            let _ = fs::write(p, format!("SKIP: 원본 바이트 불일치 @{:#x} — 아무것도 쓰지 않았다\n", addr));
        }
        return;
    }
    let mut stub = LT_JOIN_STUB.load(Ordering::Relaxed);
    if stub == 0 { stub = lt_build_join_stub(addr + LT_JOIN_LEN); LT_JOIN_STUB.store(stub, Ordering::Relaxed); }
    if stub == 0 {
        LT_JOIN_APPLIED.store(0, Ordering::Relaxed);
        if let Some(p) = pth("lt_revive.txt") { let _ = fs::write(p, "SKIP: 스텁 할당 실패\n"); }
        return;
    }
    // 레지스터 무파괴 폼(14B): jmp qword [rip+0] + .quad stub
    let mut patch = [0x90u8; LT_JOIN_LEN];
    patch[0] = 0xff; patch[1] = 0x25;
    patch[2..6].copy_from_slice(&0u32.to_le_bytes());
    patch[6..14].copy_from_slice(&stub.to_le_bytes());
    let ok = fs2_write(addr, &patch);
    LT_JOIN_APPLIED.store(if ok { 1 } else { 0 }, Ordering::Relaxed);
    if let Some(p) = pth("lt_revive.txt") {
        let _ = fs::write(p, format!(
            "lt_revive_join=1 (수정) stub={:#x} @base{:#x}\n\
             「아군에게 붙기」 판단이 살아납니다 — 원본은 자기 자신을 아군으로 골라 영원히 실행되지 않았습니다.\n\
             ⚠개발사가 테스트하지 않은 판단이라 밸런스 영향이 미지수이고, 리플레이 재현이 깨집니다.\n", stub, base));
    }
}

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
    // ⛔[08-04] **`lt_ally_join`은 死노브다 — 게임 측 로직 버그로 이 분기가 도달 불가능하다.**
    //   RE = `RE\2026-08-04_line_total-전수해독-purpose정체-죽은분기-0.5.3.md`
    //   "가장 가까운 아군" argmin 루프(`0xc578c6~0xc57d11`)가 **자기 자신을 제외하지 않아** 항상 자기(거리 0)를 고르고,
    //   게이트 `d²(ally,enemy) < d²(ally,me)=0`이 unsigned 비교라 **영구 false**다.
    //   ⟹ 패치는 유지하되(무해) 유저에게는 "효과 없음"으로 표기한다. 값을 바꿔도 게임 동작은 안 변한다.
    let v_lt_ally_join: u64 = { if lt_ally_join < 0 { 9765625u64 } else { let x = lt_ally_join.max(0) as u64; x.wrapping_mul(x) >> 8 } };
    p!(base + 0xd9fc63, &[0x48,0x81,0xfa], 3, 4, v_lt_ally_join);   // ←0.5.3 c57dcf
    let v_lt_around_radius: u64 = { if lt_around_radius < 0 { 80000u64 } else { lt_around_radius.max(0) as u64 } };
    // ⛔[08-04] `0xc57ed3`은 위 死분기 안의 사이트라 **효과가 없다**(제거해도 무해하나 오해 유발 방지용으로 주석 유지).
    //   살아있는 사이트는 아래 `0xc5816c`·`0xc58296` 둘뿐이다.
    p!(base + 0xd9fd6f, &[0x48,0xc7,0x85,0xf0,0x00,0x00,0x00], 7, 4, v_lt_around_radius);   // ←0.5.3 c57ed3
    p!(base + 0xd9ffe7, &[0x48,0xc7,0x85,0xf0,0x00,0x00,0x00], 7, 4, v_lt_around_radius);   // ←0.5.3 c5816c
    p!(base + 0xda0470, &[0x48,0xc7,0x85,0xf0,0x00,0x00,0x00], 7, 4, v_lt_around_radius);   // ←0.5.3 c58296
    let v_lt_phase_mask: u64 = { if lt_phase_mask < 0 { 417u64 } else { lt_phase_mask.max(0) as u64 } };
    p!(base + 0xd9f4c7, &[0xba], 1, 4, v_lt_phase_mask);   // ←0.5.3 c5763d
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
    macro_rules! pskip { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        // ⛔0.5.4 재배치 미확정 — 세기만 하고 **패치하지 않는다**.
        tot += 1; let _ = ($a, $pre, $off, $w, $v);
    }}; }
    let v_nx_cull_dist19: u64 = { if nx_cull_dist19 < 0 { 390625u64 } else { let x = nx_cull_dist19.max(0) as u64; x.wrapping_mul(x) >> 14 } };
    p!(base + 0xdadb55, &[0x49,0x81,0xf8], 3, 4, v_nx_cull_dist19);   // ←0.5.3 dee222  ★재조사로 복구: nx_cull (053 3곳→054 1곳 통합)
    pskip!(base + 0xdee2b1, &[0x49,0x81,0xf8], 3, 4, v_nx_cull_dist19);   // ⛔0.5.4 미확정: 시그 3→1 / 완화 3→1 (골격 80%)
    pskip!(base + 0xdee335, &[0x49,0x81,0xf8], 3, 4, v_nx_cull_dist19);   // ⛔0.5.4 미확정: 시그 3→1 / 완화 3→1 (골격 80%)
    let v_nx_around_atk: u64 = { if nx_around_atk < 0 { 80000u64 } else { nx_around_atk.max(0) as u64 } };
    p!(base + 0xda1e59, &[0x48,0xc7,0x85,0xb8,0x00,0x00,0x00], 7, 4, v_nx_around_atk);   // ←0.5.3 d95316
    let v_nx_around_def: u64 = { if nx_around_def < 0 { 80000u64 } else { nx_around_def.max(0) as u64 } };
    p!(base + 0xdad2df, &[0x48,0xc7,0x85,0x58,0x01,0x00,0x00], 7, 4, v_nx_around_def);   // ←0.5.3 dedabf
    p!(base + 0xdad51b, &[0x48,0xc7,0x85,0x58,0x01,0x00,0x00], 7, 4, v_nx_around_def);   // ←0.5.3 dedd0a
    p!(base + 0xdad7ea, &[0x48,0xc7,0x85,0x58,0x01,0x00,0x00], 7, 4, v_nx_around_def);   // ←0.5.3 deddf7
    p!(base + 0xdad892, &[0x48,0xc7,0x85,0x58,0x01,0x00,0x00], 7, 4, v_nx_around_def);   // ←0.5.3 dede9f
    NX_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("nx_imm.txt") {
        let _ = fs::write(pp, format!("applied={}/{} nx_cull_dist19={} nx_around_atk={} nx_around_def={} @base{:#x}\n",
            ok, tot, nx_cull_dist19, nx_around_atk, nx_around_def, base));
    }
}

// ★★[08-04 2차] 숨기(`0xca43c0` hide.rs) — 노브가 0개였던 핸들러. 후보 선별 30사이트는 prefix 4종
//   근거 = RE6-08-04_실행층4핸들러-line_total-hide-nexus-노브라벨오류-0.5.3.md
//   전 사이트 정적 검증 통과(`gen_wave2.py` 205/205). 기본 -1 = 원본 유지.
// ★★[08-04 정정] `hide`는 **2상태 기계**다 — 이 사실을 모르고 붙인 옛 라벨이 몇 개 틀렸다.
//   RE = `RE\2026-08-04_hide는2상태기계-랜드마크↔부쉬진동-권장방향정반대-0.5.3.md`
//   · Phase 0(`plan+9==0`) = 목적지가 **부쉬가 아니라 맵 랜드마크**(내 진영 웨이포인트)
//   · Phase 1(`plan+9!=0`) = 실제 부쉬
//   · 전이 = 단방향 래치. 리셋은 **플랜이 새로 만들어질 때만**(생성자 3곳이 phase=0으로 초기화)
//   ⟹ 왕복 = "랜드마크 ↔ 부쉬" 2점 진동. 인게임 실측 = **Phase 0 체류가 51.3%**(hide 총 진입 187,273 중 96,137).
//   ⛔`hd_bush_near`의 옛 설명("이만큼 가까우면 부시로 안 움직임")은 **틀렸다** — 제안을 멈추는 게 아니라
//     **부쉬 단계로 전환**하는 게이트다. 따라서 **낮추면 개선이 아니라 악화**(Phase 0에 더 오래 묶임).
unsafe fn apply_hd_imm() {
    let hd_bush_near = tune("hd_bush_near", -1);   // ★랜드마크에 이만큼 가까워지면 부쉬 단계로 전환(원본 100000, 2곳). ↑=우회 단축
    let hd_path_radius = tune("hd_path_radius", -1);   // Phase0 목표점 무작위 흔들림 반경(원본 60000, 2곳)
    let hd_around_radius = tune("hd_around_radius", -1);   // Phase0 랜드마크 주변 배회 반경(원본 80000, 2곳)
    let hd_detect_max = tune("hd_detect_max", -1);   // 적 후보 최대 탐지거리(원본 250000, ★2곳)
    let hd_fight_cut = tune("hd_fight_cut", -1);   // 교전 후보 컷 거리(원본 150000)
    let hd_cand_select = tune("hd_cand_select", -1);   // 부시·후퇴지점 후보 선별 거리(원본 150000, 30곳)
    let hd_trace_leash = tune("hd_trace_leash", -1);   // 추적 시 붙는 거리(원본 15000)
    let hd_vision_mem = tune("hd_vision_mem", -1);   // 적 목격 정보 유효 틱(원본 120, ★2곳)
    let hd_ph0_ttl = tune("hd_ph0_ttl", -1);   // ★[08-04 신설] Phase0 이동 오더 유효기간 틱(원본 5, 2곳)
    let hd_skip_landmark = tune("hd_skip_landmark", 0);   // ★[08-04 신설] 1=랜드마크 우회를 건너뛰고 처음부터 부쉬로(생성자 3곳)
    let mut sig = 0u64;
    for v in [hd_bush_near, hd_path_radius, hd_around_radius, hd_detect_max, hd_fight_cut, hd_cand_select, hd_trace_leash, hd_vision_mem, hd_ph0_ttl, hd_skip_landmark] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == HD_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32; }}; }
    let v_hd_bush_near: u64 = { if hd_bush_near < 0 { 10000000000u64 } else { let x = hd_bush_near.max(0) as u64; x.wrapping_mul(x) } };
    p!(base + 0xd81ada, &[0x48,0xb8], 2, 8, v_hd_bush_near);   // ←0.5.3 ca46b7
    p!(base + 0xd81c47, &[0x48,0xb8], 2, 8, v_hd_bush_near);   // ←0.5.3 ca4812
    let v_hd_path_radius: u64 = { if hd_path_radius < 0 { 60000u64 } else { hd_path_radius.max(0) as u64 } };
    p!(base + 0xd81b09, &[0x48,0xc7,0x44,0x24,0x20], 5, 4, v_hd_path_radius);   // ←0.5.3 ca46e3
    p!(base + 0xd81c76, &[0x48,0xc7,0x44,0x24,0x20], 5, 4, v_hd_path_radius);   // ←0.5.3 ca483e
    let v_hd_around_radius: u64 = { if hd_around_radius < 0 { 80000u64 } else { hd_around_radius.max(0) as u64 } };
    p!(base + 0xd81ba0, &[0x48,0xc7,0x45,0x08], 4, 4, v_hd_around_radius);   // ←0.5.3 ca471f
    p!(base + 0xd81d0d, &[0x48,0xc7,0x45,0x08], 4, 4, v_hd_around_radius);   // ←0.5.3 ca487a
    let v_hd_detect_max: u64 = { if hd_detect_max < 0 { 62500000001u64 } else { let x = hd_detect_max.max(0) as u64; x.wrapping_mul(x).wrapping_add(1) } };
    p!(base + 0xd81f21, &[0x48,0xba], 2, 8, v_hd_detect_max);   // ←0.5.3 ca4ae1
    // ★[08-04] **반쪽 적용 결함 수정** — 같은 게이트가 축약함수 `0xdc2c90` 안에도 있는데 안 걸려 있었다.
    //   ⚠그쪽은 `+1`이 없는 순수 제곱값이다(hide는 `<=`, 여기는 `<` 비교라 상수가 1 다르다).
    let v_hd_detect_max_h: u64 = { if hd_detect_max < 0 { 62500000000u64 } else { let x = hd_detect_max.max(0) as u64; x.wrapping_mul(x) } };
    //   ⚠주소 주의: `0xdc2dce`는 **상수가 놓인 위치**이고 명령 시작은 그 2바이트 앞인 `0xdc2dcc`다.
    //     (검색 스크립트가 찍는 건 패턴 위치라 그걸 그대로 쓰면 조용히 skip된다 — 실제로 한 번 그랬다.)
    p!(base + 0xe3d8bc, &[0x48,0xba], 2, 8, v_hd_detect_max_h);   // ←0.5.3 dc2dcc
    let v_hd_fight_cut: u64 = { if hd_fight_cut < 0 { 22500000000u64 } else { let x = hd_fight_cut.max(0) as u64; x.wrapping_mul(x) } };
    p!(base + 0xd8259a, &[0x48,0xb8], 2, 8, v_hd_fight_cut);   // ←0.5.3 ca50fa
    let v_hd_cand_select: u64 = { if hd_cand_select < 0 { 22500000001u64 } else { let x = hd_cand_select.max(0) as u64; x.wrapping_mul(x).wrapping_add(1) } };
    p!(base + 0xd82e39, &[0x49,0xb8], 2, 8, v_hd_cand_select);   // ←0.5.3 ca598f
    p!(base + 0xd82e9e, &[0x49,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca59f2
    p!(base + 0xd82f0c, &[0x49,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca5a59
    p!(base + 0xd82f7a, &[0x49,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca5ac0
    p!(base + 0xd82fe7, &[0x49,0xb8], 2, 8, v_hd_cand_select);   // ←0.5.3 ca5b26
    p!(base + 0xd83050, &[0x48,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca5b8f
    p!(base + 0xd83533, &[0x49,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca6080
    p!(base + 0xd8359c, &[0x49,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca60e2
    p!(base + 0xd83608, &[0x49,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca6147
    p!(base + 0xd83674, &[0x49,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca61ac
    p!(base + 0xd836e0, &[0x49,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca6211
    p!(base + 0xd83744, &[0x48,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca6275
    p!(base + 0xd83798, &[0x48,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca62c9
    p!(base + 0xd837eb, &[0x48,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca631c
    p!(base + 0xd8383e, &[0x48,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca636f
    p!(base + 0xd8388f, &[0x48,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca63c0
    p!(base + 0xd83976, &[0x49,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca64a7
    p!(base + 0xd839db, &[0x49,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca650c
    p!(base + 0xd83a45, &[0x49,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca6576
    p!(base + 0xd83aaf, &[0x49,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca65e0
    p!(base + 0xd83ba4, &[0x48,0xba], 2, 8, v_hd_cand_select);   // ←0.5.3 ca66ce
    p!(base + 0xd83d70, &[0x48,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca68a0
    p!(base + 0xd83dc3, &[0x48,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca68f3
    p!(base + 0xd83e16, &[0x48,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca6946
    p!(base + 0xd83e67, &[0x48,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca6997
    p!(base + 0xd83eee, &[0x49,0xb8], 2, 8, v_hd_cand_select);   // ←0.5.3 ca6a1e
    p!(base + 0xd83f55, &[0x49,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca6a85
    p!(base + 0xd83fbf, &[0x49,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca6aef
    p!(base + 0xd84029, &[0x49,0xb9], 2, 8, v_hd_cand_select);   // ←0.5.3 ca6b59
    p!(base + 0xd8408c, &[0x48,0xba], 2, 8, v_hd_cand_select);   // ←0.5.3 ca6bbc
    let v_hd_trace_leash: u64 = { if hd_trace_leash < 0 { 15000u64 } else { hd_trace_leash.max(0) as u64 } };
    p!(base + 0xd842e3, &[0x48,0xc7,0x45,0x28], 4, 4, v_hd_trace_leash);   // ←0.5.3 ca6df5
    let v_hd_vision_mem: u64 = { if hd_vision_mem < 0 { 120u64 } else { hd_vision_mem.max(0) as u64 } };
    p!(base + 0xd81fa5, &[0x49,0x83,0xc7], 3, 1, v_hd_vision_mem);   // ←0.5.3 ca4b65
    // ★[08-04] **반쪽 적용 결함 수정** — 축약함수 쪽 사본(레지스터가 r14라 prefix가 다르다)
    p!(base + 0xe3d866, &[0x49,0x83,0xc6], 3, 1, v_hd_vision_mem);   // ←0.5.3 dc2d76
    // ★[08-04 신설] Phase 0 이동 오더 유효기간
    let v_hd_ph0_ttl: u64 = { if hd_ph0_ttl < 0 { 5u64 } else { hd_ph0_ttl.max(0) as u64 } };
    p!(base + 0xd81ba8, &[0x48,0xc7,0x45,0x10], 4, 4, v_hd_ph0_ttl);   // ←0.5.3 ca4727
    p!(base + 0xd81d15, &[0x48,0xc7,0x45,0x10], 4, 4, v_hd_ph0_ttl);   // ←0.5.3 ca4882
    // ★★[08-04 신설] 랜드마크 우회 건너뛰기 — 생성자가 쓰는 imm16의 상위 바이트가 phase다.
    //   `mov word [x+0x10], 0x0001` = out_line 1 + phase 0  →  `0x0101` 로 만들면 phase 1로 시작한다.
    //   ⟹ Phase 0(랜드마크 우회)이 **처음부터 존재하지 않게** 되어 곧장 부쉬로 간다.
    //   기본 0 = 원본(0x0001)이라 켜지 않으면 동작 변화 0. 끄면 그대로 되돌아간다.
    let v_hd_phase: u64 = if hd_skip_landmark != 0 { 0x0101 } else { 0x0001 };
    p!(base + 0xe1490b, &[0x66,0xc7,0x46,0x10], 4, 2, v_hd_phase);   // ←0.5.3 c55d3b
    p!(base + 0xcd7a3e, &[0x66,0xc7,0x40,0x10], 4, 2, v_hd_phase);   // ←0.5.3 c7a72e
    p!(base + 0xcd75ee, &[0x66,0xc7,0x40,0x10], 4, 2, v_hd_phase);   // ←0.5.3 d81b6e
    HD_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("hd_imm.txt") {
        let _ = fs::write(pp, format!("applied={}/{} hd_bush_near={} hd_path_radius={} hd_around_radius={} hd_detect_max={} hd_fight_cut={} hd_cand_select={} hd_trace_leash={} hd_vision_mem={} hd_ph0_ttl={} hd_skip_landmark={} @base{:#x}\n",
            ok, tot, hd_bush_near, hd_path_radius, hd_around_radius, hd_detect_max, hd_fight_cut, hd_cand_select, hd_trace_leash, hd_vision_mem, hd_ph0_ttl, hd_skip_landmark, base));
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
    p!(base + 0xd671f3, &[0x49,0x81,0xf9], 3, 4, v_d4_ally_radius_a);   // ←0.5.3 d71cd3
    p!(base + 0xd672b8, &[0x49,0x81,0xf9], 3, 4, v_d4_ally_radius_a);   // ←0.5.3 d71d98
    p!(base + 0xd67378, &[0x49,0x81,0xf9], 3, 4, v_d4_ally_radius_a);   // ←0.5.3 d71e58
    p!(base + 0xd67438, &[0x49,0x81,0xf9], 3, 4, v_d4_ally_radius_a);   // ←0.5.3 d71f18
    p!(base + 0xd674fb, &[0x49,0x81,0xf9], 3, 4, v_d4_ally_radius_a);   // ←0.5.3 d71fdb
    let v_d4_ally_radius_b: u64 = { if d4_ally_radius_b < 0 { 87890625u64 } else { let x = d4_ally_radius_b.max(0) as u64; x.wrapping_mul(x) >> 8 } };
    p!(base + 0xd677b6, &[0x49,0x81,0xfc], 3, 4, v_d4_ally_radius_b);   // ←0.5.3 d72296
    p!(base + 0xd677f0, &[0x49,0x81,0xfb], 3, 4, v_d4_ally_radius_b);   // ←0.5.3 d722d0
    p!(base + 0xd67852, &[0x49,0x81,0xfd], 3, 4, v_d4_ally_radius_b);   // ←0.5.3 d72332
    p!(base + 0xd6788d, &[0x49,0x81,0xfd], 3, 4, v_d4_ally_radius_b);   // ←0.5.3 d7236d
    p!(base + 0xd678ee, &[0x49,0x81,0xfd], 3, 4, v_d4_ally_radius_b);   // ←0.5.3 d723ce
    p!(base + 0xd67929, &[0x49,0x81,0xfd], 3, 4, v_d4_ally_radius_b);   // ←0.5.3 d72409
    p!(base + 0xd6798a, &[0x49,0x81,0xfd], 3, 4, v_d4_ally_radius_b);   // ←0.5.3 d7246a
    p!(base + 0xd679c5, &[0x49,0x81,0xfd], 3, 4, v_d4_ally_radius_b);   // ←0.5.3 d724a5
    p!(base + 0xd67a20, &[0x48,0x81,0xfa], 3, 4, v_d4_ally_radius_b);   // ←0.5.3 d72500
    p!(base + 0xd67a62, &[0x48,0x3d], 2, 4, v_d4_ally_radius_b);   // ←0.5.3 d72542
    let v_d4_early_leave: u64 = { if d4_early_leave < 0 { 112890625u64 } else { let x = d4_early_leave.max(0) as u64; x.wrapping_mul(x) >> 8 } };
    p!(base + 0xd67cb1, &[0x48,0x3d], 2, 4, v_d4_early_leave);   // ←0.5.3 d72791
    let v_d4_partner_dist: u64 = { if d4_partner_dist < 0 { 40000000000u64 } else { let x = d4_partner_dist.max(0) as u64; x.wrapping_mul(x) } };
    p!(base + 0xd670df, &[0x48,0xb9], 2, 8, v_d4_partner_dist);   // ←0.5.3 d71bbf
    let v_d4_hp_safe: u64 = { if d4_hp_safe < 0 { 51u64 } else { d4_hp_safe.max(0) as u64 } };
    p!(base + 0xd66f84, &[0x48,0x83,0xf8], 3, 1, v_d4_hp_safe);   // ←0.5.3 d71a64
    let v_d4_from_mid: u64 = { if d4_from_mid < 0 { 1000u64 } else { d4_from_mid.max(0) as u64 } };
    p!(base + 0xd66f78, &[0x49,0x81,0x7c,0x08,0x60], 5, 4, v_d4_from_mid);   // ←0.5.3 d71a58
    let v_d4_from_mid_mode: u64 = { if d4_from_mid_mode < 0 { 2001u64 } else { d4_from_mid_mode.max(0) as u64 } };
    p!(base + 0xd675f5, &[0x48,0x81,0x78,0x10], 4, 4, v_d4_from_mid_mode);   // ←0.5.3 d720d5
    let v_d4_ally_cnt: u64 = { if d4_ally_cnt < 0 { 3u64 } else { d4_ally_cnt.max(0) as u64 } };
    p!(base + 0xd67e16, &[0x48,0x83,0xbc,0x24,0x88,0x00,0x00,0x00], 8, 1, v_d4_ally_cnt);   // ←0.5.3 d728f6
    p!(base + 0xd67e54, &[0x48,0x83,0xbc,0x24,0x88,0x00,0x00,0x00], 8, 1, v_d4_ally_cnt);   // ←0.5.3 d72934
    let v_d4_minion_cnt: u64 = { if d4_minion_cnt < 0 { 2u64 } else { d4_minion_cnt.max(0) as u64 } };
    p!(base + 0xd67e2a, &[0x83,0xfe], 2, 1, v_d4_minion_cnt);   // ←0.5.3 d7290a
    let v_d4_gather_radius: u64 = { if d4_gather_radius < 0 { 150000u64 } else { d4_gather_radius.max(0) as u64 } };
    p!(base + 0xd676e7, &[0x48,0xc7,0x44,0x24,0x40], 5, 4, v_d4_gather_radius);   // ←0.5.3 d721c7
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
    p!(base + 0xc84e04, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3b6f5
    p!(base + 0xc84e96, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3b790
    p!(base + 0xc84f1d, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3b820
    p!(base + 0xc84fa4, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3b8b0
    p!(base + 0xc8502b, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3b937
    p!(base + 0xc85327, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3bc41
    p!(base + 0xc853b9, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3bcdc
    p!(base + 0xc85440, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3bd6c
    p!(base + 0xc854c7, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3bdfc
    p!(base + 0xc8554e, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3be83
    p!(base + 0xc8584a, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3c192
    p!(base + 0xc858dc, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3c22d
    p!(base + 0xc85963, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3c2bd
    p!(base + 0xc859ea, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3c34d
    p!(base + 0xc85a6c, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3c3d4
    p!(base + 0xc8603b, &[0x48,0xb8], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3c9c6
    p!(base + 0xc841da, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3aa44
    p!(base + 0xc8426b, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3aade
    p!(base + 0xc842f2, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3ab6e
    p!(base + 0xc84379, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3abfe
    p!(base + 0xc84400, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3ac85
    p!(base + 0xc845f6, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3ae8e
    p!(base + 0xc84687, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3af28
    p!(base + 0xc8470e, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3afb8
    p!(base + 0xc84795, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3b048
    p!(base + 0xc8481c, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3b0cf
    p!(base + 0xc84a14, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3b2d5
    p!(base + 0xc84aa5, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3b36f
    p!(base + 0xc84b2c, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3b3ff
    p!(base + 0xc84bb3, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3b48f
    p!(base + 0xc84c35, &[0x48,0xb9], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3b516
    p!(base + 0xc85e18, &[0x49,0xb9], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3c794
    p!(base + 0xc85fa7, &[0x49,0xb9], 2, 8, v_c3_enemy_near_a);   // ←0.5.3 c3c92d
    let v_c3_enemy_near_b: u64 = { if c3_enemy_near_b < 0 { 14400000001u64 } else { let x = c3_enemy_near_b.max(0) as u64; x.wrapping_mul(x).wrapping_add(1) } };
    p!(base + 0xc85c36, &[0x48,0xb8], 2, 8, v_c3_enemy_near_b);   // ←0.5.3 c3c5a3
    p!(base + 0xc85c9f, &[0x48,0xb8], 2, 8, v_c3_enemy_near_b);   // ←0.5.3 c3c611
    p!(base + 0xc86175, &[0x48,0xb8], 2, 8, v_c3_enemy_near_b);   // ←0.5.3 c3ca96
    p!(base + 0xc851ee, &[0x48,0xb9], 2, 8, v_c3_enemy_near_b);   // ←0.5.3 c3baf5
    p!(base + 0xc85711, &[0x48,0xb9], 2, 8, v_c3_enemy_near_b);   // ←0.5.3 c3c041
    p!(base + 0xc84504, &[0x48,0xbf], 2, 8, v_c3_enemy_near_b);   // ←0.5.3 c3ad8e
    p!(base + 0xc84920, &[0x48,0xbf], 2, 8, v_c3_enemy_near_b);   // ←0.5.3 c3b1d8
    p!(base + 0xc84d32, &[0x49,0xb9], 2, 8, v_c3_enemy_near_b);   // ←0.5.3 c3b61a
    p!(base + 0xc85255, &[0x49,0xb9], 2, 8, v_c3_enemy_near_b);   // ←0.5.3 c3bb61
    p!(base + 0xc85778, &[0x49,0xb9], 2, 8, v_c3_enemy_near_b);   // ←0.5.3 c3c0ad
    p!(base + 0xc85d01, &[0x49,0xb9], 2, 8, v_c3_enemy_near_b);   // ←0.5.3 c3c678
    p!(base + 0xc85e90, &[0x49,0xb9], 2, 8, v_c3_enemy_near_b);   // ←0.5.3 c3c811
    p!(base + 0xc85dba, &[0x49,0xba], 2, 8, v_c3_enemy_near_b);   // ←0.5.3 c3c736
    p!(base + 0xc85f49, &[0x49,0xba], 2, 8, v_c3_enemy_near_b);   // ←0.5.3 c3c8cf
    p!(base + 0xc8622e, &[0x49,0xba], 2, 8, v_c3_enemy_near_b);   // ←0.5.3 c3cb55
    let v_c3_minion_near: u64 = { if c3_minion_near < 0 { 14400000001u64 } else { let x = c3_minion_near.max(0) as u64; x.wrapping_mul(x).wrapping_add(1) } };
    p!(base + 0xe4f3fd, &[0x49,0xba], 2, 8, v_c3_minion_near);   // ←0.5.3 d90bfd
    p!(base + 0xe4f486, &[0x49,0xba], 2, 8, v_c3_minion_near);   // ←0.5.3 d90c86
    p!(base + 0xe4f50e, &[0x49,0xba], 2, 8, v_c3_minion_near);   // ←0.5.3 d90d0e
    let v_c3_ally_hp: u64 = { if c3_ally_hp < 0 { 79u64 } else { c3_ally_hp.max(0) as u64 } };
    p!(base + 0xc83def, &[0x48,0x83,0xf8], 3, 1, v_c3_ally_hp);   // ←0.5.3 c3a629
    p!(base + 0xc83f39, &[0x48,0x83,0xf8], 3, 1, v_c3_ally_hp);   // ←0.5.3 c3a77f
    p!(base + 0xc84083, &[0x48,0x83,0xf8], 3, 1, v_c3_ally_hp);   // ←0.5.3 c3a8d5
    p!(base + 0xc84d4f, &[0x48,0x83,0xf8], 3, 1, v_c3_ally_hp);   // ←0.5.3 c3b637
    p!(base + 0xc85272, &[0x48,0x83,0xf8], 3, 1, v_c3_ally_hp);   // ←0.5.3 c3bb83
    p!(base + 0xc85795, &[0x48,0x83,0xf8], 3, 1, v_c3_ally_hp);   // ←0.5.3 c3c0cf
    let v_c3_minion_margin: u64 = { if c3_minion_margin < 0 { 64000u64 } else { c3_minion_margin.max(0) as u64 } };
    p!(base + 0xd64c0a, &[0x48,0x05], 2, 4, v_c3_minion_margin);   // ←0.5.3 e2321a
    let v_c3_hurt_scale: u64 = { if c3_hurt_scale < 0 { 100u64 } else { c3_hurt_scale.max(0) as u64 } };
    p!(base + 0xcdced8, &[0x48,0x6b,0x97,0x58,0x06,0x00,0x00], 7, 1, v_c3_hurt_scale);   // ←0.5.3 cc7d38
    p!(base + 0xcdcf9a, &[0x48,0x6b,0x96,0x58,0x06,0x00,0x00], 7, 1, v_c3_hurt_scale);   // ←0.5.3 cc7dfa
    p!(base + 0xcdd051, &[0x48,0x6b,0x96,0x58,0x06,0x00,0x00], 7, 1, v_c3_hurt_scale);   // ←0.5.3 cc7eb1
    p!(base + 0xcdd108, &[0x48,0x6b,0x96,0x58,0x06,0x00,0x00], 7, 1, v_c3_hurt_scale);   // ←0.5.3 cc7f68
    p!(base + 0xcdd1b9, &[0x49,0x6b,0x8b,0x58,0x06,0x00,0x00], 7, 1, v_c3_hurt_scale);   // ←0.5.3 cc8019
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
    p!(base + 0x1066edd, &[0x49,0x83,0xff], 3, 1, v_ex_ult_level_x);   // ←0.5.3 fdb9ed
    p!(base + 0x1066f85, &[0x49,0x83,0xff], 3, 1, v_ex_ult_level_x);   // ←0.5.3 fdba95
    p!(base + 0x106705d, &[0x49,0x83,0xff], 3, 1, v_ex_ult_level_x);   // ←0.5.3 fdbb6d
    p!(base + 0xdb308a, &[0x49,0x83,0xbe,0xb0,0x05,0x00,0x00], 7, 1, v_ex_ult_level_x);   // ←0.5.3 c8ab0a
    p!(base + 0xc83f95, &[0x49,0x83,0xbd,0xb0,0x05,0x00,0x00], 7, 1, v_ex_ult_level_x);   // ←0.5.3 c3a7db
    let v_ex_skill2_level_x: u64 = { if ex_skill2_level_x < 0 { 3u64 } else { ex_skill2_level_x.max(0) as u64 } };
    p!(base + 0x1056e3d, &[0x49,0x83,0xff], 3, 1, v_ex_skill2_level_x);   // ←0.5.3 fcb9ad
    p!(base + 0x1056ede, &[0x49,0x83,0xff], 3, 1, v_ex_skill2_level_x);   // ←0.5.3 fcba4e
    p!(base + 0x1056fad, &[0x49,0x83,0xff], 3, 1, v_ex_skill2_level_x);   // ←0.5.3 fcbb1d
    p!(base + 0xdb3064, &[0x49,0x83,0xbe,0xb0,0x05,0x00,0x00], 7, 1, v_ex_skill2_level_x);   // ←0.5.3 c8aae4
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
    p!(base + 0xd861a1, &[0x48,0x83,0xf8], 3, 1, v_eh_flee_clear_hp);   // ←0.5.3 c68992
    p!(base + 0xd8628b, &[0x48,0x83,0xf8], 3, 1, v_eh_flee_clear_hp);   // ←0.5.3 c68a7f
    p!(base + 0xe14b2e, &[0x48,0x83,0xf8], 3, 1, v_eh_flee_clear_hp);   // ←0.5.3 da0825
    p!(base + 0xe14bd1, &[0x48,0x83,0xf8], 3, 1, v_eh_flee_clear_hp);   // ←0.5.3 da08c5
    let v_eh_reach_margin: u64 = { if eh_reach_margin < 0 { 25000u64 } else { eh_reach_margin.max(0) as u64 } };
    p!(base + 0xd87fbd, &[0x41,0xb8], 2, 4, v_eh_reach_margin);   // ←0.5.3 c6a83d
    p!(base + 0xe1681d, &[0x41,0xb8], 2, 4, v_eh_reach_margin);   // ←0.5.3 da253e
    p!(base + 0xd8ad23, &[0x48,0xc7,0x44,0x24,0x30], 5, 4, v_eh_reach_margin);   // ←0.5.3 c6d3b5
    p!(base + 0xe19563, &[0x48,0xc7,0x44,0x24,0x30], 5, 4, v_eh_reach_margin);   // ←0.5.3 da4fc3
    let v_eh_recall_radius: u64 = { if eh_recall_radius < 0 { 60000u64 } else { eh_recall_radius.max(0) as u64 } };
    p!(base + 0xd87685, &[0x48,0xc7,0x44,0x24,0x20], 5, 4, v_eh_recall_radius);   // ←0.5.3 c69fd5
    p!(base + 0xe15f85, &[0x48,0xc7,0x44,0x24,0x20], 5, 4, v_eh_recall_radius);   // ←0.5.3 da1ce2
    let v_eh_around_radius: u64 = { if eh_around_radius < 0 { 80000u64 } else { eh_around_radius.max(0) as u64 } };
    p!(base + 0xd8774e, &[0x48,0xc7,0x85,0x28,0x05,0x00,0x00], 7, 4, v_eh_around_radius);   // ←0.5.3 c6a026
    p!(base + 0xe16048, &[0x48,0xc7,0x85,0x28,0x05,0x00,0x00], 7, 4, v_eh_around_radius);   // ←0.5.3 da1d33
    let v_eh_trace_arrive: u64 = { if eh_trace_arrive < 0 { 15000u64 } else { eh_trace_arrive.max(0) as u64 } };
    p!(base + 0xd87aba, &[0x48,0xc7,0x85,0x48,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);   // ←0.5.3 c6a349
    p!(base + 0xd8856a, &[0x48,0xc7,0x85,0x48,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);   // ←0.5.3 c6aaff
    p!(base + 0xd88609, &[0x48,0xc7,0x85,0x48,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);   // ←0.5.3 c6ac5d
    p!(base + 0xd886a8, &[0x48,0xc7,0x85,0x48,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);   // ←0.5.3 c6acf9
    p!(base + 0xd8938b, &[0x48,0xc7,0x85,0x48,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);   // ←0.5.3 c6ba21
    p!(base + 0xe1632f, &[0x48,0xc7,0x85,0x48,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);   // ←0.5.3 da2049
    p!(base + 0xe16dca, &[0x48,0xc7,0x85,0x48,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);   // ←0.5.3 da2837
    p!(base + 0xe16e69, &[0x48,0xc7,0x85,0x48,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);   // ←0.5.3 da28cf
    p!(base + 0xe16f08, &[0x48,0xc7,0x85,0x48,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);   // ←0.5.3 da2967
    p!(base + 0xe17bbb, &[0x48,0xc7,0x85,0x48,0x05,0x00,0x00], 7, 4, v_eh_trace_arrive);   // ←0.5.3 da366b
    let v_eh_band_low: u64 = { if eh_band_low < 0 { 12000u64 } else { eh_band_low.max(0) as u64 } };
    p!(base + 0xd88cfe, &[0xb9], 1, 4, v_eh_band_low);   // ←0.5.3 c6b3a1
    p!(base + 0xe1755e, &[0xb9], 1, 4, v_eh_band_low);   // ←0.5.3 da300a
    p!(base + 0xd88d4f, &[0xbe], 1, 4, v_eh_band_low);   // ←0.5.3 c6b3eb
    p!(base + 0xe175af, &[0xbe], 1, 4, v_eh_band_low);   // ←0.5.3 da305b
    p!(base + 0xd88d59, &[0x48,0xc7,0x85,0xd0,0x04,0x00,0x00], 7, 4, v_eh_band_low);   // ←0.5.3 c6b3f5
    p!(base + 0xe175b9, &[0x48,0xc7,0x85,0xd0,0x04,0x00,0x00], 7, 4, v_eh_band_low);   // ←0.5.3 da3065
    p!(base + 0xd88cf8, &[0x48,0x3d], 2, 4, v_eh_band_low.wrapping_add(1));   // ←0.5.3 c6b39b
    p!(base + 0xe17558, &[0x48,0x3d], 2, 4, v_eh_band_low.wrapping_add(1));   // ←0.5.3 da3004
    let v_eh_band_high: u64 = { if eh_band_high < 0 { 45000u64 } else { eh_band_high.max(0) as u64 } };
    p!(base + 0xd88d07, &[0x48,0x81,0xf9], 3, 4, v_eh_band_high);   // ←0.5.3 c6b3aa
    p!(base + 0xe17567, &[0x48,0x81,0xf9], 3, 4, v_eh_band_high);   // ←0.5.3 da3013
    p!(base + 0xd88d0e, &[0xbb], 1, 4, v_eh_band_high);   // ←0.5.3 c6b3b1
    p!(base + 0xe1756e, &[0xbb], 1, 4, v_eh_band_high);   // ←0.5.3 da301a
    let v_eh_commit_hp: u64 = { if eh_commit_hp < 0 { 50u64 } else { eh_commit_hp.max(0) as u64 } };
    p!(base + 0xd89646, &[0x48,0x83,0xbd,0x88,0x05,0x00,0x00], 7, 1, v_eh_commit_hp);   // ←0.5.3 c6bcb8
    p!(base + 0xe17e74, &[0x48,0x83,0xbd,0x88,0x05,0x00,0x00], 7, 1, v_eh_commit_hp);   // ←0.5.3 da3924
    let v_eh_commit_r_low: u64 = { if eh_commit_r_low < 0 { 70000u64 } else { eh_commit_r_low.max(0) as u64 } };
    p!(base + 0xd8964e, &[0xb8], 1, 4, v_eh_commit_r_low);   // ←0.5.3 c6bcc0
    p!(base + 0xe17e7c, &[0xb8], 1, 4, v_eh_commit_r_low);   // ←0.5.3 da392c
    let v_eh_commit_r_high: u64 = { if eh_commit_r_high < 0 { 40000u64 } else { eh_commit_r_high.max(0) as u64 } };
    p!(base + 0xd89653, &[0x41,0xbd], 2, 4, v_eh_commit_r_high);   // ←0.5.3 c6bcc5
    p!(base + 0xe17e81, &[0x41,0xbd], 2, 4, v_eh_commit_r_high);   // ←0.5.3 da3931
    let v_eh_abort_hp: u64 = { if eh_abort_hp < 0 { 44u64 } else { eh_abort_hp.max(0) as u64 } };
    p!(base + 0xd896e4, &[0x48,0x83,0xbd,0x88,0x05,0x00,0x00], 7, 1, v_eh_abort_hp);   // ←0.5.3 c6bd5d
    p!(base + 0xe17f12, &[0x48,0x83,0xbd,0x88,0x05,0x00,0x00], 7, 1, v_eh_abort_hp);   // ←0.5.3 da39c2
    let v_eh_abort_dist: u64 = { if eh_abort_dist < 0 { 220000u64 } else { eh_abort_dist.max(0) as u64 } };
    p!(base + 0xd896f2, &[0x48,0x81,0xbd,0x60,0x03,0x00,0x00], 7, 4, v_eh_abort_dist);   // ←0.5.3 c6bd6b
    p!(base + 0xe17f20, &[0x48,0x81,0xbd,0x60,0x03,0x00,0x00], 7, 4, v_eh_abort_dist);   // ←0.5.3 da39d0
    let v_eh_score_norm: u64 = { if eh_score_norm < 0 { 320000u64 } else { eh_score_norm.max(0) as u64 } };
    p!(base + 0xd89a90, &[0x48,0x3d], 2, 4, v_eh_score_norm);   // ←0.5.3 c6c145
    p!(base + 0xd89a96, &[0x41,0xb8], 2, 4, v_eh_score_norm);   // ←0.5.3 c6c14b
    p!(base + 0xd89adc, &[0xba], 1, 4, v_eh_score_norm);   // ←0.5.3 c6c191
    p!(base + 0xd89d9f, &[0x48,0x3d], 2, 4, v_eh_score_norm);   // ←0.5.3 c6c45e
    p!(base + 0xd89da5, &[0xb9], 1, 4, v_eh_score_norm);   // ←0.5.3 c6c464
    p!(base + 0xd89e2b, &[0xb9], 1, 4, v_eh_score_norm);   // ←0.5.3 c6c4ea
    p!(base + 0xe182d0, &[0x48,0x3d], 2, 4, v_eh_score_norm);   // ←0.5.3 da3d96
    p!(base + 0xe182d6, &[0x41,0xb8], 2, 4, v_eh_score_norm);   // ←0.5.3 da3d9c
    p!(base + 0xe1831c, &[0xba], 1, 4, v_eh_score_norm);   // ←0.5.3 da3de2
    p!(base + 0xe185df, &[0x48,0x3d], 2, 4, v_eh_score_norm);   // ←0.5.3 da409e
    p!(base + 0xe185e5, &[0xb9], 1, 4, v_eh_score_norm);   // ←0.5.3 da40a4
    p!(base + 0xe1866b, &[0xb9], 1, 4, v_eh_score_norm);   // ←0.5.3 da412a
    EH_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("eh_imm.txt") {
        let _ = fs::write(pp, format!("applied={}/{} eh_flee_clear_hp={} eh_reach_margin={} eh_recall_radius={} eh_around_radius={} eh_trace_arrive={} eh_band_low={} eh_band_high={} eh_commit_hp={} eh_commit_r_low={} eh_commit_r_high={} eh_abort_hp={} eh_abort_dist={} eh_score_norm={} @base{:#x}\n",
            ok, tot, eh_flee_clear_hp, eh_reach_margin, eh_recall_radius, eh_around_radius, eh_trace_arrive, eh_band_low, eh_band_high, eh_commit_hp, eh_commit_r_low, eh_commit_r_high, eh_abort_hp, eh_abort_dist, eh_score_norm, base));
    }
}

// ════════════════════════════════════════════════════════════════════════════════
// ★★★[08-04 신설] 게임 원본 결함 수정 — 적 상대 위협/이득 계산에서 **스킬2 피해가 무시되는** 버그
//   근거 = RE\2026-08-04_d07a60-위협디스크립터-생산자-스킬2버그-0.5.3.md (버그 발견)
//          RE\2026-08-04_스킬2버그-수정지점-d08404-d086d4-0.5.3.md   (수정 지점 확정)
//
// [무엇이 잘못됐나]
//   `0xd07a60`(score_parameter.rs)의 **적 분기**는 피해 매트릭스 4열 중 슬롯2(스킬2, 열 `+0x50`/`+0x1e0`)를
//   **아예 계산하지 않고** 슬롯1(스킬1) 결과를 그대로 복사한다.
//     0xd087d8  mov [rsi+0x10], rdx   ; ← 0xd087c8 의 스킬1 값과 같은 rdx
//     0xd08812  mov [rsi+0x30], r10   ; ← 0xd0880e 의 스킬1 값과 같은 r10
//   함수 전체에서 `+0x50`/`+0x1e0` 접근이 0회(실측). **아군 분기는 정상.**
//   ⟹ AI가 "저 자리로 가면 얼마나 아플까"를 셀 때 **적의 스킬2를 통째로 빼먹는다.**
//
// [왜 4바이트 교체로는 못 고치나]
//   `0xd087c0` 시점엔 행렬 베이스는 살아 있지만 **인덱스(param_4)와 두 분모가 이미 클로버**돼 있다.
//   올바른 값을 만들 재료가 그 자리에 없다.
//
// [고치는 방법 = 훅 2 + NOP 2]
//   행렬·인덱스·분모·출력포인터가 **동시에 살아 있는** 두 지점에서 미리 계산해 넣고,
//   뒤의 잘못된 store 2개를 NOP 한다. 두 지점 모두 해당 분기의 straight-line 상이라 무조건 실행된다.
//     A: 0xd08404 (내/타깃 쪽)  r13=행렬 rbp=idx rbx=분모(≥1 보정됨) rsi=출력  → [rsi+0x10]
//     B: 0xd086d4 (gain 쪽)     rdx=행렬 r12=idx r13=분모            rsi=출력  → [rsi+0x30]
//   공식(같은 분기 형제 3개에서 도출) = `min(150, ((M[half+idx*8]>>1) + M[val+idx*8]) * 100 / max(1,분모))`
//   슬롯2의 (val, half) = (0x050, 0x1e0).
//
// [안전성 근거]
//   · 두 사이트 16B는 명령 경계 정확히 일치, rip-relative 없음(그대로 재실행 가능)
//   · 진입 시 rax는 dead(첫 명령이 덮어씀) ⟹ `movabs rax,stub; jmp rax` 12B + NOP 4B 로 후킹 가능
//   · 함수 내 분기 타깃 106개 전수에 이 구간 진입 0건 · xref 0건 · `.pdata` unwind flags=0
//   · `[rsi+0x10]`·`[rsi+0x30]` 을 쓰는 명령은 각각 단 1곳뿐 ⟹ 조기 write 안전
//   · 스텁은 게임 레지스터를 하나도 안 건드린다(push/pushfq 로 rax·rcx·rdx·플래그 보존)
//   · 분모 0 방어를 넣었다(원본은 cmp/adc 로 이미 보정하지만 방어적으로)
//
// ⚠기본 OFF. `fix_skill2_dmg=1` 일 때만 적용된다.
// ⚠호출자 `0xd37580` 이 결과를 200엔트리 메모 캐시에 담으므로, 토글 직후엔 캐시가 갱신될 때까지 반영이 늦다.
// ════════════════════════════════════════════════════════════════════════════════
const FS2_A_RVA:  usize = 0xd08404;   // 훅 A 진입
const FS2_B_RVA:  usize = 0xd086d4;   // 훅 B 진입
const FS2_N1_RVA: usize = 0xd087d8;   // 잘못된 store (내/타깃)
const FS2_N2_RVA: usize = 0xd08812;   // 잘못된 store (gain)
const FS2_A_ORIG: [u8; 16] = [0x49,0x8b,0x84,0xed,0x08,0x02,0x00,0x00, 0x48,0xd1,0xe8, 0x49,0x03,0x44,0xed,0x78];
const FS2_B_ORIG: [u8; 16] = [0x4a,0x8b,0x84,0xe2,0x08,0x02,0x00,0x00, 0x48,0xd1,0xe8, 0x4a,0x03,0x44,0xe2,0x78];
const FS2_N1_ORIG: [u8; 4] = [0x48,0x89,0x56,0x10];   // mov [rsi+0x10], rdx
const FS2_N2_ORIG: [u8; 4] = [0x4c,0x89,0x56,0x30];   // mov [rsi+0x30], r10
const FS2_NOP4:    [u8; 4] = [0x0f,0x1f,0x40,0x00];   // nop dword [rax+0]

static FS2_APPLIED: AtomicI64 = AtomicI64::new(-1);   // -1=미결정 / 0=원본 / 1=수정

/// 지정 주소에 바이트열을 쓴다(RWX 전환 → 복사 → 권한복구 → i-cache flush).
unsafe fn fs2_write(addr: usize, bytes: &[u8]) -> bool {
    let mut old: u32 = 0;
    if VirtualProtect(addr, bytes.len(), 0x40, &mut old) == 0 { return false; }
    core::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, bytes.len());
    VirtualProtect(addr, bytes.len(), old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, bytes.len());
    true
}

unsafe fn fs2_bytes_eq(addr: usize, want: &[u8]) -> bool {
    if !readable(addr, want.len()) { return false; }
    for (i, &b) in want.iter().enumerate() { if rd_u8(addr + i) != b { return false; } }
    true
}

/// 스킬2 값을 계산해 out에 쓰는 스텁을 만든다.
///  `mode 0` = 훅 A (M=r13, idx=rbp, div=rbx, out=[rsi+0x10])
///  `mode 1` = 훅 B (M=rdx, idx=r12, div=r13, out=[rsi+0x30])
unsafe fn fs2_build_stub(mode: u32, orig: &[u8; 16], ret_addr: usize) -> usize {
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = stub_reg(VirtualAlloc(0, 256, MEM_CR, RWX), 256, 0xF10 + mode as usize);
    if stub == 0 { return 0; }
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x50, 0x51, 0x52, 0x9c]);          // push rax; push rcx; push rdx; pushfq
    if mode == 0 {
        s.extend_from_slice(&[0x49,0x8b,0x84,0xed,0xe0,0x01,0x00,0x00]); // mov rax,[r13+rbp*8+0x1e0]
        s.extend_from_slice(&[0x48,0xd1,0xe8]);                          // shr rax,1
        s.extend_from_slice(&[0x49,0x03,0x44,0xed,0x50]);                // add rax,[r13+rbp*8+0x50]
        s.extend_from_slice(&[0x48,0x6b,0xc0,0x64]);                     // imul rax,rax,100
        s.extend_from_slice(&[0x48,0x89,0xd9]);                          // mov rcx,rbx        (분모)
    } else {
        s.extend_from_slice(&[0x4a,0x8b,0x84,0xe2,0xe0,0x01,0x00,0x00]); // mov rax,[rdx+r12*8+0x1e0]
        s.extend_from_slice(&[0x48,0xd1,0xe8]);                          // shr rax,1
        s.extend_from_slice(&[0x4a,0x03,0x44,0xe2,0x50]);                // add rax,[rdx+r12*8+0x50]
        s.extend_from_slice(&[0x48,0x6b,0xc0,0x64]);                     // imul rax,rax,100
        s.extend_from_slice(&[0x4c,0x89,0xe9]);                          // mov rcx,r13        (분모)
    }
    s.extend_from_slice(&[0x48,0x85,0xc9]);                              // test rcx,rcx
    s.extend_from_slice(&[0x75,0x05]);                                   // jnz +5
    s.extend_from_slice(&[0xb9,0x01,0x00,0x00,0x00]);                    // mov ecx,1   (0 나눗셈 방어)
    s.extend_from_slice(&[0x31,0xd2]);                                   // xor edx,edx
    s.extend_from_slice(&[0x48,0xf7,0xf1]);                              // div rcx
    s.extend_from_slice(&[0x48,0x3d,0x96,0x00,0x00,0x00]);               // cmp rax,150
    s.extend_from_slice(&[0xb9,0x96,0x00,0x00,0x00]);                    // mov ecx,150
    s.extend_from_slice(&[0x48,0x0f,0x42,0xc8]);                         // cmovb rcx,rax   = min(.,150)
    s.extend_from_slice(if mode == 0 { &[0x48,0x89,0x4e,0x10] }          // mov [rsi+0x10],rcx
                        else         { &[0x48,0x89,0x4e,0x30] });        // mov [rsi+0x30],rcx
    s.extend_from_slice(&[0x9d, 0x5a, 0x59, 0x58]);                      // popfq; pop rdx; pop rcx; pop rax
    s.extend_from_slice(orig);                                           // 훔친 원본 16B 재실행
    s.extend_from_slice(&[0xff,0x25,0x00,0x00,0x00,0x00]);               // jmp qword [rip+0]
    s.extend_from_slice(&ret_addr.to_le_bytes());                        //   ← 복귀 주소(레지스터 미사용)
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());
    stub
}

/// `fix_skill2_dmg` 토글 처리. 켜면 훅 2 + NOP 2, 끄면 원본 4곳 복원.
unsafe fn apply_fix_skill2() {
    let want = if tune("fix_skill2_dmg", 0) != 0 { 1i64 } else { 0i64 };
    if FS2_APPLIED.load(Ordering::Relaxed) == want { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let (a, b, n1, n2) = (base + FS2_A_RVA, base + FS2_B_RVA, base + FS2_N1_RVA, base + FS2_N2_RVA);

    if want == 0 {
        // ── 원본 복원 ── (한 번도 안 걸었으면 아무것도 안 한다)
        if FS2_APPLIED.load(Ordering::Relaxed) == 1 {
            fs2_write(a, &FS2_A_ORIG); fs2_write(b, &FS2_B_ORIG);
            fs2_write(n1, &FS2_N1_ORIG); fs2_write(n2, &FS2_N2_ORIG);
        }
        FS2_APPLIED.store(0, Ordering::Relaxed);
        if let Some(p) = pth("fix_imm.txt") {
            let _ = fs::write(p, format!("fix_skill2_dmg=0 (원본) @base{:#x}\n", base));
        }
        return;
    }

    // ── 적용 전 원본 바이트 전수 대조. 하나라도 다르면 **아무것도 하지 않는다.** ──
    let ok_a  = fs2_bytes_eq(a,  &FS2_A_ORIG);
    let ok_b  = fs2_bytes_eq(b,  &FS2_B_ORIG);
    let ok_n1 = fs2_bytes_eq(n1, &FS2_N1_ORIG);
    let ok_n2 = fs2_bytes_eq(n2, &FS2_N2_ORIG);
    if !(ok_a && ok_b && ok_n1 && ok_n2) {
        FS2_APPLIED.store(0, Ordering::Relaxed);
        if let Some(p) = pth("fix_imm.txt") {
            let _ = fs::write(p, format!(
                "SKIP: 원본 바이트 불일치 A={} B={} N1={} N2={} @base{:#x}\n\
                 (게임 패치로 주소가 옮겨졌거나 다른 모드가 먼저 건드린 상태 — 아무것도 쓰지 않았다)\n",
                ok_a, ok_b, ok_n1, ok_n2, base));
        }
        return;
    }

    let sa = fs2_build_stub(0, &FS2_A_ORIG, a + 16);
    let sb = fs2_build_stub(1, &FS2_B_ORIG, b + 16);
    if sa == 0 || sb == 0 {
        FS2_APPLIED.store(0, Ordering::Relaxed);
        if let Some(p) = pth("fix_imm.txt") { let _ = fs::write(p, "SKIP: 스텁 할당 실패\n"); }
        return;
    }
    // 진입 16B → movabs rax,stub; jmp rax (12B) + nop 4B. rax 는 진입 시 dead.
    let mut pa = [0x90u8; 16];
    pa[0] = 0x48; pa[1] = 0xb8; pa[2..10].copy_from_slice(&sa.to_le_bytes()); pa[10] = 0xff; pa[11] = 0xe0;
    let mut pb = [0x90u8; 16];
    pb[0] = 0x48; pb[1] = 0xb8; pb[2..10].copy_from_slice(&sb.to_le_bytes()); pb[10] = 0xff; pb[11] = 0xe0;
    // ★NOP 을 먼저 건다 — 훅이 먼저 걸리면 그 사이 틱에 "새 값을 썼다가 옛 store 가 덮어쓰는" 창이 생긴다.
    let r1 = fs2_write(n1, &FS2_NOP4);
    let r2 = fs2_write(n2, &FS2_NOP4);
    let r3 = fs2_write(a, &pa);
    let r4 = fs2_write(b, &pb);
    FS2_APPLIED.store(if r1 && r2 && r3 && r4 { 1 } else { 0 }, Ordering::Relaxed);
    if let Some(p) = pth("fix_imm.txt") {
        let _ = fs::write(p, format!(
            "fix_skill2_dmg=1 (수정) hookA={:#x} hookB={:#x} nop1={} nop2={} @base{:#x}\n\
             적 스킬2 피해가 위협·이득 계산에 반영됩니다(원본은 스킬1 값을 두 번 씀).\n",
            sa, sb, r1, r2, base));
    }
}

// ════════════════════════════════════════════════════════════════════════════════
// ★게임 결함 수정 ②: AI 피해 예측이 `hp_ratio`(체력 비례 계수)를 무시한다
// ════════════════════════════════════════════════════════════════════════════════
// 실제 전투(`0x1551100` / `0x1538d70`)는 이렇게 계산한다:
//     피해 = damage + attack_ratio×공격력/100 + hp_ratio×시전자최대HP/100
//                                            + target_hp_ratio×대상최대HP/100
// 그런데 AI가 "이 스킬 얼마나 아플까"를 물을 때 타는 예측 함수는 62바이트짜리로,
//     예측 = damage + attack_ratio×공격력/100
// 만 계산하고 `hp_ratio`(구조체 `+0x20`)를 **아예 읽지 않는다.**
// ⟹ 체력 비례 스킬은 AI가 제 위력을 과소평가하고(공격), 맞을 위험도 과소평가한다(방어).
//
// [수정 방법 = 코드 패치가 아니라 vtable 슬롯 교체]
//   예측 함수는 Effect trait vtable 의 `+0x28` 슬롯에 걸려 있다. 그 슬롯을 담은
//   `.rdata` 주소 18곳(AttackEffect·FixedAttackEffect 10 + ApAttackEffect 8)에
//   내 스텁 주소를 써 넣으면 된다. 코드는 한 바이트도 안 건드린다.
//     · `.pdata`/unwind 정보와 무관 (원본 함수가 그대로 살아 있음)
//     · 타입 정확 — 다른 Effect 종류로 전파 0
//     · 되돌리기 = 원래 함수 주소를 다시 써 넣으면 끝
//
// [스텁이 하는 일]  원본 항 + `hp_ratio×시전자최대HP/100` 한 항 추가.
//   · ÷100 은 **항마다 따로** 한다 — 실제 전투가 그렇게 하므로 합쳐서 나누면 값이 어긋난다.
//   · `hp_ratio <= 0` 이면 그 항을 통째로 건너뛴다 ⟹ 안 쓰는 스킬은 원본과 **비트동일**.
//   · 반환은 rax=물리 / rdx=마법 **2워드**라 C ABI 로 못 쓴다 → 손으로 짠 기계어.
//
// [못 고치는 것]
//   · `target_hp_ratio` — 예측 함수에 대상이 안 넘어오고 접근자도 없다(슬롯 7개 전수 확인).
//   · `cc_damage` — 같은 이유 + 기본·모드·워크샵 전 파일에서 값이 0건이라 고쳐도 변화 없음.
//
// ⚠기본 OFF. `fix_hp_ratio=1` 일 때만 적용된다.
// ⚠실제 피해량은 안 바뀐다(예측 전용 API) — **AI 판단만** 바뀐다. 그래서 리플레이 재현은 깨진다.
// ════════════════════════════════════════════════════════════════════════════════
const HR_AE_FN_RVA: usize = 0x155ffc0;   // AttackEffect·FixedAttackEffect 예측 leaf
const HR_AP_FN_RVA: usize = 0x15730b0;   // ApAttackEffect 예측 leaf
/// AttackEffect / FixedAttackEffect vtable 의 `+0x28` 슬롯이 놓인 .rdata RVA
const HR_AE_SLOTS: [usize; 10] = [
    0x31c7d48, 0x31dca98, 0x320b688, 0x32147d8, 0x3214db8,
    0x3228d10, 0x3234a40, 0x32485d0, 0x325d1f0, 0x326e060,
];
/// ApAttackEffect vtable 의 `+0x28` 슬롯이 놓인 .rdata RVA
const HR_AP_SLOTS: [usize; 8] = [
    0x31dc6d8, 0x31f0e40, 0x320b2b8, 0x3214ca0,
    0x322aa48, 0x323fe60, 0x325d8c8, 0x327a250,
];

static HR_APPLIED: AtomicI64 = AtomicI64::new(-1);   // -1=미결정 / 0=원본 / 1=수정
static HR_AE_STUB: AtomicUsize = AtomicUsize::new(0);   // 스텁은 1회만 할당해 재사용
static HR_AP_STUB: AtomicUsize = AtomicUsize::new(0);

/// `.rdata` 의 함수 포인터 한 칸을 덮어쓴다(RW 전환 → 쓰기 → 권한복구).
/// 지금 값이 `expect` 와 다르면 **아무것도 쓰지 않는다**(패치로 주소가 옮겨졌거나 남이 먼저 건드린 상태).
unsafe fn hr_write_slot(slot: usize, expect: usize, newv: usize) -> bool {
    if rd_u64(slot) != Some(expect as u64) { return false; }
    let mut old: u32 = 0;
    if VirtualProtect(slot, 8, 0x04, &mut old) == 0 { return false; }   // PAGE_READWRITE
    core::ptr::write_unaligned(slot as *mut u64, newv as u64);
    VirtualProtect(slot, 8, old, &mut old);
    true
}

/// 예측 스텁을 만든다. `ap=false` → AttackEffect(물리, 스탯 `[st+0]`, 결과 rax)
///                    `ap=true`  → ApAttackEffect(마법, 스탯 `[st+8]`, 결과 rdx)
///
/// 진입 계약: rcx=&Effect, rdx=미사용, r8=&Entity, r9=&접근자테이블.
/// `[r9+0x38]` 이 시전자 스탯블록 포인터를 준다 — `[st+0]`공격력 `[st+8]`주문력 `[st+0x10]`최대HP.
unsafe fn hr_build_stub(ap: bool) -> usize {
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = stub_reg(VirtualAlloc(0, 256, MEM_CR, RWX), 256, if ap { 0xF21 } else { 0xF20 });
    if stub == 0 { return 0; }
    let atk_off: u8 = if ap { 0x08 } else { 0x00 };       // 주문력 / 공격력
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x56]);                                     // push rsi
    s.extend_from_slice(&[0x48,0x83,0xec,0x20]);                      // sub rsp,0x20
    s.extend_from_slice(&[0x48,0x89,0xce]);                           // mov rsi,rcx      (self)
    s.extend_from_slice(&[0x4c,0x89,0xc1]);                           // mov rcx,r8       (entity)
    s.extend_from_slice(&[0x41,0xff,0x51,0x38]);                      // call [r9+0x38]  → rax=&stats
    s.extend_from_slice(&[0x49,0x89,0xc2]);                           // mov r10,rax      (스탯블록 보관)
    s.extend_from_slice(&[0x49,0xbb]);                                // movabs r11, ÷100 매직
    s.extend_from_slice(&0x28f5c28f5c28f5c3u64.to_le_bytes());
    // ── 항1: attack_ratio × (공격력|주문력) ÷ 100 ──
    s.extend_from_slice(&[0x48,0x8b,0x4e,0x18]);                      // mov rcx,[rsi+0x18]
    if ap { s.extend_from_slice(&[0x49,0x0f,0xaf,0x4a,atk_off]); }    // imul rcx,[r10+8]
    else  { s.extend_from_slice(&[0x49,0x0f,0xaf,0x0a]); }            // imul rcx,[r10]
    s.extend_from_slice(&[0x48,0xc1,0xe9,0x02]);                      // shr rcx,2
    s.extend_from_slice(&[0x48,0x89,0xc8]);                           // mov rax,rcx
    s.extend_from_slice(&[0x49,0xf7,0xe3]);                           // mul r11
    s.extend_from_slice(&[0x48,0xc1,0xea,0x02]);                      // shr rdx,2      = rcx/100
    s.extend_from_slice(&[0x49,0x89,0xd0]);                           // mov r8,rdx     (누산기)
    // ── 항2(추가): hp_ratio × 시전자 최대HP ÷ 100 ──  hp_ratio<=0 이면 통째로 건너뜀
    s.extend_from_slice(&[0x48,0x8b,0x4e,0x20]);                      // mov rcx,[rsi+0x20]  hp_ratio
    s.extend_from_slice(&[0x48,0x85,0xc9]);                           // test rcx,rcx
    s.extend_from_slice(&[0x7e,0x16]);                                // jle +22 (아래 6개 명령 건너뜀)
    s.extend_from_slice(&[0x49,0x0f,0xaf,0x4a,0x10]);                 // imul rcx,[r10+0x10] 최대HP
    s.extend_from_slice(&[0x48,0xc1,0xe9,0x02]);                      // shr rcx,2
    s.extend_from_slice(&[0x48,0x89,0xc8]);                           // mov rax,rcx
    s.extend_from_slice(&[0x49,0xf7,0xe3]);                           // mul r11
    s.extend_from_slice(&[0x48,0xc1,0xea,0x02]);                      // shr rdx,2
    s.extend_from_slice(&[0x49,0x01,0xd0]);                           // add r8,rdx
    // ── 결과 = 누산기 + damage, 반대편 워드는 0 ──
    if ap {
        s.extend_from_slice(&[0x4c,0x89,0xc2]);                       // mov rdx,r8
        s.extend_from_slice(&[0x48,0x03,0x56,0x10]);                  // add rdx,[rsi+0x10]  damage
        s.extend_from_slice(&[0x31,0xc0]);                            // xor eax,eax    (물리 0)
    } else {
        s.extend_from_slice(&[0x4c,0x89,0xc0]);                       // mov rax,r8
        s.extend_from_slice(&[0x48,0x03,0x46,0x10]);                  // add rax,[rsi+0x10]  damage
        s.extend_from_slice(&[0x31,0xd2]);                            // xor edx,edx    (마법 0)
    }
    s.extend_from_slice(&[0x48,0x83,0xc4,0x20]);                      // add rsp,0x20
    s.extend_from_slice(&[0x5e]);                                     // pop rsi
    s.extend_from_slice(&[0xc3]);                                     // ret
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());
    stub
}

/// `fix_hp_ratio` 토글 처리. 켜면 vtable 슬롯 18곳을 스텁으로, 끄면 원래 함수로 되돌린다.
unsafe fn apply_fix_hp_ratio() {
    let want = if tune("fix_hp_ratio", 0) != 0 { 1i64 } else { 0i64 };
    if HR_APPLIED.load(Ordering::Relaxed) == want { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let (ae_fn, ap_fn) = (base + HR_AE_FN_RVA, base + HR_AP_FN_RVA);

    if want == 0 {
        // ── 원본 복원 ── (한 번도 안 걸었으면 아무것도 안 한다)
        if HR_APPLIED.load(Ordering::Relaxed) == 1 {
            let (sae, sap) = (HR_AE_STUB.load(Ordering::Relaxed), HR_AP_STUB.load(Ordering::Relaxed));
            for &s in HR_AE_SLOTS.iter() { hr_write_slot(base + s, sae, ae_fn); }
            for &s in HR_AP_SLOTS.iter() { hr_write_slot(base + s, sap, ap_fn); }
        }
        HR_APPLIED.store(0, Ordering::Relaxed);
        if let Some(p) = pth("fix_hp_ratio.txt") {
            let _ = fs::write(p, format!("fix_hp_ratio=0 (원본) @base{:#x}\n", base));
        }
        return;
    }

    // ── 적용 전 18곳 전수 대조. 하나라도 다르면 **아무것도 하지 않는다.** ──
    let bad_ae = HR_AE_SLOTS.iter().filter(|&&s| rd_u64(base + s) != Some(ae_fn as u64)).count();
    let bad_ap = HR_AP_SLOTS.iter().filter(|&&s| rd_u64(base + s) != Some(ap_fn as u64)).count();
    if bad_ae != 0 || bad_ap != 0 {
        HR_APPLIED.store(0, Ordering::Relaxed);
        if let Some(p) = pth("fix_hp_ratio.txt") {
            let _ = fs::write(p, format!(
                "SKIP: vtable 슬롯 불일치 AE불일치={}/10 AP불일치={}/8 @base{:#x}\n\
                 (게임 패치로 주소가 옮겨졌거나 다른 모드가 먼저 건드린 상태 — 아무것도 쓰지 않았다)\n",
                bad_ae, bad_ap, base));
        }
        return;
    }

    // 스텁은 한 번만 만들어 재사용한다(토글을 껐다 켤 때마다 새로 잡으면 누수).
    let mut sae = HR_AE_STUB.load(Ordering::Relaxed);
    if sae == 0 { sae = hr_build_stub(false); HR_AE_STUB.store(sae, Ordering::Relaxed); }
    let mut sap = HR_AP_STUB.load(Ordering::Relaxed);
    if sap == 0 { sap = hr_build_stub(true);  HR_AP_STUB.store(sap, Ordering::Relaxed); }
    if sae == 0 || sap == 0 {
        HR_APPLIED.store(0, Ordering::Relaxed);
        if let Some(p) = pth("fix_hp_ratio.txt") { let _ = fs::write(p, "SKIP: 스텁 할당 실패\n"); }
        return;
    }

    let n_ae = HR_AE_SLOTS.iter().filter(|&&s| hr_write_slot(base + s, ae_fn, sae)).count();
    let n_ap = HR_AP_SLOTS.iter().filter(|&&s| hr_write_slot(base + s, ap_fn, sap)).count();
    HR_APPLIED.store(if n_ae == 10 && n_ap == 8 { 1 } else { 0 }, Ordering::Relaxed);
    if let Some(p) = pth("fix_hp_ratio.txt") {
        let _ = fs::write(p, format!(
            "fix_hp_ratio=1 (수정) 슬롯 {}/10 + {}/8 교체  stubAE={:#x} stubAP={:#x} @base{:#x}\n\
             체력 비례 스킬(hp_ratio)의 피해가 AI 예측에 반영됩니다.\n\
             ※실제 피해량은 안 바뀝니다 — AI의 스킬 가치평가·타깃 선택·위험 회피만 바뀝니다.\n\
             ※대상 체력 비례(target_hp_ratio)는 예측 함수에 대상이 안 넘어와서 못 고칩니다.\n",
            n_ae, n_ap, sae, sap, base));
    }
}
