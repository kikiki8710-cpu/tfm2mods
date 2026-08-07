# -*- coding: utf-8 -*-
"""소스에서 센 패치 사이트 수 ↔ 인게임 실측 `applied=N/M` 의 M 을 함수별로 대조한다.

목적 = "내 파서가 어느 함수를 못 봤나"를 특정. 총계만 보면 어디가 비는지 모른다.
(실측 824 vs 파서 721 = 103 누락. 총계로는 원인을 못 찾는다.)
"""
import io, os, re, sys, collections

sys.path.insert(0, r'C:\tfm2mods\v54')
import sites as S1
import sites2 as S2
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRCDIR = r'C:\tfm2mods\tfm2_ai_adjust\src'

# 인게임 실측 M (2026-08-05 14:21 한 경기)
RUNTIME = {
    'ae': 13, 'auction': 71, 'bv': 35, 'c3': 63, 'cast': 38, 'd19': 10, 'd4': 24,
    'db': 43, 'eh': 56, 'exec': 6, 'gank': 14, 'gb': 9, 'hd': 47, 'ldsc': 6,
    'lt': 5, 'lv': 9, 'move': 27, 'move2': 12, 'new': 18, 'nx': 8, 'obj': 14,
    'pe': 103, 'plan': 7, 'rt': 11, 'score': 22, 'score2': 40, 'sev': 33,
    'th': 54, 'vis': 1, 'visshort': 25,
}


def func_ranges(path):
    lines = io.open(path, encoding='utf-8').read().split('\n')
    b = [i for i, l in enumerate(lines) if re.match(r'(unsafe )?fn \w+', l)]
    out = []
    for k, s in enumerate(b):
        e = b[k + 1] if k + 1 < len(b) else len(lines)
        out.append((re.match(r'(?:unsafe )?fn (\w+)', lines[s]).group(1), s + 1, e + 1))
    return out


def main():
    ranges = {}
    for fn in ('detour.rs', 'disc19_repro.rs'):
        for name, s, e in func_ranges(os.path.join(SRCDIR, fn)):
            ranges[(fn, name)] = (s, e)

    cnt = collections.Counter()
    for x in S1.parse() + S2.parse():
        for (f, name), (s, e) in ranges.items():
            if x['file'] == f and s <= x['line'] < e:
                cnt[name] += 1
                break
        else:
            cnt['(범위밖) ' + x['file']] += 1

    # apply_XXX_imm → 로그 키 추정
    print('%-28s %6s %6s  %s' % ('함수', '소스', '실측M', '차이'))
    used = set()
    for name in sorted(cnt):
        m = re.match(r'apply_(\w+?)_imm$', name)
        key = m.group(1) if m else None
        rt = RUNTIME.get(key)
        if rt is not None:
            used.add(key)
        d = ('%+d' % (cnt[name] - rt)) if rt is not None else '-'
        flag = '  ★불일치' if rt is not None and cnt[name] != rt else ''
        print('%-28s %6d %6s  %s%s' % (name, cnt[name], rt if rt is not None else '-', d, flag))

    print('\n실측엔 있는데 소스 함수로 못 묶인 키:')
    for k, v in RUNTIME.items():
        if k not in used:
            print('  %-12s M=%d' % (k, v))
    print('\n소스 합계 %d / 실측 합계 %d' % (sum(cnt.values()), sum(RUNTIME.values())))


if __name__ == '__main__':
    main()
