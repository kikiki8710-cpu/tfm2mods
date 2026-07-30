// Discover the Style sub-struct type + fields behind runner.style / .color etc.
extern crate engine_ui;
use engine_ui::runner as R;

// reveal type of `style` field
fn t_color(r: &R::ColorRunner) { let _: () = r.style; }
fn t_label(r: &R::LabelRunner) { let _: () = r.style; }
fn t_image(r: &R::ImageRunner) { let _: () = r.style; }

fn main() {}
