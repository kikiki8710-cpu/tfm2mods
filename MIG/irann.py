#!/usr/bin/env python3
"""irann.py — SDK LLVM IR(.ll) 한 함수를 **읽을 수 있게** 주석·축약해 출력한다.

왜: `_gaibc/*.ll` 원문은 한 줄이 400자를 넘고 `!dbg !12345` 뿐이라 사람이 못 읽는다.
    이 도구는 ①DILocation 을 소스 줄(`;L410` / 인라인이면 `;L410<377`)로 풀고
    ②`#dbg_value(v, !var, ...)` 를 `;; name = v` 로 바꾸고 ③속성·메타 잡음을 지우고
    ④망글링 심볼을 `ai::`/`gc::` 로 줄여 **원본 소스에 가까운 형태**로 만든다.

사용:
    python MIG\\irann.py <module.ll> <함수이름조각> [-o out.ll] [--no-dbg] [--no-ln]   (기본: 원문 줄번호 `NNNNN| ` 접두)
    python MIG\\irann.py C:\\tfm2mods\\_gaibc\\m07.ll position_eval_at_uncached -o pe.ll

판정 근거로 쓸 때 주의: 주석은 **보조**다. 오프셋·상수는 원문 줄에서 그대로 읽는다.
"""
import io, re, sys, os

NOISE = [
    (re.compile(r'\bnoalias\s+'), ''), (re.compile(r'\bnoundef\s+'), ''),
    (re.compile(r'\bnonnull\s+'), ''), (re.compile(r'\breadonly\s+'), ''),
    (re.compile(r'\bwriteonly\s+'), ''), (re.compile(r'\bdead_on_unwind\s+'), ''),
    (re.compile(r'\bwritable\s+'), ''), (re.compile(r'\binbounds nuw\s+'), ''),
    (re.compile(r'\binbounds\s+'), ''),
    (re.compile(r'captures\([^)]*\)\s*'), ''),
    (re.compile(r'dereferenceable(?:_or_null)?\([0-9]+\)\s*'), ''),
    (re.compile(r'align\s+\d+\s*'), ''),
    (re.compile(r',\s*!(?:tbaa|noalias|alias\.scope|align|noundef|invariant\.load|range|prof|nonnull|nosanitize)\s*![0-9]+'), ''),
    (re.compile(r'\brange\(i\d+ -?\d+, -?\d+\)\s*'), ''),
    (re.compile(r'getelementptr i8, ptr'), 'gep'),
    (re.compile(r'getelementptr\s+\{[^}]*\},\s*ptr'), 'gepS'),
]
SYM = [
    (re.compile(r'@_RNv[A-Za-z0-9_]*?7game_ai([0-9]+)([A-Za-z0-9_]+)'), lambda m: '@ai::' + m.group(2)),
    (re.compile(r'@_RNv[A-Za-z0-9_]*?9game_core([0-9]+)([A-Za-z0-9_]+)'), lambda m: '@gc::' + m.group(2)),
    (re.compile(r'@_RINv[A-Za-z0-9_]*?4core([0-9]+)([A-Za-z0-9_]+)'), lambda m: '@core::' + m.group(2)),
    (re.compile(r'@_RNv[A-Za-z0-9_]*?4core([0-9]+)([A-Za-z0-9_]+)'), lambda m: '@core::' + m.group(2)),
]


def load_meta(path):
    """`!N = ...` 메타 줄만 dict 로. (전체 파일을 한 번 읽는다 — 200MB 도 수 초)"""
    meta = {}
    with io.open(path, encoding='utf-8', errors='ignore') as f:
        for line in f:
            if line.startswith('!'):
                k = line.split(' ', 1)[0]
                meta[k] = line.rstrip('\n')
    return meta


def loc_chain(meta, mid, depth=0):
    """DILocation → 'line' 또는 'line<부모line<...' (인라인 체인)."""
    l = meta.get('!' + mid)
    if l is None or 'DILocation' not in l or depth > 12:   # 09-15: 6 → 12(r14 get_input 계열이 7단 인라인 · 사슬이 잘렸다)
        return ''
    ln = re.search(r'line:\s*(\d+)', l)
    out = ln.group(1) if ln else '?'
    ia = re.search(r'inlinedAt:\s*!(\d+)', l)
    if ia:
        sub = loc_chain(meta, ia.group(1), depth + 1)
        if sub:
            out += '<' + sub
    return out


def var_name(meta, mid):
    l = meta.get('!' + mid)
    if l is None:
        return None
    m = re.search(r'name:\s*"([^"]*)"', l)
    if m:
        return m.group(1)
    # DIExpression 조각이면 부모 변수명이 없다
    return None


def find_fn(path, frag):
    """함수 정의의 (시작줄, 끝줄) 1-based. frag 는 심볼 부분 문자열.
    ★09-15: 조각이 여러 define 에 걸리면 **심볼이 그 조각으로 끝나는 것(본체)** 을 우선한다 — `get_input` 조각이
      `…get_input…{closure}` 인스턴스(먼저 나옴)에 걸려 엉뚱한 함수를 주석하던 함정(heapsurf 09-14 와 같은 부류)."""
    cands = []
    with io.open(path, encoding='utf-8', errors='ignore') as f:
        for i, line in enumerate(f, 1):
            if line.startswith('define') and frag in line:
                m = re.search(r'@(\S+?)\(', line)
                sym = m.group(1) if m else ''
                cands.append((0 if sym.endswith(frag) else 1, i))
    if not cands:
        return None, None
    cands.sort()
    start = cands[0][1]
    end = None
    with io.open(path, encoding='utf-8', errors='ignore') as f:
        for i, line in enumerate(f, 1):
            if i > start and line.startswith('}'):
                end = i
                break
    return start, end


def annotate(path, frag, out, keep_dbg=True, ln=True):
    meta = load_meta(path)
    s, e = find_fn(path, frag)
    if s is None:
        print('NOT FOUND: %s in %s' % (frag, path), file=sys.stderr)
        return 1
    res = []
    with io.open(path, encoding='utf-8', errors='ignore') as f:
        for i, line in enumerate(f, 1):
            if i < s:
                continue
            if i > e:
                break
            line = line.rstrip('\n')
            st = line.strip()
            # ── #dbg_value / #dbg_declare → ;; name = value
            m = re.match(r'#dbg_(?:value|declare)\((.*), !(\d+), !DIExpression\((.*?)\), !(\d+)\)', st)
            if m:
                if not keep_dbg:
                    continue
                val, vid, expr, _ = m.groups()
                nm = var_name(meta, vid)
                if nm is None:
                    continue
                if 'poison' in val:
                    continue
                frag_ex = ''
                fm = re.search(r'DW_OP_LLVM_fragment,\s*(\d+),\s*(\d+)', expr)
                if fm:
                    frag_ex = '[%d..+%d]' % (int(fm.group(1)) // 8, int(fm.group(2)) // 8)
                res.append(('%6d| ' % i if ln else '') + '    ;; %s%s = %s' % (nm, frag_ex, val.strip()))
                continue
            if st.startswith('#dbg_') or st.startswith('call void @llvm.dbg'):
                continue
            if 'llvm.lifetime' in st or 'llvm.experimental.noalias' in st:
                continue
            # ── !dbg !N → ;L…
            loc = ''
            dm = re.search(r'!dbg !(\d+)', line)
            if dm:
                loc = loc_chain(meta, dm.group(1))
                line = re.sub(r',?\s*!dbg !\d+', '', line)
            for rx, rep in NOISE:
                line = rx.sub(rep, line)
            for rx, rep in SYM:
                line = rx.sub(rep, line)
            line = re.sub(r'\s+', ' ', line.rstrip())
            if loc:
                line = '%-118s ;L%s' % (line, loc)
            # ★09-15(23차 적발 · 명세 배치 5개가 각자 슬라이서를 짰다): 원문 `.ll` 줄번호를 접두해 `m07.ll:NNNNN` 인용을
            #   바로 쓸 수 있게 한다(`--no-ln` 으로 끈다). 판정 근거는 여전히 원문 줄이다.
            res.append(('%6d| ' % i + line) if ln else line)
    txt = '\n'.join(res) + '\n'
    if out:
        io.open(out, 'w', encoding='utf-8').write(txt)
        print('%s: %d..%d (%d lines) -> %s (%d lines)' % (os.path.basename(path), s, e, e - s + 1, out, len(res)))
    else:
        sys.stdout.write(txt)
    return 0


if __name__ == '__main__':
    a = sys.argv[1:]
    if len(a) < 2:
        print(__doc__)
        sys.exit(2)
    out = None
    keep = True
    if '-o' in a:
        i = a.index('-o'); out = a[i + 1]; del a[i:i + 2]
    if '--no-dbg' in a:
        keep = False; a.remove('--no-dbg')
    ln = True
    if '--no-ln' in a:
        ln = False; a.remove('--no-ln')
    sys.exit(annotate(a[0], a[1], out, keep, ln))
