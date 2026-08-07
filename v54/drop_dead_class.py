# -*- coding: utf-8 -*-
"""바이트패치 노브에 붙은 `_class_` 오버라이드를 제거한다 — **멈춤의 진짜 원인.**

경위(이분탐색 7판으로 확정):
  cfg 에 `_class_` 키가 하나라도 있으면 CLASS_ANY=true (tfm2_ai_adjust.rs:2182) →
  `skip_untuned` 최적화가 **통째로** 꺼진다 (:2186 `if SKIP_UNTUNED && !CLASS_ANY && !CHAMP_ANY`).
  그러면 손대지 않은 판단까지 전부 Rust 재구현 경로로 흘러 배속 재생이 멈춘다.

그런데 이 20개가 붙은 8개 노브는 **전부 apply_*_imm() 에서만 읽힌다 = 바이트 패치**다.
바이트 패치는 exe 기계어 상수를 고치는 것이라 **본질적으로 전역**이고, 클래스별로 다를 수 없다.
게다가 클래스 조회는 CUR_CLASS(판단 진입부 RAII, :680)가 설정된 동안만 동작하는데
패치는 그 밖에서 일어나므로 **클래스 값은 조회조차 되지 않는다.**
⟹ 얻는 것 0, 잃는 것 = 최적화 전체. 빼도 기능 손실이 없다."""
import sys, io, re, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

MOD = ('C:/Program Files (x86)/Steam/steamapps/common/Teamfight Manager2'
       '/mods/tfm2_ai_adjust')
STAGE = 'C:/Users/dev/AppData/Local/Temp/claude/aiadj_rel_0806/tfm2_ai_adjust'
# 바이트패치(apply_*_imm)에서만 읽히는 노브 = 클래스별 적용이 원리상 불가능
IMM_ONLY = ['bt_vision_mem', 'cs_lead_attack', 'ex_order_hold', 'ex_think_min',
            'mv2_avoid_bias', 'mv2_avoid_coef', 'mv2_avoid_margin', 'sf_margin']

FILES = [MOD + '/tfm2_ai_adjust.cfg', MOD + '/config/테스트A.cfg', MOD + '/config/테스트B.cfg',
         MOD + '/config/테스트C.cfg',
         STAGE + '/tfm2_ai_adjust.cfg', STAGE + '/config/테스트A.cfg',
         STAGE + '/config/테스트B.cfg', STAGE + '/config/테스트C.cfg']

NOTE = ('# [08-07] 아래 _class_ 값은 **바이트패치 노브라 클래스별 적용이 불가능**해 원래 동작하지 않았고,\n'
        '#   존재하는 것만으로 skip_untuned 최적화를 전부 꺼서 재생이 멈췄습니다. 그래서 뺍니다.\n')
for p in FILES:
    if not os.path.exists(p):
        print('  [없음] %s' % os.path.basename(p)); continue
    out, n = [], 0
    for ln in io.open(p, encoding='utf-8'):
        s = ln.strip()
        if s and not s.startswith('#') and '=' in s:
            k = s.split('=', 1)[0].strip()
            if any(k.startswith(b + '_class_') for b in IMM_ONLY):
                if n == 0:
                    out.append(NOTE)
                out.append('# [죽은 값 — 제거] ' + s + '\n'); n += 1; continue
        out.append(ln)
    io.open(p, 'w', encoding='utf-8', newline='\n').write(''.join(out))
    left = len([1 for l in ''.join(out).split('\n')
                if l.strip() and not l.strip().startswith('#') and '_class_' in l])
    print('  %-8s %-20s 제거 %2d개 · 남은 _class_ 키 %d개'
          % (os.path.basename(os.path.dirname(p))[:6], os.path.basename(p), n, left))
