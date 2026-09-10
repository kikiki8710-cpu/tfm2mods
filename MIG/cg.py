#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""cg.py - exe / 내 DLL 의 **직접 호출 그래프**를 만들어 캐시한다(dllmatch 2안의 보강재).

왜 필요한가:
  오프셋 지문은 **큰 함수에만 통한다**. 841B 짜리 캐시 래퍼는 지문 원소가 4개뿐이라
  원리적으로 판별이 안 된다(실측: `position_eval_at` 이 그래서 미매칭으로 남았다).
  그런데 그 래퍼의 정체는 **호출 관계로는 즉시** 드러난다 -
  0xd84db0 이 0xd851d0 을 부르고, 0xd851d0 의 호출자는 그 하나뿐이었다.
  => 지문으로 큰 함수를 고정(앵커)하고, 그래프를 타고 작은 함수로 이름을 전파한다.

경계는 전부 **.pdata(RUNTIME_FUNCTION)** 를 쓴다 - 크기를 추정하지 않는다.
"""
import io
import json
import os
import struct
import sys

import capstone

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

HERE = os.path.dirname(os.path.abspath(__file__))


def pe(path):
    d = open(path, 'rb').read()
    off = struct.unpack_from('<I', d, 0x3c)[0]
    nsec = struct.unpack_from('<H', d, off + 6)[0]
    optsz = struct.unpack_from('<H', d, off + 20)[0]
    base = struct.unpack_from('<Q', d, off + 24 + 24)[0]
    secs = [struct.unpack_from('<IIII', d, off + 24 + optsz + i * 40 + 8) for i in range(nsec)]
    exc = struct.unpack_from('<II', d, off + 24 + 112 + 3 * 8)
    return d, base, secs, exc


def make_off(secs):
    def f(rva):
        for vsz, va, rsz, ra in secs:
            if va <= rva < va + max(vsz, rsz):
                return ra + rva - va
        return None
    return f


def func_ranges(d, secs, exc):
    va, size = exc
    o = make_off(secs)(va)
    if o is None:
        return {}
    out = {}
    for i in range(size // 12):
        b, e, _u = struct.unpack_from('<III', d, o + i * 12)
        if e > b:
            out[b] = max(out.get(b, 0), e - b)
    return out


def build(path, cache):
    """rva -> [callee rva]. 대상은 .pdata 에 있는 함수만(=경계가 확실한 것)."""
    if os.path.exists(cache):
        raw = json.load(open(cache, encoding='utf-8'))
        return {int(k): v for k, v in raw.items()}
    d, base, secs, exc = pe(path)
    fr = func_ranges(d, secs, exc)
    off = make_off(secs)
    cs = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    known = set(fr)
    g = {}
    for rva, sz in fr.items():
        o = off(rva)
        if o is None or sz <= 0 or sz > 0x20000:
            continue
        outs = set()
        for ins in cs.disasm(d[o:o + sz], rva):
            # 직접 호출만 쓴다. 간접(call rax)·가상호출은 대상이 정적으로 안 보인다.
            if ins.mnemonic == 'call' and ins.op_str.startswith('0x'):
                t = int(ins.op_str, 16)
                if t in known and t != rva:
                    outs.add(t)
            # 꼬리 점프(tail call)도 호출로 친다 - **자기 함수 밖으로** 뛰는 것만.
            elif ins.mnemonic == 'jmp' and ins.op_str.startswith('0x'):
                t = int(ins.op_str, 16)
                if t in known and not (rva <= t < rva + sz):
                    outs.add(t)
        if outs:
            g[rva] = sorted(outs)
    json.dump({str(k): v for k, v in g.items()}, open(cache, 'w'), separators=(',', ':'))
    return g


if __name__ == '__main__':
    EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
    DLL = os.path.join(os.environ.get('TEMP', r'C:\Temp'), 'tfm2_map3', 'm.dll')
    import time
    for tag, p, c in (('exe', EXE, os.path.join(HERE, 'cg_exe.json')),
                      ('dll', DLL, os.path.join(HERE, 'cg_dll.json'))):
        t0 = time.time()
        g = build(p, c)
        print('%s: 노드 %d개 · 엣지 %d개 · %.1fs -> %s'
              % (tag, len(g), sum(len(v) for v in g.values()), time.time() - t0, c))
