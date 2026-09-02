#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""offsets.py — ★구조체 오프셋 축을 기계 검사한다 (MIG 의 두 번째 축).

왜 이게 따로 필요한가 (2026-09-02 실사고, 0.5.8):
  `mig_verify check` 가 검사하는 명제는 **"그 주소의 12B 가 그대로인가"** 뿐이다.
  0.5.8 은 RVA 를 전부 재핀했는데도 게임이 크래시했다 — 원인은 주소가 아니라
  **게임 구조체가 0x10 커져서 그 뒤 필드가 전부 밀린 것**이었다.
  모드는 그 필드를 하드코딩으로 읽으므로, 주소가 맞아도 **엉뚱한 필드를 읽는다**.
  README ⑦ 에 "offsets 는 exe 대조 불가 — 별도 확인" 이라고 적혀 있었지만
  **기계 검사가 없으면 그 문장은 지켜지지 않는다.** 그래서 도구로 만든다.

핵심 아이디어
  매니페스트가 이미 "우리가 의존하는 게임 함수"의 주소를 들고 있다.
  그 함수가 **어떤 오프셋들을 쓰는지**(메모리 피연산자 disp 히스토그램)를 지문으로 떠 두면,
  다음 버전에서 같은 함수를 재핀한 뒤 지문을 다시 떠서 diff 하는 것만으로
  **구조체 필드 이동이 자동으로 드러난다.** (0.5.8 실측: ORACLE 함수에서
  +0x110/0x130/0x158/0x198/0x310 → 전부 +0x10 이 이 방식으로 잡혔다.)

사용
  python MIG\offsets.py snap   --exe <현행exe> [MOD ...]   # 지문 채록(마이그 완료 후 1회)
  python MIG\offsets.py check  --exe <신exe>   [MOD ...]   # 재핀 후 대조 -> 이동한 오프셋 전수
  python MIG\offsets.py sources               [MOD ...]   # 이동분을 하드코딩한 소스 위치 지목
종료코드: 0=클린 / 1=이동 감지(=그 회차 작업 목록)
"""
import sys, os, re, json, argparse, collections

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
import mig_verify as MV                      # noqa: E402
from repin import Img                        # noqa: E402
import capstone                              # noqa: E402

FPDIR = os.path.join(MV.MIGD, 'offsets')
DISP = re.compile(r'\+ 0x([0-9a-f]{2,4})\]')
_md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
# 지문에서 뺄 값: 스택 프레임(rbp/rsp 상대)은 구조체가 아니다.
STACKREG = ('rbp', 'rsp')


def fp_path(mod):
    return os.path.join(FPDIR, mod + '.json')


def fingerprint(img, fn):
    """함수 본문의 메모리 피연산자 disp 히스토그램(스택 상대 제외)."""
    size = img.fn[fn]['size']
    code = img.read(fn, size)
    if not code:
        return {}
    h = collections.Counter()
    for ins in _md.disasm(code, fn):
        ops = ins.op_str
        for m in DISP.finditer(ops):
            seg = ops[max(0, m.start() - 12):m.start()]
            if any(r in seg for r in STACKREG):
                continue
            h['0x%x' % int(m.group(1), 16)] += 1
    return dict(h)


def live_fn_entries(mod, img):
    """매니페스트 엔트리 중 '현재 값이 함수 시작인 것' = 지문 대상."""
    man = MV.load_man(mod)
    if not man:
        return []
    out = []
    for e in man['entries']:
        if e.get('ignore'):
            continue
        v = int(e['value'], 16)
        if v in img.fn:
            out.append((e, v))
    return out


def cmd_snap(mods, a):
    img = Img(a.exe, a.pkl)
    os.makedirs(FPDIR, exist_ok=True)
    for mod in mods:
        ents = live_fn_entries(mod, img)
        if not ents:
            continue
        doc = {'mod': mod, 'game_ver': a.ver, 'fns': {}}
        for e, v in ents:
            doc['fns'][e['value']] = {'name': e['name'],
                                      'size': img.fn[v]['size'],
                                      'offs': fingerprint(img, v)}
        json.dump(doc, open(fp_path(mod), 'w', encoding='utf-8'),
                  ensure_ascii=False, indent=1)
        print('%-26s 함수 %3d개 지문 채록 -> offsets/%s.json' % (mod, len(ents), mod))


def shift_pairs(old, new):
    """사라진 오프셋 ↔ 생긴 오프셋을 델타별로 짝짓는다(+0x10 같은 균일 이동 탐지)."""
    o = sorted(int(x, 16) for x in set(old) - set(new))
    n = sorted(int(x, 16) for x in set(new) - set(old))
    by_delta = collections.defaultdict(list)
    for x in o:
        for y in n:
            if x != y:
                by_delta[y - x].append((x, y))
    best = max(by_delta.items(), key=lambda kv: len(kv[1])) if by_delta else None
    return o, n, best


def cmd_check(mods, a):
    img = Img(a.exe, a.pkl)
    shifted = {}                      # mod -> [(fn_name, delta, [(old,new),...])]
    bad = 0
    for mod in mods:
        p = fp_path(mod)
        if not os.path.isfile(p):
            print('%-26s !지문 없음 — 먼저 snap 할 것' % mod)
            continue
        doc = json.load(open(p, encoding='utf-8'))
        man = MV.load_man(mod)
        cur = {e['name']: e for e in man['entries'] if not e.get('ignore')}
        rows = []
        for oldval, rec in doc['fns'].items():
            e = cur.get(rec['name'])
            if not e:
                continue
            v = int(e['value'], 16)
            if v not in img.fn:
                rows.append((rec['name'], None, None, '재핀값이 함수시작 아님 %s' % e['value']))
                continue
            now = fingerprint(img, v)
            o, n, best = shift_pairs(rec['offs'], now)
            if not o and not n:
                continue
            delta, pairs = (best[0], best[1]) if best else (None, [])
            rows.append((rec['name'], delta, pairs,
                         '사라짐 %s / 생김 %s' % ([hex(x) for x in o][:8], [hex(x) for x in n][:8])))
        if rows:
            bad += len(rows)
            shifted[mod] = rows
            print('%-26s ★오프셋 변화 %d개 함수' % (mod, len(rows)))
            for name, delta, pairs, note in rows:
                d = ('Δ%+#x x%d' % (delta, len(pairs))) if delta is not None else '-'
                print('    %-26s %-12s %s' % (name, d, note))
        else:
            print('%-26s 오프셋 지문 일치' % mod)
    if shifted:
        json.dump({m: [{'fn': r[0], 'delta': r[1],
                        'pairs': [[hex(x), hex(y)] for x, y in (r[2] or [])]}
                       for r in rows] for m, rows in shifted.items()},
                  open(os.path.join(FPDIR, '_shifts.json'), 'w', encoding='utf-8'),
                  ensure_ascii=False, indent=1)
        print('\n-> %s (sources 하위명령이 이 파일을 읽어 소스 위치를 지목한다)'
              % os.path.join(FPDIR, '_shifts.json'))
    return bad


HEXLIT = re.compile(r'0x([0-9a-fA-F]{2,4})\b')


def cmd_sources(mods, a):
    """이동한 오프셋 값을 소스에 하드코딩한 자리를 전부 찾아준다(= 그 회차 수정 목록)."""
    sp = os.path.join(FPDIR, '_shifts.json')
    if not os.path.isfile(sp):
        print('_shifts.json 없음 — 먼저 check 를 돌릴 것')
        return 1
    sh = json.load(open(sp, encoding='utf-8'))
    # ★그 모드 **자신의** 함수가 움직인 오프셋만 본다.
    #   (모드 간 교차 매칭은 0x30·0x64 같은 흔한 수 때문에 노이즈만 낸다 — 09-02 실측 898건)
    per_mod = {}
    for mod, rows in sh.items():
        d = per_mod.setdefault(mod, {})
        for r in rows:
            for o, n in r['pairs']:
                d.setdefault(int(o, 16), []).append((r['fn'], n))
    floor = a.floor
    print('모드별 이동 오프셋(자기 함수 기준) — 소스 하드코딩 위치 (>= %#x, 덧셈 문맥만):' % floor)
    hit = 0
    for mod in mods:
        moved = per_mod.get(mod)
        if not moved:
            continue
        man = MV.load_man(mod)
        exclude = man.get('exclude', []) if man else []
        for path in MV.sources(mod):
            rel = os.path.relpath(path, os.path.join(MV.ROOT, mod)).replace('\\', '/')
            if any(rel.startswith(x) for x in exclude):
                continue
            raw = open(path, 'rb').read().decode('utf-8', 'replace')
            for i, line in enumerate(MV.mask_code(raw).split('\n')):
                for m in HEXLIT.finditer(line):
                    v = int(m.group(1), 16)
                    if v not in moved or v < floor:
                        continue
                    # 오프셋다운 문맥만: `+ 0x..` / `add(0x..)` / `const *OFF* = 0x..`
                    pre = line[max(0, m.start() - 24):m.start()]
                    ctx = ('+' in pre or 'add(' in pre or
                           re.search(r'(OFF|OFFSET|SLOT|STRIDE|_OF)\w*\s*(:\s*\w+)?\s*=\s*$', pre))
                    if not ctx:
                        continue
                    fn, new = moved[v][0]
                    print('  %-22s %-28s:%-5d 0x%-4x -> 0x%-4x  (근거 함수 %s)'
                          % (mod, rel, i + 1, v, int(new, 16), fn))
                    hit += 1
    print('총 %d곳. ⚠같은 수가 다른 구조체를 뜻할 수 있다 — 함수 문맥으로 확인하고 고칠 것.' % hit)
    return 0


if __name__ == '__main__':
    ap = argparse.ArgumentParser()
    ap.add_argument('cmd', choices=['snap', 'check', 'sources'])
    ap.add_argument('mods', nargs='*')
    ap.add_argument('--exe', default=MV.GAME_EXE)
    ap.add_argument('--pkl', default=os.path.join(MV.ROOT, '_fnidx_058.pkl'))
    ap.add_argument('--ver', default=MV.GAME_VER)
    ap.add_argument('--floor', type=lambda x:int(x,0), default=0x40, help='이 값 미만 오프셋은 노이즈로 보고 무시')
    a = ap.parse_args()
    mods = a.mods or list(MV.MODS)
    if a.cmd == 'snap':
        cmd_snap(mods, a)
        sys.exit(0)
    elif a.cmd == 'check':
        sys.exit(1 if cmd_check(mods, a) else 0)
    else:
        sys.exit(cmd_sources(mods, a))
