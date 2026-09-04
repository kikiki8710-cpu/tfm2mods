# -*- coding: utf-8 -*-
"""midpin.py — 함수 **내부**(mid-function) 바이트패치 사이트를 재핀한다.

왜 새 도구인가 (2026-09-02):
    `repin.py` 의 3단(바이트 12B 유일검색 → skeleton → owner+콜그래프)이
    125건에서 "동형 클론에 막힘"으로 실패했다. 그 3단은 전부 **사이트 주변만** 본다.
    여기서는 축을 바꾼다:

        ① 사이트의 **owner 함수**를 구 exe 에서 찾고
        ② owner 를 신 exe 로 매칭(skel → head → 콜그래프 투영)한 뒤
        ③ **함수 내부를 명령 단위로 정렬**해 사이트의 "몇 번째 명령인가"로 대응시킨다.

    ③이 핵심이다. 동형 클론이 아무리 많아도 **owner 가 확정되면 그 안에서는 유일**하다.
    정렬은 `posdiff.py` 와 같은 발상 — 니모닉·오퍼랜드 형태로 앵커를 잡고,
    구/신 명령열을 LCS 로 맞춘 뒤 사이트가 걸린 명령의 대응을 읽는다.

판정
    EXACT     대응 명령의 니모닉·길이·즉시값 폭이 전부 같음 → 안전
    SHIFTED   대응은 찾았으나 명령 길이가 달라 사이트 내 오프셋 보정 필요 → 값 제시 + 검증 필요
    OWNER_NG  owner 매칭 실패 → ghidra-re 필요
    NO_ALIGN  owner 는 맞췄으나 정렬에서 그 명령이 사라짐(본문 재구조화) → ghidra-re 필요

사용
  python MIG\midpin.py survey                 # 125건의 owner 매칭 가능성만 집계
  python MIG\midpin.py plan  [--out plan.json]# 재핀 후보 산출
"""
import argparse, difflib, json, os, pickle, struct, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")

from capstone import Cs, CS_ARCH_X86, CS_MODE_64

MODS = os.path.join("C:", os.sep, "tfm2mods")
ANA = os.path.join("C:", os.sep, "Users", "jungs", "Desktop", "claude", "tfm2")
LIVE = os.path.join("C:", os.sep, "Program Files (x86)", "Steam", "steamapps",
                    "common", "Teamfight Manager2", "TeamfightManager2.exe")
MAN = os.path.join(MODS, "MIG", "manifest", "tfm2_ai_adjust.json")

md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = False


class Img:
    def __init__(s, exe, pkl, cg=None):
        d = open(exe, "rb").read()
        s.raw = d
        pe = struct.unpack_from("<I", d, 0x3C)[0]
        n = struct.unpack_from("<H", d, pe + 6)[0]
        st = pe + 24 + struct.unpack_from("<H", d, pe + 20)[0]
        s.secs = []
        for i in range(n):
            o = st + i * 40
            nm = d[o:o + 8].rstrip(b"\0").decode(errors="replace")
            vsz, va, rsz, rr = struct.unpack_from("<IIII", d, o + 8)
            s.secs.append((nm, va, max(vsz, rsz), rr, rsz))
        P = pickle.load(open(pkl, "rb"))
        s.fn = {(int(k, 16) if isinstance(k, str) else k):
                dict(v, size=int(v["size"]))
                for k, v in P["idx"].items()}   # size 가 문자열로 저장돼 있다
        s.by_skel = P["by_skel"]
        s.by_head = P.get("by_head", {})
        s.starts = sorted(s.fn)
        s.cg = pickle.load(open(cg, "rb")) if cg else None
        s.cal = norm_map(s.cg["callee"]) if s.cg else {}
        s.calr = norm_map(s.cg["caller"]) if s.cg else {}

    def roff(s, rva):
        for nm, va, vsz, rr, rsz in s.secs:
            if va <= rva < va + vsz:
                o = rva - va
                return rr + o if o < rsz else None
        return None

    def read(s, rva, n):
        o = s.roff(rva)
        return b"" if o is None else s.raw[o:o + n]

    def owner(s, rva):
        import bisect as _b
        i = _b.bisect_right(s.starts, rva) - 1
        if i < 0:
            return None
        st = s.starts[i]
        f = s.fn[st]
        return st if rva < st + f["size"] else None

    def insns(s, start):
        """함수 전체를 (rva, mnemonic, op_form, size) 로."""
        f = s.fn.get(start)
        if not f:
            return []
        code = s.read(start, f["size"])
        out = []
        for i in md.disasm(code, start):
            # 오퍼랜드는 숫자를 지워 형태만 남긴다(즉시값 변화 내성)
            form = "".join(c for c in i.op_str if not c.isdigit()).replace("0x", "")
            out.append((i.address, i.mnemonic, form, i.size))
        return out


def norm_map(d):
    return {(int(k, 16) if isinstance(k, str) else k):
            [(int(x, 16) if isinstance(x, str) else x) for x in v] for k, v in d.items()}


def match_owner(o, n, start, gmap):
    """구 exe 함수 start 를 신 exe 에서 찾는다."""
    f = o.fn.get(start)
    if not f:
        return None, "NOT_FN"
    if start in gmap:
        return gmap[start], "GMAP"
    c = [int(x, 16) if isinstance(x, str) else x for x in n.by_skel.get(f["skel"], [])]
    if len(c) == 1:
        return c[0], "SKEL"
    if len(c) > 1:
        same = [x for x in c if n.fn[x]["size"] == f["size"]]
        if len(same) == 1:
            return same[0], "SKEL_SIZE"
    h = [int(x, 16) if isinstance(x, str) else x for x in n.by_head.get(f.get("head"), [])]
    if len(h) == 1:
        return h[0], "HEAD"
    if len(h) > 1:
        same = [x for x in h if abs(n.fn[x]["size"] - f["size"]) < max(64, f["size"] // 8)]
        if len(same) == 1:
            return same[0], "HEAD_SIZE"
    # 콜그래프 투영: caller 집합을 사영해 교집합
    if o.cg and n.cg:
        oc = norm_map(o.cg["caller"]).get(start, [])
        proj = [gmap[c] for c in oc if c in gmap]
        if proj:
            ne = norm_map(n.cg["callee"])
            cand = {}
            for c in proj:
                for t in ne.get(c, []):
                    cand[t] = cand.get(t, 0) + 1
            best = [t for t, k in cand.items() if k == len(proj)
                    and t in n.fn and abs(n.fn[t]["size"] - f["size"]) < max(96, f["size"] // 6)]
            if len(best) == 1:
                return best[0], "CG"
    return None, "MULTI"


def match_owner_callee(o, n, start, gmap, ncal, ncaller_of):
    """★callee 집합 채점 — owner 의 호출 대상들을 신 exe 로 투영해 가장 많이 겹치는 함수를 고른다.

    `RVA_RETREAT`(2026-09-02)를 이 방법으로 풀었다: 후보 3개 중 정답만 callee 13/13 이 겹쳤고
    경쟁 후보는 4/13 이었다. skel/head/caller 가 전부 클론에 막혀도 **호출 대상의 집합**은
    함수의 신원을 강하게 특정한다.

    반환 (신 rva, 근거문자열) 또는 (None, 사유)
    """
    f = o.fn.get(start)
    if not f:
        return None, "NOT_FN"
    oc = o.cal.get(start, [])
    proj = set(gmap[c] for c in oc if c in gmap)
    if len(proj) < 4:
        return None, "CALLEE_FEW(%d)" % len(proj)
    # 후보 풀 = 투영된 callee 중 하나라도 부르는 함수
    pool = {}
    for t in proj:
        for c in ncaller_of.get(t, []):
            pool[c] = pool.get(c, 0) + 1
    if not pool:
        return None, "NO_POOL"
    sz = f["size"]
    scored = []
    for c, hit in pool.items():
        g = n.fn.get(c)
        if not g:
            continue
        tot = len(ncal.get(c, []))
        # 겹침 비율(재현율) + 크기 근접
        rec = hit / len(proj)
        szr = min(g["size"], sz) / max(g["size"], sz, 1)
        scored.append((rec, szr, hit, c, g["size"]))
    if not scored:
        return None, "NO_CAND"
    scored.sort(key=lambda x: (-x[0], -x[1]))
    top = scored[0]
    second = scored[1] if len(scored) > 1 else (0, 0, 0, 0, 0)
    # 확정 조건: 재현율 0.6+ , 2위와 뚜렷한 차이, 크기 30% 이내
    if top[0] >= 0.6 and top[1] >= 0.7 and (top[0] - second[0] >= 0.2 or top[2] - second[2] >= 3):
        return top[3], "CALLEE %d/%d (2위 %d) size %d->%d" % (
            top[2], len(proj), second[2], sz, top[4])
    return None, "CALLEE_AMBIG top=%d/%d 2nd=%d/%d" % (top[2], len(proj), second[2], len(proj))


def insn_ratio(o, n, ostart, nstart):
    """두 함수의 명령열 유사도(0~1). 동형 클론 쌍을 가르는 최종 판별자."""
    oi, ni = o.insns(ostart), n.insns(nstart)
    if not oi or not ni:
        return 0.0
    a = ["%s|%s" % (m, fm) for _, m, fm, _ in oi]
    b = ["%s|%s" % (m, fm) for _, m, fm, _ in ni]
    return difflib.SequenceMatcher(a=a, b=b, autojunk=False).ratio()


def match_owner_align(o, n, start, gmap, ncal, ncaller_of, topk=6):
    """★callee 채점이 동점(클론 쌍)일 때 — 후보들을 **함수 전체 명령열 정렬 비율**로 가른다.

    callee 집합이 같아도 본문 명령 순서까지 같을 확률은 낮다.
    상위 후보 topk 개에 대해 ratio 를 재고, 1위가 2위보다 뚜렷하면 확정.
    """
    f = o.fn.get(start)
    if not f:
        return None, "NOT_FN"
    oc = o.cal.get(start, [])
    proj = set(gmap[c] for c in oc if c in gmap)
    pool = {}
    for t in proj:
        for c in ncaller_of.get(t, []):
            pool[c] = pool.get(c, 0) + 1
    # ★callee 가 빈약한 작은 함수: **caller 투영**으로 풀을 만든다.
    #   구 caller 가 gmap 에 있으면, 신 caller 가 부르는 대상들이 곧 후보다.
    if len(pool) < 2:
        for oc2 in o.calr.get(start, []):
            nc = gmap.get(oc2)
            if nc is None:
                continue
            for t in ncal.get(nc, []):
                pool[t] = pool.get(t, 0) + 1
    if len(pool) < 2:
        return None, "POOL_SMALL"
    sz = f["size"]
    cand = sorted(pool.items(), key=lambda kv: -kv[1])[:40]
    scored = []
    for c, hit in cand:
        g = n.fn.get(c)
        if not g or max(g["size"], sz) / max(min(g["size"], sz), 1) > 1.6:
            continue
        scored.append((hit, c))
    if not scored:
        return None, "NO_CAND"
    scored.sort(key=lambda x: -x[0])
    top = [c for _, c in scored[:topk]]
    rat = sorted(((insn_ratio(o, n, start, c), c) for c in top), reverse=True)
    if not rat:
        return None, "NO_RATIO"
    r1, c1 = rat[0]
    r2 = rat[1][0] if len(rat) > 1 else 0.0
    if r1 >= 0.55 and (r1 - r2) >= 0.08:
        return c1, "ALIGN %.2f (2위 %.2f) size %d->%d" % (r1, r2, sz, n.fn[c1]["size"])
    return None, "ALIGN_AMBIG %.2f/%.2f" % (r1, r2)


def align_site(o, n, ostart, nstart, site):
    """owner 내부에서 site 가 걸린 명령을 신 exe 로 대응시킨다."""
    oi, ni = o.insns(ostart), n.insns(nstart)
    if not oi or not ni:
        return None, "NO_INSN", None
    # site 가 걸린 명령 index
    k = None
    for idx, (a, m, fm, sz) in enumerate(oi):
        if a <= site < a + sz:
            k = idx
            break
    if k is None:
        return None, "SITE_OUT", None
    okey = ["%s|%s" % (m, fm) for _, m, fm, _ in oi]
    nkey = ["%s|%s" % (m, fm) for _, m, fm, _ in ni]
    sm = difflib.SequenceMatcher(a=okey, b=nkey, autojunk=False)
    for a0, b0, ln in sm.get_matching_blocks():
        if a0 <= k < a0 + ln:
            j = b0 + (k - a0)
            delta = site - oi[k][0]
            new = ni[j][0] + delta
            ok = (oi[k][1] == ni[j][1] and oi[k][3] == ni[j][3])
            return new, ("EXACT" if ok else "SHIFTED"), (oi[k], ni[j])
    return None, "NO_ALIGN", None


def load_unres():
    j = json.load(open(MAN, encoding="utf-8"))
    ents = j.get("entries", j)
    items = list(ents.items()) if isinstance(ents, dict) else [(e.get("name"), e) for e in ents]
    out = []
    for k, v in items:
        if isinstance(v, dict) and v.get("unresolved") and v.get("sect") == ".text":
            val = v.get("value")
            rva = int(val, 16) if isinstance(val, str) else val
            out.append((v.get("name"), rva, v))
    return out


def build(o7, n8):
    gmap = {}
    for skel, l7 in o7.by_skel.items():
        a = [int(x, 16) if isinstance(x, str) else x for x in l7]
        b = [int(x, 16) if isinstance(x, str) else x for x in n8.by_skel.get(skel, [])]
        if len(a) == 1 and len(b) == 1:
            gmap[a[0]] = b[0]
    return gmap


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("cmd", choices=["survey", "plan"])
    ap.add_argument("--out", default=os.path.join(MODS, "MIG", "midpin_plan.json"))
    a = ap.parse_args()

    o7 = Img(os.path.join(ANA, "tfm2_0.5.7", "TeamfightManager2.exe"),
             os.path.join(MODS, "_fnidx_057.pkl"), os.path.join(MODS, "_cg_057.pkl"))
    n8 = Img(LIVE, os.path.join(MODS, "_fnidx_058.pkl"), os.path.join(MODS, "_cg_058.pkl"))
    gmap = build(o7, n8)
    print("전역 UNIQUE 대응 = %d" % len(gmap))
    sites = load_unres()
    print(".text unresolved = %d건" % len(sites))
    print()

    import collections
    # ★owner 를 먼저 반복 해결한다 — 하나 풀리면 gmap 이 커져 다음 것이 풀린다(연쇄).
    owners = sorted({o7.owner(r) for _, r, _ in sites} - {None})
    solved = {}
    for rnd in range(8):
        got = 0
        for ost in owners:
            if ost in solved:
                continue
            nst, how = match_owner(o7, n8, ost, gmap)
            if nst is None:
                nst, how = match_owner_callee(o7, n8, ost, gmap, n8.cal, n8.calr)
                how = "C:" + how if nst is not None else how
            if nst is None:
                nst, how = match_owner_align(o7, n8, ost, gmap, n8.cal, n8.calr)
                how = "A:" + how if nst is not None else how
            if nst is not None:
                solved[ost] = (nst, how)
                gmap[ost] = nst
                got += 1
        if got == 0:
            break
    print("owner 해결 %d/%d (반복 %d라운드)" % (len(solved), len(owners), rnd + 1))
    print()

    stat = collections.Counter()
    rows = []
    for name, rva, v in sites:
        ost = o7.owner(rva)
        if ost is None:
            stat["OWNER_NG(구exe 함수밖)"] += 1
            rows.append((name, rva, None, None, "OWNER_NG", None))
            continue
        nst, how = solved.get(ost, (None, "MULTI"))
        if nst is None:
            stat["OWNER_NG(%s)" % how] += 1
            rows.append((name, rva, ost, None, "OWNER_NG", how))
            continue
        new, st, pair = align_site(o7, n8, ost, nst, rva)
        stat[st] += 1
        rows.append((name, rva, ost, nst, st, (how, new, pair)))

    print("%-28s %s" % ("판정", "건수"))
    print("-" * 44)
    for k, c in stat.most_common():
        print("%-28s %d" % (k, c))

    if a.cmd == "plan":
        out = []
        for name, rva, ost, nst, st, extra in rows:
            if st in ("EXACT", "SHIFTED") and extra:
                how, new, pair = extra
                out.append({"name": name, "old": "0x%x" % rva, "new": "0x%x" % new,
                            "verdict": st, "owner_old": "0x%x" % ost,
                            "owner_new": "0x%x" % nst, "owner_how": how,
                            "old_insn": "%s %s" % (pair[0][1], pair[0][2]),
                            "new_insn": "%s %s" % (pair[1][1], pair[1][2])})
        json.dump(out, open(a.out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print()
        print("재핀 후보 %d건 -> %s" % (len(out), a.out))


if __name__ == "__main__":
    main()
