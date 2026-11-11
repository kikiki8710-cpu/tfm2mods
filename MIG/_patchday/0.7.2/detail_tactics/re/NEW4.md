# NEW4 — 상세전술 0.7.2 새 전술 4개 창 RE (2026-11-11)

대상 exe = `F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe`(SHA256 `4B324F31…19FB` = 현행 `sites.rs` 와 같음). 기계판 = `NEW4.json`.

## 요약

| # | id(가칭) | 창 | 주체 Src | team_chk | 기본값 등가 | 권고·막힌 곳 |
|---|---|---|---|---|---|---|
| 1 | `nexus_finish_start_range` | 2 (`0xea1897`·`0xea18c1`) | Caller(`0x48`) 3갈래: engage `mem:rbp:0xb48` · battle `mem:rbp:0x290` · attack_nexus `stack:0xd0` | Caller 3갈래(`0xb10`·`0x268`·`stack:0x28`) | v=100 → `(250000+1)²`=`0xe8d51f221` / −1 = `0xe8d51f220` ✓ | Player 범위 권고. 헬퍼 호출처 3곳이 전부 |
| 2 | `weak_team_finish_caution` | 2 (`0x101b53f`·`0x101b575`) | Caller(`0x88`) 3갈래: `stack:0x68`·`stack:0x68`·`stack:0x60`(헬퍼가 push 한 호출자 rsi/rdi) | `reg:r14`(=`[Player+0x820]`) | v=100 → 원본 / v=0 → 계수 100(=0.7.1 식 `a+m<b`) ✓ | ⚠메인 원안(상한 95→v%)은 **아군이 있으면 할증이 최대 90** 이라 v≥95 무효 → **기울기 배율**로 대체 권고(원안 창도 검증해 `alt_cap_variant` 로 둠). Team 범위 |
| 3 | `objective_engage_call_range` (메인 가칭 「기지 근처 지원 요청 거리」) | **12** (r13 2 + 적↔나 5 + 오브젝트 없음 적↔나 5) | `mem:rbp:0xb48` | `mem:rbp:0xb10` | 230000²·180000²·250000²·250000²+1 ✓ | **「기지」가 아니다** — 적↔나(25만)·적↔**에픽/세르펜 지점**(23만). 180000² 갈래는 정적으로 도달 불가. C017 창은 이 경로와 **별개**(새 위치 조건 뒤 아님) |
| 4 | `pre_gank_enemy_check_range` | 2 (`0xe6b903`·`0xe6b90d`) | `mem:rbp:0xb48` (C004 와 같음) | `mem:rbp:0xb10` | 250000²·150000² ✓ | 그대로 구현 가능 |

- 창 18개 전부 `gen_extras.build_site` 검증(≥5B · call/ret/RIP 상대 없음 · 함수 안 직접 분기·점프표 유입 없음) + `.text` 전체 rel32 우연 유입 0 + 기존 210창과 겹침 0 통과(`scratchpad\newtac\check.py`).
- 기본값 등가: `verify_default_equiv.py` 의 `relocated()`·Unicorn 하네스를 그대로 써서 창마다 무작위 200표본(레지스터·플래그 무작위) — 원본 경로와 동적 경로의 **16레지스터·플래그·출구 주소 전부 일치 18/18**(대안 2/2). 비기본 값에서 바뀌는 레지스터는 값 레지스터 1개뿐(`scratchpad\newtac\emu_check.py`).
- 사이트 수: 210 + 18 = 228 ≤ `CAP` 256.

---

## §1 넥서스 마무리 시작 거리 — 헬퍼 `0xea1800`(finish_race.rs:515)

### 헬퍼 본문(근거)
```
ea1800 push r14 ; push rsi ; push rdi ; push rbx ; sub rsp,0x28        ; 창 rsp = 진입 −0x48 → 반환주소 [rsp+0x48]
ea1809 cmp byte [rcx+0xee3],2 ; jne 거짓 ; ea1816 cmp byte [rcx+0xee6],4 ; jne 거짓   ; TeamPlan: Focus ∧ Nexus
ea1823 mov ecx,1 ; ea1828 sub rcx,[r8+0x820]                         ; r8 = Player(A3) → 적팀
ea1839 mov rax,[rsp+0x70] ; ea183e rdx=[rax+0x658] ; ea1845 r8=[rax+0x660] ; A5 = 내 챔피언 좌표 (★여기서 r8=Player 소실)
ea184f mov rax,[r10+rcx*8+0x1a0] ; test ; je ea18a6                  ; 적 넥서스
ea1897 movabs r9,0xe8d51f221 ; cmp rsi,r9 ; jb ea1922(참)             ; ▶창1  거리² < 250001²
ea18aa r9=[r10+rcx+0x140] ; rcx=[r10+rcx+0x158]                       ; 남은 적 구조물 목록·개수
ea18c1 movabs r11,0xe8d51f220                                         ; ▶창2
ea18d0.. 루프: cmp r14,r11 ; ja ea18d0(다음) → 하나라도 거리² ≤ 250001²−1 이면 참
```
- 두 비교 모두 「거리² < (R+1)²」이다(`0xe8d51f221 = 250001²`). 그래서 값 = `(R+1)²`·`(R+1)²−1`, R = 250000×v/100. 메인 문구 「제곱+1」은 `R²+1` 이 아니라 **`(R+1)²`** 이어야 기본값이 원본과 같다(`R²+1 = 0xe8d4a5101` ≠ `0xe8d51f221`).
- rbp 는 헬퍼가 건드리지 않는다(`widen\pro.py`: rbp 쓰기 0).

### 호출처 전수 = 3 (E8 3 · E9/lea-rip/절대주소 참조 0)
| 호출 | 함수 | r8(=Player) 출처 | 참일 때 |
|---|---|---|---|
| `0xe672d2` (ret `0xe672d7`) | engage `0xe65690` | `0xe67285 mov r8,[rbp+0xb48]` · flow: `[rbp+0xb48]←A4`, `r8←A4` | 교전 제안 생략 → 함수 끝(W1 ③-2 a) |
| `0x106bce1` (ret `0x106bce6`) | battle `0x106b890` | `0x106bc93 mov r8,[rbp+0x290]` · flow: `[rbp+0x290]←A4` · `0x106bc9a lea rax,[r8+0x820]` | `0x106bcf5~` 사유 4 · `[r15+0xe0]=4` 전투 종료 |
| `0x1168ac4` (ret `0x1168ac9`) | attack_nexus `0x11689f0` | `0x1168ac1 mov r8,rax`, rax=`[rsp+0x80]`=A5(`0x11689fd`) · `[rax+0x820]`팀·`[rax+0x8b0]`포지션 | `0x1168acd mov byte [rsi+8],1` 집중 직행 |

### 주체
- 창 시점 헬퍼 안에는 Player 가 없다(`0xea1845` 에서 r8 덮임) → **Caller(`return_off 0x48`)**.
  - engage: rbp 그대로 → `mem:rbp:0xb48` / 팀 `mem:rbp:0xb10`(flow `[rbp+0xb10]←[A4+0x820]`, G4 와 같은 슬롯).
  - battle: `mem:rbp:0x290` / 팀 `mem:rbp:0x268`(flow `[rbp+0x268]←[A4+0x820]`).
  - attack_nexus(rbp 프레임 없음 · 6push+0x28=0x58): Player = 호출자 `[rsp+0x80]` = 창 rsp + 8(ret) + 0x48 + 0x80 = **`stack:0xd0`** / 팀 = 호출자 rbx(`[A5+0x820]`, `0x1168a05`)를 헬퍼가 push 한 자리 **`stack:0x28`**.
- 「팀 번호만으로 내 팀 확인」은 지금 장치로는 안 된다: `micro.rs` dispatch(L117–128)가 Src 로 Player 를 얻어 `+0x810` athlete 로 `is_my_athlete` 를 본다. 기존 Team 범위 전술(C057 `mem:rbp:0x5f0`/`0x5e0` · C011)도 Player Src + 팀 확인 꼴이다.
- 범위 권고 **Player**: 거리는 선수마다 자기 챔피언 기준이다(Team 으로 해도 같은 Src 로 된다).

### 기본값 등가 · 콜백
- v=100: `(250000+1)² = 62,500,500,001 = 0xe8d51f221` ✓ · `−1 = 0xe8d51f220` ✓. v=400 → 1,000,001² ≈ 1.0e12(u64 여유).
```rust
// extra_ops.rs
fn finish_reach(v:i64)->Option<u64> {let r=scale(250_000,valid(v,25,400)?)+1;Some(r*r)} // 0.7.2 0xea1800: 거리² < (R+1)²
pub(crate) fn finish_reach_lt(_: &Frame,v:i64)->Option<u64> {finish_reach(v)}
pub(crate) fn finish_reach_le(_: &Frame,v:i64)->Option<u64> {Some(finish_reach(v)?-1)}
```
PORT: `'finish_reach_lt': lambda f, v: (250000 * n(v, 25, 400) // 100 + 1) ** 2` · `'finish_reach_le': lambda f, v: (250000 * n(v, 25, 400) // 100 + 1) ** 2 - 1`

### 쉬운 말(제안)
「집중 공성으로 넥서스를 노릴 때, 적 넥서스나 남은 적 구조물이 이 거리 안에 들어오면 싸움을 접고 바로 넥서스를 칩니다. 100 = 원래(25만). 높일수록 더 멀리서부터 마무리에 들어갑니다.」

### 위험·한계
- 0.7.1 은 engage·attack_nexus 에서 이 거리 조건이 없었다(W1 ③-2 a · W3 3.1) → 값을 크게 할수록 0.7.1 동작에 가깝다. battle 쪽 0.7.1 대응(`BP+0x47e` 250000)은 W3 의 **추정**.
- 「남은 적 구조물」 목록(`cache+팀*0x20+0x140`) 이름은 미확인(W3 ④ — 0.7.0 「쌍둥이 포탑 없음」 판정에 쓰던 목록).
- 같은 250001² 가 경주 공격자 집계 `0xf96686 movabs r13,0xe8d51f221`(`0xf96630` ← `0x101e020` 추정기)에도 있다. **이 방식(내 선수 한정 창)으로는 넣지 않는다** — 추정기 결과가 15틱 TLS 캐시(W8b §A)라 선수별 값이 캐시에 섞인다. 필요하면 별도 설계.

---

## §2 약팀 마무리 신중도 — `0x101b430`

### 본문(근거)
```
101b430 push r15,r14,r12,rsi,rdi,rbp,rbx ; sub rsp,0x50           ; 창 rsp = 진입 −0x88 → 반환 [rsp+0x88]
101b43e rsi=r9(여유) ; rbx=r8(ctx) ; r14=rdx(A2=Player) ; 101b45a call 101b230(flag=1)  ; a=[+0x38] b=[+0x40] flag=byte[+0x48]
101b479 mov r14,[r14+0x820]                                     ; ★Player 소실 → r14=팀
101b491 call [vt+0x220] ; je 101b53f                            ; 선수 목록(비면 아군 없음)
101b4cc~101b51f 내 팀마다 x=min(100, [+0x288]·[+0x218]/1000·[+0x280]/1000) ; sum+=9x+100 ; n++
101b521 n=max(n,1) ; avg=sum/n ; 101b54f c=sat(1000−avg)
101b55b add=((c·0xccd)>>15)&0xfffe ; 101b569 cmp ax,0x5f ; mov ecx,0x5f ; cmovb ecx,eax   ; add=min(add,95)
101b575 add ecx,0x64 ; movzx ecx,cx                              ; ▶창(권고) 계수=100+add
101b53f mov ecx,0xc3 ; jmp 101b57b                               ; ▶창 아군 없음 = 195
101b598~101b5c6 a·계수/100 + 여유 < b → al
```

### 메인 원안(상한 95 → round(95×v/100)) 평가
- 구현은 된다: 창 `0x101b569` `66 83 f8 5f b9 5f 00 00 00`(9B · cmp+mov 묶음) · dyn `push r11; mov r11,[rsp-8]; cmp ax,r11w; mov ecx,r11d; pop r11` · 검증 통과(`NEW4.json` `alt_cap_variant`).
- ★그러나 아군이 1명이라도 있으면 `x≤100` → `avg∈[100,1000]` → `c∈[0,900]` → `add ≤ (900·3277)>>15 = 90`. **상한 95 는 아군이 있을 때 한 번도 걸리지 않는다**(W5 「캡 95 는 선수 0명일 때만」과 같은 결론). 그래서
  - v=100~130: 상한 95~124 → 아군이 있으면 효과 0(아군 없음 계수만 195→224).
  - v<95: 「할증이 상한보다 큰 약팀만」 잘리는 계단형 — 「신중도 %」 뜻과 다르다.
- **권고 = 기울기 배율**: 계수 = 100 + min(add,95)×v/100 · 아군 없음 = 100 + 95×v/100.
  - 창 `0x101b575` `83 c1 64 0f b7 c9`, `first_len 3`(첫 명령 `add ecx,0x64` 만 교체 · `movzx` 는 창에서 그대로) · dyn `mov ecx,[rsp-0x10]; add ecx,0x64` → 콜백은 `ecx×v/100` 을 내고 원래 add 가 그 위에서 돈다 → **플래그까지 원본과 같다**(처음 `mov ecx,값` 만 쓴 판은 플래그만 달라 등가 검사 △ → 이렇게 고침).
  - 창 `0x101b53f` `b9 c3 00 00 00`(5B · `je` 유입은 창 첫 바이트라 허용) · dyn `mov ecx,[rsp-0x10]`.
  - 값 예(add=90 인 가장 약한 팀): v 0/50/100/130 → 계수 100/145/190/217. 아군 없음 100/147/195/223.

### 기본값 등가
- v=100: `scale(c,100)=c` → `add ecx,0x64; movzx` 원본과 동일(레지스터·플래그) ✓ · 아군 없음 `100+95 = 0xc3` ✓.
- v=0: 계수 100 → `a·100/100 + m < b` = `a + m < b`(`mul`·`shr 2`·`×0x28f5c28f5c28f5c3>>66` 은 정확한 ÷100) = **0.7.1 식**(W8b §B 구 `@eaca01~eaca37`). 단 0.7.2 의 **추정기 새 플래그 요구(byte `+0x48`) · 15틱 캐시는 남는다**(이 전술 범위 밖).

### 어디에 넣어야 「진입·유지」에 듣나
- 능력 할증이 있는 곳은 `0x101b430` 하나다. 호출처 3곳 전부(E8 3 · 그 밖 참조 0) = **시작 `0xfd5fa8`(fd5f00) · 잠금 해제 `0xea6395`(ea6090) · 유지 중 Focus 전환 판정 `0xeb5a2a`(eb3ee0)** → 한 곳의 창으로 셋 다 덮인다.
- 「Focus 유지」 판정 `0x101b3a0`(호출 `0xeb5bc9` 하나)은 `0x101b3ec mov edx,0x64` = `a·100/100 < b`(여유 없음) — 게임이 원래 할증을 안 건다. 이 전술은 거기에 듣지 않는다(설계상 같음 · 넣으려면 능력 계수를 콜백이 다시 계산해야 해서 이 방식으로는 하지 않음).

### 주체
- 창 시점 Player 는 레지스터에 없다(r14=팀) → **Caller(`return_off 0x88`)** — 헬퍼가 push 한 호출자 레지스터:
  - `0xea639a`: ea6090 `0xea638f mov rdx,rsi`, flow `rsi←A4` = handler `0xe411a7 mov r9,[rbp+0x2fa0]`(Player) → push rsi 자리 **`stack:0x68`**(진입−0x20).
  - `0xeb5a2f`: eb3ee0 `0xeb5a21 mov rdx,rsi`, flow `rsi←A4` ← eaf380 A4 ← eb99b0 A4 ← handler `0xe449f7 mov r9,[rbp+0x2fa0]` → **`stack:0x68`**.
  - `0xfd5fad`: fd5f00 `0xfd5f9f mov rdx,rdi`, flow `rdi←A3` ← ea6090 `0xea63f1 mov r8,rsi`(A4 Player) → push rdi 자리 **`stack:0x60`**(진입−0x28).
  - 그 슬롯은 함수 안에서 안 덮인다(출력 구조 `[rsp+0x30..0x4f]` · 인자 `[rsp+0x20]` 만 씀).
- team_chk **`reg:r14`** = `[A2+0x820]`(flow `r14←[A2+820]`, 두 창 모두) — 호출처와 무관하게 쓸 수 있는 교차 확인.
- 범위 **Team** 권고: 팀 플랜(siege_kind) 결정이고 팀원 handler 각각이 평가할 수 있어 Player 범위면 「누가 평가했나」에 따라 팀 결정이 흔들린다.

### 콜백
```rust
// extra_ops.rs
pub(crate) fn finish_caution_alone(_: &Frame,v:i64)->Option<u64> {Some(scale(95,valid(v,0,130)?)+100)} // 0x101b53f 아군 없음 195
pub(crate) fn finish_caution_scale(f:&Frame,v:i64)->Option<u64> {let c=f.reg(1)?&0xffff_ffff;Some(scale(c,valid(v,0,130)?)&0xffff_ffff)} // 0x101b575 ecx=min(할증,95)
```
PORT: `'finish_caution_alone': lambda f, v: 95 * n(v, 0, 130) // 100 + 100` · `'finish_caution_scale': lambda f, v: (f.reg(1) & 0xffffffff) * n(v, 0, 130) // 100 & 0xffffffff`
(대안: `'finish_cap': lambda f, v: (95 * n(v, 0, 130) + 50) // 100` · `'finish_cap_alone': … + 100`)

### 쉬운 말(제안)
「팀 능력치가 낮을수록 ‘집중 공성으로 우리가 먼저 넥서스를 깬다’는 계산을 비관적으로 봅니다(원래 최대 1.9배). 0 = 능력 보정 없음(0.7.1 방식), 100 = 원래, 130 = 보정 1.3배. 공성을 시작·전환할 때만 보고, 이미 집중 공성 중인지 유지 판정에는 쓰지 않습니다.」

### 위험·한계
- 능력 계수 필드(`+0x288·+0x218·+0x280`) 이름은 추정(W8b ④). 런타임 검증 = `0x101b430` 반환을 v=0/100 에서 구 식 `a+m<b` 와 대조.

---

## §3 오브젝트 근처 교전 요청 거리 — engage `0xe65690` 블록 `0xe6a0a6~0xe6ae84`

### 누구와 어느 지점 사이 거리인가 (확정)
- **적↔나**: 적 챔피언 `rdi=[[rbp+0xae0]+적팀*0x28+i*8]`(cache+0x210 챔피언 표 · `[rbp+0xb18]`=적팀) ↔ **내 챔피언** `r14=[[rbp+0xb08]+[rbp+0xae8]*8]`(G4: `[rbp+0xb08]`=내 팀 챔피언 표 · `[rbp+0xae8]`=포지션). 예 `0xe6aa3f~0xe6aa83` → `0xe6aa86 movabs rdx,0xe8d4a5100 ; cmp r8,rdx ; ja 다음 적`.
- **적↔오브젝트 지점**: `0xe6a977~0xe6a9ac` 지점 = `[[[rbp+0xb28]+0x20] + 0x7068 + esi*0x20 + 내팀*0x10]`(x,y). esi = 4(`0xe6a5a6`, `[LPH+0xc88]` 갈래 · 팀 목표 `[LPH+0xfe6]`=0) 또는 5(`0xe6a253`, `[LPH+0xc68]` 갈래 · `[LPH+0xfe6]`=1). 표 4·5 = **에픽·세르펜 위치**(근거: 0.7.1 W8 ③-4 `[world+0x7068+kind×0x20+팀×0x10]` kind 4 epic·5 serpen · 0.7.2 W5 §3 오브젝트 지점 `+0x80` · 0.5.3 epic_hunt kind 4/serpen 5 — 번호↔이름은 0.7.2 에서 재확인하지 않음). `0xe6aac0 cmp rdx,r13 ; jbe 요청(0xe6ae4d)`.
- **「기지」 근거는 찾지 못했다.** W1 의 「우리 기지 지점·라인 4·5」 표기는 위 근거로 「에픽·세르펜 지점」으로 정정 제안. 우리 넥서스(`[cache+0x1a0+내팀*8]`)를 쓰는 100000²+1·200000²+1 검사(`0xe6a1e7`·`0xe6a575`)는 **다른 생성기**(`0xe69ff6`/`0xe6a06f` 진입)이며 이 블록에서 도달 불가(CFG: `0xe6a0a6 → 0xe6a1e7/0xe6a575` False).

### 흐름
```
e69ebe cl=[LPH+0xc6b] ; ≠2 → eax=([c68]<<8)|1 → 점프표[1]=e6a196: [c68]==1 일 때만 e6a0a6
       =2 → [c8b]≠2 → eax=[c88]<<8 → 점프표[0]=e6a196: [c88]==1 일 때만
             [c8b]==2 → al=[fe6](팀 목표): 0·1 → [fe7]==1 일 때만 · 2,3,4,6,7,8·None → 진입 · 5 → 건너뜀 · 9 → [fe8]&1
e6a0a6 게이트: e6a0ad cmp dword [[rbp+0xad0]],1 ; ja HP검사 · 아니면 LPH+0x2858==1 ∧ +0x1c80==4 여야 함  (= [[rbp+0xad0]]≥2 ∨ (…))
       → 내 챔프 HP% > 59(0xe6a121/0xe6a131 cmp rax,0x3b)
e6a13b 반경 선택: c68 갈래 → esi=5 · c88 갈래 → esi=4 · 팀 목표 0→4·1→5 (상태 1~2 → e6a264 r13=230000² · ≥3 → e6a96d 180000² · 0 → 오브젝트 없음)
       팀 목표 None·기타 → e6a5bf 오브젝트 없음 경로
오브젝트 경로(e6a977~e6ae25) 적 5명: 보임 ∧ 0x10e95e0(적 자기 우물 구석) 아님 ∧ 적↔나 ≤ 250000² ∧ 적↔지점 ≤ r13 → e6ae4d
오브젝트 없음 경로(e6a5bf~e6a92b) 적 5명: 보임 ∧ 우물 밖 ∧ 적↔나 < 250000²+1 → e6ae4d
e6ae4d 요청 {0x249f0=150000, 0x3c=60, 0, 0x3d090, 0x3d090, flags 0x3010100} → call 0xe62430 (교전 제안 — 요청 종류·소비는 추정)
```
- ★**180000² 갈래(`0xe6a96d`)는 정적으로 도달 불가**: 입구 점프표가 상태 바이트(`[c68]`·`[c88]`·`[fe7]`)==1 일 때만 `0xe6a0a6` 에 들이고, 반경 선택은 같은 바이트를 다시 읽어 `(1−1)<2` → 항상 `0xe6a264`(230000²). 사이에 호출이 없어 값이 바뀔 수 없다. `0xe6a96d` 의 유일 선행 `0xe6a5b9` 도 같은 조건. ⟹ 실제 반경은 230000 하나. 창은 일관성용으로 넣었다(빼도 됨 · 실행 비용 0).
- 0.7.1 은 HP>59 뒤 위치 조건 없이 요청했다(W1 ③-2 b). 값을 키울수록 0.7.1 에 가깝다(보임·우물 밖 조건은 남음).

### 창 12 (전부 10B movabs · 첫 바이트 외 유입 없음)
| 창 | 명령 | 소비 | dyn | 콜백 |
|---|---|---|---|---|
| `0xe6a264` | `movabs r13,0xc5115f100`(230000²) | `cmp rdx,r13 ; jbe 요청`(적 5명 공용) | `mov r13,[rsp-0x10]` | `objective_call_wide` |
| `0xe6a96d` | `movabs r13,0x78b30c400`(180000²) | 같음(도달 불가) | 같음 | `objective_call_narrow` |
| `0xe6aa86`·`0xe6ab60`·`0xe6ac3a`·`0xe6ad14`·`0xe6adee` | `movabs rdx,0xe8d4a5100`(250000²) | `cmp r8,rdx ; ja 다음 적` | `mov rdx,[rsp-0x10]` | `objective_call_enemy` |
| `0xe6a688`·`0xe6a72a`·`0xe6a7cc`·`0xe6a86e`·`0xe6a918` | `movabs rax,0xe8d4a5101`(250000²+1) | `cmp r8,rax ; jb 요청` | `mov rax,[rsp-0x10]` | `objective_call_enemy_free` |
- 메인이 적은 `@0xe6aa86` 은 **적 5명 언롤 사본 중 0번**이다 — 하나만 바꾸면 적 슬롯 0만 바뀐다. 5사본 전부 넣었다.
- 「오브젝트 없음」 경로 5사본(`0xe8d4a5101`)은 메인 목록에 없었지만 같은 「적↔나 25만」이라 함께 넣는 것을 권고(빼면 「에픽·세르펜이 걸렸을 때만」 듣는 7창 전술).
- 주체: flow 결과 14창 모두 `[rbp+0xb48]←A4`(Player) · `[rbp+0xb10]←[A4+0x820]` = C004 와 같은 Src.

### C017(`base_battle_call_range`, 창 `0xe6c4e4`)과의 관계 — 별개
- C017 창 = `0xe6c4de call 0x1015e60(rng,0,1000)` 뒤 `cmp rax,0x289 ; ja` → 통과하면 `[LPH+0x2568]` 큐에 태그 2(전투 콜) 삽입. 그 앞은 `0xe6c41f call 0x106b490` → `[rbp+0xa60]` 요청(`0xbe00`·`0x2000001`) → `0xe6c4b4 call 0x1078070`(G4: 앞 게이트 거리² ≤ 120000² · `LPH+0x2858==1 ∧ +0x1c80==4`).
- CFG(`scratchpad\newtac\cfg.py`): 함수 입구 → `0xe6c4e4` 가 `0xe6a0a6` 를 피해서도 도달(True) · `0xe6a0a6` → `0xe6c4e4` 는 합류점 `0xe69e2a` 를 빼면 도달 불가(False). 요청 구조체도 다르다(`[rbp+0x4c0]` vs `[rbp+0xa60]`).
- ⟹ **C017 창은 새 위치 조건 뒤에 있지 않다.** W1 의 「효과 범위 축소 가능」 우려는 이 창에는 해당하지 않는다(범위: 0.7.2 engage `0xe65690` 정적 CFG 기준).

### 기본값 등가 · 콜백
- v=100: `230000²=0xc5115f100` · `180000²=0x78b30c400` · `250000²=0xe8d4a5100` · `+1=0xe8d4a5101` ✓(비교식 그대로: ≤R² / <R²+1).
```rust
// extra_ops.rs
pub(crate) fn objective_call_wide(_: &Frame,v:i64)->Option<u64> {let r=scale(230_000,valid(v,25,400)?);Some(r*r)}
pub(crate) fn objective_call_narrow(_: &Frame,v:i64)->Option<u64> {let r=scale(180_000,valid(v,25,400)?);Some(r*r)}
pub(crate) fn objective_call_enemy(_: &Frame,v:i64)->Option<u64> {let r=scale(250_000,valid(v,25,400)?);Some(r*r)}
pub(crate) fn objective_call_enemy_free(_: &Frame,v:i64)->Option<u64> {let r=scale(250_000,valid(v,25,400)?);Some(r*r+1)}
```
PORT: `(230000 * n(v, 25, 400) // 100) ** 2` · `(180000 …) ** 2` · `(250000 …) ** 2` · `(250000 …) ** 2 + 1`

### 쉬운 말(제안) — 이름도 「오브젝트·근처 적 교전 요청 거리」 권고
「체력 60% 이상인 선수가 근처의 보이는 적을 보고 교전을 제안하는 거리입니다. 원래는 적이 나와 25만 안에 있어야 하고, 에픽·세르펜이 걸려 있으면 그 적이 오브젝트 자리에서 23만 안이어야 합니다. 100 = 원래. 높일수록 더 먼 적에게도 반응합니다.」

### 위험·한계
- 요청 `{150000,60,…,0x3010100}` 의 소비(`0xe62430` 이후)와 게이트 `[[rbp+0xad0]]`·`LPH+0xc68/+0xc88`(Option 꼴 · `+3` 바이트 2=없음 추정) 이름은 미확인.
- 한 번 평가에 최대 6창(r13 1 + 적 5)이 dispatch 된다(같은 함수의 C004 와 비슷한 규모).

---

## §4 갱 선공 적 확인 거리 — engage 평가 입구 `0xe6b8c1`

```
e6b8f1 call [World.vt+0x40] ; e6b8f5 test rax,rax ; e6b8f8 mov rax,[rbp+0xb18] ; e6b8ff lea rdx,[rax+rax*4]
e6b903 movabs rcx,0xe8d4a5100   ; ▶창1 250000²
e6b90d movabs rax,0x53d1ac100   ; ▶창2 150000²
e6b917 cmove rax,rcx            ; vt+0x40 == 0 → 250000² · ≠0 → 150000²
e6b977 cmp r11(적↔내 챔프 거리²),rax ; ja 다음 적   (적 5명 언롤 · rax 를 5번 모두 재사용: 0xe6bbb6·…·0xe6bf31)
→ 보임 ∧ 우물 밖이면 0xe6c021(평가 [rbp+0xad0]>=2 → 0xf1da50) · 5명 모두 탈락이면 평가 건너뜀(G4)
```
- 창 사이 `cmove` 가 쓰는 ZF 는 `0xe6b8f5 test` 에서 온다 — 스텁이 pushfq/popfq 로 보존하고 dyn(`mov`)은 플래그를 안 건드린다(무작위 플래그 200표본 일치).
- 주체 = C004 와 같은 `mem:rbp:0xb48` / `mem:rbp:0xb10`(flow 두 창 모두 `←A4`·`←[A4+820]`). C004 창(`0xe6b89d`·HP 5곳)과 겹침 없음.
- 대안(1창): `0xe6b903~0xe6b91a` 24B(`movabs rcx`·`movabs rax`·`cmove`) 를 한 창으로 잡고 콜백이 `f.flags()` 의 ZF 로 둘 중 하나를 고르는 꼴(dyn = `mov rax,[rsp-0x10]` + 원래 `movabs rcx,0xe8d4a5100` 재현). dispatch 1회로 줄지만 지금 문체(창 1개=상수 1개)와 다르다 — 메인 판단.

### 기본값 등가 · 콜백
- v=100: `250000² = 0xe8d4a5100` · `150000² = 0x53d1ac100` ✓.
```rust
// extra_lane.rs (pregank_radius 옆)
pub(crate) fn pregank_enemy_radius(_f:&Frame,v:i64)->Option<u64>{if !(25..=400).contains(&v){return None;}{let r=250000u64.wrapping_mul(v as u64)/100;Some(r.wrapping_mul(r))}}
pub(crate) fn pregank_enemy_radius_alt(_f:&Frame,v:i64)->Option<u64>{if !(25..=400).contains(&v){return None;}{let r=150000u64.wrapping_mul(v as u64)/100;Some(r.wrapping_mul(r))}}
```
PORT: `lambda f, v: (250000 * v // 100) ** 2 if 25 <= v <= 400 else None` · `(150000 …) ** 2 …`

### 쉬운 말(제안)
「갱 도착 전 선공을 검토하기 전에, 내 챔피언 근처에 보이는 적 챔피언이 있는지 보는 거리입니다(원래 25만 · 특수 상태 15만). 100 = 원래. 높이면 더 먼 적이 있어도 선공을 검토합니다.」 (C004 설명이 이미 「갱 시도 시 적 확인 거리」를 가리킨다.)

### 위험·한계
- `World vt+0x40` 의 뜻은 추정(W1 ④ 「생략」 게이트). 150000 갈래가 언제 쓰이는지는 런타임으로 봐야 한다.

---

## §5 승격 메모
- `gen_extras.records()` 의 `len(rows)==56`·`==68` assert · `visible_extras.json` 29 · `tactics::N`/범주는 새 전술 4개에 맞게 바꿔야 한다(승격 쪽 몫).
- `verify_default_equiv.py` PORT 에 콜백 10개(대안 2개 별도)를 추가해야 기계 검사가 돈다. 표본 부족 우려 없음(콜백이 입력 범위 밖에서 None 을 안 낸다 · `finish_caution_scale` 도 모든 rcx 에서 원본과 같음).
- `tab`·`low/high/min/max` 는 제안값: 1·2 = 「푸시·타워」, 3 = 「오브젝트」, 4 = 「갱·로밍」.
- 쓴 도구(읽기 전용 · 스크래치 `C:\Users\jungs\AppData\Local\Temp\claude\F--claude-tfm2--claude-worktrees-salary-transfer-fee-analysis-1bf168\c5a49446-ad0c-452b-95b1-138bd849eaca\scratchpad\newtac\`): `gen_new4.py`(창 바이트를 exe 에서 직접 읽어 JSON 생성) · `check.py` · `emu_check.py` · `cfg.py`(함수 CFG 도달성).
