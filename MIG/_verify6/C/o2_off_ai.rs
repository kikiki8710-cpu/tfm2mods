#![allow(unused, dead_code, non_snake_case)]
//! C6 프로브 ②  ⓐ`offset_of!` — **game_ai 쪽 타입**(필드 private 가능성이 있어 따로 뗐다)
//! 컴파일이 안 되면 그 행은 「private 이라 offset_of 불가」로 기록하고 tcx 로 남긴다.
use game_ai::plan_legacy::handler::LegacyPlanHandler;
use game_ai::plan_legacy::old::{LineGankCoverPlan, LineGankerPlan};
use std::mem::{offset_of, size_of};

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
macro_rules! sz {
    ($t:ty, $exp:expr) => {{
        let e: usize = $exp;
        let g: usize = size_of::<$t>();
        println!("SZ\t{}\t{}\t{}\t{}", stringify!($t), e, g,
                 if e == g { "OK" } else { "MISMATCH" });
        if e != g { unsafe { BAD += 1; } }
    }};
}

fn main() {
    sz!(LegacyPlanHandler, 6168);
    sz!(LineGankerPlan, 48);
    sz!(LineGankCoverPlan, 40);

    // specs[14] — LineGankerPlan
    chk!("LineGankerPlan", 0x0, "chats", offset_of!(LineGankerPlan, chats));
    // ⛔ setup_limit: **private 필드** — offset_of 불가(E0616). tcx 정본 유지.
    // ⛔ wait_limit: private 필드 — offset_of 불가(E0616).
    chk!("LineGankerPlan", 0x28, "line", offset_of!(LineGankerPlan, line));
    chk!("LineGankerPlan", 0x29, "phase", offset_of!(LineGankerPlan, phase));

    // specs[13] — LineGankCoverPlan (명세 sig.params 가 self.line = +0x20 이라고 적었다)
    // ⛔ LineGankCoverPlan::line: private 필드 — offset_of 불가(E0616).

    // specs[11]/[12] — LegacyPlanHandler
    chk!("LegacyPlanHandler", 0x0, "data", offset_of!(LegacyPlanHandler, data));
    chk!("LegacyPlanHandler", 0xf8, "team_plan", offset_of!(LegacyPlanHandler, team_plan));
    chk!("LegacyPlanHandler", 0x5e8, "plan", offset_of!(LegacyPlanHandler, plan));
    chk!("LegacyPlanHandler", 0x768, "sub_plan", offset_of!(LegacyPlanHandler, sub_plan));
    chk!("LegacyPlanHandler", 0x858, "pending_trace_events", offset_of!(LegacyPlanHandler, pending_trace_events));
    chk!("LegacyPlanHandler", 0x990, "positioning_score", offset_of!(LegacyPlanHandler, positioning_score));
    chk!("LegacyPlanHandler", 0x1610, "mf_swap", offset_of!(LegacyPlanHandler, mf_swap));
    chk!("LegacyPlanHandler", 0x1628, "v3_lapse_passive_fallbacks",
         offset_of!(LegacyPlanHandler, v3_lapse_passive_fallbacks));
    chk!("LegacyPlanHandler", 0x530, "pending_global_ult_target",
         offset_of!(LegacyPlanHandler, pending_global_ult_target));

    println!("\nTOTAL_MISMATCH\t{}", unsafe { BAD });
}
