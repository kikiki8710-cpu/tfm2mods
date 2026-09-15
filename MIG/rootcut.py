#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""rootcut.py — 거대 함수 IR 을 **루트 소스 줄**(`!dbg`→`inlinedAt` 최상위) 기준으로 분책 경계를 잘라 준다 (2026-09-15 · r12 `upd_blocks.json` 방식 도구화)

왜: ≥5k IR줄 함수는 한 배치가 못 읽는다(r12 update 14,476줄 → 4분책). 분책 경계는 **함수 자신의 소스 줄**(인라인된 콜리 줄이
    아니라 그 인라인의 최상위 `inlinedAt` 이 가리키는 줄)로 잘라야 배치가 「소스 a~b 줄」을 담당할 수 있다.

사용: python -X utf8 MIG\\rootcut.py <module.ll> <define 줄> [--parts 4] [--json _next\\<id>_blocks.json]
출력: 루트 줄별 IR 줄수 히스토그램 요약 + parts 개로 나눈 `cuts = [[src_from, src_to, ir_count], …]`
      (경계는 루트 줄 사이에서만 · 한 루트 줄이 통째로 한 조각에 들어간다)
"""
import io, re, sys, json, collections


def load_meta(path):
    meta = {}
    with io.open(path, encoding="utf-8", errors="ignore") as f:
        for l in f:
            if l.startswith("!") and ("DILocation" in l or "DISubprogram" in l or "DILexicalBlock" in l):
                m = re.match(r"(!\d+) = (.*)", l.rstrip("\n"))
                if m:
                    meta[m.group(1)] = m.group(2)
    return meta


def root_line(meta, mid, depth=0):
    l = meta.get("!" + mid)
    if l is None or "DILocation" not in l or depth > 64:
        return None
    ia = re.search(r"inlinedAt:\s*!(\d+)", l)
    if ia:
        r = root_line(meta, ia.group(1), depth + 1)
        if r is not None:
            return r
    ln = re.search(r"line:\s*(\d+)", l)
    return int(ln.group(1)) if ln else None


def main():
    a = sys.argv[1:]
    if len(a) < 2:
        print(__doc__); return 2
    path, s = a[0], int(a[1])
    parts = int(a[a.index("--parts") + 1]) if "--parts" in a else 4
    jout = a[a.index("--json") + 1] if "--json" in a else None
    meta = load_meta(path)
    per = collections.Counter(); order = []
    nolines = 0
    with io.open(path, encoding="utf-8", errors="ignore") as f:
        for i, l in enumerate(f, 1):
            if i < s:
                continue
            if i > s and l.startswith("}"):
                break
            m = re.search(r"!dbg !(\d+)", l)
            if not m:
                nolines += 1; continue
            r = root_line(meta, m.group(1))
            if r is None:
                nolines += 1; continue
            per[r] += 1
    lines = sorted(per)
    total = sum(per.values())
    print(u"루트 줄 %d개 · IR(dbg) %d줄 · dbg 없음 %d줄 · 소스 %d~%d" % (len(lines), total, nolines, lines[0], lines[-1]))
    # 균등 분책
    target = total / float(parts)
    cuts = []; acc = 0; frm = lines[0]
    for k, ln in enumerate(lines):
        acc += per[ln]
        last = (k == len(lines) - 1)
        if (acc >= target and len(cuts) < parts - 1) or last:
            cuts.append([frm, ln, acc]); acc = 0
            if not last:
                frm = lines[k + 1]
    for c in cuts:
        print(u"  part %s: 소스 %d~%d · IR %d줄" % (chr(65 + cuts.index(c)), c[0], c[1], c[2]))
    top = per.most_common(8)
    print(u"  큰 루트 줄 top8: " + u" · ".join(u"L%d=%d" % kv for kv in top))
    if jout:
        json.dump({"cuts": cuts, "per": {str(k): per[k] for k in lines}}, io.open(jout, "w", encoding="utf-8"), ensure_ascii=False)
        print(u"→ " + jout)
    return 0


if __name__ == "__main__":
    sys.exit(main())
