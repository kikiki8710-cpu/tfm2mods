#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""mig060.py — 분석한 함수들이 0.6.0 exe 에서 어떻게 바뀌었나 (SDK 없이 exe↔exe 만으로). (2026-09-16 신설)

왜: 0.6.0 은 전면 재컴파일(함수 +15,147 · skel 전역 대응 25%)이라 바이트/스켈레톤 지문이 약하다. game_ai 함수엔 더 강한 지문이 있다 —
    **패닉 Location 집합**(`core::panic::Location{file,len,line,col}` 를 `lea r,[rip+d]` 로 참조). 소스가 안 바뀐 함수는 (파일,줄,열) 집합이 그대로,
    줄만 밀린 함수는 (파일,열) 집합이 같고 줄 델타가 상수다. 그 다음 skel/head 해시(fnindex.pkl) · 크기 · 니모닉 히스토그램 · 즉치 집합 · 콜리 수로 변화 등급을 매긴다.

절차:
  ① 두 exe 의 .rdata 에서 Location 구조체 인덱스(주소 → (file,line,col)) · .text 전수 regex 로 `lea` rip-rel 타깃 → 소유 함수(.pdata) → 함수별 Location 집합
  ② 대상(0.5.8 RVA 목록 = 명세 264 + AI함수지도 노드) 마다 0.6.0 후보: 정확 집합 일치 → (file,col) 집합 + 상수 줄델타 → skel/head 해시 → 미발견
  ③ 짝마다 등급: 동일(skel 동일) / 이동만(크기·mnem·즉치 동일) / 소폭(크기 ±5%·즉치 ⊆) / 변경(그 외) · 근거 수치
  ④ 출력 `_next\\mig060_judge.{json,md}` (+ `--map` 으로 지도 fndata 에 `v060` 필드)

사용: python -X utf8 MIG\\mig060.py [--only 264|map] [--max N]
"""
import io, json, os, re, struct, sys, bisect, pickle, collections, math
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_IMM, X86_OP_MEM
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
OLD = r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.8\TeamfightManager2.exe"
NEW = r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.6.0\TeamfightManager2.exe"
PKL = {"old": r"C:\tfm2mods\_fnidx_058.pkl", "new": r"C:\tfm2mods\_fnidx_060.pkl"}
CG = {"old": r"C:\tfm2mods\_cg_058.pkl", "new": r"C:\tfm2mods\_cg_060.pkl"}
BASE = 0x140000000
MAP = r"C:\Users\jungs\Desktop\claude\tfm2\.claude\worktrees\swap-order-button-style-fe9e04\mods_report\tfm2_ai_adjust\AI함수지도.html"
SPEC = os.path.join(HERE, "_spec", "specs20_v3.json")
CACHE = os.path.join(HERE, "_next", "mig060_locidx_%s.pkl")

class Exe:
    def __init__(s, path, tag):
        s.path = path; s.tag = tag
        s.pe = pefile.PE(path, fast_load=True); s.img = s.pe.get_memory_mapped_image()
        s.secs = {x.Name.rstrip(b"\0").decode(): (x.VirtualAddress, max(x.Misc_VirtualSize, x.SizeOfRawData)) for x in s.pe.sections}
        pd = [x for x in s.pe.sections if x.Name.startswith(b".pdata")][0]
        raw = s.img[pd.VirtualAddress: pd.VirtualAddress + pd.Misc_VirtualSize]
        s.ends = {}
        for i in range(0, len(raw) - 11, 12):
            b, e, u = struct.unpack_from("<III", raw, i)
            if b == 0: break
            s.ends[b] = e
        s.starts = sorted(s.ends)
        s.md = Cs(CS_ARCH_X86, CS_MODE_64); s.md.detail = True
        s.locs = None; s.fnloc = None
    def in_sec(s, r, n):
        va, sz = s.secs[n]; return va <= r < va + sz
    def owner(s, rva):
        i = bisect.bisect_right(s.starts, rva) - 1
        if i < 0: return None
        b = s.starts[i]
        return b if rva < s.ends[b] else None
    def try_loc(s, r):
        if not s.in_sec(r, ".rdata"): return None
        try: p, l, line, col = struct.unpack_from("<QQII", s.img, r)
        except Exception: return None
        p -= BASE
        if 0 < l < 400 and 0 < line < 100000 and 0 < col < 1000 and s.in_sec(p, ".rdata"):
            try: f = s.img[p:p + l].decode("utf-8")
            except Exception: return None
            if f.endswith(".rs"): return (f, line, col)
        return None
    def build_locidx(s):
        cp = CACHE % s.tag
        if os.path.exists(cp):
            s.locs, s.fnloc = pickle.load(open(cp, "rb")); return
        va, sz = s.secs[".text"]; buf = s.img[va:va + sz]
        # lea r64,[rip+disp32]: REX.W(48/4C) 8D modrm(05|0D|15|1D|25|2D|35|3D)
        s.locs = {}; s.fnloc = collections.defaultdict(list)
        for m in re.finditer(b"[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]", buf):
            i = m.start()
            if i + 7 > len(buf): break
            disp = struct.unpack_from("<i", buf, i + 3)[0]
            tgt = va + i + 7 + disp
            loc = s.locs.get(tgt)
            if loc is None:
                if tgt in s.locs: continue
                loc = s.try_loc(tgt); s.locs[tgt] = loc
            if loc is None: continue
            o = s.owner(va + i)
            if o is not None: s.fnloc[o].append(loc)
        s.fnloc = dict(s.fnloc)
        pickle.dump((s.locs, s.fnloc), open(cp, "wb"))
    def consts(s, start):
        end = s.ends.get(start, start + 16); cs = collections.Counter(); ncall = 0; nins = 0
        for ins in s.md.disasm(s.img[start:end], start):
            nins += 1
            if ins.mnemonic in ("call",): ncall += 1
            if ins.mnemonic.startswith("j") or ins.mnemonic == "call": continue   # 분기/호출 타깃은 주소라 exe 마다 다르다
            for op in ins.operands:
                if op.type == X86_OP_IMM:
                    v = abs(op.imm)
                    if v >= BASE: continue                                        # 절대 주소 즉치(vtable·문자열) 제외
                    if 16 <= v < (1 << 40) and (v & (v - 1)) and v % 4096: cs[v] += 1
                elif op.type == X86_OP_MEM and op.mem.disp and op.mem.base not in (0,):
                    v = abs(op.mem.disp)
                    if 16 <= v < (1 << 24) and (v & (v - 1)): cs[("d", v)] += 1
        return cs, ncall, nins

def sig_exact(L): return tuple(sorted(collections.Counter(L).items()))
def sig_fc(L): return tuple(sorted(collections.Counter((f, c) for f, l, c in L).items()))

def main():
    av = sys.argv[1:]
    O = Exe(OLD, "old"); N = Exe(NEW, "new")
    print(u"Location 인덱스…"); O.build_locidx(); N.build_locidx()
    print(u"  old fn %d(loc 있음 %d) · new fn %d(loc 있음 %d)" % (len(O.starts), len(O.fnloc), len(N.starts), len(N.fnloc)))
    FO = pickle.load(open(PKL["old"], "rb"))["idx"]; FN = pickle.load(open(PKL["new"], "rb"))["idx"]
    FO = {(int(k, 16) if isinstance(k, str) else k): v for k, v in FO.items()}; FN = {(int(k, 16) if isinstance(k, str) else k): v for k, v in FN.items()}
    by_skel_new = collections.defaultdict(list); by_head_new = collections.defaultdict(list)
    for k, v in FN.items(): by_skel_new[v["skel"]].append(k); by_head_new[v["head"]].append(k)
    CGo = pickle.load(open(CG["old"], "rb")); CGn = pickle.load(open(CG["new"], "rb"))
    # 0.6.0 역색인
    ex_new = collections.defaultdict(list); fc_new = collections.defaultdict(list)
    for f, L in N.fnloc.items(): ex_new[sig_exact(L)].append(f); fc_new[sig_fc(L)].append(f)
    fc_item_new = collections.defaultdict(list)
    for f, L in N.fnloc.items():
        for key in set((ff, c) for ff, l, c in L): fc_item_new[key].append(f)
    # 대상
    D = json.load(io.open(SPEC, encoding="utf-8"))["specs"]
    targets = {}
    for i, sp in enumerate(D):
        a = ((sp.get("exe") or {}).get("addr") or "").lower()
        if a: targets[int(a, 16)] = {"i": i, "name": sp["name"], "src": "spec"}
    if "--only" not in av or av[av.index("--only") + 1] != "264":
        h = io.open(MAP, encoding="utf-8").read()
        for n in json.loads(re.search(r'<script id="fndata" type="application/json">(.*?)</script>', h, re.S).group(1)):
            if n["a"].startswith("spec-") or n.get("L") == "ext" and n.get("ext", 1) > 1: continue
            r = int(n["a"], 16)
            if r not in targets: targets[r] = {"i": None, "name": n.get("n") or ("FUN_" + n["a"]), "src": "map" if n.get("L") != "ext" else "ext"}
    if "--max" in av: targets = dict(list(targets.items())[:int(av[av.index("--max") + 1])])
    print(u"대상 %d" % len(targets))
    # 수동 확정(mig060_diag 진단 · Location 부분 일치 + cos ≈1 · 09-16): 구 → 신
    OVERRIDE = {0xe4c5c0: (0xd3d210, u"수동: handler.rs Location 19→55 · J 0.35 · cos 0.999 · 42KB→77KB(AI 루트 update 대폭 확장)"),
                0xe900b0: (0xf3da50, u"수동: lib.rs Location 2→4(889/914 → 1273/1323 · +384) · cos 1.000 · 7.9KB→9.4KB"),
                0xdda220: (0xf10f70, u"수동: team_plan.rs Location 45→31 · J 0.29 · cos 0.998 · 24.7KB→30.9KB")}
    res = []
    for r, meta in sorted(targets.items()):
        row = dict(meta); row["old"] = "%x" % r
        if r not in O.ends: row["verdict"] = u"구 exe .pdata 시작 아님"; res.append(row); continue
        L = O.fnloc.get(r, [])
        cands = []; how = None
        if r in OVERRIDE: cands, how = [OVERRIDE[r][0]], OVERRIDE[r][1]
        if L and not cands:
            c = ex_new.get(sig_exact(L), [])
            if c: cands, how = c, u"Location 정확 일치"
            else:
                c = fc_new.get(sig_fc(L), [])
                if c:
                    # 줄 델타 상수 검사
                    good = []
                    for f2 in c:
                        L2 = N.fnloc[f2]
                        d = set()
                        for (f, l, col), (f_, l2, col_) in zip(sorted(L), sorted(L2)):
                            d.add(l2 - l)
                        if len(d) <= 2: good.append((f2, sorted(d)))
                    if good: cands, how = [g[0] for g in good], u"Location (파일,열) 일치 · 줄 델타 %s" % good[0][1]
        if not cands and r in FO:
            c = by_skel_new.get(FO[r]["skel"], [])
            if c: cands, how = c, u"skel 해시 일치"
            else:
                c = by_head_new.get(FO[r]["head"], [])
                if len(c) <= 3 and c: cands, how = c, u"head 해시 일치(후보 %d)" % len(c)
        if not cands and len(L) >= 3:
            # (파일,열) 다중집합 Jaccard 부분 일치 — 소스가 편집돼 Location 이 늘거나 줄어든 경우
            fcL = collections.Counter((f, c) for f, l, c in L)
            pool = collections.Counter()
            for key in fcL:
                for f2 in fc_item_new.get(key, []): pool[f2] += 1
            scored = []
            for f2, hit in pool.most_common(200):
                fc2 = collections.Counter((f, c) for f, l, c in N.fnloc[f2])
                inter = sum((fcL & fc2).values()); union = sum((fcL | fc2).values())
                j = inter / union if union else 0
                if j >= 0.5: scored.append((j, f2))
            scored.sort(reverse=True)
            if scored and (len(scored) == 1 or scored[0][0] - scored[1][0] >= 0.12):
                cands, how = [scored[0][1]], u"Location (파일,열) 부분 일치 J=%.2f(%d/%d)" % (scored[0][0], sum((fcL & collections.Counter((f, c) for f, l, c in N.fnloc[scored[0][1]])).values()), sum(fcL.values()))
        if not cands:
            row["verdict"] = u"미발견"; row["how"] = u"Location %d · skel/head/Jaccard 미일치" % len(L); res.append(row); continue
        if len(cands) > 1:
            # 크기 가장 가까운 것
            osz = O.ends[r] - r
            cands.sort(key=lambda f2: abs((N.ends.get(f2, f2) - f2) - osz))
            how += u" · 후보 %d 중 크기 최근접" % len(cands)
        f2 = cands[0]
        row["new"] = "%x" % f2; row["how"] = how
        row["conf"] = "C(수동)" if how.startswith(u"수동") else "A" if how.startswith(u"Location 정확") else ("B" if how.startswith(u"Location (파일") and u"후보" not in how else ("C" if u"skel" in how or u"후보" in how else "D"))
        osz = O.ends[r] - r; nsz = N.ends.get(f2, f2) - f2
        row["size"] = [osz, nsz]
        fo = FO.get(r, {}); fn = FN.get(f2, {})
        same_skel = fo.get("skel") and fo.get("skel") == fn.get("skel")
        # mnem cos
        mo = fo.get("mnem", {}); mn = fn.get("mnem", {})
        keys = set(mo) | set(mn); dot = sum(mo.get(k, 0) * mn.get(k, 0) for k in keys)
        cos = dot / (math.sqrt(sum(v * v for v in mo.values())) * math.sqrt(sum(v * v for v in mn.values())) + 1e-9) if mo and mn else 0
        co, nco, nio = O.consts(r); cn, ncn, nin = N.consts(f2)
        imm_o = set(k for k in co if not isinstance(k, tuple)); imm_n = set(k for k in cn if not isinstance(k, tuple))
        dsp_o = set(k[1] for k in co if isinstance(k, tuple)); dsp_n = set(k[1] for k in cn if isinstance(k, tuple))
        row["ninsn"] = [nio, nin]; row["ncall"] = [nco, ncn]; row["mnem_cos"] = round(cos, 4)
        row["imm_only_old"] = sorted(imm_o - imm_n)[:20]; row["imm_only_new"] = sorted(imm_n - imm_o)[:20]
        row["disp_only_old"] = sorted(dsp_o - dsp_n)[:20]; row["disp_only_new"] = sorted(dsp_n - dsp_o)[:20]
        row["callers"] = [len(CGo["caller"].get(r, [])), len(CGn["caller"].get(f2, []))]
        row["callees"] = [len(CGo["callee"].get(r, [])), len(CGn["callee"].get(f2, []))]
        # 즉치/변위 차이가 한 상수 델타(예: PlayerState +0xd0)로 전부 설명되나
        def delta_explains(old, new):
            ro, rn = sorted(old - new), sorted(new - old)
            if not ro or len(ro) != len(rn): return None
            ds = set(b - a for a, b in zip(ro, rn))
            return ds.pop() if len(ds) == 1 else None
        d_imm = delta_explains(imm_o, imm_n); d_dsp = delta_explains(dsp_o, dsp_n)
        row["delta"] = {"imm": d_imm, "disp": d_dsp}
        # 등급
        if same_skel: v = u"동일(skel)"
        elif abs(nsz - osz) <= max(16, osz * 0.02) and imm_o == imm_n and cos > 0.995: v = u"이동만"
        elif abs(nsz - osz) <= osz * 0.08 and not (imm_o - imm_n) and cos > 0.98: v = u"소폭(+%dB · 즉치 ⊆)" % (nsz - osz)
        elif abs(nsz - osz) <= osz * 0.05 and cos > 0.98 and ((imm_o == imm_n or d_imm is not None) and (dsp_o == dsp_n or d_dsp is not None)):
            v = u"오프셋 이동(Δimm %s · Δdisp %s)" % (hex(d_imm) if d_imm else "0", hex(d_dsp) if d_dsp else "0")
        elif dsp_o != dsp_n and imm_o == imm_n and cos > 0.97: v = u"오프셋만 변경(구조체 이동 · 즉치 동일)"
        else: v = u"변경(크기 %+dB · 즉치 −%d/+%d · cos %.3f)" % (nsz - osz, len(imm_o - imm_n), len(imm_n - imm_o), cos)
        row["verdict"] = v; res.append(row)
        print(u"%s → %s  %-28s %-40s %s" % (row["old"], row["new"], v[:28], meta["name"][:40], how[:50]))
    # ── 2차 패스: 호출자 그래프 — 구 호출자 C(이미 짝 지어진) → 신 C' 의 콜리 중 크기·니모닉·Location 유사도 최고
    mapped = {int(r["old"], 16): int(r["new"], 16) for r in res if "new" in r}
    used = set(mapped.values())
    def score_pair(r, f2):
        osz = O.ends[r] - r; nsz = N.ends.get(f2, f2) - f2
        fo = FO.get(r, {}); fn = FN.get(f2, {}); mo = fo.get("mnem", {}); mn = fn.get("mnem", {})
        keys = set(mo) | set(mn); dot = sum(mo.get(k, 0) * mn.get(k, 0) for k in keys)
        cos = dot / (math.sqrt(sum(v * v for v in mo.values())) * math.sqrt(sum(v * v for v in mn.values())) + 1e-9) if mo and mn else 0
        fcL = collections.Counter((f, c) for f, l, c in O.fnloc.get(r, [])); fc2 = collections.Counter((f, c) for f, l, c in N.fnloc.get(f2, []))
        j = sum((fcL & fc2).values()) / max(1, sum((fcL | fc2).values())) if (fcL or fc2) else 0.5
        sz = min(osz, nsz) / max(osz, nsz, 1)
        return cos * 0.5 + j * 0.3 + sz * 0.2, cos, j, sz
    for row in res:
        if "new" in row or row.get("verdict") != u"미발견": continue
        r = int(row["old"], 16)
        pool = collections.Counter()
        for C in CGo["caller"].get(r, []):
            C2 = mapped.get(C)
            if C2 is None: continue
            for f2 in CGn["callee"].get(C2, []):
                if f2 in used or f2 not in N.ends: continue
                pool[f2] += 1
        if not pool: continue
        scored = sorted(((score_pair(r, f2), f2, hit) for f2, hit in pool.items()), reverse=True)
        (sc, cos, j, sz), f2, hit = scored[0]
        if cos < 0.9 or sz < 0.4: continue
        if len(scored) > 1 and sc - scored[1][0][0] < 0.05: continue
        row["new"] = "%x" % f2; row["how"] = u"호출자 그래프(공유 호출자 %d · cos %.3f · J %.2f · 크기비 %.2f)" % (hit, cos, j, sz); row["conf"] = "C"
        osz = O.ends[r] - r; nsz = N.ends[f2] - f2; row["size"] = [osz, nsz]
        co, nco, nio = O.consts(r); cn, ncn, nin = N.consts(f2)
        imm_o = set(k for k in co if not isinstance(k, tuple)); imm_n = set(k for k in cn if not isinstance(k, tuple))
        row["ninsn"] = [nio, nin]; row["ncall"] = [nco, ncn]; row["mnem_cos"] = round(cos, 4)
        row["imm_only_old"] = sorted(imm_o - imm_n)[:20]; row["imm_only_new"] = sorted(imm_n - imm_o)[:20]; row["disp_only_old"] = []; row["disp_only_new"] = []
        row["callers"] = [len(CGo["caller"].get(r, [])), len(CGn["caller"].get(f2, []))]; row["callees"] = [len(CGo["callee"].get(r, [])), len(CGn["callee"].get(f2, []))]
        row["verdict"] = u"변경(크기 %+dB · 즉치 −%d/+%d · cos %.3f · 호출자 경유)" % (nsz - osz, len(imm_o - imm_n), len(imm_n - imm_o), cos)
        used.add(f2); mapped[r] = f2
    os.makedirs(os.path.join(HERE, "_next"), exist_ok=True)
    json.dump(res, io.open(os.path.join(HERE, "_next", "mig060_judge.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    C = collections.Counter(r["verdict"].split("(")[0] for r in res)
    L = [u"# 0.6.0 변화 판정 — 분석 함수 %d (exe↔exe · 패닉 Location 집합 + fnindex skel + 즉치/변위/니모닉 · 2026-09-16)" % len(res), u"",
         u"> " + u" · ".join(u"%s %d" % kv for kv in C.most_common()), u"",
         u"| 구 RVA | 신 RVA | 함수 | 출처 | 신뢰 | 판정 | 크기 | 명령 | call | 즉치 −/+ | 변위 −/+ | 호출자 | 즉치 차(구→신) | 근거 |", u"|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"]
    for r in res:
        if "new" in r:
            L.append(u"| `%s` | `%s` | %s | %s | %s | %s | %d→%d | %d→%d | %d→%d | %d/%d | %d/%d | %d→%d | %s | %s |" % (
                r["old"], r["new"], r["name"][:44], r["src"], r["conf"], r["verdict"], r["size"][0], r["size"][1], r["ninsn"][0], r["ninsn"][1], r["ncall"][0], r["ncall"][1],
                len(r["imm_only_old"]), len(r["imm_only_new"]), len(r["disp_only_old"]), len(r["disp_only_new"]), r["callers"][0], r["callers"][1], (u" ".join(hex(x) for x in r["imm_only_old"][:4]) + u" → " + u" ".join(hex(x) for x in r["imm_only_new"][:4])) if (r["imm_only_old"] or r["imm_only_new"]) else u"", r["how"]))
        else:
            L.append(u"| `%s` | — | %s | %s | — | %s | | | | | | | | %s |" % (r["old"], r["name"][:44], r["src"], r["verdict"], r.get("how", "")))
    io.open(os.path.join(HERE, "_next", "mig060_judge.md"), "w", encoding="utf-8").write(u"\n".join(L))
    print(u"\n".join(u"%s %d" % kv for kv in C.most_common()))

if __name__ == "__main__":
    main()
