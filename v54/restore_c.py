# -*- coding: utf-8 -*-
"""테스트C 를 **원본 값 그대로** 복원한다 (내 변경 중 동작을 바꾼 것만 되돌림).

빈 cfg 로는 멈추지 않았으므로 원인은 cfg 값 안에 있다. 내가 원본에 가한 변경 중
실제로 패치되는 값을 바꾼 것은 **`-1` → 실제 기본값 펼치기 9건** 뿐이다.
`-1` 은 '그 바이트를 건드리지 않는다', 펼친 값은 '같다고 믿는 값으로 덮어쓴다' 이므로
내가 채운 기본값이 하나라도 틀렸으면 그대로 오패치가 된다. 전부 `-1` 로 되돌린다.

그 밖의 변경은 유지: oi_*→nx_* 개명(알리아스라 무영향)·계측 플래그 탈락·ct_hunt/sp_seen=0·e9jt=0.
pe_noise_exempt 는 원본값 100000 그대로."""
import sys, io, re, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

MOD = ('C:/Program Files (x86)/Steam/steamapps/common/Teamfight Manager2'
       '/mods/tfm2_ai_adjust')
STAGE = 'C:/Users/dev/AppData/Local/Temp/claude/aiadj_rel_0806/tfm2_ai_adjust'

# 내가 -1 → 기본값 으로 펼쳤던 9개. 원본 테스트C 에서는 전부 -1 이었다.
EXPANDED = ['gb_op_phase', 'gb_push_hp', 'gb_reach_cap', 'gb_reach_margin',
            'gb_scout_radius', 'gk_hp_base_gank', 'vw_jungle', 'vw_nexus', 'vw_score']

SRC = MOD + '/config/_실험전_현재값_20260806_2350.cfg'   # 실험 직전 = 현재 테스트C 값
t = io.open(SRC, encoding='utf-8').read()
done = []
for k in EXPANDED:
    pat = re.compile(r'(?m)^(\s*%s\s*=\s*)(\S+)(.*)$' % k)
    m = pat.search(t)
    if not m:
        print('  [키 없음] %s' % k); continue
    if m.group(2) != '-1':
        t = pat.sub(lambda mm: mm.group(1) + '-1' + mm.group(3), t)
        done.append('%s %s→-1' % (k, m.group(2)))

for p in [MOD + '/tfm2_ai_adjust.cfg', MOD + '/config/테스트C.cfg',
          STAGE + '/tfm2_ai_adjust.cfg', STAGE + '/config/테스트C.cfg']:
    io.open(p, 'w', encoding='utf-8', newline='\n').write(t)

print('되돌림 %d건:' % len(done))
for d in done:
    print('   ' + d)
act = len([1 for l in t.split('\n') if l.strip() and not l.strip().startswith('#') and '=' in l])
print('\n활성 키 = %d   첫3바이트 = %s' % (act, open(MOD + '/tfm2_ai_adjust.cfg', 'rb').read(3).hex()))
