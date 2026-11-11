# usage: ndiff.py OLDSTART OLDEND NEWSTART NEWEND  -> normalized instruction diff (addresses/ rbp disp masked)
import sys, re, subprocess, difflib, os
H = os.path.dirname(os.path.abspath(__file__))
def get(which, a, b):
    out = subprocess.run([sys.executable, '-X', 'utf8', os.path.join(H, 'dz.py'), which, a, b], capture_output=True, text=True, encoding='utf-8').stdout
    rows = []
    for line in out.splitlines():
        m = re.match(r'\s+([0-9a-f]+)\s+((?:[0-9a-f]{2} )+)\s*(.*)', line)
        if not m: continue
        ins = m.group(3)
        ins = re.sub(r'0x14[0-9a-f]{7}', 'ADDR', ins)
        ins = re.sub(r'\s+(LOC .*|\[0x[0-9a-f]+\])$', '', ins)
        ins = re.sub(r'rip [+-] 0x[0-9a-f]+', 'rip+X', ins)
        if 'nop' in ins.split(' ')[0]: continue
        rows.append((m.group(1), ins))
    return rows
o = get('old', sys.argv[1], sys.argv[2]); n = get('new', sys.argv[3], sys.argv[4])
mode = sys.argv[5] if len(sys.argv) > 5 else 'diff'
def norm(s):
    s = re.sub(r'(rbp|rsp) \+ 0x[0-9a-f]+', r'\1+D', s)
    return s
sm = difflib.SequenceMatcher(a=[norm(x[1]) for x in o], b=[norm(x[1]) for x in n], autojunk=False)
print('ratio', round(sm.ratio(), 3), 'old', len(o), 'new', len(n))
for tag, i1, i2, j1, j2 in sm.get_opcodes():
    if tag == 'equal': continue
    print(f'--- {tag} old[{i1}:{i2}] new[{j1}:{j2}]')
    for x in o[i1:i2]: print('  -', x[0], x[1])
    for x in n[j1:j2]: print('  +', x[0], x[1])
