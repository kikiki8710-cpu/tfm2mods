# -*- coding: utf-8 -*-
r"""
extract_bundle_all.py — TFM2 bundle.game_data 전량 추출 (버전 무관).

★게임 폴더의 bundle_unpacked_full\ 은 STALE 스냅샷이다(2026-06-14자).
  UI/에셋 참조 작업 전에는 이 스크립트로 현행 번들에서 재추출해서 쓸 것.
  (0.5.1 실측: stale 대비 banpick/layout.ui 68,056B→69,620B 등 광범위 변경.)

번들 레코드 포맷 (순차 파싱):
    [u32 count]
    반복: [u32 typelen][type][u32 pathlen][path][u32 datalen][data]
  - type 예: ui / style / sprite / sound / font / aseprite_resources / setting / text / folder
  - path 예: "asset/base/ui/layout/banpick/layout"
  - 출력 규칙: "asset/base/" 제거 + 확장자로 type 부착
      → ui\layout\banpick\layout.ui

사용법:
    python extract_bundle_all.py <out_dir> [bundle_path] [--no-folder] [--only TYPE1,TYPE2]
      bundle_path 생략 시 게임 설치본 사용.
      --no-folder  : type=="folder" 레코드 저장 안 함(기본은 저장, 원본 구조 재현)
      --only ui,style : 특정 타입만 추출(전량 대신)
예시:
    python extract_bundle_all.py C:\tfm2mods\bundle_unpacked_0.5.1
    python extract_bundle_all.py .\ui_only --only ui,style
"""
import os, sys, struct, io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

DEFAULT_BUNDLE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\bundle.game_data"


def u32(f):
    b = f.read(4)
    if len(b) < 4:
        return None
    return struct.unpack("<I", b)[0]


def extract(bundle_path, out_dir, keep_folder=True, only=None):
    os.makedirs(out_dir, exist_ok=True)
    size = os.path.getsize(bundle_path)
    print("bundle: %s (%d bytes)" % (bundle_path, size))

    by_type = {}
    written = skipped = 0
    with open(bundle_path, "rb") as f:
        count = u32(f)
        n = 0
        while True:
            tl = u32(f)
            if tl is None:
                break
            if tl > 1000:
                print("!! bad typelen %d at %d — stop" % (tl, f.tell() - 4))
                break
            typ = f.read(tl).decode("latin1")
            pl = u32(f)
            if pl is None or pl > 4096:
                print("!! bad pathlen at %d — stop" % (f.tell() - 4))
                break
            path = f.read(pl).decode("utf-8", "replace")
            dl = u32(f)
            if dl is None:
                break
            n += 1

            want = True
            if only and typ not in only:
                want = False
            if typ == "folder" and not keep_folder:
                want = False

            if not want:
                f.seek(dl, 1)
                skipped += 1
                continue

            data = f.read(dl)
            rel = path.replace("asset/base/", "").replace("/", os.sep)
            out = os.path.join(out_dir, rel + "." + typ)
            d = os.path.dirname(out)
            if d:
                os.makedirs(d, exist_ok=True)
            with open(out, "wb") as g:
                g.write(data)
            written += 1
            by_type[typ] = by_type.get(typ, 0) + 1

    print("declared=%s scanned=%d  written=%d  skipped=%d" % (count, n, written, skipped))
    print("--- by type ---")
    for t in sorted(by_type, key=lambda k: -by_type[k]):
        print("  %-22s %d" % (t, by_type[t]))
    print("out: %s" % out_dir)
    return written


def main():
    args = [a for a in sys.argv[1:]]
    keep_folder = True
    only = None
    if "--no-folder" in args:
        keep_folder = False
        args.remove("--no-folder")
    if "--only" in args:
        i = args.index("--only")
        only = set(args[i + 1].split(","))
        del args[i:i + 2]
    if not args:
        print(__doc__)
        sys.exit(1)
    out_dir = args[0]
    bundle = args[1] if len(args) > 1 else DEFAULT_BUNDLE
    if not os.path.exists(bundle):
        print("번들 없음: %s" % bundle)
        sys.exit(1)
    extract(bundle, out_dir, keep_folder, only)


if __name__ == "__main__":
    main()
