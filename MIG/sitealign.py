#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""sitealign.py — repin.py plan 결과의 **함수 중간 사이트(MID_*)** 를 명령 단위 정렬로 교차검증한다.
왜 (2026-09-16 · 0.6.0):
    repin.match_mid 는 사이트 앞바이트(forward 16/12/10/8B)가 신 owner 안에서 유일할 때만
    "본문검색" 하고, 아니면 **동일오프셋 가정**(nown + off) 으로 값을 낸다.
    콜사이트(`e8 rel32`)·rip-rel 은 앞바이트 자체가 바뀌므로 대부분 가정으로 떨어지는데,
    0.6.0 은 함수 +15,147개(+11%) 전면 재컴파일이라 함수 본문 길이가 흔히 달라진다.
    ⟹ 가정값을 그대로 쓰면 엉뚱한 명령 한복판을 훅/패치한다(0.5.8 크래시 메커니즘과 동일).
방법 = midpin.align_site (구/신 owner 의 명령열을 니모닉|오퍼랜드형태 키로 LCS 정렬).
판정
    AGREE      정렬값 == repin 값                → 그대로
    DISAGREE   정렬값 != repin 값(EXACT/SHIFTED) → ★정렬값으로 교체 후보 (근거 2중이므로 정렬 우선)
    NO_ALIGN   owner 는 맞췄으나 그 명령이 사라짐  → ghidra-re
    FAIL_*     repin 이 실패한 엔트리에 owner 재매칭(midpin 3단) + 정렬 시도
사용
  python MIG\sitealign.py --map MIG/repin_map_060.json --old <구exe> --new <신exe>
        --oldpkl _fnidx_058.pkl --newpkl _fnidx_060.pkl --oldcg _cg_058.pkl --newcg _cg_060.pkl
        [--write MIG/repin_map_060.aligned.json]
"""
import sys, os, json, argparse
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import midpin as MP  # Img / align_site / match_owner*
import difflib, re

ROOT = r"C:\tfm2mods"
_REG = re.compile(r"\b(r[a-z0-9]+|e[a-z]{2}|[abcd][lhx]|[sd]il?|[sb]pl?|xmm\d+)\b")


def loose_key(m, form):
    """레지스터 재할당 내성: 레지스터 이름을 전부 R 로 지운 형태."""
    return "%s|%s" % (m, _REG.sub("R", form))


_INS = {}
_SM = {}


def _insns(I, start, tag):
    k = (tag, start)
    if k not in _INS:
        _INS[k] = I.insns(start)
    return _INS[k]


def _blocks(okeys, nkeys, ck):
    """SequenceMatcher 매칭블록 — (owner쌍, 키종류) 단위 캐시. 거대 함수(>6k명령)는 site 주변 창으로 자른다."""
    if ck not in _SM:
        _SM[ck] = difflib.SequenceMatcher(a=okeys, b=nkeys, autojunk=False).get_matching_blocks()
    return _SM[ck]


def align_site_loose(O, N, ostart, nstart, site, win=1500):
    """midpin.align_site 가 NO_ALIGN 일 때의 2차 정렬.
    ① 레지스터 지운 키로 LCS(블록 길이 ≥3)  ② 니모닉만으로 LCS
    ③ 그래도 없으면 site 앞뒤 매칭 블록 사이 '틈'의 길이가 구/신 같으면 index 사영(니모닉·길이 일치 조건).
    거대 함수는 site ±win 명령 창(신 함수는 비례 위치 ±win·1.3)만 정렬한다(O(n²) 회피).
    반환 (new, tag, (oi_k, ni_j)) / (None, 'NO_ALIGN', None)"""
    oi, ni = _insns(O, ostart, 'o'), _insns(N, nstart, 'n')
    if not oi or not ni:
        return None, "NO_INSN", None
    k = next((i for i, (a, m, fm, sz) in enumerate(oi) if a <= site < a + sz), None)
    if k is None:
        return None, "SITE_OUT", None
    delta = site - oi[k][0]
    # 창 절단
    if len(oi) > 2 * win or len(ni) > 2 * win:
        o0 = max(0, k - win); o1 = min(len(oi), k + win)
        c = int(k * len(ni) / max(1, len(oi)))
        n0 = max(0, c - int(win * 1.3)); n1 = min(len(ni), c + int(win * 1.3))
        wtag = "W"
    else:
        o0, o1, n0, n1, wtag = 0, len(oi), 0, len(ni), ""
    ois, nis = oi[o0:o1], ni[n0:n1]
    kk = k - o0
    for tag, keyf in (("LOOSE", lambda t: loose_key(t[1], t[2])), ("MNEM", lambda t: t[1])):
        ok_, nk_ = [keyf(t) for t in ois], [keyf(t) for t in nis]
        blocks = _blocks(ok_, nk_, (ostart, nstart, tag, o0, n0))
        for a0, b0, ln in blocks:
            if a0 <= kk < a0 + ln and ln >= 3:
                j = b0 + (kk - a0)
                return nis[j][0] + delta, wtag + tag, (ois[kk], nis[j])
        prev = max((b for b in blocks if b[0] + b[2] <= kk and b[2] >= 3), key=lambda b: b[0], default=None)
        nxt = min((b for b in blocks if b[0] > kk and b[2] >= 3), key=lambda b: b[0], default=None)
        if prev and nxt:
            og = (prev[0] + prev[2], nxt[0]); ng = (prev[1] + prev[2], nxt[1])
            if og[1] - og[0] == ng[1] - ng[0]:
                j = ng[0] + (kk - og[0])
                if ois[kk][1] == nis[j][1] and ois[kk][3] == nis[j][3]:
                    return nis[j][0] + delta, wtag + tag + "_GAP", (ois[kk], nis[j])
    return None, "NO_ALIGN", None


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--map", required=True)
    ap.add_argument("--old", required=True), ap.add_argument("--new", required=True)
    ap.add_argument("--oldpkl", required=True), ap.add_argument("--newpkl", required=True)
    ap.add_argument("--oldcg"), ap.add_argument("--newcg")
    ap.add_argument("--write", default=None)
    ap.add_argument("mods", nargs="*")
    a = ap.parse_args()

    O = MP.Img(a.old, a.oldpkl, a.oldcg)
    N = MP.Img(a.new, a.newpkl, a.newcg)
    gmap = MP.build(O, N)
    m = json.load(open(a.map, encoding="utf-8"))
    stats = {}
    changed = 0
    for mod, ents in m.items():
        if a.mods and mod not in a.mods:
            continue
        for old, e in ents.items():
            v = int(old, 16)
            kind = e.get("kind", "")
            new = e.get("new")
            own = O.owner(v)
            if own is None or (e.get("sect_old") or ".text") != ".text":
                continue                              # .rdata / owner 없음 = 이 도구 범위 밖
            if own == v:
                continue                              # 함수시작 = repin 의 fn 축 몫
            # ── 신 owner 결정: repin 이 준 owner 우선, 없으면 midpin 3단
            nown = None
            if new is not None and kind.startswith("MID_"):
                nown = N.owner(int(new, 16))
            how = "REPIN"
            if nown is None:
                nown, how = MP.match_owner(O, N, own, gmap)
                if nown is None:
                    nown, how = MP.match_owner_callee(O, N, own, gmap, N.cal, N.calr)
                    how = "C:" + how if nown is not None else how
                if nown is None:
                    nown, how = MP.match_owner_align(O, N, own, gmap, N.cal, N.calr)
                    how = "A:" + how if nown is not None else how
            if nown is None:
                verdict = "FAIL_OWNER"
                e["align"] = {"verdict": verdict, "how": how}
                stats[verdict] = stats.get(verdict, 0) + 1
                print("%-22s %-28s %-10s %s (%s)" % (mod, e["name"], old, verdict, how))
                continue
            big = O.fn[own]["size"] > 40000 or N.fn[nown]["size"] > 40000
            an, st, pair = (None, None, None) if big else MP.align_site(O, N, own, nown, v)
            if an is None:
                an, st, pair = align_site_loose(O, N, own, nown, v)
            if an is None:
                verdict = "NO_ALIGN" if new is not None else "FAIL_NO_ALIGN"
                e["align"] = {"verdict": verdict, "how": how, "nown": hex(nown), "st": st}
                stats[verdict] = stats.get(verdict, 0) + 1
                print("%-22s %-28s %-10s %s owner %s->%s (%s)" % (mod, e["name"], old, verdict, hex(own), hex(nown), how))
                continue
            oi, ni = pair
            desc = "%s %s(%dB) -> %s %s(%dB)" % (oi[1], oi[2][:18], oi[3], ni[1], ni[2][:18], ni[3])
            if new is None:
                verdict = "FAIL_ALIGNED_" + st
                e["align"] = {"verdict": verdict, "how": how, "new": hex(an), "insn": desc}
                e["new_aligned"] = hex(an)
                print("%-22s %-28s %-10s %s -> %s  [%s] %s" % (mod, e["name"], old, verdict, hex(an), how, desc))
            elif hex(an) == new:
                verdict = "AGREE"
                e["align"] = {"verdict": verdict, "st": st}
            else:
                verdict = "DISAGREE_" + st
                e["align"] = {"verdict": verdict, "repin": new, "new": hex(an), "insn": desc,
                              "repin_note": e.get("note", "")}
                e["new_aligned"] = hex(an)
                print("%-22s %-28s %-10s %s repin=%s align=%s  %s | %s" % (mod, e["name"], old, verdict, new, hex(an), desc, e.get("note", "")[:60]))
                changed += 1
            stats[verdict] = stats.get(verdict, 0) + 1
    print()
    print("집계:", dict(sorted(stats.items())))
    if a.write:
        json.dump(m, open(a.write, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print("->", a.write, "(new_aligned 필드 추가 · repin 값은 보존 — 교체는 apply 전 사람이 결정)")


if __name__ == "__main__":
    main()
