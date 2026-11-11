# -*- coding: utf-8 -*-
r"""기본값 등가 검사 — 창마다 「기본값으로 돌린 동적 경로」가 「원본 경로」와 같은 결과를 내는지 Unicorn 으로 대조한다.

왜: 콜백 입력 레지스터·dyn 조각은 targets·추적기·gen_sites 어디서도 의미를 검증하지 않는다. 틀려도 값을 바꾼 선수에게서만
조용히 어긋난다(21 §4-A·B). 기본값이면 동적 경로가 원본과 같아야 한다는 불변식은 이것을 기계로 잡는다.

    python -X utf8 tools/verify_default_equiv.py [--samples 40] [--only 이름일부] [--mutate] [--out 결과.json]

- 입력 = src/sites.rs(생성물) + tools/patchday/baseline 없이 src/tactics.rs 의 기본값. 게임 실행 없음·파일 수정 없음.
- 콜백은 Rust 소스를 그대로 옮긴 PORT 표로 계산한다(Frame 입력 = 창 진입 시점 레지스터·메모리).
- 콜백이 None 을 내는 표본은 dispatch 가 원본으로 보내므로 건너뛴다.
- --mutate: 콜백 입력을 일부러 틀리게 해서 검사기가 잡는지 본다(음성 대조).
종료코드: 0 = 전 창 일치 · 1 = 불일치 있음 · 2 = 표본 부족/실행 오류.
"""
import argparse
import json
import random
import re
import struct
import sys
from pathlib import Path

MOD = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(MOD / 'target' / 'verify_deps'))
sys.path.insert(0, str(MOD / 'tools'))
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE, UC_HOOK_MEM_WRITE, UcError  # noqa: E402
from unicorn.x86_const import (UC_X86_REG_EFLAGS, UC_X86_REG_R10, UC_X86_REG_R11, UC_X86_REG_R12, UC_X86_REG_R13, UC_X86_REG_R14,  # noqa: E402
                               UC_X86_REG_R15, UC_X86_REG_R8, UC_X86_REG_R9, UC_X86_REG_RAX, UC_X86_REG_RBP, UC_X86_REG_RBX,
                               UC_X86_REG_RCX, UC_X86_REG_RDI, UC_X86_REG_RDX, UC_X86_REG_RSI, UC_X86_REG_RSP)

REGS = [UC_X86_REG_RAX, UC_X86_REG_RCX, UC_X86_REG_RDX, UC_X86_REG_RBX, UC_X86_REG_RSP, UC_X86_REG_RBP, UC_X86_REG_RSI, UC_X86_REG_RDI,
        UC_X86_REG_R8, UC_X86_REG_R9, UC_X86_REG_R10, UC_X86_REG_R11, UC_X86_REG_R12, UC_X86_REG_R13, UC_X86_REG_R14, UC_X86_REG_R15]
RN = 'rax rcx rdx rbx rsp rbp rsi rdi r8 r9 r10 r11 r12 r13 r14 r15'.split()
M = (1 << 64) - 1
BASE = 0x140000000
CODE = 0x2100000
LOW = 0x2000000                  # 0..LOW = 작은 수 qword 로 채운 메모리(포인터로도, 정수로도 말이 되게)
RSP = 0x1800000
RBP = 0x1810000
FLAGS = 0x8d5                    # CF PF AF ZF SF OF


def s64(x):
    return x - (1 << 64) if x >> 63 else x


def tdiv(a, b):
    q = abs(a) // abs(b)
    return q if (a >= 0) == (b >= 0) else -q


class F:
    def __init__(s, regs, rd):
        s.r, s.rd = regs, rd

    def reg(s, i):
        return s.r[i]

    def mem(s, r, d):
        return s.rd((s.r[r] + d) & M)

    def mem2(s, r, a, b):
        return s.rd((s.mem(r, a) + b) & M)


def n(v, lo, hi):
    return v if lo <= v <= hi else None


def need(*a):
    return all(x is not None for x in a)


def scale(x, v):
    return x * v // 100


def vt(t):
    return t if 0 < t <= 100000 else None


def tpsm(t, v):
    t = vt(t)
    return None if t is None else t * v


def rng(v):
    return 0 <= v <= 1000000


def rel(v, k):
    v = n(v, 25, 200)
    return None if v is None else v * k // 100


def stage(v):
    if n(v, 50, 150) is None:
        return None
    return (-3 if v < 100 else -1 if v > 100 else -2) & M


def opt(fn):
    def w(f, v):
        try:
            return fn(f, v)
        except TypeError:   # None 이 산술에 들어간 경우 = Rust 의 ? 조기 반환
            return None
    return w


PORT = {k: opt(x) for k, x in {
    'tower_hits': lambda f, v: f.reg(0) * n(v, 1, 10),
    'chase_ratio_rdx': lambda f, v: f.reg(2) * n(v, 1, 20),
    'chase_ratio_rcx': lambda f, v: f.reg(1) * n(v, 1, 20),
    'radius_square': lambda f, v: n(v, 1000, 65535) ** 2,
    'radius_square_minus_one': lambda f, v: n(v, 1000, 65535) ** 2 - 1,
    'trade_hp_divisor': lambda f, v: f.reg(1) // n(v, 1, 32),
    'heal_rsi': lambda f, v: f.reg(1) + f.reg(6) * n(v, 0, 10),
    'heal_r8': lambda f, v: f.reg(1) + f.reg(8) * n(v, 0, 10),
    'shield_r8': lambda f, v: f.reg(8) * (n(v, 0, 10) + 1),
    'shield_rsi': lambda f, v: f.reg(6) * (n(v, 0, 10) + 1),
    'heal_rdx_rcx': lambda f, v: f.reg(2) + f.reg(1) * n(v, 0, 10),
    'shield_rcx': lambda f, v: f.reg(1) * (n(v, 0, 10) + 1),
    'buff_arrival_ticks': lambda f, v: None if f.reg(1) & 1 else (f.reg(1) // 2) * n(v, 1, 30),
    'isolated_percent': lambda f, v: (f.reg(0) * 0xcccccccccccccccd & M) * n(v, 0, 1000),
    'front_ally_distance_score': lambda f, v: (f.reg(0) // 4000) * n(v, 1, 400) // 100,
    'front_ally_cap': lambda f, v: n(v, 1, 400) * 25 // 100,
    'front_ally_total_cap': lambda f, v: n(v, 1, 400) * 60 // 100,
    'wave_risk_divisor': lambda f, v: tdiv(s64(f.reg(13)), n(v, 1, 32)),
    'risk_retreat_high': lambda f, v: n(v, 1, 50) * 24 // 10,
    'risk_retreat_pressure': lambda f, v: f.reg(0) * (n(v, 1, 50) * 2),
    'unseen_risk_divisor': lambda f, v: tdiv(s64(f.reg(7)), n(v, 1, 32)),
    'relocate_inner': lambda f, v: rel(v, 8000),
    'relocate_mid': lambda f, v: rel(v, 30000),
    'relocate_self_near': lambda f, v: rel(v, 24000),
    'relocate_self_far': lambda f, v: rel(v, 48000),
    'relocate_mid_result': lambda f, v: f.reg(2) + f.reg(1) + rel(v, 30000),
    'relocate_outer_result': lambda f, v: f.reg(2) + f.reg(1) + rel(v, 70000),
    'candidate_move_budget': lambda f, v: f.reg(0) * (n(v, 0, 120) + 2),
    'candidate_move_budget_rcx': lambda f, v: f.reg(1) * (n(v, 0, 120) + 2),
    'forward_credit_percent': lambda f, v: f.reg(7) * n(v, 0, 100) // 100,
    'move_penalty_negative': lambda f, v: -n(v, 0, 1000),
    'move_penalty_result': lambda f, v: f.reg(12) - n(v, 0, 1000),
    'gank_deadline': lambda f, v: tpsm(f.mem(5, 0x130), v) if rng(v) else None,
    'call_early': lambda f, v: tpsm(f.reg(1), v) if rng(v) else None,
    'call_diagnostic': lambda f, v: tpsm(f.reg(1), v) if rng(v) else None,
    'call_general': lambda f, v: tpsm(f.reg(0), v) if rng(v) else None,
    'same_lane_retry': lambda f, v: (None if f.reg(0) & 1 or vt(f.reg(0) // 2) is None else tpsm(f.reg(0) // 2, v)) if rng(v) else None,
    'pregank_hp': lambda f, v: 35 * v // 100 if rng(v) else None,
    'pregank_radius': lambda f, v: (200000 * v // 100) ** 2 if rng(v) else None,
    'defense_risk': lambda f, v: 35 * v // 100 if rng(v) else None,
    'defense_long': lambda f, v: 25 * v // 100 if rng(v) else None,
    'defense_short': lambda f, v: 10 * v // 100 if rng(v) else None,
    'gank_recheck': lambda f, v: tpsm(f.reg(6), v) if rng(v) else None,
    'roam_extra': lambda f, v: tpsm(f.reg(1), v) if rng(v) else None,
    'lane_range_a': lambda f, v: f.reg(2) * (v + 2) if rng(v) else None,
    'lane_range_b': lambda f, v: f.reg(3) * (v + 2) if rng(v) else None,
    'lane_range_c': lambda f, v: f.reg(14) * (v + 2) if rng(v) else None,
    'superiority_hp': lambda f, v: 20 + 5 * v if rng(v) else None,
    'superiority_count': lambda f, v: 1 + v if rng(v) else None,
    'lane_call_cap': lambda f, v: tpsm(f.mem2(7, 8, 0x12f8), v) if rng(v) else None,
    'lane_call_deadline_half': lambda f, v: tpsm(f.reg(0), v // 2) if rng(v) and v % 2 == 0 else None,
    'mia_global': lambda f, v: vt(f.reg(1)) * v // 100 if rng(v) else None,
    'mia_memory': lambda f, v: f.reg(2) + vt(f.reg(1)) * 4 * v // 100 if rng(v) else None,
    'mia_hp': lambda f, v: 60 * v // 100 if rng(v) else None,
    'mia_radius': lambda f, v: (((170000 * 100 // v) ** 2 >> 8) - 1) if rng(v) and v else None,
    'relay_limit': lambda f, v: vt(f.mem(9, 0x12f8)) * v // 100 if rng(v) else None,
    'collapse_metric': lambda f, v: -(3000 * 100 // n(v, 50, 150)),
    'collapse_stage': lambda f, v: stage(v),
    'radius_squared': lambda f, v: n(v, 1000, 2000000) ** 2,
    'pressure_stage': lambda f, v: max(scale(3, n(v, 50, 150)), 1),
    'pressure_metric': lambda f, v: scale(2000, n(v, 50, 150)),
    'pressure_count': lambda f, v: max(scale(2, n(v, 50, 150)), 1),
    'defense_threat_hp': lambda f, v: n(v, 11, 100) - 10,
    'structure_rcx': lambda f, v: f.reg(1) * n(v, 0, 100),
    'structure_rdx': lambda f, v: f.reg(2) * n(v, 0, 100),
    'respawn_seconds': lambda f, v: f.mem2(15, 8, 0x12f8) * n(v, 1, 120),
    'damage_increment': lambda f, v: f.reg(0) * (n(v, 1, 10) - 1),
    'finish_seconds3': lambda f, v: f.reg(15) * n(v, 1, 30),
    'finish_increment2': lambda f, v: f.reg(15) * (n(v, 1, 30) - 2),
    'camp_survival_margin': lambda f, v: scale(f.mem(5, -0x10), n(v, 0, 300)),
    'camp_arrival_margin': lambda f, v: scale(f.reg(1) >> 1, n(v, 0, 300)),
    'tower_radius_high': lambda f, v: scale(140000, n(v, 50, 150)) ** 2 + 1,
    'tower_radius_low': lambda f, v: scale(100000, n(v, 50, 150)) ** 2 + 1,
    'tower_minion_gate': lambda f, v: None if n(v, 50, 150) is None else (2 if v < 100 else 0 if v > 100 else 1),
    'tower_wave_limit': lambda f, v: 300000 // n(v, 50, 150),
    'negative': lambda f, v: -v,
    'square_shift8': lambda f, v: (v * v) >> 8 if v >= 0 else None,
    'ops_radius_squared': lambda f, v: n(v, 1000, 2000000) ** 2,
    'ops_camp_tps': lambda f, v: f.mem(1, 0x12f8) * n(v, 0, 10),
    'ops_gather_square': lambda f, v: n(v, 1000, 2000000) ** 2,
    'combat_rsi_add': lambda f, v: f.reg(12) + n(v, 0, 1000),
    'combat_negative': lambda f, v: -n(v, 0, 1000),
}.items()}
# 원본에 없던 검사를 더하는 창 — 기본값 등가가 성립할 이유가 없다(값 미설정이면 dispatch 가 원본으로 보낸다)
EXEMPT = {'tactical_roam_self_hp'}


def _tps_rel(ptr, out, mul):
    """창 앞 명령이 메모리의 TPS 로 만든 레지스터 관계를 표본에 재현한다(창 안에서는 안 보이는 입력)."""
    def w(regs, rd):
        regs[ptr] &= ~7
        regs[out] = rd(rd(regs[ptr] + 8) + 0x12f8) * mul
    return w


def _gank_rel(regs, rd):
    regs[0] = rd(RBP + 0x130) * 4   # 0.7.2 TPS = [rbp+0x130] (0xf6e3af 저장 · 0.7.1 [rbp+0xb8] · 0x10d0a73 lea rax,[rax*4])


RELATION = {
    'gank_deadline': _gank_rel,
    'respawn_seconds': _tps_rel(15, 0, 5),    # lea rax,[rax+rax*4] 뒤 창이 ×3
    'lane_call_cap': _tps_rel(7, 1, 2),       # add rcx,rcx 뒤 창이 ×5
}
# 음성 대조: 이전 버전 콜백 입력(0.7.2 에서 틀린 값). 0.7.2 에 입력이 바뀐 것은 0.7.1 값 · 그 밖은 0.7.0 값.
MUTATE = {
    'risk_retreat_pressure': lambda f, v: f.reg(1) * (n(v, 1, 50) * 2),
    'wave_risk_divisor': lambda f, v: tdiv(s64(f.reg(14)), n(v, 1, 32)),
    'unseen_risk_divisor': lambda f, v: tdiv(s64(f.reg(12)), n(v, 1, 32)),
    'forward_credit_percent': lambda f, v: f.reg(14) * n(v, 0, 100) // 100,
    'move_penalty_result': lambda f, v: f.reg(13) - n(v, 0, 1000),
    'gank_recheck': lambda f, v: tpsm(f.reg(15), v),
    'lane_range_b': lambda f, v: f.reg(12) * (v + 2),
    'lane_range_c': lambda f, v: f.reg(7) * (v + 2),
    'finish_seconds3': lambda f, v: f.reg(14) * n(v, 1, 30),
    'finish_increment2': lambda f, v: f.reg(14) * (n(v, 1, 30) - 2),
    'camp_survival_margin': lambda f, v: scale(f.mem(5, -0x20), n(v, 0, 300)),
    'gank_deadline': lambda f, v: tpsm(f.mem(5, 0xb8), v),
    'combat_rsi_add': lambda f, v: f.reg(6) + n(v, 0, 1000),
}


def parse_defaults():
    """src/tactics.rs 표 순서 → (id, default, kind). 기본32 v("id",...) · 표 뒤 GENERATED EXTRAS 까지 같은 순서."""
    s = (MOD / 'src' / 'tactics.rs').read_text(encoding='utf-8')
    body = s[s.index('pub static TACTICS'):s.index('pub const N:')]
    out = []
    for m in re.finditer(r'^\s*(v|t)\("([a-z0-9_]+)".*$', body, re.M):
        if m.group(1) == 't':
            out.append((m.group(2), None, 'Toggle'))
            continue
        nums = re.search(r',\s*[PT],\s*(-?\d+),\s*(-?\d+),\s*(-?\d+),\s*(-?\d+),\s*(-?\d+)\),?\s*$', m.group(0))
        out.append((m.group(2), int(nums.group(1)), 'Value'))
    return out


def transform(tf, v, fr, maxv):
    if tf.startswith('Tf::Compute'):
        name = tf.split('::')[-1].rstrip(')')
        if name in EXEMPT:
            return 'EXEMPT'
        r = PORT[name](fr, v)
        return None if r is None else r & M
    m = re.match(r'Tf::Add\((-?\d+)\)', tf)
    if m:
        x = v + int(m.group(1))
    elif tf == 'Tf::Sq1':
        x = v * v + 1
    else:
        m = re.match(r'Tf::Map \{ on: (-?\d+), off: (-?\d+) \}', tf)
        x = int(m.group(2)) if v == 0 else int(m.group(1))
    return None if x < 0 or x > maxv else x


def abs_jump(t):
    return bytes([0xff, 0x25, 0, 0, 0, 0]) + struct.pack('<Q', t)


def relocated(p, dynamic):
    """micro.rs relocated() 를 그대로 옮긴 것(base = 0x140000000)."""
    out = bytearray()
    at = 0
    win = bytes(p['win'])
    brs = {b['off']: b for b in p['branches']}
    if dynamic:
        if p['dynamic']:
            out += bytes(p['dynamic'])
        else:
            b = brs[0]
            m = re.match(r'Tf::Map \{ on: (-?\d+), off: (-?\d+) \}', p['fields'][0]['tf'])
            off = int(m.group(2))
            tgt = BASE + int(b['target_rva'], 16) if isinstance(b['target_rva'], str) else BASE + b['target_rva']
            out += bytes([0x9c, 0x80, 0x7c, 0x24, 0xf8, off, 0x75, 15, 0x9d]) + abs_jump(tgt)
            out += bytes([0x9d, 0x70 | (b['cc'] ^ 1), 14]) + abs_jump(tgt)
        at = p['first_len']
    while at < len(win):
        b = brs.get(at)
        if b:
            tgt = BASE + (int(b['target_rva'], 16) if isinstance(b['target_rva'], str) else b['target_rva'])
            if b['cc'] is not None:
                out += bytes([0x70 | (b['cc'] ^ 1), 14])
            out += abs_jump(tgt)
            at += b['len']
        else:
            out.append(win[at])
            at += 1
    out += abs_jump(BASE + p['rva'] + len(win))
    return bytes(out)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--samples', type=int, default=40)
    ap.add_argument('--only')
    ap.add_argument('--mutate', action='store_true')
    ap.add_argument('--out')
    a = ap.parse_args()
    if a.mutate:
        PORT.update({k: opt(v) for k, v in MUTATE.items()})
    import track_update as T
    ver, sha, patches = T.parse_sites((MOD / 'src' / 'sites.rs').read_text(encoding='utf-8'))
    tacs = parse_defaults()
    assert len(tacs) == 100, len(tacs)
    print('sites %s · 창 %d · 전술표 %d' % (ver, len(patches), len(tacs)))

    rnd = random.Random(20261104)
    low = bytearray(LOW)
    vals = [rnd.randrange(1000, 100000) & ~7 for _ in range(4096)]
    pat = b''.join(struct.pack('<Q', x) for x in vals)
    for o in range(0, LOW, len(pat)):
        low[o:o + len(pat)] = pat
    low = bytes(low)

    def rd(addr):
        if 0 <= addr <= LOW - 8:
            return struct.unpack_from('<Q', low, addr)[0]
        raise TypeError   # 못 읽음 = Rust 의 None

    u = Uc(UC_ARCH_X86, UC_MODE_64)
    u.mem_map(0, LOW)
    u.mem_write(0, low)
    u.mem_map(CODE, 0x10000)
    u.mem_map(BASE, 0x2000000)
    st = {}

    def on_code(uc, addr, size, _):
        if addr >= BASE:
            st['exit'] = addr
            uc.emu_stop()

    def on_write(uc, access, addr, size, value, _):
        st['w'].append((addr, size))

    u.hook_add(UC_HOOK_CODE, on_code)
    u.hook_add(UC_HOOK_MEM_WRITE, on_write)

    def run(code, regs, flags, value):
        st.clear()
        st['w'] = []
        st['exit'] = None
        u.mem_write(CODE, code + b'\xf4' * 16)
        u.ctl_flush_tb()   # 같은 주소에 코드를 바꿔 쓰므로 번역 캐시를 비운다(안 비우면 이전 창을 실행한다)
        u.mem_write(RSP - 16, struct.pack('<Q', value))
        for r, x in zip(REGS, regs):
            u.reg_write(r, x)
        u.reg_write(UC_X86_REG_EFLAGS, flags)
        err = None
        try:
            u.emu_start(CODE, CODE + 0x8000, count=300)
        except UcError as e:
            err = str(e)
        res = [u.reg_read(r) for r in REGS]
        fl = u.reg_read(UC_X86_REG_EFLAGS) & FLAGS
        wr = {}
        for addr, size in st['w']:
            if RSP - 0x100 <= addr < RSP:   # rsp 아래 = 스크래치(값 슬롯·push)
                continue
            for i in range(size):
                wr[addr + i] = bytes(u.mem_read(addr + i, 1))[0]
        ex = st['exit']
        for addr, size in st['w']:
            if addr < LOW:
                u.mem_write(addr, low[addr:addr + size])
        u.mem_write(RSP - 16, low[RSP - 16:RSP - 8])
        return res, fl, wr, ex, err

    bad, thin, exempt, passed = [], [], [], 0
    report = []
    for p in patches:
        if a.only and a.only not in p['name']:
            continue
        tac = tacs[p['tactic']]
        f0 = p['fields'][0]
        tf = f0['tf']
        maxv = (1 << 63) - 1 if 'MAX' in str(f0['max']) else int(f0['max'], 0)
        if tac[2] == 'Toggle':
            dv = 1   # 토글 기본 = 켜짐(원본) — 불일치면 보고에 나온다
        else:
            dv = tac[1]
        co, cd = relocated(p, False), relocated(p, True)
        good = skipped = 0
        first = None
        for i in range(a.samples * 4):
            if good >= a.samples:
                break
            regs = [rnd.randrange(2, 100000) & ~(rnd.getrandbits(1)) for _ in REGS]
            regs[4], regs[5] = RSP, RBP
            flags = 0x202 | (rnd.getrandbits(12) & FLAGS)
            cname = tf.split('::')[-1].rstrip(')')
            if cname in RELATION:
                RELATION[cname](regs, rd)
            fr = F(regs, rd)
            val = transform(tf, dv, fr, maxv)
            if val == 'EXEMPT':
                exempt.append(p['name'])
                good = -1
                break
            if val is None:
                skipped += 1
                continue
            A = run(co, regs, flags, val)
            B = run(cd, regs, flags, val)
            if A[4] and B[4]:
                st['lasterr'] = A[4]
                skipped += 1
                continue
            good += 1
            diff = []
            for k in range(16):
                if A[0][k] != B[0][k]:
                    diff.append('%s %#x≠%#x' % (RN[k], A[0][k], B[0][k]))
            if A[3] != B[3]:
                diff.append('출구 %s≠%s' % (A[3] and hex(A[3] - BASE), B[3] and hex(B[3] - BASE)))
            if A[2] != B[2]:
                diff.append('메모리쓰기 %d≠%d바이트' % (len(A[2]), len(B[2])))
            if A[4] != B[4]:
                diff.append('오류 %s≠%s' % (A[4], B[4]))
            fdiff = A[1] != B[1]
            if diff or fdiff:
                first = first or dict(regs={RN[k]: hex(regs[k]) for k in range(16)}, value=hex(val), diff=diff,
                                      flags=(hex(A[1]), hex(B[1])) if fdiff else None)
                if diff:
                    first['hard'] = True
                first['n'] = first.get('n', 0) + 1
        if good == -1:
            continue
        rec = dict(name=p['name'], tactic=tac[0], tf=tf.split('::')[-1].rstrip(')'), default=dv, samples=good, skipped=skipped, err=st.get('lasterr'))
        if first:
            rec['first'] = first
            bad.append(rec)
        elif good < min(a.samples, 8):
            thin.append(rec)
        else:
            passed += 1
        report.append(rec)
    hard = [b for b in bad if b['first'].get('hard')]
    soft = [b for b in bad if not b['first'].get('hard')]
    print('일치 %d · 불일치(값·출구·메모리) %d · 플래그만 다름 %d · 표본 부족 %d · 제외 %d' % (passed, len(hard), len(soft), len(thin), len(exempt)))
    for b in hard:
        print('✗', b['name'], b['tactic'], b['tf'], 'v=%s' % b['default'], '%d/%d' % (b['first']['n'], b['samples']), b['first']['diff'][:3], 'val', b['first']['value'])
    for b in soft:
        print('△', b['name'], b['tactic'], b['tf'], 'flags', b['first']['flags'], '%d/%d' % (b['first']['n'], b['samples']))
    for b in thin:
        print('?', b['name'], b['tactic'], b['tf'], 'samples', b['samples'], 'skipped', b['skipped'], b.get('err'))
    if a.out:
        Path(a.out).write_text(json.dumps(report, ensure_ascii=False, indent=1) + '\n', encoding='utf-8', newline='\n')
    sys.exit(1 if hard else 2 if thin else 0)


if __name__ == '__main__':
    main()
