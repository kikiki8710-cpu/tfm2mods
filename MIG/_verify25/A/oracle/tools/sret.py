# -*- coding: utf-8 -*-
"""② sret 원소: 태그 store(+177) · push/extend/truncate 호출 · 생성자 호출 전수"""
from load import *
import re
# alloca 크기 사전
alloc = {}
for i in range(LO, HI+1):
    m = re.match(r'\s*(%\d+) = alloca \[(\d+) x i8\]', L(i))
    if m: alloc[m.group(1)] = int(m.group(2))
# gep +177 from alloca
g177 = {}
for i in range(LO, HI+1):
    m = re.match(r'\s*(%\d+) = getelementptr inbounds (?:nuw )?i8, ptr (%\d+), i64 177', L(i))
    if m: g177[m.group(1)] = m.group(2)
print('== 태그 store (+0xb1) ==')
for i in range(LO, HI+1):
    m = re.match(r'\s*store i8 (\d+), ptr (%\d+)', L(i))
    if m and m.group(2) in g177:
        base = g177[m.group(2)]
        print(i, 'tag=%s' % m.group(1), 'elem=%s(%dB)' % (base, alloc.get(base, -1)), ';L'+chain(i))
print('== push/extend/truncate/reserve/from_iter/new_in 호출 ==')
pat = re.compile(r'call [^@]*@(\S*(?:push|extend|truncate|reserve_internal|from_iter_in|Extend)\S*)\(([^)]*)\)')
for i in range(LO, HI+1):
    s = L(i)
    if 'call' in s and re.search(r'4push|6extend|8truncate|reserve_internal|from_iter_in|6Extend', s):
        sym = re.search(r'@(\S+?)\(', s)
        args = re.search(r'\(([^)]*)\)', s[s.find('@'):])
        print(i, (sym.group(1)[-60:] if sym else '?'), '|', (args.group(1)[:120] if args else ''), ';L'+chain(i))
print('== 생성자/콜리 sret 호출(SmallAction*) ==')
for i in range(LO, HI+1):
    s = L(i)
    if 'call' in s and re.search(r'SmallAction|base_battle_action|kite_reposition_point|legacy_', s):
        sym = re.search(r'@(\S+?)\(', s)
        args = re.search(r'\(([^)]*)\)', s[s.find('@'):])
        print(i, (sym.group(1)[-70:] if sym else '?'), '|', (args.group(1)[:140] if args else ''), ';L'+chain(i))
print('== memcpy 184B (원소 복사) ==')
for i in range(LO, HI+1):
    s = L(i)
    if 'llvm.memcpy' in s and 'i64 184' in s:
        print(i, s.strip()[:150], ';L'+chain(i))
