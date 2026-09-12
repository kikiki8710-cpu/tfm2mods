#![crate_type="lib"]
pub fn f(v: &[i32]) -> bool { let a = v.iter().any(|x| *x > 3); let b = v.iter().all(|y| *y < 9); a || b }
pub fn g(v: &[i32]) -> bool { v.iter().any(|x| { let q = *x * 2; q > 3 }) }
