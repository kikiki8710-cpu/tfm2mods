//! 게임을 실행하지 않고 실제 산술 콜백을 호출하는 독립 검사 실행기.
#![allow(dead_code)]
use std::io::{self, BufRead};
mod micro {
    pub(crate) struct Frame { pub regs:[u64;16], pub flags:u64 }
    impl Frame {
        pub(crate) fn reg(&self,r:u8)->Option<u64>{ self.regs.get(r as usize).copied() }
        pub(crate) fn flags(&self)->u64{self.flags}
        pub(crate) fn carry(&self)->bool{self.flags&1!=0}
    }
}
#[path="../src/extra_combat.rs"] mod extra_combat;
fn main() {
    for line in io::stdin().lock().lines() {
        let line=line.expect("검사 입력");
        let fields:Vec<_>=line.split_ascii_whitespace().collect();
        assert_eq!(fields.len(),19);
        let value=fields[1].parse::<i64>().expect("값");
        let flags=u64::from_str_radix(fields[2],16).expect("flags");
        let mut regs=[0;16];
        for k in 0..16 {regs[k]=u64::from_str_radix(fields[3+k],16).expect("GPR");}
        let f=micro::Frame{regs,flags};
        let fun:fn(&micro::Frame,i64)->Option<u64>=match fields[0] {
            "tower_hits"=>extra_combat::tower_hits,
            "chase_ratio_rdx"=>extra_combat::chase_ratio_rdx,
            "chase_ratio_rcx"=>extra_combat::chase_ratio_rcx,
            "radius_square"=>extra_combat::radius_square,
            "radius_square_minus_one"=>extra_combat::radius_square_minus_one,
            "trade_hp_divisor"=>extra_combat::trade_hp_divisor,
            "heal_r8"=>extra_combat::heal_r8,
            "shield_r8"=>extra_combat::shield_r8,
            "heal_r14"=>extra_combat::heal_r14,
            "shield_r14"=>extra_combat::shield_r14,
            "buff_arrival_ticks"=>extra_combat::buff_arrival_ticks,
            "isolated_percent"=>extra_combat::isolated_percent,
            "front_ally_distance_score"=>extra_combat::front_ally_distance_score,
            "front_ally_cap"=>extra_combat::front_ally_cap,
            "front_ally_total_cap"=>extra_combat::front_ally_total_cap,
            "wave_risk_divisor"=>extra_combat::wave_risk_divisor,
            "risk_retreat_high"=>extra_combat::risk_retreat_high,
            "risk_retreat_pressure"=>extra_combat::risk_retreat_pressure,
            "unseen_risk_divisor"=>extra_combat::unseen_risk_divisor,
            "relocate_inner"=>extra_combat::relocate_inner,
            "relocate_mid"=>extra_combat::relocate_mid,
            "relocate_outer"=>extra_combat::relocate_outer,
            "relocate_self_near"=>extra_combat::relocate_self_near,
            "relocate_self_far"=>extra_combat::relocate_self_far,
            "relocate_mid_result"=>extra_combat::relocate_mid_result,
            "relocate_outer_result"=>extra_combat::relocate_outer_result,
            "candidate_move_budget"=>extra_combat::candidate_move_budget,
            "forward_credit_percent"=>extra_combat::forward_credit_percent,
            "move_penalty_negative"=>extra_combat::move_penalty_negative,
            "move_penalty_result"=>extra_combat::move_penalty_result,
            _=>panic!("알 수 없는 콜백"),
        };
        match fun(&f,value) {Some(v)=>println!("{v:016x}"),None=>println!("none")}
    }
}
