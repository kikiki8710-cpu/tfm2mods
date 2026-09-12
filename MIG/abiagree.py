#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""abiagree.py — **exe 의 함수가 IR `define` 과 같은 ABI 로 컴파일됐는가.** (2026-09-12 신설)

## 왜 필요한가 — 실제 크래시가 났다
`#04 handle_line_defense` sweep 을 켜자 **0xc0000005**(액세스 위반)로 게임이 죽었다.
분석 결과:

| | rcx | rdx | r8 | r9 | 스택 |
|---|---|---|---|---|---|
| **IR define** | `i64`(값) | `&StdRng`(320B) | `&PlayerState`(2528B) | `&OperationData`(24B) | `LineType`(i8) |
| **exe 실측** | `[rcx+0x930]` 역참조 = **&PlayerState** | `[rdx+0x10]` | `test r8b,r8b` = **LineType** | ptr | — |

⟹ exe 의 그 함수는 **LTO 로 내부화되면서 rustc 가 ABI 를 바꿨다**(`a0: usize` 는 죽고
   `PlayerState+0x930` 에서 다시 읽는다 · `LineType` 이 스택에서 r8 로 올라왔다).
   래퍼가 IR 순서대로 받은 줄 알고 내 링크사본에 넘기면 **인자가 어긋나 즉사**한다.

★**기존 검사가 왜 못 잡았나** — `llvm-nm` 으로 **rlib 심볼이 `T`(노출)인지**만 봤다.
  그건 「내 사본을 부를 수 있나」를 말할 뿐, **「게임 쪽이 같은 ABI 인가」는 아무도 안 봤다.**
  ⟹ 「노출돼 있다」와 「같은 규약으로 부를 수 있다」는 **다른 사실**이다.

## 판정식 — 반증형(주장을 지지하는 관측이 0일 때만 불일치)
IR 이 `%k = i64/i8/i32`(**값**)라고 했는데 exe 가 그 인자 레지스터를 **포인터로 역참조**하면
그건 표기로 설명되지 않는다 ⟹ **ABI 불일치 확정**.
반대로 IR 이 `ptr` 인데 exe 가 역참조를 안 하는 것은 **증거가 아니다**(안 쓰고 넘길 수 있다) — 기각에 안 쓴다.

사용: python -X utf8 MIG\\abiagree.py
"""
import io
import json
import os
import re
import struct
import sys

import capstone

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
EXE = (r"C:\Program Files (x86)\Steam\steamapps\common"
       r"\Teamfight Manager2\TeamfightManager2.exe")
SPEC = r"C:\tfm2mods\MIG\_spec\specs20_v3.json"
IRDIR = r"C:\tfm2mods\_gaibc"

# Windows x64: 정수/포인터 인자 1~4 = rcx, rdx, r8, r9. 5번째부터 스택.
AREG = ["rcx", "rdx", "r8", "r9"]
SUB = {"rcx": ("ecx", "cx", "cl"), "rdx": ("edx", "dx", "dl"),
       "r8": ("r8d", "r8w", "r8b"), "r9": ("r9d", "r9w", "r9b")}

# ★대상 = 2단계 sweep 표(5개). 나머지는 애초에 sweep 밖이다.
TARGET = [9, 1, 8, 4, 16]


def sections(d):
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    n = struct.unpack_from("<H", d, pe + 6)[0]
    opt = struct.unpack_from("<H", d, pe + 20)[0]
    # ⚠PE 섹션 헤더 +8 의 순서 = (VirtualSize, VirtualAddress, SizeOfRawData, PointerToRawData).
    #   내가 이걸 (va, vsz, ra, rsz) 로 읽어 **엉뚱한 곳을 디스어셈**하고 「RVA 가 틀렸다」는
    #   잘못된 결론을 낼 뻔했다(2026-09-12). 순서를 여기 못 박아 둔다.
    return [struct.unpack_from("<IIII", d, pe + 24 + opt + i * 40 + 8) for i in range(n)]


def r2o(secs, rva):
    for vsz, va, rsz, ra in secs:
        if va <= rva < va + max(vsz, rsz):
            return ra + (rva - va)
    return None


ARGSPLIT = re.compile(r",(?![^(]*\))")


def ir_args(sp):
    p = os.path.join(IRDIR, sp["ir"]["file"])
    src = io.open(p, encoding="utf-8", errors="replace").read().split("\n")
    dl = None
    for k in range(sp["ir"]["frm"] - 1, sp["ir"]["frm"] + 8):
        if src[k].lstrip().startswith("define"):
            dl = src[k]
            break
    if not dl:
        return None
    i = dl.find("(", dl.find("@"))
    depth, k = 0, i
    while k < len(dl):
        if dl[k] == "(":
            depth += 1
        elif dl[k] == ")":
            depth -= 1
            if depth == 0:
                break
        k += 1
    out = []
    for a in ARGSPLIT.split(dl[i + 1:k]):
        a = a.strip()
        if a:
            out.append((a.split()[0], a))
    return out


def main():
    d = open(EXE, "rb").read()
    secs = sections(d)
    D = json.load(io.open(SPEC, encoding="utf-8"))["specs"]
    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    md.detail = False

    bad, ok = [], []
    for i in TARGET:
        sp = D[i]
        ex = sp.get("exe") or {}
        if not ex.get("addr"):
            print(u"#%02d %-34s RVA 없음 — 건너뜀" % (i, sp["name"][:34]))
            continue
        rva = int(ex["addr"], 16)
        n = int(ex.get("bytes") or 400)
        off = r2o(secs, rva)
        ins = list(md.disasm(d[off:off + n], 0x140000000 + rva))
        args = ir_args(sp) or []
        # 인자 레지스터별로 **역참조 여부**를 본다. 첫 40명령만(프롤로그 근처에서 갈린다).
        deref = {}
        for r in AREG:
            pats = [r] + list(SUB[r])
            for x in ins[:40]:
                o = x.op_str
                if ("[%s" % r) in o or ("[%s +" % r) in o or ("[%s+" % r) in o:
                    deref.setdefault(r, []).append(x)
        # 판정
        why = []
        for k, r in enumerate(AREG):
            if k >= len(args):
                break
            ty = args[k][0]
            if ty != "ptr" and r in deref:
                ex1 = deref[r][0]
                why.append(u"a%d: IR=`%s`(값) 인데 exe 가 `%s` 로 **역참조**(+0x%x)"
                           % (k, ty, ex1.op_str, ex1.address - (0x140000000 + rva)))
        tag = u"⛔ABI 불일치" if why else u"✅모순 없음"
        (bad if why else ok).append(i)
        print(u"\n#%02d %-34s 0x%-8x %d instrs   %s"
              % (i, sp["name"][:34], rva, len(ins), tag))
        print(u"    IR 인자: %s" % u" ".join(a[0] for a in args))
        print(u"    exe 역참조: %s"
              % (u" · ".join(u"%s(%d회)" % (r, len(v)) for r, v in sorted(deref.items())) or u"없음"))
        for w in why:
            print(u"    ★%s" % w)

    print(u"\n" + u"=" * 92)
    print(u"★ABI 불일치 **%d개** %s · 모순 없음 %d개 %s"
          % (len(bad), bad, len(ok), ok))
    print(u"⚠「모순 없음」은 **확정이 아니다** — IR 이 `ptr` 이라 한 자리를 exe 가 안 쓰는 것은")
    print(u"   증거가 못 된다(반증형 판정식). 실제 확인은 런타임 DIFF=0 이다.")


if __name__ == "__main__":
    main()
