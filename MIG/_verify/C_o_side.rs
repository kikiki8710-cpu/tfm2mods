#![allow(unused,dead_code,non_snake_case)]
use game_core::*;
fn main(){
  let pool = bumpalo::Bump::new();
  let mut setting: GameSetting = Default::default();
  setting.width = 960000; setting.height = 960000;
  let mw: MacroWeights = Default::default();
  let ms: MapSetting = Default::default();
  let map = MapDef::moba(&setting);
  let champs: Vec<String> = Vec::new();
  let items: Vec<Box<dyn ItemInfo>> = Vec::new();
  let ctx = GameContext{ pool:&pool, setting:&setting, macro_weights:&mw, map_setting:&ms,
    map:&map, champion_list:&champs, item_list:&items, ignore_minion:false, debug:false,
    tutorial:TutorialType::None, trace_level:TraceLevel::Off };
  println!("META\twidth={}\theight={}", setting.width, setting.height);
  // --- polarity probe
  let pts: [(u64,u64);9] = [(0,0),(100000,100000),(80000,820000),(820000,80000),
                            (480000,480000),(959999,959999),(900000,0),(0,900000),(864000,96000)];
  for (x,y) in pts {
    println!("SIDE\tx={}\ty={}\tx+y={}\tis_top_side={}\tis_bottom_side={}\try_lt_x={}",
      x,y,x+y, is_top_side(&ctx,x,y), is_bottom_side(&ctx,x,y),
      (setting.height.wrapping_sub(y)) < x);
  }
  // --- bushes grid census
  let mut cnt = std::collections::BTreeMap::<usize,(usize,u64,u64)>::new();
  for yy in 0..30usize { for xx in 0..30usize {
    let v = map.bushes[yy][xx];
    if v==0 { continue; }
    let e = cnt.entry(v).or_insert((0,0,0));
    e.0 += 1; e.1 += xx as u64; e.2 += yy as u64;
  }}
  for (v,(n,sx,sy)) in &cnt {
    let cx = (*sx as f64)/(*n as f64)*32000.0+16000.0;
    let cy = (*sy as f64)/(*n as f64)*32000.0+16000.0;
    println!("BUSH\tid={}\tcells={}\tcx={:.0}\tcy={:.0}\tis_top_side={}", v, n, cx, cy,
      is_top_side(&ctx, cx as u64, cy as u64));
  }
  println!("NEXUS	{:?}	FOUNT	{:?}", map.nexus_pos, map.fountains);
  for t in 0..2usize { for l in [LineType::Top, LineType::Mid, LineType::Bottom] {
    println!("START	team={}	line={:?}	pos={:?}", t, l, l.get_start_position(&setting, t)); } }
  println!("BUSHIDS\tmin={:?}\tmax={:?}\tcount={}", cnt.keys().next(), cnt.keys().last(), cnt.len());
}
