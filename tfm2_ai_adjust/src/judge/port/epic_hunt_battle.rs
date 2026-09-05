//! epic_hunt_battle — game-ai\src\plan_legacy\old\epic\hunt_and_battle.rs
//!   게임 0.5.8 · RVA 0xccc010 (591B / 143명령) · 원본 행 29,31,32 · 역할 plan_handler
//!   콜리 0xe7a8c0, 0x31a37c0, 0x31a3863, 0x31a3b40 · vtable 슬롯 0x40, 0x1f0
//!   필드 오프셋 0x8×7 0x10×1 0x12×1 0x18×1 0x20×2 0x28×1 0x30×1 0x38×2 0x40×3 0x48×2 0x50×2 0x58×2 0x60×2 0x68×2 0x70×2 0x98×1 0xa0×1 0xf0×1 0xf8×2 0x100×1 0x1a0×1 0x1a8×1 0x1e0×1 0x1f0×1 0x628×2 0x660×1 0x668×1 0x670×2 0x930×1 0x9c0×1 0x12f8×1 0x6d70×1 0x6d80×1
//! 포팅 규약(CLAUDE.md §3): 게임 함수 호출 0 · 메모리 읽기는 judge::world(safe read) 경유 · 반환 None = 판단 불가(가드) → 호출부가 게임 원본으로 passthrough.
//! 스켈레톤 생성 = python MIG\aiport.py skeleton epic_hunt_battle (디컴 원문은 아래 주석). 이 파일은 사람이 채운다 — 재생성은 --force 뿐.
use super::super::{Args8, MpOut};
use super::hunt_battle;

/// 공통 본체 = port\hunt_battle.rs (epic/serpen 은 목표 슬롯·타이머 오프셋·else 코드만 다르다).
/// 반환 None = 가드/콜리 캡처 없음 → NA(검증) / passthrough(live).
pub unsafe fn epic_hunt_battle(a: &Args8) -> Option<MpOut> { hunt_battle::hunt_battle(a, &hunt_battle::EPIC) }

// ═══ 원본 디컴(자동 동봉, 참고용 — decomp\0.5.8\plan_legacy\old\epic\hunt_and_battle.md) ═══
// ## `0xccc010`  —  원본 행 29~32
//
// | | |
// |---|---|
// | RVA | `0xccc010` ~ `0xccc25f` (591 B) |
// | 명령 수 | 143 |
// | 원본 행 | 29, 29, 31, 32 |
//
// **콜리**: `0x31a37c0`×2 · `0xe7a8c0`(lib.rs:1626)×1 · `0x31a3863`×1 · `0x31a3b40`×1
//
// **상수**(|v|≥16): `0x140ccc208`(5382128136)×4 · `0x88`(136)×2 · `0x140ccc155`(5382127957)×2 · `0x140ccc1b9`(5382128057)×2 · `0x140ccc1ff`(5382128127)×2 · `0x1431a37c0`(5420758976)×2 · `0x140ccc23b`(5382128187)×1 · `0x140ccc22d`(5382128173)×1 · `0x140ccc251`(5382128209)×1 · `0x64`(100)×1 · `0x20`(32)×1 · `0x140ccc0ae`(5382127790)×1 · `0x140ccc0b3`(5382127795)×1 · `0x140ccc21f`(5382128159)×1 · `0x140ccc0fc`(5382127868)×1 · `0x140ccc0ff`(5382127871)×1 · `0x6d70`(28016)×1 · `0x140e7a8c0`(5383891136)×1 · `0x33`(51)×1 · `0x1431a3863`(5420759139)×1 · `0x1431a3b40`(5420759872)×1
//
// **필드 오프셋**: `+0x8`×7 · `+0x10`×1 · `+0x12`×1 · `+0x18`×1 · `+0x20`×2 · `+0x28`×1 · `+0x30`×1 · `+0x38`×2 · `+0x40`×3 · `+0x48`×2 · `+0x50`×2 · `+0x58`×2 · `+0x60`×2 · `+0x68`×2 · `+0x70`×2 · `+0x98`×1 · `+0xa0`×1 · `+0xf0`×1 · `+0xf8`×2 · `+0x100`×1 · `+0x1a0`×1 · `+0x1a8`×1 · `+0x1e0`×1 · `+0x1f0`×1 · `+0x628`×2 · `+0x660`×1 · `+0x668`×1 · `+0x670`×2 · `+0x930`×1 · `+0x9c0`×1 · `+0x12f8`×1 · `+0x6d70`×1 · `+0x6d80`×1
//
// **vtable 간접호출**: `+0x40`×1 · `+0x1f0`×1
//
// ```c
// // fn @ hunt_and_battle.rs:?   [RVA 0xccc010]
//
// void FUN_140ccc010(undefined8 *param_1,char *param_2,undefined8 param_3,undefined8 param_4,
//                   longlong param_5,longlong *param_6,longlong param_7)
//
// {
//   ulonglong uVar1;
//   undefined8 *puVar2;
//   longlong lVar3;
//   ulonglong uVar4;
//   ulonglong uVar5;
//   longlong lVar6;
//   longlong lVar7;
//   longlong lVar8;
//   code *pcVar9;
//   bool bVar10;
//   ulonglong uVar11;
//   ulonglong uVar12;
//   longlong lVar13;
//   undefined8 uVar14;
//   longlong lVar15;
//   longlong local_58 [3];
//
//   uVar1 = *(ulonglong *)(param_5 + 0x930);  // 0x930=side
//   if (1 < uVar1) {
//     FUN_1431a3863(uVar1,2,&PTR_s_game_ai_src_plan_legacy_old_epic_1433d67a0);
//                     /* WARNING: Does not return */
//     pcVar9 = (code *)invalidInstructionException();
//     (*pcVar9)();
//   }
//   puVar2 = (undefined8 *)*param_6;
//   lVar3 = puVar2[uVar1 * 5 + (ulonglong)*(uint *)(param_5 + 0x9c0) + 0x3c];  // 0x9c0=role
//   if (lVar3 == 0) {
//     FUN_1431a37c0(&PTR_s_game_ai_src_plan_legacy_old_epic_1433d67b8);
//                     /* WARNING: Does not return */
//     pcVar9 = (code *)invalidInstructionException();
//     (*pcVar9)();
//   }
//   uVar4 = *(ulonglong *)(lVar3 + 0x628);  // 0x628=최대HP
//   if (uVar4 == 0) {
//     FUN_1431a3b40(&PTR_s_game_ai_src_plan_legacy_old_epic_1433d67d0);
//                     /* WARNING: Does not return */
//     pcVar9 = (code *)invalidInstructionException();
//     (*pcVar9)();
//   }
//   uVar5 = *(ulonglong *)(lVar3 + 0x670);  // 0x670=현재HP
//   uVar11 = uVar5 * 100;
//   if ((uVar11 | uVar4) >> 0x20 == 0) {
//     uVar12 = (uVar11 & 0xffffffff) / (uVar4 & 0xffffffff);
//     uVar11 = (uVar11 & 0xffffffff) % (uVar4 & 0xffffffff);
//   }
//   else {
//     uVar12 = uVar11 / uVar4;
//     uVar11 = uVar11 % uVar4;
//   }
//   uVar14 = *puVar2;
//   lVar6 = puVar2[1];
//   lVar13 = (**(code **)(lVar6 + 0x40))(uVar14);
//   if (lVar13 == 0) {
//     if (*(longlong *)(uVar11 + 0x1a8) == 0) {
//       lVar13 = 0;
//     }
//     else {
//       lVar13 = (**(code **)(lVar6 + 0x1f0))(uVar14,**(undefined8 **)(uVar11 + 0x1a0));
//     }
//     lVar7 = param_6[1];
//     lVar8 = *(longlong *)(lVar7 + 0x20);
//     lVar15 = uVar1 * 0x20;
//     bVar10 = false;
//     if ((*(ulonglong *)(lVar8 + 0x6d70 + lVar15) <= *(ulonglong *)(lVar3 + 0x660)) &&  // 0x660=x
//        (bVar10 = false, *(ulonglong *)(lVar3 + 0x660) <= *(ulonglong *)(lVar8 + 0x6d80 + lVar15))) {  // 0x660=x
//       bVar10 = *(ulonglong *)(lVar3 + 0x668) <= *(ulonglong *)(lVar8 + lVar15 + 0x6d88) &&  // 0x668=y
//                *(ulonglong *)(lVar8 + lVar15 + 0x6d78) <= *(ulonglong *)(lVar3 + 0x668);  // 0x668=y
//     }
//     FUN_140e7a8c0(local_58,param_3,param_4,param_5,uVar14,lVar6,lVar7);
//     if (((lVar13 == 0) || (*(longlong *)(lVar13 + 0x670) != *(longlong *)(lVar13 + 0x628))) ||  // 0x670=현재HP · 0x628=최대HP
//        ((uVar14 = 5, !(bool)(uVar5 < uVar4 & bVar10) && ((0x32 < uVar12 && (local_58[0] == 0)))))) {
//       if ((*(ulonglong *)(param_7 + 0xa0) <
//            (ulonglong)
//            (*(longlong *)(*(longlong *)(lVar7 + 8) + 0x12f8) + *(longlong *)(param_7 + 0x98))) &&  // 0x12f8=tick/sec
//          (*param_2 != '\0')) {
//         param_1[1] = *(undefined8 *)(param_2 + 8);
//         *(undefined2 *)(param_1 + 2) = 1;
//         *(undefined1 *)((longlong)param_1 + 0x12) = 0;
//         uVar14 = 9;
//       }
//       else {
//         *(undefined1 *)(param_1 + 1) = 0;
//         uVar14 = 0xb;
//       }
//     }
//     *param_1 = uVar14;
//     return;
//   }
//   FUN_1431a37c0(&PTR_s_game_ai_src_plan_legacy_old_epic_1433d67e8);
//                     /* WARNING: Does not return */
//   pcVar9 = (code *)invalidInstructionException();
//   (*pcVar9)();
// }
//
// ```
//
