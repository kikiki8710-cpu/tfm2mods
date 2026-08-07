# -*- coding: utf-8 -*-
"""pe_noise_exempt 원상복구 (100 → 100000).

내가 0.5.4 단위 변경이라고 보고 100000→100 으로 "환산"했는데 **둘 다 틀렸다**:
  ① 100000 은 유저가 고친 값이 아니라 **baseline(default.txt:443)의 기본값** 그대로였다.
  ② 모드가 **이미 내부에서 1/1000 을 한다**(detour.rs:1809 `pnex / 1000`) — cfg 단위는 옛 단위 그대로.
     그래서 100 을 넣으면 v=0 이 되어 `[+0x1f8] >= 0` 이 항상 참 = **전원 면제 = 노이즈 기능이 꺼진다.**
원본 동작으로 되돌리려던 변경이 정반대로 기능을 껐다."""
import sys, io, re, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

MOD = ('C:/Program Files (x86)/Steam/steamapps/common/Teamfight Manager2'
       '/mods/tfm2_ai_adjust')
STAGE = ('C:/Users/dev/AppData/Local/Temp/claude/aiadj_rel_0806/tfm2_ai_adjust')
FILES = [MOD + '/tfm2_ai_adjust.cfg', MOD + '/config/테스트B.cfg', MOD + '/config/테스트C.cfg',
         STAGE + '/tfm2_ai_adjust.cfg', STAGE + '/config/테스트B.cfg', STAGE + '/config/테스트C.cfg']

PAT = re.compile(r'(?m)^\s*pe_noise_exempt\s*=\s*(\S+).*$')
for p in FILES:
    if not os.path.exists(p):
        print('  [없음] %s' % p); continue
    t = io.open(p, encoding='utf-8').read()
    m = PAT.search(t)
    if not m:
        print('  [키 없음] %s' % os.path.basename(p)); continue
    old = m.group(1)
    # 잘못 붙여둔 "단위 환산" 주석도 같이 걷어낸다
    t = re.sub(r'(?m)^#\s*\[0\.5\.4 단위 환산\].*\n', '', t)
    t = PAT.sub('pe_noise_exempt = 100000', t)
    io.open(p, 'w', encoding='utf-8', newline='\n').write(t)
    now = PAT.search(io.open(p, encoding='utf-8').read()).group(1)
    print('  %-14s %-20s %s → %s' % (os.path.basename(os.path.dirname(p)),
                                      os.path.basename(p), old, now))
