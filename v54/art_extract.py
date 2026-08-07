# -*- coding: utf-8 -*-
"""발행된 아티팩트 HTML에서 **내 본문만** 뽑아낸다(프레임 런타임 preamble 제거) + 목차 출력."""
import sys, io, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRCF = (r'C:\Users\dev\.claude\projects\C--Users-dev-Desktop-claude-tfm2--claude-worktrees-'
        r'item-tactics-conflict-check-86a5d3\e2b9bb3b-0660-4ff5-9ee7-51903acd7108\tool-results'
        r'\artifact-8fd78e28-1785896427-b9f4.html')
OUT = (r'C:\Users\dev\AppData\Local\Temp\claude\C--Users-dev-Desktop-claude-tfm2--claude-worktrees-'
       r'item-tactics-conflict-check-86a5d3\e2b9bb3b-0660-4ff5-9ee7-51903acd7108\scratchpad\flow.html')

t = io.open(SRCF, encoding='utf-8', errors='replace').read()
i = t.index('<title>TFM2')
j = t.rindex('</body>') if '</body>' in t else len(t)
body = t[i:j].rstrip()
io.open(OUT, 'w', encoding='utf-8', newline='\n').write(body)
print('본문 %d바이트 추출' % len(body))

print('\n== 섹션 뼈대 ==')
for m in re.finditer(r'<section class="band">|<h2[^>]*>(.*?)</h2>|<h4>(.*?)(?:<em>|</h4>)|class="node[^"]*">(\d+)<',
                     body, re.S):
    if m.group(0).startswith('<section'):
        print('  --- band ---')
    elif m.group(1):
        print('  H2  %s' % re.sub(r'<[^>]+>', '', m.group(1)).strip()[:60])
    elif m.group(2):
        print('      h4  %s' % re.sub(r'<[^>]+>', '', m.group(2)).strip()[:60])
    elif m.group(3):
        print('  [%s]' % m.group(3))
print('\n== 버전 표기 위치 ==')
for m in re.finditer(r'0\.5\.[0-9]|2026-08-0[0-9]', body):
    print('  %s @ %d' % (m.group(0), m.start()))
