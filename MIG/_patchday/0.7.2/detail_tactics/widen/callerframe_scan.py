# -*- coding: utf-8 -*-
"""승격본 sites.rs 의 Src/team_chk 중 「호출자 프레임」에 기대는 것을 찾아 0.7.1 호출자에서 그 슬롯이 실제로 쓰이는지 검사(읽기 전용)."""
import sys, re, struct, collections, pickle, os
sys.path.insert(0, r'F:\claude\tfm2mods\MIG'); import aidiff as A
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
N = A.Img(r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe')
md = Cs(CS_ARCH_X86, CS_MODE_64)
S = open(r'F:\claude\tfm2mods\tfm2_detail_tactics\src\sites.rs', encoding='utf-8').read()
cache = os.path.join(os.path.dirname(os.path.abspath(__file__)), '_callmap072.pkl')
if os.path.exists(cache):
    CM = pickle.load(open(cache, 'rb'))
else:
    CM = collections.defaultdict(list)
    for nm, va, sz, pr in N.secs:
        if nm != '.text': continue
        blob = N.data[pr:pr + sz]
        for m in re.finditer(rb'\xe8', blob):
            i = m.start()
            if i + 5 > len(blob): break
            t = va + i + 5 + struct.unpack_from('<i', blob, i + 1)[0]
            CM[t].append(va + i)
    CM = dict(CM); pickle.dump(CM, open(cache, 'wb'))
def prologue(f):
    """rbp 를 프레임으로 세우나 · push 순서 · sub 크기"""
    pushes = []; sub = 0; sets_rbp = False
    for i in md.disasm(N.code(f[0], 0x60), A.BASE + f[0]):
        if i.mnemonic == 'push': pushes.append(i.op_str)
        elif i.mnemonic == 'sub' and i.op_str.startswith('rsp,'):
            try: sub = int(i.op_str.split(',')[1], 0)
            except: sub = -1
        elif i.mnemonic == 'lea' and i.op_str.startswith('rbp, [rsp'): sets_rbp = True; break
        elif i.mnemonic == 'mov' and i.op_str == 'rbp, rsp': sets_rbp = True; break
        elif i.mnemonic == 'call': continue
        elif pushes and i.mnemonic not in ('push', 'sub', 'mov', 'call'): break
    return pushes, sub, sets_rbp
def rbp_writes(f, d):
    w = r = 0
    for i in md.disasm(N.code(f[0], f[1] - f[0]), A.BASE + f[0]):
        o = i.op_str
        tag = 'rbp + 0x%x]' % d if d >= 0 else 'rbp - 0x%x]' % (-d)
        if tag in o:
            if o.split(',')[0].strip().endswith(tag) and i.mnemonic.startswith('mov'): w += 1
            else: r += 1
    return w, r
rows = []
for m in re.finditer(r'Patch \{ name: "([^"]+)", tactic: (\d+), rva: (0x[0-9a-f]+),.*?src: (.*?), team_chk: (.*?), first_len', S):
    name, t, rva, src, tc = m.groups(); rva = int(rva, 16)
    f = N.owner(rva)
    pushes, sub, sets_rbp = prologue(f)
    exprs = []
    for kind, blob in (('src', src), ('team', tc)):
        if 'Caller(' in blob: continue  # Caller 사례는 B 판정에서 개별 확정됨 — 별도
        for mm in re.finditer(r'Src::Mem\(5, (-?0x[0-9a-f]+)\)', blob):
            if not sets_rbp: exprs.append((kind, 'rbp', int(mm.group(1), 16)))
        for mm in re.finditer(r'Src::StackMem\((0x[0-9a-f]+), (0x[0-9a-f]+)\)', blob):
            a, b = int(mm.group(1), 16), int(mm.group(2), 16)
            # 저장 rbp 위치 = 창 시점 rsp 기준(프롤로그 기준 근사: sub + 8*(len(pushes)-1-idx(rbp)))
            if 'rbp' in pushes:
                k = len(pushes) - 1 - pushes.index('rbp')
                if a == sub + 8 * k: exprs.append((kind, 'saved_rbp', b))
    if not exprs: continue
    callers = CM.get(f[0], [])
    cinfo = []
    for c in callers:
        cf = N.owner(c)
        if not cf: continue
        cp = prologue(cf)
        ws = [(e[0], e[2], rbp_writes(cf, e[2])) for e in exprs]
        cinfo.append((hex(c + 5), hex(cf[0]), 'rbp프레임' if cp[2] else 'rbp미설정', ws))
    rows.append((int(t), name, hex(f[0]), sets_rbp, exprs, cinfo))
for r in rows:
    t, name, own, sr, ex, ci = r
    bad = any(w == 0 for c in ci for (_, _, (w, rr)) in c[3]) or any(c[2] == 'rbp미설정' for c in ci) or not ci
    print(('★' if bad else ' '), t, name, 'owner', own, 'exprs', [(k, s, hex(d)) for k, s, d in ex])
    for c in ci: print('      ret', c[0], 'in', c[1], c[2], [(k, hex(d), 'w%d/r%d' % wr) for k, d, wr in c[3]])
