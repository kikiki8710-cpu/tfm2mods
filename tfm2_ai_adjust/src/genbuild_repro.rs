// =====================================================================
//  ⛔⛔ [0.5.7 휴면·DORMANT — 재활성 전 전면 재작성 필수] ⛔⛔
//    이 재현물(my_generic_build/gb_region_d/my_gb_mainbuild/my_f80320)은 구세대(0.4.13~0.5.0)
//    함수(FUN_1420def90/1420dedc0/141f80320)를 겨냥. 0.5.7 실 함수 = FUN_140ceb5f0(GENERIC_BUILD).
//    rva_057.rs가 MIG_GB_CHANGED=true로 훅 SKIP(차단) = 현재 게임에 영향 없음(휴면).
//    ★0.5.7 행단위 RE(GENERIC_BUILD P1/P2)가 뒤집은 3대 정정 전부 미반영:
//      ① 상수 8 = tag 아닌 bumpalo 빈-Vec sentinel(산출=BumpVec<*Entity>)
//      ② 이동점 = team_plan.rs FUN_140c8bb60 splitmix64 결정론(RNG-free) — 이 재현은 RngSim gen_range 소비(정반대)
//      ③ 후보선택 = fight_check FUN_140cc4c50 argmin(승률예측) — 이 재현은 dist²최소(다른 아키텍처)
//    ⟹ 재활성하려면 0.5.7 RE 기준 전면 재작성. LIVE 개입은 apply_gb_imm(별개, 배선됨)만.
// =====================================================================
//  generic_build 옵션 스코어러 화이트박스 재현 (survivability score)
//  원본: FUN_141f80320 @ RVA 0x1f80320 (3차 핫픽스, 3844B)
//  generic_build(0x1bf5980)이 옵션 평가에 사용. 단일 ulonglong 점수 반환.
//
//  전략(1차): 오케스트레이션 + RNG draw 순서 + 능력테이블 + 산술만 재현.
//    술어(dec1f0/dfd1e0/dec4d0/dfb1a0)·f5db30·skill게터·ally풀·e1b330은
//    게임 포인터로 호출(oracle) → RNG draw 정렬(최난점)부터 검증.
//  검증: f80320_capture(entry RNG스냅샷+예측 / 리턴 score+draw수 대조, kind11).
//
//  ★RNG draw 소스: list1(후보당 base+슬롯0~3+type3부재0~1+type5 0~1), list2(후보당 2).
//    적/아군 거리루프는 f5db30 직접(roll 없음).
// =====================================================================

const F5_STUB: bool = false;   // ★재활성(2026-06-18): 크래시=환경(업데이트)확정. base getter RVA 마이그완료(0x18b9050/0x1bc6f10). e1b330이 my_f5db30 사용하므로 정확도 위해 실값.

#[derive(Clone, Copy)]
pub struct F80Ctx {
    pub p1: u64,     // count/index (local_c8)
    pub rng: usize,  // param_2 = &rng
    pub p3: usize,   // param_3 (ptr; *p3 = rhd)
    pub athlete: usize, // param_4 (+0x380, +0x218)
    pub p5: usize,   // param_5 (self entity)
    pub p6: usize,   // param_6 (list1 desc: [0]=begin, [3]=len)
    pub p7: usize,   // param_7 (list2 desc)
}

/// 자명한 vtable 게터 에뮬레이트 (게임호출 없이 pure read → 크래시 불가).
/// getter_ptr=함수주소, self_ptr=rcx 인자. 미지원 패턴=None.
/// 프로브 실측: vt+0x90=`mov rax,[rcx+disp]`(11변종) 또는 `xor eax,eax`; vt+0xa8=`mov eax,1`.
pub unsafe fn emulate_getter(getter_ptr: usize, self_ptr: usize) -> Option<i64> {
    if !readable(getter_ptr, 8) { return None; }
    let b0 = rd_u8(getter_ptr); let b1 = rd_u8(getter_ptr + 1); let b2 = rd_u8(getter_ptr + 2);
    // ⚠ret-check 없는 loose 버전(391/9 실용최적). 엄격ret-check는 비순수 ready함수를 false로 떨궈 오히려 악화(363/37).
    //   loose가 비순수 ready함수의 첫 mov로 ready-필드를 읽어 우연 일치(0x57cdc0=return0는 정확). 남은 9=재귀 ready(0x18b9490/0x1a7e440, vt+0x48/0x50/0x58 다형) 미재현.
    // 48 8b 41 d8 : mov rax,[rcx+disp8]
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x41 {
        let disp = rd_u8(getter_ptr + 3) as i8 as i64;
        return rd_i64((self_ptr as i64 + disp) as usize);
    }
    // 48 8b 81 d32 : mov rax,[rcx+disp32]
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x81 {
        let disp = rd_i32(getter_ptr + 3)? as i64;
        return rd_i64((self_ptr as i64 + disp) as usize);
    }
    // 48 8b 01 : mov rax,[rcx]
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x01 { return rd_i64(self_ptr); }
    // b8 imm32 : mov eax,imm
    if b0 == 0xb8 { return Some(rd_u32(getter_ptr + 1) as i64); }
    // 31 c0 / 33 c0 : xor eax,eax → 0
    if (b0 == 0x31 || b0 == 0x33) && b1 == 0xc0 { return Some(0); }
    None
}

// ★능력 base-power 재귀 walker: 0x1937ca0=서브능력 vt+0x28 합산(재귀), 0x1c4c9b0=terminal(flat[0x10]+ratio[0x18]*stat0/100).
//   stat0 = *(atk+0x600)(공격자 1차 유효스탯). g=base게터 주소, p1=능력데이터, atk=공격자엔티티.
unsafe fn base_power(g: usize, p1: usize, atk: usize, exe: usize, depth: u32) -> Option<i64> {
    if depth > 8 { return Some(0); }
    if g == exe + 0x190e740 {   // 0.4.13_5(was 0x1937ca0) mask-sig 유일
        let cnt = rd_u64(p1 + 0x28)?;
        if cnt > 64 { return Some(0); }
        let lb = rd_u64(p1 + 0x20)? as usize;     // 서브리스트 base (plVar3=lb+8)
        let mut sum = 0i64;
        for i in 0..cnt {
            let e = lb + (i as usize) * 0x18;
            let data = rd_u64(e)? as usize;        // plVar3[-1]
            let svt = rd_u64(e + 8)? as usize;     // *plVar3
            let sg = rd_u64(svt + 0x28)? as usize;
            let sarg = ((rd_u64(svt + 0x10)?.wrapping_sub(1) & !0xf) as usize).wrapping_add(data).wrapping_add(0x10);
            sum = sum.wrapping_add(base_power(sg, sarg, atk, exe, depth + 1)?);
        }
        Some(sum)
    } else if g == exe + 0x1bc6f10 {   // 0.4.13_5(was 0x1c4c9b0) mask-sig 유일
        Some(rd_i64(p1 + 0x10)? + (rd_i64(p1 + 0x18)? * rd_i64(atk + 0x600)?) / 100)
    } else {
        Some(emulate_getter(g, p1).unwrap_or(0))   // 미지 terminal
    }
}

// ★f5db30 재현(0x1f5db30, 능력데미지 2-컴포넌트): comp1 base=base_power(rax), comp2 base=base게터 rdx(=0, 0x1c4c9b0이 xor edx,edx).
//   저항=combat-pipeline(시트=atk+0x358, tgt+0x610/618/620). ⚠디컴파일의 "param_2"는 실제로 base게터 rdx반환(=0)이지 incoming arg 아님.
unsafe fn my_f5db30(p1: usize, atk: usize, p5_tgt: usize, exe: usize) -> Option<i64> {
    if F5_STUB { return Some(1000); }   // ★BISECT 스텁
    let p1vt = rd_u64(p1 + 8)? as usize;
    let bg = rd_u64(p1vt + 0x28)? as usize;
    let aligned = ((rd_u64(p1vt + 0x10)?.wrapping_sub(1) & !0xf) as usize).wrapping_add(rd_u64(p1)? as usize).wrapping_add(0x10);
    let mut lvar4 = base_power(bg, aligned, atk, exe, 0)?;
    let p2: i64 = 0;   // base게터 rdx = 0
    if lvar4 == 0 && p2 == 0 { return Some(0); }
    let dtype = rd_u32(p1 + 0x2c);
    let sheet = atk + 0x358;
    let s = |o: usize| rd_i64(sheet + o).unwrap_or(0);
    let l610 = rd_i64(p5_tgt + 0x628)?;   // 마법저항용 스탯
    let l618 = rd_i64(p5_tgt + 0x618)?;   // comp1 방어스탯
    let local78 = rd_i64(atk + 0x628)?;   // 공격자 스탯(0xd8 계수용)
    // component1 (base=lvar4)
    let uv10: i64 = if dtype.wrapping_sub(2) < 2 { ((s(0xf0) + 100) * lvar4) / 100 }
        else {
            let amp = dtype != 0 && (dtype & 6) == 2;
            if dtype == 0 { lvar4 += s(0xd0) * l610 / 100; } else { lvar4 += s(0xe0) * l610 / 100; }
            if amp { ((s(0xf0) + 100) * lvar4) / 100 } else { s(0xd8) * local78 / 100 + lvar4 }
        };
    let r1 = if (s(0xa8) as u64) < 0x65 { 100 - s(0xa8) } else { 0 };
    let d1 = (r1 * l618 / 100 + 100) as u64;
    let c1 = if d1 != 0 { (uv10 as u64).wrapping_mul(100) / d1 } else { 0 };
    // component2 (base=p2)
    let l620 = rd_i64(p5_tgt + 0x620)?;
    let mut p2v = p2;
    let uv8: i64 = if dtype.wrapping_sub(2) < 2 { ((s(0xf0) + 100) * p2v) / 100 }
        else {
            let amp = dtype != 0 && (dtype & 6) == 2;
            if dtype == 0 { p2v += s(0xd0) * l610 / 100; } else { p2v += s(0xe0) * l610 / 100; }
            if amp { ((s(0xf0) + 100) * p2v) / 100 } else { s(0xd8) * local78 / 100 + p2v }
        };
    let r2 = if (s(0xb0) as u64) < 0x65 { 100 - s(0xb0) } else { 0 };
    let d2 = (r2 * l620 / 100 + 100) as u64;
    let c2 = if d2 != 0 { (uv8 as u64).wrapping_mul(100) / d2 } else { 0 };
    Some((c1 + (c1 == 0) as u64 + c2 + (c2 == 0) as u64) as i64)
}

/// elem의 skill 슬롯(vtoff=vtable, dataoff=data) 게터(slot=0x90/0xa8) 에뮬레이트.
#[inline] pub unsafe fn skill_getter(elem: usize, vtoff: usize, dataoff: usize, slot: usize) -> Option<i64> {
    let vt = rd_u64(elem + vtoff)? as usize;
    let gptr = rd_u64(vt + slot)? as usize;
    let data = rd_u64(elem + dataoff)? as usize;
    emulate_getter(gptr, data)
}

// 능력 리스트(elem+0x2b0, count elem+0x2b8, stride 0x28)
#[inline] unsafe fn ab_list(elem: usize) -> (usize, usize) {
    (rd_u64(elem + 0x2b0).unwrap_or(0) as usize, rd_u64(elem + 0x2b8).unwrap_or(0) as usize)
}
#[inline] unsafe fn ab_type(base: usize, i: usize) -> i32 { rd_i32(base + i * 0x28).unwrap_or(-1) }
#[inline] unsafe fn ab_has(elem: usize, t: i32) -> bool {
    let (b, c) = ab_list(elem); for i in 0..c { if ab_type(b, i) == t { return true; } } false
}
// ★재귀 ready walker: 쿨다운 그룹이 재귀 중첩(0x18b9490 2서브리스트 / 0x18e1da0·0x1a7e440 1서브리스트, 각 sub의 vt+0x58 재귀).
//   terminal(비재귀 vt+0x58)은 emulate_getter. 미지 terminal은 GB_TERM에 기록(찾기용). any-ready면 true.
unsafe fn ready_walk(rfn: usize, arg: usize, exe: usize, depth: u32) -> Option<bool> {
    if depth > 10 { return Some(false); }
    // 서브리스트 순회 헬퍼: (offset, count_offset, stride). 각 sub{data@0,vt@8}의 vt+0x58 재귀.
    let walk_list = |aoff: usize, coff: usize, stride: usize, d: u32| -> Option<bool> {
        let mut p = rd_u64(arg + aoff)? as usize;
        let cnt = rd_u64(arg + coff)?;
        if cnt > 256 { return Some(false); }
        let mut i = 0u64;
        while i < cnt {
            let data = rd_u64(p)? as usize;
            let svt = rd_u64(p + 8)? as usize;
            let g = rd_u64(svt + 0x58)? as usize;
            let larg = ((rd_u64(svt + 0x10)?.wrapping_sub(1) & !0xf) as usize).wrapping_add(data).wrapping_add(0x10);
            if ready_walk(g, larg, exe, d + 1)? { return Some(true); }
            p += stride; i += 1;
        }
        Some(false)
    };
    if rfn == exe + 0x1db1eb0 {   // 0.4.13_5(was 0x18b9490) mask-sig 유일
        if walk_list(0x50, 0x58, 0x18, depth)? { return Some(true); }
        return walk_list(0x68, 0x70, 0x10, depth);
    }
    if rfn == exe + 0x1db2c30 { return walk_list(0x50, 0x58, 0x18, depth); }   // 0.4.13_5(was 0x18e1da0)
    if rfn == exe + 0x1b46ae0 { return walk_list(0x8, 0x10, 0x10, depth); }   // 0.4.13_5(was 0x1a7e440). vt+0x48 분기 미발화 → vt+0x58만
    // terminal: 순수게터면 emulate. 미지면 GB_TERM에 기록 후 loose 시도.
    match emulate_getter(rfn, arg) {
        Some(v) => Some(v != 0),
        None => { GB_TERM.store(rfn.wrapping_sub(exe), core::sync::atomic::Ordering::Relaxed); Some(false) }
    }
}
unsafe fn dispatch_ready(rfn: usize, arg: usize, exe: usize) -> Option<bool> {
    ready_walk(rfn, arg, exe, 0)
}
/// 능력 ready 체크: (*(elem[holder]+0xb0))(((elem[holder][0x10]-1)&~0xf)+elem[vec]+0x10)
unsafe fn ab_ready(elem: usize, holder: usize, vec: usize, exe: usize) -> Option<bool> {
    let vt = rd_u64(elem + holder)? as usize;
    let gptr = rd_u64(vt + 0xb0)? as usize;
    let x = rd_u64(vt + 0x10)?;
    let vecp = rd_u64(elem + vec)? as usize;
    let arg = ((x.wrapping_sub(1) & !0xf) as usize).wrapping_add(vecp).wrapping_add(0x10);
    dispatch_ready(gptr, arg, exe)
}
/// 디스크립터 plVar8={data,vtable} ready 체크: (*(plVar8[1]+0xb0))(plVar8[0]+((plVar8[1][0x10]-1)&~0xf)+0x10)
unsafe fn desc_ready(p8: usize, exe: usize) -> Option<bool> {
    let data = rd_u64(p8)? as usize;
    let vt = rd_u64(p8 + 8)? as usize;
    let gptr = rd_u64(vt + 0xb0)? as usize;
    let x = rd_u64(vt + 0x10)?;
    let arg = data.wrapping_add((x.wrapping_sub(1) & !0xf) as usize).wrapping_add(0x10);
    dispatch_ready(gptr, arg, exe)
}
/// 술어 threshold: lvar8=vt90(슬롯 vt90d/vt90v), uvar9=vt_a8(슬롯 a8d/a8v, 술어별 상이!), min=minv.
///   uv=max(minv, lvar8*100/max(1,elem[0x3e8]+extra+100)); return (uv - uv/uvar9) < elem[thr].
///   ★dec4d0/dfb1a0는 vt+0xa8을 s5b0별 다른 슬롯(elem+0x598/0x5a0)서 읽음 + min=1 (dfd1e0=동일슬롯+min=3).
#[allow(clippy::too_many_arguments)]
unsafe fn pred_thr_fail(elem: usize, vt90d: usize, vt90v: usize, a8d: usize, a8v: usize, thr: usize, extra: i64, minv: u64) -> Option<bool> {
    let lvar8 = skill_getter(elem, vt90v, vt90d, 0x90)?;
    let uvar9 = skill_getter(elem, a8v, a8d, 0xa8)?;
    if uvar9 == 0 { return None; }                       // 게임 panic 경로
    let div = ((rd_i32(elem + 0x3e8)? as i64) + extra + 100).max(1) as u64;
    let mut uv = ((lvar8 as u64).wrapping_mul(100)) / div;
    if uv < minv { uv = minv; }
    Some(uv - uv / (uvar9 as u64) < rd_u64(elem + thr)?)
}
/// 최종 능력타입 스캔: type∉{2,3,4,5} 있으면 false, 전부 {2,3,4,5}면 (elem[chk]!=-1)
unsafe fn pred_final_scan(elem: usize, chk: usize) -> Option<bool> {
    let (b, c) = ab_list(elem);
    for i in 0..c { let t = ab_type(b, i); if !(2..=5).contains(&t) { return Some(false); } }
    Some(rd_i32(elem + chk)? != -1)
}

// dfd1e0 재현: 슬롯1(data+0x568/vt+0x570), thr+0xb8, ready holder+0x4b8/vec+0x4b0
unsafe fn gb_dfd1e0(elem: usize, exe: usize) -> Option<bool> {
    if ab_has(elem, 4) { return Some(false); }
    if ab_has(elem, 5) {
        if rd_i32(elem + 0x4f8)? != -1 && ab_ready(elem, 0x4b8, 0x4b0, exe)? { return Some(false); }
    } else if rd_i32(elem + 0x68)? != 0xd { return Some(false); }
    // dfd1e0: vt90/vt_a8 동일슬롯(0x568/0x570), min=3
    if pred_thr_fail(elem, 0x568, 0x570, 0x568, 0x570, 0xb8, 0, 3)? { return Some(false); }
    pred_final_scan(elem, 0x4e0)
}
// dec4d0 재현: 슬롯2(data+0x578/vt+0x580), thr+0xc0, desc=elem+0x4e8(if elem[0x5b0]>2) ready holder+0x4f0/vec+0x4e8
unsafe fn gb_dec4d0(elem: usize, exe: usize) -> Option<bool> {
    if ab_has(elem, 4) { return Some(false); }
    let s5b0 = rd_u64(elem + 0x5c8)?;
    if ab_has(elem, 5) {
        if s5b0 > 2 && rd_i32(elem + 0x500 + 0x30)? != -1 && ab_ready(elem, 0x4f0, 0x4e8, exe)? { return Some(false); }
    } else if rd_i32(elem + 0x68)? != 0xd { return Some(false); }
    // dec4d0: vt90=슬롯(0x578/0x580), vt_a8=(s5b0<3? 0x598/0x5a0 : 0x578/0x580), min=1
    let (a8d, a8v) = if s5b0 < 3 { (0x598usize, 0x5a0usize) } else { (0x578, 0x580) };
    if pred_thr_fail(elem, 0x578, 0x580, a8d, a8v, 0xc0, 0, 1)? { return Some(false); }
    if s5b0 > 2 { Some(rd_i32(elem + 0x500 + 0x30)? != -1) } else { Some(false) }
}
// dfb1a0 재현: 슬롯3(data+0x588/vt+0x590), thr+0xc8, desc=elem+0x520(if elem[0x5b0]>4), 제수 extra=elem[0x454]
unsafe fn gb_dfb1a0(elem: usize, exe: usize) -> Option<bool> {
    if ab_has(elem, 4) { return Some(false); }
    let s5b0 = rd_u64(elem + 0x5c8)?;
    if ab_has(elem, 5) {
        if s5b0 > 4 && rd_i32(elem + 0x520 + 0x30)? != -1 && ab_ready(elem, 0x528, 0x520, exe)? { return Some(false); }
    } else if rd_i32(elem + 0x68)? != 0xd { return Some(false); }
    let extra = rd_i32(elem + 0x454)? as i64;
    // dfb1a0: vt90=슬롯(0x588/0x590), vt_a8=(s5b0<5? 0x598/0x5a0 : 0x588/0x590), min=1, div에 elem[0x454] 가산
    let (a8d, a8v) = if s5b0 < 5 { (0x598usize, 0x5a0usize) } else { (0x588, 0x590) };
    if pred_thr_fail(elem, 0x588, 0x590, a8d, a8v, 0xc8, extra, 1)? { return Some(false); }
    if s5b0 > 4 { Some(rd_i32(elem + 0x520 + 0x30)? != -1) } else { Some(false) }
}

// 스킬 코스트/레벨 게터: (*(elem[0x560]+0x90))(elem[0x558]) = emulate_getter(vt+0x90).
unsafe fn skill_cost(elem: usize) -> Option<i64> {
    let vt = rd_u64(elem + 0x560)? as usize;
    emulate_getter(rd_u64(vt + 0x90)? as usize, rd_u64(elem + 0x558)? as usize)
}
// div = lvl*100/max(1,elem[0x3e4]+100); if<4 →3
unsafe fn skill_div(elem: usize) -> Option<u64> {
    if F5_STUB { return Some(3); }   // ★BISECT 스텁
    let lvl = skill_cost(elem)? as u64;
    let d = (rd_i32(elem + 0x3e4)? + 100).max(1) as u64;
    let mut v = (lvl.wrapping_mul(100)) / d;
    if v < 4 { v = 3; }
    Some(v)
}
#[inline] fn dist2_sat(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = if ax >= bx { ax - bx } else { bx - ax };
    let dy = if ay >= by { ay - by } else { by - ay };
    let sx = dx.checked_mul(dx).unwrap_or(u64::MAX);
    let sy = dy.checked_mul(dy).unwrap_or(u64::MAX);
    sx.checked_add(sy).unwrap_or(u64::MAX)
}
// ★e1b330 재현(0x1e1b330, p1>=30 특수스코어): 근처 적(dist²<0x35a4e9001 & ent[0x68]==1 & alive)마다
//   f5db30(memoized=FUN_14189c4a0) × 쿨감 × 능력매치배율 × 거리배율 누적.
unsafe fn my_e1b330(p1cnt: u64, p3_holder: usize, p5_self: usize, sx: u64, sy: u64, exe: usize) -> Option<i64> {
    if p1cnt < 0x1e || rd_u8(p5_self) != 0 { return Some(0); }
    let team = rd_i64(p5_self + 8)?;
    if (1 - team) as u64 >= 2 { return Some(0); }
    let opp = (1 - team) as usize;
    let rhd = rd_u64(p3_holder)? as usize;
    let rh = rd_u64(p3_holder + 8)? as usize;
    let thr = rd_i64(rd_u64(rh + 8)? as usize + 0x12f8)?;
    let lvar20 = if rd_i32(p5_self + 0x68)? == 0xd { 0x3ci64 } else { 0x28 };
    let lvar6 = rd_u64(p5_self + 0x5c0)?;
    let mut acc = 0i64;
    for (bi, ci) in [(opp*4 + 2, opp*4 + 5), (opp*4 + 0xa, opp*4 + 0xd), (opp*4 + 0x12, opp*4 + 0x15)] {
        let begin = rd_u64(rhd + bi*8)? as usize;
        let cnt = rd_u64(rhd + ci*8)?;
        if cnt > 64 || !ptr_ok(begin) { continue; }
        for k in 0..cnt {
            let ent = rd_u64(begin + (k as usize)*8)? as usize;
            if !ptr_ok(ent) { continue; }
            let d2 = dist2_sat(sx, sy, rd_u64(ent + 0x660)?, rd_u64(ent + 0x668)?);
            if d2 < 0x35a4e9001 && rd_i32(ent + 0x68)? == 1 && rd_i32(ent + 0x4c0)? != -1 {
                let dmg = my_f5db30(ent + 0x490, ent, p5_self, exe)?;   // FUN_14189c4a0 = memoized f5db30
                if dmg != 0 {
                    let div = skill_div(ent)?;
                    let v = (thr as u64).wrapping_mul(dmg as u64) / div;
                    let i3 = rd_i32(ent + 0x88)?;
                    let m1: i64 = if i3 == 1 && rd_i64(ent + 0x90)? == lvar6 as i64 { 100 }
                        else if i3 == 1 { lvar20 } else { 0x50 };
                    let m2: i64 = if d2 > 8100000000 { if rd_u8(ent + 0x118) != 0 { 0x41 } else { 0x2d } } else { 100 };
                    acc = acc.wrapping_add(((m1 as u64).wrapping_mul(v).wrapping_mul(m2 as u64) / 10000) as i64);
                }
            }
        }
    }
    Some(acc)
}

type PredFn = unsafe extern "win64" fn(usize) -> u64;          // dec1f0/dfd1e0/dec4d0/dfb1a0(elem)->char(low)
type Vt90Fn = unsafe extern "win64" fn(usize, usize) -> i64;   // skill level getter

#[inline] unsafe fn call_pred(exe: usize, rva: usize, elem: usize) -> u8 {
    let f: PredFn = core::mem::transmute(exe + rva);
    (f(elem) & 0xff) as u8
}
// (call_f5db30/F5Fn 제거: 죽은 오라클 + RVA_DEFAULT_AB2 const화로 불필요, 2026-06-19)

/// 능력 비용/스킬레벨 게터: (*(elem[0x560]+0x90))(elem[0x558], elem)
#[inline] unsafe fn skill_lvl(elem: usize) -> i64 {
    let v560 = rd_u64(elem + 0x560).unwrap_or(0) as usize;
    let g = rd_u64(v560 + 0x90).unwrap_or(0) as usize;
    if !ptr_ok(g) { return 0; }
    let f: Vt90Fn = core::mem::transmute(g);
    f(rd_u64(elem + 0x558).unwrap_or(0) as usize, elem)
}

/// 능력 리스트(elem+0x2b0, count elem+0x2b8, stride 0x28)에 type t 존재?
#[inline] unsafe fn has_ability_type(elem: usize, t: i32) -> bool {
    let base = rd_u64(elem + 0x2b0).unwrap_or(0) as usize;
    let cnt = rd_u64(elem + 0x2b8).unwrap_or(0) as usize;
    if !ptr_ok(base) { return false; }
    for i in 0..cnt { if rd_i32(base + i*0x28).unwrap_or(-1) == t { return true; } }
    false
}

/// 0x1f80320 재현. 반환=(score, my_draws). None=재현불가(panic경로/특수).
pub unsafe fn my_f80320(c: &F80Ctx, exe: usize) -> Option<(u64, u32)> {
    // 특수 경로: *(p5+0x470)!=0 → Vec정리 후 0x7fff..ff. (스코어 아님, 스킵)
    if rd_u8(c.p5 + 0x470) != 0 { return Some((0x7fffffffffffffff, 0)); }
    let rhd  = rd_u64(c.p3)? as usize;          // local_78
    let champ = rd_u64(rhd)? as usize;          // local_a8
    let vtbl = rd_u64(rhd + 8)? as usize;       // local_b0
    let p3_1 = rd_u64(c.p3 + 8)? as usize;      // local_70 = p3[1]
    let thr  = rd_i64(rd_u64(p3_1 + 8)? as usize + 0x12f8)?;  // *(local_70[1]+0x12f8)
    // self a8 record (local_68)
    let self_a8 = dd7_slot_a8(champ, rd_u64(c.p5 + 0x5c0)?);
    if self_a8 == 0 { return None; }            // 게임 panic
    let self_lane = rd_u32(self_a8 + 0x738) as usize;
    // 판단력 roll 범위
    let jc = (rd_i64(c.athlete + 0x380)?.wrapping_mul(rd_i64(c.athlete + 0x218)?)) / 1000;
    let jc = jc.clamp(0, 100);
    let spread = ((900i64 - 9 * jc) >> 1) as u64;   // (900-9c)/2
    let lo = 1000 - spread; let hi = 1000 + spread;

    let mut rng = RngSim::new(c.rng)?;
    let mut draws = 0u32;
    for k in 0..6 { GB_SITE[k].store(0, Ordering::Relaxed); }   // ★per-site draw 진단 리셋
    let w = |off: usize| -> i64 { rd_i64(rhd + (off) * 8).unwrap_or(0) }; // local_78[off]
    let abil_w = |idx0: usize, off: usize| -> i64 { rd_i64(rhd + (idx0 + off) * 8).unwrap_or(0) };
    let _ = w;

    let mut num: u64 = 0;       // uVar6 (분자)
    let mut den: u64 = 0;       // local_58 (분모)

    // ── list1 루프 (param_6): 능력 가중치 ──
    let l1_begin = rd_u64(c.p6)? as usize;
    let l1_len = rd_u64(c.p6 + 0x18)? as usize;     // param_6[3]
    for i in 0..l1_len {
        let elem = rd_u64(l1_begin + i*8)? as usize;
        let lv5 = dd7_slot_a8(champ, rd_u64(elem + 0x5c0)?);
        if lv5 == 0 { return None; }
        // dec1f0==0(=type3 능력 존재) → role 점프테이블 조기탈출. 재현 보류(캡처 제외). 순수-read 게이트.
        if ab_has(elem, 3) { return None; }
        let team = rd_u64(lv5 + 0x6a8)?;
        if team >= 2 { return None; }
        let team = team as usize;
        let a8lane = rd_u32(lv5 + 0x738) as usize;
        let idx0 = team*500 + a8lane*100 + self_lane;
        // base
        let mut roll = rng.gen_range(lo, hi)? as i64; draws += 1; GB_SITE[0].fetch_add(1, Ordering::Relaxed);
        let mut uv15 = (roll * abil_w(idx0, 0x46)) / 1000;
        // slot 게이트 3개: gb_pred==true || elem[0x68]!=0xd || elem[OFF_thr] <= thr  (전부 순수-read)
        let role13 = rd_i32(elem + 0x68).unwrap_or(0) == 0xd;
        let preds = [
            (gb_dfd1e0(elem, exe), 0x4b, 0xb8usize),
            (gb_dec4d0(elem, exe), 0x50, 0xc0),
            (gb_dfb1a0(elem, exe), 0x55, 0xc8),
        ];
        for (pred, slotoff, throff) in preds {
            let pass = pred? || !role13 || rd_i64(elem + throff).unwrap_or(0) <= thr;
            if pass {
                roll = rng.gen_range(lo, hi)? as i64; draws += 1; GB_SITE[1].fetch_add(1, Ordering::Relaxed);
                let uv16 = (roll * abil_w(idx0, slotoff)) / 1000;
                if uv15 < uv16 { uv15 = uv16; }
            }
        }
        // type3 부재 → +0x78 draw (dec1f0 게이트로 has3=false 보장; 안전 체크)
        if !ab_has(elem, 3) {
            roll = rng.gen_range(lo, hi)? as i64; draws += 1; GB_SITE[2].fetch_add(1, Ordering::Relaxed);
            den = den.wrapping_add(((roll * abil_w(idx0, 0x78)) / 1000) as u64);
        }
        // ★type4/type5 tail (디컴 정확): has4면 둘 다 스킵. !has4면:
        //   +0x7d 발생 UNLESS (has5 && elem[0x4e0]!=-1 && ab_ready(0x4b8,0x4b0))
        //   +0x82 발생 UNLESS (has5 && plVar8[+0x30]!=-1 && desc_ready(plVar8))  plVar8=(elem[0x5b0]<3)?DEFAULT:elem+0x4e8
        if !ab_has(elem, 4) {
            let has5 = ab_has(elem, 5);
            let skip7d = has5 && rd_i32(elem + 0x4f8)? != -1 && ab_ready(elem, 0x4b8, 0x4b0, exe).unwrap_or(false);
            if !skip7d {
                roll = rng.gen_range(lo, hi)? as i64; draws += 1; GB_SITE[3].fetch_add(1, Ordering::Relaxed);
                den = den.wrapping_add(((roll * abil_w(idx0, 0x7d)) / 1000) as u64);
            }
            let p8 = if rd_u64(elem + 0x5c8)? < 3 { default_ab2_ptr() } else { elem + 0x500 };
            let skip82 = has5 && rd_i32(p8 + 0x30)? != -1 && desc_ready(p8, exe).unwrap_or(false);
            if !skip82 {
                roll = rng.gen_range(lo, hi)? as i64; draws += 1; GB_SITE[4].fetch_add(1, Ordering::Relaxed);
                den = den.wrapping_add(((roll * abil_w(idx0, 0x82)) / 1000) as u64);
            }
        }
        let _ = roll;
        num = num.wrapping_add(uv15 as u64);
    }

    // ── list2 루프 (param_7): f5db30 데미지 + 2 roll/후보 ──
    let l2_begin = rd_u64(c.p7)? as usize;
    let l2_len = rd_u64(c.p7 + 0x18)? as usize;
    for i in 0..l2_len {
        let elem = rd_u64(l2_begin + i*8)? as usize;
        if rd_i32(elem + 0x4c0)? == -1 { return None; }   // 게임 panic
        let dmg = my_f5db30(elem + 0x490, elem, c.p5, exe)?;   // atk=elem, target=self(p5)
        let roll1 = rng.gen_range(lo, hi)? as i64; draws += 1; GB_SITE[5].fetch_add(1, Ordering::Relaxed);
        let div = skill_div(elem)?;
        let roll2 = rng.gen_range(lo, hi)? as i64; draws += 1; GB_SITE[5].fetch_add(1, Ordering::Relaxed);
        den = den.wrapping_add(((thr as u64).wrapping_mul(dmg as u64).wrapping_mul(roll1 as u64) / 1000) / div);
        num = num.wrapping_add((roll2 as u64).wrapping_mul(dmg as u64) / 1000);
    }

    // ── 적 레인 거리 루프 (roll 없음, f5db30 직접) ──
    let opp = 1 - rd_i64(self_a8 + 0x6a8)?;
    if (opp as u64) >= 2 { return None; }
    let opp = opp as usize;
    let mut local_50 = num;
    let sx = rd_u64(c.p5 + 0x660)?; let sy = rd_u64(c.p5 + 0x668)?;
    let enl_begin = rd_u64(rhd + (opp*4 + 0x1e)*8)? as usize;
    let enl_cnt = rd_u64(rhd + (opp*4 + 0x21)*8)?;
    if !F5_STUB && enl_cnt <= 64 && ptr_ok(enl_begin) {
        for k in 0..enl_cnt {
            let elem = rd_u64(enl_begin + (k as usize)*8)? as usize;
            if !ptr_ok(elem) { continue; }
            let d2 = dist2_sat(sx, sy, rd_u64(elem + 0x660)?, rd_u64(elem + 0x668)?);
            if d2 < 0x53d1ac101 && rd_i32(elem + 0x4c0)? != -1 {
                let dmg = my_f5db30(elem + 0x490, elem, c.p5, exe)?;
                let div = skill_div(elem)?;
                den = den.wrapping_add((thr as u64).wrapping_mul(dmg as u64) / div);
                local_50 = local_50.wrapping_add(dmg as u64);
            }
        }
    }

    // ── p1>=30: e1b330 / p1<30: 아군풀(미재현→보류) ──
    if c.p1 < 0x1e {
        let _ = vtbl;
        return None;   // 아군풀 경로(p1<30) 보류 — 캡처는 전부 p1>=30
    }
    // p1>=30: e1b330 (게이트 vtbl[0x28]/[0x20] 조건은 우선 무조건 호출, score DIFF로 검증)
    // ★재활성(2026-06-18): 크래시는 환경(0.4.13_5 업데이트)였음 확정. score DIFF 75건 전부 l1=0(den=e1b330뿐)서 my>game = e1b330 누락이 원인. E1B330 0x1d2f2d0 바디동일 확정.
    let bisect_e1b330 = true;
    if bisect_e1b330 {
        let e = my_e1b330(c.p1, c.p3, c.p5, sx, sy, exe)?;
        den = den.wrapping_add(e as u64);
    }

    // ── 최종: (max(0, self_hp - local_50) * 0x3c) / den ──
    let self_hp = rd_u64(c.p5 + 0x670)?;
    let rem = if local_50 <= self_hp { self_hp - local_50 } else { 0 };
    let d = den + (den == 0) as u64;
    Some((rem.wrapping_mul(0x3c) / d, draws))
}

// ════════════════════════════════════════════════════════════════════════
// my_generic_build (FUN_1420def90 본체 재현, task#23) — 1차: 영역 A early-exit
// 반환 Option<(kind@+0x58, arg@+0x60)>. None=미예측(메인빌드/영역 B/C/D 미구현).
// 영역 A early-exit: zone클램프(team별 2박스)/deadline 게이트/후보수집 nearest. 정독=genbuild_body_A.md+asm.
//   param2>0xb : (box안 || deadline미경과) → kind4(RETURN/HOLD, arg=진입잔존 o_60); else 메인빌드 None
//   param2<=0xb: (box안 || deadline미경과) → 후보수집→nearest(후보≥1 kind3 arg=best+0x5a8 / 0개 kind4); else None
// 검증=gbbody.txt(792 오라클). dd7_slot128/dd7_slot20/rd_*/ptr_ok = 같은 스코프(include).
// ════════════════════════════════════════════════════════════════════════
pub struct GBCtx {
    pub param2: u64, pub athlete: usize, pub rh_chain: usize, pub s_champ: usize,
    pub o_30: i64, pub o_38: i64, pub o_60: u64,   // out 진입 스냅샷(deadline out+0x30/+0x38, arg잔존 out+0x60)
}
#[inline] fn gb_in_box(team: u64, x: u64, y: u64) -> bool {
    let b = |v: u64, lo: u64, hi: u64| v.wrapping_sub(lo) < hi;   // v in [lo, lo+hi)
    if team == 1 {
        (x < 0xfa01 && b(y, 0xe4250, 0x27101)) || (x < 0x27101 && b(y, 0x9f9f0, 0xfa01))
    } else {
        (b(x, 0xe4250, 0x27101) && y < 0xfa01) || (b(x, 0x9f9f0, 0xfa01) && y < 0x27101)
    }
}
unsafe fn gb_collect_cands(rhd: usize, team: u64) -> ([usize; 5], usize) {
    // [rhd+0x1e0 + (1-team)*5*8] 5슬롯 non-null (asm 0x20df38d/0x20df66e)
    let base = rhd + 0x1e0 + (1u64.wrapping_sub(team).wrapping_mul(5) as usize) * 8;
    let mut v = [0usize; 5]; let mut n = 0usize;   // ★Vec→스택배열(힙할당 제거)
    for i in 0..5usize { let e = rd_u64(base + i * 8).unwrap_or(0) as usize; if e != 0 { v[n] = e; n += 1; } }
    (v, n)
}
unsafe fn gb_nearest(t: usize, cands: &[usize]) -> usize {
    let tx = rd_u64(t + 0x660).unwrap_or(0); let ty = rd_u64(t + 0x668).unwrap_or(0);
    let d2 = |e: usize| -> u64 {
        let ex = rd_u64(e + 0x660).unwrap_or(0); let ey = rd_u64(e + 0x668).unwrap_or(0);
        let dx = if ex >= tx { ex - tx } else { tx - ex };
        let dy = if ey >= ty { ey - ty } else { ty - ey };
        dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
    };
    let mut best = cands[0]; let mut bd = d2(cands[0]);
    for &e in &cands[1..] { let dd = d2(e); if dd < bd { bd = dd; best = e; } }   // min dist²(첫 min 유지)
    best
}
// ════════════════════════════════════════════════════════════════════════
// my_gb_mainbuild — generic_build 메인빌드(영역 A후반→B→C→D) 출력-우선 재현 (task#23)
//   진입: param2>0xb && zone-in && !deadline (영역 A early-exit 미해당; gbbody 446캡처가 전부 이 경로).
//   전략: 오케스트레이션(분기→kind/arg)만 재현, callee=oracle/component. 검증=gbbody(kind14) OK/DIFF/NP.
//   ★단계적 — 현재 S1(foundation): 영역 A 로컬 추출만, kind 미산출(None). 단계계획=genbuild_body_D.md "구체 단계 계획".
//   재사용 예정 자산: my_f80320 / cand_filter_repro / my_combat_dmg / my_203cb30·690 / gb_dfd1e0류.
// ════════════════════════════════════════════════════════════════════════
struct GbMain {
    disc_low: u8, disc_mid: u8, disc_high: u8,   // disc-2D 키 (영역 A §1: S+0x3e6/7/8)
    team: u64, opp: u64,                          // athlete.team(+0x6a8) / 1-team(상대 lane idx)
}
// 영역 A §1 disc-2D 키 추출: esi=(byte[S+0x3e8]<<16)|word[S+0x3e6]. lo=[0x3e6] mid=[0x3e7] hi=[0x3e8].
unsafe fn gb_extract(s_champ: usize, athlete: usize) -> Option<GbMain> {
    let team = rd_u64(athlete + 0x6a8).unwrap_or(99);
    if team > 1 { return None; }
    Some(GbMain {
        disc_low: rd_u8(s_champ + 0x3e6), disc_mid: rd_u8(s_champ + 0x3e7), disc_high: rd_u8(s_champ + 0x3e8),
        team, opp: 1 - team,
    })
}
#[allow(unused_variables)]
unsafe fn my_gb_mainbuild(c: &GBCtx, rhd: usize, champ: usize, t: usize, team: u64, exe: usize) -> Option<(i64, u64)> {
    let m = gb_extract(c.s_champ, c.athlete)?;
    // TODO S2: 영역 A §9 첫 F80320(my_f80320) ENGAGE code6 push + §10 disc_low facet 디스패치(r14=out+0x8d → 4/5/그외)
    // TODO S3: 영역 B kind4/7 게이트(0x20e0d9d: out.kind prior·0x20eccc0·0x20bfa70 oracle·out+0x80) — 첫 검증가능 OK
    // TODO S4: 영역 C disc-2D LANE F80320×4(argmax) → best 라인후보 + 위협[0x240]/HP%[0x270]/도달[0x250·258]
    // TODO S5: 영역 D 출력조립(D정본 완전제어흐름, my_203cb30/690 post-commit·argmax) → kind@+0x58/arg@+0x60
    let _ = (m.disc_low, m.disc_mid, m.disc_high, m.opp, m.team, rhd, champ, t, team, exe);
    None   // S1: 미예측(NP) 유지. S2~에서 kind 산출 → gbbody OK 전환.
}

// ════════════════════════════════════════════════════════════════════════
// gb_region_d — 영역 D 출력결정(kind@+0x58, arg@+0x60)의 RNG-free 순수재현 (S5 core)
//   정본 = genbuild_body_D.md (4차 ghidra-re 바이트확정). 입력=영역D 진입시점 로컬(상류 A/B/C가 채움).
//   ★검증: 런타임 캡처(mid-func 0x42a3 detour)로 RegionD 채워 game kind vs gb_region_d 대조.
//   확정분기(param2≥30 0x3a92 경로): kind0/2/3/4. sil!=1·0x44a6 idle·0x4659는 TODO(미확정).
//   kind0 = sil==1 && l108>l158 && dl  (임계값 무관 robust → 첫 OK 표적).
// ════════════════════════════════════════════════════════════════════════
#[allow(dead_code)]
pub struct RegionD {
    pub r12: u64, pub r13: u64,        // self보간거리²(=[0xe8]²+rbx²) / A*경로거리²(=[0x240]²)
    pub l108: i64, pub l158: i64,      // [0x108] 동시액션임계 / [0x158] 슬롯·팀 카운트
    pub l120: i64, pub l140: i64,      // [0x120] F80320 4th점수 / [0x140] 위치비교값
    pub l270: i64, pub l27e: u8, pub l27f: u8,  // [0x270] HP% / [0x27e]·[0x27f] 거점·도달 bool
    pub out_8b: u8,                    // out+0x8b 상태분류(0x20ea7f0 결과)
    pub l130: i64,                     // [0x130] base 곱수
    pub arg_src: u64,                  // [[0x280]+0x5a8] 공통 arg(대상 athlete 위치/타겟핸들)
    pub t_arg: u64,                    // T+0x5a8 (kind3 0x441c용)
    pub param2: u64,
    pub o40: i64,                      // out+0x40 (dedc0 타이밍분기 게이트)
    pub o88: u8,                       // out+0x88 (dedc0 facet5 토글)
    pub o8d: u8,                       // out+0x8d (dedc0 상태바이트 b)
    pub o60: u64,                      // out+0x60 캡처시점(=kind4가 미기록으로 유지하는 arg)
    pub l_e0: i64,                     // [0xe0] 동시-액션 카운트(0x44a6/0x452e `cmp lE0`)
    pub l258: u8,                      // [0x258] 페이즈/모드 플래그(0x4724 cl=l258|al)
    pub l148: i64,                     // [0x148] param2 사본(진단용)
    pub l64: u8,                       // [0x64] 플래그(0x4628 ^1)
    pub dedc0: Option<bool>,           // 해결된 dedc0 게이트(caller가 my_dedc0 or shadow-call로 채움). None=timing 미해결(NP).
}
// dedc0 (FUN_1420dedc0) 순수재현: Option<bool>. out+0x40!=0 → pure b-logic. out+0x40==0 → timing(vtable getter)필요시 None.
//   b-logic: b=out+0x8d; if b>2 && b!=4 { b==5 ? (out+0x88^1)!=0 : true } else false.
//   out+0x40==0: timing실패→false / 통과→b-logic. b-logic==false면 둘다 false → Some(false). b-logic==true면 ambiguous → None.
fn my_dedc0(o40: i64, o88: u8, o8d: u8) -> Option<bool> {
    let b = o8d;
    let b_logic = if b > 2 && b != 4 { if b == 5 { (o88 ^ 1) != 0 } else { true } } else { false };
    if o40 != 0 { Some(b_logic) }
    else if !b_logic { Some(false) }
    else { None }   // out+0x40==0 & b_logic=true → timing(vtable getter) 필요 = 미확정
}
#[allow(dead_code)]
pub unsafe fn gb_region_d(d: &RegionD, exe: usize) -> Option<(i64, u64, u16)> {
    let _pg = perf_guard(3);
    // 임계(empirical concrete, ghidra-re >>2유도는 오류): r15=base/50(120 검증)·r14=3·base/100(180)·rbx=base/100(60).
    let threat = |i: u8| rd_i64(exe + 0x3669428 + (i as usize).min(3) * 8).unwrap_or(100);
    let base = (if d.l158 >= 3 { threat(d.out_8b) } else { 100 }).wrapping_mul(d.l130).wrapping_mul(TUNE_GB_MULT.load(Ordering::Relaxed)) / 100;   // ★튜닝: 영역D 거리임계 배율(t_gb%; 세 임계 60/120/180 비례)
    let rbx_thr = (base / tune("gb_rbx_div", 100).max(1)) as u64;          // ★튜닝: 근거리밴드 divisor(기본 base/100)
    let r15_thr = (base / tune("gb_r15_div", 50).max(1)) as u64;           // ★튜닝: 중거리밴드(기본 base/50)
    let r14_thr = (base.wrapping_mul(tune("gb_r14_num", 3)) / 100) as u64; // ★튜닝: 원거리밴드(기본 base×3/100)
    let l140u = d.l140 as u64;
    let dl = d.r12 <= d.r13 && d.l120 <= d.l140;   // (r12≤r13) & (l120≤l140)
    let dedc0 = d.dedc0;                            // caller가 해결(my_dedc0 순수 or out+0x40==0&&b_logic시 dedc0 오라클). None=NP.
    let arg = d.arg_src;
    // ★반환 = (kind@+0x58, arg@+0x60, push코드@action Vec). push코드 0=push없음. kind4 4분기만 push(c-table: 0x207/0x307/0x407/0x507, l108/lE0 게이트).
    let p1 = d.l108 >= 1; let p2 = d.l108 >= 2;     // push [0x108] 게이트
    // ★0x20e42a3 훅 도달 ⟺ sil==1 (cmp sil,1; je 0x42a3). 따라서 sil 게이트 불필요(항상 sil==1 경로). [sil 재구성은 sil_prior 무시라 부정확→제거]
    if d.l108 >= d.l158 {                  // path B (0x4318)
        if d.l108 <= d.l158 {             // B2 (l108==l158, 0x43e7)
            if dl { return Some((2, arg, 0)); }                      // 0x43fd kind2
            // B2 dl==0 → 0x44a6 트리
            if d.l_e0 >= 2 && l140u <= r14_thr {                     // 0x44a6 (lE0≥2 && l140≤r14) → 0x4518 push 0x407
                match dedc0 { Some(true)=>return Some((4, d.o60, if p1 {0x407} else {0})), Some(false)=>{}, None=>return None }
            }
            if d.l120 <= d.l140 { return Some((2, arg, 0)); }        // 0x4712 jbe → kind2
            if l140u <= r15_thr {                                    // 0x471b (l140≤r15) → 0x4841 push 0x507
                return match dedc0 { Some(false)=>Some((2,arg,0)), Some(true)=>Some((4, d.o60, if p2 {0x507} else {0})), None=>None };
            }
            let cl = d.l258 != 0 || l140u <= r14_thr;                // 0x4724 cl = l258 | (l140≤r14)
            if !cl { return Some((2, arg, 0)); }                     // cl==0 → 0x4762 kind2
            match dedc0 { Some(false)=>Some((2,arg,0)), Some(true)=>Some((3,arg,0)), None=>None }  // cl!=0 → esi=gate|2
        } else {                          // B1 (l108>l158, 0x4347)
            if dl { return Some((0, arg, 0)); }                      // 0x435d kind0
            if d.l120 > d.l140 {                                     // 0x4659 트리 (★런타임 ground-truth, l140 밴드별 캐스케이드)
                if l140u >= r14_thr { return Some((0, arg, 0)); }    // ≥180(r14) → kind0
                if l140u < rbx_thr {                                 // <60(rbx) → dedc0?kind4(+push0x307):kind2
                    return match dedc0 { Some(true)=>Some((4, d.o60, if p2 {0x307} else {0})), Some(false)=>Some((2,arg,0)), None=>None };
                }
                if l140u < r15_thr {                                 // 60≤l140<120(r15) → dedc0?kind3:kind2 (런타임확정)
                    return match dedc0 { Some(true)=>Some((3, arg, 0)), Some(false)=>Some((2,arg,0)), None=>None };
                }
                return Some((2, arg, 0));                            // 120≤l140<180 → kind2
            }
            let esi = if l140u <= r15_thr { 2 } else { 0 };          // 0x4482→0x4769 (l120≤l140)
            Some((esi, arg, 0))
        }
    } else {                              // path A (l108<l158, 0x42c5)
        // 0x42cc/0x42dc: (r12≤r13 && l120≤l140) → 상단 dedc0 게이트 ; else → 0x452e
        if d.r12 <= d.r13 && d.l120 <= d.l140 {
            match dedc0 { Some(true)=>return Some((3,arg,0)), Some(false)=>{}, None=>return None }  // 0x2cc1 kind3
        }
        // 0x452e/0x45c7/0x45e4 (ghidra-re 바이트확정): l140≤r14 & dedc0=true →
        //   lE0≠0 → 0x452e kind4 + push 0x207(l108≥2) ; lE0==0 → 0x45e4→0x4615: (l64홀 && l148≥0x20 && l140>rbx)→kind3, else kind4.
        if l140u <= r14_thr {
            match dedc0 {
                Some(true) => {
                    if d.l_e0 != 0 {
                        Some((4, d.o60, if p2 { 0x207 } else { 0 }))          // 0x452e → kind4 + push 0x207(l108≥2)
                    } else if (d.l64 & 1) == 1 && d.l148 >= 0x20 && l140u > rbx_thr {
                        Some((3, arg, 0))                                     // 0x463b esi=3 → kind3(arg=arg_src)
                    } else {
                        Some((4, d.o60, 0))                                   // 0x4861 facet → kind4 유지(push 없음)
                    }
                }
                Some(false) => Some((2, arg, 0)),                            // dedc0=false → 0x4762 kind2
                None => None,
            }
        } else {
            Some((2, arg, 0))                                                // l140>r14 → kind2 (P1 다운그레이드는 o8b==2시만=이 매치업 out8b=0 미발생)
        }
    }
}
// ════ 0.5.0_3 region D 결정트리 (a7013, ganker.rs FUN_1422d9780). macro_op 재구조화 → 구 gb_region_d와 별개. RNG-free. ════
//   입력 locals(capture 디투어가 새 RBP오프셋서 읽음): cnt(TTD전진 [0x110])·da(원거리정규 [0x100])·db(TTD전진 [0x108])·d2(거리² [0x170])·l2(한계² [0x1d0])·score(SCORE_BRANCH refA[0x20]-vt0x28)·sim_scale(sim[0x12f8]).
//   반환 action decision: 0=SKIP(커밋안함)·3=MOVE(action code3, payload=self[0x5a8])·0xb=ENGAGE_B(family-B FUN_1422e3d80)·-1=ABORT(threat게이트).
//   ⬜capture 디투어 레이아웃/gbrd 하네스 macro_op 대조/스코어러 FUN_1420a5030(CNT/DB 산출)·threat합 = 검증재개시 배선.
pub fn gb_region_d_050(cnt: i64, da: i64, db: i64, d2: u64, l2: u64, score: i64, sim_scale: i64) -> i64 {
    if cnt > tune("gb_cnt_skip", 30) && d2 <= l2 { return 0; }          // CNT>30 & 근접 → SKIP
    if da < tune("gb_da_thr", 121) {
        if cnt < tune("gb_cnt_move", 61) { return 3; }                  // MOVE(code3)
        return if d2 <= l2 { 0 } else { 3 };       // CNT>=61: 근접 SKIP / else MOVE
    }
    // da >= 121
    if db > tune("gb_db_engage", 120) && d2 >= l2 { return 0xb; }         // DB>120 & 원거리 → ENGAGE_B
    // SCORE_BRANCH: SCORE > sim_scale*3 → SKIP; D2>L2 → ENGAGE_B else SKIP
    if score > sim_scale.wrapping_mul(tune("gb_score_mult", 3)) { return 0; }
    if d2 > l2 { 0xb } else { 0 }
}
pub unsafe fn my_generic_build(c: &GBCtx, _exe: usize) -> Option<(i64, u64)> {
    let rhd = rd_u64(c.rh_chain).unwrap_or(0) as usize; if !ptr_ok(rhd) { return None; }
    let champ = rd_u64(rhd).unwrap_or(0) as usize; if !ptr_ok(champ) { return None; }   // sim
    let handle = rd_u64(c.athlete + 0x6a0).unwrap_or(0);
    let t = dd7_slot128(champ, handle); if !ptr_ok(t) { return None; }   // resolved entity (vt128)
    let team = rd_u64(c.athlete + 0x6a8).unwrap_or(99); if team > 1 { return None; }
    let tx = rd_u64(t + 0x660).unwrap_or(0); let ty = rd_u64(t + 0x668).unwrap_or(0);
    let in_box = gb_in_box(team, tx, ty);
    // zone밖 deadline 게이트(0x20df5f5/0x20df635): out+0x30!=0 && dd7_slot20(champ) < out+0x38
    let deadline_active = c.o_30 != 0 && dd7_slot20(champ) < c.o_38;
    if c.param2 > 0xb {
        if in_box || deadline_active { return Some((4, c.o_60)); }   // 0x2d0/0x62b kind4 RETURN/HOLD
        my_gb_mainbuild(c, rhd, champ, t, team, _exe)   // 0x726 메인빌드(영역 A후반/B/C/D) — S1 foundation(현 None)
    } else {
        if !(in_box || deadline_active) { return None; }   // 0x726 메인빌드 미예측
        let (cands, ncand) = gb_collect_cands(rhd, team);  // 0x38d/0x66e
        if ncand == 0 { return Some((4, c.o_60)); }        // 0x5a5 fallback kind4
        let best = gb_nearest(t, &cands[..ncand]);         // 0x450 nearest
        Some((3, rd_u64(best + 0x5c0).unwrap_or(0)))       // 0x504 kind3 MOVE-TO-NEAREST(arg=best+0x5a8)
    }
}

// ════════════════════════════════════════════════════════════════════════
// 영역 D callee: 0x203cb30(단일 종합점수 3슬롯) / 0x20c0690(post 1슬롯)
//   정본 = genbuild_body_D.md "0x203cb30/0x20c0690 재현물 매핑"(2026-06-21).
//   resolver vt[0x28]=gb_resolver(SHIM oracle), my_combat_dmg(순수재현), norm=vt[0x90](oracle).
//   ★검증대상: 함수시작 detour로 (entity ptr → 반환) game vs mine DIFF=0.
// ════════════════════════════════════════════════════════════════════════
const RVA_GB_ATKCTX_CB30: usize = 0x364b518;   // &PTR_1435d8018 (0x203cb30 resolver r9) ← 0.5.7 재핀 +0x20 (주변4KB 일치율 100%)
const RVA_GB_ATKCTX_C0690: usize = 0x35efd68;  // &PTR_1435efd48 (0x20c0690 resolver r9) ← 0.5.7 재핀 +0x20 (주변4KB 일치율 100%)

// resolver vt[0x28]: owner+vt_off=슬롯vtable, owner+buf_off=data버퍼. getter(aligned,g2,g3,atk_ctx)→(rax,rdx)=base 2값.
//   ★SHIM_BOTH 섀도우호출(게임 base getter 1회). SHIM 부재시 (0,0)=스킵.
unsafe fn gb_resolver(owner: usize, g2: usize, g3: usize, vt_off: usize, buf_off: usize, atk_ctx: usize) -> (i64, i64) {
    let v = rd_u64(owner + vt_off).unwrap_or(0) as usize;
    if !ptr_ok(v) { return (0, 0); }
    let inner = rd_u64(v + 0x10).unwrap_or(0) as usize;
    let buf = rd_u64(owner + buf_off).unwrap_or(0) as usize;
    let aligned = (inner.wrapping_sub(1) & !0xf).wrapping_add(buf).wrapping_add(0x10);
    let gptr = rd_u64(v + 0x28).unwrap_or(0) as usize;
    if !ptr_ok(gptr) || !ptr_ok(aligned) { return (0, 0); }
    let both = SHIM_BOTH.load(Ordering::Relaxed);
    if both == 0 { return (0, 0); }
    let mut o = [0i64; 2];
    let s: ShimBoth = core::mem::transmute(both);
    s(o.as_mut_ptr() as usize, gptr, aligned, g2, g3, atk_ctx);
    (o[0], o[1])
}
// norm = max((vt[0x90](e[data], e)*100)/max(e[accel]+100,1), 3). do_max=false면 q+(q==0)(div가드, n3).
unsafe fn gb_norm(e: usize, vt_off: usize, data_off: usize, accel_off: usize, do_max: bool) -> u64 {
    let v = rd_u64(e + vt_off).unwrap_or(0) as usize;
    let g = rd_u64(v + 0x90).unwrap_or(0) as usize;
    let a0 = rd_u64(e + data_off).unwrap_or(0) as usize;
    if !ptr_ok(g) { return if do_max { 3 } else { 1 }; }
    let f: G2 = core::mem::transmute(g);
    let raw = f(a0, e);
    let div = ((rd_i32(e + accel_off).unwrap_or(0) as i64) + 100).max(1) as u64;
    let q = (raw as u64).wrapping_mul(100) / div;
    if do_max { q.max(3) } else { q + (q == 0) as u64 }
}
// slot_score: gate(owner+gate)!=-1 → resolver → skip검사 → (my_combat_dmg(atk=a,tgt=s,flag0/1)×2)*100000.
#[allow(clippy::too_many_arguments)]
unsafe fn gb_slot(owner: usize, a: usize, s: usize, g2: usize, g3: usize,
                  vt_off: usize, buf_off: usize, gate: usize, dt_off: usize,
                  atk_ctx: usize, exe: usize, skip_owner_null: bool) -> u64 {
    if rd_i32(owner + gate).unwrap_or(-1) == -1 { return 0; }
    let (b0, b1) = gb_resolver(owner, g2, g3, vt_off, buf_off, atk_ctx);
    let skip = if skip_owner_null { b0 == 0 && a == 0 } else { b0 == 0 && b1 == 0 };
    if skip { return 0; }
    let dt = rd_u32(owner + dt_off);
    let d0 = my_combat_dmg(a, s, b0, dt, 0, exe);
    let d1 = my_combat_dmg(a, s, b1, dt, 1, exe);
    ((d0 + d1) as u64).wrapping_mul(100000)
}
/// 0x203cb30: 단일 엔티티 종합점수(3슬롯). rh=resolver핸들, a=athlete(점수대상=atk), s=S(sim/tgt).
#[allow(dead_code)]
pub unsafe fn my_203cb30(rh: usize, a: usize, s: usize, exe: usize) -> u64 {
    let ctx = exe + RVA_GB_ATKCTX_CB30;
    let s1 = gb_slot(a, a, s, rh, a, 0x480, 0x478, 0x4a8, 0x4a4, ctx, exe, false);   // 슬롯1
    let n1 = gb_norm(a, 0x560, 0x558, 0x3e4, true);
    let s2 = gb_slot(a, a, s, rh, a, 0x4b8, 0x4b0, 0x4e0, 0x4dc, ctx, exe, false);   // 슬롯2
    let n2 = gb_norm(a, 0x570, 0x568, 0x3e8, true);
    let plv = if rd_u64(a + 0x5c8).unwrap_or(0) < 3 { default_ab2_ptr() } else { a + 0x500 };  // 슬롯3 desc
    let s3 = gb_slot(plv, a, s, rh, a, 0x8, 0x0, 0x30, 0x2c, ctx, exe, true);
    let n3 = gb_norm(a, 0x580, 0x578, 0x3e8, false);
    s1 / n1 + s2 / n2 + s3 / n3
}
/// 0x20c0690: post-commit 점수(1슬롯). struct{[0]=rh,[8]=a,[0x10]=s}. resolver 2nd=a,3rd=s, atk_ctx=efd48.
#[allow(dead_code)]
pub unsafe fn my_20c0690(rh: usize, a: usize, s: usize, exe: usize) -> u64 {
    let _ = rh;
    let ctx = exe + RVA_GB_ATKCTX_C0690;
    let (b0, b1) = gb_resolver(a, a, s, 0x480, 0x478, ctx);
    let slot = if b0 == 0 && b1 == 0 { 0u64 } else {
        let dt = rd_u32(a + 0x4a4);
        ((my_combat_dmg(a, s, b0, dt, 0, exe) + my_combat_dmg(a, s, b1, dt, 1, exe)) as u64).wrapping_mul(100000)
    };
    slot / gb_norm(a, 0x560, 0x558, 0x3e4, true)
}
