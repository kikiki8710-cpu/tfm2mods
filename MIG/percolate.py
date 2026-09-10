#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""percolate.py - **씨앗 기반 그래프 정합(seeded graph matching / percolation)** 실험.

무엇을 하는가:
  이미 확정된 exe함수↔DLL심볼 짝(씨앗)을 출발점으로, **호출 관계**를 타고
  미확정 exe 함수에 DLL 심볼(=이름)을 전파할 수 있는지 **정직하게 측정**한다.
  구현이 목적이 아니라 **"이 데이터에서 임계밀도를 넘는가"에 예/아니오**를 내는 게 목적이다.

핵심 개념 - 증인(witness):
  이미 맞춰진 짝 (E', D') 가 있을 때,
    E' 를 부르는 E 와 D' 를 부르는 D  ->  (E,D) 에 **피호출 증인** +1
    E' 가 부르는 E 와 D' 가 부르는 D  ->  (E,D) 에 **호출 증인** +1
  증인이 임계 r 이상 모이면 그 짝을 확정하고, 확정된 짝이 다시 증인을 뿌린다.

★측정 프로토콜(이 파일의 진짜 알맹이)
  1) hold-out : 씨앗의 30% 를 떼고 나머지로만 전파 -> 뗀 것이 복원되는지. 시드 5회 이상.
     복원율(recall) / 정확도(precision) / **오답률** 을 낸다. 오답률이 1순위 지표.
  2) ★함정 시험(decoy) : 뗀 씨앗의 **정답 D 를 후보 풀에서 아예 제거**하고 돌린다.
     그러면 그 E 들에는 **정답이 존재하지 않으므로**, 이름이 붙는 족족 오답이다.
     exe 노드 12만 개 중 대다수는 DLL 에 짝이 없다(부분그래프 매칭). hold-out 정확도는
     "정답이 있는 모집단"에서만 잰 수치라 그대로 믿으면 안 되고, 이 시험이 그 반쪽을 잰다.

비교는 **완전일치**로만 한다(부분문자열 금지). 판정 단위는 DLL RVA -
  링커 ICF 로 한 주소에 심볼이 여러 개 접힐 수 있어 주소 일치가 옳은 단위이고,
  '정답 망글심볼이 그 주소의 심볼 집합에 들어 있나'도 함께 낸다.

기존 파일은 하나도 건드리지 않는다. 입력 캐시는 `_perc_prep.py` 가 만든다.
"""
import argparse
import io
import json
import os
import random
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

HERE = os.path.dirname(os.path.abspath(__file__))
CACHE = os.path.join(HERE, '_perc_cache.json')
OUT = os.path.join(HERE, 'percolate.json')
MIN_FP = 5          # 지문 원소가 이보다 적으면 지문 판정을 하지 않는다(원리적 판별 불가)


# ══════════════════════════════════════════════ 데이터 적재
class Data(object):
    pass


def load():
    d = Data()
    c = json.load(io.open(CACHE, encoding='utf-8'))
    d.succ_e = {int(k): v for k, v in c['cg_e'].items()}
    d.succ_d = {int(k): v for k, v in c['cg_d'].items()}

    def rev(g):
        out = {}
        for a, bs in g.items():
            for b in bs:
                out.setdefault(b, []).append(a)
        return out
    d.pred_e, d.pred_d = rev(d.succ_e), rev(d.succ_d)
    d.dsym = {k: tuple(v) for k, v in c['dsym'].items()}       # 망글 -> (rva, size)
    d.src_of = c['src_of']
    d.rva_of = {k: v[0] for k, v in d.dsym.items()}
    d.syms_at = {}                                             # dll rva -> [망글...] (ICF 접힘)
    for n, r in d.rva_of.items():
        d.syms_at.setdefault(r, []).append(n)
    d.dcand = set(d.syms_at)                                   # ★후보는 rlib 심볼 주소로 한정
    d.seeds = {int(k): v for k, v in c['seeds'].items()}       # exe rva -> {m,grade,src,from_cg}
    d.fp_e = {int(k): set(v) for k, v in c['fp_e'].items()}
    d.fp_d = {k: set(v) for k, v in c['fp_d'].items()}
    d.size_e = {int(k): v for k, v in c['size_e'].items()}
    # dll 주소 단위 지문(ICF 접힌 심볼은 같은 코드 = 같은 지문)
    d.fp_dr = {}
    for n, r in d.rva_of.items():
        if n in d.fp_d and r not in d.fp_dr:
            d.fp_dr[r] = d.fp_d[n]
    d.pairs = {}                                               # exe rva -> dll rva (씨앗 정답)
    for e, v in d.seeds.items():
        d.pairs[e] = d.rva_of[v['m']]
    return d


def _two(g, n, cap2, lim):
    """n 의 **2홉 이웃**(중간 노드 차수 <= cap2 인 경로만). lim 을 넘으면 포기한다."""
    out = set()
    for x in g.get(n, ()):
        gx = g.get(x, ())
        if len(gx) > cap2:
            continue
        out.update(gx)
        if len(out) > lim:
            return ()
    out.discard(n)
    return out


def jac(a, b):
    if not a or not b:
        return 0.0
    i = len(a & b)
    return i / float(len(a) + len(b) - i)


# ══════════════════════════════════════════════ 전파 본체
def percolate(d, train, r=3, cap=40, margin=True, mutual=True,
              fp_mode='off', fp_min=0.15, block_d=None, max_rounds=40, verbose=False,
              hop2=False, cap2=8, w2=0.5):
    """train = {exe rva: dll rva} 씨앗. 반환 = (매칭 dict, 증인수 dict).

    cap      : 증인 노드의 차수 상한. memcpy/패닉핸들러 같은 **허브**는 증인이 못 된다
               (허브 하나가 수천 쌍에 무차별로 증인을 뿌려 오염시킨다).
    margin   : E 의 1위 후보가 2위보다 **엄격히 나을 때만** 확정(동점이면 미적용).
    mutual   : D 쪽에서도 E 가 1위여야 확정(1:1 규율).
    fp_mode  : 'off' 구조만 | 'score' 동점을 지문으로 가름 | 'gate' 지문이 낮으면 거부
    block_d  : 후보 풀에서 뺄 dll rva 집합(★함정 시험용).
    """
    dcand = d.dcand if not block_d else (d.dcand - block_d)
    succ_e, pred_e, succ_d, pred_d = d.succ_e, d.pred_e, d.succ_d, d.pred_d
    marks = {}          # (E<<32|D) -> [피호출증인, 호출증인, 2홉증인]
    elig = set()
    M = dict(train)
    usedD = set(train.values())

    def tot_of(v):
        return v[0] + v[1] + w2 * v[2]

    def bump(E, D, k):
        key = (E << 32) | D
        v = marks.get(key)
        if v is None:
            v = marks[key] = [0, 0, 0]
        v[k] += 1
        if tot_of(v) >= r:
            elig.add(key)

    def spread(E1, D1):
        # (1) 공통 피호출자 E1/D1 을 통해: E1 의 호출자들 x D1 의 호출자들
        A, B = pred_e.get(E1, ()), pred_d.get(D1, ())
        if len(A) <= cap and len(B) <= cap:
            Bc = [x for x in B if x in dcand]
            for E in A:
                for D in Bc:
                    bump(E, D, 0)
        # (2) 공통 호출자: E1 이 부르는 것들 x D1 이 부르는 것들
        A, B = succ_e.get(E1, ()), succ_d.get(D1, ())
        if len(A) <= cap and len(B) <= cap:
            Bc = [x for x in B if x in dcand]
            for E in A:
                for D in Bc:
                    bump(E, D, 1)
        # (3) ★2홉 증인(선택). E''->x->E1 / D''->y->D1 처럼 **중간 노드를 건너뛴 관계**.
        #     1홉 증인이 희박할 때 쓰는 확장인데, 중간 노드가 인라인 차이로 사라진
        #     경우를 건져낸다. 대신 후보가 폭증하므로 중간 노드 차수를 cap2 로 조인다.
        if hop2:
            A2 = _two(pred_e, E1, cap2, 80)
            B2 = _two(pred_d, D1, cap2, 80)
            if A2 and B2:
                Bc = [x for x in B2 if x in dcand]
                for E in A2:
                    for D in Bc:
                        bump(E, D, 2)

    for E, D in train.items():
        if D in dcand:
            spread(E, D)

    def key_of(E, D, w):
        """확정 우선순위 = (증인수, 지문자카드). 지문은 동점을 가르는 데만 쓴다."""
        if fp_mode == 'off':
            return (w, 0.0)
        fe, fd = d.fp_e.get(E), d.fp_dr.get(D)
        if fe is None or fd is None or len(fe) < MIN_FP or len(fd) < MIN_FP:
            return (w, 0.0)
        return (w, jac(fe, fd))

    def fp_ok(E, D):
        if fp_mode != 'gate':
            return True
        fe, fd = d.fp_e.get(E), d.fp_dr.get(D)
        if fe is None or fd is None or len(fe) < MIN_FP or len(fd) < MIN_FP:
            return True             # ★작은 함수는 지문으로 판정 불가 - 통과시킨다
        return jac(fe, fd) >= fp_min

    rounds = 0
    for _ in range(max_rounds):
        rounds += 1
        # 살아 있는 후보만 남긴다
        live = []
        dead = []
        for key in elig:
            E, D = key >> 32, key & 0xffffffff
            if E in M or D in usedD:
                dead.append(key)
                continue
            v = marks[key]
            w = tot_of(v)
            if w < r:
                continue
            live.append((E, D, w))
        for key in dead:
            elig.discard(key)
        if not live:
            break
        # E 별 1·2위
        bestE, secE = {}, {}
        for E, D, w in live:
            k = key_of(E, D, w)
            b = bestE.get(E)
            if b is None or k > b[0]:
                if b is not None:
                    secE[E] = b[0]
                bestE[E] = (k, D, w)
            else:
                s = secE.get(E)
                if s is None or k > s:
                    secE[E] = k
        cands = []
        for E, (k, D, w) in bestE.items():
            if margin and E in secE and not (k > secE[E]):
                continue                     # 동점 = 못 가른다 -> 미적용(오염보다 낫다)
            if not fp_ok(E, D):
                continue
            cands.append((k, w, E, D))
        if mutual:
            bestD = {}
            for k, w, E, D in cands:
                b = bestD.get(D)
                if b is None or k > b[0]:
                    bestD[D] = (k, w, E)
            cands = [(k, w, E, D) for D, (k, w, E) in bestD.items()]
        cands.sort(key=lambda x: (-x[0][0], -x[0][1], x[2]))
        newly = 0
        for k, w, E, D in cands:
            if E in M or D in usedD:
                continue
            M[E] = D
            usedD.add(D)
            spread(E, D)
            newly += 1
        if verbose:
            print('    라운드 %d: +%d (누적 %d, 후보쌍 %d)' % (rounds, newly, len(M), len(marks)))
        if not newly:
            break
    wit = {}
    for E, D in M.items():
        if E in train:
            continue
        v = marks.get((E << 32) | D)
        wit[E] = (tot_of(v), v[0], v[1], v[2]) if v else (0, 0, 0, 0)
    return M, wit


# ══════════════════════════════════════════════ 명령들
def cmd_census(d, a):
    print('=== 그래프 인구조사 ===')
    print('exe 노드 %d · 엣지 %d / dll 노드 %d · 엣지 %d'
          % (len(d.succ_e), sum(len(v) for v in d.succ_e.values()),
             len(d.succ_d), sum(len(v) for v in d.succ_d.values())))
    print('dll 심볼 주소(후보 풀) %d개 · ICF 로 심볼 2개 이상 접힌 주소 %d개'
          % (len(d.dcand), sum(1 for v in d.syms_at.values() if len(v) > 1)))
    gr = {}
    for v in d.seeds.values():
        gr[v['grade'] + ('/cg' if v['from_cg'] else '')] = gr.get(
            v['grade'] + ('/cg' if v['from_cg'] else ''), 0) + 1
    print('씨앗 %d개 등급별 %s' % (len(d.seeds), sorted(gr.items())))
    dd = {}
    for e, v in d.seeds.items():
        dd.setdefault(d.rva_of[v['m']], []).append(e)
    print('  씨앗이 가리키는 dll 주소 %d개(exe 여러 개가 한 심볼을 가리키는 경우 %d)'
          % (len(dd), sum(1 for v in dd.values() if len(v) > 1)))

    # 차수 분포
    for tag, g, rg in (('exe', d.succ_e, d.pred_e), ('dll', d.succ_d, d.pred_d)):
        outs = sorted((len(v) for v in g.values()), reverse=True)
        ins = sorted((len(v) for v in rg.values()), reverse=True)
        print('  %s 진입차수 상위 %s ... 중앙값 %d / 진출차수 상위 %s ... 중앙값 %d'
              % (tag, ins[:5], ins[len(ins) // 2], outs[:5], outs[len(outs) // 2]))

    # ★증인 밀도: 씨앗 짝 (E,D) 자신이 나머지 씨앗으로부터 몇 개의 증인을 받는가
    print()
    print('=== 증인 밀도(씨앗을 정답으로 두고 실측) ===')
    for cap in (20, 40, 100, 10 ** 9):
        hist = {}
        for E, D in d.pairs.items():
            w = 0
            for x in d.succ_e.get(E, ()):
                if x in d.pairs and len(d.pred_e.get(x, ())) <= cap:
                    dx = d.pairs[x]
                    if dx in d.succ_d.get(D, ()) and len(d.pred_d.get(dx, ())) <= cap:
                        w += 1
            for x in d.pred_e.get(E, ()):
                if x in d.pairs and len(d.succ_e.get(x, ())) <= cap:
                    dx = d.pairs[x]
                    if dx in d.pred_d.get(D, ()) and len(d.succ_d.get(dx, ())) <= cap:
                        w += 1
            hist[min(w, 6)] = hist.get(min(w, 6), 0) + 1
        n = len(d.pairs)
        ge = lambda t: sum(v for k, v in hist.items() if k >= t)  # noqa: E731
        print('  cap=%-10s 증인 0개 %d(%.0f%%) · >=2 %d(%.0f%%) · >=3 %d(%.0f%%) · >=4 %d(%.0f%%)'
              % (cap if cap < 10 ** 8 else '무제한', hist.get(0, 0), 100.0 * hist.get(0, 0) / n,
                 ge(2), 100.0 * ge(2) / n, ge(3), 100.0 * ge(3) / n, ge(4), 100.0 * ge(4) / n))
    print('  ※ 위는 **씨앗 전부가 이미 맞춰진 이상 상태**에서의 증인 수다.')
    print('    hold-out 은 70% 만 쓰므로 실제 증인은 이보다 적다(상한선으로 읽을 것).')


def _split(d, pool, frac, sd, train_all=False):
    """pool 에서 frac 만큼 떼어 test, 나머지는 train.
    train_all=True 면 **pool 밖 씨앗(C등급 등)도 train 에 넣는다** - 실제 운용에 가까운 조건.
    (정답지는 여전히 pool = A/B 등급만 쓴다. C/cg 는 호출그래프로 만든 라벨이라
     정답지로 쓰면 순환논증이 된다.)"""
    rng = random.Random(sd)
    ks = sorted(pool)
    rng.shuffle(ks)
    nt = int(round(len(ks) * frac))
    test = set(ks[:nt])
    train = {e: d.pairs[e] for e in ks[nt:]}
    if train_all:
        for e in d.seeds:
            if e not in test and e not in train:
                train[e] = d.pairs[e]
    return train, test


def weak_fp(d, e):
    """이 exe 함수는 **지문으로는 원리적으로 판별 불가**인가.
    지문 원소가 MIN_FP 미만이면(작은 래퍼 함수) 지문 매칭이 손도 못 댄다 -
    퍼콜레이션이 실제로 값어치를 주장하는 곳이 바로 여기다."""
    fe = d.fp_e.get(e)
    return fe is None or len(fe) < MIN_FP


def _eval(d, M, test, decoy=False):
    got = named = right = 0
    wrongs = []
    sub = {0: [0, 0, 0], 1: [0, 0, 0]}      # 0=지문가능 1=지문불가 : [대상, 이름붙음, 정답]
    for e in sorted(test):
        s = sub[1 if weak_fp(d, e) else 0]
        s[0] += 1
        if e not in M:
            continue
        s[1] += 1
        named += 1
        if decoy:
            wrongs.append((e, M[e]))
            continue
        if M[e] == d.pairs[e]:
            right += 1
            s[2] += 1
        else:
            wrongs.append((e, M[e]))
    got = len(test)
    return got, named, right, wrongs, sub


def cmd_holdout(d, a):
    pool = [e for e, v in d.seeds.items()
            if (not a.exclude_cg or not v['from_cg']) and v['grade'] in a.grades]
    print('=== hold-out 검증 ===')
    print('정답지 씨앗 풀 %d개 (등급 %s%s) · 떼어낼 비율 %.0f%% · 시드 %d회'
          % (len(pool), a.grades, ', 그래프전파 출신 제외' if a.exclude_cg else '',
             100 * a.frac, a.seeds))
    print()
    hdr = ('  %-22s %7s %7s %7s %7s %8s' %
           ('설정', '복원율', '정확도', '오답률', '신규', '실패쌍'))
    print(hdr)
    print('  ' + '-' * (len(hdr) - 2))
    results = []
    for cfg in a.configs:
        r, fpm, mar, mut = cfg['r'], cfg['fp'], cfg['margin'], cfg['mutual']
        acc = [0, 0, 0, 0]        # test, named, right, new(비씨앗 신규매칭)
        subacc = [[0, 0, 0], [0, 0, 0]]
        allw = []
        for sd in range(1, a.seeds + 1):
            train, test = _split(d, pool, a.frac, sd, a.train_all)
            M, wit = percolate(d, train, r=r, cap=a.cap, margin=mar, mutual=mut,
                               fp_mode=fpm, fp_min=a.fp_min, hop2=a.hop2)
            g, n, ok, w, sub = _eval(d, M, test)
            for i in (0, 1):
                for j in (0, 1, 2):
                    subacc[i][j] += sub[i][j]
            acc[0] += g; acc[1] += n; acc[2] += ok
            acc[3] += sum(1 for e in M if e not in train and e not in d.seeds)
            allw += w
        rec = 100.0 * acc[1] / max(acc[0], 1)
        pre = 100.0 * acc[2] / max(acc[1], 1)
        tag = 'r=%d%s%s%s' % (r, '' if fpm == 'off' else '+지문' + fpm,
                              '' if mar else ' -margin', '' if mut else ' -mutual')
        print('  %-22s %6.1f%% %6.1f%% %6.1f%% %7.0f %8d'
              % (tag, rec, pre, 100.0 - pre, acc[3] / float(a.seeds), len(allw)))
        results.append((tag, rec, pre, acc[3] / float(a.seeds), allw))
        if a.split_fp:
            for i, nm in ((0, '  └지문가능'), (1, '  └지문불가')):
                s = subacc[i]
                print('  %-22s %6.1f%% %6.1f%% %6.1f%%   (대상 %d · 이름붙음 %d · 정답 %d)'
                      % (nm, 100.0 * s[1] / max(s[0], 1), 100.0 * s[2] / max(s[1], 1),
                         100.0 - 100.0 * s[2] / max(s[1], 1), s[0] // a.seeds, s[1], s[2]))

    if a.decoy:
        print()
        print('=== ★함정 시험(정답 D 를 후보 풀에서 제거 - 이름이 붙으면 100%% 오답) ===')
        print('  %-22s %10s %10s' % ('설정', '오탐률', '오탐수/회'))
        for cfg in a.configs:
            r, fpm, mar, mut = cfg['r'], cfg['fp'], cfg['margin'], cfg['mutual']
            tot = named = 0
            for sd in range(1, a.seeds + 1):
                train, test = _split(d, pool, a.frac, sd, a.train_all)
                blk = {d.pairs[e] for e in test}
                M, _ = percolate(d, train, r=r, cap=a.cap, margin=mar, mutual=mut,
                                 fp_mode=fpm, fp_min=a.fp_min, block_d=blk, hop2=a.hop2)
                _, n, _, _, _ = _eval(d, M, test, decoy=True)
                tot += len(test); named += n
            tag = 'r=%d%s%s%s' % (r, '' if fpm == 'off' else '+지문' + fpm,
                                  '' if mar else ' -margin', '' if mut else ' -mutual')
            print('  %-22s %9.1f%% %10.1f' % (tag, 100.0 * named / max(tot, 1),
                                              named / float(a.seeds)))

    if a.show_wrong:
        print()
        print('=== 오답 표본(첫 %d개) ===' % a.show_wrong)
        tag, _, _, _, allw = max(results, key=lambda x: x[2] * x[1])
        for e, dr in allw[:a.show_wrong]:
            truth = d.seeds[e]['m']
            print('  0x%09x %6dB' % (0x140000000 + e, d.size_e.get(e, 0)))
            print('      예측 %s' % d.syms_at[dr][0])
            print('      정답 %s' % truth)
    return results


def cmd_baseline(d, a):
    """★대조군: **지문만** 쓰는 매칭(호출그래프 전혀 안 씀)을 같은 hold-out 으로 잰다.
    구조(증인)를 더하는 게 실제로 이득인지 보려면 이 수치가 있어야 한다.
    조건은 전파 쪽보다 **후하게** 준다 - 후보를 rlib 심볼 4천 개로 좁혀 주고,
    '정답이 존재하는 E' 만 물어본다."""
    pool = [e for e, v in d.seeds.items()
            if (not a.exclude_cg or not v['from_cg']) and v['grade'] in a.grades]
    Ds = [(D, f) for D, f in d.fp_dr.items() if len(f) >= MIN_FP]
    print('=== 지문 단독 대조군 (호출그래프 미사용) ===')
    print('  후보 %d개 · 정답지 %d개 · 시드 %d회' % (len(Ds), len(pool), a.seeds))
    print('  %-16s %7s %7s %7s' % ('임계', '복원율', '정확도', '오답률'))
    for thr in (0.0, 0.3, 0.5, 0.7, 0.9):
        g = n = ok = 0
        for sd in range(1, a.seeds + 1):
            _train, test = _split(d, pool, a.frac, sd, a.train_all)
            for E in test:
                g += 1
                fe = d.fp_e.get(E)
                if not fe or len(fe) < MIN_FP:
                    continue
                b1 = b2 = -1.0
                bD = None
                for D, fd in Ds:
                    j = jac(fe, fd)
                    if j > b1:
                        b2, b1, bD = b1, j, D
                    elif j > b2:
                        b2 = j
                if bD is None or b1 < thr or b1 <= b2:
                    continue        # 임계 미달이거나 동점이면 판정하지 않는다
                n += 1
                ok += (bD == d.pairs[E])
        print('  j>=%-13.2f %6.1f%% %6.1f%% %6.1f%%'
              % (thr, 100.0 * n / max(g, 1), 100.0 * ok / max(n, 1),
                 100.0 - 100.0 * ok / max(n, 1)))


def cmd_run(d, a):
    cfg = a.configs[0]
    train = {e: d.pairs[e] for e in d.seeds}
    M, wit = percolate(d, train, r=cfg['r'], cap=a.cap, margin=cfg['margin'],
                       mutual=cfg['mutual'], fp_mode=cfg['fp'], fp_min=a.fp_min,
                       hop2=a.hop2, verbose=True)
    new = [(e, M[e]) for e in sorted(M) if e not in train]
    print('전 씨앗 %d개로 전파 -> 신규 매칭 %d개' % (len(train), len(new)))
    rows = []
    for e, dr in new:
        fe, fd = d.fp_e.get(e), d.fp_dr.get(dr)
        j = jac(fe, fd) if fe and fd and len(fe) >= MIN_FP and len(fd) >= MIN_FP else None
        w = wit.get(e, (0, 0, 0, 0))
        rows.append({'addr': '0x%x' % (0x140000000 + e), 'rva': e,
                     'mangled': d.syms_at[dr][0], 'aliases': d.syms_at[dr][1:],
                     'mod': d.src_of.get(d.syms_at[dr][0], '?'),
                     'witness': w[0], 'w_callee': w[1], 'w_caller': w[2], 'w_2hop': w[3],
                     'jaccard': None if j is None else round(j, 3),
                     'bytes': d.size_e.get(e, 0)})
    rows.sort(key=lambda x: (-x['witness'], -(x['jaccard'] or 0)))
    json.dump({'_설정': {'r': cfg['r'], 'cap': a.cap, 'margin': cfg['margin'],
                        'mutual': cfg['mutual'], 'fp': cfg['fp'], 'fp_min': a.fp_min,
                        'hop2': a.hop2, '씨앗': len(train)},
               '_측정': a.note or '(미기재)',
               '_경고': 'hold-out 정확도는 **정답이 존재하는 모집단**에서만 잰 값이다. '
                       'exe 함수의 96.8%는 DLL 에 짝이 없어 "매칭 없음"이 정답이며, '
                       '그 모집단의 오탐률은 함정 시험(decoy) 수치로 따로 봐야 한다. '
                       '주입 전 사람이 눈으로 확인할 것.',
               'rows': rows},
              io.open(OUT, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
    print('저장: %s' % OUT)
    print()
    print('=== 신규 매칭 상위 20 (사람이 읽고 눈검사할 것) ===')
    for x in rows[:20]:
        print('  %s  증인%4.1f(피%d/호%d/2홉%d) j=%-5s %6dB  %s  [%s]'
              % (x['addr'], x['witness'], x['w_callee'], x['w_caller'], x['w_2hop'],
                 x['jaccard'], x['bytes'], x['mangled'][:66], x['mod']))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('cmd', choices=['census', 'holdout', 'baseline', 'run'])
    ap.add_argument('--r', type=int, default=None, help='단일 임계값으로 돌릴 때')
    ap.add_argument('--cap', type=int, default=40)
    ap.add_argument('--frac', type=float, default=0.30)
    ap.add_argument('--seeds', type=int, default=5)
    ap.add_argument('--fp-min', type=float, default=0.15)
    ap.add_argument('--grades', default='AB')
    ap.add_argument('--exclude-cg', action='store_true', default=True)
    ap.add_argument('--include-cg', dest='exclude_cg', action='store_false')
    ap.add_argument('--decoy', action='store_true')
    ap.add_argument('--hop2', action='store_true', help='2홉 증인도 센다(가중 0.5)')
    ap.add_argument('--note', default='', help='결과 json 에 남길 측정 요약')
    ap.add_argument('--split-fp', action='store_true',
                    help='지문가능/지문불가로 갈라서 낸다(상보성 측정)')
    ap.add_argument('--train-all', action='store_true',
                    help='정답지 밖 씨앗(C등급)도 학습에 쓴다')
    ap.add_argument('--show-wrong', type=int, default=0)
    ap.add_argument('--fp', default='off', choices=['off', 'score', 'gate'])
    ap.add_argument('--no-margin', dest='margin', action='store_false', default=True)
    ap.add_argument('--no-mutual', dest='mutual', action='store_false', default=True)
    ap.add_argument('--sweep', default='', help='"r:fp:margin:mutual,..." 로 설정 여러 개')
    a = ap.parse_args()

    if a.sweep:
        a.configs = []
        for part in a.sweep.split(','):
            f = part.split(':')
            a.configs.append({'r': int(f[0]), 'fp': f[1] if len(f) > 1 else 'off',
                              'margin': (f[2] != '0') if len(f) > 2 else True,
                              'mutual': (f[3] != '0') if len(f) > 3 else True})
    else:
        rs = [a.r] if a.r else [2, 3, 4]
        a.configs = [{'r': x, 'fp': a.fp, 'margin': a.margin, 'mutual': a.mutual} for x in rs]

    d = load()
    {'census': cmd_census, 'holdout': cmd_holdout, 'baseline': cmd_baseline,
     'run': cmd_run}[a.cmd](d, a)


main()
