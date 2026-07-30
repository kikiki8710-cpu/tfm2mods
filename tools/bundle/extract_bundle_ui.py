# -*- coding: utf-8 -*-
r"""
extract_bundle_ui.py — TFM2 bundle.game_data 에서 .ui / .style 전량 추출 (버전 무관).

★게임 폴더의 bundle_unpacked_full\ 은 STALE 이다(구버전 스냅샷).
  UI 작업(레이아웃 참조·조각 작성·override) 전에는 반드시 이 스크립트로 현행 번들에서 재추출할 것.
  (0.5.0_3 실측: stale 대비 .ui 11개 추가 · 30개 내용 변경.)

번들 레코드 포맷 (2026-07-15 규명):
    [u32 extlen][ext]["ui"|"style"][u32 pathlen][path "asset/base/..."][u32 bodylen][body 텍스트]

사용법:
    python extract_bundle_ui.py [out_dir]
      out_dir 생략 시 .\bundle_ui_<size>\ 에 추출.
출력:
    out_dir\ui\layout\...\*.ui , out_dir\style\*.style
"""
import os, io, re, sys, struct

BUNDLE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\bundle.game_data"

def extract(bundle_path, out_dir):
    data = open(bundle_path, "rb").read()
    print("bundle: %s (%d bytes)" % (bundle_path, len(data)))

    def rec_at(i, ext):
        """i = ext 문자열 시작 오프셋. 앞 4B=extlen 검증 → (path, body) 또는 None."""
        if i < 4:
            return None
        if struct.unpack_from("<I", data, i - 4)[0] != len(ext):
            return None
        p = i + len(ext)
        if p + 4 > len(data):
            return None
        plen = struct.unpack_from("<I", data, p)[0]
        if not (4 <= plen <= 300):
            return None
        path = data[p + 4: p + 4 + plen]
        if not path.startswith(b"asset/"):
            return None
        q = p + 4 + plen
        if q + 4 > len(data):
            return None
        blen = struct.unpack_from("<I", data, q)[0]
        if not (1 <= blen <= 4_000_000):
            return None
        return path.decode("utf-8", "replace"), data[q + 4: q + 4 + blen]

    found = {}
    for ext in (b"ui", b"style"):
        for m in re.finditer(re.escape(ext), data):
            r = rec_at(m.start(), ext)
            if not r:
                continue
            path, body = r
            try:
                txt = body.decode("utf-8")
            except UnicodeDecodeError:
                continue
            if "{" not in txt or "}" not in txt:      # .ui/.style 은 중괄호 DSL
                continue
            if path not in found or len(txt) > len(found[path][1]):
                found[path] = (ext.decode(), txt)

    n = {"ui": 0, "style": 0}
    for path, (ext, txt) in sorted(found.items()):
        rel = path.replace("asset/base/", "").replace("/", os.sep)
        dst = os.path.join(out_dir, rel + "." + ext)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        io.open(dst, "w", encoding="utf-8", newline="").write(txt)
        n[ext] += 1

    print("extracted:  .ui = %d   .style = %d" % (n["ui"], n["style"]))
    print("out: %s" % out_dir)
    return found

if __name__ == "__main__":
    out = sys.argv[1] if len(sys.argv) > 1 else \
        os.path.join(os.path.dirname(os.path.abspath(__file__)),
                     "bundle_ui_%d" % os.path.getsize(BUNDLE))
    extract(BUNDLE, out)
