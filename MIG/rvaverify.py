#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""rvaverify.py — **명세 20함수의 `exe.addr` 이 정말 그 함수인가.** (2026-09-12 신설)

## 왜 만드나 — 실물로 두 건이 틀렸다
`#04`·`#16` 의 `exe.addr` 이 **다른 함수**를 가리키고 있었다(2026-09-12 ghidra-re 확정):
  · `0xd3e4b0` = `has_line_defense_threat`(4-arg) ≠ `handle_line_defense`(6-arg) → 진짜는 **`0xd3cfa0`**
  · `0xc809d0` = `std::thread::LocalKey::with`(TLS 메모) ≠ `max_range_nearly_can_use` → 진짜는 **`0xe0daa0`**
그 주소로 sweep 을 켜자 **게임이 `0xc0000005` 로 죽었다.**

★그리고 이 오류는 **1단계 발화수까지 오염**시켰다 — `#16` 의 「1억 회」는 `LocalKey::with` 의 호출수였다.
  즉 **주소가 틀리면 그 위에 쌓은 모든 측정이 거짓**이 된다. 그래서 이 축을 먼저 세운다.

## 판정 신호 (2026-09-12 재설계 — 회귀 시험으로 검증했다)
**S1 소스 줄 귀속(강한 신호)** — `aimap.json[info][rva].lines`(= exe 함수에 실재하는 패닉 `Location`) 중
   **IR 본문 줄범위 `[선언줄, max DILocation]` 안에 드는 것이 0개**면 불일치.
   ★**반증형**이다 — 「전부 범위 안」을 요구하면 **정답이 걸린다**(인라인된 callee 의 줄이 섞이므로).
**S2 스택 인자(약한 신호)** — IR 인자 **6개 이상**인데 **진입 rsp 기준** 인자영역(`E+0x28` 이상) 읽기가 0개.
   ⚠**단독 기각 근거로 쓰지 않는다** — `#09`(5인자·스택읽기 0·**DIFF=0 으로 증명된 주소**)가 반례다.
**S3 크기** — 참고만(LTO 인라인 때문에 정상적으로도 크게 갈린다. 7차 규칙).

⟹ 판정 = **강한 신호 1개 이상**, 또는 약한 신호 2개 이상. 약한 신호 단독은 `🟡참고`.

## ★★검사기 자신의 성적 (회귀 시험 — `rvaverify.py _spec\specs20.bak-2026-09-12-rva.json`)
| 입력 | 결과 |
|---|---|
| **정정 전 명세**(틀린 주소 6건) | 적발 **[4,10,11,12,15,16] = 6/6** · 오탐 **0** |
| **정정 후 명세**(전부 정답) | 적발 **0** · 오탐 **0** |

★이 성적은 **회귀 시험을 만든 뒤에야** 얻었다. 중간에 「오탐을 줄이는」 수정을 했더니 적발이
  **5건 → 2건으로 떨어졌는데**(진짜 오류 4건을 놓쳤다) 회귀 시험 없이는 **조용히 통과**로 보였다.
  ⟹ ★**느슨해진 검사기는 항상 조용하다.** 검사기를 고칠 때는 **알려진 오류로 반드시 되돌려 돌려라.**

★부수: 초판의 `PROVEN`(DIFF=0 화이트리스트)은 **S2 오탐을 가리고 있었다**.
  **알려진 정답 목록이 있어야 정확해 보이는 검사기는 정확한 게 아니다** — 지금은 화이트리스트를
  꺼도 오탐 0 이다(회귀 모드가 실제로 끄고 돌린다).
"""
import io
import json
import os
import re
import struct
import sys

import capstone

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
EXE = (r"C:\Program Files (x86)\Steam\steamapps\common"
       r"\Teamfight Manager2\TeamfightManager2.exe")
SPEC = r"C:\tfm2mods\MIG\_spec\specs20_v3.json"
AIMAP = r"C:\tfm2mods\MIG\aimap.json"
IRDIR = r"C:\tfm2mods\_gaibc"

# 런타임으로 이미 증명된 것(대조 DIFF=0). 주소가 틀렸다면 갈렸을 표본 수.
PROVEN = {9: 254119, 1: 481514, 8: 761970,
          # r7 잎(09-13): ai_adjust judge 계층 DIFF=0(DONE.md「judge obj_helpers 5종 DIFF=0·NA0(09-10 1판)」·「judge upgrade_item … 3.33e6」)
          #   #24/#29 는 S1 이 「선언줄 이전의 인라인 헬퍼 줄(objective_helpers.rs:12)」만 잡혀 오경보 — 런타임이 주소를 증명한다.
          24: 131631, 29: 263083, 26: 96317, 21: 3330000}   # 표본 = REPORT ai_adjust 03 §「첫 판에 9건 DIFF=0」(09-10) · upgrade_item = judge-layer §26
# ghidra-re 가 2026-09-12 에 확정한 정정값.
FIXED = {4: ("d3cfa0", u"0xd3e4b0 = has_line_defense_threat 였다"),
         16: ("e0daa0", u"0xc809d0 = LocalKey::with(TLS 메모) 였다")}

ARGSPLIT = re.compile(r",(?![^(]*\))")


def sections(d):
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    n = struct.unpack_from("<H", d, pe + 6)[0]
    opt = struct.unpack_from("<H", d, pe + 20)[0]
    # ⚠순서 = (VirtualSize, VirtualAddress, SizeOfRawData, PointerToRawData)
    return [struct.unpack_from("<IIII", d, pe + 24 + opt + i * 40 + 8) for i in range(n)]


def r2o(secs, rva):
    for vsz, va, rsz, ra in secs:
        if va <= rva < va + max(vsz, rsz):
            return ra + (rva - va)
    return None


def ir_argc(sp):
    p = os.path.join(IRDIR, sp["ir"]["file"])
    src = io.open(p, encoding="utf-8", errors="replace").read().split("\n")
    for k in range(sp["ir"]["frm"] - 1, sp["ir"]["frm"] + 8):
        if src[k].lstrip().startswith("define"):
            dl = src[k]
            i = dl.find("(", dl.find("@"))
            depth, j = 0, i
            while j < len(dl):
                if dl[j] == "(":
                    depth += 1
                elif dl[j] == ")":
                    depth -= 1
                    if depth == 0:
                        break
                j += 1
            return len([a for a in ARGSPLIT.split(dl[i + 1:j]) if a.strip()])
    return None


STK = re.compile(r"\[(rbp|rsp)\s*([+\-])\s*(0x[0-9a-f]+)\]")


def stack_arg_reads(ins):
    u"""**진입 rsp 기준**으로 「들어온 스택 인자 영역」 읽기를 센다. 반환 (건수, rbp재베이스량).

    Win64: 진입 시 `[rsp]`=리턴주소, **5번째 인자부터 `[rsp+0x28]`, 0x30, 0x38…**.
    프롤로그가 rsp 를 내리므로 **현재 변위를 그대로 보면 안 된다** — 진입 기준으로 환산해야 한다.
      · `push` ×k        → rsp = E − 8k
      · `sub rsp, N`     → rsp = E − 8k − N
      · `lea rbp,[rsp+M]`→ rbp = E − 8k − N + M
    ⟹ `[rsp+v]` 의 진입변위 = v − 8k − N · `[rbp+v]` 의 진입변위 = v + M − 8k − N.

    ★★왜 이렇게까지 하나 — 초판은 **raw 변위 ≥ 0x28** 을 셌다. 그러면
      ①정답 주소(`lea rbp,[rsp+0x80]` 뒤 `rbp+0x1xx` 로 인자를 읽는 함수)를 **「인자 0」으로 오판**하고,
      ②그걸 고치려 rbp 변위를 그냥 인정하면 **지역변수 접근까지 인자로 세어** 틀린 주소가 통과한다
        (회귀 시험에서 `0xca4a80` 이 「스택인자 11개」로 통과했다).
      ⟹ 두 오류의 방향이 반대라서 **정확히 계산하는 것 말고는 길이 없다.**
    """
    k = n = m = 0
    seen_sub = False
    for x in ins[:14]:
        if x.mnemonic == "push":
            k += 1
        elif x.mnemonic == "sub" and (x.op_str or u"").startswith("rsp, 0x") and not seen_sub:
            n = int(x.op_str.split("0x")[1], 16)
            seen_sub = True
        elif x.mnemonic == "lea":
            mm = re.match(r"rbp, \[rsp \+ (0x[0-9a-f]+)\]", x.op_str or u"")
            if mm:
                m = int(mm.group(1), 16)
    base_shift = 8 * k + n
    cnt = 0
    for x in ins[:60]:
        for breg, sign, hx in STK.findall(x.op_str or u""):
            if sign != "+":
                continue
            v = int(hx, 16)
            eff = v - base_shift + (m if breg == "rbp" else 0)
            # 0x28 이상 = 5번째 인자 이후. 상한을 두어 엉뚱한 큰 변위를 배제한다.
            if 0x28 <= eff <= 0x200:
                cnt += 1
    return cnt, m

# ── S1 정정: 「선언줄」이 아니라 **본문 줄범위** 와 비교한다 ────────────────────
#  ★왜 — `#18 v3_epicops_buff_window` 을 **오경보로 의심 목록에 올렸다**(2026-09-12).
#    패닉 줄 668 은 그 함수의 **본문 범위 [634, 682] 안**인데, 선언줄 634 와의 거리 34 만 보고
#    임계 30 을 넘겼다고 판정했다. 그 오경보 하나 때문에 멀쩡한 주소를 규명 대상에 넣었다.
#  ⟹ 비교 대상을 `[DISubprogram.line, max DILocation.line]` 로 바꾼다. 그러면
#    ①같은 함수 안의 패닉은 **거리 0** 이 되고 ②범위 **밖**이면 오히려 **강한 불일치 신호**가 된다.
#    (즉 이 수정은 오탐만 줄이는 게 아니라 **판별력을 올린다** — 느슨해지는 게 아니다.)
#  ⚠실제 형식엔 `column:` 이 **없다**: `!13699 = !DILocation(line: 825, scope: !13689)`.
#    `column` 을 필수로 둔 정규식이 **전건 미매치**를 냈고, 그 결과 모든 범위가 `None` 이 되어
#    S1 이 조용히 옛 방식으로 후퇴했다 — **「검사가 꺼진 것」이 「통과」로 보이는** 형태다.
#    ⟹ 형식 가정은 **원문 한 줄을 확인한 뒤** 박을 것.
_DILOC = re.compile(r"^!(\d+) = !DILocation\(line: (\d+)(?:, column: \d+)?, scope: !(\d+)(, inlinedAt)?")
_NODE = re.compile(r"^!(\d+) = ")
_FILEREF = re.compile(r"\bfile: !(\d+)")
_SCOPEREF = re.compile(r"\bscope: !(\d+)")
_DBGREF = re.compile(r"!dbg !(\d+)")
_IRCACHE = {}


def _meta(f):
    u"""IR 파일의 메타데이터 표를 한 번만 파싱해 캐시한다."""
    if f not in _IRCACHE:
        src = io.open(os.path.join(IRDIR, f), encoding="utf-8",
                      errors="replace").read().split("\n")
        loc, nfile, nscope = {}, {}, {}
        for ln in src:
            if not ln.startswith("!"):
                continue
            m = _DILOC.match(ln)
            if m:
                loc[m.group(1)] = (int(m.group(2)), m.group(3), bool(m.group(4)))
            m2 = _NODE.match(ln)
            if m2:
                n = m2.group(1)
                mf = _FILEREF.search(ln)
                if mf:
                    nfile[n] = mf.group(1)
                ms = _SCOPEREF.search(ln)
                if ms:
                    nscope[n] = ms.group(1)
        _IRCACHE[f] = (src, loc, nfile, nscope)
    return _IRCACHE[f]


def _fileof(n, nfile, nscope, depth=0):
    u"""메타 노드의 소속 DIFile 을 `scope:` 사슬을 타고 찾는다."""
    while n and depth < 12:
        if n in nfile:
            return nfile[n]
        n = nscope.get(n)
        depth += 1
    return None


def ir_body_range(sp):
    u"""IR `define` 본문이 덮는 **자기 파일의** 소스 줄범위 (lo, hi). 못 구하면 None.

    ★★초판은 본문의 `!dbg` 를 **전부** 모아 범위를 냈다 — 그래서 **인라인된 다른 파일의 줄**과
      `line: 0` 이 섞여 범위가 `(0, 3416)` 처럼 터졌고, **틀린 주소 4건을 통과시켰다**
      (회귀 시험에서 적발: 적발 5건 → 2건으로 **판별력이 깎였다**, 2026-09-12).
      ⟹ 교훈: **오탐을 줄이는 수정은 판별력을 같이 깎을 수 있다.** 반드시 옛 오류로 회귀 시험하라
         (`python rvaverify.py _spec\\specs20.bak-2026-09-12-rva.json`).
    ⟹ 지금은 ①`inlinedAt` 이 붙은 위치 제외 ②`scope` 의 DIFile 이 **함수 자신의 파일과 같은 것만**
      ③`line: 0` 제외. 그래야 범위가 「이 함수의 본문」이 된다.
    """
    src, loc, nfile, nscope = _meta(sp["ir"]["file"])
    # 함수 자신의 DIFile = define 줄의 `!dbg !N`(DISubprogram) 에서 얻는다.
    myfile = None
    for k in range(sp["ir"]["frm"] - 1, min(sp["ir"]["frm"] + 8, len(src))):
        if src[k].lstrip().startswith("define"):
            m = _DBGREF.search(src[k])
            if m:
                myfile = _fileof(m.group(1), nfile, nscope)
            break
    got = []
    for k in range(sp["ir"]["frm"] - 1, min(sp["ir"]["to"], len(src))):
        for n in _DBGREF.findall(src[k]):
            e = loc.get(n)
            if not e:
                continue
            line, scope, inlined = e
            if inlined or line <= 0:
                continue
            if myfile and _fileof(scope, nfile, nscope) != myfile:
                continue
            got.append(line)
    if not got:
        return None
    lo = min(got)
    sl = sp.get("src_line")
    if isinstance(sl, int):
        lo = min(lo, sl)
    return (lo, max(got))


def main():
    # ★검사기를 **검사할** 수 있게 정본 경로를 바꿔 끼운다.
    #   `python rvaverify.py _spec\specs20.bak-2026-09-12-rva.json` = **정정 전(틀린 주소) 명세**로 돌려
    #   「고친 검사기가 옛 오류를 여전히 잡는가」를 회귀 시험한다. 이 구멍이 없으면
    #   오탐을 줄이는 수정이 **판별력까지 깎았는지 알 수 없다**(느슨해진 검사기는 늘 조용하다).
    spec = SPEC
    for a in sys.argv[1:]:
        if a.endswith(".json"):
            spec = a if os.path.isabs(a) else os.path.join(
                os.path.dirname(os.path.abspath(__file__)), a)
    if spec != SPEC:
        print(u"⚠정본 대체: %s  (회귀 시험 모드 — PROVEN/FIXED 표식은 끈다)\n" % spec)
    d = open(EXE, "rb").read()
    secs = sections(d)
    D = json.load(io.open(spec, encoding="utf-8"))["specs"]
    if spec != SPEC:
        PROVEN.clear()
    am = json.load(io.open(AIMAP, encoding="utf-8"))["info"]
    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    md.detail = False

    print(u"#  함수                                 addr       src_line  aimap.lines   IR인자  스택인자  판정")
    print(u"-" * 118)
    sus = []
    for i, sp in enumerate(D):
        ex = sp.get("exe") or {}
        a = (ex.get("addr") or u"").lower()
        if not a:
            print(u"%02d %-34s %-10s  RVA 없음" % (i, sp["name"][:34], u"-"))
            continue
        rva = int(a, 16)
        key = u"0x%x" % rva
        lines = (am.get(key) or {}).get("lines") or []
        sl = sp.get("src_line")
        argc = ir_argc(sp)
        # S2 — 스택 인자 읽기(프롤로그 이후 40명령 안)
        off = r2o(secs, rva)
        ins = list(md.disasm(d[off:off + int(ex.get("bytes") or 400)], 0x140000000 + rva))
        stk, reb = stack_arg_reads(ins)
        # ── 판정: **강한 신호**와 **약한 신호**를 나눈다 ─────────────────────────
        #  ★★약한 신호 단독으로 기각하지 않는다. 근거 = `#09 check_favorable_engage_formation`
        #    (IR 5인자 · 스택 인자 읽기 **0개** · 그러나 **DIFF=0 으로 주소가 증명된** 함수).
        #    즉 「인자가 5개면 스택을 읽어야 한다」는 **틀렸다** — 5번째 인자가 안 쓰이면 안 읽는다.
        #    ⟹ 초판은 이 오탐을 `PROVEN` 화이트리스트로 **가리고 있었다.**
        #       ★알려진 정답 목록이 있어야 정확해 보이는 검사기는 정확한 게 아니다.
        strong, weak = [], []
        rng = ir_body_range(sp)
        if lines and rng:
            lo, hi = rng
            # ★★**반증형 판정식** — 「지지 관측이 0건일 때만 불일치」(프로젝트 공통 규칙).
            #   ⚠「모든 줄이 범위 안이어야 한다」로 쓰면 **정답이 걸린다**: `aimap.lines` 는 exe 함수에
            #     **물리적으로 있는** 패닉 줄이라 **인라인된 callee 의 줄까지 포함**하는데(같은 파일이면
            #     더 흔하다), IR 본문범위는 `inlinedAt` 을 걸러 **자기 줄만** 담는다 ⟹ 두 집합의 성격이 다르다.
            #   실측: `#14 update`(주소 정답 · jac 1.00) 의 줄 = [43,43,44,274,304] / 범위 [39,62].
            #     274·304 는 인라인된 것이고, 43·44 가 **범위 안에 있다** ⟹ 지지 있음 = 통과해야 한다.
            inside = [l for l in lines if lo <= l <= hi]
            # ★09-13(18차 C): 같은 파일의 **인라인 콜리 줄**(예 item_v26_affordable:1692 ← should_recall_to_shop:1664)도 exe 함수 안에 물리적으로 있다.
            #   본문 IR 의 !dbg 사슬(inlinedAt 포함)에 그 줄이 있으면 지지 관측이다.
            if not inside:
                try:
                    _src, _loc, _nf, _ns = _meta(sp["ir"]["file"])
                    _myfile = None
                    for _k in range(sp["ir"]["frm"] - 1, min(sp["ir"]["frm"] + 8, len(_src))):
                        if _src[_k].lstrip().startswith("define"):
                            _m = _DBGREF.search(_src[_k]); _myfile = _fileof(_m.group(1), _nf, _ns) if _m else None; break
                    _all = set()
                    for _k in range(sp["ir"]["frm"] - 1, min(sp["ir"]["to"], len(_src))):
                        for _n in _DBGREF.findall(_src[_k]):
                            _e = _loc.get(_n)
                            if _e and _e[0] > 0 and (_myfile is None or _fileof(_e[1], _nf, _ns) == _myfile):
                                _all.add(_e[0])
                    inside = [l for l in lines if l in _all]
                except Exception:
                    pass
            if not inside:
                dist = min(min(abs(l - lo), abs(l - hi)) for l in lines)
                strong.append(u"S1 줄 %s 중 IR 본문범위 [%d,%d] **안에 드는 것이 0개**(최소거리 %d)"
                              % (lines, lo, hi, dist))
        elif lines and not rng:
            # ★**unknown 을 mismatch 로 바꾸지 않는다**(오늘 세 번째 재발 방지).
            #   옛 방식(선언줄 단독 비교)으로 후퇴하면 `#18` 류 오경보가 되살아난다.
            weak.append(u"S1 판정불가 — IR 본문 줄범위 미산출(기각 근거 아님)")
        if argc and argc >= 6 and stk == 0:
            weak.append(u"S2 IR %d인자인데 진입기준 스택 인자 읽기 0개 (rbp 재베이스 %#x)"
                        % (argc, reb))
        why = list(strong)
        if len(weak) >= 2:
            why += weak                       # 약한 신호 2개 이상이면 함께 의심으로 올린다
        tag = u"✅" if not why else u"⛔의심"
        if not why and weak:
            tag = u"🟡참고(%d)" % len(weak)
        if i in PROVEN:
            tag = u"★런타임 증명(DIFF=0 %s회)" % u"{:,}".format(PROVEN[i])
            why = []
        if why:
            sus.append(i)
        for w in weak:
            if w not in why:
                print(u"      🟡%s" % w)
        print(u"%02d %-34s 0x%-8s %-9s %-13s %-7s %-9s %s"
              % (i, sp["name"][:34], a, sl, lines or u"-", argc, stk, tag))
        for w in why:
            print(u"      ★%s" % w)
    print(u"-" * 118)
    print(u"★의심 **%d개** %s" % (len(sus), sus))
    print(u"\n2026-09-12 ghidra-re 확정 정정:")
    for k, (v, w) in FIXED.items():
        print(u"   #%02d %-34s → **0x%s**   (%s)" % (k, D[k]["name"][:34], v, w))


if __name__ == "__main__":
    main()
