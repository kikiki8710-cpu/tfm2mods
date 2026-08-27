# TFM2 모드 버전 마이그레이션 — 슬림 포인터 (2026-08-28 전면 재설계)

> ★**마이그 정본은 이 파일이 아니라 `MIG\` 다.** 절차 = `MIG\README.md`,
> 모드별 버전 민감 지점 전수 목록 = `MIG\manifest\<MOD>.json`, 도구 = `MIG\mig_verify.py`.
>
> 이 파일은 옛 참조(`MIGRATION.md §7` 등)를 살려두는 포인터만 남긴다.
> **여기에 새 이력·새 RVA 를 append 하지 말 것** — 그 누적이 0.5.7 마이그를 최악으로
> 만든 원인이었다(`MIG\RETRO-0.5.7.md`). 값 갱신 = 매니페스트, 함정 기록 = 매니페스트 notes.

## 요지 (패치가 오면)
```
python C:\tfm2mods\MIG\mig_verify.py check --exe <새exe>   ← STALE = 작업 목록 전부
```
이후 절차 = `MIG\README.md`. 완료 정의 = check 전 PASS + coverage 클린 + dups 연동 확인.

## §7. 현행 RVA 표 → `MIG\manifest\`
구 §7(버전별 RVA 표 + 세션 이력 §7.2-A1~A14 등, 636KB)은 통째로
**`_archive\MIGRATION-이력-2026-08-28.md`** 로 이동했다.
- 현행 값이 필요하면: `MIG\manifest\<MOD>.json` (0.5.7 채록 기준, 기계 검증 가능)
- 과거 경위·세션 이력이 필요하면: 위 아카이브 grep (버전 태그로 검색)
- 구 마이그 스크립트(mig056_* 등 35종): `_archive\mig_scripts\`
  (현행 엔진 = `_mig057.py` 의 match_fn/match_mid/find_unique_bytes + `migrate_rva.py`)

## 옛 섹션 안내
- §3 절차(migrate_rva) / §4 모드별 체크리스트(0.4.x 시절 3모드 — 전부 사멸) / §6 함정 목록
  → 전부 아카이브. §6 의 버전무관 교훈(에셋게터 string-xref, filter_handler len 추적 등)은
  해당 모드 매니페스트 notes 와 `MEM\` 메모리로 승계됐다.
