# -*- coding: utf-8 -*-
"""e9jt 를 다시 0(꺼짐)으로 — 이번 멈춤/크래시의 원인.

e9jt=1 이면 my_e9a30_jt_v() 가 vtable 함수포인터를 transmute 해서 **게임 함수를 직접 호출**한다
(jt_fn(vbuf) / v_fn(sret, vbuf)). CLAUDE.md §3 = "게임함수 shadow-CALL 도 AV 위험 →
위험 shadow-call 은 cfg 게이트(기본 OFF)로 격리". 코드의 `static E9_JT = AtomicBool::new(false)` 가
그 안전 게이트이고, 유저 cfg 의 `e9jt = 0` 이 설계에 맞는 값이었다.

내가 `config\\default.txt` 의 `e9jt = 1` 을 "모드 기본값" 으로 읽고 켰는데,
**그 baseline 항목이 코드와 모순된 잘못된 값**이었다. baseline 도 0 으로 정정한다."""
import sys, io, re, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

MOD = ('C:/Program Files (x86)/Steam/steamapps/common/Teamfight Manager2'
       '/mods/tfm2_ai_adjust')
STAGE = 'C:/Users/dev/AppData/Local/Temp/claude/aiadj_rel_0806/tfm2_ai_adjust'
FILES = [MOD + '/tfm2_ai_adjust.cfg', MOD + '/config/테스트A.cfg', MOD + '/config/테스트B.cfg',
         MOD + '/config/테스트C.cfg', MOD + '/config/default.txt',
         STAGE + '/tfm2_ai_adjust.cfg', STAGE + '/config/테스트A.cfg', STAGE + '/config/테스트B.cfg',
         STAGE + '/config/테스트C.cfg', STAGE + '/config/default.txt']

PAT = re.compile(r'(?m)^(\s*e9jt\s*=\s*)(\S+)(.*)$')
for p in FILES:
    if not os.path.exists(p):
        print('  [없음] %s' % os.path.basename(p)); continue
    t = io.open(p, encoding='utf-8').read()
    m = PAT.search(t)
    if not m:
        print('  [키 없음] %s' % os.path.basename(p)); continue
    old = m.group(2)
    t = PAT.sub(lambda mm: mm.group(1) + '0' + mm.group(3), t)
    io.open(p, 'w', encoding='utf-8', newline='\n').write(t)
    print('  %-8s %-20s e9jt %s → 0' % (os.path.basename(os.path.dirname(p))[:6],
                                        os.path.basename(p), old))

# 편집기 기본값도 코드(안전 게이트 false)에 맞춘다
E = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
s = io.open(E, encoding='utf-8').read()
n = 0
for old, new in [('"e9jt" => "1"', '"e9jt" => "0"')]:
    if old in s:
        s = s.replace(old, new); n += s.count(new)
io.open(E, 'w', encoding='utf-8', newline='\n').write(s)
print('\n편집기 기본값 정정 = %s' % ('완료' if n else '해당 항목 없음(확인 필요)'))
