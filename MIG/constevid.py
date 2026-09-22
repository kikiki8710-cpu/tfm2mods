#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""constevid.py — 모드 소스의 구조체 오프셋 상수(`const X: usize = 0x..;` 소형값) 하나하나에 대해,
그 모드가 훅/호출하는 게임 함수(매니페스트 함수시작 엔트리 · 구→신 대응)들 안에서
그 disp 가 **정렬된 신 명령에서 무엇으로 나타나는지** 를 집계한다 (fieldmap.align 기반).
출력: 상수명 값 | 함수별 [불변 n / 이동 →v n / 미정렬 n] | 판정(불변/이동후보/근거없음)
사용: python MIG\constevid.py <MOD> --old .. --oldpkl .. --new .. --newpkl .. [--src 파일] [--lo 0x40]
"""
import sys, os, re, json, argparse, collections
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import midpin as MP, fieldmap as FM, mig_verify as MV
CONST = re.compile(r'^(?:pub )?const ([A-Z_0-9]+): usize = (0x[0-9a-fA-F]+);', re.M)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('mod')
    ap.add_argument('--old', required=True), ap.add_argument('--oldpkl', required=True)
    ap.add_argument('--new', required=True), ap.add_argument('--newpkl', required=True)
    ap.add_argument('--src', default=None)
    ap.add_argument('--lo', type=lambda x: int(x, 0), default=0x40)
    ap.add_argument('--hi', type=lambda x: int(x, 0), default=0xfffff)
    a = ap.parse_args()
    O = MP.Img(a.old, a.oldpkl); N = MP.Img(a.new, a.newpkl)
    man = MV.load_man(a.mod)
    srcs = [a.src] if a.src else MV.sources(a.mod)
    consts = []
    for p in srcs:
        for m in CONST.finditer(open(p, encoding='utf-8', errors='replace').read()):
            v = int(m.group(2), 16)
            if a.lo <= v <= a.hi and not m.group(1).endswith(('_RVA', '_SIG', '_LEN')) and 'RVA' not in m.group(1):
                consts.append((m.group(1), v, os.path.basename(p)))
    vals = {v for _, v, _ in consts}
    # 함수 쌍: offsets/<mod>.json 구지문(구 RVA) ↔ 매니페스트 현재 값(신 RVA)
    fp = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'offsets', a.mod + '.json')
    base = json.load(open(fp, encoding='utf-8'))['fns'] if os.path.isfile(fp) else {}
    cur = {e['name']: e for e in man['entries'] if not e.get('ignore')}
    fns = []
    for oldv, rec in base.items():
        e = cur.get(rec['name'])
        if not e or e.get('unresolved'):
            continue
        of, nf = int(oldv, 16), int(e['value'], 16)
        if of in O.fn and nf in N.fn and of != nf:
            fns.append((rec['name'], of, nf))
    evid = collections.defaultdict(lambda: collections.defaultdict(collections.Counter))
    for name, of, nf in fns:
        ow, nw = FM.walk(O, of), FM.walk(N, nf)
        pairs = dict(FM.align(ow, nw))
        for i, (oa, om, oo, ok, od) in enumerate(ow):
            for ob, ov in od:
                if ov not in vals:
                    continue
                j = pairs.get(i)
                if j is None:
                    evid[ov][name]['?'] += 1; continue
                na, nm, no, nk, nd = nw[j]
                if om != nm or len(nd) != len(od):
                    evid[ov][name]['?'] += 1; continue
                nv = nd[[b for b, _ in od].index(ob)][1] if len(od) == len(nd) else None
                evid[ov][name]['=' if nv == ov else ('->%#x' % nv)] += 1
    print('%-22s %-8s %s' % ('상수', '값', '함수별 증거 [=불변 / ->신값 / ?미정렬]  → 판정'))
    for name, v, f in consts:
        per = evid.get(v, {})
        if not per:
            print('%-22s %-8s (훅 함수 안 사용 없음 — 다른 근거 필요)' % (name, hex(v))); continue
        tot = collections.Counter()
        parts = []
        for fn, c in per.items():
            tot.update(c)
            parts.append('%s[%s]' % (fn.replace('_RVA', ''), ' '.join('%s%d' % (k, n) for k, n in c.most_common())))
        same = tot.get('=', 0); unk = tot.get('?', 0)
        moves = [(k, n) for k, n in tot.items() if k.startswith('->')]
        if moves:
            mk, mn = max(moves, key=lambda kv: kv[1])
            verdict = ('★이동후보 %s (x%d, 불변 %d)' % (mk[2:], mn, same)) if mn >= same else ('불변 우세(%d) · 이동 %s x%d' % (same, mk[2:], mn))
        else:
            verdict = '불변(%d)' % same if same else '근거없음(미정렬 %d)' % unk
        print('%-22s %-8s %s  → %s' % (name, hex(v), ' '.join(parts)[:150], verdict))


if __name__ == '__main__':
    main()
