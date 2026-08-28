# -*- coding: utf-8 -*-
"""gpt_out 의 결과를 **원본과 나란히** 한 장으로 만든다(검수용 컨택트 시트).

왜: 배치로 여러 개를 만들면 하나씩 열어보기 번거롭다. 원본 대비로 봐야
    크기·발 위치·컨셉이 맞는지 즉시 판단할 수 있다.
★비교는 반드시 **프레임 중앙 정렬** — 게임이 그렇게 그린다. 아래 정렬로 보면
  프레임 높이가 다른 애니에서 엉뚱하게 어긋나 보인다.

실행
  python contact_sheet.py                     # gpt_out 전량
  python contact_sheet.py android bard        # 특정 챔프만
  python contact_sheet.py --new               # 아직 시트에 안 들어간 것만
"""
import argparse, json, os, sys
from PIL import Image, ImageDraw, ImageFont

MOD  = r"C:\tfm2mods\sylas"
DROP = os.path.join(MOD, "gpt_out")
VAN  = r"C:\Users\jungs\Desktop\claude\tfm2\bundle_unpacked_0826\aseprite_resources\champions"
FANIM = os.path.join(MOD, "aseprite_resources", "champions", "sylas#anim.fanim")
OUT  = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\sylas\gpt_refs"

def van_frames(champ, anim):
    a = json.load(open(os.path.join(VAN, champ + "#anim.fanim"), encoding="utf-8"))["anims"][anim]["frames"]
    sh = Image.open(os.path.join(VAN, champ + "#sheet.png")).convert("RGBA")
    out = []
    for f in a:
        d = f["data"]; x, y, w, h = int(d["x"]), int(d["y"]), int(d["w"]), int(d["h"])
        out.append(sh.crop((x, y, x + w, y + h)))
    return out

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("champs", nargs="*")
    ap.add_argument("--new", action="store_true", help="아직 사일러스 시트에 없는 것만")
    ap.add_argument("--zoom", type=int, default=2)
    ap.add_argument("--out", default=os.path.join(OUT, "_contact.png"))
    ap.add_argument("--gen-only", dest="genonly", action="store_true")
    a = ap.parse_args()
    try: sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    except Exception: pass
    have = set(json.load(open(FANIM, encoding="utf-8"))["anims"])
    ATL = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\sylas\vanilla_ult_sprites.json"
    body = {}
    for c in json.load(open(ATL, encoding="utf-8"))["champs"]:
        for r in c["anims"]:
            body[(c["id"], r["n"])] = r["body"]

    items = []
    for fn in sorted(os.listdir(DROP)):
        if not fn.endswith(".png") or "__" not in fn: continue
        champ, anim = fn[:-4].split("__", 1)
        anim = anim.split("@", 1)[0]
        if a.champs and champ not in a.champs: continue
        if a.new and ("%s_%s" % (anim, champ)) in have: continue
        if a.genonly and not body.get((champ, anim), True): continue
        items.append((champ, anim, os.path.join(DROP, fn)))
    if not items:
        print("대상 없음"); return

    Z = a.zoom
    rows = []
    for champ, anim, p in items:
        st = Image.open(p).convert("RGBA")
        try:  van = van_frames(champ, anim)
        except Exception: van = []
        n = len(van) if van else 1
        FW = st.width // max(1, n)
        mine = [st.crop((i*FW, 0, (i+1)*FW, st.height)) for i in range(n)] if n else [st]
        rows.append((champ, anim, van, mine))

    LBL, GAP, PADT = 170, 6, 22
    # ★행마다 자기 내용 크기로 잡는다(전역 최대치를 쓰면 시트가 쓸데없이 거대해진다)
    def row_metrics(van, mine):
        h = max([c.height for c in van] + [c.height for c in mine])
        w = max([c.width  for c in van] + [c.width  for c in mine])
        return w*Z + 6, h*Z + 6
    metr = [row_metrics(v, m) for _, _, v, m in rows]
    W = LBL + max((cw+GAP)*max(len(m), len(v)) for (cw,_), (_,_,v,m) in zip(metr, rows))
    H = 10 + sum(ch*2 + PADT for _, ch in metr)
    img = Image.new("RGB", (W, H), (24, 25, 32)); dr = ImageDraw.Draw(img)
    try: F = ImageFont.truetype(r"C:\Windows\Fonts\malgunbd.ttf", 15)
    except Exception: F = ImageFont.load_default()

    y = 8
    for (champ, anim, van, mine), (CW, CH) in zip(rows, metr):
        dr.text((8, y + 4), champ, font=F, fill=(240,242,248))
        dr.text((8, y + 24), anim, font=F, fill=(150,158,175))
        dr.text((8, y + CH + 4), "원본↑ 사일러스↓", font=F, fill=(112,120,138))
        for r, fs in enumerate((van, mine)):
            for i, c in enumerate(fs):
                x0 = LBL + i*(CW+GAP); y0 = y + r*CH
                cx = x0 + CW//2; cy = y0 + CH//2
                q = c.resize((c.width*Z, c.height*Z), Image.NEAREST)
                img.paste(q, (cx - q.width//2, cy - q.height//2), q)
                dr.rectangle((x0, y0, x0+CW-1, y0+CH-1), outline=(56,60,74))
        y += CH*2 + PADT
        dr.line((0, y-11, W, y-11), fill=(46,50,62))
    img.save(a.out)
    print("컨택트 시트: %s  (%d건, %dx%d)" % (a.out, len(rows), img.width, img.height))

if __name__ == "__main__":
    main()
