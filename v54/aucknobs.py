# -*- coding: utf-8 -*-
"""경매 강제귀환 12노브 — prefix/imm위치/원본값을 0.5.4 exe 로 전수 검증."""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
from pe2 import load
import capstone
p = load('054')
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64); md.detail = True

K = [
 ('auc_flee_version_gate', 0xead271, '48 83 f8', 3, 1, 1),
 ('auc_flee_undying_gate', 0xead285, '41 80 b9 70 04 00 00', 7, 1, 0),
 ('auc_flee_hp_field',     0xead5e1, '48 3b 81', 3, 4, 0x658),
 ('auc_flee_nexus_mask',   0xead68d, 'a9', 1, 4, 0x100),
 ('auc_flee_goal_far',     0xead6c8, 'b9', 1, 4, 928000),
 ('auc_flee_goal_near_a',  0xead6cd, 'ba', 1, 4, 32000),
 ('auc_flee_goal_near_b',  0xead6d6, '41 b8', 2, 4, 32000),
 ('auc_flee_end_delay',    0xead6ff, '49 c7 84 24 28 15 00 00', 8, 4, 5),
 ('auc_flee_pathfinder',   0xead72f, '41 c6 84 24 8d 15 00 00', 8, 1, 2),
 ('auc_flee_with_skill',   0xead738, '41 c7 84 24 90 15 00 00', 8, 4, 1),
 ('auc_flee_score',        0xead759, '49 c7 84 24 08 15 00 00', 8, 4, 99999),
 ('auc_flee_action_tag',   0xead765, '41 c6 84 24 c1 15 00 00', 8, 1, 3),
]
bad = 0
for name, a, pre, off, w, orig in K:
    pb = bytes(int(x, 16) for x in pre.split())
    got = p.rd(a, 24)
    ok_pre = got[:len(pb)] == pb
    ins = next(md.disasm(got, a), None)
    # imm/disp 실제 위치를 capstone 인코딩에서 확인
    real = None
    if ins:
        e = ins.encoding
        if w == 4 and e.disp_offset and name == 'auc_flee_hp_field': real = e.disp_offset
        elif e.imm_offset: real = e.imm_offset
        elif e.disp_offset: real = e.disp_offset
    cur = int.from_bytes(got[off:off+w], 'little')
    if w == 1 and cur > 127: cur -= 256
    st = []
    if not ok_pre: st.append('prefix불일치(%s)' % got[:len(pb)].hex())
    if real is not None and real != off: st.append('imm위치 실제=%d≠표=%d' % (real, off))
    if cur != orig: st.append('원본 실제=%d≠표=%d' % (cur, orig))
    if ins and off + w > ins.size: st.append('명령경계 초과(size=%d)' % ins.size)
    if st: bad += 1
    print('%-24s %s  %s' % (name, 'OK ' if not st else '⚠', ' / '.join(st) or (ins.mnemonic + ' ' + ins.op_str)))
print('\n검증 %d/%d 통과' % (len(K) - bad, len(K)))
