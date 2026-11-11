# -*- coding: utf-8 -*-
r"""묶음 원문을 mods_report\_0.7.2_RE 로 복사한다(CLAUDE.md §11 — 첫 줄 지시: · 요약: · 원문 그대로).

    python -X utf8 save_re.py <W1..W9> "<요약>"
"""
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RE = os.path.join(r'F:\claude\tfm2\.claude\worktrees\salary-transfer-fee-analysis-1bf168', 'mods_report', '_0.7.2_RE')
os.makedirs(RE, exist_ok=True)
TOPIC = {'W1': '플랜처리기', 'W2': '빅플랜전투', 'W3': '빅플랜운영', 'W4': '서브플랜', 'W5': '팀플랜', 'W6': '공통점수',
         'W7': '경로RL', 'W8': '보조함수', 'W9': '게임코어',
         'W7a': '경로필드', 'W7b': '경로탐색RL', 'W8a': '보조함수짝', 'W8b': '짝없는피호출',
         'W9a': '게임코어_시뮬엔티티', 'W9b': '게임코어_게임이적유틸', '공통원인': '준비단계'}
g, summary = sys.argv[1], sys.argv[2]
body = open(os.path.join(HERE, g + '.md'), encoding='utf-8').read()
out = os.path.join(RE, '2026-11-11_AI변경_%s_%s.md' % (g, TOPIC.get(g, g)))
head = ('지시: 0.7.1→0.7.2 AI 판단함수 변경점 전수 조사 %s(공통 지침 = MODS\\MIG\\_patchday\\0.7.2\\aiwide\\BRIEF_AI.md)\n'
        '요약: %s\n\n---\n\n' % (g, summary))
open(out, 'w', encoding='utf-8', newline='\n').write(head + body)
print(out, os.path.getsize(out))
