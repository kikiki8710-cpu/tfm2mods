#![allow(unused, dead_code, non_snake_case)]
//! A6-O3 — `specs[1] calculate_jungle_action_score` 계수표 재확인(수법: 5차와 동일)
//!        + `specs[4] handle_line_defense` 의 열거형 태그·튜토리얼 게이트·**version/_debug 무관 반증시험**.
//! 5차 대비 확장분:
//!   (N1) `MorgardDefenseStrategy` 실제 태그값(명세 `0=Gather 1=Battle`) 실측.
//!   (N2) `handle_line_defense` 를 **version 0..=8 전값**으로 호출해 `notes[0]`(「_version 은 전혀 안 쓰인다」)
//!        을 **반증 시도**한다. + `_debug`(DebugFrameData) 가 호출 전후로 바뀌는지 바이트 diff(수법 ⓒ).
//!   (N3) `LineType` 3값 × 반환 일관성.
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

fn main() {
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

    // ── (N1) 열거형 태그 실측 ─────────────────────────────────────────
    {
        let g = MorgardDefenseStrategy::Gather;
        let b = MorgardDefenseStrategy::Battle;
        println!("TAG\tMorgardDefenseStrategy\tGather={}\tBattle={}\t(명세 0=Gather 1=Battle)",
                 unsafe { *(&g as *const _ as *const u8) },
                 unsafe { *(&b as *const _ as *const u8) });
        println!("TAG\tLineType\tTop={}\tMid={}\tBottom={}",
                 unsafe { *(&LineType::Top as *const _ as *const u8) },
                 unsafe { *(&LineType::Mid as *const _ as *const u8) },
                 unsafe { *(&LineType::Bottom as *const _ as *const u8) });
        use game_ai::MinionActionType as MAT;
        println!("TAG\tMinionActionType\tPull={}\tNormal={}\tPush={}",
                 unsafe { *(&MAT::Pull as *const _ as *const u8) },
                 unsafe { *(&MAT::Normal as *const _ as *const u8) },
                 unsafe { *(&MAT::Push as *const _ as *const u8) });
    }
    // 튜토리얼 게이트
    {
        let tts = [TutorialType::None, TutorialType::First, TutorialType::TopSolo, TutorialType::Bottom,
                   TutorialType::MidSolo, TutorialType::MidBottom, TutorialType::JungleOnly,
                   TutorialType::Line, TutorialType::Total];
        let mut s = String::new();
        for t in tts.iter() {
            let tag = unsafe { *(t as *const TutorialType as *const u8) };
            s += &format!("{}={} ", tag, t.spawn_epic());
        }
        println!("SPAWN_EPIC\t{}\t(명세 consts[0]=-1·consts[1]=6 ⟹ tutorial-1 <u 6 이면 false = 0/7/8 만 true)", s);
        println!("MORGARD_EXISTS\t{}", game_ai::plan_legacy::rule_scope::morgard_exists(&ctx));
    }

    let mut game = mkgame(&setting, &ms, &map, &ctx);

    // ── specs[1] 계수표 (5차와 같은 방법, 재확인) ─────────────────────
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let player: &PlayerState = game.get_player_by_position(0, Position::Jungle).unwrap();
        let champ: &Entity = cache.player_champion[0][1].unwrap();
        let dynchamp: &dyn AbstractEntity = champ as &dyn AbstractEntity;
        let ef = Effect {
            range: 200000, growth_range: 0, start_timing: 0,
            casting: CastingType::Targeting, target: CastingTarget::Enemy,
            ty: Arc::new(AttackEffect::new(500, 0)) as Arc<dyn EffectType>,
            attack_type: AttackType::Skill,
        };
        let zp: std::mem::MaybeUninit<game_ai::ScoreParameter> = std::mem::MaybeUninit::zeroed();
        let za: std::mem::MaybeUninit<Box<dyn Action>> = std::mem::MaybeUninit::zeroed();
        let parameter: &game_ai::ScoreParameter = unsafe { zp.assume_init_ref() };
        let action: &Box<dyn Action> = unsafe { za.assume_init_ref() };

        let mut spawned: Vec<(String, Entity)> = Vec::new();
        for team in 0..2usize {
            for jt in [JungleType::Rhino, JungleType::Mushroom, JungleType::Stump, JungleType::Bee] {
                let cs = game.mode.jungle_runner.get_camp_state(team, jt);
                for (k, e) in cs.spawn(&setting, 0, 5000 + team * 100).into_iter().enumerate() {
                    spawned.push((format!("Jungle(t{},{:?})#{}", team, jt, k), e));
                }
            }
        }
        for (i, id) in game.world.tower_ids.iter().enumerate().take(2) {
            if let Some(e) = game.world.entity.get(*id) { spawned.push((format!("Tower#{}", i), e.clone())); }
        }
        for (i, id) in game.world.nexus_ids.iter().enumerate().take(2) {
            if let Some(e) = game.world.entity.get(*id) { spawned.push((format!("Nexus#{}", i), e.clone())); }
        }
        if let Some(e) = cache.player_champion[1][0] { spawned.push(("Champion(enemy)".into(), e.clone())); }

        for (nm, t) in spawned.iter() {
            let tag = unsafe { std::ptr::read_unaligned(((t as *const Entity as usize) + 0x68) as *const i64) };
            let c0 = unsafe { std::ptr::read_unaligned(((t as *const Entity as usize) + 0x98) as *const usize) };
            let hp = t.hp as i64;
            if hp == 0 { println!("SCORE\t{}\ttag={}\thp=0 → skip", nm, tag); continue; }
            let value = Effect::expected_damage_target(&ef, &ctx, dynchamp, t) as i64;
            let mut rnd = rand::rngs::StdRng::seed_from_u64(5);
            let got = game_ai::calculate_jungle_action_score(&mut rnd, player, &data, parameter, action, &ef, t);
            let is_j = tag == 4 && c0 < 2;
            let coef: i64 = match tag {
                3 => 200, 2 => 80,
                4 => if c0 < 2 { if hp <= value { 40 } else { 20 } } else { 0 },
                1 => -10, _ => 0,
            };
            let based: i64 = if is_j { 5 } else { 0 };
            let pred = std::cmp::min(coef, coef.wrapping_mul(value) / hp) + based;
            println!("SCORE\t{}\ttag={}\tcamp0={}\thp={}\tvalue={}\tcoef={}\tbased={}\tgame={}\tmine={}\t{}",
                     nm, tag, c0, hp, value, coef, based, got, pred,
                     if got == pred { "MATCH" } else { "**MISMATCH**" });
        }
        // hp 스윕 — 40/20 arm 경계(`hp <= value`) 판별
        if let Some((_, e0)) = spawned.iter().find(|(n, _)| n.starts_with("Jungle")) {
            for hpv in [1usize, 495, 496, 497, 992, 993, 10000] {
                let mut e = e0.clone();
                e.hp = hpv;
                let value = Effect::expected_damage_target(&ef, &ctx, dynchamp, &e) as i64;
                let hp = e.hp as i64;
                let mut rnd = rand::rngs::StdRng::seed_from_u64(5);
                let got = game_ai::calculate_jungle_action_score(&mut rnd, player, &data, parameter, action, &ef, &e);
                let coef: i64 = if hp <= value { 40 } else { 20 };
                let pred = std::cmp::min(coef, coef.wrapping_mul(value) / hp) + 5;
                let coef_alt: i64 = if hp < value { 40 } else { 20 };
                let pred_alt = std::cmp::min(coef_alt, coef_alt.wrapping_mul(value) / hp) + 5;
                println!("HPSWEEP\thp={}\tvalue={}\tcoef(le)={}\tgame={}\tmine(hp<=value)={}\talt(hp<value)={}\t{}",
                         hp, value, coef, got, pred, pred_alt,
                         if got == pred && pred != pred_alt { "MATCH(판별)" }
                         else if got == pred { "MATCH" } else { "**MISMATCH**" });
            }
        }
    }

    // ── (N2) specs[4] version/_debug 무관 반증시험 ─────────────────────
    {
        // 적팀 에픽 버프를 켜서 첫 게이트를 통과시킨다(MobaMode 필드 pub)
        game.mode.epic_minion_buff_time = [600, 600];
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
        println!("MOBA\tepic_minion_buff_time={:?}\tremain_epic_time(0)={}\tremain_epic_time(1)={}",
                 game.mode.epic_minion_buff_time,
                 game.mode.remain_epic_time(0), game.mode.remain_epic_time(1));
        for line in [LineType::Top, LineType::Mid, LineType::Bottom] {
            let mut outs: Vec<String> = Vec::new();
            let mut dbg_changed = 0usize;
            for ver in 0..=8usize {
                let mut rnd = rand::rngs::StdRng::seed_from_u64(9);
                let mut dbg: DebugFrameData = Default::default();
                let before: Vec<u8> = unsafe {
                    std::slice::from_raw_parts(&dbg as *const _ as *const u8,
                                               std::mem::size_of::<DebugFrameData>()).to_vec() };
                let r = game_ai::plan_legacy::old::handle_line_defense(ver, &mut rnd, player, &data, line, &mut dbg);
                let after: Vec<u8> = unsafe {
                    std::slice::from_raw_parts(&dbg as *const _ as *const u8,
                                               std::mem::size_of::<DebugFrameData>()).to_vec() };
                if before != after { dbg_changed += 1; }
                outs.push(format!("{}", r));
            }
            let all_same = outs.iter().all(|x| x == &outs[0]);
            println!("HLD_VERSION\tline={:?}\tv0..8={:?}\tall_same={}\tdebug_bytes_changed={}/9\t{}",
                     line, outs, all_same, dbg_changed,
                     if all_same && dbg_changed == 0 { "명세 notes[0] 유지(반증 실패)" } else { "**명세 notes[0] 반증**" });
        }
        // rnd 소비 여부 — PlayerState::strategy 로 넘어가는지
        for line in [LineType::Mid] {
            let mut rnd1 = rand::rngs::StdRng::seed_from_u64(9);
            let b1: Vec<u8> = unsafe { std::slice::from_raw_parts(&rnd1 as *const _ as *const u8, 320).to_vec() };
            let mut dbg: DebugFrameData = Default::default();
            let _ = game_ai::plan_legacy::old::handle_line_defense(2, &mut rnd1, player, &data, line, &mut dbg);
            let a1: Vec<u8> = unsafe { std::slice::from_raw_parts(&rnd1 as *const _ as *const u8, 320).to_vec() };
            println!("HLD_RND\tline={:?}\trnd_state_changed={}\t(명세 sig.params[2]: strategy 로 전달)",
                     line, b1 != a1);
        }
    }
}
