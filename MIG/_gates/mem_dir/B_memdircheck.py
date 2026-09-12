# -*- coding: utf-8 -*-
u"""memdircheck — **`mem[].dir`(읽기/쓰기 방향) 기계 대조**. 7차 배치B 신설 제안(= 다음 라운드의 `G14`).

## 왜 만드나
`SPEC_RUNBOOK §S5-c` 의 「검사받지 않는 축」 표에서 **가장 큰 축이 `mem.dir` 451행**인데
6라운드 동안 어떤 게이트도 안 봤다. G12(`consts.src_line`)를 붙였을 때 **즉시 56건**이 나왔고,
그 오류들은 1차부터 그대로 있었다. 같은 일이 여기서도 일어날 수 있다.

## 판정 방법 — 오프셋 → 실제 명령
IR 범위에서 `getelementptr inbounds nuw i8, ptr %B, i64 <10진>` 를 모아 **오프셋별 SSA 이름**을
만들고, 그 이름이 `load` 의 피연산자인지 `store` 의 대상인지 본다. 오프셋 `0x0` 은 gep 이 없으므로
**함수 인자 포인터의 직접 load/store** 를 센다.

| 관측 | 뜻 |
|---|---|
| `load` | 그 오프셋에서 **읽음** |
| `store` | 그 오프셋에 **씀** |
| `addr` | 주소만 만들어 **호출 인자로 넘김**(역참조는 피호출자 안) |

- `dir=r` 는 `load` **또는** `addr` 로 충족(05 `plan@0x5e8` 처럼 `get_name(&self.plan)` 인 경우).
- `dir=w` 는 `store` 가 있어야 충족. `addr` 뿐이면 **판정보류**(`&mut` 로 넘겼을 수 있다).
- ⚠**이건 필요조건 검사**다. 같은 오프셋을 다른 구조체가 쓰면 통과할 수 있다(G12 와 같은 한계).
  잡고자 하는 실패 모드는 **방향 뒤집힘(r↔w)** 이고 그건 이 검사로 잡힌다.

사용: python -X utf8 memdircheck.py [specidx ...]
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import srclinecheck as S

GEP = re.compile(r"^\s*(%[\w.$-]+) = getelementptr(?: inbounds)?(?: nuw)? i8, ptr ([%@][\w.$-]+), i64 (\d+)")
# ★배열 인덱싱 gep 은 바이트 gep 결과를 **다시 받는다**(`[5 x ptr], ptr %19, i64 %11`).
#   전파를 안 하면 `player_champion(+0x1e0)`·`fountains(+0x6d70)` 처럼
#   **배열 필드가 통째로 「gep 미검출」로 빠진다**(초판이 그랬다).
#   ⚠타입이 `{ i64, i64, i64, i64 }` 처럼 **쉼표를 품는다** — `[^,]+` 로 자르면 안 된다.
GEP2 = re.compile(r"^\s*(%[\w.$-]+) = getelementptr\b.*?, ptr (%[\w.$-]+),")
LOAD = re.compile(r"^\s*(%[\w.$-]+) = (?:tail )?load [^,]+, ptr (%[\w.$-]+)")
STORE = re.compile(r"^\s*store [^,]+, ptr (%[\w.$-]+)")
ATOM = re.compile(r"^\s*(?:%[\w.$-]+ = )?(?:call|invoke|tail call)")


def ops_by_offset(sp):
    ir = sp["ir"]
    f, a, b = ir["file"], ir["frm"], ir["to"]
    src, _ = S.load(f)
    name2off = {}
    ops = {}

    def mark(off, kind):
        ops.setdefault(off, set()).add(kind)

    lo, hi = a - 1, min(b, len(src))
    for k in range(lo, hi):
        m = GEP.match(src[k])
        if m:
            name2off[m.group(1)] = int(m.group(3))
            continue
        m = GEP2.match(src[k])
        if m and m.group(2) in name2off:
            name2off[m.group(1)] = name2off[m.group(2)]
    # 인자 포인터(%0..%9)의 직접 load/store = 오프셋 0
    for k in range(lo, hi):
        ln = src[k]
        m = LOAD.match(ln)
        if m:
            tgt = m.group(2)
            mark(name2off.get(tgt, 0) if (tgt in name2off or re.match(r"^%\d+$", tgt)) else None, "load") \
                if False else None
            if tgt in name2off:
                mark(name2off[tgt], "load")
            elif re.match(r"^%\d+$", tgt):
                mark(0, "load")
            continue
        m = STORE.match(ln)
        if m:
            tgt = m.group(1)
            if tgt in name2off:
                mark(name2off[tgt], "store")
            elif re.match(r"^%\d+$", tgt):
                mark(0, "store")
            continue
        if ATOM.match(ln):
            for nm in re.findall(r"(%[\w.$-]+)", ln):
                if nm in name2off:
                    mark(name2off[nm], "addr")
    ops.pop(None, None)
    return ops


def check_spec(sp):
    u"""`[(mem 인덱스, 사유, 상세)]` — 방향이 IR 과 어긋난 행만."""
    out = []
    ir = sp.get("ir") or {}
    if not (ir.get("file") and ir.get("frm") and ir.get("to")):
        return []
    try:
        ops = ops_by_offset(sp)
    except Exception as e:
        return []
    for j, m in enumerate(sp.get("mem") or []):
        d = m.get("dir")
        off = str(m.get("offset") or "")
        if d not in ("r", "w") or not off.startswith("0x"):
            continue
        mo = re.match(r"0x([0-9a-fA-F]+)", off)
        if not mo:
            continue
        o = int(mo.group(1), 16)
        seen = ops.get(o)
        if not seen:
            continue                      # 그 오프셋 gep 자체가 안 잡힘 = 판정보류
        if d == "r" and not (seen & {"load", "addr"}):
            out.append((j, u"dir=r 인데 그 오프셋에 load/addr 가 없다(store 만)", u"%s %s" % (off, sorted(seen))))
        if d == "w" and "store" not in seen:
            why = u"dir=w 인데 store 가 없다"
            if "addr" in seen:
                continue                  # &mut 로 넘겼을 수 있다 — 판정보류
            out.append((j, why, u"%s %s" % (off, sorted(seen))))
    return out


if __name__ == "__main__":
    D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
    idxs = [int(x) for x in sys.argv[1:]] or list(range(20))
    tot = bad = hold = 0
    for i in idxs:
        sp = D["specs"][i]
        try:
            ops = ops_by_offset(sp)
        except Exception as e:
            print(u"\n===== specs[%d] %s — IR 로드 실패 %s" % (i, sp["name"], e)); continue
        print(u"\n===== specs[%d] %s =====" % (i, sp["name"]))
        for j, m in enumerate(sp.get("mem") or []):
            d, off = m.get("dir"), str(m.get("offset") or "")
            if d not in ("r", "w") or not off.startswith("0x"):
                continue
            tot += 1
            # ★오프셋 칸에 `0x860[len]` 같은 표기가 섞여 있다 — 앞의 16진수만 뗀다.
            mo = re.match(r"0x([0-9a-fA-F]+)", off)
            if not mo:
                continue
            o = int(mo.group(1), 16)
            seen = sorted(ops.get(o) or [])
            if not seen:
                v = u"(판정보류: 그 오프셋 gep 미검출)"; hold += 1
            elif d == "r":
                v = u"OK" if ({"load", "addr"} & set(seen)) else u"**불일치**"
            else:
                v = u"OK" if "store" in seen else (u"(판정보류: addr 만)" if "addr" in seen else u"**불일치**")
            if u"불일치" in v:
                bad += 1
            if u"판정보류: addr" in v:
                hold += 1
            print(u"  mem[%-2d] %-26s %-8s dir=%s 관측=%-22s %s"
                  % (j, (m.get("base") or u"")[:26], off, d, seen, v))
    print(u"\n%s\n검사 %d행 · **불일치 %d건** · 판정보류 %d건\n%s" % (u"=" * 92, tot, bad, hold, u"=" * 92))
