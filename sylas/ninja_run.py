# -*- coding: utf-8 -*-
r"""ninja `ult_run` 전용 — 원본은 **닌자 달리기 포즈 전체가 금색 + 금색 속도선**이다.

왜 별도 스크립트인가
  구 상태는 바닐라를 그대로 복사해 둬서 화면에 **닌자가 뛰어다녔다**(유저 지적 2026-08-29).
  "챔피언이 통째로 금색"이라 파이프라인이 캐릭터로 인식하지 못하고 이펙트 취급했다.
  재료가 `gpt_out` 스트립이 아니라 **사일러스 자기 `run` 애니**여서 `ninja_ult.py` 의
  SPEC 파이프라인과 경로가 다르다.

만드는 법
  사일러스 run(5프레임) → 8프레임으로 순환 → `to_gold`(밝기 보존 금색 매핑)
  속도선 = 바닐라 `ult_run` 에서 **최대 연결성분(=몸통)을 뺀 나머지**
  배치 = 바닐라 같은 프레임의 **몸통 중심**에 맞춘다(프레임 중앙 기준, 발은 바닥)

실행:  python ninja_run.py [--write]
"""
import json, io, os, shutil, sys
from collections import deque
from PIL import Image

sys.path.insert(0, r"C:\tfm2mods\sylas")
from ninja_ult import gold_ramp, to_gold, van_anim, MOD, DROP

CH = os.path.join(MOD, "aseprite_resources", "champions")


def split_body(f):
    """최대 연결성분(몸통)과 나머지(속도선)로 가른다."""
    W, H = f.size; px = f.load()
    seen = [[False]*W for _ in range(H)]; comps = []
    for y in range(H):
        for x in range(W):
            if seen[y][x] or px[x, y][3] <= 8: continue
            q = deque([(x, y)]); seen[y][x] = True; pts = []
            while q:
                cx, cy = q.popleft(); pts.append((cx, cy))
                for dx in (-1, 0, 1):
                    for dy in (-1, 0, 1):
                        nx, ny = cx+dx, cy+dy
                        if 0 <= nx < W and 0 <= ny < H and not seen[ny][nx] and px[nx, ny][3] > 8:
                            seen[ny][nx] = True; q.append((nx, ny))
            comps.append(pts)
    comps.sort(key=len, reverse=True)
    body = Image.new("RGBA", f.size, (0, 0, 0, 0))
    rest = Image.new("RGBA", f.size, (0, 0, 0, 0))
    bp, rp = body.load(), rest.load()
    for ci, pts in enumerate(comps):
        t = bp if ci == 0 else rp
        for x, y in pts: t[x, y] = px[x, y]
    return body, rest


def main():
    try: sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    except Exception: pass
    write = "--write" in sys.argv

    md = json.load(io.open(os.path.join(CH, "sylas#anim.fanim"), encoding="utf-8-sig"))
    ms = Image.open(os.path.join(CH, "sylas#sheet.png")).convert("RGBA")
    sy = [ms.crop((int(f["data"]["x"]), int(f["data"]["y"]),
                   int(f["data"]["x"]) + int(f["data"]["w"]),
                   int(f["data"]["y"]) + int(f["data"]["h"])))
          for f in md["anims"]["run"]["frames"]]
    van = van_anim("ninja", "ult_run")
    n = len(van); ramp = gold_ramp()
    print("[금 램프] %d색 · 바닐라 %d프레임 · 사일러스 run %d프레임(순환)" % (len(ramp), n, len(sy)))

    FW = max(max(f.width for f in van), max(f.width for f in sy)) + 8
    FH = max(max(f.height for f in van), max(f.height for f in sy))
    out = Image.new("RGBA", (FW * n, FH), (0, 0, 0, 0))
    print("[프레임] %dx%d" % (FW, FH))
    for i in range(n):
        body, streak = split_body(van[i])
        g = to_gold(sy[i % len(sy)], ramp)
        gb = g.split()[3].getbbox(); bb = body.split()[3].getbbox()
        want = ((bb[0] + bb[2]) / 2 - van[i].width / 2) if bb else 0.0
        gx = int(round(FW / 2 + want - (gb[0] + gb[2]) / 2)) if gb else 0
        gx = max(0, min(FW - g.width, gx))
        cell = Image.new("RGBA", (FW, FH), (0, 0, 0, 0))
        cell.paste(g, (gx, FH - g.height))                       # 빈 자리 → 마스크 없이
        sx = int(round(FW / 2 - van[i].width / 2))
        cell.paste(streak, (sx, FH - van[i].height), streak)     # 기존 위 → 마스크 사용
        out.paste(cell, (i * FW, 0))
        print("   f%d 몸통중심 %+.1f → 금사일러스 x=%d · 속도선 %dpx"
              % (i, want, gx, sum(1 for q in streak.getdata() if q[3] > 8)))

    dst = os.path.join(DROP, "ninja__ult_run.png")
    if write:
        if not os.path.exists(dst + ".bak_copy"): shutil.copy2(dst, dst + ".bak_copy")
        out.save(dst); print("[기록] %s (백업 .bak_copy)" % dst)
    else:
        pv = os.path.join(r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\sylas\gpt_refs",
                          "_ninja_ult_run.png")
        Z = 5
        from PIL import ImageDraw
        img = Image.new("RGB", (10 + n * (FW * Z + 6), 30 + FH * Z * 2 + 20), (22, 23, 29))
        dr = ImageDraw.Draw(img)
        for i in range(n):
            x0 = 6 + i * (FW * Z + 6)
            for r, c in enumerate((van[i], out.crop((i * FW, 0, (i + 1) * FW, FH)))):
                y0 = 20 + r * (FH * Z + 10)
                q = c.resize((c.width * Z, c.height * Z), Image.NEAREST)
                img.paste(q, (x0 + (FW * Z - q.width) // 2, y0 + (FH * Z - q.height)), q)
                dr.rectangle((x0, y0, x0 + FW * Z, y0 + FH * Z), outline=(58, 62, 76))
            dr.text((x0 + 3, 6), "f%d" % i, fill=(226, 230, 240))
        img.save(pv); print("[미리보기] %s  (위=바닐라 / 아래=사일러스)" % pv)
        print("\n미리보기만 했다. 적용하려면 --write")


if __name__ == "__main__":
    main()
