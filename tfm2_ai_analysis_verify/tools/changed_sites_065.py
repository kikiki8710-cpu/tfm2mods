"""0.6.5 에서 바뀐 코드에 관측 지점을 추가한다 (2026-10-21).

① 함수쌍(atlas_064_065.csv)마다 신 명령열 중 구와 정렬되지 않은(추가·교체) 명령
② 0.6.5 신규·재작성 판정 함수(RE G1~G6·본진체류)의 명령 전체
에서 '판단 값이 지나가는 명령'만 고른다: cmp/test(뒤에 조건 소비), 즉치(주소 아님)를 쓰는 산술, mul/imul/div/idiv.
call/jmp/push/pop·스택 조작·rip 상대 lea 는 뺀다(gen_sites 가 걸 수 없거나 의미 없음).
출력: <out>/extra_v065_changed.txt (gen_sites --extra 형식: `RVA 메모`)
사용: python -B -X utf8 tools/changed_sites_065.py <migration_065 폴더> <out 폴더>
"""
import collections, csv, difflib, os, re, sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
import _x065  # noqa: E402
from _x065 import O, N, PO, PN, insns  # noqa: E402
from capstone.x86 import X86_OP_IMM, X86_OP_MEM  # noqa: E402
_x065.cs.detail = True

mig, out = sys.argv[1:3]
os.makedirs(out, exist_ok=True)
NUM = re.compile(r"0x[0-9a-f]+|\b\d+\b")
REG = re.compile(r"\b(?:[re]?(?:ax|bx|cx|dx|si|di|bp|sp)|r\d+[dwb]?|[abcd][lh]|[sd]il|[sb]pl|xmm\d+)\b")
# 0.6.5 신규·재작성 판정 함수(RE 판정 원문 기준) — 함수 전체를 본다
NEW_FUNCS = {
    0x15ce3c0: "기분판정", 0xdab3e0: "오브젝트전력비교", 0xdacd70: "데드라인게이트", 0xf49ee0: "팀지정대상제외",
    0xf40d70: "팀교전대상평가", 0xf40710: "fight_model평가", 0xd625c0: "교전평가래퍼", 0xf51460: "후보제외술어",
    0xd7cd90: "활동라벨", 0xec7610: "막타여유", 0xf38320: "반응지연", 0xf0c500: "다음템short", 0xf0c160: "can_buy",
    0xd57940: "passive_jungle신규", 0xe32b00: "적도착집계", 0xfdbc10: "처치시간추정", 0xf7d360: "경로비용배수",
}
SKIP = ("call", "jmp", "push", "pop", "ret", "leave", "enter", "nop", "int3")


def want(i):
    m = i.mnemonic
    if m in SKIP or m.startswith("j") or m.startswith(("rep", "movs", "stos")):
        return False
    if any(op.type == X86_OP_MEM and op.mem.base and i.reg_name(op.mem.base) in ("rsp", "rip") for op in i.operands):
        return False
    if m in ("cmp", "test") or m in ("mul", "imul", "div", "idiv"):
        return True
    for op in i.operands:
        if op.type == X86_OP_IMM:
            v = op.imm & 0xFFFFFFFFFFFFFFFF
            if not (0x140000000 <= v < 0x150000000) and m not in ("mov", "movabs", "and", "or", "xor", "lea"):
                return True
            if m in ("mov", "movabs") and v not in (0, 1) and not (0x140000000 <= v < 0x150000000):
                return True  # 상수 로드(계수·임계값 후보)
    return False


def k1(i):
    return i.mnemonic + "|" + NUM.sub("N", i.op_str)


def k2(i):
    return i.mnemonic + "|" + REG.sub("R", NUM.sub("N", i.op_str))


picked = collections.OrderedDict()
atlas = list(csv.DictReader(open(os.path.join(mig, "atlas_064_065.csv"), encoding="utf-8-sig")))
for r in atlas:
    if not r["new"]:
        continue
    o, n = int(r["old"], 16), int(r["new"], 16)
    if o not in PO or n not in PN:
        continue
    A = insns(O, o, int(PO[o]["size"])); B = insns(N, n, int(PN[n]["size"]))
    sm = difflib.SequenceMatcher(a=[k2(i) for i in A], b=[k2(i) for i in B], autojunk=False)
    for tag, a1, a2, b1, b2 in sm.get_opcodes():
        if tag in ("insert", "replace"):
            for i in B[b1:b2]:
                if want(i):
                    picked.setdefault(i.address - 0x140000000, f"changed:{r['name'][:40]}")
for f, lab in NEW_FUNCS.items():
    if f not in PN:
        print("함수 색인 없음", hex(f)); continue
    for i in insns(N, f, int(PN[f]["size"])):
        if want(i):
            picked.setdefault(i.address - 0x140000000, f"newfn:{lab}")
with open(os.path.join(out, "extra_v065_changed.txt"), "w", encoding="utf-8") as h:
    h.write("# 0.6.5 바뀐 코드·신규 판정 함수의 관측 지점 (changed_sites_065.py)\n")
    for rva, note in sorted(picked.items()):
        h.write(f"{rva:#x} {note}\n")
c = collections.Counter(v.split(":")[0] for v in picked.values())
print("지점", len(picked), dict(c))
