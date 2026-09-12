> ⚠**이 파일은 배치 A 가 직접 못 남겼다.** 그 하네스가 보고서 `.md` 작성을 막아서,
> 배치가 최종 응답으로 돌려준 본문을 **메인이 원문 그대로** 옮겼다(2026-09-11).

---

# 11차 배치 A — 전수 감사 보고 (`00`~`04`, 게임 0.5.8)

> 신선도: 착수 시 `[A] FRESH` · patch 제출 직전 재확인 `[A] FRESH`(둘 다 도장 `2026-09-11 22:12:19`).
> 사전검증: `cd /c/tfm2mods/MIG && PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 11 --only A --dry`
> → **정정 10/10 · ev상향 1/1 · 실패 0 · 동작 변경 0건** (경고 1건 = `/specs[3]/consts` 삽입으로 뒤 인덱스 6행 이동 → 적용 후 `--restamp` 필요).

---

## 0. 한 줄 결론

**10차 대비 적발이 82 → 11 건으로 줄었고(−87%), 「값·방향·대상」 축에서 나온 것은 0 건이다.**
남은 11건은 전부 **인용 좌표(어느 줄을 가리키나) · `closed` 화석 · 전파 누락 · 빠진 행** 이고,
그래서 `behavior_change` 가 **전건 `false`** 다. 이건 7차에서 지적받은 「전건 false 로 낸 오독」이 아니라
실제로 값 축이 수렴했다는 뜻이며, 근거는 §3 의 전수 대조 결과다(**mem 108행 · consts 40행 전건 IR 일치**).

---

## 1. 실제로 실행한 것 (명령줄 그대로)

```bash
cd /c/tfm2mods/MIG
python -X utf8 dossierfresh.py 11 A                      # 착수 / 제출 직전 2회

# ── IR 원문 (양방향 대조의 본체) ─────────────────────────────
sed -n '39984,40103p' /c/tfm2mods/_gaibc/m05.ll           # 01 calculate_jungle_action_score 전문
awk 'NR>=43967 && NR<=44365 {print NR": "$0}' /c/tfm2mods/_gaibc/m04.ll | grep -v "#dbg_value"   # 00 ult
awk 'NR>=34867 && NR<=34980 {print NR": "$0}' /c/tfm2mods/_gaibc/m12.ll | grep -v "#dbg"          # 02 sub_plan
awk 'NR>=33864 && NR<=34294 {print NR": "$0}' /c/tfm2mods/_gaibc/m10.ll | grep -v "#dbg"          # 03 defensive_crisis
awk 'NR>=58175 && NR<=58644 {print NR": "$0}' /c/tfm2mods/_gaibc/m04.ll | grep -v "#dbg"          # 04 handle_line_defense

# ── !dbg 사슬 해석 (consts.src_line · knobs.where 전수) ──────
for id in 44379 44402 44410 44412 44413 44407 …; do grep -m1 "^!$id = " m05.ll; done   # 함수마다 20~30개
#   (00 = !56306 계열 60개 / 02 = !62810 계열 15개 / 03 = !40590 계열 41개 / 04 = !66939 계열 44개)

# ── 호출처 전수 ─────────────────────────────────────────────
grep -n "call.*calculate_jungle_action_score" /c/tfm2mods/_gaibc/m*.ll
grep -n "call.*abstract_input3ult(" /c/tfm2mods/_gaibc/m*.ll
grep -n "call.*AttackNexusPlan8sub_plan" /c/tfm2mods/_gaibc/m*.ll
grep -n "call.*buff_value16defensive_crisis(" /c/tfm2mods/_gaibc/m*.ll
grep -n "call.*defense_nexus19handle_line_defense(" /c/tfm2mods/_gaibc/m*.ll

# ── 범위 밖 knob 좌표 검산 (_gcbc) ──────────────────────────
sed -n '81810,81820p;81868,81890p' /c/tfm2mods/_gcbc/g06.ll          # 00 knobs[4]
awk 'NR>=81679 && NR<=81900 {print NR": "$0}' g06.ll | grep -n ', -4'
awk 'NR>=306876 && NR<=306884 {print NR": "$0}' /c/tfm2mods/_gcbc/g02.ll   # 00 knobs[5]
sed -n '52706p;52846p;52503,52507p;52770,52774p' /c/tfm2mods/_gcbc/g06.ll  # 03 knobs[15][16][18]
awk 'NR>=52355 && NR<=52850' g06.ll | grep -E "umax|udiv i64 .*, %|add i64 .*100"
sed -n '105197,105198p;105211p;105462p;126472,126479p' /c/tfm2mods/_gcbc/g15.ll  # 03 knobs[7][13][14]
sed -n '31242p;31289p;31294,31297p;31486p;31586p;31666p;31688p;31790p;32954,32958p' _gaibc/m15.ll
sed -n '115823p;115875,115876p;115891,115894p;115908p' /c/tfm2mods/_gcbc/g15.ll  # 04 knobs[4]

# ── 도구 (closed 화석 반증 · exe 확인) ──────────────────────
PYTHONIOENCODING=utf-8 python -X utf8 divtable.py EffectType 0x118
PYTHONIOENCODING=utf-8 python -X utf8 divtable.py Action
PYTHONIOENCODING=utf-8 python -X utf8 tcxdict.py Entity
PYTHONIOENCODING=utf-8 python -X utf8 name2rva.py defensive_crisis
head -4 _verify4/A/A4_o01c.tsv ; sed -n '5,34p' _verify4/A/A4_o01c.tsv   # ev_up 근거 원문

PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 11 --only A --dry
```

**방향**: 매 함수 `ir.frm~ir.to` 를 **먼저 통째로 읽고** 거기서 나온 로드/스토어/상수/비교를 명세표에
맞춰 보는 「IR→명세」로 갔다(10차 배치C 관측 그대로). 게이트 후보는 이 배치 몫이 0 건이라 쓸 게 없었다.

---

## 2. 함수별 발견

### `00 ult` — **6건** (실오류 4 · 보강 1 + closed 2건 포함)

| # | 칸 | 무엇 |
|---|---|---|
| A-1 | `knobs[4].where` | **`can_ult` CC 집합의 좌표가 다른 계산을 가리킨다.** `g06.ll:81816~81818` 은 `%67 = load i32, ptr %66`(Entity+**0x46c** ult_cooldown_mult)·`%68 = gep … i64 1024`(**0x400** skill_cooldown_mult)로 history[0] **3번(쿨타임 보정)** 이다. 인용한 `(tag-6) u< -4` 는 can_ult(g06.ll:81679~) 안에 **정확히 한 곳**, **81878~81879** 에 있다. |
| A-2 | `knobs[5].where` | `g02.ll:306879` 는 `#dbg_value` 줄. `ret i64 1` 은 **306881**. 10차 형태 「명령이 아닌 줄」. 슬롯(`Action` vt+0xa8 = `cooltime_use_count`)은 `divtable.py Action` 으로 재확인 — 맞다. |
| A-3 | `closed[0]`(=`unknown[0]`) | ★**화석.** 「`divtable.py EffectType 0x118` 이 '없음: EffectType' 을 반환(1회 시도) · Arc<dyn> 이라 `_gaibc` 에 vtable 전역이 없다」 → 11차에 그대로 돌리니 `_gcbc` 자동 폴백으로 **두 vtable 에서 `0x118 → on_caster`** 가 나온다. **바뀐 것은 사실이 아니라 도구다.** 같은 파일 `history[3]`·`mem[24]`·`mem[25]` 는 이미 `on_caster`/`linear_move_speed` 를 싣고 있었다 = 판정 반전. |
| A-4 | `closed[7]`(=`unknown[7]`) | ★**전파 누락.** 「버전 게이트가 그 하위에 있는지는 미확인」 → `safe_move_avoiding_enemy_well`(m04.ll:43378~43964)에서 `%1`(version)을 쓰는 비교는 **43914 `icmp ugt i64 %1, 1`** 단 하나. 같은 파일 `history[4](c)`·`knobs[7]` 이 이미 v1/v2+ 를 적고 있었다 = 판정 반전. |
| A-5 | `mem[11].note` | 근거가 인용한 `43306`·`43339` 는 **`ult` IR 범위(43967~44365) 밖**(= `linear_cast_range_with_margin` 의 인라인 사본). 문면은 맞지만 담당 함수 안의 같은 명령(44251/44262 champ · 44283/44294 target)을 보강. |

**깨끗했던 것**: `mem` 29행 전수 ✓(오프셋·방향 전건 IR 일치, 빠진 접근 0) · `consts` 10행 `src_line` 전수 ✓(`!dbg` 사슬 루트까지 추적) · `knobs[0][1][2][3][6][7][8]` where ✓ 전부 문면까지 정확 · `callers` 1곳(m07.ll:12776) ✓ · `sig.params` 7행 타입·`readonly`/`readnone` 속성 전건 IR 일치 ✓ · `one_line`/`layer` ✓.

### `01 calculate_jungle_action_score` — **0건**

전수 대조에서 **한 칸도 틀리지 않았다.** 확인 범위:
- `mem` 8행: `0x930/0x9c0/0x0/0x8/0x1e0/0x68/0x98/0x670` 전부 IR gep 상수와 일치(2352/2496/0/8/480/104/152/1648). 방향 전건 `r`(IR store 0개) ✓.
- `consts` 12행 `src_line`: 522/526/526/528/528/530/531/531/536/536/536/542 — 각 phi 인입 블록 종결자의 `!dbg` 루트까지 추적해 **전건 일치**. 특히 `0 @536`(9차 배치A 가 못 박은 phi 귀속)과 `5 @542`(!44407→!44378=542) 재확인.
- `knobs` 7행 where: 40043 phi · 40066 select · 40091 select · 40059~40063 — 문면까지 일치.
- `callers` 3곳(m02.ll:40037/40081/40124) ✓, `closed[6]` 의 L159/171/183 도 `!46605/!46610/!46615` = 159/171/183 로 확정 ✓.
- `logic` 전문 ↔ IR: `icmp sgt %27,%25 → select 20,40`(hp>value→20) 의 arm 반전, `smin(coef, coef*value/hp) + based`, `sub`/`sdiv` 부호까지 일치.
- `sig.params` 이름 7개를 `!DILocalVariable` 로 재확인(`_rnd/player/data/_parameter/_action/effect/t`) + 지역 5개(`champ/value/hp/coef/based`) ✓.
- `mem[7]` 「hp 는 usize 필드인데 i64 로 로드」 → `tcxdict Entity` = `0x670 hp usize` ✓ / DWARF 지역 `hp` 타입 = `i64` ✓ (둘 다 맞다 = `as i64` 캐스트).

### `02 sub_plan` — **0건**

- `mem` 21행 전수 일치(2352/2496/0/8/32=0x20/480/28016=**0x6d70**/0,8,16,24/1632/1640/1648/**1576=0x628**/%1+8/sret 0,8,9,10). 쓰기 4행 방향 ✓.
- `consts` 6행: `2@36`(!62937=36) ✓ · `1@45`(!62970=45) ✓ · `0@45`(!62985→루트 45) ✓ · `2@48`(!62989=48) ✓. `5@42`·`16@50` 은 phi 인입이라 IR 귀속이 **판별력 없음**(약한 후보) — 명세가 그 사실을 이미 적고 rmeta 줄길이로 세웠다. 뒤집지 않았다.
- `knobs[4]` = `m13.ll:6720 sub i64 1, %102` ✓ 정확.
- `callers` 1곳(m02.ll:8484) ✓ · `siblings` 9개 = `AttackNexusPlan` 메서드 전수(자유함수는 0 이 정상) ✓.
- `logic` 의 De Morgan 형(`!(y<ly || y>ry)`)·bumpalo len@+0x18(gep stride 32 + 328) 전부 IR 과 일치.

### `03 defensive_crisis` — **4건**(보강 2 · 실오류 2 + 행 추가 1)

| # | 칸 | 무엇 |
|---|---|---|
| A-6 | `knobs[5].where` | 빈 `towers` Vec 생성의 소스 귀속은 **:34**(`!40950` = vec.rs:547 ← `!40749` = buff_value.rs:**34**). `:33` 은 `check_kill_die_tick` **호출줄**(`!40949`). 두 줄이 한 칸에 뭉개져 있었다. |
| A-7 | `knobs[16].where` | ★`g06.ll:52706` = 블록 라벨 `187:`, `52846` = 함수 닫는 `}` — **둘 다 명령이 아니다**(10차 형태). 실제는 **52708~52709**(물리) · **52842~52843**(마법) `udiv` + `umax(.., 1)`. 52846 은 ±2 안에도 없다. |
| A-8 | `no_map_reason` | ★**판정 반전.** 「인라인됐거나 지도 선별에서 빠진 것으로 보인다(미확인)」 → `name2rva.py defensive_crisis` = **`0xe11e90  437B  패닉줄 [26]`**. exe 에 독립 함수가 **있다** ⟹ 인라인이 아니라 지도 선별 누락이다. (명세 머리표가 지시한 확인을 그대로 수행한 결과다.) |
| A-9 | `consts` **행 추가** | ★**빠져 있던 것.** 팀 첨자 bounds-check `2`(m10.ll:33899 `icmp ult i64 %21, 2`)가 없다. **같은 구문을 `00`·`01`·`02` 는 셋 다 `consts[0]` 으로 싣고 있다** — `03` 만 빠졌다. |

**ev 상향 1건**: `knobs[16]`(최종 피해식) **ev4 → ev2**. 근거 = `_verify4/A/A4_o01c.tsv` 가 `def` 를 0/25/50/100/200/400/900/1900 으로 쓸어 `501/401/334/251/167/101/51/26` 을 **26/26 MATCH·mismatch 0** 으로 맞춘 실행. ⚠같은 라운드의 `A4_o01b.tsv`(26/26 이 전부 0, 판별력 없음)와 **혼동하면 안 된다** — §4 참조.

**깨끗했던 것**: `mem` 22행 전수 일치(0/8/16/0/8/**4856=0x12f8**/2352/480/0,8/**1472=0x5c0**/0,24/**104=0x68**/**184,192,200=0xb8,0xc0,0xc8**/**1224=0x4c8**/**1272=0x4f8**/**1280=0x500**/1336=0x538/1480=0x5c8/48=0x30) · `consts` 7행 `src_line` 전건 `!dbg` 사슬 일치(22/43/44/45/48/35/48, 루트 `!40841/!40854/!40867/!40902/!40943` 전부 확인) · `callers` 3곳 ✓ · aux 인용 4개(m10.ll:55883 `max_range` · 55916 `add i64 %9, 30000` · 55948 `is_enemy_well_danger` · 55966 `is_recent_visible`) **전부 문면까지 정확** · 범위 밖 knob 12개(`m15.ll:31242/31289/31294~31297/31486/31586/31666/31688/31790/32956`, `g15.ll:105197/105198/105211/105462`) **전부 일치** · `sig.params[0]`·`[4]` 가 인용한 m10.ll:33874/33929/33981/33982/33983 **전부 정확**.

### `04 handle_line_defense` — **1건**(보강)

| # | 칸 | 무엇 |
|---|---|---|
| A-10 | `closed[1]`(=`unknown[1]`) | 물음이 「'관측하는 팀'별인지 '관측당하는 팀'별인지 미확정」인데 **같은 행의 `why` 칸이 1차부터 '인덱스 의미 확정'** 이라 적고 있었고, `03 closed[1]` 에 전문(= `Blackboard[T].last_visible[pos]` 는 팀 (1−T) 가 팀 T 를 본 틱)이 실려 있다. 10차가 02·03·04 의 `closed` 를 손볼 때 이 한 행만 남았다. |

**깨끗했던 것**: `mem` 28행 전수 일치(8/56=**0x38**/0,8/**64=0x40** vt/**576=0x240** MobaMode/2352/**304=0x130**/**384,400,416,432,448,464**=0x180~0x1d0/0,24 Vec/**1472=0x5c0**/1632/1640/**14=0xe** Strategy/Blackboard 0,0x28,0x50/BMP 0x10,0x20) · `consts` 5행 `src_line` 전건 일치(549 체인 `runner.rs:263 ← rule_scope.rs:46 ← defense_nexus.rs:549` 를 `!66990→!66989→!66984` 로 확인 / 572 / 578×2) · `knobs` 7행 where **전건 문면 일치**(58200 · 58583 · 58635 · 58636 · 60449 · 60455 · g15.ll:115823/115875/115876/115891~115893/115908) · `callers` 4곳 ✓ · `logic` 의 `select i1 %169, i1 %171, i1 %172`(Battle → `(near−1)<u2` / Gather → `near>u1`) ✓.

---

## 3. 「빠져 있던 것」 — **1건**

`03 consts` 의 팀 첨자 bounds-check `2`(A-9). 전수 대조가 아니면 안 나온다 — 게이트는 *있는 칸이 틀렸나*만 본다.

그 외 `mem`(108행) · `knobs`(49행) · `consts`(40행) · `callers`(12곳) 을 IR→명세 방향으로 전부 훑었는데
**다른 누락은 없었다.** 특히 10차가 형태로 지목한 **「Vec 의 ptr/len 은 싣고 원소 역참조는 안 싣는」 결손은
배치 A 에서 재발 0** 이다(`03 mem[10][11]` · `04 mem[17][18]` 둘 다 ptr/len 을 별 행으로 갖고 있고,
원소 역참조는 `03` 은 `Entity(near_enemies 원소)` 8행, `04` 는 `Entity 0x660/0x668` 로 실려 있다).

---

## 4. 내가 세웠다가 **스스로 반증한 가설 — 4건** (전부 `patch.json` 에 넣지 않았다)

1. **「`01 closed[2]` 의 '오라클 26/26' 은 판별력 0 인 실행을 확증으로 쓴 것」** — `METHOD_MAP ⑥` 한계 2 가
   「실전 것도 액션 파라미터가 0 이어서 `expected_damage_target` 이 **26/26 전부 0**(조기반환)」이라 적고 있어
   같은 숫자·같은 라운드라 동일 실행으로 보였다. **반증**: `_verify4/A/A4_REPORT.md:124` 가
   **두 실행**을 명시한다 — `A4_o01b`(26/26 이 0, 판별력 없음) vs `A4_o01c`(`AttackEffect` 수동 구성, 26/26 MATCH).
   `A4_o01c.tsv` 원문을 열어 `def` 스윕 8행이 `501/401/334/251/167/101/51/26` 으로 **실제로 갈린다**는 걸 확인했다.
   ⟹ 오류가 아니고, 오히려 `03 knobs[16]` ev 상향의 근거가 됐다. **「같은 숫자 = 같은 실행」이 함정이었다.**
2. **「`00 knobs[4].effect` 의 '`Bind·BlockAttack` 만 궁 허용' 이 `value`(∉{2,3,4,5})와 모순」** —
   태그 4·5 가 빠져 있어 보였다. **반증**: `history[0]` 의 1·2번이 BlockSkill(4)·BlockMoveSkill(5)을
   **앞 단계에서 이미** 걸러서, 6번 시점의 잔여 허용집합이 실질 {Bind, BlockAttack}(+조건부 BlockMoveSkill)이다.
   `value` 는 **그 한 비교**를, `effect` 는 **함수 전체의 순효과**를 말한다 — 층위가 다를 뿐 모순이 아니다.
   (추론으로 남의 주장을 기각하지 말라는 §S5-b 규칙 그대로 물러섰다.)
3. **「`03 mem[8]`·`04 mem[6]` 이 `&dyn AbstractGame` 의 +0x0/+0x8 을 한 행에 묶어 규약 위반」** —
   `04 mem[18]` 이 스스로 「한 행에 오프셋을 묶으면 기계 검사가 안 된다」고 적고 있어 위반으로 보였다.
   **반증**: 두 함수 모두 그 vtable 로 **디스패치를 하지 않는다**(클로저 캡처/`champions` 인자로만 넘긴다).
   `04` 는 실제 디스패치가 있는 `vt+0x40` 을 별 행(`mem[7]`)으로 두었다 = 규약은 지켜졌다. 행 추가 취소.
4. **「`00 consts` 에 `radius_mult == 0` 분기의 `0` 이 빠졌다」**(m04.ll:44253·44285) —
   **반증**: `radius*(0+100)/100 == radius` 라 그 분기는 **결과를 바꾸지 않는 fast-path** 다.
   판정 상수가 아니고, `consts[7]=100` 이 이미 식을 담는다. 「부재를 결함으로 세지 마라」 적용.

추가로 **패치에 넣지 않은 판정 2건**(근거가 약해서):
- `03 knobs[7]` 의 `g15.ll:126475 store i64 1000` → 실제로 126475 는 `gep … i64 720`(=0x2d0) 이고
  `store i64 1000` 은 **126476**. **±2 안이라 G13 기준으론 통과**하고, 바로 위 126474 에도
  `store i64 1000`(offset 712)이 있어 「어느 쪽을 가리켰나」가 애매하다 ⟹ **보류**(패치 안 냄).
- `03 knobs[15]`(방어 관통식, `g06.ll:52772`) → 그 줄은 `%219 = udiv i64 %218, 100` 이고 관통식인지
  다른 % 항인지 문면만으로 못 가른다. **재료 부재**: `A4_o01c.tsv` 에 `pen` 컬럼이 없어 오라클로도 못 가린다.
  가르려면 `Effect`/`AttackEffect` 에 관통 필드를 넣은 새 오라클이 필요하다.

---

## 5. 판정 어휘로 정리한 잔여

| 판정 | 대상 | 범위·시도 |
|---|---|---|
| `미탐색` | `03 open[0]` 의 이터레이터 조각(`m01.ll:37431~37606`, 175줄) | 이번 라운드도 안 읽었다. 담당 함수 본문(33864~34294)과 call_mut 심(55868~55976)만 읽었다. |
| `미탐색` | `04 knobs[5]·[6]`(from_mid −3000 / minion_count −2) **경계 스윕** | `history[5]` 의 「점등 성공」은 **OR·AND 구조**를 실행 확증한 것이고 임계값 자체를 ±1 로 쓸지는 않았다. ⟹ **ev 상향을 내지 않았다.** 다음 라운드가 `has_line_defense_threat` 오라클에 `from_mid = −3000 / −3001` 두 케이스만 넣으면 닫힌다. |
| `미탐색` | `03 knobs[18]`(`expected_target_hp_ratio` 가 **감산 전에** 더해지는가) | `A4_o01c.tsv` 의 `thr>0` 행은 전부 `def=0`(또는 `thp=1`)이라 「전/후」가 안 갈린다. `thr=10 × def=100` 한 줄이면 갈린다. |
| `재료 부재` | `03 knobs[15]` 방어 관통식의 IR 좌표 확정 | 시도 = ①`g06.ll:52770~52774` 문면 판독 ②`awk`로 52355~52850 의 `udiv/umax/add 100` 전수 추출 ③`A4_o01c.tsv` 컬럼 확인(pen 없음). 셋 다 못 가름. |
| `재료 부재` | `03 exe` dict(addr 외 bytes/instrs/callers/callees) | `name2rva.py` 로 `0xe11e90 · 437B` 까지는 얻었으나 호출/피호출은 `dllmatch.py`+`cg.py` 가 필요하고, `errors[]` 경로 문법으로 dict 를 **생성할 수 없다**(§6-③). `no_map_reason` 문자열로만 남겼다. |
| `표기 불가` | `01 consts[6][7]` 의 `hp <= value` ↔ `value >= hp` | 줄 길이가 같다(둘 다 11자). **arm 반전 구조는 확정**(IR `select %43, 20, 40`). 종전 판정 유지. |
| `사실 서술` | `02 open[0]`(`version` 이 무엇을 게이트하나) | 이 함수에는 분기가 없다는 것까지가 사실. **`00` 쪽에서 답이 나왔다**(`safe_move_avoiding_enemy_well` 의 `version > 1`) — 다음 라운드가 `02 open[0]` 에 그 포인터를 넣으면 닫힌다(`open` 은 패치 불가라 산문으로 남긴다). |

---

## 6. ★내 지시(도시에)의 오류 — **6건** (`brief_errors[]` 와 동일, 여기선 근거 포함)

1. **§1 `ev≥4(미실행)` 컬럼이 두 종류를 섞는다.** `03` 의 14 중 `knobs[7]~[20]` 은 `defensive_crisis` 본문이
   아니라 `check_kill_die_tick`·`get_damage`·`AthleteParameter` **다른 함수의 줄**이다. 경고문은 표 **아래**에 있어
   처음 읽을 땐 14가 표적으로 보인다. ⟹ `ev≥4(범위 안)` / `(범위 밖)` 두 칸으로 쪼개면 구조적으로 해결된다.
2. **§5 `insert` 설명이 `mem` 만 예로 든다.** 실제 `applypatch.resolve` 는 `knobs`→(`knobs`,`new_knobs`) ·
   `consts`→`constants` · `history`→`resolved` 도 같은 방식으로 매핑한다. `consts` 에 삽입해도 되는지 판단하려고
   도구 소스를 읽어야 했다.
3. ★★**§1·10차 형태표가 `closed[]` 를 주 표적으로 지목해 놓고, `patch.json` 으로 `closed` 를 못 쓴다.**
   v3 의 `closed` 는 v2 `unknown`/`still_unknown` 을 `mkspec3.is_closed` 로 거른 파생물이라
   `/specs[i]/closed[j]/q` 는 `resolve()` 에서 `spec.get("closed")` = `None` → **「배열이 아니다」로 거부**된다.
   도시에 §5 는 `open`/`notes` 만 ⛔로 표시했는데 **`closed` 도 같은 제약**이다.
   우회로(`/specs[i]/unknown` 에 **문면 치환**)를 도구 소스(`applypatch.py:331~341`)를 읽어 직접 찾았다 —
   모르고 `/specs[i]/closed[j]/q` 로 냈으면 이번 `closed` 정정 **3건이 전부 0건 적용**이었다.
   (7차 배치A 가 적발한 「계약에 없는 키 `entries`」와 **정확히 같은 사고**가 다른 자리에서 재발한 것이다.)
4. **숙제를 시켰는데 답을 담을 칸의 쓰기 경로가 없다.** `03` 머리표가 「`dllmatch.py`·`name2rva.py` 로 확인하라」
   고 지시하는데, 확인 결과를 담을 `exe` 는 dict 라 `errors[]` 로 **스칼라 치환만** 되고 생성이 안 된다.
   결국 `no_map_reason` 문자열에 적었다.
5. **§4-b 가 「낼 수 없는 축」을 검사시킨다.** `consts.kind` 를 표본 대조하라면서 §5 에서는
   「파생이라 `errors[]` 로 못 쓴다」고 막는다. 이번엔 담당 5함수 `consts` **40행의 `kind` 가 전부
   IR 소비 오프코드와 정합**해 문제가 없었지만(§7), 틀렸다면 같은 벽에 부딪혔을 것이다.
6. **§0 신선도가 `.ll` 을 안 본다.** 명세의 모든 줄번호가 `_gaibc`/`_gcbc` 의 `.ll` 에 걸려 있는데
   스탬프는 `_spec` 해시와 도시에 자신만 찍는다. `.ll` 디렉터리의 mtime 한 줄이라도 §0 에 넣어야 한다
   (이번엔 `m00~m23.ll` 이 전부 `2026-09-08 03:47` 로 일관돼 문제 없었다 — 운이 좋았던 것이다).

**판정 반전(=오류로 셈) 3건**: A-3(`00 closed[0]` 「도구 한계로 불가」→가능) · A-4(`00 closed[7]` 「미확인」→확인됨) ·
A-8(`03 no_map_reason` 「인라인된 듯」→exe 에 독립 함수 있음). 셋 다 `errors[]` 에 포함돼 있다.

---

## 7. §4-b 「아직 게이트가 없는 축」 — 담당 5함수 표본 대조 결과

| 축 | 대조 행수 | 불일치 | 어떻게 쟀나 |
|---|---|---|---|
| `mem.dir` | **108/108** | **0** | 각 함수 IR 전문에서 `load`/`store` 를 뽑아 베이스+오프셋으로 매칭. `00` 의 `w` 3행(sret +0x0 판별자 · +0x8 memcpy 24B · 전체 32B)과 `02` 의 `w` 4행(sret 0/8/9/10)이 IR store 와 1:1. `01`·`03`·`04` 는 게임 구조체 store 0개 ⟹ `writes` 빈 배열이 맞다. **7차 배치A 의 「89행 불일치 0」과 합쳐 이 축은 닫아도 된다고 본다.** |
| `consts.kind` | **40/40** | **0** | `임계`=`icmp` 피연산자 / `태그`=`switch` case·판별자 비교 / `센티널`=니치 `-1` / `산출값`=`phi`/`select` 결과 / `인덱스`=`sub`·gep 첨자 / `계수`=`mul`/`udiv` 로 보고 IR 소비 오프코드와 대조. `03 consts[5]`(값 1 / kind `태그`)만 애매한데 — 실제 소비는 `shl i64 %17, 1` 의 **shift 량**이라 `태그`보다 `계수`가 맞다. ⚠단 `kind` 는 파생이라 `errors[]` 로 못 고치고 `meaning` 낱말을 바꿔야 하는데, 그 `meaning` 은 「'2초 안에 죽는가'」로 이미 정확하다 ⟹ **도구(`mkspec3` 의 낱말 규칙) 쪽 문제**로 보고 손대지 않았다. |
| `sig.params.role` | **34/34** | **0** | IR `define` 의 파라미터 속성(`readnone`/`readonly`/`captures(...)`/`dereferenceable(N)`)과 대조. `02` 의 `rnd`·`_team_plan`·`_debug` = `readnone` ✓ / `04` 의 `_debug` = `readnone` ✓ / `01` 의 `_parameter`·`_action` 은 IR 이 `readonly`(readnone 아님)지만 **본문 참조 0** 이라 「전혀 안 씀」 서술은 유효(LLVM 의 보수적 추론). `00` 의 파라미터 이름 7개는 `!DILocalVariable` 로 직접 확인. |

**검사기 제안(다음 라운드 게이트)**
- **G20 `knobs.where` 의 「명령인가」 판정.** 이번 3건(`00 knobs[5]` `#dbg_value` · `03 knobs[16]` 블록 라벨 + 닫는 `}`)은
  전부 **인용 줄에 명령이 아예 없는** 경우다. G13 은 「그 줄 ±2 에 인용 명령이 있나」만 봐서
  `#dbg_*`·`^\d+:`(블록 라벨)·`^}`·빈 줄을 **통과시킨다**. 판정식: `where` 가 `<파일>:<줄>` 을 담고 있으면
  그 줄을 읽어 `^\s*(%\w+ = |tail call|call|br|ret|store|switch|unreachable|invoke|cleanup)` 에
  걸리지 않으면 **경고**(기각 아님 — ±2 안에 명령이 있으면 구제). `_gcbc` 경로(`g\d\d\.ll`)도 대상에 넣어야 한다:
  이번 3건 중 2건이 `_gcbc` 였고 G13 은 `_gaibc` 만 본다.
- **G21 `closed[].q` 의 「도구 불가」 문구 감시.** `불가|없음:|미확인|미확정|안 봄|확인 불가` 가 `q` 에 있는데
  같은 spec 의 `resolved`(history) 나 `reads/knobs/constants` 가 **그 대상의 답을 이미 담고 있으면** 경고.
  이번 A-3·A-4·A-10 이 전부 그 형태였고(10차는 22건), **한 라운드에 다 안 없어진다**는 게 이번 결과다.
  느슨한 앵커는 G17 처럼 오탐이 지배하므로 **대상 토큰(`0x118`, `version`, `Blackboard`)을 q 와 history 양쪽에서
  교집합**으로 잡아야 한다.
- **G22 `consts` 의 구문 누락(`panic_bounds_check` 상수).** 함수 IR 에 `panic_bounds_check(i64 %x, i64 N, ...)` 가
  있는데 `consts` 에 값 `N` 이 없으면 경고. A-9 가 그 형태이고, 기계 판정이 쉽다(호출 인자 2번째가 곧 길이).

---

## 8. ★10차 대비 — 무엇이 줄고 무엇이 남았나

| 형태(10차가 지목) | 10차 | 11차 배치 A | 판정 |
|---|---|---|---|
| `mem` 값·오프셋 오류 | 다수 | **0/108** | ✅ **수렴** |
| `mem` 원소 역참조 누락(체계적 결손) | `07`·`08`·`14`·`18` 반복 | **0** | ✅ 배치 A 엔 재발 없음 |
| `consts.src_line` 오귀속 | (G12 축) | **0/40** | ✅ **수렴** |
| `sig.params` 타입·역할 | (10차에 칸이 숨겨져 있었음) | **0/34** | ✅ 칸이 보이게 된 첫 라운드인데 **불일치 0** |
| `siblings` 전 행 | (10차에 칸이 숨겨져 있었음) | **0** (`02` 9행 전수 ✓ · 나머지 4함수는 자유함수라 0 이 정상) | ✅ 같은 이유로 0 |
| ★`closed[]` 썩음 | **22건**(D 15 · A 7) | **3건**(`00`×2 · `04`×1) | ⚠**줄었지만 안 끝났다** — 10차가 배치 A 에서 7건을 고쳤는데 **3건이 남았다**(`00 closed[0]`·`closed[7]` 은 10차가 손댄 파일에서 안 고쳐진 것) |
| ★`knobs.where` 가 명령이 아닌 줄 | (형태로 제시) | **3건**(`00 knobs[5]` · `03 knobs[16]` ×2 좌표) + 좌표 오지정 1건(`00 knobs[4]`) | ⚠**그대로 남았다** — 전부 **`_gcbc` 참조**다. G13 이 `_gaibc` 만 보기 때문 |
| ★`history` 확정 → 표 미전파 | `10` 스킬2/궁 게이트 | **2건**(`00 closed[7]`·`04 closed[1]`) + ev 1건(`03 knobs[16]`) | ⚠**남았다**(G17 의 오탐률 90% 때문에 실질 미검출) |
| ★「아예 빠진 것」 | `09 knobs[7]` | **1건**(`03 consts` bounds-check) | ⚠소수 남음 |
| ★오라클이 의미로는 틀림 | `12 Chat +0x1` | **0** — 다만 **인접 함정**을 하나 확인했다: `A4_o01b`(판별력 0)와 `A4_o01c`(판별력 있음)가 **둘 다 "26/26"** 이라 문서 두 곳에서 같은 숫자로 인용된다 | ⚠기록 함정 |

**⟹ 결론.**
1. **값 축(mem·consts·sig·siblings)은 수렴했다.** 두 라운드 연속 전수로 훑어 불일치 0 이고,
   10차에 **처음 보이게 된 세 축**(`sig.params` 타입 · `siblings` 전 행 · `closed` 의 `why`)에서도 0 이 나왔다.
   ⟹ 「칸을 숨긴 게 원인이었나」에 대한 답은 **아니다** — 보이게 하고 다시 재도 그 축들은 깨끗하다.
2. **안 끝난 것은 정확히 두 부류다.**
   ① **문서 좌표**(`knobs.where` 가 가리키는 줄) — 특히 **`_gcbc` 참조는 어떤 게이트도 안 본다.**
   ② **`closed`/전파** — 10차가 22건을 고쳤는데 같은 배치에서 3건이 더 나왔다.
   둘 다 「사람이 한 번 훑어서 다 잡는」 종류가 아니라 **기계 검사가 없어서 남은 것**이다(§S5-c 그대로).
3. ⟹ **전수 감사를 한 번 더 도는 것은 답이 아니다.** 위 §7 의 **G20·G21·G22** 를 붙이는 것이
   같은 노동의 1/10 로 같은 결과를 낸다. 이번 11건 중 **G20 이 4건, G21 이 3건, G22 가 1건** — **8/11 을 기계가 잡는다.**
   `found_by` 도 그 말을 한다: **new 6 · reused 5** — 새 기법이 절반이라는 건 명세가 불안정해서가 아니라
   **아직 계측기가 없는 축을 손으로 밟고 있다**는 뜻이다.

---

## 9. patch 요약

- `errors[]` **10건** (실오류 6 · 보강 4, 그중 **행 추가 1**) — `behavior_change` **0건**
- `ev_up[]` **1건** (`/specs[3]/knobs[16]` ev4→ev2)
- `brief_errors[]` **6건**
- 사전검증 `--dry` = **정정 10/10 · ev상향 1/1 · 실패 0**
- ⚠적용 후 **`--restamp` 필수**(`/specs[3]/consts` 삽입으로 v2 기준 6행 이동, v3 는 더 밀릴 수 있음)
