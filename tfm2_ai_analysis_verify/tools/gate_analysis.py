"""미실행 지점의 '막는 분기' 자동 분석.

실행된 함수 안에 있는데 실행되지 않은 지점(미도달)마다, 그 지점을 지배(dominate)하는 조건 분기 중
실측에서 한쪽으로만 간 비교(트레이서 '한쪽만')를 찾아 "이 조건이 한 번도 성립하지 않아서 안 불렸다"를 낸다.

방법: 함수 본문 선형 디스어셈블 → 기본 블록 CFG → 지배자 트리(Cooper-Harvey-Kennedy).
       트레이서 비교 지점 직후의 첫 조건 점프(jcc)를 그 비교의 분기로 보고, 실측 조건(참/거짓 횟수)으로
       '안 간 쪽' 간선을 정한다. 안 간 쪽 후속 블록이 미도달 지점 블록을 지배하면 그 비교가 막은 것.
사용: python -B -X utf8 tools/gate_analysis.py <sites.json> <site_catalog.json> <출력 폴더>
"""
import bisect, collections, csv, json, os, sys
import pefile, capstone

EXE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
ATLAS = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_ai_adjust\function_atlas_064_20261018"
CTRL_END = {"ret", "jmp", "ud2", "int3", "hlt"}


def jcc(i):
    return i.mnemonic.startswith("j") and i.mnemonic != "jmp"


def target(i):
    try:
        op = i.operands[0]
        return op.imm if op.type == capstone.x86.X86_OP_IMM else None
    except Exception:
        return None


def build_cfg(insns, start, end):
    leaders = {start}
    for k, i in enumerate(insns):
        if jcc(i) or i.mnemonic == "jmp":
            t = target(i)
            if t is not None and start <= t <= end:
                leaders.add(t)
            if k + 1 < len(insns):
                leaders.add(insns[k + 1].address)
        elif i.mnemonic in CTRL_END and k + 1 < len(insns):
            leaders.add(insns[k + 1].address)
    addrs = [i.address for i in insns]
    leaders = sorted(x for x in leaders if x in set(addrs))
    blocks = {}  # start -> (insn list)
    idx = {a: k for k, a in enumerate(addrs)}
    for n, b in enumerate(leaders):
        e = leaders[n + 1] if n + 1 < len(leaders) else end + 1
        blocks[b] = [i for i in insns[idx[b]:] if i.address < e]
    succ = collections.defaultdict(list)
    for b, ins in blocks.items():
        last = ins[-1]
        nxt = last.address + last.size
        if jcc(last):
            t = target(last)
            if t in blocks:
                succ[b].append(("taken", t))
            if nxt in blocks:
                succ[b].append(("fall", nxt))
        elif last.mnemonic == "jmp":
            t = target(last)
            if t in blocks:
                succ[b].append(("jmp", t))
        elif last.mnemonic not in CTRL_END and nxt in blocks:
            succ[b].append(("fall", nxt))
    return leaders, blocks, succ


def dominators(entry, nodes, succ):
    order, seen = [], set()
    st = [(entry, iter([t for _, t in succ.get(entry, [])]))]
    seen.add(entry)
    while st:
        n, it = st[-1]
        nx = next(it, None)
        if nx is None:
            order.append(n); st.pop()
        elif nx not in seen:
            seen.add(nx); st.append((nx, iter([t for _, t in succ.get(nx, [])])))
    rpo = order[::-1]
    pos = {n: k for k, n in enumerate(rpo)}
    preds = collections.defaultdict(list)
    for a in rpo:
        for _, b in succ.get(a, []):
            if b in pos:
                preds[b].append(a)
    idom = {entry: entry}
    changed = True
    while changed:
        changed = False
        for n in rpo[1:]:
            ps = [p for p in preds[n] if p in idom]
            if not ps:
                continue
            new = ps[0]
            for p in ps[1:]:
                a, b = p, new
                while a != b:
                    while pos[a] > pos[b]:
                        a = idom[a]
                    while pos[b] > pos[a]:
                        b = idom[b]
                new = a
            if idom.get(n) != new:
                idom[n] = new; changed = True
    return idom, preds


def dominated_by(idom, d, n):
    while True:
        if n == d:
            return True
        p = idom.get(n)
        if p is None or p == n:
            return False
        n = p


def main():
    sites_p, cat_p, out = sys.argv[1], sys.argv[2], sys.argv[3]
    os.makedirs(out, exist_ok=True)
    S = json.load(open(sites_p, encoding="utf-8"))["sites"]
    cat = {s["id"]: s for s in json.load(open(cat_p, encoding="utf-8"))["sites"]}
    funcs = {}
    for r in csv.DictReader(open(os.path.join(ATLAS, "functions.csv"), encoding="utf-8-sig")):
        funcs[r["RVA(0.6.4)"]] = (int(r["RVA(0.6.4)"], 16), int(r["끝RVA"], 16), r["의미ID"])
    # 오류 처리(panic·오류 종료) 호출 대상: 아틀라스 호출 표 분류
    panic_t = set()
    for e in csv.DictReader(open(os.path.join(ATLAS, "edges.csv"), encoding="utf-8-sig")):
        if e["분류"] in ("PANIC_BOUNDS_HELPER_DOCUMENTED", "ERROR_TERMINAL_PATTERN_CANDIDATE"):
            panic_t.add(e["피호출자RVA/심볼"])
    pe = pefile.PE(EXE, fast_load=True)
    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64); md.detail = True; md.skipdata = True
    byf = collections.defaultdict(list)
    for s in S:
        byf[s["function"]].append(s)
    results, summary = [], collections.Counter()
    gates = collections.defaultdict(lambda: {"blocked": [], "info": None})
    for f, ss in byf.items():
        un = [s for s in ss if s["stage"] == "미도달"]
        if not un or len(un) == len(ss) or f not in funcs:
            continue
        st, en, sem = funcs[f]
        insns = list(md.disasm(pe.get_data(st, en - st + 1), st))
        leaders, blocks, succ = build_cfg(insns, st, en)
        if st not in blocks:
            continue
        idom, preds = dominators(st, list(blocks), succ)
        blk_of = lambda a: leaders[bisect.bisect_right(leaders, a) - 1]
        exec_blocks = {blk_of(int(x["rva"], 16)) for x in ss if x["stage"] != "미도달"} | {st}
        # 한쪽만 비교 → 안 간 쪽 간선 후속 블록
        never = []  # (gate site row, dead block start)
        for s in ss:
            c = cat.get(s["id"])
            if not c or s["kind"] not in ("CMP_IMM", "CMP_DYN", "TEST") or s["match_hits"] + s["menu_hits"] == 0:
                continue
            if s["rel"] is None or s["cond_false"] is None:
                continue
            ct, cf = s["cond_true"], s["cond_false"]
            if ct and cf:
                continue
            b = blk_of(int(s["rva"], 16))
            last = blocks[b][-1]
            if not jcc(last) or last.address < int(s["rva"], 16):
                continue
            # 이 jcc 의 조건이 트레이서 소비 조건과 같은지(바로 그 소비 명령인지) 확인
            if c.get("consumer") and not c["consumer"].startswith(last.mnemonic):
                continue
            kinds = dict(succ.get(b, []) and [(k, t) for k, t in succ[b]])
            dead = kinds.get("taken") if ct == 0 else kinds.get("fall")
            if dead is None or len([p for p in preds[dead] if p in idom]) != 1:
                continue  # 다른 길로도 오는 블록이면 이 분기 하나로 막혔다고 할 수 없다
            never.append((s, dead))
        def to_panic(b, depth=0):
            """이 블록이 조건 분기 없이 panic/오류 종료 호출로 끝나는가(최대 3블록 직진)."""
            while b in blocks and depth < 4:
                for i in blocks[b]:
                    if i.mnemonic == "call":
                        t = target(i)
                        if t is not None and hex(t) in panic_t:
                            return True
                kinds = [k for k, _ in succ.get(b, [])]
                if len(kinds) != 1:
                    return False
                b = succ[b][0][1]; depth += 1
            return False
        for u in un:
            ub = blk_of(int(u["rva"], 16))
            if ub in blocks and to_panic(ub):
                summary["오류 처리(panic) 경로"] += 1
                results.append({"site": u["rva"], "function": f, "gate": None, "why": "panic 경로"}); continue
            if ub not in idom:
                summary["CFG 밖(간접 점프 등)"] += 1
                results.append({"site": u["rva"], "function": f, "gate": None, "why": "CFG 밖"}); continue
            hit = [(g, d) for g, d in never if dominated_by(idom, d, ub)]
            if hit:
                # 가장 가까운(깊은) 막는 분기
                g, d = max(hit, key=lambda x: int(x[0]["rva"], 16))
                summary["한쪽만 비교가 막음"] += 1
                key = g["rva"]
                gates[key]["blocked"].append(u["rva"])
                gates[key]["info"] = {"gate_site": g["rva"], "function": f, "semantic": sem, "insn": g["insn"],
                                      "consumer": g["consumer"], "rel": g["rel"], "T": g["T"], "signed": g["signed"],
                                      "never": "참" if g["cond_true"] == 0 else "거짓",
                                      "observed": [g["lhs_min"], g["lhs_max"]], "hits": g["match_hits"] + g["menu_hits"]}
                results.append({"site": u["rva"], "function": f, "gate": key, "why": "한쪽만 비교"})
            else:
                # 지배자 사슬을 올라가며, 실행된 블록에서 이 지점 쪽으로 갈라지는 첫 조건 분기를 정적으로 찾는다
                cond = None
                n = ub
                while n in idom and idom[n] != n:
                    p_ = idom[n]
                    last = blocks[p_][-1]
                    if jcc(last) and p_ in exec_blocks:
                        prod = None
                        for i in reversed(blocks[p_][:-1]):
                            if i.mnemonic in ("cmp", "test", "sub", "and", "or", "xor", "dec", "inc", "bt", "add", "neg") or i.mnemonic.startswith(("comis", "ucomis")):
                                prod = i; break
                            if i.mnemonic == "call":
                                break
                        side = "taken" if any(k == "taken" and dominated_by(idom, t, ub) for k, t in succ.get(p_, [])) else "fall"
                        cond = {"branch": hex(last.address), "jcc": f"{last.mnemonic} {last.op_str}", "side": side,
                                "producer": f"{prod.mnemonic} {prod.op_str}" if prod else None,
                                "producer_at": hex(prod.address) if prod else None}
                        break
                    n = p_
                key = "u:" + (cond["branch"] if cond else "none")
                summary["추적 안 된 분기가 막음" if cond else "막는 분기 미상"] += 1
                if cond:
                    gates[key]["blocked"].append(u["rva"])
                    gates[key]["info"] = {"gate_site": cond["branch"], "function": f, "semantic": sem, "insn": cond["producer"] or "(직전 비교 없음)",
                                          "consumer": cond["jcc"], "rel": "미추적", "T": None, "signed": "x",
                                          "never": "taken" if cond["side"] == "taken" else "fall-through",
                                          "observed": [None, None], "hits": 0, "untraced": True}
                results.append({"site": u["rva"], "function": f, "gate": key if cond else None, "why": "추적 안 된 분기" if cond else "미상", "cond": cond})
    glist = sorted(({**v["info"], "blocked_sites": len(v["blocked"]), "blocked": v["blocked"]} for v in gates.values()),
                   key=lambda x: -x["blocked_sites"])
    json.dump({"summary": dict(summary), "gates": glist, "sites": results},
              open(os.path.join(out, "gates.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    L = ["# 미실행 분기의 '막는 비교' 자동 분석", "", f"- 대상 = 실행된 함수 안의 미도달 지점 · 결과 {dict(summary)}", "",
         "막는 비교 = 그 지점을 지배하는 조건 분기 중 실측에서 한쪽으로만 간 비교. '안 간 쪽' 조건이 성립해야 그 지점들이 실행된다.", "",
         "| 막는 비교 지점 | 함수 | 명령 → 분기 | 한 번도 안 나온 조건 | 관측 값 범위 | 적중 | 막힌 지점 수 |", "|---|---|---|---|---|---:|---:|"]
    for g in glist[:300]:
        cond = (f"분기 `{g['consumer']}` 의 {g['never']} 쪽(미추적 · 직전 `{g['insn']}`)" if g.get("untraced")
                else f"{g['rel']} {g['T'] if g['T'] is not None else '(동적)'} 이 {g['never']}")
        L.append(f"| `{g['gate_site']}` | `{g['function']}` {g['semantic'][:40]} | `{g['insn']}` → `{g['consumer']}` | {cond} | {g['observed'][0]} ~ {g['observed'][1]} | {g['hits']:,} | {g['blocked_sites']} |")
    open(os.path.join(out, "gates.md"), "w", encoding="utf-8").write("\n".join(L) + "\n")
    print(dict(summary), "gates", len(glist))


if __name__ == "__main__":
    main()
