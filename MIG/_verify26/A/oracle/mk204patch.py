# -*- coding: utf-8 -*-
"""26차 A · 204 calculate_score_parameter patch.json 생성 (mkpatch 참조구현 사용)."""
import sys, io
sys.stdout.reconfigure(encoding='utf-8')
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=26, batch="A")
S = "/specs[204]"

# ── G12 consts[20] = 오탐 ─────────────────────────────────────────────
p.fix(S + "/consts[20]/meaning",
      old=u"dbg 없음(`mul %433, 20` 호이스트)이라 1782/1793 중 어느 줄의 항인지 귀속 불가 — 값·용처는 확정: 평타·스킬·스킬2 도달 판정 전부에 가산",
      new=u"dbg 없음(`mul %433, 20` 호이스트)이라 1782/1793 중 어느 줄의 항인지 귀속 불가 — 값·용처는 확정: 평타·스킬·스킬2 도달 판정 전부에 가산 (★G12 오탐: m07.ll:40063 `%799 = mul i64 %433, 20` 은 `!dbg` 없는 LICM 호이스트이고 아군 루프 복제 41725 `%1504 = mul i64 %1232, 20` 도 같다. 첫 dbg 소비 = 40215 `%883 = add i64 %800, %864 ;L1793` · 40330 `%935 = add i64 %810, %915 ;L26<1809` · 40442 ;L26<1824 · 40608 ;L26<1840 · 40737 ;L26<1856 · 40866 ;L26<1871 / 41841 ;L1958 · 41956 ;L26<1974 · 42068 ;L26<1989. 게이트 후보 2069/2082/2097 은 consts[41](내 챔프 루프 · 이 함수 단독 실행에선 사장) 의 별개 사이트. 오라클 C14d/e: 적 move_speed 1→2 에서 도달 경계가 120020→120040 으로 +20 이동 = 계수 20 실측)",
      evidence=u"m07.ll:40063 `  %799 = mul i64 %433, 20`(줄 끝 !dbg 없음) · 40064 `%800 = add i64 %799, %795`(없음) · 40215 `%883 = add i64 %800, %864, !dbg` 루트 L1793 · 41725 `%1504 = mul i64 %1232, 20`(없음) → 41841 ;L1958. srclinecheck 는 !dbg 없는 명령을 못 보므로 후보에 2069/2082/2097(42996/43130/43267 · consts[41] 사이트)만 남는다 = 도구 오탐. 오라클 o204 C14b~e(_verify26/A/oracle/o204_cases.log) 계수 20 실측",
      behavior_change=False, found_by="reused", kind=u"오탐")

# ── consts[18] kind 미상 → 임계 낱말 ─────────────────────────────────
p.fix(S + "/consts[18]/meaning",
      old=u"70000² — near_towers 필터의 대안 조건:",
      new=u"70000² 거리 임계(`icmp ult dist², 4900000000`) — near_towers 필터의 대안 조건:",
      evidence=u"aux m07.ll:61755/61878 `icmp ult i64 %.., 4900000000` 비교 상한 = 임계. 오라클 C31z/C32z: 대안 조건이 비대칭(적 타워 ↔ near_enemies_with_action · 내 타워 ↔ near_allies_with_action)임을 실행으로 확인(나-내타워 155000 이고 적이 내 타워 95000 이어도 내 타워는 near_towers 에 안 들어감)",
      behavior_change=False, found_by="new", kind=u"보강")

# ── TLS 절: 유령 호출자 score_parameter_cached 정정 + 간접 TLS ───────
p.fix(S + "/sig/tls/role",
      old=u"작성자/소비자 아님. 지시문 승계: score_parameter_cached(r15 · TLS 200엔트리 5차원 캐시)의 미스 경로에서 호출되는 순수 계산 함수",
      new=u"작성자 아님 · 직접 소비자 아님 · 간접 소비자(콜리 경유). 호출자 3곳 전부 직접 호출이며 캐시 경유 없음: LegacyPlanHandler::get_small_action(auction.rs:10 `_t_csp` · m13.ll:45850) · mod_ai_ext::small_action_input(m15.ll:23147) · <PlayerAiContext as ModAiSmallActionExt>(m15.ll:56921). `score_parameter_cached` 라는 심볼은 _gaibc 전 모듈에 존재하지 않는다(grep 0건)",
      evidence=u"`grep -l score_parameter_cached C:/tfm2mods/_gaibc/*.ll` → 0 파일. m13.ll:45850 `invoke void @…calculate_score_parameter(ptr … sret([5384 x i8]) … %95, i64 noundef %2, …)` !dbg !50134 = DILocation(line: 10, scope: !49196=DILexicalBlock(get_small_action, auction.rs, line 9)) · m15.ll:23147(small_action_input line 50) · m15.ll:56921(ModAiSmallActionExt line 41)",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix(S + "/sig/tls/key",
      old=u"해당 없음(캐시 키는 score_parameter_cached 명세 몫)",
      new=u"직접 키 없음. 간접: precompute_champion_powers(L1501 나 · L1547 아군마다 · L1578 적마다) 가 CHAMP_POWERS_MEMO 를 키 (game.seed = vt+0x20, game.tick = vt+0x28) 로 열고 slot[team][pos] 의 (tag==1 && slot.version==version) 이면 hit → p+0xb8..0xd0(attack_power·util_power_base·cc_time_x_inv_cd·buff_inv_cd_count) 에 그대로 복사",
      evidence=u"m04.ll:53288~53295 `%19 = load ptr, ptr %17+32 … %20 = call i64 %19(ptr %15)`(vt+0x20 seed) · +40 tick · m04.ll:53308 LocalKey::with(closure#0) · m00.ll:77956~78122 closure#0: `%18 = load i64, ptr %6+488`(seed) `icmp eq %18, %20` · `%26 = load i64, ptr %6+496`(tick) · `%56 = gep [5 x { i64, [5 x i64] }], ptr %16(+8), i64 %31(team)` `%57 = gep …, i64 %47(pos)` · `%58 = load i64, ptr %57`(tag) `%60 = load %57+8`(version) `%64 = icmp eq %60, %62(version 인자)` · hit → memcpy 32B(%57+16) ; m04.ll:53347~53354 store p+184/192/200/208",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix(S + "/sig/tls/layout",
      old=u"해당 없음",
      new=u"직접 없음. 간접 CHAMP_POWERS_MEMO(m04.ll:141 `thread_local global <{ [504 x i8], [1 x i8], [7 x i8] }>`) = RefCell borrow 플래그 8B @+0 + ChampPowersMemo 496B @+8: slot[2][5] Option<(usize version, i64 attack_power, i64 util_power_base, i64 cc_time_x_inv_cd, i64 buff_inv_cd_count)> 48B 원소(태그 8B Direct · 0=None/1=Some) @memo+0x0..0x1e0 · seed @memo+0x1e0 · tick @memo+0x1e8 (tcxdict ChampPowersMemo 496B: slot[0][0]@tag 0x0 · slot[t][p] = 0x0 + (t*5+p)*48). 전역 기준 slot = +8+(t*5+p)*48 · seed +488 · tick +496",
      evidence=u"tcxdict ChampPowersMemo --deep(496B · utils.rs:778) · DI !1796~!1800(seed offset 3840 bit · tick 3904 · slot 3840 bit 크기) · m00.ll:78022~78041 slot 리셋 store 오프셋 8,56,104,…,440(=8+48k · k=0..9) · 488/496 seed/tick",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/sig/tls/invalidation",
      old=u"해당 없음",
      new=u"직접 없음. 간접 CHAMP_POWERS_MEMO: 조회 클로저가 (memo.seed != game.seed || memo.tick != game.tick) 이면 seed/tick 을 갱신하고 slot 10개 태그를 전부 0(None) 으로 리셋한 뒤 miss; 같은 (seed,tick) 안에서는 slot[team][pos] 가 Some 이고 version 이 같으면 hit(엔티티 스탯 변화는 키에 없다) ⟹ 한 프로세스에서 같은 (seed,tick) 으로 재호출하면 스탯을 바꿔도 +0x9d0..0x9e8 이 첫 값으로 재생 → 오라클은 케이스당 프로세스 1개(o204 · 63 케이스 전부 별도 프로세스)",
      evidence=u"m00.ll:78000~78041 `br i1 %21(seed 일치), label %24, label %33` · `%28 = icmp eq i64 %26, %27`(tick) · %33: `store i64 %20, ptr %17`(seed) `store i64 %34, ptr %35`(tick) + `store i64 0` ×10(slot 태그) → `br label %29` · %55~%66: 태그·version 검사 후 memcpy 32",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/sig/tls/call_conditions",
      old=u"참조하는 @anon 상수는 .11(Entity 용 dyn vtable 88B)·.31(정적 Option<Effect> None)·.153~.170(panic Location) 뿐이며",
      new=u"참조하는 @anon 상수는 .11(Entity 용 dyn AbstractEntity vtable · 72회)·.31(정적 Option<Effect> None · 9회)·.123~.170(panic Location 48개 = unwrap_failed 46 + panic_bounds_check 2 · score_parameter.rs 1462~2393) 뿐이며",
      evidence=u"m07.ll 37862~46714 `@anon.81aa…` 참조 집계: .11 ×72 · .31 ×9 · .123~.170 각 1회(m07.ll:144~191 정의 = `<{ ptr @anon….2, [16 x i8] c\"\\1E…\" }>` Location{file 30자, line, col} · .123 line 0x5b6=1462 · .152 line 0x861=2145 · .170 line 0x959=2393)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/sig/tls/call_conditions",
      old=u"다른 배치가 TLS 접점을 발견하면 그 배치가 정정.",
      new=u"다른 배치가 TLS 접점을 발견하면 그 배치가 정정. ★26차 A 콜리 1단계 TLS 스캔(직접 호출 심볼 41개의 define 본문): 간접 접점 2 — ①precompute_champion_powers(m04.ll:53234 · LocalKey::with) → CHAMP_POWERS_MEMO(항상 · 호출 1+|near_allies|+|near_enemies| 회 · key/layout/invalidation 절 참조) ②enemy_minion_wave_risk_damage_at(m07.ll:47104 · L2341 `!is_line_phase(tick)` 경로만) → expected_attack_damage_cached → position_eval ATTACK_DMG_CACHE(88B · m07.ll). enemy_minion_line_action_danger_damage_at(m07.ll:48226 · L2338 라인 페이즈 경로) 는 1단계 TLS 0(하위 enemy_minion_line_action_damage_at 은 미독). game_core 콜리(expected_damage_target·is_in_orbit·has_cc·check_projectile·check·range_adjust·remain_action_time·iter_minions·iter_towers_without_nexus·player_by_champion_id)·bumpalo·drop_glue·fold 클로저 전부 TLS 0",
      evidence=u"scratchpad26A/callee_tls2.py: 본문 call/invoke 대상 41 심볼 → define 본문 grep(threadlocal/LocalKey/call_once/__RUST_STD_INTERNAL_VAL). hit = m04.ll:53234 precompute_champion_powers ['LocalKey'] · m07.ll:47104 enemy_minion_wave_risk_damage_at ['LocalKey' → `@…LocalKey…AttackDamageCacheEE4with…expected_attack_damage_cached`] · m12.ll fold 클로저 5개의 'call_once' 는 FnOnce::call_once 이름(TLS 아님)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/sig/tls/name",
      old=u"없음 — 이 함수 자체의 TLS 접점 0",
      new=u"없음 — 이 함수 자체의 TLS 접점 0 (간접 소비 2: CHAMP_POWERS_MEMO ← precompute_champion_powers · ATTACK_DMG_CACHE ← enemy_minion_wave_risk_damage_at, call_conditions 참조)",
      evidence=u"위 call_conditions 항목과 동일(callee_tls2.py 1단계 스캔)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/one_line",
      old=u"TLS 접점 없음(순수 계산).",
      new=u"직접 TLS 접점 없음(콜리 precompute_champion_powers 의 CHAMP_POWERS_MEMO 를 간접 소비 · rnd/_debug 미사용). ★L2004~2100(자기 행동 확정피해·내 챔프 위협 적재)은 player.action 이 L1463 RunAway 고정이라 이 함수 단독 실행에선 도달 불가.",
      evidence=u"m07.ll:38142 `store i64 0, ptr %75`(%75=%47+2328=player.action@tag) 가 함수 전체 유일 store · 42138 `%1702 = load i64, ptr %75` → 42143 `icmp eq %1702, 0` → 42144 `br i1 %1703, label %1706, label %1704` 항상 %1706. 오라클 C20 실측",
      behavior_change=False, found_by="new", kind=u"보강")

# ── logic: L2004 사장 표기 + reach 한계 ──────────────────────────────
p.fix(S + "/logic",
      old=u"if parameter.player.action != SmallAction::RunAway {   // L2004: 0x918 태그 != 0 (42138~42144) · RunAway 면 L2111 로 직행(L2005~2100 전부 스킵)",
      new=u"if parameter.player.action != SmallAction::RunAway {   // L2004: 0x918 태그 != 0 (42138~42144) · RunAway 면 L2111 로 직행(L2005~2100 전부 스킵)\n  // ★★(26차 A) 이 함수 단독 실행에서는 항상 거짓 = L2005~2100 사장(블록 %1704~%1968 약 133개): player.action@tag(+0x918) 의 store 는 L1463 `store i64 0, ptr %75`(m07.ll:38142) 하나뿐이고, &mut parameter.player 를 받는 precompute_champion_powers(L1501) 도 p+184..208 만 쓴다(m04.ll:53347~53354) · Vec push/reserve 는 힙·len/ptr/cap 만. reach.py(version/gamemode 상수만 접음)는 이 「alloca 필드 store→load 상수」를 못 접어 926 블록 전부 live 로 보고한다. 오라클 C20(내 블랙보드 Attack(e0)+공격 이펙트 100·쿨 0·사거리 안): sret player.action=0 유지 · near_enemies[0].risk_possible len 0 · risk/applyed 0. ⟹ 런타임 프로브: 42147~43334 사이트는 발화 0 이 정상. 행동이 채워진 ScoreParameter 로 이 블록을 타는 것은 호출자 쪽 &mut ScoreParameter 경로(calculate_score_parameter_value 등 · 별도 명세)다",
      evidence=u"scratchpad26A/sret_live.py: %47 파생 gep 79개 전수에서 +0x918 store = 38142 1건. m04.ll:53234 precompute_champion_powers 의 %2 파생 gep = +96/+104(load) · +184/+192/+200/+208(store) 뿐. 오라클 _verify26/A/oracle/o204_cases.log C20 MATCH",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/logic",
      old=u"// ★TLS 접점 0 · rnd(gen_range) 사용 0 · NA(사장) 콜리 0(reach.txt: 블록 926 전부 live)",
      new=u"// ★직접 TLS 접점 0(간접: CHAMP_POWERS_MEMO·ATTACK_DMG_CACHE — sig.tls 참조) · rnd(gen_range) 사용 0(define `readnone` · 오라클 rnd_used=false 63/63) · _debug 쓰기 0(`readnone` · 오라클 debug_changed=false) · NA(사장) 콜리 0(reach.txt: 블록 926 전부 live — 단 L2004~2100 은 메모리 상수 사장, 위 참조)",
      evidence=u"m07.ll:37862 define `ptr noalias noundef readnone align 16 captures(none) dereferenceable(320) %2` · `ptr noalias noundef readnone align 8 captures(none) dereferenceable(224) %5` · 본문 %2/%5 참조 = #dbg_value 1건씩. o204_cases.log 전 케이스 rnd_used=false debug_changed=false",
      behavior_change=False, found_by="new", kind=u"보강")

# ── sret 절: store 전수 수치 + 포인터 필드 ───────────────────────────
p.fix(S + "/sig/ret",
      old=u"⟹ ev1 대조 시 미기록 구간(2320+16+294+7 B)은 undef 페이로드로 취급할 것.",
      new=u"⟹ ev1 대조 시 미기록 구간(2320+16+294+7 B)은 undef 페이로드로 취급할 것. ★26차 A store 전수(주석본 %47 파생 gep 79개 · store/memset/memcpy + 콜리 &mut precompute_champion_powers p+184..208 + reserve_internal_or_panic ptr/cap): 기록 2747B / 미기록 2637B = 2320+16+294+7 정확히 일치(scratchpad26A/sret_live.py). 런타임 SRET_LIVE 마스크 = [0x0,0x8)+[0x918,0x920)+[0x930,0xa22)+49×[0x9f0+56k, +50)+[0x14a8,0x1501). ⚠프로세스 의존 포인터 필드(값 대조 대신 역참조 대조): +0x938/+0x958/+0x14c0/+0x14e0(bumpalo Vec `a` = &Bump) · +0x930/+0x950(빈 Vec ptr=8 고정이나 push 후 힙) · +0x14b8/+0x14d8(near_* buf.ptr · push 후 bumpalo 힙) · 힙 원소 216B 안의 +0x20/+0x40(risk/gain_possible.a) · +0x18(risk_possible.buf.ptr) · 원소의 action 페이로드 +0x8..0x18 은 blackboard Option<SmallAction> 24B memcpy 라 RunAway 면 잔류값(오라클 실측 tgt=140710853185760). 필드값 실측 63/63 MATCH(o204_cases.log)",
      evidence=u"sret_live.py 출력: `[0x8,0x918) 2320B ★미기록` `[0x920,0x930) 16B ★미기록` `[0xa22,0xa28) 6B ★미기록` ×49 `[0x1501,0x1508) 7B ★미기록` · 기록 바이트 합 2747/5384. m07.ll:38146 `store ptr inttoptr (i64 8 to ptr), ptr %77`(+0x930) · 38148 `store ptr %65, ptr %78`(+0x938 = context.pool)",
      behavior_change=False, found_by="new", kind=u"보강")

# ── consts[41]·knobs[15]·knobs[14]: 사장 사이트 표기 ─────────────────
p.fix(S + "/consts[41]/meaning",
      old=u"move_speed × 20 = 20틱(1/3초) 이동 여유를 사거리에 가산(m07.ll:42996 · L2082 43130 · L2097 43267 · e 쪽은 배치 B 41725 %1504)",
      new=u"move_speed × 20 = 20틱(1/3초) 이동 여유를 사거리에 가산(m07.ll:42996 · L2082 43130 · L2097 43267 — ★이 세 사이트는 L2004 블록 안이라 이 함수 단독 실행에선 사장(logic L2004 참조) · 살아있는 사이트는 적 루프 40063 %799 / 아군 루프 41725 %1504 = consts[20] · 오라클 C14d/e 계수 20 실측)",
      evidence=u"logic L2004 보강 항목과 동일 근거(38142 유일 store · 42144 항상 %1706). 42996/43130/43267 은 %1704~%1968 사이",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/knobs[15]/effect",
      old=u"move_speed×N 틱만큼 사거리를 늘려 본다. 올리면 더 먼 적도 위협권으로 들어온다",
      new=u"move_speed×N 틱만큼 사거리를 늘려 본다. 올리면 더 먼 적도 위협권으로 들어온다 (★2069/2082/2097 사이트는 이 함수 단독 실행에선 사장 — player.action 이 RunAway 고정, logic L2004 참조. 실효 사이트 = 배치 B 40063(%799 적→아군/나) · 41725(%1504 아군→적). 오라클 C14d/e: 적 move_speed 1→2 에서 도달 경계 +20 실측)",
      evidence=u"m07.ll:38142/42138~42144 · o204_cases.log C14b~e MATCH",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/knobs[14]/effect",
      old=u"올리면 더 먼 미래(쿨다운·CC 중)의 스킬/평타까지 risk_possible 로 잡아 적 위험 평가가 비관적으로 커진다. 내리면 즉시 가능한 위협만 남는다",
      new=u"올리면 더 먼 미래(쿨다운·CC 중)의 스킬/평타까지 risk_possible 로 잡아 적 위험 평가가 비관적으로 커진다. 내리면 즉시 가능한 위협만 남는다 (★1975/1990 은 살아있고 2069/2082/2097 은 이 함수 단독 실행에선 사장 — logic L2004 참조. 오라클 C12/C13: 적 attack_cooldown 120 → 등록 · 121 → 미등록 · C29c/d: 평타·스킬 tick 은 각각 독립 판정)",
      evidence=u"o204_cases.log C12/C13/C29c/C29d MATCH · m07.ll:42144",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/knobs[4]/effect",
      old=u"현재 보이거나 최근 120틱(=2초@60tps) 내 보였던 적만 near_enemies 에 포함. 늘리면 시야 밖 적을 더 오래 위험/이득 계산에 남긴다",
      new=u"현재 보이거나 최근 120틱(=2초@60tps) 내 보였던 적만 near_enemies 에 포함. 늘리면 시야 밖 적을 더 오래 위험/이득 계산에 남긴다 (오라클 C5/C6: blackboard[1-team].last_visible[pos] = tick-120 → 포함 · tick-121 → 제외 · C7: visible_state[0]=Visible 이면 last_visible 없이도 포함 · C8: 적 팀 블랙보드 small_actions[pos] 가 None 이면 가시라도 제외)",
      evidence=u"o204_cases.log C5/C6/C7/C8 MATCH · shared.is_recent_visible 판정식 (A)/(B)",
      behavior_change=False, found_by="new", kind=u"보강")

# ── ev 상향(오라클 실행 확인) ─────────────────────────────────────────
ORC = u"오라클 실행 확인: _verify26/A/oracle/o204.rs(pub game_ai::calculate_score_parameter 직접 호출 · 케이스당 프로세스 1개) 63/63 MATCH — "
p.ev(S + "/sig/params[2]", ORC + u"전 케이스 rnd_used=false(호출 전후 StdRng clone 비교)", to=2)
p.ev(S + "/sig/params[5]", ORC + u"전 케이스 debug_changed=false(DebugFrameData 224B 전후 바이트 비교)", to=2)
p.ev(S + "/knobs[3]", ORC + u"C2 adist 200000 → near_allies 4 · C3 200001 → 0 · C8b 적 200001 → 0", to=2)
p.ev(S + "/knobs[4]", ORC + u"C5 last_visible=tick-120 포함 · C6 tick-121 제외", to=2)
p.ev(S + "/knobs[6]", ORC + u"C9 미시전→risk · C10 action_state 3→applyed · C11 논타겟 시전중→risk · C21/C21b 아군→적 동형 · C29a/b 스킬 동형", to=2)
p.ev(S + "/knobs[8]", ORC + u"C25a Skill2 행동 + level 1 → PANIC(unwrap) · C25b level 3 → 통과", to=2)
p.ev(S + "/knobs[10]", ORC + u"C12 cooldown 120 → PossibleGain 등록 · C13 121 → 미등록", to=2)
p.ev(S + "/knobs[11]", ORC + u"C14b/c 경계 120020/120021 · C14d/e speed 2 → 120040/120041", to=2)
p.ev(S + "/knobs[12]", ORC + u"C11 casting Position + action_state 3 → risk_damage(applyed 0)", to=2)
p.ev(S + "/knobs[13]", ORC + u"C28b Ult 행동·level 5·사거리 안 → risk_possible 에 평타 1건뿐(궁 미등록)", to=2)
p.ev(S + "/knobs[17]", ORC + u"C18 타깃 소실 · 미니언 0 < 2 → risk_possible_tower 가산(값 2 자체의 경계는 미니언 부재로 미실측)", to=3)
p.ev(S + "/knobs[18]", ORC + u"C25a level 1 Skill2 PANIC · C28a level 4 Ult PANIC · C28b level 5 통과", to=2)
p.ev(S + "/knobs[19]", ORC + u"C19a None tick 10000000 > 9999999 → 타워 블록 스킵 · C19b/c Bottom(3) 10801 스킵/10800 실행 · C19d/e MidBottom 14401/14400 · C33a/b 게이트가 배치 D 아군·적 타워 블록까지 덮음", to=2)
p.ev(S + "/knobs[26]", ORC + u"C9/C16/C21/C22/C31b 등 risk_damage = 누적>>1 · risk_possible_tower 는 반감 없음(C15/C17a/C18)", to=2)
p.ev(S + "/knobs[27]", ORC + u"C1b (960000,960000) → cx=cy=29 · C1c (959999,32000) → 29/1", to=2)
p.ev(S + "/consts[17]", ORC + u"C2/C3/C8b 200000 포함·200001 제외", to=2)
p.ev(S + "/consts[19]", ORC + u"C12/C13 cooldown 120/121", to=2)
p.ev(S + "/consts[20]", ORC + u"C14d/e 계수 20", to=2)
p.ev(S + "/consts[82]", ORC + u"C1b min(30,29)=29", to=2)
p.ev(S + "/consts[86]", ORC + u"C1c 959999/32000=29 · 32000/32000=1", to=2)
p.ev(S + "/consts[87]", ORC + u"C9 risk_damage 100 → 50", to=2)
p.ev(S + "/consts[2]", ORC + u"C14a RunAway 적은 위협 적재 없음 · C26a RunAway 는 attack_effect None 이어도 패닉 없음", to=2)
p.ev(S + "/consts[3]", ORC + u"C9/C10 Attack(6) arm", to=2, frm=3)
p.ev(S + "/consts[4]", ORC + u"C29a/b Skill(7) arm", to=2)
p.ev(S + "/consts[5]", ORC + u"C25a/b Skill2(8) arm", to=2)
p.ev(S + "/consts[6]", ORC + u"C28a/b Ult(9) arm", to=2)
p.ev(S + "/consts[11]", ORC + u"C10 action_state 3 → is_in_attack", to=2)
p.ev(S + "/consts[12]", ORC + u"C29b action_state 4 → is_in_skill", to=2)
p.ev(S + "/consts[66]", ORC + u"C31c/C32c 타깃 소실 · 미니언 0 < 3 → risk_damage(값 3 의 경계는 미니언 부재로 미실측)", to=3)
# mem(상한 3): 오라클이 같은 오프셋으로 직독한 필드
for i, why in [(143, u"+0x0 wave_tag=0"), (144, u"+0x918 act=0"), (147, u"+0x970 id=champ.id"), (148, u"+0x978 team"), (149, u"+0x980 pos(C24 me=2 → 2)"),
               (150, u"+0x988 applyed_damage(C10)"), (152, u"+0x998 risk_damage(C9)"), (155, u"+0x9b0 risk_possible_tower(C15)"), (159, u"+0x14a8/+0x14b0 cx/cy(C1b/c)"),
               (160, u"+0x14b8 ptr/+0x14d0 len(C2 4)"), (161, u"+0x14d8 ptr/+0x14f0 len(C5 5)"), (162, u"+0x14f8 version(C23 1)"), (163, u"+0x1500 v3tb=0"),
               (166, u"원소 +0x58 id/+0x60 team/+0x68 pos"), (170, u"아군 원소 +0x70(C22 대상 아군)"), (171, u"아군 원소 +0x80(C22 rd=50)"), (177, u"+0x948 len(C9 1 · C29a 2)"),
               (178, u"아군 원소 +0x80"), (180, u"아군 원소 +0x30 len(C22 1)"), (181, u"적 원소 +0x80(C21 200)"), (182, u"적 원소 +0x70(C21b 400)"),
               (183, u"적 원소 risk_possible push(C30b 8 = 평타 4+스킬 4)"), (184, u"적 원소 risk_possible push(C21 4 · from/tick/value 직독)"),
               (197, u"+0x998 타워(C16 50)"), (198, u"+0x9b0 타워(C15/C17a/C18 100)"), (199, u"적 원소 +0x80 타워(C31b/c 50)"), (200, u"적 원소 +0x98 타워(C31a/d 100)"),
               (201, u"아군 원소 +0x80 타워(C32b/c 50)"), (202, u"아군 원소 +0x98 타워(C32a 100)"), (203, u"+0x998 adjust lshr(C9)"), (209, u"+0x14a8 cx"), (210, u"+0x14b0 cy"), (211, u"sret 전체 memcpy(오라클 반환값 직독)")]:
    p.ev(S + "/mem[%d]" % i, ORC + why, to=3)

# ── 내 지시(도시에)의 오류 ─────────────────────────────────────────────
p.brief_error(u"지시문 ④의 「TLS 절 작성자 신규 3 … 간접 소비(…)」 목록에 CHAMP_POWERS_MEMO 의 소비자로 calculate_score_parameter(L1501/1547/1578 → precompute_champion_powers) 가 빠져 있다 — 이 함수의 tls 절은 「접점 0」이라 적혀 있었고 실제로는 콜리 경유 간접 소비자(케이스당 프로세스 1개 규칙의 적용 대상)다.")
p.brief_error(u"명세 tls.role/key 가 승계한 「score_parameter_cached(r15 · TLS 200엔트리 5차원 캐시)의 미스 경로」는 존재하지 않는 심볼이다(_gaibc grep 0건) — 호출자는 get_small_action(auction.rs:10)·small_action_input·ModAiSmallActionExt 직접 호출 3곳. 유령 호출자 이름이 지시문에서 명세로 두 라운드 승계됐다.")
p.brief_error(u"지시문 ①의 G12 후보 [2069,2082,2097] 은 같은 값(20)의 다른 사이트(consts[41])이고, 그 세 사이트는 이 함수 단독 실행에선 사장 블록 안이다 — 게이트 후보를 「고쳐 넣을 줄」로 읽으면 살아있는 사이트(40063/41725)를 사장 사이트로 바꿔 적게 된다.")
p.brief_error(u"지시문 ②의 sret 항목 「calculate_score_parameter · 미초기화 패딩 범위 = wave_snapshot None 페이로드 · PositioningScore 6B×49 · 꼬리 7B」에 player.action 페이로드 16B([0x920,0x930))가 빠져 있다(명세 ret 절에는 있음 · store 전수로 2637B 재확인).")
p.brief_error(u"reach.py 의 「사장 0 · 926 블록 전부 live」를 도시에가 봉인 재료로 제시하지만, reach.py 는 인자 상수(version/gamemode)만 접는다 — alloca 필드 store→load 상수(L1463 player.action=0 → L2004)를 못 접어 약 133 블록(14%)이 live 로 잘못 보고된다. 다음 라운드 게이트/도구 제안: heapsurf 식 콜리 쓰기 표면과 결합해 「함수 내 유일 store 상수 필드의 load」를 접는 규칙.")
p.save()
