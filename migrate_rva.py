# -*- coding: utf-8 -*-
# migrate_rva.py — TFM2 패치(핫픽스/버전업) 때 구 exe RVA → 새 exe RVA 자동 마이그레이터.
#   원리: 구 함수 바디를 디스어셈→ rip-relative/상대분기 바이트는 와일드카드, 나머지 고정 = "마스크 시그니처".
#         핫픽스는 함수 바디가 거의 동일(주소만 이동)하므로 새 exe 에서 유일 매치로 재탐색.
#   다중매치(제네릭 모노모픽 게터 등)는 string-xref 로 확정 → find_callee_of_string().
# 사용: python migrate_rva.py   (OLD/NEW 경로 + TARGETS 수정해서)
import struct, sys
from capstone import *
from capstone.x86 import X86_OP_MEM, X86_REG_RIP, X86_OP_IMM, X86_GRP_JUMP, X86_GRP_CALL
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True

# OLD = 직전 베이스(=현재 모드 소스의 RVA가 맞는 exe). 패치 받으면 ① 먼저 직전 게임exe를 새 폴더에 백업 →
#   ② OLD 경로를 그 백업으로, TARGETS 의 RVA를 새(직전) 값으로 갱신 → ③ python migrate_rva.py.
OLD = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.0_3\TeamfightManager2.exe"  # 0.5.0_3(69,047,296, buildid 24125999) — 현재 모드소스 베이스
NEW = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.1\TeamfightManager2.exe"  # 0.5.1(69,233,664, Steam 라이브와 동일)
# (구RVA=0.5.0_3, 이름) — 구RVA = OLD 베이스의 현재값(=모드 소스 상수와 동일). 마이그 후 신값으로 갱신해둘 것.
# ★byte-patch 사이트(mid-func imm) 및 점프-앵커는 마스크시그 부적합 → 컨테이너-델타(mig503*.py 방식)로. LOADER/ANIM=asset-getter MULTI→string-xref(layout/main).
TARGETS = [
    # ai_adjust (function-start hooks, 전부 UNIQUE)
    (0x1f37f70, "ai:RETREAT"), (0x22db820, "ai:GENERIC_BUILD"), (0x1f553a0, "ai:FC59A0"),
    (0x1f54d10, "ai:PREGATE"), (0x2375b90, "ai:F80320"), (0x19e40e0, "ai:CONDGATE"),
    (0x19e4a50, "ai:MOVEPRI"), (0x19e7d30, "ai:COMMIT_FN"), (0x22e85a0, "ai:COMBAT_FN"),
    (0x20a5030, "ai:TTD"),
    (0x1c83700, "ai:DISC19_HANDLER(FUN_141c83700)"), (0x1c81980, "ai:DISC18_HANDLER"),
    # item_editor
    (0x1fc12b0, "item_editor:STAT_FN"), (0x1fc0960, "item_editor:PER_ITEM"), (0x1fc1560, "item_editor:SUM"),
    # scrim / draft_overlay / item_tactics (공유 UI 엔진)
    (0x2416070, "scrim/it:DD_SETOPT"), (0x1b78420, "scrim:ITEMNET_FWD"),
    (0x2499f30, "PARSER"), (0x25ab3d0, "ALLOC"), (0x25ab430, "DEALLOC"),
    (0x51cd40, "LOADER (MULTI→string-xref layout/main)"), (0x51bbc0, "draft_overlay:ANIM_GET (LOADER-0x1180)"),
    # item_tactics
    (0x1fb8b10, "it:BUY_ITEM"), (0x25ab470, "it:REALLOC"), (0xb8d100, "it:SLOT_HELPER"),
    (0x14aa3db, "it:C6_SEAM(resume=+0xf)"), (0x20eb870, "it:owned_cap_sig(imm=+7)"),
    (0x1fb8cdd, "it:gate3_sig(jbe=+9)"),
    # comptest_unlock (클라측 resolved; 서버영역 0x13xxxxx=ghidra-re 후속)
    (0xf687e0, "ct:RUN_handler"), (0xf76b00, "ct:LOADING"), (0x1a73f30, "ct:daily_remaining"),
    (0xf5f290, "ct:FORGE_CALLER"), (0xc57100, "ct:DISP"),
]

def load(p):
    d = open(p, "rb").read(); pe = struct.unpack_from("<I", d, 0x3c)[0]
    nsec = struct.unpack_from("<H", d, pe + 6)[0]; opt = pe + 24
    ib = struct.unpack_from("<Q", d, opt + 24)[0]; sectab = opt + struct.unpack_from("<H", d, pe + 20)[0]
    secs = []
    for i in range(nsec):
        o = sectab + i * 40; nm = d[o:o + 8].rstrip(b"\0").decode(errors="replace")
        vsz, va, rsz, rraw = struct.unpack_from("<IIII", d, o + 8); secs.append((nm, va, vsz, rraw, rsz))
    return d, ib, secs
def roff(secs, rva):
    for nm, va, vsz, rraw, rsz in secs:
        if va <= rva < va + max(vsz, rsz): return rraw + (rva - va)
def text_sec(secs):
    for nm, va, vsz, rraw, rsz in secs:
        if nm == ".text": return va, rraw, rsz

def make_pattern(d, ib, secs, rva, nbytes):
    off = roff(secs, rva); code = d[off:off + nbytes + 16]
    pat = bytearray(); mask = bytearray()
    for ins in md.disasm(code, ib + rva):
        if len(pat) >= nbytes: break
        wild = any(op.type == X86_OP_MEM and op.mem.base == X86_REG_RIP for op in ins.operands)
        if (ins.group(X86_GRP_JUMP) or ins.group(X86_GRP_CALL)) and any(op.type == X86_OP_IMM for op in ins.operands): wild = True
        for bb in ins.bytes: pat.append(bb); mask.append(0 if wild else 1)
    return bytes(pat), bytes(mask)
def find(text, base, pat, mask):
    pre = bytearray()
    for b, m in zip(pat, mask):
        if m: pre.append(b)
        else: break
    pre = bytes(pre); hits = []; s = 0
    while True:
        i = text.find(pre, s)
        if i < 0: break
        s = i + 1; seg = text[i:i + len(pat)]
        if len(seg) == len(pat) and all(mask[j] == 0 or seg[j] == pat[j] for j in range(len(pat))): hits.append(base + i)
    return hits

def find_callee_of_string(nd, nib, nsecs, needle):
    """needle(bytes) 문자열을 LEA(rip)로 로드한 직후 call 하는 함수 RVA 들을 반환 (다중게터 확정용)."""
    tva, traw, trsz = text_sec(nsecs); text = nd[traw:traw + trsz]
    str_rvas = set(); s = 0
    while True:
        i = nd.find(needle, s)
        if i < 0: break
        for nm, va, vsz, rraw, rsz in nsecs:
            if rraw <= i < rraw + rsz: str_rvas.add(va + (i - rraw))
        s = i + 1
    modrm = {0x05, 0x0d, 0x15, 0x1d, 0x2d, 0x35, 0x3d}
    callees = {}
    for i in range(len(text) - 7):
        if text[i] in (0x48, 0x4c) and text[i + 1] == 0x8d and text[i + 2] in modrm:
            disp = struct.unpack_from("<i", text, i + 3)[0]
            if (tva + i + 7 + disp) in str_rvas:
                code = text[i:i + 260]
                for ins in md.disasm(code, tva + i):
                    if ins.group(X86_GRP_CALL) and ins.operands and ins.operands[0].type == X86_OP_IMM:
                        callees[ins.operands[0].imm] = callees.get(ins.operands[0].imm, 0) + 1; break
    return callees

if __name__ == "__main__":
    od, oib, os_ = load(OLD); nd, nib, ns = load(NEW)
    nva, nraw, nrsz = text_sec(ns); ntext = nd[nraw:nraw + nrsz]
    for rva, nm in TARGETS:
        if roff(os_, rva) is None: print(f"{nm:40s} old={hex(rva)} NOT-in-old-text"); continue
        pat, mask = make_pattern(od, oib, os_, rva, 0xa0)
        hits = find(ntext, nva, pat, mask)
        tag = "OK" if len(hits) == 1 else ("MULTI(string-xref 필요)" if len(hits) > 1 else "NONE")
        print(f"{nm:40s} old={hex(rva)} -> {[hex(h) for h in hits][:6]} {tag}")
    # 다중게터 확정 예시:
    c = find_callee_of_string(nd, nib, ns, b"asset/base/ui/layout/main")
    print("layout 'main' 호출 게터:", {hex(k): v for k, v in c.items()})
