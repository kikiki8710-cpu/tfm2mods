# -*- coding: utf-8 -*-
"""이펙트 분리 방법 3종을 모두 시도해 **어느 것이 통하는지 판정**한다.

챔피언마다 통하는 방법이 다르다(실측).
  palette  팔레트 차집합 — 이펙트가 궁 전용 색일 때        (demon 주황 불꽃)
  blob     연결 성분     — 이펙트가 캐릭터와 떨어져 있을 때 (dancer 흰 초승달)
  top      빈 띠 위쪽    — 이펙트가 머리 위에 떠 있을 때    (dark_mage 보라)

판정 기준 = **이펙트를 찾은 프레임 수**(많을수록 좋음), 동률이면 픽셀 수가 많은 쪽.
⚠자동으로 붙이지 않는다 — 틀린 방법이 몸을 이펙트로 오인하면 프레임이 망가진다.
   이 스크립트는 **어느 방법을 쓸지 표로 보여줄 뿐**이고, 적용은 graft_effect.py 로 한 건씩 한다.

실행
  python auto_graft.py                 # gpt_out 전량 판정
  python auto_graft.py druid fighter   # 특정 챔프만
"""
import argparse, json, os, sys
from PIL import Image
sys.path.insert(0, r"C:\tfm2mods\sylas")
from graft_effect import char_palette, effect_mask, blob_effect, van_anim

MOD  = r"C:\tfm2mods\sylas"
DROP = os.path.join(MOD, "gpt_out")
VAN  = r"C:\Users\jungs\Desktop\claude\tfm2\bundle_unpacked_0826\aseprite_resources\champions"
ATL  = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\sylas\vanilla_ult_sprites.json"
FAN  = os.path.join(MOD, "aseprite_resources", "champions", "sylas#anim.fanim")


def auto_top(c):
    w, h = c.size; al = c.split()[3]
    rows = [any(al.getpixel((x, y)) > 8 for x in range(w)) for y in range(h)]
    occ = [y for y, v in enumerate(rows) if v]
    if not occ: return None
    best, cur = None, None
    for y in range(occ[0], occ[-1]):
        if not rows[y]:
            if cur is None: cur = y
        else:
            if cur is not None:
                if best is None or (y - cur) > (best[1] - best[0]): best = (cur, y)
                cur = None
    return best[0] if best else None


def score(frames, pal, mode):
    hit = px = 0
    for c in frames:
        if mode == "palette":  e = effect_mask(c, pal)
        elif mode == "blob":   e, _ = blob_effect(c)
        else:
            t = auto_top(c)
            e = effect_mask(c, pal, t) if t is not None else Image.new("RGBA", c.size, (0,0,0,0))
        bb = e.split()[3].getbbox()
        if not bb: continue
        n = sum(1 for p in e.getdata() if p[3] > 8)
        tot = sum(1 for p in c.getdata() if p[3] > 8)
        if tot and n > tot * 0.75:      # 몸까지 먹었다 = 오판
            continue
        hit += 1; px += n
    return hit, px


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("champs", nargs="*")
    a = ap.parse_args()
    try: sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    except Exception: pass

    body = {}
    for c in json.load(open(ATL, encoding="utf-8"))["champs"]:
        for r in c["anims"]: body[(c["id"], r["n"])] = r["body"]

    pals = {}
    print("%-30s %-4s %-14s %-14s %-14s  %s" %
          ("대상", "프레", "palette", "blob", "top", "권장"))
    for fn in sorted(os.listdir(DROP)):
        if not fn.endswith(".png") or "__" not in fn: continue
        cid, anim = fn[:-4].split("__", 1)
        if a.champs and cid not in a.champs: continue
        if not body.get((cid, anim), True): continue          # 이펙트 전용은 이미 원본
        try: frames = van_anim(cid, anim)
        except Exception: continue
        if cid not in pals: pals[cid] = char_palette(cid)
        n = len(frames)
        res = {m: score(frames, pals[cid], m) for m in ("palette", "blob", "top")}
        best = max(res, key=lambda m: (res[m][0], res[m][1]))
        if res[best][0] == 0: best = "—"
        print("%-30s %-4d %-14s %-14s %-14s  %s" % (
            cid + "__" + anim, n,
            "%d프레임 %dpx" % res["palette"], "%d프레임 %dpx" % res["blob"],
            "%d프레임 %dpx" % res["top"], best))


if __name__ == "__main__":
    main()
