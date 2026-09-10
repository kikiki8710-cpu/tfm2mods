# -*- coding: utf-8 -*-
u"""B_ploc — 한 .ll 에서 패닉 Location 상수(파일,줄,★칼럼)를 전부 뽑는다.
사용: python -X utf8 B_ploc.py <ll경로> [파일basename필터] [줄a] [줄b]
"""
import io, os, re, struct, sys

try:
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
except Exception:
    pass


def unesc(s):
    out = bytearray(); i = 0
    while i < len(s):
        if s[i] == '\\' and i + 2 < len(s) and all(c in '0123456789abcdefABCDEF' for c in s[i+1:i+3]):
            out.append(int(s[i+1:i+3], 16)); i += 3
        elif s[i] == '\\' and i + 1 < len(s):
            out.append(ord(s[i+1])); i += 2
        else:
            out.append(ord(s[i])); i += 1
    return bytes(out)


def run(path, filt=None, a=None, b=None):
    paths = {}; pend = []
    for ln in io.open(path, encoding='utf-8', errors='ignore'):
        if not ln.startswith('@'):
            continue
        nm = ln.split(' =', 1)[0].strip()
        m = re.search(r'\[\d+ x i8\] c"((?:[^"\\]|\\..)*)"', ln)
        if m and 'ptr @' not in ln:
            by = unesc(m.group(1)).rstrip(b'\x00')
            if by.endswith(b'.rs'):
                paths[nm] = by.decode('ascii', 'replace')
            continue
        m2 = re.search(r'ptr (@[A-Za-z0-9_.$]+), \[16 x i8\] c"((?:[^"\\]|\\..)*)"', ln)
        if m2:
            pend.append((nm, m2.group(1), unesc(m2.group(2))))
    rows = []
    for nm, pref, blob in pend:
        if pref not in paths or len(blob) < 16:
            continue
        slen, = struct.unpack_from('<Q', blob, 0)
        line, col = struct.unpack_from('<II', blob, 8)
        f = paths[pref]
        base = re.split(r'[\\/]', f)[-1]
        if filt and filt not in base:
            continue
        if a is not None and not (a <= line <= b):
            continue
        rows.append((base, line, col, nm, pref, slen, len(f)))
    for r in sorted(rows, key=lambda x: (x[0], x[1], x[2])):
        flag = "" if r[5] == r[6] else "  ⚠len불일치 %d vs %d" % (r[5], r[6])
        print(u"%-28s %5d:%-4d  %s -> %s%s" % (r[0], r[1], r[2], r[3], r[4], flag))
    print(u"-- %d개" % len(rows))


if __name__ == '__main__':
    p = sys.argv[1]
    filt = sys.argv[2] if len(sys.argv) > 2 else None
    a = int(sys.argv[3]) if len(sys.argv) > 4 else None
    b = int(sys.argv[4]) if len(sys.argv) > 4 else None
    run(p, filt, a, b)
