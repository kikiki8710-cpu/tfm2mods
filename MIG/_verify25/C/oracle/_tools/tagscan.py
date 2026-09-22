"""tagscan.py — 한 define 범위 안에서 SmallActionPlay 태그 바이트(+177) store 와 생성자 호출을 전수해
그 함수가 만들 수 있는 variant 집합을 얻는다. usage: python tagscan.py <file> <define_line>
"""
import sys, re, io
path, dl = sys.argv[1], int(sys.argv[2])
with io.open(path, encoding='utf-8', errors='replace') as f:
    L = f.read().split('\n')
end = dl
for j in range(dl, min(dl + 200000, len(L))):
    if L[j - 1] == '}':
        end = j; break
print('define', dl, '..', end, L[dl - 1][:140])
gep177 = set()
tags = {}
ctors = {}
for i in range(dl + 1, end):
    ln = L[i - 1]
    m = re.match(r'\s*(%\d+) = getelementptr inbounds(?: nuw)? i8, ptr (%\d+), i64 177\b', ln)
    if m: gep177.add(m.group(1)); continue
    m = re.match(r'\s*store i8 (-?\d+), ptr (%\d+)', ln)
    if m and m.group(2) in gep177:
        tags.setdefault(int(m.group(1)), []).append(i); continue
    m = re.search(r'@(_R[^(]*?(SmallAction[A-Za-z]+)\d+(new[a-z_]*|from_iter_in|push|extend|retain|truncate)[^(]*)\(', ln)
    if m and ('invoke' in ln or 'call' in ln):
        ctors.setdefault(m.group(2) + '::' + m.group(3), []).append(i)
    m = re.search(r'(?:invoke|call) [^@]*@(\S*?(action_candidates|battle_action|attack_summon_action|attack_jungle_action|small_action|discipline)\S*?)\(', ln)
    if m and 'llvm.' not in ln and 'drop' not in ln and 'clone' not in ln:
        ctors.setdefault('callee:' + m.group(1)[-90:], []).append(i)
NAMES = {3: 'RunAway', 4: 'Recall', 5: 'Around', 6: 'AroundHide', 7: 'AroundRegion', 8: 'AroundRunAway', 9: 'Positioning', 11: 'AroundPositionBush', 12: 'AroundBush', 13: 'LaneMinionPosition', 14: 'Trace', 15: 'Attack', 16: 'Skill', 17: 'Skill2', 18: 'Ult', 19: 'Stop'}
for t in sorted(tags):
    print('tag %3d %-20s lines %s' % (t, NAMES.get(t, '?'), tags[t][:12]))
for k in sorted(ctors):
    print('ctor/callee', k, ctors[k][:8])
