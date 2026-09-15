# r15 거대 7 + 미러 래퍼 1 편성표 (생성 2026-09-15 · rvaname 실명 `_next\r15_big_rvaname.txt` · 전부 [확정]/argscan)

> r14 보류 8(TLS 메모 소비자)의 미러 열쇠 = `position_eval_at`(EPC_CACHE) · `interaction_score`(INTER_CTX/LAG). `v47_siege_stance`(SIEGE_STANCE_CACHE)는 exe 에 독립 본체가 없다(0xca2570 = 안쪽 클로저 call_mut · 본체는 호출자에 인라인 추정) · `v48_cast_beams`(CAST_BEAMS)는 with-클로저(m00.ll:93237)가 exe 어디인지 미확정 → r15 뒤 namebycaller 로 후속.

| # | RVA | IR 실명(짧게) | IR | define 줄~끝 | IR 줄수 | exe 크기 | 호출자 | 비고 |
|---|---|---|---|---|---|---|---|---|
| 1 | `0xd851d0` | `position_eval::position_eval_at_uncached` | m07.ll | 24749~34229 | 9481 | 30055B | 1 | internal fastcc · exe 7 = IR 7(argscan 09-15) · sret 56B PositioningScore · EPC 값 생산 · ai_adjust port DIFF=0 선례(REPORT ai_adjust RE 09-06 position_eval) |
| 2 | `0xd5bbf0` | `action_score::calculate_interaction_action_score` | m05.ll | 40106~44246 | 4141 | 19586B | 1 | internal fastcc · i64 · exe 9 = IR 9(argscan 09-15 · ai_adjust 03 L2613 호출부 0xd57cc4) · agent_link 사본 대조 DIFF 3/483,612(09-09 보류) |
| 3 | `0xdc3240` | `move_actions::SmallActionRunAway::get_input` | m08.ll | 106076~110004 | 3929 | 14264B | 4 | hidden · sret 32B Option<Input> · &mut self 136B · pf@0x38(BOX_SUBST · heapsurf dealloc 1120/8@0x60 · 70/1@0x68) |
| 4 | `0xd57540` | `action_score::interaction_score` | m05.ll | 34861~37632 | 2772 | 9212B | 14 | pub · i64 · INTER_CTX/LAG 메모 작성자(r14 보류 8 의 미러 열쇠) · agent_link 대조 DIFF 49/1,047,028(09-09 보류) |
| 5 | `0xdbd260` | `move_actions::SmallActionRecall::get_input` | m08.ll | 99743~102302 | 2560 | 8891B | 1 | hidden · sret 32B · &mut self · pf@0x0(BOX_SUBST) |
| 6 | `0xd31f20` | `abstract_input::get_input_target` | m04.ll | 33111~35159 | 2049 | 8863B | 4 | internal fastcc · sret 24B Option<Input>(_OPT_INPUT) · ⚠argscan exe **10** ≠ IR 9(+0x28·+0x30 이 `imul 0x7d00` 좌표 스칼라 → ArgumentPromotion 의심 · 배치가 IR gep 전수로 대응표 · EXE_ABI FAKE 또는 UNRECOVERABLE) |
| 7 | `0xd59940` | `action_score::calculate_action_score` | m05.ll | 37635~39981 | 2347 | 8506B | 6 | pub · i64 range(-2^63,341) |
| 8 | `0xd84db0` | `position_eval::position_eval_at` | m07.ll | 24507~24711 | 205 | 841B | 32 | ★미러 래퍼 · pub · sret 56B · exe 7인자 = IR 7(argscan 09-15 · +0x30 i8) · EPC_CACHE TLS with-클로저 인라인 · 게임 판마다 EPC 를 채우는 작성자 → r14 보류 8 재판 열쇠 |

## 분책 기준(04 §8 · METHOD_MAP 「거대 함수 명세 분책」)
- IR 줄수 ≥ 5,000 → 루트 줄 지도로 소스 줄 범위 분책(`mergespec.py _spec\r15\<접두>`) · 그 미만은 단일 배치.
- internal 3(d851d0·d5bbf0·d31f20) = `argscan.py` 로 exe 인자 수 = IR 확인 후 patches.json 노출(ailink).
