#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""tcx 덤프에서 fn 시그니처를 lifetime 제거해 보기 좋게 출력.
사용: python sig.py <크레이트> <부분문자열> [<부분문자열>...]"""
import json, io, re, sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

crate = sys.argv[1]
pats = sys.argv[2:]
d = json.load(io.open('C:/tfm2mods/MIG/_tcx/%s.json' % crate, encoding='utf-8'))

LT = re.compile(r"&'\^?\d*\.?\w*\(DefId\([^)]*\)\)\s*")
LT2 = re.compile(r"'\^\d+\.Named\(DefId\([^)]*\)\)\s*")


def clean(s):
    if not s:
        return s
    s = LT.sub('&', s)
    s = LT2.sub('', s)
    s = re.sub(r'\s+', ' ', s)
    return s


for it in d['items']:
    p = it.get('p', '')
    if not any(x in p for x in pats):
        continue
    sp = it.get('sp') or {}
    print('%s' % p)
    print('   v=%s  mir=%s  xinl=%s  k=%s  %s:%s' % (
        it.get('v'), it.get('mir'), it.get('xinl'), it.get('k'),
        sp.get('f'), sp.get('l')))
    if it.get('sig'):
        print('   sig= %s' % clean(it['sig']))
    print()
