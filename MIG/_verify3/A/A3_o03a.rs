#![allow(unused, dead_code, non_snake_case)]
// 3차 배치A 오라클 1 — 03 defensive_crisis 의 cc_threat 진리표를 여는 재료:
//   game_ai::effect_cc_time(version, &Effect) 를 실전 챔피언 61종 x 4슬롯에 전수 적용.
//   Game 을 만들 필요조차 없다: ChampionInfo::{attack,skill,skill2,ult}() -> Box<dyn Action>,
//   Action::effect() -> Option<Effect> 전부 pub.
use game_core::*;
use std::sync::Arc;

fn probe(name: &str, ci: &dyn ChampionInfo) {
    let getters: [(&str, u8); 4] = [("attack",0),("skill",1),("skill2",2),("ult",3)];
    for (sn, k) in getters.iter() {
        let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match k {
            0 => ci.attack(), 1 => ci.skill(), 2 => ci.skill2(), _ => ci.ult() }));
        let a = match got { Ok(a) => a, Err(_) => { println!("{}	{}	PANIC_ACTION", name, sn); continue; } };
        let e = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| a.effect())) {
            Ok(e) => e, Err(_) => { println!("{}	{}	PANIC_EFFECT", name, sn); continue; } };
        match e {
            None => println!("{}\t{}\tNO_EFFECT\t-\t-\t-\t-\t-", name, sn),
            Some(eff) => {
                // version 무관성 검사: 1~5 전부
                let mut v: Vec<String> = Vec::new();
                for ver in 1usize..=5 {
                    let r = game_ai::effect_cc_time(ver, &eff);
                    v.push(match r { Some(t) => format!("{}", t), None => "None".to_string() });
                }
                let same = v.iter().all(|x| *x == v[0]);
                println!("{}	{}	OK	cc={}	ver_same={}	vers={}	casting={:?}	range={}",
                    name, sn, v[0], same, v.join(","), eff.casting, eff.range);
            }
        }
    }
}

macro_rules! P { ($($t:ident),*) => { $( probe(stringify!($t), &$t::default()); )* } }

fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    println!("champ	slot	state	cc	ver_same	vers	casting	range");
    P!(AndroidChampionInfo, ArcherChampionInfo, BardChampionInfo, BarrierMagicianChampionInfo,
       BerserkerChampionInfo, BomberChampionInfo, BoomerangHunterChampionInfo, CavalryKnightChampionInfo,
       ChefChampionInfo, CircusBladeChampionInfo, ClownChampionInfo, DancerChampionInfo,
       DarkMageChampionInfo, DemonChampionInfo, DokkaebiChampionInfo,
       DruidChampionInfo, DualBladerChampionInfo, EnchanterChampionInfo, ExecutionerChampionInfo,
       ExorcistChampionInfo, FighterChampionInfo, GamblerChampionInfo, GhostChampionInfo,
       GuardianSpiritChampionInfo, GunnerChampionInfo, HammererChampionInfo, HitmanChampionInfo,
       HunterChampionInfo, IceMageChampionInfo, IllusionistChampionInfo, InquisitorChampionInfo,
       JiangshiChampionInfo, KnightChampionInfo, LancerChampionInfo, LightningMageChampionInfo,
       MagicKnightChampionInfo, MonkChampionInfo, NecromancerChampionInfo, NinjaChampionInfo,
       OgreChampionInfo, PlagueDoctorChampionInfo, PoisonDartHunterChampionInfo, PoleWarriorChampionInfo,
       PriestChampionInfo, PrisonerChampionInfo, PyromancerChampionInfo, PythonessChampionInfo,
       ShadowmancerChampionInfo, ShieldBearerChampionInfo, SiegeBreakerChampionInfo, SoldierChampionInfo,
       SpiritCallerChampionInfo, SwordmanChampionInfo, TaoistChampionInfo, VampireChampionInfo,
       VoodooShamanChampionInfo, WerewolfChampionInfo, WhipMasterChampionInfo, WhiteMageChampionInfo,
       WindMageChampionInfo);
}
