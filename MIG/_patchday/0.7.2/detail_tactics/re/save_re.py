# -*- coding: utf-8 -*-
r"""그룹 판정 원문을 REPORT RE 폴더로 복사한다(CLAUDE.md §11 — 첫 줄 지시: · 요약: · 원문 그대로).

    python -X utf8 save_re.py <G1..G6> "<요약>"
"""
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RE = os.path.join(r'F:\claude\tfm2\.claude\worktrees\salary-transfer-fee-analysis-1bf168', 'mods_report', 'tfm2_detail_tactics', 'RE')
TOPIC = {'G1': '전투점수', 'G2': '교전위치평가', 'G3': '넥서스방어매복라인', 'G4': '핸들러콜갱', 'G5': '정글빅플랜', 'G6': '팀플랜오브젝트'}
g, summary = sys.argv[1], sys.argv[2]
body = open(os.path.join(HERE, g + '.md'), encoding='utf-8').read()
out = os.path.join(RE, '2026-11-11_0.7.2-창판정_%s_%s.md' % (g, TOPIC[g]))
head = ('지시: 상세전술 0.7.1→0.7.2 창 판정 %s(공통 지침 = MODS\\MIG\\_patchday\\0.7.2\\detail_tactics\\re\\BRIEF.md)\n'
        '요약: %s\n\n---\n\n' % (g, summary))
open(out, 'w', encoding='utf-8', newline='\n').write(head + body)
print(out, os.path.getsize(out))
