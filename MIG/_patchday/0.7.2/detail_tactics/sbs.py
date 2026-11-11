# -*- coding: utf-8 -*-
r"""sbs — 상세전술 창 하나(또는 전술 하나)의 구/신 디스어셈을 나란히 보여 준다 (0.7.1 대응 조사용 · 읽기 전용).

    python -X utf8 sbs.py <tactic_id | 창 이름 일부 | 구RVA> [-n 40] [--track track.json]

출력(창마다):
  ① track_update 판정 코드·사유·제안(rva/src/team_chk)
  ② 0.6.5 targets 의 원래 의미(asm·operand_meaning·branch taken/not_taken·src·team_chk)
  ③ 구 owner 안 창 ±n 명령 (패닉 Location 주석 · ▶=창 시작)
  ④ 신 후보 ±n 명령 (같은 형식 · 후보가 없으면 신 owner 머리와 track 의 꼴 후보 목록)
비파괴: 파일을 쓰지 않는다.
"""
import argparse, glob, json, os, sys
sys.path.insert(0, r'F:\claude\tfm2mods\MIG')
import aidiff as A
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_MEM, X86_REG_RIP

HERE = os.path.dirname(os.path.abspath(__file__))
TARGETS = r'F:\claude\tfm2mods\tfm2_detail_tactics\tools\targets'

ap = argparse.ArgumentParser()
ap.add_argument('key')
ap.add_argument('-n', type=int, default=40)
ap.add_argument('--track', default=os.path.join(HERE, 'track.json'))
a = ap.parse_args()

t = json.load(open(a.track, encoding='utf-8'))
old = A.Img(t['old_exe']); new = A.Img(t['new_exe'])
lo = A.find_locations(old); ln = A.find_locations(new)
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True

# 0.6.5 targets 원문에서 창 rva → 원래 기록 찾기
orig = {}
for p in glob.glob(os.path.join(TARGETS, '*_0.7.1.json')):
    def walk(o, path):
        if isinstance(o, dict):
            r = o.get('rva') or o.get('key')
            if isinstance(o.get('rva'), str):
                orig.setdefault(o['rva'].lower(), []).append((os.path.basename(p), {k: v for k, v in o.items() if k not in ('sites',)}))
            for k, v in o.items():
                walk(v, path + [k])
        elif isinstance(o, list):
            for v in o:
                walk(v, path)
    walk(json.load(open(p, encoding='utf-8')), [])

def dis(img, loc, b, e, center, n, wlen):
    rows = list(md.disasm(img.code(b, e - b), A.BASE + b))
    idx = next((i for i, x in enumerate(rows) if x.address - A.BASE >= center), None)
    if idx is None:
        return ['  (창이 owner 안에 없음)']
    out = []
    for x in rows[max(0, idx - n): idx + n]:
        r = x.address - A.BASE
        note = ''
        for op in x.operands:
            if op.type == X86_OP_MEM and op.mem.base == X86_REG_RIP:
                tg = x.address + x.size + op.mem.disp - A.BASE
                L = loc.get(tg)
                if L:
                    note = '   ; LOC %s:%d:%d' % (L[0].split('\\')[-1], L[1], L[2])
        mark = '▶' if center <= r < center + wlen else ' '
        out.append('%s %8x  %-26s %s %s%s' % (mark, r, x.bytes.hex(), x.mnemonic, x.op_str, note))
    return out

key = a.key.lower()
ws = [w for w in t['windows'] if key in (w['tactic_id'].lower(), w['name'].lower(), w['old_rva'].lower()) or key in w['name'].lower()]
if not ws:
    sys.exit('창 없음: ' + a.key)
for w in ws:
    d = w.get('detail') or {}
    own = d.get('owner') or {}
    print('=' * 110)
    print('[%d] %s  창 %s  판정 %s  auto=%s' % (w['tactic'], w['name'], w['old_rva'], w['code'], w.get('auto')))
    print('  owner 구 %s  신 %s  Location 판정 %s  lost=%s born=%s' % (
        own.get('old') and [hex(x) for x in own['old']], own.get('new') and [hex(x) for x in own['new']],
        own.get('loc_verdict'), own.get('lost'), own.get('born')))
    for r in w.get('reasons', []):
        print('  - ' + r[:400])
    if w.get('proposal'):
        p = w['proposal']
        print('  제안: rva=%s src=%s team_chk=%s dyn=%s' % (p.get('rva'), p.get('src'), p.get('team_chk'), p.get('dynamic_hex')))
    if d.get('src_items'):
        print('  src_items: ' + json.dumps(d['src_items'], ensure_ascii=False)[:800])
    for f, o in orig.get(w['old_rva'].lower(), [])[:2]:
        keep = {k: o[k] for k in ('asm', 'role', 'operand_meaning', 'branch', 'src', 'team_chk', 'tf', 'note', 'meaning', 'desc') if k in o}
        print('  원래(%s): %s' % (f, json.dumps(keep, ensure_ascii=False)[:1200]))
    wl = len(bytes.fromhex(w.get('detail', {}).get('new_win_hex', '') or '00')) if False else 1
    print('  --- 구 0.7.1 ---')
    orva = int(w['old_rva'], 16)
    if own.get('old'):
        for s in dis(old, lo, own['old'][0], own['old'][1], orva, a.n, 8):
            print(s)
    print('  --- 신 0.7.2 ---')
    if w.get('new_rva') and own.get('new'):
        for s in dis(new, ln, own['new'][0], own['new'][1], int(w['new_rva'], 16), a.n, 8):
            print(s)
    elif own.get('new'):
        print('  (창 후보 없음 — 신 owner %s · 꼴 후보는 사유 참조)' % [hex(x) for x in own['new']])
    else:
        print('  (신 owner 없음)')
