#![allow(unused,dead_code,non_snake_case)]
use game_core::*;
fn probe_BigGoal(x:&game_core::BigGoal){ match x {
  game_core::BigGoal::Line{line: a0} => { /*V BigGoal::Line*/ let _:()=*a0; }
  game_core::BigGoal::Jungle{camp: a0,team: a1} => { /*V BigGoal::Jungle*/ let _:()=*a0; let _:()=*a1; }
  game_core::BigGoal::Epic => { /*V BigGoal::Epic*/  }
  game_core::BigGoal::Serpen => { /*V BigGoal::Serpen*/  }
  game_core::BigGoal::Nexus{team: a0} => { /*V BigGoal::Nexus*/ let _:()=*a0; }
  game_core::BigGoal::Battle{focus: a0} => { /*V BigGoal::Battle*/ let _:()=*a0; }
  game_core::BigGoal::Recall => { /*V BigGoal::Recall*/  }
} }
fn probe_MainObjective(x:&game_ai::plan_legacy::team_plan::MainObjective){ match x {
  game_ai::plan_legacy::team_plan::MainObjective::Morgard{phase: a0,with_battle: a1} => { /*V MainObjective::Morgard*/ let _:()=*a0; let _:()=*a1; }
  game_ai::plan_legacy::team_plan::MainObjective::Serpen{phase: a0,with_battle: a1} => { /*V MainObjective::Serpen*/ let _:()=*a0; let _:()=*a1; }
  game_ai::plan_legacy::team_plan::MainObjective::Defense => { /*V MainObjective::Defense*/  }
  game_ai::plan_legacy::team_plan::MainObjective::DefenseLine(a0) => { /*V MainObjective::DefenseLine*/ let _:()=*a0; }
  game_ai::plan_legacy::team_plan::MainObjective::Nexus(a0) => { /*V MainObjective::Nexus*/ let _:()=*a0; }
  game_ai::plan_legacy::team_plan::MainObjective::PressEpic(a0) => { /*V MainObjective::PressEpic*/ let _:()=*a0; }
  game_ai::plan_legacy::team_plan::MainObjective::SplitEpic(a0) => { /*V MainObjective::SplitEpic*/ let _:()=*a0; }
  game_ai::plan_legacy::team_plan::MainObjective::Repair => { /*V MainObjective::Repair*/  }
  game_ai::plan_legacy::team_plan::MainObjective::Gank{line: a0} => { /*V MainObjective::Gank*/ let _:()=*a0; }
  game_ai::plan_legacy::team_plan::MainObjective::Dive{line: a0} => { /*V MainObjective::Dive*/ let _:()=*a0; }
  game_ai::plan_legacy::team_plan::MainObjective::PressTower{line: a0} => { /*V MainObjective::PressTower*/ let _:()=*a0; }
  game_ai::plan_legacy::team_plan::MainObjective::ComebackPick{line: a0,ready: a1} => { /*V MainObjective::ComebackPick*/ let _:()=*a0; let _:()=*a1; }
} }
fn probe_SubObjectiveType(x:&game_ai::plan_legacy::team_plan::SubObjectiveType){ match x {
  game_ai::plan_legacy::team_plan::SubObjectiveType::LineBattle{line: a0,do_dive: a1} => { /*V SubObjectiveType::LineBattle*/ let _:()=*a0; let _:()=*a1; }
  game_ai::plan_legacy::team_plan::SubObjectiveType::JungleBattle{camp: a0,team: a1} => { /*V SubObjectiveType::JungleBattle*/ let _:()=*a0; let _:()=*a1; }
} }
fn probe_BigPlan(x:&game_ai::plan_legacy::types::BigPlan){ match x {
  game_ai::plan_legacy::types::BigPlan::ForcePassive => { /*V BigPlan::ForcePassive*/  }
  game_ai::plan_legacy::types::BigPlan::PassiveLine(a0) => { /*V BigPlan::PassiveLine*/ let _:()=*a0; }
  game_ai::plan_legacy::types::BigPlan::SinglePlanLine(a0) => { /*V BigPlan::SinglePlanLine*/ let _:()=*a0; }
  game_ai::plan_legacy::types::BigPlan::SinglePlanBattle(a0) => { /*V BigPlan::SinglePlanBattle*/ let _:()=*a0; }
  game_ai::plan_legacy::types::BigPlan::DeathMatchBattle(a0) => { /*V BigPlan::DeathMatchBattle*/ let _:()=*a0; }
  game_ai::plan_legacy::types::BigPlan::PassiveJungle(a0) => { /*V BigPlan::PassiveJungle*/ let _:()=*a0; }
  game_ai::plan_legacy::types::BigPlan::ActiveRecall(a0) => { /*V BigPlan::ActiveRecall*/ let _:()=*a0; }
  game_ai::plan_legacy::types::BigPlan::Battle(a0) => { /*V BigPlan::Battle*/ let _:()=*a0; }
  game_ai::plan_legacy::types::BigPlan::LineGanker(a0) => { /*V BigPlan::LineGanker*/ let _:()=*a0; }
  game_ai::plan_legacy::types::BigPlan::LineGankCover(a0) => { /*V BigPlan::LineGankCover*/ let _:()=*a0; }
  game_ai::plan_legacy::types::BigPlan::EpicHuntAndPoke(a0) => { /*V BigPlan::EpicHuntAndPoke*/ let _:()=*a0; }
  game_ai::plan_legacy::types::BigPlan::EpicHuntAndBattle(a0) => { /*V BigPlan::EpicHuntAndBattle*/ let _:()=*a0; }
  game_ai::plan_legacy::types::BigPlan::SerpenHuntAndPoke(a0) => { /*V BigPlan::SerpenHuntAndPoke*/ let _:()=*a0; }
  game_ai::plan_legacy::types::BigPlan::SerpenHuntAndBattle(a0) => { /*V BigPlan::SerpenHuntAndBattle*/ let _:()=*a0; }
  game_ai::plan_legacy::types::BigPlan::AttackNexus(a0) => { /*V BigPlan::AttackNexus*/ let _:()=*a0; }
  game_ai::plan_legacy::types::BigPlan::DefenseNexus(a0) => { /*V BigPlan::DefenseNexus*/ let _:()=*a0; }
} }
fn probe_Chat(x:&game_core::Chat){ match x {
  game_core::Chat::Start(a0) => { /*V Chat::Start*/ let _:()=*a0; }
  game_core::Chat::Mia(a0,a1) => { /*V Chat::Mia*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::JungleCheck(a0,a1) => { /*V Chat::JungleCheck*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::Battle(a0,a1) => { /*V Chat::Battle*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::BattleDive(a0,a1) => { /*V Chat::BattleDive*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::BattleLine(a0,a1) => { /*V Chat::BattleLine*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::BattleHelp(a0,a1) => { /*V Chat::BattleHelp*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::BattleStop(a0) => { /*V Chat::BattleStop*/ let _:()=*a0; }
  game_core::Chat::GankRequest(a0,a1) => { /*V Chat::GankRequest*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::CoverLine(a0,a1) => { /*V Chat::CoverLine*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::GankLineCover(a0,a1) => { /*V Chat::GankLineCover*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::HideLine(a0,a1) => { /*V Chat::HideLine*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::HideLineToo(a0,a1) => { /*V Chat::HideLineToo*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::LineCover(a0,a1) => { /*V Chat::LineCover*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::DefenseLine(a0,a1) => { /*V Chat::DefenseLine*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::Ok(a0) => { /*V Chat::Ok*/ let _:()=*a0; }
  game_core::Chat::Reject(a0) => { /*V Chat::Reject*/ let _:()=*a0; }
  game_core::Chat::Cancel(a0) => { /*V Chat::Cancel*/ let _:()=*a0; }
  game_core::Chat::CounterJungle(a0,a1) => { /*V Chat::CounterJungle*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::Lead(a0) => { /*V Chat::Lead*/ let _:()=*a0; }
  game_core::Chat::Split(a0,a1) => { /*V Chat::Split*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::Press(a0,a1) => { /*V Chat::Press*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::PressChange(a0,a1) => { /*V Chat::PressChange*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::Repair(a0) => { /*V Chat::Repair*/ let _:()=*a0; }
  game_core::Chat::SerpenPrepare(a0,a1) => { /*V Chat::SerpenPrepare*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::SerpenSetup(a0) => { /*V Chat::SerpenSetup*/ let _:()=*a0; }
  game_core::Chat::SerpenCheck(a0) => { /*V Chat::SerpenCheck*/ let _:()=*a0; }
  game_core::Chat::SerpenEnemyHunt(a0) => { /*V Chat::SerpenEnemyHunt*/ let _:()=*a0; }
  game_core::Chat::SerpenAssemble(a0) => { /*V Chat::SerpenAssemble*/ let _:()=*a0; }
  game_core::Chat::SerpenHunt(a0) => { /*V Chat::SerpenHunt*/ let _:()=*a0; }
  game_core::Chat::SerpenBattle(a0) => { /*V Chat::SerpenBattle*/ let _:()=*a0; }
  game_core::Chat::SerpenGiveUp(a0) => { /*V Chat::SerpenGiveUp*/ let _:()=*a0; }
  game_core::Chat::SerpenSteal(a0) => { /*V Chat::SerpenSteal*/ let _:()=*a0; }
  game_core::Chat::MorgardPrepare(a0,a1) => { /*V Chat::MorgardPrepare*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::MorgardSetup(a0) => { /*V Chat::MorgardSetup*/ let _:()=*a0; }
  game_core::Chat::MorgardCheck(a0) => { /*V Chat::MorgardCheck*/ let _:()=*a0; }
  game_core::Chat::MorgardEnemyHunt(a0) => { /*V Chat::MorgardEnemyHunt*/ let _:()=*a0; }
  game_core::Chat::MorgardAssemble(a0) => { /*V Chat::MorgardAssemble*/ let _:()=*a0; }
  game_core::Chat::MorgardHunt(a0) => { /*V Chat::MorgardHunt*/ let _:()=*a0; }
  game_core::Chat::MorgardBattle(a0) => { /*V Chat::MorgardBattle*/ let _:()=*a0; }
  game_core::Chat::MorgardGiveUp => { /*V Chat::MorgardGiveUp*/  }
  game_core::Chat::MorgardSteal(a0) => { /*V Chat::MorgardSteal*/ let _:()=*a0; }
  game_core::Chat::AttackNexus(a0,a1) => { /*V Chat::AttackNexus*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::DefenseNexus(a0) => { /*V Chat::DefenseNexus*/ let _:()=*a0; }
  game_core::Chat::GankDive(a0,a1) => { /*V Chat::GankDive*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::PressTower(a0,a1) => { /*V Chat::PressTower*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::ComebackPick(a0,a1) => { /*V Chat::ComebackPick*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::PlayCall(a0,a1,a2) => { /*V Chat::PlayCall*/ let _:()=*a0; let _:()=*a1; let _:()=*a2; }
  game_core::Chat::PlayPhaseChange(a0,a1,a2) => { /*V Chat::PlayPhaseChange*/ let _:()=*a0; let _:()=*a1; let _:()=*a2; }
  game_core::Chat::PlayPropose(a0,a1,a2,a3) => { /*V Chat::PlayPropose*/ let _:()=*a0; let _:()=*a1; let _:()=*a2; let _:()=*a3; }
  game_core::Chat::GankPlan(a0,a1,a2) => { /*V Chat::GankPlan*/ let _:()=*a0; let _:()=*a1; let _:()=*a2; }
  game_core::Chat::EarlyPlan(a0,a1) => { /*V Chat::EarlyPlan*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::EarlyPlanChange(a0,a1) => { /*V Chat::EarlyPlanChange*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::AcceptAfterTask(a0,a1) => { /*V Chat::AcceptAfterTask*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::DeclineWithReason(a0,a1) => { /*V Chat::DeclineWithReason*/ let _:()=*a0; let _:()=*a1; }
  game_core::Chat::CounterRequest(a0,a1,a2) => { /*V Chat::CounterRequest*/ let _:()=*a0; let _:()=*a1; let _:()=*a2; }
  game_core::Chat::ReadySignal(a0,a1) => { /*V Chat::ReadySignal*/ let _:()=*a0; let _:()=*a1; }
} }
fn probe_SubObjective(x:&game_ai::plan_legacy::team_plan::SubObjective){ let _:()=x.ty; let _:()=x.tick; }
fn main(){}
