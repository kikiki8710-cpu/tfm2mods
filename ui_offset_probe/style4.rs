extern crate engine_ui;
use engine_ui::runner as R;
use std::mem::offset_of;
// color/back_color type
fn ct(r:&R::ColorRunner){ let _:()=r.style.normal.color; }
fn st(r:&R::ColorRunner){ let _:()=r.style.normal.stroke; }
// offsets (errors reveal). 0 => no error.
const _: [();0] = [(); offset_of!(R::ColorRunner, style.normal.color)];        //L4
const _: [();0] = [(); offset_of!(R::ColorRunner, style.normal.back_color)];   //L5
const _: [();0] = [(); offset_of!(R::ColorRunner, style.normal.stroke)];       //L6
const _: [();0] = [(); offset_of!(R::ColorRunner, style.normal.rounding)];     //L7
const _: [();0] = [(); offset_of!(R::ColorRunner, style.hover.color)];         //L8
const _: [();0] = [(); offset_of!(R::LabelRunner, style.normal.color)];        //L9
const _: [();0] = [(); offset_of!(R::LabelRunner, style.normal.size)];         //L10
const _: [();0] = [(); offset_of!(R::LabelRunner, style.normal.font)];         //L11
const _: [();0] = [(); offset_of!(R::ImageRunner, style.normal.source)];       //L12
fn main(){}
