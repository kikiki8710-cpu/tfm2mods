# -*- coding: utf-8 -*-
"""노브 재배치 준비 — 0.5.3 패치 사이트를 **소속 함수 + 소스 파일**로 되짚는다.

⚠**이 스크립트는 0.5.4 위치를 찾지 않는다.** 그건 다음 단계다.
   여기서 하는 일은 "우리 노브가 0.5.3에서 **어느 소스 파일의 어느 함수**에 있었나"를
   전부 적어 두는 것이다. 그래야 0.5.4에서 **같은 소스 파일**을 먼저 보고,
   그 안에서 문맥으로 찾을 수 있다.

   지금까지 오판이 난 방식 = 0.5.4 전역에서 상수값만 스캔 → 다른 자리를 잡음.
   올바른 방식 = 소스 파일 → 함수 → 명령 문맥 → 그때서야 상수 확인.

산출: knob_sites_053.tsv  (노브 / RVA / 함수시작 / 함수끝 / 소스파일 / 명령)
"""
import io, os, re, sys

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE

D = r'C:\tfm2mods\v54'
SRC = r'C:\tfm2mods\tfm2_ai_adjust\src'

# detour.rs 등에서 `base + 0xXXXXXX` 형태의 패치 주소를 긁는다.
ADDR = re.compile(r'base\s*\+\s*0x([0-9a-fA-F]{5,7})')
# 바로 앞줄들에서 노브 이름을 추정하기 위한 tune 변수 맵
TUNE = re.compile(r'let\s+(\w+)\s*=\s*tune\("([a-z][a-z0-9_]*)"')


def build():
    e = load('053')
    # 함수시작 → 소스
    src_of = {}
    for ln in io.open(os.path.join(D, '053_srcmap.tsv'), encoding='utf-8'):
        s, en, src, lines = ln.rstrip('\n').split('\t')
        src_of[int(s, 16)] = src

    rows = []
    for fn in os.listdir(SRC):
        if not fn.endswith('.rs'):
            continue
        p = os.path.join(SRC, fn)
        text = io.open(p, encoding='utf-8', errors='replace').read()
        var2knob = {m.group(1): m.group(2) for m in TUNE.finditer(text)}
        for i, line in enumerate(text.split('\n'), 1):
            if line.lstrip().startswith('//'):
                continue          # 주석 안의 주소는 오탐(0.5.3에서 실제로 겪음)
            for m in ADDR.finditer(line):
                rva = int(m.group(1), 16)
                # 같은 줄에서 노브 변수 찾기
                knob = ''
                for v, k in var2knob.items():
                    if re.search(r'\b%s\b' % re.escape(v), line):
                        knob = k
                        break
                f = e.func_of(rva)
                ins = e.dis(rva, 16)
                asm = ('%s %s' % (ins[0].mnemonic, ins[0].op_str)) if ins else ''
                by = ins[0].bytes.hex() if ins else ''
                rows.append((knob or '?', rva, f[0] if f else 0, f[1] if f else 0,
                             src_of.get(f[0] if f else 0, '(소스미상)'), by, asm, '%s:%d' % (fn, i)))

    out = os.path.join(D, 'knob_sites_053.tsv')
    with io.open(out, 'w', encoding='utf-8', newline='') as fo:
        fo.write('knob\trva\tfn_start\tfn_end\tsrc\tbytes\tasm\twhere\n')
        for r in sorted(set(rows)):
            fo.write('%s\t%06x\t%06x\t%06x\t%s\t%s\t%s\t%s\n' % r)

    # 요약
    import collections
    bysrc = collections.Counter(r[4] for r in set(rows))
    print('패치 사이트 %d개 / 노브 이름이 붙은 것 %d개' %
          (len(set(rows)), len([r for r in set(rows) if r[0] != '?'])))
    print('\n소스 파일별 사이트 수 (상위 20):')
    for s, c in bysrc.most_common(20):
        print('  %4d  %s' % (c, s[:78]))
    print('\n→ %s' % out)


if __name__ == '__main__':
    build()
