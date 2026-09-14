//! typeid.rs — rlib TypeId → exe TypeId 치환을 **내 DLL 이미지 안에서** 수행한다. (2026-09-14 r14 이관 · 원본 `tfm2_ai_adjust\src\judge\agent_link.rs:278`)
//!
//! 왜: rlib(IR) 과 exe 는 크레이트 해시가 달라 `TypeId::of::<T>()` 상수가 다르다. 내 링크사본이 게임이 만든 객체의
//!   `Any::type_id`(게임 vtable → 게임 상수)와 `is::<T>()` 로 비교하면 항상 false 가 된다 — r14 판 2 실측:
//!   `v57_summon_command_score`(TypeId 4종) 가 game=Some(8) / mine=None(55,625/10.4M). 표 = `typeid_tbl.rs`(자동생성 · `MIG\typeid_map_058.json`).
//! 언제: **sweep 설치 전 1회**. 게임 판마다 재적용 불필요(이미지 상수).
use core::sync::atomic::{AtomicUsize, Ordering};

pub static TYPEID_HITS: AtomicUsize = AtomicUsize::new(0);

#[path = "typeid_tbl.rs"]
mod typeid_tbl;

unsafe fn self_base() -> usize {
    let mut h: crate::HMODULE = 0;
    if crate::GetModuleHandleExW(0x4 | 0x2, self_base as *const () as *const u16, &mut h) == 0 { return 0; }
    h
}

pub unsafe fn patch_typeids_in_self() -> String {
    let base = self_base();
    if base == 0 { return "[typeid] self base 0\n".into(); }
    let e_lfanew = core::ptr::read_unaligned((base + 0x3c) as *const u32) as usize;
    let nt = base + e_lfanew;
    let nsec = core::ptr::read_unaligned((nt + 6) as *const u16) as usize;
    let opt_sz = core::ptr::read_unaligned((nt + 20) as *const u16) as usize;
    let mut out = String::new(); let mut total = 0usize;
    // ★내 매핑 테이블(TYPEID_MAP) 자체도 .rdata 에 from 값을 품고 있어 스캔에 잡힌다 → 그 범위는 건너뛴다.
    let tbl = typeid_tbl::TYPEID_MAP.as_ptr() as usize;
    let tbl_end = tbl + typeid_tbl::TYPEID_MAP.len() * 32;
    for i in 0..nsec {
        let sh = nt + 24 + opt_sz + i * 40;
        let vsz = core::ptr::read_unaligned((sh + 8) as *const u32) as usize;
        let va = core::ptr::read_unaligned((sh + 12) as *const u32) as usize;
        if vsz < 16 { continue; }
        let start = base + va; let end = start + vsz;
        let mut a = start;
        let mut page_ok = false;
        while a + 16 <= end {
            if a & 0xfff == 0 || a == start { page_ok = crate::readable(a & !0xfff, 0x1000 + 16); }   // 페이지 단위 1회
            if !page_ok { a = (a & !0xfff) + 0x1000; continue; }
            if a >= tbl && a < tbl_end { a = tbl_end; continue; }
            let w = core::ptr::read_unaligned(a as *const [u8; 16]);
            for (k, (from, to)) in typeid_tbl::TYPEID_MAP.iter().enumerate() {
                if w == *from {
                    let mut old: u32 = 0;
                    if crate::VirtualProtect(a, 16, 0x04, &mut old) != 0 {
                        core::ptr::write_unaligned(a as *mut [u8; 16], *to);
                        crate::VirtualProtect(a, 16, old, &mut old);
                        total += 1; out.push_str(&format!("[typeid] #{} @+{:#x} 치환\n", k, a - base));
                    }
                }
            }
            a += 1;
        }
    }
    TYPEID_HITS.store(total, Ordering::SeqCst);
    out.push_str(&format!("[typeid] 총 {} 치환 (표 {}종)\n", total, typeid_tbl::TYPEID_MAP.len()));
    out
}
