# -*- coding: utf-8 -*-
"""`_class_` 오버라이드가 **실제로 먹히는 노브**와 **먹지 않는 노브**를 전수로 가른다.

원리: 클래스 값 조회는 thread-local CUR_CLASS 가 세팅된 동안만 동작한다(tfm2_ai_adjust.rs:484·509).
CUR_CLASS 는 **판단(judge) 진입 RAII 가드**에서만 세팅된다(:680).
  · 판단 본문에서 읽는 tune() → 클래스 값 먹음     ✅
  · apply_*_imm() 안에서 읽는 tune() → 바이트 패치라 전역, 클래스 조회조차 안 됨  ❌"""
import sys, io, re, glob, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
FN = re.compile(r'^\s*(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+([A-Za-z0-9_]+)')
TUNE = re.compile(r'tune\(\s*"([a-zA-Z0-9_]+)"')

imm, judge = {}, {}
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    cur = '?'
    for ln in io.open(f, encoding='utf-8'):
        m = FN.match(ln)
        if m:
            cur = m.group(1)
        for k in TUNE.findall(ln):
            # apply_*_imm / apply_* 계열 = 바이트 패치 적용 함수
            (imm if (cur.startswith('apply_')) else judge).setdefault(k, set()).add(cur)

only_imm = sorted(set(imm) - set(judge))
both = sorted(set(imm) & set(judge))
only_judge = sorted(set(judge) - set(imm))

print('클래스별 값이 **먹는** 노브 (판단 본문에서 읽힘)      : %d개' % len(only_judge))
print('클래스별 값이 **안 먹는** 노브 (바이트패치 전용)      : %d개' % len(only_imm))
print('양쪽에서 읽히는 노브 (부분적으로 먹음 — 확인 필요)   : %d개' % len(both))

io.open('C:/tfm2mods/v54/class_capable.txt', 'w', encoding='utf-8').write(
    '[클래스별 값 먹음]\n' + '\n'.join(only_judge) +
    '\n\n[클래스별 값 안 먹음 = 바이트패치 전용]\n' + '\n'.join(only_imm) +
    '\n\n[양쪽]\n' + '\n'.join('%s  imm=%s judge=%s' % (k, sorted(imm[k]), sorted(judge[k])) for k in both) + '\n')

print('\n── 클래스별 값이 먹는 노브 (앞 60개) ──')
for i in range(0, min(60, len(only_judge)), 5):
    print('  ' + '  '.join('%-22s' % x for x in only_judge[i:i+5]))
if both:
    print('\n── 양쪽에서 읽히는 노브 ──')
    for k in both:
        print('  %-22s imm=%s / judge=%s' % (k, sorted(imm[k]), sorted(judge[k])))
print('\n(전체 목록 = v54/class_capable.txt)')
