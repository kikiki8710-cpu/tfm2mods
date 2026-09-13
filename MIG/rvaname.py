#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""rvaname.py — exe RVA 목록의 **IR 실명**을 패닉 Location 지문으로 일괄 판정한다. (2026-09-13 신설 · fnprobe+locfind 합성)
왜: 선별표 라벨은 dllmatch 추정이라 틀린다(r8: 11건 중 9건 오라벨). 묶음 편성 전에 한 번에 확정하고 IR define 줄까지 받아 배치 지시문에 쓴다.
사용: python -X utf8 MIG\\rvaname.py <rva> [<rva> ...]      (rva = 16진 · 0x 유무 무관)
출력: rva · Location(최대 3) · 후보 define(심볼 · 파일:줄) — 클로저/이터레이터 인스턴스(`INvX`/`{closure}`/`4core4iter`)는 뒤로 밀고 표시.
판정 어휘: 본체 후보가 1개면 「확정」, 인스턴스만이면 「클로저」, Location 이 없으면 「지문 없음(offscan 로)」.
"""
import io, os, re, subprocess, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
IR = r"C:\tfm2mods\_gaibc"

def run(args):
    r = subprocess.run([sys.executable, "-X", "utf8"] + args, capture_output=True, text=True, encoding="utf-8", errors="replace")
    return r.stdout

def locs(rva):
    out = run([os.path.join(HERE, "fnprobe.py"), rva])
    seen = []
    for m in re.finditer(r"(game-\w+\\[^\s]+?):(\d+):(\d+)", out):
        t = (m.group(1), int(m.group(2)), int(m.group(3)))
        if t not in seen: seen.append(t)
    return seen

def defs_for(f, line, col):
    out = run([os.path.join(HERE, "locfind.py"), os.path.basename(f), str(line), str(col)])
    res = []
    cur = None
    for ln in out.split("\n"):
        m = re.match(r"(m\d\d\.ll)\s+@", ln)
        if m: cur = m.group(1)
        m2 = re.search(r"ref-in define: (\S+)", ln)
        if m2: res.append((cur, m2.group(1)))
    return res

def is_inst(sym):
    # v0 망글: `_RNv`/`_RNvM` = 본체 · `_RINv…` = 제네릭 인스턴스 · `_RNC…` = 클로저 · `_RNvX` = 트레이트 impl(대개 core 인스턴스)
    return not (sym.startswith("_RNvM") or sym.startswith("_RNvNt") or sym.startswith("_RNvC"))

def main():
    for a in sys.argv[1:]:
        rva = a.lower().replace("0x", "")
        L = locs(rva)
        print(u"■ 0x%s  Location %d개: %s" % (rva, len(L), u" · ".join(u"%s:%d:%d" % (os.path.basename(f), l, c) for f, l, c in L[:3])))
        if not L:
            print(u"   지문 없음 → offscan.py(변위) 로"); continue
        cands = {}   # sym -> (hit count, files)
        LA = [t for t in L if t[0].startswith("game-ai")][:4]   # game_core 줄(simulation.rs 등)은 어디에나 인라인돼 변별력 0
        for f, l, c in LA:
            for mf, sym in set(defs_for(f, l, c)):
                e = cands.setdefault(sym, [0, set()]); e[0] += 1; e[1].add(mf)
        body = sorted([(v[0], s, v[1]) for s, v in cands.items() if not is_inst(s)], reverse=True)
        inst = sorted([(v[0], s, v[1]) for s, v in cands.items() if is_inst(s)], reverse=True)
        top = [b for b in body if b[0] == body[0][0]] if body else []
        tag = u"확정" if len(top) == 1 else (u"후보 %d(적중 %d)" % (len(top), top[0][0]) if top else u"클로저/인스턴스만")
        print(u"   [%s]" % tag)
        for n, s, m in body[:3]: print(u"   본체 %d/%d  %s  (%s)" % (n, len(LA), s[:150], ",".join(sorted(m))))
        for n, s, m in inst[:2]: print(u"   인스턴스 %d  %s  (%s)" % (n, s[:110], ",".join(sorted(m))))

if __name__ == "__main__":
    main()
