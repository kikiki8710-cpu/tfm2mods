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
    # ⚠인자 없이 실행하면 **전체 재빌드**(10분+)가 걸리던 것을 막는다.
    #   3차 배치에서 담당자 둘이 이걸로 2분씩 날리고 결국 DWARF 로 우회했다.
    #   빌드는 `--build` 를 명시할 때만.
    if not args:
        print(__doc__)
        if os.path.exists(OUT):
            import json as _j
            _d = _j.load(io.open(OUT, encoding='utf-8'))
            print('현재 사전: 열거형 %d개 (%s)' % (len(_d), OUT))
        else:
            print('⚠사전이 아직 없다 — `python dienum.py --build` 로 만들어라(10분+).')
        return
    if args[0] == '--build':
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
    # ⚠래퍼가 앞서면 안 된다. `SubPlan` 을 물었는데 `Option<enum2$<..SubPlan>>` 이
    #   먼저 나와서 담당자들이 "사전에 없다"고 판단했다(3차 실측).
    #   **마지막 경로 성분이 정확히 일치**하는 것을 최우선으로 한다.
    w = want.lower()

    def leafname(k):
        s = k.split('<')[0].rstrip(':')
        return s.split('::')[-1].lower()

    exact = [k for k in d if k.lower() == w]
    leafhit = [k for k in d if leafname(k) == w and k not in exact]
    # Option/Result 같은 래퍼는 뒤로 민다
    wrap = lambda k: k.startswith(('core::', 'alloc::', 'std::'))
    leafhit.sort(key=lambda k: (wrap(k), len(k)))
    rest = sorted((k for k in d if w in k.lower() and k not in exact and k not in leafhit),
                  key=lambda k: (wrap(k), len(k)))
    hits = exact + leafhit
    # ⚠부분일치를 같은 화면에 섞으면 `TeamType 0 → Player` 와 `Option<..TeamType> 0 → (없음)`
    #   이 나란히 떠서 오독한다(4차에서 3명이 지적). 정확일치가 있으면 부분일치는 목록만.
    partial = rest
    if not hits:
        # ⚠4차 담당자 보고: "없음" 이 **진짜 없음인지 이름을 잘못 줬는지 구별이 안 돼서,
        #   그대로 믿었으면 오답을 냈을 수 있다." 근접 후보를 반드시 같이 보여준다.
        import difflib
        leaves = {}
        for k in d:
            leaves.setdefault(leafname(k), k)
        near = difflib.get_close_matches(w, list(leaves), n=6, cutoff=0.5)
        sub = [k for k in d if any(t in k.lower() for t in (w[:6], w[-6:]) if len(t) >= 4)][:6]
        print('없음: %s' % want)
        if near:
            print('  혹시 이것? %s' % ' / '.join(leaves[n].split('::')[-1] for n in near))
        if sub:
            print('  부분일치 후보:')
            for k in sub[:5]:
                print('    %s' % k[:90])
        if not near and not sub:
            print('  (사전 %d개 중 비슷한 이름도 없음 — 정말 열거형이 아닐 수 있다.' % len(d))
            print('   소스 타입이 그냥 u8/usize 면 열거형 사전에 있을 수 없다.)')
        return
    for k in hits[:3]:
        v = d[k]
        # ⚠4차 담당자 제안: 출력만 봐선 "니치 밀림 없음"인지 "우연히 일치"인지 구분이 안 된다.
        #   태그 집합이 0..N-1 과 같은지 **명시**하면 이 배치 최대 함정이 사실상 사라진다.
        tags = sorted(int(x) for x in v['variants'])
        flat = (tags == list(range(len(tags))))
        mark = ('태그 = variant 인덱스 (밀림 없음)' if flat
                else '★니치 밀림 — 태그 ≠ variant 인덱스. 그대로 쓰지 말 것')
        print('=== %s (%s · variant %d) — %s ===' % (k, v['kind'], len(v['variants']), mark))
        if len(args) > 1:
            q = args[1]
            print('  태그 %s → %s' % (q, v['variants'].get(str(int(q, 0)), '(없음)')))
        else:
            print('  %-6s %s' % ('태그', 'variant  (태그 = DWARF DISCR_EXACT 값 그대로)'))
            for t in sorted(v['variants'], key=int):
                print('  %-6s %s' % (t, v['variants'][t]))
            if v.get('default'):
                print('  (기본) %s   ← DISCR_EXACT 없음 = 나머지 전부' % v['default'])
        print()
    # ⚠4차 실측(위험): `Option<ref$<MobaMode>>` 니치판은 0=None 인데, 실제 반환은
    #   16B 태그판이라 tag==0 이 Some 이었다. 이름만 보고 믿으면 **분기가 정반대**가 된다.
    opts = [k for k in hits + partial if 'option::Option' in k]
    if len(opts) > 1:
        print('⚠같은 이름의 Option 이 %d종 있다 — 니치판(0/255=None)과 태그판(0=Some)이' % len(opts))
        print('  섞여 있을 수 있다. **반환 타입의 실제 크기(16B=태그판)를 IR 로 확인하고** 골라라.')
        for k in opts[:4]:
            print('   · %s' % k[:88])
        print()
    if hits and partial:
        print('(부분일치 %d건은 생략 — 이름에 포함만 된 다른 타입들: %s)'
              % (len(partial), ', '.join(k.split('::')[-1][:28] for k in partial[:4])))


if __name__ == '__main__':
    main()
