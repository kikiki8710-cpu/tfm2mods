# -*- coding: utf-8 -*-
u"""7차 배치 D 의 patch.json 생성 — `mkpatch.py`(정본 참조구현)를 import 해서 쓴다."""
import sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import mkpatch

p = mkpatch.Patch(round=7, batch="D")

# ══════════════════════════════════════════════════════════════════
# ① G12 실오류 2건 — IR !dbg inlinedAt 사슬을 직접 열어 확인
# ══════════════════════════════════════════════════════════════════
p.fix("/specs[16]/consts[0]/src_line", old="2401", new="2399",
      evidence=u"m10.ll 51936/51954/52106/52205 `icmp eq i32 %x, -1` 의 !dbg 사슬 = "
               u"[option.rs:742 <- battle.rs:2399/2407/2414/2421]; (level-1) 의 `add i64 %x, -1` 은 "
               u"[effect.rs:26 <- battle.rs:2403/2410/2417/2424]. battle.rs:2401 에는 어떤 "
               u"!DILocation 도 매핑되지 않는다(본문 참조 줄 집합 = "
               u"[2399,2402,2403,2407,2409,2410,2414,2416,2417,2421,2423,2424,2429]). 첫 사이트 2399 로 정정.",
      behavior_change=False, found_by="inherited", force=True)   # ★force 이유 = 아래 주석(도구 계약 충돌)

p.fix("/specs[18]/consts[4]/src_line", old="654", new="658",
      evidence=u"m09.ll:7013 `store i8 25, ptr %59` 의 !dbg !14843 사슬 = "
               u"[mod.rs:1933 <- mod.rs:1045 <- mod.rs:1004 <- **epic.rs:658**]. "
               u"654 는 `objective = Some(Serpen{..})` 의 세 스토어(m09.ll:6977/6979/6981, !dbg !14813 = "
               u"epic.rs:654)가 쓰는 줄이다. history 의 줄 복원표(658 = chats.push(Chat::SerpenSetup(0))) "
               u"와도 일치 — 표만 옛 값이었다(G8 형).",
      behavior_change=False, found_by="inherited", force=True)
# ★★`force=True` 의 이유 — **mkpatch 와 applypatch 의 계약이 정수 필드에서 서로 어긋난다.**
#   mkpatch.fix  : 값 필드는 `tgt == old` 동등 비교 ⟹ `old` 가 **int** 여야 통과(문서 예시도 int)
#   applypatch.already : `(e.get("new") or u"")[:80]` ⟹ `new` 가 int 면 **TypeError 로 죽는다**
#   ⟹ 둘 다 만족하는 타입이 없다. 여기서는 문자열로 내고(applypatch `conv` 가 int 로 되돌린다)
#      mkpatch 쪽 검증만 force 로 넘긴다. 정본 현재값(2401 / 654)은 손으로 확인했다.

# ══════════════════════════════════════════════════════════════════
# ② mem.dir 축 — 처음 검사된 축에서 나온 실오류 1건
# ══════════════════════════════════════════════════════════════════
p.fix("/specs[18]/mem[15]/base", old=u"TeamPlan", new=u"Chat(chats 버퍼 원소, 24B)",
      evidence=u"IR 방향표(memdir.py): (%0, 200=0xc8) = **'r' 뿐**. m09.ll:7211 `%141 = load ptr, ptr %140` "
               u"(TeamPlan+0xc8 읽기)로 버퍼 주소를 얻고, 쓰기는 `%142 = getelementptr {i8,[23 x i8]}, ptr %141, "
               u"i64 %152` 가 가리키는 **버퍼 원소**에 들어간다(m09.ll:7213/7215/7217, 세르펜 경로는 7013/7015, "
               u"수리 경로는 6955/6957). TeamPlan+0xc8 자체에는 store 가 0건이다.",
      behavior_change=False, found_by="new")
p.fix("/specs[18]/mem[15]/offset", old=u"0xc8", new=u"0x0",
      evidence=u"버퍼 원소 기준 태그 오프셋. 같은 원소의 LineType 은 +0x1, 공통 usize 는 +0x8.",
      behavior_change=False, found_by="new")
p.fix("/specs[18]/mem[15]/name", old=u"chats.ptr(push 대상 버퍼)",
      new=u"Chat 태그 (Repair 23 / SerpenSetup 25 / Press 21 / PressChange 22)",
      evidence=u"m09.ll 6955·7013·7213 의 `store i8 <태그>, ptr %<원소>`.",
      behavior_change=False, found_by="new")
p.fix("/specs[18]/mem[15]/note",
      old=u"원소 24B. 태그 @+0, Press/PressChange 는 LineType @+1, 공통 usize 필드 @+8 에 항상 0 을 넣는다",
      new=u"원소 24B. 태그 @+0, Press/PressChange 는 LineType @+1, 공통 usize 필드 @+8 에 항상 0 을 넣는다. "
          u"★**베이스는 TeamPlan 이 아니라 힙 버퍼다** — TeamPlan+0xc8(chats.ptr)은 이 함수에서 **읽기 전용**이고"
          u"(쓰기 0건) 그 읽은 포인터가 가리키는 원소에 쓴다. 필드 쪽 읽기는 mem[8] 이 이미 담고 있다",
      evidence=u"memdir.py 오프셋 방향표: (%0,192)=r · (%0,200)=r · (%0,208)=r,w · (%142,0/1/8)=w.",
      behavior_change=False, found_by="new")

# ══════════════════════════════════════════════════════════════════
# ③ consts.kind 축 — kind 는 `meaning` 문면에서 파생되므로 meaning 을 고친다
#    (kind 를 직접 쓰면 v2 에 그 키가 없어 **no-op** 이다)
# ══════════════════════════════════════════════════════════════════
p.fix("/specs[17]/consts[3]/meaning",
      old=u"Option None 니치 태그", new=u"Option None 니치 센티널",
      evidence=u"같은 대상(Option 의 None 니치)을 15 consts[4]·16 consts[0]·18 consts[7] 은 '센티널' 로, "
               u"17 consts[3]·19 consts[4] 는 '태그' 로 불러 kind 가 갈렸다. mkspec3.py:329 의 분류 우선순위가 "
               u"'태그' > '센티널' 이라 문면에 '태그' 가 섞이면 센티널이 태그로 앉는다. 오라클 D7_o1 실측: "
               u"Option<TowerType>::None = [255](1B) — variant 판별자가 아니라 니치 센티널이다.",
      behavior_change=False, found_by="new")
p.fix("/specs[19]/consts[4]/meaning",
      old=u"Option<JungleType>::None 의 니치 태그(=255)",
      new=u"Option<JungleType>::None 의 니치 센티널(=255)",
      evidence=u"오라클 D7_o1: `Option<JungleType>` size=1, None bytes=[255]. "
               u"Some(Rhino)=[0]/Some(Mushroom)=[1]/Some(Bee)=[3]/Some(Stump)=[2] 와 겹치지 않는 니치값이다. "
               u"17 consts[3] 과 같은 분류 표류.",
      behavior_change=False, found_by="new")

_JT = u"★`JungleType` 의 **판별자**다(임계가 아니다). 오라클 D7_o1 실측 태그 = Rhino 0 · Mushroom 1 · Bee 3 · Stump 2"
p.fix("/specs[19]/consts[0]/meaning",
      old=u"jungle_camps[0] = JungleType::Rhino (dienum JungleType 0=Rhino)",
      new=u"jungle_camps[0] = JungleType::Rhino — " + _JT + u" (dienum JungleType 0=Rhino)",
      evidence=u"consts.kind 축 검사(constkind.py): 19 의 캠프 4값이 전부 `임계` 로 앉아 있었다. "
               u"mkspec3 이 meaning 에서 '태그/판별자/variant' 를 못 찾아 기본값 `임계` 로 떨어뜨린 것. "
               u"IR 용법도 STORE/GEP 뿐이고 REL(임계 비교)이 하나도 없다.",
      behavior_change=False, found_by="new")
p.fix("/specs[19]/consts[1]/meaning",
      old=u"jungle_camps[1] = JungleType::Mushroom",
      new=u"jungle_camps[1] = JungleType::Mushroom — " + _JT,
      evidence=u"위와 같음. IR 용법 = GEP,STORE (REL 0건).",
      behavior_change=False, found_by="new")
p.fix("/specs[19]/consts[2]/meaning",
      old=u"jungle_camps[2] = JungleType::Bee",
      new=u"jungle_camps[2] = JungleType::Bee — " + _JT,
      evidence=u"위와 같음. 오라클이 Bee=3 을 직접 읽어 '2보다 3이 먼저' 주석을 실행으로 확증했다.",
      behavior_change=False, found_by="new")
p.fix("/specs[19]/consts[3]/meaning",
      old=u"jungle_camps[3] = JungleType::Stump",
      new=u"jungle_camps[3] = JungleType::Stump — " + _JT,
      evidence=u"위와 같음. 오라클 Stump=2.",
      behavior_change=False, found_by="new")

# ══════════════════════════════════════════════════════════════════
# ④ G8(표에 옛 값) — history 가 닫은 것이 표에 '미확정' 으로 남아 있다
# ══════════════════════════════════════════════════════════════════
p.fix("/specs[15]/mem[15]/note", old=u"의미 미확정 — unknown 참조",
      new=u"★**소비처 0건 — 노브가 아니다**(history 참조). 생성 14곳 전부 리터럴 60이고 "
          u"`main_goal.__1`(goal+0x10) 을 읽는 코드는 `BattlePlanGoal::Debug::fmt` 하나뿐이다",
      evidence=u"history[1] 이 이미 '소비처가 존재하지 않는다 — 노브가 아니다' 로 닫았는데 표에 옛 문면이 "
               u"남아 있었다(G8 형, 게이트가 못 잡음 — G8 은 `logic`↔표만 본다).",
      behavior_change=False, found_by="inherited")
p.fix("/specs[15]/knobs[4]/effect", old=u"미확정 — unknown 참조. ",
      new=u"★**산술 소비처 0건이라 사실상 노브가 아니다**(history 참조 — 읽는 코드는 Debug::fmt 뿐). ",
      evidence=u"history[1] 과 같은 근거. `knobs` 에 남겨 두되 '바꿔도 동작이 안 바뀐다' 를 명시한다.",
      behavior_change=False, found_by="inherited")

# ══════════════════════════════════════════════════════════════════
# ⑤ 19 knobs[6] — 오라클 이분탐색으로 계수·짝 매핑 확정
# ══════════════════════════════════════════════════════════════════
p.fix("/specs[19]/knobs[6]/effect", old=u"두 캠프 연속 클리어 판단의 이동 예산",
      new=u"두 캠프 연속 클리어 판단의 이동 예산. ★**실측 공식**(오라클 D7_o2, 1단위 이분탐색): "
          u"`is_side_cleared(c)` 의 경계 = `max( dist(c) + tps + 1 , dist(짝(c)) + 6*tps + 1 )` "
          u"— 즉 자기 캠프는 `is_cleared` 와 같은 1초 여유, **짝 캠프에는 5초가 더 붙어 6*tps** 가 된다. "
          u"★**짝 매핑 = Rhino↔Bee · Mushroom↔Stump**",
      evidence=u"D7_o2.exe tps=30/60/90 × 캠프 4 = 12칸. "
               u"Rhino(dist 496257) 경계 496288/496318/496348 = dist+tps+1 (자기 조건이 지배) · "
               u"Bee(343722) 496438/496618/496798 = Rhino_dist + 6*tps + 1 · "
               u"Mushroom(365557) 529875/530055/530235 = Stump_dist + 6*tps + 1 · "
               u"Stump(529694) 529725/529755/529785 = dist+tps+1. 손계산 12/12 일치, 오류 0.",
      behavior_change=False, found_by="new")

# ══════════════════════════════════════════════════════════════════
# ⑥ 16 logic — 바로 옆 `max_range_cached` 가 TLS 메모라는 경고(프로브 함정)
# ══════════════════════════════════════════════════════════════════
p.fix("/specs[16]/logic", old=u"주의 3) 함수 전체가 순수 읽기다",
      new=u"주의 5) ★**측정 함정** — 이 함수 자체는 TLS 메모가 아니지만(범위 51913~52374 안에 "
          u"threadlocal 접근 0건), **같은 모듈의 `max_range_cached`(m10.ll:51072)가 "
          u"`MAX_RANGE_CACHE`(thread_local RefCell<MaxRangeCache>, m10.ll:143/144, 1624B)로 "
          u"이 함수의 결과를 메모한다.** `SinglePlanBattle::update_v32:505` 의 "
          u"`dist_sq > max_range_cached²` 게이트를 오라클로 재려면 **한 프로세스 = 한 케이스**여야 한다"
          u"(TEMPLATE.rs 함정 ③). 이 함수를 직접 부르는 오라클은 안전하다.\n"
          u"주의 3) 함수 전체가 순수 읽기다",
      evidence=u"tlsscan.py TLS 전역 목록 + `grep MAX_RANGE_CACHE m10.ll` → 소비자는 "
               u"`LocalKey<RefCell<MaxRangeCache>>::with::{closure}` 를 부르는 `max_range_cached` "
               u"(m10.ll:51072, anon....169). mem 방향표에도 이 범위에 threadlocal 접근이 없다.",
      behavior_change=False, found_by="new")

# ══════════════════════════════════════════════════════════════════
# ⑦ ev 상향 — 근거만 준다(숫자는 mkspec3 가 파생시킨다)
# ══════════════════════════════════════════════════════════════════
EV_NICHE = (u"오라클 D7_o1 실행 확증 — `Option<TowerType>::None` size=1 bytes=[255], "
            u"`Option<SinglePlanBattle>` size=144 = inner size 144(니치라 오버헤드 0)이고 "
            u"None 의 +0x0 8바이트를 i64 로 읽으면 **-1**. IR 의 `dereferenceable(144)`/"
            u"`memcpy .. i64 144`/`store i64 -1, ptr %0` 과 완전 일치")
p.ev("/specs[15]/consts[4]", to=2, evidence=EV_NICHE, found_by="new")
p.ev("/specs[15]/mem[16]", to=3, evidence=EV_NICHE, found_by="new")
p.ev("/specs[15]/mem[17]", to=3, evidence=EV_NICHE, found_by="new")
p.ev("/specs[15]/consts[2]", to=2,
     evidence=u"오라클 D7_o1 실행 확증 — `cache.iter_towers_without_nexus(1 - team)` 을 team 0/1 양쪽에서 "
              u"돌려 **각각 8개, 8/8 전부 적팀(`Entity.team == Player(1-team)`), 좌표 유일 8**. "
              u"무결성 지표 towers=16 · twin 2/2 (TEMPLATE ② 오염 없음) — `1 - team` 의 극성이 실행으로 확정",
     found_by="new")
p.ev("/specs[19]/knobs[6]", to=2,
     evidence=u"오라클 D7_o2 1단위 이분탐색 tps 30/60/90 × 캠프 4 = **12/12 손계산 일치**(위 effect 의 공식)",
     found_by="new")
p.ev("/specs[19]/consts[2]", to=2,
     evidence=u"오라클 D7_o1 직독 — `Option<JungleType>` Some(Bee) bytes=[3] (Rhino 0 · Mushroom 1 · Stump 2). "
              u"기존 근거였던 '236/236' 은 `best_jungle_goal` end-to-end 일치라 태그값 자체의 증거가 아니었다",
     found_by="new")
p.ev("/specs[19]/consts[3]", to=2,
     evidence=u"오라클 D7_o1 직독 — Some(Stump) bytes=[2]. 배열 순서 Rhino,Mushroom,Bee,Stump 가 "
              u"저장 태그 0,1,3,2 와 일치함을 실행으로 확인",
     found_by="new")

p.brief_error(u"§5 의 patch.json 스키마가 틀렸다 — `entries[]`/`why` 로 적혀 있으나 `applypatch.py` 가 "
              u"읽는 키는 **`errors[]`(old/new/evidence/kind/behavior_change/found_by)** 와 **`ev_up[]`** 다. "
              u"§5 대로 냈으면 이번 배치 보고가 통째로 no-op 이었다(5차 317행 유실과 같은 형태).")
p.brief_error(u"§5 의 `\"op\": \"append\"` 는 `applypatch.py` 에 구현이 없다(`apply_error` 는 치환 전용). "
              u"`notes` 추가는 기존 항목 문면을 확장하는 방식뿐이다.")
p.brief_error(u"주 표적으로 지목된 **G13 은 배치 D 에 0건**이다(corpus 8건 전부 specs 00/03/04/14). "
              u"도시에 §4 도 G13 절을 싣지 않았는데 지시문만 G13 을 1순위로 지목했다.")
p.brief_error(u"도시에 §4 의 머리 숫자(`G12 47건` · `G9 4건`)는 **corpus 총계**이고 그 아래 목록은 배치 D 몫"
              u"(10건 · 1건)이다. '이 배치 몫만 추렸다' 와 숫자가 어긋난다.")
p.brief_error(u"§4 가 준 「실제 후보」 목록은 신뢰할 수 없다 — `srclinecheck` 의 리터럴 정규식이 "
              u"SSA 레지스터(`%3`·`%21`·`%22`)·`align 4`·`dereferenceable(N)` 까지 긁는다. "
              u"18 consts[5]/[6] 의 '실제 후보 = [638]' 은 `%21`/`%22` 노이즈였다.")

p.save()
