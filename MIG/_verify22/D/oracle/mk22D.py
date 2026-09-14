# -*- coding: utf-8 -*-
"""22차 배치D patch.json 생성 (mkpatch 참조구현 사용)."""
import sys, io, json
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import mkpatch

p = mkpatch.Patch(round=22, batch="D")

# ───────────────────────── 129 should_add_self_etc_buff_action ─────────────────────────
# G16 P3 — 승격 인자 2행 → 규약(i=소스 위치, 팻포인터 1행)대로 1행으로 합친다(&dyn 타입이면 ir_slots=2 로 IR 5슬롯과 정합)
p.fix("/specs[129]/sig/params[3]/name",
      old="effect.ty.data_ptr",
      new="effect (승격: ty=Arc<dyn EffectType> → %3 ArcInner ptr · %4 vtable ptr)",
      evidence="m15.ll:34622 define `…dereferenceable(1728) %2, ptr %3, ptr readonly captures(address_is_null) %4)` — 소스 인자 4(&Effect 56B)가 IR 에서 (ArcInner ptr, vtable ptr) 2슬롯. paramrole P3 규약 = i 는 소스 위치·팻포인터도 1행 ⟹ 5행이 아니라 4행(&dyn 타입 표기로 ir_slots=2)",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix("/specs[129]/sig/params[3]/type",
      old="ptr (Arc<dyn EffectType> ArcInner*)",
      new="&dyn EffectType (팻포인터 2슬롯: %3 = Arc<dyn EffectType> ArcInner ptr · %4 = EffectType vtable ptr — 소스 &Effect(56B) 중 ty 만 ArgumentPromotion)",
      evidence="m15.ll:34622 `ptr %3, ptr readonly captures(address_is_null) %4` · 34643~34648 `%12 = gep %4, 16 / %13 = load i64 !range / %15 = and (%13-1), -16 / %16 = gep %3, %15 / %17 = gep %16, 16`(ArcInner 데이터 = &dyn self) · 34649~34651 `%18 = gep %4, 144 / %19 = load ptr / %20 = call i1 %19(%17)`",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix("/specs[129]/sig/params[3]/role",
      old="ArcInner 데이터 오프셋 = round_up(16, align)(34643~34648) → &dyn EffectType self",
      new="%3 = ArcInner ptr(34639 `icmp ne ptr %3, null` → llvm.assume) · %4 = vtable ptr(readonly captures(address_is_null), 34641 assume). vtable+0x10 align → ArcInner 데이터 오프셋 = round_up(16, align)(34643~34648) → &dyn EffectType self · vtable+0x90(144) = etc_buff 호출(34649~34651). ★exe(0xebcbd0)는 %4 를 한 번 더 승격해 인자 6개(rcx player, rdx data, r8 champ, r9 ArcInner ptr, [rsp+0x20] param_5 = *(vtable+0x10) align, [rsp+0x28] param_6 = *(vtable+0x90) etc_buff fn ptr)",
      evidence="argscan 0xebcbd0: entry_rsp+0x20 `mov rax,[rsp+0x180]`(align) · +0x28 `call [rsp+0x188]`(fn ptr) ⟹ 스택 2+레지스터 4 = 6 · decomp fight_check.md:1454 `FUN_140ebcbd0(param_1..param_4, longlong param_5, code *param_6)` L1497 `(*param_6)(param_4 + (param_5 - 1U & 0xfffffffffffffff0) + 0x10)` · 호출자 L7487 `FUN_140ebcbd0(lStack_a0,param_5,puVar2,puVar2[0x99],*(puVar2[0x9a]+0x10),*(puVar2[0x9a]+0x90))`",
      behavior_change=False, found_by="new", kind=u"보강")
p.errors.append({"op": "delete", "path": "/specs[129]/sig/params[4]", "guard": "effect.ty.vtable_ptr",
                 "kind": u"실오류", "old": "effect.ty.vtable_ptr", "new": "(params[3] 로 병합)",
                 "evidence": "G16 P3: sig.tcx 소스 인자 4 ⟹ 4행. 승격 2슬롯은 params[3] 한 행(&dyn 타입, ir_slots=2)에 %3/%4 로 기재 — 위 3건 정정에 내용 전부 이관",
                 "behavior_change": False, "found_by": "new"})
p.fix("/specs[129]/sig/abi",
      old="exe 호출자 대조 시 인자 5개(player, data, champ, ty_ptr, ty_vtable)로 봐야 함",
      new="exe(0xebcbd0)는 %4(vtable ptr)를 한 번 더 승격해 **인자 6개**로 받는다: rcx player · rdx data · r8 champ · r9 ArcInner ptr · [rsp+0x20] *(vtable+0x10)=align(i64) · [rsp+0x28] *(vtable+0x90)=etc_buff fn ptr. sweep 편입 시 IR 5슬롯 ↔ exe 6슬롯 대응표는 params[3] role 참조",
      evidence="argscan.py 0xebcbd0 → `스택 인자 2개 + 레지스터 4 = exe 인자 6` · decomp/0.5.8/fight_check.md:1454~1497 `bool FUN_140ebcbd0(longlong param_1,longlong *param_2,longlong param_3,longlong param_4,longlong param_5,code *param_6)` / `cVar7 = (*param_6)(param_4 + (param_5 - 1U & 0xfffffffffffffff0) + 0x10)` · 호출자 2곳(L7487·L7617) 모두 `*(vtable+0x10), *(vtable+0x90)` 전달",
      behavior_change=False, found_by="new", kind=u"실오류", force=True)   # v3 sig 에 abi 가 없어 mkpatch 조회 불가 — v2 signature.abi 에 실재(v2q 확인)
# G18 — logic 이 인용한 vtable+0x90(및 +0x10) 을 mem 표에 넣는다
p.errors.append({"op": "insert", "path": "/specs[129]/mem", "at": 16, "guard": "etc_buff (vtable 슬롯 +0x90)", "guard_key": "name",
                 "new": {"base": "vtable(EffectType)", "offset": "0x90", "name": "etc_buff (vtable 슬롯 +0x90)",
                         "note": "m15.ll:34649~34651 `%18 = gep %4, 144` → `%19 = load ptr, !invariant.load` → `%20 = call i1 %19(ptr %17)` — L609 게이트. divtable EffectType 0x90 = etc_buff(tcx 메서드 idx14). 예: HitmanUltEffect 구현은 `ret i1 true`(g10.ll:313166~313168). exe 에선 이 로드가 호출자로 승격돼 param_6 로 들어온다"},
                 "kind": u"보강", "old": None,
                 "evidence": "G18: logic `// L609 vtable+0x90 (34649~34652 → %314 true)` 인용 오프셋이 mem 에 없었다. m15.ll:34649 `%18 = getelementptr inbounds nuw i8, ptr %4, i64 144`",
                 "behavior_change": False, "found_by": "new"})
p.errors.append({"op": "insert", "path": "/specs[129]/mem", "at": 17, "guard": "align (vtable 슬롯 +0x10)", "guard_key": "name",
                 "new": {"base": "vtable(EffectType)", "offset": "0x10", "name": "align (vtable 슬롯 +0x10)",
                         "note": "m15.ll:34643~34648 `%13 = load i64, ptr %12(=%4+16), !range !16440, !invariant.load` → `(align-1) & -16` → ArcInner 데이터 오프셋 → +16 = &dyn EffectType self(%17). exe 에선 호출자가 `*(vtable+0x10)` 을 param_5 로 넘긴다"},
                 "kind": u"보강", "old": None,
                 "evidence": "m15.ll:34643 `%12 = getelementptr inbounds nuw i8, ptr %4, i64 16` · 34644 `%13 = load i64, ptr %12, align 8, !range !16440, !invariant.load !8` · 34645~34648",
                 "behavior_change": False, "found_by": "new"})
# G15 — consts[7] kind: 낱말 `임계` 를 빼고 `길이` 로(부정문 가드 뒤 규칙③이 CMP_ORD 로 되돌리던 자리)
p.fix("/specs[129]/consts[7]/meaning",
      old="팀 배열 bounds(34737) — 임계 아님",
      new="팀 배열 길이 2 — player_champion[1-team] bounds 검사(icmp ult 34737 → panic_bounds_check 인자 35150). 판정 상수가 아니라 배열 길이",
      evidence="m15.ll:34737 `%65 = icmp ult i64 %64, 2` · 35150 `panic_bounds_check(i64 noundef %64, i64 noundef 2, …)`. mkspec3._kind: 낱말 `임계`(부정문)→neg_hit→None→규칙③ CMP_ORD→`임계` 로 되돌아감. `길이` 낱말 + CALLARG 관측(panic_bounds_check 인자)으로 규칙② 구제",
      behavior_change=False, found_by="reused", kind=u"실오류")

p.errors.append({"path": "/specs[129]/consts[7]/kind", "kind": u"실오류", "old": None, "new": u"길이",
                 "evidence": "m15.ll:34737 `%65 = icmp ult i64 %64, 2`(bounds) · 35150 `panic_bounds_check(i64 noundef %64, i64 noundef 2, …)`. kindchk.observe 가 이 창에서 CMP_ORD 만 보고 CALLARG(35150)를 못 잡아 낱말 `길이` 가 규칙②로 구제되지 않는다(시뮬레이션: word=길이·kind=임계) ⟹ 19차 규칙(v2 행 kind 명시 우선)으로 확정",
                 "behavior_change": False, "found_by": "new"})

# ───────────────────────── 130 v55_mark_value ─────────────────────────
p.fix("/specs[130]/mem[3]/note",
      old="sret 48B = Option<MarkProfile>: tag i64@0(1=Some,0=None)",
      new="sret 48B = Option<MarkProfile>: tag i64@0(1=Some,0=None) ★살아있는 바이트: None 경로(기본 impl, g02.ll:309685 등 `initializes((0, 8))`)는 태그 8B 만 store 하고 +8..+48 은 undef · Some 경로(HitmanUltEffect g10.ll:313142 `initializes((0, 48))`)는 48B 전부 store(+8 base=damage · +16 accum_pct=damage_ratio · +24..+40 memset 0 = per_hit·max_hits · +40 window=mark_duration). 본 함수는 태그(33101 `!range`)를 본 뒤에만 +8..+40 을 읽는다",
      evidence="g10.ll:313142 `define internal void @…HitmanUltEffect…expected_mark(ptr … sret([48 x i8]) … initializes((0, 48)) %0, …)` 313154~313161 store ×3 + memset(16) + `store i64 1, ptr %0` · g02.ll:309685 `…AddCastedEffect…expected_mark…initializes((0, 8))` · m10.ll:33101 `%29 = load i64, ptr %15, align 8, !dbg !40197, !range !15865` → 33102 trunc → 33103 br",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[130]/one_line",
      old="hp_value 환산, 0..160",
      new="hp_value 환산, 상한 160 만(smin — 하한 클램프 없음: hp_value<0 이면 음수 그대로 반환)",
      evidence="m10.ll:33295 `%125 = call noundef i64 @llvm.smin.i64(i64 %124, i64 160)` 뒤 smax 없음 · 오라클 o130 case10 hp_value=-10 → got=-10(model 일치)",
      behavior_change=False, found_by="new", kind=u"보강")
ev130 = "오라클 실행 확인(22차 배치D o130.rs · #[link_name] 직접 호출 · 케이스당 프로세스 1개 · 14/14 MATCH vs 독립 재구현)"
for j, why in [(0, "case3 dur=30 → window_sec=max(0,1)=1"), (1, "case0(t 를 아군 옆으로) 10 vs case12(적 기지 150000 밖) 6 · case8 scatter 7"),
               (4, "case1 damage=100000 → 4000 캡 → 26 · case7 accum 캡 20"), (6, "case4 hp=0 → 분모 1 → 160"),
               (7, "case2 hp_value=1000 → 160"), (9, "case5 타워 t → 0 · case6 AddCastedEffect(None) → 0 · case9 explosion 0 → 0"),
               (8, "case 전부 ctx.debug=false: _debug 224B 제로버퍼 무변경(dbg_touched=false)")]:
    p.ev("/specs[130]/consts[%d]" % j, evidence=ev130 + " — " + why, to=2, found_by="new")
for j in (6, 12, 13, 14, 15, 19, 20):
    p.ev("/specs[130]/mem[%d]" % j, evidence=ev130 + " — 그 오프셋을 raw ptr 로 써서 넣은 값이 결과에 그대로 반영", to=3, found_by="new")

# ───────────────────────── 131 noncombat_steroid_value ─────────────────────────
p.fix("/specs[131]/consts[15]/meaning",
      old="최종 합계 /2 (sdiv 35830). 34986·35481 의 2 는 팀 배열 bounds(임계 아님).",
      new="최종 합계 반감 계수 — v/2 (sdiv 35830). 34986·35481 의 2 는 팀 배열 bounds(배열 길이).",
      evidence="m10.ll:35830 `%339 = sdiv i64 %338, 2` (ARITH 소비) · 34986 `%19 = icmp ult i64 %18, 2`(bounds). mkspec3._kind: `임계`(부정문) neg_hit → 규칙③ CMP_ORD 우선으로 `임계` 복귀. `계수` 낱말은 ARITH 관측으로 규칙② 구제",
      behavior_change=False, found_by="reused", kind=u"실오류")
ev131 = "오라클 실행 확인(22차 배치D o131.rs · #[link_name] 직접 호출 · 실전 게임 cache/data · 케이스당 프로세스 1개 · 23/23 MATCH vs 독립 재구현)"
for j, why in [(3, "case1 attack_mult=50/atk 200 → 100→40 캡 → 20 · case17 140→70→40 최종캡"), (5, "case4 attack=50→30 · magic_power=-5→0"),
               (6, "case7 mr_pen=90 → 30→20"), (7, "case6 crit 50 → 10/500·share → 5 · case19 crit 4 → 0"), (8, "case7 def_pen 30 → 10"),
               (9, "case8 range 6000 → 2×4=8 · case9 100000/atk1000 → 25 캡"), (10, "case8 atk200/50=4"), (11, "case9 → 25"),
               (12, "case12 skill_cooldown_mult=0 → 생략 · case10 =50 → 가산"), (13, "case21 아군 4명 이동 n_near=1 → 0 · case22 1명만 → n=2 → 5 · case10 n=5 → 5"),
               (14, "case16 share=750 (1000-750)·10/500=5 캡 15 안"), (15, "case1 40/2=20 · case17 70/2→40"), (16, "case0 전부 0 → 0 · case18 50·1/100=0"),
               (0, "case16 cache attack=[300;5] skill=[100;5] → share 750 · 나머지 500 기본값"), (1, "case16 (1000-750)=250"), (2, "case1 200·50/100"),
               (4, "case3 attack_speed_mult 100 → 200·100/200=100·500/500 → 40 캡")]:
    p.ev("/specs[131]/consts[%d]" % j, evidence=ev131 + " — " + why, to=2, found_by="new")
for j in (0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 13, 14, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29):
    p.ev("/specs[131]/mem[%d]" % j, evidence=ev131 + " — 그 오프셋에 raw ptr/제로버퍼로 써넣은 값이 결과에 그대로 반영(BuffState 288B·Entity·ChampionCache·NoncombatSteroidWindow)", to=3, found_by="new")

# ───────────────────────── 132 champion_hp_value_uncached ─────────────────────────
p.fix("/specs[132]/consts[2]/meaning",
      old="dbg values[8..+8]=10",
      new="dbg values[8..+8]=10 · src_line 934 는 IR 사슬에 있다: `!61969 = !DILocation(line: 934, scope: !61839)`(루트 스코프) 가 m04.ll:52243 memset 의 !dbg — 길이 10 이 `i64 72`(원소 1~9 = 9×8B) 로 접혀 리터럴 10 이 그 줄에 없을 뿐(tcx `{constant#0}` utils.rs:934:29 = 배열 길이 상수)",
      evidence="m04.ll:52243 `call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(72) %5, i8 0, i64 72, i1 false), !dbg !61969` · `!61969 = !DILocation(line: 934, scope: !61839)` · 52246 memset !61970 = line 935. G12(srclinecheck) 가 「그 줄에 리터럴 10」을 요구해 못 본 오탐 — 명세 src_line 934 가 옳다",
      behavior_change=False, found_by="reused", kind=u"오탐")
p.fix("/specs[132]/knobs[4]/where",
      old="utils.rs:934 (배열 10)",
      new="utils.rs:934 (배열 10 — 리터럴은 접혀 m04.ll:52883 `panic_bounds_check(i64 noundef %285, i64 noundef 10` · m04.ll:52954 `slice_index_fail(i64 noundef 0, i64 noundef %79, i64 noundef 10` 에만 남는다)",
      evidence="m04.ll:52883 `tail call void @…panic_bounds_check(i64 noundef %285, i64 noundef 10, …)` · 52954 `tail call void @…slice_index_fail(i64 noundef 0, i64 noundef %79, i64 noundef 10, …)` · 본체 진입부는 memset 72 / `ugt 8` / `ugt 9` / `ult 11` 로 접혀 G19 창(관측 [0,1,8,72])에 10 이 없었다 — 값은 옳다(오탐)",
      behavior_change=False, found_by="reused", kind=u"오탐")
ev132 = "오라클 실행 확인(22차 배치D o132.rs · hidden define 을 #[link_name] 으로 직접 호출 · 제로버퍼 ScoreParameter/ChampionScoreParameter · 케이스당 프로세스 1개 · 10/10 MATCH vs 독립 재구현)"
for j, why in [(0, "case7 cc_time_x_inv_cd=1000·dps"), (1, "case7 buff_inv_cd_count=1000·dps"), (2, "case5/6 아군 9+적 5 → 적은 안 들어감(n=10)"),
               (3, "case5/6 n=10 잘림"), (4, "case5/6 적 루프 미진입"), (5, "case0 n=1 → 배율 100 → 1500"),
               (7, "case5 r=111 → clamp 100 → 200"), (8, "case6 r=44 → +50=94 · case5/1 r≥51 → ×2"), (9, "case2 r=0 → 50 → 750"),
               (10, "case1 r=100 → 200 → 3000"), (11, "case6 n-1=9 분모"), (12, "case4 팀 불일치 → opp_dps 가산(target 적팀)")]:
    p.ev("/specs[132]/consts[%d]" % j, evidence=ev132 + " — " + why, to=2, found_by="new")
for j in range(16):
    p.ev("/specs[132]/mem[%d]" % j, evidence=ev132 + " — 제로버퍼에 그 오프셋만 써넣어 호출, 결과가 재구현과 전건 일치(다른 오프셋은 0)", to=3, found_by="new")


# ───────────────────────── 133 buff_value_v54 ─────────────────────────
ev133 = "오라클 실행 확인(22차 배치D o133.rs · #[link_name] 직접 호출 · 실전 게임 cache/data · near_enemies 비움(앵커 None) · 케이스당 프로세스 1개 · 25/25 MATCH vs 독립 재구현)"
for j, why in [(0, "case13 duration=Time(120) → window 2 · 나머지 Permanent(tag 0) → 6"), (1, "case13 120/60=2 · Permanent → 6 캡"),
               (2, "a_ps/s_ps/u_ps = 캐시 5칸 합/5 (100·100·200) 으로 case1·3·4·14·21 값 일치"), (3, "case1 attack_mult 50 → 50 · case7 def*mult/100 · case9/22 hp 항"),
               (4, "case16 def_pen 50 → def_after 50 → max(50,-99)+100 분모"), (5, "case10/11 toughness 100 → hv/200 = 10·50"),
               (7, "case18 damaged_amplify: 아군 5명 전부 120000 내 → min(5,3)=3"), (8, "case18 min(near,3)=3 · case11 cc_threat 만 → hv/3=33"),
               (10, "case3/4/24 covered=5(60000 내) >1 → 쿨감 항 발화"), (11, "case2 epic=0 → offense 미환산 0 · case23 hv<0 → smax 0"),
               (14, "case18/20/24 → 160 캡"), (15, "case7 d_def*(incoming/2)=100*100/250=40 · case22 hp_mult"),
               (16, "case7 max(def+100+d_def,1) · 분모 max(hp,1)=600 전 케이스")]:
    p.ev("/specs[133]/consts[%d]" % j, evidence=ev133 + " — " + why, to=2, found_by="new")
for j in (2, 4, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 27, 28, 41, 42, 43, 44, 55, 57, 58, 59, 60, 61, 65, 68, 69, 70, 75, 76, 77, 78, 79):
    p.ev("/specs[133]/mem[%d]" % j, evidence=ev133 + " — 그 오프셋에 raw ptr/제로버퍼(BuffState 288B·Entity·ChampionCache·DefensiveCrisis 2B)로 써넣은 값이 결과에 그대로 반영", to=3, found_by="new")

# ───────────────────────── 지시문 오류 ─────────────────────────
p.brief_error(u"§1 「vis 가 pub 이 아니면 오라클 직접 진입이 막힌다 — 상위 pub 래퍼/형제 복제본을 노려라」는 과잉이다. `in:game_ai` 라도 IR define 이 `hidden`(비-internal)이면 `extern \"Rust\" { #[link_name = \"<망글링>\"] fn … }` 로 fat LTO 프로브에서 직접 호출된다(이번 130·131·132 전부 이 수법, 37/37 MATCH). 조건 = ①프로브가 game_ai 의 pub 항목을 하나라도 참조해야 크레이트가 링크 그래프에 들어간다(없으면 LNK2019 — 1차 시도 실측) ②`internal fastcc`(129 등)는 내부화돼 불가(범위 명시). METHOD_MAP ⑥ 한계1·TEMPLATE 에 반영 요망")
p.brief_error(u"§5 「`consts.kind` 는 v2 에 없는 파생 필드라 errors[] 로 직접 못 쓴다」는 낡았다 — mkspec3._kind 는 19차부터 「v2 행이 kind 를 명시하면 관측 추정보다 우선」이고 applypatch 는 `old: null` 로 없는 키를 채운다(_subtree _MISS 경로). 이번엔 지시대로 `meaning` 낱말만 고쳤지만(129 c7 → 길이 · 131 c15 → 계수), 낱말 체인(`인덱스`·`임계`·`상한`·`문턱` 이 `길이` 보다 먼저 잡힘)을 피해 써야 해서 문면이 부자연스럽다 — 명시 kind 경로를 지시문에 적어 두는 편이 안전하다")
p.brief_error(u"§4 G19 [132] knobs[4] 「관측(창)=[0, 1, 8, 72]」: 창이 본체 진입부(memset 72)만 봤고 리터럴 10 이 있는 panic 콜리 사이트(52883·52954)는 창 밖이었다 — knobval 이 `where` 에 IR 줄이 없을 때 src_anchors(934)로 잡는 창은 접힌 배열 길이를 원리적으로 못 본다(길이 N 배열은 memset (N-1)×8 로 남는다). 검사기 제안: value 가 `길이`/배열 노브면 `panic_bounds_check(…, i64 noundef N` · `slice_index_fail(…, i64 noundef N` · memset `(N-1)*8` 을 동치 관측으로 인정")
p.brief_error(u"§4 G12 [132] consts[2] 「IR 사슬에 934 가 없다」는 거짓 — `!61969 = !DILocation(line: 934, scope: !61839)` 가 52243 memset 에 붙어 있다. srclinecheck 가 「그 줄에 리터럴 값」을 요구해 접힌 상수(10→72)를 놓친다. 제안: src_line 대조와 값 대조를 분리해, 사슬에 줄이 있으면 G12 통과·값은 G19 몫으로")

out = p.save()
print(out)
