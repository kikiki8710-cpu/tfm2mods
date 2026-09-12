# -*- coding: utf-8 -*-
u"""sig.params.role 기계 대조 시제품 (7차 배치A) — 「안 씀 / 전달만 / readnone」 주장을 IR 로 검증한다.

판정 축 3개:
  A) define 줄의 그 인자 위치에 `readnone` 속성이 붙어 있는가 (LLVM 이 단언한 미사용)
  B) 본문(dbg 줄 제외)에서 `%i` 가 몇 번 등장하는가
  C) 그 등장이 **전부 call/invoke 의 인자 자리**인가 (= 「전달만」)

사용: python -X utf8 paramuse.py <spec인덱스...>
"""
import io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.dirname(os.path.dirname(HERE))
IRDIR = r"C:\tfm2mods\_gaibc"
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))

CLAIM_UNUSED = re.compile(u"전혀 안 씀|전혀 안 쓰|한 번도 참조되지 않|readnone|미사용|직접 안 씀|"
                          u"직접 소비하지 않|안 건드|본문에서 안 씀")
CLAIM_PASS = re.compile(u"전달만|넘김|그대로 전달|넘긴다|전달된다")


def split_params(defline):
    u"""define 헤더의 최상위 괄호 안을 콤마로 쪼갠다(중첩 괄호 무시).

    ⚠반환 타입에 `range(i64 .., ..)` 가 붙을 수 있으므로 **`@심볼` 뒤의 첫 `(`** 부터 센다.
    (초판은 `defline.index("(")` 라 `calculate_jungle_action_score` 에서 인자 2개로 깨졌다)
    """
    at = defline.index("@")
    i = defline.index("(", at)
    depth = 0
    cur = ""
    out = []
    for ch in defline[i:]:
        if ch == "(":
            depth += 1
            if depth == 1:
                continue
        elif ch == ")":
            depth -= 1
            if depth == 0:
                out.append(cur)
                break
        if depth >= 1:
            if ch == "," and depth == 1:
                out.append(cur)
                cur = ""
                continue
            cur += ch
    return [x.strip() for x in out]


for arg in sys.argv[1:]:
    i = int(arg)
    sp = D["specs"][i]
    f, a, b = sp["ir"]["file"], sp["ir"]["frm"], sp["ir"]["to"]
    src = io.open(os.path.join(IRDIR, f), encoding="utf-8", errors="replace").read().split("\n")
    body = src[a - 1:b]
    # define 헤더는 여러 줄일 수 있으나 이 코퍼스는 한 줄이다
    head = body[0]
    ps = split_params(head)
    sret = u"sret" in ps[0] if ps else False
    # 명세가 sret 를 인자표에 세는가 — 첫 행 이름으로 판정
    counts_sret = bool(sp["sig"]["params"]) and u"sret" in (sp["sig"]["params"][0].get("name") or u"")
    shift = 1 if (sret and not counts_sret) else 0
    print(u"\n=== specs[%d] %s  (%s %d~%d) · IR 인자 %d개(sret=%s) / 명세 %d개 · 정렬보정 +%d"
          % (i, sp["name"], f, a, b, len(ps), sret, len(sp["sig"]["params"]), shift))
    if shift:
        print(u"  ★인자표가 sret 를 세지 않는다 — 명세 i=1 이 IR %1 이다(00 은 i=1 이 IR %0). 규약 불일치.")
    for j, p in enumerate(sp["sig"]["params"]):
        k = j + shift
        pa = ps[k] if k < len(ps) else u"(IR 인자 없음)"
        ro = re.sub(r"\s+", " ", p.get("role") or u"")
        ssa = re.search(r"%(\w+)\s*$", pa)
        name = u"%" + ssa.group(1) if ssa else u"?"
        rn = u"readnone" in pa
        ronly = u"readonly" in pa
        uses = 0
        callonly = True
        pat = re.compile(re.escape(name) + r"(?![\w.])")
        for ln in body[1:]:
            s = ln.strip()
            if s.startswith("#dbg_") or s.startswith(";"):
                continue
            n = len(pat.findall(ln))
            if not n:
                continue
            uses += n
            if not re.search(r"\b(tail )?call\b|\binvoke\b", ln):
                callonly = False
        cu = bool(CLAIM_UNUSED.search(ro))
        cp = bool(CLAIM_PASS.search(ro))
        verdict = []
        if cu and not cp:
            verdict.append(u"OK-미사용" if (rn or uses == 0) else u"★불일치: 본문 사용 %d회" % uses)
        elif cu and cp:
            verdict.append(u"OK-전달만" if (uses == 0 or callonly) else u"★불일치: call 밖 사용")
        elif cp:
            verdict.append(u"OK-전달만" if callonly else u"★불일치: call 밖 사용")
        else:
            verdict.append(u"(주장 없음)")
        print(u"  p[%d] %-18s %-4s attrs=%-9s uses=%-3d callonly=%-5s  %s"
              % (j, p.get("name"), name, ("readnone" if rn else ("readonly" if ronly else "-")),
                 uses, callonly, u" ".join(verdict)))
        print(u"        role: %s" % ro[:150])
