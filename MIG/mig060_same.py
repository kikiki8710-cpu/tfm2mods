# -*- coding: utf-8 -*-
"""
mig060_same.py — mig060 가 「동일(skel)/이동만/오프셋 이동/오프셋만」 으로 매긴 짝을 **명령 단위 정렬 대조**로 확정한다.

왜: fnindex 의 skel 은 op_str 의 숫자를 전부 I 로 치환한 해시라 「동일(skel)」 이어도 즉치·변위·rip-rel 상수 데이터·콜리가 전부 달라도
    같은 해시가 나온다(노브 상수 5→7 도 못 잡는다). 그래서 정렬된 명령 짝마다 피연산자를 실제로 비교한다.
무엇을 보나(정렬 = difflib 로 skel 토큰열 정렬):
  ① 구조: 삽입/삭제/니모닉 불일치 · 함수 내부 분기 타깃의 정렬 인덱스 불일치 → 「구조 변경」
  ② 즉치: 값 불일치 → 델타 목록(전부 소수의 델타로 설명되면 「오프셋 이동」, 아니면 「상수 변경」)
  ③ 변위: [base+disp] 의 disp 불일치 → 델타 목록(②와 같이 판정)
  ④ rip-rel 데이터: movss/movsd/mov… [rip+x] 는 가리키는 바이트(op.size)를 두 exe 에서 읽어 비교(부동소수 상수·정적 테이블) → 「데이터 변경」
      lea [rip+x] 는 Location(파일,열 비교 · 줄은 무시) / 함수 포인터(콜리 매핑) / 문자열·정적(뒤따르는 길이 즉치로 길이 추정해 비교)
  ⑤ 콜리: call/jmp 타깃 → 구 콜리의 0.6.0 짝(mig060 결과 → skel 유일 → Location 정확 유일) 과 실제 타깃 대조 · 콜리 자체의 등급(변경이면 표기)
      IAT 경유 call [rip+x] 는 import 이름 비교
출력: _next\mig060_same.{json,md}
사용: python mig060_same.py [--all]   (기본 = spec 만 · --all = 769 전부)
"""
import io, json, os, sys, struct, pickle, collections, difflib, re
sys.stdout.reconfigure(encoding="utf-8")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mig060 as M
from capstone.x86 import X86_OP_IMM, X86_OP_MEM, X86_OP_REG, X86_REG_RIP

HERE = M.HERE
RE_NUM = re.compile(r"0x[0-9a-fA-F]+|\b\d+\b")
OKV = (u"동일", u"이동만", u"오프셋 이동", u"오프셋만")
KNOWN = {(0x2e8, 0x5c8): "Blackboard stride", (0x9e0, 0xab0): "Player stride", (0x320, 0x350): "DieTick?", (0xfa0, 0x1090): "DieTick arr?", (0x148, 0x168): "0x148→0x168"}

class X(M.Exe):
    def __init__(s, path, tag):
        M.Exe.__init__(s, path, tag); s.build_locidx()
        s.pe.parse_data_directories(directories=[1])   # import
        s.iat = {}
        for e in getattr(s.pe, "DIRECTORY_ENTRY_IMPORT", []):
            for im in e.imports:
                if im.address: s.iat[im.address - M.BASE] = (e.dll.decode(errors="replace"), (im.name or b"#%d" % (im.ordinal or 0)).decode(errors="replace"))
    def insns(s, f):
        return list(s.md.disasm(s.img[f:s.ends[f]], f))
    def rd(s, r, n):
        try: return bytes(s.img[r:r + n])
        except Exception: return b""
    def in_text(s, r): return s.in_sec(r, ".text")

def tok(ins): return ins.mnemonic + "|" + RE_NUM.sub("I", ins.op_str)
def regname(md, r): return md.reg_name(r)

def main():
    allrows = "--all" in sys.argv
    O = X(M.OLD, "old"); N = X(M.NEW, "new")
    FO = {(int(k, 16) if isinstance(k, str) else k): v for k, v in pickle.load(open(M.PKL["old"], "rb"))["idx"].items()}
    FN = {(int(k, 16) if isinstance(k, str) else k): v for k, v in pickle.load(open(M.PKL["new"], "rb"))["idx"].items()}
    by_skel_new = collections.defaultdict(list)
    for k, v in FN.items(): by_skel_new[v["skel"]].append(k)
    J = json.load(io.open(os.path.join(HERE, "_next", "mig060_judge.json"), encoding="utf-8"))
    JM = {int(r["old"], 16): r for r in J}
    # 콜리 매핑: mig060 결과 → skel 유일(크기 동일) → Location 정확 집합 유일
    loc_sig_new = collections.defaultdict(list)
    for f, L in N.fnloc.items(): loc_sig_new[M.sig_exact(L)].append(f)
    loc_fc_new = collections.defaultdict(list)
    for f, L in N.fnloc.items(): loc_fc_new[M.sig_fc(L)].append(f)
    mapc = {}
    def map_callee(t):
        if t in mapc: return mapc[t]
        r = None
        if t in JM and "new" in JM[t]: r = (int(JM[t]["new"], 16), u"mig060:" + JM[t]["verdict"].split("(")[0])
        elif t in FO:
            c = by_skel_new.get(FO[t]["skel"], [])
            c2 = [x for x in c if FN[x]["size"] == FO[t]["size"]]
            if len(c2) == 1: r = (c2[0], u"skel유일")
            elif t in O.fnloc:
                c = loc_sig_new.get(M.sig_exact(O.fnloc[t]), [])
                if len(c) == 1: r = (c[0], u"Loc정확유일")
                else:
                    c = loc_fc_new.get(M.sig_fc(O.fnloc[t]), [])
                    if len(c) == 1: r = (c[0], u"Loc(파일,열)유일")
        mapc[t] = r; return r
    def callee_grade(to, tn):
        """실제 타깃 짝 (to→tn) 의 등급"""
        if to in JM and "new" in JM[to]:
            if int(JM[to]["new"], 16) == tn: return JM[to]["verdict"].split("(")[0]
            return u"불일치(mig060 는 %s)" % JM[to]["new"]
        fo, fn = FO.get(to), FN.get(tn)
        if fo and fn and fo["skel"] == fn["skel"]: return u"skel동일"
        if to in O.fnloc and tn in N.fnloc:
            a, b = set((f, c) for f, l, c in O.fnloc[to]), set((f, c) for f, l, c in N.fnloc[tn])
            j = len(a & b) / max(1, len(a | b))
            if j >= 0.5 and fo and fn and abs(fo["size"] - fn["size"]) <= fo["size"] * 0.05: return u"Loc유사(J%.2f·%+dB)" % (j, fn["size"] - fo["size"])
            return u"콜리 변경?(J%.2f·%+dB)" % (j, (fn["size"] - fo["size"]) if fo and fn else 0)
        if fo and fn: return u"미지(%+dB)" % (fn["size"] - fo["size"])
        return u"미지"

    OKG = (u"동일", u"이동만", u"오프셋", u"skel동일", u"Loc유사", u"소폭")
    targets = [r for r in J if "new" in r and r["verdict"].split("(")[0] in OKV and (allrows or r["src"] == "spec")]
    if "--pair" in sys.argv:   # --pair OLD NEW : 등급 무관 한 짝만(변경 함수의 차이 내역 보기)
        po, pn = sys.argv[sys.argv.index("--pair") + 1].lower(), sys.argv[sys.argv.index("--pair") + 2].lower()
        base = JM.get(int(po, 16), {}); targets = [dict(base, old=po, new=pn, name=base.get("name", "FUN_" + po), src=base.get("src", "?"), verdict=base.get("verdict", u"?"))]
    print(u"대상 %d" % len(targets))
    out = []; mismatch = collections.Counter()
    for r in targets:
        a, b = int(r["old"], 16), int(r["new"], 16)
        IO, IN = O.insns(a), N.insns(b)
        TO, TN = [tok(x) for x in IO], [tok(x) for x in IN]
        sm = difflib.SequenceMatcher(None, TO, TN, autojunk=False)
        pairs = []; struct_diff = []; moved = 0
        rest_o = []; rest_n = []
        for op, i1, i2, j1, j2 in sm.get_opcodes():
            if op == "equal": pairs += list(zip(range(i1, i2), range(j1, j2)))
            elif op == "replace" and i2 - i1 == j2 - j1 and all(IO[i].mnemonic == IN[j].mnemonic for i, j in zip(range(i1, i2), range(j1, j2))):
                pairs += list(zip(range(i1, i2), range(j1, j2)))
            else:
                rest_o += list(range(i1, i2)); rest_n += list(range(j1, j2))
        # 블록 이동(컴파일러 레이아웃): 남은 구/신 명령열을 다시 정렬 — 토큰이 같은 런은 이동 블록으로 짝짓는다
        rounds = 0
        while rest_o and rest_n and rounds < 6:   # 블록 회전(A B → B A)은 한 번에 안 잡히므로 남은 것끼리 반복 정렬
            rounds += 1
            sm2 = difflib.SequenceMatcher(None, [TO[i] for i in rest_o], [TN[j] for j in rest_n], autojunk=False)
            nro, nrn, got = [], [], 0
            for op, i1, i2, j1, j2 in sm2.get_opcodes():
                if op == "equal" or (op == "replace" and i2 - i1 == j2 - j1 and all(IO[rest_o[i]].mnemonic == IN[rest_n[j]].mnemonic for i, j in zip(range(i1, i2), range(j1, j2)))):
                    pairs += [(rest_o[i], rest_n[j]) for i, j in zip(range(i1, i2), range(j1, j2))]; moved += i2 - i1; got += i2 - i1
                else:
                    nro += rest_o[i1:i2]; nrn += rest_n[j1:j2]
            rest_o, rest_n = nro, nrn
            if not got: break
        if rest_o and rest_n:
            struct_diff.append(u"구 %d명령(%s…) / 신 %d명령(%s…) 짝 없음" % (len(rest_o), TO[rest_o[0]][:30], len(rest_n), TN[rest_n[0]][:30]))
        elif rest_o or rest_n:
            struct_diff.append(u"구 %d명령 / 신 %d명령 짝 없음" % (len(rest_o), len(rest_n)))
        pairs.sort(); amap = dict(pairs)
        addr_idx_o = {x.address: i for i, x in enumerate(IO)}; addr_idx_n = {x.address: i for i, x in enumerate(IN)}
        imm_d = []; disp_d = []; data_d = []; lea_d = []; reg_d = []; br_d = []; callees = []; typeid_d = []; enum_d = []; loc_ok = 0; data_ok = 0; stack_d = 0; panic_re = 0
        for i, j in pairs:
            x, y = IO[i], IN[j]
            if len(x.operands) != len(y.operands): struct_diff.append(u"%x 피연산자 수" % x.address); continue
            for k, (oa, ob) in enumerate(zip(x.operands, y.operands)):
                if oa.type != ob.type: struct_diff.append(u"%x 피연산자 형" % x.address); continue
                if oa.type == X86_OP_REG:
                    if oa.reg != ob.reg: reg_d.append(u"%x %s→%s" % (x.address, regname(O.md, oa.reg), regname(N.md, ob.reg)))
                elif oa.type == X86_OP_IMM:
                    if x.mnemonic == "call" or x.mnemonic.startswith("j"):
                        to, tn = oa.imm, ob.imm   # disasm 을 RVA 기준으로 돌렸으므로 imm 도 RVA
                        if to in addr_idx_o or tn in addr_idx_n:   # 함수 내부 분기
                            ei = amap.get(addr_idx_o.get(to, -1)); ai = addr_idx_n.get(tn)
                            if ei is None or ai is None or ei != ai:
                                # 두 타깃이 모두 패닉 스텁(lea Location … call/ud2)이면 동형 스텁 재배열 — 의미 無
                                def is_stub(E, L, idx):
                                    if idx is None: return False
                                    seen_loc = False
                                    for z in L[idx: idx + 7]:
                                        if z.mnemonic == "lea" and len(z.operands) == 2 and z.operands[1].type == X86_OP_MEM and z.operands[1].mem.base == X86_REG_RIP and E.try_loc(z.address + z.size + z.operands[1].mem.disp): seen_loc = True
                                        if z.mnemonic in ("call", "ud2", "int3"): return seen_loc
                                    return False
                                if is_stub(O, IO, addr_idx_o.get(to)) and is_stub(N, IN, addr_idx_n.get(tn)): panic_re += 1
                                else: br_d.append(u"%x → %x/%x" % (x.address, to, tn))
                        else:
                            callees.append((to, tn, callee_grade(to, tn)))
                    elif oa.imm != ob.imm: imm_d.append((x.address, oa.imm, ob.imm))
                elif oa.type == X86_OP_MEM:
                    if oa.mem.base == X86_REG_RIP and ob.mem.base == X86_REG_RIP:
                        to = x.address + x.size + oa.mem.disp; tn = y.address + y.size + ob.mem.disp
                        if to in O.iat or tn in N.iat:
                            io_, in_ = O.iat.get(to), N.iat.get(tn)
                            if io_ != in_: struct_diff.append(u"%x IAT %s≠%s" % (x.address, io_, in_))
                            else: data_ok += 1
                            continue
                        if O.in_text(to) and O.owner(to) is None and N.in_text(tn) and N.owner(tn) is None:
                            # .pdata 밖 thunk(memcpy 류): 16B 바이트 비교
                            if O.rd(to, 16) == N.rd(tn, 16): data_ok += 1
                            else: callees.append((to, tn, u"미지(pdata 밖 thunk)"))
                            continue
                        if O.in_text(to) and O.owner(to) is not None:
                            callees.append((O.owner(to), N.owner(tn), callee_grade(O.owner(to), N.owner(tn)) if N.owner(tn) is not None else u"미지")); continue
                        lo, ln = O.try_loc(to), N.try_loc(tn)
                        if lo or ln:
                            if lo and ln and lo[0] == ln[0] and lo[2] == ln[2]: loc_ok += 1
                            else: lea_d.append(u"%x Location %s ≠ %s" % (x.address, lo, ln))
                            continue
                        if x.mnemonic == "lea" and any(IO[i + d].mnemonic == "movsxd" for d in (1, 2, 3) if i + d < len(IO)):
                            # 점프 테이블: 항목(i32 상대) + 테이블 base = 타깃 → 정렬 인덱스 비교
                            fo_, fn_ = a, b; eo, en = O.ends[a], N.ends[b]; k = 0; bad = 0; nent = 0
                            # 경계 = 직전 6명령 안의 cmp r, imm(+1) — 구/신이 다르면 case 수 변경
                            def bound(L, i0):
                                for d in range(1, 7):
                                    if i0 - d >= 0 and L[i0 - d].mnemonic in ("cmp", "mov") and len(L[i0 - d].operands) == 2 and L[i0 - d].operands[1].type == X86_OP_IMM and 0 < L[i0 - d].operands[1].imm < 256 and L[i0 - d].operands[0].type == X86_OP_REG: return L[i0 - d].operands[1].imm + 1   # cmp r,N ; ja  또는  mov r,N ; cmovae(니치 enum 기본 인덱스)
                                return None
                            bo_, bn_ = bound(IO, i), bound(IN, j)
                            if bo_ and bn_ and bo_ != bn_:
                                def tgt(E, base, kk, f0, e0):
                                    v = E.rd(base + 4 * kk, 4)
                                    if len(v) < 4: return None
                                    t = base + struct.unpack("<i", v)[0]
                                    return t if f0 <= t < e0 else None
                                found = None
                                if bo_ == bn_ + 1:
                                    for kdel in range(bo_):
                                        ok = True
                                        for ko in range(bo_):
                                            if ko == kdel: continue
                                            kn = ko - (1 if ko > kdel else 0)
                                            po_, pn_ = tgt(O, to, ko, a, O.ends[a]), tgt(N, tn, kn, b, N.ends[b])
                                            ei = amap.get(addr_idx_o.get(po_, -1)); ai = addr_idx_n.get(pn_)
                                            if po_ is None or pn_ is None or ei is None or ai is None or ei != ai: ok = False; break
                                        if ok: found = kdel; break
                                if found is not None: enum_d.append(u"%x switch %d→%d case #%d 제거 · 나머지 %d case 타깃 동일" % (x.address, bo_, bn_, found, bn_))
                                else: br_d.append(u"%x switch case 수 %d→%d(정렬 실패)" % (x.address, bo_, bn_))
                                continue
                            lim = bo_ or bn_ or 512; pat = []
                            while k < lim:
                                vo = O.rd(to + 4 * k, 4); vn = N.rd(tn + 4 * k, 4)
                                if len(vo) < 4 or len(vn) < 4: break
                                po_ = to + struct.unpack("<i", vo)[0]; pn_ = tn + struct.unpack("<i", vn)[0]
                                if not (fo_ <= po_ < eo and fn_ <= pn_ < en): break
                                nent += 1
                                ei = amap.get(addr_idx_o.get(po_, -1)); ai = addr_idx_n.get(pn_)
                                if addr_idx_o.get(po_) is None and addr_idx_n.get(pn_) is None:   # 선형 디스어셈이 못 잡은 타깃: 그 자리 4명령 skel 토큰 비교
                                    t1 = [tok(z) for _, z in zip(range(4), O.md.disasm(O.rd(po_, 40), po_))]; t2 = [tok(z) for _, z in zip(range(4), N.md.disasm(N.rd(pn_, 40), pn_))]
                                    pat.append(t1 == t2 and len(t1) > 0)
                                else: pat.append(ei is not None and ai is not None and ei == ai)
                                k += 1
                            bad = pat.count(False)
                            if bad and lim == 512 and len(pat) >= 3:
                                # 경계 미상: 앞쪽 연속 일치 + 꼬리 불일치면 꼬리는 인접 테이블(같은 함수의 다른 switch)
                                head_ok = 0
                                while head_ok < len(pat) and pat[head_ok]: head_ok += 1
                                if head_ok >= 3: bad = 0; nent = head_ok   # 첫 불일치 이후는 인접 테이블로 간주(경계 미상 · 명령열은 이미 정렬됨)
                            if bad: br_d.append(u"%x 점프테이블 %d/%d 항목 불일치" % (x.address, bad, nent))
                            else: data_ok += 1
                            continue
                        if x.mnemonic == "lea":
                            # 길이 추정: 앞뒤 3명령 안의 작은 즉치(같은 정렬 위치)
                            ln_ = None
                            for d in (1, 2, 3, -1):
                                if 0 <= i + d < len(IO) and IO[i + d].mnemonic == "mov" and len(IO[i + d].operands) == 2 and IO[i + d].operands[1].type == X86_OP_IMM and 1 <= IO[i + d].operands[1].imm <= 512:
                                    ln_ = IO[i + d].operands[1].imm; break
                            n = ln_ or 16
                            bo, bn = O.rd(to, n), N.rd(tn, n)
                            if bo == bn: data_ok += 1
                            else:
                                # 8B 포인터(vtable·정적)면 가리키는 곳 한 단계 더
                                po = struct.unpack_from("<Q", bo + b"\0" * 8)[0] - M.BASE if len(bo) >= 8 else -1
                                pn = struct.unpack_from("<Q", bn + b"\0" * 8)[0] - M.BASE if len(bn) >= 8 else -1
                                if 0 < po < len(O.img) and 0 < pn < len(N.img) and O.owner(po) is not None and N.owner(pn) is not None:
                                    g_ = callee_grade(O.owner(po), N.owner(pn))
                                    if g_.startswith(u"불일치"): g_ = u"동일(vtable slot0 · ICF 추정)"   # vtable 첫 슬롯(drop) 은 동일 함수 접힘으로 짝이 흔들린다
                                    callees.append((O.owner(po), N.owner(pn), u"ptr:" + g_)); continue
                                # 포인터 쌍(fn ptr + static / data ptr + vtable): 8B 단위로 VA 면 각각 판정
                                halves = []
                                for hh in range(0, min(len(bo), len(bn)) // 8 * 8, 8):
                                    qo = struct.unpack_from("<Q", bo, hh)[0] - M.BASE; qn = struct.unpack_from("<Q", bn, hh)[0] - M.BASE
                                    if not (0 < qo < len(O.img) and 0 < qn < len(N.img)): halves = None; break
                                    if O.in_text(qo) and O.owner(qo) is not None and N.owner(qn) is not None: halves.append(u"fn:" + callee_grade(O.owner(qo), N.owner(qn)))
                                    elif O.in_text(qo): halves.append(u"fn:pdata밖")
                                    elif O.in_sec(qo, ".rdata") and N.in_sec(qn, ".rdata"): halves.append(u"rdata:" + (u"동일32B" if O.rd(qo, 32) == N.rd(qn, 32) else u"다름"))
                                    else: halves.append(u"static")
                                if halves:
                                    bad = [h for h in halves if h.endswith(u"다름") or (h.startswith(u"fn:") and not h.replace(u"fn:", u"").startswith(OKG))]
                                    if not bad: data_ok += 1
                                    else: lea_d.append(u"%x lea 포인터쌍 %s" % (x.address, u"·".join(halves)))
                                    continue
                                pre = 0
                                while pre < min(len(bo), len(bn)) and bo[pre] == bn[pre]: pre += 1
                                lea_d.append(u"%x lea %dB(접두 동일 %dB) %s ≠ %s" % (x.address, n, pre, bo[:16].hex(), bn[:16].hex()))
                        else:
                            n = oa.size or 8
                            bo, bn = O.rd(to, n), N.rd(tn, n)
                            if bo == bn: data_ok += 1
                            else:
                                fmt = {4: "<f", 8: "<d"}.get(n)
                                if n == 16 and x.mnemonic in ("pcmpeqb", "pcmpeqd", "pxor", "vpcmpeqb"): typeid_d.append(u"%x %s 16B(TypeId?)" % (x.address, x.mnemonic))
                                elif fmt and len(bo) == n and len(bn) == n:
                                    data_d.append(u"%x %s %dB %g→%g (%s→%s)" % (x.address, x.mnemonic, n, struct.unpack(fmt, bo)[0], struct.unpack(fmt, bn)[0], bo.hex(), bn.hex()))
                                else: data_d.append(u"%x %s %dB %s→%s" % (x.address, x.mnemonic, n, bo.hex(), bn.hex()))
                    elif oa.mem.base == X86_REG_RIP or ob.mem.base == X86_REG_RIP:
                        struct_diff.append(u"%x rip 여부" % x.address)
                    else:
                        if oa.mem.base != ob.mem.base or oa.mem.index != ob.mem.index: reg_d.append(u"%x mem 레지스터" % x.address)
                        if oa.mem.scale != ob.mem.scale: struct_diff.append(u"%x scale" % x.address)
                        if oa.mem.disp != ob.mem.disp:
                            rg = regname(O.md, oa.mem.base)
                            if rg in ("rsp", "rbp") and abs(oa.mem.disp) < 0x2000 and abs(ob.mem.disp) < 0x2000: stack_d += 1   # 스택 슬롯 재배치(의미 無)
                            else: disp_d.append((x.address, rg, oa.mem.disp, ob.mem.disp))
        # 델타 분석
        deltas = collections.Counter()
        known = collections.Counter()
        for _, o, n in imm_d:
            deltas[n - o] += 1
            if (o, n) in KNOWN: known[KNOWN[(o, n)]] += 1
        for _, _, o, n in disp_d:
            deltas[n - o] += 1
            if (o, n) in KNOWN: known[KNOWN[(o, n)]] += 1
        cg = collections.Counter(g.split("(")[0].replace("ptr:", "") for _, _, g in callees)
        bad_callee = sorted(set(u"%x→%x %s" % (to, tn, g) for to, tn, g in callees if not g.replace("ptr:", "").startswith(OKG)))
        # 소형 즉치(열거 태그·카운트·노브) 변경 = |구| < 0x100 이고 델타가 작다 → 레이아웃 이동이 아니라 의미 변경 후보
        small = [(hex(a_), o, n) for a_, o, n in imm_d if abs(o) < 0x100 and abs(n - o) <= 8]
        small += [(hex(a_), o, n) for a_, rg, o, n in disp_d if abs(o) < 0x40 and abs(n - o) <= 8 and rg in ("rsp", "rbp") and False]
        layout = [(o, n) for _, o, n in imm_d if (hex(0), o, n) not in small and abs(o) >= 0x100] + [(o, n) for _, _, o, n in disp_d]
        if struct_diff or br_d: v = u"❌구조 변경"
        elif data_d: v = u"⚠데이터 변경"
        elif small: v = u"⚠소형 즉치 변경(%s)" % u"·".join(u"%x→%x" % (o, n) for _, o, n in small[:5])
        elif imm_d or disp_d: v = u"✅오프셋만(Δ%s)" % u"·".join(hex(d) for d, _ in deltas.most_common(5))
        elif [z for z in lea_d if u"Location" not in z]: v = u"⚠lea 데이터 차이(검토)"
        else: v = u"✅동일"
        if moved: v += u" · 블록이동 %d" % moved
        if stack_d: v += u" · 스택슬롯 %d" % stack_d
        if panic_re: v += u" · 패닉스텁 재배열 %d" % panic_re
        if typeid_d: v += u" · TypeId %d" % len(typeid_d)
        if enum_d: v += u" · switch case 제거 %d" % len(enum_d)
        for to, tn, g in callees:
            if g.startswith(u"불일치") and not g.startswith(u"ptr:"): mismatch[(to, tn)] += 1
        if bad_callee: v += u" · 콜리 주의 %d" % len(bad_callee)
        row = dict(old=r["old"], new=r["new"], name=r["name"], src=r["src"], v0=r["verdict"].split("(")[0], strict=v, ninsn=[len(IO), len(IN)], aligned=len(pairs),
                   struct_diff=struct_diff[:10], branch_diff=br_d[:10], imm=[(hex(a_), hex(o), hex(n)) for a_, o, n in imm_d[:20]], small=[(a_, hex(o), hex(n)) for a_, o, n in small], typeid=typeid_d[:5], moved=moved, stack=stack_d, enum=enum_d, disp=[(hex(a_), rg, hex(o), hex(n)) for a_, rg, o, n in disp_d[:20]],
                   data=data_d[:20], lea=lea_d[:10], reg=len(reg_d), loc_ok=loc_ok, data_ok=data_ok, deltas={hex(k): c for k, c in deltas.items()}, known=dict(known),
                   callees=len(callees), callee_grades=dict(cg), bad_callee=bad_callee[:12])
        out.append(row)
        print(u"%s→%s %-38s %-14s → %s" % (r["old"], r["new"], r["name"][:38], row["v0"], v))
    json.dump(out, io.open(os.path.join(HERE, "_next", "mig060_same%s.json" % ("_all" if allrows else ("_pair" if "--pair" in sys.argv else ""))), "w", encoding="utf-8"), ensure_ascii=False, indent=0)
    # 호출부 실측이 mig060 짝과 다른 콜리 = mig060 정정 후보(호출부 합의)
    fix = collections.defaultdict(collections.Counter)
    for (to, tn), c in mismatch.items(): fix[to][tn] += c
    fixes = [dict(old="%x" % to, was=JM[to]["new"], now="%x" % max(c, key=c.get), votes=dict((("%x" % k), v) for k, v in c.items()), name=JM[to]["name"]) for to, c in fix.items()]
    json.dump(fixes, io.open(os.path.join(HERE, "_next", "mig060_same_fix%s.json" % ("_all" if allrows else ("_pair" if "--pair" in sys.argv else ""))), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(u"mig060 짝 정정 후보(호출부 실측) %d: " % len(fixes) + u" · ".join(u"%s %s→%s" % (f["old"], f["was"], f["now"]) for f in fixes))
    C = collections.Counter(x["strict"].split(u" ·")[0].split("(")[0] for x in out)
    C2 = collections.Counter((u"콜리 주의" if x["bad_callee"] else u"콜리 전부 OK") for x in out)
    L = [u"# mig060 동일/이동/오프셋 짝의 명령 단위 확정 대조 (%d · %s)" % (len(out), u"전체" if allrows else u"spec"), u"",
         u"판정 분포: " + u" · ".join(u"%s %d" % kv for kv in C.most_common()) + u"  ‖  " + u" · ".join(u"%s %d" % kv for kv in C2.most_common()), u"",
         u"판정 규칙: ✅동일 = 정렬된 전 명령의 피연산자·rip-rel 데이터·Location(파일,열)·콜리 짝이 전부 같음 / ✅오프셋만 = 차이가 즉치·변위의 구조체 오프셋 이동뿐(≥0x100 즉치 · 변위) / ⚠소형 즉치 = |구|<0x100 이고 |Δ|≤8 인 즉치(열거 태그·카운트·노브 후보 — 읽어야 함) / ⚠데이터 = rip-rel 상수(부동소수 등) 바이트 차이 / ❌구조 = 블록 이동으로도 정렬 안 되는 삽입·삭제·니모닉 차이 또는 분기 타깃 불일치. 블록이동 = 컴파일러 레이아웃 변경(의미 無) · TypeId = 16B pcmpeqb 상수(컴파일마다 바뀜 · 의미 無) · 콜리 주의 = 콜리 짝이 변경/미지(이 함수 자체는 같아도 하위가 다름).", u"",
         u"| 구 | 신 | 함수 | mig060 | **확정** | 명령 | 즉치차 | 변위차 | 데이터차 | Loc✓ | 데이터✓ | 콜리(등급) | 주의 콜리 / 근거 |", u"|---|---|---|---|---|---|---|---|---|---|---|---|---|"]
    for x in out:
        ev = u"; ".join(x["struct_diff"][:2] + x["branch_diff"][:2] + x["enum"][:2] + [u"소형 %s:%s→%s" % (a_, o, n) for a_, o, n in x["small"][:4]] + [u"imm %s→%s" % (o, n) for _, o, n in x["imm"][:3]] + [u"[%s+%s→%s]" % (rg, o, n) for _, rg, o, n in x["disp"][:3]] + x["data"][:3] + x["lea"][:2] + x["bad_callee"][:3])
        L.append(u"| `%s` | `%s` | %s | %s | **%s** | %d/%d | %d | %d | %d | %d | %d | %d(%s) | %s |" % (
            x["old"], x["new"], x["name"][:40], x["v0"], x["strict"], x["ninsn"][0], x["ninsn"][1], len(x["imm"]), len(x["disp"]), len(x["data"]), x["loc_ok"], x["data_ok"], x["callees"],
            u"·".join(u"%s%d" % (k[:8], c) for k, c in sorted(x["callee_grades"].items(), key=lambda kv: -kv[1])[:4]), ev[:300].replace("|", "¦")))
    io.open(os.path.join(HERE, "_next", "mig060_same%s.md" % ("_all" if allrows else ("_pair" if "--pair" in sys.argv else ""))), "w", encoding="utf-8").write(u"\n".join(L))
    print(u"\n" + L[2])

if __name__ == "__main__": main()
