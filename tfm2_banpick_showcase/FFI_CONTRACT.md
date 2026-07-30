# tfm2_banpick_showcase — FFI 계약서 (0.5.2, 2026-07-25 ghidra-re 확정)

> 출처 = ghidra-re 디스어셈 직독(콜사이트 교차검증). 전부 **0.5.2 RVA** (base 0x140000000).
> 공통: Win64 callconv. "스택 argN" = 콜 시점 `[RSP+8*(N-1)]` (arg5=+0x20, arg6=+0x28, …).
> f32 인자 = XMM(1~4번째) 또는 스택 dword. cmd 버퍼 = 스택 로컬 0xd0B — **memset(0) 후 필요 필드만 채우면 됨**.

## 훅 대상 3지점

### A. 0x11e2370 — 연출 상태 세팅 (진영 스태시 detour)
| 인자 | 위치 | 타입 | 의미 |
|---|---|---|---|
| 1 | RCX | ptr | self (match_ui 러너) |
| 2 | RDX | ptr | UI 트리 ctx |
| 3 | R8 | ptr | app/asset ctx |
| 4 | R9 | u64 | **팀 id** |
| 5 | 스택 arg5 | ptr | 챔피언 키 ptr |
| 6 | 스택 arg6 | usize | 챔피언 키 len |
| 7 | 스택 arg7 | u8 | **0=밴, ≠0=픽** |

- 반환 없음. detour 최소 읽기: `is_blue = (*(u64*)(self+0x3d0) == R9)`; `mode = (arg7==0) ? BAN : (is_blue ? BLUE_PICK : RED_PICK)`. 밴 카드의 진영도 이 비교식으로 판별됨.
- **self+0x350 채우는 사이트 = 바로 이 함수** (챔프키 clone: alloc 0x8b7f80(len,1)+memcpy, len==0이면 (0,1,0), 기존 String은 cap∉{-1,0}이면 free).
- self 기록: +0x348 활성, +0x360/+0x36c 타깃유효, +0x364=(cx,cy), +0x370=(w,h), +0x378=0(타이머), +0x37c=mode(0밴/1픽blue/2픽red).

### B. 0x11f9030 — 카드 드로우 헬퍼 (픽 대체 / 밴 패스스루)
| # | 위치 | 타입 | 의미 |
|---|---|---|---|
| 1 | RCX | ptr | 게임/리소스 ctx (에셋 store — 99c860에 그대로 패스스루) |
| 2 | RDX | ptr | draw-list |
| 3 | R8 | ptr | 챔피언 키 ptr (= self+0x350) |
| 4 | R9 | usize | 챔피언 키 len |
| 5 | 스택+0x28(arg5) | `*const [f32;4]` | 카드 rect (x,y,w,h) **로컬좌표** — 항상 {-180,-240,360,480} |
| 6 | 스택+0x30(arg6) | `*const [f32;4]` | RGBA 틴트: 밴={0.639,0.663,0.714,1} / 블루픽={0.357,0.451,1.0,1} / 레드픽={0.937,0.392,0.443,1} |
| 7 | 스택+0x38(arg7) | u8 | greyscale (밴 t≥0.07 → 1) |
| 8 | 스택+0x40(arg8) | f32 | t (등장 진행도 0~1 클램프) |

- **transform: 호출자가 op0x11 push → 헬퍼 → op0x12 pop. 헬퍼는 로컬좌표만 그리면 됨(비행 트윈·흔들림 전부 호출자 몫). [확정]**
- 내부 지도: 표시명 획득(0x11f9162) → 장식 0x4b8/0x4ba/0x4bb → 일러 조회(0x11f93c2 = fdabe0) → 일러 cmd 0x4bc → 플레이트 0x4bd → 이름 텍스트 0x4be.
- 내부 일러 rect = rect+{+28,+28,-56,-124} = 304×356. 플레이트 = (x+28, y+h−84, w−56, 56).

### C. 0xfdabe0 — 밴픽 전용 일러 에셋 조회 (에셋 리다이렉트 detour)
- 키 포맷 `asset/base/ui/banpick/illustrations/<champ>` 조회(99c860) → 있으면 cover-크롭 UV / 없으면 out.cap=-1 센티널(→호출자가 aseprite idle 폴백).
- out 구조체(0x28B): +0x00 cap(-1=없음) / +0x08 ptr / +0x10 len (키 String) / +0x18 u0 / +0x1c v0 / +0x20 uw / +0x24 vh.
- 정밀 시그니처(인자 레지스터)·호출자 소비 방식 = **후속 RE 대기 중** (아래 §미확정).
- out 키 String은 게임이 0x8b7f90(ptr,cap,1)로 free.

## draw cmd 공통

### 제출
- **0x248b1c0** `b1c0(list RCX, &cmd RDX)` — 일반 cmd 제출. cmd 내용(힙 String 포함) **move** → 호출 후 cmd 필드 free 금지, 스택 버퍼 재사용 가능.
- **0x248b400** `b400(list RCX, &cmd RDX)` — **텍스트 cmd 전용** 제출 레인(내부 레이아웃 처리). 텍스트는 반드시 b400.

### 라운드사각 cmd (op 0x0a) — 직접 구성 + b1c0
| 오프셋 | 타입 | 값 |
|---|---|---|
| +0x00 | u64 | `0x800000000000000A` |
| +0x08 | u32 | 0 |
| +0x0C | f32 | 코너 반경 |
| +0x1C | u32 | 1 (원본 고정값) |
| +0x20 | f32 | 스트로크 폭 |
| +0x24/28/2C | f32×3 | 스트로크 RGB |
| +0x30 | f32 | 스트로크 A |
| +0x34 | u32 | 0 |
| +0x58..0x64 | f32×4 | rect x,y,w,h (로컬) |
| +0x68 | u64 | layer(z) |
| +0x70/74/78 | f32×3 | 채움 RGB |
| +0x7C | f32 | 채움 A |
| 나머지 | — | 0 |

원본 장식 4건 (t=클램프 진행도, P=틴트 RGBA, A=P.a):
| layer | 반경 | rect | 스트로크(폭, RGB, A) | 채움(RGB, A) |
|---|---|---|---|---|
| 0x4b8 외곽 | 22 | 카드±10 인플레이트 | 8.0, P.rgb, clamp01(0.05t)·A | P.rgb, clamp01(0.18t)·A |
| 0x4ba 배경 | 18 | 카드 rect | 3.0, {0.0627,0.0706,0.1020}, min(0.98t,1) | {0.063,0.071,0.102}…※채움=P.rgb, min(0.95t,1)·A |
| 0x4bb 일러프레임 | 14 | 내부 rect | 1.0, {0.0275,0.0314,0.0431}, min(0.92t,1) | {0.2902,0.2980,0.3373}, min(0.55t,1) |
| 0x4bd 플레이트 | 12 | 플레이트 rect | 2.0, {0.0588,0.0627,0.0863}, min(0.96t,1) | P.rgb, min(0.72t,1)·A |

### 이미지 cmd 체인 (op 0x04) — 핑퐁 A/B 버퍼, 각 단계 in은 move(free 금지)
1. **0x248c130** `c130(&cmd RCX, key_ptr RDX, key_len R8, x XMM3, y arg5, layer arg6 u32, w arg7, h arg8, 0,0,0,0 arg9~12)` (arg5,7~12 = f32 dword). key는 내부 clone → 우리 키 수명 걱정 無. TLS 단조 카운터(+0x58) 자동 기입 — **반드시 c130 경유(직접 구성 금지)**.
2. **0x248c7c0** `c7c0(&B RCX, &A RDX, &uv R8)` — uv=[f32;4]{u0,v0,uw,vh}(비율).
3. **0x248cd40** `cd40(&A RCX, &B RDX, flag R8B)` — flag→+0xAC. 일러 에셋 경로=0 / aseprite 캐시=1. **우리 텍스처는 0**.
4. **0xff0c20** `ff0c20(&B RCX, &A RDX, "color" R8, 5 R9, &val arg5)` — val=[f32;4]{1,1,1,t} 페이드인.
5. (greyscale 시) **0x248e850** `e850(&A RCX, &B RDX, "asset/base/shader/greyscale" R8, 0x1b R9)` — op7 셰이더 래퍼로 변환.
6. `b1c0(list, &최종)`.
- 원본 배치: x=rect.x+28, y=rect.y+28, w=rect.w−56, h=rect.h−124, layer=0x4bc.

### 텍스트 cmd — 0x248c1e0 + b400
`c1e0(&cmd RCX, text_ptr RDX, text_len R8, font_key R9, font_len arg5=0x18, &rgba arg6, &rect arg7, layer arg8 u32=0x4be, size arg9 f32=30.0, arg10 u8=1, arg11 u8=1, &outline arg12, arg13 f32=4.0)`
- font = `"asset/base/font/set/bold"` (len 0x18). rgba = {0.9098,0.9098,0.9098,t}. rect = 플레이트 rect. arg10/11=1,1(중앙정렬 추정 — 원본값 유지). outline = 16B {u64 0, u32 0, f32 t·0.8667}(원본값 그대로).
- text/font는 내부 clone → 호출 후 우리 text free 안전.

## 보조 함수
| RVA | 시그니처 | 용도 |
|---|---|---|
| 0x1217630 | `f(out RCX=&String24, ctx RDX(=헬퍼 param1), champ R8, len R9)` | 챔프 표시명("asset/base/text/champion"의 `<champ>.name`). String{cap@0, ptr@8, len@0x10}. 실패=(0,1,0). **free = 0x8b7f90(ptr,cap,1), cap≠0일 때만** (게임·모드 둘 다 Rust System 할당자 = 교차 free 안전, 기존 모드 선례 있음) |
| 0x99c860 | `f(store RCX, key RDX, len R8) → RAX = &(obj,vtbl) 16B 엔트리 주소 or NULL` | 키→텍스처 에셋. **fat ptr가 아니라 엔트리 주소**: obj=[RAX], vtbl=[RAX+8]. alias 해석+텍스처 type 검사 포함(부재/비텍스처=NULL). w=`((f32(*)(obj))vtbl[5])(obj)`(vtbl+0x28), h=vtbl+0x30, 반환 XMM0. 참조만(free 불요) |
| 0x8b7f80 | `(size, align)→ptr(0=실패)` | 게임 alloc |
| 0x8b7f90 | `(ptr, size, align)` | 게임 dealloc |
| 0x25e9af0 | `(&out, &src)` | String clone |
| 0x8b7fc0 | — | 가짜 noreturn 스텁 — 호출 불요 |

## 후속 RE 확정분 (2026-07-25 2차)

### 밴 2분할 경로 — 0x11f9030 경유로 자동 커버 [확정]
- 분할 페이즈 시퀀스: 0x248b690(list, snap_key)로 오프스크린 렌더타깃 열기 → **0x11f9030 3번째 콜사이트(0x124e4d1)가 카드 전체를 greyscale=1로 타깃 안에 그림** → 타깃 닫기 → 0x1201d90 ×2(상/하 반쪽)가 그 스냅샷 텍스처를 op0x05 폴리곤으로 제출.
- 0x1201d90 = 독립 드로우 `(list, tex_key, len, &rect, half_flag u8[1=상/0=하], t f32)` — 11f9030/fdabe0/c130 미호출. **건드릴 필요 없음** (스냅샷 원본이 곧 헬퍼 출력이므로 훅이 자동 반영).
- 0x11f9030 콜사이트 전수 = **3곳**: 0x124e390(일반 밴) / 0x124e4d1(분할 스냅샷) / 0x124f3bf(픽). 동일 시그니처.

### 훅 3지점 진입부 (0.5.2 실측)
| 함수 | 진입 바이트 | orig_len | 비고 |
|---|---|---|---|
| 0x11e2370 | `55 41 57 41 56 41 55 41 54 56 57 53` (push×8) | **12** | rip-rel 無·rax 의존 無 |
| 0x11f9030 | 동일 12B | **12** | 동일 |
| 0xfdabe0 | `55 56 57 53` + `48 81 EC 98 00 00 00` + `48 8D AC 24 80 00 00 00` | **19** | 전부 rsp 상대 |
- 세 함수 모두 **chkstk 없음**(`sub rsp, imm32` 직접) → rax-tail 안전, r11-tail도 안전. orig_len 구간 내 분기 타깃 없음.

### 0xfdabe0 정밀 시그니처 [확정]
`f(out RCX = *IllustOut(0x28B), store RDX(=11f9030 param1 패스스루), champ R8, len R9, tw arg5 f32, th arg6 f32)`
- out: +0x00 cap u64(실패 시 **-1만 기록**, 타 필드 미기록) / +0x08 ptr / +0x10 len / +0x18 u0 / +0x1C v0 / +0x20 uw / +0x24 vh. 추가 필드 없음.
- 호출자 소비: cap==-1이면 aseprite 폴백 / 아니면 키 (ptr,len)을 **c130에 그대로 전달(내부 clone)** → UV = out+0x18 → cd40(0) → color → grey → 제출 → **cap!=0이면 0x8b7f90(ptr, cap, 1)로 게임이 free**. ⚠cap은 dealloc size로 그대로 쓰임 → **할당 실크기와 정확 일치 필수**(본 모드: 게임 alloc(len,1) + cap=len).
- **fdabe0 콜러 전수 3곳**: ①0x11f9030(카드, tw/th=304/356) ②0x124f45a(픽 비행 미니 아이콘) ③0x1220a70(밴픽 보드 슬롯 위젯 — 일러 있으면 위젯 세팅 0xb5dc60, 없으면 137×184 스프라이트 폴백 0xfdd830). ⚠②③은 사이드 스태시 타이밍과 어긋날 수 있고 ③은 banpick_illust 슬롯 일러와 이중 표시 위험 → **본 모드는 tw/th 304×356 게이트로 카드만 리다이렉트**.

## 기하패치 — 밴 분할 연출 확대 (3차 RE 2026-07-25, 0.5.2)

바닐라 밴 카드/분할 스냅샷 = 360×480. 모드가 설치 시점에 아래를 패치해 **520×408**로 확대 (cfg ban_layout=1일 때, 전 사이트 사전검증 통과 시에만 일괄 적용).

**A. 배타 rdata 상수 (값 교체)** — xref 전수 확인, 0x124db10(+0x1201d90) 내부뿐:
| RVA | 바닐라 | 의미 | 새값(520×408, cut=60) |
|---|---|---|---|
| 0x3731380 | {-180,-240,360,480} | 카드 로컬 rect(밴 e0b2·픽 f381 공용, 1201d90 반쪽 rect 원본) | {-260,-204,520,408} |
| 0x37313b0 | {0,0,360,480} | 스냅샷 패스 헬퍼 rect(**좌상단 원점** — 07-25 "우하단 조각" 버그의 원인) | {0,0,520,408} |
| 0x37313e0 | {360,340} | 취소선 방향 = {w, h−2·cut} | {520,288} |
| 0x37313f0 | {-180,170} | 취소선 시작 = {−w/2, h/2−cut} | {-260,144} |
| 0x3731400 | {0,170} | 앵커 {0, h/2−cut} | {0,144} |
| 0x37313c0 | {0.6866,0.727} | 분리 법선 = (h−2c, w)/‖·‖ (별도 상수!) | {0.4845,0.8748} |

**B. 코드 즉치**: 0x124e2ba `mov dword [rsp+0x20], 0x43F00000`(480.0 = 스냅샷 타깃 높이, b730 arg5) → 408.0

**C. 공유 상수 → disp 재타깃** (.rdata 패딩 0x3fd2b00에 슬롯 [520, -60, 60, -260] 기록 후 disp32 재계산):
| disp 위치 | 현재 타깃 | 의미 |
|---|---|---|
| 0x124e2c2 | 360.0(광공유 31곳) | 스냅샷 타깃 폭(b730 arg4=xmm3) |
| 0x1201e19 / 0x1201e27 | ∓70.0(공유 14곳) | 1201d90 대각 컷 오프셋(하/상) |
| 0x124e8cf / 0x124efa1 | -180.0({-180,0} 외부 4곳 공유) | 지그재그 x 오프셋 {−w/2,0} |

- 취소선 진행 상수 0.82(0x1436e8e98)는 **0x11f9030 폴백 일러 스케일과 공유 — 건드리지 말 것**. 12.0/5.0(강도/세그) = 크기 무관, 패치 불요.
- 축소비행 스케일 = self+0x374/480.0·1.18 clamp[0.14,0.24] — 슬롯 기준이라 패치 불요.
- FUN_14248b730 = 명명 렌더타깃 등록 `(list, name, len, w=xmm3, h=arg5)`, 레지스트리 0x70 stride, 키 21B memcmp 중복검사(1회 렌더).

## idle 폴백·좌우반전 (4차 RE 2026-07-25, 0.5.2)

**반전 = tag4 cmd의 +0xad(flip_x)/+0xae(flip_y) u8** (인게임 유닛 tag3 +0x90/+0x91과 동일 메커니즘). Skia 디스패처 0x9f3090이 translate(x,y)→scale(±1)→로컬 (0,0,w,h) 드로우 ⟹ **flip_x=1이면 화면 커버 [x−w,x] → c130의 x에 "오른쪽 끝" 전달 필수**. c7c0/cd40/ff0c20/e850 전부 +0xad/ae 보존. ⚠음수 UV/음수 w는 반전 아님(empty rect 처리) — 사용 금지.

**idle 폴백 재현 시퀀스**:
1. `0x5ab7d0(store, "asset/base/aseprite_resources/champions/<champ>#anim", len) → anim*|NULL` (참조 반환, 유효 = obj && *(obj+0x18)!=0)
2. anim = hashbrown SwissTable: +0x00 ctrl / +0x08 mask / +0x18 items. 엔트리 = ctrl−(i+1)·0x30 `{kcap,kptr,klen,fcap,fptr,flen}` (ctrl[i]&0x80==0 = 점유). "idle"(len4, LE 0x656c6469) 탐색은 선형 스캔으로 충분.
3. Frame = 0x14B `{x,y,w,h,duration}` — duration>0 첫 프레임, 없으면 마지막. fw/fh = +0x08/+0x0c.
4. scale = clamp(min(iw·0.82/fw, ih·0.82/fh), 2.0, 6.4)
5. `0x121aca0(out RCX, store RDX, champ R8/R9, tw arg5, th arg6, scale arg7)` — **무부작용 순수 계산기**(렌더타깃 생성 아님·캐시 없음·재호출 안전). out(0x30B) = {cap[-1=실패]/ptr/len 시트키 String(caller 소유→free 0x8b7f90(ptr,cap,1)), +0x18 u0/v0/uw/vh(중심 크롭+champion_view 스타일 오프셋 반영), **+0x28 그릴 w=min(tw,scale·fw), +0x2c h**}
6. x = ix+(iw−w)/2 (flip이면 +w), y = iy+(ih−h)/2 → c130(x,y,0x4bc,w,h) → c7c0(uv) → **cd40(1)** → color(1,1,1,t) → (grey면 e850) → flip 바이트 → b1c0.

**기존 서술 정정 (이번 확정)**:
- ~~cd40 flag = "일러 에셋 경로=0/aseprite 캐시=1"~~ → **+0xac 샘플링 플래그: 1=nearest(픽셀아트)/0=linear**. 키 조회 경로는 동일(시트 키도 일반 에셋 store).
- ~~e850 = op7 셰이더 래퍼로 변환~~ → **tag4 유지, 셰이더 String(+0x20)만 교체**(uniform 리셋, flip·샘플링 보존).
- tag4 cmd 추가 필드: +0x20 셰이더 String(기본 "asset/base/shader/nine_patch") / +0x8c 회전 rad / +0x90 nine-patch 인셋 4 / +0xa0 피벗 / +0xac 샘플링 / +0xad flip_x / +0xae flip_y.
- fdabe0 arg5/6 = rect 파생 (rect.w−56, rect.h−124) — 기하패치 후 464×284 (훅 C 게이트 두 크기 수용).

## 잔여 미확정
- 외부에서 세 함수 프롤로그 중간(fn+1~11)으로 진입하는 코드 부재 = 표준 가정(전수 스캔 안 함)
- 0x1220a70(슬롯 위젯) 호출 시점의 사이드 문맥 — 슬롯 위젯까지 아트 적용을 확장할 경우 런타임 확인 필요(현재 게이트로 제외 중)
- 미상(실용 판정): op0x0a +0x1C=1 의미, c1e0 arg10/11·outline 의미, cd40 flag 렌더 의미 — 전부 원본값 사용
- 런타임 검증 권장: ①모드 에셋 키가 99c860에서 texture로 잡히는지 ②1217630 String 교차 free 1회 — 실패해도 폴백(원본 호출)으로 빠지게 구현

## 설계 메모 (본 모드)
- 훅 A(0x11e2370): 스태시(진영+mode+챔프키) 후 원본. 훅 B(0x11f9030): 픽이면 커스텀 가로형 레이아웃 전체 대체, 밴/아트부재/비활성이면 원본 트램폴린 폴백. 훅 C(0xfdabe0): 키를 `asset/tfm2_banpick_illust/illust/<side>/<champ>[-1]`로 리다이렉트(밴 경로 커버), 부재 시 원본.
- 아트팩: 배포 확인됨 — illust/blue 151·red 171·red_noflip 151 png, `<champ>.png` + `<champ>-1.png`(확대구도). 원본 1710×1044(1.638), -1 = 1140×696.
- 카드 로컬 rect 항상 {-180,-240,360,480}·transform은 호출자 몫 → 픽 커스텀 레이아웃은 로컬좌표로 자유 설계 가능(가로형 일러 영역).
- cfg: enabled / red_flip(1=red, 0=red_noflip) / zoom(1=`-1` 확대구도) / debug. 밴 진영별 아트 = 훅 A 스태시로 해결.
