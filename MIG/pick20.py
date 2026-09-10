#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""pick20.py — 명세 작성 시범 배치용 **대표 표본**을 뽑는다.

왜 표본 설계가 중요한가:
  쉬운 것만 고르면 "753개에 며칠"이 과소추정된다. 이 배치의 목적은 결과물이 아니라
  **속도와 QC 통과율 실측**이므로 대표성이 전부다.

표본 규칙:
  - 100~500 IR 줄 구간(함수 개수 36.7% · 가장 대표적)
  - `game-ai\src\**` 소스만(core/std 제네릭 인스턴스 제외)
  - 클로저 제외(부모의 일부라 단독 명세 단위가 아님)
  - **한 소스 파일당 최대 2개** — 한 파일에 몰리면 문맥이 공유돼 속도가 부풀려진다
  - 파일은 AI 판단 계층 위주로 정렬(plan_legacy / *_score / *_eval / fight_* / 등)

출력 = 명세 작성자가 바로 열 수 있게 **IR 파일 + 본체 줄범위**까지.
"""
import io
import json
import os
import re
import sys
from collections import defaultdict

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

HERE = os.path.dirname(os.path.abspath(__file__))
IRDIR = r'C:\tfm2mods\_gaibc'
OUT = os.path.join(HERE, 'spec_batch1.json')

RE_DEF = re.compile(r'^define\b[^\n]*?@("[^"]+"|[\w.$]+)\([^\n]*?!dbg (![0-9]+)', re.M)
RE_SUB = re.compile(r'^(![0-9]+) = (?:distinct )?!DISubprogram\(name: "([^"]*)"[^\n]*?file: (![0-9]+), line: ([0-9]+)', re.M)
RE_FILE = re.compile(r'^(![0-9]+) = !DIFile\(filename: "([^"]*)"', re.M)

LO, HI = 100, 500


def collect():
    rows = []
    for fn in sorted(os.listdir(IRDIR)):
        if not fn.endswith('.ll'):
            continue
        text = io.open(os.path.join(IRDIR, fn), encoding='utf-8', errors='replace').read()
        files = dict(RE_FILE.findall(text))
        subs = {}
        for mid, name, fid, line in RE_SUB.findall(text):
            subs[mid] = (name, files.get(fid, '?'), int(line))
        lines = text.split('\n')
        cur, start = None, 0
        for i, ln in enumerate(lines):
            if ln.startswith('define'):
                m = RE_DEF.match(ln)
                cur, start = (m.group(2) if m else None), i
            elif ln == '}' and cur is not None:
                info = subs.get(cur)
                n = i - start
                if info and LO <= n < HI:
                    name, sf, sl = info
                    if sf.startswith('game-ai') and 'closure' not in name:
                        rows.append(dict(name=name, sym=cur.strip('"'), src=sf, src_line=sl,
                                         ir_file=fn, ir_from=start + 1, ir_to=i + 1, ir_lines=n))
                cur = None
    return rows


# 판단 계층으로 보이는 파일에 가중치. (이름만으로 거르지 않는다 — 오늘 그렇게 틀렸다.
#  정렬 우선순위일 뿐이고, 표본에는 다른 파일도 섞는다.)
JUDGE_HINT = ('plan_legacy', 'action_score', 'position_eval', 'fight_', 'buff_value',
              'objective', 'tower_', 'battle', 'score_parameter', 'abstract_input')


def main():
    rows = collect()
    # 같은 소스 함수(파일:줄)의 인스턴스 중복 제거 — 가장 큰 것만
    best = {}
    for r in rows:
        k = (r['src'], r['src_line'])
        if k not in best or r['ir_lines'] > best[k]['ir_lines']:
            best[k] = r
    rows = list(best.values())

    byfile = defaultdict(list)
    for r in rows:
        byfile[r['src']].append(r)

    def prio(f):
        return (0 if any(h in f for h in JUDGE_HINT) else 1, f)

    picked, seen = [], defaultdict(int)
    # 라운드로빈: 파일을 돌며 한 개씩 — 한 파일 몰림 방지
    for rnd in range(2):
        for f in sorted(byfile, key=prio):
            if len(picked) >= 20:
                break
            cands = sorted(byfile[f], key=lambda r: -r['ir_lines'])
            if seen[f] < len(cands) and seen[f] < 2:
                picked.append(cands[seen[f]])
                seen[f] += 1
        if len(picked) >= 20:
            break

    picked = picked[:20]
    io.open(OUT, 'w', encoding='utf-8', newline='').write(
        json.dumps(picked, ensure_ascii=False, indent=1))

    print('후보 풀 %d개(소스 함수 · 100~500 IR줄) → 표본 %d개 · %s'
          % (len(rows), len(picked), OUT))
    print()
    print('%-3s %-44s %-40s %6s %-9s %s'
          % ('#', '함수', '소스', 'IR줄', 'IR파일', '줄범위'))
    for i, r in enumerate(picked, 1):
        print('%-3d %-44s %-40s %6d %-9s %d~%d'
              % (i, r['name'][:44], (r['src'].replace('game-ai\\\\src\\\\', '') + ':' + str(r['src_line']))[:40],
                 r['ir_lines'], r['ir_file'], r['ir_from'], r['ir_to']))
    print()
    print('IR 줄 합계 %s · 중앙값 %d'
          % (f"{sum(r['ir_lines'] for r in picked):,}",
             sorted(r['ir_lines'] for r in picked)[len(picked) // 2]))


if __name__ == '__main__':
    main()
