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
import io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
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


def spec_text(i, live=True):
    src = strip_old_fields(S[i]) if live else S[i]
    t = json.dumps(src, ensure_ascii=False)
    return STRIKE.sub(u" ", t) if live else t


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
            for e in pj.get("errors") or []:
                m = PATH.match(e.get("path") or u"")
                if not m:
                    continue
                i = int(m.group(1))
                seen += 1
                full, liv = spec_text(i, False), spec_text(i, True)
                key = (e.get("new") or u"")[:60]
                if key and key not in full:
                    lost.append((rn, b, e["path"], key))
                old = (e.get("old") or u"")[:60]
                if old and old in liv:
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
                    evback.append((rn, b, u2["path"], got, tgt))

    # ── ★커버리지 — **검사할 게 없으면 초록을 찍지 않는다** ────────────
    #   6차 배치C 적발: `patch.json` 이 `_verify5/D` 하나뿐인데 "유실 0"을 찍었다.
    #   그 배치의 5차 결론 4건 + ev 171행이 증발해 있었는데도 초록이었다.
    #   ⟹ **조용한 no-op 금지**를 내가 써놓고 세 번째로 같은 실수를 했다
    #      (`closelist` needle · `mem.chk` · 여기). 이제 **보고서는 있는데 패치가 없으면 결함**으로 센다.
    gap, pend = [], []
    for rd in rounds:
        rn = rd.replace("_verify", "") or "1"
        if want and rn != want:
            continue
        for b in ("A", "B", "C", "D"):
            bd = os.path.join(HERE, rd, b)
            if not os.path.isdir(bd):
                continue
            has_rep = any(f.lower().endswith(".md") and "report" in f.lower()
                          for f in os.listdir(bd))
            has_pat = os.path.exists(os.path.join(bd, "patch.json"))
            has_app = os.path.exists(os.path.join(bd, "applied.json"))
            if has_rep and not has_pat:
                gap.append((rn, b, u"보고서만 있고 patch.json 없음 — 결론을 기계로 못 본다"))
            elif has_pat and not has_app:
                pend.append((rn, b))

    print(u"=" * 92)
    print(u"전 라운드 회귀 감사 — 대조 %d건" % seen)
    print(u"=" * 92)
    print(u"\n### ★커버리지 — 보고서는 있는데 `patch.json` 이 없는 배치 %d건" % len(gap))
    if gap:
        print(u"   이 배치들의 결론은 **기계로 확인할 수 없다.** 아래 「유실 0」은 그만큼 공허하다.")
        for rn, b, why in gap:
            print(u"   [%s차 %s] %s" % (rn, b, why))
    else:
        print(u"   없음 — 보고한 배치는 전부 기계 판독 패치를 냈다.")
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
