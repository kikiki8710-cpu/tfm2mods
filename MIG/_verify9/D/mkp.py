# -*- coding: utf-8 -*-
u"""9차 배치D `patch.json` 생성기 — 손으로 JSON 을 쓰다 오타를 내지 않으려고 기계로 만든다."""
import io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
HERE = os.path.join(MIG, "_verify9", "D")
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))

# ── ① 함수 안 좌표계 철자 통일 (같은 원점 = 같은 문자열)
UNIFY = [
    (11, 14, u"BigPlan(지역변수 plan)",
     u"같은 함수의 mem[15]/[16]/[17] 이 같은 지역변수 `plan` 을 `BigPlan(지역변수 plan)` 으로 "
     u"부른다. `, 384B` 는 좌표계가 아니라 크기 주석이라 `base` 에 들어갈 값이 아니다 — "
     u"같은 명세의 logic 이 `// handler.rs:434 (기존 plan drop 후 384B memcpy)` 로 이미 적고 있다. "
     u"철자가 갈리면 `G14 base_anchors_for` 가 `m2[\"base\"] != base` **문자열 동일**로 형제를 "
     u"모으므로 mem[14] 만 형제 0 이 돼 동정 실패(보류)한다"),
    (12, 22, u"PendingTraceEvent(힙 원소)",
     u"mem[13] 이 같은 구조체를 `PendingTraceEvent(힙 원소)` 로 부른다. tick 도 그 원소의 필드다 — "
     u"m13.ll:29612 `%84 = getelementptr inbounds nuw i8, ptr %12, i64 176` 로 스택 조립본 %12 에 "
     u"쓰고 m13.ll:29652 `memcpy(%98, %12, 184)` 로 그 원소에 통째로 옮긴다(둘은 memcpy 로 "
     u"바이트 동일). 좌표 원점이 같으면 철자도 같아야 한다"),
    (15, 12, u"SinglePlanBattle",
     u"mem[11] 이 같은 값을 `SinglePlanBattle` 로 부른다. `%14`(new_dive sret)와 `%15` 는 "
     u"m13.ll:33697 `memcpy(ptr %15, ptr %14, 144)` 로 이어진 **같은 값의 복사본**이라 좌표계가 "
     u"같다. `로컬 b, 다이브 분기` 는 이미 그 행 note 에 「스택 로컬(%14)에만 쓰고 그 뒤 "
     u"battle(%15)로 memcpy」로 적혀 있다"),
]

# ── ② base 문법 정규화: `<타입>[.<필드경로>][(<한정어>)]` — 타입이 맨 앞, 한정어는 괄호 안
CANON = {
    u"dyn EffectType vtable": u"EffectType::vtable",
    u"AbstractGame vtable": u"AbstractGame::vtable",
    u"dyn AbstractGame vtable": u"AbstractGame::vtable",
    # ★초판은 `SubPlan(sret, LineDefense 페이로드)` 로 고쳤는데, 그러면 같은 sret 버퍼를 가리키는
    #   `02 mem[17] SubPlan(sret 반환버퍼 %0)`(offset 0x0 판별자) 와 **새 철자 분열**이 생긴다
    #   (post.py 로 잡았다 — 규약을 문법만 보고 적용하면 좌표계 동일성을 깬다). 둘 다 `SubPlan(sret)`
    #   로 모은다. `07` 이 이미 그 철자를 쓰고 있고, 「LineDefense 페이로드」는 name 칸이
    #   (`LineDefenseSubPlan.style` 등) 이미 말하고 있다.
    u"SubPlan(sret) LineDefense 페이로드": u"SubPlan(sret)",
    u"SubPlan(sret 반환버퍼 %0)": u"SubPlan(sret)",
    u"bumpalo Vec<&Entity>": u"Vec<&Entity>(bumpalo)",
    u"bumpalo Vec<JungleType>": u"Vec<JungleType>(bumpalo)",
    u"반환 Option<SinglePlanBattle>": u"Option<SinglePlanBattle>(반환)",
}
CANON_EV = (u"규약 D9-OFF ③: `base` 는 `<타입>[.<필드경로>][(<한정어>)]` 꼴이어야 좌표 원점의 "
            u"머리(타입)를 기계로 뽑을 수 있다. 같은 vtable 을 `AbstractGame vtable`(5행)·"
            u"`AbstractGame::vtable`(4행)·`dyn AbstractGame vtable`(6행) 세 철자로 부르고 있어 "
            u"머리 추출이 철자별 특수처리를 요구했다(8차에 검사기 넷이 각자 다르게 파싱한 것과 "
            u"같은 원인). 같은 오프셋(0x28 tick·0x40 get_game_mode·0x1f0 get_entity_by_id)에 "
            u"필드명도 셋 다 일치하므로 좌표계가 같음은 확정")

errors = []
for (i, j, new, ev) in UNIFY:
    old = D["specs"][i]["mem"][j]["base"]
    errors.append({"path": "/specs[%d]/mem[%d]/base" % (i, j), "kind": u"보강",
                   "old": old, "new": new, "evidence": ev,
                   "behavior_change": False, "found_by": "new"})

for i, sp in enumerate(D["specs"]):
    for j, m in enumerate(sp.get("mem") or []):
        b = m.get("base")
        if b in CANON:
            errors.append({"path": "/specs[%d]/mem[%d]/base" % (i, j), "kind": u"보강",
                           "old": b, "new": CANON[b],
                           "evidence": CANON_EV, "behavior_change": False, "found_by": "new"})

# ── ③ mem[14] 에서 뺀 크기 정보를 note 로 옮긴다(정보 손실 방지)
n14 = D["specs"][11]["mem"][14]["note"]
errors.append({"path": "/specs[11]/mem[14]/note", "kind": u"보강", "old": n14,
               "new": n14 + u" · 지역변수 `plan` 은 BigPlan 전체 384B(`base` 에서 옮긴 크기 주석)",
               "evidence": u"base 칸에서 `, 384B` 를 뺀 대신 같은 사실을 note 에 남긴다 — "
                           u"규약은 좌표계와 무관한 정보를 note 로 보낸다",
               "behavior_change": False, "found_by": "new"})

# ── ④ G18 적발(반증 통과분): specs[15] SinglePlanBattle+0x68 chats 행 누락
inserts = [{
    "op": "insert", "path": "/specs[15]/mem", "at": 13,
    "guard": u"chats (Vec<Chat> 24B)", "guard_key": "name",
    "new": {
        "base": u"SinglePlanBattle",
        "offset": "0x68",
        "name": u"chats (Vec<Chat> 24B)",
        "value": u"미채택 경로에서 **drop 만** 한다 — 값을 읽지 않는다",
        "note": (u"G18(9차 배치D) 적발. logic 257행이 `drop(battle.chats /*+0x68 Vec<Chat>*/)` 로 "
                 u"인용하는데 mem 표에 행이 없었다. IR 원문 m13.ll:33724 "
                 u"`%128 = getelementptr inbounds nuw i8, ptr %15, i64 104` → 33726 "
                 u"`invoke void @…Vec<Chat> as Drop>::drop(ptr … dereferenceable(24) %128)` "
                 u"(언와인드 경로는 33732 `RawVec<Chat>::drop`). 채택 경로에서는 33717 "
                 u"`memcpy(ptr %0, ptr %15, 144)` 로 sret 에 그대로 실려 나간다. "
                 u"⚠`dir` 은 `-` 로 둔다 — drop 글루는 읽기도 쓰기도 아니라 G14 의 대상이 아니다"),
        "dir": "-",
        "chk": "OK",
        "ev": 3,
    },
    "found_by": "new",
}]

BRIEF = [
    u"★판정 반전 1 — 지시문 임무① 이 「`0x860[len]` 처럼 좌표계가 섞인 값이 **있어**」라고 현재형으로 "
    u"적었지만, `specs20_v3.json` 의 `mem[].offset` **451행 전량이 이미 순수 16진수**다"
    u"(`^0x[0-9a-fA-F]+$` 위반 0행). 8차 정정이 이미 반영돼 있었다. 남아 있는 좌표계 혼선은 "
    u"`offset` 칸이 아니라 **`base` 칸**이다 — 문법 위반 22행 · 함수 안 철자 분열 5그룹.",

    u"★판정 반전 2 — 지시문이 8차의 「`A=7 vs D=7` 인데 집합이 다른」 원인을 `0x860[len]` 파싱 차이로 "
    u"지목하는데, `memdir.py` docstring §1 자신이 「D 는 예외로 떨궈 보고, A 는 `int(,16)` 실패로 "
    u"버렸다」고 적는다 — **둘 다 그 행을 대상에서 제외**했다. 실제로 갈린 4행"
    u"(A=specs[11] plan+0x5e8·specs[12] GameContext+0x38 / D=specs[0] sret+0x8·specs[12] "
    u"CallHandled+0x38) 중 `0x860[len]` 행(specs[12] mem[13])은 **하나도 없다**. 갈림의 원인은 "
    u"오프셋 표기가 아니라 **base 동정의 부재**였다.",

    u"도시에 §1 「할 일」 1 이 `MIG\\_gates\\mem_offset\\` 에 다른 배치의 검사기 후보가 모여 있다고 "
    u"지시하는데 **그 디렉터리는 없다**(`_gates\\` = `consts_kind`·`mem_dir`·`params_role`·`src_line` "
    u"넷뿐). 8차 `mem_dir` 축용 문단이 축 이름만 바꿔 복사된 것이고, §2 의 「`_gates\\mem_offset\\` 의 "
    u"후보 전부」도 같다. ⟹ 「통합판」은 만들 수 없었고 **새로 설계**했다.",

    u"도시에 §5 규칙이 「새 항목 추가(`append`)는 구현이 없다」고 단정하는데 `applypatch.py:245~271` 에 "
    u"**`op:insert` 가 구현돼 있다**(8차 신설). 지시문 쪽은 맞게 적었다 — 도시에 본문이 낡았다. "
    u"이번 patch 의 유일한 실적발(`15 mem` 삽입)이 그 경로로 나간다.",

    u"도시에 §0 정본 스탬프가 `_spec/specs20.json`(v2) 해시를 v3 와 나란히 싣는데, **v2 에는 `mem` "
    u"행이 0개**다(실측). 이 축의 정본은 v3 뿐이고, v2 해시는 이 배치의 신선도 판단에 아무 정보도 "
    u"주지 않는다.",

    u"도시에 §3 이 「이 축의 전 20함수 행 = **451개**」라고 분모를 하나로 적었는데 이 배치는 **두 축**을 "
    u"맡는다. 451 은 `offset` 표기 축의 분모이고, `logic`↔표 상호참조 축의 분모는 `mem` 행이 아니라 "
    u"**`logic` 의 인용 수**다(실측 P1 오프셋 227 · P2 필드 사슬 297). 분모가 틀리면 커버리지도 틀린다.",
]

out = {"round": 9, "batch": "D", "errors": errors + inserts, "ev_up": [],
       "brief_errors": BRIEF}
io.open(os.path.join(HERE, "patch.json"), "w", encoding="utf-8").write(
    json.dumps(out, ensure_ascii=False, indent=1))
print(u"errors=%d (그중 insert=%d)" % (len(out["errors"]), len(inserts)))
