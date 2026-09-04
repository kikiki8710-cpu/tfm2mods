# -*- coding: utf-8 -*-
"""sitepin.py — 바이트패치 사이트를 **prefix + 원본 즉시값**으로 재핀한다.

왜 이 축인가 (2026-09-02):
    `repin.py` 3단(바이트 12B/skeleton/콜그래프)도, `midpin.py` 의 owner 정렬도
    동형 클론에 막혔다. 그런데 **소스와 orig_table 이 이미 강한 지문을 갖고 있다**:

        patch_imm_bytes(base + 0xd2e0e4, &[0x48,0x83,0xf8], 3, 1, ...)
                                          ^prefix 바이트    ^off ^width
        orig_table: (0xd2e0e4, 3, 1, 0x32)                        ^원본 즉시값

    prefix(3~4B) 하나로는 흔하지만 **prefix + 그 자리의 원본 즉시값**이 같이 맞는 곳은
    훨씬 드물다. owner 후보 안으로 검색 범위를 좁히면 사실상 유일해진다.

⚠ 왜 중요한가 — `orig_guard_ok` 는 **표에 없는 사이트를 그냥 통과시킨다**(orig_table.rs 주석).
   즉 미재핀 사이트는 무가드로 패치돼 엉뚱한 명령의 즉시값을 덮어쓴다. 0.5.8 크래시의 메커니즘.

사용
  python MIG\sitepin.py parse      # 소스에서 사이트 지문 추출 → 통계
  python MIG\sitepin.py pin        # 0.5.7 검증 + 0.5.8 재핀 후보 산출
  python MIG\sitepin.py pin --write-plan <json>
"""
import argparse, json, os, re, struct, sys, collections

sys.stdout.reconfigure(encoding="utf-8", errors="replace")

MODS = os.path.join("C:", os.sep, "tfm2mods")
ANA = os.path.join("C:", os.sep, "Users", "jungs", "Desktop", "claude", "tfm2")
LIVE = os.path.join("C:", os.sep, "Program Files (x86)", "Steam", "steamapps",
                    "common", "Teamfight Manager2", "TeamfightManager2.exe")
SRC = os.path.join(MODS, "tfm2_ai_adjust", "src")

# patch_imm_bytes(base + 0xRVA, &[0x..,0x..], IMM_OFF, WIDTH, ...)
RE_SITE = re.compile(
    r"patch_imm_bytes\(\s*base\s*\+\s*(0x[0-9a-fA-F_]+)\s*,\s*&\[([^\]]*)\]\s*,\s*(\d+)\s*,\s*(\d+)")
RE_ORIG = re.compile(r"\(\s*(0x[0-9a-fA-F]+)\s*,\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)")


def pe_sections(path):
    d = open(path, "rb").read()
    pe = struct.unpack_from("<I", d, 0x3C)[0]
    n = struct.unpack_from("<H", d, pe + 6)[0]
    st = pe + 24 + struct.unpack_from("<H", d, pe + 20)[0]
    secs = []
    for i in range(n):
        o = st + i * 40
        nm = d[o:o + 8].rstrip(b"\0").decode(errors="replace")
        vsz, va, rsz, rr = struct.unpack_from("<IIII", d, o + 8)
        secs.append((nm, va, vsz, rr, rsz))
    return d, secs


class Exe:
    def __init__(s, path):
        s.raw, s.secs = pe_sections(path)

    def off(s, rva):
        for nm, va, vsz, rr, rsz in s.secs:
            if va <= rva < va + min(vsz, rsz):
                return rr + (rva - va)
        return None

    def read(s, rva, n):
        o = s.off(rva)
        return b"" if o is None else s.raw[o:o + n]

    def text(s):
        for nm, va, vsz, rr, rsz in s.secs:
            if nm == ".text":
                return va, rr, rsz
        return None


def parse_sites():
    """소스 전체에서 patch_imm_bytes 사이트 지문을 뽑는다."""
    out = []
    for fn in sorted(os.listdir(SRC)):
        if not fn.endswith(".rs"):
            continue
        p = os.path.join(SRC, fn)
        for ln, line in enumerate(open(p, encoding="utf-8", errors="replace"), 1):
            for m in RE_SITE.finditer(line):
                rva = int(m.group(1).replace("_", ""), 16)
                pref = [int(x.strip(), 16) for x in m.group(2).split(",") if x.strip()]
                out.append({"file": fn, "line": ln, "rva": rva,
                            "prefix": pref, "imm_off": int(m.group(3)),
                            "width": int(m.group(4))})
    return out


def parse_orig():
    p = os.path.join(SRC, "orig_table.rs")
    tab = {}
    for m in RE_ORIG.finditer(open(p, encoding="utf-8", errors="replace").read()):
        rva = int(m.group(1), 16)
        tab[rva] = (int(m.group(2)), int(m.group(3)), int(m.group(4)))
    return tab


def imm_at(exe, rva, off, width):
    b = exe.read(rva + off, width)
    return int.from_bytes(b, "little") if len(b) == width else None


def verify(exe, s, expect=None):
    """이 exe 에서 사이트가 성립하는지 (prefix 일치 + 즉시값)."""
    b = exe.read(s["rva"], len(s["prefix"]))
    if len(b) != len(s["prefix"]) or list(b) != s["prefix"]:
        return False, None
    v = imm_at(exe, s["rva"], s["imm_off"], s["width"])
    if expect is not None and v != expect:
        return False, v
    return True, v


def search(exe, s, expect, lo, hi, tail=None):
    """[lo,hi) RVA 구간에서 prefix + 즉시값(+후미 문맥)이 맞는 곳을 전부."""
    va, rr, rsz = exe.text()
    pat = bytes(s["prefix"])
    blob = exe.raw[rr:rr + rsz]
    b0 = max(0, lo - va)
    b1 = min(len(blob), hi - va)
    hits, i = [], b0
    while True:
        i = blob.find(pat, i, b1)
        if i < 0:
            break
        rva = va + i
        v = imm_at(exe, rva, s["imm_off"], s["width"])
        if expect is None or v == expect:
            if tail:
                t = exe.read(rva + s["imm_off"] + s["width"], len(tail))
                if list(t) != tail:
                    i += 1
                    continue
            hits.append(rva)
        i += 1
    return hits


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("cmd", choices=["parse", "pin"])
    ap.add_argument("--write-plan")
    ap.add_argument("--source", choices=["src", "table"], default="table")
    ap.add_argument("--window", type=lambda x: int(x, 0), default=0x60000,
                    help="구 RVA 주변 검색 창 반경(기본 ±384KB)")
    a = ap.parse_args()

    orig = parse_orig()
    if a.source == "table":
        # ★orig_table 891행을 사이트 원본으로 쓴다(소스 직접호출 98건보다 전수).
        #   prefix 는 구 exe 실측 문맥으로 만든다 — 즉시값 자리는 비워 둔다(expect 로 따로 검증).
        e7pre = Exe(os.path.join(ANA, "tfm2_0.5.7", "TeamfightManager2.exe"))
        sites = []
        for rva, (io, w, exp) in sorted(orig.items()):
            ctx = e7pre.read(rva, io)          # 즉시값 직전까지 = prefix
            if len(ctx) != io or io == 0:
                continue
            sites.append({"file": "orig_table", "line": 0, "rva": rva,
                          "prefix": list(ctx), "imm_off": io, "width": w})
    else:
        sites = parse_sites()
    print("소스 사이트 = %d건 / orig_table = %d행" % (len(sites), len(orig)))
    byfile = collections.Counter(s["file"] for s in sites)
    for f, c in byfile.most_common():
        print("   %-26s %d" % (f, c))
    have = sum(1 for s in sites if s["rva"] in orig)
    print("orig_table 에 있는 사이트 = %d / %d  (없는 것은 **무가드 패치**)" % (have, len(sites)))
    if a.cmd == "parse":
        return

    e7 = Exe(os.path.join(ANA, "tfm2_0.5.7", "TeamfightManager2.exe"))
    e8 = Exe(LIVE)
    stat = collections.Counter()
    plan = []
    for s in sites:
        exp = orig.get(s["rva"], (None, None, None))[2]
        ok7, v7 = verify(e7, s, exp)
        ok8, v8 = verify(e8, s, exp)
        if ok8:
            stat["OK_0.5.8(그대로 유효)"] += 1
            continue
        if not ok7:
            stat["구exe서도 불일치(지문 낡음)"] += 1
            continue
        # 0.5.7 에선 성립, 0.5.8 에선 깨짐 → 재핀 대상
        lo, hi = s["rva"] - a.window, s["rva"] + a.window
        tail = list(e7.read(s["rva"] + s["imm_off"] + s["width"], 6))
        hits = search(e8, s, v7, lo, hi, tail if len(tail) == 6 else None)
        if len(hits) == 1:
            stat["UNIQUE(창 내 유일)"] += 1
            plan.append({"file": s["file"], "line": s["line"],
                         "old": "0x%x" % s["rva"], "new": "0x%x" % hits[0],
                         "prefix": " ".join("%02x" % x for x in s["prefix"]),
                         "imm_off": s["imm_off"], "width": s["width"],
                         "orig_imm": v7, "how": "prefix+imm 창내 유일"})
        elif len(hits) == 0:
            stat["NONE(창 내 후보 0)"] += 1
        else:
            stat["MULTI(%d~)" % min(len(hits), 9)] += 1
    print()
    print("%-28s %s" % ("판정", "건수"))
    print("-" * 46)
    for k, c in stat.most_common():
        print("%-28s %d" % (k, c))
    if a.write_plan:
        json.dump(plan, open(a.write_plan, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print()
        print("재핀 계획 %d건 -> %s" % (len(plan), a.write_plan))


if __name__ == "__main__":
    main()
