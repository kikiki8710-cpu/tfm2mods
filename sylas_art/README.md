# Sylas 도트 일러스트 — 외부 AI 학습/생성 키트

TFM2 바닐라 챔피언 63종 스프라이트로 **화풍(스타일) 학습 → 사일러스 도트 세트 생성**을 위한 데이터셋·가이드.

## 폴더 구조
```
sylas_art/
├── dataset/
│   ├── images/              # 학습용: 3167장 캐릭터 프레임 PNG + 동명 .txt 캡션
│   ├── idle_only/           # 챔피언별 idle 1장(66장) — PixelLab 단일 레퍼런스/스타일 감잡기용
│   └── _idle_contact_sheet.png   # 데이터셋 눈검수용 모음
├── silas_prompts.txt        # 사일러스 생성 프롬프트(포즈별)
└── README.md
```
- 모든 프레임 = 원본 도트를 **8배 nearest 확대**(투명 배경 유지). 캐릭터 몸체 포즈만(effect/투사체 프레임 제외).
- 캡션 trigger word = **`tfm2_sprite`** (이 단어가 곧 "이 게임 도트 화풍").
- 포즈 태그 분포: idle 276 / run 474 / attack 355 / skill 451 / skill2 454 / ult 505 / hit 68 / dead 584.

---

## 경로 A) 스타일 LoRA 학습 (SDXL + kohya_ss)
1. **베이스 모델**: SDXL 1.0 (또는 Pony/픽셀아트 체크포인트).
2. **데이터**: `dataset/images/` 통째로 투입(이미지+캡션 자동 페어링).
3. **설정 권장값**:
   - resolution 1024 (또는 768), batch 2~4, network_dim 16~32, network_alpha 16
   - learning_rate 1e-4 (UNet), text_encoder_lr 5e-5
   - epochs 10~20, repeats 2~4, scheduler cosine, optimizer AdamW8bit
   - trigger word `tfm2_sprite` 는 캡션에 이미 박혀있음
4. **산출**: `tfm2_sprite.safetensors` (LoRA 파일)
5. **생성**: 그 LoRA 로드 + `silas_prompts.txt` 프롬프트로 사일러스 생성.

## 경로 B) PixelLab (학습 불필요, 애니메이션 일관성 최강 — 권장)
1. pixellab.ai 접속.
2. `dataset/idle_only/` 의 도트들로 스타일 감을 잡고(또는 레퍼런스로 업로드), **사일러스 idle 1장**을 먼저 생성(프롬프트=silas_prompts.txt의 idle).
3. 그 idle을 **캐릭터로 등록** → run/attack/skill/ult/hit/dead **스켈레톤 애니메이션** 자동 생성(프레임 일관성 유지).
4. 각 포즈 프레임을 PNG로 export.

## 경로 C) ControlNet 포즈 참조 (대안)
- `dataset/images/` 의 같은 태그 프레임(예: `whip_master_attack_*`)을 ControlNet(scribble/lineart) 참조로 두고 사일러스로 img2img → 포즈 일관성 확보.

---

## 다음 단계 (생성 후 → 나에게)
포즈별 프레임 PNG들을 주면 내가:
1. 팔레트 양자화 + 투명 배경 복원 + 프레임 크롭/정렬
2. 가로 일렬 `sylas#sheet.png` 조립 + `sylas#anim.fanim` 좌표 작성
3. 스킬 아이콘 3종 + (원하면) 밴픽 초상화 통합
4. 모드 폴더 반영 → 게임 배포 → 인게임 검증

## 목표 애니메이션 스펙 (게임 표준)
| 태그 | 목표 프레임 | 프레임 크기 |
|---|---|---|
| idle | 4 | ~28×46 |
| run | 7 | ~28×46 |
| attack | 5 | ~30×46 |
| skill / skill2 / ult | 5 | ~30×48 |
| hit | 1 | ~30×46 |
| dead | 8~10 | ~30×46 |

(프레임 크기는 가변 OK — 최종 조립은 내가 fanim 좌표로 맞춤. 투명 배경 + 캐릭터 중앙 정렬만 지켜주면 됨.)
