#![allow(unused, dead_code, non_snake_case)]
//! C6 프로브 ③ — ⓐ`offset_of!` 보강분: **튜플 필드**(mf_swap.0/.1)와 중첩(team_plan.objective).
//! `(u8, usize)` 튜플은 rustc 가 **필드를 재배치할 수 있어** `.0` 이 +0 이라는 보장이 없다 —
//! 명세가 `.0 = +0x1610` · `.1 = +0x1618` 이라고 적었으므로 이걸 실행으로 확인한다.
use game_ai::plan_legacy::handler::LegacyPlanHandler;
use std::mem::offset_of;

static mut BAD: usize = 0;
macro_rules! chk {
    ($base:literal, $exp:expr, $name:literal, $got:expr) => {{
        let e: usize = $exp;
        let g: usize = $got;
        println!("OFF\t{}\t0x{:x}\t{}\t{}\t{}\t{}", $base, e, $name, e, g,
                 if e == g { "OK" } else { "MISMATCH" });
        if e != g { unsafe { BAD += 1; } }
    }};
}

fn main() {
    chk!("LegacyPlanHandler", 0x1610, "mf_swap.0 (u8, src 코드)", offset_of!(LegacyPlanHandler, mf_swap.0));
    chk!("LegacyPlanHandler", 0x1618, "mf_swap.1 (usize, tick)", offset_of!(LegacyPlanHandler, mf_swap.1));
    chk!("LegacyPlanHandler", 0x517, "team_plan.objective(LegacyPlanHandler 기준 절대)",
         offset_of!(LegacyPlanHandler, team_plan.objective));
    println!("\nTOTAL_MISMATCH\t{}", unsafe { BAD });
}
