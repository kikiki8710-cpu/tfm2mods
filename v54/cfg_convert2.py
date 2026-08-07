# -*- coding: utf-8 -*-
"""테스트C 재변환 — 원본(Downloads cfg)에서 다시 만든다.

기준(유저 지시): **원래 값을 그대로 쓰되, 없어진 키는 후계가 있으면 옮기고 정 없으면 탈락.**
  ① `oi_*` 9개 → 모드의 알리아스 표대로 `nx_*` 로 **개명해서 값 유지**
     (모드가 로드 때 자동 개명하므로 원래도 작동하던 키였다 — 1차 변환에서 잘못 죽였다)
  ② `ldsc_early_mask` 는 **원래 값 128611 로 유지** (1차에서 새 원본값으로 올린 것은 추측이었다)
  ③ 죽은 노브 27개는 **주석 해제** — 키가 사라진 게 아니라 게임 쪽 코드가 없을 뿐이다
  ④ 개발용 계측 플래그 20개 + `mv_vision_mem` 은 탈락 유지
     (`mv_vision_mem` 은 소스가 "알리아스를 두지 않는다 — 옛 값이 조용히 전역 적용되면 더 위험"이라 명시)
  ⑤ `pe_noise_exempt` 만 단위 환산 유지(0.5.4에서 1/1000 로 바뀜 — 그대로 두면 127 로 잘린다)
"""
import sys, io, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRC = r'C:\Users\dev\Downloads\tfm2_ai_adjust\tfm2_ai_adjust.cfg'
DSTS = [r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2'
        r'\mods\tfm2_ai_adjust\config\테스트C.cfg']

# 모드 알리아스 표(tfm2_ai_adjust.rs)에서 oi_* 부분
ALIAS = {
    'oi_enable': 'nx_enable', 'oi_dn_count_gate': 'nx_dn_count_gate',
    'oi_dn_nexus_hp': 'nx_dn_nexus_hp', 'oi_dn_hp_crit': 'nx_dn_hp_crit',
    'oi_dn_hp_low': 'nx_dn_hp_low', 'oi_dn_near_dist': 'nx_dn_near_dist',
    'oi_dn_pred_dist': 'nx_dn_pred_dist', 'oi_dn_lane_margin': 'nx_dn_vision_mem',
    'oi_an_count_gate': 'nx_an_count_gate', 'oi_an_finish_hp': 'nx_an_finish_hp',
    'oi_an_cull_dist': 'nx_an_cull_dist',
}
# 후계가 없어 탈락시키는 것(개발용 계측·프로브 + 의도적 알리아스 부재)
DROP = {'replace', 'disppred', 'move', 'move_tag', 'move_off', 'move_x', 'move_y',
        'dd7cap', 'rngcap', 'rng_repl', 'pgcap', 'pg_a', 'pg_b', 'pg_c', 'tecap',
        'seed_rotate', 'dmgcap', 'candcap', 'gbcap', 'gbcallee', 'strat_rotate',
        'e9a30cap', 'class_probe', 'class_sheet', 'mv_vision_mem'}

t = io.open(SRC, encoding='utf-8').read()
out, n_alias, n_drop, n_unit = [], 0, 0, 0
for ln in t.split('\n'):
    s = ln.strip()
    if not s or s.startswith('#') or '=' not in s:
        out.append(ln); continue
    k, v = [x.strip() for x in s.split('=', 1)]

    if k in DROP:
        out.append('# [0.5.4 탈락 — 후계 키 없음] ' + s); n_drop += 1; continue

    if k in ALIAS:
        out.append('# [0.5.4 개명] %s → %s (값 유지)' % (k, ALIAS[k]))
        out.append('%s = %s' % (ALIAS[k], v)); n_alias += 1; continue

    if k == 'pe_noise_exempt':
        try:
            iv = int(v)
            if iv > 200:
                out.append('# [0.5.4 단위 환산] 이 값의 단위가 1/1000 로 바뀌었다. %s 를 그대로 두면 0~127 로 잘려 127 이 된다.' % v)
                out.append('%s = %d' % (k, round(iv / 1000))); n_unit += 1; continue
        except ValueError:
            pass

    out.append(ln)

hdr = ['# 테스트C (게임 0.5.4) — 원본: ' + SRC.replace('\\', '/'),
       '# 변환 원칙: 원래 값 그대로 유지. 없어진 키만 후계로 옮기고, 후계가 없으면 탈락.',
       '#   개명 %d · 탈락 %d · 단위 환산 %d · 그 밖의 값은 손대지 않음' % (n_alias, n_drop, n_unit),
       '#   ※죽은 노브(dm_*·d4_* 등)는 키가 사라진 게 아니라 게임 쪽 코드가 없을 뿐이라 그대로 둔다.', '']
body = '\n'.join(hdr + out)
for d in DSTS:
    io.open(d, 'w', encoding='utf-8', newline='\n').write(body)

print('개명 %d · 탈락 %d · 단위 환산 %d' % (n_alias, n_drop, n_unit))
print('첫3바이트 =', open(DSTS[0], 'rb').read(3).hex())
act = len([1 for l in body.split('\n') if l.strip() and not l.strip().startswith('#') and '=' in l])
print('활성 키 = %d' % act)
