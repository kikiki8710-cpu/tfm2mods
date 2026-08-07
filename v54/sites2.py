# -*- coding: utf-8 -*-
"""2차 파서 — **`patch_imm_bytes(...)` 직접 호출** 사이트.

1차(`sites.py`)는 `p!`/`pany!` 매크로만 봤다. 그런데 코드베이스엔 매크로를 안 쓰고
`ok += patch_imm_bytes(base + 0xXXX, &[..], off, w, val) as u32;` 로 직접 부르는 자리가
따로 있다. 이걸 빼먹은 탓에 인게임 실측이 **applied=569/824** 로 나왔다
(569 = 1차로 옮긴 수와 정확히 일치, 나머지 255가 통째로 실패).

⚠교훈: "패치 사이트 목록"을 만들 땐 **호출 형태를 가정하지 말고 실제 패치 함수의
   호출자를 전수로 세어라.** 매크로만 세면 조용히 1/3을 놓친다.

  · 단독:  patch_imm_bytes(base + 0xRVA, &[..], off, w, val)
  · 루프:  for a in [0xA, 0xB] { ... patch_imm_bytes(base + a, &[..], off, w, val) ... }
"""
import io, os, re, sys, collections

SRCDIR = r'C:\tfm2mods\tfm2_ai_adjust\src'

DIRECT = re.compile(
    r'patch_imm_bytes\(\s*base\s*\+\s*0x([0-9a-fA-F]+)\s*,\s*(&\[[^\]]*\])\s*,\s*(\d+)\s*,\s*(\d+)\s*,')
LOOP = re.compile(r'for\s+(\w+)\s+in\s+\[([^\]]*?0x[0-9a-fA-F]+[^\]]*?)\]\s*\{(.*?)\n(\s*)\}', re.S)
LDIRECT = re.compile(
    r'patch_imm_bytes\(\s*base\s*\+\s*(\w+)\s*,\s*(&\[[^\]]*\])\s*,\s*(\d+)\s*,\s*(\d+)\s*,')
BYTES = re.compile(r'0x([0-9a-fA-F]{1,2})')
ADDRLIT = re.compile(r'0x([0-9a-fA-F]{4,8})')



# ★표+루프 형태 — `const NAME: [(usize, &[u8], usize); N] = [(0xA, &[..], off), ...];`
#   + `for &(a, pre, off) in NAME.iter() { p!(base + a, pre, off, W, ..) }`
#   스택 오버플로를 피하려고 명시 전개를 표로 되돌리면서 생긴 형태다.
#   이걸 파서가 못 보면 62사이트가 **검증·원본값 가드 사각지대**가 된다.
TBL = re.compile(
    r'(?:const|static)\s+(\w+)\s*:\s*\[\(usize,\s*&\[u8\],\s*usize\);\s*\d+\]\s*=\s*\[(.*?)\];',
    re.S)
TROW = re.compile(r'\(\s*0x([0-9a-fA-F]+)\s*,\s*&\[([^\]]*)\]\s*,\s*(\d+)\s*\)')
TUSE = re.compile(r'for\s*&\(a,\s*pre,\s*off\)\s*in\s*(\w+)\.iter\(\)\s*\{[^}]*?,\s*(\d+)\s*,')


def parse_tables(files=('detour.rs', 'disc19_repro.rs')):
    out = []
    for fn in files:
        p = os.path.join(SRCDIR, fn)
        if not os.path.exists(p):
            continue
        t = io.open(p, encoding='utf-8').read()
        starts = [0] + [i + 1 for i, c in enumerate(t) if c == chr(10)]

        def lineno(pos):
            import bisect
            return bisect.bisect_right(starts, pos)

        tabs = {m.group(1): (m.group(2), lineno(m.start())) for m in TBL.finditer(t)}
        for m in TUSE.finditer(t):
            name, w = m.group(1), int(m.group(2))
            if name not in tabs:
                continue
            body, ln = tabs[name]
            for r in TROW.finditer(body):
                out.append(dict(file=fn, line=ln, rva=int(r.group(1), 16),
                                pre=[tuple(int(x, 16) for x in BYTES.findall(r.group(2)))],
                                pre_txt=r.group(2), off=int(r.group(3)), w=w, loop=(name, ())))
    return out


def parse(files=('detour.rs', 'disc19_repro.rs', 'serpen.rs', 'tfm2_ai_adjust.rs')):
    out = []
    for fn in files:
        p = os.path.join(SRCDIR, fn)
        if not os.path.exists(p):
            continue
        t = io.open(p, encoding='utf-8').read()
        starts = [0] + [i + 1 for i, c in enumerate(t) if c == '\n']

        def lineno(pos):
            import bisect
            return bisect.bisect_right(starts, pos)

        seen = set()
        for lm in LOOP.finditer(t):
            var, arr, body = lm.group(1), lm.group(2), lm.group(3)
            addrs = [int(x, 16) for x in ADDRLIT.findall(arr)]
            for cm in LDIRECT.finditer(body):
                if cm.group(1) != var:
                    continue
                for a in addrs:
                    out.append(dict(file=fn, line=lineno(lm.start()), rva=a,
                                    pre=[tuple(int(x, 16) for x in BYTES.findall(cm.group(2)))],
                                    pre_txt=cm.group(2), off=int(cm.group(3)), w=int(cm.group(4)),
                                    loop=(var, tuple(addrs))))
                    seen.add(lm.start() + cm.start())
        for m in DIRECT.finditer(t):
            out.append(dict(file=fn, line=lineno(m.start()), rva=int(m.group(1), 16),
                            pre=[tuple(int(x, 16) for x in BYTES.findall(m.group(2)))],
                            pre_txt=m.group(2), off=int(m.group(3)), w=int(m.group(4)),
                            loop=None))
    return out + parse_tables()


if __name__ == '__main__':
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    s = parse()
    print('직접 호출 사이트 %d개' % len(s))
    for k, v in collections.Counter(x['file'] for x in s).most_common():
        print('  %-22s %4d' % (k, v))
    print('  루프형 %d' % len([x for x in s if x['loop']]))
