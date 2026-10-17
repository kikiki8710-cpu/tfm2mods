# 카탈로그와 오프라인 관측 도구

게임을 실행하거나 설치 폴더를 변경하지 않는다. Python과 Capstone이 있는 분석 환경에서 사용한다.

```powershell
python -B -X utf8 tools/generate_catalog.py --model-scope C:/Users/jungs/Desktop/claude/tfm2/mods_report/tfm2_ai_adjust/runtime_validation_prep_20261018/probe_scope.json --model-preparation C:/Users/jungs/Desktop/claude/tfm2/mods_report/tfm2_ai_analysis_verify/tested_model_preparation.json
python -B -X utf8 -m unittest discover -s tests -p 'python*.py' -v
python -B -X utf8 tools/analyze_logs.py --summary <run-dir>/summary.json --observations <run-dir>/observations_<run-id>.jsonl --catalog catalog.json --output-dir <analysis-dir>
```

실행 시작 영수증이 별도로 있으면 집계기에 `--session <session.json>`을 전달한다. session과 summary의 `run_id`, baseline SHA가 다르면 중단한다. summary는 분석할 JSONL과 같은 실행의 **최종 파일**을 사용한다. 집계 중에는 게임 로그를 변경하지 않는다.

카탈로그는 기존 census 741개를 보존한다. 고정 PE·현재 설치 SHA·PDATA·FILE/BSS·본문 길이/명령수·원래 계약/원문 SHA를 확인한다. `tools/data`의 본문은 현재 버전 바이트다. `src/generated.rs`에는 승인된 8개 중 ABI와 진입 조건을 충족한 항목만 공개한다. 첫 12B 전에 branch/CALL/RET/RIP 상대명령이 나오면 제외한다. 완전한 명령으로 12~32B를 만든다. 원문 계약의 consumer 분류는 독립 조절 가능한 정책 임계값 수로 합산하지 않는다.

`entry_ready`, `model_implemented`, `paired_wrapper_prepared`, `runtime_no_samples`는 별개다. A의 `implemented_model_ids` 영수증은 모델 구현만 뜻한다. 후속 준비 영수증은 다음 형식으로 명시할 수 있다. `paired_wrapper_prepared=true`에는 `tests_passed=true`가 필요하며 모든 source SHA가 현재 파일과 같아야 한다.

```json
{"schema_version":1,"baseline_sha256":"a1d05045405a12e2bed182daeafcad8c8a40cf29ab68ba6ea5bfd30257439582","tests_passed":true,"source_files":{"C:/tfm2mods/tfm2_ai_analysis_verify/src/models.rs":"<SHA>","C:/tfm2mods/tfm2_ai_analysis_verify/src/engine.rs":"<SHA>"},"probes":[{"rva":"0xe49c00","model_implemented":true,"paired_wrapper_prepared":true}]}
```

관측 0은 성공이 아니다. vanilla/foreign chain, 반환 mask, 부분 output 차이, read fault/입력 변이/모델 panic/skip/drop/IO 손실을 분리한다. 최종 summary 누적계수와 bounded JSONL 표본을 합산하지 않는다. `observed_branch=false`는 snapshot에서 계산한 predicate이며 실제 분기 계측이 아니다. 아래/같음/위가 모두 있어도 전체 입력·모든 분기·실제 provider·런타임 검증 완료를 뜻하지 않는다. Python 단위시험은 오프라인 도구 시험이며 게임 검증 건수에 더하지 않는다.

`processing_errors`는 replay/output read/후처리 panic 실패이며, `unlogged_errors`는 오류 이벤트 기록 실패다. `io_lost_or_uncertain_events`는 drain된 batch의 기록 보장 불가 상한으로 일부 실제 기록된 이벤트도 포함할 수 있다. 이 상한을 `dropped` 등과 더해 유일 사례 수를 만들지 않는다. `SKIP`의 expected/mask가 null인 경우는 기대값 0으로 해석하지 않는다. summary 계수가 JSONL보다 많으면 부족분과 관측 범위 불완전을 표시한다. stride/예산/동시 snapshot으로 생길 수도 있으므로 차이 전부를 IO 손실로 단정하지 않는다.

현재 준비 상태 정본은 `tools/validation_receipt.json`이다. `validation_receipt_pre_final_build.json`은 빌드 전 paired0 시점의 역사 영수증이며 현재 카탈로그 판정에 사용하지 않는다. 위 생성 예제는 최종 로컬 시험 영수증에 결박된 `tested_model_preparation.json`을 사용한다. 이전 구현완료 전용 영수증으로 생성하면 paired 준비를 0으로 되돌리므로 현재 상태 재생성에는 사용하지 않는다.
