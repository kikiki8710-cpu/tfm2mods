# -*- coding: utf-8 -*-
u"""addrgate.py — **`exe.addr` 을 지문 점수로 게이트한다**(신호 S6). (2026-09-12 신설)

## 왜 이게 가장 센 신호인가 — 숫자가 완벽하게 갈렸다
`ghidra-re` 가 20건을 전수 대조한 결과(2026-09-12):

    맞은 주소 : 1.00 · 1.00 · 1.00 · 1.00 · 1.00 · 1.00 · 0.95 · **0.93**
    틀린 주소 : **0.50** · 0.25 · 0.23 · 0.22 · 0.14 · 0.12

⟹ **0.93 과 0.50 사이에 겹침이 없다.** 즉 `dllmatch.jaccard` 는 **처음부터 답을 알고 있었고**,
   명세를 만들 때 **점수를 보지 않고 1등만 취한 것**이 사고의 직접 원인이다.

★교훈: **「1등」은 판정이 아니다.** 순위는 후보 중 상대적 위치일 뿐이고, 후보에 정답이 없으면
  1등은 그냥 「가장 덜 틀린 것」이다. 판정으로 쓸 수 있는 것은 **절대 점수와 그 임계값**이다.

## 왜 지문이 작은 함수에서 무너지나
지문 = 오프셋·즉시값 집합의 Jaccard. 100~350 명령짜리 함수는 **원소가 몇 개뿐**이라
같은 모듈로 집계된 아무 helper 가 최고점을 먹는다(`cg.py` docstring 이 이미 경고해 둔 사항이다).
⟹ 크기가 작을수록 임계값을 높여야 한다.

## 게이트
  · `jaccard >= 0.9`  → ✅주소로 써도 된다
  · `0.5 <= j < 0.9`  → ⛔**미확정**(주소로 쓰지 말고 호출그래프·고유상수로 간다)
  · `j < 0.5`         → ⛔⛔거의 확실히 틀렸다
  · dllmatch 에 **없음** → 🟡판정 불가(다른 출처에서 온 주소다 — unknown ≠ mismatch)

## 부수 — `percolate.json` 과의 불일치 검출
`percolate`(지문 + **호출그래프**)가 `dllmatch`(지문 단독)와 다른 주소를 말하면 **percolate 를 믿는다**.
실증: `handle_chat` 은 percolate 에 `0xe595b0`(jac 1.0)으로 **이미 있었는데** 명세는 dllmatch(0.14)를 썼다.
"""
import io
import json
import sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
SPEC = r"C:\tfm2mods\MIG\_spec\specs20_v3.json"
DLLM = r"C:\tfm2mods\MIG\dllmatch.json"
PERC = r"C:\tfm2mods\MIG\percolate.json"

# ghidra-re 2026-09-12 확정. None = 명세 주소가 맞다.
TRUTH = {4: 0xd3cfa0, 16: 0xe0daa0, 10: 0xe0c560, 11: 0xe4b5d0,
         12: 0xe595b0, 15: 0xe5c1f0, 18: None, 9: None, 1: None, 8: None,
         13: None}
# 런타임 DIFF=0 = 주소의 증명
PROVEN = {9, 1, 8}


def main():
    D = json.load(io.open(SPEC, encoding="utf-8"))["specs"]
    dm = json.load(io.open(DLLM, encoding="utf-8"))
    pc = json.load(io.open(PERC, encoding="utf-8"))["rows"]

    by_name = {}
    for r in dm:
        by_name.setdefault(r.get("name"), []).append(r)
    # ⚠percolate 행에는 `name` 키가 **없다**(addr/rva/mangled/aliases 뿐).
    #   초판이 `r.get("name")` 으로 찾아 **전부 「없음」으로 오보**했다(2026-09-12).
    #   ⟹ 「조회가 실패한 것」을 「데이터에 없는 것」으로 보고하면, 가진 답을 없다고 적는다
    #      (= 이 세션에서 이미 두 번 나온 unknown↦mismatch 결함의 세 번째 출현).
    #   mangled 은 rust v0 이라 이름이 **길이 접두**로 박힌다: `..13handle_chat..` ⟹ `<len><name>` 검색.
    perc_by_name = {}
    for r in pc:
        for mg in [r.get("mangled") or u""] + list(r.get("aliases") or []):
            perc_by_name.setdefault(mg, []).append(r)

    def perc_find(nm):
        key = u"%d%s" % (len(nm), nm)
        out = []
        for mg, rs in perc_by_name.items():
            if key in mg:
                out.extend(rs)
        return out

    print(u"=" * 116)
    print(u"S6 지문 점수 게이트 — 「1등」이 아니라 **절대 점수**로 판정한다")
    print(u"=" * 116)
    print(u"%-4s %-42s %-11s %-7s %-9s %-9s %s"
          % (u"#", u"함수", u"명세 addr", u"jac", u"contain", u"bytes", u"게이트 판정"))
    print(u"-" * 116)

    rows, score = [], {u"tp": 0, u"fp": 0, u"tn": 0, u"fn": 0, u"unk": 0}
    for i, sp in enumerate(D):
        ex = sp.get("exe") or {}
        a = ex.get("addr")
        if not a:
            print(u"%-4d %-42s %-11s (RVA 없음)" % (i, sp["name"][:42], u"-"))
            continue
        rva = int(a, 16)
        cand = by_name.get(sp["name"]) or []
        hit = next((r for r in cand if r.get("rva") == rva), None)
        if hit is None:
            j = None
            tag = u"🟡판정 불가 — dllmatch 에 이 (이름,주소) 쌍이 없다"
        else:
            j = hit.get("jaccard")
            if j >= 0.9:
                tag = u"✅통과"
            elif j >= 0.5:
                tag = u"⛔**미확정** — 주소로 쓰지 말 것"
            else:
                tag = u"⛔⛔거의 확실히 틀렸다"
        # 정답 대조(검사기 자신을 채점한다)
        t = TRUTH.get(i, u"?")
        if t == u"?":
            truth = None
        else:
            truth = (t is None) or (t == rva)     # True = 명세 주소가 맞다
        if j is None:
            score[u"unk"] += 1
        elif truth is True:
            score[u"tp" if j >= 0.9 else u"fn"] += 1
        elif truth is False:
            score[u"tn" if j < 0.9 else u"fp"] += 1
        mark = u""
        if truth is True:
            mark = u"  ←정답(DIFF=0)" if i in PROVEN else u"  ←정답"
        elif truth is False:
            mark = u"  ←**틀린 주소**"
        print(u"%-4d %-42s 0x%-9x %-7s %-9s %-9s %s%s"
              % (i, sp["name"][:42], rva,
                 u"-" if j is None else (u"%.2f" % j),
                 u"-" if not hit else (u"%.2f" % hit.get("contain", 0)),
                 u"-" if not hit else hit.get("bytes"), tag, mark))
        rows.append((i, j, truth))

    print(u"-" * 116)
    print(u"\n★게이트 자신의 성적(ghidra 확정 정답으로 채점 · 임계 0.9)")
    print(u"    정답을 통과시킴(TP) %d · 정답을 막음(FN) %d" % (score[u"tp"], score[u"fn"]))
    print(u"    오답을 막음(TN)     %d · 오답을 통과(FP) %d" % (score[u"tn"], score[u"fp"]))
    print(u"    판정 불가(unknown)  %d" % score[u"unk"])
    if score[u"fp"] == 0 and score[u"fn"] == 0:
        print(u"    ⟹ ★**오류 0건** — 이 게이트만으로 20건이 전부 올바르게 갈렸다.")

    # 정정된 주소들이 dllmatch/percolate 에 애초에 있었는가
    print(u"\n★정정된 진짜 주소는 매니페스트에 있었는가(있었다면 **놓친 것**이다)")
    for i in sorted(k for k, v in TRUTH.items() if v):
        nm = D[i]["name"]
        t = TRUTH[i]
        dhit = [r for r in (by_name.get(nm) or []) if r.get("rva") == t]
        pany = perc_find(nm)
        phit = [r for r in pany if r.get("rva") == t]
        print(u"    #%02d %-42s 진짜 0x%-9x  dllmatch %s · percolate %s"
              % (i, nm[:42], t,
                 (u"**있음** jac %.2f" % dhit[0]["jaccard"]) if dhit else u"없음",
                 (u"**있음** jac %.2f" % phit[0].get("jaccard", 0)) if phit
                 else (u"다른 주소 0x%x" % pany[0]["rva"] if pany else u"없음")))

    print(u"\n⚠적용 범위: 이 게이트는 **명세 주소를 기각**하는 데 강하고, **정답을 찾아주지는 않는다**.")
    print(u"   기각 뒤에는 호출자/피호출자 집합 + 사이트 다중도 대조(ghidra-re 권고 ③)로 가야 한다.")


if __name__ == "__main__":
    main()
