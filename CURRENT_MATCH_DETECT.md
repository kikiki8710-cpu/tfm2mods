# TFM2 — "지금 화면에 재생 중인 경기" 식별 (재사용 레시피)

> 확립 2026-07-17 (게임 **0.5.1 정식**), tfm2_elemental_serpen 인게임 검증 완료.
> 행단위 RE 3건 + 유저 착안으로 도출. **다른 모드에서 그대로 복사해 쓸 수 있음.**

## 문제

TFM2는 **배경 리그 경기 30~40개가 동시에 sim을 돌린다.** sim 계층 훅(세르펜 AI, buy, AI 판단 등)에는
그 경기들이 전부 섞여 들어온다. 화면에 보이는 건 그중 **딱 하나**. 이걸 못 가리면:
- 전역 상태(색/표시)가 배경 경기 값으로 오염돼 깜빡임
- 배경 경기에 개입해 밸런스가 틀어짐

## ★해결: "재생할 경기를 고르는 지점"을 훅한다

착안: **배경에서 수십 경기가 도는데 화면엔 하나만 재생된다면, 그걸 고르는 코드가 반드시 있다.**

**Game 런처 `0x20588a0`** — 클라 씬빌더(`0x722ca0`)가 경기를 화면에 띄울 때 호출하며
**seed를 인자로 직접 넘긴다.**

```
0x20588a0  fn(rcx = out Game, edx = 셀렉터, r8 = seed(순수 u64), r9d = 0)
  프롤로그 12B 8-push: 55 41 57 41 56 41 55 41 54 56 57 53   (rip-rel 없음 → orig_len 12 안전)
```

**화면 경기 식별 = 호출자 주소(retaddr) 게이트** — 콜사이트 9곳 중:

| retaddr RVA | 정체 |
|---|---|
| **0x72f507** | ★화면 경기 (경로 A, call@0x72f502) |
| **0x733e9f** | ★화면 경기 (경로 B, call@0x733e9a) |
| 0x2061132 | 배경 리그 (FUN_142061100) → 자동 배제 |
| 0xc884f5 · 0x10e7834 · 0x111e650 · 0x13dd59b · 0x16238c8 · 0x1659d50 | 기타 |

⇒ **retaddr만 보면 화면 경기와 배경 경기가 깨끗이 갈린다.**

## 코드 (그대로 복사 가능)

```rust
const LAUNCHER_RVA: usize = 0x20588a0;
const LAUNCHER_PROLOGUE: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];
const LAUNCHER_RET_A: usize = 0x72f507;
const LAUNCHER_RET_B: usize = 0x733e9f;
static RENDER_SEED: AtomicU64 = AtomicU64::new(0);   // ★화면 경기 seed

// asm 스텁이 saved=rsp로 호출. push 순서 = r12,rsi,rdi,rbx,r11,r10,r9,r8,rdx,rcx (10개)
//   → saved[0]=rcx, saved[1]=rdx, saved[2]=r8, saved[3]=r9, ..., saved[10]=원래 [rsp]=retaddr
unsafe extern "C" fn cap_launcher(saved: *mut u64, _rsp: usize) -> u64 {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved.is_null() { return; }
        let seed = *saved.add(2);            // r8
        let ret  = *saved.add(10) as usize;  // return address
        let base = GetModuleHandleW(core::ptr::null());
        if base == 0 || ret <= base { return; }
        let rva = ret - base;
        if rva == LAUNCHER_RET_A || rva == LAUNCHER_RET_B {
            RENDER_SEED.store(seed, Ordering::Relaxed);  // ★화면에 재생할 경기 확정
        }
    }));
    0
}

// 설치 (init/declare_mod 시점 권장 — 경기 시작을 놓치지 않게)
install_stub_generic(LAUNCHER_RVA, 12, cap_launcher as usize, &LAUNCHER_PROLOGUE);
```

**sim 훅에서 사용:**
```rust
// rcx = provider(World)인 sim 훅(예: 세르펜 AI 0x1f8d0c0)
let seed = safe_read_u64(rcx + 0xeab8).unwrap_or(0);   // provider+0xeab8 = 경기 seed
let is_render_match = seed != 0 && seed == RENDER_SEED.load(Ordering::Relaxed);
```

`install_stub_generic` 구현은 `tfm2_elemental_serpen/src/lib.rs` 참조(asm 스텁: 레지스터 push →
`cap_fn(rsp)` 호출 → 복원 → 원본 12B 실행 → `fn+12`로 jmp).

## 핵심 근거

- **`provider + 0xeab8` = 경기 seed = 불변**(경기당 고유). write는 3곳뿐:
  ctor `0x21d1082`(런처 seed 1회 저장) / vtable 세터 `0x207a980`(**코드 xref 0건**) / deep-clone `0x224a34c`.
  read는 게터 + `Debug` format_args 2곳뿐 ⇒ **값 대조 안전**.
  (대조군: `+0xeac0` tick은 sim 루프에 hot write 있음)
- provider = **`World`(0xeaf0)** + 인라인 `MobaMode`(@+0xeaf0). `Game + 0x1dc0` = provider data / `+0x1dc8` = provider vtable.

## ❌ 재시도 금지 (전부 실패 확인)

| 방법 | 결과 |
|---|---|
| **db 메모리 스캔**(0..0x20000에서 Game→provider→seed 대조) | 매칭 0~3개 요동, **VEH 폴트 25만**(성능 재앙). 폐기 |
| **`db+0x1340` → `+0x1dc0` → `+0xeab8` 3-deref** | ❌ `db+0x1340`은 **Game 포인터가 아님**. ClientScene enum의 **인라인 payload 첫 8바이트**(실측 0). enum이라 payload 의미가 **태그마다 다름**(tag11~14에선 Game 포인터일 수 있으나 경기화면 태그에선 아님) |
| **db → provider 링크 탐색** | ❌ **존재하지 않음(확정)**. `GameView`(scene payload)는 **순수 이벤트-리플레이 렌더 상태** — rmeta의 Default derive가 전 필드(32개) 열거하는데 Game/World/provider/seed/match_id **없음**. (KB의 "GameClient→Game(Arc\<RwLock\<Game\>\>)" 기록은 근거 없음 = 반증됨) |
| **`runner_ctor 0x205a2f0` 훅** | ❌ **튜토리얼 전용 함수**(콜사이트 3곳 전부 `tutorial_morgad` 셋업). 0.5.0_3 → 0.5.1 마이그 때 **모노모픽 제네릭 사본**을 집은 것(8-push 프롤로그는 Rust 범용이라 변별력 0). 미발화(rctor_n=0) |
| **스폰 클로저 `0x50edd0`/`0x50e230`** | ❌ 잡히는 provider/tid가 화면 경기와 **무관**(★LIVE 매칭 0건) |
| **`RENDER_TID`(skia 렌더 스레드) 대조** | ❌ 렌더 tid ≠ sim tid (영원히 불일치) |
| **detour 호출 빈도(rate)로 판별** | ❌ 화면/배경 전부 600~900/s로 뭉쳐 구분 불가 |

## 관련 (같이 쓰면 좋은 것)

- **재생 커서(화면에 지금 보이는 프레임)** = `db + 0x1598` (u64, Spectator_Chat 검증).
  events Vec = `db+0x1670`(cap)/`+0x1678`(ptr)/`+0x1680`(len). **World.tick(`provider+0xeac0`)과 1:1 동일 축**(실측 비율 1.00).
  ⇒ sim은 재생보다 앞서 달리므로, **렌더 표시값은 played_tick 기준으로 조회**해야 함.
  ⚠유효성은 frames(ptr/len)만 검사할 것 — `played <= len` 같은 조건 추가 금지(둘은 비교 대상이 아님).
- **경기 화면 판정** = UI에 **`game_time` 노드 존재**(`ui_kit::find(&ui.root,"game_time").is_some()`).
  Spectator_Chat 검증 방식, 라이브/리플레이 모두 통과. ⚠`mod_api::Scene::InGame`은 "세션 진행중"이지 경기 화면이 아님.
  ⚠`db+0x1338`(ClientScene 태그) 해석은 KB 표가 실측과 어긋나므로 쓰지 말 것.
