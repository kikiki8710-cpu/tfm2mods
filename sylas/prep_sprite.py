# -*- coding: utf-8 -*-
"""
생성 스프라이트 정규화 도구 (pack_sprites.py 의 앞단)
=====================================================
GPT가 준 큰 이미지를 게임이 쓸 수 있는 **균일 스트립**으로 만든다.
성직자 3종을 손으로 처리하며 나온 절차를 그대로 자동화한 것이다.

  원본 큰 이미지  →  ①알파 노이즈 컷  ②프레임 자동 분할  ③배율 산출  ④발 높이 정렬  →  gpt_out\<champ>__<anim>.png

왜 pack 과 분리했나
  pack 은 결정적인 배치·fanim 생성이라 판단이 없다. 이 단계는 "몇 프레임인지, 얼마나 크게,
  얼마나 띄울지"를 정해야 해서 사람 확인이 필요하다. 그래서 항상 프리뷰를 먼저 낸다.

실행
  python prep_sprite.py <원본이미지> <champ> <anim>                 # 프리뷰만
  python prep_sprite.py <원본이미지> <champ> <anim> --write         # gpt_out 에 기록
  주요 옵션
    --frame 85x100        프레임 규격 강제(기본: 원본 fanim 값)
    --pad 24              발밑 여백 고정
    --pad 0,8,16,24       프레임별 여백(점점 떠오르는 연출 등)
    --match ult_idle      이미 만든 다른 애니의 캐릭터 크기에 맞춤(전환이 매끄러워진다)
    --target 74           캐릭터 목표 높이(--match 없을 때)
    --cuts 100,250,400    분할 지점 수동 지정(자동 검출이 틀렸을 때)
    --thr 12              알파 노이즈 임계값

★알파 노이즈 주의
  생성물에는 알파 1~8 짜리 거의 안 보이는 픽셀이 깔려 있다. 이걸 안 지우면
  getbbox() 가 실제 내용의 2배 넘는 영역을 잡고, 그 높이로 축소해서 **캐릭터가 절반이 된다**
  (2026-08-26 실측: priest/ult_heal 조각이 574px 로 잡혔으나 실제 내용은 258px).
"""
import json, os, sys, argparse
from PIL import Image, ImageDraw, ImageFont

MOD  = r"C:\tfm2mods\sylas"
DROP = os.path.join(MOD, "gpt_out")
PREV = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\sylas\gpt_refs"
VAN  = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\bundle_unpacked_full\aseprite_resources\champions"

def dechroma(im, tol=64):
    """★배경을 투명으로. 2단계다.

    ①**마젠타 전역 제거** — 프롬프트로 `#FF00FF` 를 요구하지만 순수색으로 안 나온다
      (실측: `#cb0d91` `#cc18a1` `#c70f8f`). 그래서 고정색이 아니라 "마젠타 계열"로 판정한다.
    ②**테두리 flood-fill** — 모델이 장마다 배경을 다르게 칠한다(실측: 어떤 프레임은 회청색 `#bab9c9`).
      그런 경우를 위해 이미지 가장자리에서 시작해 **비슷한 색이 이어지는 영역**을 지운다.
      캐릭터는 테두리에 안 닿으므로(프롬프트에서 여백을 요구) 안전하다.

    이미 알파가 있는 이미지(생성기가 투명 배경을 준 경우)는 건드리지 않는다.
    """
    a = im.split()[3]
    if a.getextrema()[0] < 250: return im, False
    w, h = im.size; px = im.load(); n = 0

    def magenta(c):
        r, g, b = c[0], c[1], c[2]
        return r > 140 and b > 120 and g < 120 and (r - g) > 55 and (b - g) > 45

    for y in range(h):
        for x in range(w):
            if magenta(px[x, y]):
                px[x, y] = (0, 0, 0, 0); n += 1

    # ── 테두리 flood-fill (남은 불투명 배경 제거)
    from collections import deque
    seeds = []
    for x in range(w):
        for y in (0, h - 1):
            if px[x, y][3] > 0: seeds.append((x, y))
    for y in range(h):
        for x in (0, w - 1):
            if px[x, y][3] > 0: seeds.append((x, y))
    seen = set()
    for sx, sy in seeds:
        if (sx, sy) in seen or px[sx, sy][3] == 0: continue
        base = px[sx, sy][:3]
        q = deque([(sx, sy)])
        while q:
            x, y = q.popleft()
            if (x, y) in seen: continue
            seen.add((x, y))
            c = px[x, y]
            if c[3] == 0: continue
            if abs(c[0]-base[0]) > tol or abs(c[1]-base[1]) > tol or abs(c[2]-base[2]) > tol: continue
            px[x, y] = (0, 0, 0, 0); n += 1
            if x > 0:     q.append((x-1, y))
            if x < w - 1: q.append((x+1, y))
            if y > 0:     q.append((x, y-1))
            if y < h - 1: q.append((x, y+1))
    return im, (n > 0 and ("배경", n))

def clean(im, thr):
    """알파 thr 이하를 완전 투명으로. 부풀려진 bbox 를 막는다."""
    a = im.split()[3].point(lambda v: 0 if v <= thr else v)
    out = im.copy(); out.putalpha(a); return out

def tight(im, thr):
    c = clean(im, thr); bb = c.split()[3].getbbox()
    return c.crop(bb) if bb else c

def segment(im, want, thr, manual=None):
    """세로 빈 띠로 프레임을 나눈다. 부족하면 넓은 조각을 밀도 최소점에서 쪼갠다."""
    w, h = im.size; px = clean(im, thr).load()
    col = [sum(1 for y in range(h) if px[x, y][3] > 0) for x in range(w)]
    if manual:
        cuts = [0] + sorted(manual) + [w]
        return [(cuts[i], cuts[i+1]-1) for i in range(len(cuts)-1)], "수동 지정"
    runs, s = [], None
    for x in range(w):
        if col[x] == 0:
            if s is None: s = x
        else:
            if s is not None: runs.append((s, x-1, x-s)); s = None
    if s is not None: runs.append((s, w-1, w-s))
    gaps = [r for r in runs if r[2] >= 6]
    xs = [x for x, c in enumerate(col) if c > 0]
    if not xs: return [], "내용 없음"
    def build(gs):
        out, prev = [], xs[0]
        for a, b, _ in sorted(gs):
            if a > prev: out.append((prev, a-1))
            prev = b + 1
        if prev <= xs[-1]: out.append((prev, xs[-1]))
        return out
    segs = build(gaps)
    # ★잡티 제거 — 가장 넓은 조각의 15% 미만은 프레임이 아니라 노이즈다
    #   (실측: 크로마키 잔여로 7px 짜리 조각이 프레임으로 잡혀 1x1 프레임이 나왔다)
    if segs:
        big = max(b - a + 1 for a, b in segs)
        segs = [(a, b) for a, b in segs if (b - a + 1) >= big * 0.15]
    # ★조각이 너무 많으면 = 캐릭터와 이펙트 사이 틈까지 경계로 센 것이다
    #   (실측 2026-08-26 archer/ult_end: 화살이 몸에서 떨어져 있어 독립 프레임으로 잡히고
    #    마지막 캐릭터가 잘려나갔다). 틈 넓이 순으로 **가장 넓은 want-1 개**만 경계로 인정한다.
    if len(segs) > want:
        inner = sorted([g for g in gaps if g[0] > xs[0] and g[1] < xs[-1]],
                       key=lambda g: -g[2])[:want-1]
        segs = build(inner)
        note = f"넓은 틈 {want-1}개로 {len(segs)}조각(좁은 틈은 이펙트로 간주)"
    else:
        note = f"빈 띠로 {len(segs)}조각"
    # 부족분은 **가장 넓은 조각**을 밀도 최소점에서 쪼개 채운다
    while len(segs) < want:
        i = max(range(len(segs)), key=lambda k: segs[k][1]-segs[k][0])
        a, b = segs[i]; seg = col[a:b+1]; n = len(seg)
        lo, hi = int(n*0.3), int(n*0.7)
        if hi <= lo: break
        m = min(range(lo, hi+1), key=lambda j: seg[j])
        segs[i:i+1] = [(a, a+m-1), (a+m+1, b)]
        note += f" → {a+m} 에서 분할"
    return segs[:want], note

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("src"); ap.add_argument("champ"); ap.add_argument("anim")
    ap.add_argument("--frame"); ap.add_argument("--pad"); ap.add_argument("--match")
    ap.add_argument("--target", type=int, default=74); ap.add_argument("--cuts")
    ap.add_argument("--footalign", action="store_true",
        help="발 위치로 좌우 정렬(이펙트가 한쪽으로 뻗어도 몸이 안 흔들린다)")
    ap.add_argument("--grow", action="store_true",
        help="폭 상한 때문에 캐릭터가 눌리면 프레임을 넓힌다(이펙트가 옆으로 긴 동작용)")
    ap.add_argument("--thr", type=int, default=12); ap.add_argument("--write", action="store_true")
    ap.add_argument("--equalize", action="store_true",
                    help="프레임마다 캐릭터 크기를 중앙값에 맞춘다(프레임별 생성물은 배율이 제각각이다)")
    a = ap.parse_args()
    try: sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    except Exception: pass

    van = json.load(open(os.path.join(VAN, f"{a.champ}#anim.fanim"), encoding="utf-8"))["anims"][a.anim]
    fr = van["frames"]; n = len(fr)
    d0 = fr[0]["data"]; oW, oH = int(d0["w"]), int(d0["h"])
    FW, FH = (int(v) for v in a.frame.lower().split("x")) if a.frame else (oW, oH)
    print(f"[원본] {a.champ}/{a.anim} : {n}프레임 {oW}x{oH}   →  [출력] {FW}x{FH}")

    im = Image.open(a.src).convert("RGBA")
    im, did = dechroma(im)
    if did:
        _, npx = did           # ⚠npx 로 받는다 — n 은 프레임 수라 덮으면 안 된다(실사고 2026-08-26)
        print(f"[크로마키] 배경 {npx}px 투명화 ({100*npx/(im.size[0]*im.size[1]):.0f}%)")
    cuts = [int(v) for v in a.cuts.split(",")] if a.cuts else None
    segs, note = segment(im, n, a.thr, cuts)
    if len(segs) != n:
        print(f"✗ 프레임 {n}개가 필요한데 {len(segs)}조각만 나왔다 ({note}). --cuts 로 직접 지정하라.")
        return
    print(f"[분할] {note}  →  {[(x,y,y-x+1) for x,y in segs]}")
    ps = [tight(im.crop((s, 0, e+1, im.size[1])), a.thr) for s, e in segs]
    print(f"[조각] {[p.size for p in ps]}   (알파 {a.thr} 이하 제거 후)")

    # ★[--equalize] 프레임마다 따로 생성하면 캐릭터 크기가 장마다 다르다(실측: 3번째만 작음).
    #   내용 높이의 **중앙값**에 맞춰 각 조각을 개별 보정한다. 원본 애니의 의도된 크기 변화까지
    #   뭉갤 수 있으니 **프레임별 생성물에만** 쓴다.
    if a.equalize and len(ps) > 1:
        hs = sorted(p.size[1] for p in ps); med = hs[len(hs)//2]
        fixed = []
        for p in ps:
            k = med / max(1, p.size[1])
            if abs(k - 1.0) < 0.02: fixed.append(p); continue
            fixed.append(p.resize((max(1, round(p.size[0]*k)), med), Image.BOX))
        print(f"[균일화] 내용 높이 {[x.size[1] for x in ps]} → 중앙값 {med} 로 통일")
        ps = fixed

    # 발밑 여백 — 지정 없으면 원본 프레임의 아래 여백을 따른다
    if a.pad:
        v = [int(x) for x in a.pad.split(",")]
        pads = v if len(v) == n else v*n if len(v) == 1 else None
        if pads is None: print(f"✗ --pad 는 1개 또는 {n}개여야 한다"); return
    else:
        sh = Image.open(os.path.join(VAN, f"{a.champ}#sheet.png")).convert("RGBA")
        pads = []
        for f in fr:
            d = f["data"]; x, y, w, h = int(d["x"]), int(d["y"]), int(d["w"]), int(d["h"])
            bb = sh.crop((x, y, x+w, y+h)).split()[3].getbbox()
            pads.append(h - bb[3] if bb else 0)
    print(f"[여백] 발밑 {pads}")

    # 배율 — 기준 애니가 있으면 **마지막 프레임**을 거기에 맞춘다(전환 매끄럽게)
    if a.match:
        ref = Image.open(os.path.join(DROP, f"{a.champ}__{a.match}.png")).convert("RGBA")
        rb = ref.crop((0, 0, FW, FH)).split()[3].getbbox()
        rw, rh = rb[2]-rb[0], rb[3]-rb[1]
        last = ps[-1]; sc = min(rw/last.size[0], rh/last.size[1])
        print(f"[배율] {a.match} 캐릭터 {rw}x{rh} 에 맞춤 → {sc:.4f}")
    else:
        mh = max(p.size[1] for p in ps); mw = max(p.size[0] for p in ps)
        if a.grow:
            # 이펙트가 옆으로 길면 폭 상한이 배율을 지배해 캐릭터가 눌린다
            # (실측 2026-08-26 archer/ult_pre: 33x43 프레임에 내용 20px). 높이 기준으로 정하고 프레임을 넓힌다.
            sc = a.target/mh
            needw, needh = int(mw*sc)+2, int(mh*sc) + max(pads) + 2
            if needw > FW: print(f"[확장] 프레임 폭 {FW} → {needw}"); FW = needw
            if needh > FH: print(f"[확장] 프레임 높이 {FH} → {needh}"); FH = needh
            print(f"[배율] 목표 높이 {a.target} (폭 상한 해제) → {sc:.4f}")
        else:
            sc = min(a.target/mh, (FW-2)/mw)
            print(f"[배율] 목표 높이 {a.target}, 폭 상한 {FW-2} → {sc:.4f}")
            if (FW-2)/mw < a.target/mh * 0.8:
                print("   ⚠폭 상한이 배율을 지배한다 — 캐릭터가 눌린다. --grow 를 고려해라.")


    # ★바닐라의 "프레임 중앙 대비 발 오프셋"을 그대로 베낀다.
    #   게임은 프레임 **중앙**을 기준으로 그리는데, 바닐라는 발을 중앙에 두지 않는다
    #   (실측 2026-08-26 archer/ult_loop: 폭 95/96/96/89 인데 중앙 대비 발이 전부 +12).
    #   발을 중앙에 맞추면 원본보다 12px 왼쪽에 서게 된다.
    def _foot_cx_of(img):
        al = img.split()[3]; bb = al.getbbox()
        if not bb: return img.size[0]/2
        band = max(1, int((bb[3]-bb[1])*0.25))
        fb = al.crop((0, bb[3]-band, img.size[0], bb[3])).getbbox()
        return (fb[0]+fb[2])/2 if fb else (bb[0]+bb[2])/2
    voff = []
    try:
        _vs = Image.open(os.path.join(VAN, f"{a.champ}#sheet.png")).convert("RGBA")
        for f in fr:
            d = f["data"]; x, y, w, h = int(d["x"]), int(d["y"]), int(d["w"]), int(d["h"])
            voff.append(_foot_cx_of(_vs.crop((x, y, x+w, y+h))) - w/2)
    except Exception:
        voff = [0.0]*n
    print("[원본 발오프셋] 중앙 대비 %s" % [round(v,1) for v in voff])

    def foot_cx(q):
        """발(내용 아래 25%)의 좌우 중심. 이펙트는 대개 공중에 떠 있어 발에는 안 걸린다.
        내용 전체를 가운데 맞추면, 이펙트가 한쪽으로 뻗은 프레임에서 몸이 반대로 밀려
        애니메이션 중 캐릭터가 좌우로 흔들린다(2026-08-26 archer/ult_pre 실측)."""
        al = q.split()[3]
        bb = al.getbbox()
        if not bb: return q.size[0]/2
        band = max(1, int((bb[3]-bb[1])*0.25))
        fb = al.crop((0, bb[3]-band, q.size[0], bb[3])).getbbox()
        if not fb: return (bb[0]+bb[2])/2
        return (fb[0]+fb[2])/2

    qs = []
    for p in ps:
        nw, nh = max(1, round(p.size[0]*sc)), max(1, round(p.size[1]*sc))
        qs.append(p.resize((nw, nh), Image.BOX))

    if a.footalign:
        cxs = [foot_cx(q) for q in qs]
        # ★프레임 폭은 "필요한 좌우 여유"에서 나온다.
        #   게임은 프레임 **중앙**을 기준으로 그리므로 발 위치는 중앙 대비 voff 로 고정돼야 한다.
        #   폭이 모자라면 클램프가 걸려 그 프레임만 어긋난다(실측 2026-08-26 archer/ult_pre f2 = -16).
        #   왼쪽 필요분 = cx-voff, 오른쪽 필요분 = (nw-cx)+voff → 둘 중 큰 쪽의 2배가 최소 폭.
        need = 0.0
        for i, q in enumerate(qs):
            need = max(need, cxs[i] - voff[i], (q.size[0] - cxs[i]) + voff[i])
        needw = int(need * 2) + 2
        if a.grow and needw > FW:
            print(f"[확장] 프레임 폭 {FW} → {needw} (원본 발오프셋 유지에 필요한 여유)")
            FW = needw
        print("[정렬] 발 중심 %s → 프레임중앙%+s (원본 오프셋 재현)"
              % ([round(c,1) for c in cxs], [round(v,1) for v in voff]))

    strip = Image.new("RGBA", (FW*n, FH), (0,0,0,0))
    rows = []
    for i, q in enumerate(qs):
        nw, nh = q.size
        if a.footalign:
            x = int(round(i*FW + FW/2 + voff[i] - cxs[i]))
            xc = max(i*FW, min(x, i*FW + FW - nw))   # 프레임 밖으로 나가지 않게
            if xc != x: print(f"   ⚠f{i} 클램프 {x-i*FW} → {xc-i*FW} (폭 부족 — 위치 어긋남)")
            x = xc
        else:
            x = i*FW + (FW-nw)//2
        strip.paste(q, (x, max(0, FH-pads[i]-nh)), q)
        bb = strip.crop((i*FW, 0, (i+1)*FW, FH)).split()[3].getbbox()
        rows.append((i, bb[2]-bb[0], bb[3]-bb[1], bb[1], FH-bb[3], pads[i]))
    for i, w_, h_, top, bot, want in rows:
        flag = "" if bot == want else f"  ⚠목표 {want}"
        print(f"   f{i}: 내용 {w_:3}x{h_:3}  위여백 {top:3}  아래여백 {bot:3}{flag}")

    Z = 4; pv = Image.new("RGB", (strip.width*Z, strip.height*Z+40), (38,40,48))
    q = strip.resize((strip.width*Z, strip.height*Z), Image.NEAREST); pv.paste(q, (0,40), q)
    dr = ImageDraw.Draw(pv)
    try: F = ImageFont.truetype(r"C:\Windows\Fonts\arialbd.ttf", 22)
    except Exception: F = ImageFont.load_default()
    for i in range(n):
        dr.rectangle((i*FW*Z, 40, (i+1)*FW*Z-1, 40+FH*Z-1), outline=(90,95,112))
        dr.text((i*FW*Z+8, 10), f"f{i} pad{pads[i]}", font=F, fill=(232,234,240))
        dr.line((i*FW*Z, 40+(FH-pads[i])*Z, (i+1)*FW*Z-1, 40+(FH-pads[i])*Z), fill=(120,200,140), width=2)
    pp = os.path.join(PREV, f"_preview_{a.champ}_{a.anim}.png"); pv.save(pp)
    print(f"[프리뷰] {pp}")

    if a.write:
        os.makedirs(DROP, exist_ok=True)
        dst = os.path.join(DROP, f"{a.champ}__{a.anim}.png"); strip.save(dst)
        # ★위치를 여기서 확정했으니 pack 은 다시 정렬하면 안 된다(이중 정렬 = 어긋남).
        if a.footalign:
            import json as _js
            mp = os.path.join(DROP, "_placed.json")
            try: cur = _js.load(open(mp, encoding="utf-8"))
            except Exception: cur = []
            key = f"{a.champ}__{a.anim}"
            if key not in cur: cur.append(key)
            _js.dump(sorted(cur), open(mp, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
            print(f"[정렬확정] {key} → pack 정렬 제외 목록에 등록")
        print(f"[기록] {dst}  {strip.size}\n다음: python pack_sprites.py --write --deploy")
    else:
        print("\n프리뷰만 했다. 기록하려면 --write 를 붙여라.")

if __name__ == "__main__":
    main()
