use mod_api::*;
fn nf(n:&Node){ let Node{..}=n; }                 // Node 전체 필드
fn lt(n:&Node){ let _:()=n.layout; }              // layout 타입
fn rt(n:&Node){ let _:()=n.rect; }                // rect 타입
fn main(){}
