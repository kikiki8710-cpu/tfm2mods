# -*- coding: utf-8 -*-
u"""`_prop.json` → `patch.json` (applypatch 스키마). ev 불변도 같이 검사한다."""
import io, json, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkspec3 as M

P = json.load(io.open(r"C:\tfm2mods\MIG\_verify8\B\_prop.json", encoding="utf-8"))

# 임계↔태그 뒤집힘은 재구현을 틀리게 한다(`end_plan < 4` 류 범위판정으로 쓸 수 있다).
# 부시 ID 행은 `meaning` 이 이미 「이 값을 돌려준다」로 읽혀 비교식으로 재구현될 수 없다 ⟹ false.
BC = {u"태그": True, u"임계": True, u"인덱스": False}

errs, bad = [], 0
for p in P:
    e0, e1 = M.evtier(p["old"]), M.evtier(p["new"])
    if e0 != e1:
        print(u"!! ev 변동 [%02d]c%d %d→%d" % (p["i"], p["j"], e0, e1)); bad += 1
    errs.append({
        "path": "/specs[%d]/consts[%d]/meaning" % (p["i"], p["j"]),
        "kind": p["ekind"],
        "old": p["old"],
        "new": p["new"],
        "evidence": p["why"],
        "behavior_change": (False if p["ekind"] == u"보강" else BC[p["want"]]),
        "found_by": "new",
    })

BRIEF = [
 u"§1 「임계 111행」이 틀렸다 — 정본 `specs20_v3.json` 실측은 **임계 107 · 태그 64 · 센티널 9 · 인덱스 6 = 186** 이다(`Counter(c['kind'])`). 111 은 어느 판에도 없다.",
 u"지시문의 후보 3벌 실측치 「26/40 · 72/186 · 6건」이 전부 실행값과 다르다 — 내가 그대로 돌린 결과는 **B 102/186 · C 68/186(+판정보류 20) · D 52/186(R1 0·R2 12·R3 40)** 이다. 어느 인자로 돌려도 26·40·72·6 은 안 나온다.",
 u"★지시문의 IR 오프코드 매핑 `phi·select·store`→태그 가 **틀렸다**. `store`/`phi`/`select` 는 「값을 산출한다」만 말할 뿐 **태그와 산출값을 가르지 못한다** — 가르려면 목적지 슬롯이 열거형 판별자인지 알아야 하고 그건 IR 오프코드가 아니라 `tcx`/`dienum` 의 영역이다. 나는 이 매핑을 그대로 쓴 초판에서 `15 c0`·`17 c1`·`18 c2~c6`(전부 STORE 로만 관측되는 정상 태그 행)을 오적발했다.",
 u"★지시문의 `gep`→인덱스 매핑도 **틀렸다**. 불투명 포인터 시대의 LLVM 은 구조체 필드 접근을 전부 `getelementptr i8, ptr %p, i64 <바이트>` 로 정규화하므로 GEP 인덱스는 **배열 첨자가 아니라 바이트 오프셋**이고, 값의 종류를 전혀 말해 주지 않는다. 초판이 이 매핑으로 **7건을 오적발**했다(`02 c4` 태그 16 ↔ `m12.ll:34916` 의 구조체 `+0x10` 등). ⟹ GEP 는 기각 근거에서 빼고 구제에만 써야 한다.",
 u"★§5 표의 `errors[].kind` 어휘 3종(`실오류`/`오탐`/`보강`)으로는 이 축의 주된 판정을 낼 수 없다. 이 축의 결론 중 **53행은 「동작은 확정인데 명세 칸에 담을 형식이 없다」 = `표기 불가`**(§5-b 어휘)다 — `계수`·`산출값`·`길이`·`비트마스크` 넷은 현행 4종 어휘에 칸이 없어 `meaning` 을 어떻게 고쳐도 못 옮긴다. 지시문은 「고칠 자리는 `meaning` 이거나 파생 규칙」이라고만 적고 **`meaning` 으로 고칠 수 있는 범위가 현행 4종 안뿐**이라는 한계를 안 적었다.",
 u"§0 신선도 확인은 `FRESH` 였다(착수 시·제출 직전 2회). 다만 `dossierfresh.py` 는 `_gates\\consts_kind\\` 의 후보 3벌을 스탬프에 안 넣는다 — 이 라운드의 실질 입력인데 신선도 밖이다.",
 u"★★`applypatch.py` 의 `already()` 버그 — `new[:80]` **부분문자열**로 「이미 적용」을 판정하므로, **긴 `meaning` 의 꼬리만 고치는 정정은 조용히 버려진다**. 이 패치 초판의 `06 c0`·`07 c1`(이 라운드에서 가장 확실한 2건 = 부정문 자기모순)이 실제로 그렇게 삼켜졌다(`정정 47/47 … (이미 적용 2)`). 이 도구의 존재 이유가 「조용한 no-op 금지」인데 그걸 이 함수가 뚫고 있다 — `already()` 는 앞 80자가 아니라 **전체 문자열 동일성**을 봐야 한다. 나는 머리쪽 문면도 같이 고쳐 우회했지만 근본 수정이 필요하다.",
 u"★**7차 배치C `C_kind_proposal.json` 37행에 대한 부분 반전(= 오류 1건으로 셀다)** — 그 제안은 `/specs[i]/consts[j]/kind` 경로라 **원천 적용 불가**인 채 남아 있었다(같은 라운드의 배치A 가 「파생 필드는 못 쓴다」를 적발했는데 제안 파일은 안 고쳌다). 내가 전수를 IR 로 다시 본 결과 **11행(10 c1 · 11 c4/c6~c14)은 확인**해 `meaning` 패치로 번역했고, **26행(13·14 부시 ID)의 `인덱스` 라벨은 기각**했다. 근거: 그 26행은 `select` 로 산출될 뿐 이 함수 IR 이 그 값으로 첫자를 때리는 자리가 **0건**(GEP/MINMAX/ARITH 없음)이라 `임계`(지지 0) → `인덱스`(지지 0) 는 진전이 아니라 **라벨 교체**다. 실제로 넣고 시뮬레이션했더니 게이트의 강한 적발이 **16 → 28 로 늘었다** ⇒ 내 패치를 내가 반증하고 물렸다. 그 26행의 참된 소속은 `산출값` 이고 그건 현행 4종에 칸이 없다 ⇒ **표기 불가**",
]

out = {"round": 8, "batch": "B", "errors": errs, "ev_up": [], "brief_errors": BRIEF}
json.dump(out, io.open(r"C:\tfm2mods\MIG\_verify8\B\patch.json", "w", encoding="utf-8"),
          ensure_ascii=False, indent=1)
print(u"errors %d · ev 변동 %d · behavior_change true %d"
      % (len(errs), bad, sum(1 for e in errs if e["behavior_change"])))
