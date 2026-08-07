# -*- coding: utf-8 -*-
"""TFM2 UI 시뮬레이터 빌드 - bundle_ui의 .ui/.style + 번들 이미지 에셋을 템플릿에 내장해 단일 HTML 생성.

사용법:
    python build_sim.py [번들추출폴더] [bundle.game_data경로]
기본 = 최신 버전 백업 폴더(아래 BUNDLE_DIR/BUNDLE_DATA).
패치로 .ui가 바뀌면 새 경로를 인자로 주고 재실행하면 된다.

에셋 내장 규칙:
  - .ui/.style이 참조하는 source 경로(svg/png) + 그 "#sheet"(png)/"#data"(sprite_sheet json UV) 페어
  - 폰트 = Roboto Light/Regular/Bold (라틴 - 한글은 시스템 Malgun 폴백; NotoSansKR은 너무 큼)
번들 레코드 포맷 (ANA\tfm2-ui-dsl-reference.md §1, 선두 u32 = 레코드 수):
    [u32 extlen][ext][u32 pathlen][path][u32 bodylen][body]
"""
import base64
import json
import re
import struct
import sys
from pathlib import Path

HERE = Path(__file__).parent
TEMPLATE = HERE / "ui_simulator_template.html"
OUT = HERE / "ui_simulator.html"
BUNDLE_DIR = Path(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\bundle_ui")
BUNDLE_DATA = Path(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\bundle.game_data")

FONTS = {
    "300": "asset/base/font/Roboto/Roboto-Light",
    "400": "asset/base/font/Roboto/Roboto-Regular",
    "700": "asset/base/font/Roboto/Roboto-Bold",
}
EXTRA_ASSETS = [  # 참조 수집에 안 걸려도 항상 내장할 기본 에셋
    "asset/base/sprite/white",
]


def scan(data):
    pos = 4  # 선두 u32 = 레코드 수
    n = len(data)
    while pos + 4 <= n:
        (extlen,) = struct.unpack_from("<I", data, pos)
        if extlen == 0 or extlen > 64:
            break
        pos += 4
        ext = data[pos:pos + extlen].decode("ascii", "replace")
        pos += extlen
        (pathlen,) = struct.unpack_from("<I", data, pos)
        if pathlen == 0 or pathlen > 512:
            break
        pos += 4
        path = data[pos:pos + pathlen].decode("utf-8", "replace")
        pos += pathlen
        (bodylen,) = struct.unpack_from("<I", data, pos)
        pos += 4
        yield ext, path, pos, bodylen
        pos += bodylen


def main():
    bundle = Path(sys.argv[1]) if len(sys.argv) > 1 else BUNDLE_DIR
    bdata_path = Path(sys.argv[2]) if len(sys.argv) > 2 else BUNDLE_DATA
    if not bundle.is_dir():
        sys.exit(f"번들 폴더 없음: {bundle}")

    files = {}
    styles = {}
    for p in sorted(bundle.rglob("*.ui")):
        files[p.relative_to(bundle).as_posix()] = p.read_text(encoding="utf-8-sig")
    for p in sorted(bundle.rglob("*.style")):
        styles[p.relative_to(bundle).as_posix()] = p.read_text(encoding="utf-8-sig")

    # ---------- 이미지 에셋 추출 ----------
    images = {}   # path -> {"t":"svg","d":raw svg 텍스트} | {"t":"png","d":base64}
    sheets = {}   # base path -> {tag: [x,y,w,h] 정규화}
    fonts = {}    # weight -> base64 ttf
    if bdata_path.is_file():
        raw = bdata_path.read_bytes()
        recs = {}
        for ext, path, off, blen in scan(raw):
            recs[path] = (ext, off, blen)

        refs = set(EXTRA_ASSETS)
        for text in list(files.values()) + list(styles.values()):
            for m in re.finditer(r'"(asset/[^"?#]+)"', text):
                refs.add(m.group(1))

        def embed(path):
            if path in images or path not in recs:
                return path in images
            ext, off, blen = recs[path]
            body = raw[off:off + blen]
            if ext == "svg":
                images[path] = {"t": "svg", "d": body.decode("utf-8", "replace")}
            elif ext == "png":
                images[path] = {"t": "png", "d": base64.b64encode(body).decode()}
            else:
                return False
            return True

        for r in sorted(refs):
            direct = embed(r)
            # #sheet/#data 페어 (직접 레코드가 없거나, rect_tag 크롭용으로 존재할 수 있음)
            if (r + "#sheet") in recs:
                if embed(r + "#sheet") and (r + "#data") in recs:
                    ext, off, blen = recs[r + "#data"]
                    try:
                        j = json.loads(raw[off:off + blen].decode("utf-8", "replace"))
                        tags = {k: [v["x"], v["y"], v["w"], v["h"]] for k, v in j.get("images", {}).items()}
                        if tags:
                            sheets[r] = tags
                    except (ValueError, KeyError, TypeError):
                        pass
            if not direct and (r + "#sheet") not in recs:
                pass  # 미존재 참조(스타일 프리셋 #표기 등) - 무시

        for w, fpath in FONTS.items():
            if fpath in recs:
                ext, off, blen = recs[fpath]
                fonts[w] = base64.b64encode(raw[off:off + blen]).decode()
    else:
        print(f"경고: bundle.game_data 없음({bdata_path}) - 이미지 없이 빌드")

    m = re.search(r"tfm2[_ ]?(\d+\.\d+\.\d+)", str(bundle))
    ver = m.group(1) if m else "?"
    version = f"게임 {ver} · .ui {len(files)}개 · 이미지 {len(images)}개 내장"

    data = {"version": version, "files": files, "styles": styles,
            "images": images, "sheets": sheets, "fonts": fonts}
    js = json.dumps(data, ensure_ascii=False, separators=(",", ":")).replace("</", "<\\/")

    tpl = TEMPLATE.read_text(encoding="utf-8")
    marker = "/*__BUNDLE_JSON__*/"
    if marker not in tpl:
        sys.exit("템플릿에 마커 없음")
    OUT.write_text(tpl.replace(marker, js, 1), encoding="utf-8")  # BOM 없는 UTF-8
    print(f"OK: {OUT} ({OUT.stat().st_size:,}B) - .ui {len(files)} / .style {len(styles)} / "
          f"이미지 {len(images)} / 시트 {len(sheets)} / 폰트 {len(fonts)}")


if __name__ == "__main__":
    main()
