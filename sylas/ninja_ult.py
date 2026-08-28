# -*- coding: utf-8 -*-
"""ninja 전용 — 궁 연출이 **이펙트가 아니라 캐릭터 자체의 변신**이라 일반 파이프라인이 안 통한다.

원본 관찰(유저 지적 + 실측)
  ult_pre     f0·f1 정상 → f2 노란 세로줄무늬로 흩어짐 → f3~f6 회색 연기만(캐릭터 없음)
  ult_attack  f0 줄무늬 → f1~f3 **캐릭터 자체가 노란 실루엣** + 노란 참격 → f4 정상 복귀
  (f0 노랑 98%·연결성분 13개 / f1 노랑 98%·1덩어리 / f2 56%·1덩어리 / f4 노랑 5%)

⟹ "원본 이펙트를 떼어 얹기"로는 안 된다. 필요한 것은 두 가지다.
   ① 캐릭터가 없는 프레임(줄무늬·연기) → **원본을 그대로** 쓴다
   ② 캐릭터가 노란 프레임              → **사일러스를 노랗게 물들인다**(원본 노랑 팔레트로)

노랗게 물들이기 = 밝기 보존 매핑. 사일러스 픽셀의 밝기를 원본 노랑 램프에 대응시킨다
(단색으로 칠하면 픽셀아트의 명암이 죽는다).

실행: python ninja_ult.py [--write]
"""
import json, os, shutil, sys
from collections import Counter
from PIL import Image, ImageDraw

sys.path.insert(0, r"C:\tfm2mods\sylas")
from graft_effect import van_anim

MOD  = r"C:\tfm2mods\sylas"
DROP = os.path.join(MOD, "gpt_out")
PREV = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\sylas\gpt_refs"

# 프레임별 처리 — 원본을 눈으로 보고 정한다(자동 판정이 통하지 않는 케이스라 명시가 정직하다)
SPEC = {
    #                f0        f1       f2                f3         f4         f5         f6
    "ult_pre":    ["sylas", "sylas", "stripe+smoke", "vanilla", "vanilla", "vanilla", "vanilla"],
    "ult_attack": ["stripe", "gold", "gold", "gold", "sylas"],
}
# 줄무늬 소멸/재등장 프레임에서 **몸 모양을 뜰 원본 프레임**(사일러스 쪽 인덱스)
STRIPE_SRC = {"ult_pre": 1, "ult_attack": 1}


def gold_ramp():
    """원본 노란 실루엣(ult_attack f1)에서 밝기순 노랑 램프를 만든다."""
    c = van_anim("ninja", "ult_attack")[1]
    cnt = Counter(p[:3] for p in c.getdata() if p[3] > 8)
    cols = sorted(cnt, key=lambda k: 0.299*k[0] + 0.587*k[1] + 0.114*k[2])
    return cols


def stripe(im, ramp, period=2):
    """★세로 줄무늬 소멸/재등장 — **사일러스 몸에서** 만든다.
    원본 줄무늬는 1px 바가 2px 간격(실측: 열별 픽셀 [0,0,5,0,8,0,17,...] = 한 칸 건너 한 줄).
    ⚠원본 줄무늬 스프라이트를 그대로 쓰면 **닌자 몸 크기**라 사일러스와 안 맞는다(유저 지적).
    """
    g = to_gold(im, ramp)
    px = g.load()
    bb = g.split()[3].getbbox()
    if not bb: return g
    for y in range(g.height):
        for x in range(g.width):
            if px[x, y][3] > 8 and (x - bb[0]) % period != 0:
                px[x, y] = (0, 0, 0, 0)
    return g


def to_gold(im, ramp):
    """밝기를 보존한 채 노랑 램프로 매핑."""
    out = im.copy(); px = out.load()
    for y in range(im.height):
        for x in range(im.width):
            p = px[x, y]
            if p[3] <= 8: continue
            lum = (0.299*p[0] + 0.587*p[1] + 0.114*p[2]) / 255.0
            g = ramp[min(len(ramp)-1, int(lum * len(ramp)))]
            px[x, y] = (g[0], g[1], g[2], p[3])
    return out


def main():
    write = "--write" in sys.argv
    try: sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    except Exception: pass
    ramp = gold_ramp()
    print("[노랑 램프] %d색 (%s … %s)" % (len(ramp),
          "#%02x%02x%02x" % ramp[0], "#%02x%02x%02x" % ramp[-1]))

    for anim, spec in SPEC.items():
        van = van_anim("ninja", anim)
        p = os.path.join(DROP, "ninja__%s.png" % anim)
        if not os.path.exists(p):
            print("✗ %s 생성물 없음" % anim); continue
        st = Image.open(p).convert("RGBA")
        n = len(van)
        if len(spec) != n:
            print("✗ %s 프레임 수 불일치 spec %d vs 원본 %d" % (anim, len(spec), n)); continue
        FW = st.width // n; FH = st.height
        mine = [st.crop((i*FW, 0, (i+1)*FW, FH)) for i in range(n)]

        # 원본 프레임을 쓸 경우를 대비해 필요한 폭·높이를 먼저 구한다(대칭 확장)
        padx = pady = 0
        for i, mode in enumerate(spec):
            if mode != "vanilla": continue
            c = van[i]
            padx = max(padx, (c.width  - FW + 1)//2)
            pady = max(pady, (c.height - FH + 1)//2)
        padx, pady = max(0, padx), max(0, pady)
        if padx or pady:
            print("[확장] %s 프레임 %dx%d → %dx%d (원본 프레임 수용, 대칭)"
                  % (anim, FW, FH, FW+padx*2, FH+pady*2))
            NFW, NFH = FW+padx*2, FH+pady*2
            g = Image.new("RGBA", (NFW*n, NFH), (0,0,0,0))
            for i in range(n): g.paste(mine[i], (i*NFW+padx, pady), mine[i])
            st, FW, FH = g, NFW, NFH
            mine = [st.crop((i*FW, 0, (i+1)*FW, FH)) for i in range(n)]

        out = Image.new("RGBA", (FW*n, FH), (0,0,0,0))
        for i, mode in enumerate(spec):
            if mode == "sylas":
                cell = mine[i]; note = "사일러스 그대로"
            elif mode == "gold":
                cell = to_gold(mine[i], ramp); note = "사일러스를 노랗게"
            elif mode == "stripe":
                cell = stripe(mine[STRIPE_SRC[anim]], ramp); note = "사일러스를 세로줄무늬로"
            elif mode == "stripe+smoke":
                # 사일러스는 제자리에서 줄무늬로 흩어지고, 연기는 원본을 중앙 기준으로 얹는다
                cell = stripe(mine[STRIPE_SRC[anim]], ramp)
                c = van[i]
                # 원본에서 연기 = 노랑이 아닌 부분
                sm = c.copy(); sp = sm.load()
                for y in range(c.height):
                    for x in range(c.width):
                        q0 = sp[x, y]        # ⚠`p` 로 쓰면 파일 경로 변수를 덮는다(2026-08-29 실사고)
                        if q0[3] > 8 and q0[0] > 150 and q0[1] > 110 and q0[2] < 110:
                            sp[x, y] = (0, 0, 0, 0)      # 노란 줄무늬는 버린다(사일러스 것을 쓴다)
                sb = sm.split()[3].getbbox()
                if sb:
                    sm2 = sm.crop(sb)
                    ox = int(round(FW/2 + (sb[0] - c.width/2)))
                    oy = int(round(FH/2 + (sb[1] - c.height/2)))
                    cell.paste(sm2, (ox, oy), sm2)
                note = "사일러스 줄무늬 + 원본 연기"
            else:
                # 원본 프레임을 **중앙 기준**으로 놓는다(게임이 중앙 기준으로 그린다)
                c = van[i]
                cell = Image.new("RGBA", (FW, FH), (0,0,0,0))
                cell.paste(c, ((FW-c.width)//2, (FH-c.height)//2), c)
                note = "원본 그대로 %dx%d" % c.size
            out.paste(cell, (i*FW, 0), cell)
            print("   %s f%d  %s" % (anim, i, note))

        if write:
            shutil.copy2(p, p + ".bak_ninja")
            out.save(p); print("[기록] %s (백업 .bak_ninja)" % p)

        # 미리보기
        Z = 3
        CW = max(FW, max(c.width for c in van)); CH = max(FH, max(c.height for c in van))
        img = Image.new("RGB", (120 + (CW*Z+8)*n, 24 + (CH*Z+20)*2), (24,25,32))
        dr = ImageDraw.Draw(img)
        dr.text((8, 24 + CH*Z//2), "원본", fill=(220,224,234))
        dr.text((8, 24 + CH*Z + 20 + CH*Z//2), "사일러스", fill=(220,224,234))
        for i in range(n):
            x0 = 120 + i*(CW*Z+8)
            for r, c in enumerate((van[i], out.crop((i*FW,0,(i+1)*FW,FH)))):
                y0 = 20 + r*(CH*Z+20)
                q = c.resize((c.width*Z, c.height*Z), Image.NEAREST)
                cx = x0+CW*Z//2; cy = y0+CH*Z//2
                img.paste(q, (cx-q.width//2, cy-q.height//2), q)
                dr.rectangle((x0,y0,x0+CW*Z-1,y0+CH*Z-1), outline=(56,60,74))
        pv = os.path.join(PREV, "_ninja_%s.png" % anim)
        img.save(pv); print("[미리보기] %s" % pv)
    if not write: print("\n미리보기만 했다. 적용하려면 --write")


if __name__ == "__main__":
    main()
