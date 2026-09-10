#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""speccmp.py — **같은 함수를 독립적으로 두 번 명세한 결과**를 비교해 신뢰도를 잰다.

왜 필요한가:
  `qcspec.py` 는 **지어낸 것**만 잡는다(본문에 없는 상수·호출). 잡지 못하는 게 둘 있다:
    ① **빠뜨린 것** — 진짜 상수를 안 적어도 QC 는 통과한다
    ② **얕은 이해** — logic 이 틀려도 상수만 맞으면 통과한다
  독립 재현 2회를 대조하면 ①이 드러난다. 한쪽에만 있는 항목 = 둘 중 하나가 틀렸거나
  불완전하다는 **신호**이고, 사람이 볼 곳을 정확히 찍어준다.

  ⚠일치가 곧 정답은 아니다(둘 다 같은 걸 놓쳤을 수 있다). 이건 **하한** 측정이다.

사용:
  python speccmp.py _spec/r1 _spec/r2
  python speccmp.py _spec/r1 _spec/r2 --detail <명세이름>
"""
import io
import json
import os
import re
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')


def leaf(sym):
    s = str(sym).strip('"@')
    for _ in range(8):
        s2 = re.sub(r'<[^<>]*>', '', s)
        if s2 == s:
            break
        s = s2
    s = s.replace('<', '').replace('>', '')
    if '::' in s:
        parts = [p for p in s.split('::') if p.strip()]
        s = parts[-1] if parts else ''
    return re.sub(r'[^A-Za-z0-9_]', '', s)


def norm_off(o):
    try:
        return int(str(o), 16) if str(o).lower().startswith('0x') else int(o)
    except (ValueError, TypeError):
        return None


def sets_of(spec):
    consts = {c.get('value') for c in spec.get('constants', []) if c.get('value') is not None}
    calls = {leaf(c if isinstance(c, str) else c.get('name', '')) for c in spec.get('calls', [])}
    calls.discard('')
    reads = {norm_off(r.get('offset')) for r in spec.get('reads', [])}
    reads.discard(None)
    return consts, calls, reads


def jac(a, b):
    if not a and not b:
        return 1.0
    return len(a & b) / float(len(a | b))


def main():
    if len(sys.argv) < 3:
        print(__doc__)
        return 1
    d1, d2 = sys.argv[1], sys.argv[2]
    detail = None
    if '--detail' in sys.argv:
        i = sys.argv.index('--detail')
        detail = sys.argv[i + 1] if i + 1 < len(sys.argv) else ''

    names = sorted(set(os.listdir(d1)) & set(os.listdir(d2)))
    names = [n for n in names if n.endswith('.json')]
    only1 = sorted(set(os.listdir(d1)) - set(os.listdir(d2)))
    only2 = sorted(set(os.listdir(d2)) - set(os.listdir(d1)))

    print('공통 %d개' % len(names) + (' · %s 에만 %d' % (d1, len(only1)) if only1 else '')
          + (' · %s 에만 %d' % (d2, len(only2)) if only2 else ''))
    print()
    print('%-46s %6s %6s %6s   %s' % ('명세', '상수', '호출', '필드', '불일치(한쪽에만)'))
    tot = [0, 0, 0]
    diffs = []
    for n in names:
        s1 = json.load(io.open(os.path.join(d1, n), encoding='utf-8'))
        s2 = json.load(io.open(os.path.join(d2, n), encoding='utf-8'))
        c1, k1, r1 = sets_of(s1)
        c2, k2, r2 = sets_of(s2)
        jc, jk, jr = jac(c1, c2), jac(k1, k2), jac(r1, r2)
        tot[0] += jc
        tot[1] += jk
        tot[2] += jr
        miss = len(c1 ^ c2) + len(k1 ^ k2) + len(r1 ^ r2)
        print('%-46s %5.0f%% %5.0f%% %5.0f%%   %d' % (n[:-5][:46], jc * 100, jk * 100, jr * 100, miss))
        diffs.append((n, c1 ^ c2, k1 ^ k2, r1 ^ r2, s1, s2))

    m = len(names) or 1
    print()
    print('평균 일치도 —  상수 %.1f%% · 호출 %.1f%% · 구조체필드 %.1f%%'
          % (tot[0] / m * 100, tot[1] / m * 100, tot[2] / m * 100))
    print('⚠일치는 **하한**이다 — 둘 다 같은 걸 놓쳤으면 일치로 나온다.')

    if detail is not None:
        print()
        for n, dc, dk, dr, s1, s2 in diffs:
            if detail and detail not in n:
                continue
            if not (dc or dk or dr):
                continue
            print('=== %s ===' % n)
            c1, k1, r1 = sets_of(s1)
            for v in sorted(dc, key=str):
                print('  상수 %-16s : %s 에만' % (v, d1 if v in c1 else d2))
            for v in sorted(dk):
                print('  호출 %-16s : %s 에만' % (v, d1 if v in k1 else d2))
            for v in sorted(dr):
                print('  필드 %-16s : %s 에만' % (hex(v), d1 if v in r1 else d2))
            print('  한줄요약 A: %s' % str(s1.get('one_line', ''))[:90])
            print('  한줄요약 B: %s' % str(s2.get('one_line', ''))[:90])
            print()
    return 0


if __name__ == '__main__':
    sys.exit(main())
