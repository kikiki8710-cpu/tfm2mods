# -*- coding: utf-8 -*-
# serpen_054b.py — 함수 지문 대조(독립 2번째 방법): 문자열참조/immediate/disp 히스토그램
import sys, io, struct, pickle, collections, bisect, re
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True

class Img:
    def __init__(self, path, pkl, tag):
        self.tag = tag
        d = open(path, "rb").read(); self.raw = d
        pe = struct.unpack_from("<I", d, 0x3c)[0]
        nsec = struct.unpack_from("<H", d, pe + 6)[0]; opt = pe + 24
        sectab = opt + struct.unpack_from("<H", d, pe + 20)[0]
        self.secs = []
        for i in range(nsec):
            o = sectab + i*40
            nm = d[o:o+8].rstrip(b"\0").decode(errors="replace")
            vsz, va, rsz, rraw = struct.unpack_from("<IIII", d, o+8)
            self.secs.append((nm, va, max(vsz,rsz), rraw, rsz))
        P = pickle.load(open(pkl,"rb"))["idx"]
        self.fn = {(int(k,16) if isinstance(k,str) else k): v for k,v in P.items()}
        self.starts = sorted(self.fn)
    def roff(self, rva):
        for nm, va, vsz, rraw, rsz in self.secs:
            if va <= rva < va+vsz: return rraw + (rva-va)
    def sec(self, rva):
        for nm, va, vsz, rraw, rsz in self.secs:
            if va <= rva < va+vsz: return nm
        return "?"
    def read(self, rva, n):
        o = self.roff(rva); return None if o is None else self.raw[o:o+n]
    def size(self, rva): return self.fn.get(rva, {}).get("size")
    def strat(self, rva, maxlen=64):
        b = self.read(rva, maxlen)
        if not b: return None
        s = b.split(b"\0")[0]
        if len(s) >= 4 and all(32 <= c < 127 for c in s): return s.decode()
        # rust str: not null-terminated; take printable prefix
        pr = bytes(c for c in b if 32 <= c < 127)
        i = 0
        while i < len(b) and 32 <= b[i] < 127: i += 1
        return b[:i].decode() if i >= 6 else None
    def profile(self, rva):
        n = self.fn[rva]["size"]; b = self.read(rva, n)
        strs = []; imms = collections.Counter(); disps = collections.Counter(); mnem = collections.Counter()
        for ins in md.disasm(b, rva):
            mnem[ins.mnemonic] += 1
            try: ops = ins.operands
            except Exception: continue
            for op in ops:
                if op.type == 2:  # IMM
                    v = op.imm
                    if abs(v) > 7: imms[v] += 1
                elif op.type == 3:  # MEM
                    if op.mem.base == 0 and op.mem.index == 0: continue
                    d0 = op.mem.disp
                    if d0: disps[d0] += 1
            if ins.mnemonic == "lea" and "rip" in ins.op_str:
                m = re.search(r"rip \+ (0x[0-9a-f]+|\-?\d+)", ins.op_str)
                if m:
                    tgt = ins.address + ins.size + int(m.group(1), 0)
                    if self.sec(tgt) in (".rdata", ".data"):
                        s = self.strat(tgt)
                        if s: strs.append(s)
        return strs, imms, disps, mnem

O = Img(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe", r"C:\tfm2mods\_fnidx_053.pkl", "053")
N = Img(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\TeamfightManager2.exe", r"C:\tfm2mods\_fnidx_054.pkl", "054")

import json
PAIRS = json.loads(sys.argv[1])
for nm, old, cands in PAIRS:
    old = int(old,16)
    so, io_, do, mo = O.profile(old)
    print("="*90)
    print(f"[{nm}] 0.5.3 {old:#x} size={O.size(old)}")
    print(f"   strs({len(so)}): {so[:12]}")
    print(f"   top imm : {[(hex(k) if k>0 else k, v) for k,v in io_.most_common(10)]}")
    print(f"   top disp: {[(hex(k) if k>0 else k, v) for k,v in do.most_common(12)]}")
    for c in cands:
        c = int(c,16)
        if c not in N.fn: print(f"   -- {c:#x} 함수시작 아님"); continue
        sn, iN, dN, mN = N.profile(c)
        ci = sum((io_ & iN).values()) / max(1, sum(io_.values()))
        cd = sum((do & dN).values()) / max(1, sum(do.values()))
        cs = len(set(so) & set(sn)) / max(1, len(set(so)))
        print(f"   ↳ 0.5.4 {c:#x} size={N.size(c)} imm일치={ci:.3f} disp일치={cd:.3f} str일치={cs:.3f}({len(set(so)&set(sn))}/{len(set(so))})")
        print(f"       strs({len(sn)}): {sn[:12]}")
        print(f"       top disp: {[(hex(k) if k>0 else k, v) for k,v in dN.most_common(12)]}")
        onlyo = [k for k,_ in io_.most_common() if k not in iN][:8]
        print(f"       구에만 있는 imm: {[hex(k) if k>0 else k for k in onlyo]}")
