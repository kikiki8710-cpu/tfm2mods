# -*- coding: utf-8 -*-
"""release_ai_adjust_054.py — tfm2_ai_adjust 0.5.4 릴리스 zip 생성 (/deploy §4 준수).

`release_ai_adjust_053.py` 의 0.5.4 판. 바뀐 점만:
  · 배포처 0.5.3 → **0.5.4**
  · NORM 에 **`class_verify` 추가** — 08-07 에 클래스별 값 검증하려고 켠 진단 스위치다.
    켠 채로 배포하면 남의 게임에서 2초마다 파일을 쓴다.
  · config\\ 를 **화이트리스트**로 좁혔다. 구판은 `.cfg`/`.txt` 를 전부 담았는데, 라이브 config 에
    개인 튜닝본·백업이 35개까지 늘어 그대로면 **개인 파일이 통째로 유출**된다(구 zip 은 4개만 담겨
    있었으니 지금까진 우연히 무사했던 셈).
  · `mod.mod_info` version/last_updated/description 갱신(라이브에도 반영 = 관행).

불변 원칙
  ★배포처 = <게임설치>\\mods\\release\\<게임버전>\\<MOD_ID>.zip  (C:\\tfm2mods\\release 아님)
  ★zip 루트에 <MOD_ID>\\ 한 겹
  ★cfg 는 "라이브 복사"가 아니라 **스테이징 사본에서** 배포 기본값으로 정규화 — 라이브는 무접촉
  ★BOM 없는 UTF-8 유지 / 개인·런타임 산출물 제외
  ★zip 생성 시 entryNameEncoding 인자 금지(한글 파일명 깨짐)
"""
import io, sys, os, re, json, shutil, zipfile
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

MODID = "tfm2_ai_adjust"
GAME = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2"
LIVE = os.path.join(GAME, "mods", MODID)
RELDIR = os.path.join(GAME, "mods", "release", "0.5.4")
STAGE = r"C:\Users\dev\AppData\Local\Temp\tfm2_stage_054"
OUT = os.path.join(RELDIR, MODID + ".zip")

NEW_VER = "1.6.0"
NEW_DATE = "2026-08-07"

# 직전 zip(18엔트리) 구성을 그대로 따른다. `config_editor.hta` 는 mshta 차단으로 동작하지 않는
# 구 편집기지만, 구 zip 에 들어 있던 자산이라 임의로 빼지 않는다(구성 변경은 별도 판단 사항).
FILES = ["mod.mod_info", "tfm2_ai_adjust.cfg", "tfm2_ai_adjust.dll",
         "ui_inject.txt", "설정편집기.exe", "config_editor.hta"]
# ★화이트리스트 — 개인 튜닝본·백업이 섞이지 않게 이름을 못 박는다.
CONFIG_KEEP = {"default.txt", "테스트A.cfg", "테스트B.cfg", "테스트C.cfg"}
DIRS = {
    "config": lambda n: n in CONFIG_KEEP,
    "players": lambda n: n == "_사용법.txt",
    "ui_inject": lambda n: n.lower().endswith(".ui"),
}

# 진단 플래그만 배포 기본값으로. 유저 튜닝값(vis_window·oi_*·d19i_*·_class_ 등)은 관행대로 유지.
NORM = {
    "log": ("0", "프로덕션(진단 OFF)"),
    "mpcap": ("0", "프로덕션(캡처 OFF)"),
    "hang_diag": ("0", "개발자 전용 워치독 — 배포본은 OFF"),
    "adv_prof": ("0", "개발자 전용 프로파일 — 배포본은 OFF"),
    "sp_seen": ("0", "subplan 후퇴 누적 측정 — 배포본은 OFF"),
    "dcap": ("0", "disc18/19 캡처 — 배포본은 OFF"),
    "judge_dump": ("0", "판단 풀덤프 — 배포본은 OFF"),
    "roam_diag": ("0", "후퇴 사유 집계 — 배포본은 OFF"),
    "ct_hunt": ("0", "구성테스트 추적 — 핫패스 파일 IO라 배포본은 OFF"),
    # ★[08-07 신규] 클래스별 값 검증용. 켜면 2초마다 class_verify.txt 를 쓴다 — 배포본은 OFF.
    "class_verify": ("0", "클래스 적용 검증 — 배포본은 OFF(확인할 때만 1)"),
    "class_probe": ("0", "클래스 오프셋 탐색 — 배포본은 OFF"),
    "class_sheet": ("0", "클래스맵 덤프 — 배포본은 OFF"),
    "probe": ("0", "런타임 계측 훅 — 배포본은 OFF"),
}

if os.path.isdir(STAGE):
    shutil.rmtree(STAGE)
root = os.path.join(STAGE, MODID)
os.makedirs(root)

print("── 스테이징 구성 ──")
for f in FILES:
    src = os.path.join(LIVE, f)
    if not os.path.exists(src):
        print("  ⚠ 없음(건너뜀):", f)
        continue
    shutil.copy2(src, os.path.join(root, f))
    print("  + %-24s %10d B" % (f, os.path.getsize(src)))
for sub, keep in DIRS.items():
    s = os.path.join(LIVE, sub)
    if not os.path.isdir(s):
        continue
    os.makedirs(os.path.join(root, sub), exist_ok=True)
    got, skipped = [], 0
    for n in sorted(os.listdir(s)):
        if not os.path.isfile(os.path.join(s, n)):
            continue
        if keep(n):
            shutil.copy2(os.path.join(s, n), os.path.join(root, sub, n))
            got.append(n)
        else:
            skipped += 1
    print("  + %s/ : %d개 담음, %d개 제외(개인·백업)" % (sub, len(got), skipped))
    for n in got:
        print("      %s" % n)

# ── mod_info 갱신(스테이징 + 라이브 둘 다 — 관행상 dll·mod_info 는 라이브 반영) ──
mip = os.path.join(root, "mod.mod_info")
raw = open(mip, "rb").read()
assert raw[:3] != b"\xef\xbb\xbf", "mod_info 에 BOM — 중단"
mi = json.loads(raw.decode("utf-8"))
old_ver = mi["version"]
mi["version"] = NEW_VER
mi["last_updated"] = NEW_DATE
assert mi["author"] == "tfm2mods", "author 가 tfm2mods 가 아니다: %r" % mi["author"]
tag = "⑥클래스(포지션 유형)별 값"
if tag not in mi["description"]:
    mi["description"] = mi["description"].replace(
        "를 조절.", "  %s(전사/원거리/마법사/전투보조/암살자마다 다른 값)를 조절." % tag, 1)
txt = json.dumps(mi, ensure_ascii=False, indent=2) + "\n"
open(mip, "wb").write(txt.encode("utf-8"))
open(os.path.join(LIVE, "mod.mod_info"), "wb").write(txt.encode("utf-8"))
print("\n── mod_info : v%s → v%s · %s · deps %s" %
      (old_ver, NEW_VER, NEW_DATE, mi["dependencies"][0]["version"]))

# ── cfg 정규화(스테이징 사본만 — 라이브 무접촉) ──
cfgp = os.path.join(root, "tfm2_ai_adjust.cfg")
raw = open(cfgp, "rb").read()
assert raw[:3] != b"\xef\xbb\xbf", "라이브 cfg 에 BOM 이 있다 — 중단"
t = raw.decode("utf-8")
print("\n── cfg 정규화(스테이징 사본) ──")
for k, (v, why) in NORM.items():
    pat = re.compile(r'(?m)^(\s*%s\s*=\s*)(\S+)(.*)$' % k)
    m = pat.search(t)
    if not m:
        continue                                  # 없는 키는 추가하지 않는다(기본값이 이미 OFF)
    if m.group(2) != v:
        t = pat.sub(lambda mm: mm.group(1) + v + "   # " + why, t, count=1)
        print("  ~ %-14s %s → %s" % (k, m.group(2), v))
open(cfgp, "wb").write(t.encode("utf-8"))
assert open(cfgp, "rb").read()[:3] != b"\xef\xbb\xbf"
ncls = len([l for l in t.split("\n") if "_class_" in l and not l.lstrip().startswith("#")])
print("  = 클래스별 값 %d줄 유지(유저 튜닝값은 관행대로 보존)" % ncls)

# ── config 프리셋 BOM 제거(스테이징만) ──
cfgdir = os.path.join(root, "config")
for n in sorted(os.listdir(cfgdir)):
    p = os.path.join(cfgdir, n)
    b = open(p, "rb").read()
    if b[:3] == b"\xef\xbb\xbf":
        open(p, "wb").write(b[3:])
        print("  BOM 제거(스테이징): config/%s" % n)

# ── zip (★entryNameEncoding 인자 금지 = 한글 파일명 보존) ──
os.makedirs(RELDIR, exist_ok=True)
if os.path.exists(OUT):
    print("\n  (기존 zip 덮어씀: %d B)" % os.path.getsize(OUT))
with zipfile.ZipFile(OUT, "w", zipfile.ZIP_DEFLATED) as z:
    for dp, _, fns in os.walk(root):
        for n in sorted(fns):
            full = os.path.join(dp, n)
            z.write(full, MODID + "/" + os.path.relpath(full, root).replace("\\", "/"))
print("\n=== zip 생성: %s (%d B) ===" % (OUT, os.path.getsize(OUT)))
with zipfile.ZipFile(OUT) as z:
    for i in z.infolist():
        print("  %-46s %10d" % (i.filename, i.file_size))
    print("  총 %d 엔트리" % len(z.infolist()))
