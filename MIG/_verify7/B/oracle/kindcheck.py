# -*- coding: utf-8 -*-
u"""kindcheck — **`consts[].kind`(상수 종류) 기계 대조**. 7차 배치B 신설 제안(= 다음 라운드의 `G15`).

## 왜 만드나
`SPEC_RUNBOOK §S5-c` 의 무검사 축 3개 중 둘째(186행). 6라운드 동안 아무도 안 봤다.

## 무엇을 보나 — **그 상수를 소비하는 IR 오프코드**
값의 종류는 산문이 아니라 **쓰임새**로 정해진다. IR 범위에서 그 리터럴을 피연산자로 받는
명령의 오프코드를 모아 「관측 종류」를 만들고, 명세의 `kind` 와 대조한다.

| 관측 오프코드 | 관측 종류 |
|---|---|
| `icmp` | **임계**(비교 문턱) |
| `mul` / `shl` / `udiv` / `sdiv` / `lshr` | **계수**(배수·스케일) |
| `getelementptr` 의 인덱스 | **인덱스** |
| `phi` / `store`(태그 슬롯) / `select` 의 결과값 | **태그**(코드값) |
| `call`/`invoke` 인자 | **인자상수**(길이·플래그·enum 값) |

⚠**어휘가 모자란다.** 현행 `kind` 는 `{센티널, 인덱스, 임계, 태그}` 4종뿐인데 실제로는
**계수**(`hp*100`, `tps*2`, `×15`)와 **패턴 길이**(`starts_with("PassiveLine")` 의 11)가 있다.
그래서 그 둘이 전부 `임계` 로 밀려 들어가 있다 — 이게 이 축의 주된 오염이다.
⟹ 제안: 어휘를 `{센티널, 인덱스, 임계, 태그, 계수, 길이}` 6종으로 넓히고 이 검사기를 붙인다.

사용: python -X utf8 kindcheck.py [specidx ...]
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import srclinecheck as S

OPC = [
    (re.compile(r"=\s*icmp\b"), u"임계"),
    (re.compile(r"=\s*(?:mul|shl|udiv|sdiv|lshr|ashr)\b"), u"계수"),
    (re.compile(r"=\s*getelementptr\b"), u"인덱스"),
    (re.compile(r"=\s*phi\b"), u"태그"),
    (re.compile(r"=\s*select\b"), u"태그"),
    (re.compile(r"^\s*store\b"), u"태그"),
    (re.compile(r"=\s*(?:tail )?(?:call|invoke)\b|^\s*(?:tail )?(?:call|invoke)\b"), u"인자상수"),
    (re.compile(r"=\s*(?:add|sub)\b"), u"오프셋가감"),
]


def observed(sp, val):
    ir = sp["ir"]
    f, a, b = ir["file"], ir["frm"], ir["to"]
    src, _ = S.load(f)
    # ★`%9`(SSA)·`!12115`(메타데이터)·`i64`(타입 폭)은 리터럴이 아니다
    pat = re.compile(r"(?<![\w.\-%!$])" + re.escape(str(val)) + r"(?![\w.])")
    out = {}
    for k in range(a - 1, min(b, len(src))):
        ln = src[k]
        if "#dbg_" in ln:
            continue
        body = ln.split(", !")[0]          # 메타데이터 꼬리 제거
        if not pat.search(body):
            continue
        for rx, kind in OPC:
            if rx.search(body):
                out[kind] = out.get(kind, 0) + 1
                break
    return out


if __name__ == "__main__":
    D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
    idxs = [int(x) for x in sys.argv[1:]] or list(range(20))
    tot = mism = 0
    for i in idxs:
        sp = D["specs"][i]
        print(u"\n===== specs[%d] %s =====" % (i, sp["name"]))
        for j, c in enumerate(sp.get("consts") or []):
            kind = c.get("kind") or u"-"
            tot += 1
            obs = observed(sp, c.get("value"))
            top = sorted(obs.items(), key=lambda x: -x[1])
            names = [t[0] for t in top]
            ok = (kind in names) or (kind == u"센티널" and names)
            if not ok:
                mism += 1
            print(u"  consts[%-2d] value=%-12s kind=%-6s 관측=%-40s %s"
                  % (j, c.get("value"), kind,
                     u", ".join(u"%s×%d" % t for t in top) or u"(없음)",
                     u"OK" if ok else u"**어긋남**"))
    print(u"\n%s\n검사 %d행 · kind 와 관측이 어긋난 행 %d건\n%s" % (u"=" * 96, tot, mism, u"=" * 96))
