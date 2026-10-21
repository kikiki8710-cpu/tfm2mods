"""0.6.4 → 0.6.5 분석 함수 전수 명령 대조 (2026-10-21).

함수 대응(atlas_064_065.csv)마다 구·신 명령열을 (니모닉 + 숫자→N) 키로 정렬해 차이를 다섯 갈래로 나눈다.
  same     = 원문 동일
  reg      = 레지스터만 다름(코드젠)
  const    = 명령 구조 같고 숫자(즉치·변위)만 다름 → (구값, 신값) 쌍으로 모은다
  scaled   = 0.6.5 debuff_ratio 배율 삽입(신 쪽 [..+0x288] 과 /1000 매직 0x20c49ba5e353f7cf 동반)
  struct   = 명령이 추가·삭제·교체된 덩어리(설명 안 된 구조 변경 — RE 대상 후보)
숫자 쌍은 여러 함수에 반복되면(≥5 함수) 구조체 배치 이동으로, 한두 함수에만 있으면 값 변경 후보로 본다.
출력(<out>): func_diff_065.csv · const_pairs_065.csv · hunks_065.md(구조 변경 덩어리 원문)
사용: python -B -X utf8 tools/func_diff.py <migration_065 폴더>
"""
import collections, csv, difflib, os, re, sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
import _x065  # noqa: E402
from _x065 import O, N, PO, PN, insns  # noqa: E402
from capstone.x86 import X86_OP_IMM, X86_OP_MEM  # noqa: E402
_x065.cs.detail = True

mig = sys.argv[1]
atlas = list(csv.DictReader(open(os.path.join(mig, "atlas_064_065.csv"), encoding="utf-8-sig")))
NUM = re.compile(r"0x[0-9a-f]+|\b\d+\b")
REG = re.compile(r"\b(?:[re]?(?:ax|bx|cx|dx|si|di|bp|sp)|r\d+[dwb]?|[abcd][lh]|[sd]il|[sb]pl|xmm\d+)\b")
MAGIC1000 = "0x20c49ba5e353f7cf"


def txt(i):
    return f"{i.mnemonic} {i.op_str}"


def k1(i):
    return i.mnemonic + "|" + NUM.sub("N", i.op_str)


def k2(i):
    return i.mnemonic + "|" + REG.sub("R", NUM.sub("N", i.op_str))


def nums(i):
    """비교할 숫자만: 즉치(이미지 주소 제외) + 구조체 필드 변위(베이스가 rip/rsp/rbp 가 아닌 메모리).
    분기·호출 대상, RIP 상대(데이터 주소), 스택 지역변수 변위([rsp/rbp+x] — 프레임 배치로 늘 바뀜)는 뺀다."""
    if i.mnemonic == "call" or i.mnemonic.startswith("j"):
        return []
    out = []
    for op in i.operands:
        if op.type == X86_OP_IMM:
            v = op.imm & 0xFFFFFFFFFFFFFFFF
            if not (0x140000000 <= v < 0x150000000):
                out.append(("i", v))
        elif op.type == X86_OP_MEM:
            base = i.reg_name(op.mem.base) if op.mem.base else ""
            if base not in ("rip", "rsp", "rbp", "esp", "ebp") and op.mem.disp:
                out.append(("d", op.mem.disp & 0xFFFFFFFFFFFFFFFF))
    return out


def stackish(i):
    """숫자 차이가 스택·RIP 상대에서만 났는지(= 코드젠 차이로 본다)"""
    return any(op.type == X86_OP_MEM and op.mem.base and i.reg_name(op.mem.base) in ("rip", "rsp", "rbp") for op in i.operands)


rows, pairs, hunks = [], collections.defaultdict(lambda: [0, set()]), []
for r in atlas:
    if not r["new"]:
        continue
    o, n = int(r["old"], 16), int(r["new"], 16)
    if o not in PO or n not in PN:
        continue
    A = insns(O, o, int(PO[o]["size"])); B = insns(N, n, int(PN[n]["size"]))
    c = collections.Counter()
    sm = difflib.SequenceMatcher(a=[k1(i) for i in A], b=[k1(i) for i in B], autojunk=False)
    fh = []
    for tag, a1, a2, b1, b2 in sm.get_opcodes():
        if tag == "equal":
            for x, y in zip(A[a1:a2], B[b1:b2]):
                if txt(x) == txt(y):
                    c["same"] += 1
                    continue
                na, nb = nums(x), nums(y)
                if na == nb:
                    c["frame" if stackish(x) or stackish(y) else "reg"] += 1
                    continue
                c["const"] += 1
                for (ka, va), (kb, vb) in zip(na, nb):
                    if va != vb:
                        e = pairs[(ka, va, vb)]; e[0] += 1; e[1].add(r["old"])
            continue
        sa, sb = A[a1:a2], B[b1:b2]
        # 레지스터만 다른 교체 덩어리
        if tag == "replace" and [k2(i) for i in sa] == [k2(i) for i in sb]:
            c["reg"] += len(sa)
            continue
        btxt = " ".join(txt(i) for i in sb)
        if "+ 0x288]" in btxt and (MAGIC1000 in btxt or "0x10624dd3" in btxt or "0xc350" in btxt or "0xc34f" in btxt):
            c["scaled"] += max(len(sa), len(sb))
            continue
        c["struct"] += max(len(sa), len(sb))
        fh.append((tag, sa, sb))
    total = len(A)
    verdict = "그대로" if c["struct"] == 0 and c["const"] == 0 and c["scaled"] == 0 else (
        "구조변경" if c["struct"] else ("스탯배율" if c["scaled"] else "숫자만"))
    rows.append(dict(old=r["old"], new=r["new"], name=r["name"], area=r["area"], fn_verdict=r["verdict"], insns_old=total, insns_new=len(B),
                     same=c["same"], reg=c["reg"], frame=c["frame"], const=c["const"], scaled=c["scaled"], struct=c["struct"], hunks=len(fh), verdict=verdict))
    if fh:
        hunks.append((r, fh))

os.makedirs(mig, exist_ok=True)
with open(os.path.join(mig, "func_diff_065.csv"), "w", encoding="utf-8-sig", newline="") as h:
    w = csv.DictWriter(h, fieldnames=list(rows[0])); w.writeheader(); w.writerows(rows)
with open(os.path.join(mig, "const_pairs_065.csv"), "w", encoding="utf-8-sig", newline="") as h:
    w = csv.writer(h); w.writerow(["what", "old", "new", "old_hex", "new_hex", "delta", "count", "functions", "kind", "fn_list"])
    for (k, va, vb), (cnt, fs) in sorted(pairs.items(), key=lambda x: (-len(x[1][1]), -x[1][0])):
        kind = "배치이동(≥5함수)" if len(fs) >= 5 else "값변경후보"
        w.writerow(["즉치" if k == "i" else "필드변위", va, vb, hex(va), hex(vb), vb - va, cnt, len(fs), kind, " ".join(sorted(fs)[:12])])
with open(os.path.join(mig, "hunks_065.md"), "w", encoding="utf-8") as h:
    h.write("# 0.6.4→0.6.5 구조 변경 덩어리 원문 (func_diff.py)\n\n")
    for r, fh in sorted(hunks, key=lambda x: -sum(max(len(a), len(b)) for _, a, b in x[1])):
        h.write(f"## {r['old']} → {r['new']} {r['name']} ({r['verdict']})\n\n")
        for tag, sa, sb in fh:
            h.write(f"- {tag} 구 {len(sa)} / 신 {len(sb)}\n```\n")
            for i in sa:
                h.write(f"- {i.address - 0x140000000:#x} {txt(i)}\n")
            for i in sb:
                h.write(f"+ {i.address - 0x140000000:#x} {txt(i)}\n")
            h.write("```\n")
        h.write("\n")
vc = collections.Counter(r["verdict"] for r in rows)
print(len(rows), dict(vc))
print("숫자쌍", len(pairs), "배치이동", sum(1 for v in pairs.values() if len(v[1]) >= 5), "값변경후보", sum(1 for v in pairs.values() if len(v[1]) < 5))
