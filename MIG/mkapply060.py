# -*- coding: utf-8 -*-
"""
mkapply060.py — 0.6.0 반영(logic_060 채우기) 배치 브리핑 생성.
변경 73(specs20_v060 v060.verdict ≠ 동치/소멸) 을 RE 출처별로 묶어 `_next/apply060/batch_NN.md` 를 만든다.
각 함수 블록 = 0.5.8 명세(one_line·sig·logic 전문·consts·callees) + 0.6.0 판정·패치 요지(§B/§D 행) + RE 파일·절 + callee_changed + dispcheck 변위 짝.
에이전트는 블록마다 `_next/apply060/out/<old>.md` 를 아래 형식으로 쓴다:
  # <old>→<new> <name>
  ## logic_060            (0.5.8 logic 과 같은 의사 Rust 형식 · v3 경로만 · 바뀐 줄에 `// ★0.6.0` 주석 · 오프셋/태그/슬롯은 0.6.0 값)
  ## changes              (0.5.8 대비 변경점 목록 — 줄 단위)
  ## verified             (Ghidra 0.6.0 디컴으로 확인한 지점 · 확인 못 한 지점)
  ## confidence           (A/B/C + 한 줄 이유)
병합 = applyspec060.py
"""
import io, json, os, re, sys, collections
sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__)); OUT = os.path.join(HERE, "_next", "apply060")
os.makedirs(os.path.join(OUT, "out"), exist_ok=True)
V = json.load(io.open(os.path.join(HERE, "_spec", "specs20_v060.json"), encoding="utf-8"))
DC = {f["old"]: f for f in json.load(io.open(os.path.join(HERE, "_next", "dispcheck060.json"), encoding="utf-8"))}
RE_DIR = r"C:\Users\jungs\Desktop\claude\tfm2\.claude\worktrees\swap-order-button-style-fe9e04\mods_report\tfm2_judge_verify\RE"
W = {}
for line in io.open(os.path.join(HERE, "_next", "spec_patch_060.md"), encoding="utf-8"):
    m = re.match(r"\|\s*`([0-9a-f]+)`\s*\|\s*`([0-9a-f]+)`\s*\|\s*([^|]*)\|\s*(.*)\|\s*(w\d+ §[^|]*)\|\s*$", line.rstrip())
    if m: W[m.group(1)] = m.group(5).strip()
def re_file(ref):
    m = re.search(r"(2026-09-1[67]_[^`\s]+?원문\.md)", ref or "")
    if m: return m.group(1)
    m = re.match(r"w(\d+) §", ref or "")
    if m:
        for fn in os.listdir(RE_DIR):
            if fn.startswith("2026-09-17_r21_심층_w%s_" % m.group(1)): return fn
    return None
ch = [s for s in V["specs"] if s["v060"]["addr_058"] and not s["v060"]["verdict"].startswith((u"동치", u"✅동치", u"소멸"))]
groups = collections.defaultdict(list)
for s in ch:
    b = s["v060"]; ref = b.get("patch_ref") or ""
    f = re_file(ref) or re_file(W.get(b["addr_058"], "")) or "(없음)"
    groups[f].append(s)
# 배치: RE 파일별로 묶되 9 개 상한 · 작은 그룹은 합침
batches = []; cur = []
for f, lst in sorted(groups.items(), key=lambda kv: -len(kv[1])):
    for s in lst:
        cur.append((f, s))
        if len(cur) >= 9: batches.append(cur); cur = []
if cur: batches.append(cur)
def block(f, s):
    b = s["v060"]; o = b["addr_058"]
    L = [u"### `%s` → `%s` %s (i=%s · %s)" % (o, b["addr"], s["name"], s.get("i"), b["verdict"]),
         u"- 0.5.8 src: `%s:%s` · one_line: %s" % (s.get("src"), s.get("src_line"), s.get("one_line")),
         u"- 0.6.0 판정: **%s** · 패치 요지: %s" % (b.get("patch_kind"), b.get("patch_note")),
         u"- RE 정본: `%s` · 절: %s" % (f, b.get("patch_ref"))]
    if b.get("callee_changed"): L.append(u"- 콜리 변경(같이 갈아야 함): " + u" · ".join(u"`%s→%s` %s(%s)" % (c["old"], c["new"], c["name"][:30], c["verdict"]) for c in b["callee_changed"]))
    d = DC.get(o)
    if d and d.get("rows"):
        pairs = sorted({(r[2], r[3], r[4]) for r in d["rows"] if r[2] != r[3]})
        if pairs: L.append(u"- dispcheck 변위 짝(구→신 · 규칙): " + u" · ".join(u"%s→%s(%s)" % (a, c, (k or "?")[:18]) for a, c, k in pairs[:24]))
    sg = s.get("sig") or {}; sg = sg.get("tcx") if isinstance(sg, dict) and sg.get("tcx") else json.dumps(sg, ensure_ascii=False)
    L.append(u"- sig: `%s`" % str(sg)[:300])
    if s.get("consts"): L.append(u"- consts: %s" % json.dumps(s["consts"], ensure_ascii=False)[:600])
    L.append(u"- 0.5.8 logic 전문:\n```\n%s\n```" % (s.get("logic") or ""))
    if s.get("logic_note"): L.append(u"- logic_note: %s" % s["logic_note"])
    return u"\n".join(L)
COMMON = io.open(os.path.join(HERE, "_next", "r21_prompt_common.md"), encoding="utf-8").read() if os.path.exists(os.path.join(HERE, "_next", "r21_prompt_common.md")) else u""
idx = [u"# apply060 배치 색인(변경 73 · %d 배치)" % len(batches), u""]
for n, bt in enumerate(batches, 1):
    fn = u"batch_%02d.md" % n
    L = [u"# apply060 %s — 반영(logic_060) 브리핑 · 함수 %d" % (fn, len(bt)), u"",
         u"★**version = 3 런타임 확정(09-17)** — v<3/v2 경로는 명세에서 제외(사장). 태그/오프셋/vt 슬롯 = `spec_patch_060.md` §A(dispcheck 추가 행 포함). 출력 형식·절차 = `mkapply060.py` 도크스트링. 출력 = `_next/apply060/out/<old>.md`.", u"",
         u"RE 정본 파일: " + u" · ".join(sorted({f for f, _ in bt})), u""]
    for f, s in bt: L.append(block(f, s)); L.append(u"")
    io.open(os.path.join(OUT, fn), "w", encoding="utf-8").write(u"\n".join(L))
    idx.append(u"- %s: %s" % (fn, u" · ".join(u"`%s` %s" % (s["v060"]["addr_058"], s["name"][:28]) for _, s in bt)))
io.open(os.path.join(OUT, "INDEX.md"), "w", encoding="utf-8").write(u"\n".join(idx))
print(u"배치 %d · 함수 %d · RE 파일 %d" % (len(batches), len(ch), len(groups)))
for f, lst in sorted(groups.items(), key=lambda kv: -len(kv[1])): print(u"  %2d %s" % (len(lst), f))
