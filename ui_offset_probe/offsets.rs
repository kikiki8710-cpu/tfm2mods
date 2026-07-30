// Stage 2: extract exact byte offsets + sizes via compile-time type errors.
// Each line errors "expected [();0], found [();N]" where N = the offset/size.
extern crate engine_ui;
use engine_ui::runner as R;
use std::mem::{offset_of, size_of, align_of};

macro_rules! off { ($tag:literal, $t:ty, $f:tt) => {
    const _: [(); 0] = [(); offset_of!($t, $f)];   // err reveals offset; tag in path comment
}; }
macro_rules! sz { ($t:ty) => {
    const _: [(); 0] = [(); size_of::<$t>()];
    const _: [(); 1] = [(); align_of::<$t>()];
}; }

// ---- sizes (size, then align) ----
sz!(R::LabelRunner); sz!(R::ImageRunner); sz!(R::ColorRunner); sz!(R::ButtonRunner);
sz!(R::DropdownRunner); sz!(R::CheckboxRunner); sz!(R::SelectableRunner);
sz!(R::ColorSelectableRunner); sz!(R::SliderRunner); sz!(R::TextEditRunner);
sz!(R::ScrollViewRunner); sz!(R::TreeViewRunner); sz!(R::AnimationRunner);
sz!(R::SvgRunner); sz!(R::CanvasRunner);

// ---- LabelRunner ----
off!("Label.style", R::LabelRunner, style);
off!("Label.text", R::LabelRunner, text);
off!("Label.decorate", R::LabelRunner, decorate);
off!("Label.z", R::LabelRunner, z);
off!("Label.fit_width", R::LabelRunner, fit_width);
off!("Label.fit_height", R::LabelRunner, fit_height);
off!("Label.binds", R::LabelRunner, binds);

// ---- ImageRunner ----
off!("Image.style", R::ImageRunner, style);
off!("Image.ignore_event", R::ImageRunner, ignore_event);
off!("Image.z", R::ImageRunner, z);

// ---- ColorRunner ----
off!("Color.style", R::ColorRunner, style);
off!("Color.ignore_event", R::ColorRunner, ignore_event);
off!("Color.mouse_hover_effect", R::ColorRunner, mouse_hover_effect);
off!("Color.z", R::ColorRunner, z);

// ---- ButtonRunner ----
off!("Button.style", R::ButtonRunner, style);
off!("Button.hover_sound", R::ButtonRunner, hover_sound);
off!("Button.click_sound", R::ButtonRunner, click_sound);
off!("Button.z", R::ButtonRunner, z);

// ---- DropdownRunner (only public) ----
off!("Dropdown.style", R::DropdownRunner, style);
off!("Dropdown.hover_sound", R::DropdownRunner, hover_sound);
off!("Dropdown.click_sound", R::DropdownRunner, click_sound);

// ---- CheckboxRunner ----
off!("Checkbox.style", R::CheckboxRunner, style);
off!("Checkbox.text", R::CheckboxRunner, text);
off!("Checkbox.selected", R::CheckboxRunner, selected);

// ---- SelectableRunner ----
off!("Selectable.image", R::SelectableRunner, image);
off!("Selectable.selected_image", R::SelectableRunner, selected_image);
off!("Selectable.label", R::SelectableRunner, label);
off!("Selectable.selected_label", R::SelectableRunner, selected_label);
off!("Selectable.hover_sound", R::SelectableRunner, hover_sound);
off!("Selectable.click_sound", R::SelectableRunner, click_sound);
off!("Selectable.text", R::SelectableRunner, text);
off!("Selectable.selected", R::SelectableRunner, selected);
off!("Selectable.hint", R::SelectableRunner, hint);

// ---- ColorSelectableRunner ----
off!("ColorSel.image", R::ColorSelectableRunner, image);
off!("ColorSel.selected_image", R::ColorSelectableRunner, selected_image);
off!("ColorSel.icon", R::ColorSelectableRunner, icon);
off!("ColorSel.selected_icon", R::ColorSelectableRunner, selected_icon);
off!("ColorSel.icon2", R::ColorSelectableRunner, icon2);
off!("ColorSel.selected_icon2", R::ColorSelectableRunner, selected_icon2);
off!("ColorSel.label", R::ColorSelectableRunner, label);
off!("ColorSel.selected_label", R::ColorSelectableRunner, selected_label);
off!("ColorSel.text", R::ColorSelectableRunner, text);
off!("ColorSel.text_offset", R::ColorSelectableRunner, text_offset);
off!("ColorSel.icon_size", R::ColorSelectableRunner, icon_size);
off!("ColorSel.selected", R::ColorSelectableRunner, selected);
off!("ColorSel.hint", R::ColorSelectableRunner, hint);

// ---- SliderRunner ----
off!("Slider.style", R::SliderRunner, style);
off!("Slider.ratio", R::SliderRunner, ratio);

// ---- TextEditRunner ----
off!("TextEdit.style", R::TextEditRunner, style);
off!("TextEdit.numeric_only", R::TextEditRunner, numeric_only);
off!("TextEdit.max_length", R::TextEditRunner, max_length);
off!("TextEdit.text", R::TextEditRunner, text);
off!("TextEdit.is_editing", R::TextEditRunner, is_editing);

// ---- ScrollViewRunner ----
off!("Scroll.bar_width", R::ScrollViewRunner, bar_width);
off!("Scroll.speed", R::ScrollViewRunner, speed);
off!("Scroll.y_max", R::ScrollViewRunner, y_max);
off!("Scroll.ratio", R::ScrollViewRunner, ratio);

fn main() {}
