# -*- coding: utf-8 -*-
"""0.5.3 RVA → 0.5.4 RVA 배치 재핀 (exe↔exe 마스크 시그니처).

왜 이 방식인가: 0.5.3→0.5.4 는 함수가 커져 **함수 내부 오프셋이 보존되지 않는다**
(disc19 재핀 때 실측). 그래서 2단계로 한다.
  ① 컨테이너 함수를 **마스크 시그니처**로 매칭 (rel32 분기·rip-relative disp32 를 와일드카드)
  ② 매칭된 0.5.4 함수 **안에서** 대상 명령을 그 명령의 마스크 바이트로 다시 찾는다

출력: matched(유일) / multi(다중매치=불확정) / none(로직변경 → ★Ghidra RE 필요) / nofn(.pdata 밖)

사용: python repin.py [주소파일] [출력]
"""
import sys, io, struct, os
import bisect as _bs
import pefile, capstone
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

OLD = r'C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe'
NEW = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
md.detail = True


class Exe:
    def __init__(self, path):
        self.pe = pefile.PE(path, fast_load=True)
        self.data = self.pe.__data__
        pd = [s for s in self.pe.sections if s.Name.rstrip(b'\x00') == b'.pdata'][0]
        raw = self.data[pd.PointerToRawData: pd.PointerToRawData + pd.SizeOfRawData]
        self.fn = []
        for i in range(0, len(raw) - 11, 12):
            b, e, _ = struct.unpack_from('<III', raw, i)
            if b == 0:
                break
            self.fn.append((b, e))
        self.fn.sort()
        self.st = [f[0] for f in self.fn]
        t = [s for s in self.pe.sections if s.Name.rstrip(b'\x00') == b'.text'][0]
        self.tva, self.toff, self.tsz = t.VirtualAddress, t.PointerToRawData, t.SizeOfRawData
        self.text = self.data[self.toff: self.toff + self.tsz]

    def own(self, r):
        i = _bs.bisect_right(self.st, r) - 1
        return self.fn[i] if i >= 0 and self.fn[i][0] <= r < self.fn[i][1] else None

    def off(self, r):
        return self.toff + (r - self.tva) if self.tva <= r < self.tva + self.tsz else None

    def bytes_at(self, r, n):
        o = self.off(r)
        return self.data[o:o + n] if o is not None else b''


def mask(code, va):
    """(바이트, 마스크) — 마스크 0 = 와일드카드. rel32/rip-disp32/rel8 을 죽인다."""
    b = bytearray(code)
    m = bytearray(b'\xff' * len(code))
    for ins in md.disasm(code, va):
        i = ins.address - va
        # 상대 분기
        if ins.mnemonic in ('call', 'jmp') or ins.mnemonic.startswith('j'):
            if ins.op_str.startswith('0x'):
                for k in range(max(0, ins.size - 4), ins.size):
                    m[i + k] = 0
        # rip-relative
        if 'rip' in ins.op_str:
            for k in range(max(0, ins.size - 8), ins.size):
                m[i + k] = 0
    return bytes(b), bytes(m)


def find(hay, b, m, limit=4):
    """마스크 매치 위치들(최대 limit개). 마스크가 살아있는 최장 리터럴 구간을 앵커로 씀."""
    best, bl, cur, cs = 0, 0, 0, 0
    for i, mm in enumerate(m):
        if mm:
            if cur == 0:
                cs = i
            cur += 1
            if cur > bl:
                bl, best = cur, cs
        else:
            cur = 0
    if bl < 4:
        return []
    anchor = b[best:best + bl]
    out, p = [], 0
    while len(out) < limit:
        p = hay.find(anchor, p)
        if p < 0:
            break
        s = p - best
        if s >= 0 and s + len(b) <= len(hay):
            seg = hay[s:s + len(b)]
            if all(m[k] == 0 or seg[k] == b[k] for k in range(len(b))):
                out.append(s)
        p += 1
    return out


def main():
    src = sys.argv[1] if len(sys.argv) > 1 else 'd_addrs.txt'
    out = sys.argv[2] if len(sys.argv) > 2 else 'repin_out.txt'
    o, n = Exe(OLD), Exe(NEW)
    addrs = [int(x, 16) for x in io.open(src).read().split()]
    print('대상 %d개 / OLD .text %dB / NEW .text %dB' % (len(addrs), o.tsz, n.tsz))

    # 컨테이너별 그룹
    groups, nofn = {}, []
    for a in addrs:
        f = o.own(a)
        (groups.setdefault(f, []).append(a) if f else nofn.append(a))
    print('컨테이너 %d개 · .pdata 밖 %d개' % (len(groups), len(nofn)))

    res = {'matched': [], 'multi': [], 'none': [], 'inner_fail': []}
    for gi, (f, sites) in enumerate(sorted(groups.items()), 1):
        b53, e53 = f
        code = o.bytes_at(b53, min(e53 - b53, 900))
        if not code:
            res['none'] += [(a, 'no-bytes') for a in sites]
            continue
        bb, mm = mask(code, 0x140000000 + b53)
        hits = find(n.text, bb, mm)
        if len(hits) != 1:
            key = 'multi' if len(hits) > 1 else 'none'
            res[key] += [(a, '%s(%d)' % (key, len(hits))) for a in sites]
        else:
            b54 = n.tva + hits[0]
            f54 = n.own(b54) or (b54, b54 + (e53 - b53))
            for a in sites:
                # ② 함수 안에서 대상 명령을 재탐색
                ib = o.bytes_at(a, 16)
                ins = next(md.disasm(ib, 0x140000000 + a), None)
                sz = ins.size if ins else 8
                tb, tm = mask(ib[:sz], 0x140000000 + a)
                span = n.data[n.off(f54[0]): n.off(f54[1])]
                h2 = find(span, tb, tm, limit=3)
                if len(h2) == 1:
                    res['matched'].append((a, f54[0] + h2[0]))
                else:
                    res['inner_fail'].append((a, 'fn %#x, 내부매치 %d' % (b54, len(h2))))
        if gi % 25 == 0:
            print('  … %d/%d 컨테이너' % (gi, len(groups)))

    L = []
    L.append('=== 재핀 결과 (0.5.3 → 0.5.4) ===')
    L.append('matched %d · inner_fail %d · multi %d · none %d · nofn %d'
             % (len(res['matched']), len(res['inner_fail']), len(res['multi']),
                len(res['none']), len(nofn)))
    L.append('\n[matched] 구RVA → 신RVA')
    for a, b in sorted(res['matched']):
        L.append('  %#010x -> %#010x' % (a, b))
    for k in ('inner_fail', 'multi', 'none'):
        L.append('\n[%s] ★Ghidra RE 필요' % k)
        for a, why in sorted(res[k]):
            L.append('  %#010x  %s' % (a, why))
    L.append('\n[nofn] .pdata 밖(데이터/스텁 추정)')
    L.append('  ' + ' '.join('%#x' % a for a in sorted(nofn)))
    io.open(out, 'w', encoding='utf-8').write('\n'.join(L))
    print('\n' + L[1])
    print('(전체 = %s)' % out)


main()
