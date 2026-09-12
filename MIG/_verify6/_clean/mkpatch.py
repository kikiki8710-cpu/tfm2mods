# -*- coding: utf-8 -*-
u"""mkpatch — 취소선 37곳을 **긍정 서술로 재작성**하는 patch.json 을 만든다.

`old` 는 **정본에서 그대로 잘라낸다**(마커 두 개 사이). 손으로 옮겨 적지 않는다 —
보이지 않는 공백·전각문자 때문에 `old` 가 어긋나는 사고를 원천 차단한다.
정본은 **읽기만** 한다.
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.dirname(os.path.dirname(HERE))
SRC = os.path.join(MIG, "_spec", "specs20.json")

# (v2경로, 시작마커, 끝마커, 새 문장, 예상 등장횟수)
#   시작마커 None = 그 필드 전체
E = []
def ed(p, s, e, new, n=1):
    E.append((p, s, e, new, n))


# ── specs[0] ult ────────────────────────────────────────────────
ed(u"0/logic", u"// ★**caster_r 는 여기 없다**", u"는 구조 오류였다",
   u"// ★**caster_r 는 여기 없다** — abstract_input.rs:191 에서 밖에서 더해진다")
ed(u"0/logic", u"// ★i32 -> usize 부호확장", u"은 부호확장 누락 표기",
   u"// ★`as usize` = i32 -> usize 부호확장(sext)")

# ── specs[1] 타깃 점수 ──────────────────────────────────────────
ed(u"1/logic", u"(~~< 2 임계~~", u"LLVM 접힘)",
   u"(IR 의 `ult 2` 는 `x==0||x==1` 로 접힌 것)")
ed(u"1/logic", u"// ⚠ tag 4(Jungle)이지만", u"의 접힘이다",
   u"// ⚠ tag 4(Jungle) 의 camp_type.0 은 팀 인덱스라 {0,1} 뿐이다. "
   u"소스는 `is_jungle(0)||is_jungle(1)`(entity.rs:1377 동등 비교)이고 "
   u"IR 의 `ult 2` 는 그 둘이 접힌 것이다")
ed(u"1/reads/6/note", u"이고 ~~`< 2` 요구~~", u"는 LLVM 접힘이다",
   u"이며, IR 의 `ult 2` 는 그 동등 비교 둘이 LLVM 에서 접힌 형태다")
ed(u"1/new_knobs/0/effect", u"WARN ~~「camp_type.__0", u"「죽은 가드」도 아니다",
   u"`camp_type.__0` 은 팀 인덱스(0/1)이고, `EntityType::is_jungle(&self, usize)`"
   u"(entity.rs:1377)는 **인자와의 동등 비교**라 상수 2 는 소스에 없다")

# ── specs[2] AttackNexusPlan ────────────────────────────────────
ed(u"2/logic", u"조건은 부정형이 아니다", u"오기를 만들었다)",
   u"조건은 `if has_enemy_twin_tower` **긍정형**이다\n"
   u"// (근거 = 줄길이 ±0 4줄 + IR m12.ll:34955~34971 분기방향)")
ed(u"2/constants/4/meaning", u"★줄 정정", u"L50 = 58자",
   u"★소스 줄 = **L50**(L46 은 빈 줄 1자다). L50 = 58자")

# ── specs[3] defensive_crisis ───────────────────────────────────
ed(u"3/logic", u"// ★줄 정정 : ~~L21~~", u"`let` 슬롯뿐",
   u"// ★L21 은 `let` 슬롯뿐이고 이 계산은 **L22** 다")
ed(u"3/logic", u"★줄 정정 : ~~L42~~", u"슬롯 3칸이",
   u"★L42 에는 명령이 0개다. 슬롯 3칸이")

# ── specs[4] ───────────────────────────────────────────────────
ed(u"4/new_knobs/0/value", u"(~~팀 표기 없이", u"오라클 실측, )",
   u"(양 팀 좌표 모두 오라클 실측)")

# ── specs[6] v2_response_retreat_stance ────────────────────────
ed(u"6/logic", u"// ★~~collect_in~~", u"줄길이 ±0)",
   u"// ★**2인자 `from_iter_in`** 이다 (심볼 실측 m13.ll:45590 + L26/27/30 줄길이 ±0)")

# ── specs[7] ───────────────────────────────────────────────────
ed(u"7/knobs/3/value", u"— ★**리터럴이 아니다.**", u"**오프셋**(0x12f8)이었다",
   u"— ★**리터럴이 아니라 런타임 값**이다. 10진 4856 = 오프셋 **0x12f8**"
   u"(GameSetting 내 `tick_per_second` 위치)")

# ── specs[8] ───────────────────────────────────────────────────
ed(u"8/reads/17/note", u"(~~first()~~", u"get<usize,usize>`)",
   u"(소스는 `get(0)` 이다 — 인라인 체인이 `slice/mod.rs:572 get<usize,usize>` 이고 "
   u"`fn=first` 프레임은 없다. `first()` 는 `get` 을 부르지 않고 슬라이스 패턴으로 "
   u"구현돼 있어 체인에 나타나지 않는다)")
ed(u"8/knobs/0/note", None, None, u"")
ed(u"8/new_knobs/2/effect", u"★★**정정 —", u"실제는 **", u"★★이 노브는 **")
ed(u"8/new_knobs/2/effect", u"\n⚠**같은 오독이", u"게이트 사각지대).", u"")

# ── specs[9] ───────────────────────────────────────────────────
ed(u"9/knobs/3/effect", u"★**정정", u"**`1286`", u"★**`1286`")
ed(u"9/knobs/6/effect", u"★**정정", u"실제 비교 상수는", u"★비교 상수는")
ed(u"9/new_knobs/3/effect", u"★★**극성 정정", u"09 호출부는",
   u"★★**09 경로에서 이 노브는 표 선택에 관여하지 않는다.** 09 호출부는")
ed(u"9/new_knobs/3/effect", u"\n⚠같은 사실이", u"게이트 G8)", u"")

# ── specs[12] handle_chat ──────────────────────────────────────
ed(u"12/logic", u"// ★시그니처 정정", u"이 통째로 인라인돼",
   u"// ★이 호출이 통째로 인라인돼")

# ── specs[13] target_bush_v30 ──────────────────────────────────
ed(u"13/logic", u"(~~is_top_side 본문식~~", u"원식 = x + y <= height",
   u"(오라클 확증). `is_top_side` 원식 = x + y <= height")
ed(u"13/constants/8/meaning", u"[= **!is_top_side**", u"2026-09-11 배치C]",
   u"[= **!is_top_side**(봇 사이드) 구간이다]")
ed(u"13/constants/12/meaning", u"[= **!is_top_side**", u"2026-09-11 배치C]",
   u"[= **!is_top_side**(봇 사이드) 인 구간]")

# ── specs[14] LineGankerPlan::update ───────────────────────────
ed(u"14/logic", u"// (~~결과 = is_top_side~~", u"x + y <= height)",
   u"// (오라클 확증 — `is_top_side` 원식 = x + y <= height)")
ed(u"14/knobs/3/where", u". ~~결과 = ry < champ.x~~", u"오라클 확정)",
   u". IR 술어 `ry < champ.x` 는 그 **부정**이다(오라클 확정)")
ed(u"14/new_knobs/0/effect", u"★★**판정 반전", u"자연상태",
   u"★★**`TargetMissing` 은 실제로 발화한다.** 자연상태")
ed(u"14/new_knobs/0/effect", u" ⟹ v30/v41 이 호출자별", u"**결론이 과했다.**",
   u" ⟹ v30/v41 은 호출자별 하드와이어지만, 두 값이 일치하는 구간에서는 도착 판정이 성립한다.")

# ── specs[15] single_try_engage ────────────────────────────────
ed(u"15/logic", u"/*+0xf8, 공유", u"정정*/", u"/*+0xf8, `&TeamPlan` 공유 참조*/", 2)
ed(u"15/logic", u" — ~~나머지 타워 슬라이스~~ 아님", u"", u"")
ed(u"15/logic", u"(~~팀당 10개", u"오염값이었다",
   u"(⚠하네스가 `init_tower` 를 재호출하면 타워가 2배로 늘어 "
   u"팀당 10개(6+4, twin 2쌍 좌표 중복)로 관측된다)")
ed(u"15/logic", u"★**struct variant** `{ info: Tower }` —", u"오프셋 +0x128 은 맞다:",
   u"★**struct variant** `{ info: Tower }` 다 — 튜플 variant 가 아니다(rustc 진단). "
   u"오프셋 +0x128 경로:")
ed(u"15/reads/5/note", u" — ~~&mut 로 전달", u"tcx sig)",
   u" (tcx sig) — 피호출자는 이 필드를 변경하지 않는다")
ed(u"15/reads/8/note", u"★**정정", u"오프셋 +0x128 자체는 맞다.",
   u"★`EntityType::Tower` 는 **struct variant `{ info: Tower }`** 다(`dienum`) — "
   u"접근 경로는 `ty.Tower.info`, 오프셋 +0x128.")

# ── specs[16] max_range_nearly_can_use ─────────────────────────
ed(u"16/logic", u"★**메서드 호출 + `.as_ref()` 두 줄 구조**다", u"은 구조 오류 .",
   u"★구조는 **메서드 호출 + `.as_ref()` 두 겹**이다.", 2)
ed(u"16/logic", u" — ~~champ.ty.…~~ 는 IR 표기(MIR entity.rs:1747);",
   u"", u" — IR 은 `champ.ty.*` 로 보이지만 MIR 정본은 "
        u"`Entity::attack_cooldown(&self)`(entity.rs:1747);")
ed(u"16/logic", u"// ~~> 2~~ 는 IR 접힘 표기(MIR 정본 = Ge(level, 3))", u"",
   u"// (MIR 정본 = Ge(level, 3); IR 은 `> 2` 로 접힌다)")
ed(u"16/logic", u"// ~~> 4~~ 는 IR 접힘 표기(MIR 정본 = Ge(level, 5))", u"",
   u"// (MIR 정본 = Ge(level, 5); IR 은 `> 4` 로 접힌다)")
ed(u"16/new_knobs/1/effect", u" — ~~2개(Taunt·Animation)만 제외~~ 는 누락이었다", u"", u"")

# ── specs[17] DeathMatchBattle::new ────────────────────────────
ed(u"17/logic", u"// ★정정 : `RawVec::MIN_NON_ZERO_CAP`", u"(Vec::push 인라인)",
   u"// ★`RawVec::MIN_NON_ZERO_CAP`, `size_of::<Chat>()`=24 ⟹ 첫 push 에서 "
   u"cap 이 **4**가 된다(실측 `vec_push1 w0=4 cap_api=4`, Vec::push 인라인)")
ed(u"17/logic", u"// ★~~8필드~~ 정정 :", u"80B 로 딱 맞는다",
   u"// ★8×8B + `idle_prev_pos` **16B** = 80B 로 딱 맞는다")
ed(u"17/writes/14/note", u". ~~usize::MAX~~", u"2026-09-11 검증배치 D)",
   u", 타입은 `i64`)")
ed(u"17/writes/35/note", u" — ~~Option<Option<MainObjective>>~~", u"실제로 틀린다**.",
   u"(tcx 정본). ⚠**이중 Option 으로 읽으면 니치가 한 겹 어긋나 재구현이 틀린다**.")

# ── specs[18] v3_epicops_buff_window ───────────────────────────
ed(u"18/logic", u"// i8 반환. ~~(team, data.cache, plan)~~", u"tcx sig =",
   u"// i8 반환. IR 인자 `(team, data.cache, plan)` 은 **ArgumentPromotion 아티팩트**라 "
   u"소스 인자와 다르다. tcx sig =")
ed(u"18/logic", u"/* ★~~/*p5=*/true~~", u"tcx sig =",
   u"/* ★IR 의 5번째 인자 i1 은 1B 열거형의 ABI 표현이다(불리언 아님). tcx sig =")
ed(u"18/logic", u" (~~682~~ 는 core/src/option.rs 줄번호 혼입, 2026-09-11 배치D)", u"",
   u" (IR 에 섞여 보이는 682 는 `core/src/option.rs` 줄번호다)")
ed(u"18/constants/5/meaning", u"선택 WARN 줄번호 정정", u"공통 출구)다",
   u"선택. 줄번호 = is_none 판정 **epic.rs:673**, Chat 구성 **674/676**. "
   u"⚠인라인 체인에 섞여 보이는 682 는 **`core/src/option.rs`** 줄번호이고, "
   u"epic.rs:682 자체는 함수 꼬리(lifetime.end + 공통 출구)다")
ed(u"18/constants/6/meaning", u"선택 WARN 줄번호 정정", u"공통 출구)다",
   u"선택. 줄번호는 is_none 판정 **epic.rs:673** 이고 Chat 구성은 **674/676**. "
   u"⚠인라인 체인에 보이는 682 는 **`core/src/option.rs`** 줄번호이며, "
   u"epic.rs:682 자체는 함수 꼬리(lifetime.end + 공통 출구)다")
ed(u"18/knobs/1/value", u" ※~~true~~", u"이 행에는 남아 있었다",
   u" ※5번째 인자는 `WavePriorityObject`(1B C-enum, 0=Morgard/1=Serpen)이고 "
   u"IR 의 i1 은 그 ABI 표현이다")
ed(u"18/knobs/5/where", u". ~~epic.rs:682~~", u"2026-09-11 검증배치 D)",
   u". ⚠인라인 체인에 보이는 682 는 core/src/option.rs 줄번호다")

# ── specs[19] best_jungle_goal ─────────────────────────────────
ed(u"19/logic", u"// ~~※ Morgard(4)/Serpen(5)", u"**동작이 달라진다**.",
   u"// ※ 후보 배열에는 Morgard(4)/Serpen(5)가 없다. 다만 **반환값이 일반 캠프로 "
   u"한정되지는 않는다** — 824~829 폴백이 `now_camp` 를 **그대로 돌려주므로** "
   u"Morgard/Serpen 도 나올 수 있다 (실측 `champNone_nowcamp Some(Morgard) "
   u"game=Morgard`). ⚠재구현에서 반환값을 일반 캠프로 필터링하면 **동작이 달라진다**.")


# ── open/notes(v2 `unknown[]`) — cleanspec 이 스캔하지 않는 본문 ──
ed(u"9/unknown/0", u"WARN ~~「유일 호출처", u"누출의 재발).",
   u"호출부는 **8곳**이고 **리터럴 0 은 1곳뿐**(m15.ll:33271 should_disengage_object_hunt), "
   u"나머지 7곳(plan_legacy::handler)은 **런타임 version SSA 값**을 넘긴다. 그럼에도 "
   u"**피호출자 2단이 모두 `i64 poison`** 이라 **이 체인 전체에서 죽은 인자**다"
   u"(오라클 version 12종 diff=0) ⟹ 같은 스펙 `signature.params[0].note`(ev2)와 일치한다.")
ed(u"13/unknown/1", u"~~map_regions::is_top_side 의 극성", u"검증배치 C). ",
   u"map_regions::is_top_side 의 극성 = **확정**. ")
ed(u"13/unknown/1", u"범위정정: 이건 「재료 부재」가", u"두 경로가 열려 있었다",
   u"확인 경로 = `tcxq grep game_core is_top_side` → `vis=pub, mir=1` ⟹ 오라클·MIR 둘 다 열려 있다")
ed(u"14/unknown/2", u"~~ganker.rs:279 소스 조건의 극성", u"검증배치 C). ",
   u"ganker.rs:279 소스 조건의 극성 = **확정**. ")
ed(u"14/unknown/2", u" ⚠판정 어휘 범위정정:", u"「미탐색(오라클·MIR)」이었다**",
   u" ⚠별도 define 이 없는(전량 인라인) 함수도 오라클·MIR 로 확인할 수 있다")
ed(u"14/unknown/4", u"동명 함수라 ~~담당 함수와 무관~~", u"동일 복제본**이다.",
   u"동명 함수이고, ★**문자 단위로 동일한 복제본**이다.")
ed(u"15/unknown/4", u"~~engage_requires_dive", u"bool 만 소비~~ → ", u"")
ed(u"17/unknown/4", u"~~0x17c/0x17d 의 정체 미확인~~", u"검증배치 D): ",
   u"0x17c/0x17d 의 정체 = **확정**: ")


# ── 경로 해석 ──────────────────────────────────────────────────
V3FIELD = {"reads": "mem", "writes": "mem", "constants": "consts",
           "knobs": "knobs", "new_knobs": "knobs"}


class Slot(object):
    u"""dict[key] 든 list[idx] 든 같은 얼굴로 읽고 쓴다."""
    def __init__(self, c, k):
        self.c, self.k = c, k

    def get(self, _=None):
        return self.c[self.k] if isinstance(self.c, list) else self.c.get(self.k)

    def set(self, v):
        self.c[self.k] = v


def locate(D, p):
    u"""'1/reads/6/note' → (Slot, None, v3경로)"""
    t = p.split(u"/")
    i = int(t[0]); sp = D["specs"][i]
    if len(t) == 2:
        return Slot(sp, t[1]), None, u"/specs[%d]/%s" % (i, t[1])
    if len(t) == 3:
        # 'unknown'·'still_unknown' = 문자열 배열 → v3 의 open/notes
        f, j = t[1], int(t[2])
        return Slot(sp[f], j), None, u"/specs[%d]/open[%d]" % (i, j)
    f, j, k = t[1], int(t[2]), t[3]
    row = (sp.get(f) or [])[j]
    if f == "writes":
        j3 = len(sp.get("reads") or []) + j
    elif f == "new_knobs":
        j3 = len(sp.get("knobs") or []) + j
    else:
        j3 = j
    return Slot(row, k), None, u"/specs[%d]/%s[%d]/%s" % (i, V3FIELD[f], j3, k)


def blob(spec):
    out = []
    def w(o):
        if isinstance(o, dict):
            [w(v) for v in o.values()]
        elif isinstance(o, list):
            [w(v) for v in o]
        elif isinstance(o, str):
            out.append(o)
    w(spec)
    return out


def main():
    D = json.load(io.open(SRC, encoding="utf-8"))
    errors, bad, note = [], [], []
    for p, s, e, new, n in E:
        slot, _unused, v3 = locate(D, p)
        cur = slot.get()
        if not isinstance(cur, str):
            bad.append((p, u"필드가 문자열이 아니다")); continue
        if s is None:
            old = cur
        else:
            i0 = cur.find(s)
            if i0 < 0:
                bad.append((p, u"시작 마커 못 찾음: %s" % s)); continue
            if e:
                i1 = cur.find(e, i0 + len(s))
                if i1 < 0:
                    bad.append((p, u"끝 마커 못 찾음: %s" % e)); continue
                old = cur[i0:i1 + len(e)]
            else:
                old = cur[i0:i0 + len(s)]
        c = cur.count(old)
        if c != n:
            bad.append((p, u"등장 %d회(예상 %d): %s" % (c, n, old[:50]))); continue
        if u"~~" not in old and new != u"":
            note.append((p, u"⚠old 에 취소선이 없다(동반 정리): %s" % old[:50]))
        # ★`already()` 오탐 방지 — new 의 앞 80자가 이미 그 spec 안에 있으면
        #   applypatch 가 「이미 적용」으로 **조용히 건너뛴다**.
        if new:
            head = new[:80]
            sp = D["specs"][int(p.split(u"/")[0])]
            if any(head in x for x in blob(sp)):
                bad.append((p, u"⚠new[:80] 이 이미 정본에 있다 → already() 오탐"))
                continue
        errors.append({"path": v3, "kind": u"정리", "old": old, "new": new,
                       "evidence": u"취소선 제거·근거 보존",
                       "behavior_change": False, "found_by": "reused"})
        # 같은 필드를 여러 번 고칠 때 뒤 항목의 마커가 어긋나지 않도록 반영해 둔다
        slot.set(cur.replace(old, new))

    out = {"round": 6, "batch": "_clean", "errors": errors, "ev_up": []}
    io.open(os.path.join(HERE, "patch.json"), "w", encoding="utf-8").write(
        json.dumps(out, ensure_ascii=False, indent=1))
    print(u"정정 항목 %d개 작성 (대상 %d개)" % (len(errors), len(E)))
    if bad:
        print(u"\n★문제 %d건" % len(bad))
        for p, why in bad:
            print(u"   %-26s %s" % (p, why))
        return 1
    # 남은 취소선 확인
    left = 0
    for i, sp in enumerate(D["specs"]):
        for x in blob(sp):
            left += len(re.findall(u"~~[^~]{1,200}?~~", x))
    print(u"적용 후 남은 취소선 조각 = %d" % left)
    return 0


if __name__ == "__main__":
    sys.exit(main())
