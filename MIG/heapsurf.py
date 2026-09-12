#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""heapsurf.py — `&mut self` 함수의 **전이적 힙 쓰기 표면**을 루트 self 기준 절대 오프셋으로 전수한다.

왜 = 런타임 sweep 이 self 를 바이트 스냅샷·되돌리기로 두 번 굴릴 때, 내 사본이 게임 소유 힙(Vec 버퍼 등)을
     grow/drop 하면 힙이 깨진다(0xc0000374). 그래서 **어느 self 필드가 grow_one/drop 대상인지**를 알아야 하는데,
     본문 1차 gep 만 보면 놓친다 — `#12 handle_chat` 실사고(2026-09-13): `handle_chat_inner` → `BattlePlan::update`
     → `update_v32` → … 로 `&mut TeamPlan`(self+0xf8)이 5단계 아래까지 내려가 거기서 push 했다.

원리 = 본문의 `getelementptr ... ptr %base, i64 N` 체인으로 SSA → (루트, 오프셋) 을 만들고,
       ①`grow_one/drop_glue/drop_in_place(ptr X)` 의 X 가 self 파생이면 절대 오프셋으로 기록
       ②`call/invoke @F(..., X, ...)` 에서 X 가 self 파생 **가변**(readonly 아님) 포인터면 F 의 그 파라미터를
         새 루트로 삼아 **재귀**(오프셋 누적). `readonly`/`readnone` 인자는 안 내려간다.

사용: python MIG\\heapsurf.py <심볼 조각> [self=%0] [--depth 6]
출력: (kind, self+off, 경로) 목록 — 그 오프셋들이 HEAP_SUBST 에 들어가야 할 후보다.
"""
import glob, io, re, sys
from collections import defaultdict

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
IRDIR = r"C:\tfm2mods\_gaibc"

_files = None
def files():
    global _files
    if _files is None:
        _files = {}
        for p in sorted(glob.glob(IRDIR + r"\*.ll")):
            _files[p] = io.open(p, encoding="utf-8", errors="replace").read().split("\n")
    return _files

_defidx = None
def find_define(sym):
    """심볼(전체) → (파일, 시작줄 idx, 끝줄 idx). 없으면 None(declare 만 = 다른 크레이트)."""
    global _defidx
    if _defidx is None:
        _defidx = {}
        for p, ls in files().items():
            for i, l in enumerate(ls):
                if l.startswith("define"):
                    m = re.search(r"@(_RN\w+)\(", l)
                    if m: _defidx[m.group(1)] = (p, i)
    if sym not in _defidx: return None
    p, i = _defidx[sym]
    ls = files()[p]
    j = next(k for k in range(i, len(ls)) if ls[k] == "}")
    return p, i, j

def param_names(defline):
    """define 줄의 파라미터 목록 → [(이름, readonly?)]."""
    inner = defline[defline.index("(") + 1:]
    out = []
    for a in re.split(r",(?![^(]*\))", inner):
        m = re.search(r"(%\d+)\s*[\),]", a) or re.search(r"(%\d+)\s*$", a)
        if m: out.append((m.group(1), ("readonly" in a) or ("readnone" in a)))
    return out

SKIP = ("llvm.", "grow_one", "drop_glue", "drop_in_place", "panic", "3fmt", "format", "9unwrap_failed",
        "expect_failed", "slice_index", "core9panicking", "alloc7raw_vec")

def analyze(sym, self_param, base_off, depth, seen, path, out):
    key = (sym, self_param, base_off)
    if key in seen or depth < 0: return
    seen.add(key)
    d = find_define(sym)
    if not d:
        out.append(("?declare-only", base_off, path + [sym[-40:] + "(외부 크레이트 — 미분석)"])); return
    p, i, j = d
    ls = files()[p]
    body = ls[i:j + 1]
    origin = {self_param: base_off}
    for l in body:
        l0 = l.split(", !dbg")[0]
        m = re.match(r"\s*(%\d+) = getelementptr inbounds(?: nuw)?(?: i8)?[^,]*, ptr (%\d+), i64 (\d+)\s*$", l0) or \
            re.match(r"\s*(%\d+) = getelementptr inbounds(?: nuw)? i8, ptr (%\d+), i64 (\d+)", l0)
        if m and m.group(2) in origin:
            origin[m.group(1)] = origin[m.group(2)] + int(m.group(3))
    for l in body:
        l0 = l.split(", !dbg")[0]
        m = re.search(r"(grow_one|drop_glue|drop_in_place)\w*\(ptr[^%]*(%\d+)", l0)
        if m and m.group(2) in origin:
            out.append((m.group(1), origin[m.group(2)], path + [sym[-40:]]))
        m = re.search(r"(?:call|invoke)[^@]*@(_RN\w+)\((.*)$", l0)
        if not m: continue
        callee = m.group(1)
        if any(x in callee for x in SKIP): continue
        args = [a.strip() for a in re.split(r",(?![^(]*\))", m.group(2))]
        for k, a in enumerate(args):
            mv = re.search(r"(%\d+)\s*\)?\s*$", a)
            if not mv or mv.group(1) not in origin: continue
            if "readonly" in a or "readnone" in a: continue
            dd = find_define(callee)
            if not dd:
                out.append(("?declare-only", origin[mv.group(1)], path + [sym[-40:], callee[-40:] + " arg%d" % k])); continue
            pn = param_names(files()[dd[0]][dd[1]])
            if k < len(pn) and not pn[k][1]:
                analyze(callee, pn[k][0], origin[mv.group(1)], depth - 1, seen, path + [sym[-40:]], out)

def main():
    frag = sys.argv[1]
    selfp = next((a for a in sys.argv[2:] if a.startswith("%")), "%0")
    depth = int(next((sys.argv[i + 1] for i, a in enumerate(sys.argv) if a == "--depth"), 6))
    find_define("")  # 인덱스 빌드
    syms = [s for s in _defidx if frag in s]
    if not syms:
        print("define 없음:", frag); return
    for sym in syms[:1]:
        print("root:", sym[-80:], "self=%s depth=%d" % (selfp, depth))
        out = []
        analyze(sym, selfp, 0, depth, set(), [], out)
        agg = defaultdict(lambda: [0, None])
        for kind, off, path in out:
            a = agg[(kind, off)]; a[0] += 1
            if a[1] is None or len(path) < len(a[1]): a[1] = path
        print("--- 전이적 self 힙 표면 (kind, self+off, 횟수, 최단 경로)")
        for (kind, off), (n, path) in sorted(agg.items(), key=lambda x: (x[0][1], x[0][0])):
            print("  %-14s self+%#06x ×%-3d %s" % (kind, off, n, " → ".join(path[-3:])))
        if not agg: print("  (없음)")

if __name__ == "__main__":
    main()
