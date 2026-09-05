//! passive_line 콜리 4종(census 밖 = game-core) 순수 재현 자리. ghidra-re 규명 결과로 채운다(2026-09-06 요청 중).
//!   0x1453260 (606B)  in_region(G, x, y) -> bool        — 좌표가 어떤 영역 안인가(미드 특수블록에서 적 5명 카운트 게이트)
//!   0x1323a00 (118B)  lane_pred(lane_ptr, data, vt, p5, ent) -> u8 — 적 카운트 술어
//!   0xdd9f30  (745B)  near_target(p7, p3, p4, p5, p6, tx, ty, 150000, p8) -> (arena vec, count) — 목표 주변 적 수(추정)
//!   0xe354d0  (282B)  nearest(iter) -> elem ptr — 레인 앵커에 가장 가까운 원소(추정)
//! 채워지기 전에는 None 을 돌려 부모 포팅이 NA 로 기권한다(그 경로만). 규명되면 여기만 고치면 된다.
#![allow(dead_code, unused_variables)]
use crate::*;

pub unsafe fn in_region(g: usize, x: u64, y: u64) -> Option<bool> { None }
pub unsafe fn lane_pred(lane_ptr: usize, data: usize, vt: usize, p5: usize, ent: usize) -> Option<u8> { None }
pub unsafe fn near_target_count(p7: usize, p3: usize, p4: usize, p5: usize, p6: usize, tx: u64, ty: u64, radius: u64, p8: usize) -> Option<u64> { None }
/// Some(None) = 원소 없음(게임: 넥서스 폴백) / None = 미규명(NA)
pub unsafe fn nearest_to_anchor(ptr: usize, len: u64, ax: u64, ay: u64) -> Option<Option<usize>> { None }
