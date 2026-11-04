# -*- coding: utf-8 -*-
P = r'F:\claude\tfm2mods\tfm2_elemental_serpen\src\lib.rs'
b = open(P, 'rb').read()
reps = [
 ('const GAME_PROVIDER_OFF: usize = 0x1ba0; // ★0.6.3', 'const GAME_PROVIDER_OFF: usize = 0x1b40; // ★0.7.1 Game Δ−0x60(RUNNER_CTOR mov [rsi+0x1b40],rax) // ★0.6.3'),
 ('safe_read_u64(game + 0x1ba0) { // ★0.6.3 Game 대역', 'safe_read_u64(game + 0x1b40) { // ★0.7.1 Game Δ−0x60 // ★0.6.3 Game 대역'),
 ('const DMGA_PROLOGUE: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53]; // ★0.7.0', 'const DMGA_PROLOGUE: [u8; 12] = [0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53]; // ★0.7.1 이벤트 기록 아웃라인으로 프레임 0.6.5 형 복귀(push rbp 뒤로 · RE 2026-11-04_0.7.1-재핀판정) // ★0.7.0'),
]
for o, n in reps:
    ob, nb = o.encode('utf-8'), n.encode('utf-8')
    assert b.count(ob) == 1, o
    b = b.replace(ob, nb)
open(P, 'wb').write(b)
print('ok')
