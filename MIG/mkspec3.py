# -*- coding: utf-8 -*-
u"""mkspec3 — `_spec\\specs20.json`(v2) → `_spec\\specs20_v3.json`(v3) 재구성. (2026-09-11)

## 왜 스키마를 바꾸나
v2 는 같은 사실을 `logic`(산문) · `reads/writes`(표) · `constants` · `knobs` · `resolved` 에
**2~4곳 중복** 보관한다. 그래서 정정이 한 곳만 반영되고 나머지가 살아남았다
(2026-09-11 1·2차 반증검증에서 나온 ~90건 중 대부분이 이 부류).

v3 의 원칙 세 개:
 1. **정본 필드를 명시한다.** 오프셋=`mem` / 상수=`consts` / 시그니처=`callees`·`sig`.
    `logic` 은 **재구현용 의사코드일 뿐이고 어긋나면 정본이 맞다**(`precedence` 에 명문화).
 2. **게이트 필드는 사람이 안 채운다.** `callees[].sig` · `callers` · `siblings` 는
    tcx·IR 코퍼스에서 **자동 생성**한다 — 1·2차 누락의 3대 원인이 정확히 이 셋이었다.
 3. **`open[]` 에는 진짜 열린 것만.** 해소된 항목은 `closed[]` 로 옮긴다(삭제하지 않는다).
    3차는 `open[]` 만 보면 된다.

## 증거 등급 `ev`
 1 = 런타임 실측(game==mine DIFF=0)   2 = SDK 오라클 실행   3 = tcx/MIR 정본
 4 = LLVM IR 독해                     5 = 추론(근거 약함)
`ev<=3` 이 뒤집히면 사고다. `ev>=4` 가 뒤집히는 것은 정상 수렴이다.
"""
import io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import spec3lib as L
import tcxdict as TD
import tcxaudit as TA          # ★norm_base·Audit 은 여기 있다(tcxdict 아님)
sys.path.insert(0, os.path.join(HERE, '_spec'))
import closelist as CL

SRC = os.path.join(HERE, "_spec", "specs20.json")
DST = os.path.join(HERE, "_spec", "specs20_v3.json")
STRIKE = re.compile(r"~~(?!~).+?~~")

D = json.load(io.open(SRC, encoding="utf-8"))
S = D["specs"]

# ── 증거 등급 판정 ───────────────────────────────────────────────────
EV = [
    (1, (u"DIFF=0", u"비트동일", u"런타임 실측")),
    (2, (u"오라클", u"실행 확정", u"실행 확증", u"실행 확인", u"실행 검증", u"MATCH", u"실행으로")),
    # ★`dienum`·`distruct`·`divtable` 을 3차 §7 의 ev 상향 권고대로 추가했다(4차 반영).
    #   셋 다 DWARF/PDB 에서 뽑은 덤프라 **tcx 와 같은 등급**인데 초판 목록에 빠져 있어서
    #   태그값 행(ObjectPhase/MainObjective/JungleType)과 vtable 4행이 전부 ev4 로 앉아 있었다.
    #   ⟹ ev 는 손으로 매기지 않고 **근거 문면에서 파생**하므로, 빠진 재료 이름을 넣는 것이 정답이다.
    (3, (u"tcx", u"MIR", u"tcxdict", u"정본", u"DWARF", u"srcmap", u"줄 길이", u"줄길이",
         u"dienum", u"distruct", u"divtable")),
    (4, (u"IR", u".ll:", u"icmp", u"dbg", u"m0", u"g0")),
]


def ev_mem(txt):
    u"""★`mem`(오프셋·필드명) 전용 등급 — **`ev2`(오라클)로는 못 내려간다.**

    왜: `METHOD_MAP:190` 신뢰서열은 `런타임 > tcx(컴파일러 정본) > 오라클 > IR` 이고
    `:192` 가 「**타입·오프셋·판별자는 tcx 가 정본**」이라고 못 박는다. 그런데 `ev` 축은
    `2 오라클 < 3 tcx` 라 **두 문서가 정확히 반대**다(5차 배치B 적발, 배치A 와 정면 충돌).
    ⟹ 해소: **주장의 종류마다 강한 증거가 다르다.** 오프셋은 tcx 가, 동작·임계는 오라클이 강하다.
       오프셋을 `ev2` 로 올리면 **정본에서 파생으로 강등**되므로 `ev3` 이 상한이다.
       `offset_of!` 실행 대조는 tcx 와 **같은 등급의 교차검증**이라 등급이 아니라 근거를 보강한다.
    ⚠단 **런타임 실측(`ev1`)은 예외** — 서열상 tcx 보다 위다.

    ⚠이 상한을 `applypatch` 쪽 문자열로 걸려 했다가 실패했다: `ev` 는 **문면에서 파생**되는데
      근거 문자열에 "오라클"이 들어 있으면 그게 tier2 로 잡혀 상한이 무력화된다.
      **상한은 문면이 아니라 필드의 성질에 붙어야 한다.**
    """
    e = evtier(txt)
    return 3 if e == 2 else e


# ★`apply_evup` 이 설명란에 덧붙이는 **증거 꼬리**의 머리말. (`applypatch.EVMARK` 와 짝)
#   `ev` 는 이 꼬리에서 파생되는 게 맞지만, **`kind` 는 여기를 보면 안 된다**(7차 배치B).
_EVTAIL = re.compile(u"\\s*·\\s*(?:런타임 실측 DIFF=0|오라클 실행 확증|tcx 정본 대조|실행)\\s*\\(")


# ★★`kind` 파생 — **낱말 매칭에서 「IR 관측 우선 + 낱말은 구제」로** 바꿨다. (8차 배치B 설계)
#
# 왜 — `kind=임계` 107행 중 **순서비교가 관측되는 건 25행(23%)뿐**이었다. 나머지는
# `select` 28 · 산술 15 · `phi` 8 · `switch` 8 로, 갈 곳이 없어 **기본값 `임계` 로 떨어진 것**이다.
# 즉 「임계 111행」은 분류 결과가 아니라 **나머지**였다(무검사가 아니라 **무측정** 축).
#
# 규칙(배치B `gate.py` 의 반증식과 같은 비대칭):
#   ①강한 관측이 하나도 없으면(접힘·콜리 안) **낱말을 존중**한다 — 기각하지 않는다
#   ②낱말이 관측의 **지지를 받으면 낱말 우선**(구제)
#   ③낱말이 지지를 못 받으면 **관측이 정한다**
#   ⛔`GEP` 는 어디에도 쓰지 않는다 — 불투명 포인터 IR 에서 GEP 인덱스는 **첨자가 아니라 바이트 오프셋**이다
#   ⛔`STORE/PHI/SELECT` 를 `태그` 로 매핑하지 않는다 — 산출은 태그와 **산출값**을 못 가른다
#      (그렇게 매핑한 초판이 각각 7건씩 오적발했다)
KIND_PRI = ((u"CMP_ORD", u"임계"), (u"SWCASE", u"태그"), (u"CMP_EQ", u"태그"),
            (u"MINMAX", u"인덱스"), (u"ARITH", u"계수"),
            (u"PHI", u"산출값"), (u"SELECT", u"산출값"), (u"STORE", u"산출값"), (u"RET", u"산출값"))
_SENT = (u"센티널", u"니치", u"0xff", u"MAX")


def _word_kind(t):
    u"""현행 낱말 매칭 — 이제 **결정이 아니라 힌트**다. 부정문은 배치B 의 `neg_hit` 가 거른다.

    ★★체인 순서가 판정이다 — 낱말을 넣기 전에 **기존 행을 몇 개나 가져가는지 세라.**
      (11차 후속 실측: `길이` 는 15행에 닿고 그중 6행이 「배열 길이 2 = 비교 상한」이라
       낱말만 보면 정상 `임계` 를 빼앗는다. 그래서 `임계` **뒤**에 둔다. 그래도 남는
       위험은 규칙③이 막는다 — `CMP_ORD` 가 관측되면 지지 못 받는 낱말은 진다.)
    """
    if any(k in t for k in _SENT):
        return u"센티널"
    if any(k in t for k in (u"태그", u"판별자", u"variant")):
        return u"태그"
    if u"인덱스" in t:
        return u"인덱스"
    # ★`오프셋가감` — 가산·감산 **바이어스**. (11차 후속 신설, 실측 4행)
    #   `00 c9`(사거리에서 빼는 여유분) 처럼 「값에 더하고 빼는 상수」는 배율이 아니라서
    #   `계수` 로 부르면 틀린다. 어휘가 없어 `미상` 잔여 버킷에 고여 있던 자리다.
    if any(k in t for k in (u"여유분", u"여유치", u"바이어스", u"가감", u"보정치", u"마진")):
        return u"오프셋가감"
    if u"계수" in t or u"배율" in t:
        return u"계수"
    if any(k in t for k in (u"임계", u"문턱", u"하드컷", u"컷오프", u"상한", u"하한")):
        return u"임계"
    # ★`길이` — 문자열·배열의 바이트 길이. **반드시 `임계` 뒤**(위 주석).
    #   `05 c11~c13`("PassiveLine" 11 등)은 IR 명령엔 흔적이 없고 `#dbg_value` 에만 남아
    #   강한 관측이 0 이라 규칙①로 낱말이 그대로 채택된다 — 그래서 낱말이 정확해야 한다.
    if u"길이" in t:
        return u"길이"
    return None


def _kind(sp, x, kmat):
    word = _word_kind(kmat)
    # ★부정문 가드 — 「…는 **임계가 아니다**」를 임계로 읽던 구멍(8차 배치B 실측 4행).
    #   `_EVTAIL` 은 정형 꼬리만 자르는데 실제 오염은 **본문의 부정문**이었다.
    try:
        import kindchk as KC
        if word and KC.neg_hit(kmat, word):
            word = None
        ir = sp.get("ir") or {}
        obs = KC.observe(ir.get("file"), ir.get("frm"), ir.get("to"), x.get("value")) or {}
        strong = dict((k, v) for k, v in obs.items() if k not in KC.WEAK)
        if not strong:
            return word or u"미상"                      # ①접힘·콜리 안 → 낱말 존중
        # ★★②구제는 **`obs` 전체**를 본다 — `strong` 이 아니다. (12차 배치B 적발)
        #   `kindchk` 는 「`DBGSTR`·`CALLARG` 는 **구제에 쓰고 기각엔 안 쓴다**」고 명시하는데
        #   여기서 `strong` 을 보는 바람에 그 규칙이 **코드에서 무효**였다. 지지 관측이 전부
        #   WEAK 인 `길이` 는 원리적으로 구제 불가였고, `05 c14~c19`(패턴 바이트 길이 6행)가
        #   end_plan 코드값 4·5·6·8 과 **숫자가 겹쳐서** `산출값` 으로 오분류돼 있었다.
        #   ⚠기각 쪽(③)은 그대로 `strong` 을 본다 — `CMP_ORD` 가 관측되면 낱말이 지고
        #     「배열 길이 = 비교 상한」 6행은 계속 `임계` 로 보호된다.
        if word and any(c in obs for c in KC.SUPPORT.get(word, ())):
            return word                                 # ②구제
        for c, k in KIND_PRI:                           # ③관측이 정한다
            if c in strong:
                return k
        return word or u"미상"
    except Exception:
        return word or u"임계"                          # 계측 실패 시 예전 동작


def evtier(txt):
    t = txt or u""
    for tier, keys in EV:
        if any(k in t for k in keys):
            return tier
    if any(k in t for k in (u"추정", u"보인다", u"으로 보임", u"근거 없음")):
        return 5
    return 4


# ── 해소 판정: `open` 에서 내릴 것인가 ───────────────────────────────
DONE_MARK = (u"★해소", u"해소 —", u"확정(", u"→ **확정", u"-> **확정", u"삭제 —")
IDENT = re.compile(r"(?:\+0x[0-9a-f]{2,4}|[a-z][a-z0-9_]{4,40}\.rs:\d+|[a-z_][a-z0-9_]{5,40})")
UNSURE = re.compile(u"(확정 못|확정하지 못|미확인|확인 못|확인 안|확인 불가|추정|모름|알 수 없|"
                    u"미확정|안 봄|안 읽|못 했|못 함|미탐색|미독해|미검증|재료 부재|표기 불가)")


def is_closed(i, txt, resolved_blob):
    u"""★기본값은 **열림**이다. 닫는 것은 근거가 명시적일 때만.

    초판은 "resolved 와 식별자 2개 이상 겹치면 닫힘"으로 자동 판정했다가
    `get_input_target 내부는 안 봄`·`base_sub_goal 내부`·`single_tower_dive_is_viable` 같은
    **진짜 미탐색까지 닫았다.** 과하게 열어두면 3차가 재확인만 하고 넘어가지만,
    과하게 닫으면 3차가 그것을 '새 발견'으로 또 집는다 — 없애려는 게 정확히 그것이다."""
    if any(m in txt for m in DONE_MARK):
        return True, u"본문에 해소 표기가 있다"
    why = CL.closed_reason(i, txt)
    if why:
        return True, why
    return False, None


# ★4차 배치A 제안 + specgate G10 실측(4건)으로 확장했다(2026-09-11).
#   초판은 판정 어휘 3종을 **문자 그대로** 찾고 나머지를 전부 `미탐색` 으로 떨궜다.
#   그래서 ①"확정 불가"·"알 수 없다" 같은 **동의 표현이 재료 부재인데 미탐색**으로 갔고
#         ②**이미 확정된 사실 서술**까지 미탐색으로 앉아 다음 라운드가 그걸 또 팠다.
#   `미탐색` 은 "더 파면 나온다"는 약속이므로 남발하면 라운드마다 같은 항목이 부활한다.
CLASS = ((u"표기 불가", u"표기 불가"), (u"소스 표기는 확정 불가", u"표기 불가"),
         (u"재료 부재", u"재료 부재"), (u"원리적", u"재료 부재"),
         (u"알 수 없다", u"재료 부재"), (u"복원 불가", u"재료 부재"),
         (u"복원할 수 없다", u"재료 부재"), (u"구분 못 했다", u"재료 부재"),
         (u"교차검증을 못", u"재료 부재"), (u"대조하지 못했다", u"재료 부재"),
         (u"확정 불가", u"재료 부재"), (u"확인 불가", u"재료 부재"),
         (u"미탐색", u"미탐색"))

# ★★**범위 한정된 「불가」는 재료 부재가 아니라 미탐색이다.** (2026-09-11 5차 직전 교정)
#   `/dream` 때 `확정 불가`·`확인 불가` 를 기계적으로 `재료 부재` 로 매핑했는데, 원문을 읽어보니
#   **"이 함수만으론 확정 불가" · "이 범위에서 확인 불가" · "여기서는 확정 불가"** 였다.
#   재료가 없는 게 아니라 **호출부·생성부·피호출자를 아직 안 본 것**이다(전부 도달 가능).
#   ⟹ `SPEC_RUNBOOK §4` 가 "최악의 오염"이라 부르는 방향(미탐색을 불가로 적기)을 내가 냈다.
#      실측 3건: `00` isqrt 패닉(호출부) · `02` version 게이트(피호출자) · `02` team 일치(생성부).
SCOPED_IMPOSSIBLE = re.compile(
    u"(이 함수만|이 함수만으론|이 함수 안에서만|이 범위에서|이 범위 밖|여기서는|여기선|"
    u"이 명세 범위|담당 범위)[^.。\n]{0,40}(확정 불가|확인 불가|확정하지 못|확인하지 못)")

# ★표기 불가 — **동작은 확정**이고 소스 표기만 복원 못 하는 것. 「재료 부재」와 구별해야 한다
#   (재료 부재는 "더 볼 재료가 없다", 표기 불가는 "봐도 원본 표기는 안 남는다").
#   판별 신호 = 본인이 판정 영향 없음을 명시한다.
NOTATION_ONLY = re.compile(u"(판정에는 영향 없음|판정에 영향 없|논리값은 순서와 무관|"
                           u"논리값 동일|동작은 확정|외연이 (같|동일))")

# ★사실 서술 — 물음이 아니라 **확정된 관측·기록**이다. `open[]` 에 두면 안 된다(→ `notes[]`).
#   ⚠「알 수 없다/할 수 없다」는 재료 부재이므로 `수 없다` 를 배제 문맥으로 빼야 한다.
FACT_TAIL = re.compile(u"(?<!수 )(?:없다|아니다|않는다|존재하지 않는다|확인된 사실|"
                       u"무관하다|맞다|확정했다|확정된다|확정이다|확정|사실이다|"   # ★09-13 17차: specgate.gate10 과 어휘 통일(확정 단독 어미·사실이다)
                       # ↓ **규칙 적용·판단 종료 기록** — 5차 직전 전수 독해로 추가(4건 실측)
                       u"내려놨음|내려놨다|명시했다|명시해 두었다|올렸다|넣지 않았다|넣었다|"
                       u"확인한 값이다|판단했다|기록해 뒀다|적었다)"
                       # ↓ 뒤에 **괄호 인용**(근거·규칙 출처)이 붙어도 끝으로 본다.
                       #   실측: `… 넣지 않았다(SPEC_GUIDE §3 표 규칙).` 이 이것 때문에 미탐색으로 샜다.
                       u"(?:\\s*\\([^)]{0,70}\\))?\\s*[.。]?\\s*$")


# ★**의문형으로 시작하면 끝이 사실이어도 질문이다.** (5차 직전 실측 1건)
#   `함수 이름의 v30 이 무엇의 버전**인지** — 본문에 버전 게이트 분기가 없다(…).`
#   머리가 묻고 꼬리가 답하는 형태라 꼬리만 보면 `사실 서술` 로 과닫힌다.
#   ⟹ 첫 절(`—`/`:` 앞)이 의문 어미로 끝나면 **물음으로 확정**한다.
#   과닫힘은 과열림보다 나쁘다 — 다음 라운드가 그걸 '새 발견'으로 다시 집는다(closelist 의 설계 이유).
QUESTION_HEAD = re.compile(u"^[^—:\n]{0,180}(인지|는지|한가|인가|일까|무엇을|어떤 조건)\\s*[—:]")


# ★★**판정 어휘를 부정하는 문장을 그 어휘로 분류하면 안 된다.** (6차 배치C 적발)
#   `specs[13]/open[0]` 은 「**「재료 부재」가 아니라 「미탐색」이다**」라고 적혀 있는데도
#   `재료 부재` 로 계속 앉아 있었다 — 문자열이 들어 있다는 이유로.
#   ⟹ **정정문 자체가 오분류의 원인**이었다. 부정 문맥을 먼저 지우고 나서 어휘를 찾는다.
NEGATED = re.compile(u"[「『\"']?(재료 부재|표기 불가|미탐색|사실 서술)[」』\"']?\\s*"
                     u"(?:가|이|는|은)?\\s*(?:아니라|아니다|아님|이 아니|가 아니)")

# ★**명시적 자기 선언이 최우선이다.** 「… 「미탐색」이다」라고 써 있으면 그게 답이다.
#   6차 배치C 가 잡은 `specs[13]/open[0]` 이 그 실례인데, 부정 문맥을 지워도
#   같은 문장의 **다른 표현**(`IR 만으로는 알 수 없다`)이 `재료 부재` 로 걸려 여전히 틀렸다.
#   ⟹ 키워드 추론보다 **사람이 직접 적은 분류**를 먼저 본다. 추론은 선언이 없을 때만.
DECLARED = re.compile(u"(?<!아니라 )(?<!아니라)[「『](재료 부재|표기 불가|미탐색|사실 서술)[」』]"
                      u"\\s*(?:이다|다|로 분류|로 본다)")


def classify(txt):
    # ⓪부정 문맥 제거 — 「X 가 아니라 Y」의 X 를 먼저 지운다
    txt = NEGATED.sub(u" ", txt)
    # ⓪-b 명시적 자기 선언이 있으면 그걸 따른다(키워드 추론보다 우선)
    m = DECLARED.search(txt)
    if m:
        return m.group(1)
    # ①「동작 확정 + 표기만 불가」가 최우선 — 아래 불가 매핑에 먹히면 재료 부재로 오분류된다
    if NOTATION_ONLY.search(txt) and (u"복원" in txt or u"순서" in txt or u"표기" in txt):
        return u"표기 불가"
    # ②범위 한정된 불가는 미탐색 (재료 부재로 적으면 다음 라운드가 시도조차 안 한다)
    if SCOPED_IMPOSSIBLE.search(txt):
        return u"미탐색"
    for k, v in CLASS:
        if k in txt:
            return v
    lines = [ln.strip() for ln in txt.split(u"\n") if ln.strip()]
    if lines and FACT_TAIL.search(lines[-1]) and not QUESTION_HEAD.match(txt):
        return u"사실 서술"
    return u"미탐색"


# ── 술어·피호출자 이름 수집 ──────────────────────────────────────────
CALLNAME = re.compile(r"\b([a-z_][a-z0-9_]{3,45})\s*\(")
SKIP = set(u"""if while for match let return fn move some none ok err self
print format vec box new_with min max abs sub add mul div shl shr sat unwrap
expect clone copy into from as_ref as_mut iter map filter fold any all count len
is_some is_none is_ok is_err unwrap_or saturating_sub saturating_add wrapping_add
checked_add store load gep icmp select switch phi call invoke usub umin umax sext
zext trunc bitcast inttoptr ptrtoint memcpy memset
abs_diff and_then filter_map min_by_key max_by_key collect_in from_iter_in new_in
copied cloned flatten into_iter iter_mut is_some_and unwrap_failed drop_glue
panic_bounds_check panic_const_div_by_zero panic_const_div_overflow alloca dangling
call_mut call_once pipe choose true false pred game context blackboard nexus
growth_range radius_mult sdiv udiv srem urem""".split())
# ★위 목록은 std/bumpalo/LLVM intrinsic/패닉 헬퍼/필드명이다. game_ai·game_core 의
#   **판정 술어가 아니므로** 시그니처를 물을 대상이 아니다(specgate G4 잡음의 원인이었다).


def harvest_callees(sp):
    names = set()
    for c in sp.get("calls") or []:
        if isinstance(c, str):
            names.add(c.split("::")[-1].split("(")[0].strip())
        elif isinstance(c, dict):
            for k in ("name", "fn", "callee"):
                if c.get(k):
                    names.add(str(c[k]).split("::")[-1])
    for m in CALLNAME.finditer(STRIKE.sub(u" ", sp.get("logic") or u"")):
        n = m.group(1)
        if n not in SKIP and not n.startswith("_") and not n.startswith("llvm."):
            names.add(n)
    # ★길이 필터가 `new`(3자)를 **통째로 버리고 있었다.** (12차 배치D 적발)
    #   `15` 는 비다이브 경로 전체가 `SinglePlanBattle::new`(m13.ll:33428 에 IR 앵커 실물)인데
    #   `callees` 에 없었다(`new_dive` 만 있었다). 생성자가 판정 분기의 절반인 코퍼스에서 비싼 필터다.
    #   ⚠전면 완화(`>=3`)는 산문 잡음을 크게 늘리므로 **생성자 이름만** 예외로 둔다.
    KEEP3 = (u"new",)
    return sorted(n for n in names
                  if n and n not in SKIP and (len(n) > 3 or n in KEEP3))


# ★IR 범위에서 실제로 호출되는 망글링 심볼 — `callees` 후보의 **정답지**. (11차 후속)
#   `harvest_callees` 는 `calls` 와 `logic` **산문**에서 이름을 긁는다. 그 이름이 실제로
#   이 함수가 부르는 것인지 아닌지를 산문은 말해 주지 않는다(실측: `00` 의 `is_visible` 은
#   IR 범위에 없다 — 설명에 등장할 뿐이다). IR 을 보면 그것이 갈린다.
_CALLSYM = re.compile(r"@(_R[\w.$]+)")
_IRSY = {}
IRDIR = L.CORP["gai"]     # ★`sp["ir"].file` 은 `_gaibc` 기준 파일명이다(kindchk 와 같은 원점)


def _ir_callsyms(sp):
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    if not f or not a or not b:
        return []
    key = (f, a, b)
    if key not in _IRSY:
        try:
            src = io.open(os.path.join(IRDIR, f), encoding="utf-8",
                          errors="replace").read().split("\n")
        except Exception:
            _IRSY[key] = []
            return _IRSY[key]
        out = set()
        for ln in src[a - 1:b]:
            if "call" not in ln and "invoke" not in ln:
                continue
            out.update(_CALLSYM.findall(ln))
        _IRSY[key] = sorted(out)
    return _IRSY[key]


_IDENT = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")
# Rust 문법어 — 망글링 심볼에는 **토큰으로 안 들어간다.** 경로에서 빼야 한다.
_PKW = frozenset(("impl", "as", "dyn", "mut", "ref", "for", "where", "const", "Self"))


def path_idents(path):
    u"""경로에서 **대조에 쓸 식별자**를 뽑는다.

    ★`::` 로 쪼개면 안 된다. (12차 수정 중 자기적발 — 첫 판이 `ev3` 를 56→**206** 으로 부풀렸다)
      `<game_core::CombineEffect as game_core::EffectType>::range_adjust` 를 `::` 로 쪼개면
      식별자 형태인 조각이 **`range_adjust` 하나뿐**이라, 그 이름이 든 아무 심볼에나 걸린다.
      실측 오염 = `update` **119행** · `strategy` 21행이 한꺼번에 거짓 `ev3` 가 됐다.
    ⟹ 경로 **문자열 전체**에서 식별자를 긁되 ⓐ수명(`'a`)은 지우고 ⓑ문법어는 뺀다.
      그러면 `CombineEffect`·`EffectType` 같은 **판별력 있는 성분이 살아난다.**"""
    p = re.sub(r"'\w+", u" ", path or u"")
    return [s for s in re.findall(r"[A-Za-z_]\w*", p) if s not in _PKW]


def sym_tokens(sym):
    u"""망글링 심볼을 `[(이름, 시작위치)]` 로 푼다. `mangled_names` 의 위치 보존판.

    ★**부분문자열이 아니라 토큰이어야 한다.** (12차 A·B·D 전원 적발)
      구판은 `s in sym` 이라 `game_core`·`Effect`·`range` 가 전부
      `…NtB2_6Effect12range_adjust` 안에 들어가서 **`Effect::range` 가 호출된 적이 없는데도**
      `ev3`(IR 호출 심볼 일치) 도장을 받았다. 길이접두로 경계를 세면 `12range_adjust` 는
      `range_adjust` 한 토큰이라 `range` 와 안 맞는다. 전역 거짓 앵커 3건이 이 형태였다."""
    out = []
    for i in range(len(sym)):
        if not sym[i].isdigit() or (i and sym[i - 1].isdigit()):
            continue
        j = i
        while j < len(sym) and sym[j].isdigit():
            j += 1
        n = int(sym[i:j])
        if n < 1 or j + n > len(sym):
            continue
        name = sym[j:j + n]
        if _IDENT.match(name):
            out.append((name, j))
    return out


def base_tokens(sym):
    u"""**제네릭 인자 속 클로저를 뺀** 토큰 집합.

    ★왜 빼야 하나 (12차 배치A 적발) — `Vec::from_iter_in::<Filter<…, defensive_crisis::{closure#0}>>`
      의 심볼은 `…from_iter_in…NCNv…16defensive_crisis0E…` 라, 토큰 대조만 하면
      **`game_ai::defensive_crisis` 가 자기 자신을 호출했다**고 읽힌다(자기호출 날조 2건).
      실제 콜리는 `from_iter_in` 이고 `defensive_crisis` 는 **타입 인자**일 뿐이다.
    ⟹ 클로저 네임스페이스 마커 `NC` 가 처음 나오는 자리에서 자른다.
      ⚠`NC` 가 식별자 본문 안에 있으면 안 되므로 **토큰이 차지한 구간은 건너뛴다.**"""
    toks = sym_tokens(sym)
    spans = [(p, p + len(n)) for n, p in toks]
    cut = len(sym)
    for k in range(len(sym) - 1):
        if sym[k:k + 2] != "NC":
            continue
        if any(a <= k < b for a, b in spans):
            continue
        cut = k
        break
    return set(n for n, p in toks if p < cut)


def _rank_callees(rs, leaf, irsy, hint=u""):
    u"""후보를 **IR 호출 심볼과 대조**해 순위를 매긴다. `_anchor` = IR 에 실물이 있다.

    v0 망글링은 경로 성분을 `<길이><이름>` 으로 이어 붙이므로(`…6entity6Entity10is_visible`),
    경로의 각 성분이 **전부 토큰으로** 한 심볼 안에 있으면 그 후보다.
    ⚠`leaf` 만으로 대조하면 안 된다 — `tick`·`default` 처럼 흔한 이름이 아무 심볼에나 걸린다.

    ★**비식별자 성분은 빼고 센다.** (12차 A·B·D 적발 — 놓친 앵커 9건)
      `AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus` 를 `::` 로 쪼개면
      `<'a, 'b>` 조각이 나오는데 망글링에는 그런 문자열이 **없다** ⟹ 실제로 직접 호출되는
      함수가 영원히 `ev4 ⚠미확정` 에 갇혔다. `<impl … as …>` 도 같다.

    ★`hint` = 그 명세의 `mem`/`logic` 산문. 앵커가 없어 잘라야 할 때 **명세 자신이 지목한
      구현을 우선**한다. (12차 배치D 적발 — `15 mem[3]` 이 `divtable` 로 `ExpectedGame` 을
      지목했는데 정렬키가 「경로 길이」라 그 후보가 `rs[:3]` 밖으로 잘려 나갔다.)"""
    for r in rs:
        segs = path_idents(r.get("path"))
        # ★성분이 1개뿐이면 앵커로 안 쓴다 — 판별력이 없다(위 `path_idents` 주석).
        r["_anchor"] = len(segs) >= 2 and any(
            set(segs) <= base_tokens(sym) for sym in irsy)
        r["_named"] = bool(hint) and (r.get("path") or u"") in hint
    return sorted(rs, key=lambda r: (not r["_anchor"], not r["_named"],
                                     r.get("crate") != "game_ai",
                                     len(r.get("path") or u"")))


_INDIR = re.compile(r"=\s*(?:tail\s+|musttail\s+)?(?:call|invoke)\b[^@\n]*%\d+\s*\(")


def _ir_indirect(sp):
    u"""IR 범위에 **간접호출**(vtable)이 있나. (12차 B·C·D 공통 적발)

    ★`tick`·`is_visible`·`get_entity_by_id` 류는 `&dyn AbstractGame` 의 vtable 호출이라
      **망글링 심볼이 애초에 없다.** 그런데 `pick` 이 「IR 범위에 이 호출이 없다」고 적어
      **명백한 거짓**을 말하고 있었다. 「재료 부재」와 「원리적으로 이 재료로는 불가」는
      다른 판정이고, 뭉치면 다음 라운드가 헛돈다."""
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    if not f or not a or not b:
        return False
    try:
        src = io.open(os.path.join(IRDIR, f), encoding="utf-8",
                      errors="replace").read().split("\n")
    except Exception:
        return False
    return any(_INDIR.search(ln) for ln in src[a - 1:b])


# ★망글링 심볼에서 타입명을 뽑는다. rustc v0 망글링은 `<길이><이름>` 이라
#   길이접두를 **정확히 세어** 잘라야 한다.
#   ⚠초판은 `\d+([A-Z][A-Za-z0-9]*Plan)\b` 를 썼는데 `17LineGankCoverPlan15target_bush_v30` 처럼
#     `Plan` 뒤에 바로 숫자가 붙으면 워드경계가 성립하지 않아 **20개 전부 매치 0** 이었다.
#     그리고 specgate G3 가 `plan==null` 을 무조건 통과시켜 이 구멍을 못 잡았다
#     (3차 배치 C 가 13·14 에서 적발 — G3 가 막으려던 실패와 같은 형태).
LENPFX = re.compile(r"(\d+)([A-Za-z_][A-Za-z0-9_]*)")


def mangled_names(sym):
    u"""망글링 심볼 안의 `<길이><이름>` 후보를 **겹침 허용**으로 전부 돌려준다.

    ⚠순차 워커로 만들면 안 된다 — `NtB2_17LineGankCoverPlan` 에서 `2` 를 길이로 읽어
    `_1` 을 소비해 버리고 그 뒤 `17LineGankCoverPlan` 을 놓친다(실측 1/20).
    모든 시작 위치를 독립적으로 시도하고 길이가 맞는 것만 채택한다."""
    u"""⚠`re.finditer` 로도 안 된다 — 겹치지 않으므로 `…0ozCnw_7game_ai…` 의 `0` 이
    길이 0 으로 매치되며 문자열 전체를 삼킨다(실측 0/20). 위치를 직접 훑는다."""
    out = []
    for i in range(len(sym)):
        if not sym[i].isdigit() or (i and sym[i - 1].isdigit()):
            continue
        j = i
        while j < len(sym) and sym[j].isdigit():
            j += 1
        n = int(sym[i:j])
        if n < 1 or j + n > len(sym):
            continue
        name = sym[j:j + n]
        if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", name):
            out.append(name)
    return out


def plan_type(sp):
    u"""이 함수가 달려 있는 **impl 타입**을 돌려준다(자유 함수면 None).

    ⚠"Plan 으로 끝나는 것"만 찾으면 `LegacyPlanHandler`(05·06·11·12·15)·`DeathMatchBattle`(17)
    처럼 형제가 실제로 중요한 타입을 놓친다 — 3차 배치 C 가 `LegacyPlanHandler` 형제로
    `handle_chat_inner` 계열을 찾아냈다. **UpperCamel 이름 중 마지막 것**을 쓴다."""
    ups = [n for n in mangled_names(sp.get("sym") or u"")
           if n[:1].isupper() and "_" not in n]
    return ups[-1] if ups else None


# ── 변환 ─────────────────────────────────────────────────────────────
def conv(i, sp):
    resolved_blob = json.dumps(sp.get("resolved") or [], ensure_ascii=False)
    o = {
        "i": i, "id": sp.get("id"), "name": sp.get("name"), "sym": sp.get("sym"),
        "src": sp.get("src"), "src_line": sp.get("src_line"),
        "layer": sp.get("layer"), "ir": sp.get("ir"), "one_line": sp.get("one_line"),
    }

    # sig — tcx 정본으로 덮고, 사람이 쓴 note 는 role 로 보존
    leaf = (sp.get("name") or "").split("::")[-1]
    cands = L.fnlookup(leaf)
    mine = [c for c in cands if c.get("file") and
            os.path.basename(str(sp.get("src") or "")).lower() in str(c["file"]).lower()]
    # ★2026-09-13 정정(15차 배치A 적발): 같은 파일에 동명 함수가 여럿이면(lib.rs 의 `upgrade_item` = 트레이트 impl 메서드
    #   `<AgentVerHamster as AiAgent>::upgrade_item`(pub) / 고유 메서드 / 자유함수 `game_ai::upgrade_item`(in:game_ai)) 첫 후보를
    #   집어 sig.tcx·vis·path 가 **전부 다른 함수**로 오염됐다(#21/#23 · G5/G16 오탐의 근원). ⟹ **망글 심볼의 경로 토큰**
    #   (`_RNvCs…7game_ai12upgrade_item` → [game_ai, upgrade_item]) 과 후보 `path` 의 세그먼트 일치 수로 고르고,
    #   동점이면 명세 `src_line` 에 가까운 것. 심볼이 없을 때만 옛 규칙(파일 일치 → 첫 후보).
    def _symtoks(sym):
        out, i = [], 0
        rx = re.compile(r'(\d+)([A-Za-z_][A-Za-z0-9_]*)')
        while sym and i < len(sym):
            m = rx.match(sym, i)
            if m:
                n = int(m.group(1)); ident = sym[m.end(1):m.end(1) + n]
                if len(ident) == n and re.match(r'^[A-Za-z_][A-Za-z0-9_]*$', ident):
                    out.append(ident); i = m.end(1) + n; continue
            i += 1
        return out
    stoks = [t for t in _symtoks(sp.get("sym") or "") if t not in ("game_ai", "game_core")]
    pool = mine or cands
    if stoks and pool:
        def _score(c):
            segs = re.split(r"::|[<> ]", c.get("path") or "")
            hit = sum(1 for t in stoks if t in segs)
            # 자유함수 심볼(impl 토큰 없음)인데 후보 path 에 `<… as …>`/타입 세그먼트가 있으면 감점
            extra = len([s for s in segs if s and s not in stoks and s not in ("game_ai", "game_core")])
            dist = abs(int(c.get("line") or 0) - int(sp.get("src_line") or 0))
            return (-hit, extra, dist)
        tcxfn = sorted(pool, key=_score)[0]
    else:
        tcxfn = (pool or [None])[0]
    o["sig"] = {
        "tcx": (tcxfn or {}).get("sig"),
        "vis": (tcxfn or {}).get("vis"),
        "path": (tcxfn or {}).get("path"),
        "mir": (tcxfn or {}).get("mir"),
        "ev": 3 if tcxfn and tcxfn.get("sig") else 5,
        "params": [{"i": p.get("i"), "name": p.get("name"), "type": p.get("type"),
                    "role": p.get("note"), "ev": evtier(p.get("note"))}
                   for p in (sp.get("signature", {}) or {}).get("params", [])],
        "ret": (sp.get("signature", {}) or {}).get("returns"),
    }

    o["logic"] = sp.get("logic")
    o["logic_note"] = (u"★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/"
                       u"`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** "
                       u"(v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)")

    # mem — reads + writes 통합 + 사전 대조
    mem = []
    for dirk, field in (("r", "reads"), ("w", "writes")):
        for x in sp.get(field) or []:
            base, off = x.get("base"), x.get("offset")
            # ★`tcxaudit` 의 실제 감사 판정을 그대로 박는다(OK / 오귀속 / 밀림 / 부분일치 / 확인불가).
            #   ⚠초판은 `TD.norm_base` 를 불렀는데 그 함수는 `tcxdict` 가 아니라 `tcxaudit` 에 있어
            #     AttributeError 가 try 에 걸려 **449행 전부 "조회실패"** 였다(값도 무의미했다).
            try:
                A = TA.Audit()
                A.check(u"spec", base or u"", off or u"", x.get("name"))
                r = A.rows[-1] if A.rows else None
                chk = r[0] if r else u"판정없음"
                if r and r[6] and chk != u"OK":
                    chk = u"%s(%s)" % (chk, str(r[6])[:40])
            except Exception as e:
                chk = u"조회실패(%s)" % type(e).__name__
            row = dict(x)                      # ★원본 승계 — 골라 담으면 새 키가 사라진다
            # v2 행에 명시적 `dir` 이 있으면 존중한다("-" = 읽지도 쓰지도 않는 참조용 행)
            row.update({"dir": x.get("dir") or dirk, "chk": chk,
                        # ★mem(오프셋)은 tcx 가 정본이라 ev3 이 상한 — `ev_mem` 주석 참조
                        "ev": ev_mem(x.get("note"))})
            mem.append(row)
    o["mem"] = mem

    # consts — kind 를 붙여 '노브'와 '구조적 태그'를 가른다
    consts = []
    for x in sp.get("constants") or []:
        m = (x.get("meaning") or u"")
        # ★`kind` 파생에는 **증거 꼬리를 빼고** 본다. (7차 배치B 적발)
        #   `apply_evup` 이 근거를 `meaning` **본문에 덧붙이는데**, 그 근거 문장의 낱말이
        #   분류를 뒤집는다 — `07 consts[1]`(값 51, `hp_ratio<51` **임계**)이
        #   증거문 「… **태그** 5=Recall …」 때문에 `kind=태그` 로 뒤집혀 있다.
        #   증거는 `ev` 파생에만 쓰고(`evtier(m)` 은 전체를 본다), `kind` 는 원 설명만 봐야 한다.
        #
        # ⚠**이 절단만으로는 부족하다 — 넣고 재생성해도 `kind` 분포가 불변이었다(회귀는 없다).**
        #   `_EVTAIL` 은 `apply_evup` 이 만든 **정형 꼬리**만 자르는데, 실제 오염원인 위 07 행은
        #   이전 라운드가 **손으로 쓴 자유 서술**(「★**오라클 8/8**: … 태그 5=Recall …」)이라
        #   형식이 없어 안 잘린다. ⟹ 근본 해법 둘 중 하나가 필요하다:
        #     ① 증거를 `meaning` 이 아니라 **별도 키**로 뺀다(설명과 근거를 한 필드에 섞지 않는다)
        #     ② `kind` 를 낱말이 아니라 **IR 소비 오프코드**로 판다(`icmp`→임계 / `mul·shl`→계수 /
        #        `gep`→인덱스 / `phi·select`→태그) — 7차 배치B 의 `kindcheck.py`(G15 제안)가 그것이다.
        #   지금은 ①②가 안 된 상태라 **`meaning` 문면을 고쳐 우회**한다(배치B·D 의 patch 가 그 방식).
        kmat = _EVTAIL.split(m)[0]
        kind = _kind(sp, x, kmat)
        row = dict(x)
        row.update({"kind": kind, "ev": evtier(m)})   # consts 는 임계·태그라 오라클(ev2) 도달 가능
        consts.append(row)
    o["consts"] = consts

    # knobs — knobs + new_knobs 통합
    knobs = []
    for src, tag in ((sp.get("knobs") or [], u"기존"), (sp.get("new_knobs") or [], u"신규")):
        for x in src:
            row = dict(x)
            row.update({"src": tag,
                        "ev": evtier(u"%s %s %s" % (x.get("where"), x.get("effect"), x.get("note")))})
            knobs.append(row)
    o["knobs"] = knobs

    # ★callees — sig 를 자동으로 채운다 (1차 01 「유령 노브」의 원인 봉쇄)
    # ★★11차 후속: **경로를 버리고 leaf 로만 찾던 것**을 고쳤다.
    #   구판은 `fnlookup(leaf)` 결과를 `rs[:3]` 으로 **조용히 잘랐다.** 실측 184 이름 중 **36개**가
    #   잘렸고, `02 default` 는 후보가 **988개**였다 — 즉 그 행의 `path`·`sig`·`at` 은
    #   「임의의 세 개」이고, 그런데도 `ev:3`(tcx 정본) 도장이 찍혀 있었다.
    #   ⟹ **IR 범위의 망글링 호출 심볼**을 정답지로 삼아 순위를 매긴다. v0 망글링은 경로 성분을
    #      길이접두와 함께 그대로 담으므로(`…6effect…12range_adjust`) 부분문자열로 대조된다.
    # ★★12차: 네 배치가 전부 이 블록을 적발했다. 고친 것 넷 —
    #   ①토큰 경계(거짓 앵커 3) ②비식별자 성분 제외(놓친 앵커 9)
    #   ③**「후보 1개」를 정답으로 싣던 것**(배치D 실측: 자기 담당 58행 중 **16행(28%)**이
    #     「그 함수가 아님이 IR 로 증명되는」 경로를 정의처까지 달고 실려 있었다)
    #   ④`pick` 이 **세 가지 다른 사태**를 한 문장으로 뭉치던 것.
    irsy = _ir_callsyms(sp)
    irtok = set()
    for sym in irsy:
        irtok |= base_tokens(sym)
    indir = _ir_indirect(sp)
    hint = json.dumps({k: sp.get(k) for k in ("logic", "reads", "writes")},
                      ensure_ascii=False)
    callees, unmatched = [], []
    for n in harvest_callees(sp):
        rs = L.fnlookup(n)
        if not rs:
            unmatched.append(n)
            continue
        rs = _rank_callees(rs, n, irsy, hint)
        anch = [r for r in rs if r["_anchor"]]
        # ★★앵커가 없을 때 **「왜 없는지」가 셋으로 갈린다.** 뭉치면 안 된다.
        #   ⓐ IR 토큰에 leaf 는 **있는데** 어떤 후보도 안 맞음
        #     ⟹ 진짜 콜리가 **tcx 3크레이트 밖**(std·bumpalo·core::iter)이다.
        #       이때 후보를 실으면 **틀린 함수를 정의처까지 달아 싣는 것**이 된다
        #       (`15 drop` → `ProfTimer::drop` 인데 실제는 `Vec<Chat>::drop`,
        #        `18 push` → `SpitzDatable::serialize::push` 인데 실제는 `RawVec::grow_one`).
        #       ⟹ **아무것도 싣지 않고** `callees_unmatched` 로 보낸다.
        #   ⓑ leaf 가 IR 토큰에 없고 **간접호출이 있다** ⟹ vtable 호출이라 심볼이 원래 없다.
        #   ⓒ 그 외 ⟹ 후보 중 못 고름. ★「후보 1개」도 **정답이라는 뜻이 아니다**
        #       — `fnlookup` 이 tcx 3크레이트만 보므로 동명이 하나뿐인 것일 뿐이다.
        if not anch and n in irtok:
            unmatched.append(n + u"  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다"
                                 u"(std·bumpalo·core 등). 후보 %d개는 **전부 다른 함수**라 싣지 않는다"
                             % len(rs))
            continue
        take = anch if anch else rs[:3]
        for r in take:
            callees.append({"name": n, "path": r["path"], "vis": r["vis"],
                            "sig": r["sig"], "at": u"%s:%s" % (r["file"], r["line"]),
                            "mir": r["mir"], "xinl": r["xinl"],
                            "ev": 3 if r["_anchor"] else 4,
                            "pick": (u"IR 호출 심볼 일치(토큰 경계)" if r["_anchor"] else
                                     (u"⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. "
                                      u"확정하려면 `divtable` 로 슬롯을 풀어야 한다. "
                                      u"정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 "
                                      u"하나로 못 고른다. 후보 %d개 중 상위 %d개" % (len(rs), len(take)))
                                     if indir else
                                     (u"⚠미확정 — tcx 3크레이트에 같은 leaf 가 %d개. "
                                      u"IR 범위의 직접 호출 심볼과는 안 맞는다"
                                      u"(`logic` 산문에서 긁혔거나 인라인·접힘). "
                                      u"%s상위 %d개"
                                      % (len(rs),
                                         u"★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — "
                                         u"tcx 에 동명이 하나뿐일 뿐이다. " if len(rs) == 1 else u"",
                                         len(take))))})
    o["callees"] = callees
    o["callees_unmatched"] = {
        "names": unmatched,
        "note": u"tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 "
                u"std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. "
                u"⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, "
                u"3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것."}
    o["callees_note"] = (u"★tcx 에서 **자동 생성**한다(mkspec3.py). 사람이 채우는 필드로 두면 빈다 — "
                         u"01 의 「정글 캠프 티어 컷」 유령 노브가 `is_jungle(&self, usize)` 시그니처를 "
                         u"안 본 탓에 생겼다. **인자를 받는 술어면 IR 의 임계 상수는 접힘일 수 있다.**")

    # ★callers — 전수 자동 열거 (09 「유일 호출처」의 원인 봉쇄)
    sym = sp.get("sym") or ""        # ★IR 은 `@_RNv...` 라 leading underscore 를 벗기면 0건이 된다
    sites = L.callsites(sym, dirs=("gai",)) if sym else []
    o["callers"] = {
        "count": len(sites),
        "sites": [u"%s:%d" % (s["file"], s["line"]) for s in sites],
        "scope": u"_gaibc(game_ai) 전량 call/invoke 스캔",
        "note": u"★자동 열거(spec3lib.callsites, 0.7초). "
                u"1차가 09 를 「유일 호출처」로 적었다가 실제 8곳이었다 — 세지 않고 쓰는 일을 구조적으로 막는다. "
                u"exe 쪽 `callers: []` 는 '호출자 없음'이 아니라 **조인 실패**다.",
        "ev": 4,
    }

    # ★siblings — 같은 Plan 의 다른 진입점 (14 target_bush_v41 누락의 원인 봉쇄)
    pt = plan_type(sp)
    if pt:
        sib = L.siblings(pt)
        o["siblings"] = {
            "plan": pt, "count": len(sib),
            "entries": [{"path": s["path"], "vis": s["vis"], "at": u"%s:%s" % (s["file"], s["line"]),
                         "mir": s["mir"], "sig": s["sig"]} for s in sib],
            "note": u"★자동 열거. 14 가 `update` 만 보고 `sub_plan`/`next_plan` 이 쓰는 "
                    u"`target_bush_v41`·`target_bush` 를 통째로 놓쳤다 — 형제를 안 보면 "
                    u"'이 플랜의 목표'를 틀리게 재구현한다.",
            "ev": 3,
        }
    else:
        o["siblings"] = {"plan": None, "count": 0, "entries": [],
                         "note": u"Plan 타입 메서드가 아니다(자유 함수)."}

    # open / closed
    op, cl, nt = [], [], []
    for txt in (sp.get("unknown") or []) + (sp.get("still_unknown") or []):
        t = str(txt)
        done, why = is_closed(i, t, resolved_blob)
        if done:
            cl.append({"q": t, "why": why})
            continue
        klass = classify(t)
        if klass == u"사실 서술":
            # ★확정된 관측은 `open[]` 에 두지 않는다(4차 배치A 제안 + specgate G10).
            #   미탐색으로 앉혀 두면 다음 라운드가 그걸 '새 발견'으로 또 집는다.
            nt.append({"q": t, "class": klass, "ev": evtier(t)})
        else:
            op.append({"q": t, "class": klass, "ev": evtier(t)})
    o["open"] = op
    o["closed"] = cl
    o["notes"] = nt
    o["open_note"] = (u"★다음 라운드는 **`open[]` 만** 보면 된다. `closed[]` 는 이미 닫힌 것"
                      u"(삭제하지 않고 남긴다), `notes[]` 는 **확정된 사실 서술**이라 물음이 아니다"
                      u"(파지 말고, 틀렸으면 반증할 것). "
                      u"판정 어휘 = 미탐색 / 재료 부재(범위 열거 필수) / 표기 불가 / 사실 서술. "
                      u"「불가」를 쓰기 전에 `METHOD_MAP §0` 을 볼 것.")

    # ★절단 금지 — 초판이 400자로 잘라 정보를 흘렸다. resolved 원문을 그대로 보존한다.
    o["history"] = [{"was": r.get("was"), "now": r.get("now"),
                     **{k: v for k, v in r.items() if k not in ("was", "now")}}
                    for r in (sp.get("resolved") or [])]
    o["history_note"] = (u"v2 `resolved[]` 원문 그대로(정정 이력). **본문이 아니라 참조용**이다 — "
                         u"여기 있는 사실이 `mem`/`consts`/`knobs`/`callees` 에 반영됐는지는 "
                         u"`specgate.py` 가 검사한다.")
    o["exe"] = sp.get("exe")
    o["calls_raw"] = sp.get("calls")          # v2 원문 보존(손실 방지)
    o["base_round"] = sp.get("base_round")
    o["rounds"] = sp.get("rounds")
    # ★남은 v2 키는 이름을 몰라도 통째로 승계한다 — 하드코딩하면 새 키가 조용히 사라진다
    KNOWN = {"id", "name", "sym", "src", "src_line", "layer", "ir", "one_line", "signature",
             "logic", "reads", "writes", "constants", "calls", "knobs", "unknown", "exe",
             "resolved", "new_knobs", "still_unknown", "base_round", "rounds"}
    extra = {k: v for k, v in sp.items() if k not in KNOWN}
    if extra:
        o["v2_extra"] = extra
    return o


def main():
    out = {
        "meta": {
            "game": D["meta"].get("game"), "date": "2026-09-11", "schema": "v3",
            "what": u"game_ai 판단함수 20개 명세 — 1·2차 반증검증 반영본(v3 재구성).",
            "precedence": u"★정본 우선순위: mem(오프셋) · consts(상수) · callees/sig.tcx(시그니처) "
                          u"> logic(의사코드). 어긋나면 앞의 것이 맞다.",
            "ev_tiers": {"1": u"런타임 실측(game==mine DIFF=0)", "2": u"SDK 오라클 실행",
                         "3": u"tcx/MIR/DWARF 정본", "4": u"LLVM IR 독해", "5": u"추론(근거 약함)"},
            "ev_rule": u"ev<=3 이 뒤집히면 사고다. ev>=4 가 뒤집히는 것은 정상 수렴이다.",
            "autofill": u"callees[].sig · callers · siblings 는 tcx/IR 코퍼스에서 자동 생성"
                        u"(spec3lib.py). 사람이 채우지 않는다.",
            "corrections": D["meta"].get("corrections", []),
        },
        "shared": D.get("shared"),
        "specs": [],
    }
    tot = {"open": 0, "closed": 0, "knobs": 0, "callees": 0, "unres": 0, "callers": 0}
    for i, sp in enumerate(S):
        o = conv(i, sp)
        out["specs"].append(o)
        tot["open"] += len(o["open"]); tot["closed"] += len(o["closed"])
        tot["knobs"] += len(o["knobs"]); tot["callees"] += len(o["callees"])
        tot["unres"] += len(o["callees_unmatched"]["names"])
        tot["callers"] += o["callers"]["count"]
        print(u"[%02d] %-44s open %2d / closed %2d · knobs %2d · callees %2d(미매칭 %d) · callers %d"
              % (i, (o["name"] or "")[:44], len(o["open"]), len(o["closed"]), len(o["knobs"]),
                 len(o["callees"]), len(o["callees_unmatched"]["names"]),
                 o["callers"]["count"]))
    # ★closelist 감사 — 죽은 needle 이 있으면 크게 찍는다(조용한 no-op 금지)
    try:
        a = CL.audit(out["specs"])
        nd, nw, no = len(a["dead"]), len(a["wrongidx"]), len(a["over"])
        if nd or nw:
            print("")
            print("!" * 92)
            print(u"!! closelist 죽은 needle — 문면불일치 %d · index오기 %d (조용히 아무것도 닫지 않는다)" % (nd, nw))
            for j, i, n, o in a["wrongidx"]:
                print(u"!!   CLOSE[%d] (%d, %r) -> 실제 index %s" % (j, i, n[:40], o))
            for j, i, n, o in a["dead"]:
                print(u"!!   CLOSE[%d] (%d, %r) -> 어디에도 없음" % (j, i, n[:40]))
            print("!" * 92)
        else:
            print(u"closelist 감사: 죽은 needle 0 (중복닫힘 %d건은 허용)" % no)
    except Exception as e:
        print(u"closelist 감사 실패: %s" % e)

    out["meta"]["counts"] = {
        "functions": len(out["specs"]), "open": tot["open"], "closed": tot["closed"],
        "knobs": tot["knobs"], "callees": tot["callees"],
        "callees_unresolved": tot["unres"], "caller_sites": tot["callers"],
        "mem": sum(len(x["mem"]) for x in out["specs"]),
        "consts": sum(len(x["consts"]) for x in out["specs"]),
    }
    with io.open(DST, "w", encoding="utf-8", newline="\r\n") as f:
        f.write(json.dumps(out, ensure_ascii=False, indent=1))
    print(u"\n" + "=" * 92)
    print(u"%s  (%.0f KB)" % (DST, os.path.getsize(DST) / 1024.0))
    print(u"  " + u" · ".join(u"%s %s" % (k, v) for k, v in out["meta"]["counts"].items()))


if __name__ == "__main__":
    main()
