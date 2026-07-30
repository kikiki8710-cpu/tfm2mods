use mod_api::*;
use std::mem::offset_of;
// DropdownRunner 가 mod_api 에서 접근 가능한지 + 필드 목록(__nope__ 로 노출)
const _: [(); 0] = [(); offset_of!(DropdownRunner, __nope__)];
fn main() {}
