"""tactics_verify_summary.json → 마크다운 표.

단일: python -I tools/analyze.py <summary.json> [--out 표.md]
비교: python -I tools/analyze.py <A.json> <B.json> [--out 표.md]   (A=기준, B=변경 또는 같은 조건 재실행)

- '성립' = 비교 바로 뒤 조건 명령(jcc/setcc/cmovcc)의 조건이 참인 비율. 조건 종류는 tools/sites.json 의 cond.
- 관전(fg) = 메인 스레드 + 경기 화면, 배경(bg) = 다른 스레드(다른 경기 시뮬). 비율로만 비교한다
  (같은 조건 재실행에서도 호출 수는 ±2~4% 흔들린다 — DONE 교훈).
"""
import argparse, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
# cmp 칸: 0 a<b(부호없음) · 1 a==b · 2 a>b(부호없음) · 3 a<b(부호) · 4 a>b(부호)
PRED = {
    'b': lambda c, t: c[0], 'nae': lambda c, t: c[0], 'c': lambda c, t: c[0],
    'ae': lambda c, t: t - c[0], 'nb': lambda c, t: t - c[0], 'nc': lambda c, t: t - c[0],
    'be': lambda c, t: c[0] + c[1], 'na': lambda c, t: c[0] + c[1],
    'a': lambda c, t: c[2], 'nbe': lambda c, t: c[2],
    'e': lambda c, t: c[1], 'z': lambda c, t: c[1],
    'ne': lambda c, t: t - c[1], 'nz': lambda c, t: t - c[1],
    'l': lambda c, t: c[3], 'nge': lambda c, t: c[3],
    'ge': lambda c, t: t - c[3], 'nl': lambda c, t: t - c[3],
    'le': lambda c, t: c[3] + c[1], 'ng': lambda c, t: c[3] + c[1],
    'g': lambda c, t: c[4], 'nle': lambda c, t: c[4],
    's': lambda c, t: c[3], 'ns': lambda c, t: t - c[3],  # test r,r = cmp r,0 에서만 유효
}


def cond_suffix(m):
    for p in ('cmov', 'set', 'j'):
        if m.startswith(p):
            return m[len(p):]
    return None


def stats(cls, cond):
    c = cls['cmp']
    total = c[0] + c[1] + c[2]
    out = {'hits': cls['hits'], 'n': total, 'pass': None, 'near': None