# -*- coding: utf-8 -*-
"""base_battle_action(m02.ll:66196~) 안의 SmallActionPlay 태그 store(+177) 전수 + 생성자 호출 목록"""
import io, re, sys
sys.stdout.reconfigure(encoding='utf-8')
path = r'C:\tfm2mods\_gaibc\m02.ll'
start = int(sys.argv[1]) if len(sys.argv) > 1 else 66196
with io.open(path, encoding='utf-8', errors='replace') as f:
    ls = f.read().split('\n')
end = start
for i in range(start, len(ls)):
    if ls[i].startswith('}'):
        end = i; break
print('define', start, '~', end+1, 'lines', end-start)
alloc = {}
g177 = {}
tags = {}
ctors = {}
for i in range(start-1, end):
    s = ls[i]
    m = re.match(r'\s*(%\d+) = alloca \[(\d+) x i8\]', s)
    if m: alloc[m.group(1)] = int(m.group(2))
    m = re.match(r'\s*(%\d+) = getelementptr inbounds (?:nuw )?i8, ptr (%\d+), i64 177', s)
    if m: g177[m.group(1)] = m.group(2)
    m = re.match(r'\s*store i8 (\d+), ptr (%\d+)', s)
    if m and m.group(2) in g177:
        tags.setdefault(int(m.group(1)), []).append((i+1, g177[m.group(2)], alloc.get(g177[m.group(2)])))
    m = re.search(r'(?:call|invoke) [^@]*@(\S*SmallAction\S*?)\(', s)
    if m:
        nm = m.group(1)
        if 'drop_glue' in nm or 'reserve_internal' in nm or 'push' in nm or 'truncate' in nm or 'Extend' in nm or 'from_iter' in nm: continue
        ctors.setdefault(nm[-75:], []).append(i+1)
for t in sorted(tags):
    print('tag', t, 'x%d' % len(tags[t]), tags[t][:6])
print('--- ctors')
for k, v in ctors.items():
    print(k, len(v), v[:5])
# sret 채움 (memcpy 32B → %0)
for i in range(start-1, end):
    s = ls[i]
    if 'llvm.memcpy' in s and '%0,' in s and 'i64 32' in s:
        print('sret memcpy', i+1, s.strip()[:120])
