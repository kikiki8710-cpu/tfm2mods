use mod_api::*;
// UIEvent 변형 전체 나열 (non_exhaustive면 숨겨질 수 있음)
fn ev(e: &UIEvent) { match e { } }
// Click 의 필드
fn cl(e: &UIEvent) { if let UIEvent::Click { } = e {} }
fn main(){}
