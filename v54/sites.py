# -*- coding: utf-8 -*-
"""detour.rs 의 **패치 호출부를 전수 파싱**한다.

  p!(base + 0xRVA, &[0x48,0x83,..], off, w, val)     → prefix 하나
  pany!(base + 0xRVA, CMP1, off, w, val)             → prefix 후보 배열(이름)
  pany!(base + 0xRVA, [[0xb8]], off, w, val)          → prefix 후보 배열(리터럴)

산출: 각 사이트의 (파일:줄, RVA, prefix 후보들, imm_off, width).
prefix 를 같이 뽑는 이유 = 0.5.4 에서 **스택 변위나 레지스터가 바뀌면 prefix 도
같이 고쳐야** 하기 때문. RVA 만 바꾸면 prefix 검증에서 걸려 조용히 skip 된다
(크래시는 안 나지만 노브가 죽는다 — 알아채기 어려운 실패 방식).
"""
import io, os, re, sys, collections

if __name__ == '__main__':   # ⚠import 시 감싸면 다른 래퍼와 충돌해 buffer 가 닫힌다
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
SRCDIR = r'C:\tfm2mods\tfm2_ai_adjust\src'

CALL = re.compile(
    r'\b(p|pany)!\(\s*base\s*\+\s*0x([0-9a-fA-F]+)\s*,\s*(.+?)\s*,\s*(\d+)\s*,\s*(\d+)\s*,',
    re.S)
# ★루프형 — `for a in [0xAAA usize, 0xBBB] { p!(base + a, PRE, off, w, ..); }`
#   이걸 빼먹으면 사이트의 1/3(≈136개)이 0.5.3 주소에 그대로 남는다.
LOOP = re.compile(
    r'for\s+(\w+)\s+in\s+\[([^\]]*?0x[0-9a-fA-F]+[^\]]*?)\]\s*\{(.*?)\}',
    re.S)
LCALL = re.compile(
    r'\b(p|pany)!\(\s*base\s*\+\s*(\w+)\s*,\s*(.+?)\s*,\s*(\d+)\s*,\s*(\d+)\s*,')
ADDRLIT = re.compile(r'0x([0-9a-fA-F]{4,8})')
BYTES = re.compile(r'0x([0-9a-fA-F]{1,2})')
CONSTDEF = re.compile(r'(?:const|static)\s+(\w+)\s*:\s*\[\[u8;\s*\d+\];\s*\d+\]\s*=\s*(\[.+?\]);', re.S)


def parse(files=('detour.rs', 'disc19_repro.rs', 'serpen.rs', 'tfm2_ai_adjust.rs')):
    consts = {}
    texts = {}
    for fn in files:
        p = os.path.join(SRCDIR, fn)
        if not os.path.exists(p):
            continue
        t = io.open(p, encoding='utf-8').read()
        texts[fn] = t
        for name, body in CONSTDEF.findall(t):
            groups = re.findall(r'\[([^\[\]]+)\]', body)
            consts[name] = [tuple(int(x, 16) for x in BYTES.findall(g)) for g in groups]

    out = []
    for fn, t in texts.items():
        # 줄 번호를 붙이기 위해 오프셋→줄 맵
        starts = [0]
        for i, ch in enumerate(t):
            if ch == '\n':
                starts.append(i + 1)

        def lineno(pos):
            import bisect
            return bisect.bisect_right(starts, pos)

        found = []
        for m in CALL.finditer(t):
            found.append((m.start(), m.group(1), int(m.group(2), 16),
                          m.group(3), int(m.group(4)), int(m.group(5)), None))
        for lm in LOOP.finditer(t):
            var, arr, body = lm.group(1), lm.group(2), lm.group(3)
            addrs = [int(x, 16) for x in ADDRLIT.findall(arr)]
            for cm in LCALL.finditer(body):
                if cm.group(2) != var:
                    continue        # `base + ca` 처럼 런타임 계산값 — 정적 재배치 대상 아님
                for a in addrs:
                    found.append((lm.start(), cm.group(1), a, cm.group(3),
                                  int(cm.group(4)), int(cm.group(5)), (var, tuple(addrs))))

        for pos, kind, rva, pre, off, w, loop in found:
            m = None
            pre = pre.strip()
            if pre.startswith('&['):
                cands = [tuple(int(x, 16) for x in BYTES.findall(pre))]
            elif pre.startswith('[['):
                cands = [tuple(int(x, 16) for x in BYTES.findall(g))
                         for g in re.findall(r'\[([^\[\]]+)\]', pre)]
            elif pre in consts:
                cands = consts[pre]
            else:
                cands = None          # 변수/식 — 정적으로 못 푼다
            out.append(dict(file=fn, line=lineno(pos), kind=kind, rva=rva,
                            pre=cands, pre_txt=pre, off=off, w=w, loop=loop))
    return out


if __name__ == '__main__':
    s = parse()
    print('패치 호출부 %d개' % len(s))
    c = collections.Counter(x['file'] for x in s)
    for k, v in c.most_common():
        print('  %-24s %4d' % (k, v))
    unres = [x for x in s if x['pre'] is None]
    print('\nprefix 를 정적으로 못 푼 것 %d개' % len(unres))
    for x in unres[:20]:
        print('  %s:%d  %06x  pre=%s' % (x['file'], x['line'], x['rva'], x['pre_txt'][:50]))
    dup = collections.Counter(x['rva'] for x in s)
    print('\n같은 RVA 를 두 번 이상 패치 %d개' % len([k for k, v in dup.items() if v > 1]))
