# -*- coding: utf-8 -*-
"""원본 챔피언의 **궁 이펙트만** 떼어 사일러스 생성물에 얹는다.

왜 필요한가
  모델에게 이펙트까지 그리게 하면 원본과 달라진다. 이펙트는 **원본을 그대로 쓰는 게 정확하고 빠르다.**
  모델은 캐릭터 동작만 맡기면 된다(2026-08-29 knight 에서 처음 성립).

어떻게 캐릭터와 이펙트를 가르나 — **팔레트 차집합**
  그 챔피언의 `idle`(+`attack`) 프레임에 등장하는 색 = **캐릭터 팔레트**.
  궁 프레임에서 그 팔레트에 없는 색 = **이펙트**.
  근거: 이펙트는 궁 전용 색을 쓴다(실측 — dancer 흰 초승달 / dark_mage 보라 / demon 주황 불꽃 /
  knight 보라 깃발. 전부 idle 팔레트에 없다). 색 판정보다 일반적이라 챔프별 튜닝이 필요 없다.
  ⚠검은 외곽선은 캐릭터도 쓰므로 기본 제외된다. `--outline` 으로 **이펙트에 인접한** 어두운 픽셀만 포함할 수 있다.

  `--top N` 을 주면 팔레트와 무관하게 **위에서 N행**만 이펙트로 본다(머리 위 선 같은 것).

배치는 **프레임 중앙 기준** — 게임이 그렇게 그리므로 원본에서의 중앙 대비 위치를 그대로 옮긴다.
결과는 gpt_out 의 스트립을 **덮어쓴다**(원본은 `.bak_graft` 로 백업).

실행
  python graft_effect.py dancer ult                 # 미리보기
  python graft_effect.py dancer ult --write
  python graft_effect.py dark_mage ult_line --top 12 --write
"""
import argparse, json, os, shutil, sys
from PIL import Image, ImageDraw

MOD  = r"C:\tfm2mods\sylas"
DROP = os.path.join(MOD, "gpt_out")
VAN  = r"C:\Users\jungs\Desktop\claude\tfm2\bundle_unpacked_0826\aseprite_resources\champions"
PREV = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\sylas\gpt_refs"


def van_anim(champ, anim):
    a = json.load(open(os.path.join(VAN, champ + "#anim.fanim"), encoding="utf-8"))["anims"][anim]["frames"]
    sh = Image.open(os.path.join(VAN, champ + "#sheet.png")).convert("RGBA")
    out = []
    for f in a:
        d = f["data"]; x, y, w, h = int(d["x"]), int(d["y"]), int(d["w"]), int(d["h"])
        out.append(sh.crop((x, y, x + w, y + h)))
    return out


def char_palette(champ, anims=("idle", "run", "attack", "hit")):
    """캐릭터가 쓰는 색 집합. 궁이 아닌 애니에서만 모은다."""
    pal = set()
    try:
        A = json.load(open(os.path.join(VAN, champ + "#anim.fanim"), encoding="utf-8"))["anims"]
    except Exception:
        return pal
    sh = Image.open(os.path.join(VAN, champ + "#sheet.png")).convert("RGBA")
    for an in anims:
        if an not in A: continue
        for f in A[an]["frames"]:
            d = f["data"]; x, y, w, h = int(d["x"]), int(d["y"]), int(d["w"]), int(d["h"])
            for p in sh.crop((x, y, x + w, y + h)).convert("RGBA").getdata():
                if p[3] > 8: pal.add(p[:3])
    return pal


def morph_open(im, k=1):
    """★모폴로지 열림(침식→팽창) — **얇은 선을 지우고 두꺼운 덩어리만 남긴다.**

    왜: 원본 중엔 **캐릭터 실루엣을 따라 그린 1px 테두리**를 이펙트로 쓰는 게 있다
    (실측 hunter/ult = 주황 림). 그건 그 챔피언 몸 모양이라, 체형이 다른 사일러스에 얹으면
    **남의 실루엣이 떠 있는 꼴**이 된다. 진짜 이펙트(초승달 등)는 속이 찬 덩어리라 살아남는다.

    k=1 이면 3x3 이웃이 전부 이펙트인 픽셀만 남기고(침식), 그 결과를 다시 3x3 으로 넓힌다(팽창).
    """
    w, h = im.size; src = im.load()
    def on(x, y):
        return 0 <= x < w and 0 <= y < h and src[x, y][3] > 8
    core = set()
    for y in range(h):
        for x in range(w):
            if not on(x, y): continue
            if all(on(x+dx, y+dy) for dx in range(-k, k+1) for dy in range(-k, k+1)):
                core.add((x, y))
    out = Image.new("RGBA", (w, h), (0,0,0,0)); dst = out.load()
    for (x, y) in core:
        for dx in range(-k, k+1):
            for dy in range(-k, k+1):
                nx, ny = x+dx, y+dy
                if on(nx, ny): dst[nx, ny] = src[nx, ny]
    return out


def outside_effect(im, pal, grow=3):
    """★몸통 밖 = 이펙트. 색으로도 빈도로도 못 가를 때 쓰는 **공간** 판정.

    몸통 위치는 **채도 있는 캐릭터 색**(보라 머리·빨간 옷·살색 등)으로 잡는다.
    흰색·검정·회색은 이펙트도 쓰므로 위치 판정에서 제외한다.
    그 색들의 bbox 를 몸통으로 보고 조금 넓힌 뒤, **그 밖의 픽셀을 이펙트**로 본다.
    (실측 dual_blader/ult: 흰 검기의 `#ffffff` 가 궁 238px / idle 125px = 1.9배라
     빈도로 못 가른다 — 옷이 희기 때문. 그러나 검기는 몸통 밖으로 크게 뻗는다.)
    """
    w, h = im.size; src = im.load()
    def vivid(c):
        mx, mn = max(c[:3]), min(c[:3])
        return mx > 40 and (mx - mn) > 40            # 무채색 제외
    xs, ys = [], []
    for y in range(h):
        for x in range(w):
            p = src[x, y]
            if p[3] > 8 and p[:3] in pal and vivid(p): xs.append(x); ys.append(y)
    if not xs: return Image.new("RGBA", (w, h), (0,0,0,0)), None
    x0, x1 = max(0, min(xs)-grow), min(w-1, max(xs)+grow)
    y0, y1 = max(0, min(ys)-grow), min(h-1, max(ys)+grow)
    out = Image.new("RGBA", (w, h), (0,0,0,0)); dst = out.load()
    for y in range(h):
        for x in range(w):
            p = src[x, y]
            if p[3] > 8 and not (x0 <= x <= x1 and y0 <= y <= y1): dst[x, y] = p
    return out, (x0, y0, x1, y1)


def freq_palette(champ, anims=("idle","run","attack","hit")):
    """색별 **프레임당 최대 등장 픽셀 수**. 팔레트 유무가 아니라 "얼마나 쓰이나"를 본다."""
    import json as _j
    from collections import Counter
    cap = {}
    try:
        A = _j.load(open(os.path.join(VAN, champ + "#anim.fanim"), encoding="utf-8"))["anims"]
    except Exception:
        return cap
    sh = Image.open(os.path.join(VAN, champ + "#sheet.png")).convert("RGBA")
    for an in anims:
        if an not in A: continue
        for f in A[an]["frames"]:
            d = f["data"]; x,y,w,h = int(d["x"]),int(d["y"]),int(d["w"]),int(d["h"])
            c = Counter(p[:3] for p in sh.crop((x,y,x+w,y+h)).getdata() if p[3] > 8)
            for col, k in c.items(): cap[col] = max(cap.get(col, 0), k)
    return cap


def freq_effect(im, cap, ratio=4.0, floor=25):
    """★빈도 기반 — 그 색이 **평소보다 훨씬 많이** 쓰였으면 이펙트로 본다.
    팔레트 차집합이 못 잡는 경우를 위한 것이다: 이펙트가 캐릭터와 **같은 색**을 쓰지만
    양이 압도적으로 많을 때(실측 dual_blader/ult: 흰 검기의 흰색이 idle 에도 있으나
    idle 에선 눈동자 하이라이트 몇 px, 궁에선 수백 px)."""
    from collections import Counter
    w, h = im.size; src = im.load()
    cnt = Counter(src[x, y][:3] for y in range(h) for x in range(w) if src[x, y][3] > 8)
    hot = {col for col, k in cnt.items()
           if k >= floor and k > cap.get(col, 0) * ratio}
    out = Image.new("RGBA", (w, h), (0,0,0,0)); dst = out.load()
    for y in range(h):
        for x in range(w):
            p = src[x, y]
            if p[3] > 8 and p[:3] in hot: dst[x, y] = p
    return out


def blob_effect(im):
    """연결 성분으로 가른다 — **프레임 중앙에 가장 가까운 덩어리 = 캐릭터**, 나머지 = 이펙트.
    팔레트 차집합이 안 통할 때 쓴다(실측 dancer/ult: 흰 초승달이 캐릭터와 **같은 색**이라
    ult 전용 색이 0개였다). 이펙트가 캐릭터와 떨어져 있어야 성립한다."""
    from collections import deque
    w, h = im.size; src = im.load()
    lab = [[-1]*w for _ in range(h)]
    comps = []
    for y0 in range(h):
        for x0 in range(w):
            if src[x0, y0][3] <= 8 or lab[y0][x0] >= 0: continue
            k = len(comps); q = deque([(x0, y0)]); lab[y0][x0] = k
            pts = []
            while q:
                x, y = q.popleft(); pts.append((x, y))
                for dx, dy in ((1,0),(-1,0),(0,1),(0,-1),(1,1),(1,-1),(-1,1),(-1,-1)):
                    nx, ny = x+dx, y+dy
                    if 0 <= nx < w and 0 <= ny < h and lab[ny][nx] < 0 and src[nx, ny][3] > 8:
                        lab[ny][nx] = k; q.append((nx, ny))
            comps.append(pts)
    if not comps: return Image.new("RGBA", (w, h), (0,0,0,0)), 0
    def cdist(pts):
        cx = sum(p[0] for p in pts)/len(pts); cy = sum(p[1] for p in pts)/len(pts)
        return (cx - w/2)**2 + (cy - h/2)**2
    ci = min(range(len(comps)), key=lambda i: cdist(comps[i]))
    out = Image.new("RGBA", (w, h), (0,0,0,0)); dst = out.load()
    npx = 0
    for i, pts in enumerate(comps):
        if i == ci: continue
        for (x, y) in pts: dst[x, y] = src[x, y]; npx += 1
    return out, len(comps)


def effect_mask(im, pal, top=None, outline=False):
    """이펙트 픽셀만 남긴 이미지."""
    w, h = im.size
    out = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    src = im.load(); dst = out.load()
    keep = []
    for y in range(h):
        for x in range(w):
            p = src[x, y]
            if p[3] <= 8: continue
            if top is not None:
                if y < top: dst[x, y] = p; keep.append((x, y))
                continue
            if p[:3] not in pal:
                dst[x, y] = p; keep.append((x, y))
    if outline and top is None and keep:
        ks = set(keep)
        for (x, y) in list(ks):
            for dx, dy in ((1,0),(-1,0),(0,1),(0,-1)):
                nx, ny = x+dx, y+dy
                if 0 <= nx < w and 0 <= ny < h and (nx, ny) not in ks:
                    p = src[nx, ny]
                    if p[3] > 8 and max(p[:3]) < 70:      # 어두운 외곽선만
                        dst[nx, ny] = p
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("champ"); ap.add_argument("anim")
    ap.add_argument("--top", default=None,
        help='위에서 N행을 이펙트로 (팔레트 무시). auto 면 가장 넓은 빈 띠를 찾아 그 위를 이펙트로 본다')
    ap.add_argument("--outline", action="store_true", help="이펙트에 인접한 어두운 외곽선도 포함")
    ap.add_argument("--open", dest="opening", type=int, default=0,
        help="추출한 이펙트에서 **얇은 선을 제거**(모폴로지 열림). 캐릭터 실루엣 테두리를 지울 때")
    ap.add_argument("--outside", action="store_true",
        help="몸통(채도 있는 캐릭터 색의 bbox) 밖을 이펙트로 본다 — 색·빈도가 다 안 통할 때")
    ap.add_argument("--freq", action="store_true",
        help="색의 **등장 빈도**로 판정 — 이펙트가 캐릭터와 같은 색이지만 양이 압도적일 때")
    ap.add_argument("--both", action="store_true",
        help="팔레트 ∪ 연결성분 — 한쪽만으로는 일부 이펙트를 놓칠 때"
             "(실측 dual_blader: 주황 호는 팔레트가, 흰 검기는 연결성분이 잡는다)")
    ap.add_argument("--blob", action="store_true",
                    help="연결 성분으로 가른다(중앙 덩어리=캐릭터). 팔레트가 안 통할 때")
    ap.add_argument("--mirror", action="store_true",
        help="추출한 이펙트의 오른쪽 절반을 좌우대칭해 왼쪽에 덮는다"
             "(캐릭터에 붙어 있던 쪽이 같이 잘려나갔을 때 복원)")
    ap.add_argument("--tip", action="store_true",
        help="이펙트를 **캐릭터의 무기 끝(내용 오른쪽 끝)** 에 붙인다. "
             "원본에서 이펙트가 창끝·화살촉에 달려 있을 때(실측 lancer: 고리 중심 +26.0 = 창끝 +25.5)")
    ap.add_argument("--center", action="store_true",
        help="이펙트를 **내 캐릭터의 중심**에 맞춘다(원본 중앙대비 대신). "
             "이펙트가 캐릭터를 둘러싸는 형태일 때 쓴다")
    ap.add_argument("--write", action="store_true")
    a = ap.parse_args()
    try: sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    except Exception: pass

    van = van_anim(a.champ, a.anim)
    pal = char_palette(a.champ)
    print("[팔레트] %s 캐릭터 색 %d종 (idle/run/attack/hit 에서 수집)" % (a.champ, len(pal)))

    strip_p = os.path.join(DROP, "%s__%s.png" % (a.champ, a.anim))
    if not os.path.exists(strip_p):
        print("✗ 생성물이 없다: %s" % strip_p); return
    st = Image.open(strip_p).convert("RGBA")
    n = len(van); FW = st.width // n; FH = st.height
    mine = [st.crop((i*FW, 0, (i+1)*FW, FH)) for i in range(n)]

    def auto_top(c):
        """가장 넓은 가로 빈 띠를 찾아 그 위쪽을 이펙트로 본다.
        ★"이펙트가 캐릭터 위에 떠 있고 사이가 비어 있는" 형태에 쓴다(dark_mage/ult 등).
        캐릭터가 이펙트에 붙어 있으면 빈 띠가 없으므로 None 을 돌려 아무것도 안 한다."""
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

    # ── 1차: 이펙트를 뽑고 배치 좌표를 구한다(아직 붙이지 않는다)
    effs = []
    for i, c in enumerate(van):
        if a.outside:
            e, bx = outside_effect(c, pal)
        elif a.freq:
            if "capfreq" not in globals(): globals()["capfreq"] = freq_palette(a.champ)
            e = freq_effect(c, globals()["capfreq"])
        elif a.both:
            e1 = effect_mask(c, pal, None, a.outline)
            e2, _ = blob_effect(c)
            e = e1.copy(); p1 = e.load(); p2 = e2.load()
            for yy in range(c.height):
                for xx in range(c.width):
                    if p1[xx, yy][3] <= 8 and p2[xx, yy][3] > 8: p1[xx, yy] = p2[xx, yy]
        elif a.blob:
            e, ncomp = blob_effect(c)
        else:
            tp = auto_top(c) if a.top == "auto" else (int(a.top) if a.top else None)
            e = effect_mask(c, pal, tp, a.outline)
        if a.opening:
            e = morph_open(e, a.opening)
        bb = e.split()[3].getbbox()
        if not bb:
            print("   f%d 이펙트 없음" % i); effs.append(None); continue
        e2 = e.crop(bb)
        if a.mirror:
            # ★오른쪽 절반을 좌우대칭해 왼쪽에 덮는다.
            #   캐릭터와 붙어 있던 쪽이 성분 분리에서 같이 잘려나갔을 때 복원용
            #   (실측 dancer/ult f2: 왼쪽 초승달이 캐릭터에 닿아 함께 제거됐다).
            W2 = e2.width; half = (W2 + 1) // 2
            right = e2.crop((W2 - half, 0, W2, e2.height)).transpose(Image.FLIP_LEFT_RIGHT)
            e2.paste(right, (0, 0), right)
        if a.tip:
            # ★원본에서 "이펙트 중심 − 캐릭터 오른쪽 끝" 관계를 구해, 내 캐릭터의 오른쪽 끝에 같은 관계로 붙인다.
            #   절대 위치(중앙 대비)를 그대로 쓰면 무기 길이가 다른 만큼 어긋난다
            #   (실측 lancer: 사일러스 창끝이 +44.5~+77.5 인데 고리를 +26.0 에 두어 몸통에 얹혔다).
            ch = c.copy(); ep2 = e.load(); cp2 = ch.load()
            for yy in range(c.height):
                for xx in range(c.width):
                    if ep2[xx, yy][3] > 8: cp2[xx, yy] = (0, 0, 0, 0)
            cb = ch.split()[3].getbbox()
            mb2 = mine[i].split()[3].getbbox()
            if cb and mb2:
                dx = ((bb[0]+bb[2])/2) - cb[2]              # 원본: 이펙트중심 − 캐릭터 우끝
                dy = ((bb[1]+bb[3])/2) - ((cb[1]+cb[3])/2)  # 원본: 이펙트중심 − 캐릭터 세로중심
                cx2 = mb2[2] + dx                            # 내 캐릭터 우끝 + 같은 관계
                cy2 = (mb2[1]+mb2[3])/2 + dy
                px = int(round(i*FW + cx2 - e2.width/2)); py = int(round(cy2 - e2.height/2))
                relx = rely = float("nan")
            else:
                relx = bb[0] - c.width/2; rely = bb[1] - c.height/2
                px = int(round(i*FW + FW/2 + relx)); py = int(round(FH/2 + rely))
        elif a.center:
            # 내 프레임의 캐릭터 중심에 이펙트 중심을 맞춘다
            mb = mine[i].split()[3].getbbox()
            if mb:
                mcx = (mb[0] + mb[2]) / 2; mcy = (mb[1] + mb[3]) / 2
            else:
                mcx, mcy = FW/2, FH/2
            px = int(round(i*FW + mcx - e2.width/2)); py = int(round(mcy - e2.height/2))
            relx = rely = float("nan")
        else:
            relx = bb[0] - c.width/2
            rely = bb[1] - c.height/2
            px = int(round(i*FW + FW/2 + relx)); py = int(round(FH/2 + rely))
        effs.append((e2, px - i*FW, py))
        npx = sum(1 for p in e2.getdata() if p[3] > 8)
        print("   f%d 이펙트 %dx%d (%dpx) → 프레임 내 (%d,%d)"
              % (i, e2.width, e2.height, npx, px - i*FW, py))

    # ── 2차: 프레임 밖으로 나가면 **대칭으로** 넓힌다.
    #   대칭이어야 프레임 중앙이 그대로라 캐릭터·이펙트의 중앙 대비 위치가 보존된다
    #   (한쪽만 늘리면 중앙이 이동해 화면에서 전부 어긋난다).
    padx = pady = 0
    for e in effs:
        if not e: continue
        e2, ex, ey = e
        padx = max(padx, -ex, ex + e2.width - FW)
        pady = max(pady, -ey, ey + e2.height - FH)
    padx = max(0, padx); pady = max(0, pady)
    if padx or pady:
        print("[확장] 프레임 %dx%d → %dx%d (이펙트 수용, 좌우·상하 대칭)"
              % (FW, FH, FW + padx*2, FH + pady*2))
        NFW, NFH = FW + padx*2, FH + pady*2
        grown = Image.new("RGBA", (NFW*n, NFH), (0,0,0,0))
        for i in range(n):
            grown.paste(mine[i], (i*NFW + padx, pady), mine[i])
        st, FW, FH = grown, NFW, NFH
        effs = [(e[0], e[1]+padx, e[2]+pady) if e else None for e in effs]
        mine = [st.crop((i*FW, 0, (i+1)*FW, FH)) for i in range(n)]

    out = st.copy()
    for i, e in enumerate(effs):
        if e: out.paste(e[0], (i*FW + e[1], e[2]), e[0])

    # 미리보기 = 원본 / 추출한 이펙트 / 합성본
    Z = 3
    CW = max(max(c.width for c in van), FW); CH = max(max(c.height for c in van), FH)
    img = Image.new("RGB", (140 + (CW*Z+8)*n, 26 + (CH*Z+22)*3), (24,25,32))
    dr = ImageDraw.Draw(img)
    for r, label in enumerate(("원본", "추출 이펙트", "합성")):
        dr.text((8, 20 + r*(CH*Z+22) + CH*Z//2), label, fill=(220,224,234))
    for i in range(n):
        x0 = 140 + i*(CW*Z+8)
        for r, c in enumerate((van[i],
                               (effs[i][0] if effs[i] else Image.new("RGBA",(1,1))),
                               out.crop((i*FW,0,(i+1)*FW,FH)))):
            y0 = 20 + r*(CH*Z+22)
            q = c.resize((c.width*Z, c.height*Z), Image.NEAREST)
            cx = x0+CW*Z//2; cy = y0+CH*Z//2
            img.paste(q, (cx-q.width//2, cy-q.height//2), q)
            dr.rectangle((x0,y0,x0+CW*Z-1,y0+CH*Z-1), outline=(56,60,74))
    pv = os.path.join(PREV, "_graft_%s_%s.png" % (a.champ, a.anim))
    img.save(pv); print("[미리보기] %s" % pv)

    if a.write:
        shutil.copy2(strip_p, strip_p + ".bak_graft")
        out.save(strip_p)
        print("[기록] %s (백업 .bak_graft)" % strip_p)
    else:
        print("\n미리보기만 했다. 적용하려면 --write")


if __name__ == "__main__":
    main()
