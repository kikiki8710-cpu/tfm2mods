//! passive_jungle — game-ai\src\plan_legacy\old\passive_jungle.rs
//!   게임 0.5.8 · RVA 0xd2e500 (2993B / 699명령) · 원본 행 143,154,180,192,194,205 · 역할 plan_handler
//!   콜리 0xffa3e0, 0x12857f0, 0x2b1b410, 0x31a01a3, 0x31a0cbf, 0x31a37c0, 0x31a3863, 0x31a3b40 · vtable 슬롯 0x40, 0x90, 0xa0, 0x118
//!   필드 오프셋 0x8×10 0x10×5 0x11×1 0x18×1 0x20×8 0x28×5 0x30×1 0x40×3 0x48×1 0x60×1 0x68×1 0x80×1 0x88×1 0x90×3 0xa0×2 0xc8×1 0xd0×1 0xd8×1 0xe0×2 0xe8×2 0xf0×2 0xf8×3 0x100×4 0x108×5 0x110×6 0x118×6 0x120×8 0x128×10 0x130×13 0x13e×2 0x13f×5 0x140×1 0x1b0×1 0x1b8×1 0x1e0×1 0x1f0×2 0x3f0×1 0x3fc×2 0x4c0×2 0x4c8×2 0x4d0×2 0x4f8×1 0x500×1 0x570×2 0x578×2 0x5c0×1 0x5c8×1 0x628×4 0x660×1 0x668×2 0x670×6 0x930×1 0x9c0×1 0x12f8×4 0x6d70×1 0x6d80×1
//! 포팅 규약(CLAUDE.md §3): 게임 함수 호출 0 · 메모리 읽기는 judge::world(safe read) 경유 · 반환 None = 판단 불가(가드) → 호출부가 게임 원본으로 passthrough.
//! 스켈레톤 생성 = python MIG\aiport.py skeleton passive_jungle (디컴 원문은 아래 주석). 이 파일은 사람이 채운다 — 재생성은 --force 뿐.
use crate::*;
use super::super::world::*;
use super::super::ScorerArgs;

/// 반환: Some(점수) / None = 재현 불가 경로(가드) → passthrough
pub unsafe fn passive_jungle(a: &ScorerArgs) -> Option<i64> {
    let _ = a;
    None   // TODO: 포팅
}

// ═══ 원본 디컴(자동 동봉, 참고용 — decomp\0.5.8\plan_legacy\old\passive_jungle.md) ═══
// ## `0xd2e500`  —  원본 행 143~205
//
// | | |
// |---|---|
// | RVA | `0xd2e500` ~ `0xd2f0b1` (2993 B) |
// | 명령 수 | 699 |
// | 원본 행 | 143, 143, 154, 180, 192, 194, 205 |
//
// **콜리**: `0x31a37c0`×5 · `0x31a0cbf`×4 · `0x2b1b410`×3 · `0x31a01a3`×3 · `0x12857f0`×2 · `0xffa3e0`×1 · `0x31a3863`×1 · `0x31a3b40`×1
//
// **상수**(|v|≥16): `0x20`(32)×7 · `0x140d2e682`(5382530690)×5 · `0x64`(100)×5 · `0x140d2eb9f`(5382531999)×5 · `0x140d2e992`(5382531474)×5 · `0x140d2ee07`(5382532615)×5 · `0x1431a37c0`(5420758976)×5 · `0x140d2e5d1`(5382530513)×4 · `0x140d2ed85`(5382532485)×4 · `0x1431a0cbf`(5420747967)×4 · `0x140d2e959`(5382531417)×4 · `0x3e8`(1000)×4 · `0xfffffffffffffff0`(-16)×4 · `0x10`(16)×4 · `0x140d2ec50`(5382532176)×4 · `0xc0`(192)×3 · `0x180`(384)×3 · `0x120`(288)×3 · `0x60`(96)×3 · `0x150`(336)×3 · `0x90`(144)×3 · `0xf0`(240)×3 · `0x30`(48)×3 · `0x1b0`(432)×3 · `0x3d`(61)×3 · `0x7ffffffffffffff8`(9223372036854775800)×3 · `0x142b1b410`(5413909520)×3 · `0x1431a01a3`(5420745123)×3 · `0x140d2ed73`(5382532467)×3 · `0x140d2ecef`(5382532335)×3 · `0x1c8`(456)×2 · `0x140d2f085`(5382533253)×2 · `0x490`(1168)×2 · `0x1412857f0`(5388130288)×2 · `0x140d2e89c`(5382531228)×2 · `0x140d2e7f2`(5382531058)×2 · `0x140d2e910`(5382531344)×2 · `0x140d2ea8b`(5382531723)×2 · `0x140d2ef5a`(5382532954)×2 · `0x140d2f072`(5382533234)×1
//
// **필드 오프셋**: `+0x-58`×7 · `+0x-50`×4 · `+0x-48`×3 · `+0x-40`×1 · `+0x-10`×2 · `+0x8`×10 · `+0x10`×5 · `+0x11`×1 · `+0x18`×1 · `+0x20`×8 · `+0x28`×5 · `+0x30`×1 · `+0x40`×3 · `+0x48`×1 · `+0x60`×1 · `+0x68`×1 · `+0x80`×1 · `+0x88`×1 · `+0x90`×3 · `+0xa0`×2 · `+0xc8`×1 · `+0xd0`×1 · `+0xd8`×1 · `+0xe0`×2 · `+0xe8`×2 · `+0xf0`×2 · `+0xf8`×3 · `+0x100`×4 · `+0x108`×5 · `+0x110`×6 · `+0x118`×6 · `+0x120`×8 · `+0x128`×10 · `+0x130`×13 · `+0x13e`×2 · `+0x13f`×5 · `+0x140`×1 · `+0x1b0`×1 · `+0x1b8`×1 · `+0x1e0`×1 · `+0x1f0`×2 · `+0x3f0`×1 · `+0x3fc`×2 · `+0x4c0`×2 · `+0x4c8`×2 · `+0x4d0`×2 · `+0x4f8`×1 · `+0x500`×1 · `+0x570`×2 · `+0x578`×2 · `+0x5c0`×1 · `+0x5c8`×1 · `+0x628`×4 · `+0x660`×1 · `+0x668`×2 · `+0x670`×6 · `+0x930`×1 · `+0x9c0`×1 · `+0x12f8`×4 · `+0x6d70`×1
//
// **vtable 간접호출**: `+0x40`×2 · `+0x90`×2 · `+0xa0`×2 · `+0x118`×2
//
// ```c
// // fn @ passive_jungle.rs:?   [RVA 0xd2e500]
//
// /* WARNING: Control flow encountered bad instruction data */
// /* WARNING: Instruction at (ram,0x000140d3c6a5) overlaps instruction at (ram,0x000140d3c6a4)
//     */
// /* WARNING: Removing unreachable block (ram,0x000140d3bddb) */
// /* WARNING: Removing unreachable block (ram,0x000140d39a9a) */
// /* WARNING: Type propagation algorithm not settling */
// /* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
//
// ulonglong *****
// FUN_140d2e500(ulonglong *****param_1,ulonglong ******param_2,undefined8 param_3,undefined8 param_4,
//              longlong param_5,longlong *param_6)
//
// {
//   undefined4 uVar1;
//   undefined8 *puVar2;
//   ulonglong *****pppppuVar3;
//   void *pvVar4;
//   undefined4 *puVar5;
//   undefined8 uVar6;
//   undefined8 uVar7;
//   undefined8 uVar8;
//   ulonglong **ppuVar9;
//   ulonglong ****ppppuVar10;
//   ulonglong ***pppuVar11;
//   byte *pbVar12;
//   ulonglong uVar13;
//   undefined1 auVar14 [16];
//   undefined1 auVar15 [16];
//   code *pcVar16;
//   uint uVar17;
//   uint uVar18;
//   undefined1 (*pauVar19) [16];
//   char cVar20;
//   longlong lVar21;
//   uint *puVar22;
//   ulonglong ******ppppppuVar23;
//   HANDLE pvVar24;
//   longlong lVar25;
//   longlong *plVar26;
//   longlong lVar27;
//   ulonglong uVar28;
//   longlong *plVar29;
//   longlong lVar30;
//   ulonglong ***pppuVar31;
//   byte *pbVar32;
//   ulonglong *****pppppuVar33;
//   ulonglong uVar34;
//   ulonglong uVar35;
//   undefined8 uVar36;
//   longlong lVar37;
//   ulonglong *****pppppuVar38;
//   byte bVar39;
//   int iVar40;
//   uint uVar41;
//   ulonglong ****ppppuVar42;
//   ulonglong ******ppppppuVar43;
//   longlong lVar44;
//   longlong lVar45;
//   byte bVar49;
//   longlong unaff_RBX;
//   uint *puVar46;
//   longlong *plVar47;
//   ulonglong uVar48;
//   byte **ppbVar50;
//   ulonglong ******ppppppuVar51;
//   ulonglong ******ppppppuVar52;
//   ulonglong *****pppppuVar53;
//   ulonglong ******ppppppuVar54;
//   ulonglong ****unaff_RBP;
//   ulonglong ******unaff_RSI;
//   ulonglong ******ppppppuVar55;
//   ulonglong ******unaff_RDI;
//   ulonglong ******ppppppuVar56;
//   ulonglong uVar57;
//   ulonglong *puVar58;
//   char *pcVar59;
//   ulonglong ******ppppppuVar60;
//   ulonglong *****pppppuVar61;
//   undefined8 *puVar62;
//   ulonglong ******in_R11;
//   longlong lVar63;
//   ulonglong ****unaff_R12;
//   undefined1 (*pauVar64) [16];
//   ulonglong uVar65;
//   ulonglong ******unaff_R13;
//   undefined1 *puVar66;
//   longlong lVar67;
//   ulonglong ****ppppuVar68;
//   int iVar69;
//   undefined8 *unaff_R14;
//   ulonglong ******ppppppuVar70;
//   ulonglong *****unaff_R15;
//   bool bVar71;
//   undefined4 unaff_XMM6_Da;
//   undefined4 unaff_XMM6_Db;
//   undefined4 unaff_XMM6_Dc;
//   undefined4 unaff_XMM6_Dd;
//   ulonglong *****unaff_retaddr;
//   ulonglong ******ppppppuStackX_8;
//   byte bStackX_17;
//   longlong in_stack_00001530;
//   undefined1 (*in_stack_00001538) [16];
//   undefined1 (*in_stack_00001540) [16];
//   ushort in_stack_00001548;
//   longlong in_stack_00001550;
//   undefined1 *in_stack_000015d0;
//   ulonglong *****in_stack_000015d8;
//   longlong in_stack_000015e8;
//   longlong in_stack_000015f0;
//   longlong in_stack_000015f8;
//   ulonglong in_stack_00001600;
//   longlong in_stack_00001608;
//   longlong in_stack_00001610;
//   undefined8 in_stack_00001618;
//   undefined8 in_stack_00001628;
//   ulonglong in_stack_00001630;
//   longlong *in_stack_00001638;
//   ulonglong in_stack_000016c8;
//   ulonglong ******ppppppuStack_210;
//   ulonglong *****pppppuStack_208;
//   byte *apbStack_200 [3];
//   ulonglong ****ppppuStack_1e8;
//   ulonglong ******ppppppuStack_1e0;
//   ulonglong ******ppppppuStack_1d8;
//   ulonglong ******ppppppuStack_1d0;
//   byte *pbStack_1c8;
//   ulonglong ******ppppppuStack_1c0;
//   ulonglong uStack_1b8;
//   ulonglong ******ppppppuStack_1b0;
//   byte *pbStack_1a8;
//   ulonglong ******ppppppuStack_1a0;
//   int iStack_198;
//   undefined4 uStack_194;
//   ulonglong *****pppppuStack_190;
//   ulonglong ******ppppppuStack_188;
//   byte *pbStack_180;
//   ulonglong uStack_178;
//   ulonglong *puStack_170;
//   ulonglong uStack_168;
//   ulonglong ****ppppuStack_160;
//   ulonglong ***pppuStack_158;
//   undefined4 uStack_150;
//   undefined4 uStack_14c;
//   undefined8 uStack_148;
//   longlong lStack_140;
//   ulonglong *puStack_138;
//   ulonglong ****ppppuStack_130;
//   ulonglong auStack_128 [13];
//   ulonglong uStack_c0;
//   undefined1 *puStack_b8;
//   ulonglong uStack_b0;
//   ulonglong *****pppppuStack_a8;
//   ulonglong ******ppppppuStack_a0;
//   ulonglong uStack_98;
//   longlong lStack_90;
//   ulonglong *****pppppuStack_88;
//   ulonglong ******ppppppuStack_80;
//   undefined8 uStack_78;
//   code *pcStack_70;
//   ulonglong *****pppppuStack_68;
//   ulonglong ******ppppppuStack_60;
//   ulonglong ****ppppuStack_58;
//   undefined6 uStack_50;
//   byte bStack_4a;
//   byte bStack_49;
//   ulonglong ****ppppuStack_48;
//
//   pppppuVar33 = (ulonglong *****)&pppppuStack_208;
//   ppppppuVar51 = &pppppuStack_208;
//   pppppuVar61 = (ulonglong *****)&pppppuStack_208;
//   ppppppuVar52 = &pppppuStack_208;
//   ppppppuVar54 = &pppppuStack_208;
//   pppppuVar53 = (ulonglong *****)&pppppuStack_208;
//   pppppuVar38 = (ulonglong *****)&pppppuStack_208;
//   ppppppuVar23 = &pppppuStack_208;
//   ppppppuVar60 = (ulonglong ******)&ppppppuStack_188;
//   ppppuStack_48 = (ulonglong ****)0xfffffffffffffffe;
//   uVar35 = *(ulonglong *)(param_5 + 0x930);  // 0x930=side
//   if (1 < uVar35) {
//     ppppppuStack_210 = (ulonglong ******)0x140d2f083;
//     FUN_1431a3863(uVar35,2,&PTR_s_game_ai_src_plan_legacy_old_pass_1433d9040);
//                     /* WARNING: Does not return */
//     pcVar16 = (code *)invalidInstructionException();
//     (*pcVar16)();
//   }
//   puVar2 = (undefined8 *)*param_6;
//   uVar34 = (ulonglong)*(uint *)(param_5 + 0x9c0);  // 0x9c0=role
//   puVar62 = puVar2 + uVar35 * 5;
//   ppppuVar68 = (ulonglong ****)puVar62[uVar34 + 0x3c];
//   if (ppppuVar68 == (ulonglong ****)0x0) {
//     ppppppuStack_210 = (ulonglong ******)0x140d2f053;
//     FUN_1431a37c0(&PTR_s_game_ai_src_plan_legacy_old_pass_1433d9058);
//                     /* WARNING: Does not return */
//     pcVar16 = (code *)invalidInstructionException();
//     (*pcVar16)();
//   }
//   ppppppuVar56 = (ulonglong ******)param_2[9];
//   bVar39 = *(byte *)(param_2 + 0xc);
//   uVar65 = (ulonglong)bVar39;
//   pppppuVar3 = (ulonglong *****)param_6[1];
//   puVar46 = (uint *)pppppuVar3[4];
//   ppppppuVar55 = (ulonglong ******)ppppuVar68[0xcc];
//   if ((((*(ulonglong *******)((longlong)puVar46 + (uVar35 * 4 + 0xdae) * 8) <= ppppppuVar55) &&
//        (ppppppuVar55 <= *(ulonglong *******)((longlong)puVar46 + (uVar35 * 4 + 0xdb0) * 8))) &&
//       (param_2 = (ulonglong ******)ppppuVar68[0xcd],
//       *(ulonglong *******)((longlong)puVar46 + (uVar35 * 4 + 0xdaf) * 8) <= param_2)) &&
//      (param_2 <= *(ulonglong *******)((longlong)puVar46 + (uVar35 * 4 + 0xdb1) * 8))) {
//     param_2 = (ulonglong ******)ppppuVar68[0xce];
//     ppppuVar42 = (ulonglong ****)0x5;
//     puVar22 = (uint *)param_1;
//     if (ppppuVar68[0xc5] <= param_2) goto LAB_140d2e5d1;
//     goto LAB_140d2ed85;
//   }
// LAB_140d2e5d1:
//   uStack_78 = *puVar2;
//   lStack_90 = puVar2[1];
//   pcStack_70 = *(code **)(lStack_90 + 0x40);
//   ppppppuStack_210 = (ulonglong ******)0x140d2e5f3;
//   lVar21 = (*pcStack_70)();
//   if (lVar21 != 0) {
//     ppppppuStack_210 = (ulonglong ******)0x140d2f037;
//     FUN_1431a37c0(&PTR_s_game_ai_src_plan_legacy_old_pass_1433d90b8);
//                     /* WARNING: Does not return */
//     pcVar16 = (code *)invalidInstructionException();
//     (*pcVar16)();
//   }
//   pppppuStack_68 = pppppuVar3;
//   ppppppuVar70 = (ulonglong ******)(ulonglong)bVar39;
//   puVar22 = &switchD_140d2e62d::caseD_4c;
//   ppppppuStack_80 = ppppppuVar70;
//   ppppppuVar43 = (ulonglong ******)
//                  ((longlong)&switchD_140d2e62d::caseD_4c +
//                  (longlong)(int)(&switchD_140d2e62d::caseD_4c)[(longlong)ppppppuVar70]);
//   bVar71 = ppppppuVar43 == (ulonglong ******)0x0;
//   ppppppuStack_60 = ppppppuVar56;
//   bStack_49 = bVar39;
//   cVar20 = (char)ppppppuVar43;
//   bVar49 = (byte)((ulonglong)puVar46 >> 8);
//   pppppuVar3 = (ulonglong *****)&pppppuStack_208;
//                     /* WARNING (jumptable): Sanity check requires truncation of jumptable */
//                     /* WARNING: Could not find normalized switch variable to match jumptable */
//   switch(bVar39) {
//   case 0:
//     lVar21 = 0xc0;
//     if (ppppppuVar56 == (ulonglong ******)0x0) {
//       lVar21 = 0;
//     }
//     break;
//   case 1:
//     lVar21 = 0x30;
//     if (ppppppuVar56 != (ulonglong ******)0x0) {
//       lVar21 = 0xf0;
//     }
//     break;
//   case 2:
//     lVar21 = 0x60;
//     if (ppppppuVar56 != (ulonglong ******)0x0) {
//       lVar21 = 0x120;
//     }
//     break;
//   case 3:
//     lVar21 = 0x90;
//     if (ppppppuVar56 != (ulonglong ******)0x0) {
//       lVar21 = 0x150;
//     }
//     break;
//   case 4:
//     lVar21 = 0x180;
//     break;
//   case 5:
//     lVar21 = 0x1b0;
//     break;
//   case 6:
//     UNK_1433da069 = UNK_1433da069 + cVar20;
//     puVar46 = (uint *)0x45a85821;
//     goto code_r0x000140d2e9af;
//   case 7:
//     lVar21 = 0x90;
//     if (ppppppuVar56 != (ulonglong ******)0x0) {
//       lVar21 = 0x150;
//     }
//     goto code_r0x000140d2eb9f;
//   case 8:
//     goto switchD_140d2e62d_caseD_8;
//   case 9:
//     lVar21 = 0x60;
//     if (ppppppuVar56 != (ulonglong ******)0x0) {
//       lVar21 = 0x120;
//     }
//     goto code_r0x000140d2eb9f;
//   case 10:
//     *(byte *)((longlong)puVar46 + 0xffff07d) = *(byte *)((longlong)puVar46 + 0xffff07d) - 0x24;
//     goto switchD_140d2e62d_caseD_8;
//   case 0xb:
//     lVar21 = 0x30;
//     if (ppppppuVar56 != (ulonglong ******)0x0) {
//       lVar21 = 0xf0;
//     }
// code_r0x000140d2eb9f:
//     ppppppuVar52 = *(ulonglong *******)((longlong)param_2 + lVar21 + 0x28);
//     uVar35 = (longlong)ppppppuVar52 * 8;
//     bVar71 = (ulonglong)ppppppuVar52 >> 0x3d != 0;
//     if (0x7ffffffffffffff8 < uVar35 || bVar71) {
//       ppppppuStack_210 = (ulonglong ******)0x140d2ebd6;
//       FUN_1431a0cbf(0,uVar35);
//                     /* WARNING: Does not return */
//       pcVar16 = (code *)invalidInstructionException();
//       (*pcVar16)();
//     }
//     pvVar4 = *(void **)((longlong)param_2 + lVar21 + 0x20);
//     if (uVar35 == 0) {
//       ppppppuVar23 = (ulonglong ******)&DAT_00000008;
//       ppppppuVar56 = (ulonglong ******)0x0;
//     }
//     else {
//       ppppppuStack_210 = (ulonglong ******)0x140d2ebec;
//       ppppppuVar23 = FUN_142b1b410((void *)(ulonglong)bVar71,0,uVar35);
//       ppppppuVar56 = ppppppuVar52;
//       if (ppppppuVar23 == (ulonglong ******)0x0) {
//         ppppppuStack_210 = (ulonglong ******)0x140d2f0af;
//         FUN_1431a0cbf(8,uVar35);
//                     /* WARNING: Does not return */
//         pcVar16 = (code *)invalidInstructionException();
//         (*pcVar16)();
//       }
//     }
//     ppppppuStack_1e0 = ppppppuVar56;
//     ppppppuStack_1d8 = ppppppuVar23;
//     if (ppppppuVar52 != (ulonglong ******)0x0) {
//       ppppppuStack_210 = (ulonglong ******)0x140d2ec34;
//       memcpy(ppppppuVar23,pvVar4,uVar35);
//       pcVar16 = *(code **)(lStack_90 + 0x1f0);
//       lVar21 = 0;
//       ppppppuStack_1d0 = ppppppuVar52;
//       do {
//         ppppppuStack_210 = (ulonglong ******)0x140d2ec6a;
//         lVar37 = (*pcVar16)(uStack_78,*(undefined8 *)((longlong)ppppppuVar23 + lVar21));
//         if ((((lVar37 != 0) && (*(int *)(lVar37 + 0x68) == 4)) && (*(int *)(lVar37 + 0x88) == 1)) &&
//            (*(ulonglong ****)(lVar37 + 0x90) == ppppuStack_58[0xb8])) {  // 0xb8=SmallAction stride
//           ppppppuStack_210 = (ulonglong ******)0x140d2ec9c;
//           pvVar24 = GetProcessHeap();
//           ppppppuStack_210 = (ulonglong ******)0x140d2ecaa;
//           HeapFree(pvVar24,0,ppppppuVar23);
//           ppppppuStack_210 = (ulonglong ******)0x140d2ecb7;
//           pcVar59 = (char *)(*pcStack_70)(uStack_78);
//           goto code_r0x000140d2ecb7;
//         }
//         lVar21 = lVar21 + 8;
//       } while (uVar35 - lVar21 != 0);
//     }
//     if (ppppppuVar56 != (ulonglong ******)0x0) {
//       ppppppuStack_210 = (ulonglong ******)0x140d2ecfa;
//       pvVar24 = GetProcessHeap();
//       ppppppuStack_210 = (ulonglong ******)0x140d2ed08;
//       HeapFree(pvVar24,0,ppppppuVar23);
//     }
//     pppuVar11 = ppppuStack_58[0xc5];
//     if (pppuVar11 == (ulonglong ***)0x0) {
//       ppppppuStack_210 = (ulonglong ******)0x140d2f0a0;
//       FUN_1431a3b40(&PTR_s_game_ai_src_plan_legacy_old_pass_1433d9070);
//                     /* WARNING: Does not return */
//       pcVar16 = (code *)invalidInstructionException();
//       (*pcVar16)();
//     }
//     uVar35 = (longlong)ppppuStack_58[0xce] * 100;
//     puVar22 = (uint *)pppppuStack_88;
//     if ((uVar35 | (ulonglong)pppuVar11) >> 0x20 == 0) {
//       uVar35 = (uVar35 & 0xffffffff) / ((ulonglong)pppuVar11 & 0xffffffff);
//       if ((char)ppppuVar68 != '\0') goto code_r0x000140d2ed57;
// code_r0x000140d2ed6d:
//       ppppuVar42 = (ulonglong ****)0x5;
//       if (uVar35 < 0x29) goto LAB_140d2ed85;
//       goto code_r0x000140d2ed73;
//     }
//     uVar35 = uVar35 / (ulonglong)pppuVar11;
//     if ((char)ppppuVar68 == '\0') goto code_r0x000140d2ed6d;
// code_r0x000140d2ed57:
//     ppppuVar42 = (ulonglong ****)0x5;
//     if (0x14 < uVar35) goto code_r0x000140d2ed73;
//     goto LAB_140d2ed85;
//   case 0xc:
//     iVar69 = CONCAT13(switchD_140d2e62d::caseD_4c._3_1_,
//                       CONCAT12(switchD_140d2e62d::caseD_4c._2_1_,
//                                CONCAT11(switchD_140d2e62d::caseD_4c._1_1_,
//                                         (char)switchD_140d2e62d::caseD_4c))) + 0x433da0dc;
//     switchD_140d2e62d::caseD_4c._0_1_ = (char)iVar69;
//     switchD_140d2e62d::caseD_4c._1_1_ = (char)((uint)iVar69 >> 8);
//     switchD_140d2e62d::caseD_4c._2_1_ = (undefined1)((uint)iVar69 >> 0x10);
//     switchD_140d2e62d::caseD_4c._3_1_ = (char)((uint)iVar69 >> 0x18);
//     pcVar59 = (char *)((ulonglong)ppppppuVar60 & 0xffffffff);
//     ppppppuVar60 = (ulonglong ******)0x433da0dc;
//     *(byte *)ppppppuVar43 = (*(byte *)ppppppuVar43 - (char)pcVar59) - CARRY1(bVar49,bVar49);
//     *pcVar59 = *pcVar59 + (char)pcVar59;
// code_r0x000140d2ecb7:
//     if (pcVar59 != (char *)0x0) {
//       ppppppuStack_210 = (ulonglong ******)0x140d2f070;
//       FUN_1431a37c0(&PTR_s_game_ai_src_plan_legacy_old_pass_1433d9088);
//                     /* WARNING: Does not return */
//       pcVar16 = (code *)invalidInstructionException();
//       (*pcVar16)();
//     }
//                     /* WARNING: Could not recover jumptable at 0x000140d2ecdc. Too many branches */
//                     /* WARNING: Treating indirect jump as call */
//     pppppuVar33 = (ulonglong *****)
//                   (*(code *)((longlong)&UINT_1433da10c +
//                             (longlong)(int)(&UINT_1433da10c)[(longlong)ppppppuVar60[0x21]]))
//                             ((code *)((longlong)&UINT_1433da10c +
//                                      (longlong)(int)(&UINT_1433da10c)[(longlong)ppppppuVar60[0x21]])
//                             );
//     return pppppuVar33;
//   case 0xd:
//                     /* WARNING: Bad instruction - Truncating control flow here */
//     halt_baddata();
//   case 0xe:
//     return (ulonglong *****)puVar22;
//   case 0xf:
//     *(byte *)ppppppuVar56 = *(byte *)ppppppuVar56 + cVar20;
//     LOCK();
//     *(int *)((longlong)puVar46 + -5) = (int)puVar46;
//     UNLOCK();
//                     /* WARNING: Bad instruction - Truncating control flow here */
//     halt_baddata();
//   case 0x10:
//     goto switchD_140d2e62d_caseD_10;
//   case 0x11:
//                     /* WARNING: Bad instruction - Truncating control flow here */
//     halt_baddata();
//   case 0x12:
//     ppbVar50 = apbStack_200;
//     pppppuVar33 = (ulonglong *****)apbStack_200;
//     pppppuStack_208 = (ulonglong *****)0x140d322c2;
//     uVar35 = FUN_1412a07d0();
//     ppppppuVar56 = (ulonglong ******)
//                    ((longlong)ppppppuVar56 + (ulonglong)(ppppppuVar56 == (ulonglong ******)0x0));
//     if ((uVar35 | (ulonglong)ppppppuVar56) >> 0x20 != 0) {
//       uVar35 = uVar35 % (ulonglong)ppppppuVar56;
//       goto LAB_140d322e5;
//     }
//     goto LAB_140d322e1;
//   case 0x13:
//     switchD_140d2e62d::caseD_4c._0_1_ = (char)switchD_140d2e62d::caseD_4c + -0x24;
//     UNK_1433da067 = UNK_1433da067 + cVar20;
//     ppppppuVar51 = (ulonglong ******)&ppppppuStack_210;
//     puVar22 = (uint *)&UNK_1433da050;
//     ppppppuStack_210 = &pppppuStack_208;
//   case 0x15:
//     *(uint *)((longlong)ppppppuVar51 + 0x28) = (uint)ppppppuVar43;
//     *(uint **)((longlong)ppppppuVar51 + 0x20) = puVar22;
//     *(undefined1 *)((longlong)ppppppuVar51 + 0x30) = 0xc;
//     *(undefined8 *)((longlong)ppppppuVar51 + -8) = 0x140d323d8;
//     FUN_140d84db0((undefined1 *)((longlong)ppppppuVar51 + 0xb0),
//                   *(undefined8 *)((longlong)ppppppuVar51 + 0x50),ppppppuVar70,
//                   *(undefined8 *)((longlong)ppppppuVar51 + 0x1c0));
//     uVar35 = *(ulonglong *)((longlong)ppppppuVar51 + 0x110);
//     lVar21 = *(longlong *)((longlong)ppppppuVar51 + 0xc0);
//     lVar37 = 0;
//     *(byte **)((longlong)ppppppuVar51 + 0x78) = (byte *)((longlong)ppppppuVar56 + -2);
//     *(undefined8 *)((longlong)ppppppuVar51 + 0x58) = *(undefined8 *)((longlong)ppppppuVar51 + 0xb0);
//     if ((((longlong)ppppppuVar56 + -2 < 0x1e) && ((longlong)uVar35 < 0x1e)) &&
//        (-1 < (longlong)((ulonglong)((longlong)ppppppuVar56 - 2U) | uVar35))) {
//       *(longlong *)((longlong)ppppppuVar51 + 0x28) =
//            (longlong)((longlong)ppppppuVar56 + -2) * 32000 + 16000;
//       *(ulonglong *)((longlong)ppppppuVar51 + 0x20) = uVar35 * 32000 + 16000;
//       *(undefined1 *)((longlong)ppppppuVar51 + 0x30) = 0xc;
//       *(undefined8 *)((longlong)ppppppuVar51 + -8) = 0x140d3294c;
//       FUN_140d84db0((undefined1 *)((longlong)ppppppuVar51 + 0xb0),
//                     *(undefined8 *)((longlong)ppppppuVar51 + 0x50),ppppppuVar70,
//                     *(undefined8 *)((longlong)ppppppuVar51 + 0x1c0));
//       uVar35 = *(ulonglong *)((longlong)ppppppuVar51 + 0x110);
//       puVar46 = *(uint **)((longlong)ppppppuVar51 + 0xb0);
//       lVar37 = *(longlong *)((longlong)ppppppuVar51 + 0xc0);
//     }
//     lVar30 = 0;
//     lVar27 = 9999;
//     *(byte **)((longlong)ppppppuVar51 + 0x88) = (byte *)((longlong)ppppppuVar56 + -1);
//     *(ulonglong *****)((longlong)ppppppuVar51 + 0x80) = ppppuVar68;
//     *(uint **)((longlong)ppppppuVar51 + 0x60) = puVar46;
//     if (((longlong)ppppppuVar56 + -1 < 0x1e) && ((longlong)uVar35 < 0x1e)) {
//       lVar44 = 9999;
//       if (-1 < (longlong)((ulonglong)((longlong)ppppppuVar56 - 1U) | uVar35)) {
//         *(longlong *)((longlong)ppppppuVar51 + 0x28) =
//              (longlong)((longlong)ppppppuVar56 + -1) * 32000 + 16000;
//         *(ulonglong *)((longlong)ppppppuVar51 + 0x20) = uVar35 * 32000 + 16000;
//         *(undefined1 *)((longlong)ppppppuVar51 + 0x30) = 0xc;
//         *(undefined8 *)((longlong)ppppppuVar51 + -8) = 0x140d329eb;
//         FUN_140d84db0((undefined1 *)((longlong)ppppppuVar51 + 0xb0),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x50),ppppppuVar70,
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x1c0));
//         uVar35 = *(ulonglong *)((longlong)ppppppuVar51 + 0x110);
//         lVar44 = *(longlong *)((longlong)ppppppuVar51 + 0xb0);
//         lVar30 = *(longlong *)((longlong)ppppppuVar51 + 0xc0);
//       }
//     }
//     else {
//       lVar44 = 9999;
//     }
//     lVar67 = 0;
//     if ((((longlong)ppppppuVar56 < 0x1e) && ((longlong)uVar35 < 0x1e)) &&
//        (-1 < (longlong)((ulonglong)ppppppuVar56 | uVar35))) {
//       *(longlong *)((longlong)ppppppuVar51 + 0x28) = (longlong)ppppppuVar56 * 32000 + 16000;
//       *(ulonglong *)((longlong)ppppppuVar51 + 0x20) = uVar35 * 32000 + 16000;
//       *(undefined1 *)((longlong)ppppppuVar51 + 0x30) = 0xc;
//       *(undefined8 *)((longlong)ppppppuVar51 + -8) = 0x140d32a6a;
//       FUN_140d84db0((undefined1 *)((longlong)ppppppuVar51 + 0xb0),
//                     *(undefined8 *)((longlong)ppppppuVar51 + 0x50),
//                     *(undefined8 *)((longlong)ppppppuVar51 + 0x70),
//                     *(undefined8 *)((longlong)ppppppuVar51 + 0x1c0));
//       uVar35 = *(ulonglong *)((longlong)ppppppuVar51 + 0x110);
//       lVar27 = *(longlong *)((longlong)ppppppuVar51 + 0xb0);
//       lVar67 = *(longlong *)((longlong)ppppppuVar51 + 0xc0);
//     }
//     lVar45 = 0;
//     lVar63 = 9999;
//     *(byte **)((longlong)ppppppuVar51 + 0x90) = (byte *)((longlong)ppppppuVar56 + 1);
//     if ((((longlong)ppppppuVar56 + 1 < 0x1e) && ((longlong)uVar35 < 0x1e)) &&
//        (-1 < (longlong)(*(ulonglong *)((longlong)ppppppuVar51 + 0x90) | uVar35))) {
//       *(longlong *)((longlong)ppppppuVar51 + 0x28) =
//            *(longlong *)((longlong)ppppppuVar51 + 0x90) * 32000 + 16000;
//       *(ulonglong *)((longlong)ppppppuVar51 + 0x20) = uVar35 * 32000 + 16000;
//       *(undefined1 *)((longlong)ppppppuVar51 + 0x30) = 0xc;
//       *(undefined8 *)((longlong)ppppppuVar51 + -8) = 0x140d32afd;
//       FUN_140d84db0((undefined1 *)((longlong)ppppppuVar51 + 0xb0),
//                     *(undefined8 *)((longlong)ppppppuVar51 + 0x50),
//                     *(undefined8 *)((longlong)ppppppuVar51 + 0x70),
//                     *(undefined8 *)((longlong)ppppppuVar51 + 0x1c0));
//       uVar35 = *(ulonglong *)((longlong)ppppppuVar51 + 0x110);
//       lVar63 = *(longlong *)((longlong)ppppppuVar51 + 0xb0);
//       lVar45 = *(longlong *)((longlong)ppppppuVar51 + 0xc0);
//     }
//     lVar25 = *(longlong *)((longlong)ppppppuVar51 + 0x1d0) + 2;
//     *(longlong *)((longlong)ppppppuVar51 + 0xa8) = lVar25;
//     if (((lVar25 < 0x1e) && ((longlong)uVar35 < 0x1e)) &&
//        (lVar25 = *(longlong *)((longlong)ppppppuVar51 + 0xa8),
//        -1 < (longlong)(*(ulonglong *)((longlong)ppppppuVar51 + 0xa8) | uVar35))) {
//       lVar25 = lVar25 * 32000 + 16000;
//       *(longlong *)((longlong)ppppppuVar51 + 0x108) = lVar25;
//       *(longlong *)((longlong)ppppppuVar51 + 0x28) = lVar25;
//       *(ulonglong *)((longlong)ppppppuVar51 + 0x20) = uVar35 * 32000 + 16000;
//       *(undefined1 *)((longlong)ppppppuVar51 + 0x30) = 0xc;
//       *(undefined8 *)((longlong)ppppppuVar51 + -8) = 0x140d32ba2;
//       FUN_140d84db0((undefined1 *)((longlong)ppppppuVar51 + 0xb0),
//                     *(undefined8 *)((longlong)ppppppuVar51 + 0x50),
//                     *(undefined8 *)((longlong)ppppppuVar51 + 0x70),
//                     *(undefined8 *)((longlong)ppppppuVar51 + 0x1c0));
//       lVar25 = *(longlong *)((longlong)ppppppuVar51 + 0xc0) -
//                *(longlong *)((longlong)ppppppuVar51 + 0xb0);
//     }
//     else {
//       *(longlong *)((longlong)ppppppuVar51 + 0x108) = lVar25 * 32000 + 16000;
//       lVar25 = -9999;
//     }
//     lVar21 = lVar21 - *(longlong *)((longlong)ppppppuVar51 + 0x58);
//     lVar37 = lVar37 - *(longlong *)((longlong)ppppppuVar51 + 0x60);
//     lVar30 = lVar30 - lVar44;
//     lVar67 = lVar67 - lVar27;
//     lVar45 = lVar45 - lVar63;
//     lVar27 = lVar37;
//     if (lVar37 < lVar21) {
//       lVar27 = lVar21;
//     }
//     uVar35 = (ulonglong)(lVar21 < lVar37);
//     lVar21 = lVar27;
//     if (lVar27 <= lVar30) {
//       lVar21 = lVar30;
//     }
//     if (lVar27 < lVar30) {
//       uVar35 = 2;
//     }
//     lVar37 = lVar21;
//     if (lVar21 <= lVar67) {
//       lVar37 = lVar67;
//     }
//     if (lVar21 < lVar67) {
//       uVar35 = 3;
//     }
//     lVar21 = lVar37;
//     if (lVar37 <= lVar45) {
//       lVar21 = lVar45;
//     }
//     if (lVar37 < lVar45) {
//       uVar35 = 4;
//     }
//     lVar37 = lVar21;
//     if (lVar21 <= lVar25) {
//       lVar37 = lVar25;
//     }
//     if (lVar21 < lVar25) {
//       uVar35 = 5;
//     }
//     *(longlong *)((longlong)ppppppuVar51 + 0x150) =
//          *(longlong *)((longlong)ppppppuVar51 + 0x98) * 32000 + 16000;
//     *(longlong *)((longlong)ppppppuVar51 + 0x148) =
//          *(longlong *)((longlong)ppppppuVar51 + 0x78) * 32000 + 16000;
//     *(longlong *)((longlong)ppppppuVar51 + 0x140) =
//          *(longlong *)((longlong)ppppppuVar51 + 0x88) * 32000 + 16000;
//     *(longlong *)((longlong)ppppppuVar51 + 0x138) =
//          *(longlong *)((longlong)ppppppuVar51 + 0x1d0) * 32000 + 16000;
//     *(longlong *)((longlong)ppppppuVar51 + 0x130) =
//          *(longlong *)((longlong)ppppppuVar51 + 0x90) * 32000 + 16000;
//     puVar66 = (undefined1 *)(*(longlong *)((longlong)ppppppuVar51 + 0x1c8) * 32000 + -48000);
//     lVar21 = -2;
//     uVar34 = 0;
//     do {
//       *(longlong *)((longlong)ppppppuVar51 + 0x60) = lVar21;
//       uVar65 = *(longlong *)((longlong)ppppppuVar51 + 0x1c8) + lVar21;
//       lVar21 = -9999;
//       lVar30 = -9999;
//       if (((*(longlong *)((longlong)ppppppuVar51 + 0x98) < 0x1e) && ((longlong)uVar65 < 0x1e)) &&
//          (-1 < (longlong)(uVar65 | *(ulonglong *)((longlong)ppppppuVar51 + 0x98)))) {
//         *(undefined8 *)((longlong)ppppppuVar51 + 0x28) =
//              *(undefined8 *)((longlong)ppppppuVar51 + 0x150);
//         *(undefined1 **)((longlong)ppppppuVar51 + 0x20) = puVar66;
//         *(undefined1 *)((longlong)ppppppuVar51 + 0x30) = 0xc;
//         *(undefined8 *)((longlong)ppppppuVar51 + -8) = 0x140d32d9b;
//         FUN_140d84db0((undefined1 *)((longlong)ppppppuVar51 + 0xb0),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x50),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x70),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x1c0));
//         lVar30 = *(longlong *)((longlong)ppppppuVar51 + 0xc0) -
//                  *(longlong *)((longlong)ppppppuVar51 + 0xb0);
//       }
//       if (lVar37 < lVar30) {
//         uVar35 = 0;
//       }
//       lVar27 = lVar30;
//       if (lVar30 < lVar37) {
//         lVar27 = lVar37;
//       }
//       if (((*(longlong *)((longlong)ppppppuVar51 + 0x78) < 0x1e) && ((longlong)uVar65 < 0x1e)) &&
//          (-1 < (longlong)(uVar65 | *(ulonglong *)((longlong)ppppppuVar51 + 0x78)))) {
//         *(undefined8 *)((longlong)ppppppuVar51 + 0x28) =
//              *(undefined8 *)((longlong)ppppppuVar51 + 0x148);
//         *(undefined1 **)((longlong)ppppppuVar51 + 0x20) = puVar66;
//         *(undefined1 *)((longlong)ppppppuVar51 + 0x30) = 0xc;
//         *(undefined8 *)((longlong)ppppppuVar51 + -8) = 0x140d32e11;
//         FUN_140d84db0((undefined1 *)((longlong)ppppppuVar51 + 0xb0),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x50),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x70),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x1c0));
//         lVar21 = *(longlong *)((longlong)ppppppuVar51 + 0xc0) -
//                  *(longlong *)((longlong)ppppppuVar51 + 0xb0);
//       }
//       *(bool *)((longlong)ppppppuVar51 + 0x58) = lVar37 < lVar30;
//       if (lVar27 < lVar21) {
//         uVar35 = 1;
//       }
//       lVar30 = lVar21;
//       if (lVar21 < lVar27) {
//         lVar30 = lVar27;
//       }
//       *(bool *)((longlong)ppppppuVar51 + 0x68) = lVar27 < lVar21;
//       lVar37 = -9999;
//       lVar21 = -9999;
//       if (((*(longlong *)((longlong)ppppppuVar51 + 0x88) < 0x1e) && ((longlong)uVar65 < 0x1e)) &&
//          (-1 < (longlong)(uVar65 | *(ulonglong *)((longlong)ppppppuVar51 + 0x88)))) {
//         *(undefined8 *)((longlong)ppppppuVar51 + 0x28) =
//              *(undefined8 *)((longlong)ppppppuVar51 + 0x140);
//         *(undefined1 **)((longlong)ppppppuVar51 + 0x20) = puVar66;
//         *(undefined1 *)((longlong)ppppppuVar51 + 0x30) = 0xc;
//         *(undefined8 *)((longlong)ppppppuVar51 + -8) = 0x140d32ea2;
//         FUN_140d84db0((undefined1 *)((longlong)ppppppuVar51 + 0xb0),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x50),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x70),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x1c0));
//         lVar21 = *(longlong *)((longlong)ppppppuVar51 + 0xc0) -
//                  *(longlong *)((longlong)ppppppuVar51 + 0xb0);
//       }
//       if (lVar30 < lVar21) {
//         uVar35 = 2;
//       }
//       lVar27 = lVar21;
//       if (lVar21 < lVar30) {
//         lVar27 = lVar30;
//       }
//       if (((*(longlong *)((longlong)ppppppuVar51 + 0x1d0) < 0x1e) && ((longlong)uVar65 < 0x1e)) &&
//          (-1 < (longlong)(uVar65 | *(ulonglong *)((longlong)ppppppuVar51 + 0x1d0)))) {
//         *(undefined8 *)((longlong)ppppppuVar51 + 0x28) =
//              *(undefined8 *)((longlong)ppppppuVar51 + 0x138);
//         *(undefined1 **)((longlong)ppppppuVar51 + 0x20) = puVar66;
//         *(undefined1 *)((longlong)ppppppuVar51 + 0x30) = 0xc;
//         *(undefined8 *)((longlong)ppppppuVar51 + -8) = 0x140d32f18;
//         FUN_140d84db0((undefined1 *)((longlong)ppppppuVar51 + 0xb0),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x50),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x70),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x1c0));
//         lVar37 = *(longlong *)((longlong)ppppppuVar51 + 0xc0) -
//                  *(longlong *)((longlong)ppppppuVar51 + 0xb0);
//       }
//       if (lVar27 < lVar37) {
//         uVar35 = 3;
//       }
//       lVar44 = lVar37;
//       if (lVar37 < lVar27) {
//         lVar44 = lVar27;
//       }
//       *(bool *)((longlong)ppppppuVar51 + 0x118) = lVar27 < lVar37;
//       lVar27 = -9999;
//       if (((*(longlong *)((longlong)ppppppuVar51 + 0x90) < 0x1e) && ((longlong)uVar65 < 0x1e)) &&
//          (-1 < (longlong)(uVar65 | *(ulonglong *)((longlong)ppppppuVar51 + 0x90)))) {
//         *(undefined8 *)((longlong)ppppppuVar51 + 0x28) =
//              *(undefined8 *)((longlong)ppppppuVar51 + 0x130);
//         *(undefined1 **)((longlong)ppppppuVar51 + 0x20) = puVar66;
//         *(undefined1 *)((longlong)ppppppuVar51 + 0x30) = 0xc;
//         *(undefined8 *)((longlong)ppppppuVar51 + -8) = 0x140d32fa7;
//         FUN_140d84db0((undefined1 *)((longlong)ppppppuVar51 + 0xb0),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x50),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x70),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x1c0));
//         lVar27 = *(longlong *)((longlong)ppppppuVar51 + 0xc0) -
//                  *(longlong *)((longlong)ppppppuVar51 + 0xb0);
//       }
//       lVar37 = -9999;
//       if (lVar44 < lVar27) {
//         uVar35 = 4;
//       }
//       lVar67 = lVar27;
//       if (lVar27 < lVar44) {
//         lVar67 = lVar44;
//       }
//       if ((((longlong)*(ulonglong *)((longlong)ppppppuVar51 + 0xa8) < 0x1e) &&
//           ((longlong)uVar65 < 0x1e)) &&
//          (-1 < (longlong)(uVar65 | *(ulonglong *)((longlong)ppppppuVar51 + 0xa8)))) {
//         *(undefined8 *)((longlong)ppppppuVar51 + 0x28) =
//              *(undefined8 *)((longlong)ppppppuVar51 + 0x108);
//         *(undefined1 **)((longlong)ppppppuVar51 + 0x20) = puVar66;
//         *(undefined1 *)((longlong)ppppppuVar51 + 0x30) = 0xc;
//         *(undefined8 *)((longlong)ppppppuVar51 + -8) = 0x140d3302b;
//         FUN_140d84db0((undefined1 *)((longlong)ppppppuVar51 + 0xb0),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x50),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x70),
//                       *(undefined8 *)((longlong)ppppppuVar51 + 0x1c0));
//         lVar37 = *(longlong *)((longlong)ppppppuVar51 + 0xc0) -
//                  *(longlong *)((longlong)ppppppuVar51 + 0xb0);
//       }
//       uVar65 = *(longlong *)((longlong)ppppppuVar51 + 0x60) + 3;
//       if (((*(byte *)((longlong)ppppppuVar51 + 0x58) | *(byte *)((longlong)ppppppuVar51 + 0x68)) & 1
//           ) != 0) {
//         uVar34 = uVar65 & 0xffffffff;
//       }
//       if (lVar30 < lVar21 || (*(byte *)((longlong)ppppppuVar51 + 0x118) & 1) != 0) {
//         uVar34 = uVar65 & 0xffffffff;
//       }
//       if (lVar67 < lVar37) {
//         uVar35 = 5;
//         uVar34 = uVar65 & 0xffffffff;
//       }
//       if (lVar37 < lVar67) {
//         lVar37 = lVar67;
//       }
//       if (lVar44 < lVar27) {
//         uVar34 = uVar65 & 0xffffffff;
//       }
//       lVar21 = *(longlong *)((longlong)ppppppuVar51 + 0x60) + 1;
//       puVar66 = &DAT_00007d00 + (longlong)puVar66;
//     } while (lVar21 != 3);
//     puVar5 = *(undefined4 **)((longlong)ppppppuVar51 + 0xa0);
//     *puVar5 = 1;
//     lVar21 = *(longlong *)((longlong)ppppppuVar51 + 0x98) + uVar35;
//     if ((longlong)(*(longlong *)((longlong)ppppppuVar51 + 0x98) + uVar35) < 1) {
//       lVar21 = 0;
//     }
//     lVar37 = *(longlong *)((longlong)ppppppuVar51 + 0x110) + (longlong)(int)uVar34;
//     if (*(longlong *)((longlong)ppppppuVar51 + 0x110) + (longlong)(int)uVar34 < 1) {
//       lVar37 = 0;
//     }
//     if (0x1c < lVar21) {
//       lVar21 = 0x1d;
//     }
//     if (0x1c < lVar37) {
//       lVar37 = 0x1d;
//     }
//     lVar30 = _UNK_1433d8638 - *(longlong *)(*(longlong *)((longlong)ppppppuVar51 + 0x80) + 0x668);  // 0x668=y
//     *(longlong *)(puVar5 + 2) =
//          (lVar37 * 32000 - *(longlong *)(*(longlong *)((longlong)ppppppuVar51 + 0x80) + 0x660)) +  // 0x660=x
//          _UNK_1433d8630;
//     *(longlong *)(puVar5 + 4) = lVar21 * 32000 + lVar30;
//     return (ulonglong *****)(lVar21 * 32000);
//   case 0x14:
//     uVar34 = (longlong)(int)puVar46 * 0x433da0dc;
//     uVar35 = uVar34 & 0xffffffff;
//     *(byte *)ppppppuVar43 = *(byte *)ppppppuVar43 | (byte)(uVar34 >> 0x28);
// LAB_140d322e1:
//     uVar35 = (uVar35 & 0xffffffff) % ((ulonglong)ppppppuVar56 & 0xffffffff);
//     ppbVar50 = (byte **)pppppuVar33;
// LAB_140d322e5:
//                     /* WARNING: Could not recover jumptable at 0x000140d32302. Too many branches */
//                     /* WARNING: Treating indirect jump as call */
//     pppppuVar33 = (ulonglong *****)
//                   (*(code *)((longlong)&UINT_1433da124 +
//                             (longlong)(int)(&UINT_1433da124)[*(uint *)(ppbVar50 + 0xd)]))
//                             (&UINT_1433da124,uVar35);
//     return pppppuVar33;
//   case 0x16:
//   case 0x1c:
//   case 0x1e:
//     puVar46 = &switchD_140d38079::switchdataD_1433da134;
//     pppppuVar33 = (ulonglong *****)(ulonglong)((int)ppppppuVar60 + 0xc30);
//     goto switchD_140d38079_caseD_3;
//   case 0x17:
//     pppppuVar33 = (ulonglong *****)&pppppuStack_208;
//     if ((*ppppppuVar55 == (ulonglong *****)*in_stack_00001638) &&
//        (((*ppppppuVar55 != (ulonglong *****)0x0 ||
//          (pppppuVar33 = (ulonglong *****)&pppppuStack_208,
//          ppppppuVar55[1] == (ulonglong *****)in_stack_00001638[1])) &&
//         (pppppuVar33 = (ulonglong *****)&pppppuStack_208, *(byte *)(ppppppuVar55 + 0x11) != 0)))) {
//       if (0xc < in_stack_00001630) {
//         ppppppuStack_210 = (ulonglong ******)0x140d38f64;
//         FUN_1431a3a70(0,in_stack_00001630,0xc,&PTR_s_game_ai_src_utils_rs_1433d8a28);
//         goto LAB_140d3902c;
//       }
//       uVar35 = 0;
//       lVar21 = in_stack_00001610;
//       do {
//         if (*(ulonglong ******)(&stack0x000013f8 + uVar35 * 8) == ppppppuVar55[0x12]) {
//           pppppuVar33 = (ulonglong *****)&pppppuStack_208;
//           if (*(int *)(ppppppuVar55 + 0x98) != -1) {
//             if (in_stack_00001600 <= uVar35) {
//               ppppppuStack_210 = (ulonglong ******)0x140d38ff0;
//               FUN_1431a3863(uVar35,in_stack_00001600,&PTR_s_game_ai_src_utils_rs_1433d9678);
//               goto LAB_140d3902c;
//             }
//             ppppuStack_1e8 = *(ulonglong *****)(in_stack_000015e8 + uVar35 * 8);
//             ppppppuStack_210 = (ulonglong ******)0x140d381f5;
//             uVar34 = FUN_1412857f0(ppppppuVar55 + 0x92,in_stack_00001628,ppppppuVar55,
//                                    &PTR_FUN_1433d8768);
//             ppppppuStack_210 = (ulonglong ******)0x140d38210;
//             lVar21 = (*(code *)ppppppuVar55[0xaf][0x12])(ppppppuVar55[0xae],ppppppuVar55);
//             uVar41 = *(int *)((longlong)ppppppuVar55 + 0x3fc) + 100;
//             if ((int)uVar41 < 2) {
//               uVar41 = (uint)bVar39;
//             }
//             uVar28 = lVar21 * 100;
//             if (uVar28 >> 0x20 == 0) {
//               uVar28 = uVar28 & 0xffffffff;
//             }
//             pppppuVar33 = (ulonglong *****)(uVar28 / uVar41);
//             if ((SBORROW8(0,uVar34)) && (pppppuVar33 == (ulonglong *****)0xffffffffffffffff)) {
//               ppppppuStack_210 = (ulonglong ******)0x140d38fff;
//               FUN_1431a3b80(&PTR_s_game_ai_src_utils_rs_1433d9690);
//               goto LAB_140d3902c;
//             }
//             if (pppppuVar33 < &DAT_00000004) {
//               pppppuVar33 = param_1;
//             }
//             if ((uVar34 | (ulonglong)pppppuVar33) >> 0x20 == 0) {
//               *(ulonglong *)(&stack0x00001558 + uVar35 * 8) =
//                    *(longlong *)(&stack0x00001558 + uVar35 * 8) +
//                    (uVar34 & 0xffffffff) / ((ulonglong)pppppuVar33 & 0xffffffff);
//               pppppuVar33 = (ulonglong *****)&pppppuStack_208;
//             }
//             else {
//               *(longlong *)(&stack0x00001558 + uVar35 * 8) =
//                    *(longlong *)(&stack0x00001558 + uVar35 * 8) +
//                    (longlong)uVar34 / (longlong)pppppuVar33;
//               pppppuVar33 = (ulonglong *****)&pppppuStack_208;
//             }
//           }
//           break;
//         }
//         uVar35 = uVar35 + 1;
//         lVar21 = lVar21 + -8;
//         pppppuVar33 = (ulonglong *****)&pppppuStack_208;
//       } while (lVar21 != 0);
//     }
//     goto switchD_140d38079_caseD_3;
//   default:
//                     /* WARNING: Bad instruction - Truncating control flow here */
//     halt_baddata();
//   case 0x1d:
//     switchD_140d2e62d::caseD_4c._0_1_ = (char)switchD_140d2e62d::caseD_4c + -0x48;
//     pppppuVar33 = (ulonglong *****)&pppppuStack_208;
// switchD_140d38079_caseD_3:
//     while( true ) {
//       *(undefined8 *)((longlong)pppppuVar33 + -8) = 0x140d38058;
//       lVar21 = FUN_14183ed40(uVar65);
//       if (lVar21 == 0) break;
//       uVar35 = *(longlong *)(lVar21 + 0x68) - 1;
//       if (uVar35 < 10) {
//                     /* WARNING: Could not recover jumptable at 0x000140d38079. Too many branches */
//                     /* WARNING: Treating indirect jump as call */
//         pppppuVar33 = (ulonglong *****)
//                       (*(code *)((longlong)puVar46 +
//                                 (longlong)(int)*(uint *)((longlong)puVar46 + uVar35 * 4)))();
//         return pppppuVar33;
//       }
//     }
//     if (1 < in_stack_000016c8) {
//       pcVar16 = *(code **)(in_stack_00001608 + 0x210);
//       *(undefined8 *)((longlong)pppppuVar33 + -8) = 0x140d382bd;
//       (*pcVar16)(&stack0x00001530,in_stack_00001618);
//       uVar41 = (uint)in_stack_00001548;
// LAB_140d38300:
//       do {
//         if (in_stack_00001530 == 0) {
//           if (in_stack_00001538 == in_stack_00001540) break;
//           in_stack_00001530 = 0;
//           pauVar19 = (undefined1 (*) [16])(in_stack_00001538[0x13] + 8);
//           pauVar64 = in_stack_00001538;
//           if (*(int *)in_stack_00001538[4] == 6) goto LAB_140d38374;
// LAB_140d383f1:
//           in_stack_00001538 = pauVar19;
//           if (in_stack_000016c8 != 2) {
//             uVar36 = *(undefined8 *)(pauVar64[0xf] + 8);
//             pcVar16 = *(code **)(in_stack_00001608 + 0x1f0);
//             *(undefined8 *)((longlong)pppppuVar33 + -8) = 0x140d38425;
//             plVar26 = (longlong *)(*pcVar16)(in_stack_00001618,uVar36);
//             if (((plVar26 != (longlong *)0x0) && (*plVar26 == *in_stack_00001638)) &&
//                ((*plVar26 != 0 || (plVar26[1] == in_stack_00001638[1])))) {
//               uVar35 = 0;
//               while (uVar34 = uVar35, uVar34 < in_stack_00001630) {
//                 if (in_stack_00001600 <= uVar34) {
//                   *(undefined8 *)((longlong)pppppuVar33 + -8) = 0x140d38f99;
//                   FUN_1431a3863(uVar34,in_stack_00001600,&PTR_s_game_ai_src_utils_rs_1433d9648);
//                   goto LAB_140d3902c;
//                 }
//                 lVar21 = *(longlong *)(in_stack_000015e8 + uVar34 * 8);
//                 *(undefined8 *)((longlong)pppppuVar33 + -8) = 0x140d384be;
//                 cVar20 = FUN_14129feb0(pauVar64[0x12] + 0xc,pauVar64,lVar21);
//                 uVar35 = uVar34 + 1;
//                 if (cVar20 != '\0') {
//                   if ((longlong)*(int *)(lVar21 + 0x470) == 0) {  // 0x470=사거리%보정
//                     uVar65 = *(ulonglong *)(lVar21 + 0x680);  // 0x680=기본사거리
//                   }
//                   else {
//                     uVar65 = (ulonglong)
//                              (((longlong)*(int *)(lVar21 + 0x470) + 100) *  // 0x470=사거리%보정
//                              *(longlong *)(lVar21 + 0x680)) / 100;  // 0x680=기본사거리
//                   }
//                   uVar36 = *(undefined8 *)(lVar21 + 0x660);  // 0x660=x
//                   uVar6 = *(undefined8 *)(lVar21 + 0x668);  // 0x668=y
//                   *(undefined8 *)((longlong)pppppuVar33 + -8) = 0x140d38516;
//                   cVar20 = FUN_14132aa00(pauVar64,uVar36,uVar6,uVar65);
//                   if (cVar20 != '\0') {
//                     *(undefined8 *)((longlong)pppppuVar33 + -8) = 0x140d38534;
//                     uVar36 = FUN_14132b310(pauVar64,in_stack_00001628,plVar26,lVar21);
//                     if (0xb < uVar34) {
//                       *(undefined8 *)((longlong)pppppuVar33 + -8) = 0x140d38fcc;
//                       FUN_1431a3863(uVar34,0xc,&PTR_s_game_ai_src_utils_rs_1433d9660);
//                       goto LAB_140d3902c;
//                     }
//                     uVar65 = *(ulonglong *)(&stack0x00001468 + uVar34 * 8);
//                     if (uVar65 < 0x10) {
//                       (&ppppppuStack_1d8)[uVar34 * 0x20 + uVar65 * 2] = (ulonglong ******)0x1;
//                       (&ppppppuStack_1d0)[uVar34 * 0x20 + uVar65 * 2] = (ulonglong ******)uVar36;
//                       *(ulonglong *)(&stack0x00001468 + uVar34 * 8) = uVar65 + 1;
//                     }
//                   }
//                 }
//               }
//             }
//           }
//           goto LAB_140d38300;
//         }
//         if (in_stack_00001550 == 0) break;
//         if ((short)uVar41 == 0) {
//           do {
//             auVar14 = *in_stack_00001538;
//             in_stack_00001530 = in_stack_00001530 + -0x1400;
//             in_stack_00001538 = in_stack_00001538 + 1;
//             uVar41 = (ushort)((ushort)(SUB161(auVar14 >> 7,0) & 1) |
//                               (ushort)(SUB161(auVar14 >> 0xf,0) & 1) << 1 |
//                               (ushort)(SUB161(auVar14 >> 0x17,0) & 1) << 2 |
//                               (ushort)(SUB161(auVar14 >> 0x1f,0) & 1) << 3 |
//                               (ushort)(SUB161(auVar14 >> 0x27,0) & 1) << 4 |
//                               (ushort)(SUB161(auVar14 >> 0x2f,0) & 1) << 5 |
//                               (ushort)(SUB161(auVar14 >> 0x37,0) & 1) << 6 |
//                               (ushort)(SUB161(auVar14 >> 0x3f,0) & 1) << 7 |
//                               (ushort)(SUB161(auVar14 >> 0x47,0) & 1) << 8 |
//                               (ushort)(SUB161(auVar14 >> 0x4f,0) & 1) << 9 |
//                               (ushort)(SUB161(auVar14 >> 0x57,0) & 1) << 10 |
//                               (ushort)(SUB161(auVar14 >> 0x5f,0) & 1) << 0xb |
//                               (ushort)(SUB161(auVar14 >> 0x67,0) & 1) << 0xc |
//                               (ushort)(SUB161(auVar14 >> 0x6f,0) & 1) << 0xd |
//                               (ushort)(SUB161(auVar14 >> 0x77,0) & 1) << 0xe |
//                              (ushort)(byte)(auVar14[0xf] >> 7) << 0xf) ^ 0xffff;
//           } while (uVar41 == 0);
//         }
//         uVar17 = 0;
//         for (uVar18 = uVar41; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
//           uVar17 = uVar17 + 1;
//         }
//         uVar41 = uVar41 - 1 & uVar41;
//         in_stack_00001550 = in_stack_00001550 + -1;
//         lVar21 = in_stack_00001530 + (ulonglong)uVar17 * -0x140;
//         pauVar64 = (undefined1 (*) [16])(lVar21 + -0x138);
//         pauVar19 = in_stack_00001538;
//         if (*(int *)(lVar21 + -0xf8) != 6) goto LAB_140d383f1;
// LAB_140d38374:
//         in_stack_00001538 = pauVar19;
//         if (0xc < in_stack_00001630) {
//           *(undefined8 *)((longlong)pppppuVar33 + -8) = 0x140d38f43;
//           FUN_1431a3a70(0,in_stack_00001630,0xc,&PTR_s_game_ai_src_utils_rs_1433d8a28);
//           goto LAB_140d3902c;
//         }
//         lVar21 = (longlong)&ppppppuStack_1d0;
//         uVar35 = 0;
//         lVar37 = in_stack_00001610;
//         do {
//           if (*(longlong *)(&stack0x000013f8 + uVar35 * 8) == *(longlong *)pauVar64[5]) {
//             if (in_stack_00001600 <= uVar35) {
//               *(undefined8 *)((longlong)pppppuVar33 + -8) = 0x140d38fde;
//               FUN_1431a3863(uVar35,in_stack_00001600,&PTR_s_game_ai_src_utils_rs_1433d9630);
//               goto LAB_140d3902c;
//             }
//             lVar37 = *(longlong *)(in_stack_000015e8 + uVar35 * 8);
//             uVar36 = *(undefined8 *)(pauVar64[0xf] + 8);
//             pcVar16 = *(code **)(in_stack_00001608 + 0x1f0);
//             *(undefined8 *)((longlong)pppppuVar33 + -8) = 0x140d385ce;
//             lVar30 = (*pcVar16)(in_stack_00001618,uVar36);
//             if (lVar30 != 0) {
//               uVar36 = *(undefined8 *)pauVar64[0x10];
//               uVar6 = *(undefined8 *)(pauVar64[0x10] + 8);
//               uVar7 = *(undefined8 *)(lVar37 + 0x660);  // 0x660=x
//               uVar8 = *(undefined8 *)(lVar37 + 0x668);  // 0x668=y
//               *(undefined8 *)((longlong)pppppuVar33 + -8) = 0x140d38605;
//               uVar34 = FUN_1412a07d0(uVar36,uVar6,uVar7,uVar8);
//               uVar65 = *(longlong *)(pauVar64[4] + 8) +
//                        (ulonglong)(*(longlong *)(pauVar64[4] + 8) == 0);
//               if ((uVar34 | uVar65) >> 0x20 == 0) {
//                 uVar34 = (uVar34 & 0xffffffff) / (uVar65 & 0xffffffff);
//               }
//               else {
//                 uVar34 = uVar34 / uVar65;
//               }
//               *(undefined8 *)((longlong)pppppuVar33 + -8) = 0x140d38646;
//               uVar36 = FUN_14132b310(pauVar64,in_stack_00001628,lVar30,lVar37);
//               if (((undefined1 *)(uVar34 + 5) <= in_stack_000015d0) &&
//                  (uVar65 = *(ulonglong *)(&stack0x00001468 + uVar35 * 8), uVar65 < 0x10)) {
//                 *(undefined1 **)(lVar21 + (uVar65 * 2 + -1) * 8) = (undefined1 *)(uVar34 + 5);
//                 *(undefined8 *)(lVar21 + uVar65 * 0x10) = uVar36;
//                 *(ulonglong *)(&stack0x00001468 + uVar35 * 8) = uVar65 + 1;
//               }
//             }
//             break;
//           }
//           uVar35 = uVar35 + 1;
//           lVar21 = lVar21 + 0x100;
//           lVar37 = lVar37 + -8;
//         } while (lVar37 != 0);
//       } while( true );
//     }
//     uVar35 = 0x12;
//     if ((ulonglong)in_stack_000015d0 / 5 < 0x12) {
//       uVar35 = (ulonglong)in_stack_000015d0 / 5;
//     }
//     if (in_stack_000015d0 < (undefined1 *)0x5) {
//       if (((((((in_stack_00001630 != 1) && (in_stack_00001630 != 2)) && (in_stack_00001630 != 3)) &&
//             ((in_stack_00001630 != 4 && (in_stack_00001630 != 5)))) &&
//            ((in_stack_00001630 != 6 && ((in_stack_00001630 != 7 && (in_stack_00001630 != 8)))))) &&
//           (in_stack_00001630 != 9)) &&
//          (((in_stack_00001630 != 10 && (in_stack_00001630 != 0xb)) && (in_stack_00001630 != 0xc))))
//       {
// LAB_140d38f9f:
//         *(undefined8 *)((longlong)pppppuVar33 + -8) = 0x140d38fb5;
//         FUN_1431a3863(0xc,0xc,&PTR_s_game_ai_src_utils_rs_1433d95e8);
// LAB_140d3902c:
//                     /* WARNING: Does not return */
//         pcVar16 = (code *)invalidInstructionException();
//         (*pcVar16)();
//       }
//     }
//     else {
//       plVar26 = (longlong *)&stack0x00000ae0;
//       puVar66 = &stack0x00000ae8;
//       uVar34 = 0;
//       do {
//         if (uVar34 == 0xc) goto LAB_140d38f9f;
//         lVar27 = uVar34 * 0xc0;
//         lVar21 = *(longlong *)(&stack0x00001558 + uVar34 * 8);
//         *(longlong *)(&stack0x00000b90 + lVar27) = lVar21;
//         lVar37 = *(longlong *)(&stack0x00000b80 + lVar27);
//         uVar65 = lVar21 * 5;
//         lVar30 = *(longlong *)(&stack0x00001468 + uVar34 * 8);
//         if (lVar30 == 0) {
//           lVar30 = lVar37 + lVar21 * -5;
//           *(longlong *)(&stack0x00000ae8 + lVar27) = lVar30;
//           *(undefined8 *)(&stack0x00000b98 + lVar27) = 1;
//           uVar28 = 0xffffffffffffffff;
//           if (lVar30 < 1) {
//             if (lVar37 < 1) {
//               uVar28 = 5;
//             }
//             else {
//               uVar28 = lVar37 * 5;
//               if ((uVar65 == 0xffffffffffffffff) && (uVar57 = lVar37 * -5, SBORROW8(0,uVar28))) {
// LAB_140d38f78:
//                 *(undefined8 *)((longlong)pppppuVar33 + -8) = 0x140d38f84;
//                 FUN_1431a3b80(&PTR_s_game_ai_src_utils_rs_1433d9600,uVar57);
//                 goto LAB_140d3902c;
//               }
//               if ((uVar28 | uVar65) >> 0x20 == 0) {
//                 uVar28 = (uVar28 & 0xffffffff) / (uVar65 & 0xffffffff);
//               }
//               else {
//                 uVar28 = (longlong)uVar28 / (longlong)uVar65;
//               }
//             }
//           }
//           if (&DAT_00000009 < in_stack_000015d0) {
//             lVar44 = lVar21 * 10 - lVar37;
//             lVar37 = lVar37 + lVar21 * -10;
//             lVar67 = 1;
//             lVar63 = 2;
//             lVar30 = 5;
//             do {
//               lVar67 = lVar67 + 1;
//               *(longlong *)(puVar66 + lVar63 * 8 + -8) = lVar37;
//               *(longlong *)(&stack0x00000b98 + lVar27) = lVar63;
//               if ((lVar37 < 1) && (uVar28 == 0xffffffffffffffff)) {
//                 lVar45 = *(longlong *)(puVar66 + lVar63 * 8 + -0x10);
//                 if (lVar45 < 1) {
//                   uVar28 = lVar67 * 5;
//                 }
//                 else {
//                   uVar28 = lVar45 * 5;
//                   uVar48 = lVar45 + lVar44;
//                   if ((uVar48 == 0xffffffffffffffff) && (uVar57 = lVar45 * -5, SBORROW8(0,uVar28)))
//                   goto LAB_140d38f78;
//                   if ((uVar28 | uVar48) >> 0x20 == 0) {
//                     uVar28 = (uVar28 & 0xffffffff) / (uVar48 & 0xffffffff);
//                   }
//                   else {
//                     uVar28 = (longlong)uVar28 / (longlong)uVar48;
//                   }
//                   uVar28 = uVar28 + lVar30;
//                 }
//               }
//               lVar30 = lVar30 + 5;
//               lVar45 = (1 - uVar35) + lVar63;
//               lVar63 = lVar63 + 1;
//               lVar44 = lVar44 + uVar65;
//               lVar37 = lVar37 + lVar21 * -5;
//             } while (lVar45 != 1);
//           }
//         }
//         else {
//           pppppuVar38 = (ulonglong *****)(&ppppppuStack_1d8)[uVar34 * 0x20];
//           uVar28 = 0xffffffffffffffff;
//           lVar44 = 1;
//           lVar67 = 0;
//           plVar47 = plVar26;
//           pppppuVar53 = (ulonglong *****)0x0;
//           do {
//             pppppuVar61 = (ulonglong *****)((longlong)pppppuVar53 + 5);
//             lVar37 = lVar37 + lVar21 * -5;
//             if (pppppuVar53 < pppppuVar38 && pppppuVar38 <= pppppuVar61) {
//               lVar37 = lVar37 - (longlong)(&ppppppuStack_1d0)[uVar34 * 0x20];
//             }
//             if (lVar30 != 1) {
//               if ((&pbStack_1c8)[uVar34 * 0x20] <= pppppuVar61 &&
//                   pppppuVar53 < (&pbStack_1c8)[uVar34 * 0x20]) {
//                 lVar37 = lVar37 - (longlong)(&ppppppuStack_1c0)[uVar34 * 0x20];
//               }
//               if (lVar30 != 2) {
//                 if ((ulonglong *****)(&uStack_1b8)[uVar34 * 0x20] <= pppppuVar61 &&
//                     pppppuVar53 < (ulonglong *****)(&uStack_1b8)[uVar34 * 0x20]) {
//                   lVar37 = lVar37 - (longlong)(&ppppppuStack_1b0)[uVar34 * 0x20];
//                 }
//                 if (lVar30 != 3) {
//                   if ((&pbStack_1a8)[uVar34 * 0x20] <= pppppuVar61 &&
//                       pppppuVar53 < (&pbStack_1a8)[uVar34 * 0x20]) {
//                     lVar37 = lVar37 - (longlong)(&ppppppuStack_1a0)[uVar34 * 0x20];
//                   }
//                   if (lVar30 != 4) {
//                     if (*(ulonglong ******)(&iStack_198 + uVar34 * 0x40) <= pppppuVar61 &&
//                         pppppuVar53 < *(ulonglong ******)(&iStack_198 + uVar34 * 0x40)) {
//                       lVar37 = lVar37 - (longlong)(&pppppuStack_190)[uVar34 * 0x20];
//                     }
//                     if (lVar30 != 5) {
//                       if (ppppppuVar60[uVar34 * 0x20] <= pppppuVar61 &&
//                           pppppuVar53 < ppppppuVar60[uVar34 * 0x20]) {
//                         lVar37 = lVar37 - (longlong)(&pbStack_180)[uVar34 * 0x20];
//                       }
//                       if (lVar30 != 6) {
//                         if ((ulonglong *****)(&uStack_178)[uVar34 * 0x20] <= pppppuVar61 &&
//                             pppppuVar53 < (ulonglong *****)(&uStack_178)[uVar34 * 0x20]) {
//                           lVar37 = lVar37 - (longlong)(&puStack_170)[uVar34 * 0x20];
//                         }
//                         if (lVar30 != 7) {
//                           if ((ulonglong *****)(&uStack_168)[uVar34 * 0x20] <= pppppuVar61 &&
//                               pppppuVar53 < (ulonglong *****)(&uStack_168)[uVar34 * 0x20]) {
//                             lVar37 = lVar37 - (longlong)(&ppppuStack_160)[uVar34 * 0x20];
//                           }
//                           if (lVar30 != 8) {
//                             if ((&ppppuStack_160)[uVar34 * 0x20 + 1] <= pppppuVar61 &&
//                                 pppppuVar53 < (&ppppuStack_160)[uVar34 * 0x20 + 1]) {
//                               lVar37 = lVar37 - *(longlong *)(&uStack_150 + uVar34 * 0x40);
//                             }
//                             if (lVar30 != 9) {
//                               if ((ulonglong *****)(&uStack_148)[uVar34 * 0x20] <= pppppuVar61 &&
//                                   pppppuVar53 < (ulonglong *****)(&uStack_148)[uVar34 * 0x20]) {
//                                 lVar37 = lVar37 - (&lStack_140)[uVar34 * 0x20];
//                               }
//                               if (lVar30 != 10) {
//                                 if ((&puStack_138)[uVar34 * 0x20] <= pppppuVar61 &&
//                                     pppppuVar53 < (&puStack_138)[uVar34 * 0x20]) {
//                                   lVar37 = lVar37 - auStack_128[uVar34 * 0x20 + 0xffffffffffffffff];
//                                 }
//                                 if (lVar30 != 0xb) {
//                                   if ((ulonglong *****)auStack_128[uVar34 * 0x20] <= pppppuVar61 &&
//                                       pppppuVar53 < (ulonglong *****)auStack_128[uVar34 * 0x20]) {
//                                     lVar37 = lVar37 - auStack_128[uVar34 * 0x20 + 1];
//                                   }
//                                   if (lVar30 != 0xc) {
//                                     if ((ulonglong *****)auStack_128[uVar34 * 0x20 + 2] <=
//                                         pppppuVar61 &&
//                                         pppppuVar53 <
//                                         (ulonglong *****)auStack_128[uVar34 * 0x20 + 2]) {
//                                       lVar37 = lVar37 - auStack_128[uVar34 * 0x20 + 3];
//                                     }
//                                     if (lVar30 != 0xd) {
//                                       if ((ulonglong *****)auStack_128[uVar34 * 0x20 + 4] <=
//                                           pppppuVar61 &&
//                                           pppppuVar53 <
//                                           (ulonglong *****)auStack_128[uVar34 * 0x20 + 4]) {
//                                         lVar37 = lVar37 - auStack_128[uVar34 * 0x20 + 5];
//                                       }
//                                       if (lVar30 != 0xe) {
//                                         if ((ulonglong *****)auStack_128[uVar34 * 0x20 + 6] <=
//                                             pppppuVar61 &&
//                                             pppppuVar53 <
//                                             (ulonglong *****)auStack_128[uVar34 * 0x20 + 6]) {
//                                           lVar37 = lVar37 - auStack_128[uVar34 * 0x20 + 7];
//                                         }
//                                         if (lVar30 != 0xf) {
//                                           if ((ulonglong *****)auStack_128[uVar34 * 0x20 + 8] <=
//                                               pppppuVar61 &&
//                                               pppppuVar53 <
//                                               (ulonglong *****)auStack_128[uVar34 * 0x20 + 8]) {
//                                             lVar37 = lVar37 - auStack_128[uVar34 * 0x20 + 9];
//                                           }
//                                           if (lVar30 != 0x10) {
//                                             *(undefined8 *)((longlong)pppppuVar33 + -8) =
//                                                  0x140d3902b;
//                                             FUN_1431a3863(0x10,0x10,
//                                                           &PTR_s_game_ai_src_utils_rs_1433d9618);
//                                             goto LAB_140d3902c;
//                                           }
//                                         }
//                                       }
//                                     }
//                                   }
//                                 }
//                               }
//                             }
//                           }
//                         }
//                       }
//                     }
//                   }
//                 }
//               }
//             }
//             lVar67 = lVar67 + 1;
//             plVar47[1] = lVar37;
//             *(longlong *)(&stack0x00000b98 + lVar27) = lVar44;
//             if ((lVar37 < 1) && (uVar28 == 0xffffffffffffffff)) {
//               plVar29 = plVar47;
//               if (pppppuVar53 == (ulonglong *****)0x0) {
//                 plVar29 = (longlong *)(&stack0x00000b80 + lVar27);
//               }
//               lVar63 = *plVar29;
//               if (lVar63 < 1) {
//                 uVar28 = lVar67 * 5;
//               }
//               else {
//                 uVar28 = lVar63 * 5;
//                 uVar65 = lVar63 - lVar37;
//                 uVar57 = ~uVar65;
//                 if (uVar28 == 0x8000000000000000 && uVar57 == 0) goto LAB_140d38f78;
//                 if ((uVar28 | uVar65) >> 0x20 == 0) {
//                   uVar28 = (uVar28 & 0xffffffff) / (uVar65 & 0xffffffff);
//                 }
//                 else {
//                   uVar28 = (longlong)uVar28 / (longlong)uVar65;
//                 }
//                 uVar28 = (longlong)pppppuVar53 + uVar28;
//               }
//             }
//             plVar47 = plVar47 + 1;
//             lVar44 = lVar44 + 1;
//             pppppuVar53 = pppppuVar61;
//           } while ((ulonglong *****)(uVar35 * 5) != pppppuVar61);
//         }
//         uVar34 = uVar34 + 1;
//         *(ulonglong *)(&stack0x00000ba0 + lVar27) = uVar28;
//         plVar26 = plVar26 + 0x18;
//         puVar66 = puVar66 + 0xc0;
//       } while (uVar34 != in_stack_00001630);
//     }
//     *(undefined8 *)((longlong)pppppuVar33 + -8) = 0x140d388b7;
//     memcpy(in_stack_000015d8,&stack0x00000ae8,0x910);
//     if ((in_stack_000015f8 != 0) &&
//        (*(longlong *)(*(longlong *)(in_stack_000015f0 + 0x10) + 0x20) == in_stack_000015e8)) {
//       *(longlong *)(*(longlong *)(in_stack_000015f0 + 0x10) + 0x20) =
//            in_stack_000015e8 + in_stack_000015f8 * 8;
//     }
//     return in_stack_000015d8;
//   case 0x1f:
//     cVar20 = cVar20 + (char)((ulonglong)ppppppuVar43 >> 8);
//     pcVar59 = (char *)((ulonglong)ppppppuVar55 & 0xffffffff);
//     *pcVar59 = *pcVar59 + (char)pcVar59;
//     pcVar59[-0x7d] = pcVar59[-0x7d] + cVar20;
//     *(byte *)ppppppuVar56 = *(byte *)ppppppuVar56 + cVar20;
//                     /* WARNING: Bad instruction - Truncating control flow here */
//     halt_baddata();
//   case 0x20:
//   case 0x21:
//   case 0x22:
//     puVar62 = (undefined8 *)*param_6;
//     uVar36 = *puVar62;
//     lVar21 = puVar62[1];
//     lVar37 = *(longlong *)(uVar34 + 0x5c0);  // 0x5c0=핸들
//     ppppppuStack_210 = (ulonglong ******)0x140d399f8;
//     cVar20 = (**(code **)(lVar21 + 0xf8))(uVar36);
//     if (cVar20 == '\0') {
//       ppppppuStack_210 = (ulonglong ******)0x140d39a0a;
//       lVar30 = (**(code **)(lVar21 + 0x150))(uVar36,lVar37);
//       if (lVar30 != 0) {
//         pppuVar11 = ppppuVar68[(ulonglong)*(uint *)(lVar30 + 0x9c0) + 0x3c];  // 0x9c0=role
//         ppppppuStack_210 = (ulonglong ******)0x140d39a29;
//         pppuVar31 = (ulonglong ***)(**(code **)(lVar21 + 0x28))(uVar36);
//         if (pppuVar31 <= pppuVar11 + 0xf) goto LAB_140d39a35;
//       }
//       pppppuVar33 = (ulonglong *****)0x0;
//     }
//     else {
// LAB_140d39a35:
//       if (*(uint *)(param_1 + 0xd) == 0xd) {
//                     /* WARNING: Could not recover jumptable at 0x000140d39a4e. Too many branches */
//                     /* WARNING: Treating indirect jump as call */
//         pppppuVar33 = (ulonglong *****)
//                       (*(code *)((longlong)&UINT_1433da15c +
//                                 (longlong)(int)(&UINT_1433da15c)[(longlong)param_1[0xe]]))();
//         return pppppuVar33;
//       }
//       pppppuVar33 = ppppppuVar55[0x3d];
//       ppppppuStack_210 = (ulonglong ******)0x140d39a6b;
//       lVar30 = FUN_140d31bb0(puVar62);
//       if (lVar30 == 0) {
//         iVar69 = 0x32;
//       }
//       else {
//         uVar35 = 100;
//         if (*(ulonglong *)(lVar30 + 0x1f0) < 100) {
//           uVar35 = *(ulonglong *)(lVar30 + 0x1f0);
//         }
//         iVar69 = (int)uVar35;
//       }
//       ppppppuStack_210 = (ulonglong ******)0x140d39a95;
//       lVar21 = (**(code **)(lVar21 + 0x28))(uVar36);
//       iVar40 = 100 - (int)pppppuVar33;
//       if ((ulonglong *****)0x64 < pppppuVar33) {
//         iVar40 = 0;
//       }
//       uVar65 = (ulonglong)((int)((ulonglong)(uint)(iVar40 * iVar40 * 0x3a) * 0x68db9 >> 0x20) + 2);
//       uVar35 = (lVar21 << 0x28 ^ lVar37 << 0x14 ^ (ulonglong)ppppppuVar55[0x125]) +
//                0x9e3779b97f4a7c15;
//       uVar35 = (uVar35 >> 0x1e ^ uVar35) * -0x40a7b892e31b1a47;
//       uVar35 = (uVar35 >> 0x1b ^ uVar35) * -0x6b2fb644ecceee15;
//       uVar35 = uVar35 >> 0x1f ^ uVar35;
//       uVar34 = (((ulonglong)(uint)(iVar40 * iVar40 * 0x72) * 0x1a36e3 >> 0x22) - uVar65) + 7;
//       if (uVar34 != 0) {
//         auVar14._8_8_ = 0;
//         auVar14._0_8_ = uVar35;
//         auVar15._8_8_ = 0;
//         auVar15._0_8_ = uVar34;
//         uVar35 = SUB168(auVar14 * auVar15,8) + uVar65;
//       }
//       pppppuVar33 = (ulonglong *****)
//                     (ulonglong)
//                     ((iVar40 * ((uint)(iVar69 * iVar69 * 0xc) / 10000) & 0xffff) / 100 + uVar35 == 0
//                     );
//     }
//     return pppppuVar33;
//   case 0x23:
//   case 0x24:
//   case 0x25:
//   case 0x26:
//     *(byte *)((longlong)puVar46 + 3) = *(byte *)((longlong)puVar46 + 3) + cVar20;
//     cRam00000002867b41b8 = cRam00000002867b41b8 + -0x24;
//     *(byte *)((longlong)puVar46 + 3) = *(byte *)((longlong)puVar46 + 3) + cVar20;
//                     /* WARNING: Bad instruction - Truncating control flow here */
//     halt_baddata();
//   case 0x27:
//   case 0x2a:
//     if ((longlong)ppppppuVar43 < 0) {
//       UINT_1433da114._1_1_ = UINT_1433da114._1_1_ + cVar20;
//       puVar22 = (uint *)((longlong)&switchD_140d2e62d::caseD_4c + 2);
//     }
//     else {
//       switchD_140d2e62d::caseD_4c._0_1_ = (char)switchD_140d2e62d::caseD_4c + -0x24;
//     }
//   case 0x30:
//     LOCK();
//     *(int *)(param_2 + -0x2e00000) = (int)param_2;
//     UNLOCK();
//     ppppppuVar60 = (ulonglong ******)((longlong)ppppppuVar55 + 1);
//     *(byte *)ppppppuVar56 = *(byte *)ppppppuVar55;
//     *(char *)puVar22 = *(char *)puVar22 + (char)puVar22;
//     *(byte *)((longlong)puVar22 + 0xb0) = *(char *)((longlong)puVar22 + 0xb0) + bVar49;
//     if (unaff_R15 < *(ulonglong ******)((longlong)ppppppuVar60 + (longlong)puVar22)) {
// code_r0x000140d3bc81:
//       pppppuVar33 = (ulonglong *****)0x0;
//       ppppppuVar52 = &pppppuStack_208;
//       goto code_r0x000140d3bca3;
//     }
// LAB_140d3bc98:
//     pppppuVar33 = *(ulonglong ******)((longlong)puVar46 + pcStack_70 * 8);
//     ppppppuVar52 = (ulonglong ******)pppppuVar38;
// code_r0x000140d3bca3:
//     *(undefined8 *)((longlong)ppppppuVar52 + -8) = 0x140d3bcab;
//     cVar20 = FUN_14129ed50(ppppppuVar60);
//     if ((((cVar20 != '\0') || (*(int *)(ppppppuVar60 + 0xd) != 0xd)) ||
//         (ppppppuVar60[0x17] <= unaff_R15)) &&
//        (pppppuVar33 < *(ulonglong ******)((longlong)puVar46 + (pcStack_70 + 5) * 8))) {
//       pppppuVar33 = *(ulonglong ******)((longlong)puVar46 + (pcStack_70 + 5) * 8);
//     }
//     *(undefined8 *)((longlong)ppppppuVar52 + -8) = 0x140d3bce1;
//     cVar20 = FUN_14128cf70(ppppppuVar60);
//     if ((((cVar20 != '\0') || (*(int *)(ppppppuVar60 + 0xd) != 0xd)) ||
//         (ppppppuVar60[0x18] <= unaff_R15)) &&
//        (pppppuVar33 < *(ulonglong ******)((longlong)puVar46 + (pcStack_70 + 10) * 8))) {
//       pppppuVar33 = *(ulonglong ******)((longlong)puVar46 + (pcStack_70 + 10) * 8);
//     }
//     uVar65 = uVar65 + 8;
//     unaff_retaddr =
//          (ulonglong *****)
//          ((longlong)unaff_retaddr +
//          (longlong)*(ulonglong ******)((longlong)puVar46 + (pcStack_70 + 0x3c) * 8) +
//          (longlong)*(ulonglong ******)((longlong)puVar46 + (pcStack_70 + 0x37) * 8) +
//          (longlong)*(ulonglong ******)((longlong)puVar46 + (pcStack_70 + 0x32) * 8));
//     unaff_R14 = (undefined8 *)((longlong)unaff_R14 + (longlong)pppppuVar33);
//     ppppppuVar70 = (ulonglong ******)0x53d1ac100;
//     puVar46 = (uint *)unaff_RSI;
//     ppppppuVar56 = ppppppuStackX_8;
//     ppppuVar68 = unaff_RBP;
// LAB_140d3bad0:
//     while (uVar65 == 0x28) {
//       lVar21 = 0;
//       if (unaff_R14 <= (undefined8 *)CONCAT44(uStack_194,iStack_198)) {
//         lVar21 = (longlong)CONCAT44(uStack_194,iStack_198) - (longlong)unaff_R14;
//       }
//       uVar35 = lVar21 * 0x3c;
//       pcVar59 = (char *)((longlong)unaff_retaddr +
//                         (ulonglong)(unaff_retaddr == (ulonglong *****)0x0));
//       if ((uVar35 | (ulonglong)pcVar59) >> 0x20 == 0) {
//         unaff_retaddr = (ulonglong *****)((uVar35 & 0xffffffff) / ((ulonglong)pcVar59 & 0xffffffff))
//         ;
//       }
//       else {
//         unaff_retaddr = (ulonglong *****)(uVar35 / (ulonglong)pcVar59);
//       }
//       uVar65 = 0;
//       in_R11 = (ulonglong ******)0x0;
//       puVar62 = (undefined8 *)0x0;
//       do {
//         while (ppppppuVar55 = *(ulonglong *******)((longlong)puVar46 + uVar65),
//               ppppppuVar55 == (ulonglong ******)0x0) {
// LAB_140d3bde0:
//           uVar65 = uVar65 + 8;
//           ppppuVar42 = unaff_R12;
//           if (uVar65 == 0x28) goto LAB_140d3c140;
//         }
//         param_2 = (ulonglong ******)ppppppuVar55[0xcc];
//         puVar22 = (uint *)ppppppuVar55[0xcd];
//         ppppppuVar60 = (ulonglong ******)param_1[0xcc];
//         pppppuVar33 = (ulonglong *****)param_1[0xcd];
//         lVar21 = (longlong)ppppppuVar60 - (longlong)param_2;
//         if (ppppppuVar60 < param_2) {
//           lVar21 = (longlong)param_2 - (longlong)ppppppuVar60;
//         }
//         lVar37 = (longlong)pppppuVar33 - (longlong)puVar22;
//         if (pppppuVar33 < puVar22) {
//           lVar37 = (longlong)puVar22 - (longlong)pppppuVar33;
//         }
//         if (ppppppuVar70 < (ulonglong ******)(lVar37 * lVar37 + lVar21 * lVar21))
//         goto LAB_140d3bde0;
//         if (*(char *)param_1 == '\0') {
//           ppppuVar42 = param_1[1];
//           if ((ulonglong ****)0x1 < ppppuVar42) {
//             *(undefined8 *)((longlong)ppppppuVar52 + -8) = 0x140d3c499;
//             FUN_1431a3863(ppppuVar42,2,&PTR_s_game_core_src_simulation_entity__1433d8a80);
//             goto LAB_140d3c4df;
//           }
//           if (ppppppuVar55[(longlong)ppppuVar42 * 3 + 7] != (ulonglong *****)0x0)
//           goto LAB_140d3bde0;
//         }
//         if ((*(byte *)ppppppuVar55 != 0) ||
//            (ppppppuVar55[1] != (ulonglong *****)(1 - *(longlong *)(unaff_RBX + 0x930))))  // 0x930=side
//         goto LAB_140d3bf0f;
//         if (*(longlong *)(unaff_RBX + 0x930) != 1) {  // 0x930=side
//           bVar71 = param_2 + -100000 < (ulonglong ******)0x27101;
// code_r0x000140d3bedd:
//           if (!(bool)(bVar71 & puVar22 < (ulonglong *****)0xfa01)) {
//             if (param_2 + -0x1b580 < (ulonglong ******)0xfa01) {
// code_r0x000140d3bf03:
//               if (puVar22 < (ulonglong *****)0x27101) goto LAB_140d3bde0;
//             }
//             goto LAB_140d3bf0f;
//           }
//           goto LAB_140d3bde0;
//         }
//         if (param_2 < (ulonglong ******)0xfa01 &&
//             (ulonglong *****)((longlong)puVar22 + -800000) < (ulonglong *****)0x27101)
//         goto LAB_140d3bde0;
//         if (param_2 < (ulonglong ******)0x27101) {
// code_r0x000140d3bebb:
//           if ((ulonglong *****)((longlong)puVar22 + -0xdac00) < (ulonglong *****)0xfa01)
//           goto LAB_140d3bde0;
//         }
// LAB_140d3bf0f:
//         puVar46 = (uint *)in_R11;
//         pppppuVar33 = ppppppuVar55[0xb8];  // 0xb8=SmallAction stride
//         *(undefined8 *)((longlong)ppppppuVar52 + -8) = 0x140d3bf2c;
//         puVar22 = (uint *)FUN_140d31bb0(uStack_78,pppppuVar33);
//         ppppppuVar23 = ppppppuVar52;
//         unaff_R14 = puVar62;
//         if ((ulonglong *****)puVar22 == (ulonglong *****)0x0) {
//           *(undefined8 *)((longlong)ppppppuVar52 + -8) = 0x140d3c466;
//           FUN_1431a37c0(&PTR_s_game_ai_src_plan_legacy_old_pass_1433d9770);
//           goto LAB_140d3c4df;
//         }
// code_r0x000140d3bf35:
//         ppppppuVar70 = *(ulonglong *******)((longlong)puVar22 + 0x930);  // 0x930=side
//         param_2 = unaff_R13;
//         if ((ulonglong ******)0x1 < ppppppuVar70) {
//           *(undefined8 *)((longlong)ppppppuVar23 + -8) = 0x140d3c4b0;
//           FUN_1431a3863(ppppppuVar70,2,&PTR_s_game_ai_src_plan_legacy_old_pass_1433d9788);
//           goto LAB_140d3c4df;
//         }
// code_r0x000140d3bf46:
//         uVar41 = *(uint *)((longlong)puVar22 + 0x9c0);  // 0x9c0=role
//         *(undefined8 *)((longlong)ppppppuVar23 + -8) = 0x140d3bf54;
//         cVar20 = FUN_14128cc90(ppppppuVar55);
//         ppppppuVar56 = ppppppuStack_a0 + (longlong)ppppppuVar70 * 500 + (ulonglong)uVar41 * 100;
//         unaff_R13 = param_2;
//         if (cVar20 == '\0') goto code_r0x000140d3bf7c;
// LAB_140d3c04f:
//         pppppuVar33 = ppppppuVar56[(longlong)param_2];
//         ppppppuVar52 = ppppppuVar23;
// code_r0x000140d3c053:
//         *(undefined8 *)((longlong)ppppppuVar52 + -8) = 0x140d3c05b;
//         cVar20 = FUN_14129ed50(ppppppuVar55);
//         if ((((cVar20 != '\0') || (*(int *)(ppppppuVar55 + 0xd) != 0xd)) ||
//             (ppppppuVar55[0x17] <= unaff_R15)) &&
//            (pppppuVar33 < ppppppuVar56[(longlong)((longlong)unaff_R13 + 5)])) {
//           pppppuVar33 = ppppppuVar56[(longlong)((longlong)unaff_R13 + 5)];
//         }
//         *(undefined8 *)((longlong)ppppppuVar52 + -8) = 0x140d3c091;
//         cVar20 = FUN_14128cf70(ppppppuVar55);
//         if ((((cVar20 != '\0') || (*(int *)(ppppppuVar55 + 0xd) != 0xd)) ||
//             (ppppppuVar55[0x18] <= unaff_R15)) &&
//            (pppppuVar33 < ppppppuVar56[(longlong)((longlong)unaff_R13 + 10)])) {
//           pppppuVar33 = ppppppuVar56[(longlong)((longlong)unaff_R13 + 10)];
//         }
//         uVar65 = uVar65 + 8;
//         puVar62 = (undefined8 *)
//                   ((longlong)unaff_R14 +
//                   (longlong)ppppppuVar56[(longlong)((longlong)unaff_R13 + 0x3c)] +
//                   (longlong)ppppppuVar56[(longlong)((longlong)unaff_R13 + 0x37)] +
//                   (longlong)ppppppuVar56[(longlong)((longlong)unaff_R13 + 0x32)]);
//         in_R11 = (ulonglong ******)((longlong)puVar46 + (longlong)pppppuVar33);
//         ppppppuVar70 = (ulonglong ******)0x53d1ac100;
//         puVar46 = (uint *)unaff_RSI;
//         ppppppuVar56 = ppppppuStackX_8;
//         ppppuVar68 = unaff_RBP;
//         ppppuVar42 = unaff_R12;
//       } while (uVar65 != 0x28);
// LAB_140d3c140:
//       while (ppppuVar42 != ppppuVar68) {
//         pppuVar11 = *ppppuVar42;
//         ppppuVar42 = ppppuVar42 + 1;
//         if (*(int *)(pppuVar11 + 0x98) != -1) {
//           *(ulonglong ******)((longlong)ppppppuVar52 + 0x20) = param_1;
//           *(undefined8 *)((longlong)ppppppuVar52 + -8) = 0x140d3c184;
//           lVar21 = FUN_1412857f0(pppuVar11 + 0x92,pppppuStack_a8,pppuVar11,&PTR_FUN_1433d8768);
//           ppuVar9 = pppuVar11[0xae];
//           pcVar16 = (code *)pppuVar11[0xaf][0x12];
//           *(undefined8 *)((longlong)ppppppuVar52 + -8) = 0x140d3c1a1;
//           lVar37 = (*pcVar16)(ppuVar9,pppuVar11);
//           uVar41 = *(int *)((longlong)pppuVar11 + 0x3fc) + 100;
//           uVar35 = (ulonglong)uVar41;
//           if ((int)uVar41 < 2) {
//             uVar35 = 1;
//           }
//           uVar34 = lVar37 * 100;
//           if (uVar34 >> 0x20 == 0) {
//             uVar34 = uVar34 & 0xffffffff;
//           }
//           uVar34 = uVar34 / uVar35;
//           if (uVar34 < 4) {
//             uVar34 = 3;
//           }
//           uVar35 = lVar21 * (longlong)unaff_R15;
//           if ((uVar35 | uVar34) >> 0x20 == 0) {
//             uVar35 = (uVar35 & 0xffffffff) / (uVar34 & 0xffffffff);
//           }
//           else {
//             uVar35 = uVar35 / uVar34;
//           }
//           puVar62 = (undefined8 *)((longlong)puVar62 + uVar35);
//           in_R11 = (ulonglong ******)((longlong)in_R11 + lVar21);
//           puVar46 = (uint *)unaff_RSI;
//           ppppppuVar56 = ppppppuStackX_8;
//           ppppuVar68 = unaff_RBP;
//         }
//       }
//       lVar21 = 0;
//       if (in_R11 <= param_1[0xce]) {
//         lVar21 = (longlong)param_1[0xce] - (longlong)in_R11;
//       }
//       uVar35 = lVar21 * 0x3c;
//       uVar34 = (longlong)puVar62 + (ulonglong)(puVar62 == (undefined8 *)0x0);
//       if ((uVar35 | uVar34) >> 0x20 == 0) {
//         pbVar32 = (byte *)((uVar35 & 0xffffffff) / (uVar34 & 0xffffffff));
//       }
//       else {
//         pbVar32 = (byte *)(uVar35 / uVar34);
//       }
//       if ((byte *)((longlong)ppppppuStack_80 + (longlong)unaff_retaddr) <= pbVar32) {
//         if (*(uint *)(param_1 + 0x98) == 0xffffffff) {
//           lVar37 = 0;
//           lVar21 = (longlong)(int)*(uint *)(param_1 + 0x8e);
//           if (lVar21 != 0) goto LAB_140d3c279;
// LAB_140d3c2ae:
//           ppppuVar42 = param_1[0xd0];
//         }
//         else {
//           lVar37 = (longlong)param_1[0x87] + (longlong)param_1[0x94] +
//                    ((longlong)param_1[0xb9] + -1) * (longlong)param_1[0x95];
//           lVar21 = (longlong)(int)*(uint *)(param_1 + 0x8e);
//           if (lVar21 == 0) goto LAB_140d3c2ae;
// LAB_140d3c279:
//           ppppuVar42 = (ulonglong ****)((ulonglong)((lVar21 + 100) * (longlong)param_1[0xd0]) / 100)
//           ;
//         }
//         ppppuVar68 = param_1[0xcc];
//         ppppuVar10 = param_1[0xcd];
//         *(undefined8 *)((longlong)ppppppuVar52 + -8) = 0x140d3c2d0;
//         uVar35 = FUN_1412a07d0(ppppuVar68,ppppuVar10,uStack_178,puStack_170);
//         ppppuVar10 = param_1[200];
//         bStackX_17 = 3;
//         puVar46 = (uint *)unaff_RSI;
//         ppppppuVar56 = ppppppuStackX_8;
//         ppppuVar68 = unaff_RBP;
//         if ((uVar35 <= lVar37 + uStack_98 + (longlong)ppppuVar10 * uStack_168 + (longlong)ppppuVar42
//             ) && ((unaff_retaddr < pppppuStack_190 || (ppppuStack_160 < ppppuVar10)))) {
//           pbVar32 = (byte *)CONCAT71((int7)((ulonglong)ppppuVar10 >> 8),1);
//           if (unaff_retaddr <= pppppuStack_88) {
//             if (ppppuStack_48 == (ulonglong ****)CONCAT17(bStack_49,CONCAT16(bStack_4a,uStack_50)))
//             {
//               ppppppuVar60 = (ulonglong ******)&ppppppuStack_60;
//               *(undefined8 *)((longlong)ppppppuVar52 + -8) = 0x140d3c362;
//               FUN_1410ea8e0(ppppppuVar60,ppppuStack_48,1,1);
//               pbVar32 = (byte *)CONCAT71((int7)((ulonglong)ppppppuVar60 >> 8),1);
//             }
//             ppppppuStack_60[(longlong)ppppuStack_48] = (ulonglong *****)ppppppuStack_188;
//             ppppuStack_48 = (ulonglong ****)((longlong)ppppuStack_48 + 1);
//           }
//           bStackX_17 = (byte)pbVar32 | 2;
//           pbStack_180 = pbVar32;
//         }
//       }
//       ppppppuVar70 = (ulonglong ******)0x53d1ac100;
//       if (unaff_RDI == ppppppuStack_1a0) {
//         bVar39 = 4;
//         if (((ulonglong)pbStack_180 & 1) == 0) {
//           bVar39 = bStackX_17;
//         }
//         pppppuStack_68[3] = (ulonglong ****)CONCAT17(bStack_49,CONCAT16(bStack_4a,uStack_50));
//         pppppuStack_68[4] = ppppuStack_48;
//         bVar49 = 0;
//         if (ppppuStack_48 == (ulonglong ****)0x0) {
//           bVar49 = bVar39;
//         }
//         pppppuStack_68[1] = (ulonglong ****)ppppppuStack_60;
//         pppppuStack_68[2] = ppppuStack_58;
//         *(byte *)pppppuStack_68 = bVar49;
//         if ((pbStack_1c8 != (byte *)0x0) && (ppppppuStack_1d0[2][4] == unaff_R12)) {
//           ppppppuStack_1d0[2][4] = unaff_R12 + (longlong)pbStack_1c8;
//         }
//         if ((puStack_b8 != (undefined1 *)0x0) &&
//            (*(longlong *)(*(longlong *)(uStack_c0 + 0x10) + 0x20) == lStack_90)) {
//           *(longlong *)(*(longlong *)(uStack_c0 + 0x10) + 0x20) =
//                lStack_90 + (longlong)puStack_b8 * 8;
//         }
//         return pppppuStack_68;
//       }
//       param_1 = *unaff_RDI;
//       ppppppuStack_188 = (ulonglong ******)param_1[0xb8];  // 0xb8=SmallAction stride
//       *(undefined8 *)((longlong)ppppppuVar52 + -8) = 0x140d3ba20;
//       unaff_RBX = FUN_140d31bb0(uStack_78);
//       if (unaff_RBX == 0) {
//         *(undefined8 *)((longlong)ppppppuVar52 + -8) = 0x140d3c454;
//         FUN_1431a37c0(&PTR_s_game_ai_src_plan_legacy_old_pass_1433d9758);
//         goto LAB_140d3c4df;
//       }
//       unaff_RDI = unaff_RDI + 1;
//       unaff_R13 = (ulonglong ******)(ulonglong)*(uint *)(unaff_RBX + 0x9c0);  // 0x9c0=role
//       uVar35 = *(ulonglong *)(unaff_RBX + 0x230);
//       if (99 < uVar35) {
//         uVar35 = 100;
//       }
//       ppppppuStack_80 = (ulonglong ******)(ulonglong)(((int)uVar35 * -800 + 80000U) / 1000 + 0x50);
//       pppppuStack_190 = (ulonglong *****)(ulonglong)(((int)uVar35 * 0x1c2 & 0xffffU) / 1000 + 0x2d);
//       unaff_R14 = (undefined8 *)0x0;
//       unaff_retaddr = (ulonglong *****)0x0;
//       uVar65 = 0;
//     }
//     ppppppuVar55 = *(ulonglong *******)((longlong)ppppppuVar56 + uVar65);
//     if (ppppppuVar55 != (ulonglong ******)0x0) {
//       pppppuVar33 = ppppppuVar55[0xcc];
//       pppppuVar38 = ppppppuVar55[0xcd];
//       pppppuVar53 = (ulonglong *****)param_1[0xcc];
//       pppppuVar61 = (ulonglong *****)param_1[0xcd];
//       lVar21 = (longlong)pppppuVar53 - (longlong)pppppuVar33;
//       if (pppppuVar53 < pppppuVar33) {
//         lVar21 = (longlong)pppppuVar33 - (longlong)pppppuVar53;
//       }
//       lVar37 = (longlong)pppppuVar61 - (longlong)pppppuVar38;
//       if (pppppuVar61 < pppppuVar38) {
//         lVar37 = (longlong)pppppuVar38 - (longlong)pppppuVar61;
//       }
//       if ((ulonglong ******)(lVar37 * lVar37 + lVar21 * lVar21) <= ppppppuVar70) {
// code_r0x000140d3bb2d:
//         uVar35 = (longlong)ppppppuVar55[0xce] * 100;
//         uVar34 = (longlong)ppppppuVar55[0xc5] +
//                  (ulonglong)(ppppppuVar55[0xc5] == (ulonglong *****)0x0);
//         if ((uVar35 | uVar34) >> 0x20 == 0) {
//           puVar22 = (uint *)((uVar35 & 0xffffffff) / (uVar34 & 0xffffffff));
//           pppppuVar3 = (ulonglong *****)ppppppuVar52;
// switchD_140d2e62d_caseD_2f:
//           ppppppuVar52 = (ulonglong ******)pppppuVar3;
//           if (&DAT_00000027 < puVar22) goto LAB_140d3bb69;
//         }
//         else if (0x27 < uVar35 / uVar34) goto LAB_140d3bb69;
//       }
//     }
//     uVar65 = uVar65 + 8;
//     goto LAB_140d3bad0;
//   case 0x28:
//     switchD_140d2e62d::caseD_4c._3_1_ = switchD_140d2e62d::caseD_4c._3_1_ + cVar20;
//                     /* WARNING: Bad instruction - Truncating control flow here */
//     halt_baddata();
//   case 0x29:
//     goto switchD_140d2e62d_caseD_29;
//   case 0x2b:
//   case 0x2e:
//     ppppppuVar52 = &pppppuStack_208;
//     goto code_r0x000140d3bb2d;
//   case 0x2c:
//   case 0x2d:
//     UNK_1433da067 = UNK_1433da067 + cVar20;
//     iVar69 = CONCAT13(switchD_140d2e62d::caseD_4c._3_1_,
//                       CONCAT12(switchD_140d2e62d::caseD_4c._2_1_,
//                                CONCAT11(switchD_140d2e62d::caseD_4c._1_1_,
//                                         (char)switchD_140d2e62d::caseD_4c))) + 0x433da0dc;
//     switchD_140d2e62d::caseD_4c._0_1_ = (char)iVar69;
//     switchD_140d2e62d::caseD_4c._1_1_ = (char)((uint)iVar69 >> 8);
//     switchD_140d2e62d::caseD_4c._2_1_ = (undefined1)((uint)iVar69 >> 0x10);
//     switchD_140d2e62d::caseD_4c._3_1_ = (char)((uint)iVar69 >> 0x18);
//                     /* WARNING: Bad instruction - Truncating control flow here */
//     halt_baddata();
//   case 0x2f:
//   case 0x34:
//     goto switchD_140d2e62d_caseD_2f;
//   case 0x31:
//     switchD_140d2e62d::caseD_4c._0_1_ = (char)switchD_140d2e62d::caseD_4c + -0x24;
//     goto code_r0x000140d3bb85;
//   case 0x32:
//     puVar22 = (uint *)&DAT_000009c0;
//     pppppuVar61 = (ulonglong *****)&pppppuStack_208;
//     goto code_r0x000140d3bb96;
//   case 0x33:
//     uVar41 = CONCAT13((undefined1)UINT_1433da0e0,
//                       CONCAT12(switchD_140d2e62d::caseD_4c._3_1_,
//                                CONCAT11(switchD_140d2e62d::caseD_4c._2_1_,
//                                         switchD_140d2e62d::caseD_4c._1_1_))) | 0xb8e0ffc8;
//     switchD_140d2e62d::caseD_4c._1_1_ = (char)uVar41;
//     switchD_140d2e62d::caseD_4c._2_1_ = (undefined1)(uVar41 >> 8);
//     switchD_140d2e62d::caseD_4c._3_1_ = (char)(uVar41 >> 0x10);
//     UINT_1433da0e0._0_1_ = (undefined1)(uVar41 >> 0x18);
//     ppppppuStack_210 = (ulonglong ******)0x140d3bbdf;
//     lVar21 = func_0x000188d3bbdf();
//     pppppuVar38 = (ulonglong *****)&pppppuStack_208;
//     ppppppuVar60 = ppppppuVar55;
//     if (((ulonglong)unaff_R15 & 0xffffffff) < *(ulonglong *)((longlong)ppppppuVar55 + lVar21))
//     goto code_r0x000140d3bc81;
//     goto LAB_140d3bc98;
//   case 0x35:
//   case 0x38:
//     UNK_1433da067 = UNK_1433da067 + cVar20;
//     ppppppuVar52 = (ulonglong ******)&ppppppuStack_210;
//     ppppppuVar23 = (ulonglong ******)&ppppppuStack_210;
//     ppppppuStack_210 = (ulonglong ******)0x290d8d48;
//     if (ppppppuVar43 != (ulonglong ******)0x1) {
//       if (*(ulonglong ******)((longlong)ppppppuVar55 + 0x1433da0dc) <= unaff_R15)
//       goto LAB_140d3c04f;
//       pppppuVar33 = (ulonglong *****)0x0;
//       goto code_r0x000140d3c053;
//     }
// code_r0x000140d3bf7c:
//                     /* WARNING: Could not recover jumptable at 0x000140d3bf8e. Too many branches */
//                     /* WARNING: Treating indirect jump as call */
//     pppppuVar33 = (ulonglong *****)
//                   (*(code *)((longlong)&UINT_1433da1b0 +
//                             (longlong)(int)(&UINT_1433da1b0)[(longlong)ppppppuVar55[0xd]]))();
//     return pppppuVar33;
//   case 0x36:
//                     /* WARNING: Bad instruction - Truncating control flow here */
//     halt_baddata();
//   case 0x37:
//     *(byte *)param_2 = *(byte *)param_2 + 1;
//     goto code_r0x000140d3bf35;
//   case 0x39:
//   case 0x3c:
//     puVar22 = (uint *)0x432ff4dc;
//     ppppppuVar52 = &pppppuStack_208;
//     goto code_r0x000140d3bebb;
//   case 0x3a:
//   case 0x3b:
//                     /* WARNING: Bad instruction - Truncating control flow here */
//     halt_baddata();
//   case 0x3d:
//   case 0x42:
//     bVar71 = (uint)ppppppuVar43 < 0x27101;
//     goto code_r0x000140d3bedd;
//   case 0x3e:
//     switchD_140d2e62d::caseD_4c._1_1_ = switchD_140d2e62d::caseD_4c._1_1_ + cVar20;
//     return (ulonglong *****)puVar22;
//   case 0x3f:
//     puVar22 = &switchD_140d2e62d::caseD_4c;
//     ppppppuVar52 = &pppppuStack_208;
//     goto code_r0x000140d3bf03;
//   case 0x40:
//     iVar69 = CONCAT13(switchD_140d2e62d::caseD_4c._3_1_,
//                       CONCAT12(switchD_140d2e62d::caseD_4c._2_1_,
//                                CONCAT11(switchD_140d2e62d::caseD_4c._1_1_,
//                                         (char)switchD_140d2e62d::caseD_4c))) + 0x433da0dc;
//     switchD_140d2e62d::caseD_4c._0_1_ = (char)iVar69;
//     switchD_140d2e62d::caseD_4c._1_1_ = (char)((uint)iVar69 >> 8);
//     switchD_140d2e62d::caseD_4c._2_1_ = (undefined1)((uint)iVar69 >> 0x10);
//     switchD_140d2e62d::caseD_4c._3_1_ = (char)((uint)iVar69 >> 0x18);
//     UNK_1433da067 = UNK_1433da067 + cVar20;
//     pcVar59 = (char *)((ulonglong)ppppppuVar55 & 0xffffffff);
//     ppppppuVar55 = (ulonglong ******)0x433da0dc;
//     bVar39 = bRam00000000cc1bbf23 >> 3;
//     bRam00000000cc1bbf23 = bRam00000000cc1bbf23 << 5 | bVar39;
//     *(byte *)ppppppuVar43 = *(byte *)ppppppuVar43 + (char)pcVar59 + ((bVar39 & 1) != 0);
//     *pcVar59 = *pcVar59 + (char)pcVar59;
//     ppppppuVar52 = &pppppuStack_208;
//     goto LAB_140d3bf0f;
//   case 0x41:
//     ppppppuVar23 = (ulonglong ******)&ppppppuStack_210;
//     cRam00000000e40c0a25 = cRam00000000e40c0a25 + (char)param_2;
//     uVar1 = LocalDescriptorTableRegister();
//     switchD_140d2e62d::caseD_4c._0_1_ = (char)uVar1;
//     switchD_140d2e62d::caseD_4c._1_1_ = (char)((uint)uVar1 >> 8);
//     switchD_140d2e62d::caseD_4c._2_1_ = (undefined1)((uint)uVar1 >> 0x10);
//     switchD_140d2e62d::caseD_4c._3_1_ = (char)((uint)uVar1 >> 0x18);
//     ppppppuStack_210 = ppppppuVar60;
//     param_2 = unaff_R13;
//     goto code_r0x000140d3bf46;
//   case 0x43:
//   case 0x4a:
//   case 0x4b:
//     ppppppuStack_210 = &pppppuStack_208;
//     uVar35 = FUN_140c87850();
//     puVar22 = (uint *)(uVar35 & 0xffffffffffffff01);
//   case 0x44:
//   case 0x46:
//     return (ulonglong *****)puVar22;
//   case 0x45:
//     goto switchD_140d2e62d_caseD_45;
//   case 0x47:
//     ppppppuVar54 = (ulonglong ******)(ulonglong)((int)&pppppuStack_208 - 0x58);
//     puVar22 = (uint *)*param_2;
//     puVar46 = *(uint **)puVar22;
//     ppppppuVar55 = param_2;
//     ppppppuVar56 = ppppppuVar43;
//     goto switchD_140d2e62d_caseD_45;
//   case 0x48:
//     UNK_1433da065 = UNK_1433da065 + cVar20 * -2;
//     ppppppuVar54 = (ulonglong ******)&ppppppuStack_210;
//     ppppppuStack_210 = ppppppuVar55;
//     goto code_r0x000140d3c6b1;
//   case 0x49:
//     uVar35 = (ulonglong)ppppppuVar43 & 0xffffffff;
//     if (0xfffffffebcc25f23 <
//         (ulonglong)(longlong)(int)(&switchD_140d2e62d::caseD_4c)[(longlong)ppppppuVar70]) {
//       uVar35 = (ulonglong)param_2 & 0xffffffff;
//     }
//     do {
//       if (puVar62 <= (undefined8 *)(uVar35 * uVar35 + (longlong)puVar22 * (longlong)puVar22)) {
//         if ((auStack_128[0] != 0) &&
//            (*(ulonglong *****)(*(longlong *)(uStack_178 + 0x10) + 0x20) == ppppuStack_130)) {
//           *(ulonglong *****)(*(longlong *)(uStack_178 + 0x10) + 0x20) =
//                ppppuStack_130 + auStack_128[0];
//         }
//         return (ulonglong *****)0x1;
//       }
//       do {
//         do {
//           ppppppuVar55 = ppppppuVar55 + 1;
//           if (ppppppuVar70 == ppppppuVar55) {
//             do {
//               puVar58 = puStack_138;
//               lVar21 = lStack_140;
//               ppppppuVar60 = ppppppuStack_1d8;
//               if ((auStack_128[0] != 0) &&
//                  (*(ulonglong *****)(*(longlong *)(uStack_178 + 0x10) + 0x20) == ppppuStack_130)) {
//                 *(ulonglong *****)(*(longlong *)(uStack_178 + 0x10) + 0x20) =
//                      ppppuStack_130 + auStack_128[0];
//               }
//               do {
//                 ppppppuVar60 = (ulonglong ******)((longlong)ppppppuVar60 + 1);
//                 if (ppppppuVar60 == ppppppuStack_1c0) {
//                   if (*(char *)(uStack_168 + 0x6b9) != '\0') {
//                     ppppppuStack_210 = (ulonglong ******)0x140d3cb3b;
//                     FUN_141821530(&ppppuStack_160,puStack_138,1 - lStack_140,*puStack_170);
//                     if (uStack_148 != 0) {
//                       lVar37 = 0;
//                       do {
//                         lVar30 = *(longlong *)((longlong)ppppuStack_160 + lVar37);
//                         if (((*(int *)(lVar30 + 0x68) == 1) && (*(int *)(lVar30 + 0x88) == 1)) &&
//                            (*(longlong *)(lVar30 + 0x90) == *(longlong *)(uStack_168 + 0x5c0))) {  // 0x5c0=핸들
//                           if (CONCAT44(uStack_14c,uStack_150) == 0) {
//                             return (ulonglong *****)0x1;
//                           }
//                           if (*(ulonglong *****)
//                                (*(longlong *)((longlong)pppuStack_158 + 0x10) + 0x20) !=
//                               ppppuStack_160) {
//                             return (ulonglong *****)0x1;
//                           }
//                           *(ulonglong *****)(*(longlong *)((longlong)pppuStack_158 + 0x10) + 0x20) =
//                                ppppuStack_160 + CONCAT44(uStack_14c,uStack_150);
//                           return (ulonglong *****)0x1;
//                         }
//                         lVar37 = lVar37 + 8;
//                       } while (uStack_148 << 3 != lVar37);
//                     }
//                     if ((CONCAT44(uStack_14c,uStack_150) != 0) &&
//                        (*(ulonglong *****)(*(longlong *)((longlong)pppuStack_158 + 0x10) + 0x20) ==
//                         ppppuStack_160)) {
//                       *(ulonglong *****)(*(longlong *)((longlong)pppuStack_158 + 0x10) + 0x20) =
//                            ppppuStack_160 + CONCAT44(uStack_14c,uStack_150);
//                     }
//                   }
//                   if (puVar58[lVar21 * 4 + 0x29] == 0) {
//                     return (ulonglong *****)0x0;
//                   }
//                   uVar35 = *(ulonglong *)(*(longlong *)puVar58[lVar21 * 4 + 0x26] + 0x660);  // 0x660=x
//                   uVar34 = *(ulonglong *)(*(longlong *)puVar58[lVar21 * 4 + 0x26] + 0x668);  // 0x668=y
//                   uVar65 = *(ulonglong *)(uStack_168 + 0x660);  // 0x660=x
//                   uVar28 = *(ulonglong *)(uStack_168 + 0x668);  // 0x668=y
//                   lVar37 = uVar65 - uVar35;
//                   if (uVar65 < uVar35) {
//                     lVar37 = uVar35 - uVar65;
//                   }
//                   lVar30 = uVar28 - uVar34;
//                   if (uVar28 < uVar34) {
//                     lVar30 = uVar34 - uVar28;
//                   }
//                   bVar39 = (byte)puStack_170[7];
//                   if (8 < bVar39) {
//                     return (ulonglong *****)0x0;
//                   }
//                   uVar35 = lVar30 * lVar30 + lVar37 * lVar37;
//                   uVar34 = (ulonglong)(uint)((1 - (int)lVar21) * 0x20);
//                   uVar41 = (uint)bVar39;
//                   if ((0x185U >> (bVar39 & 0x1f) & 1) == 0) {
//                     if ((10U >> (bVar39 & 0x1f) & 1) != 0) goto code_r0x000140d3ce0f;
//                     if ((0x30U >> (uVar41 & 0x1f) & 1) == 0) {
//                       return (ulonglong *****)0x0;
//                     }
// code_r0x000140d3cd4a:
//                     if (puVar58[lVar21 + 0x34] == 0 && puVar58[lVar21 + 0x36] == 0) {
//                       lVar37 = *(longlong *)((longlong)puVar58 + uVar34 + 0x68);
//                       if (lVar37 != 0) {
//                         lVar21 = 0;
//                         uVar57 = 0;
//                         do {
//                           lVar30 = *(longlong *)
//                                     (*(longlong *)((longlong)puVar58 + uVar34 + 0x50) + lVar21 * 8);
//                           uVar48 = *(ulonglong *)(lVar30 + 0x660);  // 0x660=x
//                           uVar13 = *(ulonglong *)(lVar30 + 0x668);  // 0x668=y
//                           lVar30 = uVar65 - uVar48;
//                           if (uVar65 < uVar48) {
//                             lVar30 = uVar48 - uVar65;
//                           }
//                           lVar27 = uVar28 - uVar13;
//                           if (uVar28 < uVar13) {
//                             lVar27 = uVar13 - uVar28;
//                           }
//                           uVar57 = (uVar57 + 1) -
//                                    (ulonglong)
//                                    (uVar35 < (ulonglong)(lVar27 * lVar27 + lVar30 * lVar30));
//                           lVar21 = lVar21 + 1;
//                         } while (lVar37 != lVar21);
//                         lVar21 = lStack_140;
//                         puVar58 = puStack_138;
//                         if (5 < uVar57) {
//                           return (ulonglong *****)0x1;
//                         }
//                       }
//                     }
//                     if (8 < bVar39) {
//                       return (ulonglong *****)0x0;
//                     }
//                     uVar41 = 0x1ab >> (bVar39 & 0x1f);
//                   }
//                   else {
//                     if (puVar58[lVar21 + 0x30] == 0 && puVar58[lVar21 + 0x32] == 0) {
//                       lVar37 = *(longlong *)((longlong)puVar58 + uVar34 + 0x28);
//                       if (lVar37 != 0) {
//                         lVar21 = 0;
//                         uVar57 = 0;
//                         do {
//                           lVar30 = *(longlong *)
//                                     (*(longlong *)((longlong)puVar58 + uVar34 + 0x10) + lVar21 * 8);
//                           uVar48 = *(ulonglong *)(lVar30 + 0x660);  // 0x660=x
//                           uVar13 = *(ulonglong *)(lVar30 + 0x668);  // 0x668=y
//                           lVar30 = uVar65 - uVar48;
//                           if (uVar65 < uVar48) {
//                             lVar30 = uVar48 - uVar65;
//                           }
//                           lVar27 = uVar28 - uVar13;
//                           if (uVar28 < uVar13) {
//                             lVar27 = uVar13 - uVar28;
//                           }
//                           uVar57 = (uVar57 + 1) -
//                                    (ulonglong)
//                                    (uVar35 < (ulonglong)(lVar27 * lVar27 + lVar30 * lVar30));
//                           lVar21 = lVar21 + 1;
//                         } while (lVar37 != lVar21);
//                         lVar21 = lStack_140;
//                         puVar58 = puStack_138;
//                         if (5 < uVar57) {
//                           return (ulonglong *****)0x1;
//                         }
//                       }
//                     }
//                     if (8 < uVar41) {
//                       return (ulonglong *****)0x0;
//                     }
//                     if ((0x1b1U >> (uVar41 & 0x1f) & 1) != 0) goto code_r0x000140d3cd4a;
//                     uVar41 = 10 >> (bVar39 & 0x1f);
//                   }
//                   if ((uVar41 & 1) == 0) {
//                     return (ulonglong *****)0x0;
//                   }
// code_r0x000140d3ce0f:
//                   if (puVar58[lVar21 + 0x38] == 0 && puVar58[lVar21 + 0x3a] == 0) {
//                     lVar21 = *(longlong *)((longlong)puVar58 + uVar34 + 0xa8);
//                     if (lVar21 != 0) {
//                       lVar37 = 0;
//                       uVar57 = 0;
//                       do {
//                         lVar30 = *(longlong *)
//                                   (*(longlong *)((longlong)puVar58 + uVar34 + 0x90) + lVar37 * 8);
//                         uVar48 = *(ulonglong *)(lVar30 + 0x660);  // 0x660=x
//                         uVar13 = *(ulonglong *)(lVar30 + 0x668);  // 0x668=y
//                         lVar30 = uVar65 - uVar48;
//                         if (uVar65 < uVar48) {
//                           lVar30 = uVar48 - uVar65;
//                         }
//                         lVar27 = uVar28 - uVar13;
//                         if (uVar28 < uVar13) {
//                           lVar27 = uVar13 - uVar28;
//                         }
//                         uVar57 = (uVar57 + 1) -
//                                  (ulonglong)
//                                  (uVar35 < (ulonglong)(lVar27 * lVar27 + lVar30 * lVar30));
//                         lVar37 = lVar37 + 1;
//                       } while (lVar21 != lVar37);
//                       if (5 < uVar57) {
//                         return (ulonglong *****)0x1;
//                       }
//                     }
//                   }
//                   return (ulonglong *****)0x0;
//                 }
//                 bVar39 = *(byte *)ppppppuVar60;
//               } while (*(longlong *)(uStack_1b8 + 0x180 + (ulonglong)bVar39 * 0x20) != 0 ||
//                        *(longlong *)(uStack_1b8 + 400 + (ulonglong)bVar39 * 0x20) != 0);
//               ppppppuStack_188 = ppppppuStack_1d0;
//               pbStack_180 = pbStack_1c8;
//               if (bVar39 != 0) {
//                 ppppppuStack_188 = ppppppuStack_1b0;
//                 pbStack_180 = pbStack_1a8;
//                 if (bVar39 == 2) {
//                   ppppppuStack_188 = ppppppuStack_1a0;
//                   pbStack_180 = (byte *)CONCAT44(uStack_194,iStack_198);
//                 }
//               }
//               pppuStack_158 = (ulonglong ***)*puStack_170;
//               ppppuStack_160 = (ulonglong ****)0x8;
//               uStack_148 = CONCAT44(unaff_XMM6_Dd,unaff_XMM6_Dc);
//               lVar21 = 0;
//               lVar37 = 0;
//               ppppppuStack_1d8 = ppppppuVar60;
//               uStack_150 = unaff_XMM6_Da;
//               uStack_14c = unaff_XMM6_Db;
//               while (lVar37 != 0x28) {
//                 pppuVar11 = *(ulonglong ****)((longlong)ppppppuVar56 + lVar37);
//                 lVar37 = lVar37 + 8;
//                 if (pppuVar11 != (ulonglong ***)0x0) {
//                   if (lVar21 == CONCAT44(uStack_14c,uStack_150)) {
//                     ppppppuStack_210 = (ulonglong ******)0x140d3c91e;
//                     FUN_1410ea8e0(&ppppuStack_160,lVar21,1,1);
//                     lVar21 = uStack_148;
//                   }
//                   ppppuStack_160[lVar21] = pppuVar11;
//                   lVar21 = uStack_148 + 1;
//                   uStack_148 = lVar21;
//                 }
//               }
//               ppppuStack_130 = ppppuStack_160;
//               uStack_178 = (ulonglong)pppuStack_158;
//               auStack_128[0] = CONCAT44(uStack_14c,uStack_150);
//             } while (lVar21 == 0);
//             ppppppuVar70 = (ulonglong ******)(lVar21 << 3);
//             param_1 = (ulonglong *****)*puStack_138;
//             pppppuStack_190 = (ulonglong *****)puStack_138[1];
//             ppppuVar68 = pppppuStack_190[0x1f];
//             ppppppuVar55 = (ulonglong ******)0x0;
//           }
//           lVar21 = *(longlong *)((longlong)ppppppuVar55 + (longlong)ppppuStack_130);
//           uVar36 = *(undefined8 *)(lVar21 + 0x5c0);  // 0x5c0=핸들
//           ppppppuStack_210 = (ulonglong ******)0x140d3ca36;
//           cVar20 = (*(code *)ppppuVar68)(param_1,lStack_140,uVar36);
//           if (cVar20 != '\0') goto code_r0x000140d3c990;
//           ppppppuStack_210 = (ulonglong ******)0x140d3ca4f;
//           lVar37 = (*(code *)pppppuStack_190[0x2a])(param_1,uVar36);
//         } while (lVar37 == 0);
//         pppppuVar33 = ppppppuStack_1e0[(ulonglong)*(uint *)(lVar37 + 0x9c0) + 0x3c];  // 0x9c0=role
//         ppppppuStack_210 = (ulonglong ******)0x140d3ca71;
//         pppppuVar38 = (ulonglong *****)(*(code *)pppppuStack_190[5])(param_1);
//       } while (pppppuVar33 + 0xf < pppppuVar38);
// code_r0x000140d3c990:
//       ppppppuVar60 = *(ulonglong *******)(lVar21 + 0x660);  // 0x660=x
//       pbVar32 = *(byte **)(lVar21 + 0x668);  // 0x668=y
//       ppppppuVar52 = *(ulonglong *******)(uStack_168 + 0x660);  // 0x660=x
//       lVar21 = (longlong)ppppppuVar52 - (longlong)ppppppuVar60;
//       if (ppppppuVar52 < ppppppuVar60) {
//         lVar21 = (longlong)ppppppuVar60 - (longlong)ppppppuVar52;
//       }
//       pbVar12 = *(byte **)(uStack_168 + 0x668);  // 0x668=y
//       lVar37 = (longlong)pbVar12 - (longlong)pbVar32;
//       if (pbVar12 < pbVar32) {
//         lVar37 = (longlong)pbVar32 - (longlong)pbVar12;
//       }
//       puVar62 = (undefined8 *)(lVar37 * lVar37 + lVar21 * lVar21);
//       puVar22 = (uint *)((longlong)ppppppuVar52 - (longlong)ppppppuStack_188);
//       if (ppppppuVar52 < ppppppuStack_188) {
//         puVar22 = (uint *)((longlong)ppppppuStack_188 - (longlong)ppppppuVar52);
//       }
//       uVar35 = (longlong)pbVar12 - (longlong)pbStack_180;
//       if (pbVar12 < pbStack_180) {
//         uVar35 = (longlong)pbStack_180 - (longlong)pbVar12;
//       }
//     } while( true );
//   case 0x4c:
//                     /* WARNING: Bad instruction - Truncating control flow here */
//     halt_baddata();
//   }
//   uVar34 = *(ulonglong *)((longlong)param_2 + lVar21 + 0x28);
//   uVar35 = uVar34 * 8;
//   bVar71 = uVar34 >> 0x3d != 0;
//   if (0x7ffffffffffffff8 < uVar35 || bVar71) {
//     ppppppuStack_210 = (ulonglong ******)0x140d2e6b9;
//     FUN_1431a0cbf(0,uVar35);
//                     /* WARNING: Does not return */
//     pcVar16 = (code *)invalidInstructionException();
//     (*pcVar16)();
//   }
//   ppppuStack_58 = *(ulonglong *****)((longlong)param_2 + lVar21 + 0x20);
//   pppppuStack_88 = param_1;
//   if (uVar35 == 0) {
//     puVar66 = &DAT_00000008;
//     uStack_c0 = 0;
//   }
//   else {
//     ppppppuStack_210 = (ulonglong ******)0x140d2e6dd;
//     puVar66 = FUN_142b1b410((void *)(ulonglong)bVar71,0,uVar35);
//     uStack_c0 = uVar34;
//     if (puVar66 == (undefined1 *)0x0) {
//       ppppppuStack_210 = (ulonglong ******)0x140d2f092;
//       FUN_1431a0cbf(8,uVar35);
//                     /* WARNING: Does not return */
//       pcVar16 = (code *)invalidInstructionException();
//       (*pcVar16)();
//     }
//   }
//   puStack_b8 = puVar66;
//   uStack_98 = uStack_c0;
//   if (uVar34 != 0) {
//     bVar71 = ppppppuStack_60 != (ulonglong ******)0x0;
//     ppppppuStack_210 = (ulonglong ******)0x140d2e73c;
//     pvVar4 = ppppuStack_58;
//     ppppuStack_58 = ppppuVar68;
//     memcpy(puVar66,pvVar4,uVar35);
//     ppppppuStack_1d8 = (ulonglong ******)&ppppppuStack_a0;
//     ppppppuStack_1d0 = &pppppuStack_a8;
//     pbStack_1c8 = &bStack_4a;
//     ppppppuVar60 = (ulonglong ******)&ppppppuStack_1e0;
//     ppppppuStack_210 = (ulonglong ******)0x140d2e78d;
//     ppppppuStack_1e0 = (ulonglong ******)puVar46;
//     uStack_b0 = uVar34;
//     pppppuStack_a8 = (ulonglong *****)(ulonglong)bVar71;
//     ppppppuStack_a0 = ppppppuVar70;
//     bStack_4a = bVar39;
//     ppppppuVar23 = (ulonglong ******)FUN_140ffa3e0(&PTR_LAB_14344acc8);
//     ppppppuVar52 = (ulonglong ******)ppppuStack_58[0xcd];
//     lVar21 = (longlong)ppppppuVar23 - (longlong)ppppppuVar55;
//     if (ppppppuVar23 < ppppppuVar55) {
//       lVar21 = (longlong)ppppppuVar55 - (longlong)ppppppuVar23;
//     }
//     lVar37 = (longlong)ppppppuVar60 - (longlong)ppppppuVar52;
//     if (ppppppuVar60 < ppppppuVar52) {
//       lVar37 = (longlong)ppppppuVar52 - (longlong)ppppppuVar60;
//     }
//     ppppuVar68 = ppppuStack_58;
//     if ((ulonglong)(lVar37 * lVar37 + lVar21 * lVar21) < 0x35a4e9001) {
//       pcVar16 = *(code **)(lStack_90 + 0x1f0);
//       uVar35 = 0;
//       uVar65 = 0;
//       do {
//         ppppppuStack_210 = (ulonglong ******)0x140d2e80e;
//         lVar21 = (*pcVar16)(uStack_78);
//         if (lVar21 != 0) {
//           if (*(int *)(lVar21 + 0x4c0) == -1) {
//             uVar28 = 0;
//           }
//           else {
//             ppppuStack_1e8 = ppppuStack_58;
//             ppppppuStack_210 = (ulonglong ******)0x140d2e84c;
//             lVar37 = FUN_1412857f0(lVar21 + 0x490,pppppuStack_68,lVar21,&PTR_FUN_1433d8768);  // 0x490=어빌슬롯 base(+k*0x38)
//             ppppppuStack_210 = (ulonglong ******)0x140d2e867;
//             lVar30 = (**(code **)(*(longlong *)(lVar21 + 0x578) + 0x90))
//                                (*(undefined8 *)(lVar21 + 0x570),lVar21);
//             uVar41 = *(int *)(lVar21 + 0x3fc) + 100;
//             uVar28 = (ulonglong)uVar41;
//             if ((int)uVar41 < 2) {
//               uVar28 = 1;
//             }
//             uVar57 = lVar30 * 100;
//             if (uVar57 >> 0x20 == 0) {
//               uVar57 = uVar57 & 0xffffffff;
//             }
//             uVar57 = uVar57 / uVar28;
//             if (uVar57 < 4) {
//               uVar57 = 3;
//             }
//             uVar28 = lVar37 * 1000;
//             if ((uVar28 | uVar57) >> 0x20 == 0) {
//               uVar28 = (uVar28 & 0xffffffff) / (uVar57 & 0xffffffff);
//             }
//             else {
//               uVar28 = uVar28 / uVar57;
//             }
//           }
//           uVar35 = uVar35 + uVar28;
//         }
//         uVar65 = uVar65 + 1;
//       } while (uVar65 != uVar34);
//       ppppuVar68 = ppppuStack_58;
//       if (uVar35 == 0) {
//         if (pppppuStack_68[1][0x25f] != (ulonglong ***)0xffffffffffffffff)
//         goto code_r0x000140d2e910;
//       }
//       else {
//         uVar34 = (longlong)ppppuStack_58[0xce] * 1000;
//         if ((uVar34 | uVar35) >> 0x20 == 0) {
//           if (pppppuStack_68[1][0x25f] <
//               (ulonglong ***)((uVar34 & 0xffffffff) / (uVar35 & 0xffffffff)))
//           goto code_r0x000140d2e910;
//         }
//         else if (pppppuStack_68[1][0x25f] < (ulonglong ***)(uVar34 / uVar35)) {
// code_r0x000140d2e910:
//           ppppppuStack_210 = (ulonglong ******)0x140d2e916;
//           pvVar24 = GetProcessHeap();
//           ppppppuStack_210 = (ulonglong ******)0x140d2e924;
//           HeapFree(pvVar24,0,puVar66);
// code_r0x000140d2ed73:
//           pppppuStack_88[1] = (ulonglong ****)ppppppuStack_60;
//           *(byte *)(pppppuStack_88 + 2) = bStack_49;
//           *(char *)((longlong)pppppuStack_88 + 0x11) = '\0';
//           ppppuVar42 = (ulonglong ****)0x6;
//           puVar22 = (uint *)pppppuStack_88;
// LAB_140d2ed85:
//           *(ulonglong *****)puVar22 = ppppuVar42;
// switchD_140d2e62d_caseD_10:
//           return (ulonglong *****)puVar22;
//         }
//       }
//     }
//   }
//   ppppuStack_58 = ppppuVar68;
//   if (uStack_98 != 0) {
//     ppppppuStack_210 = (ulonglong ******)0x140d2e969;
//     pvVar24 = GetProcessHeap();
//     ppppppuStack_210 = (ulonglong ******)0x140d2e977;
//     HeapFree(pvVar24,0,puVar66);
//   }
//   ppppuVar68 = ppppuStack_58;
//   pppppuVar33 = pppppuStack_68;
//   if (*(int *)(ppppuStack_58 + 0x7e) < 1) {
//     if (*(int *)(ppppuStack_58 + 0x9f) == -1) {
// code_r0x000140d2ea8b:
//       ppppuVar42 = (ulonglong ****)&DAT_1433d8898;
//       if ((ulonglong ***)0x2 < ppppuVar68[0xb9]) {
//         ppppuVar42 = ppppuVar68 + 0xa0;
//       }
//       if (*(int *)(ppppuVar42 + 6) != -1) {
//         uVar35 = 0;
//         if (ppppuVar68[0xce] <= ppppuVar68[0xc5]) {
//           uVar35 = (longlong)ppppuVar68[0xc5] - (longlong)ppppuVar68[0xce];
//         }
//         ppppppuStack_210 = (ulonglong ******)0x140d2eaf0;
//         uVar34 = (*(code *)ppppuVar42[1][8])
//                            ((longlong)*ppppuVar42 +
//                             ((longlong)ppppuVar42[1][2] - 1U & 0xfffffffffffffff0) + 0x10,
//                             pppppuVar33,ppppuVar68,&PTR_FUN_1433d8768);
//         if (uVar34 <= uVar35) {
//           uVar35 = uVar34;
//         }
//         if (uVar35 == 0) {
//           ppppuStack_1e8 = (ulonglong ****)&PTR_FUN_1433d8768;
//           ppppppuStack_210 = (ulonglong ******)0x140d2eb34;
//           (*(code *)ppppuVar42[1][0x14])
//                     (&ppppppuStack_1e0,
//                      (longlong)*ppppuVar42 +
//                      ((longlong)ppppuVar42[1][2] - 1U & 0xfffffffffffffff0) + 0x10,pppppuVar33,
//                      ppppuStack_58);
//         }
//       }
//     }
//     else {
//       uVar35 = 0;
//       if (ppppuStack_58[0xce] <= ppppuStack_58[0xc5]) {
//         uVar35 = (longlong)ppppuStack_58[0xc5] - (longlong)ppppuStack_58[0xce];
//       }
//       ppppppuStack_210 = (ulonglong ******)0x140d2ea34;
//       uVar34 = (*(code *)ppppuStack_58[0x9a][8])
//                          ((longlong)ppppuStack_58[0x99] +
//                           ((longlong)ppppuStack_58[0x9a][2] - 1U & 0xfffffffffffffff0) + 0x10,
//                           pppppuStack_68,ppppuStack_58,&PTR_FUN_1433d8768);
//       if (uVar34 <= uVar35) {
//         uVar35 = uVar34;
//       }
//       if (uVar35 == 0) {
//         ppppuStack_1e8 = (ulonglong ****)&PTR_FUN_1433d8768;
//         ppppppuStack_210 = (ulonglong ******)0x140d2ea7b;
//         (*(code *)ppppuVar68[0x9a][0x14])
//                   (&ppppppuStack_1e0,
//                    (longlong)ppppuVar68[0x99] +
//                    ((longlong)ppppuVar68[0x9a][2] - 1U & 0xfffffffffffffff0) + 0x10,pppppuVar33,
//                    ppppuVar68);
//         if ((iStack_198 == -1) || ((int)ppppuStack_160 < 1)) goto code_r0x000140d2ea8b;
//       }
//     }
//   }
//   ppppppuStack_210 = (ulonglong ******)0x140d2e99f;
//   lVar21 = (*pcStack_70)(uStack_78);
//   if (lVar21 != 0) {
//     ppppppuStack_210 = (ulonglong ******)0x140d2f045;
//     FUN_1431a37c0(&PTR_s_game_ai_src_plan_legacy_old_pass_1433d90a0);
//                     /* WARNING: Does not return */
//     pcVar16 = (code *)invalidInstructionException();
//     (*pcVar16)();
//   }
//   puVar46 = &UINT_1433da0f4;
// code_r0x000140d2e9af:
//                     /* WARNING: Could not recover jumptable at 0x000140d2e9bd. Too many branches */
//                     /* WARNING: Treating indirect jump as call */
//   pppppuVar33 = (ulonglong *****)
//                 (*(code *)((longlong)(int)puVar46[(longlong)ppppppuStack_80] + (longlong)puVar46))
//                           ((code *)((longlong)(int)puVar46[(longlong)ppppppuStack_80] +
//                                    (longlong)puVar46));
//   return pppppuVar33;
// switchD_140d2e62d_caseD_8:
//   pbVar32 = (byte *)((longlong)puVar46 + 0x4100287d);
//   *pbVar32 = *pbVar32 << 7 | *pbVar32 >> 1;
//                     /* WARNING: Bad instruction - Truncating control flow here */
//   halt_baddata();
// LAB_140d3bb69:
//   *(undefined8 *)((longlong)ppppppuVar52 + -8) = 0x140d3bb7c;
//   puVar22 = (uint *)FUN_140d31bb0(uStack_78);
//   pppppuVar53 = (ulonglong *****)ppppppuVar52;
//   if ((ulonglong *****)puVar22 == (ulonglong *****)0x0) {
//     *(undefined8 *)((longlong)ppppppuVar52 + -8) = 0x140d3c475;
//     FUN_1431a37c0(&PTR_s_game_ai_src_plan_legacy_old_pass_1433d97a0);
//   }
//   else {
// code_r0x000140d3bb85:
//     ppppppuVar70 = *(ulonglong *******)((longlong)puVar22 + 0x930);  // 0x930=side
//     pppppuVar61 = pppppuVar53;
//     if (ppppppuVar70 < (ulonglong ******)0x2) {
// code_r0x000140d3bb96:
//       uVar41 = *(uint *)((longlong)puVar22 + 0x9c0);  // 0x9c0=role
//       *(undefined8 *)((longlong)pppppuVar61 + -8) = 0x140d3bba4;
//       cVar20 = FUN_14128cc90(ppppppuVar55);
//       puVar46 = (uint *)(ppppppuStack_a0 + (longlong)ppppppuVar70 * 500 + (ulonglong)uVar41 * 100);
//       bVar71 = cVar20 == '\0';
// switchD_140d2e62d_caseD_29:
//       pppppuVar38 = pppppuVar61;
//       ppppppuVar60 = ppppppuVar55;
//       if (bVar71) {
//                     /* WARNING: Could not recover jumptable at 0x000140d3bbd7. Too many branches */
//                     /* WARNING: Treating indirect jump as call */
//         pppppuVar33 = (ulonglong *****)
//                       (*(code *)((longlong)&UINT_1433da178 +
//                                 (longlong)(int)(&UINT_1433da178)[(longlong)ppppppuVar55[0xd]]))();
//         return pppppuVar33;
//       }
//       goto LAB_140d3bc98;
//     }
//     *(undefined8 *)((longlong)pppppuVar53 + -8) = 0x140d3c4c7;
//     FUN_1431a3863(ppppppuVar70,2,&PTR_s_game_ai_src_plan_legacy_old_pass_1433d97b8);
//   }
// LAB_140d3c4df:
//                     /* WARNING: Does not return */
//   pcVar16 = (code *)invalidInstructionException();
//   (*pcVar16)();
// switchD_140d2e62d_caseD_45:
//   ppppuVar68 = *(ulonglong *****)((longlong)puVar22 + 8);
//   pppuVar11 = ppppuVar68[4];
//   *(undefined8 *)((longlong)ppppppuVar54 + -8) = 0x140d3c6a0;
//   uVar36 = (*(code *)pppuVar11)(puVar46);
//   *(undefined8 *)((longlong)ppppppuVar54 + 0x28) = uVar36;
//   pppuVar11 = ppppuVar68[5];
//   *(undefined8 *)((longlong)ppppppuVar54 + -8) = 0x140d3c6ac;
//   uVar36 = (*(code *)pppuVar11)(puVar46);
//   *(undefined8 *)((longlong)ppppppuVar54 + 0x30) = uVar36;
// code_r0x000140d3c6b1:
//   *(undefined1 **)((longlong)ppppppuVar54 + 0x38) = (undefined1 *)((longlong)ppppppuVar54 + 0x28);
//   *(undefined1 **)((longlong)ppppppuVar54 + 0x40) = (undefined1 *)((longlong)ppppppuVar54 + 0x30);
//   *(ulonglong *******)((longlong)ppppppuVar54 + 0x48) = ppppppuVar56;
//   *(ulonglong *******)((longlong)ppppppuVar54 + 0x50) = ppppppuVar55;
//   *(undefined8 *)((longlong)ppppppuVar54 + -8) = 0x140d3c6e0;
//   uVar35 = FUN_140c87850(&PTR_LAB_1433d97d0,(undefined1 *)((longlong)ppppppuVar54 + 0x38));
//   return (ulonglong *****)(ulonglong)((uVar35 & 0x10001) != 0);
// }
//
// /* === 보강: jumptable 미복구 구간 (Ghidra "Too many branches") ===
//    ※ case 목록 = rip-rel lea 테이블 휴리스틱 디코드(서명 4B 오프셋, .text 이탈 시 종료).
//    ※ arms=0 은 스위치 테이블이 아니라 간접 tail-call(fmt/vtable) 일 수 있다.
//    JT @ 0x140d2e9bd  table=0x1433da0f4  arms=70
//      case   0 -> 0x140d2e9bf
//      case   1 -> 0x140d2eb82
//      case   2 -> 0x140d2eb52
//      case   3 -> 0x140d2eb6a
//      case   4 -> 0x140d2eb4b
//      case   5 -> 0x140d2eb9a
//      case   6 -> 0x140d2ecc6
//      case   7 -> 0x140d2edd7
//      case   8 -> 0x140d2edb1
//      case   9 -> 0x140d2edc4
//      case  10 -> 0x140d2edaa
//      case  11 -> 0x140d2edea
//      case  12 -> 0x140d322d4
//      case  13 -> 0x140d323d8
//      case  14 -> 0x140d322f2
//      case  15 -> 0x140d323c5
//      case  16 -> 0x140d3803b
//      case  17 -> 0x140d38093
//      case  18 -> 0x140d38010
//      case  19 -> 0x140d38010
//      case  20 -> 0x140d38010
//      case  21 -> 0x140d38010
//      case  22 -> 0x140d3803b
//      case  23 -> 0x140d38062
//      case  24 -> 0x140d3803b
//      case  25 -> 0x140d380e5
//      case  26 -> 0x140d399ee
//      case  27 -> 0x140d399ee
//      case  28 -> 0x140d399ee
//      case  29 -> 0x140d399e8
//      case  30 -> 0x140d399e8
//      case  31 -> 0x140d399e8
//      case  32 -> 0x140d399e8
//      case  33 -> 0x140d3bc14
//      case  34 -> 0x140d3bbc3
//      case  35 -> 0x140d3bbd7
//      case  36 -> 0x140d3bc14
//      case  37 -> 0x140d3bb55
//      case  38 -> 0x140d3bb87
//      case  39 -> 0x140d3bb87
//      case  40 -> 0x140d3bb55
//      case  41 -> 0x140d3bb70
//      case  42 -> 0x140d3bc02
//      case  43 -> 0x140d3bb9b
//      case  44 -> 0x140d3bbaf
//      case  45 -> 0x140d3bbeb
//      case  46 -> 0x140d3bb70
//      case  47 -> 0x140d3bf93
//      case  48 -> 0x140d3bf42
//      case  49 -> 0x140d3bf56
//      case  50 -> 0x140d3bf93
//      case  51 -> 0x140d3bed4
//      case  52 -> 0x140d3bf06
//      case  53 -> 0x140d3bf06
//      case  54 -> 0x140d3bed4
//      case  55 -> 0x140d3beef
//      case  56 -> 0x140d3bf81
//      case  57 -> 0x140d3bf1a
//      case  58 -> 0x140d3bf2e
//      case  59 -> 0x140d3bf6a
//      case  60 -> 0x140d3beef
//      case  61 -> 0x140d3c680
//      case  62 -> 0x140d3c68f
//      case  63 -> 0x140d3c6ad
//      case  64 -> 0x140d3c68f
//      case  65 -> 0x140d3c69e
//      case  66 -> 0x140d3c6bc
//      case  67 -> 0x140d3ca12
//      case  68 -> 0x140d3c680
//      case  69 -> 0x140d3c680
//    JT @ 0x140d2ecdc  table=0x1433da10c  arms=64
//      case   0 -> 0x140d2ecde
//      case   1 -> 0x140d2edef
//      case   2 -> 0x140d2edc9
//      case   3 -> 0x140d2eddc
//      case   4 -> 0x140d2edc2
//      case   5 -> 0x140d2ee02
//      case   6 -> 0x140d322ec
//      case   7 -> 0x140d323f0
//      case   8 -> 0x140d3230a
//      case   9 -> 0x140d323dd
//      case  10 -> 0x140d38053
//      case  11 -> 0x140d380ab
//      case  12 -> 0x140d38028
//      case  13 -> 0x140d38028
//      case  14 -> 0x140d38028
//      case  15 -> 0x140d38028
//      case  16 -> 0x140d38053
//      case  17 -> 0x140d3807a
//      case  18 -> 0x140d38053
//      case  19 -> 0x140d380fd
//      case  20 -> 0x140d39a06
//      case  21 -> 0x140d39a06
//      case  22 -> 0x140d39a06
//      case  23 -> 0x140d39a00
//      case  24 -> 0x140d39a00
//      case  25 -> 0x140d39a00
//      case  26 -> 0x140d39a00
//      case  27 -> 0x140d3bc2c
//      case  28 -> 0x140d3bbdb
//      case  29 -> 0x140d3bbef
//      case  30 -> 0x140d3bc2c
//      case  31 -> 0x140d3bb6d
//      case  32 -> 0x140d3bb9f
//      case  33 -> 0x140d3bb9f
//      case  34 -> 0x140d3bb6d
//      case  35 -> 0x140d3bb88
//      case  36 -> 0x140d3bc1a
//      case  37 -> 0x140d3bbb3
//      case  38 -> 0x140d3bbc7
//      case  39 -> 0x140d3bc03
//      case  40 -> 0x140d3bb88
//      case  41 -> 0x140d3bfab
//      case  42 -> 0x140d3bf5a
//      case  43 -> 0x140d3bf6e
//      case  44 -> 0x140d3bfab
//      case  45 -> 0x140d3beec
//      case  46 -> 0x140d3bf1e
//      case  47 -> 0x140d3bf1e
//      case  48 -> 0x140d3beec
//      case  49 -> 0x140d3bf07
//      case  50 -> 0x140d3bf99
//      case  51 -> 0x140d3bf32
//      case  52 -> 0x140d3bf46
//      case  53 -> 0x140d3bf82
//      case  54 -> 0x140d3bf07
//      case  55 -> 0x140d3c698
//      case  56 -> 0x140d3c6a7
//      case  57 -> 0x140d3c6c5
//      case  58 -> 0x140d3c6a7
//      case  59 -> 0x140d3c6b6
//      case  60 -> 0x140d3c6d4
//      case  61 -> 0x140d3ca2a
//      case  62 -> 0x140d3c698
//      case  63 -> 0x140d3c698
//    JT @ 0x140d32302  table=0x1433da124  arms=58
//      case   0 -> 0x140d32304
//      case   1 -> 0x140d32408
//      case   2 -> 0x140d32322
//      case   3 -> 0x140d323f5
//      case   4 -> 0x140d3806b
//      case   5 -> 0x140d380c3
//      case   6 -> 0x140d38040
//      case   7 -> 0x140d38040
//      case   8 -> 0x140d38040
//      case   9 -> 0x140d38040
//      case  10 -> 0x140d3806b
//      case  11 -> 0x140d38092
//      case  12 -> 0x140d3806b
//      case  13 -> 0x140d38115
//      case  14 -> 0x140d39a1e
//      case  15 -> 0x140d39a1e
//      case  16 -> 0x140d39a1e
//      case  17 -> 0x140d39a18
//      case  18 -> 0x140d39a18
//      case  19 -> 0x140d39a18
//      case  20 -> 0x140d39a18
//      case  21 -> 0x140d3bc44
//      case  22 -> 0x140d3bbf3
//      case  23 -> 0x140d3bc07
//      case  24 -> 0x140d3bc44
//      case  25 -> 0x140d3bb85
//      case  26 -> 0x140d3bbb7
//      case  27 -> 0x140d3bbb7
//      case  28 -> 0x140d3bb85
//      case  29 -> 0x140d3bba0
//      case  30 -> 0x140d3bc32
//      case  31 -> 0x140d3bbcb
//      case  32 -> 0x140d3bbdf
//      case  33 -> 0x140d3bc1b
//      case  34 -> 0x140d3bba0
//      case  35 -> 0x140d3bfc3
//      case  36 -> 0x140d3bf72
//      case  37 -> 0x140d3bf86
//      case  38 -> 0x140d3bfc3
//      case  39 -> 0x140d3bf04
//      case  40 -> 0x140d3bf36
//      case  41 -> 0x140d3bf36
//      case  42 -> 0x140d3bf04
//      case  43 -> 0x140d3bf1f
//      case  44 -> 0x140d3bfb1
//      case  45 -> 0x140d3bf4a
//      case  46 -> 0x140d3bf5e
//      case  47 -> 0x140d3bf9a
//      case  48 -> 0x140d3bf1f
//      case  49 -> 0x140d3c6b0
//      case  50 -> 0x140d3c6bf
//      case  51 -> 0x140d3c6dd
//      case  52 -> 0x140d3c6bf
//      case  53 -> 0x140d3c6ce
//      case  54 -> 0x140d3c6ec
//      case  55 -> 0x140d3ca42
//      case  56 -> 0x140d3c6b0
//      case  57 -> 0x140d3c6b0
//    JT @ 0x140d38079  table=0x1433da134  arms=10  (cmp/ja bound=10 적용; 초과분은 인접 테이블 과읽)
//      case   0 -> 0x140d3807b
//      case   1 -> 0x140d380d3
//      case   2 -> 0x140d38050
//      case   3 -> 0x140d38050
//      case   4 -> 0x140d38050
//      case   5 -> 0x140d38050
//      case   6 -> 0x140d3807b
//      case   7 -> 0x140d380a2
//      case   8 -> 0x140d3807b
//      case   9 -> 0x140d38125
//    JT @ 0x140d39a4e  table=0x1433da15c  arms=44
//      case   0 -> 0x140d39a56
//      case   1 -> 0x140d39a56
//      case   2 -> 0x140d39a56
//      case   3 -> 0x140d39a50
//      case   4 -> 0x140d39a50
//      case   5 -> 0x140d39a50
//      case   6 -> 0x140d39a50
//      case   7 -> 0x140d3bc7c
//      case   8 -> 0x140d3bc2b
//      case   9 -> 0x140d3bc3f
//      case  10 -> 0x140d3bc7c
//      case  11 -> 0x140d3bbbd
//      case  12 -> 0x140d3bbef
//      case  13 -> 0x140d3bbef
//      case  14 -> 0x140d3bbbd
//      case  15 -> 0x140d3bbd8
//      case  16 -> 0x140d3bc6a
//      case  17 -> 0x140d3bc03
//      case  18 -> 0x140d3bc17
//      case  19 -> 0x140d3bc53
//      case  20 -> 0x140d3bbd8
//      case  21 -> 0x140d3bffb
//      case  22 -> 0x140d3bfaa
//      case  23 -> 0x140d3bfbe
//      case  24 -> 0x140d3bffb
//      case  25 -> 0x140d3bf3c
//      case  26 -> 0x140d3bf6e
//      case  27 -> 0x140d3bf6e
//      case  28 -> 0x140d3bf3c
//      case  29 -> 0x140d3bf57
//      case  30 -> 0x140d3bfe9
//      case  31 -> 0x140d3bf82
//      case  32 -> 0x140d3bf96
//      case  33 -> 0x140d3bfd2
//      case  34 -> 0x140d3bf57
//      case  35 -> 0x140d3c6e8
//      case  36 -> 0x140d3c6f7
//      case  37 -> 0x140d3c715
//      case  38 -> 0x140d3c6f7
//      case  39 -> 0x140d3c706
//      case  40 -> 0x140d3c724
//      case  41 -> 0x140d3ca7a
//      case  42 -> 0x140d3c6e8
//      case  43 -> 0x140d3c6e8
//    JT @ 0x140d3bbd7  table=0x1433da178  arms=37
//      case   0 -> 0x140d3bc98
//      case   1 -> 0x140d3bc47
//      case   2 -> 0x140d3bc5b
//      case   3 -> 0x140d3bc98
//      case   4 -> 0x140d3bbd9
//      case   5 -> 0x140d3bc0b
//      case   6 -> 0x140d3bc0b
//      case   7 -> 0x140d3bbd9
//      case   8 -> 0x140d3bbf4
//      case   9 -> 0x140d3bc86
//      case  10 -> 0x140d3bc1f
//      case  11 -> 0x140d3bc33
//      case  12 -> 0x140d3bc6f
//      case  13 -> 0x140d3bbf4
//      case  14 -> 0x140d3c017
//      case  15 -> 0x140d3bfc6
//      case  16 -> 0x140d3bfda
//      case  17 -> 0x140d3c017
//      case  18 -> 0x140d3bf58
//      case  19 -> 0x140d3bf8a
//      case  20 -> 0x140d3bf8a
//      case  21 -> 0x140d3bf58
//      case  22 -> 0x140d3bf73
//      case  23 -> 0x140d3c005
//      case  24 -> 0x140d3bf9e
//      case  25 -> 0x140d3bfb2
//      case  26 -> 0x140d3bfee
//      case  27 -> 0x140d3bf73
//      case  28 -> 0x140d3c704
//      case  29 -> 0x140d3c713
//      case  30 -> 0x140d3c731
//      case  31 -> 0x140d3c713
//      case  32 -> 0x140d3c722
//      case  33 -> 0x140d3c740
//      case  34 -> 0x140d3ca96
//      case  35 -> 0x140d3c704
//      case  36 -> 0x140d3c704
//    JT @ 0x140d3bf8e  table=0x1433da1b0  arms=23
//      case   0 -> 0x140d3c04f
//      case   1 -> 0x140d3bffe
//      case   2 -> 0x140d3c012
//      case   3 -> 0x140d3c04f
//      case   4 -> 0x140d3bf90
//      case   5 -> 0x140d3bfc2
//      case   6 -> 0x140d3bfc2
//      case   7 -> 0x140d3bf90
//      case   8 -> 0x140d3bfab
//      case   9 -> 0x140d3c03d
//      case  10 -> 0x140d3bfd6
//      case  11 -> 0x140d3bfea
//      case  12 -> 0x140d3c026
//      case  13 -> 0x140d3bfab
//      case  14 -> 0x140d3c73c
//      case  15 -> 0x140d3c74b
//      case  16 -> 0x140d3c769
//      case  17 -> 0x140d3c74b
//      case  18 -> 0x140d3c75a
//      case  19 -> 0x140d3c778
//      case  20 -> 0x140d3cace
//      case  21 -> 0x140d3c73c
//      case  22 -> 0x140d3c73c
//
//    --- capstone 선형 디스어셈 (본체 0xd2e500~0xd2f0b1) ---
//    140d2e500  push     rbp
//    140d2e501  push     r15
//    140d2e503  push     r14
//    140d2e505  push     r13
//    140d2e507  push     r12
//    140d2e509  push     rsi
//    140d2e50a  push     rdi
//    140d2e50b  push     rbx
//    140d2e50c  sub      rsp, 0x1c8
//    140d2e513  lea      rbp, [rsp + 0x80]
//    140d2e51b  mov      qword ptr [rbp + 0x140], 0xfffffffffffffffe
//    140d2e526  mov      r15, rcx
//    140d2e529  mov      r9, qword ptr [rbp + 0x1b0]
//    140d2e530  mov      rcx, qword ptr [r9 + 0x930]
//    140d2e537  cmp      rcx, 2
//    140d2e53b  jae      0x140d2f072
//    140d2e541  mov      r8, qword ptr [rbp + 0x1b8]
//    140d2e548  mov      rax, qword ptr [r8]
//    140d2e54b  mov      r9d, dword ptr [r9 + 0x9c0]
//    140d2e552  lea      r10, [rcx + rcx*4]
//    140d2e556  lea      r10, [rax + r10*8]
//    140d2e55a  mov      r13, qword ptr [r10 + r9*8 + 0x1e0]
//    140d2e562  test     r13, r13
//    140d2e565  je       0x140d2f047
//    140d2e56b  mov      rdi, qword ptr [rdx + 0x48]
//    140d2e56f  movzx    r12d, byte ptr [rdx + 0x60]
//    140d2e574  mov      r14, qword ptr [r8 + 8]
//    140d2e578  mov      rbx, qword ptr [r14 + 0x20]
//    140d2e57c  shl      rcx, 5
//    140d2e580  mov      rsi, qword ptr [r13 + 0x660]
//    140d2e587  cmp      rsi, qword ptr [rbx + rcx + 0x6d70]
//    140d2e58f  jb       0x140d2e5d1
//    140d2e591  cmp      rsi, qword ptr [rbx + rcx + 0x6d80]
//    140d2e599  ja       0x140d2e5d1
//    140d2e59b  add      rcx, rbx
//    140d2e59e  add      rcx, 0x6d70
//    140d2e5a5  mov      rdx, qword ptr [r13 + 0x668]
//    140d2e5ac  cmp      rdx, qword ptr [rcx + 8]
//    140d2e5b0  jb       0x140d2e5d1
//    140d2e5b2  cmp      rdx, qword ptr [rcx + 0x18]
//    140d2e5b6  ja       0x140d2e5d1
//    140d2e5b8  mov      rdx, qword ptr [r13 + 0x670]
//    140d2e5bf  mov      ecx, 5
//    140d2e5c4  cmp      rdx, qword ptr [r13 + 0x628]
//    140d2e5cb  jb       0x140d2ed85
//    140d2e5d1  mov      rcx, qword ptr [rax]
//    140d2e5d4  mov      rax, qword ptr [rax + 8]
//    140d2e5d8  mov      qword ptr [rbp + 0xf8], rax
//    140d2e5df  mov      rax, qword ptr [rax + 0x40]
//    140d2e5e3  mov      qword ptr [rbp + 0x110], rcx
//    140d2e5ea  mov      qword ptr [rbp + 0x118], rax
//    140d2e5f1  call     rax
//    140d2e5f3  test     rax, rax
//    140d2e5f6  jne      0x140d2f02b
//    140d2e5fc  mov      qword ptr [rbp + 0x120], r14
//    140d2e603  movzx    r14d, r12b
//    140d2e607  mov      ecx, r14d
//    140d2e60a  lea      rax, [rip + 0x26abacb]
//    140d2e611  mov      qword ptr [rbp + 0x108], rcx
//    140d2e618  movsxd   rcx, dword ptr [rax + rcx*4]
//    140d2e61c  add      rcx, rax
//    140d2e61f  mov      qword ptr [rbp + 0x128], rdi
//    140d2e626  mov      byte ptr [rbp + 0x13f], r12b
//    140d2e62d  jmp      rcx
//    140d2e62f  test     rdi, rdi
//    140d2e632  mov      eax, 0xc0
//    140d2e637  cmove    rax, rdi
//    140d2e63b  jmp      0x140d2e682
//    140d2e63d  mov      eax, 0x180
//    140d2e642  jmp      0x140d2e682
//    140d2e644  test     rdi, rdi
//    140d2e647  mov      ecx, 0x120
//    140d2e64c  mov      eax, 0x60
//    140d2e651  cmovne   rax, rcx
//    140d2e655  jmp      0x140d2e682
//    140d2e657  test     rdi, rdi
//    140d2e65a  mov      ecx, 0x150
//    140d2e65f  mov      eax, 0x90
//    140d2e664  cmovne   rax, rcx
//    140d2e668  jmp      0x140d2e682
//    140d2e66a  test     rdi, rdi
//    140d2e66d  mov      ecx, 0xf0
//    140d2e672  mov      eax, 0x30
//    140d2e677  cmovne   rax, rcx
//    140d2e67b  jmp      0x140d2e682
//    140d2e67d  mov      eax, 0x1b0
//    140d2e682  mov      rdi, qword ptr [rdx + rax + 0x28]
//    140d2e687  lea      r12, [rdi*8]
//    140d2e68f  mov      rcx, rdi
//    140d2e692  shr      rcx, 0x3d
//    140d2e696  setne    cl
//    140d2e699  movabs   r8, 0x7ffffffffffffff8
//    140d2e6a3  cmp      r12, r8
//    140d2e6a6  seta     r8b
//    140d2e6aa  or       r8b, cl
//    140d2e6ad  je       0x140d2e6bb
//    140d2e6af  xor      ecx, ecx
//    140d2e6b1  mov      rdx, r12
//    140d2e6b4  call     0x1431a0cbf
//    140d2e6b9  ud2      
//    140d2e6bb  mov      qword ptr [rbp + 0x100], r15
//    140d2e6c2  mov      rdx, qword ptr [rdx + rax + 0x20]
//    140d2e6c7  test     r12, r12
//    140d2e6ca  je       0x140d2e6f5
//    140d2e6cc  mov      qword ptr [rbp + 0x130], rdx
//    140d2e6d3  xor      edx, edx
//    140d2e6d5  mov      r8, r12
//    140d2e6d8  call     0x142b1b410
//    140d2e6dd  test     rax, rax
//    140d2e6e0  je       0x140d2f085
//    140d2e6e6  mov      r15, rax
//    140d2e6e9  mov      rax, rdi
//    140d2e6ec  mov      rdx, qword ptr [rbp + 0x130]
//    140d2e6f3  jmp      0x140d2e6fd
//    140d2e6f5  mov      r15d, 8
//    140d2e6fb  xor      eax, eax
//    140d2e6fd  mov      qword ptr [rbp + 0xf0], rax
//    140d2e704  mov      qword ptr [rbp + 0xc8], rax
//    140d2e70b  mov      qword ptr [rbp + 0xd0], r15
//    140d2e712  test     rdi, rdi
//    140d2e715  mov      qword ptr [rbp + 0x130], r13
//    140d2e71c  je       0x140d2e959
//    140d2e722  xor      r13d, r13d
//    140d2e725  cmp      qword ptr [rbp + 0x128], 0
//    140d2e72d  setne    r13b
//    140d2e731  mov      rcx, r15
//    140d2e734  mov      r8, r12
//    140d2e737  call     0x1431a01a3
//    140d2e73c  mov      qword ptr [rbp + 0xd8], rdi
//    140d2e743  mov      byte ptr [rbp + 0x13e], r14b
//    140d2e74a  mov      qword ptr [rbp + 0xe8], r14
//    140d2e751  mov      qword ptr [rbp + 0xe0], r13
//    140d2e758  mov      qword ptr [rbp - 0x58], rbx
//    140d2e75c  lea      rax, [rbp + 0xe8]
//    140d2e763  mov      qword ptr [rbp - 0x50], rax
//    140d2e767  lea      rax, [rbp + 0xe0]
//    140d2e76e  mov      qword ptr [rbp - 0x48], rax
//    140d2e772  lea      rax, [rbp + 0x13e]
//    140d2e779  mov      qword ptr [rbp - 0x40], rax
//    140d2e77d  lea      rcx, [rip + 0x271c544]
//    140d2e784  lea      rdx, [rbp - 0x58]
//    140d2e788  call     0x140ffa3e0
//    140d2e78d  nop      
//    140d2e78e  mov      rcx, qword ptr [rbp + 0x130]
//    140d2e795  mov      rcx, qword ptr [rcx + 0x668]
//    140d2e79c  mov      r8, rsi
//    140d2e79f  sub      r8, rax
//    140d2e7a2  sub      rax, rsi
//    140d2e7a5  cmovb    rax, r8
//    140d2e7a9  mov      r8, rcx
//    140d2e7ac  sub      r8, rdx
//    140d2e7af  sub      rdx, rcx
//    140d2e7b2  cmovb    rdx, r8
//    140d2e7b6  imul     rax, rax
//    140d2e7ba  imul     rdx, rdx
//    140d2e7be  add      rdx, rax
//    140d2e7c1  movabs   rax, 0x35a4e9000
//    140d2e7cb  cmp      rdx, rax
//    140d2e7ce  ja       0x140d2e959
//    140d2e7d4  mov      rax, qword ptr [rbp + 0xf8]
//    140d2e7db  mov      rbx, qword ptr [rax + 0x1f0]
//    140d2e7e2  xor      r12d, r12d
//    140d2e7e5  xor      r14d, r14d
//    140d2e7e8  jmp      0x140d2e801
//    140d2e7ea  nop      word ptr [rax + rax]
//    140d2e7f0  xor      eax, eax
//    140d2e7f2  add      r12, rax
//    140d2e7f5  inc      r14
//    140d2e7f8  cmp      r14, rdi
//    140d2e7fb  je       0x140d2e8d0
//    140d2e801  mov      rdx, qword ptr [r15 + r14*8]
//    140d2e805  mov      rcx, qword ptr [rbp + 0x110]
//    140d2e80c  call     rbx
//    140d2e80e  nop      
//    140d2e80f  mov      rsi, rax
//    140d2e812  test     rax, rax
//    140d2e815  je       0x140d2e7f5
//    140d2e817  cmp      dword ptr [rsi + 0x4c0], -1
//    140d2e81e  je       0x140d2e7f0
//    140d2e820  mov      rcx, rsi
//    140d2e823  add      rcx, 0x490
//    140d2e82a  mov      rax, qword ptr [rbp + 0x130]
//    140d2e831  mov      qword ptr [rsp + 0x20], rax
//    140d2e836  mov      rdx, qword ptr [rbp + 0x120]
//    140d2e83d  mov      r8, rsi
//    140d2e840  lea      r9, [rip + 0x26a9f21]
//    140d2e847  call     0x1412857f0
//    140d2e84c  nop      
//    140d2e84d  mov      r13, rax
//    140d2e850  mov      rcx, qword ptr [rsi + 0x570]
//    140d2e857  mov      rax, qword ptr [rsi + 0x578]
//    140d2e85e  mov      rdx, rsi
//    140d2e861  call     qword ptr [rax + 0x90]
//    140d2e867  nop      
//    140d2e868  mov      ecx, dword ptr [rsi + 0x3fc]
//    140d2e86e  add      ecx, 0x64
//    140d2e871  cmp      ecx, 2
//    140d2e874  mov      edx, 1
//    140d2e879  cmovl    ecx, edx
//    140d2e87c  imul     rax, rax, 0x64
//    140d2e880  mov      rdx, rax
//    140d2e883  shr      rdx, 0x20
//    140d2e887  je       0x140d2e8b9
//    140d2e889  xor      edx, edx
//    140d2e88b  div      rcx
//    140d2e88e  mov      rcx, rax
//    140d2e891  cmp      rcx, 4
//    140d2e895  jae      0x140d2e89c
//    140d2e897  mov      ecx, 3
//    140d2e89c  imul     rax, r13, 0x3e8
//    140d2e8a3  mov      rdx, rax
//    140d2e8a6  or       rdx, rcx
//    140d2e8a9  shr      rdx, 0x20
//    140d2e8ad  je       0x140d2e8c7
//    140d2e8af  xor      edx, edx
//    140d2e8b1  div      rcx
//    140d2e8b4  jmp      0x140d2e7f2
//    140d2e8b9  xor      edx, edx
//    140d2e8bb  div      ecx
//    140d2e8bd  mov      ecx, eax
//    140d2e8bf  cmp      rcx, 4
//    140d2e8c3  jb       0x140d2e897
//    140d2e8c5  jmp      0x140d2e89c
//    140d2e8c7  xor      edx, edx
//    140d2e8c9  div      ecx
//    140d2e8cb  jmp      0x140d2e7f2
//    140d2e8d0  test     r12, r12
//    140d2e8d3  je       0x140d2e93e
//    140d2e8d5  mov      rax, qword ptr [rbp + 0x130]
//    140d2e8dc  imul     rax, qword ptr [rax + 0x670], 0x3e8
//    140d2e8e7  mov      rcx, rax
//    140d2e8ea  or       rcx, r12
//    140d2e8ed  shr      rcx, 0x20
//    140d2e8f1  mov      rcx, qword ptr [rbp + 0x120]
//    140d2e8f8  je       0x140d2ed9f
//    140d2e8fe  xor      edx, edx
//    140d2e900  div      r12
//    140d2e903  mov      rcx, qword ptr [rcx + 8]
//    140d2e907  cmp      rax, qword ptr [rcx + 0x12f8]
//    140d2e90e  jbe      0x140d2e959
//    140d2e910  call     qword ptr [rip + 0x25a7082]
//    140d2e916  mov      rcx, rax
//    140d2e919  xor      edx, edx
//    140d2e91b  mov      r8, r15
//    140d2e91e  call     qword ptr [rip + 0x25a70bc]
//    140d2e924  mov      r15, qword ptr [rbp + 0x100]
//    140d2e92b  mov      rsi, qword ptr [rbp + 0x128]
//    140d2e932  movzx    ebx, byte ptr [rbp + 0x13f]
//    140d2e939  jmp      0x140d2ed73
//    140d2e93e  mov      rax, 0xffffffffffffffff
//    140d2e945  mov      rcx, qword ptr [rbp + 0x120]
//    140d2e94c  mov      rcx, qword ptr [rcx + 8]
//    140d2e950  cmp      rax, qword ptr [rcx + 0x12f8]
//    140d2e957  ja       0x140d2e910
//    140d2e959  cmp      qword ptr [rbp + 0xf0], 0
//    140d2e961  je       0x140d2e977
//    140d2e963  call     qword ptr [rip + 0x25a702f]
//    140d2e969  mov      rcx, rax
//    140d2e96c  xor      edx, edx
//    140d2e96e  mov      r8, r15
//    140d2e971  call     qword ptr [rip + 0x25a7069]
//    140d2e977  mov      r13b, 1
//    140d2e97a  mov      r15, qword ptr [rbp + 0x130]
//    140d2e981  cmp      dword ptr [r15 + 0x3f0], 0
//    140d2e989  jle      0x140d2e9d7
//    140d2e98b  mov      r14, qword ptr [rbp + 0x120]
//    140d2e992  mov      rcx, qword ptr [rbp + 0x110]
//    140d2e999  call     qword ptr [rbp + 0x118]
//    140d2e99f  test     rax, rax
//    140d2e9a2  jne      0x140d2f039
//    140d2e9a8  lea      rax, [rip + 0x26ab745]
//    140d2e9af  mov      rcx, qword ptr [rbp + 0x108]
//    140d2e9b6  movsxd   rcx, dword ptr [rax + rcx*4]
//    140d2e9ba  add      rcx, rax
//    140d2e9bd  jmp      rcx
//    140d2e9bf  mov      rcx, qword ptr [rbp + 0x128]
//    140d2e9c6  test     rcx, rcx
//    140d2e9c9  mov      eax, 0xc0
//    140d2e9ce  cmove    rax, rcx
//    140d2e9d2  jmp      0x140d2eb9f
//    140d2e9d7  cmp      dword ptr [r15 + 0x4f8], -1
//    140d2e9df  mov      r14, qword ptr [rbp + 0x120]
//    140d2e9e6  je       0x140d2ea8b
//    140d2e9ec  mov      rax, qword ptr [r15 + 0x628]
//    140d2e9f3  xor      edi, edi
//    140d2e9f5  sub      rax, qword ptr [r15 + 0x670]
//    140d2e9fc  cmovae   rdi, rax
//    140d2ea00  mov      rax, qword ptr [r15 + 0x4c8]
//    140d2ea07  mov      r10, qword ptr [r15 + 0x4d0]
//    140d2ea0e  mov      rcx, qword ptr [r10 + 0x10]
//    140d2ea12  dec      rcx
//    140d2ea15  and      rcx, 0xfffffffffffffff0
//    140d2ea19  add      rcx, rax
//    140d2ea1c  add      rcx, 0x10
//    140d2ea20  lea      rsi, [rip + 0x26a9d41]
//    140d2ea27  mov      rdx, r14
//    140d2ea2a  mov      r8, r15
//    140d2ea2d  mov      r9, rsi
//    140d2ea30  call     qword ptr [r10 + 0x40]
//    140d2ea34  cmp      rdi, rax
//    140d2ea37  cmovae   rdi, rax
//    140d2ea3b  test     rdi, rdi
//    140d2ea3e  jne      0x140d2e992
//    140d2ea44  mov      rax, qword ptr [r15 + 0x4c8]
//    140d2ea4b  mov      r10, qword ptr [r15 + 0x4d0]
//    140d2ea52  mov      rcx, qword ptr [r10 + 0x10]
//    140d2ea56  dec      rcx
//    140d2ea59  and      rcx, 0xfffffffffffffff0
//    140d2ea5d  lea      rdx, [rax + rcx]
//    140d2ea61  add      rdx, 0x10
//    140d2ea65  mov      qword ptr [rsp + 0x20], rsi
//    140d2ea6a  lea      rcx, [rbp - 0x58]
//    140d2ea6e  mov      r8, r14
//    140d2ea71  mov      r9, r15
//    140d2ea74  call     qword ptr [r10 + 0xa0]
//    140d2ea7b  cmp      dword ptr [rbp - 0x10], -1
//    140d2ea7f  je       0x140d2ea8b
//    140d2ea81  cmp      dword ptr [rbp + 0x28], 0
//    140d2ea85  jg       0x140d2e992
//    140d2ea8b  lea      rax, [r15 + 0x500]
//    140d2ea92  cmp      qword ptr [r15 + 0x5c8], 3
//    140d2ea9a  lea      rdi, [rip + 0x26a9df7]
//    140d2eaa1  cmovae   rdi, rax
//    140d2eaa5  cmp      dword ptr [rdi + 0x30], -1
//    140d2eaa9  je       0x140d2edba
//    140d2eaaf  mov      rax, qword ptr [r15 + 0x628]
//    140d2eab6  xor      ebx, ebx
//    140d2eab8  sub      rax, qword ptr [r15 + 0x670]
//    140d2eabf  cmovae   rbx, rax
//    140d2eac3  mov      rax, qword ptr [rdi]
//    140d2eac6  mov      r10, qword ptr [rdi + 8]
//    140d2eaca  mov      rcx, qword ptr [r10 + 0x10]
//    140d2eace  dec      rcx
//    140d2ead1  and      rcx, 0xfffffffffffffff0
//    140d2ead5  add      rcx, rax
//    140d2ead8  add      rcx, 0x10
//    140d2eadc  lea      rsi, [rip + 0x26a9c85]
//    140d2eae3  mov      rdx, r14
//    140d2eae6  mov      r8, r15
//    140d2eae9  mov      r9, rsi
//    140d2eaec  call     qword ptr [r10 + 0x40]
//    140d2eaf0  cmp      rbx, rax
//    140d2eaf3  cmovae   rbx, rax
//    140d2eaf7  test     rbx, rbx
//    140d2eafa  jne      0x140d2e992
//    140d2eb00  mov      rax, qword ptr [rdi]
//    140d2eb03  mov      r10, qword ptr [rdi + 8]
//    140d2eb07  mov      rcx, qword ptr [r10 + 0x10]
//    140d2eb0b  dec      rcx
//    140d2eb0e  and      rcx, 0xfffffffffffffff0
//    140d2eb12  lea      rdx, [rax + rcx]
//    140d2eb16  add      rdx, 0x10
//    140d2eb1a  mov      qword ptr [rsp + 0x20], rsi
//    140d2eb1f  lea      rcx, [rbp - 0x58]
//    140d2eb23  mov      r8, r14
//    140d2eb26  mov      r9, qword ptr [rbp + 0x130]
//    140d2eb2d  call     qword ptr [r10 + 0xa0]
//    140d2eb34  cmp      dword ptr [rbp - 0x10], -1
//    140d2eb38  setne    al
//    140d2eb3b  cmp      dword ptr [rbp + 0x28], 0
//    140d2eb3f  setg     r13b
//    140d2eb43  and      r13b, al
//    140d2eb46  jmp      0x140d2e992
//    140d2eb4b  mov      eax, 0x180
//    140d2eb50  jmp      0x140d2eb9f
//    140d2eb52  cmp      qword ptr [rbp + 0x128], 0
//    140d2eb5a  mov      ecx, 0x120
//    140d2eb5f  mov      eax, 0x60
//    140d2eb64  cmovne   rax, rcx
//    140d2eb68  jmp      0x140d2eb9f
//    140d2eb6a  cmp      qword ptr [rbp + 0x128], 0
//    140d2eb72  mov      ecx, 0x150
//    140d2eb77  mov      eax, 0x90
//    140d2eb7c  cmovne   rax, rcx
//    140d2eb80  jmp      0x140d2eb9f
//    140d2eb82  cmp      qword ptr [rbp + 0x128], 0
//    140d2eb8a  mov      ecx, 0xf0
//    140d2eb8f  mov      eax, 0x30
//    140d2eb94  cmovne   rax, rcx
//    140d2eb98  jmp      0x140d2eb9f
//    140d2eb9a  mov      eax, 0x1b0
//    140d2eb9f  movabs   r8, 0x7ffffffffffffff8
//    140d2eba9  mov      rbx, qword ptr [rdx + rax + 0x28]
//    140d2ebae  lea      r15, [rbx*8]
//    140d2ebb6  mov      rcx, rbx
//    140d2ebb9  shr      rcx, 0x3d
//    140d2ebbd  setne    cl
//    140d2ebc0  cmp      r15, r8
//    140d2ebc3  seta     r8b
//    140d2ebc7  or       r8b, cl
//    140d2ebca  je       0x140d2ebd8
//    140d2ebcc  xor      ecx, ecx
//    140d2ebce  mov      rdx, r15
//    140d2ebd1  call     0x1431a0cbf
//    140d2ebd6  ud2      
//    140d2ebd8  mov      rsi, qword ptr [rdx + rax + 0x20]
//    140d2ebdd  test     r15, r15
//    140d2ebe0  je       0x140d2ec0d
//    140d2ebe2  xor      edx, edx
//    140d2ebe4  mov      r8, r15
//    140d2ebe7  call     0x142b1b410
//    140d2ebec  test     rax, rax
//    140d2ebef  je       0x140d2f0a2
//    140d2ebf5  mov      r12, rax
//    140d2ebf8  mov      rdi, rbx
//    140d2ebfb  mov      qword ptr [rbp - 0x58], rdi
//    140d2ebff  mov      qword ptr [rbp - 0x50], r12
//    140d2ec03  test     rbx, rbx
//    140d2ec06  jne      0x140d2ec26
//    140d2ec08  jmp      0x140d2ecef
//    140d2ec0d  mov      r12d, 8
//    140d2ec13  xor      edi, edi
//    140d2ec15  mov      qword ptr [rbp - 0x58], rdi
//    140d2ec19  mov      qword ptr [rbp - 0x50], r12
//    140d2ec1d  test     rbx, rbx
//    140d2ec20  je       0x140d2ecef
//    140d2ec26  mov      rcx, r12
//    140d2ec29  mov      rdx, rsi
//    140d2ec2c  mov      r8, r15
//    140d2ec2f  call     0x1431a01a3
//    140d2ec34  mov      qword ptr [rbp - 0x48], rbx
//    140d2ec38  mov      rax, qword ptr [rbp + 0xf8]
//    140d2ec3f  mov      rsi, qword ptr [rax + 0x1f0]
//    140d2ec46  xor      ebx, ebx
//    140d2ec48  jmp      0x140d2ec5d
//    140d2ec4a  nop      word ptr [rax + rax]
//    140d2ec50  add      rbx, 8
//    140d2ec54  cmp      r15, rbx
//    140d2ec57  je       0x140d2ecef
//    140d2ec5d  mov      rdx, qword ptr [r12 + rbx]
//    140d2ec61  mov      rcx, qword ptr [rbp + 0x110]
//    140d2ec68  call     rsi
//    140d2ec6a  nop      
//    140d2ec6b  test     rax, rax
//    140d2ec6e  je       0x140d2ec50
//    140d2ec70  cmp      dword ptr [rax + 0x68], 4
//    140d2ec74  jne      0x140d2ec50
//    140d2ec76  cmp      dword ptr [rax + 0x88], 1
//    140d2ec7d  jne      0x140d2ec50
//    140d2ec7f  mov      rax, qword ptr [rax + 0x90]
//    140d2ec86  mov      rcx, qword ptr [rbp + 0x130]
//    140d2ec8d  cmp      rax, qword ptr [rcx + 0x5c0]
//    140d2ec94  jne      0x140d2ec50
//    140d2ec96  call     qword ptr [rip + 0x25a6cfc]
//    140d2ec9c  mov      rcx, rax
//    140d2ec9f  xor      edx, edx
//    140d2eca1  mov      r8, r12
//    140d2eca4  call     qword ptr [rip + 0x25a6d36]
//    140d2ecaa  mov      rcx, qword ptr [rbp + 0x110]
//    140d2ecb1  call     qword ptr [rbp + 0x118]
//    140d2ecb7  test     rax, rax
//    140d2ecba  jne      0x140d2f064
//    140d2ecc0  lea      rax, [rip + 0x26ab445]
//    140d2ecc7  mov      rcx, qword ptr [rbp + 0x108]
//    140d2ecce  movsxd   rcx, dword ptr [rax + rcx*4]
//    140d2ecd2  add      rcx, rax
//    140d2ecd5  mov      r8, qword ptr [rbp + 0x128]
//    140d2ecdc  jmp      rcx
//    140d2ecde  test     r8, r8
//    140d2ece1  mov      eax, 0xc0
//    140d2ece6  cmove    rax, r8
//    140d2ecea  jmp      0x140d2ee07
//    140d2ecef  test     rdi, rdi
//    140d2ecf2  je       0x140d2ed08
//    140d2ecf4  call     qword ptr [rip + 0x25a6c9e]
//    140d2ecfa  mov      rcx, rax
//    140d2ecfd  xor      edx, edx
//    140d2ecff  mov      r8, r12
//    140d2ed02  call     qword ptr [rip + 0x25a6cd8]
//    140d2ed08  mov      rax, qword ptr [rbp + 0x130]
//    140d2ed0f  mov      rcx, qword ptr [rax + 0x628]
//    140d2ed16  test     rcx, rcx
//    140d2ed19  mov      r15, qword ptr [rbp + 0x100]
//    140d2ed20  mov      rsi, qword ptr [rbp + 0x128]
//    140d2ed27  je       0x140d2f094
//    140d2ed2d  imul     rax, qword ptr [rax + 0x670], 0x64
//    140d2ed35  mov      rdx, rax
//    140d2ed38  or       rdx, rcx
//    140d2ed3b  shr      rdx, 0x20
//    140d2ed3f  movzx    ebx, byte ptr [rbp + 0x13f]
//    140d2ed46  je       0x140d2ed5f
//    140d2ed48  xor      edx, edx
//    140d2ed4a  div      rcx
//    140d2ed4d  mov      ecx, 5
//    140d2ed52  test     r13b, r13b
//    140d2ed55  je       0x140d2ed6d
//    140d2ed57  cmp      rax, 0x15
//    140d2ed5b  jae      0x140d2ed73
//    140d2ed5d  jmp      0x140d2ed85
//    140d2ed5f  xor      edx, edx
//    140d2ed61  div      ecx
//    140d2ed63  mov      ecx, 5
//    140d2ed68  test     r13b, r13b
//    140d2ed6b  jne      0x140d2ed57
//    140d2ed6d  cmp      rax, 0x29
//    140d2ed71  jb       0x140d2ed85
//    140d2ed73  mov      qword ptr [r15 + 8], rsi
//    140d2ed77  mov      byte ptr [r15 + 0x10], bl
//    140d2ed7b  mov      byte ptr [r15 + 0x11], 0
//    140d2ed80  mov      ecx, 6
//    140d2ed85  mov      qword ptr [r15], rcx
//    140d2ed88  mov      rax, r15
//    140d2ed8b  add      rsp, 0x1c8
//    140d2ed92  pop      rbx
//    140d2ed93  pop      rdi
//    140d2ed94  pop      rsi
//    140d2ed95  pop      r12
//    140d2ed97  pop      r13
//    140d2ed99  pop      r14
//    140d2ed9b  pop      r15
//    140d2ed9d  pop      rbp
//    140d2ed9e  ret      
//    140d2ed9f  xor      edx, edx
//    140d2eda1  div      r12d
//    140d2eda4  mov      rcx, qword ptr [rcx + 8]
//    140d2eda8  cmp      rax, qword ptr [rcx + 0x12f8]
//    140d2edaf  ja       0x140d2e910
//    140d2edb5  jmp      0x140d2e959
//    140d2edba  xor      r13d, r13d
//    140d2edbd  jmp      0x140d2e992
//    140d2edc2  mov      eax, 0x180
//    140d2edc7  jmp      0x140d2ee07
//    140d2edc9  test     r8, r8
//    140d2edcc  mov      ecx, 0x120
//    140d2edd1  mov      eax, 0x60
//    140d2edd6  cmovne   rax, rcx
//    140d2edda  jmp      0x140d2ee07
//    140d2eddc  test     r8, r8
//    140d2eddf  mov      ecx, 0x150
//    140d2ede4  mov      eax, 0x90
//    140d2ede9  cmovne   rax, rcx
//    140d2eded  jmp      0x140d2ee07
//    140d2edef  test     r8, r8
//    140d2edf2  mov      ecx, 0xf0
//    140d2edf7  mov      eax, 0x30
//    140d2edfc  cmovne   rax, rcx
//    140d2ee00  jmp      0x140d2ee07
//    140d2ee02  mov      eax, 0x1b0
//    140d2ee07  mov      rdi, qword ptr [rdx + rax + 0x28]
//    140d2ee0c  lea      r12, [rdi*8]
//    140d2ee14  mov      rcx, rdi
//    140d2ee17  shr      rcx, 0x3d
//    140d2ee1b  setne    cl
//    140d2ee1e  movabs   r8, 0x7ffffffffffffff8
//    140d2ee28  cmp      r12, r8
//    140d2ee2b  seta     r8b
//    140d2ee2f  or       r8b, cl
//    140d2ee32  jne      0x140d2e6af
//    140d2ee38  mov      r13, qword ptr [rdx + rax + 0x20]
//    140d2ee3d  test     r12, r12
//    140d2ee40  je       0x140d2ee5a
//    140d2ee42  xor      edx, edx
//    140d2ee44  mov      r8, r12
//    140d2ee47  call     0x142b1b410
//    140d2ee4c  test     rax, rax
//    140d2ee4f  je       0x140d2f085
//    140d2ee55  mov      rcx, rdi
//    140d2ee58  jmp      0x140d2ee61
//    140d2ee5a  mov      eax, 8
//    140d2ee5f  xor      ecx, ecx
//    140d2ee61  mov      qword ptr [rbp - 0x58], rcx
//    140d2ee65  mov      qword ptr [rbp - 0x50], rax
//    140d2ee69  test     rdi, rdi
//    140d2ee6c  mov      qword ptr [rbp + 0x118], rax
//    140d2ee73  je       0x140d2efae
//    140d2ee79  mov      qword ptr [rbp + 0x108], rcx
//    140d2ee80  mov      rcx, rax
//    140d2ee83  mov      rdx, r13
//    140d2ee86  mov      r8, r12
//    140d2ee89  call     0x1431a01a3
//    140d2ee8e  mov      qword ptr [rbp - 0x48], rdi
//    140d2ee92  xor      r15d, r15d
//    140d2ee95  mov      ebx, 1
//    140d2ee9a  xor      r14d, r14d
//    140d2ee9d  jmp      0x140d2eeb3
//    140d2ee9f  xor      edx, edx
//    140d2eea1  div      rcx
//    140d2eea4  add      r15, rax
//    140d2eea7  inc      r14
//    140d2eeaa  cmp      r14, rdi
//    140d2eead  je       0x140d2ef88
//    140d2eeb3  mov      rax, qword ptr [rbp + 0x118]
//    140d2eeba  mov      rdx, qword ptr [rax + r14*8]
//    140d2eebe  mov      rcx, qword ptr [rbp + 0x110]
//    140d2eec5  call     rsi
//    140d2eec7  nop      
//    140d2eec8  mov      r12, rax
//    140d2eecb  test     rax, rax
//    140d2eece  je       0x140d2eea7
//    140d2eed0  cmp      dword ptr [r12 + 0x4c0], -1
//    140d2eed9  mov      rax, qword ptr [rbp + 0x130]
//    140d2eee0  je       0x140d2f055
//    140d2eee6  mov      rcx, r12
//    140d2eee9  add      rcx, 0x490
//    140d2eef0  mov      qword ptr [rsp + 0x20], rax
//    140d2eef5  mov      rdx, qword ptr [rbp + 0x120]
//    140d2eefc  mov      r8, r12
//    140d2eeff  lea      r9, [rip + 0x26a9862]
//    140d2ef06  call     0x1412857f0
//    140d2ef0b  nop      
//    140d2ef0c  mov      r13, rax
//    140d2ef0f  mov      rcx, qword ptr [r12 + 0x570]
//    140d2ef17  mov      rax, qword ptr [r12 + 0x578]
//    140d2ef1f  mov      rdx, r12
//    140d2ef22  call     qword ptr [rax + 0x90]
//    140d2ef28  nop      
//    140d2ef29  mov      ecx, dword ptr [r12 + 0x3fc]
//    140d2ef31  add      ecx, 0x64
//    140d2ef34  cmp      ecx, 2
//    140d2ef37  cmovl    ecx, ebx
//    140d2ef3a  imul     rax, rax, 0x64
//    140d2ef3e  mov      rdx, rax
//    140d2ef41  shr      rdx, 0x20
//    140d2ef45  je       0x140d2ef7a
//    140d2ef47  xor      edx, edx
//    140d2ef49  div      rcx
//    140d2ef4c  mov      rcx, rax
//    140d2ef4f  cmp      rcx, 4
//    140d2ef53  jae      0x140d2ef5a
//    140d2ef55  mov      ecx, 3
//    140d2ef5a  imul     rax, r13, 0x3e8
//    140d2ef61  mov      rdx, rax
//    140d2ef64  or       rdx, rcx
//    140d2ef67  shr      rdx, 0x20
//    140d2ef6b  jne      0x140d2ee9f
//    140d2ef71  xor      edx, edx
//    140d2ef73  div      ecx
//    140d2ef75  jmp      0x140d2eea4
//    140d2ef7a  xor      edx, edx
//    140d2ef7c  div      ecx
//    140d2ef7e  mov      ecx, eax
//    140d2ef80  cmp      rcx, 4
//    140d2ef84  jae      0x140d2ef5a
//    140d2ef86  jmp      0x140d2ef55
//    140d2ef88  cmp      r15, 1
//    140d2ef8c  adc      r15, 0
//    140d2ef90  movzx    ebx, byte ptr [rbp + 0x13f]
//    140d2ef97  mov      r14, qword ptr [rbp + 0x120]
//    140d2ef9e  mov      rdi, qword ptr [rbp + 0x130]
//    140d2efa5  mov      rcx, qword ptr [rbp + 0x108]
//    140d2efac  jmp      0x140d2efc2
//    140d2efae  mov      r15d, 1
//    140d2efb4  movzx    ebx, byte ptr [rbp + 0x13f]
//    140d2efbb  mov      rdi, qword ptr [rbp + 0x130]
//    140d2efc2  test     rcx, rcx
//    140d2efc5  mov      rsi, qword ptr [rbp + 0x128]
//    140d2efcc  je       0x140d2efe6
//    140d2efce  call     qword ptr [rip + 0x25a69c4]
//    140d2efd4  mov      rcx, rax
//    140d2efd7  xor      edx, edx
//    140d2efd9  mov      r8, qword ptr [rbp + 0x118]
//    140d2efe0  call     qword ptr [rip + 0x25a69fa]
//    140d2efe6  imul     rax, qword ptr [rdi + 0x670], 0x3e8
//    140d2eff1  mov      rcx, rax
//    140d2eff4  or       rcx, r15
//    140d2eff7  shr      rcx, 0x20
//    140d2effb  je       0x140d2f004
//    140d2effd  xor      edx, edx
//    140d2efff  div      r15
//    140d2f002  jmp      0x140d2f009
//    140d2f004  xor      edx, edx
//    140d2f006  div      r15d
//    140d2f009  mov      rdx, qword ptr [r14 + 8]
//    140d2f00d  mov      ecx, 5
//    140d2f012  cmp      rax, qword ptr [rdx + 0x12f8]
//    140d2f019  mov      r15, qword ptr [rbp + 0x100]
//    140d2f020  ja       0x140d2ed73
//    140d2f026  jmp      0x140d2ed85
//    140d2f02b  lea      rcx, [rip + 0x26aa086]
//    140d2f032  call     0x1431a37c0
//    140d2f037  ud2      
//    140d2f039  lea      rcx, [rip + 0x26aa060]
//    140d2f040  call     0x1431a37c0
//    140d2f045  ud2      
//    140d2f047  lea      rcx, [rip + 0x26aa00a]
//    140d2f04e  call     0x1431a37c0
//    140d2f053  ud2      
//    140d2f055  lea      rcx, [rip + 0x26a98e4]
//    140d2f05c  call     0x1431a37c0
//    140d2f061  nop      
//    140d2f062  ud2      
//    140d2f064  lea      rcx, [rip + 0x26aa01d]
//    140d2f06b  call     0x1431a37c0
//    140d2f070  ud2      
//    140d2f072  lea      r8, [rip + 0x26a9fc7]
//    140d2f079  mov      edx, 2
//    140d2f07e  call     0x1431a3863
//    140d2f083  ud2      
//    140d2f085  mov      ecx, 8
//    140d2f08a  mov      rdx, r12
//    140d2f08d  call     0x1431a0cbf
//    140d2f092  ud2      
//    140d2f094  lea      rcx, [rip + 0x26a9fd5]
//    140d2f09b  call     0x1431a3b40
//    140d2f0a0  ud2      
//    140d2f0a2  mov      ecx, 8
//    140d2f0a7  mov      rdx, r15
//    140d2f0aa  call     0x1431a0cbf
//    140d2f0af  ud2      
//
//    --- 추가 청크(Ghidra 비연속 함수바디) 0xd321e2~0xd32608 ---
//    140d321e2  pop      rdi
//    140d321e3  pop      rsi
//    140d321e4  pop      r12
//    140d321e6  pop      r13
//    140d321e8  pop      r14
//    140d321ea  pop      r15
//    140d321ec  ret      
//    140d321ed  mov      r11, rsi
//    140d321f0  mov      r8, qword ptr [rsi + 0x680]
//    140d321f7  mov      rsi, qword ptr [rsp + 0x1d8]
//    140d321ff  movsxd   rax, dword ptr [rsi + 0x470]
//    140d32206  test     rax, rax
//    140d32209  jne      0x140d32039
//    140d3220f  jmp      0x140d320a7
//    140d32214  xor      edx, edx
//    140d32216  div      ecx
//    140d32218  mov      edi, eax
//    140d3221a  mov      qword ptr [rsp + 0xb0], r8
//    140d32222  mov      qword ptr [rsp + 0xb8], r9
//    140d3222a  mov      byte ptr [rsp + 0xc0], 0
//    140d32232  lea      rdx, [rsp + 0xb0]
//    140d3223a  mov      rcx, r15
//    140d3223d  call     0x140d25d30
//    140d32242  imul     rax, rdi
//    140d32246  shr      rax, 3
//    140d3224a  movabs   rcx, 0x20c49ba5e353f7cf
//    140d32254  mul      rcx
//    140d32257  mov      rbx, rdx
//    140d3225a  shr      rbx, 4
//    140d3225e  mov      rax, qword ptr [rsp + 0x78]
//    140d32263  mov      rcx, qword ptr [rax]
//    140d32266  mov      rax, qword ptr [rax + 8]
//    140d3226a  mov      qword ptr [rsp + 0x108], rax
//    140d32272  mov      rax, qword ptr [rax + 0x40]
//    140d32276  mov      qword ptr [rsp + 0x78], rcx
//    140d3227b  mov      r12, rax
//    140d3227e  call     rax
//    140d32280  cmp      rax, 2
//    140d32284  jne      0x140d322e8
//    140d32286  mov      rcx, qword ptr [rsp + 0x98]
//    140d3228e  mov      rax, qword ptr [rsp + 0x90]
//    140d32296  call     qword ptr [rax + 0xf8]
//    140d3229c  cmp      rax, 1
//    140d322a0  jne      0x140d322dd
//    140d322a2  mov      rdi, rdx
//    140d322a5  mov      rcx, qword ptr [r13 + 0x660]
//    140d322ac  mov      rdx, qword ptr [r13 + 0x668]
//    140d322b3  mov      r8, qword ptr [rsp + 0x60]
//    140d322b8  mov      r9, qword ptr [rsp + 0x58]
//    140d322bd  call     0x1412a07d0
//    140d322c2  cmp      rdi, 1
//    140d322c6  adc      rdi, 0
//    140d322ca  mov      rcx, rax
//    140d322cd  or       rcx, rdi
//    140d322d0  shr      rcx, 0x20
//    140d322d4  je       0x140d322e1
//    140d322d6  xor      edx, edx
//    140d322d8  div      rdi
//    140d322db  jmp      0x140d322e5
//    140d322dd  xor      eax, eax
//    140d322df  jmp      0x140d322e5
//    140d322e1  xor      edx, edx
//    140d322e3  div      edi
//    140d322e5  add      rbx, rax
//    140d322e8  mov      rdi, qword ptr [rsp + 0x1d0]
//    140d322f0  mov      eax, dword ptr [rsp + 0x68]
//    140d322f4  lea      rcx, [rip + 0x26a7e29]
//    140d322fb  movsxd   rax, dword ptr [rcx + rax*4]
//    140d322ff  add      rax, rcx
//    140d32302  jmp      rax
//    140d32304  mov      rax, qword ptr [rsi + 0x5c0]
//    140d3230b  mov      rcx, qword ptr [rsp + 0xa0]
//    140d32313  mov      dword ptr [rcx], 0
//    140d32319  mov      qword ptr [rcx + 8], rax
//    140d3231d  jmp      0x140d321d9
//    140d32322  mov      rcx, qword ptr [rsp + 0x98]
//    140d3232a  mov      rax, qword ptr [rsp + 0x90]
//    140d32332  call     qword ptr [rax + 0xd8]
//    140d32338  test     al, al
//    140d3233a  je       0x140d324c8
//    140d32340  mov      rax, qword ptr [rsp + 0x1c8]
//    140d32348  lea      rcx, [rax - 3]
//    140d3234c  lea      rax, [rdi - 3]
//    140d32350  xor      esi, esi
//    140d32352  mov      ebx, 0x270f
//    140d32357  mov      qword ptr [rsp + 0x98], rax
//    140d3235f  cmp      rax, 0x1d
//    140d32363  mov      qword ptr [rsp + 0x110], rcx
//    140d3236b  jg       0x140d328d7
//    140d32371  cmp      rcx, 0x1d
//    140d32375  jg       0x140d328d7
//    140d3237b  lea      rax, [rdi - 3]
//    140d3237f  or       rax, rcx
//    140d32382  mov      edx, 0x270f
//    140d32387  js       0x140d328dc
//    140d3238d  imul     rax, rcx, 0x7d00
//    140d32394  add      rax, 0x3e80
//    140d3239a  lea      rcx, [rdi - 3]
//    140d3239e  imul     rcx, rcx, 0x7d00
//    140d323a5  add      rcx, 0x3e80
//    140d323ac  mov      qword ptr [rsp + 0x28], rcx
//    140d323b1  mov      qword ptr [rsp + 0x20], rax
//    140d323b6  mov      byte ptr [rsp + 0x30], 0xc
//    140d323bb  lea      rcx, [rsp + 0xb0]
//    140d323c3  mov      rdx, qword ptr [rsp + 0x50]
//    140d323c8  mov      r8, r14
//    140d323cb  mov      r9, qword ptr [rsp + 0x1c0]
//    140d323d3  call     0x140d84db0
//    140d323d8  mov      rcx, qword ptr [rsp + 0x110]
//    140d323e0  mov      rdx, qword ptr [rsp + 0xb0]
//    140d323e8  mov      rsi, qword ptr [rsp + 0xc0]
//    140d323f0  jmp      0x140d328dc
//    140d323f5  mov      rax, qword ptr [rsp + 0xa0]
//    140d323fd  mov      dword ptr [rax], 3
//    140d32403  jmp      0x140d321d9
//    140d32408  mov      rcx, qword ptr [rsp + 0x98]
//    140d32410  mov      rax, qword ptr [rsp + 0x90]
//    140d32418  call     qword ptr [rax + 0xd8]
//    140d3241e  test     al, al
//    140d32420  je       0x140d32743
//    140d32426  mov      rax, qword ptr [rsp + 0x1c8]
//    140d3242e  lea      r12, [rax - 3]
//    140d32432  lea      rax, [rdi - 3]
//    140d32436  xor      esi, esi
//    140d32438  mov      ebx, 0x270f
//    140d3243d  mov      qword ptr [rsp + 0x78], rax
//    140d32442  cmp      rax, 0x1d
//    140d32446  jg       0x140d330d1
//    140d3244c  cmp      r12, 0x1d
//    140d32450  jg       0x140d330d1
//    140d32456  lea      rax, [rdi - 3]
//    140d3245a  or       rax, r12
//    140d3245d  mov      eax, 0x270f
//    140d32462  js       0x140d330d6
//    140d32468  imul     rax, r12, 0x7d00
//    140d3246f  add      rax, 0x3e80
//    140d32475  lea      rcx, [rdi - 3]
//    140d32479  imul     rcx, rcx, 0x7d00
//    140d32480  add      rcx, 0x3e80
//    140d32487  mov      qword ptr [rsp + 0x28], rcx
//    140d3248c  mov      qword ptr [rsp + 0x20], rax
//    140d32491  mov      byte ptr [rsp + 0x30], 0xc
//    140d32496  lea      rcx, [rsp + 0xb0]
//    140d3249e  mov      rdx, qword ptr [rsp + 0x50]
//    140d324a3  mov      r8, r14
//    140d324a6  mov      r9, qword ptr [rsp + 0x1c0]
//    140d324ae  call     0x140d84db0
//    140d324b3  mov      rax, qword ptr [rsp + 0xb0]
//    140d324bb  mov      rsi, qword ptr [rsp + 0xc0]
//    140d324c3  jmp      0x140d330d6
//    140d324c8  mov      rcx, rsi
//    140d324cb  mov      rsi, qword ptr [rsi + 0x68]
//    140d324cf  cmp      rsi, 0xd
//    140d324d3  jne      0x140d3280f
//    140d324d9  mov      rax, qword ptr [rcx + 0x70]
//    140d324dd  cmp      eax, 2
//    140d324e0  jne      0x140d3280f
//    140d324e6  mov      qword ptr [rsp + 0x68], rbx
//    140d324eb  mov      qword ptr [rsp + 0x50], rbp
//    140d324f0  mov      rbp, r13
//    140d324f3  mov      rsi, qword ptr [rcx + 0x78]
//    140d324f7  mov      rbx, qword ptr [rcx + 0x80]
//    140d324fe  mov      rax, qword ptr [rsp + 0x60]
//    140d32503  mov      qword ptr [rsp + 0x120], rax
//    140d3250b  mov      rax, qword ptr [rsp + 0x58]
//    140d32510  mov      qword ptr [rsp + 0x128], rax
//    140d32518  mov      qword ptr [rsp + 0xb0], 0
//    140d32524  mov      qword ptr [rsp + 0xb8], 0x3e8
//    140d32530  mov      byte ptr [rsp + 0xc0], 0
//    140d32538  lea      r13, [rsp + 0xb0]
//    140d32540  mov      rcx, r15
//    140d32543  mov      rdx, r13
//    140d32546  call     0x140d25d30
//    140d3254b  mov      r15, rax
//    140d3254e  mov      rdx, rbx
//    140d32551  mov      rax, rsi
//    140d32554  cmp      r15, qword ptr [rsp + 0x88]
//    140d3255c  jae      0x140d325b7
//    140d3255e  mov      rcx, qword ptr [rsp + 0x78]
//    140d32563  call     r12
//    140d32566  cmp      qword ptr [rsp + 0xa8], 0x28
//    140d3256f  jb       0x140d32584
//    140d32571  mov      rcx, rax
//    140d32574  mov      rdx, qword ptr [rsp + 0x58]
//    140d32579  mov      rax, qword ptr [rsp + 0x60]
//    140d3257e  cmp      rcx, 2
//    140d32582  je       0x140d325b7
//    140d32584  mov      r8, qword ptr [rbp + 0x660]
//    140d3258b  mov      r9, qword ptr [rbp + 0x668]
//    140d32592  mov      rax, qword ptr [rsp + 0x1c0]
//    140d3259a  mov      rax, qword ptr [rax + 8]
//    140d3259e  mov      rdx, qword ptr [rax + 8]
//    140d325a2  mov      rcx, qword ptr [rax + 0x20]
//    140d325a6  add      r8, r8
//    140d325a9  sub      r8, rsi
//    140d325ac  add      r9, r9
//    140d325af  sub      r9, rbx
//    140d325b2  call     0x1418096a0
//    140d325b7  mov      rcx, qword ptr [rsp + 0x1c0]
//    140d325bf  mov      r8, qword ptr [rcx + 8]
//    140d325c3  mov      rcx, qword ptr [r8 + 8]
//    140d325c7  mov      r9, qword ptr [r8 + 0x18]
//    140d325cb  mov      r8, qword ptr [r8 + 0x20]
//    140d325cf  mov      r10, qword ptr [rsp + 0x1d8]
//    140d325d7  mov      rdi, qword ptr [r10 + 0x5c0]
//    140d325de  mov      r10, qword ptr [r10 + 0x640]
//    140d325e5  imul     r10, qword ptr [rsp + 0x68]
//    140d325eb  mov      qword ptr [rsp + 0xb0], 0
//    140d325f7  mov      qword ptr [rsp + 0x48], r13
//    140d325fc  mov      qword ptr [rsp + 0x40], rdx
//    140d32601  mov      qword ptr [rsp + 0x38], rax
//
//    --- 추가 청크(Ghidra 비연속 함수바디) 0xd37f59~0xd38325 ---
//    140d37f59  mov      dword ptr [rbp + 0x15e0], edi
//    140d37f5f  xorps    xmm6, xmm6
//    140d37f62  movaps   xmmword ptr [rbp + 0x1730], xmm6
//    140d37f69  movaps   xmmword ptr [rbp + 0x1720], xmm6
//    140d37f70  movaps   xmmword ptr [rbp + 0x1710], xmm6
//    140d37f77  movaps   xmmword ptr [rbp + 0x1700], xmm6
//    140d37f7e  movaps   xmmword ptr [rbp + 0x16f0], xmm6
//    140d37f85  movaps   xmmword ptr [rbp + 0x16e0], xmm6
//    140d37f8c  lea      rcx, [rbp - 0x50]
//    140d37f90  mov      r8d, 0xc00
//    140d37f96  xor      edx, edx
//    140d37f98  call     0x1431a01af
//    140d37f9d  movaps   xmmword ptr [rbp + 0x1640], xmm6
//    140d37fa4  movaps   xmmword ptr [rbp + 0x1630], xmm6
//    140d37fab  movaps   xmmword ptr [rbp + 0x1620], xmm6
//    140d37fb2  movaps   xmmword ptr [rbp + 0x1610], xmm6
//    140d37fb9  movaps   xmmword ptr [rbp + 0x1600], xmm6
//    140d37fc0  movaps   xmmword ptr [rbp + 0x15f0], xmm6
//    140d37fc7  lea      rcx, [rbp + 0xbb0]
//    140d37fce  mov      rdx, qword ptr [rbp + 0x17a0]
//    140d37fd5  call     rbx
//    140d37fd7  nop      
//    140d37fd8  movdqu   xmm0, xmmword ptr [rbp + 0xbb0]
//    140d37fe0  movups   xmm1, xmmword ptr [rbp + 0xbc0]
//    140d37fe7  movups   xmm2, xmmword ptr [rbp + 0xbd0]
//    140d37fee  movups   xmm3, xmmword ptr [rbp + 0xbe0]
//    140d37ff5  movaps   xmmword ptr [rbp + 0xc60], xmm3
//    140d37ffc  movaps   xmmword ptr [rbp + 0xc50], xmm2
//    140d38003  movaps   xmmword ptr [rbp + 0xc40], xmm1
//    140d3800a  movdqa   xmmword ptr [rbp + 0xc30], xmm0
//    140d38012  mov      rax, qword ptr [rbp + 0x17b8]
//    140d38019  lea      rax, [rax*8]
//    140d38021  mov      qword ptr [rbp + 0x1798], rax
//    140d38028  lea      r12, [rbp + 0xc30]
//    140d3802f  lea      rbx, [rip + 0x26a20fe]
//    140d38036  mov      r14d, 1
//    140d3803c  mov      r15d, 3
//    140d38042  nop      word ptr cs:[rax + rax]
//    140d38050  mov      rcx, r12
//    140d38053  call     0x14183ed40
//    140d38058  nop      
//    140d38059  mov      rsi, rax
//    140d3805c  test     rax, rax
//    140d3805f  je       0x140d38294
//    140d38065  mov      rax, qword ptr [rsi + 0x68]
//    140d38069  dec      rax
//    140d3806c  cmp      rax, 9
//    140d38070  ja       0x140d38050
//    140d38072  movsxd   rax, dword ptr [rbx + rax*4]
//    140d38076  add      rax, rbx
//    140d38079  jmp      rax
//    140d3807b  mov      rdx, qword ptr [rsi]
//    140d3807e  mov      rax, qword ptr [rbp + 0x17c0]
//    140d38085  cmp      rdx, qword ptr [rax]
//    140d38088  jne      0x140d38050
//    140d3808a  mov      eax, 0x90
//    140d3808f  mov      ecx, 0x88
//    140d38094  test     rdx, rdx
//    140d38097  jne      0x140d3815c
//    140d3809d  jmp      0x140d38147
//    140d380a2  mov      rax, qword ptr [rsi]
//    140d380a5  mov      rcx, qword ptr [rbp + 0x17c0]
//    140d380ac  cmp      rax, qword ptr [rcx]
//    140d380af  jne      0x140d38050
//    140d380b1  test     rax, rax
//    140d380b4  jne      0x140d380c7
//    140d380b6  mov      rax, qword ptr [rsi + 8]
//    140d380ba  mov      rcx, qword ptr [rbp + 0x17c0]
//    140d380c1  cmp      rax, qword ptr [rcx + 8]
//    140d380c5  jne      0x140d38050
//    140d380c7  lea      rax, [rsi + 0x100]
//    140d380ce  jmp      0x140d38169
//    140d380d3  cmp      qword ptr [rbp + 0x1850], 0
//    140d380db  je       0x140d38050
//    140d380e1  mov      rax, qword ptr [rsi]
//    140d380e4  mov      rcx, qword ptr [rbp + 0x17c0]
//    140d380eb  cmp      rax, qword ptr [rcx]
//    140d380ee  jne      0x140d38050
//    140d380f4  test     rax, rax
//    140d380f7  jne      0x140d3810e
//    140d380f9  mov      rax, qword ptr [rsi + 8]
//    140d380fd  mov      rcx, qword ptr [rbp + 0x17c0]
//    140d38104  cmp      rax, qword ptr [rcx + 8]
//    140d38108  jne      0x140d38050
//    140d3810e  cmp      qword ptr [rsi + 0x88], 1
//    140d38116  jne      0x140d38050
//    140d3811c  lea      rax, [rsi + 0x98]
//    140d38123  jmp      0x140d38169
//    140d38125  mov      rdx, qword ptr [rsi]
//    140d38128  mov      rax, qword ptr [rbp + 0x17c0]
//    140d3812f  cmp      rdx, qword ptr [rax]
//    140d38132  jne      0x140d38050
//    140d38138  mov      eax, 0x78
//    140d3813d  mov      ecx, 0x70
//    140d38142  test     rdx, rdx
//    140d38145  jne      0x140d3815c
//    140d38147  mov      rdx, qword ptr [rsi + 8]
//    140d3814b  mov      r8, qword ptr [rbp + 0x17c0]
//    140d38152  cmp      rdx, qword ptr [r8 + 8]
//    140d38156  jne      0x140d38050
//    140d3815c  cmp      byte ptr [rsi + rcx], 0
//    140d38160  je       0x140d38050
//    140d38166  add      rax, rsi
//    140d38169  cmp      qword ptr [rbp + 0x17b8], 0xc
//    140d38171  ja       0x140d38f49
//    140d38177  mov      rax, qword ptr [rax]
//    140d3817a  mov      rcx, qword ptr [rbp + 0x1798]
//    140d38181  xor      r13d, r13d
//    140d38184  nop      word ptr cs:[rax + rax]
//    140d38190  cmp      qword ptr [rbp + r13*8 + 0x1580], rax
//    140d38198  je       0x140d381a8
//    140d3819a  inc      r13
//    140d3819d  add      rcx, -8
//    140d381a1  jne      0x140d38190
//    140d381a3  jmp      0x140d38050
//    140d381a8  cmp      dword ptr [rsi + 0x4c0], -1
//    140d381af  je       0x140d38050
//    140d381b5  mov      rdx, qword ptr [rbp + 0x1788]
//    140d381bc  cmp      r13, rdx
//    140d381bf  jae      0x140d38fe1
//    140d381c5  mov      rcx, rsi
//    140d381c8  add      rcx, 0x490
//    140d381cf  mov      rax, qword ptr [rbp + 0x1770]
//    140d381d6  mov      rax, qword ptr [rax + r13*8]
//    140d381da  mov      qword ptr [rsp + 0x20], rax
//    140d381df  mov      rdx, qword ptr [rbp + 0x17b0]
//    140d381e6  mov      r8, rsi
//    140d381e9  lea      r9, [rip + 0x26a0578]
//    140d381f0  call     0x1412857f0
//    140d381f5  nop      
//    140d381f6  mov      rdi, rax
//    140d381f9  mov      rcx, qword ptr [rsi + 0x570]
//    140d38200  mov      rax, qword ptr [rsi + 0x578]
//    140d38207  mov      rdx, rsi
//    140d3820a  call     qword ptr [rax + 0x90]
//    140d38210  nop      
//    140d38211  mov      ecx, dword ptr [rsi + 0x3fc]
//    140d38217  add      ecx, 0x64
//    140d3821a  cmp      ecx, 2
//    140d3821d  cmovl    ecx, r14d
//    140d38221  imul     rax, rax, 0x64
//    140d38225  mov      rdx, rax
//    140d38228  shr      rdx, 0x20
//    140d3822c  je       0x140d38240
//    140d3822e  xor      edx, edx
//    140d38230  div      rcx
//    140d38233  mov      rcx, rax
//    140d38236  mov      rax, rdi
//    140d38239  neg      rax
//    140d3823c  jo       0x140d3824e
//    140d3823e  jmp      0x140d38258
//    140d38240  xor      edx, edx
//    140d38242  div      ecx
//    140d38244  mov      ecx, eax
//    140d38246  mov      rax, rdi
//    140d38249  neg      rax
//    140d3824c  jno      0x140d38258
//    140d3824e  cmp      rcx, -1
//    140d38252  je       0x140d38ff3
//    140d38258  cmp      rcx, 4
//    140d3825c  cmovb    rcx, r15
//    140d38260  mov      rax, rdi
//    140d38263  or       rax, rcx
//    140d38266  shr      rax, 0x20
//    140d3826a  je       0x140d38281
//    140d3826c  mov      rax, rdi
//    140d3826f  cqo      
//    140d38271  idiv     rcx
//    140d38274  add      qword ptr [rbp + r13*8 + 0x16e0], rax
//    140d3827c  jmp      0x140d38050
//    140d38281  mov      eax, edi
//    140d38283  xor      edx, edx
//    140d38285  div      ecx
//    140d38287  add      qword ptr [rbp + r13*8 + 0x16e0], rax
//    140d3828f  jmp      0x140d38050
//    140d38294  cmp      qword ptr [rbp + 0x1850], 1
//    140d3829c  jbe      0x140d386a4
//    140d382a2  lea      rcx, [rbp + 0x16b8]
//    140d382a9  mov      rdx, qword ptr [rbp + 0x17a0]
//    140d382b0  mov      rax, qword ptr [rbp + 0x1790]
//    140d382b7  call     qword ptr [rax + 0x210]
//    140d382bd  nop      
//    140d382be  mov      rbx, qword ptr [rbp + 0x16b8]
//    140d382c5  mov      rcx, qword ptr [rbp + 0x16c0]
//    140d382cc  mov      rax, qword ptr [rbp + 0x16c8]
//    140d382d3  mov      qword ptr [rbp + 0x1750], rax
//    140d382da  movzx    r15d, word ptr [rbp + 0x16d0]
//    140d382e2  mov      rax, qword ptr [rbp + 0x16d8]
//    140d382e9  mov      qword ptr [rbp + 0x1768], rax
//    140d382f0  jmp      0x140d38300
//    140d382f2  mov      rcx, qword ptr [rbp + 0x17a8]
//    140d382f9  nop      dword ptr [rax]
//    140d38300  test     rbx, rbx
//    140d38303  je       0x140d383d0
//    140d38309  cmp      qword ptr [rbp + 0x1768], 0
//    140d38311  je       0x140d386a4
//    140d38317  test     r15w, r15w
//    140d3831b  jne      0x140d3833d
//    140d3831d  nop      dword ptr [rax]
//    140d38320  movdqa   xmm0, xmmword ptr [rcx]
//
//    --- 추가 청크(Ghidra 비연속 함수바디) 0xd3992e~0xd39c56 ---
//    140d3992e  scasd    eax, dword ptr [rdi]
//
//    --- 추가 청크(Ghidra 비연속 함수바디) 0xd3bab7~0xd3c24f ---
//    140d3bab7  div      ecx
//    140d3bab9  cmp      rax, 0x28
//    140d3babd  jb       0x140d3bb60
//    140d3bac3  jmp      0x140d3bb69
//    140d3bac8  nop      dword ptr [rax + rax]
//    140d3bad0  cmp      r12, 0x28
//    140d3bad4  je       0x140d3bd80
//    140d3bada  mov      rsi, qword ptr [rdi + r12]
//    140d3bade  test     rsi, rsi
//    140d3bae1  je       0x140d3bb60
//    140d3bae7  mov      rax, qword ptr [rsi + 0x660]
//    140d3baee  mov      rcx, qword ptr [rsi + 0x668]
//    140d3baf5  mov      rdx, qword ptr [r15 + 0x660]
//    140d3bafc  mov      r8, qword ptr [r15 + 0x668]
//    140d3bb03  mov      r9, rax
//    140d3bb06  sub      r9, rdx
//    140d3bb09  sub      rdx, rax
//    140d3bb0c  cmovb    rdx, r9
//    140d3bb10  mov      rax, rcx
//    140d3bb13  sub      rax, r8
//    140d3bb16  sub      r8, rcx
//    140d3bb19  cmovb    r8, rax
//    140d3bb1d  imul     rdx, rdx
//    140d3bb21  imul     r8, r8
//    140d3bb25  add      r8, rdx
//    140d3bb28  cmp      r8, r14
//    140d3bb2b  ja       0x140d3bb60
//    140d3bb2d  imul     rax, qword ptr [rsi + 0x670], 0x64
//    140d3bb35  mov      rcx, qword ptr [rsi + 0x628]
//    140d3bb3c  cmp      rcx, 1
//    140d3bb40  adc      rcx, 0
//    140d3bb44  mov      rdx, rax
//    140d3bb47  or       rdx, rcx
//    140d3bb4a  shr      rdx, 0x20
//    140d3bb4e  jne      0x140d3bab4
//    140d3bb54  xor      edx, edx
//    140d3bb56  div      ecx
//    140d3bb58  cmp      rax, 0x28
//    140d3bb5c  jae      0x140d3bb69
//    140d3bb5e  nop      
//    140d3bb60  add      r12, 8
//    140d3bb64  jmp      0x140d3bad0
//    140d3bb69  mov      rdx, qword ptr [rsi + 0x5c0]
//    140d3bb70  mov      rcx, qword ptr [rbp + 0x110]
//    140d3bb77  call     0x140d31bb0
//    140d3bb7c  test     rax, rax
//    140d3bb7f  je       0x140d3c469
//    140d3bb85  mov      r14, qword ptr [rax + 0x930]
//    140d3bb8c  cmp      r14, 2
//    140d3bb90  jae      0x140d3c4b3
//    140d3bb96  mov      edi, dword ptr [rax + 0x9c0]
//    140d3bb9c  mov      rcx, rsi
//    140d3bb9f  call     0x14128cc90
//    140d3bba4  nop      
//    140d3bba5  imul     rcx, r14, 0xfa0
//    140d3bbac  add      rcx, qword ptr [rbp + 0xe8]
//    140d3bbb3  imul     rbx, rdi, 0x320
//    140d3bbba  add      rbx, rcx
//    140d3bbbd  test     al, al
//    140d3bbbf  jne      0x140d3bc98
//    140d3bbc5  mov      rax, qword ptr [rsi + 0x68]
//    140d3bbc9  lea      rcx, [rip + 0x269e5a8]
//    140d3bbd0  movsxd   rax, dword ptr [rcx + rax*4]
//    140d3bbd4  add      rax, rcx
//    140d3bbd7  jmp      rax
//    140d3bbd9  mov      eax, 0xe8
//    140d3bbde  mov      rcx, qword ptr [rbp + 0x178]
//    140d3bbe5  cmp      qword ptr [rsi + rax], rcx
//    140d3bbe9  ja       0x140d3bc81
//    140d3bbef  jmp      0x140d3bc98
//    140d3bbf4  mov      eax, 0xb0
//    140d3bbf9  mov      rcx, qword ptr [rbp + 0x178]
//    140d3bc00  cmp      qword ptr [rsi + rax], rcx
//    140d3bc04  ja       0x140d3bc81
//    140d3bc06  jmp      0x140d3bc98
//    140d3bc0b  mov      eax, 0x1f0
//    140d3bc10  mov      rcx, qword ptr [rbp + 0x178]
//    140d3bc17  cmp      qword ptr [rsi + rax], rcx
//    140d3bc1b  ja       0x140d3bc81
//    140d3bc1d  jmp      0x140d3bc98
//    140d3bc1f  mov      eax, 0xf0
//    140d3bc24  mov      rcx, qword ptr [rbp + 0x178]
//    140d3bc2b  cmp      qword ptr [rsi + rax], rcx
//    140d3bc2f  ja       0x140d3bc81
//    140d3bc31  jmp      0x140d3bc98
//    140d3bc33  mov      eax, 0xd8
//    140d3bc38  mov      rcx, qword ptr [rbp + 0x178]
//    140d3bc3f  cmp      qword ptr [rsi + rax], rcx
//    140d3bc43  ja       0x140d3bc81
//    140d3bc45  jmp      0x140d3bc98
//    140d3bc47  mov      eax, 0xb8
//    140d3bc4c  mov      rcx, qword ptr [rbp + 0x178]
//    140d3bc53  cmp      qword ptr [rsi + rax], rcx
//    140d3bc57  ja       0x140d3bc81
//    140d3bc59  jmp      0x140d3bc98
//    140d3bc5b  mov      eax, 0x110
//    140d3bc60  mov      rcx, qword ptr [rbp + 0x178]
//    140d3bc67  cmp      qword ptr [rsi + rax], rcx
//    140d3bc6b  ja       0x140d3bc81
//    140d3bc6d  jmp      0x140d3bc98
//    140d3bc6f  mov      eax, 0xd0
//    140d3bc74  mov      rcx, qword ptr [rbp + 0x178]
//    140d3bc7b  cmp      qword ptr [rsi + rax], rcx
//    140d3bc7f  jbe      0x140d3bc98
//    140d3bc81  xor      r14d, r14d
//    140d3bc84  jmp      0x140d3bca3
//    140d3bc86  mov      eax, 0xc8
//    140d3bc8b  mov      rcx, qword ptr [rbp + 0x178]
//    140d3bc92  cmp      qword ptr [rsi + rax], rcx
//    140d3bc96  ja       0x140d3bc81
//    140d3bc98  mov      rax, qword ptr [rbp + 0x118]
//    140d3bc9f  mov      r14, qword ptr [rbx + rax*8]
//    140d3bca3  mov      rcx, rsi
//    140d3bca6  call     0x14129ed50
//    140d3bcab  nop      
//    140d3bcac  test     al, al
//    140d3bcae  jne      0x140d3bcc6
//    140d3bcb0  cmp      dword ptr [rsi + 0x68], 0xd
//    140d3bcb4  jne      0x140d3bcc6
//    140d3bcb6  mov      rax, qword ptr [rbp + 0x178]
//    140d3bcbd  cmp      qword ptr [rsi + 0xb8], rax
//    140d3bcc4  ja       0x140d3bcd9
//    140d3bcc6  mov      rax, qword ptr [rbp + 0x118]
//    140d3bccd  mov      rax, qword ptr [rbx + rax*8 + 0x28]
//    140d3bcd2  cmp      rax, r14
//    140d3bcd5  cmova    r14, rax
//    140d3bcd9  mov      rcx, rsi
//    140d3bcdc  call     0x14128cf70
//    140d3bce1  nop      
//    140d3bce2  test     al, al
//    140d3bce4  jne      0x140d3bd0c
//    140d3bce6  cmp      dword ptr [rsi + 0x68], 0xd
//    140d3bcea  jne      0x140d3bd0c
//    140d3bcec  mov      rax, qword ptr [rbp + 0x178]
//    140d3bcf3  cmp      qword ptr [rsi + 0xc0], rax
//    140d3bcfa  jbe      0x140d3bd0c
//    140d3bcfc  mov      rdi, qword ptr [rbp + 0x190]
//    140d3bd03  mov      rcx, qword ptr [rbp + 0x118]
//    140d3bd0a  jmp      0x140d3bd26
//    140d3bd0c  mov      rcx, qword ptr [rbp + 0x118]
//    140d3bd13  mov      rax, qword ptr [rbx + rcx*8 + 0x50]
//    140d3bd18  cmp      rax, r14
//    140d3bd1b  cmova    r14, rax
//    140d3bd1f  mov      rdi, qword ptr [rbp + 0x190]
//    140d3bd26  mov      r13, qword ptr [rbp + 0x180]
//    140d3bd2d  mov      rax, qword ptr [rbp + 0x188]
//    140d3bd34  add      rax, qword ptr [rbx + rcx*8 + 0x190]
//    140d3bd3c  add      rax, qword ptr [rbx + rcx*8 + 0x1b8]
//    140d3bd44  add      r12, 8
//    140d3bd48  add      rax, qword ptr [rbx + rcx*8 + 0x1e0]
//    140d3bd50  mov      qword ptr [rbp + 0x188], rax
//    140d3bd57  add      qword ptr [rbp + 0x170], r14
//    140d3bd5e  movabs   r14, 0x53d1ac100
//    140d3bd68  mov      rbx, qword ptr [rbp + 0x158]
//    140d3bd6f  jmp      0x140d3bad0
//    140d3bd74  nop      word ptr cs:[rax + rax]
//    140d3bd80  mov      rax, qword ptr [rbp - 0x10]
//    140d3bd84  sub      rax, qword ptr [rbp + 0x170]
//    140d3bd8b  mov      ecx, 0
//    140d3bd90  cmovae   rcx, rax
//    140d3bd94  imul     rax, rcx, 0x3c
//    140d3bd98  mov      r8, qword ptr [rbp + 0x188]
//    140d3bd9f  cmp      r8, 1
//    140d3bda3  adc      r8, 0
//    140d3bda7  mov      rcx, rax
//    140d3bdaa  or       rcx, r8
//    140d3bdad  shr      rcx, 0x20
//    140d3bdb1  je       0x140d3bdc0
//    140d3bdb3  xor      edx, edx
//    140d3bdb5  div      r8
//    140d3bdb8  jmp      0x140d3bdc5
//    140d3bdba  nop      word ptr [rax + rax]
//    140d3bdc0  xor      edx, edx
//    140d3bdc2  div      r8d
//    140d3bdc5  mov      qword ptr [rbp + 0x188], rax
//    140d3bdcc  xor      r12d, r12d
//    140d3bdcf  xor      r11d, r11d
//    140d3bdd2  xor      r10d, r10d
//    140d3bdd5  cmp      r12, 0x28
//    140d3bdd9  jne      0x140d3bdee
//    140d3bddb  jmp      0x140d3c130
//    140d3bde0  add      r12, 8
//    140d3bde4  cmp      r12, 0x28
//    140d3bde8  je       0x140d3c130
//    140d3bdee  mov      rsi, qword ptr [rbx + r12]
//    140d3bdf2  test     rsi, rsi
//    140d3bdf5  je       0x140d3bde0
//    140d3bdf7  mov      rdx, qword ptr [rsi + 0x660]
//    140d3bdfe  mov      rax, qword ptr [rsi + 0x668]
//    140d3be05  mov      rcx, qword ptr [r15 + 0x660]
//    140d3be0c  mov      r8, qword ptr [r15 + 0x668]
//    140d3be13  mov      r9, rdx
//    140d3be16  sub      r9, rcx
//    140d3be19  sub      rcx, rdx
//    140d3be1c  cmovb    rcx, r9
//    140d3be20  mov      r9, rax
//    140d3be23  sub      r9, r8
//    140d3be26  sub      r8, rax
//    140d3be29  cmovb    r8, r9
//    140d3be2d  imul     rcx, rcx
//    140d3be31  imul     r8, r8
//    140d3be35  add      r8, rcx
//    140d3be38  cmp      r8, r14
//    140d3be3b  ja       0x140d3bde0
//    140d3be3d  cmp      byte ptr [r15], 0
//    140d3be41  jne      0x140d3be5d
//    140d3be43  mov      rcx, qword ptr [r15 + 8]
//    140d3be47  cmp      rcx, 1
//    140d3be4b  ja       0x140d3c488
//    140d3be51  lea      rcx, [rcx + rcx*2]
//    140d3be55  cmp      qword ptr [rsi + rcx*8 + 0x38], 0
//    140d3be5b  jne      0x140d3bde0
//    140d3be5d  cmp      byte ptr [rsi], 0
//    140d3be60  jne      0x140d3bf0f
//    140d3be66  mov      rcx, qword ptr [rbp + 0x148]
//    140d3be6d  mov      rcx, qword ptr [rcx + 0x930]
//    140d3be74  mov      r8d, 1
//    140d3be7a  sub      r8, rcx
//    140d3be7d  cmp      qword ptr [rsi + 8], r8
//    140d3be81  jne      0x140d3bf0f
//    140d3be87  cmp      rcx, 1
//    140d3be8b  jne      0x140d3becf
//    140d3be8d  cmp      rdx, 0xfa01
//    140d3be94  setb     cl
//    140d3be97  lea      r8, [rax - 0xc3500]
//    140d3be9e  cmp      r8, 0x27101
//    140d3bea5  setb     r8b
//    140d3bea9  test     cl, r8b
//    140d3beac  jne      0x140d3bde0
//    140d3beb2  cmp      rdx, 0x27100
//    140d3beb9  ja       0x140d3bf0f
//    140d3bebb  add      rax, -0xdac00
//    140d3bec1  cmp      rax, 0xfa01
//    140d3bec7  jb       0x140d3bde0
//    140d3becd  jmp      0x140d3bf0f
//    140d3becf  lea      rcx, [rdx - 0xc3500]
//    140d3bed6  cmp      rcx, 0x27101
//    140d3bedd  setb     cl
//    140d3bee0  cmp      rax, 0xfa01
//    140d3bee6  setb     r8b
//    140d3beea  test     cl, r8b
//    140d3beed  jne      0x140d3bde0
//    140d3bef3  add      rdx, -0xdac00
//    140d3befa  cmp      rdx, 0xfa00
//    140d3bf01  ja       0x140d3bf0f
//    140d3bf03  cmp      rax, 0x27101
//    140d3bf09  jb       0x140d3bde0
//    140d3bf0f  mov      rbx, r11
//    140d3bf12  mov      qword ptr [rbp + 0x170], r10
//    140d3bf19  mov      rdx, qword ptr [rsi + 0x5c0]
//    140d3bf20  mov      rcx, qword ptr [rbp + 0x110]
//    140d3bf27  call     0x140d31bb0
//    140d3bf2c  test     rax, rax
//    140d3bf2f  je       0x140d3c45a
//    140d3bf35  mov      r14, qword ptr [rax + 0x930]
//    140d3bf3c  cmp      r14, 2
//    140d3bf40  jae      0x140d3c49c
//    140d3bf46  mov      edi, dword ptr [rax + 0x9c0]
//    140d3bf4c  mov      rcx, rsi
//    140d3bf4f  call     0x14128cc90
//    140d3bf54  nop      
//    140d3bf55  imul     rcx, r14, 0xfa0
//    140d3bf5c  add      rcx, qword ptr [rbp + 0xe8]
//    140d3bf63  imul     rdi, rdi, 0x320
//    140d3bf6a  add      rdi, rcx
//    140d3bf6d  test     al, al
//    140d3bf6f  mov      rdx, qword ptr [rbp + 0x168]
//    140d3bf76  jne      0x140d3c04f
//    140d3bf7c  mov      rax, qword ptr [rsi + 0x68]
//    140d3bf80  lea      rcx, [rip + 0x269e229]
//    140d3bf87  movsxd   rax, dword ptr [rcx + rax*4]
//    140d3bf8b  add      rax, rcx
//    140d3bf8e  jmp      rax
//    140d3bf90  mov      eax, 0xe8
//    140d3bf95  mov      rcx, qword ptr [rbp + 0x178]
//    140d3bf9c  cmp      qword ptr [rsi + rax], rcx
//    140d3bfa0  ja       0x140d3c038
//    140d3bfa6  jmp      0x140d3c04f
//    140d3bfab  mov      eax, 0xb0
//    140d3bfb0  mov      rcx, qword ptr [rbp + 0x178]
//    140d3bfb7  cmp      qword ptr [rsi + rax], rcx
//    140d3bfbb  ja       0x140d3c038
//    140d3bfbd  jmp      0x140d3c04f
//    140d3bfc2  mov      eax, 0x1f0
//    140d3bfc7  mov      rcx, qword ptr [rbp + 0x178]
//    140d3bfce  cmp      qword ptr [rsi + rax], rcx
//    140d3bfd2  ja       0x140d3c038
//    140d3bfd4  jmp      0x140d3c04f
//    140d3bfd6  mov      eax, 0xf0
//    140d3bfdb  mov      rcx, qword ptr [rbp + 0x178]
//    140d3bfe2  cmp      qword ptr [rsi + rax], rcx
//    140d3bfe6  ja       0x140d3c038
//    140d3bfe8  jmp      0x140d3c04f
//    140d3bfea  mov      eax, 0xd8
//    140d3bfef  mov      rcx, qword ptr [rbp + 0x178]
//    140d3bff6  cmp      qword ptr [rsi + rax], rcx
//    140d3bffa  ja       0x140d3c038
//    140d3bffc  jmp      0x140d3c04f
//    140d3bffe  mov      eax, 0xb8
//    140d3c003  mov      rcx, qword ptr [rbp + 0x178]
//    140d3c00a  cmp      qword ptr [rsi + rax], rcx
//    140d3c00e  ja       0x140d3c038
//    140d3c010  jmp      0x140d3c04f
//    140d3c012  mov      eax, 0x110
//    140d3c017  mov      rcx, qword ptr [rbp + 0x178]
//    140d3c01e  cmp      qword ptr [rsi + rax], rcx
//    140d3c022  ja       0x140d3c038
//    140d3c024  jmp      0x140d3c04f
//    140d3c026  mov      eax, 0xd0
//    140d3c02b  mov      rcx, qword ptr [rbp + 0x178]
//    140d3c032  cmp      qword ptr [rsi + rax], rcx
//    140d3c036  jbe      0x140d3c04f
//    140d3c038  xor      r14d, r14d
//    140d3c03b  jmp      0x140d3c053
//    140d3c03d  mov      eax, 0xc8
//    140d3c042  mov      rcx, qword ptr [rbp + 0x178]
//    140d3c049  cmp      qword ptr [rsi + rax], rcx
//    140d3c04d  ja       0x140d3c038
//    140d3c04f  mov      r14, qword ptr [rdi + rdx*8]
//    140d3c053  mov      rcx, rsi
//    140d3c056  call     0x14129ed50
//    140d3c05b  nop      
//    140d3c05c  test     al, al
//    140d3c05e  jne      0x140d3c076
//    140d3c060  cmp      dword ptr [rsi + 0x68], 0xd
//    140d3c064  jne      0x140d3c076
//    140d3c066  mov      rax, qword ptr [rbp + 0x178]
//    140d3c06d  cmp      qword ptr [rsi + 0xb8], rax
//    140d3c074  ja       0x140d3c089
//    140d3c076  mov      rax, qword ptr [rbp + 0x168]
//    140d3c07d  mov      rax, qword ptr [rdi + rax*8 + 0x28]
//    140d3c082  cmp      rax, r14
//    140d3c085  cmova    r14, rax
//    140d3c089  mov      rcx, rsi
//    140d3c08c  call     0x14128cf70
//    140d3c091  nop      
//    140d3c092  test     al, al
//    140d3c094  mov      r10, qword ptr [rbp + 0x170]
//    140d3c09b  mov      r11, rbx
//    140d3c09e  jne      0x140d3c0c9
//    140d3c0a0  cmp      dword ptr [rsi + 0x68], 0xd
//    140d3c0a4  jne      0x140d3c0c9
//    140d3c0a6  mov      rax, qword ptr [rbp + 0x178]
//    140d3c0ad  cmp      qword ptr [rsi + 0xc0], rax
//    140d3c0b4  jbe      0x140d3c0c9
//    140d3c0b6  mov      rax, rdi
//    140d3c0b9  mov      rdi, qword ptr [rbp + 0x190]
//    140d3c0c0  mov      rcx, qword ptr [rbp + 0x168]
//    140d3c0c7  jmp      0x140d3c0e6
//    140d3c0c9  mov      rcx, qword ptr [rbp + 0x168]
//    140d3c0d0  mov      rax, qword ptr [rdi + rcx*8 + 0x50]
//    140d3c0d5  cmp      rax, r14
//    140d3c0d8  cmova    r14, rax
//    140d3c0dc  mov      rax, rdi
//    140d3c0df  mov      rdi, qword ptr [rbp + 0x190]
//    140d3c0e6  mov      r13, qword ptr [rbp + 0x180]
//    140d3c0ed  add      r10, qword ptr [rax + rcx*8 + 0x190]
//    140d3c0f5  add      r10, qword ptr [rax + rcx*8 + 0x1b8]
//    140d3c0fd  add      r12, 8
//    140d3c101  add      r10, qword ptr [rax + rcx*8 + 0x1e0]
//    140d3c109  add      r11, r14
//    140d3c10c  movabs   r14, 0x53d1ac100
//    140d3c116  mov      rbx, qword ptr [rbp + 0x158]
//    140d3c11d  cmp      r12, 0x28
//    140d3c121  jne      0x140d3bdee
//    140d3c127  nop      word ptr [rax + rax]
//    140d3c130  mov      r14, qword ptr [rbp + 0x160]
//    140d3c137  nop      word ptr [rax + rax]
//    140d3c140  cmp      r14, r13
//    140d3c143  je       0x140d3b9a0
//    140d3c149  mov      r12, qword ptr [r14]
//    140d3c14c  add      r14, 8
//    140d3c150  cmp      dword ptr [r12 + 0x4c0], -1
//    140d3c159  je       0x140d3c140
//    140d3c15b  mov      r13, r11
//    140d3c15e  mov      rbx, r10
//    140d3c161  lea      rcx, [r12 + 0x490]
//    140d3c169  mov      qword ptr [rsp + 0x20], r15
//    140d3c16e  mov      rdx, qword ptr [rbp + 0xe0]
//    140d3c175  mov      r8, r12
//    140d3c178  lea      r9, [rip + 0x269c5e9]
//    140d3c17f  call     0x1412857f0
//    140d3c184  nop      
//    140d3c185  mov      rsi, rax
//    140d3c188  mov      rcx, qword ptr [r12 + 0x570]
//    140d3c190  mov      rax, qword ptr [r12 + 0x578]
//    140d3c198  mov      rdx, r12
//    140d3c19b  call     qword ptr [rax + 0x90]
//    140d3c1a1  nop      
//    140d3c1a2  mov      ecx, dword ptr [r12 + 0x3fc]
//    140d3c1aa  add      ecx, 0x64
//    140d3c1ad  cmp      ecx, 2
//    140d3c1b0  mov      edx, 1
//    140d3c1b5  cmovl    ecx, edx
//    140d3c1b8  imul     rax, rax, 0x64
//    140d3c1bc  mov      rdx, rax
//    140d3c1bf  shr      rdx, 0x20
//    140d3c1c3  je       0x140d3c1cf
//    140d3c1c5  xor      edx, edx
//    140d3c1c7  div      rcx
//    140d3c1ca  mov      rcx, rax
//    140d3c1cd  jmp      0x140d3c1d5
//    140d3c1cf  xor      edx, edx
//    140d3c1d1  div      ecx
//    140d3c1d3  mov      ecx, eax
//    140d3c1d5  mov      rdi, qword ptr [rbp + 0x190]
//    140d3c1dc  mov      r10, rbx
//    140d3c1df  mov      r11, r13
//    140d3c1e2  cmp      rcx, 4
//    140d3c1e6  mov      eax, 3
//    140d3c1eb  cmovb    rcx, rax
//    140d3c1ef  mov      rax, rsi
//    140d3c1f2  imul     rax, qword ptr [rbp + 0x178]
//    140d3c1fa  mov      rdx, rax
//    140d3c1fd  or       rdx, rcx
//    140d3c200  shr      rdx, 0x20
//    140d3c204  mov      rbx, qword ptr [rbp + 0x158]
//    140d3c20b  mov      r13, qword ptr [rbp + 0x180]
//    140d3c212  je       0x140d3c21b
//    140d3c214  xor      edx, edx
//    140d3c216  div      rcx
//    140d3c219  jmp      0x140d3c21f
//    140d3c21b  xor      edx, edx
//    140d3c21d  div      ecx
//    140d3c21f  add      r10, rax
//    140d3c222  add      r11, rsi
//    140d3c225  jmp      0x140d3c140
//    140d3c22a  nop      word ptr [rax + rax]
//    140d3c230  xor      edx, edx
//    140d3c232  div      r10d
//    140d3c235  jmp      0x140d3b9d4
//    140d3c23a  nop      word ptr [rax + rax]
//    140d3c240  cmp      dword ptr [r15 + 0x4c0], -1
//    140d3c248  je       0x140d3c29f
//
//    --- 추가 청크(Ghidra 비연속 함수바디) 0xd3c680~0xd3c978 ---
//    140d3c680  push     r14
//    140d3c682  push     rsi
//    140d3c683  push     rdi
//    140d3c684  push     rbx
//    140d3c685  sub      rsp, 0x58
//    140d3c689  mov      rsi, rdx
//    140d3c68c  mov      rdi, rcx
//    140d3c68f  mov      rax, qword ptr [rdx]
//    140d3c692  mov      rbx, qword ptr [rax]
//    140d3c695  mov      r14, qword ptr [rax + 8]
//    140d3c699  mov      rcx, rbx
//    140d3c69c  call     qword ptr [r14 + 0x20]
//    140d3c6a0  mov      qword ptr [rsp + 0x28], rax
//    140d3c6a5  mov      rcx, rbx
//    140d3c6a8  call     qword ptr [r14 + 0x28]
//    140d3c6ac  mov      qword ptr [rsp + 0x30], rax
//    140d3c6b1  lea      rax, [rsp + 0x28]
//    140d3c6b6  mov      qword ptr [rsp + 0x38], rax
//    140d3c6bb  lea      rax, [rsp + 0x30]
//    140d3c6c0  mov      qword ptr [rsp + 0x40], rax
//    140d3c6c5  mov      qword ptr [rsp + 0x48], rdi
//    140d3c6ca  mov      qword ptr [rsp + 0x50], rsi
//    140d3c6cf  lea      rcx, [rip + 0x269d0fa]
//    140d3c6d6  lea      rdx, [rsp + 0x38]
//    140d3c6db  call     0x140c87850
//    140d3c6e0  test     eax, 0x10001
//    140d3c6e5  setne    al
//    140d3c6e8  add      rsp, 0x58
//    140d3c6ec  pop      rbx
//    140d3c6ed  pop      rdi
//    140d3c6ee  pop      rsi
//    140d3c6ef  pop      r14
//    140d3c6f1  ret      
//    140d3c6f2  int3     
//    140d3c6f3  int3     
//    140d3c6f4  int3     
//    140d3c6f5  int3     
//    140d3c6f6  int3     
//    140d3c6f7  int3     
//    140d3c6f8  int3     
//    140d3c6f9  int3     
//    140d3c6fa  int3     
//    140d3c6fb  int3     
//    140d3c6fc  int3     
//    140d3c6fd  int3     
//    140d3c6fe  int3     
//    140d3c6ff  int3     
//    140d3c700  push     rbp
//    140d3c701  push     r15
//    140d3c703  push     r14
//    140d3c705  push     r13
//    140d3c707  push     r12
//    140d3c709  push     rsi
//    140d3c70a  push     rdi
//    140d3c70b  push     rbx
//    140d3c70c  sub      rsp, 0x108
//    140d3c713  lea      rbp, [rsp + 0x80]
//    140d3c71b  movaps   xmmword ptr [rbp + 0x70], xmm6
//    140d3c71f  mov      qword ptr [rbp + 0x68], 0xfffffffffffffffe
//    140d3c727  mov      rcx, qword ptr [r8 + 0x930]
//    140d3c72e  cmp      rcx, 1
//    140d3c732  ja       0x140d3cebe
//    140d3c738  mov      rax, qword ptr [r9]
//    140d3c73b  mov      rdx, qword ptr [rax + rcx*8 + 0x170]
//    140d3c743  mov      qword ptr [rbp + 0x20], rdx
//    140d3c747  test     rdx, rdx
//    140d3c74a  je       0x140d3ceb0
//    140d3c750  mov      qword ptr [rbp + 0x50], rax
//    140d3c754  mov      qword ptr [rbp + 0x48], rcx
//    140d3c758  mov      rax, qword ptr [r9 + 8]
//    140d3c75c  mov      qword ptr [rbp + 0x18], rax
//    140d3c760  movzx    eax, byte ptr [rax + 0x38]
//    140d3c764  lea      rcx, [rip + 0x269da7d]
//    140d3c76b  movsxd   rax, dword ptr [rcx + rax*4]
//    140d3c76f  add      rax, rcx
//    140d3c772  jmp      rax
//    140d3c774  lea      r8, [rip + 0x269c8bf]
//    140d3c77b  mov      r10d, 3
//    140d3c781  jmp      0x140d3c7bd
//    140d3c783  lea      r8, [rip + 0x269c609]
//    140d3c78a  mov      r10d, 1
//    140d3c790  jmp      0x140d3c7bd
//    140d3c792  lea      r8, [rip + 0x269c1bf]
//    140d3c799  mov      r10d, 1
//    140d3c79f  jmp      0x140d3c7bd
//    140d3c7a1  lea      r8, [rip + 0x269c708]
//    140d3c7a8  mov      r10d, 1
//    140d3c7ae  jmp      0x140d3c7bd
//    140d3c7b0  lea      r8, [rip + 0x269c881]
//    140d3c7b7  mov      r10d, 2
//    140d3c7bd  add      r10, r8
//    140d3c7c0  mov      rcx, qword ptr [rbp + 0x48]
//    140d3c7c4  mov      rdx, qword ptr [rbp + 0x50]
//    140d3c7c8  lea      r11, [rdx + rcx*8]
//    140d3c7cc  test     rcx, rcx
//    140d3c7cf  mov      esi, 0xdea80
//    140d3c7d4  mov      eax, 0x69780
//    140d3c7d9  mov      edi, 0x69780
//    140d3c7de  cmove    rdi, rsi
//    140d3c7e2  mov      qword ptr [rbp - 0x10], rdi
//    140d3c7e6  cmove    rsi, rax
//    140d3c7ea  mov      qword ptr [rbp - 0x18], rsi
//    140d3c7ee  mov      esi, 0xafc80
//    140d3c7f3  mov      eax, 0x3a980
//    140d3c7f8  cmove    rax, rsi
//    140d3c7fc  mov      qword ptr [rbp - 0x20], rax
//    140d3c800  mov      eax, 0x3a980
//    140d3c805  cmove    rsi, rax
//    140d3c809  mov      qword ptr [rbp - 0x28], rsi
//    140d3c80d  mov      esi, 0x80e80
//    140d3c812  mov      eax, 0xbb80
//    140d3c817  mov      edi, 0xbb80
//    140d3c81c  cmove    rdi, rsi
//    140d3c820  mov      qword ptr [rbp - 0x40], rdi
//    140d3c824  cmove    rsi, rax
//    140d3c828  mov      qword ptr [rbp - 0x48], rsi
//    140d3c82c  mov      eax, 1
//    140d3c831  sub      rax, rcx
//    140d3c834  imul     rcx, rax, 0x2e8
//    140d3c83b  add      rcx, qword ptr [r9 + 0x10]
//    140d3c83f  mov      qword ptr [rbp - 0x58], rcx
//    140d3c843  lea      rax, [rax + rax*4]
//    140d3c847  lea      rdi, [rdx + rax*8]
//    140d3c84b  add      rdi, 0x1e0
//    140d3c852  xorps    xmm6, xmm6
//    140d3c855  mov      qword ptr [rbp - 0x38], r10
//    140d3c859  mov      qword ptr [rbp - 0x30], r11
//    140d3c85d  jmp      0x140d3c86c
//    140d3c85f  nop      
//    140d3c860  inc      r8
//    140d3c863  cmp      r8, r10
//    140d3c866  je       0x140d3cb06
//    140d3c86c  movzx    eax, byte ptr [r8]
//    140d3c870  mov      ecx, eax
//    140d3c872  shl      ecx, 5
//    140d3c875  mov      rdx, qword ptr [r11 + rcx + 0x180]
//    140d3c87d  or       rdx, qword ptr [r11 + rcx + 0x190]
//    140d3c885  jne      0x140d3c860
//    140d3c887  mov      qword ptr [rbp - 0x50], r8
//    140d3c88b  mov      rcx, qword ptr [rbp - 0x48]
//    140d3c88f  mov      qword ptr [rbp], rcx
//    140d3c893  mov      rcx, qword ptr [rbp - 0x40]
//    140d3c897  mov      qword ptr [rbp + 8], rcx
//    140d3c89b  test     eax, eax
//    140d3c89d  je       0x140d3c8c4
//    140d3c89f  mov      rcx, qword ptr [rbp - 0x28]
//    140d3c8a3  mov      qword ptr [rbp], rcx
//    140d3c8a7  mov      rcx, qword ptr [rbp - 0x20]
//    140d3c8ab  mov      qword ptr [rbp + 8], rcx
//    140d3c8af  cmp      eax, 2
//    140d3c8b2  jne      0x140d3c8c4
//    140d3c8b4  mov      rax, qword ptr [rbp - 0x18]
//    140d3c8b8  mov      qword ptr [rbp], rax
//    140d3c8bc  mov      rax, qword ptr [rbp - 0x10]
//    140d3c8c0  mov      qword ptr [rbp + 8], rax
//    140d3c8c4  mov      rax, qword ptr [rbp + 0x18]
//    140d3c8c8  mov      rax, qword ptr [rax]
//    140d3c8cb  mov      qword ptr [rbp + 0x28], 8
//    140d3c8d3  mov      qword ptr [rbp + 0x30], rax
//    140d3c8d7  lea      rax, [rbp + 0x38]
//    140d3c8db  movups   xmmword ptr [rax], xmm6
//    140d3c8de  xor      r14d, r14d
//    140d3c8e1  xor      esi, esi
//    140d3c8e3  nop      word ptr cs:[rax + rax]
//    140d3c8f0  cmp      rsi, 0x28
//    140d3c8f4  je       0x140d3c940
//    140d3c8f6  mov      rbx, qword ptr [rdi + rsi]
//    140d3c8fa  add      rsi, 8
//    140d3c8fe  test     rbx, rbx
//    140d3c901  je       0x140d3c8f0
//    140d3c903  cmp      r14, qword ptr [rbp + 0x38]
//    140d3c907  jne      0x140d3c923
//    140d3c909  mov      r8d, 1
//    140d3c90f  lea      rcx, [rbp + 0x28]
//    140d3c913  mov      rdx, r14
//    140d3c916  mov      r9b, 1
//    140d3c919  call     0x1410ea8e0
//    140d3c91e  nop      
//    140d3c91f  mov      r14, qword ptr [rbp + 0x40]
//    140d3c923  mov      rax, qword ptr [rbp + 0x28]
//    140d3c927  mov      qword ptr [rax + r14*8], rbx
//    140d3c92b  mov      r14, qword ptr [rbp + 0x40]
//    140d3c92f  inc      r14
//    140d3c932  mov      qword ptr [rbp + 0x40], r14
//    140d3c936  jmp      0x140d3c8f0
//    140d3c938  nop      dword ptr [rax + rax]
//    140d3c940  mov      rax, qword ptr [rbp + 0x28]
//    140d3c944  mov      qword ptr [rbp + 0x58], rax
//    140d3c948  mov      rax, qword ptr [rbp + 0x30]
//    140d3c94c  mov      qword ptr [rbp + 0x10], rax
//    140d3c950  mov      rax, qword ptr [rbp + 0x38]
//    140d3c954  mov      qword ptr [rbp + 0x60], rax
//    140d3c958  test     r14, r14
//    140d3c95b  je       0x140d3ca90
//    140d3c961  shl      r14, 3
//    140d3c965  mov      rax, qword ptr [rbp + 0x50]
//    140d3c969  mov      r15, qword ptr [rax]
//    140d3c96c  mov      rax, qword ptr [rax + 8]
//    140d3c970  mov      qword ptr [rbp - 8], rax
//
//    --- 추가 청크(Ghidra 비연속 함수바디) 0xd3ca12~0xd3ccce ---
//    140d3ca12  add      esi, 8
//    140d3ca15  cmp      r14, rsi
//    140d3ca18  je       0x140d3ca90
//    140d3ca1a  mov      rax, qword ptr [rbp + 0x58]
//    140d3ca1e  mov      rbx, qword ptr [rax + rsi]
//    140d3ca22  mov      r12, qword ptr [rbx + 0x5c0]
//    140d3ca29  mov      rcx, r15
//    140d3ca2c  mov      rdx, qword ptr [rbp + 0x48]
//    140d3ca30  mov      r8, r12
//    140d3ca33  call     r13
//    140d3ca36  nop      
//    140d3ca37  test     al, al
//    140d3ca39  jne      0x140d3c990
//    140d3ca3f  mov      rcx, r15
//    140d3ca42  mov      rdx, r12
//    140d3ca45  mov      rax, qword ptr [rbp - 8]
//    140d3ca49  call     qword ptr [rax + 0x150]
//    140d3ca4f  nop      
//    140d3ca50  test     rax, rax
//    140d3ca53  je       0x140d3ca11
//    140d3ca55  mov      eax, dword ptr [rax + 0x9c0]
//    140d3ca5b  mov      rcx, qword ptr [rbp - 0x58]
//    140d3ca5f  mov      r12, qword ptr [rcx + rax*8 + 0x1e0]
//    140d3ca67  mov      rcx, r15
//    140d3ca6a  mov      rax, qword ptr [rbp - 8]
//    140d3ca6e  call     qword ptr [rax + 0x28]
//    140d3ca71  nop      
//    140d3ca72  add      r12, 0x78
//    140d3ca76  cmp      r12, rax
//    140d3ca79  jae      0x140d3c990
//    140d3ca7f  jmp      0x140d3ca11
//    140d3ca81  nop      word ptr cs:[rax + rax]
//    140d3ca90  cmp      qword ptr [rbp + 0x60], 0
//    140d3ca95  mov      r8, qword ptr [rbp - 0x50]
//    140d3ca99  mov      r10, qword ptr [rbp - 0x38]
//    140d3ca9d  mov      r11, qword ptr [rbp - 0x30]
//    140d3caa1  je       0x140d3c860
//    140d3caa7  mov      rax, qword ptr [rbp + 0x10]
//    140d3caab  mov      rax, qword ptr [rax + 0x10]
//    140d3caaf  mov      rcx, qword ptr [rbp + 0x58]
//    140d3cab3  cmp      qword ptr [rax + 0x20], rcx
//    140d3cab7  jne      0x140d3c860
//    140d3cabd  mov      rcx, qword ptr [rbp + 0x58]
//    140d3cac1  mov      rdx, qword ptr [rbp + 0x60]
//    140d3cac5  lea      rcx, [rcx + rdx*8]
//    140d3cac9  mov      qword ptr [rax + 0x20], rcx
//    140d3cacd  jmp      0x140d3c860
//    140d3cad2  mov      al, 1
//    140d3cad4  cmp      qword ptr [rbp + 0x60], 0
//    140d3cad9  je       0x140d3ce98
//    140d3cadf  mov      rcx, qword ptr [rbp + 0x10]
//    140d3cae3  mov      rcx, qword ptr [rcx + 0x10]
//    140d3cae7  mov      rdx, qword ptr [rbp + 0x58]
//    140d3caeb  cmp      qword ptr [rcx + 0x20], rdx
//    140d3caef  jne      0x140d3ce98
//    140d3caf5  mov      r8, qword ptr [rbp + 0x60]
//    140d3caf9  lea      rdx, [rdx + r8*8]
//    140d3cafd  mov      qword ptr [rcx + 0x20], rdx
//    140d3cb01  jmp      0x140d3ce98
//    140d3cb06  mov      rax, qword ptr [rbp + 0x20]
//    140d3cb0a  cmp      byte ptr [rax + 0x6b9], 0
//    140d3cb11  mov      rsi, qword ptr [rbp + 0x48]
//    140d3cb15  mov      rdi, qword ptr [rbp + 0x50]
//    140d3cb19  je       0x140d3cbd2
//    140d3cb1f  mov      r8d, 1
//    140d3cb25  sub      r8, rsi
//    140d3cb28  mov      rax, qword ptr [rbp + 0x18]
//    140d3cb2c  mov      r9, qword ptr [rax]
//    140d3cb2f  lea      rcx, [rbp + 0x28]
//    140d3cb33  mov      rdx, rdi
//    140d3cb36  call     0x141821530
//    140d3cb3b  mov      rax, qword ptr [rbp + 0x28]
//    140d3cb3f  mov      rcx, qword ptr [rbp + 0x40]
//    140d3cb43  test     rcx, rcx
//    140d3cb46  je       0x140d3cbb3
//    140d3cb48  shl      rcx, 3
//    140d3cb4c  mov      rdx, qword ptr [rbp + 0x20]
//    140d3cb50  mov      rdx, qword ptr [rdx + 0x5c0]
//    140d3cb57  xor      r8d, r8d
//    140d3cb5a  jmp      0x140d3cb69
//    140d3cb5c  nop      dword ptr [rax]
//    140d3cb60  add      r8, 8
//    140d3cb64  cmp      rcx, r8
//    140d3cb67  je       0x140d3cbb3
//    140d3cb69  mov      r9, qword ptr [rax + r8]
//    140d3cb6d  cmp      dword ptr [r9 + 0x68], 1
//    140d3cb72  jne      0x140d3cb60
//    140d3cb74  cmp      dword ptr [r9 + 0x88], 1
//    140d3cb7c  jne      0x140d3cb60
//    140d3cb7e  cmp      qword ptr [r9 + 0x90], rdx
//    140d3cb85  jne      0x140d3cb60
//    140d3cb87  mov      rcx, qword ptr [rbp + 0x38]
//    140d3cb8b  test     rcx, rcx
//    140d3cb8e  je       0x140d3ce92
//    140d3cb94  mov      rdx, qword ptr [rbp + 0x30]
//    140d3cb98  mov      rdx, qword ptr [rdx + 0x10]
//    140d3cb9c  cmp      qword ptr [rdx + 0x20], rax
//    140d3cba0  jne      0x140d3ce92
//    140d3cba6  lea      rax, [rax + rcx*8]
//    140d3cbaa  mov      qword ptr [rdx + 0x20], rax
//    140d3cbae  jmp      0x140d3ce92
//    140d3cbb3  mov      rcx, qword ptr [rbp + 0x38]
//    140d3cbb7  test     rcx, rcx
//    140d3cbba  je       0x140d3cbd2
//    140d3cbbc  mov      rdx, qword ptr [rbp + 0x30]
//    140d3cbc0  mov      rdx, qword ptr [rdx + 0x10]
//    140d3cbc4  cmp      qword ptr [rdx + 0x20], rax
//    140d3cbc8  jne      0x140d3cbd2
//    140d3cbca  lea      rax, [rax + rcx*8]
//    140d3cbce  mov      qword ptr [rdx + 0x20], rax
//    140d3cbd2  mov      rax, rsi
//    140d3cbd5  shl      rax, 5
//    140d3cbd9  cmp      qword ptr [rdi + rax + 0x148], 0
//    140d3cbe2  je       0x140d3ce96
//    140d3cbe8  add      rax, rdi
//    140d3cbeb  add      rax, 0x130
//    140d3cbf1  mov      rax, qword ptr [rax]
//    140d3cbf4  mov      rax, qword ptr [rax]
//    140d3cbf7  mov      r8, qword ptr [rax + 0x660]
//    140d3cbfe  mov      rax, qword ptr [rax + 0x668]
//    140d3cc05  mov      rdx, qword ptr [rbp + 0x20]
//    140d3cc09  mov      rcx, qword ptr [rdx + 0x660]
//    140d3cc10  mov      rdx, qword ptr [rdx + 0x668]
//    140d3cc17  mov      r10, r8
//    140d3cc1a  sub      r10, rcx
//    140d3cc1d  mov      r9, rcx
//    140d3cc20  sub      r9, r8
//    140d3cc23  cmovb    r9, r10
//    140d3cc27  mov      r10, rax
//    140d3cc2a  sub      r10, rdx
//    140d3cc2d  mov      r8, rdx
//    140d3cc30  sub      r8, rax
//    140d3cc33  cmovb    r8, r10
//    140d3cc37  mov      rax, qword ptr [rbp + 0x18]
//    140d3cc3b  movzx    r10d, byte ptr [rax + 0x38]
//    140d3cc40  xor      eax, eax
//    140d3cc42  cmp      r10d, 8
//    140d3cc46  ja       0x140d3ce98
//    140d3cc4c  imul     r9, r9
//    140d3cc50  imul     r8, r8
//    140d3cc54  add      r8, r9
//    140d3cc57  mov      r9d, 1
//    140d3cc5d  sub      r9d, esi
//    140d3cc60  shl      r9d, 5
//    140d3cc64  mov      r11d, 0x185
//    140d3cc6a  bt       r11d, r10d
//    140d3cc6e  jb       0x140d3cc95
//    140d3cc70  mov      r11d, 0xa
//    140d3cc76  bt       r11d, r10d
//    140d3cc7a  jb       0x140d3ce0f
//    140d3cc80  mov      r11d, 0x30
//    140d3cc86  bt       r11d, r10d
//    140d3cc8a  jb       0x140d3cd4a
//    140d3cc90  jmp      0x140d3ce98
//    140d3cc95  mov      rax, qword ptr [rdi + rsi*8 + 0x180]
//    140d3cc9d  or       rax, qword ptr [rdi + rsi*8 + 0x190]
//    140d3cca5  jne      0x140d3cd2e
//    140d3ccab  lea      r11, [rdi + r9]
//    140d3ccaf  add      r11, 0x10
//    140d3ccb3  mov      rax, qword ptr [r11 + 0x18]
//    140d3ccb7  test     rax, rax
//    140d3ccba  je       0x140d3cd2e
//    140d3ccbc  mov      r11, qword ptr [r11]
//    140d3ccbf  xor      esi, esi
//    140d3ccc1  xor      edi, edi
// */
// ```
//
// ---
//
