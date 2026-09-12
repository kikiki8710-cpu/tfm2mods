#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""sweep20chk.py — **2단계(sweep) 대조가 가능한 함수가 몇 개인가**를 먼저 잰다. (2026-09-12)

## 왜 재고 시작하나
1단계(발화수)는 스텁이 레지스터·스택을 안 건드려서 **ABI 를 몰라도** 됐다.
2단계는 다르다 — **게임 함수와 내 링크사본을 같은 인자로 호출해 반환을 비교**해야 하므로
인자·반환 타입을 알아야 하고, 틀리면 **게임이 즉사**한다(`gensweep.py` 가 그래서 타입을 제한한다).

`gensweep.py` 의 현행 지원 범위:
  · 인자 = `ptr` / `i64` / `i32` / `i8` 만
  · 반환 = `i64` / `void` / `i1` 만
  · 320B 역참조 인자 = `StdRng` 로 보고 호출 전후로 떠서 되돌린다(그래야 난수열이 안 갈린다)

⟹ 이 범위 밖(sret·페어반환·부동소수·5인자+ 등)은 **전용 래퍼**를 따로 써야 한다.
   1단계에서 19/20 을 쟀다고 2단계도 19개가 되는 게 아니다. **그걸 먼저 숫자로 확인한다.**

★1단계 실측(판 종료 #1 확정치) = 발화 17 · 미발화 2(`#07`·`#15`).
   미발화는 **표본 불성립**이라 대조해도 의미가 없다 — 단 유저 지시로 **표에는 남겨 둔다**
   (「뜰 때까지 계속 본다」). 여기서는 **발화분 17개**의 ABI 적합성을 판정한다.
"""
import io
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.stdout.reconfigure(encoding="utf-8", errors="replace")

SPEC = r"C:\tfm2mods\MIG\_spec\specs20_v3.json"
IRDIR = r"C:\tfm2mods\_gaibc"
TBL = r"C:\tfm2mods\tfm2_judge_verify\src\probe20_tbl.rs"

OK_ARG = ("ptr", "i64", "i32", "i8")
OK_RET = ("i64", "void", "i1")

ARGSPLIT = re.compile(r",(?![^(]*\))")


def define_of(f, frm, to):
    p = os.path.join(IRDIR, f)
    src = io.open(p, encoding="utf-8", errors="replace").read().split("\n")
    for k in range(frm - 1, min(frm + 8, len(src))):
        if src[k].lstrip().startswith("define"):
            return src[k]
    return None


def parse(dl):
    u"""`define <ret> @sym(<args>)` → (반환형, [인자형…], sret여부)"""
    i = dl.find("@")
    head = dl[:i]
    m = re.search(r"define\s+(?:internal\s+|private\s+|fastcc\s+|noundef\s+|zeroext\s+|signext\s+|dso_local\s+)*"
                  r"([\w.]+(?:\s*\{[^}]*\})?)\s*$", head.strip())
    ret = (m.group(1).strip() if m else head.strip().split()[-1])
    j = dl.find("(", i)
    depth, k = 0, j
    while k < len(dl):
        if dl[k] == "(":
            depth += 1
        elif dl[k] == ")":
            depth -= 1
            if depth == 0:
                break
        k += 1
    inner = dl[j + 1:k]
    args, sret = [], False
    for a in ARGSPLIT.split(inner):
        a = a.strip()
        if not a:
            continue
        if "sret(" in a:
            sret = True
        args.append(a.split()[0])
    return ret, args, sret


def main():
    D = json.load(io.open(SPEC, encoding="utf-8"))["specs"]
    # 1단계 실측 — 판 종료 #1 확정치
    FIRED = {16: 105791090, 2: 20484329, 4: 2799265, 19: 2262947, 11: 2131572,
             18: 1344257, 8: 802140, 1: 484745, 10: 471270, 9: 265243,
             14: 137288, 13: 95813, 0: 93075, 6: 78345, 3: 32401, 5: 2764, 12: 250}
    DEAD = {7: u"미발화(11차 「발화 0 = 사장」 재현)", 15: u"미발화"}
    # `probe20_tbl.rs` 에서 실제 RVA 를 읽는다(정본은 표다 — 여기 베끼지 않는다)
    t = io.open(TBL, encoding="utf-8").read()
    RVA = dict((int(m.group(1)), m.group(2)) for m in
               re.finditer(r"idx:\s*(\d+),\s*rva:\s*0x([0-9a-fA-F]+)", t))
    RVA.update(dict((int(m.group(1)), m.group(2)) for m in
                    re.finditer(r"idx:\s*(\d+),\s*target_rva:\s*0x([0-9a-fA-F]+)", t)))

    ok, need, skip = [], [], []
    print(u"#  함수                                    호출수        반환      인자                     판정")
    print(u"-" * 118)
    for i, sp in enumerate(D):
        ir = sp.get("ir") or {}
        if not ir.get("file"):
            skip.append((i, sp["name"], u"IR 범위 없음"))
            continue
        dl = define_of(ir["file"], ir["frm"], ir["to"])
        if not dl:
            skip.append((i, sp["name"], u"define 못 찾음"))
            continue
        ret, args, sret = parse(dl)
        bad_a = [a for a in args if a not in OK_ARG]
        bad_r = ret not in OK_RET
        cnt = FIRED.get(i)
        cs = u"{:,}".format(cnt) if isinstance(cnt, int) else (u"·미발화" if i in DEAD else u"?")
        why = []
        if sret:
            why.append(u"sret")
        if bad_r:
            why.append(u"반환 %s" % ret)
        if bad_a:
            why.append(u"인자 %s" % u"/".join(sorted(set(bad_a))))
        if len(args) > 8:
            why.append(u"인자 %d개" % len(args))
        verdict = u"✅대조 가능" if not why else (u"⚠전용 래퍼 필요 — " + u" · ".join(why))
        (ok if not why else need).append((i, sp["name"], cnt, verdict))
        print(u"%02d %-38s %12s  %-8s %-24s %s"
              % (i, sp["name"][:38], cs, ret, u" ".join(args)[:24], verdict))

    print(u"-" * 118)
    fo = [x for x in ok if isinstance(x[2], int)]
    fn = [x for x in need if isinstance(x[2], int)]
    print(u"★**발화분 17개 중** — 표준 래퍼로 대조 가능 **%d** · 전용 래퍼 필요 **%d**" % (len(fo), len(fn)))
    if fn:
        print(u"\n전용 래퍼가 필요한 것(호출수 큰 순):")
        for i, nm, c, v in sorted(fn, key=lambda x: -(x[2] or 0)):
            print(u"   #%02d %-36s %12s  %s" % (i, nm[:36], u"{:,}".format(c), v))
    if skip:
        print(u"\n판정 불가: %s" % u", ".join(u"#%02d %s(%s)" % s for s in skip))


if __name__ == "__main__":
    main()
