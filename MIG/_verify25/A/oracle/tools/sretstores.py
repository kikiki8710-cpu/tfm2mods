# -*- coding: utf-8 -*-
"""함수 define 안에서 sret %0 (또는 지정 ptr) 파생 gep 에 대한 store/memcpy/memset 전수 → 기록 바이트 범위"""
import io, re, sys
sys.stdout.reconfigure(encoding='utf-8')
path, start = sys.argv[1], int(sys.argv[2])
root = sys.argv[3] if len(sys.argv) > 3 else '%0'
with io.open(path, encoding='utf-8', errors='replace') as f:
    ls = f.read().split('\n')
# 함수 끝 찾기
end = start
for i in range(start, len(ls)):
    if ls[i].startswith('}'):
        end = i; break
regs = {root: 0}
ranges = []
def sz(ty):
    m = {'i8':1,'i16':2,'i32':4,'i64':8,'ptr':8,'i1':1,'double':8,'float':4}
    if ty in m: return m[ty]
    mm = re.match(r'\{ (.*) \}', ty)
    return None
for i in range(start, end+1):
    s = ls[i]
    m = re.match(r'\s*(%\d+) = getelementptr inbounds (?:nuw )?i8, ptr (%\d+), i64 (\d+)', s)
    if m and m.group(2) in regs:
        regs[m.group(1)] = regs[m.group(2)] + int(m.group(3)); continue
    m = re.match(r'\s*store (\S+) ([^,]+), ptr (%\d+)', s)
    if m and m.group(3) in regs:
        off = regs[m.group(3)]; n = sz(m.group(1)) or 0
        ranges.append((off, off+n, i+1, 'store %s %s' % (m.group(1), m.group(2)[:30])))
        continue
    if 'llvm.memcpy' in s or 'llvm.memset' in s:
        mm = re.search(r'\(ptr [^%]*(%\d+), (?:ptr [^%]*(%\d+)|i8 (\d+)), i64 (\d+)', s)
        if mm and mm.group(1) in regs:
            off = regs[mm.group(1)]; n = int(mm.group(4))
            ranges.append((off, off+n, i+1, ('memcpy from %s' % mm.group(2)) if mm.group(2) else ('memset %s' % mm.group(3))))
        continue
    # sret 를 다른 call 로 넘김
    m = re.search(r'(?:call|invoke) [^@]*@(\S+?)\(([^)]*)\)', s)
    if m:
        for r, off in regs.items():
            if re.search(r'ptr [^,]*' + re.escape(r) + r'[,)]', m.group(2)) or re.search(r'ptr [^,]*' + re.escape(r) + r'$', m.group(2)):
                sret = 'sret' in m.group(2).split(r)[0].split(',')[-1]
                ranges.append((off, None, i+1, 'CALL %s%s' % (m.group(1)[-60:], ' (sret)' if sret else '')))
for r in sorted(ranges, key=lambda x: (x[0], x[2])):
    print('%#5x..%s  L%d  %s' % (r[0], ('%#x' % r[1]) if r[1] is not None else '?', r[2], r[3]))
