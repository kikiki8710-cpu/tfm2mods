#![allow(unused, dead_code, non_snake_case)]
//! A6-O1 — 배치 A(`specs[0]`~`specs[4]`) **`mem` 전행 오프셋 일괄 대조**.
//! 수법 ⓐ(`std::mem::offset_of!` 일괄 대조) — 5차 배치A 가 쓰던 것 그대로(= `found_by: reused`).
//! 5차 대비 **확장분**:
//!   (N1) bumpalo `Vec` 의 `len` 이 `+0x10` 인지 `+0x18` 인지 — 5차는 `len==cap==2` 라 못 갈랐다.
//!        ⟹ `with_capacity_in(8)` + `push` 3회로 **len != cap** 을 만들어 가른다.
//!   (N2) `AttackNexusPlan`(16B) 의 `+0x0 team` / `+0x8 line` 을 바이트로 확인(수법 ⓒ).
//!   (N3) `MapDef.fountains[team]` 내부 (lx,ly,rx,ry) 의 튜플 오프셋.
//!   (N4) `Option<Input>` / `SubPlan` sret 레이아웃.
//!   (N5) `DefensiveCrisis` 두 bool 의 오프셋.
mod tmpl;
use game_core::*;
use std::mem::offset_of;
use tmpl::*;

static mut NOK: usize = 0;
static mut NNG: usize = 0;

fn chk(spec: &str, base: &str, name: &str, claim: usize, got: usize) {
    let ok = claim == got;
    unsafe { if ok { NOK += 1 } else { NNG += 1 } }
    println!("{}\t{}\t{}\tclaim=0x{:x}\tgot=0x{:x}\t{}", spec, base, name, claim, got,
             if ok { "MATCH" } else { "**MISMATCH**" });
}
fn chkv(spec: &str, base: &str, name: &str, claim: usize, got: usize) {
    let ok = claim == got;
    unsafe { if ok { NOK += 1 } else { NNG += 1 } }
    println!("{}\t{}\t{}\tclaim={}\tgot={}\t{}", spec, base, name, claim, got,
             if ok { "MATCH" } else { "**MISMATCH**" });
}

fn main() {
    println!("== A6-O1 mem 오프셋 일괄 대조 (수법 ⓐ) ==");

    // ───────── OperationData / GameContext / AbstractGameWithCache ─────────
    chk("0,1,2,3,4", "OperationData", "cache", 0x0, offset_of!(OperationData, cache));
    chk("0,1,2,3,4", "OperationData", "context", 0x8, offset_of!(OperationData, context));
    chk("3,4", "OperationData", "blackboard", 0x10, offset_of!(OperationData, blackboard));
    chk("3,4", "GameContext", "pool", 0x0, offset_of!(GameContext, pool));
    chk("0,3,4", "GameContext", "setting", 0x8, offset_of!(GameContext, setting));
    chk("0,2,4", "GameContext", "map", 0x20, offset_of!(GameContext, map));
    chk("4", "GameContext", "tutorial", 0x38, offset_of!(GameContext, tutorial));
    chk("3,4", "AbstractGameWithCache", "game", 0x0, offset_of!(AbstractGameWithCache, game));
    chk("2,4", "AbstractGameWithCache", "twin_towers", 0x130, offset_of!(AbstractGameWithCache, twin_towers));
    chk("4", "AbstractGameWithCache", "top_tower", 0x180, offset_of!(AbstractGameWithCache, top_tower));
    chk("4", "AbstractGameWithCache", "top_tower2", 0x190, offset_of!(AbstractGameWithCache, top_tower2));
    chk("4", "AbstractGameWithCache", "mid_tower", 0x1a0, offset_of!(AbstractGameWithCache, mid_tower));
    chk("4", "AbstractGameWithCache", "mid_tower2", 0x1b0, offset_of!(AbstractGameWithCache, mid_tower2));
    chk("4", "AbstractGameWithCache", "bottom_tower", 0x1c0, offset_of!(AbstractGameWithCache, bottom_tower));
    chk("4", "AbstractGameWithCache", "bottom_tower2", 0x1d0, offset_of!(AbstractGameWithCache, bottom_tower2));
    chk("0,1,2,3", "AbstractGameWithCache", "player_champion", 0x1e0, offset_of!(AbstractGameWithCache, player_champion));
    chk("3", "GameSetting", "tick_per_second", 0x12f8, offset_of!(GameSetting, tick_per_second));
    chk("2", "MapDef", "fountains", 0x6d70, offset_of!(MapDef, fountains));
    chk("4", "MobaMode", "epic_minion_buff_time", 0x240, offset_of!(MobaMode, epic_minion_buff_time));
    chk("4", "Strategy", "morgard_defense", 0xe, offset_of!(Strategy, morgard_defense));
    chk("4", "Blackboard", "top_minion_state", 0x0, offset_of!(Blackboard, top_minion_state));
    chk("4", "Blackboard", "mid_minion_state", 0x28, offset_of!(Blackboard, mid_minion_state));
    chk("4", "Blackboard", "bottom_minion_state", 0x50, offset_of!(Blackboard, bottom_minion_state));
    chk("4", "BrainMinionParameter", "from_mid", 0x10, offset_of!(BrainMinionParameter, from_mid));
    chk("4", "BrainMinionParameter", "minion_count", 0x20, offset_of!(BrainMinionParameter, minion_count));

    // ───────── Effect / Entity ─────────
    chk("0", "Effect", "ty", 0x0, offset_of!(Effect, ty));
    chk("0", "Effect", "range", 0x10, offset_of!(Effect, range));
    chk("0", "Effect", "growth_range", 0x18, offset_of!(Effect, growth_range));
    chk("0", "Effect", "target", 0x28, offset_of!(Effect, target));
    chk("0,3", "Effect", "casting", 0x30, offset_of!(Effect, casting));
    chk("0", "Entity", "team", 0x0, offset_of!(Entity, team));
    chk("0", "Entity", "visible_state", 0x38, offset_of!(Entity, visible_state));
    chk("1,3", "Entity", "ty", 0x68, offset_of!(Entity, ty));
    chk("0", "Entity", "stat_buff_cached.range", 0x438, offset_of!(Entity, stat_buff_cached.range));
    chk("0", "Entity", "stat_buff_cached.radius_mult", 0x470, offset_of!(Entity, stat_buff_cached.radius_mult));
    chk("3", "Entity", "skill_effect", 0x4c8, offset_of!(Entity, skill_effect));
    chk("3", "Entity", "skill2_effect", 0x500, offset_of!(Entity, skill2_effect));
    chk("0,3", "Entity", "ult_effect", 0x538, offset_of!(Entity, ult_effect));
    chk("3,4", "Entity", "id", 0x5c0, offset_of!(Entity, id));
    chk("0,3", "Entity", "level", 0x5c8, offset_of!(Entity, level));
    chk("2", "Entity", "stat_cached.hp", 0x628, offset_of!(Entity, stat_cached.hp));
    chk("0,2,4", "Entity", "x", 0x660, offset_of!(Entity, x));
    chk("0,2,4", "Entity", "y", 0x668, offset_of!(Entity, y));
    chk("1,2", "Entity", "hp", 0x670, offset_of!(Entity, hp));
    chk("0", "Entity", "radius", 0x680, offset_of!(Entity, radius));
    // skill_effect 안의 casting (spec3 mem[17]) = 0x4c8 + 0x30
    chk("3", "Entity", "skill_effect.casting(=+0x4c8+0x30)", 0x4f8,
        offset_of!(Entity, skill_effect) + offset_of!(Effect, casting));

    // ───────── 구조체 크기(명세가 괄호로 적은 값) ─────────
    chkv("-", "size", "Entity", 1728, std::mem::size_of::<Entity>());
    chkv("-", "size", "PlayerState", 2528, std::mem::size_of::<PlayerState>());
    chkv("-", "size", "OperationData", 24, std::mem::size_of::<OperationData>());
    chkv("-", "size", "GameContext", 64, std::mem::size_of::<GameContext>());
    chkv("-", "size", "AbstractGameWithCache", 8840, std::mem::size_of::<AbstractGameWithCache>());
    chkv("-", "size", "GameSetting", 5432, std::mem::size_of::<GameSetting>());
    chkv("-", "size", "MapDef", 28112, std::mem::size_of::<MapDef>());
    chkv("-", "size", "Effect", 56, std::mem::size_of::<Effect>());
    chkv("-", "size", "MobaMode", 640, std::mem::size_of::<MobaMode>());
    chkv("-", "size", "Strategy", 24, std::mem::size_of::<Strategy>());
    chkv("-", "size", "Blackboard", 744, std::mem::size_of::<Blackboard>());
    chkv("-", "size", "BrainMinionParameter", 40, std::mem::size_of::<BrainMinionParameter>());
    chkv("-", "size", "DefensiveCrisis", 2, std::mem::size_of::<game_ai::DefensiveCrisis>());
    chkv("-", "size", "Option<Input>", 32, std::mem::size_of::<Option<Input>>());
    chkv("-", "size", "InputTarget", 24, std::mem::size_of::<InputTarget>());
    chkv("-", "size", "SubPlan", 72, std::mem::size_of::<game_ai::plan_legacy::sub_plan::SubPlan>());
    chkv("-", "size", "Option<Effect>", 56, std::mem::size_of::<Option<Effect>>());
    chkv("-", "size", "AttackNexusPlan", 16,
         std::mem::size_of::<game_ai::plan_legacy::old::AttackNexusPlan>());
    chkv("-", "stride", "player_champion[team]", 40,
         std::mem::size_of::<[Option<&Entity>; 5]>());
    chkv("-", "stride", "twin_towers[team]", 32,
         std::mem::size_of::<bumpalo::collections::Vec<&Entity>>());
    chkv("-", "stride", "visible_state[i]", 24, std::mem::size_of::<VisibleState>());
    chkv("-", "stride", "fountains[team]", 32, std::mem::size_of::<(u64, u64, u64, u64)>());

    // ───────── PlayerState 중첩 (offset_of 가 안 되면 실주소 차) ─────────
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
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let ps = game.get_player_by_position(0, Position::Top).unwrap();
    let base = ps as *const PlayerState as usize;
    chk("0,1,2,3,4", "PlayerState", "info.team",
        0x930, (&ps.info.team as *const usize as usize) - base);
    chk("0,1,2", "PlayerState", "info.position",
        0x9c0, (&ps.info.position as *const Position as usize) - base);

    // ───────── (N3) MapDef.fountains[team] 내부 튜플 오프셋 ─────────
    {
        let f = &map.fountains[0];
        let fb = f as *const (u64, u64, u64, u64) as usize;
        chk("2", "MapDef.fountains[team]", "lx", 0x0, (&f.0 as *const u64 as usize) - fb);
        chk("2", "MapDef.fountains[team]", "ly", 0x8, (&f.1 as *const u64 as usize) - fb);
        chk("2", "MapDef.fountains[team]", "rx", 0x10, (&f.2 as *const u64 as usize) - fb);
        chk("2", "MapDef.fountains[team]", "ry", 0x18, (&f.3 as *const u64 as usize) - fb);
        println!("FOUNTAIN\tteam0={:?}\tteam1={:?}", map.fountains[0], map.fountains[1]);
    }

    // ───────── (N1) bumpalo Vec 의 len 위치 — len != cap 으로 갈랐다 ─────────
    {
        let b = bumpalo::Bump::new();
        let mut v: bumpalo::collections::Vec<&Entity> =
            bumpalo::collections::Vec::with_capacity_in(8, &b);
        let e0 = cache.player_champion[0][0].unwrap();
        v.push(e0); v.push(e0); v.push(e0);
        let w = unsafe { std::slice::from_raw_parts(&v as *const _ as *const usize, 4) };
        println!("BUMPVEC\tlen()={}\tcap()={}\twords(+0x0,+0x8,+0x10,+0x18)=[{:x},{:x},{},{}]",
                 v.len(), v.capacity(), w[0], w[1], w[2], w[3]);
        let len_at = if w[3] == v.len() && w[2] != v.len() { 0x18usize }
                     else if w[2] == v.len() && w[3] != v.len() { 0x10 } else { 0xffff };
        chk("2,3,4", "bumpalo Vec", "len (★5차 미판별분)", 0x18, len_at);
        // 결과적으로 twin_towers[0].len 의 절대 오프셋
        chk("2", "AbstractGameWithCache", "twin_towers[0].len(0x130+len_at)",
            0x148, 0x130 + len_at);
    }

    // ───────── (N2) AttackNexusPlan 16B 바이트 레이아웃 (수법 ⓒ) ─────────
    {
        use game_ai::plan_legacy::old::AttackNexusPlan;
        let p1 = AttackNexusPlan::new(1, LineType::Mid);
        let p2 = AttackNexusPlan::new(0, LineType::Bottom);
        let w1 = unsafe { std::slice::from_raw_parts(&p1 as *const _ as *const usize, 2) };
        let w2 = unsafe { std::slice::from_raw_parts(&p2 as *const _ as *const usize, 2) };
        println!("ANPLAN\tnew(1,Mid)  words=[{},{}]\tdbg={:?}", w1[0], w1[1], p1);
        println!("ANPLAN\tnew(0,Bottom) words=[{},{}]\tdbg={:?}", w2[0], w2[1], p2);
        // team 이 +0x0 이면 w[0] 이 1/0, line 이 +0x8 이면 w[1] 이 Mid(1)/Bottom(2)
        let team_at0 = w1[0] == 1 && w2[0] == 0;
        let line_at8 = w1[1] == 1 && w2[1] == 2;
        chk("2", "AttackNexusPlan", "team(★5차 미실측)", 0x0, if team_at0 { 0x0 } else { 0xffff });
        chk("2", "AttackNexusPlan", "line(★5차 미실측)", 0x8, if line_at8 { 0x8 } else { 0xffff });
        println!("LINETYPE\tTop={} Mid={} Bottom={}",
                 LineType::Top as usize, LineType::Mid as usize, LineType::Bottom as usize);
    }

    // ───────── (N4) Option<Input> / SubPlan sret 레이아웃 ─────────
    {
        let it = InputTarget::Target { target_id: 7 };
        let v: Option<Input> = Some(Input::Ult { target: it });
        let b = unsafe { std::slice::from_raw_parts(&v as *const _ as *const u8, 32) };
        let tag = unsafe { std::ptr::read_unaligned(&v as *const _ as *const i64) };
        chkv("0", "Option<Input>", "Some(Input::Ult) tag@+0x0", 5, tag as usize);
        let n: Option<Input> = None;
        let ntag = unsafe { std::ptr::read_unaligned(&n as *const _ as *const i64) };
        println!("OPTINPUT\tNone tag@+0x0={}\t(명세 -1)\t{}", ntag,
                 if ntag == -1 { "MATCH" } else { "**MISMATCH**" });
        let pay = unsafe { std::ptr::read_unaligned(
            ((&v as *const _ as usize) + 8) as *const InputTarget) };
        println!("OPTINPUT\tpayload@+0x8={:?}\t(명세 InputTarget 24B @ +0x8)", pay);
    }

    // ───────── (N5) DefensiveCrisis 두 bool ─────────
    {
        let dc = game_ai::DefensiveCrisis { die_imminent: true, cc_threat: false };
        let base = &dc as *const _ as usize;
        chk("3", "DefensiveCrisis", "die_imminent", 0x0,
            (&dc.die_imminent as *const bool as usize) - base);
        chk("3", "DefensiveCrisis", "cc_threat", 0x1,
            (&dc.cc_threat as *const bool as usize) - base);
    }

    unsafe { println!("\nTOTAL\tMATCH={}\tMISMATCH={}", NOK, NNG); }
}
