//! `0xe7a8c0` — 이름은 `ability_pick` 이지만 실제 동작은 **"보유 아이템에서 승급 가능한 다음 아이템
//! 후보를 모아 균등 랜덤으로 1개 뽑기"** 다(RE 2026-09-07, `RE\2026-09-07_ability_pick-0xe7a8c0-전수해독`).
//!
//! ★소비자 `hunt_poke`/`hunt_battle` 은 반환 `Option` 의 **태그만** 본다(`0xccc1b1 cmp qword [rsp+0x70],0`).
//!   그리고 게임은 **후보가 0개면 RNG 을 전혀 건드리지 않는다**(`0xe7ab1a` 이전에 조기반환).
//!   ⟹ **`n > 0` 여부만 순수 재현하면** 두 소비자에게 필요한 정보가 전부이고 RNG 스트림도 건드리지 않는다.
//!   (뽑힌 값 `(a,b)` 까지 재현하려면 `RngSim` 으로 Lemire 거절샘플링을 비트동일하게 돌려야 한다 — §8 체크리스트.)
//!
//! 인자(래퍼 기준): `p1`=sret · `p2`=미사용 · `p3`=RNG · **`p4`=선수 sim** · `p5`/`p6`=미사용 · **`p7`=G**
//!
//! 계약:
//! ```text
//! own = p4.0x4a0[0 .. p4.0x4a8]        // stride 0x10 = (data, vt) = Box<dyn SettingItem>
//! db  = *(G+0x30)                       // Vec<Box<dyn SettingItem>> {cap@0, ptr@8, len@0x10}
//! max_tier = own.filter(|o| o.vt70() <= 3).map(vt70).max().unwrap_or(0)
//! n = Σ over (own i, name in own[i].vt80()) of
//!       let idx = db.position(|c| c.vt58() == name)?;
//!       c.vt50() && c.vt70() > max_tier && c.vt68() <= p4.0x998
//! tag = (n != 0) as u64
//! ```
//! ⚠Arc 페이로드 보정 **없음** — `data` 를 그대로 `rcx` 로 넘긴다(`0xe7a93d`, `0xe7aab7`).

use crate::{rd_u64, rd_u8, ptr_ok};
use std::cell::RefCell;

/// 이름 문자열(`&str`) 을 DB 인덱스로. 게임은 TLS HashMap 캐시를 쓰지만(성능 전용) 여기서는
/// **(db_ptr, db_len) 세대 + 이름 포인터** 로 키를 잡은 작은 메모로 대체한다 — 결과는 동일하다.
thread_local! {
    static NAME_MEMO: RefCell<(usize, u64, Vec<((usize, u64), Option<u64>)>)> =
        const { RefCell::new((0, 0, Vec::new())) };
}

/// `vt+slot` 이 돌려주는 u64(단순 게터). obj = **data 원본**.
#[inline]
unsafe fn g_u64(vt: usize, slot: usize, obj: usize) -> Option<u64> {
    let f = rd_u64(vt + slot)? as usize;
    match super::as_callees::decode_getter(f, obj) {
        Some(v) => Some(v),
        None => { super::dyn_eff::unseen(0x7000 | slot as u32,
                                         super::dyn_eff::impl_rva(vt, slot).unwrap_or(0)); None }
    }
}
/// `vt+slot` 이 돌려주는 **포인터**(`&Vec<..>` / `&String`). `lea`/`mov rax,rcx` 형만 처리.
#[inline]
unsafe fn g_ptr(vt: usize, slot: usize, obj: usize) -> Option<usize> {
    let f = rd_u64(vt + slot)? as usize; if !ptr_ok(f) { return None; }
    let (b0, b1, b2) = (rd_u8(f), rd_u8(f + 1), rd_u8(f + 2));
    // lea rax,[rcx+d32]; ret / lea rax,[rcx+d8]; ret / mov rax,rcx; ret
    if b0 == 0x48 && b1 == 0x8d && b2 == 0x81 && rd_u8(f + 7) == 0xc3 {
        return Some((obj as isize + crate::rd_i32(f + 3)? as isize) as usize);
    }
    if b0 == 0x48 && b1 == 0x8d && b2 == 0x41 && rd_u8(f + 4) == 0xc3 {
        return Some((obj as isize + rd_u8(f + 3) as i8 as isize) as usize);
    }
    if b0 == 0x48 && b1 == 0x89 && b2 == 0xc8 && rd_u8(f + 3) == 0xc3 { return Some(obj); }
    // mov rax,[rcx+d]; ret 형(간접 보관)도 허용
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x81 && rd_u8(f + 7) == 0xc3 {
        return rd_u64((obj as isize + crate::rd_i32(f + 3)? as isize) as usize).map(|v| v as usize);
    }
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x41 && rd_u8(f + 4) == 0xc3 {
        return rd_u64((obj as isize + rd_u8(f + 3) as i8 as isize) as usize).map(|v| v as usize);
    }
    super::dyn_eff::unseen(0x7100 | slot as u32, super::dyn_eff::impl_rva(vt, slot).unwrap_or(0));
    None
}
/// `String`/`Vec<u8>` 헤더 `{cap@0, ptr@8, len@0x10}` → 바이트 슬라이스 (ptr, len)
#[inline] unsafe fn str_of(p: usize) -> Option<(usize, u64)> { Some((rd_u64(p + 8)? as usize, rd_u64(p + 0x10)?)) }

#[inline]
unsafe fn bytes_eq(a: usize, b: usize, n: u64) -> Option<bool> {
    if n > 4096 { return None; }
    for i in 0..n as usize { if rd_u8(a + i) != rd_u8(b + i) { return Some(false); } }
    Some(true)
}

/// DB 에서 이름과 일치하는 첫 원소의 인덱스(`position`). 게임 `0x105fae0` 와 동치.
unsafe fn lookup(db_ptr: usize, db_len: u64, name: (usize, u64)) -> Option<Option<u64>> {
    // 세대 검사 후 메모 조회
    let hit = NAME_MEMO.with(|c| {
        let m = c.borrow();
        if m.0 == db_ptr && m.1 == db_len { m.2.iter().find(|(k, _)| *k == name).map(|(_, v)| *v) } else { None }
    });
    if let Some(v) = hit { return Some(v); }
    let mut found: Option<u64> = None;
    for j in 0..db_len.min(4096) {
        let e = db_ptr + j as usize * 0x10;
        let (cd, cv) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
        if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
        let sp = g_ptr(cv, 0x58, cd)?;
        let (bp, bl) = str_of(sp)?;
        if bl == name.1 && bytes_eq(bp, name.0, bl)? { found = Some(j); break; }
    }
    NAME_MEMO.with(|c| {
        let mut m = c.borrow_mut();
        if m.0 != db_ptr || m.1 != db_len { *m = (db_ptr, db_len, Vec::new()); }
        if m.2.len() < 4096 { m.2.push((name, found)); }
    });
    Some(found)
}


/// DIFF 로그용 — `tag()` 의 판단 재료를 전부 찍는다(보유 아이템·티어·후보별 가용/티어/가격).
pub unsafe fn diag(p5: usize, g: usize) -> String {
    let mut out = String::new();
    let f = || -> Option<String> {
        let own_len = rd_u64(p5 + 0x4a8)?; let own_ptr = rd_u64(p5 + 0x4a0)? as usize;
        let db = rd_u64(g + 0x30)? as usize; let (db_ptr, db_len) = (rd_u64(db + 8)? as usize, rd_u64(db + 0x10)?);
        let gold = rd_u64(p5 + 0x998)?;
        let mut s = format!("own={} gold={} db_len={} |", own_len, gold, db_len);
        let mut max_tier = 0u64;
        for i in 0..own_len.min(16) as usize {
            let e = own_ptr + i * 0x10; let (d, v) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
            let t = g_u64(v, 0x70, d)?; if t <= 3 && t > max_tier { max_tier = t; }
            s += &format!(" own{}:t{}", i, t);
        }
        s += &format!(" | maxT={} |", max_tier);
        for i in 0..own_len.min(16) as usize {
            let e = own_ptr + i * 0x10; let (d, v) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
            let names = g_ptr(v, 0x80, d)?; let (nptr, nlen) = (rd_u64(names + 8)? as usize, rd_u64(names + 0x10)?);
            for k in 0..nlen.min(8) as usize {
                let name = str_of(nptr + k * 0x18)?;
                let nm = if name.1 != 0 && name.1 < 64 { String::from_utf8_lossy(std::slice::from_raw_parts(name.0 as *const u8, name.1 as usize)).into_owned() } else { "?".into() };
                match lookup(db_ptr, db_len, name)? {
                    None => { s += &format!(" {}:nodb", nm); }
                    Some(idx) => {
                        let c = db_ptr + idx as usize * 0x10; let (cd, cv) = (rd_u64(c)? as usize, rd_u64(c + 8)? as usize);
                        let (av, ti, pr) = (g_u64(cv, 0x50, cd)? & 1, g_u64(cv, 0x70, cd)?, g_u64(cv, 0x68, cd)?);
                        s += &format!(" {}:av{} t{} p{}{}", nm, av, ti, pr, if av == 1 && ti > max_tier && pr <= gold { "*" } else { "" });
                    }
                }
            }
        }
        Some(s)
    };
    out += &f().unwrap_or_else(|| "NA".into());
    out
}

/// 반환 = `Option` 태그(0 = None · 1 = Some). None = 미재현(NA).
pub unsafe fn tag(p5: usize, g: usize) -> Option<u64> {
    if !ptr_ok(p5) || !ptr_ok(g) { return None; }
    let own_len = rd_u64(p5 + 0x4a8)?; if own_len > 4096 { return None; }
    let own_ptr = rd_u64(p5 + 0x4a0)? as usize;
    if own_len != 0 && !ptr_ok(own_ptr) { return None; }
    let db = rd_u64(g + 0x30)? as usize; if !ptr_ok(db) { return None; }
    let (db_ptr, db_len) = (rd_u64(db + 8)? as usize, rd_u64(db + 0x10)?);
    if db_len != 0 && !ptr_ok(db_ptr) { return None; }
    let gold = rd_u64(p5 + 0x998)?;

    // ── A. 보유 아이템 중 vt70() <= 3 인 것들의 최대값 (없으면 0)
    let mut max_tier: u64 = 0;
    for i in 0..own_len as usize {
        let e = own_ptr + i * 0x10;
        let (d, v) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
        if !ptr_ok(d) || !ptr_ok(v) { return None; }
        let t = g_u64(v, 0x70, d)?;
        if t <= 3 && t > max_tier { max_tier = t; }
    }

    // ── B. 후보 수집 (개수만 세면 된다)
    let mut n: u64 = 0;
    for i in 0..own_len as usize {
        let e = own_ptr + i * 0x10;
        let (d, v) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
        let names = g_ptr(v, 0x80, d)?;
        let (nptr, nlen) = (rd_u64(names + 8)? as usize, rd_u64(names + 0x10)?);
        if nlen > 4096 { return None; }
        if nlen != 0 && !ptr_ok(nptr) { return None; }
        for k in 0..nlen as usize {
            let name = str_of(nptr + k * 0x18)?;
            if name.1 != 0 && !ptr_ok(name.0) { return None; }
            let idx = match lookup(db_ptr, db_len, name)? { Some(x) => x, None => continue };
            if idx >= db_len { return None; }                    // 게임은 여기서 패닉
            let c = db_ptr + idx as usize * 0x10;
            let (cd, cv) = (rd_u64(c)? as usize, rd_u64(c + 8)? as usize);
            if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
            if g_u64(cv, 0x50, cd)? & 1 == 0 { continue; }        // 가용
            if g_u64(cv, 0x70, cd)? <= max_tier { continue; }     // 티어가 더 높아야
            if g_u64(cv, 0x68, cd)? > gold { continue; }          // 가격 ≤ 자원
            n += 1;
            if n > 0 { return Some(1); }                          // 태그만 필요하므로 조기 종료
        }
    }
    Some(0)
}
