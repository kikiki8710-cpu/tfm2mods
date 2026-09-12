#![allow(unused, dead_code, non_snake_case)]
//! B6_o1 — ⓐ `std::mem::offset_of!` **일괄 대조** (5차 배치A 가 발명, 배치B 는 이번이 처음 = `inherited`)
//!
//! 목적: `specs[5]`~`specs[9]` 의 `mem` 표 전량을 **한 프로세스에서** 런타임 오프셋과 대조한다.
//! 출력 = TSV. `MISMATCH` 가 하나라도 있으면 명세가 틀린 것이다.
//!
//! ⚠오프셋의 **정본은 tcx**(METHOD_MAP §5)이므로 이 실행은 등급을 올리는 게 아니라
//!   tcx 와 **같은 등급의 교차검증**이다(→ `mem` 행은 ev3 이 상한).
use game_core::*;
use std::mem::{offset_of, size_of};

macro_rules! chk {
    ($spec:expr, $row:expr, $exp:expr, $t:ty, $($f:tt)+) => {{
        let got = offset_of!($t, $($f)+);
        let exp: usize = $exp;
        println!("OFF\t{}\t{}\t{}\t{}\t0x{:x}\t0x{:x}\t{}",
            $spec, $row, stringify!($t), stringify!($($f)+), got, exp,
            if got == exp { "OK" } else { "MISMATCH" });
    }};
}
macro_rules! sz {
    ($t:ty, $exp:expr) => {{
        let got = size_of::<$t>();
        let exp: usize = $exp;
        println!("SIZE\t{}\t{}\t{}\t{}", stringify!($t), got, exp,
            if got == exp { "OK" } else { "MISMATCH" });
    }};
}

/// 두 Debug 출력에서 **바뀐 `이름: 값` 조각**을 이름만 뽑아 이어 붙인다.
/// 하나만 나오면 그 오프셋이 그 필드 전용이라는 뜻이다.
fn diff_fields(a: &str, b: &str) -> String {
    let split = |s: &str| -> Vec<String> {
        s.split(", ").map(|x| x.to_string()).collect()
    };
    let (va, vb) = (split(a), split(b));
    let mut out = vec![];
    for (x, y) in va.iter().zip(vb.iter()) {
        if x != y {
            out.push(format!("{} => {}", x.chars().take(60).collect::<String>(),
                                          y.chars().take(60).collect::<String>()));
        }
    }
    if va.len() != vb.len() { out.push(format!("(조각수 {}!={})", va.len(), vb.len())); }
    format!("[{}] {}", out.len(), out.join(" | "))
}

fn main() {
    println!("#B6_o1 ⓐ offset_of! 일괄 대조 — specs[5..9] mem 전량");
    println!("#spec\trow\ttype\tfield\tgot\texpect\tverdict");

    // ── 공통(06·07·08·09) ───────────────────────────────────────────
    chk!("06", "mem[0]", 0x0, OperationData, cache);
    chk!("06", "mem[1]", 0x8, OperationData, context);
    chk!("06", "mem[2]", 0x10, OperationData, blackboard);
    chk!("06", "mem[3]", 0x930, PlayerState, info.team);
    chk!("06", "mem[4]", 0x9c0, PlayerState, info.position);
    chk!("06", "mem[5]", 0x0, AbstractGameWithCache, game);
    chk!("06", "mem[6]", 0x1e0, AbstractGameWithCache, player_champion);
    chk!("06", "mem[7]", 0x660, Entity, x);
    chk!("06", "mem[8]", 0x668, Entity, y);
    chk!("06", "mem[9]", 0x5c0, Entity, id);
    chk!("06", "mem[10]", 0x0, GameContext, pool);
    chk!("06", "mem[11]", 0x8, GameContext, setting);
    chk!("06", "mem[12]", 0x12f8, GameSetting, tick_per_second);

    // ── 07 ─────────────────────────────────────────────────────────
    chk!("07", "mem[0]", 0x930, PlayerState, info.team);
    chk!("07", "mem[1]", 0x9c0, PlayerState, info.position);
    chk!("07", "mem[2]", 0x0, OperationData, cache);
    chk!("07", "mem[3]", 0x8, OperationData, context);
    chk!("07", "mem[4]", 0x0, AbstractGameWithCache, game);
    chk!("07", "mem[5]", 0x1e0, AbstractGameWithCache, player_champion);
    chk!("07", "mem[10]", 0x8, GameContext, setting);
    chk!("07", "mem[11]", 0x20, GameContext, map);
    chk!("07", "mem[12]", 0x12f8, GameSetting, tick_per_second);
    chk!("07", "mem[13]", 0x6d70, MapDef, fountains);
    chk!("07", "mem[14]", 0x670, Entity, hp);
    chk!("07", "mem[15]", 0x628, Entity, stat_cached.hp);
    chk!("07", "mem[16]", 0x660, Entity, x);
    chk!("07", "mem[17]", 0x668, Entity, y);

    // ── 08 ─────────────────────────────────────────────────────────
    chk!("08", "mem[2]", 0x930, PlayerState, info.team);
    chk!("08", "mem[3]", 0x0, OperationData, cache);
    chk!("08", "mem[4]", 0x8, OperationData, context);
    chk!("08", "mem[5]", 0x10, OperationData, blackboard);
    chk!("08", "mem[6]", 0x8, GameContext, setting);
    chk!("08", "mem[7]", 0x20, GameContext, map);
    chk!("08", "mem[8]", 0x12f8, GameSetting, tick_per_second);
    chk!("08", "mem[9]", 0x0, AbstractGameWithCache, game);
    chk!("08", "mem[11]", 0x1e0, AbstractGameWithCache, player_champion);
    chk!("08", "mem[19]", 0x660, Entity, x);
    chk!("08", "mem[20]", 0x668, Entity, y);

    // ── 09 ─────────────────────────────────────────────────────────
    chk!("09", "mem[0]", 0x930, PlayerState, info.team);
    chk!("09", "mem[1]", 0x9c0, PlayerState, info.position);
    chk!("09", "mem[2]", 0x0, OperationData, cache);
    chk!("09", "mem[3]", 0x8, OperationData, context);
    chk!("09", "mem[4]", 0x1e0, AbstractGameWithCache, player_champion);
    chk!("09", "mem[5]", 0x8, GameContext, setting);
    chk!("09", "mem[6]", 0x20, GameContext, map);
    chk!("09", "mem[7]", 0x12f8, GameSetting, tick_per_second);
    chk!("09", "mem[8]", 0x6d70, MapDef, fountains);
    chk!("09", "mem[9]", 0x5c0, Entity, id);
    chk!("09", "mem[10]", 0x628, Entity, stat_cached.hp);
    chk!("09", "mem[11]", 0x660, Entity, x);
    chk!("09", "mem[12]", 0x668, Entity, y);
    chk!("09", "mem[13]", 0x670, Entity, hp);

    // ── 05 · game_ai 쪽 ────────────────────────────────────────────
    use game_ai::plan_legacy::handler::LegacyPlanHandler;
    // mem[0] v50_dive_ep_live · mem[21] v50_dive_ep_abort_src 는 **private 필드**라 offset_of! 불가(E0616).
    //   ⟹ ⓑ derive(Debug) 관통 + 센티널 주입으로 아래 #DBGOFF 에서 오프셋↔이름을 확정한다.
    chk!("05", "mem[19]", 0x5e8, LegacyPlanHandler, plan);
    chk!("05", "mem[20]", 0x1480, LegacyPlanHandler, last_dive_abandon_tick);
    chk!("05", "mem[22]", 0x888, LegacyPlanHandler, v50_dive_episodes);
    chk!("07", "mem[18]", 0x98, game_ai::GoalData, epic.epic_ally_tick);
    chk!("07", "mem[19]", 0xa0, game_ai::GoalData, epic.epic_ally_killed_tick);

    // ── 05 레코드 타입(V50DiveEpisode) — logic 의 22필드 대응표 ──────
    chk!("05", "rec+0x00", 0x00, V50DiveEpisode, max_catch_break);
    chk!("05", "rec+0x08", 0x08, V50DiveEpisode, uncatch_total);
    chk!("05", "rec+0x10", 0x10, V50DiveEpisode, start_tick);
    chk!("05", "rec+0x18", 0x18, V50DiveEpisode, end_tick);
    chk!("05", "rec+0x20", 0x20, V50DiveEpisode, ep_ticks);
    chk!("05", "rec+0x28", 0x28, V50DiveEpisode, in_range_ticks);
    chk!("05", "rec+0x30", 0x30, V50DiveEpisode, holder_ticks);
    chk!("05", "rec+0x38", 0x38, V50DiveEpisode, team_holder_ticks);
    chk!("05", "rec+0x40", 0x40, V50DiveEpisode, minion_cover_ticks);
    chk!("05", "rec+0x48", 0x48, V50DiveEpisode, soaked_hp);
    chk!("05", "rec+0x50", 0x50, V50DiveEpisode, start_race_adv);
    chk!("05", "rec+0x54", 0x54, V50DiveEpisode, target_pos);
    chk!("05", "rec+0x58", 0x58, V50DiveEpisode, abort_src);
    chk!("05", "rec+0x59", 0x59, V50DiveEpisode, start_model);
    chk!("05", "rec+0x5a", 0x5a, V50DiveEpisode, start_na);
    chk!("05", "rec+0x5b", 0x5b, V50DiveEpisode, start_ne);
    chk!("05", "rec+0x5c", 0x5c, V50DiveEpisode, start_tgt_hp);
    chk!("05", "rec+0x5d", 0x5d, V50DiveEpisode, end_reason);
    chk!("05", "rec+0x5e", 0x5e, V50DiveEpisode, end_plan);
    chk!("05", "rec+0x5f", 0x5f, V50DiveEpisode, tower);
    chk!("05", "rec+0x60", 0x60, V50DiveEpisode, start_in_range);
    chk!("05", "rec+0x61", 0x61, V50DiveEpisode, aborted);

    // ── ⓑ derive(Debug) 관통 + 센티널 주입 — private 필드의 오프셋↔이름 확정 ──
    //   offset_of! 가 막힌 2행(05 mem[0] 0x570 · mem[21] 0x1811)을 실행으로 잡는다.
    //   ⚠1차 시도는 0x570(Option 판별자)에 0 을 써서 **세그폴트**했다 — None 일 때 페이로드
    //     120B 가 미초기화라 `Position` 등 열거형 필드의 판별자가 쓰레기가 되고 Debug 가 그걸 매치한다.
    //     ⟹ 판별자 슬롯은 **쓰지 말고 읽는다**. 스칼라(u8) 필드만 센티널을 주입한다.
    {
        use rand::SeedableRng;
        let mut rnd = rand::rngs::StdRng::seed_from_u64(1);
        let base = LegacyPlanHandler::new(3usize, &mut rnd, 60usize, Position::Top);
        let p = &base as *const LegacyPlanHandler as *const u8;
        let raw570: i64 = unsafe { std::ptr::read_unaligned(p.add(0x570) as *const i64) };
        let raw1811: u8 = unsafe { std::ptr::read_unaligned(p.add(0x1811)) };
        let d0 = format!("{:?}", base);
        println!("#DBGOFF");
        println!("DBG	0x570	raw_i64	{}	exp=-1(None)	{}", raw570,
                 if raw570 == -1 { "OK" } else { "MISMATCH" });
        println!("DBG	0x570	debug_says_None	{}", d0.contains("v50_dive_ep_live: None"));
        println!("DBG	0x1811	raw_u8	{}	debug_says_same	{}", raw1811,
                 d0.contains(&format!("v50_dive_ep_abort_src: {}", raw1811)));
        // 센티널 주입 — 0x1811 만 바뀌는지 Debug 로 확인(스칼라라 UB 없음)
        let mut h2 = base.clone();
        unsafe { std::ptr::write_unaligned((&mut h2 as *mut _ as *mut u8).add(0x1811), 0xABu8); }
        let d2 = format!("{:?}", h2);
        println!("DBG	0x1811	sentinel_171	{}", d2.contains("v50_dive_ep_abort_src: 171"));
        println!("DBG	0x1811	changed_fields	{}", diff_fields(&d0, &d2));
    }

    println!("#SIZE");
    sz!(LegacyPlanHandler, 6168);
    sz!(V50DiveEpisode, 104);
    sz!(OperationData, 24);
    sz!(GameContext, 64);
    sz!(PlayerState, 2528);
    sz!(Entity, 1728);
    sz!(AbstractGameWithCache, 8840);
}
