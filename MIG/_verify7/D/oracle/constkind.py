# -*- coding: utf-8 -*-
u"""`consts[].kind` 정합 검사 — **IR 이 그 리터럴을 어떻게 쓰는가**와 대조한다.

★먼저 확인한 사실(7차 배치D): `kind` 는 사람이 쓰는 축이 **아니다**.
`mkspec3.py:329` 가 `meaning` 문면을 키워드 매칭해 파생시킨다 —
    "태그|판별자|variant" → 태그  >  "센티널|니치|0xff|MAX" → 센티널  >  "인덱스" → 인덱스  >  (그 외) 임계
⟹ `kind` 오류 = ①`meaning` 문면 문제 ②분류기 우선순위 버그. **IR 과 직접 대조되는 축이 아니다.**
그래서 이 도구는 IR 이 말하는 **용법 부류**를 따로 뽑아 파생값과 붙여 본다.

IR 용법 부류
  SWITCH : `switch iN %x, label %d [ iN VAL, label ... ]` 의 case 값 → 판별자 디스패치
  EQ     : `icmp eq|ne iN %x, VAL`                                  → 동치 비교(태그 또는 센티널)
  REL    : `icmp ult|ule|ugt|uge|slt|sle|sgt|sge iN %x, VAL`        → 임계 비교
  STORE  : `store iN VAL, ptr ...`                                  → 값 저장(태그 저장 포함)
  ARITH  : `add|sub|mul|udiv|urem|shl|... VAL`                      → 산술 상수
  GEP    : `getelementptr ..., i64 VAL`                             → 인덱스/stride
  SELECT : `select i1 %c, iN VAL, iN VAL2`                          → 두 값 중 택일

판정 규칙(보수적 — 결함만 센다)
  R1 센티널 오분류 : 값이 -1/255/i64::MAX 인데 `kind`=태그 → 니치 센티널을 태그로 부른 것
  R2 태그 누락     : IR 용법에 SWITCH 가 있는데 `kind`=임계 → 판별자인데 임계로 부른 것
  R3 임계 아님     : IR 용법에 REL 이 **하나도 없는데** `kind`=임계 → 임계라 부를 근거 없음

용법: python -X utf8 constkind.py [specidx ...]
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
IRDIR = r"C:\tfm2mods\_gaibc"

SENT = {"-1", "255", "9223372036854775807", "18446744073709551615", "-1.0"}


def usage(f, a, b, val):
    src = io.open(os.path.join(IRDIR, f), encoding="utf-8", errors="replace").read().split("\n")
    v = re.escape(str(val))
    pat_case = re.compile(r"^\s*i\d+ " + v + r", label ")
    pat_eq = re.compile(r"icmp (eq|ne) i\d+ [^,]+, " + v + r"(?![\w.])")
    pat_rel = re.compile(r"icmp (ult|ule|ugt|uge|slt|sle|sgt|sge) i\d+ [^,]+, " + v + r"(?![\w.])")
    pat_st = re.compile(r"^\s*store i\d+ " + v + r"(?![\w.]), ptr")
    pat_ar = re.compile(r"= (add|sub|mul|udiv|urem|sdiv|shl|lshr|ashr|and|or|xor)\b[^=]*?"
                        r"(?<![\w.\-])" + v + r"(?![\w.])")
    pat_gep = re.compile(r"getelementptr[^=]*, i\d+ " + v + r"(?![\w.])")
    pat_sel = re.compile(r"= select i1 [^,]+, i\d+ [^,]*?(?<![\w.\-])" + v + r"(?![\w.])")
    out = set()
    inswitch = False
    for k in range(a - 1, min(b, len(src))):
        ln = src[k]
        if "#dbg_" in ln:
            continue
        st = ln.strip()
        if st.startswith("switch "):
            inswitch = True
        if inswitch:
            if pat_case.match(ln):
                out.add("SWITCH")
            if "]" in st:
                inswitch = False
        if pat_eq.search(ln):
            out.add("EQ")
        if pat_rel.search(ln):
            out.add("REL")
        if pat_st.match(ln):
            out.add("STORE")
        if pat_sel.search(ln):
            out.add("SELECT")
        elif pat_ar.search(ln):
            out.add("ARITH")
        if pat_gep.search(ln):
            out.add("GEP")
    return out


def main():
    D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
    idxs = [int(x) for x in sys.argv[1:] if x.isdigit()] or list(range(20))
    n = r1 = r2 = r3 = 0
    for i in idxs:
        sp = D["specs"][i]
        ir = sp.get("ir") or {}
        f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
        if not f:
            continue
        hdr = False
        for j, c in enumerate(sp.get("consts") or []):
            val, kind = c.get("value"), (c.get("kind") or "")
            u = usage(f, a, b, val)
            n += 1
            bad = []
            if str(val) in SENT and kind == u"태그":
                bad.append(u"R1 센티널을 태그로"); r1 += 1
            if "SWITCH" in u and "REL" not in u and kind == u"임계":
                bad.append(u"R2 판별자를 임계로"); r2 += 1
            if kind == u"임계" and u and not (u & {"REL", "EQ", "SWITCH"}):
                bad.append(u"R3 비교에 안 쓰인다(임계 근거 없음)"); r3 += 1
            if bad:
                if not hdr:
                    print(u"\n===== specs[%d] %s =====" % (i, sp["name"])); hdr = True
                print(u"  consts[%-2d] value=%-22s kind=%-4s IR용법=%-28s %s"
                      % (j, val, kind, ",".join(sorted(u)) or u"(없음)", u" / ".join(bad)))
    print(u"\n---- 검사 %d건 · R1 %d · R2 %d · R3 %d" % (n, r1, r2, r3))


main()
