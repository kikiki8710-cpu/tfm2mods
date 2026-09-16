#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""extid.py — 지문(패닉 Location) 없는 exe 함수의 IR 짝을 「상수 지문(변위·즉치) + 콜리 이름 + 크기」로 좁힌다. (2026-09-16 신설)

offscan.py 의 반대 방향: offscan 은 IR gep 상수를 주면 exe 를 찾는다. 여기는 exe RVA 를 주면 IR 을 찾는다.
  exe 쪽 지문 = capstone 으로 함수 본문의 메모리 변위(disp)·즉치(imm) 집합(16 미만·2의 거듭제곱·페이지 정렬 값은 잡음이라 제외)
  IR 쪽 지문 = `define` 본문의 `getelementptr … i64 N` / `i32 N`·`i64 N` 상수 집합(같은 잡음 제외) — 캐시 `_next\\ir_consts.json`
  점수 = 공유 상수 수(≥3 이어야 의미) · Jaccard · 크기비(exe 바이트/4 ↔ IR 명령수) · 콜리 이름 일치(extname.py 결과)
사용: python -X utf8 MIG\\extid.py <rva> [<rva> ...]   |   --ext (지도 ext 노드 중 미명명 전부)   [--apply]
출력: 표 + `_next\\extid.json` · --apply 면 확신 후보(공유 ≥4 · 2위와 격차 ≥2)를 `_next\\extname_apply.json` 에 저장(fnmap_ext 가 읽어 이름 `[상수지문]` 으로 붙임)
"""
import io, json, os, re, struct, sys
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_MEM, X86_OP_IMM
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import fnmap_ret, retedges, extname
IRD = r"C:\tfm2mods\_gaibc"
CACHE = os.path.join(HERE, "_next", "ir_consts.json")

def noise(v):
    if v < 16 or v > (1 << 40): return True
    if v & (v - 1) == 0: return True          # 2 의 거듭제곱
    if v % 4096 == 0: return True
    return False

def ir_consts():
    if os.path.exists(CACHE): return json.load(io.open(CACHE, encoding="utf-8"))
    idx = {}
    for k in range(16):
        p = os.path.join(IRD, "m%02d.ll" % k)
        if not os.path.exists(p): continue
        cur = None; ins = 0; cs = set()
        for ln in io.open(p, encoding="utf-8", errors="replace"):
            if ln.startswith("define "):
                m = re.search(r"@([^\s(]+)\(", ln); cur = m.group(1).strip('"') if m else None; ins = 0; cs = set(); continue
            if cur is None: continue
            if ln.startswith("}"):
                idx[cur] = {"f": "m%02d" % k, "ins": ins, "k": sorted(cs)}; cur = None; continue
            s = ln.strip()
            if not s or s.startswith(";") or s.endswith(":"): continue
            ins += 1
            for m in re.finditer(r"\bi(?:8|16|32|64) (-?\d+)\b", s):
                v = abs(int(m.group(1)))
                if not noise(v): cs.add(v)
    json.dump(idx, io.open(CACHE, "w", encoding="utf-8"))
    return idx

def exe_consts(md, img, start, end):
    cs = set(); ncall = 0
    for ins in md.disasm(img[start:end], start):
        if ins.mnemonic in ("call", "jmp") and ins.operands and ins.operands[0].type == X86_OP_IMM: ncall += 1
        for op in ins.operands:
            if op.type == X86_OP_MEM and op.mem.disp:
                v = abs(op.mem.disp)
                if op.mem.base == 0 or ins.mnemonic in ("lea",) and op.mem.base in (0,): continue
                if not noise(v): cs.add(v)
            elif op.type == X86_OP_IMM:
                v = abs(op.imm)
                if not noise(v): cs.add(v)
    return cs, ncall

def main():
    av = sys.argv[1:]
    H = fnmap_ret.DEF_MAP
    h = io.open(H, encoding="utf-8").read()
    dmap = json.loads(re.search(r'<script id="fndata" type="application/json">(.*?)</script>', h, re.S).group(1))
    BY = {n["a"]: n for n in dmap}
    if "--ext" in av:
        rvas = [n["a"] for n in dmap if n.get("L") == "ext" and (not n.get("n") or n.get("v") == "cg")]
    else:
        rvas = [x.lower().replace("0x", "") for x in av if not x.startswith("--")]
    starts, ends = retedges.pdata_starts()
    pe = pefile.PE(retedges.EXE, fast_load=True); img = pe.get_memory_mapped_image()
    md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True
    idx = ir_consts()
    # 콜리 이름 일치(extname.json 이 있으면)
    EN = os.path.join(HERE, "_next", "extname.json")
    en = json.load(io.open(EN, encoding="utf-8")) if os.path.exists(EN) else {}
    # ★idf 가중: 어디에나 있는 상수(Debug::fmt 류가 수백 개 공유)는 0 에 가깝게 · 희귀 상수는 크게. fmt/Debug/Display 인스턴스는 후보에서 제외.
    import math
    df = {}
    for sym, info in idx.items():
        for v in info["k"]: df[v] = df.get(v, 0) + 1
    NF = len(idx)
    idf = lambda v: math.log(NF / (1 + df.get(v, 0)))
    skip = lambda sym: any(t in sym for t in ("3fmt", "5Debug", "7Display", "9Formatter"))
    res = {}; apply = {}
    L = [u"# exe → IR 정체 추정(extid · 상수 지문)", u"", u"| rva | 현재 이름 | exe B / 상수 | IR 후보 1(공유·IR명령·크기비) | 후보 2 | 판정 |", u"|---|---|---|---|---|---|"]
    for a in rvas:
        s = int(a, 16); e = ends.get(s)
        if not e: continue
        cs, ncall = exe_consts(md, img, s, e)
        rows = []
        exe_ins = max(1, (e - s) / 4.0)
        for sym, info in idx.items():
            if skip(sym): continue
            shared = cs.intersection(info["k"])
            if len(shared) < 2: continue
            ratio = min(info["ins"], exe_ins) / max(info["ins"], exe_ins)
            if ratio < 0.15: continue
            w = sum(idf(v) for v in shared)                       # 희귀 상수 가중 합
            jac = len(shared) / max(1, len(cs | set(info["k"])))
            bonus = 1.0 if any(sym == c[0] for c in en.get(a, [])[:3]) else 0.0
            rows.append((w + bonus * 6 + jac * 4, len(shared), jac, ratio, sym, info["ins"], bonus))
        rows.sort(key=lambda r: (-r[0], -r[3]))
        top = rows[:5]
        res[a] = [[r[4], r[1], round(r[2], 3), round(r[3], 2), r[5], r[6]] for r in top]
        nm = lambda sym: u"::".join([x for x in extname.idents(sym) if x not in ("game_ai", "plan_legacy", "sub_plan")][-3:]) or sym[:40]
        verdict = u"—"
        if top and top[0][1] >= 3 and top[0][3] >= 0.3 and (len(top) < 2 or top[0][0] >= top[1][0] * 1.5):
            verdict = u"★확신(공유 %d · 점수 %.1f vs %.1f)" % (top[0][1], top[0][0], top[1][0] if len(top) > 1 else 0)
            apply[a] = {"sym": top[0][4], "name": nm(top[0][4]), "shared": top[0][1], "ins": top[0][5], "ratio": round(top[0][3], 2)}
        elif top and top[0][1] >= 3:
            verdict = u"후보(공유 %d)" % top[0][1]
        def cell(r): return u"`%s` 공유 %d · 점수 %.1f · IR %d · 비 %.2f%s" % (nm(r[4]), r[1], r[0], r[5], r[3], u" · 콜리✓" if r[6] else u"")
        L.append(u"| `%s` | %s | %dB / %d | %s | %s | %s |" % (a, (BY.get(a) or {}).get("n") or u"—", e - s, len(cs), cell(top[0]) if top else u"—", cell(top[1]) if len(top) > 1 else u"—", verdict))
        print(u"%s %-36s exeB=%5d consts=%3d  %s  %s" % (a, ((BY.get(a) or {}).get("n") or "-")[:36], e - s, len(cs), verdict, nm(top[0][4]) if top else ""))
    json.dump(res, io.open(os.path.join(HERE, "_next", "extid.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    io.open(os.path.join(HERE, "_next", "extid.md"), "w", encoding="utf-8").write(u"\n".join(L))
    if "--apply" in av:
        P = os.path.join(HERE, "_next", "extname_apply.json")
        old = json.load(io.open(P, encoding="utf-8")) if os.path.exists(P) else {}
        old.update(apply); json.dump(old, io.open(P, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print(u"apply %d → %s" % (len(apply), P))

if __name__ == "__main__":
    main()
