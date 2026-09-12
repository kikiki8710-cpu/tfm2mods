#![allow(unused, dead_code, non_snake_case)]
use game_core::*;

fn t1(g: &Game) {
    let _: () = g.world.entity;
}
fn t2(e: &Entity) {
    let _: () = e.ult_effect;
    let _: () = e.level;
    let _: () = e.stat_buff_cached;
    let _: () = e.cc;
}
fn t3(ef: &Effect) {
    let _: () = ef.casting;
    let _: () = ef.target;
    let _: () = ef.ty;
    let _: () = ef.attack_type;
    let _: () = ef.start_timing;
}
fn t4(s: &ScoreParameter) {
    let _: () = s.version;
    let _: () = s.near_allies;
    let _: () = s.near_enemies;
    let _: () = s.player;
    let _: () = s.positioning_score;
    let _: () = s.wave_snapshot;
    let _: () = s.v3_turnback_hold;
}
fn t5(p: &PositioningScoreData) {
    let _: () = p.cx;
    let _: () = p.value;
}
fn main() {}
