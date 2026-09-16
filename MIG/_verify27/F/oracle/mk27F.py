# -*- coding: utf-8 -*-
"""27차 F patch.json 생성 — mkpatch 참조구현 사용(old 는 정본과 즉시 대조). 실행: python -X utf8 mk27F.py"""
import sys, io, json
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=27, batch="F")

# ───────────── 263 get_small_action_score_closure · G12 5건 + 같은 규약 위반 4건(게이트 미적발) ─────────────
# 규약 = src_line 은 명세 own 파일(auction.rs) 줄(`srclinecheck._ownlines` 가 own 파일 줄만 대조). 이 상수들은 small_action.rs:308~326(get_action 인라인)
# 의 줄이고 own 줄은 사슬 `;L309<179<310<1162<107<2155<607`(_next\reach\ca6700.ll:49106~49110) 의 **179**(`c.get_action()`).
ev263 = (u"IR 사슬 _next\\reach\\ca6700.ll:%s `%s ;L%d<179<310<1162<107<2155<607` — own 파일(auction.rs) 줄은 179 뿐. "
         u"309/314/310/312/321/324 는 small_action.rs(get_action 인라인 · 논리 표는 meaning·logic 에 유지). G12 규약 = own 파일 줄")
for j, old, ln, ins in ((1, 309, 49106, u"%67 = icmp ne i8 %58, 10"), (2, 309, 49108, u"%68 = add nsw i8 %58, -3"),
                        (3, 309, 49109, u"%69 = icmp samesign ugt i8 %58, 2"), (4, 309, 49110, u"%70 = select i1 %69, i8 %68, i8 7"),
                        (9, 314, 49138, u"br label %81")):
    p.fix("/specs[263]/consts[%d]/src_line" % j, old=old, new=179, evidence=ev263 % (ln, ins, old),
          behavior_change=False, found_by="reused", kind=u"실오류")
# 게이트가 못 잡은 같은 규약 위반(phi 인입 상수 · 강후보 0 → 판정보류였음). 사슬: 49141 `br label %81 ;L312<179<…`, 49144 `;L321<179<…`, 49153 `;L324<179<…`,
# 310 은 IR 에 L310 마커가 없고 phi 49162 `[0, %66]` 인입의 블록 %66 종결자(switch 49129) 사슬이 `;L309<179<…`.
for j, old, ln, ins in ((7, 310, 49129, u"switch i8 %70 … ] ;L309<179"), (8, 312, 49141, u"br label %81 ;L312<179"),
                        (10, 321, 49144, u"br label %81 ;L321<179"), (11, 324, 49153, u"br label %81 ;L324<179")):
    p.fix("/specs[263]/consts[%d]/src_line" % j, old=old, new=179,
          evidence=u"_next\\reach\\ca6700.ll:%d `%s<310<1162<107<2155<607` — own 파일 줄 179(위 5건과 같은 규약 · G12 는 phi 인입/약후보라 미적발). "
                   u"small_action.rs 줄(%d)은 meaning 의 L%d 표기로 보존" % (ln, ins, old, old),
          behavior_change=False, found_by="new", kind=u"실오류")
# G16 문면 보강: %2 bump 는 define 줄에 속성이 있다
p.fix("/specs[263]/sig/params[2]/role", old=u"define 줄 속성 없음(plain ptr).",
      new=u"define 줄 속성: noundef nonnull align 8(readonly·captures 없음).",
      evidence=u"m01.ll:48954 define 줄 `ptr noundef nonnull align 8 %2` — 「속성 없음」은 오기(G16 define 속성 대조)",
      behavior_change=False, found_by="new", kind=u"보강")

# ───────────── 266 can_near_enemies_range · G12 2건 ─────────────
ev266 = (u"_next\\reach\\dd9f30.ll:%d `store i64 %d, ptr %%7%d, ;L24<1002<537` — Range{0,5} 필드 store 의 !dbg 루트가 team_plan.rs:537(filter_map 조립 줄). "
         u"505 는 from_iter 호출 줄(26102 `;L505`)이지 이 store 의 사슬엔 없다(G12 규약 = own 파일 줄 · 강후보 537)")
p.fix("/specs[266]/consts[3]/src_line", old=505, new=537, evidence=ev266 % (26094, 0, 0), behavior_change=False, found_by="reused")
p.fix("/specs[266]/consts[4]/src_line", old=505, new=537, evidence=ev266 % (26096, 5, 1), behavior_change=False, found_by="reused")

# ───────────── 267 expected_dps · G12 2건 + G18 1건 ─────────────
ev267 = (u"_next\\reach\\eb5dd0.ll:%d `%s ;L%s<26` — ProfTimer drop 시점(스코프 끝 = fight_check.rs:26 return) 의 명령. "
         u"10 은 `_t` 선언 줄(23541 `store i64 52 ;L179<10` 의 52 만 10 이 맞다)")
p.fix("/specs[267]/consts[5]/src_line", old=10, new=26, evidence=ev267 % (23669, u"atomicrmw add … PHASE_NANOS, i64 416", u"3937<3162<185<825<825"),
      behavior_change=False, found_by="reused")
p.fix("/specs[267]/consts[6]/src_line", old=10, new=26, evidence=ev267 % (23661, u"%69 = mul i64 %67, 1000000000", u"632<185<825<825"),
      behavior_change=False, found_by="reused")
p.fix("/specs[267]/logic", old=u"if ult.tag(+0x30) != -1 {",
      new=u"if ult.tag(+0x30 = Entity@0x530 skill2_effect@tag · level<=2 면 상수 None 의 태그) != -1 {",
      evidence=u"G18 P1: `+0x30` 은 select 결과 포인터 기준 상대 오프셋이고 mem 표의 행은 Entity 0x530(mem[6] · m15.ll:23610~23612 `gep %46, 48`). 표 좌표(0x530)를 같은 문장에 병기",
      behavior_change=False, found_by="reused", kind=u"보강")

# ───────────── 264 push_candidate · G16 P3 + G5 (ArgumentPromotion 행 병합 · 선례 261 kite_reposition_point 형식) ─────────────
# 1) params[6](target.y) 삭제 → 2) params[5] 를 소스 인자 `target` 1행으로 (IR 두 슬롯 병기)
p.errors.append({"op": "delete", "path": "/specs[264]/sig/params[6]", "guard": "target.y", "kind": u"실오류",
                 "old": None, "new": None,
                 "evidence": u"G16 P3/G5: sig.tcx 소스 인자 15 vs params 16 — i=6 이 두 행(target.x/target.y). 선례 [261] kite_reposition_point 는 승격된 소스 인자를 1행으로 두고 type 에 IR 두 슬롯을 병기(`&Entity → 스칼라 승격 i64 %5(=nearest.x…) · i64 %6`). 같은 형식으로 병합(이 행은 아래 params[5] 에 흡수)",
                 "behavior_change": False, "found_by": "reused"})
p.fix("/specs[264]/sig/params[5]/name", old=u"target.x", new=u"target",
      evidence=u"m11.ll:44238 define: `i64 %5, i64 %6`(속성 없음) = 소스 `target: &Entity`(tcx 6번째) 의 ArgumentPromotion 조각 — 호출자 choose_goal 이 target+0x660/+0x668 을 load 해 넘김(m11.ll:43534~43537). 행 1개로 병합(선례 261)",
      behavior_change=False, found_by="reused")
p.fix("/specs[264]/sig/params[5]/type", old=u"i64 %5 (ArgumentPromotion 조각 1/2 · DI `target` = ptr poison)",
      new=u"&Entity → 스칼라 승격 i64 %5(=target.x, Entity+0x660) · i64 %6(=target.y, Entity+0x668) (ArgumentPromotion · DI `target` = ptr poison)",
      evidence=u"m11.ll:44238 define 줄 `i64 %5, i64 %6` · 호출자 m11.ll:43534~43537 load Entity+0x660/+0x668 · 선례 [261] params[5].type 형식",
      behavior_change=False, found_by="reused")
p.fix("/specs[264]/sig/params[5]/role", old=u"IR 속성 없음. 468줄 utils::distance 3번째 인자로만. 호출자 choose_goal 이 target+0x660 을 load 해 넘김(m11.ll:43534~43535)",
      new=u"IR 속성 없음(둘 다). %5 = 468줄 utils::distance 3번째 인자 · %6 = 4번째 인자(target+0x668, 43536~43537). 호출자 choose_goal 이 target+0x660/+0x668 을 load 해 넘김(m11.ll:43534~43537). exe(0.5.8 0xe25450)도 두 스칼라([entry+0x28]/[entry+0x30] → utils::distance r8/r9 · e255c4/e255bc)로 받는다",
      evidence=u"m11.ll:44238~44465 %5/%6 사용처 = 44284~ utils::distance 호출 인자만 · 0.5.8 exe 디스어셈 e255bc~e255d2(argscan --caller 0xe248f0 3 호출부 [rsp+0x28]=target.x [rsp+0x30]=target.y)",
      behavior_change=False, found_by="reused")
# 3) exe ↔ IR 대응표(sig.exe_args · 선례 185 형식 · gensweep20 EXE_ABI 재료) — exe 17 vs IR 16 의 여분 = positioning_score(%7) 의 (cx, cy) 승격
exe_args = {
    "note": (u"0.5.8 백업 exe(tfm2_0.5.8\\TeamfightManager2.exe · 설치 exe 는 09-16 14:40 0.6.0 패치로 RVA 불일치) argscan 0xe25450: 레지스터 4 + 스택 [entry+0x20..+0x80] 13개 = exe 인자 17 vs IR 16. "
             u"여분 1 = IR %7 `&PositioningScoreData(2760B)` 가 exe 에선 (cx@+0xab8, cy@+0xac0) 두 i64 스칼라로 ArgumentPromotion(#185 get_input_target 와 동형). "
             u"근거 = 진입부 디스어셈: e25466/e2546e `[rsp+0x120]/[rsp+0x118]`(=entry+0x50/+0x48)→ /32000 clamp 29 = y/x(xi=r14·yi=r12) · e254ad `[r9+8]`=data.context · e254db `[r8+0x930]`=player.team(is_enemy_well_danger 인라인) · "
             u"e25565 `movzx [rsp+0x148]`(entry+0x78)=line→is_near_line r9d · e255bc/e255c4 `[rsp+0x100]/[rsp+0xf8]`(entry+0x30/+0x28)→utils::distance(0x12a07d0) r9/r8 = target.y/target.x · "
             u"e255b4 `[rsp+0x130]`(entry+0x60)=attack_range(cmp rax,r15 · 0x11170=70000 분기) · e255eb `[rsp+0x138]`(entry+0x68)=preferred_range(|dist−pr|) · "
             u"e255f3 `[rsp+0x108]`(entry+0x38)=cx(`xi−cx−4` cmp −7) · e25665 `[rsp+0x110]`(entry+0x40)=cy(`yi−cy−4`) · e2567e `movzx [rsp+0x128]`(entry+0x58)=purpose(i8 → 0xd84db0 4번째) · "
             u"e2563c `[rsp+0xf0]`(entry+0x20)=champ(`[r13+0x660]/[r13+0x668]`) · e257ca `add r10,[rsp+0x140]`(entry+0x70)=target_score · e2579d `[rsp+0x150]`(entry+0x80)=source_bonus(호출부 3곳 상수 0x23/0xa/0 = 35/10/0) · "
             u"rcx=candidates(lea 지역 Vec · push 0xe27360 rcx) · rdx=version(rbx → 0xd84db0 rdx) · r9=data. exe 는 is_enemy_well_danger 를 인라인(0xfa01/0x27101/0xc3500/0xdac00 사각형)하고 position_score_at_position 을 (sret,version,셀중심x,셀중심y,purpose) 5인자 콜리 0xd84db0 로 부른다(player/data/positioning_score 미전달 → 셀 키 TLS 캐시 추정)."),
    "map": [
        {"exe": "rcx", "ir": "%0", "what": "candidates &mut bumpalo::Vec<(u64,u64,i64)>"},
        {"exe": "rdx", "ir": "%1", "what": "version"},
        {"exe": "r8", "ir": "%2", "what": "player &PlayerState"},
        {"exe": "r9", "ir": "%3", "what": "data &OperationData"},
        {"exe": "[entry+0x20]", "ir": "%4", "what": "champ &Entity"},
        {"exe": "[entry+0x28]", "ir": "%5", "what": "target.x (i64 · 소스 target:&Entity 의 +0x660 승격 — IR 도 스칼라)"},
        {"exe": "[entry+0x30]", "ir": "%6", "what": "target.y (i64 · +0x668)"},
        {"exe": "[entry+0x38]", "ir": "%7 → load(%7+0xab8)", "what": "positioning_score.cx (i64 승격 · exe 만)"},
        {"exe": "[entry+0x40]", "ir": "%7 → load(%7+0xac0)", "what": "positioning_score.cy (i64 승격 · exe 만)"},
        {"exe": "[entry+0x48]", "ir": "%8", "what": "x"},
        {"exe": "[entry+0x50]", "ir": "%9", "what": "y"},
        {"exe": "[entry+0x58]", "ir": "%10", "what": "position_eval_purpose (i8)"},
        {"exe": "[entry+0x60]", "ir": "%11", "what": "attack_range"},
        {"exe": "[entry+0x68]", "ir": "%12", "what": "preferred_range"},
        {"exe": "[entry+0x70]", "ir": "%13", "what": "target_score"},
        {"exe": "[entry+0x78]", "ir": "%14", "what": "line (i8)"},
        {"exe": "[entry+0x80]", "ir": "%15", "what": "source_bonus (호출부 상수 35/10/0)"},
    ],
    "gensweep_exe_abi": "[0, 1, 2, 3, 4, 5, 6, (\"FAKE\", 2760, [(0xab8, 7, \"i64\"), (0xac0, 8, \"i64\")]), 9, 10, 11, 12, 13, 14, 15, 16]",
    "fake_verdict": (u"FAKE 재구성 가능(#185 와 동형): ①exe 재호출(훅→원본): IR %7 에서 cx=*(ps+0xab8), cy=*(ps+0xac0) 를 꺼내 +0x38/+0x40 에 넣고 IR %8..%15 를 exe +0x48..+0x80 으로 한 칸씩 민다. "
                     u"②IR 재현 호출(원본 인자→내 함수): 2760B PositioningScoreData 가짜 버퍼에 +0xab8/+0xac0 만 채워 %7 로 넘긴다 — ⚠단 IR 사본은 %7 을 position_score_at_position(m11.ll:44358 · 4번째 인자)에도 넘기므로 "
                     u"그 콜리가 다른 필드를 읽으면 가짜 버퍼로는 재현이 갈린다(exe 는 그 콜리를 셀 좌표 5인자로 부르므로 게임 쪽은 안 읽음). 적용 범위 = 진입부 detour 재호출 방식 · ⚠define internal fastcc 라 sweep 편입은 patches.json 노출 필요(#40 부류). "
                     u"⚠RVA 0xe25450·크기 952B 는 0.5.8 기준 — 0.6.0 exe 에선 .pdata 시작이 아니다(0xe23740~0xe254b4 7540B 안) → 재핀 필요"),
}
p.errors.append({"path": "/specs[264]/sig/exe_args", "kind": u"보강", "old": None, "new": exe_args,
                 "evidence": u"0.5.8 백업 exe argscan/fnprobe/capstone 디스어셈(scratchpad27F\\pc_dis.txt · argscan058.py 0xe25450 [--caller 0xe248f0]) — 대응표는 note 의 명령 주소로 전건 추적 가능",
                 "behavior_change": False, "found_by": "reused"})

# ───────────── ev 상향(오라클 실행 확증) ─────────────
ORC = u"오라클 실행 확인: _verify27\\F\\oracle\\o27F.rs(케이스당 프로세스 1개 · run27F.py · o27F_cases.log) — "
# 262 target_score(hidden → link_name 직접 진입) 16/16 MATCH
for j, why in ((0, u"hp_ratio=hp*100/max(maxhp,1): max=0 → ratio 100000(hp 1000) · 250/1000 → 25"),
               (1, u"umax(maxhp,1) 가드: ts 1000 0 0 0 → hp_ratio 100000 · smax(score,1): dx 300000/900000 → Some(1)"),
               (2, u"킬 여유 5: dmg 50 에 hp 55 → 113(+90) / hp 56 → 73(+50) 경계 갈림"),
               (8, u"+90: hp 3·0·55(dmg50) → 113 = 90+20+rnd 3"),
               (9, u"임계 26: ratio 25 → +50(73) / ratio 26 → +25"),
               (10, u"+50: ratio 20·25 → 73"),
               (11, u"임계 51: ratio 50 → +25(48) / ratio 51 → 0(23)"),
               (18, u"+20 사거리 안: dist 0·5000·100000(range 120000) 가산 / dist 150000 → 미가산"),
               (19, u"dp/-3000 0 방향 절삭: dx 5000 → −1(22) · dx 100000 → −33(15) · dx 150000 → −50"),
               (20, u"gen_range(0..=10) 정확히 1회·무조건: 16/16 케이스에서 StdRng 클론의 gen_range 1회 후 상태 == 게임 호출 후 상태(rnd_same_after=true) · rnd 값 3 재현"),
               (21, u"하한 0: 위와 같은 사이트")):
    p.ev("/specs[262]/consts[%d]" % j, evidence=ORC + u"target_score 16/16 MATCH(명세식 독립 재구현 vs 게임 · wave None) — " + why, to=2, frm=4, found_by="new")
for j, why in ((0, u"5 마진 경계 55/56"), (4, u"+90 즉시 처치 3건"), (5, u"26/51·+50/+25 4건"), (8, u"+20 3건/미가산 3건"), (9, u"-3000 절삭 3건"), (10, u"난수 0..=10 · rnd 1회 소비 16/16")):
    p.ev("/specs[262]/knobs[%d]" % j, evidence=ORC + u"target_score 16/16 MATCH — " + why, to=2, frm=4, found_by="new")
# 267 expected_dps(pub) 10/10 MATCH
for j, why in ((0, u"None 슬롯(-1) 건너뜀: a=0/s=0/s2=0 조합 6건 · 전부 None → 0"),
               (1, u"×1000/cd: 100·1000/3=33333 · 199·1000/3=66333 · 298·1000/1=298000 · 12223·1000/3=4074333"),
               (2, u"level>2 게이트: s2=300 lv=3 → 298000 / lv=2 → 0 · (100,200,300) lv3 397666 / lv2 99666")):
    p.ev("/specs[267]/consts[%d]" % j, evidence=ORC + u"expected_dps 10/10 MATCH(Σ dmg×1000/cd 독립 재구현 vs 게임) — " + why, to=2, frm=4, found_by="new")
for j, why in ((0, u"1000 스케일 10건"), (1, u"레벨 게이트 2 vs 3 2건"), (2, u"쿨 하한 실측 cd=(3,3,1): attack/skill 3 · skill2 1 → dmg×1000/3 · /1")):
    p.ev("/specs[267]/knobs[%d]" % j, evidence=ORC + u"expected_dps 10/10 MATCH — " + why, to=2, frm=4, found_by="new")
# 266 can_near_enemies_range(pub) 15/15 MATCH (gamemode=0 · DM 분기 미실행)
for j, why in ((3, u"Range 0..5 슬롯 순회: d=2000000 → 5명 전원 [23,24,25,26,27] id 오름차순"),
               (4, u"위와 같음(5 슬롯)"),
               (5, u"tps*3=180 경계: tick 180(elapsed 180) → 도달원반 경로(D) · tick 190 → 추정 경로(E) — 둘 다 게임과 일치"),
               (6, u"err>300000 제외: tick 5000 → err 480000 · tick 20000 → 1800000 → 전원 제외 []"),
               (7, u"허용 = err + ms·3·tps + d: ms 3000·tick 1000 → lim=err+540000+d 로 전원 포함 · ms 1 → lim=err+180+d 로 전원 제외")):
    p.ev("/specs[266]/consts[%d]" % j, evidence=ORC + u"can_near_enemies_range 15/15 MATCH(Moba · 명세식 독립 재구현: unseen_error_radius/unseen_estimated_pos/utils::distance pub 콜리 사용) — " + why, to=2, frm=4, found_by="new")
for j, why in ((0, u"180/190 경계"), (1, u"480000/1800000 제외"), (2, u"ms·3·tps 여유(ms 1 vs 3000)")):
    p.ev("/specs[266]/knobs[%d]" % j, evidence=ORC + u"can_near_enemies_range 15/15 MATCH — " + why, to=2, frm=4, found_by="new")

# ───────────── brief_errors ─────────────
p.brief_error(u"★게임이 09-16 14:40 에 0.6.0 으로 패치됐다(설치 폴더 exe·bundle·mod-sdk-stable base_version=0.6.0 동시 갱신). 도시에(14:55 생성)·명세 6건의 exe RVA(e25030·ca6700·e25450·e02bc0·dd9f30·eb5dd0)·기준 함수 e083c0/e05450 전부 현행 exe .pdata 시작이 아니다 — exe 축 지시(fnprobe/argscan)는 `tfm2_0.5.8\\TeamfightManager2.exe` 백업으로만 성립(6 RVA·크기 1045/1007/952/871/745/742B 일치 확인). MIG 도구의 EXE 경로가 설치 exe 하드코딩이라 스크래치 사본(argscan058/fnprobe058)으로 우회했다.")
p.brief_error(u"지시 ③의 대응표 키 `sig.exe_abi` 는 정본에 없는 이름 — 선례(185 get_input_target)와 mkspec3 가 v3 로 노출하는 키는 `sig.exe_args`(note/map/fake_verdict)다. exe_abi 로 쓰면 v3 재생성에서 사라진다(mkspec3.py:582 화이트리스트). exe_args 로 냈다.")
p.brief_error(u"지시 ④(base_battle_action 의 base_defense_focus 인라인 확인)는 배치 F 담당(262~267) 밖이다 — 배치 A 몫.")
p.brief_error(u"§4 G12 [263] 5건의 「실제 후보 179」는 맞지만 판정 이유 「IR 사슬에 그 줄이 없다」는 부정확 — 309 는 사슬(`;L309<179<…`)에 **있다**. 게이트는 own 파일(auction.rs) 줄만 세므로(srclinecheck._ownlines) 문면을 「own 파일 줄이 아니다」로 고쳐야 배치가 IR 을 다시 뒤지지 않는다. 같은 규약 위반 4건(consts 7/8/10/11 · phi 인입)은 강후보 0 이라 미적발이었다.")
p.brief_error(u"G16 P3 [264] 「16행이어야 하는데 15행」 판정은 ArgumentPromotion 을 모른다 — 두 행이 같은 i=6 이라 소스 인자 수는 15 로 맞다. 선례(261)대로 1행 병합으로 닫았지만, 게이트가 `len(set(i))` 로 세면 이런 케이스는 오탐이 아니라 표기 선택의 문제가 된다(둘 중 하나로 규약 명문화 필요).")
p.brief_error(u"applypatch 는 스칼라 필드의 null→문자열 치환을 지원하지 않는다(`sig.abi` null 을 산문으로 채우려면 실패 — cur None 이면 open/notes 문면 탐색으로 떨어져 「필드가 문자열이 아니고 문면도 못 찾음」). dict 만 가능해 exe 대응표를 exe_args(dict)로만 냈다.")
p.brief_error(u"[266] open[3](exe 0xc99fe0 상수 0x7d1·0xbb8·0x9c41) 해소 — 0.5.8 exe fnprobe: c99fe0 = from_iter 클로저 본체(panic Location team_plan.rs:506:7·537:23 · 300000 포함) 이고 2001/3000/40000(+1)·57000/10000 은 game_core `unseen_error_radius`(_gcbc g11.ll:264589) · 2001/1000 은 `unseen_estimated_pos`(g11.ll:264615) 의 리터럴 = 두 콜리의 exe 인라인. IR 은 call 이라 본문에 없는 것이 정상. → notes 로 이동 요청(산문).")
p.brief_error(u"[265] open[0] 은 G10 대로 사실 서술 → notes 이동 요청. [266] open[0](534 좌우 순서·표기 불가·동작 확정)은 notes 로, open[2] 의 is_recent_visible_big_action 은 G7 대로 shared 확정이니 그 이름만 제거하고 나머지 콜리(unseen_error_radius/unseen_estimated_pos/awareness_lapse_state) 계약만 항목으로 남길 것(산문).")
p.save()
