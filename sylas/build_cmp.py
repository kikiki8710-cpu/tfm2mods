# -*- coding: utf-8 -*-
"""궁 이식표(원본 대조 HTML)를 **현재 시트 기준으로 다시** 만든다.

왜 스크립트로 남기나
  2026-08-29 이전엔 즉석 코드로 만들어 놓고 저장을 안 해서, 스프라이트를 고칠 때마다
  표를 다시 만들 방법이 없었다. 표는 아트를 고칠 때마다 낡으므로 **재생성이 일상**이다.

입력
  aseprite_resources\champions\sylas#anim.fanim + #sheet.png   내 현재 아트(정본)
  bundle_unpacked_0826\aseprite_resources\champions\<champ>#*  바닐라 원본
  bundle_unpacked_0826\text\champion.i18n                      한글명·궁 설명
  mods_report\sylas\_cmp_data.json                             ★기존 kind(gen/copy) 분류를 승계
  mods_report\sylas\_cmp_template.html                         `__DATA__` 를 치환할 껍데기

출력
  mods_report\sylas\_cmp_data.json      (기존 파일은 .bak_<n> 로 백업)
  mods_report\sylas\궁이식-원본대조.html

실행
  python C:\tfm2mods\sylas\build_cmp.py
"""
import base64, io, json, os, re, shutil, sys
from PIL import Image

MOD  = r"C:\tfm2mods\sylas"
CH   = os.path.join(MOD, "aseprite_resources", "champions")
BUN  = r"C:\Users\jungs\Desktop\claude\tfm2\bundle_unpacked_0826"
VAN  = os.path.join(BUN, "aseprite_resources", "champions")
I18N = os.path.join(BUN, "text", "champion.i18n")
REP  = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\sylas"
DATA = os.path.join(REP, "_cmp_data.json")
TMPL = os.path.join(REP, "_cmp_template.html")
OUT  = os.path.join(REP, "궁이식-원본대조.html")


def load(p):
    with io.open(p, encoding="utf-8-sig") as f: return json.load(f)


def b64(im):
    b = io.BytesIO(); im.save(b, "PNG", optimize=True)
    return base64.b64encode(b.getvalue()).decode("ascii")


def frames(fanim, sheet, anim):
    """anim 의 프레임들을 [{png,w,h,d}] 로."""
    out = []
    for f in fanim["anims"][anim]["frames"]:
        g = f["data"]; x, y, w, h = int(g["x"]), int(g["y"]), int(g["w"]), int(g["h"])
        out.append({"png": b64(sheet.crop((x, y, x + w, y + h))),
                    "w": w, "h": h, "d": f.get("duration", 0.1)})
    return out


def main():
    try: sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    except Exception: pass

    myd = load(os.path.join(CH, "sylas#anim.fanim"))
    mys = Image.open(os.path.join(CH, "sylas#sheet.png")).convert("RGBA")
    ko  = load(I18N)["ko"]["description"]
    champs = sorted({f.split("#")[0] for f in os.listdir(VAN) if f.endswith("#anim.fanim")},
                    key=len, reverse=True)          # 긴 이름 우선(=접미사 매칭 모호성 제거)
    old = {r["key"]: r for r in load(DATA)["rows"]} if os.path.exists(DATA) else {}

    cache = {}
    def van(c):
        if c not in cache:
            try:
                cache[c] = (load(os.path.join(VAN, c + "#anim.fanim")),
                            Image.open(os.path.join(VAN, c + "#sheet.png")).convert("RGBA"))
            except Exception:
                cache[c] = None
        return cache[c]

    rows, skipped = [], []
    for name in myd["anims"]:
        # `<원본태그>_<공여자>` 규칙(CLAUDE.md·REPORT §27). 공여자 이름이 접미사로 붙는다.
        donor = next((c for c in champs if name.endswith("_" + c)), None)
        if not donor:
            continue                                 # 사일러스 본인 애니
        tag = name[: -(len(donor) + 1)]
        v = van(donor)
        if not v or tag not in v[0]["anims"]:
            skipped.append((name, "바닐라 %s 에 '%s' 없음" % (donor, tag))); continue
        vd, vs = v
        o = old.get(name, {})
        # ★kind 는 기존 분류를 승계한다. 아트를 고쳐도 "바닐라 복사/생성" 이라는 성격은 안 바뀐다.
        kind = o.get("kind")
        if kind is None:
            same = (len(vd["anims"][tag]["frames"]) == len(myd["anims"][name]["frames"]))
            kind = "copy" if same else "gen"
        rows.append({"key": name, "donor": donor, "tag": tag,
                     "kname": ko.get(donor, {}).get("name", donor) if isinstance(ko.get(donor), dict) else donor,
                     "ult": re.sub(r"<[^>]*>", "", (ko.get(donor) or {}).get("ult", "")) if isinstance(ko.get(donor), dict) else "",
                     "kind": kind,
                     "van": frames(vd, vs, tag),
                     "sy":  frames(myd, mys, name)})
    rows.sort(key=lambda r: (r["donor"], r["tag"]))
    stat = {"done": len(rows), "gen": sum(r["kind"] == "gen" for r in rows),
            "copy": sum(r["kind"] == "copy" for r in rows), "todo": len(skipped),
            "champs": len({r["donor"] for r in rows})}
    data = {"rows": rows, "todo": [s[0] for s in skipped], "stat": stat}

    if os.path.exists(DATA):
        n = 0
        while os.path.exists(DATA + ".bak_%d" % n): n += 1
        shutil.copy2(DATA, DATA + ".bak_%d" % n)
    with io.open(DATA, "w", encoding="utf-8", newline="\n") as f:
        json.dump(data, f, ensure_ascii=False)
    with io.open(TMPL, encoding="utf-8") as f: html = f.read()
    with io.open(OUT, "w", encoding="utf-8", newline="\n") as f:
        f.write(html.replace("__DATA__", json.dumps(data, ensure_ascii=False)))

    print("애니 %d종 / 챔프 %d명  (생성 %d · 바닐라복사 %d)" %
          (stat["done"], stat["champs"], stat["gen"], stat["copy"]))
    if skipped:
        print("제외 %d건:" % len(skipped))
        for n_, w in skipped: print("   %-34s %s" % (n_, w))
    print("[기록] %s  (%.1f MB)" % (OUT, os.path.getsize(OUT) / 1048576))


if __name__ == "__main__":
    main()
