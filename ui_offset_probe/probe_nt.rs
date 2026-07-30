use mod_api::*;
use std::mem::{offset_of, size_of, align_of};
// [(); 0] = [(); N] 트릭: 에러가 N(오프셋/크기) 폭로
const _: [(); 0] = [(); offset_of!(NodeTemplate, id)];
const _: [(); 0] = [(); offset_of!(NodeTemplate, name)];
const _: [(); 0] = [(); offset_of!(NodeTemplate, style_path)];
const _: [(); 0] = [(); offset_of!(NodeTemplate, property)];
const _: [(); 0] = [(); offset_of!(NodeTemplate, child)];
const _: [(); 0] = [(); size_of::<NodeTemplate>()];
const _: [(); 0] = [(); align_of::<NodeTemplate>()];
fn main(){}
