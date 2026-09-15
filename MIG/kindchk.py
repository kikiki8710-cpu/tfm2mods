# -*- coding: utf-8 -*-
u"""G15 `consts[].kind` — **상수의 종류** 게이트. 8차 배치B 통합판(후보 B·C·D 흡수).

## 이 축이 왜 「무측정」인가
`mkspec3.py:349` 가 `meaning` 문자열의 낱말로 `kind` 를 유도하고 **잔여를 전부 `임계`로 떨군다**.
그래서 「임계 107행(186 중 58%)」은 분류 결과가 아니라 **나머지**다. 실측:
`kind=임계` 107행 중 **순서비교(`icmp ult/slt/…`)가 관측되는 행은 20행뿐**이고, 나머지 87행은
`select`(28) · 산술(15) · `phi`(8) · `switch` 케이스(8) 등 **임계일 수 없는 자리**에 있다.

## 후보 3벌이 왜 갈렸나 (실행값: B 102 / C 68+보류20 / D 52 = R2 12·R3 40)
| 후보 | 전역 불일치 | 갈린 이유 |
|---|---|---|
| `B_kindcheck` | **102** | 관측 어휘에 `계수`·`인자상수`·`오프셋가감` 이 있는데 **명세 어휘(4종)엔 없다** ⟹ 그 셋으로 관측되면 **구조적으로 영원히 불일치**(14행). 리터럴 미검출(접힘) 10행도 불일치로 센다. 게다가 `icmp eq` 와 `icmp ult` 를 둘 다 `임계` 로 뭉쳐 태그 판정을 못 한다 |
| `C_kindchk` | **68**(+보류 20) | 「필요 문맥이 하나도 없을 때만」 올리는 보수적 판정식이라 B 보다 낮다. 접힘·콜리인자를 **판정보류**로 뺀 것이 옳은 설계. 대신 `임계`에 `ORD` 를 강제해 `icmp eq` 전용 행을 전부 올린다 |
| `D_constkind` | **52** | 규칙 3개만 쓰는 최소판. 유일하게 **피연산자 슬롯**을 정규식으로 강제해(`icmp op iN %x, VAL`) 잡음이 가장 적다. 대신 `태그`·`센티널`·`인덱스` 오분류는 아예 안 본다(`R1` 0건) |

⟹ **셋이 각각 다른 것을 재고 있었다**: B=어휘 폭 / C=판정식 보수성 / D=피연산자 정밀도.
통합판은 **D 의 피연산자 정밀도 + C 의 보수적 판정식 + B 의 어휘 확장**을 합친다.

## ★★판정식을 「분류」가 아니라 **「반증」**으로 뒤집었다 (이 게이트의 핵심)
초판(분류식: 관측→kind 환산 후 대조)은 강한 적발 21건을 냈는데 **IR 원문 대조에서 7건이 오탐**이었다.
전부 같은 원인이다 — **`getelementptr` 인덱스는 배열 첨자가 아니라 바이트 오프셋**이다.
불투명 포인터 시대의 LLVM 은 구조체 필드 접근을 전부 `getelementptr i8, ptr %p, i64 <바이트>` 로
정규화하므로, `+0x10` 을 읽는 줄은 **값 16 을 쓰는 모든 상수와 우연히 겹친다**
(`02 c4` SubPlan::AttackNexus 태그 16 ↔ `m12.ll:34916` 의 `i64 16` = 구조체 +0x10).

★그리고 그것이 `SPEC_RUNBOOK §S5-b` 가 경고한 바로 그 함정이다 —
분류식은 **「후보가 있는데 주장이 없으면 불일치」**라서 **관측을 늘리는 것이 곧 오탐을 늘린다.**
반증식은 반대다: **「주장을 지지하는 관측이 하나도 없을 때만 불일치」** ⟹ 관측이 늘면 오탐이 **준다.**
귀속·추론은 구제에만 쓰라는 규칙이 판정식 수준에서 강제된다.

### 반증 규칙 4개 (전부 「지지 관측 0」일 때만 발화)
| 규칙 | kind | 지지 관측 | 근거 |
|---|---|---|---|
| `R임계` | 임계 | `CMP_ORD`(`icmp/fcmp ult·ule·ugt·uge·slt·sle·sgt·sge·o*`) | 순서비교가 한 번도 없으면 문턱일 수 없다 |
| `R태그` | 태그 | `SWCASE`·`CMP_EQ`·`STORE`·`PHI`·`SELECT`·`RET` | 판별자는 **비교되거나 기록**된다 |
| `R센티널` | 센티널 | 값 자체가 센티널 리터럴 + 위 관측 | `-1`·`255`·`i64::MAX`·니치 기준값 |
| `R인덱스` | 인덱스 | `GEP`·`MINMAX`(clamp)·`ARITH`(`1-team` 류 첨자 산출) | |

### 어휘 결손 (오류가 아니라 **어휘의 부재**)
`kind=임계` 인데 지지 관측이 없고 **관측이 `ARITH` 뿐 → `계수`**, **`PHI/SELECT/STORE/RET` 뿐 → `산출값`**
이면 그건 명세가 틀린 게 아니라 **현행 4종 어휘에 담을 칸이 없어 잔여 버킷으로 떨어진 것**이다.
따로 센다(`어휘결손`). 이 축을 닫으려면 `mkspec3` 의 파생 규칙을 고쳐야 한다 — `REPORT.md §5`.

### 억제 (오탐 방지)
- **접힘 억제**: 리터럴이 IR 범위에 한 번도 없으면 `보류`(재료 부재이지 오류가 아니다).
- **약한 관측만 있으면 보류**: `CALLARG`(콜리 안에서 쓰인다) · `DBGSTR`(`#dbg_value` 의 `&str` 길이 조각).
  ★`DBGSTR` 은 잡음이 아니다 — `05 c11~c13`("PassiveLine" 11자 등)은 IR **명령**엔 흔적이 없고
  `#dbg_value(i64 11, …, fragment 64,64)` 에만 남아 있다. 그래서 구제에는 쓰고 기각에는 안 쓴다.
- ★**`GEP` 는 기각 근거로 쓰지 않는다** — 위에 적은 바이트-오프셋 겹침 때문. 구제에만 쓴다.

### ★부정문 검사 (NEG) — IR 없이 판정되는 자기모순
`meaning` 이 「… **X 가 아니다**」라고 적었는데 `kind`==X 인 행. 낱말 매칭이 부정문을 못 보는
데서 생긴다. 실측 4건 — `06 c0`(「인덱스가 아니다」→kind=인덱스) · `07 c1`(「태그가 아니다」→kind=태그) ·
`08 c3`·`14 c5`(「임계가 아니다」→kind=임계, 잔여 버킷).

사용:
    python -X utf8 gate.py              # 전량
    python -X utf8 gate.py 7 13         # 지정 명세만
    python -X utf8 gate.py --hard       # 확정 오류(강·NEG)만
`specgate.py` 연동 진입점 = `check_spec(sp)` → `[(j, 등급, kind, 관측, 사유)]`
등급 = `강`(반증됨=오류) / `NEG`(자기모순=오류) / `어휘결손` / `보류`
"""
import io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
IRDIR = r"C:\tfm2mods\_gaibc"
_SRC = {}


def _load(f):
    if f not in _SRC:
        _SRC[f] = io.open(os.path.join(IRDIR, f), encoding="utf-8",
                          errors="replace").read().split("\n")
    return _SRC[f]


TY = r"(?:i\d+|half|float|double|fp128)"
ORD_ = ("ult", "ule", "ugt", "uge", "slt", "sle", "sgt", "sge",
        "olt", "ole", "ogt", "oge")
ARI_ = ("add", "sub", "mul", "udiv", "sdiv", "urem", "srem", "shl", "lshr",
        "ashr", "and", "or", "xor", "fadd", "fsub", "fmul", "fdiv")

# ★값 자체가 센티널인 리터럴. `-1` 은 i8/i32/i64 어느 폭에서도 니치 None 으로 쓰인다.
SENTVAL = {"-1", "255", "9223372036854775807", "-9223372036854775808",
           "18446744073709551615", "4294967295", "65535", "-1.0"}

PRODUCE = ("STORE", "PHI", "SELECT", "RET")          # 값을 **내보내는** 자리
COMPARE_EQ = ("SWCASE", "CMP_EQ")                    # 이산 코드로 **소비**하는 자리
WEAK = ("CALLARG", "DBGSTR", "GEP")                  # ★구제에만 쓴다(기각 금지)

# kind → 그 주장을 **지지**하는 관측 클래스
SUPPORT = {
    # ★09-13(15차 배치B 적발): `llvm.umin/umax` 클램프의 상한·하한도 임계다 — 낱말이 `상한` 인데 `MINMAX` 가
    #   지지 목록에 없어 규칙③(KIND_PRI) 로 `인덱스` 가 됐다(#26 consts[8]). MINMAX 를 구제 관측에 넣는다.
    u"임계":   ("CMP_ORD", "MINMAX"),
    u"태그":   COMPARE_EQ + PRODUCE,
    u"센티널": COMPARE_EQ + PRODUCE,
    u"인덱스": ("GEP", "MINMAX", "ARITH"),
    # 확장 어휘(제안) — 지금 정본엔 없지만 붙이면 바로 검사된다
    u"계수":   ("ARITH", "MINMAX"),
    u"산출값": PRODUCE,
    # ★11차 후속 신설 — `미상` 잔여 버킷(7행)을 분해해서 나온 두 칸.
    u"오프셋가감": ("ARITH", "MINMAX"),
    # ⚠`길이` 의 지지 관측은 **전부 WEAK** 다. 그래서 이 kind 는 **구제만 되고 기각은 안 된다** —
    #   `_kind` 규칙③에서 `CMP_ORD` 가 관측되면 낱말이 지고 `임계` 로 되돌아간다(배열 길이 6행).
    u"길이":   ("DBGSTR", "CALLARG", "CMP_EQ"),
}


def _ops(val):
    v = re.escape(str(val))
    nb = r"(?![\w.])"
    # `[^,]+,\s*V` = 둘째 피연산자 · `V\s*,` = 첫째 피연산자. 둘 다 **타입 접두 뒤**여야 한다.
    # (`align 8`·`!dbg !56`·`%5`·`i8` 은 이 형태가 아니라 자동으로 걸러진다)
    both = r"(?:[^,]+,\s*" + v + nb + r"|" + v + nb + r"\s*,)"
    return (
        ("SWCASE", re.compile(r"^\s*" + TY + r" " + v + r", label ")),
        ("CMP_ORD", re.compile(r"=\s*[fi]cmp\s+(?:samesign\s+)?(?:n\w+\s+)?(?:" +
                               "|".join(ORD_) + r")\s+" + TY + r"\s+" + both)),
        ("CMP_EQ", re.compile(r"=\s*[fi]cmp\s+(?:samesign\s+)?(?:n\w+\s+)?"
                              r"(?:eq|ne|oeq|one|ueq|une)\s+" + TY + r"\s+" + both)),
        ("MINMAX", re.compile(r"llvm\.[us]?(?:min|max)\w*\..*?" + TY + r"\s+" + v + nb)),
        # ★산술 **intrinsic** 도 `ARITH` 다. (11차 후속 — 측정해서 1행)
        #   `llvm.usub.sat.i64(%x, 150000)` 은 LLVM 에선 `call` 이라 `CALLARG`(약)로만 잡혀,
        #   **기각에도 구제에도 못 쓰이는 상태**로 `00 c9`(사거리 여유분)가 `kind=미상` 에 고여 있었다.
        #   포화·오버플로 검사 산술은 소스의 `saturating_sub`/`checked_add` 가 그대로 내려온 것이라
        #   `sub`/`add` 명령과 **의미가 같다**. 관측 클래스도 같아야 한다.
        #   ⚠전수 측정: CALLARG 관측이 붙은 행은 20함수 통틀어 5행이고 그중 이 형태는 1행이다.
        #     (나머지 4행은 `llvm.umin` 이라 이미 MINMAX(강)가 함께 잡혀 판정이 안 바뀐다.)
        ("ARITH", re.compile(r"llvm\.(?:[us](?:add|sub|mul|div)\.(?:sat|with\.overflow)|"
                             r"abs|fsh[lr]|fmuladd)\.[\w.]*\(.*?" + TY + r"\s+" + v + nb)),
        ("ARITH", re.compile(r"=\s*(?:(?:nsw|nuw|exact|fast|reassoc|nnan|ninf|nsz|arcp|"
                             r"contract|afn|disjoint|samesign)\s+)*(?:" + "|".join(ARI_) +
                             r")\s+" + TY + r"\s+" + both)),
        ("GEP", re.compile(r"getelementptr\b.*?,\s*" + TY + r"\s+" + v + nb)),
        ("STORE", re.compile(r"^\s*store\s+(?:volatile\s+)?" + TY + r"\s+" + v + nb + r",")),
        ("SELECT", re.compile(r"=\s*select\b.*?" + TY + r"\s+" + v + nb)),
        ("PHI", re.compile(r"=\s*phi\b.*?\[\s*" + v + nb + r"\s*,")),
        ("RET", re.compile(r"^\s*ret\s+" + TY + r"\s+" + v + nb)),
        # ★09-15(23차 C 적발 · 151/153/154 consts[1] `i64 noundef 2`): 속성이 **타입 뒤**에도 붙는다(`i64 noundef 2`)
        #   → 타입 뒤 `(?:\w+\s+)*` 추가. `sim_kind2.py` 로 129 consts[7]·151/153/154 consts[1] 통과 확인(C 보고).
        ("CALLARG", re.compile(r"(?:tail\s+|musttail\s+)?(?:call|invoke)\b.*?[(,]\s*"
                               r"(?:\w+\s+)*" + TY + r"\s+(?:\w+\s+)*" + v + nb)),
        ("DBGSTR", re.compile(r"#dbg_value\(" + TY + r"\s+" + v + nb +
                              r".*fragment,\s*64,\s*64")),
    )


def observe(f, a, b, val):
    u"""리터럴이 **피연산자 슬롯**에 있을 때만 센다 → {클래스: [줄번호…]}"""
    if not f or not a or not b or val is None:
        return {}
    src = _load(f)
    ops = _ops(val)
    out = {}
    for k in range(a - 1, min(b, len(src))):
        ln = src[k]
        if ln.lstrip().startswith(";"):
            continue
        dbg = "#dbg_" in ln
        body = ln if dbg else ln.split(", !")[0]
        for name, rx in ops:
            if (name == "DBGSTR") != dbg:
                continue
            if rx.search(body):
                out.setdefault(name, []).append(k + 1)
    return out


# ── 부정문 검사 ──────────────────────────────────────────────────────
_KWOF = {u"태그": (u"태그", u"판별자", u"variant"),
         u"센티널": (u"센티널", u"니치"),
         u"인덱스": (u"인덱스",),
         u"임계": (u"임계",),
         u"오프셋가감": (u"여유분", u"여유치", u"바이어스"),
         u"길이": (u"길이",)}
_NEG = re.compile(u"(?:값)?\\s*(?:가|이|은|는|도)?\\s*(?:아니다|아님|아니라|아니고)")


def neg_hit(meaning, kind):
    u"""`meaning` 이 「… <kind> 가 아니다」라고 적었으면 그 대목을 돌려준다."""
    m = meaning or u""
    for kw in _KWOF.get(kind, ()):
        for mt in re.finditer(re.escape(kw), m):
            if _NEG.match(m[mt.end():mt.end() + 16]):
                return m[max(0, mt.start() - 46):mt.end() + 18]
    return None


def _cite(obs, keys):
    return u" ".join(u"%s@%s" % (k, obs[k][0]) for k in keys if k in obs)


# ── 본체 ─────────────────────────────────────────────────────────────
def check_spec(sp):
    u"""반환: [(j, 등급, kind, 관측dict, 사유)]"""
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    out = []
    for j, c in enumerate(sp.get("consts") or []):
        kind = (c.get("kind") or u"").strip()
        val = c.get("value")

        # ① 부정문 자기모순 — IR 없이 판정된다. 가장 강한 신호라 먼저 본다.
        ng = neg_hit(c.get("meaning") or u"", kind)
        if ng:
            out.append((j, u"NEG", kind, {},
                        u"`meaning` 이 「%s」라 적었는데 kind=%s" % (ng.strip(), kind)))
            continue

        sup = SUPPORT.get(kind)
        if sup is None:
            out.append((j, u"보류", kind, {}, u"어휘 밖 kind — 판정 규칙 없음"))
            continue

        obs = observe(f, a, b, val)
        if not obs:
            out.append((j, u"보류", kind, obs,
                        u"리터럴이 IR 범위에 없다(상수 접힘) — 재료 부재"))
            continue

        # ② 지지 관측이 하나라도 있으면 통과. **관측이 늘수록 오탐이 준다.**
        if set(sup) & set(obs):
            continue

        # ③ 센티널 구제 — 값 자체가 센티널이면 산출/비교 없이도 인정
        if kind == u"센티널" and str(val) in SENTVAL:
            continue

        strong = [k for k in obs if k not in WEAK]
        if not strong:
            out.append((j, u"보류", kind, obs,
                        u"약한 관측뿐(%s) — 콜리 안·디버그정보·바이트오프셋" % u",".join(sorted(obs))))
            continue

        # ④ 어휘 결손 — `임계` 잔여 버킷이 삼킨 두 종류
        if kind == u"임계":
            if set(strong) <= {"ARITH", "MINMAX"}:
                out.append((j, u"어휘결손", kind, obs,
                            u"산술 소비뿐(%s) ⟹ **계수** — 현행 4종에 이 칸이 없다"
                            % _cite(obs, strong)))
                continue
            if set(strong) <= set(PRODUCE):
                out.append((j, u"어휘결손", kind, obs,
                            u"산출 전용(%s) ⟹ **산출값**(코드·ID·점수) — 현행 4종에 이 칸이 없다"
                            % _cite(obs, strong)))
                continue

        # ⑤ 등급 — ★적발을 **내가 직접 반증한 결과**를 규칙으로 남긴다(도시에 §1-3)
        #   `SWCASE`(switch 케이스 라벨)는 이산 코드의 결정적 증거라 무조건 `강`.
        #   그게 없으면 두 가지를 의심한다:
        #     ⓐ 값이 `0` — LLVM 은 부호없는 `x > 0` 을 **`icmp ne x, 0` 으로 정규화**한다.
        #       ⟹ 0 에 대한 동등비교는 `임계` 주장을 **반증하지 못한다**.
        #     ⓑ `ARITH` 관측 — 문턱이 `shl`/`mul` 로 접히면 리터럴은 **계수 쪽에만** 남는다
        #       (`03 c5` tps*2 의 `shl …, 1` · `09 c0`·`09 c5` 가 명세 자신이 적어 둔 실례).
        #   둘 중 하나면 `약` 으로 내린다 — **약은 errors[] 로 내지 않는다.**
        lvl = u"강"
        if "SWCASE" not in obs:
            if str(val) == "0":
                lvl = u"약"
            elif "ARITH" in obs and kind == u"임계":
                lvl = u"약"
        note = u"" if lvl == u"강" else (
            u" ⚠값 0 의 동등비교는 `x >u 0` 의 정규화형일 수 있다" if str(val) == "0"
            else u" ⚠산술 관측 — 문턱이 접혀 리터럴이 계수 쪽에만 남았을 수 있다")
        out.append((j, lvl, kind, obs,
                    u"kind=%s 를 지지하는 관측이 **0건**. 실관측 = %s%s"
                    % (kind, _cite(obs, strong), note)))
    return out


def main():
    args = sys.argv[1:]
    hard = "--hard" in args
    idxs = [int(x) for x in args if x.isdigit()] or list(range(20))
    D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
    tally, n = {}, 0
    for i in idxs:
        sp = D["specs"][i]
        rows = check_spec(sp)
        n += len(sp.get("consts") or [])
        for r in rows:
            tally[r[1]] = tally.get(r[1], 0) + 1
        show = [r for r in rows if (not hard or r[1] in (u"강", u"NEG"))]
        if show:
            print(u"\n===== specs[%d] %s =====" % (i, sp["name"]))
            for (j, lvl, kind, obs, why) in show:
                print(u"  consts[%-2d] value=%-22s kind=%-5s [%s] %s"
                      % (j, str(sp["consts"][j].get("value"))[:22], kind, lvl, why))
    print(u"\n" + u"=" * 96)
    print(u"검사 %d행 · " % n + u" · ".join(u"%s %d" % (k, v) for k, v in
                                          sorted(tally.items(), key=lambda x: -x[1])))
    print(u"=" * 96)


if __name__ == "__main__":
    main()
