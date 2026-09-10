# -*- coding: utf-8 -*-
u"""B_plocfull — .ll 에서 패닉 Location 상수를 **전체 경로 + 줄:칸** 으로 뽑는다.
사용: python -X utf8 B_plocfull.py <ll> [경로부분필터] [줄a] [줄b]
"""
import io, os, re, struct, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from B_ploc import unesc

try:
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
except Exception:
    pass

RE_STR = re.compile(r'\[\d+ x i8\] c"((?:[^"\\]|\\..)*)"')
RE_LOC = re.compile(r'ptr (@[A-Za-z0-9_.$]+), \[16 x i8\] c"((?:[^"\\]|\\..)*)"')


def run(path, filt=None, a=None, b=None):
    paths = {}; pend = []
    for ln in io.open(path, encoding='utf-8', errors='ignore'):
        if not ln.startswith('@'):
            continue
        nm = ln.split(' =', 1)[0].strip()
        m2 = RE_LOC.search(ln)
        if m2:
            pend.append((nm, m2.group(1), unesc(m2.group(2))))
            continue
        m = RE_STR.search(ln)
        if m and 'ptr @' not in ln:
            by = unesc(m.group(1)).rstrip(b'\x00')
            if by.endswith(b'.rs'):
                paths[nm] = by.decode('ascii', 'replace')
    rows = []
    for nm, pref, blob in pend:
        if pref not in paths or len(blob) < 16:
            continue
        slen, = struct.unpack_from('<Q', blob, 0)
        line, col = struct.unpack_from('<II', blob, 8)
        f = paths[pref]
        if filt and filt not in f:
            continue
        if a is not None and not (a <= line <= b):
            continue
        rows.append((f, line, col, nm))
    for r in sorted(rows, key=lambda x: (x[0], x[1], x[2])):
        print(u"%-64s %5d:%-4d  %s" % (r[0], r[1], r[2], r[3]))
    print(u"-- %d개" % len(rows))


if __name__ == '__main__':
    a = int(sys.argv[3]) if len(sys.argv) > 4 else None
    b = int(sys.argv[4]) if len(sys.argv) > 4 else None
    run(sys.argv[1], sys.argv[2] if len(sys.argv) > 2 else None, a, b)
