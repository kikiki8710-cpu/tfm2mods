# -*- coding: utf-8 -*-
u"""rolecheck — **`sig.params[].role` 기계 대조**. 7차 배치B 신설 제안(= 다음 라운드의 `G16`).

## 왜 만드나
`SPEC_RUNBOOK §S5-c` 무검사 축 셋째(124행). `role` 은 자유 산문이라 전부는 못 본다 —
**기계로 볼 수 있는 두 조각만** 본다. 그 둘이 실제 오류가 나는 자리다.

### ① 인자 개수 불변식
`IR define 인자 수  ==  spec 인자 수 − (poison/「IR 인자로 안 넘어옴」이라고 적은 수)`
LLVM 이 미사용 인자를 지우면 명세와 IR 의 자리 번호가 **통째로 한 칸씩 밀린다.**
05 는 `_version`/`_tps` 둘이 지워져 `%1`=aborted 인데, 이걸 놓치면 인자 역할이 전부 어긋난다.

### ② 속성 주장 대조
`role` 에 `readnone` / `readonly` / `captures(none)` 라고 적었으면 **`define` 줄의 그 인자에
실제로 그 속성이 있는지** 본다. 「미사용」 주장의 최강 근거가 이 속성이기 때문이다.
⚠ 속성이 **없는데** 「미사용」이라고 적은 것은 이 검사가 잡는다(역은 못 잡는다 —
속성이 없어도 실제로 안 쓸 수 있다).

사용: python -X utf8 rolecheck.py [specidx ...]
"""
import io, json, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import srclinecheck as S

DROPPED = (u"poison", u"인자로 안 넘어옴", u"인자에서 제거")
ATTRS = ("readnone", "readonly", "writeonly", "captures(none)", "sret", "noalias")


def ir_args(sp):
    ir = sp["ir"]
    src, _ = S.load(ir["file"])
    ln = None
    for k in range(ir["frm"] - 1, min(ir["frm"] + 5, len(src))):
        if src[k].lstrip().startswith("define"):
            ln = src[k]
            break
    if ln is None:
        return None
    body = ln[ln.index("(") + 1:ln.rindex(")")]
    out, d, cur = [], 0, ""
    for ch in body:
        if ch in "([{":
            d += 1
        if ch in ")]}":
            d -= 1
        if ch == "," and d == 0:
            out.append(cur.strip()); cur = ""
        else:
            cur += ch
    if cur.strip():
        out.append(cur.strip())
    return out


def check_spec(sp):
    u"""`[(사유, 상세)]`"""
    out = []
    args = ir_args(sp)
    if args is None:
        return []
    ps = (sp.get("sig") or {}).get("params") or []
    # ★「피호출자가 poison 을 받는다」는 **이 함수 인자가 지워졌다는 말이 아니다** —
    #   09 `version` 이 정확히 그 형태라 초판이 오탐을 냈다.
    dropped = [p for p in ps
               if any(t in (p.get("role") or u"") for t in DROPPED)
               and u"피호출자" not in (p.get("role") or u"")]
    want = len(ps) - len(dropped)
    if want != len(args):
        out.append((u"인자 개수 불변식 깨짐",
                    u"IR %d개 vs spec %d개 − 소거주장 %d개 = %d"
                    % (len(args), len(ps), len(dropped), want)))
        return out                        # 자리 매핑이 깨졌으니 속성 대조는 의미 없음
    live = [p for p in ps if p not in dropped]
    for n, (p, a) in enumerate(zip(live, args)):
        role = p.get("role") or u""
        for at in ATTRS:
            if at in role and at not in a:
                out.append((u"role 이 주장한 IR 속성이 define 에 없다",
                            u"p%s(%s) `%s` — IR arg%d = %s"
                            % (p.get("i"), p.get("name"), at, n, a[:90])))
    return out


if __name__ == "__main__":
    D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
    idxs = [int(x) for x in sys.argv[1:]] or list(range(20))
    bad = 0
    for i in idxs:
        sp = D["specs"][i]
        try:
            rows = check_spec(sp)
        except Exception as e:
            print(u"specs[%d] %s — 실패 %s" % (i, sp["name"], e)); continue
        args = ir_args(sp)
        ps = (sp.get("sig") or {}).get("params") or []
        print(u"specs[%-2d] %-38s IR인자 %-2s spec %-2d  %s"
              % (i, sp["name"], len(args) if args is not None else u"?", len(ps),
                 u"OK" if not rows else u"**%d건**" % len(rows)))
        for why, det in rows:
            bad += 1
            print(u"      - %s : %s" % (why, det))
    print(u"\n%s\n불일치 %d건\n%s" % (u"=" * 92, bad, u"=" * 92))
