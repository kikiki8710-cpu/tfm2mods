# -*- coding: utf-8 -*-
"""item_tactics 0.5.3 마이그 보조 — PE 바이트스캔 + capstone 디스어셈 공용 헬퍼."""
import sys, re, struct
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

E52 = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe"
E53 = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe"
BASE = 0x140000000

class Exe:
    def __init__(self, path):
        self.path = path
        self.pe = pefile.PE(path, fast_load=True)
        with open(path, 'rb') as f:
            self.data = f.read()
        self.secs = []
        for s in self.pe.sections:
            nm = s.Name.rstrip(b'\x00').decode('latin1')
            self.secs.append((nm, s.VirtualAddress, s.Misc_VirtualSize,
                              s.PointerToRawData, s.SizeOfRawData))
        self.text = next(x for x in self.secs if x[0] == '.text')
        self.md = Cs(CS_ARCH_X86, CS_MODE_64)
        self.md.detail = False
        self._pdata = None

    def off(self, rva):
        for nm, va, vs, pr, sr in self.secs:
            if va <= rva < va + max(vs, sr):
                d = rva - va
                if d < sr:
                    return pr + d
        return None

    def rva_of_off(self, off):
        for nm, va, vs, pr, sr in self.secs:
            if pr <= off < pr + sr:
                return va + (off - pr)
        return None

    def read(self, rva, n):
        o = self.off(rva)
        if o is None:
            return None
        return self.data[o:o+n]

    def text_bytes(self):
        nm, va, vs, pr, sr = self.text
        return self.data[pr:pr+sr], va, pr, sr

    def scan(self, pattern, sect='.text'):
        """pattern: 'aa bb ?? cc' hex with ?? wildcard -> list of RVA"""
        toks = pattern.split()
        rx = b''
        for t in toks:
            if t == '??':
                rx += b'.'
            elif t.startswith('[') :   # [48,49] 형태 = 택일
                alts = t.strip('[]').split(',')
                rx += b'[' + b''.join(bytes([int(a, 16)]) for a in alts) + b']'
            else:
                rx += re.escape(bytes([int(t, 16)]))
        sec = next(x for x in self.secs if x[0] == sect)
        nm, va, vs, pr, sr = sec
        blob = self.data[pr:pr+sr]
        return [va + m.start() for m in re.finditer(rx, blob, re.DOTALL)]

    def pdata(self):
        """[(start_rva, end_rva)] 정렬됨"""
        if self._pdata is not None:
            return self._pdata
        sec = next((x for x in self.secs if x[0] == '.pdata'), None)
        out = []
        if sec:
            nm, va, vs, pr, sr = sec
            blob = self.data[pr:pr+min(vs, sr)]
            for i in range(0, len(blob) - 11, 12):
                s, e, u = struct.unpack_from('<III', blob, i)
                if s == 0 and e == 0:
                    continue
                out.append((s, e))
            out.sort()
        self._pdata = out
        return out

    def func_of(self, rva):
        import bisect
        pd = self.pdata()
        starts = [s for s, e in pd]
        i = bisect.bisect_right(starts, rva) - 1
        if i >= 0 and pd[i][0] <= rva < pd[i][1]:
            return pd[i]
        return None

    def dis(self, rva, n=40, maxb=200):
        b = self.read(rva, maxb)
        if not b:
            return []
        out = []
        for ins in self.md.disasm(b, BASE + rva):
            out.append((ins.address - BASE, ins.mnemonic, ins.op_str, ins.bytes.hex()))
            if len(out) >= n:
                break
        return out

    def pr_dis(self, rva, n=25, tag=''):
        print(f"--- {tag} rva={rva:#x} ---")
        for a, m, o, h in self.dis(rva, n):
            print(f"  {a:#010x}  {h:<24} {m} {o}")

    def hexat(self, rva, n=24):
        b = self.read(rva, n)
        return b.hex(' ') if b else None

    def calls_to(self, target_rva, sect='.text'):
        """직접 e8 rel32 콜사이트 전수. -> [(callsite_rva, retaddr_rva)]"""
        nm, va, vs, pr, sr = next(x for x in self.secs if x[0] == sect)
        blob = self.data[pr:pr+sr]
        out = []
        start = 0
        while True:
            i = blob.find(b'\xe8', start)
            if i < 0:
                break
            start = i + 1
            if i + 5 > len(blob):
                break
            rel = struct.unpack_from('<i', blob, i + 1)[0]
            site = va + i
            tgt = site + 5 + rel
            if tgt == target_rva:
                out.append((site, site + 5))
        return out

O = Exe(E52)
N = Exe(E53)

if __name__ == '__main__':
    print("0.5.2 sections:", [(s[0], hex(s[1]), hex(s[2])) for s in O.secs])
    print("0.5.3 sections:", [(s[0], hex(s[1]), hex(s[2])) for s in N.secs])
    print("pdata 0.5.2:", len(O.pdata()), " 0.5.3:", len(N.pdata()))

def riprefs(E, target, sect='.text'):
    """rip-rel lea(48/4c 8d xx) 로 target(.rdata rva)을 가리키는 .text 사이트 열거"""
    import re, struct
    nm, va, vs, pr, sr = next(x for x in E.secs if x[0] == sect)
    blob = E.data[pr:pr+sr]
    out = []
    rx = re.compile(rb'[\x48\x4c\x49\x4d]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]', re.DOTALL)
    for m in rx.finditer(blob):
        i = m.start()
        if i + 7 > len(blob):
            continue
        disp = struct.unpack_from('<i', blob, i + 3)[0]
        site = va + i
        if site + 7 + disp == target:
            out.append(site)
    return out

def branches_to(E, tgt, ops=(0xe8, 0xe9), sect='.text'):
    """numpy 벡터화: tgt로 가는 call/jmp rel32 사이트 전수 -> [(site, op)]"""
    import numpy as np
    nm, va, vs, pr, sr = next(x for x in E.secs if x[0] == sect)
    arr = np.frombuffer(E.data[pr:pr+sr], dtype=np.uint8)
    out = []
    for op in ops:
        idx = np.where(arr[:-5] == op)[0]
        rel = (arr[idx+1].astype(np.int64) | (arr[idx+2].astype(np.int64) << 8)
               | (arr[idx+3].astype(np.int64) << 16)
               | (arr[idx+4].astype(np.int8).astype(np.int64) << 24))
        t = va + idx + 5 + rel
        for i in idx[t == tgt]:
            out.append((int(va + i), op))
    return sorted(out)
