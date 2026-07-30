# ULT# 블록의 궁 트리에 등장하는 effect vtable 수집 → 스키마 커버리지 진단
import re, json, sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
DUMP = r"C:\tfm2mods\sylas_hijack\effect_dump.txt"
SCHEMA = r"C:\tfm2mods\sylas_hijack\tools\effect_vtable_schema.json"

sch = json.load(open(SCHEMA, encoding="utf-8"))["vtables"]
known = set(int(k,16) for k in sch)

txt = open(DUMP, encoding="utf-8", errors="replace").read()
# ULT# 블록만: "===== NAME ULT#n ent=" ~ 다음 ===== 까지
blocks = re.split(r"(?=^===== )", txt, flags=re.M)
per_champ = {}   # champ -> set(vt)
vt_freq = {}     # vt -> count (ULT 트리 내)
for b in blocks:
    m = re.match(r"===== (\w+) ULT#\d+ ent=", b)
    if not m: continue
    champ = m.group(1)
    for vm in re.finditer(r"vt=RVA:(0x[0-9a-f]+)", b):
        vt = int(vm.group(1),16)
        per_champ.setdefault(champ, set()).add(vt)
        vt_freq[vt] = vt_freq.get(vt,0)+1

all_vt = set(vt_freq)
unknown = sorted(all_vt - known, key=lambda v: -vt_freq[v])
print(f"ULT# 캡처 챔프: {len(per_champ)}명")
print(f"궁 트리 등장 distinct vtable: {len(all_vt)}개 (스키마등록 {len(all_vt & known)}, 미등록 {len(unknown)})\n")
print("=== 미등록 vtable (빈도순) — Ghidra Debug::fmt 추출 대상 ===")
for v in unknown:
    print(f"  0x{v:x}  x{vt_freq[v]}")
print(f"\n총 미등록 {len(unknown)}개")
# 챔프별 미등록 개수(어느 챔프가 미등록 타입 많이 쓰나)
print("\n=== 챔프별 미등록 vtable 수 (많을수록 디코드 불완전) ===")
for c in sorted(per_champ, key=lambda c: -len(per_champ[c]-known)):
    u = per_champ[c]-known
    if u: print(f"  {c}: {len(u)}개 미등록 / 총 {len(per_champ[c])}")
