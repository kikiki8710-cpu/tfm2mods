# -*- coding: utf-8 -*-
u"""G17 — 축 `history` → 표 **전파** 검사. (8차 배치 D 신설, 2026-09-11)

왜 만드나
────────
`history[]` 는 **닫힌 정정 이력**(`was` → `now` + 근거)이다. 거기 있는 결론이
`mem`/`consts`/`knobs`/`logic` 표에 실제로 내려왔는지는 **어느 게이트도 보지 않았다.**
`G8` 이 그 자리에 있지만 `G8` 은 **`logic` 의 취소선만** 원천으로 삼는다 —
`history` 가 몇 라운드 전에 닫은 값이 표에 안 실려도 `G8` 은 통과시킨다.
⟹ `G17` 은 **`G8` 의 원천을 `logic` → `history` 로 바꾼 짝**이고, 거기에
   `history` 특유의 세 문면(등급 이동 · 폐기 선언 · 확정 선언)을 더한 것이다.

★신호 설계 — **무엇을 근거로 기계 대조하는가** (자유 서술을 통째로 비교하지 않는 이유)
────────────────────────────────────────────────────────────────
`history.now` 는 산문이라 문장 단위 대조가 불가능하다. 그래서 **문면 규약이 이미 만들어 둔
구조적 신호 네 가지만** 쓴다. 넷 다 「표를 읽으면 참/거짓이 갈리는」 것들이다.

  A 취소선 `~~X~~`  = "X 는 폐기된 옛 값"이라는 **명시 선언**. 표에 X 가 취소선 없이
                      살아 있으면 전파 실패다. (`G8` 과 동일 판정, 원천만 다르다)
  B `ev N→M` + 슬롯지목 = `ev` 는 표의 **정수 필드**라 완전 대조가 된다.
                      `consts[2]`·`knobs[1]` 처럼 칸을 지목한 것만 본다(지목 없는 것은
                      대상이 산문에만 있어 기계로 못 고른다 ⟹ 후보로도 올리지 않는다).
                      ⚠`mem` 의 `ev` 는 근거가 무엇이든 **상한 3**(오프셋 주장의 최강 근거가
                      tcx 이기 때문)이라 `M<3` 이고 실제가 3 이면 정상으로 본다.
  C 폐기 선언       = `노브가 아니다` / `존재하지 않는다` / `소비처 0건` 류. 그 결론이
                      대상 행에 **아무 흔적도 없으면** 표가 아직 그것을 노브로 팔고 있다.
                      ★앵커는 **세 조건 동시 충족**을 요구한다 — ①`was` 의 수치 리터럴이
                      그 행의 **`value` 칸**과 같고 ②`was` 의 드문 토큰이 행 본문에 있고
                      ③행에 폐기 흔적이 없다. **실측 근거**: ①을 빼고 ②③만으로 돌리면
                      초판 어휘로 11건(오탐 10) · TRACE 어휘 보강 후 10건이 나오는데
                      손검증 결과 **9건이 오탐(90%)** 이었다 —
                      오탐 전건의 공통점이 「폐기된 값 자체는 그 행에 없다」였다.
                      ①을 넣으면 10 → 1 이 되고 남은 1건이 진짜다(8차 D 실측·전건 손반증).
                      느슨판이 필요하면 `--loose` 로 볼 수 있다(후보 제시형, 오탐률 90% 기지).
  D 확정 ↔ placeholder = 표 행이 아직 `미상`·`불명`·`미확정` 인데, **같은 (base, offset)**
                      을 `history` 가 `확정` 으로 닫았다면 전파 실패다.

★오탐을 늘리지 않기 위해 **일부러 빼는 것**
────────────────────────────────────────────
  ⛔ 오프셋만으로 mem 행을 앵커하지 않는다. 실측 결과 `+0x10`·`+0xc0`·`+0x1e0` 은
     서로 다른 base(`BattlePlanGoal` vs `Effect`, `TeamPlan` vs `AbstractGameWithCache`)에
     동시에 존재해 **base 없이 맞추면 3/3 전건 오탐**이었다(probe6 실측).
     ⟹ D 는 base 문자열 일치를 **필수**로 요구한다.
  ⛔ 「history 의 코드 토큰이 표에 없다」 식의 **부재 판정**은 쓰지 않는다.
     7차에 G12 를 그렇게 고치다 오탐이 16→22 로 늘었다 — 판정식이
     「후보가 있는데 주장이 없으면 불일치」면 **후보를 늘리는 것이 곧 오탐을 늘린다.**
     A·C·D 는 전부 「표에 **무엇이 있다**」를 조건으로 걸어 그 함정을 피한다.
  ⛔ 귀속·추론으로 남의 주장을 **기각**하지 않는다. 추론은 구제(오탐 제거)에만 쓴다.

쓰는 법
───────
  python -X utf8 gate.py            전량
  python -X utf8 gate.py --only 15  한 함수
`specgate.py` 승격 시: `import gate as G17; rows = G17.check_spec(sp)` →
`[(slot, why, detail), ...]` (G13 의 `whereline.check_spec` 과 같은 모양).
"""
import io, json, os, re, sys

STRIKE = re.compile(r"~~(?!~).+?~~")

# ── A: 취소선 ──────────────────────────────────────────────────────────
OLD = re.compile(r"~~([^~]{4,60})~~")

# ── B: 등급 이동 + 슬롯 지목 ────────────────────────────────────────────
# ★부정문 가드 (12차 신설) — 「그 행은 대상이 아니다」를 지시로 읽지 않기 위한 것.
# ⚠★**창(窓) 단독으로는 안 된다 — 넣자마자 진짜 지시 3개를 삼켰다**(슬롯 7개 중 5개 억제).
#   `consts[3]·consts[4]·knobs[1] **ev 4→2**(⚠…밀렸다…)` 처럼 **지시 뒤에 해설이 붙으면**
#   그 해설의 부정어가 지시까지 덮는다. 내가 이 가드를 넣은 라운드에 바로 그 일이 났고,
#   게이트가 **0 을 찍었지만 그건 깨끗해서가 아니라 눈을 가려서**였다(조용한 no-op 재발).
# ⟹ 부정이 **`ev N→M` 마커보다 앞설 때만** 억제한다. 지시는 언제나 슬롯 바로 뒤에 등급을 단다.
NEG_SLOT = re.compile(u"대상이 아니|해당 없|제외한다|아니고|아니다|아니라")


def neg_slot(tail):
    u"""`tail` = 슬롯 지목 바로 뒤 문맥. 부정이 등급 마커보다 **앞**이면 지시가 아니다."""
    ng = NEG_SLOT.search(tail)
    if not ng:
        return False
    ev = EV.search(tail)
    return ev is None or ng.start() < ev.start()
EV = re.compile(u"ev\\s*(\\d)\\s*(?:->|→|=>)\\s*\\*{0,2}(?:ev)?\\s*(\\d)")
SLOT = re.compile(u"(constants|consts|knobs|new_knobs|mem|reads|writes)\\s*\\[\\s*(\\d+)\\s*\\]")
FIELD = {u"constants": "consts", u"consts": "consts", u"knobs": "knobs",
         u"new_knobs": "knobs", u"mem": "mem", u"reads": "mem", u"writes": "mem"}

# ── C: 폐기 선언 ────────────────────────────────────────────────────────
RETRACT = re.compile(u"(노브가 아니다|노브가 아님|존재하지 않는다|소비처가 존재하지 않|"
                     u"소비처 0건|소비처가 없다|항목 삭제|그 노브는 존재)")
#   그 결론이 행에 남긴 흔적(하나라도 있으면 전파된 것으로 본다)
#   ⚠ 어휘가 좁으면 전파된 행을 오탐한다 — `15 mem[5]` 이 「변경하지 않는다」로 적혀 있는데
#      초판 어휘에 그 표현이 없어 오탐이 났다(8차 D 실측). 넓게 잡는 쪽이 안전하다.
TRACE = re.compile(u"(노브가 아니|아니다|아님|존재하지 않|0건|history 참조|소비처|삭제|"
                   u"무영향|변경하지 않|영향 없|진단용|계측|죽은|미사용)")
#   폐기된 '값' 자체를 앵커로 쓴다 — 이것이 C 의 오탐률을 90%(9/10) → 0%(0/1) 로 내린 조건이다
NUM = re.compile(r"(?<![\w.])(\d{1,12})(?![\w.])")

# ── D: 확정 ↔ placeholder ──────────────────────────────────────────────
PH = re.compile(u"(미상|불명|미확정|알 수 없|이름 모름|\\?\\?\\?|이름 미|미규명)")
CONFIRM = re.compile(u"(★?확정|규명|복원 완료|전량 확정)")
OFFRE = re.compile(r"\+?(0x[0-9a-fA-F]{1,5})")

# 앵커로 쓸 수 있는 '드문 토큰'(일반어를 앵커로 쓰면 아무 행에나 붙는다)
RARE = re.compile(r"\b([A-Za-z_][A-Za-z0-9_]{6,45})\b")
STOP = set(u"""history resolved unknown constants knobs specs value offset note
tcxdict tcxaudit oracle confirm 오라클 확정 정정 실행""".split())

#   `--loose` = 후보 제시형(C 의 값 앵커를 뺀 판). 8차 D 실측 오탐률 9/10 = 90%.
LOOSE = [False]


def _live(o):
    return STRIKE.sub(u" ", o if isinstance(o, str) else json.dumps(o, ensure_ascii=False))


def _rows(sp, f):
    v = sp.get(f) or []
    return v if isinstance(v, list) else [v]


def _rare(s):
    return set(t for t in RARE.findall(s or u"") if t.lower() not in STOP)


def check_spec(sp):
    u"""[(slot, why, detail), ...] — slot 은 `consts[2]` 같은 문자열 라벨."""
    out = []
    hist = sp.get("history") or []
    if not hist:
        return out
    logic = sp.get("logic") or u""

    for hi, h in enumerate(hist):
        raw = json.dumps(h, ensure_ascii=False)
        live = _live(raw)
        was = h.get("was") or u""
        now_only = json.dumps({k: v for k, v in h.items() if k != "was"}, ensure_ascii=False)

        # ── A. 취소선으로 폐기 선언한 옛 값이 표에 살아 있는가 ──────────
        olds = []
        for m in OLD.finditer(raw):
            o = m.group(1).strip(u"`* ")
            if len(o) >= 5 and not o.startswith(u"//"):
                olds.append(o)
        for o in olds:
            for f in ("mem", "consts", "knobs", "sig"):
                for j, x in enumerate(_rows(sp, f)):
                    if o in _live(x):
                        out.append((u"%s[%d]" % (f, j),
                                    u"[A] history[%d] 이 `~~%s~~` 로 폐기했는데 표에 취소선 없이 남아 있다"
                                    % (hi, o[:44]),
                                    (x.get("name") or x.get("what") or u"")[:60]
                                    if isinstance(x, dict) else u""))
            if o in STRIKE.sub(u" ", logic):
                out.append((u"logic",
                            u"[A] history[%d] 이 `~~%s~~` 로 폐기했는데 logic 에 살아 있다" % (hi, o[:44]),
                            u""))

        # ── B. `ev N→M` + 슬롯 지목 ─────────────────────────────────────
        evs = set(EV.findall(raw))
        if evs:
            tos = sorted(set(int(b) for _a, b in evs))
            if len(tos) == 1:
                want = tos[0]
                # ★★**「그 행은 대상이 아니다」라고 적은 슬롯을 지시로 읽지 마라.** (12차 신설)
                #   `kindchk.neg_hit` 과 같은 부정문 가드다. 실제로 이 가드가 없어서,
                #   12차에 내가 `history[9]` 에 「⚠현재 `consts[2]` 는 이 지시의 대상이
                #   아니고 ev4 가 옳다」는 **주의를 적었더니 그 주의가 곧 적발**이 됐다.
                #   ⟹ 명세가 「아니다」라고 말할 수 있어야 한다. 못 하면 정정자가
                #      **사실을 적는 대신 게이트를 피해 쓰게** 된다.
                for mt in set((m.group(1), m.group(2), m.start()) for m in SLOT.finditer(raw)):
                    fname, idx, at = mt
                    if neg_slot(raw[at:at + 90]):
                        continue
                    f, j = FIELD[fname], int(idx)
                    rows = _rows(sp, f)
                    if j >= len(rows) or not isinstance(rows[j], dict):
                        continue
                    cur = rows[j].get("ev")
                    if cur is None or cur == want:
                        continue
                    # mem 의 ev 는 상한 3 — 더 내려갈 수 없는 것은 정상
                    if f == "mem" and want < 3 and cur == 3:
                        continue
                    out.append((u"%s[%d]" % (f, j),
                                u"[B] history[%d] 이 `ev→%d` 로 선언했는데 표는 ev=%s" % (hi, want, cur),
                                (rows[j].get("name") or rows[j].get("what")
                                 or rows[j].get("meaning") or u"")[:60]))

        # ── C. 폐기 선언이 대상 행에 흔적을 안 남겼는가 ─────────────────
        mret = RETRACT.search(_live(now_only))
        if mret:
            anc = _rare(was)
            nums = set(NUM.findall(was))
            if anc and (nums or LOOSE[0]):
                for f in ("knobs", "consts", "mem"):
                    for j, x in enumerate(_rows(sp, f)):
                        if not isinstance(x, dict):
                            continue
                        b = _live(x)
                        if not (anc & _rare(b)):
                            continue
                        if TRACE.search(b):
                            continue
                        if not LOOSE[0]:
                            # ★엄격 앵커: 폐기된 수치가 이 행의 `value` 칸에 그대로 있어야 한다
                            v = x.get("value")
                            if v is None or not (nums & set(NUM.findall(str(v)))):
                                continue
                        out.append((u"%s[%d]" % (f, j),
                                    u"[C] history[%d] 이 「%s」로 닫았는데 이 행엔 그 흔적이 없다"
                                    % (hi, mret.group(1)),
                                    (x.get("name") or x.get("what") or u"")[:60]))

        # ── D. 확정 선언 ↔ 표의 placeholder (base+offset 동시 일치 필수) ──
        if CONFIRM.search(live):
            hoffs = set(x.lower() for x in OFFRE.findall(live))
            hrare = _rare(live)
            if hoffs:
                for j, x in enumerate(_rows(sp, "mem")):
                    if not isinstance(x, dict):
                        continue
                    b = _live(x)
                    if not PH.search(b):
                        continue
                    off = (x.get("offset") or u"").lower().lstrip("+")
                    base = (x.get("base") or u"")
                    if off not in hoffs:
                        continue
                    # ★base 문자열이 history 본문에도 나와야 한다(오프셋 충돌 방지)
                    if base and base.split("(")[0].strip() not in live:
                        continue
                    if hrare & _rare(b):
                        continue
                    out.append((u"mem[%d]" % j,
                                u"[D] history[%d] 이 +%s 를 확정했는데 이 행은 아직 미확정 문면이다"
                                % (hi, off), (x.get("name") or u"")[:60]))
    # 같은 (slot, why) 중복 제거
    seen, uniq = set(), []
    for r in out:
        k = (r[0], r[1])
        if k in seen:
            continue
        seen.add(k)
        uniq.append(r)
    return uniq


def main():
    here = os.path.dirname(os.path.abspath(__file__))
    mig = os.path.abspath(os.path.join(here, "..", ".."))
    d = json.load(io.open(os.path.join(mig, "_spec", "specs20_v3.json"), encoding="utf-8"))
    only = None
    if "--only" in sys.argv:
        only = int(sys.argv[sys.argv.index("--only") + 1])
    LOOSE[0] = "--loose" in sys.argv
    if LOOSE[0]:
        print(u"⚠ --loose = 후보 제시형. 8차 D 손검증 실측 오탐률 **9/10 = 90%**.\n"
              u"  건수를 성능으로 읽지 마라 — 반드시 행을 직접 열어 반증하라.\n")
    n = 0
    for i, sp in enumerate(d["specs"]):
        if only is not None and i != only:
            continue
        for slot, why, det in check_spec(sp):
            n += 1
            print(u"  [%02d] %-44s %-12s %s" % (i, sp["name"][:44], slot, why))
            if det:
                print(u"        %s" % det[:140])
    print(u"\n[G17 history→표 전파] %d건" % n)


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    main()
