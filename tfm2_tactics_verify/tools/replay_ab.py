"""다시보기 A/B 지문 도구 (2026-10-22).

다시보기는 시드 재시뮬레이션이라 같은 조건이면 판단이 똑같이 흐른다 → 게이트별 호출 수·비교 분포·차이 히스토그램이
**숫자까지 같아야** 한다. 노브를 바꾼 다시보기에서 이 지문이 갈리면 그 노브가 판단을 바꾼 것이다.
요약 파일은 프로세스 시작 이후 누적이라, 다시보기 직전·직후 스냅샷의 **차이**를 한 회 지문으로 쓴다.
  snap  <label> <name>            요약(tactics_verify_summary_<label>.json)을 snaps/<name>.json 으로 복사
  delta <before> <after> <name>   두 스냅샷 차이 → deltas/<name>.json (게이트별 hits·cmp·hist 차)
  cmp   <deltaA> <deltaB>         두 지문 비교 — 게이트별 같음/다름, 다르면 어디가 얼마나
폴더 기본 = 이 스크립트와 같은 곳의 ab/ (또는 --dir)
"""
import argparse, json, os, shutil, sys

LOGS = r'E:\claude\tfm2\logs\game_mods\tfm2_tactics_verify'


def load(p): return json.load(open(p, encoding='utf-8'))


def gate_vec(g):
    out = {}
    for side in ('fg', 'bg'):
        c = g.get(side) or {}
        out[side] = {'hits': c.get('hits', 0), 'cmp': list(c.get('cmp', [0] * 5)), 'hist': {int(k): v for k, v in c.get('hist', [])}}
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('cmd'); ap.add_argument('args', nargs='*'); ap.add_argument('--dir', default=None)
    a = ap.parse_args()
    d = a.dir or os.path.join(os.path.dirname(os.path.abspath(__file__)), 'ab')
    os.makedirs(os.path.join(d, 'snaps'), exist_ok=True); os.makedirs(os.path.join(d, 'deltas'), exist_ok=True)
    if a.cmd == 'snap':
        label, name = a.args
        src = os.path.join(LOGS, f'tactics_verify_summary_{label}.json')
        dst = os.path.join(d, 'snaps', f'{name}.json')
        shutil.copy2(src, dst)
        j = load(dst)
        print(name, 'written', j['written_unix_ms'], 'frame', j['frame'], 'bg합', sum(g['bg']['hits'] for g in j['gates']))
    elif a.cmd == 'delta':
        b, af, name = a.args
        B = {g['id']: gate_vec(g) for g in load(os.path.join(d, 'snaps', f'{b}.json'))['gates']}
        A = {g['id']: gate_vec(g) for g in load(os.path.join(d, 'snaps', f'{af}.json'))['gates']}
        out = {}
        for gid in A:
            x, y = B.get(gid), A[gid]
            dd = {}
            for side in ('fg', 'bg'):
                hx = x[side]['hist'] if x else {}
                h = {k: v - hx.get(k, 0) for k, v in y[side]['hist'].items() if v - hx.get(k, 0)}
                dd[side] = {'hits': y[side]['hits'] - (x[side]['hits'] if x else 0),
                            'cmp': [p - q for p, q in zip(y[side]['cmp'], x[side]['cmp'] if x else [0] * 5)], 'hist': h}
            out[gid] = dd
        json.dump(out, open(os.path.join(d, 'deltas', f'{name}.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
        tot = sum(v['fg']['hits'] + v['bg']['hits'] for v in out.values())
        print(name, '게이트', len(out), '총 호출', tot)
    elif a.cmd == 'cmp':
        x, y = (load(os.path.join(d, 'deltas', f'{n}.json')) for n in a.args)
        same = diff = 0
        rows = []
        for gid in sorted(set(x) | set(y)):
            p, q = x.get(gid), y.get(gid)
            if p == q:
                same += 1; continue
            diff += 1
            hp = (p['fg']['hits'] + p['bg']['hits']) if p else 0
            hq = (q['fg']['hits'] + q['bg']['hits']) if q else 0
            cp = [s + t for s, t in zip(p['fg']['cmp'], p['bg']['cmp'])] if p else None
            cq = [s + t for s, t in zip(q['fg']['cmp'], q['bg']['cmp'])] if q else None
            rows.append(f'  다름 {gid:48s} 호출 {hp} → {hq} · 작/같/큼 {cp[:3] if cp else "-"} → {cq[:3] if cq else "-"}')
        print(f'같음 {same} · 다름 {diff}  ({a.args[0]} vs {a.args[1]})')
        print('\n'.join(rows))
    else:
        sys.exit(__doc__)


if __name__ == '__main__':
    main()
