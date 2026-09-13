#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""probe20.py — **20개 AI 판단함수 전용** 카운트 프로브 표 생성기. (2026-09-12 신설)

## `aiprobe.py` 와 무엇이 다른가
`aiprobe.py` 는 `aimap.json` 에서 **명령수 상위 N개**를 골라 표를 만든다(남은 포팅 범위 추정용).
이쪽은 **명세 20함수만** 대상으로 한다 — 목적이 「범위 추정」이 아니라
**`ev1`(런타임 `game==mine` DIFF=0) 측정의 선행조건인 「발화하는가」**를 재는 것이다.
★프로젝트 규율: 「DIFF=0 은 **표본 수와 함께**」(`DONE.md`) · 「**발화 0 = 검증 표본 불성립**」(교훈 17).
   ⟹ 발화수를 모르고 대조를 배선하면 `07` 처럼 「포팅했는데 발화 0 = 사장」을 뒤늦게 안다.

## 왜 프로브가 ABI 를 안 따지나
스텁이 **레지스터·스택·인자를 일절 건드리지 않는다**(`lock inc qword [rip-16]` → 원본 프롤로그 → 원본 복귀).
플래그만 바뀌고 x64 ABI 는 플래그를 호출자 보존 대상으로 안 본다.
⟹ `gensweep.py` 가 막는 **sret·페어반환·5인자+** 도 **프로브는 전부 가능**하다.
   (대조(sweep)는 인자·반환을 알아야 하지만, 발화수는 몰라도 된다.)

## 막는 것은 단 하나 — 프롤로그 모양
진입부 12바이트를 `48 b8 <stub> ff e0` 로 덮으므로, 그 12바이트를 **스텁으로 옮겨 실행**할 수 있어야 한다.
⟹ 그 안에 **분기·call·rip-상대**가 있으면 옮길 수 없다(주소가 어긋난다) ⟹ 프로브 불가.
   `aiprobe.prolog_of` 가 capstone 으로 그걸 판정한다. 이 스크립트는 그 함수를 **재사용**한다(복사 금지).

## ★프로브 종류 2가지 (2026-09-12 확장)
**①진입부 프로브**(`PROBES20`) — 위 방식. 기본.
**②호출부 프로브**(`CALLSITES20`) — 진입부를 못 건드리는 함수용. `call rel32`(5B)의 **rel32 만**
   우리 스텁으로 돌리고 스텁이 `lock inc` 후 원 함수로 절대점프한다.
   ⟹ **원 함수의 명령을 하나도 옮기지 않는다**(진입부 방식보다 오히려 안전. 명령 재배치 0).
   ⚠대신 두 제약: **㉠호출부를 전부 찾아야** 한다(하나라도 놓치면 그 경로가 안 세어진다 ⟹
   이 스크립트가 `.text` 전역 스캔 + 절대주소 리터럴 검사로 **전수성**을 검산한다) ·
   **㉡`E8 rel32` 는 ±2GB** 만 닿는다(⟹ 런타임에서 스텁을 exe 근처에 할당해야 한다.
   근접 검산 실패 시 **설치하지 않는다** — `probe.rs::alloc_near`).
   1호 적용 = **#13 `target_bush_v30`**(진입부 `+9` 의 `je rel32` 가 12B 스틸을 막는다).

사용:
    python -X utf8 MIG\\probe20.py                 # 판정만(생성 안 함)
    python -X utf8 MIG\\probe20.py --out <path>     # 표 생성
"""
import io
import json
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.stdout.reconfigure(encoding="utf-8", errors="replace")

import aiprobe                      # ★prolog_of·sections 재사용(복사하면 규칙이 갈린다)

SPEC = r"C:\tfm2mods\MIG\_spec\specs20_v3.json"

# ★명세 `exe.addr` 가 비어 있는 5건. 그중 둘은 **다른 정본이 이미 확정**해 뒀다.
#   ⚠명세에 미반영이라 여기 적어 두고, 출처를 같이 남긴다(추측이 아니다).
EXTRA = {
    3: ("e01c40", u"judge/gen_fns.rs:49 `DEFC` — 명세 `exe.addr` 미반영"),
    7: ("ccc010", u"DONE.md `hunt_and_battle 13 0xccc010` — 명세 `exe.addr` 미반영"),
    # ★2026-09-12 ghidra-re 확정분(명세 `exe.addr` 미반영 — 다음 라운드에 patch 로 반영할 것)
    5: ("e59190", u"ghidra-re 2026-09-12 확정 — 지문 6종 + 호출자 `update` 2회가 IR 2곳과 일치"),
    13: ("df1c80", u"ghidra-re 2026-09-12 확정 — 패닉 Location 6/6 완전일치 + 오프셋·반환도메인 일치"),
}
# ⛔**#17 `DeathMatchBattle::new` 는 RVA 가 없다 — exe 에 독립 함수가 아니다.**
#   IR 호출부 2곳(`m13.ll:16217`·`16237`)이 둘 다 `LegacyPlanHandler::update`(0xe4c5c0) 안이고
#   LTO 가 **각각 인라인**했다(구간 A `0xe4d901~63` · B `0xe4d964~9f0` · 공통 tail `0xe4da09~dbad`,
#   tail 은 CSE 로 합쳐짐). ⟹ **진입부가 없으므로 카운트 프로브를 붙일 수 없다.**
#   개입하려면 ①`update` 상위 훅에서 BigPlan `+0x5e8` 관측 ②그 3구간 mid-function 핀
#   (`MIG\midpin.py`·`sitepin.py`). 판정 = **재료 부재가 아니라 「대상 부재」**(확정).
# ★★명세 20함수가 **아닌데** 재는 값이 있는 함수. `{idx: (rva, name, module, 사유)}`
#   idx 는 20 이상을 쓴다(명세 인덱스와 겹치지 않게) — 회계에서도 **따로** 센다.
#
#   왜 필요한가 — `#02` 를 MISSING20 으로 내리면 `0xcaf9f0` 의 프로브가 사라진다. 그런데 그 함수는
#   **판당 2,048만 회 도는 플랜 디스패처**로 모든 플랜 선택이 여기를 지난다 ⟹ 계속 재야 한다.
#   ★단 **옛 이름으로 재면 안 된다.** 「#02 attack_nexus/sub_plan 2,048만 회」라는 표기가
#   이 사고의 본체였다 — 같은 숫자를 **진짜 이름으로** 재는 것이 정정이다.
# ★AUX 키는 **명세 idx 와 절대 겹치면 안 된다** — 09-13 r7 편입으로 명세 i=20 이 생기자 AUX[20]=0xcaf9f0 이
#   gensweep20 의 P_RVA[20] 을 덮어 `#20 v27_active_objective_discipline` 의 주소가 BigPlan::sub_plan 으로 찍혔다(생성 단계 적발).
#   ⟹ 명세 밖 보조 항목은 idx 90~ 를 쓴다(명세는 40개 · 서브트리 107 이 다 들어와도 <90).
AUX = {
    90: ("caf9f0", u"BigPlan::sub_plan", u"plan_legacy/types",
         u"★#02 의 호스트. ghidra 확정(점프테이블 16엔트리 ↔ IR switch 16 case · 니치 디코드 일치 · "
         u"16 arm 전수 대응). 1단계 발화 20,484,329 는 **이 함수**의 호출수(= BigPlan 16 variant 합계)이고 "
         u"attack_nexus arm 의 실행수가 아니다. attack_nexus arm = 점프테이블 idx14 = +0x67(midpin 대상)"),
}

NO_ENTRY = {
    2: u"exe 에 독립 함수가 없다 — `BigPlan::sub_plan`(0xcaf9f0) 안 **점프테이블 idx14 arm**"
       u"(`0xcafa57` = +0x67)으로 LTO 인라인(확정: attack_nexus.rs 의 panic::Location static 이 "
       u"이미지 전역에 정확히 2개이고 각 .text 참조가 1개씩, 둘 다 0xcaf9f0 내부 · IR 호출 사이트 1곳 · "
       u".pdata 에 0xcafa57 엔트리 없음). 진입부가 없어 카운트 프로브 불가. "
       u"★호스트는 `AUX[90]`(~~20~~ · 09-13 명세 i=20 과 충돌해 이동) 으로 따로 계측한다 — ~~「#02 = 2,048만 회」~~ 는 **디스패처 호출수**였다. "
       u"개입/실측이 필요하면 = +0x67 에 midpin(첫 명령 7B 라 5B jmp 수용 · rip-상대/분기 없음 · "
       u"진입은 점프테이블 유일 · 직후 cmp 가 flags 재설정 ⟹ r11/r10/rdx/rsi/rbx/rax 보존 필요)",
    17: u"exe 에 독립 함수가 없다 — `update`(0xe4c5c0) 안으로 LTO 인라인(A 0xe4d901 · B 0xe4d964 · "
        u"tail 0xe4da09, CSE 병합). 진입부가 없어 카운트 프로브 불가. ★단 **데스매치 전용**이라 "
        u"MOBA 모드 검증에는 쓰이지 않는다(유저 확인 2026-09-12) ⟹ 20 중 19 로 측정 성립. "
        u"개입이 필요해지면 = update 상위 훅 또는 3구간 mid-function 핀(midpin.py/sitepin.py)",
}

BASE = 0x140000000

# ★★호출부 프로브 대상. `{idx: (target_rva_hex, [call site RVA…], 사유)}`
#   여기 적은 사이트는 **전부 검산된다**(아래 `verify_callsites`) — 베껴 넣은 값은 통과하지 못한다:
#     ㉠사이트 5바이트가 `E8 <rel32>` 인가  ㉡`site+5+rel32 == target` 인가
#     ㉢`.text` 전역 스캔 결과와 **집합이 같은가**(= 호출부 전수. 놓친 사이트 = 안 세는 경로)
#     ㉣절대주소 8B 리터럴 0곳인가(= vtable·함수포인터 경유 간접호출 없음)
#   하나라도 어긋나면 **생성 실패로 멈춘다**(SystemExit). 틀린 표로 게임 `.text` 를 패치하는 것이
#   이 프로젝트에서 가장 비싼 사고다.
CALLSITE = {
    13: ("df1c80", [0xcafc27, 0xdf230e],
         u"진입부 12B 스틸 불가(je@+9 가 잘린다 · 명령 경계 0/4/7/9/15/21 · 21B 안에 분기 둘) — "
         u"호출부 리다이렉트로 전환. 호출부 2곳이 전수(간접호출 0곳 검산)"),
    # r7 잎(09-13): #30 objective_is_damaged(175B) — 진입부 +9 `je 0x140ec8b2b`(cmp dl,5 / je) 가 12B 스틸을 막는다(#13 과 동형).
    #   호출부 = .text 전수 E8 스캔 2곳(0xddbd6d·0xdddf0d · 같은 호출자 함수 안 2회) · 절대주소 8B 리터럴 0곳(disrva/스캔 09-13 실측).
    30: ("ec8af0", [0xddbd6d, 0xdddf0d],
         u"진입부 12B 스틸 불가(je@+9 · cmp dl,5) — 호출부 리다이렉트. 호출부 2곳 전수(간접호출 0곳 검산)"),
    # r8 잎(09-13): #40 v3_assign_anchor(583B) — 진입부 +11 `jae rel32`(cmp r8,2 / jae) 가 12B 스틸을 막는다(#13·#30 과 동형).
    #   호출부 = fnprobe .text 전수 E8 스캔 3곳(0xe4723c·0xe474a2·0xe493c4 · 전부 passive_plan 0xe46bc0 안).
    40: ("e4a780", [0xe4723c, 0xe474a2, 0xe493c4],
         u"진입부 12B 스틸 불가(jae@+11 · cmp r8,2) — 호출부 리다이렉트. 호출부 3곳 전수(passive_plan 안 · 간접호출 0곳 검산)"),
    # r9(09-13 저녁): #58 target_bush_v41(698B) — 진입부 +6 `je rel32`(test cl,cl) 가 12B 스틸을 막는다.
    #   호출부 = fnprobe .text 전수 E8 스캔 2곳(0xcafd90 BigPlan::sub_plan 디스패처 · 0xdba7f7 LineGankerPlan::sub_plan 0xdb9430).
    58: ("db8ba0", [0xcafd90, 0xdba7f7],
         u"진입부 12B 스틸 불가(je@+6 · test cl,cl) — 호출부 리다이렉트. 호출부 2곳 전수(간접호출 0곳 검산)"),
}


# ── PE .pdata(RUNTIME_FUNCTION) — 호출부가 **어느 함수 안**인지 자동 도출 ──
#   목적 = 리포트에 「디스패처 경유 vs next_plan 경유」를 사람 손으로 적지 않고 **실측으로** 싣는 것.
#   레이아웃: DataDirectory[3] = Exception table → 12B 엔트리 (BeginAddress, EndAddress, UnwindInfo) RVA.
def rva2off(secs, rva):
    for va, vsz, ra, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            return ra + (rva - va)
    return None


def pdata(d, secs):
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    magic = struct.unpack_from("<H", d, pe + 24)[0]
    if magic != 0x20b:
        return []                               # PE32+ 아님 — 이 프로젝트 전제 위반
    dd = pe + 24 + 112                           # DataDirectory 시작(PE32+)
    prva, psz = struct.unpack_from("<II", d, dd + 3 * 8)
    off = rva2off(secs, prva)
    if off is None:
        return []
    out = []
    for i in range(psz // 12):
        b, e, _u = struct.unpack_from("<III", d, off + i * 12)
        out.append((b, e))
    out.sort()
    return out


def owner_fn(pd, rva):
    """rva 를 포함하는 함수 (begin, end). 없으면 None."""
    lo, hi = 0, len(pd) - 1
    while lo <= hi:
        m = (lo + hi) // 2
        b, e = pd[m]
        if rva < b:
            hi = m - 1
        elif rva >= e:
            lo = m + 1
        else:
            return (b, e)
    return None


def scan_calls(d, secs, target):
    """`E8 rel32` 로 target 을 부르는 사이트 전수. (`MIG\\_chk13.py` 와 같은 방식)"""
    hits = []
    for va, vsz, ra, rsz in secs:
        n = max(vsz, rsz)
        blob = d[ra:ra + n]
        i = 0
        while True:
            i = blob.find(b"\xe8", i)
            if i < 0 or i + 5 > len(blob):
                break
            rel = struct.unpack_from("<i", blob, i + 1)[0]
            site = va + i
            if site + 5 + rel == target:
                hits.append(site)
            i += 1
    return sorted(set(hits))


def verify_callsites(d, secs, pd, idx, target, sites, known):
    """CALLSITE 선언을 exe 로 검산. 실패 = SystemExit(표를 만들지 않는다).
    반환 = [(site, 원본5B, rel32, 라벨)] · 그리고 전수성/간접호출 메모."""
    rows = []
    for s in sorted(sites):
        off = rva2off(secs, s)
        if off is None:
            raise SystemExit(u"⛔#%d 검산 실패: 사이트 0x%x 가 어느 섹션에도 없다" % (idx, s))
        by = d[off:off + 5]
        if by[0] != 0xE8:
            raise SystemExit(u"⛔#%d 검산 실패: 사이트 0x%x 첫 바이트 %02x ≠ E8 (call rel32 아님)"
                             % (idx, s, by[0]))
        rel = struct.unpack_from("<i", by, 1)[0]
        if s + 5 + rel != target:
            raise SystemExit(u"⛔#%d 검산 실패: 사이트 0x%x 의 call 목표 0x%x ≠ 0x%x"
                             % (idx, s, s + 5 + rel, target))
        ow = owner_fn(pd, s)
        if ow:
            lab = u"fn 0x%x+0x%x" % (ow[0], s - ow[0])
            nm = known.get(ow[0])
            if nm:
                lab += u" (#%d %s/%s)" % nm
        else:
            lab = u"fn ?(.pdata 미등재)"
        rows.append((s, by, rel, lab))

    # ㉢전수성 — 선언 집합 == 스캔 집합
    found = scan_calls(d, secs, target)
    if found != sorted(sites):
        raise SystemExit(u"⛔#%d 검산 실패: 호출부 전수 불일치 — 선언 %s vs 실측 %s"
                         % (idx, [u"0x%x" % x for x in sorted(sites)], [u"0x%x" % x for x in found]))
    # ㉣간접호출 — 절대주소 8B 리터럴
    n_abs = d.count(struct.pack("<Q", BASE + target))
    return rows, n_abs


def load():
    D = json.load(io.open(SPEC, encoding="utf-8"))["specs"]
    out = []
    for i, s in enumerate(D):
        ex = s.get("exe") or {}
        a = (ex.get("addr") or u"").strip().lower()
        src = u"명세 exe.addr (evidence=%s)" % (ex.get("evidence") or u"?")
        if not a and i in EXTRA:
            a, src = EXTRA[i][0], EXTRA[i][1]
        out.append({"i": i, "name": s["name"], "addr": a, "src": src,
                    "ins": ex.get("instrs"), "mod": ex.get("module") or u"?"})
    # ★명세 밖 보조 대상(AUX) — 회계에서 분리하기 위해 `aux` 표식을 단다.
    for i in sorted(AUX):
        a, nm, mod, why = AUX[i]
        out.append({"i": i, "name": nm, "addr": a, "src": why,
                    "ins": None, "mod": mod, "aux": True})
    return out


def main():
    rows = load()
    d = open(aiprobe.EXE, "rb").read()
    secs = aiprobe.sections(d)
    pd = pdata(d, secs)
    ok, bad, nul, cs = [], [], [], []
    print(u"#  함수                                        RVA        프롤로그  판정")
    print(u"-" * 96)
    for r in rows:
        if r["i"] in CALLSITE:
            # ★호출부 프로브 — 진입부는 건드리지 않는다. 검산은 아래(이름 표가 완성된 뒤).
            tgt, sites, why = CALLSITE[r["i"]]
            if r["addr"] and r["addr"] != tgt:
                raise SystemExit(u"⛔#%d CALLSITE target 0x%s ≠ 확정 RVA 0x%s"
                                 % (r["i"], tgt, r["addr"]))
            r["addr"], r["sites"], r["why"] = tgt, sorted(sites), why
            cs.append(r)
            print(u"%02d %-42s 0x%-8s %-9s ★호출부 프로브 %d곳 — 진입부 스틸 불가"
                  % (r["i"], r["name"][:42], tgt, u"(호출부)", len(sites)))
            continue
        if r["i"] in NO_ENTRY:
            r["why"] = NO_ENTRY[r["i"]]
            nul.append(r)
            print(u"%02d %-42s %-10s %-9s ⛔대상 부재 — 인라인돼 진입부가 없다"
                  % (r["i"], r["name"][:42], u"(인라인)", u"-"))
            continue
        if not r["addr"]:
            r["why"] = u"RVA 미확정 — ghidra-re 필요"
            nul.append(r)
            print(u"%02d %-42s %-10s %-9s ★RVA 미확정 — ghidra-re 필요"
                  % (r["i"], r["name"][:42], u"-", u"-"))
            continue
        rva = int(r["addr"], 16)
        p = aiprobe.prolog_of(d, secs, rva)
        if p is None:
            r["why"] = (u"프롤로그 12B 안에 분기/call/rip-상대 — 스텁으로 옮길 수 없다"
                        u"(#13 실측: +9 의 `je rel32` 중간이 잘린다. 명령 경계 0/4/7/9/15 — "
                        u"15B 를 옮기고 rel32 를 재계산하면 가능하나 §3 위험 구역이라 1단계에선 제외)")
            bad.append(r)
            print(u"%02d %-42s 0x%-8s %-9s ⛔프로브 불가 — 프롤로그 12B 안에 분기/call/rip-상대"
                  % (r["i"], r["name"][:42], r["addr"], u"-"))
            continue
        ln, by = p
        r["len"], r["prolog"] = ln, by
        ok.append(r)
        print(u"%02d %-42s 0x%-8s %-9s OK  %s"
              % (r["i"], r["name"][:42], r["addr"], u"%dB" % ln,
                 u" ".join(u"%02x" % b for b in by[:6]) + u" …"))
    print(u"-" * 96)

    # ── 호출부 프로브 검산(⛔틀리면 여기서 멈춘다 — 표를 만들지 않는다) ──
    known = {int(r["addr"], 16): (r["i"], r["mod"], r["name"]) for r in ok}
    if cs:
        print(u"\n=== 호출부 프로브 검산 (exe 실측) ===")
    for r in cs:
        rows_, n_abs = verify_callsites(d, secs, pd, r["i"], int(r["addr"], 16), r["sites"], known)
        r["chk"] = rows_
        r["n_abs"] = n_abs
        print(u"#%02d %s → 0x%s  (호출부 %d곳 · 전수성 OK · 절대주소 8B 리터럴 %d곳%s)"
              % (r["i"], r["name"], r["addr"], len(rows_), n_abs,
                 u"" if n_abs == 0 else u" ⚠간접호출 가능 — 이 경로는 안 세어진다"))
        for s, by, rel, lab in rows_:
            print(u"    site 0x%-9x  %s  = call 0x%s  (rel32 %+d = %#010x)  %s"
                  % (s, u" ".join(u"%02x" % b for b in by), r["addr"], rel, rel & 0xFFFFFFFF, lab))
        if n_abs:
            print(u"    ⚠절대주소 리터럴이 있으면 호출부 리다이렉트가 **전 호출을 덮지 못한다**"
                  u" — 카운트는 하한선으로만 읽어라.")

    # ★회계는 **명세 20함수만** 센다. AUX 는 따로 보고한다 — 섞으면 「20/20」이 깨져 보인다.
    aux_ok = [r for r in ok if r.get("aux")]
    spec_ok = [r for r in ok if not r.get("aux")]
    spec_rows = [r for r in rows if not r.get("aux")]
    print(u"\n★진입부 프로브 **%d** + 호출부 프로브 **%d** = 측정 가능 **%d** "
          u"/ 프롤로그 불가 %d / 대상·RVA 부재 %d  (명세 총 %d)"
          % (len(spec_ok), len(cs), len(spec_ok) + len(cs), len(bad), len(nul), len(spec_rows)))
    if aux_ok:
        print(u"  ＋명세 밖 보조(AUX) **%d개** = %s"
              % (len(aux_ok), u", ".join(u"#%d %s @0x%s" % (r["i"], r["name"], r["addr"])
                                        for r in aux_ok)))
        for r in aux_ok:
            print(u"      #%d 사유: %s" % (r["i"], r["src"]))
    if nul:
        print(u"  제외(대상·RVA 부재) = %s"
              % u", ".join(u"#%02d %s" % (r["i"], r["name"]) for r in nul))
    if bad:
        print(u"  프롤로그 불가 = %s" % u", ".join(u"#%02d %s" % (r["i"], r["name"]) for r in bad))
    for r in nul + bad:
        print(u"    #%02d 사유: %s" % (r["i"], r["why"]))

    if "--out" not in sys.argv:
        print(u"\n(판정만 했다. 표를 만들려면 `--out <path>`)")
        return
    out = sys.argv[sys.argv.index("--out") + 1]
    L = [u"//! probe20_tbl.rs — **자동생성**(`MIG\\probe20.py`). 손으로 고치지 말 것.",
         u"//!   명세 20함수(`MIG\\_spec\\specs20_v3.json`)의 **발화수 전용** 프로브 표.",
         u"//!   ★목적 = `ev1`(런타임 DIFF=0) 측정의 **선행조건** 확인 — 「발화 0 = 검증 표본 불성립」.",
         u"//!   스텁은 레지스터·스택·인자를 안 건드리므로 sret·페어반환·5인자+ 도 안전하다.",
         u"#![allow(dead_code)]",
         u"pub struct P20 { pub idx: u8, pub rva: usize, pub len: u8, pub prolog: &'static [u8],",
         u"                 pub name: &'static str, pub module: &'static str, pub ins: u32 }",
         u"pub static PROBES20: &[P20] = &["]
    for r in ok:
        L.append(u"    P20 { idx: %d, rva: 0x%s, len: %d, prolog: &[%s], name: \"%s\", module: \"%s\", ins: %d },"
                 % (r["i"], r["addr"], r["len"],
                    u", ".join(u"0x%02x" % b for b in r["prolog"]),
                    r["name"], r["mod"], r["ins"] or 0))
    L.append(u"];")
    L.append(u"")
    # ── 호출부 프로브 표 ──
    L += [u"/// ★**호출부 프로브** — 진입부를 못 건드리는 함수용(#13). `call rel32`(5B)의 rel32 만",
          u"///   우리 스텁으로 돌린다 ⟹ **원 함수의 명령을 하나도 옮기지 않는다**(진입부 방식보다 안전).",
          u"///   `sites` = 그 함수를 부르는 `E8 rel32` 사이트 **전수**(생성기가 .text 전역 스캔으로 검산.",
          u"///   절대주소 8B 리터럴 0곳 = vtable·함수포인터 경유 간접호출 없음도 같이 검산했다).",
          u"///   ⚠`E8 rel32` 는 ±2GB ⟹ 스텁을 exe 근처에 할당해야 한다(`probe.rs::alloc_near`,",
          u"///   근접 검산 실패 시 **설치하지 않는다**).",
          u"pub struct C20 { pub idx: u8, pub target_rva: usize, pub sites: &'static [usize],",
          u"                 pub labels: &'static [&'static str], pub name: &'static str,",
          u"                 pub module: &'static str, pub ins: u32, pub why: &'static str }",
          u"pub static CALLSITES20: &[C20] = &["]
    for r in cs:
        L.append(u"    C20 { idx: %d, target_rva: 0x%s, sites: &[%s], labels: &[%s], "
                 u"name: \"%s\", module: \"%s\", ins: %d, why: \"%s\" },"
                 % (r["i"], r["addr"],
                    u", ".join(u"0x%08x" % s for s, _b, _r, _l in r["chk"]),
                    u", ".join(u"\"%s\"" % l for _s, _b, _r, l in r["chk"]),
                    r["name"], r["mod"], r["ins"] or 0, r["why"]))
    L.append(u"];")
    L.append(u"")
    L.append(u"/// ★표에 **없는** 함수와 그 이유. 「빠진 것을 모르는 상태」를 만들지 않는다.")
    L.append(u"pub static MISSING20: &[(u8, &str, &str)] = &[")
    for r in nul + bad:
        L.append(u"    (%d, \"%s\", \"%s\")," % (r["i"], r["name"], r.get("why") or u"?"))
    L.append(u"];")
    io.open(out, "w", encoding="utf-8").write(u"\n".join(L) + u"\n")
    # ⚠앞의 회계 줄은 **명세 20함수만** 세고 이 줄은 **표에 실린 전부**를 센다(AUX 포함).
    #   두 숫자가 다른 것이 정상이므로 기준을 문면에 박아 둔다(안 적으면 다음 세션이 불일치로 읽는다).
    print(u"\n-> %s  (표에 실린 진입부 %d = 명세 %d + AUX %d · 호출부 %d · 명세 제외 %d)"
          % (out, len(ok), len(ok) - len(aux_ok), len(aux_ok), len(cs), len(nul) + len(bad)))


if __name__ == "__main__":
    main()
