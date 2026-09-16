# -*- coding: utf-8 -*-
u"""specgate — 명세의 **완결 조건**을 기계로 검사한다. (2026-09-11 신설)

왜 만드나 — 2026-09-11 반증검증에서 정정 13건이 나왔는데, 그중 **9건은 새 도구가 아니라
"그 시점에 이미 갖고 있던 재료"로 잡혔어야 하는 누락**이었다. 원인은 실력이 아니라 절차다:
분석에 **완결 조건이 없어서** 라운드마다 다른 것을 발견했다.

실제 누락 사례(전부 이 게이트에 걸린다):
  G1 자기모순  : 17 flee_die 가 logic 엔 i64::MAX, note 엔 usize::MAX (같은 파일)
                 13 극성이 resolved 엔 정답, logic 엔 오답
                 18 줄번호가 still_unknown 엔 674/676, constants 엔 682
                 01 「티어 컷」이 같은 파일 resolved 의 "camp_type.__0 = 팀 인덱스" 와 모순
  G2 호출부    : 09 「유일 호출처」 — 실제 8곳. grep 을 안 했을 뿐이다
  G3 형제함수  : 14 가 target_bush_v30 만 보고 sub_plan 이 쓰는 v41 을 통째로 놓쳤다
  G4 술어 sig  : 01 이 `icmp ult x,2` 를 소스 임계로 읽었다. is_jungle(&self, usize) 의
                 시그니처를 봤다면 그게 `x==0||x==1` 의 접힘임을 알았다

사용:
  python -X utf8 specgate.py              전량
  python -X utf8 specgate.py --only 14    한 함수
  python -X utf8 specgate.py --gate G1    한 게이트
"""
import io, json, os, re, sys
from collections import Counter

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
V3 = os.path.join(HERE, "_spec", "specs20_v3.json")
SPEC = V3 if os.path.exists(V3) else os.path.join(HERE, "_spec", "specs20.json")
IS_V3 = SPEC == V3
STRIKE = re.compile(r"~~(?!~).+?~~")

D = json.load(io.open(SPEC, encoding="utf-8"))
S = D["specs"]
CNT = Counter()
ROWS = []


def flag(gate, i, msg, detail=""):
    CNT[gate] += 1
    ROWS.append((gate, i, msg, detail))


NOTES = []
BROKEN = {}


def broken(gate, ex):
    u"""★★**검사기가 깨졌으면 「0」이 아니라 「깨졌다」고 말해야 한다.** (9차 실사고)

    모든 게이트가 `try: import X / except: return` 이라 **검사기가 예외를 내면 조용히 0** 으로 찍혔다.
    9차에 내가 `srclinecheck.py` 를 순환 import 가 되게 덮어썼는데
    `G12 = 0` 이 떠서 **「8건이 전부 해소됐다」로 읽었다.** 실제로는 검사가 **한 번도 안 돌았다.**

    이 프로젝트가 반복해 당한 **「조용한 no-op」** 이고(closelist needle · mem.chk ·
    auditrounds 공허한 초록 · `--gate` 요약줄에 이어 **다섯 번째**), 이번엔 내가 만들었다.
    ⟹ 예외를 삼키되 **그 사실을 결함으로 세고 머리줄에 `!` 로 표시**한다."""
    BROKEN[gate] = u"%s: %s" % (type(ex).__name__, str(ex)[:160])


def note(gate, i, msg, detail=""):
    u"""★**결함이 아니라 「참고」**로 남긴다 — 건수에 넣지 않는다. (8차 신설)

    게이트가 「사람이 한 번 봐 두면 좋은 것」과 「고쳐야 하는 것」을 같은 통에 넣으면
    **0 을 목표로 삼을 수 없다.** 0 이 안 되는 게이트는 곧 무시되고, 무시되면 진짜 결함도 같이 샌다
    (`SPEC_RUNBOOK §S5-b` 의 「게이트가 오탐을 내면 게이트를 고쳐라, 손으로 넘기지 마라」와 같은 이유)."""
    NOTES.append((gate, i, msg, detail))


def txt(sp, keys):
    u"""취소선(이미 정정된 옛 값)은 뺀 본문."""
    return STRIKE.sub(u" ", json.dumps({k: sp.get(k) for k in keys}, ensure_ascii=False))


# ── G1. 자기모순 — 같은 명세 안에서 서로 다른 말을 하는가 ──────────────
# 상수·타입·센티널은 표기가 여러 곳에 중복된다. 중복된 곳이 서로 다르면 그 자체가 결함이다.
PAIRS = [
    (u"센티널 폭", r"(usize::MAX|u64::MAX)", r"(i64::MAX|isize::MAX)"),
    (u"Option 겹수", r"Option<Option<", r"(?<!Option<)Option<[A-Z]\w+>(?!\s*>)"),
]
NUMKEY = re.compile(r"(?:rs|ll):(\d{2,6})")


def gate1(i, sp):
    live = txt(sp, ("logic", "mem", "consts", "knobs", "sig", "one_line") if IS_V3 else
               ("logic", "reads", "writes", "constants", "calls", "knobs", "new_knobs",
                "signature", "one_line"))
    resv = txt(sp, ("history", "open", "closed") if IS_V3 else
               ("resolved", "unknown", "still_unknown"))
    for label, a, b in PAIRS:
        ha, hb = re.search(a, live), re.search(b, live)
        if ha and hb:
            flag("G1", i, u"자기모순(%s): 같은 명세에 '%s' 와 '%s' 가 동시에 있다"
                 % (label, ha.group(0), hb.group(0)))
    # 같은 심볼에 붙은 줄번호가 본 표와 resolved/unknown 에서 다른가
    # 상수 src_line 이 meaning 의 줄번호와 어긋나는지.
    # ⚠오탐 주의: meaning 이 **인라인된 callee** 의 줄(entity.rs:1701 등)을 인용하는 것은 정상이다.
    #   모순은 **이 함수 자신의 소스 파일**에서 다른 줄을 가리킬 때만 성립한다.
    myfile = os.path.basename(str(sp.get("src") or "")).strip()
    if not myfile:
        return
    for j, c in enumerate(sp.get("consts" if IS_V3 else "constants") or []):
        sl = c.get("src_line")
        if not isinstance(sl, int):
            continue
        mean = STRIKE.sub(u" ", str(c.get("meaning", "")))
        # ⚠오탐 제거: 같은 파일의 **다른 용처**를 의도적으로 함께 적는 경우가 있다
        #   (예: "같은 값 0 은 chat.rs:18 의 TraceLevel::Off 비교에도" / "별개로 … 도 2").
        #   그런 표지가 있으면 모순이 아니다.
        if any(k in mean for k in (u"별개로", u"같은 값", u"두 곳", u"에도 ", u"->", u"→")):
            continue
        # ★**주장한 줄이 본문에도 있으면 「의도적 이중 인용」이다.**
        #   예: src_line=674 인데 meaning 이 "is_none 판정 673, Chat 구성 674/676" 라고 적는 경우.
        #   ⚠초판은 `정정` 이라는 단어로 이런 걸 피하고 있었는데, `cleanspec` 이 그 단어를
        #     출처 표기로 보고 지우자 **오탐 2건이 튀어나왔다**. 우연에 기대던 필터였다.
        #     ⟹ 단어가 아니라 **구조**로 판정한다.
        if re.search(re.escape(myfile) + r":?\s*%d\b" % sl, mean) or (u"%d" % sl) in mean:
            continue
        # ★09-13(18차 C): 같은 파일이라도 **이 함수 본문 밖 줄**(인라인된 이웃 함수 · 예 dive_rejoin_cd:973 ← try_engage_dive:112)은 모순이 아니다.
        try:
            import rvaverify as _RV
            _rng = _RV.ir_body_range(sp) if (sp.get("ir") or {}).get("file") else None
        except Exception:
            _rng = None
        for m in re.finditer(re.escape(myfile) + r":(\d{2,6})", mean):
            if int(m.group(1)) != sl:
                if _rng and not (_rng[0] <= int(m.group(1)) <= _rng[1]):
                    continue
                flag("G1", i, u"constants[%d] src_line=%s 인데 meaning 은 같은 파일 %s:%s 를 가리킨다"
                     % (j, sl, myfile, m.group(1)), mean[:130])
                break


# ── G2. 호출부 전수 열거 — "유일 호출처" 류 주장에 근거가 있는가 ────────
SOLE = re.compile(u"(유일 호출처|호출처는 하나|단 한 곳|유일한 호출)")
COUNTED = re.compile(u"호출부\\s*\\d+\\s*곳|호출부 전수|call sites?\\s*[:=]\\s*\\d+")


def gate2(i, sp):
    live = txt(sp, ("logic", "signature", "calls", "knobs", "new_knobs",
                    "unknown", "still_unknown", "exe"))
    if SOLE.search(live) and not COUNTED.search(live):
        flag("G2", i, u"'유일 호출처' 주장이 있는데 **전수 개수**가 없다 — grep 으로 세라",
             SOLE.search(live).group(0))
    if IS_V3:
        cl = sp.get("callers") or {}
        if not cl.get("count"):
            flag("G2", i, u"callers.count == 0 — 자동 열거가 0곳이다. vtable 간접호출이거나 "
                          u"심볼이 어긋난 것이니 손으로 확인해야 한다", str(cl.get("scope")))
        return
    ex = sp.get("exe") or {}
    if isinstance(ex, dict) and ex.get("callers") == []:
        flag("G2", i, u"exe.callers 가 빈 배열이다 — '호출자 없음'인지 '조인 실패'인지 구분 안 됨")


# ── G3. 형제 함수 — 같은 플랜의 다른 진입점을 봤는가 ────────────────────
SIB = ("sub_plan", "next_plan", "is_end", "update", "on_enter", "on_exit")


ARG_PROMOTED = {129}   # 09-14 r13: should_add_self_etc_buff_action `&Effect`(56B) → %3 Arc data ptr + %4 vtable ptr(IR 5 인자 · tcx 4) — G5 개수 대조 면제
FREE_FN = {0, 1, 3, 4, 9, 10, 16, 19} | set(range(21, 33)) | set(range(34, 40)) | {41, 45, 46, 48, 49, 50, 51, 52, 53} | {59, 60, 61, 62, 64, 65, 66, 70, 71, 73} | {78, 79, 81, 82, 84, 85, 87, 90, 92, 93, 94, 95, 96, 98, 99, 101} | {103} | ({112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 126, 127, 129, 130, 131, 132, 133}) | ({134, 135, 136, 137, 142, 147, 148, 149, 150, 152, 156, 158, 159, 160, 161, 162, 163, 164, 165, 166, 169, 170, 171}) | ({180, 181, 183, 185, 186, 187}) | ({201}) | ({204, 207, 209, 211, 215, 217, 221, 222, 224, 226, 227, 232, 233, 236, 238}) | ({249, 251, 252, 254, 255, 257, 259, 260, 261, 265, 267})   # r18 자유 함수 11(09-16 오후 · _RNvNt/_RNvC 심볼 · 나머지 8 은 impl 메서드/클로저 인스턴스) · r17 자유 함수 15(09-16 · _RNvNt… 심볼 실측 · 나머지 30 은 impl 메서드) · r16 abstract_input::attack(09-16) · r15 거대 자유 함수 6(09-15 · 182/184 get_input 는 메서드) · r14 중간 자유 함수 23(09-14) · r13 잎 20(09-14 · 125 lane_minion·128 SmallActionTrace 는 메서드) · r11 103 position_risk(09-14) · r10 16(09-13 밤) · 실측으로 자유 함수(impl 타입 없음)인 것만 · 59~73 = r9(09-13 저녁) · 21~32·34~39 = r7 잎 · 41·45·46·48~53 = r8 잎(심볼 _RNvNt · 09-13)


def gate3(i, sp):
    if IS_V3:
        sb = sp.get("siblings") or {}
        if sb.get("plan") and not sb.get("entries"):
            flag("G3", i, u"siblings.plan=%s 인데 entries 가 비었다 — 자동 열거 실패" % sb.get("plan"))
        # ★`plan == null` 을 무조건 통과시키면 안 된다.
        #   초판이 그렇게 만들어서, 망글링 파싱 버그로 **20개 전부 plan=null** 이 된 것을 못 잡았다
        #   (3차 배치 C 가 13·14 에서 손으로 적발 — G3 가 막으려던 실패와 같은 형태).
        #   자유 함수 화이트리스트에 없는데 null 이면 파싱 실패를 의심해야 한다.
        elif not sb.get("plan") and i not in FREE_FN:
            flag("G3", i, u"siblings.plan == null 인데 자유 함수 화이트리스트에 없다 "
                          u"— 망글링 파싱 실패를 의심하라", (sp.get("sym") or u"")[:90])
        return
    sym = (sp.get("sym") or "") + " " + (sp.get("name") or "")
    cands = [x for x in re.findall(r"\d+([A-Z][A-Za-z0-9]*)", sym) if x.endswith("Plan")]
    if not cands:
        return
    plan = cands[-1]          # 가장 안쪽 타입
    live = txt(sp, ("logic", "calls", "unknown", "still_unknown", "new_knobs", "one_line"))
    seen = [s for s in SIB if s in live]
    mine = (sp.get("name") or "").split("::")[-1]
    if mine in seen:
        seen.remove(mine)
    if not seen:
        flag("G3", i, u"%s 의 형제 진입점(%s)을 하나도 언급하지 않는다 — "
                      u"`grep \"^define.*%s\"` 로 목록부터 뽑아라" % (plan, "/".join(SIB), plan))


# ── G4. 등장 술어의 tcx 시그니처가 해결됐는가 ──────────────────────────
# IR 의 `< N` 상수를 소스 임계로 오독하는 사고(01)가 여기서 걸린다.
# std 제공 술어 — game_ai/game_core 의 판정 술어가 아니라 물을 시그니처가 없다
STD_PRED = set(u"""is_none is_some is_ok is_err is_empty is_null is_nan is_sign_negative
is_char_boundary is_ascii is_alphanumeric can_unwind has_key""".split())
PRED = re.compile(r"\b(is_[a-z0-9_]{2,30}|can_[a-z0-9_]{2,30}|has_[a-z0-9_]{2,30})\s*\(")
SIGD = re.compile(u"(fn\\s*\\(|시그니처|sig\\s*=|tcx sig)")


_IRTXT = {}


def _really_called(sp, names):
    u"""★17차(09-13): 배치 A·B 가 G4 「callees_unmatched ≥8」 6건을 전수 확인했더니 **전부 필드 load**(cache·focused·fountains·
    minion_count…)였다 — `logic` 이 `name(+0x..)` 꼴로 적은 필드를 harvest 가 호출로 긁은 것. 담당 IR 범위(본체+aux)의
    `call`/`invoke` 줄에 그 이름 토큰(v0 망글 `<len><name>`)이 없으면 호출이 아니므로 접는다(범위 = 이 함수 IR 안 · 인라인된 콜리는 못 본다)."""
    ir = sp.get("ir") or {}
    f = ir.get("file")
    if not f:
        return names
    rngs = [(ir.get("frm"), ir.get("to"))] + [(a.get("frm"), a.get("to")) for a in (ir.get("aux") or [])]
    key = (f, tuple(rngs))
    if key not in _IRTXT:
        path = os.path.join(r"C:\tfm2mods\_gaibc", f)
        try:
            src = io.open(path, encoding="utf-8", errors="replace").read().split("\n")
        except Exception:
            return names
        calls = []
        for a0, b0 in rngs:
            if not (isinstance(a0, int) and isinstance(b0, int)):
                continue
            for ln in src[a0 - 1:b0]:
                if ("call " in ln or "invoke " in ln) and "#dbg" not in ln and "llvm." not in ln:
                    calls.append(ln)
        _IRTXT[key] = "\n".join(calls)
    txt = _IRTXT[key]
    keep = []
    for nm in names:
        base = re.split(r"[\s(⟵]", nm.strip())[0]
        if base and re.search(r"\b%d%s\b" % (len(base), re.escape(base)), txt):
            keep.append(nm)
    return keep


def gate4(i, sp):
    if IS_V3:
        nosig = [c.get("name") for c in (sp.get("callees") or []) if not c.get("sig")]
        if nosig:
            flag("G4", i, u"callees 중 sig 가 비어 있는 항목 %d개" % len(nosig), ", ".join(nosig[:8]))
        um = (sp.get("callees_unmatched") or {}).get("names") or []
        um = _really_called(sp, um)
        if len(um) >= 8:
            flag("G4", i, u"callees_unmatched %d개 — 판정에 쓰이는 술어가 섞였는지 손으로 확인" % len(um),
                 ", ".join(um[:10]))
        return
    live = txt(sp, ("logic", "calls", "constants", "knobs", "new_knobs"))
    resv = txt(sp, ("resolved", "unknown", "still_unknown"))
    preds = set(m.group(1) for m in PRED.finditer(live)) - STD_PRED
    if not preds:
        return
    missing = []
    for p in sorted(preds):
        near = re.search(re.escape(p) + r".{0,220}", live + resv, re.S)
        if near and SIGD.search(near.group(0)):
            continue
        # calls[] 에 시그니처가 잡혀 있으면 통과
        if any(p in json.dumps(c, ensure_ascii=False) and SIGD.search(json.dumps(c, ensure_ascii=False))
               for c in (sp.get("calls") or [])):
            continue
        missing.append(p)
    if missing:
        flag("G4", i, u"술어 %d개의 tcx 시그니처가 명세 어디에도 없다 "
                      u"(인자를 받는 술어면 IR 의 임계 상수는 접힘일 수 있다)"
             % len(missing), ", ".join(missing[:8]))


# ── G5. `sig.tcx`(정본) ↔ `sig.params[].type` 의 &/&mut 대조 ────────────
# 3차 배치 B 가 손으로 적발한 유형: `08 params[5]` 이 `&mut TeamPlan` 인데 tcx 는 `&TeamPlan`.
# **같은 스펙 안에 정답이 이미 있었다** — 기계로 잡힌다.
TYNAME = re.compile(r"([A-Za-z_][A-Za-z0-9_]*)")


def split_args(tcx):
    u"""`fn(A, B, C) -> R` 의 최상위 인자만 쪼갠다. 중첩 `<>`·`()` 안의 콤마는 무시한다.

    ⚠문자열 매칭(`"&TeamPlan" in tcx`)으로 하면 안 된다 — tcx 는 완전경로
    `&game_ai::plan_legacy::team_plan::TeamPlan` 을 쓰므로 매치가 실패하고
    **알려진 양성(08 params[6])을 놓친다**(초판이 그래서 0건이었다)."""
    m = re.match(r"\s*fn\s*\((.*)\)\s*(?:->.*)?$", tcx, re.S)
    if not m:
        return []
    body, out, depth, cur = m.group(1), [], 0, []
    for ch in body:
        if ch in "<([":
            depth += 1
        elif ch in ">)]":
            depth -= 1
        if ch == "," and depth == 0:
            out.append("".join(cur).strip())
            cur = []
        else:
            cur.append(ch)
    if "".join(cur).strip():
        out.append("".join(cur).strip())
    return out


def gate5(i, sp):
    if not IS_V3:
        return
    sg = sp.get("sig") or {}
    tcx = sg.get("tcx") or u""
    args = split_args(tcx)
    # ★`(sret)` 슬롯은 **ABI 산물이고 소스 인자가 아니다** — tcx sig 에는 없다.
    #   빼지 않으면 00·07·15 가 전부 "개수 불일치"로 잡히는데 그건 오류가 아니다
    #   (명세가 `(sret)` 라고 정확히 표기해 둔 것이다).
    params = [p for p in (sg.get("params") or [])
              if "sret" not in str(p.get("name") or "").lower()]
    if not args or not params:
        return
    if len(args) != len(params):
        # ★09-13: 팻포인터(&dyn/&[T]/&str) 는 IR 2슬롯 — 명세 params 가 IR 순서를 따르면 tcx 보다 많은 게 정상.
        #   확장 슬롯 수가 맞으면 (data, vtable|len) 로 인자를 복제해 정렬한다(15차 배치A 「G5/G16 오탐」).
        try:
            import paramrole as PR
            exp = []
            for a in args:
                k = PR.ir_slots(a)
                exp.extend([a] if k == 1 else [a, a + u" /*fat ptr 2슬롯*/"])
        except Exception:
            exp = args
        if len(exp) == len(params):
            args = exp
        elif i in ARG_PROMOTED:
            return   # ★09-14 r13: LTO ArgumentPromotion — 소스 `&Effect` 가 IR 에서 (Arc data, vtable) 2 스칼라로 분해(명세 signature 에 기재) · exe 는 argscan 으로 별도 확인
        else:
            flag("G5", i, u"sig.tcx 인자 %d개(팻포인터 확장 %d) vs params %d개 — 개수가 다르다" % (len(args), len(exp), len(params)),
                 u" | ".join(a[:34] for a in args))
            return
    for p, a in zip(params, args):
        t = STRIKE.sub(u" ", str(p.get("type") or u""))
        has_mut = "&mut" in t.replace(" ", "")
        want_mut = a.replace(" ", "").startswith("&mut")
        if has_mut != want_mut:
            flag("G5", i, u"params[%s](%s) 의 가변성이 tcx 와 다르다 — 명세 `%s` / tcx `%s`"
                 % (p.get("i"), p.get("name"), t[:40], a[:60]))


# ── G6. `logic` ↔ 정본(mem/consts/callees) 어긋남 ──────────────────────
# ★3차의 잔존 오염원은 사실상 이것 하나였다 — `logic` 산문이 표의 정정을 못 받는다
# (3차 배치 D 의 실오류 5건 중 3건, 배치 A 의 E1/E2, 배치 C 의 E8).
# 정본에 있는 「신값」이 logic 에 없고 「구값」이 logic 에 살아 있으면 잡는다.
LOGIC_PAIRS = [
    (u"&mut ", u"공유"),          # &mut 표기 ↔ 공유 참조 확정
]


def gate6(i, sp):
    if not IS_V3:
        return
    lg = STRIKE.sub(u" ", sp.get("logic") or u"")
    if not lg:
        return
    # 정본 note/meaning 이 `~~구~~ → 신` 으로 정정한 옛 값이 logic 에 그대로 남아 있는가
    for field in ("mem", "consts", "knobs"):
        for x in sp.get(field) or []:
            blob = json.dumps(x, ensure_ascii=False)
            for m in re.finditer(r"~~([^~]{4,60})~~", blob):
                old = m.group(1).strip("`* ")
                if len(old) < 5:
                    continue
                if old in lg:
                    flag("G6", i, u"%s 가 `~~%s~~` 로 정정했는데 logic 에 그 옛 값이 살아 있다"
                         % (field, old[:50]), x.get("name") or x.get("what") or u"")
                    break


# ══════════════════════════════════════════════════════════════════════
# 4차 반증검증(2026-09-11)이 드러낸 누출 3방향 + 오분류 1 = G7~G10
#
# 4차는 **스펙 내용**은 거의 수렴했는데(오프셋·값 오류 0) 파이프라인이 샜다.
# 샌 곳이 전부 "G6 가 안 보는 방향" 이었다:
#   G7 과열림      : 이미 해소된 항목이 open 에 남아 다음 라운드가 '새 발견'으로 또 집는다
#                    (4차 배치B 8건 · 배치C 프로토타입이 전 20개에서 18건 지목)
#   G8 G6 의 역방향: `logic` 은 고쳐졌는데 **표에 옛 값이 남았다**
#                    (4차 배치D E3 — 3차 P-4 가 `ty.Tower.0` 을 logic 에서만 고쳤다)
#   G9 오염 전파   : `logic` 이 메서드를 필드로 적으면 `harvest_callees` 가 못 잡는다
#                    (4차 배치D E2 — Entity 4메서드 누락. callees 는 logic 의 오류를 상속한다)
#   G10 오분류     : `q` 가 사실 서술("…확정", "…아니다")인데 class 가 `미탐색`
#                    (4차 배치A 제안 — 다음 라운드가 이걸 미탐색으로 읽고 또 판다)
# ══════════════════════════════════════════════════════════════════════

# ── G7. 과열림 — open 항목이 resolved/history 와 같은 얘기인가 ───────────
TOKEN = re.compile(r"[A-Za-z_][A-Za-z0-9_]{4,}|0x[0-9a-f]{2,}|\d{3,}")
SOLVED = re.compile(u"(확정|해소|종결|복원|MATCH|diff\\s*=\\s*0|오라클|전문)")


def _toks(s):
    return set(t.lower() for t in TOKEN.findall(s or u"")) - {"specs", "note", "value"}


def _shared_units():
    u"""`shared` 를 (경로, 본문) 단위로 편다.

    ★초판은 `shared` 를 아예 안 봤다. 그래서 **답이 `shared` 에 있는데 `open` 에 그대로 남은 항목**을
      못 잡았다 — 4차 배치A 가 `03 open[0]`(`shared.is_recent_visible.blackboard_인덱스_의미` 에
      ★확정으로 답이 있다)을 손으로 지목했는데 그게 5차 직전까지 열려 있었다.
      `shared` 는 **여러 함수가 공유하는 사실**을 모으는 자리이므로, 함수별 `history` 만 보면
      「다른 함수가 이미 규명한 것」을 영원히 못 닫는다.
    """
    out = []
    for k, v in (D.get("shared") or {}).items():
        if isinstance(v, dict):
            for k2, v2 in v.items():
                out.append((u"%s.%s" % (k, k2), json.dumps(v2, ensure_ascii=False)))
        else:
            out.append((k, json.dumps(v, ensure_ascii=False)))
    return out


_SHU = [None]


def gate7(i, sp):
    if not IS_V3:
        return
    if _SHU[0] is None:
        _SHU[0] = _shared_units()
    hist = [(u"history/closed", json.dumps(h, ensure_ascii=False))
            for h in (sp.get("history") or []) + (sp.get("closed") or [])]
    # ⚠방법론 버킷(오라클 레시피·함정)은 「어떤 함수의 사실」이 아니라 기법 모음이라
    #   토큰 겹침 규칙에서 빼야 한다 — 안 빼면 `readonly`·`StdRng` 같은 일반어로 오탐이 난다.
    cand = hist + [(u"shared." + k, b) for k, b in _SHU[0] if u"함정" not in k and u"레시피" not in k]
    sh_keys = [k for k in (D.get("shared") or {}) if len(k) >= 8]
    for x in sp.get("open") or []:
        q = x.get("q") or u""
        ql = STRIKE.sub(u" ", q)
        qt = _toks(ql)
        if len(qt) < 4:
            continue
        # ── 규칙 A(고정밀): open 이 `shared` **키 이름 자체**를 부르는데 그 키가 확정을 선언한다 ──
        #    `shared` 는 여러 함수가 공유하는 사실의 자리다. open 이 그 이름을 말하면서
        #    아직 묻고 있다면, 십중팔구 다른 함수가 이미 규명한 것을 모르고 있는 것이다.
        hit = None
        for k in sh_keys:
            if k in ql:
                sub = json.dumps((D["shared"] or {}).get(k), ensure_ascii=False)
                if u"★확정" in sub or u"확정:" in sub or u"확정." in sub:
                    hit = k
                    break
        if hit:
            flag("G7", i, u"★open 이 `shared.%s` 를 부르는데 그 항목은 **확정**이다 — 과열림" % hit,
                 q[:120])
            continue
        # ── 규칙 B(토큰 겹침): 문면이 달라도 같은 얘기인 경우 ──
        for where, hb in cand:
            inter = qt & _toks(hb)
            if len(inter) >= max(6, int(len(qt) * 0.65)) and SOLVED.search(hb):
                flag("G7", i, u"open 항목의 답이 **%s** 에 이미 있다 (%d/%d 토큰 겹침) — 과열림 의심"
                     % (where, len(inter), len(qt)), q[:120])
                break


# ── G8. G6 의 역방향 — logic 은 고쳤는데 표에 옛 값이 남았다 ─────────────
def gate8(i, sp):
    if not IS_V3:
        return
    lg = sp.get("logic") or u""
    if not lg:
        return
    # logic 이 `~~구~~` 로 정정 선언한 옛 값들을 모은다
    olds = []
    for m in re.finditer(r"~~([^~]{4,60})~~", lg):
        o = m.group(1).strip(u"`* ")
        if len(o) >= 5 and not o.startswith(u"//"):
            olds.append(o)
    if not olds:
        return
    for field in ("mem", "consts", "knobs", "sig"):
        rows = sp.get(field) or []
        rows = rows if isinstance(rows, list) else [rows]
        for x in rows:
            blob = json.dumps(x, ensure_ascii=False)
            live = STRIKE.sub(u" ", blob)      # 표에서도 취소선 처리된 건 이미 정정된 것
            for o in olds:
                if o in live:
                    flag("G8", i, u"logic 은 `~~%s~~` 로 정정했는데 %s 에 그 옛 값이 "
                                  u"취소선 없이 남아 있다" % (o[:44], field),
                         (x.get("name") or x.get("what") or u"") if isinstance(x, dict) else u"")
                    break


# ── G9. logic 표기가 callees 자동생성을 오염시키는가 ────────────────────
# `harvest_callees` 는 `name(` 만 함수로 본다. 그래서 `champ.ty.skill2_cooldown` 처럼
# **메서드를 필드로** 적으면 그 함수가 callees 에서 통째로 빠진다(4차 배치D E2, 4개 누락).
#
# ⚠순진하게 "필드 표기인데 동명 함수가 있다" 로 잡으면 **오탐이 지배한다** — Rust 는
#   필드 `line` 과 메서드 `line()` 이 공존하고, `nearest_enemy`·`chats`·`pos` 가 실제로 그렇다
#   (초판이 이 4건을 전부 올렸다). 그래서 두 등급으로 나눈다:
#     ★강함  = `a.b.c` 에서 **중간 b 의 타입이 enum** 인데 c 로 필드 접근을 이어간 경우.
#              variant 없이 enum 을 관통하는 필드 경로는 **존재할 수 없다** ⟹ c 는 메서드다.
#              (D-E2 의 `champ.ty.skill2_cooldown` 이 정확히 이 꼴. `ty` = EntityType)
#     손확인 = 동명 함수가 있으나 동명 필드도 있어 기계로 못 가르는 것. 목록만 낸다.
FIELDPATH = re.compile(r"\.([a-z_][a-z0-9_]{3,45})\b(?!\s*\()")
CHAIN = re.compile(r"\b([a-z_][a-z0-9_]*)\.([a-z_][a-z0-9_]*)\.([a-z_][a-z0-9_]{3,45})\b(?!\s*\()")
_DS = [None]


def _distruct():
    u"""필드명 → 그 필드를 가진 (구조체, 타입) 목록. distruct.json 은 절대 쓰기 금지(읽기만)."""
    if _DS[0] is None:
        idx = {}
        try:
            d = json.load(io.open(os.path.join(HERE, "distruct.json"), encoding="utf-8"))
            for sname, rec in d.items():
                for f in (rec or {}).get("fields") or []:
                    idx.setdefault(f.get("name"), []).append((sname, f.get("type") or u""))
        except Exception:
            pass
        _DS[0] = idx
    return _DS[0]


def _is_enum_field(name):
    u"""그 이름의 필드가 enum 타입인가(Option 니치는 제외 — 그건 관통이 정상)."""
    for _s, t in _distruct().get(name) or []:
        if t.startswith("enum2$<") and "option::Option" not in t:
            return t
    return None


def gate9(i, sp):
    if not IS_V3:
        return
    lg = STRIKE.sub(u" ", sp.get("logic") or u"")
    if not lg:
        return
    try:
        import spec3lib as L
    except Exception:
        return
    have = set((c.get("name") or u"") for c in (sp.get("callees") or []))
    ds = _distruct()

    def isfn(n):
        try:
            return bool(L.fnlookup(n))
        except Exception:
            return False

    # ★강함 — enum 필드를 관통하는 경로
    for a, b, c in set(CHAIN.findall(lg)):
        if c in have:
            continue
        et = _is_enum_field(b)
        if et and isfn(c):
            flag("G9", i, u"★`%s.%s.%s` — 중간 `%s` 는 enum(%s) 이라 필드 경로를 이을 수 없다. "
                          u"`%s` 는 **메서드**이고 callees 자동수집에서 빠진다"
                 % (a, b, c, b, et[7:-1][:40], c), u"logic 표기를 `%s()` 로 고칠 것" % c)

    # 손확인 — 동명 함수가 있으나 동명 필드도 있어 기계로 못 가르는 것
    # ★`mem[].name` 은 `nearest_enemy 태그(...)` 처럼 **주석 꼬리가 붙어 있다.**
    #   점으로만 쪼개면 조각 전체가 한 덩이라 `nearest_enemy` 와 매칭되지 않아
    #   **동명 필드가 있는데도 「함수만 있다」로 흘러갔다**(8차 배치D 적발 — 이것만 고쳐도 4→2).
    memn = set()
    for x in sp.get("mem") or []:
        for part in (x.get("name") or u"").split("."):
            memn.add(part)                              # 원문 조각(기존 동작 보존)
            m = re.match(r"\s*([A-Za-z_][\w]*)", part)  # ★선두 식별자
            if m:
                memn.add(m.group(1))
    amb = []
    for m in sorted(set(FIELDPATH.findall(lg))):
        if m in have or m in memn:
            continue
        if isfn(m):
            amb.append(u"%s%s" % (m, u"(동명 필드 있음)" if m in ds else u" ★함수만"))
    # ★**「손확인」은 결함이 아니라 정보다 — 건수에 넣지 않는다.** (8차)
    #   동명 필드가 실재하면 `logic` 이 필드로 적은 게 맞을 수 있다. 실제로 7차 배치B 가
    #   `07 target_bush` 를 「self 타입 `EpicHuntAndBattlePlan` 에서는 Field」로 확인했다.
    #   ⚠그렇다고 「동명 함수가 다른 Self 타입이면 빼라」를 넣으면 안 된다 — 8차 배치D 가
    #     회귀 시험으로 반증했다: 그러면 **G9 가 원래 잡으라고 만들어진 4차 D-E2**
    #     (`Entity::skill2_cooldown`·`ult_cooldown`)까지 같이 지운다(필드 소유자 ≠ 함수 Self).
    #   ⟹ 규칙을 바꾸지 말고 **등급을 나눠** 센다. `★강함`(enum 관통)만 결함이다.
    if amb:
        note("G9", i, u"손확인 %d개 — logic 이 필드처럼 적었고 tcx 에 동명 함수가 있다. "
                      u"**동명 필드가 실재하면 정상**이다(결함 아님)" % len(amb), ", ".join(amb[:8]))


# ── G10. open 항목의 class 오분류 — 사실 서술을 미탐색으로 뒀는가 ────────
# ★09-13 17차: 배치 A·C 가 「specgate 와 mkspec3 의 FACT_TAIL 이 달라 같은 문장이 한쪽은 사실·한쪽은 미탐색」을 지적 →
#   정규식을 mkspec3 에서 import 한다(단일 출처). 그러면 G10 은 「어미는 사실인데 QUESTION_HEAD(물음 머리) 때문에 미탐색으로 남은 것」만 잡는다.
from mkspec3 import FACT_TAIL  # noqa: E402


def gate10(i, sp):
    if not IS_V3:
        return
    for x in sp.get("open") or []:
        if (x.get("class") or u"") != u"미탐색":
            continue
        q = STRIKE.sub(u" ", x.get("q") or u"").strip()
        if not q:
            continue
        last = [ln.strip() for ln in q.split(u"\n") if ln.strip()][-1]
        if FACT_TAIL.search(last):
            flag("G10", i, u"class=미탐색 인데 문면이 **사실 서술**로 끝난다 — "
                           u"다음 라운드가 이걸 또 판다", last[-110:])


# ── G11. `history` 가 지시한 ev 상향이 표에 반영됐는가 ─────────────────
# ★5차 배치A 제안·실측. `history` 에 「consts[2]·consts[3]·knobs[1] **ev 4→2**」처럼
#   **어느 행을 몇으로 올릴지 명시**해 놓고 표는 그대로 `ev=4` 인 경우가 7행 있었다.
#   왜 새는가 — `ev` 는 **근거 문면에서 파생**되는데(`mkspec3.evtier`), `history` 에 적은
#   지시는 `consts[].meaning`/`knobs[].effect` 본문을 바꾸지 않으므로 **영원히 전달되지 않는다.**
#   G8 은 값만 보고, `audit4` 는 4차분만 본다 ⟹ 이 계열은 어느 게이트에도 안 걸렸다.
#   (같은 형태가 3차 §7 에서도 8건 새어 4차가 손으로 적발했다. 두 번 같은 일이 났으니 게이트로 만든다.)
EVDIR = re.compile(u"ev\\s*([1-5])\\s*(?:→|->)\\s*([1-5])")
EVROW = re.compile(r"(mem|consts|knobs|sig)\s*\[\s*(\d+)\s*\]")


def gate11(i, sp):
    if not IS_V3:
        return
    for hj, h in enumerate(sp.get("history") or []):
        hb = json.dumps(h, ensure_ascii=False)
        for m in EVDIR.finditer(hb):
            want = int(m.group(2))
            # 지시 문장 근처(앞 200자)에서 대상 행을 찾는다
            ctx = hb[max(0, m.start() - 200):m.end()]
            rows = EVROW.findall(ctx)
            if not rows:
                continue
            bad = []
            for field, idx in rows:
                arr = sp.get(field)
                if not isinstance(arr, list):
                    continue
                j = int(idx)
                if j >= len(arr):
                    continue
                got = arr[j].get("ev")
                if got is not None and got > want:
                    bad.append(u"%s[%d] ev=%s" % (field, j, got))
            if bad:
                flag("G11", i,
                     u"history[%d] 가 **ev %s→%s** 를 지시했는데 표가 안 따라갔다 (%d행)"
                     % (hj, m.group(1), want, len(bad)),
                     u", ".join(bad) + u"  ⟹ 해당 행의 근거 문면에 실행 근거를 적어야 ev 가 내려온다")


# ── G12. `consts[].src_line` 이 IR 이 말하는 줄과 맞는가 ────────────────
# ★★6차 배치A 가 찾아낸 **가장 중요한 구멍**이고, 도구도 그 배치가 만들었다(`srclinecheck.py`).
#
#   왜 중요한가 — 유저 질문 「조건이 같아도 오류가 계속 나오는 이유」의 답이 여기 있다.
#   배치 A 는 이번에 **새 계측기를 하나도 안 만들고**(`found_by:new`=0) 실오류 8건을 찾았다.
#   원인은 명세 불안정도, 새 계측기도 아니었다:
#      **`consts[].src_line` 은 5라운드 동안 어떤 게이트도 검사하지 않는 축이었다.**
#      `qcspec` 은 `value` 만 보고, `specgate G1` 은 `src_line` 을 *같은 문장 안의 줄번호*와만
#      대조했다(자기일관성). IR 과 대조하는 검사는 **없었다.**
#   ⟹ **오류는 검사받지 않는 축에 고인다. 라운드를 돌려도 그 축은 안 줄어든다.**
#      실제로 그 4건은 1차부터 그대로였고, 검사를 붙이자 **전 스펙에서 15건**이 즉시 드러났다
#      (배치 A 가 본 4건 + 아무도 본 적 없던 11건).
#
#   판정 방식 = 그 리터럴을 쓰는 명령의 `!dbg` **inlinedAt 사슬 전체**에서 담당 `.rs` 프레임을 모아
#   주장한 줄이 그 안에 있는지 본다. 사슬 전체를 보는 이유 = 인라인된 accessor 는 최내곽이
#   `entity.rs` 라 진짜 소스 줄이 중간 프레임에 있다.
def gate12(i, sp):
    if not IS_V3:
        return
    try:
        import srclinecheck as SLC
    except Exception:
        broken("G12", sys.exc_info()[1])
        return
    try:
        rows = SLC.check_spec(sp) if hasattr(SLC, "check_spec") else None
    except Exception:
        rows = None
    if rows is None:
        return
    for j, claim, cands in rows:
        flag("G12", i, u"consts[%d] src_line=%s 인데 IR 사슬에 그 줄이 없다" % (j, claim),
             u"실제 후보 = %s" % (cands[:8],))


# ── G13. `knobs[].where` 의 IR 줄번호·인용 명령 대조 ────────────────────
# ★「검사받지 않는 축」의 두 번째. 6차 배치A 의 규칙(**오류는 검사받지 않는 축에 고인다**)을
#   `consts.src_line`(G12) 다음으로 큰 축에 적용한 것이다.
#   축별 커버리지 실측: `mem.dir` 451 · **`knobs.where` 221** · `consts.kind` 186 · `sig.params.role` 124
#   — 982행이 5라운드 동안 아무 게이트도 안 봤다.
#   `knobs.where` 는 그중 **가장 강하게 검사된다**: 줄번호 존재 + **백틱 인용이 진짜 그 줄에 있는지**.
def gate13(i, sp):
    if not IS_V3:
        return
    try:
        import whereline as WL
        rows = WL.check_spec(sp)
    except Exception:
        broken("G13", sys.exc_info()[1])
        return
    for j, why, det in rows:
        flag("G13", i, u"knobs[%d] %s" % (j, why), det)


def gate14(i, sp):
    u"""`mem[].dir` 방향 대조. 8차 배치A 신설.

    ★**「부재」는 결함이 아니다** — 콜리 안 접근·레지스터 승격·상수 접힘이면 그 오프셋이 IR 에 안 보인다.
      후보 4벌 중 46건을 낸 판이 바로 부재를 결함으로 세고 있었다(전수 오탐).
    ★검출력은 **변이 시험**으로 쟀다: 명세를 일부러 뒤집어 넣으면 **238/448(53.1%)** 를 잡는다.
      ⟹ 적발 0 은 게이트가 무능해서가 아니라 **그 축에 진짜 오류가 없어서**다."""
    if not IS_V3:
        return
    try:
        import memdir as MD
        rows = MD.check_spec(sp)
    except Exception:
        broken("G14", sys.exc_info()[1])
        return
    for j, why, det in rows or []:
        flag("G14", i, u"mem[%d] %s" % (j, why), det)


def gate17(i, sp):
    u"""`history` 결론이 표에 **전파**됐나. 8차 배치D 신설 — `G8`(logic↔표)의 짝.

    ★느슨한 앵커는 **폐기 대상의 인접 항목**을 집는다(오탐률 90% 실측).
      ⟹ 「폐기된 수치가 그 행의 `value` 칸에 **실제로 있어야** 한다」를 조건으로 넣어 10→1."""
    if not IS_V3:
        return
    try:
        import histprop as HP
        rows = HP.check_spec(sp)
    except Exception:
        broken("G17", sys.exc_info()[1])
        return
    for slot, why, det in rows or []:
        flag("G17", i, u"%s %s" % (slot, why), det)


def gate15(i, sp):
    u"""`consts[].kind` ↔ IR 소비 오프코드 대조. 8차 배치B 신설.

    ★**판정식이 「분류」가 아니라 「반증」이다** — 이 라운드 최대 수확.
      분류식은 「후보가 있는데 주장이 없으면 불일치」라 **관측이 늘면 오탐이 늘지만**,
      반증식은 「주장을 지지하는 관측이 0건일 때만 불일치」라 **관측이 늘면 오탐이 준다.**
      ⟹ 「귀속은 구제에만」이 규칙이 아니라 **판정식의 성질**이 된다.
    ★`보류`(상수 접힘·약한 관측뿐)는 결함이 아니다 — **`강`·`NEG` 만** 센다."""
    if not IS_V3:
        return
    try:
        import kindchk as KK
        rows = KK.check_spec(sp)
    except Exception:
        broken("G15", sys.exc_info()[1])
        return
    for j, grade, kind, obs, why in rows or []:
        if grade not in (u"강", u"NEG"):
            continue
        flag("G15", i, u"consts[%d] kind=%s %s" % (j, kind, grade), why)


def gate16(i, sp):
    u"""`sig.params[].role` ↔ IR `define` 헤더 대조. 9차 배치B 통합판(8차 배치C 판의 후속).

    ★8차 판의 적발 20건(`P4` 6·`P6` 14)이 **전수 오탐**이었고, 원인이 8차가 정리한
      「오프셋 귀속 4형태」와 **전혀 달랐다**:
        ⑤**인용문 안의 낱말을 주장으로 읽기** — `role` 이 백틱으로 인용한 `define` 헤더 속
          `writeonly`·`noalias` 를 그 인자의 주장으로 셌다
        ⑥**인용 경계 붕괴** — 길이 상한이 정규식 안에 있어 184자 인용에서 백틱 짝이 밀렸고,
          **한국어 산문이 「IR 인용」으로 날조**됐다(게이트가 없는 인용을 만들어 내고 그걸 반증했다)
    ★검출력 = 변이 **599/599 (100%)**. 그중 **음성 대조 129건**(무해한 변경은 잡으면 안 된다)이
      포함돼 있어 「무조건 적발하는 소음 게이트」가 아님이 같이 증명됐다."""
    if not IS_V3:
        return
    try:
        import paramrole as PR
        rows = PR.check_spec(sp)
    except Exception:
        broken("G16", sys.exc_info()[1])
        return
    for j, why, det in rows or []:
        flag("G16", i, u"sig.params[%s] %s" % (j, why), det)


def gate18(i, sp):
    u"""`logic` 이 인용한 필드·오프셋이 `mem`/`consts` 표에 **있는가**. 9차 배치D 신설.

    `G6`(logic 미반영)·`G8`(표에 옛 값)의 빈틈이다 — 그 둘은 **값이 어긋났나**만 보고
    **아예 빠졌나**는 안 본다.

    ⚠★**검출력이 낮다 — 변이 시험 68/308 = 22.1%.** `logic` 이 자유 서술 의사코드라
      기계 대조의 구조적 한계다. **이 게이트를 통과했다고 「logic 과 표가 정합한다」고 읽지 마라.**
      (배치D 가 구제 통로를 늘려 적발 7→1 로 줄였고 포착률도 27.9→22.1% 로 내려갔는데,
       **내려간 5.8%p 는 전부 오탐이 서 있던 자리**였다 — 반증식의 성질 그대로다.)
    ⛔폐기한 탐침: **상수 리터럴 대조**(6/6 전수 오탐 — `logic` 의 숫자는 대부분 줄번호다)."""
    if not IS_V3:
        return
    try:
        import xreflogic as XR
        rows = XR.check_spec(sp)
    except Exception:
        broken("G18", sys.exc_info()[1])
        return
    for slot, why, det in rows or []:
        flag("G18", i, u"%s %s" % (slot, why), det)


def gate19(i, sp):
    u"""`knobs[].value` — 노브의 **값**이 IR 과 맞는가. 9차 배치C 신설.

    ★`G13` 은 `where`(위치)와 인용 문면만 본다 — **값은 아무도 안 봤다.**
      노브는 **바꿔 쓰라고 있는 칸**이라, `mem`/`consts` 가 틀리면 재구현자가 당하지만
      **여기가 틀리면 그 표를 보고 설정을 바꾸는 사용자가 직접 당한다.**

    ⚠★**「`G13` 통과」를 전제로 깔면 안 된다** — 실측으로 깨졌다.
      유일한 실오류(`18 knobs[8]`)가 있는 세 행은 `G13`=0 인데도 `where` 가
      7·37·**117줄** 어긋나 있었다(`G13` 이 `..` 인용을 건너뛰기 때문).

    ⚠검출력 = 변이 **34.3%**(104행 × 3변이). 대조 가능한 행이 **221 중 104(47%)** 뿐이다
      — 값 부재 14 · 산문 56 · 정수 없음 30 · 앵커 미성립 21. 「전량 대조」를 목표로 잡으면
      나머지에 **억지 판정**을 내게 된다."""
    if not IS_V3:
        return
    try:
        import knobval as KV
        rows = KV.check_spec(sp)
    except Exception:
        broken("G19", sys.exc_info()[1])
        return
    for j, why, det in rows or []:
        flag("G19", i, u"knobs[%s] %s" % (j, why), det)


_SHARED = {}


def gate20(i, sp):
    u"""G20 `shared` — **명세 간 공유 사실 대조**. (11차 후속 신설, 배치D 제안)

    ★G1~G19 는 전부 **명세 1개 안**을 본다. 두 명세가 같은 사실을 다르게 적어도
      각자 자기 안에서는 무모순이라 아무도 안 잡는다. 이 게이트가 그 사각지대다.

    ⚠**cross-spec 이라 다른 게이트와 호출 규약이 다르다** — 한 번만 계산하고
      결과를 각 행의 **자기 명세 index** 로 나눠 붙인다. 그래야 `--only N` 이
      N 의 불일치를 그대로 보여 준다(전체를 i==0 에 몰면 `--only` 가 거짓말한다).

    실측 8건(R1 5 · R2 1 · R3 2) · **억제 9건**(enum 페이로드 중첩 4 · 정밀도 2 · 표기 3).
    억제가 적발보다 많다 — 억제 규칙 없이 켜면 17건 중 9건이 오탐이다."""
    if not IS_V3:
        return
    if "rows" not in _SHARED:
        try:
            import sharedchk as SH
            _SHARED["rows"] = SH.check(S)[0]
        except Exception:
            _SHARED["rows"] = []
            broken("G20", sys.exc_info()[1])
            return
    for (rule, key, rows, why) in _SHARED["rows"]:
        # ★★**불일치 1건을 참여 명세 수만큼 세지 마라.** (신설 직후 자기적발)
        #   초판은 참여 명세마다 flag 해서 불일치 8건이 **19건**으로 찍혔다
        #   (`AbstractGameWithCache game` 한 건이 명세 6개에 걸쳐 있다).
        #   「적발 건수는 게이트의 성능이 아니라 가설이다」(7차) — 부풀린 건수는 거짓 가설이다.
        #   ⟹ **가장 낮은 참여 index 에서 한 번만** 세고, 참여자 전원은 상세에 적는다.
        if i != min(r[0] for r in rows):
            continue
        flag("G20", i, u"[%s] %s — %s" % (rule, u" ".join(str(x) for x in key), why),
             u" · ".join(u"[%02d]`%s`" % ab for ab in
                         sorted(set((a, b) for a, b, _ in rows))))


GATES = {"G1": gate1, "G2": gate2, "G3": gate3, "G4": gate4, "G5": gate5, "G6": gate6,
         "G14": gate14, "G15": gate15, "G16": gate16, "G17": gate17, "G18": gate18,
         "G19": gate19, "G20": gate20,
         "G7": gate7, "G8": gate8, "G9": gate9, "G10": gate10, "G11": gate11, "G12": gate12,
         "G13": gate13}
NAMES = {"G1": u"자기모순", "G2": u"호출부 전수", "G3": u"형제 함수", "G4": u"술어 시그니처",
         "G5": u"sig 정본 대조", "G6": u"logic 미반영",
         "G7": u"과열림", "G8": u"표에 옛 값", "G9": u"callees 오염", "G10": u"class 오분류",
         "G11": u"ev 지시 미반영", "G12": u"src_line 대조", "G13": u"knobs.where 대조",
         "G14": u"mem.dir 방향", "G15": u"consts.kind 관측",
         "G16": u"params.role 대조", "G17": u"history 전파",
         "G18": u"logic↔표 상호참조", "G19": u"knobs.value 값",
         "G20": u"명세간 공유사실"}


def main():
    only = None
    if "--only" in sys.argv:
        only = int(sys.argv[sys.argv.index("--only") + 1])
    pick = None
    if "--gate" in sys.argv:
        pick = sys.argv[sys.argv.index("--gate") + 1]
    for i, sp in enumerate(S):
        if only is not None and i != only:
            continue
        for g, fn in GATES.items():
            if pick and g != pick:
                continue
            fn(i, sp)
    # ★★**안 돌린 게이트를 `0` 으로 찍지 마라.** (8차 배치D 적발)
    #   `--gate G9` 를 돌리면 G12 가 `0` 으로 찍혀 「G12 해소됨」으로 오독된다(실제 8건).
    #   이 프로젝트가 반복해 당한 **「조용한 no-op」** 그 자체다
    #   (closelist needle · mem.chk · auditrounds 공허한 초록에 이어 네 번째).
    #   ⟹ 안 돈 것은 **`-`** 로 찍고, 부분 실행이면 머리에 경고를 박는다.
    def _cell(g):
        if pick and g != pick:
            return u"%s %s=-" % (g, NAMES[g])
        if g in BROKEN:
            return u"%s %s=**!**" % (g, NAMES[g])     # ★0 이 아니라 「안 돌았다」
        return u"%s %s=%d" % (g, NAMES[g], CNT[g])

    print(u"\n" + "=" * 100)
    print(u"## 명세 완결 조건 검사 (게임 %s)" % D["meta"].get("game"))
    if pick or only is not None:
        bits = []
        if pick:
            bits.append(u"게이트 **%s 만**(나머지는 `-` = 안 돌렸다는 뜻이지 0 이 아니다)" % pick)
        if only is not None:
            bits.append(u"함수 **%d 만**" % only)
        print(u"   ⚠**부분 실행** — " + u" · ".join(bits))
    print(u"   " + "  ".join(_cell(g) for g in sorted(GATES)))
    if BROKEN:
        print(u"   ★★**검사기가 죽었다 — `!` 는 0 이 아니라 「한 번도 안 돌았다」는 뜻이다.**")
        for g, why in sorted(BROKEN.items()):
            print(u"      %s  %s" % (g, why))
        print(u"   ⟹ 고치기 전까지 그 축의 결과를 믿지 마라.")
    print("=" * 100)
    for g in sorted(GATES):
        rows = [r for r in ROWS if r[0] == g]
        if not rows:
            continue
        print(u"\n### [%s %s] %d건" % (g, NAMES[g], len(rows)))
        for _, i, msg, det in rows:
            print(u"  [%02d] %-46s  %s" % (i, S[i]["name"][:46], msg))
            if det:
                print(u"        %s" % det[:150])
    if NOTES:
        print(u"\n### [참고 — 결함 아님, 건수에 안 들어감] %d건" % len(NOTES))
        for g, i, msg, det in NOTES:
            print(u"  (%s) [%02d] %-42s  %s" % (g, i, S[i]["name"][:42], msg))
            if det:
                print(u"        %s" % det[:150])
    print(u"\n총 %d건" % sum(CNT.values()))


if __name__ == "__main__":
    main()
