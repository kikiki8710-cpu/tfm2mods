# -*- coding: utf-8 -*-
"""디스패처 arm → 호출 대상(핸들러) → 소스파일 매핑.

  python armmap.py arms 054 e52990        # 그 함수의 JT arm 별 첫 call/jmp 타깃
  python armmap.py callers 054 e52990     # 그 함수를 call 하는 사이트 전수(direct rel32)
"""
import io, os, struct, sys, bisect
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE
from jtscan import scan, entries, srcmap, src_of, fast_func_of


def follow(e, start, fn, limit=200):
    """arm 시작에서 흘러가며 만나는 (call 타깃, 메모 write) 요약."""
    calls = []
    writes = []
    end_note = ''
    addr = start
    steps = 0
    while steps < limit:
        ins = e.dis(addr, 64)
        if not ins:
            break
        i = ins[0]
        m, o = i.mnemonic, i.op_str
        if m == 'call' and o.startswith('0x'):
            calls.append(int(o, 16) - BASE)
        if m.startswith('mov') and o.startswith('qword ptr [rsi],'):
            writes.append(o.split(',')[-1].strip())
        if m.startswith('mov') and o.startswith('byte ptr [rsi + 0xa],'):
            writes.append('B@a=' + o.split(',')[-1].strip())
        if m == 'jmp':
            if o.startswith('0x'):
                t = int(o, 16) - BASE
                if fn[0] <= t < fn[1] and t > addr:
                    addr = t; steps += 1; continue
                end_note = 'jmp %06x' % t
            else:
                end_note = 'jmp %s' % o
            break
        if m == 'ret':
            end_note = 'ret'; break
        addr += i.size
        steps += 1
        if len(calls) >= 3:
            break
    return calls, writes, end_note


def callers(e, target):
    txt = [s for s in e.sections if s[0] == '.text'][0]
    _, va, vsz, ra, rsz = txt
    d = e.raw[ra:ra + rsz]
    out = []
    n = len(d)
    for i in range(n - 5):
        if d[i] == 0xE8:
            rel = struct.unpack_from('<i', d, i + 1)[0]
            if va + i + 5 + rel == target:
                out.append(va + i)
    return out


if __name__ == '__main__':
    a = sys.argv[1:]
    ver = a[1]
    e = load(ver)
    sm = srcmap(ver)
    if a[0] == 'arms':
        rva = int(a[2], 16)
        fn = fast_func_of(e, rva)
        for jmp, jt, br, ir, lea, mv in scan(e):
            if fn[0] <= jmp < fn[1]:
                ent = entries(e, jt, fn)
                print('디스패처 %06x  JT %06x  엔트리 %d' % (fn[0], jt, len(ent)))
                for k, t in enumerate(ent):
                    c, w, note = follow(e, t, fn)
                    ss = []
                    for cc in c:
                        cf = fast_func_of(e, cc)
                        s = src_of(sm, cf[0])[0] if cf else ''
                        ss.append('%06x %s' % (cc, s if s else '?'))
                    print('  arm%-2d %06x | write=%s | call=%s | %s' % (
                        k, t, ','.join(w) if w else '-', ' ; '.join(ss) if ss else '-', note))
    elif a[0] == 'callers':
        t = int(a[2], 16)
        for c in callers(e, t):
            cf = fast_func_of(e, c)
            print('%06x  in fn %06x  %s' % (c, cf[0] if cf else 0,
                                            src_of(sm, cf[0])[0] if cf else ''))
