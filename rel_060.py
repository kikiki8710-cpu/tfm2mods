# -*- coding: utf-8 -*-
# rel_060.py — 0.6.0 stable 모드 릴리스 zip 생성기(라이브 배포처 → release\<GAME_VER>\). 구 rel_one.py 는 "직전 zip 기준" 방식이라
#   0.6.0 처럼 파일 구성이 바뀐 회차엔 못 쓴다 → 모드별 **포함 규칙**을 여기 명시한다(개인값·런타임 파일 유출 방지 = 화이트리스트).
#   사용: python rel_060.py <MOD_ID|bundle:daram2_viewplus> [...]   (인자 없음 = ALL)
#   zip 루트 = <MOD_ID>\ 한 겹(번들은 여러 개). PII 검사 = dll/텍스트에 `C:\Users`·`jungs`·`kikiki8710` 스캔.
import os, sys, io, zipfile, time
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), 'MIG'))
import mig_verify as MV
GAME = os.path.dirname(MV.GAME_EXE)
MODS_DIR = os.path.join(GAME, 'mods')
SRC = r"C:\tfm2mods"
REL = os.path.join(SRC, "release", MV.GAME_VER)
WORKSHOP = os.path.join(os.path.dirname(os.path.dirname(GAME)), 'workshop', 'content', '3009300')  # <steamapps>\workshop
PII = (b"C:\Users", b"jungs", b"kikiki8710")
COMMON = {"mod.mod_info", "mod.override_info", "preview.png"}
# (라이브 폴더, 루트 파일 추가, 포함 하위폴더, 소스에서 가져올 파일)
RULES = {
    "roster_view_plus":         (None, set(), ("text",), ()),
    "coaching_staff_view_plus": (None, set(), ("text",), ()),
    "training_view_plus":       (None, set(), ("text",), ()),
    "recruitment_view_plus":    (None, set(), ("text",), ()),
    "facility_view_plus":       (None, set(), ("text",), ()),
    "tfm2_champ_pos_lock":      (None, {"tfm2_champ_pos_lock.cfg"}, ("text",), ()),
    "tfm2_champion_exclude":    (None, set(), ("text",), ()),
    "tfm2_level_cap":           (None, {"tfm2_level_cap.cfg"}, (), ("README.txt",)),
    "custom_tier_assignment":   (None, set(), ("text",), ()),
    "tfm2_mod_order":           (None, set(), (), ("README_설치안내.txt",)),
    "tfm2_comptest_unlock":     (None, {"comptest_items.cfg"}, (), ()),
    "tfm2_elemental_serpen":    (None, {"README_en.md", "README_ko.md", "serpen_probe.cfg"}, ("config", "s", "text"), ()),  # 0.5.8 zip 구성 동일(28 엔트리)
    "banpick_view_plus":        (os.path.join(WORKSHOP, "3766306566"), set(), ("text", "ui", "asset", "skins"), ()),  # illust(761MB) 는 워크샵 배포분 · zip 제외
}
BUNDLES = {"daram2_viewplus": ["roster_view_plus", "coaching_staff_view_plus", "training_view_plus", "recruitment_view_plus", "facility_view_plus", "custom_tier_assignment"]}
EXC = (".bak", ".log", ".old", ".pdb", ".exp", ".lib")

def collect(m):
    live, extra, subs, from_src = RULES[m]
    d = live or os.path.join(MODS_DIR, m)
    out = []
    for f in sorted(os.listdir(d)):
        p = os.path.join(d, f)
        if os.path.isfile(p) and (f == f"{m}.dll" or f in COMMON or f in extra): out.append((f, p))
    for sub in subs:
        sd = os.path.join(d, sub)
        if not os.path.isdir(sd): continue
        for root, _, fs in os.walk(sd):
            for f in sorted(fs):
                low = f.lower()
                if any(x in low for x in EXC): continue
                p = os.path.join(root, f); out.append((os.path.relpath(p, d).replace("\\", "/"), p))
    for f in from_src:
        p = os.path.join(SRC, m, f)
        if os.path.exists(p): out.append((f, p))
    assert any(r == f"{m}.dll" for r, _ in out), f"{m}: dll 없음 ({d})"
    return d, out

def add(z, m, files, root_prefix=""):
    for rel, p in files:
        data = open(p, "rb").read()
        if rel == "mod.mod_info": assert data[:1] == b"{", f"{m}: mod_info BOM"
        if rel.endswith(".cfg"):
            for line in data.decode("utf-8", "replace").splitlines():
                if line.strip().startswith(("debug=", "log=")) and not line.strip().endswith("=0"): print(f"  ⚠진단 플래그 ON: {m}/{rel}: {line.strip()}")
        for k in PII:
            if k in data: print(f"  ⚠PII: {m}/{rel} ← {k.decode()}")
        z.write(p, f"{root_prefix}{m}/{rel}")
    dll = [p for r, p in files if r == f"{m}.dll"][0]; st = os.stat(dll)
    print(f"  {m}: dll {st.st_size}B {time.strftime('%Y-%m-%d %H:%M:%S', time.localtime(st.st_mtime))} · {len(files)} files")

def main():
    os.makedirs(REL, exist_ok=True)
    bundled = {m for ms in BUNDLES.values() for m in ms}
    targets = sys.argv[1:] or ([m for m in RULES if m not in bundled] + [f"bundle:{b}" for b in BUNDLES])  # 번들 소속은 개별 zip 안 만듦
    for t in targets:
        if t.startswith("bundle:"):
            name = t[7:]; out = os.path.join(REL, f"{name}.zip"); print(f"== {name}.zip")
            with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED, compresslevel=9) as z:
                for m in BUNDLES[name]: _, files = collect(m); add(z, m, files)
        else:
            out = os.path.join(REL, f"{t}.zip"); print(f"== {t}.zip")
            with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED, compresslevel=9) as z:
                _, files = collect(t); add(z, t, files)
        n = len(zipfile.ZipFile(out).infolist())
        print(f"  OUT {out} · {os.path.getsize(out):,}B · {n} entries")

if __name__ == "__main__": main()
