// =====================================================================
//  CAND_FILTER 화이트박스 재현  (transition_engine RNG 후보필터)
//  원본: FUN_141f4ec60 @ RVA 0x1f4ec60 (3차 핫픽스, size 0x4dd)
//  transition(0x1b64db0)에서 5회 호출. 14-필드 컨텍스트를 받아 5개 레인
//  후보를 가중 RNG로 필터링하여 출력 Vec(상대팀 후보 포인터)을 만든다.
//
//  ⚠ 이 모듈은 아직 라이브 결정경로에 배선되지 않음(검증 전 참조 재현).
//    검증법: tecap 캡처훅으로 14-필드 컨텍스트 + 입력 RNG state를 덤프 →
//    cand_filter_repro() 오프라인 실행 → 게임 출력 Vec과 DIFF=0 비교.
//
//  의존(plan_reimpl.rs 헬퍼): rd_u64/rd_i64/rd_u32/rd_u8, rng_gen_range, isqrt_u64
//  ★3개 trait getter = dd7700 vtable 슬롯 = 이미 화이트박스(게임호출 제로):
//    vtbl+0x20 → dd7_slot20(sim)        = rd_i64(sim+0xed00)  타이밍 스칼라
//    vtbl+0x48 → dd7_slot48(sim,sub,id) = char 술어(true ⟺ 게임 AL!=0)
//    vtbl+0xa8 → dd7_slot_a8(sim,id)    = id→record|0
//  champ(=rhd[0]) = simulation_state (transition 진입부 vt128(champ,h) 호출로 확인).
// =====================================================================

/// transition이 스택 [rbp+0x540]에 빌드하는 14-필드 컨텍스트 (전부 raw 주소/값).
#[derive(Clone, Copy)]
pub struct CandCtx {
    pub rhd: usize,        // [0]  rh.ptr.  champ=*rhd, vtbl=*(rhd+8), 후보테이블=rhd+0x1e0
    pub team_arr: usize,   // [1]  rh[+0x10]. 상대팀 athlete 배열 base
    pub athlete: usize,    // [2]  A.  team = *(A+0x6a8)
    pub rng: usize,        // [3]  &rng (PRNG state)
    pub lo: i64,           // [4]  roll range 하한(*[4])  ※ ctx엔 &lo지만 값으로 보관
    pub hi: i64,           // [5]  roll range 상한
    pub s_sub: usize,      // [6]  S+0xe8.  +0x218=레인좌표, +0x290/+0x268=stat배열
    pub anchor_x: i64,     // [7]  거리 기준 X(*[7])
    pub anchor_y: i64,     // [8]  거리 기준 Y(*[8])
    pub dist_thr: i64,     // [9]  거리 임계(*[9], 통상 150000)
    pub lane_start: u64,   // [0xa]
    pub lane_end: u64,     // [0xb]
    pub out_base: usize,   // [0xc] 출력후보배열 base = (*[0xc])+0x1e0 (=rhd+0x1e0)
    pub athlete2: usize,   // [0xd] A (team2 = 1 - *(athlete2+0x6a8))
}

/// 컨텍스트를 transition 스택에서 직접 로드 (rdx = ctx_ptr = &[rbp+0x540]).
/// 캡처훅에서 ctx_ptr만 잡으면 이걸로 복원해 재현 실행.
pub unsafe fn load_ctx(ctx_ptr: usize) -> Option<CandCtx> {
    let f = |i: usize| rd_u64(ctx_ptr + i * 8).map(|v| v as usize);
    Some(CandCtx {
        rhd:       f(0)?,
        team_arr:  f(1)?,
        athlete:   f(2)?,
        rng:       f(3)?,
        lo:        rd_i64(f(4)?)?,        // [4]=&lo
        hi:        rd_i64(f(5)?)?,        // [5]=&hi
        s_sub:     f(6)?,
        anchor_x:  rd_i64(f(7)?)?,        // [7]=&anchorX
        anchor_y:  rd_i64(f(8)?)?,        // [8]=&anchorY
        dist_thr:  rd_i64(f(9)?)?,        // [9]=&distThr
        lane_start: rd_u64(ctx_ptr + 0xa * 8)?,
        lane_end:   rd_u64(ctx_ptr + 0xb * 8)?,
        out_base:  f(0xc)? + 0x1e0,
        athlete2:  f(0xd)?,
    })
}

/// CAND_FILTER 본체 재현. 선택된 (상대팀)후보 포인터들을 순서대로 반환.
/// draws_out: 소비한 RNG draw 수(레인당 1 roll, cand!=0일 때만).
/// getter/술어/룩업 전부 dd7_slot* 화이트박스 호출 — 게임함수 호출 없음.
pub unsafe fn cand_filter_repro(ctx: &CandCtx, draws_out: &mut u32) -> Option<Vec<usize>> {
    let sim = rd_u64(ctx.rhd)? as usize;            // champ=rhd[0]=simulation_state

    let team_raw = rd_i64(ctx.athlete + 0x6a8)?;    // 0 또는 1
    if (team_raw as u64) > 1 { return None; }       // 게임은 panic; 재현은 None
    let team = (1 - team_raw) as usize;             // 후보테이블 인덱스 = 상대팀
    let candtable = ctx.rhd + 0x1e0;
    let pos_base  = ctx.s_sub + 0x218;              // 레인좌표[(x,y) i64; 5], stride 0x10
    let stat290   = ctx.s_sub + 0x290;
    let stat268   = ctx.s_sub + 0x268;

    // ★단일 RngSim: 레인마다 fcdaf0 1회 호출이 buf/idx/refills를 누적 진행해야 함.
    //   (rng_gen_range는 매 호출 idx 원점 재스냅샷 → 전 레인 동일 draw 버그. RngSim은 상태유지.)
    let mut rng_sim = RngSim::new(ctx.rng)?;
    let mut out: Vec<usize> = Vec::new();
    let vis_window = tune("vis_window", 600);   // ★호이스트: 레인 루프 불변(시야 기억창 틱)
    let mut lane = ctx.lane_start;
    while lane < ctx.lane_end {
        if lane >= 5 { return None; }               // 게임 bounds panic
        let l = lane as usize;
        let cand = rd_u64(candtable + (team * 5 + l) * 8)? as usize;
        let mut sel: usize = 0;
        if cand != 0 {
            let cand628 = rd_i64(cand + 0x628)?;
            // ── RNG roll: fcdaf0(rng, lo..=hi) = inclusive gen_range (비-null 후보마다 1회, 게이트 전) ──
            *draws_out += 1;
            let roll = rng_sim.gen_range(ctx.lo as u64, ctx.hi as u64)? as i64;
            let gv = dd7_slot20(sim);               // vt+0x20 타이밍 스칼라
            // weight = roll*cand628/1000  (원본: >>3, mulhi(0x20c4..), >>4 = /1000)
            let weight = (roll * cand628) / 1000;
            let st290 = rd_i64(stat290 + l * 8)?;
            let score_a = if st290 <= gv { gv - st290 } else { 0 };
            // 거리² → isqrt
            let bx = rd_i64(pos_base + l * 0x10)?;
            let by = rd_i64(pos_base + 8 + l * 0x10)?;
            let dx = (bx - ctx.anchor_x).unsigned_abs();
            let dy = (by - ctx.anchor_y).unsigned_abs();
            let dist = isqrt_u64(dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))) as i64;
            let dist_excess = if ctx.dist_thr <= dist { dist - ctx.dist_thr } else { 0 };
            // ── 게이트 ──
            let cand5a8 = rd_u64(cand + 0x5a8)?;
            let mut accept = false;
            if st290 <= rd_i64(stat268 + l * 8)? {
                accept = true;                                  // 숏컷
            } else {
                let r = dd7_slot_a8(sim, cand5a8);              // vt+0xa8 룩업
                if r == 0 {
                    // vt+0x48 char!=0 ⟺ dd7_slot48==true
                    if dd7_slot48(sim, team_raw as usize, cand5a8) { accept = true; }
                } else {
                    let opp = (1 - team_raw) as usize;          // team_arr 인덱스
                    let lane_idx = rd_u32(r + 0x738) as usize;
                    let laneval = rd_i64(
                        opp * 0x228 + ctx.team_arr + 0x1e0 + lane_idx * 8,
                    )?;
                    if gv <= laneval + vis_window { accept = true; }   // ★시야: 현재틱 ≤ 마지막본틱 + 기억창(틱). gv=dd7_slot20(sim)(L89) 재사용 + vis_window 호이스트(기본600≈10초). CAND_FILTER→교전/운영/합류 전반
                }
            }
            // ACCEPT 경로: 게임 vt48 char==0 ⟺ !dd7_slot48
            if accept
                && !dd7_slot48(sim, team_raw as usize, cand5a8)
                && dist_excess <= score_a * weight
            {
                let team2_raw = rd_i64(ctx.athlete2 + 0x6a8)?;
                if (team2_raw as u64) > 1 { return None; }
                let team2 = (1 - team2_raw) as usize;
                sel = rd_u64(ctx.out_base + team2 * 0x28 + l * 8)? as usize;
            }
        }
        if sel != 0 { out.push(sel); }
        lane += 1;
    }
    Some(out)
}

/// transition 출력 조립의 분류 게이트(0x1b6b9f0~).
/// generic_build이 만든 0x90B 액션의 kind에 따라 commit할지 drop할지 결정.
/// kind ∈ {3,4,7} (mask 0x98) → drop(전환안함). 그 외 → commit(memcpy 0x90 → dest).
/// = 출력 조립의 유일한 "결정 knob". 액션 내용 자체는 generic_build(0x1bf5980, 23KB, deferred)이 생성.
#[inline]
pub fn trans_should_commit(action_kind: u64) -> bool {
    !(action_kind <= 7 && (0x98u64 >> action_kind) & 1 != 0)   // {3,4,7} → false(drop)
}
