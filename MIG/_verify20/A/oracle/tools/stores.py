# -*- coding: utf-8 -*-
"""writes 전수 ↔ IR store 대조: self(%0) 파생 포인터에 대한 store/memcpy/memset/call-arg 를 전부 센다."""
import sys, re, json
sys.path.insert(0, r'C:\Users\jungs\AppData\Local\Temp\claude\C--Users-jungs-Desktop-claude-tfm2--claude-worktrees-swap-order-button-style-fe9e04\8c613d5e-d69c-428f-aff5-fe06054da7f0\scratchpad20A')
import irlib

SPEC = json.load(open(r'C:\tfm2mods\MIG\_spec\specs20_v3.json', encoding='utf-8'))['specs']

def run(idx, selfreg='%0', srcfile=None):
    s = SPEC[idx]
    ir = s['ir']
    lines, md = irlib.load(ir['file'])
    a, b = ir['frm'], ir['to']
    body = lines[a-1:b]
    pm = irlib.ptr_map(body)
    ev = []
    for k, ln in enumerate(body):
        no = a + k
        ls = ln.strip()
        dbg = irlib.dbg_of(ln)
        rt = irlib.root(md, dbg) if dbg else None
        rl = '%s:%s' % (rt[2].split('\\')[-1], rt[0]) if rt else '-'
        m = re.match(r'^store (.+?), ptr (%[\w.]+|@[\w.$]+)', ls)
        if m:
            for (r, off) in irlib.resolve(pm, m.group(2)):
                if r == selfreg:
                    ev.append((off, 'store', m.group(1)[:60], no, rl))
            continue
        m = re.match(r'^call void @llvm\.(memcpy|memset)[\w.]*\(ptr (?:[\w ]+ )?(%[\w.]+|@[\w.$]+), (?:ptr (?:[\w ]+ )?(%[\w.]+|@[\w.$]+), )?(?:i8 (-?\d+), )?i64 (\d+)', ls)
        if m:
            for (r, off) in irlib.resolve(pm, m.group(2)):
                if r == selfreg:
                    src = m.group(3) or ('i8 %s' % m.group(4))
                    ev.append((off, m.group(1), 'src=%s len=%s' % (src, m.group(5)), no, rl))
            continue
        m = re.match(r'^(?:%[\w.]+ = )?(?:tail )?(call|invoke) (.*?)@([\w.$]+)\((.*)\)', ls)
        if m:
            callee = m.group(3)
            if callee.startswith('llvm.dbg') or callee.startswith('llvm.lifetime') or callee.startswith('llvm.experimental'):
                continue
            args = m.group(4)
            for am in re.finditer(r'ptr ((?:[\w()]+ )*)(%[\w.]+)', args):
                attrs, v = am.group(1), am.group(2)
                for (r, off) in irlib.resolve(pm, v):
                    if r == selfreg:
                        ro = 'readonly' in attrs
                        ev.append((off, 'call-arg' + ('(ro)' if ro else ''), callee[:70], no, rl))
    ev.sort(key=lambda t: (t[0] if t[0] is not None else 1 << 40, t[3]))
    return ev, s

if __name__ == '__main__':
    idx = int(sys.argv[1])
    ev, s = run(idx)
    print('# specs[%d] %s  self=%%0' % (idx, s['name']))
    for off, kind, what, no, rl in ev:
        o = ('0x%x' % off) if off is not None else '?var'
        print('%-8s %-14s %-70s %s:%d  %s' % (o, kind, what, s['ir']['file'], no, rl))
    print()
    print('# spec mem w rows:')
    for i, m in enumerate(s['mem']):
        if m['dir'] == 'w':
            print('  [%d] %s %s %s  <- %s' % (i, m['base'], m['offset'], m['name'], (m.get('note') or '')[:80]))
