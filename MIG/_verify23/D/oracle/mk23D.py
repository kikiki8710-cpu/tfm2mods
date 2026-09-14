# -*- coding: utf-8 -*-
"""23차 배치D patch.json 생성 (mkpatch 참조구현 사용). 대상 specs[158~164]."""
import sys, io, json
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import mkpatch

p = mkpatch.Patch(round=23, batch="D")
NEW = "new"; RE = "reused"

# ═════════════════════════ 158 line_minion_action_candidates ═════════════════════════
O158 = "오라클 실행 확인(23차 배치D o158.rs · game_ai::line_minion_action_candidates 직접 호출 · 가짜 미니언 Entity 를 cache.top/mid/bottom_minions[1] 에 raw 주입 · 11/11 MATCH vs 독립 재구현 · 케이스당 프로세스 1개)"
p.fix("/specs[158]/logic",
      old="effect.rs:1162 헬퍼 인라인, None 이면 undef",
      new="Option::map(option.rs:1162 · !77974) 안의 closure$0(lane_minion.rs:84 · !78037) — 안에서 Effect::range(effect.rs:25~26, !78046 인라인: range + growth_range*(level-1) + stat_buff_cached.range) 를 부르고 closure 본문이 champ_radius 를 더한다(!78050 line 84). 반환 u64(map<ref$<Effect>,u64,…>). None 이면 undef",
      evidence="m11.ll:139095 `!78044 = !DILocation(line: 1162, scope: !77974, …)` · 139025 `!77974 = distinct !DILexicalBlock(scope: !77961, file: !9150, line: 1162)` · 70201 `!9150 = !DIFile(filename: \"library\\\\core\\\\src\\\\option.rs\")` · 139012 `!77961 = … name: \"map<ref$<game_core::simulation::effect::Effect>,u64,…closure_env$0>\"` · 139180 `!78129 = !DILocation(line: 26, scope: !78046(range · effect.rs:25), inlinedAt: !78050)` · 139101 `!78050 = !DILocation(line: 84, scope: !78037(closure$0))`. 1162 는 effect.rs 가 아니라 option.rs 다",
      behavior_change=False, found_by=NEW, kind=u"실오류")
p.fix("/specs[158]/logic",
      old="게임 데이터상 도달 여부 미확인",
      new="오라클 실측(o158 1차 실행): Entity::can_attack() 은 attack_effect 유무를 안 보므로 attack_effect 태그를 -1 로 두면 `lane_minion.rs:104:34 called Option::unwrap() on a None value` 패닉이 실제로 난다 — SDK 수준에선 도달 가능, 실전은 전 챔피언이 attack_effect 를 가져 미발생(추정)",
      evidence="o158.exe case1 1차 실행 stderr: `thread 'main' panicked at game-ai\\src\\small_action\\lane_minion.rs:104:34: called Option::unwrap() on a None value` (attack_effect +0x4c0 = -1 · can_attack()==true 출력)",
      behavior_change=False, found_by=NEW, kind=u"보강")
p.fix("/specs[158]/sig/params[0]/role",
      old="0x11~0x17 은 alloca 잔류(미기록)",
      new="0x11~0x17 은 alloca 잔류(미기록) — ★콜리 `::new` 의 sret 속성이 `initializes((0, 17))`(m07.ll:7129 Attack · 7645 Skill · 12020 Skill2)이라 [0x0,0x11) 17B 만 기록이 보장된다(ELEM_LIVE: Attack15/Skill16/Skill17 = [0x0,0x11)+0xb1)",
      evidence="m07.ll:7129 `define void @…SmallActionAttack3new(ptr … sret([24 x i8]) … initializes((0, 17)) %0, …)` · 7645 SmallActionSkill3new 동일 · 12020 SmallActionSkill23new 동일",
      behavior_change=False, found_by=NEW, kind=u"보강")
p.fix("/specs[158]/open[0]/q",
      old="세 슬라이스가 라인별(Top/Mid/Bot)인지는 _gcbc 본문 미확인(콜리 내부 안 팜 · 지시 범위 밖)",
      new="★해소 — 세 슬라이스 = cache.top_minions[team] / mid_minions[team] / bottom_minions[team](tcxdict AbstractGameWithCache 0x10/0x50/0x90 · 각 [bumpalo Vec<&Entity>;2]) 를 이 순서로 chain. exe 디컴 0xe2a830(lane_minion.md L94~99: `+0x10/+0x28`·`+0x50/+0x68`·`+0x90/+0xa8`, uVar6=(1-team)*0x20)과 오라클(o158 case2: top 원소 → mid 원소 순으로 후보가 나옴) 으로 확정",
      evidence="tcxdict AbstractGameWithCache: 0x10 top_minions / 0x50 mid_minions / 0x90 bottom_minions [Vec;2](64B) · decomp/0.5.8/small_action/lane_minion.md L94~99 · o158.log case2 got=[(16,1003),(16,1004),(16,1001)](top,top,mid 순)",
      behavior_change=False, found_by=NEW, kind=u"보강")
p.fix("/specs[158]/open[2]/q",
      old="L84/87/90 의 사거리 합산 헬퍼가 effect.rs:1162 의 어떤 이름인지(인라인·define 없음)",
      new="★해소 — 헬퍼는 `Effect::range(&self, champ)`(effect.rs:25~26, pub · mir=1 · 인라인)이고 1162 는 option.rs(Option::map)다. closure$0/1/2(lane_minion.rs:84/87/90)가 `e.range(champ) + champ_radius` 를 돌려준다(u64)",
      evidence="m11.ll:139180 `!78129 = !DILocation(line: 26, scope: !78046, inlinedAt: !78050)` · 139097 `!78046 = … name: \"range\", linkageName: \"…Effect5range\", … line: 25` · 139095/139025 option.rs:1162",
      behavior_change=False, found_by=NEW, kind=u"보강")
p.fix("/specs[158]/open[5]/q",
      old="entity.rs:1482 가시성 헬퍼의 이름(인라인)",
      new="★해소 — `Entity::is_visible_from(&self, from:&Entity)`(entity.rs:1481~1483, pub · mir=1 · 인라인: from.team.player_team()(entity.rs:1136) 이 None(Neutral) 이면 true, 아니면 self.visible_state[t].is_visible()(data.rs:122)) — 가시성 헬퍼의 이름",
      evidence="m11.ll dloc !78249 `line 1136 in player_team ← line 1482 in is_visible_from ← line 93` · !78271 `line 122 in is_visible ← 1483 in is_visible_from ← 93` · tcx game_core::Entity::is_visible_from entity.rs:1481 pub mir=1",
      behavior_change=False, found_by=NEW, kind=u"보강")
for j, why in [(2, "case6/10: level 3 이면 Skill2 후보(태그 17) 생성, level 1 이면 skill2 None 으로 전무"),
               (3, "case9: champ move_speed 500 → 여유 15000 가산으로 50000 포함·50100 제외(case5 ms=1 대비)"),
               (4, "case3: 타라인 미니언 dist²=80000² 제외 · 79999² 포함(엄격 <) — mid/bottom 양쪽"),
               (5, "case2: ty 태그 1 + line==Top 미니언은 150000 밖에서도 후보 · 타라인은 80000 안(50000)만 후보"),
               (6, "case4: visible_state[0] 태그 1/2 미니언 제외, 0 만 후보 · case7: champ team 태그 1(Neutral) 이면 1/2 도 후보"),
               (8, "case1/2/5: Skill 후보 원소 +0xb1 == 16 · +8 == target id"),
               (9, "case6: Skill2 후보 원소 +0xb1 == 17")]:
    p.ev("/specs[158]/consts[%d]" % j, evidence=O158 + " — " + why, to=2, found_by=NEW)
for j, why in [(0, "case9 vs case5: 30*move_speed 만큼 후보 반경 확장"), (1, "case3 경계 80000/79999"), (2, "case6/10 레벨 게이트")]:
    p.ev("/specs[158]/knobs[%d]" % j, evidence=O158 + " — " + why, to=2, found_by=NEW)
for j in (5, 11, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30):
    p.ev("/specs[158]/mem[%d]" % j, evidence=O158 + " — 그 오프셋을 raw 로 써 넣은 값(가짜 미니언 필드 · champ 이속/레벨/Effect)이 후보 목록에 그대로 반영", to=3, found_by=NEW)

# ═════════════════════════ 159 battle_ally_action ═════════════════════════
O159 = "오라클 실행 확인(23차 배치D o159.rs · game_ai::battle_ally_action 직접 호출 · Effect 조립 raw write · 10/10 MATCH vs 독립 재구현 · 케이스당 프로세스 1개)"
G20 = "G20 R2(158↔159 skill2_effect.* 오프셋 충돌) 해소: 158·164 는 절대 오프셋(0x510/0x518/0x528/0x530) 을 쓰고 159·160 만 select 된 Effect 포인터 상대 오프셋을 offset 칸에 적었다 — 값이 틀린 게 아니라 표기 기준이 달랐다. 이름은 이미 「절대 0x5xx」를 병기하므로 offset 칸을 절대값으로 통일한다(chk 오귀속도 해소). IR: m15.ll:26091~26092 `%68 = gep %47, 1280` / `select … ptr %68, ptr @anon…58` → 26094 `gep %69, 48`(=+0x530)"
p.fix("/specs[159]/mem[13]/offset", old="0x30", new="0x530", evidence=G20 + " · 26094~26096 i32 tag", behavior_change=False, found_by=NEW, kind=u"실오류")
p.fix("/specs[159]/mem[14]/offset", old="0x10", new="0x510", evidence=G20 + " · 26513·26550 (%69+16)", behavior_change=False, found_by=NEW, kind=u"실오류")
p.fix("/specs[159]/mem[15]/offset", old="0x18", new="0x518", evidence=G20 + " · 26514·26551 (%69+24)", behavior_change=False, found_by=NEW, kind=u"실오류")
p.fix("/specs[159]/mem[16]/offset", old="0x28", new="0x528", evidence=G20 + " · 26512 (%69+40)", behavior_change=False, found_by=NEW, kind=u"실오류")
p.fix("/specs[159]/consts[5]/meaning",
      old="near_allies 필터: dist² < 150000² (엄격 <, +1 없음) — aux m15.ll:58275",
      new="near_allies 필터 임계: dist² < 150000² (엄격 <, +1 없음 — 오라클 o159 case3: 149999 포함 · 150000/150001 제외) — aux m15.ll:58275 `icmp ult i64 %30, 22500000000`",
      evidence="m15.ll:58275 `%31 = icmp ult i64 %30, 22500000000` · o159.log case3 got=[(16,19),(16,22)] (dx 149999·100000 만) — G15 낱말 `임계` 보강(aux 라 본문 창 관측 0 → 미상이었다)",
      behavior_change=False, found_by=NEW, kind=u"보강")
p.fix("/specs[159]/consts[15]/meaning",
      old="SmallActionPlay 원소 크기(memcpy) — stride",
      new="SmallActionPlay 원소 바이트 길이 184 (memcpy 길이 · 배열 stride) — tcxdict SmallActionPlay 184B",
      evidence="m15.ll:26352 `llvm.memcpy … i64 184` · tcxdict --enum SmallActionPlay (184B) — G15 낱말 `길이` 보강",
      behavior_change=False, found_by=NEW, kind=u"보강")
p.fix("/specs[159]/sig/returns",
      old="원소 안 live 바이트는 24B+태그 1B 뿐",
      new="원소 안 live 바이트는 **17B**([0x0,0x11): start_tick 8 · target 8 · is_act 1 — `SmallActionSkill::new`/`Skill2::new` sret 속성 `initializes((0, 17))` m07.ll:7645/12020) + 태그 1B(+0xb1) 뿐 — [0x11,0x18) 도 미기록",
      evidence="m07.ll:7645 `define void @…SmallActionSkill3new(ptr … sret([24 x i8]) … initializes((0, 17)) %0, …)` · 12020 `…SmallActionSkill23new … initializes((0, 17))`",
      behavior_change=False, found_by=NEW, kind=u"보강", force=True)
p.fix("/specs[159]/open[4]/q",
      old="소스 표기 `dist_sq < 150000*150000` 추정, 외연은 IR 대로",
      new="소스 표기 `dist_sq < 150000*150000` 추정(표기 불가) · ★외연은 오라클로 확정 — o159 case3: dx=149999 포함 · 150000·150001 제외(엄격 <)",
      evidence="o159.log case3 `got=[(16, 19), (16, 22)] model=[…] MATCH (near=2 dists=[149999, 100000])` — adx=[150000,149999,0,150001,100000]",
      behavior_change=False, found_by=NEW, kind=u"보강")
for j, why in [(3, "case4/5: level 3 이면 Skill2(17) 후보, level 1 이면 전무"),
               (5, "case3: 150000² 경계 엄격 < (149999 포함 · 150000 제외)"),
               (7, "case7/9: AttackEffect(expected_buff_deep None) → walk 30 · 30*move_speed(1000→30000) 만큼 사거리 확장(40000 포함)"),
               (10, "case1/2/6/7: Skill 후보 원소 +0xb1 == 16"), (11, "case4/6: Skill2 후보 원소 +0xb1 == 17"),
               (9, "case1~9: Entity::radius 인라인 값(champ 10000 + e 10000)이 사거리 합산에 반영(재구현 일치)")]:
    p.ev("/specs[159]/consts[%d]" % j, evidence=O159 + " — " + why, to=2, found_by=NEW)
for j, why in [(0, "case3 경계"), (1, "case7/9 walk 30"), (3, "case4/5 레벨 게이트")]:
    p.ev("/specs[159]/knobs[%d]" % j, evidence=O159 + " — " + why, to=2, found_by=NEW)
for j in (1, 2, 4, 5, 7, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 31, 32, 33, 34, 35):
    p.ev("/specs[159]/mem[%d]" % j, evidence=O159 + " — 그 오프셋을 raw 로 써 넣은 값(Effect 56B · level · move_speed · 아군 x/y)이 후보 목록에 그대로 반영 · 결과 원소를 +0xb1/+8 로 읽어 대조", to=3, found_by=NEW)

# ═════════════════════════ 160 get_die_tick_player ═════════════════════════
O160 = "오라클 실행 확인(23차 배치D o160.rs · #[link_name] 직접 호출 · ChampionCache per_sec/hp/좌표/이속 raw write · 10/10 MATCH vs 독립 재구현 · 케이스당 프로세스 1개)"
NONE160 = "★호출자 4곳 전부 action=None · tower=None 을 넘긴다: m08.ll:97512 `store i64 -1, ptr %27` → 97514 memcpy 24B → 97515 인자6 `%19` / 97528 `%27`(같은 -1) · m10.ll:12958 `store i64 -1, ptr %30` → 12961·12973. 타워 인자는 4곳 모두 `dereferenceable_or_null(1728) null`. ⟹ Option<SmallAction> None 태그 = i64 -1(0xFFFF…, 니치 start-1)이지 11 이 아니며, RunAway 이속 차감 분기와 타워 dps 분기(div-by-zero 포함)는 r14 game_ai 호출자로는 죽은 코드다(SDK 직접 호출로는 실행됨: o160 case2/9 RunAway · case8 tower)"
p.fix("/specs[160]/sig/params[5]/role",
      old="클로저가 캡처(52125). 태그==0(Some(RunAway), SmallAction Direct 태그 0=RunAway) 이면 적 이속에서 내 이속을 뺀다(aux m01.ll:51602~51604·51714~51717)",
      new="클로저가 캡처(52125). 태그==0(Some(RunAway), SmallAction Direct 태그 0=RunAway) 이면 적 이속에서 내 이속을 뺀다(aux m01.ll:51602~51604·51714~51717). None 태그 = i64 -1(호출자 m08.ll:97512·m10.ll:12958 `store i64 -1`). " + NONE160,
      evidence="m08.ll:97512 `store i64 -1, ptr %27, align 8, !dbg !58911` · 97514 `llvm.memcpy(… %19, … %27, i64 24)` · 97515 `…get_die_tick_player(… dereferenceable_or_null(1728) null, … dereferenceable(24) %19)` · m10.ll:12958·12961 동일 · o160.log case2 got=450(RunAway 300-200=100 이속) case9 got=30033(sat 0→max 1)",
      behavior_change=False, found_by=NEW, kind=u"보강")
p.fix("/specs[160]/sig/params[4]/role",
      old="null=None(52168). Some 이면 dps_list 에 (0, 타워dps) 추가",
      new="null=None(52168). Some 이면 dps_list 에 (0, 타워dps) 추가 — r14 호출자 4곳(m08.ll:97515·97528 / m10.ll:12961·12973) 모두 null 을 넘겨 실전 미도달(SDK 직접 호출 o160 case8 로는 실행됨: 타워 attack_cooltime≠0 · dps 0 → (0,0) push)",
      evidence="m08.ll:97515/97528 · m10.ll:12961/12973 인자5 `ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable_or_null(1728) null` · o160.log case8 events=[(0, 0), (100, 1000), …] got=205 MATCH",
      behavior_change=False, found_by=NEW, kind=u"보강")
p.fix("/specs[160]/mem[18]/offset", old="0x30", new="0x530",
      evidence="G20 R2 계열(159 와 같은 상대/절대 표기 불일치): m01.ll:51491 select 된 Effect ptr +48 = Entity+0x500+0x30 = 0x530. 이름에 이미 「절대 0x530」 병기 — offset 칸을 절대값으로 통일(chk 오귀속 해소)",
      behavior_change=False, found_by=NEW, kind=u"실오류")
p.fix("/specs[160]/open[0]/q",
      old="니치 규칙상 None=11 이고, 클로저의 `tag==0` 비교는 Some(RunAway) 로 읽었다(추정 · 검증법: 호출자 m08.ll:97515/m10.ll:12961 의 action 인자 구성 추적)",
      new="★해소 — 호출자 추적 결과 None 태그 = **i64 -1**(11 이 아니다: m08.ll:97512 `store i64 -1, ptr %27` → 24B memcpy → 인자6 · m10.ll:12958 동일). `tag==0` = Some(RunAway) 는 오라클로 확인(o160 case2: Some(RunAway) 전달 시 이속 300-200 반영 → 450). 4 호출자 전부 None·tower null 이라 두 분기는 r14 에서 죽은 코드",
      evidence="m08.ll:97512~97515 · m10.ll:12958~12961 · o160.log case2/9",
      behavior_change=False, found_by=NEW, kind=u"보강")
p.fix("/specs[160]/open[2]/q",
      old="죽은 분기 가능성",
      new="죽은 분기 — 사실 서술: 센티넬 (9999999999,0) 은 무조건 push(52515~52520)되고 정렬 후 마지막이라 루프가 끝까지 돌면 tick=9999999999≠0, 중간에 죽으면 %247 경로라 52615 `icmp eq %235, 0` 은 참이 될 수 없다(len≥1 · 52577 `icmp eq %211, 0` 도 거짓). 오라클 case0/7(dps 0 · 적 없음) 반환 9999999999 는 센티넬 경로",
      evidence="m09.ll:52515 `store i64 9999999999, ptr %198` 무조건 · 52616 `select i1 %237, i64 9999999999, i64 %235` · o160.log case0/7 got=9999999999",
      behavior_change=False, found_by=NEW, kind=u"보강")
p.fix("/specs[160]/open[4]/q",
      old="비대칭은 IR 사실",
      new="비대칭은 IR 사실 — 단 r14 호출자 4곳이 tower=null 을 넘겨 타워 경로 자체가 실전 미도달(m08.ll:97515·97528 / m10.ll:12961·12973). SDK 직접 호출(o160 case8)에선 start_game 타워의 attack_cooltime≠0 이라 패닉 없이 (0,0) push",
      evidence="m08.ll:97515 `dereferenceable_or_null(1728) null` ×4 · o160.log case8 MATCH",
      behavior_change=False, found_by=NEW, kind=u"보강")
p.fix("/specs[160]/logic",
      old="if total + add_max >= hp {                 // L642 (icmp ult total+add_max, hp 의 부정)",
      new="if total + add_max >= hp {                 // L642 (icmp ult total+add_max, hp 의 부정) ★오라클 경계 확정: o160 case5/6 에서 total+add_max == hp*100 인 원소에서 죽음(670) — `>` 였다면 402",
      evidence="o160.log case5 `got=670 model=670 MATCH (alt_if_gt=402 hp=670 events=[(0,100),(0,0),(0,0),(402,50),…])` · case6(enemies 순서 반전) 동일 670",
      behavior_change=False, found_by=NEW, kind=u"보강")
for j, why in [(2, "case1~9: hp*100 · dps*100 스케일이 재구현과 일치(case4 1497)"),
               (3, "case1~9: /60 환산(case1: dt 100·dps 1000 → 죽음 205)"),
               (4, "case0/7: 죽지 않으면 9999999999"),
               (5, "case8: 타워 도달틱 0 · 초기 dps/total/tick 0"),
               (6, "case9: 이속 sat 0 → umax 1 · case1~9 umax(dps,1)"),
               (11, "case1~9: 적당 3원소(공/스킬/스킬2) 도달틱 — events 목록과 일치"),
               (12, "case1~9: attack/skill Some · skill2 는 level≤2 라 None(range 0)")]:
    p.ev("/specs[160]/consts[%d]" % j, evidence=O160 + " — " + why, to=2, found_by=NEW)
for j, why in [(0, "case0/7 센티넬 반환"), (1, "case1~9 /60"), (2, "case1~9 hp*100")]:
    p.ev("/specs[160]/knobs[%d]" % j, evidence=O160 + " — " + why, to=2, found_by=NEW)
for j in (0, 1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 19, 20, 21, 22, 23, 24, 29):
    p.ev("/specs[160]/mem[%d]" % j, evidence=O160 + " — 그 오프셋(ChampionCache +0x190/+0x1b8/+0x1e0 · Entity 이속/좌표/hp · Option<SmallAction> 태그)을 raw 로 써 넣은 값이 죽는 틱에 그대로 반영", to=3, found_by=NEW)

# ═════════════════════════ 161 lane_minion_position_action ═════════════════════════
O161 = "오라클 실행 확인(23차 배치D o161.rs · game_ai::lane_minion_position_action 직접 호출 · 가짜 미니언 raw 주입 · None 조건 4곳 4/4 MATCH · 케이스당 프로세스 1개)"
EXE161 = "★exe 0xe2a830 디컴(decomp/0.5.8/small_action/lane_minion.md L27~L190) 대응표: 인자 10 = IR 10(sret, version, rnd, data, player, line, positioning_score, wave_snapshot, end_delay, purpose · argscan 스택 6+레지스터 4). 콜리 exe: has_current_explicit_minion_action = FUN_140e266e0(param_4 data, param_5 player, param_6 line, local_190 target) **4인자 — version 이 DeadArgElim 으로 삭제**(IR m11.ll:45591 은 5인자 `i64 %0` 첫 슬롯; 안에서 %0 은 is_enemy_well_danger/is_unnecessary_enemy_tower_position 에만 전달되는데 exe 쪽 콜리 FUN_141453260(3인자)·FUN_140d97300(5인자: 캐시·player·x·y) 도 version 을 안 받는다). choose_goal = FUN_140e248f0(sret, target_score, purpose, version, player, data, champ, target, *(positioning_score+0xab8)=cx, *(+0xac0)=cy, line) **11인자 — &PositioningScoreData 가 ArgumentPromotion 으로 cx/cy 두 스칼라로 분해**(IR 은 ptr 1슬롯). target_score = FUN_140d6fc40(sret, iter, ctx, cap) 클로저 경유"
p.fix("/specs[161]/sig/params[1]/role",
      old="본 함수 분기 없음. has_current_explicit_minion_action·choose_goal 에 전달(스택 %20 경유 · 클로저 캡처 &version 은 target_score 가 안 읽어 poison 전달됨)",
      new="본 함수 분기 없음. has_current_explicit_minion_action·choose_goal 에 전달(스택 %20 경유 · 클로저 캡처 &version 은 target_score 가 안 읽어 poison 전달됨). ★exe 에선 has_current_explicit_minion_action(0xe266e0) 호출에 version 이 **빠진다**(DeadArgElim · 4인자) — choose_goal(0xe248f0) 에는 4번째 인자로 그대로 전달. " + EXE161,
      evidence="decomp lane_minion.md L169 `cVar5 = FUN_140e266e0(param_4,param_5,param_6,local_190)` · L725 `bool FUN_140e266e0(longlong *param_1,longlong param_2,char param_3,longlong *param_4)` · L180~181 `FUN_140e248f0(&local_e0,local_58,param_10,local_50,param_5,param_4,lVar2,local_190,*(param_7+0xab8),*(param_7+0xac0),param_6)` · m11.ll:45591 IR define 5인자 · 45713/45717 %0 전달 · argscan 0xe2a830 스택 6 = exe 10",
      behavior_change=False, found_by=NEW, kind=u"보강")
p.fix("/specs[161]/sig/params[6]/role",
      old="readonly. choose_goal 에만 전달(54491~54492)",
      new="readonly. choose_goal 에만 전달(54491~54492). ★exe 는 이 포인터 대신 **+0xab8 cx · +0xac0 cy(tcxdict PositioningScoreData 0xab8/0xac0 usize) 두 스칼라**를 choose_goal(0xe248f0) 9·10번째 인자로 넘긴다(ArgumentPromotion) — choose_goal 이 읽는 PositioningScoreData 필드는 이 둘뿐이라는 뜻(exe 기준)",
      evidence="decomp lane_minion.md L180~181 `*(undefined4 *)(param_7 + 0xab8),*(undefined4 *)(param_7 + 0xac0)` · tcxdict PositioningScoreData 0xab8 cx / 0xac0 cy",
      behavior_change=False, found_by=NEW, kind=u"보강")
p.fix("/specs[161]/sig/params[0]/role",
      old="+0x75 path_finder@tag(=2 None · 54486)",
      new="+0x75 path_finder@tag(=2 None · 54486 — Option<PathFinder> 72B @+0x30, 안에 Box<[(u64,u64);70]> 1120B(+0x28)·Box<[u8;70]> 70B(+0x30); 이 함수의 PathFinder 지점 = **생성 None 1곳(54486)** 뿐이고 drop_glue 는 choose_goal unwind 정리(54504 `drop_glue…SmallActionLaneMinionPosition` funclet)에서만 · 정상 경로 재생성/drop 없음 — HEAP_SUBST 대상 아님)",
      evidence="m11.ll:54485~54486 `%93 = gep %16, 117` / `store i8 2, ptr %93` · 54504 `call fastcc void @…drop_glue…SmallActionLaneMinionPositionEBH_(ptr … %16) #32 [ \"funclet\"(token %101) ]` · tcxdict SmallActionLaneMinionPosition 0x30 path_finder Option<PathFinder>(72B) · PathFinder 0x28 path Box<[(u64,u64);70]> / 0x30 planned_verdict Box<[u8;70]>",
      behavior_change=False, found_by=NEW, kind=u"보강")
p.fix("/specs[161]/consts[5]/meaning",
      old="[aux closure$0] follow_range 여유 = 2셀(32000*2).",
      new="[aux closure$0] follow_range 여유치(가산 마진) = 2셀(32000*2).",
      evidence="m11.ll:40814 `%74 = add i64 %34, 64000` — 배율이 아니라 가산 바이어스(G15 낱말 `여유치`/`마진` → 오프셋가감)",
      behavior_change=False, found_by=NEW, kind=u"보강")
p.fix("/specs[161]/consts[8]/meaning",
      old="[aux] VisibleState 0 = Visible(40718) / TeamType 0 = Player(40701 trunc)",
      new="[aux] 태그 0: VisibleState 0 = Visible(40718) / TeamType 0 = Player(40701 trunc)",
      evidence="m11.ll:40718 `icmp eq i64 %16, 0` · 40702 `trunc nuw i64 %7 to i1` — G15 낱말 `태그` 보강(aux 라 본문 창 관측 0)",
      behavior_change=False, found_by=NEW, kind=u"보강")
p.fix("/specs[161]/open[3]/q",
      old="iter_minions 56B 체인 구조(슬라이스 3개)의 의미 — 콜리 내부 안 팜",
      new="★해소 — 슬라이스 3개 = cache.top_minions[team]/mid_minions[team]/bottom_minions[team](tcxdict AbstractGameWithCache 0x10/0x50/0x90) 순 chain(exe 디컴 0xe2a830 L94~99 · o158 case2 순서 관측)",
      evidence="tcxdict AbstractGameWithCache 0x10 top_minions / 0x50 mid_minions / 0x90 bottom_minions · decomp lane_minion.md L94~99",
      behavior_change=False, found_by=NEW, kind=u"보강")
for j, why in [(0, "case0: Support 포지션 → None(0xff)"), (1, "case0~3: None 경로 +0xb1 = 0xff")]:
    p.ev("/specs[161]/consts[%d]" % j, evidence=O161 + " — " + why, to=2, found_by=NEW)
p.ev("/specs[161]/knobs[1]", evidence=O161 + " — case0 Support → None", to=2, found_by=NEW)
for j in (0, 1, 3, 6):
    p.ev("/specs[161]/mem[%d]" % j, evidence=O161 + " — position(0x9c0)==4 · attack_effect 태그(0x4c0)=-1 · 미니언 0 → 각각 None 확인", to=3, found_by=NEW)

# ═════════════════════════ 162 v57_summon_command_score ═════════════════════════
O162 = "오라클 실행 확인(23차 배치D o162.rs · #[link_name] 직접 호출(define hidden) · ChampionInfo::ult/skill/skill2 실물 Box<dyn Action> · 구울 raw 주입 · tick 0/1000 · 17/17 MATCH · 케이스당 프로세스 1개)"
p.fix("/specs[162]/open[3]/q",
      old="Effect::expected_damage_target / Entity::attack_cooltime / Blackboard::is_recent_visible 내부는 game_core 경계 — 시그니처만 기록(tcx: fn(&Effect,&GameContext,&dyn AbstractEntity,&Entity)->usize / fn(&Entity)->usize / fn(&Blackboard,&dyn AbstractGame,&PlayerState,&Entity)->bool)",
      new="Effect::expected_damage_target / Entity::attack_cooltime 내부는 game_core 경계 — 시그니처만 기록(tcx: fn(&Effect,&GameContext,&dyn AbstractEntity,&Entity)->usize / fn(&Entity)->usize). (is_recent_visible 은 callees 확정 항목이라 여기서 뺌 — 본문 g07.ll:157005: game.is_visible(team, c.id)(vtable+248) || blackboard.last_seen[c.pos](+480+pos*8)+120 >= game.tick(); 오라클 tick 0 → true · tick 1000 → false 로 실측)",
      evidence="G7 과열림 해소. g07.ll:157011~157047 `%12 = call i1 %11(game, team, id)` / `%25 = add %24, 120` / `%29 = icmp uge %25, tick` / phi[true,%29,false] · o162.log case2(tick0 → 10) vs case3(tick1000 → 5) · case13(90) vs case15(30)",
      behavior_change=False, found_by=NEW, kind=u"보강")
for j, why in [(0, "case0~3: IllusionistUltAction 분기 선택"), (1, "case4/5: NecromancerSkillAction 분기"), (2, "case6~11/16: NecromancerSkill2Action 분기"), (3, "case12/13/15: NecromancerUltAction 분기"),
               (4, "case0: t 적 → Some(0) · case9/16: 구울>0 & t 아군 → Some(0)"),
               (8, "case1~3: attack 기대피해 0 → dps 0 → 0/40 → smax 10 (하한 클램프)"),
               (9, "case13: 네크로 궁 근처 가시 적 → 90"), (10, "case1/3: 근처 가시 적 없음 → 5"),
               (11, "case5: 25"), (12, "case4: 8"), (14, "case10 vs 7: ty 태그 7 만 구울로 세고 2(Tower) 는 무시"),
               (15, "case6/10: 구울 0 → -100"), (16, "case7: 2*18=36 · case11: 3*18=54"), (17, "case8: 4*18=72 → smin 60"),
               (18, "case12/15: 30"), (19, "case2 vs 1: 적을 around+1000 으로 옮기면 근처 판정(원 위치 d²≈1.6e12 는 밖)")]:
    p.ev("/specs[162]/consts[%d]" % j, evidence=O162 + " — " + why, to=2, found_by=NEW)
for j, why in [(0, "case1/3 → 5"), (2, "case4/5 8/25"), (3, "case6/10 -100"), (4, "case7/8/11 18·60"), (5, "case12/13/15 30/90"), (6, "case1 vs 2 근처 판정")]:
    p.ev("/specs[162]/knobs[%d]" % j, evidence=O162 + " — " + why, to=2, found_by=NEW)
for j in (0, 2, 3, 4, 5, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18):
    p.ev("/specs[162]/mem[%d]" % j, evidence=O162 + " — Box<dyn Action> vtable as_any/type_id 4종 분기 · team 태그/idx 비교 · others[team] ptr/len raw 주입 · 구울 ty 태그 · 적 챔프 x/y · blackboard(tick) 이 결과에 그대로 반영", to=3, found_by=NEW)

# ═════════════════════════ 163 / 164 abstract_input::skill / skill2 ═════════════════════════
O163 = "오라클 실행 확인(23차 배치D o163.rs · game_ai::skill/skill2 직접 호출 · Effect 조립 raw write(+0x4c8/+0x500) · 15/15 MATCH vs 독립 재구현(접근 지점 = isqrt·margin·adjust_position·safe_move(pub)) · 케이스당 프로세스 1개)"
SM = " ③접근 경로 = safe_move_avoiding_enemy_well 의 sret 32B 그대로 — 그 콜리도 +0x0 태그 0(Move)+0x8 x+0x10 y 24B(m04.ll:43868~43872·43906~43910·43958~43962) 또는 -1 8B(43918·43954)만 기록하므로 Option<Input> 32B 중 [0x18,0x20) 이 live 인 경우는 Skill/Skill2 의 InputTarget Dir/Pos 뿐"
p.fix("/specs[163]/sig/returns", old="③접근 경로 = safe_move_avoiding_enemy_well 의 sret 32B 그대로", new=SM.strip(),
      evidence="m04.ll:43868 `store i64 0, ptr %0` / 43870 `store i64 %262, ptr %0+8` / 43872 `store i64 %263, ptr %0+16` · 43918/43954 `store i64 -1, ptr %0` · 43958~43962 동일 3연 store (sret.py 전수)",
      behavior_change=False, found_by=NEW, kind=u"보강", force=True)
p.fix("/specs[164]/sig/returns", old="③접근 경로 = safe_move_avoiding_enemy_well 의 sret 32B 그대로", new=SM.strip(),
      evidence="m04.ll:43868~43872 · 43906~43910 · 43918 · 43954 · 43958~43962 (sret.py 전수 · 163 과 동일 콜리)",
      behavior_change=False, found_by=NEW, kind=u"보강", force=True)
p.fix("/specs[163]/open[3]/q",
      old="이 함수 범위 밖",
      new="이 함수 범위 밖 — 분기 자체는 오라클로 실행됨(o163 case6: champ team 태그를 1(Neutral)로 두면 비가시 대상에도 접근 지점 경로 · 모델 일치). 실전 도달성(챔피언 Neutral 여부)은 여전히 미탐색",
      evidence="o163.log case6 `got=Move(805409,126575) model=Move(805409,126575) MATCH (… vis_tag=2 team_tag=1)` vs case2(team_tag 0 · vis 2 → Move(712306,79051))",
      behavior_change=False, found_by=NEW, kind=u"보강")
for j, why in [(0, "case0/1/9/12: None 경로 태그 -1"), (1, "case5 vs 3: (ty&14)==2 타워 → margin 2000"), (2, "case7: 가까운 적 → Input::Skill(태그 3) Target(id)"),
               (3, "case5 vs 3: 타워/챔피언 마스크 분기"), (4, "case5: 2000"), (5, "case3/8/14: 15000"),
               (6, "case2 vs 3: visible_state 0 만 접근 지점 · case4: casting Position → 반지름 미가산"), (7, "case3~6: radius 인라인 값 반영"), (8, "case8: level 3 → growth*(level-1)=2000 반영")]:
    p.ev("/specs[163]/consts[%d]" % j, evidence=O163 + " — " + why, to=2, found_by=NEW)
for j, why in [(0, "case5 타워 2000"), (1, "case3 15000")]:
    p.ev("/specs[163]/knobs[%d]" % j, evidence=O163 + " — " + why, to=2, found_by=NEW)
for j in (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23):
    p.ev("/specs[163]/mem[%d]" % j, evidence=O163 + " — Effect(+0x4c8 raw) · level · team 태그 · visible_state · ty · x/y · sret 태그/페이로드를 raw 로 쓰고 읽어 대조", to=3, found_by=NEW)
for j, why in [(0, "case9/12: None 태그 -1"), (1, "case9(level 1 → None) vs 10(level 3 → 접근 지점): `> 2` 게이트"), (2, "case13: Input::Skill2(태그 4)"),
               (3, "case11 타워 vs 10 챔피언"), (4, "case11(비가시 타워 → Move(tower.x,y)) · 가시 타워 margin 은 163 case5 와 동형"), (5, "case10: 15000"),
               (6, "case10/11: visible_state 0/2 분기"), (7, "case10: radius"), (8, "case10: level 3 growth 반영")]:
    p.ev("/specs[164]/consts[%d]" % j, evidence=O163 + " — " + why, to=2, found_by=NEW)
for j, why in [(0, "case11 타워"), (1, "case10 15000"), (2, "case9 vs 10 레벨 게이트")]:
    p.ev("/specs[164]/knobs[%d]" % j, evidence=O163 + " — " + why, to=2, found_by=NEW)
for j in (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23):
    p.ev("/specs[164]/mem[%d]" % j, evidence=O163 + " — Effect(+0x500 raw) · level(+0x5c8) · visible_state · ty · x/y · sret 태그/페이로드를 raw 로 쓰고 읽어 대조", to=3, found_by=NEW)

# ═════════════════════════ brief_errors ═════════════════════════
p.brief_error(u"§1 「`vis` 가 `pub` 이 아니면 오라클 직접 진입이 막힌다」는 162(in:game_ai) 에 대해 낡았다 — IR 이 `define hidden`(m10.ll:38354) 이라 `extern \"Rust\" #[link_name]` 로 직접 링크·실행됐다(17/17). 22차 C·D 의 범위 반전(METHOD_MAP ⑥)이 도시에 §1 문구에 반영되지 않았다")
p.brief_error(u"§4 G20 R2(158↔159 skill2_effect.range/growth_range/target) 는 「값이 틀렸거나 이름이 두 자리」가 아니라 **표기 기준 불일치**(159·160 은 select 된 Effect 포인터 상대 +0x10/+0x18/+0x28/+0x30, 158·164 는 Entity 절대 0x510~0x530)다 — 게이트 메시지가 세 번째 원인(상대/절대 혼용)을 제시하지 않아 오독을 유도한다. 검사기는 offset 칸이 `name` 의 「절대 0x…」 병기와 다를 때 별도 경고를 내야 한다")
p.brief_error(u"지시문 ⑥의 internal fastcc 대응표 대상(137·144·152·155·167·169/170·171)은 배치 D 범위(158~164) 밖이다 — 144(has_current_explicit_minion_action) 만 161 의 콜리라 호출자 exe 디컴(0xe2a830)으로 대응표를 냈다(version DeadArgElim → exe 4인자). 나머지는 담당 배치 몫으로 남긴다")
p.brief_error(u"§3 표의 `sig.params` 는 v3 `role` 칸인데 지시문 ②는 「signature.abi 에 기재」라 한다 — v3 sig 에 `abi` 가 없어(22D 와 같이) `ret`/`role` 로만 patch 를 냈고 exe 대응표는 params[1]/[6].role 에 실었다(force 없이 통과)")
p.brief_error(u"§1 ev≥4 열의 「knobs 상당수가 인라인된 다른 함수의 줄」 경고와 달리 이 7함수 knobs 22행은 전부 본 함수에서 닿았다(오라클로 20행 ev2 요청) — 표적 수를 낮춰 잡을 필요가 없었다")

p.brief_error(u"도구 결함(applypatch.py): v3 경로 `/specs[i]/sig/ret` 는 스칼라 분기(① `spec.get(field)`)에서 V2KEY(ret→returns) 매핑을 안 타 「필드가 문자열이 아니고 문면도 못 찾음」으로 3건 실패했다(22차 주석 「배치 A·B 가 force 로 우회」와 같은 자리). 이번엔 v2 키 `/sig/returns` 로 직접 쓰고 mkpatch 쪽은 force=True 로 냈다 — 스칼라 분기에도 V2KEY 를 적용하면 닫힌다")
p.save()
