#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""fieldcodes.py — `<구조체>+<오프셋>` 에 **저장되는 u8 코드값**을 전수로 뽑고 소스 줄을 붙인다.

왜 필요한가:
  이 코드베이스는 진단용 `u8` 코드 필드를 많이 쓴다(`exit_src`·`dive_abort_src`·
  `_mf_src`·`end_reason`…). 열거형이 아니라 **DWARF 에 이름 테이블이 없다**.
  그런데 값 → 의미는 **store 지점의 `!dbg` 소스 줄**로 복원된다(실측: `end_reason` 0~8 전량).

  손으로 하면 `grep i8 <N>` 이 온갖 무관한 상수에 걸린다. 이 도구는
  **gep(오프셋) → 그 결과 레지스터에 store 하는 i8 상수** 만 짝지어 잡는다.

⚠**오프셋만으로 거르면 못 쓴다.** `+0x108` 은 온갖 구조체와 겹쳐 2,339곳이 잡힌다.
  **소스 파일로 걸러라** — `--src battle.rs` 가 사실상 필수다.

사용:
  python fieldcodes.py 0x108 --src battle.rs      # ★권장
  python fieldcodes.py 264                        # 전량(잡음 많음)
"""
import io
import os
import re
import sys
from collections import defaultdict

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

IRDIRS = [r'C:\tfm2mods\_gaibc', r'C:\tfm2mods\_gcbc', r'C:\tfm2mods\_gvbc']
RE_GEP = re.compile(r'^\s*(%\w+) = getelementptr[^\n]*?i64 (\d+)(?:$|,|\s)')
RE_ST = re.compile(r'^\s*store i8 (-?\d+), ptr (%\w+)[^\n]*?(?:!dbg (![0-9]+))?\s*$')
RE_DBG = re.compile(r'^(![0-9]+) = !DILocation\(line: (\d+)[^\n]*?(?:scope: (![0-9]+))?'
                    r'(?:.*?inlinedAt: (![0-9]+))?\)')
# scope 사슬 → 파일. DISubprogram / DILexicalBlock / DILexicalBlockFile 전부 `file:` 를 갖는다.
RE_SCOPE = re.compile(r'^(![0-9]+) = (?:distinct )?!DI(?:Subprogram|LexicalBlock|LexicalBlockFile)'
                      r'\([^\n]*?file: (![0-9]+)')
RE_FILE = re.compile(r'^(![0-9]+) = !DIFile\(filename: "([^"]*)"')


def dbg_root(meta, mid, depth=0):
    """`inlinedAt` 루트까지 타서 진짜 소스 줄을 얻는다(2026-09-10 실측 기법)."""
    seen = set()
    while mid and mid in meta and depth < 12:
        line, scope, inl = meta[mid]
        if not inl or inl in seen:
            return line
        seen.add(mid)
        mid = inl
        depth += 1
    return meta[mid][0] if mid in meta else None


def file_of(scope, sc2file, files, depth=0):
    """scope 메타 → 소스 파일명. 없으면 None."""
    while scope and depth < 8:
        f = sc2file.get(scope)
        if f:
            return files.get(f)
        return None
    return None


def scan(off, only=None, src=None):
    rows = []
    for d in IRDIRS:
        if not os.path.isdir(d):
            continue
        for fn in sorted(os.listdir(d)):
            if not fn.endswith('.ll') or (only and fn != only):
                continue
            text = io.open(os.path.join(d, fn), encoding='utf-8', errors='replace').read()
            lines = text.split('\n')
            meta, sc2file, files = {}, {}, {}
            for ln in lines:
                if not ln.startswith('!'):
                    continue
                if '!DILocation' in ln:
                    m = RE_DBG.match(ln)
                    if m:
                        meta[m.group(1)] = (int(m.group(2)), m.group(3), m.group(4))
                elif '!DIFile(' in ln:
                    m = RE_FILE.match(ln)
                    if m:
                        files[m.group(1)] = m.group(2)
                elif 'DISubprogram' in ln or 'DILexicalBlock' in ln:
                    m = RE_SCOPE.match(ln)
                    if m:
                        sc2file[m.group(1)] = m.group(2)
            # 소스 파일 필터를 쓸 때, 그 파일이 이 모듈에 아예 없으면 건너뛴다(빠른 경로)
            if src and not any(src in v for v in files.values()):
                continue
            # gep 결과 레지스터 -> 그 줄번호
            gep = {}
            for i, ln in enumerate(lines):
                m = RE_GEP.match(ln)
                if m and int(m.group(2)) == off:
                    gep[m.group(1)] = i
            if not gep:
                continue
            for i, ln in enumerate(lines):
                m = RE_ST.match(ln)
                if not m:
                    continue
                val, reg, dbgid = m.group(1), m.group(2), m.group(3)
                if reg not in gep:
                    continue
                srcline = dbg_root(meta, dbgid) if dbgid else None
                # 이 store 의 소스 파일 — scope 사슬로 해석
                fname = None
                if dbgid and dbgid in meta:
                    fname = file_of(meta[dbgid][1], sc2file, files)
                if src and (not fname or src not in fname):
                    continue
                rows.append((os.path.basename(d), fn, i + 1, int(val), srcline, fname))
    return rows


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return
    a = sys.argv[1]
    off = int(a, 16) if a.lower().startswith('0x') else int(a)
    only = src = None
    rest = sys.argv[2:]
    while rest:
        t = rest.pop(0)
        if t == '--src' and rest:
            src = rest.pop(0)
        elif t.endswith('.ll'):
            only = t
    rows = scan(off, only, src)
    if not rows:
        print('오프셋 %s(=%d) 에 store 하는 i8 상수를 못 찾았다.' % (hex(off), off))
        print('  (값이 상수가 아니라 계산 결과이거나, 다른 오프셋일 수 있다.)')
        return
    print('`+%s`(=%d) 에 저장되는 u8 상수 %d곳\n' % (hex(off), off, len(rows)))
    byval = defaultdict(list)
    for dirn, fn, ln, val, srcline, fname in rows:
        byval[val].append((dirn, fn, ln, srcline))
    print('%-6s %-6s %s' % ('값', '곳', '위치 (IR 줄 / 소스 줄)'))
    for val in sorted(byval):
        hits = byval[val]
        srcs = sorted({h[3] for h in hits if h[3]})
        loc = ', '.join('%s:%d' % (h[1], h[2]) for h in hits[:6])
        if len(hits) > 6:
            loc += ' …+%d' % (len(hits) - 6)
        print('%-6d %-6d %s' % (val, len(hits), loc))
        if srcs:
            print('%13s소스줄: %s' % ('', ', '.join(str(s) for s in srcs[:14])))


if __name__ == '__main__':
    main()
