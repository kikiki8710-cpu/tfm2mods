# RVA 통합 카탈로그 (0.5.2 기준값 → 0.5.3 재탐색 대상)

> 생성 = `rva_catalog.py`. fog_damage_fix 제외(게임측 수정). crm/Spectator_Chat/meta_item_delegate = RVA 0.


## tfm2_ai_adjust (258건)

| 상수 | RVA(0.5.2) | 종류 | 위치 | 용도 |
|---|---|---|---|---|
| `(inline)` | `0x20def90` | inline | src/detour.rs:311 | write_named("gbbody.txt", "=== generic_build 본체(0x20def90) 출력 캡처: (dis |
| `(inline)` | `0x1b934a4` | inline | src/detour.rs:684 | ok += patch_imm_bytes(base + 0x1b934a4, &[0x48,0x83,0xf8], 3, 1, b1(nh |
| `(inline)` | `0x1b934b0` | inline | src/detour.rs:685 | ok += patch_imm_bytes(base + 0x1b934b0, &[0x48,0x83,0xf8], 3, 1, b1(nh |
| `(inline)` | `0x1b934ec` | inline | src/detour.rs:686 | ok += patch_imm_bytes(base + 0x1b934ec, &[0x48,0x83,0x7d,0xd8], 4, 1,  |
| `(inline)` | `0x1b9351c` | inline | src/detour.rs:687 | ok += patch_imm_bytes(base + 0x1b9351c, &[0x48,0x83,0x7d,0xd8], 4, 1,  |
| `(inline)` | `0x1b9302c` | inline | src/detour.rs:688 | ok += patch_imm_bytes(base + 0x1b9302c, &[0x48,0xb8], 2, 8, sq(nd)) as |
| `(inline)` | `0x1b93152` | inline | src/detour.rs:689 | ok += patch_imm_bytes(base + 0x1b93152, &[0x48,0xb8], 2, 8, sq(nd)) as |
| `(inline)` | `0x1b933d8` | inline | src/detour.rs:690 | ok += patch_imm_bytes(base + 0x1b933d8, &[0x49,0xba], 2, 8, sq0(nd)) a |
| `(inline)` | `0x1bdac25` | inline | src/detour.rs:692 | ok += patch_imm_bytes(base + 0x1bdac25, &[0x48,0xb8], 2, 8, sq(pd)) as |
| `(inline)` | `0x1bdac95` | inline | src/detour.rs:693 | ok += patch_imm_bytes(base + 0x1bdac95, &[0x49,0x83,0xc6], 3, 1, b1(lm |
| `(inline)` | `0x2376e86` | inline | src/detour.rs:695 | ok += patch_imm_bytes(base + 0x2376e86, &[0x49,0x81,0xfa], 3, 4, u32c( |
| `(inline)` | `0x23777fe` | inline | src/detour.rs:696 | ok += patch_imm_bytes(base + 0x23777fe, &[0x48,0x83,0xf8], 3, 1, b1(fh |
| `(inline)` | `0x237780a` | inline | src/detour.rs:697 | ok += patch_imm_bytes(base + 0x237780a, &[0x48,0x83,0xf8], 3, 1, b1(fh |
| `(inline)` | `0x2126ae3` | inline | src/detour.rs:719 | let ok = patch_imm_bytes(base + 0x2126ae3, &[0x48,0x81,0xc6], 3, 4, v) |
| `(inline)` | `0x2126ae3` | inline | src/detour.rs:722 | let _ = fs::write(p, format!("vis_window={} applied={}/1 @0x2126ae3(0. |
| `(inline)` | `0x22b2555` | inline | src/detour.rs:778 | ok += patch_imm_bytes(base + 0x22b2555, &[0x48,0xc7,0x44,0x24,0x40], 5 |
| `(inline)` | `0x22b2ca5` | inline | src/detour.rs:779 | ok += patch_imm_bytes(base + 0x22b2ca5, &[0x48,0xc7,0x85,0xb0,0x01,0x0 |
| `(inline)` | `0x22b2bb1` | inline | src/detour.rs:780 | ok += patch_imm_bytes(base + 0x22b2bb1, &[0x41,0xb8], 2, 4, e_jd) as u |
| `(inline)` | `0x22b58ad` | inline | src/detour.rs:781 | ok += patch_imm_bytes(base + 0x22b58ad, &[0x48,0x83,0xf8], 3, 1, e_ph) |
| `(inline)` | `0x2398342` | inline | src/detour.rs:783 | ok += patch_imm_bytes(base + 0x2398342, &[0x49,0x83,0xbe,0xb8,0x00,0x0 |
| `(inline)` | `0x2398ef3` | inline | src/detour.rs:784 | ok += patch_imm_bytes(base + 0x2398ef3, &[0x48,0xc7,0x45,0x18,0x00,0x0 |
| `(inline)` | `0x2398f3c` | inline | src/detour.rs:785 | ok += patch_imm_bytes(base + 0x2398f3c, &[0x4c,0x8b,0xad,0x80,0x00,0x0 |
| `(inline)` | `0x23ad9d7` | inline | src/detour.rs:788 | ok += patch_imm_bytes(base + 0x23ad9d7, &[0x48,0xb8], 2, 8, e_rc) as u |
| `(inline)` | `0x23ba8f3` | inline | src/detour.rs:789 | ok += patch_imm_bytes(base + 0x23ba8f3, &[0x49,0xba], 2, 8, e_rc.wrapp |
| `(inline)` | `0x22b43ae` | inline | src/detour.rs:790 | ok += patch_imm_bytes(base + 0x22b43ae, &[0x41,0xb8], 2, 4, e_rm) as u |
| `(inline)` | `0x22e3cdf` | inline | src/detour.rs:833 | ok += patch_imm_bytes(base + 0x22e3cdf, &[0x48,0x83,0xf8], 3, 1, p_t0) |
| `(inline)` | `0x22e3cf0` | inline | src/detour.rs:834 | ok += patch_imm_bytes(base + 0x22e3cf0, &[0x48,0x83,0xf9], 3, 1, p_h1) |
| `(inline)` | `0x22e3cf6` | inline | src/detour.rs:835 | ok += patch_imm_bytes(base + 0x22e3cf6, &[0x48,0x83,0xf8], 3, 1, p_t1) |
| `(inline)` | `0x22e3d00` | inline | src/detour.rs:836 | ok += patch_imm_bytes(base + 0x22e3d00, &[0x48,0x83,0xf9], 3, 1, p_h2) |
| `(inline)` | `0x22e3d06` | inline | src/detour.rs:837 | ok += patch_imm_bytes(base + 0x22e3d06, &[0x48,0x83,0xf8], 3, 1, p_t2) |
| `(inline)` | `0x22e3d10` | inline | src/detour.rs:838 | ok += patch_imm_bytes(base + 0x22e3d10, &[0x48,0x83,0xf9], 3, 1, p_h3) |
| `(inline)` | `0x22e3d16` | inline | src/detour.rs:839 | ok += patch_imm_bytes(base + 0x22e3d16, &[0x48,0x83,0xf8], 3, 1, p_t3) |
| `(inline)` | `0x22e3d2b` | inline | src/detour.rs:840 | ok += patch_imm_bytes(base + 0x22e3d2b, &[0x48,0xc1,0xf8], 3, 1, p_ds) |
| `(inline)` | `0x22e3d2f` | inline | src/detour.rs:841 | ok += patch_imm_bytes(base + 0x22e3d2f, &[0x48,0x83,0xf8], 3, 1, p_dc) |
| `(inline)` | `0x22e3d33` | inline | src/detour.rs:842 | ok += patch_imm_bytes(base + 0x22e3d33, &[0xbb], 1, 4, p_dc) as u32; |
| `(inline)` | `0x22edb5f` | inline | src/detour.rs:844 | ok += patch_imm_bytes(base + 0x22edb5f, &[0x48,0x83,0xf8], 3, 1, p_t0) |
| `(inline)` | `0x22edb65` | inline | src/detour.rs:845 | ok += patch_imm_bytes(base + 0x22edb65, &[0x49,0x83,0xf8], 3, 1, p_h1) |
| `(inline)` | `0x22edb6b` | inline | src/detour.rs:846 | ok += patch_imm_bytes(base + 0x22edb6b, &[0x48,0x83,0xf8], 3, 1, p_t1) |
| `(inline)` | `0x22edb71` | inline | src/detour.rs:847 | ok += patch_imm_bytes(base + 0x22edb71, &[0x49,0x83,0xf8], 3, 1, p_h2) |
| `(inline)` | `0x22edb7b` | inline | src/detour.rs:848 | ok += patch_imm_bytes(base + 0x22edb7b, &[0x48,0x83,0xf8], 3, 1, p_t2) |
| `(inline)` | `0x22effff` | inline | src/detour.rs:850 | ok += patch_imm_bytes(base + 0x22effff, &[0x48,0x83,0xf8], 3, 1, p_t0) |
| `(inline)` | `0x22f0005` | inline | src/detour.rs:851 | ok += patch_imm_bytes(base + 0x22f0005, &[0x48,0x83,0xf9], 3, 1, p_h1) |
| `(inline)` | `0x22f000b` | inline | src/detour.rs:852 | ok += patch_imm_bytes(base + 0x22f000b, &[0x48,0x83,0xf8], 3, 1, p_t1) |
| `(inline)` | `0x22f0011` | inline | src/detour.rs:853 | ok += patch_imm_bytes(base + 0x22f0011, &[0x48,0x83,0xf9], 3, 1, p_h2) |
| `(inline)` | `0x22f0017` | inline | src/detour.rs:854 | ok += patch_imm_bytes(base + 0x22f0017, &[0x48,0x83,0xf8], 3, 1, p_t2) |
| `(inline)` | `0x22f001d` | inline | src/detour.rs:855 | ok += patch_imm_bytes(base + 0x22f001d, &[0x48,0x83,0xf9], 3, 1, p_h3) |
| `(inline)` | `0x22f0023` | inline | src/detour.rs:856 | ok += patch_imm_bytes(base + 0x22f0023, &[0x48,0x83,0xf8], 3, 1, p_t3) |
| `(inline)` | `0x23a0c21` | inline | src/detour.rs:858 | ok += patch_imm_bytes(base + 0x23a0c21, &[0x48,0x83,0xf8], 3, 1, p_t0) |
| `(inline)` | `0x23a0c27` | inline | src/detour.rs:859 | ok += patch_imm_bytes(base + 0x23a0c27, &[0x49,0x83,0xf8], 3, 1, p_h1) |
| `(inline)` | `0x23a0c2d` | inline | src/detour.rs:860 | ok += patch_imm_bytes(base + 0x23a0c2d, &[0x48,0x83,0xf8], 3, 1, p_t1) |
| `(inline)` | `0x23a0c33` | inline | src/detour.rs:861 | ok += patch_imm_bytes(base + 0x23a0c33, &[0x49,0x83,0xf8], 3, 1, p_h2) |
| `(inline)` | `0x23a0c39` | inline | src/detour.rs:862 | ok += patch_imm_bytes(base + 0x23a0c39, &[0x48,0x83,0xf8], 3, 1, p_t2) |
| `(inline)` | `0x23a0c41` | inline | src/detour.rs:863 | ok += patch_imm_bytes(base + 0x23a0c41, &[0x49,0x83,0xf8], 3, 1, p_h3) |
| `(inline)` | `0x23a0c47` | inline | src/detour.rs:864 | ok += patch_imm_bytes(base + 0x23a0c47, &[0x48,0x83,0xf8], 3, 1, p_t3  |
| `SIMUNCHUNK_RVA` | `0x19b40c3` | const | src/detour.rs:880 | ★0.5.2(was 0.5.1 0x19adc93). version-migrator 확정: 컨테이너(rayon bridge)가  |
| `(inline)` | `0x2380e16` | inline | src/disc19_repro.rs:52 | ok += patch_imm_bytes(base + 0x2380e16, &[0x48,0x83,0xf8], 3, 1, p_sr0 |
| `(inline)` | `0x2380e22` | inline | src/disc19_repro.rs:53 | ok += patch_imm_bytes(base + 0x2380e22, &[0x48,0x83,0xf8], 3, 1, p_sr1 |
| `(inline)` | `0x2380e2e` | inline | src/disc19_repro.rs:54 | ok += patch_imm_bytes(base + 0x2380e2e, &[0x48,0x83,0xf8], 3, 1, p_sr2 |
| `(inline)` | `0x2380e3c` | inline | src/disc19_repro.rs:55 | ok += patch_imm_bytes(base + 0x2380e3c, &[0x48,0x83,0xf8], 3, 1, p_sr3 |
| `(inline)` | `0x2380e1c` | inline | src/disc19_repro.rs:57 | ok += patch_imm_bytes(base + 0x2380e1c, &[0x48,0x83,0xfe], 3, 1, p_sh1 |
| `(inline)` | `0x2380e28` | inline | src/disc19_repro.rs:58 | ok += patch_imm_bytes(base + 0x2380e28, &[0x48,0x83,0xfe], 3, 1, p_sh2 |
| `(inline)` | `0x2380e36` | inline | src/disc19_repro.rs:59 | ok += patch_imm_bytes(base + 0x2380e36, &[0x48,0x83,0xfe], 3, 1, p_sh3 |
| `(inline)` | `0x2380e92` | inline | src/disc19_repro.rs:61 | ok += patch_imm_bytes(base + 0x2380e92, &[0x48,0x83,0xf8], 3, 1, p_ah) |
| `(inline)` | `0x2380ec0` | inline | src/disc19_repro.rs:62 | ok += patch_imm_bytes(base + 0x2380ec0, &[0x48,0x83,0xf8], 3, 1, p_ah) |
| `(inline)` | `0x2380ecd` | inline | src/disc19_repro.rs:64 | ok += patch_imm_bytes(base + 0x2380ecd, &[0x48,0x83,0xfe], 3, 1, p_rhb |
| `(inline)` | `0x1f23a60` | inline | src/disc19_repro.rs:259 | 0x1f23a60 \| 0x1d204c0 => d19_bd_walk(p1, 8, 0x10, 16, e, exe, depth), |
| `(inline)` | `0x1d204c0` | inline | src/disc19_repro.rs:259 | 0x1f23a60 \| 0x1d204c0 => d19_bd_walk(p1, 8, 0x10, 16, e, exe, depth), |
| `(inline)` | `0x1a5ee60` | inline | src/disc19_repro.rs:260 | 0x1a5ee60 => d19_bd_walk(p1, 0x20, 0x28, 24, e, exe, depth), |
| `(inline)` | `0x1d1f630` | inline | src/disc19_repro.rs:261 | 0x1d1f630 => { |
| `(inline)` | `0x1dce1d0` | inline | src/disc19_repro.rs:266 | 0x1dce1d0 => { |
| `(inline)` | `0x1d328e0` | inline | src/disc19_repro.rs:270 | 0x1d328e0 => { |
| `(inline)` | `0x23a4d90` | inline | src/disc19_repro.rs:285 | 0x23a4d90 => { |
| `(inline)` | `0x20a3fd0` | inline | src/disc19_repro.rs:465 | if !code_ptr_ok(base + 0x20a3fd0) { return 0; } |
| `(inline)` | `0x20a3fd0` | inline | src/disc19_repro.rs:466 | let f: extern "C" fn(usize, usize, usize, u64, u64, i64) -> u64 = core |
| `(inline)` | `0x1c974a0` | inline | src/disc19_repro.rs:629 | if slot == exe_base().wrapping_add(0x1c974a0) { |
| `(inline)` | `0x1fce700` | inline | src/disc19_repro.rs:726 | if !code_ptr_ok(base + 0x1fce700) { return false; } |
| `(inline)` | `0x1fce700` | inline | src/disc19_repro.rs:727 | let f: D19Us = core::mem::transmute(base + 0x1fce700); |
| `(inline)` | `0x1fbe950` | inline | src/disc19_repro.rs:736 | if !code_ptr_ok(base + 0x1fbe950) { return false; } |
| `(inline)` | `0x1fbe950` | inline | src/disc19_repro.rs:737 | let f: D19Us = core::mem::transmute(base + 0x1fbe950); |
| `(inline)` | `0x19f2f60` | inline | src/disc19_repro.rs:762 | 0x19f2f60 => 0x30, 0x1a13cb0 => 0x28, 0x19ed260 => 0x20, |
| `(inline)` | `0x1a13cb0` | inline | src/disc19_repro.rs:762 | 0x19f2f60 => 0x30, 0x1a13cb0 => 0x28, 0x19ed260 => 0x20, |
| `(inline)` | `0x19ed260` | inline | src/disc19_repro.rs:762 | 0x19f2f60 => 0x30, 0x1a13cb0 => 0x28, 0x19ed260 => 0x20, |
| `(inline)` | `0x19ed250` | inline | src/disc19_repro.rs:763 | 0x19ed250 => 0x18, 0x1a3a240 => 0x08, 0xb024b0 => 0x00, |
| `(inline)` | `0x1a3a240` | inline | src/disc19_repro.rs:763 | 0x19ed250 => 0x18, 0x1a3a240 => 0x08, 0xb024b0 => 0x00, |
| `(inline)` | `0xb024b0` | inline | src/disc19_repro.rs:763 | 0x19ed250 => 0x18, 0x1a3a240 => 0x08, 0xb024b0 => 0x00, |
| `(inline)` | `0x1e85540` | inline | src/disc19_repro.rs:764 | 0x1e85540 => 0x170, |
| `(inline)` | `0x9a1230` | inline | src/disc19_repro.rs:795 | 0x9a1230 => 1, |
| `(inline)` | `0x1bbe3c0` | inline | src/disc19_repro.rs:796 | 0x1bbe3c0 => rd_u64(data + 0x40).unwrap_or(0), |
| `(inline)` | `0x1a13cb0` | inline | src/disc19_repro.rs:797 | 0x1a13cb0 => rd_u64(data + 0x28).unwrap_or(0), |
| `(inline)` | `0x50fc80` | inline | src/disc19_repro.rs:1402 | match rva { 0x50fc80 => a2(37), 0x19ec2c0 => a2(38), 0x5418a0 => a2(39 |
| `(inline)` | `0x19ec2c0` | inline | src/disc19_repro.rs:1402 | match rva { 0x50fc80 => a2(37), 0x19ec2c0 => a2(38), 0x5418a0 => a2(39 |
| `(inline)` | `0x5418a0` | inline | src/disc19_repro.rs:1402 | match rva { 0x50fc80 => a2(37), 0x19ec2c0 => a2(38), 0x5418a0 => a2(39 |
| `(inline)` | `0x50fc80` | inline | src/disc19_repro.rs:1405 | 0x50fc80 => false, |
| `(inline)` | `0x5418a0` | inline | src/disc19_repro.rs:1406 | 0x5418a0 => true, |
| `(inline)` | `0x19ec2c0` | inline | src/disc19_repro.rs:1407 | 0x19ec2c0 => descvt_any_at(obj, 0x08, 0x10, 0x10, 0x78, depth), |
| `(inline)` | `0x1e66f40` | inline | src/disc19_repro.rs:1408 | 0x1e66f40 => descvt_any_at(obj, 0x50, 0x58, 0x18, 0x78, depth), |
| `(inline)` | `0x1eacc00` | inline | src/disc19_repro.rs:1409 | 0x1eacc00 => { |
| `(inline)` | `0x1e65a80` | inline | src/disc19_repro.rs:1414 | 0x1e65a80 => { |
| `(inline)` | `0x1f23eb0` | inline | src/disc19_repro.rs:1420 | 0x1f23eb0 => descvt_cany(obj, 0x08, 0x10, 0x10, 0x78, depth), |
| `(inline)` | `0x1d1edd0` | inline | src/disc19_repro.rs:1421 | 0x1d1edd0 => descvt_cany(obj, 0x50, 0x58, 0x18, 0x78, depth), |
| `(inline)` | `0x2291570` | inline | src/disc19_repro.rs:1422 | 0x2291570 => descvt_cany(obj, 0x50, 0x58, 0x18, 0x78, depth) \|\| desc |
| `(inline)` | `0x50fc80` | inline | src/disc19_repro.rs:1468 | 0x50fc80  => false, |
| `(inline)` | `0x5418a0` | inline | src/disc19_repro.rs:1469 | 0x5418a0  => true, |
| `(inline)` | `0x1f23dd0` | inline | src/disc19_repro.rs:1470 | 0x1f23dd0 => descvt_cany(obj, 0x08, 0x10, 0x10, 0x50, depth), |
| `(inline)` | `0x1ce1070` | inline | src/disc19_repro.rs:1471 | 0x1ce1070 => rd_u64(obj + 0x18).unwrap_or(0) != 0 && rd_u64(obj + 0x10 |
| `(inline)` | `0x23a4f80` | inline | src/disc19_repro.rs:1472 | 0x23a4f80 => descvt_child(rd_u64(obj + 0x18).unwrap_or(0) as usize, rd |
| `(inline)` | `0x23b5790` | inline | src/disc19_repro.rs:1473 | 0x23b5790 => descvt_child(rd_u64(obj).unwrap_or(0) as usize, rd_u64(ob |
| `(inline)` | `0x50fc80` | inline | src/disc19_repro.rs:1481 | 0x50fc80  => false, |
| `(inline)` | `0x5418a0` | inline | src/disc19_repro.rs:1482 | 0x5418a0  => true, |
| `(inline)` | `0x1f23d70` | inline | src/disc19_repro.rs:1483 | 0x1f23d70 => descvt_cany(obj, 0x08, 0x10, 0x10, 0x58, depth), |
| `(inline)` | `0x1a671e0` | inline | src/disc19_repro.rs:1484 | 0x1a671e0 => descvt_cany(obj, 0x50, 0x58, 0x18, 0x58, depth) |
| `(inline)` | `0x1d1ed70` | inline | src/disc19_repro.rs:1486 | 0x1d1ed70 => descvt_cany(obj, 0x50, 0x58, 0x18, 0x58, depth), |
| `(inline)` | `0x1faac80` | inline | src/disc19_repro.rs:1487 | 0x1faac80 => rd_u64(obj + 0x18).unwrap_or(0) != 0, |
| `(inline)` | `0x23a4f60` | inline | src/disc19_repro.rs:1488 | 0x23a4f60 => descvt_child(rd_u64(obj + 0x18).unwrap_or(0) as usize, rd |
| `(inline)` | `0x23b5770` | inline | src/disc19_repro.rs:1489 | 0x23b5770 => descvt_child(rd_u64(obj).unwrap_or(0) as usize, rd_u64(ob |
| `(inline)` | `0x9c8850` | inline | src/disc19_repro.rs:1498 | 0x9c8850  => false, |
| `(inline)` | `0x5418a0` | inline | src/disc19_repro.rs:1499 | 0x5418a0  => true, |
| `(inline)` | `0x1f23f90` | inline | src/disc19_repro.rs:1500 | 0x1f23f90 => descvt_cany(obj, 0x08, 0x10, 0x10, 0x48, depth), |
| `(inline)` | `0x1ce10f0` | inline | src/disc19_repro.rs:1501 | 0x1ce10f0 \| 0x1ce1090 => rd_u64(obj + 0x18).unwrap_or(0) != 0, |
| `(inline)` | `0x1ce1090` | inline | src/disc19_repro.rs:1501 | 0x1ce10f0 \| 0x1ce1090 => rd_u64(obj + 0x18).unwrap_or(0) != 0, |
| `(inline)` | `0x1fabac0` | inline | src/disc19_repro.rs:1502 | 0x1fabac0 => true, |
| `(inline)` | `0x1ff1970` | inline | src/disc19_repro.rs:1503 | 0x1ff1970 => true, |
| `(inline)` | `0x23a5080` | inline | src/disc19_repro.rs:1504 | 0x23a5080 => descvt_child(rd_u64(obj + 0x18).unwrap_or(0) as usize, rd |
| `(inline)` | `0x23b5890` | inline | src/disc19_repro.rs:1505 | 0x23b5890 => descvt_child(rd_u64(obj).unwrap_or(0) as usize, rd_u64(ob |
| `(inline)` | `0x50fc80` | inline | src/disc19_repro.rs:1518 | 0x50fc80 => a2(45), |
| `(inline)` | `0x1f77e30` | inline | src/disc19_repro.rs:1519 | 0x1f77e30 \| 0x23bd430 \| 0x1a671e0 \| 0x1d1ed70 => a2(46), |
| `(inline)` | `0x23bd430` | inline | src/disc19_repro.rs:1519 | 0x1f77e30 \| 0x23bd430 \| 0x1a671e0 \| 0x1d1ed70 => a2(46), |
| `(inline)` | `0x1a671e0` | inline | src/disc19_repro.rs:1519 | 0x1f77e30 \| 0x23bd430 \| 0x1a671e0 \| 0x1d1ed70 => a2(46), |
| `(inline)` | `0x1d1ed70` | inline | src/disc19_repro.rs:1519 | 0x1f77e30 \| 0x23bd430 \| 0x1a671e0 \| 0x1d1ed70 => a2(46), |
| `(inline)` | `0x1faac80` | inline | src/disc19_repro.rs:1520 | 0x1faac80 \| 0x23bd370 \| 0x23bd3d0 \| 0x5418a0 => a2(47), |
| `(inline)` | `0x23bd370` | inline | src/disc19_repro.rs:1520 | 0x1faac80 \| 0x23bd370 \| 0x23bd3d0 \| 0x5418a0 => a2(47), |
| `(inline)` | `0x23bd3d0` | inline | src/disc19_repro.rs:1520 | 0x1faac80 \| 0x23bd370 \| 0x23bd3d0 \| 0x5418a0 => a2(47), |
| `(inline)` | `0x5418a0` | inline | src/disc19_repro.rs:1520 | 0x1faac80 \| 0x23bd370 \| 0x23bd3d0 \| 0x5418a0 => a2(47), |
| `(inline)` | `0x50fc80` | inline | src/disc19_repro.rs:1525 | 0x50fc80  => false, |
| `(inline)` | `0x5418a0` | inline | src/disc19_repro.rs:1526 | 0x5418a0  => true, |
| `(inline)` | `0x1faac80` | inline | src/disc19_repro.rs:1527 | 0x1faac80 => rd_u64(obj + 0x18).unwrap_or(0) != 0, |
| `(inline)` | `0x1f77e30` | inline | src/disc19_repro.rs:1528 | 0x1f77e30 => { |
| `(inline)` | `0x23bd430` | inline | src/disc19_repro.rs:1539 | 0x23bd430 => descvt_cany(obj, 0x08, 0x10, 0x10, 0x50, depth) |
| `(inline)` | `0x1a671e0` | inline | src/disc19_repro.rs:1541 | 0x1a671e0 => descvt_cany(obj, 0x50, 0x58, 0x18, 0x58, depth) |
| `(inline)` | `0x1d1ed70` | inline | src/disc19_repro.rs:1543 | 0x1d1ed70 => descvt_cany(obj, 0x50, 0x58, 0x18, 0x58, depth), |
| `(inline)` | `0x23bd370` | inline | src/disc19_repro.rs:1544 | 0x23bd370 => descvt_trio(rd_u64(obj + 0x18).unwrap_or(0) as usize, rd_ |
| `(inline)` | `0x23bd3d0` | inline | src/disc19_repro.rs:1545 | 0x23bd3d0 => descvt_trio(rd_u64(obj).unwrap_or(0) as usize, rd_u64(obj |
| `(inline)` | `0x1f23680` | inline | src/disc19_repro.rs:1546 | 0x1f23680 => descvt_cany(obj, 0x08, 0x10, 0x10, 0xc8, depth), |
| `(inline)` | `0x50fc80` | inline | src/disc19_repro.rs:1566 | 0x50fc80 => a2(32), 0x1f236f0 => a2(33), 0x20958d0 => a2(34), _ => a2( |
| `(inline)` | `0x1f236f0` | inline | src/disc19_repro.rs:1566 | 0x50fc80 => a2(32), 0x1f236f0 => a2(33), 0x20958d0 => a2(34), _ => a2( |
| `(inline)` | `0x20958d0` | inline | src/disc19_repro.rs:1566 | 0x50fc80 => a2(32), 0x1f236f0 => a2(33), 0x20958d0 => a2(34), _ => a2( |
| `(inline)` | `0x50fc80` | inline | src/disc19_repro.rs:1570 | 0x50fc80 => 0, |
| `(inline)` | `0x1f236f0` | inline | src/disc19_repro.rs:1571 | 0x1f236f0 => { |
| `(inline)` | `0x20958d0` | inline | src/disc19_repro.rs:1585 | 0x20958d0 => { |
| `(inline)` | `0x1f23d30` | inline | src/disc19_repro.rs:1599 | 0x1f23d30 \| 0x23a49f0 => { |
| `(inline)` | `0x23a49f0` | inline | src/disc19_repro.rs:1599 | 0x1f23d30 \| 0x23a49f0 => { |
| `(inline)` | `0x50fc80` | inline | src/disc19_repro.rs:2220 | descvt_90: 0x50fc80(=0)={} 0x1f236f0(max)={} 0x20958d0(lvl3)={} 미등재={} |
| `(inline)` | `0x1f236f0` | inline | src/disc19_repro.rs:2220 | descvt_90: 0x50fc80(=0)={} 0x1f236f0(max)={} 0x20958d0(lvl3)={} 미등재={} |
| `(inline)` | `0x20958d0` | inline | src/disc19_repro.rs:2220 | descvt_90: 0x50fc80(=0)={} 0x1f236f0(max)={} 0x20958d0(lvl3)={} 미등재={} |
| `(inline)` | `0x50fc80` | inline | src/disc19_repro.rs:2221 | descvt_78(pred): 0x50fc80(false)={} 0x19ec2c0(comp)={} 0x5418a0(true)= |
| `(inline)` | `0x19ec2c0` | inline | src/disc19_repro.rs:2221 | descvt_78(pred): 0x50fc80(false)={} 0x19ec2c0(comp)={} 0x5418a0(true)= |
| `(inline)` | `0x5418a0` | inline | src/disc19_repro.rs:2221 | descvt_78(pred): 0x50fc80(false)={} 0x19ec2c0(comp)={} 0x5418a0(true)= |
| `(inline)` | `0x50fc80` | inline | src/disc19_repro.rs:2223 | descvt_c8(aim): 0x50fc80(false)={} composite={} 기타등재={} ★미등재={}  (미등재> |
| `(inline)` | `0x3886538` | inline | src/disc19_repro.rs:2268 | let is_aoe = rd_u64(self_u + 0x580 + off).unwrap_or(0) as usize == exe |
| `(inline)` | `0x237d910` | inline | src/disc19_repro.rs:2386 | if !code_ptr_ok(base + 0x237d910) { return false; } |
| `(inline)` | `0x237d910` | inline | src/disc19_repro.rs:2387 | let f: F237d = core::mem::transmute(base + 0x237d910); |
| `(inline)` | `0x1a36e3` | inline | src/disc19_repro.rs:2423 | let hi = (((jj * 0x72) * 0x1a36e3) >> 0x22) + 6; |
| `(inline)` | `0x9502F9` | inline | src/disc19_repro.rs:2728 | if (d2 >> 10) >= 0x9502F9 && !ok68b { continue; } |
| `(inline)` | `0x236b6b0` | inline | src/disc19_repro.rs:2814 | if !code_ptr_ok(base + 0x236b6b0) { return (0, 0); } |
| `(inline)` | `0x236b6b0` | inline | src/disc19_repro.rs:2817 | let f: Fn2090 = core::mem::transmute(base + 0x236b6b0); |
| `(inline)` | `0x18c3090` | inline | src/genbuild_repro.rs:59 | if g == exe + 0x18c3090 { |
| `(inline)` | `0x1bc6f10` | inline | src/genbuild_repro.rs:73 | } else if g == exe + 0x1bc6f10 { |
| `(inline)` | `0x1db1eb0` | inline | src/genbuild_repro.rs:157 | if rfn == exe + 0x1db1eb0 { |
| `(inline)` | `0x1db2c30` | inline | src/genbuild_repro.rs:161 | if rfn == exe + 0x1db2c30 { return walk_list(0x50, 0x58, 0x18, depth); |
| `(inline)` | `0x1a45ba0` | inline | src/genbuild_repro.rs:162 | if rfn == exe + 0x1a45ba0 { return walk_list(0x8, 0x10, 0x10, depth);  |
| `(inline)` | `0x35f5f28` | inline | src/genbuild_repro.rs:587 | let threat = \|i: u8\| rd_i64(exe + 0x35f5f28 + (i as usize).min(3) *  |
| `RVA_GB_ATKCTX_CB30` | `0x35d8018` | const | src/genbuild_repro.rs:698 | &PTR_1435d8018 (0x203cb30 resolver r9) |
| `RVA_GB_ATKCTX_C0690` | `0x35efd48` | const | src/genbuild_repro.rs:699 | &PTR_1435efd48 (0x20c0690 resolver r9) |
| `DESCS` | `0x1c7df47,0x1c7d5f9,0x1caedd3` | array[3] | src/knobs.rs:25 | 항목 설명(호버 도움말). DESCS[i] = KNOBS[i] 설명. |
| `(inline)` | `0x2000000` | inline | src/mem_safety.rs:130 | if modb != 0 && rip.wrapping_sub(modb) < 0x2000000 { cb_str(&mut buf,  |
| `(inline)` | `0x2000000` | inline | src/mem_safety.rs:142 | else if modb != 0 && dm < 0x2000000 { cb_str(&mut buf, &mut pos, b"  M |
| `TEXT_END_RVA` | `0x2c087ff` | const | src/mem_safety.rs:310 | ★0.5.2(was 0.5.1 0x2c0ed7f). PE .text va=0x1000 vsz_end=0x2c08800 실측(v |
| `(inline)` | `0x2000000` | inline | src/mem_safety.rs:520 | } else if modb != 0 && dm < 0x2000000 { |
| `(inline)` | `0x2000000` | inline | src/mem_safety.rs:721 | if de < 0x8000000 \|\| (modb != 0 && dm < 0x2000000) { t.rets[t.nret]  |
| `(inline)` | `0x2000000` | inline | src/mem_safety.rs:733 | else if modb != 0 && dm < 0x2000000 { format!("MOD+{:#x}", dm) } |
| `(inline)` | `0x2000000` | inline | src/mem_safety.rs:738 | let interesting = t.busy_ms > 0 \|\| (modb != 0 && t.rip.wrapping_sub( |
| `RVA_RETREAT` | `0x1b94670` | const | src/rva_052.rs:15 | ★0.5.2(was 0.5.1 0x1e08cd0). version-migrator 확정: 니모닉멀티셋 cos=0.9999(2n |
| `RVA_TG_CALL` | `0x1feca43` | const | src/rva_052.rs:17 | ⚠0.5.0 실패(stale 0.4.14값 유지). 0.5.0 아키텍처변경: threatgate가 gb 내부 임베드=독립콜사이 |
| `RVA_THREATGATE_FN` | `0x20a8680` | const | src/rva_052.rs:19 | ⚠0.5.0 실패(stale 0.4.14값 유지). gb 내부 mid-block 임베드=독립함수/콜사이트 없음(0.4.14 핫 |
| `RVA_F2_BUILD_CALL` | `0x22dd4fe` | const | src/rva_052.rs:23 | ⚠0.5.0_3값 유지=inert(스왑 금지). ★0.5.1 확정 콜사이트=0x1e27234(gb entry 0x1e1ebb0 |
| `RVA_GENERIC_BUILD` | `0x22b2280` | const | src/rva_052.rs:25 | ★**0.5.2 확정**(ghidra-re 07-22, ~~보류 0x1e1ebb0~~): push8 프롤로그 12B·rip-r |
| `RVA_FC59A0` | `0x1bdb3e0` | const | src/rva_052.rs:29 | ★0.5.2(was 0.5.1 0x1e2c980). version-migrator: cos=0.9995(2nd 0.9940)· |
| `RVA_TABLE_A` | `0x3828818` | const | src/rva_052.rs:32 | ★0.5.2(was 0.5.1 0x384ea20). version-migrator 확정: 참조함수(pregate)가 L1-UN |
| `RVA_GB_REGIOND_HOOK` | `0x22dafea` | const | src/rva_052.rs:36 | ★07-10 정정(구 0x22daff8=명령중간절단+rel8 JBE2개 크래시): 안전슬롯 0x22dafea(INC RSI/M |
| `RVA_GB_FUNNEL` | `0x22dbc4e` | const | src/rva_052.rs:40 | ★0.5.0_3값 유지=inert(was 0x20e4a1a). region D 공통출구(result-copy→arena cle |
| `RVA_CONDGATE` | `0x21338d0` | const | src/rva_052.rs:43 | ★0.5.2(was 0.5.1 0x1cbb8b0). version-migrator **최고신뢰**: L1-UNIQUE(스켈레톤 |
| `RVA_MOVEPRI` | `0x2134240` | const | src/rva_052.rs:45 | ★0.5.2(was 0.5.1 0x1cbc220). version-migrator 확정: cos=0.9995(2nd 0.987 |
| `RVA_COMMIT_CALL` | `0x1e3dfd2` | const | src/rva_052.rs:51 | 0.5.0(was 0.4.14 0x1c9bdca). handler.rs driver급 0x1e3d5a0이 commit_fn(0 |
| `RVA_COMMIT_FN` | `0x235ffa0` | const | src/rva_052.rs:53 | ⏸**0.5.2 보류**: 함수가 너무 작아(15 instr) 스켈레톤/멀티셋 매칭 후보 0 = 재핀 불가. target-gu |
| `RVA_ENGAGE_GATE` | `0x1c9b33d` | const | src/rva_052.rs:57 | ⚠0.5.0 실패(stale 0.4.14값 유지). 0.5.0에 ADD EAX,0x64(83 c0 64) 소멸=교전+100보너 |
| `RVA_DISC18_HANDLER` | `0x2376320` | const | src/rva_052.rs:64 | ★**0.5.2 확정**(ghidra-re 07-22, ~~보류 0x1c7ca20~~): cos 갭 부족 보류가 **imm 유 |
| `RVA_DISC19_HANDLER` | `0x2380820` | const | src/rva_052.rs:66 | ★0.5.2(was 0.5.1 0x1e0ddb0). version-migrator 확정: cos=0.9999(2nd 0.997 |
| `RVA_ITEMNET_SCORER` | `0x1b9cce0` | const | src/rva_052.rs:76 | ★**0.5.2 확정**(07-22 3버전 바이트 대조, ~~0.5.0_3 0x1b78420~~ / 0.5.1=0x1bc82e |
| `RVA_C8C_DMG_SHEET` | `0x381e1e0` | const | src/rva_052.rs:86 | ★**0.5.2 확정**(07-22 3버전 대조, ~~0.5.1 0x3830c58~~ / 0.5.0_3 0x380d138). |
| `RVA_DISC7_DMG_SHEET` | `0x38d1918` | const | src/rva_052.rs:97 | ★**0.5.2 확정**(ghidra-re 07-22, ~~보류 0x3846328·강후보 0x381e1e0은 오답~~): 0. |
| `(inline)` | `0x19ed660` | inline | src/serpen.rs:626 | 0x19ed660 => rd_u64(data + 0x38).unwrap_or(0), |
| `(inline)` | `0x19f2f60` | inline | src/serpen.rs:627 | 0x19f2f60 => rd_u64(data + 0x30).unwrap_or(0), |
| `(inline)` | `0x19ed250` | inline | src/serpen.rs:628 | 0x19ed250 => rd_u64(data + 0x18).unwrap_or(0), |
| `(inline)` | `0x1a3a240` | inline | src/serpen.rs:629 | 0x1a3a240 => rd_u64(data + 8).unwrap_or(0), |
| `(inline)` | `0xb024b0` | inline | src/serpen.rs:630 | 0xb024b0 => rd_u64(data).unwrap_or(0), |
| `(inline)` | `0x50fc80` | inline | src/serpen.rs:631 | 0x50fc80 => 0, |
| `(inline)` | `0x9a1230` | inline | src/serpen.rs:632 | 0x9a1230 => 1, |
| `(inline)` | `0x1a13cb0` | inline | src/serpen.rs:633 | 0x1a13cb0 => rd_u64(data + 0x28).unwrap_or(0), |
| `(inline)` | `0x5418a0` | inline | src/serpen.rs:634 | 0x5418a0 => 1, |
| `ROLE_THR` | `0x1d3602b,0x1d36043,0x1d36058,0x1d3605d` | array[4] | src/tfm2_ai_adjust.rs:967 | (imm32 RVA, 원본) 0.4.13_5(was 0x1fd0546/55e/72/78). 인코딩 cmp-imm32→mov-i |
| `(inline)` | `0x1d3602b` | inline | src/tfm2_ai_adjust.rs:967 | const ROLE_THR: [(usize, u8); 4] = [(0x1d3602b, 100), (0x1d36043, 70), |
| `(inline)` | `0x1d36043` | inline | src/tfm2_ai_adjust.rs:967 | const ROLE_THR: [(usize, u8); 4] = [(0x1d3602b, 100), (0x1d36043, 70), |
| `(inline)` | `0x1d36058` | inline | src/tfm2_ai_adjust.rs:967 | const ROLE_THR: [(usize, u8); 4] = [(0x1d3602b, 100), (0x1d36043, 70), |
| `(inline)` | `0x1d3605d` | inline | src/tfm2_ai_adjust.rs:967 | const ROLE_THR: [(usize, u8); 4] = [(0x1d3602b, 100), (0x1d36043, 70), |
| `OK_DESC_052` | `0x381e1e0,0x38d1918` | array[2] | src/tfm2_ai_adjust.rs:1618 | C8C(확정 07-22) / DISC7(확정 07-22) |
| `(inline)` | `0x381e1e0` | inline | src/tfm2_ai_adjust.rs:1618 | const OK_DESC_052: [usize; 2] = [0x381e1e0, 0x38d1918]; |
| `(inline)` | `0x38d1918` | inline | src/tfm2_ai_adjust.rs:1618 | const OK_DESC_052: [usize; 2] = [0x381e1e0, 0x38d1918]; |
| `LANE_GATE_RVA` | `0x20d9bf9` | const | src/tfm2_ai_adjust.rs:2524 |  |
| `T3_GATE_A_RVA` | `0x1e9d318` | const | src/tfm2_ai_adjust.rs:2534 | jae 0x1e9d379 (원본 73 5f) |
| `T3_GATE_B_RVA` | `0x1e9d59b` | const | src/tfm2_ai_adjust.rs:2535 | jae 0x1e9d5fc (원본 73 5f) |
| `CALL_PUSH_A_RVA` | `0x2070ce9` | const | src/tfm2_ai_adjust.rs:2545 | mov byte[rax+rcx*8],0xb (push A) → 합류 0x2070d01 |
| `CALL_PUSH_B_RVA` | `0x2071752` | const | src/tfm2_ai_adjust.rs:2546 | (push B) → 합류 0x207176c |
| `CALL_JOIN_A_RVA` | `0x2070d01` | const | src/tfm2_ai_adjust.rs:2547 |  |
| `CALL_JOIN_B_RVA` | `0x207176c` | const | src/tfm2_ai_adjust.rs:2548 |  |
| `D19_SLOT2_EMPTY_RVA` | `0x38d1af0` | const | src/tfm2_ai_adjust.rs:2728 | ★**0.5.2 확정**(ghidra-re 07-22, ~~0.5.1 0x3846d50~~). 사용처=disc19_repro( |
| `D19_STATIC_TEMPLATE_RVA` | `0x38d1af0` | const | src/tfm2_ai_adjust.rs:2734 | ★**0.5.2 확정**(ghidra-re 07-22, ~~0.5.1 0x3846d50~~). SLOT2_EMPTY와 동일 객 |
| `D19_STATIC2_TEMPLATE_RVA` | `0x38d17b8` | const | src/tfm2_ai_adjust.rs:2745 | ⏸**0.5.2 미확정=0.5.1값 유지**(ghidra-re 07-22: 2차 emitter 재식별 실패·0 desc라 값  |
| `D19_TV7_RVA` | `0x3863a28` | const | src/tfm2_ai_adjust.rs:2751 | ★0.5.2(was 0.5.1 0x38b7d50). version-migrator 확정: 참조사이트 마스크시그 UNANIMOU |
| `(inline)` | `0x83126f` | inline | src/tfm2_ai_adjust.rs:3031 | let v2 = (((v1 as u32 as u64).wrapping_mul(cv as u32 as u64) as u128). |
| `(inline)` | `0xffffff` | inline | src/tfm2_ai_adjust.rs:4450 | else { rost.push_str(&format!(" {:x}=t{}f{}({},{}h{}e{:x})", off, rd_i |
| `(inline)` | `0x383cd68` | inline | src/tfm2_ai_adjust.rs:5494 | 0x383cd68 \| 0x38c5d78 => 0, |
| `(inline)` | `0x38c5d78` | inline | src/tfm2_ai_adjust.rs:5494 | 0x383cd68 \| 0x38c5d78 => 0, |
| `(inline)` | `0x383d080` | inline | src/tfm2_ai_adjust.rs:5495 | 0x383d080 \| 0x38c5aa0 => 1, |
| `(inline)` | `0x38c5aa0` | inline | src/tfm2_ai_adjust.rs:5495 | 0x383d080 \| 0x38c5aa0 => 1, |
| `(inline)` | `0x383d358` | inline | src/tfm2_ai_adjust.rs:5496 | 0x383d358 \| 0x38c57c8 => 2, |
| `(inline)` | `0x38c57c8` | inline | src/tfm2_ai_adjust.rs:5496 | 0x383d358 \| 0x38c57c8 => 2, |
| `(inline)` | `0x381e1e0` | inline | src/tfm2_ai_adjust.rs:5702 | let r9 = exe + 0x381e1e0; |
| `(inline)` | `0x35ef020` | inline | src/tfm2_ai_adjust.rs:5894 | let tab_a = exe + 0x35ef020; let tab_b = exe + 0x35eeff0; |
| `(inline)` | `0x35eeff0` | inline | src/tfm2_ai_adjust.rs:5894 | let tab_a = exe + 0x35ef020; let tab_b = exe + 0x35eeff0; |
| `(inline)` | `0x381e1e0` | inline | src/tfm2_ai_adjust.rs:5941 | let atkvt = exe + 0x381e1e0; |
| `(inline)` | `0x35e5730` | inline | src/tfm2_ai_adjust.rs:5967 | let dpi = if rd_i64(target + 0x5b0).unwrap_or(0) >= 3 { target + 0x4e8 |
| `(inline)` | `0x1234567` | inline | src/tfm2_ai_adjust.rs:7415 | let v = if ss != 0 { ss } else { n.wrapping_mul(0x9E3779B97F4A7C15).wr |
| `(inline)` | `0x1234567` | inline | src/tfm2_ai_adjust.rs:7452 | rd[f] = ((bh.wrapping_add(0x1234567) >> 23) % (STRAT_VC[f] as u64)) as |
| `(inline)` | `0x1fcfda0` | inline | src/tfm2_ai_adjust.rs:7575 | Ok(())=>append_log("[hook] retreat_engage replace(0x1fcfda0,12B) OK\n" |
| `(inline)` | `0x1b6ec93` | inline | src/tfm2_ai_adjust.rs:7580 | Ok(())=>append_log("[hook] commit(commit_fn @0x1b6ec93) OK\n"), |
| `(inline)` | `0x1b6e806` | inline | src/tfm2_ai_adjust.rs:7585 | Ok(())=>append_log("[hook] move-post(generic_build @0x1b6e806, 8arg) O |
| `(inline)` | `0x1feca43` | inline | src/tfm2_ai_adjust.rs:7590 | Ok(())=>append_log("[hook] threatgate(@0x1feca43→FUN_1420a8680) OK\n") |
| `(inline)` | `0x2080e20` | inline | src/tfm2_ai_adjust.rs:7597 | Ok(())=>append_log("[hook] fc59a0 recall score(@0x2080e20, 12B, replac |
| `(inline)` | `0x20def90` | inline | src/tfm2_ai_adjust.rs:7603 | Ok(())=>append_log("[hook] generic_build body 출력캡처(@0x20def90, 12B) OK |
| `(inline)` | `0x22dafea` | inline | src/tfm2_ai_adjust.rs:7610 | Ok(())=>{ GBRD_INSTALL_OK.store(1, Ordering::Relaxed); if let Some(p)= |
| `(inline)` | `0x1c383f0` | inline | src/tfm2_ai_adjust.rs:7617 | Ok(())=>append_log("[hook] facet#1 condgate(@0x1c383f0, 15B, replace-r |
| `(inline)` | `0x22df630` | inline | src/tfm2_ai_adjust.rs:7623 | Ok(())=>append_log("[hook] facet#4 movepriority(@0x22df630, 13B, repla |
| `(inline)` | `0x1b78420` | inline | src/tfm2_ai_adjust.rs:7630 | Ok(())=>{ append_log("[hook] itemnet NULL-모델 가드(@0x1b78420+12, 15B) OK |
| `(inline)` | `0x1c7ca20` | inline | src/tfm2_ai_adjust.rs:7638 | Ok(orig)=>{ ORIG_DISC18.store(orig, Ordering::Relaxed); append_log("[h |
| `(inline)` | `0x2380820` | inline | src/tfm2_ai_adjust.rs:7642 | Ok(orig)=>{ ORIG_DISC19.store(orig, Ordering::Relaxed); append_log("[h |
| `LOADER_RVA` | `0x5ac950` | const | src/ui_inject_embed.rs:24 | ★0.5.2(구 0.4.14 stale 0x540ad0). 문자열 xref 확정·프롤로그 push8 24B 0.5.1 copy |
| `PARSER_RVA` | `0x24b5a00` | const | src/ui_inject_embed.rs:25 | ★0.5.2(was 0.5.1 0x24b4590, exe2exe UNIQUE +0x1470). 프롤로그 20B 동일 |
| `ALLOC_RVA` | `0x25c4d30` | const | src/ui_inject_embed.rs:26 | ★0.5.2(was 0.5.1 0x25c5a40, exe2exe UNIQUE −0xd10). 프롤로그 20B 동일 |

## tfm2_item_tactics (31건)

| 상수 | RVA(0.5.2) | 종류 | 위치 | 용도 |
|---|---|---|---|---|
| `FN_DD_SETOPT_RVA` | `0x242f250` | const | src/lib.rs:32 | 0.5.2(구0.5.1=0x2450f40, exe2exe 스켈레톤 UNIQUE·프롤로그 16B 완전동일) |
| `SETTER_NOP_RVA` | `0xda42ee` | const | src/lib.rs:1179 | ⚠0.5.2 STALE(미마이그, SETTER_NOP_ENABLED=false라 무영향) // ⚠0.5.0_3 미마이그(STA |
| `RVA_REALLOC` | `0x25c4dd0` | const | src/lib.rs:1772 | 0.5.2(구0.5.1=0x25c5ae0, exe2exe UNIQUE·프롤로그 동일) __rust_realloc 실함수. (r |
| `CL_LAUNCHER_RVA` | `0x1d96870` | const | src/lib.rs:1813 | 0.5.2(구0.5.1=0x20588a0). exe2exe 니모닉 0.9860 + 콜사이트 9/9 동형 + 내부 seedcto |
| `(inline)` | `0xd40a63` | inline | src/lib.rs:1848 | let is_comptest = rva == 0xd40a63; |
| `(inline)` | `0x759c36` | inline | src/lib.rs:1849 | if (rva == 0x759c36 \|\| rva == 0x75e5cf \|\| is_comptest) && seed !=  |
| `(inline)` | `0x75e5cf` | inline | src/lib.rs:1849 | if (rva == 0x759c36 \|\| rva == 0x75e5cf \|\| is_comptest) && seed !=  |
| `SEEDCTOR_RVA` | `0x22c1da0` | const | src/lib.rs:1928 | 0.5.2(구0.5.1=0x21d03e0, exe2exe UNIQUE·프롤로그 17B 완전동일·launcher 내부 2콜 동형 |
| `SPAWN_RVA` | `0x1d9e0e0` | const | src/lib.rs:1976 | 0.5.2(구0.5.1=0x2060280) |
| `SIM_RVA` | `0x223d1b0` | const | src/lib.rs:2102 | ⚠0.5.2 STALE(exe2exe NO MATCH=로직변경·SIM_PROBE_ENABLED=false라 무영향) // 0. |
| `VIEW_RVA` | `0x20ae1ac` | const | src/lib.rs:2143 | ⚠0.5.2 STALE(미마이그·VIEW_HOOK_ENABLED=false라 무영향) // 0.5.0_3(구0.5.0_2=0x |
| `(inline)` | `0x722ca0` | inline | src/lib.rs:2349 | s.push_str("   (렌더필터=[0x722ca0,0x740000) — 여기 안 드는 콜러가 조합테스트 경로 후보) |
| `(inline)` | `0x740000` | inline | src/lib.rs:2349 | s.push_str("   (렌더필터=[0x722ca0,0x740000) — 여기 안 드는 콜러가 조합테스트 경로 후보) |
| `(inline)` | `0x2060280` | inline | src/lib.rs:2377 | s.push_str(&format!("  ★★v14 스폰커밋훅(0x2060280): 발화={} 렌더판정={} 내팀={} bui |
| `RVA_BUY_ITEM` | `0x211e070` | const | src/lib.rs:2658 | 0.5.2(구0.5.1=0x1f01090, exe2exe 스켈레톤 UNIQUE·프롤로그 24B 완전동일=본체 무변경, delt |
| `ITEMNET_FORWARD_RVA` | `0x1b9cce0` | const | src/lib.rs:2706 | 0.5.2(구0.5.1=0x1bc82e0, exe2exe UNIQUE·프롤로그 동일). ↓이하 0.5.1 이력: (구0.5.0 |
| `(inline)` | `0x2341440` | inline | src/lib.rs:3881 | let sig = base + 0x2341440; |
| `(inline)` | `0x2341447` | inline | src/lib.rs:3882 | let imm = base + 0x2341447; |
| `(inline)` | `0x211e428` | inline | src/lib.rs:3904 | let sig = base + 0x211e428; |
| `(inline)` | `0x211e42e` | inline | src/lib.rs:3905 | let jbe = base + 0x211e42e; |
| `CAND_GATE_RVA` | `0x1a3b280` | const | src/lib.rs:3953 | ⚠0.5.2 STALE(exe2exe NO MATCH·CAND_GATE_ON=false라 무영향) // 0.5.0_3(구0.5 |
| `RVA_SLOT_HELPER` | `0xc5cd80` | const | src/lib.rs:3975 | 0.5.2(구0.5.1=0xd81b30, exe2exe UNIQUE·선두 24B 완전동일 "blue_pla" movabs 포함 |
| `(inline)` | `0x4e46c0` | inline | src/lib.rs:3986 | (0x4e46c0, [0x49,0x83,0xfe,0x30]), |
| `(inline)` | `0x4e4a30` | inline | src/lib.rs:3987 | (0x4e4a30, [0x49,0x83,0xff,0x30]), |
| `(inline)` | `0x4e5110` | inline | src/lib.rs:3988 | (0x4e5110, [0x49,0x83,0xfe,0x30]), |
| `(inline)` | `0x4e5480` | inline | src/lib.rs:3989 | (0x4e5480, [0x49,0x83,0xfe,0x30]), |
| `LOADER_RVA` | `0x5ac950` | const | src/ui_inject.rs:20 | 0.5.2(구0.5.1=0x40f3d0). 프롤로그 24B 완전동일(push8+sub 0x98). |
| `STRAT_LOADER_RVA` | `0x5ac950` | const | src/ui_inject.rs:21 | 0.5.2: LOADER와 동일 copy로 병합(구0.5.1=0xeb17d0 별도 copy). |
| `PARSER_RVA` | `0x24b5a00` | const | src/ui_inject.rs:22 | 0.5.2(구0.5.1=0x24b4590, exe2exe UNIQUE, +0x1470) |
| `ALLOC_RVA` | `0x25c4d30` | const | src/ui_inject.rs:23 | 0.5.2(구0.5.1=0x25c5a40, exe2exe UNIQUE, −0xd10) |
| `DEALLOC_RVA` | `0x25c4d90` | const | src/ui_inject.rs:24 | 0.5.2(구0.5.1=0x25c5aa0, exe2exe UNIQUE, −0xd10). 현재 미사용. |

## tfm2_banpick_illust (30건)

| 상수 | RVA(0.5.2) | 종류 | 위치 | 용도 |
|---|---|---|---|---|
| `RVA_FX_SET` | `0x11e2370` | const | src/showcase.rs:19 | 훅 A: 연출 상태 세팅 (진영 스태시) |
| `RVA_CARD_DRAW` | `0x11f9030` | const | src/showcase.rs:20 | 훅 B: 카드 드로우 헬퍼 |
| `RVA_ILLUST_GET` | `0xfdabe0` | const | src/showcase.rs:21 | 훅 C: 밴픽 일러 에셋 조회 |
| `RVA_SUBMIT` | `0x248b1c0` | const | src/showcase.rs:22 | b1c0(list, &cmd) 일반 제출 |
| `RVA_SUBMIT_TEXT` | `0x248b400` | const | src/showcase.rs:23 | b400(list, &cmd) 텍스트 전용 |
| `RVA_IMG_BUILD` | `0x248c130` | const | src/showcase.rs:24 | c130(&cmd, key, len, x, y, layer, w, h, 0,0,0,0) |
| `RVA_IMG_UV` | `0x248c7c0` | const | src/showcase.rs:25 | c7c0(&out, &in, &uv) |
| `RVA_IMG_FLAG` | `0x248cd40` | const | src/showcase.rs:26 | cd40(&out, &in, 샘플링: 1=nearest) |
| `RVA_IMG_COLOR` | `0xff0c20` | const | src/showcase.rs:27 | ff0c20(&out, &in, "color", 5, &rgba) |
| `RVA_IMG_SHADER` | `0x248e850` | const | src/showcase.rs:28 | e850(&out, &in, shader_key, len) |
| `RVA_TEXT_BUILD` | `0x248c1e0` | const | src/showcase.rs:29 | c1e0(...) 텍스트 cmd |
| `RVA_NAME_GET` | `0x1217630` | const | src/showcase.rs:30 | 챔프 표시명 String |
| `RVA_ASSET_GET` | `0x99c860` | const | src/showcase.rs:31 | 키→텍스처 에셋 (obj,vtbl) 엔트리 주소 |
| `RVA_ANIM_GET` | `0x5ab7d0` | const | src/showcase.rs:32 | 키→애님 리소스 (참조 반환) |
| `RVA_SPRITE_CALC` | `0x121aca0` | const | src/showcase.rs:33 | idle 시트키+UV+크기 계산기(무부작용) |
| `RVA_GAME_ALLOC` | `0x8b7f80` | const | src/showcase.rs:34 | (size, align) → ptr |
| `RVA_GAME_FREE` | `0x8b7f90` | const | src/showcase.rs:35 | (ptr, size, align) |
| `RVA_C_CARD_RECT` | `0x3731380` | const | src/showcase.rs:57 | {-180,-240,360,480} 카드 로컬 rect(밴·픽 공용) |
| `RVA_C_SNAP_RECT` | `0x37313b0` | const | src/showcase.rs:58 | {0,0,360,480} 스냅샷 내부 rect(좌상단 원점) |
| `RVA_C_LINE_DIR` | `0x37313e0` | const | src/showcase.rs:59 | {360,340} 취소선 방향 |
| `RVA_C_LINE_START` | `0x37313f0` | const | src/showcase.rs:60 | {-180,170} 취소선 시작 |
| `RVA_C_LINE_ANCHOR` | `0x3731400` | const | src/showcase.rs:61 | {0,170} 앵커 |
| `RVA_C_NORMAL` | `0x37313c0` | const | src/showcase.rs:62 | {0.6866,0.727} 분리 법선 |
| `RVA_I_SNAP_H` | `0x124e2ba` | const | src/showcase.rs:63 | mov dword [rsp+0x20], 480.0 (스냅샷 타깃 높이) |
| `RVA_D_SNAP_W` | `0x124e2c2` | const | src/showcase.rs:64 | disp → 360.0 (스냅샷 폭, 광공유) |
| `RVA_D_CUT_LO` | `0x1201e19` | const | src/showcase.rs:65 | disp → -70.0 (1201d90 하단 컷) |
| `RVA_D_CUT_HI` | `0x1201e27` | const | src/showcase.rs:66 | disp → +70.0 (1201d90 상단 컷) |
| `RVA_D_ZIG_X1` | `0x124e8cf` | const | src/showcase.rs:67 | disp → -180.0 (지그재그 x) |
| `RVA_D_ZIG_X2` | `0x124efa1` | const | src/showcase.rs:68 | disp → -180.0 (〃 두 번째 블록) |
| `RVA_SLOTS` | `0x3fd2b00` | const | src/showcase.rs:69 | .rdata 패딩 슬롯 [w, cut_lo, cut_hi, zig_x] |

## tfm2_draft_overlay (5건)

| 상수 | RVA(0.5.2) | 종류 | 위치 | 용도 |
|---|---|---|---|---|
| `ANIM_GET_RVA` | `0x40e250` | const | src/lib.rs:142 | 0.5.1(2026-07-15, 구0.5.0_3=0x51bbc0). ⚠상대유도=LOADER(0x40f3d0)-0x1180=0x |
| `LOADER_RVA` | `0x40f3d0` | const | src/lib.rs:359 | 0.5.1(2026-07-15, 구0.5.0_3=0x51cd40, MOVED→string-xref 'layout/main' c |
| `BANPICK_LOADER_RVA` | `0xeb17d0` | const | src/lib.rs:365 | ⚠tfm2_item_tactics 가 같은 0xeb17d0 을 STRAT_LOADER 로 후킹 중 → 반드시 체인(진입부 12 |
| `PARSER_RVA` | `0x24b4590` | const | src/lib.rs:366 | 0.5.1(2026-07-15, 구0.5.0_3=0x2499f30, UNIQUE·item_tactics와 동일함수). .ui  |
| `ALLOC_RVA` | `0x25c5a40` | const | src/lib.rs:367 | 0.5.1(2026-07-15, 구0.5.0_3=0x25ab3d0, UNIQUE·item_tactics와 동일함수). 게임 a |

## tfm2_elemental_serpen (18건)

| 상수 | RVA(0.5.2) | 종류 | 위치 | 용도 |
|---|---|---|---|---|
| `SERPEN_RVA` | `0x21f8ca0` | const | src/lib.rs:34 | 0.5.2 (구0.5.1=0x1f8d0c0) kind6. ★migrator 오답 0x1c70e90=kind5 Epic이었음.  |
| `MOBATICK_RVA` | `0x230c290` | const | src/lib.rs:350 | 0.5.2 (구0.5.1=0x21fcf90). 마스크시그 UNIQUE + 본문 L1=0.976/L2=0.987(경미 리인라인) |
| `SPAWN_HOOKS` | `0x53aae0,0x539f40` | array[2] | src/lib.rs:405 | 0.5.2 (구0.5.1=0x50edd0/0x50e230). 둘 다 스켈레톤 L1 UNIQUE·프롤로그 동일. |
| `(inline)` | `0x53aae0` | inline | src/lib.rs:405 | const SPAWN_HOOKS: [usize; 2] = [0x53aae0, 0x539f40]; |
| `(inline)` | `0x539f40` | inline | src/lib.rs:405 | const SPAWN_HOOKS: [usize; 2] = [0x53aae0, 0x539f40]; |
| `LAUNCHER_RVA` | `0x1d96870` | const | src/lib.rs:414 | 0.5.2 (구0.5.1=0x20588a0). 콜그래프 앵커 7/7 만장일치(전 콜사이트 EQ 정렬). |
| `LAUNCHER_RET_A` | `0x759c36` | const | src/lib.rs:420 | 0.5.2 화면 경기 경로 A (구0.5.1=0x72f507) |
| `LAUNCHER_RET_B` | `0x75e5cf` | const | src/lib.rs:421 | 0.5.2 화면 경기 경로 B (구0.5.1=0x733e9f) |
| `LAUNCHER_RET_C` | `0x1555215` | const | src/lib.rs:425 | 0.5.2 리플레이 경로 (콜사이트 0x1555210+5) |
| `UILOADER_RVA` | `0x5ac950` | const | src/lib.rs:513 | 0.5.2 (구0.5.1=0x40f3d0) 제네릭 asset-get(main/ingame 계열). ⚠item_tactics 등 |
| `UIPARSER_RVA` | `0x24b5a00` | const | src/lib.rs:514 | 0.5.2 (구0.5.1=0x24b4590) .ui 텍스트 → NodeTemplate. 스켈레톤 L1 UNIQUE. |
| `UIALLOC_RVA` | `0x25c4d30` | const | src/lib.rs:515 | 0.5.2 (구0.5.1=0x25c5a40) 게임 힙 alloc(size, align). 스켈레톤 L1 UNIQUE. |
| `RENDER_STEP_RVA` | `0x811500` | const | src/lib.rs:717 | 0.5.2 (구0.5.1=0x872950). 스켈레톤 L1 UNIQUE·프롤로그 동일. |
| `RUNNER_CTOR_RVA` | `0x1d981e0` | const | src/lib.rs:744 | 0.5.2 (구0.5.1=0x205a2f0). 콜그래프 앵커 3/3 만장일치(EQ 정렬)·프롤로그 12B 동일. |
| `DMGA_RVA` | `0x22164a0` | const | src/lib.rs:1707 | 0.5.2 (구0.5.1=0x1f147e0, +0x301cc0=PUSH와 동일 델타) 최종 HP 감산 어플라이어 (모든 실드- |
| `DMGB_RVA` | `0x22d2b20` | const | src/lib.rs:1710 | 0.5.2 (구0.5.1=0x21e2400) 딜 파이프라인 (r8=World) — TLS world 캡처 전용. 스켈레톤 L1 |
| `KEYRES_RVA` | `0xc2f990` | const | src/lib.rs:1902 | 0.5.2 (구0.5.1=0x13c0e90). 콜그래프 앵커 7/7 만장일치(전 콜사이트 EQ 정렬). |
| `ARG_STR_RVA` | `0xfef190` | const | src/lib.rs:2427 | 0.5.2 (구0.5.1=0xb4fda0) |

## tfm2_comptest_unlock (47건)

| 상수 | RVA(0.5.2) | 종류 | 위치 | 용도 |
|---|---|---|---|---|
| `no_stamina_cost` | `0xe93b2d` | patch_site | src/tfm2_comptest_unlock.rs:60 | 0.5.2(구0.5.1=0xf3411d, 컨테이너정렬 HIGH·orig 05 MATCH) |
| `daily_remaining` | `0x1f14090` | patch_site | src/tfm2_comptest_unlock.rs:69 | 0.5.2(구0.5.1=0x1c0b480, 33B 본문시그 양쪽 UNIQUE) |
| `daily_inc_gate` | `0xe8cb20` | patch_site | src/tfm2_comptest_unlock.rs:78 | 0.5.2(구0.5.1=0xf2d110, 컨테이너정렬 HIGH·orig 04 MATCH) |
| `server_dedup_real` | `0xec7758` | patch_site | src/tfm2_comptest_unlock.rs:101 | ⇒ 0.5.1에서 인게임 검증(07-20)된 그 게이트가 맞음. fixed(6B nop)는 무변경. |
| `allow_dup_players` | `0xd00ee5` | patch_site | src/tfm2_comptest_unlock.rs:106 | 0.5.2(구0.5.1=0x1615495, 컨테이너 L1-UNIQUE·orig 75 76 MATCH) |
| `server_dedup` | `0xe8b5fa` | patch_site | src/tfm2_comptest_unlock.rs:122 | 0.5.2(구0.5.1=0xf2bbea, 컨테이너정렬·orig 75 10 MATCH·no-op 유지) |
| `btn5v5_roster_min_a` | `0xd967cf` | patch_site | src/tfm2_comptest_unlock.rs:138 | 0.5.2(구0.5.1=0x167fecf) cmp r12,0xa → 5 (빌더A disabled) |
| `btn5v5_roster_min_b` | `0xcf7b68` | patch_site | src/tfm2_comptest_unlock.rs:141 | 0.5.2(구0.5.1=0x160c238) cmp r13,0xa → 5 (빌더B disabled) |
| `btn5v5_warn_text` | `0xd9662c` | patch_site | src/tfm2_comptest_unlock.rs:144 | 0.5.2(구0.5.1=0x167fd2c) 경고문구 임계 정합 |
| `server_roster_min` | `0xec768e` | patch_site | src/tfm2_comptest_unlock.rs:170 | 0.5.2(0.5.1 대응=0xf67ace) 필요치 2×N → 1×N |
| `roster_count_gate` | `0xd0a74c` | patch_site | src/tfm2_comptest_unlock.rs:182 | 0.5.2(구0.5.1=0x161edbc, RUN컨테이너 L1-UNIQUE·orig MATCH) |
| `collected_gate` | `0xd0a740` | patch_site | src/tfm2_comptest_unlock.rs:187 | 0.5.2(구0.5.1=0x161edb0, RUN컨테이너 L1-UNIQUE·orig 7510 MATCH) |
| `collect_err_gate` | `0xd0a728` | patch_site | src/tfm2_comptest_unlock.rs:191 | 0.5.2(구0.5.1=0x161ed98, RUN컨테이너 L1-UNIQUE·orig 746a MATCH) |
| `run_push_gate` | `0xd0adf1` | patch_site | src/tfm2_comptest_unlock.rs:197 | 0.5.2(구0.5.1=0x161f461, RUN컨테이너 L1-UNIQUE·orig 0f846cfaffff MATCH) |
| `DISP_RVA` | `0xd3f780` | const | src/tfm2_comptest_unlock.rs:292 | 0.5.2(구0.5.1=0xc82370, L1-UNIQUE 스켈레톤·DISP_PROLOGUE 12B 실측 MATCH) |
| `INSERT_RVA` | `0xcabac0` | const | src/tfm2_comptest_unlock.rs:355 | ⬜미확정(죽은 상수·inert 확인필·0.5.1값 유지) |
| `CT_REGION_LO` | `0xe7ccd0` | const | src/tfm2_comptest_unlock.rs:364 | 0.5.2(구0.5.1환산 0xf1d2c0, 컨테이너 매칭 HIGH) |
| `CT_REGION_HI` | `0xea2345` | const | src/tfm2_comptest_unlock.rs:365 | 0.5.2(= LO + pdata 크기 0x25675) |
| `CT_CLIENT_LO` | `0xcf0000` | const | src/tfm2_comptest_unlock.rs:366 | 0.5.2(~~0xf50000~~ = 0.5.1서 이미 무효였던 범위) |
| `CT_CLIENT_HI` | `0xda0000` | const | src/tfm2_comptest_unlock.rs:367 | 0.5.2(~~0xf80000~~) |
| `ATH_ID_HI` | `0x100000` | const | src/tfm2_comptest_unlock.rs:375 |  |
| `(inline)` | `0xd00ed0` | inline | src/tfm2_comptest_unlock.rs:384 | const FORGE_CALLERS: &[usize] = &[0xd00ed0]; |
| `ENQ_RVA` | `0xcb9c80` | const | src/tfm2_comptest_unlock.rs:469 | ⬜미확정(죽은 상수·inert 확인필·0.5.1값 유지) |
| `RUN_RVA` | `0xd0a440` | const | src/tfm2_comptest_unlock.rs:526 | 0.5.2(구0.5.1=0x161eab0, **L1-UNIQUE 스켈레톤 바이트동일**·628 instr·PROL-OK pus |
| `SRV_RVA` | `0x13d4af0` | const | src/tfm2_comptest_unlock.rs:529 | ⬜미확정(죽은 상수·훅 비활성·0.5.1값 유지) |
| `LOADING_RVA` | `0xd186f0` | const | src/tfm2_comptest_unlock.rs:604 | 0.5.2(구0.5.1=0x162cf10, **L1-UNIQUE**·199 instr·LOADING_PROLOGUE 15B 실 |
| `DEDUP_INS_RVA` | `0xca75f0` | const | src/tfm2_comptest_unlock.rs:628 | ⬜미확정(죽은 상수·inert 확인필·0.5.1값 유지) |
| `SPAWN_CP_RVA` | `0x13c71b0` | const | src/tfm2_comptest_unlock.rs:629 | ⬜미확정(죽은 상수·inert 확인필·0.5.1값 유지) |
| `PUSH_RVA` | `0x101cc08` | const | src/tfm2_comptest_unlock.rs:688 | ⬜미확정(죽은 상수·프로브 호출 비활성 상태·0.5.1값 유지) |
| `FN_DD_SETOPT_RVA` | `0x242f250` | const | src/tfm2_comptest_unlock.rs:830 | ⚠0.5.1 때의 "구 0x2416070 금지" 경고와 동류 — 반드시 이 값만 쓸 것. |
| `ITEMCONV_RVA` | `0xed8770` | const | src/tfm2_comptest_unlock.rs:970 | 07-21 인게임 검증된 comp_test 아이템칸 모드템 주입 POST 훅. |
| `(inline)` | `0xf794c0` | inline | src/tfm2_comptest_unlock.rs:1039 | .map(\|s\| format!("itemconv 0xf794c0 {}", s)) |
| `COLLECT_RVA` | `0xd0bd80` | const | src/tfm2_comptest_unlock.rs:1042 | 0.5.2(구0.5.1=0x16203f0, **L1-UNIQUE**·145 instr·PROL-OK push8 12B) |
| `EF1EA0_RVA` | `0xe58c30` | const | src/tfm2_comptest_unlock.rs:1125 | (이름의 EF1EA0은 0.4.x RVA 유래 — 주소가 아니라 식별자로만 유지) |
| `ATH_GET_SC_RVA` | `0xe3b200` | const | src/tfm2_comptest_unlock.rs:1133 | shadow-call: rcx=game_ctx+0x16b90, rdx=&id → rax(0=miss), athlete*=[ra |
| `ORACLE_RVA` | `0x1d94720` | const | src/tfm2_comptest_unlock.rs:1192 | 1틱 오케(run one tick), 프롤로그=HOOK_PROLOGUE12 |
| `(inline)` | `0x20566c0` | inline | src/tfm2_comptest_unlock.rs:1356 | .map(\|s\| format!("oracle 0x20566c0 {}", s))?; |
| `SLOT_RVA` | `0xd1acf0` | const | src/tfm2_comptest_unlock.rs:1383 | 독립 확정되어 자기일치. PROL-OK push8 12B. 신뢰도 HIGH. |
| `RUST_ALLOC_RVA` | `0x8b7f80` | const | src/tfm2_comptest_unlock.rs:1393 | 각각 **정확히 1개**. (참고: alloc 실체 0x25c4d30 = uinj ALLOC_RVA와 동일 함수 — 0.5.1 |
| `RUST_DEALLOC_RVA` | `0x8b7f90` | const | src/tfm2_comptest_unlock.rs:1394 |  |
| `ATH_GET_RVA` | `0x402840` | const | src/tfm2_comptest_unlock.rs:1507 | ⬜죽은 상수(0.4.x 잔재)·훅 호출 비활성·실체=ATH_GET_SC_RVA |
| `ATH_GET_JE_TARGET_RVA` | `0x4028fb` | const | src/tfm2_comptest_unlock.rs:1508 | ⬜위와 동일(실체 기준 0.5.2값은 0xe3b2bb) |
| `CT_ARM_LO` | `0x13e1c00` | const | src/tfm2_comptest_unlock.rs:1510 | ⬜미확정(0.5.0_2 기준·HYBRID 비활성이라 inert) |
| `CT_ARM_HI` | `0x13ea200` | const | src/tfm2_comptest_unlock.rs:1511 | ⬜미확정(위와 동일) |
| `LOADER_RVA` | `0x5ac950` | const | src/ui_inject.rs:32 | 0.5.2 단일 asset-get copy(~~0.5.1 0x40f3d0=오선택~~, 실제 training은 0xeb17d0였 |
| `PARSER_RVA` | `0x24b5a00` | const | src/ui_inject.rs:33 | 0.5.2(~~0.5.1 0x24b4590~~, L1-UNIQUE·454 instr) = ai_adjust 세션 확정값과 일치 |
| `ALLOC_RVA` | `0x25c4d30` | const | src/ui_inject.rs:34 | 0.5.2(~~0.5.1 0x25c5a40~~, L1-UNIQUE·35 instr) = ai_adjust 세션 확정값과 일치 |

## tfm2_banpick_order (22건)

| 상수 | RVA(0.5.2) | 종류 | 위치 | 용도 |
|---|---|---|---|---|
| `PANIC_SITES` | `0x11da680,0x11da6a0,0x11db418,0x11db438,0x11dbed9,0x11dc023` | array[6] | src/diag.rs:64 | int3(0xCC) 패치 → 크래시 경로가 어느 site에 도달하는지 포착. |
| `RVA_PANIC_HOOK` | `0x25d4764` | const | src/diag.rs:636 | 프롤로그 13B = `55 41 56 56 57 53 48 81 EC 80 00 00 00` (rip-rel 없음, 실측 확인 |
| `RVA_PHASE_INFO` | `0x1cd9380` | const | src/hooks.rs:23 | A |
| `RVA_PHASE_SCALAR` | `0x1d04120` | const | src/hooks.rs:24 | B |
| `RVA_APPLIER` | `0x11e2140` | const | src/hooks.rs:25 | C |
| `RVA_APP_PICK_T1` | `0x11ce240` | const | src/hooks.rs:275 |  |
| `RVA_APP_PICK_T2` | `0x11ce400` | const | src/hooks.rs:276 |  |
| `RVA_APP_BAN_T1` | `0x120c020` | const | src/hooks.rs:277 |  |
| `RVA_APP_BAN_T2` | `0x120c1d0` | const | src/hooks.rs:278 |  |
| `RVA_TRANSITION` | `0x11d8ef0` | const | src/hooks.rs:279 |  |
| `RVA_AI_SITE1` | `0x1c04389` | const | src/hooks.rs:299 | phase_from은 r8=ban_count를 받아 내부에서 2배(진입 `add r8,r8`)하므로 그대로 전달. |
| `RVA_AI_JOIN1` | `0x1c04475` | const | src/hooks.rs:300 |  |
| `RVA_AI_SITE2` | `0x1c07938` | const | src/hooks.rs:301 |  |
| `RVA_AI_JOIN2` | `0x1c07a09` | const | src/hooks.rs:302 |  |
| `RVA_SFX_SITE` | `0x1251303` | const | src/hooks.rs:315 | 원본: r8 = 문자열 ptr, r9 = 길이(ban 0x1c / pick 0x1d) 세팅 후 0x1251352로 진행. |
| `RVA_SFX_END` | `0x1251352` | const | src/hooks.rs:316 |  |
| `RVA_STR_BAN` | `0x373d596` | const | src/hooks.rs:317 | "asset/base/sound/sfx/ban_sfx"  (0x1c) |
| `RVA_STR_PICK` | `0x373d5b2` | const | src/hooks.rs:318 | "asset/base/sound/sfx/pick_sfx" (0x1d) |
| `RVA_BANNER` | `0x11df9f0` | const | src/hooks.rs:357 |  |
| `RVA_LINEUP` | `0x11cedb0` | const | src/hooks.rs:384 | 실행, 아니면 스킵(레인 표시만 스테일, 밴픽 진행·전환은 별개 이벤트라 그대로 진행). |
| `RVA_COMMIT` | `0x1d075d0` | const | src/hooks.rs:402 | 원본의 중복검사·상한·fearless·allocator 경로는 전부 보존된다. |
| `RVA_TURN` | `0x1d07cf0` | const | src/hooks.rs:416 | 2워드 반환은 Rust로 직접 불가 → raw 스텁(out 파라미터 → rdx 로드)으로 처리. |

## tfm2_transfer_tweak (7건)

| 상수 | RVA(0.5.2) | 종류 | 위치 | 용도 |
|---|---|---|---|---|
| `RVA_GATE` | `0x1d15e90` | const | src/lib.rs:43 | 선수 수락 판정(의향 게이트 + 배율 문턱) |
| `RVA_TBL` | `0x3835560` | const | src/lib.rs:44 | 문턱 테이블 [2.25, 1.85, 1.95, 1.65] (전용, xref 1) |
| `thr_1_20` | `0x1d1626b` | patch_site | src/lib.rs:52 |  |
| `thr_1_45` | `0x1d162db` | patch_site | src/lib.rs:53 |  |
| `thr_1_35` | `0x1d162e9` | patch_site | src/lib.rs:54 |  |
| `pen_0_25` | `0x1d16340` | patch_site | src/lib.rs:55 |  |
| `gate_0_30` | `0x1d162ab` | patch_site | src/lib.rs:56 |  |

## tfm2_level_cap (2건)

| 상수 | RVA(0.5.2) | 종류 | 위치 | 용도 |
|---|---|---|---|---|
| `RVA_LEN_LOAD` | `0x22d3fea` | const | src/lib.rs:82 | 레벨업 함수 0x22d3c60 내 |
| `RVA_UI_CMP` | `0x80ae73` | const | src/lib.rs:88 | 인스턴스에서 ptr을 읽으므로, 앞선 이 지점에서 교체하면 인덱싱도 함께 따라온다. |
