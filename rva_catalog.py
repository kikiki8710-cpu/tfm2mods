# -*- coding: utf-8 -*-
# rva_catalog.py — 전 모드 하드코딩 RVA를 한 번에 수집해 카탈로그화.
#   목적: 패치마다 모드별 세션이 각자 주소찾기를 반복하는 낭비 제거.
#         한 번 수집 → migrate_rva.py 로 일괄 재탐색 → 결과표를 각 세션이 참조.
# 사용: python rva_catalog.py            (out: _rva_catalog.json / _rva_catalog.md / _rva_targets.py)
import os, re, json, io, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

MODS = r"C:\tfm2mods"
# fog_damage_fix = 게임측 수정으로 대응 제외(유저 지시 2026-07-29). crm/Spectator_Chat/meta_item_delegate = RVA 0.
TARGET_MODS = [
    "tfm2_ai_adjust", "tfm2_item_tactics", "tfm2_banpick_illust", "tfm2_draft_overlay",
    "tfm2_elemental_serpen", "tfm2_comptest_unlock", "tfm2_banpick_order",
    "tfm2_transfer_tweak", "tfm2_level_cap",
]
SKIP_DIRS = ("target", "_bak", "_archive", "backup", "node_modules")
# 구버전 상수 파일 = 이력용. 현행은 rva_052.rs (CURRENT.md: ai_adjust RVA 단일수정점).
SKIP_FILES = ("rva_051.rs", "rva_050.rs", "rva_0413.rs")
LO, HI = 0x100000, 0x4000000          # RVA 유효범위 (image base 0x140000000 기준 상대)

RE_CONST = re.compile(
    r'^[ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?(?:const|static)[ \t]+([A-Za-z_][A-Za-z0-9_]*)[ \t]*:[ \t]*'
    r'(usize|u64|u32|isize)[ \t]*=[ \t]*(0x[0-9a-fA-F]+)', re.M)
RE_FIELD = re.compile(r'\brva[ \t]*:[ \t]*(0x[0-9a-fA-F]+)')          # Patch { rva: 0x... }
RE_ARRAY = re.compile(r'^[ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?(?:const|static)[ \t]+([A-Za-z_][A-Za-z0-9_]*)'
                      r'[ \t]*:[ \t]*\[[^\]]*\][ \t]*=[ \t]*\[', re.M)

def line_of(src, pos):
    return src[:pos].count("\n") + 1

def comment_near(lines, idx):
    """같은 줄 뒤 주석 → 없으면 직전 주석줄들을 위로 훑어 첫 의미줄."""
    ln = lines[idx - 1] if 0 < idx <= len(lines) else ""
    m = re.search(r'//[ \t]*(.+)$', ln)
    if m and not ln.lstrip().startswith("//"):
        return m.group(1).strip()
    for j in range(idx - 2, max(idx - 6, -1), -1):
        s = lines[j].strip()
        if s.startswith("//"):
            t = s.lstrip("/").strip()
            if t:
                return t
        elif s:
            break
    return ""

def scan_mod(mod):
    root = os.path.join(MODS, mod)
    out = []
    for dp, dns, fns in os.walk(root):
        dns[:] = [d for d in dns if not any(s in d.lower() for s in SKIP_DIRS)]
        for fn in fns:
            if not fn.endswith(".rs") or fn in SKIP_FILES:
                continue
            p = os.path.join(dp, fn)
            rel = os.path.relpath(p, root).replace("\\", "/")
            try:
                src = open(p, encoding="utf-8", errors="replace").read()
            except OSError:
                continue
            lines = src.split("\n")
            seen = set()
            # 1) const/static 스칼라
            for m in RE_CONST.finditer(src):
                v = int(m.group(3), 16)
                if not (LO <= v < HI):
                    continue
                ln = line_of(src, m.start())
                out.append(dict(mod=mod, name=m.group(1), rva=m.group(3), file=rel, line=ln,
                                kind="const", note=comment_near(lines, ln)))
                seen.add((rel, ln))
            # 2) Patch { rva: 0x.. } 형태
            for m in RE_FIELD.finditer(src):
                v = int(m.group(1), 16)
                if not (LO <= v < HI):
                    continue
                ln = line_of(src, m.start())
                if (rel, ln) in seen:
                    continue
                ctx = lines[ln - 1] if ln <= len(lines) else ""
                nm = re.search(r'name[ \t]*:[ \t]*"([^"]+)"', ctx) or \
                     re.search(r'name[ \t]*:[ \t]*"([^"]+)"', "\n".join(lines[max(0, ln - 2):ln + 1]))
                out.append(dict(mod=mod, name=nm.group(1) if nm else f"(patch@{rel}:{ln})",
                                rva=m.group(1), file=rel, line=ln,
                                kind="patch_site", note=comment_near(lines, ln)))
                seen.add((rel, ln))
            # 3) 배열 상수 내부의 RVA 값
            for m in RE_ARRAY.finditer(src):
                start = m.end()
                depth, i = 1, start
                while i < len(src) and depth:
                    if src[i] == "[":
                        depth += 1
                    elif src[i] == "]":
                        depth -= 1
                    i += 1
                body = src[start:i]
                vals = [x for x in re.findall(r'0x[0-9a-fA-F]+', body) if LO <= int(x, 16) < HI]
                if not vals:
                    continue
                ln = line_of(src, m.start())
                out.append(dict(mod=mod, name=m.group(1), rva=",".join(vals), file=rel, line=ln,
                                kind=f"array[{len(vals)}]", note=comment_near(lines, ln)))
                for j in range(len(lines[ln - 1:])):
                    if line_of(src, i) <= ln + j:
                        break
                    seen.add((rel, ln + j))
            # 4) 함수 본문 인라인 리터럴 (byte-patch 사이트 등) — 코드 라인만, 주석 제외
            for idx, ln_text in enumerate(lines, start=1):
                if (rel, idx) in seen:
                    continue
                code = ln_text.split("//")[0]
                if not code.strip() or code.lstrip().startswith(("//", "*", "///")):
                    continue
                for hx in re.findall(r'0x[0-9a-fA-F]{6,8}\b', code):
                    if LO <= int(hx, 16) < HI:
                        out.append(dict(mod=mod, name="(inline)", rva=hx, file=rel, line=idx,
                                        kind="inline", note=code.strip()[:90]))
    return out

allrows = []
for mod in TARGET_MODS:
    if not os.path.isdir(os.path.join(MODS, mod)):
        print(f"!! 폴더 없음: {mod}")
        continue
    rows = scan_mod(mod)
    allrows += rows
    n_site = sum(len(r["rva"].split(",")) for r in rows)
    print(f"{mod:<26} 선언 {len(rows):>4}건 / 주소 {n_site:>4}개")

n_site = sum(len(r["rva"].split(",")) for r in allrows)
print(f"{'TOTAL':<26} 선언 {len(allrows):>4}건 / 주소 {n_site:>4}개")

json.dump(allrows, open(os.path.join(MODS, "_rva_catalog.json"), "w", encoding="utf-8"),
          ensure_ascii=False, indent=1)

# 사람이 읽는 표
with open(os.path.join(MODS, "_rva_catalog.md"), "w", encoding="utf-8") as g:
    g.write("# RVA 통합 카탈로그 (0.5.2 기준값 → 0.5.3 재탐색 대상)\n\n")
    g.write("> 생성 = `rva_catalog.py`. fog_damage_fix 제외(게임측 수정). crm/Spectator_Chat/meta_item_delegate = RVA 0.\n\n")
    for mod in TARGET_MODS:
        rows = [r for r in allrows if r["mod"] == mod]
        if not rows:
            continue
        g.write(f"\n## {mod} ({len(rows)}건)\n\n| 상수 | RVA(0.5.2) | 종류 | 위치 | 용도 |\n|---|---|---|---|---|\n")
        for r in sorted(rows, key=lambda x: (x["file"], x["line"])):
            note = r["note"].replace("|", "\\|")[:70]
            g.write(f"| `{r['name']}` | `{r['rva']}` | {r['kind']} | {r['file']}:{r['line']} | {note} |\n")

# migrate_rva.py 에 붙여넣을 TARGETS
with open(os.path.join(MODS, "_rva_targets.py"), "w", encoding="utf-8") as g:
    g.write("# 자동생성 (rva_catalog.py) — migrate_rva.py 의 TARGETS 로 사용\n")
    g.write("# 주의: patch_site/array 는 mid-func imm 이라 마스크시그 부적합 → 컨테이너-델타 방식 필요\n")
    g.write("TARGETS = [\n")
    for mod in TARGET_MODS:
        rows = [r for r in allrows if r["mod"] == mod and r["kind"] == "const"]
        if not rows:
            continue
        g.write(f"    # ── {mod} ({len(rows)}) ──\n")
        for r in sorted(rows, key=lambda x: (x["file"], x["line"])):
            g.write(f"    ({r['rva']}, \"{mod.replace('tfm2_', '')}:{r['name']}\"),\n")
    g.write("]\n")
print("\n생성: _rva_catalog.json / _rva_catalog.md / _rva_targets.py")
