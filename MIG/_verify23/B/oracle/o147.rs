#![allow(unused, dead_code, non_snake_case)]
//! 23차 B · #147 line_action_economy_adjustment (pub) — 직접 호출. 얕은 조기반환 경로 전수 + 깊은 경로 관측.
//! 표적: 비공격 액션 → 0 · effect None → 0 · 대상 None → 0 · 같은 팀 → 0 · ty ∉ {Minion,Champion} → 0 · CastingTarget 거부 → 0 ·
//!       level 게이트(skill2 >2 · ult >4).  깊은 경로(punish≥1)는 적 미니언 부재로 관측만.
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

fn w64(p: *mut Entity, off: usize, v: u64) { unsafe { std::ptr::write_volatile((p as *mut u8).add(off) as *mut u64, v) } }
fn r64(p: *const Entity, off: usize) -> u64 { unsafe { std::ptr::read_volatile((p as *const u8).add(off) as *const u64) } }
fn r32(p: *const Entity, off: usize) -> i32 { unsafe { std::ptr::read_volatile((p as *const u8).add(off) as *const i32) } }

fn mkeff(t: CastingTarget) -> Effect {
    Effect { ty: Arc::new(AttackEffect::new(100, 0)) as Arc<dyn EffectType>, range: 100000, growth_range: 0, start_timing: 20,
             target: t, attack_type: AttackType::BaseAttack, casting: CastingType::Targeting }
}

fn main() {
    let case: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(0, Position::Top).unwrap();
    let V = 55usize;
    let champ = cache.player_champion[0][0].unwrap();
    let cp = champ as *const Entity as *mut Entity;
    let ally = cache.player_champion[0][1].unwrap();
    let enemy = cache.player_champion[1][0].unwrap();
    let ep = enemy as *const Entity as *mut Entity;
    let tower_id = game.world.tower_ids.iter().copied().find(|id| game.get_entity_by_id(*id).map(|e| e.team == TeamType::Player(1)).unwrap_or(false)).unwrap();
    println!("champ\tid={} attack_tag={} skill_tag={} skill2_tag={} ult_tag={} level={} hp={}", champ.id, r32(cp, 0x4c0), r32(cp, 0x4f8), r32(cp, 0x530), r32(cp, 0x568), r64(cp, 0x5c8), r64(cp, 0x670));
    let zp: std::mem::MaybeUninit<game_ai::ScoreParameter> = std::mem::MaybeUninit::zeroed();
    let parameter: game_ai::ScoreParameter = unsafe { zp.assume_init() };
    let ty = game_ai::MinionActionType::Normal;
    let mut rnd = rand::rngs::StdRng::seed_from_u64(9);

    let (act, pred, desc): (game_ai::SmallActionPlay, Option<i64>, &str) = match case {
        0 => (game_ai::SmallActionPlay::RunAway(game_ai::SmallActionRunAway::new(&data, player, 0)), Some(0), "비공격 액션 → 0"),
        1 => (game_ai::SmallActionPlay::Attack(game_ai::SmallActionAttack::new(&data, enemy.id)), None, "Attack · 기본 attack_effect(None 이면 0)"),
        2 => { unsafe { std::ptr::write(&mut (*cp).attack_effect as *mut Option<Effect>, Some(mkeff(CastingTarget::Enemy))); }
               (game_ai::SmallActionPlay::Attack(game_ai::SmallActionAttack::new(&data, 999_999)), Some(0), "Attack · 대상 없음 → 0") }
        3 => { unsafe { std::ptr::write(&mut (*cp).attack_effect as *mut Option<Effect>, Some(mkeff(CastingTarget::Enemy))); }
               (game_ai::SmallActionPlay::Attack(game_ai::SmallActionAttack::new(&data, ally.id)), Some(0), "Attack · 같은 팀 → 0") }
        4 => { unsafe { std::ptr::write(&mut (*cp).attack_effect as *mut Option<Effect>, Some(mkeff(CastingTarget::Enemy))); }
               (game_ai::SmallActionPlay::Attack(game_ai::SmallActionAttack::new(&data, tower_id)), Some(0), "Attack · 적 타워(ty 2) → 0") }
        5 => { unsafe { std::ptr::write(&mut (*cp).attack_effect as *mut Option<Effect>, Some(mkeff(CastingTarget::Ally))); }
               (game_ai::SmallActionPlay::Attack(game_ai::SmallActionAttack::new(&data, enemy.id)), Some(0), "Attack · 적 챔피언인데 CastingTarget::Ally → check 거부 → 0") }
        6 => { unsafe { std::ptr::write(&mut (*cp).attack_effect as *mut Option<Effect>, Some(mkeff(CastingTarget::Enemy))); }
               w64(ep, 0x660, r64(cp, 0x660) + 50000); w64(ep, 0x668, r64(cp, 0x668));
               (game_ai::SmallActionPlay::Attack(game_ai::SmallActionAttack::new(&data, enemy.id)), None, "Attack · 적 챔피언 50k 거리(깊은 경로 관측)") }
        7 => { w64(cp, 0x5c8, 2); (game_ai::SmallActionPlay::Skill2(game_ai::SmallActionSkill2::new(&data, enemy.id)), Some(0), "Skill2 · level 2 → effect None → 0") }
        8 => { w64(cp, 0x5c8, 4); (game_ai::SmallActionPlay::Ult(game_ai::SmallActionUlt::new(&data, enemy.id)), Some(0), "Ult · level 4 → effect None → 0") }
        9 => (game_ai::SmallActionPlay::Skill(game_ai::SmallActionSkill::new(&data, enemy.id)), None, "Skill · 기본 skill_effect(None 이면 0)"),
        _ => (game_ai::SmallActionPlay::Stop, Some(0), "Stop → 0"),
    };
    let tag: u8 = unsafe { *((&act as *const _ as *const u8).add(0xb1)) };
    let g = game_ai::line_action_economy_adjustment(V, player, &data, &parameter, &act, ty);
    let m = match pred { Some(p) => if p == g { "MATCH" } else { "**MISMATCH**" }, None => "OBS" };
    println!("case{}\ttag={}\tgame={}\tpred={:?}\t{}\t// {}", case, tag, g, pred, m, desc);
}
