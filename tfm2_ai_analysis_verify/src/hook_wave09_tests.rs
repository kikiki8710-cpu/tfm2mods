//! F98의 실제12B prefix와동일한 rel32 JE를 자기소유코드로 대조한다. 게임함수 호출0.
use super::*;
#[test]
fn wave09_rel32_je_preserves_taken_fallthrough_and_fifth_stack_word(){
 const RVA:usize=0xf98e60;
 static PREFIX:&[u8]=&[0x48,0x83,0xec,0x58,0x84,0xc9,0x0f,0x84,0xaa,0,0,0];
 static FIX:&[BranchFixup]=&[BranchFixup{offset:6,len:6,target_rva:0xf98f16}];
 // SUB58 후 entry28은현재RSP+80이다. 분기양쪽에서stack5를읽고상이한작은값을더한다.
 let tail=|add:u8|vec![0x48,0x8b,0x84,0x24,0x80,0,0,0,0x48,0x83,0xc0,add,0x48,0x83,0xc4,0x58,0xc3];
 let mut body=PREFIX.to_vec();body.extend(tail(7));body.resize(0xb6,0x90);body.extend(tail(9));
 let origin=platform::allocate_code(&body).unwrap();let base=origin.checked_sub(RVA).unwrap();
 let spec=HookSpec{rva:RVA,prologue:PREFIX,body:PREFIX,wrapper:0,allow_foreign_chain:false,branches:FIX};
 let mut relocated=relocated_prefix(&spec,base).unwrap();rip_jump(origin+PREFIX.len(),&mut relocated);
 let copied=platform::allocate_code(&relocated).unwrap();
 let original:unsafe extern "system" fn(u64,u64,u64,u64,u64)->u64=unsafe{std::mem::transmute(origin)};
 let forwarded:unsafe extern "system" fn(u64,u64,u64,u64,u64)->u64=unsafe{std::mem::transmute(copied)};
 for lane in [0u64,1,0x100,0x101,u64::MAX]{for fifth in [0u64,0x1122334455667788,u64::MAX-16]{
  let expected=fifth.wrapping_add(if lane&255==0{9}else{7});
  assert_eq!(unsafe{original(lane,0,0,0,fifth)},expected);
  assert_eq!(unsafe{forwarded(lane,0,0,0,fifth)},expected);
 }}
 unsafe{platform::free_unpublished_code(copied);platform::free_unpublished_code(origin);}
}
