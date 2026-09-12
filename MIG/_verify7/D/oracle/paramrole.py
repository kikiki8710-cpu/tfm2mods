# -*- coding: utf-8 -*-
u"""`sig.params[].role` 기계 대조 — role 산문의 **반증 가능한 주장 2종**만 IR 로 검사한다.

§4-b 「무검사 축」 3개 중 `sig.params.role`(124행). role 은 자유 산문이라 전량 검사는 불가하지만,
다음 두 주장은 IR 로 **결정적으로** 갈린다.

  C1 「본문에서 안 쓴다 / 그대로 전달만 한다」
      ⟹ 그 인자 레지스터가 **gep·load·store 의 포인터 피연산자로 한 번도 안 나온다**.
         한 번이라도 나오면 **거짓**.
  C2 「+0xNNN (만) 읽는다」
      ⟹ role 이 적은 오프셋 집합 ⊆ 그 인자에서 실제로 파생된 gep 오프셋 집합.
         실제 집합에 없는 오프셋을 적었으면 **거짓**(과대주장),
         role 이 「만」이라고 썼는데 실제가 더 많으면 **누락**.

인자 레지스터 찾기: `define` 줄의 인자 순서를 그대로 `%0, %1, …` 로 센다(LLVM 무명 레지스터 규칙).
명세 `params[]` 는 같은 순서로 대응시킨다(개수가 다르면 그 함수는 건너뛴다 — 조용한 오대응 금지).

용법: python -X utf8 paramrole.py [specidx ...]
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
IRDIR = r"C:\tfm2mods\_gaibc"

GEP = re.compile(r"^\s*%(\d+) = getelementptr[^,]*, ptr (%\d+), i64 (-?\d+)")
GEPD = re.compile(r"^\s*%(\d+) = getelementptr[^,]*, ptr (%\d+), i64 %")
LOAD = re.compile(r"^\s*%(\d+) = load [^,]+, ptr (%\d+)")
STORE = re.compile(r"^\s*store [^,]+, ptr (%\d+)")
NOUSE = (u"안 씀", u"안 쓴다", u"안 읽", u"직접 안", u"전달만", u"그대로 전달", u"한 번도",
         u"로만 전달", u"없음 —", u"안 봄")
OFFPAT = re.compile(r"\+0x([0-9a-fA-F]+)")


def body(f, a, b):
    return io.open(os.path.join(IRDIR, f), encoding="utf-8", errors="replace").read().split("\n")[a - 1:b]


def nargs_regs(src):
    u"""define 줄에서 인자 개수를 세고 %0..%{n-1} 을 돌려준다."""
    for ln in src[:6]:
        if ln.lstrip().startswith("define "):
            # ★반환 타입에 `range(i8 0, 6)` 처럼 괄호가 붙는다 — 첫 '(' 를 잡으면 인자를 2개로 센다.
            #   심볼(`@...`) **뒤의** '(' 를 잡아야 한다.
            at = ln.find("@")
            i = ln.find("(", at if at >= 0 else 0)
            if i < 0:
                return []
            depth, j = 0, i
            while j < len(ln):
                if ln[j] == "(":
                    depth += 1
                elif ln[j] == ")":
                    depth -= 1
                    if depth == 0:
                        break
                j += 1
            inner = ln[i + 1:j]
            # 괄호 안 최상위 콤마로 자른다
            parts, d, cur = [], 0, ""
            for ch in inner:
                if ch in "([{":
                    d += 1
                elif ch in ")]}":
                    d -= 1
                if ch == "," and d == 0:
                    parts.append(cur); cur = ""
                else:
                    cur += ch
            if cur.strip():
                parts.append(cur)
            return ["%%%d" % k for k in range(len(parts))]
    return []


def touch(src, regs):
    u"""각 인자에 대해 (포인터로 쓰인 횟수, 그 인자에서 파생된 정적 오프셋 집합, 동적 gep 여부)."""
    gep = {}
    ptruse = {r: 0 for r in regs}
    offs = {r: set() for r in regs}
    dyn = {r: False for r in regs}

    def root(r):
        base, off, d = r, 0, 0
        while base in gep and d < 32:
            nb, no = gep[base]
            if no is None:
                return nb, None
            base, off, d = nb, off + no, d + 1
        return base, off

    for ln in src:
        if "#dbg_" in ln or ln.lstrip().startswith("define "):
            continue
        m = GEP.match(ln)
        if m:
            gep["%" + m.group(1)] = (m.group(2), int(m.group(3)))
            base, off = root(m.group(2))
            if base in ptruse:
                ptruse[base] += 1
                if off is not None:
                    offs[base].add(off + int(m.group(3)))
                else:
                    dyn[base] = True
            continue
        m = GEPD.match(ln)
        if m:
            gep["%" + m.group(1)] = (m.group(2), None)
            base, _ = root(m.group(2))
            if base in ptruse:
                ptruse[base] += 1; dyn[base] = True
            continue
        p = None
        m = LOAD.match(ln)
        if m:
            p = m.group(2)
        else:
            m = STORE.match(ln)
            if m:
                p = m.group(1)
        if p:
            base, off = root(p)
            if base in ptruse:
                ptruse[base] += 1
                if off is not None:
                    offs[base].add(off)
                else:
                    dyn[base] = True
    return ptruse, offs, dyn


def main():
    D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
    idxs = [int(x) for x in sys.argv[1:] if x.isdigit()] or list(range(20))
    n = c1 = c2 = skip = 0
    for i in idxs:
        sp = D["specs"][i]
        ir = sp.get("ir") or {}
        f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
        ps = (sp.get("sig") or {}).get("params") or []
        if not f or not ps:
            continue
        src = body(f, a, b)
        regs = nargs_regs(src)
        # ★sret 보정: 명세에 따라 `(sret ret)` 을 params[0] 로 싣기도 하고 안 싣기도 한다
        #   (15 는 싣고 17 은 안 싣는다 — 명세 schema 불일치, 7차 배치D 적발).
        if len(regs) == len(ps) + 1 and "sret(" in src[0]:
            regs = regs[1:]
        if len(regs) != len(ps):
            print(u"[skip] specs[%d] %s — define 인자 %d개 vs params %d개 (대응 불가)"
                  % (i, sp["name"], len(regs), len(ps))); skip += 1; continue
        ptruse, offs, dyn = touch(src, regs)
        print(u"\n===== specs[%d] %s  (%s:%d~%d) =====" % (i, sp["name"], f, a, b))
        for k, p in enumerate(ps):
            r = regs[k]
            role = p.get("role") or u""
            n += 1
            claims_none = any(w in role for w in NOUSE)
            declared = set(int(x, 16) for x in OFFPAT.findall(role))
            real = offs[r]
            msg = []
            if claims_none and ptruse[r] > 0:
                msg.append(u"**C1 거짓** — '안 씀' 인데 포인터 사용 %d회, 오프셋 %s%s"
                           % (ptruse[r], sorted(hex(x) for x in real), u" +동적" if dyn[r] else u""))
                c1 += 1
            over = declared - real
            if declared and over and not dyn[r]:
                msg.append(u"**C2 과대** — role 이 적은 %s 가 IR 에 없다(실제 %s)"
                           % (sorted(hex(x) for x in over), sorted(hex(x) for x in real)))
                c2 += 1
            print(u"  p%-2d %-8s %-4s ptr사용=%-3d 오프셋=%-42s %s"
                  % (k, (p.get("name") or "")[:8], r, ptruse[r],
                     ",".join(sorted(hex(x) for x in real))[:42] + (u"+dyn" if dyn[r] else ""),
                     u" / ".join(msg)))
    print(u"\n---- 검사 %d건 · C1 거짓 %d · C2 과대 %d · 건너뜀 %d함수" % (n, c1, c2, skip))


main()
