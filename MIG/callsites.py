# -*- coding: utf-8 -*-
"""callsites.py — exe `.text` 를 raw 스캔해 **특정 RVA 를 겨누는 `call`/`jmp rel32` 를 전수** 센다.

왜 = ★**Ghidra 의 xref 는 「코드로 분석된 영역」만 본다.** 2026-09-12 실사고:
      `v3_epicops_repair_need(0xdeaa70)` 의 런타임 발화수가 그 유일한 호출자라던
      `#18`(1,369,909)보다 **8.1% 많았다**(1,480,984). Ghidra xref 는 호출부 1곳만 알려줬는데
      이 스캔이 **2곳**을 찾았다 — 두 번째(`0x140ddb2d5`)는 Ghidra 가 **함수로 분석하지 않은 영역**에
      있어 `get_function_by_address` 조차 "No function found" 를 낸다.
   ⟹ **발화수가 CFG 예측과 어긋나면 내 독해가 아니라 도구의 분석 범위를 먼저 의심하라.**
      (CLAUDE.md §6 「도구의 상태를 세계의 상태로 착각」의 실사례.)

사용: python MIG\callsites.py            (아래 TARGETS 를 고쳐 쓴다)
⚠rel32 바이트열은 데이터와 우연히 일치할 수 있다 — 결과를 **상한**으로 읽고, 각 히트의
  앞 3바이트(인자 세팅으로 보이는가)를 함께 확인하라. 확실히 하려면 `dloc`/디스어셈으로 교차확인.
"""
import struct, sys

EXE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
BASE = 0x140000000
TARGETS = {
    0xdeaa70: "v3_epicops_repair_need",
    0xdce220: "v3_epicops_buff_window(#18)",
    0xdea4a0: "v3_epic_group_line",
    0xec9bf0: "is_object_being_taken_by_enemy",
}

d = open(EXE, "rb").read()
pe = struct.unpack_from("<I", d, 0x3c)[0]
nsec = struct.unpack_from("<H", d, pe + 6)[0]
opthdr = struct.unpack_from("<H", d, pe + 20)[0]
secs = []
for i in range(nsec):
    o = pe + 24 + opthdr + i * 40
    name = d[o:o + 8].rstrip(b"\0").decode("ascii", "replace")
    vsz, va, rsz, ra = struct.unpack_from("<IIII", d, o + 8)
    secs.append((name, va, vsz, ra, rsz))
    print("sec %-8s rva=%#010x vsz=%#x raw=%#x" % (name, va, vsz, ra))

text = [s for s in secs if s[0] == ".text"][0]
_, tva, tvsz, tra, trsz = text
buf = d[tra:tra + trsz]
print("\n.text rva %#x .. %#x (%d bytes)\n" % (tva, tva + trsz, trsz))

for tgt, nm in TARGETS.items():
    hits = []
    for op, opname in ((0xE8, "call"), (0xE9, "jmp")):
        i = 0
        while True:
            i = buf.find(bytes([op]), i)
            if i < 0 or i + 5 > len(buf):
                break
            rel = struct.unpack_from("<i", buf, i + 1)[0]
            site = tva + i                      # 명령 시작 RVA
            if site + 5 + rel == tgt:
                hits.append((site, opname, buf[max(0, i - 3):i].hex()))
            i += 1
    print("%-32s %#08x : %d 곳" % (nm, tgt, len(hits)))
    for site, opname, pre in hits:
        print("    %#010x  %-4s  (앞 3B: %s)" % (BASE + site, opname, pre))
    print()
