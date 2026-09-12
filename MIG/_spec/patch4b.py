# -*- coding: utf-8 -*-
u"""patch4b — **3차 §7 의 ev 상향 권고 8건**을 반영한다. (2026-09-11, 4차 반영분)

3차 배치B 가 8건을 권고했는데 `patch3.py` 가 `logic`/`params`/`history` 만 고쳐 **미반영**으로 남았다
(4차 배치B 가 적발). 8건의 내역:
  ① 08 상수 `15`(임계 900)      ev4 → **ev2**   오라클 13/13
  ② 07 상수 `51`(hp_ratio)      ev4 → **ev2**   오라클 8/8
  ③~⑥ vtable 4행                ev4 → **ev3**   슬롯 = 0x20 + 8×(트레이트 선언 순서), divtable 6/6
  ⑦ 태그 3종(ObjectPhase/MainObjective/JungleType) ev4 → **ev3**  dienum
  ⑧ 05 `end_reason` 코드 체계    ev5 → ev3      ← **이미 `closed[]` 로 내려가 무효**(4차에 해소)

★③~⑦ 은 여기서 손대지 않는다 — `ev` 는 근거 문면에서 파생되므로 `mkspec3.EV` 3등급에
  `dienum`/`distruct`/`divtable` 을 넣는 것으로 끝난다(그게 빠진 것이 원인이었다).
  여기서는 ①②처럼 **문면에 없던 오라클 실측**만 적어 넣는다.
"""
import io, json, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
P = "_spec/specs20.json"
D = json.load(io.open(P, encoding="utf-8"))
S = D["specs"]
n = 0


def bump(i, val, line, extra):
    global n
    for x in S[i]["constants"]:
        if x.get("value") == val and x.get("src_line") == line:
            assert extra not in x["meaning"], u"이미 반영됨: %d/%s" % (i, val)
            x["meaning"] += extra
            n += 1
            print(u"  OK %02d constants %s(rs:%d) ev 상향 근거 추가" % (i, val, line))
            return
    raise AssertionError(u"못 찾음: %d %s rs:%d" % (i, val, line))


bump(8, 15, 194,
     u". ★**오라클 13/13(ev2, 3차 배치B)**: `15 × tick_per_second` 임계가 **정확히 900** 에서 갈린다"
     u"(nrt 900 → false, 901 → true). 같은 실행에서 L164 objective 게이트 · L193 에픽 생존 · "
     u"phase 판정 · version 무영향(9종)까지 동시 확증. "
     u"⚠setup 경로는 `v24_…=true` 가 먼저 발화해 (c)(d) 는 오라클 미도달(범위 명시)")
bump(7, 51, 36,
     u". ★**오라클 8/8(ev2, 3차 배치B)**: 경계가 **정확히 50/51** 이다"
     u"(4차 배치B 재확인 49/50·51/52 24/24). 세 번째 OR 항이 `hp < max && in_heal_area` 인 것도 확증. "
     u"태그 5=Recall · 11=EpicHunt 런타임 확인")

D["meta"]["corrections"].append(
    u"2026-09-11 **3차 §7 ev 상향 8건** 반영(`_spec/patch4b.py` + `mkspec3.EV` 3등급 보강). "
    u"3차 권고가 `patch3.py` 에 안 들어가 4차까지 미반영이었다(4차 배치B 적발). "
    u"③~⑦ 의 원인은 `EV` 3등급 목록에 **`dienum`·`distruct`·`divtable` 이 빠진 것** — "
    u"셋 다 DWARF/PDB 덤프라 tcx 와 같은 등급인데 빠져서 태그·vtable 행이 전부 ev4 로 앉아 있었다. "
    u"⑧(05 end_reason)은 4차에 해소돼 `closed[]` 로 내려가 무효.")
n += 1
io.open(P, "w", encoding="utf-8").write(json.dumps(D, ensure_ascii=False, indent=1))
print(u"\n%d곳 반영 -> %s" % (n, P))
