#![allow(unused, dead_code, non_snake_case)]
//! 15차 배치A 오라클 2 — #23 buy_item · #21 upgrade_item
//! 진입 = pub 래퍼 `AgentVerHamster::buy_item / upgrade_item`(lib.rs:1173/1193). 래퍼는
//! `player.info.item_builds.len()==0` 일 때 레거시 자유함수(`game_ai::buy_item`/`upgrade_item`, in:game_ai)를
//! `version=poison, game=poison` 으로 부른다(m14.ll:38174 / 35109). item_builds 는 비워 둔다.
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify15/A/oracle/o2.rs
use game_core::*;
use rand::{Rng, SeedableRng};
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

fn base(key: &str, tier: usize, price: usize, cat: ItemCategory, next: &[&str]) -> Box<dyn ItemInfo> {
    Box::new(BaseItemInfo {
        key: key.to_string(), icon: String::new(), price, tier, stat: Default::default(),
        next_tier: next.iter().map(|s| s.to_string()).collect(), tags: Vec::new(), category: cat,
    })
}
/// ModItemEntry.runtime=None → is_active()==false (g09.ll:203734 `+400 != null`)
fn inactive(key: &str, tier: usize, price: usize, cat: ItemCategory, next: &[&str]) -> Box<dyn ItemInfo> {
    Box::new(ModItemEntry {
        key: key.to_string(), icon: String::new(), price, tier, stat: Default::default(),
        next_tier: next.iter().map(|s| s.to_string()).collect(), tags: Vec::new(), category: cat, runtime: None,
    })
}

/// SwordmanChampionInfo 의 private `category`(tcx: +0x1f0, 4B i32) 를 직접 써서 5 카테고리를 전부 만든다
fn champ_cat(cat: i32, hp: usize) -> Arc<dyn ChampionInfo> {
    let mut st: EntityStat = Default::default();
    st.hp = hp;
    let mut c: SwordmanChampionInfo = Default::default();
    c.set_stat(st);
    unsafe { std::ptr::write_unaligned(((&mut c) as *mut SwordmanChampionInfo as *mut u8).add(0x1f0) as *mut i32, cat); }
    Arc::new(c) as Arc<dyn ChampionInfo>
}

fn champ(name: &str, hp: usize) -> Arc<dyn ChampionInfo> {
    let mut st: EntityStat = Default::default();
    st.hp = hp;
    macro_rules! mk { ($t:ty) => {{ let mut c: $t = Default::default(); c.set_stat(st); Arc::new(c) as Arc<dyn ChampionInfo> }} }
    match name {
        "Swordman" => mk!(SwordmanChampionInfo),
        "Archer" => mk!(ArcherChampionInfo),
        "Pythoness" => mk!(PythonessChampionInfo),
        "Priest" => mk!(PriestChampionInfo),
        "Ninja" => mk!(NinjaChampionInfo),
        "Knight" => mk!(KnightChampionInfo),
        "Fighter" => mk!(FighterChampionInfo),
        "Monk" => mk!(MonkChampionInfo),
        "Pyromancer" => mk!(PyromancerChampionInfo),
        "IceMage" => mk!(IceMageChampionInfo),
        "Soldier" => mk!(SoldierChampionInfo),
        _ => panic!("no champ {}", name),
    }
}
const NAMES: [&str; 11] = ["Swordman", "Archer", "Pythoness", "Priest", "Ninja", "Knight", "Fighter", "Monk", "Pyromancer", "IceMage", "Soldier"];

fn cat_i(c: ItemCategory) -> i32 { unsafe { std::mem::transmute::<ItemCategory, i32>(c) } }
fn ccat_i(c: ChampionCategory) -> i32 { unsafe { std::mem::transmute::<ChampionCategory, i32>(c) } }

/// #23 명세 재구현 → 후보 인덱스 집합
fn buy_pred(shop: &[Box<dyn ItemInfo>], owned: &[Box<dyn ItemInfo>], gold: usize, ci: &dyn ChampionInfo) -> Option<Vec<usize>> {
    if owned.iter().any(|x| x.tier() < 4) { return None; }
    let n = owned.len();
    let tag = ccat_i(ci.category());
    let mut cand = Vec::new();
    for (i, it) in shop.iter().enumerate() {
        if n > 2 { continue; }
        if !it.is_active() { continue; }
        if it.price() > gold { continue; }
        if it.tier() != 0 { continue; }
        let c = cat_i(it.category());
        let is_def = c == 2 || c == 3 || c == 5;
        let is_magic = c == 4;
        let ok = match tag {
            0 => { let hp = ci.stat().hp;
                   if hp < 1550 { if n == 0 || n == 2 { c < 2 } else { is_def } }
                   else if ci.stat().hp < 1800 { if n == 2 { c < 2 } else { is_def } }
                   else { is_def } }
            4 => if n == 0 || n == 2 { c < 2 } else { is_def },
            1 => c < 2,
            2 => is_magic,
            3 => { if ci.stat().hp < 1550 { is_magic } else { is_def } }
            _ => unreachable!(),
        };
        if ok { cand.push(i); }
    }
    if cand.is_empty() { None } else { Some(cand) }
}

/// #21 명세 재구현 → 후보 (슬롯 i, item_list idx) 목록
fn upg_pred(shop: &[Box<dyn ItemInfo>], owned: &[Box<dyn ItemInfo>], gold: usize) -> Option<Vec<(usize, usize)>> {
    let my_max = owned.iter().filter(|it| it.tier() < 4).map(|it| it.tier()).max().unwrap_or(0);
    let mut cand = Vec::new();
    for (i, my) in owned.iter().enumerate() {
        for nxt in my.next_tier().iter() {
            if let Some(index) = item_index_by_key(shop, nxt) {
                let it = &shop[index];
                if it.is_active() && it.tier() > my_max && !(it.price() > gold) { cand.push((i, index)); }
            }
        }
    }
    if cand.is_empty() { None } else { Some(cand) }
}

fn main() {
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    use ItemCategory::*;
    let items: Vec<Box<dyn ItemInfo>> = vec![
        base("ad0", 0, 300, AD, &["ad1", "ad1_inactive", "missingkey"]),   // 0
        base("as0", 0, 400, AttackSpeed, &[]),                              // 1
        base("def0", 0, 500, Defense, &[]),                                 // 2
        base("mr0", 0, 500, MagicResistance, &[]),                          // 3
        base("mag0", 0, 450, Magic, &[]),                                   // 4
        base("hp0", 0, 350, Hp, &[]),                                       // 5
        base("sup0", 0, 100, Support, &[]),                                 // 6
        base("ad1", 1, 1200, AD, &["ad2", "ad2x"]),                         // 7
        base("ad2", 2, 2500, AD, &[]),                                      // 8
        base("ad2x", 2, 9999, AD, &[]),                                     // 9
        inactive("inactive0", 0, 100, AD, &[]),                             // 10
        inactive("ad1_inactive", 1, 100, AD, &[]),                          // 11
        base("t4", 4, 100, AD, &["t5"]),                                    // 12
        base("t5", 5, 100, AD, &[]),                                        // 13
        base("t3", 3, 100, Defense, &["t4"]),                               // 14
    ];
    let ctx = GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off,
    };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    // 셋업 무결성
    for (i, it) in items.iter().enumerate() {
        println!("SHOP\t{}\t{}\ttier={}\tprice={}\tcat={}\tactive={}\tidx_by_key={:?}", i, it.key(), it.tier(), it.price(), cat_i(it.category()), it.is_active(), item_index_by_key(&items, it.key()));
    }
    println!("ITEM_INDEX_MISSING\t{:?}", item_index_by_key(&items, "missingkey"));
    // 챔피언 카테고리 표
    for nm in NAMES.iter() {
        let c = champ(nm, 1234);
        println!("CHAMP\t{}\tcat={}\tstat.hp={}", nm, ccat_i(c.category()), c.stat().hp);
    }
    // 카테고리별 대표 하나
    let mut rep: [Option<&str>; 5] = [None; 5];
    for nm in NAMES.iter() { let c = champ(nm, 1); let k = ccat_i(c.category()) as usize; if rep[k].is_none() { rep[k] = Some(nm); } }
    println!("REP\t{:?}", rep);

    let mut rnd0 = rand::rngs::StdRng::seed_from_u64(99);
    let mut agent = game_ai::AgentVerHamster::new(&mut rnd0, 2, 0, Position::Top);
    let pid = game.get_player_by_position(0, Position::Top).unwrap().info.id;
    println!("PID\t{}\titem_builds_len={}", pid, game.get_player_by_position(0, Position::Top).unwrap().info.item_builds.len());

    let mk_owned = |keys: &[&str]| -> Vec<Box<dyn ItemInfo>> {
        keys.iter().map(|k| { let i = item_index_by_key(&items, k).unwrap(); match items[i].as_any().downcast_ref::<BaseItemInfo>() {
            Some(b) => Box::new(b.clone()) as Box<dyn ItemInfo>, None => panic!("owned must be BaseItemInfo") } }).collect()
    };

    // ───────────── #23 buy_item ─────────────
    let mut n_match = 0; let mut n_mis = 0;
    let owned_sets: Vec<(&str, Vec<&str>)> = vec![
        ("[]", vec![]), ("[ad1]", vec!["ad1"]), ("[t4]", vec!["t4"]), ("[t4,t5]", vec!["t4", "t5"]),
        ("[t4,t5,t4]", vec!["t4", "t5", "t4"]), ("[t3]", vec!["t3"]),
    ];
    for cat in 0..5usize {
        let nm = "Swordman(cat-patched)";
        { let c = champ_cat(cat as i32, 5); println!("CHAMP_PATCHED	cat_req={}	cat_got={}	stat.hp={}", cat, ccat_i(c.category()), c.stat().hp); }
        for hp in [1000usize, 1549, 1550, 1799, 1800, 3000] {
            for (oname, okeys) in owned_sets.iter() {
                for gold in [10000usize, 350, 50] {
                    {
                        let ps = game.world.players.get_mut(pid).unwrap();
                        ps.info.champion = champ_cat(cat as i32, hp);
                        ps.info.items = mk_owned(okeys);
                        ps.info.gold = gold;
                    }
                    let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
                    let ci: &dyn ChampionInfo = &*player.info.champion;
                    let pred_set = buy_pred(&items, &player.info.items, gold, ci);
                    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
                    let mut rnd_mine = rnd.clone();
                    let got = agent.buy_item(&mut rnd, player, &game as &dyn AbstractGame, &ctx);
                    let pred = pred_set.as_ref().map(|c| c[rnd_mine.gen_range(0..c.len())]);
                    let v = if got == pred { n_match += 1; "MATCH" } else { n_mis += 1; "**MISMATCH**" };
                    println!("BUY\tcat={}({})\thp={}\towned={}\tgold={}\tgame={:?}\tmine={:?}\tcands={:?}\t{}", cat, nm, hp, oname, gold, got, pred, pred_set, v);
                }
            }
        }
    }
    println!("BUY_SUMMARY\tmatch={}\tmismatch={}", n_match, n_mis);

    // ───────────── #21 upgrade_item ─────────────
    let mut n_match = 0; let mut n_mis = 0;
    let owned_sets: Vec<(&str, Vec<&str>)> = vec![
        ("[]", vec![]), ("[ad0]", vec!["ad0"]), ("[ad0,ad1]", vec!["ad0", "ad1"]), ("[ad0,t4]", vec!["ad0", "t4"]),
        ("[t4]", vec!["t4"]), ("[ad1,ad2]", vec!["ad1", "ad2"]), ("[t3]", vec!["t3"]), ("[t3,ad0]", vec!["t3", "ad0"]),
        ("[ad1,t4]", vec!["ad1", "t4"]), ("[t4,ad1]", vec!["t4", "ad1"]),
    ];
    for (oname, okeys) in owned_sets.iter() {
        for gold in [10000usize, 3000, 1199, 1200, 99] {
            {
                let ps = game.world.players.get_mut(pid).unwrap();
                ps.info.champion = champ("Swordman", 1000);
                ps.info.items = mk_owned(okeys);
                ps.info.gold = gold;
            }
            let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
            let pred_set = upg_pred(&items, &player.info.items, gold);
            let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
            let mut rnd_mine = rnd.clone();
            let got = agent.upgrade_item(&mut rnd, player, &game as &dyn AbstractGame, &ctx);
            let pred = pred_set.as_ref().map(|c| c[rnd_mine.gen_range(0..c.len())]);
            let v = if got == pred { n_match += 1; "MATCH" } else { n_mis += 1; "**MISMATCH**" };
            println!("UPG\towned={}\tgold={}\tgame={:?}\tmine={:?}\tcands={:?}\t{}", oname, gold, got, pred, pred_set, v);
        }
    }
    println!("UPG_SUMMARY\tmatch={}\tmismatch={}", n_match, n_mis);
    println!("DONE\tsetting_ok={}", ok);
}
