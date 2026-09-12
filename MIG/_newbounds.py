# -*- coding: utf-8 -*-
u"""정정된 6개 주소의 **함수 경계(bytes)·명령수(instrs)** 를 exe `.pdata` 에서 실측한다.

## 왜 필요한가 — `addr` 만 고치면 반쪽이다
명세 `exe.bytes`·`exe.instrs` 는 **옛(틀린) 함수의 크기**다. 그런데 이 값들은 다운스트림에서
**실제로 쓰인다**:
  · `probe20.py` / `aiprobe.prolog_of` — 프롤로그 길이 판정
  · `rvaverify.py` · `abiagree.py` — `d[off:off+bytes]` 디스어셈 범위
  · `gensweep20.py` — 트램폴린 크기 산정
⟹ 주소만 고치고 크기를 남겨두면 **엉뚱한 길이로 디스어셈**한다(= 새로운 조용한 오류를 만든다).

`.pdata`(RUNTIME_FUNCTION) 의 `BeginAddress`/`EndAddress` 가 **컴파일러가 기록한 정본 경계**다.
"""
import io
import json
import struct
import sys

import capstone

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
EXE = (r"C:\Program Files (x86)\Steam\steamapps\common"
       r"\Teamfight Manager2\TeamfightManager2.exe")
SPEC = r"C:\tfm2mods\MIG\_spec\specs20.json"

# (명세 idx, 구 RVA, 신 RVA) — ghidra-re 2026-09-12 확정
NEW = [(4, 0xd3e4b0, 0xd3cfa0), (10, 0xe12050, 0xe0c560), (11, 0xe6fe60, 0xe4b5d0),
       (12, 0xe70330, 0xe595b0), (15, 0xca4a80, 0xe5c1f0), (16, 0xc809d0, 0xe0daa0)]
# 에이전트가 보고한 크기(대조용). None = 보고 없음.
REPORTED = {10: (851, 232), 11: (672, 156), 12: (905, 198), 15: (1791, 348)}


def main():
    d = open(EXE, "rb").read()
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    nsec = struct.unpack_from("<H", d, pe + 6)[0]
    optsz = struct.unpack_from("<H", d, pe + 20)[0]
    magic = struct.unpack_from("<H", d, pe + 24)[0]
    ddoff = pe + 24 + (112 if magic == 0x20b else 96)
    secs = []
    for i in range(nsec):
        o = pe + 24 + optsz + i * 40
        # ⚠순서 = (VirtualSize, VirtualAddress, SizeOfRawData, PointerToRawData)
        secs.append(struct.unpack_from("<IIII", d, o + 8))

    def r2o(rva):
        for vsz, va, rsz, ra in secs:
            if va <= rva < va + max(vsz, rsz):
                return ra + (rva - va)
        return None

    edir, esz = struct.unpack_from("<II", d, ddoff + 3 * 8)
    po = r2o(edir)
    rt = {}
    for k in range(esz // 12):
        beg, end, unw = struct.unpack_from("<III", d, po + k * 12)
        rt[beg] = end
    print(u".pdata %d 엔트리" % len(rt))

    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    md.detail = False
    D = json.load(io.open(SPEC, encoding="utf-8"))["specs"]

    print(u"\n%-4s %-42s %-10s %-14s %-14s %s"
          % (u"#", u"함수", u"신 RVA", u"명세(옛값)", u"실측(신값)", u"에이전트 보고"))
    print(u"-" * 118)
    out = {}
    for idx, old, new in NEW:
        ex = D[idx]["exe"]
        end = rt.get(new)
        if end is None:
            print(u"%-4d %-42s 0x%-8x  ⛔.pdata 에 경계 없음(리프?)" % (idx, D[idx]["name"][:42], new))
            continue
        nb = end - new
        off = r2o(new)
        ni = len(list(md.disasm(d[off:off + nb], 0x140000000 + new)))
        rep = REPORTED.get(idx)
        agree = u""
        if rep:
            agree = (u"%dB/%d  %s" % (rep[0], rep[1],
                                      u"✅일치" if (rep[0] == nb and rep[1] == ni) else u"⚠불일치"))
        else:
            agree = u"(보고 없음)"
        print(u"%-4d %-42s 0x%-8x  %-14s %-14s %s"
              % (idx, D[idx]["name"][:42], new,
                 u"%dB/%s" % (ex.get("bytes"), ex.get("instrs")),
                 u"%dB/%d" % (nb, ni), agree))
        out[idx] = (nb, ni)

    print(u"\n★patch.json 에 넣을 값")
    for idx in sorted(out):
        nb, ni = out[idx]
        ex = D[idx]["exe"]
        print(u"  #%02d  bytes %s→%d · instrs %s→%d" % (idx, ex.get("bytes"), nb,
                                                        ex.get("instrs"), ni))
    io.open(r"C:\tfm2mods\MIG\_newbounds.json", "w", encoding="utf-8").write(
        json.dumps({str(k): v for k, v in out.items()}, ensure_ascii=False, indent=1))
    print(u"\n→ _newbounds.json 기록")


if __name__ == "__main__":
    main()
