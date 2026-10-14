# 0.6.4 수동 판정 반영 → repin_map_064.final.json
# 근거 = REPORT tfm2_comptest_unlock\RE\2026-10-14_0.6.4-재핀판정.md · tfm2_elemental_serpen\RE\2026-10-14_0.6.4-재핀판정-serpen_ui_kit_view_plus.md
import json
SRC = r'C:\tfm2mods\MIG\repin_map_064.aligned.json'
DST = r'C:\tfm2mods\MIG\repin_map_064.final.json'
FIX = {
    'tfm2_comptest_unlock': {
        '0xd0e99':   ('0xd0dd9',   'HIGH 정렬값 채택 owner 0xd0d70 +0x69 (RPLY_ON=false)'),
        '0xd0ea4':   ('0xd0de4',   'HIGH 정렬값 채택 RPLY+0xb'),
        '0x15501f0': ('0x1902e80', 'MED 엔진 0x23b3724 오답(별개 함수) · owner 0x1902e70 +0x10 (미사용)'),
        '0x136ee30': (None,        '미해결 · 엔진 0x21ccf10 은 다른 함수 · SIM_PROBE_ON=false 미사용 → 구값 유지'),
        '0x1c46968': ('0x29cc91a', 'HIGH server_pregate 시그 전역 1히트 0x29cc913+7 (엔진 0x29cc888 오답)'),
        '0x1c49f97': ('0x29d024c', 'HIGH daily_inc_gate cmp rax,4 @0x29d0249 imm (엔진 0x29cfeb7 오답)'),
        '0x1c619a3': ('0x29e8060', 'HIGH 신 핸들러 .pdata 끝'),
        '0x1c8f885': ('0x2a16881', 'HIGH server_roster_min add rbx,rbx (orig/fixed 재작성)'),
        '0x1c8f964': ('0x2a16920', 'HIGH server_dedup_real jne d8 (엔진 0x2a169a4 오답)'),
        '0x3c1f2f0': ('0x3d0b9f8', 'HIGH rdata 표 2/2 · size 0x21e0 · ctor lea 1곳'),
    },
    'tfm2_elemental_serpen': {
        '0xc73d36': ('0x7f74a6', 'HIGH call 0x16d3140 @0x7f74a1 +5 (엔진 0x7f7446 오답)'),
        '0xc78e41': ('0x7fc5c1', 'HIGH call 0x16d3140 @0x7fc5bc +5 (엔진 0x7fc551 오답)'),
    },
    'ui_kit': {
        '0x2272f20': ('0x21457b0', 'HIGH 엔진 FAIL · 니모닉 코사인 0.999999 · D_* 함수와 간격 0x29c0 유지'),
        '0x1bd9e80': ('0x2958160', 'HIGH 엔진 FAIL · sub rdx,0x56 패턴 · 대핸들러 246회 호출'),
        '0x306c50':  ('0x306bd0',  'HIGH 계보(jmp 신 DROP_MAP 0x2f65d0) · 엔진 0x1d31de0 도 기능 동일'),
    },
}
m = json.load(open(SRC, encoding='utf-8'))
n = 0
for mod, fx in FIX.items():
    for old, (new, why) in fx.items():
        e = m[mod][old]
        e['new_repin'] = e.get('new')
        e['manual'] = why
        if new is None:
            e.pop('new', None)
        else:
            e['new'] = new
        n += 1
json.dump(m, open(DST, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print('manual', n, '->', DST)
