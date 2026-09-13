# -*- coding: utf-8 -*-
u"""mergespec.py — 한 함수를 여러 배치가 분책으로 쓴 부분 명세(`<id>.partA.json` …)를 **하나의 명세 JSON** 으로 합친다. (2026-09-14 신설 · r12 update)

사용: python -X utf8 MIG\\mergespec.py _spec\\r12\\handler__LegacyPlanHandler_update  [--out <파일>]
      (접두를 주면 `<접두>.part*.json` 을 A,B,C… 순으로 읽어 `<접두>.json` 으로 쓴다)

규칙:
  · 스칼라 머리(name·sym·src·src_line·ir_file·ir_from·ir_to·layer) = 첫 조각(A) 것. 조각 간 불일치는 경고.
  · signature = A 의 것(A 가 함수 머리 담당). 다른 조각에 있으면 params 의 note 만 「(배치 X) …」 로 덧붙인다.
  · one_line = A 의 것.
  · logic = 조각 순서대로 이어 붙인다(각 조각 첫 줄 `// handler.rs:a~b (배치 X)` 헤더 유지).
  · reads/writes/constants/calls/knobs/unknown/aux = 이어 붙이되 **완전 중복**(직렬화 동일)은 제거. reads/writes 는 (base,offset,name) 같으면 note 를 합친다.
"""
import io, json, os, sys, glob, re


def _key_mem(m):
    return (str(m.get("base")), str(m.get("offset")), (m.get("name") or "").strip())


def merge(parts):
    out = dict(parts[0][1])
    tag0 = parts[0][0]
    for k in ("reads", "writes", "constants", "calls", "knobs", "unknown", "aux", "new_knobs", "resolved", "still_unknown"):
        out[k] = []
    logic = []
    seen = {}
    for tag, sp in parts:
        for k in ("name", "sym", "src", "src_line", "ir_file", "ir_from", "ir_to"):
            if k in sp and k in out and str(sp[k]) != str(out[k]):
                print(u"⚠ %s: %s 불일치 %r vs %r(A)" % (tag, k, sp[k], out[k]))
        lg = sp.get("logic") or u""
        if lg and not lg.lstrip().startswith(u"//"):
            lg = u"// (배치 %s)\n" % tag + lg
        if lg:
            logic.append(lg.rstrip())
        for k in ("reads", "writes"):
            for m in sp.get(k) or []:
                if not isinstance(m, dict):
                    out[k].append(m); continue
                key = (k,) + _key_mem(m)
                if key in seen:
                    old = seen[key]
                    n1, n2 = (old.get("note") or u""), (m.get("note") or u"")
                    if n2 and n2 not in n1:
                        old["note"] = (n1 + u" | (배치 %s) " % tag + n2) if n1 else n2
                    continue
                m = dict(m); seen[key] = m; out[k].append(m)
        for k in ("constants", "calls", "knobs", "unknown", "aux", "new_knobs", "resolved", "still_unknown"):
            for x in sp.get(k) or []:
                ser = json.dumps(x, ensure_ascii=False, sort_keys=True)
                if (k, ser) in seen:
                    continue
                seen[(k, ser)] = 1
                if k == "unknown" and isinstance(x, (str, bytes)):
                    x = u"(배치 %s) " % tag + x
                out[k].append(x)
        if tag != tag0 and sp.get("signature") and isinstance(sp["signature"], dict):
            ps = {p.get("i"): p for p in (out.get("signature") or {}).get("params") or [] if isinstance(p, dict)}
            for p in sp["signature"].get("params") or []:
                if isinstance(p, dict) and p.get("i") in ps and p.get("note"):
                    q = ps[p["i"]]
                    if p["note"] not in (q.get("note") or u""):
                        q["note"] = ((q.get("note") or u"") + u" | (배치 %s) " % tag + p["note"]).strip(u" |")
    out["logic"] = u"\n\n".join(logic)
    out["_merged_from"] = [t for t, _ in parts]
    return out


def main():
    a = sys.argv[1:]
    if not a:
        print(__doc__); return
    pre = a[0]
    outp = a[a.index("--out") + 1] if "--out" in a else pre + ".json"
    files = sorted(glob.glob(pre + ".part*.json"))
    if not files:
        print(u"조각 없음:", pre + ".part*.json"); return
    parts = []
    for f in files:
        tag = re.search(r"\.part([A-Za-z0-9]+)\.json$", f).group(1)
        parts.append((tag, json.load(io.open(f, encoding="utf-8"))))
        print(u"  조각 %s: %s (%d B)" % (tag, os.path.basename(f), os.path.getsize(f)))
    out = merge(parts)
    io.open(outp, "w", encoding="utf-8", newline="").write(json.dumps(out, ensure_ascii=False, indent=1))
    print(u"→ %s  logic %dB · reads %d · writes %d · consts %d · calls %d · knobs %d · unknown %d · aux %d" % (
        outp, len(out["logic"]), len(out["reads"]), len(out["writes"]), len(out["constants"]), len(out["calls"]), len(out["knobs"]), len(out["unknown"]), len(out["aux"])))


if __name__ == "__main__":
    main()
