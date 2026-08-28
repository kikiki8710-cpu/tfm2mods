# -*- coding: utf-8 -*-
"""boomerang_hunter 전용 — 원본 부메랑을 사일러스 프레임에 얹는다.

왜 전용 스크립트인가
  `graft_effect.py` 의 팔레트 차집합이 이 챔프에선 **속만 파낸다** — 부메랑의 붉은색과 검은 외곽선을
  캐릭터도 쓰기 때문이다(실측: 37x37 부메랑에서 29x29 안쪽만 추출됨).
  그런데 이 챔프는 **`ult_boomerang` 애니에 온전한 부메랑(37x37) 이 따로 있다.**
  ⟹ 추출하지 말고 **그 완성품을 갖다 쓴다.** 위치만 원본 프레임에서 알아내면 된다.

위치 판정
  팔레트 차집합 마스크는 모양은 못 살려도 **어디 있는지는 정확히 짚는다.**
  그 마스크의 중심에 온전한 부메랑을 중앙 맞춤으로 놓는다.
  마스크가 안 잡히는 뒤쪽 프레임(부메랑이 멀리 날아가 작아진 것)은 연결 성분으로 집는다.

배치는 언제나 **프레임 중앙 기준**(게임이 그렇게 그린다).
"""
import json, os, shutil, sys
from PIL import Image
sys.path.insert(0, r"C:\tfm2mods\sylas")
from graft_effect import char_palette, effect_mask, blob_effect, van_anim

MOD  = r"C:\tfm2mods\sylas"
DROP = os.path.join(MOD, "gpt_out")
VAN  = r"C:\Users\jungs\Desktop\claude\tfm2\bundle_unpacked_0826\aseprite_resources\champions"
CID, ANIM = "boomerang_hunter", "ult"

def main():
    write = "--write" in sys.argv
    A = json.load(open(os.path.join(VAN, CID + "#anim.fanim"), encoding="utf-8"))["anims"]
    sh = Image.open(os.path.join(VAN, CID + "#sheet.png")).convert("RGBA")
    d = A["ult_boomerang"]["frames"][0]["data"]
    boom = sh.crop((int(d["x"]), int(d["y"]),
                    int(d["x"]) + int(d["w"]), int(d["y"]) + int(d["h"])))
    bb = boom.split()[3].getbbox(); boom = boom.crop(bb)
    print("[부메랑] ult_boomerang %dx%d (원본 그대로)" % boom.size)

    van = van_anim(CID, ANIM)
    pal = char_palette(CID)
    strip_p = os.path.join(DROP, "%s__%s.png" % (CID, ANIM))
    st = Image.open(strip_p).convert("RGBA")
    n = len(van); FW = st.width // n; FH = st.height
    mine = [st.crop((i*FW, 0, (i+1)*FW, FH)) for i in range(n)]

    put = []
    for i, c in enumerate(van):
        # ① 팔레트 마스크로 **위치**를 잡는다(모양은 못 믿는다)
        m = effect_mask(c, pal); mb = m.split()[3].getbbox()
        src, use = None, None
        if mb:
            cx = (mb[0] + mb[2]) / 2 - c.width / 2
            cy = (mb[1] + mb[3]) / 2 - c.height / 2
            src, use = boom, (cx, cy)
            what = "온전한 부메랑"
        else:
            # ② 안 잡히면 연결 성분 — 멀리 날아간 작은 부메랑/잔상
            e, _ = blob_effect(c); eb = e.split()[3].getbbox()
            if eb:
                src = e.crop(eb)
                use = ((eb[0] + eb[2]) / 2 - c.width / 2, (eb[1] + eb[3]) / 2 - c.height / 2)
                what = "연결성분 %dx%d" % src.size
        if not src:
            print("   f%d 없음" % i); put.append(None); continue
        px = int(round(FW/2 + use[0] - src.width/2))
        py = int(round(FH/2 + use[1] - src.height/2))
        put.append((src, px, py))
        print("   f%d %s → 중앙대비(%+.1f,%+.1f) → 프레임 내 (%d,%d)" % (i, what, use[0], use[1], px, py))

    # 프레임 밖으로 나가면 **대칭** 확장(중앙 기준 기하 보존)
    padx = pady = 0
    for p in put:
        if not p: continue
        s2, x, y = p
        padx = max(padx, -x, x + s2.width - FW); pady = max(pady, -y, y + s2.height - FH)
    padx, pady = max(0, padx), max(0, pady)
    if padx or pady:
        print("[확장] 프레임 %dx%d → %dx%d (대칭)" % (FW, FH, FW+padx*2, FH+pady*2))
        NFW, NFH = FW+padx*2, FH+pady*2
        g = Image.new("RGBA", (NFW*n, NFH), (0,0,0,0))
        for i in range(n): g.paste(mine[i], (i*NFW+padx, pady), mine[i])
        st, FW, FH = g, NFW, NFH
        put = [(p[0], p[1]+padx, p[2]+pady) if p else None for p in put]

    out = st.copy()
    for i, p in enumerate(put):
        if p: out.paste(p[0], (i*FW + p[1], p[2]), p[0])
    if write:
        shutil.copy2(strip_p, strip_p + ".bak_graft")
        out.save(strip_p); print("[기록] %s (백업 .bak_graft)" % strip_p)
    else:
        print("\n미리보기만 했다. 적용하려면 --write")

if __name__ == "__main__":
    main()
