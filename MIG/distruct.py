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
# ⚠5차 지적: 배열·튜플처럼 **이름이 없는** 합성 타입이 전부 `?` 로 나와 정보가 통째로 버려진다
#   (`fountains ? (64B)`, `player_champion ? (80B)`, `walls ? (7200B)` — 22필드 중 10개).
#   담당자들이 크기로 역산하느라 매번 손이 갔다. 원소 타입을 합성해 준다.
RE_ARR = re.compile(
    r'^(![0-9]+) = !DICompositeType\(tag: DW_TAG_array_type, baseType: (![0-9]+)'
    r'[^\n]*?size: ([0-9]+)', re.M)
# 이름 없는 파생타입(포인터·참조·const 등) — 배열 원소 타입을 사슬로 되짚기 위해.
RE_DERIV = re.compile(
    r'^(![0-9]+) = !DIDerivedType\(tag: (DW_TAG_\w+)(?![^\n]*\bname: )'
    r'[^\n]*?baseType: (![0-9]+)', re.M)


def build():
    arrays = {}     # 파일 -> [(id, baseType, size_bits)]
    comps = {}      # id -> (tag, name, size_bits, elements_id)
    members = {}    # id -> (name, basetype_id, size_bits, offset_bits)
    tuples = {}     # id -> [ids]
    names = {}      # id -> name (타입 이름 표시용)
    derived = {}    # id -> (tag, baseType)  — 이름 없는 파생타입 사슬

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
        for mid, tg, bt in RE_DERIV.findall(text):
            derived[(fn, mid)] = (tg, bt)
        arrays[fn] = RE_ARR.findall(text)

    # 배열 타입 이름 합성: `array$<T, N>` (T 이름을 못 찾으면 원소 크기라도 보인다)
    # ⚠6차 지적(2명): 원소 타입이 `?` 로 남는 게 많았다(`array$<?>(총 80B)`).
    #   담당자가 `[[Option<&Entity>;5];2]` 를 IR 의 `[5 x ptr]` + 제네릭 인자에서
    #   **손으로 복원**했다. 원인 둘: ① 중첩 배열은 안쪽 배열 이름이 **아직 안 만들어진**
    #   상태에서 조회된다(한 패스뿐) ② 포인터·참조 원소는 `name:` 이 없는 DIDerivedType 이다.
    #   ⟹ 고정점까지 반복하고, 이름 없는 파생타입은 baseType 사슬을 따라간다.
    def typename(fn2, bt, depth=0):
        k = (fn2, bt)
        if k in names:
            return names[k]
        if depth >= 4:
            return '?'
        tag = derived.get(k)
        if tag is not None:
            tg, inner = tag
            if inner and inner != '!0':
                nm = typename(fn2, inner, depth + 1)
                if nm != '?':
                    if tg in ('DW_TAG_pointer_type', 'DW_TAG_reference_type'):
                        return '&' + nm
                    return nm
            if tg in ('DW_TAG_pointer_type', 'DW_TAG_reference_type'):
                return 'ptr'
        return '?'

    for _round in range(4):
        changed = False
        for fn2, lst in arrays.items():
            for mid, bt, sz in lst:
                k2 = (fn2, mid)
                cur = names.get(k2)
                if cur and '?' not in cur:
                    continue
                et = typename(fn2, bt)
                new = 'array$<%s>(총 %dB)' % (et, int(sz) // 8)
                if new != cur:
                    names[k2] = new
                    changed = True
        if not changed:
            break

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


def norm_type(t):
    """타입 문자열을 비교 가능한 형태로. 경로는 잎만, 기본 할당자는 제거, 공백 제거.

    `Vec<usize,alloc::alloc::Global>` 과 `Vec<usize>` 를 같게, `Vec<Box<dyn ..>>` 는 다르게.
    """
    t = str(t).replace(' ', '')
    t = t.replace(',alloc::alloc::Global', '').replace('alloc::alloc::Global', '')
    t = re.sub(r'[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)+',
               lambda m: m.group(0).split('::')[-1], t)
    return t.replace('ref$', '').replace('<>', '').rstrip(',')


def resolve_nested(d, sname, q, v, depth=0, path='', base=0):
    """오프셋 q 가 어느 필드인지 **중첩 구조체를 따라 내려가며** 찾는다.

    ⚠2차 배치 버그보고: 단계마다 상대 오프셋을 찍어 "어느 게 맞는 값인지" 헷갈렸다
      (MobaMode 0x198/0x1a8/0x1b0 이 셋 다 같은 (384) 로 보였다).
      → **누적 절대 오프셋**을 앞에 찍고 상대값은 괄호로 병기한다.
    """
    pad = '  ' * (depth + 1)
    for f in v['fields']:
        if not (f['off'] <= q < f['off'] + max(f['size'], 1)):
            continue
        here = (path + '.' if path else '') + f['name']
        absoff = base + f['off']
        inner = q - f['off']
        # ⚠4차 버그보고: 좌측 오프셋이 **컨테이너 시작**으로 고정 출력돼 질의값과 헷갈렸다
        #   (0x1a8 을 물어도 0x1a0 을 물어도 좌측이 0x198). 게다가 같은 줄의 두 델타가
        #   서로 다른 기준이라 모순처럼 보였다. → 필드 구간을 명시하고, 질의가 어디에
        #   떨어지는지를 마지막 줄에 못 박는다.
        span = '[%s~%s]' % (hex(absoff), hex(absoff + max(f['size'], 1)))
        print('%s%-8s %-30s : %s (%dB) %s'
              % (pad, hex(absoff), here, f['type'], f['size'], span))
        if depth >= 5 or inner == 0 and f['size'] <= 8:
            return
        # 필드 타입 이름으로 사전을 다시 뒤져 한 단계 더 내려간다
        for cand in base_name(f['type']):
            sub = d.get(cand)
            if sub and sub['size'] == f['size'] and cand != sname and sub['fields']:
                print('%s  └ 질의 %s 는 이 %s 안쪽 +%s' % (pad, hex(q + base), cand, hex(inner)))
                resolve_nested(d, cand, inner, sub, depth + 1, here, base + f['off'])
                return
        # ⚠6차 지적: `Vec`/`String` 에서 하강이 멈췄다("더 못 내려감 — Vec<usize,..Global>").
        #   담당자가 `+0x8=ptr, +0x10=len` 을 **사용 패턴으로 역추론**해 `unknown` 에 실었다.
        #   원인은 키 형태 불일치뿐이다 — 사전엔 `Vec<usize>` 로 있는데 필드 타입은
        #   할당자 인자까지 붙은 `Vec<usize,alloc::alloc::Global>` 이라 `d.get` 이 빗나갔다.
        #   ⟹ **잎 이름 + 크기**로 한 번 더 찾는다(크기 일치를 요구하므로 24B alloc 판과
        #     32B bumpalo 판이 섞이지 않는다).
        #   ⚠★단 **크기만 맞으면 아무거나** 집으면 안 된다. 첫 구현은 `Vec<usize>` 자리에
        #     같은 24B 인 `Vec<Box<dyn EntityPassiveRunner>>` 를 집어 필드명은 맞는데
        #     **타입명을 틀리게** 찍었다(모든 `Vec<T>` 레이아웃이 같아 크기로는 구별 불가).
        #     ⟹ 제네릭 인자까지 정규화해 **같은 타입일 때만** 조용히 내려간다.
        leaf = str(f['type']).split('<')[0].split('::')[-1].strip()
        if leaf:
            tgt = norm_type(f['type'])
            cands = [(k2, sub) for k2, sub in d.items()
                     if k2.split('<')[0].split('::')[-1] == leaf
                     and sub['size'] == f['size'] and k2 != sname and sub['fields']]
            same = [c for c in cands if norm_type(c[0]) == tgt]
            if same:
                k2, sub = same[0]
                print('%s  └ 질의 %s 는 이 %s 안쪽 +%s' % (pad, hex(q + base), k2, hex(inner)))
                resolve_nested(d, k2, inner, sub, depth + 1, here, base + f['off'])
                return
            if cands:
                k2, sub = cands[0]
                print('%s  └ 질의 %s: 이 타입의 사전 항목이 없어 **같은 이름·같은 크기의 다른'
                      ' 인스턴스**(%s) 레이아웃으로 내려간다.' % (pad, hex(q + base), k2[:60]))
                print('%s     ⚠필드 **이름·오프셋만** 믿어라. 원소 타입은 이 필드의 것(%s)이다.'
                      % (pad, str(f['type'])[:60]))
                resolve_nested(d, k2, inner, sub, depth + 1, here, base + f['off'])
                return
        if inner:
            print('%s  └ ★질의 %s = 이 필드 시작 +%s (더 못 내려감 — %s)'
                  % (pad, hex(q + base), hex(inner), f['type']))
        else:
            print('%s  └ ★질의 %s = 이 필드의 시작' % (pad, hex(q + base)))
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
    # ⚠5차 실측: `Map` 을 물으면 `core::iter::Map` 이, `Tower` 를 물으면 `vtable_type$` 이
    #   먼저 나온다. 게임 타입을 앞으로 보내고, 후보가 여럿이면 **크기와 함께 전부** 보여줘
    #   담당자가 IR 의 `dereferenceable(N)` 로 고를 수 있게 한다.
    def std(k):
        return k.startswith(('core::', 'alloc::', 'std::')) or 'vtable_type$' in k
    exact.sort(key=std)
    pre = sorted(pre, key=lambda k: (std(k), len(k)))
    sub = sorted(sub, key=lambda k: (std(k), len(k)))
    hits = exact if exact else (pre + sub)
    # ⚠6차 지적(2명): `distruct Chat` 은 32B **구조체**를 주는데 담당 함수가 쓰는 건 24B
    #   `enum2$<..Chat>` **열거형**이었고, `distruct CastingTarget` 은 Debug impl 의
    #   `vtable_type$` 만 뱉었다. 둘 다 "이 도구 소관이 아니다"라고 말해 주지 않아서
    #   담당자가 사전에 없다고 판단할 뻔했다. ⟹ 같은 이름의 열거형이 있으면 넘긴다.
    def enum_hint():
        p = os.path.join(HERE, 'dienum.json')
        if not os.path.exists(p):
            return []
        try:
            de = json.load(io.open(p, encoding='utf-8'))
        except Exception:
            return []
        w = want.lower()
        return [k for k in de if k.split('<')[0].rstrip(':').split('::')[-1].lower() == w][:3]

    eh = enum_hint()
    if eh and (not hits or all(std(k) for k in hits)
               or any(('enum2$' in f['type'] and f['type'].split('<')[0]
                       or '') and want.lower() in f['type'].lower()
                      for k in hits[:1] for f in d[k]['fields'])):
        print('※ 같은 이름의 **열거형**이 있다 — `python dienum.py %s` 로 봐라.' % want)
        for k in eh:
            print('   · %s' % k[:88])
        print()
    if not hits:
        print('없음: %s%s' % (want, '' if eh else '  (열거형이면 `dienum.py`, 함수면 `fnparts.py`)'))
        return
    if all(std(k) for k in hits):
        print('⚠후보가 전부 표준 라이브러리/`vtable_type$` 다 — **게임 타입이 아니다.**')
        print('  이 이름의 게임 구조체는 이 사전에 없다. 열거형(`dienum.py`)인지 먼저 확인하라.')
        print()
    if len(hits) > 1:
        print('⚠동명 후보 %d개 — **IR 의 dereferenceable(N) 로 골라라**:' % len(hits))
        for k in hits[:6]:
            print('   %-58s %6dB · 필드 %d' % (k[:58], d[k]['size'], len(d[k]['fields'])))
        print()
    for k in hits[:3]:
        v = d[k]
        print('=== %s (%dB · 필드 %d) ===' % (k, v['size'], len(v['fields'])))
        if len(args) > 1:
            q = int(args[1], 16) if args[1].lower().startswith('0x') else int(args[1])
            # ★중첩을 끝까지 파고든다. 1차 배치에서 담당자들이 가장 많이 막힌 게 바로
            #   `PlayerState+0x930` 처럼 **최상위 멤버 목록에 없는** 오프셋이었다
            #   (실제로는 `PlayerState.info: GamePlayer` 안에 있었다).
            print('질의 %s:' % hex(q))
            resolve_nested(d, k, q, v)
        else:
            for f in v['fields']:
                # ⚠타입명을 자르지 마라. 4차 담당자 보고: 잘린 타입명 때문에
                #   `OperationData.cache` 의 실제 타입을 알려고 .ll 을 다시 grep 해야 했다.
                print('  %-8s %-30s %s (%dB)'
                      % (f['off_hex'], f['name'], f['type'], f['size']))
        print()


if __name__ == '__main__':
    main()
