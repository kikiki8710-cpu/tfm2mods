#![crate_type="lib"]
extern crate game_ai;
extern crate rand;
use game_ai::plan_legacy::old::{EpicHuntAndBattlePlan, EpicHuntAndPokePlan};
use game_ai::plan_legacy::sub_plan::SubPlan;
use game_ai::{PlayerState, OperationData, GoalData, DebugFrameData};
use rand::rngs::StdRng;
#[inline(never)]
pub fn s1(p:&EpicHuntAndBattlePlan, t:usize, r:&mut StdRng, ps:&PlayerState, od:&OperationData, gd:&GoalData, df:&mut DebugFrameData) -> SubPlan {
    p.sub_plan(t, r, ps, od, gd, df)
}
