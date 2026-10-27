#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""상세전술 패치데이 추적기 — 전술 100·창 218 이 새 exe 에서 어디로 갔고 어떻게 바뀌었는지(이동만·기본값 변경·재배치·사본·로직·소실) 판정한다.

사용(다른 에이전트가 이대로 부른다 — 바꾸지 말 것):
  python -X utf8 tools/track_update.py snapshot --exe <현행exe> --ver 0.6.5 --pkl <fnidx.pkl> --cg <cg.pkl>
         --out tools/patchday/baseline_0.6.5.json
  python -X utf8 tools/track_update.py track --base <baseline.json> --old-exe <구exe> --new-exe <신exe> --new-ver <라벨>
         --old-pkl P --new-pkl P --old-cg P --new-cg P --out <출력폴더>
  종료코드(track): 0 = 전 전술 자동확정(AUTO) · 1 = 검토 필요 항목 있음(정상) · 2 = 실행 오류

snapshot = 현행 targets(knobs/subjects/extra_*) + src/sites.rs·tactics.rs·presentation.rs + tools/visible_extras.json 을 읽어
  전술별·창별 기준선(창 바이트·즉치↔기본값 식·소비 명령·owner 지문·패닉 Location·콜그래프·±24 정규화 토큰·프롤로그·
  Src 근거 명령·Caller 콜사이트·창 안 분기 대상)을 만든다. targets 원문도 통째로 넣는다(track 의 초안 원천).
track = 기준선의 각 창을 신 exe 에서 찾는다.
  ① owner 대응 = 패닉 Location 집합(aidiff.pair · 모듈별 행 시프트) 1순위 + repin.build_match(fnidx skel·콜그래프) 교차 확인
  ② owner 안 위치 = 명령 토큰열 정렬(difflib: 레지스터 지운 키 → 꼴만 남긴 키로 틈 메움)
  ③ 창 판정: SAME / REG_RENAME / VALUE_CHANGED / COPIES / LOGIC_CHANGED / SITE_MISSING / OWNER_MISSING (+근거 문자열)
  ④ 전술 집계: 창 전부 SAME + Caller 콜사이트·분기 대상까지 대응 = AUTO, 아니면 REVIEW(사유). VALUE_CHANGED 는 절대 AUTO 아님.
  ⑤ 출력: track.md(사람용) · track.json(전량) · review_queue.json · draft_targets/(같은 스키마 초안, REVIEW 엔 "review" 키)
     ★tools/targets/ 는 절대 덮어쓰지 않는다. 초안을 사람이 판정·승격한 뒤 gen_sites.py --ver <라벨> (review 키 남으면 중단).

판정 어휘(창 단위 · 우선순위 순):
  OWNER_MISSING  owner 함수를 신 exe 에서 못 찾음
  SITE_MISSING   owner 는 찾았으나 창 위치 후보 0 (또는 Src::Caller 반환 콜사이트 소실)
  LOGIC_CHANGED  명령 꼴 변화·소비 조건코드 변화·창 주변 비교 구조 변화·분기 대상 불일치(창 안 + 창 밖 소비 Jcc)·
                 owner aidiff EDITED + 창 주변 변화
  COPIES         같은 창 꼴의 사본 수, 또는 즉치를 담은 값 조각(창 첫 frag_n 명령)의 사본 수가 owner 안에서 달라짐
                 (컴파일러가 비교를 복제하며 뒤 분기 꼴을 바꾸는 1→N 분기 — 0.6.4→0.6.5 objective_hunt_exit_hp_30 1→3)
  VALUE_CHANGED  꼴은 같고 즉치가 다름 → 게임 기본값 구→신(tf 역산)
  REG_RENAME     꼴·즉치 같고 레지스터/스택 슬롯/필드 변위/프레임/인코딩/콜사이트가 다름 → 구→신 사상과 새 Src 제안
  SAME           창 바이트(rel 변위 제외)·프레임·Src 레지스터/슬롯·Caller 콜사이트 동일 — 유일한 자동확정

한계(README 참조): owner 밖으로 인라인된 사본은 못 센다 · Src 제안은 정렬 다수결이라 반드시 사람/런타임 확인 ·
  reference_windows(참고창)·notes 안의 RVA 는 추적하지 않는다.
"""
import argparse
import collections
import copy
import datetime
import difflib
import hashlib
import json
import math
import os
import re
import sys
import traceback
from pathlib import Path

TOOLS = Path(__file__).resolve().parent
ROOT = TOOLS.parent                       # tfm2_detail_tactics
MIGD = ROOT.parent / 'MIG'
sys.path.insert(0, str(TOOLS))
sys.path.insert(0, str(MIGD))

import aidiff        # noqa: E402  (★import 시 sys.stdout 을 교체한다 — 출력 전에 import)
import midpin        # noqa: E402
import repin         # noqa: E402
import gen_sites as GS   # noqa: E402

from capstone import Cs, CS_ARCH_X86, CS_MODE_64, CS_GRP_JUMP, CS_GRP_CALL, CS_GRP_RET   # noqa: E402
from capstone.x86 import X86_OP_IMM, X86_OP_MEM, X86_OP_REG   # noqa: E402

TOOL_VER = 1
BASE = 0x140000000
M64 = (1 << 64) - 1
CTX = 24                 # 창 앞뒤 정규화 토큰 수
MD = Cs(CS_ARCH_X86, CS_MODE_64)
MD.detail = True
PRIORITY = ['OWNER_MISSING', 'SITE_MISSING', 'LOGIC_CHANGED', 'COPIES', 'VALUE_CHANGED', 'REG_RENAME']
EXTRA_PARTS = ('lane', 'combat', 'ops', 'verified')
FLAG_WRITERS = set('cmp test add sub and or xor inc dec neg sar shl shr sal rol ror rcl rcr bt bts btr btc imul mul '
                   'adc sbb bsf bsr popcnt lzcnt tzcnt ucomiss ucomisd comiss comisd'.split())

# ───────────────────────────────────────────────────────── 레지스터 정규화
NORM = {}
for _r, _subs in {'rax': 'eax ax al ah', 'rcx': 'ecx cx cl ch', 'rdx': 'edx dx dl dh', 'rbx': 'ebx bx bl bh',
                  'rsp': 'esp sp spl', 'rbp': 'ebp bp bpl', 'rsi': 'esi si sil', 'rdi': 'edi di dil'}.items():
    NORM[_r] = _r
    for _s in _subs.split():
        NORM[_s] = _r
for _i in range(8, 16):
    for _suf in ('', 'd', 'w', 'b'):
        NORM['r%d%s' % (_i, _suf)] = 'r%d' % _i


def nreg(name):
    return NORM.get(name, name)


def flag_consumer(mn):
    return (mn.startswith('j') and mn != 'jmp') or mn.startswith('set') or mn.startswith('cmov') or mn in ('adc', 'sbb')


# ───────────────────────────────────────────────────────── 명령 토큰
class I:
    """명령 1개 + 정규화 키.
    kf = 레지스터·즉치·변위 전부(분기 대상·RIP 변위는 재배치라 제외)  kn = kf 에서 레지스터명만 지움(크기·종류는 유지)
    ks = 꼴만(니모닉+피연산자 종류·크기·메모리 기준 종류)            kc = 사본 키(레지스터·스택 변위 지움, 즉치·필드 변위 유지)"""
    __slots__ = ('rva', 'size', 'b', 'mn', 'asm', 'ops', 'kf', 'kn', 'ks', 'kc', 'regs', 'rd', 'wr', 'rel', 'mems', 'imms')


def mk(ins):
    x = I()
    x.rva = ins.address - BASE
    x.size = ins.size
    x.b = bytes(ins.bytes)
    x.mn = ins.mnemonic
    br = bool(ins.group(CS_GRP_JUMP) or ins.group(CS_GRP_CALL))
    f, n, s, c, regs, mems, imms, ops = [], [], [], [], [], [], [], []
    rel = None
    for op in ins.operands:
        if op.type == X86_OP_REG:
            nm = ins.reg_name(op.reg)
            t = 'r%d' % op.size
            f.append(nm); n.append(t); s.append(t); c.append(t)
            regs.append(nreg(nm))
            ops.append(('r', nreg(nm), op.size, nm))
        elif op.type == X86_OP_IMM:
            if br:
                rel = op.imm - BASE
                f.append('T'); n.append('T'); s.append('T'); c.append('T')
                ops.append(('T', rel))
            else:
                v = op.imm & M64
                f.append('%#x' % v); n.append('%#x' % v); s.append('i%d' % op.size); c.append('%#x' % v)
                imms.append(v)
                ops.append(('i', v, op.size))
        elif op.type == X86_OP_MEM:
            m = op.mem
            base = ins.reg_name(m.base) if m.base else ''
            idx = ins.reg_name(m.index) if m.index else ''
            seg = (ins.reg_name(m.segment) + ':') if m.segment else ''
            sc = m.scale if idx else 0
            if base == 'rip':
                bk, dsp = 'rip', 'D'
            else:
                bk = {'rsp': 'sp', 'rbp': 'bp', '': '-'}.get(base, 'g')
                dsp = '%#x' % (m.disp & M64)
            xi = 'x' if idx else ''
            f.append('%sm%d[%s+%s*%d+%s]' % (seg, op.size, base, idx, sc, dsp))
            n.append('%sm%d[%s+%s*%d+%s]' % (seg, op.size, bk, xi, sc, dsp))
            s.append('%sm%d[%s%s*%d]' % (seg, op.size, bk, xi, sc))
            c.append('%sm%d[%s%s*%d+%s]' % (seg, op.size, bk, xi, sc, 'S' if bk in ('sp', 'bp') else dsp))
            if bk == 'g':
                regs.append(nreg(base))
            if idx:
                regs.append(nreg(idx))
            mems.append((bk, nreg(base) if base else '', m.disp, op.size, nreg(idx) if idx else '', sc))
            ops.append(('m', bk, nreg(base) if base else '', nreg(idx) if idx else '', sc, m.disp, op.size, seg))
        else:
            f.append('?'); n.append('?'); s.append('?'); c.append('?')
            ops.append(('?',))
    x.kf = x.mn + ' ' + ','.join(f)
    x.kn = x.mn + ' ' + ','.join(n)
    x.ks = x.mn + ' ' + ','.join(s)
    x.kc = x.mn + ' ' + ','.join(c)
    x.regs = regs
    x.mems = mems
    x.imms = imms
    x.ops = ops
    x.rel = rel
    x.asm = ('%s %#x' % (x.mn, rel)) if (br and rel is not None) else ('%s %s' % (x.mn, ins.op_str)).strip()
    try:
        rd, wr = ins.regs_access()
        x.rd = frozenset(nreg(ins.reg_name(r)) for r in rd)
        x.wr = frozenset(nreg(ins.reg_name(r)) for r in wr)
    except Exception:
        x.rd = x.wr = frozenset()
    return x


def is_rel(x):
    return x.rel is not None


def gs_asm(x):
    """gen_sites.py 의 knobs 'asm' 표기(분기는 RVA 로 정규화)."""
    return x.asm


def hexsp(b):
    return ' '.join('%02x' % v for v in b)


def sha256_file(p):
    h = hashlib.sha256()
    with open(p, 'rb') as f:
        for blk in iter(lambda: f.read(1 << 20), b''):
            h.update(blk)
    return h.hexdigest().upper()


# ───────────────────────────────────────────────────────── exe 보기
class _Sub:
    def __init__(s, img, funcs):
        s.funcs = funcs
        s.code = img.code


class View:
    """exe 1개: aidiff.Img(.pdata·Location census) + midpin.Img(fnidx·콜그래프)."""

    def __init__(s, exe, pkl, cg):
        s.path = str(exe)
        s.A, s.census = aidiff.load(s.path, 'game-ai')
        s.sha = hashlib.sha256(s.A.data).hexdigest().upper()
        s.M = midpin.Img(s.path, pkl, cg)
        s._loc = None
        s._locset = {}
        s._ins = {}
        s._idx = {}

    def owner(s, rva):
        return s.A.owner(rva)

    def read(s, rva, n):
        return s.A.code(rva, n)

    def insns(s, fn):
        if fn not in s._ins:
            code = s.A.code(fn[0], fn[1] - fn[0])
            s._ins[fn] = [mk(x) for x in MD.disasm(code, BASE + fn[0])]
            s._idx[fn] = {x.rva: i for i, x in enumerate(s._ins[fn])}
        return s._ins[fn]

    def index(s, fn, rva):
        s.insns(fn)
        return s._idx[fn].get(rva)

    def locs(s, fn):
        """함수의 패닉 Location 지문(aidiff.census 항목). AI 계층 밖이면 즉석 계산, 없으면 None."""
        if fn in s.census:
            return s.census[fn]
        if fn not in s._locset:
            if s._loc is None:
                s._loc = aidiff.find_locations(s.A)
            r = aidiff.census(_Sub(s.A, [fn]), s._loc, '')
            s._locset[fn] = r.get(fn)
        return s._locset[fn]

    def find_key(s, fn, key):
        ins = s.insns(fn)
        L = len(key)
        if not L:
            return []
        out = []
        k0 = key[0]
        for p in range(0, len(ins) - L + 1):
            if ins[p].kc == k0 and all(ins[p + t].kc == key[t] for t in range(1, L)):
                out.append(p)
        return out


def lea_rip_targets(img, want):
    """.text 의 `REX.W lea r, [rip+disp32]` 중 대상이 want(RVA 집합)인 명령 RVA 목록 (Location 역참조용)."""
    import numpy as np
    sec = next(x for x in img.secs if x[0] == '.text')
    _nm, va, sz, pr = sec
    buf = np.frombuffer(img.data, dtype=np.uint8, count=min(sz, len(img.data) - pr), offset=pr)
    if not want or buf.size < 8:
        return []
    pos = np.nonzero(((buf[:-7] & 0xF8) == 0x48) & (buf[1:-6] == 0x8D) & ((buf[2:-5] & 0xC7) == 0x05))[0]
    b = [buf[pos + 3 + i].astype(np.int64) for i in range(4)]
    rel = b[0] | (b[1] << 8) | (b[2] << 16) | (b[3] << 24)
    rel = np.where(rel >= 1 << 31, rel - (1 << 32), rel)
    tgt = va + pos + 7 + rel
    sel = np.isin(tgt, np.array(sorted(want), dtype=np.int64))
    return [int(va + x) for x in pos[sel]]


def loc_summary(ent):
    if not ent:
        return None
    return {'top': ent['top'], 'n': len(ent['lines']),
            'lmods': {m: sorted([list(x) for x in v]) for m, v in sorted(ent['lmods'].items())}}


def prologue(ins):
    """push 목록·sub rsp·rbp 설정 → frame(=진입 rsp - 사이트 rsp)·rbp_off(rbp - 사이트 rsp)."""
    pushes, sub, cur, rbp_e, last_eax, last = [], 0, 0, None, None, -1
    push_phase = True
    for i, x in enumerate(ins[:24]):
        if x.mn == 'push' and x.ops and x.ops[0][0] == 'r' and push_phase:
            pushes.append(x.ops[0][1]); cur -= 8; last = i
            continue
        push_phase = False
        if x.mn == 'sub' and x.ops and x.ops[0][0] == 'r' and x.ops[0][1] == 'rsp' and not sub:
            if len(x.ops) > 1 and x.ops[1][0] == 'i':
                sub = x.ops[1][1]
            elif len(x.ops) > 1 and x.ops[1][0] == 'r' and x.ops[1][1] == 'rax' and last_eax is not None:
                sub = last_eax
            cur -= sub; last = i
            continue
        if x.mn == 'mov' and len(x.ops) == 2 and x.ops[0][0] == 'r' and x.ops[0][3] == 'eax' and x.ops[1][0] == 'i' and not sub:
            last_eax = x.ops[1][1]
            continue
        if x.mn == 'lea' and rbp_e is None and len(x.ops) == 2 and x.ops[0][0] == 'r' and x.ops[0][1] == 'rbp' \
                and x.ops[1][0] == 'm' and x.ops[1][1] == 'sp':
            rbp_e = cur + x.ops[1][5]; last = i
            continue
        if x.mn == 'mov' and rbp_e is None and len(x.ops) == 2 and x.ops[0][0] == 'r' and x.ops[0][1] == 'rbp' \
                and x.ops[1][0] == 'r' and x.ops[1][1] == 'rsp':
            rbp_e = cur; last = i
            continue
        if x.mn in ('ret', 'jmp') or (x.mn.startswith('j') and x.mn != 'jmp'):
            break
    frame = 8 * len(pushes) + sub
    return {'pushes': pushes, 'sub': sub, 'frame': frame,
            'rbp_off': (rbp_e + frame) if rbp_e is not None else None,
            'toks': [x.kf for x in ins[:last + 1]]}


def pro_key(p):
    return (tuple(p['pushes']), p['sub'], p['frame'], p['rbp_off'])


def slot_of(mem, pro):
    bk, _base, disp = mem[0], mem[1], mem[2]
    if bk == 'sp':
        return disp
    if bk == 'bp' and pro.get('rbp_off') is not None:
        return disp + pro['rbp_off']
    return None


def consumer(ins, k, frag_n):
    """창 값 명령(조각 마지막 명령)의 소비 의미."""
    vi = ins[k + frag_n - 1]
    out = {'value_asm': vi.asm}
    if flag_consumer(vi.mn):
        out.update(kind='self_jcc', mn=vi.mn, ks=vi.ks, asm=vi.asm, dist=0, rva=hex(vi.rva))
        return out
    if vi.mn in ('cmp', 'test') or (vi.ops and vi.ops[0][0] != 'r'):
        for t in range(k + frag_n, min(len(ins), k + frag_n + 16)):
            x = ins[t]
            if flag_consumer(x.mn):
                out.update(kind='flags', mn=x.mn, ks=x.ks, asm=x.asm, dist=t - (k + frag_n), rva=hex(x.rva))
                return out
            if x.mn in FLAG_WRITERS:
                out.update(kind='flags_clobbered', mn=x.mn, ks=x.ks, asm=x.asm, dist=t - (k + frag_n), rva=hex(x.rva))
                return out
            if x.mn in ('jmp', 'ret', 'call'):
                out.update(kind='flow', mn=x.mn, ks=x.ks, asm=x.asm, dist=t - (k + frag_n), rva=hex(x.rva))
                return out
        out.update(kind='none', mn='', ks='', asm='', dist=-1)
        return out
    dest = vi.ops[0][1]
    for t in range(k + frag_n, min(len(ins), k + frag_n + 24)):
        x = ins[t]
        if dest in x.rd:
            out.update(kind='reg', reg=dest, mn=x.mn, ks=x.ks, asm=x.asm, dist=t - (k + frag_n), rva=hex(x.rva))
            return out
        if dest in x.wr:
            out.update(kind='overwritten', reg=dest, mn=x.mn, ks=x.ks, asm=x.asm, dist=t - (k + frag_n), rva=hex(x.rva))
            return out
        if x.mn in ('jmp', 'ret'):
            out.update(kind='flow', reg=dest, mn=x.mn, ks=x.ks, asm=x.asm, dist=t - (k + frag_n), rva=hex(x.rva))
            return out
    out.update(kind='none', reg=dest, mn='', ks='', asm='', dist=-1)
    return out


VOLATILE = frozenset(('rax', 'rcx', 'rdx', 'r8', 'r9', 'r10', 'r11'))
ARGREGS = ('rcx', 'rdx', 'r8', 'r9')


def writes(x, X, barrier=False):
    """x 가 X 를 쓰나 — call 은 휘발 레지스터를 덮는다고 본다(capstone 은 모델링 안 함).
    barrier=True = call 과 대상 지점 사이에 jmp/ret 가 있어 직선 흐름으로 안 닿음 → call 덮어쓰기 무시."""
    return X in x.wr or (x.mn == 'call' and X in VOLATILE and not barrier)


def cmpish(x):
    return x.mn in ('cmp', 'test') or flag_consumer(x.mn)


def cons_key(c):
    return (c.get('kind'), c.get('mn'), c.get('ks'))


def cmp_struct(ins, k, n_w, span=8):
    return [x.ks for x in ins[max(0, k - span):k + n_w + span] if x.mn in ('cmp', 'test') or flag_consumer(x.mn)]


def ctx_tokens(ins, k, n_w):
    b = ins[max(0, k - CTX):k]
    a = ins[k + n_w:k + n_w + CTX]
    return b, a


# ───────────────────────────────────────────────────────── 소스 파싱
def rust_args(s):
    out, i, n = [], 0, len(s)
    while i < n:
        c = s[i]
        if c in ' \t,':
            i += 1
            continue
        if c == '"':
            j, buf = i + 1, []
            while s[j] != '"':
                if s[j] == '\\':
                    buf.append(s[j + 1]); j += 2
                    continue
                buf.append(s[j]); j += 1
            out.append(''.join(buf))
            i = j + 1
            continue
        j = i
        while j < n and s[j] != ',':
            j += 1
        tok = s[i:j].strip()
        out.append(int(tok) if re.fullmatch(r'-?\d+', tok) else tok)
        i = j
    return out


def parse_tactics(text):
    rows = []
    for m in re.finditer(r'^\s*([vt])\(("[a-z0-9_]+".*)\),\s*$', text, re.M):
        a = rust_args(m.group(2))
        if m.group(1) == 'v':
            tid, knob, name, desc, scope, dflt, low, high, mn, mx = a
            kind = 'Value'
        else:
            tid, knob, name, desc, scope = a
            dflt, low, high, mn, mx = 1, 0, 1, 0, 1
            kind = 'Toggle'
        rows.append(dict(index=len(rows), id=tid, knob=knob, name=name, desc=desc,
                         scope={'P': 'Player', 'T': 'Team'}[scope], kind=kind,
                         default=dflt, low=low, high=high, min=mn, max=mx))
    n = int(re.search(r'pub const N: usize = (\d+);', text).group(1))
    if len(rows) != n:
        raise SystemExit('tactics.rs 파싱 %d != N %d' % (len(rows), n))
    return rows


def parse_presentation(text):
    cats = []
    for name, body in re.findall(r'Category \{ name: "([^"]+)", ids: &\[(.*?)\] \}', text, re.S):
        cats.append((name, re.findall(r'"([a-z0-9_]+)"', body)))
    seg = text.split('// END EXTRA VISIBILITY', 1)[1]
    seg = seg[seg.index('match id {') + len('match id {'):]
    seg = seg[:seg.index('\n    }')]
    rules, default = {}, 31
    for line in seg.splitlines():
        line = line.strip().rstrip(',')
        if '=>' not in line:
            continue
        lhs, rhs = [x.strip() for x in line.split('=>', 1)]
        expr = re.sub(r'matches!\(col,\s*([0-9 |]+)\)',
                      lambda mm: 'col in (%s,)' % ','.join(mm.group(1).split('|')), rhs)
        expr = expr.replace('true', 'True').replace('false', 'False')
        mask = sum(1 << c for c in range(5) if eval(expr, {}, {'col': c}))
        if lhs == '_':
            default = mask
        else:
            for nm in re.findall(r'"([a-z0-9_]+)"', lhs):
                rules[nm] = mask
    return cats, rules, default


PAT_RE = re.compile(r'^\s*Patch \{ name: "([^"]+)", tactic: (\d+), rva: (0x[0-9a-f]+), win: &\[([^\]]*)\], '
                    r'fields: &\[(.*?)\], branches: &\[(.*?)\], src: (.*), team_chk: (.*), first_len: (\d+), '
                    r'dynamic: &\[([^\]]*)\] \},\s*$')
FIELD_RE = re.compile(r'Field \{ off: (\d+), width: (\d+), tf: (.*?), max: ([^ ]+) \}')
BR_RE = re.compile(r'Branch \{ off: (\d+), len: (\d+), cc: (Some\((\d+)\)|None), target_rva: (0x[0-9a-f]+) \}')


def bytelist(s):
    return bytes(int(x, 16) for x in re.findall(r'0x([0-9a-f]{1,2})', s))


def parse_sites(text):
    ver = re.search(r'pub const GAME_VERSION: &str = "([^"]+)";', text).group(1)
    sha = re.search(r'pub const EXE_SHA256: &str = "([0-9A-Fa-f]+)";', text).group(1)
    out, func = [], None
    for line in text.splitlines():
        mc = re.match(r'^\s*// (.*) \(함수 (0x[0-9a-f]+)\)\s*$', line)
        if mc:
            func = (mc.group(1), int(mc.group(2), 16))
            continue
        m = PAT_RE.match(line)
        if not m:
            continue
        fields = [dict(off=int(a), width=int(b), tf=c, max=d) for a, b, c, d in FIELD_RE.findall(m.group(5))]
        brs = [dict(off=int(a), len=int(b), cc=(int(d) if d else None), target_rva=e) for a, b, _c, d, e in BR_RE.findall(m.group(6))]
        out.append(dict(name=m.group(1), tactic=int(m.group(2)), rva=int(m.group(3), 16), win=bytelist(m.group(4)),
                        fields=fields, branches=brs, src_rs=m.group(7), team_chk_rs=m.group(8),
                        first_len=int(m.group(9)), dynamic=bytelist(m.group(10)),
                        comment_asm=func[0] if func else None, comment_func=func[1] if func else None))
        func = None
    return ver, sha, out


def load_targets(ver, tdir=None):
    tdir = Path(tdir) if tdir else TOOLS / 'targets'
    docs = {}
    names = {'knobs': 'knobs_%s.json' % ver, 'subjects': 'subjects_%s.json' % ver}
    for p in EXTRA_PARTS:
        names['extra_' + p] = 'extra_%s_%s.json' % (p, ver)
    for key, nm in names.items():
        docs[key] = json.loads((tdir / nm).read_text(encoding='utf-8'))
    return docs, names


def tf_add_from_rs(tf):
    m = re.fullmatch(r'Tf::Add\((-?\d+)\)', tf)
    return int(m.group(1)) if m else None


# ───────────────────────────────────────────────────────── 즉치 ↔ 기본값
def imm_inverse(tf, v):
    """창 즉치 v → 기본값 (tf 역변환). tf = ('add',n)/('sq1',)/('map',on,off)/('calc',name)."""
    if tf[0] == 'add':
        return v - tf[1], 'imm - %d' % tf[1] if tf[1] else 'imm'
    if tf[0] == 'sq1':
        r = math.isqrt(max(0, v - 1))
        return (r if r * r + 1 == v else None), 'sqrt(imm-1)'
    if tf[0] == 'map':
        return (1 if v == tf[1] else (0 if v == tf[2] else None)), 'toggle on=%d off=%d' % (tf[1], tf[2])
    return None, 'calc %s (역산 불가)' % tf[1]


def tf_of_window(w):
    """창의 기본값 변환. knobs = targets 의 tf 문자열, extras = tf dict."""
    tf = w['tf_raw']
    if isinstance(tf, dict):
        if 'calc' in tf:
            return ('calc', tf['calc'])
        if tf.get('kind') == 'Add':
            return ('add', int(tf['value']))
        if tf.get('kind') == 'Sq1':
            return ('sq1',)
        if tf.get('kind') == 'Map':
            return ('map', int(tf['on']), int(tf['off']))
        return ('calc', json.dumps(tf))
    t = GS.tf_of(tf)
    if t[0] == 'v':
        return ('add', t[1])
    if t[0] == 'sq1':
        return ('sq1',)
    if t[0] == 'toggle':
        return ('map', t[1][0], t[1][1])
    return ('calc', str(tf))


def expected_imm(tf, default):
    if tf[0] == 'add':
        return default + tf[1]
    if tf[0] == 'sq1':
        return default * default + 1
    if tf[0] == 'map':
        return tf[1]
    return None


def find_value_operand(ins, k, frag_n, want, width=None):
    """조각 안에서 값(want)을 담은 즉치 피연산자 (명령 index, 피연산자 index)."""
    if want is None:
        return None
    for t in range(frag_n - 1, -1, -1):
        x = ins[k + t]
        for oi, op in enumerate(x.ops):
            if op[0] == 'i':
                bits = 8 * (width or op[2] or 8)
                if (op[1] & ((1 << bits) - 1)) == (want & ((1 << bits) - 1)):
                    return (t, oi)
            if op[0] == 'm' and op[1] in ('g', '-') and want and (op[5] & M64) == (want & M64):
                return (t, oi)
    return None


def operand_value(x, oi):
    op = x.ops[oi] if oi < len(x.ops) else None
    if op is None:
        return None
    if op[0] == 'i':
        v = op[1]
        bits = 8 * (op[2] or 8)
        v &= (1 << bits) - 1
        return v
    if op[0] == 'm':
        return op[5]
    return None


def jcc_byte(x):
    """Map 토글(짧은 Jcc) 의 첫 바이트."""
    return x.b[0] if x.b else None


# ───────────────────────────────────────────────────────── Src 분해·변환
def fmt_like(old, new):
    old = str(old)
    if old.startswith('0x') or old.startswith('-0x'):
        return '%#x' % new if new >= 0 else '-%#x' % (-new)
    return '%x' % new if new >= 0 else '-%x' % (-new)


class SrcResult:
    def __init__(s):
        s.items = []          # (path, kind, old, new, status, note)

    def add(s, path, kind, old, new, status, note):
        s.items.append(dict(path=path, kind=kind, old=old, new=new, status=status, note=note))

    def status(s):
        st = [i['status'] for i in s.items]
        if any(x == 'UNKNOWN' for x in st):
            return 'UNKNOWN'
        if any(x == 'RENAMED' for x in st):
            return 'RENAMED'
        return 'SAME'


def xform_src(s, ctx, path, res):
    """src 원문(JSON) → 신 버전 제안 원문. ctx 가 성분별(reg/slot) 대응을 판정해 res 에 남긴다."""
    if s is None:
        return None
    if isinstance(s, dict):
        kind = s.get('kind')
        n = copy.deepcopy(s)
        if kind == 'cache_player':
            n['operation'] = ctx.reg(s['operation'], path + '.operation', res)
            n['entity'] = ctx.reg(s['entity'], path + '.entity', res)
            n['team_stack'] = fmt_like(s['team_stack'], ctx.stack(int(s['team_stack'], 16), path + '.team_stack', res))
            return n
        if kind == 'cache_slot':
            for fld in ('cache', 'team', 'pos', 'entity'):
                if fld in s:
                    n[fld] = xform_src(s[fld], ctx, path + '.' + fld, res)
            return n
        if kind == 'caller':
            ro = int(s['return_off'], 16)
            ro_new = ctx.return_off(ro, path + '.return_off', res)
            n['return_off'] = fmt_like(s['return_off'], ro_new)
            cases = []
            for ci, c in enumerate(s['cases']):
                cc = copy.deepcopy(c)
                rr = ctx.caller(int(c['ret_rva'], 16), path + '.cases[%d].ret_rva' % ci, res)
                if rr is not None and rr.get('new_ret') is not None:
                    cc['ret_rva'] = '%#x' % rr['new_ret']
                inner = ctx.inner(rr, ro, ro_new)
                cc['src'] = xform_src(c['src'], inner, path + '.cases[%d].src' % ci, res)
                cases.append(cc)
            n['cases'] = cases
            return n
        res.add(path, 'unknown_dict', json.dumps(s), json.dumps(s), 'UNKNOWN', '알 수 없는 src 형식')
        return n
    p = s.split(':')
    if p[0] == 'const':
        return s
    if p[0] == 'reg':
        return 'reg:' + ctx.reg(p[1], path, res)
    if p[0] == 'stack':
        return 'stack:' + fmt_like(p[1], ctx.stack(int(p[1], 16), path, res))
    if p[0] == 'mem':
        if p[1] in ('rbp', 'rsp'):
            if p[1] == 'rsp':
                return 'mem:rsp:' + fmt_like(p[2], ctx.stack(int(p[2], 16), path, res))
            r = ctx.rbpmem(int(p[2], 16), path, res)
            if r is None:          # rbp 가 프레임 포인터가 아님 → 일반 레지스터
                return 'mem:%s:%s' % (ctx.reg('rbp', path, res), p[2])
            return 'mem:rbp:' + fmt_like(p[2], r)
        return 'mem:%s:%s' % (ctx.reg(p[1], path, res), p[2])
    if p[0] == 'stackmem':
        return 'stackmem:%s:%s' % (fmt_like(p[1], ctx.stack(int(p[1], 16), path, res)), p[2])
    if p[0] == 'mem2':
        return 'mem2:%s:%s:%s' % (ctx.reg(p[1], path, res), p[2], p[3])
    res.add(path, 'unknown', s, s, 'UNKNOWN', '알 수 없는 src 형식')
    return s


def src_components(s, path='src'):
    """기준선용: src 원문을 성분(레지스터·슬롯)으로 풀어 근거 명령을 찾게 한다."""
    out = []
    if s is None:
        return out
    if isinstance(s, dict):
        k = s.get('kind')
        if k == 'cache_player':
            out += [(path + '.operation', 'reg', s['operation']), (path + '.entity', 'reg', s['entity']),
                    (path + '.team_stack', 'stack', int(s['team_stack'], 16))]
        elif k == 'cache_slot':
            for fld in ('cache', 'team', 'pos', 'entity'):
                if fld in s:
                    out += src_components(s[fld], path + '.' + fld)
        elif k == 'caller':
            out.append((path + '.return_off', 'return_off', int(s['return_off'], 16)))
            for ci, c in enumerate(s['cases']):
                out.append((path + '.cases[%d].ret_rva' % ci, 'ret_rva', int(c['ret_rva'], 16)))
                for pth, kd, v in src_components(c['src'], path + '.cases[%d].src' % ci):
                    out.append((pth, 'inner_' + kd, v))
        return out
    p = s.split(':')
    if p[0] == 'reg':
        out.append((path, 'reg', p[1]))
    elif p[0] == 'stack':
        out.append((path, 'stack', int(p[1], 16)))
    elif p[0] == 'mem':
        if p[1] == 'rsp':
            out.append((path, 'stack', int(p[2], 16)))
        elif p[1] == 'rbp':
            out.append((path, 'rbpmem', int(p[2], 16)))
        else:
            out.append((path, 'reg', p[1]))
    elif p[0] == 'stackmem':
        out.append((path, 'stack', int(p[1], 16)))
    elif p[0] == 'mem2':
        out.append((path, 'reg', p[1]))
    return out


# ───────────────────────────────────────────────────────── 정렬
class Align:
    """구/신 함수 명령열 대응(old idx → new idx). ① kn(레지스터 지운 키) LCS 블록(≥3) ② 블록 사이 틈을 ks(꼴) 로 메움."""

    def __init__(s, O_ins, N_ins, lo, hi, nlo, nhi):
        s.m, s.tag = {}, {}
        a = [x.kn for x in O_ins[lo:hi]]
        b = [x.kn for x in N_ins[nlo:nhi]]
        sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
        small = len(a) <= 8 or len(b) <= 8
        blocks = [bk for bk in sm.get_matching_blocks() if bk.size and (bk.size >= 3 or small)]
        pi, pj = 0, 0
        for bk in blocks + [difflib.Match(len(a), len(b), 0)]:
            if bk.a > pi and bk.b > pj:
                s._gap(O_ins, N_ins, lo + pi, lo + bk.a, nlo + pj, nlo + bk.b)
            for t in range(bk.size):
                s.m[lo + bk.a + t] = nlo + bk.b + t
                s.tag[lo + bk.a + t] = 'F'
            pi, pj = bk.a + bk.size, bk.b + bk.size
        s.range = (lo, hi, nlo, nhi)

    def _gap(s, O_ins, N_ins, i0, i1, j0, j1):
        sa = [x.ks for x in O_ins[i0:i1]]
        sb = [x.ks for x in N_ins[j0:j1]]
        if i1 - i0 == j1 - j0:
            for t in range(i1 - i0):
                if sa[t] == sb[t]:
                    s.m[i0 + t] = j0 + t
                    s.tag[i0 + t] = 'G'
            return
        if (i1 - i0) * (j1 - j0) > 4000000:
            return
        sm = difflib.SequenceMatcher(None, sa, sb, autojunk=False)
        for bk in sm.get_matching_blocks():
            if bk.size >= 2:
                for t in range(bk.size):
                    s.m[i0 + bk.a + t] = j0 + bk.b + t
                    s.tag[i0 + bk.a + t] = 'C'


def arg_spills(ins, pro):
    """진입 인자 레지스터(rcx/rdx/r8/r9)를 아직 덮기 전에 스택 슬롯에 저장하는 명령 → {슬롯: 인자} · {인자: [슬롯]}.
    직선 스캔(제어흐름 무시) — 프롤로그 근처 인자 저장(spill)을 잡는 용도. call 은 휘발 레지스터를 덮는다고 본다."""
    nwrite = collections.Counter()
    for x in ins:               # 슬롯 재사용 배제: 함수 전체에서 한 번만 기록되는 슬롯만 인자 저장으로 인정
        if x.ops and x.ops[0][0] == 'm' and x.ops[0][1] in ('sp', 'bp') and x.mn not in ('cmp', 'test') and x.mems:
            sl = slot_of(x.mems[0], pro)
            if sl is not None:
                nwrite[sl] += 1
    written, by_slot, by_arg = set(), {}, collections.defaultdict(list)
    for x in ins:
        if x.mn == 'mov' and len(x.ops) == 2 and x.ops[0][0] == 'm' and x.ops[0][1] in ('sp', 'bp') \
                and x.ops[1][0] == 'r' and x.ops[1][2] == 8 and x.ops[1][1] in ARGREGS and x.ops[1][1] not in written:
            sl = slot_of(x.mems[0], pro)
            if sl is not None and nwrite[sl] == 1:
                by_slot.setdefault(sl, x.ops[1][1])
                by_arg[x.ops[1][1]].append(sl)
        written |= x.wr
        if x.mn == 'call':
            written |= VOLATILE
        if written >= set(ARGREGS):
            break
    return by_slot, by_arg


def slot_votes(al, O_ins, N_ins, pro_o, pro_n):
    """사이트 rsp 기준 슬롯 구→신 표. ① 정렬된 같은 꼴 명령쌍(같은 변위 = 동일 표, 다른 변위 = 꼴 정렬 표)
    ② 진입 인자 저장(spill): 구 슬롯이 인자 X 를 저장하는데 신 쪽 짝 명령이 X 가 아닌 걸 저장하면 그 표는 버리고,
       신 owner 에서 X 를 저장하는 슬롯이 하나뿐이면 그 슬롯에 3표(다른 슬롯일 때만 — 동일 판정을 새로 만들지 않는다)."""
    v = collections.defaultdict(collections.Counter)
    so, _ao = arg_spills(O_ins, pro_o)
    _sn, an = arg_spills(N_ins, pro_n)
    for i, j in al.m.items():
        a, b = O_ins[i], N_ins[j]
        if a.ks != b.ks or len(a.mems) != len(b.mems):
            continue
        if a.mn == 'mov' and len(a.ops) == 2 and a.ops[0][0] == 'm' and a.ops[1][0] == 'r' and a.mems:
            sa0 = slot_of(a.mems[0], pro_o)
            if sa0 in so and so[sa0] == a.ops[1][1] and b.ops[1][1] != a.ops[1][1]:
                continue           # 구 = 인자 X 저장, 신 짝 = 다른 레지스터 저장 → 꼴만 같은 엉뚱한 짝
        for ma, mb in zip(a.mems, b.mems):
            sa, sb = slot_of(ma, pro_o), slot_of(mb, pro_n)
            if sa is not None and sb is not None:
                v[sa][sb] += 1
    for sa, X in so.items():
        cand = sorted(set(an.get(X, [])))
        if len(cand) == 1 and cand[0] != sa:
            v[sa][cand[0]] += 3
    return v


class SiteCtx:
    """한 지점(창 첫 명령 또는 콜사이트)에서 레지스터·슬롯의 구→신 대응을 판정."""

    def __init__(s, O_ins, N_ins, k, j, n_w, al, pro_o, pro_n, svotes):
        s.O, s.N, s.k, s.j, s.n_w, s.al = O_ins, N_ins, k, j, n_w, al
        s.pro_o, s.pro_n, s.sv = pro_o, pro_n, svotes
        s.frame_same = pro_n is not None and pro_key(pro_o) == pro_key(pro_n)
        s.ctx_same = False
        if j is not None and N_ins is not None:
            ob, oa = ctx_tokens(O_ins, k, n_w)
            nb, na = ctx_tokens(N_ins, j, n_w)
            s.ctx_same = [x.kf for x in ob + oa] == [x.kf for x in nb + na] and \
                [x.kf for x in O_ins[k:k + n_w]] == [x.kf for x in N_ins[j:j + n_w]]
        s.slot_identity = all(len(c) == 1 and next(iter(c)) == o for o, c in svotes.items()) if svotes is not None else False

    def mapi(s, t):
        if s.j is None:
            return None
        if s.k <= t < s.k + s.n_w:
            return s.j + (t - s.k)
        return s.al.m.get(t) if s.al else None

    def ev_reg(s, X):
        if s.j is None:
            return 'UNKNOWN', X, '신 위치 없음'
        if X in ('rsp',):
            return 'SAME', X, ''
        lw, readers, barrier = None, [], False
        for t in range(s.k - 1, -1, -1):
            x = s.O[t]
            if writes(x, X, barrier):
                lw = t
                break
            if x.mn in ('jmp', 'ret'):
                barrier = True       # 이 앞의 call 은 직선 흐름으로 창에 닿지 않는다
            if X in x.rd and len(readers) < 3 and t >= s.k - 600:
                readers.append(t)
        ev = ([lw] if lw is not None else []) + readers + \
            [t for t in range(s.k, min(len(s.O), s.k + s.n_w)) if X in s.O[t].rd or X in s.O[t].wr]
        votes, una = collections.Counter(), 0
        nlw = None
        for t in ev:
            nt = s.mapi(t)
            if nt is None or nt >= len(s.N):
                una += 1
                continue
            a, b = s.O[t], s.N[nt]
            if a.kn != b.kn or len(a.regs) != len(b.regs):
                una += 1
                continue
            pos = [i for i, r in enumerate(a.regs) if r == X]
            if pos:
                for i in pos:
                    votes[b.regs[i]] += 1
            elif (X in a.wr and X in b.wr) or (X in a.rd and X in b.rd):
                votes[X] += 1          # 암묵 피연산자(같은 명령이 같은 레지스터를 암묵 사용)
            else:
                una += 1
            if t == lw:
                nlw = nt

        def redef(Y, start):
            lb = max([t for t in range(max(0, start), s.j) if s.N[t].mn in ('jmp', 'ret')], default=-1)
            for t in range(max(0, start), s.j):
                if writes(s.N[t], Y, t < lb):
                    return t
            return None
        if votes:
            top = votes.most_common()
            Y = top[0][0]
            if len(top) == 1:
                st0 = (nlw + 1) if nlw is not None else (0 if lw is None else None)
                if st0 is not None:
                    t = redef(Y, st0)
                    if t is not None:
                        return 'UNKNOWN', X, '신 %s 가 창 전 %#x 에서 재정의 `%s`' % (Y, s.N[t].rva, s.N[t].asm)
                return ('SAME' if Y == X else 'RENAMED'), Y, '근거 %d표(%s→%s)%s' % (
                    sum(votes.values()), X, Y, (' · 정렬 안 된 근거 %d' % una) if una else '')
            return 'UNKNOWN', Y, '표 갈림 %s (다수 %s 를 제안)' % (dict(votes), Y)
        if lw is None:
            t = redef(X, 0)
            if t is not None:
                return 'UNKNOWN', X, '구 owner 는 진입값 %s 를 쓰는데 신 owner 는 창 전 %#x 에서 기록 `%s`' % (X, s.N[t].rva, s.N[t].asm)
            if X in ARGREGS:
                return 'SAME', X, '진입 인자 레지스터 %s(ABI 고정) · 신 owner 도 창 전 미기록' % X
            if s.ctx_same:
                return 'SAME', X, '호출자 상속값 · 창 ±%d 명령 동일' % CTX
            return 'UNKNOWN', X, '호출자 상속값 %s(callee-saved) — 호출자 확인 필요' % X
        if s.ctx_same and lw >= s.k - CTX:
            return 'SAME', X, '창 ±%d 명령 동일' % CTX
        return 'UNKNOWN', X, '근거 명령 대응 없음(정의 %#x `%s`)' % (s.O[lw].rva, s.O[lw].asm)

    def cmp_changed(s):
        """창 ±8 명령의 비교·분기(cmp/test/jcc/setcc/cmov) 꼴 열이 정렬 기준으로 달라졌나."""
        lo, hi = max(0, s.k - 8), min(len(s.O), s.k + s.n_w + 8)
        olds = [t for t in range(lo, hi) if cmpish(s.O[t])]
        mm = [x for x in (s.mapi(t) for t in range(lo, hi)) if x is not None]
        if not mm:
            return True
        nlo, nhi = min(mm), max(mm) + 1
        news = [t for t in range(nlo, nhi) if cmpish(s.N[t])]
        if [s.O[t].ks for t in olds] != [s.N[t].ks for t in news]:
            return True
        return any(s.mapi(t) is None or s.N[s.mapi(t)].ks != s.O[t].ks for t in olds)

    def ev_slot(s, d):
        """사이트 rsp 기준 슬롯 d."""
        if s.j is None:
            return 'UNKNOWN', d, '신 위치 없음'
        c = s.sv.get(d) if s.sv is not None else None
        if c:
            top = c.most_common()
            if len(top) == 1 or top[0][1] >= 2 * top[1][1]:
                nd = top[0][0]
                if nd != d and top[0][1] < 2:
                    # 다른 슬롯 표는 꼴 정렬(G/C)에서만 나온다 — 1표는 엉뚱한 짝일 수 있다(0.6.5→0.6.4 리허설
                    # lane_retreat_final_50@0xf0a51e: 인자 r9 저장 슬롯이 rdx 저장 슬롯과 꼴로 짝지어져 틀린 제안)
                    return 'UNKNOWN', d, '슬롯 근거 1표뿐(꼴 정렬) %+#x→%+#x 후보 — 약한 근거, 사람 확인' % (d, nd)
                return ('SAME' if nd == d else 'RENAMED'), nd, '슬롯 %d표 %+#x→%+#x%s' % (
                    top[0][1], d, nd, ('' if len(top) == 1 else ' (소수표 %s)' % top[1:4]))
            return 'UNKNOWN', d, '슬롯 표 갈림 %s' % top[:4]
        if s.frame_same and s.slot_identity:
            return 'SAME', d, '프레임 동일·이동 슬롯 없음'
        return 'UNKNOWN', d, '슬롯 근거 없음'


class Level:
    """프레임 체인의 한 단: 0 = 창 owner(사이트=창 첫 명령), 1.. = Src::Caller 의 호출자(사이트=call 명령)."""

    def __init__(s, S, ofn, nfn):
        s.S, s.ofn, s.nfn = S, ofn, nfn


class ChainCtx:
    """xform_src 용 판정기. 모든 src 성분은 '창 사이트 rsp' 기준이다(micro.rs Frame::reg(4)).
    [rsp+d] 가 어느 단의 프레임(지역 슬롯 / push 저장 레지스터 / 반환주소)에 떨어지는지 풀어 단별 대응으로 사상한다."""

    def __init__(s, T, levels, ok=True):
        s.T, s.L, s.ok = T, levels, ok

    def _bases(s):
        bo, bn = [0], [0]
        for L in s.L[:-1]:
            if L is None or L.S is None or L.S.pro_n is None:
                return bo, None
            bo.append(bo[-1] + L.S.pro_o['frame'] + 8)
            bn.append(bn[-1] + L.S.pro_n['frame'] + 8)
        return bo, bn

    def _broken(s):
        return any(L is None or L.S is None or L.S.pro_n is None for L in s.L)

    def _shortcut(s):
        return len(s.L) > 1 and s.ok and not s._broken() and all(L.S.frame_same for L in s.L)

    def reg(s, X, path, res):
        if s._shortcut():
            res.add(path, 'reg', X, X, 'SAME', '프레임·콜사이트 체인 동일')
            return X
        S0 = s.L[0].S
        st, Y, note = S0.ev_reg(X)
        if st == 'UNKNOWN' and len(s.L) > 1 and s.L[1] is not None and s.L[1].S is not None:
            st2, Y2, note2 = s.L[1].S.ev_reg(X)
            if st2 != 'UNKNOWN':
                st, Y, note = st2, Y2, 'caller 상속 · ' + note2
        res.add(path, 'reg', X, Y, st, note)
        return Y

    def _slot(s, d):
        """사이트 rsp 기준 d → (상태, 신 d, 근거)."""
        if s._shortcut():
            return 'SAME', d, '프레임·콜사이트 체인 동일'
        if s._broken():
            return 'UNKNOWN', d, '체인 중 신 대응 없는 단'
        bo, bn = s._bases()
        for i, L in enumerate(s.L):
            S = L.S
            Fo, Fn = S.pro_o['frame'], S.pro_n['frame']
            if bo[i] <= d < bo[i] + Fo:
                c = d - bo[i]
                P = S.pro_o['pushes']
                if c >= Fo - 8 * len(P) and (Fo - c) % 8 == 0:
                    R = P[(Fo - c) // 8 - 1]
                    if i + 1 < len(s.L):
                        st, R2, note = s.L[i + 1].S.ev_reg(R)
                        note = '단%d 저장 %s ← 단%d %s' % (i, R, i + 1, note)
                    else:
                        st, R2 = ('SAME' if S.frame_same else 'UNKNOWN'), R
                        note = '단%d 저장 %s(호출자 문맥 없음 — 호출자가 같은 레지스터라고 가정)' % (i, R)
                    P2 = S.pro_n['pushes']
                    if R2 not in P2:
                        return 'UNKNOWN', d, '신 단%d 이 %s 를 저장하지 않음 %s' % (i, R2, P2)
                    nd = bn[i] + Fn - 8 * (P2.index(R2) + 1)
                    if st == 'SAME' and nd != d:
                        st = 'RENAMED'
                    return st, nd, note
                st, nc, note = S.ev_slot(c)
                return st, bn[i] + nc, '단%d 지역 슬롯 %s' % (i, note)
            if d == bo[i] + Fo:
                return ('SAME' if bn[i] + Fn == d else 'RENAMED'), bn[i] + Fn, '단%d 반환주소 슬롯' % i
        last = len(s.L) - 1
        S = s.L[last].S
        top_o = bo[last] + S.pro_o['frame'] + 8
        top_n = bn[last] + S.pro_n['frame'] + 8
        # 마지막 단 반환주소 위 = 그 단의 진입 인자 영역(홈 공간 +8..+0x20 · 스택 인자 +0x28..) — 진입 rsp 기준 ABI 고정
        c = d - top_o + 8
        if all(L.S.frame_same for L in s.L):
            return 'SAME', d, '단%d 진입 인자 영역 +%#x — 체인 프레임 동일' % (last, c)
        return 'RENAMED', d - top_o + top_n, '단%d 진입 인자 영역 +%#x(ABI 고정·시그니처 불변 가정) — 프레임 변화로 사이트 기준 재계산' % (last, c)

    def stack(s, d, path, res):
        st, nd, note = s._slot(d)
        res.add(path, 'stack', d, nd, st, note)
        return nd

    def rbpmem(s, d, path, res):
        """[rbp+d]: rbp 를 설정한 가장 가까운 단의 프레임 포인터로 본다. 아무 단도 안 세우면 None(일반 레지스터)."""
        if s._broken():
            res.add(path, 'rbpmem', d, d, 'UNKNOWN', '체인 중 신 대응 없는 단')
            return d
        bo, bn = s._bases()
        for i, L in enumerate(s.L):
            po, pn = L.S.pro_o, L.S.pro_n
            if po.get('rbp_off') is None:
                continue
            if pn.get('rbp_off') is None:
                res.add(path, 'rbpmem', d, d, 'UNKNOWN', '신 단%d 에 rbp 프레임 없음' % i)
                return d
            st, nd, note = s._slot(bo[i] + po['rbp_off'] + d)
            new = nd - bn[i] - pn['rbp_off']
            res.add(path, 'rbpmem', d, new, st, '%s (단%d rbp_off %#x→%#x)' % (note, i, po['rbp_off'], pn['rbp_off']))
            return new
        return None

    def return_off(s, ro, path, res):
        if s._broken():
            res.add(path, 'return_off', ro, ro, 'UNKNOWN', '체인 중 신 대응 없는 단')
            return ro
        bo, bn = s._bases()
        i = len(s.L) - 1
        S = s.L[i].S
        exp_o = bo[i] + S.pro_o['frame']
        new = bn[i] + S.pro_n['frame']
        if exp_o != ro:
            res.add(path, 'return_off', ro, ro, 'SAME' if all(L.S.frame_same for L in s.L) else 'UNKNOWN',
                    '기준선 체인 frame %#x ≠ return_off %#x (프롤로그 해석 불일치)' % (exp_o, ro))
            return ro
        st = 'SAME' if new == ro and S.frame_same else ('RENAMED' if new != ro else 'UNKNOWN')
        res.add(path, 'return_off', ro, new, st, '단%d frame %#x→%#x pushes %s→%s' % (
            i, S.pro_o['frame'], S.pro_n['frame'], S.pro_o['pushes'], S.pro_n['pushes']))
        return new

    def caller(s, ret, path, res):
        L = s.L[-1]
        if L is None or L.S is None or L.nfn is None:
            res.add(path, 'ret_rva', '%#x' % ret, None, 'UNKNOWN', '상위 단 대응 없음')
            return None
        rr = s.T.track_ret(ret, L.ofn, L.nfn)
        res.add(path, 'ret_rva', '%#x' % ret, ('%#x' % rr['new_ret']) if rr.get('new_ret') is not None else None,
                rr['status'], rr['note'])
        return rr

    def inner(s, rr, ro, ro_new):
        if rr is None or rr.get('site') is None:
            return ChainCtx(s.T, s.L + [None], False)
        return ChainCtx(s.T, s.L + [Level(rr['site'], rr['cfo'], rr['cfn'])], s.ok and rr['status'] == 'SAME')


# ───────────────────────────────────────────────────────── 기준선 (snapshot)
def collect_windows(ver, sites_text, targets, tactics):
    """sites.rs PATCHES ↔ targets 항목 연결."""
    gver, gsha, patches = parse_sites(sites_text)
    if gver != ver:
        raise SystemExit('sites.rs GAME_VERSION %s ≠ --ver %s' % (gver, ver))
    kn = {k['id']: (ki, k) for ki, k in enumerate(targets['knobs']['knobs'])}
    ex = {}
    for part in EXTRA_PARTS:
        for ti, t in enumerate(targets['extra_' + part]['tactics']):
            ex[t['audit_id'].lower()] = ('extra_' + part, ti, t)
    subj = targets['subjects']['sites']
    wins = []
    for p in patches:
        tag, vr = p['name'].split('@')
        if tag.startswith('extra_'):
            src_file, ti, t = ex[tag[len('extra_'):]]
            si = next(i for i, s in enumerate(t['sites']) if s['rva'] == vr)
            s = t['sites'][si]
            w = dict(source=src_file, t_index=ti, s_index=si, tf_raw=s['tf'], src_raw=s['src'], team_chk_raw=s.get('team_chk'),
                     value_rva=int(vr, 16), subj_key=None)
        else:
            ki, k = kn[tag]
            si = next(i for i, s in enumerate(k['sites']) if s['rva'] == vr)
            s = k['sites'][si]
            sj = subj[vr]
            w = dict(source='knobs', t_index=ki, s_index=si, tf_raw=s['tf'], src_raw=sj['src'], team_chk_raw=sj.get('team_chk'),
                     value_rva=int(vr, 16), subj_key=vr)
        w.update(name=p['name'], tactic=p['tactic'], tactic_id=tactics[p['tactic']]['id'], rva=p['rva'], win=p['win'],
                 fields=p['fields'], branches=p['branches'], src_rs=p['src_rs'], team_chk_rs=p['team_chk_rs'],
                 first_len=p['first_len'], dynamic=p['dynamic'], comment_func=p['comment_func'])
        wins.append(w)
    return gsha, wins


def window_geometry(V, w):
    fn = V.owner(w['rva'])
    if fn is None:
        raise SystemExit('%s: owner 없음' % w['name'])
    ins = V.insns(fn)
    k = V.index(fn, w['rva'])
    if k is None:
        raise SystemExit('%s: 창 시작 %#x 이 명령 경계 아님' % (w['name'], w['rva']))
    n, ln = 0, 0
    while ln < len(w['win']):
        ln += ins[k + n].size
        n += 1
    if ln != len(w['win']):
        raise SystemExit('%s: 창 길이 %d 가 명령 경계와 안 맞음' % (w['name'], len(w['win'])))
    off, frag_n = 0, 0
    for t in range(n):
        if off < w['first_len']:
            frag_n += 1
        off += ins[k + t].size
    return fn, ins, k, n, frag_n


def window_value_info(w, ins, k, n_w, frag_n, tac):
    """즉치 원값 ↔ 기본값 식."""
    tf = tf_of_window(w)
    out = {'tf': list(tf), 'fields': []}
    imms = []
    for t in range(n_w):
        for oi, op in enumerate(ins[k + t].ops):
            if op[0] == 'i':
                imms.append({'insn': t, 'op': oi, 'value': op[1], 'in_fragment': t < frag_n})
    out['window_imms'] = imms
    winb = b''.join(ins[k + t].b for t in range(n_w))
    if w['source'] == 'knobs':
        for f in w['fields']:
            cur = int.from_bytes(winb[f['off']:f['off'] + f['width']], 'little')
            off, t_at = 0, None
            for t in range(n_w):
                if off <= f['off'] < off + ins[k + t].size:
                    t_at = t
                    break
                off += ins[k + t].size
            vo = None
            if t_at is not None:
                x = ins[k + t_at]
                for oi, op in enumerate(x.ops):
                    if op[0] == 'i' and (op[1] & ((1 << (8 * f['width'])) - 1)) == cur:
                        vo = (t_at, oi)
                        break
                if vo is None and tf[0] == 'map':
                    vo = (t_at, -1)       # 토글 = Jcc 첫 바이트
            dflt, expr = imm_inverse(tf, cur)
            out['fields'].append({'off': f['off'], 'width': f['width'], 'tf_rs': f['tf'], 'imm': cur, 'operand': vo,
                                  'default_expr': expr, 'default': dflt})
    else:
        want = expected_imm(tf, tac['default']) if tf[0] != 'calc' else None
        vo = find_value_operand(ins, k, frag_n, want)
        cur = operand_value(ins[k + vo[0]], vo[1]) if vo else None
        dflt, expr = imm_inverse(tf, cur) if cur is not None else (None, imm_inverse(tf, 0)[1])
        out['fields'].append({'off': 0, 'width': 8, 'tf_rs': (w['fields'][0]['tf'] if w['fields'] else ''), 'imm': cur,
                              'operand': vo, 'default_expr': expr, 'default': dflt,
                              'fragment_imms': [i['value'] for i in imms if i['in_fragment']]})
    return out


def evidence_list(V, fn, ins, k, n_w, comp, pro):
    """성분(레지스터·슬롯)의 근거 명령(정의·사용) — 사람이 볼 기준선 기록용."""
    path, kind, val = comp
    out = []
    if kind in ('reg', 'inner_reg'):
        X = val
        barrier = False
        for t in range(k - 1, -1, -1):
            x = ins[t]
            if writes(x, X, barrier):
                out.append({'rva': hex(x.rva), 'role': 'def', 'asm': x.asm})
                break
            if x.mn in ('jmp', 'ret'):
                barrier = True
            if X in x.rd and len(out) < 3:
                out.append({'rva': hex(x.rva), 'role': 'use', 'asm': x.asm})
        for t in range(k, min(len(ins), k + n_w)):
            if X in ins[t].rd or X in ins[t].wr:
                out.append({'rva': hex(ins[t].rva), 'role': 'window', 'asm': ins[t].asm})
    elif kind in ('stack', 'inner_stack', 'rbpmem', 'inner_rbpmem'):
        s = val if 'stack' in kind else (val + pro['rbp_off'] if pro.get('rbp_off') is not None else None)
        if s is not None:
            hits = []
            for t, x in enumerate(ins):
                for mm in x.mems:
                    if slot_of(mm, pro) == s:
                        hits.append((abs(t - k), t))
            for _d, t in sorted(hits)[:6]:
                out.append({'rva': hex(ins[t].rva), 'role': 'slot', 'asm': ins[t].asm})
    return {'path': path, 'kind': kind, 'value': val if isinstance(val, str) else hex(val), 'evidence': out}


def describe_caller(V, ret, owner_fn):
    fn = V.owner(ret)
    if fn is None:
        return {'ret_rva': hex(ret), 'error': 'caller owner 없음'}
    ins = V.insns(fn)
    kc = None
    for i, x in enumerate(ins):
        if x.rva + x.size == ret:
            kc = i
            break
    if kc is None:
        return {'ret_rva': hex(ret), 'error': '반환주소 앞 명령 없음'}
    x = ins[kc]
    b, a = ctx_tokens(ins, kc, 1)
    fi = V.M.fn.get(fn[0], {})
    return {'ret_rva': hex(ret), 'call_rva': hex(x.rva), 'call_hex': hexsp(x.b), 'call_asm': x.asm,
            'call_target': (hex(x.rel) if x.rel is not None else None),
            'target_is_owner': (x.rel == owner_fn[0]) if x.rel is not None else False,
            'caller_fn': {'start': hex(fn[0]), 'end': hex(fn[1]), 'size': fn[1] - fn[0], 'call_index': kc,
                          'fnidx': {kk: fi.get(kk) for kk in ('skel', 'head', 'size', 'ninsn')},
                          'loc': loc_summary(V.locs(fn)), 'prologue': prologue(ins)},
            'ctx': {'before': [[t.kn, t.regs] for t in b], 'after': [[t.kn, t.regs] for t in a]}}


def cg_info(V, start):
    cal = V.M.cal.get(start, [])
    calr = V.M.calr.get(start, [])
    return {'n_callee': len(cal), 'n_caller': len(calr), 'callee': [hex(c) for c in cal[:64]],
            'caller': [hex(c) for c in calr[:64]]}


def cmd_snapshot(a):
    targets, names = load_targets(a.ver)
    tactics = parse_tactics((ROOT / 'src' / 'tactics.rs').read_text(encoding='utf-8'))
    cats, base_rules, base_default = parse_presentation((ROOT / 'src' / 'presentation.rs').read_text(encoding='utf-8'))
    visible = json.loads((TOOLS / 'visible_extras.json').read_text(encoding='utf-8'))
    sites_text = (ROOT / 'src' / 'sites.rs').read_text(encoding='utf-8')
    gsha, wins = collect_windows(a.ver, sites_text, targets, tactics)
    V = View(a.exe, a.pkl, a.cg)
    if V.sha != gsha.upper() or V.sha != targets['knobs']['exe_sha256'].upper():
        raise SystemExit('exe sha %s ≠ sites.rs/targets %s' % (V.sha, gsha))
    tab_of = {}
    for name, ids in cats:
        for i in ids:
            tab_of[i] = name
    ex = {}
    for part in EXTRA_PARTS:
        for t in targets['extra_' + part]['tactics']:
            ex['extra_' + t['audit_id'].lower()] = ('extra_' + part, t)
    tac_out = []
    for t in tactics:
        if t['knob'] in ex:
            src_file, et = ex[t['knob']]
            pos_mask, audit = et['pos_mask'], et['audit_id']
            tab_json = et.get('tab')
        else:
            src_file, audit, tab_json = 'knobs', None, None
            pos_mask = base_rules.get(t['id'], base_default)
        tac_out.append(dict(t, source=src_file, audit_id=audit, tab=tab_of.get(t['id']), tab_json=tab_json,
                            visible=t['id'] in tab_of, pos_mask=pos_mask, windows=[]))
    win_out = []
    caller_cache = {}
    nret_bad = 0
    for wi, w in enumerate(wins):
        fn, ins, k, n_w, frag_n = window_geometry(V, w)
        if fn[0] != w['comment_func']:
            raise SystemExit('%s: owner %#x ≠ sites.rs 주석 함수 %#x' % (w['name'], fn[0], w['comment_func']))
        winb = b''.join(ins[k + t].b for t in range(n_w))
        if winb != w['win'] or V.read(w['rva'], len(w['win'])) != w['win']:
            raise SystemExit('%s: exe 바이트 ≠ sites.rs win' % w['name'])
        tac = tactics[w['tactic']]
        pro = prologue(ins)
        b, af = ctx_tokens(ins, k, n_w)
        fi = V.M.fn.get(fn[0], {})
        comps = src_components(w['src_raw'], 'src') + src_components(w['team_chk_raw'], 'team_chk')
        callers = []
        for path, kind, val in comps:
            if kind.endswith('ret_rva'):
                if val not in caller_cache:
                    caller_cache[val] = describe_caller(V, val, fn)
                callers.append(dict(caller_cache[val], path=path))
            if kind == 'return_off' and val != pro['frame']:
                nret_bad += 1
                print('[경고] %s %s return_off %#x ≠ 프롤로그 frame %#x' % (w['name'], path, val, pro['frame']))
        brs = []
        for br in w['branches']:
            tg = int(br['target_rva'], 16)
            ti = V.index(fn, tg) if fn[0] <= tg < fn[1] else None
            brs.append({'off': br['off'], 'target_rva': br['target_rva'], 'in_owner': fn[0] <= tg < fn[1],
                        'target_index': ti, 'target_asm': ins[ti].asm if ti is not None else None})
        key = tuple(x.kc for x in ins[k:k + n_w])
        copies = V.find_key(fn, key)
        rec = {
            'index': wi, 'name': w['name'], 'tactic': w['tactic'], 'tactic_id': w['tactic_id'],
            'source': w['source'], 't_index': w['t_index'], 's_index': w['s_index'], 'subj_key': w['subj_key'],
            'rva': hex(w['rva']), 'value_rva': hex(w['value_rva']), 'win_hex': hexsp(w['win']), 'win_len': len(w['win']),
            'first_len': w['first_len'], 'n_insn': n_w, 'frag_n': frag_n,
            'fields': w['fields'], 'branches': w['branches'], 'dynamic_hex': hexsp(w['dynamic']),
            'tf_raw': w['tf_raw'], 'src_raw': w['src_raw'], 'team_chk_raw': w['team_chk_raw'],
            'src_rs': w['src_rs'], 'team_chk_rs': w['team_chk_rs'],
            'insns': [{'rva': hex(ins[k + t].rva), 'hex': hexsp(ins[k + t].b), 'asm': ins[k + t].asm} for t in range(n_w)],
            'first_asm': ins[k].asm,
            'value': window_value_info(w, ins, k, n_w, frag_n, tac),
            'consumer': consumer(ins, k, frag_n),
            'owner': {'fn': [fn[0], fn[1]], 'start': hex(fn[0]), 'end': hex(fn[1]), 'size': fn[1] - fn[0],
                      'offset': w['rva'] - fn[0], 'index': k, 'n_insn': len(ins),
                      'fnidx': {kk: fi.get(kk) for kk in ('skel', 'head', 'size', 'ninsn')},
                      'loc': loc_summary(V.locs(fn)), 'cg': cg_info(V, fn[0]), 'prologue': pro},
            'ctx': {'before': [[t.kn, t.regs] for t in b], 'after': [[t.kn, t.regs] for t in af],
                    'cmp_struct': cmp_struct(ins, k, n_w)},
            'src_evidence': [evidence_list(V, fn, ins, k, n_w, c, pro) for c in comps if not c[1].endswith(('ret_rva', 'return_off'))],
            'callers': callers,
            'branch_targets': brs,
            'copies': {'key': list(key), 'count': len(copies), 'rvas': [hex(ins[p].rva) for p in copies]},
        }
        win_out.append(rec)
        tac_out[w['tactic']]['windows'].append(wi)
    for t in tac_out:
        if not t['windows']:
            raise SystemExit('창 없는 전술: %s' % t['id'])
    inputs = {}
    for p in [ROOT / 'src' / 'sites.rs', ROOT / 'src' / 'tactics.rs', ROOT / 'src' / 'presentation.rs',
              TOOLS / 'visible_extras.json'] + [TOOLS / 'targets' / nm for nm in names.values()]:
        inputs[str(p)] = sha256_file(p)
    base = {
        'tool': 'track_update.py', 'tool_ver': TOOL_VER, 'created': datetime.datetime.now().isoformat(timespec='seconds'),
        'game_version': a.ver, 'exe': str(a.exe), 'exe_sha256': V.sha, 'pkl': str(a.pkl), 'cg': str(a.cg),
        'pkl_sha256': sha256_file(a.pkl), 'cg_sha256': sha256_file(a.cg), 'inputs': inputs,
        'counts': {'tactics': len(tac_out), 'windows': len(win_out), 'callers': len(caller_cache),
                   'owners': len({tuple(w['owner']['fn']) for w in win_out}), 'return_off_mismatch': nret_bad},
        'target_files': names, 'targets_raw': targets, 'visible_extras': visible,
        'categories': [[n, ids] for n, ids in cats],
        'tactics': tac_out, 'windows': win_out,
    }
    out = Path(a.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(base, ensure_ascii=False, indent=1), encoding='utf-8', newline='\n')
    print('기준선 %s — 전술 %d · 창 %d · owner %d · 콜사이트 %d · return_off 불일치 %d'
          % (out, len(tac_out), len(win_out), base['counts']['owners'], len(caller_cache), nret_bad))
    return 0


# ───────────────────────────────────────────────────────── 추적 (track)
def loc_lines(extra):
    lost = ['%s:%d:%d' % (m.split('\\')[-1], x[0], x[1]) for m, x in extra[0][:24]]
    born = ['%s:%d:%d' % (m.split('\\')[-1], x[0], x[1]) for m, x in extra[1][:24]]
    return lost, born


def split_shift(extra):
    """aidiff 는 모듈당 시프트 1개만 본다. 잃은 행 = 생긴 행(같은 모듈·행·열)이면 일부 행은 그대로이고
    나머지만 밀린 것 = 함수 중간(또는 인라인된 같은 파일 callee)에 소스 행이 끼거나 빠짐."""
    lost, born = extra if extra else ([], [])
    return bool(lost) and set(lost) == set(born)


def module_renamed(extra):
    """EDITED 의 잃은 행·생긴 행이 (행,열) 로는 같고 모듈 이름만 다르면 = 소스 파일 이름 변경(내용 불변)."""
    lost, born = extra if extra else ([], [])
    if not lost or not born:
        return None
    lo = {x for _m, x in lost}
    bo = {x for _m, x in born}
    if len(lo) != len(lost) or len(bo) != len(born):
        return None
    d, n = aidiff.shift_of(lo, bo)
    if n == len(lo) == len(bo):
        mo = sorted({m for m, _x in lost})
        mn = sorted({m for m, _x in born})
        if mo != mn:       # 전체 경로 비교(파일 이름 변경·디렉터리 이동 둘 다)
            tail = lambda ms: ','.join('\\'.join(m.split('\\')[-2:]) for m in ms)  # noqa: E731
            return 'MODULE_RENAMED(%s→%s%s)' % (tail(mo), tail(mn), (' 행%+d' % d) if d else '')
    return None


class Tracker:
    def __init__(s, a, base):
        s.a, s.base = a, base
        s.O = View(a.old_exe, a.old_pkl, a.old_cg)
        if s.O.sha != base['exe_sha256'].upper():
            raise RuntimeError('--old-exe sha %s ≠ 기준선 %s' % (s.O.sha, base['exe_sha256']))
        s.N = View(a.new_exe, a.new_pkl, a.new_cg)
        s.pairs, s.gone, s.added = aidiff.pair(s.O.census, s.N.census)
        s.match_fn, s.match_mid = repin.build_match(s.O.M, s.N.M, s.O.M.cg, s.N.M.cg)
        import types
        for v in (s.O.M, s.N.M):          # midpin.Img 에 repin.Img.find_all 을 붙인다(섹션 튜플 형식 동일)
            v.find_all = types.MethodType(repin.Img.find_all, v)
        s._gmap = None
        s._own, s._al, s._sv, s._ret = {}, {}, {}, {}

    def gmap(s):
        if s._gmap is None:
            s._gmap = midpin.build(s.O.M, s.N.M)
        return s._gmap

    def loc_search(s, ofn):
        """AI 계층(census) 밖 함수: 같은 모듈 Location 을 참조하는 신 함수들 중 Location 집합이 (행 시프트 포함) 같은 유일 후보."""
        lo = s.O.locs(ofn)
        if not lo:
            return None, 'Location 없음'
        if s.N._loc is None:
            s.N._loc = aidiff.find_locations(s.N.A)
        mods = set(lo['lmods'])
        want = {a for a, (m, _l, _c) in s.N._loc.items() if m in mods}
        cands = {s.N.owner(h) for h in lea_rip_targets(s.N.A, want)} - {None}
        full = []
        for fn in cands:
            ln = s.N.locs(fn)
            if not ln:
                continue
            v, _x = aidiff.verdict(lo, ln)
            if v == 'IDENTICAL' or v.startswith('SHIFTED'):
                full.append((fn, v))
        if len(full) == 1:
            return full[0][0], 'LOC_SEARCH %s (후보 %d 중 유일)' % (full[0][1], len(cands))
        same = [x for x in full if x[0][1] - x[0][0] == ofn[1] - ofn[0]]
        if len(same) == 1:
            return same[0][0], 'LOC_SEARCH_WEAK %s (완전일치 %d · 크기로 유일)' % (same[0][1], len(full))
        best = sorted(((aidiff.mod_overlap(lo, s.N.locs(fn))[1], fn) for fn in cands if s.N.locs(fn)), reverse=True)[:1]
        hint = (' · 최선 부분일치 %#x(겹침 %d/%d)' % (best[0][1][0], best[0][0], len(lo['lines']))) if best else ''
        return None, 'LOC_SEARCH 실패(같은 모듈 참조 %d · 완전일치 %d)%s' % (len(cands), len(full), hint)

    def map_owner(s, ofn):
        if ofn in s._own:
            return s._own[ofn]
        r = {'old': [ofn[0], ofn[1]], 'loc_how': None, 'loc_verdict': None, 'loc_new': None, 'repin': None,
             'repin_how': None, 'new': None, 'agree': None, 'lost': [], 'born': [], 'note': ''}
        loc_new = None
        if ofn in s.O.census:
            if ofn in s.pairs:
                kn, _d, _n = s.pairs[ofn]
                v, extra = aidiff.verdict(s.O.census[ofn], s.N.census[kn])
                loc_new = kn
                if v == 'EDITED' and extra:
                    r['lost'], r['born'] = loc_lines(extra)
                    v = module_renamed(extra) or ('EDITED(시프트 갈림)' if split_shift(extra) else v)
                r.update(loc_how='LOC_PAIR', loc_verdict=v)
            else:
                r['loc_how'] = 'LOC_GONE'
        else:
            r['loc_how'] = 'LOC_NA' if s.O.locs(ofn) else 'LOC_NONE'
        if loc_new is None and r['loc_how'] in ('LOC_NA', 'LOC_GONE'):
            fnd, how = s.loc_search(ofn)
            r['loc_search'] = how
            if fnd is not None:
                loc_new = fnd
                r['weak'] = how.startswith('LOC_SEARCH_WEAK')
                v, extra = aidiff.verdict(s.O.locs(ofn), s.N.locs(fnd))
                r.update(loc_how=r['loc_how'] + '+SEARCH', loc_verdict=v)
        rp = None
        try:
            res, nr, note = s.match_fn(ofn[0])
            r['repin_how'] = '%s %s' % (res, note)
            if isinstance(nr, int):
                rp = s.N.owner(nr)
        except Exception as e:  # noqa
            r['repin_how'] = 'repin 오류 %r' % e
        if rp is None and loc_new is None:
            try:
                nst, how = midpin.match_owner_callee(s.O.M, s.N.M, ofn[0], s.gmap(), s.N.M.cal, s.N.M.calr)
                if nst is None:
                    nst, how = midpin.match_owner_align(s.O.M, s.N.M, ofn[0], s.gmap(), s.N.M.cal, s.N.M.calr)
                    how = 'A:' + how
                else:
                    how = 'C:' + how
                r['repin_how'] = (r['repin_how'] or '') + ' | midpin ' + how
                if nst is not None:
                    rp = s.N.owner(nst)
            except Exception as e:  # noqa
                r['repin_how'] = (r['repin_how'] or '') + ' | midpin 오류 %r' % e
        r['repin'] = [rp[0], rp[1]] if rp else None
        r['loc_new'] = [loc_new[0], loc_new[1]] if loc_new else None
        if loc_new:
            new = loc_new
            r['agree'] = (rp == loc_new) if rp else None
            if rp and rp != loc_new:
                r['note'] = 'owner 대응 불일치: Location %#x ≠ repin %#x (Location 채택)' % (loc_new[0], rp[0])
        elif rp:
            new = rp
            lo, ln = s.O.locs(ofn), s.N.locs(rp)
            if lo and ln:
                v, extra = aidiff.verdict(lo, ln)
                if v == 'EDITED' and extra:
                    r['lost'], r['born'] = loc_lines(extra)
                    v = module_renamed(extra) or ('EDITED(시프트 갈림)' if split_shift(extra) else v)
                r['loc_verdict'] = v
                r['loc_how'] = (r['loc_how'] or '') + '+CHECK'
            elif lo and not ln:
                r['loc_verdict'] = 'LOC_LOST'
            r['note'] = 'Location 대응 없음 → repin 단독'
        else:
            new = None
        r['new'] = [new[0], new[1]] if new else None
        r['_new'] = new
        s._own[ofn] = r
        return r

    def align(s, ofn, nfn, k):
        O_ins, N_ins = s.O.insns(ofn), s.N.insns(nfn)
        if len(O_ins) <= 6000 and len(N_ins) <= 6000:
            key = (ofn, nfn, 0)
            if key not in s._al:
                s._al[key] = Align(O_ins, N_ins, 0, len(O_ins), 0, len(N_ins))
            return s._al[key]
        lo, hi = max(0, k - 2000), min(len(O_ins), k + 2000)
        c = int(k * len(N_ins) / max(1, len(O_ins)))
        nlo, nhi = max(0, c - 2600), min(len(N_ins), c + 2600)
        key = (ofn, nfn, lo, hi)
        if key not in s._al:
            s._al[key] = Align(O_ins, N_ins, lo, hi, nlo, nhi)
        return s._al[key]

    def svotes(s, al, ofn, nfn, pro_o, pro_n):
        key = id(al)
        if key not in s._sv:
            s._sv[key] = slot_votes(al, s.O.insns(ofn), s.N.insns(nfn), pro_o, pro_n)
        return s._sv[key]

    def map_target(s, tg, ofn, nfn, al):
        """분기/호출 대상 구 RVA → 기대 신 RVA."""
        if ofn[0] <= tg < ofn[1]:
            ti = s.O.index(ofn, tg)
            if ti is None:
                return None, '대상이 명령 경계 아님'
            nj = al.m.get(ti)
            if nj is None and not (al.range[0] <= ti < al.range[1]):
                nj = s.align(ofn, nfn, ti).m.get(ti)     # 거대 함수: 대상 주변 구간을 따로 정렬
            if nj is None:
                return None, '대상 명령 정렬 안 됨'
            return s.N.insns(nfn)[nj].rva, 'owner 내부'
        fo = s.O.owner(tg)
        if fo and fo[0] == tg:
            r = s.map_owner(fo)
            if r['_new']:
                return r['_new'][0], '함수 대응'
            return None, '대상 함수 대응 없음'
        return None, 'owner 밖 비함수 대상'

    def track_ret(s, ret, win_ofn, win_nfn):
        """Src::Caller 반환주소 → 신 반환주소 + 콜사이트 판정."""
        if ret in s._ret:
            return s._ret[ret]
        r = {'old_ret': ret, 'new_ret': None, 'status': 'MISSING', 'note': '', 'site': None, 'detail': {},
             'cfo': None, 'cfn': None}
        cfo = s.O.owner(ret)
        if cfo is None:
            r['note'] = '구 caller owner 없음'
            s._ret[ret] = r
            return r
        O_ins = s.O.insns(cfo)
        kc = next((i for i, x in enumerate(O_ins) if x.rva + x.size == ret), None)
        if kc is None:
            r['note'] = '구 반환주소 앞 명령 없음'
            s._ret[ret] = r
            return r
        own = s.map_owner(cfo)
        cfn = own['_new']
        r['detail']['caller_owner'] = {k: v for k, v in own.items() if k != '_new'}
        if cfn is None:
            r['note'] = 'caller 함수 대응 없음(%s)' % own['repin_how']
            s._ret[ret] = r
            return r
        N_ins = s.N.insns(cfn)
        al = s.align(cfo, cfn, kc)
        jc = al.m.get(kc)
        if jc is None or N_ins[jc].mn != O_ins[kc].mn:
            key = (O_ins[kc].kc,)
            cand = [p for p in s.N.find_key(cfn, key)]
            r['note'] = 'call 정렬 실패 · 같은 꼴 후보 %d' % len(cand)
            a0 = O_ins[kc]
            if len(cand) > 1 and a0.rel is not None:
                if a0.rel == win_ofn[0]:
                    e0 = win_nfn[0] if win_nfn else None
                else:
                    e0 = s.map_target(a0.rel, cfo, cfn, al)[0]
                if e0 is not None:
                    cand = [p for p in cand if N_ins[p].rel == e0]
                    r['note'] += ' → 호출 대상 %#x 로 거르면 %d' % (e0, len(cand))
            if len(cand) == 1:
                jc = cand[0]
                r['note'] += ' → 유일 후보 채택'
            else:
                r['detail']['candidates'] = [hex(N_ins[p].rva + N_ins[p].size) for p in cand[:8]]
                s._ret[ret] = r
                return r
        a = O_ins[kc]
        exp, why = (None, '간접 호출')
        if a.rel is not None:
            if a.rel == win_ofn[0]:
                exp, why = (win_nfn[0] if win_nfn else None), '창 owner'
            else:
                exp, why = s.map_target(a.rel, cfo, cfn, al)
        if exp is not None and N_ins[jc].rel != exp:
            tc = [p for p in range(len(N_ins)) if N_ins[p].mn == a.mn and N_ins[p].rel == exp]
            if len(tc) == 1:
                r['note'] = (r['note'] + ' · ' if r['note'] else '') + '정렬 %#x 대신 호출 대상(%#x)이 같은 유일 call 채택' % (
                    N_ins[jc].rva, exp)
                jc = tc[0]
        b = N_ins[jc]
        pro_o, pro_n = prologue(O_ins), prologue(N_ins)
        site = SiteCtx(O_ins, N_ins, kc, jc, 1, al, pro_o, pro_n, s.svotes(al, cfo, cfn, pro_o, pro_n))
        r['site'] = site
        r['cfo'], r['cfn'] = cfo, cfn
        r['new_ret'] = b.rva + b.size
        tgt_ok = (b.rel == exp) if a.rel is not None else (a.kn == b.kn)
        same_bytes = (a.mn == b.mn and a.size == b.size) if a.rel is not None else (a.b == b.b)
        problems = []
        if not same_bytes:
            problems.append('call 바이트 다름 %s → %s' % (a.asm, b.asm))
        if not tgt_ok:
            problems.append('호출 대상 %s ≠ 기대 %s(%s)' % (('%#x' % b.rel) if b.rel is not None else b.asm,
                                                         ('%#x' % exp) if exp is not None else '?', why))
        if not site.frame_same:
            problems.append('caller 프레임 %s → %s' % (pro_key(pro_o), pro_key(pro_n)))
        if not site.ctx_same:
            problems.append('caller 콜사이트 ±%d 명령 다름' % CTX)
        r['status'] = 'SAME' if not problems else ('CHANGED' if tgt_ok else 'TARGET_MISMATCH')
        r['note'] = (r['note'] + ' · ' if r['note'] else '') + ('콜사이트 동일' if not problems else '; '.join(problems))
        r['detail'].update(call_old=a.asm, call_new=b.asm, new_caller=hex(cfn[0]))
        s._ret[ret] = r
        return r

    # ─────────────────────────────── 창 1개
    def window(s, w):
        r = {'index': w['index'], 'name': w['name'], 'tactic': w['tactic'], 'tactic_id': w['tactic_id'],
             'old_rva': w['rva'], 'old_value_rva': w['value_rva'], 'new_rva': None, 'new_value_rva': None,
             'code': None, 'flags': [], 'reasons': [], 'auto': False, 'proposal': {}, 'detail': {}}

        def flag(code, why):
            if code not in r['flags']:
                r['flags'].append(code)
            r['reasons'].append('%s: %s' % (code, why))

        ofn = tuple(w['owner']['fn'])
        O_ins = s.O.insns(ofn)
        k = s.O.index(ofn, int(w['rva'], 16))
        n_w, frag_n = w['n_insn'], w['frag_n']
        oldb = b''.join(O_ins[k + t].b for t in range(n_w)) if k is not None else b''
        if k is None or hexsp(oldb) != w['win_hex']:
            raise RuntimeError('%s: 구 exe 창 바이트가 기준선과 다름' % w['name'])
        own = s.map_owner(ofn)
        r['detail']['owner'] = {kk: vv for kk, vv in own.items() if kk != '_new'}
        if own['note']:
            r['reasons'].append('owner: ' + own['note'])
        nfn = own['_new']
        if nfn is None:
            flag('OWNER_MISSING', 'owner %#x 대응 없음 (Location %s · repin %s)' % (ofn[0], own['loc_how'], own['repin_how']))
            return s._finish(r, w)
        N_ins = s.N.insns(nfn)
        al = s.align(ofn, nfn, k)
        j, how = al.m.get(k), al.tag.get(k)
        key_o = tuple(x.kc for x in O_ins[k:k + n_w])
        c_old = s.O.find_key(ofn, key_o)
        if j is not None and j + n_w > len(N_ins):
            j, how = None, None
        if j is None:
            # 앞뒤 대응 명령 사이 틈의 길이가 같으면 위치로 사영(첫 명령 꼴이 바뀐 경우 → 아래 비교에서 LOGIC_CHANGED)
            tp = next((t for t in range(k - 1, max(-1, k - 200), -1) if t in al.m), None)
            tn = next((t for t in range(k + 1, min(len(O_ins), k + 200)) if t in al.m), None)
            if tp is not None and tn is not None and al.m[tn] - al.m[tp] == tn - tp:
                jj = al.m[tp] + (k - tp)
                if jj + n_w <= len(N_ins) and s.N.find_key(nfn, key_o) == []:
                    j, how = jj, 'GAP_POS'
        if j is None:
            cand = s.N.find_key(nfn, key_o)
            if len(cand) == 1:
                j, how = cand[0], 'KEY_UNIQUE'
            elif len(cand) > 1:
                c = int(k * len(N_ins) / max(1, len(O_ins)))
                j, how = min(cand, key=lambda p: abs(p - c)), 'KEY_NEAREST'
                flag('COPIES', '정렬 실패 · 같은 꼴 후보 %d개 %s (구 owner 안 %d개)' % (
                    len(cand), [hex(N_ins[p].rva) for p in cand[:8]], len(c_old)))
            else:
                ks = tuple(x.ks for x in O_ins[k:k + n_w])
                sc = [p for p in range(len(N_ins) - n_w + 1) if all(N_ins[p + t].ks == ks[t] for t in range(n_w))]
                if len(sc) == 1:
                    j, how = sc[0], 'SHAPE_UNIQUE'
                else:
                    fc = s.N.find_key(nfn, tuple(x.kc for x in O_ins[k:k + frag_n]))
                    flag('SITE_MISSING', 'owner %#x→%#x 안에서 창 대응 실패 (꼴 후보 %d개 %s)%s' % (
                        ofn[0], nfn[0], len(sc), [hex(N_ins[p].rva) for p in sc[:8]],
                        (' · 참고: 값 조각 `%s` 와 같은 꼴·즉치 위치 %d개 %s — 창이 갈라졌거나 명령이 끼었을 수 있음' % (
                            ' ; '.join(x.asm for x in O_ins[k:k + frag_n]), len(fc), [hex(N_ins[p].rva) for p in fc[:8]])) if fc else ''))
                    r['detail']['shape_candidates'] = [hex(N_ins[p].rva) for p in sc[:16]]
                    r['detail']['fragment_candidates'] = [hex(N_ins[p].rva) for p in fc[:16]]
                    return s._finish(r, w)
        r['detail']['align'] = how
        r['new_rva'] = N_ins[j].rva
        try:
            ob12 = s.O.read(int(w['rva'], 16), 12)
            nr_, kind_, _note = repin.repin_entry({'value': w['rva'], 'bytes': ob12.hex()}, s.O.M, s.N.M, s.match_fn, s.match_mid)
            r['detail']['repin_site'] = {'new': hex(nr_) if nr_ is not None else None, 'kind': kind_}
            if nr_ is not None and nr_ != r['new_rva']:
                if kind_.startswith('BYTES_UNIQUE'):
                    r['repin_conflict'] = True
                    r['reasons'].append('BLOCK: repin 바이트 유일검색 %s → %#x ≠ 정렬 %#x' % (kind_, nr_, r['new_rva']))
                else:
                    r['reasons'].append('참고: repin %s → %#x ≠ 정렬 %#x (repin 은 동일 오프셋 가정일 수 있음)' % (kind_, nr_, r['new_rva']))
        except Exception as e:  # noqa
            r['detail']['repin_site'] = {'error': repr(e)}
        prefix = int(w['value_rva'], 16) - int(w['rva'], 16)
        if prefix:
            pre_n = 0
            off = 0
            while off < prefix:
                off += O_ins[k + pre_n].size
                pre_n += 1
            r['new_value_rva'] = N_ins[j + pre_n].rva
        else:
            r['new_value_rva'] = N_ins[j].rva
        # ── 창 명령 비교
        key_n = tuple(x.kc for x in N_ins[j:j + n_w])
        c_new = s.N.find_key(nfn, key_n)
        if len(c_new) != len(c_old) and 'COPIES' not in r['flags']:
            flag('COPIES', '같은 꼴 사본 %d→%d개 (신 %s)' % (len(c_old), len(c_new), [hex(N_ins[p].rva) for p in c_new[:8]]))
        r['detail']['copies'] = {'old': len(c_old), 'new': len(c_new), 'new_rvas': [hex(N_ins[p].rva) for p in c_new[:16]]}
        # 값 조각(창 첫 frag_n 명령 = 즉치를 담은 명령까지) 단위 사본 — 컴파일러가 비교를 복제하며 뒤 분기 꼴을 바꾸면
        # (0.6.4→0.6.5 objective_hunt_exit_hp_30: `cmp r14,0x1e; jb` 1곳 → `cmp r15,0x1e; jae`×2 + `; jb`×1) 창 키로는 안 잡힌다.
        fk_o = tuple(x.kc for x in O_ins[k:k + frag_n])
        fk_n = tuple(x.kc for x in N_ins[j:j + frag_n])
        f_old = s.O.find_key(ofn, fk_o)
        f_new = s.N.find_key(nfn, fk_n)
        r['detail']['copies'].update(frag_old=len(f_old), frag_new=len(f_new), frag_new_rvas=[hex(N_ins[p].rva) for p in f_new[:16]])
        frag_has_imm = any(op[0] == 'i' for x in O_ins[k:k + frag_n] for op in x.ops)   # 즉치 없는 조각(add rax,rax 등)은 흔해서 사본 수가 무의미
        if frag_has_imm and len(f_old) != len(f_new):
            flag('COPIES', '값 조각 사본 %d→%d개 (구 `%s` · 신 `%s` 위치 %s) — 갈라진 사본마다 지점 추가/삭제 판단' % (
                len(f_old), len(f_new), ' ; '.join(x.asm for x in O_ins[k:k + frag_n]),
                ' ; '.join(x.asm for x in N_ins[j:j + frag_n]), [hex(N_ins[p].rva) for p in f_new[:8]]))
        if fk_n != fk_o:
            alt = [p for p in s.N.find_key(nfn, fk_o) if p != j]
            if alt:
                r['detail']['fragment_alt'] = [hex(N_ins[p].rva) for p in alt[:16]]
                r['reasons'].append('참고: 정렬 위치의 값 조각이 구와 다른데(꼴·즉치·필드), 구 조각 `%s` 와 같은 명령이 신 owner %s 에 있음 '
                                    '— 정렬이 다른 사본을 짚었을 수 있다(VALUE_CHANGED 면 기본값 변경으로 단정하기 전에 확인)' % (
                                        ' ; '.join(x.asm for x in O_ins[k:k + frag_n]), [hex(N_ins[p].rva) for p in alt[:8]]))
        diffs = []
        for t in range(n_w):
            a, b = O_ins[k + t], N_ins[j + t]
            if a.ks != b.ks:
                diffs.append(('SHAPE', t, a.asm, b.asm))
                continue
            if is_rel(a) and is_rel(b):
                if a.size != b.size:
                    diffs.append(('ENC', t, a.asm, b.asm))
                continue
            if a.b == b.b:
                continue
            found = False
            for oa, ob in zip(a.ops, b.ops):
                if oa == ob:
                    continue
                found = True
                if oa[0] == 'r':
                    diffs.append(('REG', t, oa[3], ob[3]))
                elif oa[0] == 'i':
                    diffs.append(('IMM', t, oa[1], ob[1]))
                elif oa[0] == 'm':
                    if oa[2] != ob[2] or oa[3] != ob[3]:
                        diffs.append(('REG', t, '%s/%s' % (oa[2], oa[3]), '%s/%s' % (ob[2], ob[3])))
                    if oa[5] != ob[5]:
                        diffs.append(('SLOT' if oa[1] in ('sp', 'bp') else 'FIELD', t, oa[5], ob[5]))
            if not found:
                diffs.append(('ENC', t, hexsp(a.b), hexsp(b.b)))
        r['detail']['diffs'] = [[d[0], d[1], str(d[2]), str(d[3])] for d in diffs]
        for d in diffs:
            if d[0] == 'SHAPE':
                flag('LOGIC_CHANGED', '창 %d번째 명령 꼴 변화 `%s` → `%s`' % (d[1], d[2], d[3]))
            elif d[0] == 'IMM':
                flag('VALUE_CHANGED', '창 %d번째 명령 즉치 %#x → %#x%s' % (d[1], d[2], d[3], '' if d[1] < frag_n else ' (조각 밖 보조 즉치)'))
            elif d[0] == 'REG':
                flag('REG_RENAME', '창 %d번째 명령 레지스터 %s → %s' % (d[1], d[2], d[3]))
            elif d[0] == 'SLOT':
                flag('REG_RENAME', '창 %d번째 명령 스택 변위 %#x → %#x' % (d[1], d[2] & M64, d[3] & M64))
            elif d[0] == 'FIELD':
                flag('REG_RENAME', '창 %d번째 명령 구조체 필드 변위 %#x → %#x' % (d[1], d[2] & M64, d[3] & M64))
            elif d[0] == 'ENC':
                flag('REG_RENAME', '창 %d번째 명령 인코딩 변화 `%s` → `%s` (창 재구성 필요)' % (d[1], d[2], d[3]))
        newwin = b''.join(N_ins[j + t].b for t in range(n_w))
        r['detail']['new_win_hex'] = hexsp(newwin)
        r['detail']['new_insns'] = [{'rva': hex(N_ins[j + t].rva), 'hex': hexsp(N_ins[j + t].b), 'asm': N_ins[j + t].asm}
                                    for t in range(n_w)]
        if len(newwin) < 5:
            flag('LOGIC_CHANGED', '신 창 길이 %dB < 5 (E9 rel32 불가)' % len(newwin))
        # ── 소비 의미
        co = consumer(O_ins, k, frag_n)
        cn = consumer(N_ins, j, frag_n)
        r['detail']['consumer'] = {'old': co, 'new': cn}
        if cons_key(co) != cons_key(cn):
            flag('LOGIC_CHANGED', '소비 명령 변화 `%s` → `%s` (%s→%s)' % (co.get('asm'), cn.get('asm'), co.get('kind'), cn.get('kind')))
        elif co.get('kind') == 'flags' and co.get('rva') and cn.get('rva'):
            # 창 밖 소비 Jcc 의 분기 대상도 대응해야 한다(꼴만 같고 다른 블록으로 가면 판단 의미가 바뀐다)
            ti_o, ti_n = s.O.index(ofn, int(co['rva'], 16)), s.N.index(nfn, int(cn['rva'], 16))
            if ti_o is not None and ti_n is not None and ti_o >= k + n_w:
                a_c, b_c = O_ins[ti_o], N_ins[ti_n]
                if a_c.rel is not None:
                    exp_c, why_c = s.map_target(a_c.rel, ofn, nfn, al)
                    r['detail']['consumer']['target'] = {'old': hex(a_c.rel), 'new': hex(b_c.rel) if b_c.rel is not None else None,
                                                         'expected': hex(exp_c) if exp_c is not None else None, 'why': why_c}
                    if exp_c is None or b_c.rel != exp_c:
                        flag('LOGIC_CHANGED', '소비 분기 `%s` 대상 %#x → 신 %s ≠ 기대 %s (%s)' % (
                            a_c.asm, a_c.rel, hex(b_c.rel) if b_c.rel is not None else '?', hex(exp_c) if exp_c is not None else '?', why_c))
        # ── 비교 구조·주변
        ob, oa = ctx_tokens(O_ins, k, n_w)
        nb, na = ctx_tokens(N_ins, j, n_w)
        ctx_kf = [x.kf for x in ob + oa] == [x.kf for x in nb + na]
        ctx_kn = [x.kn for x in ob + oa] == [x.kn for x in nb + na]
        ratio = difflib.SequenceMatcher(None, [x.ks for x in ob + oa], [x.ks for x in nb + na], autojunk=False).ratio()
        r['detail']['ctx'] = {'kf_same': ctx_kf, 'kn_same': ctx_kn, 'shape_ratio': round(ratio, 3)}
        edited = str(own.get('loc_verdict') or '').startswith('EDITED')
        if edited and not ctx_kn:
            if own['loc_verdict'] == 'EDITED(시프트 갈림)':
                flag('LOGIC_CHANGED', 'owner 소스 변경(aidiff EDITED · 행 시프트 갈림: %s 등은 그대로, 나머지 행만 밀림 = 함수 중간 편집 의심) + 창 주변 변화' % (
                    own['lost'][:4],))
            else:
                flag('LOGIC_CHANGED', 'owner 소스 변경(aidiff EDITED) + 창 주변 변화 (사라진 행 %s · 생긴 행 %s)' % (
                    own['lost'][:6], own['born'][:6]))
        elif edited:
            r['reasons'].append('참고: owner aidiff EDITED 이지만 창 ±%d 명령은 불변' % CTX)
        # ── 창 안 분기 대상
        bts = []
        for t in range(n_w):
            a, b = O_ins[k + t], N_ins[j + t]
            if a.rel is None:
                continue
            exp, why = s.map_target(a.rel, ofn, nfn, al)
            ok = (exp is not None and b.rel == exp)
            bts.append({'old': hex(a.rel), 'new': hex(b.rel) if b.rel is not None else None,
                        'expected': hex(exp) if exp is not None else None, 'ok': ok, 'why': why})
            if not ok:
                flag('LOGIC_CHANGED', '창 안 분기 대상 %#x → 신 %s ≠ 기대 %s (%s)' % (
                    a.rel, hex(b.rel) if b.rel is not None else '?', hex(exp) if exp is not None else '?', why))
        r['detail']['branch_targets'] = bts
        # ── 프레임
        pro_o, pro_n = prologue(O_ins), prologue(N_ins)
        r['detail']['prologue'] = {'old': {kk: pro_o[kk] for kk in ('pushes', 'sub', 'frame', 'rbp_off')},
                                   'new': {kk: pro_n[kk] for kk in ('pushes', 'sub', 'frame', 'rbp_off')}}
        if pro_key(pro_o) != pro_key(pro_n):
            flag('REG_RENAME', 'owner 프레임 변화 pushes %s→%s · sub %#x→%#x · rbp_off %s→%s' % (
                pro_o['pushes'], pro_n['pushes'], pro_o['sub'], pro_n['sub'], pro_o['rbp_off'], pro_n['rbp_off']))
        # ── Src / team_chk
        site = SiteCtx(O_ins, N_ins, k, j, n_w, al, pro_o, pro_n, s.svotes(al, ofn, nfn, pro_o, pro_n))
        ctxw = ChainCtx(s, [Level(site, ofn, nfn)])
        if site.cmp_changed():
            flag('LOGIC_CHANGED', '창 ±8 명령의 비교·분기 구조 변화(정렬 기준)')
        res = SrcResult()
        new_src = xform_src(copy.deepcopy(w['src_raw']), ctxw, 'src', res)
        new_chk = xform_src(copy.deepcopy(w['team_chk_raw']), ctxw, 'team_chk', res)
        r['detail']['src_items'] = res.items
        r['proposal'].update(src=new_src, team_chk=new_chk)
        for it in res.items:
            if it['kind'] == 'ret_rva':
                if it['status'] == 'MISSING':
                    flag('SITE_MISSING', 'Caller 반환 콜사이트 %s 소실 — %s' % (it['old'], it['note']))
                elif it['status'] == 'TARGET_MISMATCH':
                    flag('LOGIC_CHANGED', 'Caller 콜사이트 %s → %s 호출 대상 불일치 — %s' % (it['old'], it['new'], it['note']))
                elif it['status'] == 'UNKNOWN':
                    flag('REG_RENAME', 'Caller %s 판정 불가 — %s' % (it['old'], it['note']))
                elif it['status'] != 'SAME':
                    flag('REG_RENAME', 'Caller 콜사이트 %s → %s 변화 — %s' % (it['old'], it['new'], it['note']))
                continue
            if it['status'] == 'RENAMED':
                flag('REG_RENAME', '주체 %s %s → %s 재지정 제안 (%s)' % (it['path'], fmtv(it['old']), fmtv(it['new']), it['note']))
            elif it['status'] == 'UNKNOWN':
                flag('REG_RENAME', '주체 %s %s 대응 근거 부족 — 사람 확인 (%s)' % (it['path'], fmtv(it['old']), it['note']))
        # ── 값(기본값) 변화
        vi = w['value']
        vals = []
        tf = tuple(vi['tf'])
        for fd in vi['fields']:
            vo = fd.get('operand')
            if vo is None:
                continue
            t_at, oi = vo
            if oi == -1:
                nb0, ob0 = jcc_byte(N_ins[j + t_at]), jcc_byte(O_ins[k + t_at])
                vals.append({'old_imm': ob0, 'new_imm': nb0, 'old_default': fd['default'],
                             'new_default': imm_inverse(tf, nb0)[0] if nb0 is not None else None, 'expr': fd['default_expr']})
                continue
            nv = operand_value(N_ins[j + t_at], oi) if (j + t_at) < len(N_ins) else None
            ov = fd['imm']
            if nv is None:
                continue
            if w['source'] == 'knobs':
                nv &= (1 << (8 * fd['width'])) - 1
            nd = imm_inverse(tf, nv)[0] if nv != ov else fd['default']
            vals.append({'old_imm': ov, 'new_imm': nv, 'old_default': fd['default'], 'new_default': nd, 'expr': fd['default_expr']})
        changed_vals = [v for v in vals if v['old_imm'] != v['new_imm']]
        if changed_vals and r['detail'].get('fragment_alt'):
            for v in changed_vals:
                v['suspect'] = '정렬 의심 — 구 값 그대로인 같은 조각이 %s 에 있음' % ','.join(r['detail']['fragment_alt'][:4])
        r['detail']['values'] = vals
        if changed_vals:
            flag('VALUE_CHANGED', '; '.join('게임 기본값 %s → %s (즉치 %s → %s, %s)%s' % (
                v['old_default'], v['new_default'], v['old_imm'], v['new_imm'], v['expr'],
                (' [%s]' % v['suspect']) if v.get('suspect') else '') for v in changed_vals))
        # ── 창 바이트·패치 제안
        r['proposal'].update(rva=r['new_rva'], value_rva=r['new_value_rva'], win_hex=hexsp(newwin),
                             first_len=sum(N_ins[j + t].size for t in range(frag_n)),
                             first_asm=N_ins[j].asm, value_asm=N_ins[j + frag_n - 1].asm,
                             function=[nfn[0], nfn[1]],
                             prefix_hex=hexsp(b''.join(N_ins[j + t].b for t in range(frag_n - 1))) if frag_n > 1 else '',
                             dynamic_hex=s.dynamic_for(w, O_ins, N_ins, k, j, frag_n, r))
        r['detail']['site_ctx'] = {'ctx_same': site.ctx_same, 'frame_same': site.frame_same, 'slot_identity': site.slot_identity}
        return s._finish(r, w)

    def dynamic_for(s, w, O_ins, N_ins, k, j, frag_n, r):
        old = w['dynamic_hex']
        if w['source'] == 'knobs':
            return old   # gen_sites 가 다시 만든다
        ob = b''.join(O_ins[k + t].b for t in range(frag_n))
        nb = b''.join(N_ins[j + t].b for t in range(frag_n))
        if ob == nb:
            return old
        # 즉치만 다르고 꼴·레지스터·메모리 피연산자가 같으면 동적 조각(값은 r11)도 그대로 유효
        if all(O_ins[k + t].ks == N_ins[j + t].ks and O_ins[k + t].regs == N_ins[j + t].regs
               and O_ins[k + t].mems == N_ins[j + t].mems for t in range(frag_n)):
            return old
        if frag_n == 1:
            try:
                regen_old = GS.dynamic_instruction(_CsShim(O_ins[k]))
                if hexsp(regen_old) == old:
                    return hexsp(GS.dynamic_instruction(_CsShim(N_ins[j])))
            except Exception as e:  # noqa
                r['reasons'].append('dynamic 재생성 실패: %r' % e)
        if 'REG_RENAME' not in r['flags']:
            r['flags'].append('REG_RENAME')
        r['reasons'].append('REG_RENAME: dynamic_hex 는 손으로 만든 형식 — 신 조각 `%s` 에 맞춰 재작성 필요' %
                            ' ; '.join(N_ins[j + t].asm for t in range(frag_n)))
        return old

    def _finish(s, r, w):
        r['code'] = next((c for c in PRIORITY if c in r['flags']), 'SAME')
        own = r['detail'].get('owner', {})
        owner_conflict = own.get('agree') is False
        weak = bool(own.get('weak')) and own.get('agree') is not True
        r['auto'] = r['code'] == 'SAME' and not owner_conflict and not weak and not r.get('repin_conflict')
        if r['code'] == 'SAME' and owner_conflict:
            r['reasons'].append('BLOCK: owner 대응 불일치 — 자동확정 보류')
        if r['code'] == 'SAME' and weak:
            r['reasons'].append('BLOCK: owner 를 Location 약한 근거(크기 동률 해소)로만 찾음 — 자동확정 보류')
        return r


def fmtv(v):
    return ('%#x' % v) if isinstance(v, int) else str(v)


class _CsShim:
    """gen_sites.dynamic_instruction 은 capstone insn 을 받는다 — 같은 바이트를 다시 디스어셈해 넘긴다."""

    def __new__(cls, x):
        return next(MD.disasm(x.b, BASE + x.rva))


# ───────────────────────────────────────────────────────── 초안·보고서
def build_drafts(base, wres, tres, new_sha, label):
    T = copy.deepcopy(base['targets_raw'])
    names = base['target_files']

    def set_sha(doc):
        if 'exe_sha256' in doc:
            doc['exe_sha256'] = new_sha.lower() if doc['exe_sha256'] == doc['exe_sha256'].lower() else new_sha.upper()
        if 'game_version' in doc:
            doc['game_version'] = label

    for d in T.values():
        set_sha(d)
    new_sites = {}
    subj_old = T['subjects']['sites']
    rekey = {}
    for w, r in zip(base['windows'], wres):
        p = r['proposal']
        review = None if r['auto'] else '; '.join(r['reasons'])[:1500]
        if w['source'] == 'knobs':
            site = T['knobs']['knobs'][w['t_index']]['sites'][w['s_index']]
            if p.get('value_rva') is not None:
                site['rva'] = '%#x' % p['value_rva']
                site['asm'] = p['value_asm']
            if review:
                site['review'] = '%s — %s' % (r['code'], review)
            ent = copy.deepcopy(subj_old[w['subj_key']])
            if p.get('value_rva') is not None:
                if 'patch_start' in ent:
                    ent['patch_start'] = '%#x' % p['rva']
                if 'prefix_hex' in ent and p.get('prefix_hex'):
                    ent['prefix_hex'] = p['prefix_hex']
                if 'src' in p:
                    ent['src'] = p['src']
                if 'team_chk' in ent:
                    ent['team_chk'] = p.get('team_chk')
            if review:
                ent['review'] = '%s — %s' % (r['code'], review)
            nk = ('%#x' % p['value_rva']) if p.get('value_rva') is not None else w['subj_key']
            rekey[w['subj_key']] = (nk, ent)
        else:
            site = T[w['source']]['tactics'][w['t_index']]['sites'][w['s_index']]
            if p.get('rva') is not None:
                old_win = site['win_hex']
                site['rva'] = '%#x' % p['rva']
                if p['win_hex'] != old_win:
                    site['win_hex'] = p['win_hex']
                    if 'first_asm' in site:
                        site['first_asm'] = p['first_asm']
                    if 'original_asm' in site:
                        site['original_asm'] = p['first_asm']
                if site.get('first_len') != p['first_len']:
                    site['first_len'] = p['first_len']
                if p.get('dynamic_hex') and p['dynamic_hex'] != site.get('dynamic_hex'):
                    site['dynamic_hex'] = p['dynamic_hex']
                site['src'] = p.get('src')
                if 'team_chk' in site:
                    site['team_chk'] = p.get('team_chk')
                if 'function' in site:
                    f_old = site['function']
                    nf = ['%#x' % p['function'][0], '%#x' % p['function'][1]]
                    if int(f_old[0], 16) != p['function'][0] or int(f_old[1], 16) != p['function'][1]:
                        site['function'] = nf
            if review:
                site['review'] = '%s — %s' % (r['code'], review)
    # subjects 재키(원래 순서 보존)
    for k_old, v in subj_old.items():
        if k_old in rekey:
            nk, ent = rekey[k_old]
            new_sites[nk] = ent
        else:
            new_sites[k_old] = v
    T['subjects']['sites'] = new_sites
    # 전술 단위 기본값 변화
    for t, tr in zip(base['tactics'], tres):
        if not tr['value_changes']:
            continue
        nds = sorted({v['new_default'] for v in tr['value_changes'] if v['new_default'] is not None}, key=str)
        msg = 'VALUE_CHANGED — 게임 기본값 %s → %s · 낮게/높게·min/max 재검토' % (t['default'], (nds[0] if len(nds) == 1 else nds) if nds else '?')
        if t['source'] == 'knobs':
            k = next(x for x in T['knobs']['knobs'] if x['id'] == t['knob'])
            if len(nds) == 1:
                k['default'] = nds[0]
            k['review'] = msg + ' · tactics.rs 기본32 표(손으로 쓴 default)도 수정 필요'
        else:
            et = next(x for x in T[t['source']]['tactics'] if x['audit_id'] == t['audit_id'])
            if len(nds) == 1:
                et['default'] = nds[0]
            et['review'] = msg
    out = {}
    for key, nm in names.items():
        new_nm = re.sub(r'_[0-9.]+\.json$', '_%s.json' % label, nm)
        out[new_nm] = T[key]
    return out


def tactic_summary(base, wres):
    out = []
    by_t = collections.defaultdict(list)
    for w, r in zip(base['windows'], wres):
        by_t[w['tactic']].append((w, r))
    for t in base['tactics']:
        rows = by_t[t['index']]
        codes = collections.Counter(r['code'] for _w, r in rows)
        auto = all(r['auto'] for _w, r in rows)
        reasons = []
        for w, r in rows:
            for x in r['reasons']:
                reasons.append('%s: %s' % (w['name'], x))
        vchg = []
        for w, r in rows:
            for v in r['detail'].get('values', []):
                if v['old_imm'] != v['new_imm']:
                    vchg.append(dict(v, window=w['name']))
        verdict = 'AUTO' if auto else 'REVIEW'
        out.append({'index': t['index'], 'id': t['id'], 'name': t['name'], 'tab': t['tab'], 'visible': t['visible'],
                    'source': t['source'], 'audit_id': t['audit_id'], 'verdict': verdict,
                    'codes': dict(codes), 'worst': next((c for c in PRIORITY if c in codes), 'SAME'),
                    'reasons': reasons, 'value_changes': vchg,
                    'windows': [{'name': w['name'], 'old_rva': w['rva'], 'new_rva': hex(r['new_rva']) if r['new_rva'] is not None else None,
                                 'code': r['code'], 'auto': r['auto']} for w, r in rows]})
    return out


def write_reports(outdir, base, header, wres, tres, owners, drafts):
    outdir.mkdir(parents=True, exist_ok=True)
    dd = outdir / 'draft_targets'
    dd.mkdir(exist_ok=True)
    for nm, doc in drafts.items():
        (dd / nm).write_text(json.dumps(doc, ensure_ascii=False, indent=1) + '\n', encoding='utf-8', newline='\n')
    def clean(r):
        r = dict(r)
        r['old_rva'] = r['old_rva'] if isinstance(r['old_rva'], str) else hex(r['old_rva'])
        r['old_value_rva'] = r['old_value_rva'] if isinstance(r['old_value_rva'], str) else hex(r['old_value_rva'])
        r['new_rva'] = hex(r['new_rva']) if isinstance(r['new_rva'], int) else r['new_rva']
        r['new_value_rva'] = hex(r['new_value_rva']) if isinstance(r['new_value_rva'], int) else r['new_value_rva']
        p = dict(r['proposal'])
        for kk in ('rva', 'value_rva'):
            if isinstance(p.get(kk), int):
                p[kk] = hex(p[kk])
        if 'function' in p and p['function']:
            p['function'] = [hex(x) for x in p['function']]
        r['proposal'] = p
        return r
    wj = [clean(r) for r in wres]
    full = dict(header, tactics=tres, windows=wj, owners=owners)
    (outdir / 'track.json').write_text(json.dumps(full, ensure_ascii=False, indent=1, default=str) + '\n',
                                       encoding='utf-8', newline='\n')
    rq = [dict(t, window_detail=[w for w in wj if w['tactic'] == t['index'] and not w['auto']])
          for t in tres if t['verdict'] != 'AUTO']
    (outdir / 'review_queue.json').write_text(json.dumps(rq, ensure_ascii=False, indent=1, default=str) + '\n',
                                              encoding='utf-8', newline='\n')
    # ── track.md
    L = []
    w_ = L.append
    w_('# 상세전술 패치데이 추적 — %s → %s' % (header['old_ver'], header['new_ver']))
    w_('')
    w_('- 생성 %s · track_update.py v%d' % (header['created'], TOOL_VER))
    w_('- 구 exe `%s` sha256 `%s`' % (header['old_exe'], header['old_sha256']))
    w_('- 신 exe `%s` sha256 `%s`' % (header['new_exe'], header['new_sha256']))
    w_('- 입력 해시: ' + ' · '.join('`%s` %s' % (Path(k).name, v[:16]) for k, v in header['inputs'].items()))
    w_('')
    tc = collections.Counter(t['verdict'] for t in tres)
    wc = collections.Counter(r['code'] for r in wres)
    w_('## 요약')
    w_('')
    wt = collections.Counter(t['worst'] for t in tres)
    w_('**전술 %d개 = AUTO(자동확정) %d · REVIEW(검토 필요) %d** · 창 %d개 중 자동확정 %d' % (
        len(tres), tc.get('AUTO', 0), tc.get('REVIEW', 0), len(wres), sum(1 for r in wres if r['auto'])))
    w_('')
    w_('| 판정 | 뜻 | 전술 수(가장 나쁜 창 기준) | 창 수 |')
    w_('|---|---|---|---|')
    mean = {'SAME': '그대로(이동만)', 'REG_RENAME': '레지스터·스택·프레임만 바뀜 → 주체 재지정',
            'VALUE_CHANGED': '게임이 기본값을 바꿈', 'COPIES': '사본 수가 바뀜',
            'LOGIC_CHANGED': '로직(비교·분기) 바뀜', 'SITE_MISSING': '창(또는 Caller 콜사이트) 못 찾음',
            'OWNER_MISSING': '함수 자체를 못 찾음'}
    for c in ['SAME'] + PRIORITY[::-1]:
        w_('| %s | %s | %d | %d |' % (c, mean[c], wt.get(c, 0), wc.get(c, 0)))
    nb = sum(1 for r in wres if r['code'] == 'SAME' and not r['auto'])
    if nb:
        w_('')
        w_('> SAME 이지만 자동확정 보류(owner 대응 불일치·약한 근거·repin 바이트 충돌) 창 %d개 — review_queue.json 의 BLOCK 사유' % nb)
    oc = collections.Counter(re.sub(r'^(SHIFTED|EDITED)\(.*', r'\1*', str(o['loc_verdict'] or o['loc_how'])) for o in owners)
    w_('')
    w_('owner %d개 — Location 판정: %s · repin 일치 %d / 불일치 %d / repin 없음 %d' % (
        len(owners), ' · '.join('%s %d' % kv for kv in sorted(oc.items(), key=lambda x: str(x[0]))),
        sum(1 for o in owners if o['agree'] is True), sum(1 for o in owners if o['agree'] is False),
        sum(1 for o in owners if o['agree'] is None)))
    w_('')
    w_('## 게임이 기본값을 바꾼 전술')
    w_('')
    vt = [t for t in tres if t['value_changes']]
    if not vt:
        w_('- 없음')
    else:
        w_('| 탭 | 이름 | 구 기본값 → 신 | 창 |')
        w_('|---|---|---|---|')
        for t in vt:
            for v in t['value_changes']:
                w_('| %s | %s | %s → **%s** (즉치 %s→%s, %s)%s | `%s` |' % (
                    t['tab'] or '(숨김)', t['name'], v['old_default'], v['new_default'], v['old_imm'], v['new_imm'], v['expr'],
                    (' ⚠%s' % v['suspect']) if v.get('suspect') else '', v['window']))
    w_('')
    w_('## 검토 필요 전술')
    w_('')
    rv = [t for t in tres if t['verdict'] != 'AUTO']
    if not rv:
        w_('- 없음 — 전 전술 자동확정')
    for t in rv:
        w_('### [%d] %s (`%s`) — %s · %s' % (t['index'], t['name'], t['id'], t['tab'] or '(숨김)', t['worst']))
        w_('')
        for x in t['reasons'][:24]:
            w_('- %s' % x)
        if len(t['reasons']) > 24:
            w_('- … 외 %d건 (review_queue.json)' % (len(t['reasons']) - 24))
        for w in [w for w in wj if w['tactic'] == t['index'] and not w['auto']]:
            p = w['proposal']
            if p:
                w_('- 제안 `%s`: rva %s · src `%s` · team_chk `%s`' % (
                    w['name'], p.get('rva'), json.dumps(p.get('src'), ensure_ascii=False), json.dumps(p.get('team_chk'), ensure_ascii=False)))
        w_('')
    w_('## 전술 전량')
    w_('')
    w_('| # | 탭 | 이름 | 표시 | 판정 | 창 (구 RVA → 신 RVA : 판정) |')
    w_('|---|---|---|---|---|---|')
    for t in tres:
        w_('| %d | %s | %s | %s | %s | %s |' % (
            t['index'], t['tab'] or '-', t['name'], 'O' if t['visible'] else '숨김', t['verdict'],
            '<br>'.join('%s → %s : %s' % (x['old_rva'], x['new_rva'] or '없음', x['code']) for x in t['windows'])))
    w_('')
    (outdir / 'track.md').write_text('\n'.join(L) + '\n', encoding='utf-8', newline='\n')


def cmd_track(a):
    base = json.loads(Path(a.base).read_text(encoding='utf-8'))
    if base.get('tool_ver') != TOOL_VER:
        raise RuntimeError('기준선 tool_ver %s ≠ %s — snapshot 다시' % (base.get('tool_ver'), TOOL_VER))
    T = Tracker(a, base)
    wres = []
    for w in base['windows']:
        wres.append(T.window(w))
    tres = tactic_summary(base, wres)
    owners = []
    for ofn, r in sorted(T._own.items()):
        o = {kk: vv for kk, vv in r.items() if kk != '_new'}
        o['old'] = [hex(x) for x in o['old']]
        for kk in ('new', 'repin', 'loc_new'):
            if o.get(kk):
                o[kk] = [hex(x) for x in o[kk]]
        owners.append(o)
    inputs = {str(a.base): sha256_file(a.base), str(a.old_pkl): sha256_file(a.old_pkl), str(a.new_pkl): sha256_file(a.new_pkl),
              str(a.old_cg): sha256_file(a.old_cg), str(a.new_cg): sha256_file(a.new_cg)}
    header = {'tool': 'track_update.py', 'tool_ver': TOOL_VER, 'created': datetime.datetime.now().isoformat(timespec='seconds'),
              'old_ver': base['game_version'], 'new_ver': a.new_ver, 'old_exe': str(a.old_exe), 'new_exe': str(a.new_exe),
              'old_sha256': T.O.sha, 'new_sha256': T.N.sha, 'inputs': inputs,
              'counts': {'tactics': len(tres), 'auto': sum(1 for t in tres if t['verdict'] == 'AUTO'),
                         'review': sum(1 for t in tres if t['verdict'] != 'AUTO'),
                         'windows': dict(collections.Counter(r['code'] for r in wres)),
                         'windows_auto': sum(1 for r in wres if r['auto'])}}
    drafts = build_drafts(base, wres, tres, T.N.sha, a.new_ver)
    write_reports(Path(a.out), base, header, wres, tres, owners, drafts)
    c = header['counts']
    print('전술 AUTO %d · REVIEW %d · 창 %s → %s' % (c['auto'], c['review'], c['windows'], a.out))
    return 0 if c['review'] == 0 else 1


def main():
    ap = argparse.ArgumentParser(description='상세전술 패치데이 추적기')
    sp = ap.add_subparsers(dest='cmd', required=True)
    s1 = sp.add_parser('snapshot')
    s1.add_argument('--exe', required=True)
    s1.add_argument('--ver', required=True)
    s1.add_argument('--pkl', required=True)
    s1.add_argument('--cg', required=True)
    s1.add_argument('--out', required=True)
    s2 = sp.add_parser('track')
    s2.add_argument('--base', required=True)
    s2.add_argument('--old-exe', required=True, dest='old_exe')
    s2.add_argument('--new-exe', required=True, dest='new_exe')
    s2.add_argument('--new-ver', required=True, dest='new_ver')
    s2.add_argument('--old-pkl', required=True, dest='old_pkl')
    s2.add_argument('--new-pkl', required=True, dest='new_pkl')
    s2.add_argument('--old-cg', required=True, dest='old_cg')
    s2.add_argument('--new-cg', required=True, dest='new_cg')
    s2.add_argument('--out', required=True)
    a = ap.parse_args()
    try:
        if a.cmd == 'snapshot':
            return cmd_snapshot(a)
        return cmd_track(a)
    except SystemExit as e:          # 내부 검증 실패(문자열) = 실행 오류
        if isinstance(e.code, int) or e.code is None:
            return 2 if e.code else 0
        print('[오류] %s' % e.code, file=sys.stderr)
        return 2
    except Exception:
        traceback.print_exc()
        return 2


if __name__ == '__main__':
    sys.exit(main())
