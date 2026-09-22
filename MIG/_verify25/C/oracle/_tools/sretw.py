"""sretw.py — 한 define 안에서 sret(%0 또는 지정 레지스터)에 대한 store/memcpy/memset/call-sret 을 전수해
오프셋 범위와 소속 블록(조건부 여부)을 찍는다. gep 별칭(%N = gep %0, K)을 전이적으로 따라간다.
usage: python sretw.py <file> <define_line> [reg=%0]
"""
import sys, re, io

def load(path):
    with io.open(path, encoding='utf-8', errors='replace') as f:
        return f.read().split('\n')

def size_of(ty):
    m = re.match(r'i(\d+)$', ty)
    if m: return (int(m.group(1)) + 7) // 8
    if ty == 'ptr': return 8
    if ty in ('double',): return 8
    if ty in ('float',): return 4
    m = re.match(r'\[(\d+) x i8\]', ty)
    if m: return int(m.group(1))
    m = re.match(r'<(\d+) x i(\d+)>', ty)
    if m: return int(m.group(1)) * int(m.group(2)) // 8
    return None

def main():
    path, dl = sys.argv[1], int(sys.argv[2])
    reg = sys.argv[3] if len(sys.argv) > 3 else '%0'
    L = load(path)
    # find end
    end = dl
    for j in range(dl, min(dl + 100000, len(L))):
        if L[j - 1] == '}':
            end = j; break
    print('define', dl, '..', end, L[dl - 1][:160])
    alias = {reg: 0}   # reg -> offset from sret
    blk = 'entry'
    blocks = {}   # name -> preds
    stores = []
    for i in range(dl + 1, end):
        ln = L[i - 1]
        mb = re.match(r'^(\d+):\s*; preds = (.*)$', ln)
        if mb:
            blk = '%' + mb.group(1); blocks[blk] = mb.group(2); continue
        mb = re.match(r'^(\d+):\s*$', ln)
        if mb:
            blk = '%' + mb.group(1); blocks[blk] = ''; continue
        m = re.match(r'\s*(%\d+) = getelementptr inbounds(?: nuw)? i8, ptr (%\d+), i64 (-?\d+)', ln)
        if m and m.group(2) in alias:
            alias[m.group(1)] = alias[m.group(2)] + int(m.group(3)); continue
        m = re.match(r'\s*(%\d+) = getelementptr inbounds(?: nuw)? i8, ptr (%\d+), i64 (%\d+)', ln)
        if m and m.group(2) in alias:
            alias[m.group(1)] = ('var', alias[m.group(2)], m.group(3)); continue
        m = re.match(r'\s*store (\S+) (.+?), ptr (%\d+)', ln)
        if m and m.group(3) in alias:
            off = alias[m.group(3)]; sz = size_of(m.group(1))
            stores.append((i, blk, off, sz, 'store %s %s' % (m.group(1), m.group(2)[:40])))
            continue
        m = re.search(r'call void @llvm\.memcpy\.[^(]*\(ptr[^%]*(%\d+), ptr[^%]*(%\d+), i64 (\d+)', ln)
        if m and m.group(1) in alias:
            stores.append((i, blk, alias[m.group(1)], int(m.group(3)), 'memcpy <- %s' % m.group(2)))
            continue
        m = re.search(r'call void @llvm\.memset\.[^(]*\(ptr[^%]*(%\d+), i8 (\d+), i64 (\d+)', ln)
        if m and m.group(1) in alias:
            stores.append((i, blk, alias[m.group(1)], int(m.group(3)), 'memset %s' % m.group(2)))
            continue
        # call with sret into alias
        m = re.search(r'call [^@]*@(\S+?)\(ptr [^%]*sret\(\[(\d+) x i8\]\)[^%]*(%\d+)', ln)
        if m and m.group(3) in alias:
            stores.append((i, blk, alias[m.group(3)], int(m.group(2)), 'call-sret @%s' % m.group(1)[:80]))
            continue
        # any other call taking alias ptr as arg (possible write)
        for a in re.findall(r'ptr [^,()]*?(%\d+)', ln):
            if a in alias and ' call ' in ln and 'llvm.lifetime' not in ln and 'llvm.dbg' not in ln:
                fn = re.search(r'@(\S+?)\(', ln)
                stores.append((i, blk, alias[a], None, 'call-arg @%s' % (fn.group(1)[:80] if fn else '?')))
                break
    for s in stores:
        print('%6d %-8s off=%-14s sz=%-5s %s' % s)
    print('blocks', len(blocks))

if __name__ == '__main__':
    main()
