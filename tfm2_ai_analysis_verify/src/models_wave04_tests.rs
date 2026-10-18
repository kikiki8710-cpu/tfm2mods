//! 생성된handexpected메모리fixture. 실행은root통합지시후별도다.
use super::*;
use std::collections::BTreeMap;
struct Fixture { fields:BTreeMap<(u64,u8),u64>, reads:Vec<(u64,u8)> }
impl Memory for Fixture {fn read(&mut self,a:u64,w:u8)->Result<u64,&'static str>{self.reads.push((a,w));self.fields.get(&(a,w)).copied().ok_or("W04_FIXTURE_MISSING_READ")}}
#[test]
fn wave04_model_19_handexpected_boundaries(){
{ // empty
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),0u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // first_hit_0
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),12u64),((65680u64,8u8),987u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,65536u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // first_hit_1
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),12u64),((65680u64,8u8),10u64),((65856u64,8u8),987u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,65712u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // first_hit_2
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),12u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),987u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,65888u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // first_hit_3
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),12u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),987u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,66064u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // first_hit_4
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),12u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),987u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,66240u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // first_hit_5
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),12u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),987u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,66416u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // first_hit_6
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),12u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),987u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,66592u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // first_hit_7
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),12u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),987u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,66768u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // first_hit_8
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),12u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),987u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,66944u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // first_hit_9
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),12u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),987u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,67120u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // first_hit_10
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),12u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),987u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,67296u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // first_hit_11
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),12u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),987u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,67472u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // miss_len_1
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),1u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // miss_len_2
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),2u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // miss_len_3
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),3u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // miss_len_4
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),4u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // miss_len_5
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),5u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // miss_len_6
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),6u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // miss_len_7
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),7u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // miss_len_8
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),8u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // miss_len_9
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),9u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // miss_len_10
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),10u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // miss_len_11
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),11u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // miss_len_12
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),12u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // duplicate_first
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),12u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),987u64),((66208u64,8u8),987u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,65888u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // count13_early_hit_allowed
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),13u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),987u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,66416u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // count13_last_miss_panic
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),13u64),((65680u64,8u8),10u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,987u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("E1B430_BOUNDS_PANIC_AFTER_TWELFTH_MISS"));
}
{ // key_0
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),1u64),((65680u64,8u8),0u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,65536u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // key_18446744073709551615
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),1u64),((65680u64,8u8),18446744073709551615u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,18446744073709551615u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,65536u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // full64_key_not_low32
let mut f=Fixture{fields:BTreeMap::from([((67648u64,8u8),1u64),((65680u64,8u8),1u64),((65856u64,8u8),11u64),((66032u64,8u8),12u64),((66208u64,8u8),13u64),((66384u64,8u8),14u64),((66560u64,8u8),15u64),((66736u64,8u8),16u64),((66912u64,8u8),17u64),((67088u64,8u8),18u64),((67264u64,8u8),19u64),((67440u64,8u8),20u64),((67616u64,8u8),21u64)]),reads:Vec::new()};
let result=evaluate(19,[65536u64,4294967297u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // missing_read_never_default0
let mut f=Fixture{fields:BTreeMap::from([]),reads:Vec::new()};
let result=evaluate(19,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("W04_FIXTURE_MISSING_READ"));
}
}
#[test]
fn wave04_model_20_handexpected_boundaries(){
{ // time_0
let mut f=Fixture{fields:BTreeMap::from([((65688u64,8u8),0u64),((65696u64,8u8),18u64),((65536u64,8u8),10u64),((65544u64,8u8),20u64),((65552u64,8u8),30u64),((65560u64,8u8),40u64),((65568u64,8u8),50u64),((65576u64,8u8),60u64),((65584u64,8u8),70u64),((65592u64,8u8),80u64),((65600u64,8u8),90u64),((65608u64,8u8),100u64),((65616u64,8u8),110u64),((65624u64,8u8),120u64),((65632u64,8u8),130u64),((65640u64,8u8),140u64),((65648u64,8u8),150u64),((65656u64,8u8),160u64),((65664u64,8u8),170u64),((65672u64,8u8),180u64)]),reads:Vec::new()};
let result=evaluate(20,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // time_1
let mut f=Fixture{fields:BTreeMap::from([((65688u64,8u8),0u64),((65696u64,8u8),18u64),((65536u64,8u8),10u64),((65544u64,8u8),20u64),((65552u64,8u8),30u64),((65560u64,8u8),40u64),((65568u64,8u8),50u64),((65576u64,8u8),60u64),((65584u64,8u8),70u64),((65592u64,8u8),80u64),((65600u64,8u8),90u64),((65608u64,8u8),100u64),((65616u64,8u8),110u64),((65624u64,8u8),120u64),((65632u64,8u8),130u64),((65640u64,8u8),140u64),((65648u64,8u8),150u64),((65656u64,8u8),160u64),((65664u64,8u8),170u64),((65672u64,8u8),180u64)]),reads:Vec::new()};
let result=evaluate(20,[65536u64,1u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,2u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // time_4
let mut f=Fixture{fields:BTreeMap::from([((65688u64,8u8),0u64),((65696u64,8u8),18u64),((65536u64,8u8),10u64),((65544u64,8u8),20u64),((65552u64,8u8),30u64),((65560u64,8u8),40u64),((65568u64,8u8),50u64),((65576u64,8u8),60u64),((65584u64,8u8),70u64),((65592u64,8u8),80u64),((65600u64,8u8),90u64),((65608u64,8u8),100u64),((65616u64,8u8),110u64),((65624u64,8u8),120u64),((65632u64,8u8),130u64),((65640u64,8u8),140u64),((65648u64,8u8),150u64),((65656u64,8u8),160u64),((65664u64,8u8),170u64),((65672u64,8u8),180u64)]),reads:Vec::new()};
let result=evaluate(20,[65536u64,4u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,8u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // time_5
let mut f=Fixture{fields:BTreeMap::from([((65688u64,8u8),0u64),((65696u64,8u8),18u64),((65536u64,8u8),10u64),((65544u64,8u8),20u64),((65552u64,8u8),30u64),((65560u64,8u8),40u64),((65568u64,8u8),50u64),((65576u64,8u8),60u64),((65584u64,8u8),70u64),((65592u64,8u8),80u64),((65600u64,8u8),90u64),((65608u64,8u8),100u64),((65616u64,8u8),110u64),((65624u64,8u8),120u64),((65632u64,8u8),130u64),((65640u64,8u8),140u64),((65648u64,8u8),150u64),((65656u64,8u8),160u64),((65664u64,8u8),170u64),((65672u64,8u8),180u64)]),reads:Vec::new()};
let result=evaluate(20,[65536u64,5u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,10u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // time_6
let mut f=Fixture{fields:BTreeMap::from([((65688u64,8u8),0u64),((65696u64,8u8),18u64),((65536u64,8u8),10u64),((65544u64,8u8),20u64),((65552u64,8u8),30u64),((65560u64,8u8),40u64),((65568u64,8u8),50u64),((65576u64,8u8),60u64),((65584u64,8u8),70u64),((65592u64,8u8),80u64),((65600u64,8u8),90u64),((65608u64,8u8),100u64),((65616u64,8u8),110u64),((65624u64,8u8),120u64),((65632u64,8u8),130u64),((65640u64,8u8),140u64),((65648u64,8u8),150u64),((65656u64,8u8),160u64),((65664u64,8u8),170u64),((65672u64,8u8),180u64)]),reads:Vec::new()};
let result=evaluate(20,[65536u64,6u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,12u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // time_85
let mut f=Fixture{fields:BTreeMap::from([((65688u64,8u8),0u64),((65696u64,8u8),18u64),((65536u64,8u8),10u64),((65544u64,8u8),20u64),((65552u64,8u8),30u64),((65560u64,8u8),40u64),((65568u64,8u8),50u64),((65576u64,8u8),60u64),((65584u64,8u8),70u64),((65592u64,8u8),80u64),((65600u64,8u8),90u64),((65608u64,8u8),100u64),((65616u64,8u8),110u64),((65624u64,8u8),120u64),((65632u64,8u8),130u64),((65640u64,8u8),140u64),((65648u64,8u8),150u64),((65656u64,8u8),160u64),((65664u64,8u8),170u64),((65672u64,8u8),180u64)]),reads:Vec::new()};
let result=evaluate(20,[65536u64,85u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,170u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // time_89
let mut f=Fixture{fields:BTreeMap::from([((65688u64,8u8),0u64),((65696u64,8u8),18u64),((65536u64,8u8),10u64),((65544u64,8u8),20u64),((65552u64,8u8),30u64),((65560u64,8u8),40u64),((65568u64,8u8),50u64),((65576u64,8u8),60u64),((65584u64,8u8),70u64),((65592u64,8u8),80u64),((65600u64,8u8),90u64),((65608u64,8u8),100u64),((65616u64,8u8),110u64),((65624u64,8u8),120u64),((65632u64,8u8),130u64),((65640u64,8u8),140u64),((65648u64,8u8),150u64),((65656u64,8u8),160u64),((65664u64,8u8),170u64),((65672u64,8u8),180u64)]),reads:Vec::new()};
let result=evaluate(20,[65536u64,89u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,178u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // time_90
let mut f=Fixture{fields:BTreeMap::from([((65688u64,8u8),0u64),((65696u64,8u8),18u64),((65536u64,8u8),10u64),((65544u64,8u8),20u64),((65552u64,8u8),30u64),((65560u64,8u8),40u64),((65568u64,8u8),50u64),((65576u64,8u8),60u64),((65584u64,8u8),70u64),((65592u64,8u8),80u64),((65600u64,8u8),90u64),((65608u64,8u8),100u64),((65616u64,8u8),110u64),((65624u64,8u8),120u64),((65632u64,8u8),130u64),((65640u64,8u8),140u64),((65648u64,8u8),150u64),((65656u64,8u8),160u64),((65664u64,8u8),170u64),((65672u64,8u8),180u64)]),reads:Vec::new()};
let result=evaluate(20,[65536u64,90u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,180u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // time_91
let mut f=Fixture{fields:BTreeMap::from([((65688u64,8u8),0u64),((65696u64,8u8),18u64),((65536u64,8u8),10u64),((65544u64,8u8),20u64),((65552u64,8u8),30u64),((65560u64,8u8),40u64),((65568u64,8u8),50u64),((65576u64,8u8),60u64),((65584u64,8u8),70u64),((65592u64,8u8),80u64),((65600u64,8u8),90u64),((65608u64,8u8),100u64),((65616u64,8u8),110u64),((65624u64,8u8),120u64),((65632u64,8u8),130u64),((65640u64,8u8),140u64),((65648u64,8u8),150u64),((65656u64,8u8),160u64),((65664u64,8u8),170u64),((65672u64,8u8),180u64)]),reads:Vec::new()};
let result=evaluate(20,[65536u64,91u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,180u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // time_95
let mut f=Fixture{fields:BTreeMap::from([((65688u64,8u8),0u64),((65696u64,8u8),18u64),((65536u64,8u8),10u64),((65544u64,8u8),20u64),((65552u64,8u8),30u64),((65560u64,8u8),40u64),((65568u64,8u8),50u64),((65576u64,8u8),60u64),((65584u64,8u8),70u64),((65592u64,8u8),80u64),((65600u64,8u8),90u64),((65608u64,8u8),100u64),((65616u64,8u8),110u64),((65624u64,8u8),120u64),((65632u64,8u8),130u64),((65640u64,8u8),140u64),((65648u64,8u8),150u64),((65656u64,8u8),160u64),((65664u64,8u8),170u64),((65672u64,8u8),180u64)]),reads:Vec::new()};
let result=evaluate(20,[65536u64,95u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,180u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // time_18446744073709551615
let mut f=Fixture{fields:BTreeMap::from([((65688u64,8u8),0u64),((65696u64,8u8),18u64),((65536u64,8u8),10u64),((65544u64,8u8),20u64),((65552u64,8u8),30u64),((65560u64,8u8),40u64),((65568u64,8u8),50u64),((65576u64,8u8),60u64),((65584u64,8u8),70u64),((65592u64,8u8),80u64),((65600u64,8u8),90u64),((65608u64,8u8),100u64),((65616u64,8u8),110u64),((65624u64,8u8),120u64),((65632u64,8u8),130u64),((65640u64,8u8),140u64),((65648u64,8u8),150u64),((65656u64,8u8),160u64),((65664u64,8u8),170u64),((65672u64,8u8),180u64)]),reads:Vec::new()};
let result=evaluate(20,[65536u64,18446744073709551615u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,180u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // count0_tail_baseline
let mut f=Fixture{fields:BTreeMap::from([((65688u64,8u8),123u64),((65696u64,8u8),0u64),((65536u64,8u8),10u64),((65544u64,8u8),20u64),((65552u64,8u8),30u64),((65560u64,8u8),40u64),((65568u64,8u8),50u64),((65576u64,8u8),60u64),((65584u64,8u8),70u64),((65592u64,8u8),80u64),((65600u64,8u8),90u64),((65608u64,8u8),100u64),((65616u64,8u8),110u64),((65624u64,8u8),120u64),((65632u64,8u8),130u64),((65640u64,8u8),140u64),((65648u64,8u8),150u64),((65656u64,8u8),160u64),((65664u64,8u8),170u64),((65672u64,8u8),180u64)]),reads:Vec::new()};
let result=evaluate(20,[65536u64,100u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,123u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // negative_trunc0
let mut f=Fixture{fields:BTreeMap::from([((65688u64,8u8),10u64),((65696u64,8u8),18u64),((65536u64,8u8),18446744073709551605u64),((65544u64,8u8),20u64),((65552u64,8u8),30u64),((65560u64,8u8),40u64),((65568u64,8u8),50u64),((65576u64,8u8),60u64),((65584u64,8u8),70u64),((65592u64,8u8),80u64),((65600u64,8u8),90u64),((65608u64,8u8),100u64),((65616u64,8u8),110u64),((65624u64,8u8),120u64),((65632u64,8u8),130u64),((65640u64,8u8),140u64),((65648u64,8u8),150u64),((65656u64,8u8),160u64),((65664u64,8u8),170u64),((65672u64,8u8),180u64)]),reads:Vec::new()};
let result=evaluate(20,[65536u64,1u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,6u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // wrapped_delta_multiply
let mut f=Fixture{fields:BTreeMap::from([((65688u64,8u8),9223372036854775807u64),((65696u64,8u8),18u64),((65536u64,8u8),9223372036854775808u64),((65544u64,8u8),20u64),((65552u64,8u8),30u64),((65560u64,8u8),40u64),((65568u64,8u8),50u64),((65576u64,8u8),60u64),((65584u64,8u8),70u64),((65592u64,8u8),80u64),((65600u64,8u8),90u64),((65608u64,8u8),100u64),((65616u64,8u8),110u64),((65624u64,8u8),120u64),((65632u64,8u8),130u64),((65640u64,8u8),140u64),((65648u64,8u8),150u64),((65656u64,8u8),160u64),((65664u64,8u8),170u64),((65672u64,8u8),180u64)]),reads:Vec::new()};
let result=evaluate(20,[65536u64,4u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,9223372036854775807u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // count19_capture_reject_not_game_upfront
let mut f=Fixture{fields:BTreeMap::from([((65688u64,8u8),0u64),((65696u64,8u8),19u64),((65536u64,8u8),10u64),((65544u64,8u8),20u64),((65552u64,8u8),30u64),((65560u64,8u8),40u64),((65568u64,8u8),50u64),((65576u64,8u8),60u64),((65584u64,8u8),70u64),((65592u64,8u8),80u64),((65600u64,8u8),90u64),((65608u64,8u8),100u64),((65616u64,8u8),110u64),((65624u64,8u8),120u64),((65632u64,8u8),130u64),((65640u64,8u8),140u64),((65648u64,8u8),150u64),((65656u64,8u8),160u64),((65664u64,8u8),170u64),((65672u64,8u8),180u64)]),reads:Vec::new()};
let result=evaluate(20,[65536u64,1u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("E1EDE0_CAPTURE_COUNT_ABOVE18"));
}
{ // time0_no_count_read
let mut f=Fixture{fields:BTreeMap::from([((65688u64,8u8),456u64)]),reads:Vec::new()};
let result=evaluate(20,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,456u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
assert!(!f.reads.contains(&(65696u64,8u8)));
}
{ // missing_read_never_default0
let mut f=Fixture{fields:BTreeMap::from([]),reads:Vec::new()};
let result=evaluate(20,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("W04_FIXTURE_MISSING_READ"));
}
}
#[test]
fn wave04_model_21_handexpected_boundaries(){
{ // exclude_0
let mut f=Fixture{fields:BTreeMap::from([((65560u64,8u8),0u64),((65568u64,8u8),5u64),((65552u64,8u8),69632u64),((65536u64,8u8),73728u64),((65544u64,8u8),77824u64),((69632u64,8u8),0u64),((73728u64,8u8),0u64),((78392u64,8u8),256u64),((78400u64,8u8),0u64),((78408u64,8u8),256u64),((78416u64,8u8),0u64),((78424u64,8u8),256u64),((78432u64,8u8),256u64),((78440u64,8u8),0u64),((78448u64,8u8),256u64),((78456u64,8u8),0u64),((78464u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(21,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,2u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // exclude_1
let mut f=Fixture{fields:BTreeMap::from([((65560u64,8u8),0u64),((65568u64,8u8),5u64),((65552u64,8u8),69632u64),((65536u64,8u8),73728u64),((65544u64,8u8),77824u64),((69632u64,8u8),0u64),((73728u64,8u8),1u64),((78392u64,8u8),256u64),((78400u64,8u8),0u64),((78408u64,8u8),256u64),((78416u64,8u8),0u64),((78424u64,8u8),256u64),((78432u64,8u8),256u64),((78440u64,8u8),0u64),((78448u64,8u8),256u64),((78456u64,8u8),0u64),((78464u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(21,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,3u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // exclude_2
let mut f=Fixture{fields:BTreeMap::from([((65560u64,8u8),0u64),((65568u64,8u8),5u64),((65552u64,8u8),69632u64),((65536u64,8u8),73728u64),((65544u64,8u8),77824u64),((69632u64,8u8),0u64),((73728u64,8u8),2u64),((78392u64,8u8),256u64),((78400u64,8u8),0u64),((78408u64,8u8),256u64),((78416u64,8u8),0u64),((78424u64,8u8),256u64),((78432u64,8u8),256u64),((78440u64,8u8),0u64),((78448u64,8u8),256u64),((78456u64,8u8),0u64),((78464u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(21,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,2u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // exclude_4
let mut f=Fixture{fields:BTreeMap::from([((65560u64,8u8),0u64),((65568u64,8u8),5u64),((65552u64,8u8),69632u64),((65536u64,8u8),73728u64),((65544u64,8u8),77824u64),((69632u64,8u8),0u64),((73728u64,8u8),4u64),((78392u64,8u8),256u64),((78400u64,8u8),0u64),((78408u64,8u8),256u64),((78416u64,8u8),0u64),((78424u64,8u8),256u64),((78432u64,8u8),256u64),((78440u64,8u8),0u64),((78448u64,8u8),256u64),((78456u64,8u8),0u64),((78464u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(21,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,2u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // exclude_9
let mut f=Fixture{fields:BTreeMap::from([((65560u64,8u8),0u64),((65568u64,8u8),5u64),((65552u64,8u8),69632u64),((65536u64,8u8),73728u64),((65544u64,8u8),77824u64),((69632u64,8u8),0u64),((73728u64,8u8),9u64),((78392u64,8u8),256u64),((78400u64,8u8),0u64),((78408u64,8u8),256u64),((78416u64,8u8),0u64),((78424u64,8u8),256u64),((78432u64,8u8),256u64),((78440u64,8u8),0u64),((78448u64,8u8),256u64),((78456u64,8u8),0u64),((78464u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(21,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,3u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // excluded_skips_bad_team
let mut f=Fixture{fields:BTreeMap::from([((65560u64,8u8),1u64),((65568u64,8u8),2u64),((65552u64,8u8),69632u64),((65536u64,8u8),73728u64),((65544u64,8u8),77824u64),((69632u64,8u8),2u64),((73728u64,8u8),1u64),((78392u64,8u8),256u64),((78400u64,8u8),0u64),((78408u64,8u8),256u64),((78416u64,8u8),0u64),((78424u64,8u8),256u64),((78432u64,8u8),256u64),((78440u64,8u8),0u64),((78448u64,8u8),256u64),((78456u64,8u8),0u64),((78464u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(21,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
assert!(!f.reads.contains(&(69632u64,8u8)));
}
{ // excluded_id5_skips_bad_team
let mut f=Fixture{fields:BTreeMap::from([((65560u64,8u8),5u64),((65568u64,8u8),6u64),((65552u64,8u8),69632u64),((65536u64,8u8),73728u64),((65544u64,8u8),77824u64),((69632u64,8u8),2u64),((73728u64,8u8),5u64),((78392u64,8u8),256u64),((78400u64,8u8),0u64),((78408u64,8u8),256u64),((78416u64,8u8),0u64),((78424u64,8u8),256u64),((78432u64,8u8),256u64),((78440u64,8u8),0u64),((78448u64,8u8),256u64),((78456u64,8u8),0u64),((78464u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(21,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
assert!(!f.reads.contains(&(69632u64,8u8)));
}
{ // empty_0
let mut f=Fixture{fields:BTreeMap::from([((65560u64,8u8),0u64),((65568u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(21,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // empty_10
let mut f=Fixture{fields:BTreeMap::from([((65560u64,8u8),10u64),((65568u64,8u8),1u64)]),reads:Vec::new()};
let result=evaluate(21,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // bad_team_before_bad_id
let mut f=Fixture{fields:BTreeMap::from([((65560u64,8u8),5u64),((65568u64,8u8),6u64),((65552u64,8u8),69632u64),((65536u64,8u8),73728u64),((65544u64,8u8),77824u64),((69632u64,8u8),2u64),((73728u64,8u8),0u64),((78392u64,8u8),256u64),((78400u64,8u8),0u64),((78408u64,8u8),256u64),((78416u64,8u8),0u64),((78424u64,8u8),256u64),((78432u64,8u8),256u64),((78440u64,8u8),0u64),((78448u64,8u8),256u64),((78456u64,8u8),0u64),((78464u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(21,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("D81CE0_TEAM_BOUNDS_PANIC"));
}
{ // bad_id
let mut f=Fixture{fields:BTreeMap::from([((65560u64,8u8),5u64),((65568u64,8u8),6u64),((65552u64,8u8),69632u64),((65536u64,8u8),73728u64),((65544u64,8u8),77824u64),((69632u64,8u8),0u64),((73728u64,8u8),0u64),((78392u64,8u8),256u64),((78400u64,8u8),0u64),((78408u64,8u8),256u64),((78416u64,8u8),0u64),((78424u64,8u8),256u64),((78432u64,8u8),256u64),((78440u64,8u8),0u64),((78448u64,8u8),256u64),((78456u64,8u8),0u64),((78464u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(21,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("D81CE0_ROLE_BOUNDS_PANIC"));
}
{ // capture32_limit
let mut f=Fixture{fields:BTreeMap::from([((65560u64,8u8),0u64),((65568u64,8u8),33u64)]),reads:Vec::new()};
let result=evaluate(21,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("D81CE0_CAPTURE_RANGE_ABOVE32"));
}
{ // missing_read_never_default0
let mut f=Fixture{fields:BTreeMap::from([]),reads:Vec::new()};
let result=evaluate(21,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("W04_FIXTURE_MISSING_READ"));
}
}
#[test]
fn wave04_model_22_handexpected_boundaries(){
{ // team0_range0_0
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),0u64),((65560u64,8u8),0u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range0_1
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),0u64),((65560u64,8u8),1u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range0_2
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),0u64),((65560u64,8u8),2u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range0_3
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),0u64),((65560u64,8u8),3u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,2u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range0_4
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),0u64),((65560u64,8u8),4u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,2u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range0_5
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),0u64),((65560u64,8u8),5u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,3u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range1_1
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),1u64),((65560u64,8u8),1u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range1_2
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),1u64),((65560u64,8u8),2u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range1_3
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),1u64),((65560u64,8u8),3u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range1_4
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),1u64),((65560u64,8u8),4u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range1_5
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),1u64),((65560u64,8u8),5u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,2u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range2_2
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),2u64),((65560u64,8u8),2u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range2_3
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),2u64),((65560u64,8u8),3u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range2_4
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),2u64),((65560u64,8u8),4u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range2_5
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),2u64),((65560u64,8u8),5u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,2u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range3_3
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),3u64),((65560u64,8u8),3u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range3_4
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),3u64),((65560u64,8u8),4u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range3_5
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),3u64),((65560u64,8u8),5u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range4_4
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),4u64),((65560u64,8u8),4u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range4_5
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),4u64),((65560u64,8u8),5u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team0_range5_5
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),5u64),((65560u64,8u8),5u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range0_0
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),0u64),((65560u64,8u8),0u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range0_1
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),0u64),((65560u64,8u8),1u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range0_2
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),0u64),((65560u64,8u8),2u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range0_3
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),0u64),((65560u64,8u8),3u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,2u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range0_4
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),0u64),((65560u64,8u8),4u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,2u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range0_5
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),0u64),((65560u64,8u8),5u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,3u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range1_1
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),1u64),((65560u64,8u8),1u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range1_2
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),1u64),((65560u64,8u8),2u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range1_3
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),1u64),((65560u64,8u8),3u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range1_4
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),1u64),((65560u64,8u8),4u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range1_5
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),1u64),((65560u64,8u8),5u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,2u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range2_2
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),2u64),((65560u64,8u8),2u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range2_3
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),2u64),((65560u64,8u8),3u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range2_4
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),2u64),((65560u64,8u8),4u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range2_5
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),2u64),((65560u64,8u8),5u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,2u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range3_3
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),3u64),((65560u64,8u8),3u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range3_4
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),3u64),((65560u64,8u8),4u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range3_5
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),3u64),((65560u64,8u8),5u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range4_4
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),4u64),((65560u64,8u8),4u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range4_5
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),4u64),((65560u64,8u8),5u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team1_range5_5
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),5u64),((65560u64,8u8),5u64),((65544u64,8u8),69632u64),((69632u64,8u8),1u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // reverse_empty_no_team
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),9u64),((65560u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team_bad
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),0u64),((65560u64,8u8),1u64),((65544u64,8u8),69632u64),((69632u64,8u8),2u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("D81BD0_TEAM_BOUNDS_PANIC"));
}
{ // start5_bad
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),5u64),((65560u64,8u8),6u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("D81BD0_START_BOUNDS_PANIC"));
}
{ // end6_capture_domain
let mut f=Fixture{fields:BTreeMap::from([((65552u64,8u8),0u64),((65560u64,8u8),6u64),((65544u64,8u8),69632u64),((69632u64,8u8),0u64),((65536u64,8u8),73728u64),((74296u64,8u8),256u64),((74304u64,8u8),0u64),((74312u64,8u8),256u64),((74320u64,8u8),0u64),((74328u64,8u8),256u64),((74336u64,8u8),256u64),((74344u64,8u8),0u64),((74352u64,8u8),256u64),((74360u64,8u8),0u64),((74368u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("D81BD0_CAPTURE_RANGE_END_ABOVE5"));
}
{ // missing_read_never_default0
let mut f=Fixture{fields:BTreeMap::from([]),reads:Vec::new()};
let result=evaluate(22,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("W04_FIXTURE_MISSING_READ"));
}
}
#[test]
fn wave04_model_23_handexpected_boundaries(){
{ // line0_mode0
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),0u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line0_mode1
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),1u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line0_mode2
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),2u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line0_mode3
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),3u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line0_mode4
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),4u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line0_mode5
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),5u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line0_mode6
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),6u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line0_mode7
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),7u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line0_mode8
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),8u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line0_mode9
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),9u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line1_mode0
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),0u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77880u64,8u8),18446744073709548615u64),((77896u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,1u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line1_mode1
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),1u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77880u64,8u8),18446744073709548615u64),((77896u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,1u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line1_mode2
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),2u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77880u64,8u8),18446744073709548615u64),((77896u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,1u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line1_mode3
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),3u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77880u64,8u8),18446744073709548615u64),((77896u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,1u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line1_mode4
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),4u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77880u64,8u8),18446744073709548615u64),((77896u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,1u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line1_mode5
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),5u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77880u64,8u8),18446744073709548615u64),((77896u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,1u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line1_mode6
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),6u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77880u64,8u8),18446744073709548615u64),((77896u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,1u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line1_mode7
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),7u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77880u64,8u8),18446744073709548615u64),((77896u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,1u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line1_mode8
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),8u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77880u64,8u8),18446744073709548615u64),((77896u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,1u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line1_mode9
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),9u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77880u64,8u8),18446744073709548615u64),((77896u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,1u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line2_mode0
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),0u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,2u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line2_mode1
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),1u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,2u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line2_mode2
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),2u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,2u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line2_mode3
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),3u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,2u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line2_mode4
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),4u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,2u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line2_mode5
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),5u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,2u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line2_mode6
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),6u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,2u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line2_mode7
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),7u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,2u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line2_mode8
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),8u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,2u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line2_mode9
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),9u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,2u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line256_mode0
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),0u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,256u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line256_mode1
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),1u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,256u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line256_mode2
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),2u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,256u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line256_mode3
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),3u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,256u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line256_mode4
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),4u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,256u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line256_mode5
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),5u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,256u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line256_mode6
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),6u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,256u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line256_mode7
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),7u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,256u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line256_mode8
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),8u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,256u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line256_mode9
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),9u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,256u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line255_mode0
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),0u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,255u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line255_mode1
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),1u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,255u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line255_mode2
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),2u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,255u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line255_mode3
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),3u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,255u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line255_mode4
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),4u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,255u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line255_mode5
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),5u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,255u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line255_mode6
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),6u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,255u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line255_mode7
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),7u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,255u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line255_mode8
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),8u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,255u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // line255_mode9
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),9u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77920u64,8u8),18446744073709548615u64),((77936u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,255u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // progress_-3002
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),0u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548614u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // progress_-3001
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),0u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // progress_-3000
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),0u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548616u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // progress_-9223372036854775808
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),0u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),9223372036854775808u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // progress_9223372036854775807
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),0u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),9223372036854775807u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // count32_-4
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),0u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // count32_-3
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),0u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967293u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // count32_-2
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),0u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),4294967294u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // count32_-2147483648
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),0u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),2147483648u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // count32_2147483647
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),0u64),((67584u64,8u8),0u64),((69648u64,8u8),77824u64),((77840u64,8u8),18446744073709548615u64),((77856u64,4u8),2147483647u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team_full64_bad
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),0u64),((67584u64,8u8),4294967296u64),((69648u64,8u8),77824u64),((11854109814800u64,8u8),18446744073709548615u64),((11854109814816u64,4u8),4294967292u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("E54190_TEAM_BOUNDS_PANIC"));
}
{ // mode_reject_skips_player
let mut f=Fixture{fields:BTreeMap::from([((69640u64,8u8),73728u64),((73784u64,1u8),9u64)]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
assert!(!f.reads.contains(&(67584u64,8u8)));
}
{ // missing_read_never_default0
let mut f=Fixture{fields:BTreeMap::from([]),reads:Vec::new()};
let result=evaluate(23,[65536u64,69632u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("W04_FIXTURE_MISSING_READ"));
}
}
#[test]
fn wave04_model_24_handexpected_boundaries(){
{ // t0i0f0s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),0u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90680u64,8u8),1u64),((90584u64,8u8),0u64),((90592u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i0f0s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),0u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90680u64,8u8),1u64),((90584u64,8u8),0u64),((90592u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i0f256s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),0u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90680u64,8u8),1u64),((90584u64,8u8),256u64),((90592u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i0f256s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),0u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90680u64,8u8),1u64),((90584u64,8u8),256u64),((90592u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i1f0s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),1u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90688u64,8u8),1u64),((90584u64,8u8),0u64),((90592u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i1f0s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),1u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90688u64,8u8),1u64),((90584u64,8u8),0u64),((90592u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i1f256s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),1u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90688u64,8u8),1u64),((90584u64,8u8),256u64),((90592u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i1f256s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),1u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90688u64,8u8),1u64),((90584u64,8u8),256u64),((90592u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i2f0s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),2u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90696u64,8u8),1u64),((90616u64,8u8),0u64),((90624u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i2f0s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),2u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90696u64,8u8),1u64),((90616u64,8u8),0u64),((90624u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i2f256s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),2u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90696u64,8u8),1u64),((90616u64,8u8),256u64),((90624u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i2f256s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),2u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90696u64,8u8),1u64),((90616u64,8u8),256u64),((90624u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i3f0s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),3u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90704u64,8u8),1u64),((90648u64,8u8),0u64),((90656u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i3f0s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),3u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90704u64,8u8),1u64),((90648u64,8u8),0u64),((90656u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i3f256s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),3u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90704u64,8u8),1u64),((90648u64,8u8),256u64),((90656u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i3f256s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),3u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90704u64,8u8),1u64),((90648u64,8u8),256u64),((90656u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i4f0s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),4u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90712u64,8u8),1u64),((90584u64,8u8),0u64),((90592u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i4f0s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),4u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90712u64,8u8),1u64),((90584u64,8u8),0u64),((90592u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i4f256s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),4u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90712u64,8u8),1u64),((90584u64,8u8),256u64),((90592u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t0i4f256s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),4u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90712u64,8u8),1u64),((90584u64,8u8),256u64),((90592u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i0f0s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),0u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90720u64,8u8),1u64),((90584u64,8u8),0u64),((90592u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i0f0s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),0u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90720u64,8u8),1u64),((90584u64,8u8),0u64),((90592u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i0f256s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),0u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90720u64,8u8),1u64),((90584u64,8u8),256u64),((90592u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i0f256s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),0u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90720u64,8u8),1u64),((90584u64,8u8),256u64),((90592u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i1f0s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),1u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90728u64,8u8),1u64),((90584u64,8u8),0u64),((90592u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i1f0s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),1u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90728u64,8u8),1u64),((90584u64,8u8),0u64),((90592u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i1f256s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),1u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90728u64,8u8),1u64),((90584u64,8u8),256u64),((90592u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i1f256s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),1u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90728u64,8u8),1u64),((90584u64,8u8),256u64),((90592u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i2f0s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),2u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90736u64,8u8),1u64),((90616u64,8u8),0u64),((90624u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i2f0s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),2u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90736u64,8u8),1u64),((90616u64,8u8),0u64),((90624u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i2f256s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),2u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90736u64,8u8),1u64),((90616u64,8u8),256u64),((90624u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i2f256s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),2u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90736u64,8u8),1u64),((90616u64,8u8),256u64),((90624u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i3f0s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),3u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90744u64,8u8),1u64),((90648u64,8u8),0u64),((90656u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i3f0s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),3u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90744u64,8u8),1u64),((90648u64,8u8),0u64),((90656u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i3f256s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),3u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90744u64,8u8),1u64),((90648u64,8u8),256u64),((90656u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i3f256s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),3u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90744u64,8u8),1u64),((90648u64,8u8),256u64),((90656u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i4f0s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),4u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90752u64,8u8),1u64),((90584u64,8u8),0u64),((90592u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i4f0s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),4u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90752u64,8u8),1u64),((90584u64,8u8),0u64),((90592u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i4f256s0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),4u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90752u64,8u8),1u64),((90584u64,8u8),256u64),((90592u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // t1i4f256s256
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),1u64),((81920u64,8u8),4u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90752u64,8u8),1u64),((90584u64,8u8),256u64),((90592u64,8u8),256u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // mode9_false
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),0u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),9u64),((90680u64,8u8),1u64),((90584u64,8u8),0u64),((90592u64,8u8),1u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // null_entity_false
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),0u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90680u64,8u8),0u64),((90584u64,8u8),0u64),((90592u64,8u8),1u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // team_bad
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),2u64),((81920u64,8u8),0u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90760u64,8u8),1u64),((90584u64,8u8),0u64),((90592u64,8u8),1u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("FFA970_TEAM_BOUNDS_PANIC"));
}
{ // id_bad
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),5u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90720u64,8u8),1u64),((90584u64,8u8),0u64),((90592u64,8u8),1u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("FFA970_ROLE_BOUNDS_PANIC"));
}
{ // first_zero_does_not_read_second
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73736u64,8u8),77824u64),((77824u64,8u8),0u64),((81920u64,8u8),0u64),((73728u64,8u8),86016u64),((86016u64,8u8),90112u64),((86024u64,8u8),94208u64),((94264u64,1u8),0u64),((90680u64,8u8),1u64),((90584u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(24,[65536u64,81920u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
assert!(!f.reads.contains(&(90592u64,8u8)));
}
{ // missing_read_never_default0
let mut f=Fixture{fields:BTreeMap::from([]),reads:Vec::new()};
let result=evaluate(24,[65536u64,69632u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("W04_FIXTURE_MISSING_READ"));
}
}
#[test]
fn wave04_model_25_handexpected_boundaries(){
{ // hp30_29
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73728u64,1u8),0u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),0u64),((77824u64,8u8),86016u64),((86488u64,8u8),0u64),((86496u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),0u64),((86584u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),1u64),((65544u64,8u8),102400u64),((103008u64,8u8),131072u64),((86624u64,8u8),0u64),((102968u64,8u8),0u64),((131176u64,4u8),0u64),((132640u64,8u8),100u64),((132712u64,8u8),29u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // hp30_30
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73728u64,1u8),0u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),0u64),((77824u64,8u8),86016u64),((86488u64,8u8),0u64),((86496u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),0u64),((86584u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),1u64),((65544u64,8u8),102400u64),((103008u64,8u8),131072u64),((86624u64,8u8),0u64),((102968u64,8u8),0u64),((131176u64,4u8),0u64),((132640u64,8u8),100u64),((132712u64,8u8),30u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // hp30_31
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73728u64,1u8),0u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),0u64),((77824u64,8u8),86016u64),((86488u64,8u8),0u64),((86496u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),0u64),((86584u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),1u64),((65544u64,8u8),102400u64),((103008u64,8u8),131072u64),((86624u64,8u8),0u64),((102968u64,8u8),0u64),((131176u64,4u8),0u64),((132640u64,8u8),100u64),((132712u64,8u8),31u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // candidate_0
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73728u64,1u8),0u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),0u64),((77824u64,8u8),86016u64),((86488u64,8u8),0u64),((86496u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),0u64),((86584u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),1u64),((65544u64,8u8),102400u64),((103008u64,8u8),131072u64),((86624u64,8u8),0u64),((102968u64,8u8),0u64),((131176u64,4u8),0u64),((132640u64,8u8),100u64),((132712u64,8u8),30u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // candidate_1
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73729u64,1u8),0u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),0u64),((77824u64,8u8),86016u64),((86488u64,8u8),0u64),((86496u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),0u64),((86592u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),1u64),((65544u64,8u8),102400u64),((103016u64,8u8),131072u64),((86632u64,8u8),0u64),((102976u64,8u8),0u64),((131176u64,4u8),0u64),((132640u64,8u8),100u64),((132712u64,8u8),30u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,1u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // candidate_2
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73730u64,1u8),0u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),0u64),((77824u64,8u8),86016u64),((86520u64,8u8),0u64),((86528u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),0u64),((86600u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),1u64),((65544u64,8u8),102400u64),((103024u64,8u8),131072u64),((86640u64,8u8),0u64),((102984u64,8u8),0u64),((131176u64,4u8),0u64),((132640u64,8u8),100u64),((132712u64,8u8),30u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,2u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // candidate_3
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73731u64,1u8),0u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),0u64),((77824u64,8u8),86016u64),((86552u64,8u8),0u64),((86560u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),0u64),((86608u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),1u64),((65544u64,8u8),102400u64),((103032u64,8u8),131072u64),((86648u64,8u8),0u64),((102992u64,8u8),0u64),((131176u64,4u8),0u64),((132640u64,8u8),100u64),((132712u64,8u8),30u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,3u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // candidate_4
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73732u64,1u8),0u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),0u64),((77824u64,8u8),86016u64),((86488u64,8u8),0u64),((86496u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),0u64),((86616u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),1u64),((65544u64,8u8),102400u64),((103040u64,8u8),131072u64),((86656u64,8u8),0u64),((103000u64,8u8),0u64),((131176u64,4u8),0u64),((132640u64,8u8),100u64),((132712u64,8u8),30u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,4u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // max0_clamp1
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73728u64,1u8),0u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),0u64),((77824u64,8u8),86016u64),((86488u64,8u8),0u64),((86496u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),0u64),((86584u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),1u64),((65544u64,8u8),102400u64),((103008u64,8u8),131072u64),((86624u64,8u8),0u64),((102968u64,8u8),0u64),((131176u64,4u8),0u64),((132640u64,8u8),0u64),((132712u64,8u8),1u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // hp100wrap_not_wide
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73728u64,1u8),0u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),0u64),((77824u64,8u8),86016u64),((86488u64,8u8),0u64),((86496u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),0u64),((86584u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),1u64),((65544u64,8u8),102400u64),((103008u64,8u8),131072u64),((86624u64,8u8),0u64),((102968u64,8u8),0u64),((131176u64,4u8),0u64),((132640u64,8u8),18446744073709551615u64),((132712u64,8u8),18446744073709551615u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // subtype_low32_1
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73728u64,1u8),0u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),0u64),((77824u64,8u8),86016u64),((86488u64,8u8),0u64),((86496u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),0u64),((86584u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),1u64),((65544u64,8u8),102400u64),((103008u64,8u8),131072u64),((86624u64,8u8),0u64),((102968u64,8u8),0u64),((131176u64,4u8),15u64),((132640u64,8u8),100u64),((132712u64,8u8),30u64),((131184u64,8u8),1u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // subtype_low32_4294967297
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73728u64,1u8),0u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),0u64),((77824u64,8u8),86016u64),((86488u64,8u8),0u64),((86496u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),0u64),((86584u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),1u64),((65544u64,8u8),102400u64),((103008u64,8u8),131072u64),((86624u64,8u8),0u64),((102968u64,8u8),0u64),((131176u64,4u8),15u64),((132640u64,8u8),100u64),((132712u64,8u8),30u64),((131184u64,8u8),4294967297u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // subtype_low32_4294967296
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73728u64,1u8),0u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),0u64),((77824u64,8u8),86016u64),((86488u64,8u8),0u64),((86496u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),0u64),((86584u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),1u64),((65544u64,8u8),102400u64),((103008u64,8u8),131072u64),((86624u64,8u8),0u64),((102968u64,8u8),0u64),((131176u64,4u8),15u64),((132640u64,8u8),100u64),((132712u64,8u8),30u64),((131184u64,8u8),4294967296u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // subtype8_load_required
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73728u64,1u8),0u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),0u64),((77824u64,8u8),86016u64),((86488u64,8u8),0u64),((86496u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),0u64),((86584u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),1u64),((65544u64,8u8),102400u64),((103008u64,8u8),131072u64),((86624u64,8u8),0u64),((102968u64,8u8),0u64),((131176u64,4u8),15u64),((132640u64,8u8),100u64),((132712u64,8u8),30u64),((131184u64,4u8),1u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("W04_FIXTURE_MISSING_READ"));
}
{ // wrapper_team_bad
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73728u64,1u8),0u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),0u64),((77824u64,8u8),86016u64),((86488u64,8u8),0u64),((86496u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),2u64),((86584u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),1u64),((65544u64,8u8),102400u64),((103008u64,8u8),131072u64),((86624u64,8u8),0u64),((102968u64,8u8),0u64),((131176u64,4u8),0u64),((132640u64,8u8),100u64),((132712u64,8u8),30u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("FD4260_WRAPPER_TEAM_BOUNDS_PANIC"));
}
{ // capture_team_bad
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73728u64,1u8),0u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),0u64),((77824u64,8u8),86016u64),((86488u64,8u8),0u64),((86496u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),0u64),((86584u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),2u64),((65544u64,8u8),102400u64),((103008u64,8u8),131072u64),((86624u64,8u8),0u64),((102968u64,8u8),0u64),((131176u64,4u8),0u64),((132640u64,8u8),100u64),((132712u64,8u8),30u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("FD4260_CAPTURE_TEAM_BOUNDS_PANIC"));
}
{ // excluded_shortcut
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73728u64,1u8),1u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),0u64),((77824u64,8u8),86016u64),((86488u64,8u8),0u64),((86496u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),0u64),((86584u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),1u64),((65544u64,8u8),102400u64),((103008u64,8u8),131072u64),((86624u64,8u8),0u64),((102968u64,8u8),0u64),((131176u64,4u8),0u64),((132640u64,8u8),100u64),((132712u64,8u8),30u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
assert!(!f.reads.contains(&(69640u64,8u8)));
}
{ // bad_id_no_reads
let mut f=Fixture{fields:BTreeMap::from([]),reads:Vec::new()};
let result=evaluate(25,[65536u64,5u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("FD4260_ID_BOUNDS_PANIC"));
}
{ // mode9_false
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((69632u64,8u8),73728u64),((73728u64,1u8),0u64),((69640u64,8u8),77824u64),((77832u64,8u8),81920u64),((81976u64,1u8),9u64),((77824u64,8u8),86016u64),((86488u64,8u8),0u64),((86496u64,8u8),1u64),((69648u64,8u8),90112u64),((90112u64,8u8),0u64),((86584u64,8u8),94208u64),((65552u64,8u8),98304u64),((98304u64,8u8),1u64),((65544u64,8u8),102400u64),((103008u64,8u8),131072u64),((86624u64,8u8),0u64),((102968u64,8u8),0u64),((131176u64,4u8),0u64),((132640u64,8u8),100u64),((132712u64,8u8),30u64)]),reads:Vec::new()};
let result=evaluate(25,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,255u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // missing_read_never_default0
let mut f=Fixture{fields:BTreeMap::from([]),reads:Vec::new()};
let result=evaluate(25,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("W04_FIXTURE_MISSING_READ"));
}
}
#[test]
fn wave04_model_26_handexpected_boundaries(){
{ // hp50_49
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),0u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),49u64),((132696u64,8u8),192000u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // hp50_50
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),0u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),192000u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // hp50_51
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),0u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),51u64),((132696u64,8u8),192000u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // tag4_edge_191999
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),0u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),191999u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // tag4_edge_192000
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),0u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),192000u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // tag4_edge_192001
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),0u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),192001u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // tag4_gap63999
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),1000000u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),1063999u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // tag4_gap64000
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),1000000u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),1064000u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // tag4_gap64001
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),1000000u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),1064001u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // tag5_gap63999
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),5u64),((77832u64,8u8),81920u64),((86720u64,8u8),1063999u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),1000000u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // tag5_gap64000
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),5u64),((77832u64,8u8),81920u64),((86720u64,8u8),1064000u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),1000000u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // tag5_gap64001
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),5u64),((77832u64,8u8),81920u64),((86720u64,8u8),1064001u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),1000000u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // tag5_edge_191999
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),5u64),((77832u64,8u8),81920u64),((86720u64,8u8),191999u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),0u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // tag5_edge_192000
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),5u64),((77832u64,8u8),81920u64),((86720u64,8u8),192000u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),0u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // tag5_edge_192001
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),5u64),((77832u64,8u8),81920u64),((86720u64,8u8),192001u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),0u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // tag4_cut_807999
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),807999u64),((86712u64,8u8),1000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),1000000u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // tag4_cut_808000
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),808000u64),((86712u64,8u8),1000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),1000000u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // tag4_cut_808001
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),808001u64),((86712u64,8u8),1000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),1000000u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // eventother_no_hp_xy
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),3u64),((77832u64,8u8),81920u64),((86720u64,8u8),0u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // eventother_max0_still_panic
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),3u64),((77832u64,8u8),81920u64),((86720u64,8u8),0u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),0u64),((132712u64,8u8),50u64),((132696u64,8u8),192000u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("DC0D80_DIVISOR_ZERO_GAME_PANIC_DOMAIN"));
}
{ // max0_not_clamped
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),0u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),0u64),((132712u64,8u8),50u64),((132696u64,8u8),192000u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("DC0D80_DIVISOR_ZERO_GAME_PANIC_DOMAIN"));
}
{ // empty_ignores_event_context
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69632u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),0u64),((86712u64,8u8),2000000u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,0u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
assert!(!f.reads.contains(&(65552u64,8u8)));
assert!(!f.reads.contains(&(65560u64,8u8)));
}
{ // reverse_slice
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69631u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),0u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),192000u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("DC0D80_INVALID_CAPTURE_SLICE"));
}
{ // unaligned_slice
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69633u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),0u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),192000u64),((132704u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("DC0D80_INVALID_CAPTURE_SLICE"));
}
{ // length17_capture_reject
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69768u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),0u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),192000u64),((132704u64,8u8),0u64),((69640u64,8u8),135168u64),((136736u64,8u8),100u64),((136808u64,8u8),50u64),((136792u64,8u8),192000u64),((136800u64,8u8),0u64),((69648u64,8u8),139264u64),((140832u64,8u8),100u64),((140904u64,8u8),50u64),((140888u64,8u8),192000u64),((140896u64,8u8),0u64),((69656u64,8u8),143360u64),((144928u64,8u8),100u64),((145000u64,8u8),50u64),((144984u64,8u8),192000u64),((144992u64,8u8),0u64),((69664u64,8u8),147456u64),((149024u64,8u8),100u64),((149096u64,8u8),50u64),((149080u64,8u8),192000u64),((149088u64,8u8),0u64),((69672u64,8u8),151552u64),((153120u64,8u8),100u64),((153192u64,8u8),50u64),((153176u64,8u8),192000u64),((153184u64,8u8),0u64),((69680u64,8u8),155648u64),((157216u64,8u8),100u64),((157288u64,8u8),50u64),((157272u64,8u8),192000u64),((157280u64,8u8),0u64),((69688u64,8u8),159744u64),((161312u64,8u8),100u64),((161384u64,8u8),50u64),((161368u64,8u8),192000u64),((161376u64,8u8),0u64),((69696u64,8u8),163840u64),((165408u64,8u8),100u64),((165480u64,8u8),50u64),((165464u64,8u8),192000u64),((165472u64,8u8),0u64),((69704u64,8u8),167936u64),((169504u64,8u8),100u64),((169576u64,8u8),50u64),((169560u64,8u8),192000u64),((169568u64,8u8),0u64),((69712u64,8u8),172032u64),((173600u64,8u8),100u64),((173672u64,8u8),50u64),((173656u64,8u8),192000u64),((173664u64,8u8),0u64),((69720u64,8u8),176128u64),((177696u64,8u8),100u64),((177768u64,8u8),50u64),((177752u64,8u8),192000u64),((177760u64,8u8),0u64),((69728u64,8u8),180224u64),((181792u64,8u8),100u64),((181864u64,8u8),50u64),((181848u64,8u8),192000u64),((181856u64,8u8),0u64),((69736u64,8u8),184320u64),((185888u64,8u8),100u64),((185960u64,8u8),50u64),((185944u64,8u8),192000u64),((185952u64,8u8),0u64),((69744u64,8u8),188416u64),((189984u64,8u8),100u64),((190056u64,8u8),50u64),((190040u64,8u8),192000u64),((190048u64,8u8),0u64),((69752u64,8u8),192512u64),((194080u64,8u8),100u64),((194152u64,8u8),50u64),((194136u64,8u8),192000u64),((194144u64,8u8),0u64),((69760u64,8u8),196608u64),((198176u64,8u8),100u64),((198248u64,8u8),50u64),((198232u64,8u8),192000u64),((198240u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("DC0D80_CAPTURE_LENGTH_ABOVE16"));
}
{ // max_capture16_budget
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69760u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),1000000u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),1063999u64),((132704u64,8u8),0u64),((69640u64,8u8),135168u64),((136736u64,8u8),100u64),((136808u64,8u8),50u64),((136792u64,8u8),1063999u64),((136800u64,8u8),0u64),((69648u64,8u8),139264u64),((140832u64,8u8),100u64),((140904u64,8u8),50u64),((140888u64,8u8),1063999u64),((140896u64,8u8),0u64),((69656u64,8u8),143360u64),((144928u64,8u8),100u64),((145000u64,8u8),50u64),((144984u64,8u8),1063999u64),((144992u64,8u8),0u64),((69664u64,8u8),147456u64),((149024u64,8u8),100u64),((149096u64,8u8),50u64),((149080u64,8u8),1063999u64),((149088u64,8u8),0u64),((69672u64,8u8),151552u64),((153120u64,8u8),100u64),((153192u64,8u8),50u64),((153176u64,8u8),1063999u64),((153184u64,8u8),0u64),((69680u64,8u8),155648u64),((157216u64,8u8),100u64),((157288u64,8u8),50u64),((157272u64,8u8),1063999u64),((157280u64,8u8),0u64),((69688u64,8u8),159744u64),((161312u64,8u8),100u64),((161384u64,8u8),50u64),((161368u64,8u8),1063999u64),((161376u64,8u8),0u64),((69696u64,8u8),163840u64),((165408u64,8u8),100u64),((165480u64,8u8),50u64),((165464u64,8u8),1063999u64),((165472u64,8u8),0u64),((69704u64,8u8),167936u64),((169504u64,8u8),100u64),((169576u64,8u8),50u64),((169560u64,8u8),1063999u64),((169568u64,8u8),0u64),((69712u64,8u8),172032u64),((173600u64,8u8),100u64),((173672u64,8u8),50u64),((173656u64,8u8),1063999u64),((173664u64,8u8),0u64),((69720u64,8u8),176128u64),((177696u64,8u8),100u64),((177768u64,8u8),50u64),((177752u64,8u8),1063999u64),((177760u64,8u8),0u64),((69728u64,8u8),180224u64),((181792u64,8u8),100u64),((181864u64,8u8),50u64),((181848u64,8u8),1063999u64),((181856u64,8u8),0u64),((69736u64,8u8),184320u64),((185888u64,8u8),100u64),((185960u64,8u8),50u64),((185944u64,8u8),1063999u64),((185952u64,8u8),0u64),((69744u64,8u8),188416u64),((189984u64,8u8),100u64),((190056u64,8u8),50u64),((190040u64,8u8),1063999u64),((190048u64,8u8),0u64),((69752u64,8u8),192512u64),((194080u64,8u8),100u64),((194152u64,8u8),50u64),((194136u64,8u8),1063999u64),((194144u64,8u8),0u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,16u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // q_wrap_unsigned
let mut f=Fixture{fields:BTreeMap::from([((65536u64,8u8),69632u64),((65544u64,8u8),69640u64),((65552u64,8u8),73728u64),((65560u64,8u8),77824u64),((73728u64,1u8),4u64),((77832u64,8u8),81920u64),((86720u64,8u8),0u64),((86712u64,8u8),2000000u64),((69632u64,8u8),131072u64),((132640u64,8u8),100u64),((132712u64,8u8),50u64),((132696u64,8u8),192000u64),((132704u64,8u8),1u64)]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
let ev=result.unwrap();assert_eq!(ev.expected,1u64);assert_eq!(ev.mask,18446744073709551615u64);assert!(ev.metrics.iter().all(|m|!m.observed_branch));
}
{ // missing_read_never_default0
let mut f=Fixture{fields:BTreeMap::from([]),reads:Vec::new()};
let result=evaluate(26,[65536u64,0u64,0u64,0u64],&mut f);
assert_eq!(result.err(),Some("W04_FIXTURE_MISSING_READ"));
}
}
