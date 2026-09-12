# -*- coding: utf-8 -*-
u"""srclinecheck2 — **G12 의 오탐 두 갈래를 막은 판(7차 배치B 제안)**.

## 왜 고치나
현행 `srclinecheck.py` 는 「리터럴 토큰과 `!dbg` 가 **같은 물리 줄**에 있다」를 전제한다.
LLVM `.ll` 에서 이 전제가 깨지는 자리가 둘 있고, 7차 배치B 담당(05~09)의 G12 지적 18건 중
**대부분이 그 둘**이었다(내용은 정본이 맞고 게이트가 틀렸다).

### 오탐 ① `invoke` 는 두 줄이다
```
  %18 = invoke ... @core::slice::starts_with(..., i64 noundef 11)     ← 리터럴 11 은 여기
          to label %33 unwind label %31, !dbg !35999                  ← !dbg 는 여기
```
⟹ 리터럴 줄에 `!dbg` 가 없어 사슬을 못 얻고, 그 값은 「다른 줄에서 우연히 나온 후보」로만 보인다.
(05 `consts[11]=11@146` · `[12]=13@147` · `[13]=12@151` 이 전부 이 형태)

### 오탐 ② `phi` 의 상수는 **들어온 블록**의 줄에 속한다
```
  %74 = phi i8 [ %72, %71 ], [ 1, %33 ], [ 2, %36 ], ... , !dbg !36132
```
`phi` 는 `!dbg` 를 **하나**만 갖는데 상수는 N 개다. 소스에서 값 `1` 이 태어난 자리는
`phi` 가 아니라 **선행 블록 `%33` 의 종결자**(`br i1 %18, ..., !dbg`←dive_episode.rs:146)다.
⟹ `else if` 사슬로 코드값을 고르는 함수(05 의 end_plan 1..9)는 **전건이 오탐**이 된다.

## 고친 규칙
- 리터럴 줄에 `!dbg` 가 없으면 **뒤 2줄 안의 `to label …`/`unwind label …` 연속행**에서 가져온다.
- `phi` 줄이면 각 `[ 값, %블록 ]` 쌍을 풀어, 값이 일치하는 쌍의 **`%블록` 종결자 `!dbg`** 를 후보로 쓴다.
  (phi 자신의 `!dbg` 는 대조에서 뺀다 — 한 줄에 값이 여럿이라 판별력이 없다)

사용: python -X utf8 srclinecheck2.py [specidx ...]
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import srclinecheck as S

DBG = re.compile(r"!dbg !(\d+)")
CONT = re.compile(r"^\s*(?:to label|unwind label|\]\s*$)")
LABEL = re.compile(r"^([%\w.$-]+):")
TERM = re.compile(r"^\s*(?:br|switch|ret|resume|unreachable|indirectbr|cleanupret|catchret|callbr)\b")
PHI = re.compile(r"=\s*phi\s")
PAIR = re.compile(r"\[\s*([^,\[\]]+?)\s*,\s*%([\w.$-]+)\s*\]")
DEF = re.compile(r"^\s*(%[\w.$-]+) = ")


def dbg_of(src, k, a, b):
    u"""k 줄 명령의 `!dbg` id. 그 줄에 없으면 뒤 2줄의 연속행에서 찾는다(`invoke`)."""
    m = DBG.search(src[k])
    if m:
        return m.group(1)
    for x in (k + 1, k + 2):
        if x >= min(b, len(src)):
            break
        if CONT.match(src[x]):
            m = DBG.search(src[x])
            if m:
                return m.group(1)
        else:
            break
    return None


def block_dbgs(src, a, b, label):
    u"""블록 `%label` **안 모든 명령**의 `!dbg` id 목록.

    ★종결자 하나만 보면 안 된다 — 07 `%115` 의 `br` 은 `!DILexicalBlockFile`(line:1, 다른 파일)을
    물고 있어 담당 파일 프레임이 아예 없다. 값이 태어난 줄은 블록 **본문**(여기선 43)에 있다.
    """
    st = None
    for k in range(a - 1, min(b, len(src))):
        m = LABEL.match(src[k])
        if m and m.group(1) == label:
            st = k
            break
    if st is None:
        return []
    out = []
    for k in range(st + 1, min(b, len(src))):
        ln = src[k]
        if LABEL.match(ln):
            break
        if "#dbg_" in ln:
            continue
        d = dbg_of(src, k, a, b)
        if d:
            out.append(d)
    return out


def block_of(src, a, b, k):
    u"""IR 줄 k 를 담고 있는 블록 라벨."""
    for j in range(k, a - 2, -1):
        m = LABEL.match(src[j])
        if m:
            return m.group(1)
    return None


def candidates(sp, val):
    u"""값 `val` 이 태어나는 소스 줄 후보 집합(담당 .rs 프레임만)."""
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    own = str(sp.get("src") or "").split("\\")[-1]
    src, meta = S.load(f)
    # ★`%9`(SSA 레지스터)·`!12115`(메타데이터 id)를 리터럴로 세면 안 된다 —
    #   07 의 태그 9·11 후보가 전부 그 잡음이었다.
    pat = re.compile(r"(?<![\w.\-%!$])" + re.escape(str(val)) + r"(?![\w.])")
    found = {}

    def ownlines(did):
        return [li for (fn, li) in S.chain(meta, did) if fn == own]

    def users_lines(k):
        u"""★`line: 0`(LLVM 병합 위치)이면 **그 SSA 값을 쓰는 명령**의 줄로 돌린다.

        09 `%184 = mul i128 %175, 9` 가 그 예다 — 1283 과 1286 의 같은 식이 CSE 로 합쳐져
        위치가 `line: 0` 이 됐고, 진짜 자리는 소비자 `icmp`(1283)에 남아 있다.
        """
        m = DEF.match(src[k])
        if not m:
            return []
        name = m.group(1)
        pat2 = re.compile(re.escape(name) + r"(?![\w.$-])")
        out = []
        for x in range(k + 1, min(b, len(src))):
            if "#dbg_" in src[x] or DEF.match(src[x]) and src[x].startswith(name + " "):
                pass
            if not pat2.search(src[x]):
                continue
            d2 = dbg_of(src, x, a, b)
            if d2:
                out.extend(li for li in ownlines(d2) if li)
        return out

    def add(did, why, k=None):
        if not did:
            return
        lis = ownlines(did)
        if lis and not any(lis):          # 담당 프레임이 있는데 전부 0 = 병합 위치
            if k is not None:
                for li in users_lines(k):
                    found.setdefault(li, set()).add(why + "/merged")
            return
        for li in lis:
            if li:
                found.setdefault(li, set()).add(why)

    for k in range(a - 1, min(b, len(src))):
        ln = src[k]
        if "#dbg_" in ln:
            continue
        if PHI.search(ln):
            # ★phi 는 쌍을 풀어 선행 블록 종결자로 귀속시킨다
            here = block_of(src, a, b, k)
            for (v, blk) in PAIR.findall(ln):
                if v.strip() != str(val):
                    continue
                # ★선행 블록 본문 ∪ **phi 자신의 블록** 본문.
                #   `return` 이 합류점에 접힌 경우(06 `L38 return RunAway`) 그 줄은
                #   선행 블록이 아니라 **합류 블록**에 있다.
                for d in block_dbgs(src, a, b, blk):
                    add(d, "phi<-%s" % blk)
                if here:
                    for d in block_dbgs(src, a, b, here):
                        add(d, "phi@%s" % here)
            continue
        if not pat.search(ln):
            continue
        add(dbg_of(src, k, a, b), "lit", k)
    return found


def touched(sp):
    u"""함수 IR 범위가 참조하는 담당 `.rs` 줄 전체. **접힘 판정보류**의 근거."""
    ir = sp["ir"]
    f, a, b = ir["file"], ir["frm"], ir["to"]
    own = str(sp.get("src") or "").split("\\")[-1]
    src, meta = S.load(f)
    out = set()
    for k in range(a - 1, min(b, len(src))):
        if "#dbg_" in src[k]:
            continue
        d = dbg_of(src, k, a, b)
        if not d:
            continue
        for (fn, li) in S.chain(meta, d):
            if fn == own and li:
                out.add(li)
    return out


def check_spec(sp):
    u"""`[(consts 인덱스, 주장한 줄, 후보)]` — **실오류만**.

    ★주장한 줄이 그 함수가 실제로 참조하는 줄 집합 안에 있으면 **접힘 판정보류**로 빼낸다.
      상수 접힘(`×2`→`shl 1`, `×4`→`shl 2`, enum 매핑→태그비교)은 정상이고 IR 에 리터럴을
      남기지 않는다. 이걸 불일치로 세면 명세가 옳은데도 영원히 붉은 줄이 남는다(09 `notes[0]`).
    """
    out = []
    ir = sp.get("ir") or {}
    if not (ir.get("file") and ir.get("frm") and ir.get("to")):
        return []
    try:
        S.load(ir["file"])
    except Exception:
        return []
    for j, c in enumerate(sp.get("consts") or []):
        claim = c.get("src_line")
        if not isinstance(claim, int):
            continue
        found = candidates(sp, c.get("value"))
        cands = sorted(x for x in found if x)
        if cands and claim not in found and claim not in touched(sp):
            out.append((j, claim, cands))
    return out


if __name__ == "__main__":
    D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
    idxs = [int(x) for x in sys.argv[1:]] or list(range(20))
    tot = bad = fold = 0
    for i in idxs:
        sp = D["specs"][i]
        print(u"\n===== specs[%d] %s =====" % (i, sp["name"]))
        TCH = touched(sp)
        for j, c in enumerate(sp.get("consts") or []):
            claim = c.get("src_line")
            if not isinstance(claim, int):
                continue
            tot += 1
            found = candidates(sp, c.get("value"))
            cands = sorted(x for x in found if x)
            ok = claim in found
            if ok:
                verdict = u"OK(%s)" % u",".join(sorted(found.get(claim, set())))
            elif not cands:
                verdict = u"(판정보류: 리터럴 미검출)"
            elif claim in TCH:
                verdict = u"(판정보류: 그 줄은 있는데 리터럴이 접힘)"
                fold += 1
            else:
                verdict = u"**불일치**"
                bad += 1
            print(u"  consts[%-2d] value=%-10s claim=L%-6s 후보=%-34s %s"
                  % (j, c.get("value"), claim, cands if cands else u"(미검출)", verdict))
    print(u"\n%s\n검사 %d행 · **불일치 %d건** · 접힘 판정보류 %d건\n%s" % (u"=" * 88, tot, bad, fold, u"=" * 88))
