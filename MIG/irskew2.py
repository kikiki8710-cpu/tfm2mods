#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""irskew2.py - SDK rlib(IR) 과 배포 exe 의 소스 어긋남을 **필드 오프셋 지문**으로 탐지한다.

왜 오프셋인가 (irskew.py 실패에서 배운 것):
  첫 시도는 **상수 일치**로 판정했다가 실패했다 - 상수는 접히고(`x<N` -> `x<=N-1`),
  인라인되며, 다른 모듈 파일로 흩어진다. 확인된 사례조차 못 잡았다.
  반면 **구조체 필드 오프셋은 최적화 수준과 무관하게 보존된다**.
  실제 사례가 그걸 증명한다:
      IR (SDK rlib) : +1632 / +1640            (미니언 좌표 -> 거리 판정)
      exe (배포본)  : +0x68 / +0x88 / +0x90 / +0x5c0  (상태/종류/대상 -> 타깃 판정)
  겹치는 오프셋이 **하나도 없다**. 이 지표였으면 즉시 잡혔다.

방법:
  1) IR 함수 -> exe RVA 짝짓기
     - 함수 끝 = **같은 파일의 다음 DISubprogram 시작 줄**(DILocation 최대값으로 잡으면
       인라인된 콜리의 줄까지 들어와 범위가 부풀고 후보가 애매해진다)
     - aimap.json 의 패닉 줄이 그 구간에 들어가는 exe 함수가 유일할 때만 판정
  2) IR 오프셋 = 그 함수 **및 인라인되는 헬퍼**의 `getelementptr ... i64 N`
     (헬퍼는 다른 모듈 파일에 있을 수 있어 전 모듈을 색인한다)
  3) exe 오프셋 = 함수 바이트 구간을 디스어셈해 메모리 오퍼랜드의 변위(displacement)
  4) 겹침이 낮으면 skew 의심

판정 한계:
  - 오프셋이 작으면(<0x20) 스택/공용이라 신호가 약하다 -> MIN_OFF 로 자른다.
  - "낮은 겹침 = 의심" 이지 확정이 아니다. 확정은 Ghidra 디컴파일로 한다.

★자체 검증: `--selftest` 는 확인된 사례를 잡고 형제 함수는 안 잡는지 본다.
  이 검증을 통과하지 못하면 **출력을 쓰지 말 것**(첫 시도가 그래서 폐기됐다).
"""
import argparse
import glob
import io
import json
import os
import re
import struct
import sys

import capstone

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
AIMAP = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'aimap.json')
# game_core 도 색인한다 - `iter_minions` 같은 헬퍼가 거기 있어서
# game_ai 만 보면 IR 오프셋 집합이 체계적으로 비어 exe 에만 있는 것처럼 보인다.
IRDIRS = [r'C:	fm2mods\_gaibc', r'C:	fm2mods\_gcbc']

MIN_OFF = 0x20        # 이보다 작은 변위는 스택/공용이라 신호가 약하다
MAX_OFF = 0x4000
MIN_N = 6             # 오프셋이 이보다 적으면 판정하지 않는다(표본 부족)

DEF_RE = re.compile(r'^define[^\n]*@([A-Za-z0-9_.$]+)\(', re.M)
SUB_RE = re.compile(r'!DISubprogram\(name: "([^"]+)", linkageName: "([^"]+)"[^)]*?file: (![0-9]+), line: (\d+)')
FILE_RE = re.compile(r'(![0-9]+) = !DIFile\(filename: "([^"]+)"')
CALL_RE = re.compile(r'(?:call|invoke).*?@([A-Za-z0-9_.$]+)\(')
GEP_RE = re.compile(r'getelementptr[^,\n]*,[^,\n]*,\s*i64 (\d+)')


def sections(d):
    pe = struct.unpack_from('<I', d, 0x3c)[0]
    n = struct.unpack_from('<H', d, pe + 6)[0]
    opt = struct.unpack_from('<H', d, pe + 20)[0]
    return [struct.unpack_from('<IIII', d, pe + 24 + opt + i * 40 + 8) for i in range(n)]


def build():
    """IR 색인 -> {sym: (name, src, start, end)}, {sym: offsets}, {sym: callees}"""
    sym_off, sym_calls, top = {}, {}, {}
    for f in sorted(sum([glob.glob(os.path.join(p, '*.ll')) for p in IRDIRS], [])):
        t = io.open(f, encoding='utf-8', errors='ignore').read()
        fmap = dict(FILE_RE.findall(t))
        starts = {}
        for m in SUB_RE.finditer(t):
            starts[m.group(2)] = (m.group(1), fmap.get(m.group(3), '?'), int(m.group(4)))
        per_file = {}
        for _n, _f, _l in starts.values():
            per_file.setdefault(_f, set()).add(_l)
        per_file = {k: sorted(v) for k, v in per_file.items()}
        for m in DEF_RE.finditer(t):
            sym = m.group(1)
            j = t.find('\n}\n', m.start())
            body = t[m.start():j if j > 0 else m.start() + 40000]
            sym_off[sym] = {int(x) for x in GEP_RE.findall(body) if MIN_OFF <= int(x) < MAX_OFF}
            sym_calls[sym] = [c for c in set(CALL_RE.findall(body)) if not c.startswith('llvm.')]
            if sym in starts:
                nm, src, ln = starts[sym]
                nxt = [v for v in per_file.get(src, []) if v > ln]
                top[sym] = (nm, src, ln, (nxt[0] - 1) if nxt else ln + 400)
        del t
    return sym_off, sym_calls, top


def gather(sym, sym_off, sym_calls, depth=3):
    """인라인되는 헬퍼의 오프셋까지 합친다(모듈 경계를 넘는다)."""
    out, seen, q = set(sym_off.get(sym, ())), {sym}, [sym]
    for _ in range(depth):
        nxt = []
        for s in q:
            for c in sym_calls.get(s, ()):
                if c in seen or c not in sym_off:
                    continue
                seen.add(c)
                out |= sym_off[c]
                nxt.append(c)
        q = nxt
        if not q:
            break
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--selftest', action='store_true', help='확인된 사례로 도구를 검증')
    ap.add_argument('--name', default='')
    ap.add_argument('--thr', type=float, default=0.35, help='겹침 비율 임계(기본 0.35)')
    a = ap.parse_args()

    d = open(EXE, 'rb').read()
    secs = sections(d)

    def off(r):
        for vsz, va, rsz, ra in secs:
            if va <= r < va + max(vsz, rsz):
                return ra + r - va
        return None

    cs = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    cs.detail = True

    info = json.load(open(AIMAP)).get('info', {})
    by_mod = {}
    for rva, e in info.items():
        if not isinstance(e, dict):
            continue
        mod = str(e.get('mod', ''))
        ls = [l for l in e.get('lines', []) if isinstance(l, int)]
        if not mod or not ls:
            continue
        by_mod.setdefault(mod.replace(chr(92), '/').split('/')[-1], []).append(
            (min(ls), max(ls), int(rva, 16), int(e.get('bytes', 0))))
    for k in by_mod:
        by_mod[k].sort()

    sym_off, sym_calls, top = build()

    def exe_offsets(rva, sz):
        o = off(rva)
        if o is None or sz <= 0:
            return set()
        out = set()
        for ins in cs.disasm(d[o:o + sz], rva):
            for op in ins.operands:
                if op.type == capstone.x86.X86_OP_MEM:
                    v = op.mem.disp
                    if MIN_OFF <= v < MAX_OFF:
                        out.add(int(v))
        return out

    rows = []
    for sym, (nm, src, ln, end) in top.items():
        if a.name and a.name not in nm:
            continue
        base = src.replace(chr(92), '/').split('/')[-1].replace('.rs', '')
        hit = [c for c in by_mod.get(base, []) if ln <= c[0] <= end]
        if len(hit) != 1:
            continue
        _lo, _hi, rva, sz = hit[0]
        ir = gather(sym, sym_off, sym_calls)
        ex = exe_offsets(rva, sz)
        if len(ir) < MIN_N or len(ex) < MIN_N:
            continue
        inter = ir & ex
        # exe 쪽 기준 겹침: exe 가 읽는 필드 중 IR 에도 있는 비율
        cov = len(inter) / len(ex)
        rows.append((cov, nm, base, ln, rva, len(ir), len(ex), len(inter), sorted(ex - ir)[:6]))

    rows.sort()
    if a.selftest:
        print('=== 자체 검증 ===')
        want_flag = 'nexus_final_stand_uncached'
        want_ok = ['nexus_last_stand_uncached', 'base_attacking_minion_uncached']
        got = {r[1]: r for r in rows}
        ok = True
        r = got.get(want_flag)
        if r is None:
            print('  FAIL  %s 가 대조 목록에 없다(짝짓기 실패)' % want_flag)
            ok = False
        else:
            print('  %s  %s 겹침 %.2f (IR %d / exe %d / 교집합 %d)'
                  % ('PASS' if r[0] < a.thr else 'FAIL', want_flag, r[0], r[5], r[6], r[7]))
            ok &= r[0] < a.thr
        for w in want_ok:
            r = got.get(w)
            if r is None:
                print('  SKIP  %s 대조 안 됨' % w)
                continue
            print('  %s  %s 겹침 %.2f (무죄여야 함)' % ('PASS' if r[0] >= a.thr else 'FAIL', w, r[0]))
            ok &= r[0] >= a.thr
        print('  => %s' % ('검증 통과 - 출력을 신뢰해도 된다' if ok else '검증 실패 - 출력을 쓰지 말 것'))
        return

    print('대조된 함수 %d개 · 임계 겹침 %.2f' % (len(rows), a.thr))
    susp = [r for r in rows if r[0] < a.thr]
    print('  skew 의심 %d개' % len(susp))
    print()
    print('%-6s %-38s %-22s %-10s %s' % ('겹침', '함수', '소스', 'exe RVA', 'exe 에만 있는 오프셋'))
    for cov, nm, base, ln, rva, ni, ne, nx, only in susp[:40]:
        print('%.3f  %-38s %-16s:%-5d 0x%-8x %s' % (cov, nm[:38], base[:16], ln, rva,
                                                    [hex(x) for x in only]))


main()
