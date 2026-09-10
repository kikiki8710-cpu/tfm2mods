#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""ghidra_syms.py - rlib 의 DWARF 를 exe RVA 에 이어 **Ghidra 주입용 심볼표**를 만든다.

> ⚠ STALE — 최신 = `dllmatch.py`(2안). 이 파일은 **줄 집합 추출에 결함이 있다**(2026-09-10 확인).
>   `LOC_ALL_RE`/`LOC_OWN_RE` 로 본문의 `!dbg` 를 전부 긁는데, **DILocation 의 scope 가
>   어느 소스 파일인지 보지 않는다.** 그래서 **인라인된 콜리(다른 파일)의 줄이 섞인다**.
>   실측: `position_eval_at_uncached` 의 줄집합이 [1, 8, 9, 12, ...] 로 나왔는데
>   exe 쪽 패닉 줄은 [374, 412, 418, ...] 이라 하나도 안 겹쳤다 —
>   크기·지문·호출관계 3중 근거로 맞다고 확인한 짝인데도 그랬다.
>   이 파일의 조인 기준이 바로 그 줄 집합이므로, **v2 의 커버리지 한계(52.5%)와 오조인의
>   상당 부분이 여기서 왔을 가능성이 크다.** 고친 추출은 `dllmatch.ir_index()` 가 정본
>   (scope -> DISubprogram/DILexicalBlock 의 `file:` 을 따라가 **자기 파일 줄만** 남긴다).

왜 필요한가 (2026-09-10 실사고):
  exe 가 stripped 라 Ghidra 는 전부 `FUN_140xxxxxx` 로 보여준다. 이름이 없으니
  ①어느 함수인지 매번 손으로 찾아야 하고 ②시그니처를 몰라 디컴파일이 틀린다.
  실제로 Ghidra 가 가상호출의 **3번째 인자를 놓쳤고**(2개로 표시), 그대로 구현했다가
  r8 이 쓰레기라 게임이 AV 로 죽었다. 이름/시그니처만 있었으면 안 났을 사고다.

조인 기준 (v2 - v1 의 "시작줄이 구간 안" 은 너무 거칠었다):
  ★**exe 함수의 패닉 줄 집합 ⊆ IR 함수의 '자기 줄' 집합**
  - IR 함수의 '자기 줄' = 본문이 참조하는 DILocation 중 **`inlinedAt` 이 없는 것**.
    (`inlinedAt` 이 있으면 인라인된 콜리의 줄이라 그 함수 것이 아니다.
     v1 은 이걸 구분 못 해 범위가 부풀었고, 그 탓에 한 주소에 심볼 64개가 몰려
     `equivalent_*`(2인자)/`new`(5인자)/`__clone_box_*`(1인자)가 뒤섞이는 오조인이 났다.)
  - 한 RVA 를 여러 IR 함수가 주장하면 **줄 집합이 가장 작은(=가장 구체적인) 쪽**을 쓴다.
    그래도 동률이면 버린다(오염보다 미적용이 낫다).

출력: MIG/ghidra_syms.json  = [{addr, name, proto, src, line, nargs, bytes, mangled}]
검증: `--selftest` — 확인된 사례가 제대로 붙는지. **통과 전에는 주입하지 말 것.**
"""
import argparse
import glob
import io
import json
import os
import re
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

HERE = os.path.dirname(os.path.abspath(__file__))
AIMAP = os.path.join(HERE, 'aimap.json')
OUT = os.path.join(HERE, 'ghidra_syms.json')
IRDIRS = [r'C:\tfm2mods\_gaibc', r'C:\tfm2mods\_gcbc']

SUB_RE = re.compile(r'!DISubprogram\(name: "([^"]+)", linkageName: "([^"]+)"[^)]*?file: (![0-9]+), line: (\d+)')
FILE_RE = re.compile(r'(![0-9]+) = !DIFile\(filename: "([^"]+)"')
DEF_RE = re.compile(r'^define[^@\n]*@([A-Za-z0-9_.$]+)\(([^\n]*?)\)\s*(?:unnamed_addr|#)', re.M)
DBG_RE = re.compile(r'!dbg (![0-9]+)')
# ★inlinedAt 이 **없는** DILocation 만 = 그 함수 자신의 줄
LOC_OWN_RE = re.compile(r'^(![0-9]+) = !DILocation\(line: (\d+)(?:, column: \d+)?, scope: ![0-9]+\)$', re.M)
# 인라인 포함 전체(포함 판정용). exe 함수는 인라인된 콜리의 패닉도 품는다.
LOC_ALL_RE = re.compile(r'^(![0-9]+) = !DILocation\(line: (\d+)[^)]*\)$', re.M)


def split_args(s):
    """괄호 깊이를 세며 최상위 콤마로만 자른다(`range(i64 0, 5)` 같은 것 때문)."""
    out, depth, cur = [], 0, ''
    for ch in s:
        if ch in '([{':
            depth += 1
        elif ch in ')]}':
            depth -= 1
        if ch == ',' and depth == 0:
            out.append(cur.strip())
            cur = ''
        else:
            cur += ch
    if cur.strip():
        out.append(cur.strip())
    return out


def c_type(a):
    a = a.strip()
    if a.startswith('ptr'):
        return 'void *'
    if a.startswith('i64'):
        return 'longlong'
    if a.startswith('i32'):
        return 'int'
    if a.startswith('i16'):
        return 'short'
    if a.startswith('i8') or a.startswith('i1'):
        return 'char'
    if a.startswith('double'):
        return 'double'
    if a.startswith('float'):
        return 'float'
    return 'longlong'


def short_name(dwarf_name, src):
    mod = src.replace(chr(92), '/').split('/')[-1].replace('.rs', '')
    nm = re.sub(r'[^A-Za-z0-9_]', '_', dwarf_name)
    return ('%s__%s' % (mod, nm))[:180]


def build():
    """IR 함수 -> (이름, 소스, 시작줄, 자기줄집합, 인자, 반환)"""
    out = {}
    for f in sorted(sum([glob.glob(os.path.join(p, '*.ll')) for p in IRDIRS], [])):
        t = io.open(f, encoding='utf-8', errors='ignore').read()
        fmap = dict(FILE_RE.findall(t))
        starts = {}
        for m in SUB_RE.finditer(t):
            starts[m.group(2)] = (m.group(1), fmap.get(m.group(3), '?'), int(m.group(4)))
        # ★inlinedAt 없는 DILocation 만 색인 = 각 함수 자신의 줄
        own = {k: int(v) for k, v in LOC_OWN_RE.findall(t)}
        allm = {k: int(v) for k, v in LOC_ALL_RE.findall(t)}
        for m in DEF_RE.finditer(t):
            sym, args = m.group(1), m.group(2)
            if sym not in starts:
                continue
            nm, src, ln = starts[sym]
            j = t.find('\n}\n', m.start())
            body = t[m.start():j if j > 0 else m.start() + 40000]
            refs = DBG_RE.findall(body)
            lines = {own[x] for x in refs if x in own}      # 자기 줄(구체성 순위용)
            alll = {allm[x] for x in refs if x in allm}     # 인라인 포함(포함 판정용)
            lines.discard(0); alll.discard(0)
            lines.add(ln); alll.add(ln)
            head = t[m.start():m.start() + m.group(0).find('@')]
            ret = 'void' if ' void ' in head else ('char' if ' i1 ' in head else 'longlong')
            if 'sret' in args:
                ret = 'void'
            out[sym] = (nm, src, ln, lines, alll, split_args(args), ret)
        del t
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--selftest', action='store_true')
    a = ap.parse_args()

    top = build()
    info = json.load(open(AIMAP)).get('info', {})

    # 모듈별 exe 함수
    by_mod = {}
    for rva, e in info.items():
        if not isinstance(e, dict):
            continue
        mod = str(e.get('mod', ''))
        ls = {l for l in e.get('lines', []) if isinstance(l, int)}
        if not mod or not ls:
            continue
        by_mod.setdefault(mod.replace(chr(92), '/').split('/')[-1], []).append(
            (ls, int(rva, 16), int(e.get('bytes', 0))))

    # ★exe 패닉 줄 ⊆ IR 자기 줄  인 쌍을 모은다
    claims = {}          # rva -> [(줄집합크기, sym)]
    for sym, (nm, src, ln, lines, alll, args, ret) in top.items():
        base = src.replace(chr(92), '/').split('/')[-1].replace('.rs', '')
        for ls, rva, sz in by_mod.get(base, []):
            # ★포함 판정은 **인라인 포함 전체 줄** 로 한다(exe 함수는 인라인된 콜리의 패닉도 품는다).
            #   순위는 **자기 줄이 적은 쪽**(가장 구체적인 함수)을 우선한다.
            if ls <= alll:
                claims.setdefault(rva, []).append((len(lines), len(alll), sym, sz))

    rows, amb = [], 0
    for rva, cand in claims.items():
        cand.sort()
        if len(cand) > 1 and cand[0][:2] == cand[1][:2]:
            amb += 1                     # 동률 - 오염보다 미적용이 낫다
            continue
        _n, _m, sym, sz = cand[0]
        nm, src, ln, lines, alll, args, ret = top[sym]
        ctypes = [c_type(x) for x in args]
        name = short_name(nm, src)
        proto = '%s %s(%s)' % (ret, name,
                               ', '.join('%s a%d' % (c, i) for i, c in enumerate(ctypes)) or 'void')
        rows.append({'addr': '0x%x' % (0x140000000 + rva), 'rva': rva, 'name': name, 'proto': proto,
                     'src': src.replace(chr(92), '/').split('/')[-1].replace('.rs', ''),
                     'line': ln, 'nargs': len(args), 'bytes': sz, 'mangled': sym})
    rows.sort(key=lambda r: r['rva'])

    if a.selftest:
        want = {0xd3fe50: 'nexus_final_stand_uncached',
                0xd3e660: 'nexus_last_stand_uncached',
                0xd405d0: 'base_attacking_minion_uncached',
                0xd57540: 'interaction_score',
                0xd84db0: 'position_eval_at'}
        got = {r['rva']: r for r in rows}
        ok = True
        print('=== 자체 검증 ===')
        for rva, exp in want.items():
            r = got.get(rva)
            if r is None:
                print('  FAIL  0x%x 미매칭 (기대 %s)' % (rva, exp))
                ok = False
            elif exp not in r['name']:
                print('  FAIL  0x%x -> %s  (기대 %s)' % (rva, r['name'], exp))
                ok = False
            else:
                print('  PASS  0x%x -> %-52s %s' % (rva, r['name'], r['proto'][:56]))
        # 한 이름이 여러 주소를 먹지 않는지
        from collections import Counter
        dup = [k for k, v in Counter(r['name'] for r in rows).items() if v > 1]
        print('  이름 중복 %d개%s' % (len(dup), (' 예: ' + ', '.join(dup[:3])) if dup else ''))
        print('  => %s' % ('검증 통과' if ok else '검증 실패 - 주입하지 말 것'))
        print()

    json.dump(rows, open(OUT, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
    print('심볼 %d개 -> %s' % (len(rows), OUT))
    print('aimap RVA %d개 중 %.1f%% 에 이름 (동률이라 버린 주소 %d)'
          % (len(info), 100.0 * len(rows) / max(len(info), 1), amb))


main()
