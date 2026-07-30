# -*- coding: utf-8 -*-
# match3_053.py — 2차 매칭 결과의 L3_AMBIG(쌍둥이 함수) 를 "순서 대응"으로 푸는 후처리.
#   근거: 미해결 다수가 제네릭 모노모픽화로 생긴 동일코드 N벌이다.
#         0.5.2 의 형제 K개와 0.5.3 의 후보 K개가 같은 수로 나란히 있으면,
#         링커가 그룹 내 순서를 뒤집을 이유가 없으므로 주소 오름차순끼리 짝지운다.
#   ⚠ 이건 추정이다 — 결과에 ORDER 등급을 따로 붙여 Ghidra 확인 대상임을 남긴다.
import json, io, sys, collections, re, pickle
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

d = json.load(open(r"C:\tfm2mods\_rva_match2_053.json", encoding="utf-8"))
B = pickle.load(open(r"C:\tfm2mods\_fnidx_053.pkl", "rb"))
RE_C = re.compile(r'(0x[0-9a-f]+)\(cos([0-9.]+),z(\d+),r([0-9.]+)\)')

def cands_of(r):
    out = []
    for c in r.get("info", {}).get("cands", []):
        m = RE_C.match(c)
        if m:
            out.append((int(m.group(1), 16), float(m.group(2)), int(m.group(3)), float(m.group(4))))
        else:
            try:
                out.append((int(c, 16), None, None, None))
            except ValueError:
                pass
    return out

amb = [r for r in d if r["grade"] in ("L3_AMBIG", "L1_MULTI", "L2_MULTI") and not r.get("new")]
print(f"AMBIG/MULTI 미해결 {len(amb)}건")

# 근접 후보(최상위 cos 와 0.002 이내) 집합이 같은 것끼리 그룹
groups = collections.defaultdict(list)
for r in amb:
    cs = cands_of(r)
    if not cs:
        continue
    if cs[0][1] is None:                       # L1_MULTI (cos 정보 없음)
        key = tuple(sorted(c[0] for c in cs))
    else:
        top = cs[0][1]
        key = tuple(sorted(c[0] for c in cs if top - c[1] <= 0.002))
    if len(key) >= 2:
        groups[key].append(r)

resolved = 0
for key, members in groups.items():
    members.sort(key=lambda r: int(r["old"], 16))
    cands = sorted(key)
    if len(members) == len(cands):
        for r, c in zip(members, cands):
            r["new"] = hex(c)
            r["grade"] = "ORDER"
            r["info"]["order"] = f"형제 {len(members)}개 ↔ 후보 {len(cands)}개 순서대응"
            resolved += 1
    else:
        for r in members:
            r["info"]["order"] = f"형제 {len(members)} vs 후보 {len(cands)} 개수 불일치 → 미해결"

print(f"순서대응으로 해결 {resolved}건")

fs = [r for r in d if r.get("offset_in_fn") == 0]
ok = sum(1 for r in fs if r.get("new"))
print(f"함수시작 해결 {ok}/{len(fs)}")
print("등급:", dict(collections.Counter(r["grade"] for r in fs)))

json.dump(d, open(r"C:\tfm2mods\_rva_final_053.json", "w", encoding="utf-8"),
          ensure_ascii=False, indent=1)

# ── 최종 산출물: 모드별 마이그 지시서 ──
CONF = {"L1_EXACT": "확정", "L1_ZONE": "확정", "L2_HEAD": "확정", "L2_ZONE": "확정",
        "L3_ZONE": "유력", "L3_SIM": "유력", "ORDER": "추정(순서)",
        "L3_AMBIG": "미해결", "L1_MULTI": "미해결", "L2_MULTI": "미해결",
        "L3_WEAK": "미해결", "NONE": "미해결", "NOT_IN_TEXT": "데이터/비RVA"}
bymod = collections.defaultdict(list)
for r in d:
    seen = set()
    for ref in r["refs"]:
        if ref["mod"] in seen:
            continue
        seen.add(ref["mod"])
        bymod[ref["mod"]].append((ref, r))

with open(r"C:\tfm2mods\_MIGRATE_053.md", "w", encoding="utf-8") as g:
    g.write("# 0.5.3 마이그레이션 지시서 — 모드별 세션 인계용\n\n")
    g.write("> 이 파일 하나만 보고 각 모드 세션이 작업할 수 있게 쓴 것. "
            "생성 = `rva_catalog.py` → `fnindex.py` → `match2_053.py` → `match3_053.py` (전부 `C:\\tfm2mods\\`)\n\n")
    g.write("## 0. 버전\n\n")
    g.write("| | 0.5.2 (OLD = 모드 소스 베이스) | 0.5.3 (NEW) |\n|---|---|---|\n")
    g.write("| buildid | 24310934 | **24451609** |\n")
    g.write("| exe | 69,209,088B | **74,970,624B** |\n")
    g.write("| sha256[:16] | 40b55c1b819dff50 | **6afff2cdb6bfa98e** |\n")
    g.write("| exe 백업 | `tfm2_0.5.2\\` | `tfm2_0.5.3\\` (+bundle, bundle_unpacked 1.1GB) |\n")
    g.write("| Ghidra MCP | `ghidra` (8080) | `ghidra_beta` (8081) |\n\n")
    g.write("## 1. 전 모드 공통 (반드시)\n\n")
    g.write("- **SDK = `C:\\tfm2mods\\sdk_053\\mod-sdk`** (base_version 0.5.3). `build_inj.ps1` L29 `$SDK` 전환 필요.\n")
    g.write("- **toolchain 무변경** = `nightly-2026-05-24` (rustc 1.98.0-nightly 23a3312d9) — 재설치 불필요.\n")
    g.write("- ★**게임 rlib 236개 전원 내용 DIFF ⟹ RVA 0 모드까지 전 모드 재빌드 필수.** "
            "재빌드만 하면 되는 모드 = `community_reaction_mod` · `Spectator_Chat` · `tfm2_meta_item_delegate` · "
            "save_probe · daram2 뷰플러스 9종.\n")
    g.write("- ⚠**빌드 플래그는 rustc 명령줄에 직접**: `-C opt-level=1 -C overflow-checks=off` "
            "(opt-level 2/3 = 재현 디투어 프레임 팽창 → STATUS_STACK_OVERFLOW).\n")
    g.write("- ★**신설 `libgame_ai` 크레이트** — 0.5.3에서 AI가 `game_core`에서 분리됐다(game_core rlib 407MB→333MB). "
            "AI 계열 함수는 위치뿐 아니라 코드가 바뀌었다고 보고 접근할 것.\n")
    g.write("- **대응 제외**: `tfm2_fog_damage_fix`(게임측에서 수정 — 마지막에 인게임 확인만) · "
            "`tfm2_transfer_tweak`(불필요 판정, 유저 지시 2026-07-29).\n\n")
    g.write("## 2. 이번 패치의 성격 — 읽고 시작할 것\n\n")
    g.write("- ⚠ **연속바이트 마스크시그(`migrate_rva.py`)는 이번에 전멸했다.** `.text` 44.0→48.6MB(+10.5%), "
            "함수 120,995→132,960개(+11,965). 핵심 훅 6종을 160B 마스크시그로 찾으면 **전부 NONE**이 나온다. "
            "그래서 `.pdata` 함수경계 + 명령 스켈레톤 해시 + 니모닉 코사인 + 국소 앵커 투표로 매칭했다.\n")
    g.write("- **0.5.3 함수는 0.5.2 대비 대체로 2~10% 크다**(재컴파일로 코드 자체가 변함). "
            "⟹ **함수내 오프셋이 보존되지 않는다.**\n")
    g.write("- 신뢰도 등급:\n"
            "  - **확정** = 명령 구조 일치 + 유일. 상수만 교체하면 된다.\n"
            "  - **유력** = 니모닉 코사인 최상위 + 2순위와 갭 + 크기비 정상. 대개 맞지만 "
            "**훅 설치 전 프롤로그(12B push8 등)·orig_len 경계·rip-rel 유무를 반드시 실측**할 것.\n"
            "  - **추정** = 쌍둥이(제네릭 모노모픽) 함수를 주소 순서로 짝지은 것. Ghidra 확인 권장.\n"
            "  - **미해결** = ghidra-re 필요. 억지로 넣지 말 것 — 신원검증 실패 시 미설치=inert가 안전하다.\n")
    g.write("- ⚠ **mid-func 사이트**(byte-patch imm·콜사이트)는 컨테이너 함수가 확정돼도 "
            "**오프셋을 그대로 옮기면 안 된다**(위 크기 변화). 컨테이너 안에서 원래 명령 패턴으로 재탐색해야 한다. "
            "`ai_adjust`의 byte-patch 62사이트가 전부 여기 해당.\n")
    g.write("- ⚠ 표의 `(inline)` 행은 소스 본문에서 긁어온 리터럴이라 **RVA가 아닌 상수(마스크·크기값)가 섞여 있다.** "
            "실제 RVA인지 소스에서 확인하고 쓸 것. 상수 선언(`const RVA_*`)과 `patch_site` 행이 진짜 대상이다.\n\n")
    g.write("## 3. 모드별 표\n")
    for mod in sorted(bymod):
        items = bymod[mod]
        starts = [(ref, r) for ref, r in items if r.get("offset_in_fn") == 0]
        mids = [(ref, r) for ref, r in items if r.get("offset_in_fn", 0) > 0]
        outs = [(ref, r) for ref, r in items if r["grade"] == "NOT_IN_TEXT"]
        ok_n = sum(1 for _, r in starts if r.get("new"))
        g.write(f"\n## {mod}\n\n")
        g.write(f"함수시작(훅 대상) **{ok_n}/{len(starts)} 해결** · mid-func 사이트 {len(mids)} · .text밖 {len(outs)}\n\n")
        def emit(rows_, title, note=""):
            if not rows_:
                return
            g.write(f"\n### {title}\n{note}\n\n| 상수 | 0.5.2 | → 0.5.3 | 신뢰도 | 근거 | 위치 |\n|---|---|---|---|---|---|\n")
            for ref, r in sorted(rows_, key=lambda x: (x[0]["file"], x[0]["line"])):
                inf = r.get("info", {})
                why = []
                if inf.get("cos") is not None:
                    why.append(f"cos {inf['cos']}/2nd {inf.get('second')}")
                if inf.get("zone"):
                    why.append(f"앵커{inf['zone']}")
                if inf.get("ratio"):
                    why.append(f"크기{inf['ratio']}")
                if inf.get("order"):
                    why.append(inf["order"])
                new = r.get("new") or ("후보: " + ", ".join(hex(c[0]) for c in cands_of(r)[:3])
                                       if cands_of(r) else "—")
                g.write(f"| `{ref['name']}` | `{r['old']}` | `{new}` | {CONF.get(r['grade'], r['grade'])} | "
                        f"{'; '.join(why)} | {ref['file']}:{ref['line']} |\n")

        real = [(ref, r) for ref, r in starts if not ref["kind"].startswith("inline")]
        inl = [(ref, r) for ref, r in starts if ref["kind"].startswith("inline")]
        emit(real, "함수 시작 RVA — ★주 대상(상수 선언)")
        emit(inl, "함수 시작 RVA — 참고(소스 본문 리터럴)",
             "> ⚠ 소스에서 긁어온 값이라 RVA가 아닌 상수가 섞여 있다. 쓰기 전에 소스에서 용도를 확인할 것.")
        if mids:
            g.write("\n### mid-func 사이트 (컨테이너 기준 재도출 필요)\n\n"
                    "| 상수 | 0.5.2 | 컨테이너 0.5.2 → 0.5.3 | 함수내 오프셋 | 컨테이너 신뢰도 |\n|---|---|---|---|---|\n")
            for ref, r in sorted(mids, key=lambda x: (x[0]["file"], x[0]["line"]))[:60]:
                g.write(f"| `{ref['name']}` | `{r['old']}` | `{r.get('container_old')}` → "
                        f"`{r.get('container_new', '—')}` | +{r.get('offset_in_fn')} | "
                        f"{CONF.get(r['grade'], r['grade'])} |\n")
            if len(mids) > 60:
                g.write(f"\n(외 {len(mids)-60}건 — 전체는 `_rva_final_053.json`)\n")
print("생성: _rva_final_053.json / _MIGRATE_053.md")
