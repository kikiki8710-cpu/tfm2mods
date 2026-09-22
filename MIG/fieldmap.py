#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""fieldmap.py — 구/신 함수를 **명령 단위 정렬**(anchor LIS + 틈 difflib)로 맞춘 뒤,
정렬된 명령 쌍에서 메모리 disp 가 달라진 자리를 뽑아 **(구off → 신off) 필드 이동표**를 만든다.
왜 (2026-09-16 · 0.6.0):
    `posdiff.py` 는 "명령 인덱스가 같은 자리끼리" 비교한다 — 명령 수가 같을 때만 믿을 수 있다.
    0.6.0 은 전면 재컴파일(함수 +11%)이라 거의 모든 함수의 명령 수가 다르다.
    `offsets.py check` 는 집합 diff 라 어느 필드가 어디로 갔는지 1:1 이 안 나온다.
    ⟹ 정렬 기반으로 1:1 대응을 뽑는다. 결과는 **후보**다 — 소스 반영 전 각 자리를 문맥으로 재확인한다.
방법:
    ① 레지스터 지운 키(loose)로 유일 4-gram 앵커를 잡고 LIS 로 단조 정렬
    ② 앵커 사이 틈은 difflib(loose 키) 로 채움
    ③ 정렬된 쌍 중 니모닉·오퍼랜드 형태가 같고 disp 만 다른 것을 표로
사용:
  python MIG\fieldmap.py --old <구exe> --oldpkl <구pkl> --new <신exe> --newpkl <신pkl>
        (--fn <구RVA> <신RVA> | --mod <MOD> [--mod ...])   [--lo 0x40] [--hi 0xffff] [--json out.json]
  --mod 는 매니페스트 함수시작 엔트리 중 offsets/<MOD>.json(구 지문)에 있는 함수 전부를 돈다.
"""
import sys, os, re, json, argparse, difflib, collections, bisect
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import midpin as MP
import mig_verify as MV

DISP = re.compile(r'\[(r[a-z0-9]+|e[a-z]{2}) ([+-]) 0x([0-9a-f]+)\]')
_REG = re.compile(r"\b(r[a-z0-9]+|e[a-z]{2}|[abcd][lhx]|[sd]il?|[sb]pl?|xmm\d+)\b")


def loose(m, form):
    return "%s|%s" % (m, _REG.sub("R", form))


def walk(img, fn):
    """[(rva, mnem, op_str, loose_key, [(base, disp)])] — 스택 상대(rbp/rsp)는 disp 목록에서 제외."""
    out = []
    for a, m, fm, sz in img.insns(fn):
        pass
    # midpin.insns 는 op_str 원문을 안 남기므로 직접 디스어셈
    f = img.fn[fn]
    code = img.read(fn, f['size'])
    for ins in MP.md.disasm(code, fn):
        ds = [(g.group(1), int(g.group(3), 16) * (1 if g.group(2) == '+' else -1))
              for g in DISP.finditer(ins.op_str) if g.group(1) not in ('rbp', 'rsp', 'ebp', 'esp')]
        form = "".join(c for c in ins.op_str if not c.isdigit()).replace("0x", "")
        out.append((ins.address, ins.mnemonic, ins.op_str, loose(ins.mnemonic, form), ds))
    return out


def align(ow, nw, K=4):
    """anchor(유일 K-gram) LIS + 틈 difflib. 반환 [(i, j)] 단조 쌍."""
    ok = [x[3] for x in ow]; nk = [x[3] for x in nw]

    def grams(keys):
        d = collections.defaultdict(list)
        for i in range(len(keys) - K + 1):
            d[tuple(keys[i:i + K])].append(i)
        return d
    go, gn = grams(ok), grams(nk)
    cand = sorted((io[0], gn[g][0]) for g, io in go.items() if len(io) == 1 and g in gn and len(gn[g]) == 1)
    # LIS on j
    tails, prev, idx = [], [-1] * len(cand), []
    for t, (i, j) in enumerate(cand):
        p = bisect.bisect_left([cand[x][1] for x in tails], j)
        if p == len(tails):
            tails.append(t)
        else:
            tails[p] = t
        prev[t] = tails[p - 1] if p > 0 else -1
    lis = []
    t = tails[-1] if tails else -1
    while t != -1:
        lis.append(cand[t]); t = prev[t]
    lis.reverse()
    pairs = []
    # 앵커 K-gram 전체를 쌍으로 + 틈 채움
    anchors = []
    for i, j in lis:
        anchors.append((i, j, K))
    pi, pj = 0, 0
    for i, j, ln in anchors + [(len(ok), len(nk), 0)]:
        if i > pi and j > pj:
            sm = difflib.SequenceMatcher(a=ok[pi:i], b=nk[pj:j], autojunk=False)
            for a0, b0, l2 in sm.get_matching_blocks():
                for q in range(l2):
                    pairs.append((pi + a0 + q, pj + b0 + q))
        for q in range(ln):
            pairs.append((i + q, j + q))
        pi, pj = i + ln, j + ln
    # 중복/역행 제거
    seen, outp, lastj = set(), [], -1
    for i, j in sorted(set(pairs)):
        if j > lastj:
            outp.append((i, j)); lastj = j
    return outp


def fieldmap(O, N, of, nf, lo, hi):
    ow, nw = walk(O, of), walk(N, nf)
    pairs = align(ow, nw)
    votes = collections.Counter(); rows = []
    same_disp = collections.Counter()
    for i, j in pairs:
        oa, om, oo, okey, od = ow[i]; na, nm, no, nkey, nd = nw[j]
        if om != nm or len(od) != len(nd) or not od:
            continue
        for (ob, ov), (nb, nv) in zip(od, nd):
            if not (lo <= abs(ov) <= hi):
                continue
            if ov == nv:
                same_disp[ov] += 1
            else:
                votes[(ov, nv)] += 1
                rows.append((oa, na, om, ob, nb, ov, nv))
    return ow, nw, pairs, votes, rows, same_disp


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--old', required=True), ap.add_argument('--oldpkl', required=True)
    ap.add_argument('--new', required=True), ap.add_argument('--newpkl', required=True)
    ap.add_argument('--fn', nargs=2, action='append', default=[])
    ap.add_argument('--mod', action='append', default=[])
    ap.add_argument('--lo', type=lambda x: int(x, 0), default=0x40)
    ap.add_argument('--hi', type=lambda x: int(x, 0), default=0xffff)
    ap.add_argument('--json', default=None)
    a = ap.parse_args()
    O = MP.Img(a.old, a.oldpkl); N = MP.Img(a.new, a.newpkl)
    jobs = [(None, x, int(y, 0), int(z, 0)) for x, y, z in [('fn',) + tuple(p) for p in a.fn]]
    for mod in a.mod:
        man = MV.load_man(mod)
        fp = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'offsets', mod + '.json')
        base = json.load(open(fp, encoding='utf-8'))['fns'] if os.path.isfile(fp) else {}
        cur = {e['name']: e for e in man['entries'] if not e.get('ignore')}
        for oldv, rec in base.items():
            e = cur.get(rec['name'])
            if not e:
                continue
            of, nf = int(oldv, 16), int(e['value'], 16)
            if of not in O.fn or nf not in N.fn or of == nf:
                continue
            jobs.append((mod, rec['name'], of, nf))
    out = {}
    for mod, name, of, nf in jobs:
        ow, nw, pairs, votes, rows, same = fieldmap(O, N, of, nf, a.lo, a.hi)
        tag = '%s.%s' % (mod, name) if mod else name
        print('== %-40s 구 %s(%d명령) ↔ 신 %s(%d명령) 정렬쌍 %d (%.0f%%)'
              % (tag, hex(of), len(ow), hex(nf), len(nw), len(pairs), 100.0 * len(pairs) / max(1, len(ow))))
        if votes:
            print('   이동:', ', '.join('%#x→%#x(x%d)' % (o, n, c) for (o, n), c in sorted(votes.items(), key=lambda kv: (-kv[1], kv[0]))[:24]))
        moved = {o for (o, n) in votes}
        stay = sorted(o for o in same if o not in moved)
        if stay:
            print('   불변:', ' '.join('%#x' % o for o in stay[:40]))
        out[tag] = {'old': hex(of), 'new': hex(nf), 'n_old': len(ow), 'n_new': len(nw), 'pairs': len(pairs),
                    'moves': [{'old': hex(o), 'new': hex(n), 'votes': c} for (o, n), c in votes.items()],
                    'stay': [hex(o) for o in stay]}
    if a.json:
        json.dump(out, open(a.json, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
        print('->', a.json)


if __name__ == '__main__':
    main()


def trace(O, N, of, nf, disps, ctxn=1):
    """특정 구 disp 들의 사용 자리마다 정렬된 신 명령의 disp 를 나열(불변 포함). 판정 근거용."""
    ow, nw = walk(O, of), walk(N, nf)
    pairs = dict(align(ow, nw))
    res = collections.defaultdict(list)
    for i, (oa, om, oo, ok, od) in enumerate(ow):
        for ob, ov in od:
            if ov not in disps:
                continue
            j = pairs.get(i)
            if j is None:
                res[ov].append((hex(oa), oo, None, None)); continue
            na, nm, no, nk, nd = nw[j]
            nv = [v for b, v in nd]
            res[ov].append((hex(oa), oo, hex(na), no))
    return res
