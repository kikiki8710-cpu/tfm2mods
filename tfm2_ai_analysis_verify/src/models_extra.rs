//! round02 추가 순수 모델. 게임 함수·원본 함수·TLS 직접 호출 없이 Memory만 사용한다.
//! id8..15는 entry_proposals.json의 제안 순서다. 설치·실게임 검증 완료를 뜻하지 않는다.
use crate::engine::{Evaluation, Memory, Metric};

pub const FIRST_EXTRA_MODEL_ID: usize = 8;
pub const EXTRA_MODEL_COUNT: usize = 8;
const SLICE_LIMIT: u64 = 32; // 게임 상한이 아니라 96필드 관측 예산: 포인터/HP/최대HP 각1개.

fn read(m: &mut dyn Memory, base: u64, offset: u64, width: u8) -> Result<u64, &'static str> {
    let v = m.read(base.wrapping_add(offset), width)?;
    match width {
        1 => Ok(v & 0xff), 2 => Ok(v & 0xffff), 4 => Ok(v & 0xffff_ffff), 8 => Ok(v),
        _ => Err("EXTRA_INVALID_READ_WIDTH"),
    }
}
fn metric(name: &'static str, lhs: u64, rhs: u64, relation: &'static str, predicate: bool) -> Metric {
    Metric { name, lhs: lhs as i128, rhs: rhs as i128, relation, predicate, observed_branch: false }
}
fn result(expected: u64, mask: u64, metrics: Vec<Metric>) -> Evaluation {
    Evaluation { expected, mask, output_ptr: None, output_fields: Vec::new(), metrics }
}
fn boolean(value: bool, metrics: Vec<Metric>) -> Evaluation { result(u64::from(value), 0xff, metrics) }
fn square_distance(x: u64, y: u64, ax: u64, ay: u64) -> u64 {
    let dx = x.wrapping_sub(ax);
    let dy = y.wrapping_sub(ay);
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}

/// RCX/RDX/R8/R9의 원래 비트값을 받는다. 미지원 도메인/읽기오류를 예상0으로 바꾸지 않는다.
pub fn evaluate(id: usize, args: [u64; 4], memory: &mut dyn Memory) -> Result<Evaluation, &'static str> {
    match id {
        8 | 11 => slice_hp_count(args, memory, 40), // DC0BB0 / F27860
        9 | 10 => slice_hp_count(args, memory, 50), // DC0F80 / F27690
        12 => any_opponent_except_role_one(args, memory), // EF2410
        13 => anchor_or_role_radius(args, memory), // FA9600
        14 => directional_middle_hp(args, memory), // 1019890
        15 => forecast(args, memory), // FE2700
        _ => Err("UNSUPPORTED_EXTRA_MODEL_ID"),
    }
}

// 유효 forward aligned begin/end 범위만 관측한다. 원문에는 이 유효성·32상한 가드가 없다.
fn slice_hp_count(a: [u64; 4], m: &mut dyn Memory, threshold: u64) -> Result<Evaluation, &'static str> {
    if a[1] < a[0] { return Err("HP_SLICE_REVERSED"); }
    let diff = a[1] - a[0];
    if diff & 7 != 0 { return Err("HP_SLICE_UNALIGNED_LENGTH"); }
    let n = diff >> 3;
    if n > SLICE_LIMIT { return Err("HP_SLICE_CAPTURE_CAPACITY"); }
    let mut count = 0u64;
    let mut metrics = Vec::new();
    for i in 0..n {
        let entity = read(m, a[0], i*8, 8)?;
        if entity == 0 { continue; }
        let maximum = read(m, entity, 0x620, 8)?;
        if maximum == 0 { return Err("HP_DIVISION_ZERO_GAME_PANIC_DOMAIN"); }
        let numerator = read(m, entity, 0x668, 8)?.wrapping_mul(100);
        // DIV32/64 선택은 두 피연산자가 모두 u32일 때 같은 unsigned 몫이다.
        let percent = numerator / maximum;
        let included = percent >= threshold;
        metrics.push(metric(if threshold == 40 {"hp_percent_ge40"} else {"hp_percent_ge50"}, percent, threshold, ">=", included));
        count = count.wrapping_add(u64::from(included));
    }
    Ok(result(count, u64::MAX, metrics))
}

// EF2410: 상대 role0/2/3/4만 조회한다. 150000²+1와 unsigned strict 비교 → 경계포함.
fn any_opponent_except_role_one(a: [u64; 4], m: &mut dyn Memory) -> Result<Evaluation, &'static str> {
    let opponent = 1u64.wrapping_sub(a[0]);
    if opponent > 1 { return Err("EF2410_TEAM_BOUNDS_GAME_PANIC_DOMAIN"); }
    let slots = a[1].wrapping_add(0x238 + opponent*40);
    let mut metrics = Vec::new();
    for offset in [0, 16, 24, 32] {
        let entity = read(m, slots, offset, 8)?;
        if entity == 0 { continue; }
        let x = read(m, entity, 0x658, 8)?;
        let y = read(m, entity, 0x660, 8)?;
        let d2 = square_distance(x, y, a[2], a[3]);
        let inside = d2 < 22_500_000_001;
        metrics.push(metric("opponent_except_role1_d2", d2, 22_500_000_001, "<", inside));
        if inside { return Ok(boolean(true, metrics)); }
    }
    Ok(boolean(false, metrics))
}

// FA9600: candidate null/동일ID 단락 뒤에만 team/role/roster/radius를 읽는다.
fn anchor_or_role_radius(a: [u64; 4], m: &mut dyn Memory) -> Result<Evaluation, &'static str> {
    let candidate_ref = read(m, a[1], 8, 8)?;
    let candidate = read(m, candidate_ref, 0, 8)?;
    if candidate == 0 { return Ok(boolean(false, Vec::new())); }
    let capture = read(m, a[0], 0, 8)?;
    let anchor = read(m, capture, 0, 8)?;
    let candidate_id = read(m, candidate, 0x5b8, 8)?;
    let anchor_id = read(m, anchor, 0x5b8, 8)?;
    let same = candidate_id == anchor_id;
    let mut metrics = vec![metric("same_anchor_id", candidate_id, anchor_id, "==", same)];
    if same { return Ok(boolean(true, metrics)); }
    let player = read(m, capture, 0x10, 8)?;
    let opponent = 1u64.wrapping_sub(read(m, player, 0x800, 8)?);
    if opponent > 1 { return Err("FA9600_TEAM_BOUNDS_GAME_PANIC_DOMAIN"); }
    let role_ref = read(m, capture, 0x18, 8)?;
    let role = read(m, role_ref, 0, 8)?;
    if role >= 5 { return Err("FA9600_ROLE_BOUNDS_GAME_PANIC_DOMAIN"); }
    let cache = read(m, capture, 8, 8)?;
    let target = read(m, cache, 0x238 + opponent*40 + role*8, 8)?;
    if target == 0 { return Err("FA9600_MISSING_ROLE_GAME_PANIC_DOMAIN"); }
    let radius_ref = read(m, capture, 0x20, 8)?;
    let cx = read(m, candidate, 0x658, 8)?;
    let cy = read(m, candidate, 0x660, 8)?;
    let tx = read(m, target, 0x658, 8)?;
    let ty = read(m, target, 0x660, 8)?;
    let radius = read(m, radius_ref, 0, 8)?; // 원문처럼 pointee는 XY 뒤에 읽는다.
    let d2 = square_distance(cx, cy, tx, ty);
    let r2 = radius.wrapping_mul(radius);
    let inside = d2 <= r2;
    metrics.push(metric("role_radius_squared_inclusive", d2, r2, "<=", inside));
    Ok(boolean(inside, metrics))
}

// 1019890: wrapHP*100/DIV 뒤 rawtag4/5의 서로 다른 단락 순서를 보존한다.
fn directional_middle_hp(a: [u64; 4], m: &mut dyn Memory) -> Result<Evaluation, &'static str> {
    let entity = read(m, a[1], 8, 8)?;
    let maximum = read(m, entity, 0x620, 8)?;
    if maximum == 0 { return Err("MIDDLE_HP_DIVISION_ZERO_GAME_PANIC_DOMAIN"); }
    let capture = read(m, a[0], 0, 8)?;
    let tag_ref = read(m, capture, 0, 8)?;
    let numerator = read(m, entity, 0x668, 8)?.wrapping_mul(100);
    let context = read(m, capture, 8, 8)?;
    let percent = numerator / maximum;
    let mut metrics = vec![metric("middle_minimum_hp40", percent, 40, ">=", percent >= 40)];
    if percent < 40 { return Ok(boolean(false, metrics)); }
    let tag = read(m, tag_ref, 0, 1)?;
    if tag != 4 && tag != 5 { return Ok(boolean(false, metrics)); }
    let x = read(m, entity, 0x658, 8)?;
    if tag == 4 {
        metrics.push(metric("middle_tag4_x_le192000", x, 192_000, "<=", x <= 192_000));
        if x <= 192_000 { return Ok(boolean(true, metrics)); }
    }
    let settings = read(m, context, 8, 8)?;
    let height = read(m, settings, 0x12c0, 8)?;
    let q = height.wrapping_sub(read(m, entity, 0x660, 8)?);
    let inside = if tag == 4 {
        let difference = x.wrapping_sub(q);
        metrics.push(metric("middle_tag4_x_le_q", x, q, "<=", x <= q));
        if x <= q { true }
        else {
            let cut = read(m, settings, 0x12b8, 8)?.wrapping_sub(192_000);
            metrics.push(metric("middle_tag4_q_ge_width_minus192000", q, cut, ">=", q >= cut));
            if q >= cut { true }
            else {
                metrics.push(metric("middle_tag4_gap_lt64000", difference, 64_000, "<", difference < 64_000));
                difference < 64_000
            }
        }
    } else {
        metrics.push(metric("middle_tag5_q_le_x", q, x, "<=", q <= x));
        metrics.push(metric("middle_tag5_q_le192000", q, 192_000, "<=", q <= 192_000));
        if q <= x || q <= 192_000 { true }
        else {
            let cut = read(m, settings, 0x12b8, 8)?.wrapping_sub(192_000);
            metrics.push(metric("middle_tag5_x_ge_width_minus192000", x, cut, ">=", x >= cut));
            if x >= cut { true }
            else {
                let difference = q.wrapping_sub(x);
                metrics.push(metric("middle_tag5_gap_lt64000", difference, 64_000, "<", difference < 64_000));
                difference < 64_000
            }
        }
    };
    Ok(boolean(inside, metrics))
}

// FE2700: DD87A0와 동형이나 별도RVA/본문핀. 원값5/18/90과wrap64·signed /5를유지한다.
fn forecast(a: [u64; 4], m: &mut dyn Memory) -> Result<Evaluation, &'static str> {
    let (record, time) = (a[0], a[1]);
    if time == 0 { return Ok(result(read(m, record, 0x98, 8)?, u64::MAX, Vec::new())); }
    let count = read(m, record, 0xa0, 8)?;
    if count > 18 { return Err("FE2700_SAMPLE_CAPTURE_CAPACITY"); }
    let index = time/5;
    let mut metrics = vec![metric("forecast_tail_index_ge_count", index, count, ">=", index >= count)];
    if index >= count {
        let offset = if count == 0 {0x98} else {(count-1)*8};
        return Ok(result(read(m, record, offset, 8)?, u64::MAX, metrics));
    }
    metrics.push(metric("forecast_first_interval_lt5", time, 5, "<", time < 5));
    let (before, after, remainder) = if time < 5 {
        let after = read(m, record, 0, 8)?;
        let before = read(m, record, 0x98, 8)?;
        (before, after, time)
    } else {
        if index == 0 || index-1 > 17 || time >= 90 { return Err("FE2700_INTERPOLATION_GAME_PANIC_DOMAIN"); }
        (read(m, record, (index-1)*8, 8)?, read(m, record, index*8, 8)?, time-index*5)
    };
    let delta = after.wrapping_sub(before).wrapping_mul(remainder) as i64;
    Ok(result(before.wrapping_add((delta/5) as u64), u64::MAX, metrics))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    #[derive(Default)]
    struct Fixture { fields: BTreeMap<(u64,u8),u64>, reads: Vec<(u64,u8)> }
    impl Fixture {
        fn put(&mut self,a:u64,w:u8,v:u64) { self.fields.insert((a,w),v); }
        fn q(&mut self,a:u64,v:u64) { self.put(a,8,v); }
        fn xy(&mut self,a:u64,x:u64,y:u64) { self.q(a+0x658,x); self.q(a+0x660,y); }
    }
    impl Memory for Fixture {
        fn read(&mut self,a:u64,w:u8)->Result<u64,&'static str> {
            self.reads.push((a,w)); self.fields.get(&(a,w)).copied().ok_or("EXTRA_FIXTURE_MISSING_READ")
        }
    }
    fn val(id:usize,a:[u64;4],f:&mut Fixture)->u64 {
        let e=evaluate(id,a,f).unwrap();
        assert!(e.metrics.iter().all(|m|!m.observed_branch));
        assert!(e.output_fields.is_empty()); e.expected
    }
    #[test]
    fn hp_four_functions_threshold_minus_equal_plus() {
        for (id,t) in [(8,40),(9,50),(10,50),(11,40)] {
            for (hp,want) in [(t-1,0),(t,1),(t+1,1)] {
                let mut f=Fixture::default();f.q(0x1000,0x2000);f.q(0x2620,100);f.q(0x2668,hp);
                let e=evaluate(id,[0x1000,0x1008,0,0],&mut f).unwrap();
                assert_eq!((e.expected,e.mask),(want,u64::MAX));
            }
        }
    }
    #[test]
    fn hp_empty_null_div64_and_wrapped_numerator() {
        let mut f=Fixture::default();assert_eq!(val(8,[0x1000,0x1000,0,0],&mut f),0);assert!(f.reads.is_empty());
        f.q(0x1000,0);assert_eq!(val(8,[0x1000,0x1008,0,0],&mut f),0);
        f.q(0x1000,0x2000);f.q(0x2620,u64::MAX);f.q(0x2668,u64::MAX);
        assert_eq!(val(8,[0x1000,0x1008,0,0],&mut f),0); // 무한정밀100배라면 결과가 다르다.
        f.q(0x2620,1u64<<40);f.q(0x2668,1u64<<39);assert_eq!(val(9,[0x1000,0x1008,0,0],&mut f),1);
    }
    #[test]
    fn hp_invalid_slice_capacity_read_and_zero_divisor_skip() {
        let mut f=Fixture::default();
        for a in [[0x1008,0x1000,0,0],[0x1000,0x1001,0,0],[0x1000,0x1108,0,0],[0x1000,0x1008,0,0]] {
            assert!(evaluate(8,a,&mut f).is_err());
        }
        f.q(0x1000,0x2000);f.q(0x2620,0);assert!(evaluate(8,[0x1000,0x1008,0,0],&mut f).is_err());
        assert!(!f.reads.contains(&(0x2668,8)));
    }
    #[test]
    fn hp_maximum_capture_budget32() {
        let mut f=Fixture::default();
        for j in 0..32 {let e=0x2000+j*0x1000;f.q(0x1000+j*8,e);f.q(e+0x620,100);f.q(e+0x668,40);}
        assert_eq!(val(8,[0x1000,0x1100,0,0],&mut f),32);assert_eq!(f.reads.len(),96);
    }
    #[test]
    fn opponent_radius_inclusive_and_role1_not_consumed() {
        for (distance,want) in [(149_999,1),(150_000,1),(150_001,0),(1u64<<63,1)] {
            let mut f=Fixture::default();let slots=0x2000+0x238+40;
            for off in [0,16,24,32] {f.q(slots+off,0);}
            f.q(slots,0x3000);f.xy(0x3000,distance,0);
            assert_eq!(val(12,[0,0x2000,0,0],&mut f),want);assert!(!f.reads.contains(&(slots+8,8)));
        }
    }
    #[test]
    fn opponent_invalid_team_and_unreadable_skip() {
        let mut f=Fixture::default();assert!(evaluate(12,[2,0,0,0],&mut f).is_err());assert!(f.reads.is_empty());
        assert!(evaluate(12,[0,0x2000,0,0],&mut f).is_err());
    }
    fn role_radius_fixture(radius:u64)->Fixture {
        let mut f=Fixture::default();f.q(0x1000,0x3000);f.q(0x2008,0x2100);f.q(0x2100,0x4000);
        f.q(0x3000,0x5000);f.q(0x45b8,1);f.q(0x55b8,2);f.q(0x3010,0x6000);f.q(0x6800,0);
        f.q(0x3018,0x7000);f.q(0x7000,0);f.q(0x3008,0x8000);f.q(0x8000+0x238+40,0x9000);
        f.q(0x3020,0xa000);f.xy(0x4000,100,0);f.xy(0x9000,0,0);f.q(0xa000,radius);f
    }
    #[test]
    fn role_radius_threshold_and_late_read_order() {
        for (radius,want) in [(99,0),(100,1),(101,1),(1u64<<63,0)] {
            let mut f=role_radius_fixture(radius);let e=evaluate(13,[0x1000,0x2000,0,0],&mut f).unwrap();
            assert_eq!((e.expected,e.mask),(want,0xff));assert_eq!(f.reads.last(),Some(&(0xa000,8)));
        }
    }
    #[test]
    fn role_radius_shortcuts_and_panic_domains() {
        let mut f=Fixture::default();f.q(0x2008,0x2100);f.q(0x2100,0);
        assert_eq!(val(13,[0x1000,0x2000,0,0],&mut f),0);assert_eq!(f.reads.len(),2);
        let mut f=role_radius_fixture(100);f.q(0x45b8,2);f.fields.remove(&(0x3010,8));
        assert_eq!(val(13,[0x1000,0x2000,0,0],&mut f),1);
        let mut f=role_radius_fixture(100);f.q(0x6800,2);assert!(evaluate(13,[0x1000,0x2000,0,0],&mut f).is_err());
        let mut f=role_radius_fixture(100);f.q(0x7000,5);assert!(evaluate(13,[0x1000,0x2000,0,0],&mut f).is_err());
        let mut f=role_radius_fixture(100);f.q(0x8000+0x238+40,0);assert!(evaluate(13,[0x1000,0x2000,0,0],&mut f).is_err());
    }
    fn middle_fixture(tag:u64,hp:u64,x:u64,q:u64)->Fixture {
        let mut f=Fixture::default();f.q(0x2008,0x3000);f.q(0x3620,100);f.q(0x3668,hp);
        f.q(0x1000,0x4000);f.q(0x4000,0x5000);f.q(0x4008,0x6000);f.put(0x5000,1,tag);
        f.xy(0x3000,x,0);f.q(0x6008,0x7000);f.q(0x7000+0x12c0,q);f.q(0x7000+0x12b8,2_000_000);f
    }
    #[test]
    fn middle_hp40_and_early_tag4_edge() {
        for (hp,want) in [(39,0),(40,1),(41,1)] {
            let mut f=middle_fixture(4,hp,192_000,1_000_000);f.fields.remove(&(0x6008,8));
            assert_eq!(val(14,[0x1000,0x2000,0,0],&mut f),want);
        }
        let mut f=middle_fixture(3,40,0,0);f.fields.remove(&(0x3658,8));assert_eq!(val(14,[0x1000,0x2000,0,0],&mut f),0);
    }
    #[test]
    fn middle_directional_strict64000_boundaries() {
        for (d,want) in [(63_999,1),(64_000,0),(64_001,0)] {
            let mut f=middle_fixture(4,40,1_000_000+d,1_000_000);assert_eq!(val(14,[0x1000,0x2000,0,0],&mut f),want);
            let mut f=middle_fixture(5,40,1_000_000,1_000_000+d);assert_eq!(val(14,[0x1000,0x2000,0,0],&mut f),want);
        }
    }
    #[test]
    fn forecast_spacing_signed_wrap_and_capture_failures() {
        let mut f=Fixture::default();f.q(0x1098,0);f.q(0x10a0,18);
        for j in 0..18 {f.q(0x1000+j*8,(j+1)*10);}
        for t in [0,1,4,5,6,89,90,91,u64::MAX] {assert_eq!(val(15,[0x1000,t,0,0],&mut f),t.saturating_mul(2).min(180));}
        f.q(0x1098,10);f.q(0x1000,(-11i64)as u64);assert_eq!(val(15,[0x1000,1,0,0],&mut f),6);
        f.q(0x1098,i64::MAX as u64);f.q(0x1000,i64::MIN as u64);assert_eq!(val(15,[0x1000,4,0,0],&mut f),i64::MAX as u64);
        f.q(0x10a0,19);assert!(evaluate(15,[0x1000,1,0,0],&mut f).is_err());
        let mut f=Fixture::default();assert!(evaluate(15,[0x1000,0,0,0],&mut f).is_err());
    }
    #[test]
    fn unsupported_extra_id_is_error() {
        let mut f=Fixture::default();assert!(evaluate(99,[0;4],&mut f).is_err());
    }
}
