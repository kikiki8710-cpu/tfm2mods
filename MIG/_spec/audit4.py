# -*- coding: utf-8 -*-
u"""audit4 — **4차 반증검증 결과가 정말 명세에 들어갔는지** 기계로 대조한다. (5차 착수 전 밑준비)

## 왜 필요한가
이 프로젝트의 반복 실패 모드는 **"적용 완료"라고 보고했는데 실제로는 STALE** 이다.
1차 정정 13건 중 10건이 `resolved[]` 로그에만 들어가고 표에는 안 들어갔는데 나는 완료라고 했다.
3차 정정도 `open→closed` **이동만** 되고 본문은 옛 결론 그대로였다(4차 배치B 가 적발).
⟹ 사람이 읽고 "있네" 하는 것으로는 못 잡는다. **배치 보고서의 항목마다 고유 문면을 정해 grep** 한다.

## 검사 방식
항목당 `(라벨, 스펙 index 또는 None=전역, 반드시 있어야 하는 문면들, 절대 없어야 하는 문면들)`.
- `must`  : 하나라도 없으면 **미반영**
- `forbid`: **취소선 밖에** 살아 있으면 **STALE**(옛 값이 정정 안 된 채 남음)
취소선 판정은 `~~...~~` 안쪽인지를 위치로 본다(문자열 포함만 보면 정정형 기록을 오탐한다).
"""
import io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
V3 = json.load(io.open(os.path.join(HERE, "specs20_v3.json"), encoding="utf-8"))
S = V3["specs"]
SH = json.dumps(V3["shared"], ensure_ascii=False)
ALL = json.dumps(V3, ensure_ascii=False)
STRIKE = re.compile(r"~~(?!~).+?~~", re.S)


def blob(i):
    return ALL if i is None else json.dumps(S[i], ensure_ascii=False)


def _strip_was(obj):
    u"""`history[].was` · `closed[].q` 처럼 **정의상 옛 주장을 담는 필드**를 지운다.

    ★초판이 이걸 안 해서 오탐 2건이 났다:
      · `15.history[9].was` = "3차: … 9축 전부와 무관하다" → `was` 는 옛 결론을 적는 자리다.
      · `16.logic` 주의4 = "`champ.ty.skill2_cooldown` 표기 때문에 …" → 버그를 **설명하는 인용**이다.
    ⟹ 「옛 값이 살아 있다」와 「옛 값을 인용해 설명한다」는 문자열로는 같아 보인다.
       전자는 필드 위치로, 후자는 **금지 문면을 코드 형태로 길게 잡아** 가른다.
    """
    if isinstance(obj, dict):
        return {k: _strip_was(v) for k, v in obj.items()
                if k not in ("was", "q")}
    if isinstance(obj, list):
        return [_strip_was(x) for x in obj]
    return obj


def live(i):
    u"""취소선(이미 정정된 옛 값) + `was`/`q`(옛 주장 보관 필드)를 뺀 본문.
    여기 남아 있으면 **진짜로 살아 있는** 것이다."""
    src = V3 if i is None else S[i]
    return STRIKE.sub(u" ", json.dumps(_strip_was(src), ensure_ascii=False))


# ── 4차 배치 A~D 보고서에서 뽑은 검사 항목 ────────────────────────────
# (라벨, idx, must[], forbid[])
CHECKS = [
    # ═══ 배치 A (00~04) — 실오류 0 · 실질 규명 6 · 오분류 11 ═══
    # ⚠초판 감사기가 **배치 A 를 통째로 빼먹었다.** `patch4.py` 에도 배치 A 항목이 없었고
    #   (오라클 함정만 넣었다), 그래서 배치 A 가 지목한 `03 open[0]` 오분류가
    #   5차 직전까지 열려 있었다. **네 배치 중 하나를 통째로 누락하는 것**이
    #   이 프로젝트에서 가장 비싼 실수 유형이라 맨 앞에 둔다.
    (u"A 01 expected_damage_target ev5→ev2(오라클 26/26)", 1,
     [u"ev5→ev2", u"26/26", u"get_damage"], []),
    (u"A 01 반환 = HP축 절대 피해량(경감식)", 1,
     [u"umax(raw_phys*100/(def_eff+100),1)", u"g06.ll:52355"], []),
    (u"A 01 이펙트 종류별 차등의 소재", 1,
     [u"Effect+0x2c attack_type"], []),
    (u"A 04 get_game_mode 구현 4개 · Game 만 Moba", 4,
     [u"as_moba"], []),
    (u"A 04 핸들러 정체 = TeamPlan(1064B)", 4,
     [u"1064", u"0x41f"], []),
    (u"A→03 오분류(답이 shared 에 있음) 닫힘", 3,
     [u"blackboard_인덱스_의미"], [u"self 가 '적팀이 보는 시야판'인지"]),

    # ═══ 배치 B — 실오류 2 + 근거 오귀속 1 + 새 발견 7 ═══
    (u"B-E1 09 옛 결론(살아있는 버전 게이트) 정정", 9,
     [u"죽은 인자", u"version 12종 diff=0"],
     [u"⟹ 주 경로에서는 **살아 있는 버전 게이트**다. 재구현 시 0 하드코딩 금지"]),
    (u"B-E2 09 에픽버프 노브 분기 극성", 9,
     [u"09 경로는 buff 값과 무관하게 항상 엄격표", u"m15.ll:35472", u"or(ca, buff≠0)"],
     [u"강제로 0 을 읽게 하면 **항상 완화표**"]),
    (u"B-E3 09 closed[0].why 근거 오귀속(closelist needle)", 9,
     [u"version(p1) 이 실제로 무엇을 가르는지"], []),
    (u"B-N1 미니언 함수 전문(minion_wave_risk.rs:130~231)", 9,
     [u"minion_wave_risk.rs:130~231", u"saturating_mul(2)", u"Entity+0x11a"], []),
    (u"B-N2 미니언 생성 우회(팀당 9마리)", None,
     [u"minion_wave_setting", u"run_tick` 600", u"팀당 9마리"], []),
    (u"B-N3 07 Hide 경로 transmute 개방", 7,
     [u"transmute", u"HideSubPlan"], []),
    (u"B-N4 09 독립 재구현 1800/1800", 9, [u"1800/1800"], []),
    (u"B-N6 05 오라클 범위 정정(plan_v50_dive_episodes)", 5,
     [u"plan_v50_dive_episodes"], []),
    (u"B-N7 shared 좌우 정정(buff 가 좌항)", 9, [u"좌우가 반대"], []),

    # ═══ 배치 C — 실오류 0 · 종결 4 ═══
    (u"C 14 region_point 산출식 종결(27칸)", 14,
     [u"0x2218", u"blue_dist_one_count", u"g15.ll:103609", u"27칸"], []),
    (u"C 14 setup_limit/wait_limit 소비처 = is_end", 14,
     [u"m08.ll:94519", u"36/36"], []),
    (u"C 13 target_bush 만 blackboard", 13,
     [u"m10.ll:12124", u"blackboard 를 보지 않는다"], []),
    (u"C 10 divtable 런타임 대조 4/4", 10,
     [u"슬롯 인덱스는 impl 무관"], []),
    (u"C→17 vtable 구현체 우려 해소", 17, [u"impl 무관"], []),

    # ═══ 배치 D — 실오류 6 + 판정반전 3 + 신규확정 3 ═══
    (u"D-E1 16 이펙트 슬롯 = 메서드+as_ref 두 줄", 16,
     [u"champ.attack_effect().as_ref()", u"champ.skill_effect().as_ref()",
      u"champ.skill2_effect().as_ref()", u"champ.ult_effect().as_ref()"],
     [u"if let Some(e) = champ.attack_effect {"]),
    (u"D-E2 16 callees 4개 수복", 16,
     [u"attack_effect", u"skill_effect", u"skill2_cooldown", u"ult_cooldown"],
     [u"champ.ty.skill2_cooldown /*+0xc0*/ > tick", u"champ.ty.ult_cooldown /*+0xc8*/ > tick"]),
    (u"D-E3 15 ty.Tower.info.ty (튜플 오기 정정)", 15,
     [u"ty.Tower.info.ty"], [u'"name": "ty.Tower.0"']),
    (u"D-E4 17 8필드 → 9필드", 17, [u"9필드"], [u"틱·누적 계열 8필드"]),
    (u"D-E5 18 TeamPlan 0xc8/0xd0 행 분리", 18,
     [u'"offset": "0xc8"', u'"offset": "0xd0"'], [u'"offset": "0xc8/0xd0"']),
    (u"D-R1 15 TLS 메모 판정반전(대상 hp 는 판별 축)", 15,
     [u"DieTickCache", u"1999", u"2000", u"m05.ll:44249"], []),
    (u"D-R2 17 base_sub_goal 축 = 적 우물 위험", 17,
     [u"is_ignored_well_enemy", u"m10.ll:29294"], []),
    (u"D-N1 16 battle.rs 33줄 ±0", 16, [u"33줄"], []),
    (u"D-N2 single_tower_dive_is_viable game==mine 12/12", 15,
     [u"12/12", u"single_battle.rs:944"], []),

    # ═══ 3차 §7 ev 상향 8건 (4차에 미반영으로 적발됐던 것) ═══
    (u"ev상향 08 상수 15 오라클 13/13", 8, [u"13/13"], []),
    (u"ev상향 07 상수 51 오라클 8/8", 7, [u"8/8"], []),

    # ═══ shared — 오라클 함정 ═══
    (u"함정 TLS 메모 = 케이스당 프로세스 1개", None,
     [u"thread_local", u"케이스당 프로세스 1개"], []),
    (u"함정 실전 ChampionInfo 액션 파라미터 0", None,
     [u"AttackEffect", u"26/26"], []),
    (u"함정 3차 앵커 프로브 세팅 오염", None, [u"setting_ok"], []),
]

# ── ev 수치 검사(문면이 아니라 값) ───────────────────────────────────
EVCHK = [
    (u"08 consts 15 → ev2", 8, "consts", "value", 15, 2),
    (u"07 consts 51 → ev2", 7, "consts", "value", 51, 2),
]


def main():
    print(u"=" * 96)
    print(u"4차 반영 감사 — 배치 보고서 항목 ↔ specs20_v3.json 기계 대조")
    print(u"=" * 96)
    miss, stale = [], []
    for label, i, must, forbid in CHECKS:
        b = blob(i)
        lv = live(i)
        nm = [m for m in must if m not in b]
        st = [f for f in forbid if f in lv]
        if nm:
            miss.append((label, nm)); mark = u"★미반영"
        elif st:
            stale.append((label, st)); mark = u"★STALE"
        else:
            mark = u"  OK   "
        print(u"%s %s" % (mark, label))
        for m in nm:
            print(u"          없음: %s" % m[:70])
        for f in st:
            print(u"          취소선 밖에 살아있음: %s" % f[:70])

    print(u"\n" + u"-" * 96)
    everr = []
    for label, i, field, key, val, want in EVCHK:
        got = [x.get("ev") for x in S[i][field] if x.get(key) == val]
        ok = got and got[0] == want
        print(u"%s %s  (ev=%s, 기대 %d)" % (u"  OK   " if ok else u"★불일치", label, got, want))
        if not ok:
            everr.append(label)

    print(u"\n" + u"=" * 96)
    print(u"미반영 %d · STALE %d · ev불일치 %d   → %s"
          % (len(miss), len(stale), len(everr),
             u"전건 반영됨" if not (miss or stale or everr) else u"★조치 필요"))
    print(u"=" * 96)
    return 1 if (miss or stale or everr) else 0


if __name__ == "__main__":
    sys.exit(main())
