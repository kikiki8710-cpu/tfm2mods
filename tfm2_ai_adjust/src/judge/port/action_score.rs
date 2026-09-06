//! action_score — 기저 스코어러 `0xd57540`(action_score.rs:617~932, 1,988명령) 포팅 자리. 2026-09-06 착수(RE 진행 중).
//!   호출 = 각 SubPlan 스코어러가 tail-call. 반환 i64 점수. 검증 훅 = `judge_scorer_cmp!`(원본 먼저 → 재현 → i64 대조, mod.rs).
//!   큰 콜리는 1단계에서 캡처 링(`judge_capture_ring!`)으로 값을 받아 본체 구조부터 대조하고, 2단계에서 콜리를 하나씩 순수 포팅한다(obj_helpers 방식).
use crate::*;
use super::super::ScorerArgs;

/// 아직 미구현 — None(NA). RE 반환 후 채운다.
pub unsafe fn base_score(_a: &ScorerArgs) -> Option<i64> { None }
