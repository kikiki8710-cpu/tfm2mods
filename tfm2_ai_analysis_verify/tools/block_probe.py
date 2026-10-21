"""M2 블록 입구 계측 지점 생성 (2026-10-21, v0.5.4).

관측 계획(observation_plan.json)에서 손잡이 지점이 미도달(deep = 더 아래에서 막힘, untr = 막는 분기 미계측)인 것마다,
함수 CFG 에서 그 지점 블록의 지배자 사슬을 위로 올라가 '마지막으로 실행이 확인된 블록'까지의 블록들 첫 명령에 지점을 단다.
다음 회차의 도달 수로 흐름이 끊기는 블록을 특정하고, 그 블록 직전 분기만 RE(M4)하면 된다.
출력 = gen_sites.py --extra 형식(줄마다 'RVA 메모').
사용: python -B -X utf8 tools/block_probe.py <observation_plan.json> <sites.json> <site_catalog.json> <출력 txt>
"""
import bisect, collections, csv, json, os, sys
import pefile, capstone
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from gate_analysis import EXE, ATLAS, build_cfg, dominators, jcc  # noqa: E402

ENTRY_GUARD = 16  # gen_sites.py 와 같음
BAD = {"jmp", "call", "ret", "retn", "int3", "int", "push", "pop", "nop", "ud2", "hlt", "leave", "enter"}
MAX_CHAIN = 12

plan_p, sites_p, cat_p, out_p = sys.argv[1:5]
plan = json.load(open(plan_p, encoding="utf-8"))
S = json.load(open(sites_p, encoding="utf-8"))["sites"]
have = {s["rva"] for s in json.load(open(cat_p, encoding="utf-8"))["sites"]}
executed = collections.defaultdict(set)
for s in S:
    if s["stage"] != "미도달":
        executed[s["function"]].add(int(s["rva"], 16))
funcs = {}
for r in csv.DictReader(open(os.path.join(ATLAS, "functions.csv"), encoding="utf-8-sig")):
    funcs[r["RVA(0.6.4)"]] = (int(r["RVA(0.6.4)"], 16), int(r["끝RVA"], 16))
pe = pefile.PE(EXE, fast_load=True)
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64); md.detail = True; md.skipdata = True

targets = collections.defaultdict(list)  # function -> [(knob rva, 후보명)]
for p in plan:
    for b in p.get("bad_sites", []):
        if b["state"] in ("deep", "untr"):
            targets[b["function"]].append((int(b["rva"], 16), p["name"]))

lines, stat = [], collections.Counter()
seen = set()
for f, ts in targets.items():
    if f not in funcs:
        stat["함수 경계 없음"] += 1; continue
    st, en = funcs[f]
    insns = list(md.disasm(pe.get_data(st, en - st + 1), st))
    leaders, blocks, succ = build_cfg(insns, st, en)
    if st not in blocks:
        continue
    idom, _ = dominators(st, list(blocks), succ)
    blk_of = lambda a: leaders[bisect.bisect_right(leaders, a) - 1]
    exec_blocks = {blk_of(a) for a in executed[f]} | {st}
    for knob, name in ts:
        chain, n = [], blk_of(knob)
        while n in idom and n not in exec_blocks and len(chain) < MAX_CHAIN:
            chain.append(n)
            if idom[n] == n:
                break
            n = idom[n]
        if not chain:
            stat["사슬 없음(이미 실행 블록)"] += 1; continue
        for b in reversed(chain):  # 위(실행 쪽)부터
            pick = next((i for i in blocks[b] if i.mnemonic not in BAD and not jcc(i) and not i.mnemonic.startswith("rep")
                         and i.address - st >= ENTRY_GUARD), None)
            if pick is None:
                stat["걸 명령 없음"] += 1; continue
            if pick.address in have or pick.address in seen:
                stat["기존/중복"] += 1; continue
            seen.add(pick.address)
            lines.append(f"{hex(pick.address)} m2_block {name} 손잡이 {hex(knob)} 사슬 {hex(b)}")
            stat["추가"] += 1
hdr = ["# v0.5.4 M2 블록 입구 계측(자동 생성: tools/block_probe.py). 손잡이 지점까지 지배자 사슬의 미실행 블록 첫 명령."]
open(out_p, "w", encoding="utf-8", newline="\n").write("\n".join(hdr + lines) + "\n")
print(dict(stat), "함수", len(targets))
