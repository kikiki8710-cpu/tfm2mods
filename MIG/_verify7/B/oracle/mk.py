# -*- coding: utf-8 -*-
u"""7차 배치B 의 patch.json 생성 — `mkpatch.py` 참조구현을 그대로 쓴다(새로 만들지 않는다)."""
import sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=7, batch="B")

# ══════════ ① 실오류 — `mem.dir` 축(무검사 축)에서 나온 유일한 실오류 ══════════
p.fix("/specs[5]/mem[27]/base",
      old=u"LegacyPlanHandler",
      new=u"V50DiveEpisode(힙 원소)",
      evidence=u"memdircheck(신설) — `LegacyPlanHandler+0x890` 에는 이 함수 범위 안에서 store 가 0건이고 "
               u"load 만 있다. m13.ll:29325 `%128=gep %0,2192` → 29327 load `%129` → 29329 "
               u"`%130 = gep {104B}, ptr %129, i64 %123` 이고 22필드 store 는 전부 `%130+k`(힙) 로 간다",
      behavior_change=False, found_by="new")
p.fix("/specs[5]/mem[27]/offset", old=u"0x890", new=u"0x0",
      evidence=u"쓰기 주소는 힙 원소 시작(+0x00~0x61). `+0x890` 은 그 포인터를 **읽는** 자리이고 "
               u"이미 mem[23](dir=r)로 실려 있다 — 같은 오프셋에 r/w 두 줄이 서 있어 "
               u"재구현자가 핸들러 필드에 쓰는 것으로 읽을 수 있었다",
      behavior_change=False, found_by="new")
p.fix("/specs[5]/mem[27]/note",
      old=u"dive_episode.rs:157~. 필드 대응은 logic 참조",
      new=u"dive_episode.rs:157~. 필드 대응은 logic 참조. ★쓰기 대상은 `LegacyPlanHandler+0x890` "
          u"**자체가 아니라** 거기서 읽은 힙 포인터 + len×104 다. `+0x888/+0x890` 에 store 가 생기는 "
          u"경우는 `RawVec::grow_one` 이 도는 때뿐이고 그건 mem[29] 로 따로 실려 있다",
      evidence=u"memdircheck(신설) 관측: `0x890` = {load} 만, `0x888` = {addr, load}. "
               u"store 는 gep 체인 끝(힙)에만 있다",
      behavior_change=False, found_by="new")

# ══════════ ② `consts.kind` 오분류의 원인 문면 정정 ══════════
#   ★kind 는 손으로 쓰는 값이 아니라 `mkspec3.py:329` 가 **`meaning` 에서 파생**시킨다.
#     그래서 고칠 자리는 `kind` 가 아니라 `meaning` 이다(v2 에는 `kind` 키 자체가 없다).
p.fix("/specs[6]/consts[0]/meaning",
      old=u"(같은 리터럴 2가 team 인덱스 bounds-check 길이로도 쓰임)",
      new=u"(같은 리터럴 2 가 team 배열 bounds-check 길이 `icmp ult team, 2` 로도 쓰인다 — "
          u"**이 행 자체는 version 비교 임계**이지 인덱스가 아니다)",
      kind=u"분류오류",
      evidence=u"kindcheck(신설): IR 관측 = icmp×2 · 인자상수×1 로 **임계**인데 `mkspec3` 이 `meaning` 의 "
               u"「인덱스」 한 단어를 잡아 kind=인덱스 로 파생시켰다",
      behavior_change=False, found_by="new")
p.fix("/specs[7]/consts[1]/meaning",
      old=u"태그 5=Recall · 11=EpicHunt 런타임 확인",
      new=u"SubPlan 5=Recall · 11=EpicHunt 로 귀결되는 것을 런타임 확인(**이 행 자체는 HP 임계**이지 태그가 아니다)",
      kind=u"분류오류",
      evidence=u"kindcheck(신설): IR 관측 = icmp(`icmp ult 51`) 인데 kind=태그 로 앉아 있었다. 원인은 "
               u"5차에 `meaning` 끝에 덧붙은 **증거 문장 속의 「태그」**를 `mkspec3:329` 가 잡은 것 — "
               u"★`applypatch.apply_evup` 이 증거를 `meaning` 에 덧붙이는 설계라 **증거가 분류를 오염시킨다**",
      behavior_change=False, found_by="new")
p.fix("/specs[7]/consts[5]/meaning",
      old=u"AroundBushOutlineType::Outline — Hide",
      new=u"AroundBushOutlineType::Outline 의 **태그값 1** — Hide",
      kind=u"분류오류",
      evidence=u"kindcheck(신설): IR 관측 = store(태그 슬롯)×2 인데 kind=임계 였다. enum variant 값이다",
      behavior_change=False, found_by="new")
p.fix("/specs[8]/consts[0]/meaning",
      old=u"JungleType::Morgard (dienum JungleType 4).",
      new=u"JungleType::Morgard 의 **태그값 4** (dienum JungleType 4). ⚠`src_line=164` 는 맞지만 "
          u"**IR 에 리터럴이 없다** — L164 `take_active(Morgard)` 와 L172 `take_setup_like(Morgard)` 는 "
          u"`icmp eq i8 (TeamPlan+0x41f), 0` 하나로 완전히 접혔고, `i8 4` 가 살아남는 곳은 "
          u"**L169 `camp_pos`(m10.ll:7691) · L174 `v24_…`(m10.ll:7709) 둘뿐**이다.",
      kind=u"보강",
      evidence=u"srclinecheck2(신설) 접힘 판정보류 + irdump --line 164 실측(4명령 전부 `team_plan.rs:231/244` "
               u"프레임을 거쳐 `icmp eq i8 %11, 0` 으로 끝난다)",
      behavior_change=False, found_by="new")
p.fix("/specs[8]/consts[3]/meaning",
      old=u"셀 크기 — 월드좌표를 셀좌표로 나누는 좌표변환(임계 아님).",
      new=u"셀 크기 — 월드좌표를 셀좌표로 나누는 **좌표변환 계수**다(임계가 아니다 — IR 관측도 `udiv` 2회뿐).",
      kind=u"분류오류",
      evidence=u"G1 자기모순 잠복: `meaning` 이 「임계 아님」이라고 쓰는데 파생 `kind` 는 「임계」였다. "
               u"kindcheck 관측 = 계수×2",
      behavior_change=False, found_by="new")

# ══════════ ③ ev 상향 — 오라클 실행 확증 ══════════
p.ev("/specs[8]/knobs[3]", to=2, found_by="new",
     evidence=u"B7_o1.tsv #M 46/46 MATCH — `MainObjective` **12 variant 전수** 스윕(2·3차는 4개만 돌렸다). "
              u"태그 0(Morgard)만 본문을 계속 타고, 1~11 과 None 은 전부 즉시 true "
              u"⟹ 이 플랜이 캠프 상수 4(Morgard)에 고정돼 있음이 실행으로 확정")
p.ev("/specs[8]/knobs[5]", to=2, found_by="new",
     evidence=u"B7_o1.tsv #M — 0→Morgard(계속) / 1 Serpen·2 Defense·3 DefenseLine·4 Nexus·5 PressEpic·"
              u"6 SplitEpic·7 Repair·8 Gank·9 Dive·10 PressTower·11 ComebackPick → **전부 즉시 true** "
              u"⟹ 「0→4 / 1→5 / 그 외 None」 표의 「그 외 None」이 8 variant 에 대해 처음 실행 확인됐다. "
              u"덤: `camp_pos(Morgard, blue)==camp_pos(Morgard, red)==(288000,288000)` (Serpen=(672000,672000))")
p.ev("/specs[9]/consts[0]", to=2, found_by="new",
     evidence=u"B7_o3.tsv — 미니언 18마리를 세운 뒤 유리 대형(rear 아군 1명)을 만들어 `tick_per_second` 를 "
              u"20점 스윕. 09 의 판정 반전점이 **tps 40→41** 이고 `danger` 창 임계가 81 이므로 2×41=82 "
              u"⟹ 계수 **K=2 만 20/20 일치, K=1·3·4 는 기각**. `shl i64 %30,1` 로 접혀 리터럴이 없던 값을 "
              u"실행으로 고정했다")
p.ev("/specs[9]/knobs[5]", to=2, found_by="new",
     evidence=u"B7_o3.tsv 같은 실행 — 조회 구간이 `tick_per_second × 2` 임을 반전점 tps 40/41 로 격리")
p.ev("/specs[9]/knobs[8]", to=2, found_by="new",
     evidence=u"B7_o3.tsv 같은 실행 — `m15.ll:35470 shl i64 %30,1` 의 계수 2 를 실행으로 확정")
p.ev("/specs[9]/knobs[9]", to=2, found_by="new",
     evidence=u"B7_o4.tsv — 창을 tps×2 로 고정하고 `(champion_action, predict_retarget)` 4조합을 09 의 "
              u"실제 판정과 대조: **(true,false) 만 20/20 일치**, (false,false)·(true,true)·(false,true) 전부 기각")

# ══════════ ④ 도시에(지시)의 오류 ══════════
for t in [
    u"§5 의 `patch.json` 스키마가 실제 계약과 다르다 — 도시에는 `{\"entries\":[{path,old,new,why}]}` 로 "
    u"적었지만 `applypatch.py` 가 읽는 것은 `errors[]`(path/kind/old/new/evidence/behavior_change/found_by) 와 "
    u"`ev_up[]` 이다. 도시에대로 냈으면 **전건이 조용히 무시**된다(=5차 317행 유실의 재현).",

    u"§5 의 `\"op\": \"append\"` 는 **존재하지 않는다.** `applypatch.py` 에 `op` 분기가 없다 — "
    u"`notes[]` 에 항목을 새로 추가하는 경로는 현재 계약에 없다(문면 치환만 가능).",

    u"§4-b 가 `consts.kind`(186행) 를 「명세의 축」으로 제시했으나, **`kind` 는 손으로 쓰는 값이 아니다.** "
    u"`mkspec3.py:329` 가 `meaning` 문자열에서 키워드로 파생시키는 값이고 v2 정본(`specs20.json`)에는 "
    u"`kind` 키가 아예 없다 ⟹ `/specs[i]/consts[j]/kind` 로는 **패치가 불가능**하다(「`old` 가 그 키에 없다」). "
    u"고칠 자리는 `meaning` 이거나 `mkspec3` 의 파생 규칙이다.",

    u"§1 의 `ev≥4(미실행)` 집계 스코프가 안 적혀 있다. 실측하면 그 수치(7/5/6/4/5)는 **`consts`+`knobs` 만** "
    u"세고 `open`·`notes`·**`sig.params`** 를 뺀 값이다. 배치 B 담당의 `sig.params[].ev4` 만 **26행**이라 "
    u"체감 분모가 두 배 넘게 다르다(5차 「ev 집계 스코프 미기재」 사고의 재발).",

    u"`applypatch.apply_evup` 이 3단 경로를 못 받는다 — `parse_path` 는 `outer`(`sig`)를 돌려주는데 "
    u"`apply_evup` 이 그걸 **버리고** `resolve(spec, 'params', j)` 를 부른다. 그래서 "
    u"`/specs[i]/sig/params[j]` ev 상향은 항상 「인덱스 범위 밖」으로 실패한다. `apply_error` 는 "
    u"같은 경로를 제대로 처리하므로 **두 함수가 서로 다른 계약**을 쓰고 있다.",

    u"`mkpatch.locate` 의 PATH 정규식이 3단 경로(`/specs[i]/sig/params[j]/role`)를 못 받는다 — "
    u"`applypatch.PATH` 는 받는데 참조구현이 못 받아 배치가 그 자리를 **주소지정조차 못 한다**. "
    u"6차가 `applypatch` 쪽만 고치고 `mkpatch` 를 안 고쳤다.",

    u"★**`applypatch.apply_evup` 이 증거를 `meaning` 에 덧붙이는 설계가 `mkspec3` 의 `kind` 파생을 오염시킨다.** "
    u"07 `consts[1]`(51, HP 임계)이 5차 증거문 「…태그 5=Recall…」 때문에 kind=태그 로 뒤집혔다. "
    u"두 도구가 같은 필드를 반대 목적으로 쓰고 있다 ⟹ 증거는 별도 키(`ev_note`)로 빼거나 "
    u"`mkspec3` 이 `· 오라클 실행 확증(` 이후 꼬리를 **잘라내고** 파생해야 한다.",
]:
    p.brief_error(t)

p.save()
