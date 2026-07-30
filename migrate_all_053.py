# -*- coding: utf-8 -*-
# migrate_all_053.py — _rva_catalog.json 의 전 모드 RVA를 0.5.2 → 0.5.3 으로 일괄 재탐색.
#   migrate_rva.py 의 마스크시그 엔진을 그대로 재사용하되, TARGETS 를 손으로 적는 대신
#   카탈로그에서 읽어 전 모드를 한 번에 처리한다 (모드별 세션이 각자 주소찾기 반복하는 낭비 제거).
# 출력: _rva_result_053.json (기계용) / _rva_result_053.md (사람용, 모드별 표)
import json, os, io, sys, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
sys.path.insert(0, r"C:\tfm2mods")
import migrate_rva as M                      # load/roff/text_sec/make_pattern/find 재사용

OLD = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe"   # 모드 소스 베이스
NEW = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe"   # 신 버전
CAT = r"C:\tfm2mods\_rva_catalog.json"
SIG_BYTES = 0xa0

rows = json.load(open(CAT, encoding="utf-8"))
# 고유 RVA로 접기 — 같은 주소를 여러 모드/파일이 쓰면 한 번만 탐색하고 참조처를 모아둔다.
uniq = collections.OrderedDict()
for r in rows:
    for hx in r["rva"].split(","):
        v = int(hx, 16)
        e = uniq.setdefault(v, {"rva": hx, "refs": []})
        e["refs"].append({"mod": r["mod"], "name": r["name"], "kind": r["kind"],
                          "file": r["file"], "line": r["line"], "note": r["note"]})
print(f"카탈로그 {len(rows)}선언 → 고유 주소 {len(uniq)}개")

od, oib, osec = M.load(OLD)
nd, nib, nsec = M.load(NEW)
nva, nraw, nrsz = M.text_sec(nsec)
ntext = nd[nraw:nraw + nrsz]
ova, oraw, orsz = M.text_sec(osec)
print(f"OLD .text rva={hex(ova)} size={orsz}  /  NEW .text rva={hex(nva)} size={nrsz}")

out = []
stat = collections.Counter()
for i, (v, e) in enumerate(sorted(uniq.items()), 1):
    rec = dict(old=hex(v), refs=e["refs"])
    if M.roff(osec, v) is None:
        rec.update(status="NOT-IN-OLD-TEXT", hits=[])
    else:
        pat, mask = M.make_pattern(od, oib, osec, v, SIG_BYTES)
        hits = M.find(ntext, nva, pat, mask)
        rec["hits"] = [hex(h) for h in hits]
        if len(hits) == 1:
            rec.update(status="OK", new=hex(hits[0]), delta=hits[0] - v)
        elif len(hits) > 1:
            rec["status"] = "MULTI"
        else:
            rec["status"] = "NONE"
    stat[rec["status"]] += 1
    out.append(rec)
    if i % 25 == 0:
        print(f"  ... {i}/{len(uniq)}  {dict(stat)}", flush=True)

print("\n=== 집계 ===")
for k, c in stat.most_common():
    print(f"  {k:<16} {c}")

# 델타 분포 — 전역 단일 델타면 '주소만 이동', 제각각이면 재정렬(버전업급)
deltas = collections.Counter(r["delta"] for r in out if r.get("delta") is not None)
print("\n=== 델타 상위 ===")
for d, c in deltas.most_common(12):
    print(f"  {d:+#x}  ({d:+d})  x{c}")

json.dump(out, open(r"C:\tfm2mods\_rva_result_053.json", "w", encoding="utf-8"),
          ensure_ascii=False, indent=1)

# 모드별 사람용 표
bymod = collections.defaultdict(list)
for r in out:
    for ref in r["refs"]:
        bymod[ref["mod"]].append((ref, r))
with open(r"C:\tfm2mods\_rva_result_053.md", "w", encoding="utf-8") as g:
    g.write("# RVA 일괄 재탐색 결과 0.5.2 → 0.5.3\n\n")
    g.write(f"- OLD `{OLD}`\n- NEW `{NEW}`\n- 집계: " +
            " / ".join(f"{k} {c}" for k, c in stat.most_common()) + "\n\n")
    g.write("> `OK`=유일매치(그대로 교체 가능) · `MULTI`=다중매치(string-xref 등 확정 필요) · "
            "`NONE`=미발견(로직 변경 = ghidra-re 필요)\n")
    for mod in sorted(bymod):
        items = bymod[mod]
        cnt = collections.Counter(r["status"] for _, r in items)
        g.write(f"\n## {mod} — " + " / ".join(f"{k} {c}" for k, c in cnt.most_common()) + "\n\n")
        g.write("| 상수 | 0.5.2 | → 0.5.3 | 판정 | 종류 | 위치 |\n|---|---|---|---|---|---|\n")
        for ref, r in sorted(items, key=lambda x: (x[0]["file"], x[0]["line"])):
            new = r.get("new", "—" if r["status"] != "MULTI" else ", ".join(r["hits"][:3]))
            g.write(f"| `{ref['name']}` | `{r['old']}` | `{new}` | {r['status']} | "
                    f"{ref['kind']} | {ref['file']}:{ref['line']} |\n")
print("\n생성: _rva_result_053.json / _rva_result_053.md")
