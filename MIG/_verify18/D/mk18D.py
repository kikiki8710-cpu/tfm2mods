# -*- coding: utf-8 -*-
# 18차 배치 D patch.json 생성 (mkpatch 경유 — old 실재 검증)
import sys, io
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import mkpatch

p = mkpatch.Patch(round=18, batch="D")

# ───────────────────────────── G16 P3 `i` 번호 규약 (sret=0, 소스 1..n) ─────────────────────────────
G16_EV = (u"규약 정본 07·15 = i 는 소스 시그니처 위치(1-based), sret 만 0. "
          u"이 함수는 `define void`(sret 없음): m09.ll:{ln} `define void @{sym}(… %0 … %{last})` — "
          u"#72·#73 은 이미 1..n 이고 #74~#76 만 0-기반이었다")
for spec, n, ln, sym in ((74, 6, 4395, u"…GoalData6update"),
                         (75, 7, 22178, u"…SerpenStanceData11update_plan"),
                         (76, 7, 41134, u"…EpicStanceData11update_plan")):
    for j in range(n):
        p.fix(u"/specs[%d]/sig/params[%d]/i" % (spec, j), old=j, new=j + 1,
              evidence=G16_EV.format(ln=ln, sym=sym, last=n - 1),
              behavior_change=False, found_by="reused", kind=u"실오류")

# ───────────────────────────── G5 sig 정본 대조 (ptr → &mut …) ─────────────────────────────
p.fix(u"/specs[75]/sig/params[2]/type", old=u"ptr", new=u"&mut StdRng(320B)",
      evidence=u"tcx sig `&mut rand::rngs::std::StdRng` · m09.ll:22178 `%2 = ptr noalias readnone align 16 captures(none)`"
               u"(readnone 이라 dereferenceable(320) 표식이 빠졌을 뿐 타입은 &mut StdRng)",
      behavior_change=False, found_by="reused", kind=u"실오류")
p.fix(u"/specs[76]/sig/params[2]/type", old=u"ptr", new=u"&mut StdRng(320B)",
      evidence=u"tcx sig `&mut rand::rngs::std::StdRng` · m09.ll:41134 `%2 = ptr noalias readnone align 16 captures(none)`",
      behavior_change=False, found_by="reused", kind=u"실오류")
p.fix(u"/specs[76]/sig/params[6]/type", old=u"ptr", new=u"&mut DebugFrameData(224B)",
      evidence=u"tcx sig `&mut game_core::DebugFrameData` · m09.ll:41134 `%6 = ptr noalias readnone align 8 captures(none)` "
               u"(호출자 m09.ll:4711 이 `ptr noalias nonnull align 8 poison` 으로 넘김 — 미사용이지만 타입은 &mut DebugFrameData)",
      behavior_change=False, found_by="reused", kind=u"실오류")

# ───────────────────────────── G20 R2 MapDef region_dist — 이름 갈라기 ─────────────────────────────
R2_EV = (u"tcxdict MapDef 0x54e8 = `region_dist[0][2]` · 0x5510 = `region_dist[0][7]` (region_dist@0x54d8 [[usize;27];27]) — "
         u"두 값 모두 옳고 같은 배열의 다른 열이다. sharedchk.norm_name 이 첨자 값을 `[]` 로 지워 "
         u"`region_dist[][]` 로 같아지므로 첨자 밖에 `col2`/`col7` 를 붙여 판별력을 준다")
p.fix(u"/specs[75]/mem[26]/name", old=u"region_dist[a][2] (=region_dist@0x54d8 + 2*8)",
      new=u"region_dist[a][2] col2 (=region_dist@0x54d8 + 2*8)", evidence=R2_EV,
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(u"/specs[76]/mem[22]/name", old=u"region_dist[a][7] (=region_dist@0x54d8 + 7*8)",
      new=u"region_dist[a][7] col7 (=region_dist@0x54d8 + 7*8)", evidence=R2_EV,
      behavior_change=False, found_by="new", kind=u"보강")

# ───────────────────────────── 리전 2/7/0/26 — 「추정」 → 확정 ─────────────────────────────
REG_EV = (u"정적 전역 재계산(_gcbc/g07.ll:276 regions `.270` · :277 region_dist `.271` · :278 region_centers `.272`, "
          u"scratch v18D/mapglob.py): regions[21][21]=2 · region_centers[2]=(688941,688941)=셀(21,21) 중심 / "
          u"regions[9][9]=7 · region_centers[7]=(294683,297518) / regions[27][3]=0(블루 넥서스 셀) · regions[3][27]=26(레드 넥서스 셀). "
          u"v2 shared.맵_좌표계 와 일치")
p.fix(u"/specs[75]/mem[26]/note", old=u"리전 2 = 세르펜 리전(추정)",
      new=u"리전 2 = Serpen 리전 — 확정(tcxdict MapDef 0x54e8=region_dist[0][2] · shared.맵_좌표계 regions[21][21]=2 · region_centers[2]=(688941,688941) · _gcbc/g07.ll:276~278 정적 전역 재계산)",
      evidence=REG_EV, behavior_change=False, found_by="reused", kind=u"보강")
p.fix(u"/specs[76]/mem[22]/note", old=u"리전 7 = 에픽 리전(추정)",
      new=u"리전 7 = Morgard(에픽) 리전 — 확정(tcxdict MapDef 0x5510=region_dist[0][7] · shared.맵_좌표계 regions[9][9]=7 · region_centers[7]=(294683,297518) · _gcbc/g07.ll:276~278 정적 전역 재계산)",
      evidence=REG_EV, behavior_change=False, found_by="reused", kind=u"보강")
p.fix(u"/specs[75]/consts[3]/meaning",
      old=u"리전 id 2 = 세르펜 리전(추정) — region_center(2)(closure#4 :446) / region_dist(·,2)(closure#8/#9 :494,:500)",
      new=u"리전 인덱스 2 = Serpen 리전 — 확정(shared.맵_좌표계: 정적 전역 _gcbc/g07.ll:276 regions[21][21]=2 · :278 region_centers[2]=(688941,688941)). "
          u"region_center(2)(closure#4 :446, gep +27584/+27592 로 접힘) / region_dist(·,2)(closure#8/#9 :494,:500, gep +21736)",
      evidence=REG_EV, behavior_change=False, found_by="reused", kind=u"보강")
p.fix(u"/specs[76]/consts[1]/meaning",
      old=u"리전 id 7 = 에픽 리전(추정) — region_dist(·,7)·region_center(7) 의 b 인자 (aux + 본문 closure#6/#7)",
      new=u"리전 인덱스 7 = Morgard(에픽) 리전 — 확정(shared.맵_좌표계: 정적 전역 _gcbc/g07.ll:276 regions[9][9]=7 · :278 region_centers[7]=(294683,297518)). "
          u"region_dist(·,7)·region_center(7) 의 b 인자 (aux closure#2 m09.ll:66232 gep +21776 · 본문 closure#6/#7)",
      evidence=REG_EV, behavior_change=False, found_by="reused", kind=u"보강")
p.fix(u"/specs[74]/consts[6]/meaning",
      old=u"사망 적(team 0)의 리전 코드 = 0 (본진 리전 추정: 확정 근거 없음 — unknown 참조)",
      new=u"사망 적(team 0)의 리전 코드 = 0 = 블루(team0) 본진 리전 — 확정(정적 전역 _gcbc/g07.ll:276 regions[27][3]=0 = 블루 넥서스 셀 · 스타트 위치 셀 regions[25][2]=0 · region_centers[0]=(128000,832000)). 태그 아님 — 리전 인덱스값",
      evidence=REG_EV, behavior_change=False, found_by="reused", kind=u"보강")
p.fix(u"/specs[74]/consts[7]/meaning",
      old=u"사망 적(team 1)의 리전 코드 = 26 (본진 리전 추정: 확정 근거 없음)",
      new=u"사망 적(team 1)의 리전 코드 = 26 = 레드(team1) 본진 리전 — 확정(정적 전역 _gcbc/g07.ll:276 regions[3][27]=26 = 레드 넥서스 셀 · region_centers[26]=(832000,128000)). 리전 인덱스값(사망 적은 본진에 있다고 가정)",
      evidence=REG_EV, behavior_change=False, found_by="reused", kind=u"보강")

# ───────────────────────────── #75 150000² 경계 (ult 22500000001 = ≤ 150000²) ─────────────────────────────
B_EV = (u"m09.ll:65731 `%43 = icmp ult i64 %42, 22500000001` (closure#2, !55272=goal_data.rs:430) · "
        u"m09.ll:65946 `%84 = icmp ult i64 %83, 22500000001` (closure#4, !55465=:452) — "
        u"dist_sq < 150000²+1 ⟺ dist_sq ≤ 150000². 명세 logic 의 `< 150000²` 는 경계값(정확히 150000)에서 어긋난다")
p.fix(u"/specs[75]/logic", old=u"distance_sq(serpen) < 150000²", new=u"distance_sq(serpen) <= 150000²",
      evidence=B_EV, behavior_change=True, found_by="new", kind=u"실오류")
p.fix(u"/specs[75]/consts[1]/meaning",
      old=u"150000² + 1 — 아군(closure#2 :430)·적(closure#4 내부 :452) 이 세르펜 150k(≈4.7셀) 안이면 in_epic (aux)",
      new=u"임계 150000²+1 — `icmp ult %42, 22500000001`(m09.ll:65731 closure#2 :430) · `icmp ult %83, 22500000001`(m09.ll:65946 closure#4 :452) = dist_sq ≤ 150000²: 아군·적이 세르펜 150k(≈4.7셀) 이내(경계 포함)면 in_epic (aux)",
      evidence=B_EV, behavior_change=False, found_by="new", kind=u"보강")

# ───────────────────────────── #74 heal_commit 극성 (★행동 변경) ─────────────────────────────
HC_EV = (u"m09.ll:4518~4519 `%73 = and i1 %71,%72`(lx≤x≤rx) `br i1 %73, %74, %80` · :4527~4528 `%79 = icmp ult y, ly` `br %79, %80, %89` · "
         u"블록 %80 = 사각형 밖: :4531 `#dbg_value(i8 0, !12362)`(!12362 = DILocalVariable name \"in_heal_area\" line 45) 인데 "
         u":4542~4543 `%88 = icmp ult i64 %87, 36` `br i1 %88, label %100, label %158` 로 hp_ratio<36 이면 그대로 %100 으로 간다 · "
         u"블록 %89: :4555 `%96 = icmp ugt i64 %78, %68`(y>ry = 밖) · :4556 `#dbg_value(i1 %96, !12362, DW_OP_not)`(in_heal_area = !%96) · "
         u":4560~4561 `%99 = and i1 %96, %98` `br i1 %99, label %100, label %158` — 즉 **in_heal_area 가 거짓일 때만** %100 · "
         u":4564~4565 %100 = base_defense_focus → %102 · :4568 `%103 = phi i8 [0,%43],[1,%100],[0,%47],[0,%51]` :4569 `store i8 %103, ptr %41`(%41 = self+240 = heal_commit@0xf0). "
         u"⟹ 힐 커밋 시작 조건은 `!in_heal_area && hp_ratio<36 && base_defense_focus`(우물 **밖**에서 저체력 → 우물 복귀 커밋). 오라클은 base_defense_focus 가 TLS 메모 HashMap(비트 0|16) 경유라 세계 조립이 필요해 미실행(2회 규칙)")
p.fix(u"/specs[74]/logic",
      old=u"if in_heal_area && hp_ratio < 36 && base_defense_focus(player,data) { new = true }   // :47 (&& 단락: base_defense_focus 는 앞 둘 통과 시만 호출)",
      new=u"if !in_heal_area && hp_ratio < 36 && base_defense_focus(player,data) { new = true }   // :47 ★우물 밖에서만(IR %80/%89 분기) (&& 단락: base_defense_focus 는 앞 둘 통과 시만 호출)",
      evidence=HC_EV, behavior_change=True, found_by="new", kind=u"실오류")
p.fix(u"/specs[74]/mem[21]/note",
      old=u"1: !before && in_heal_area && hp_ratio<36 && base_defense_focus(:47)",
      new=u"1: !before && !in_heal_area && hp_ratio<36 && base_defense_focus(:47) — m09.ll:4568~4569 phi [1,%100] · %100 은 in_heal_area 거짓 분기(%80·%89)에서만 도달",
      evidence=HC_EV, behavior_change=True, found_by="new", kind=u"실오류")
p.fix(u"/specs[74]/knobs[0]/effect",
      old=u"올리면 더 높은 HP 에서도 우물 힐 커밋을 걸어 풀피까지 우물에 머문다(본진수비 문맥+우물 안에서만). 내리면 커밋이 드물어진다",
      new=u"올리면 더 높은 HP 에서도 힐 커밋(우물 복귀 후 풀피까지 대기)이 걸린다 — 조건은 본진수비 문맥(base_defense_focus)+**우물 밖**에 있을 때(우물 안이면 커밋을 새로 걸지 않는다). 내리면 커밋이 드물어진다",
      evidence=HC_EV, behavior_change=False, found_by="new", kind=u"실오류")

# ───────────────────────────── D9-OFF: enemy_region 행 base/offset 정규화 ─────────────────────────────
ER_EV = (u"tcxdict GoalData --deep: 0x0 enemy_region[0]@tag(8B) · 0x8 enemy_region[0]@Some.0.region · 0x10 enemy_region[0]@Some.0.last_known (stride 24). "
         u"base `EnemyRegionInfo`(16B: region@0, last_known@8)는 Option 밖 구조체라 tcxaudit 이 오귀속을 냈다 — D9-OFF 규칙 2(컨테이너 원소 → `GoalData.enemy_region[p]`)로 내리면 tcxaudit OK(3/3 실측)")
for spec, rows in ((75, (37, 38, 39)), (76, (26, 27, 28))):
    for j in rows:
        p.fix(u"/specs[%d]/mem[%d]/base" % (spec, j), old=u"EnemyRegionInfo(enemy_region[p])", new=u"GoalData.enemy_region[p]",
              evidence=ER_EV, behavior_change=False, found_by="new", kind=u"보강")
# #74 writes 3행: `0x0 + pos*24` 는 D9-OFF 규칙 1(순수 16진수 하나) 위반 — 원소 기준으로
for j, off_old, off_new, nm_old, nm_new in ((23, u"0x0 + pos*24", u"0x0", u"enemy_region[pos]@tag", u"@tag"),
                                            (24, u"0x8 + pos*24", u"0x8", u"enemy_region[pos].region", u"region"),
                                            (25, u"0x10 + pos*24", u"0x10", u"enemy_region[pos].last_known", u"last_known")):
    p.fix(u"/specs[74]/mem[%d]/base" % j, old=u"GoalData", new=u"GoalData.enemy_region[p]",
          evidence=ER_EV + u" · m09.ll:4786~4790 `%222 = gep { i64, [2 x i64] }, ptr %0, i64 %184` store 1 / +8 %219(region) / +16 %221(tick) · :4828 `store i64 0, ptr %226`(None)",
          behavior_change=False, found_by="new", kind=u"보강")
    p.fix(u"/specs[74]/mem[%d]/offset" % j, old=off_old, new=off_new, evidence=ER_EV,
          behavior_change=False, found_by="new", kind=u"보강")
    p.fix(u"/specs[74]/mem[%d]/name" % j, old=nm_old, new=nm_new,
          evidence=u"#75/#76 의 같은 자리 행과 이름을 맞춘다(G20 R1: 같은 (base,offset) 은 같은 이름) · p = Position::as_index (0..5)",
          behavior_change=False, found_by="new", kind=u"보강")

# ───────────────────────────── 근처 판정 <4 실측 보강 (knob 효과) ─────────────────────────────
NEAR_EV = (u"정적 전역 region_dist(_gcbc/g07.ll:277) 재계산: 대칭 홉거리 행렬 max 6 · region_dist[a][2]<4 인 a = 27개 중 25개(제외 5·20 = Top 라인 원단) · "
           u"region_dist[a][7]<4 인 a = 25개(제외 9·25 = Bottom 라인 원단)")
p.fix(u"/specs[75]/knobs[1]/effect",
      old=u"올리면 ally_near/enemy_near 가 넓게 잡혀 인원 비교(ally_ok) 성립이 쉬워진다",
      new=u"올리면 ally_near/enemy_near 가 넓게 잡혀 인원 비교(ally_ok) 성립이 쉬워진다. ★실측: 기본값 4 에서 이미 27리전 중 25개가 통과(제외 = 5·20, Top 라인 원단)라 사실상 거의 전원을 센다 — 좁히려면 3 이하로",
      evidence=NEAR_EV, behavior_change=False, found_by="new", kind=u"보강")
p.fix(u"/specs[76]/knobs[0]/effect",
      old=u"올리면 ally_near/enemy_near 가 넓게 잡힘 → 인원 우세 조건이 쉬워져 CheckTry 증가",
      new=u"올리면 ally_near/enemy_near 가 넓게 잡힘 → 인원 우세 조건이 쉬워져 CheckTry 증가. ★실측: 기본값 4 에서 이미 27리전 중 25개가 통과(제외 = 9·25, Bottom 라인 원단)라 사실상 거의 전원을 센다 — 좁히려면 3 이하로",
      evidence=NEAR_EV, behavior_change=False, found_by="new", kind=u"보강")

# ───────────────────────────── ev 상향 ─────────────────────────────
GD_EV = u"tcxdict GoalData --deep: 0x0/0x8/0x10 enemy_region[0] tag/region/last_known(stride 24) · 0x78 epic(0x88 epic_enemy_tick · 0xa8 stance) · 0xb0 serpen(0xe0 stance) · 0xe8 last_base_defense_tick · 0xf0 heal_commit — 전행 일치. IR store 전수 확인(m09.ll:4477/4569 heal_commit · :4695 last_base_defense_tick · :4786~4790/:4828 enemy_region · :4711/:4713 update_plan 콜리)"
for j in (18, 19, 20, 21, 22, 23, 24, 25, 26, 27):
    p.ev(u"/specs[74]/mem[%d]" % j, evidence=GD_EV, to=3, frm=4, found_by="reused")
p.ev(u"/specs[75]/mem[26]", evidence=u"tcxdict MapDef 0x54e8 = region_dist[0][2]", to=3, frm=5, found_by="reused")
p.ev(u"/specs[76]/mem[22]", evidence=u"tcxdict MapDef 0x5510 = region_dist[0][7]", to=3, frm=5, found_by="reused")
for spec, rows in ((75, (37, 38, 39)), (76, (26, 27, 28))):
    for j in rows:
        p.ev(u"/specs[%d]/mem[%d]" % (spec, j), evidence=u"tcxdict GoalData --deep enemy_region[0]@tag 0x0 / region 0x8 / last_known 0x10 (base 를 GoalData.enemy_region[p] 로 내려 tcxaudit OK)", to=3, frm=4, found_by="new")
p.ev(u"/specs[75]/consts[3]", evidence=u"IR 정적 전역 _gcbc/g07.ll:276~278 재계산 regions[21][21]=2 · region_centers[2]=(688941,688941)", to=4, frm=5, found_by="reused")
p.ev(u"/specs[76]/consts[1]", evidence=u"IR 정적 전역 _gcbc/g07.ll:276~278 재계산 regions[9][9]=7 · region_centers[7]=(294683,297518)", to=4, frm=5, found_by="reused")
p.ev(u"/specs[74]/consts[6]", evidence=u"IR 정적 전역 _gcbc/g07.ll:276 regions[27][3]=0(블루 넥서스 셀)", to=4, frm=5, found_by="reused")
p.ev(u"/specs[74]/consts[7]", evidence=u"IR 정적 전역 _gcbc/g07.ll:276 regions[3][27]=26(레드 넥서스 셀)", to=4, frm=5, found_by="reused")

# ───────────────────────────── 지시 오류 ─────────────────────────────
p.brief_error(u"§4 G20 R2 지시 「같은 배열의 다른 인덱스면 이름에 인덱스를 붙여 R2 해소」 — 두 행은 이미 `[a][2]`/`[a][7]` 로 인덱스가 붙어 있었다. sharedchk.norm_name 이 첨자 값을 `[]` 로 지우므로 첨자 안 인덱스는 판별자가 못 된다(첨자 밖 접미 `col2`/`col7` 로 우회). 근본 해결은 norm_name 이 **숫자 리터럴 첨자는 보존**하도록 고치는 것(`[a]`→`[]`, `[2]`→`[2]`)")
p.brief_error(u"초점 「#75 클로저 IR 별도 define 6(#2~#7)·exe 주소 5개」 — 메인이 준 5개는 closure#2/#3/#5/#6/#7 이고 **closure#4(s2_0, m09.ll:65779~65952)의 exe 본체 0xdeaf70 이 목록에 없었다**(from_iter_in 인스턴스 0xca89a0 안에서만 호출돼 본체 콜리 스캔에 안 보임). Epic 도 같은 형태로 closure#4 = 0xdeb110(← 0xca8f60). unknown 을 닫으려면 콜리를 2단까지 훑어야 한다")
p.brief_error(u"§1 표 「#74 GoalData::update ev≥4 39행」·「G10/G12/G15/G19 잔여 중 담당 몫」 — specgate --only 72~76 실측 G10/G12/G15/G19 = 전부 0 이라 담당 몫이 없다(초점 문장이 빈 집합을 가리킴)")
p.brief_error(u"§4-b 「mem.dir ✅G14」로 닫혀 있지만 #74 mem[21](heal_commit w) 의 **근거 문면 극성**(in_heal_area && …)은 어떤 게이트도 안 봤다 — dir 은 맞고 조건이 뒤집힌 형태. mem.note 안의 조건식은 무검사 축")

p.save()
print(u"errors=%d ev_up=%d brief=%d warn=%d" % (len(p.errors), len(p.ev_up), len(p.brief_errors), len(p.warn)))
