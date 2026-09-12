#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""mkfnexplain.py — 판단함수 「내용 설명」 문서 생성기 (2026-09-13 신설)

정본 `_spec\\specs20_v3.json` 에서 사람이 읽는 설명(한 줄 요약 · 인자 역할 · 읽는/쓰는 상태 ·
판정 의사코드 · 조절값 · 호출 관계 · 남은 물음)을 함수마다 뽑아 한 문서로 만든다.
명세 밖 함수(#21~#24)는 `_spec\\explain_extra.md`(손 작성 · IR 독해 결과)를 그대로 뒤에 붙인다.
런타임 상태(ev1)는 `REPORT\\tfm2_judge_verify\\00_상태원장.md §1` 표에서 읽는다.

사용:  python -X utf8 MIG\\mkfnexplain.py            → REPORT\\tfm2_judge_verify\\05_함수내용_설명_00~24.md
       python -X utf8 MIG\\mkfnexplain.py --out <경로>
⛔생성물을 손으로 고치지 말 것 — 명세가 정본. 틀리면 applypatch 로 명세를 고치고 재생성.
"""
import io, json, os, re, sys, time

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
V3 = os.path.join(HERE, "_spec", "specs20_v3.json")
EXTRA = os.path.join(HERE, "_spec", "explain_extra.md")
REP = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_judge_verify"
LEDGER = os.path.join(REP, u"00_상태원장.md")
OUT = os.path.join(REP, u"05_함수내용_설명_00~24.md")
SKIP_CALLEE = ("core::", "std::", "alloc::", "compiler_builtins", "panic", "unwrap_failed")


BOILER = re.compile(u"\\s*·\\s*(tcx 정본 대조|오라클 실행 확증|tcxdict \\*\\*tcx 정본\\*\\*)\\(.*$")


def cut(s, n):
    u"""공백 정규화 + 검증 상용구(「· tcx 정본 대조(…)」「· 오라클 실행 확증(…)」) 제거 + 길이 절단.
    상용구는 명세엔 근거로 필요하지만 설명 문서에선 줄마다 반복되는 잡음이다(제거 전 323KB)."""
    s = re.sub(r"\s+", " ", (s or "")).strip()
    s = BOILER.sub(u"", s)
    return s if len(s) <= n else s[:n - 1] + u"…"


def ledger_status():
    u"""원장 §1 표 → {idx: (RVA셀, ev1셀, 기구셀)}"""
    st = {}
    if not os.path.exists(LEDGER):
        return st
    for l in io.open(LEDGER, encoding="utf-8"):
        m = re.match(r"^\|\s*(\d{2})\s*\|", l)
        if not m:
            continue
        c = [x.strip() for x in l.strip().strip("|").split("|")]
        if len(c) >= 8:
            st[int(m.group(1))] = (c[2], c[5], c[7])
    return st


def fn_section(s, st):
    i = s["i"]
    L = []
    rva, ev1, mech = st.get(i, (u"(원장에 없음)", u"—", u"—"))
    L.append(u"## #%02d `%s` — %s" % (i, s["name"], cut(s.get("one_line"), 400)))
    L.append(u"")
    L.append(u"| 항목 | 값 |\n|---|---|")
    L.append(u"| 소스 · 계층 | `%s:%s` · %s · IR `%s:%s~%s` |" % (s.get("src"), s.get("src_line"), s.get("layer"),
             s["ir"].get("file"), s["ir"].get("frm"), s["ir"].get("to")))
    L.append(u"| exe · 런타임(ev1) | %s · **%s** · 기구: %s |" % (cut(rva, 120), cut(ev1, 40), cut(mech, 120)))
    sig = s.get("sig") or {}
    L.append(u"| 시그니처(tcx) | `%s` · vis %s |" % (sig.get("tcx"), sig.get("vis")))
    callers = s.get("callers") or {}
    L.append(u"| 호출자 | %s곳(_gaibc 실측) |" % callers.get("count", "?"))
    L.append(u"")
    # 인자
    params = sig.get("params") or []
    if params:
        L.append(u"**입력(인자 역할)**")
        for p in params:
            L.append(u"- `%s` %s — %s" % (p.get("name"), cut(p.get("type"), 60), cut(p.get("role"), 160)))
        L.append(u"")
    # 상태
    mem = s.get("mem") or []
    r = [m for m in mem if "r" in (m.get("dir") or "")]
    w = [m for m in mem if "w" in (m.get("dir") or "")]
    def memrow(m):
        return u"- `%s.%s` @%s — %s" % (m.get("base"), m.get("name"), m.get("offset"), cut(m.get("note"), 110))
    if r:
        L.append(u"**읽는 상태**(%d)" % len(r))
        L += [memrow(m) for m in r[:30]]
        if len(r) > 30: L.append(u"- … +%d (명세 `mem`)" % (len(r) - 30))
        L.append(u"")
    if w:
        L.append(u"**쓰는 상태**(%d)" % len(w))
        L += [memrow(m) for m in w[:30]]
        if len(w) > 30: L.append(u"- … +%d" % (len(w) - 30))
        L.append(u"")
    # 판정 흐름
    L.append(u"**판정 흐름(재구현용 의사코드 · 오프셋·상수의 정본은 위 표/명세)**")
    L.append(u"```rust")
    L.append((s.get("logic") or u"(없음)").rstrip())
    L.append(u"```")
    L.append(u"")
    # 노브
    kn = s.get("knobs") or []
    if kn:
        L.append(u"**조절값(노브)** — 값 · 위치 · 바꾸면")
        L.append(u"| 무엇 | 값 | 위치 | 효과 | ev |\n|---|---|---|---|---|")
        for k in kn:
            L.append(u"| %s | `%s` | %s | %s | %s |" % (cut(k.get("what"), 40), k.get("value"), cut(k.get("where"), 40), cut(k.get("effect"), 150), k.get("ev", "")))
        L.append(u"")
    # 상수
    cs = s.get("consts") or []
    if cs:
        L.append(u"**상수**(%d)" % len(cs))
        for c in cs[:20]:
            L.append(u"- `%s` (%s · L%s) — %s" % (c.get("value"), c.get("kind", ""), c.get("src_line"), cut(c.get("meaning"), 120)))
        if len(cs) > 20: L.append(u"- … +%d" % (len(cs) - 20))
        L.append(u"")
    # 호출
    ce = []
    for c in s.get("callees") or []:
        p = c.get("path") or c.get("name") or ""
        if any(x in p for x in SKIP_CALLEE) or c.get("name") == s["name"]:
            continue
        if p not in ce: ce.append(p)
    if ce:
        L.append(u"**호출하는 게임 함수**: " + u" · ".join(u"`%s`" % x for x in ce[:25]) + (u" · …+%d" % (len(ce) - 25) if len(ce) > 25 else u""))
        L.append(u"")
    # 남은 물음
    op = s.get("open") or []
    if op:
        L.append(u"**남은 물음(open %d)**" % len(op))
        for o in op[:6]:
            L.append(u"- %s" % cut(o.get("q"), 200))
        L.append(u"")
    L.append(u"---")
    L.append(u"")
    return L


def main():
    out = OUT
    if "--out" in sys.argv:
        out = sys.argv[sys.argv.index("--out") + 1]
    D = json.load(io.open(V3, encoding="utf-8"))
    st = ledger_status()
    specs = D["specs"]
    H = []
    H.append(u"# 판단함수 내용 설명 — #00~#24 (게임 0.5.8 · 생성 %s)" % time.strftime("%Y-%m-%d %H:%M"))
    H.append(u"")
    H.append(u"> **생성물 — 손편집 금지.** 정본 = `MIG\\_spec\\specs20_v3.json`(#00~#19) + `MIG\\_spec\\explain_extra.md`(#21~#24 · IR 독해) + 런타임 상태 = `00_상태원장.md §1`. 재생성: `python -X utf8 MIG\\mkfnexplain.py`. 틀린 곳은 `applypatch.py` 로 명세를 고친 뒤 재생성.")
    H.append(u"> **읽는 법**: 함수마다 ①한 줄 요약 ②소스·RVA·ev1 ③인자 역할 ④읽는/쓰는 게임 상태(오프셋) ⑤판정 흐름(의사코드) ⑥조절값(노브) ⑦상수 ⑧호출 관계 ⑨남은 물음. 전문(근거·ev·history 포함)은 `REPORT\\tfm2_ai_adjust\\06_판단함수_명세_v3.md`. 증거 등급 ev = 1 런타임 / 2 오라클 / 3 tcx / 4 IR / 5 추론.")
    H.append(u"> #17 `DeathMatchBattle::new` 는 데스매치 전용(MOBA 미사용 · 원장 분모 제외)이지만 명세는 있어 수록. #20 은 없음(다음 대상 `LegacyPlanHandler::update` 예정 자리).")
    H.append(u"")
    H.append(u"## 목차")
    H.append(u"| # | 함수 | 계층 | 한 줄 | ev1 |\n|---|---|---|---|---|")
    for s in specs:
        rva, ev1, mech = st.get(s["i"], ("", u"—", ""))
        H.append(u"| %02d | `%s` | %s | %s | %s |" % (s["i"], s["name"], s.get("layer"), cut(s.get("one_line"), 90), cut(ev1, 20)))
    extra_txt = io.open(EXTRA, encoding="utf-8").read() if os.path.exists(EXTRA) else u""
    for m in re.finditer(r"^### #(\d{2}) `([^`]+)` — (.+)$", extra_txt, re.M):
        H.append(u"| %s | `%s` | 명세 밖(IR 독해) | %s | ✅ DIFF 0 |" % (m.group(1), m.group(2), cut(m.group(3), 90)))
    H.append(u"")
    H.append(u"---")
    H.append(u"")
    body = []
    for s in specs:
        body += fn_section(s, st)
    tail = []
    if extra_txt:
        tail.append(u"# 명세 밖 4함수 (#21~#24) — `#18` 의 피호출자 · IR 독해 (2026-09-13)")
        tail.append(u"")
        tail.append(u"> 출처 = `MIG\\_spec\\explain_extra.md`(RE 원문 = `REPORT\\tfm2_judge_verify\\RE\\2026-09-13_21~24_IR독해_내용명세.md`). 20함수 명세와 달리 **반증검증 라운드를 거치지 않았다**(ev4 · 1회 독해). 런타임 DIFF 0 은 원장 §1 참조.")
        tail.append(u"")
        tail.append(extra_txt.rstrip())
        tail.append(u"")
    txt = u"\n".join(H + body + tail)
    io.open(out, "w", encoding="utf-8", newline="\n").write(txt)
    print(u"%s  %dB · 함수 %d + extra %s" % (out, len(txt.encode("utf-8")), len(specs), u"있음" if extra_txt else u"없음"))


if __name__ == "__main__":
    main()
