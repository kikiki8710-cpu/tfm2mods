# -*- coding: utf-8 -*-
u"""auditrounds — **지금까지 모든 라운드의 정정이 아직 살아 있는지** 자동 회귀 검사. (2026-09-11 신설)

## 왜 만드나 — "반영했다"는 주장에 검증 수단이 없었다
| 라운드 | 무슨 일 | 누가 잡았나 |
|---|---|---|
| 1차 | 정정 13건 중 10건이 표에 안 들어갔는데 "적용 완료"로 보고 | 나중에 손으로 |
| 3차 | 정정이 **항목 이동만** 되고 본문은 옛 결론 | **4차 배치B 가 손으로** |
| 3차 | §7 ev 상향 8건 전부 미반영 | **4차가 손으로** |
| 4차 | `patch4.py` 가 배치 A 를 통째로 누락 | **5차 직전 내가 손으로** |
| 5차 | `history` 가 `ev 4→2` 지시했는데 표 미반영 | **5차 배치A 가 손으로** |

전부 **사람이 손으로 찾았다.** 그래서 4차 뒤에 `audit4.py`(손으로 쓴 36항목 체크리스트)를 만들었는데,
그건 **그 라운드 전용**이라 라운드마다 새로 써야 하고, 실제로 5차분은 안 써서 또 손검사가 필요했다.

⟹ `patch.json` 계약이 생겼으니 체크리스트를 **손으로 쓸 필요가 없다**:
   적용된 정정은 정의상 `new` 가 정본에 있고 `old` 는 (취소선 밖에) 없어야 한다.
   **모든 라운드의 모든 patch.json 을 그냥 다시 대조하면 된다.**

## 검사 규칙
- `new` 가 없다 → **유실**(반영이 날아갔거나 애초에 안 됨)
- `old` 가 **취소선 밖에** 살아 있다 → **STALE**(옛 값이 되살아났거나 다른 곳에 남음)
  ⚠`history[].was` 와 `closed[].q` 는 **정의상 옛 주장을 담는 자리**라 제외한다.
    5차에 이걸 안 걸러서 오탐 2건이 났다(`was` 필드 + 버그를 설명하는 **인용**).
- `ev_up` 대상 행의 현재 `ev` 가 목표보다 나쁘다 → **ev 되돌아감**

사용:  python -X utf8 auditrounds.py          (전 라운드)
       python -X utf8 auditrounds.py --round 5
"""
import hashlib, io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
V3 = json.load(io.open(os.path.join(HERE, "_spec", "specs20_v3.json"), encoding="utf-8"))
S = V3["specs"]
STRIKE = re.compile(r"~~(?!~).+?~~", re.S)
PATH = re.compile(r"^/specs\[(\d+)\]/([A-Za-z_]+)")


def strip_old_fields(obj):
    u"""`was`/`q` = 옛 주장 보관 필드. 여기 남은 옛 값은 STALE 이 아니다."""
    if isinstance(obj, dict):
        return {k: strip_old_fields(v) for k, v in obj.items() if k not in ("was", "q")}
    if isinstance(obj, list):
        return [strip_old_fields(x) for x in obj]
    return obj


def _flatten(obj, out):
    u"""값 문자열만 **원문 그대로** 모은다.

    ⚠`json.dumps` 결과와 비교하면 안 된다 — 개행이 `\\n`, 따옴표가 `\\"` 로 이스케이프돼
    원문에 있는 문자열이 "없다"고 나온다. 6차 반영 직후 **유실 2 · STALE 12 가 전부 이 오탐**이었다.
    (같은 실수를 오늘 `patch5.py` 의 `find()` 에서도 했다 — `"최근 가시" 창` 이 안 걸렸다.)
    """
    if isinstance(obj, dict):
        for v in obj.values():
            _flatten(v, out)
    elif isinstance(obj, list):
        for v in obj:
            _flatten(v, out)
    elif isinstance(obj, str):
        out.append(obj)


def spec_text(i, live=True):
    src = strip_old_fields(S[i]) if live else S[i]
    parts = []
    _flatten(src, parts)
    sep = chr(10) + chr(32) + chr(10)   # 리터럴 이스케이프 회피(도구 경유시 훼손됨)
    t = sep.join(parts)
    return STRIKE.sub(u" ", t) if live else t


_AP = [None]


def _already(spec, e):
    if _AP[0] is None:
        try:
            import applypatch as AP
            _AP[0] = AP
        except Exception:
            _AP[0] = False
    if not _AP[0]:
        return True          # 못 부르면 유실로 세지 않는다(조용한 오탐 방지)
    try:
        return bool(_AP[0].already(spec, e))
    except Exception:
        return True


_CS = [None]


def norm_cmp(t):
    u"""`cleanspec` 과 **같은 정리**를 적용한 비교용 형태.

    라운드 순서가 `패치 → cleanspec` 이라, 정리 뒤에는 본문에서 출처 표기가 사라진다.
    그런데 `patch.json` 의 `old`/`new` 는 정리 전 문자열이라 **그대로 비교하면 어긋난다.**
    ⟹ 양쪽에 같은 정리를 걸어 비교한다(같은 규칙을 두 번 구현하지 않도록 `cleanspec` 을 가져다 쓴다).
    """
    if _CS[0] is None:
        try:
            import cleanspec as CS
            _CS[0] = CS
        except Exception:
            _CS[0] = False
    if not _CS[0]:
        return t
    try:
        return _CS[0].clean(t, [])
    except Exception:
        return t


def cur_ev(i, field, idx):
    arr = S[i].get(field)
    if not isinstance(arr, list) or idx >= len(arr):
        return None
    return arr[idx].get("ev")


def main():
    want = None
    if "--round" in sys.argv:
        want = sys.argv[sys.argv.index("--round") + 1]
    rounds = sorted(d for d in os.listdir(HERE)
                    if re.fullmatch(r"_verify\d*", d) and os.path.isdir(os.path.join(HERE, d)))
    lost, stale, evback, seen = [], [], [], 0
    later = []

    _INS = {}

    def _shifted(i, field, rn):
        u"""`rn` 차 **이후**에 `specs[i].<field>` 배열에 행이 삽입됐나."""
        if not _INS:
            for rd2 in rounds:
                rn2 = rd2.replace("_verify", "") or "1"
                base2 = os.path.join(HERE, rd2)
                for b2 in (os.listdir(base2) if os.path.isdir(base2) else []):
                    f2 = os.path.join(base2, b2, "patch.json")
                    if not os.path.exists(f2):
                        continue
                    try:
                        pj2 = json.load(io.open(f2, encoding="utf-8"))
                    except Exception:
                        continue
                    for e2 in pj2.get("errors") or []:
                        if e2.get("op") != "insert":
                            continue
                        m2 = re.match(r"^/specs\[(\d+)\]/([A-Za-z_/]+)", e2.get("path") or u"")
                        if m2:
                            _INS.setdefault((int(m2.group(1)),
                                             m2.group(2).split("/")[-1]), set()).add(rn2)
        try:
            cur = int(rn)
        except Exception:
            return False
        for r in _INS.get((i, field), ()):
            try:
                if int(r) >= cur:
                    return True
            except Exception:
                pass
        return False

    _LATER = {}

    def _fixed_later(rn, pth):
        u"""`rn` 차 **이후** 라운드의 어떤 배치가 같은 경로를 고쳤나."""
        if not _LATER:
            for rd2 in rounds:
                rn2 = rd2.replace("_verify", "") or "1"
                for b2 in (os.listdir(os.path.join(HERE, rd2))
                           if os.path.isdir(os.path.join(HERE, rd2)) else []):
                    f2 = os.path.join(HERE, rd2, b2, "patch.json")
                    if not os.path.exists(f2):
                        continue
                    try:
                        pj2 = json.load(io.open(f2, encoding="utf-8"))
                    except Exception:
                        continue
                    for e2 in (pj2.get("errors") or []) + (pj2.get("ev_up") or []):
                        p2 = (e2.get("path") or u"").rsplit("/", 1)
                        for key in ((e2.get("path") or u""), p2[0]):
                            _LATER.setdefault(key, set()).add(rn2)
        try:
            cur = int(rn)
        except Exception:
            return False
        base = (pth or u"").rsplit("/", 1)[0]
        for key in (pth, base):
            for r in _LATER.get(key, ()):
                try:
                    if int(r) > cur:
                        return True
                except Exception:
                    pass
        return False

    for rd in rounds:
        rn = rd.replace("_verify", "") or "1"
        if want and rn != want:
            continue
        for b in ("A", "B", "C", "D"):
            f = os.path.join(HERE, rd, b, "patch.json")
            # ★**반영된 것만** 회귀 검사한다. 아직 안 붙인 패치를 "유실"로 세면
            #   감사가 시끄러워져서 진짜 유실이 묻힌다(6차 반영 직전에 510건이 그렇게 떴다).
            if not os.path.exists(f) or not os.path.exists(os.path.join(HERE, rd, b, "applied.json")):
                continue
            pj = json.load(io.open(f, encoding="utf-8"))
            # ★**도장이 있으면 그걸 본다.** 「그때 무엇이 들어갔나」(patch 문자열)가 아니라
            #   **「지금 그 자리가 무엇인가」**를 비교한다. 뒤에 정리 패스가 같은 필드를 다시 써도
            #   `--restamp` 로 기준선을 다시 잡으면 감사가 계속 유효하다(실측: 18/34 가 그 경우였다).
            aj = {}
            try:
                aj = json.load(io.open(os.path.join(HERE, rd, b, "applied.json"), encoding="utf-8"))
            except Exception:
                pass
            stamps = aj.get("stamps") or {}
            if stamps:
                import applypatch as _AP2
                _D2 = json.load(io.open(os.path.join(HERE, "_spec", "specs20.json"), encoding="utf-8"))
                for pth, want_hash in stamps.items():   # ★(라운드 필터) 를 덮어쓰지 마라 — 실제로 덮어써서 gap 루프가 통째로 건너뛰었다
                    seen += 1
                    cur = _AP2.field_now(_D2, pth)
                    got = hashlib.sha256((cur or u"").encode("utf-8")).hexdigest()[:16]
                    if cur is None:
                        lost.append((rn, b, pth, u"경로가 사라졌다"))
                    elif got != want_hash:
                        # ★★**나중 라운드가 같은 경로를 다시 고쳤으면 STALE 이 아니다.** (8차 정리)
                        #   실측 4건이 전부 그 형태였다: 6차A `knobs[3]/[4]` → 7차A 가 인용에 타입을 보탰고,
                        #   7차B `consts[0]/[1] meaning` → 8차B 가 `kind` 파생을 고치며 다시 썼다.
                        #   **정당한 후속 정정을 결함으로 세면** 이 감사가 영원히 빨간불이 되고,
                        #   빨간불이 상수가 되면 아무도 안 본다(`specgate.note()` 와 같은 이유).
                        # ★★삽입으로 **인덱스가 밀린 것**도 STALE 이 아니다. (11차)
                        #   `_shifted` 를 `evback` 에만 붙여 두었는데, 도장 쪽에도 같은 일이 난다 —
                        #   11차 D 의 `/specs[17]/mem` 삽입이 9차 D 의 `mem[5]` 도장을 밀었다.
                        #   **같은 계산을 두 곳에 다르게 두면 한쪽만 고쳐진다**(오늘 네 번째).
                        m3 = re.match(r"^/specs\[(\d+)\]/([A-Za-z_]+)", pth or u"")
                        if m3 and _shifted(int(m3.group(1)), m3.group(2), rn):
                            later.append((rn, b, pth,
                                          u"그 배열에 이후 라운드가 행을 삽입했다 — 인덱스가 밀렸다"))
                        elif _fixed_later(rn, pth):
                            later.append((rn, b, pth, u"나중 라운드가 같은 칸을 다시 고쳤다 — 정상"))
                        else:
                            stale.append((rn, b, pth, u"도장 불일치 — 그 뒤 누가 바꿨다"))
                continue
            for e in pj.get("errors") or []:
                m = PATH.match(e.get("path") or u"")
                if not m:
                    continue
                i = int(m.group(1))
                seen += 1
                full, liv = spec_text(i, False), spec_text(i, True)
                # ⚠**한계**: `cleanspec` 이 패치 뒤에 돌면 본문 출처 표기가 사라져
                #   `patch.json` 문자열과 몇 글자 어긋난다(STALE 2~3건이 그 잔여).
                #   조각에 `cleanspec.clean` 을 거는 정규화는 **더 나빴다**(문맥 의존 정규식이
                #   60자 조각에선 다르게 잘린다 — 유실 0→3). ⟹ 권위 있는 확인은
                #   **`applypatch.py <N> --dry` 가 「이미 적용」으로 세는지**다. 여기는 보조 신호로 둔다.
                # ★유실 판정은 **`applypatch.already()` 와 같은 규칙**을 쓴다.
                #   따로 구현했더니 `cleanspec` 정리 뒤에 12/34 가 「없다」로 오탐났다.
                #   같은 사실은 한 곳에만 — 이 프로젝트가 6라운드 겪은 그 규칙을 도구에도 적용한다.
                if not _already(S[i], e):
                    lost.append((rn, b, e["path"], (e.get("new") or u"")[:60]))
                # ★**덧붙이기식 정정은 `old` 가 남는 게 정상이다.**
                #   정정 34건 중 13건이 `new` 안에 `old` 를 그대로 품는 형태였다
                #   (원문 뒤에 근거·정정을 덧붙이는 방식). 그걸 STALE 로 세면
                #   진짜 전파 실패가 오탐 속에 묻힌다 — 6차 반영 직후 12건이 전부 이것이었다.
                oldf, newf = (e.get("old") or u""), (e.get("new") or u"")
                old = oldf[:60]
                if old and oldf not in newf and old in liv:
                    stale.append((rn, b, e["path"], old))
            for u2 in pj.get("ev_up") or []:
                m = re.match(r"^/specs\[(\d+)\]/([A-Za-z_]+)\[(\d+)\]", u2.get("path") or u"")
                if not m:
                    continue
                seen += 1
                got = cur_ev(int(m.group(1)), m.group(2), int(m.group(3)))
                tgt = int(u2.get("to", 2))
                # mem 은 ev3 이 상한이므로 목표를 보정해 비교한다(ev_mem 규칙)
                if m.group(2) == "mem" and tgt < 3:
                    tgt = 3
                if got is not None and got > tgt:
                    # ★★**그 배열에 나중에 행이 삽입됐으면 인덱스가 밀렸다** — 판정 보류.
                    #   9차에 `op:insert` 로 `/specs[15]/mem[12]` 를 넣자 5차 도장이
                    #   **다른 행을 가리키게 돼** ev되돌아감 1건이 떴다(내용은 멀쩡했다).
                    #   인덱스 경로는 삽입에 취약하고, 기계는 「그때 무엇을 가리켰나」를 모른다.
                    if _shifted(int(m.group(1)), m.group(2), rn):
                        later.append((rn, b, u2["path"],
                                      u"그 배열에 이후 라운드가 행을 삽입했다 — 인덱스가 밀려 판정 불가"))
                    else:
                        evback.append((rn, b, u2["path"], got, tgt))

    # ── ★커버리지 — **검사할 게 없으면 초록을 찍지 않는다** ────────────
    #   6차 배치C 적발: `patch.json` 이 `_verify5/D` 하나뿐인데 "유실 0"을 찍었다.
    #   그 배치의 5차 결론 4건 + ev 171행이 증발해 있었는데도 초록이었다.
    #   ⟹ **조용한 no-op 금지**를 내가 써놓고 세 번째로 같은 실수를 했다
    #      (`closelist` needle · `mem.chk` · 여기). 이제 **보고서는 있는데 패치가 없으면 결함**으로 센다.
    # ★★단 **「계약 이전」은 결손이 아니다.** (8차 정리)
    #   `patch.json` 계약은 **6차에 생겼다.** 3~5차 배치에 그게 없는 건 결함이 아니라 **사실**이고,
    #   그걸 「★조치 필요」로 찍으면 **영원히 빨간불**이라 이 감사 자체가 무시된다.
    #   ⟹ 고칠 수 있는 것과 구조적으로 그런 것을 **같은 통에 넣지 마라**
    #      (`specgate` 의 `note()` 와 같은 이유 — 0 을 목표로 삼을 수 없는 지표는 죽는다).
    PATCH_CONTRACT_FROM = 6
    gap, pend, legacy = [], [], []
    for rd in rounds:
        rn = rd.replace("_verify", "") or "1"
        if want and rn != want:
            continue
        try:
            pre = int(rn) < PATCH_CONTRACT_FROM
        except Exception:
            pre = False
        for b in ("A", "B", "C", "D"):
            bd = os.path.join(HERE, rd, b)
            if not os.path.isdir(bd):
                continue
            has_rep = any(f.lower().endswith(".md") and "report" in f.lower()
                          for f in os.listdir(bd))
            has_pat = os.path.exists(os.path.join(bd, "patch.json"))
            has_app = os.path.exists(os.path.join(bd, "applied.json"))
            if has_rep and not has_pat:
                (legacy if pre else gap).append(
                    (rn, b, u"보고서만 있고 patch.json 없음 — 결론을 기계로 못 본다"))
            elif has_pat and not has_app:
                pend.append((rn, b))

    print(u"=" * 92)
    print(u"전 라운드 회귀 감사 — 대조 %d건" % seen)
    print(u"=" * 92)
    if legacy:
        print(u"\n### (참고) `patch.json` 계약 **이전**(≤%d차) 배치 %d건 — 결손이 아니다"
              % (PATCH_CONTRACT_FROM - 1, len(legacy)))
        print(u"   그 라운드엔 기계 판독 계약이 없었다. 결론은 **RE 원문**에만 있다"
              u"(`REPORT\\tfm2_ai_adjust\\RE\\`).")
        print(u"   " + u", ".join(u"%s차%s" % (rn, b) for rn, b, _ in legacy))
    print(u"\n### ★커버리지 — 보고서는 있는데 `patch.json` 이 없는 배치 %d건 (계약 이후만)" % len(gap))
    if gap:
        print(u"   이 배치들의 결론은 **기계로 확인할 수 없다.** 아래 「유실 0」은 그만큼 공허하다.")
        for rn, b, why in gap:
            print(u"   [%s차 %s] %s" % (rn, b, why))
    else:
        print(u"   없음 — 보고한 배치는 전부 기계 판독 패치를 냈다.")
    if later:
        print(u"\n### (참고) 나중 라운드가 **같은 칸을 다시 고친 것** %d건 — STALE 이 아니다" % len(later))
        print(u"   정정이 겹쳐 쌓이는 건 정상이다. 이걸 결함으로 세면 감사가 영원히 빨간불이 된다.")
        for rn, b, pth, why in later[:12]:
            print(u"   [%s차 %s] %s" % (rn, b, pth))
    print(u"\n### 반영 대기 — patch.json 은 냈는데 아직 안 붙인 배치 %d건" % len(pend))
    for rn, b in pend:
        print(u"   [%s차 %s] `applypatch.py %s --only %s` 를 돌려라" % (rn, b, rn, b))
    for lab, rows, fmt in ((u"유실(정정이 정본에 없다)", lost, u"   [%s차 %s] %s\n        없음: %s"),
                           (u"STALE(옛 값이 취소선 밖에 살아있다)", stale, u"   [%s차 %s] %s\n        살아있음: %s"),
                           (u"ev 되돌아감", evback, u"   [%s차 %s] %s  현재 ev%s (목표 ev%s)")):
        print(u"\n### %s — %d건" % (lab, len(rows)))
        for r in rows[:30]:
            print(fmt % r)
    bad = len(lost) + len(stale) + len(evback) + len(gap)
    print(u"\n" + u"=" * 92)
    if not seen:
        print(u"⚠대조할 `patch.json` 이 없다 — 배치가 기계 판독 패치를 내지 않았다는 뜻이다.")
        print(u"   (5차까지가 그랬고, 그래서 ev 상향 492행 중 317행이 유실됐다.)")
    print(u"커버리지 결손 %d · 유실 %d · STALE %d · ev되돌아감 %d  → %s"
          % (len(gap), len(lost), len(stale), len(evback),
             u"전건 유지됨" if not bad else u"★조치 필요"))
    print(u"=" * 92)
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
