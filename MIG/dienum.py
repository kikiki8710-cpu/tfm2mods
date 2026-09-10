#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""dienum.py — 열거형의 **태그값 ↔ variant 이름** 사전.

왜 필요한가 (1·2차 배치 실측):
  이 작업 **최대의 함정**이 "태그값 = variant 인덱스"라는 착각이다. 니치 최적화 때문에
  판별자가 밀린다 — `SubPlan` 은 `DISCR = variantIndex + 2`, `BigPlan` 도 `DeathMatchBattle`
  이 니치로 0..1 을 먹어 밀렸다. 1차에서 3명이 독립적으로 밟았고, 2차에서도 담당자들이
  매번 손으로 `DISCR_EXACT` 를 전 variant 훑고 있다. **그건 사전이면 된다.**

DWARF 구조:
  enum2$<X>  (DW_TAG_structure_type)
    └ elements: [Variant0, Variant1, ...]        각각 DW_TAG_structure_type
         ├ DISCR_EXACT (DIFlagStaticMember, extraData: i64 N)   ← ★진짜 태그값
         └ <실제 variant 이름> (DW_TAG_member)                   ← 페이로드
  ⚠DISCR_EXACT 가 **없는** variant = 나머지 전부를 받는 기본 갈래(니치의 '채워진' 쪽).

사용:
  python dienum.py --build
  python dienum.py SubPlan          # 태그 전표
  python dienum.py SubPlan 5        # 태그 5 가 무엇인지
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
OUT = os.path.join(HERE, 'dienum.json')

# ⚠선택 그룹 + 게으른 매칭을 섞으면 scope/size/elements 가 전부 빈 채로도 매칭된다
#   (1차 빌드가 대수형 열거형을 0개로 낸 원인). 줄 전체를 잡고 키:값으로 뽑는다.
# ⚠★`enum2$<X>` 래퍼는 **DW_TAG_union_type** 이다(structure_type 아님).
#   VariantN 구조체의 scope 가 이 union 을 가리키는데, structure_type 만 잡으면
#   부모를 영영 못 찾아 대수형 열거형이 0개로 나온다(1~3차 빌드가 전부 이걸로 실패).
RE_STRUCT = re.compile(
    r'^(![0-9]+) = !DICompositeType\(tag: DW_TAG_(?:structure|union)_type, name: "([^"]*)"(.*)$',
    re.M)
RE_SKV = re.compile(r'(scope|size|elements): (![0-9]+|[0-9]+)')
RE_MEM = re.compile(r'^(![0-9]+) = !DIDerivedType\(tag: DW_TAG_member, name: "([^"]*)"(.*)$', re.M)
RE_TUP = re.compile(r'^(![0-9]+) = !\{(.*)\}$', re.M)
RE_ENUMT = re.compile(
    r'^(![0-9]+) = !DICompositeType\(tag: DW_TAG_enumeration_type, name: "([^"]*)"'
    r'[^\n]*?elements: (![0-9]+)', re.M)
RE_ENUMR = re.compile(r'^(![0-9]+) = !DIEnumerator\(name: "([^"]*)", value: (-?[0-9]+)', re.M)


def build():
    out = {}
    for fn in sorted(os.listdir(IRDIR)):
        if not fn.endswith('.ll'):
            continue
        text = io.open(os.path.join(IRDIR, fn), encoding='utf-8', errors='replace').read()
        structs, scope_of, elems_of, size_of = {}, {}, {}, {}
        for mid, name, rest in RE_STRUCT.findall(text):
            kv = dict(RE_SKV.findall(rest))
            structs[mid] = name
            scope_of[mid] = kv.get('scope', '')
            elems_of[mid] = kv.get('elements', '')
            size_of[mid] = int(kv.get('size', 0))
        mems = {}
        for mid, name, rest in RE_MEM.findall(text):
            ex = re.search(r'extraData: i64 (-?[0-9]+)', rest)
            static = 'DIFlagStaticMember' in rest
            off = re.search(r'offset: ([0-9]+)', rest)
            bt = re.search(r'baseType: (![0-9]+)', rest)
            mems[mid] = (name, int(ex.group(1)) if ex else None, static,
                         int(off.group(1)) if off else 0,
                         bt.group(1) if bt else '')
        tups = {}
        for mid, body in RE_TUP.findall(text):
            tups[mid] = [x.strip() for x in body.split(',') if x.strip().startswith('!')]

        # 1) 순수 C 형 열거형(DW_TAG_enumeration_type) — 이름:값 직결
        for mid, name, el in RE_ENUMT.findall(text):
            if name in ('VariantNames',):
                continue
            vals = {}
            for e in tups.get(el, []):
                m = re.search(r'^%s = !DIEnumerator\(name: "([^"]*)", value: (-?[0-9]+)' % re.escape(e),
                              text, re.M)
                if m:
                    vals[int(m.group(2))] = m.group(1)
            if vals and (name not in out or len(vals) > len(out[name]['variants'])):
                out[name] = dict(kind='enum', variants={str(k): v for k, v in vals.items()})

        # 2) Rust 대수적 열거형 — VariantN 구조체의 DISCR_EXACT 가 진짜 태그
        byscope = defaultdict(list)
        for mid, name in structs.items():
            if name.startswith('Variant') and scope_of.get(mid):
                byscope[scope_of[mid]].append(mid)
        for scope, vids in byscope.items():
            ename = structs.get(scope)
            if not ename:
                continue
            ename = re.sub(r'^enum2\$<|>$', '', ename)
            variants, default = {}, None
            for vid in vids:
                tag, vname, psize = None, None, 0
                for e in tups.get(elems_of.get(vid, ''), []):
                    m = mems.get(e)
                    if not m:
                        continue
                    nm, ex, static, _off, bt = m
                    if nm == 'DISCR_EXACT' and static:
                        tag = ex
                    elif not static and vname is None:
                        # ⚠멤버 이름은 대개 `value` 라 쓸모없다. **baseType 이 가리키는
                        #   구조체의 이름**이 진짜 variant 이름이다(None/Tower/Champion…).
                        vname = structs.get(bt) or nm
                if vname is None:
                    vname = structs.get(vid, '?')
                if tag is None:
                    default = vname
                else:
                    variants[str(tag)] = vname
            if variants:
                prev = out.get(ename)
                if prev is None or len(variants) > len(prev.get('variants', {})):
                    d = dict(kind='rust_enum', variants=variants)
                    if default:
                        d['default'] = default        # DISCR_EXACT 없는 갈래(나머지 전부)
                    out[ename] = d
    return out


def main():
    args = sys.argv[1:]
    if not args or args[0] == '--build':
        d = build()
        io.open(OUT, 'w', encoding='utf-8', newline='').write(
            json.dumps(d, ensure_ascii=False, indent=0, sort_keys=True))
        rust = sum(1 for v in d.values() if v['kind'] == 'rust_enum')
        print('열거형 %d개 (Rust 대수형 %d · C형 %d) → %s'
              % (len(d), rust, len(d) - rust, OUT))
        # 니치 밀림이 실제로 일어난 것들 = 태그 != 인덱스
        skew = [k for k, v in d.items() if v['kind'] == 'rust_enum'
                and sorted(int(x) for x in v['variants']) != list(range(len(v['variants'])))]
        print('★태그가 0부터 연속이 아닌(=니치 밀림) 열거형 %d개 — 손으로 읽으면 틀리는 것들'
              % len(skew))
        for k in sorted(skew)[:10]:
            v = d[k]['variants']
            print('   %-28s 태그 %s' % (k[:28], ' '.join(sorted(v, key=int))[:60]))
        return
    if not os.path.exists(OUT):
        print('사전이 없다. 먼저 `python dienum.py --build`.')
        return
    d = json.load(io.open(OUT, encoding='utf-8'))
    want = args[0]
    exact = [k for k in d if k.lower() == want.lower()]
    hits = exact if exact else sorted(k for k in d if want.lower() in k.lower())
    if not hits:
        print('없음: %s' % want)
        return
    for k in hits[:3]:
        v = d[k]
        print('=== %s (%s · variant %d) ===' % (k, v['kind'], len(v['variants'])))
        if len(args) > 1:
            q = args[1]
            print('  태그 %s → %s' % (q, v['variants'].get(str(int(q, 0)), '(없음)')))
        else:
            for t in sorted(v['variants'], key=int):
                print('  %4s  %s' % (t, v['variants'][t]))
            if v.get('default'):
                print('  (기본) %s   ← DISCR_EXACT 없음 = 나머지 전부' % v['default'])
        print()


if __name__ == '__main__':
    main()
