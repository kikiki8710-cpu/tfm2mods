//! 고정0.6.4 로컬계약의 독립 예상값. 게임함수 호출/원본 결과 변경 없음.
//! id 순서는 probe_scope.json과 같다. 관측하지 않은 내부 분기 metric은 모두 false다.

use crate::engine::{Evaluation, Memory, Metric};

const U64_MASK: u64 = u64::MAX;
const AL_MASK: u64 = 0xff;
const RECORD_CAP: u64 = 32; // 게임 상한이 아니라 MAX_FIELDS=96의 관측 예산이다.

fn read(m: &mut dyn Memory, base: u64, offset: u64, width: u8) -> Result<u64, &'static str> {
    let value = m.read(base.wrapping_add(offset), width)?;
    match width {
        1 => Ok(value & 0xff),
        2 => Ok(value & 0xffff),
        4 => Ok(value & 0xffff_ffff),
        8 => Ok(value),
        _ => Err("MODEL_INVALID_READ_WIDTH"),
    }
}

fn result(expected: u64, mask: u64, metrics: Vec<Metric>) -> Evaluation {
    Evaluation { expected, mask, output_ptr: None, output_fields: Vec::new(), metrics }
}

fn metric(name: &'static str, lhs: u64, rhs: u64, relation: &'static str, predicate: bool) -> Metric {
    Metric { name, lhs: lhs as i128, rhs: rhs as i128, relation, predicate, observed_branch: false }
}

fn boolean(value: bool, metrics: Vec<Metric>) -> Evaluation {
    result(u64::from(value), AL_MASK, metrics)
}

/// 제곱합은 원문의 low64 곱/합이다. SUB 절대값 선택의 부호반전은 제곱의 모듈러값을 바꾸지 않는다.
fn distance_squared(x: u64, y: u64, ax: u64, ay: u64) -> u64 {
    let dx = x.wrapping_sub(ax);
    let dy = y.wrapping_sub(ay);
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}

/// args는 RCX/RDX/R8/R9 원값. 지원하지 않는 입력을 예상값0으로 바꾸지 않는다.
pub fn evaluate(id: usize, args: [u64; 4], memory: &mut dyn Memory) -> Result<Evaluation, &'static str> {
    match id {
        0 => forecast(args, memory),
        1 => major_slots(args, memory),
        2 => first_present_role(args, memory),
        3 => all_near_enemy_nexus(args, memory),
        4 => same_lane_near_role(args, memory),
        5 => dual_timestamp(args, memory),
        6 => record_hp_gate(args, memory),
        7 => forecast_record(args, memory),
        8..=15 => crate::models_extra::evaluate(id,args,memory),
        16..=18 => crate::models_wave03::evaluate(id,args,memory),
        19..=26 => crate::models_wave04::evaluate(id,args,memory),
        27|28|32..=36 => crate::models_wave05_direct::evaluate(id,args,memory),
        29..=31 => crate::models_wave05_outputs::evaluate(id,args,memory),
        37 => crate::models_wave06_direct::evaluate(37,args,memory),
        38 => crate::models_wave06_iterator::evaluate(39,args,memory),
        39 => crate::models_d8a1f0::evaluate(args,memory),
        40 => crate::models_next::evaluate(1000,args,memory),
        41 => crate::models_candidates::evaluate(0xd8c680,args,memory),
        42|43 => crate::models_wave08_direct::evaluate(id,args,memory),
        _ => Err("UNKNOWN_MODEL_ID"),
    }
}

// DD87A0: t/count는unsigned, 보간은wrap64 후 signed /5의 0방향절삭이다.
fn forecast(a: [u64; 4], m: &mut dyn Memory) -> Result<Evaluation, &'static str> {
    let (record, time) = (a[0], a[1]);
    if time == 0 { return Ok(result(read(m, record, 0x98, 8)?, U64_MASK, Vec::new())); }
    let count = read(m, record, 0xa0, 8)?;
    if count > 18 { return Err("FORECAST_SAMPLE_CAPACITY"); }
    let index = time / 5;
    if index >= count {
        let offset = if count == 0 { 0x98 } else { (count - 1) * 8 };
        return Ok(result(read(m, record, offset, 8)?, U64_MASK, Vec::new()));
    }
    let (before, after, remainder) = if time < 5 {
        let after = read(m, record, 0, 8)?;
        let before = read(m, record, 0x98, 8)?;
        (before, after, time)
    } else {
        if index == 0 || index - 1 > 17 || time >= 90 { return Err("FORECAST_INTERPOLATION_BOUNDS"); }
        let before = read(m, record, (index - 1) * 8, 8)?;
        let after = read(m, record, index * 8, 8)?;
        (before, after, time - 5 * index)
    };
    let product = after.wrapping_sub(before).wrapping_mul(remainder) as i64;
    let interpolated = before.wrapping_add((product / 5) as u64);
    let metrics = vec![metric("forecast_remainder_is_zero", remainder, 0, "==", remainder == 0)];
    Ok(result(interpolated, U64_MASK, metrics))
}

// E52EE0: slot1/4의 존재나 entity 본문은 이 함수가 읽지 않는다.
fn major_slots(a: [u64; 4], m: &mut dyn Memory) -> Result<Evaluation, &'static str> {
    let team = a[1];
    if team > 1 { return Err("TEAM_BOUNDS"); }
    let world = read(m, a[0], 0, 8)?;
    let base = world.wrapping_add(0x238 + team * 40);
    for offset in [0, 16, 24] {
        if read(m, base, offset, 8)? != 0 { return Ok(boolean(true, Vec::new())); }
    }
    Ok(boolean(false, Vec::new()))
}

// D98BD0: MOV EDX,EDX의 low32 소비. allnull sentinel5는 항상 false다.
fn first_present_role(a: [u64; 4], m: &mut dyn Memory) -> Result<Evaluation, &'static str> {
    if a[0] > 1 { return Err("TEAM_BOUNDS"); }
    let base = a[2].wrapping_add(0x238 + a[0] * 40);
    for role in 0..5u64 {
        if read(m, base, role * 8, 8)? != 0 {
            return Ok(boolean(role == (a[1] as u32 as u64), Vec::new()));
        }
    }
    Ok(boolean(false, Vec::new()))
}

// E49C00: null아군은 제외하며 빈팀true. unsigned 600000² 포함비교.
fn all_near_enemy_nexus(a: [u64; 4], m: &mut dyn Memory) -> Result<Evaluation, &'static str> {
    let opposite = 1u64.wrapping_sub(a[0]);
    if opposite > 1 { return Err("TEAM_BOUNDS"); }
    let nexus = read(m, a[1], 0x1c8 + opposite * 8, 8)?;
    if nexus == 0 { return Ok(boolean(false, Vec::new())); }
    let nx = read(m, nexus, 0x658, 8)?;
    let ny = read(m, nexus, 0x660, 8)?;
    let base = a[1].wrapping_add(0x238 + a[0] * 40);
    let mut metrics = Vec::new();
    for role in 0..5u64 {
        let entity = read(m, base, role * 8, 8)?;
        if entity == 0 { continue; }
        let x = read(m, entity, 0x658, 8)?;
        let y = read(m, entity, 0x660, 8)?;
        let squared = distance_squared(x, y, nx, ny);
        let inside = squared <= 360_000_000_000;
        metrics.push(metric("all_near_nexus_radius_squared", squared, 360_000_000_000, "<=", inside));
        if !inside { return Ok(boolean(false, metrics)); }
    }
    Ok(boolean(true, metrics))
}

// FA9DC0: captureRef 포인터층을 생략하지 않는다. 150000 동률은 거절한다.
fn same_lane_near_role(a: [u64; 4], m: &mut dyn Memory) -> Result<Evaluation, &'static str> {
    let capture = read(m, a[0], 0, 8)?;
    let role = read(m, a[1], 0, 8)?;
    let own_role_ptr = read(m, capture, 0, 8)?;
    let own_role = read(m, own_role_ptr, 0, 8)?;
    if role == own_role { return Ok(boolean(false, Vec::new())); }
    let team_ptr = read(m, capture, 0x18, 8)?;
    let team = read(m, team_ptr, 0, 8)?;
    if team > 1 { return Err("TEAM_BOUNDS"); }
    if role > 4 { return Err("ROLE_BOUNDS"); }
    let bb = read(m, capture, 0x10, 8)?;
    let row = bb.wrapping_add(team * 0xac8 + role * 32);
    if read(m, row, 0x7f0, 1)? != 0 { return Ok(boolean(false, Vec::new())); }
    let extra = read(m, capture, 0x20, 8)?;
    let lane = read(m, extra, 0x11c, 1)?;
    if read(m, row, 0x7f1, 1)? != lane { return Ok(boolean(false, Vec::new())); }
    let world = read(m, capture, 8, 8)?;
    let entity = read(m, world, 0x238 + team * 40 + role * 8, 8)?;
    if entity == 0 { return Ok(boolean(false, Vec::new())); }
    let anchor = read(m, capture, 0x28, 8)?;
    let x = read(m, entity, 0x658, 8)?;
    let y = read(m, entity, 0x660, 8)?;
    let ax = read(m, anchor, 0x658, 8)?;
    let ay = read(m, anchor, 0x660, 8)?;
    let quantized = distance_squared(x, y, ax, ay) >> 8;
    let inside = quantized < 87_890_625;
    Ok(boolean(inside, vec![metric("same_lane_radius_shift8", quantized, 87_890_625, "<", inside)]))
}

// F51E50: outerRef→outerCapture→innerCapture 두 층과 now/limit 재독 순서를 유지한다.
fn dual_timestamp(a: [u64; 4], m: &mut dyn Memory) -> Result<Evaluation, &'static str> {
    let outer = read(m, a[0], 0, 8)?;
    let inner = read(m, outer, 0, 8)?;
    let player = read(m, inner, 8, 8)?;
    let opposite = 1u64.wrapping_sub(read(m, player, 0x800, 8)?);
    if opposite > 1 { return Err("TEAM_BOUNDS"); }
    let candidate = read(m, a[1], 0, 8)?;
    let world = read(m, inner, 0, 8)?;
    let candidate_id = read(m, candidate, 0x5b8, 8)?;
    let mut matched = None;
    for role in 0..5u64 {
        let entity = read(m, world, 0x238 + opposite * 40 + role * 8, 8)?;
        if entity != 0 && read(m, entity, 0x5b8, 8)? == candidate_id {
            matched = Some(role);
            break;
        }
    }
    let role = match matched { Some(role) => role, None => return Ok(boolean(true, Vec::new())) };
    let first_ptr = read(m, inner, 0x10, 8)?;
    let first = read(m, first_ptr, role * 8, 8)?;
    let mut metrics = Vec::new();
    if first != 0 {
        let now_ptr = read(m, inner, 0x18, 8)?;
        let now = read(m, now_ptr, 0, 8)?;
        let limit_ptr = read(m, inner, 0x20, 8)?;
        let limit = read(m, limit_ptr, 0, 8)?;
        let age = now.saturating_sub(first);
        let recent = age <= limit;
        metrics.push(metric("first_stamp_recent", age, limit, "<=", recent));
        if recent { return Ok(boolean(true, metrics)); }
    }
    let second_ptr = read(m, inner, 0x28, 8)?;
    let second = read(m, second_ptr, role * 8, 8)?;
    if second == 0 { return Ok(boolean(true, metrics)); }
    let now_ptr = read(m, inner, 0x18, 8)?;
    let limit_ptr = read(m, inner, 0x20, 8)?;
    let now = read(m, now_ptr, 0, 8)?;
    let age = now.saturating_sub(second);
    let limit = read(m, limit_ptr, 0, 8)?;
    let expired = age > limit;
    metrics.push(metric("second_stamp_expired", age, limit, ">", expired));
    Ok(boolean(expired, metrics))
}

// E9EB70: 32record는 관측예산이다. HP×100 wrap 뒤 unsigned 나눗셈을 한다.
fn record_hp_gate(a: [u64; 4], m: &mut dyn Memory) -> Result<Evaluation, &'static str> {
    let player = a[1];
    let data = a[2];
    let team = read(m, player, 0x800, 8)?;
    if team > 1 { return Err("TEAM_BOUNDS"); }
    let bb = read(m, data, 0x10, 8)?;
    let bank = bb.wrapping_add(team * 0xac8);
    let count = read(m, bank, 0x2c0, 8)?;
    if count == 0 { return Ok(boolean(false, Vec::new())); }
    if count > RECORD_CAP { return Err("RECORD_CAPTURE_CAPACITY"); }
    let role = read(m, player, 0x890, 4)?;
    if role > 4 { return Err("ROLE_CAPTURE_DOMAIN"); }
    let records = read(m, bank, 0x2b8, 8)?;
    for index in 0..count {
        let record = records.wrapping_add(index * 80);
        if read(m, record, 0x46, 1)? != 0 { continue; }
        if read(m, record, 0x40 + role, 1)? == 0 { continue; }
        let world = read(m, data, 0, 8)?;
        let entity = read(m, world, 0x238 + team * 40 + role * 8, 8)?;
        if entity == 0 { return Ok(boolean(false, Vec::new())); }
        let maximum = read(m, entity, 0x620, 8)?.max(1);
        let hp = read(m, entity, 0x668, 8)?;
        let numerator = hp.wrapping_mul(100);
        // 두 피연산자가32bit면 DIV32와 같고, 그외는 원문 DIV64와 같은 몫이다.
        let percent = numerator / maximum;
        let accepted = percent >= 25;
        return Ok(boolean(accepted, vec![metric("record_hp_minimum_percent", percent, 25, ">=", accepted)]));
    }
    Ok(boolean(false, Vec::new()))
}

// FDB9A0: 12×176B 표. record 내용이 아닌 같은 버퍼의 포인터를 반환한다.
fn forecast_record(a: [u64; 4], m: &mut dyn Memory) -> Result<Evaluation, &'static str> {
    let count = read(m, a[0], 0x840, 8)?;
    if count > 12 { return Err("FORECAST_RECORD_CAPACITY"); }
    for index in 0..count {
        let record = a[0].wrapping_add(index * 176);
        if read(m, record, 0x90, 8)? == a[1] { return Ok(result(record, U64_MASK, Vec::new())); }
    }
    Ok(result(0, U64_MASK, Vec::new()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[derive(Default)]
    struct Fixture { fields: BTreeMap<(u64, u8), u64>, reads: usize }
    impl Fixture {
        fn put(&mut self, address: u64, width: u8, value: u64) { self.fields.insert((address, width), value); }
        fn q(&mut self, address: u64, value: u64) { self.put(address, 8, value); }
        fn xy(&mut self, entity: u64, x: u64, y: u64) { self.q(entity + 0x658, x); self.q(entity + 0x660, y); }
    }
    impl Memory for Fixture {
        fn read(&mut self, address: u64, width: u8) -> Result<u64, &'static str> {
            self.reads += 1;
            self.fields.get(&(address, width)).copied().ok_or("FIXTURE_MISSING_FIELD")
        }
    }
    fn value(id: usize, args: [u64; 4], f: &mut Fixture) -> u64 {
        let evaluation = evaluate(id, args, f).unwrap();
        assert!(evaluation.metrics.iter().all(|m| !m.observed_branch));
        evaluation.expected
    }

    #[test]
    fn forecast_spacing_tail_signed_and_wrap() {
        let mut f = Fixture::default();
        f.q(0x1098, 0); f.q(0x10a0, 18);
        for j in 0..18 { f.q(0x1000 + j * 8, (j + 1) * 10); }
        for t in [0, 1, 4, 5, 6, 89, 90, 91] {
            assert_eq!(value(0, [0x1000, t, 0, 0], &mut f), (t * 2).min(180));
        }
        f.q(0x1098, 10); f.q(0x1000, (-11i64) as u64);
        assert_eq!(value(0, [0x1000, 1, 0, 0], &mut f), 6); // -21/5는0방향절삭으로-4.
        f.q(0x1098, i64::MAX as u64); f.q(0x1000, i64::MIN as u64);
        assert_eq!(value(0, [0x1000, 4, 0, 0], &mut f), i64::MAX as u64);
        f.q(0x10a0, 0); assert_eq!(value(0, [0x1000, 90, 0, 0], &mut f), i64::MAX as u64);
        f.q(0x10a0, 19); assert!(evaluate(0, [0x1000, 1, 0, 0], &mut f).is_err());
    }

    #[test]
    fn three_slots_exhaustive_presence_and_al_mask() {
        for mask in 0..8u64 {
            let mut f = Fixture::default(); f.q(0x1000, 0x2000);
            for (bit, offset) in [0, 16, 24].iter().enumerate() { f.q(0x2238 + offset, (mask >> bit) & 1); }
            let e = evaluate(1, [0x1000, 0, 0, 0], &mut f).unwrap();
            assert_eq!(e.expected, u64::from(mask != 0)); assert_eq!(e.mask, 0xff);
        }
    }

    #[test]
    fn first_role_low32_all_masks_and_empty_sentinel() {
        for mask in 0..32u64 {
            for requested in 0..6u64 {
                let mut f = Fixture::default();
                for j in 0..5 { f.q(0x2238 + j * 8, (mask >> j) & 1); }
                let expected = mask != 0 && mask.trailing_zeros() as u64 == requested;
                assert_eq!(value(2, [0, requested | (1u64 << 32), 0x2000, 0], &mut f), u64::from(expected));
            }
        }
    }

    #[test]
    fn nexus_inclusive_radius_empty_and_wrapped_square() {
        for (distance, expected) in [(599_999, 1), (600_000, 1), (600_001, 0), (1u64 << 63, 1)] {
            let mut f = Fixture::default(); f.q(0x21d0, 0x3000); f.xy(0x3000, 0, 0);
            for j in 0..5 { f.q(0x2238 + j * 8, 0); }
            f.q(0x2258, 0x4000); f.xy(0x4000, distance, 0);
            assert_eq!(value(3, [0, 0x2000, 0, 0], &mut f), expected);
            f.q(0x2258, 0); assert_eq!(value(3, [0, 0x2000, 0, 0], &mut f), 1);
            f.q(0x21d0, 0); assert_eq!(value(3, [0, 0x2000, 0, 0], &mut f), 0);
        }
    }

    fn lane_fixture(distance: u64) -> Fixture {
        let mut f = Fixture::default();
        f.q(0x1000, 0x2000); f.q(0x1100, 1); f.q(0x2000, 0x1200); f.q(0x1200, 0);
        f.q(0x2018, 0x1300); f.q(0x1300, 0); f.q(0x2010, 0x4000);
        f.put(0x4810, 1, 0); f.put(0x4811, 1, 7); f.q(0x2020, 0x5000); f.put(0x511c, 1, 7);
        f.q(0x2008, 0x6000); f.q(0x6240, 0x7000); f.q(0x2028, 0x8000);
        f.xy(0x7000, distance, 0); f.xy(0x8000, 0, 0); f
    }

    #[test]
    fn lane_strict_radius_and_byte_guards() {
        for (distance, expected) in [(149_999, 1), (150_000, 0), (150_001, 0)] {
            let mut f = lane_fixture(distance); assert_eq!(value(4, [0x1000, 0x1100, 0, 0], &mut f), expected);
        }
        let mut f = lane_fixture(1); f.put(0x4811, 1, 8); assert_eq!(value(4, [0x1000, 0x1100, 0, 0], &mut f), 0);
        f.put(0x4811, 1, 7); f.put(0x4810, 1, 1); assert_eq!(value(4, [0x1000, 0x1100, 0, 0], &mut f), 0);
        f.q(0x1100, 1u64 << 32); assert!(evaluate(4, [0x1000, 0x1100, 0, 0], &mut f).is_err());
    }

    fn timestamps(first: u64, second: u64, now: u64, limit: u64) -> Fixture {
        let mut f = Fixture::default(); f.q(0x1000, 0x2000); f.q(0x2000, 0x3000);
        f.q(0x3008, 0x4000); f.q(0x4800, 0); f.q(0x1100, 0x5000); f.q(0x55b8, 99);
        f.q(0x3000, 0x6000);
        for j in 0..5 { f.q(0x6260 + j * 8, if j == 2 { 0x7000 } else { 0 }); }
        f.q(0x75b8, 99); f.q(0x3010, 0x8000); f.q(0x8010, first);
        f.q(0x3028, 0x9000); f.q(0x9010, second); f.q(0x3018, 0xa000); f.q(0xa000, now);
        f.q(0x3020, 0xb000); f.q(0xb000, limit); f
    }

    #[test]
    fn timestamps_threshold_priority_zero_and_future() {
        for (first, second, expected) in [(89,90,0),(90,90,1),(91,90,1),(0,89,1),(0,90,0),(0,91,0),(0,101,0),(0,0,1)] {
            let mut f = timestamps(first, second, 100, 10);
            assert_eq!(value(5, [0x1000,0x1100,0,0], &mut f), expected);
        }
        let mut f = timestamps(101, 100, 100, 0); assert_eq!(value(5,[0x1000,0x1100,0,0],&mut f),1);
        let mut f = timestamps(0, u64::MAX, 0, 0); assert_eq!(value(5,[0x1000,0x1100,0,0],&mut f),0);
    }

    fn hp_fixture(hp: u64, maximum: u64) -> Fixture {
        let mut f = Fixture::default(); f.q(0x1800, 0); f.put(0x1890, 4, 2); f.q(0x2010, 0x3000);
        f.q(0x32c0, 1); f.q(0x32b8, 0x4000); f.put(0x4046,1,0); f.put(0x4042,1,2);
        f.q(0x2000,0x5000); f.q(0x5248,0x6000); f.q(0x6620,maximum); f.q(0x6668,hp); f
    }

    #[test]
    fn hp_percent_threshold_zero_denominator_wrap_and_capacity() {
        for (hp, maximum, expected) in [(24,100,0),(25,100,1),(26,100,1),(0,0,0),(1,0,1),(1u64<<62,1,0),(1u64<<31,1u64<<32,1)] {
            let mut f=hp_fixture(hp,maximum); assert_eq!(value(6,[0,0x1000,0x2000,0],&mut f),expected);
        }
        let mut f=hp_fixture(25,100); f.q(0x32c0,33); assert!(evaluate(6,[0,0x1000,0x2000,0],&mut f).is_err());
        let mut f=hp_fixture(25,100); f.fields.remove(&(0x4042,1)); assert!(evaluate(6,[0,0x1000,0x2000,0],&mut f).is_err());
    }

    #[test]
    fn hp_maximum_capture_fits_96_fields_and_role_is_u32() {
        let mut f=hp_fixture(25,100); f.q(0x32c0,32);
        // read(width4)는 원문 DWORD처럼 상위bit를 제거해야 한다.
        f.put(0x1890,4,0x1_0000_0002);
        for j in 0..32 {f.put(0x4046+j*80,1,0);f.put(0x4042+j*80,1,if j==31 {2}else{0});}
        assert_eq!(value(6,[0,0x1000,0x2000,0],&mut f),1);
        assert!(f.reads<=96);
    }

    #[test]
    fn record_pointer_first_match_last_match_and_capacity() {
        let mut f=Fixture::default(); f.q(0x1840,0); assert_eq!(value(7,[0x1000,7,0,0],&mut f),0);
        f.q(0x1840,12); for j in 0..12 {f.q(0x1090+j*176,j);}
        assert_eq!(value(7,[0x1000,11,0,0],&mut f),0x1000+11*176);
        assert_eq!(value(7,[0x1000,99,0,0],&mut f),0);
        f.q(0x1090,11); assert_eq!(value(7,[0x1000,11,0,0],&mut f),0x1000);
        f.q(0x1840,13); assert!(evaluate(7,[0x1000,11,0,0],&mut f).is_err());
    }

    #[test]
    fn missing_reads_are_errors_and_never_default_zero() {
        for id in 0..8 { assert!(evaluate(id,[0;4],&mut Fixture::default()).is_err()); }
        assert!(evaluate(crate::engine::MODEL_COUNT,[0;4],&mut Fixture::default()).is_err());
    }
}
