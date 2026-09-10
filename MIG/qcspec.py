#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""qcspec.py — 함수 명세(JSON)를 **IR 본문과 기계적으로 대조**해 지어낸 내용을 거른다.

왜 필요한가:
  IR 을 읽고 요약을 쓰는 작업은 반드시 환각이 섞인다. 사람이 다 검토하면 병렬화의
  이득이 사라지므로 **검사를 기계에 맡긴다.** 여기서 잡는 것은 "틀린 사실"이다
  (얕은 이해는 못 잡는다 - 그건 사람 몫).

검사 항목:
  C1 상수   - 명세가 적은 상수 값이 본문에 **실제로** 있는가
  C2 호출   - 명세가 적은 피호출 함수가 본문의 `call` 에 **실제로** 있는가
  C3 오프셋 - 명세가 적은 구조체 오프셋이 본문의 gep/주소계산에 있는가
  C4 범위   - ir_from/ir_to 가 진짜 그 함수의 본체인가(define 줄 + 심볼 일치)
  C5 정직   - `unknown` 필드가 있는가(빈 배열도 허용하되 **필드 자체가 없으면 반려**)

사용: python qcspec.py <명세.json 또는 디렉토리>
"""
import io
import json
import os
import re
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

IRDIR = r'C:\tfm2mods\_gaibc'
_cache = {}


def ir_lines(fn):
    if fn not in _cache:
        _cache[fn] = io.open(os.path.join(IRDIR, fn), encoding='utf-8',
                             errors='replace').read().split('\n')
    return _cache[fn]


def body(spec):
    ls = ir_lines(spec['ir_file'])
    out = ls[spec['ir_from'] - 1: spec['ir_to']]
    # ⚠6차 지적(사각지대): 필터 술어가 `call_mut` 심에 인라인되면 **판정 상수의 3분의 1이
    #   담당 범위 밖**에 있다(defensive_crisis: 30000·max_range·is_ignored_well_enemy·
    #   is_recent_visible 4건). 지침대로 knobs/unknown 에 실었지만 C1/C2 가 손을 못 대
    #   **기계 검증 없이** 남았다. ⟹ 보조 범위를 선언하면 같이 검사한다.
    #   `"aux": [{"ir_file":"m10.ll","ir_from":55868,"ir_to":55990}, ...]`
    for a in (spec.get('aux') or []):
        try:
            out += ir_lines(a['ir_file'])[a['ir_from'] - 1: a['ir_to']]
        except Exception:
            pass
    return out


def leaf(sym):
    """망글 심볼/경로에서 마지막 이름 성분. 비교를 이름 단위로 하려고.
    ⚠제네릭은 **베이스 이름만** 남긴다 - `drop_glue<Vec<Chat>>` 를 통째로 뭉개면
      `drop_glueVecChat` 같은 존재하지 않는 이름이 나와 오탐 반려가 된다(1차 배치 실측)."""
    s = sym.strip('"@')
    # ⚠제네릭 인자 그룹을 **먼저 통째로 제거**한다. `split('<')[0]` 로 자르면
    #   `Vec::<T>::from_iter_in` 이 `Vec::` -> 마지막 `::` 성분이 **빈 문자열**이 되고,
    #   빈 이름은 검사에서 조용히 건너뛰어진다 = 검증 없이 통과(1차 배치 실측).
    #   반려 버그보다 나쁘다 - 틀린 걸 통과시키니까.
    for _ in range(8):
        s2 = re.sub(r'<[^<>]*>', '', s)
        if s2 == s:
            break
        s = s2
    s = s.replace('<', '').replace('>', '')
    if '::' in s:
        parts = [p for p in s.split('::') if p.strip()]
        s = parts[-1] if parts else ''
    return re.sub(r'[^A-Za-z0-9_]', '', s)


def mangled_parts(sym):
    """Rust v0 망글의 <길이><이름> 성분을 **순차 파싱**으로 뽑는다.

    ⚠정규식 findall 로는 안 된다 - `Cs97f5S1uJLkH_9game_core` 처럼 해시 안에 숫자가
      섞이면 greedy 매칭이 뒤 성분을 통째로 삼켜 `game_core` 를 영영 못 본다(실측).
    """
    out = set()
    i, n = 0, len(sym)
    while i < n:
        if sym[i].isdigit() and (i == 0 or not sym[i - 1].isdigit()):
            j = i
            while j < n and sym[j].isdigit():
                j += 1
            ln = int(sym[i:j])
            if 0 < ln <= n - j:
                cand = sym[j:j + ln]
                if re.fullmatch(r'[A-Za-z_][A-Za-z0-9_]*', cand):
                    out.add(cand)
            i = j
        else:
            i += 1
    return out


def check(spec):
    """(치명 오류 목록, 경고 목록) 반환."""
    err, warn = [], []
    b = body(spec)
    text = '\n'.join(b)
    # C4 는 **주 범위만** 본다(보조 범위를 붙인 b 로 검사하면 끝줄이 어긋난다).
    prim = ir_lines(spec['ir_file'])[spec['ir_from'] - 1: spec['ir_to']]

    # C4 범위
    if not prim or not prim[0].startswith('define'):
        err.append('C4 범위: ir_from 줄이 define 이 아님 (%r)' % (prim[0][:60] if prim else ''))
    elif spec.get('sym') and spec['sym'] not in prim[0]:
        err.append('C4 범위: define 줄에 sym 이 없음')
    if prim and prim[-1].strip() != '}':
        err.append('C4 범위: ir_to 줄이 본체 끝(}) 이 아님')
    for a in (spec.get('aux') or []):
        try:
            ax = ir_lines(a['ir_file'])[a['ir_from'] - 1: a['ir_to']]
        except Exception:
            err.append('C4 범위: aux 파일을 못 읽음 (%r)' % a.get('ir_file'))
            continue
        if not ax or not ax[0].startswith('define') or ax[-1].strip() != '}':
            err.append('C4 범위: aux %s %s~%s 가 define~} 가 아님'
                       % (a.get('ir_file'), a.get('ir_from'), a.get('ir_to')))

    # C5 정직
    if 'unknown' not in spec:
        err.append('C5 정직: `unknown` 필드 없음 - 모르는 것을 안 적었으면 반려')

    # C1 상수
    # ⚠2026-09-10 정정: 종결문자에 괄호류가 빠져 있어 `range(i64 .., 206)` 의 206 처럼
    #   **괄호 앞 상수를 못 잡았다**. 그 결과 명세 작성자가 진짜 상수를 "본문에 없다"는
    #   반려를 피하려고 목록에서 빼는 일이 실제로 발생했다(1차 배치 실측).
    #   검사기의 오탐은 사실을 지우게 만든다 — 관대한 쪽으로 고친다.
    ints = set(re.findall(r'(?:^|[\s,(\[{])(-?[0-9]+)(?=[\s,)\]}]|$)', text, re.M))
    for c in spec.get('constants', []):
        v = c.get('value')
        if v is None:
            err.append('C1 상수: value 없는 항목')
            continue
        if str(v) not in ints:
            err.append('C1 상수: %s 가 본문에 없음 (근거 없는 값)' % v)
        # ⚠6차 지적: `tps*2` 가 `shl 1` 로 접혀 있어 `value`=1(시프트량)을 등록해야
        #   C1 을 통과한다. 그러면 **상수 목록만 훑는 사람에겐 "임계가 1"로 읽힌다.**
        #   ⟹ `folded_from` 에 소스 수준 값을 적게 하고, 접힘이 의심되면 요구한다.
        if c.get('folded_from') is not None and not str(c.get('meaning', '')).strip():
            err.append('C1 상수: folded_from 을 적었으면 meaning 에 접힘을 설명하라 (%s)' % v)
        if (c.get('folded_from') is None and str(v) in ('1', '2', '3', '4', '5', '6')
                and re.search(r'\bshl\b[^\n]*\b%s\b' % re.escape(str(v)), text)
                and 'shl' not in str(c.get('meaning', ''))):
            warn.append('C1 상수: %s 가 `shl` 피연산자로 보인다 — 시프트량이면 '
                        '`folded_from`(실제 배수)과 meaning 을 적어라' % v)

    # C2 호출
    # ⚠2026-09-10 정정 2건(1차 배치 실측 - 둘 다 작성자에게 **사실 왜곡**을 강요했다):
    #   ①`invoke`(언와인딩 경로 호출)를 안 잡아서, 본문에 실존하는 피호출을 calls 에서
    #     빼야 통과했다. LLVM 은 unwind 가 있는 호출을 invoke 로 낸다 - 똑같은 호출이다.
    #   ②짧은 이름(`can_ult` 7자 · `check` 5자)이 "8자 이상만 부분일치" 규칙에 걸려,
    #     읽기 좋은 `Entity::can_ult` 대신 **망글 심볼 전체**를 적어야 통과했다.
    #     → 망글의 <길이접두><이름> 성분을 미리 풀어 called 에 넣어 길이 제한을 없앤다.
    called = set()
    for m in re.finditer(r'\b(?:call|invoke)\b[^\n]*?@("[^"]+"|[\w.$]+)\(', text):
        sym = m.group(1).strip('"')
        called.add(leaf(sym))
        called |= mangled_parts(sym)
    calledtxt = '\n'.join(sorted(called))
    for c in spec.get('calls', []):
        nm = leaf(c if isinstance(c, str) else c.get('name', ''))
        if not nm:
            continue
        if nm in called:
            continue
        if len(nm) >= 8 and nm in calledtxt:
            continue
        err.append('C2 호출: %s 를 본문에서 못 찾음' % nm)

    # C3 오프셋 (gep 의 바이트 오프셋은 10진수로 나온다)
    # ⚠`writes` 를 신설해 놓고 검사는 `reads` 만 돌아 **사각지대**였다(4차 담당자 보고).
    for r in list(spec.get('reads', [])) + list(spec.get('writes', [])):
        off = r.get('offset')
        if off is None:
            continue
        try:
            n = int(str(off), 16) if str(off).lower().startswith('0x') else int(off)
        except ValueError:
            # ⚠5차 지적: `Vec` 원소 쓰기처럼 **고정 구조체 오프셋이 아닌 쓰기**를 적을 자리가
            #   양식에 없어서, 서술형(`"0x8 -> chats.ptr[len]+0"`)을 넣으면 경고가 났다.
            #   서술형은 정상적인 표현이므로 앞머리의 16진수만 뽑아 검사하고, 없으면 넘어간다.
            m0 = re.match(r'\s*(0[xX][0-9a-fA-F]+|[0-9]+)', str(off))
            if m0:
                off = m0.group(1)
                n = int(off, 16) if off.lower().startswith('0x') else int(off)
            else:
                continue
        if n == 0:
            continue
        if str(n) not in ints:
            warn.append('C3 오프셋: %s(=%d) 가 본문에 없음' % (off, n))

    # 필수 서술 필드
    for f in ('one_line', 'logic'):
        if not str(spec.get(f, '')).strip():
            err.append('필수 필드 비어있음: %s' % f)

    return err, warn


def main():
    target = sys.argv[1] if len(sys.argv) > 1 else '.'
    paths = []
    if os.path.isdir(target):
        for fn in sorted(os.listdir(target)):
            if fn.endswith('.json'):
                paths.append(os.path.join(target, fn))
    else:
        paths = [target]

    npass = nfail = nwarn = 0
    for p in paths:
        try:
            spec = json.load(io.open(p, encoding='utf-8'))
        except Exception as e:
            print('[FAIL] %-34s 파싱 불가: %s' % (os.path.basename(p), e))
            nfail += 1
            continue
        specs = spec if isinstance(spec, list) else [spec]
        for s in specs:
            err, warn = check(s)
            tag = os.path.basename(p) + (':' + s.get('name', '?') if len(specs) > 1 else '')
            if err:
                nfail += 1
                print('[FAIL] %s' % tag)
                for e in err:
                    print('        %s' % e)
            else:
                npass += 1
                print('[PASS] %s%s' % (tag, '  (경고 %d)' % len(warn) if warn else ''))
            for w in warn:
                print('        ⚠ %s' % w)
            nwarn += len(warn)
    print()
    # ⚠경고를 요약에 안 찍으면 사실상 안 보인다(감사에서 드러남).
    #   C3(오프셋)는 gep 접힘 때문에 일부러 경고로 두지만, **몇 건인지는 보여야** 한다.
    print('통과 %d / 반려 %d / 경고 %d건' % (npass, nfail, nwarn))
    return 1 if nfail else 0


if __name__ == '__main__':
    sys.exit(main())
