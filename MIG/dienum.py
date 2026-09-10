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
RE_ANYNAME = re.compile(r'^(![0-9]+) = !D\w+\([^\n]*?name: "([^"]*)"', re.M)
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
        # 구조체가 아닌 타입(기본형·포인터 등)의 이름도 필요하다 — 페이로드 필드 타입 표시용
        names_of = dict(RE_ANYNAME.findall(text))

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
            variants, default, payload = {}, None, {}
            for vid in vids:
                tag, vname, psize, poff, ptype, pbt = None, None, 0, None, None, None
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
                        # ★페이로드: 그 variant 구조체 이름과 **enum 안에서의 바이트 오프셋**.
                        #   4차에서 3명이 독립적으로 요청했고, `MainObjective` 를
                        #   "byte1=tag" 로 오독한 사고의 직접 원인이었다.
                        ptype, poff, pbt = vname, _off // 8, bt
                if vname is None:
                    vname = structs.get(vid, '?')
                if tag is None:
                    default = vname
                else:
                    variants[str(tag)] = vname
                    if ptype:
                        # ★★5차 수정 — 조용한 오답 2종을 동시에 없앤다.
                        #  ①이전에는 payload 를 **이름**으로만 저장하고 조회 시
                        #    distruct.json 에서 베어 이름(`"Ult"` `"Serpen"`)으로 찾았다.
                        #    다른 열거형의 동명 variant / 동명 엔티티가 덮어써서
                        #    3B 열거형에 470B 구조체가 붙는 사고가 났다.
                        #  ②페이로드 필드 오프셋에 **래퍼 `__0` 자신의 오프셋을 안 더했다**.
                        #    8B 판별자를 쓰는 열거형에서 전 필드가 8바이트씩 밀렸다.
                        #    (1B 태그 열거형은 우연히 맞아서 더 위험했다.)
                        #  ⟹ 이름 조회를 버리고 **같은 DWARF 패스에서 직접 해석**하고,
                        #    래퍼 오프셋을 누적한 절대값으로 저장한다.
                        def expand(bt0, base, depth=0):
                            """variant 구조체의 멤버를 절대 오프셋으로 편다.

                            ⚠튜플 variant 는 `__0` 래퍼 한 겹이 더 있다 — 거기서 멈추면
                              `enum+0x8 __0 : Champion` 만 보이고 정작 필요한
                              `skill_cooldown` 이 안 나온다(5차 담당자가 원한 것).
                              한 겹짜리 래퍼면 **뚫고 들어간다.**
                            """
                            res = []
                            for e2 in tups.get(elems_of.get(bt0, ''), []):
                                m2 = mems.get(e2)
                                if not m2 or m2[2]:
                                    continue
                                n2, _e2, _s2, o2, b2 = m2
                                res.append((n2, base + o2 // 8, b2))
                            if depth < 2 and len(res) == 1 and res[0][0] in ('__0', 'value'):
                                inner = expand(res[0][2], res[0][1], depth + 1)
                                if inner:
                                    return inner
                            return res

                        flds = [dict(name=n3, off=o3,
                                     type=structs.get(b3) or names_of.get(b3, '?'))
                                for n3, o3, b3 in expand(pbt, poff)]
                        payload[str(tag)] = dict(type=ptype, off=poff, fields=flds)
            if variants:
                prev = out.get(ename)
                if prev is None or len(variants) > len(prev.get('variants', {})):
                    d = dict(kind='rust_enum', variants=variants)
                    if payload:
                        # ★담당자들이 손으로 하던 크기 교차검증을 도구가 한다.
                        #   "3B 열거형에 288B 필드가 있을 리 없다" 를 기계가 알아채게.
                        esz = size_of.get(scope, 0) // 8
                        for t2, pl in payload.items():
                            over = [f for f in pl['fields'] if esz and f['off'] >= esz]
                            if over:
                                pl['suspect'] = esz
                        d['payload'] = payload
                        d['size'] = esz
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

    def leafname0(k):
        """표시용 — **바깥 타입명**(대소문자 보존). `A<B<..C> >` 는 `A<…>` 로."""
        s = k.split('<')[0].rstrip(':').split('::')[-1]
        return (s + '<…>') if '<' in k else s

    # ⚠5차 지적: 부분일치 목록에 `LineType> `, `LineType> > ` 같은 **제네릭 문자열 조각**이
    #   별개 타입인 양 나열돼 담당자가 "내가 잘못된 타입을 보고 있나" 하고 되짚었다.
    #   닫는 꺾쇠가 남은 이름은 파싱 잔해이므로 후보에서 제외한다.
    #   ⚠★6차 정정: **위 진단이 틀렸다.** `LineType> ` 는 파싱 잔해가 아니라
    #     `ControlFlow<...ControlFlow<..LineType,Infallible> >,tuple$<> >` 라는 **정상 타입**을
    #     `k.split('::')[-1][:28]` 로 잘라 보여준 **표시 버그**였다. 5차 수정이 엉뚱한 데를
    #     때렸고(그래서 안 고쳐졌고), 이름 필터로 지웠다면 멀쩡한 항목을 날렸을 것이다.
    #     ⟹ 사전은 건드리지 않는다. 아래 부분일치 **출력** 쪽에서 바깥 타입명으로 보여준다.
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
        # ⚠5차 지적(3명): 질의 이름이 **구조체**면 `Option<X>` 래퍼만 보여주거나 "없음" 이라
        #   나와서, 작성자가 "이름 없는 variant 를 가진 열거형"으로 오독할 위험이 컸다.
        #   distruct 에 있으면 **구조체라고 못 박고 그쪽으로 보낸다.**
        ds0 = os.path.join(HERE, 'distruct.json')
        if os.path.exists(ds0):
            dd0 = json.load(io.open(ds0, encoding='utf-8'))
            st = [k for k in dd0 if k.split('<')[0].split('::')[-1].lower() == w]
            if st:
                print('`%s` 는 **열거형이 아니라 구조체**다 (%dB · 필드 %d).'
                      % (want, dd0[st[0]]['size'], len(dd0[st[0]]['fields'])))
                print('  → `python distruct.py %s` 를 써라. 이 도구(dienum)는 열거형 전용이다.'
                      % want)
                return
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
            t = str(int(q, 0))
            print('  태그 %s → %s' % (q, v['variants'].get(t, '(없음)')))
            pl = (v.get('payload') or {}).get(t)
            if not pl and v.get('kind') == 'enum':
                print('  페이로드 없음 — C형 열거형(값만 있는 태그)이다.')
            if pl:
                # ★★5차 수정: distruct.json 이름 조회를 **버렸다**. 그게 동명 충돌의 원인이었다.
                #   이제 빌드 시점에 같은 DWARF 패스에서 해석해 둔 필드를 그대로 쓴다.
                flds = pl.get('fields')
                if pl.get('suspect'):
                    print('  ⚠페이로드 해석이 의심스럽다 — 필드가 enum 크기(%dB)를 넘는다.'
                          % pl['suspect'])
                    print('    동명 타입 오결합일 수 있으니 **DWARF 로 직접 확인**하라.')
                if flds:
                    # ⚠6차 지적: 오프셋이 DWARF 선언 순서대로 나와 뒤섞여 보였다
                    #   (`+0x48,0x50,0x58,0x60,0x68` 다음에 `+0x38`, `+0x8`, `+0x70`).
                    #   읽는 데 지장은 없었으나 **밀림 오독 위험**이 있어 오프셋순으로 정렬한다.
                    flds = sorted(flds, key=lambda f: f['off'])
                    # ⚠6차 지적(2명): "enum+0x0 부터" 로 읽히는데 실제 첫 필드는 +0x1 이라
                    #   한 칸 밀렸나 되짚었다. 아래 줄의 오프셋이 **enum 선두 기준 절대값**임을 명시.
                    print('  페이로드 %s — 페이로드 시작 enum+%s / 아래는 **enum 선두 기준 절대 오프셋**'
                          % (pl['type'], hex(pl['off'])))
                    for f in flds[:16]:
                        print('    enum+%-6s %-24s %s' % (hex(f['off']), f['name'], f['type']))
                    # 튜플 variant(`__0` 한 겹)는 안쪽 구조체를 따로 봐야 한다.
                    if len(flds) == 1 and flds[0]['name'] in ('__0', 'value'):
                        print('    ⟹ 안쪽은 `python distruct.py %s` 로 보고,'
                              % str(flds[0]['type']).split('<')[0].split('::')[-1])
                        print('       거기 오프셋에 **enum+%s 를 더하라**.' % hex(flds[0]['off']))
                elif flds is not None:
                    print('  페이로드 없음 (fieldless variant) — 태그만 있는 갈래다.')
                else:
                    print('  (이 사전 판본엔 페이로드 정보가 없다 — `--build` 를 다시 돌려라)')
        else:
            print('  %-6s %s' % ('태그', 'variant  (태그 = DWARF DISCR_EXACT 값 그대로)'))
            for t in sorted(v['variants'], key=int):
                print('  %-6s %s' % (t, v['variants'][t]))
            if v.get('default'):
                print('  (기본) %s   ← DISCR_EXACT 없음 = 나머지 전부' % v['default'])
        print()
    # ⚠4차 실측(위험): `Option<ref$<MobaMode>>` 니치판은 0=None 인데, 실제 반환은
    #   16B 태그판이라 tag==0 이 Some 이었다. 이름만 보고 믿으면 **분기가 정반대**가 된다.
    # ⚠6차 지적(노이즈): `dienum LineType`(Option 아님)에도 "같은 이름의 Option 이 19종…"
    #   블록이 딸려 나왔다. 부분일치에 `Option<LineType>` 들이 섞였다는 이유뿐이라
    #   질의와 무관한 경고였다. **정확일치가 답을 다 준 경우엔 띄우지 않는다** —
    #   경고가 상시 뜨면 진짜 위험할 때(4차 사례) 무시하게 된다.
    opts = [k for k in hits + partial if 'option::Option' in k]
    if len(opts) > 1 and (not hits or 'option' in w or
                          any('option::Option' in k for k in hits)):
        print('⚠같은 이름의 Option 이 %d종 있다 — 니치판(0/255=None)과 태그판(0=Some)이' % len(opts))
        print('  섞여 있을 수 있다. **반환 타입의 실제 크기(16B=태그판)를 IR 로 확인하고** 골라라.')
        for k in opts[:4]:
            print('   · %s' % k[:88])
        print()
    if hits and partial:
        # ⚠이터레이터 내부 타입(`ControlFlow`·`Result` 래핑)은 명세 작성과 무관한 잡음이라
        #   따로 세어서 개수만 알린다. 남은 것만 **바깥 타입명**으로 보여준다.
        noise = [k for k in partial if k.startswith(('core::ops::control_flow::',
                                                     'core::iter::', 'core::result::'))]
        real = [k for k in partial if k not in noise]
        if real:
            print('(부분일치 %d건은 생략 — 이름에 포함만 된 다른 타입들: %s)'
                  % (len(real), ', '.join(leafname0(k) for k in real[:4])))
        if noise:
            print('(그 외 %d건은 이터레이터 내부 타입(ControlFlow 등) — 무시해도 된다.)'
                  % len(noise))


if __name__ == '__main__':
    main()
