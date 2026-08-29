# -*- coding: utf-8 -*-
"""
사일러스 스프라이트 패커
=======================
GPT가 그려 온 스트립 PNG들을 사일러스 시트에 배치하고 `sylas#anim.fanim`을 다시 생성한다.

왜 배치를 미리 안 정해도 되는가
  fanim은 프레임을 **절대좌표(x,y,w,h)** 로 참조한다. 시트와 fanim을 **항상 같이 생성**하면
  배치를 언제든 다시 짜도 좌표가 자동으로 맞는다. 그래서 아트가 다 나온 뒤에 한 번에
  넉넉히 배치하는 게 맞다(유저 판단 2026-08-25).

입력
  gpt_out\<champ>__<anim>.png   가로 스트립. 프레임 N개가 **같은 폭**으로 이어져 있어야 한다.
                                 N은 원본 애니의 프레임 수와 같아야 한다(자동 검증).
  파일명은 gpt_refs\ 의 레퍼런스 시트 이름과 **똑같이** 두면 된다.

출력
  aseprite_resources\champions\sylas#sheet.png   (기존 파일은 .bak_pack_<n> 로 백업)
  aseprite_resources\champions\sylas#anim.fanim  (BOM 없는 UTF-8 — BOM이면 파서가 죽는다)

배치
  밴드(가로 띠) 하나 = 그룹 하나. 그룹 0 = 사일러스 기존 애니, 그 다음은 공여자별로 한 밴드.
  밴드 사이 GUTTER, 프레임 사이 GAP 만큼 띄운다. 나중에 같은 챔프 애니가 늘어도
  그 밴드만 넓어지고 아래 밴드가 밀릴 뿐이라 관리가 쉽다.

실행
  python C:\tfm2mods\sylas\pack_sprites.py            # 미리보기(파일 안 씀)
  python C:\tfm2mods\sylas\pack_sprites.py --write    # 실제 기록
  python C:\tfm2mods\sylas\pack_sprites.py --write --deploy   # 게임 폴더까지 복사
"""
import json, os, sys, shutil, collections, io
# ★알파 이중적용 주의(2026-08-29): **투명 캔버스**에 `paste(im, pos, im)` 를 하면
#   마스크가 알파에도 곱해져 `새α = α²/255`, RGB 도 `rgb*α/255` 로 검게 눌린다.
#   실측 피해: 반투명 연출 17종 213,641px (바닐라 α77 → 23). 불투명 픽셀은 멀쩡해서 오래 안 보였다.
#   ⟹ **빈 캔버스에 배치할 때는 마스크를 주지 말 것.** 기존 내용 위에 합성할 때만 마스크가 옳다.
from PIL import Image

MOD   = r"C:\tfm2mods\sylas"
CH    = os.path.join(MOD, "aseprite_resources", "champions")
SHEET = os.path.join(CH, "sylas#sheet.png")
FANIM = os.path.join(CH, "sylas#anim.fanim")
DROP  = os.path.join(MOD, "gpt_out")
GAME  = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\sylas\aseprite_resources\champions"
# ★신 언팩을 본다(2026-08-29 정정 — prep_sprite 와 동일 결함).
#   구 `bundle_unpacked_full` 에는 `.data_champion` 신규 챔피언(crossbowman·nightmare)과
#   alchemist 의 `ult_projectile` 이 없어, "원본에 그 애니 없음"으로 **조용히 건너뛰었다**.
VAN   = r"C:\Users\jungs\Desktop\claude\tfm2\bundle_unpacked_0826\aseprite_resources\champions"

GAP, GUTTER, MARGIN = 8, 24, 8      # 프레임 간격 / 밴드 간격 / 시트 여백

def load_fanim(p):
    with open(p, encoding="utf-8") as f: return json.load(f)

def frames_of(anim): return anim.get("frames") or []

def cut(strip, n):
    """스트립을 프레임 n개로 등분. 폭이 안 나눠떨어지면 None."""
    w, h = strip.size
    if n <= 0 or w % n: return None
    fw = w // n
    return [strip.crop((i*fw, 0, (i+1)*fw, h)) for i in range(n)]


def align_to(src_img, gen_img):
    """생성 프레임을 원본 프레임의 **내용 위치**에 맞춰 평행이동한다.

    왜: 게임은 프레임 사각형 기준으로 스프라이트를 그린다. 캔버스 크기가 같아도
        캔버스 안 캐릭터 위치가 다르면 그만큼 화면에서 어긋나 보인다.
        실측(2026-08-25 berserker/ult_dash): 원본 bbox 시작 (1,1) vs 생성 (16,20) → 19px 떠 보였다.

    앵커 규칙: 원본의 **위/아래 여백 중 작은 쪽**에 붙인다.
        TFM2 프레임은 내용이 한쪽에 붙고 반대쪽 여백으로 의미를 표현한다
        (예: priest/ult_idle = 위여백 1, 아래여백 42 → 아래 여백이 **부양 높이**).
        아래 기준으로 맞추면 키가 큰 생성물의 머리가 프레임 밖으로 잘린다(실측: -27px).

    ⚠프레임 크기가 원본과 다르면 **일부러 바꾼 것**이므로 정렬하지 않는다
      (예: 부양 높이를 키우려고 85x81 → 85x100 으로 늘린 경우).
    """
    if src_img.size != gen_img.size:
        return gen_img, None                     # None = 정렬 생략(의도적 규격 변경)
    sb, gb = src_img.split()[3].getbbox(), gen_img.split()[3].getbbox()
    if not sb or not gb: return gen_img, (0, 0)
    W, H = src_img.size
    dx = round(((sb[0]+sb[2]) - (gb[0]+gb[2])) / 2)
    dy = (sb[1] - gb[1]) if sb[1] <= (H - sb[3]) else (sb[3] - gb[3])
    # 프레임 밖으로 나가지 않게 클램프
    dx = max(-gb[0], min(W - gb[2], dx))
    dy = max(-gb[1], min(H - gb[3], dy))
    if dx == 0 and dy == 0: return gen_img, (0, 0)
    out = Image.new("RGBA", gen_img.size, (0,0,0,0))
    out.paste(gen_img, (dx, dy))
    return out, (dx, dy)

def main():
    try: sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    except Exception: pass

    write  = "--write"  in sys.argv
    deploy = "--deploy" in sys.argv
    os.makedirs(DROP, exist_ok=True)

    base = load_fanim(FANIM)
    sheet = Image.open(SHEET).convert("RGBA")

    # ── 1. 기존 사일러스 애니 → (이름, [프레임 이미지], [duration])
    groups = collections.OrderedDict()
    cur = []
    for name, v in base["anims"].items():
        fr = frames_of(v)
        if not fr: continue
        imgs, durs = [], []
        for f in fr:
            d = f["data"]; x,y,w,h = int(d["x"]),int(d["y"]),int(d["w"]),int(d["h"])
            imgs.append(sheet.crop((x, y, x+w, y+h))); durs.append(f.get("duration", 0.1))
        cur.append((name, imgs, durs))
    groups["_sylas"] = cur

    # ── 2. gpt_out 의 새 아트 흡수
    added, skipped = [], []
    for fn in sorted(os.listdir(DROP)):
        if not fn.lower().endswith(".png"): continue
        stem = fn[:-4]
        if "__" not in stem: skipped.append((fn, "이름이 <champ>__<anim>.png 형식이 아님")); continue
        champ, anim = stem.split("__", 1)
        # ★★저장 이름 = **항상 `<원본애니>_<공여자>`**(2026-08-28 유저 지시로 규칙 통일).
        #   왜: 평이름으로 저장하면 사일러스 본인 애니를 **조용히 덮어쓴다**
        #   (실사고: 광전사 `ult_dash` 가 사일러스 `ult_dash` 37x38 10프레임을 날렸다).
        #   pack 은 gpt_out 에서 온 이름을 `_sylas` 밴드에서 빼기 때문에 원본이 사라진다.
        #   emit_swap(이미터 큐 스왑)이 `<태그>_<공여자>` 를 찾으므로 접미사가 **정답이자 기본값**이다.
        #   사일러스에 그 이름이 없더라도 예외 없이 붙인다 — 공여자가 늘면 언젠가 겹친다.
        #   `@저장이름` 을 명시하면 그것을 우선한다(특수한 경우용 탈출구).
        if "@" in anim:
            anim, save_as = anim.split("@", 1)
        else:
            save_as = f"{anim}_{champ}"
        # 원본에서 프레임 수·duration 을 가져온다(타이밍을 원본과 맞추기 위해)
        try:
            src = load_fanim(os.path.join(VAN, f"{champ}#anim.fanim"))["anims"][anim]
        except Exception:
            skipped.append((fn, f"원본 {champ}#anim.fanim 에 '{anim}' 없음")); continue
        n = len(frames_of(src))
        strip = Image.open(os.path.join(DROP, fn)).convert("RGBA")
        parts = cut(strip, n)
        if parts is None:
            skipped.append((fn, f"폭 {strip.size[0]}px 가 프레임 {n}개로 안 나눠떨어짐")); continue
        durs = [f.get("duration", 0.1) for f in frames_of(src)]
        # ★원본 프레임 위치에 정렬 (--noalign 으로 끌 수 있다)
        # ★정렬은 **애니 단위로 전부 아니면 전무**.
        #   align_to 는 규격이 같을 때만 움직이는데, 바닐라는 한 애니 안에서도 프레임마다
        #   규격이 다르다(실측 2026-08-26 archer/ult_loop = 95x77 / 96x77 / 96x77 / 89x77).
        #   프레임별로 적용하면 규격이 우연히 일치하는 프레임 하나만 이동해
        #   **그 프레임에서만 캐릭터가 튄다**(유저 관찰: "좌우로 떨린다").
        #   → 대표 프레임에서 이동량을 한 번 구해 **모든 프레임에 같은 값**을 적용한다.
        shifts = []
        # prep_sprite 가 원본 발 위치를 재현해 배치를 확정한 것은 다시 정렬하지 않는다
        _placed = set()
        try:
            import json as _js
            _placed = set(_js.load(open(os.path.join(DROP, "_placed.json"), encoding="utf-8")))
        except Exception:
            pass
        if f"{champ}__{anim}" in _placed:
            shifts = [None] * len(parts)
        elif "--noalign" not in sys.argv:
            vsheet = Image.open(os.path.join(VAN, f"{champ}#sheet.png")).convert("RGBA")
            vfr = frames_of(src)
            hit = None
            for i, f in enumerate(vfr):
                d = f["data"]; x,y,w,h = int(d["x"]),int(d["y"]),int(d["w"]),int(d["h"])
                if parts[i].size == (w, h):
                    _, sft = align_to(vsheet.crop((x, y, x+w, y+h)), parts[i])
                    if sft is not None: hit = sft; break
            if hit and hit != (0, 0):
                dx, dy = hit
                for i in range(len(parts)):
                    out = Image.new("RGBA", parts[i].size, (0,0,0,0))
                    out.paste(parts[i], (dx, dy))
                    parts[i] = out
                shifts = [hit] * len(parts)
            else:
                shifts = [hit if hit else None] * len(parts)
        groups.setdefault(champ, []).append((save_as, parts, durs))
        added.append((champ, anim if save_as==anim else f"{anim}→{save_as}", n, strip.size, shifts))

    # gpt_out 로 들어온 이름은 기존 _sylas 밴드에서 뺀다(같은 이름이 두 밴드에 중복되는 것 방지).
    # 그대로 두면 시트에 옛 버전이 남아 자리만 먹고, 밴드 높이도 불필요하게 커진다.
    newnames = {a for _, items in groups.items() if _ != "_sylas" for a, _, _ in items}
    if newnames:
        groups["_sylas"] = [t for t in groups["_sylas"] if t[0] not in newnames]

    # ── 3. 밴드 배치
    out_anims, y = collections.OrderedDict(), MARGIN
    width = 0
    plan = []
    for gname, items in groups.items():
        if not items: continue
        bh = max(im.size[1] for _, imgs, _ in items for im in imgs)
        x = MARGIN
        for name, imgs, durs in items:
            fr = []
            for im, du in zip(imgs, durs):
                fr.append({"duration": du,
                           "data": {"x": float(x), "y": float(y),
                                    "w": float(im.size[0]), "h": float(im.size[1])}})
                plan.append((im, x, y)); x += im.size[0] + GAP
            out_anims[name] = {"frames": fr}
        width = max(width, x - GAP + MARGIN)
        plan.append((None, gname, (y, bh)))          # 밴드 기록(로그용)
        y += bh + GUTTER

    height = y - GUTTER + MARGIN
    print(f"[배치] 시트 {width} x {height}  /  밴드 {len(groups)}개  /  애니 {len(out_anims)}개")
    for p in plan:
        if p[0] is None:
            g,(by,bh) = p[1], p[2]
            cnt = len(groups[g])
            print(f"   밴드 {g:18} y={by:5} h={bh:4}  애니 {cnt}개")
    if added:
        print("[추가]")
        for c,a,n,sz,sf in added:
            if any(t is None for t in sf): mv = "정렬 생략(프레임 규격을 일부러 바꿈)"
            elif any(x or y for x,y in sf): mv = " ".join(f"({x:+d},{y:+d})" for x,y in sf)
            else: mv = "정렬 불필요"
            print(f"   {c}__{a}  {n}프레임  {sz[0]}x{sz[1]}  이동 {mv}")
    if skipped:
        print("[건너뜀]")
        for fn,why in skipped: print(f"   {fn}  : {why}")
    if not added:
        print("   (gpt_out 에 새 아트가 없다. 지금은 기존 애니만 재배치하는 셈)")

    if not write:
        print("\n미리보기만 했다. 실제로 쓰려면 --write 를 붙여라.")
        return

    # ── 4. 기록
    out = Image.new("RGBA", (width, height), (0,0,0,0))
    for im, x, y2 in plan:
        if im is None: continue
        out.paste(im, (x, y2))
    n = 0
    while os.path.exists(SHEET + f".bak_pack_{n}"): n += 1
    shutil.copy2(SHEET, SHEET + f".bak_pack_{n}"); shutil.copy2(FANIM, FANIM + f".bak_pack_{n}")
    out.save(SHEET, "PNG", optimize=True)

    doc = dict(base); doc["anims"] = out_anims
    txt = json.dumps(doc, ensure_ascii=False, indent=1)
    with open(FANIM, "w", encoding="utf-8", newline="\r\n") as f: f.write(txt)
    with open(FANIM, "rb") as f:
        assert f.read(1) == b"{", "BOM 이 생겼다 — 게임 파서가 죽는다"
    json.load(open(FANIM, encoding="utf-8"))          # 파싱 재검증
    print(f"\n기록 완료: {SHEET} / {FANIM}  (백업 .bak_pack_{n})")

    if deploy:
        for p in (SHEET, FANIM):
            shutil.copy2(p, os.path.join(GAME, os.path.basename(p)))
        print(f"게임 폴더 배포 완료: {GAME}")

if __name__ == "__main__":
    main()
