//! specemu — `slot.vt+0xa0`(sret BuffSpec 0x120) **잎 impl 전용 미니 에뮬레이터**.
//!
//! 왜 에뮬레이터인가: 잎은 챔피언 어빌리티마다 하나씩 있어 수십 개인데, 전부 **같은 골격의 직선 코드**다
//!   (`call [EST+0x30]` 로 스탯블록 복사 → 몇 개의 `coeff*stat/100 + flat` 항 → sret 에 상수 스토어 나열).
//!   RVA 표를 늘리는 대신 그 명령 집합만 해석한다. `decode_getter`·`composite_shape`·`delegate_pair` 와 같은 전략.
//!
//! ★fail-closed: 모르는 opcode·모르는 주소를 만나면 **즉시 None**(호출부는 NA 로 집계). 값을 지어내지 않는다.
#![allow(dead_code)]
use crate::*;

pub const SPEC_SIZE: usize = 0x120;
/// sret 버퍼를 가리키는 가짜 주소(실제 메모리와 겹치지 않는 상위 대역)
const SRET: u64 = 0xF100_0000_0000_0000;
/// 자기 스택 프레임을 가리키는 가짜 주소. `STK_ENTRY` = 진입 시점 rsp(= 반환주소 위치).
const STK: u64 = 0xF200_0000_0000_0000;
const STK_ENTRY: u64 = STK + 0x1000;
const STK_LO: u64 = STK + 0x800;
const STK_HI: u64 = STK + 0x1800;

#[inline] fn is_sret(a: u64) -> bool { a >= SRET && a < SRET + 0x1000 }
#[inline] fn is_stk(a: u64) -> bool { a >= STK_LO && a < STK_HI }

struct Em {
    r: [Option<u64>; 16],
    stk: [Option<u64>; 512],
    out: [u8; SPEC_SIZE],
    /// xmm0 이 0 으로 채워져 있는가(잎이 쓰는 유일한 xmm 용법)
    xmm0_zero: bool,
}

impl Em {
    #[inline] fn ld_stk(&self, a: u64) -> Option<u64> { if !is_stk(a) || a % 8 != 0 { return None; } self.stk[((a - STK_LO) / 8) as usize] }
    #[inline] fn st_stk(&mut self, a: u64, v: Option<u64>) -> Option<()> { if !is_stk(a) || a % 8 != 0 { return None; } self.stk[((a - STK_LO) / 8) as usize] = v; Some(()) }
    /// 로드: sret/스택은 모델에서, 그 외는 실제 게임 메모리에서
    unsafe fn load(&self, a: u64, sz: usize) -> Option<u64> {
        if is_sret(a) {
            let o = (a - SRET) as usize; if o + sz > SPEC_SIZE { return None; }
            let mut v = 0u64; for i in 0..sz { v |= (self.out[o + i] as u64) << (8 * i); }
            return Some(v);
        }
        if is_stk(a) { let v = self.ld_stk(a)?; return Some(if sz >= 8 { v } else { v & ((1u64 << (8 * sz)) - 1) }); }
        if !ptr_ok(a as usize) { return None; }
        Some(match sz { 8 => rd_u64(a as usize)?, 4 => rd_u32(a as usize) as u64, 1 => rd_u8(a as usize) as u64, _ => return None })
    }
    unsafe fn store(&mut self, a: u64, v: Option<u64>, sz: usize) -> Option<()> {
        if is_sret(a) {
            let o = (a - SRET) as usize; if o + sz > SPEC_SIZE { return None; }
            let v = v?;                                  // 미지값을 sret 에 쓰면 재현 불가 → 실패
            for i in 0..sz { self.out[o + i] = ((v >> (8 * i)) & 0xff) as u8; }
            return Some(());
        }
        if is_stk(a) { if sz != 8 { return None; } return self.st_stk(a, v); }
        None                                             // 실제 메모리 쓰기 = 부작용 → 재현 대상 아님
    }
}

/// ModRM 해석 결과
struct Mrm { reg: usize, rm_reg: Option<usize>, ea: Option<u64>, len: usize }

unsafe fn modrm(em: &Em, p: usize, rex: u8) -> Option<Mrm> {
    let m = rd_u8(p);
    let md = m >> 6;
    let reg = ((((rex >> 2) & 1) << 3) | ((m >> 3) & 7)) as usize;
    let rm = (((rex & 1) << 3) | (m & 7)) as usize;
    if md == 3 { return Some(Mrm { reg, rm_reg: Some(rm), ea: None, len: 1 }); }
    let mut len = 1usize;
    let (base_reg, idx): (usize, Option<(usize, u64)>) = if (m & 7) == 4 {
        let sib = rd_u8(p + 1); len += 1;
        let b = (((rex & 1) << 3) | (sib & 7)) as usize;
        let i = ((((rex >> 1) & 1) << 3) | ((sib >> 3) & 7)) as usize;
        (b, if i == 4 { None } else { Some((i, 1u64 << (sib >> 6))) })
    } else { (rm, None) };
    if md == 0 && (base_reg & 7) == 5 { return None; }               // rip-상대/disp32 전용 = 정적 데이터, 미지원
    let disp: i64 = match md {
        1 => { len += 1; rd_u8(p + len - 1) as i8 as i64 }
        2 => { len += 4; rd_i32(p + len - 4)? as i64 }
        _ => 0,
    };
    let mut a = em.r[base_reg]?.wrapping_add(disp as u64);
    if let Some((i, s)) = idx { a = a.wrapping_add(em.r[i]?.wrapping_mul(s)); }
    Some(Mrm { reg, rm_reg: None, ea: Some(a), len })
}

/// `slot.vt+0xa0` 잎 impl 을 해석해 BuffSpec 을 만든다.
///   `inline` = 게임이 rdx 로 넘기는 값(이미 Arc 보정된 payload) · `sim`/`me` = r8/r9 · `est` = 5번째 인자.
/// ★`vt+0x28` 전용 진입점 — sret 이 없고 인자가 한 칸씩 당겨진다:
/// `rcx = payload · rdx = ctx(패스스루) · r8 = ent(att) · r9 = EST 서술자`, 반환 = `(rax, rdx)` = (물리, 마법).
pub unsafe fn run_eff28(f: usize, payload: u64, ctx: u64, ent: u64, est: u64) -> Option<(u64, u64)> {
    let em = run_regs(f, payload, ctx, ent, est, None)?;
    Some((em.r[0]?, em.r[2]?))
}
/// ★non-sret 단일값 슬롯(`vt+0x40/0x88/0x98/0xb0/0x80`) 전용 — `rcx=p · rdx=arg3 · r8=me · r9=EST`, 반환 `rax`.
pub unsafe fn run_leaf_i64(f: usize, payload: u64, me: u64, est: u64) -> Option<i64> {
    run_regs(f, payload, 0, me, est, None).and_then(|em| em.r[0]).map(|v| v as i64)
}
pub unsafe fn run_spec_leaf(f: usize, inline: u64, sim: u64, me: u64, est: u64) -> Option<[u8; SPEC_SIZE]> {
    run_regs(f, SRET, inline, sim, me, Some(est)).map(|em| em.out)
}
/// 공통 실행기. `stk28=Some(est)` 면 `rcx=sret·rdx=inline·r8=sim·r9=me·[rsp+0x28]=est`(= `vt+0xa0` 규약),
/// `None` 이면 `rcx=payload·rdx=ctx·r8=ent·r9=est`(= `vt+0x28` 규약, sret 없음).
unsafe fn run_regs(f: usize, a1: u64, a2: u64, a3: u64, a4: u64, stk28: Option<u64>) -> Option<Em> {
    if !ptr_ok(f) { return None; }
    let mut em = Em { r: [None; 16], stk: [None; 512], out: [0u8; SPEC_SIZE], xmm0_zero: false };
    em.r[1] = Some(a1);
    em.r[2] = Some(a2);
    em.r[8] = Some(a3);
    em.r[9] = Some(a4);
    em.r[4] = Some(STK_ENTRY);       // rsp
    if let Some(v) = stk28 { em.st_stk(STK_ENTRY + 0x28, Some(v))?; }   // 5번째 인자(sret 규약에만 있다)
    let mut ip = f;
    let mut steps = 0usize;
    loop {
        steps += 1; if steps > 400 { return None; }
        let mut q = ip; let mut rex = 0u8;
        loop { let b = rd_u8(q); if (0x40..0x50).contains(&b) { rex = b; q += 1; } else { break; } }
        let op = rd_u8(q); let w = rex & 8 != 0;
        match op {
            0x50..=0x57 => { let v = em.r[((((rex & 1) << 3) as u8) | (op - 0x50)) as usize]; let sp = em.r[4]?.wrapping_sub(8); em.r[4] = Some(sp); em.st_stk(sp, v)?; ip = q + 1; }
            0x58..=0x5f => { let sp = em.r[4]?; let v = em.ld_stk(sp); em.r[((((rex & 1) << 3) as u8) | (op - 0x58)) as usize] = v; em.r[4] = Some(sp.wrapping_add(8)); ip = q + 1; }
            0x83 | 0x81 => {                                     // add/sub r/m64, imm
                let mm = modrm(&em, q + 1, rex)?; let rr = mm.rm_reg?;
                let (imm, l) = if op == 0x83 { (rd_u8(q + 1 + mm.len) as i8 as i64, 1usize) } else { (rd_i32(q + 1 + mm.len)? as i64, 4usize) };
                let ext = (rd_u8(q + 1) >> 3) & 7;
                let base = em.r[rr]?;
                em.r[rr] = Some(match ext { 0 => base.wrapping_add(imm as u64), 5 => base.wrapping_sub(imm as u64), _ => return None });
                ip = q + 1 + mm.len + l;
            }
            0x89 => { let mm = modrm(&em, q + 1, rex)?; let v = em.r[mm.reg];                       // mov r/m, r
                      if let Some(rr) = mm.rm_reg { em.r[rr] = if w { v } else { v.map(|x| x & 0xffff_ffff) }; }
                      else { em.store(mm.ea?, v, if w { 8 } else { 4 })?; }
                      ip = q + 1 + mm.len; }
            0x8b => { let mm = modrm(&em, q + 1, rex)?;                                             // mov r, r/m
                      let v = if let Some(rr) = mm.rm_reg { em.r[rr] } else { em.load(mm.ea?, if w { 8 } else { 4 }) };
                      em.r[mm.reg] = v; ip = q + 1 + mm.len; }
            0x8d => { let mm = modrm(&em, q + 1, rex)?; em.r[mm.reg] = Some(mm.ea?); ip = q + 1 + mm.len; }   // lea
            // ★0x01 = `add r/m64, r64` (0x03 의 방향 반대판). mod=3 만 지원하면 충분하다(RE 2026-09-07 실측).
            0x01 => { let mm = modrm(&em, q + 1, rex)?; let rr = mm.rm_reg?;
                      em.r[rr] = match (em.r[rr], em.r[mm.reg]) { (Some(x), Some(y)) => Some(if w { x.wrapping_add(y) } else { x.wrapping_add(y) & 0xffff_ffff }), _ => None };
                      ip = q + 1 + mm.len; }
            0x03 => { let mm = modrm(&em, q + 1, rex)?;                                             // add r, r/m
                      let b = if let Some(rr) = mm.rm_reg { em.r[rr] } else { em.load(mm.ea?, if w { 8 } else { 4 }) };
                      em.r[mm.reg] = match (em.r[mm.reg], b) { (Some(x), Some(y)) => Some(if w { x.wrapping_add(y) } else { x.wrapping_add(y) & 0xffff_ffff }), _ => None };
                      ip = q + 1 + mm.len; }
            0x31 | 0x33 => { let mm = modrm(&em, q + 1, rex)?; let rr = mm.rm_reg?;                 // xor r, r
                             em.r[mm.reg] = if rr == mm.reg { Some(0) } else { None };
                             ip = q + 1 + mm.len; }
            0xb8..=0xbf => { let rr = ((((rex & 1) << 3) as u8) | (op - 0xb8)) as usize;
                             if w { em.r[rr] = Some(rd_u64(q + 1)?); ip = q + 9; }
                             else { em.r[rr] = Some(rd_u32(q + 1) as u64); ip = q + 5; } }
            0xc6 | 0xc7 => { let mm = modrm(&em, q + 1, rex)?;                                      // mov r/m, imm
                             let (imm, l, sz) = if op == 0xc6 { (rd_u8(q + 1 + mm.len) as u64, 1usize, 1usize) }
                                                else { (rd_i32(q + 1 + mm.len)? as i64 as u64, 4usize, if w { 8 } else { 4 }) };
                             if let Some(rr) = mm.rm_reg { em.r[rr] = Some(imm); } else { em.store(mm.ea?, Some(imm), sz)?; }
                             ip = q + 1 + mm.len + l; }
            0xc1 => { let mm = modrm(&em, q + 1, rex)?; let rr = mm.rm_reg?;                        // shr/shl/sar r, imm8
                      let sh = rd_u8(q + 1 + mm.len) as u32; let ext = (rd_u8(q + 1) >> 3) & 7; let v = em.r[rr]?;
                      em.r[rr] = Some(match ext { 5 => v >> sh, 4 => v << sh, 7 => ((v as i64) >> sh) as u64, _ => return None });
                      ip = q + 1 + mm.len + 1; }
            0xf7 => { let mm = modrm(&em, q + 1, rex)?; let ext = (rd_u8(q + 1) >> 3) & 7;          // mul r/m64
                      if ext != 4 || !w { return None; }
                      let b = if let Some(rr) = mm.rm_reg { em.r[rr]? } else { em.load(mm.ea?, 8)? };
                      let p = (em.r[0]? as u128).wrapping_mul(b as u128);
                      em.r[0] = Some(p as u64); em.r[2] = Some((p >> 64) as u64);
                      ip = q + 1 + mm.len; }
            0x0f => {
                let op2 = rd_u8(q + 1);
                match op2 {
                    0xaf => { let mm = modrm(&em, q + 2, rex)?;                                     // imul r, r/m
                              let b = if let Some(rr) = mm.rm_reg { em.r[rr] } else { em.load(mm.ea?, if w { 8 } else { 4 }) };
                              em.r[mm.reg] = match (em.r[mm.reg], b) { (Some(x), Some(y)) => Some(x.wrapping_mul(y)), _ => None };
                              ip = q + 2 + mm.len; }
                    0x57 => { let mm = modrm(&em, q + 2, rex)?;                                     // xorps xmm,xmm
                              if mm.rm_reg != Some(mm.reg) { return None; }
                              em.xmm0_zero = true; ip = q + 2 + mm.len; }
                    0x11 => { let mm = modrm(&em, q + 2, rex)?;                                     // movups m128, xmm
                              if !em.xmm0_zero { return None; }
                              let a = mm.ea?; em.store(a, Some(0), 8)?; em.store(a.wrapping_add(8), Some(0), 8)?;
                              ip = q + 2 + mm.len; }
                    _ => return None,
                }
            }
            0xff => { let mm = modrm(&em, q + 1, rex)?; let ext = (rd_u8(q + 1) >> 3) & 7;          // call r/m64
                      if ext != 2 { return None; }
                      // ★mod=3 = `call <reg>` — 컴파일러가 EST 접근자를 비휘발 레지스터에 캐시해 2회 부르는 형
                      let tgt = match mm.rm_reg { Some(rr) => em.r[rr]?, None => em.load(mm.ea?, 8)? };
                      est_call(&mut em, tgt)?;
                      ip = q + 1 + mm.len; }
            0xc3 => return Some(em),
            _ => return None,
        }
    }
}

/// EST 서술자(`0x1433da3d0`)가 들고 있는 접근자들. 잎이 부르는 것만 처리한다.
unsafe fn est_call(em: &mut Em, tgt: u64) -> Option<()> {
    let b = crate::exe_base() as u64; if b == 0 || tgt <= b { return None; }
    let (rcx, rdx) = (em.r[1], em.r[2]);
    let ret: Option<u64> = match tgt - b {
        // ★★self 는 **rcx** 다 — sret 판(0xc8c8a0)만 rcx=dst·rdx=self 이고 나머지 6종은 `mov rax,[rcx+…]` 형이다.
        //   ~~전부 rdx~~ 로 읽던 것은 `+0xa0` 세계에선 sret 판만 불려 안 터진 **잠복 버그**였고,
        //   `+0x28` 잎(`mov rcx,r8; call [r9+0x38]`)에 그대로 쓰면 전부 틀린다(RE 2026-09-07).
        0xc8c860 => Some(rd_u64(rcx? as usize + 0x660)?),        // x
        0xc8c870 => Some(rd_u64(rcx? as usize + 0x668)?),        // y
        0xc8c880 => Some(rd_u64(rcx? as usize + 0x670)?),        // hp
        0xc8c890 => Some(rd_u64(rcx? as usize + 0x5c8)?),        // level
        0xc8c8e0 => Some(rcx? + 0x618),                          // &스탯블록
        0xc8c850 => Some(rcx? + 0x370),
        // 스탯 스냅샷 복사: sret(rcx) ← self(rdx).0x618.. (9워드)
        0xc8c8a0 => {
            let (dst, src) = (rcx?, rdx? as usize);
            for i in 0..9usize { let v = rd_u64(src + 0x618 + i * 8)?; em.store(dst.wrapping_add((i * 8) as u64), Some(v), 8)?; }
            Some(dst)
        }
        _ => return None,
    };
    // 볼라틸 레지스터는 호출로 파괴된다
    for i in [1usize, 2, 8, 9, 10, 11] { em.r[i] = None; }
    em.r[0] = ret;
    Some(())
}
