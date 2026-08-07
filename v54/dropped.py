# -*- coding: utf-8 -*-
"""테스트C 변환에서 **주석 처리(탈락)** 시킨 키를 값과 함께 뽑고, 후계 키 후보를 찾는다.
   후계 찾는 법: ①이름이 거의 같은 현행 키(접미/접두 차이) ②설명이 같은 개념을 가리키는 키"""
import sys, io, re, json, difflib
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

st = json.load(io.open('C:/tfm2mods/v54/audit_state.json', encoding='utf-8'))
wired, in_tab, desc = set(st['wired']), set(st['in_tab']), st['desc']
DEAD = re.compile(r'⛔|\[은퇴\]|작동하지 않습니다|폐기된 값')
CLS = re.compile(r'_class_(melee|range|magician|util|assassin)$')

SRC = r'C:\Users\dev\Downloads\tfm2_ai_adjust\tfm2_ai_adjust.cfg'
miss, dead = [], []
for ln in io.open(SRC, encoding='utf-8').read().split('\n'):
    s = ln.strip()
    if not s or s.startswith('#') or '=' not in s:
        continue
    k, v = [x.strip() for x in s.split('=', 1)]
    if k.startswith('__'):
        continue
    base = CLS.sub('', k)
    if base not in wired:
        miss.append((k, v))
    elif DEAD.search(desc.get(base, '')):
        dead.append((k, v))

cands = sorted(in_tab)
print('== 배선에 없어 탈락시킨 키 %d개 — 후계 후보 ==' % len(miss))
for k, v in miss:
    near = difflib.get_close_matches(k, cands, n=3, cutoff=0.62)
    near = [x for x in near if x != k]
    print('  %-24s = %-10s 후보: %s' % (k, v, ', '.join(near) if near else '(없음)'))

print('\n== 죽은 노브라 주석 처리한 키 %d개 (키 자체는 존재) ==' % len(dead))
for i in range(0, len(dead), 4):
    print('   ' + '  '.join('%s=%s' % (k, v) for k, v in dead[i:i + 4]))
