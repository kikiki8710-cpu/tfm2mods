#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""callgraph.py — exe 전 함수의 직접 호출 대상(call/jmp rel32) 인덱스 빌더.

왜: 재링크 패치에서 스켈레톤 지문이 MULTI(동형 함수 다수)로 갈릴 때,
    "이 함수가 누구를 부르는가 / 누가 부르는가" 가 유일한 판별자가 된다.
    UNIQUE 로 확정된 대응(global map)을 통해 callee 집합을 사영해 후보를 채점한다.

사용: python callgraph.py <exe> <fnidx.pkl> <out.pkl>
출력: {'callee': {fn_start: [target,...]}, 'caller': {target: [fn_start,...]}}
"""
import sys, struct, pickle, time, io

from capstone import Cs, CS_ARCH_X86, CS_MODE_64

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = False


def load(p):
    d = open(p, 'rb').read()
    pe = struct.unpack_from('<I', d, 0x3c)[0]
    nsec = struct.unpack_from('<H', d, pe + 6)[0]
    opt = pe + 24
    sectab = opt + struct.unpack_from('<H', d, pe + 20)[0]
    secs = []
    for i in range(nsec):
        o = sectab + i * 40
        nm = d[o:o + 8].rstrip(b'\0').decode(errors='replace')
        vsz, va, rsz, rraw = struct.unpack_from('<IIII', d, o + 8)
        secs.append((nm, va, max(vsz, rsz), rraw, rsz))
    return d, secs


def main(exe, pkl, out):
    d, secs = load(exe)
    P = pickle.load(open(pkl, 'rb'))
    idx = {(int(k, 16) if isinstance(k, str) else k): v for k, v in P['idx'].items()}
    starts = set(idx)

    def roff(rva):
        for nm, va, vsz, rraw, rsz in secs:
            if va <= rva < va + vsz:
                o = rva - va
                return rraw + o if o < rsz else None
        return None

    callee, caller = {}, {}
    t0 = time.time()
    for n, (st, f) in enumerate(sorted(idx.items())):
        o = roff(st)
        if o is None:
            continue
        code = d[o:o + f['size']]
        tg = []
        # e8 rel32 (call) / e9 rel32 (tail jmp) 만 — 바이트 스캔이 아니라 디스어셈으로
        for ins in md.disasm(code, st):
            if ins.mnemonic in ('call', 'jmp') and ins.op_str.startswith('0x'):
                t = int(ins.op_str, 16)
                if t in starts and t != st:
                    tg.append(t)
        if tg:
            u = sorted(set(tg))
            callee[st] = u
            for t in u:
                caller.setdefault(t, []).append(st)
        if n % 20000 == 0 and n:
            print('  ... %d/%d  %ds' % (n, len(idx), time.time() - t0))
    pickle.dump({'callee': callee, 'caller': caller}, open(out, 'wb'), 2)
    print('[saved] %s  callee %d / caller %d  %ds'
          % (out, len(callee), len(caller), time.time() - t0))


if __name__ == '__main__':
    main(*sys.argv[1:4])
