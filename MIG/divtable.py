#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""divtable.py — `dyn Trait` 의 **vtable 슬롯 오프셋 → 함수 이름**.

왜 필요한가 (1~3차 명세 배치 실측):
  `unknown` 478건 중 **36건(7.5%)이 "vtable 슬롯 이름을 못 찾음"** 이었다.
  담당자들이 DWARF `vtable_type$` 를 뒤졌는데 거기엔 `__method8` `__method62` 처럼
  **번호만** 있다(game_core 트레이트 impl DWARF 가 `_gaibc` 에 없다).
  ⟹ 대부분 "원리적으로 불가"로 포기했다.

  그런데 한 담당자가 우회로를 찾았다: **vtable 전역 상수의 함수포인터 배열을 순서대로 읽는다.**
  이건 IR 에 그대로 있다. 그 방법을 도구로 만든다.

vtable 전역 레이아웃 (Rust):
  <{ ptr drop_glue, [16 x i8] size+align, ptr method0, ptr method1, ... }>
  ⟹ 바이트 오프셋 0x00=drop / 0x08=size / 0x10=align / 0x18부터 8B 씩 메서드

⚠담당자 경고(실측): 값 목록을 **쉼표 단위로 순서대로** 뽑아야 한다.
  `grep '@_RNvX'` 처럼 심볼 접두로 필터하면 `_RNvY`(shim) 항목이 빠져 **슬롯이 밀린다**.
  실제로 그렇게 0x40 을 엉뚱한 함수로 오독할 뻔한 사례가 있었다.

사용:
  python divtable.py AbstractGame            # 후보 vtable 목록
  python divtable.py AbstractGame 0x28       # 그 슬롯이 무슨 함수인지
"""
import io
import os
import re
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

IRDIR = r'C:\tfm2mods\_gaibc'
# ★2026-09-10: game_core 본문 IR. 1~6차는 "`Arc<dyn Trait>` 은 vtable 전역이 `_gaibc` 에
#   없어 원리적으로 불가"로 8건을 포기했는데 **거짓이었다** — `EffectType` impl 들의 정적
#   vtable 전역이 `_gcbc` 에 있다(`g04.ll:927` AttackEffect, 34슬롯·296B, DWARF 와 일치).
#   그래서 `_gaibc` 에서 못 찾으면 자동으로 여기를 본다.
COREDIR = r'C:\tfm2mods\_gcbc'
# 전역 상수 한 줄 전체를 잡는다(값 목록이 매우 길다).
RE_GLOB = re.compile(r'^(@[\w.]+) = (?:private )?(?:unnamed_addr )?constant <\{([^\n]*)$', re.M)


def mangled_parts(sym):
    out, i, n = [], 0, len(sym)
    while i < n:
        if sym[i].isdigit() and (i == 0 or not sym[i - 1].isdigit()):
            j = i
            while j < n and sym[j].isdigit():
                j += 1
            ln = int(sym[i:j])
            if 0 < ln <= n - j:
                c = sym[j:j + ln]
                # ⚠`NtB4_12AbstractGame` 처럼 백참조(`B4_`) 뒤에서 길이를 잘못 집으면
                #   `_12Abs` 같은 쓰레기 성분이 나온다. 밑줄로 시작하면 버린다.
                if re.fullmatch(r'[A-Za-z][A-Za-z0-9_]*', c):
                    out.append(c)
            i = j
        else:
            i += 1
    return out


def slots_of(valpart):
    """값 목록에서 **순서대로** 항목을 뽑아 (바이트오프셋 -> 심볼) 을 만든다.

    항목은 `ptr @sym` / `[16 x i8] c"..."` / `ptr null` 등. 크기를 누적해 오프셋을 센다.
    """
    out, off = {}, 0
    # ⚠전역 한 줄은 `constant <{타입목록}> <{값목록}>` 이다. **타입 선언부를 같이 세면**
    #   오프셋이 통째로 밀린다(1차 시험에서 최소 오프셋이 0x328 로 나온 원인).
    #   `}> <{` 뒤의 값 목록만 취한다.
    m = re.search(r'\}>\s*<\{(.*)$', valpart)
    if m:
        valpart = m.group(1)
    # `c"..."` 안에 쉼표가 들어갈 수 있으니 문자열 리터럴을 먼저 치환해 보호한다.
    lits = []

    def stash(m):
        lits.append(m.group(0))
        return '\x00LIT%d\x00' % (len(lits) - 1)

    v = re.sub(r'c"(?:[^"\\]|\\.)*"', stash, valpart)
    for item in v.split(','):
        item = item.strip()
        if not item:
            continue
        m = re.match(r'\[(\d+) x i8\]', item)
        if m:
            off += int(m.group(1))
            continue
        if item.startswith('ptr'):
            s = re.search(r'@([\w.$]+)', item)
            if s:
                out[off] = s.group(1)
            off += 8
            continue
        m = re.match(r'i(\d+)\b', item)
        if m:
            off += max(int(m.group(1)) // 8, 1)
            continue
        off += 8          # 모르는 항목도 포인터 크기로 가정(순서를 잃지 않는 게 중요)
    return out


def find(want, irdir=IRDIR):
    res = []
    for fn in sorted(os.listdir(irdir)):
        if not fn.endswith('.ll'):
            continue
        text = io.open(os.path.join(irdir, fn), encoding='utf-8', errors='replace').read()
        for name, val in RE_GLOB.findall(text):
            if want.lower() not in val.lower():
                continue
            sl = slots_of(val)
            if len(sl) < 3:
                continue
            res.append((fn, name, sl))
    return res


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return
    want = sys.argv[1]
    q = None
    if len(sys.argv) > 2:
        a = sys.argv[2]
        q = int(a, 16) if a.lower().startswith('0x') else int(a)

    res = find(want)  # _gaibc 먼저(빠른 경로)
    # ⚠후보가 **하나도 없을 때도** game_core 폴백을 타야 한다. 처음엔 여기서 바로
    #   `없음` 을 찍고 return 해서 `divtable EffectType` 이 폴백에 닿지도 못했다.
    # ⚠4차 실측(조용한 오답): `divtable Entity 0x28` 이 `AbstractGame::tick` 을 자신 있게 뱉었다.
    #   값 문자열에 'Entity' 가 우연히 들어 있었을 뿐 Entity 의 vtable 이 아니다.
    #   **요청 이름이 슬롯 심볼의 이름 성분으로 실제 등장하는지** 확인한다.
    # ⚠★4차 최악의 결함: `divtable Effect` 가 `AbstractGame` vtable 을 **경고 없이** 반환했다.
    #   'Effect' 가 그 vtable 의 어느 메서드 이름에 한 번 등장했다는 이유뿐이었다.
    #   담당자 증언: "0x118 을 그대로 믿었으면 완전히 틀린 이름을 명세에 박을 뻔했다."
    #   ⟹ 진짜 트레이트 vtable 이면 그 이름이 **슬롯 대부분**에 등장한다. 과반을 요구한다.
    if not want.strip():
        print('트레이트 이름을 줘라. (예: AbstractGame / AbstractEntity)')
        return

    def own_ratio(sl):
        if not sl:
            return 0.0
        n = sum(1 for sym in sl.values() if want in mangled_parts(sym))
        return n / float(len(sl))
    good = [r for r in res if own_ratio(r[2]) >= 0.5]
    if not good and os.path.isdir(COREDIR):
        # ★`_gaibc` 에 과반 일치 vtable 이 없으면 game_core(`_gcbc`)를 본다.
        #   `EffectType`·`Action`·`EffectBuff` 처럼 game_core 트레이트의 정적 vtable 전역이
        #   거기 있다. 1~6차가 "원리적 불가"로 포기한 8건이 전부 이 경로로 풀린다.
        print('※ `_gaibc` 에 과반 일치 vtable 이 없어 `_gcbc`(game_core)를 훑는다 — 오래 걸린다.')
        res = find(want, COREDIR)
        good = [r for r in res if own_ratio(r[2]) >= 0.5]
        if good:
            print('★`_gcbc`(game_core) 에서 찾았다 — 줄번호는 `C:\\tfm2mods\\_gcbc\\<파일>` 기준.')
            print()
    if not good and not res:
        print('없음: %s  (두 IR 어디에도 이 이름이 든 vtable 전역이 없다)' % want)
        return
    if not good:
        best = max(((own_ratio(r[2]), r) for r in res), key=lambda x: x[0], default=(0, None))
        print('⚠`%s` 의 vtable 을 못 찾았다 — 가장 근접한 후보도 슬롯의 %.0f%% 에만 등장.'
              % (want, best[0] * 100))
        print('  **슬롯 이름을 그대로 믿지 마라.** 다른 트레이트의 vtable 일 가능성이 크다.')
        print('  (값 문자열에 우연히 포함됐을 뿐일 수 있다. 트레이트 이름으로 다시 쳐라:')
        print('   예 AbstractGame / AbstractEntity / EffectType)')
        print('  ※ 구조체 필드 오프셋을 찾는 거라면 이 도구가 아니라 `distruct.py` 다.')
        return
    res = good
    # 슬롯이 가장 많은 것 = 진짜 트레이트 vtable(작은 건 클로저 vtable)
    res.sort(key=lambda r: -len(r[2]))
    for fn, name, sl in res[:2]:
        # ⚠4차 제안: `dereferenceable(816)` 대조를 하려면 총 바이트가 필요한데
        #   슬롯 개수만 찍어서 담당자가 100*8+16 을 손으로 역산했다.
        total = (max(sl) + 8) if sl else 0
        print('=== %s  (%s · 슬롯 %d개 · vtable 총 %dB · `%s` 일치율 %.0f%%) ==='
              % (name[:44], fn, len(sl), total, want, own_ratio(sl) * 100))
        # ⚠5차 지적: 출력 심볼은 이 vtable 전역을 만든 **특정 impl** 것이다.
        #   슬롯 번호 ↔ 메서드 이름 대응은 트레이트 공통이라 유효하지만,
        #   "런타임에 이 impl 이 호출된다"로 읽으면 오독이다.
        print('    ※슬롯 번호↔메서드 이름만 유효하다. 심볼의 impl 은 이 vtable 전역 기준일 뿐 —')
        print('      런타임에 어느 구현체가 꽂히는지는 이 도구로 알 수 없다.')
        if q is not None:
            sym = sl.get(q)
            if sym:
                p = mangled_parts(sym)
                print('  %s → %s' % (hex(q), '::'.join(p[-3:]) or sym))
                # ⚠앞을 남기고 자르면 `..._13get_game` 처럼 **꼬리(_mode)가 사라져** 오독한다.
                #   4차 실측: 그 탓에 담당자가 슬롯 계산을 2라운드 의심했다. 가운데를 생략한다.
                print('     %s' % (sym if len(sym) <= 120
                                   else sym[:60] + ' … ' + sym[-55:]))
            else:
                near = sorted(sl)
                print('  %s → (그 오프셋에 항목 없음). 가진 오프셋 예: %s'
                      % (hex(q), ' '.join(hex(x) for x in near[:8])))
        else:
            for off in sorted(sl)[:40]:
                p = mangled_parts(sl[off])
                print('  %-7s %s' % (hex(off), '::'.join(p[-3:]) or sl[off][:60]))
        print()


if __name__ == '__main__':
    main()
