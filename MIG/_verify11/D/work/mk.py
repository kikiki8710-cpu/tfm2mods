# -*- coding: utf-8 -*-
import io, json, os

os.chdir(r"C:\tfm2mods\MIG")
E = []


def err(path, kind, old, new, ev, bc=False, fb="new"):
    E.append({"path": path, "kind": kind, "old": old, "new": new,
              "evidence": ev, "behavior_change": bc, "found_by": fb})


def ins(path, at, guard, row, ev, bc=False, fb="new"):
    E.append({"op": "insert", "path": path, "at": at, "guard": guard,
              "guard_key": "name", "new": row, "kind": "보강",
              "evidence": ev, "behavior_change": bc, "found_by": fb})


# ───────────── 15 ─────────────
err("/specs[15]/open[0]", "실오류",
    "남은 미탐색은 `engage_requires_dive` **하나뿐**이다(범위 명시).",
    "~~남은 미탐색은 `engage_requires_dive` 하나뿐이다~~ → ★**해소(11차 배치D) — 이 문면은 쓰일 때 이미 낡아 있었다**: "
    "`engage_requires_dive` 는 2차에 판정식 전량이 확정됐고(`history[4]` — `Effect::is_in_range_ex` 공식 + 오라클 900/900), "
    "3차에 합성 엔티티 **2,073,600쌍 mismatch 0** 으로 재확증됐다(`history[6]`). "
    "이 항목의 `closed.why` 자신이 바로 그 검증(「3차 배치D: 합성 엔티티 2,073,600쌍 mismatch 0」)을 인용한다 — "
    "물음과 근거가 정면으로 어긋나 있었다. 남은 미탐색은 **없다**(적용 범위 = 0.5.8 `_gaibc`/`_gcbc` IR + 오라클).",
    "같은 명세 `history[4]`(2차배치D: `_gcbc/g06.ll:51634~51782` is_in_range 전문 + `D2_oracle5.exe` game==mine 900/900) 와 "
    "`history[6]`(3차배치D: 676/676 · 진리표 260쌍 · 합성 1,440 엔티티 2,073,600쌍 mismatch 0). "
    "`closed[3].why` = 「3차 배치D: 합성 엔티티 2,073,600쌍 mismatch 0」 — 곧 engage_requires_dive 의 검증 근거 그 자체다")

# ───────────── 16 ─────────────
err("/specs[16]/open[0]", "실오류",
    "m14.ll:60624 의 %33 이 무슨 값인지는 담당 범위 밖이라 안 봄",
    "~~m14.ll:60624 의 %33 이 무슨 값인지는 담당 범위 밖이라 안 봄~~ → ★**해소: `history[4]` 참조**(11차 배치D 재확인) — "
    "`m14.ll:60622` `%32 = getelementptr inbounds nuw i8, ptr %31, i64 4856` → `60623` `%33 = load i64, ptr %32` 이므로 "
    "**4856 = 0x12f8 = `GameSetting.tick_per_second`**(= 정확히 1초). 반환값에는 `60625` `%35 = add i64 %34, 20000` 이 붙는다. "
    "⟹ 13개 호출부 중 **이 한 곳만 tps 스케일을 따라간다**",
    "IR 원문 m14.ll:60622~60625 `%32 = getelementptr inbounds nuw i8, ptr %31, i64 4856` / "
    "`%33 = load i64, ptr %32` / `%34 = tail call … max_range_nearly_can_use(…, i64 noundef %33)` / `%35 = add i64 %34, 20000`. "
    "같은 명세 `history[4]` 가 이미 확정한 것을 `closed[3]` 이 안 따라갔다(G17 형)")

err("/specs[16]/knobs[3]/where", "실오류",
    "entity.rs:1774/1789/1804",
    "entity.rs:1775/1790/1805 (각 함수의 `fn` 선언은 1774/1789/1804, **인용한 명령이 있는 본문 줄은 +1**)",
    "dloc.py m10.ll 로 `!dbg` 사슬 전개: `%97 = icmp eq i64 %96, 13, !dbg !56316` → `line 1775 in skill_cooldown` ← battle.rs:2409 / "
    "`%164 = icmp eq i64 %163, 13, !dbg !56335` → `line 1790 in skill2_cooldown` ← 2416 / "
    "`%223 = icmp eq i64 %222, 13, !dbg !56352` → `line 1805 in ult_cooldown` ← 2423. "
    "같은 명세의 `knobs[1]`/`knobs[2]` 는 본문 줄(1693/1701)을 쓰는데 `knobs[3]` 만 선언 줄을 썼다")

err("/specs[16]/logic", "보강",
    "// Entity::skill_cooldown() 인라인 (entity.rs:1774): ty==Champion 이면 +0xb8, 아니면 0",
    "// Entity::skill_cooldown() 인라인 (본문 entity.rs:1775 · `fn` 선언 1774): ty==Champion 이면 +0xb8, 아니면 0",
    "dloc m10.ll !56316 = `line 1775 in skill_cooldown` ← battle.rs:2409. 1774 는 DISubprogram 선언 줄이다")

err("/specs[16]/logic", "보강",
    "// Entity::attack_cooldown() 인라인 (entity.rs:1747) — ty(+0x68) 14-way switch",
    "// Entity::attack_cooldown() 인라인 (본문 switch 는 entity.rs:1748 · `fn` 선언 1747) — ty(+0x68) 14-way switch",
    "dloc m10.ll !56282(`switch i64 %19` 의 !dbg) = `line 1748 in attack_cooldown` ← battle.rs:2402")

# ───────────── 17 ─────────────
err("/specs[17]/open[0]", "실오류",
    "Chat::Battle 의 두 번째 필드 __1 에 항상 0 을 넣는 의미 — Chat 소비측(채팅 처리기)을 안 봐서 이 0 이 '없음'인지 '기본값'인지 확정 불가.",
    "~~Chat::Battle 의 두 번째 필드 __1 … 소비측을 안 봐서 확정 불가~~ → ★**해소: `history[0]` 참조**(11차 배치D 재확인) — "
    "`_gaibc` 에서 Chat 페이로드를 읽는 곳은 `handle_chat_inner` 하나뿐이고, 읽는 자리는 **`chat+0x8`(=`__0`, 대상 엔티티 id)뿐**이다: "
    "m13.ll:29993 `%141 = getelementptr inbounds nuw i8, ptr %6, i64 8` → 29994 `%142 = load i64, ptr %141` → "
    "30002 `%149 = tail call … %148(ptr %144, i64 %142)`(`%148` = vtable `+0x1f0 get_entity_by_id`). "
    "**`chat+0x10`(=`__1`)을 읽는 코드는 0건** ⟹ 이 0 은 '없음/기본값' 을 가르는 문제가 아니라 **죽은 슬롯**이다 "
    "(적용 범위 = 0.5.8 `_gaibc`·`_gcbc`; game_core 쪽은 복사·Debug·serde 뿐).",
    "IR 원문 m13.ll:29993~30003(위 인용) + 같은 명세 `history[0]` 의 전수 스캔. "
    "`15 consts[1]`(BattlePlanGoal::TryKill.__1)이 같은 결론을 이미 표에 반영했는데 17 만 안 따라갔다")

err("/specs[17]/knobs[1]/effect", "실오류",
    "Chat 소비측이 이 값을 쓰면(예: 우선순위·시각) 여기서 조절 가능하나, 소비측을 안 봐서 효과 미확정 — unknown 참조",
    "★**소비처 0건이라 사실상 노브가 아니다**(`history[0]`): `chat+0x10`(=`__1`)을 읽는 코드는 `_gaibc` 전량에 0건이고, "
    "유일 소비처 `handle_chat_inner`(m13.ll:29993~30003)는 `chat+0x8`(=`__0`, 대상 id)만 읽는다. "
    "⟹ 이 값을 바꿔도 동작이 변하지 않는다 — `15 consts[1]`·`15 knobs[4]`(BattlePlanGoal::TryKill.__1)와 **같은 형태**이고 15 는 이미 그렇게 정정돼 있다",
    "IR 원문 m13.ll:29993 `%141 = getelementptr inbounds nuw i8, ptr %6, i64 8` → 30002 get_entity_by_id 인자. "
    "같은 명세 `history[0]`: 「_gaibc 전체 구조 스캔 결과 Chat 페이로드를 읽는 곳은 위 1군데뿐. enum+0x10 은 game_ai 어디서도 읽히지 않는다」")

_ev17 = ("IR 원문 m05.ll:17744~17750 — `%19 = load ptr, ptr %7`(지역 Vec+0x8) / `store i8 3, ptr %19` / "
         "`%20 = getelementptr inbounds nuw i8, ptr %19, i64 8` `store i64 %13, ptr %20` / "
         "`%21 = getelementptr inbounds nuw i8, ptr %19, i64 16` `store i64 0, ptr %21` / `store i64 1, ptr %8`. "
         "`logic`(750줄)과 `knobs[1]`(「m05.ll 17749 `store i64 0, ptr %21`」)이 이 쓰기들을 인용하는데 `mem` 에 행이 없었다 — "
         "10차가 `18` 에서 고친 「Vec 원소 역참조를 안 싣는다」와 같은 결손")

ins("/specs[17]/mem", 19, "Chat 태그 (Battle 3)",
    {"base": "Chat(chats 원소[0], 힙 버퍼)", "offset": "0x0", "name": "Chat 태그 (Battle 3)",
     "value": "3 = Chat::Battle",
     "note": "`goal.tag == 0`(TryKill) 경로에서만 쓴다. IR 원문 m05.ll:17745 `store i8 3, ptr %19` "
             "(`%19 = load ptr, ptr %7` = 지역 `Vec<Chat>`+0x8 데이터 포인터, grow_one 직후 재로드). "
             "★좌표 원점은 `DeathMatchBattle` 이 아니라 **힙 원소**다(D9-OFF 규약 2). 11차 배치D 행 추가"},
    _ev17, bc=True)
ins("/specs[17]/mem", 20, "Chat::Battle.__0 (대상 엔티티 id)",
    {"base": "Chat(chats 원소[0], 힙 버퍼)", "offset": "0x8", "name": "Chat::Battle.__0 (대상 엔티티 id)",
     "value": "goal.TryKill.__0 (= p2 goal +0x8 에서 읽은 target)",
     "note": "IR 원문 m05.ll:17746~17747 `%20 = getelementptr inbounds nuw i8, ptr %19, i64 8` → `store i64 %13, ptr %20`. "
             "★`Chat` enum+0x8 = **대상 엔티티 id 슬롯**(틱 아님) — 유일 소비처 `handle_chat_inner`(m13.ll:29993~30003)가 "
             "이 자리를 vtable `+0x1f0 get_entity_by_id` 의 인자로 그대로 넘긴다(`history[0]`). 11차 배치D 행 추가"},
    _ev17, bc=True)
ins("/specs[17]/mem", 21, "Chat::Battle.__1",
    {"base": "Chat(chats 원소[0], 힙 버퍼)", "offset": "0x10", "name": "Chat::Battle.__1",
     "value": "0 (항상)",
     "note": "IR 원문 m05.ll:17748~17749 `%21 = getelementptr inbounds nuw i8, ptr %19, i64 16` → `store i64 0, ptr %21`. "
             "★**읽는 코드가 `_gaibc` 전량에 0건 = 죽은 슬롯**(`history[0]`) — `knobs[1]` 의 「소비측 미확인」은 11차에 정정됐다. "
             "11차 배치D 행 추가"},
    _ev17, bc=True)
ins("/specs[17]/mem", 22, "지역 chats.len (push 후)",
    {"base": "Vec<Chat>(지역 스택 24B)", "offset": "0x10", "name": "지역 chats.len (push 후)",
     "value": "1 (TryKill 경로) / 0 (그 외)",
     "note": "IR 원문 m05.ll:17750 `store i64 1, ptr %8`(`%8 = getelementptr inbounds nuw i8, ptr %6, i64 16`). "
             "초기값 0 은 m05.ll:17707 `store i64 0, ptr %8`. 이 24B 지역 Vec 이 뒤에 `DeathMatchBattle+0xf8`(`chats` 행)으로 "
             "통째 memcpy 된다. 11차 배치D 행 추가"},
    _ev17)
ins("/specs[17]/mem", 5, "지역 chats.ptr (원소 주소의 출처)",
    {"base": "Vec<Chat>(지역 스택 24B)", "offset": "0x8", "name": "지역 chats.ptr (원소 주소의 출처)",
     "note": "`RawVec::grow_one`(cap 0→4) 직후 다시 읽는다 — IR 원문 m05.ll:17744 `%19 = load ptr, ptr %7` "
             "(`%7 = getelementptr inbounds nuw i8, ptr %6, i64 8`). 이 포인터가 곧 `Chat` 원소[0]의 주소이고 "
             "위 세 `Chat(...)` 행의 좌표 원점이다. 초기값은 m05.ll:17705 `store ptr inttoptr (i64 8 to ptr), ptr %7`(dangling = align 8). "
             "11차 배치D 행 추가"},
    _ev17)

# ───────────── 18 ─────────────
err("/specs[18]/open[0]", "보강",
    "⚠적용 범위: 「+0x8 이 무엇인가」는 확정, 「Repair/SerpenSetup/Press 가 거기 0 을 넣는 뜻(=대상 없음)」은 소비측 미탐색",
    "⚠적용 범위: 「+0x8 이 무엇인가」는 확정. ~~「Repair/SerpenSetup/Press 가 거기 0 을 넣는 뜻(=대상 없음)」은 소비측 미탐색~~ "
    "→ ★**해소(11차 배치D)**: 17 의 `history[0]` 전수 스캔 = `_gaibc` 에서 Chat 페이로드를 읽는 곳은 `handle_chat_inner` 한 군데뿐이고 "
    "**태그 3/4/6(Battle 계)에서만 `chat+0x8` 을 읽는다**(m13.ll:29993~30003) ⟹ 태그 21/22/23/25 가 넣는 0 은 "
    "**아무도 읽지 않는 자리채움**이다(적용 범위 = 0.5.8 `_gaibc`·`_gcbc`; 미탐색 = exe·미추출 rlib)",
    "17 `history[0]`: 「유일 — handle_chat_inner m13.ll:29993~30003. tag 3(Battle)/4(BattleDive)/6(BattleHelp) 에서 chat+0x8 을 "
    "… 넘긴다」 + 「_gaibc 전체 구조 스캔 결과 Chat 페이로드를 읽는 곳은 위 1군데뿐」. "
    "IR 원문 m13.ll:29993 `%141 = getelementptr inbounds nuw i8, ptr %6, i64 8` 로 재확인")

err("/specs[18]/mem[16]/note", "실오류",
    "원소 24B. 태그 @+0, Press/PressChange 는 LineType @+1, 공통 usize 필드 @+8 에 항상 0 을 넣는다 ★**행 분리 **",
    "`Vec::push` 의 길이 갱신 — `load(TeamPlan+0xd0)` → `add …, 1` → 같은 자리에 되쓴다. "
    "IR 원문 m09.ll:6942~6943 `%28 = add i64 %19, 1` / `store i64 %28, ptr %18`(Repair 경로) · "
    "7016~7017 `%61 = add i64 %52, 1` / `store i64 %61, ptr %51`(SerpenSetup 경로). ★**행 분리 **",
    "10차의 행 분리 때 `mem[15]`(Chat 원소 태그)의 `note` 가 이 행에 **그대로 복사**돼 남았다 — "
    "이 행은 `TeamPlan+0xd0`(Vec 길이)인데 설명은 `Chat` 원소 24B 레이아웃을 말한다. "
    "IR 로 대조하면 이 자리에는 `store i8`/`store i64 0` 이 없고 `add`+`store` 뿐이다", bc=True)

err("/specs[18]/mem[21]/note", "보강",
    "네 경로(Repair 23 / SerpenSetup 25 / Press 21 / PressChange 22) 전부 `store i64 0`.",
    "네 경로(Repair 23 / SerpenSetup 25 / Press 21 / PressChange 22) 전부 `store i64 0`. "
    "⚠**이름 `__1` 은 Press/PressChange 기준이다** — `Chat::Press(LineType, usize)`/`PressChange` 는 `__0`=LineType@+0x1 · `__1`=usize@+0x8 이지만 "
    "`Chat::Repair(usize)`/`SerpenSetup(usize)` 은 이 자리가 **`__0`** 이다(`Chat::Battle(usize,usize)` 도 +0x8 이 `__0` — 명세 `17` 참조). "
    "variant 마다 필드 번호가 다르니 「+0x8 = 항상 `__1`」로 읽지 말 것.",
    "명세 `17` 의 IR 원문 m05.ll:17746~17747 이 `Chat::Battle` 의 `__0`(대상 id)을 `chat+0x8` 에 넣는다 — "
    "같은 바이트를 18 은 `__1`, 17 은 `__0` 이라 부르고 있었다. 둘 다 자기 variant 기준으론 맞고, "
    "틀린 것은 18 의 「**공통** usize 슬롯 = `__1`」이라는 일반화다")

err("/specs[18]/logic", "보강",
    "// [2] 적이 오브젝트를 먹는 중 + 세르펜 교전에서 확실히 이긴다 → 세르펜 징벌 — 651~654",
    "// [2] 적이 오브젝트를 먹는 중 + 세르펜 교전에서 확실히 이긴다 → 세르펜 징벌 — 651~658",
    "dloc m09.ll !14843(`store i8 25, ptr %59` 의 !dbg) = `ptr::write<Chat>`@ptr/mod.rs:1933 ← `Vec::push_mut`@vec.rs:1045 ← "
    "`Vec::push`@vec.rs:1004 ← **epic.rs:658**. 같은 명세 `consts[4].src_line` 이 이미 658 인데 `logic` 의 블록 머리만 654 로 남아 있었다")

err("/specs[18]/logic", "보강",
    " self.chats.push(Chat::SerpenSetup(0)); // 태그 25",
    " self.chats.push(Chat::SerpenSetup(0)); // 658, 태그 25",
    "dloc m09.ll !14843 루트 = epic.rs:658(위와 같은 근거). 이 블록의 다른 모든 문장에는 줄번호가 붙어 있는데 이 한 줄만 빠져 있었다")

# ───────────── 19 ─────────────
err("/specs[19]/consts[4]/meaning", "실오류",
    "② min_by_key 결과 unwrap 잔여검사 2곳(819/833줄, 시드가 있어 실제로는 발생 불가)",
    "② min_by_key 결과 unwrap 잔여검사 2곳(**821/836**줄 — 폴백 체인은 818 에서 시작해 `.unwrap()` 이 **821**, "
    "본선 체인은 833 에서 시작해 `.unwrap()` 이 **836** 이다. 819/833 은 체인의 *키 클로저/시작* 줄이지 unwrap 줄이 아니다. "
    "시드가 있어 실제로는 발생 불가) ③ `choose(rnd).unwrap()` 의 null 검사(829줄)",
    "dloc.py m04.ll: `%71 = icmp eq i8 %70, -1, !dbg !71892` → `unwrap<JungleType>`@option.rs:1011 ← "
    "**passive_jungle.rs:821** / `%129 = icmp eq i8 %128, -1, !dbg !72192` → ← **passive_jungle.rs:836** / "
    "`%142 = icmp eq ptr %136, null, !dbg !72211` → ← **829**. "
    "(참고: !71849 = `closure$2`@819 ← min_by_key ← **818**, !72133·!72138 = `closure$3`@834·835 ← min_by_key ← **833**)")

err("/specs[19]/logic", "실오류",
    "[818] let mode = (*data.cache.game).get_game_mode() // vtable +0x40, indirect call\n"
    " .unwrap(); // tag!=0 이면 option::unwrap_failed\n"
    " let runner: &JungleRunner = &mode.jungle_runner; // MobaMode +0x18, 480B\n"
    "[819] return jungle_camps.into_iter()\n"
    " .min_by_key(|c| runner.get_camp_state(player.info.team, *c).next_respawn_tick)\n"
    " .unwrap();",
    "[818] return jungle_camps.into_iter()\n"
    "[819] .min_by_key(|c| data.cache.game.get_game_mode() // vtable +0x40, indirect call\n"
    "                        .unwrap() // tag!=0 이면 option::unwrap_failed\n"
    "                        .jungle_runner // MobaMode +0x18, 480B\n"
    "                        .get_camp_state(player.info.team, *c).next_respawn_tick)\n"
    "[821] .unwrap();\n"
    " // ★DWARF 정본(11차 배치D): vtable +0x40 간접호출(m04.ll:62706~62708)의 !dbg 는 scope 가\n"
    " // `closure$2 @passive_jungle.rs:819` 이고 inlinedAt 루트가 818 이다 ⟹ get_game_mode() 는\n"
    " // **키 클로저 안**에 있다(LLVM 이 루프 밖으로 호이스트했을 뿐, 818 의 `let` 이 아니다).\n"
    " // 체인 끝 `.unwrap()` 은 **821**(!71892 = option.rs:1011 ← 821).",
    "dloc.py m04.ll !71849(= `%43 = getelementptr inbounds nuw i8, ptr %36, i64 64` 의 !dbg) = "
    "`line 819 in closure$2` → … → `line 818 in best_jungle_goal`. "
    "818 의 `let` 으로 호이스트된 것이라면 innermost scope 가 `best_jungle_goal`@818 이어야 한다. "
    "그리고 !71892(`%71 = icmp eq i8 %70, -1`) = `unwrap<JungleType>`@option.rs:1011 ← **821**")

err("/specs[19]/logic", "보강",
    "[817] if not_cleared_camps.len() == 0 { // 전부 클리어된 상태",
    "[817] if not_cleared_camps.is_empty() { // 전부 클리어된 상태. ★소스 형태는 `is_empty()` 다 — "
    "m04.ll:62643 의 !dbg 사슬이 `len<JungleType>`@vec.rs:1617 ← `is_empty<JungleType>`@vec.rs:1636 ← passive_jungle.rs:817 이다. "
    "IR 에서는 `load(len @+0x18)` + `icmp eq i64 …, 0` 으로 접힌다",
    "dloc.py m04.ll !71630(`%29 = getelementptr inbounds nuw i8, ptr %11, i64 24` 의 !dbg) = "
    "`line 1617 in len<JungleType>` ← `line 1636 in is_empty<JungleType>` ← `line 817 in best_jungle_goal`")

ins("/specs[19]/mem", 15, "캠프 태그(i8) — 원소 역참조",
    {"base": "JungleType(not_cleared_camps 원소, bumpalo 버퍼)", "offset": "0x0", "name": "캠프 태그(i8) — 원소 역참조",
     "note": "본선 fold 가 **실제로 읽는 값**. IR 원문 m04.ll:62852 `%100 = load i8, ptr %91, align 1, !range !41907`"
             "(`!41907 = !{i8 0, i8 6}`), 원점 `%91 = load ptr, ptr %11`(= Vec+0x0, 앞 행) · 다음 원소는 "
             "m04.ll:62850 `%99 = getelementptr inbounds nuw i8, ptr %91, i64 1`(stride 1B). "
             "`!dbg` 사슬 = `ptr::read<JungleType>`@ptr/mod.rs:1733 ← `IntoIter::next`@vec.rs:2450 ← passive_jungle.rs:833. "
             "★10차가 `07`·`08`·`14`·`18` 에서 고친 「Vec 의 ptr/len 은 싣고 **원소 역참조는 안 싣는다**」와 같은 결손 — "
             "포인터와 길이만으로는 재구현이 안 된다. 11차 배치D 행 추가"},
    "IR 원문 m04.ll:62809 `%91 = load ptr, ptr %11` / 62850 `%99 = getelementptr inbounds nuw i8, ptr %91, i64 1` / "
    "62852 `%100 = load i8, ptr %91, align 1, !dbg !72085, !range !41907` · "
    "dloc m04.ll !72085 = `read<JungleType>`@ptr/mod.rs:1733 ← `next<JungleType>`@vec.rs:2450 ← passive_jungle.rs:833")

err("/specs[18]/sig/params[2]/role", "보강",
    "PlayerState::strategy 호출에만 넘긴다(전략 샘플링용). 본문에서 직접 안 쓴다",
    "PlayerState::strategy 호출에만 넘긴다. 본문에서 직접 안 쓴다. "
    "⚠~~(전략 샘플링용)~~ — 같은 명세 `closed[5]`/`history[5]` 정본과 모순이었다: 본체 `_gcbc/g15.ll:130480` 의 "
    "`rnd` 인자(%2, StdRng 320B)에 **`readnone`** 이 붙어 있어 **RNG 상태를 전혀 안 건드린다**(받고 버리는 잔재). "
    "샘플링하지 않으므로 시드·호출 순서에 영향이 없다",
    "IR 원문 _gcbc/g15.ll:130480 `define void @…PlayerState8strategy(ptr … sret([24 x i8]) … %0, ptr noalias noundef readonly … dereferenceable(2528) %1, "
    "**ptr noalias noundef readnone align 16 captures(none) dereferenceable(320) %2**, ptr noundef nonnull %3, ptr noalias noundef readonly … dereferenceable(816) %4)`. "
    "같은 명세 `closed[5]` 가 이미 「readnone ⟹ RNG 를 전혀 안 건드린다」로 닫아 뒀는데 인자표만 「샘플링용」으로 남아 있었다(G1 자기모순)")

BRIEF = [
    "★**`closed[]` 는 이 계약으로 주소 지정이 안 된다.** 이번 라운드의 주 표적인데 `applypatch.py` 의 `resolve()` 에 `closed` 매핑이 없어 "
    "`/specs[i]/closed[n]/*` 는 전건 「인덱스 범위 밖(v3→v2 경계 확인)」으로 거부된다(dry-run 으로 실측). "
    "`closed[].q` 는 v2 `unknown`/`still_unknown` 에 있어 `/specs[i]/open[n]` 문면 경로로 우회되지만 "
    "**`closed[].why` 는 v2 에 아예 없다** — `_spec/closelist.py` 의 `CLOSE` 목록에서 파생되므로 `errors[]` 로는 못 고친다. "
    "이번 patch 의 `closed` 정정을 전부 `/specs[i]/open[n]` 으로 낸 이유다.",

    "★**`closelist.closed_reason()` 이 첫 매치를 돌려주어 나중에 추가된 정확한 근거가 영구히 가려진다.** "
    "실례 ① `19 closed[5]`(map 클로저 s_0): 초판 needle 이 「resolved[2]: _gcbc/g09.ll:137367~137440 확정」을 주는데 "
    "그 좌표는 `get_camp_state`(= `history[2]`·`closed[2]`)의 것이고 map 클로저(`from_iter_in` m01.ll 20914~21098)와 **무관**하다. "
    "4차에 올바른 근거 `(19, \"map 클로저(s_0)\", \"4차 배치A/C: 항등 사상 확인 기록(확정 서술)\")` 가 추가됐지만 앞 항목에 가려 안 쓰인다. "
    "실례 ② `15 closed[3]`: needle `(15, \"engage_requires_dive\")` 가 **그 이름을 언급만 하는 다른 항목**"
    "(single_tower_dive_is_viable 전량 확정)에 붙어 있다. `closelist.py` 자신이 4차에 「needle 은 그 항목의 고유 문면으로 잡는다 — "
    "함수명은 여러 항목이 언급한다」고 적어 뒀는데 **그 형태가 최소 2건 남아 있다.** "
    "제안: `audit()` 에 `shadowed`(같은 (i, 항목)을 두 needle 이 맞히면 뒤엣것이 죽는다)를 추가하고 `closed_reason` 을 **마지막 매치** 우선으로.",

    "도시에 §1 표에 **`closed` 개수 열이 없다.** 이번 라운드의 주 표적이 `closed` 인데 배치가 규모를 모르고 들어간다 "
    "(실측 담당분 = 15:7 / 16:7 / 17:8 / 18:10 / 19:7 = **39건**). `open`·`mem`·`consts`·`knobs` 는 있는데 `closed` 만 빠졌다.",

    "§5 의 `errors[].kind` 어휘가 도구와 다르다 — 도시에는 `실오류 / 오탐 / 보강`, `applypatch.py` docstring 은 "
    "`실오류 | 판정반전 | 분류오류 | 보강`. 7차에 「스키마가 계약과 다르면 0건 적용」이 실제로 났던 자리다"
    "(`kind` 는 적용에 안 쓰여 이번엔 무해했지만, 「판정반전」을 낼 자리가 도시에 어휘엔 없다).",

    "§3 분책의 인자표 `i` 칸에 **규약 주석이 없다.** 15·17 은 sret 행이 `i=0` 이라 0-based IR 인덱스처럼 보이고 16·18·19 는 `i=1` 부터라 "
    "1-based 로 보여, 같은 칸이 두 뜻인 것처럼 읽힌다. 실제로는 「**1-based 소스 자리, sret = 0**」으로 5함수 전부 일관된다"
    "(내가 「규약 불일치」로 적발했다가 스스로 반증했다). 한 줄 주석이면 이 오탐이 막힌다.",

    "§1 의 `ev≥4(미실행)` 주의문과 §5 의 「`mem` 의 `ev` 는 상한 3」 규칙이 어긋난다 — `18 mem[18]~[21]` 4행이 `ev=4` 로 찍혀 있다"
    "(10차 `op:insert` 로 들어온 행들이 `tcx 정본 대조` 문구를 안 달아 `evtier` 가 4 를 준 것). "
    "상한이 규칙이면 `mkspec3` 이 `mem` 에서 `min(ev,3)` 을 강제해야 하고, 아니면 §5 문구를 고쳐야 한다.",
]

out = {"round": 11, "batch": "D", "errors": E, "ev_up": [], "brief_errors": BRIEF}
io.open(r"_verify11\D\patch.json", "w", encoding="utf-8").write(
    json.dumps(out, ensure_ascii=False, indent=1))
print("errors", len(E), "(insert %d)" % sum(1 for e in E if e.get("op") == "insert"))
