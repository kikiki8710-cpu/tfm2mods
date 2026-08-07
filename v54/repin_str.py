# -*- coding: utf-8 -*-
"""0.5.3 → 0.5.4 재핀 (문자열 xref 앵커).

바이트 마스크 시그니처는 **재컴파일된 버전업에선 실패**한다(실측: 정답 404쌍 중 1개만 매칭).
반면 Rust 바이너리의 문자열(패닉 메시지·소스경로·필드명)은 재컴파일에도 내용이 보존된다.
⟹ "이 함수가 참조하는 문자열 집합"을 지문으로 삼아 신 exe 에서 같은 지문의 함수를 찾는다.

구현 메모: 디스어셈 대신 `lea r64,[rip+disp32]`(48 8d /r) 바이트 패턴을 스캔한다 —
51MB 를 capstone 으로 전수 디스어셈하면 너무 느리다. 오탐(문자열이 아닌 데이터)은
"대상이 .rdata 이고 6자 이상 ASCII" 조건으로 거른다.
"""
import sys, io, re, struct, os, pickle
import bisect as _bs
import pefile
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

LEA = re.compile(rb'\x48\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]', re.S)
ASCII_OK = re.compile(rb'[ -~]{6,}')


class Exe:
    def __init__(self, path, tag):
        self.tag = tag
        pe = pefile.PE(path, fast_load=True)
        self.d = pe.__data__
        sec = {s.Name.rstrip(b'\x00'): s for s in pe.sections}
        pd = sec[b'.pdata']
        raw = self.d[pd.PointerToRawData: pd.PointerToRawData + pd.SizeOfRawData]
        self.fn = []
        for i in range(0, len(raw) - 11, 12):
            b, e, _ = struct.unpack_from('<III', raw, i)
            if b == 0:
                break
            self.fn.append((b, e))
        self.fn.sort()
        self.st = [f[0] for f in self.fn]
        t = sec[b'.text']
        self.tva, self.toff, self.tsz = t.VirtualAddress, t.PointerToRawData, t.SizeOfRawData
        r = sec[b'.rdata']
        self.rva0, self.roff, self.rsz = r.VirtualAddress, r.PointerToRawData, r.SizeOfRawData

    def own(self, a):
        i = _bs.bisect_right(self.st, a) - 1
        return self.fn[i] if i >= 0 and self.fn[i][0] <= a < self.fn[i][1] else None

    def strat(self, rva, n=64):
        if not (self.rva0 <= rva < self.rva0 + self.rsz):
            return None
        o = self.roff + (rva - self.rva0)
        b = self.d[o:o + n]
        m = ASCII_OK.match(b)
        return m.group(0)[:48] if m else None

    def index(self):
        """함수 → 참조 문자열 집합"""
        cache = 'C:/tfm2mods/v54/_strx_%s.pkl' % self.tag
        if os.path.exists(cache):
            return pickle.load(open(cache, 'rb'))
        text = self.d[self.toff: self.toff + self.tsz]
        idx = {}
        n = 0
        for m in LEA.finditer(text):
            p = m.start()
            if p + 7 > len(text):
                continue
            disp = struct.unpack_from('<i', text, p + 3)[0]
            site = self.tva + p
            tgt = site + 7 + disp
            s = self.strat(tgt)
            if not s:
                continue
            f = self.own(site)
            if f:
                idx.setdefault(f[0], set()).add(s)
                n += 1
        print('  [%s] 문자열참조 %d건 · 지문 보유 함수 %d개' % (self.tag, n, len(idx)))
        pickle.dump(idx, open(cache, 'wb'))
        return idx


def main():
    o = Exe(r'C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe', '053')
    n = Exe(r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe', '054')
    print('인덱스 구축…')
    oi, ni = o.index(), n.index()
    # 역색인: 문자열 → 신 함수들
    inv = {}
    for f, ss in ni.items():
        for s in ss:
            inv.setdefault(s, set()).add(f)

    src = sys.argv[1] if len(sys.argv) > 1 else 'truth_old.txt'
    addrs = [int(x, 16) for x in io.open(src).read().split()]
    out = {}
    for a in addrs:
        f = o.own(a)
        if not f:
            continue
        S = oi.get(f[0])
        if not S:
            out[a] = ('nofp', None, 0)
            continue
        score = {}
        for s in S:
            for g in inv.get(s, ()):
                score[g] = score.get(g, 0) + 1
        if not score:
            out[a] = ('nomatch', None, 0)
            continue
        best = sorted(score.items(), key=lambda kv: -kv[1])
        top, c = best[0]
        second = best[1][1] if len(best) > 1 else 0
        j = c / max(1, len(S))
        out[a] = ('ok' if (c > second and j >= 0.5) else 'weak', top, round(j, 2))
    io.open(sys.argv[2] if len(sys.argv) > 2 else 'repin_str_out.txt', 'w', encoding='utf-8').write(
        '\n'.join('%#x %s %s %s' % (a, v[0], ('%#x' % v[1]) if v[1] else '-', v[2])
                  for a, v in sorted(out.items())))
    from collections import Counter
    print(Counter(v[0] for v in out.values()))


main()
