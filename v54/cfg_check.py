# -*- coding: utf-8 -*-
"""cfg 파일을 현행(0.5.4) 노브 집합과 대조해 '변환이 필요한지' 판정한다.
   ①없는 키(배선 0) ②죽은 노브(설명에 ⛔) ③단위가 바뀐 키 ④imm8 범위 초과 후보"""
import sys, io, re, json, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

st = json.load(io.open('C:/tfm2mods/v54/audit_state.json', encoding='utf-8'))
wired, desc, in_tab = set(st['wired']), st['desc'], set(st['in_tab'])
DEAD = re.compile(r'⛔|\[은퇴\]|작동하지 않습니다|폐기된 값')

# 0.5.4에서 단위/의미가 바뀐 키
UNIT = {'pe_noise_exempt': ('÷1000', lambda v: max(0, min(127, round(v / 1000))) if v > 200 else v)}

FILES = sys.argv[1:]
for path in FILES:
    raw = open(path, 'rb').read()
    txt = raw.decode('utf-8')
    print('\n' + '=' * 70)
    print('%s  (%d bytes, BOM %s)' % (os.path.basename(path), len(raw),
                                      '있음★' if raw[:3] == b'\xef\xbb\xbf' else '없음'))
    keys = {}
    for i, ln in enumerate(txt.split('\n')):
        s = ln.strip()
        if not s or s.startswith('#') or '=' not in s:
            continue
        k, v = s.split('=', 1)
        keys[k.strip()] = (v.strip(), i + 1)
    print('설정된 키 = %d' % len(keys))

    miss, dead, unit, big = [], [], [], []
    for k, (v, ln) in sorted(keys.items()):
        if k.startswith('__') or k in ('champion', 'class'):
            continue
        base = re.sub(r'_class_(melee|range|magician|util|assassin)$', '', k)
        if base not in wired:
            miss.append((k, v, ln)); continue
        d = desc.get(base, '')
        if DEAD.search(d):
            dead.append((k, v, ln))
        if base in UNIT:
            unit.append((k, v, ln))
        try:
            iv = int(v)
        except ValueError:
            continue
        if iv > 127 and re.search(r'0~127|imm8', d):
            big.append((k, v, ln))

    def show(t, rows):
        print('  %s : %d건' % (t, len(rows)))
        for k, v, ln in rows[:20]:
            print('     L%-4d %-24s = %s' % (ln, k, v))
        if len(rows) > 20:
            print('     ... 외 %d건' % (len(rows) - 20))

    show('★배선에 없는 키(무반응)', miss)
    show('★죽은 노브(⛔ — 값 무의미)', dead)
    show('★단위가 바뀐 키(0.5.4)', unit)
    show('⚠범위(0~127) 초과 의심', big)
    if not (miss or dead or unit or big):
        print('  → 변환할 것 없음. 그대로 써도 된다.')
