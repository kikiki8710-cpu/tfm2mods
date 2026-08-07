# -*- coding: utf-8 -*-
"""재배치 2차 — 1차에서 '확정'이 안 된 사이트를 **완화 매칭**으로 좁힌다.

1차는 실바이트 시그니처가 같아야 확정한다. 그래서 **레지스터가 재할당된 자리**
(예: `49 83 c6 78`(r14) → `49 83 c5 78`(r13))는 전부 탈락한다. 그게 오히려 정상이다 —
바이트가 달라졌으면 모드의 패치 코드도 같이 고쳐야 하기 때문이다.

여기서는 한 단계 완화한다: **니모닉 + 즉치 오프셋 + 폭 + 원본값**이 같으면 후보로 본다.
그리고 **바이트가 어떻게 달라졌는지를 같이 출력**해서, 모드 쪽에서 무엇을 고쳐야
하는지 바로 알 수 있게 한다.

⚠확정은 여전히 **개수가 같을 때만** 한다. 개수가 다르면 '반쪽 노브'(사이트가 늘거나
   줄어든 것)일 수 있으므로 사람이 봐야 한다 — 이 프로젝트에서 반복된 사고 유형이다.
"""
import io, os, sys, collections

sys.path.insert(0, r'C:\tfm2mods\v54')
import reloc as R
from pe2 import load
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
# ⚠reloc 은 __main__ 에서만 감싼다. `import reloc` 이 이미 감쌌고, 재대입하면
#   앞 래퍼가 GC 되면서 **바탕 buffer 를 닫아** "closed file" 로 터진다(실측 2회).

D = r'C:\tfm2mods\v54'
E3, E4 = R.E3, R.E4
B = 0x140000000


def load_rows():
    rows = []
    first = True
    for ln in io.open(os.path.join(D, 'reloc_054.tsv'), encoding='utf-8'):
        if first:
            first = False
            continue
        p = ln.rstrip('\n').split('\t')
        rows.append([int(p[0], 16), p[1], int(p[2]), int(p[3]), int(p[4]), p[5], p[6]])
    return rows


def main():
    rows = load_rows()
    todo = [r for r in rows if r[5] != '확정']
    byfn = collections.defaultdict(list)
    for r in todo:
        f = E3.func_of(r[0])
        if f:
            byfn[f].append(r)

    fixed = collections.Counter()
    regchg = []
    for (fs, fe), sites in sorted(byfn.items()):
        pr = R.pair_fn(fs, fe)
        if not pr:
            continue
        bs, be, ratio = pr
        i3 = {i.address - B: i for i in R.insns(E3, fs, fe)}
        i4 = R.insns(E4, bs, be)

        for r in sites:
            rva, off, w, orig = r[0], r[2], r[3], r[4]
            ins = i3.get(rva)
            if ins is None:
                continue
            # 완화 키 = 니모닉 + 길이 + 즉치 위치/폭 + 원본값
            def key(x):
                return (x.mnemonic, len(x.bytes))
            k3 = [a for a, x in sorted(i3.items())
                  if key(x) == key(ins) and off + w <= len(x.bytes)
                  and R.val(x, off, w) == orig]
            k4 = [x.address - B for x in i4
                  if key(x) == key(ins) and off + w <= len(x.bytes)
                  and R.val(x, off, w) == orig]
            if not k4 or len(k3) != len(k4):
                r[6] += ' | 완화매칭 %d→%d' % (len(k3), len(k4))
                continue
            idx = k3.index(rva)
            tgt = k4[idx]
            t = next(x for x in i4 if x.address - B == tgt)
            r[1] = '%06x' % tgt
            r[5] = '확정(완화)'
            r[6] = '%d/%d번째, 니모닉·값 동일, 골격 %.0f%%' % (idx + 1, len(k3), ratio * 100)
            fixed['확정(완화)'] += 1
            if R.sig(ins, off, w) != R.sig(t, off, w):
                regchg.append((rva, tgt, off, w, orig,
                               ins.bytes.hex(), '%s %s' % (ins.mnemonic, ins.op_str),
                               t.bytes.hex(), '%s %s' % (t.mnemonic, t.op_str)))

    with io.open(os.path.join(D, 'reloc_054.tsv'), 'w', encoding='utf-8', newline='') as fo:
        fo.write('rva053\trva054\timm_off\twidth\torig\t판정\t근거\n')
        for r in rows:
            fo.write('%06x\t%s\t%d\t%d\t%d\t%s\t%s\n'
                     % (r[0], r[1], r[2], r[3], r[4], r[5], r[6]))

    c = collections.Counter(r[5] for r in rows)
    print('2차 후 판정')
    for k, v in c.most_common():
        print('  %-14s %4d' % (k, v))

    print('\n★바이트가 달라진 자리 %d개 — 모드의 패치 바이트를 같이 고쳐야 한다' % len(regchg))
    for rva, tgt, off, w, orig, b3, a3, b4, a4 in regchg:
        print('  %06x → %06x  (off %d, %dB, 원본 %d)' % (rva, tgt, off, w, orig))
        print('      053 %-22s %s' % (b3, a3))
        print('      054 %-22s %s' % (b4, a4))

    with io.open(os.path.join(D, 'reloc_regchange.txt'), 'w', encoding='utf-8') as fo:
        for x in regchg:
            fo.write('%06x\t%06x\t%d\t%d\t%d\t%s\t%s\t%s\t%s\n' % x)

    print('\n남은 미확정:')
    for r in rows:
        if not r[5].startswith('확정'):
            print('  %06x  off%d %dB 원본%-12d %s — %s' % (r[0], r[2], r[3], r[4], r[5], r[6][:88]))


if __name__ == '__main__':
    main()
