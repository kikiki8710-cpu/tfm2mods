// gen_fns.rs — ★자동생성(python MIG\aiport.py gen). 손으로 고치지 말 것 — 정본 = MIG\judge\manifest.json
//   게임 0.5.8 · exe sha 4ed3aed08971efd0 · 2026-09-06 01:02
//   FnSpec.prolog = 훅이 옮기는 선두 바이트(명령 경계 ≥12B). 설치기는 exe 바이트가 이와 **완전 일치**할 때만 패치한다(패치판/스테일 방어).
#![allow(dead_code)]
pub const GAME_VER: &str = "0.5.8";
pub struct FnSpec { pub name: &'static str, pub sym: &'static str, pub role: &'static str, pub rva: usize, pub size: usize, pub prolog: &'static [u8], pub status: &'static str }
pub const STEAL_SCORE: FnSpec = FnSpec { name: "steal_score", sym: r"game-ai\src\plan_legacy\sub_plan\steal.rs", role: "scorer", rva: 0xcbbca0, size: 269, prolog: &[0x56, 0x48, 0x83, 0xec, 0x20, 0x48, 0x8b, 0x54, 0x24, 0x58, 0x0f, 0xb6, 0x41, 0x08], status: "ported" };   // 원본 행 27,28 · 70명령 · vslots 0x40,0x1f0
pub static ALL: &[&FnSpec] = &[&STEAL_SCORE];
