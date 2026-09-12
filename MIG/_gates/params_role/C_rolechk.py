# -*- coding: utf-8 -*-
u"""`sig.params[].role`(인자 역할) 기계 대조 — **§4-b 무검사 축 124행의 첫 계측기.** (7차 배치C 신설)

## 발상
`role` 은 자유서술이라 통째로는 검사할 수 없다. 하지만 **검사 가능한 주장 두 종류**가 그 안에 있다:

1. **「안 쓴다」류** — `안 씀` / `읽지 않` / `전달만` / `쓰이지 않`.
   ⟹ 그 인자 `%k` 가 IR 범위에서 **load/gep 의 베이스로 한 번도 안 쓰였는지** 본다.
      (호출 인자로만 등장하면 참, `gep %k, N` 이나 `load .., ptr %k` 가 있으면 **거짓**)
2. **오프셋 인용** — `+0x930` / `(0x9c0)` 같은 숫자.
   ⟹ 그 인자에서 출발하는 gep 사슬에 **그 오프셋이 실제로 있는지** 본다.

`i` 는 1-based 이고 IR 인자는 0-based 이므로 **`%(i-1)`** 이 그 인자다.
⚠`sret` 반환 슬롯이 있는 함수는 `%0` 이 반환 슬롯이라 한 칸 밀린다 — `params[0].i` 가 0 이면
그 함수는 sret 형이므로 매핑을 `%i` 로 바꾼다(명세가 `i` 를 그렇게 적어 두었다).

## 한계
- 스칼라 승격된 인자(`i24 MainObjective`, `usize team`)는 gep/load 가 아예 없다 ⟹ 「안 쓴다」
  판정이 **거짓 양성 없이** 참으로 나오지만, 「읽는다」 주장은 검증할 수 없다.
- 콜리 안에서의 사용은 안 보인다(`mem.dir` 검사기와 같은 한계).
"""
import io, json, os, re, sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import srclinecheck as S
import memdir as MD

UNUSED = (u"안 씀", u"안씀", u"안 쓴", u"안 쓰", u"안쓴", u"읽지 않", u"전달만",
          u"쓰이지 않", u"안 읽", u"사용하지 않", u"없다", u"미사용")
OFFPAT = re.compile(r"\+?0x([0-9a-fA-F]{1,5})\b")
# 절 구분 — 한국어 서술이라 어절이 아니라 **구두점**으로 쪼갠다.
CLAUSE = re.compile(u"[.。·]\\s*|(?<=[다움씀])\\s*,\\s*")
# 스칼라 승격(SROA)된 인자 — 소스 구조체 오프셋이 IR 에 gep 로 남지 않는다
SROA = re.compile(u"인자승격|승격|스칼라")


def uses(src, a, b, arg):
    u"""인자 `%k` 가 **메모리 베이스로** 쓰인 줄 목록 / 그 인자에서 나온 오프셋 집합."""
    lo, hi = a - 1, min(b, len(src))
    reads, writes, off = MD.offsets(src, a, b)
    # 인자에서 출발한 SSA 들(오프셋 사슬)을 다시 추적 — 베이스 태그가 필요하다
    owner = {arg: 0}
    lines = []
    for k in range(lo, hi):
        ln = src[k]
        m = MD.GEP.match(ln) or MD.GEPV.match(ln) or MD.GEPANY.match(ln)
        if m and m.group(2) in owner:
            base = owner[m.group(2)]
            n = 0
            mm = MD.GEP.match(ln)
            if mm:
                n = int(mm.group(3))
            owner[m.group(1)] = None if base is None else base + n
            lines.append((k + 1, ln.strip()[:100]))
            continue
        m = MD.LOAD.match(ln)
        if m and m.group(1) in owner:
            lines.append((k + 1, ln.strip()[:100]))
            continue
        m = MD.STORE.match(ln)
        if m and m.group(1) in owner:
            lines.append((k + 1, ln.strip()[:100]))
    return lines, {v for v in owner.values() if v is not None}


def check_spec(sp):
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    if not f or not a or not b:
        return []
    src, _m = S.load(f)
    ps = (sp.get("sig") or {}).get("params") or []
    base0 = 0 if (ps and ps[0].get("i") == 0) else 1     # sret 보정
    out = []
    for p in ps:
        role = p.get("role") or ""
        k = p.get("i")
        if k is None:
            continue
        arg = "%%%d" % (k - base0)
        lines, offs = uses(src, a, b, arg)
        # ★**절 단위로 쪼갠다.** 초판은 role 전체를 한 덩어리로 봐서
        #   「cache(+0x0), context(+0x8) 를 읽음. blackboard(+0x10) **는 안 씀**」 같은 한 줄에서
        #   ①「안 씀」이 있으니 인자 전체가 미사용이라고 읽고 ②부정절의 0x10 을 「있어야 하는데 없다」로
        #   읽어 **한 행에서 오탐 2건**을 냈다(specs[14] i=5). 긍정절과 부정절은 요구가 정반대다.
        if SROA.search(role):
            continue                    # 스칼라 승격 인자 — 소스 오프셋이 IR 에 남지 않는다
        for cl in CLAUSE.split(role):
            if not cl.strip():
                continue
            neg = any(t in cl for t in UNUSED)
            want = {int(x, 16) for x in OFFPAT.findall(cl)}
            if neg and not want:
                # 인자 전체가 미사용이라는 주장
                if lines:
                    out.append((k, p.get("name"),
                                u"「%s」인데 %s 를 베이스로 %d회 접근" % (cl.strip()[:40], arg, len(lines)),
                                lines[:3]))
            elif neg and want:
                hit = sorted(want & offs)
                if hit:
                    out.append((k, p.get("name"),
                                u"「%s」인데 %s 사슬에 그 오프셋이 있다"
                                % (cl.strip()[:40], ["0x%x" % x for x in hit]), []))
            elif want:
                miss = sorted(want - offs)
                if miss:
                    out.append((k, p.get("name"), u"인용 오프셋 %s 가 %s 사슬에 없다 (실제 %s)"
                                % (["0x%x" % x for x in miss], arg,
                                   ["0x%x" % x for x in sorted(offs)][:12]), []))
    return out


if __name__ == "__main__":
    D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
    lo = int(sys.argv[1]) if len(sys.argv) > 1 else 0
    hi = int(sys.argv[2]) if len(sys.argv) > 2 else 20
    tot = n = 0
    for i in range(lo, hi):
        sp = D["specs"][i]
        rows = check_spec(sp)
        n += len((sp.get("sig") or {}).get("params") or [])
        tot += len(rows)
        print(u"specs[%2d] %-46s params %d · 결함후보 %d"
              % (i, sp["name"], len((sp.get("sig") or {}).get("params") or []), len(rows)))
        for (k, nm, why, ex) in rows:
            print(u"    params i=%s %-16s %s" % (k, nm, why))
            for (lnno, txt) in ex:
                print(u"        %s: %s" % (lnno, txt))
    print(u"\n합계: params %d행 · 결함후보 %d" % (n, tot))
