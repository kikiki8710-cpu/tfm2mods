#!/usr/bin/env python3
"""aimap.py — game_ai 계층 **상관관계 지도**를 만든다.

왜: "몇 개 남았나" 를 함수 수로 세면 답이 안 나온다(640개 중 대부분은 다른 결정의 하부다).
    필요한 것은 **어느 결정이 어느 코드를 소유하는가** 다. 이 도구는 두 층을 합쳐 그 지도를 만든다.

    ① RVA 층 = `MIG\\decomp\\<ver>\\**.md`(aidump 스캐폴딩)의 `## \`0xRVA\`` + `**콜리**` 표.
       크기·명령수·소속 파일이 있고 **직접 call 만** 담는다(간접/vtable 은 빠진다 = 하한).
    ② 심볼 층 = SDK IR `_gaibc\\*.ll` 의 `call/invoke @심볼`. 이름이 살아 있고 크레이트 내부는 완전하다.
       RVA 는 없다. 모듈(경로) 단위 집계에 쓴다.

출력:
    MIG\\AIMAP.md    — 사람이 읽는 지도(모듈 표 · 결정 지점 소유 범위 · 진입점 · 미소유 허브)
    MIG\\aimap.json  — 기계용(graph, closure, 통계)

사용:
    python MIG\\aimap.py [--ver 0.5.8] [--ir C:\\tfm2mods\\_gaibc]
"""
import io, json, os, re, sys
from collections import defaultdict

DEC = r'C:\tfm2mods\MIG\decomp'
GEN = r'C:\tfm2mods\tfm2_ai_adjust\src\judge\gen_fns.rs'
JUDGE_SRC = r'C:\tfm2mods\tfm2_ai_adjust\src\judge'


def load_rva_layer(ver):
    """스캐폴딩 → (info, callees). info[rva] = dict(bytes, ins, mod, lines)"""
    root = os.path.join(DEC, ver)
    info, callees = {}, {}
    for dp, _, files in os.walk(root):
        for fn in files:
            if not fn.endswith('.md') or fn == 'INDEX.md':
                continue
            rel = os.path.relpath(os.path.join(dp, fn), root).replace('\\', '/')[:-3]
            t = io.open(os.path.join(dp, fn), encoding='utf-8', errors='ignore').read()
            cur = None
            for line in t.split('\n'):
                m = re.match(r'^## `(0x[0-9a-f]+)`', line)
                if m:
                    cur = m.group(1)
                    info[cur] = {'bytes': 0, 'ins': 0, 'mod': rel, 'lines': []}
                    callees[cur] = set()
                    continue
                if not cur:
                    continue
                m = re.match(r'^\| RVA \| `0x[0-9a-f]+` ~ `0x[0-9a-f]+` \((\d+) B\) \|', line)
                if m:
                    info[cur]['bytes'] = int(m.group(1)); continue
                m = re.match(r'^\| 명령 수 \| (\d+) \|', line)
                if m:
                    info[cur]['ins'] = int(m.group(1)); continue
                m = re.match(r'^\| 원본 행 \| ([0-9, ]+) \|', line)
                if m:
                    info[cur]['lines'] = [int(x) for x in m.group(1).split(',')]; continue
                if line.startswith('**콜리**'):
                    callees[cur] |= set(re.findall(r'`(0x[0-9a-f]+)`', line))
    for k in callees:                      # AI 계층 밖(런타임/CRT) 호출은 버린다
        callees[k] &= set(info)
    return info, callees


def load_ir_layer(irdir):
    """IR → 모듈(파일) 단위 심볼 호출 집계. mod_edges[(src_mod, dst_mod)] = 건수"""
    if not os.path.isdir(irdir):
        return {}, {}
    sym_mod, edges = {}, defaultdict(int)
    # DISubprogram: name + file → 심볼이 어느 소스 파일 것인지
    fre = re.compile(r'!DIFile\(filename: "game-ai\\\\src\\\\([^"]+)"')
    for fn in sorted(os.listdir(irdir)):
        if not fn.endswith('.ll'):
            continue
        path = os.path.join(irdir, fn)
        cur = None
        with io.open(path, encoding='utf-8', errors='ignore') as f:
            for line in f:
                if line.startswith('define'):
                    m = re.search(r'@(_RNv[A-Za-z0-9_]+)', line)
                    cur = m.group(1) if m else None
                    if cur:
                        mm = re.search(r'7game_ai(?:Nt)?[0-9]+([a-z_0-9]+)', cur)
                        sym_mod[cur] = mm.group(1) if mm else '?'
                    continue
                if cur and ('call ' in line or 'invoke ' in line):
                    for c in re.findall(r'@(_RNv[A-Za-z0-9_]+)', line):
                        mm = re.search(r'7game_ai(?:Nt)?[0-9]+([a-z_0-9]+)', c)
                        dst = mm.group(1) if mm else None
                        if dst and sym_mod.get(cur):
                            edges[(sym_mod[cur], dst)] += 1
    return sym_mod, dict(edges)


def hooked_set(info):
    src = io.open(GEN, encoding='utf-8').read()
    return set(x.lower() for x in re.findall(r'rva: (0x[0-9a-f]+)', src)) & set(info)


def mentioned_set():
    men = set()
    for dp, _, files in os.walk(JUDGE_SRC):
        for f in files:
            if f.endswith('.rs'):
                men |= set(x.lower() for x in re.findall(r'0x[0-9a-f]{6,7}',
                           io.open(os.path.join(dp, f), encoding='utf-8', errors='ignore').read()))
    return men


def closure(seeds, callees):
    seen, stack = set(seeds), list(seeds)
    while stack:
        n = stack.pop()
        for c in callees.get(n, ()):
            if c not in seen:
                seen.add(c); stack.append(c)
    return seen


def main(ver='0.5.8', irdir=r'C:\tfm2mods\_gaibc'):
    info, callees = load_rva_layer(ver)
    hooked = hooked_set(info)
    men = mentioned_set()
    rdeg = defaultdict(int)
    for s, ds in callees.items():
        for d in ds:
            rdeg[d] += 1
    tb = sum(v['bytes'] for v in info.values())
    own = closure(hooked, callees)
    ob = sum(info[r]['bytes'] for r in own)

    # 모듈 집계
    mods = defaultdict(lambda: {'n': 0, 'b': 0, 'hook': 0, 'own': 0, 'ownb': 0, 'repro': 0})
    for r, v in info.items():
        m = mods[v['mod']]
        m['n'] += 1; m['b'] += v['bytes']
        if r in hooked: m['hook'] += 1
        if r in own: m['own'] += 1; m['ownb'] += v['bytes']
        if r in hooked or r in men: m['repro'] += 1

    # 결정 지점별 소유 범위
    per = []
    for h in sorted(hooked, key=lambda r: -info[r]['bytes']):
        c = closure({h}, callees)
        per.append({'rva': h, 'mod': info[h]['mod'], 'bytes': info[h]['bytes'],
                    'reach_n': len(c), 'reach_b': sum(info[x]['bytes'] for x in c),
                    'mods': sorted({info[x]['mod'] for x in c})})

    entries = sorted([r for r in info if rdeg[r] == 0], key=lambda r: -info[r]['bytes'])
    hubs = sorted([r for r in info if r not in own and rdeg[r] >= 2],
                  key=lambda r: (-rdeg[r], -info[r]['bytes']))
    gaps = sorted([r for r in own if r not in hooked and r not in men],
                  key=lambda r: -info[r]['ins'])

    sym_mod, ir_edges = load_ir_layer(irdir)
    mod_out = defaultdict(int)
    for (a, b), n in ir_edges.items():
        if a != b:
            mod_out[(a, b)] += n

    L = []
    L.append('# game_ai 계층 상관관계 지도 (게임 %s · 자동생성 `MIG\\aimap.py`)\n' % ver)
    L.append('> ⚠호출 그래프는 **직접 call 만** 담는다(Ghidra 콜리 표). vtable·간접 호출은 빠져 있어 '
             '"도달 가능" 수치는 **하한**이다. 모듈 간 호출은 SDK IR 심볼로 따로 집계했다.\n')
    L.append('## 1. 전체\n')
    L.append('| | 함수 | 바이트 | 비중 |')
    L.append('|---|---|---|---|')
    L.append('| AI 계층 전체 | %d | %s | 100%% |' % (len(info), f'{tb:,}'))
    L.append('| judge 훅 = 결정 지점 | %d | %s | %.1f%% |'
             % (len(hooked), f"{sum(info[r]['bytes'] for r in hooked):,}",
                100 * sum(info[r]['bytes'] for r in hooked) / tb))
    L.append('| 결정 지점에서 도달 = **모드가 소유해야 할 코드** | %d | %s | %.1f%% |'
             % (len(own), f'{ob:,}', 100 * ob / tb))
    L.append('| 도달 밖(다른 결정의 하부·미발화) | %d | %s | %.1f%% |'
             % (len(info) - len(own), f'{tb - ob:,}', 100 * (tb - ob) / tb))
    L.append('')
    L.append('## 2. 모듈별 (소유 바이트 순)\n')
    L.append('| 모듈 | 함수 | 바이트 | 훅 | 소유 | 소유B | 재현흔적 |')
    L.append('|---|---|---|---|---|---|---|')
    for m, v in sorted(mods.items(), key=lambda x: -x[1]['ownb'])[:28]:
        L.append('| `%s` | %d | %s | %d | %d | %s | %d |'
                 % (m, v['n'], f"{v['b']:,}", v['hook'], v['own'], f"{v['ownb']:,}", v['repro']))
    L.append('')
    L.append('## 3. 결정 지점이 소유하는 범위\n')
    L.append('| 결정 지점 | 모듈 | 자기 B | 도달 함수 | 도달 B | 걸치는 모듈 |')
    L.append('|---|---|---|---|---|---|')
    for p in per:
        L.append('| `%s` | `%s` | %s | %d | %s | %s |'
                 % (p['rva'], p['mod'], f"{p['bytes']:,}", p['reach_n'], f"{p['reach_b']:,}",
                    ', '.join('`%s`' % x for x in p['mods'][:4]) + (' …' if len(p['mods']) > 4 else '')))
    L.append('')
    L.append('## 4. 진입점 (아무도 안 부르는 함수 = 상위 결정 후보) — 상위 20\n')
    L.append('| RVA | 모듈 | 바이트 | 명령 | 훅? |')
    L.append('|---|---|---|---|---|')
    for r in entries[:20]:
        L.append('| `%s` | `%s` | %s | %d | %s |'
                 % (r, info[r]['mod'], f"{info[r]['bytes']:,}", info[r]['ins'],
                    '✅' if r in hooked else ''))
    L.append('')
    L.append('## 5. 미소유 허브 (여러 곳에서 불리는데 소유 밖) — 상위 20\n')
    L.append('| RVA | 모듈 | 피호출 | 바이트 |')
    L.append('|---|---|---|---|')
    for r in hubs[:20]:
        L.append('| `%s` | `%s` | %d | %s |' % (r, info[r]['mod'], rdeg[r], f"{info[r]['bytes']:,}"))
    L.append('')
    L.append('## 6. 소유 범위 안인데 소스에 흔적 없음 (= 진짜 구멍 후보)\n')
    if not gaps:
        L.append('없음.')
    else:
        L.append('| RVA | 모듈 | 바이트 | 명령 |')
        L.append('|---|---|---|---|')
        for r in gaps:
            L.append('| `%s` | `%s` | %s | %d |' % (r, info[r]['mod'], f"{info[r]['bytes']:,}", info[r]['ins']))
    L.append('')
    if mod_out:
        L.append('## 7. 모듈 간 호출 (SDK IR 심볼 기준, 상위 25)\n')
        L.append('| 호출자 모듈 | 피호출 모듈 | 건수 |')
        L.append('|---|---|---|')
        for (a, b), n in sorted(mod_out.items(), key=lambda x: -x[1])[:25]:
            L.append('| `%s` | `%s` | %d |' % (a, b, n))
        L.append('')
    io.open(r'C:\tfm2mods\MIG\AIMAP.md', 'w', encoding='utf-8').write('\n'.join(L) + '\n')
    io.open(r'C:\tfm2mods\MIG\aimap.json', 'w', encoding='utf-8').write(json.dumps(
        {'info': info, 'callees': {k: sorted(v) for k, v in callees.items()},
         'hooked': sorted(hooked), 'own': sorted(own), 'per_decision': per,
         'entries': entries[:60], 'hubs': hubs[:60], 'gaps': gaps,
         'ir_mod_edges': {'%s>%s' % k: v for k, v in mod_out.items()}}, ensure_ascii=False))
    print('AIMAP.md / aimap.json 생성 · 함수 %d · 소유 %d(%.1f%%) · 진입점 %d · 허브 %d · 구멍 %d'
          % (len(info), len(own), 100 * ob / tb, len(entries), len(hubs), len(gaps)))


if __name__ == '__main__':
    a = sys.argv[1:]
    ver = a[a.index('--ver') + 1] if '--ver' in a else '0.5.8'
    ird = a[a.index('--ir') + 1] if '--ir' in a else r'C:\tfm2mods\_gaibc'
    main(ver, ird)
