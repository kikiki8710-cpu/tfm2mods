# -*- coding: utf-8 -*-
u"""tcxaudit — 이미 발표한 오프셋 주장을 **tcx 정본 사전(tcxdict)** 으로 전수 재확인.

대상:
  1) _spec\\specs20.json          specs[].reads / writes  (base, offset, name)
  2) _spec\\specs20.json          shared[] 안의 오프셋 표(Entity_공통_오프셋 / Blackboard.fields / 맵_좌표계.그리드 …)
  3) _spec\\resolved_2026-09-10.json  같은 형태의 오프셋 표 + 함수별 오버레이
  4) (선택) --md <파일>          마크다운 안 `<별칭>+0xNNN` 패턴

판정:
  OK        사전이 그 오프셋에서 같은 필드를 준다
  오귀속     다른 구조체/다른 필드다
  밀림       동명 struct/enum-variant 혼동으로 +N 어긋남(= distruct 의 대표 결함)
  확인불가   사전에 그 타입이 없다 / 대상이 vtable·bump alloc 등 구조체가 아니다

⚠`~~취소선~~`(§7 정정형 기록의 **이미 정정된 옛 값**) 안의 오프셋은 스캔에서 제외한다(2026-09-11).
  그것을 잡으면 정정을 쓸 때마다 영구 오탐이 하나씩 쌓인다(실측: `~~PlayerState+0x2496~~` 이
  배치 A·D 감사에서 나란히 「오귀속」으로 잡혔다). 제외 건수는 리포트 머리에 찍는다 — 조용한 억제 금지.

사용: python -X utf8 tcxaudit.py            (전량)
      python -X utf8 tcxaudit.py --only specs
"""
import json, io, os, sys, re
from collections import defaultdict, Counter

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import tcxdict as TD

try:
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
except Exception:
    pass

SPEC = os.path.join(HERE, "_spec", "specs20.json")
RESOLVED = os.path.join(HERE, "_spec", "resolved_2026-09-10.json")

# ── base 이름 → full def_path (근거를 함께 남긴다) ─────────────────────
# 명세가 쓰는 짧은 이름은 leaf 다. leaf 가 게임 안에서 유일하면 자동 해결되고,
# 아래 표는 **자동 해결이 모호하거나 별칭인 것만** 손으로 못박은 것이다.
ALIAS = {
    "PlayerState": "game_core::PlayerState",
    "PlayerState(=GamePlayer info)": "game_core::PlayerState",
    "PlayerState(GamePlayer)": "game_core::PlayerState",
    "OperationData(p3 data)": "game_ai::OperationData",
    "SubPlan(sret)": "game_ai::plan_legacy::sub_plan::SubPlan",
    "SubPlan(sret 반환버퍼 %0)": "game_ai::plan_legacy::sub_plan::SubPlan",
    "SubPlan(sret) LineDefense 페이로드": "game_ai::plan_legacy::sub_plan::SubPlan",
    "dyn AbstractGame vtable": "@SKIP:vtable 슬롯(구조체 아님) — divtable.py 소관",
    "AbstractGame vtable": "@SKIP:vtable 슬롯",
    "AbstractGame::vtable": "@SKIP:vtable 슬롯",
    "dyn EffectType vtable": "@SKIP:vtable 슬롯",
    # bumpalo Vec 은 표준 Vec 과 레이아웃이 다르다(할당자 참조가 끼어든다).
    # tcx 그래프에 실물 인스턴스가 있으므로 그걸로 검증한다.
    #   Vec{ buf: RawVec{ ptr@0x0, a(&Bump)@0x8, cap@0x10 }, len@0x18 }  size=32
    "Vec<&Entity>(bumpalo, 32B)": "bumpalo::collections::vec::Vec<'{erased}, &'{erased} game_core::Entity>",
    "bumpalo Vec<&Entity>": "bumpalo::collections::vec::Vec<'{erased}, &'{erased} game_core::Entity>",
    # Vec<JungleType> 판은 그래프에 없다(지역변수 타입) — 같은 제네릭의 다른 인스턴스로 대체 검증
    "bumpalo Vec<JungleType>": "bumpalo::collections::vec::Vec<'{erased}, &'{erased} game_core::Entity>",
    "TraceEventType::CallHandled": "game_ai::plan_legacy::handler::TraceEventType",
    "TraceEventType": "game_ai::plan_legacy::handler::TraceEventType",
    "Option<Input> (sret 반환슬롯)": "std::option::Option<game_core::Input>",
    "반환 Option<SinglePlanBattle>": "std::option::Option<game_ai::plan_legacy::old::SinglePlanBattle>",
    "BattlePlanGoal::TryKill(p2 goal)": "game_ai::plan_legacy::old::BattlePlanGoal",
    "BattlePlanGoal(p2 goal)": "game_ai::plan_legacy::old::BattlePlanGoal",
    "BattlePlanGoal(스택 24B 임시)": "game_ai::plan_legacy::old::BattlePlanGoal",
    "MainObjective(인자 %4, i24)": "game_ai::plan_legacy::team_plan::MainObjective",
}

# shared 그룹 이름 -> 베이스 타입 (그룹명에서 기계적으로 못 뽑는 것만)
GROUP_BASE = {
    u"맵_좌표계": "game_core::MapDef",
    u"ff_call_계측_8칸": "game_ai::plan_legacy::handler::LegacyPlanHandler",
    u"Entity_공통_오프셋": "game_core::Entity",
    u"Blackboard": "game_core::Blackboard",
    u"위협모델_ChampionCache": "game_core::ChampionCache",
    u"AbstractGame_vtable": "@SKIP:vtable 슬롯",
    u"EffectType_vtable": "@SKIP:vtable 슬롯",
    u"AbstractGame vtable": "@SKIP:vtable 슬롯",
    u"포맷템플릿_문법": "@SKIP:포맷 옵코드 표(구조체 아님)",
    u"mf_src_코드표": "game_ai::plan_legacy::handler::LegacyPlanHandler",
    u"SubPlan_merge": "game_ai::plan_legacy::sub_plan::SubPlan",
}
# 괄호/한글 주석을 떼면 leaf 가 되는 것들
PAREN = re.compile(r"\s*[\(（].*$")


def norm_base(b):
    if b in ALIAS:
        return ALIAS[b]
    x = PAREN.sub("", b).strip()
    x = x.replace("dyn ", "").strip()
    return x


def nav(tystr, dotted):
    u"""`MapDef.fountains[team]` 처럼 base 가 '타입.필드[..]' 인 경우,
    그 필드(배열이면 원소)의 타입 문자열을 돌려준다."""
    cur = tystr
    for seg in dotted.split("."):
        arr = seg.endswith("]")
        seg = IDXPAT.sub("", seg)
        o = TD.T(cur)
        if o is None:
            return None
        nxt = None
        for v in o.get("vs", []) or []:
            for f in v["f"]:
                if f["n"] == seg:
                    nxt = f["t"]
                    break
            if nxt:
                break
        if nxt is None:
            return None
        cur = nxt
        while arr:
            o2 = TD.T(cur)
            if o2 and o2.get("k") == "array":
                cur = o2["el"]
                arr = False
            else:
                break
        # 첨자를 안 썼어도 배열이면 원소로 내려가는 편이 명세 관례에 맞다
    return cur


def parse_off(offs):
    m = re.search(r"0x[0-9a-fA-F]+", str(offs))
    if m:
        return int(m.group(0), 16)
    m = re.search(r"\d+", str(offs))
    return int(m.group(0)) if m else None


TAGWORDS = (u"판별자", u"discriminant", u"discr", u"tag", u"태그", u"니치", u"niche")
IDXPAT = re.compile(r"\[[^\]]*\]")


def segs_of(n):
    u"""점표기 이름 -> 비교용 세그먼트 목록.
    - `[team]` `[0]` `[2][5]` 같은 **첨자는 전부 제거**(명세는 변수명, tcx 는 0 을 쓴다)
    - `__0` / `0` 같은 튜플 인덱스는 `#` 로 통일
    - 판별자 표현은 전부 `tag`
    - `Some`/`Ok` 등 Option/Result 래퍼 세그먼트는 제거(명세는 보통 안 쓴다)"""
    n = IDXPAT.sub("", n)
    n = n.replace("@", ".")
    out = []
    for s in n.split("."):
        s = s.strip()
        if not s:
            continue
        if s in ("Some", "Ok"):
            continue
        if s.startswith("__") and s[2:].isdigit():
            s = "#"
        elif s.isdigit():
            s = "#"
        elif any(w in s for w in TAGWORDS):
            s = "tag"
        out.append(s)
    return out


def norm_name(n):
    u"""명세의 필드 이름 문자열을 점표기로 정리.
    ⚠`Input::Ult.target` 처럼 `::` 가 든 이름을 ':' 로 쪼개면 안 된다(초판에서 실제로 밟았다)."""
    if n is None:
        return ""
    n = n.strip().replace("::", ".")
    n = re.split(r"[ (（—·]|(?<!:):(?!:)", n)[0]
    return n.replace("@", ".").strip(".")


WHOLE = (u"전체", u"통째", u"sret", u"memcpy", u"덮어씀")


def tcx_names_at(tystr, off):
    rows = TD.walk(tystr, want=off)
    out = []
    for o, name, t, sz, note in rows:
        if o != off:
            continue
        out.append((name.replace("@", "."), t, sz, 0))
    if not out:                       # 정확히 그 오프셋에서 시작하는 leaf 가 없으면 커버 leaf
        for o, name, t, sz, note in rows:
            out.append((name.replace("@", "."), t, sz, off - o))
    return out


def sub_at(hay, ned):
    u"""ned 가 hay 의 연속 부분수열인가."""
    if not ned:
        return False
    for i in range(0, len(hay) - len(ned) + 1):
        if hay[i:i + len(ned)] == ned:
            return True
    return False


def seg_eq(a, b):
    if a == b:
        return True
    # 타입이름 vs variant 이름 표기차: LineDefenseSubPlan ↔ LineDefense
    if a[:1].isupper() and b[:1].isupper() and (a.startswith(b) or b.startswith(a)):
        return True
    return False


def subseq(hay, ned):
    u"""ned 가 hay 의 (연속이 아니어도 되는) 부분수열인가 — 중간 경로 생략 표기 허용."""
    if not ned:
        return False
    i = 0
    for h in hay:
        if i < len(ned) and seg_eq(ned[i], h):
            i += 1
    return i == len(ned)


def cmp_name(claim, cands):
    c = norm_name(claim)
    if not c:
        return "OK?" if cands else "MISS"
    if not cands:
        return "MISS"
    raw = str(claim)
    # 판별자/니치 주장: 그 오프셋에 태그가 있으면 맞다
    if any(w in raw for w in TAGWORDS):
        for name, t, sz, delta in cands:
            if delta == 0 and segs_of(name)[-1:] == ["tag"]:
                return "OK"
    # "전체 32B" 처럼 구조체 통째 쓰기
    if any(w in raw for w in WHOLE):
        return "OK?"
    cs = segs_of(c)
    if not cs:
        return "OK?"
    best = "BAD"
    cs2 = [s for s in cs if s != "#"]
    for name, t, sz, delta in cands:
        ts = segs_of(name)
        if sub_at(ts, cs) or sub_at(cs, ts):
            return "OK" if delta == 0 else "OK~"
        ts2 = [s for s in ts if s != "#"]
        if cs2 and (subseq(ts2, cs2) or subseq(cs2, ts2)):
            best = "OK~"          # 중간 경로 생략/표기차
            continue
        # 명세가 이름 앞에 **타입 이름**을 붙인 경우(`Input::Ult.target`) 첫 성분을 떼고 재시도
        if len(cs2) > 1 and cs2[0][:1].isupper() and subseq(ts2, cs2[1:]):
            best = "OK~"
            continue
        # 튜플 필드는 이름이 없다 — 명세가 붙인 별명(lx/ly)은 오프셋만 맞으면 OK
        if delta == 0 and ts2 == [] and len(cs2) <= 1:
            best = "OK~"
            continue
        if best == "BAD" and cs2 and ts2 and seg_eq(cs2[-1], ts2[-1]):
            best = "OK~"
        elif best == "BAD" and cs2 and ts2 and seg_eq(cs2[0], ts2[0]):
            best = "PREFIX"
    return best


class Audit(object):
    def __init__(self):
        self.rows = []
        self.cnt = Counter()
        self.masked = Counter()      # 파일별로 ~~취소선~~ 안에서 건너뛴 오프셋 주장 수

    def add(self, verdict, src, base, off, claim, got, note=""):
        self.rows.append((verdict, src, base, off, claim, got, note))
        self.cnt[verdict] += 1

    def check(self, src, base, offs, claim, note=""):
        nb = norm_base(base)
        if nb.startswith("@SKIP"):
            self.add(u"확인불가", src, base, offs, claim, "", nb[6:])
            return
        off = parse_off(offs)
        if off is None:
            self.add(u"확인불가", src, base, offs, claim, "", u"오프셋 파싱 실패")
            return
        # base 가 '타입.필드' 형태면 그 필드 타입으로 내려간다
        sub = None
        if "." in nb and "::" not in nb:
            head, sub = nb.split(".", 1)
            nb = head
        elif "::" in nb and nb not in TD.D()["by_path"]:
            pass
        st, cands = TD.resolve_name(nb)
        if st == "none":
            va = TD.variant_alias(nb)
            if va:
                self.add(u"확인불가", src, base, offs, claim, "",
                         u"타입 없음 — 다만 **열거형 variant** 로는 존재: %s (밀림 후보)" % ", ".join(va))
            else:
                self.add(u"확인불가", src, base, offs, claim, "", u"tcx 사전에 타입 없음")
            return
        tys = [t for _, ts in cands for t in ts]
        gplain = [t for t in tys if (TD.T(t) or {}).get("game") and "<" not in t]
        plain = [t for t in tys if "<" not in t]
        if len(tys) == 1:
            ty = tys[0]
        elif len(gplain) == 1:
            ty = gplain[0]
        elif len(plain) == 1:
            ty = plain[0]
        else:
            self.add(u"확인불가", src, base, offs, claim, "",
                     u"★모호: 동명 def_path %d개 %s"
                     % (len(set((TD.T(t) or {}).get("p") for t in tys)),
                        [(t, (TD.T(t) or {}).get("sz")) for t in tys][:6]))
            return
        if sub:
            ty2 = nav(ty, sub)
            if ty2 is None:
                self.add(u"확인불가", src, base, offs, claim, "", u"베이스 하위경로 '%s' 를 %s 에서 못 찾음" % (sub, ty))
                return
            ty = ty2
        cands2 = tcx_names_at(ty, off)
        got = " | ".join("%s%s : %s" % (n, ("(+%d)" % dl if dl else ""), t)
                         for n, t, s, dl in cands2[:3]) or u"(없음=패딩/미관통)"
        v = cmp_name(claim, cands2)
        # 밀림 검사: 같은 이름의 enum variant 로 해석하면 맞는가
        if v == "BAD":
            for vt in TD.variant_alias(nb):
                vo = TD.T(vt)
                for vv in vo["vs"]:
                    if vv["n"] != nb.rsplit("::", 1)[-1]:
                        continue
                    base_off = min((f["o"] for f in vv["f"] if f.get("o") is not None), default=None)
                    if base_off is None:
                        continue
                    for f in vv["f"]:
                        if f.get("o") == off and norm_name(claim).split(".")[-1] == f["n"]:
                            self.add(u"밀림", src, base, offs, claim,
                                     got, u"독립 struct %s 로는 안 맞고, **%s::%s** variant 로 보면 맞다(페이로드 +0x%x 밀림)"
                                     % (ty, vt, vv["n"], base_off))
                            return
        if v == "PREFIX":
            self.add(u"부분일치", src, base, offs, claim, got,
                     u"첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 한다")
        elif v in ("OK", "OK~", "OK?"):
            note2 = {"OK": u"", "OK~": u"표기차(첨자/튜플인덱스/타입이름 접두)",
                     "OK?": u"이름 미기재(오프셋만 검증)"}[v]
            self.add(u"OK", src, base, offs, claim, got, note2)
        elif v == "MISS":
            self.add(u"오귀속", src, base, offs, claim, got, u"그 오프셋에 필드가 없다(패딩/관통불가)")
        else:
            self.add(u"오귀속", src, base, offs, claim, got, u"사전은 다른 필드를 준다")


HEXKEY = re.compile(r"^\+?0x[0-9a-fA-F]+$")


def audit_spec_rw(A):
    d = json.load(io.open(SPEC, encoding="utf-8"))
    for sp in d["specs"]:
        for k in ("reads", "writes"):
            for e in sp.get(k, []):
                A.check(u"specs20/%s/%s" % (sp["id"], k), e.get("base", "?"), e.get("offset", "?"), e.get("name"))
    return d


def audit_shared(A, d, tag):
    u"""shared/_shared 안의 오프셋 표를 자동 인식해 감사."""
    sh = d.get("shared") or d.get("_shared") or {}
    for grp, v in sh.items():
        if not isinstance(v, dict):
            continue
        # (a) {"0x68": "설명"} 형태
        base = GROUP_BASE.get(grp, grp.split("_")[0])
        hexk = [k for k in v if HEXKEY.match(str(k))]
        if hexk:
            for k in hexk:
                A.check(u"%s/shared/%s" % (tag, grp), base, k, str(v[k]))
        # (b) {"fields":[{off,name}]} / {"그리드":[{off,name}]} / {"표":[{off,name}]}
        for lk, lv in v.items():
            if not isinstance(lv, list):
                continue
            for it in lv:
                if not isinstance(it, dict):
                    continue
                off = it.get("off") or it.get("offset")
                if off is None or not HEXKEY.match(str(off)):
                    continue
                A.check(u"%s/shared/%s.%s" % (tag, grp, lk), base, off, it.get("name"))


def audit_resolved(A):
    d = json.load(io.open(RESOLVED, encoding="utf-8"))
    audit_shared(A, d, "resolved")
    # 함수별 오버레이 안의 {off,name} 리스트
    for fn, v in d.items():
        if fn.startswith("_") or not isinstance(v, dict):
            continue
        stack = [(fn, v)]
        while stack:
            p, o = stack.pop()
            if isinstance(o, dict):
                off = o.get("off") or o.get("offset")
                base = o.get("base") or o.get("type") or o.get("구조체")
                if off and base and HEXKEY.match(str(off)):
                    A.check(u"resolved/%s" % p, str(base), str(off), o.get("name"))
                for k2, v2 in o.items():
                    if isinstance(v2, (dict, list)):
                        stack.append((p + "/" + str(k2), v2))
            elif isinstance(o, list):
                for i, x in enumerate(o):
                    if isinstance(x, (dict, list)):
                        stack.append((p + "[%d]" % i, x))


def report(A):
    order = [u"오귀속", u"밀림", u"부분일치", u"확인불가", u"OK"]
    print(u"\n" + "=" * 100)
    print(u"## 마이그레이션 감사 결과 — tcx 정본 사전 대조 (게임 0.5.8)")
    print(u"총 %d건  " % len(A.rows) + "  ".join(u"%s=%d" % (k, A.cnt[k]) for k in order))
    if A.masked:
        print(u"※ ~~취소선~~(이미 정정된 옛 값)이라 스캔에서 제외한 오프셋 주장 %d건 — %s"
              % (sum(A.masked.values()),
                 ", ".join(u"%s %d" % (k, v) for k, v in sorted(A.masked.items()))))
    print("=" * 100)
    for v in order:
        rows = [r for r in A.rows if r[0] == v]
        if not rows:
            continue
        print(u"\n### [%s] %d건" % (v, len(rows)))
        if v == u"OK":
            sub = Counter(r[6] for r in rows)
            for k, n in sub.most_common():
                print(u"   %-24s %d건" % (k or u"완전일치", n))
            for r in rows:
                if r[6]:
                    print(u"   · %-38s %-30s %-8s 주장=%-28s tcx=%s" % (r[1], r[2], r[3], r[4], r[5]))
            continue
        for r in rows:
            print(u"\n- %s   base=%s  offset=%s" % (r[1], r[2], r[3]))
            print(u"    주장 : %s" % (r[4],))
            if r[5]:
                print(u"    tcx  : %s" % (r[5],))
            if r[6]:
                print(u"    사유 : %s" % (r[6],))


# ─────────────────────────────── 산문(prose) 안의 `타입+0xNNN` 스캔 ──
# 명세 본문·REPORT 문서는 구조화된 표가 아니라 산문에 오프셋을 적는다.
# 여기서 나오는 주장이 손으로 쓴 것이라 오류 확률이 가장 높다.
PROSE = re.compile(r"([A-Za-z_][A-Za-z0-9_]{2,40})\s*\+\s*(0x[0-9a-fA-F]{1,6})")
# 산문 별칭(짧은 이름) — 근거를 주석으로 남긴다
PROSE_ALIAS = {
    "entity": "game_core::Entity", "champ": "game_core::Entity", "ent": "game_core::Entity",
    "target": "game_core::Entity", "effect": "game_core::Effect",
    "cfg": "game_core::GameSetting", "setting": "game_core::GameSetting",
    "map": "game_core::MapDef", "player": "game_core::PlayerState",
    "handler": "game_ai::plan_legacy::handler::LegacyPlanHandler",
    "cache": "game_core::AbstractGameWithCache",
}
PROSE_SKIP = set("""vt vtable rsp rbp rax rcx rdx rdi rsi rip base ptr buf self other
game data ctx rec me W w slot spec node buffer stack frame arg args obj addr""".split())


# ★`~~취소선~~` = **이미 정정된 옛 값**이다(§7 정정형 기록 규약).
# 스캔하면 정정을 쓸 때마다 영구 오탐이 하나씩 쌓인다 — 2026-09-11 실측:
# `~~PlayerState+0x2496~~` 가 배치 A·D 감사에서 나란히 「오귀속」으로 잡혔다.
# 길이를 보존해 공백으로 덮는다(문맥 창 text[m.start()-60:] 정렬 유지).
STRIKE = re.compile(r"~~(?!~).+?~~")


def mask_struck(text):
    n = [0]

    def rep(m):
        n[0] += len(PROSE.findall(m.group(0)))
        return u" " * len(m.group(0))

    return STRIKE.sub(rep, text), n[0]


def prose_scan(A, text, srclabel):
    text, nmask = mask_struck(text)
    if nmask:
        A.masked[srclabel] += nmask
    seen = set()
    for m in PROSE.finditer(text):
        nm, hx = m.group(1), m.group(2)
        low = nm.lower()
        if low in PROSE_SKIP:
            continue
        key = (nm, hx)
        if key in seen:
            continue
        seen.add(key)
        tgt = PROSE_ALIAS.get(low, nm)
        st, cands = TD.resolve_name(tgt, strict=True)   # ★부분일치 폴백 금지(오탐 방지)
        if st == "none":
            continue                       # 타입 이름이 아니다(주소·별칭) — 잡음이라 버린다
        tys = [t for _, ts in cands for t in ts]
        gplain = [t for t in tys if (TD.T(t) or {}).get("game") and "<" not in t]
        plain = [t for t in tys if "<" not in t]
        ty = tys[0] if len(tys) == 1 else (gplain[0] if len(gplain) == 1 else (plain[0] if len(plain) == 1 else None))
        if ty is None:
            A.add(u"확인불가", srclabel, nm, hx, u"(산문)", "", u"★모호: 동명 def_path 여럿")
            continue
        o = TD.T(ty)
        off = int(hx, 16)
        sz = o.get("sz")
        ctx = text[max(0, m.start() - 60): m.end() + 90].replace("\n", " ")
        if sz is not None and off >= sz:
            A.add(u"오귀속", srclabel, nm, hx, u"(산문) …%s…" % ctx[:120],
                  u"%s 크기 %dB(0x%x)" % (ty, sz, sz), u"★오프셋이 타입 크기를 넘는다 — 다른 타입이거나 구버전")
            continue
        cands2 = tcx_names_at(ty, off)
        exact = [c for c in cands2 if c[3] == 0]
        got = " | ".join(n for n, t, s, dl in (exact or cands2)[:4])
        if not exact:
            A.add(u"부분일치", srclabel, nm, hx, u"(산문) …%s…" % ctx[:120], got,
                  u"그 오프셋에서 시작하는 필드가 없다(패딩/미관통) — 사람 확인 필요")
            continue
        toks = set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", ctx))
        hit = any(any(s in toks for s in re.split(r"[.\[\]]", n) if s) for n, t, s2, dl in exact)
        A.add(u"OK", srclabel, nm, hx, u"(산문)", got,
              u"" if hit else u"주변 텍스트에 필드명이 없어 이름 대조는 못 함(오프셋만 유효)")


def audit_prose_files(A, paths):
    for p in paths:
        if not os.path.exists(p):
            print(u"[skip] 없음: %s" % p, file=sys.stderr)
            continue
        with io.open(p, encoding="utf-8", errors="replace") as f:
            prose_scan(A, f.read(), os.path.basename(p))


def main():
    A = Audit()
    d = audit_spec_rw(A)
    audit_shared(A, d, "specs20")
    audit_resolved(A)
    if "--prose" in sys.argv:
        extra = [a for a in sys.argv[1:] if not a.startswith("--")]
        audit_prose_files(A, [SPEC, RESOLVED] + extra)
    report(A)


if __name__ == "__main__":
    main()
