# -*- coding: utf-8 -*-
# bytepatch2_053.py — 1차(bytepatch_053.py)에서 MISS/NOCONT/OK? 로 남은 사이트를 두 기법으로 마무리.
#   ① 마스크 스캔  : 레지스터·스택오프셋 바이트만 와일드카드 → 재컴파일로 레지스터 배정이 바뀐 사이트를 잡는다.
#   ② 순서 대응    : OLD 전역에 N벌 있는 복제 클러스터(모노모픽화)가 NEW 에도 N벌이면 주소 오름차순으로 짝짓는다.
#   ③ 개수 확증    : OLD 컨테이너 내 시그 개수 == NEW 컨테이너 내 개수면 순서대응이 성립 → OK? 를 승격.
import sys, io, re, json
import bytepatch_053 as B          # ⚠ 이 모듈이 sys.stdout 을 utf-8 로 재래핑한다 — 여기서 또 감싸면
                                   #    앞 wrapper 가 GC 되며 buffer 를 닫아버리므로 그대로 물려 쓴다.

DO, DN, SO, SN, FO, FN = B.DO, B.DN, B.SO, B.SN, B.FO, B.FN
roff, owner = B.roff, B.owner


def textblob(d, secs):
    for nm, va, vsz, rraw, rsz in secs:
        if nm == ".text":
            return va, d[rraw:rraw + rsz]


VAO, BO = textblob(DO, SO)
VAN, BN = textblob(DN, SN)


def scan(blob, va, pat):
    return [va + m.start() for m in re.finditer(pat, blob, re.S)]


def maskpat(sig, mask_idx):
    """sig 바이트열에서 mask_idx 위치를 . 로 치환한 정규식"""
    out = b""
    for i, b in enumerate(sig):
        out += b"." if i in mask_idx else re.escape(bytes([b]))
    return out


def contbytes(fns, d, secs, rva):
    f = owner(fns, rva)
    if not f:
        return None, None
    o = roff(secs, f[0])
    return f, d[o:o + (f[1] - f[0])]


def report(name, old_site, sig, mask_idx, new_cont=None):
    """마스크 시그로 OLD/NEW 전역(또는 NEW 컨테이너) 스캔 후 순서대응."""
    pat = maskpat(sig, mask_idx)
    ho = scan(BO, VAO, pat)
    hn = scan(BN, VAN, pat)
    tag = f"[{name}] 0x{old_site:x} sig={sig.hex(' ')} mask={sorted(mask_idx)}"
    if new_cont is not None:
        f = owner(FN, new_cont)
        hn_c = [h for h in hn if f and f[0] <= h < f[1]]
        if len(hn_c) == 1:
            print(f"{tag}\n    ✓ NEW 컨테이너 내 유일 → **0x{hn_c[0]:x}**")
            return hn_c[0]
        print(f"{tag}\n    컨테이너 내 {len(hn_c)}건: " + ", ".join(hex(x) for x in hn_c[:6]))
    if len(ho) == len(hn) and ho:
        try:
            k = ho.index(old_site)
        except ValueError:
            print(f"{tag}\n    ✗ OLD 전역({len(ho)})에 자기 자신 없음")
            return None
        print(f"{tag}\n    ✓ 전역 개수 일치 OLD {len(ho)} == NEW {len(hn)}, 순서 #{k} → **0x{hn[k]:x}**")
        return hn[k]
    print(f"{tag}\n    ~ 개수 불일치 OLD {len(ho)} vs NEW {len(hn)}  " +
          ("OLD:" + ",".join(hex(x) for x in ho[:5]) + " NEW:" + ",".join(hex(x) for x in hn[:5])))
    return None


print("=" * 78)
print("① MISS 6건 — 레지스터/스택오프셋 마스크")
print("=" * 78)
R = {}
# cmp qword [rbp-0x28], 0x1f  → [rbp-??] 오프셋만 마스크
R["dn_hp_low"] = report("dn_hp_low", 0x1b934ec, bytes.fromhex("48837dd81f"), {3}, 0xdec6b0)
R["dn_hp_crit"] = report("dn_hp_crit", 0x1b9351c, bytes.fromhex("48837dd815"), {3}, 0xdec6b0)
# add r14, 0x78 → 레지스터(ModRM) 마스크
R["dn_lane_margin"] = report("dn_lane_margin", 0x1bdac95, bytes.fromhex("4983c678"), {0, 2}, 0xdf9320)
# cmp r10, 0x5f5e0 → REX+ModRM 마스크
R["an_cull_dist"] = report("an_cull_dist", 0x2376e86, bytes.fromhex("4981fae0f50500"), {0, 2}, 0xd94d00)
# mov qword [rbp+0x1b0], 0x3d090 → 변위 4B 마스크
R["gb_far"] = report("gb_far", 0x22b2ca5, bytes.fromhex("48c785b001000090d00300"), {3, 4, 5, 6}, 0xe06c10)
# mov r8d, 0xd693a401 → 레지스터 마스크
R["gb_pred"] = report("gb_pred", 0x22b2bb1, bytes.fromhex("41b801a493d6"), {0, 1}, 0xe06c10)

print()
print("=" * 78)
print("② NOCONT 10건 — 컨테이너 미해결분, 전역 순서대응")
print("=" * 78)
# sev 클러스터 4벌(0x22e3cdf / 0x22edb5f / 0x22effff / 0x23a0c21) 중 미해결 1벌
sev = bytes.fromhex("31")  # placeholder (아래에서 문맥 시그 사용)
for nm, site, sg, mk in [
    ("sev4_tr49",  0x22edb5f, bytes.fromhex("83f8314c0f"), set()),
    ("vis_window", 0x2126ae3, bytes.fromhex("4881c658020000"), {0, 2}),
    ("d19_hp_low", 0x2398342, bytes.fromhex("48837dd81f"), {3}),
    ("d19_near1",  0x2398ef3, bytes.fromhex("48b801e9a4350300 0000".replace(" ", "")), set()),
    ("d19_near2",  0x2398f3c, bytes.fromhex("48b801e9a4350300 0000".replace(" ", "")), set()),
    ("reach_cap2", 0x23ba8f3, bytes.fromhex("48b8010444044009 0000".replace(" ", "")), set()),
]:
    o = roff(SO, site)
    real = DO[o:o + len(sg)]
    if real != sg:
        print(f"[{nm}] OLD 실제 바이트 = {DO[o:o+12].hex(' ')} (기대 {sg.hex(' ')}) — 시그 재설정 필요")
        continue
    R[nm] = report(nm, site, sg, mk)
print()
json.dump({k: (hex(v) if v else None) for k, v in R.items()},
          open(r"C:\tfm2mods\_bytepatch2_053.json", "w", encoding="utf-8"), indent=1)
