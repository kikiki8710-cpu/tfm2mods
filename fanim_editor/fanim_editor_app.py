# -*- coding: utf-8 -*-
"""TFM2 fanim 편집기 — pywebview 네이티브 앱.
fanim_editor.html UI를 네이티브 창(WebView2)에 띄우고, 파일 IO를 파이썬 브리지로 처리.
빌드: pyinstaller --onefile --noconsole --name fanim_editor --add-data "fanim_editor.html;." fanim_editor_app.py
"""
import base64
import json
import os
import sys

import webview

CONF_DIR = os.path.join(os.environ.get("APPDATA", os.path.expanduser("~")), "tfm2_fanim_editor")
RECENT_PATH = os.path.join(CONF_DIR, "recent.json")


def load_html():
    # exe 옆에 fanim_editor.html이 있으면 그걸 우선 사용(재빌드 없이 UI 수정 가능)
    if getattr(sys, "frozen", False):
        ext_dir = os.path.dirname(sys.executable)
    else:
        ext_dir = os.path.dirname(os.path.abspath(__file__))
    ext = os.path.join(ext_dir, "fanim_editor.html")
    if os.path.exists(ext):
        with open(ext, "r", encoding="utf-8-sig") as f:
            return f.read()
    bundled = os.path.join(getattr(sys, "_MEIPASS", ext_dir), "fanim_editor.html")
    with open(bundled, "r", encoding="utf-8-sig") as f:
        return f.read()


class Api:
    def _win(self):
        return webview.windows[0]

    # ── 대화상자 ──
    def pick_dir(self):
        r = self._win().create_file_dialog(webview.FOLDER_DIALOG)
        if not r:
            return None
        return r[0] if isinstance(r, (list, tuple)) else r

    def pick_file(self, kind):
        if kind == "fanim":
            types = ("fanim 파일 (*.fanim)", "모든 파일 (*.*)")
        else:
            types = ("PNG 이미지 (*.png)", "모든 파일 (*.*)")
        r = self._win().create_file_dialog(webview.OPEN_DIALOG, file_types=types)
        if not r:
            return None
        return r[0] if isinstance(r, (list, tuple)) else r

    def save_file(self, suggested):
        r = self._win().create_file_dialog(webview.SAVE_DIALOG, save_filename=suggested or "file")
        if not r:
            return None
        return r[0] if isinstance(r, (list, tuple)) else r

    # ── 폴더 스캔 ──
    def scan_fanim(self, root):
        root = os.path.abspath(root)
        out = []
        for dirpath, dirnames, filenames in os.walk(root):
            rel = os.path.relpath(dirpath, root)
            depth = 0 if rel == "." else rel.count(os.sep) + 1
            if depth >= 5:
                dirnames[:] = []
                continue
            for fn in filenames:
                if fn.lower().endswith(".fanim"):
                    ap = os.path.join(dirpath, fn)
                    rp = os.path.relpath(ap, root).replace("\\", "/")
                    out.append({"name": fn, "relPath": rp, "absPath": ap, "absDir": dirpath})
        out.sort(key=lambda x: x["relPath"].lower())
        return out

    # ── 파일 IO ──
    def read_text(self, path):
        with open(path, "r", encoding="utf-8-sig") as f:
            return f.read()

    def read_b64(self, path):
        with open(path, "rb") as f:
            return base64.b64encode(f.read()).decode("ascii")

    def read_b64_if_exists(self, path):
        if not os.path.isfile(path):
            return None
        return self.read_b64(path)

    def write_text(self, path, text):
        # 게임 파서 규칙: BOM 없는 UTF-8
        with open(path, "wb") as f:
            f.write(text.encode("utf-8"))
        return True

    def write_b64(self, path, b64):
        with open(path, "wb") as f:
            f.write(base64.b64decode(b64))
        return True

    # ── 프로젝트(최근 파일) 영속화 ──
    def load_recent(self):
        if not os.path.isfile(RECENT_PATH):
            return []
        try:
            with open(RECENT_PATH, "r", encoding="utf-8") as f:
                data = json.load(f)
            return data if isinstance(data, list) else []
        except Exception:
            return []

    def save_recent(self, items):
        os.makedirs(CONF_DIR, exist_ok=True)
        with open(RECENT_PATH, "w", encoding="utf-8") as f:
            json.dump(items, f, ensure_ascii=False)
        return True


def main():
    webview.create_window(
        "TFM2 fanim 편집기",
        html=load_html(),
        js_api=Api(),
        width=1280,
        height=860,
        min_size=(900, 600),
    )
    webview.start()


if __name__ == "__main__":
    main()
