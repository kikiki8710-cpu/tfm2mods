# -*- coding: utf-8 -*-
"""
Gemini 이미지 모델로 사일러스용 강탈 궁 스프라이트를 생성한다.
=============================================================
`gpt_refs\<champ>__<anim>.png`(레퍼런스 시트)와 같은 이름 `.txt`(프롬프트)를 그대로 보낸다.
결과는 `gpt_out\<champ>__<anim>.png` 로 떨어지고, 이어서 prep_sprite.py 로 정규화하면 된다.

실행
  python gen_sprite.py <champ> <anim>              # 한 장
  python gen_sprite.py --all                       # gpt_refs 전량(이미 있는 건 건너뜀)
  python gen_sprite.py --list                      # 아직 안 만든 것 목록
  옵션
    --model <이름>   기본 gemini-3.1-flash-image (2.5-flash-image=나노바나나, 3-pro-image는 503 잦음)
    --force                         이미 있어도 다시 생성
    --delay 6                       요청 간격(초) — 무료 티어 분당 한도 대비
    --out <dir>                     기본 gpt_out

키는 `.gemini_key` 파일에서 읽는다(리포에 안 올라가게 .gitignore 등록됨).
"""
import base64, io, json, os, sys, time, argparse, urllib.request, urllib.error

MOD  = r"C:\tfm2mods\sylas"
REFS = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\sylas\gpt_refs"
OUT  = os.path.join(MOD, "gpt_out")
KEY  = os.path.join(MOD, ".gemini_key")
LOG  = os.path.join(MOD, "gen_sprite_log.txt")
EP   = "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={key}"

def key():
    with open(KEY, encoding="utf-8") as f: return f.read().strip()

def note(msg):
    print(msg)
    with open(LOG, "a", encoding="utf-8") as f: f.write(msg + "\n")

def pairs():
    """(champ, anim, png, txt) — 프롬프트가 같이 있는 것만"""
    out = []
    for fn in sorted(os.listdir(REFS)):
        if not fn.endswith(".png") or fn.startswith("_"): continue
        stem = fn[:-4]
        txt = os.path.join(REFS, stem + ".txt")
        if not os.path.exists(txt) or "__" not in stem: continue
        champ, anim = stem.split("__", 1)
        mot = os.path.join(REFS, "_motion", fn)
        out.append((champ, anim, mot if os.path.exists(mot) else os.path.join(REFS, fn), txt))
    return out

# ─────────────────────────────────────────────────────────────
# OpenAI 백엔드 (gpt-image-*). Gemini 와 같은 레퍼런스·프롬프트를 그대로 쓴다.
# images/edits 는 이미지 여러 장을 순서대로 받는다 → 1번=캐릭터, 2번=동작.
KEY_OAI = os.path.join(MOD, ".openai_key")
EP_OAI  = "https://api.openai.com/v1/images/edits"

def key_oai():
    with open(KEY_OAI, encoding="utf-8") as f: return f.read().strip()

def _multipart(fields, files):
    """fields=[(name,value)], files=[(name,filename,bytes)] → (content_type, body)"""
    bd = "----tfm2sylas7a3f9c2e"
    buf = io.BytesIO()
    CRLF = chr(13) + chr(10)
    for k, v in fields:
        buf.write(("--" + bd + CRLF
                   + 'Content-Disposition: form-data; name="' + k + '"' + CRLF
                   + CRLF + v + CRLF).encode())
    for k, fn, blob in files:
        buf.write(("--" + bd + CRLF
                   + 'Content-Disposition: form-data; name="' + k + '"; filename="' + fn + '"' + CRLF
                   + "Content-Type: image/png" + CRLF + CRLF).encode())
        buf.write(blob); buf.write(CRLF.encode())
    buf.write(("--" + bd + "--" + CRLF).encode())
    return "multipart/form-data; boundary=" + bd, buf.getvalue()

def gen_openai(model, prompt, img_path, dst, timeout=300, extra=None, size="1536x1024"):
    paths = [os.path.join(REFS, "_char_sylas.png"), img_path] + list(extra or [])
    files = []
    for i, p in enumerate(paths):
        with open(p, "rb") as f:
            files.append(("image[]", "ref%d.png" % (i + 1), f.read()))
    fields = [("model", model), ("prompt", prompt), ("n", "1"), ("size", size)]
    ct, body = _multipart(fields, files)
    req = urllib.request.Request(EP_OAI, data=body,
                                 headers={"Authorization": "Bearer " + key_oai(),
                                          "Content-Type": ct})
    try:
        with urllib.request.urlopen(req, timeout=timeout) as r:
            d = json.load(r)
    except urllib.error.HTTPError as e:
        return False, "HTTP %d: %s" % (e.code, e.read()[:300].decode("utf-8", "replace"))
    except Exception as e:
        return False, "%s: %s" % (type(e).__name__, e)
    dd = (d.get("data") or [{}])[0]
    b64 = dd.get("b64_json")
    if not b64: return False, "이미지 없음: " + json.dumps(d)[:300]
    raw = base64.b64decode(b64)
    with open(dst, "wb") as f: f.write(raw)
    return True, "%dKB" % (len(raw) // 1024)


def gen(model, prompt, img_path, dst, timeout=180, extra=None):
    """모델 이름으로 백엔드를 고른다 — gpt-image-* / chatgpt-image-* = OpenAI, 나머지 = Gemini."""
    if model.startswith("gpt-image") or model.startswith("chatgpt-image"):
        return gen_openai(model, prompt, img_path, dst, max(timeout, 300), extra)
    return gen_gemini(model, prompt, img_path, dst, timeout, extra)


def gen_gemini(model, prompt, img_path, dst, timeout=180, extra=None):
    """extra = 이미 만든 같은 챔프의 다른 동작들(연쇄 생성). 무기·복장 일관성을 위해 함께 보낸다."""
    def att(path):
        with open(path, "rb") as f:
            parts.append({"inline_data": {"mime_type": "image/png",
                                          "data": base64.b64encode(f.read()).decode()}})
    parts = [{"text": prompt}]
    att(os.path.join(REFS, "_char_sylas.png"))   # 1번 = 캐릭터(사일러스)
    att(img_path)                                 # 2번 = 동작(SOURCE)
    for ep in (extra or []): att(ep)              # 3번~ = 같은 챔프의 앞선 결과(일관성)
    body = {
        "contents": [{"role": "user", "parts": parts}],
        # 이미지 모델은 IMAGE 모달리티를 명시해야 이미지를 돌려준다
        "generationConfig": {"responseModalities": ["IMAGE"]},
    }
    req = urllib.request.Request(
        EP.format(model=model, key=key()),
        data=json.dumps(body).encode(),
        headers={"Content-Type": "application/json"},
    )
    try:
        with urllib.request.urlopen(req, timeout=timeout) as r:
            d = json.load(r)
    except urllib.error.HTTPError as e:
        return False, f"HTTP {e.code}: {e.read()[:300].decode('utf-8','replace')}"
    except Exception as e:
        return False, f"{type(e).__name__}: {e}"
    cands = d.get("candidates") or []
    if not cands: return False, f"후보 없음: {json.dumps(d)[:300]}"
    for p in cands[0].get("content", {}).get("parts", []):
        blob = p.get("inline_data") or p.get("inlineData")
        if blob and blob.get("data"):
            raw = base64.b64decode(blob["data"])
            with open(dst, "wb") as f: f.write(raw)
            return True, f"{len(raw)//1024}KB"
    txt = " / ".join(p.get("text","")[:120] for p in cands[0].get("content",{}).get("parts",[]))
    fin = cands[0].get("finishReason")
    return False, f"이미지 없음 (finish={fin}) {txt[:200]}"


PF_PROMPT = """이미지를 첨부했다. 역할이 다르다.

**[1번] = 그릴 캐릭터 "사일러스".** 얼굴·머리 모양·체형·복장·색을 **그대로** 그려라.
   **사일러스의 외형**: 짧은 **은발**, **가면을 쓰고 있다**(얼굴을 가림), 어두운 색 복장에 **붉은 악센트**. 이 세 가지는 반드시 유지해라.
**[2번] = 참고할 자세 하나뿐.** {kname}의 `{anim}` 동작 중 **{idx}번째 자세**({n}개 중).
   ⛔2번의 캐릭터를 그리지 마라. 생김새·머리·복장을 가져오면 틀린 것이다. **자세만** 본다.
{prev_note}
# 할 일
**1번의 사일러스가 2번의 자세를 취한 그림을 딱 1장** 그려라. (동작 맥락: "{desc}")

# 지켜야 할 것
- **머리는 짧은 은발 그대로.** 긴 머리를 만들거나 색을 바꾸지 마라.
- **가면을 반드시 씌워라.** 사일러스는 가면을 쓴 캐릭터다. 맨 얼굴로 그리거나 가면을 벗기지 마라.
  다만 1번에 없는 모자·투구를 새로 씌우지는 마라.
- 2번이 무기를 들었으면 **같은 종류**를 사일러스 색(어두운 톤 + 붉은 악센트)으로 새로 디자인해 들려라.
  2번과 다른 종류의 무기를 만들지 마라. 2번이 맨손이면 맨손이다.
- **캐릭터 1명만.** 여러 컷·여러 포즈·연속 장면을 그리지 마라. 프레임 나누기 금지.
- 캐릭터를 이미지 **가운데**에 두고, 화면 높이의 **약 70%**를 채우게 그려라(프레임마다 같은 비율).
- **배경은 순수 마젠타 `#FF00FF` 단색.** 투명·흰색 금지. 캐릭터가 잘리지 않게 사방에 여백을 둬라.
- **픽셀아트**. 그라데이션·블러 금지, 색 경계가 또렷하게. 글자·테두리 금지.
"""

VAN_DIR = r"C:\Users\jungs\Desktop\claude\tfm2\bundle_unpacked_0826\aseprite_resources\champions"
ATLAS   = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\sylas\vanilla_ult_sprites.json"

def split_motion(champ, anim, nframes, workdir):
    """원본 동작 스트립을 **프레임 단위** 이미지로 쪼갠다(마젠타 배경 + 확대).
    여러 프레임을 한 장에 요구하는 것이 실패의 주원인이라, 한 자세씩만 보여준다."""
    import json as _j
    from PIL import Image
    a = _j.load(open(os.path.join(VAN_DIR, champ + "#anim.fanim"), encoding="utf-8"))["anims"][anim]
    sh = Image.open(os.path.join(VAN_DIR, champ + "#sheet.png")).convert("RGBA")
    out = []
    for i, f in enumerate(a["frames"][:nframes]):
        d = f["data"]; x, y, w, h = int(d["x"]), int(d["y"]), int(d["w"]), int(d["h"])
        cr = sh.crop((x, y, x + w, y + h))
        z = max(1, min(10, round(320 / max(h, 1))))
        q = cr.resize((w * z, h * z), Image.NEAREST)
        pad = 40
        m = Image.new("RGB", (q.width + pad * 2, q.height + pad * 2), (255, 0, 255))
        m.paste(q, (pad, pad), q)
        dst = os.path.join(workdir, f"{champ}__{anim}__src{i}.png"); m.save(dst)
        out.append(dst)
    return out

def run_per_frame(a, todo, note, gen, time):
    """프레임별 생성 → gen_out\\_frames 에 out{i}.png 로 저장 → 스트립으로 합쳐 gen_out 에 둔다."""
    import json as _j
    from PIL import Image
    DESC = {}
    try:
        _d = _j.load(open(ATLAS, encoding="utf-8"))
        DESC = {x["id"]: (x.get("name", x["id"]), x.get("ult", "")) for x in _d["champs"]}
    except Exception:
        pass
    work = os.path.join(a.out, "_frames"); os.makedirs(work, exist_ok=True)
    ok = fail = 0
    for c, an, _png, _txt in todo:
        try:
            fr = _j.load(open(os.path.join(VAN_DIR, c + "#anim.fanim"), encoding="utf-8"))["anims"][an]["frames"]
        except Exception as e:
            note(f"{c}__{an}  FAIL 원본 없음 {e}"); fail += 1; continue
        n = len(fr)
        mots = split_motion(c, an, n, work)
        kname, desc = DESC.get(c, (c, ""))
        outs = []
        for i, mp in enumerate(mots):
            dst = os.path.join(work, f"{c}__{an}__out{i}.png")
            prev = [outs[-1]] if outs else []
            pnote = ("**[3번] = 방금 만든 직전 자세.** 무기·복장·머리·색을 **그것과 똑같이** 유지해라.\n"
                     "   ★**캐릭터를 3번과 같은 크기로** 그려라. 화면에서 차지하는 비율이 같아야 한다.\n"
                     if prev else "")
            pr = PF_PROMPT.format(kname=kname, anim=an, idx=i + 1, n=n,
                                  desc=(desc or "")[:100], prev_note=pnote)
            s2 = False
            for attempt in range(3):
                s2, msg = gen(a.model, pr, mp, dst, extra=prev)
                if s2: break
                if attempt < 2: time.sleep(a.delay)
            note(f"  {c}__{an} f{i+1}/{n}  {'OK ' + msg if s2 else 'FAIL ' + msg}")
            if s2: outs.append(dst)
            time.sleep(a.delay)
        if len(outs) != n:
            note(f"{c}__{an}  FAIL {len(outs)}/{n} 프레임만 생성"); fail += 1; continue
        # 생성된 프레임들을 가로 스트립으로 합친다(폭은 최대치에 맞추고 마젠타로 채움)
        ims = [Image.open(p).convert("RGB") for p in outs]
        W = max(i.width for i in ims); H = max(i.height for i in ims)
        strip = Image.new("RGB", (W * n, H), (255, 0, 255))
        for i, im in enumerate(ims):
            strip.paste(im, (i * W + (W - im.width) // 2, (H - im.height) // 2))
        dstrip = os.path.join(a.out, f"{c}__{an}.png"); strip.save(dstrip)
        note(f"{c}__{an}  OK {n}프레임 합침 → {os.path.basename(dstrip)} {strip.size}")
        ok += 1
    note(f"== 프레임별 완료: 성공 {ok} / 실패 {fail} ==")

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("champ", nargs="?"); ap.add_argument("anim", nargs="?")
    ap.add_argument("--all", action="store_true"); ap.add_argument("--list", action="store_true")
    ap.add_argument("--model", default="gemini-3.1-flash-image")
    ap.add_argument("--force", action="store_true"); ap.add_argument("--delay", type=float, default=6.0)
    ap.add_argument("--out", default=OUT)
    ap.add_argument("--per-frame", dest="perframe", action="store_true",
                    help="프레임을 한 장씩 생성해 합친다(여러 프레임 동시 요구가 실패 원인)")
    ap.add_argument("--chain", action="store_true",
                    help="같은 챔프의 앞선 결과를 참고로 물려줘 무기·복장을 통일한다")
    a = ap.parse_args()
    try: sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    except Exception: pass
    os.makedirs(a.out, exist_ok=True)

    todo = pairs()
    if a.champ: todo = [t for t in todo if t[0] == a.champ and (not a.anim or t[1] == a.anim)]
    if not a.force:
        todo = [t for t in todo if not os.path.exists(os.path.join(a.out, f"{t[0]}__{t[1]}.png"))]

    if a.list:
        print(f"미생성 {len(todo)}건");
        for c, an, _, _ in todo: print(f"  {c}__{an}")
        return
    if not todo: print("생성할 것이 없다(--force 로 재생성)"); return
    if not (a.all or a.champ):
        print(f"미생성 {len(todo)}건. 한 장은 `<champ> <anim>`, 전량은 --all"); return

    if a.perframe:
        run_per_frame(a, todo, note, gen, time); return

    ok = fail = 0
    for i, (c, an, png, txt) in enumerate(todo, 1):
        dst = os.path.join(a.out, f"{c}__{an}.png")
        prompt = open(txt, encoding="utf-8").read()
        # ★연쇄: 같은 챔프에서 이미 만든 것을 참고로 붙인다(활 모양이 장마다 달라지는 것 방지)
        extra = []
        if a.chain:
            for pc, pa, _, _ in todo[:i-1]:
                if pc != c: continue
                q = os.path.join(a.out, f"{pc}__{pa}.png")
                if os.path.exists(q): extra.append(q)
            extra = extra[-2:]                       # 최근 2장이면 충분
            if extra:
                prompt += (
                    "\n# ★일관성 (중요)\n"
                    "추가로 첨부한 이미지들은 **같은 궁극기의 다른 동작**으로 이미 만든 것이다.\n"
                    "무기의 **모양·색·크기**, 복장, 이펙트 색을 **그 이미지들과 똑같이** 유지해라.\n"
                    "무기를 새로 디자인하지 말고 거기 있는 것을 그대로 써라.\n"
                )
        for attempt in range(3):
            s, msg = gen(a.model, prompt, png, dst, extra=extra)
            if s: break
            if attempt < 2: time.sleep(a.delay)
        note(f"[{i}/{len(todo)}] {c}__{an}  {'OK ' + msg if s else 'FAIL(3회) ' + msg}")
        ok += s; fail += (not s)
        if i < len(todo): time.sleep(a.delay)
    note(f"== 완료: 성공 {ok} / 실패 {fail} ==")
    if ok: print("다음: python prep_sprite.py <원본> <champ> <anim> [옵션] --write  →  pack_sprites.py --write --deploy")

if __name__ == "__main__":
    main()
