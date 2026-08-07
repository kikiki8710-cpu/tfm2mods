# -*- coding: utf-8 -*-
"""편집기에 노출된 전 설정값을 탭 구조 그대로 Notion 마크다운으로 뽑는다.
   HTML 조각(<b>·<br>·\\ 줄이음)을 노션 마크다운으로 바꾸고, 원본값을 함께 싣는다."""
import sys, io, re, json

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
st = json.load(io.open('C:/tfm2mods/v54/audit_state.json', encoding='utf-8'))
desc, orig, tabs = st['desc'], st['orig'], st['tabs']

ed = io.open('C:/tfm2mods/ai_adjust_editor/src/main.rs', encoding='utf-8').read()
TITLE = {}
for m in re.finditer(r'Tab\{\s*id:"(\w+)",\s*title:"([^"]*)"', ed):
    TITLE[m.group(1)] = m.group(2).lstrip('• ').strip()

DEAD = re.compile(r'⛔|\[은퇴\]|작동하지 않습니다|폐기된 값')


def clean(s):
    s = s.replace('\\"', '"')
    s = re.sub(r'<br\s*/?>', ' ', s)
    s = re.sub(r'</?b>', '**', s)
    s = re.sub(r'</?[a-zA-Z][^>]*>', '', s)
    s = s.replace('\\\n', ' ')
    s = re.sub(r'\s+', ' ', s).strip()
    return s


# 층별 묶음 (문서 분할 단위)
GROUPS = [
    ('01', '판단을 만드는 단계', ['planpick', 'lane', 'recall', 'object', 'battle', 'def',
                                  'disc17', 'disc19', 'nexus_def_misc', 'gb', 'nexus_auction']),
    ('02', '실행 단계', ['exec', 'cast', 'hide', 'movein', 'pathsys']),
    ('03', '모든 단계가 함께 쓰는 계산', ['posrisk', 'severity', 'judge', 'vision']),
    ('04', '교전·합류·포탑·능력치 · 기타', ['regrouped', 'misc', 'engine']),
]

out_files = []
for no, gname, tabids in GROUPS:
    body = []
    total = 0
    for tid in tabids:
        keys = tabs.get(tid, [])
        if not keys:
            continue
        body.append('## %s' % TITLE.get(tid, tid))
        live = [k for k in keys if not k.startswith('§')]
        body.append('> 설정값 %d개' % len(live))
        body.append('')
        for k in keys:
            if k.startswith('§'):
                body.append('### ' + k.lstrip('§◆ ').strip())
                body.append('')
                body.append('| 키 | 원본값 | 설명 |')
                body.append('| --- | --- | --- |')
                continue
            if not body or not body[-1].startswith('|'):
                body.append('| 키 | 원본값 | 설명 |')
                body.append('| --- | --- | --- |')
            d = clean(desc.get(k, '(설명 없음)'))
            o = orig.get(k)
            if not o:
                mo = re.search(r'원본\s*(?:값\s*)?\**\s*(0[xX][0-9a-fA-F]+|[0-9][0-9,]*)', d)
                o = mo.group(1) if mo else '—'
            mark = '⛔ ' if DEAD.search(d) else ''
            body.append('| `%s` | %s | %s%s |' % (k, o, mark, d))
            total += 1
        body.append('')
    txt = '\n'.join(body)
    path = 'C:/tfm2mods/v54/knobdoc_%s.md' % no
    io.open(path, 'w', encoding='utf-8', newline='\n').write(txt)
    out_files.append((no, gname, total, len(txt), path))
    print('%s %-28s 설정값 %3d  %6d자  %s' % (no, gname, total, len(txt), path))
print('\n합계 %d개' % sum(x[2] for x in out_files))
