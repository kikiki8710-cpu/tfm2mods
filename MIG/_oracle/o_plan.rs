#![allow(unused, dead_code, non_snake_case)]
// 보충 오라클: plan_allowed(구성 가능한 BigPlan 변형만) + chat 49 PlayPropose 3번째 u8 확장(0..7)
use game_core::*;
use game_ai::plan_legacy::rule_scope as rs;
use game_ai::plan_legacy::types::BigPlan as BP;
use game_ai::plan_legacy::old::*;
use rand::SeedableRng;

fn row(f: &str, inp: &str, out: &str) { println!("{}\t{}\t{}", f, inp, out); }

fn main() {
    let pool = bumpalo::Bump::new();
    let setting: GameSetting = Default::default();
    let mw: MacroWeights = Default::default();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let tuts = [TutorialType::None, TutorialType::First, TutorialType::TopSolo,
                TutorialType::Bottom, TutorialType::MidSolo, TutorialType::MidBottom,
                TutorialType::JungleOnly, TutorialType::Line, TutorialType::Total];
    let tnames = ["None","First","TopSolo","Bottom","MidSolo","MidBottom","JungleOnly","Line","Total"];
    let lines = [LineType::Mid, LineType::Bottom, LineType::Top];
    let lnames = ["Mid","Bottom","Top"];
    let jts = [JungleType::Rhino, JungleType::Mushroom, JungleType::Stump,
               JungleType::Bee, JungleType::Morgard, JungleType::Serpen];
    let jnames = ["Rhino","Mushroom","Stump","Bee","Morgard","Serpen"];

    for ti in 0..9usize {
        let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
            map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
            tutorial: tuts[ti], trace_level: TraceLevel::Off };
        let tn = tnames[ti];

        row("plan_allowed", &format!("tut={};plan=ForcePassive", tn),
            &format!("{}", rs::plan_allowed(&ctx, &BP::ForcePassive)));
        row("plan_allowed", &format!("tut={};plan=ActiveRecall", tn),
            &format!("{}", rs::plan_allowed(&ctx, &BP::ActiveRecall(Default::default()))));
        row("plan_allowed", &format!("tut={};plan=EpicHuntAndPoke", tn),
            &format!("{}", rs::plan_allowed(&ctx, &BP::EpicHuntAndPoke(Default::default()))));
        row("plan_allowed", &format!("tut={};plan=EpicHuntAndBattle", tn),
            &format!("{}", rs::plan_allowed(&ctx, &BP::EpicHuntAndBattle(Default::default()))));
        row("plan_allowed", &format!("tut={};plan=SerpenHuntAndPoke", tn),
            &format!("{}", rs::plan_allowed(&ctx, &BP::SerpenHuntAndPoke(Default::default()))));
        row("plan_allowed", &format!("tut={};plan=SerpenHuntAndBattle", tn),
            &format!("{}", rs::plan_allowed(&ctx, &BP::SerpenHuntAndBattle(Default::default()))));
        row("plan_allowed", &format!("tut={};plan=AttackNexus(default)", tn),
            &format!("{}", rs::plan_allowed(&ctx, &BP::AttackNexus(Default::default()))));
        row("plan_allowed", &format!("tut={};plan=DefenseNexus(default)", tn),
            &format!("{}", rs::plan_allowed(&ctx, &BP::DefenseNexus(Default::default()))));

        for li in 0..3usize {
            row("plan_allowed", &format!("tut={};plan=PassiveLine({})", tn, lnames[li]),
                &format!("{}", rs::plan_allowed(&ctx, &BP::PassiveLine(PassiveLinePlan::new(lines[li])))));
            row("plan_allowed", &format!("tut={};plan=SinglePlanLine({})", tn, lnames[li]),
                &format!("{}", rs::plan_allowed(&ctx, &BP::SinglePlanLine(SinglePlanLine::new(lines[li])))));
            row("plan_allowed", &format!("tut={};plan=LineGanker({})", tn, lnames[li]),
                &format!("{}", rs::plan_allowed(&ctx, &BP::LineGanker(LineGankerPlan::new(lines[li], 0, 0)))));
            row("plan_allowed", &format!("tut={};plan=LineGankCover({})", tn, lnames[li]),
                &format!("{}", rs::plan_allowed(&ctx, &BP::LineGankCover(LineGankCoverPlan::new(lines[li], 0)))));
        }
        for ji in 0..6usize {
            for team in 0..2usize {
                let mut rng = rand::rngs::StdRng::seed_from_u64(1);
                let mut pj = PassiveJunglePlan::new(&mut rng, team);
                pj.jungle = jts[ji];
                pj.team = team;
                row("plan_allowed", &format!("tut={};plan=PassiveJungle({},{})", tn, jnames[ji], team),
                    &format!("{}", rs::plan_allowed(&ctx, &BP::PassiveJungle(pj))));
            }
        }

        // chat 49 PlayPropose: 3번째 u8(c)을 0..7 로 확장해 무영향 확인
        for a in 0..8u8 { for b in 0..8u8 { for c in 0..8u8 {
            row("chat_allowed", &format!("tut={};chat=49:PlayPropose;a={},b={},c={}", tn, a, b, c),
                &format!("{}", rs::chat_allowed(&ctx, &Chat::PlayPropose(a, b, c, 0))));
        }}}
        // chat 50 GankPlan: u8 을 0..15 로 확장
        for li in 0..3usize { for a in 0..16u8 {
            row("chat_allowed", &format!("tut={};chat=50:GankPlan;line={},a={}", tn, lnames[li], a),
                &format!("{}", rs::chat_allowed(&ctx, &Chat::GankPlan(lines[li], a, 0))));
        }}
        // chat 47/51/55: u8 코드를 0..15 로 확장(경계 확인)
        for a in 0..16u8 { for b in 0..16u8 {
            row("chat_allowed", &format!("tut={};chat=47:PlayCall;a={},b={}", tn, a, b),
                &format!("{}", rs::chat_allowed(&ctx, &Chat::PlayCall(a, b, 0))));
            row("chat_allowed", &format!("tut={};chat=55:CounterRequest;a={},b={}", tn, a, b),
                &format!("{}", rs::chat_allowed(&ctx, &Chat::CounterRequest(a, b, 0))));
        }}
        for a in 0..16u8 {
            row("chat_allowed", &format!("tut={};chat=51:EarlyPlan;a={}", tn, a),
                &format!("{}", rs::chat_allowed(&ctx, &Chat::EarlyPlan(a, 0))));
            row("chat_allowed", &format!("tut={};chat=52:EarlyPlanChange;a={}", tn, a),
                &format!("{}", rs::chat_allowed(&ctx, &Chat::EarlyPlanChange(a, 0))));
        }
    }
}
