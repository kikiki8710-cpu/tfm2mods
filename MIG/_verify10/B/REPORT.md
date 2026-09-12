> ⚠**이 파일은 배치 B 가 직접 못 남겼다.** 그 하네스가 보고서 `.md` 작성을 막아서,
> 배치가 최종 응답으로 돌려준 본문을 **메인이 원문 그대로** 옮겼다(2026-09-11).
> `patch.json` 과 보조 산출물은 배치가 직접 디스크에 썼다.

---

# 10차 배치 B 보고 — 담당 `05`~`09` 전수 감사 (게임 0.5.8)

신선도 `python -X utf8 dossierfresh.py 10 B` → **FRESH** (착수 시 · 제출 직전 2회).
사전 검증 `PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 10 --only B --dry` → **정정 14 성공 / 0 실패 · 동작 변경 0건 · found_by new=14**. ⚠삽입 8건이 있으므로 **적용 후 `--restamp` 필수**.

## §0 방법 — 「IR→명세」 방향

함수마다 담당 IR 줄범위를 통독(`#dbg_value` 제거)하고 ①모든 gep/load/store 오프셋을 16진 변환해 `mem` 과 **양방향** 대조 ②본문 전 리터럴 ↔ `consts` ③`knobs[].where` 가 가리키는 IR 줄을 **원문으로** 확인 ④모든 `!dbg` 를 `dloc.py` 로 `inlinedAt` 루트까지 ⑤`closed`/`notes`/`one_line`/`callees`/`siblings`/`callers` 를 `_spec/specs20_v3.json`·`_tcx`·IR 전량 grep 과 대조.

실제로 돌린 명령(그대로):
```bash
cd /c/tfm2mods/MIG
python -X utf8 dossierfresh.py 10 B
awk 'NR>=28946 && NR<=29380 {printf "%d: %s\n", NR, $0}' /c/tfm2mods/_gaibc/m13.ll | grep -v "#dbg_"   # 05
awk 'NR>=45212 && NR<=45626 {printf "%d: %s\n", NR, $0}' /c/tfm2mods/_gaibc/m13.ll | grep -v "#dbg_"   # 06
awk 'NR>=41916 && NR<=42000 {printf "%d: %s\n", NR, $0}' /c/tfm2mods/_gaibc/m12.ll | grep -v "#dbg_"   # 06 closure$0
awk 'NR>=48918 && NR<=49140 {printf "%d: %s\n", NR, $0}' /c/tfm2mods/_gaibc/m02.ll | grep -v "#dbg_"   # 07
awk 'NR>=7655  && NR<=7899  {printf "%d: %s\n", NR, $0}' /c/tfm2mods/_gaibc/m10.ll | grep -v "#dbg_"   # 08
awk 'NR>=35410 && NR<=35852 {printf "%d: %s\n", NR, $0}' /c/tfm2mods/_gaibc/m15.ll | grep -v "#dbg_"   # 09
grep -n "^@anon.282069a2ed2ad3a275929b639963fb55\.\(92\|93\|94\|51\|52\|50\|89\|48\|53\) " /c/tfm2mods/_gaibc/m13.ll
python -X utf8 dloc.py /c/tfm2mods/_gaibc/m13.ll 35976 36000 36002 36003 35929 35937 35942 35951 35953 35955 35963 35968 36162 36163 36165 36175 36262
python -X utf8 dloc.py /c/tfm2mods/_gaibc/m13.ll 35999 36013 36030 36047 36064 36081 36097 36114 36131
python -X utf8 dloc.py /c/tfm2mods/_gaibc/m13.ll 48765 48766 48771 48750 48745 48779 49049 49084 49107 49113 49116 49141 49146 49151 49152 49153 49155 49156 49158
python -X utf8 dloc.py /c/tfm2mods/_gaibc/m02.ll 54426 54437 54444 54446 54404 54455 54475 54476 54508 54432 54512 54514 54516 54525 54529 54538 54425 54539 54540 54542 54544
python -X utf8 dloc.py /c/tfm2mods/_gaibc/m10.ll 19220 19222 19223 19225 19226 19211 19239 19241 19184 19179 19275 19279 19281 19254 19288 19321 19322 19323 19367 19369 19127 19371 19378
python -X utf8 rmeta_srcmap.py game_ai dive_episode.rs 125 175
python -X utf8 rmeta_srcmap.py game_ai fight_check.rs 1196 1350
cd /c/tfm2mods/_gaibc && for f in m*.ll; do grep -c "NtB2_21EpicHuntAndBattlePlan8sub_plan" $f; done
PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 10 --only B --dry
```

## §1 함수별 발견

### `05` v50_fold_dive_episode — 이번 라운드 최대 수확(판정반전 1)

**불일치 0인 축**: `mem` 30행이 IR 고유 접근 25 read + 5 write 와 **1:1(누락 0·잉여 0)**. 레코드 22필드 `+0x00~+0x61` ↔ `LegacyPlanHandler+0x5c8~+0x5e5 / +0x1811 / 인자` 대응이 store 22개와 전건 일치. A/B/C 3블록, end_plan 9단 분류 순서, `take()` 의 무조건 `store i64 -1`, `len==cap → grow_one` 전부 일치. `calls` 8개 = IR 실제 call/invoke 8개와 정확히 일치. `callers` 2곳(23653·23856) 확인.

**① ★판정반전 — `knobs[1].where` 의 「확정」이 거짓**
`knobs[1]` 은 `131 (확정 — 132~137 은 한국어 주석 6줄이라 코드가 없다)`, `closed[0]` 은 「131 인지 132~137 의 중첩 if 인지 **확정 못 함**」 — 두 칸이 정면 모순인데 **G1 이 못 잡았다**(서로 다른 낱말). 줄길이 산술 결과 **`closed[0]` 쪽이 옳다**:

| 근거 | 실측 |
|---|---|
| 눈금 | `L125=112B` ↔ `pub(crate) fn v50_fold_dive_episode(&mut self, _version: usize, _tps: usize, aborted: bool, end_reason: u8) {` = 들여쓰기 2 + 109자 = **111자 ±0** ⟹ 들여쓰기 단위 2 · fn 본문 4 |
| L131 용량 | `59B = 58자`. `if let Some(live) = self.v50_dive_ep_live.as_ref() {` 만으로 56자. `&& live.in_range_ticks == 0` 을 붙이면 **공백을 다 지워도 67자** (`self.v50_dive_ep_live` 21자·`in_range_ticks` 14자는 tcx 고정) ⟹ 어떤 철자로도 안 들어간다 |
| 중첩 깊이 | `L139=43B=42자` = 들여쓰기 **8** + `if no_contact && end_reason != 7 {`(34자) **±0** |
| 닫는 괄호 | `L141/142/143 = 10/8/6B` = 닫는 중괄호 **3개**(들여쓰기 8/6/4) |
| ⟹ | L131(4)과 L139(8) 사이에 **들여쓰기 6 블록이 하나 더 열린다** = `in_range_ticks==0` 은 **독립 중첩 if**, **132~137 중 한 줄** |
| 어느 줄? | 132~137 은 전부 mb>0(코드+한국어 주석). 필요한 ASCII 39자를 담을 수 있는 건 **L134(43)**·차순 L135(38). L132 33·L133 30·L136 29·L137 28 은 부족 |

IR 도 정합: in_range 의 `%9 = load`/`%10 = icmp eq`(m13.ll:28990~28991)에 **`!dbg` 가 없고**(SimplifyCFG 투기) 뒤 `select` 만 `!35976`=131 을 문다 ⟹ IR 만으론 못 정한다는 원래 판정이 옳았다. 판정 어휘 = **미탐색**(범위 = 132~137 중 어느 줄, L134 최유력). `logic` 에도 「소스는 `&&` 하나가 아니라 중첩 if 둘」을 명시(의미 동일 → `behavior_change=false`).

**② 실오류** — `closed[0]` 의 `option.rs:742 is_some` → 실제 **`as_ref`**(dloc 실측).

**③ ★빠져 있던 것 — `consts` 패턴 길이 6개**: 11/13/12 셋만 싣고 **8(LineGank)·4(Epic)·6(Serpen)·6(Recall)·6(Battle)·5(Nexus)** 를 빠뜨렸다. 전부 IR 에 `i64 noundef N` 리터럴로 있고 `@anon.…` 전역 배열 길이와 일치하며 `!dbg` 루트가 각각 148~153.

### `06` v2_response_retreat_stance — **불일치 0. 깨끗하다**

`mem` 13행 = IR 고유 접근 13종과 완전 일치. 게임 구조체 store 0(스택 alloca 뿐) ⟹ `writes` 가 빈 것이 맞다. `consts` 4행(2@14·1@20·3@36·4@38) 전부 dloc 루트와 일치. `knobs[0].where`(45232 `icmp ult i64 %0, 2`)·`knobs[4].where`(45596~45600 빈 Vec 조립) 원문 일치. `closure$0`(m12.ll:41916~41998)을 직접 열어 술어 순서·인자·캡처 슬롯 6개까지 `logic` 과 전건 일치 확인. `check_kill_die_tick` 인자 8개 순서와 `die > tps`(`icmp ugt %118,%122`) 일치.

**남은 미탐색 1건인데 `open` 이 비어 있다(과소열림).** `history[5]` 가 살아 있는 미탐색(「관측된 RunAway 가 06 의 반환인지 가르기」, 범위 = `AttackEffect` 조립으로 `die > tps` 성립 → KitingBack 유도)을 적는데 `open` 은 0건. 판정 = **미탐색**, 이번 라운드 **오라클 미착수**(이유: `TEMPLATE.rs` 함정 ③ TLS 메모 = 케이스당 프로세스 1개 + 함정 ④ `AttackEffect` 직접 조립이 동시에 필요해 새 하네스가 필요하고, 착수했으면 나머지 4함수 전칸 대조를 못 끝냈다). ★구조적으로 이건 `G7`(과열림)의 정반대인 **과소열림**이고 어떤 게이트도 안 본다. 게다가 `applypatch` 는 `open`/`notes` 에 `op:insert` 를 **못 한다**(`resolve()` → `(None,None)`).

### `07` sub_plan — `mem` 한 칸이 통째로 빠져 있었다

불일치 0: `logic` 6단 전부, 오프셋 28행 중 27행, `consts` 7행 전부, sret 6곳 store, vtable 0x40/0x1f0, `MobaMode 0x1a0/0x1a8`, `GoalData 0x98/0xa0`, `MapDef 0x6d70`(사각형 4좌표 x∈[f.0,f.2] ∧ y∈[f.1,f.3]) 전건 일치. `callers` 1곳은 `_gaibc` 전량 grep 재확인, `siblings` 8개 누락 0.

**★빠져 있던 것 — `live_list[0]` 역참조**: `mem[8]`=.ptr, `mem[9]`=.len 인데 **포인터를 역참조해 id 를 읽는 접근이 표에 없다** — `m02.ll:49015 %52 = load i64, ptr %51` → `get_entity_by_id(%52)`(49018). `logic` 은 이미 `live[0]` 이라 쓰고 있어 **G18 이 잡았어야 할 형태**인데 앵커(배열 첨자 산문)가 안 걸렸다. D9-OFF 규약대로 `usize(live_list 힙 원소)/0x0` 로 삽입.

**실오류 — `knobs[6].where` 가 2줄 어긋난다**: `m02.ll:49081 (icmp eq epic.hp, epic.max_hp)` 인데 49081 은 `gep … i64 1576` 이고 실제 `icmp eq` 는 **49083**. **G13 의 ±2 허용이 오기를 덮은 실례.**

### `08` is_end — 07 과 똑같은 누락 1건

불일치 0: 오프셋 21행 중 20행, `consts` 6행, `knobs[0].where`(7812 `icmp ugt %85, 22500000000`)·`knobs[1].where`(7888 `mul i64 %121, 15`) 원문 일치, vtable 0x28/0x40/0x100/0x1f0, `TeamPlan 0x41f/0x420`, `MobaMode 0x1a0/0x1a8/0x1b0`(432=0x1B0 확인), 종료 경로 (a)~(g) 7가지와 phi(7816·7821) 대응 전건 일치. `%5`(team_plan) store 0건 재확인.

**★빠져 있던 것 — `live_list[0]` 역참조**(m10.ll:7852 `%100 = load i64, ptr %99`). **07 과 같은 결손이 같은 자리에서 반복** ⟹ 개별 실수가 아니라 **「Vec 의 ptr/len 은 싣고 원소 역참조는 안 싣는」 체계적 누락**이다(담당 5함수 중 힙 원소 행이 있는 건 05 `writes` 뿐이었다).

### `09` check_favorable_engage_formation — 불일치 0 · `open[0]` 실제 진전

`mem` 14행 = IR 고유 접근 14종 완전 일치. `consts` 9행 전부(접힘 2건 포함 — `shl i128 %176,2`@35776, `shl i64 %30,1`@35471 로 `notes[0]` 서술 확인). 카운터 3개 phi 귀속(`%80`=rear/`%79`=flank/`%88`=front)을 증가 지점 5곳에서 역추적해 **분기 순서(front→rear→flank→else front)와 `logic` 전건 일치**, 최종 `phi i1`(35639) 6갈래도 반환표와 일치. `knobs[7]`(9차에 채워진 미니언 임계표) 자리에 있음.

**★`open[0]` 진전 — 줄길이 산술 1차 실행**(6차엔 재료만 확보하고 「아직 대조 안 함」이었다):

| 얻은 것 | 근거 |
|---|---|
| **`fight_check.rs` 들여쓰기 단위 = 2칸**(새 사실) | L1211/L1319=4B(3자)=`  }` · L1242=6B(5자)=`    }` · L1241=16B(15자)=`      continue;` · L1210=18B(17자)=`    return false;` · L1318=17B(16자)=`    return true;` · L1233=19B(18자)=`  for ap in 0..5 {` **±0** |
| 조건부 순수 길이 | **L1316 16자 · L1321 38자 · L1331 17자** |
| ★**L1321 은 `logic` 철자 그대로가 아니다** | 세 줄이 같은 꼴 `if X <비교> {` 라면 `len(front)=len(rear)+1` 과 1321 식이 동시에 성립해야 하는데 **`2·len(flank)=3`** 모순. 들여쓰기 항이 소거되므로 **들여쓰기 가정과 무관한 모순** |

판정 = **미탐색 유지**(범위 축소). ⚠의미는 건드리지 않았다 — 4차 배치B 독립재구현 **1800/1800** 으로 논리값은 확정이고 **철자만의 문제**(추론을 기각에 쓰지 않았다).

## §2 「빠져 있던 것」 — **8건**

| # | 어디 | 무엇 | 왜 게이트가 못 봤나 |
|---|---|---|---|
| 1 | `07 mem` | `live_list[0]` 힙 원소 read | G18 앵커가 `live[0]` 산문을 못 잡음 |
| 2 | `08 mem` | `live_list[0]` 힙 원소 read | 같은 결손 반복 |
| 3~8 | `05 consts` | 패턴 길이 6개(8/4/6/6/6/5) | 게이트는 **있는 칸이 틀렸나**만 본다 |

(+ 구조적 결손 1건: `06 open` 과소열림 — **도구가 막아** patch 불가, 산문으로만 보고)

## §3 스스로 반증한 오탐 — **4건**

| 후보 | 왜 버렸나 |
|---|---|
| 「`05 closed[0]` 이 낡았다 — `knobs[1]` 이 이미 확정했으니 닫아라」 | ★**처음 이 방향으로 갈 뻔했다.** 줄길이 산술을 돌리니 **정반대**였다(`closed[0]` 이 옳고 `knobs[1]` 이 거짓). 반증을 먼저 안 했으면 **판정을 거꾸로 뒤집을 뻔했다** |
| 「`08`/`09 consts` 에 `1 - team` 의 `1` 이 빠졌다」 | `SPEC_GUIDE` §constants 경계표 = **「배열 인덱스·stride 는 적지 않는다」** ⟹ 08·09 가 맞다(다만 06 은 싣고 있어 양식이 갈림 → brief_errors) |
| 「`08 mem[18] MobaMode 0x1b0` 이 틀렸다」 | `gep … i64 432` = 0x1B0 ⟹ 맞다. 10진→16진 확인 전에 의심했던 것 |
| 「`09 consts` 에 분수 중심 `/2` 가 빠졌다」 | `lshr i64 %48,1` 로 리터럴 2 가 없고, 중심계산은 좌표변환 계수가 아니다 |

## §4 내 지시(도시에·지시문)의 오류 — **6건**

1. ★**`mkdossier.py` L225 가 `sig.params` 를 `("i","name","ty","role")` 로 렌더하는데 v3 키는 `type`** ⟹ 담당 5함수 인자표 **31행 전부 타입 공란**. 이번 임무표가 `sig.params[]` 를 G16 대상으로 지목했는데 **그 칸이 도시에에서 물리적으로 안 보였다**.
2. ★**`mkdossier.py` L266 이 `siblings.entries` 를 `("name","sym","note")` 로 렌더하는데 실제 키는 `path`/`vis`/`at`/`mir`/`sig`** ⟹ **형제 표 전 행이 공란**(05·06 41행, 07 8행, 08 6행). 임무표가 지목한 축인데 도시에만 보고는 확인 불가였다(나는 v3 JSON 직독으로 대조 — **05~09 누락 0**).
3. ★**도시에 §5 의 「⚠새 항목 추가(`append`)는 구현이 없다」가 거짓** — `applypatch.py` L239~279 에 `op:insert`/`op:delete` 가 있다. **지시문 본문과 도시에 §5 가 한 라운드 안에서 반대**다.
4. ⚠**`op:insert` 를 `open`/`notes` 에는 못 쓴다** ⟹ 06 의 과소열림을 기계로 못 고친다.
5. ⚠**지시문이 「`06` 전 행의 『직접 호출 재료 부재』」를 미해결 `open` 으로 인용**했는데 ①06 의 `open` 은 **0건**이고 ②그 판정은 `history[5]` 에서 **이미 판정반전으로 뒤집혔다**(`update` pub 경로로 6차에 실행됨).
6. ⚠**`applypatch --dry` 로그가 삽입 경고를 「★적용 실패 N건」 블록에 넣는다** — 내 patch 는 요약이 「정정 14 성공/0 실패」인데 그 위에 「★적용 실패 8건」이 찍혀 **되돌릴 위험**. 덤으로 그 경고의 「이후 N행 이동」이 v2 배열 기준이라 **과소계상**(`/specs[7]/mem at=22` 를 「0행 이동」이라 하지만 v3 에선 writes 6행이 밀린다).

**판정 반전도 오류로 센다** ⟹ 이번 라운드 판정반전 **1건**(05 `knobs[1]`).

## §5 patch 집계

- `errors` **14건** (실오류 3 · 판정반전 1 · 보강 10 — 그중 **삽입 8**: mem 2 · consts 6), `ev_up` 0, `brief_errors` 8.
- **`behavior_change` = 0건.** 담당 5함수의 판정 로직·임계·분기 순서·반환 태그는 IR 과 전건 일치했고, 정정 14건은 전부 ①소스 줄 귀속 ②표 결손 보충 ③미탐색 범위 정정이라 재구현 결과를 바꾸지 않는다. 9차 결론(「검사받은 자리는 더 이상 오류를 안 낸다」)과 정합하며, **이번 것은 전부 「검사받지 않는 축」과 「아예 빠진 칸」에서 나왔다**.

## §6 다음 라운드에 넘기는 것

| 무엇 | 어떻게 |
|---|---|
| ★**G20 후보 — 과소열림** | `history`/`closed` 에 「미탐색/남은/아직」이 있는데 `open` 이 그 항목을 안 실으면 경고(`G7` 의 정반대, 현재 무검사) |
| ★**G21 후보 — 컨테이너 역참조 결손** | `mem` 에 `*.ptr` 과 `*.len` 이 같이 있는데 **ptr 을 역참조하는 load 에 대응하는 행이 없으면** 경고(07·08 에서 같은 결손) |
| ★**렌더러 2건 수리** | `mkdossier.py` L225 `ty`→`type`, L266 `("name","sym","note")`→`("path","vis","at")`. **고치기 전까지 `sig.params`·`siblings` 축은 배치에 전달되지 않는다** |
| `05` L132~137 확정 | exe 디스어셈 또는 식별자 후보 + 정밀 줄길이 산술(들여쓰기 2 확정으로 재료가 강해졌다) |
| `06` 오라클 | `AttackEffect`(72B 전 필드 pub) 조립 → `die > tps` → `update` 경유 KitingBack. ⚠`check_kill_die_tick` 은 TLS 메모 = **케이스당 프로세스 1개** |
| `09` L1321 철자 | 들여쓰기 2 + 조건부 38자 제약을 식별자 길이 후보와 결합 |
