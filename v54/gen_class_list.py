# -*- coding: utf-8 -*-
"""모드/편집기에 심을 "클래스별 값이 먹는 노브" 목록을 생성한다.

기준 = tune() 호출이 **apply_* (바이트패치 적용 함수) 밖**에 있는가.
클래스 조회는 CUR_CLASS(판단 진입 RAII)가 세팅된 동안만 동작하므로,
바이트패치 함수 안에서만 읽히는 노브는 클래스별 값이 원리상 적용될 수 없다."""
import sys, io, re, glob, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
FN = re.compile(r'^\s*(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+([A-Za-z0-9_]+)')
TUNE = re.compile(r'tune\(\s*"([a-zA-Z0-9_]+)"')
BAD = {'key'}          # 주석 속 예시(tune("key",...))가 잡히는 오탐

imm, judge = set(), set()
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    cur = '?'
    for ln in io.open(f, encoding='utf-8'):
        if ln.lstrip().startswith('//'):
            continue                      # 주석 줄은 제외(오탐 차단)
        m = FN.match(ln)
        if m:
            cur = m.group(1)
        for k in TUNE.findall(ln):
            if k in BAD:
                continue
            (imm if cur.startswith('apply_') else judge).add(k)

# ★[08-07] 마이크로 디투어로 연 노브(class_micro.rs MICRO_SITES)도 "먹음"이다.
#   그 노브들은 apply_*_imm 안에서도 tune() 이 불리지만, 실제 상수는 디투어가 클래스별로 공급한다.
#   콜백이 tune(site.key, ...) 를 **변수로** 부르므로 위 스캔에 안 잡힌다 → 표를 직접 읽는다.
MICRO = re.compile(r'key:\s*"([a-zA-Z0-9_]+)"')
micro = set()
mpath = os.path.join(SRC, 'class_micro.rs')
if os.path.exists(mpath):
    body = io.open(mpath, encoding='utf-8').read()
    tbl = body.split('MICRO_SITES: &[MicroSite] = &[', 1)
    if len(tbl) == 2:
        micro = set(MICRO.findall(tbl[1]))

capable = sorted(judge | micro)            # 판단 본문에서 읽힘 or 마이크로 디투어로 열림
imm_only = sorted(imm - judge - micro)
print('먹음 %d (판단 %d + 마이크로 %d) · 안먹음(바이트패치 전용) %d'
      % (len(capable), len(judge), len(micro), len(imm_only)))
if micro:
    print('  마이크로 디투어: ' + ', '.join(sorted(micro)))

rs = ['// ★[08-07 자동생성 — v54/gen_class_list.py] 클래스별 값(`{key}_class_<cls>`)이 **실제로 먹는** 노브.',
      '//   기준 = tune() 호출이 apply_*(바이트패치 적용 함수) 밖에 있는가. 클래스 조회는 CUR_CLASS(판단 진입',
      '//   RAII)가 세팅된 동안만 동작하므로, 바이트패치 안에서만 읽히는 노브는 클래스별 값이 원리상 안 먹는다.',
      '//   ⚠이 목록에 없는 노브에 _class_ 를 걸면 **아무 효과 없이** skip_untuned 최적화만 꺼졌다(08-06 멈춤 사고).',
      'pub static CLASS_CAPABLE: [&str; %d] = [' % len(capable)]
for i in range(0, len(capable), 6):
    rs.append('    ' + ' '.join('"%s",' % x for x in capable[i:i + 6]))
rs.append('];')
io.open('C:/tfm2mods/tfm2_ai_adjust/src/class_capable.rs', 'w', encoding='utf-8', newline='\n').write('\n'.join(rs) + '\n')
print('→ src/class_capable.rs 생성')

ed = ['// ★[08-07 자동생성] 클래스별 값이 먹는 노브. 이 목록에 없으면 편집기에서 클래스별 칸을 숨긴다.',
      'pub const CLASS_CAPABLE: &[&str] = &[']
for i in range(0, len(capable), 6):
    ed.append('    ' + ' '.join('"%s",' % x for x in capable[i:i + 6]))
ed.append('];')
io.open('C:/tfm2mods/ai_adjust_editor/src/class_capable.rs', 'w', encoding='utf-8', newline='\n').write('\n'.join(ed) + '\n')
print('→ 편집기 src/class_capable.rs 생성')
