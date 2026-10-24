# Codex 인수인계 — 상세전술 모드 "적용 장치" (2026-10-22, 게임 0.6.5)

> 2026-10-23 Codex 갱신: 아래 내용은 인수 당시 원문입니다. `micro.rs`는 재작성했고 `lib.rs`·`ui.rs`·`roster.rs`를 추가해 모드 전체를 연결했습니다. 현재 상태/검증은 `README.md`와 `mods_report/tfm2_detail_tactics/02_구현정보.md`를 보세요. 기존 ‘미완성/아직 없음’ 설명은 과거 상태입니다.

## 한 줄 요약
게임 AI 판단 함수 안의 문턱 값 32종을 **내 팀 선수가 판단할 때만** 설정값으로 바꿔 주는 부분을 만들어 주세요.
나머지(전술 표·설정 파일·세이브 저장·UI)는 이쪽에서 만들고 있고, 연결점은 `src/tactics.rs` 의 함수 하나입니다.

## ★2026-10-23 갱신 (Claude) — 먼저 읽을 것
- **선수 ID 오프셋 확정**: Player `+0x810` = athlete_id(**u64**). 근거 = `GamePlayer` Debug 구현 `0x1b14b90` 필드명 표(athlete_id `+0x810` · id `+0x818` · team `+0x820` · position `+0x8b0`). ⚠`+0x818` 은 경기 내 유닛 id(선수 ID 아님). 런타임 값 대조는 미실시 — 아래 진단 표본으로 확인.
  원문 = `C:\Users\jungs\Desktop\claude\tfm2\.claude\worktrees\swap-order-button-style-fe9e04\mods_report\tfm2_detail_tactics\RE\2026-10-23_PlayerState_athlete_id_오프셋.md`
- **지점 표 생성 완료**: `tools/gen_sites.py` → `src/sites.rs`(자동 생성 · 손으로 고치지 말 것).
  - 입력 = `tools/targets/knobs_0.6.5.json`(아래 자료표의 knobs32_065.json 사본) + `tools/targets/subjects_0.6.5.json`(지점별 판단 주체 위치 · RE 전반16/후반16 을 옮긴 것).
  - 검증 = exe sha · 지점 명령 asm · 즉치 = 기본값 변환 · 창(≥5B) 안 RIP 상대/call/ret 없음 · 창 안쪽으로 오는 분기 없음(함수 직접 분기 + 점프 표 + `.text` 전체 rel32 패턴) · 창 겹침 없음. 실행 로그 = `tools/gen_sites_0.6.5.log.txt`.
  - 결과 = **지점 42곳 · 지원 전술 26개**. 재실행: `python -I tools/gen_sites.py --exe C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.6.5\TeamfightManager2.exe`
  - `sites.rs` 는 `crate::micro::{Branch, Field, Patch, Src, Tf}` 타입을 가정한다(필드: Patch{name,tactic,rva,win,fields,branches,src,team_chk} · Field{off,width,tf,max} · Tf::{Add(i64),Sq1,Map{on,off}} · Src::{None,Reg(u8),Stack(i32),Mem(u8,i32),StackMem(i32,i32)} · Branch{off,len,cc:Option<u8>,target_rva}). 타입·출력 형식이 안 맞으면 **생성기를 고쳐서** 다시 생성할 것.
  - `team_chk` = 같은 시점 팀 값 위치(Player 팀과 다르면 원본으로 두는 교차 확인용).
  - `Tf::Map` = 켬·끔 전술을 값으로 구현하는 지점(서폿 라인 미니언 처리 금지: 켬 4 / 끔 127).
- **이번 버전 미지원 6개**(지점에서 판단 주체를 못 잡음 → 원본 유지, `sites::UNSUPPORTED`): 갱 중 적 포탑 교전 시 포기 · 정글링·로밍 중단 체력 · 라인 배정 체력 · 원딜·서폿 짝 동행 확인 · 위험 구역 비켜 걷기 여유 · 기지 방어 출동 기준 적 수. UI 는 이 6개를 「이번 버전 미지원」으로 표시할 예정.
- **파일 상태**: `src/micro.rs` = 미완성 초안(중간에 끊김 — 참고용, 갈아엎어도 됨) · `src/values.rs` = 중간에 끊김(설정 파일 모듈 — 이쪽에서 마무리) · `src/lib.rs` = 아직 없음(이쪽에서 작성, 설치 호출 자리 비움).

## 맡을 범위
- 새 모듈(예: `src/apply.rs`, 이름 자유) — 지점 설치·판단 주체 확인·값 적용.
- `src/micro.rs` 는 이전에 쓰다 멈춘 미완성 초안입니다. 참고만 하거나 지우고 새로 써도 됩니다.
- `lib.rs` 에서 설치 함수를 한 번 부르는 자리는 이쪽에서 비워 둡니다(`// TODO(codex): apply::install()`).

## 연결점 (이것만 쓰면 됨)
```rust
// src/tactics.rs
tactics::TACTICS[t].knob        // 지점 표의 노브 id (knobs32_065.json 의 id 와 같음)
tactics::value_for(t, athlete_id, pos) -> Option<i64>
    // t = TACTICS 순서 번호, athlete_id = 판단 중인 선수의 선수 ID, pos = 포지션 0탑..4서폿
    // None 이면 원본 그대로 두면 됨(내 팀 아님 / 단계 '기본' / 비무장)
    // Some(v) 면 그 지점 문턱을 v 로(지점 표의 변환 규칙 tf 를 적용: ('v',d) → v+d, ('sq1',) → v*v+1,
    //   켬·끔 전술은 v=0 이 '끔' — 원래 조건 분기를 무조건 분기로)
tactics::armed() -> bool        // 꺼져 있으면 아무것도 바꾸지 말 것
```
- 값은 시뮬 스레드에서 매번 읽어도 됩니다(원자 변수, 락 없음). 유저가 모달에서 단계를 바꾸면 다음 판단부터 바로 반영돼야 합니다.
- 팀 단위 전술(`Scope::Team`)도 같은 함수로 얻습니다(포지션 무관 팀 칸 값이 나옴).

## 자료 (먼저 읽을 것)
| 무엇 | 경로 |
|---|---|
| 32개 노브 · 53개 지점(0.6.5 RVA·원본 바이트·즉치 위치·변환 규칙) | `C:\Users\jungs\Desktop\claude\tfm2\.claude\worktrees\ingame-verification-mode-dd9a33\mods_report\tfm2_detail_tactics\_work\impl_v1\knobs32_065.json` |
| 지점마다 "판단 중인 선수"가 어디 있는지(앞 16개) | 같은 폴더 위 `RE\2026-10-22_판단주체위치_전반16.md` |
| 같은 내용(뒤 16개) | `RE\2026-10-22_판단주체위치_후반16.md` |
| 재사용할 선례 코드 위치·주의점 | `_work\impl_v1\2026-10-22_prior_work_선례조회.md` |
| 지점 원본 정의(전체 56지점·검증 도구) | `C:\tfm2mods\tfm2_tactics_verify\tools\targets\probe_targets_0.6.5.json`, `tools\gen_sites.py` |
| 효과 검증 결과(어떤 값이 실제 경기를 바꾸는지) | `mods_report\tfm2_tactics_verify\runtime_20261022_replayAB\README.md` |

RE 요지:
- Player 구조: 팀 `+0x820`(0/1), 포지션 `+0x8b0`(0..4). 선수 ID 는 `+0x810` **추정**(이전 버전마다 팀 −0x10) — 실제 값으로 확인 필요.
- 판단 함수는 **양 팀 10명 모두**에 대해 불린다 → 내 팀 확인 없이 바꾸면 상대 팀도 바뀐다.
- 일부 지점은 판단 선수 포인터가 그 자리에서 사라져 있다(RE 표에 대안 위치 기재: 호출자 프레임·함수 진입 시점 등).
- 함정: 정글링 중단 체력은 라이너 로밍에서도 불림 · 전투 중 후퇴 체력 사본 지점은 카테고리 분기 전 · 타워 추격 결과는 틱 단위 캐시 · 기지 방어 피해율 49 뒤에 사본(29/17/10)이 더 있음.

## 지켜야 할 프로젝트 규칙 (`C:\Users\jungs\Desktop\claude\tfm2\CLAUDE.md` §3)
- 원본 바이트가 표와 한 바이트라도 다르면 그 지점은 설치하지 않는다(주 1회 패치 대응). 한 전술의 지점 일부만 설치되면 그 전술 전체를 원본으로 둔다.
- 게임 쪽에서 불리는 코드는 패닉이 게임으로 새지 않게 막고(`catch_unwind`), 포인터는 안전 읽기만(`src/platform.rs` 의 `safe_read_*` / `fast_read_*`).
- 설치는 1회 확정. 매 프레임 재설치 금지. 같은 지점을 다른 모드(`tfm2_tactics_verify` 무장 시)가 잡고 있으면 설치하지 않고 로그만.
- 측정 모드(`tfm2_ai_analysis_verify`)와 동시 사용 금지.

## 빌드·확인
- 빌드: `powershell -ExecutionPolicy Bypass -File C:\tfm2mods\tfm2_detail_tactics\build_local.ps1 -RunTests` (nightly-2026-05-24, stable SDK `sdk_064_stable`).
- 확인 방법(이쪽에서 진행 가능): 내 팀(T1) 경기 다시보기를 단계 값만 바꿔 재생 → 다시보기는 결정론이라 결과가 달라지면 적용된 것. 상대 팀 선수 판단은 그대로여야 함.
- 진단용으로 지점별 "호출 수 / 내 팀 수 / 값 적용 수" 카운터와, 처음 몇 번의 (선수 ID, 팀, 포지션) 표본을 로그로 남겨 주면 선수 ID 오프셋 확정에 씁니다.

## ⚠ 인게임 자동 조작 시 주의 — 중간에 PowerShell 창이 뜰 수 있음 (유저 안내 2026-10-25)
- 이 PC에서는 **디스코드·카카오톡 봇**이 돌고 있어 **중간중간 PowerShell 창이 잠깐 앞에 뜰 수 있다.**
- 게임을 클릭·키 입력으로 조작하는 도중 이 창이 앞에 오면 클릭이 게임이 아니라 그 창으로 가서 절차가 꼬인다(2026-10-22 다시보기 A/B D13 회차: "알 수 없는 앱이 앞에 있음" → 불러오기 이상 → 그 회차 무효).
- 대응: 묶음 조작 전에 게임 창을 다시 앞으로(`open_application`) 가져오고, 단계마다 화면을 확인할 것. 창이 끼어든 회차는 결과를 쓰지 말고 다시 측정. 게임을 강제 종료하지 말 것(메뉴로 종료).
