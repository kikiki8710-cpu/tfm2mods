# -*- coding: utf-8 -*-
"""REPORT.md 작성 보조(§6 필수 산출물). 실행: python -X utf8 _verify15/D/oracle/write_report.py"""
import io
txt = u"""# 15차 배치 D 보고 — `35`~`39` (게임 0.5.8) · 2026-09-13

기계 적용분 = `_verify15/D/patch.json` (정정 50 · ev상향 93 · brief_errors 6). `applypatch.py 15 --only D --dry` = **정정 50/50 · ev상향 93/93 · 실패 0 · 동작 변경 3건**.
오라클 = `_verify15/D/oracle/D15_o1.rs` (출력 `D15_o1_case0..7.out`, 케이스당 프로세스 1개) · mem 전행 tcx 대조 = `oracle/tcx_mem_check.txt` · 패치 생성 스크립트 = `oracle/mkpatch_D.py`.

## 0. 신선도

- 착수 시 `dossierfresh.py 15 D` = **FRESH**. 제출 직전 재실행 = **STALE** — `specs20_v3.json` 이 도시에 생성(08:00:49) 2분 뒤(08:02:40)에 재생성됐다. 확인 결과 `DOSSIER_D.md`·`spec_35~39.md` 는 그대로(08:00:49)이고, 담당 5함수의 mem/consts/knobs/open/params 행수·`exe` 값(36 `e6d000`=15126528 등)도 도시에와 같다. `--dry` 가 현행 v2 에 대해 50/50 통과했으므로 **patch 는 현행 정본 기준으로 유효**하다. (§5 지시 오류 ⑥)

## 1. 함수별 결과

| # | 함수 | 정정(실오류/오탐/보강) | behavior_change | ev상향 | 오라클 |
|---|---|---|---|---|---|
| 35 | resolve_fight_stake | 20 (20/0/0) | **3** | 11 | 불가(`in:game_ai`, 루트 재수출 없음 — `_tcx` `v=in:game_ai`) |
| 36 | calculate_nexus_defense_count | 11 (7/3/1) | 0 | 20 | **8세계 전부 MATCH** |
| 37 | i_am_chosen_defender | 15 (10/5/0) | 0 | 31 | **190/190 × 7세계 MATCH** |
| 38 | can_tower_focused_when_battle | 2 (1/0/1) | 0 | 22 | **96/96 × 7세계 MATCH** |
| 39 | is_wave_priority_start_line | 2 (1/0/1) | 0 | 9 | 불가(`in:game_ai`) |

### 35 resolve_fight_stake — ★실오류(동작 변경) 1건 + G16/G13/G10
- ★**L597 `rescue_ally` 선택 키가 `dist²` 단독이 아니라 `(dist², id)` 튜플**이다(동거리면 id 작은 아군). 근거: m10.ll:42666 fold 심볼 `…min_by_key3keyRB1n_TyjE…` = `key<&&Entity, (u64,usize), {closure#1}>`, 누산기 store 42661/42663/42665(dist²@+0 · id@+8 · &elem@+16). 줄 길이 산술로 소스 597행(99자)이 `          diff.rescue_ally = remaining.iter().min_by_key(|a| (a.distance_sq(champ), a.id)).map(|a| a.id);` 로 ±0 복원(tcx closure#1 c=52 · closure#2 c=90). → `logic`·`knobs[2].value/where`·`mem[12].note` 3건 behavior_change=true.
- G16: sret 행 삽입(`op:insert at=0`) + 11 인자 role 에 `%k` 명기(슬라이스 2개는 `%6/%7`·`%8/%9`). ⚠삽입은 role 정정 **뒤**에 놓았다(경로는 삽입 전 인덱스) — 적용 후 `--restamp` 필요.
- G13: knobs[0] 42385→42389 · knobs[1] 42480→42508(G13 미적발, 인용이 `i8 0` 리터럴) · knobs[2] 범위 정밀화.
- G10: open[1]·open[3] 문면을 물음으로 교체. **notes 로 옮길 사실(메인이 넣을 것)**: ①「diff 저울의 baseline 인자 = abandon.net_value(+0x30): m10.ll:42516~42519」 ②「closure#1/#2 는 인라인(define 없음), 극성·순서는 phi %113/%114(42688)로 확정, 키 = (dist², id)」.
- **빠진 행(siblings)**: `resolve_fight_stake_roster`(fight_model.rs:645 · m10.ll:47870 · `fn(usize,&OperationData,&Entity,&[(&Entity,i64,bool)],&[&Entity],i8,Option<&Entity>,usize)->FightPrediction`, closure 656/666 이 597 과 같은 꼴) — 35 의 호출처 m10.ll:16359/16464 바로 옆 16386/16527 에서 호출된다. siblings 자동열거가 못 잡았다(G3 사각).
- callees: `[0] ally_is_bound` 는 aux m10.ll:56201 `tail call fastcc … fight_model13ally_is_bound(i64 %20, ptr %21, …)` 로 **IR 호출 심볼 일치**(ev3 감) — `callees` 는 patch 경로가 없어 산문 보고. `[1][2] line`·`[4] resolve_fight_stake` 는 `logic` 산문(`.line`, 함수 자기 이름)에서 긁힌 잡음.

### 36 calculate_nexus_defense_count — 오라클 8세계
- 오라클(`A36`): 시작 상태 0 · `Game+0xed00+0x240 epic_minion_buff_time[1]=1000` raw write → **team0 만** enemy_has_epic(consts[3] `1-team` 극성 실행 확정) · `cache+0x21c0+team*8 top_lead[team]=0` raw write → weak_lead_state → epic∧weak 일 때만 +1(case4=1 / case5·6=0, AND 확정) · `data.context.tutorial=First` 만 바꾼 세계(case7) → 0(consts[0] `(tutorial-1)<u 6` 게이트 확정). **very_weak(적 미니언 150000 이내)·near_nexus(가시 적 챔프)·minion_floor(구조물 타격 미니언 6+) 경로는 미실행** — 미니언 생성(수법 ⑧ 600틱)·가시성 조작을 이번엔 시도하지 않았다(**미탐색**).
- ★**tutorial=First 로 세계를 만들면 넥서스가 없어 `handler.rs:2330:50` `Option::unwrap()` 패닉**(case2 실측, exit 101) — open[1] 「패닉 경로 도달 가능성」의 인접 사실. notes 후보: 「튜토리얼 1~6 세계는 nexus None → L2330 unwrap 패닉. 호출자가 게이트를 거는지는 미탐색」.
- open[0] 보강: **start_game 직후 양팀·3라인 line_lead 전부 2**(2 = 대등 기준선 가설). 계산식은 `_gcbc g15.ll:103509~`(simulation.rs:1697~1699, 27칸 격자 `icmp sgt i32 …, 2`) — **미탐색**.
- mem[8] 오귀속 해소: `top_lead[team]` 으로 이름을 한 필드로(D9-OFF) — IR m13.ll:56376/56410/56411 = cache+0x21c0+line*16+team*8, tcxdict 0x21d0 mid_lead/0x21e0 bottom_lead.
- G12 3건 **전부 오탐**: 리터럴이 aux 클로저(m12.ll 42894~42979)에 있고 `!dbg` 사슬이 정확히 2362/2364 를 가리킨다(`!72356`=2362, `!72374`=2364 switch). `srclinecheck` 가 `aux` 범위를 안 훑는다.
- G13 5건 정정(56495/56625/56917/56950/42965). knobs[2] 56920→56917 은 G13 미적발(±3).

### 37 i_am_chosen_defender — 오라클 190/190 × 7
- 독립 재구현(수법 ⓓ) 대조: 미니언·가시 푸셔 없는 세계에서 키 = (false, 0, dist², id) → need 0..=5 × exclude 3종 × 10명 + exclude=[me] = 190건 전수 MATCH(7세계). 확정: `need_count==0→false`·`exclude∋me→false`·아군 5슬롯 전부 비교·strict `<`·`better < need_count`·동거리 id 순. **cant_handle_pusher / -clear 성분은 입력 판별력 부재**(푸셔·미니언 없음) — 미실행.
- G16 P1: 4 인자 role 에 `%0/%1/%2/%3·%4` 명기.
- G12 5건 **전부 오탐**: [4] 317 = `trunc nuw i64 %281 to i1`(59386, `!68337→!68321` 317) — 리터럴 1 이 trunc 로 접힘 / [5] 318 = `trunc nuw i64 %293 to i1`(59412, 루트 318) / [16][17][18] = aux 클로저 m04.ll 15768/15801/15753 의 `!dbg` 루트가 464/465/474. 검사기가 (a) aux 범위 (b) `trunc…to i1` 을 못 본다.
- G7·G10 open[0]: `is_recent_visible`(shared 확정) 언급 제거 + 사실 꼬리 제거. **notes 후보**: 「expected_trade_net_hp / expected_damage_target / is_area 는 호출 계약만 사용: cant = net_hp<0(`lshr 63`), clear = dmg>0∧is_area ? dmg×wave_size : 0」. open[1] → 「L348 튜플 선언 순서는 mir=0 미탐색」으로 교체, **notes 후보**: 「키 비교 순서 bool(59270 `samesign ult i8`) → i64(59280 `slt`) → u64(59290 `ult`) → u64(59297 `ult`)」.
- G13 4건 정정(59363/59449/58996/15768·15801).

### 38 can_tower_focused_when_battle — 오라클 96/96 × 7
- 적 타워 8기 × d∈{0,5000} × dist∈{thr-1,thr,thr+1}(thr = range+(lv-1)·growth+stat_range+t.radius+champ.radius+d+15000): true/true/false 전부 MATCH → **`<=` 극성·15000·d 가산 실행 확정**. `tower_attack_disable_tick=0` 세계(case3) → 전부 false(knobs[1]). ⚠기본 타워는 `range=growth=stat_range=0, radius_mult=0` 이라 **(lv-1)·growth / stat_range / (100+mult)/100 항은 판별력 부재**(미실행).
- ★새 사실(open[0] 보강): `cache.towers(enemy)` 는 **넥서스를 포함**(9 = iter_towers_without_nexus 8 + Nexus 1). 기본 넥서스는 attack_effect=None 이라 건너뛰지만, 넥서스에 공격 이펙트가 있으면 이 함수의 판정 대상이 된다. 파괴 타워 제외 여부는 여전히 **미탐색**.
- G13 knobs[0] 51201→51185(`add i64 %5, 15000` 은 루프 밖 호이스트, `!dbg` 없음) + 51298 `icmp ugt`.

### 39 is_wave_priority_start_line
- IR 전수 대조: 분기 순서(`>8` 단락 → nexus None → first_tower None → 기본좌표 select → any(`Entity::distance < tower_distance`)) · 오프셋(top/mid/bottom_minions 0x10/0x50/0x90 phi [16,80,144] · first_tower 0x180+line·32 · nexus 0x170 · len +0x18) · 상수 6개 좌표 · 극성 전부 일치. 정정 = knobs[0] 52887→52901, consts[3] meaning 「(x,y) 를 뒤집는다」→ select 원문대로(team0 → (48000,272000)) 보강.
- 형제 0 · 호출처 2 그대로. 오라클 불가(`in:game_ai`).

## 2. §4-b 무게이트 축 표본 대조
- `mem.dir` 89행(5함수 전부): IR load/store 와 **불일치 0** (35 의 w 4행 = 42535~42537 store / 42691·42693 store / 42539 memcpy).
- `mem.offset` 79행(tcx 사전에 타입이 있는 행 전부): `tcxdict <base> <off>` **불일치 0** → 전부 ev3 상향(`oracle/tcx_mem_check.txt`). ⚠`Entity 0x88/0x90/0x128` 은 tcxdict 가 첫 variant 이름(`Bear`/`Eagle`)으로 답하지만 `--enum EntityType 1/2` + `Minion 0x18`·`Tower 0xb8` 로 Minion/Tower 페이로드임을 확인(명세가 맞다). **검사기 제안**: tcxaudit 이 `ty@<Variant>.…` 표기의 variant 를 `--enum` 페이로드로 교차 확인하도록.
- `one_line`: 5건 전부 IR 과 정합(35 의 「차분 판정」·39 의 「9+ ∧ 타워 안쪽」·38 의 「+15000」).
- `callees`: 35 `[0]` 은 aux 에서 IR 호출 일치(ev3 감) · `[1][2][4]` 잡음. 36·37·38·39 의 ev3 행(champions/minions/iter_minions/is_recent_visible/distance/towers)은 본문 call 로 재확인.
- `siblings`: 35 에 `resolve_fight_stake_roster` 누락(위).

## 3. 판정 어휘 (§5-b ①)
| 대상 | 판정 | 범위·재료 |
|---|---|---|
| 35·39 오라클 | **재료 부재** | `_tcx` `v=in:game_ai` · 루트 재수출 없음(`game_ai::resolve_fight_stake` 경로 없음). 시도하지 않은 것 = 상위 pub 래퍼(호출처 m10.ll:16359 소속 함수)의 오라클 — **미탐색** |
| 35 L597 `a.distance_sq(champ)` vs `champ.distance_sq(a)` | **표기 불가** | 동일 길이 20자, 동작 동일 |
| 36 very_weak/near_nexus/minion_floor 경로 | **미탐색** | 미니언 생성(수법 ⑧)·가시성 조작을 이번 라운드에서 시도 안 함 |
| 36 line_lead 의미 체계 | **미탐색** | `_gcbc g15.ll:103509~` 계산식 독해 |
| 37 cant_handle_pusher / clear 성분 | **미탐색(입력 판별력 부재)** | 가시 푸셔·AttackEffect 조립(TEMPLATE 함정 ④) 필요 |
| 38 (lv-1)·growth / stat_range / radius_mult≠0 항 | **미탐색(입력 판별력 부재)** | 타워 Entity 를 ptr::write 로 조작(수법 ⑦) |
| 36 tutorial 1~6 세계의 L2330 패닉 도달 가능성(호출자 게이트) | **미탐색** | 호출처 11곳(m09.ll·m13.ll) 독해 |

## 4. 실제로 실행한 것 (§5-b ②)
```
python -X utf8 dossierfresh.py 15 D                       # FRESH(착수) / STALE(제출 직전, §0 참조)
sh _verify3/build.sh C:/tfm2mods/MIG/_verify15/D/oracle/D15_o1.rs   # 41s / 44s(2회)
for k in 0..7: %TEMP%\\tfm2_spanprobe\\D15_o1.exe $k > _verify15/D/oracle/D15_o1_case$k.out   # case2 exit 101(패닉, 의도된 관측)
python -X utf8 tcxdict.py <Base> <off>  (mem 79행 일괄 → oracle/tcx_mem_check.txt)
python -X utf8 tcxdict.py --enum EntityType / TeamType / TowerType ; tcxdict Tower / Minion / Effect
python -X utf8 rmeta_srcmap.py game_ai fight_model.rs 596 598   # L597 = 100(LF 포함) → 99자
python -X utf8 specgate.py --only 35 --gate G16 ; python -X utf8 paramrole.py
python -X utf8 _verify15/D/oracle/mkpatch_D.py            # patch.json 생성(mkpatch 참조구현)
PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 15 --only D --dry   # 50/50 · 93/93 · 실패 0
```
IR 원문 확인 범위: m10.ll 42357~42695 + aux 56168~56212 · m13.ll 56190~56961 + aux m12.ll 42894~42979 · m04.ll 58647~59462 + aux 15665~15883 · m07.ll 51100~51309 · m15.ll 52851~53084 · `!dbg` inlinedAt 루트 42건.

## 5. 내 지시(도시에)의 오류 (§5-b ③) — patch.json `brief_errors` 6건
1. §4 G16 [35] 두 번째 항목 문면이 「8차 확정 규약 = sret 를」에서 잘림(규약 본문은 paramrole.py:346~348 에서 읽어야 했다).
2. §4 G12 [37] 5건·[36] 3건이 **전부 오탐** — `srclinecheck` 가 `aux` 범위와 `trunc … to i1` 접힘을 안 본다(검사기 결함).
3. §4 G13 이 35 knobs[1](`i8 0` 리터럴 인용)·36 knobs[2](±3)를 못 잡았다.
4. §1 표의 형제 0개 — 35 에 `resolve_fight_stake_roster` 가 있다(G3 사각).
5. `mkpatch.Patch.ev()` 가 `/specs[i]/mem[j]` 최상위 배열 경로를 거부한다(mkpatch.py:134, PATH group(3)/group(4) 혼동) — dict 직접 삽입으로 우회.
6. §0 신선도 — v3 가 도시에 생성 2분 뒤 재생성돼 착수 FRESH → 제출 STALE. 내용은 동일(§0 확인)이나 `mkdossier` 직후 정본을 다시 만들면 배치가 착수 확인만으로는 못 잡는다.

## 6. 판정 반전 (§5-b ④)
- **1건**: 35 knobs[2]/logic 「rescue_ally = distance_sq 최소」 → 「(distance_sq, id) 사전순 최소」. r7 명세의 결론이 틀렸다(동거리 타이브레이크가 슬라이스 순서가 아니라 id).
- 그 외 오탐 8건은 「명세가 맞다」 확인이라 반전 아님. G10/G7 4건은 분류 이동.

## 7. 다음 라운드에 남기는 것
- notes 삽입(메인): §1 의 35 ①②, 36 「tutorial 1~6 → L2330 unwrap 패닉」·「초기 line_lead=2」, 37 ①②, 38 「towers() 에 넥서스 포함」.
- 적용 후 `applypatch --restamp`(35 params 삽입).
- 검사기: `srclinecheck` 에 (a) `spec.aux` 범위 스캔 (b) `trunc nuw iN %x to i1` = `==1` 비교 취급 / G13 에 opcode 아닌 인자 리터럴 인용(`i8 0`) 대조 / tcxaudit 에 `ty@Variant` 페이로드 교차 확인 / siblings 열거에 `<name>_roster` 류 동형 시그니처 탐지.

소요: 약 30분(IR 독해 5함수 12분 · 오라클 작성·2회 빌드·8회 실행 8분 · tcx 대조·패치 생성·검증 10분). 막힌 지점: mkpatch.ev 경로 거부(우회 2분) · tutorial First 세계 패닉(ctx 만 바꾸는 우회로 해소).
"""
io.open(r"C:\tfm2mods\MIG\_verify15\D\REPORT.md", "w", encoding="utf-8").write(txt)
print(len(txt))
