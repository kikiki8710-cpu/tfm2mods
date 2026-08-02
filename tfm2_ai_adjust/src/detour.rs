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

#[inline] unsafe fn patch_imm_bytes(addr: usize, prefix: &[u8], imm_off: usize, width: usize, val: u64) -> bool {
    if !readable(addr, imm_off + width) { return false; }
    for (i, &b) in prefix.iter().enumerate() {
        if rd_u8(addr + i) != b { return false; }   // opcode 불일치 = RVA 어긋남 → skip(크래시 방지)
    }
    let site = addr + imm_off;
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
// ⛔★`an_count_gate` 영구 폐기(재핀 금지 · cfg `oi_an_count_gate` = DEAD): 구 사이트 `cmp qword[rbx+0x5b0],5`는
//   **배열 bounds-check 관용구**다(0.5.2에 동일 패턴 37곳, 예외없이 `cmp [X+0x5b0],N` → `lea 정적더미` → `cmovae 실원소` →
//   `cmp [reg+0x30],-1` 형태이며 imm=3/5가 항상 짝으로 등장 — 0.5.1에서 "짝구조 일치"를 강후보 근거로 삼았던 게 바로 이 지문이었다).
//   N을 바꾸면 없는 원소를 실포인터로 읽어 **OOB → 크래시/미정의**. 게다가 an 컨테이너(0x2376320) 안엔 이 패턴이 아예 없다.
// ⬜미해결(패치와 무관): 서브플랜 디스패처가 0.5.2에 최소 2개(`0x2134240` JT `0x38ae274` / `0x1dabcc0` JT `0x3842688`)이고
//   둘이 같은 idx에 다른 타깃을 준다 ⟹ **disc 번호 정본 재확인 필요**. 단 위 배선은 disc 번호가 아니라 **imm 지문 기반 함수 동정**이라 무영향.
unsafe fn apply_objective_imm() {
    // 게임 원본 imm값(0.5.0_2 실측): count 0x26 / nexus_hp 0x32 / hp_crit 0x15 / hp_low 0x1f
    //   / lane_margin 0x78 / an_gate 5 / near_dist 120000 / pred_dist 240000 (dist는 거리, 코드가 제곱).
    let enable = tune("oi_enable", 0) != 0;
    let cg = tune("oi_dn_count_gate", 0x26);
    let nh = tune("oi_dn_nexus_hp",   0x32);
    let hc = tune("oi_dn_hp_crit",    0x15);
    let hl = tune("oi_dn_hp_low",     0x1f);
    let lm = tune("oi_dn_lane_margin",0x78);
    let ag = tune("oi_an_count_gate",  5);
    let nd = tune("oi_dn_near_dist",  120000);
    let pd = tune("oi_dn_pred_dist",  240000);
    // ★[07-16 확증배선] disc18 핸들러 0x1c7ca20 내부 2사이트(ghidra-re DIFF 확정):
    //   fh = 적 넥서스 HP% 게이트(0x1c7df47 cmp rax,0x38=56). ≥이값이면 아군 무관 즉시 마무리오더, 미만이면 아군2+도달 필요.
    //   cd = 넥서스공격 후보 거리컬링(0x1c7d5f9 cmp r8,0x5f5e0). (dist²>>14)>이값이면 그 아군 후보 스킵.
    let fh = tune("oi_an_finish_hp",  0x38);      // 56(%)
    let cd = tune("oi_an_cull_dist",  0x5f5e0);   // 390624 (dist²>>14 스케일 임계, ≈넥서스 2.5셀 반경)
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
    ok += patch_imm_bytes(base + 0xdecd00, &[0x48,0x83,0xf8], 3, 1, b1(nh)) as u32;              // dn_nexus_hp #1 orig 0x32 (64bit div 경로)
    ok += patch_imm_bytes(base + 0xdecd0c, &[0x48,0x83,0xf8], 3, 1, b1(nh)) as u32;              // dn_nexus_hp #2 orig 0x32 (32bit div 경로) ★둘 다 필수
    ok += patch_imm_bytes(base + 0xdecd48, &[0x48,0x83,0x7d,0xd0], 4, 1, b1(hl)) as u32;         // dn_hp_low  orig 0x1f  (★0.5.3: 스택변위 [rbp-0x28]→**[rbp-0x30]** = prefix 마지막 d8→d0. 주소는 맞는데 이 prefix를 안 고쳐 8/12로 skip됐던 자리)
    ok += patch_imm_bytes(base + 0xdecd78, &[0x48,0x83,0x7d,0xd0], 4, 1, b1(hc)) as u32;         // dn_hp_crit orig 0x15  (★0.5.3: 변위 d8→d0, 위와 동일 사유. 0.5.2엔 1곳뿐=병합)
    ok += patch_imm_bytes(base + 0xdec8a1, &[0x48,0xb8], 2, 8, sq(nd)) as u32;                   // dn_near_dist #1 orig 0x35a4e9001 (=120000²+1)
    ok += patch_imm_bytes(base + 0xdec9cd, &[0x48,0xb8], 2, 8, sq(nd)) as u32;                   // dn_near_dist #2 orig 0x35a4e9001
    ok += patch_imm_bytes(base + 0xdecc38, &[0x49,0xba], 2, 8, sq0(nd)) as u32;                  // ★dn_near_dist #3 orig 0x35a4e9000 (**+1 없음** = sq0 / movabs r10 / 후보 루프 컬링)
    // ── dn 클러스터 B: 컨테이너 0x1bdaaa0 ──
    ok += patch_imm_bytes(base + 0xdf94a5, &[0x48,0xb8], 2, 8, sq(pd)) as u32;                   // dn_pred_dist  orig 0xd693a4001 (=240000²+1, exe 전역 유일)
    ok += patch_imm_bytes(base + 0xdf9513, &[0x49,0x83,0xc5], 3, 1, b1(lm)) as u32;              // dn_lane_margin orig 0x78 (★0.5.3: `add r14`→**`add r13`** = prefix c6→c5. 명령·의미 동일, 레지스터 배정만 바뀜)
    // ── an 클러스터: 컨테이너 0x2376320 (0.5.1 disc18 핸들러 0x1c7ca20 후계) ──
    // ★0.5.3 재구성(ghidra-re 07-29 + 실측): 0.5.2의 **단일 루프 1사이트**가 **3연속 루프 3사이트**로 분열했다.
    //   0.5.2 = 헬퍼(0x22c8a70)가 만든 Vec 1개를 1회 스캔 / 0.5.3 = [rbp+0x198]+idx*32 의 (ptr,len) **3쌍**을 인라인 체이닝,
    //   세 루프가 같은 본문(cmp byte[r12],0 → call 0xfcb660)으로 합류 = 논리적으로 같은 한 스캔 ⟹ **3사이트 전부 패치해야 커버리지 동일**.
    // ★★극성 반전: 0.5.2 `cmp r10,0x5f5e0; jbe(수락)` ⟺ 0.5.3 `cmp r8,0x5f5e1; jae(스킵)` — 같은 판정을 반대로 인코딩.
    //   ⟹ 임계값에 **+1** 을 실어야 의미가 같다(원본 복원값 0x5f5e0 → write 0x5f5e1 로 자동 정합).
    //   prefix 도 `49 81 fa`(cmp r10) → **`49 81 f8`(cmp r8)** 로 바뀌었다.
    let cd1 = u32c(cd).saturating_add(1);
    ok += patch_imm_bytes(base + 0xd95603, &[0x49,0x81,0xf8], 3, 4, cd1) as u32;                  // an_cull_dist #1 (리스트A)
    ok += patch_imm_bytes(base + 0xd95693, &[0x49,0x81,0xf8], 3, 4, cd1) as u32;                  // an_cull_dist #2 (리스트B)
    ok += patch_imm_bytes(base + 0xd95717, &[0x49,0x81,0xf8], 3, 4, cd1) as u32;                  // an_cull_dist #3 (리스트C)
    ok += patch_imm_bytes(base + 0xd960e8, &[0x48,0x83,0xf8], 3, 1, b1(fh)) as u32;              // an_finish_hp #1 orig 0x38 (64bit div 경로)
    ok += patch_imm_bytes(base + 0xd960f4, &[0x48,0x83,0xf8], 3, 1, b1(fh)) as u32;              // an_finish_hp #2 orig 0x38 (32bit div 경로) ★0.5.1은 #1만 패치=결함이었음
    OBJIMM_SIG.store(sig, Ordering::Relaxed);
    // ★LOG_ON 무관 직접 write(설치확증 — d19_imm.txt·itemnet_guard와 동일). write_named은 LOG_ON 게이트라 프로덕션서 미확인됐음.
    if let Some(p) = pth("obj_imm.txt") {
        let _ = fs::write(p, format!("oi_enable={} applied={}/14 cg={}=DEAD nh={} hc={} hl={} lm={} an={}=DEAD near={} pred={} fh={} cull={} @base{:#x}\n",
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
    let ok = patch_imm_bytes(base + 0xc43083, &[0x48,0x81,0xc6], 3, 4, v);   // add rsi, imm32
    VISIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(p) = pth("vis_imm.txt") {
        let _ = fs::write(p, format!("vis_window={} applied={}/1 @0xc43083(0.5.3) @base{:#x}\n", vw, ok as u32, base));
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
    ok += patch_imm_bytes(base + 0xe06e22, &[0x48,0xc7,0x44,0x24,0x40], 5, 4, e_cr) as u32;                  // 근접반경²  orig 0x249f0
    ok += patch_imm_bytes(base + 0xe07610, &[0x48,0xc7,0x85,0x70,0x02,0x00,0x00], 7, 4, e_lr) as u32;        // 라인range² orig 0x3d090 (★0.5.3: rbp 변위 0x1b0→**0x270** = prefix 4~7B 교체. 0.5.1→0.5.2땐 0x180→0x1b0였음)
    ok += patch_imm_bytes(base + 0xe075ca, &[0xb8], 1, 4, e_jd) as u32;                                 // 합류max거리²(지배) orig 0xd693a401 (★0.5.3: `41 b8`(mov r8d)→**`b8`(mov eax)** 로 인코딩 축소 ⟹ 사이트가 +1(0xe075c9→**0xe075ca**), prefix 1B, off 2→1. 뒤 비교도 cmp r9,r8→cmp r8,rax 로 대응. 구 인코딩 mov r8d로 변경)
    ok += patch_imm_bytes(base + 0xe0a328, &[0x48,0x83,0xf8], 3, 1, e_ph) as u32;                            // 라인압박 HP%<30
    // ── 거점헬퍼 0x2398240: op·scout ──
    ok += patch_imm_bytes(base + 0xcc3bbd, &[0x48,0x83,0xb9,0xb8,0x00,0x00,0x00], 7, 1, e_op) as u32;   // 운영진입 phase>30 (★0.5.3: 컨테이너 0x2398240→**0xcc3960**, `[r14+0xb8]`→**`[rcx+0xb8]`** = prefix 49 83 be→48 83 b9)
    // ★0.5.3: 0.5.2 는 같은 5슬롯 루프의 임계값을 **프리헤더+latch 2곳**에 호이스트했었는데(그래서 2사이트),
    //   0.5.3 은 호이스트 없이 **루프 본문 1곳**만 둔다 ⟹ 2사이트 → **1사이트 병합**. `movabs r9`→**`movabs rax`**.
    // ★★극성 반전: 0.5.2 `cmp rdx,r9; jae(스킵)`(임계=d²+1) ⟺ 0.5.3 `cmp rdx,rax; ja(스킵)`(임계=d²)
    //   ⟹ 인코딩을 **sq1(d²+1) → sqd(d²)** 로 바꿔야 한다. 여기서 sq1 을 쓰면 반경이 1 어긋난다.
    // ⚠같은 함수의 `movabs r13, 0x53d1ac101`(=150000²+1 @0xcc4399)은 **다른 반경**이고 원래도 미패치 — 값만 보고 잡지 말 것.
    let e_sr2 = if on(sr) { sqd(sr) } else { 0x35a4e9000 };   // ⚠sq1 아님(극성 반전)
    ok += patch_imm_bytes(base + 0xcc40e6, &[0x48,0xb8], 2, 8, e_sr2) as u32;   // 거점반경² (0.5.2 #1+#2 통합)
    // ⛔합류 phase≥12 2사이트(구 0x1e1f4ea/0x1e1fa74) = 0.5.2 게이트 삭제 → 제거(상단 주석)
    // ── reach (전역공유 ⚠): 0x23ad980 / 0x23ba8d0 ──
    ok += patch_imm_bytes(base + 0xcdd067, &[0x48,0xb8], 2, 8, e_rc) as u32;                                 // reach cap² #1(≤)
    ok += patch_imm_bytes(base + 0xcdfeed, &[0x49,0xba], 2, 8, e_rc.wrapping_add(1)) as u32;                 // reach cap² #2(<, +1경계)
    ok += patch_imm_bytes(base + 0xe08858, &[0x41,0xb8], 2, 4, e_rm) as u32;                                 // reach margin
    GBIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(p) = pth("gb_imm.txt") {
        let _ = fs::write(p, format!("gb_enable={} applied={}/9 close={} line={} join={} scout={} op_ph={} join_ph={}=DEAD push_hp={} reach_cap={} reach_mgn={} @base{:#x}\n",
            enable, ok, cr, lr, jd, sr, op, jp, ph, rc, rm, base));
    }
}

// ★★[07-23 신설] 공유 위협 severity 사다리 byte-patch — **전 서브플랜 공통의 "이 위협이 유의미한가" 필터**.
//   ghidra-re 규명(0.5.2): disc19 인라인 사다리(d19_sev_* 소관)와 **같은 모양의 사본이 exe에 5곳 더** 있고, 각각:
//     [A] `0x22dd9a0` = **위협 평가 정본 본체**(TLS memo 래퍼 `0x22dd690` 경유·전 핸들러 ~60콜사이트가 공유) — 사다리 7 + **할인율 레버**
//         ('사소' 판정 위협은 `min(cap, threat>>shift)`로 축소 누산; 원본 shift=2(1/4)·cap=0x12=18).
//     [B] `0x22e6460` = 드라이버B(JT2 `0x1dabcc0` 계열 = 넥서스 공방 포함) 디스패치 직전 공통 위협 컨텍스트 빌더 — 축약 사다리 5.
//     [C] `0x22efed0` = 위협 유의성 필터 leaf(disc0/1/3·disc4 경로) — 사다리 7 (+branch A 별도 4임계 = ⬜미배선·후보).
//     [E] `0x23a04d0` = 공유 후보-스코어링 평가자(JT2 다수 래퍼 15종 경유) — 사다리 7(tr3만 jb 인코딩 = **+1**).
//     [D] `0x22f8a90`(disc5/6 후퇴판정 leaf) = ⬜**미배선**(disc5/6 라이브 발화 미확정 + 트레일러 매핑 신뢰도 중).
//   ★의미: `tr = threat*100/hp_cur` 사다리 — **tr 임계↓ = 더 겁쟁이(위협을 더 심각하게), ↑ = 더 대담**. hp 경계 = 단계 전환점.
//   ★설계: 4사본 26사이트를 **같은 값으로 일괄 패치**(사본별 개별화 금지 — 판단 일관성. disc19 사본만 d19_sev_*로 별도인 것은
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
    let mut sig = enable as u64;
    for v in [tr0, tr1, tr2, tr3, hp1, hp2, hp3, dsh, dcp] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == SEVIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64| (v.max(0).min(0x7f)) as u64;
    let (p_t0, p_t1, p_t2, p_t3, p_h1, p_h2, p_h3, p_ds, p_dc) = if enable {
        (b1(tr0), b1(tr1), b1(tr2), b1(tr3), b1(hp1), b1(hp2), b1(hp3), b1(dsh.min(63)), b1(dcp))
    } else {
        (0x31, 0x1d, 0x11, 0x09, 0x41, 0x28, 0x19, 0x02, 0x12)
    };
    let mut ok = 0u32;
    // ── [A] 위협 평가 정본 본체 0x22dd9a0 (사다리 7 + 할인 3) ──
    ok += patch_imm_bytes(base + 0xcd103f, &[0x48,0x83,0xf8], 3, 1, p_t0) as u32;   // tr>49
    ok += patch_imm_bytes(base + 0xcd1050, &[0x48,0x83,0xf9], 3, 1, p_h1) as u32;   // hp%>65 (rcx)
    ok += patch_imm_bytes(base + 0xcd1056, &[0x48,0x83,0xf8], 3, 1, p_t1) as u32;   // tr>29
    ok += patch_imm_bytes(base + 0xcd1060, &[0x48,0x83,0xf9], 3, 1, p_h2) as u32;   // hp%>40
    ok += patch_imm_bytes(base + 0xcd1066, &[0x48,0x83,0xf8], 3, 1, p_t2) as u32;   // tr>17
    ok += patch_imm_bytes(base + 0xcd1070, &[0x48,0x83,0xf9], 3, 1, p_h3) as u32;   // hp%>25
    ok += patch_imm_bytes(base + 0xcd1076, &[0x48,0x83,0xf8], 3, 1, p_t3) as u32;   // tr>9
    ok += patch_imm_bytes(base + 0xcd108b, &[0x48,0xc1,0xf8], 3, 1, p_ds) as u32;   // 할인 shift (sar rax,imm8)
    ok += patch_imm_bytes(base + 0xcd108f, &[0x48,0x83,0xf8], 3, 1, p_dc) as u32;   // 할인 cap 비교
    ok += patch_imm_bytes(base + 0xcd1093, &[0xbb], 1, 4, p_dc) as u32;             // 할인 cap 값 (mov ebx,imm32)
    // ── [B] 드라이버B 공통 위협 빌더 0x22e6460 (축약 5) ──
    ok += patch_imm_bytes(base + 0xd1af8f, &[0x48,0x83,0xf8], 3, 1, p_t0) as u32;   // tr>49
    ok += patch_imm_bytes(base + 0xd1af95, &[0x49,0x83,0xf8], 3, 1, p_h1) as u32;   // hp%>65 (r8)
    ok += patch_imm_bytes(base + 0xd1af9b, &[0x48,0x83,0xf8], 3, 1, p_t1) as u32;   // tr>29
    ok += patch_imm_bytes(base + 0xd1afa1, &[0x49,0x83,0xf8], 3, 1, p_h2) as u32;   // hp%>40
    ok += patch_imm_bytes(base + 0xd1afa7, &[0x48,0x83,0xf8], 3, 1, p_t2) as u32;   // tr>17
    // ── [C] 위협 유의성 필터 leaf 0x22efed0 branch B (7) ──
    ok += patch_imm_bytes(base + 0xcd4c6f, &[0x48,0x83,0xf8], 3, 1, p_t0) as u32;   // tr>49
    ok += patch_imm_bytes(base + 0xcd4c75, &[0x48,0x83,0xf9], 3, 1, p_h1) as u32;   // hp%>65 (rcx)
    ok += patch_imm_bytes(base + 0xcd4c7b, &[0x48,0x83,0xf8], 3, 1, p_t1) as u32;   // tr>29
    ok += patch_imm_bytes(base + 0xcd4c81, &[0x48,0x83,0xf9], 3, 1, p_h2) as u32;   // hp%>40
    ok += patch_imm_bytes(base + 0xcd4c87, &[0x48,0x83,0xf8], 3, 1, p_t2) as u32;   // tr>17
    ok += patch_imm_bytes(base + 0xcd4c8d, &[0x48,0x83,0xf9], 3, 1, p_h3) as u32;   // hp%>25
    ok += patch_imm_bytes(base + 0xcd4c93, &[0x48,0x83,0xf8], 3, 1, p_t3) as u32;   // tr>9
    // ── [E] 공유 후보-스코어링 평가자 0x23a04d0 (7, tr3만 +1 인코딩) ──
    ok += patch_imm_bytes(base + 0xc82224, &[0x48,0x83,0xf8], 3, 1, p_t0) as u32;   // tr>49
    ok += patch_imm_bytes(base + 0xc8222a, &[0x49,0x83,0xf8], 3, 1, p_h1) as u32;   // hp%>65 (r8)
    ok += patch_imm_bytes(base + 0xc82230, &[0x48,0x83,0xf8], 3, 1, p_t1) as u32;   // tr>29
    ok += patch_imm_bytes(base + 0xc82236, &[0x49,0x83,0xf8], 3, 1, p_h2) as u32;   // hp%>40
    ok += patch_imm_bytes(base + 0xc8223c, &[0x48,0x83,0xf8], 3, 1, p_t2) as u32;   // tr>17
    ok += patch_imm_bytes(base + 0xc82244, &[0x49,0x83,0xf8], 3, 1, p_h3) as u32;   // hp%>25
    ok += patch_imm_bytes(base + 0xc8224a, &[0x48,0x83,0xf8], 3, 1, p_t3 + 1) as u32; // tr>=10 (jb = tr3+1 인코딩)
    SEVIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(p) = pth("sev_imm.txt") {
        let _ = fs::write(p, format!("sv_enable={} applied={}/29 tr=[{} {} {} {}] hp=[{} {} {}] discount=shift{} cap{} @base{:#x}\n",
            enable, ok, tr0, tr1, tr2, tr3, hp1, hp2, hp3, dsh, dcp, base));
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
unsafe fn apply_numbers_sp(disc: i64, entry_rsp: usize, p1: usize) {
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
        let side = rd_i64(r14 + 0x820).unwrap_or(-1);
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
