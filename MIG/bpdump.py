"""bpdump.py — bpcatalog.json 의 전 함수를 irann 방식(소스줄 주석·잡음 제거)으로 일괄 덤프한다. python bpdump.py <catalog.json> <outdir>

2026-09-15 신설(champ_pos_lock 밴픽 정적 전수조사). 모듈당 메타를 1회만 로드해 수천 함수를 한 번에 낸다.
출력 = <outdir>\<소스파일 경로를 __ 로 이은 이름>\<src_line:05d>_<함수명>.txt + <outdir>\INDEX.md(파일별 함수 목록·IR 줄수).
"""
import io, os, re, sys, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import irann

def safe(s):
    return re.sub(r'[^A-Za-z0-9_.-]+', '_', s).strip('_')[:100]

def main():
    cat = json.load(io.open(sys.argv[1], encoding='utf-8'))
    outdir = sys.argv[2]
    os.makedirs(outdir, exist_ok=True)
    roots = {"_gcbc": r"C:\tfm2mods\_gcbc", "_gvbc": r"C:\tfm2mods\_gvbc", "_gaibc": r"C:\tfm2mods\_gaibc"}
    bymod = {}
    for c in cat:
        bymod.setdefault((c["root"], c["module"]), []).append(c)
    index = {}
    for (root, mod), fns in sorted(bymod.items()):
        path = os.path.join(roots[root], mod)
        meta = irann.load_meta(path)
        lines = io.open(path, encoding='utf-8', errors='ignore').readlines()
        for c in fns:
            s, e = c["ll_start"], c["ll_end"]
            res = []
            for i in range(s, e + 1):
                line = lines[i - 1].rstrip('\n')
                st = line.strip()
                m = re.match(r'#dbg_(?:value|declare)\((.*), !(\d+), !DIExpression\((.*?)\), !(\d+)\)', st)
                if m:
                    val, vid, expr, _ = m.groups()
                    nm = irann.var_name(meta, vid)
                    if nm is None or 'poison' in val:
                        continue
                    frag_ex = ''
                    fm = re.search(r'DW_OP_LLVM_fragment,\s*(\d+),\s*(\d+)', expr)
                    if fm:
                        frag_ex = '[%d..+%d]' % (int(fm.group(1)) // 8, int(fm.group(2)) // 8)
                    res.append('%6d|     ;; %s%s = %s' % (i, nm, frag_ex, val.strip()))
                    continue
                if st.startswith('#dbg_') or st.startswith('call void @llvm.dbg'):
                    continue
                if 'llvm.lifetime' in st or 'llvm.experimental.noalias' in st:
                    continue
                loc = ''
                dm = re.search(r'!dbg !(\d+)', line)
                if dm:
                    loc = irann.loc_chain(meta, dm.group(1))
                    line = re.sub(r',?\s*!dbg !\d+', '', line)
                for rx, rep in irann.NOISE:
                    line = rx.sub(rep, line)
                for rx, rep in irann.SYM:
                    line = rx.sub(rep, line)
                line = re.sub(r'\s+', ' ', line.rstrip())
                if loc:
                    line = '%-118s ;L%s' % (line, loc)
                res.append('%6d| %s' % (i, line))
            d = os.path.join(outdir, safe(c["file"].replace('\\\\', '__').replace('\\', '__')))
            os.makedirs(d, exist_ok=True)
            fn = '%05d_%s.txt' % (c["src_line"], safe(c["name"]))
            k = 1
            while os.path.exists(os.path.join(d, fn)):
                k += 1
                fn = '%05d_%s~%d.txt' % (c["src_line"], safe(c["name"]), k)
            hdr = '; %s  src=%s:%d  ir=%s/%s:%d..%d (%d lines)  sym=%s\n' % (
                c["name"], c["file"], c["src_line"], root, mod, s, e, e - s + 1, c["sym"])
            io.open(os.path.join(d, fn), 'w', encoding='utf-8').write(hdr + '\n'.join(res) + '\n')
            index.setdefault(c["file"], []).append((c["src_line"], c["name"], e - s + 1, os.path.join(os.path.basename(d), fn)))
        print(root, mod, len(fns), 'fn')
    with io.open(os.path.join(outdir, 'INDEX.md'), 'w', encoding='utf-8') as f:
        f.write('# 밴픽 IR 전수 덤프 인덱스 (bpdump.py)\n\n')
        for file, fs in sorted(index.items()):
            f.write('## %s (%d fn, %d IR lines)\n' % (file, len(fs), sum(x[2] for x in fs)))
            for sl, name, n, rel in sorted(fs):
                f.write('- L%d `%s` (%d) → `%s`\n' % (sl, name, n, rel))
            f.write('\n')
    print('done', sum(len(v) for v in index.values()), 'fn ->', outdir)

if __name__ == '__main__':
    main()
