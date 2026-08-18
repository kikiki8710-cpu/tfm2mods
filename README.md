# tfm2mods — Teamfight Manager 2 네이티브 모드 모음

Teamfight Manager 2(팀파이트 매니저 2)를 Rust DLL로 확장하는 네이티브 모드 프로젝트입니다.
**현재 대응 게임 버전 = 0.5.5** · 완성 모드는 [Releases](https://github.com/kikiki8710-cpu/tfm2mods/releases/latest)에서 zip으로 받을 수 있습니다.

## 문서

| 자료 | 내용 |
|---|---|
| 📚 [tfm2mods_knowledge_public](https://github.com/kikiki8710-cpu/tfm2mods_knowledge_public) | 리버스엔지니어링 정본·모드별 작업 문서(mods_report)·작업 메모리 — 지식베이스 공개판 |
| 🎮 [노션 — 팀파매2 워크스페이스](https://app.notion.com/p/3a1d6f375d69803593d9db0f15844fff) | 정리판 문서 허브: 📖 정보정리(게임 내부 동작 01~02장) + 📚 모딩가이드(01~07장, 모드 사례연구·현행 모드 총람 포함) |

## 모드 목록 (0.5.5)

| 모드 | 설명 |
|---|---|
| `tfm2_ai_adjust` | 인게임 AI 판단 상수를 노브 577개로 노출·편집(전용 GUI 설정편집기 동봉). 기본값 = 원본과 비트 동일 |
| `tfm2_item_tactics` | 아이템 슬롯 3→4칸 확장 + 개인전술 지정 아이템 주입(빌드 AI의 자연 빌드업 방식) |
| `tfm2_banpick_illust` | 밴픽 화면 챔피언 스플래시 일러스트 + 셀렉트 연출 카드 + 버프/너프 시각화(이름 색·레이더) |
| `tfm2_banpick_order` | 밴/픽 턴 시퀀스 자유 재정의(인터리브·팀 순서) + 자체 밴픽 AI 보정 |
| `tfm2_elemental_serpen` | 원소/장로 드래곤 시스템을 세르펜에 이식 — 속성 배정·팀버프·처형·리플레이 호환 |
| `tfm2_comptest_unlock` | 조합 테스트 제약 해제(횟수·체력·중복) + RUN 1클릭 순차 N경기 실행 |
| `tfm2_draft_overlay` | 밴픽 화면 메타 분석 팝업(메타 대시보드 연동) |
| `tfm2_level_cap` | 경기 중 챔피언 최대 레벨 12 → 설정값(기본 18) 확장 |
| `tfm2_mod_order` | 모드 관리 팝업의 표시 순서를 키보드로 변경·영속화 |
| `Spectator_Chat` | 관전/다시보기 화면에 경기 상황 반응형 가짜 관중 채팅 |
| `community_reaction_mod` | 경기 결과·하이라이트 추출 + AI 커뮤니티 반응글 갤러리 |
| `tfm2_html_overlay` | 게임 창 위 상시 HTML 패널(WebView2) — stable ABI, 패치 무풍 |
| `tfm2_flow_capture` | 모든 경기 시뮬레이션을 ~1초 간격 샘플링해 파일로 기록(읽기 전용) |
| `tfm2_stat_exp` | 지정 선수의 판단력/오더 스탯 강제값 통제실험 도구 |
| `tfm2_bancard_keep` | 환경설정 "밴 카드 수" 리셋 방지 핫픽스 |
| `legacy_save_patcher` | 구 세이브에 모드 엔트리 삽입해 호환 경고 제거(stable ABI, 원저자 daram2) |
| `tfm2_meta_item_delegate` | 메타 대시보드용 아이템 데이터 수집 |
| `ui_kit` | 모드 공용 UI 모듈(드롭다운·드래그 팝업 등 — `#[path]` import용) |

## 설치

1. [Releases](https://github.com/kikiki8710-cpu/tfm2mods/releases/latest)에서 원하는 모드 zip을 받는다.
2. 압축을 풀어 `<게임 설치 폴더>\mods\` 아래에 모드 폴더째 넣는다.
3. 게임 타이틀 화면 → 모드 관리에서 활성화.

모드별 사용법(설정 파일 키·조작키)은 각 zip 동봉 README/cfg 주석과 지식베이스의 `mods_report/<모드ID>/01_구조.md` 참조.

## 소스 빌드

- 게임 동봉 mod SDK(버전 일치 필수) + Rust `nightly-2026-05-24` + **rust-lld 링커**
- 최적화는 `-C opt-level=1 -C overflow-checks=off` 고정(상위 최적화 = 스택오버플로 크래시)
- 일반 모드 = `build_inj.ps1` / 대형 모드(ai_adjust·banpick_illust·banpick_order) = rustc 직접 빌드 / stable ABI 모드 = `cargo build --release`
- 의존성 버전 대역 `>=0.5.5, <0.5.6` — 게임이 대역 밖 모드를 자동 비활성한다

## 저장소 구성

```
<모드ID>/            0.5.5 실사용 모드 소스 (src/*.rs + mod.mod_info)
ai_adjust_editor/    ai_adjust 전용 GUI 설정편집기 (cargo 프로젝트)
ui_kit/              공용 UI 모듈
MIGRATION.md         버전별 RVA·구조체 마이그레이션 정본 (§7.5 = 0.5.5 현행)
MOD_REGISTRY.md      모드 티어 표 (현행/실험/폐기 판정)
MIGRATE_NEXT.md      다음 패치 대응 런북
build_*.ps1, rel_*.py, pii_check_055.py, migrate_rva.py 등  빌드·릴리스·마이그 도구
hooks/, panicmap/, tools/  보조 도구
```

## 주의

- **RVA/오프셋 값은 게임 버전과 세트**입니다. 패치가 오면 값이 아니라 재유도 방법(`MIGRATE_NEXT.md`)을 신뢰하세요.
- 문서·소스 속 `C:\Users\dev\...` 류 경로는 익명화된 플레이스홀더입니다.
- 모드는 전부 로컬 싱글플레이 게임의 개인적 확장 용도로 제작됐습니다.
