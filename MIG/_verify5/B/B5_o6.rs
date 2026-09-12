#![allow(unused, dead_code, non_snake_case)]
//! B5_o6 — 05 보조: `LegacyPlanHandler` 실체 확인 (크기 · None 센티널 −1 · 초기 상태)
//! `LegacyPlanHandler::new` 는 pub 이므로 구조체를 실제로 만들어 바이트를 읽는다.
//! 검증 대상: 명세 05 의 `consts[-1]`(v50_dive_ep_live 판별자 None = −1) · `mem` 오프셋 3종의 초기값 ·
//!            구조체 크기 6168B.
use game_core::*;
use game_ai::plan_legacy::handler::LegacyPlanHandler;
use rand::SeedableRng;

fn main() {
    println!("size_of::<LegacyPlanHandler>()\t{}\texp=6168", std::mem::size_of::<LegacyPlanHandler>());
    let mut rnd = rand::rngs::StdRng::seed_from_u64(1);
    let h = LegacyPlanHandler::new(3usize, &mut rnd, 60usize, Position::Top);
    let p = &h as *const LegacyPlanHandler as *const u8;
    let rd64 = |off: usize| -> i64 { unsafe { std::ptr::read_unaligned(p.add(off) as *const i64) } };
    let rd8  = |off: usize| -> u8  { unsafe { std::ptr::read_unaligned(p.add(off)) } };
    println!("#OFF\toffset\tname\tvalue");
    println!("OFF\t0x570\tv50_dive_ep_live 판별자\t{}\t(-1 = None 기대)", rd64(0x570));
    println!("OFF\t0x1480\tlast_dive_abandon_tick\t{}", rd64(0x1480));
    println!("OFF\t0x1811\tv50_dive_ep_abort_src\t{}", rd8(0x1811));
    println!("OFF\t0x888\tv50_dive_episodes.cap\t{}", rd64(0x888));
    println!("OFF\t0x898\tv50_dive_episodes.len\t{}", rd64(0x898));
    println!("OFF\t0x5e8\tplan 태그(BigPlan)\t{}", rd64(0x5e8));
    println!("VERDICT\tnone_sentinel_is_minus1={}", rd64(0x570) == -1);
}
