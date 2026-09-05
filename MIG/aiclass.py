# -*- coding: utf-8 -*-
r"""aiclass — AI 계층 640함수를 **역할별로 분류**해 재현 우선순위를 만든다

    python MIG\aiclass.py <exe> [-o class.md]

"선수 행동판단을 전면 대체한다"는 목표에서 640개를 무작정 재현할 필요는 없다.
역할이 다르고, 재현 난이도와 **판단에 미치는 영향**도 다르다.

분류 기준(전부 기계적으로 판정 가능한 신호):
  빌더   후보 SmallAction 을 만든다 — `0xccc7a0`(Vec grow) 또는 `0xc8c8f0`(push) 호출
  스코어러 점수를 낸다 — `0xd57540`(기본점수) 또는 `0xd59940`/`0xd5ba80` 호출, 혹은 -99999 상수
  리졸버  SmallAction → 실제 명령 — `small_action`/`move_actions`/`around`/`cast`/`trace` 모듈
  경로    경로탐색 — `path_finder`/`path_field`/`free_dist` 모듈
  진입점  매 틱/재계획 — `lib.rs`·`handler` 모듈의 큰 함수
  헬퍼    나머지(술어·수치 산출)

출력에 **명령 수 누적 비율**을 넣어, "상위 N개만 해도 몇 %를 덮는가"를 볼 수 있게 한다.
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

SCORE_FNS = {0xd57540, 0xd59940, 0xd5ba80, 0xe388c0, 0xe28410}
BUILD_FNS = {0xccc7a0, 0xc8c8f0, 0xcada80}
SOFT_REJECT = (-99999, -9999, -8999999)


def classify(mod, v):
    m = mod.replace('/', '\\')
    calls = set(v['callees'])
    has_build = bool(calls & BUILD_FNS)
    has_score = bool(calls & SCORE_FNS) or any(k in v['imms'] for k in SOFT_REJECT)

    if 'path_finder' in m or 'path_field' in m or 'free_dist' in m:
        return '경로'
    if has_build:
        return '빌더'
    if has_score:
        return '스코어러'
    if ('small_action' in m or 'move_actions' in m or '\\around' in m
            or '\\cast' in m or '\\trace' in m or 'abstract_input' in m):
        return '리졸버'
    if m.endswith('lib.rs') or 'handler' in m:
        return '진입점'
    return '헬퍼'


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('exe')
    ap.add_argument('-o', '--out', default='')
    a = ap.parse_args()

    import aidiff
    img, fn = aidiff.load(a.exe, 'game-ai', False)

    rows = []
    for (b, e), v in fn.items():
        rows.append((classify(v['top'], v), b, e, v['n'], v['top']))

    agg = collections.defaultdict(lambda: [0, 0])
    for c, b, e, n, m in rows:
        agg[c][0] += 1
        agg[c][1] += n
    tot_n = sum(x[1] for x in agg.values())

    L = []
    w = L.append
    w('# AI 계층 640함수 — 역할별 분류 (재현 우선순위용)')
    w('')
    w('| 역할 | 함수 | 명령 | 비중 | 대체하려면 |')
    w('|---|---|---|---|---|')
    ORDER = ['빌더', '스코어러', '진입점', '리졸버', '경로', '헬퍼']
    NOTE = {
        '빌더': '★후보를 만드는 곳 = 무엇을 할 수 있는지를 결정',
        '스코어러': '★★점수 = 무엇을 고를지를 결정. 조정의 핵심',
        '진입점': '매 틱 흐름·재계획·경매. 구조가 커서 마지막에',
        '리졸버': '선택된 액션 → 실제 명령. 게임 상태 조회가 많다',
        '경로': '경로탐색. 판단이라기보다 계산 — 나중에',
        '헬퍼': '술어·수치 산출. 위 것들을 재현하며 딸려 온다',
    }
    for c in ORDER:
        if c not in agg:
            continue
        f, n = agg[c]
        w('| %s | %d | %d | %.1f%% | %s |' % (c, f, n, 100.0 * n / tot_n, NOTE[c]))
    w('')
    w('**합계 %d함수 / %d명령**' % (len(rows), tot_n))
    w('')

    # 스코어러·빌더는 개별 목록까지 (조정의 대상이라 실무에서 바로 쓴다)
    for c in ('스코어러', '빌더'):
        sub = sorted([r for r in rows if r[0] == c], key=lambda r: -r[3])
        w('## %s %d개 (명령 많은 순)' % (c, len(sub)))
        w('')
        w('| RVA | 명령 | 모듈 |')
        w('|---|---|---|')
        for _, b, e, n, m in sub[:40]:
            w('| `0x%x` | %d | `%s` |' % (b, n, m.split('src\\')[-1] if 'src\\' in m else m))
        if len(sub) > 40:
            w('| … | | %d개 더 |' % (len(sub) - 40))
        w('')

    txt = '\n'.join(L)
    if a.out:
        io.open(a.out, 'w', encoding='utf-8', newline='\n').write(txt)
        print('→ %s' % a.out)
    print('\n'.join(L[:22]))


if __name__ == '__main__':
    main()
