# -*- coding: utf-8 -*-
r"""aiscope — exe 안의 **Rust 모듈 트리 전체**를 뽑아 재현 범위를 실측한다

    python MIG\aiscope.py <exe> [-o scope.md] [--depth 2]

`game-ai` 만 보던 `aidiff`/`aidump` 와 달리, **패닉 Location 이 가리키는 모든 `.rs` 경로**를
모아 크레이트/모듈 단위로 집계한다. "게임 중 돌아가는 시뮬레이션을 전부 대체한다"는
목표를 세우려면 **무엇을 얼마나 대체해야 하는지**부터 숫자로 알아야 한다.

출력: 크레이트별 함수 수·명령 수·바이트, 그리고 **AI 계층이 실제로 의존하는지** 표시.
"""
import argparse
import collections
import io
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')


def top_of(mod, depth):
    """`game-ai\\src\\plan_legacy\\sub_plan\\battle.rs` → depth 만큼의 접두."""
    p = mod.replace('/', '\\').split('\\')
    # `<크레이트>\src\...` 형태면 src 를 건너뛴다
    if len(p) >= 2 and p[1] == 'src':
        head = [p[0]] + p[2:]
    else:
        head = p
    return '\\'.join(head[:depth]) if len(head) > depth else '\\'.join(head)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('exe')
    ap.add_argument('-o', '--out', default='')
    ap.add_argument('--depth', type=int, default=1, help='집계 깊이(1=크레이트)')
    ap.add_argument('--nocache', action='store_true')
    a = ap.parse_args()

    import aidiff
    img, fn = aidiff.load(a.exe, '', a.nocache)      # ★prefix 없이 = 전량
    print('Rust Location 을 가진 함수 %d개' % len(fn))

    ai = set(k for k, v in fn.items() if 'game-ai' in v['top'])
    print('그중 game-ai = %d개' % len(ai))
    print()

    # AI 가 직접 call 하는 대상의 소유 모듈 → "AI 가 의존하는 크레이트"
    dep = collections.Counter()
    for k in ai:
        for c in fn[k]['callees']:
            o = img.owner(c)
            if o and o in fn and o not in ai:
                dep[top_of(fn[o]['top'], a.depth)] += fn[k]['callees'][c]

    agg = collections.defaultdict(lambda: [0, 0, 0])   # 함수, 명령, 바이트
    for (b, e), v in fn.items():
        t = top_of(v['top'], a.depth)
        agg[t][0] += 1
        agg[t][1] += v['n']
        agg[t][2] += e - b

    rows = sorted(agg.items(), key=lambda x: -x[1][1])
    L = []
    w = L.append
    w('# Rust 모듈 트리 — 재현 범위 실측')
    w('')
    w('> 패닉 `Location` 실측. exe = `%s`' % os.path.basename(a.exe))
    w('> 총 %d함수 (game-ai %d)' % (len(fn), len(ai)))
    w('')
    w('| 모듈 | 함수 | 명령 | 바이트 | AI가 직접 호출 |')
    w('|---|---|---|---|---|')
    tf = ti = tb = 0
    for t, (f, i, b) in rows:
        tf += f
        ti += i
        tb += b
        w('| `%s` | %d | %d | %d | %s |'
          % (t, f, i, b, ('%d회' % dep[t]) if dep.get(t) else '—'))
    w('')
    w('**합계 — %d함수 / %d명령 / %d바이트**' % (tf, ti, tb))

    txt = '\n'.join(L)
    if a.out:
        io.open(a.out, 'w', encoding='utf-8', newline='\n').write(txt)
        print('→ %s' % a.out)
    print(txt if not a.out else '\n'.join(L[:40]))


if __name__ == '__main__':
    main()
