# -*- coding: utf-8 -*-
"""여러 챔피언의 궁 스프라이트를 한 번에 만든다 (생성 → 규격변환 → gpt_out 적재).

한 챔피언의 궁 애니는 두 부류로 갈린다.
  ① **몸이 있는 것**(body=True)  → gpt-image-2 로 사일러스를 새로 그린다
  ② **이펙트뿐**(body=False)     → 바닐라를 그대로 뜬다. 캐릭터가 없으니 새로 그릴 이유가 없다
                                    (프레임 규격이 제각각일 수 있어 **셀 중앙 정렬**로 옮긴다)

패킹·배포는 하지 않는다. 배치가 끝난 뒤 사람이 결과를 보고 `pack_sprites.py --write --deploy`.

실행
  python batch_sprites.py knight lancer ninja ...
  python batch_sprites.py --auto 12        # 아직 안 만든 것 중 앞에서 12종
  옵션 --target 46 (기본) · --skip-gen(이미 oai_out 에 있으면 재생성 안 함)

★prep 은 항상 --fixref 로 돈다: 오프셋을 **내용이 가장 적은 프레임**의 값으로 통일한다.
  이펙트가 섞이면 좌우·상하 오프셋 측정이 오염돼 캐릭터가 프레임마다 밀린다
  (실측 demon = 아래로 처짐 / boomerang_hunter = 왼쪽으로 13px 밀림).
"""
import argparse, json, os, subprocess, sys, time
from PIL import Image

MOD   = r"C:\tfm2mods\sylas"
OAI   = os.path.join(MOD, "oai_out")
DROP  = os.path.join(MOD, "gpt_out")
VAN   = r"C:\Users\jungs\Desktop\claude\tfm2\bundle_unpacked_0826\aseprite_resources\champions"
ATLAS = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\sylas\vanilla_ult_sprites.json"
REFS  = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\sylas\gpt_refs"
FANIM = os.path.join(MOD, "aseprite_resources", "champions", "sylas#anim.fanim")
LOG   = os.path.join(MOD, "batch_sprites_log.txt")


def note(m):
    print(m, flush=True)
    with open(LOG, "a", encoding="utf-8") as f:
        f.write(m + "\n")


def copy_vanilla(champ, anim):
    """이펙트 전용 애니를 바닐라에서 그대로 뜬다. 셀 중앙 정렬로 중앙 기준 기하를 보존."""
    a = json.load(open(os.path.join(VAN, champ + "#anim.fanim"), encoding="utf-8"))["anims"]
    sh = Image.open(os.path.join(VAN, champ + "#sheet.png")).convert("RGBA")
    fr = a[anim]["frames"]
    sizes = [(int(f["data"]["w"]), int(f["data"]["h"])) for f in fr]
    W = max(w for w, _ in sizes); H = max(h for _, h in sizes)
    strip = Image.new("RGBA", (W * len(fr), H), (0, 0, 0, 0))
    for i, f in enumerate(fr):
        d = f["data"]; x, y, w, h = int(d["x"]), int(d["y"]), int(d["w"]), int(d["h"])
        strip.paste(sh.crop((x, y, x + w, y + h)), (i * W + (W - w) // 2, (H - h) // 2))
    strip.save(os.path.join(DROP, "%s__%s.png" % (champ, anim)))
    return len(fr), W, H


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("champs", nargs="*")
    ap.add_argument("--auto", type=int, default=0, help="아직 안 만든 것 중 앞에서 N종")
    ap.add_argument("--target", type=int, default=46)
    ap.add_argument("--skip-gen", dest="skipgen", action="store_true")
    ap.add_argument("--delay", type=float, default=3.0)
    a = ap.parse_args()
    try: sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    except Exception: pass

    D = json.load(open(ATLAS, encoding="utf-8"))
    have = set(json.load(open(FANIM, encoding="utf-8"))["anims"])
    by_id = {c["id"]: c for c in D["champs"]}

    champs = list(a.champs)
    if a.auto:
        for c in D["champs"]:
            if len(champs) >= a.auto: break
            need = [r for r in c["anims"]
                    if r["ult"] and r["body"] and not r.get("legacy")
                    and os.path.exists(os.path.join(REFS, "%s__%s.png" % (c["id"], r["n"])))
                    and ("%s_%s" % (r["n"], c["id"])) not in have]
            if need: champs.append(c["id"])

    note("=== 배치 시작: %d종 %s ===" % (len(champs), champs))
    ok = fail = copied = 0
    for ci, cid in enumerate(champs, 1):
        c = by_id.get(cid)
        if not c:
            note("[%d/%d] %-18s SKIP 아틀라스에 없음" % (ci, len(champs), cid)); continue
        # ★이미 시트에 들어간 것은 건너뛴다 — 안 그러면 검증 끝난 아트를 덮어쓴다
        #   (실사고 2026-08-29: 광전사 ult_dash 가 재생성될 뻔했다)
        gen_anims = [r["n"] for r in c["anims"]
                     if r["ult"] and r["body"] and not r.get("legacy")
                     and os.path.exists(os.path.join(REFS, "%s__%s.png" % (cid, r["n"])))
                     and ("%s_%s" % (r["n"], cid)) not in have]
        eff_anims = [r["n"] for r in c["anims"]
                     if r["ult"] and not r["body"] and not r.get("legacy")
                     and ("%s_%s" % (r["n"], cid)) not in have]
        note("[%d/%d] %-18s 생성 %s / 바닐라복사 %s" % (ci, len(champs), cid, gen_anims, eff_anims))

        for an in eff_anims:
            try:
                n, W, H = copy_vanilla(cid, an)
                note("        · %s__%s  바닐라 %d프레임 %dx%d" % (cid, an, n, W, H)); copied += 1
            except Exception as e:
                note("        ✗ %s__%s 바닐라복사 실패: %s" % (cid, an, e))

        for an in gen_anims:
            src = os.path.join(OAI, "%s__%s.png" % (cid, an))
            attempt = 0
            if not (a.skipgen and os.path.exists(src)):
                r = subprocess.run([sys.executable, os.path.join(MOD, "gen_sprite.py"), cid, an,
                                    "--model", "gpt-image-2", "--force", "--out", OAI],
                                   capture_output=True, text=True, encoding="utf-8", errors="replace")
                line = [l for l in (r.stdout or "").splitlines() if "OK" in l or "FAIL" in l]
                note("        · 생성 %s" % (line[-1] if line else (r.stderr or "")[:120]))
                if not os.path.exists(src):
                    note("        ✗ %s__%s 생성 실패" % (cid, an)); fail += 1; continue
                time.sleep(a.delay)
            r = subprocess.run([sys.executable, os.path.join(MOD, "prep_sprite.py"), src, cid, an,
                                "--equalize", "--grow", "--footalign", "--fixref",
                                "--target", str(a.target), "--write"],
                               capture_output=True, text=True, encoding="utf-8", errors="replace")
            out = r.stdout or ""
            # ★분할이 불균등하면 모델이 프레임을 제대로 안 그린 것 → 최대 2회 재생성
            if "⚠분할 불균등" in out and attempt < 2 and not a.skipgen:
                attempt += 1
                note("        ↻ %s__%s 분할 불균등 → 재생성 %d/2" % (cid, an, attempt))
                try: os.remove(src)
                except Exception: pass
                subprocess.run([sys.executable, os.path.join(MOD, "gen_sprite.py"), cid, an,
                                "--model", "gpt-image-2", "--force", "--out", OAI],
                               capture_output=True, text=True, encoding="utf-8", errors="replace")
                time.sleep(a.delay)
                r = subprocess.run([sys.executable, os.path.join(MOD, "prep_sprite.py"), src, cid, an,
                                    "--equalize", "--grow", "--footalign", "--fixref",
                                "--target", str(a.target), "--write"],
                                   capture_output=True, text=True, encoding="utf-8", errors="replace")
                out = r.stdout or ""
            if "[기록]" in out:
                keep = [l for l in out.splitlines() if l.startswith(("[분할]", "[확장]", "[정렬]", "[기록]"))]
                for l in keep: note("          " + l.strip())
                ok += 1
            else:
                bad = [l for l in out.splitlines() if l.startswith("✗")]
                note("        ✗ %s__%s 변환 실패: %s" % (cid, an, bad[0] if bad else out[-160:]))
                fail += 1
    note("=== 완료: 생성·변환 성공 %d / 실패 %d / 바닐라복사 %d ===" % (ok, fail, copied))
    note("다음: python pack_sprites.py --write --deploy")


if __name__ == "__main__":
    main()
