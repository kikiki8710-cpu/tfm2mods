# -*- coding: utf-8 -*-
# knob_audit.py — 설정편집기(ai_adjust_editor)의 TABS 정의를 파싱해 "지금 쓸 수 있는 노브"를 분류한다.
#   편집기 섹션 헤더의 마커로 판정: §◆/§🆕/§★ = 실작동 / §⛔/§⚠ = 폐기·死레버.
#   교차검증: 코드가 실제로 읽는 키(tune("...") + cfg 핸들러 match 아암)와 대조해
#            "편집기엔 살아있다고 적혀 있는데 코드가 안 읽는" 불일치를 찾아낸다.
import io, sys, re, glob, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

EDITOR = r"C:\tfm2mods\ai_adjust_editor\src\main.rs"
MODSRC = r"C:\tfm2mods\tfm2_ai_adjust\src"
CFG = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ai_adjust\tfm2_ai_adjust.cfg"

QUOTED = re.compile(r'"((?:[^"\\]|\\.)*)"')

s = open(EDITOR, encoding="utf-8").read()
tabs = re.findall(r'Tab\{\s*id:"([a-z0-9_]+)",\s*title:"([^"]*)",\s*keys:&\[(.*?)\],\s*note:', s, re.S)

live, dead, per_tab = [], [], []
for tid, title, keys in tabs:
    items = QUOTED.findall(keys)
    is_dead = False
    L, D = [], []
    for it in items:
        if it.startswith("§"):
            is_dead = ("⛔" in it) or ("⚠" in it)
            continue
        (D if is_dead else L).append(it)
    live += L
    dead += D
    per_tab.append((title, L, D))

# 코드가 실제 읽는 키
code = ""
for f in glob.glob(os.path.join(MODSRC, "*.rs")):
    b = os.path.basename(f)
    if b.startswith("rva_05") or "." in b[:-3]:
        continue
    code += open(f, encoding="utf-8", errors="replace").read()
reads = set(re.findall(r'tune\("([a-z0-9_]+)"', code)) | set(re.findall(r'^\s*"([a-z0-9_]+)"\s*=>', code, re.M))

cfgkeys = set()
for l in open(CFG, encoding="utf-8-sig"):
    m = re.match(r'\s*([a-z0-9_]+)\s*=', l)
    if m:
        cfgkeys.add(m.group(1))

Lset, Dset = dict.fromkeys(live), dict.fromkeys(dead)   # 순서 보존 dedup
print("=" * 76)
print("탭별 노브 (✅=실작동 / ⛔=폐기·死)")
print("=" * 76)
for title, L, D in per_tab:
    print("\n● %s" % title)
    if L:
        print("   ✅ %2d  %s" % (len(L), " ".join(L)))
    if D:
        print("   ⛔ %2d  %s" % (len(D), " ".join(D)))

print()
print("=" * 76)
print("교차검증")
print("=" * 76)
ghost = [k for k in Lset if k not in reads]
print("① 편집기는 '실작동'인데 **코드가 안 읽음** (%d개) — 진짜 죽었거나 키 오타:" % len(ghost))
print("   ", " ".join(ghost) if ghost else "(없음 ✅)")
zombie = [k for k in Dset if k in reads]
print("② 편집기는 '폐기'인데 코드는 읽음 (%d개) — 되살릴 후보:" % len(zombie))
print("   ", " ".join(zombie) if zombie else "(없음)")
notin = [k for k in Lset if k not in cfgkeys]
print("③ 실작동인데 cfg에 아직 줄이 없음 (%d개) — 편집기로 값 넣으면 생성됨(기본값 동작 중):" % len(notin))
print("   ", " ".join(notin) if notin else "(없음)")
orphan = sorted(cfgkeys - set(Lset) - set(Dset))
print("④ cfg에 있는데 편집기에 없음 (%d개):" % len(orphan))
print("   ", " ".join(orphan) if orphan else "(없음)")
print()
print("합계: 실작동 %d · 폐기 %d · cfg 줄 %d" % (len(Lset), len(Dset), len(cfgkeys)))
