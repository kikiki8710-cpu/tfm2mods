# -*- coding: utf-8 -*-
"""소스 앵커(패닉 Location)가 없는 함수의 0.5.4 짝을 **호출자 문맥**으로 찾는다.

원리: 그 함수를 부르는 0.5.3 함수는 대개 소스 앵커가 있다. 그 호출자의 0.5.4 짝을
구한 뒤, 짝 안에서 **같은 순번의 call** 이 가리키는 곳이 답이다.
크기·골격으로 교차확인해서 우연 일치를 거른다.

  python bycaller.py d90bd0 d0cab0
"""
import io, sys, collections

sys.path.insert(0, r'C:\tfm2mods\v54')
import reloc as R
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
B = 0x140000000
E3, E4 = R.E3, R.E4


def calls(E, s, e):
    out = []
    for i in R.insns(E, s, e):
        if i.mnemonic == 'call' and i.op_str.startswith('0x'):
            try:
                out.append((i.address - B, int(i.op_str, 16) - B))
            except ValueError:
                pass
    return out


def go(target):
    f3 = E3.func_of(target)
    print('053 대상 %06x-%06x (%dB)  src=%s'
          % (f3[0], f3[1], f3[1] - f3[0], R.SRC3.get(f3[0], '(없음)')))
    hits = []
    for s, e, src in R.S3:
        cl = [c for c in calls(E3, s, e)]
        idxs = [k for k, (a, t) in enumerate(cl) if t == f3[0]]
        if idxs:
            hits.append((s, e, src, cl, idxs))
    if not hits:
        print('  호출자 없음 — 다른 앵커 필요')
        return
    votes = collections.Counter()
    for s, e, src, cl, idxs in hits:
        pr = R.pair_fn(s, e)
        print('  호출자 %06x [%s] 골격 %s'
              % (s, src.split(chr(92))[-1][:34], ('%.0f%%' % (pr[2] * 100)) if pr else '짝없음'))
        if not pr:
            continue
        cl4 = calls(E4, pr[0], pr[1])
        for k in idxs:
            if k < len(cl4):
                votes[cl4[k][1]] += 1
    print('  후보(같은 순번 call 타깃):')
    for t, n in votes.most_common(5):
        g = E4.func_of(t)
        sz = (g[1] - g[0]) if g else 0
        print('    %06x  득표 %d  크기 %dB (053 %dB, 차 %+d)  src=%s'
              % (t, n, sz, f3[1] - f3[0], sz - (f3[1] - f3[0]),
                 next((x[2] for x in R.S4 if x[0] == t), '(없음)')[:60]))


if __name__ == '__main__':
    for a in sys.argv[1:]:
        go(int(a, 16))
        print()
