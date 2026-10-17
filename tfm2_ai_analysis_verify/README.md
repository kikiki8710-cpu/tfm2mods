# AI 분석 검증 모드 준비본

실제 게임의 함수 호출을 한 번 통과시키고, 입력에서 독립 계산한 예상값과 실제 반환값을 기록합니다. 판단 결과나 계수를 바꾸지 않습니다. **실게임 검증 결과는 아직 0건입니다.**

준비 범위는 741개 추적 카탈로그, 독립 계산 모델 8개, 진입 연결 후보 5개입니다. 모델이 없는 733개와 진입 연결이 없는 모델3개는 런타임 검증 완료가 아닙니다. 전체 741개가 즉시 비교되는 모드라고 해석하면 안 됩니다.

|연결 후보|비교할 내용|
|---|---|
|`fdb9a0`|최대12개 예측 레코드의 key 첫 일치 포인터/없음|
|`e49c00`|존재하는 아군이 상대 기지 600000 반경 안에 모두 있는지|
|`fa9dc0`|다른 역할·같은 라인·150000 엄격 거리 조건|
|`f51e50`|두 기록 시각의 경과·미래시각·우선 조건|
|`e9eb70`|참여 레코드와 HP25% 조건|

`dd87a0/e52ee0/d98bd0` 모델도 구현했지만 초기 명령에 분기가 있어 relocation 없는 진입 훅에서 제외했습니다. 지원ABI는 정수 레지스터 인수 최대4개와 AL 또는 전체 RAX 출력입니다. sret·RAX/RDX쌍·XMM·스택인수를 지원하는 범용 hook은 아닙니다.

## 현재 준비 상태

로컬 시험은 게임 실행 없이 수행합니다. 계산식 경계·wrap·정수폭, 원본 호출1회, 입력읽기 실패, 입력변동, AL만 비교하면서 전체 원반환 유지, 설치미완료 비교차단, 큐유실, 초기/관측파일 I/O 실패를 다룹니다. source/test 영수증과 실제 DLL은 최종 준비 결과의 SHA를 따릅니다. `tools/validation_receipt.json`은 오프라인 집계기 시험이며 실제 게임 사례가 아닙니다.

## 로컬 빌드

```powershell
powershell -File .\build_local.ps1 -RunTests
```

`target/release/tfm2_ai_analysis_verify.dll`을 만들며 게임 폴더에 복사하거나 게임을 실행하지 않습니다. stable SDK의 경계만 사용하고 game_ai rlib나 원본 게임 헬퍼는 예상값 계산에 호출하지 않습니다.

## 이후 첫 인게임 시험

1. 준비된 `tfm2_ai_analysis_verify` 폴더를 게임의 `mods` 아래에 놓고 게임 모드 관리에서 활성화합니다. 다른 AI 조정 모드가 꺼진 원본 조건으로 시작합니다. 이는 사용자가 별도로 실행할 단계입니다.
2. 기본 `verify.cfg`는 `armed=false`입니다. 처음 로드에서는 `summary.json`의 `identity_verified=true`, `armed=false`, `NO_SAMPLES`를 확인합니다. 표본 없음이 정상이며 성공 판정은 아닙니다.
3. 첫 관측은 `probes=fdb9a0` 한 종류로 둡니다. 경기와 배경 시뮬레이션이 없는 초기화 상태에서 다음 실행을 준비하고 `armed=true`로 바꿉니다. 설정은 시작할 때 한 번 읽으므로 다시 시작해야 반영됩니다.
4. 관측 준비 후 평소처럼 경기/리플레이를 실행합니다. `summary.json`의 `install_status`와 `calls/compared/mismatches`를 확인합니다. 관측 후 잠시 기다려 파일 갱신을 확인합니다.
5. 한 함수의 설치·ABI·원반환 유지가 확인되면 주석의 다섯 후보 목록으로 범위를 늘립니다. 모드 폴더의 `summary.json`과 같은 `run_id`의 `observations_<run_id>.jsonl`을 함께 분석합니다.

SHA 또는 전체 함수 원문이 다르면 설치하지 않습니다. 설정을 켠 뒤 설치에 실패하면 매 프레임 재시도하지 않습니다. 게임을 다시 시작하기 전에는 자동 재후킹하지 않습니다.

## 결과 읽기

```powershell
python -B -X utf8 tools/analyze_logs.py --summary <모드폴더>/summary.json --observations <모드폴더>/observations_<run_id>.jsonl --catalog catalog.json --output-dir <분석결과폴더>
```

- `MATCH/MISMATCH`: 안정성을 확인한 관측 필드에서 예상 출력과 실제 출력의 일치/차이입니다.
- `NO_SAMPLES`: 원본 비교 표본이 없습니다. 함수 호출0과 관측조건 미충족을 확인합니다.
- `SKIP/UNSTABLE`: 읽기 실패·상한초과·모델오류 또는 원본 실행 전후 입력변동이며 성공 표본에 넣지 않습니다.
- `FOREIGN_MATCH/FOREIGN_MISMATCH`: `allow_foreign_chain=true`에서 다른 모드가 먼저 설치한 체인을 관측한 것입니다. 원본 게임 검증 집계에 합치지 않습니다.
- `dropped/io_errors/io_lost_or_uncertain_events/unlogged_errors`: 기록이 불완전합니다. 요약계수와 저장된 원문 표본은 별개이며 부족분을 전부 유일 게임 사례나 전부 IO손실로 단정하지 않습니다.

임계점의 아래/동일/위가 실제 관측에 모두 왔는지 별도 집계합니다. 현재 metric은 복사 입력에서 계산한 값이고 `observed_branch=false`입니다. 게임 내부 분기 레지스터를 직접 계측한 값으로 오인하면 안 됩니다. 모든 저장 표본의 일치도 전체 분기·모든 입력·실 provider 완료를 뜻하지 않습니다.

## native 설치의 미검증 범위

이 준비본의 native 훅은 실험용입니다. 경기/배경 시뮬레이션이 없다는 조건은 사용자의 수동 전제이며 자동 판별하지 않습니다. 기존 스레드 정지·RIP/원문 재검사로 위험을 줄이지만 신규 스레드 생성 경쟁, private LTO caller의 미추적 레지스터 잔류, 간접 caller·EH·반환주소의 원형까지 검증한 것은 아닙니다. 직접 caller10개의 제한된 정적 감사에서 알려진 반례는 없었고 실게임 실측은 별도입니다.

`REFUSED_BEFORE_WRITE`와 `INSTALL_ERROR_ORIGINAL_RETAINED_ENTRY_UNVERIFIED`를 구별합니다. 후자는 코드쓰기를 시도한 뒤의 실패이며, 특히 부분쓰기 실패가 원문으로 안전하게 rollback되었다는 보장이 없습니다. 비교를 중단하고 trampoline을 유지합니다. 이 상태를 원문불변·검증성공으로 취급하지 않습니다.
