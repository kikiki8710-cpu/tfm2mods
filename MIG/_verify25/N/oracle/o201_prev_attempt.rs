#![allow(unused, dead_code, non_snake_case)]
#![feature(thread_local)]
//! 25차 N · #201 abstract_input::attack (pub `game_ai::attack`) 오라클.
//! ① 안전 호출(`game_ai::attack`) ↔ ② raw 호출(`extern "C"` + 32B sret 버퍼 0xEE 프리필, `#[link_name]`) 두 경로로 부르고
//!    sret 32B 의 **살아있는 바이트**(프리필이 남은 자리 = 미기록)를 그대로 찍는다.
//! ③ 명세 `logic` 대로 pub 함수(Effect::range/range_adjust · Entity::radius · EntityType::is_tower · utils::isqrt ·
//!    Game::adjust_position · game_ai::safe_move_avoiding_enemy_well)로 독립 재구현한 예측과 비트 대조(수법 ⓓ).
//! 한 프로세스 = 한 케이스(argv[1]).
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

extern "C" {
    // IR: define void @..attack(ptr sret([32 x i8]) %0, i64 %1, ptr %2, ptr %3, ptr %4, ptr %5, ptr %6)  — ccc(fastcc 아님)
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai14abstract_input6attack"]
    fn attack_raw(out: *mut [u8; 32], version: usize, rnd: *mut rand::rngs::StdRng, player: *const PlayerState,
                  data: *const OperationData, ps: *const PositioningScoreData, target: *const Entity);
}

extern "Rust" {
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai13position_eval14POS_EVAL_CACHE0023___RUST_STD_INTERNAL_VAL"] static mut TLS_POS_EVAL_CACHE: [u8; 32];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai13position_eval14POS_EVAL_CACHE0s_023___RUST_STD_INTERNAL_VAL"] static mut TLS_POS_EVAL_CACHE_s: [u8; 32];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai13position_eval16ATTACK_DMG_CACHE0023___RUST_STD_INTERNAL_VAL"] static mut TLS_ATTACK_DMG_CACHE: [u8; 96];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai13position_eval22TOWER_MINION_CNT_CACHE0023___RUST_STD_INTERNAL_VAL"] static mut TLS_TOWER_MINION_CNT_CACHE: [u8; 96];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai13position_eval13PE_CAND_MASKS0023___RUST_STD_INTERNAL_VAL"] static mut TLS_PE_CAND_MASKS: [u8; 40];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai13position_eval13PE_PLAYER_CTX0023___RUST_STD_INTERNAL_VAL"] static mut TLS_PE_PLAYER_CTX: [u8; 48];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai16tower_discipline18SIEGE_STANCE_CACHE0023___RUST_STD_INTERNAL_VAL"] static mut TLS_SIEGE_STANCE_CACHE: [u8; 96];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai11fight_check15SLOT_READY_MEMO0023___RUST_STD_INTERNAL_VAL"] static mut TLS_SLOT_READY_MEMO: [u8; 96];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai11fight_check12CC_TIME_MEMO0023___RUST_STD_INTERNAL_VAL"] static mut TLS_CC_TIME_MEMO: [u8; 96];
    #[thread_local] #[link_name = "_RNvNCNKNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model19RESOLVE_FIGHT_CACHE0023___RUST_STD_INTERNAL_VAL"] static mut TLS_RESOLVE_FIGHT_CACHE: [u8; 96];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai12action_score9INTER_CTX0023___RUST_STD_INTERNAL_VAL"] static mut TLS_INTER_CTX: [u8; 112];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai5utils13HP_VALUE_MEMO0023___RUST_STD_INTERNAL_VAL"] static mut TLS_HP_VALUE_MEMO: [u8; 104];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai11fight_check14DIE_TICK_CACHE0023___RUST_STD_INTERNAL_VAL"] static mut TLS_DIE_TICK_CACHE: [u8; 96];
    #[thread_local] #[link_name = "_RNvNCNKNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6battle15MAX_RANGE_CACHE0023___RUST_STD_INTERNAL_VAL"] static mut TLS_MAX_RANGE_CACHE: [u8; 1632];
    #[thread_local] #[link_name = "_RNvNCNKNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6battle10CAST_BEAMSs_0023___RUST_STD_INTERNAL_VAL"] static mut TLS_CAST_BEAMS: [u8; 336];
    #[thread_local] #[link_name = "_RNvNCNKNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus15LAST_STAND_MEMO0023___RUST_STD_INTERNAL_VAL"] static mut TLS_LAST_STAND_MEMO: [u8; 96];
    #[thread_local] #[link_name = "_RNvNCNKNvNvMNtNtCs97f5S1uJLkH_9game_core10simulation7map_defNtBa_6MapDef8camp_pos13CAMP_POS_MEMOs0_0023___RUST_STD_INTERNAL_VAL"] static mut TLS_CAMP_POS_MEMO: [u8; 408];
}
fn tls_snap() -> Vec<(&'static str, u8, u64)> { unsafe { vec![
    ("POS_EVAL_CACHE", std::ptr::read_volatile(&raw const TLS_POS_EVAL_CACHE[24]), TLS_POS_EVAL_CACHE[..24].iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64))),
    ("POS_EVAL_CACHE_s", std::ptr::read_volatile(&raw const TLS_POS_EVAL_CACHE_s[24]), TLS_POS_EVAL_CACHE_s[..24].iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64))),
    ("ATTACK_DMG_CACHE", std::ptr::read_volatile(&raw const TLS_ATTACK_DMG_CACHE[88]), TLS_ATTACK_DMG_CACHE[..88].iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64))),
    ("TOWER_MINION_CNT_CACHE", std::ptr::read_volatile(&raw const TLS_TOWER_MINION_CNT_CACHE[88]), TLS_TOWER_MINION_CNT_CACHE[..88].iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64))),
    ("PE_CAND_MASKS", std::ptr::read_volatile(&raw const TLS_PE_CAND_MASKS[32]), TLS_PE_CAND_MASKS[..32].iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64))),
    ("PE_PLAYER_CTX", std::ptr::read_volatile(&raw const TLS_PE_PLAYER_CTX[40]), TLS_PE_PLAYER_CTX[..40].iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64))),
    ("SIEGE_STANCE_CACHE", std::ptr::read_volatile(&raw const TLS_SIEGE_STANCE_CACHE[88]), TLS_SIEGE_STANCE_CACHE[..88].iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64))),
    ("SLOT_READY_MEMO", std::ptr::read_volatile(&raw const TLS_SLOT_READY_MEMO[88]), TLS_SLOT_READY_MEMO[..88].iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64))),
    ("CC_TIME_MEMO", std::ptr::read_volatile(&raw const TLS_CC_TIME_MEMO[88]), TLS_CC_TIME_MEMO[..88].iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64))),
    ("RESOLVE_FIGHT_CACHE", std::ptr::read_volatile(&raw const TLS_RESOLVE_FIGHT_CACHE[88]), TLS_RESOLVE_FIGHT_CACHE[..88].iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64))),
    ("INTER_CTX", std::ptr::read_volatile(&raw const TLS_INTER_CTX[104]), TLS_INTER_CTX[..104].iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64))),
    ("HP_VALUE_MEMO", std::ptr::read_volatile(&raw const TLS_HP_VALUE_MEMO[96]), TLS_HP_VALUE_MEMO[..96].iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64))),
    ("DIE_TICK_CACHE", std::ptr::read_volatile(&raw const TLS_DIE_TICK_CACHE[88]), TLS_DIE_TICK_CACHE[..88].iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64))),
    ("MAX_RANGE_CACHE", std::ptr::read_volatile(&raw const TLS_MAX_RANGE_CACHE[1624]), TLS_MAX_RANGE_CACHE[..1624].iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64))),
    ("CAST_BEAMS", std::ptr::read_volatile(&raw const TLS_CAST_BEAMS[328]), TLS_CAST_BEAMS[..328].iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64))),
    ("LAST_STAND_MEMO", std::ptr::read_volatile(&raw const TLS_LAST_STAND_MEMO[88]), TLS_LAST_STAND_MEMO[..88].iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64))),
    ("CAMP_POS_MEMO", std::ptr::read_volatile(&raw const TLS_CAMP_POS_MEMO[400]), TLS_CAMP_POS_MEMO[..400].iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64))),
]}}

#[derive(Debug)]
struct AutoFx;
impl EffectType for AutoFx {
    fn apply(&self, _r: &mut rand::rngs::StdRng, _g: &mut dyn AbstractGame, _c: &GameContext, _u: usize, _t: InputTarget, _a: AttackType, _o: Option<EffectOptionalInfo>, _f: &mut Option<&mut GameFrameData>) {}
    fn auto_target(&self) -> bool { true }
}
fn mkeffect(casting: CastingType) -> Effect {
    Effect { ty: Arc::new(AutoFx), range: 50000, growth_range: 0, start_timing: 0, target: CastingTarget::Enemy, attack_type: AttackType::BaseAttack, casting }
}

fn w64(p: *mut Entity, off: usize, v: u64) { unsafe { std::ptr::write_volatile((p as *mut u8).add(off) as *mut u64, v) } }
fn w32(p: *mut Entity, off: usize, v: i32) { unsafe { std::ptr::write_volatile((p as *mut u8).add(off) as *mut i32, v) } }
fn r64(p: *const Entity, off: usize) -> u64 { unsafe { std::ptr::read_volatile((p as *const u8).add(off) as *const u64) } }
fn r32(p: *const Entity, off: usize) -> i32 { unsafe { std::ptr::read_volatile((p as *const u8).add(off) as *const i32) } }

fn hex(b: &[u8]) -> String { b.iter().map(|x| format!("{:02x}", x)).collect::<Vec<_>>().join(" ") }
fn words(o: &Option<Input>) -> [u64; 4] { unsafe { std::mem::transmute_copy::<Option<Input>, [u64; 4]>(o) } }

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
    let ps: PositioningScoreData = Default::default();
    let player = game.get_player_by_position(0, Position::Top).unwrap();
    let V = 2usize;
    let champ = cache.player_champion[0][0].unwrap();
    let cp = champ as *const Entity as *mut Entity;
    let ally = cache.player_champion[0][1].unwrap();
    let enemy = cache.player_champion[1][0].unwrap();
    let ep = enemy as *const Entity as *mut Entity;
    let etower = cache.mid_tower[1].unwrap();
    let tp = etower as *const Entity as *mut Entity;
    let enexus = cache.nexus[1].unwrap();
    let np = enexus as *const Entity as *mut Entity;
    println!("ids\tchamp={} ({},{}) r={} lvl={} team={:?}\tenemy={} ({},{})\ttower={} ({},{}) ty_tag={}\tnexus={} ({},{}) ty_tag={}",
        champ.id, champ.x, champ.y, champ.radius, champ.level, champ.team, enemy.id, enemy.x, enemy.y,
        etower.id, etower.x, etower.y, r64(tp, 0x68), enexus.id, enexus.x, enexus.y, r64(np, 0x68));
    println!("champ.attack_effect tag@0x4c0={} can_attack={} aspd_mult={} eff.range={:?} growth={:?} casting@0x4c0={} target@0x4b8={}",
        r32(cp, 0x4c0), champ.can_attack(), champ.attack_speed_mult(),
        champ.attack_effect.as_ref().map(|e| e.range), champ.attack_effect.as_ref().map(|e| e.growth_range), r32(cp, 0x4c0), r32(cp, 0x4b8));

    // 케이스별 세계 조립 (write_volatile — 함정 ⑦: 포인터 인자로)
    let (target, desc): (&Entity, &str) = match case {
        0 => { w64(ep, 0x660, champ.x + 300000); w64(ep, 0x668, champ.y + 100000); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Visible) }; (enemy, "적 챔프 · 가시 · 사거리 밖(300k,100k) → Move(사거리−15000 지점)") }
        1 => { w64(ep, 0x660, champ.x + 300000); w64(ep, 0x668, champ.y + 100000); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Unknown) }; (enemy, "적 챔프 · 비가시(Unknown) · 사거리 밖 → safe_move(target.x,target.y) (L185)") }
        2 => { unsafe { std::ptr::write_volatile(&mut (*tp).visible_state[0], VisibleState::Visible) }; (etower, "적 미드 타워 · 가시 → Move(사거리−2000 지점)") }
        3 => { unsafe { std::ptr::write_volatile(&mut (*np).visible_state[0], VisibleState::Visible) }; (enexus, "적 넥서스(태그 3) · 가시 → 마스크 14 로 is_tower → 2000") }
        4 => { w64(ep, 0x660, champ.x + 5000); w64(ep, 0x668, champ.y); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Visible) }; (enemy, "적 챔프 · 가시 · 사거리 안(5000) → Input::Attack(InputTarget)") }
        5 => { unsafe { std::ptr::write_volatile(&mut (*(ally as *const Entity as *mut Entity)).visible_state[0], VisibleState::Visible) }; (ally, "아군 챔프 → CastingTarget::check 거부 → None (L157)") }
        6 => { w32(cp, 0x4c0, -1); w64(ep, 0x660, champ.x + 300000); w64(ep, 0x668, champ.y); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Visible) }; (enemy, "champ.attack_effect = None(니치 −1 @0x4c0) → None (L154)") }
        7 => { w64(cp, 0x0, 1); w64(ep, 0x660, champ.x + 300000); w64(ep, 0x668, champ.y + 100000); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Unknown) }; (enemy, "champ.team=Neutral(태그 1) · 대상 비가시 → is_visible_from=true 로 가시 경로(L166~)") }
        8 => { w64(ep, 0x660, champ.x + 300000); w64(ep, 0x668, champ.y + 100000); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Visible) }; w32(ep, 0x470, 50); w32(cp, 0x470, -50); (enemy, "radius_mult 50/−50 → radius*(mult+100)/100 경로") }
        9 => { w64(ep, 0x660, champ.x + 300000); w64(ep, 0x668, champ.y + 100000); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Visible) }; w64(ep, 0x680, 0); w64(cp, 0x680, 0); w64(cp, 0x438, 0); (enemy, "radius 0/0 · stat.range 0 → full_range 작아 saturating_sub 0 가능성") }
        10 => { w64(ep, 0x660, champ.x + 300000); w64(ep, 0x668, champ.y + 100000); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Invisible { last_x: 0, last_y: 0 }) }; (enemy, "적 챔프 · Invisible(태그 1) → L185 경로") }
        11 => { w64(ep, 0x660, champ.x + 300000); w64(ep, 0x668, champ.y + 100000); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Visible) }; w64(ep, 0x5c8, 1); w64(cp, 0x5c8, 5); (enemy, "champ.level=5 → (level−1)*growth_range 항") }
        12 => { unsafe { std::ptr::write_volatile(&mut (*np).visible_state[0], VisibleState::Visible); std::ptr::write_volatile(&mut (*np).can_target, true); std::ptr::write_volatile(&mut (*np).block_target_tick, 0usize) }; (enexus, "적 넥서스(태그 3) · can_target=1 · block_target_tick=0 · 가시 → 마스크 14 → margin 2000") }
        13 => { w64(ep, 0x660, champ.x + 5000); w64(ep, 0x668, champ.y); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Unknown) }; (enemy, "적 챔프 · 사거리 안(5000) · 비가시 · Targeting → get_input_target None(L354) → L185 safe_move(target.x,y)") }
        14 => { w64(ep, 0x660, champ.x); w64(ep, 0x668, champ.y); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Visible) }; (enemy, "적 챔프 · 같은 좌표(sz=0) · 가시 → is_in_range 참이라 Attack (div_by_zero 미도달)") }
        15 => { unsafe { std::ptr::write_volatile(&mut (*tp).visible_state[0], VisibleState::Visible) }; w32(tp, 0x470, 0); w64(tp, 0x680, 0); w64(cp, 0x680, 1000); w64(cp, 0x438, 0); (etower, "적 타워 · 반지름 0/1000 → full 1000 < margin 2000 → saturating_sub 0 → x=target.x") }
        16 => { w64(ep, 0x660, champ.x + 5000); w64(ep, 0x668, champ.y); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Visible) }; w32(cp, 0x4c0, 3); (enemy, "casting=CastingType::None(3) · 사거리 안 · 가시 → get_input_target Some(InputTarget::None 태그 3) → Input::Attack{None}") }
        17 => { w64(ep, 0x660, champ.x + 5000); w64(ep, 0x668, champ.y); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Visible) }; w32(cp, 0x4c0, 1); (enemy, "casting=Position(1) · 사거리 안 · 가시 → Attack{Pos 또는 Target}") }
        18 => { w64(ep, 0x660, champ.x + 5000); w64(ep, 0x668, champ.y); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Visible) }; w32(cp, 0x4c0, 2); (enemy, "casting=Direction(2) · 사거리 안 · 가시 → Attack{Dir}") }
        19 => { w64(ep, 0x660, champ.x + 5000); w64(ep, 0x668, champ.y); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Visible); std::ptr::write_volatile(&mut (*cp).attack_effect, Some(mkeffect(CastingType::Position))) }; (enemy, "커스텀 EffectType(auto_target=true) · casting=Position · 사거리 안 · 가시 → get_input_target L534 술어 참 → position_score_at_cell 7x7 → POS_EVAL_CACHE 초기화 기대") }
        20 => { w64(ep, 0x660, champ.x + 5000); w64(ep, 0x668, champ.y); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Visible); std::ptr::write_volatile(&mut (*cp).attack_effect, Some(mkeffect(CastingType::Direction))) }; (enemy, "커스텀 EffectType(auto_target=true) · casting=Direction · 사거리 안 · 가시 → L379 술어 참 → position_score_at_cell") }
        21 => { w64(ep, 0x660, champ.x + 300000); w64(ep, 0x668, champ.y + 100000); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Visible); std::ptr::write_volatile(&mut (*cp).attack_effect, Some(mkeffect(CastingType::Position))) }; (enemy, "커스텀 EffectType(auto_target=true) · 사거리 밖 → is_in_range false → Move · TLS 무접촉 기대") }
        22 => { w64(ep, 0x660, champ.x + 300000); w64(ep, 0x668, champ.y + 100000); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Visible) }; w32(cp, 0x470, -50); (enemy, "champ.radius_mult=−50 만 → champ.r 5000 → full 15000 → fd 0 (case0 의 fd 5000 과 갈림)") }
        23 => { w64(ep, 0x660, champ.x + 300000); w64(ep, 0x668, champ.y + 100000); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Visible) }; w32(ep, 0x470, 100); (enemy, "target.radius_mult=+100 만 → target.r 20000 → full 30000 → fd 15000") }
        24 => { w64(ep, 0x660, champ.x + 300000); w64(ep, 0x668, champ.y + 100000); unsafe { std::ptr::write_volatile(&mut (*ep).visible_state[0], VisibleState::Visible) }; w64(cp, 0x5c8, 5); w64(cp, 0x4a8, 3000); (enemy, "champ.level=5 · growth_range=3000 → (level−1)*growth = 12000 → full 32000 → fd 17000") }
        _ => (enemy, "-"),
    };
    println!("case\t{}\t{}", case, desc);
    println!("target\tid={} ({},{}) r={} ty_tag={} vis0_tag={} radius_mult={}", target.id, target.x, target.y, target.radius, r64(target, 0x68), r64(target, 0x38), r32(target, 0x470));

    // ① raw 호출 — sret 버퍼 0xEE 프리필
    let mut rnd1 = rand::rngs::StdRng::seed_from_u64(11);
    let snap0 = tls_snap();
    let mut buf = [0xEEu8; 32];
    unsafe { attack_raw(&mut buf, V, &mut rnd1, player, &data, &ps, target) };
    println!("raw_sret\t{}", hex(&buf));
    let snap1 = tls_snap();
    for (a, b) in snap0.iter().zip(snap1.iter()) { println!("tls	{}	state {}->{}	{}", a.0, a.1, b.1, if a.2 != b.2 || a.1 != b.1 { "CHANGED" } else { "same" }); }
    let tag = u64::from_le_bytes(buf[0..8].try_into().unwrap());
    let w1 = u64::from_le_bytes(buf[8..16].try_into().unwrap());
    let w2 = u64::from_le_bytes(buf[16..24].try_into().unwrap());
    let w3 = u64::from_le_bytes(buf[24..32].try_into().unwrap());
    println!("raw_words\ttag={} (i64 {})\t+8={:#x}\t+16={:#x}\t+24={:#x}", tag, tag as i64, w1, w2, w3);

    // ② 안전 호출
    let mut rnd2 = rand::rngs::StdRng::seed_from_u64(11);
    let g = game_ai::attack(V, &mut rnd2, player, &data, &ps, target);
    let gw = words(&g);
    println!("safe\t{:?}\twords={:x?}", g, gw);
    println!("rnd_consumed\t{}", rnd1 != rand::rngs::StdRng::seed_from_u64(11));

    // ③ 독립 재구현 예측
    let pred: Option<Input> = (|| {
        if r32(cp, 0x4c0) == -1 { println!("pred_path\tL154 attack_effect None"); return None; }
        let eff = champ.attack_effect.as_ref().unwrap();
        if !champ.can_attack() { println!("pred_path\tL151 !can_attack"); return None; }
        if !eff.target.check(champ, target) { println!("pred_path\tL157 CastingTarget::check false"); return None; }
        // get_input_target 은 internal — 사거리 밖(is_in_range false)일 때만 None 이 확정. 안이면 Some 이라 가정하고 Attack 태그만 본다.
        let in_range = eff.is_in_range(champ, target);
        let team_tag = r64(cp, 0x0);
        let visible = if team_tag & 1 == 1 { true } else { let t = r64(cp, 0x8) as usize; r64(target, 0x38 + 24 * t) == 0 };
        let casting = r32(cp, 0x4c0);
        if in_range && (visible || casting == 1 || casting == 2) { println!("pred_path	L162 get_input_target Some(추정: is_in_range ∧ (가시 ∨ casting∈Position,Direction)) → Attack"); return Some(Input::Attack { target: InputTarget::None }); }
        if in_range { println!("pred_path	get_input_target None(L354: 비가시 ∧ Targeting) → L185"); }
        if !visible {
            println!("pred_path\tL185 비가시 → safe_move(target.x,target.y)");
            return game_ai::safe_move_avoiding_enemy_well(V, player, &data, champ, target.x, target.y);
        }
        let dx = champ.x as i64 - target.x as i64; let dy = champ.y as i64 - target.y as i64;
        let sz = game_core::utils::isqrt(dx * dx + dy * dy);
        let full = eff.range(champ).wrapping_add(eff.range_adjust(champ, target)).wrapping_add(champ.radius() as u64).wrapping_add(target.radius() as u64);
        let margin: u64 = if target.ty.is_tower() { 2000 } else { 15000 };
        let fd = full.saturating_sub(margin);
        let x = target.x as i64 + (fd as i64).wrapping_mul(dx) / sz;
        let y = target.y as i64 + (fd as i64).wrapping_mul(dy) / sz;
        let (ax, ay) = Game::adjust_position(&map, &setting, x, y);
        println!("pred_path\tL166~183 가시 · dx={} dy={} sz={} eff.range={} range_adjust={} champ.r={} target.r={} full={} margin={} fd={} xy=({},{}) adj=({},{})",
            dx, dy, sz, eff.range(champ), eff.range_adjust(champ, target), champ.radius(), target.radius(), full, margin, fd, x, y, ax, ay);
        game_ai::safe_move_avoiding_enemy_well(V, player, &data, champ, ax, ay)
    })();
    let pw = words(&pred);
    println!("pred\t{:?}\twords={:x?}", pred, pw);
    let attack_like = matches!(g, Some(Input::Attack { .. })) && matches!(pred, Some(Input::Attack { .. }));
    let m = if attack_like { "MATCH(Attack 태그 · InputTarget 은 콜리 소관)" }
            else if gw[0] == pw[0] && (gw[0] == u64::MAX || (gw[1] == pw[1] && gw[2] == pw[2])) { "MATCH" } else { "MISMATCH" };
    println!("verdict\t{}\traw==safe:{}", m, gw[0] == tag && (gw[0] == u64::MAX || (gw[1] == w1 && gw[2] == w2)));
}
