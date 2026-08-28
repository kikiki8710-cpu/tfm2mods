# -*- coding: utf-8 -*-
"""동작 레퍼런스(`gpt_refs\\_motion\\*.png`)에서 **이펙트를 지운다**.

왜: 프롬프트에 "이펙트 그리지 마"라고 써도, 레퍼런스 그림에 이펙트가 보이면 모델이 따라 그린다.
    그리고 이펙트가 섞이면 `--equalize` 가 캐릭터가 아니라 "이펙트 포함 덩어리"의 높이를 맞춰서
    **캐릭터 크기가 프레임마다 달라진다**(2026-08-29 dancer 실사고).

방법 두 가지를 순서대로 시도한다.
  ① 팔레트 차집합 — 그 챔프의 `idle/run/attack/hit` 에 없는 색 = 이펙트
  ② 연결 성분     — ①이 아무것도 못 찾으면, 중앙에서 먼 덩어리들을 이펙트로 본다
  둘 다 실패하면(이펙트가 캐릭터에 붙어 색도 같은 경우) **그냥 둔다** — 지우다 몸을 깎느니 낫다.

원본은 `.bak_full` 로 백업한다.
"""
import argparse, json, os, shutil, sys
from PIL import Image
sys.path.insert(0, r"C:\tfm2mods\sylas")
from graft_effect import char_palette, effect_mask, blob_effect, van_anim

REF = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\sylas\gpt_refs"
MOT = os.path.join(REF, "_motion")
VAN = r"C:\Users\jungs\Desktop\claude\tfm2\bundle_unpacked_0826\aseprite_resources\champions"


def strip_effect(frames, pal):
    """이펙트를 지운 프레임들과, 프레임당 제거 픽셀 수를 돌려준다."""
    out, removed = [], []
    for c in frames:
        eff = effect_mask(c, pal)
        if not eff.split()[3].getbbox():
            eff, _ = blob_effect(c)
        o = c.copy(); ep = eff.load(); op = o.load(); k = 0
        for y in range(c.height):
            for x in range(c.width):
                if ep[x, y][3] > 8: op[x, y] = (0, 0, 0, 0); k += 1
        # 몸까지 깎았으면(내용의 절반 이상 제거) 되돌린다
        before = sum(1 for p in c.getdata() if p[3] > 8)
        if before and k > before * 0.5:
            out.append(c); removed.append(0)
        else:
            out.append(o); removed.append(k)
    return out, removed


def render(frames, dst):
    CW = max(c.width for c in frames); CH = max(c.height for c in frames)
    Z = max(1, min(12, int(340 / max(CH, 1))))
    GAP, PAD, n = Z*8, Z*6, len(frames)
    img = Image.new("RGB", (PAD*2 + CW*Z*n + GAP*(n-1), PAD*2 + CH*Z), (255, 0, 255))
    for i, c in enumerate(frames):
        q = c.resize((c.width*Z, c.height*Z), Image.NEAREST)
        img.paste(q, (PAD + i*(CW*Z+GAP) + (CW*Z-q.width)//2, PAD + (CH*Z-q.height)), q)
    img.save(dst)
    return img.size, Z


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("only", nargs="*", help="특정 champ 만 (비우면 전량)")
    ap.add_argument("--write", action="store_true")
    a = ap.parse_args()
    try: sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    except Exception: pass

    pals, ok, skip, fail = {}, 0, 0, 0
    for fn in sorted(os.listdir(MOT)):
        if not fn.endswith(".png") or "__" not in fn: continue
        cid, anim = fn[:-4].split("__", 1)
        if a.only and cid not in a.only: continue
        try:
            frames = van_anim(cid, anim)
        except Exception as e:
            print("  ✗ %-28s 원본 없음 (%s)" % (fn[:-4], e)); fail += 1; continue
        if cid not in pals: pals[cid] = char_palette(cid)
        cleaned, rm = strip_effect(frames, pals[cid])
        tot = sum(rm)
        if tot == 0:
            print("  · %-28s 제거 0 — 그대로 둔다" % fn[:-4]); skip += 1; continue
        if a.write:
            p = os.path.join(MOT, fn)
            if not os.path.exists(p + ".bak_full"): shutil.copy2(p, p + ".bak_full")
            size, Z = render(cleaned, p)
            print("  ✓ %-28s 이펙트 %5dpx 제거  %s x%d" % (fn[:-4], tot, size, Z))
        else:
            print("  ✓ %-28s 이펙트 %5dpx 제거 예정  프레임별 %s" % (fn[:-4], tot, rm))
        ok += 1
    print("\n정리 %d / 변화없음 %d / 실패 %d" % (ok, skip, fail))
    if not a.write: print("미리보기만 했다. 적용하려면 --write")


if __name__ == "__main__":
    main()
