> v0.2.6: 기존39개와 제자리 삽입·seed 출력 2개를 합쳐41개 준비. 신규2개 실게임 관측 전. 1016B00/E53290은 RAX+RDX 반환 어댑터 미지원으로 제외한다.

> v0.2.5: 기존37개+E20D00/1014470 총39개 준비. 1016B00은 None RDX 반환 소비로 설치 제외한다. 이번 두 모델은 실게임 관측 전이며 준비/관측/항목완료를 구분한다.

# AI 분석 검증 v0.2.4

실제 게임 호출은 원본을 한 번 실행하고 반환값을 유지합니다. 별도 Rust 모델이 같은 입력을 읽어 예상값을 계산합니다. 게임 헬퍼를 예상값 계산에 호출하거나 계수를 변경하지 않습니다.

고정 버전은 0.6.4 SHA `a1d05045405a12e2bed182daeafcad8c8a40cf29ab68ba6ea5bfd30257439582`입니다. 추적 집합은 741개이며 이번 실험 배선은 37개입니다. 옛 catalog.json/evidence_manifest.json은 첫 준비시점 동결 자료입니다. 현 배선은 `wave05_scope.json`과 REPORT/runtime_20261018_auto24/batch01를 따릅니다. 로컬 시험은 실게임 검증에 합산하지 않습니다.

|함수|비교 내용|
|---|---|
|dd87a0 / fe2700|18개 예측 표본, 간격5의 정수 보간|
|e52ee0|팀 주요 세 슬롯 존재|
|d98bd0|팀 다섯 슬롯의 첫 위치 일치 게이트|
|e49c00|존재 아군의 상대 기지 600000 반경|
|fa9dc0|다른 역할·같은 라인·엄격150000 거리|
|f51e50|두 시각 기록의 거절 조건|
|e9eb70|참여 레코드와 HP25%|
|fdb9a0|최대12개 예측 레코드의 첫 key 일치 포인터|
|dc0bb0 / f27860|HP40% 이상 개수|
|dc0f80 / f27690|HP50% 이상 개수|
|ef2410|역할1 제외 상대 네 슬롯과 포함150000 거리|
|fa9600|자기 ID 또는 동적 반경 조건|
|1019890|HP40%와 방향별 중앙192000·엄격64000 영역|
|fd19f0|태그1일 때 동적 배율/나눗값/상한의 정수 계산|
|fd37f0|동적 배율/나눗값/차감값 계산의 양수부|
|fe8f30|라인 레코드의 byte 플래그와 상대 구조물 존재|
|e1b430 / e1ede0|별도 예측 레코드 ID 조회와5tick 보간|
|d81ce0 / d81bd0|자기ID 제외 팀 슬롯 수와 상대팀 슬롯 수|
|e54190|라인 압력의 signed progress·count 문턱|
|ffa970 / fd4260|라인 슬롯 개방과 후보 HP30%·상태 게이트|
|dc0d80|HP50%·방향별192000/64000 영역 개수|

## 설치와 수집

기본 `armed=false`입니다. 활성화 후 전체 게임 재시작이 필요합니다. 설정은 시작할 때 한 번 읽습니다. 실험 전 세이브와 config를 백업하고 다른 AI 조정 모드를 꺼 원본 조건을 만듭니다.

타이틀에서도 배경 AI가 실행됩니다. 설치는 타이틀을 안전한 정지 지점으로 가정하지 않습니다. RX 원본 trampoline과 영구 breakpoint descriptor를 먼저 게시하고, 원자적인 INT3 진입 게이트를 세운 뒤 기존 스레드의 RIP를 검사합니다. tail 작성·읽기검사·명령 캐시 flush 뒤 첫 바이트를 게시합니다. 정확한 등록 entry의 breakpoint만 원본 trampoline으로 즉시 우회하며 대기하거나 게임 함수를 재호출하지 않습니다.

전체 SHA/본문이 다르면 설치를 거절합니다. 설치 실패 후 같은 실행에서 재시도하지 않습니다. `INSTALL_ERROR_ORIGINAL_RETAINED_ENTRY_UNVERIFIED`는 성공이 아니며 비교를 차단합니다. tail 작성 뒤 오류는 원문 rollback을 주장하지 않고 INT3 원본 우회를 유지합니다. 외부 VEH/debugger, prefix 중간 진입, private LTO 잔류 레지스터, EH/unwind의 모든 경우를 입증한 범용 detour는 아닙니다.

수집은 매 실행 꺼진 상태입니다. 목표 경기 진입 직전 다음 명령으로 해당 run_id를 확인하고 시작합니다. 실행당 수집 세션 하나만 허용하며 타이틀(scene1)은 제외하고 InGame4/GameTest5에서만 비교합니다. 이 scene은 상위 UI 상태여서 내부 리플레이 단계 자체를 증명하지는 않습니다. CLI는 UI를 조작하지 않고 관측 제어 파일만 씁니다.

```powershell
python -B -X utf8 tools/capture_control.py start --summary <모드폴더>/summary.json --run-id <현재run_id> --label <경기구간명>
python -B -X utf8 tools/capture_control.py stop --summary <모드폴더>/summary.json --run-id <현재run_id>
```

처음64개 적격 호출은 모두 채집하고 이후4096호출 간격, 함수당5000개 상한입니다. 파일 작업자는 UI 프레임과 별개로 owned event만 기록합니다. v0.2.2는 실행별 append-only summaries_<run_id>.jsonl을 정본으로 쓰며, 관측 원문 flush 뒤 완전한 요약 레코드를 append·flush합니다. summary.json은 호환 별칭이며 교체 실패는 별도 오류로 남깁니다. 정본 기록 실패는 수집을 닫습니다. stop은 같은 run의 새 완전 레코드에서 새 예약 차단·진행 중 비교0·enqueued=persisted를 확인해야 종료로 인정합니다. queue_full/queue_disconnected/io_errors/io_lost_or_uncertain_events는 각각 남기며 유실0을 가정하지 않습니다.

## 판독

v0.2.4의 probe_strides는 함수별 채집 간격입니다. 지정하지 않으면 전역 sample_stride를 쓰고 첫64/함수별 상한/큐 상한은 유지합니다. 실제 간격은 각 probe의 effective_sample_stride에 남깁니다. 고빈도 조회·예측은4096을 유지하고 저빈도 HP·영역 조건은 더 자주 관측합니다. 이는 게임 계수를 변경하는 설정이 아닙니다.

v0.2.3의 추가8개도 제한된 caller·프롤로그 검수에 근거한 실험 연결입니다. FFA970/FD4260의 역할표0,2,3은 고정 .rdata FILE 값이며 v0.2.3에서 로드된 table 값 직접 동일성은 별도 미검증이었고 v0.2.4는 시작 시 별도 읽기 영수증을 남깁니다. 출력 일치를 모든 정적 상수의 직접 관측으로 승격하지 않습니다.

`summaries_<run_id>.jsonl`과 같은 run_id의 `observations_<run_id>.jsonl`을 함께 보존합니다. MATCH는 관측 입력에 대한 예상 출력과 원본 출력의 비트 일치입니다. AL 반환은 AL만 비교하고 전체 RAX는 원본 그대로 반환합니다. 입력 변동은 UNSTABLE, 읽기/모델 실패는 SKIP, 외부 체인 결과는 FOREIGN으로 분리합니다.

함수 출력 일치와 각 임계점 아래/동일/위 관측을 별도 집계합니다. metric은 snapshot에서 계산하므로 `observed_branch=false`입니다. 내부 분기 레지스터를 직접 읽었다는 뜻이 아닙니다. 드문 전술/상황 및 데스매치 미관측은 남겨 두며, 표본0을 검증 완료로 처리하지 않습니다. 모든 계수의 독립 조절 가능성이나 전이 하위함수까지 검증됐다고 주장하지 않습니다.

첫 시험은 fdb9a0 집계1000일치/차이0, 저장512건 재계산 일치였으나 타이틀에서 상한을 소진했고488건이 유실됐습니다. 이는 경기 표본 검증이 아니며 `INCOMPLETE_WITH_LOG_LOSS`로 보존합니다. v0.2.0 비활성 시험에서는 요약 I/O 오류1건이 나와 실제수집을 시작하지 않았습니다. v0.2.1은17개 함수4234건 재계산 MATCH/관측유실0이지만 요약 rename 오류1로 중단됐습니다. InGame 준비+경기 혼합 관측이며 특정 리플레이 전용으로 해석하지 않습니다. 원문은 round02 각 고정 하위폴더에 보존합니다.

## 로컬 빌드

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File build_local.ps1 -RunTests
python -B -X utf8 tools/stage_wave05.py --report <이번회차REPORT경로>
```

고정 nightly-2026-05-24/stable SDK, opt1/overflow off로 빌드합니다. 로컬 명령은 게임을 실행하지 않습니다. 배포는 기존 폴더 백업·설치본 SHA·독립 검수를 거칩니다.

추가3개는 고정본문·제한caller 두 단계의 명시적 추가volatile 소비 미발견에 근거한 총괄 실험선택입니다. 더 깊은 opaque call의 private LTO 계약은 미확정이며, 전역 ABI가 증명됐다는 의미로 승격하지 않습니다. wave03_entry_selection.json에 원근거와 판단 범위를 남깁니다.

## 0.2.4 추가 관측

이력 선택·250k 거리, HP25/45·피해12%, HP40 집계 출력24B, 점수4개 안정정렬 출력32B, 커서5함수의 AL 및8B 변경을 대조합니다. 출력이 입력과 겹치는 미지원 경우는 SKIP하며 보호를 완화하지 않습니다. 첫 차이는 수집을 닫고 별도 원문을 보존합니다. 시작 시 loaded 상수6곳을2회 읽지만 전체 실행의 불변성·각호출 실제분기 trace를 증명하지 않습니다.
