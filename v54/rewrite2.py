# -*- coding: utf-8 -*-
"""detour.rs 0.5.3 → 0.5.4 실제 재작성.

방침
  · 균일한 루프 = 제자리에서 주소·prefix 만 교체
  · prefix 가 갈리거나 미확정이 섞인 루프 = **개별 `p!` 로 펼침**
  · 미확정 사이트 = `pskip!` (tot 에는 세되 패치 안 함)
원본은 `detour.rs.053bak` 으로 보존한다.
"""
import io, os, re, sys, collections, shutil

sys.path.insert(0, r'C:\tfm2mods\v54')
import sites as S
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

D = r'C:\tfm2mods\v54'
SRCDIR = r'C:\tfm2mods\tfm2_ai_adjust\src'
DET = os.path.join(SRCDIR, 'detour.rs')

PSKIP = ('    macro_rules! pskip { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{\n'
         '        // ⛔0.5.4 재배치 미확정 — 세기만 하고 **패치하지 않는다**.\n'
         '        //    (orig_guard 는 표에 없는 RVA 를 통과시키므로, 옛 주소를 남기면 오패치 위험)\n'
         '        tot += 1; let _ = ($a, $pre, $off, $w, $v);\n'
         '    }}; }\n')


def load_map():
    m = {}
    first = True
    for ln in io.open(os.path.join(D, 'map_054.tsv'), encoding='utf-8'):
        if first:
            first = False
            continue
        p = ln.rstrip('\n').split('\t')
        m[(int(p[3], 16), int(p[5]), int(p[6]))] = dict(
            new=int(p[4], 16) if p[4] != '-' else 0, newpre=p[8],
            tier=p[9], why=p[10])
    return m


def pre_lit(h):
    return '&[' + ','.join('0x%02x' % b for b in bytes.fromhex(h)) + ']'


PRE_RE = re.compile(r'&\[(?:\s*0x[0-9a-fA-F]{1,2}\s*,?)+\]')


def main():
    mp = load_map()
    site = S.parse()
    lines = io.open(DET, encoding='utf-8').read().split('\n')

    bysite = {}
    for x in site:
        bysite.setdefault((x['rva'], x['off'], x['w']), []).append(x)

    # ── 루프 분류 ──────────────────────────────────────────
    loops = collections.defaultdict(list)
    for x in site:
        if x['loop']:
            loops[(x['line'], x['loop'][0], x['loop'][1], x['off'], x['w'])].append(x)

    expand = set()          # 펼쳐야 하는 루프 키
    for k, lst in loops.items():
        pres, bad = set(), False
        for x in lst:
            r = mp.get((x['rva'], x['off'], x['w']))
            if not r or not r['tier'].startswith('확정'):
                bad = True
            else:
                pres.add(r['newpre'])
        if bad or len(pres) > 1:
            expand.add(k)

    stat = collections.Counter()
    out = []
    i = 0
    while i < len(lines):
        ln = lines[i]
        raw = ln

        # ① 이 줄에서 시작하는 루프인가?
        lk = [k for k in loops if k[0] == i + 1]
        if lk:
            # 루프 블록 끝(닫는 중괄호가 있는 줄)까지 모은다
            j = i
            depth = 0
            block = []
            while j < len(lines):
                block.append(lines[j])
                depth += lines[j].count('{') - lines[j].count('}')
                if depth <= 0 and j > i:
                    break
                if depth <= 0 and '{' in lines[j] and '}' in lines[j]:
                    break
                j += 1
            btxt = '\n'.join(block)
            indent = re.match(r'\s*', lines[i]).group(0)

            if any(k in expand for k in lk):
                # 개별 `p!` 로 펼친다
                new = []
                for k in lk:
                    for x in sorted(loops[k], key=lambda y: y['rva']):
                        r = mp.get((x['rva'], x['off'], x['w']))
                        pre = pre_lit(r['newpre']) if r and r['tier'].startswith('확정') else \
                              pre_lit(''.join('%02x' % b for b in x['pre'][0]))
                        # 원래 루프 안 p! 의 값 인자를 그대로 쓴다
                        mcall = re.search(r'\b(p|pany)!\(\s*base\s*\+\s*%s\s*,\s*.+?\s*,\s*%d\s*,\s*%d\s*,\s*(.+?)\);'
                                          % (re.escape(k[1]), x['off'], x['w']), btxt, re.S)
                        vexpr = mcall.group(2).strip() if mcall else '/*값?*/'
                        if r and r['tier'].startswith('확정'):
                            new.append('%sp!(base + 0x%06x, %s, %d, %d, %s);   // ←0.5.3 %06x'
                                       % (indent, r['new'], pre, x['off'], x['w'], vexpr, x['rva']))
                            stat['확정'] += 1
                        else:
                            new.append('%spskip!(base + 0x%06x, %s, %d, %d, %s);   // ⛔0.5.4 미확정: %s'
                                       % (indent, x['rva'], pre, x['off'], x['w'], vexpr,
                                          (r['why'] if r else '매핑없음')[:60]))
                            stat['pskip'] += 1
                out.append('%s// ↓0.5.4: prefix 가 사이트마다 달라져 루프를 펼침(원래 `for %s in [..]`)'
                           % (indent, lk[0][1]))
                out.extend(new)
            else:
                # 제자리 교체: 배열 주소 + prefix
                nb = btxt
                for k in lk:
                    for x in loops[k]:
                        r = mp[(x['rva'], x['off'], x['w'])]
                        nb = re.sub(r'0x0*%x(usize)?' % x['rva'],
                                    lambda m: '0x%06x%s' % (r['new'], m.group(1) or ''), nb)
                        stat['확정'] += 1
                    r0 = mp[(loops[k][0]['rva'], loops[k][0]['off'], loops[k][0]['w'])]
                    nb = re.sub(r'(\b(?:p|pany)!\(\s*base\s*\+\s*%s\s*,\s*)%s'
                                % (re.escape(k[1]), PRE_RE.pattern),
                                lambda m: m.group(1) + pre_lit(r0['newpre']), nb, count=1)
                out.extend(nb.split('\n'))
            i = j + 1
            continue

        # ② 단독 사이트
        m = re.search(r'\b(p|pany)!\(\s*base\s*\+\s*0x([0-9a-fA-F]+)', ln)
        if m:
            old = int(m.group(2), 16)
            hit = [k for k in bysite if k[0] == old and any(y['loop'] is None for y in bysite[k])]
            if hit:
                k = hit[0]
                r = mp.get(k)
                if r and r['tier'].startswith('확정'):
                    ln = re.sub(r'(\bbase\s*\+\s*)0x%x\b' % old,
                                lambda mm: mm.group(1) + '0x%06x' % r['new'], ln, count=1)
                    if m.group(1) == 'p':
                        ln = PRE_RE.sub(lambda mm: pre_lit(r['newpre']), ln, count=1)
                    ln = ln.rstrip() + '   // ←0.5.3 %06x' % old
                    stat['확정'] += 1
                else:
                    ln = re.sub(r'\b(p|pany)!\(', 'pskip!(', ln, count=1)
                    ln = ln.rstrip() + '   // ⛔0.5.4 미확정: %s' % ((r['why'] if r else '매핑없음')[:60])
                    stat['pskip'] += 1
        out.append(ln)
        i += 1

    # ③ pskip! 매크로를 p! 정의 옆에 심는다
    res = []
    for ln in out:
        res.append(ln)
        if re.match(r'\s*macro_rules!\s+p\s*\{', ln):
            pass
    txt = '\n'.join(out)
    txt = re.sub(r'(\n(\s*)macro_rules! p \{[^\n]*\n(?:.*?\n)?\s*\}\}; \}\n)',
                 lambda m: m.group(1) + PSKIP, txt)

    shutil.copyfile(DET, DET + '.053bak')
    io.open(DET, 'w', encoding='utf-8', newline='\n').write(txt)
    print('재작성 완료  확정 %d · pskip %d' % (stat['확정'], stat['pskip']))
    print('원본 백업 = detour.rs.053bak')


if __name__ == '__main__':
    main()
