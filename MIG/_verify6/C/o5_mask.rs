#![allow(unused, dead_code, non_snake_case)]
//! C6 프로브 ⑤ — `specs[10]` 진입 마스크 `(mo & 65534) == 768` 를 **날바이트로** 검증한다.
//! (reused: 오라클 실행 + 구조체 바이트 덤프 ⓒ)
//!
//! 명세 주장:
//!   mem[0] MainObjective+0x0 = tag / mem[1] +0x1 = phase(ObjectPhase) / mem[2] +0x2 = with_battle
//!   consts[0] 65534 = 0x00FFFE 마스크 · consts[1] 768 = 0x000300 기대값
//!   ⟹ 통과 조건 = tag ∈ {0 Morgard, 1 Serpen} ∧ phase == 3(Hunt) ∧ with_battle 무관
//! 검증 = 실제 값들을 만들어 3바이트를 읽고, 술어가 위 조건과 **정확히 같은 집합**을 내는지 본다.
use game_ai::plan_legacy::team_plan::{MainObjective, ObjectPhase};
use game_core::*;

fn raw3(mo: &MainObjective) -> (u8, u8, u8) {
    unsafe {
        let p = mo as *const _ as *const u8;
        (*p, *p.add(1), *p.add(2))
    }
}

fn main() {
    println!("size_of::<MainObjective>()\t{}", std::mem::size_of::<MainObjective>());
    println!("\n### O6-E MainObjective 3바이트 + 마스크 술어");
    println!("name\tb0(tag)\tb1(phase)\tb2(with_battle)\tv24\t(v&65534)\tGATE");
    let phases = [
        ("Setup", ObjectPhase::Setup),
        ("Assemble", ObjectPhase::Assemble),
        ("Hunt", ObjectPhase::Hunt),
    ];
    let mut gate_pass: Vec<String> = Vec::new();
    let mut show = |name: String, mo: MainObjective, gp: &mut Vec<String>| {
        let (b0, b1, b2) = raw3(&mo);
        let v = (b0 as u32) | ((b1 as u32) << 8) | ((b2 as u32) << 16);
        let gate = (v & 65534) == 768;
        if gate { gp.push(name.clone()) }
        println!("{}\t{}\t{}\t{}\t{}\t{}\t{}", name, b0, b1, b2, v, v & 65534, gate);
    };
    for (pn, p) in phases {
        for wb in [false, true] {
            show(format!("Morgard({},{})", pn, wb),
                 MainObjective::Morgard { phase: p, with_battle: wb }, &mut gate_pass);
            show(format!("Serpen({},{})", pn, wb),
                 MainObjective::Serpen { phase: p, with_battle: wb }, &mut gate_pass);
        }
    }
    let others: Vec<(&str, MainObjective)> = vec![
        ("Defense", MainObjective::Defense),
        ("DefenseLine(Top)", MainObjective::DefenseLine(LineType::Top)),
        ("DefenseLine(Mid)", MainObjective::DefenseLine(LineType::Mid)),
        ("DefenseLine(Bottom)", MainObjective::DefenseLine(LineType::Bottom)),
        ("Nexus(Top)", MainObjective::Nexus(LineType::Top)),
        ("PressEpic(Top)", MainObjective::PressEpic(LineType::Top)),
        ("SplitEpic(Top)", MainObjective::SplitEpic(LineType::Top)),
        ("Repair", MainObjective::Repair),
        ("Gank(Top)", MainObjective::Gank { line: LineType::Top }),
        ("Dive(Top)", MainObjective::Dive { line: LineType::Top }),
        ("PressTower(Top)", MainObjective::PressTower { line: LineType::Top }),
        ("ComebackPick(Top,f)", MainObjective::ComebackPick { line: LineType::Top, ready: false }),
        ("ComebackPick(Top,t)", MainObjective::ComebackPick { line: LineType::Top, ready: true }),
    ];
    for (n, o) in others {
        show(n.to_string(), o, &mut gate_pass);
    }
    println!("\nGATE 통과 집합 ({}개): {}", gate_pass.len(), gate_pass.join(", "));
    println!("기대 = Morgard(Hunt,*) · Serpen(Hunt,*) 4개");

    // Strategy 기본값 + object_finish 바이트 위치
    println!("\n### O6-F Strategy 기본값 / object_finish(+0xf)");
    let st: Strategy = Default::default();
    unsafe {
        let p = &st as *const _ as *const u8;
        let bytes: Vec<String> = (0..24).map(|i| format!("{:02x}", *p.add(i))).collect();
        println!("raw24\t{}", bytes.join(" "));
        println!("+0xf\t{}\t(0 이면 KillPriority)", *p.add(15));
    }
    println!("Debug\t{:?}", st);
    println!("\nDONE");
}
