# -*- coding: utf-8 -*-
"""함수 쌍의 **즉시값(임계치) 다중집합**을 비교 — 'AI 판단 상수가 바뀌었나'.

bodydiff 의 유사도는 스택프레임/레지스터 재배치만으로도 0.4 까지 떨어져서
'로직이 바뀌었나'의 지표로 못 쓴다(0.5.4 는 구조체 오프셋이 통째로 밀렸다).
즉시값 다중집합은 그 잡음에 거의 안 흔들린다.
"""
import sys, collections
sys.path.insert(0, r'C:\tfm2mods\v54')
import capstone
from pe2 import load, BASE
from jtscan import fast_func_of

SKIP_MN = {'call', 'jmp', 'lea'}


def imms(e, s, en):
    c = collections.Counter()
    for i in e.dis(s, en - s):
        if i.mnemonic in SKIP_MN or i.mnemonic.startswith('j'):
            continue
        for op in i.operands:
            if op.type == capstone.x86.X86_OP_IMM:
                v = op.imm
                if -0x10000 < v < 0x10000000 and v not in (0, 1, -1):
                    c[v] += 1
    return c


def cmp2(a, b):
    ea, eb = load('053'), load('054')
    fa, fb = fast_func_of(ea, a), fast_func_of(eb, b)
    ca, cb = imms(ea, *fa), imms(eb, *fb)
    only_a = ca - cb
    only_b = cb - ca
    same = sum((ca & cb).values())
    return same, only_a, only_b
