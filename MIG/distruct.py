#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""distruct.py — IR 디버그정보에서 **구조체 오프셋 → 필드 이름** 사전을 한 번에 뽑는다.

왜 이게 필요한가 (1차 명세 배치 실측):
  담당자 전원이 같은 데서 시간을 잃었다 — "이 `+0x8c` 가 무슨 필드인가"를 알려고
  `!N` → 멤버 → 타입 → variant → 멤버 로 **5~6단 메타데이터 사슬을 매번 손으로** 탔다.
  함수당 25~30분 중 **12~15분**이 여기였다. IR 본문 읽기는 5분도 안 걸렸다.
  ⟹ 이건 함수마다 새로 할 일이 아니라 **한 번 만들어 두고 조회할 사전**이다.

주의:
  - DI 의 `offset:` 은 **비트** 단위다. 바이트로 나눠 저장한다(여기서 실수가 잦았다).
  - `size:` 도 비트다.
  - 열거형(variant part)은 `DW_TAG_variant_part` 아래에 variant 별 멤버가 달린다.
    태그(판별자) 오프셋과 각 variant 의 페이로드 오프셋을 같이 담는다.

출력 = `distruct.json`  { "구조체이름": {"size": B, "fields": [{off, off_hex, name, type, size}]} }
조회 = `python distruct.py <구조체이름>` 또는 `python distruct.py <구조체이름> 0x8c`
"""
import io
import json
import os
import re
import sys
from collections import defaultdict

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

HERE = os.path.dirname(os.path.abspath(__file__))
IRDIR = r'C:\tfm2mods\_gaibc'
OUT = os.path.join(HERE, 'distruct.json')

RE_COMP = re.compile(
    r'^(![0-9]+) = !DICompositeType\(tag: (DW_TAG_\w+), name: "([^"]*)"[^\n]*?'
    r'size: ([0-9]+)[^\n]*?elements: (![0-9]+)', re.M)
# ⚠필드 순서를 가정하지 마라. 실제로는 `size: 64, align: 64, offset: 64, flags: ...` 라
#   size 와 offset 사이에 align 이 낀다(이걸 놓쳐 1차 빌드가 오프셋을 전부 0으로 냈다).
#   또 offset 이 **아예 없는** 멤버도 정상이다(= 0).
RE_MEMBER = re.compile(
    r'^(![0-9]+) = !DIDerivedType\(tag: DW_TAG_member, name: "([^"]*)"(.*)$', re.M)
RE_KV = re.compile(r'\b(baseType|size|align|offset): (![0-9]+|[0-9]+)')
RE_TUPLE = re.compile(r'^(![0-9]+) = !\{(.*)\}$', re.M)
RE_NAMED = re.compile(r'^(![0-9]+) = !D\w+\([^\n]*?name: "([^"]*)"', re.M)


def build():
    comps = {}      # id -> (tag, name, size_bits, elements_id)
    members = {}    # id -> (name, basetype_id, size_bits, offset_bits)
    tuples = {}     # id -> [ids]
    names = {}      # id -> name (타입 이름 표시용)

    for fn in sorted(os.listdir(IRDIR)):
        if not fn.endswith('.ll'):
            continue
        text = io.open(os.path.join(IRDIR, fn), encoding='utf-8', errors='replace').read()
        for mid, tag, name, size, el in RE_COMP.findall(text):
            key = (fn, mid)
            comps[key] = (tag, name, int(size), (fn, el))
        for mid, name, rest in RE_MEMBER.findall(text):
            kv = dict(RE_KV.findall(rest))
            bt = kv.get('baseType', '!0')
            members[(fn, mid)] = (name, (fn, bt),
                                  int(kv.get('size', 0)), int(kv.get('offset', 0)))
        for mid, body in RE_TUPLE.findall(text):
            tuples[(fn, mid)] = [(fn, x.strip()) for x in body.split(',') if x.strip().startswith('!')]
        for mid, name in RE_NAMED.findall(text):
            names[(fn, mid)] = name

    out = {}
    for key, (tag, name, size, el) in comps.items():
        if tag != 'DW_TAG_structure_type' or not name:
            continue
        els = tuples.get(el, [])
        fields = []
        for e in els:
            m = members.get(e)
            if not m:
                continue
            fname, bt, fsize, foff = m
            if foff % 8 or fsize % 8:
                continue                      # 비트필드는 이 사전의 대상이 아니다
            fields.append(dict(off=foff // 8, off_hex=hex(foff // 8), name=fname,
                               type=names.get(bt, '?'), size=fsize // 8))
        if not fields:
            continue
        fields.sort(key=lambda f: f['off'])
        prev = out.get(name)
        # 같은 이름이 여러 cgu 에 있으면 **필드가 가장 많은 것**을 채택(가장 완전한 판)
        if prev is None or len(fields) > len(prev['fields']):
            out[name] = dict(size=size // 8, fields=fields)
    return out


def load():
    if not os.path.exists(OUT):
        print('사전이 없다. 먼저 `python distruct.py --build` 를 돌려라.')
        sys.exit(1)
    return json.load(io.open(OUT, encoding='utf-8'))


def base_name(t):
    """`ref$<game_core::..::MapDef>` / `MapDef` / `Vec<T>` 에서 조회 가능한 이름 후보를 낸다."""
    t = str(t)
    for m in re.findall(r'[A-Za-z_][A-Za-z0-9_]*', t.replace('ref$', '').replace('ptr$', '')):
        yield m


def resolve_nested(d, sname, q, v, depth=0, path=''):
    """오프셋 q 가 어느 필드인지 **중첩 구조체를 따라 내려가며** 찾는다."""
    pad = '  ' * (depth + 1)
    for f in v['fields']:
        if not (f['off'] <= q < f['off'] + max(f['size'], 1)):
            continue
        here = (path + '.' if path else '') + f['name']
        print('%s%s (%d) → %-28s : %s (%dB)'
              % (pad, hex(q if depth else f['off']), f['off'], here, f['type'][:40], f['size']))
        inner = q - f['off']
        if depth >= 5 or inner == 0 and f['size'] <= 8:
            return
        # 필드 타입 이름으로 사전을 다시 뒤져 한 단계 더 내려간다
        for cand in base_name(f['type']):
            sub = d.get(cand)
            if sub and sub['size'] == f['size'] and cand != sname and sub['fields']:
                print('%s  ↓ %s 안쪽 +%s' % (pad, cand, hex(inner)))
                resolve_nested(d, cand, inner, sub, depth + 1, here)
                return
        if inner:
            print('%s  (더 못 내려감 — %s 내부 +%s)' % (pad, f['type'][:30], hex(inner)))
        return


def main():
    args = sys.argv[1:]
    if not args or args[0] == '--build':
        d = build()
        io.open(OUT, 'w', encoding='utf-8', newline='').write(
            json.dumps(d, ensure_ascii=False, indent=0, sort_keys=True))
        print('구조체 %d개 · 필드 %d개 → %s'
              % (len(d), sum(len(v['fields']) for v in d.values()), OUT))
        big = sorted(d.items(), key=lambda kv: -kv[1]['size'])[:12]
        print()
        print('%-46s %8s %6s' % ('구조체', '크기B', '필드'))
        for n, v in big:
            print('%-46s %8d %6d' % (n[:46], v['size'], len(v['fields'])))
        return

    d = load()
    want = args[0]
    # 정확일치 > 접두일치 > 부분일치 순. (부분일치만 쓰면 `Entity` 를 물었을 때
    #  `Arc<dyn$<..EntityAction>>` 같은 게 먼저 나온다 - 실측 불편.)
    exact = [k for k in d if k.lower() == want.lower()]
    pre = [k for k in d if k.lower().startswith(want.lower()) and k not in exact]
    sub = [k for k in d if want.lower() in k.lower() and k not in exact and k not in pre]
    hits = exact if exact else (sorted(pre) + sorted(sub))
    if not hits:
        print('없음: %s' % want)
        return
    for k in hits[:3]:
        v = d[k]
        print('=== %s (%dB · 필드 %d) ===' % (k, v['size'], len(v['fields'])))
        if len(args) > 1:
            q = int(args[1], 16) if args[1].lower().startswith('0x') else int(args[1])
            # ★중첩을 끝까지 파고든다. 1차 배치에서 담당자들이 가장 많이 막힌 게 바로
            #   `PlayerState+0x930` 처럼 **최상위 멤버 목록에 없는** 오프셋이었다
            #   (실제로는 `PlayerState.info: GamePlayer` 안에 있었다).
            resolve_nested(d, k, q, v)
        else:
            for f in v['fields']:
                print('  %-8s %-30s %-28s %dB'
                      % (f['off_hex'], f['name'], f['type'][:28], f['size']))
        print()


if __name__ == '__main__':
    main()
