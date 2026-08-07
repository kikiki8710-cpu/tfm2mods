# -*- coding: utf-8 -*-
"""★sites.py 가 못 보는 패치 호출 형태까지 **전수** 훑어 0.5.4 실바이트와 대조한다.

sites.py 는 `p!` / `pany!` 와 `for a in [..]` 형태만 파싱한다. 그래서
  ① 튜플 루프  `for (ca, mv, pre) in [(0xA, 0xB, &[..]), ..] { p!(base + ca, ..); }`
  ② 생 호출    `patch_imm_bytes(base + 0xRVA, &[..], off, w, ..)`
  ③ 토글       `patch_toggle_bytes(base + 0xRVA, &[..], &[..], ..)`
가 통째로 빠진다 — 0.5.3 주소가 그대로 남아도 검증에 안 잡힌다.

⚠남은 0.5.3 주소는 두 가지로 갈린다:
  · prefix 불일치 → 조용히 skip = **노브만 죽음**(크래시 없음)
  · prefix 우연 일치 → orig_guard 는 표에 없는 RVA 를 통과시키므로 **엉뚱한 자리를 덮어씀**
후자를 찾는 게 이 도구의 목적이다.

  python audit_all.py            # 미커버 RVA 전수 대조
"""
import io, os, re, sys

sys.path.insert(0, r'C:\tfm2mods\v54')
import sites as S
from pe2 import load

if __name__ == '__main__':
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRCDIR = r'C:\tfm2mods\tfm2_ai_adjust\src'
FILES = ('detour.rs', 'disc19_repro.rs', 'serpen.rs', 'tfm2_ai_adjust.rs')
BYTES = re.compile(r'0x([0-9a-fA-F]{1,2})')


def prefixes_in(text):
    """구간 안의 모든 `&[0x..,0x..]` 리터럴을 prefix 후보로 뽑는다."""
    out = []
    for m in re.finditer(r'&\[((?:\s*0x[0-9a-fA-F]{1,2}\s*,?)+)\]', text):
        out.append(bytes(int(x, 16) for x in BYTES.findall(m.group(1))))
    return out


def main():
    covered = set(x['rva'] for x in S.parse())
    E = load('054')
    rows = []
    for fn in FILES:
        p = os.path.join(SRCDIR, fn)
        if not os.path.exists(p):
            continue
        t = io.open(p, encoding='utf-8').read()
        lines = t.split('\n')
        # ① 라인 단위(생 호출·토글)
        for ln, line in enumerate(lines, 1):
            if 'pskip!' in line:
                continue
            for m in re.finditer(r'base\s*\+\s*0x([0-9a-fA-F_]{5,8})', line):
                rva = int(m.group(1).replace('_', ''), 16)
                if rva in covered:
                    continue
                rows.append((rva, '%s:%d' % (fn, ln), prefixes_in(line), line.strip()[:80]))
        # ② 튜플 루프 — 블록 전체의 prefix 후보를 모두 붙인다
        for m in re.finditer(r'for\s*\([^)]*\)\s*in\s*\[(.*?)\]\s*\{(.*?)\n    \}', t, re.S):
            blk = m.group(0)
            pres = prefixes_in(blk)
            ln = t[:m.start()].count('\n') + 1
            for h in re.finditer(r'0x([0-9a-fA-F]{5,8})', m.group(1)):
                rva = int(h.group(1), 16)
                if rva in covered:
                    continue
                rows.append((rva, '%s:%d(튜플루프)' % (fn, ln), pres, blk.split('\n')[0][:80]))

    seen, uniq = set(), []
    for r in rows:
        if r[0] in seen:
            continue
        seen.add(r[0]); uniq.append(r)
    uniq.sort()

    danger, dead, unk = [], [], []
    for rva, where, pres, src in uniq:
        b = E.rd(rva, 16)
        if not b:
            unk.append((rva, where, '.text 밖', src)); continue
        hit = [p for p in pres if b.startswith(p)]
        if hit:
            danger.append((rva, where, hit[0].hex(), b.hex(), src))
        elif pres:
            dead.append((rva, where, '/'.join(p.hex() for p in pres)[:40], b.hex(), src))
        else:
            unk.append((rva, where, 'prefix 없음', src))

    print('미커버 RVA %d개 (sites.py 커버 %d개 제외)\n' % (len(uniq), len(covered)))
    print('★prefix 일치 = 0.5.4 에서도 그 자리를 실제로 덮어씀 → 오패치 위험: %d' % len(danger))
    for r in danger:
        print('  %06x  %-28s pre=%s  054=%s' % (r[0], r[1], r[2], r[3][:24]))
    print('\nprefix 불일치 = 조용히 skip(노브 사망): %d' % len(dead))
    for r in dead[:200]:
        print('  %06x  %-28s pre=%s  054=%s' % (r[0], r[1], r[2], r[3][:24]))
    if unk:
        print('\n판정불가: %d' % len(unk))
        for r in unk[:40]:
            print('  %06x  %-28s %s' % (r[0], r[1], r[2]))


if __name__ == '__main__':
    main()
