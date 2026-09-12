#![allow(unused, dead_code, non_snake_case)]
//! A5-O7 — 남은 `ev<=3` 표본 재확인:
//!  · specs[0] mem[27] `Option<Input>` +0x8 = Input::Ult.target(24B) / consts[8] 태그 5
//!  · specs[3] knobs[19] `AthleteStat.judgement` = PlayerState+0x218
//!    specs[3] knobs[20] `AthleteParameter.judgement_mental_ratio` = PlayerState+0x450 (초기 1000)
//!  · specs[4] mem[7] `AbstractGame vtable +0x40 = get_game_mode` — 런타임 슬롯 호출 대조
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

fn main() {
    // ── Option<Input> 레이아웃 ──
    let ult = Some(Input::Ult { target: InputTarget::Pos { x: 0x1111_2222_3333_4444, y: 0x5555_6666_7777_8888 } });
    let p = &ult as *const Option<Input> as *const u8;
    unsafe {
        let tag = std::ptr::read_unaligned(p as *const i64);
        println!("OPT_INPUT\tSome(Ult) tag@+0x0={}\t(명세 5)", tag);
        let it: InputTarget = std::ptr::read_unaligned(p.add(8) as *const InputTarget);
        println!("OPT_INPUT\tpayload@+0x8 as InputTarget = {:?}\t(명세: Input::Ult.target 24B @ +0x8)", it);
    }
    let none: Option<Input> = None;
    unsafe {
        println!("OPT_INPUT\tNone tag@+0x0={}\t(명세 -1)",
                 std::ptr::read_unaligned(&none as *const Option<Input> as *const i64));
    }
    for (nm, v) in [("Move", Input::Move { x: 1, y: 2 }), ("Return", Input::Return),
                    ("Attack", Input::Attack { target: InputTarget::None }),
                    ("Skill", Input::Skill { target: InputTarget::None }),
                    ("Skill2", Input::Skill2 { target: InputTarget::None }),
                    ("Ult", Input::Ult { target: InputTarget::None })] {
        let o = Some(v);
        unsafe {
            println!("INPUT_TAG\t{}\t{}", nm,
                     std::ptr::read_unaligned(&o as *const Option<Input> as *const i64));
        }
    }

    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off,
    };
    let game = mkgame(&setting, &ms, &map, &ctx);

    // ── PlayerState 오프셋 실측 (mkgame 은 judgement=80, mental=60 을 넣는다) ──
    let ps: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
    let b = ps as *const PlayerState as usize;
    unsafe {
        println!("PS\t+0x218={}\t(명세 AthleteStat.judgement, mkgame=80)", std::ptr::read_unaligned((b + 0x218) as *const usize));
        println!("PS\t+0x450={}\t(명세 judgement_mental_ratio 초기 1000)", std::ptr::read_unaligned((b + 0x450) as *const usize));
        println!("PS\t+0x180(=AthleteParameter 시작 추정)\t{}", std::ptr::read_unaligned((b + 0x180) as *const usize));
    }

    // ── bumpalo Vec(32B) 내부 len 오프셋 실측: twin_towers[0] 은 len==2 여야 한다 ──
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let v = &cache.twin_towers[0];
        let raw = v as *const _ as *const u64;
        let words: [u64; 4] = unsafe { [*raw, *raw.add(1), *raw.add(2), *raw.add(3)] };
        let n = v.len() as u64;
        let hit: Vec<usize> = (0..4).filter(|i| words[*i] == n).collect();
        println!("BUMPVEC	len()={}	words(+0x0,+0x8,+0x10,+0x18)={:x?}	len 과 일치하는 워드 index={:?}	(명세 len@+0x18 ⟹ index 3)",
                 n, words, hit);
        let buf = words[0] as *const *const Entity;
        let e0 = unsafe { *buf };
        println!("BUMPVEC	ptr@+0x0=0x{:x}	*(ptr)={:p}	원소0 주소={:p}	verdict={}",
                 words[0], e0, v[0] as *const Entity,
                 if e0 == (v[0] as *const Entity) { "ptr@+0x0 = 버퍼 기점 CONFIRMED" } else { "**MISMATCH**" });
    }

    // ── AbstractGame vtable +0x40 슬롯을 직접 호출해 get_game_mode 인지 확인 ──
    let dg: &dyn AbstractGame = &game as &dyn AbstractGame;
    let (dataptr, vt): (*const (), *const usize) = unsafe { std::mem::transmute(dg) };
    unsafe {
        println!("VT\tvtable[0]=drop 0x{:x}\tsize={}\talign={}",
                 *vt.add(0), *vt.add(1), *vt.add(2));
        type F = unsafe extern "Rust" fn(*const ()) -> GameMode<'static>;
        let slot = *vt.add(0x40 / 8);
        let f: F = std::mem::transmute(slot);
        let gm = f(dataptr);
        let via_slot = gm.as_moba().map(|m| m as *const MobaMode);
        let via_trait = AbstractGame::get_game_mode(dg).as_moba().map(|m| m as *const MobaMode);
        let direct = &game.mode as *const MobaMode;
        println!("VT\t+0x40 slot 호출 as_moba()={:?}\ttrait 호출={:?}\t&game.mode={:?}\tverdict={}",
                 via_slot, via_trait, direct,
                 if via_slot == via_trait && via_slot == Some(direct) { "get_game_mode CONFIRMED" } else { "**MISMATCH**" });
        // remain_epic_time 직독 확인 (명세: 시간 산술 없이 배열 직독)
        let m = AbstractGame::get_game_mode(dg).as_moba().unwrap();
        println!("MOBA\tepic_minion_buff_time={:?}\tremain_epic_time(0)={}\tremain_epic_time(1)={}",
                 m.epic_minion_buff_time, m.remain_epic_time(0), m.remain_epic_time(1));
    }
}
