# -*- coding: utf-8 -*-
u"""#13 `target_bush_v30`(0xdf1c80) 을 **어떻게 셀 수 있나** — 두 방식의 실물 타당성 판정.

## 왜 필요한가
진입부 12B 스틸이 불가하다(`+9` 의 `je rel32` 중간이 잘린다). 대안 둘을 **재 보고** 고른다.

**A) 호출부 리다이렉트** — `call rel32`(5B)를 우리 스텁으로 돌린다.
   스텁이 `lock inc` 후 원 함수로 절대점프. **명령 재배치가 전혀 없다**(가장 안전).
   ⚠단 `E8 rel32` 는 ±2GB 안만 닿는다 ⟹ 스텁을 exe 근처에 할당해야 한다.
   ⚠그리고 **호출부를 전부 찾아야** 한다 — 하나라도 놓치면 그 경로 호출이 안 세어진다.

**B) 15B 진입부 스틸 + 분기 보정** — `je rel32` 를 절대 간접점프로 바꿔 스텁에 옮긴다.
   근접 할당이 필요 없다. 대신 `probe.rs` 에 특수 케이스가 생긴다.

이 스크립트는 **A 가 성립하는지**(호출부가 전부 `E8 rel32` 인가, 몇 개인가)와
**B 의 분기 목표**를 실측한다.
"""
import io
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import aiprobe
import capstone

TARGET = 0xdf1c80
BASE = 0x140000000


def main():
    d = open(aiprobe.EXE, "rb").read()
    secs = aiprobe.sections(d)

    def rva2off(rva):
        for va, vsz, ra, rsz in secs:
            if va <= rva < va + max(vsz, rsz):
                return ra + (rva - va)
        return None

    # ── B) 진입부 15B 와 분기 목표 ────────────────────────────────────
    off = rva2off(TARGET)
    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    md.detail = False
    print(u"=== B) 진입부 명령 분해 (0x%x) ===" % TARGET)
    tot = 0
    jt = None
    for ins in md.disasm(d[off:off + 40], BASE + TARGET):
        mark = u""
        if ins.mnemonic.startswith("j"):
            jt = int(ins.op_str, 16)
            mark = u"   ★분기 — 이게 12B 스틸을 막는다"
        print(u"  +%-3d %-22s %-28s (%dB)%s"
              % (tot, u" ".join(u"%02x" % b for b in ins.bytes), u"%s %s" % (ins.mnemonic, ins.op_str),
                 ins.size, mark))
        tot += ins.size
        if tot >= 20:
            break
    if jt:
        print(u"  ⟹ 분기 목표 = **0x%x** (RVA 0x%x) · 15B 스틸 시 이 값을 절대점프로 재작성하면 된다"
              % (jt, jt - BASE))

    # ── A) 호출부 전수 — `E8 rel32` 로 TARGET 을 부르는 곳 ───────────
    print(u"\n=== A) 호출부 전수 스캔 (call rel32 → 0x%x) ===" % TARGET)
    hits = []
    for va, vsz, ra, rsz in secs:
        n = max(vsz, rsz)
        blob = d[ra:ra + n]
        i = 0
        while True:
            i = blob.find(b"\xe8", i)
            if i < 0 or i + 5 > len(blob):
                break
            rel = struct.unpack_from("<i", blob, i + 1)[0]
            site = va + i                      # call 명령의 RVA
            if site + 5 + rel == TARGET:
                hits.append(site)
            i += 1
    for h in hits:
        print(u"  0x%-9x  call → 0x%x   (패치 5B: `E8 <새 rel32>`)" % (h, TARGET))
    print(u"  ⟹ **%d곳**" % len(hits))

    # ── 간접호출 가능성 (주소가 데이터로 들고 있나) ────────────────────
    abs8 = struct.pack("<Q", BASE + TARGET)
    rva4 = struct.pack("<I", TARGET)
    n_abs = sum(d.count(abs8) for _ in (1,))
    print(u"\n=== 간접 참조 확인 ===")
    print(u"  절대주소 8B 리터럴 = %d곳 · RVA 4B 리터럴 = %d곳" % (d.count(abs8), d.count(rva4)))
    print(u"  ⟹ 0 이면 vtable·함수포인터 경유 호출이 없다 = A 로 전 호출을 덮을 수 있다")


if __name__ == "__main__":
    main()
