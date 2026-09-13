# -*- coding: utf-8 -*-
"""G12 재검: consts[].src_line ↔ 리터럴 사용 명령의 !dbg 루트 줄(명세 src 파일 기준)."""
import sys, re, json, collections
sys.path.insert(0, r'C:\Users\jungs\AppData\Local\Temp\claude\C--Users-jungs-Desktop-claude-tfm2--claude-worktrees-swap-order-button-style-fe9e04\8c613d5e-d69c-428f-aff5-fe06054da7f0\scratchpad20A')
import irlib
SPEC = json.load(open(r'C:\tfm2mods\MIG\_spec\specs20_v3.json', encoding='utf-8'))['specs']

def lit_hits(ls, val):
    """명령 텍스트에서 값 val 이 '진짜 상수 피연산자'로 쓰이는지. 잡음 제거."""
    v = str(val)
    if ls.startswith('#dbg') or ls.startswith(';') or ls.startswith('!'):
        return False
    if re.match(r'^\s*%[\w.]+ = getelementptr', ls):
        return False
    if re.match(r'^\s*%[\w.]+ = alloca', ls):
        return False
    s = re.sub(r'!dbg !\d+|!\w+ !\d+', '', ls)
    s = re.sub(r'align \d+', '', s)
    s = re.sub(r'%[\w.]+', '%R', s)
    s = re.sub(r'dereferenceable(?:_or_null)?\(\d+\)', '', s)
    s = re.sub(r'range\([^)]*\)', '', s)
    s = re.sub(r'label %R', '', s)
    s = re.sub(r'@[\w.$]+', '@SYM', s)
    s = re.sub(r'\[\d+ x ', '[N x ', s)
    s = re.sub(r'\bi(8|16|24|32|64|128)\b', 'iT', s)
    return re.search(r'(?<![\w.\-])' + re.escape(v) + r'(?![\w.])', s) is not None

def run(idx):
    s = SPEC[idx]
    srcbase = s['src'].split('\\')[-1]
    ranges = [(s['ir']['file'], s['ir']['frm'], s['ir']['to'])] + [(x['file'], x['frm'], x['to']) for x in s['ir'].get('aux', [])]
    out = []
    for ci, c in enumerate(s['consts']):
        val = c['value']
        cands = collections.Counter()
        ex = []
        for (f, a, b) in ranges:
            lines, md = irlib.load(f)
            for k in range(a - 1, b):
                ln = lines[k]
                if not lit_hits(ln, val):
                    continue
                d = irlib.dbg_of(ln)
                if d is None:
                    continue
                ch = irlib.chain(md, d)
                rt = ch[-1] if ch else None
                if not rt:
                    continue
                fb = rt[2].split('\\')[-1]
                key = (fb, rt[0], f == s['ir']['file'])
                cands[key] += 1
                if len(ex) < 400:
                    ex.append((f, k + 1, fb, rt[0], ln.strip()[:100]))
        claimed = c['src_line']
        own = sorted(set(l for (fb, l, main) in cands if fb == srcbase))
        ok = claimed in own
        out.append((ci, val, claimed, ok, own, ex))
    return s, out

if __name__ == '__main__':
    idx = int(sys.argv[1])
    s, out = run(idx)
    print('# specs[%d] %s src=%s' % (idx, s['name'], s['src']))
    for ci, val, claimed, ok, own, ex in out:
        print('consts[%d] value=%s claimed=%s %s own-root-lines=%s' % (ci, val, claimed, 'OK' if ok else '**MISS**', own[:40]))
        if not ok or '-v' in sys.argv:
            for e in ex[:12]:
                print('     %s:%d root=%s:%s  %s' % e)
