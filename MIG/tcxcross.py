# -*- coding: utf-8 -*-
u"""tcx 권위 덤프 vs 기존 DWARF 역산 사전(distruct.json / dienum.json) 기계 대조.
사용: python -X utf8 tcxcross.py [struct|enum|collide] > 결과.txt
"""
import json, io, os, sys, re, functools
from collections import defaultdict
TCX = r"C:\tfm2mods\MIG\_tcx"
HERE = r"C:\tfm2mods\MIG"
CRATES = ("game_ai", "game_core", "game_view")

@functools.lru_cache(maxsize=8)
def load(cr):
    with io.open(os.path.join(TCX, cr + ".json"), encoding="utf-8") as f:
        return json.load(f)

def tcx_structs():
    """leaf 이름 -> [(crate, 표시경로, size, [(fname,off)])]
    ★단일 variant 구조체뿐 아니라 **열거형의 각 variant 페이로드**도 후보로 넣는다.
      DWARF 역산 사전(distruct)은 variant 도 같은 leaf 이름의 '구조체'로 담기 때문."""
    out = defaultdict(list)
    for cr in CRATES:
        for it in load(cr)["items"]:
            if "adt" not in it: continue
            lay = it.get("layout")
            if not lay: continue
            vs = it["adt"]["variants"]
            if len(vs) == 1:
                off = lay.get("offsets")
                if off is None: continue
                fl = [(f["n"], off[i]) for i, f in enumerate(vs[0]["fields"]) if i < len(off)]
                out[it["p"].rsplit("::", 1)[-1]].append((cr, it["p"], lay["size"], fl))
            else:
                vl = lay.get("vlayouts") or []
                for vi, v in enumerate(vs):
                    off = vl[vi]["offsets"] if vi < len(vl) else lay.get("offsets")
                    if off is None: continue
                    fl = [(f["n"], off[i]) for i, f in enumerate(v["fields"]) if i < len(off)]
                    out[v["n"]].append((cr, it["p"] + "::" + v["n"] + "(variant)", lay["size"], fl))
    return out

def tcx_enums():
    """다중 variant ADT: leaf -> [(crate, fullpath, {discr:name})]"""
    out = defaultdict(list)
    for cr in CRATES:
        for it in load(cr)["items"]:
            if "adt" not in it: continue
            vs = it["adt"]["variants"]
            if len(vs) < 2: continue
            m = {}
            for v in vs:
                d = v.get("discr") or ""
                mm = re.search(r"val:\s*(-?\d+)", d)
                if mm: m[int(mm.group(1))] = v["n"]
            if not m: continue
            leaf = it["p"].rsplit("::", 1)[-1]
            out[leaf].append((cr, it["p"], m, it.get("layout") or {}))
    return out

def leaf_of_dwarf(k):
    k = k.strip()
    k = re.sub(r"^enum2\$<(.*)>$", r"\1", k).strip()
    if "<" in k:                       # 제네릭 인스턴스는 대조 대상에서 제외
        return None
    return k.rsplit("::", 1)[-1]

def do_struct():
    ds = json.load(io.open(os.path.join(HERE, "distruct.json"), encoding="utf-8"))
    tx = tcx_structs()
    n_cmp = n_size = n_off = n_nomatch = n_amb = 0
    print("## A. distruct.json(구조체) vs tcx layout_of")
    for name, ent in sorted(ds.items()):
        cands = tx.get(name)
        if not cands:
            n_nomatch += 1
            continue
        # 튜플필드 표기 차이(__0 vs 0)는 잡음이므로 정규화
        dfields = dict(((f["name"][2:] if f["name"].startswith("__") and f["name"][2:].isdigit() else f["name"]), f["off"]) for f in ent.get("fields", []))
        dsize = ent.get("size")
        # 후보 중 '가장 잘 맞는' 것을 고른다(동명 타입 다수 대비)
        best = None; best_score = -1
        for (cr, p, sz, fl) in cands:
            tf = dict(fl)
            common = set(tf) & set(dfields)
            score = sum(1 for k in common if tf[k] == dfields[k]) + (5 if sz == dsize else 0)
            if score > best_score: best_score, best = score, (cr, p, sz, tf)
        cr, p, sz, tf = best
        if len(cands) > 1: n_amb += 1
        n_cmp += 1
        probs = []
        if dsize is not None and sz != dsize:
            probs.append("SIZE distruct=%s tcx=%s" % (dsize, sz))
            n_size += 1
        bad = [(k, dfields[k], tf[k]) for k in sorted(set(tf) & set(dfields)) if tf[k] != dfields[k]]
        if bad:
            n_off += 1
            probs.append("OFF " + ", ".join("%s distruct=0x%x tcx=0x%x" % b for b in bad[:6]))
        only_d = sorted(set(dfields) - set(tf))
        only_t = sorted(set(tf) - set(dfields))
        if probs or (only_d and only_t):
            print("\n- %s  -> tcx %s (%s)%s" % (name, p, cr, "  [동명후보 %d개]" % len(cands) if len(cands) > 1 else ""))
            for x in probs: print("    " + x)
            if only_d: print("    distruct에만: %s" % only_d[:8])
            if only_t: print("    tcx에만:     %s" % only_t[:8])
    print("\n### 요약(struct): 대조 %d / 크기불일치 %d / 오프셋불일치 %d / tcx에 동명 없음 %d / 동명중복 %d (distruct 총 %d)"
          % (n_cmp, n_size, n_off, n_nomatch, n_amb, len(ds)))

def do_enum():
    de = json.load(io.open(os.path.join(HERE, "dienum.json"), encoding="utf-8"))
    tx = tcx_enums()
    n_cmp = n_bad = n_nomatch = 0
    print("## B. dienum.json(열거형 판별자) vs tcx adt_def discriminant")
    for k, ent in sorted(de.items()):
        leaf = leaf_of_dwarf(k)
        if not leaf: continue
        cands = tx.get(leaf)
        if not cands:
            n_nomatch += 1
            continue
        dv = dict((int(a), b) for a, b in ent.get("variants", {}).items())
        best = None; best_score = -1
        for (cr, p, m, lay) in cands:
            score = sum(1 for t in set(m) & set(dv) if m[t] == dv[t])
            score += 3 * len(set(m.values()) & set(dv.values()))
            if score > best_score: best_score, best = score, (cr, p, m, lay)
        cr, p, m, lay = best
        n_cmp += 1
        bad = [(t, dv[t], m[t]) for t in sorted(set(m) & set(dv)) if m[t] != dv[t]]
        onlyd = sorted(set(dv) - set(m)); onlyt = sorted(set(m) - set(dv))
        if bad or (onlyd and onlyt):
            n_bad += 1
            print("\n- %s  -> tcx %s (%s)" % (k.strip(), p, cr))
            for t, a, b in bad[:8]: print("    태그 %d: dienum=%s  tcx=%s" % (t, a, b))
            if onlyd: print("    dienum에만 태그: %s" % [(t, dv[t]) for t in onlyd[:6]])
            if onlyt: print("    tcx에만 태그:   %s" % [(t, m[t]) for t in onlyt[:6]])
            if lay.get("tag_enc"): print("    tcx tag_enc=%s" % lay["tag_enc"][:120])
    print("\n### 요약(enum): 대조 %d / 불일치 %d / tcx에 동명 없음 %d (dienum 총 %d)" % (n_cmp, n_bad, n_nomatch, len(de)))

def do_collide():
    """distruct 키(=leaf name)가 tcx 에서 여러 타입에 해당하는 경우 = 오귀속 위험."""
    ds = json.load(io.open(os.path.join(HERE, "distruct.json"), encoding="utf-8"))
    tx = tcx_structs()
    print("## C. 이름 충돌(같은 leaf 이름의 타입이 게임 크레이트 안에서 2개 이상)")
    n = 0
    for name in sorted(ds):
        cands = tx.get(name) or []
        if len(cands) > 1:
            n += 1
            sizes = sorted(set(c[2] for c in cands))
            dsz = ds[name].get("size")
            hit = [c[1] for c in cands if c[2] == dsz]
            print("- %-34s distruct.size=%-6s  tcx 동명 %d개 sizes=%s  크기일치=%s" % (
                name, dsz, len(cands), sizes[:6], hit[:3] if hit else "없음(!)"))
    print("\n### 충돌 %d건" % n)

if __name__ == "__main__":
    what = sys.argv[1] if len(sys.argv) > 1 else "struct"
    {"struct": do_struct, "enum": do_enum, "collide": do_collide}[what]()
