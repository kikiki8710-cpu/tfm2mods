# -*- coding: utf-8 -*-
# bytepatch3_053.py — NOCONT(컨테이너 미해결) 사이트를 전역 순서대응으로 해결.
#   근거: 이 사이트들은 제네릭 모노모픽화로 같은 코드가 N벌 복제된 클러스터에 속한다.
#         OLD 전역에 N개, NEW 전역에도 N개면 링커가 그룹 내 순서를 뒤집을 이유가 없으므로 주소순으로 짝짓는다.
#         시그는 추측하지 않고 소스에서 파싱한 prefix + OLD exe 의 실제 imm 을 쓴다.
import sys, re, json
import bytepatch_053 as B

DO, DN, SO, SN, FO, FN = B.DO, B.DN, B.SO, B.SN, B.FO, B.FN
roff, owner = B.roff, B.owner


def textblob(d, secs):
    for nm, va, vsz, rraw, rsz in secs:
        if nm == ".text":
            return va, d[rraw:rraw + rsz]


VAO, BO = textblob(DO, SO)
VAN, BN = textblob(DN, SN)
SITES = {s["rva"]: s for s in B.parse_sites()}


def sig_of(rva, extra=0):
    """소스 prefix + OLD 실제 imm (+뒤쪽 extra 바이트 문맥)"""
    s = SITES[rva]
    o = roff(SO, rva)
    n = s["off"] + s["w"] + extra
    return DO[o:o + n], s


def solve(rva, name, extras=(0, 6, 12, 20)):
    for ex in extras:
        sig, s = sig_of(rva, ex)
        ho = [VAO + m.start() for m in re.finditer(re.escape(sig), BO, re.S)]
        hn = [VAN + m.start() for m in re.finditer(re.escape(sig), BN, re.S)]
        if not ho or rva not in ho:
            continue
        if len(ho) == len(hn):
            k = ho.index(rva)
            f = owner(FN, hn[k])
            print(f"[{name}] 0x{rva:x} → **0x{hn[k]:x}**  (시그{len(sig)}B, 전역 {len(ho)}=={len(hn)} 순서 #{k}"
                  f", NEW컨테이너 0x{f[0]:x})" if f else "")
            return hn[k]
        if len(hn) == 1 and len(ho) >= 1:
            f = owner(FN, hn[0])
            print(f"[{name}] 0x{rva:x} → **0x{hn[0]:x}**  (시그{len(sig)}B, NEW 전역 유일 / OLD {len(ho)}건"
                  f", NEW컨테이너 0x{f[0]:x})")
            return hn[0]
    sig, s = sig_of(rva, 0)
    ho = [VAO + m.start() for m in re.finditer(re.escape(sig), BO, re.S)]
    hn = [VAN + m.start() for m in re.finditer(re.escape(sig), BN, re.S)]
    print(f"[{name}] 0x{rva:x} ✗ 미해결  sig={sig.hex(' ')} OLD {len(ho)} vs NEW {len(hn)}")
    return None


R = {}
print("=" * 74)
print("NOCONT 사이트 — 전역 순서대응")
print("=" * 74)
for rva, nm in [
    (0x2126ae3, "vis_window"),
    (0x2398342, "d19b_hp_low"), (0x2398ef3, "d19b_near1"), (0x2398f3c, "d19b_near2"),
    (0x23ba8f3, "reach_cap2_gt"),
    (0x22edb5f, "sev2_tr49"), (0x22edb65, "sev2_hp65"), (0x22edb6b, "sev2_tr29"),
    (0x22edb71, "sev2_hp40"), (0x22edb7b, "sev2_tr17"),
]:
    R[nm] = solve(rva, nm)

print()
print("=" * 74)
print("잔여: an_cull_dist (OLD 2 vs NEW 1) — imm 단독 스캔")
print("=" * 74)
for rva, nm in [(0x2376e86, "an_cull_dist_18"), (0x2381df5, "an_cull_dist_19")]:
    sig = bytes.fromhex("e0f50500")           # imm32 0x5f5e0 단독
    ho = [VAO + m.start() for m in re.finditer(re.escape(sig), BO, re.S)]
    hn = [VAN + m.start() for m in re.finditer(re.escape(sig), BN, re.S)]
    print(f"[{nm}] imm 0x5f5e0 전역: OLD {len(ho)}건 {[hex(x) for x in ho[:6]]}")
    print(f"          NEW {len(hn)}건 {[hex(x) for x in hn[:6]]}")
    for h in hn:
        f = owner(FN, h)
        o = roff(SN, h - 3)
        print(f"    NEW @0x{h:x} 앞3B={DN[o:o+3].hex(' ')} fn=0x{f[0]:x}" if f else f"    NEW @0x{h:x} (함수밖)")
    break

json.dump({k: (hex(v) if v else None) for k, v in R.items()},
          open(r"C:\tfm2mods\_bytepatch3_053.json", "w", encoding="utf-8"), indent=1)
