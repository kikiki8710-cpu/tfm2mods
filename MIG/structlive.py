#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""structlive.py — tcx `--deep` 레이아웃에서 **구조체의 살아있는 바이트 맵**(패딩·Vec 삼중항 제외 · 열거형/Option 은
   variant 조건부)을 만들고, `gensweep20.py` 가 쓰는 파이썬 리터럴로 낸다.

왜 = 스택 alloca 에서 만들어 memcpy 되는 구조체(BigPlan 페이로드 등)는 **안 쓰는 칸에 스택 잔재**가 들어와 원시 바이트 비교가
     거짓 DIFF 를 낸다(2026-09-13 `#11`/`#12`: 페이로드 376B 를 통째로 제외해 「범위한정」에 머묾). 타입이 아는 「살아있는 칸」만
     비교하면 그 제외가 사라진다.

원리 = `tcxdict.py <타입> --deep` 의 잎 목록(절대 오프셋·크기·경로)을 읽어:
   · `NAME@tag tag:NB (Direct|Niche)`  → 열거형 판별자. Direct 면 그 바이트가 live. Niche 인데 같은 오프셋에 `.buf.inner.cap`
     이 있으면 **cap 니치**(`UsizeNoHighBit`: None = 상위비트) ⟹ 바이트 비교 대신 「양쪽 None/Some 일치」만.
   · `NAME@V.…` 잎 → 열거형 NAME 이 variant V 일 때만 live. V 의 값 = Option 이면 None=0/Some=1(Direct) 또는 상위비트(cap 니치),
     그 밖의 열거형은 `tcxdict --enum <필드타입>` 으로 메모리태그를 찾는다(못 찾으면 **제외**하고 보고).
   · `.buf.inner.cap`/`.buf.inner.ptr`/`.len` (Vec 부품) → 제외(HEAP_SUBST 가 len·내용을 따로 비교한다).
   · `(0B)` 마커 → 제외. 나머지 잎 → live.
출력 형식(파이썬) = [(off, len, [cond, …])]  cond = ("direct", tag_off, tag_len, value) | ("notin", tag_off, tag_len, [values])
                 | ("hib", off, want_some: bool) | ("hibeq", off)   — notin = untagged(암묵) variant · hib* = Option<Vec> cap 니치

사용: python MIG\\structlive.py <구조체 타입> [--base 0x8]   → 리터럴 출력 (base 는 잎 오프셋에 더할 값)
"""
import io, re, subprocess, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
TCX = r"C:\tfm2mods\MIG\tcxdict.py"

def run(*a):
    return subprocess.run([sys.executable, TCX] + list(a), capture_output=True).stdout.decode("utf-8", "replace").split("\n")

def field_types(ty):
    """비-deep 목록: 필드명 → 선언 타입."""
    out = {}
    for l in run(ty):
        m = re.match(r"\s+0x[0-9a-f]+\s+(\S+)\s+(\S.*?)\s+\((\d+)B\)", l)
        if m: out[m.group(1)] = m.group(2).strip()
    return out

def enum_tags(ty):
    """열거형 타입 → {variant 이름: 메모리태그(int)}, untagged 는 None. (Option<T> 도 tcx --enum 이 지원)"""
    out = {}
    for l in run("--enum", ty):
        m = re.match(r"\s+(\d+)\s+(\d+)\s+(\S+)\s+(\w+)", l)
        if m:
            out[m.group(4)] = int(m.group(3)) if m.group(3).isdigit() else None
    return out

def enum_variant_fields(ty):
    """열거형 타입 → {variant: {필드명: 타입}} (tcx --enum 의 페이로드 표)."""
    out, cur = {}, None
    for l in run("--enum", ty):
        mp = re.match(r"\s+페이로드 (\w+) —", l)
        if mp: cur = mp.group(1); out.setdefault(cur, {}); continue
        mf = re.match(r"\s+enum\+0x[0-9a-f]+\s+(\S+)\s+(\S.*?)\s+\((\d+)B\)", l)
        if mf and cur is not None:
            out[cur][mf.group(1)] = mf.group(2).strip()
    return out

_TYCACHE = {}
def path_type(root, prefix):
    """루트 구조체 `root` 안의 잎 경로 접두 `prefix`(예 `main_objective@Some.0@ComebackPick.line`)의 선언 타입.
       seg0 = 구조체 필드 점경로 · 이후 seg = `Variant[.필드…]` (열거형 → variant 페이로드 → 필드). 못 찾으면 None."""
    key = (root, prefix)
    if key in _TYCACHE: return _TYCACHE[key]
    cur = root
    try:
        for i, seg in enumerate(prefix.split("@")):
            parts = seg.split(".")
            if i == 0:
                fields = parts
            else:
                vf = enum_variant_fields(cur)
                cur = vf[parts[0]][parts[1]] if len(parts) > 1 else None
                fields = parts[2:]
            for f in fields:
                cur = field_types(cur)[f]
    except (KeyError, TypeError):
        cur = None
    _TYCACHE[key] = cur
    return cur

def build(ty, base=0):
    leaves = []
    for l in run(ty, "--deep"):
        m = re.match(r"\s+0x([0-9a-f]+)\s+(\S+)\s+(.*?)\s+\((\d+)B\)(.*)$", l)
        if not m: continue
        off, path, typ, sz, rest = int(m.group(1), 16), m.group(2), m.group(3).strip(), int(m.group(4)), m.group(5)
        leaves.append((off, path, typ, sz, rest))
    ftypes = field_types(ty)
    # 열거형 노드: path 접두(태그 앞부분) → (off, len, enc, cap니치?)
    enums = {}
    caps = {(o, p.split("@")[0]) for (o, p, t, s, r) in leaves if ".buf.inner.cap" in p}
    for (o, p, t, s, r) in leaves:
        if p.endswith("@tag"):
            name = p[:-4]
            enc = "Niche" if "Niche" in r else "Direct"
            capn = any(co == o for (co, cp) in caps if cp.startswith(name.split("@")[0] + "@") or cp == name)
            enums[name] = (o, s, enc, capn)
    # 태그 값 해석 캐시: (열거형 path) → {variant: memtag|None(untagged)}
    vals = {}
    def variant_tags(ename):
        if ename not in vals:
            ft = path_type(ty, ename)
            vals[ename] = enum_tags(ft) if ft else {}
        return vals[ename]
    live, warn = [], []
    for (o, p, t, s, r) in leaves:
        if s == 0: continue
        if ".buf.inner.cap" in p or ".buf.inner.ptr" in p or p.endswith(".len") or ".buf." in p: continue
        conds, bad = [], False
        # 경로의 @V 마다 조건
        segs = p.split("@")
        prefix = segs[0]
        for seg in segs[1:]:
            vn = seg.split(".")[0]
            if vn == "tag":
                continue
            eo, el, enc, capn = enums.get(prefix, (None, None, None, None))
            if eo is None:
                bad = True; warn.append("enum node 없음: " + prefix); break
            if capn:
                conds.append(("hib", base + eo, vn == "Some"))
            else:
                tags = variant_tags(prefix)
                if vn not in tags:
                    bad = True; warn.append("variant 값 미해석: %s@%s (타입 %s)" % (prefix, vn, path_type(ty, prefix))); break
                v = tags[vn]
                if v is None:   # untagged(암묵) variant = 다른 variant 의 태그값이 아닐 때
                    conds.append(("notin", base + eo, el, sorted(x for x in tags.values() if x is not None)))
                else:
                    conds.append(("direct", base + eo, el, v))
            prefix = prefix + "@" + seg.split(".")[0] + (("." + ".".join(seg.split(".")[1:])) if "." in seg else "")
        if bad: continue
        if p.endswith("@tag"):
            eo, el, enc, capn = enums[p[:-4]]
            if capn:
                live.append((base + o, s, conds + [("hibeq", base + o)]))   # None/Some 일치만
                continue
        live.append((base + o, s, conds))
    return live, sorted(set(warn))


# ── 열거형 단위 빌더(+캐시) — gensweep20.py 가 import 해서 쓴다 ──────────────────
import json, os
CACHE = r"C:\tfm2mods\MIG\_structlive_cache.json"

def build_enum(enum_type, payload_base=0x8, use_cache=True):
    """열거형 타입 → {메모리태그: (페이로드 타입, [(off,len,conds)…])}. untagged(암묵) variant 는 키 -1(페이로드 base 0)."""
    cache = {}
    if use_cache and os.path.exists(CACHE):
        try: cache = json.load(io.open(CACHE, encoding="utf-8"))
        except Exception: cache = {}
    key = "%s@%#x" % (enum_type, payload_base)
    if key in cache:
        return {int(k): (v[0], [tuple(x) if not isinstance(x, list) else (x[0], x[1], [tuple(c) for c in x[2]]) for x in v[1]]) for k, v in cache[key].items()}
    out = {}
    cur = None
    lines = run("--enum", enum_type)
    tags = {}
    for l in lines:
        m = re.match(r"\s+(\d+)\s+(\d+)\s+(\S+)\s+(\w+)", l)
        if m: tags[m.group(4)] = int(m.group(3)) if m.group(3).isdigit() else None
    for l in lines:
        mp = re.match(r"\s+페이로드 (\w+) —", l)
        if mp: cur = mp.group(1); continue
        mf = re.match(r"\s+enum\+0x([0-9a-f]+)\s+\S+\s+(\S.*?)\s+\((\d+)B\)", l)
        if mf and cur is not None:
            off, pty, sz = int(mf.group(1), 16), mf.group(2), int(mf.group(3))
            tg = tags.get(cur)
            if sz == 0: cur = None; continue
            if tg is None: tg = -1   # untagged(암묵) variant = 페이로드가 enum+0x0 에서 시작, 키 -1 (codegen 은 `_` arm)
            live, warn = build(pty, off)
            out[tg] = (pty, live)
            if warn: sys.stderr.write("[structlive] %s(%s): %s\n" % (enum_type, cur, "; ".join(warn)))
            cur = None
    cache[key] = {str(k): [v[0], [[x[0], x[1], [list(c) for c in x[2]]] for x in v[1]]] for k, v in out.items()}
    io.open(CACHE, "w", encoding="utf-8").write(json.dumps(cache, ensure_ascii=False, indent=0))
    return out

def enum_tag_range(enum_type):
    """열거형의 **태그가 있는** variant 의 메모리태그 (최소, 최대). untagged(암묵) 판정 = 이 범위 밖."""
    ts = [v for v in enum_tags(enum_type).values() if v is not None]
    return (min(ts), max(ts)) if ts else (0, 0)

def main():
    ty = sys.argv[1]
    base = int(next((sys.argv[i + 1] for i, a in enumerate(sys.argv) if a == "--base"), "0"), 0)
    live, warn = build(ty, base)
    print("# %s — structlive.py 자동 생성 · base=%#x · 잎 %d · 경고 %d" % (ty, base, len(live), len(warn)))
    for w in warn: print("#   ⚠ " + w)
    print("[")
    for o, s, c in live:
        print("    (%#x, %d, %r)," % (o, s, c))
    print("]")

if __name__ == "__main__":
    main()
