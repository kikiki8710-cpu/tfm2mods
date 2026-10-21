# 0.6.5 수동 판정 반영 → repin_map_065.final.json
# 근거 = REPORT tfm2_comptest_unlock\RE\2026-10-21_0.6.5-재핀판정.md · tfm2_elemental_serpen\RE\2026-10-21_0.6.5-재핀판정-serpen_ui_kit_view_plus.md
import json
SRC = r'C:\tfm2mods\MIG\repin_map_065.aligned.json'
DST = r'C:\tfm2mods\MIG\repin_map_065.final.json'
FIX = {
    'tfm2_comptest_unlock': {
        '0xd0dd9':   ('0xd0af9',   'HIGH 엔진 FAIL · 정렬값 owner 0xd0a90 +0x69 (RPLY_ON=false)'),
        '0xd0de4':   ('0xd0b04',   'HIGH 엔진 FAIL · RPLY+0xb'),
        '0x79a560':  ('0x79ad90',  'MED 엔진 FAIL · EH 펑클릿 계보 +0x830 블록 (죽은 상수)'),
        '0x85944d':  ('0xce4edd',  'HIGH DRIVE+0x13 (엔진 0xce9e01 = 후반 다른 블록 오답)'),
        '0x252c82f': ('0x2a52ca0', 'HIGH dr_inline_d 정렬값 · orig cmove r14,rax 재작성 (엔진 0x2a52c6f 오답)'),
        '0x29cc91a': ('0x28b19a3', 'HIGH server_pregate 시그 전역 1히트 0x28b199c+7 (엔진 0x28b1aaa 오답)'),
        '0x29d024c': ('0x28b5371', 'HIGH daily_inc_gate cmp rax,4 @0x28b536e imm (엔진 0x28b53dc 오답)'),
        '0x29e8060': ('0x28cd18e', 'HIGH 엔진 FAIL · 신 핸들러 .pdata 끝'),
        '0x2894f40': ('0x241bc00', 'LOW 죽은 상수 · inc r14 한복판 (엔진 0x241bc20 오답)'),
        '0x3d0b9f8': ('0x3ce5040', 'HIGH 엔진 FAIL · rdata 표 5/5 · size 0x21e0 · ctor lea 1곳'),
    },
    'tfm2_elemental_serpen': {
        '0x7f74a6':  ('0xc848b5',  'HIGH call 0x15ad190 @0xc848b0 +5 (엔진 0xc84926 오답)'),
        '0x7fc5c1':  ('0xc88baf',  'HIGH call 0x15ad190 @0xc88baa +5 (엔진 0xc89a41 오답)'),
        '0x1792080': ('0x186f1e0', 'HIGH 엔진 FAIL · 단일 콜러 0x175f5c0 콜순서 104'),
        '0x171bbd0': ('0x15fc1d0', 'HIGH 엔진 FAIL · 콜리 89=89 · rdata 2곳 · World 문맥 일치'),
    },
    'ui_kit': {
        '0x21457b0': ('0x2360f30', 'HIGH 엔진 FAIL · 니모닉 코사인 0.999999 · 씬 오프셋 정렬 일치'),
        '0x2958160': ('0x2859590', 'HIGH 엔진 FAIL · pdata 없는 leaf · sub rdx,0x56 패턴 · 대핸들러 246회 호출'),
    },
}
m = json.load(open(SRC, encoding='utf-8'))
n = 0
for mod, fx in FIX.items():
    for old, (new, why) in fx.items():
        e = m[mod][old]
        e['new_repin'] = e.get('new')
        e['manual'] = why
        if new is None: e.pop('new', None)
        else: e['new'] = new
        n += 1
miss = [(mod, o, e['name']) for mod, ents in m.items() for o, e in ents.items() if not e.get('new')]
json.dump(m, open(DST, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print('manual', n, '->', DST); print('new 없음(구값 유지):', miss)
