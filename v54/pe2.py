# -*- coding: utf-8 -*-
"""0.5.3 / 0.5.4 두 판을 나란히 다루는 공용 도구 (capstone 기반, Ghidra 불필요).

설계 원칙 — 2026-08-05 교훈 반영:
  ⚠**노브 위치부터 찾지 않는다.** 그렇게 하면 "상수가 같은 다른 자리"를 잡아
    "설정값이 죽었다"고 오판한다(0.5.3에서 실제로 겪음).
  ⟹ ①0.5.4의 구조를 **독립적으로** 먼저 세운다(패닉 Location·.pdata·점프테이블)
     ②그 다음에야 0.5.3 지도와 대조한다.

패닉 Location 문자열 = 컴파일러가 박아 넣은 `소스경로:줄:열` → **버전이 바뀌어도
살아남는 가장 단단한 앵커**다(0.5.3에서 plan 이름을 이걸로 확정했다).
"""
import io, os, struct, sys

try:
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64, CS_OPT_DETAIL
except ImportError:
    sys.exit('capstone 필요: pip install capstone')

BASE = 0x140000000
ROOT = r'C:\Users\dev\Desktop\claude\tfm2'
PATHS = {
    '053': os.path.join(ROOT, 'tfm2_0.5.3', 'TeamfightManager2.exe'),
    '054': os.path.join(ROOT, 'tfm2_0.5.4', 'TeamfightManager2.exe'),
}


class Exe:
    def __init__(self, ver):
        self.ver = ver
        self.raw = io.open(PATHS[ver], 'rb').read()
        self._parse()
        self.md = Cs(CS_ARCH_X86, CS_MODE_64)
        self.md.detail = True

    def _parse(self):
        pe = struct.unpack_from('<I', self.raw, 0x3c)[0]
        assert self.raw[pe:pe + 4] == b'PE\0\0'
        nsec = struct.unpack_from('<H', self.raw, pe + 6)[0]
        opt = struct.unpack_from('<H', self.raw, pe + 20)[0]
        self.imagebase = struct.unpack_from('<Q', self.raw, pe + 24 + 24)[0]
        so = pe + 24 + opt
        self.sections = []
        for i in range(nsec):
            o = so + i * 40
            name = self.raw[o:o + 8].rstrip(b'\0').decode('latin1')
            vsz, va, rsz, ra = struct.unpack_from('<IIII', self.raw, o + 8)
            self.sections.append((name, va, vsz, ra, rsz))
        # 디렉터리: .pdata (예외 테이블) = index 3
        dd = pe + 24 + (112 if opt >= 240 else 96)
        self.pdata_rva, self.pdata_sz = struct.unpack_from('<II', self.raw, dd + 3 * 8)

    def off(self, rva):
        for _, va, vsz, ra, rsz in self.sections:
            if va <= rva < va + max(vsz, rsz):
                d = rva - va
                return ra + d if d < rsz else None
        return None

    def rd(self, rva, n):
        o = self.off(rva)
        return self.raw[o:o + n] if o is not None else b''

    def u32(self, rva):
        b = self.rd(rva, 4)
        return struct.unpack('<I', b)[0] if len(b) == 4 else None

    def u64(self, rva):
        b = self.rd(rva, 8)
        return struct.unpack('<Q', b)[0] if len(b) == 8 else None

    def funcs(self):
        """.pdata 에서 (시작RVA, 끝RVA) 전수. 함수 경계의 1차 근거."""
        out = []
        for i in range(self.pdata_sz // 12):
            b = self.rd(self.pdata_rva + i * 12, 12)
            if len(b) < 12:
                break
            s, e, _ = struct.unpack('<III', b)
            if s and e > s:
                out.append((s, e))
        out.sort()
        return out

    def func_of(self, rva):
        for s, e in self.funcs():
            if s <= rva < e:
                return (s, e)
        return None

    def dis(self, rva, n):
        return list(self.md.disasm(self.rd(rva, n), BASE + rva))


def load(ver):
    return Exe(ver)


if __name__ == '__main__':
    for v in ('053', '054'):
        e = load(v)
        print('%s  크기 %d  섹션 %d  함수(.pdata) %d' % (v, len(e.raw), len(e.sections), len(e.funcs())))
        for nm, va, vsz, ra, rsz in e.sections:
            print('   %-8s rva %08x  vsz %08x  raw %08x' % (nm, va, vsz, ra))
