# 12차 전수 감사 — 배치 A(`00`~`04`)

> 배치가 하네스 제약으로 파일을 못 써서 **메인이 반환 원문을 그대로 옮겨 적었다**(가공·요약 없음).

게임 0.5.8 · 정본 `_spec/specs20_v3.json` `4cc9a34ba642ab87` · `dossierfresh 12 A` = **FRESH**(착수 시·제출 직전 2회)

## 1. 정정 건수

| 분류 | 건수 | 내역 |
|---|---|---|
| **실오류** | **6** | `03 mem[13][14][15]/name`(`.0` 누락 3) · `03 mem[8]/name`·`04 mem[6]/name`(팻포인터 절반 미구분 2) · `01 consts[10]/meaning`(IR 줄번호 오기 1) |
| **보강** | **2** | `03 logic`(`is_some()&&unwrap()` → `as_ref().is_some_and(..)`) · `04 logic`(수동 for → `.filter(..).count()`) |
| **행추가** | **2** | `03 mem` at=9 · `04 mem` at=7 — 둘 다 `AbstractGameWithCache+0x8 game.vtable_ptr` |
| **`behavior_change`** | **0** | 6건 전부 오프셋·임계·분기 방향은 이미 맞다. 틀린 건 이름과 인용줄이라 **출력은 같다**. 단 `03` 의 `ty.Champion.skill_cooldown` 은 Rust 로 그대로 옮기면 **컴파일이 안 된다**(없는 필드명) |
| **`ev_up`** | **2** | `02 consts[0]` 4→3 · `03 consts[1]` 4→3 (둘 다 `01 consts[0]` 과 같은 사실·같은 근거인데 등급만 달랐다) |
| **오탐 판정** | **2** | G11 1 · G17 1 |
| **도구 결함** | **7** | §4 |

`applypatch.py 12 --only A --dry` = **정정 10/10 · ev상향 2/2 · 실패 0 · 동작 변경 0건**. (삽입 2건은 `v2 reads[9]`/`reads[7]` WARN — 적용 후 `--restamp` 필요.)

## 2. 게이트 6건 — 각각의 판정

`specgate.py --only N` 실행 결과 6건은 **전부 `03`** 에 있고 `00·01·02·04` 는 20게이트 전부 0.

### ⓐ G11 「ev 지시 미반영」 → **오탐**
`03 history[9]` = 「⟹ `consts[2]`·`consts[3]`·`knobs[1]` **ev 4→2**」는 **3차 시점의 인덱스**다. **11차 배치A 가 `/specs[3]/consts` 에 행을 삽입**했고(`_verify11/A/patch.json` 의 `{"op":"insert","path":"/specs[3]/consts"}` = 값 2 「팀 배열 길이」) 뒤 인덱스가 1 씩 밀렸다. 지시 대상은 **지금의 `consts[3]`(level>2)·`consts[4]`(level>4)** 이고 둘 다 이미 `· 오라클 실행 확증(A6_o5.tsv …)` 으로 **ev=2**. 지시는 이행돼 있다.

현재 `consts[2]`(값 13)는 그 지시의 대상이 아니었고 근거도 IR 뿐이라 **ev=4 가 옳다**:
```
m10.ll:34072  %71 = getelementptr inbounds nuw i8, ptr %70, i64 104      ; Entity+0x68 = ty
m10.ll:34074  %73 = icmp eq i64 %72, 13, !dbg !40848
dloc → entity.rs:1775 skill_cooldown ← buff_value.rs:43 ← 332 any ← 41 defensive_crisis   (src_line 43 ✔)
```

### ⓑ G17 「history 전파」 → **오탐(ⓐ와 같은 원인)**

### ⓒ G20 R1 「Entity 0xb8/0xc0/0xc8」 3건 → **실오류(03 쪽). `16` 이 맞다**
```
$ tcxdict.py --enum EntityType
  페이로드 Champion — enum+0x8   0   game_core::Champion (112B)     ← 필드 이름이 `0`
  페이로드 Minion  — enum+0x8   info game_core::Minion (176B)      ← 나머지 12 variant 는 전부 `info`
$ tcxdict.py Champion → 0x48 skill_cooldown / 0x50 skill2 / 0x58 ult
```
EntityType 14 variant 중 **Champion 만 튜플 variant**라 페이로드 필드명이 `0`. Entity+0x68=`ty` ⟹ 페이로드 Entity+0x70, +0x48/0x50/0x58 = **0xb8/0xc0/0xc8** ✔ (IR `i64 184`@34109 · `192`@34119 · `200`@34127). 오프셋은 맞고 **이름만 `.0` 이 빠졌다**. 함께 묶여 나온 `ty.Minion.info.*`/`ty.Bear.info.*` 는 enum 페이로드 중첩이라 모순 아님.

### ⓓ G20 R2 「`AbstractGameWithCache game` 이 0x0/0x8 양쪽」 → **실오류(표기) + 행 누락**
`tcxdict.py AbstractGameWithCache` → `0x0 game : &dyn AbstractGame (16B)` — **필드는 하나인데 폭이 16B** = 0x0 data / 0x8 vtable. IR 이 두 절반을 따로 읽는다:

| 명세 | IR |
|---|---|
| `03` | `m10.ll:33914 %27 = load ptr, ptr %18`(0x0) · `33915 %28 = gep i8, ptr %18, i64 8` → `33916 %29 = load` — 클로저 환경 `%9+40`(33935)/`%9+48`(33937)에 **따로** 저장 |
| `04` | `m04.ll:58209 %23 = load ptr, ptr %22`(0x0) · `58210~58211`(0x8) → `58212 %26 = gep i8, ptr %25, i64 64`(vtable+0x40 get_game_mode) · `58629` 에서 `PlayerState::strategy` 5번째 인자(`dereferenceable(816)`)로 재사용 |

`04 mem[6]` 의 `note` 는 이미 `+0x0=data(%23), +0x8=vtable(%25)` 라 적고 있었는데 **이름 칸·행 구조에 반영되지 않아** 기계 대조에서 12·18 의 0x8 `game` 과 충돌했다 — §4-b 의 「한 행에 오프셋을 묶으면 기계 검사가 안 된다」 실례이고, 그 규약을 **같은 명세(`04 mem[17][18]`·`mem[23~25]`)가 스스로 적용 중**이었다.
⚠**범위**: `03`·`04` 는 빠지지만 R2 자체는 안 닫힌다 — `06`·`07`·`18`(0x0), `12`·`18`(0x8)이 여전히 그냥 `game` 이다. 적발은 `min(index)`=06 으로 옮겨 간다.

## 3. ★11차가 고친 자리 — **정정 8/8 전부 유효, 그러나 부작용 1건**

| 11차 정정 | 재검증 | 결과 |
|---|---|---|
| `00 knobs[4]/where` → `g06.ll:81878~81879` | `81874 %99 = load i32, ptr %95` / `81878 add nsw i32 %99,-6` / `81879 icmp ult i32 %100,-4` | ✔ |
| `00 knobs[5]/where` → `g02.ll:306881 ret i64 1` | `306881: ret i64 1`(`TargetProjectileAction…cooltime_use_count`) | ✔ |
| `00 mem[11]/note` in-함수 인용 | `44251 load` · `44262 sext` · `44283 load` · `44294 sext` | ✔ 4줄 정확 |
| `03 knobs[5]/where` `:33`→`:34` | `!40950`(34265~34269) = `vec.rs:547 new_in ← buff_value.rs:34` · 호출줄 `!40949` = `:33` | ✔ |
| `03 knobs[16]/where` → `52708~52709/52842~52843/52641/52829` | `52641 add 100` · `52707~52709 mul100/udiv/umax(,1)` · `52829 add 100` · `52841~52843` 동형 · `52844 add %259,%190`(AD+AP) | ✔ 6줄 |
| `00 unknown`×2 · `03 no_map_reason` · `04 unknown` · `03 consts` 삽입 | 문면·IR(`33899 icmp ult %21,2`, `!40699` = `simulation.rs:1905 ← buff_value.rs:22`) | ✔ |

**부작용 = 「정정은 오류를 옮기지, 없애지 않는다」의 정확한 재현.** 11차의 유일한 행 삽입이 `history[9]` 산문의 `consts[2]`·`consts[3]` 문자열 참조를 엉뚱한 행으로 밀었고, 그 결과 12차 G11·G17 이 **없는 오류 2건을 적발**했다. 11차가 고친 자리가 다시 틀린 건 아니지만 **11차의 정정이 12차의 오탐 2건을 만들었다.**

## 4. 도구 결함 · 내 지시 오류

**① `applypatch` 의 행 삽입이 산문 속 인덱스 참조를 갱신하지 않는다** (G11/G17 영구 오탐의 원인). `--restamp` 는 도장만 다시 잡고 `resolved`/`unknown` 본문의 `consts[N]` 문자열은 손대지 않는데 G11/G17 은 그걸 파싱한다. 고침 둘 다 필요: (a) `insert` 시 같은 spec 의 `resolved`/`unknown`/`still_unknown` 본문에서 `<field>[k]`(k≥at) → `[k+1]` 치환 + WARN 기록, (b) G11/G17 이 **인덱스가 아니라 함께 적힌 근거 문자열**(`level>2`·`cool<=tps` 등)로 매칭하고, 인덱스 참조뿐이면 **적발이 아니라 `⚠확인 필요`** 로 내릴 것.

**② `mkspec3._rank_callees` 의 앵커가 맨 부분문자열이라 접두 이름이 거짓 `ev3`** —
```python
r["_anchor"] = bool(segs) and any(all(s in sym for s in segs) for sym in irsy)
```
`['game_core','Effect','range']` 가 `…6Effect12range_adjust` 에 전부 부분문자열로 들어간다. ⟹ **`00 callees[20] game_core::Effect::range` 가 「IR 호출 심볼 일치」(ev3) 도장을 받았는데 `ult` 본문에 그 호출이 없다**(호출 심볼 12개 전량 열거로 확인 — `effect_range_with_radii` 안에 인라인). 고침: v0 망글링은 `<길이><이름>` 이므로 `all((str(len(s))+s) in sym for s in segs)`.

**③ 같은 함수 — 경로에 `::<'a, 'b>` 가 끼면 절대 앵커 안 된다**(참 `ev3` 2건이 잠겨 있었다):

| 행 | 현재 | 실제 IR | 판정 |
|---|---|---|---|
| `03 callees[14] …::<'a,'b>::player_by_champion_id` | ev4 | `m10.ll:33983 %48 = call fastcc …21AbstractGameWithCache21player_by_champion_id` | **ev3 이어야** |
| `04 callees[1] …::<'a,'b>::champions` | ev4 | `…21AbstractGameWithCache9champions` 직접 호출 | **ev3 이어야** |
| `03 callees[9] …::<'a,'b>::iter_champions` | ev4 | `from_iter_in` 제네릭 인자 속 `…14iter_champions0`(클로저) | ev3 아님(정당) |

**④ 같은 함수 — 제네릭 인자 속 클로저 경로가 「호출」로 읽혀 자기호출을 날조한다**(2건): `03 callees[1] game_ai::defensive_crisis` **ev3**(앵커 = `…from_iter_in…NC…buff_value16defensive_crisis0E…` = 자기 클로저) · `04 callees[7] …handle_line_defense` **ev3**(앵커 = `min_by_key…NC…handle_line_defense0E…`). 고침: `_RI<base><generic args>E` 에서 **첫 `I` 앞의 base 만** 대조. ②③④ 를 함께 고치면 배치 A 의 `callees` 86행 중 오판 6건(거짓 ev3 3 · 놓친 ev3 2 · 클로저 1)이 닫힌다.

**⑤ ★지시 오류 — `callees` 는 `patch.json` 으로 주소지정 자체가 안 된다.** 도시에는 `callees` 를 주 표적으로 지목하면서 산출물을 `patch.json` 하나로 못 박는데, `applypatch.resolve()` 에 `callees` 분기가 없다(v2 키 `calls`, v3 키 `callees`, `V2KEY` 에도 없음 ⟹ `spec.get("callees")` = `None`). 게다가 `callees` 는 매 빌드마다 `mkspec3` 이 재생성하므로 정본을 고쳐도 날아간다. **이 표적은 `_rank_callees` 수정으로만 닫힌다** — 지시문이 그 사실을 적어야 배치가 헛돌지 않는다.

**⑥ 지시 오류 — §4 게이트 상세가 문장 도중에 잘려 있다**: `…★**src_line 정정 : 43**(`icmp eq i` 에서 끊긴다. §3 은 「표의 어떤 칸도 `...` 로 줄이지 않았다. 줄인 자리가 곧 지시 오류가 난 자리였다」고 선언하는데 §4 상세는 그 보장 밖이다.

**⑦ 지시 오류 — `kind` 어휘가 도구 안에서 갈린다**: `applypatch.py` L21 = `실오류|판정반전|분류오류|보강`, 같은 파일 L544 = `실오류/오탐/보강`, 도시에 §5 = 후자. 검증은 안 하지만 집계가 갈린다. L21 을 L544 에 맞춰라.

## 5. 전수 감사 — 무엇을 몇 행 대조했나 (게이트가 안 보는 칸 포함, IR→명세 방향 포함)

| 축 | 행수 | 결과 |
|---|---|---|
| `mem.offset` | **108** | **불일치 0**. `04` 의 0x180/0x190/0x1a0/0x1b0/0x1c0/0x1d0 은 gep 리터럴이 아니라 `m04.ll:58353~58354 phi i64 [416,%48],[448,%49],[384,%47]` / `[432],[464],[400]` 으로 들어 있다(METHOD_MAP 함정 ⑤) |
| `mem.name` | 108 | tcx 대조 → **실오류 5**(§2ⓒⓓ) |
| `mem.dir` | 108 | **불일치 0**. `00` store 5건 전부 sret(`w` 3행 ✔) · `01` store 0 (`writes` 빈 배열 ✔) · `02` store 4건 전부 sret(`w` 4행 ✔) · `03`·`04` store 는 전부 alloca ⟹ `writes` 빈 배열이 맞다. **이 축은 닫아도 된다** |
| **`mem` 누락** | — | **IR→명세 방향에서 2건 발견** — `AbstractGameWithCache+0x8`(03·04). 행추가 반영 |
| `consts.value` | **41** | IR 리터럴(icmp·switch·phi·select·store) 전량 → **누락 0 · 과잉 0** |
| `consts.src_line` | **41** | `dloc.py` 로 `inlinedAt` 루트까지 전개 → **불일치 0**. 예: `00` `44025→!56400=entity.rs:1701←286` · `44200→!56503=data.rs:122←entity.rs:1483←328` · `44246→!56528=191` · `44313→!56544←!56369=333` / `02` `!62937=36`·`!62985=vec.rs:1636←45`·`!62989=48` / `03` `!40848=entity.rs:1775←43`·`!40855=1693←44`·`!40868=1701←45`·`!40819=35`·`!40892=48` / `04` `!66990=runner.rs:263←rule_scope.rs:46←549`·`!67475=572 closure$2←…←filter.rs:142 count←573`·`!67551=578` |
| `consts.kind` | 41 | **전부 타당 · `미상` 0건**. 11차 대비 바뀐 유일 행 = `00 consts[9]` 150000 `미상`→**`오프셋가감`**, 소비처가 `llvm.usub.sat.i64(%205,150000)`(총사거리에서 빼는 여유분)이므로 **맞는 분류** |
| `knobs.where` | **49** | 인용 IR 줄 전건 열람 → **불일치 0**(`01` 40043/40066/40091/40059~63 · `04` 58200/58583/58635/58636/60449/60455 · `03` 34019/34144/34081/34086/34265~69 + `g06.ll` 6줄) |
| `callees` | **86** | IR 호출 심볼 전량 열거 후 재앵커 → **오판 6**(§4-②③④, 통로 없어 도구안으로 보고) |
| `callers` | **12사이트** | 독립 grep 재현 → **완전 일치**(`00` m07:12776 / `01` m02:40037·40081·40124 / `02` m02:8484 / `03` m05:43115·43804·44220 / `04` m09:9206·9272·9353 + m13:31086) |
| `sig.params` | **34** | `define` 헤더 속성 대조 → **불일치 0**. `i` 규약(sret=0, 소스 1..n)도 5함수 전부 준수 |
| `one_line`·`layer` | 10 | **불일치 0** |
| `closed[].why` | **38** | **뒤집힌 것 0**(1건은 §6 에서 더 강한 근거로 승격 가능) |
| `history` | **35** | 인용 IR 전건 → **불일치 0**. `03 history[0]` aux: `55883 max_range` · `55916 %29 = add i64 %9, 30000` · `55948 is_enemy_well_danger` · `55966 is_recent_visible`, 그리고 `55934~55945` 가 `c.team.tag==0 && c.team.payload == 1 - player.info.team`(`%40 = sub i64 1, %39`)로 `is_ignored_well_enemy` 서술과 정확히 일치 |
| `logic` | 5 | 분기 방향·순서·상수 전부 일치, **보강 2**. 검증 예: `00` 의 `champ.team 태그==Neutral 이면 가시성 검사 생략`(`44173 load ptr %21` → `44174 trunc nuw i64 %123 to i1` → `br %143/%128`) · `01` 의 `min(coef, coef*value/hp)+based`(`40050 mul`→`40093 sdiv`→`40096 llvm.smin`→`40097 add`) · `02` 의 샘 사각형 4비교 + `phi [5,%49],[2,%61],[16,%55]` · `04` 의 `select %169(Battle), (near-1)<u2, near>u1` |

## 6. `open` 3건 판정 (`patch.json` 으로 못 쓰는 칸 — 메인이 반영해 달라)

**`04 open[0]`(`rnd` 가 `PlayerState::strategy` 안에서 소비되나) → 해소 · 사실 서술.** 현재 본문은 6차 오라클 실측에 기대며 스스로 「상태 1종에서만 잰 값이라 일반화하지 말 것(`TeamColorStrategy_random` 경로가 있다)」이라 좁혀 뒀는데, **그 제한은 더 필요 없다**:
```
_gcbc/g15.ll:130480
define void @…11PlayerState8strategy(ptr … sret([24 x i8]) %0,
    ptr … readonly … dereferenceable(2528) %1,
    ptr noalias noundef **readnone** align 16 … dereferenceable(320) %2,   ; &mut StdRng
    ptr noundef nonnull %3, ptr … readonly … dereferenceable(816) %4)
```
`%2` 가 **`readnone`** = 어떤 경로에서도 역참조하지 않는다(컴파일러가 전 경로에 대해 증명). 호출 심볼도 `m04.ll:58629` 가 정확히 이 define. 같은 방법으로 `02` 의 `rnd`/`_team_plan`/`_debug`(`m12.ll:34867` 의 `%3`·`%6`·`%7`)와 `04` 의 `_debug`(`%5`)도 `readnone` 재확인.

**`03 open[0]`(`m01.ll:37431~37606` 미탐색) → 해소 · 사실 서술.** 읽었다. `bumpalo Vec::from_iter_in<Filter<FilterMap<slice::Iter<Option<&Entity>>, iter_champions::{closure#0}>, defensive_crisis::{closure#0}>>` 의 제네릭 본체다: `37454~37465` 슬라이스 begin/end + 어댑터 상태 복사 · `37487 %25 = load ptr, ptr %23` / `37492 icmp eq ptr %25, null` = **`filter_map(id)` 의 None 건너뛰기** · `37502 invoke …defensive_crisis::{closure#0}…call_mut` = 필터 클로저(본체는 이미 `history[0]` aux `m10.ll:55868~55976` 로 펴 놓음) · `37517` 루프 종료 · `37534` → `reserve_internal_or_panic` push. **판정 상수 0 · 새 오프셋 0 · 새 노브 0** ⟹ 명세에 추가할 것 없음. `open` 에서 내려 `notes` 로.

**`02 open[0]`(`version` 이 무엇을 게이트하나) → 미탐색 유지 · 범위 명시.** 「이 함수에 버전 분기가 없다」는 **확정**(IR 에 비교 0건). 「version 이 전역적으로 무엇을 게이트하나」는 이 함수의 물음이 아니다 ⟹ 물음을 「이 함수에서 version 이 쓰이는가」로 좁히면 즉시 닫히고, 넓은 물음은 버전 게이트 보유 함수로 옮겨야 한다(`00 closed` 가 같은 물음을 `safe_move_avoiding_enemy_well` 의 `m04.ll:43914 icmp ugt i64 %1, 1` 로 닫은 선례).

## 7. 실행한 명령 (재현용)

```bash
cd /c/tfm2mods/MIG
python -X utf8 dossierfresh.py 12 A                  # 착수 전 · 제출 직전 (둘 다 FRESH)
python -X utf8 _verify12/A/dump.py <i> <field>       # v3 명세 칸 덤프(신규)
python -X utf8 _verify12/A/irscan.py 0 1 2 3 4       # IR gep/리터럴/store ↔ 명세 양방향(신규)
python -X utf8 _verify12/A/calleechk.py 0 4          # callees 앵커 재판정 · 길이접두 엄밀판(신규)
python -X utf8 tcxdict.py --enum EntityType ; tcxdict.py Champion ; tcxdict.py AbstractGameWithCache
python -X utf8 dloc.py m10.ll 40892 40903 40944 40819 40848 40855 40868
python -X utf8 dloc.py m04.ll 66990 66984 67475 67551
python -X utf8 dloc.py m12.ll 62937 62970 62985 62986 62988 62989 62968 62969
python -X utf8 specgate.py --only 0|1|2|3|4
PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 12 --only A --dry
awk 'NR>=A && NR<=B {printf "%d: %s\n",NR,$0}' /c/tfm2mods/_gaibc/mNN.ll   # IR 원문은 전부 절대 줄번호로 확인
```

산출물: `_verify12/A/patch.json` · 보조 스크립트 `dump.py`·`irscan.py`·`calleechk.py` · 추출물 `ir_00_ult.txt`·`ir_03.txt`·`ir_04.txt`·`calleechk.out`. (`REPORT.md` 는 하네스가 막아 파일로 남기지 못했다 — 이 응답이 전문이다.)
