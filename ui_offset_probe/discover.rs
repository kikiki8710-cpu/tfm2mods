// Stage 1: discover runner module paths + field lists (compile-only probe).
// Empty-pattern destructure on &T forces rustc to list all fields (or report privacy).
extern crate engine_ui;

fn f_label(r: &engine_ui::runner::LabelRunner) { let engine_ui::runner::LabelRunner {} = r; }
fn f_image(r: &engine_ui::runner::ImageRunner) { let engine_ui::runner::ImageRunner {} = r; }
fn f_color(r: &engine_ui::runner::ColorRunner) { let engine_ui::runner::ColorRunner {} = r; }
fn f_button(r: &engine_ui::runner::ButtonRunner) { let engine_ui::runner::ButtonRunner {} = r; }
fn f_dropdown(r: &engine_ui::runner::DropdownRunner) { let engine_ui::runner::DropdownRunner {} = r; }
fn f_checkbox(r: &engine_ui::runner::CheckboxRunner) { let engine_ui::runner::CheckboxRunner {} = r; }
fn f_selectable(r: &engine_ui::runner::SelectableRunner) { let engine_ui::runner::SelectableRunner {} = r; }
fn f_colorselectable(r: &engine_ui::runner::ColorSelectableRunner) { let engine_ui::runner::ColorSelectableRunner {} = r; }
fn f_slider(r: &engine_ui::runner::SliderRunner) { let engine_ui::runner::SliderRunner {} = r; }
fn f_textedit(r: &engine_ui::runner::TextEditRunner) { let engine_ui::runner::TextEditRunner {} = r; }
fn f_scroll(r: &engine_ui::runner::ScrollViewRunner) { let engine_ui::runner::ScrollViewRunner {} = r; }
fn f_tree(r: &engine_ui::runner::TreeViewRunner) { let engine_ui::runner::TreeViewRunner {} = r; }
fn f_anim(r: &engine_ui::runner::AnimationRunner) { let engine_ui::runner::AnimationRunner {} = r; }
fn f_svg(r: &engine_ui::runner::SvgRunner) { let engine_ui::runner::SvgRunner {} = r; }
fn f_canvas(r: &engine_ui::runner::CanvasRunner) { let engine_ui::runner::CanvasRunner {} = r; }

fn main() {}
