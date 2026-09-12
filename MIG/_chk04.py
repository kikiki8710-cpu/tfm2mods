# -*- coding: utf-8 -*-
u"""게임 exe 의 `0xd3e4b0` 이 정말 `handle_line_defense` 인가.

★가설 — `#04` 의 `exe.addr` 근거는 명세상 **`evidence: "fp"`(지문 매칭)** 이고 확정이 아니다.
  그 주소가 다른 함수라면, `w_4` 가 그 함수의 인자를 받아 그대로 내 사본 `my_4` 에 넘기므로
  `a3` 이 엉뚱한 값이 되고 `[a3+8]` 역참조에서 **0xc0000005** 가 난다 — 실제 크래시와 일치한다.

판별 지문(내 사본이 실제로 보인 모양):
  · 5번째 스택 인자를 **바이트로 읽어** 0/1/2 로 **3분기**한다(LineType Top/Mid/Bottom)
  · 4번째 인자(r9)를 `+8` 로 역참조한다(OperationData.context)
⟹ 게임 함수도 같은 모양이어야 한다. 아니면 주소가 틀린 것이다.
"""
import io
import json
import os
import struct
import sys

import capstone

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
EXE = (r"C:\Program Files (x86)\Steam\steamapps\common"
       r"\Teamfight Manager2\TeamfightManager2.exe")
SPEC = r"C:\tfm2mods\MIG\_spec\specs20_v3.json"


def sections(d):
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    n = struct.unpack_from("<H", d, pe + 6)[0]
    opt = struct.unpack_from("<H", d, pe + 20)[0]
    return [struct.unpack_from("<IIII", d, pe + 24 + opt + i * 40 + 8) for i in range(n)]


def r2o(secs, rva):
    for vsz, va, rsz, ra in secs:
        if va <= rva < va + max(vsz, rsz):
            return ra + (rva - va)
    return None


d = open(EXE, "rb").read()
secs = sections(d)
D = json.load(io.open(SPEC, encoding="utf-8"))["specs"]
ex = D[4]["exe"]
rva = int(ex["addr"], 16)
n = int(ex.get("bytes") or 400)
print(u"#04 게임 함수 = 0x%x · %d B · %s instrs · evidence=%s"
      % (rva, n, ex.get("instrs"), ex.get("evidence")))
print(u"IR 범위 = %s %d~%d (%d줄)\n" % (D[4]["ir"]["file"], D[4]["ir"]["frm"],
                                      D[4]["ir"]["to"], D[4]["ir"]["to"] - D[4]["ir"]["frm"]))

off = r2o(secs, rva)
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
md.detail = False
ins = list(md.disasm(d[off:off + n], 0x140000000 + rva))
print(u"--- 게임 0x%x 디스어셈 (앞 26명령) ---" % rva)
for i in ins[:26]:
    print(u"  +0x%-4x %-8s %s" % (i.address - (0x140000000 + rva), i.mnemonic, i.op_str))

# 지문 검사
txt = u"\n".join(u"%s %s" % (i.mnemonic, i.op_str) for i in ins)
print(u"\n--- 지문 대조 ---")
c1 = u"rbp + 0x" in txt or u"rsp + 0x" in txt
byte_arg = [i for i in ins if i.mnemonic == "movzx" and ("byte ptr" in i.op_str)]
r9deref = [i for i in ins if "[r9" in i.op_str]
print(u"  5번째 스택 인자를 바이트로 읽나 : %s  (movzx byte 명령 %d개)"
      % (u"있음" if byte_arg else u"**없음**", len(byte_arg)))
for i in byte_arg[:4]:
    print(u"      +0x%-4x %s %s" % (i.address - (0x140000000 + rva), i.mnemonic, i.op_str))
print(u"  r9(a3) 역참조           : %s" % (u"있음" if r9deref else u"**없음**"))
for i in r9deref[:4]:
    print(u"      +0x%-4x %s %s" % (i.address - (0x140000000 + rva), i.mnemonic, i.op_str))
cmps = [i for i in ins if i.mnemonic == "cmp" and i.op_str.endswith(", 1")]
print(u"  `cmp …, 1`(3분기 흔적)   : %d개" % len(cmps))
print(u"\n총 명령 %d개 (명세 instrs=%s)" % (len(ins), ex.get("instrs")))
