# -*- coding: utf-8 -*-
u"""정밀 관측기 시제품 — 리터럴이 **피연산자 슬롯**에 있을 때만 센다.

후보 3벌의 공통 결함: 줄 어딘가에 숫자가 보이면 그 줄의 오프코드를 그대로 관측으로 삼는다.
LLVM 텍스트 IR 에서 피연산자는 **반드시 `<타입> <값>`** 형태다(`i64 100`, `double 1.0e+00`).
`align 8`·`!dbg !56`·`%5`·`i8` 는 그 형태가 아니다 ⟹ 타입 접두를 강제하면 잡음이 사라진다.
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
IRDIR = r"C:\tfm2mods\_gaibc"
_C = {}


def load(f):
    if f not in _C:
        _C[f] = io.open(os.path.join(IRDIR, f), encoding="utf-8",
                        errors="replace").read().split("\n")
    return _C[f]


TY = r"(?:i\d+|half|float|double|fp128|ptr)"
ORD_ = ("ult", "ule", "ugt", "uge", "slt", "sle", "sgt", "sge",
        "olt", "ole", "ogt", "oge", "ult", "ule", "ugt", "uge")
ARI_ = ("add", "sub", "mul", "udiv", "sdiv", "urem", "srem", "shl", "lshr",
        "ashr", "and", "or", "xor", "fadd", "fsub", "fmul", "fdiv", "fneg")


def litpat(val):
    v = str(val)
    alts = [re.escape(v)]
    # 부동소수 리터럴은 IR 에서 16진 표기(`0x3FF0…`)/지수표기로 나온다 — 정수만 정확히 센다
    if re.match(r"^-?\d+$", v):
        n = int(v)
        if n < 0:                       # i8 -1 은 IR 에서 -1 로도, 255 로도 안 나온다(-1 로 나옴)
            pass
    return re.compile(r"(?<![\w.])" + TY + r"\s+" + "|".join(alts) + r"(?![\w.])")


def usage(f, a, b, val):
    u"""반환: {클래스: [줄번호…]} · 클래스는 IR 이 그 상수를 **어떻게 소비하는가**"""
    src = load(f)
    v = re.escape(str(val))
    OP = (
        ("SWCASE", re.compile(r"^\s*" + TY + r" " + v + r", label ")),
        ("CMP_ORD", re.compile(r"=\s*(?:fcmp|icmp)\s+(?:samesign\s+)?(?:n\w+\s+)?(?:" +
                               "|".join(ORD_) + r")\s+" + TY + r"\s+(?:[^,]+,\s*" + v +
                               r"(?![\w.])|" + v + r"(?![\w.])\s*,)")),
        ("CMP_EQ", re.compile(r"=\s*(?:fcmp|icmp)\s+(?:samesign\s+)?(?:n\w+\s+)?(?:eq|ne|oeq|one)\s+" +
                              TY + r"\s+(?:[^,]+,\s*" + v + r"(?![\w.])|" + v +
                              r"(?![\w.])\s*,)")),
        ("ARITH", re.compile(r"=\s*(?:nsw |nuw |exact |fast |reassoc |nnan |ninf |nsz |arcp |contract |afn |samesign )*(?:" +
                             "|".join(ARI_) + r")\s+" + TY + r"\s+(?:[^,]+,\s*" + v +
                             r"(?![\w.])|" + v + r"(?![\w.])\s*,)")),
        ("MINMAX", re.compile(r"llvm\.(?:u|s|f)?(?:min|max)\w*\..*?" + TY + r"\s+" + v + r"(?![\w.])")),
        ("GEP", re.compile(r"getelementptr\b.*?,\s*" + TY + r"\s+" + v + r"(?![\w.])")),
        ("STORE", re.compile(r"^\s*store\s+(?:volatile\s+)?" + TY + r"\s+" + v + r"(?![\w.]),")),
        ("SELECT", re.compile(r"=\s*select\b.*?" + TY + r"\s+" + v + r"(?![\w.])")),
        ("PHI", re.compile(r"=\s*phi\b.*?\[\s*" + v + r"(?![\w.])\s*,")),
        ("CALLARG", re.compile(r"(?:tail\s+|musttail\s+)?(?:call|invoke)\b.*?[(,]\s*(?:\w+\s+)*" +
                               TY + r"\s+" + v + r"(?![\w.])")),
        ("RET", re.compile(r"^\s*ret\s+" + TY + r"\s+" + v + r"(?![\w.])")),
        ("INSERT", re.compile(r"=\s*insertvalue\b.*?" + TY + r"\s+" + v + r"(?![\w.])")),
    )
    out = {}
    for k in range(a - 1, min(b, len(src))):
        ln = src[k]
        if "#dbg_" in ln or ln.lstrip().startswith(";"):
            continue
        body = ln.split(", !")[0]
        for name, rx in OP:
            if rx.search(body):
                out.setdefault(name, []).append(k + 1)
    return out


if __name__ == "__main__":
    from collections import Counter
    D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
    cnt = Counter(); byk = {}
    rows = []
    for i in range(20):
        sp = D["specs"][i]
        ir = sp.get("ir") or {}
        f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
        for j, c in enumerate(sp.get("consts") or []):
            u_ = usage(f, a, b, c.get("value"))
            key = tuple(sorted(u_))
            cnt[key] += 1
            byk.setdefault(c.get("kind"), Counter())[key] += 1
            rows.append((i, j, c.get("value"), c.get("kind"), key,
                         {k: v[:3] for k, v in u_.items()}))
    print(u"── 관측 클래스 조합 빈도(상위 30) ──")
    for k, n in cnt.most_common(30):
        print(u"  %-45s %d" % (",".join(k) or u"(관측없음)", n))
    print()
    for kind in (u"임계", u"태그", u"센티널", u"인덱스"):
        print(u"── kind=%s (%d행) ──" % (kind, sum(byk.get(kind, Counter()).values())))
        for k, n in byk.get(kind, Counter()).most_common(14):
            print(u"    %-45s %d" % (",".join(k) or u"(관측없음)", n))
    json.dump([{"i": r[0], "j": r[1], "val": r[2], "kind": r[3],
                "cls": list(r[4]), "lines": r[5]} for r in rows],
              io.open(r"C:\tfm2mods\MIG\_verify8\B\_obs.json", "w", encoding="utf-8"),
              ensure_ascii=False, indent=1)
