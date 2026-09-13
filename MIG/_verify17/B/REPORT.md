# 17차 배치 B 보고 — specs[45]~[48] (게임 0.5.8)

신선도: `python -X utf8 dossierfresh.py 17 B` → FRESH(착수 시·제출 직전 2회).
사전검증: `PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 17 --only B --dry` → 정정 26/26 · ev상향 4/4 · 동작변경 0.
patch 생성 = 스크래치 `v17B\mk17b.py`(mkpatch 참조구현). 오라클은 돌리지 않았다(이번 초점 = 게이트 미해소·무검사 축).

## 0. 내 지시(도시에)의 오류 — brief_errors 4건
1. §1·§3 담당이 45~49(5개)인데 호출 메시지는 45~48 — #49 는 이 배치가 손대지 않았다.
2. §4 G12 [47] consts[11](값 2) 「실제 후보」에 진짜 소비 지점 725/745/752(`store i8 2`)가 없다.
3. §4 G15 [47] consts[3] NEG 는 「shl 은 …접힘이지 이 임계가 아님」을 kind 부정으로 읽은 kindchk._NEG 오탐. IR 은 `icmp samesign ult i8 %58, 5`(m09.ll:7370).
4. §4 G12 [47] consts[12](5@697) 는 7분기 합류 꼬리 병합 블록(`!dbg line 0`, m09.ll:7509) — line-0 종결 블록을 phi 선행 블록 줄로 귀속하는 규칙 필요.

## 1. 게이트별 판정
| 게이트 | 함수 | 판정 | 근거 |
|---|---|---|---|
| G4 | #45 8개 | 오탐 — 전부 필드 load. 본문 call = vtable %14/%30×2/distance/battle_recency/%74/panic 뿐 | m04.ll:46702·46721~46723·46782·46797·46843·46871·46873 · closure#0 15558/15568 |
| G4 | #47 10개 | 오탐 — grow_one 만 실제 call(alloc RawVec<Chat>::grow_one), 나머지 필드/dbg/인라인 헬퍼 | m09.ll:7292·7487·7812·7867·7982·8037·8111·8137·7713·7739 |
| G7 | #46 open[1] | 과열림 확정 → shared.is_recent_visible 사실로 문면 교체 | shared + m04.ll:57179~58172 |
| G10 | #46 open[5] | 사실 확인+확장: [0]만 씀(m04.ll:57680) · [0]=TwinA(TowerType 태그 3). 적용 범위: 초기 엔티티 순서(swap_remove g07.ll:172100 재배열 미검증) | g15.ll:72933/73638/73651/73799 · 107508 · 108095 · g09.ll:59116~59117 |
| G10 | #47 open[1] | 사실 확정: minion_count = 우리 라인 미니언 수 − 적 라인 미니언 수(i32) | g11.ll:50017~50026 · g07.ll:157635/157637/157639 |
| G12 | #47 consts[10] 0@719 | 실오류 → 730 (719 는 dbg_value line 0 뿐) | m09.ll:7768 · 7824 · 86239 |
| G12 | #47 consts[11] 2@721 | 실오류 → 725 (+meaning 태그화) | m09.ll:7759 · 7879 |
| G12 | #47 consts[12] 5@697 | 오탐 — 꼬리 병합 블록 line 0. 발화 줄 697·725·730·742·745·749·752 | m09.ll:7507·7509·86036 |
| G15 | #47 consts[3] | 오탐 — 부정의 주어는 shl. 문면만 「별개 상수」로 | m09.ll:7370 |
| G16 | #48 params | 정정 — role 5행에 %0~%5 1:1 명기(focus=%4 tag+%5 id). paramrole 시뮬 0건 | m10.ll:48404·48466·48481 |

## 2. 그 밖에 닫은 open(사실 서술로 교체 — notes 이동은 메인)
- #45 open[0]: L355/L360 can_target 두 호출은 소스에 두 번 적힌 것(둘 다 scope=클로저·inlinedAt 없음, m04.ll:97495/97543, 인자 5개 동일).
- #46 open[2]: tower_pos 6좌표 = MapDef::moba TowerDef Top2/Mid2/Bottom2(태그 5/6/7)와 일치(g09.ll:59116~59117). 넥서스 (96000,864000)/(864000,96000)(g07.ll:152539~152545).
- #46 open[3]: valid_lines 인라인 표(m04.ll:57229~57270) ≡ line_exists 화이트리스트(shared.rule_scope_게이트) — TutorialType 0~8 전부 같은 집합.
- #48 open[0]: camp_pos = camps 에서 ty 일치 첫 CampDef 의 pos[flag?0:1](g07.ll:152585 xor 반전, TLS 메모, 본체 g02.ll:6914~7161). Morgard(288000,288000)·Serpen(672000,672000) x==y 라 team 무관.
- #48 open[3]: exe 0xe0b730 확정(메인 09-13).

## 3. ev 상향 4건
#46 mem[10] 4→3 · #47 mem[15]/[16] 4→3(chk 「확인불가」는 사전 조회 누락 — tcxdict Blackboard 0x20/0x70 즉답) · #48 mem[0] 5→3.

## 4. 실행한 명령
```
python -X utf8 dossierfresh.py 17 B
python -X utf8 specgate.py --only 45|46|47|48
sed -n '46695,46881p' _gaibc/m04.ll | grep -n -E "call|invoke|icmp|getelementptr|load"
sed -n '15501,15584p' m04.ll ; grep -n '^!28023 = \|^!28071 = ' m04.ll
sed -n '57179,58172p' m04.ll | grep -n 'i64 304'
grep -n '^define.*(AbstractGameWithCache3new|new_with_prev_cache|Game15init_twin_tower|World10add_entity|World13remove_entity|MapDef4moba|MapDef8camp_pos|Blackboard6update|BrainMinionParameter6update)' _gcbc/g*.ll
python -X utf8 tcxdict.py Entity 0x128 / --enum TowerType / --enum JungleType / TowerDef / CampDef / BrainMinionParameter / Blackboard 0x20 / PlayerState 0x930 / AbstractGameWithCache 0x130
python -X utf8 rmeta_srcmap.py game_ai "old\epic.rs" 684 760
python(paramrole.check_spec 시뮬레이션) ; python -X utf8 mk17b.py ; applypatch.py 17 --only B --dry
```

## 5. 미탐색으로 남긴 것(범위)
- #46 open[0] nexus can_target=true 조건 — store 지점은 Game::run_tick 의 activate_can_target<closure_env$1..5>(g15.ll:194375~195752, game.rs:2202)로 좁힘. 미탐색 = 그 5개 closure 술어.
- #46 twin_towers 순서의 swap_remove 재배열(초기 이후) — 미검증(런타임 훅으로 twin_towers[team][0].ty 확인 가능).
- #45 open[5] 오라클(pub) 미실행. #47 open[0]/[5]/[6], #48 open[2] 미착수.

## 6. 검사기 제안
- G12: line-0 종결 블록의 store/phi 를 선행 블록 줄 집합으로 귀속(약한 후보) + `store i8 <val>` 도 후보로.
- G15 NEG: 부정 어휘 앞 30자 안에 다른 명령 이름(shl/mul/gep)이 주어면 NEG 내리지 말 것.
- G4: unmatched 이름이 담당 IR 범위의 call/invoke 심볼에 없으면 자동으로 「필드/인라인」으로 접기(#45 8/8, #47 9/10).
