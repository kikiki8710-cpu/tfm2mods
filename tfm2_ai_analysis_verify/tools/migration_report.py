"""0.6.4 → 0.6.5 분석 함수·측정 지점 변화 요약 (2026-10-21).

입력: atlas_064_065.csv(atlas_migrate) · sites_064_065.csv(site_migrate) · measured_summary/(A~H 범주·상세전술 후보)
출력: <migration 폴더>/summary.md · tactic_065.csv · stat_scaled_065.csv
사용: python -B -X utf8 tools/migration_report.py <migration_065 폴더> <measured_summary 폴더> <site_catalog.json>
"""
import collections, csv, json, os, re, sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
from _x065 import N, PN, insns  # noqa: E402

mig, ms, cat_p = sys.argv[1:4]
atlas = list(csv.DictReader(open(os.path.join(mig, "atlas_064_065.csv"), encoding="utf-8-sig")))
sites = list(csv.DictReader(open(os.path.join(mig, "sites_064_065.csv"), encoding="utf-8-sig")))
cats = {r["f"]: r for r in csv.DictReader(open(os.path.join(ms, "function_categories.csv"), encoding="utf-8-sig"))}
tactics = json.load(open(os.path.join(ms, "tactic_coverage.json"), encoding="utf-8"))
catalog = json.load(open(cat_p, encoding="utf-8"))["sites"]
fa = {r["old"]: r for r in atlas}
site_by_old = {r["old"]: r for r in sites}


def vkind(v):
    """판정 문자열 → 대분류: 그대로 / 이동 / 수정 / 병합 / 삭제 / 미매핑"""
    if v.startswith("aidiff:"):
        return {"IDENTICAL": "그대로", "SHIFTED": "이동", "EDITED": "수정", "MERGED": "병합", "DELETED": "삭제", "RENAMED": "이름변경"}[v.split(":")[1]]
    if v == "미매핑":
        return "미매핑"
    return "수정" if v.endswith("상수변경") else "그대로"


# 1) 스탯 배율(+0x288 × stat / 1000) 패턴이 새로 들어간 0.6.5 함수
PAT = re.compile(r"qword ptr \[\w+ \+ 0x288\]")
scaled = {}
for r in atlas:
    if not r["new"]:
        continue
    n = int(r["new"], 16)
    if n not in PN:
        continue
    L = insns(N, n, int(PN[n]["size"]))
    hit = []
    for i, q in enumerate(L):
        if q.mnemonic in ("imul", "mov") and PAT.search(q.op_str):
            nxt = " ".join(x.op_str for x in L[i + 1:i + 8])
            if "0x20c49ba5e353f7cf" in nxt or "0x10624dd3" in nxt:
                hit.append(hex(q.address - 0x140000000))
    if hit:
        scaled[r["old"]] = hit

# 2) 사이트 상태 요약 단위
OKSET = {"same", "disp", "reg"}


def site_state(r):
    """지점 1개의 0.6.5 상태: 유지 / 재배치(값 생존) / 배율(×1000) / 사라짐(값 변경·코드 소실) / 함수없음"""
    st, sv = r["status"], r.get("survival", "")
    if st in OKSET:
        return "유지"
    if st == "nofunc":
        return "함수없음"
    # noimm = 곱셈 사슬(lea/shl/sub)·레지스터 비교 조각 — 값이 명령에 안 박혀 있어 생존 판정 불가 → 회차 대조로 넘김
    return {"survive": "재배치", "scaled": "배율", "noimm": "판정불가"}.get(sv, "사라짐")


out = []
w = out.append
w("# 0.6.4 → 0.6.5 분석 함수·측정 지점 변화 요약 (2026-10-21)\n")
w("생성 = `C:\\tfm2mods\\tfm2_ai_analysis_verify\\tools\\migration_report.py` · 원표 = `atlas_064_065.csv`(함수) · `sites_064_065.csv`(지점 42,307) · `tactic_065.csv`(상세전술 후보) · `stat_scaled_065.csv`.\n")

# 함수 전체
w("## 1. 분석 아틀라스 함수 1,242개\n")
c = collections.Counter(vkind(r["verdict"]) for r in atlas)
w("| 판정 | 수 |\n|---|---|")
for k in ("그대로", "이동", "수정", "이름변경", "병합", "삭제", "미매핑"):
    w(f"| {k} | {c[k]} |")
w("\n- 그대로 = aidiff IDENTICAL 또는 보조 짝(골격·호출순서·머리·배치)에서 즉치 다중집합 동일. 이동 = aidiff SHIFTED(패닉 위치 줄만 이동). 수정 = aidiff EDITED 또는 보조 짝에서 상수 변경.")
w(f"- 미매핑 {c['미매핑']}개는 모두 `REGISTRY_ONLY_501`(측정 지점 없는 등록부 전용) — 측정 대상 영향 = 아래 §2.\n")

# 범주별
w("## 2. 측정 범주(A~H)별 함수 판정\n")
w("| 범주 | 함수 | 그대로 | 이동 | 수정 | 이름변경 | 병합 | 삭제 | 미매핑 | 스탯배율 신규 |\n|---|---|---|---|---|---|---|---|---|---|")
bycat = collections.defaultdict(list)
for f, r in cats.items():
    bycat[r["cat"]].append(f)
for k in sorted(bycat):
    fs = bycat[k]
    cc = collections.Counter(vkind(fa[f]["verdict"]) if f in fa else "아틀라스밖" for f in fs)
    w(f"| {k} | {len(fs)} | {cc['그대로']} | {cc['이동']} | {cc['수정']} | {cc['이름변경']} | {cc['병합']} | {cc['삭제']} | {cc['미매핑'] + cc['아틀라스밖']} | {sum(1 for f in fs if f in scaled)} |")
w("")

# 지점
w("## 3. 측정 지점 42,307곳\n")
cs = collections.Counter(r["status"] for r in sites)
w("| 상태 | 수 | 뜻 |\n|---|---|---|")
for k, d in (("same", "명령 원문 동일"), ("disp", "메모리 변위만 이동(필드 오프셋)"), ("reg", "레지스터만 바뀜(코드젠)"),
             ("imm_struct", "즉치 변경 — 구조 크기·오프셋·enum 등(손잡이 아님)"), ("imm_knob", "즉치 변경 — 의미 기록 값·M1 손잡이"),
             ("unaligned", "대응 명령 없음(근처 코드 재작성)"), ("nofunc", "함수 삭제·미매핑")):
    w(f"| {k} | {cs[k]} | {d} |")
pairs = collections.Counter((r["imm_old"], r["imm_new"]) for r in sites if r["status"] == "imm_struct")
w("\n구조상수 변경 상위(구→신, 10진): " + " · ".join(f"`{a}→{b}`×{n}" for (a, b), n in pairs.most_common(15)) + "\n")

# 스탯 배율
w("## 4. ★스탯 배율 신규 — `stat × [선수+0x288] / 1000`\n")
w(f"0.6.5 에서 AI 가 선수 스탯(+0x1e8/+0x1f0/+0x1f8 등)을 읽는 자리에 `[선수+0x288]` 곱과 /1000(매직 `0x20c49ba5e353f7cf`)이 새로 들어갔다. 0.6.4 의 같은 함수들엔 없다. 분석 함수 **{len(scaled)}개**. "
  "스탯을 쓰는 계수·임계점은 상수가 같아도 입력이 달라졌으므로 **재측정 대상**이다(필드 의미 = RE 진행 중).\n")
w("| 0.6.4 | 0.6.5 | 범주 | 이름 | 배율 지점 |\n|---|---|---|---|---|")
with open(os.path.join(mig, "stat_scaled_065.csv"), "w", encoding="utf-8-sig", newline="") as h:
    cw = csv.writer(h); cw.writerow(["old", "new", "cat", "name", "sites_065"])
    for o, hit in sorted(scaled.items(), key=lambda x: (cats.get(x[0], {}).get("cat", "Z"), x[0])):
        r = fa[o]; k = cats.get(o, {}).get("cat", "-")
        cw.writerow([o, r["new"], k, r["name"], " ".join(hit)])
        w(f"| `{o}` | `{r['new']}` | {k} | {r['name']} | {' '.join(hit[:3])}{' …' if len(hit) > 3 else ''} |")
w("")

# 손잡이 값 변경
w("## 5. 즉치가 바뀐 의미 기록 지점(imm_knob)\n")
w("| 0.6.4 | 0.6.5 | 함수 | 구 명령 | 신 명령 |\n|---|---|---|---|---|")
for r in sites:
    if r["status"] == "imm_knob":
        w(f"| `{r['old']}` | `{r['new']}` | {fa[r['fn_old']]['name'] if r['fn_old'] in fa else r['fn_old']} | `{r['insn_old']}` | `{r['insn_new']}` |")
w("")

# 상세전술 후보
w("## 6. 상세전술 후보 135개 — 손잡이 지점 0.6.5 상태\n")
trows = []
for t in tactics:
    ks = t.get("knob", [])
    st = collections.Counter(site_state(site_by_old[k[1]]) if k[1] in site_by_old else "카탈로그밖" for k in ks)
    fns = sorted({k[0] for k in ks})
    ed = [f for f in fns if f in fa and vkind(fa[f]["verdict"]) in ("수정", "병합", "삭제")]
    sc = [f for f in fns if f in scaled]
    lost = st["사라짐"] + st["함수없음"]
    if lost:
        verdict = "재분석"  # 손잡이 값·코드가 0.6.5 에서 안 보임 → RE 로 새 손잡이 위치·값 확인
    elif sc or st["배율"]:
        verdict = "재측정(스탯배율)"  # 손잡이는 살아 있으나 입력 스탯이 stat×m/1000 로 바뀜
    elif ed or st["재배치"] or st["판정불가"]:
        verdict = "확인"  # 값은 살아 있고 위치·주변 코드만 변경 → 지점 표 재생성 후 회차 대조
    else:
        verdict = "그대로"
    trows.append(dict(section=t["section"], name=t["name"], stage=t["stage"], knobs=len(ks), keep=st["유지"],
                      moved=st["재배치"], unknown=st["판정불가"], scaled=st["배율"], lost=st["사라짐"], nofunc=st["함수없음"],
                      edited_fns=" ".join(ed), stat_scaled_fns=" ".join(sc), verdict=verdict))
with open(os.path.join(mig, "tactic_065.csv"), "w", encoding="utf-8-sig", newline="") as h:
    cw = csv.DictWriter(h, fieldnames=list(trows[0])); cw.writeheader(); cw.writerows(trows)
vc = collections.Counter(r["verdict"] for r in trows)
w("| 판정 | 수 | 뜻 |\n|---|---|---|")
w(f"| 그대로 | {vc['그대로']} | 손잡이 지점 전부 명령 유지(원문·변위·레지스터만) + 근거 함수 수정 없음 → 0.6.4 측정 결과 그대로 |")
w(f"| 확인 | {vc['확인']} | 손잡이 값은 신 함수에 살아 있음(재배치) 또는 근거 함수 수정 → 지점 표 재생성 후 회차에서 값 대조 |")
w(f"| 재측정(스탯배율) | {vc['재측정(스탯배율)']} | 손잡이는 살아 있으나 근거 함수가 스탯을 stat×[+0x288]/1000 로 읽게 바뀜 → 입력 분포 재측정 |")
w(f"| 재분석 | {vc['재분석']} | 손잡이 값이 신 함수에서 안 보이거나 함수 삭제 → RE 로 새 위치·값 확인 |")
w("\n- 재배치 = 정렬이 못 맞췄지만 같은 니모닉·같은 즉치 명령이 신 함수에 있음(작은 값 0·1 은 우연 일치 가능 — 약한 증거).")
w("- 판정은 후보 단위 최악값: 손잡이 1개라도 사라지면 재분석.")
for vv in ("재분석", "재측정(스탯배율)", "확인"):
    w(f"\n### {vv}\n")
    w("| 구역 | 후보 | 단계 | 손잡이 | 유지 | 재배치 | 판정불가 | 배율 | 사라짐 | 함수없음 | 수정 함수 | 스탯배율 함수 |\n|---|---|---|---|---|---|---|---|---|---|---|---|")
    for r in trows:
        if r["verdict"] == vv:
            w(f"| {r['section']} | {r['name']} | {r['stage']} | {r['knobs']} | {r['keep']} | {r['moved']} | {r['unknown']} | {r['scaled']} | {r['lost']} | {r['nofunc']} | {r['edited_fns']} | {r['stat_scaled_fns']} |")
w("")

# 경계확정 기록
w("## 7. 경계확정(임계점 실측 확정) 지점\n")
edge = list(csv.DictReader(open(os.path.join(ms, "edge_confirmed.csv"), encoding="utf-8-sig")))
rec2sites = collections.defaultdict(list)
for s in catalog:
    for c in s.get("claims", []):
        rec2sites[c.get("record")].append(hex(s["rva"]))
ecs = collections.Counter()
for e in edge:
    ss = [site_state(site_by_old[x]) for x in rec2sites.get(e["record"], []) if x in site_by_old]
    st = ("유지" if all(x == "유지" for x in ss) else "재배치" if all(x in ("유지", "재배치", "배율", "판정불가") for x in ss) else "사라짐") if ss else "지점없음"
    if e["function"] in scaled and st == "유지":
        st = "유지·스탯배율"
    ecs[st] += 1
w(f"경계확정 기록 {len(edge)}건: " + " · ".join(f"{k} {v}" for k, v in ecs.most_common()) + "\n")

# 대응불명 상위 함수
w("## 8. 대응불명(unaligned) 지점이 많은 함수 상위 20\n")
uc = collections.Counter(r["fn_old"] for r in sites if r["status"] == "unaligned")
w("| 0.6.4 | 0.6.5 | 판정 | 범주 | 이름 | 대응불명 | 스탯배율 |\n|---|---|---|---|---|---|---|")
for f, n in uc.most_common(20):
    r = fa.get(f, {})
    w(f"| `{f}` | `{r.get('new','')}` | {r.get('verdict','')} | {cats.get(f, {}).get('cat', '-')} | {r.get('name','')} | {n} | {'O' if f in scaled else ''} |")
w("")

# 삭제·병합
w("## 9. 삭제·병합·이름변경 함수\n")
for r in atlas:
    if vkind(r["verdict"]) in ("삭제", "병합", "이름변경"):
        n = sum(1 for s in sites if s["fn_old"] == r["old"])
        w(f"- `{r['old']}` {r['name']} — {r['verdict']}{' → `' + r['new'] + '`' if r['new'] else ''} · 범주 {cats.get(r['old'], {}).get('cat', '-')} · 지점 {n}")
open(os.path.join(mig, "summary.md"), "w", encoding="utf-8").write("\n".join(out) + "\n")
print("ok", len(scaled), vc, ecs)
