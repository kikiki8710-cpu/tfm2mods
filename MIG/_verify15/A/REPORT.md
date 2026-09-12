# 15차 배치 A 보고 — #20~#24 (게임 0.5.8)

패치 = `_verify15/A/patch.json` (정정 30 · ev상향 142 · brief_errors 5 · `applypatch.py 15 --only A --dry` = **30/30 · 142/142 · 실패 0 · 동작 변경 2**).
오라클 = `_verify15/A/oracle/{o1,o2}.rs` → `o1.tsv`(285행) · `o2.tsv`(628행). **총 848 비교행 MATCH · MISMATCH 0.**

## 0. 신선도 (§0)
- 착수 시 `dossierfresh 15 A` = **FRESH**. 작업 중 08:31 에 배치 B/C/D/M 패치가 적용돼(`_verify15/{B,C,D,M}/applied.json`) v3 해시가 바뀌고 **STALE** 이 됐다.
- 담당 20~24 를 `specs20.bak-2026-09-13-0757-addspec.json` 과 **이름 기준**으로 대조: 다섯 함수 모두 `exe` 블록만 변경, 내가 고치는 필드(consts/knobs/logic/sig.params)는 불변 → 진행. 다른 배치 patch.json 에 `/specs[2x]` 경로 0건. `old` 는 생성 시점의 현재 v3 와 대조·통과.
- v3 배열은 39→40 으로 끝에 1개가 추가됐다(인덱스 20~24 불변).

## 1. 함수별 결과

| # | 함수 | 정적 대조 | 오라클 | 정정 | 반전 |
|---|---|---|---|---|---|
| 20 | v27_active_objective_discipline | mem 18/18 tcx 일치 · consts 6/6 사슬 일치 · vtable 3슬롯 divtable 일치 | o1 V27 6 + V27_HP 10 = **16/16** (kind 니치·target·until strict·36% 경계 359→None/360→Some·Serpen 리스트 독립) | 2(보강) | open[2] 표기불가 → **반전**(`as_moba`+`unwrap` 사슬) |
| 21 | upgrade_item | mem 22/22 · consts 5/5 · 1차 루프의 정체가 `any()` 였음 | o2 UPG **50/50** (튜플·RNG 선택까지 동일) | 4 (실오류 2·보강 1·오탐 1) | open[0] 표기불가 → **부분 반전**(구조 확정) · open[2] → 사실 |
| 22 | can_tower_focused | mem 24/24 · **15000 이 Effect::range 상수가 아님** · consts[6]/[7] 실오류 | o1 CTF 48 + CTF2 24 + CTF_NE 6 + GATE 1 = **79/79** (15000 판별 24행·전체 사거리식·radius_mult sext·nearest_enemy 두 경로·disable_tick 게이트) | 15 (실오류 10·보강 3·i 번호 5 포함) | knobs[1] 효과 **판정 반전** · open[2]/[3]/[4] → 사실 |
| 23 | buy_item | mem 20/20 · consts 20/20 사슬 일치 | o2 BUY **540/540** (5 카테고리 × hp 6 × 보유 6 × gold 3, 1550/1800 경계 판별) | 3 (보강 2·오탐 1) | open[0]/[1] 표기불가 → **반전**·사실 |
| 24 | v23_healthy_allies_near_point | mem 7/7 · consts 3/3 | o1 V23 150 + V23HP 6 = **156/156** (ule 경계 판별·hp≥min 경계 36/37) | 6 (i 번호) | open[0] → 사실(tcx 시그니처) |

behavior_change **2건**(둘 다 #22): `consts[2].meaning` 과 `logic` L27~30 보강 — 명세대로 `Effect::range` 에 15000 을 넣어 재구현한 뒤 그 함수를 L28 의 미니언 카운트 술어(`is_in_range`)에 재사용하면 카운트가 틀린다. `can_tower_focused` 자신의 반환값은 변하지 않는다(그래서 다른 #22 항목은 false).

## 2. 실제로 실행한 것 (명령줄)
```
cd /c/tfm2mods/MIG && python -X utf8 dossierfresh.py 15 A                       # FRESH(착수) / STALE(08:31 이후, §0)
python -X utf8 dloc.py m09.ll 24854 24875 24904 24889 24905 24913 24943 24944 24953 …   # #20 사슬 25개
python -X utf8 dloc.py m14.ll 20461 20471 20545 20663 20772 20818 20822 20824 20857 …   # #21
python -X utf8 dloc.py m07.ll 61192 61207 … 61471 61191                                  # #22 38개
python -X utf8 dloc.py m14.ll 23154 23224 … 23520                                        # #23 40개
python -X utf8 dloc.py m15.ll 60048 60049 60060 60083 60138 60147 60168 60172 60173 60188 60081   # #24
python -X utf8 tcxdict.py ObjectiveDisciplineState --deep / TeamPlan 0x130 0x149 / MobaMode 0x1a0 0x1d8 / Entity 0x628 0x670 0x68 0x88 0x98 0x438 0x470 0x490 0x4a0 0x4a8 0x4c0 0x5c0 0x5c8 0x660 0x668 0x680 / PlayerState 0x4a0 0x4f0 0x510 0x930 0x998 0x9c0 / GameContext / AbstractGameWithCache 0x1e0 / GameSetting 0x13f8 / Effect --deep / BaseItemInfo / SwordmanChampionInfo
python -X utf8 tcxdict.py --enum ObjectiveDisciplineKind JungleType GameMode EntityType ItemCategory ChampionCategory
python -X utf8 divtable.py AbstractGame 0x28 0x40 0x1f0 / ItemInfo 0x50 0x68 0x70 0x80
grep -n "^define.*AbstractGameWithCache6towers\|12iter_minions" /c/tfm2mods/_gcbc/*.ll   # g15.ll:109077 / 102624
grep -n "^define.*6Effect11is_in_range" /c/tfm2mods/_gcbc/*.ll                          # g06.ll:51634
grep -c "i64 15000" /c/tfm2mods/_gcbc/g06.ll                                              # 0
python -X utf8 rmeta_srcmap.py game_ai tower_discipline.rs 9 53
sh _verify3/build.sh C:/tfm2mods/MIG/_verify15/A/oracle/o1.rs && %TEMP%/tfm2_spanprobe/o1.exe > _verify15/A/oracle/o1.tsv
sh _verify3/build.sh C:/tfm2mods/MIG/_verify15/A/oracle/o2.rs && %TEMP%/tfm2_spanprobe/o2.exe > _verify15/A/oracle/o2.tsv
python -X utf8 _verify15/A/oracle/mkpatch_A15.py                                          # mkpatch.Patch 경유 생성(JSON 은 json.dump — heredoc 미경유)
PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 15 --only A --dry
```
오라클 입력 무결성: `setting_ok=true`(real_setting) · towers 16 · twin 2/2 · TLS 메모 없음(5함수 모두 순수, `tlsscan` 대상 아님).
**default 챔프의 `stat_cached.hp == 1`** — 1차 실행에서 36% 임계가 hp 0/1 로 붕괴해 무판별이었다. `Entity.stat_cached.hp = 1000` 을 주입해 재측정(TEMPLATE 함정 목록에 없음 → brief_errors).

## 3. 게이트 미해소 9건의 처분 (§4)
| 게이트 | 항목 | 판정 | 처분 |
|---|---|---|---|
| G1 | 20 consts[1] 490 vs 326 | **오탐성 문면** — 같은 inlinedAt 사슬(326 ← 490) | meaning 에 `→ 루트 490` 표기(보강) |
| G12 | 22 consts[6] 1@25 | **실오류** — IR 리터럴은 `-1`(`add i64 %level, -1`) | value 1→-1, meaning 정정 |
| G12 | 22 consts[7] 1@21 | **실오류** — L21 의 Some 은 `trunc i64→i1`(리터럴 없음); 리터럴 1 의 진짜 소비처는 L17 `1 - team` | src_line 21→17, meaning 을 적팀 인덱스로 (Some 태그 사실은 mem[10] 에 보존) |
| G16 | 21 params 7행 vs 6 | **오탐** — `&dyn AbstractGame` 이 (data,vtable) 2행 | params[4].role 에 근거 기입 |
| G16 | 22 i 번호 | 실오류(규약) | i 0..4 → 1..5 |
| G16 | 23 params 6행 vs 5 | **오탐** — 위와 같음(sret 없음 + dyn 2분할) | params[3].role 에 근거 |
| G16 | 24 i 번호 | 실오류(규약) | i 0..5 → 1..6 |
| G5 | 21 / 23 sig.tcx 5 vs params | **오탐(파생 필드 오류)** — `sig.tcx/vis/path` 가 동명 **트레이트 메서드**(pub, lib.rs:440/437)를 가리키는데 IR 심볼은 `define hidden` 자유함수(`in:game_ai`, lib.rs:1603/1477) | v2 에 없는 파생 필드라 patch 불가 → §5 도구 수정 |

## 4. 게이트가 안 보는 축에서 나온 것 (§4-b)
1. **`consts[2]` 15000 의 귀속(#22)** — meaning·knobs 가 「game_core Effect::range 공용 상수, 이 함수만 못 바꿈」이라 했으나 **거짓**. `_gcbc/g06.ll:51634 is_in_range_ex` 가 같은 effect.rs:26 `range` 를 인라인하는데 15000 이 없고 g06.ll 전체에 15000 이 0건. IR `!dbg` 가 effect.rs:26 을 가리키는 것은 add 재결합 아티팩트(두 사이트 L25/L40 모두). 오라클이 `R = range(t)+15000+t.radius()+champ.radius()` 경계를 R 참/R+1 거짓으로 24행 판별(대립가설 기각). **어떤 게이트도 `!dbg` 귀속의 진위를 보지 않는다** — 검사기 제안: 상수 X 의 `!dbg` 루트가 다른 크레이트 함수 F 로 잡히면, `_gcbc` 에서 F 의 다른 인라인/아웃오브라인 사이트를 찾아 X 가 거기에도 있는지 대조(없으면 「호출자 리터럴」로 재귀속).
2. **`knobs.where` 가 사이트를 하나만 적는다(#21)** — 임계 4 가 L1610(`any`)과 L1614(`filter`) 두 곳. G13 은 「인용 줄에 명령이 있나」만 보므로 누락 사이트를 못 본다. 검사기 제안: 같은 값·같은 함수 안의 `!dbg` 루트 줄이 2개 이상이면 `where` 에 전부 있어야 통과.
3. **`logic` 의 IR 해석 주석이 틀림(#21)** — 「1차 루프로 첫 원소 탐색」이 실제로는 `slice::iter::any` 였다(inlinedAt 프레임 이름으로 확정). 재현엔 무해(외연 동일)라 보강.
4. **`callees` ev4 행** — divtable 로 `AbstractGame` 0x28/0x40/0x1f0, `ItemInfo` 0x50/0x68/0x70/0x80/0xa0, `ChampionInfo` 0x20/0x30 슬롯이 IR 오프셋과 일치함을 확인했으나 **v3 파생 배열이라 ev_up 이 거부됐다**(`인덱스 범위 밖` 15건 → 제거). callees 근거를 실을 자리가 계약에 없다.
5. **불일치 0 인 축**: mem 91행(tcx 전수 일치), consts src_line 42행(G12 지적 2건 외 전부 사슬 일치), knobs.value 18행, sig.params.role(dyn 2분할 외 일치).

## 5. 내 지시(도시에)·도구의 오류 (§5-b 3)
1. §1 `vis=pub` for 21·23 — **거짓**(brief_errors[0]). 원인 = `mkspec3.conv()` L510~516 이 leaf 이름+파일명으로 **첫** tcx 항목을 고른다(`lib.rs` 에 `upgrade_item` 이 3개: 트레이트 impl 440 · inherent 1193 · 자유함수 1603). 수정 = `sp.src_line`(1603) 과 tcx `sp.l` 을 대조해 고르기. 이 오류는 sig.tcx/vis/path/G5/G16 오탐의 공통 원인이다.
2. G16(P3) 행 수 규칙이 `&dyn`/`&[T]`/`&str` 팻포인터 2분할을 안 센다(brief_errors[1]).
3. **`mkpatch.Patch.ev()` 가 2단 경로를 전부 거부한다** — `if not m or m.group(3) is None` 에서 group(3) 은 applypatch PATH 의 `outer` 이고 인덱스는 group(4). `/specs[20]/mem[0]` 같은 정상 경로가 `ValueError` 로 죽는다. 이 배치는 같은 계약의 `_ev` 를 몽키패치해 우회했다(`oracle/mkpatch_A15.py` 상단). 이전 라운드들이 어떻게 통과했는지 확인 필요(각자 우회했다면 6차 재발명 사고의 재판).
4. §4 G12 「실제 후보 [17,52]」는 정답이었다(brief_errors[2]) — 7차 「G12 는 전부 오탐」 교훈을 이 두 건에 그대로 적용했으면 틀렸다. **게이트 적발은 가설**이고, 오탐 전례가 기각 근거는 아니다.
5. TEMPLATE.rs 에 `stat_cached.hp == 1` 함정 부재(brief_errors[4]).
6. §0 신선도가 4배치 동시 실행에서 남의 패치 적용으로 STALE 이 된다(brief_errors[3]) — 함수 단위 변경 알림이 있으면 재독 없이 진행 가능.

## 6. `open[]`·`notes[]` 처분 — 산문(계약상 patch 불가 · 메인이 넣는다)
판정 어휘: 미탐색 / 재료 부재(범위 열거) / 표기 불가 / 사실 서술.

**#20**
- open[0] version 미사용 → **사실 서술** (`#dbg_value(i64 poison, !24837)`; 호출자 5곳은 미탐색 그대로).
- open[1] 다른 캠프에 규율이 세팅되나 → **미탐색** 유지(`update_v27_objective_discipline` objective_discipline.rs:334 미독해). 단 함수 자체는 Rhino(0) 인자로도 Some(state) 를 돌려준다(오라클 「Rhino target」 MATCH).
- open[2] **표기 불가 → 반전**: 소스 = `get_game_mode().as_moba().unwrap()` (inlinedAt: `as_moba` game.rs:231 + `unwrap<&MobaMode>` option.rs:1013). `match … unreachable!()` 이면 unwrap 프레임이 없다.
- open[3] wait_pos·until_tick 세팅 규칙 → **미탐색** 유지(범위: `v27_objective_wait_pos` :316 · `update_v27_objective_discipline` :334).
- closed[0] 유효(이 라운드도 `_gaibc` 에 v27_objective_entity define 0건).

**#21**
- open[0] **표기 불가 → 부분 반전**: 구조 = `if items.iter().any(|it| it.tier()<4) { filter(..<4).map(tier).max() … } else { 0 }` 로 확정(closure$0 any@1610 · closure$1 Filter/closure$2 Map/max_by@1614). 남는 표기 불가 = `max().unwrap_or(0)` vs `if let Some` (unwrap_failed 없음 = `unwrap()` 은 아님).
- open[1] L1626 세 조건 순서 → **사실 서술**: `&&` 단락 + 세 항이 전부 불투명 vtable 호출이라 LLVM 이 재배열 못 함 ⟹ IR 순서(is_active → tier → price) = 소스 순서. (칼럼 없음과 무관)
- open[2] version/game 미사용 → **사실 서술**: pub 래퍼 `AgentVerHamster::upgrade_item`(lib.rs:1193, m14.ll:35109)이 `i64 poison`·`ptr nonnull poison` 을 넘긴다. **래퍼는 `player.info.item_builds.len()==0`(PlayerState+0x4f0) 일 때만 이 레거시 함수를 부르고, 아니면 `item_v26_slot`/`item_v26` 경로** — notes 후보(명세 어디에도 없던 사실).
- open[3] item_index_by_key TLS 메모 → **미탐색** 유지(오라클에선 키 조회 15/15 정확·missingkey None).
- open[4] → **사실 서술**(divtable 슬롯이 IR 오프셋과 일치·오라클 실행).

**#22**
- open[0] nearest_enemy.0 의미 → **미탐색** 유지(tcx: `ty@Tower.info.nearest_enemy@Some.0.0`, 오라클은 0 을 써 넣어도 무관 — 이 함수는 .1 만 읽음).
- open[1] closure$0 즉시호출 여부 → **표기 불가** 유지(외연 동일).
- open[2] towers/iter_minions 본문 → **사실 서술**: `towers(team, pool)`(g15.ll:109077) = `top_tower/top_tower2/mid_tower/mid_tower2/bottom_tower/bottom_tower2[team]`(cache+0x180~0x1d0, Some 만 push) + `twin_towers[team]`(cache+0x130) 전부; `iter_minions(team)`(g15.ll:102624) = `top_minions[team] ⊕ mid_minions[team] ⊕ bottom_minions[team]`(cache+0x10/0x50/0x90) Chain — **추가 필터 없음**. 생존/가시성 필터는 cache 생성(`AbstractGameWithCache::new`) 쪽이라 미탐색.
- open[3] is_in_range 본문 → **사실 서술(반증)**: `is_in_range → is_in_range_ex`(effect.rs:64/79~80, g06.ll:51634) = `dist² <= (Effect::range(caster) + ty.range_adjust(vtable+0xe8) + [caster.radius() if casting==0] + target.radius())²` — **L34 식과 다르고 15000 이 없다**. 재현 시 L28 술어를 L34 식으로 대체하면 틀린다(behavior_change true 2건의 근거).
- open[4] L34 `cnt<2 && dist<=R²` 순서 → **표기 불가**(둘 다 부작용 없는 순수식이라 순서가 결과에 영향 없음; cnt 자체는 L27~30 문장에서 먼저 계산됨).
- open[5] L10 `>=` 조기반환 vs `if tick < X {closure()} else {false}` → **표기 불가** 유지.

**#23**
- open[0] **표기 불가 → 반전**: `slice::iter::macros.rs:332 any<…closure_env$0>` 프레임 = `.iter().any(|x| x.tier() < 4)`.
- open[1] stat() 재호출 → **사실 서술**: sret alloca 두 개(%9 L1523 · %8 L1536)로 vtable+0x30 을 두 번 호출 — 지역변수 재사용이면 1회였을 것.
- open[2] version/game → **사실 서술**(#21 과 동일 래퍼 구조: m14.ll:38174, item_builds 게이트).
- open[3] ChampionCategory 5 이상 → **사실 서술**(tcx variant 5개, unreachable).
- open[4] Support(6) 비후보 → **사실 서술**(오라클 540행 중 idx 6 후보 0회). 의도 여부는 재료 부재(소스 주석·_docs grep 0건).
- open[5] `icmp ult len, 2^59` assume → **사실 서술**(컴파일러 삽입).

**#24**
- open[0] v23_healthy 시그니처 → **사실 서술**: tcx `fn(&Entity, usize) -> bool`(objective_helpers.rs:11, in:game_ai); IR 동작 `hp*100/stat_cached.hp >= min`.
- open[1] 사망 챔피언이 None 인지 Some(hp=0) 인지 → **미탐색** 유지(범위: `AbstractGameWithCache::new` g15.ll, 오라클로는 사망 상태 재현 필요). 오라클에서 hp=0 Some 은 `min_hp>0` 이면 제외·`min_hp=0` 이면 포함됨을 확인(V23HP min_hp=0 → 5).
- open[2] 호출자 값 → **미탐색** 유지(호출처 8곳).
- notes[0] healthy→거리 순서 → 유지하되 근거 추가: 두 항이 순수식이라 **표기 불가**(순서 무관).

## 7. 막힌 지점 · 소요
- 막힘: (1) `mkpatch.Patch.ev` 2단 경로 거부(우회) (2) callees ev_up 불가(파생 배열) (3) default 챔프 hp=1 로 1차 오라클 무판별(재측정) (4) `champ()` 카테고리가 전부 Melee(private `category` 필드를 tcx 오프셋 +0x1f0 으로 직접 써서 해결) (5) 신선도 STALE(원인 특정 후 진행).
- 시도 2회 후 중단한 것: 없음. 재료 부재로 남긴 것: #23 open[4] 의도(소스 주석 없음) · #24 open[1](cache 생성 미독해).
- 체감 소요: 정본 3종 독해 ~15%, IR·사슬 정적 대조 ~35%, 오라클 2종 작성·빌드·재측정 ~35%, 패치·보고 ~15%.
