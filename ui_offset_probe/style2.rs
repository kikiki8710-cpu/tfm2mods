extern crate engine_ui;
use engine_ui::runner as R;
fn f1(r: &R::ColorRunner) { let _ = r.style.__nope__; }
fn f2(r: &R::ColorRunner) { let _ = r.style.color.__nope__; }
fn f3(r: &R::LabelRunner) { let _ = r.style.color.__nope__; }
fn main() {}
