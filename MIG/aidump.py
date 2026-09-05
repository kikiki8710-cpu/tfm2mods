# -*- coding: utf-8 -*-
r"""aidump — AI 판단계층 **원본 소스 트리 복원** 스캐폴딩

    python MIG\aidump.py <exe> -o <출력디렉터리> [--ver 0.5.8]

`aidiff.py` 가 만든 census 캐시를 재사용해, AI 계층 함수 전량을
**원본 `game-ai\src\**.rs` 트리 모양 그대로** 재구성한다.

각 모듈 파일에는 그 파일에 속한 함수들이 **원본 행 번호 순서로** 배치되고,
함수마다 복호에 필요한 사실이 헤더로 붙는다:
  · RVA / 크기 / 명령 수
  · **원본 행 범위**(패닉 Location 실측) — 이게 "몇 번째 줄 함수인가"를 준다
  · 콜리(같은 모듈이면 함수명 대신 행번호로 연결)
  · 상수 전수 · 필드 오프셋 · vtable 슬롯

디컴 본문은 이 스캐폴딩의 `<본문>` 자리에 채워 넣는다.
⟹ 채워 넣고 나면 결과물이 **원본 소스 트리와 같은 배치**가 되어, 파일 하나를 열면
   그 `.rs` 의 함수들이 원본 순서대로 보인다.
"""
import argparse
import collections
import hashlib
import io
import os
import pickle
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import aidiff                                                    # noqa: E402  (stdout 재바인딩은 aidiff 가 한다)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('exe')
    ap.add_argument('-o', '--out', required=True)
    ap.add_argument('--ver', default='')
    ap.add_argument('-p', '--prefix', default='game-ai')
    ap.add_argument('-b', '--batches', type=int, default=4,
                    help='디컴 배치 수(명령 수 균등 분할표를 INDEX.md 에 넣는다). 0=끔')
    a = ap.parse_args()

    img, fn = aidiff.load(a.exe, a.prefix, False)
    print("AI 계층 함수 %d개" % len(fn))

    # 모듈별로 묶고 원본 행 순으로 정렬
    mods = collections.defaultdict(list)
    for (b, e), v in fn.items():
        lines = sorted(l for l, _ in v['lmods'].get(v['top'], set())) if v.get('lmods') else \
                sorted(l for l, _ in v['lines'])
        first = lines[0] if lines else 10 ** 9
        mods[v['top']].append((first, b, e, v, lines))
    for m in mods:
        mods[m].sort()

    os.makedirs(a.out, exist_ok=True)
    idx = ['# AI 판단계층 원본 소스 트리 복원 — %s' % (a.ver or os.path.basename(a.exe)),
           '',
           '> 모듈·행 번호는 Rust 패닉 `Location` **실측**이다(추정 아님).',
           '> 각 파일은 그 `.rs` 에 속한 함수를 **원본 행 순서**로 담는다.',
           '',
           '| 원본 모듈 | 함수 | 총 명령 | 파일 |',
           '|---|---|---|---|']

    total_fn = total_ins = 0
    for m in sorted(mods):
        items = mods[m]
        rel = m.replace('\\', '/')
        if rel.startswith('game-ai/src/'):
            rel = rel[len('game-ai/src/'):]
        dst = os.path.join(a.out, rel[:-3] + '.md') if rel.endswith('.rs') else \
              os.path.join(a.out, rel + '.md')
        os.makedirs(os.path.dirname(dst), exist_ok=True)

        L = ['# `%s`' % m, '']
        L.append('원본 함수 %d개 · 총 %d 명령. 행 번호는 패닉 Location 실측.'
                 % (len(items), sum(v['n'] for _, _, _, v, _ in items)))
        L.append('')
        for first, b, e, v, lines in items:
            span = '%d~%d' % (lines[0], lines[-1]) if lines else '(행 미상)'
            L.append('---')
            L.append('')
            L.append('## `0x%x`  —  원본 행 %s' % (b, span))
            L.append('')
            L.append('| | |')
            L.append('|---|---|')
            L.append('| RVA | `0x%x` ~ `0x%x` (%d B) |' % (b, e, e - b))
            L.append('| 명령 수 | %d |' % v['n'])
            L.append('| 원본 행 | %s |' % (', '.join(str(x) for x in lines) or '—'))
            others = [(k, c) for k, c in v['mods'].items() if k != v['top']]
            if others:
                L.append('| 참조 타 모듈 | %s |' %
                         ', '.join('`%s`×%d' % (k.split('\\')[-1], c) for k, c in others))
            L.append('')
            # 콜리 — 같은 exe 안 AI 함수면 모듈·행으로 연결
            cal = []
            for c, cnt in sorted(v['callees'].items(), key=lambda x: -x[1])[:24]:
                own = fn.get(next((k for k in fn if k[0] == c), None))
                if own:
                    ol = sorted(l for l, _ in own['lines'])
                    cal.append('`0x%x`(%s:%s)×%d' % (c, own['top'].split('\\')[-1],
                                                     ol[0] if ol else '?', cnt))
                else:
                    cal.append('`0x%x`×%d' % (c, cnt))
            if cal:
                L.append('**콜리**: ' + ' · '.join(cal))
                L.append('')
            big = [(k, c) for k, c in v['imms'].items() if abs(k) >= 16]
            if big:
                L.append('**상수**(|v|≥16): ' + ' · '.join(
                    '`0x%x`(%d)×%d' % (k & 0xffffffffffffffff, k, c)
                    for k, c in sorted(big, key=lambda x: -x[1])[:40]))
                L.append('')
            if v['offs']:
                L.append('**필드 오프셋**: ' + ' · '.join(
                    '`+0x%x`×%d' % (k, c) for k, c in sorted(v['offs'].items())[:60]))
                L.append('')
            if v['vslots']:
                L.append('**vtable 간접호출**: ' + ' · '.join(
                    '`+0x%x`×%d' % (k, c) for k, c in sorted(v['vslots'].items())))
                L.append('')
            L.append('```c')
            L.append('// <본문> — 디컴 결과를 여기에 채운다')
            L.append('```')
            L.append('')
        io.open(dst, 'w', encoding='utf-8', newline='\n').write('\n'.join(L))
        n = len(items)
        ins = sum(v['n'] for _, _, _, v, _ in items)
        total_fn += n
        total_ins += ins
        idx.append('| `%s` | %d | %d | [%s](%s) |' % (m, n, ins, rel, rel[:-3] + '.md'))

    idx += ['', '**합계 — 모듈 %d개 / 함수 %d개 / %d 명령**' % (len(mods), total_fn, total_ins)]

    # ── 배치 분할 ── 다음 버전에도 그대로 쓰도록 **명령 수 균등**으로 자동 분배한다.
    #   디컴은 ghidra-re 에이전트 N개에 나눠 맡기는데, 매번 손으로 쪼개면 그것도 반복노동이다.
    if a.batches > 0:
        want = [(m, len(v), sum(x[3]['n'] for x in v)) for m, v in mods.items()]
        want.sort(key=lambda x: -x[2])
        bins = [[0, []] for _ in range(a.batches)]     # [명령합, [(모듈,함수수,명령)]]
        for m, nf, ni in want:                          # LPT: 큰 것부터 가장 적은 통에
            b = min(bins, key=lambda x: x[0])
            b[0] += ni
            b[1].append((m, nf, ni))
        idx += ['', '## 디컴 배치 분할 (명령 수 균등 · LPT)', '']
        for i, (tot, items) in enumerate(bins, 1):
            idx.append('### 배치 %d — %d 함수 / %d 명령' %
                       (i, sum(x[1] for x in items), tot))
            idx.append('')
            idx.append('| 파일 | 함수 | 명령 |')
            idx.append('|---|---|---|')
            for m, nf, ni in sorted(items, key=lambda x: -x[2]):
                rel = m.replace('\\', '/')
                if rel.startswith('game-ai/src/'):
                    rel = rel[len('game-ai/src/'):]
                idx.append('| `%s` | %d | %d |' % (rel[:-3] + '.md', nf, ni))
            idx.append('')

    io.open(os.path.join(a.out, 'INDEX.md'), 'w', encoding='utf-8', newline='\n').write('\n'.join(idx))
    print("모듈 %d개 / 함수 %d개 / %d 명령  →  %s" % (len(mods), total_fn, total_ins, a.out))
    if a.batches:
        print("배치 %d개 분할표를 INDEX.md 에 넣었다 — 그대로 ghidra-re 에 나눠 주면 된다." % a.batches)


if __name__ == '__main__':
    main()
