//! steal_score — game-ai\src\plan_legacy\sub_plan\steal.rs
//!   게임 0.5.8 · RVA 0xcbbca0 (269B / 70명령) · 원본 행 27,28 · 역할 scorer
//!   콜리 0x31a37c0 · vtable 슬롯 0x40, 0x1f0
//!   필드 오프셋 0x8×6 0x10×2 0x40×2 0x58×1 0x60×2 0xb1×1 0x1f0×1 0x5c0×2
//! 포팅 규약(CLAUDE.md §3): 게임 함수 호출 0 · 메모리 읽기는 judge::world(safe read) 경유 · 반환 None = 판단 불가(가드) → 호출부가 게임 원본으로 passthrough.
//! 스켈레톤 생성 = python MIG\aiport.py skeleton steal_score (디컴 원문은 아래 주석). 이 파일은 사람이 채운다 — 재생성은 --force 뿐.
use crate::*;
use super::super::world::*;
use super::super::ScorerArgs;

use super::super::layout::*;

/// steal(SubPlan 18) 스코어러 포팅 — 원문 = 아래 [MANUAL] 복원(0.5.8, capstone + JT 0x1433d76ec 17아암).
/// 반환: Some(점수) / None = 재현 불가 경로(가드: 포인터 불량·미지 태그) → passthrough(게임 원본).
/// RNG-free(난수 호출 0) → 검증·대체 모두 안전.
pub unsafe fn steal_score(a: &ScorerArgs) -> Option<i64> {
    let st = a.p1;        // rcx  = steal 상태
    let holder = a.p6;    // [rsp+0x58] = Holder(목표 슬롯 보유, *holder = World-ish X)
    let action = a.p7;    // [rsp+0x60] = 평가 대상 SmallAction
    if !ptr_ok(st) || !ptr_ok(holder) || !ptr_ok(action) { return None; }
    let w = World::from_holder(holder)?;
    // phase(st+8): 0 → T0 슬롯 / 2 → 0(도둑질 완료) / 그 외(1) → T1 슬롯
    //   (게임은 각 분기에서 vt+0x40(data)!=0 이면 panic — assert 취급, 재현하지 않는다)
    let phase = rd_u8(st + STEAL_PHASE);
    let (set_off, slot_off) = match phase {
        0 => (HOLDER_T0_SET, HOLDER_T0_SLOT),
        2 => return Some(0),
        _ => (HOLDER_T1_SET, HOLDER_T1_SLOT),
    };
    if rd_u64(holder + set_off)? == 0 { return Some(0); }          // 목표 미지정 → 0
    let slot = rd_u64(holder + slot_off)? as usize;
    if !ptr_ok(slot) { return None; }
    let h = rd_u64(slot)?;                                          // *slot = 목표 핸들
    let tgt = match w.entity(h) { Some(e) => e, None => return Some(0) };   // vt+0x1f0 순수 재현: NULL → 0
    let tgt_h = tgt.handle()?;                                      // tgt+0x5c0
    // 태그 디스패치: idx = (tag >= 3) ? tag-3 : 7  ← `sub dl,3; cmovae`  (태그 0·1·2 는 태그 10 과 같은 아암)
    let tag = rd_u8(action + SA_TAG);
    let idx = if tag >= 3 { tag - 3 } else { 7 };
    Some(match idx {
        4 | 7 | 8 | 9 => 90,                                                              // 태그 7·10(+0·1·2)·11·12
        11 => if rd_u64(action + 0x60)? == tgt_h { 100 } else { -99999 },                 // 태그 14: action[+0x60] == 목표 핸들
        12..=15 => if rd_u64(action + 0x08)? == tgt_h { 1000 } else { -99999 },           // 태그 15~18: action[+0x08] == 목표 핸들
        0..=3 | 5 | 6 | 10 | 16 => 0,                                                     // 태그 3~6·8·9·13·19
        _ => return None,                                                                 // idx ≥ 17 = JT 범위 밖(태그 ≥ 20): 게임은 UB — 재현 불가
    })
}

// ═══ 원본 디컴(자동 동봉, 참고용 — decomp\0.5.8\plan_legacy\sub_plan\steal.md) ═══
// ## `0xcbbca0`  —  원본 행 27~28
//
// | | |
// |---|---|
// | RVA | `0xcbbca0` ~ `0xcbbdad` (269 B) |
// | 명령 수 | 70 |
// | 원본 행 | 27, 28 |
//
// **콜리**: `0x31a37c0`×2
//
// **상수**(|v|≥16): `0x20`(32)×4 · `0x140cbbd4b`(5382061387)×4 · `0x1431a37c0`(5420758976)×2 · `0x140cbbce0`(5382061280)×1 · `0x140cbbd9f`(5382061471)×1 · `0x1c8`(456)×1 · `0x140cbbd00`(5382061312)×1 · `0x140cbbd91`(5382061457)×1 · `0x198`(408)×1 · `0x5a`(90)×1 · `0x3e8`(1000)×1 · `0x140cbbd80`(5382061440)×1 · `0x64`(100)×1 · `0xfffffffffffe7961`(-99999)×1
//
// **필드 오프셋**: `+0x8`×6 · `+0x10`×2 · `+0x40`×2 · `+0x58`×1 · `+0x60`×2 · `+0xb1`×1 · `+0x1f0`×1 · `+0x5c0`×2
//
// **vtable 간접호출**: `+0x40`×2 · `+0x1f0`×1
//
// ```c
// // [MANUAL] 손으로 복원함 — Ghidra 디컴 불가
// //   실패 사유: Low-level Error: Backward normalization not implemented
// //   (크기 문제가 아니다. 268 B / 70 명령인데도 디컴파일러가 이 패턴을 처리 못 한다.
// //    타임아웃 상향·payload 상향·프로토타입 명시 모두 시도했고 전부 같은 에러였다.)
// //   근거: 아래 capstone 디스어셈 전문 + 점프테이블 0x1433d76ec 디코드(17아암).
// //   ⚠이 블록은 `// [MANUAL]` 마커 때문에 aifill --force 가 덮어쓰지 않는다.
// //
// // fn @ steal.rs:27   [RVA 0xcbbca0 ~ 0xcbbdad, 268 B]
// //
// // 호출 규약(실측): rcx = steal 상태, [rsp+0x58] = ctx(첫 워드가 world dyn 객체),
// //                  [rsp+0x60] = 평가 대상 SmallAction
// //
// //   world 는 Rust dyn trait 팻 포인터다: [rsi] = data, [rsi+8] = vtable.
// //   vt+0x40  = (뭔지 미상) — 0 이 아니면 패닉하므로 사실상 assert
// //   vt+0x1f0 = 핸들 → 엔티티 조회 (다른 함수에서 확립된 관례)
//
// long long steal_score(StealState *st,          /* rcx */
//                       Ctx        *ctx,          /* [rsp+0x58] */
//                       SmallAction *action)      /* [rsp+0x60] */
// {
//     Dyn *w = ctx->world;                        /* rsi = [rdx] */
//     unsigned phase = st->phase;                 /* [rcx+8] */
//
//     unsigned long long *slot;                   /* 목표 핸들이 들어 있는 슬롯 */
//
//     if (phase == 0) {                           /* 0xcbbce0 */
//         if (w->vt->f40(w->data) != 0) panic();  /* 0xcbbced → 0xcbbd91 */
//         if (ctx->at_0x1a8 == NULL) return 0;    /* 목표 미지정 → 0 */
//         slot = ctx->at_0x1a0;
//     } else if (phase == 2) {                    /* 0xcbbcb8 */
//         return 0;                               /* 도둑질 완료 — 점수 없음 */
//     } else {                                    /* phase == 1: 0xcbbcbe */
//         if (w->vt->f40(w->data) != 0) panic();  /* 0xcbbccb → 0xcbbd9f */
//         if (ctx->at_0x1d8 == NULL) return 0;
//         slot = ctx->at_0x1d0;
//     }
//
//     Entity *tgt = w->vt->get_entity(w->data, *slot);   /* vt+0x1f0, 0xcbbd0f */
//     if (tgt == NULL) return 0;                  /* 목표가 이미 사라짐 → 0 */
//
//     /* 0xcbbd20 ~ 0xcbbd49 — 태그 디스패치.
//        idx = (tag >= 3) ? tag - 3 : 7   ← `sub dl,3; cmovae`
//        ★태그 0·1·2 는 전부 idx 7(=태그 10)과 같은 아암으로 간다. */
//     unsigned idx = ((unsigned char)action->tag >= 3)
//                  ? (unsigned char)(action->tag - 3) : 7;
//
//     switch (idx) {                              /* 점프테이블 0x1433d76ec */
//     case  4:                                    /* 태그 7  */
//     case  7:                                    /* 태그 10 (+ 태그 0,1,2) */
//     case  8:                                    /* 태그 11 */
//     case  9:                                    /* 태그 12 */
//         return 90;                              /* 0xcbbd53 */
//
//     case 11:                                    /* 태그 14 */
//         /* 0xcbbd70 — 액션의 +0x60 이 목표 핸들과 같은가 */
//         return (action->at_0x60 == tgt->handle) ? 100 : -99999;
//
//     case 12:                                    /* 태그 15 */
//     case 13:                                    /* 태그 16 */
//     case 14:                                    /* 태그 17 */
//     case 15:                                    /* 태그 18 */
//         /* 0xcbbd5e — 액션의 +0x08 이 목표 핸들과 같은가 */
//         return (action->at_0x08 == tgt->handle) ? 1000 : -99999;
//
//     default:                                    /* 태그 3,4,5,6,8,9,13,19 */
//         return 0;                               /* 0xcbbd4b */
//     }
// }
//
// /* 점프테이블 0x1433d76ec 전개 (exe 바이트에서 직접 디코드)
//      idx0  태그3  -> 0x140cbbd4b   return 0
//      idx1  태그4  -> 0x140cbbd4b   return 0
//      idx2  태그5  -> 0x140cbbd4b   return 0
//      idx3  태그6  -> 0x140cbbd4b   return 0
//      idx4  태그7  -> 0x140cbbd53   return 90
//      idx5  태그8  -> 0x140cbbd4b   return 0
//      idx6  태그9  -> 0x140cbbd4b   return 0
//      idx7  태그10 -> 0x140cbbd53   return 90     (태그 0,1,2 도 여기로)
//      idx8  태그11 -> 0x140cbbd53   return 90
//      idx9  태그12 -> 0x140cbbd53   return 90
//      idx10 태그13 -> 0x140cbbd4b   return 0
//      idx11 태그14 -> 0x140cbbd70   action[+0x60] == handle ? 100  : -99999
//      idx12 태그15 -> 0x140cbbd5e   action[+0x08] == handle ? 1000 : -99999
//      idx13 태그16 -> 0x140cbbd5e   (동일)
//      idx14 태그17 -> 0x140cbbd5e   (동일)
//      idx15 태그18 -> 0x140cbbd5e   (동일)
//      idx16 태그19 -> 0x140cbbd4b   return 0      ← ★Hold 는 여기서도 0
//    idx17 은 0x140cbcf7e 로 함수 범위 밖 = 테이블 끝(태그 19 까지만 유효). */
//
// /* --- capstone linear disassembly ---
// 140cbbca0  push     rsi
// 140cbbca1  sub      rsp, 0x20
// 140cbbca5  mov      rdx, qword ptr [rsp + 0x58]
// 140cbbcaa  movzx    eax, byte ptr [rcx + 8]
// 140cbbcae  mov      rsi, qword ptr [rdx]
// 140cbbcb1  test     eax, eax
// 140cbbcb3  je       0x140cbbce0
// 140cbbcb5  cmp      eax, 2
// 140cbbcb8  je       0x140cbbd4b
// 140cbbcbe  mov      rcx, qword ptr [rsi]
// 140cbbcc1  mov      rax, qword ptr [rsi + 8]
// 140cbbcc5  call     qword ptr [rax + 0x40]
// 140cbbcc8  test     rax, rax
// 140cbbccb  jne      0x140cbbd9f
// 140cbbcd1  mov      eax, 0x1c8
// 140cbbcd6  cmp      qword ptr [rdx + rax + 0x10], 0
// 140cbbcdc  jne      0x140cbbd00
// 140cbbcde  jmp      0x140cbbd4b
// 140cbbce0  mov      rcx, qword ptr [rsi]
// 140cbbce3  mov      rax, qword ptr [rsi + 8]
// 140cbbce7  call     qword ptr [rax + 0x40]
// 140cbbcea  test     rax, rax
// 140cbbced  jne      0x140cbbd91
// 140cbbcf3  mov      eax, 0x198
// 140cbbcf8  cmp      qword ptr [rdx + rax + 0x10], 0
// 140cbbcfe  je       0x140cbbd4b
// 140cbbd00  mov      rax, qword ptr [rdx + rax + 8]
// 140cbbd05  mov      rcx, qword ptr [rsi]
// 140cbbd08  mov      r8, qword ptr [rsi + 8]
// 140cbbd0c  mov      rdx, qword ptr [rax]
// 140cbbd0f  call     qword ptr [r8 + 0x1f0]
// 140cbbd16  test     rax, rax
// 140cbbd19  je       0x140cbbd4b
// 140cbbd1b  mov      rcx, qword ptr [rsp + 0x60]
// 140cbbd20  movzx    edx, byte ptr [rcx + 0xb1]
// 140cbbd27  sub      dl, 3
// 140cbbd2a  movzx    edx, dl
// 140cbbd2d  mov      r8d, 7
// 140cbbd33  cmovae   r8d, edx
// 140cbbd37  movzx    edx, r8b
// 140cbbd3b  lea      r8, [rip + 0x271b9aa]
// 140cbbd42  movsxd   rdx, dword ptr [r8 + rdx*4]
// 140cbbd46  add      rdx, r8
// 140cbbd49  jmp      rdx
// 140cbbd4b  xor      eax, eax
// 140cbbd4d  add      rsp, 0x20
// 140cbbd51  pop      rsi
// 140cbbd52  ret      
// 140cbbd53  mov      eax, 0x5a
// 140cbbd58  add      rsp, 0x20
// 140cbbd5c  pop      rsi
// 140cbbd5d  ret      
// 140cbbd5e  mov      rcx, qword ptr [rcx + 8]
// 140cbbd62  cmp      rcx, qword ptr [rax + 0x5c0]
// 140cbbd69  mov      ecx, 0x3e8
// 140cbbd6e  jmp      0x140cbbd80
// 140cbbd70  mov      rcx, qword ptr [rcx + 0x60]
// 140cbbd74  cmp      rcx, qword ptr [rax + 0x5c0]
// 140cbbd7b  mov      ecx, 0x64
// 140cbbd80  mov      rax, 0xfffffffffffe7961
// 140cbbd87  cmove    rax, rcx
// 140cbbd8b  add      rsp, 0x20
// 140cbbd8f  pop      rsi
// 140cbbd90  ret      
// 140cbbd91  lea      rcx, [rip + 0x271a2b0]
// 140cbbd98  call     0x1431a37c0
// 140cbbd9d  ud2      
// 140cbbd9f  lea      rcx, [rip + 0x271a2ba]
// 140cbbda6  call     0x1431a37c0
// 140cbbdab  ud2      
// 140cbbdad  int3     
// */
// ```
//
