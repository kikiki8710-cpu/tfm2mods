# -*- coding: utf-8 -*-
u"""mkspec3_md — `specs20_v3.json` → REPORT 용 마크다운. (2026-09-11)

JSON 이 정본이고 이 문서는 **투영**이다. 손으로 고치지 말고 이 스크립트를 다시 돌려라.
"""
import io, json, os, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "_spec", "specs20_v3.json")
# ★★**base 가 정본이다.** (2026-09-11 사고)
#   초판은 worktree 경로만 박아 놨고, 그래서 이 문서(623KB)가 **base 에서 안 보였다.**
#   `MEM\` 은 worktree 밖 공유 경로라 그 포인터들이 허공을 가리키고 있었다.
#   ⟹ **base 를 먼저 쓰고, worktree 사본이 있으면 거기도 같이 쓴다.**
BASE = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_ai_adjust"
WT = os.path.join(r"C:\Users\jungs\Desktop\claude\tfm2\.claude\worktrees",
                  r"silerus-mode-continue-aed092\mods_report\tfm2_ai_adjust")
NAME = u"06_판단함수_명세_v3.md"
DSTS = [os.path.join(p, NAME) for p in (BASE, WT) if os.path.isdir(p)]
DST = DSTS[0] if DSTS else os.path.join(BASE, NAME)

D = json.load(io.open(SRC, encoding="utf-8"))
M, S = D["meta"], D["specs"]
o = []
w = o.append

EVN = {1: u"런타임실측", 2: u"오라클", 3: u"tcx/MIR", 4: u"IR", 5: u"추론"}


def ev(x):
    return EVN.get(x, u"?")


w(u"# game_ai 판단함수 20개 명세 (v3)")
w(u"")
w(u"> 게임 **%s** · %s · 스키마 **%s**" % (M.get("game"), M.get("date"), M.get("schema")))
w(u"> 정본 = `C:\\tfm2mods\\MIG\\_spec\\specs20_v3.json` — **이 문서는 그 투영이다**"
  u"(`mkspec3_md.py` 로 재생성. 손으로 고치지 말 것).")
w(u"")
w(u"## 0. 이 문서를 읽는 규칙 (먼저 읽어라)")
w(u"")
w(u"**정본 우선순위** — %s" % M.get("precedence"))
w(u"")
w(u"**증거 등급 `ev`**")
w(u"")
w(u"| ev | 뜻 |")
w(u"|---|---|")
for k in sorted(M["ev_tiers"]):
    w(u"| %s | %s |" % (k, M["ev_tiers"][k]))
w(u"")
w(u"> %s" % M.get("ev_rule"))
w(u"")
w(u"**자동 생성 필드** — %s" % M.get("autofill"))
w(u"")
w(u"⚠**`open` 만 남은 일이다.** `closed` 는 이미 닫힌 것(근거와 함께 보존). "
  u"판정 어휘 = `미탐색` / `재료 부재`(범위 열거 필수) / `표기 불가`. "
  u"「불가」를 쓰기 전에 `MIG\\METHOD_MAP.md §0` 을 볼 것.")
w(u"")
c = M["counts"]
w(u"## 1. 규모")
w(u"")
w(u"| 항목 | 수 |")
w(u"|---|---|")
for k, lab in (("functions", u"함수"), ("mem", u"메모리 접근(reads+writes)"), ("consts", u"상수"),
               ("knobs", u"조정점(노브)"), ("callees", u"피호출자(시그니처 해결)"),
               ("callees_unresolved", u"피호출자(미매칭 — 확인 필요)"),
               ("caller_sites", u"호출부(자동 열거)"), ("open", u"★남은 미확정"), ("closed", u"닫힌 항목")):
    w(u"| %s | %s |" % (lab, c.get(k)))
w(u"")
w(u"## 2. 함수 목록")
w(u"")
w(u"| # | 함수 | 계층 | open | 노브 | 호출부 |")
w(u"|---|---|---|---|---|---|")
for s in S:
    w(u"| %02d | `%s` | %s | **%d** | %d | %d |"
      % (s["i"], s["name"], s.get("layer") or u"", len(s["open"]), len(s["knobs"]),
         s["callers"]["count"]))
w(u"")
w(u"---")
w(u"")

for s in S:
    w(u"# %02d. `%s`" % (s["i"], s["name"]))
    w(u"")
    w(u"> %s" % (s.get("one_line") or u""))
    w(u"")
    w(u"- **소스** `%s:%s` · **계층** %s" % (s.get("src"), s.get("src_line"), s.get("layer")))
    ir = s.get("ir") or {}
    w(u"- **IR** `%s:%s~%s`" % (ir.get("file"), ir.get("frm"), ir.get("to")))
    sg = s["sig"]
    w(u"- **가시성** `%s` · **경로** `%s` · MIR %s" % (sg.get("vis"), sg.get("path"), sg.get("mir")))
    w(u"")
    if sg.get("tcx"):
        w(u"**시그니처(tcx 정본, ev%d)**" % sg.get("ev", 3))
        w(u"")
        w(u"```rust")
        w(sg["tcx"])
        w(u"```")
        w(u"")
    if sg.get("params"):
        w(u"| # | 인자 | 타입 | 역할 | ev |")
        w(u"|---|---|---|---|---|")
        for p in sg["params"]:
            w(u"| %s | `%s` | `%s` | %s | %s |"
              % (p.get("i"), p.get("name"), p.get("type"),
                 (p.get("role") or u"").replace("|", "\\|"), ev(p.get("ev"))))
        w(u"")
    if sg.get("ret"):
        w(u"**반환** %s" % sg["ret"])
        w(u"")

    w(u"### 판정 로직 (의사코드)")
    w(u"")
    w(u"> ⚠%s" % s["logic_note"])
    w(u"")
    w(u"```rust")
    w(s.get("logic") or u"")
    w(u"```")
    w(u"")

    if s["mem"]:
        w(u"### 메모리 접근 — ★오프셋 정본")
        w(u"")
        w(u"| r/w | base | offset | 이름 | 값 | 비고 | 사전 | ev |")
        w(u"|---|---|---|---|---|---|---|---|")
        for m in s["mem"]:
            w(u"| %s | `%s` | `%s` | `%s` | %s | %s | %s | %s |"
              % (m.get("dir"), m.get("base"), m.get("offset"), m.get("name"),
                 str(m.get("value") or u"")[:40].replace("|", "\\|"),
                 (m.get("note") or u"").replace("|", "\\|").replace("\n", " "),
                 m.get("chk"), ev(m.get("ev"))))
        w(u"")

    if s["consts"]:
        w(u"### 상수 — ★상수 정본")
        w(u"")
        w(u"| 값 | 줄 | 종류 | 뜻 | ev |")
        w(u"|---|---|---|---|---|")
        for x in s["consts"]:
            w(u"| `%s` | %s | %s | %s | %s |"
              % (x.get("value"), x.get("src_line"), x.get("kind"),
                 (x.get("meaning") or u"").replace("|", "\\|").replace("\n", " "), ev(x.get("ev"))))
        w(u"")

    if s["knobs"]:
        w(u"### 조정점 (노브) — %d개" % len(s["knobs"]))
        w(u"")
        for x in s["knobs"]:
            w(u"**%s** (%s, ev%s)" % (x.get("what"), x.get("src"), x.get("ev")))
            w(u"")
            w(u"- 어디: %s" % (x.get("where") or u""))
            w(u"- 값: `%s`" % str(x.get("value")))
            w(u"- 영향: %s" % (x.get("effect") or u""))
            if x.get("note"):
                w(u"- 비고: %s" % x["note"])
            w(u"")

    w(u"### 피호출자 — ★시그니처 정본 (자동 생성)")
    w(u"")
    w(u"> %s" % s["callees_note"])
    w(u"")
    if s["callees"]:
        w(u"| 이름 | 경로 | 가시성 | 시그니처 | 위치 | mir |")
        w(u"|---|---|---|---|---|---|")
        for x in s["callees"]:
            w(u"| `%s` | `%s` | `%s` | `%s` | %s | %s |"
              % (x.get("name"), x.get("path"), x.get("vis"),
                 (x.get("sig") or u"").replace("|", "\\|"), x.get("at"), x.get("mir")))
        w(u"")
    um = s.get("callees_unmatched") or {}
    if um.get("names"):
        w(u"**미매칭 %d개** — %s" % (len(um["names"]), um.get("note")))
        w(u"")
        w(u"`" + u"` · `".join(um["names"]) + u"`")
        w(u"")

    cl = s["callers"]
    w(u"### 호출부 — %d곳 (자동 열거)" % cl["count"])
    w(u"")
    w(u"> %s" % cl["note"])
    w(u"")
    w(u"범위: %s" % cl["scope"])
    if cl["sites"]:
        w(u"")
        w(u"`" + u"` · `".join(cl["sites"]) + u"`")
    w(u"")

    sb = s["siblings"]
    if sb.get("plan"):
        w(u"### 형제 진입점 — `%s` (%d개, 자동 열거)" % (sb["plan"], sb["count"]))
        w(u"")
        w(u"> %s" % sb["note"])
        w(u"")
        w(u"| 경로 | 가시성 | 위치 | mir |")
        w(u"|---|---|---|---|")
        for e in sb["entries"]:
            w(u"| `%s` | `%s` | %s | %s |" % (e["path"], e["vis"], e["at"], e["mir"]))
        w(u"")

    w(u"### ★남은 미확정 (open) — %d건" % len(s["open"]))
    w(u"")
    if s["open"]:
        for x in s["open"]:
            w(u"- **[%s / ev%s]** %s" % (x["class"], x["ev"], x["q"]))
    else:
        w(u"*(없음)*")
    w(u"")
    if s["closed"]:
        w(u"<details><summary>닫힌 항목 %d건 (참조용)</summary>" % len(s["closed"]))
        w(u"")
        for x in s["closed"]:
            w(u"- ~~%s~~" % x["q"][:220].replace("\n", " "))
            w(u"  - 닫은 근거: %s" % x["why"])
        w(u"")
        w(u"</details>")
        w(u"")
    if s.get("history"):
        w(u"<details><summary>정정 이력 %d건 (참조용 · 본문 아님)</summary>" % len(s["history"]))
        w(u"")
        for h in s["history"]:
            w(u"- **%s**" % str(h.get("was"))[:160].replace("\n", " "))
            w(u"  - → %s" % str(h.get("now")).replace("\n", "\n    "))
        w(u"")
        w(u"</details>")
        w(u"")
    w(u"---")
    w(u"")

txt = u"\n".join(o)
for _d in DSTS or [DST]:
    with io.open(_d, "w", encoding="utf-8", newline="\r\n") as f:
        f.write(txt)
    print(u"%s  (%d KB · %d줄)" % (_d, len(txt.encode("utf-8")) / 1024, txt.count("\n") + 1))
