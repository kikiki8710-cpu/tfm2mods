# -*- coding: utf-8 -*-
"""아직 0.5.4 로 못 옮긴(=조용히 죽어 있는) 패치 사이트를 전수로 뽑는다.

죽는 방식은 셋뿐이고, 셋 다 **크래시가 아니라 무동작**이다:
  ① 054 에 그 주소가 명령이 아님   (0.5.3 주소가 그대로 남은 경우)
  ② prefix 불일치                  (주소는 맞는데 레지스터·변위가 바뀐 경우)
  ③ 즉치 범위 초과                 (명령이 짧아진 경우)
`patch_imm_bytes` 가 셋 다 검사하고 아무것도 안 쓰므로 안전하지만, 그만큼
"적용 N/M" 숫자로만 드러난다.
"""
import io, os, re, sys, collections

sys.path.insert(0, r'C:\tfm2mods\v54')
import sites as S1
import sites2 as S2
import reloc as R
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
B = 0x140000000
E3, E4 = R.E3, R.E4
SRC = r'C:\tfm2mods\tfm2_ai_adjust\src'
_c = {}


def ins(rva):
    f = E4.func_of(rva)
    if not f:
        return None
    if f[0] not in _c:
        _c[f[0]] = {i.address - B: i for i in R.insns(E4, f[0], f[1])}
    return _c[f[0]].get(rva)


def fnmap(fn):
    lines = io.open(os.path.join(SRC, fn), encoding='utf-8').read().split('\n')
    b = [i for i, l in enumerate(lines) if re.match(r'(unsafe )?fn \w+', l)] + [len(lines)]
    return [(re.match(r'(?:unsafe )?fn (\w+)', lines[b[k]]).group(1), b[k] + 1, b[k + 1] + 1)
            for k in range(len(b) - 1)]


def main():
    maps = {f: fnmap(f) for f in ('detour.rs', 'disc19_repro.rs')}
    rows = []
    for x in S1.parse() + S2.parse():
        i = ins(x['rva'])
        if i is None:
            why = '054에 명령 없음(0.5.3 주소 잔존)'
        elif x['off'] + x['w'] > len(i.bytes):
            why = '즉치범위초과'
        elif not any(bytes(i.bytes[:len(p)]) == bytes(p) for p in (x['pre'] or [])):
            why = 'prefix불일치(레지스터/변위 변경)'
        else:
            continue
        fn = next((n for n, s, e in maps.get(x['file'], []) if s <= x['line'] < e), x['file'])
        d = E3.func_of(x['rva'])
        src = R.SRC3.get(d[0], '') if d else ''
        rows.append((fn, x['rva'], x['off'], x['w'], why,
                     src.split(chr(92))[-1][:40], x['file'], x['line']))

    rows.sort()
    print('아직 죽어 있는 사이트 %d개\n' % len(rows))
    cur = None
    for fn, rva, off, w, why, src, f, ln in rows:
        if fn != cur:
            print('■ %s' % fn)
            cur = fn
        print('    %06x  off%-2d w%d   %-30s %-30s %s:%d' % (rva, off, w, why, src, f, ln))
    print('\n함수별:', dict(collections.Counter(r[0] for r in rows)))


if __name__ == '__main__':
    main()
