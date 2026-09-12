# -*- coding: utf-8 -*-
u"""cleanspec — 정본 본문에서 **고고학 지층을 걷어낸다**. (2026-09-11 신설, 유저 지시)

## 왜
유저 지시: *"막 기존거에 취소선 긋고 몇차때 반박당함 이런거 써둔다거나 그런거 없이
그냥 딱 지금 어떻게 분석되어있는가만 적게"*.

실측(6라운드 누적): 본문 174,449자 중 취소선은 927자(0.5%)뿐인데
**「N차 배치X」 출처 표기가 323회**다(`mem` 에만 206회). 진짜 잡음은 취소선이 아니라 **출처 표기**다.

부작용도 이미 실물로 나왔다 — 5차 회귀 감사에서 내 오탐 2건이 전부
`history[].was` 인용과 본문 취소선 때문이었다. 지층이 **기계 검사를 방해**한다.

## 규칙 — 본문에 남는 것과 나가는 것
| 종류 | 예 | 판정 |
|---|---|---|
| **무엇이 참인가** | `cd <= tick 이면 폐기` | 유지 |
| **왜 믿을 수 있나(근거)** | `오라클 122/122` · `m05.ll:44249` · `tcx` · `DWARF` | ★**유지** |
| **누가 언제 찾았나** | `(5차 배치D)` · `4차 배치B 적발` | 제거 → `history[]` |
| **논쟁 이력** | `~~X~~ → Y` · `N차가 뒤집었다` | 제거(`Y` 만 남김) → `history[]` |
| **작성 과정 메모** | `⚠3차 정정이 항목 이동만 되고…` · `내가 …했다` | 제거 |

★**근거를 지우면 안 되는 이유**: `ev` 는 근거 문면에서 **파생**된다(`mkspec3.evtier`).
"오라클 122/122" 를 지우면 그 행이 `ev2 → ev4` 로 **강등**된다.
⟹ 이 도구는 **정리 전후 `ev` 분포가 나빠지면 실패**한다(불변식).

## 이력은 어디로 가나 — 사라지지 않는다
- `history[]`(현재 188항목) = `{was, now}` 쌍. 논쟁·반전의 정본.
- `REPORT\tfm2_ai_adjust\RE\*.md` = 라운드별 원문 전량(§11).
- `MEM\DONE.md` = 재시도 금지 판정 한 줄.
⟹ 본문에서 빼도 **「이미 반박된 걸 다시 결론짓는」 사고는 막힌다.**
   오히려 지금은 같은 사실이 본문·history 양쪽에 있어 **이중화**돼 있다.

## ⚠실행 시점
라운드가 **도는 중에는 쓰지 마라.** 배치의 `patch.json` 이 `old` 문자열로 대상을 찾는데
본문을 바꾸면 전부 어긋난다. **라운드 반영이 끝난 직후**가 유일한 안전 시점이다.

사용:
  python -X utf8 cleanspec.py --dry          변경 표본 + ev 영향만 보고(파일 안 씀)
  python -X utf8 cleanspec.py --dry -v       제거될 문자열 전부 출력
  python -X utf8 cleanspec.py                적용(백업 자동)
"""
import io, json, os, re, shutil, sys
from collections import Counter

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
SRC = os.path.join(HERE, "_spec", "specs20.json")

# ── 제거 대상 ─────────────────────────────────────────────────────────
# ⚠순서가 중요하다. 괄호형을 먼저 지워야 잔여 구두점이 안 남는다.
DROP = [
    # ①괄호 안 출처 표기 — `(5차 배치D)` `(2026-09-11 4차 검증배치 B)` `(4차 배치B 적발)`
    re.compile(u"\\s*[（(][^()（）]{0,40}?[1-9]차[^()（）]{0,40}?[)）]"),
    # ②`— N차 배치X 적발` 꼴 후치 수식
    re.compile(u"\\s*[—-]{1,2}\\s*[1-9]차\\s*배치\\s*[A-D][^.。\n]{0,30}"),
    # ③`★정정(…)`·`★확정(…)` 의 라운드 부분만 — 마커는 남긴다
    re.compile(u"[1-9]차\\s*배치\\s*[A-D]\\s*[:：]?\\s*"),
    re.compile(u"[1-9]차\\s*(?:반증검증|검증배치|검증|정정)\\s*[A-D]?\\s*[:：]?\\s*"),
    # ④작성 과정 메모 — 문장 단위
    re.compile(u"\\s*⚠[^。\n]{0,160}?(?:내가|나는|내 브리핑|항목 이동만|보고했다|적발했다|"
               u"놓쳤다|빠뜨렸다|재발|사각지대)[^。\n]{0,160}?[.。]"),
    # ⑤`~~구~~ → 신` 에서 화살표 앞부분만 — **신값이 바로 뒤에 오는 형태만** 안전하다
    re.compile(u"~~[^~]{1,200}?~~\\s*(?:→|->)\\s*"),
]
# ⛔**취소선 단독 문장은 기계로 못 지운다.**
#   `이고 ~~\`< 2\` 요구~~ 는 LLVM 접힘이다`  → `이고 는 LLVM 접힘이다`(비문)
#   `WARN ~~「… 티어 컷」~~ 은 **거짓**`        → `WARN 은 **거짓**`(무엇이 거짓인지 소실)
#   취소선 안이 그 문장의 **주어**라서 지우면 문장이 무너진다. **다시 써야 한다.**
#   ⟹ 이 도구는 그런 자리를 **목록으로 뽑아 주고** 손대지 않는다(`--strike` 로 확인).
STRIKE_LEFT = re.compile(u"~~[^~]{1,200}?~~")
# 제거 후 정리
# ⚠`\s+([,.。·])` 같은 구두점 정리는 **쓰지 마라.** 첫 판에 넣었다가
#   `@anon...269` → `@anon.269` · `icmp ult ..` → `icmp ult.` 로 **내용을 훼손**했다.
#   IR·심볼 표기에는 `..`·`...` 가 의미를 갖는다. 공백 압축만 한다.
TIDY = [(re.compile(u"[ \t]{2,}"), u" "),
        (re.compile(u"\n{3,}"), u"\n\n")]


def _keep_re():
    u"""★근거 보호 목록은 **`mkspec3.EV` 에서 가져온다.** 복사하지 않는다.

    첫 판은 이 목록을 손으로 복사했다가 `실행 확인`(EV tier2 키)이 빠져서,
    그 표현이 유일한 근거였던 행 하나가 **ev2 → ev4 로 강등**됐다.
    같은 사실을 두 파일에 적으면 어긋난다 — 이 프로젝트가 6라운드 내내 겪은 바로 그 실패다.
    """
    import mkspec3 as M
    keys = [re.escape(k) for _tier, ks in M.EV for k in ks]
    extra = [u"\\d+/\\d+", u"\\.ll:\\d", u"\\.rs:\\d", u"0x[0-9a-f]{2,}",
             u"실행 확인", u"실측", u"진리표"]
    return re.compile(u"(" + u"|".join(keys + extra) + u")")


KEEP = _keep_re()

TARGET_KEYS = ("logic", "one_line")
ROW_FIELDS = ("reads", "writes", "constants", "knobs", "new_knobs")
ROW_KEYS = ("note", "meaning", "effect", "what", "where", "value", "name")


def clean(txt, dropped):
    if not isinstance(txt, str) or not txt:
        return txt
    out = txt
    for pat in DROP:
        def rep(m):
            frag = m.group(0)
            if KEEP.search(frag):       # ★근거가 섞여 있으면 건드리지 않는다
                return frag
            dropped.append(frag.strip())
            return u" "
        out = pat.sub(rep, out)
    for pat, r in TIDY:
        out = pat.sub(r, out)
    return out.strip()


def evdist(path):
    import importlib
    for m in ("mkspec3",):
        if m in sys.modules:
            del sys.modules[m]
    # v3 를 다시 만들지 않고, evtier 만 빌려 본문에서 직접 센다
    import mkspec3 as M
    D = json.load(io.open(path, encoding="utf-8"))
    c = Counter()
    for sp in D["specs"]:
        for x in (sp.get("reads") or []) + (sp.get("writes") or []):
            c[M.ev_mem(x.get("note"))] += 1
        for x in sp.get("constants") or []:
            c[M.evtier(x.get("meaning") or u"")] += 1
        for x in (sp.get("knobs") or []) + (sp.get("new_knobs") or []):
            c[M.evtier(u"%s %s %s" % (x.get("where"), x.get("effect"), x.get("note")))] += 1
    return c


def main():
    dry = "--dry" in sys.argv
    verbose = "-v" in sys.argv
    before = evdist(SRC)
    D = json.load(io.open(SRC, encoding="utf-8"))
    dropped = []
    n = 0
    for sp in D["specs"]:
        for k in TARGET_KEYS:
            if isinstance(sp.get(k), str):
                new = clean(sp[k], dropped)
                if new != sp[k]:
                    sp[k] = new; n += 1
        for f in ROW_FIELDS:
            for x in sp.get(f) or []:
                if not isinstance(x, dict):
                    continue
                for k in ROW_KEYS:
                    if isinstance(x.get(k), str):
                        new = clean(x[k], dropped)
                        if new != x[k]:
                            x[k] = new; n += 1

    tmp = SRC + ".clean.tmp"
    io.open(tmp, "w", encoding="utf-8").write(json.dumps(D, ensure_ascii=False, indent=1))
    after = evdist(tmp)

    print(u"=" * 92)
    print(u"정리 대상 필드 %d곳 · 제거 조각 %d개" % (n, len(dropped)))
    print(u"=" * 92)
    print(u"\n### ev 분포 (불변식: 나빠지면 실패)")
    bad = False
    for e in (1, 2, 3, 4, 5):
        b, a = before.get(e, 0), after.get(e, 0)
        mark = u""
        if e <= 3 and a < b:
            mark = u"  ★강등 %d행" % (b - a); bad = True
        print(u"   ev%d  %4d → %4d%s" % (e, b, a, mark))

    cnt = Counter(dropped)
    print(u"\n### 제거된 조각 상위 %d종" % min(20, len(cnt)))
    for s, c in cnt.most_common(20 if not verbose else 10000):
        print(u"   x%-3d %s" % (c, s[:110]))

    # ── 남은 취소선 — 기계로 못 지운다. 다시 써야 할 자리 목록 ──────────
    left = []
    for i, sp in enumerate(D["specs"]):
        for k in TARGET_KEYS:
            if isinstance(sp.get(k), str) and STRIKE_LEFT.search(sp[k]):
                left.append((i, k, None, sp[k]))
        for f in ROW_FIELDS:
            for j, x in enumerate(sp.get(f) or []):
                if not isinstance(x, dict):
                    continue
                for k in ROW_KEYS:
                    if isinstance(x.get(k), str) and STRIKE_LEFT.search(x[k]):
                        left.append((i, u"%s[%d].%s" % (f, j, k), None, x[k]))
    print(u"\n### ⛔기계로 못 지우는 취소선 — **다시 써야 할 자리 %d곳**" % len(left))
    print(u"   (취소선 안이 문장의 주어라 삭제하면 비문이 되거나 뜻이 사라진다)")
    if "--strike" in sys.argv:
        for i, where, _, txt in left:
            m = STRIKE_LEFT.search(txt)
            s = max(0, m.start() - 60)
            print(u"   specs[%02d].%s\n      …%s…" % (i, where, txt[s:m.end() + 80].replace(u"\n", u" ")))
    else:
        for i, where, _, txt in left[:12]:
            m = STRIKE_LEFT.search(txt)
            print(u"   specs[%02d].%-24s %s" % (i, where, m.group(0)[:70]))
        if len(left) > 12:
            print(u"   … 외 %d곳 (`--strike` 로 전량)" % (len(left) - 12))

    if bad:
        print(u"\n★ev 강등 발생 — **적용하지 않는다.** 근거 보호(KEEP) 목록을 넓혀야 한다.")
        os.remove(tmp)
        return 1
    if dry:
        print(u"\n(--dry: 파일을 쓰지 않았다)")
        os.remove(tmp)
        return 0
    shutil.copy2(SRC, SRC + ".bak.clean.json")
    shutil.move(tmp, SRC)
    print(u"\n-> %s  (백업 = %s.bak.clean.json · 이제 `mkspec3.py` 재생성)" % (SRC, SRC))
    return 0


if __name__ == "__main__":
    sys.exit(main())
