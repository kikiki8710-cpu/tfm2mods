# 4차 반증검증 — 배치 C (판단함수 10~14) / 게임 0.5.8

작업 파일 = `C:\tfm2mods\MIG\_verify4\C\` (`C4_a.rs`/`C4_b.rs`/`C4_c.rs` + `.tsv`,
`C4_closeaudit.py`, `C4_g7.py`). `_spec\`·`_verify\`·`_verify2\`·`_verify3\` 는 **읽기만** 했고
`specs20_v3.json` 직접 수정 없음.

## 0. 결론 한 줄

**값·구조·시그니처·오프셋·분기극성이 스펙과 어긋난 사례 = 0건.** 억지 정정 없음.
대신 **「이미 닫힌 질문이 open 에 남아 있는」 과열림 12건**과 그 **기계적 원인(도구 결함 4종)** 을 찾았고,
**3차의 최대 잔여였던 `region_point` 산출식을 오라클 27/27 로 닫았다.**

| 구분 | 건수 |
|---|---|
| 과열림(open→closed) | **12건** (11:3 · 12:6 · 14:3) |
| 내가 새로 닫은 것 | **4건** (13 open[0] · 13 open[3] · 10 open[2] · 14 open[2] ev5) |
| 재분류(어휘) | 3건 (10 open[0]·10 open[1]·14 open[1]) |
| 도구 결함 | **4종** (closelist 침묵 no-op · 과닫힘 · specgate G7 부재 · G3 하드코딩) |
| 신규 사실 | region_point 산출식 · is_end 소비규칙 · target_bush 의 blackboard 사용 · vtable 슬롯 Game impl 대조 |
| 값 오류 | **0건** |

`ev<=3` 표본 재확인: 오프셋 21건(`tcxdict` 질의) 전건 일치 · 11 open[2](ev3)는 **뒤집힌 게 아니라 이미 닫혀 있었다** ·
13 open[0](ev3)은 **간접증거 → 직접증거로 승격**. **사고 0건.**

`callees_unmatched` 판정 술어 혼입 **0건**(10 필드명 2 / 11 필드명 3 / 12 std 2 / 13 가드 1 / 14 인트린식·가드 4).

---

## 1. 도구 결함 (이 배치의 1순위 과제)

### 1-0. 3차가 고친 2건은 제대로 고쳐졌다 (확인)
- `siblings` 망글링 파싱: **12/20 검출, 8/20 `plan=null`.** 그 8개는 전부 심볼이 `_RNvNt…`(자유 함수)이고
  검출된 12개는 전부 `_RNvM…`(inherent impl 메서드) — **20/20 정확**. 내 배치 13/14 도 정상
  (`LineGankCoverPlan` 10 / `LineGankerPlan` 14, `target_bush_v30:248`·`target_bush_v41:310`·`target_bush:351` 포함).
- `specgate` G3 화이트리스트: `FREE_FN={0,1,3,4,9,10,16,19}` 가 위 8개와 정확히 일치. 현재 G1~G6 **전부 0**.
- `mem[].chk`: 내 배치 5개의 `mem` 89행 중 `확인불가(vtable 슬롯)` 3행 외 전부 `OK`, `조회실패` 0.

### 1-1. `closelist.py` 의 needle 이 0건 매칭이면 조용히 no-op 이다 (감사 도구 없음)
`closed_reason(i, txt)` 는 `j==i and needle in txt` 부분문자열 매칭이고, **아무것도 안 맞아도 아무 말이 없다.**
`siblings` 망글링 파싱 실패·G3 `plan==null` 통과와 **정확히 같은 부류의 침묵 실패**다.
감사 스크립트를 만들어 전수 확인했다(`C4_closeaudit.py`) — **CLOSE 103개 중 10개가 아무것도 닫지 않았다**:

| 종류 | 건수 | 실측 |
|---|---|---|
| **index 오기**(다른 index 에서 매칭) | 4 | `(8,"is_recent_visible 본체")`→실제 6·10 / `(8,"15")`→3·4·9·11·13·14 / `(13,"bushes")`→**14** / `(18,"Prepare")`→17 |
| 문면 불일치(어디에도 0건) | 6 | `(2,"buf.a")` `(3,"check_kill_die_tick 반환값")` `(4,"Blackboard 배열을")` `(6,"별개 클로저")` `(12,"position_exists")` `(19,"SliceRandom")` |

내 배치의 `/specs[14]/open[0]`(부시 사전)이 아직 열려 있는 직접 원인 = `closelist.py:172` 의 index 오기(13→14) 다.
3차 리포트 §2.14 의 제목이 「(14 open[1])」인데 등록은 `i=13` 으로 됐다.
`(8,"15")` 는 needle 이 2글자다 — spec 8 의 open 문면에 "15" 가 우연히 들어오면 엉뚱한 항목을 닫는다. 잠복 지뢰.
**제안**: `closelist` 적용 시 매칭 0건인 needle 을 에러로 올릴 것(+ needle 최소 길이 강제).

### 1-2. needle 하나가 같은 index 의 open 여러 건을 한꺼번에 닫는다(과닫힘)
전수 11건. **내 배치**: `(11,"passive_plan")` 이 3건을 동시에 닫았다.
세 건 자체는 `history[3]/[4]/[5]` 로 실질 정당화되지만, `closed[1]/[2]/[3]` 의 `why` 가 전부
「tcx sig 가 (BigPlan, u8)」로 붙어 감사 흔적이 틀렸다(무엇이 무엇을 닫았는지 역추적 불가).
**제안**: needle 이 2건 이상 매칭하면 경고 + `why` 를 항목별로 요구.

### 1-3. specgate 에 `open[] ↔ history[]` 게이트가 없다 (이 라운드 최대 수확)
`history_note` 는 *"여기 있는 사실이 `mem`/`consts`/`knobs`/`callees` 에 반영됐는지는 `specgate.py` 가 검사한다"* 고
적혀 있지만, **`open[]` 이 history 로 이미 닫힌 질문을 물고 있는지는 아무도 검사하지 않는다.**
실측: **내 배치 open 20건 중 9건**(11 의 3건 전부 + 12 의 6건 전부)이 같은 명세 `history[]` 에 답이 있는데 열려 있었다.
G6(표의 `~~구~~` 가 logic 에 남았는가)과 방향이 반대라 G1~G6 전부가 이걸 못 본다.

제안 게이트 **G7**(프로토타입 = `C4_g7.py`, 식별자 집합 유사도):
`open[].q` ↔ `history[].was` 유사도 >=0.70 이면 적발 → 전 20함수에서 **18건**(>=0.45 면 25건).
내 배치 8건이 0.72~1.00 으로 잡힌다(예: `/specs[11]/open[1]` ↔ `history[1]` = **1.00**, 문장이 거의 동일).

### 1-4. G3 의 `FREE_FN` 이 하드코딩 인덱스 집합이다 (같은 부류의 잠복 결함)
지금은 정확하지만 명세 추가·재정렬 시 조용히 썩는다. 망글링으로 유도 가능하고 실측 20/20 일치:
`sym.startswith("_RNvM")` ⟺ `siblings.plan != null`.
또 G3 는 `siblings.count != len(entries)` 를 검사하지 않는다(현재는 전건 일치).

---

## 2. region_point 산출식 — 3차의 최대 잔여를 닫았다 (신규, ev 2 / 27칸 전부 일치)

`region_point` 는 `AbstractGameWithCache+0x2218` 의 `[i32; 27]` 이고, 생성 함수는
`AbstractGameWithCache::new_with_prev_cache`(game-core/simulation.rs:1780, `_gcbc/g15.ll:102716~108636`).

### (a) 결합식 — `_gcbc/g15.ll:103504~103610` (simulation.rs:1703~1729)
지역변수 DWARF 이름이 그대로 있었다(`#dbg_declare`):
`%9 = region_point[27]` / `%8 = blue_dist_one_count[27]` / `%7 = red_dist_one_count[27]` /
`%22 = blue_regions[27]`(bool) / `%21 = red_regions[27]`(bool).

```
memset(region_point, 0); memset(blue_dist_one_count, 0); memset(red_dist_one_count, 0);
for r in 0..27 {                                    // simulation.rs:1703  (블루 카운트)
    if blue_regions[r] && !red_regions[r] {         // 양쪽이 다 소유한 region 은 건너뛴다
        for &n in map.region_adj[r] { blue_dist_one_count[n] += 1 }   // n>=27 이면 panic
    }
}
for r in 0..27 { if red_regions[r] && !blue_regions[r] {              // 대칭
        for &n in map.region_adj[r] { red_dist_one_count[n] += 1 } } }
for r in 0..27 {                                    // simulation.rs:1729  (결합, store 1곳)
    region_point[r] = (blue_regions[r] ? 5 : 0) + blue_dist_one_count[r] as i32
                    - (red_regions[r]  ? 5 : 0) - red_dist_one_count[r] as i32;
}
```
3차가 「store 0건」으로 오판했던 지점이 여기다 — store 는 루프 안에 정확히 1개(`g15.ll:103609`)다.
인접표는 `MapDef+0x0` = `region_adj: Vec<Vec<usize>>`(IR 은 `map+8`=ptr, `map+16`=len 으로 읽는다).

### (b) blue/red_regions 마킹 — `_gcbc/g15.ll:105597~105750` (simulation.rs:1400~1440 클로저, 호출 1478/1487)
```
// 팀 T 의 타워 엔티티마다 (클로저 #0 = blue, #2 = red)
cx = min(Entity.x / 32000, 29) ; cy = min(Entity.y / 32000, 29)
r  = map.regions[cy][cx]                       // MapDef+0x38b8
i  = lane_seq(line, T).position(|z| z == r)    // 7칸 선형탐색(완전 언롤), 미발견이면 마킹 없음
for k in 0..=i { regions_of_T[ lane_seq(line,T)[k] ] = true }   // prefix 포함 마킹
```

### (c) 오라클 반증 — `C4_b.rs` / `C4_b.tsv`
시작 상태 실측 `region_point` =
`[8,3,0,-6,6,7,0,0,6,7,-3,-3,-7,3,2,-2,0,0,-6,7,-7,-3,6,-6,3,-7,-8]`.
`map.region_adj`(27행)와 `lane_seq` 6종을 읽어 팀·라인별 prefix 길이 `(0..8)^6` 전수 탐색 →
적합 8건, 전부 `blue=(a,b,c) red=(a,b,c)`, a,b,c ∈ {3,4} 이고 최소해가 **prefix 3**.
(4 를 더하는 것은 양 팀 seq 의 index 3 이 같은 region(Top 17 / Mid 6 / Bottom 16)이라
`blue&&red` 게이트에서 상쇄돼 결과가 불변 — 축퇴이지 불일치가 아니다.)
⟹ **27칸 전부 일치.** 결합식 확정(ev 2).

prefix 3 의 근거도 실측했다(`C4_c.rs`): 각 팀·라인의 1차 타워가 `lane_seq` 인덱스 **2**,
2차 타워가 0 또는 1 이다(team0 Top: tower1 region 22 = idx 2 / tower2 region 5 = idx 1).
⟹ 마킹 경계 = 그 라인의 가장 전진한 생존 타워, 시작 상태에서 `0..=2` = prefix 3. 계산과 일치.

### (d) `*_lead` 독립 재현
3차의 산출식(`team0: p>2 / team1: p<-2` 인 동안 전진, 첫 실패에서 break)을 위 실측값으로 돌려
`top/mid/bottom_lead` **6/6 일치**(전부 2). `AbstractGameWithCache+0x21c0` / `AbstractGameWithCache+0x21d0` /
`AbstractGameWithCache+0x21e0`.

**남는 미탐색(범위 명시)**: 마킹 루프가 훑는 엔티티 집합의 정확한 필터(블록 254 의 3-way phi —
`top_tower`/`tower2`/`twin_towers` 중 무엇을 포함하는지)와 타워 파괴 후 상태.
**방법**: 오라클에서 타워 엔티티를 죽이거나 좌표를 옮겨 2차 데이터점을 만드는 것 — 아직 안 했다.

---

## 3. 내가 새로 닫은 것

### 3-1. `/specs[14]/open[2]` (ev5 → ev2) — `setup_limit`/`wait_limit` 의 실제 소비처
「소비처는 `next_plan`/`is_end` 로 추정」 → **`is_end` 만 쓴다.** `next_plan` 은 안 읽는다.
- `LineGankerPlan::is_end`(ganker.rs:64) = `_gaibc/m08.ll:94519~94567`(전문 49줄).
- `LineGankerPlan::next_plan`(`m08.ll:94975~98368`) 은 self 의 gep 가 `+0x28`(line)·`+0x29`(phase) 두 개뿐
  — `+0x18`/`+0x20` gep 0건.

```
fn is_end(&self, _version, _rnd, player, data, _debug) -> bool {
    let tick = data.cache.game.tick();            // vtable+0x28
    if tick >= self.wait_limit  { return true }   // LineGankerPlan+0x20  (m08.ll:94533 icmp ult)
    match self.phase {                            // LineGankerPlan+0x29
        Cancel        => true,
        WaitResponse  => tick >= self.setup_limit, // LineGankerPlan+0x18
        Setup | ChangeJungle(_) => false,
    }
}
```
IR 의 `select(v>5, v-6, 3)` + `switch {2→true, 0→블록24, default→false}` 는
니치 태그(ChangeJungle 0..5 / WaitResponse 6 / Setup 7 / Cancel 8)를 논리 인덱스로 되돌리는 접힘이다.

**오라클 진리표 36/36 일치**(`C4_a.tsv` `IS_END` 행, phase 4종 × setup_limit 3종 × wait_limit 3종).
덤으로 `LineGankerPlan::new(line, a, b)` 의 2번째 인자 = `setup_limit`, 3번째 = `wait_limit` 확정
(`new(Top,111,222)` → `setup_limit: 111, wait_limit: 222`). 오프셋은 `LineGankerPlan+0x18`(setup_limit) ·
`LineGankerPlan+0x20`(wait_limit) · `LineGankerPlan+0x28`(line) · `LineGankerPlan+0x29`(phase).
`C4_a.tsv` 의 `-1` 표기는 `tick.wrapping_sub(1)` = `u64::MAX` 를 `as i64` 로 찍은 것이다(진짜 -1 아님).

### 3-2. `/specs[13]/open[0]` (ev3) — 인자승격된 `%0/%1/%2` 의 출처가 간접증거 3개 → 직접증거
3차는 DWARF·인라인 dbg_value·`entity.rs:580` 세 간접증거로 매핑했다고 적었다. 호출부에 gep 가 그대로 있다.
`LineGankCoverPlan::sub_plan`(`_gaibc/m10.ll:11778~11791`, cover.rs:227):
```
%24 = load i8 , gep %1(&LineGankCoverPlan), +32   => arg0 = self.line
%9  = load i64, gep %4(&PlayerState)     , +2352  => arg1 = player.info.team
%14 = load i32, gep %4                   , +2496  => arg2 = player.info.position
%16 = load ptr, ptr %5(&OperationData)   , +0     => arg3 = data.cache
%26 = load ptr, gep %5                   , +8     => arg4 = data.context
```
두 번째 호출부(`m10.ll:12292`, cover.rs 내 `next_plan`)도 동형이다.
`tcxdict` 교차확인: `LineGankCoverPlan+0x20` = `line: LineType`, `LineGankCoverPlan+0x18` = `wait_limit`,
`PlayerState+0x930` = `info.team`, `PlayerState+0x9c0` = `info.position`.

### 3-3. `/specs[13]/open[3]` — 「다른 `v*` 판본이 blackboard 를 쓰는지」 = 쓴다
- `LineGankCoverPlan::target_bush`(cover.rs:196)는 `OperationData+0x10`(blackboard)를 읽는다 —
  `_gaibc/m10.ll:12124`(`gep %5,+16`), `!dbg` 사슬 = cover.rs:210 → next_plan cover.rs:36,
  그 안에서 `Blackboard::minion_state`(game_core blackboard.rs:379~381)가 인라인돼
  `blackboard[team]` 를 라인별 필드(+0 / +40 / +80)로 색인한다.
- `LineGankerPlan::target_bush`(ganker.rs:351)도 같다 — `m08.ll` 의 `gep %5,+16`,
  사슬 = ganker.rs:365 → next_plan ganker.rs:142.
- `target_bush_v41` 은 인자승격으로 `(&cache, &ctx)` 만 받아 접근 불가(3차 확정), `target_bush_v30` 은 안 읽음.
⟹ 부시 선택기 3종 중 `target_bush` 만 blackboard(미니언 상태)를 본다.

### 3-4. `/specs[10]/open[2]` — vtable 슬롯을 구체 `Game` impl 로 런타임 대조 (ev 2)
`divtable` 이 준 이름은 `ExpectedGame` vtable 전역 기준이라는 도구 경고가 open 의 근거였다.
`&dyn AbstractGame`(진짜 `Game`)의 팻포인터에서 슬롯을 바이트 오프셋으로 꺼내 간접호출하고
직접 트레이트 호출과 대조했다(`C4_a.tsv` `VT` 행):

| 슬롯 | 이름 | 결과 |
|---|---|---|
| `AbstractGame vtable+0x28` | `tick` | 간접 0 == 직접 0 OK |
| `AbstractGame vtable+0x40` | `get_game_mode` | 태그 0(Moba) == 0 OK |
| `AbstractGame vtable+0x1f0` | `get_entity_by_id` | 반환 포인터 동일 OK |
| `AbstractGame vtable+0x108` | `strategy` | Debug 문자열 동일 OK (3차 신규 주장 재확인) |

⟹ 슬롯 인덱스는 impl 무관(트레이트 선언 순서가 정한다)이 실행으로 확인됐다.
남는 것은 「런타임에 어느 impl 이 꽂히나」뿐이고 그것은 함수의 성질이 아니라 호출자(배경 시뮬 여부)의 성질이다
⟹ 이 명세의 open 으로 둘 이유가 없다.

---

## 4. 과열림 12건 (open → closed. 전부 같은 명세 안에 답이 있다)

### `/specs[11]` — open 3건 전부. 3차 §2.8/§2.9/§2.10 이 종결 선언했고 `history[0..2]` 에 답이 있다.
| 경로 | 답이 있는 곳 | 요지 |
|---|---|---|
| `/specs[11]/open[0]` | `history[0]` | `_ => true` 아님. IR switch 에 4·5·6 명시 case + default unreachable. 소스 줄 = `rule_scope.rs:96` (`\|` 결합 arm, 문자수 77 검산) |
| `/specs[11]/open[1]` | `history[1]` | `GameMode` 16B, tag 0 Moba→`&MobaMode` / 1 SingleLane / 2 DeathMatch. ptr = 각 impl self 안 서브구조체. 이 함수는 tag 만 씀 |
| `/specs[11]/open[2]` (ev3) | `history[2]` | 구체 impl 3개 전부 `memory(none)` 순수 ⟹ 2회 호출이 다를 수 없다. 1회로 접어도 안전 |

### `/specs[12]` — open 6건 전부. `history[0]/[1]/[2]/[4]/[5]/[6]` 에 1:1 대응(유사도 0.72~1.00).
| 경로 | 답이 있는 곳 | 요지 |
|---|---|---|
| `/specs[12]/open[0]` | `history[0]` | `player_count` 값표 확정(접힘 안 된 호출부 2곳). 비교값 = 5 (`==5` vs `>=5` 만 표기 불가 — `closed[1]` 에 재조사금지로 등재) |
| `/specs[12]/open[1]` | `history[1]` + 3차 §2.11 | `chat_allowed` 전표 확정(`m13.ll:53199~53946`), 297칸 전수 진리표 스펙과 전칸 일치 |
| `/specs[12]/open[2]` | `history[2]/[10]/[12]` | `handle_chat_inner` 전체 디스패치 = `m13.ll:29802`(chat.rs:45) 31-case switch, arm별 계산식·계측 8칸·궁 예약 전부 기록됨 |
| `/specs[12]/open[3]` | `history[5]` + 내 독립 재확증 | 아래 4-1 |
| `/specs[12]/open[4]` | `history[4]` | `misunderstood` 는 플랜 판정에 불개입. 한 곳(chat.rs:598)에서 `objective_misunderstanding` 기록부만 set/clear |
| `/specs[12]/open[5]` | `history[6]` | 184 = `PendingTraceEvent` 순수 원소 크기, SPEC_GUIDE §3 제외 규칙에 정확히 해당 |

**부수 발견(자기모순)**: `/specs[12]/open[2]` 는 「inner 내부는 안 봄」이라고 하는데
같은 명세 `knobs[]` 12개가 `chat.rs:75 / 83 / 141 / 144 / 151 / 163 / 165 / 174 / 199 / 202 / 204~206 / 218 / 235 / 236 / 239 / 245~290 / 598`
= 전부 inner 본문(chat.rs:41~)을 값까지 서술한다. G1(자기모순)은 `consts.src_line` 만 보므로 이걸 못 잡는다.
⟹ **4차 브리핑의 「12 `handle_chat_inner` 본문이 여러 라운드째 미탐색」도 사실과 다르다**(브리핑 오류로 보고).

### 4-1. `/specs[12]/open[3]` — 줄 길이 산술로 독립 재확증(방법이 다르다)
`history[5]` 는 `!dbg` 줄번호 순서(19 < 22)로 조기반환형을 확정했다. 나는 줄 길이 + 들여쓰기로 같은 결론을 얻었다:
`tcx sp` 의 `c=3` ⟹ `impl` indent 0 / `fn` indent 2 / 본문 indent 4(BRIEF §4 함정 12).
`rmeta_srcmap game_ai "handler\chat.rs"` 실측(본문 길이 = 표의 `chars` − 1):

| 줄 | 길이 | 복원 |
|---|---|---|
| L9 | 37 | `    if from == player.info.position {`  (4 + 33 = 37 정확 일치) |
| L10 | 13 | `      return;`  (6 + 7) |
| L11 | 5 | `    }` |
| L18 | **47** | `    if !data.context.trace_level.is_enabled() {`  (4 + 43 = 47 정확 일치) |
| L19 | 91 | inner 호출 |
| L20 | 13 | `      return;` |
| L21 | 5 | `    }` |
| L22~L37 | — | indent 4(트레이스 본문. `else` 안이면 indent 6 이어야 한다) |
| L38 | 7 | `    });` |
| L39 | 3 | `  }`  (fn 닫기, indent 2) |

`!` 를 뺀 `    if data.context.trace_level.is_enabled() {` 는 46 자라 1글자 차이로 배제된다.
⟹ 조기반환형 확정(ev3, 두 방법 독립 일치).

### `/specs[14]` — 3건
| 경로 | 답 | 내 재검증 |
|---|---|---|
| `/specs[14]/open[0]` | 3차 §2.14 의 24행 표 + `history[0]` | 독립 재산출: `MapDef+0x1c98` 을 전수 훑어 id 1~24 · id 0("부시 없음") 829칸 — 3차 표와 전건 일치(11/13/18 의 1 단위 차이는 정수나눗셈 반올림). `C4_a.tsv` `BUSHID` 행 |
| `/specs[14]/open[2]` | 위 3-1 | ev5 → ev2 |
| `/specs[14]/open[3]` | 3차 §3 + `tcxdict --enum game_core::Chat` | `Chat::Cancel` = 태그 `Chat+0x0`(17) + `CancelReason` 이 `Chat+0x1`, 다른 variant 는 `Chat+0x8` 에 usize. 24B 원시덤프 실측: `11 00 00…` / `11 02 00…`(`C4_a.tsv` `CHATRAW`). 나머지 22B = 이 variant 의 패딩 |

---

## 5. 재분류 제안 (오류 아님 — 3차 §7 이 냈는데 미반영)

- `/specs[10]/open[0]/class` : `미탐색` → `표기 불가`(phase 리터럴 3 이 768 = 3<<8 로 상수접힘).
- `/specs[10]/open[1]` : 질문이 아니라 `SPEC_GUIDE §3` 표 규칙의 적용 기록 → `closed` 또는 `logic_note`.
- `/specs[12]/open[5]` : 같은 부류(위 4장에 포함).
- `/specs[14]/open[1]` : 질문이 아니라 도구 오매핑 경고(`fnparts target_bush_v30` 이 `LineGankCoverPlan` 쪽
  `m10.ll:11483` 을 내놓는다) → `logic_note` 로. 이 경고 자체는 맞다: `ganker.rs` 의 `target_bush_v30` 은
  `define` 이 없고(update 안에 전량 인라인), `m08.ll:94136` 의 `define` 은 `target_bush_v41` 이다.

---

## 6. `ev<=3` 표본 재확인 (뒤집힘 0)

`tcxdict` 직접 질의 전건 일치. 한 행 = 한 오프셋:

- `PlayerState+0x930` = info.team
- `PlayerState+0x9c0` = info.position (Direct 태그 4B)
- `GameSetting+0x12c0` = height
- `MapDef+0x1c98` = bushes
- `Entity+0x628` = stat_cached.hp
- `Entity+0x670` = hp
- `Entity+0x660` = x
- `Entity+0x668` = y
- `Entity+0x68` = ty 판별자
- `Entity+0x88` = ty@Tower.info.nearest_enemy 판별자
- `Entity+0x128` = ty@Tower.info.ty 판별자
- `GameContext+0x38` = tutorial
- `GameContext+0x39` = trace_level
- `GameContext+0x20` = map
- `GameContext+0x8` = setting
- `LegacyPlanHandler+0x5e8` = plan
- `LegacyPlanHandler+0x517` = team_plan.objective
- `LegacyPlanHandler+0x858` = pending_trace_events.cap
- `LegacyPlanHandler+0x860` = pending_trace_events.ptr
- `LegacyPlanHandler+0x868` = pending_trace_events.len
- `LegacyPlanHandler+0x530` = pending_global_ult_target

12 `knobs` 의 계측 필드도 전건 일치:

- `LegacyPlanHandler+0x15b8` = ff_call_recv
- `LegacyPlanHandler+0x15c0` = ff_call_ignored
- `LegacyPlanHandler+0x15c8` = ff_call_in_battle
- `LegacyPlanHandler+0x15d0` = ff_call_no_help
- `LegacyPlanHandler+0x15d8` = ff_call_too_far
- `LegacyPlanHandler+0x15e0` = ff_call_low_hp
- `LegacyPlanHandler+0x15e8` = ff_call_bail
- `LegacyPlanHandler+0x15f0` = ff_call_join
- `LegacyPlanHandler+0x180a` = v3_epicops_armed
- `LegacyPlanHandler+0x1806` = counter_jungle_route_init
- `LegacyPlanHandler+0x1610` = mf_swap.0
- `LegacyPlanHandler+0x1618` = mf_swap.1
- `LegacyPlanHandler+0x1628` = v3_lapse_passive_fallbacks
- `LegacyPlanHandler+0x538` = pending_global_ult_target.Some.0.0
- `LegacyPlanHandler+0x540` = pending_global_ult_target.Some.0.1

이 라운드에서 새로 쓴 오프셋:

- `AbstractGameWithCache+0x2218` = region_point
- `AbstractGameWithCache+0x21c0` = top_lead
- `AbstractGameWithCache+0x21d0` = mid_lead
- `AbstractGameWithCache+0x21e0` = bottom_lead
- `MapDef+0x0` = region_adj
- `MapDef+0x38b8` = regions
- `MapDef+0x6ba0` = region_centers
- `MapDef+0x6d50` = nexus_pos
- `LineGankerPlan+0x18` = setup_limit
- `LineGankerPlan+0x20` = wait_limit
- `LineGankerPlan+0x28` = line
- `LineGankerPlan+0x29` = phase
- `LineGankCoverPlan+0x18` = wait_limit
- `LineGankCoverPlan+0x20` = line
- `OperationData+0x10` = blackboard

---

## 7. 남기는 미탐색 (적용 범위 명시)

| 대상 | 판정 | 범위 |
|---|---|---|
| `blue/red_regions` 마킹이 훑는 엔티티 필터(블록 254 의 3-way phi) | 미탐색 | `_gcbc/g15.ll:103366` 주변 엔티티 스캔 루프 IR 독해로 뚫린다. 방법 = 오라클에서 타워를 죽여 2차 데이터점 만들기(미시도) |
| `passive_plan` 반환 `u8` 의 의미 | 미탐색 | `in:game_ai::plan_legacy::handler` = private 이라 오라클 불가. 남은 경로 = `m13.ll:6495~9987` 본문 독해 |
| `is_ignored_well_enemy` 원본 전문 | 미탐색 | 10 의 인라인 사이트(fight_model.rs:1232)만 확인 |
| `rule_scope.rs:39` 의 `==5` vs `>=5` · `goal_allowed` 개별 arm vs `_ => true` | 표기 불가 | 외연 동일 + 줄 길이도 동일("== 5"/">= 5" 둘 다 4자) ⟹ MIR·IR·기계어·실행·줄길이 전 경로 차이 없음 |
| 함수 이름의 `v30`/`v41` 의 출처 | 재료 부재(범위: tcx·IR 3코퍼스·오라클·rmeta 주석) | 버전 게이트 부재 + 호출자별 하드와이어까지가 한계 |
| 13 open[1] Top L151/L156(=3/6)·Bottom L181/L186(=15/20)을 왜 두 분기로 썼나 | 표기 불가 | 값·극성은 확정. 「왜」는 코드에 없는 의도 |
| 런타임에 `Game` vs `ExpectedGame` 중 어느 impl 이 꽂히나 | 미탐색 | 슬롯은 확정(§3-4). impl 선택은 호출자 성질 ⟹ 이 명세 밖. 방법 = 런타임 훅 |

---

## 8. 제출 게이트

```
python -X utf8 specgate.py                                   -> G1~G6 전부 0 (총 0건)
python -X utf8 tcxaudit.py                                   -> 총 550건 오귀속=0 밀림=0 부분일치=0 확인불가=18
python -X utf8 tcxaudit.py --prose _verify4/C/C4_REPORT.md   -> 총 734건 오귀속=0 밀림=0 부분일치=1 확인불가=18
```
내 문서가 더한 184행에서 **오귀속 0 · 밀림 0**. 남는 `부분일치 1`(=`specs20.json` 의 `cache+0x8` 팻포인터)과
`확인불가 18`(vtable 슬롯)은 **내 문서가 아니라 `_spec` 쪽 기존 항목**이다.

### 브리핑 §5 수치와의 차이 (보고 대상)
BRIEF §5 는 기준선을 「인자 없이 돌리면 766건 · 오귀속 0 · 밀림 0 · 부분일치 2 · 확인불가 18」로 적었다.
실측은 **인자 없이 550건 · 부분일치 0 · 확인불가 18** 이고, `--prose` 를 주면 `specs20.json`/`resolved_*.json`
산문까지 추가로 스캔해 734건 · 부분일치 1 이 된다. ⟹ **766/2 라는 숫자는 재현되지 않는다**(측정 조건이 다른 듯).
결론(오귀속 0 · 밀림 0)은 동일하므로 판정에는 영향 없다.
