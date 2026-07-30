# 전 챔프(base + 워크샵 모드챔프)의 UV 크롭 + 게임 스프라이트 키를 미리 계산 → champ_uv.rs (모드가 include).
#   CHAMP_UV  = 전신(id, u0,v0,u1,v1, 픽셀w,h)  — 챔프정보 전신 이미지.
#   CHAMP_FACE= 얼굴 크롭(id, u0,v0, 너비uv,높이uv) — 정식 레시피(17px 정사각 + face.{x,y}/2, ghidra-re 확정).
#   CHAMP_KEY = 게임 에셋 키(id, "asset/<ns>/aseprite_resources/champions/<id>") — base는 base, 모드챔프는 모드 경로.
#     → 오버레이가 IMG_PREFIX 하드코딩 대신 이 키를 써서 모드챔프 스프라이트도 조회.
import os, json, struct, glob, zlib
try:
    from PIL import Image
    HAVE_PIL = True
except Exception:
    HAVE_PIL = False  # Pillow 없으면 Leef PNG 생성 스킵(키/UV는 그대로)

BASE_DIR = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\bundle_unpacked_full\aseprite_resources\champions"
BASE_FACE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\bundle_unpacked_full\style\champion_view.champion_view"
WORKSHOP = r"C:\Program Files (x86)\Steam\steamapps\workshop\content\3009300"
# 로컬 게임 mods 폴더도 스캔(워크샵 외 로컬 모드챔프 aseprite 대응).
MODS_DIR = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods"
OUT = r"C:\tfm2mods\tfm2_draft_overlay\champ_uv.rs"
# 대시보드 대상: 기본=0.4.14, main.cjs 가 TFM2_DASH_ROOT 로 0.5.0 워크샵 대시보드 지정.
DASH_ROOT = os.environ.get("TFM2_DASH_ROOT") or r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TFM2.gg\resources\app\tfm2_meta_dashboard"
DASH_OUT = os.path.join(DASH_ROOT, "mod_champ_assets.json")
CROP = 17.0
dash_assets = {}  # 대시보드 표시용: 모드챔프 id → {sheet, frame, sheetWidth, sheetHeight}

def png_size(path):
    with open(path, "rb") as f:
        f.seek(16)
        return struct.unpack(">II", f.read(8))

# raw .aseprite(depth=32 RGBA) frame0 렌더 → bbox 크롭한 PIL 이미지. Leef 등 export 시트 없는 챔프용.
def render_aseprite_frame0(path):
    d = open(path, "rb").read()
    fsz, magic, frames, W, H, depth = struct.unpack("<IHHHHH", d[:14])
    if magic != 0xA5E0 or depth != 32:
        return None
    pos, layers, cels = 128, [], []
    for fi in range(frames):
        fstart = pos
        (fbytes,) = struct.unpack("<I", d[pos:pos+4])
        old_n, = struct.unpack("<H", d[pos+6:pos+8])
        new_n, = struct.unpack("<I", d[pos+12:pos+16])
        nchunks = new_n if new_n != 0 else old_n
        cp = pos + 16
        for _ in range(nchunks):
            csz, ctype = struct.unpack("<IH", d[cp:cp+6])
            cdata = d[cp+6:cp+csz]
            if fi == 0 and ctype == 0x2004:      # Layer: bit0=visible
                (flags,) = struct.unpack("<H", cdata[:2])
                layers.append(bool(flags & 1))
            elif fi == 0 and ctype == 0x2005:    # Cel
                li, x, y, op, ct = struct.unpack("<HhhBH", cdata[:9])
                body = cdata[16:]                 # 헤더16B(zindex+reserved 포함) 후 데이터
                if ct in (0, 2):
                    cw, ch = struct.unpack("<HH", body[:4])
                    pix = zlib.decompress(body[4:]) if ct == 2 else body[4:]
                    cels.append((li, x, y, op, cw, ch, pix))
            cp += csz
        pos = fstart + fbytes
    canvas = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    for (li, x, y, op, cw, ch, pix) in sorted(cels, key=lambda c: c[0]):
        if (li < len(layers) and not layers[li]) or len(pix) < cw*ch*4:
            continue
        cel = Image.frombytes("RGBA", (cw, ch), pix[:cw*ch*4])
        if op < 255:
            cel.putalpha(cel.split()[3].point(lambda v: v*op//255))
        layer = Image.new("RGBA", (W, H), (0, 0, 0, 0))
        layer.paste(cel, (x, y))
        canvas = Image.alpha_composite(canvas, layer)
    bbox = canvas.getbbox()
    return canvas.crop(bbox) if bbox else None

MOD_CH_DIR = os.path.join(os.path.dirname(DASH_OUT), "assets", "mod-champions")

def load_faces(path):
    try:
        cv = json.load(open(path, encoding="utf-8")).get("entries", {})
        out = {}
        for k, v in cv.items():
            f = (v or {}).get("face", {}) or {}
            out[k] = (float(f.get("x", 0)), float(f.get("y", 0)))
        return out
    except Exception:
        return {}

def idle_frame0(fanim_path):
    try:
        anim = json.load(open(fanim_path, encoding="utf-8"))
    except Exception:
        return None
    anims = anim.get("anims", {})
    tag = "idle" if "idle" in anims else (next(iter(anims), None))
    if not tag:
        return None
    frames = anims[tag].get("frames", [])
    if not frames:
        return None
    d = frames[0].get("data", {})
    if not all(k in d for k in ("x", "y", "w", "h")):
        return None
    return float(d["x"]), float(d["y"]), float(d["w"]), float(d["h"])

body, face, keys = [], [], []
rt_faces = []  # raw .aseprite 챔프(fanim 없음)용 face 오프셋(id, fx, fy) — 런타임 17px 얼굴 크롭 레시피에 사용.
seen = set()

def add(cid, champ_dir, key, face_map):
    if cid in seen:
        return
    # ⚠ 스프라이트 파일명 = 키 basename(cid와 다를 수 있음: 예 meiling→hong). cid로 찾으면 누락→빈칸.
    sbase = key.rsplit("/", 1)[-1]
    fr = idle_frame0(os.path.join(champ_dir, sbase + "#anim.fanim"))
    if fr is None:
        return
    png = os.path.join(champ_dir, sbase + "#sheet.png")
    if not os.path.exists(png):
        return
    W, H = png_size(png)
    if W <= 0 or H <= 0:
        return
    x, y, w, h = fr
    seen.add(cid)
    body.append((cid, x / W, y / H, (x + w) / W, (y + h) / H, w, h))
    # 얼굴: champion_view face(없으면 머리로 당기는 기본 -(h-17)) + 정식 레시피
    fx, fy = face_map.get(cid, (0.0, -(h - CROP)))
    u0px = x + fx / 2.0 + ((w - CROP) * 0.5 if w > CROP else 0.0)
    v0px = y + fy / 2.0 + ((h - CROP) * 0.5 if h > CROP else 0.0)
    cw, ch = min(w, CROP), min(h, CROP)
    face.append((cid, u0px / W, v0px / H, cw / W, ch / H))
    keys.append((cid, key))
    # 대시보드용: 모드챔프(base 아님)는 assets/mod-champions/<id>.png + idle 프레임0 rect 로 표시.
    if not key.startswith("asset/base/"):
        dash_assets[cid] = {"sheet": "assets/mod-champions/%s.png" % cid,
                            "frame": {"x": x, "y": y, "w": w, "h": h},
                            "sheetWidth": W, "sheetHeight": H}

# 1) base 챔프
base_faces = load_faces(BASE_FACE)
for fn in sorted(os.listdir(BASE_DIR)):
    if fn.endswith("#anim.fanim"):
        cid = fn[:-len("#anim.fanim")]
        add(cid, BASE_DIR, "asset/base/aseprite_resources/champions/" + cid, base_faces)

# 2) 워크샵 모드챔프 (각 팩의 champion/*.data_champion sprite 필드 = 게임 에셋 키)
n_mod = 0
_packs = sorted(glob.glob(os.path.join(WORKSHOP, "*"))) + sorted(glob.glob(os.path.join(MODS_DIR, "*")))
for pack in _packs:
    cdir = os.path.join(pack, "aseprite_resources", "champions")
    ddir = os.path.join(pack, "champion")
    if not (os.path.isdir(cdir) and os.path.isdir(ddir)):
        continue
    pack_faces = load_faces(os.path.join(pack, "style", "champion_view.champion_view"))
    for df in sorted(os.listdir(ddir)):
        if not df.endswith(".data_champion"):
            continue
        cid = df[:-len(".data_champion")]
        try:
            dc = json.load(open(os.path.join(ddir, df), encoding="utf-8"))
        except Exception:
            continue
        key = dc.get("sprite") or dc.get("Sprite")
        if not key or "/aseprite_resources/champions/" not in key:
            continue  # 스프라이트 키 없으면(=base 재사용 등) 스킵
        before = len(seen)
        add(cid, cdir, key, pack_faces)
        if len(seen) > before:
            n_mod += 1
        elif cid not in seen and os.path.exists(os.path.join(cdir, key.rsplit("/", 1)[-1] + ".aseprite")):
            # raw .aseprite(export 시트/fanim 없음) → 키 + face오프셋 등록(인게임: 런타임 #anim 프레임rect).
            seen.add(cid)
            keys.append((cid, key))
            fx, fy = pack_faces.get(cid, (0.0, 0.0))
            rt_faces.append((cid, fx, fy))
            n_mod += 1
            # 대시보드용: .aseprite frame0 렌더 → assets/mod-champions/<cid>.png + dash_assets(전신 썸네일).
            # 이미 있는 png 는 렌더 스킵(크기만 읽어 dash_assets 유지) → "없는거만" 요구 충족.
            try:
                dst = os.path.join(MOD_CH_DIR, cid + ".png")
                cw = ch = None
                if os.path.exists(dst):
                    cw, ch = png_size(dst)
                elif HAVE_PIL:
                    img = render_aseprite_frame0(os.path.join(cdir, key.rsplit("/", 1)[-1] + ".aseprite"))
                    if img is not None:
                        os.makedirs(MOD_CH_DIR, exist_ok=True)
                        img.save(dst)
                        cw, ch = img.size
                if cw and ch:
                    dash_assets[cid] = {"sheet": "assets/mod-champions/%s.png" % cid,
                                        "frame": {"x": 0, "y": 0, "w": cw, "h": ch},
                                        "sheetWidth": cw, "sheetHeight": ch}
            except Exception as e:
                print("WARN aseprite render %s: %s" % (cid, e))

lines = ["// 자동생성 (gen_champ_uv.py) — base + 워크샵 모드챔프 UV + 게임 스프라이트 키.",
         "pub static CHAMP_UV: &[(&str, f32, f32, f32, f32, f32, f32)] = &["]
for cid, u0, v0, u1, v1, w, h in body:
    lines.append(f'    ("{cid}", {u0:.6}f32, {v0:.6}f32, {u1:.6}f32, {v1:.6}f32, {float(w)}f32, {float(h)}f32),')
lines.append("];")
lines.append("pub static CHAMP_FACE: &[(&str, f32, f32, f32, f32)] = &[")
for cid, u0, v0, uw, vh in face:
    lines.append(f'    ("{cid}", {u0:.7}f32, {v0:.7}f32, {uw:.7}f32, {vh:.7}f32),')
lines.append("];")
lines.append("// 게임 에셋 키(#sheet 접미 제외) — 오버레이가 <key>#sheet 로 조회.")
lines.append("pub static CHAMP_KEY: &[(&str, &str)] = &[")
for cid, key in keys:
    lines.append(f'    ("{cid}", "{key}"),')
lines.append("];")
lines.append("// raw .aseprite 챔프 face 오프셋(id, fx, fy) — 런타임 얼굴 크롭(17px 레시피)용. champion_view 기준.")
lines.append("pub static CHAMP_RT_FACE: &[(&str, f32, f32)] = &[")
for cid, fx, fy in rt_faces:
    lines.append(f'    ("{cid}", {float(fx)}f32, {float(fy)}f32),')
lines.append("];")
open(OUT, "w", encoding="utf-8").write("\n".join(lines) + "\n")
try:
    json.dump(dash_assets, open(DASH_OUT, "w", encoding="utf-8"), ensure_ascii=False)
    print("wrote dashboard mod_champ_assets.json: %d champs" % len(dash_assets))
except Exception as e:
    print("WARN dashboard json:", e)
print("generated champ_uv.rs: total=%d (mod champs=%d)" % (len(body), n_mod))
