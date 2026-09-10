#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""knobscan.py — game_ai IR 에서 **매직 상수 전수 목록**을 뽑는다(노브 후보 인벤토리).

왜 이게 가능한가:
  exe 에서는 상수가 접히고 인라인돼 "어느 소스 줄의 무엇인지"를 되찾을 수 없다
  (irskew.py 가 그래서 폐기됐다). 반면 rlib IR 은 **최적화 전 단계**라
  `!dbg !N -> !DILocation(line:, scope:) -> !DISubprogram(name:, file:)` 사슬이 살아 있다.
  ⟹ 상수 하나하나에 **함수명 + 파일:줄** 을 붙일 수 있다.

출력 = 함수 / 파일:줄 / 상수 / 어떤 연산에 쓰였나.
"""
import io
import os
import re
import sys
from collections import defaultdict

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

HERE = os.path.dirname(os.path.abspath(__file__))
IRDIR = r'C:\tfm2mods\_gaibc'

RE_LOC = re.compile(r'^(![0-9]+) = !DILocation\(line: ([0-9]+),(?: column: [0-9]+,)? scope: (![0-9]+)', re.M)
RE_SUB = re.compile(r'^(![0-9]+) = (?:distinct )?!DISubprogram\(name: "([^"]*)"[^\n]*?file: (![0-9]+)[^\n]*?line: ([0-9]+)', re.M)
RE_LEX = re.compile(r'^(![0-9]+) = !DILexicalBlock(?:File)?\([^\n]*?scope: (![0-9]+)', re.M)
RE_FILE = re.compile(r'^(![0-9]+) = !DIFile\(filename: "([^"]*)"', re.M)

# 관심 연산: 비교·가감산에 쓰인 즉치. 이게 임계값·상수 파라미터가 사는 자리다.
# ⚠피연산자는 `add nsw i64 %25, 100` 처럼 **뒤쪽**에 온다 — 타입 바로 뒤가 아니다.
RE_OP = re.compile(
    r'^\s*(?:%\S+ = )?(icmp|add|sub|mul|udiv|sdiv|shl|and|or)\b([^\n]*?), !dbg (![0-9]+)', re.M)
# 정수 리터럴만. 타입 토큰(i64 의 64)은 앞이 공백/콤마가 아니라 걸리지 않는다.
RE_INT = re.compile(r'(?:^|[\s,])(-?[0-9]+)(?=[\s,]|$)')

# 이 밑은 노이즈(인덱스·정렬·부호비트 등). 임계값은 대개 이 위다.
MIN_ABS = 10


def scan(path):
    src = io.open(path, encoding='utf-8', errors='replace').read()
    files = dict(RE_FILE.findall(src))
    subs = {}
    for mid, name, fid, line in RE_SUB.findall(src):
        subs[mid] = (name, files.get(fid, '?'), int(line))
    lex = dict(RE_LEX.findall(src))
    locs = {}
    for mid, line, scope in RE_LOC.findall(src):
        locs[mid] = (int(line), scope)

    def resolve(scope, depth=0):
        """DILexicalBlock 을 타고 올라가 소속 DISubprogram 을 찾는다."""
        while scope not in subs and scope in lex and depth < 32:
            scope = lex[scope]
            depth += 1
        return subs.get(scope)

    out = []
    for op, operands, dbg in RE_OP.findall(src):
        vals = [int(x) for x in RE_INT.findall(operands)]
        vals = [v for v in vals if abs(v) >= MIN_ABS]
        if not vals:
            continue
        loc = locs.get(dbg)
        if not loc:
            continue
        sub = resolve(loc[1])
        if not sub:
            continue
        for v in vals:
            out.append((sub[0], sub[1], loc[0], v, op))
    return out


if __name__ == '__main__':
    targets = sys.argv[1:] or ['m10.ll']
    rows = []
    for t in targets:
        rows.extend(scan(os.path.join(IRDIR, t)))

    # 같은 (함수, 줄, 값) 중복 제거 — 인라인으로 여러 번 나온다.
    uniq = sorted(set(rows), key=lambda r: (r[1], r[2], r[0]))
    print('상수 사이트 %d개 (중복제거 후) / 함수 %d개 / 파일 %d개'
          % (len(uniq), len({r[0] for r in uniq}), len({r[1] for r in uniq})))
    print()
    byfile = defaultdict(int)
    for r in uniq:
        byfile[r[1]] += 1
    print('파일별 상위:')
    for f, n in sorted(byfile.items(), key=lambda kv: -kv[1])[:12]:
        print('  %-42s %4d' % (f, n))
    print()
    print('샘플 (fight_model / action_score / path_finder):')
    shown = 0
    for name, f, line, v, op in uniq:
        if not any(k in f for k in ('fight_model', 'action_score', 'path_finder')):
            continue
        print('  %-46s %s:%-5d  %-5s %d' % (name[:46], os.path.basename(f), line, op, v))
        shown += 1
        if shown >= 30:
            break
