# -*- coding: utf-8 -*-
# verify_sites_053.py — 소스에 지금 박혀 있는 patch_imm_bytes 사이트가 0.5.3 exe 에서 prefix 검증을 통과하는지 정적 확인.
#   인게임 로그가 "applied=8/12" 처럼 카운트만 주므로, 어느 사이트가 skip 됐는지 여기서 특정한다.
#   (patch_imm_bytes 는 prefix 불일치 시 false 반환 → applied 카운트에서 빠진다)
import sys, re
import bytepatch_053 as B

roff = B.roff
SRCS = [r"C:\tfm2mods\tfm2_ai_adjust\src\detour.rs",
        r"C:\tfm2mods\tfm2_ai_adjust\src\disc19_repro.rs"]

# 사이트가 속한 apply_* 함수를 알아내기 위한 함수 경계 추출
RE_FN = re.compile(r'^unsafe fn (apply_\w+)', re.M)


def fn_of(src, pos):
    last = None
    for m in RE_FN.finditer(src):
        if m.start() < pos:
            last = m.group(1)
        else:
            break
    return last or "?"


total = {}
for path in SRCS:
    src = open(path, encoding="utf-8").read()
    B.SRC = path
    sites = B.parse_sites()
    print(f"===== {path.split(chr(92))[-1]} : {len(sites)}사이트 =====")
    for s in sites:
        rva, pre, off, w = s["rva"], s["pre"], s["off"], s["w"]
        # 소스 내 위치 → 소속 apply_ 함수
        m = re.search(r'patch_imm_bytes\(\s*base\s*\+\s*0x%x\b' % rva, src)
        fn = fn_of(src, m.start()) if m else "?"
        o = roff(B.SN, rva)
        if o is None:
            st, cur = "SECT", "-"
        else:
            cur = B.DN[o:o + off]
            st = "OK " if cur == pre else "FAIL"
        total.setdefault(fn, [0, 0])
        total[fn][1] += 1
        if st == "OK ":
            total[fn][0] += 1
        else:
            print(f"  [{st}] {fn:<22s} 0x{rva:<9x} 기대={pre.hex(' '):<14s} 실제={cur.hex(' ') if o else '섹션밖'}   {s['cmt'][:40]}")
    print()

print("===== apply_* 별 집계 (예상 applied) =====")
for fn, (ok, n) in sorted(total.items()):
    mark = "  ← 로그와 대조" if fn in ("apply_objective_imm", "apply_gb_imm", "apply_sev_imm", "apply_vis_imm") else ""
    print(f"  {fn:<24s} {ok}/{n}{mark}")
