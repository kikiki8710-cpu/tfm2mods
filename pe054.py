# -*- coding: utf-8 -*-
# pe054.py - 0.5.3 / 0.5.4 exe common PE loader + search utils (2026-08-05 migration)
import struct, sys, io, re, bisect, collections
try:
    sys.stdout.reconfigure(encoding="utf-8")
except Exception:
    pass

P53 = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe"
P54 = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\TeamfightManager2.exe"


class PE:
    def __init__(self, path):
        self.path = path
        d = open(path, "rb").read()
        self.d = d
        pe = struct.unpack_from("<I", d, 0x3c)[0]
        nsec = struct.unpack_from("<H", d, pe + 6)[0]
        opt = pe + 24
        self.ib = struct.unpack_from("<Q", d, opt + 24)[0]
        sectab = opt + struct.unpack_from("<H", d, pe + 20)[0]
        self.secs = []
        for i in range(nsec):
            o = sectab + i * 40
            nm = d[o:o + 8].rstrip(b"\0").decode(errors="replace")
            vsz, va, rsz, rraw = struct.unpack_from("<IIII", d, o + 8)
            self.secs.append((nm, va, vsz, rraw, rsz))
        magic = struct.unpack_from("<H", d, opt)[0]
        ddir = opt + (112 if magic == 0x20b else 96)
        self.ex, self.ez = struct.unpack_from("<II", d, ddir + 3 * 8)
        po = self.roff(self.ex)
        fns = [struct.unpack_from("<III", d, po + i * 12)[:2] for i in range(self.ez // 12)]
        fns.sort()
        self.fns = fns
        self.fstarts = [f[0] for f in fns]
        self._fset = set(self.fstarts)
        self._callers = None

    def sec(self, nm):
        for s in self.secs:
            if s[0] == nm:
                return s
        return None

    def roff(self, rva):
        for nm, va, vsz, rraw, rsz in self.secs:
            if va <= rva < va + max(vsz, rsz):
                o = rva - va
                return rraw + o if o < rsz else None
        return None

    def read(self, rva, n):
        o = self.roff(rva)
        return self.d[o:o + n] if o is not None else b""

    def text(self):
        nm, va, vsz, rr, rs = self.sec(".text")
        return va, self.d[rr:rr + rs]

    def blob(self, name):
        s = self.sec(name)
        if not s:
            return None, b""
        nm, va, vsz, rr, rs = s
        return va, self.d[rr:rr + rs]

    def func_of(self, rva):
        j = bisect.bisect_right(self.fstarts, rva) - 1
        if j < 0:
            return None
        s, e = self.fns[j]
        return (s, e) if s <= rva < e else None

    def is_fstart(self, rva):
        return rva in self._fset

    def callers(self):
        if self._callers is not None:
            return self._callers
        va, blob = self.text()
        n = len(blob)
        cnt = collections.Counter()
        byfn = collections.defaultdict(set)
        sites = collections.defaultdict(list)
        i = 0
        while True:
            i = blob.find(b"\xe8", i)
            if i < 0 or i + 5 > n:
                break
            rel = int.from_bytes(blob[i + 1:i + 5], "little", signed=True)
            site = va + i
            t = site + 5 + rel
            if va <= t < va + n:
                cnt[t] += 1
                sites[t].append(site)
                j = bisect.bisect_right(self.fstarts, site) - 1
                if j >= 0:
                    byfn[t].add(self.fstarts[j])
            i += 1
        self._callers = (cnt, byfn, sites)
        return self._callers

    def find_bytes(self, lit, where=".rdata"):
        va, blob = self.blob(where)
        return [va + m.start() for m in re.finditer(re.escape(lit), blob)]

    def find_bytes_all(self, lit):
        out = []
        for nm, va, vsz, rr, rs in self.secs:
            if rs == 0:
                continue
            blob = self.d[rr:rr + rs]
            for m in re.finditer(re.escape(lit), blob):
                out.append((nm, va + m.start()))
        return out

    def lea_to(self, target):
        va, blob = self.text()
        out = []
        for m in re.finditer(rb'[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]....', blob, re.S):
            s = va + m.start()
            rel = int.from_bytes(blob[m.start() + 3:m.start() + 7], "little", signed=True)
            if s + 7 + rel == target:
                out.append(s)
        return out

    def call_after(self, site, window=64):
        va, blob = self.text()
        o = site - va
        seg = blob[o:o + window]
        i = seg.find(b"\xe8")
        while i >= 0 and i + 5 <= len(seg):
            rel = int.from_bytes(seg[i + 1:i + 5], "little", signed=True)
            t = site + i + 5 + rel
            if va <= t < va + len(blob):
                return t, site + i
            i = seg.find(b"\xe8", i + 1)
        return None, None


def dis(pe, rva, n=16, count=12):
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    b = pe.read(rva, n)
    out = []
    for ins in md.disasm(b, rva):
        out.append("%08x %-22s %s %s" % (ins.address, ins.bytes.hex(), ins.mnemonic, ins.op_str))
        if len(out) >= count:
            break
    return out


def load53():
    return PE(P53)


def load54():
    return PE(P54)
