# -*- coding: utf-8 -*-
"""sites.rs 의 Caller 형식 Src/team_chk 사례를 전수 검사(callerframe_scan.py 가 건너뛰는 부분의 짝, 읽기 전용).

사례마다:
  ① ret_rva-5 가 E8 call 이고 그 대상이 창 owner(바깥 단계) 또는 중첩 단계의 래퍼인가
  ② return_off == owner 프롤로그(push*8 + sub) [+ 래퍼 프레임]  (chkstk owner 는 「?」)
  ③ 하위 Src 가 호출자 프레임을 읽는 경우
     - StackMem(a,b): a == owner 의 저장 rbp 자리(래퍼가 rbp 를 안 바꾸면 그대로) 이고,
       호출자 함수가 rbp 프레임을 세우며 rbp 를 덮지 않고, [rbp+b] 쓰기 ≥1 (chkstk 스택 인자면 쓰기 0·읽기 ≥1 허용 = 'arg')
     - Mem(5,d): owner 가 rbp 프레임을 세우고 [rbp+d] 쓰기 ≥1
     - Stack(d): owner 프롤로그의 push 슬롯이면 어떤 레지스터인지 출력(수동 판정)
문제 = '★'. 사용: python -X utf8 callercase_scan.py [sites.rs]
"""
import sys, re, struct
sys.path.insert(0, r'F:\claude\tfm2mods\MIG'); import aidiff as A
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
N = A.Img(r'F:\claude\tfm2\tfm2_0.7.0\TeamfightManager2.exe'); md = Cs(CS_ARCH_X86, CS_MODE_64)
S = open(sys.argv[1] if len(sys.argv) > 1 else r'F:\claude\tfm2mods\tfm2_detail_tactics\src\sites.rs', encoding='utf-8').read()


def prologue(fs):
    """(pushes, sub, chkstk여부, rbp프레임여부, rbp_from_entry) — chkstk 면 sub = mov eax 크기."""
    pushes = []; sub = None; chk = False; sets_rbp = False; eax = None; rbp_e = None
    for i in md.disasm(N.code(fs, 0x60), A.BASE + fs):
        if i.mnemonic == 'push': pushes.append(i.op_str)
        elif i.mnemonic == 'mov' and i.op_str.startswith('eax,'): eax = int(i.op_str.split(',')[1], 0)
        elif i.mnemonic == 'call' and eax is not None: chk = True
        elif i.mnemonic == 'sub' and i.op_str.startswith('rsp,'):
            v = i.op_str.split(',')[1].strip()
            sub = eax if v == 'rax' else int(v, 0)
        elif i.mnemonic == 'lea' and i.op_str.startswith('rbp, [rsp'):
            m = re.search(r'\+ (0x[0-9a-f]+|\d+)\]', i.op_str); disp = int(m.group(1), 0) if m else 0
            sets_rbp = True; rbp_e = -(8 * len(pushes) + (sub or 0)) + disp; break
        elif i.mnemonic == 'mov' and i.op_str == 'rbp, rsp':
            sets_rbp = True; rbp_e = -(8 * len(pushes) + (sub or 0)); break
        elif pushes and i.mnemonic not in ('push', 'sub', 'mov', 'call'): break
    return pushes, sub, chk, sets_rbp, rbp_e


def is_arg(fs, d):
    """[rbp+d] 가 그 함수의 스택 인자(5번째 이후) 자리인가."""
    p = prologue(fs)
    return p[3] and p[4] is not None and p[4] + d >= 0x28


def rbp_info(fs, fe, d):
    w = r = 0; clob = []
    tag = 'rbp + 0x%x]' % d if d >= 0 else 'rbp - 0x%x]' % (-d)
    ins = list(md.disasm(N.code(fs, fe - fs), A.BASE + fs))
    for k, i in enumerate(ins):
        o = i.op_str
        if tag in o:
            if o.split(',')[0].strip().endswith(tag) and i.mnemonic.startswith('mov'): w += 1
            else: r += 1
        if k > 14 and re.match(r'(rbp|ebp|bpl),', o) and i.mnemonic not in ('cmp', 'test'): clob.append(hex(i.address - A.BASE))
    return w, r, clob


def call_target(ret):
    c = ret - 5
    b = N.code(c, 5)
    if b[0] != 0xe8: return None
    return c + 5 + struct.unpack_from('<i', b, 1)[0]


# ---- 아주 작은 Src 파서
def parse(s, i=0):
    s = s
    m = re.compile(r'\s*Src::(\w+)').match(s, i); kind = m.group(1); i = m.end()
    if kind == 'None': return ('None',), i
    if kind == 'Caller':
        m = re.compile(r'\((0x[0-9a-f]+|\d+), &\[').match(s, i); off = int(m.group(1), 0); i = m.end(); cases = []
        while True:
            m = re.compile(r'\s*Caller \{ ret_rva: (0x[0-9a-f]+), src: ').match(s, i)
            if not m: break
            ret = int(m.group(1), 16); sub, i = parse(s, m.end())
            i = re.compile(r'\s*\},?').match(s, i).end(); cases.append((ret, sub))
        i = re.compile(r'\s*\]\)').match(s, i).end()
        return ('Caller', off, cases), i
    m = re.compile(r'\(([^()]*)\)').match(s, i)
    if m:
        args = [int(x.strip(), 0) for x in m.group(1).split(',')]; return (kind, *args), m.end()
    m = re.compile(r' \{[^}]*\}').match(s, i)
    return (kind,), m.end()


import collections
CM = collections.defaultdict(list)
for nm, va, sz, pr in N.secs:
    if nm != '.text': continue
    blob = N.data[pr:pr + sz]
    for mm in re.finditer(b'\xe8', blob):
        i = mm.start()
        if i + 5 > len(blob): break
        CM[va + i + 5 + struct.unpack_from('<i', blob, i + 1)[0]].append(va + i)

rows = []
for m in re.finditer(r'Patch \{ name: "([^"]+)", tactic: \d+, rva: (0x[0-9a-f]+),.*?src: (Src::.*?), team_chk: (Src::.*?), first_len', S):
    name, rva = m.group(1), int(m.group(2), 16)
    for kind, blob in (('src', m.group(3)), ('team', m.group(4))):
        if 'Caller(' not in blob: continue
        tree, _ = parse(blob)
        rows.append((name, rva, kind, tree))

bad = 0; seen = 0
for name, rva, kind, tree in rows:
    own = N.owner(rva); p, sub, chk, sr, _ = prologue(own[0])
    frame0 = None if sub is None else len(p) * 8 + sub
    saved0 = (sub + 8 * (len(p) - 1 - p.index('rbp'))) if ('rbp' in p and sub is not None) else None

    def walk(node, callee_fs, frame, saved_rbp, depth, path):
        global bad, seen
        _, off, cases = node
        okoff = (frame == off) if frame is not None else None
        for ret, sub_src in cases:
            seen += 1
            tgt = call_target(ret); cf = N.owner(ret - 5)
            msgs = []; star = False
            if tgt != callee_fs: msgs.append('call대상 %s≠%x' % (hex(tgt) if tgt else None, callee_fs)); star = True
            if okoff is False: msgs.append('return_off %#x≠프레임 %#x' % (off, frame)); star = True
            if okoff is None: msgs.append('return_off ?(chkstk)')
            k = sub_src[0]
            if k == 'Caller':
                # 중첩: 이 호출자 함수(cf) 는 래퍼. 래퍼 프레임을 더해 다음 단계로.
                wp, wsub, wchk, wsr, _ = prologue(cf[0])
                wclob = rbp_info(cf[0], cf[1], 0)[2]
                if wclob or 'rbp' in wp: msgs.append('래퍼 rbp 변경 %s' % wclob)
                nf = None if frame is None or wsub is None else off + 8 + len(wp) * 8 + wsub
                print(('★' if star else ' '), name, kind, path + ['%x' % ret], ' ; '.join(msgs) or 'ok', '→ 중첩')
                bad += star
                walk(sub_src, cf[0], nf, saved_rbp, depth + 1, path + ['%x' % ret])
                continue
            cp, csub, cchk, csr, _ = prologue(cf[0])
            if k == 'StackMem':
                a, b = sub_src[1], sub_src[2]
                frames = [cf]
                if saved_rbp is not None and a == saved_rbp:
                    pass
                elif saved_rbp is None and frame is not None and 'rbp' in cp and csub is not None:
                    # owner 가 rbp 를 안 푸시 → 호출자 cf 가 푸시한 저장 rbp = 조부 호출자의 rbp
                    cs = frame + 8 + csub + 8 * (len(cp) - 1 - cp.index('rbp'))
                    if a != cs: msgs.append('a=%#x≠호출자 저장rbp %#x' % (a, cs)); star = True
                    else: msgs.append('2단: 저장rbp=호출자 %x 의 push' % cf[0])
                    frames = [N.owner(c) for c in CM.get(cf[0], [])]
                    if not frames: msgs.append('조부 호출자 없음'); star = True
                else:
                    msgs.append('a=%#x≠저장rbp %s' % (a, hex(saved_rbp) if saved_rbp is not None else '?')); star = True
                for fr in frames:
                    w, r, clob = rbp_info(fr[0], fr[1], b); fp = prologue(fr[0]); arg = is_arg(fr[0], b)
                    if not fp[3]: msgs.append('%x rbp프레임 아님' % fr[0]); star = True
                    if clob: msgs.append('%x rbp 덮어씀 %s' % (fr[0], clob)); star = True
                    if w == 0 and not (arg and r > 0): msgs.append('%x [rbp+%#x] 쓰기0' % (fr[0], b)); star = True
                    msgs.append('%x [rbp+%#x] w%d/r%d%s' % (fr[0], b, w, r, ' (스택인자)' if arg else ''))
            elif k == 'Mem' and sub_src[1] == 5:
                d = sub_src[2]
                fr = own if sr else cf   # owner 가 rbp 를 안 세우면 rbp = 이 단계 호출자의 rbp
                if fr is cf and depth == 0 and False: pass
                w, r, clob = rbp_info(fr[0], fr[1], d); fp = prologue(fr[0]); arg = is_arg(fr[0], d)
                if not fp[3] or (fr is cf and clob) or (w == 0 and not (arg and r > 0)): star = True
                msgs.append('%s %x [rbp+%#x] w%d/r%d%s%s' % ('owner' if fr is own else '호출자', fr[0], d, w, r,
                            ' (스택인자)' if arg else '', (' clob%s' % clob) if (fr is cf and clob) else ''))
            elif k == 'Stack':
                d = sub_src[1]
                if sub is not None and not chk and d >= sub and d < frame0:
                    reg = p[len(p) - 1 - (d - sub) // 8]
                    msgs.append('stack:%#x = owner 저장 %s (호출 시점 호출자 %s 값 — 수동 확인)' % (d, reg, reg))
                else:
                    msgs.append('stack:%#x (owner 지역/인자 — 수동 확인)' % d)
            else:
                msgs.append('하위 %s' % (sub_src,))
            print(('★' if star else ' '), name, kind, path + ['%x' % ret], ' ; '.join(msgs))
            bad += star

    walk(tree, own[0], frame0, saved0, 0, [])
print('사례 %d · ★ %d' % (seen, bad))
