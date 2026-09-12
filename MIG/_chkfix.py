# -*- coding: utf-8 -*-
u"""정정된 두 주소가 **내 검사 축(rvaverify S1·S2)을 통과하는가.** (2026-09-12)

## 왜 이걸 또 하나 — 「에이전트가 그렇다고 했다」는 근거가 아니다
`ghidra-re` 가 `#04`=`0xd3cfa0` · `#16`=`0xe0daa0` 이라고 확정했다. 그 근거는 ghidra 디컴파일이다.
그런데 **틀린 주소 2건도 원래는 「지문 매칭이 그렇다고 했다」였다**(`evidence:"fp"`).
⟹ 출처가 바뀌었을 뿐 「남이 그랬다」인 것은 같다. **내가 가진 독립 축으로 재확인**해야 한다.

S1(소스 줄 귀속)과 S2(스택 인자)는 ghidra 와 **다른 재료**를 쓴다:
  · S1 = `aimap.json`(DWARF 줄 정보) ↔ 명세 `src_line`(IR DISubprogram)
  · S2 = capstone 디스어셈 ↔ IR `define` 인자 개수
⟹ 이 둘이 정정값을 지지하면 **세 번째·네 번째 독립 증거**가 된다.

## 판정
구 주소와 신 주소를 **같은 화면에 놓고** 신호를 대조한다. 신 주소가 구 주소보다
**두 신호 모두에서 낫지 않으면** 정정을 받아들이지 않는다.
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
AIMAP = r"C:\tfm2mods\MIG\aimap.json"
IRDIR = r"C:\tfm2mods\_gaibc"

# (명세 idx, 구 RVA, 신 RVA, 구 주소의 정체)
CASES = [
    (4, 0xd3e4b0, 0xd3cfa0, u"has_line_defense_threat(4-arg)"),
    (16, 0xc809d0, 0xe0daa0, u"std::thread::LocalKey::with(TLS 메모)"),
]

ARGSPLIT = re.compile(r",(?![^(]*\))")
STK = re.compile(r"\[(?:rbp|rsp|r\d+)\s*([+\-])\s*(0x[0-9a-f]+)\]")


def sections(d):
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    n = struct.unpack_from("<H", d, pe + 6)[0]
    opt = struct.unpack_from("<H", d, pe + 20)[0]
    # ⚠순서 = (VirtualSize, VirtualAddress, SizeOfRawData, PointerToRawData)
    return [struct.unpack_from("<IIII", d, pe + 24 + opt + i * 40 + 8) for i in range(n)]


def r2o(secs, rva):
    for vsz, va, rsz, ra in secs:
        if va <= rva < va + max(vsz, rsz):
            return ra + (rva - va)
    return None


def ir_args(sp):
    p = os.path.join(IRDIR, sp["ir"]["file"])
    src = io.open(p, encoding="utf-8", errors="replace").read().split("\n")
    for k in range(sp["ir"]["frm"] - 1, sp["ir"]["frm"] + 8):
        if src[k].lstrip().startswith("define"):
            dl = src[k]
            i = dl.find("(", dl.find("@"))
            depth, j = 0, i
            while j < len(dl):
                if dl[j] == "(":
                    depth += 1
                elif dl[j] == ")":
                    depth -= 1
                    if depth == 0:
                        break
                j += 1
            return [a.strip() for a in ARGSPLIT.split(dl[i + 1:j]) if a.strip()]
    return []


def stack_args(d, secs, md, rva, nbytes=400):
    """프롤로그 이후 45명령 안에서 **양수 변위 >= 0x28** 인 스택 읽기 수."""
    off = r2o(secs, rva)
    if off is None:
        return None, 0
    ins = list(md.disasm(d[off:off + nbytes], 0x140000000 + rva))
    hits = []
    for x in ins[:45]:
        for sign, hx in STK.findall(x.op_str):
            if sign == "+" and int(hx, 16) >= 0x28:
                hits.append((x.address - (0x140000000 + rva), x.mnemonic, x.op_str))
    return len(ins), hits


def main():
    d = open(EXE, "rb").read()
    secs = sections(d)
    D = json.load(io.open(SPEC, encoding="utf-8"))["specs"]
    am = json.load(io.open(AIMAP, encoding="utf-8"))["info"]
    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    md.detail = False

    verdict = []
    for idx, old, new, whatold in CASES:
        sp = D[idx]
        sl = sp.get("src_line")
        args = ir_args(sp)
        print(u"\n" + u"=" * 96)
        print(u"#%02d %s   IR %d인자 · 명세 src_line %s" % (idx, sp["name"], len(args), sl))
        print(u"    구 주소 0x%x 의 정체 = %s" % (old, whatold))
        print(u"-" * 96)
        print(u"    %-10s %-22s %-10s %s" % (u"주소", u"aimap.lines", u"줄 거리", u"스택인자(>=0x28)"))
        row = {}
        for tag, rva in ((u"구", old), (u"신", new)):
            lines = (am.get(u"0x%x" % rva) or {}).get("lines") or []
            dist = min(abs(l - sl) for l in lines) if (lines and isinstance(sl, int)) else None
            n, hits = stack_args(d, secs, md, rva)
            row[tag] = (dist, len(hits), lines, n, hits)
            print(u"  %s 0x%-8x %-22s %-10s %d개" %
                  (tag, rva, unicode_list(lines), u"-" if dist is None else dist, len(hits)))
        # 판정 — 신 주소가 두 축 모두에서 나쁘지 않고, 하나 이상에서 나아야 한다
        # ★★unknown(항목 없음) 과 mismatch(값 어긋남) 를 **같은 칸에 넣지 마라.**
        #   초판이 신 주소의 `aimap.lines == []`(= 그 주소가 640 후보 집합 **밖**이라 정보가 없음)을
        #   「개선 없음 ⟹ 지지 못함」으로 찍어 **정답 2건을 오답으로 판정**했다(2026-09-12).
        #   검사기는 **「모른다」를 낼 수 있어야 한다** — 없으면 후보 집합에 정답이 없을 때
        #   무언가를 고르고 마는 지문 매칭과 **같은 결함**이 된다(그게 이 오염의 기계적 원인이었다).
        do, so, lo, _, _ = row[u"구"]
        dn, sn, ln, _, hn = row[u"신"]
        need = len(args) >= 5
        if not ln:                       # 신 주소에 줄 정보가 **없다** = 판정 불가(≠ 어긋남)
            s1 = None
            s1txt = (u"🟡**판정 불가** — 신 주소가 aimap(640 후보) **밖**이라 줄 정보가 없다.\n"
                     u"                  ⟹ 「어긋났다」가 아니라 「모른다」다. S1 은 이 건에 쓸 수 없다\n"
                     u"                  ⟹ ★그리고 **정답이 후보 집합 밖이라는 사실 자체**가 오염의 원인이다"
                     u"(→ callerprof.py S4/S5)")
        elif do is None or (dn is not None and dn < do):
            s1 = True
            s1txt = u"✅개선 (거리 %s → %s)" % (do, dn)
        else:
            s1 = False
            s1txt = u"⛔어긋남 (구 %s · 신 %s — 신 주소가 더 멀다)" % (do, dn)
        s2 = (sn > 0) if need else None
        print(u"\n    S1 줄 귀속  : %s" % s1txt)
        if need:
            print(u"    S2 스택 인자: %s (IR %d인자 ⟹ 스택 읽기가 있어야 한다)" % (
                u"✅있음 %d개" % sn if s2 else u"⛔없음", len(args)))
            for o, m, s in hn[:4]:
                print(u"         +0x%-4x %-7s %s" % (o, m, s))
        else:
            print(u"    S2 스택 인자: — (IR %d인자 ⟹ 레지스터로 충분, 판정 대상 아님)" % len(args))
        # ★기각은 **명시적 반증**이 있을 때만. unknown 은 기각 근거가 아니다(구제·기각 비대칭).
        if s1 is False or s2 is False:
            ok, vt = False, u"⛔**반증됨 — 재조사 필요**"
        elif s1 is True or s2 is True:
            ok, vt = True, u"✅**정정값 지지**"
        else:
            ok, vt = None, u"🟡**판정 보류** — 두 축 모두 「모른다」다(S4/S5 로 넘길 것)"
        verdict.append((idx, ok))
        print(u"\n    ★판정: %s" % vt)

    print(u"\n" + u"=" * 96)
    print(u"지지 %d / %d  %s" % (sum(1 for _, o in verdict if o), len(verdict), verdict))
    print(u"⚠이 검사는 **S1·S2 두 축**만 본다. 통과해도 「같은 함수임」의 최종 증명은 아니다 —")
    print(u"   최종 심판은 여전히 **런타임 DIFF=0** 이다(그게 나면 주소도 동시에 증명된다).")


def unicode_list(v):
    return u"[]" if not v else (u"[%s]" % u",".join(str(x) for x in v[:4]))


if __name__ == "__main__":
    main()
