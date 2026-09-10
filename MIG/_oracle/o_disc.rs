#![allow(unused, dead_code)]
// 열거형 실제 메모리 태그(판별자) 실측 — 선언 순서와 다를 수 있음(니치/명시 판별자)
use game_core::*;
use game_ai::plan_legacy::team_plan::{MainObjective as MO, SubObjectiveType as SOT, ObjectPhase as OP};
use game_ai::plan_legacy::steal::StealAction as SA;

fn tag<T>(v: &T) -> u64 {
    // 첫 바이트를 태그로 읽는다(모두 1바이트 태그 열거형인지 size 로 함께 확인)
    unsafe { *(v as *const T as *const u8) as u64 }
}
macro_rules! p { ($g:expr, $($e:expr),+ $(,)?) => { $( println!("{}\t{}\t{}\t{}", $g, stringify!($e), tag(&$e), std::mem::size_of_val(&$e)); )+ } }

fn main() {
    p!("TutorialType", TutorialType::None, TutorialType::First, TutorialType::TopSolo,
       TutorialType::Bottom, TutorialType::MidSolo, TutorialType::MidBottom,
       TutorialType::JungleOnly, TutorialType::Line, TutorialType::Total);
    p!("LineType", LineType::Mid, LineType::Bottom, LineType::Top);
    p!("Position", Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support);
    p!("JungleType", JungleType::Rhino, JungleType::Mushroom, JungleType::Stump,
       JungleType::Bee, JungleType::Morgard, JungleType::Serpen);
    p!("StealTarget", StealTarget::Epic, StealTarget::Serpen);
    p!("BigGoal", BigGoal::Line{line:LineType::Top}, BigGoal::Jungle{camp:JungleType::Rhino, team:0},
       BigGoal::Epic, BigGoal::Serpen, BigGoal::Nexus{team:0}, BigGoal::Battle{focus:None}, BigGoal::Recall);
    p!("StealAction", SA::None, SA::Lurk(StealTarget::Epic), SA::Commit(StealTarget::Epic));
    p!("ObjectPhase", OP::Setup, OP::Assemble, OP::Hunt, OP::None);
    p!("Chat", Chat::Start(0), Chat::Mia(Position::Top,0), Chat::JungleCheck(LineType::Top,0),
       Chat::CounterJungle(JungleType::Rhino,0), Chat::Split(LineType::Top,0),
       Chat::PlayCall(0,0,0), Chat::PlayPropose(0,0,0,0), Chat::GankPlan(LineType::Top,0,0),
       Chat::EarlyPlan(0,0), Chat::CounterRequest(0,0,0), Chat::ReadySignal(0,0));
    println!("SIZE\tGameContext\t{}\t-", std::mem::size_of::<GameContext>());
    println!("SIZE\tChat\t{}\t-", std::mem::size_of::<Chat>());
    println!("SIZE\tBigGoal\t{}\t-", std::mem::size_of::<BigGoal>());
    println!("SIZE\tBigPlan\t{}\t-", std::mem::size_of::<game_ai::plan_legacy::types::BigPlan>());
}
