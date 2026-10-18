# AI 분석 검증 v0.2.0

실제 게임 호출은 원본을 한 번 실행하고 반환값을 유지합니다. 별도 Rust 모델이 같은 입력을 읽어 예상값을 계산합니다. 게임 헬퍼를 예상값 계산에 호출하거나 계수를 변경하지 않습니다.

고정 버전은 0.6.4 SHA `a1d05045405a12e2bed182daeafcad8c8a40cf29ab68ba6ea5bfd30257439582`입니다. 추적 집합은 741개이며 이번 배선은 16개입니다. 옛 catalog.json/evidence_manifest.json은 첫 준비시점 동결 자료입니다. 현 배선은 `round02_scope.json`과 REPORT/runtime_20261018_round02를 따릅니다. 로컬 시험은 실게임 검증에 합산하지 않습니다.

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

## 설치와 수집

기본 `armed=false`입니다. 활성화 후 전체 게임 재시작이 필요합니다. 설정은 시작할 때 한 번 읽습니다. 실험 전 세이브와 config를 백업하고 다른 AI 조정 모드를 꺼 원본 조건을 만듭니다.

타이틀에서도 배경 AI가 실행됩니다. 설치는 타이틀을 안전한 정지 지점으로 가정하지 않습니다. RX 원본 trampoline과 영구 breakpoint descriptor를 먼저 게시하고, 원자적인 INT3 진입 게이트를 세운 뒤 기존 스레드의 RIP를 검사합니다. tail 작성·읽기검사·명령 캐시 flush 뒤 첫 바이트를 게시합니다. 정확한 등록 entry의 breakpoint만 원본 trampoline으로 즉시 우회하며 대기하거나 게임 함수를 재호출하지 않습니다.

전체 SHA/본문이 다르면 설치를 거절합니다. 설치 실패 후 같은 실행에서 재시도하지 않습니다. `INSTALL_ERROR_ORIGINAL_RETAINED_ENTRY_UNVERIFIED`는 성공이 아니며 비교를 차단합니다. tail 작성 뒤 오류는 원문 rollback을 주장하지 않고 INT3 원본 우회를 유지합니다. 외부 VEH/debugger, prefix 중간 진입, private LTO 잔류 레지스터, EH/unwind의 모든 경우를 입증한 범용 detour는 아닙니다.

수집은 매 실행 꺼진 상태입니다. 목표 경기 진입 직전 다음 명령으로 해당 run_id를 확인하고 시작합니다. 실행당 수집 세션 하나만 허용하며 타이틀(scene1)은 제외하고 InGame4/GameTest5에서만 비교합니다. 이 scene은 상위 UI 상태여서 내부 리플레이 단계 자체를 증명하지는 않습니다. CLI는 UI를 조작하지 않고 관측 제어 파일만 씁니다.

```powershell
python -B -X utf8 tools/capture_control.py start --summary <모드폴더>/summary.json --label <경기구간명>
python -B -X utf8 tools/capture_control.py stop --summary <모드폴더>/summary.json
```

처음64개 적격 호출은 모두 채집하고 이후4096호출 간격, 함수당5000개 상한입니다. 파일 작업자는 UI 프레임과 별개로 owned event만 기록합니다. stop은 새 예약 차단·진행 중 비교0·enqueued=persisted를 확인해야 종료로 인정합니다. queue_full/queue_disconnected/io_errors/io_lost_or_uncertain_events는 각각 남기며 유실0을 가정하지 않습니다.

## 판독

`summary.json`과 같은 run_id의 `observations_<run_id>.jsonl`을 함께 보존합니다. MATCH는 관측 입력에 대한 예상 출력과 원본 출력의 비트 일치입니다. AL 반환은 AL만 비교하고 전체 RAX는 원본 그대로 반환합니다. 입력 변동은 UNSTABLE, 읽기/모델 실패는 SKIP, 외부 체인 결과는 FOREIGN으로 분리합니다.

함수 출력 일치와 각 임계점 아래/동일/위 관측을 별도 집계합니다. metric은 snapshot에서 계산하므로 `observed_branch=false`입니다. 내부 분기 레지스터를 직접 읽었다는 뜻이 아닙니다. 드문 전술/상황 및 데스매치 미관측은 남겨 두며, 표본0을 검증 완료로 처리하지 않습니다. 모든 계수의 독립 조절 가능성이나 전이 하위함수까지 검증됐다고 주장하지 않습니다.

첫 시험은 fdb9a0 집계1000일치/차이0, 저장512건 재계산 일치였으나 타이틀에서 상한을 소진했고488건이 유실됐습니다. 이는 경기 표본 검증이 아니며 `INCOMPLETE_WITH_LOG_LOSS`로 보존합니다. v0.2.0의 실제 시험 결과는 별도 round02 영수증을 따릅니다.

## 로컬 빌드

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File build_local.ps1 -RunTests
python -B -X utf8 tools/stage_round02.py
```

고정 nightly-2026-05-24/stable SDK, opt1/overflow off로 빌드합니다. 로컬 명령은 게임을 실행하지 않습니다. 배포는 기존 폴더 백업·설치본 SHA·독립 검수를 거칩니다.
