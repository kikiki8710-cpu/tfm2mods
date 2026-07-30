# tfm2_banpick_illust — 밴픽 일러스트

밴픽 화면에서 **픽이 확정된 슬롯 배경에 챔피언 스플래시 일러스트**를 깔아준다.
창작마당에서 내려간 daram2 "Pick Ban Plus"(`banpick_view_plus`, 워크샵 3751386306)의
일러스트 기능만 떼어내 게임 0.5.1 UI 에 맞춰 단독 모드로 재구현한 것.

원본은 dll 만 남고 소스·`.ui`·`mod_info` 가 유실됐으므로, dll 문자열 분석으로 **에셋 키 규약**만
복원하고 표시 로직은 새로 짰다.

## 설계

- **순수 SDK** — 하드코딩 RVA·raw 메모리 오프셋 0개. 게임 패치 시 `build.ps1` 의 `$SDK` 만
  새 SDK 로 바꾸고 재빌드하면 된다(RVA 마이그레이션 불필요).
- **에셋 키 = `asset/base/ui/banpick/illust/<champion_id>`**
  원본 모드가 정착시킨 생태계 규약이라 **바꾸지 말 것**. 다른 아트 모드(예: Touhou Project)가
  자기 챔피언에 대해 같은 키를 `mod.override_info` 로 override 해두면 그 아트가 자동으로 잡힌다.
  바닐라 65종은 이 모드의 `mod.override_info` 가 `asset/tfm2_banpick_illust/illust/<id>` 로 매핑.
- **표시 경로**
  1. `.ui` 오버라이드가 `banpick/blue_pick_slot`·`red_pick_slot` 의 `#done` **최상단**에
     `#bpi_illust`(image) + `#bpi_dim`(color) 을 심는다. 최상단 선언 = 가장 뒤에 그려짐 = 배경.
  2. dll 이 매 프레임 `post_update` 에서 `#blue_picks`/`#red_picks` 를 재귀 탐색해
     `#bpi_illust` 를 자식으로 가진 노드(=`#done`)를 찾고,
     `#done > #champion > #icon` 의 image source 에서 챔피언 id 를 파싱해
     `#bpi_illust` 의 source 를 위 키로 설정 + `visible`, 레드 진영은 `flip_x`.

## 파일

| 경로 | 내용 |
|---|---|
| `src/lib.rs` | 노드 탐색·일러스트 적용 |
| `src/config.rs` | `.cfg` 로드/자동생성 |
| `ui/layout/banpick/*.ui` | base 0.5.1 사본 + 일러스트 노드 삽입 |
| `mod.override_info` | 픽슬롯 레이아웃 2건 + 일러스트 키 65건 |
| `build.ps1` / `deploy.ps1` | 빌드 / 게임 폴더 배포 |

일러스트 PNG(65종 172MB)는 **소스트리에 두지 않고 배포 폴더에만** 둔다(중복 저장 방지).
`deploy.ps1` 이 워크샵 3751386306 의 `illust\` 에서 복사한다(이미 있으면 생략).
`<id>-1.png` 는 같은 그림의 저해상도 사본이라 제외.

## 설정

`<게임>\mods\tfm2_banpick_illust\tfm2_banpick_illust.cfg` (없으면 자동 생성, 수정 후 게임 재시작)

| 키 | 기본 | 설명 |
|---|---|---|
| `enabled` | 1 | 기능 켬/끔 |
| `red_flip` | 1 | 레드 진영 좌우반전(인물이 안쪽을 보게) |
| `dim_alpha` | 160 | 일러스트 위 어둡기 0~255(이름 가독성). 0=안 덮음 |
| `hide_portrait` | 1 | 바닐라 소형 초상화·구분선 숨김 |
| `debug` | 0 | 챔프 아이콘 실제 source 를 `illust_debug.txt` 로 1회 덤프 |

## 알려진 미검증 지점

- **챔피언 아이콘 source 문자열의 실제 형태** — 게임이 런타임에 넣는 값이라 정적분석으로
  확정 못 했다. `asset/base/aseprite_resources/champions/<id>#sheet` 형태로 추정하고,
  "경로 마지막 세그먼트 우선, 그게 시트/폴더성 일반명이면 `#` fragment 폴백"으로 방어했다.
  일러스트가 안 나오면 `debug = 1` 로 두고 밴픽을 한 번 본 뒤 `illust_debug.txt` 의
  실관측값에 맞춰 `champ_id_from_source()` 를 고치면 된다.

## 주의

- `.ui` 파일에 `//` 주석 금지 — base 번들 전체에 용례가 0건이라 파서 지원이 확인되지 않았다.
- `mod.mod_info`·`mod.override_info` 는 **BOM 없는 UTF-8**(첫 바이트 `0x7b`). BOM 이면 파서
  실패로 모드가 강제 비활성된다. 반대로 한글이 든 `.ps1` 은 PowerShell 5.1 이 ANSI 로 읽어
  깨지므로 **UTF-8 BOM 필수**.
- 게임 모드 메뉴에서 활성화해야 적용된다(코드 모드 경고 1회 수락 필요). 적용은 게임 전체 재시작.
