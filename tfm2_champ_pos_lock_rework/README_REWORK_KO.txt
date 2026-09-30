TFM2 Champion Position Lock: Rework v1.0.0
============================================================
표시 이름 : TFM2 Champion Position Lock: Rework
모드 ID   : tfm2_champ_pos_lock_rework
폴더명    : tfm2_champ_pos_lock_rework
제작자    : tfm2mods(원작) · flover(Rework)
게임      : Teamfight Manager 2 v0.6.1 / Windows

[중요]
- 원작 Champion Position Lock과 별도 모드 ID를 사용합니다.
- 원작과 Rework를 동시에 활성화하지 마세요. 원작을 비활성/제거한 뒤 사용하세요.
- 새 모드 ID이므로 원작 세이브에 저장된 포지션 설정은 자동 이관되지 않습니다.
  Rework에서 포지션 목록을 한 번 다시 설정해 주세요.

[Rework 핵심]
1. 기본 필요 수는 고정값이 아닙니다.
   세트 수, 밴카드 수, 일반/피어리스/하드 피어리스 등 현재 규칙에 따라 자동 계산됩니다.
2. 모든 포지션을 표시된 필요 수까지 미리 채울 필요는 없습니다.
   필요 수보다 적어도 지정한 챔피언이 있으면 그 포지션 제한이 먼저 작동합니다.
3. 지정 챔피언을 우선 사용합니다.
   밴/픽/피어리스로 해당 포지션의 지정 풀이 실제 고갈될 때만 그 포지션에 자유 예비가 열립니다.
4. 다른 포지션 전용 챔피언은 자동으로 자유 챔피언이 되지 않습니다.
5. 실제로 챔피언이 겹치는 포지션끼리만 공동풀/subset 검사를 합니다.
   관계없는 부족 포지션 때문에 다른 포지션 필요 수가 같이 부풀지 않습니다.
6. 아무 포지션에도 넣지 않은 챔피언 = 자유 예비입니다.
7. 유저 직접 픽 / AI 픽 / 수석코치 위임 / AI 포지션 배치 / 자동 스왑에 같은 규칙을 적용합니다.

[UI]
- 선택 N · 기본 필요 M : 해당 포지션 단독 상태
- 선택 N · 공동 H/W : 겹치는 포지션 공동풀 상태
- 직접 겹침(포지션별, 중복 포함) : 다른 포지션과 함께 넣은 수
- 공동풀 검사 ... 통과/부족 : 현재 가장 빡센 공동 조건

[빌드]
1. BUILD_AND_PACKAGE.bat 실행
2. 성공 시 dist\tfm2_champ_pos_lock_rework_v1.0.0.zip 생성
3. ZIP 안 tfm2_champ_pos_lock_rework 폴더를 게임 mods 폴더에 설치

[소스 유지보수]
- 실제 Rework 동작 소스: tfm2_champ_pos_lock_rework\src
- 실제 배포 UI 문구: tfm2_champ_pos_lock_rework\assets\text\poslock.i18n
- _classic_058 폴더는 과거 0.5.8 계열 참고/보관용이며 현재 Rework 빌드에는 사용하지 않습니다.
- 향후 업그레이드 시 FINAL 배포 ZIP을 역수정하지 말고 이 BUILD_READY 소스를 기준으로 수정/빌드하세요.
