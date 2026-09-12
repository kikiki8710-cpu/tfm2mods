> ⚠**이 파일은 배치 D 가 직접 못 남겼다.** 그 하네스가 보고서 `.md` 작성을 막아서,
> 배치가 최종 응답으로 돌려준 본문을 **메인이 원문 그대로** 옮겼다(2026-09-11).

---

# 11차 전수 감사 — 배치 D (함수 `15`~`19`) 보고서

> 게임 0.5.8 · 2026-09-11 · 도시에 도장 `2026-09-11 22:12:20`
> 기계 적용분 = `_verify11/D/patch.json` (`errors` 22 · `insert` 6 · `behavior_change` 4 · `ev_up` 0)

## 0. 실제로 실행한 것 (명령줄 그대로)

```bash
cd /c/tfm2mods/MIG
python -X utf8 dossierfresh.py 11 D                     # 착수 / 제출 직전 2회, 둘 다 FRESH

# IR 원문 추출 (담당 5함수 전량)
sed -n '33374,33739p' /c/tfm2mods/_gaibc/m13.ll > _verify11/D/work/f15.ll   # 15
sed -n '51913,52374p' /c/tfm2mods/_gaibc/m10.ll > _verify11/D/work/f16.ll   # 16
sed -n '17695,17825p' /c/tfm2mods/_gaibc/m05.ll > _verify11/D/work/f17.ll   # 17
sed -n '6879,7264p'   /c/tfm2mods/_gaibc/m09.ll > _verify11/D/work/f18.ll   # 18
sed -n '62570,62978p' /c/tfm2mods/_gaibc/m04.ll > _verify11/D/work/f19.ll   # 19

# !dbg 사슬 전개 (inlinedAt 루트까지)
python -X utf8 dloc.py m10.ll 56313 56332 56195 56316 56335 56352 56282 56194 56299 56210 56262
python -X utf8 dloc.py m13.ll 38615 38628 38629 38928 38913 38930 38932 38584 38601 38613 38616 38623 38624 38626
python -X utf8 dloc.py m09.ll 14767 14768 14769 14803 14806 14809 14810 14811 14812 14813 14843 14869
python -X utf8 dloc.py m04.ll 71565 71566 71527 71609 71630 71621 71529 71849 71877 71892 71905 71907 71908 71910 72082 72085 72133 72138 72192 72211 71862 72197

# gep 오프셋 전수 ↔ mem 표 대조
grep -o "getelementptr inbounds nuw i8, ptr %[0-9]*, i64 [0-9]*" f16.ll | sort | uniq -c
grep -n "store\|getelementptr\|memcpy\|memset\|switch\|icmp\|call \|invoke \|phi" f1{5,7,8,9}.ll

# knobs.where 인용 명령 원문 대조
for L in 65185 65219 65254 65289 65324 65338 65390 65424 65459 65494 65529 65128 64600 64391 64471; do sed -n "${L}p" /c/tfm2mods/_gaibc/m09.ll; done
sed -n '7189,7202p;7070,7074p;7098,7101p;7182,7185p' /c/tfm2mods/_gaibc/m09.ll
sed -n '64385,64392p;64465,64472p' /c/tfm2mods/_gaibc/m09.ll
sed -n '60618,60626p' /c/tfm2mods/_gaibc/m14.ll
sed -n '29990,30006p' /c/tfm2mods/_gaibc/m13.ll      # handle_chat_inner 의 chat+0x8 소비
sed -n '130480p'      /c/tfm2mods/_gcbc/g15.ll       # PlayerState::strategy 의 rnd readnone
grep -n "^!9450 = \|^!10463 = " /c/tfm2mods/_gaibc/m10.ll ; grep -n "^!41907 = \|^!28546 = " /c/tfm2mods/_gaibc/m04.ll

PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 11 --only D --dry
```

오라클은 쓰지 않았다. 이번 표적(`closed`·`mem` 누락·`knobs.where` 좌표·`logic` 줄귀속)은 **전부 IR 문면 문제**라 실행으로 갈리지 않는다(`METHOD_MAP ⑥` 한계 3). `16` 의 `max_range_cached` TLS 메모는 **건드리지 않았다** — 이 함수 자체는 TLS 가 아니고(범위 안 threadlocal 접근 0건) 프로브를 짤 이유가 없었다.

---

## 1. 함수별 발견

### `15` single_try_engage — 실오류 1
* **`closed[3]` 이 자기 근거와 정면으로 어긋나 있었다.** 물음은 「남은 미탐색은 `engage_requires_dive` **하나뿐**」(4차 문면)인데, 같은 행의 `why` 가 **「3차 배치D: 합성 엔티티 2,073,600쌍 mismatch 0」** = 바로 그 `engage_requires_dive` 의 검증 결과다. `history[4]`(2차, `Effect::is_in_range_ex` 공식 전문 + 오라클 900/900) · `history[6]`(3차, 676/676 · 진리표 260쌍 · 합성 2,073,600쌍)이 **4차가 이 문장을 쓰기 전에 이미** 닫아 놓은 것이었다.
* mem 19행 · consts 8행 · knobs 9행 · sig.params 8행 IR 전수 대조 → **불일치 0**.

### `16` max_range_nearly_can_use — 실오류 2 · 보강 2
* **`closed[3]` 썩음** — 「`m14.ll:60624` 의 `%33` 이 무슨 값인지는 담당 범위 밖이라 안 봄」이 남아 있는데, `history[4]` 가 이미 `%32 = gep %31, i64 **4856**`(=0x12f8 `GameSetting.tick_per_second`) → `%33 = load` 로 확정했고 IR 원문으로 재확인했다(`%35 = add i64 %34, **20000**` 까지). ★**`closed[6]` 은 「같은 형태 5건을 10차에 전부 정정했다」고 스스로 적어 뒀는데 이 여섯 번째가 목록에서 빠져 있었다** — 도시에가 예고한 「자기가 적어 두고 안 고친」 형태의 실물.
* **`knobs[3].where` 좌표 오류** — `entity.rs:1774/1789/1804` 인데, 인용한 `icmp eq i64 %ty, 13` + `select` 의 `!dbg` 는 `1775`/`1790`/`1805`다. 1774/1789/1804 는 **`fn` 선언 줄**. 같은 표의 `knobs[1]`·`knobs[2]` 는 본문 줄(1693/1701)을 쓰므로 **한 표 안에서 규약이 갈렸다.** `logic` 2곳(1774·1747 → 1775·1748)도 보강.

### `17` DeathMatchBattle::new — 실오류 2 · **행 추가 5**
* **`closed[1]`·`knobs[1]` 이 동시에 낡아 있었다.** 둘 다 「Chat 소비측을 안 봐서 `__1`=0 의 뜻이 미확정」이라는데, 같은 파일 `history[0]` 이 전수 스캔으로 닫아 놓았다. IR 재확인 — `m13.ll:29993 %141 = gep %6, i64 8` → `29994 %142 = load` → `30002` vtable `+0x1f0 get_entity_by_id` 인자. **`chat+0x10` 을 읽는 코드는 0건 = 죽은 슬롯.** ⟹ `knobs[1]` 은 `15 consts[1]`/`15 knobs[4]`(BattlePlanGoal::TryKill.__1)와 **똑같은 「소비처 0건이라 노브가 아니다」** 인데 15 만 정정돼 있었다.
* ★**`mem` 5행 누락 (행 추가)** — `logic`(750줄)·`knobs[1]`(「`m05.ll 17749 store i64 0, ptr %21`」)이 인용하는 **Chat 원소 쓰기 3건이 `mem` 에 통째로 없었다.**

| 추가 행 | IR 원문 |
|---|---|
| `Chat(chats 원소[0], 힙) +0x0` tag=3 (w) | m05.ll:17745 `store i8 3, ptr %19` |
| `Chat … +0x8` `__0`=target (w) | 17746~17747 `%20 = gep %19, i64 8` → `store i64 %13, ptr %20` |
| `Chat … +0x10` `__1`=0 (w) | 17748~17749 `%21 = gep %19, i64 16` → `store i64 0, ptr %21` |
| `Vec<Chat>(지역 24B) +0x10` len=1 (w) | 17750 `store i64 1, ptr %8` (초기 0 = 17707) |
| `Vec<Chat>(지역 24B) +0x8` ptr (r) | 17744 `%19 = load ptr, ptr %7` — **세 원소 행의 좌표 원점** |

  앞 3행은 `behavior_change=true`(10차가 `18` 의 같은 추가를 그렇게 판정한 선례) — mem 만 보고 재구현하면 **채팅 페이로드가 빈다**.
* mem 43행(쓰기 29 사이트 + 읽기 6) 전수 대조 불일치 0. knobs IR 줄 5/5 일치.

### `18` v3_epicops_buff_window — 실오류 1 · 보강 4
* **`mem[16].note` 가 `mem[15]` 의 것을 통째로 달고 있다.** 이 행은 `TeamPlan+0xd0`(Vec 길이)인데 설명은 **Chat 원소 레이아웃**이다. 10차가 한 행을 쪼갤 때 `note` 를 복사하고 뒤쪽을 안 고친 것. IR 상 이 자리엔 `store i8`/`store i64 0` 이 **없고** `%28 = add i64 %19, 1` / `store i64 %28, ptr %18`(m09.ll:6942~6943), `%61 = add i64 %52, 1` / `store`(7016~7017) 뿐이다. `behavior_change=true`(7차 「쓰기 대상 오기 = 소유 구조체 오프셋 ↔ 힙 원소」와 같은 형).
* **`mem[21]` 의 「**공통** usize 슬롯 = `__1`」은 과일반화** — Press/PressChange 기준으로만 맞다. `Chat::Repair(usize)`/`SerpenSetup(usize)` 은 `+0x8` 이 `__0` 이고 `17` 은 같은 바이트를 `__0`(Battle) 이라 부른다.
* **`closed[2]` 잔여 「소비측 미탐색」이 닫혀 있었다** — 17 `history[0]` 전수 스캔이 「태그 3/4/6 에서만 `chat+0x8` 을 읽는다」이므로 21/22/23/25 의 0 은 **아무도 읽지 않는 자리채움**(범위 = `_gaibc`·`_gcbc`).
* **`sig.params[2].role` 이 `closed[5]` 와 자기모순** — 인자표는 rnd 를 「(전략 샘플링용)」이라 하는데 `closed[5]`/`history[5]` 는 `readnone` 이라 RNG 를 안 건드린다고 닫았다. IR 재확인: `_gcbc/g15.ll:130480` `… ptr noalias noundef **readnone** align 16 captures(none) dereferenceable(320) %2 …`.
* **`logic` [2] 블록이 `651~654`** 인데 `chats.push(Chat::SerpenSetup(0))` 은 **658**이다(`!14843` = `ptr::write<Chat>`@mod.rs:1933 ← `Vec::push`@vec.rs:1004 ← **epic.rs:658**). `consts[4].src_line` 은 이미 658 이었다.
* ★**10차가 고친 `knobs[17]`(#dbg 좌표 4줄)·`knobs[12]`(`icmp samesign ugt i8`)는 유지** — 재확인 8/8 원문 일치.

### `19` best_jungle_goal — 실오류 2 · 보강 1 · **행 추가 1**
* ★**`consts[4].meaning` 의 unwrap 줄번호 2개가 둘 다 틀렸다.** 「819/833줄」 → IR 정본 **821/836**: `%71 = icmp eq i8 %70, -1, !dbg !71892` → `unwrap<JungleType>`@option.rs:1011 ← **passive_jungle.rs:821**, `%129 = icmp eq i8 %128, -1, !dbg !72192` → ← **836**. 819/833 은 *키 클로저/체인 시작* 줄이다. 세 번째 unwrap(`choose(rnd).unwrap()` null 검사, `!72211` ← **829**)이 아예 빠져 있어 채웠다.
* ★**`logic` 의 폴백 블록 구조가 DWARF 와 어긋난다.** 명세는 `[818] let mode = …get_game_mode().unwrap(); let runner = &mode.jungle_runner;` 로 **호이스트된 `let` 두 개**를 적는데, vtable `+0x40` 간접호출(m04.ll:62706~62708)의 `!dbg` 는 scope 가 **`closure$2 @passive_jungle.rs:819`** 이고 inlinedAt 루트가 818 이다 ⟹ `get_game_mode()` 는 **키 클로저 안**에 있고 LLVM 이 호이스트한 것뿐이다. 체인 끝 `.unwrap()` 은 **821**. (`get_game_mode` 가 `memory(none)` 이라 `behavior_change=false`.)
* **`logic` `[817] len() == 0` 은 소스상 `is_empty()`** — `!71630` = `len<JungleType>`@vec.rs:1617 ← `is_empty<JungleType>`@vec.rs:1636 ← 817.
* ★**`mem` 1행 누락 (행 추가)** — `Vec<JungleType>` 의 `ptr`(mem[14])·`len`(mem[15])은 있는데 **원소 역참조가 없다**: `m04.ll:62852 %100 = load i8, ptr %91, align 1, !range !41907`(`= !{i8 0, i8 6}`), stride 1B(`62850 %99 = gep %91, i64 1`), `!dbg` 사슬 = `ptr::read<JungleType>` ← `IntoIter::next`@vec.rs:2450 ← passive_jungle.rs:833. **본선 fold 가 실제로 읽는 값이다.**
* **`closed[5]` 의 근거가 엉뚱한 것에 붙어 있다** — §4-2. **계약상 고칠 수 없어** 산문으로만 보고.

---

## 2. 「빠져 있던 것」 — **6건** (전부 `op:insert`)

| 함수 | 행 | 왜 빠지면 안 되나 |
|---|---|---|
| `17` | `Chat(원소[0]) +0x0 / +0x8 / +0x10` (w) | 채팅 페이로드 전체. mem 만으로 재구현하면 **빈 Chat** 이 나간다 (`behavior_change`) |
| `17` | `Vec<Chat>(지역) +0x10 len` (w) · `+0x8 ptr` (r) | 원소 주소의 출처 + 길이 |
| `19` | `JungleType(원소) +0x0` (r) | 본선 fold 의 입력 |

⟹ 10차가 `07`·`08`·`14`·`18` 에서 잡은 **같은 체계적 형태가 `17`·`19` 에 남아 있었다.** 게이트는 구조적으로 못 본다(*있는 칸이 틀렸나*만 본다). `G18` 이 `17` 3행을 잡을 수 있었는데 못 잡았다(검출력 22.1%).

---

## 3. 오탐 반증 — **14건** (적발했다가 IR 원문으로 스스로 기각)

| # | 가설 | 반증 |
|---|---|---|
| 1 | `16 consts[0]` 「CastingType 범위 -1..3」이 mem[17] 「[-1,4)」와 모순 | `!9450 = !{i32 -1, i32 4}`. 「-1..3」을 **포함**으로 읽으면 동일 — 표기 혼재일 뿐 |
| 2 | `sig.params.i` 규약이 15·17(0-based) vs 16·18·19(1-based)로 갈렸다 | 「**1-based 소스 자리, sret = 0**」으로 **5함수 전부 일관**. 내 오독 |
| 3 | `16 knobs[1]/[2]`(1693/1701)이 `history[7]/[8]`(1692/1700)과 모순 | dloc 결과 **본문 줄이 1693/1701 이 맞다**(`!56313`/`!56332`). history 쪽은 `fn` 선언 줄. **표가 옳다** |
| 4 | `16 mem` 31행 누락 의심 | gep 전수 + switch phi 오프셋(176/184/200/208/216/232/240/272/496) → **31/31 일치, 누락 0** |
| 5 | `17 mem` 43행 누락 의심 | store/memcpy/memset **29 사이트 전수** → 43/43 일치 |
| 6 | `15 mem` 19행 | gep 전수 일치(0x1f0·0xf8·0x990·0x930·0x68·0x128·0x660·0x668·0x58·0x8c + 스택 goal 3 + sret 2) |
| 7 | `19 mem` 16행 | 전수 일치(원소 행 1건만 별건) |
| 8 | `18 knobs` IR 인용 8건(6·7·8·9·12·14·16·17) | **8/8 원문 일치** — 10차 정정 유지 |
| 9 | `19 knobs[0]` 좌표 4줄(62593/62595/62597/62599) | `store i8 0/1/3/2` **4/4 일치** |
| 10 | `16 knobs[0]` 호출부 13곳 분포 | 표본 4곳(13159=40 · 15125=50 · 16295=50 · 60624=`%33`) 일치 |
| 11 | `17 knobs[0..3]` IR 줄(17709/17749/17775·17777·17809/17804) | **5/5 일치** |
| 12 | `17 exe=없음` vs `18 exe=dce220`(`define internal fastcc`) 역전 | 그 칸이 「인라인」과 「조인 실패」를 못 가른다고 명세가 이미 명시 — **부재를 결함으로 세지 않는다** |
| 13 | `mem.dir` 무검사 축 재확인 (**115행**) | IR load/store 방향과 **전수 일치, 불일치 0** |
| 14 | `sig.params.role` 무검사 축 재확인 (**30행**) | `define` 의 `sret`/`readonly`/`readnone`/`noalias`/`dereferenceable(N)` 전수 대조 → **불일치 0** |

> **적발 22 / 반증 14 = 오탐률 39%.** 10차 배치C 관측과 같은 방향 — 「게이트가 준 후보」가 아니라 **「IR 원문에서 역으로 찾은 것」이라 8·9차(70~95%)보다 훨씬 낮다.** 반증 14 중 **7건은 「대조했더니 완전 일치」** 로, 이건 결과가 없는 게 아니라 **그 축이 닫혔다는 결과**다.

---

## 4. 내 지시(도시에)·도구의 오류 — 6건

**4-1. ★`closed[]` 는 이 계약으로 주소 지정이 안 된다 (최우선).** 이번 라운드의 주 표적인데 `applypatch.py` 의 `resolve()` 에 `closed` 매핑이 없다. 실측: `/specs[19]/closed[5]/why` → 「인덱스 범위 밖(v3→v2 경계 확인)」. `closed[].q` 는 v2 `unknown`/`still_unknown` 에 있어 **`/specs[i]/open[n]` 문면 경로로 우회**된다(이번 4건을 그렇게 냈다). 그러나 **`closed[].why` 는 v2 에 아예 없다** — `_spec/closelist.py` 의 `CLOSE` 목록에서 파생되므로 `errors[]` 로는 **원리적으로 못 고친다**(`consts.kind` 와 같은 파생 필드 문제). §5 는 이 사실을 말하지 않는다.

**4-2. ★`closelist.closed_reason()` 이 첫 매치를 돌려주어 나중 근거가 영구히 가려진다.** ①`19 closed[5]`(map 클로저 `s_0`): 초판 needle 이 「`resolved[2]`: `_gcbc/g09.ll:137367~137440` 확정」을 주는데 **그 좌표는 `get_camp_state`**(= `history[2]`·`closed[2]`)의 것이고 map 클로저(`from_iter_in` m01.ll 20914~21098)와 **무관**하다. 4차에 올바른 근거 `(19,"map 클로저(s_0)","4차 배치A/C: 항등 사상 확인 기록")` 가 추가됐지만 앞 항목에 가려 절대 안 쓰인다. ②`15 closed[3]`: needle `(15,"engage_requires_dive")` 가 **그 이름을 언급만 하는 다른 항목**에 붙었다. `closelist.py` 자신이 4차에 「needle 은 그 항목의 고유 문면으로 잡는다」는 교훈을 적어 뒀는데 **그 형태가 최소 2건 살아 있다.** `audit()` 은 `dead`/`wrongidx`/`over` 만 보고 **`shadowed`** 를 안 본다. ⟹ `audit()` 에 `shadowed` 추가 + `closed_reason` 을 **마지막 매치 우선**으로.

**4-3. 도시에 §1 표에 `closed` 개수 열이 없다.** 주 표적이 `closed` 인데 규모를 모르고 들어간다(실측 담당분 = 15:7 / 16:7 / 17:8 / 18:10 / 19:7 = **39건**).

**4-4. `errors[].kind` 어휘가 도구와 다르다.** 도시에 = `실오류/오탐/보강`, `applypatch.py` docstring = `실오류|판정반전|분류오류|보강`. **「판정반전」을 낼 칸이 도시에 어휘엔 없다**(§5-b 는 판정반전을 오류로 세라고 한다).

**4-5. §3 분책 인자표 `i` 칸에 규약 주석이 없다.** 실제로는 「1-based 소스 자리, sret=0」으로 일관되는데 **내가 「규약 불일치」로 적발했다가 스스로 반증했다**(§3 #2). 한 줄 주석이면 막힌다.

**4-6. `mem` 의 `ev` 상한 3 규칙과 파생값이 어긋난다.** §5 는 「`mem` 의 `ev` 는 상한 3」인데 `18 mem[18]~[21]` 4행이 **`ev=4`** 다(10차 insert 행들이 `tcx 정본 대조` 문구를 안 달아 `evtier` 가 4 를 준 것). 상한이 규칙이면 `mkspec3` 이 `min(ev,3)` 을 강제해야 하고, 아니면 §5 문구를 고쳐야 한다.

---

## 5. ★10차 대비 — 무엇이 줄고 무엇이 남았나

**줄어든 것**

| 축 | 10차 | 11차 | 판정 |
|---|---|---|---|
| `knobs.where` IR 인용 오류 | **11건** | **1건**(`16 knobs[3]`, 그것도 IR 줄이 아니라 **소스 줄** 오기) | ★**IR 인용은 닫혔다** |
| `mem` 값·오프셋 | `18` 4행 누락 + 값 오류 | **값·오프셋 오류 0**(115행 전수) | ★닫혔다 |
| `mem.dir` | 127행 불일치 0 | **115행 불일치 0** | 재확인 완료 — 게이트 없이 둬도 된다 |
| `sig.params.role` | 30행 불일치 0 | **30행 불일치 0**(+`readnone`·`range` 속성까지) | 재확인 완료 |
| `consts.src_line` | — | **32행 전수 일치** | 닫혔다 |
| 오탐률 | 10차 급감 | **39%** | 「IR→명세 방향」 계속 유효 |

**남은 것 — 성질이 바뀌었다.** 10차의 수확은 「값이 틀린 칸」과 「아예 없는 행」이었다. 11차의 22건 중 **값이 틀린 것은 3건**(`16 knobs[3]` 좌표 · `19 consts[4]` 줄번호 · `18 mem[16]` 잘못 붙은 note)뿐이고 나머지는 전부 **「다른 칸에 이미 답이 있는데 이 칸이 안 따라간 것」**이다:

| 형태 | 건수 | 실례 |
|---|---|---|
| `closed[].q` 가 같은 파일 `history[]` 를 안 따라감 (G17 형) | **4** | `15 closed[3]` · `16 closed[3]` · `17 closed[1]` · `18 closed[2]` |
| 다른 명세가 이미 고친 것을 이 명세만 안 고침 | **2** | `17 knobs[1]`(15 는 정정됨) · `18 mem[21]`(17 과 다른 이름) |
| 같은 파일 안 자기모순 (G1 형) | **1** | `18 sig.params[2]`「샘플링용」↔`closed[5]`「readnone」 |
| `logic` 줄귀속 | **4** | `19` 818/821·is_empty · `18` 658 · `16` 1774/1747 |
| `mem` 행 결손 (10차와 같은 형태) | **6** | `17` 5 · `19` 1 |

⟹ **결론 3가지**

1. **「IR 값이 틀렸나」는 사실상 닫혔다.** 115 mem 행 · 32 consts · knobs IR 인용을 전수로 다시 대조해 나온 값 오류가 3건뿐이다. **다음 라운드에 이 축을 또 훑는 것은 수익이 없다.**
2. **남은 것은 「전파」다.** `history` 가 닫은 결론이 `closed`/`knobs`/`sig.params` 로 안 내려오고, **한 명세에서 고친 것이 같은 사실을 공유하는 다른 명세로 안 건너간다.** 라운드를 더 돌려도 안 줄어든다 — `G17` 은 오탐률 90% 라 느슨하고, **명세 간 전파를 보는 게이트는 아예 없다.** ⟹ **`shared` 사실 키**(Chat 페이로드 · BattlePlanGoal.__1 · AbstractGame vtable 슬롯처럼 **여러 명세가 같은 바이트를 말하는 것**)를 추출해 명세 간 문면을 대조하는 게이트를 제안한다. 이번 22건 중 **6건**이 그 하나로 잡힌다.
3. **`closed[]` 는 10차에 「한 번 훑었다」로 끝나지 않았다.** 10차가 `D` 에서 15건을 고쳤는데 이번에 또 4건이 나왔고, 그중 `16 closed[3]` 은 **바로 옆 `closed[6]` 이 「같은 형태 5건을 전부 정정했다」고 적어 둔 목록에서 빠진 여섯 번째**였다. 원인은 내용이 아니라 **도구**다 — §4-1·§4-2. **이 둘을 고치기 전에는 `closed` 를 또 훑어도 같은 비율로 남는다.**

---

## 6. 판정 어휘로 남기는 것

* **사실 서술** — `mem.dir` 115행 · `sig.params.role` 30행 · `consts.src_line` 32행 · `18 knobs` IR 인용 8행은 **IR 과 전수 일치**한다(0.5.8, `_gaibc` 기준). 물음이 아니니 다시 파지 말 것.
* **표기 불가** — `19` 폴백 체인의 **820줄**에 무엇이 있는지. `!DILocation` 이 820 을 한 번도 안 남긴다(819 키 클로저 · 821 unwrap 만). 줄 길이 산술(`rmeta_srcmap`)은 **미탐색**.
* **미탐색** — `18 open[0]`(version 축, 6차에 2회 반증 실패로 유지) · `15 open[0]`(판별력 있는 세계에서의 `update`/`dive_is_viable` version 축) — 오라클 세계 구성이 필요하고 이번 표적이 아니었다.
* **「재료 부재」가 아니었던 것** — `16 closed[3]` 의 `%33`, `15 closed[3]` 의 `engage_requires_dive` 는 **둘 다 이미 확정돼 있었다.** 「담당 범위 밖이라 안 봄」이라는 문면이 **다음 세션에겐 「재료 부재」로 읽힌다** — 이번에 둘 다 그렇게 굳을 뻔했다.

---

## 7. 다음 라운드 인계

1. `applypatch.py --restamp` **필수** (`op:insert` 6건 — `17 mem` 5 · `19 mem` 1).
2. §4-1·§4-2 를 먼저 고칠 것. 안 고치면 `closed[].why` 는 **몇 라운드를 돌려도 못 고친다.**
3. 게이트 신설 = **명세 간 `shared` 사실 대조**. 이번 22건의 27%를 단독으로 덮는다.
4. 도시에 §1 표에 `closed` 개수 열 추가(담당 39건).
