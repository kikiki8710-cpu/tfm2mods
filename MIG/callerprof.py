# -*- coding: utf-8 -*-
u"""callerprof.py — **「누가 이 주소를 부르는가」로 주소를 검증한다**(신호 S4). (2026-09-12 신설)

## 왜 이 축이 필요한가 — 오염의 메커니즘을 찾았다
`#04`·`#16` 의 정답 주소(`0xd3cfa0`·`0xe0daa0`)는 `aimap.json[info]`(**640개**)에 **없다.**
`decomp\<ver>\**.md` 의 **콜리 목록에만** 나온다(이름 주석도 없다).

⟹ ★**후보 집합이 불완전한데 「가장 닮은 것」을 고르게 하면, 정답이 집합에 없을 때 반드시 오답이 뽑힌다.**
   명세 `exe.addr` 의 `evidence:"fp"`(지문 매칭)는 **「해당 없음」을 낼 수 없는 판정기**였다.
   이것이 15건 중 14건이 추정이 된 이유이자, 2건이 실제로 다른 함수를 가리킨 **기계적 원인**이다.
   (12차 감사의 「후보 1개 = 정답 오류 28%」와 **같은 결함의 다른 얼굴**이다.)

## S4 = 역호출 프로파일
`aimap.json[callees]` 를 뒤집어 **호출자 집합**을 만들고, 그 호출자들의 **모듈 분포**를 본다.
  · 주장된 주소의 호출자가 **명세가 말하는 모듈과 무관한 곳**뿐이면 → 의심.
  · 반대로 **여러 서브플랜이 공유**하면 유틸, **한 모듈만** 부르면 그 모듈 전용 — 역할이 드러난다.

⟹ `aimap.lines`(S1) 와 ghidra 디컴(별개 재료) 어느 쪽에도 의존하지 않는 **독립 축**이다.

## ★부수 산출 — 정답 후보 발굴
`info` 에 없으면서(=이름 미부여) **명세 모듈의 decomp 파일이 호출하는** 주소들을 뽑는다.
정답이 640 집합 밖이었다는 것이 실증됐으므로, **후보는 거기서 찾아야 한다.**

⚠한계: `decomp` 콜리는 **직접 `call` 만** 담는다(간접·vtable 누락 = 하한).
   ⟹ 「호출자 0건」은 「안 불린다」가 아니라 **「직접 호출이 안 잡혔다」**다. 기각 근거로 쓰지 마라.

## ★★`aimap.json[callees]` 를 쓰지 않고 `decomp` 를 직접 읽는 이유
`aimap.py` 는 마지막에 **`callees[k] &= set(info)`** 를 한다(「AI 계층 밖 호출은 버린다」).
모듈 지도를 만드는 원래 목적엔 맞지만, **여기서는 찾아야 할 증거를 정확히 지운다** —
정답 주소가 640 집합 밖이므로 **그 필터를 통과하지 못하고 호출자 0건으로 보인다**(실측 확인).
⟹ ★**「도구의 상태」를 「세계의 상태」로 착각하는 전형**(METHOD_MAP §0 의 실패 유형 ②).
   그래서 이 도구는 **필터 이전의 원본**(`decomp\<ver>\**.md` 의 `**콜리**` 줄)을 직접 파싱한다.
"""
import io
import json
import os
import re
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
SPEC = r"C:\tfm2mods\MIG\_spec\specs20_v3.json"
AIMAP = r"C:\tfm2mods\MIG\aimap.json"
DEC = r"C:\tfm2mods\MIG\decomp"
VER = u"0.5.8"


def load_raw_graph():
    u"""decomp 원본에서 **필터 없는** (owner_mod, callees) 를 만든다.

    반환: secmod[rva] = 모듈경로 · rawcallers[rva] = [(호출자rva, 호출자모듈, 횟수)]
    """
    root = os.path.join(DEC, VER)
    secmod, rawcallers = {}, defaultdict(list)
    hdr = re.compile(r"^## `(0x[0-9a-f]+)`")
    cnt = re.compile(r"`(0x[0-9a-f]+)`(?:\([^)]*\))?(?:×(\d+))?")
    for dp, _, files in os.walk(root):
        for fn in files:
            if not fn.endswith(".md") or fn == "INDEX.md":
                continue
            mod = os.path.relpath(os.path.join(dp, fn), root).replace("\\", "/")[:-3]
            cur = None
            for line in io.open(os.path.join(dp, fn), encoding="utf-8",
                                errors="ignore"):
                m = hdr.match(line)
                if m:
                    cur = m.group(1)
                    secmod[cur] = mod
                    continue
                if cur and line.startswith(u"**콜리**"):
                    for rva, n in cnt.findall(line):
                        rawcallers[rva].append((cur, mod, int(n or 1)))
    return secmod, rawcallers

# rvaverify.py 가 의심으로 찍은 5건 + 확정 정정 2건 + 런타임 증명 3건
SUSPECT = [10, 11, 12, 15, 18]
FIXED = {4: 0xd3cfa0, 16: 0xe0daa0}
PROVEN = {9: 254119, 1: 481514, 8: 761970}


def main():
    am = json.load(io.open(AIMAP, encoding="utf-8"))
    info = am["info"]
    D = json.load(io.open(SPEC, encoding="utf-8"))["specs"]
    secmod, rawcallers = load_raw_graph()
    print(u"(decomp 원본 파싱: 섹션 %d개 · 호출 대상 %d개 — aimap info 는 %d개)"
          % (len(secmod), len(rawcallers), len(info)))

    def modof(rva):
        return (info.get(rva) or {}).get("mod") or secmod.get(rva) or u"?(집합 밖)"

    def prof(rva):
        cs = rawcallers.get(rva, [])
        agg = defaultdict(int)
        for _crva, cmod, n in cs:
            agg[cmod] += n
        return sorted(agg.items(), key=lambda kv: -kv[1]), sum(n for _, _, n in cs)

    print(u"=" * 104)
    print(u"S4 역호출 프로파일 — 「누가 부르는가」로 주소를 검증한다")
    print(u"=" * 104)
    print(u"\n【1】확정 정정 2건 — 구 주소 vs 신 주소의 호출자 구성\n")
    for i, new in sorted(FIXED.items()):
        sp = D[i]
        old = u"0x%x" % int(sp["exe"]["addr"], 16)
        print(u"#%02d %s   (명세 모듈 = %s)" % (i, sp["name"], sp.get("mod") or u"?"))
        for tag, rva in ((u"구", old), (u"신", u"0x%x" % new)):
            pr, n = prof(rva)
            inset = u"info 있음" if rva in info else u"**info 밖**"
            print(u"   %s %-10s [%s] 호출자 %d개 : %s"
                  % (tag, rva, inset, n,
                     u" · ".join(u"%s×%d" % (m, c) for m, c in pr[:6]) or u"(직접 호출 0건)"))
        print()

    print(u"\n【2】런타임 증명 3건(DIFF=0) — 정상 프로파일이 어떤 모양인지 기준선\n")
    for i in sorted(PROVEN):
        sp = D[i]
        rva = u"0x%x" % int(sp["exe"]["addr"], 16)
        pr, n = prof(rva)
        print(u"#%02d %-42s %-10s 호출자 %2d : %s"
              % (i, sp["name"][:42], rva, n,
                 u" · ".join(u"%s×%d" % (m, c) for m, c in pr[:5]) or u"(직접 호출 0건)"))

    print(u"\n\n【3】★의심 5건 — 주장된 주소의 호출자가 명세 모듈과 맞는가\n")
    for i in SUSPECT:
        sp = D[i]
        ex = sp.get("exe") or {}
        if not ex.get("addr"):
            print(u"#%02d %s — RVA 없음" % (i, sp["name"]))
            continue
        rva = u"0x%x" % int(ex["addr"], 16)
        pr, n = prof(rva)
        mod = modof(rva)
        print(u"#%02d %s" % (i, sp["name"]))
        print(u"    주장 %s · 그 주소의 소속모듈 = **%s** · 명세 src_line %s · evidence=%s"
              % (rva, mod, sp.get("src_line"), ex.get("evidence")))
        print(u"    호출자 %d개 : %s"
              % (n, u" · ".join(u"%s×%d" % (m, c) for m, c in pr[:6]) or u"(직접 호출 0건)"))
        print()

    # ── S5 = 발화수 ↔ 직접 호출자 수 모순 ────────────────────────────────
    # 판당 수백만 번 불리는 함수에 **직접 호출자가 0개**면 모순이다.
    # (vtable·간접 호출만으로 불리는 경우가 있으니 확정은 아니지만, 발화수가 클수록 강해진다.)
    FIRE = {16: 105791090, 2: 20484329, 4: 2799265, 19: 2262947, 11: 2131572,
            18: 1344257, 8: 802140, 1: 484745, 10: 471270, 9: 265243,
            14: 137288, 13: 95813, 0: 93075, 6: 78345, 3: 32401, 5: 2764,
            12: 250, 7: 0, 15: 0}
    print(u"\n【5】★S5 = 발화수 ↔ 직접 호출자 수 **모순** 검사")
    print(u"    (판당 수백만 번 불리는 함수에 직접 호출자가 0개면 모순이다)\n")
    print(u"    %-4s %-42s %-10s %-14s %-8s %s"
          % (u"#", u"함수", u"주장 addr", u"1단계 발화", u"호출자", u"판정"))
    rows = []
    for i, sp in enumerate(D):
        ex = sp.get("exe") or {}
        if not ex.get("addr"):
            continue
        rva = u"0x%x" % int(ex["addr"], 16)
        _pr, n = prof(rva)
        f = FIRE.get(i)
        if f is None:
            continue
        if f >= 100000 and n == 0:
            tag = u"⛔**모순** — 대량 발화인데 직접 호출자 0"
        elif n == 0 and f == 0:
            tag = u"⚠양쪽 0 (호출자도 발화도 없다 — 주소 자체를 의심하라)"
        elif n == 0:
            tag = u"🟡호출자 0 (발화 소량 — 간접호출 가능)"
        else:
            tag = u"✅호출자 있음"
        rows.append((i, tag))
        print(u"    %-4d %-42s %-10s %-14s %-8d %s"
              % (i, sp["name"][:42], rva, u"{:,}".format(f), n, tag))
    bad = [i for i, t in rows if t.startswith(u"⛔")]
    print(u"\n    ⟹ ★**모순 %d건** %s" % (len(bad), bad))
    print(u"    ⚠이 신호의 성격: **구제가 아니라 적발**용이다. 그런데 「직접 호출자 0」은 간접호출로")
    print(u"       설명될 수 있으므로 **단독 기각 근거로는 쓰지 마라** — 다른 축과 합쳐서만 판정하라.")
    print(u"    ★실측 대조: 확정 오류 2건(`0xd3e4b0`·`0xc809d0`)은 **둘 다 호출자 0** 이었고,")
    print(u"       DIFF=0 로 증명된 3건은 **셋 다 호출자 있음**(2·1·7)이었다.")

    print(u"\n【4】참고 — 640 집합의 성격")
    inset = sum(1 for sp in D if (sp.get("exe") or {}).get("addr")
                and (u"0x%x" % int(sp["exe"]["addr"], 16)) in info)
    have = sum(1 for sp in D if (sp.get("exe") or {}).get("addr"))
    print(u"    명세 addr 보유 %d건 중 **%d건이 info(640) 안**에 있다." % (have, inset))
    print(u"    그런데 확정된 정답 2건은 **둘 다 info 밖**이었다.")
    print(u"    ⟹ ★후보 집합이 정답을 포함한다는 보장이 없는데 「가장 닮은 것」을 뽑았다 =")
    print(u"       **「해당 없음」을 낼 수 없는 판정기**. 이것이 `evidence:\"fp\"` 오염의 기계적 원인이다.")
    print(u"\n⚠S4 의 한계: decomp 콜리는 **직접 call 만** 담는다(간접·vtable 누락).")
    print(u"   「호출자 0건」은 기각 근거가 못 된다 — 구제(지지)에만 쓰라.")


if __name__ == "__main__":
    main()
