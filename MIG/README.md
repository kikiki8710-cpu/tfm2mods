# MIG — 매니페스트 구동 마이그레이션 (0.5.7 재설계, 2026-08-28)

> **마이그 = "매니페스트의 각 엔트리를 새 버전에서 찾는 것."**
> 소스 스캔 휴리스틱·기억·이력 문서에서 즉석 재구성하던 구 방식은 폐기
> (왜 = `RETRO-0.5.7.md` — 0.5.6→0.5.7 에서 stale 사고 10건 전부 사후 발견).

## 구성
```
MIG\
  README.md            이 문서 (절차)
  RETRO-0.5.7.md       재설계 배경 = 0.5.7 마이그 실패 회고
  mig_verify.py        도구: gen / check / coverage / dups / rebase
  manifest\<MOD>.json  ★모드별 버전 민감 지점 전수 목록 (기계 정본)
```
매니페스트 엔트리 = 이름 · 값 · 버전 · 종류 · 소스 위치 · **현행 exe 채록 바이트(12B)** · 재탐색 방법.
`offsets` = 구조체 오프셋 축(exe 대조 불가 — RE 로 검증), `notes` = 모드별 함정, `build` = 빌드 명령.

## 패치가 오면 (표준 절차)

```
① 새 exe 확보 (백업: C:\Users\jungs\Desktop\claude\tfm2\tfm2_<신버전>\)
② python MIG\mig_verify.py check --exe <새exe>
     → STALE = 이동한 지점 전수 목록. 이것이 이번 마이그의 작업 목록이다.
     ⚠PASS 여도 안심 금물 아님 — PASS = "바이트 그대로" = 진짜로 안 움직였다는 뜻(강한 증거).
③ 모드별로 STALE 엔트리 재탐색:
     1순위 = 매니페스트의 bytes 12B 를 새 exe 에서 find_unique (단일 매치면 끝)
     2순위 = _mig 엔진 match_fn(함수시작) / match_mid(컨테이너 승계)
     3순위 = ghidra-re (본문 변경 = 로직이 바뀐 것)
     ⚠컨테이너 승계 실패 ≠ 재핀 불가 — sig 단일 매치로 잡히는 경우 있음(banpick_order HL_COUNT).
④ 소스 수정 + 매니페스트 value 동시 갱신 (둘이 어긋나면 coverage 가 잡음)
⑤ python MIG\mig_verify.py rebase <MOD> --exe <새exe>   ← 바이트 재채록
⑥ 완료 판정 3종 세트 (전부 통과해야 "마이그 완료"):
     check 전 PASS  +  coverage 클린  +  dups 로 연동 그룹 동시 갱신 확인
⑦ offsets(구조체 축)는 exe 대조 불가 — 각 항목의 verify 방법(RE/런타임 스캔)으로 별도 확인
⑧ SDK 교체(sdk_<신버전>) 후 전 모드 재빌드 — ★RVA 0 모드도 재빌드 필수(rlib DIFF)
⑨ 인게임 검증 → REPORT 검증표 갱신 → rel_commit
```

## 개발 규칙 (마이그 아닐 때)
- 소스에 **RVA 대역 상수를 새로 넣으면** `gen` 재실행(기존 큐레이션은 보존됨).
  안 하면 다음 `coverage` 가 미등록으로 잡아준다 — 그게 이 시스템의 안전망이다.
- 구조체 오프셋을 새로 확정하면 해당 모드 매니페스트의 `offsets` 에 수동 추가.
- 새 모드 = `mig_verify.py` 의 `MODS` 목록에 추가 후 `gen`.
  ★설치 여부 기준이 아니다 — **공유모듈(ui_kit)·mod_info 유실 모드도 이 목록에 있어야 한다**(0.5.7 사고 1·9).

## 사각지대 7종 — 이 시스템이 어떻게 막나
| # | 사각지대 (0.5.7 실사고) | 방어 |
|---|---|---|
| 1 | 함수시작 아닌 중간 사이트 | check(바이트 대조는 종류 무관) |
| 2 | .rdata 주소 (RUNNER_VT) | check + sect 필드(.text 아님이 명시됨) |
| 3 | 코드 인라인 imm (byte-patch 대상) | check |
| 4 | 여러 줄 배열 (CPROD_CALLSITES) | 추출기가 const 8줄 창으로 배열 원소 귀속 |
| 5 | 공유모듈 (ui_kit) | MODS 목록에 명시 포함 |
| 6 | 콜사이트 대응 오류 | method 필드에 판별자 기록(재발 방지) |
| 7 | 인라인 리터럴 (rva == 0x…) | 추출기가 const 여부 무관 전수 추출 + coverage |
| + | 로컬 복사본 (0x1788×3모드) | **dups** — 같은 값 보유 모드 그룹 보고 |
| + | 침묵 사망 (sig fail-safe) | check 는 로그·증상과 무관하게 exe 만 본다 |
| 8 | 무효 구값 엔트리 (HR_AE_FN 오핀 08-28 — 구값이 함수시작 아닌데 바이트 이주가 유일 매치 통과) | **재핀 전 구값 유효성 선검증**(함수시작=프롤로그/vtable=.rdata)·무효=ghidra-re 재규명·`*_FN`은 재핀 후 함수시작 검증 |

## 현행 상태 (2026-08-28 채록 기준)
- 0.5.7 정합: item_tactics·champ_pos_lock·comptest·banpick_order·banpick_illust·serpen·
  draft_overlay·level_cap·champion_exclude·bancard_keep·ui_kit + SDK 전용 6종
- **미정합(의도)**: `tfm2_ai_adjust` = 0.5.7-PENDING(잔여 49+345, 별도 세션) /
  `stat_exp`·`flow_capture` = 0.5.5 잔존(배포용 아님, 보류) / `sylas` = 0.5.6(유저 제외)
  → 각 매니페스트 notes 에 명시. ⚠이들의 bytes 는 "0.5.7 exe 의 그 주소" 채록이라
  **이동 감지용으로만 유효**(값 자체의 정당성 보증 아님).

## 구버전 자료
- 구 MIGRATION.md(636KB, §7 세션 이력 누적) → `_archive\MIGRATION-이력-2026-08-28.md`
- 구 mig 스크립트 35종 → `_archive\mig_scripts\` (현행 엔진 `_mig057*.py`·`migrate_rva.py` 만 잔류)
