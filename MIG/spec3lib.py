# -*- coding: utf-8 -*-
u"""spec3lib — 명세 v3 의 **게이트 필드를 코퍼스에서 자동으로 채운다.** (2026-09-11 신설)

왜 만드나 — 2026-09-11 1·2차 반증검증에서 나온 누락의 대부분은 "사람이 안 적은 것"이었고,
그 종류가 정확히 셋이었다:
  ① 술어의 시그니처를 안 봐서 IR 상수를 소스 임계로 오독 (01 「정글 티어 컷」 유령 노브)
  ② 호출부를 안 세서 「유일 호출처」라고 씀 (09 — 실제 8곳)
  ③ 형제 진입점을 안 봐서 다른 선택기를 통째로 놓침 (14 — target_bush_v41/target_bush)
셋 다 **기계로 뽑을 수 있다.** 사람이 채우는 필드로 두면 또 빈다 — 그래서 여기서 자동 생성한다.

제공:
  fnlookup(leaf)          이름(leaf)으로 3크레이트 Fn/AssocFn 조회 → 경로·가시성·sp·mir·sig
  cleansig(sig)           tcx sig 의 수명 잡음 제거 → 읽을 수 있는 형태
  callsites(sym, dirs)    IR 코퍼스에서 그 심볼을 call/invoke 하는 지점 전수
  siblings(planty)        같은 Plan 타입의 다른 진입점(define) 전수
"""
import io, json, os, re, functools, subprocess

HERE = os.path.dirname(os.path.abspath(__file__))
TCX = os.path.join(HERE, "_tcx")
CORP = {"gai": r"C:\tfm2mods\_gaibc", "gc": r"C:\tfm2mods\_gcbc", "gv": r"C:\tfm2mods\_gvbc"}
CRATES = ("game_ai", "game_core", "game_view")


@functools.lru_cache(maxsize=8)
def _load(cr):
    with io.open(os.path.join(TCX, cr + ".json"), encoding="utf-8") as f:
        return json.load(f)


# ── 시그니처 정리 ────────────────────────────────────────────────────
# tcx 의 sig 는 `&'^2.Named(DefId(103:8933 ~ game_ai[c893]::...::'_)) game_core::PlayerState`
# 처럼 수명 잡음이 길다. 타입만 남긴다.
_LT = re.compile(r"'\^?\d*\.?Named\(DefId\([^)]*\)\)\s*|'\^\d+\s*|'_#?\d*\s*|'[a-z]+\s+")
_GEN = re.compile(r"<'[^<>]*?(?:<[^<>]*>)?[^<>]*?>")


def cleansig(sig):
    if not sig:
        return None
    s = _LT.sub("", sig)
    for _ in range(6):                       # 중첩 제네릭의 수명 인자 덩어리 제거
        s2 = re.sub(r"<\s*>", "", s)
        s2 = re.sub(r"<\s*,", "<", s2)
        s2 = re.sub(r",\s*>", ">", s2)
        s2 = re.sub(r",\s*,", ",", s2)
        if s2 == s:
            break
        s = s2
    s = re.sub(r"\s{2,}", " ", s).replace("& mut", "&mut").replace("& ", "&")
    return s.strip()


def _leaf(p):
    return p.split("::")[-1]


@functools.lru_cache(maxsize=4096)
def fnlookup(leaf, crate=None):
    u"""이름으로 함수를 찾는다. 반환 = [{crate,path,vis,file,line,mir,xinl,sig}]"""
    out = []
    for cr in ((crate,) if crate else CRATES):
        try:
            d = _load(cr)
        except Exception:
            continue
        for x in d["items"]:
            if x["k"] not in ("Fn", "AssocFn"):
                continue
            if _leaf(x["p"]) != leaf:
                continue
            sp = x.get("sp") or {}
            out.append({
                "crate": cr, "path": x["p"], "vis": x.get("v"),
                "file": sp.get("f"), "line": sp.get("l"), "col": sp.get("c"),
                "mir": bool(x.get("mir")), "xinl": bool(x.get("xinl")),
                "sig": cleansig(x.get("sig")),
            })
    return out


def adt(name, crate=None):
    u"""구조체/열거형 레코드(판별자·레이아웃 포함) 조회."""
    out = []
    for cr in ((crate,) if crate else CRATES):
        try:
            d = _load(cr)
        except Exception:
            continue
        for x in d["items"]:
            if x["k"] in ("Struct", "Enum") and _leaf(x["p"]) == name:
                out.append({"crate": cr, "path": x["p"], "kind": x["k"],
                            "sp": x.get("sp"), "layout": x.get("layout"),
                            "vs": x.get("vs")})
    return out


# ── 호출부 전수 (G2 를 구조적으로 막는다) ─────────────────────────────
# ripgrep 이 PATH 에 없는 환경이라 순수 파이썬으로 스캔한다.
# 바이트 단위 `in` 사전필터를 먼저 거치므로 303MB 코퍼스가 수 초에 끝난다.
CACHE = os.path.join(HERE, "_verify2", "_cache")


def _scan(dirs, needle, linepat):
    u"""needle(바이트)이 든 줄만 골라 linepat 로 확정. 반환 = [{corpus,file,line,text}]"""
    nb = needle.encode("utf-8", "replace")
    rx = re.compile(linepat)
    rows = []
    for k in dirs:
        d = CORP.get(k)
        if not d or not os.path.isdir(d):
            continue
        for fn in sorted(os.listdir(d)):
            if not fn.endswith(".ll"):
                continue
            path = os.path.join(d, fn)
            with io.open(path, "rb") as f:
                for i, raw in enumerate(f, 1):
                    if nb not in raw:
                        continue
                    t = raw.decode("utf-8", "replace").rstrip()
                    if rx.search(t):
                        rows.append({"corpus": k, "file": fn, "line": i,
                                     "text": t.strip()[:220]})
    return rows


def _cached(key, fn):
    try:
        os.makedirs(CACHE, exist_ok=True)
        p = os.path.join(CACHE, re.sub(r"[^A-Za-z0-9_.-]", "_", key)[:150] + ".json")
        if os.path.exists(p):
            with io.open(p, encoding="utf-8") as f:
                return json.load(f)
        v = fn()
        with io.open(p, "w", encoding="utf-8") as f:
            f.write(json.dumps(v, ensure_ascii=False))
        return v
    except Exception:
        return fn()


def callsites(sym, dirs=("gai", "gc")):
    u"""IR 코퍼스에서 `sym` 을 call/invoke 하는 지점 전수(define 줄 제외).
    ★이 함수가 있으면 「유일 호출처」를 세지 않고 쓰는 일이 구조적으로 불가능해진다."""
    return _cached("calls_" + sym + "_" + "".join(dirs),
                   lambda: _scan(dirs, sym,
                                 r"(?:call|invoke).*@" + re.escape(sym)
                                 + r"(?![A-Za-z0-9_])"))


def defsite(sym, dirs=("gai", "gc", "gv")):
    u"""define 이 어느 코퍼스에 있는지. 「_gaibc 에 없다」를 「본문 없음」으로 오독하는
    사고(METHOD_MAP ① 규칙2)를 막는다."""
    return _cached("def_" + sym + "_" + "".join(dirs),
                   lambda: _scan(dirs, sym,
                                 r"^define.*@" + re.escape(sym)
                                 + r"(?![A-Za-z0-9_])"))


# ── 형제 진입점 (G3 를 구조적으로 막는다) ─────────────────────────────
PLAN_ENTRY = ("sub_plan", "next_plan", "is_end", "update", "is_cancel",
              "on_enter", "on_exit", "new")


def siblings(planty):
    u"""같은 Plan 타입에 달린 함수 전수(tcx 기준). 14 가 target_bush_v41 을 놓친 종류의 누락을 막는다."""
    out = []
    for cr in CRATES:
        try:
            d = _load(cr)
        except Exception:
            continue
        for x in d["items"]:
            if x["k"] not in ("Fn", "AssocFn"):
                continue
            if ("::" + planty + "::") not in ("::" + x["p"] + "::"):
                # impl 메서드는 p 에 타입명이 들어간다
                if planty not in x["p"]:
                    continue
            sp = x.get("sp") or {}
            out.append({"crate": cr, "path": x["p"], "vis": x.get("v"),
                        "file": sp.get("f"), "line": sp.get("l"),
                        "mir": bool(x.get("mir")), "sig": cleansig(x.get("sig"))})
    # 같은 타입 이름을 부분문자열로 포함하는 다른 타입 제거
    out = [o for o in out if re.search(r"\b" + re.escape(planty) + r"\b", o["path"])]
    out.sort(key=lambda o: (o["file"] or "", o["line"] or 0))
    return out


if __name__ == "__main__":
    import sys
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    cmd = sys.argv[1] if len(sys.argv) > 1 else "help"
    if cmd == "fn":
        for r in fnlookup(sys.argv[2]):
            print(u"%-10s %-9s %s" % (r["crate"], r["vis"], r["path"]))
            print(u"    %s:%s  mir=%s xinl=%s" % (r["file"], r["line"], r["mir"], r["xinl"]))
            print(u"    %s" % (r["sig"] or u"(sig 없음)"))
    elif cmd == "calls":
        rows = callsites(sys.argv[2])
        print(u"호출부 %d곳" % len(rows))
        for r in rows:
            print(u"  %s:%d  %s" % (r["file"], r["line"], r["text"][:120]))
    elif cmd == "def":
        for r in defsite(sys.argv[2]):
            print(u"  %s / %s:%d" % (r["corpus"], r["file"], r["line"]))
    elif cmd == "sib":
        for r in siblings(sys.argv[2]):
            print(u"  %-9s %s  (%s:%s) mir=%s" % (r["vis"], r["path"], r["file"], r["line"], r["mir"]))
    else:
        print(__doc__)
