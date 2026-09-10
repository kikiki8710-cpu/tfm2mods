#![allow(unused, dead_code, non_snake_case)]
// 배치 B — 05/06 오라클 도달성 실측(막히는지 "확인"하는 것이 목적. 컴파일 실패가 정상 결과)
use game_ai::plan_legacy::handler::LegacyPlanHandler;

fn main() {
    let mut h: LegacyPlanHandler = unsafe { std::mem::zeroed() };
    // 05
    h.v50_fold_dive_episode(0usize, 60usize, false, 0u8);
    // 06
    // h.v2_response_retreat_stance(...) — 인자 구성 전에 가시성부터 확인
    let _ = LegacyPlanHandler::v2_response_retreat_stance;
}
