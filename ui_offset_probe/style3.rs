extern crate engine_ui;
use engine_ui::runner as R;
fn c(r: &R::ColorRunner) { let _ = r.style.normal.__nope__; }
fn l(r: &R::LabelRunner) { let _ = r.style.normal.__nope__; }
fn i(r: &R::ImageRunner) { let _ = r.style.normal.__nope__; }
fn main() {}
