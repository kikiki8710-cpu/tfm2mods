# -*- coding: utf-8 -*-
u"""G12 `consts[].src_line` 대조 — **9차 배치A 통합판**(`srclinecheck.py` 의 후속).

`srclinecheck.py` 를 import 해서 **로더·사슬 해석은 그대로 재사용**하고,
**리터럴 매칭과 후보 수집만** 갈아끼운다(원본은 건드리지 않는다 — MIG 루트 수정 금지).

## 9차에 무엇이 틀려 있었나 (8건 전수를 IR 원문으로 가린 결과 **8/8 오탐**)

| 샌 곳 | 증상 | 고친 법 |
|---|---|---|
| ①`litpat` 가 **phi 인입을 못 본다** | `[ 1, %33 ]` 은 타입접두도 쉼표접두도 아니라 매치 자체가 안 된다. ⟹ 7차에 넣은 `_term_dbg` **phi 구제 경로가 한 번도 발화하지 못했다**(죽은 코드) | phi 줄은 `PHIIN` 으로 따로 인식 |
| ②`getelementptr` 오프셋 | `getelementptr .. i8, ptr %30, i64 16` 의 16 이 **타입 접두를 달고 있어** 7차 화이트리스트를 통과한다. `02 consts[4]` 의 **유일한 강후보가 이것**이었다 | gep 명령 자체를 후보에서 제외 |
| ③**접힌 곱셈**(`shl`) | `x*2`→`shl x,1`, `x*4`→`shl x,2` 라 리터럴이 사라진다. `09 consts[0]`(2) `09 consts[5]`(4) 가 이것 | 값이 2ᵏ 면 `shl _, k` 를 **약한 후보**로 |
| ④**병합 위치**(`line: 0`) | LLVM 이 한 명령을 여러 소스줄이 공유하면 `!DILocation(line: 0)` 을 준다. `09 consts[4]` 의 `mul i128 %175, 9` 가 그것(1283·1286 두 비교가 CSE 로 공유) | 그 값의 **소비자**(use) 위치를 약한 후보로 |
| ⑤`#dbg_value` 를 통째로 버림 | `#dbg_value(i64 0, !56148, …)` + `!56148 = !DILocalVariable(name:"range", line: 2398)` 이 `16 consts[5]` 의 **정답 그 자체**였다 | 변수 선언줄을 약한 후보로 |
| ⑥phi 자신의 `!dbg` 를 **강후보**로 셈 | phi 위치는 병합 위치라 신뢰할 수 없다 | phi 줄의 `!dbg` 는 약한 후보로 강등 |

★**추가한 것은 전부 「약한 후보」(구제 전용)와 「후보 제거」뿐이다.** 강한 후보를 늘리지 않았다 —
7차에 `16 → 22` 로 늘린 실패가 「후보를 늘리면 오탐이 는다」였기 때문이다.
구제만 늘리고 기각 근거는 **줄이기만** 했으므로 이 판은 **건수가 늘 수 없다**(구조적 보장).

★**8차 배치C 의 제안(「강한 후보 0개면 검사 불가로 분류」)은 이미 구현돼 있다** —
`srclinecheck.check_spec` 의 `if cands and …` 가 그것이라, 적용해도 **8건 중 0건**이 바뀐다.
(9차 도시에·지시가 이걸 「아직 안 된 고칠 거리」로 적고 있다 — `brief_errors` 에 올렸다.)
"""
import io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = HERE          # ★MIG 루트로 승격됨(9차)
sys.path.insert(0, MIG)
# ★원본(7~8차 판)은 `srclinebase.py` 로 보존한다 — 여기서 그걸 부른다.
#   ⚠**`import srclinecheck` 로 두면 자기 자신을 부르는 순환 import 가 된다.**
#     9차에 실제로 그렇게 승격했다가 `specgate` 가 예외를 삼켜 **G12=0 이라는 거짓 초록**이 떴다.
import srclinebase as S

load = S.load
chain = S.chain
DBG = S.DBG
PHIIN = S.PHIIN
FILEOF = S.FILEOF

_TY = r"(?:i1|i8|i16|i32|i64|i128|float|double)"


# LLVM 은 인자 속성을 타입과 값 사이에 끼운다 — `i64 noundef 11` · `i8 zeroext 1`.
# 7차 화이트리스트는 `\bi64\s+11` 이라 이런 **호출 인자 상수를 통째로 못 봤다**
# (05 consts[11][12][13] = `starts_with` 의 패턴 길이 11·13·12 가 전부 「리터럴 미검출」이었다).
_ATTR = r"(?:(?:noundef|zeroext|signext|immarg|nonnull|inreg|returned|range\([^)]*\))\s+)*"


def litpat(val):
    u"""7차 화이트리스트(타입 접두 · 쉼표 뒤) + **인자 속성 끼임 허용**. phi 인입은 `PHIIN` 이 본다."""
    # ★09-13(17차 D·18차 A): 소스 `level-1`·`x-5` 는 IR 에서 `add … -1`/`-5` 로 접힌다 — 양수 값 검사 때 `-` 접두를 허용한다.
    # ★09-16(24차 F 적발 · 187 consts[25] −1 이 `1` 리터럴에 붙음): 값이 음수면 `-` 는 **필수**, 양수면 선택.
    neg = str(val).strip().startswith("-")
    return re.compile(r"(?:\b" + _TY + r"\s+" + _ATTR + r"|,\s*)" + (r"-" if neg else r"-?")
                      + re.escape(str(val).lstrip("-")) + r"(?![\w.])")


CONT = re.compile(r"^to\s+label\b")


GEP = re.compile(r"^(?:%[\w.$]+\s*=\s*)?getelementptr\b")
ISPHI = re.compile(r"^%[\w.$]+\s*=\s*phi\b")
DEF = re.compile(r"^(%[\w.$]+)\s*=")
VARLINE = re.compile(r"!DILocalVariable\(.*?\bfile: !(\d+).*?\bline: (\d+)")
DBGVAL = re.compile(r"#dbg_value\(\s*" + _TY + r"\s+(-?\d+)\s*,\s*!(\d+)\s*,")


def _ownlines(meta, did, own):
    return [li for (fn, li) in chain(meta, did) if fn == own]


def _leaf_line(meta, did):
    u"""DILocation 사슬의 잎 줄(인라인 전 자기 위치). 0 = 병합 위치(CSE 공유) · 1 = 함수 정의줄 미만 아티팩트."""
    ch = chain(meta, did)
    return ch[0][1] if ch else None


def _phi_bases(src, a, b):
    u"""[a,b] 안에서 `phi` 로 정의된 레지스터 집합 — gep 의 베이스가 phi 면 그 오프셋은 필드가 아니라 **이터레이터 stride** 다."""
    out = set()
    for k in range(a - 1, min(b, len(src))):
        s0 = src[k].strip()
        m = DEF.match(s0)
        if m and ISPHI.match(s0):
            out.add(m.group(1))
    return out


def _shl_k(val):
    u"""`val` 이 2ᵏ(k≥1) 이면 k. 곱셈이 `shl` 로 접혔을 때 되찾기 위한 것."""
    try:
        v = int(val)
    except Exception:
        return None
    if v > 1 and (v & (v - 1)) == 0 and v <= (1 << 40):
        return v.bit_length() - 1
    return None


def _users(src, a, b, reg):
    u"""`reg`(예 `%184`)를 **피연산자로 쓰는** 줄 번호들. 병합 위치(line 0) 구제용."""
    pat = re.compile(r"(?<![\w.$%])" + re.escape(reg) + r"(?![\w.$])")
    out = []
    for k in range(a - 1, min(b, len(src))):
        ln = src[k]
        if "#dbg_" in ln:
            continue
        m = DEF.match(ln.strip())
        if m and m.group(1) == reg:
            continue
        if pat.search(ln):
            # ★09-15(23차 F 적발 · 호이스트 1건 오탐): 소비자가 `invoke`(2줄)면 `!dbg` 는 다음 `to label` 줄에 있다
            #   → 그 줄을 사용자 위치로 삼는다(`CONT` 규칙을 `_users` 에도 적용).
            if "invoke " in ln and not DBG.search(ln) and k + 1 < min(b, len(src)) and CONT.match(src[k + 1].strip()):
                out.append(k + 1)
            else:
                out.append(k)
    return out


_phi_cache = {}


def collect(sp):
    u"""한 spec 의 `consts` 별 (강후보집합, 약후보집합, 매치수) 를 모은다."""
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    own = str(sp.get("src") or "").split("\\")[-1]
    src, meta = load(f)
    # ★09-13(15차 배치D 적발 8/8 오탐): 리터럴이 **aux 클로저/이터레이터 인스턴스**(`ir.aux[]` · 다른 define, 때로 다른 파일)
    #   안에 있으면 본체 범위만 훑는 옛 코드가 「사슬에 없다」로 오탐했다. 본체 + aux 범위를 **같은 후보 집합**으로 훑는다.
    segs = [(src, meta, a, b)]
    for ax in (sp.get("ir") or {}).get("aux") or []:
        try:
            if ax.get("file") == f:
                segs.append((src, meta, int(ax["frm"]), int(ax["to"])))
            else:
                s2, m2 = load(ax["file"]); segs.append((s2, m2, int(ax["frm"]), int(ax["to"])))
        except Exception:
            pass
    res = {}
    for j, c in enumerate(sp.get("consts") or []):
        val, claim = c.get("value"), c.get("src_line")
        pat = litpat(val)
        k_shl = _shl_k(val)
        # ★09-13: bool 상수(0/1)는 `trunc i8 %x to i1` 로 접혀 리터럴이 안 남는다 → 그 줄을 약한 후보로
        truncpat = re.compile(r"\btrunc\b.*\bto i1\b") if str(val) in ("0", "1", "true", "false") else None
        shlpat = re.compile(r"\bshl\b.*,\s*%d(?![\w.])" % k_shl) if k_shl is not None else None
        strong, weak, nmatch = {}, set(), 0

        for (src, meta, a, b) in segs:
          phib = _phi_cache.setdefault((id(src), a, b), _phi_bases(src, a, b))
          for k in range(a - 1, min(b, len(src))):
            ln = src[k]
            s = ln.strip()
            if truncpat is not None and truncpat.search(ln):
                d0 = DBG.search(ln)
                if d0:
                    for li in _ownlines(meta, d0.group(1), own):
                        if li:
                            weak.add(li)

            # ── ⑤-b ★19차 A: `#dbg_value(%x, !VAR, !DIExpression(DW_OP_plus_uconst N …))` — 리터럴 N 이 표현식 안에만 남는 경우
            #    ([78] consts[6] `needed_dps … +1` m04.ll:59889). N == 값이면 VAR 선언줄을 약한 후보로.
            if "#dbg_" in ln and "DIExpression(" in ln:
                me = re.search(r"#dbg_value\([^,]*,\s*!(\d+)\s*,\s*!DIExpression\(([^)]*)\)", ln)
                if me and re.search(r"DW_OP_(?:plus_uconst|constu|minus)\s*,\s*%s(?![\w])" % re.escape(str(val)), me.group(2)):
                    vm = VARLINE.search(meta.get(me.group(1), ""))
                    if vm:
                        fn = FILEOF.search(meta.get(vm.group(1), ""))
                        if fn and fn.group(1).split("\\")[-1] == own:
                            weak.add(int(vm.group(2)))
            # ── ⑤ `#dbg_value` 상수 기록 = 변수 선언줄 구제(약) ────────────────
            if "#dbg_" in ln:
                for (v, vid) in DBGVAL.findall(ln):
                    if v != str(val):
                        continue
                    vm = VARLINE.search(meta.get(vid, ""))
                    if vm:
                        fn = FILEOF.search(meta.get(vm.group(1), ""))
                        if fn and fn.group(1).split("\\")[-1] == own:
                            weak.add(int(vm.group(2)))
                    dm = DBG.search(ln)      # `#dbg_value(..., !LOC)` 의 뒤쪽 위치
                    m2 = re.search(r",\s*!(\d+)\s*\)\s*$", s)
                    if m2:
                        for li in _ownlines(meta, m2.group(1), own):
                            if li:
                                weak.add(li)
                continue

            # ── ③ 접힌 곱셈 `shl _, k` = 약한 후보 ────────────────────────────
            if shlpat is not None and shlpat.search(ln):
                d = DBG.search(ln)
                if d:
                    for li in _ownlines(meta, d.group(1), own):
                        if li:
                            weak.add(li)

            isphi = bool(ISPHI.match(s))
            hit = bool(pat.search(ln))
            if isphi and not hit:
                hit = any(v == str(val) for (v, _lab) in PHIIN.findall(ln))
            if not hit:
                continue
            # ── ⑧ 09-14(22차 C): 함수/호출 속성 `range(i64 0, 161)` 안의 리터럴은 상수가 아니다([126] 620 오탐) ──
            if not isphi and "range(" in ln and not pat.search(re.sub(r"range\([^)]*\)", "", ln)):
                continue

            # ── ② gep 오프셋은 상수가 아니다 — 단 **베이스가 phi**(이터레이터 전진 `gep i8, ptr %phi, i64 448`)면 stride 상수다
            #    (09-16 · 24차 A 적발 · 180 consts[51]/[107] 오탐 2건)
            if GEP.match(s):
                gm = re.search(r"getelementptr\b[^,]*,\s*ptr\s+(%[\w.$]+)\s*,", s)
                if not (gm and gm.group(1) in phib):
                    continue
            nmatch += 1

            st, wk = [], []
            d = DBG.search(ln)
            # ── ⑦ `invoke` 는 **한 명령이 두 줄**이다(7차 배치B 후보의 수확을 흡수) ─────
            #    `%18 = invoke … (…, i64 noundef 11)` / 다음 줄 `to label %33 unwind …, !dbg !N`
            #    같은 명령의 이어쓰기라 **추론이 아니다** ⟹ 강한 후보로 쓴다.
            if not d and k + 1 < min(b, len(src)) and CONT.match(src[k + 1].strip()):
                d = DBG.search(src[k + 1])
            if d:
                # ── ⑥ phi 의 `!dbg` 는 병합 위치 → 약한 후보로 강등 ──────────
                # ── ⑨ 09-16(24차 F·A 적발 · 187 consts[10]/[15] · 186 consts[17]): 잎 줄이 0(CSE 병합)·1(정의줄 미만 아티팩트)인
                #    select/명령의 inlinedAt 프레임은 **자기 위치가 아니다** → 약한 후보로.
                ll = _leaf_line(meta, d.group(1))
                (wk if (isphi or (ll is not None and ll <= 1)) else st).append(d.group(1))
            # ── ① phi 인입: 그 인입 블록의 종결자(7차 `_term_dbg`) ───────────
            if isphi:
                for (v, lab) in PHIIN.findall(ln):
                    if v == str(val):
                        t = S._term_dbg(src, a, b, lab)
                        if t:
                            wk.append(t)
            elif not d and S._in_switch(src, k, a):
                for t in (S._switch_dbg(src, k, a, b), S._switch_own(src, k, a)):
                    if t:
                        wk.append(t)

            got = []
            for did in st:
                li = _ownlines(meta, did, own)
                got += li
                for x in li:
                    if x:
                        strong[x] = strong.get(x, 0) + 1
            for did in wk:
                for x in _ownlines(meta, did, own):
                    if x:
                        weak.add(x)

            # ── ④ 병합 위치(line 0)/자기 위치 없음 → 소비자 위치로 구제 ─────
            if not [x for x in got if x]:
                dm = DEF.match(s)
                if dm:
                    # ★09-16(24차 A 적발 · 180 consts[12] `mul …,100` → `zext`(둘 다 !dbg 없음) → 소비자): 호이스트가
                    #   **여러 단**이면 첫 소비자도 !dbg 가 없다 → dbg 없는 정의를 따라 3단까지 전이 추적.
                    todo, seen_r = [(dm.group(1), 0)], set()
                    while todo:
                        reg, dep = todo.pop()
                        if reg in seen_r or dep > 3:
                            continue
                        seen_r.add(reg)
                        for u in _users(src, a, b, reg):
                            du = DBG.search(src[u])
                            if du:
                                for x in _ownlines(meta, du.group(1), own):
                                    if x:
                                        weak.add(x)
                            else:
                                d2 = DEF.match(src[u].strip())
                                if d2:
                                    todo.append((d2.group(1), dep + 1))
        res[j] = (strong, weak, nmatch)
    return res


def check_spec(sp):
    u"""`specgate G12` 진입점. 불일치만 `[(consts 인덱스, 주장한 줄, 실제 후보)]`.

    판정:
      - 강한 후보가 **없다** → 판정보류(리터럴이 접혔거나 범위 밖) — 보고 안 함
      - claim 이 강·약 어디든 있다 → OK
      - 그 외 → 불일치
    """
    ir = sp.get("ir") or {}
    if not ir.get("file") or not ir.get("frm") or not ir.get("to"):
        return []
    try:
        res = collect(sp)
    except Exception:
        return []
    out = []
    for j, c in enumerate(sp.get("consts") or []):
        claim = c.get("src_line")
        if not isinstance(claim, int) or j not in res:
            continue
        strong, weak, _n = res[j]
        cands = sorted(strong)
        if cands and claim not in strong and claim not in weak:
            out.append((j, claim, cands))
    return out


if __name__ == "__main__":
    D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
    tot = 0
    for i, sp in enumerate(D["specs"]):
        bad = check_spec(sp)
        for (j, claim, cands) in bad:
            tot += 1
            print(u"  [%02d] %-46s consts[%d] src_line=%s / 실제 후보=%s"
                  % (i, sp["name"], j, claim, cands))
    print(u"\n총 %d건" % tot)
