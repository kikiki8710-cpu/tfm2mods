# -*- coding: utf-8 -*-
"""114: target 오프셋 +0x8 · 페이로드 대조는 살아있는 17B(+0..+0x11)만."""
import io
p = r'C:\tfm2mods\MIG\_verify22\A\oracle\v22A_o1.rs'
s = io.open(p, encoding='utf-8').read()
a = "got.push((rd::<u8>(el, 0xb1), rd::<usize>(el, 0x10)));"
b = "got.push((rd::<u8>(el, 0xb1), rd::<usize>(el, 0x8)));"
assert a in s; s = s.replace(a, b)
a2 = "            if cur != ref24 { pay_ok = false; println!(\"114\\tpayload mismatch i={} cur={:?} ref={:?}\", i, cur, ref24); }"
b2 = """            // 살아있는 바이트 = start_tick(8)+target(8)+is_act(1) = 17B. +0x11..+0x18 은 패딩(값 비결정)
            if cur[..17] != ref24[..17] { pay_ok = false; println!("114\\tpayload mismatch i={} cur={:?} ref={:?}", i, cur, ref24); }
            println!("114\\tel[{}] tag={} live17={:?} pad7={:?} start_tick={} target={} is_act={}", i, tag, &cur[..17], &cur[17..], rd::<usize>(el, 0), rd::<usize>(el, 8), rd::<u8>(el, 16));"""
assert a2 in s; s = s.replace(a2, b2)
io.open(p, 'w', encoding='utf-8').write(s)
print('patched')
