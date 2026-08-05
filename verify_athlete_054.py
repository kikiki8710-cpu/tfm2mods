# -*- coding: utf-8 -*-
"""verify_athlete_054.py — athlete 구조체의 id 오프셋을 0.5.3/0.5.4 exe 로 직접 대조한다.

왜: tfm2_item_tactics 의 팀 조인(is_my_athlete)은 `athlete + O_ATHLETE_ID` 를 MY_ATHLETES 와
    맞춰본다. 0.5.4 에서 athlete 이 -0x10 시프트했다는 판정에 따라 0x810 -> 0x800 으로 바꿨는데,
    이게 틀리면 **id 대신 team 을 읽어 조인이 조용히 전부 실패**한다(크래시 없음, 지정만 사라짐).
    그래서 모드 소스가 아니라 **게임 코드가 실제로 무엇을 읽는지**로 확인한다.

방법: 게임의 로스터 순회 패턴을 찾는다.
    stride 상수(0.5.3=0x8d0 / 0.5.4=0x8c0)를 쓰는 imul/add 명령을 전부 찾고,
    그 근처에서 [reg+disp] 로 읽는 disp 들을 모아 히스토그램을 낸다.
    id 는 순회 직후 읽히는 대표 필드라 상위에 나와야 한다.
"""
import io, sys, collections
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

TARGETS = [
    ("0.5.3", r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe", 0x8d0),
    ("0.5.4", r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\TeamfightManager2.exe", 0x8c0),
]
BASE = 0x140000000
WINDOW = 40          # stride 명령 이후 몇 개 명령까지 볼지


def load_text(path):
    pe = pefile.PE(path, fast_load=True)
    for s in pe.sections:
        if s.Name.rstrip(b"\x00") == b".text":
            return s.get_data(), BASE + s.VirtualAddress
    raise SystemExit("no .text")


for label, path, stride in TARGETS:
    data, va = load_text(path)
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = True
    print(f"\n===== {label}  stride=0x{stride:x}  .text {len(data):,}B")

    # 1) stride 를 즉값으로 쓰는 명령 위치 수집 (imul r,r,imm / add r,imm / lea 등)
    #    선형 디스어셈은 비싸므로, 먼저 바이트로 후보를 좁힌다.
    imm = stride.to_bytes(4, "little")
    cand = []
    start = 0
    while True:
        i = data.find(imm, start)
        if i < 0:
            break
        cand.append(i)
        start = i + 1
    print(f"  stride 즉값 바이트 출현 {len(cand):,}곳")

    # 2) 각 후보 주변을 디스어셈해서 진짜 stride 명령인지 확인하고, 이후 disp 를 모은다
    hist = collections.Counter()
    sites = 0
    for off in cand:
        s = max(0, off - 16)
        chunk = data[s: off + 8 + WINDOW * 8]
        insns = list(md.disasm(chunk, va + s))
        # stride 를 즉값으로 갖는 명령을 찾는다
        k = None
        for idx, ins in enumerate(insns):
            if ins.address <= va + off < ins.address + ins.size:
                # 이 명령이 stride 를 immediate 로 쓰는가
                if f"0x{stride:x}" in ins.op_str and ins.mnemonic in ("imul", "add", "lea", "mov"):
                    k = idx
                break
        if k is None:
            continue
        sites += 1
        # 이후 WINDOW 개 명령에서 [reg + disp] 의 disp 수집 (athlete 필드 대역만)
        for ins in insns[k + 1: k + 1 + WINDOW]:
            for op in ins.operands:
                if op.type == 3:  # X86_OP_MEM
                    d = op.mem.disp
                    if 0x700 <= d <= 0x900:
                        hist[d] += 1
    print(f"  진짜 stride 명령 {sites}곳")
    print(f"  그 직후 읽는 athlete 대역(0x700~0x900) disp 상위:")
    for d, n in hist.most_common(12):
        print(f"      +0x{d:<5x} {n:>5}회")
