"""irq.py — m02.ll 등 원문 .ll 을 통째 로드해 줄 범위·패턴·!dbg 사슬을 조회한다.
usage: python irq.py <file> range A B | grep <regex> [A B] | dbg <메타번호>... | loc <line>
"""
import sys, re, io, functools

@functools.lru_cache(maxsize=None)
def load(path):
    with io.open(path, encoding='utf-8', errors='replace') as f:
        return f.read().split('\n')

@functools.lru_cache(maxsize=None)
def meta(path):
    """!N = 메타데이터 정의 dict"""
    d = {}
    for ln in load(path):
        if ln.startswith('!') and ' = ' in ln:
            k, v = ln.split(' = ', 1)
            d[k] = v
    return d

def chain(path, n):
    """!dbg !n → [(line, file/scope, inlinedAt)] 루트까지"""
    m = meta(path)
    out = []
    cur = '!%s' % n
    while cur in m:
        v = m[cur]
        v = v.replace('distinct ', '', 1)
        if not v.startswith('!DILocation'):
            out.append((cur, v[:120])); break
        line = re.search(r'line: (\d+)', v)
        scope = re.search(r'scope: (!\d+)', v)
        inl = re.search(r'inlinedAt: (!\d+)', v)
        sc = m.get(scope.group(1), '') if scope else ''
        # scope → subprogram name/file
        name = ''
        s = scope.group(1) if scope else None
        depth = 0
        while s and s in m and depth < 6:
            sv = m[s]
            nm = re.search(r'name: "([^"]*)"', sv)
            fl = re.search(r'file: (!\d+)', sv)
            if sv.startswith('!DISubprogram'):
                name = (nm.group(1) if nm else '?')
                if fl:
                    fv = m.get(fl.group(1), '')
                    fn = re.search(r'filename: "([^"]*)"', fv)
                    name += ' @' + (fn.group(1) if fn else '?')
                break
            sp = re.search(r'scope: (!\d+)', sv)
            s = sp.group(1) if sp else None
            depth += 1
        out.append((cur, int(line.group(1)) if line else None, name))
        cur = inl.group(1) if inl else None
    return out

def main():
    path = sys.argv[1]; cmd = sys.argv[2]
    L = load(path)
    if cmd == 'range':
        a, b = int(sys.argv[3]), int(sys.argv[4])
        w = int(sys.argv[5]) if len(sys.argv) > 5 else 220
        for i in range(a, b + 1):
            print('%d| %s' % (i, L[i - 1][:w]))
    elif cmd == 'grep':
        rx = re.compile(sys.argv[3])
        a = int(sys.argv[4]) if len(sys.argv) > 4 else 1
        b = int(sys.argv[5]) if len(sys.argv) > 5 else len(L)
        w = int(sys.argv[6]) if len(sys.argv) > 6 else 200
        for i in range(a, b + 1):
            if rx.search(L[i - 1]):
                print('%d| %s' % (i, L[i - 1][:w]))
    elif cmd == 'dbg':
        for n in sys.argv[3:]:
            print(n, chain(path, n.lstrip('!')))
    elif cmd == 'loc':
        # 줄 안의 !dbg !N 을 찾아 사슬 출력
        for ls in sys.argv[3:]:
            i = int(ls)
            m = re.search(r'!dbg !(\d+)', L[i - 1])
            print(i, L[i - 1][:160])
            if m:
                print('   ', chain(path, m.group(1)))
    elif cmd == 'define':
        rx = re.compile(sys.argv[3])
        for i, ln in enumerate(L, 1):
            if ln.startswith('define') and rx.search(ln):
                print('%d| %s' % (i, ln[:400]))
                # find end
                for j in range(i, min(i + 200000, len(L))):
                    if L[j - 1] == '}':
                        print('   end %d (len %d)' % (j, j - i)); break

if __name__ == '__main__':
    main()
