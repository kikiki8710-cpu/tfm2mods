extern crate engine_ui;
use engine_ui::runner as R;
fn fields(r:&R::ColorRunner){ let _ = r.style.normal.color.__nope__; }
fn rtype(r:&R::ColorRunner){ let _:()=r.style.normal.color.r; }   // guess component
fn main(){}
