# tfm2_tactics_verify — 상세전술 문턱 인게임 확인 모드 (설계)

목적: 상세전술 후보의 문턱(HP%, 인원, 거리, 점수)이 **실제 경기에서 어떻게 갈리는지(②)** 와
**값을 바꾸면 그 판단이 얼마나 바뀌는지(③)** 를 리플레이로 확인한다. 코덱스 검수 모드(①: 함수 출력 일치)는 반복하지 않는다.

## 1. 동작
1. 게임 시작 → `tactics_verify.cfg` 를 한 번 읽는다. exe SHA 가 `src/sites.rs` BASELINE(현재 0.6.5 `daf040db…`, targets 파일에서 생성)이 아니면 아무것도 설치하지 않는다.
2. 전체 `post_update` 프레임이 `install_delay_frames` 이상이면 현재 화면 종류와 무관하게 한 번만(2026-10-21 진입부 작업 지시 반영):
   - **패치**: cfg 의 `set=` 값대로 문턱 즉치 바이트를 바꾼다(원래 바이트를 대조한 뒤, 다른 스레드를 멈추고 쓰기).
   - **프로브**: 비교 명령(cmp/test) 35곳의 첫 바이트를 INT3(0xCC)로 바꾼다. 예외 핸들러가 그 순간 비교되는
     레지스터/스택 값을 기록하고, 복사해 둔 원래 명령(스텁)으로 실행을 돌려보낸다. 게임 판단은 바꾸지 않는다(패치 제외).
3. 기록 스레드가 2초마다 `tactics_verify_summary.json` 을 덮어쓴다(사이트별 호출 수 + 값 표본).
4. 분석은 경기 뒤 `tools/analyze.py` 가 한다. 원래 문턱(또는 패치 값) 기준으로 "통과/불통과" 비율, 값 분포,
   문턱 근처 표본 수를 계산해 **작은 마크다운 표 하나**를 낸다. A/B 는 요약 두 개를 비교한다(절대 횟수가 아니라 비율 — 훅 호출 수는 같은 조건에서도 ±2~4% 흔들림, DONE 교훈).

## 2. 관전 경기 구분
- 핸들러가 현재 스레드 ID 를 메인 스레드(post_update 에서 갱신)와 비교한다. 메인 = 관전 경기(fg), 그 외 = 배경 경기(bg). (lane_tactics `fg()` 방식)
- scene 이 4/5 가 아닐 때의 호출은 `offscene` 으로만 센다.
- 값 표본은 fg 만 모은다(사이트당 최대 `platform::SAMPLE_CAP=1024`개, 저수지 표본). 횟수는 모두 센다.

## 3. 안전
- 코덱스 `platform.rs` 를 그대로 쓰고, 별도 `SITE_GATES` 표만 추가했다(기존 게이트 코드는 손대지 않음).
- 핸들러 안: 할당·락·게임 CALL 없음. 값 읽기는 CONTEXT 레지스터, 또는 rsp/rbp 기준 스택 주소만(현재 스레드 스택이라 유효). 힙 피연산자는 읽지 않고 횟수만.
- 모든 사이트는 설치 전 원래 바이트를 대조한다. 하나라도 다르면 그 사이트만 거절하고 summary 에 남긴다.
- 사이트당 호출이 `max_hits` 를 넘으면 기록 스레드가 INT3 를 원래 바이트로 되돌려(무장 해제) 성능 저하를 막는다.
- `armed=false` 가 기본. DLL 이 있어도 cfg 를 켜기 전에는 아무것도 하지 않는다.
- 다른 모드와 같은 명령을 건드리지 않는다. 시험 중에는 lane_tactics·ai_analysis_verify 를 끈다.

## 4. 파일
- `tools/gen_sites.py` : `probe_targets.json` + exe 바이트 + capstone → `src/sites.rs`(사이트·패치 표), `tools/sites.json`(분석용 판정 규칙)
- `src/platform.rs` : 코덱스 사본 + SITE_GATES(값 기록 게이트)
- `src/probe.rs` : 게이트 설치·해제, `src/patch.rs` : 즉치 패치, `src/config.rs`, `src/lib.rs`, `src/summary.rs`
- `tools/analyze.py` : 요약 → 표(단일/AB), `tools/session.py` : 게임 꺼진 상태에서 백업·활성화·원복

## 5. cfg
```
armed=true
label=base            # 요약 파일에 기록되는 이름
install_delay_frames=120
max_hits=3000000      # 사이트당 이 이상이면 무장 해제
set=gank_abort_hp_41=55   # 문턱 바꾸기(논리값). 여러 개 가능
```

## 6. 한계 (1차)
- lea 배율(12×TPS·5×TPS·피해×3)과 `inc`(인원 여유 +1) 는 SIB/명령 재작성이 필요해 1차 패치 대상에서 뺐다(관측은 비교 명령에서 한다).
- 힙 메모리 비교(갱 포기 이유 9 게이트 등)는 횟수만.
- 경계 상황이 리플레이에 안 나오면 "미관측"으로 남는다.
- 요약의 원자 카운터/표본은 Relaxed로 각각 읽는다. 한 시점의 트랜잭션 스냅샷이 아니므로 수집 중에는 합계나 표본 쌍이 일시적으로 어긋날 수 있다.
- fg/bg는 메인 스레드와 현재 scene에 의한 분류이며, 실제 관전 경기 소유권을 독립적으로 입증한 표식은 아니다.
- 호출 예산 해제는 요약 주기마다 확인하므로 `max_hits`를 넘는 초과 호출이 생길 수 있다. 부분 패치·설치 거절은 로그/요약으로 판정하며 실게임 검증은 별도다.

## 7. 게임 패치 대응 (2026-10-22 — 이 게임은 거의 매주 패치된다)
지점 주소의 정본 = `tools/targets/probe_targets_<버전>.json` 하나. `src/sites.rs` 는 생성물이다.
1. 패치 감지: `python MIG\run.py ...`(전 모드) 또는 `python MIG\mig_verify.py check tfm2_tactics_verify --exe <신exe>` → STALE 이면 아래.
2. 지점 이전: `python -X utf8 tools/migrate_targets.py tools/targets/probe_targets_<구>.json <구exe> <신exe> <신버전> tools/targets/overrides_<구>_<신>.json [--oldpkl/--newpkl/--oldcg/--newcg]`
   - BYTES_UNIQUE + 바이트 동일만 자동 확정. 나머지는 **overrides 에 사람 판정**(옮김 / 사본 분기 리스트 / null=비활성+사유) — 판정 없으면 중단.
   - 옮긴 지점의 명령 꼴·즉치값이 구와 다르면 중단(문턱이 바뀐 것 = 기획 재검토).
   - 산출: `probe_targets_<신>.json` + `migrate_<구>_<신>.log.md`(지점별 근거).
3. 생성: `python -X utf8 tools/gen_sites.py tools/targets/probe_targets_<신>.json` → `src/sites.rs`(BASELINE SHA 포함) · 비활성 엔트리의 노브는 자동 제외.
4. `mod.mod_info` deps 대역 갱신 → `build_local.ps1 -RunTests` → `MIG\mig_verify.py rebase tfm2_tactics_verify`(채록 바이트 갱신) → `check` PASS.
- 노브(KNOBS)는 버전 무관 key(최초 0.6.4 RVA, 사본은 `#n`)로 지점을 가리킨다. 사본 key 는 그 버전에 있으면 쓴다.
- 0.6.4→0.6.5 실적: 56지점 중 자동 41 · 사람 판정 15(사본 분기 1·비활성 2 포함) · 0.6.4 재생성 = 기존 sites.rs 와 바이트 동일(회귀 확인).
