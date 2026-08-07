# -*- coding: utf-8 -*-
"""TFM2 UI 시뮬레이터 빌드 — bundle_ui의 .ui/.style 전체를 템플릿에 내장해 단일 HTML 생성.

사용법:
    python build_sim.py [번들추출폴더]
기본 번들 = 최신 버전 백업 폴더의 bundle_ui (아래 BUNDLE_DIR).
패치로 .ui가 바뀌면 새 추출 폴더를 인자로 주고 재실행하면 된다.
"""
import json
import sys
import re
from pathlib import Path

HERE = Path(__file__).parent
TEMPLATE = HERE / "ui_simulator_template.html"
OUT = HERE / "ui_simulator.html"
BUNDLE_DIR = Path(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\bundle_ui")


def main():
    bundle = Path(sys.argv[1]) if len(sys.argv) > 1 else BUNDLE_DIR
    if not bundle.is_dir():
        sys.exit(f"번들 폴더 없음: {bundle}")

    files = {}
    styles = {}
    for p in sorted(bundle.rglob("*.ui")):
        key = p.relative_to(bundle).as_posix()
        files[key] = p.read_text(encoding="utf-8-sig")
    for p in sorted(bundle.rglob("*.style")):
        key = p.relative_to(bundle).as_posix()
        styles[key] = p.read_text(encoding="utf-8-sig")

    # 버전 추정: 번들 폴더 경로에서 tfm2_X.Y.Z 패턴
    m = re.search(r"tfm2[_ ]?(\d+\.\d+\.\d+)", str(bundle))
    version = f"게임 {m.group(1)} · .ui {len(files)}개 내장" if m else f".ui {len(files)}개 내장"

    data = {"version": version, "files": files, "styles": styles}
    # </script> 방어: JSON 안의 "</"를 "<\/"로
    js = json.dumps(data, ensure_ascii=False, separators=(",", ":")).replace("</", "<\\/")

    tpl = TEMPLATE.read_text(encoding="utf-8")
    marker = "/*__BUNDLE_JSON__*/"
    if marker not in tpl:
        sys.exit("템플릿에 마커 없음")
    out = tpl.replace(marker, js, 1)
    OUT.write_text(out, encoding="utf-8")  # BOM 없는 UTF-8
    print(f"OK: {OUT} ({OUT.stat().st_size:,}B, .ui {len(files)}개, .style {len(styles)}개, {version})")


if __name__ == "__main__":
    main()
