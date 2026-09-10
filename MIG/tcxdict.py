# -*- coding: utf-8 -*-
u"""tcxdict — rustc `TyCtxt` 권위 덤프로 만든 **구조체/열거형 정본 사전**.

기존 `distruct.py`(DWARF 역산) 의 근본 결함 두 개를 고친 대체품이다:
  1) 키가 leaf 이름 하나뿐이라 **동명 타입끼리 조용히 덮어썼다** → 여기선 키가 full def_path,
     leaf 는 **별칭 인덱스**일 뿐이고 모호하면 **"모호"로 후보 전량을 반환**한다.
  2) `Vec`/`String`/배열/`Option` 내부가 통짜였다 → 여기선 `tcxtg` 타입그래프가
     **표준 라이브러리 제네릭 인스턴스까지 컴파일러가 계산한 실제 오프셋**으로 들어 있다.

재료:
  _tcx\\{game_ai,game_core,game_view}.json     (tcxdump.rs — DefId 별 span/가시성/ADT)
  _tcx\\tg_{...}.jsonl                          (tcxtg.rs  — 구체 타입 레이아웃 그래프)
산출: _tcx\\dict.json

사용:
  python -X utf8 tcxdict.py --build                  # 사전 생성
  python -X utf8 tcxdict.py Entity                   # 필드 전개(최상위, 오프셋순)
  python -X utf8 tcxdict.py Entity --deep            # 절대 오프셋 전개(중첩 관통)
  python -X utf8 tcxdict.py Entity 0x628             # 그 오프셋이 무슨 필드인지(중첩 관통)
  python -X utf8 tcxdict.py --enum SubPlan           # variant + 논리인덱스 + 실제 메모리태그
  python -X utf8 tcxdict.py --enum SubPlan 5         # 메모리태그 5 가 무엇인지 + 페이로드
  python -X utf8 tcxdict.py --ambig                  # 동명 다중(모호) 전량
  python -X utf8 tcxdict.py --stats                  # 사전 통계 + 기존 사전 커버리지

★전개 규칙(이 규칙은 문서에도 그대로 있다):
  - 재귀 진입: 단일 variant ADT(struct/union, 표준 라이브러리 포함) · 튜플 · 배열 · 열거형
  - 열거형은 variant 마다 `@Variant` 세그먼트를 붙여 전개하고, 태그 자체는 `@tag` 로 표시
  - 배열: 원소 수 <= ARR_MAX(기본 8)이면 `[i]` 로 전부 전개, 더 크면
      * 목록 모드에서는 `[0..N] stride=S` 한 줄로 요약
      * 오프셋 조회에서는 `i = (want-base)//stride` 로 **해당 원소만** 계산해 관통
  - 참조/생포인터/`dyn`/클로저/함수포인터에서 **멈춘다**(다른 객체 경계)
  - 최대 깊이 MAX_DEPTH(기본 6), 같은 타입이 경로에 다시 나오면 중단(순환 가드)
"""
import json, io, os, sys, re, functools
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
TCX = os.path.join(HERE, "_tcx")
DICT = os.path.join(TCX, "dict.json")
CRATES = ("game_ai", "game_core", "game_view")            # 게임 로직 크레이트
ENGINE = ("engine_core", "engine_ui", "common")           # 엔진/공용(있으면 같이 싣는다)
ALLCR = CRATES + ENGINE
GAME_PREFIX = tuple(c + "::" for c in CRATES)
ENG_PREFIX = tuple(c + "::" for c in ENGINE)

ARR_MAX = 8
MAX_DEPTH = 6

try:
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
except Exception:
    pass


# ─────────────────────────────────────────────────────────── build ──
def parse_discr(s):
    if not s:
        return None
    m = re.search(r"val:\s*(-?\d+)", s)
    return int(m.group(1)) if m else None


def memory_tag(tag, idx, discr):
    u"""variant 논리 인덱스 -> **실제 메모리 태그**.
    근거 = rustc_const_eval/src/interpret/discriminant.rs `tag_for_variant`
      Direct: tag = truncate(declared_discr, tag_size)
      Niche : untagged_variant 는 태그 없음(암묵). 그 외 tag = niche_start + (idx - niche_variants.start)
              (tag 크기의 머신 산술 = wrapping)"""
    if not tag:
        return None
    sz = tag.get("sz")
    mod = (1 << (8 * sz)) if sz else None
    if tag.get("enc") == "Direct":
        if discr is None:
            return None
        return (discr % mod) if mod else discr
    # Niche
    if idx == tag.get("untagged"):
        return None
    if not (tag.get("nv0") <= idx <= tag.get("nv1")):
        return None
    v = tag.get("nstart") + (idx - tag.get("nv0"))
    return (v % mod) if mod else v


def build(verbose=True):
    def log(*a):
        if verbose:
            print(*a, file=sys.stderr)

    # 1) tcxdump 아이템에서 def_path -> span/가시성/repr
    meta = {}
    for cr in ALLCR:
        p = os.path.join(TCX, cr + ".json")
        if not os.path.exists(p):
            log("[build] skip(없음)", p); continue
        log("[build] load", p)
        with io.open(p, encoding="utf-8") as f:
            d = json.load(f)
        for it in d["items"]:
            if "adt" not in it:
                continue
            e = meta.setdefault(it["p"], {})
            e.setdefault("cr", cr)
            e.setdefault("sp", it.get("sp"))
            e.setdefault("v", it.get("v"))
            e.setdefault("dk", it.get("k"))
            e.setdefault("repr", it["adt"].get("repr", "")[:0] or None)
            # variant 별 소스 위치(열거형 variant 정의 줄)
            vsp = {}
            for v in it["adt"]["variants"]:
                if v.get("sp"):
                    vsp[v["n"]] = v["sp"]
            if vsp:
                e.setdefault("vsp", vsp)
        del d
    log("[build] adt meta paths =", len(meta))

    # 2) 타입 그래프
    types = {}
    for cr in ALLCR:
        p = os.path.join(TCX, "tg_%s.jsonl" % cr)
        if not os.path.exists(p):
            log("[build] skip(없음)", p); continue
        log("[build] load", p)
        n = 0
        with io.open(p, encoding="utf-8") as f:
            for ln in f:
                ln = ln.strip()
                if not ln:
                    continue
                o = json.loads(ln)
                n += 1
                t = o["t"]
                cur = types.get(t)
                if cur is None:
                    types[t] = o
                    o["src"] = [cr]
                elif cr not in cur["src"]:
                    cur["src"].append(cr)
        log("      ", n, "types (누적", len(types), ")")

    # 3) 메타 병합 + 태그 계산
    for t, o in types.items():
        p = o.get("p")
        m = meta.get(p) if p else None
        if m:
            o["cr"] = m.get("cr")
            o["sp"] = m.get("sp")
            o["v"] = m.get("v")
            vsp = m.get("vsp") or {}
        else:
            vsp = {}
        o["game"] = bool(p and p.startswith(GAME_PREFIX))
        o["eng"] = bool(p and p.startswith(ENG_PREFIX))
        tag = o.get("tag")
        for v in o.get("vs", []):
            v["discr"] = parse_discr(v.pop("d", None))
            v["tag"] = memory_tag(tag, v["i"], v["discr"])
            if v["n"] in vsp:
                v["sp"] = vsp[v["n"]]

    # 4) 인덱스
    by_path = defaultdict(list)
    by_leaf = defaultdict(set)
    by_variant = defaultdict(set)      # ★variant 이름 -> 그 variant 를 가진 열거형 타입
    for t, o in types.items():
        p = o.get("p")
        if not p:
            continue
        by_path[p].append(t)
        by_leaf[p.rsplit("::", 1)[-1]].add(p)
        if o.get("k") == "enum" and o.get("game"):
            for v in o.get("vs", []):
                if v.get("f"):          # 페이로드가 있는 variant 만(=구조체로 오인될 수 있는 것)
                    by_variant[v["n"]].add(t)

    out = {
        "meta": {
            "what": "rustc TyCtxt(layout_of) 권위 구조체/열거형 사전",
            "game": "0.5.8",
            "src": ["_tcx/%s.json" % c for c in CRATES] + ["_tcx/tg_%s.jsonl" % c for c in CRATES],
            "rules": {"ARR_MAX": ARR_MAX, "MAX_DEPTH": MAX_DEPTH},
            "n_types": len(types),
            "n_paths": len(by_path),
        },
        "types": types,
        "by_path": {k: v for k, v in by_path.items()},
        "by_leaf": {k: sorted(v) for k, v in by_leaf.items()},
        "by_variant": {k: sorted(v) for k, v in by_variant.items()},
    }
    log("[build] write", DICT)
    with io.open(DICT, "w", encoding="utf-8") as f:
        json.dump(out, f, ensure_ascii=False)
    log("[build] done  types=%d paths=%d leaves=%d  (%.1f MB)"
        % (len(types), len(by_path), len(by_leaf), os.path.getsize(DICT) / 1e6))
    return out


# ──────────────────────────────────────────────────────────── load ──
@functools.lru_cache(maxsize=1)
def D():
    if not os.path.exists(DICT):
        print("사전이 없다. 먼저: python -X utf8 tcxdict.py --build", file=sys.stderr)
        sys.exit(2)
    with io.open(DICT, encoding="utf-8") as f:
        return json.load(f)


def T(tystr):
    return D()["types"].get(tystr)


def is_leafish(o):
    u"""더 이상 관통하지 않는 타입인가."""
    if o is None:
        return True
    return o.get("k") in ("ref", "ptr", "slice", None) or o.get("k") not in (
        "struct", "union", "enum", "array", "tuple")


# ───────────────────────────────────────────────────── name lookup ──
def resolve_name(name, strict=False):
    u"""이름 -> (상태, 후보목록).
    상태: 'exact-type' / 'exact-path' / 'ambig' / 'none'
    후보: [(def_path, [tystr,...])]
    strict=True 면 **부분일치 폴백을 끈다**(산문 스캔처럼 오탐이 치명적인 곳에서 쓴다 —
    끄지 않으면 `payload` 가 `CurrentDatabaseEditPayload` 로 붙어 가짜 판정을 낸다)."""
    d = D()
    if name in d["types"]:
        return "exact-type", [(d["types"][name].get("p") or name, [name])]
    bp = d["by_path"]
    if name in bp:
        return ("exact-path" if len(bp[name]) == 1 else "ambig"), [(name, bp[name])]
    leaf = name.rsplit("::", 1)[-1]
    paths = d["by_leaf"].get(leaf)
    if not paths and not strict:
        # 부분일치 폴백
        paths = sorted(set(p for p in bp if p.rsplit("::", 1)[-1].lower() == leaf.lower()))
        if not paths:
            paths = sorted(set(p for p in bp if leaf.lower() in p.rsplit("::", 1)[-1].lower()))[:40]
    if not paths:
        return "none", []
    if name != leaf:                     # 사용자가 경로 일부를 줬으면 그걸로 좁힌다
        paths = [p for p in paths if name in p] or paths
    cands = [(p, bp[p]) for p in paths]
    tys = [t for _, ts in cands for t in ts]
    return ("exact-path" if len(tys) == 1 else "ambig"), cands


def print_ambig(name, cands, hint=""):
    tot = sum(len(ts) for _, ts in cands)
    print(u"★모호: '%s' 에 해당하는 타입이 %d개다 (조용히 하나를 고르지 않는다)%s" % (name, tot, hint))
    for p, ts in sorted(cands):
        for t in ts:
            o = T(t)
            sp = (o.get("sp") or {}) if o else {}
            print(u"  %-9s %-8s %-70s size=%-7s %s:%s"
                  % (("게임" if (o and o.get("game")) else "라이브러리"),
                     (o or {}).get("k", "?"), t, (o or {}).get("sz", "?"),
                     sp.get("f", ""), sp.get("l", "")))
    print(u"\n  → full def_path 로 다시 물어라.  예: python -X utf8 tcxdict.py %s" % (sorted(cands)[0][1][0]))


def variant_alias(name):
    u"""이름이 (구조체가 아니라) 열거형 variant 로도 존재하는가 — distruct 최대 함정."""
    leaf = name.rsplit("::", 1)[-1]
    return D().get("by_variant", {}).get(leaf) or []


def warn_variant(name, chosen):
    vs = variant_alias(name)
    if not vs:
        return
    print(u"  ⚠ 같은 이름의 **열거형 variant** 도 있다 — 독립 struct 와 오프셋이 다르다(페이로드는 태그 뒤로 밀린다):")
    for t in vs:
        o = T(t)
        vv = [v for v in o["vs"] if v["n"] == name.rsplit("::", 1)[-1]][0]
        o0 = min((f["o"] for f in vv["f"] if f.get("o") is not None), default=None)
        print(u"      %s::%s (enum %sB, 페이로드 시작 +0x%s)  ← `--enum %s` 로 볼 것"
              % (t, vv["n"], o.get("sz"), format(o0, "x") if o0 is not None else "?", t))


def pick(name, quiet=False):
    u"""단일 타입으로 확정되면 tystr 반환, 모호하면 None(+안내 출력)."""
    st, cands = resolve_name(name)
    if st == "none":
        if not quiet:
            print(u"[없음] '%s' 는 tcx 사전에 **타입으로는** 없다. (⚠사전에 없다 ≠ 존재하지 않는다 — "
                  u"SDK rlib 밖 타입이거나 제네릭이라 layout 이 없을 수 있다)" % name)
            warn_variant(name, None)
        return None
    tys = [t for _, ts in cands for t in ts]
    if len(tys) == 1:
        return tys[0]
    # 제네릭 인스턴스 여럿(같은 def_path) — 비제네릭 판이 하나면 그것
    plain = [t for t in tys if "<" not in t]
    if len(plain) == 1:
        if not quiet:
            print(u"[주의] 제네릭 인스턴스 %d개 중 비제네릭 판 %s 를 쓴다 (나머지: %s …)"
                  % (len(tys), plain[0], [t for t in tys if t != plain[0]][:3]))
        return plain[0]
    gs = [t for t in tys if (T(t) or {}).get("game")]
    gplain = [t for t in gs if "<" not in t]
    if len(gplain) == 1:
        if not quiet:
            print(u"[주의] 동명 %d개 중 게임 비제네릭 타입은 1개 → %s 를 쓴다" % (len(tys), gplain[0]))
        return gplain[0]
    if not quiet:
        print_ambig(name, cands)
    return None


# ─────────────────────────────────────────────────────── expansion ──
def variant_fields(o, vi):
    v = o["vs"][vi]
    return [(f["n"], f["t"], f["o"]) for f in v["f"] if f.get("o") is not None]


def walk(tystr, base=0, prefix="", depth=0, seen=(), want=None, arr_all=False):
    u"""타입을 절대 오프셋으로 편다.
    want 가 None 이면 전체 leaf 목록, 아니면 want 를 덮는 경로만.
    반환: [(abs_off, dotted_path, tystr, size, note)]"""
    o = T(tystr)
    if o is None:
        return []
    sz = o.get("sz")
    if want is not None and sz is not None and not (base <= want < base + sz):
        return []
    if depth >= MAX_DEPTH or tystr in seen:
        return [(base, prefix.rstrip("."), tystr, sz, u"깊이제한" if depth >= MAX_DEPTH else u"순환")]
    seen = seen + (tystr,)
    k = o.get("k")
    out = []

    if k in ("struct", "union"):
        for n, t, off in variant_fields(o, 0):
            out += _child(t, base + off, prefix + n, depth, seen, want, arr_all)
    elif k == "tuple":
        for f in o.get("f", []):
            if f.get("o") is None:
                continue
            out += _child(f["t"], base + f["o"], prefix + f["n"], depth, seen, want, arr_all)
    elif k == "array":
        cnt, stride = o.get("cnt"), o.get("stride")
        if not cnt or not stride:
            return [(base, prefix.rstrip("."), tystr, sz, u"배열(원소불명)")]
        if want is not None:
            i = (want - base) // stride
            if 0 <= i < cnt:
                out += _child(o["el"], base + i * stride, u"%s[%d]" % (prefix.rstrip("."), i),
                              depth, seen, want, arr_all)
        elif cnt <= ARR_MAX or arr_all:
            for i in range(int(cnt)):
                out += _child(o["el"], base + i * stride, u"%s[%d]" % (prefix.rstrip("."), i),
                              depth, seen, want, arr_all)
        else:
            out.append((base, u"%s[0..%d]" % (prefix.rstrip("."), cnt), o["el"], stride,
                        u"배열 %d개 stride=%d (요약 — --arrall 로 전량)" % (cnt, stride)))
    elif k == "enum":
        tag = o.get("tag") or {}
        if tag.get("off") is not None:
            toff = base + tag["off"]
            if want is None or want == toff:
                out.append((toff, prefix.rstrip(".") + "@tag", "tag:%dB" % (tag.get("sz") or 0),
                            tag.get("sz"), u"판별자 (%s)" % tag.get("enc")))
        for vi, v in enumerate(o["vs"]):
            for n, t, off in variant_fields(o, vi):
                lbl = u"%s@%s.%s" % (prefix.rstrip("."), v["n"], n) if prefix else u"@%s.%s" % (v["n"], n)
                out += _child(t, base + off, lbl, depth, seen, want, arr_all)
    else:
        out.append((base, prefix.rstrip("."), tystr, sz, ""))
    return out


def _child(t, off, label, depth, seen, want, arr_all):
    o = T(t)
    if o is None:
        return [(off, label, t, None, u"타입 미수록")]
    if is_leafish(o):
        if want is not None:
            e = off + (o.get("sz") or 1)
            if not (off <= want < e):
                return []
        return [(off, label, t, o.get("sz"), u"→ %s" % o.get("el") if o.get("el") else "")]
    sub = walk(t, off, label + ".", depth + 1, seen, want, arr_all)
    if want is None:
        # 컨테이너 자체도 leaf 로 한 줄 남긴다(요약 가독성)
        return sub if sub else [(off, label, t, o.get("sz"), "")]
    if sub:
        return sub
    # want 가 이 필드 범위 안이지만 더 안쪽을 못 뚫은 경우
    e = off + (o.get("sz") or 0)
    return [(off, label, t, o.get("sz"), u"내부 +0x%x (관통 실패)" % (want - off))] if off <= want < e else []


# ────────────────────────────────────────────────────────── output ──
def hdr(tystr):
    o = T(tystr)
    sp = o.get("sp") or {}
    nf = len(o["vs"][0]["f"]) if o.get("vs") else 0
    kind = {"struct": "struct", "enum": "enum", "union": "union"}.get(o.get("k"), o.get("k"))
    extra = u"variant %d" % len(o["vs"]) if o.get("k") == "enum" else u"필드 %d" % nf
    print(u"=== %s (%sB · %s · %s) ===" % (tystr, o.get("sz"), kind, extra))
    print(u"    align=%s  vis=%s  src=%s:%s  %s"
          % (o.get("al"), o.get("v"), sp.get("f"), sp.get("l"),
             u"[게임 타입]" if o.get("game") else u"[라이브러리 타입]"))
    warn_variant(tystr, tystr)


def show_fields(tystr, deep=False, arr_all=False):
    o = T(tystr)
    hdr(tystr)
    if o.get("k") == "enum":
        show_enum(tystr)
        return
    if deep:
        rows = walk(tystr, arr_all=arr_all)
        for off, name, t, sz, note in sorted(rows, key=lambda r: (r[0], r[1])):
            print(u"  0x%-7x %-52s %-46s %s" % (off, name, t, (u"(%sB) " % sz if sz is not None else "") + (note or "")))
        print(u"  -- leaf %d개 (절대 오프셋, 규칙: 배열<=%d 전개 / 깊이<=%d)" % (len(rows), ARR_MAX, MAX_DEPTH))
        return
    for n, t, off in sorted(variant_fields(o, 0), key=lambda x: x[2]):
        sub = T(t)
        print(u"  0x%-7x %-34s %-60s %s" % (off, n, t, u"(%sB)" % sub.get("sz") if sub and sub.get("sz") is not None else ""))
    print(u"  -- 최상위 필드만. 중첩 절대오프셋은 `--deep`, 특정 오프셋은 `<타입> <오프셋>`")


def show_offset(tystr, want):
    o = T(tystr)
    hdr(tystr)
    print(u"질의 +0x%x:" % want)
    if o.get("sz") is not None and want >= o["sz"]:
        print(u"  ⚠ 이 타입의 크기(%dB=0x%x)를 넘는다 — 다른 타입일 가능성" % (o["sz"], o["sz"]))
    rows = walk(tystr, want=want)
    if not rows:
        print(u"  [해당 필드 없음] 패딩이거나(=필드 사이 공백), 관통 불가 지점이다.")
        # 가장 가까운 앞 필드
        allr = walk(tystr)
        prev = [r for r in allr if r[0] <= want]
        if prev:
            r = max(prev, key=lambda x: x[0])
            print(u"  가장 가까운 앞 leaf: 0x%-6x %s : %s (%sB)  → 질의는 그 뒤 +%d" % (r[0], r[1], r[2], r[3], want - r[0]))
        return
    for off, name, t, sz, note in sorted(rows, key=lambda r: (-r[0], r[1])):
        mark = u"★" if off == want else u"  "
        print(u"  %s0x%-7x %-52s %-46s %s" % (mark, off, name, t, (u"(%sB) " % sz if sz is not None else "") + (note or "")))


def show_enum(tystr, tagq=None):
    o = T(tystr)
    if o.get("k") != "enum":
        print(u"[주의] %s 는 열거형이 아니다(k=%s). 필드 목록을 보여준다." % (tystr, o.get("k")))
        show_fields(tystr)
        return
    tag = o.get("tag") or {}
    enc = tag.get("enc", "?")
    if tagq is None:
        hdr(tystr)
    print(u"  판별자: enum+0x%s (%sB) · 인코딩=%s%s"
          % (format(tag.get("off"), "x") if tag.get("off") is not None else "?",
             tag.get("sz"), enc,
             u"  [니치: untagged=%d, niche_variants=%d..=%d, niche_start=%d]"
             % (tag.get("untagged"), tag.get("nv0"), tag.get("nv1"), tag.get("nstart"))
             if enc == "Niche" else ""))
    if enc == "Niche":
        print(u"  ⚠ **논리 인덱스 ≠ 메모리 태그**. 태그 = niche_start + (idx − nv0); "
              u"untagged variant(%s) 는 태그가 없다(암묵)." % o["vs"][tag.get("untagged")]["n"]
              if tag.get("untagged") is not None and tag.get("untagged") < len(o["vs"]) else "")
    print(u"  %-5s %-8s %-8s %s" % ("idx", "선언discr", "메모리태그", "variant"))
    for v in o["vs"]:
        if tagq is not None and v["tag"] != tagq:
            continue
        print(u"  %-5d %-8s %-8s %s%s"
              % (v["i"], v["discr"] if v["discr"] is not None else "-",
                 v["tag"] if v["tag"] is not None else u"없음(암묵)",
                 v["n"], u"  (%d필드)" % len(v["f"]) if v["f"] else ""))
    # 페이로드
    for vi, v in enumerate(o["vs"]):
        if tagq is not None and v["tag"] != tagq:
            continue
        if not v["f"]:
            continue
        print(u"\n  페이로드 %s — **enum 선두 기준 절대 오프셋**" % v["n"])
        for n, t, off in sorted(variant_fields(o, vi), key=lambda x: x[2]):
            sub = T(t)
            print(u"    enum+0x%-5x %-34s %-56s %s"
                  % (off, n, t, u"(%sB)" % sub.get("sz") if sub and sub.get("sz") is not None else ""))


def show_ambig_all():
    d = D()
    rows = []
    SERDE = ("__Field", "__FieldVisitor", "__Visitor")   # serde derive 내부 — 잡음
    nserde = 0
    for leaf, paths in d["by_leaf"].items():
        if len(paths) < 2:
            continue
        if leaf in SERDE:
            nserde += 1
            continue
        ngame = sum(1 for p in paths if any(d["types"][t].get("game") for t in d["by_path"][p]))
        rows.append((ngame, len(paths), leaf, paths))
    rows.sort(key=lambda r: (-r[0], -r[1], r[2]))
    ng = sum(1 for r in rows if r[0] >= 2)
    print(u"=== 동명 다중(모호) 전량 — leaf %d개 / 그중 게임타입끼리 충돌 %d개  (serde derive 내부 %d종 제외) ==="
          % (len(rows), ng, nserde))
    print(u"    (leaf 이름 하나만 키로 쓰던 구사전 distruct 가 조용히 하나를 골라 덮어쓰던 지점들)")
    for ngame, n, leaf, paths in rows:
        print(u"\n- %-34s def_path %d개 (게임 %d)" % (leaf, n, ngame))
        for p in sorted(paths):
            for t in sorted(d["by_path"][p]):
                o = d["types"][t]
                sp = o.get("sp") or {}
                print(u"    %-9s %-7s size=%-7s %-72s %s:%s"
                      % (u"게임" if o.get("game") else u"라이브러리", o.get("k"), o.get("sz"), t,
                         sp.get("f", ""), sp.get("l", "")))
    print(u"\n### 별도: 같은 def_path 인데 구체 인스턴스가 여럿인 제네릭 타입")
    gen = [(p, tl) for p, tl in d["by_path"].items() if len(tl) > 1]
    gen.sort(key=lambda x: -len(x[1]))
    ggen = [(p, tl) for p, tl in gen if any(d["types"][t].get("game") for t in tl)]
    print(u"    총 %d개 (그중 게임 타입 %d개). 상위:" % (len(gen), len(ggen)))
    for p, tl in ggen[:25]:
        szs = sorted(set(d["types"][t].get("sz") for t in tl))
        print(u"    %-64s 인스턴스 %-4d sizes=%s" % (p, len(tl), szs[:8]))


def show_stats():
    d = D()
    ts = d["types"]
    game = {t: o for t, o in ts.items() if o.get("game")}
    lib = len(ts) - len(game)
    nk = defaultdict(int)
    nfield = nvariant = 0
    for t, o in game.items():
        nk[o.get("k")] += 1
        for v in o.get("vs", []):
            nvariant += 1
            nfield += len(v["f"])
    print(u"=== tcxdict 통계 (게임 0.5.8 · %s) ===" % DICT)
    print(u"  타입 총 %d개  = 게임 %d + 라이브러리/제네릭 인스턴스 %d" % (len(ts), len(game), lib))
    for k, n in sorted(nk.items(), key=lambda x: -x[1]):
        print(u"    게임 %-8s %d" % (k, n))
    print(u"  게임 타입 필드 %d개 / variant %d개" % (nfield, nvariant))
    print(u"  def_path %d개 / leaf 별칭 %d개 / variant 별칭 %d개"
          % (len(d["by_path"]), len(d["by_leaf"]), len(d.get("by_variant", {}))))
    SERDE = ("__Field", "__FieldVisitor", "__Visitor")
    amb = [(l, p) for l, p in d["by_leaf"].items() if len(p) > 1 and l not in SERDE]
    ambg = [(l, p) for l, p in amb
            if sum(1 for x in p if any(ts[t].get("game") for t in d["by_path"][x])) >= 2]
    gen = [p for p, tl in d["by_path"].items() if len(tl) > 1]
    print(u"  ★모호(동명 다중 def_path) leaf %d개 / 그중 게임타입끼리 %d개" % (len(amb), len(ambg)))
    print(u"   (별도) 같은 def_path 인데 구체 인스턴스가 여럿인 제네릭 = %d개" % len(gen))

    # 기존 사전과의 커버리지
    for fn, kind in (("distruct.json", "struct"), ("dienum.json", "enum")):
        p = os.path.join(HERE, fn)
        if not os.path.exists(p):
            continue
        with io.open(p, encoding="utf-8") as f:
            old = json.load(f)
        oldleaf = set()
        for k in old:
            kk = re.sub(r"^enum2\$<(.*)>$", r"\1", k.strip())
            if "<" in kk:
                continue
            oldleaf.add(kk.rsplit("::", 1)[-1])
        want = set(l for l, ps in d["by_leaf"].items()
                   if any(ts[t].get("game") and ts[t].get("k") == ("enum" if kind == "enum" else "struct")
                          for x in ps for t in d["by_path"][x]))
        vnames = set(d.get("by_variant", {}))
        miss = sorted(want - oldleaf)
        print(u"\n  -- %s (%d키) 대비 커버리지 (%s)" % (fn, len(old), kind))
        print(u"     tcx 게임 %s leaf %d개 중 구사전에 키 없음 = %d개 (%.1f%%)"
              % (kind, len(want), len(miss), 100.0 * len(miss) / max(1, len(want))))
        print(u"     구사전에만 있는 leaf(=제네릭 인스턴스·라이브러리·variant 등) = %d개" % len(oldleaf - want))
        if kind == "struct":
            print(u"     ⚠구사전 키 중 실제로는 '열거형 variant' 인 것 = %d개 (독립 struct 로 읽으면 오프셋이 밀린다)"
                  % len(oldleaf & vnames))
        print(u"     없는 것 예시: %s" % (miss[:14],))


# ──────────────────────────────────────────────────────────── main ──
def main():
    a = sys.argv[1:]
    if not a or a[0] in ("-h", "--help"):
        print(__doc__)
        return
    if a[0] == "--build":
        build()
        return
    if a[0] == "--ambig":
        show_ambig_all()
        return
    if a[0] == "--stats":
        show_stats()
        return
    if a[0] == "--enum":
        t = pick(a[1])
        if not t:
            return
        show_enum(t, int(a[2], 0) if len(a) > 2 else None)
        return
    name = a[0]
    rest = a[1:]
    deep = "--deep" in rest
    arr_all = "--arrall" in rest
    rest = [x for x in rest if not x.startswith("--")]
    t = pick(name)
    if not t:
        return
    if rest:
        show_offset(t, int(rest[0], 0))
    else:
        show_fields(t, deep=deep, arr_all=arr_all)


if __name__ == "__main__":
    main()
