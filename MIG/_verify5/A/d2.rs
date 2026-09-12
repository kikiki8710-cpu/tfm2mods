#![allow(unused, dead_code, non_snake_case)]
use game_core::*;
fn t1(p: &PlayerState) {
    let _: () = p.info;
}
fn t2(g: &Game) {
    let _: () = g.world;
    let _: () = g.mode;
}
fn t3(c: &AbstractGameWithCache) {
    let _: () = c.player_champion;
    let _: () = c.twin_towers;
    let _: () = c.top_tower;
}
fn t4(e: &Entity) {
    let _: () = e.visible_state;
    let _: () = e.team;
    let _: () = e.ult;
    let _: () = e.stat_cached;
}
fn main() {}
