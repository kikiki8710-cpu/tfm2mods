# -*- coding: utf-8 -*-
"""comptest 0.7.1 RVA 외 소스 수정 — 줄 단위 바이트 치환(줄바꿈 보존 · 줄마다 구값 단언)."""
P = r'F:\claude\tfm2mods\tfm2_comptest_unlock\src\tfm2_comptest_unlock.rs'
b = open(P, 'rb').read()
L = b.split(b'\n')
def sub(n, old, new, cnt=1):
    s = L[n - 1].decode('utf-8')
    assert s.count(old) >= 1 and (cnt == 0 or s.count(old) == cnt), (n, old, s[:120])
    L[n - 1] = s.replace(old, new).encode('utf-8')
# 러너 엔트리 슬롯 리터럴 19곳 → 상수
for n in (1308, 1312, 1606, 2040, 2042, 2059, 2060, 2061, 2320, 2322, 2972, 2973, 3010, 3011, 3020, 3021, 3077, 3081, 3086):
    sub(n, '0x1f70', 'R_ENTRY', 0)
sub(897, 'const G_OFF: usize = 0x1ba0;  // ★0.6.3', 'const G_OFF: usize = 0x1b40;  // ★0.7.1 Δ−0x60([팀 상태;2] stride 0x760→0x730 · ORACLE mov rcx,[rdx+0x1b40]) // ★0.6.3')
sub(1086, 'const CMD_STRIDE: usize = 0x1ef0;  // ★0.6.3', 'const CMD_STRIDE: usize = 0x1e90;  // ★0.7.1 RUN imul 0x1e90 // ★0.6.3')
sub(1325, 'const CD_RC_OFF: usize = 0x2198;  // ★0.6.3', 'const CD_RC_OFF: usize = 0x2138;  // ★0.7.1 러너 −0x60(RPLY3 [rsi+0x2138]) // ★0.6.3')
sub(1776, 'const CTX_SZ: usize = 0x1e70;  // ★0.6.3', 'const CTX_SZ: usize = 0x1e10;  // ★0.7.1 ARRIVE memcpy 0x1e10 // ★0.6.3')
sub(2177, '0xb8, 0x40, 0x3e, 0x00, 0x00];  // ★0.6.3 mov eax,0x3e40', '0xb8, 0x40, 0x3d, 0x00, 0x00];  // ★0.7.1 mov eax,0x3d40 // ★0.6.3 mov eax,0x3e40')
sub(2180, 'const R_STATE: usize = 0x21df;  // ★0.7.0', 'const R_ENTRY: usize = 0x1f10;  // ★0.7.1 러너 엔트리 슬롯 −0x60(ARRIVE·CSEND [r14+0x1f10] · 구 리터럴 0x1f70 19곳을 상수로)\nconst R_STATE: usize = 0x217f;  // ★0.7.1 러너 0x21e0→0x2180(−0x60 · CSEND [r14+0x217f] · 러너 마지막 바이트) // ★0.7.0')
sub(2181, 'const DETAIL_SZ: usize = 0x1e70;  // ★0.6.3', 'const DETAIL_SZ: usize = 0x1e10;  // ★0.7.1 // ★0.6.3')
sub(2405, '[0x41, 0xc6, 0x86, 0xdf, 0x21, 0x00, 0x00];  // ★0.7.0', '[0x41, 0xc6, 0x86, 0x7f, 0x21, 0x00, 0x00];  // ★0.7.1 R_STATE 0x217f // ★0.7.0')
sub(2892, 'const DETAIL_SZ2: usize = 0x1e70;  // ★0.6.3', 'const DETAIL_SZ2: usize = 0x1e10;  // ★0.7.1 // ★0.6.3')
sub(2896, '[0x48,0xc7,0x85,0xb0,0x6c,0x01,0x00,0xff,0xff,0xff,0xff,  // ★0.7.0', '[0x48,0xc7,0x85,0x50,0x6c,0x01,0x00,0xff,0xff,0xff,0xff,  // ★0.7.1: 프레임 0x18028→0x17fd8(−0x50) ⟹ [rbp+0x16c50] (신 0xaecd3a 실측) // ★0.7.0')
sub(2917, 'const DETAIL_B_OFF: usize = 0x2050;  // ★0.6.3', 'const DETAIL_B_OFF: usize = 0x1ff0;  // ★0.7.1 러너 −0x60(ARRIVE·CSEND) // ★0.6.3')
sub(3095, '(runner + 0x21da) as *mut u8, 1);', '(runner + 0x217a) as *mut u8, 1);')
sub(3096, '(runner + 0x21d4) as *mut f32, 0.0);', '(runner + 0x2174) as *mut f32, 0.0);')
sub(3127, '"sub rsp, 0x3e40",  // ★0.6.3', '"sub rsp, 0x3d40",  // ★0.7.1 ARRIVE 프레임 0x3e40→0x3d40 // ★0.6.3')
sub(3134, '"lea rbp, [rsp-0x3dc0]",  // ★0.6.3', '"lea rbp, [rsp-0x3cc0]",  // ★0.7.1 rbp 0x3dc0→0x3cc0(EPILOG lea rsp,[rbp+0x3cc0]) // ★0.6.3')
sub(3152, '"mov r8, [rbp+0x18020]",  // ★0.7.0', '"mov r8, [rbp+0x17fd0]",  // ★0.7.1 frame 0x17fd8(−0x50) → p4 [rbp+0x17fd0] (슬롯 사용횟수 236/15/10/15/1 · RE 2026-11-04_0.7.1-재핀판정) // ★0.7.0')
sub(3158, '"mov qword ptr [rbp+0x16cb0], -1",  // ★0.7.0', '"mov qword ptr [rbp+0x16c50], -1",  // ★0.7.1 0x16cb0→0x16c50 // ★0.7.0')
sub(3767, 'game + 0x1bb8', 'game + 0x1b58')
sub(3768, 'game + 0x1ba8', 'game + 0x1b48')
for n in (3780, 3818):
    sub(n, 'game + 0x1e5a', 'game + 0x1dfa', 2)
for n in (3781, 3782, 3819, 3820):
    sub(n, 'game + 0x1ba0', 'game + 0x1b40')
open(P, 'wb').write(b'\n'.join(L))
print('ok')
