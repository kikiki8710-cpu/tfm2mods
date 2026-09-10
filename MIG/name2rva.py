#!/usr/bin/env python3
"""name2rva.py — game_ai IR 의 **함수 이름** → 게임 exe **RVA** 를 소스 좌표로 잇는다.

왜: 트윈 잔차를 좁히려면 `fn_bisect`/`seq_trace` 에 함수를 더 붙여야 하는데,
    두 도구 다 exe RVA 를 요구한다. IR 은 이름을, exe 센서스(`MIG/aimap.json`)는
    RVA 와 그 함수 안에서 발화하는 **패닉 소스 줄**을 안다. 줄로 이어 붙인다.

원리: IR 의 `DISubprogram(name, file, line, ...)` 으로 함수의 소스 파일과 시작 줄을 얻고,
      본문의 DILocation 최대 줄로 끝 줄을 추정해 [시작, 끝] 구간을 만든다.
      exe 쪽은 aimap.json 의 `info[rva] = {mod, lines[]}` — 그 함수 안 패닉 줄들.
      같은 파일에서 **패닉 줄이 전부 구간 안에 들어가는** RVA 가 후보다.

사용: python MIG\\name2rva.py <이름조각> [<이름조각> ...]
"""
import glob
import io
import json
import os
import re
import sys

AIMAP = r'C:\tfm2mods\MIG\aimap.json'


def demangle_tail(sym):
    out = []
    for n, w in re.findall(r'(\d+)([A-Za-z_][A-Za-z0-9_]*)', sym):
        n = int(n)
        if 2 <= n <= len(w):
            out.append(w[:n])
    return '::'.join(out)


def main():
    wants = [w.lower() for w in sys.argv[1:]] or ['interaction_score']
    info = json.load(io.open(AIMAP, encoding='utf-8'))['info']

    # exe: mod(소스파일 basename) -> [(rva, lines)]
    by_mod = {}
    for rva, v in info.items():
        by_mod.setdefault(v.get('mod', ''), []).append((rva, v.get('lines', []), v.get('bytes', 0)))

    hits = []
    for f in sorted(glob.glob(r'C:\tfm2mods\_gaibc\m*.ll')):
        t = io.open(f, encoding='utf-8', errors='ignore').read()
        files = {m.group(1): m.group(2) for m in re.finditer(r'^!(\d+) = !DIFile\(filename: "([^"]+)"', t, re.M)}
        md = dict(re.findall(r'^!(\d+) = (.*)$', t, re.M))
        # DISubprogram: id -> (name, file basename, line)
        subs = {}
        for m in re.finditer(r'^!(\d+) = (?:distinct )?!DISubprogram\(([^\n]*)', t, re.M):
            body = m.group(2)
            nm = re.search(r'name: "([^"]+)"', body)
            fi = re.search(r'file: !(\d+)', body)
            ln = re.search(r'line: (\d+)', body)
            if not (nm and fi and ln) or fi.group(1) not in files:
                continue
            subs[m.group(1)] = (nm.group(1), re.split(r'[\\/]', files[fi.group(1)])[-1], int(ln.group(1)))
        # 본문 최대 줄
        cache = {}

        def owner(n, d=0):
            if n in cache:
                return cache[n]
            if n in subs:
                cache[n] = n
                return n
            v = md.get(n, '')
            sm = re.search(r'scope: !(\d+)', v)
            r = owner(sm.group(1), d + 1) if (sm and d < 24) else None
            cache[n] = r
            return r

        span = {}
        for m in re.finditer(r'^!\d+ = !DILocation\(line: (\d+)(?:, column: \d+)?, scope: !(\d+)', t, re.M):
            o = owner(m.group(2))
            if o:
                ln = int(m.group(1))
                a, b = span.get(o, (10 ** 9, 0))
                span[o] = (min(a, ln), max(b, ln))

        for sid, (nm, base, line) in subs.items():
            low = nm.lower()
            if not any(w in low for w in wants):
                continue
            lo, hi = span.get(sid, (line, line))
            lo, hi = min(lo, line), max(hi, line)
            mod = base[:-3] if base.endswith('.rs') else base
            cands = []
            for mk, lst in by_mod.items():
                if re.split(r'[\\/]', mk)[-1] != mod:
                    continue
                for rva, lines, nb in lst:
                    if lines and all(lo <= x <= hi for x in lines):
                        cands.append((rva, nb, lines[:6]))
            hits.append((nm, base, lo, hi, os.path.basename(f), cands))

    for nm, base, lo, hi, mo, cands in hits:
        print('%s  [%s:%d~%d]  %s' % (nm[:70], base, lo, hi, mo))
        if not cands:
            print('    exe 후보 없음 (패닉 없는 함수이거나 인라인됨)')
        for rva, nb, lines in sorted(cands, key=lambda x: -x[1]):
            print('    %s  %6dB  패닉줄 %s' % (rva, nb, lines))


if __name__ == '__main__':
    main()
