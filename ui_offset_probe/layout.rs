use mod_api::*;
fn fields(n:&Node){ let _ = n.layout.normal.__nope__; }   // Layout 필드 목록
fn x_ok(n:&mut Node){ n.layout.normal.x = 1.0; }          // x 직접 쓰기 가능?
fn y_ok(n:&mut Node){ n.layout.normal.y = 1.0; }
fn main(){}
