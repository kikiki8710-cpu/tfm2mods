#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""gensweep20.py — **2단계 sweep 코드 생성기**. 명세 20함수(`MIG\\_spec\\specs20_v3.json`)를
게임 원본 vs **내 dll 안 링크사본**(`extern crate game_ai;`)으로 **비트동일 대조**하는
`tfm2_judge_verify\\src\\sweep20.rs` 를 만든다. (2026-09-12 · 게임 0.5.8)

## 왜 `MIG\\gensweep.py` 를 고치지 않고 파생시켰나 (유저 질문 1 의 답)
`gensweep.py` 는 **`tfm2_ai_adjust` 전용**이다 — 입력이 `ai_adjust\\src\\judge\\gen_fns.rs`(그 모드가
관리하는 FnSpec 표)이고 출력이 `ai_adjust\\src\\judge\\sweep.rs` 로 **하드코딩**돼 있으며,
`aimap.json`·DISubprogram 구간 역추적 같은 그 모드 고유의 심볼 확정 경로를 품고 있다.
   ① 이번 세션은 `tfm2_ai_adjust\\` **수정 금지**다. 출력처를 인자화하려고 그 스크립트를 고치면
      ai_adjust 의 생성물 재현성(같은 스크립트 → 같은 sweep.rs)을 건드리게 된다.
   ② 입력이 근본적으로 다르다. 우리 입력은 `specs20_v3.json`(명세 20함수) + **1단계 실측 발화수**이고,
      제외 판정에 `gensweep.py` 에 없는 게이트가 셋 더 필요하다 —
      **㉠rlib 심볼 노출(llvm-nm `T`/`t`) ㉡fastcc ㉢가변 포인터 인자**.
   ③ 산출물 형태도 다르다(발화수·제외사유 표를 코드에 싣고, 진입부 프로브와의 **배타 설치**를 위해
      `is_installed_spec()` 를 노출한다).
⟹ **파생**(gensweep20.py). `gensweep.py` 에서 물려받은 것 = ①진입부 12B 스틸 + capstone 명령경계
   ②`extern "Rust"` + `#[link_name]` 로 링크사본 직접 호출 ③**320B(StdRng) 인자 떠서 되돌리기**
   ④비트마스크 게이트 ⑤대조 루프 모양(재진입 깊이 카운터 · catch_unwind).

## 제외 게이트 (「빠진 것을 모르는 상태」를 만들지 않는다 — 전부 생성물의 EXCLUDED 표에 남는다)
  · 발화 0        = 검증 표본 불성립(1단계 확정치)
  · 심볼 `t`      = rlib 에서 **internal**(비노출) ⟹ `#[link_name]` 링크 불가 (llvm-nm 실측)
  · `fastcc`      = 호출규약 비호환(게임 호출부가 fastcc 로 넘긴다)
  · `sret`        = 반환이 숨은 포인터 — 전용 래퍼
  · 반환 `void`   = **비교할 값이 없다**
  · 반환/인자 타입 미지원(페어반환 `{i64,i64}` · `i24` · 인자 `i1`)
  · ★**가변 포인터 인자**(readonly/readnone 둘 다 없는 ptr) = 두 번 호출하면 **상태가 두 번 변한다.**
    스냅샷 후 복원도 못 한다(구조체 안 Vec/Box 가 재할당되면 복원본의 포인터가 dangling).
    예외 ㉠ = `dereferenceable(320)` = `StdRng` ⟹ 320B 를 떠서 되돌린다(난수열 갈림 방지).
    예외 ㉡ = **명세 `sig.tcx` 의 해당 파라미터가 `&mut` 가 아닌 공유참조**면 가변이 아니다.
       IR 이 `readonly` 를 못 붙이는 경우가 있다(그 포인터를 불투명 호출로 넘기기만 하면 속성 추론이
       포기된다) — 그걸 가변으로 읽으면 **순수 술어까지 제외되는 오탐**이 난다(실제로 `#08 is_end` 가
       그랬다: tcx `&TeamPlan` · 명세 근거 「IR %5 에 store 0건」 · specgate G5 가 `&mut` 오기를 정정).
       ⟹ tcx 가 공유참조라고 말하면 편입하되 **caveat 로 표시**한다(리포트·헤더에 남는다).
       tcx 파라미터를 못 찾으면 **보수적으로 제외**한다(모르면 안 건드린다).
  · 진입부에서 명령 경계 12B 확보 실패(capstone) = 진입부 훅 불가

사용: `python MIG\\gensweep20.py`  (출력 = tfm2_judge_verify\\src\\sweep20.rs)
"""
import io
import json
import os
import re
import struct
import subprocess
import sys

import capstone
import sys as _sys
_sys.path.insert(0, r"C:\tfm2mods\MIG")

sys.stdout.reconfigure(encoding="utf-8", errors="replace")

EXE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
BASE = 0x140000000
SPEC = r"C:\tfm2mods\MIG\_spec\specs20_v3.json"
IRDIR = r"C:\tfm2mods\_gaibc"
TBL = r"C:\tfm2mods\tfm2_judge_verify\src\probe20_tbl.rs"
OUT = r"C:\tfm2mods\tfm2_judge_verify\src\sweep20.rs"
NM = os.path.join(os.environ["USERPROFILE"],
                  r".rustup\toolchains\nightly-2026-05-24-x86_64-pc-windows-msvc"
                  r"\lib\rustlib\x86_64-pc-windows-msvc\bin\llvm-nm.exe")
RLIB_DIRS = [r"C:\tfm2mods\sdk_058\deps_ailink", r"C:\tfm2mods\sdk_058\mod-sdk\deps"]

# ★1단계 실측 = 「판 종료 #1 확정치」(인게임 실측 2026-09-12 · 설치 19/19 · 실패 0).
#   정본 = <게임>\mods\tfm2_judge_verify\probe20.txt. 여기 값은 **주석·정렬용 사본**이다.
#   ★★2026-09-12 **재측정**(정정된 주소로 다시 잰 값). 판 종료 #1 · 프레임 77,181 · 경과 1,239.9s ·
#     설치 19/19 · 실패 0 · 크래시/패닉 0. 원문 = `REPORT\...\RE\2026-09-12_probe20_재측정_판종료1_정정주소.txt`
#   ⚠**옛 값과 절대비교하지 마라** — 그 판은 393.3s 였다. 비교는 `#09`(양쪽 다 주소가 맞았다) 기준
#     정규화로 한다. 그 대조에서 **주소를 안 바꾼 12개는 0.6~1.2배**(= 판이 달라도 비율 유지)인데
#     **주소를 바꾼 6개는 전부 그 밴드 밖**이었다(#12 4,480배 · #04 12.1배 · #11 1/343 · #16 1/24.6 ·
#     #10 1.78→0 · #15 0→0) ⟹ 옛 수치가 **다른 함수**를 재고 있었다는 독립 확인이다.
FIRED = {4: 54660390, 16: 6940762, 19: 2806126, 12: 1808301, 18: 1672540,
         8: 1554463, 1: 676759, 9: 428286, 14: 148803, 13: 132642,
         6: 107627, 0: 101915, 3: 29128, 11: 10041, 5: 3403,
         # ★r7 잎 20(i=20~39) · 2026-09-13 판 3(슬롯 96 · 설치 39/39 · Gen.G vs 디플러스 SET1 · 372s 스냅샷 —
         #   「판 종료」 감지가 메뉴 배경 presim 때문에 안 찍혀 리플레이 종료 후 스냅샷을 확정치로 씀 · 원문 =
         #   <게임>\mods	fm2_judge_verify\_r7_probe1\probe20_run3_all38.txt). #30 objective_is_damaged 는 2회 = 표본 극소.
         22: 173781504, 39: 33717543, 38: 18769014, 21: 11686232, 23: 10295069, 25: 5660210,
         20: 3158080, 31: 3144357, 29: 3055211, 34: 2818679, 24: 1370692, 26: 1149517, 27: 1081789,
         28: 866135, 35: 656947, 36: 560347, 37: 556740, 33: 506982, 32: 2110, 30: 2,
         # ★r8 잎 17(i=40~56) · 2026-09-13 13:0x 판 1(설치 56/56 · Gen.G vs 디플러스 SET1 · 221.9s 스냅샷 · 원문 =
         #   <게임>\mods	fm2_judge_verify\_r8_probe1\probe20_r8_run1.txt). 미발화 2 = #42(i42 SingleLane 전용=NA) · #47(i47 이 판 0).
         54: 25238388, 55: 18749665, 46: 10990018, 53: 8012325, 45: 7564211, 51: 6807462, 56: 6570780,
         48: 2777647, 52: 2105201, 41: 1785206, 50: 1223027, 44: 966291, 43: 709478, 40: 248491, 49: 68945}
#   `#02` 는 MISSING20(인라인)이라 여기 없다. 그 호스트 `BigPlan::sub_plan`(AUX[90] @0xcaf9f0 · ~~AUX[20]~~ 09-13 이동)의
#   재측정치 = **27,416,789**(probe20.txt 참조) — 명세 함수가 아니므로 이 표에 넣지 않는다.
# ★★**이 함수들의 1단계 발화수는 무효다** — 그때 잰 주소가 **다른 함수**였다(2026-09-12 ghidra 확정).
#   ⛔무효 수치를 숫자로 남겨두면 다음 세션이 그대로 인용한다. 그래서 **숫자 자리에 사유를 찍는다.**
#   (`#16` 의 「1억 578만」은 `std::thread::LocalKey::with` 의 호출수였다.)
#   ⟹ 정정된 주소로 **1단계를 다시 돌려야** 이 칸이 채워진다.
#   값 = 숫자 자리에 찍을 **사유 문면**(주소만 넣으면 `#02` 처럼 사유가 다른 건을 못 적는다).
#   ★재측정(2026-09-12)으로 **6건 전부 해소**됐다 — 이제 무효 표식이 필요한 명세 함수는 없다.
#   ⚠`#02` 만 남는데, 그건 「틀린 주소」가 아니라 **호스트(디스패처)를 가리켰던 것**이고
#     지금은 `MISSING20`(인라인)이라 애초에 이 표를 타지 않는다. 2,048만/2,741만은
#     `AUX[90] BigPlan::sub_plan` 의 값으로 프로브 표에 보존돼 있다.
#   ★표식을 지울 때의 규칙: **재측정이 끝난 것만 지운다.** 「고쳤으니 괜찮겠지」로 지우면
#     무효 수치가 조용히 되살아난다(이 표가 존재하는 이유).
INVALID_FIRE = {}
DEAD = {7: u"미발화(재측정 확정치 0회)",
        42: u"미발화(SinglePlanBattle = SingleLane 전용 · MOBA NA · 09-13 r8 판1 0회)", 47: u"미발화(handle_epic_line_change · 09-13 r8 판1 0회 — 이 리플레이 한정)",
        # ★재측정 전에는 `#10`·`#15` 의 0 을 믿을 수 없었다(틀린 주소에서 잰 0 이었다).
        #   지금은 **정정된 주소에서 잰 0** 이라 「이 판에서 죽은 코드」가 **유효한 판정**이다.
        10: u"미발화(재측정 확정치 0회 — 정정된 주소 0xe0c560 에서)",
        15: u"미발화(재측정 확정치 0회 — 정정된 주소 0xe5c1f0 에서)"}
# 1단계에서 **호출부 프로브**로 센 함수(진입부 12B 스틸 불가) — sweep 으로 바꾸면 그 프로브를 잃는다.
CALLSITE_PROBE = {13: u"진입부 12B 스틸 불가(je@+9) — 1단계는 호출부 리다이렉트로 셌다"}

RMAP = {"ptr": "*const u8", "i64": "i64", "i32": "i32", "i8": "u8",
        # ★`i1` = IR 의 bool. `zeroext` 계약상 0/1 만 오므로 `u8` 로 받는다
        #   (`bool` 로 받으면 상위 비트 쓰레기가 UB — 반환 처리와 같은 이유).
        #   2026-09-12: 이게 없어서 `is_object_being_taken_by_enemy`(전 인자 readonly =
        #   부작용 0 인 순수 술어)가 「인자 i1 미지원」으로 잘려 있었다.
        "i1": "u8",
        # ★i24(09-13 r8 #53 v25_scoped_battle_objective): `Option<MainObjective>` 류 3B 스칼라. x64 에서 edx 등 32비트 레지스터로
        #   오가고 **상위 8비트는 정의되지 않는다**(define 에 zeroext 없음) ⟹ u32 로 받고 비교는 `& 0xffffff`.
        "i24": "u32"}

# ★★가변 포인터인데 **대조 판정에 영향이 없다고 보는** tcx 타입. (2026-09-12 신설)
#   기본 규칙은 「가변 포인터 = 두 번 호출하면 상태가 두 번 변한다 = 제외」다. 그런데 그 규칙이
#   **디버그 싱크까지 함께 자른다** — `&mut DebugFrameData` 는 판정 대상(반환값)에 안 들어간다.
#   근거(IR 실측 `#19 best_jungle_goal` m04.ll:62570~62978):
#     · `%6`(DebugFrameData)는 **본문에서 한 번도 역참조되지 않는다** — 클로저 캡처 구조체
#       `%10+40` 에 **포인터를 넣어 넘기기만** 한다(`store ptr %6, ptr %22`).
#     ⟹ **이 함수의 제어흐름·반환값은 DebugFrameData 의 기존 내용에 의존하지 않는다.**
#   ⚠**한계(정직하게)**: 클로저 안에서 쓰는지까지는 IR 밖이라 못 봤다. 쓴다면 **중복 기록**이 남는다
#     (게임 진행에는 안 쓰이지만 버퍼가 2배로 찬다). ⟹ **런타임이 최종 판정**이다:
#     DIFF=0 이 대량 표본에서 나오면 「반환값이 그 버퍼에 의존하지 않는다」가 실증된다.
#   ⛔여기에 **게임 상태 구조체를 넣지 마라**(`LegacyPlanHandler`·`TeamPlan`·`LineGankerPlan` 등).
#     그건 두 번 쓰면 실제로 판이 달라진다.
#   ⚠키는 tcx 문면에 **실제로 있는 형태**로 적어라 — 명세의 `params[].type` 은
#     `&mut DebugFrameData(224B)` 처럼 **크레이트 접두가 없고 크기 주석이 붙는다**.
#     `game_core::DebugFrameData` 로 적었다가 **매치가 안 돼 조용히 제외 유지**됐다(2026-09-12).
# ★sret 반환 함수의 **살아 있는 바이트 범위** `{spec_idx: [(offset, len), …]}`.
#   비어 있으면 그 함수는 제외된다(아래 판정부의 실증 주석 참조).
#   채우는 법 = `tcxdict` 의 그 타입 레이아웃에서 **실제로 읽히는 필드**만 고른다.
#   형식 = `{idx: [(off, len, [조건…]), …]}` · 조건 = `(tag_off, tag_len, [허용 tag…])`
#   한 구간은 **조건이 전부 성립할 때만** 비교 대상이다(조건 없으면 무조건).
#   판별자는 **게임 버퍼**에서 읽는다 — 태그 자체도 구간으로 들어 있으니 태그가 갈리면 어차피 잡힌다.
#
#   ★`#00 ult` 의 근거 = `tcxdict` 실측 레이아웃 2개:
#     `game_core::Input`(32B enum · 판별자 `+0x0` **8B** Direct)
#        0 Move  : x `+0x8`(8B) · y `+0x10`(8B)
#        1 Return: 없음
#        2~5 Attack/Skill/Skill2/Ult: `target: InputTarget`(24B) `@+0x8`
#     `game_core::InputTarget`(24B enum · 판별자 `+0x0` **4B** Direct)
#        0 Target: target_id `+0x8`(8B)   → Input 기준 `+0x10`
#        1 Dir   : dir_x `+0x8` · dir_y `+0x10` → Input 기준 `+0x10`·`+0x18`
#        2 Pos   : x `+0x8` · y `+0x10`        → 동일
#        3 None  : 없음
#     `Option<Input>` 의 None 은 니치(태그 6)로 본다 ⟹ 태그만 비교하면 된다.
#   ⟹ 실측 DIFF 의 정체: `InputTarget` 판별자가 **4B** 인데 8B 로 비교해 **상위 4B 패딩**이 걸렸고,
#      `+0x18` 은 `Target` variant 에서 **안 쓰는 자리**였다. 살아있는 필드는 **전부 일치**했다.
# ★★**레지스터 페어 반환(P8/P64)도 판별자에 따라 두 번째 칸이 죽는다.** (2026-09-12 실측)
#   형식 = `{idx: [두 번째 칸(b)이 **살아 있는** 태그…]}` · 첫 칸(a=태그)은 항상 비교.
#   근거 = `#06 v2_response_retreat_stance` → `game_ai::plan_legacy::old::BattleSubPlanGoal`
#          (16B enum · 판별자 `+0x0` 8B): 0 Trace·1 Protect·2 Kiting·3 KitingBack·5 Assassin·
#          6 AssassinReady 는 `focus: usize` `@+0x8` 를 갖고, **4 RunAway·7 End 는 페이로드가 없다.**
#   실측: DIFF 3,771/26,210 의 첫 덤프가 `a=4`(RunAway)인데 `b` 만 갈렸다
#         (`0x1173ff3960` vs `0x16f2c8b3c88` = 스택/힙 주소 = **레지스터 잔재**).
#   ⟹ sret 과 **같은 부류**다 — 「반환이 페어」면 비교 대상은 「두 칸 전부」가 아니다.
# ★★**`&mut` 게임 상태를 「쓰지 않고」 대조하는 전략.** (2026-09-12 신설 · `#18` 전용)
#   형식 = `{idx: (인자번호, 크기, [(cap_off, ptr_off, len_off)], 요소크기)}`
#
#   기존 `StdRng`(320B) 처리와 같은 골격이다 — **게임 호출 전 상태를 띄어 두고**, 내 사본을
#   부르기 전에 그 상태로 되돌리고, 부른 뒤 **게임 호출 후 상태로 복구**한다.
#   ⟹ 내 사본은 **게임과 똑같은 입력**을 보고, 게임 상태엔 최종적으로 게임 값만 남는다.
#
#   ★그냥 되돌리기만 하면 **위험하다**: 내 사본이 `Vec` 에 push 하면 재할당이 일어나
#     **게임의 버퍼가 해제**되고, 옛 3워드를 복원하면 **이미 해제된 포인터**가 된다.
#   ⟹ 내 사본을 부르기 직전에 그 `Vec` 을 **빈 것(cap=0·ptr=0·len=0)** 으로 만든다.
#     `cap == 0` 인 `RawVec` 은 push 시 **realloc 이 아니라 alloc** 을 하므로 **게임 버퍼를 절대 안 건드린다.**
#     호출 뒤 내 사본이 할당한 것을 **해제**하고(누수 방지) 게임 상태로 복구한다.
#
#   ⚠**필드 순서를 가정하지 마라** — `#18` 의 `Vec<Chat>` 은 **(cap@0xc0, ptr@0xc8, len@0xd0)** 으로
#     통념(`ptr,cap,len`)과 **반대**다. IR 실측: `+0xc0` 은 `len` 과 `icmp eq`(= 자리 검사),
#     `+0xc8` 은 **24B 요소 gep 의 base**(인덱스가 `len`). 가정했으면 cap 자리에 포인터를 써서 즉사했다.
#   ⚠**동시성**: 되돌린 동안(아주 짧게) 다른 스레드가 그 구조체를 보면 옛 값을 읽는다.
#     `StdRng` 처리와 같은 노출이고 4,800만 표본에서 문제가 없었지만 **0 은 아니다** — caveat 로 남긴다.
# ★★진단 스위치(2026-09-12) — 1 이면 SELF_RESTORE 를 **끕다**(되돌리기·Vec 비우기 없이 호출).
#   크래시 원인이 「내 되돌리기」인지 「내 사본 자체」인지를 가르는 런타임 이분법용이다.
#   ⚠켜 둔 동안은 내 사본이 게임의 Vec 에 push 할 수 있다(메모리는 안전하나 판이 교란된다)
#   ⟹ **진단 전용**. 운영 빌드는 반드시 0.
# ★★★**exe 의 실제 ABI 가 IR `define` 과 다른 함수.** (2026-09-12 신설)
#   형식 = `{idx: [IR 인자별 ← 래퍼 인자 번호 또는 "RNG"]}`
#   래퍼는 **exe 순서로** 인자를 받고(게임 호출부가 그렇게 놓으므로),
#   내 사본은 **IR 순서로** 불러야 한다. 그 사이 변환표다.
#
#   ★`#18` 의 근거 = 명세 `exe.evidence` 에 **오늘 아침 내가 직접 적어둔 것**:
#     「ABI 불일치: IR 7-arg -> exe 6-arg (rnd 가 dead-arg elimination 으로 제거) ·
#      실측 배치 RCX=self RDX=version R8=player R9=data [rsp+0x28]=goal_data [rsp+0x30]=plan」
#   ★★그런데 생성기가 그걸 **무시하고 IR 7개로 래퍼를 만들어** 인자가 한 칸씩 밀렸고,
#     `data` 자리에 `goal_data` 가 들어가 거기서 뽑은 `cache` 가 쓰레기 → **0xc0000005**.
#     (크래시 지점 `[r15 + team*0x28 + 0x1e0]` 과 정확히 일치했다.)
#   ⟹ ★★**명세에 적힌 ABI 사실을 기계가 읽게 하지 않으면, 적어둔 것은 없는 것과 같다.**
#
#   ⚠`"RNG"` = exe 가 버린 `&mut StdRng` 자리. 내 사본은 IR 서명이라 그 칸이 필요하므로
#     **thread_local 320B 버퍼**를 준다. 게임은 그 인자를 안 쓰므로(그래서 제거됐다)
#     내 사본도 안 써야 정상이다 — 쓴다면 DIFF 로 드러난다(= 그 자체가 새 사실).
# ★명세 밖 sweep 대상(이분용). `idx` 는 목록 순서대로 20, 21… 이 붙는다.
#   `v3_epic_group_line` = `#18` 의 `false` 조건 **둘 다**가 의존하는 값을 만드는 함수
#   (`%41 = v3_epic_group_line(%40, ...)` · `%42 = (%41 == -1)` · `%65 = (self[0x41e] == %41)`).
#   IR `i8 (i64, &PlayerState, &OperationData)` — **인자 셋 다 readonly = 순수 함수**라
#   상태 처리가 전혀 필요 없다. rlib 심볼이 **이미 `T`** 라 패치도 불요.
#   exe `0xdea4a0`(180B) = `#18` 의 피호출 중 유일한 미매칭이고 epic 모듈 주소대에 있다(정황 확정).
EXTRA_SWEEP = [
    {"name": "v3_epic_group_line",
     "sym": "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic18v3_epic_group_line",
     "addr": "dea4a0", "bytes": 180, "module": "epic",
     "src": "game-ai\\src\\plan_legacy\\old\\epic.rs",
     "ir_file": "m09.ll", "ir_frm": 64337,
     "tcx": "fn(usize, &game_core::PlayerState, &game_core::OperationData) -> i8",
     "evidence": "bisect(#18 DIFF 원인 좁히기) · rlib T · exe 0xdea4a0 = #18 피호출 중 유일 미매칭"},

    # ★★2026-09-12 2차 이분 — #18 CFG 실측으로 후보가 **셋으로 확정**됐다.
    #   `_gaibc/m09.ll:6879~7264` 을 블록 단위로 읽으면 **%35 이후 모든 갈래가 false** 다
    #   (%73 %74, 그리고 %146→%74). ⟹ `strategy`·`group_line`·`formation_role` 은 **false 갈래 안**이라
    #   게임 true(88%) ↔ 내 false 의 갈림이 될 수 없다. `true` 는 세 곳에서만 나온다:
    #     ① `switch i8 %12` — `v3_epicops_repair_need` 가 1 또는 2
    #     ② `v3_serpen_contest_clear_win` == true (→%43→%56)
    #   그리고 그 둘에 닿는 게이트가 ③ `is_object_being_taken_by_enemy` 다.
    #   ⟹ 이 셋을 한 번에 대조하면 갈림이 「피호출 재현」인지 「내 인자 배치」인지 갈린다.
    #   주소 출처 = `dllmatch.json` **jaccard 1.0 / contain 1.0**(세 건 모두 정확일치).
    {"name": "v3_epicops_repair_need",
     "sym": "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic22v3_epicops_repair_need",
     "addr": "deaa70", "bytes": 1230, "module": "epic",
     "src": "game-ai\\src\\plan_legacy\\old\\epic.rs",
     "ir_file": "m09.ll", "ir_frm": 64975,
     "tcx": "fn(usize, *const AbstractGameWithCache /*널 허용*/, &Plan384) -> u8 /*range 0..3*/",
     "evidence": "bisect2(#18 true 갈래 ①) · IR `switch i8 %12` 의 입력 · 인자 전부 readonly = 부작용 0 · "
                 "rlib internal fastcc → `patches.json` 으로 노출 · dllmatch jac 1.0"},

    {"name": "is_object_being_taken_by_enemy",
     "sym": "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers30is_object_being_taken_by_enemy",
     "addr": "ec9bf0", "bytes": 364, "module": "objective_helpers",
     "src": "game-ai\\src\\plan_legacy\\team_plan\\objective_helpers.rs",
     "ir_file": "m15.ll", "ir_frm": 53833,
     "tcx": "fn(&PlayerState, &OperationData, &GoalData248, &TeamPlan, bool) -> bool",
     "evidence": "bisect2(#18 게이트 ③) · rlib `hidden`+ccc = 패치 불요 · **인자 전부 readonly**(a3 TeamPlan 까지) "
                 "= 되돌리기 불요 · dllmatch jac 1.0"},

    {"name": "v3_serpen_contest_clear_win",
     "sym": "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6serpen27v3_serpen_contest_clear_win",
     "addr": "d666c0", "bytes": 641, "module": "serpen",
     "src": "game-ai\\src\\plan_legacy\\old\\serpen.rs",
     "ir_file": "m05.ll", "ir_frm": 53103,
     "tcx": "fn(usize /*version*/, &PlayerState, &OperationData, &mut TeamPlan) -> bool",
     "evidence": "bisect2(#18 true 갈래 ②) · rlib external ccc = 패치 불요 · a3 = **가변** TeamPlan "
                 "⟹ `SELF_RESTORE[\"v3_serpen_contest_clear_win\"]` · dllmatch jac 1.0"},

    # ★#40 resolve_fight_stake DIFF 3/648,750(09-13 09:22) 규명 — `resolve_fight_full` = TLS `RefCell<ResolveFightCache>` 메모
    #   래퍼(m10.ll:39915 · 키 = allies/enemies id·version·champ·… + (seed,tick) · 히트면 캐시값). 게임 캐시는 **다른 13개 호출부**
    #   (tower_dive·siege_stance·join_stake·roster…)가 먼저 채우므로 링크사본(내 TLS)이 그 이력을 못 본다 ⟹ 순수 코어
    #   `resolve_fight_uncached`(exe 0xe083c0 8841B · fnprobe 09-13 · full 의 콜리 유일) 를 직접 sweep 해 로직을 증명한다.
    #   IR 15인자(sret 64 + 14) = exe 레지스터 4 + 스택 11(fnprobe 0x60~0xb0) 일치. %12 = &mut DebugFrameData 추정(MUT_OK_ARG).
    {"name": "resolve_fight_uncached",
     "sym": "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model22resolve_fight_uncached",
     "addr": "e083c0", "bytes": 8841, "module": "fight_model",
     "src": "game-ai\\src\\plan_legacy\\old\\fight_model.rs",
     "ir_file": "m10.ll", "ir_frm": 43974,
     "tcx": "fn(usize, &AbstractGameWithCache, &GameContext, &Entity, *const &Entity, usize, *const &Entity, usize, i8, Option<&Entity>, usize, &mut DebugFrameData, usize, usize) -> FightPrediction /*sret 64B*/",
     "evidence": "#40 DIFF 3 규명(09-13) · fnprobe 0xe05450 콜리 = 0xe083c0 유일 · rlib internal fastcc → patches.json 노출"},
]

EXE_ABI = {
    18: [0, 1, "RNG", 2, 3, 4, 5],
}

SELF_RESTORE_OFF = 0
BISECT_NO_STRFREE = 0   # ★임시 이분 스위치(2026-09-13) — 0 으로 되돌릴 것
BISECT_SKIP_MY = 0      # 이분 스위치(2026-09-13 원인 규명 완료 — 중첩 Vec 누락) — 0 유지

SELF_RESTORE = {
    18: (0, 1064, [(0xc0, 0xc8, 0xd0)], 24),   # a0 = &mut TeamPlan(1064B) · chats: Vec<Chat>(24B)
    54: (0, 1064, [(0xc0, 0xc8, 0xd0)], 24),   # #59 TeamPlan::update — 같은 self(09-13 r8)
    # ★#14 `LineGankerPlan::update` — a0 = &mut LineGankerPlan(**48B**).
    #   tcx 실측: `chats: Vec<Chat>` @0x0(24B) · `setup_limit`@0x18 · `wait_limit`@0x20 ·
    #            `line`@0x28 · `phase`@0x29 ⟹ **힙 소유 필드는 `chats` 하나뿐**이라
    #            바이트 스냅샷·되돌리기가 성립한다(6168B `LegacyPlanHandler` 와 달리).
    #   ★Vec 순서는 **IR 실측**이다(가정 아님): `_gaibc/m08.ll:94870~94880` 에서
    #     `load i64, ptr %0`(=cap) · `%0+8 → load ptr`(=ptr) · `%0+16 → load i64`(=len).
    14: (0, 48, [(0x0, 0x8, 0x10)], 24),
    # ★#05 `v50_fold_dive_episode` — a0 = &mut LegacyPlanHandler(**6168B**).
    #   6168B 를 통째로 뜨는 게 위험해 보이지만 **이 함수는 self 의 어떤 필드도 drop 하지 않는다**
    #   (IR 실측: 유일한 `drop_glue<String>` 대상이 **지역 `alloca`** 이고 예외 정리 funclet 안이다)
    #   ⟹ 옛 포인터를 되살려도 이중해제가 없다 ⟹ 바이트 스냅샷·되돌리기가 성립한다.
    #   ⚠`#11` 은 정반대다 — `self.plan` 을 `drop_glue<BigPlan>` 로 드롭한다(= 이중해제).
    #     ★**`drop_glue`/`drop_in_place` 의 대상이 지역인가 self 인가**가 이 판정의 전부다.
    #   self 쓰기 = `v50_dive_episodes: Vec<V50DiveEpisode>`(cap@0x888·ptr@0x890·len@0x898,
    #              요소 **104B** = `{i64*10, i32*2, i8*10, [6 x i8]}`) + `v50_dive_ep_live`(0x570).
    5: (0, 6168, [(0x888, 0x890, 0x898)], 104),
    # ★#12 `handle_chat` — a0 = &mut LegacyPlanHandler(6168B).
    #   `#05` 와 같은 이유로 바이트 스냅샷이 성립한다 — **정상 경로에서 self 를 드롭하지 않는다**
    #   (IR 실측: `drop_glue<String>`×4·`drop_glue<PendingTraceEvent>`×1 이 **전부 `cleanuppad`
    #    이후 = 패닉 정리 funclet 안**이다).
    #   self 쓰기 = `pending_trace_events: Vec<PendingTraceEvent>`(cap@0x858·ptr@0x860·len@0x868,
    #              요소 **184B**) + `team_plan+0x41f`(1B) + `plan`(0x5e8).
    # ⛔#12 `handle_chat` — **2026-09-13 실측 크래시로 내림.** 본체(`handle_chat`)는 얇은 래퍼고 실제 쓰기는
    #   `handle_chat_inner`(m13.ll 3,679줄)에 있다: self 오프셋 ~80개 · `drop_glue` 25건 · `grow_one` 19건 ·
    #   self 를 넘기는 피호출 17개. Vec 하나만 치환하면 내 사본이 **게임의 다른 Vec 을 realloc/free** 한다
    #   ⟹ 30~40초에 즉사(재현 2회). ★교훈 = **self 쓰기 표면은 피호출까지 포함**한다 — 본문 1차 gep 스캔은 하한선이다.
    #   ⟹ `#11` 과 같은 「힙 인식 스냅샷」(소유 필드 전면 치환) 대상. 아래 SELF_DIFF[12]/ARG_SNAP[12] 는
    #     그때를 위해 남긴다(SELF_RESTORE 가 없으면 생성기가 쓰지 않는다).
    # ★#12 — 2차(2026-09-13): 힙 표면 셋(plan·chats·pending_trace_events)을 HEAP_SUBST 로 전부 치환. static Vec 치환은 없다.
    12: (0, 6168, [], 8),
    # ★#11 `v3_fall_back_to_passive` — a0 = &mut LegacyPlanHandler(6168B). **Vec 치환은 없다**(직접 push 하는
    #   self Vec 이 없다). 대신 `PLAN_SUBST[11]` 이 `self.plan` 의 소유 Vec 을 variant 별로 치환한다.
    11: (0, 6168, [], 8),

    # ★명세 밖 이분 대상은 **이름 키**로 쓴다 — `EXTRA_SWEEP` 목록이 늘어나면
    #   숫자 idx 가 밀려서 엉뚱한 사본에 되돌리기가 붙는다(=`#21` idx 충돌사고와 동질).
    u"v3_serpen_contest_clear_win": (3, 1064, [(0xc0, 0xc8, 0xd0)], 24),   # a3 = &mut TeamPlan
}

# ★★★**반환값이 exe 에 실재하지 않는 슬롯** — `self` 부작용으로 판정한다. (2026-09-12 신설)
#   왜 = `#18 v3_epicops_buff_window` 는 DIFF 88.7% 였는데 **재현 실패가 아니었다.**
#     그 반환값은 **유일한 호출부에서 쓰이지 않는다** — exe(`0xdd1676` 의 `CALL` 직후가 `JMP` 에필로그)
#     에서도, IR(`_gaibc/m09.ll:14828` 의 `%2135` 사용처 **0건**)에서도.
#     ⟹ 컴파일러가 `phi i1` 을 구체화하지 않아 exe 는 공용 에필로그에 **정규화 안 된 `AL`**
#       (직전 연산의 잔재 — `0xff`·`2` 등)을 남기고 `RET` 한다.
#     ⟹ 그걸 `bool` 로 읽어 대조하면 **난수와 비교하는 것**이다. DIFF% = `AL != 0` 인 비율일 뿐.
#   ⟹ 이 슬롯의 진짜 출력은 **`self` 에 대한 부작용**이다. 게임 호출 후 상태(`SP`)와
#     내 사본 호출 후 상태(`SQ`)를 **바이트로 대조**한다.
#   ⚠**적용 범위**: 이 결함은 **DIFF>0 인 슬롯에만** 소급된다 — 반환이 죽은 함수라도 두 값이
#     수백만 건 내내 우연히 같을 수는 없으므로 **기존 `DIFF=0` 판정은 손상되지 않는다.**
#   ⚠검사법(각 5초): ⓐexe 에서 `CALL` 다음 명령이 결과를 만지는가(`TEST`/`CMP`/`MOV`)
#                    ⓑIR 에서 호출 결과 `%NN` 의 사용처를 grep. 둘 다 없으면 여기에 등록하라.
#   값 = {"skip": [(오프셋, 길이), …]}  — 비교에서 뺄 구간(= 설계상 당연히 다른 곳).
SELF_DIFF = {
    "v3_epicops_buff_window": {
        # `chats` 의 cap@0xc0 · ptr@0xc8 · len@0xd0 세 칸(24B). 버퍼가 서로 다르니 cap/ptr 은
        # 당연히 다르다. ★`len` 과 **내용**은 제외가 아니라 **따로 비교**한다(아래 생성 코드).
        "skip": [(0xc0, 24)],
        # ★★★요소(`game_core::Chat` = **24B 열거형 · 57 variant · 판별자 +0(1B)**)의
        #   **살아있는 바이트**. (오프셋, 길이, 이 tag 들에서만 live · 빈 리스트 = 항상 live)
        #   근거 = IR 실측(`_gaibc/m09.ll:6932~6935`):
        #     `%26 = gep { i8, [23 x i8] }, %25, %19` · `store i8 23, ptr %26` · `store i64 0, ptr %26+8`
        #   ⟹ push 는 **`+0`(tag)과 `+8..15`(payload u64)만** 쓰고 `+1..7`·`+16..23` 은 **건드리지 않는다.**
        #   2필드 variant(디스어셈 `0xdce4df/0xdce4e2` = `MOV byte[+0],BL` + `MOV byte[+1],BPL`)만 `+1` 을 쓴다.
        #   ⟹ 안 쓰는 칸은 **재사용된 버퍼의 잔재**다 — 게임 버퍼엔 옛 Chat 의 값이, 내 버퍼엔 0 이 남는다.
        #   실측(2026-09-12): 그 한 칸 때문에 `vec[0]+16: g=16 m=00` 이 떴다. **재현 차이가 아니다**
        #   (`#00` sret 패딩 · `#06` 죽은 페이로드 슬롯과 같은 부류 — 하루 다섯 번째).
        "elem_live": [
            (0, 1, []),                  # tag — 항상
            (1, 1, [0x15, 0x16]),        # 2필드 variant(Press·PressChange)만 쓰는 칸
            (8, 8, []),                  # payload u64 — 항상 (`store i64 0`)
        ],
    },
    # ★#14 `LineGankerPlan::update` — 반환이 **진짜 void** 다(죽은 반환이 아니라 애초에 값이 없다).
    #   ⟹ 그래도 판정은 같은 방식이다: **`self` 부작용**을 비교한다.
    #   IR 실측(`m08.ll:94876~94910`)상 이 함수가 self 에 쓰는 곳:
    #     · `chats` push 2곳 — `store i8 17, +0` + `store i8 0|2, +1`  ★**`+8` payload 를 안 쓴다**
    #     · `phase` — `store i8 8, self+41`
    #   ⚠`#18` 은 같은 `Chat` 인데 `+0`/`+8` 을 썼다 — **variant 마다 페이로드 위치가 다르다**
    #     ⟹ `elem_live` 는 **슬롯별**이어야 한다(이 표가 이름/idx 키인 이유).
    # ★#59(i54) `TeamPlan::update`(09-13 r8) — 반환 void · a0 = &mut TeamPlan(1064B · #18 과 같은 구조체 · chats Vec @0xc0).
    #   IR 실측(m09.ll:38805/38913/39266): push 3곳 = MorgardPrepare(33)·SerpenPrepare(24) = `+0 tag`·`+8 i64`·`+16 i64 0` /
    #   Mia(1) = `+0 tag`·`+4 i32 position`·`+8 i64 0`. exe ABI = 6인자 전부 유지(r9=player · [rsp+0x200]=data · fnprobe 09-13).
    54: {
        "skip": [(0xc0, 24)],
        "elem_live": [
            (0, 1, []),                  # tag — 항상
            (4, 4, [0x01]),              # Mia 의 position(i32)
            (8, 8, []),                  # payload u64 — 세 variant 모두
            # ~~(16, 8, [0x21, 0x18])~~ MorgardPrepare/SerpenPrepare 의 두 번째 u64 — IR 은 `store i64 0` 이지만 exe 는 **dead store 제거**
            #   (판 09-13 16:14 첫 갈림 `vec[0]+16: g=f8 m=00` = 재사용 버퍼 잔재 · #18 의 +16 과 같은 부류). 아무도 안 읽는 칸이라 비교 제외.
        ],
    },
    14: {
        "skip": [(0x0, 24)],             # chats 의 cap@0x0 · ptr@0x8 · len@0x10
        "elem_live": [
            (0, 1, []),                  # tag — 항상
            (1, 1, [0x11]),              # tag 17(=0x11) 이 쓰는 두 번째 칸
        ],
    },
    # ★#05 `v50_fold_dive_episode` — 반환 void ⟹ self 부작용으로 판정.
    #   요소 `V50DiveEpisode` = 104B `{i64*10, i32*2, i8*10, [6 x i8]}`.
    #   IR 실측(`m13.ll` push 블록)상 **`+0`~`+97` 전 필드를 쓴다**(i64 10개 → +0..79,
    #   i32 2개 → +80..87, i8 10개 → +88..97). **꼬리 `[6 x i8]`(+98..103)은 패딩**이라 아무도 안 쓴다
    #   ⟹ 그 6바이트는 재사용 버퍼의 잔재다(`#18` 의 `+16` 과 같은 부류).
    5: {
        "skip": [(0x888, 24)],           # v50_dive_episodes 의 cap/ptr/len
        "elem_live": [(0, 98, [])],      # 패딩 6B 제외한 전 필드
    },
    # ★#11 — 정적 skip 은 없다. `self.plan` 의 소유 Vec 삼중항(cap/ptr)은 **variant 별 동적 skip**(PLAN_SUBST)이고
    #   그 Vec 의 len·내용은 따로 비교한다(요소가 POD 라 바이트 비교).
    # ★1차 실측(2026-09-13): 페이로드 포함 비교 = DIFF 100%(1,146/1,146), 첫 갈림 self+0x610 = BigPlan+0x28
    #   = PassiveJunglePlan.counter_jungle_route(Option 48B) 페이로드 = **None 이면 미초기화**. `#11` 은 새 plan 을
    #   **스택 alloca 에 만들어 memcpy** 하므로 스택 잔재가 들어온다(게임 0x00 vs 내 0xf0) ⟹ 표현 차이.
    #   ⟹ 2차: 페이로드(BigPlan+0x8..+0x180)를 제외하고 **태그 + 소유 Vec 의 len·내용**만 본다.
    #   ~~⚠적용 범위: plan 페이로드의 스칼라 필드는 **미비교**~~ → 2026-09-13 `ENUM_LIVE`(structlive 타입 기반 live 맵)로 편입.
    #   skip 은 그대로 두되(본체 루프에서 뺌) ①′ 에서 variant 조건부로 정밀 비교한다 ⟹ **범위한정 해제** = 6,567 DIFF 0.
    #   3차: 2차에서 DIFF 93.8%(1,132/1,207), 첫 갈림 self+0x778 = sub_plan(0x768, SubPlan 72B **니치 열거형 17 variant**)+0x10.
    #   `SubPlan::merge(self+0x768, …)` 가 in-place 갱신 = 같은 스택 잔재 부류. tcx 전수상 **소유 필드 없음**(POD) ⟹
    #   힙 위험은 없고 페이로드만 제외(태그 8B 는 비교).
    11: {"skip": [(0x5e8 + 0x8, 384 - 8), (0x768 + 0x8, 72 - 8)]},
    # ★#12 `handle_chat` — 요소 `PendingTraceEvent`(184B) = `event: TraceEventType`(176B) + `tick: usize`@0xb0.
    #   `TraceEventType` 은 **176B 니치 열거형 16 variant**(판별자 enum+0x0 8B ·
    #   niche_start = 0x8000000000000000) ⟹ **태그의 하위 1바이트 = variant idx** 다(0..15).
    #   이 함수가 push 하는 것은 `store i64 -9223372036854775793` = `0x800000000000000F`
    #   = **idx 15 `CallHandled`(7필드)** 하나뿐이다.
    #   ★★그 variant 는 **`String` 을 5개** 품는다 ⟹ 힙 포인터라 **바이트 비교가 성립하지 않는다**
    #     (게임은 exe 쪽 할당, 내 사본은 내 DLL 쪽 할당). ⟹ `elem_str` 로 **len + 내용**을 비교한다.
    12: {
        # plan 페이로드는 본체 루프에서 빼고 `ENUM_LIVE[12]`(structlive) 가 variant 조건부로 정밀 비교(2026-09-13 범위한정 해제 =
        # 1,577,476 DIFF 0). sub_plan 은 이 경로가 안 건드려 본체 루프가 원시 비교한다.
        "skip": [(0x5e8 + 0x8, 384 - 8)],
    },
}


# ★★**간접 전달 인자**를 게임 호출 전 상태로 되돌린다. (2026-09-13 신설)
#   왜 = IR 에서 `ptr dead_on_return ... %N` 은 **호출자가 반환 후 읽지 않는 값**이라
#        피호출이 그 버퍼를 **훼손해도 된다**. 그러면 게임 호출이 먼저 훼손하고, 내 사본은
#        **다른 입력**을 받는다 ⟹ 거짓 DIFF. (`StdRng` 을 되돌리는 것과 **정확히 같은 이유**다.)
#   값 = {slot: [(인자idx, 바이트수), …]}
ARG_SNAP = {
    # `#12 handle_chat` 의 `a6` = `ptr dead_on_return noalias readonly dereferenceable(24)`
    #   = 값으로 넘긴 `Chat`(24B)을 간접 전달한 것. 지금은 `readonly` 라 훼손하지 않지만
    #   **계약상 허용**돼 있으므로 되돌린다(rlib 이 바뀌면 곧바로 거짓 DIFF 가 된다).
    12: [(6, 24)],
}


# ★★★**힙 인식 스냅샷** — self 안의 **열거형 필드**가 variant 별로 소유 Vec 을 가질 때. (2026-09-13 신설)
#   왜 = `#11 v3_fall_back_to_passive` 는 `self.plan`(BigPlan@0x5e8)을 `drop_glue<BigPlan>` 로 **드롭한 뒤 교체**한다.
#        게임 호출이 옛 plan 의 Vec 버퍼를 해제한 뒤 내가 원시 바이트를 복원하면 **해제된 포인터가 되살아나고**,
#        내 사본의 drop 이 그걸 **또 해제**한다(이중해제). ⟹ 내 사본을 부르기 **전에** 그 variant 의 소유 Vec 들을
#        **내 힙 할당**으로 바꿔치기해서 내 사본의 drop 이 **내 것**을 해제하게 한다.
#   왜 static 버퍼가 아닌가 = `drop` 은 `free` 를 부른다 — static 을 free 하면 힙이 깨진다.
#   전제(IR 실측으로 확인할 것) = ①그 경로에서 self 소유 힙을 드롭하는 곳이 **그 필드뿐**이다
#        ②소유 필드가 **평면 Vec**(요소가 힙을 안 가짐)이다 — 아니면 재귀 치환이 필요하다.
#   값 = {slot: {"off": 열거형 필드의 self 내 오프셋, "tags": 유효 메모리태그 범위(밖이면 표본 제외),
#                "vecs": {메모리태그: [(열거형 내 절대 off, 요소 크기), …]}}}
# ★★요소 live 명세 — Vec 내용 비교 때 「그 타입이 실제로 쓰는 바이트」만 본다(이름으로 참조).
#   live = [(off, len, [tag…])] (tag 비면 항상) · str = [(String 시작 off, [tag…])] (len+내용 비교 · 내 사본 할당분 해제)
ELEM_LIVE = {
    # ★`game_core::Chat` — **tcx 자동 생성**(`MIG\enumlive.py game_core::Chat`). 손 맵은 `+8..15` 를 항상 live 로 둬
    #   tag 7(BattleStop, +8 은 1B)에서 거짓 DIFF 가 났다(2026-09-13). 57 variant 의 페이로드 위치가 전부 다르다.
    "chat": {"live": [
        (0x0, 1, []),
        (0x1, 1, [0x2, 0x5, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0x11, 0x12, 0x14, 0x15, 0x16, 0x1f, 0x20, 0x29, 0x2a, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38]),
        (0x2, 1, [0x2f, 0x30, 0x31, 0x32, 0x37]),
        (0x3, 1, [0x31]),
        (0x4, 4, [0x1]),
        (0x8, 8, [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38]),
        (0x10, 8, [0x3, 0x4, 0x6, 0x18, 0x21]),
    ], "str": []},
    # `game_core::PendingTraceEvent` 184B = TraceEventType(176B 니치 열거형, 태그 하위 1B = idx) + tick@0xb0.
    # #12 가 push 하는 idx 15 CallHandled = String×5 + from(4B)@0x80 + misunderstood(1B)@0x84.
    # `(usize, LineType)`·`(JungleType, usize)`·`(usize, LineType, u8)` 16B 튜플 — rustc 가 정렬 내림차순으로 재배치해
    # usize@0 · 소형 필드 @8(·@9) · 나머지 패딩. 패딩은 스택/힙 잔재라 비교하지 않는다.
    "tup16": {"live": [(0, 10, [])], "str": []},
    "pte": {"live": [(0x00, 8, []), (0x80, 5, [0x0f]), (0xb0, 8, [])],
            "str": [(0x08, [0x0f]), (0x20, [0x0f]), (0x38, [0x0f]), (0x50, [0x0f]), (0x68, [0x0f])]},
}

_BIGPLAN_VECS = {
    # ★2026-09-13 정정 2건: ①최상위 필드만 세서 **중첩 Vec 3개를 놓쳤고** 그게 #12 의 0xc0000374(힙 파손)였다 —
    #   tcx `--deep` 재전수(PassiveLinePlan `v46_pending.trigger_ticks`@0x60·`.flee_episodes`@0x78 · PassiveJunglePlan
    #   `counter_jungle_route@Some.route`@0x18, 요소 16B · **Option 니치 = cap 상위비트**).
    #   ②요소를 원시 바이트로 비교해 `Chat` 의 죽은 칸(+2)에서 거짓 DIFF 1건 — 요소에 ELEM_LIVE 를 붙인다.
    #   (off, esz, live): "chat" = Chat 24B 열거형 · "tup16" = (usize, 1~2B) 튜플 16B(usize@0 · 소형@8..10 · 나머지 패딩).
    3:  [(0x08 + 0x18, 24, "chat"), (0x08 + 0x30, 8), (0x08 + 0x48, 8), (0x08 + 0x60, 16, "tup16"), (0x08 + 0x78, 16, "tup16")],  # PassiveLinePlan
    4:  [(0x08 + 0x00, 24, "chat")],                                       # SinglePlanLine: chats
    5:  [(0x08 + 0x68, 24, "chat")],                                       # SinglePlanBattle: chats
    7:  [(0x08 + 0x00, 24, "chat"), (0x08 + 0x18, 16, "tup16")],           # PassiveJunglePlan: chats · counter_jungle_route.route
    9:  [(0x08 + 0x68, 24, "chat"), (0x08 + 0x80, 8)],                     # BattlePlan: chats·v54_reentry_ticks
    10: [(0x08 + 0x00, 24, "chat")],                                       # LineGankerPlan: chats
    11: [(0x08 + 0x00, 24, "chat")],                                       # LineGankCoverPlan: chats
    12: [(0x08 + 0x00, 8)],                                                # EpicHuntAndPokePlan: v46_flee_threats
    14: [(0x08 + 0x00, 8)],                                                # SerpenHuntAndPokePlan: v46_flee_threats
    # 2·8(ActiveRecallPlan 0B)·13·15·16·17 = 소유 없음
    # ★untagged(암묵) variant DeathMatchBattle = 태그 8B 가 2..=17 밖(페이로드 `support_target: Option<usize>` 의 0/1 이 그 자리).
    #   페이로드가 enum+0x0 에서 시작하므로 chats@0xf8 그대로. 2026-09-13 전엔 「소유 필드를 모른다 ⟹ 표본 제외」였다.
    None: [(0xf8, 24, "chat")],                                            # DeathMatchBattle: chats
}
_BIGPLAN_SPEC = {"off": 0x5e8, "tags": (2, 17), "vecs": _BIGPLAN_VECS}

# ★★★**힙 인식 스냅샷** — 슬롯당 **명세 목록**. 명세 = 열거형 필드({"off","tags","vecs": {태그: [(off,esz[,live])]}})
#   또는 평면 필드({"off": 0, "tags": None, "vecs": [(self 내 off, esz[, live])]}).
#   왜 = 게임 호출이 self 소유 힙을 **해제/재할당**했을 수 있으므로(drop 후 교체 · grow_one), 내 사본을 부르기 전에
#        그 Vec 들을 **내 힙 할당**으로 바꿔치기해 내 사본의 drop/realloc 이 **내 것**에만 닿게 한다.
#   왜 static 버퍼가 아닌가 = drop/grow_one 은 free/realloc 을 부른다 — static 이면 힙이 깨진다.
#   전제(IR 실측으로 확인) = ①그 경로에서 self 소유 힙을 건드리는 곳이 **여기 적은 필드뿐** ②요소가 힙을 안 갖거나(평면)
#        가지면 `ELEM_LIVE.str` 로 다룬다. ③착수 전 검사 = `heapsurf.py`(grow_one/drop 대상을 %0 오프셋으로 역추적).
HEAP_SUBST = {
    # `#11` — `self.plan` 하나(drop 후 교체). 2026-09-13 실측 6,364 DIFF 0(범위한정).
    11: [_BIGPLAN_SPEC],
    # `#12 handle_chat` — heapsurf 실측(2026-09-13): `handle_chat_inner` 의 self 힙 표면 = `drop_glue(self+0x5e8)`×16
    #   + `grow_one(self+0x7c8 chats)`×19 · 래퍼 = `grow_one(self+0x858 pending_trace_events)` · `passive_plan`/`ff_note_battle_swap`
    #   은 지역만 드롭. ⟹ 셋을 전부 치환한다. ★1차 시도(Vec 1개만 static 치환)는 30~40s 에 즉사했다 — 이 표가 그 답이다.
    12: [_BIGPLAN_SPEC,
         {"off": 0, "tags": None, "vecs": [(0x7c8, 24, "chat"), (0x858, 184, "pte")]}],
}
# (호환) 옛 이름
PLAN_SUBST = HEAP_SUBST


def heap_specs(i):
    """HEAP_SUBST[i] 를 [(off, tags|None, {tag_or_None: [(voff, esz, live|None)]})] 로 정규화."""
    out = []
    for sp in HEAP_SUBST.get(i, []) or []:
        vecs = sp["vecs"]
        if isinstance(vecs, dict):
            norm = {tg: [(v[0], v[1], (v[2] if len(v) > 2 else None)) for v in lst] for tg, lst in vecs.items()}
        else:
            norm = {None: [(v[0], v[1], (v[2] if len(v) > 2 else None)) for v in vecs]}
        out.append((sp["off"], sp.get("tags"), norm))
    return out


LIVE_IDS = {nm: n + 1 for n, nm in enumerate(sorted(ELEM_LIVE))}   # 0 = raw bytes


# ★★★**타입 기반 live 맵** — self 안 열거형 필드의 **페이로드**를 variant 별 살아있는 바이트만 비교한다. (2026-09-13 신설)
#   왜 = `#11`/`#12` 가 plan(BigPlan)·sub_plan(SubPlan) 페이로드 376B/64B 를 「스택 alloca → memcpy 열거형 = 잔재」로 통째 제외해
#        **범위한정 ev1** 에 머물렀다. `structlive.py` 가 tcx `--deep` 에서 잎 단위 live 맵(패딩·Vec 삼중항 제외 · 열거형/Option 은
#        variant 조건부 · Option<Vec> 는 cap 니치 · untagged variant 는 니치 범위 밖)을 자동 생성하므로, 그 맵으로 페이로드를
#        정밀 비교하면 제외가 사라진다. Vec 의 len·내용은 HEAP_SUBST(②′)가 따로 비교한다.
#   값 = {slot: [(열거형 필드의 self 내 off, 열거형 타입 전체이름), …]}  (태그 8B 는 본체 루프가 비교)
#   ⚠SELF_DIFF[slot]["skip"] 에 그 페이로드 구간을 **그대로 둔다**(본체 루프에서 빼고 여기서 정밀 비교).
ENUM_LIVE = {
    11: [(0x5e8, "game_ai::plan_legacy::types::BigPlan"), (0x768, "game_ai::plan_legacy::sub_plan::SubPlan")],
    12: [(0x5e8, "game_ai::plan_legacy::types::BigPlan")],
}
# ★midpin 슬롯용 — 명세 행(rows)이 아니라 `pin02.rs` 같은 손 훅이 부르는 비교 fn 도 같은 기계로 방출한다.
#   값 = {slot: [(기준 off, 열거형 타입)]} · 방출명 `enumlive_cmp_{slot}_{ei}`(pub) · 히스토그램 `EH_{slot}_{ei}`.
# ★★sret 버퍼가 **열거형**인 함수(09-13 r8 #61 i56 `PassiveLinePlan::sub_plan` → `SubPlan` 72B). SRET_LIVE 의 평면 span 으로는
#   variant 별 페이로드를 못 적으므로 pin02 와 같은 `enumlive_cmp_{idx}_0`(structlive 자동 생성)로 태그@0(8B) + variant 조건부 페이로드를 비교한다.
#   값 = {spec idx(또는 이름): 열거형 타입 전체이름}. `SRET_LIVE` 와 동시에 쓰지 않는다.
# ★★sret 버퍼가 **bumpalo Vec**(ptr@0 · &Bump@8 · cap@0x10 · len@0x18 · 32B)인 함수(09-13 r8 #46 i41 `fight_participants` →
#   `Vec<(&Entity, i64, bool), &Bump>` · 원소 24B = ptr@0 · i64@8 · bool@16 · IR m10.ll:39582~39586). ptr/bump/cap 은 게임·내 사본이
#   같은 Bump 에서 **따로** 할당하니 다르고, 판정은 **len + 원소 live 바이트**다(원소의 &Entity 는 게임 엔티티 주소라 양쪽 같다).
#   부작용 = 내 사본이 같은 Bump 에 한 번 더 할당한다(아레나는 틱마다 리셋 · 게임 값엔 영향 없음).
#   값 = {spec idx: {"ptr": off, "len": off, "esz": 원소 크기, "elem_live": [(off, len, [tags])]}}
SRET_VEC = {
    41: {"ptr": 0x0, "len": 0x18, "esz": 24, "elem_live": [(0, 8, []), (8, 8, []), (16, 1, [])]},
}
SRET_ENUM = {
    56: "game_ai::plan_legacy::sub_plan::SubPlan",
}
PIN_ENUM_LIVE = {
    2: [(0x0, "game_ai::plan_legacy::sub_plan::SubPlan")],   # `#02` 인라인 arm 의 sret(SubPlan 72B) — pin02.rs
}
import structlive as _SL


def self_diff_of(i, nm):
    """SELF_DIFF 조회 — 명세 슬롯은 숫자 idx, 명세 밖 이분 대상은 이름.
    ★`update`·`new` 처럼 흔한 이름이 충돌하지 않게 명세는 idx 로 건다."""
    if i in SELF_DIFF:
        return SELF_DIFF[i]
    return SELF_DIFF.get(nm)


# ★★명세(`_spec\specs20.json`)에 `exe.addr` 이 없는 슬롯의 **주소 보충표**. (2026-09-12 신설)
#   ⛔명세는 **읽기 전용**이므로(CLAUDE.md) 여기서 보충한다. 근거를 반드시 함께 적을 것.
#   ★생성기는 프롤로그를 **capstone 으로 exe 에서 직접** 뜬다(`probe20_tbl.rs` 의 길이는 교차검사용)
#     ⟹ **RVA 하나만 주면 sweep 이 성립한다.**
SPEC_RVA_OVERRIDE = {
    # `#07 EpicHuntAndBattlePlan::sub_plan`
    #   근거 = `BigPlan::sub_plan`(exe `0xcaf9f0`) 디스패처를 디스어셈해 분기 대상 12개를 뽑고,
    #          `dllmatch.json` 의 고신뢰 앵커 6개(jac ≥ 0.94: PassiveLinePlan `0xd2c5d0` ·
    #          SinglePlanLine `0xd781e0` · PassiveJunglePlan `0xd2e500` · EpicHuntAndPokePlan `0xdefcd0` ·
    #          SerpenHuntAndPokePlan `0xdf0e90` · DefenseNexusPlan `0xd2da10`)로 대응을 고정한 뒤,
    #          남은 후보를 **`aimap.json` 의 패닉 소스줄**로 확정했다:
    #            `0xccc010` → mod `plan_legacy/old/epic/hunt_and_battle`  ← **이것**
    #            `0xccc3c0` → mod `plan_legacy/old/serpen/hunt_and_battle`(SerpenHuntAndBattlePlan)
    #            `0xdb9430` → mod `plan_legacy/old/line_gank/ganker`(LineGankerPlan::sub_plan)
    #   ⚠1단계 「발화 0회」는 **주소가 없던 상태의 값**이라 무효다 — 이 주소로 다시 센다.
    7: 0xccc010,
}

# ★★주소가 **존재하지 않는다고 확정**된 슬롯 — 인라인돼 독립 진입부가 없다.
#   ⚠**적용 범위를 함께 적는다**(CLAUDE.md §11): 「진입부 detour / 호출부 리다이렉트 방식으로는」 불가다.
#   다른 접근(인라인된 호출자 전체를 대조하는 등)까지 닫는 판정이 아니다.
INLINED_NO_ENTRY = {
    # `#02 AttackNexusPlan::sub_plan` — `BigPlan::sub_plan`(`0xcaf9f0`)에 **인라인**됐다.
    #   근거 = `aimap.json` 에서 `0xcaf9f0` 자신의 `mod` 이 `plan_legacy/old/attack_nexus`(lines [36,36])다.
    #          디스패처의 분기 대상 어디에도 attack_nexus 모듈 함수가 없고, exe 전역에 그 모듈의
    #          독립 함수는 `0xe81680`·`0xe83080`(둘 다 `plan_legacy/sub_plan/attack_nexus` = 다른 모듈)뿐이다.
    #   ★2026-09-13: 그래서 행(rows)에서는 빼되 **`pin02.rs` midpin(A 0xcafa57 + B 0xcafdaa · 게이트 bit19)** 으로 대조한다 — 397,835 DIFF 0.
    2: u"`BigPlan::sub_plan`(0xcaf9f0)에 인라인 — 독립 진입부가 없다 ⟹ pin02.rs midpin(**bit62** = 0x4000000000000000 · 09-13 bit19→62 이동)으로 대조(2026-09-13 DIFF 0)",
}


def self_restore_of(i, nm):
    """SELF_RESTORE 조회 — 이름이 우선(명세 밖 항목), 없으면 숫자 idx."""
    if nm in SELF_RESTORE:
        return SELF_RESTORE[nm]
    return SELF_RESTORE.get(i)

RET_LIVE = {
    6: [0, 1, 2, 3, 5, 6],
}

SRET_LIVE = {
    # ★r7 잎(09-13): #25(i20) Option<ObjectiveDisciplineState> 32B — 니치 판별자 = kind@0x19(1B): 2=None · 0/1=Some.
    #   Some 이면 +0x00..0x19(wait_pos 16 · until_tick 8 · target 1) 살아있음 · 0x1a~ 패딩(memcpy 잔재)은 제외. 근거 = tcxdict + r7 명세 writes[].
    20: [
        (0x19, 1, []),
        (0x00, 0x19, [(0x19, 1, [0, 1])]),
    ],
    # #26(i21) upgrade_item → Option<(usize, usize)> 24B: tag@0(8B · 0=None 1=Some) · payload +8/+16 (ai_adjust 「sret 3워드(tag,own_idx,db_idx)」).
    21: [
        (0x00, 8, []),
        (0x08, 16, [(0x00, 8, [1])]),
    ],
    # #40(i35) resolve_fight_stake → FightPrediction 64B(tcxdict): focus_target Option<usize> tag@0 payload@8 · soaker tag@0x10 payload@0x18 ·
    #   rescue_ally tag@0x20 payload@0x28 · net_value@0x30 · line@0x38(1B) · line_absolute@0x39(1B) · 0x3a~ 패딩 제외. Option 태그 = 8B Direct(0/1).
    35: [
        (0x00, 8, []), (0x08, 8, [(0x00, 8, [1])]),
        (0x10, 8, []), (0x18, 8, [(0x10, 8, [1])]),
        (0x20, 8, []), (0x28, 8, [(0x20, 8, [1])]),
        (0x30, 8, []), (0x38, 1, []), (0x39, 1, []),
    ],
    # ★r8 잎(09-13): #49(i49) resolve_join_stake → FightPrediction 64B (i35 와 동일 레이아웃 · structlive 교차확인 일치).
    49: [
        (0x00, 8, []), (0x08, 8, [(0x00, 8, [1])]),
        (0x10, 8, []), (0x18, 8, [(0x10, 8, [1])]),
        (0x20, 8, []), (0x28, 8, [(0x20, 8, [1])]),
        (0x30, 8, []), (0x38, 1, []), (0x39, 1, []),
    ],
    # #55(i55) EntityPositioningCache::new → EntityPositioningCache 424B · 43필드 55잎 전부 무조건 live(패딩만 제외) — `structlive.py` 자동 생성 09-13.
    55: [
    (0x0, 8, []),
    (0x8, 8, []),
    (0x10, 8, []),
    (0x18, 8, []),
    (0x20, 8, []),
    (0x28, 8, []),
    (0x30, 8, []),
    (0x38, 8, []),
    (0x40, 8, []),
    (0x48, 8, []),
    (0x50, 8, []),
    (0x58, 8, []),
    (0x60, 8, []),
    (0x68, 8, []),
    (0x70, 8, []),
    (0x78, 8, []),
    (0x80, 8, []),
    (0x88, 8, []),
    (0x90, 8, []),
    (0x98, 8, []),
    (0xa0, 8, []),
    (0xa8, 8, []),
    (0xb0, 8, []),
    (0xb8, 8, []),
    (0xc0, 8, []),
    (0xc8, 8, []),
    (0xd0, 8, []),
    (0xd8, 8, []),
    (0xe0, 8, []),
    (0xe8, 8, []),
    (0xf0, 8, []),
    (0xf8, 8, []),
    (0x100, 8, []),
    (0x108, 8, []),
    (0x110, 8, []),
    (0x118, 8, []),
    (0x120, 8, []),
    (0x128, 8, []),
    (0x130, 8, []),
    (0x138, 8, []),
    (0x140, 8, []),
    (0x148, 8, []),
    (0x150, 8, []),
    (0x158, 8, []),
    (0x160, 8, []),
    (0x168, 8, []),
    (0x170, 8, []),
    (0x178, 8, []),
    (0x180, 8, []),
    (0x188, 8, []),
    (0x190, 8, []),
    (0x198, 8, []),
    (0x1a0, 1, []),
    (0x1a1, 1, []),
    (0x1a2, 1, []),
    ],
    # 명세 밖 이분 resolve_fight_uncached → FightPrediction 64B (i35 와 동일 레이아웃) · ★이름 키(idx 는 명세가 늘면 밀린다)
    u"resolve_fight_uncached": [
        (0x00, 8, []), (0x08, 8, [(0x00, 8, [1])]),
        (0x10, 8, []), (0x18, 8, [(0x10, 8, [1])]),
        (0x20, 8, []), (0x28, 8, [(0x20, 8, [1])]),
        (0x30, 8, []), (0x38, 1, []), (0x39, 1, []),
    ],
    0: [
        (0x00, 8, []),                                          # Input 판별자
        (0x08, 8, [(0x00, 8, [0])]),                            # Move.x
        (0x10, 8, [(0x00, 8, [0])]),                            # Move.y
        (0x08, 4, [(0x00, 8, [2, 3, 4, 5])]),                   # InputTarget 판별자(4B!)
        (0x10, 8, [(0x00, 8, [2, 3, 4, 5]), (0x08, 4, [0, 1, 2])]),   # target_id / dir_x / x
        (0x18, 8, [(0x00, 8, [2, 3, 4, 5]), (0x08, 4, [1, 2])]),      # dir_y / y
    ],
}

MUT_OK_ARG = {35: {13: "DebugFrameData"}, 49: {8: "DebugFrameData"}, 56: {7: "DebugFrameData"}, 41: {11: "DebugFrameData"}, u"resolve_fight_uncached": {3: "GameContext", 12: "DebugFrameData"}}   # #49 a8 = &mut DebugFrameData(224B · IR %8 dereferenceable(224))
# ★internal 함수는 define 에 `sret([N x i8])` 속성이 없다(LLVM 이 내부 호출규약에서 생략) — 파서가 「반환 void + 가변 a0」로 읽는다.
#   `resolve_fight_uncached`(a0 = dereferenceable(64) 출력 버퍼) 실사고(09-13). 여기 적은 idx 는 a0 을 sret N 바이트로 강제한다.
SRET_FORCE = {u"resolve_fight_uncached": 64}   # {spec idx: {IR 인자 idx: tcx 타입명}} — 위 MUT_OK_TCX 판정을 인덱스로 적용
MUT_OK_TCX = {
    "DebugFrameData": u"디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다"
                                 u"(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)",
}
def _byname(d, i, sp):
    u"""★명세 밖 EXTRA_SWEEP 항목은 **이름 키**로 찾는다(09-13 실사고: r8 17건 편입으로 EXTRA idx 44→62 가 밀려
    `SRET_LIVE/SRET_FORCE/MUT_OK_ARG[44]` 가 #44 take_misunderstood 에 붙었다 — SELF_RESTORE 의 이름 키 관습과 동일)."""
    nm = sp.get("name") or u""
    if nm in d:
        return d[nm]
    return d.get(i)


OK_ARG = tuple(RMAP.keys())
ARGSPLIT = re.compile(r",(?![^(]*\))")


# ───────────────────────── PE 섹션 → 파일 오프셋 ─────────────────────────
def sections(d):
    pe = struct.unpack_from("<I", d, 0x3C)[0]
    n = struct.unpack_from("<H", d, pe + 6)[0]
    opt = struct.unpack_from("<H", d, pe + 20)[0]
    return [struct.unpack_from("<IIII", d, pe + 24 + opt + i * 40 + 8) for i in range(n)]


# ───────────────────────── IR define 줄 파싱 ─────────────────────────
def define_of(f, frm):
    p = os.path.join(IRDIR, f)
    src = io.open(p, encoding="utf-8", errors="replace").read().split("\n")
    for k in range(frm - 1, min(frm + 8, len(src))):
        if src[k].lstrip().startswith("define"):
            return src[k]
    return None


def parse_define(dl):
    u"""`define <attrs> <ret> @sym(<args>)` → dict(ret, args=[(ty, deref, mutable)], sret, internal, fastcc)"""
    i = dl.find("@")
    head = dl[:i]
    internal = bool(re.search(r"\b(internal|private)\b", head))
    fastcc = bool(re.search(r"\bfastcc\b", head))
    # ★반환이 **중괄호 집합체**(`{ i8, i8 }`)면 아래 단어 패턴이 못 잡고 `head.split()[-1]` 로 떨어져
    #   `}` 라는 **가짜 타입**이 나온다(2026-09-12 적발 — `#03` 이 「반환 `}` 미지원」으로 제외돼 있었다).
    #   ⟹ 집합체를 **먼저** 시도한다. 「파싱 실패」가 「미지원」으로 둔갑하면 그 함수는 영영 안 열린다.
    ma = re.search(r"(\{[^{}]*\})\s*$", head.strip())
    if ma:
        ret = re.sub(r"\s+", " ", ma.group(1).strip())
    else:
        m = re.search(r"define\s+(?:internal\s+|private\s+|fastcc\s+|noundef\s+|zeroext\s+|signext\s+|dso_local\s+)*"
                      r"([\w.]+(?:\s*\{[^}]*\})?)\s*$", head.strip())
        ret = (m.group(1).strip() if m else head.strip().split()[-1])
    j = dl.find("(", i)
    depth, k = 0, j
    while k < len(dl):
        if dl[k] == "(":
            depth += 1
        elif dl[k] == ")":
            depth -= 1
            if depth == 0:
                break
        k += 1
    args, sret = [], False
    for a in ARGSPLIT.split(dl[j + 1:k]):
        a = a.strip()
        if not a:
            continue
        if "sret(" in a:
            sret = True
        ty = a.split()[0]
        dr = re.search(r"dereferenceable\((\d+)\)", a)
        # ★가변 판정 = ptr 인데 readonly/readnone 어느 쪽도 없다 ⟹ 이 포인터로 **쓴다**(고 봐야 한다).
        mut = (ty == "ptr") and not re.search(r"\breadonly\b|\breadnone\b", a)
        args.append((ty, int(dr.group(1)) if dr else 0, mut))
    return dict(ret=ret, args=args, sret=sret, internal=internal, fastcc=fastcc)


# ───────────────────────── rlib 심볼 노출(T/t) ─────────────────────────
def nm_table():
    rlib = None
    for d in RLIB_DIRS:
        if os.path.isdir(d):
            c = [x for x in os.listdir(d) if x.startswith("libgame_ai-") and x.endswith(".rlib")]
            if c:
                rlib = os.path.join(d, c[0])
                break
    if not rlib:
        sys.exit(u"libgame_ai-*.rlib 을 못 찾았다")
    out = subprocess.run([NM, "--defined-only", rlib], capture_output=True).stdout.decode("utf-8", "replace")
    tab = {}
    for ln in out.split("\n"):
        p = ln.split()
        if len(p) >= 2 and len(p[-2]) == 1:
            tab[p[-1]] = p[-2]          # 심볼 → 'T'(외부) / 't'(internal) / ...
    return rlib, tab


# ───────────────────────── 진입부 프롤로그(capstone) ─────────────────────────
def prolog_of(d, secs, rva):
    cs = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    o = None
    for vsz, va, rsz, ra in secs:
        if va <= rva < va + max(vsz, rsz):
            o = ra + rva - va
            break
    if o is None:
        return None, u"RVA 가 섹션 밖"
    tot, pro = 0, []
    for ins in cs.disasm(d[o:o + 64], BASE + rva):
        if "rip" in ins.op_str or ins.mnemonic.startswith("j") or ins.mnemonic == "call":
            break
        pro += list(ins.bytes)
        tot += ins.size
        if tot >= 12:
            break
    if tot < 12:
        return None, u"명령 경계 12B 확보 실패(분기/rip-상대가 앞에 있다)"
    return pro, None


# ───────────────────────── 본체 ─────────────────────────
def main():
    D = json.load(io.open(SPEC, encoding="utf-8"))["specs"]
    d = open(EXE, "rb").read()
    secs = sections(d)
    # ★`patches.json` 의 raw 패치 중 **`define` 에서 `fastcc` 를 벗긴** 심볼을 뽑는다.
    #   (그 rlib 을 링크하므로 IR 덤프의 `fastcc` 표기는 그 함수에 한해 낡은 사실이다.)
    global DEFASTCC
    DEFASTCC = set()
    try:
        pj = json.load(io.open(os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                            "patches.json"), encoding="utf-8"))
        for pc in pj:
            if not pc.get("raw"):
                continue
            f, t2 = pc.get("from", ""), pc.get("to", "")
            if f.startswith("define") and "fastcc" in f and "fastcc" not in t2:
                m2 = re.search(r"@([A-Za-z0-9_\$\.]+)\(", t2)
                if m2:
                    DEFASTCC.add(m2.group(1))
    except Exception as e:
        print(u"⚠patches.json 읽기 실패(%s) — fastcc 예외 없음으로 진행" % e)
    if DEFASTCC:
        print(u"★patches.json 으로 fastcc 제거된 심볼 %d개" % len(DEFASTCC))

    rlib, nmt = nm_table()

    # probe20_tbl.rs 의 RVA·프롤로그가 정본이다(1단계가 그걸로 실제 설치에 성공했다) → 대조용으로 읽는다.
    t = io.open(TBL, encoding="utf-8").read()
    P_RVA, P_LEN = {}, {}
    for m in re.finditer(r"idx:\s*(\d+),\s*rva:\s*(0x[0-9a-fA-F]+),\s*len:\s*(\d+)", t):
        P_RVA[int(m.group(1))] = int(m.group(2), 16)
        P_LEN[int(m.group(1))] = int(m.group(3))
    P_SITES = {}
    for m in re.finditer(r"idx:\s*(\d+),\s*target_rva:\s*(0x[0-9a-fA-F]+),\s*sites:\s*&\[([^\]]*)\]", t):
        i2 = int(m.group(1))
        P_RVA[i2] = int(m.group(2), 16)
        # ★사이트를 **1단계 표에서 그대로** 가져온다 — 그 표는 생성기가 exe 로 이미 검산했다
        #   (E8 인가 · 목표가 맞나 · .text 전역 스캔과 집합이 같나 · 간접호출 0곳인가).
        #   여기서 다시 적으면 두 곳이 어긋날 수 있다.
        P_SITES[i2] = [int(x.strip(), 16) for x in m.group(3).split(",") if x.strip()]

    # ★★명세 밖 **이분(bisect) 대상**을 sweep 슬롯으로 올린다. (2026-09-12 신설)
    #   왜 = `#18` 이 DIFF 88% 인데 갈림이 `v3_epic_group_line` 인지 그 입력(`strategy` 출력)인지
    #   구분할 방법이 없었다. 그 중간 함수를 **직접 대조**하면 한 번에 갈린다.
    #   ⟹ 명세와 같은 모양의 합성 항목을 만들어 기존 경로를 그대로 태운다(특수 분기 X).
    #   ⚠`idx` 는 20 이상(명세와 겹치지 않게) · 1단계 발화수가 없으므로 `무효` 로 찍힌다.
    for ex in EXTRA_SWEEP:
        D = D + [{
            "name": ex["name"], "sym": ex["sym"], "src": ex.get("src", "?"),
            "ir": {"file": ex["ir_file"], "frm": ex["ir_frm"], "to": ex["ir_frm"] + 400},
            "exe": {"addr": ex["addr"], "module": ex.get("module", "?"),
                    "bytes": ex.get("bytes", 400), "instrs": 0,
                    "evidence": ex.get("evidence", "bisect"), "callers": [], "callees": []},
            "sig": {"tcx": ex.get("tcx", ""), "params": []},
        }]

    rows, excl = [], []
    for i, sp in enumerate(D):
        nm = sp["name"]
        sym = sp.get("sym") or ""
        cnt = FIRED.get(i)
        rva = P_RVA.get(i)
        # ★★명세 밖 이분 대상은 **`probe20_tbl.rs` 의 idx 와 충돌하면 안 된다.**
        #   실사고(2026-09-12): 합성 항목이 idx 20 을 받았는데 그 자리엔 이미 `AUX[20] BigPlan::sub_plan`
        #   (`0xcaf9f0`)이 있어서 **엉뚱한 함수 주소가 붙었다**. ⟹ 자기 `exe.addr` 을 쓰고 idx 를 밀어 둔다.
        if i >= len(D) - len(EXTRA_SWEEP):
            rva = int((sp.get("exe") or {}).get("addr"), 16)
        # ★무효 표시가 있으면 **숫자를 쓰지 않는다**(위 INVALID_FIRE 주석 참조).
        cnt_s = INVALID_FIRE[i] if i in INVALID_FIRE else (
            u"{:,}".format(cnt) if isinstance(cnt, int) else (u"0" if i in DEAD else u"?"))
        # ★명세 밖 이분 대상은 1단계 발화수가 **애초에 없다** — 「미측정」으로 찍고
        #   `bad_cnt` 를 세워 아래 표기 경로가 숫자를 만지지 않게 한다(None 포맷 오류 방지).
        _extra = i >= len(D) - len(EXTRA_SWEEP)
        if _extra and cnt is None:
            cnt_s = u"미측정(명세 밖 이분 대상)"
        # ★표시 번호는 **여기 한 곳에서만** 만든다 — 포함표와 제외표가 다른 식을 쓰면
        #   같은 번호가 둘 찍힌다(2026-09-12 실사고: `#22` 가 repair_need·is_object 둘).
        didx = i + 1 if _extra else i

        def drop(reason):
            excl.append((didx, nm, cnt_s, reason))

        if i in DEAD:
            # ★사유를 여기 박아 두면 DEAD 표를 고쳐도 문면이 안 따라온다(11차 교훈:
            #   「규칙을 적는 것과 기계가 강제하는 것은 다르다」) ⟹ DEAD 값을 그대로 쓴다.
            drop(u"%s — ★프로브는 유지(뜰 때까지 계속 본다)" % DEAD[i])
            continue
        if rva is None:
            rva = SPEC_RVA_OVERRIDE.get(i)
        if rva is None:
            if i in INLINED_NO_ENTRY:
                drop(u"**인라인 — 독립 진입부가 없다**(진입부 detour·호출부 리다이렉트 방식 한정 불가): %s"
                     % INLINED_NO_ENTRY[i])
            else:
                drop(u"probe20_tbl.rs 에 RVA 가 없다(= 1단계에서도 측정 안 됨. MISSING20 참조) — "
                     u"`SPEC_RVA_OVERRIDE` 에 주소를 보충하면 열린다")
            continue
        ir = sp.get("ir") or {}
        dl = define_of(ir["file"], ir["frm"]) if ir.get("file") else None
        if not dl:
            drop(u"IR define 줄을 못 찾았다(시그니처 미확정)")
            continue
        g = parse_define(dl)
        link = nmt.get(sym, u"없음")
        why = []
        if link != "T":
            why.append(u"rlib 심볼 %s = 링크 불가(llvm-nm 실측 · IR %s)"
                       % (u"internal('t')" if link == "t" else u"'%s'" % link,
                          u"internal" if g["internal"] else u"external"))
        # ★`_gaibc` IR 덤프는 **패치 전** 원본이다. `MIG\patches.json` 이 그 함수의 `define` 에서
        #   `fastcc` 를 벗겼다면 링크되는 rlib 은 `ccc` 이므로 여기서 막으면 안 된다.
        #   ⚠손으로 예외 목록을 만들지 않는다 — **패치 명세에서 사실을 도출**한다(목록과 실제가 어긋나는
        #     것이 이 프로젝트의 단골 사고다). 그리고 링크 가능성은 위 `llvm-nm` 실측이 이미 강제한다.
        if g["fastcc"] and sym not in DEFASTCC:
            why.append(u"fastcc = 호출규약 비호환")
        elif g["fastcc"]:
            caveat.append(u"fastcc → ccc 로 **패치된 rlib**(deps_ailink)을 링크한다 — "
                          u"`MIG\\patches.json` · 심볼은 llvm-nm 으로 `T` 확인됨")
        # ★★sret = 반환이 **숨은 출력 버퍼**다. (2026-09-12 — 전용 래퍼 구현으로 제외 사유에서 내림)
        #   옛 판정은 `sret` 을 막고 그 위에 「반환 void = 비교할 값이 없다」까지 얹었는데,
        #   **둘 다 같은 오해**다 — ABI 상 반환이 void 인 것이지 **값이 없는 게 아니다.**
        #   값은 a0 가 가리키는 N 바이트에 있다 ⟹ **그 버퍼를 비교하면 된다.**
        #   래퍼 = 게임은 **호출자 버퍼**에 쓰게 두고(게임 진행은 게임 값으로),
        #        내 사본은 **스크래치 버퍼**에 쓰게 한 뒤 N 바이트를 대조한다.
        _sf = _byname(SRET_FORCE, i, sp)
        if _sf and not g["sret"]:
            g["sret"] = True
            g["args"][0] = ("ptr", _sf, True)
        sret_n = g["args"][0][1] if g["sret"] else 0
        if g["sret"] and sret_n <= 0:
            why.append(u"sret 인데 출력 크기(dereferenceable)를 못 읽었다 — 비교 범위 미상")
        # ★★★**sret 버퍼 전체 바이트 비교는 올바른 동등성 검사가 아니다.** (2026-09-12 런타임 실증)
        #   `#00 ult`(`Option<Input>` 32B)를 켜서 실측한 결과 **대조 20,415 중 DIFF 20,406**:
        #     g=[05, 00,          13, 00]
        #     m=[05, 17c00000000, 13, 64]
        #   ⟹ **의미 있는 워드(0·2)는 완전 일치**하고, **게임이 쓰지 않은 워드(1·3)** 에만 내 사본이
        #     값을 남겼다. LTO 가 「아무도 안 읽는 필드」의 store 를 죽였고, 비-LTO 인 rlib 사본은 쓴다.
        #   ⟹ 두 값은 **의미상 같은데** 바이트로는 다르다. 크래시도 재현 실패도 아니고 **판정식의 오류**다.
        #   ★올바르게 하려면 **그 타입의 「살아 있는 바이트 범위」**가 필요하다(variant 별로 다르다)
        #     — `tcxdict` 레이아웃에서 뽑아 `SRET_LIVE[idx] = [(off,len),…]` 로 주면 이 게이트가 열린다.
        #   ⚠**그때까지는 제외한다.** 「DIFF 2만건」을 재현 실패로 기록하면 그게 더 비싼 오류다.
        live = _byname(SRET_LIVE, i, sp) if g["sret"] else None
        sret_enum = _byname(SRET_ENUM, i, sp) if g["sret"] else None
        sret_vec = _byname(SRET_VEC, i, sp) if g["sret"] else None
        if g["sret"] and live is None and sret_enum is None and sret_vec is None:
            why.append(u"sret 반환 = **버퍼 전체 바이트 비교가 부당**(LTO 가 죽인 dead store 자리가 갈린다 — "
                       u"`#00` 실측 DIFF 20,406/20,415, 의미 워드는 전부 일치). "
                       u"`SRET_LIVE` 에 살아있는 바이트 범위를 주면 편입된다")
        # ★`{ i8, i8 }` = rustc **ScalarPair** — 두 레지스터(al:dl)로 돌아온다. sret 아니다.
        #   실증: `#03 defensive_crisis`(`0xe01c40`)의 `ret` 앞이 **pop 뿐**이고(= 메모리 출력 없음)
        #   `tcxdict` 상 `game_ai::DefensiveCrisis` = **2B struct**(`die_imminent: bool`·`cc_threat: bool`).
        #   `extern "Rust"` 로 선언하므로 **Rust 레이아웃 2필드 구조체**를 쓰면 rlib·게임 양쪽과 맞는다.
        rty = ("i64" if re.fullmatch(r"i64", g["ret"]) else
               "bool" if re.fullmatch(r"i1", g["ret"]) else
               "u8" if re.fullmatch(r"i8", g["ret"]) else
               # ★i24 반환(09-13): eax 하위 24비트만 비교(`U24` = Rust u32 · 상위 8비트 미정의).
               "U24" if re.fullmatch(r"i24", g["ret"]) else
               # ⚠IR 에서 `bool` 은 **`i1`** 이다(`i8` 이 아니다). ABI 상 스칼라 폭은 1바이트라
               #   Rust 쪽은 `u8` 로 받는다(`bool` 로 받으면 상위 비트 쓰레기가 **UB** 가 된다).
               "P8" if re.fullmatch(r"\{ i[18], i[18] \}", g["ret"]) else
               # ★16B ScalarPair. `ccc` 로 바꿔도 sret 이 되지 않는다 — 같은 rlib 안에
               #   `define { i64, i64 } @gc::setting4item17item_index_by_key(...)` 처럼
               #   **external + ccc + 직접 반환** 선례가 실재한다(2026-09-12 확인) ⟹ rax:rdx.
               "P64" if re.fullmatch(r"\{ i64, i64 \}", g["ret"]) else
               "()" if g["ret"].startswith("void") else None)
        if rty is None:
            why.append(u"반환 `%s` 미지원(전용 래퍼 필요)" % g["ret"])
        elif rty == "()" and not g["sret"] and not self_diff_of(i, nm):
            # ★★「반환 void = 비교할 값이 없다」는 **오판이었다**(2026-09-12 정정).
            #   ABI 상 반환이 void 인 것이지 **출력이 없는 게 아니다** — 출력은 `&mut self` 에 있다.
            #   `SELF_DIFF` 에 「무엇을 비교할지」를 등록하면 이 게이트가 열린다(`#14` 가 첫 사례).
            #   같은 오해의 형제 = ~~sret 는 반환이 void 라 비교 불가~~(§12 에서 이미 뒤집혔다).
            why.append(u"반환 void = **비교할 값이 없다** — `SELF_DIFF` 에 비교 대상을 등록하면 열린다")
        bad = sorted({a[0] for a in g["args"] if a[0] not in OK_ARG})
        if bad:
            why.append(u"인자 %s 미지원" % u"/".join(bad))
        # ★상한 9→16(09-13): 래퍼는 `unsafe fn(a0..aN)` Rust ABI 라 인자 수에 원리적 제한이 없다(#12 가 9). 9 는 「본 적 있는 최대」였을 뿐.
        #   #33(i28) 10인자 · #40(i35) 14인자(슬라이스 2 = ptr+len ×2 · Option<&Entity> = ptr) 편입.
        if len(g["args"]) > 16:
            why.append(u"인자 %d개(상한 16)" % len(g["args"]))
        # ★가변 포인터 인자 — 예외 ㉠StdRng(320B) 떠서 되돌림 ㉡tcx 가 공유참조(`&mut` 아님)
        params = (sp.get("sig") or {}).get("params") or []
        caveat = []
        mut = []
        for k, a in enumerate(g["args"]):
            # ★sret 의 a0 는 **출력 버퍼**라 「가변」이 당연하다 — 막을 이유가 아니다(래퍼가 분리한다).
            if g["sret"] and k == 0:
                continue
            # ★SELF_RESTORE 전략이 맡는 인자 — 「그냥 가변」이 아니라 **처리 방법이 있는 가변**이다.
            sr = self_restore_of(i, nm)   # ★예외 판정은 진단 스위치와 무관하게 유지한다
            if sr and k == sr[0]:
                caveat.append(u"a%d: &mut 게임 상태(%dB) — 게임 호출 전 상태로 되돌려 내 사본을 부르고 "
                              u"게임 상태로 복구한다. Vec 은 빈 것으로 바꿔 **게임 버퍼를 건드리지 않는다**"
                              % (k, sr[1]))
                continue
            if not a[2] or a[1] == 320:
                continue
            pt = None
            for p in params:                      # params 의 "i" 는 1-based 이고 IR 인자 순서와 같다
                if p.get("i") == k + 1:
                    pt = (p.get("type") or "").strip()
                    break
            ok = next((v for t2, v in MUT_OK_TCX.items() if pt and t2 in pt), None)
            # ★인자 인덱스 직접 허용(09-13): tcx 파라미터 수(11)와 IR 인자 수(14)가 다르면(슬라이스 분할) params 매칭이 안 되므로
            #   IR 인덱스로 직접 지정한다. #40(i35) a13 = &mut DebugFrameData(224B) = MUT_OK_TCX 와 같은 「디버그 싱크」 판정.
            if pt is None and k in (_byname(MUT_OK_ARG, i, sp) or {}):
                pt = _byname(MUT_OK_ARG, i, sp)[k]; ok = MUT_OK_TCX.get(pt.split()[0]) or u"디버그 싱크(인덱스 직접 허용)"
            if pt is None:
                mut.append((k, a[1], u"tcx 파라미터 확인 불가"))
            elif ok:
                # ★허용 목록(MUT_OK_TCX) — 가변이지만 **반환 대조에 영향이 없다**고 본 것. 반드시 caveat 로 남긴다.
                caveat.append(u"a%d: 가변이지만 편입 — tcx `%s` = %s"
                              % (k, re.split(r"\s+[—-]\s+", pt)[0][:44], ok))
            elif pt.startswith("&mut"):
                mut.append((k, a[1], u"tcx `%s`" % pt[:40]))
            else:
                # tcx 타입 문자열엔 명세의 서술이 붙어 있다(「— ★공유 참조…」) → 타입만 남긴다.
                caveat.append(u"a%d: IR readonly 표기 없음 · tcx `%s` = 공유참조(쓰기 관측 0) 근거로 편입"
                              % (k, re.split(r"\s+[—-]\s+", pt)[0][:40]))
        if mut:
            why.append(u"가변 포인터 인자 %s = 두 번 호출하면 상태가 두 번 변한다(스냅샷 복원도 불가 — 내부 Vec/Box 재할당)"
                       % u", ".join(u"a%d(%dB · %s)" % (k, sz, w2) for k, sz, w2 in mut))
        pro, perr = prolog_of(d, secs, rva)
        # ★★진입부를 못 빼도 **호출부 리다이렉트**로 sweep 할 수 있다(2026-09-12 신설).
        #   래퍼가 호출부에서 **피호출자 자리에** 앉으므로 인자를 그대로 받고, 원 함수 명령은 무손상이다.
        #   ⟹ 옛 판정 「진입부 12B 불가 ⟹ sweep 도 불가」는 **방식을 한정하지 않은 과잉 일반화**였다.
        #   조건 = 1단계가 이미 검산한 사이트 목록이 있을 것(전수성·간접호출 0 포함).
        sites = P_SITES.get(i) or []
        if sites:
            caveat.append(u"호출부 리다이렉트로 설치한다(진입부 12B 불가) — 사이트 %d곳은 1단계가 "
                          u"exe 로 검산한 것(전수·간접호출 0). 한 사이트라도 빠지면 표본은 **하한선**이다"
                          % len(sites))
        else:
            if perr:
                why.append(perr)
            if i in CALLSITE_PROBE and not why:
                why.append(u"%s ⟹ sweep 진입부 훅도 불가" % CALLSITE_PROBE[i])
        if why:
            drop(u" · ".join(why))
            continue
        if P_LEN.get(i) and P_LEN[i] != len(pro):
            print(u"  ⚠#%02d 프롤로그 길이가 probe20_tbl(%d) 과 다르다 → capstone 값 %d 사용"
                  % (i, P_LEN[i], len(pro)))
        rngs = [k for k, a in enumerate(g["args"]) if a[0] == "ptr" and a[1] == 320]
        # ★exe 가 그 인자를 버렸으면(EXE_ABI 에 "RNG") 래퍼엔 그 인자가 **없다**
        #   — 스냅샷하려 들면 엉뚝한 인자(실제로는 player)를 320B 읽는다.
        _abi = EXE_ABI.get(i)
        if _abi:
            rngs = [_abi[j] for j in rngs if isinstance(_abi[j], int)]
        rows.append(dict(sret_n=sret_n, live=live, sret_enum=sret_enum, sret_vec=sret_vec, sites=sites,
                         extra=(i >= len(D) - len(EXTRA_SWEEP)),
                         selfr=(None if SELF_RESTORE_OFF else self_restore_of(i, nm)),
                         idx=didx, name=nm, sym=sym, rva=rva, args=g["args"], rty=rty,
                         pro=(pro or []), rng=rngs, cnt=cnt, cnt_s=cnt_s, mod=sp.get("src", "?"),
                         bad_cnt=(i in INVALID_FIRE) or (cnt is None),
                         cav=u" / ".join(caveat),
                         tcx=(sp.get("sig") or {}).get("tcx", "")))

    # 호출수 오름차순 = **켜는 권장 순서**(적게 뜨는 것부터 — 사고 노출과 성능 충격을 작게).
    # ★1단계 발화수가 **없는** 항목(명세 밖 이분 대상)은 -1 로 둬 **맨 앞**에 온다 —
    #   「적게 뜨는 것부터 켠다」 규율과 같은 방향이고, None 이 섞이면 정렬 자체가 깨진다.
    rows.sort(key=lambda r: r["cnt"] if isinstance(r["cnt"], int) else -1)

    # ───────────────────────── 코드 생성 ─────────────────────────
    N = len(rows)
    L = []
    w = L.append
    w(u"//! sweep20.rs — **자동 생성**(`MIG\\gensweep20.py`). 손으로 고치지 말 것.")
    w(u"//! ===========================================================================")
    w(u"//! 2단계 = **sweep**: 게임 원본 함수와 **내 dll 안 링크사본**(`extern crate game_ai;` = SDK rlib")
    w(u"//!   640함수 = 재현 정본)을 **같은 인자로 각각 호출해 반환을 비트동일 대조**한다.")
    w(u"//!   1단계(발화수)는 끝났다 — 판 종료 #1 확정치 = 설치 19/19 · 발화 17 · 미발화 2.")
    w(u"//!")
    w(u"//! ★★**sweep 과 진입부 프로브는 같은 함수에 공존할 수 없다**(둘 다 진입부 12B 를 패치한다).")
    w(u"//!   ⟹ 여기 실린 함수는 sweep 이 **프로브를 대체**한다(`probe::install_all` 이 `is_installed_spec()`")
    w(u"//!      로 건너뛴다). 나머지는 1단계 프로브를 그대로 유지한다 — 표기 = `[sweep]`/`[진입부]`/`[호출부]`.")
    w(u"//!")
    w(u"//! 켜는 법(★**기본 OFF**): `<게임>\\mods\\tfm2_judge_verify\\sweep20_on.txt` 에 비트마스크를")
    w(u"//!   16진(`0x3`) 또는 10진으로 한 줄 적고 게임 재시작. 파일이 없거나 0 이면 **한 곳도 안 건다.**")
    for k, r in enumerate(rows):
        w(u"//!     bit%d = %#x  %-32s (1단계 발화 %s)"
          % (k, 1 << k, r["name"][:32],
             r["cnt_s"] if r["bad_cnt"] else (u"%s회" % "{:,}".format(r["cnt"]))))
        if r["cav"]:
            w(u"//!               ⚠caveat: %s" % r["cav"])
        if r["bad_cnt"]:
            # ★무효 수치로는 **성능 예측도 못 한다** — 정렬 순서조차 못 믿는다.
            w(u"//!               ⛔이 함수의 1단계 발화수는 **무효**다(그때 잰 주소가 다른 함수였다).")
            w(u"//!                 ⟹ 표본 수·성능 충격·켜는 순서를 이 값으로 판단하지 마라.")
            w(u"//!                 정정된 주소로 **1단계를 다시 돌린 뒤** 판단할 것.")
        elif r["cnt"] > 10_000_000:
            w(u"//!               ⚠호출수 %s = 대조가 그 함수의 실행을 **2배**로 만든다 → 프레임 지연 각오."
              % "{:,}".format(r["cnt"]))
    w(u"//!   ⚠전체를 한 번에 켜지 마라 — 선례(ai_adjust `fn_bisect` 비트2)에 **게임 즉사**가 있다.")
    w(u"//!     권장 순서 = 위에서 아래로(호출수 적은 것부터. 이유 = 사고 노출·성능 충격이 작다).")
    w(u"//!")
    w(u"//! ⚠거짓 DIFF 를 만드는 것들(설계상 처리한 것/못 한 것)")
    w(u"//!   ①`StdRng`(320B) 인자 = 게임 호출이 난수열을 소비하므로 **첫 호출 전에 떠서 두 번째 호출 전에")
    w(u"//!     되돌린다**. 안 되돌리면 내 사본이 다른 난수를 받아 DIFF 가 거짓으로 뜬다.")
    w(u"//!   ②가변 포인터 인자가 있는 함수는 **애초에 여기 안 들어온다**(EXCLUDED 참조) — 두 번 호출이")
    w(u"//!     상태를 두 번 바꾸고, 스냅샷 복원도 내부 Vec/Box 재할당에 무너진다.")
    w(u"//!   ③내 사본의 `thread_local` 메모 캐시는 **비어 있다**(게임 것과 별 인스턴스). 순수 메모면 같은")
    w(u"//!     값이 나오지만, 캐시가 이전 tick 값을 재사용하는 구조라면 DIFF 가 캐시 차이일 수 있다.")
    w(u"//!     ⟹ DIFF≠0 은 결론이 아니라 **시작**이다(첫 DIFF 의 인자·반환·몇 번째 호출을 남긴다).")
    w(u"//! ===========================================================================")
    w(u"#![allow(dead_code)]")
    w(u"use std::panic::{catch_unwind, AssertUnwindSafe};")
    w(u"use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};")
    w(u"use std::sync::Mutex;")
    w(u"")
    w(u"// ── 내 링크사본 직접 호출(rlib 의 Rust 망글 심볼). llvm-nm 으로 `T`(외부노출) 확인분만 있다 ──")
    w(u"extern \"Rust\" {")
    for k, r in enumerate(rows):
        sig = ", ".join("a%d: %s" % (j, RMAP[a[0]]) for j, a in enumerate(r["args"]))
        rr = "" if r["rty"] == "()" else " -> %s" % ("u32" if r["rty"] == "U24" else r["rty"])
        w(u"    /// #%02d %s — %s" % (r["idx"], r["name"], r["tcx"][:160]))
        w(u"    #[link_name = \"%s\"]" % r["sym"])
        w(u"    fn my_%d(%s)%s;" % (r["idx"], sig, rr))
    w(u"}")
    w(u"")
    w(u"/// ★이분용 스위치 — 1 이면 내 사본이 push 한 요소의 String 을 해제하지 않는다(누수 감수).")
    w(u"const BISECT_NO_STRFREE: u8 = %d;" % BISECT_NO_STRFREE)
    w(u"pub struct Slot {")
    w(u"    pub bit: u8, pub idx: u8, pub name: &'static str, pub src: &'static str,")
    w(u"    pub rva: usize, pub prolog: &'static [u8], pub stage1: u64, pub rng: &'static [u8],")
    w(u"    pub caveat: &'static str,")
    # ★호출부 방식 설치용 사이트 목록(비어 있으면 진입부 훅). 1단계 호출부 프로브와 같은 재료다.
    w(u"    /// 비어 있으면 **진입부 훅**, 차 있으면 **호출부 리다이렉트**(진입부 12B 를 못 빼는 함수).")
    w(u"    pub sites: &'static [usize],")
    w(u"    pub calls: AtomicU64, pub cmp: AtomicU64, pub diff: AtomicU64, pub pan: AtomicU64,")
    w(u"    /// ★내 Vec 사본 버퍼에 안 들어가 **표본에서 뺀** 호출 수. 조용한 누락을 막으려 센다.")
    w(u"    pub skip: AtomicU64,")
    w(u"    pub base_cmp: AtomicU64, pub base_diff: AtomicU64, pub orig: AtomicUsize,")
    w(u"}")
    w(u"macro_rules! sl { ($b:expr, $i:expr, $n:expr, $s:expr, $r:expr, $p:expr, $c:expr, $g:expr, $v:expr, $st:expr) => {")
    w(u"    Slot { bit: $b, idx: $i, name: $n, src: $s, rva: $r, prolog: $p, stage1: $c, rng: $g, caveat: $v, sites: $st,")
    w(u"           calls: AtomicU64::new(0), cmp: AtomicU64::new(0), diff: AtomicU64::new(0),")
    w(u"           pan: AtomicU64::new(0), skip: AtomicU64::new(0), base_cmp: AtomicU64::new(0), base_diff: AtomicU64::new(0),")
    w(u"           orig: AtomicUsize::new(0) } } }")
    w(u"pub static S: [Slot; %d] = [" % N)
    for k, r in enumerate(rows):
        w(u"    sl!(%d, %d, \"%s\", \"%s\", %#x, &[%s], %s, &[%s], \"%s\", &[%s]),"
          % (k, r["idx"], r["name"], r["mod"].replace("\\", "\\\\"), r["rva"],
             # ★무효는 `u64::MAX` 로 싣는다(0 = 미발화 오독 방지 · 위 report 가 문면으로 바꿔 찍는다).
             ", ".join("%#04x" % b for b in r["pro"]),
             (u"u64::MAX" if r["bad_cnt"] else r["cnt"]),
             ", ".join(str(x) for x in r["rng"]), r["cav"].replace('"', "'"),
             ", ".join("%#x" % s for s in (r.get("sites") or []))))
    w(u"];")
    w(u"")
    w(u"/// ★대조에서 **빠진** 명세 함수와 그 사유. 「빠진 것을 모르는 상태」를 만들지 않는다.")
    w(u"///   (idx, name, 1단계 발화수, 사유)")
    w(u"pub static EXCLUDED: &[(u8, &str, &str, &str)] = &[")
    for i, nm, c, why in excl:
        w(u"    (%d, \"%s\", \"%s\", \"%s\")," % (i, nm, c, why.replace("\\", "\\\\").replace('"', "'")))
    w(u"];")
    w(u"")
    w(u"/// 첫 DIFF 덤프(슬롯당 1건 + 전체 상한). detour 문맥에서 잡으므로 poison-safe 하게 연다.")
    w(u"static FIRST: Mutex<Vec<(usize, String)>> = Mutex::new(Vec::new());")
    w(u"const FIRST_MAX: usize = 64;")
    w(u"thread_local! { static D: [std::cell::Cell<u32>; %d] = [const { std::cell::Cell::new(0) }; %d]; }" % (N, N))
    w(u"/// 재진입 깊이. 최상위 호출에서만 대조한다(내 사본이 같은 함수를 재귀 호출해도 2중 대조 안 함).")
    w(u"#[inline] fn top(i: usize) -> bool { D.with(|d| { let v = d[i].get(); d[i].set(v + 1); v == 0 }) }")
    w(u"#[inline] fn pop(i: usize) { D.with(|d| d[i].set(d[i].get().saturating_sub(1))); }")
    if any(r["rty"] == "P8" for r in rows):
        w(u"/// `{ i8, i8 }`(rustc **ScalarPair** · al:dl 두 레지스터) 반환용.")
        w(u"/// ⚠`repr(Rust)` 그대로 둔다 — `extern \"Rust\"` 로 부르므로 rlib 의 원본 구조체와 같은")
        w(u"///   레이아웃·같은 ABI 여야 한다. `repr(C)` 를 붙이면 C ABI(ax 한 칸)로 바뀌어 어긋난다.")
        w(u"#[derive(PartialEq, Debug, Clone, Copy)] pub struct P8 { pub a: u8, pub b: u8 }")
        w(u"")
    if any(r["rty"] == "P64" for r in rows):
        w(u"/// `{ i64, i64 }`(ScalarPair · rax:rdx) 반환용. `repr(Rust)` 유지(위 `P8` 와 같은 이유).")
        w(u"#[derive(PartialEq, Debug, Clone, Copy)] pub struct P64 { pub a: i64, pub b: i64 }")
        w(u"")
    # ── sret 살아있는-구간 비교기 ───────────────────────────────────────────
    #   ★전 바이트 비교가 왜 안 되는지는 위 `SRET_LIVE` 주석 참조(패딩·미사용 자리가 갈린다).
    if any(r.get("live") for r in rows):
        w(u"/// 살아있는 구간 하나. `c` 의 조건이 **전부** 성립할 때만 비교 대상이다.")
        w(u"#[derive(Clone, Copy)] pub struct Cond { pub off: u16, pub len: u8, pub mask: u64 }")
        w(u"#[derive(Clone, Copy)] pub struct Span { pub off: u16, pub len: u8, pub c: [Cond; 2] }")
        w(u"const NOC: Cond = Cond { off: 0, len: 0, mask: 0 };")
        w(u"/// 리틀엔디언 태그 읽기(1~8B).")
        w(u"#[inline] unsafe fn rdtag(p: *const u8, off: u16, len: u8) -> u64 {")
        w(u"    let mut v = 0u64;")
        w(u"    for i in 0..len as usize { v |= (*p.add(off as usize + i) as u64) << (8 * i); }")
        w(u"    v")
        w(u"}")
        w(u"/// 게임 버퍼 `g` 의 판별자로 **살아있는 구간만** 골라 `m` 과 비교한다.")
        w(u"/// 반환 = 갈린 첫 구간의 오프셋(없으면 None) — 「어디가」를 남겨야 다음이 짧다.")
        w(u"unsafe fn live_eq(g: *const u8, m: *const u8, sp: &[Span]) -> Option<usize> {")
        w(u"    for s in sp {")
        w(u"        let mut on = true;")
        w(u"        for c in s.c.iter() {")
        w(u"            if c.len == 0 { continue; }")
        w(u"            let t = rdtag(g, c.off, c.len);")
        w(u"            if t >= 64 || (c.mask >> t) & 1 == 0 { on = false; break; }")
        w(u"        }")
        w(u"        if !on { continue; }")
        w(u"        let (o, l) = (s.off as usize, s.len as usize);")
        w(u"        if core::slice::from_raw_parts(g.add(o), l) != core::slice::from_raw_parts(m.add(o), l) {")
        w(u"            return Some(o);")
        w(u"        }")
        w(u"    }")
        w(u"    None")
        w(u"}")
        for r in rows:
            if not r.get("live"):
                continue
            items = []
            for off, ln, conds in r["live"]:
                cs = []
                for toff, tlen, allowed in conds[:2]:
                    mask = 0
                    for a in allowed:
                        mask |= 1 << a
                    cs.append(u"Cond { off: %#x, len: %d, mask: %#x }" % (toff, tlen, mask))
                while len(cs) < 2:
                    cs.append(u"NOC")
                items.append(u"    Span { off: %#x, len: %d, c: [%s] }," % (off, ln, ", ".join(cs)))
            w(u"/// `#%02d %s` 의 살아있는 구간(생성기 `SRET_LIVE` 에서 자동 생성)." % (r["idx"], r["name"]))
            w(u"static LIVE_%d: &[Span] = &[" % r["idx"])
            for it in items:
                w(it)
            w(u"];")
        w(u"")
    w(u"fn note(i: usize, s: String) {")
    w(u"    S[i].diff.fetch_add(1, Ordering::Relaxed);")
    w(u"    let mut g = FIRST.lock().unwrap_or_else(|e| e.into_inner());")
    w(u"    if g.len() < FIRST_MAX && !g.iter().any(|x| x.0 == i) { g.push((i, s)); }")
    w(u"}")
    w(u"")
    # ★SELF_RESTORE 용 thread_local 버퍼 — 스택을 쓰면 rayon 워커에서 터진다(위 주석).
    if any(EXE_ABI.get(r["idx"]) and any(s == "RNG" for s in EXE_ABI[r["idx"]]) for r in rows):
        w(u"/// IR 의 `align 16` 계약을 지키기 위한 래퍼 — 지금은 `readnone` 이라 무해하지만")
        w(u"/// rlib 가 바뀌어 `movaps` 로 읽게 되면 정렬 위반은 곰바로 0xc0000005 다.")
        w(u"#[repr(align(16))] struct A16([u8; 320]);")
        w(u"")
    for k, r in enumerate(rows):
        if EXE_ABI.get(r["idx"]) and any(s == "RNG" for s in EXE_ABI[r["idx"]]):
            w(u"thread_local! {")
            w(u"    /// `#%02d` — exe 가 **버린** `&mut StdRng` 자리에 넘길 더미(320B)." % r["idx"])
            w(u"    /// 게임은 이 인자를 안 쓴다(그래서 dead-arg 로 제거됐다) — 내 사본도 안 써야 정상이고,")
            w(u"    /// 쓴다면 DIFF 로 드러난다(= 그 자체가 새 사실).")
            w(u"    static RNG%d: core::cell::UnsafeCell<A16> = core::cell::UnsafeCell::new(A16([0u8; 320]));" % k)
            w(u"}")
        if not r.get("selfr"):
            continue
        sz0 = r["selfr"][1]
        w(u"thread_local! {")
        w(u"    /// `#%02d` 의 **게임 호출 전** self 스냅샷(%dB). 스레드당 1개 — 재진입은 `top()` 이 막는다." % (r["idx"], sz0))
        w(u"    static SV%d: core::cell::UnsafeCell<[u8; %d]> = core::cell::UnsafeCell::new([0u8; %d]);" % (k, sz0, sz0))
        w(u"    /// `#%02d` 의 **게임 호출 후** self 스냅샷(%dB)." % (r["idx"], sz0))
        w(u"    static SP%d: core::cell::UnsafeCell<[u8; %d]> = core::cell::UnsafeCell::new([0u8; %d]);" % (k, sz0, sz0))
        _esz = r["selfr"][3]
        # ★예산 ~~4096B~~ → **12288B**(2026-09-12). 24B 요소면 170개 → **512개**.
        #   왜 = 아래 가드를 `ln + 2 <= cap` 으로 조이면서 실효 용량이 줄기 때문이고,
        #   더 근본적으로는 **채팅 큐가 170개를 넘으면 표본이 통째로 버려지기** 때문이다.
        _cap = 12288 // max(_esz, 1)
        if self_diff_of(r["idx"], r["name"]):
            w(u"    /// `#%02d` 의 **내 사본 호출 후** self 스냅샷(%dB) — 반환값이 죽은 슬롯의 판정 재료." % (r["idx"], sz0))
            w(u"    static SQ%d: core::cell::UnsafeCell<[u8; %d]> = core::cell::UnsafeCell::new([0u8; %d]);" % (k, sz0, sz0))
        for si, _ in enumerate(heap_specs(r["idx"])):
            w(u"    /// `#%02d` 명세 %d 소유 Vec 의 **게임 호출 전** 내용(32KB) + (cap,len,esz)×4 + 개수 + 수용 여부." % (r["idx"], si))
            w(u"    ///   ⚠게임 호출이 그 버퍼를 **해제/재할당**했을 수 있어 호출 뒤에 읽으면 freelist 잔재다.")
            w(u"    static PV%d_%d: core::cell::UnsafeCell<([u8; 32768], [(usize, usize, usize); 8], usize, bool)> = core::cell::UnsafeCell::new(([0u8; 32768], [(0, 0, 0); 8], 0, false));" % (k, si))
        w(u"    /// `#%02d` 의 `Vec` 사본 버퍼(용량 %d개) + 게임과 같은 len." % (r["idx"], _cap))
        w(u"    /// `.2` = 이번 호출이 내 버퍼에 **들어갔나**. 안 들어갔으면 비교를 건너뛴다(거짓 DIFF 방지).")
        w(u"    static VB%d: core::cell::UnsafeCell<([u8; %d], usize, bool)> = core::cell::UnsafeCell::new(([0u8; %d], 0, false));" % (k, _cap * _esz, _cap * _esz))
        w(u"}")
    # ★요소 live 비교 헬퍼(한 번). live id 0 = 원시 바이트. 반환 = 첫 불일치 설명.
    w(u"/// Vec 요소 비교 — `lid` 가 가리키는 ELEM_LIVE 명세로 **살아있는 바이트**만 본다(0 = 전 바이트).")
    w(u"unsafe fn elem_cmp(lid: u8, esz: usize, gb: usize, mb: usize) -> Option<String> {")
    w(u"    let tag = *(gb as *const u8);")
    w(u"    match lid {")
    for nm, lid in sorted(LIVE_IDS.items(), key=lambda x: x[1]):
        spec = ELEM_LIVE[nm]
        w(u"        %d => {" % lid)
        for (o, l, tg) in spec["live"]:
            cond = u"true" if not tg else u" || ".join("tag == %#04x" % t for t in tg)
            w(u"            if %s { for j in %d..%d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));"
              % (cond, o, o + l))
            w(u"                if gv != mv { return Some(format!(\"(tag {:#x})+{}: g={:02x} m={:02x}\", tag, j, gv, mv)); } } }")
        for (so, tg) in spec.get("str", []):
            cond = u"true" if not tg else u" || ".join("tag == %#04x" % t for t in tg)
            w(u"            if %s {" % cond)
            w(u"                let (gp, gl) = (*((gb + %#x + 8) as *const usize), *((gb + %#x + 16) as *const usize));" % (so, so))
            w(u"                let (mp, ml) = (*((mb + %#x + 8) as *const usize), *((mb + %#x + 16) as *const usize));" % (so, so))
            w(u"                if gl != ml { return Some(format!(\"+{:#x}.len: g={} m={}\", %#x, gl, ml)); }" % so)
            w(u"                if gl > 0 && gl < 65536 && gp > 0x1000 && mp > 0x1000 && gp != mp {")
            w(u"                    for j in 0..gl { let (gv, mv) = (*((gp + j) as *const u8), *((mp + j) as *const u8));")
            w(u"                        if gv != mv { return Some(format!(\"+{:#x}.str[{}]: g={:02x} m={:02x}\", %#x, j, gv, mv)); } }" % so)
            w(u"                }")
            w(u"            }")
        w(u"            None")
        w(u"        }")
    w(u"        _ => { for j in 0..esz { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));")
    w(u"            if gv != mv { return Some(format!(\"+{}: g={:02x} m={:02x}\", j, gv, mv)); } } None }")
    w(u"    }")
    w(u"}")
    w(u"/// 요소 안 String(ELEM_LIVE.str) 해제 — **내 사본이 새로 push 한 요소**에만 쓸 것.")
    w(u"unsafe fn elem_free_str(lid: u8, mb: usize) {")
    w(u"    let tag = *(mb as *const u8);")
    w(u"    match lid {")
    for nm, lid in sorted(LIVE_IDS.items(), key=lambda x: x[1]):
        spec = ELEM_LIVE[nm]
        if not spec.get("str"):
            continue
        w(u"        %d => {" % lid)
        for (so, tg) in spec["str"]:
            cond = u"true" if not tg else u" || ".join("tag == %#04x" % t for t in tg)
            w(u"            if %s { let (c, p) = (*((mb + %#x) as *const usize), *((mb + %#x + 8) as *const usize));" % (cond, so, so))
            w(u"                if c > 0 && c < (1 << 24) && p > 0x1000 { if let Ok(l) = std::alloc::Layout::from_size_align(c, 1) { std::alloc::dealloc(p as *mut u8, l); } } }")
        w(u"        }")
    w(u"        _ => {}")
    w(u"    }")
    w(u"}")
    for k, r in enumerate(rows):
        hs = heap_specs(r["idx"]) if r.get("selfr") else []
        for si, (boff, tags, vecs) in enumerate(hs):
            w(u"/// `#%02d` 명세 %d — self+%#x 의 %s → 소유 Vec 목록(off, 요소 크기, live id)." %
              (r["idx"], si, boff, u"메모리태그" if tags else u"평면 필드(태그 무관)"))
            w(u"fn hs_vecs_%d_%d(tag: u64) -> &'static [(usize, usize, u8)] {" % (r["idx"], si))
            w(u"    match tag {")
            for tg, lst in sorted(vecs.items(), key=lambda x: (x[0] is None, x[0] or 0)):
                # None 키 = 태그 범위 밖(untagged/암묵 variant) — 평면 명세(tags 없음)면 `_`
                arm = (u"t if !(%d..=%d).contains(&t)" % tags if (tg is None and tags) else u"_") if tg is None else u"%d" % tg
                w(u"        %s => &[%s]," % (arm, u", ".join(u"(%#x, %d, %d)" % (o, e, LIVE_IDS.get(lv, 0)) for (o, e, lv) in lst)))
            if None not in vecs or tags:
                w(u"        _ => &[],   // 단위 variant 등 소유 없음")
            w(u"    }")
            w(u"}")
    # ★타입 기반 live 맵 비교 fn — (slot, 열거형) 마다 1개. gb/mb = 열거형 필드의 기준 주소(태그 위치).
    w(u"#[inline(always)] unsafe fn rd_le(p: usize, n: usize) -> u64 { let mut v = 0u64; for j in 0..n { v |= (*((p + j) as *const u8) as u64) << (8 * j); } v }")
    def _cond_rs(conds):
        cs, hibeq = [], None
        for c in conds:
            if c[0] == "direct":
                cs.append(u"rd_le(gb + %#x, %d) == %d" % (c[1], c[2], c[3]))
            elif c[0] == "notin":
                cs.append(u"!matches!(rd_le(gb + %#x, %d), %s)" % (c[1], c[2], u" | ".join(u"%d" % v for v in c[3])))
            elif c[0] == "hib":
                cs.append(u"((rd_le(gb + %#x, 8) >> 63) == 0) == %s" % (c[1], u"true" if c[2] else u"false"))
            elif c[0] == "hibeq":
                hibeq = c[1]
        return (u" && ".join(cs) if cs else u"true"), hibeq
    _el_items = [(r["idx"], ei, eoff, ety) for r in rows if r.get("selfr") for ei, (eoff, ety) in enumerate(ENUM_LIVE.get(r["idx"], []))]
    _el_items += [(slot, ei, eoff, ety) for slot, lst in sorted(PIN_ENUM_LIVE.items()) for ei, (eoff, ety) in enumerate(lst)]
    _el_items += [(r["idx"], 0, 0, r["sret_enum"]) for r in rows if r.get("sret_enum")]   # sret 열거형(09-13)
    for (_idx, ei, eoff, ety) in _el_items:
        r = {"idx": _idx}
        if True:
            emap = _SL.build_enum(ety)
            lo, hi = _SL.enum_tag_range(ety)
            w(u"/// `#%02d` self+%#x `%s` — variant 별 페이로드 live 비교(structlive.py 자동 생성 · variant %d개 · 태그 %d..=%d · 밖 = untagged)." % (r["idx"], eoff, ety.split("::")[-1], len(emap), lo, hi))
            names = _SL.enum_tags(ety)   # {variant: memtag|None}
            w(u"/// variant 히스토그램 — [0] = untagged(범위 밖) · [t] = 메모리태그 t (대조 1건당 1증가 · 단위 variant 포함).")
            w(u"pub static EH_%d_%d: [AtomicU64; 32] = [const { AtomicU64::new(0) }; 32];" % (r["idx"], ei))
            w(u"pub static EH_%d_%d_NAMES: &[(u64, &str)] = &[%s];" % (r["idx"], ei,
              u", ".join(u"(%d, \"%s\")" % (0 if t is None else t, nm) for nm, t in sorted(names.items(), key=lambda x: (x[1] is None, x[1] or 0)))))
            w(u"pub unsafe fn enumlive_cmp_%d_%d(tag: u64, gb: usize, mb: usize) -> Option<String> {" % (r["idx"], ei))
            w(u"    EH_%d_%d[if (%d..=%d).contains(&tag) { tag as usize } else { 0 }].fetch_add(1, Ordering::Relaxed);" % (r["idx"], ei, lo, hi))
            w(u"    match tag {")
            for tg, (pty, live) in sorted(emap.items(), key=lambda x: (x[0] < 0, x[0])):
                arm = (u"t if !(%d..=%d).contains(&t)" % (lo, hi)) if tg < 0 else (u"%d" % tg)
                w(u"        %s => {   // %s · 잎 %d%s" % (arm, pty.split("::")[-1], len(live), u" · untagged(페이로드 base 0)" if tg < 0 else u""))
                for (o, n, conds) in live:
                    cond, hibeq = _cond_rs(conds)
                    if hibeq is not None:
                        w(u"            if %s { if (rd_le(gb + %#x, 8) >> 63) != (rd_le(mb + %#x, 8) >> 63) { return Some(format!(\"+{:#x}.opt g={} m={}\", %#x, if rd_le(gb + %#x, 8) >> 63 != 0 { \"None\" } else { \"Some\" }, if rd_le(mb + %#x, 8) >> 63 != 0 { \"None\" } else { \"Some\" })); } }"
                          % (cond, hibeq, hibeq, hibeq, hibeq, hibeq))
                    else:
                        w(u"            if %s { for j in %#x..%#x { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!(\"+{:#x}: g={:02x} m={:02x}\", j, gv, mv)); } } }"
                          % (cond, o, o + n))
                w(u"            None")
                w(u"        }")
            w(u"        _ => None,   // 단위 variant(페이로드 0B)")
            w(u"    }")
            w(u"}")
    w(u"")
    for k, r in enumerate(rows):
        tys = [RMAP[a[0]] for a in r["args"]]          # IR 기준 타입(내 사본이 기대하는 것)
        # ★★exe ABI 가 IR 과 다르면 **래퍼는 exe 순서**로 받고 **내 사본은 IR 순서**로 부른다.
        abi = EXE_ABI.get(r["idx"])
        if abi:
            nexe = max(x for x in abi if isinstance(x, int)) + 1
            etys = [None] * nexe
            for ir_i, src in enumerate(abi):
                if isinstance(src, int):
                    etys[src] = tys[ir_i]
            etys = [x or "*const u8" for x in etys]
            sig = ", ".join("a%d: %s" % (j, ty) for j, ty in enumerate(etys))
            call = ", ".join("a%d" % j for j in range(nexe))        # 게임 원본 = exe 순서 그대로
            mycall = ", ".join(("RNG%d.with(|c| c.get() as *const u8)" % k) if s == "RNG"
                               else ("a%d" % s) for s in abi)        # 내 사본 = IR 순서
        else:
            sig = ", ".join("a%d: %s" % (j, t) for j, t in enumerate(tys))
            call = ", ".join("a%d" % j for j in range(len(tys)))
            mycall = call
        rr = "" if r["rty"] == "()" else " -> %s" % ("u32" if r["rty"] == "U24" else r["rty"])
        # ★DIFF 덤프에 찍을 인자도 **래퍼가 실제로 받은 것**(exe 서명)이어야 한다 —
        #   IR 기준으로 만들면 없는 `a6` 를 참조해 컴파일이 깨진다(2026-09-12 실측).
        fmt, vals = [], []
        _dtys = etys if abi else tys
        for j, ty in enumerate(_dtys):
            if ty == "*const u8":
                fmt.append("a%d={:#x}" % j)
                vals.append("a%d as usize" % j)
            else:
                fmt.append("a%d={}" % j)
                vals.append("a%d" % j)
        w(u"unsafe fn w_%d(%s)%s {" % (r["idx"], sig, rr))
        w(u"    S[%d].calls.fetch_add(1, Ordering::Relaxed);" % k)
        # ★원본 함수의 타입은 **exe 서명**이다(IR 이 아니다). `EXE_ABI` 가 있으면 그쪽을 쓴다 —
        #   IR 타입으로 두면 인자 개수가 어긋나 게임 호출 자체가 틀어진다.
        _ftys = etys if abi else tys
        w(u"    let f: unsafe fn(%s)%s = core::mem::transmute(S[%d].orig.load(Ordering::Relaxed));"
          % (", ".join(_ftys), rr, k))
        w(u"    let t = top(%d);" % k)
        sr = r.get("selfr")
        if sr:
            ak0, sz0, _v0, _e0 = sr
            w(u"    // ★★&mut 게임 상태(%dB) — **게임 호출 전** 상태를 떠 두지 않으면 내 사본은" % sz0)
            w(u"    //   게임이 바꿔놓은 입력을 보게 돼 **거짓 DIFF** 가 난다(이 함수는 self 를 읽고도 쓴다).")
            w(u"    // ★★**스택이 아니라 thread_local 힙**에 둔다 — %dB × 2 를 스택에 잡으면" % sz0)
            w(u"    //   rayon 워커의 **소스택 + 게임 sim 재귀**에서 STATUS_STACK_OVERFLOW 로 죽는다")
            w(u"    //   (2026-09-12 실사고: 스택 배열로 두던 판이 배경 sim 시작 직후 패닉로그 없이 즉사).")
            w(u"    SV%d.with(|c| { let sv = &mut *c.get();" % k)
            w(u"        if t { core::ptr::copy_nonoverlapping(a%d, sv.as_mut_ptr(), %d); } });" % (ak0, sz0))
            # ★★Vec 의 **내용까지** 게임 호출 전에 떠 둔다.
            #   ~~빈 Vec(cap=0)~~ 로 주면 내 사본이 `len=0` 을 보는데 게임은 `len=N` 을 봤다
            #   ⟹ 「이미 채팅했나」류 판정이 갈려 **거짓 DIFF** 가 난다(실측 185,288/212,545).
            #   ⚠뜨는 시점이 **게임 호출 전**이어야 한다 — 게임 호출이 재할당하면 옛 ptr 은 해제된다.
            if not _v0:
                w(u"    VB%d.with(|c| { (&mut *c.get()).2 = true; });   // 치환할 Vec 이 없다 = 항상 표본" % k)
            for (co, po, lo) in _v0:
                w(u"    VB%d.with(|c| { let vb = &mut *c.get(); if t {" % k)
                w(u"        let ln = core::ptr::read_unaligned((a%d as usize + %#x) as *const usize);" % (ak0, lo))
                w(u"        let pz = core::ptr::read_unaligned((a%d as usize + %#x) as *const usize);" % (ak0, po))
                w(u"        vb.1 = ln; vb.2 = ln + 2 <= %d;" % (12288 // max(_e0, 1)))
                # ★★`ln == cap` 이면 내 사본의 push 가 `RawVec::grow_one` 을 불러
                #   **thread_local static 버퍼를 realloc/free** 하려 든다 = 힙 파손(잠복 크래시).
                #   ⟹ 여유 2칸을 남긴 `ln + 2 <= cap` 으로 조인다. 넘치면 그 호출은 표본에서 뺀다.
                w(u"        if ln > 0 && ln + 2 <= %d && pz > 0x1000 {" % (12288 // max(_e0, 1)))
                w(u"            core::ptr::copy_nonoverlapping(pz as *const u8, vb.0.as_mut_ptr(), ln * %d);" % _e0)
                w(u"        } else { vb.1 = ln; vb.2 = ln + 2 <= %d; }" % (12288 // max(_e0, 1)))
                w(u"    } });")
        for si, (boff, tags, vecs) in enumerate(heap_specs(r["idx"]) if r.get("selfr") else []):
            ak0 = r["selfr"][0]
            w(u"    // ★명세 %d: self+%#x 의 소유 Vec **내용을 게임 호출 전에** 떠 둔다 — 게임이 해제/재할당할 수 있다." % (si, boff))
            w(u"    PV%d_%d.with(|c| { let pv = &mut *c.get(); pv.2 = 0; pv.3 = true; if t {" % (k, si))
            w(u"        let tg = %s;" % (u"core::ptr::read_unaligned((a%d as usize + %#x) as *const u64)" % (ak0, boff) if tags else u"0u64"))
            w(u"        let mut used = 0usize;")
            w(u"        for (i, &(off, esz, _)) in hs_vecs_%d_%d(tg).iter().enumerate() {" % (r["idx"], si))
            w(u"            if i >= 8 { pv.3 = false; break; }")
            w(u"            let b = a%d as usize + %#x + off;" % (ak0, boff))
            w(u"            let (cap, ptr, len) = (core::ptr::read_unaligned(b as *const usize),")
            w(u"                                   core::ptr::read_unaligned((b + 8) as *const usize),")
            w(u"                                   core::ptr::read_unaligned((b + 16) as *const usize));")
            w(u"            pv.2 = i + 1;")
            w(u"            if ptr > 0x1000 && len <= cap && cap < (1 << 20) {")
            w(u"                if used + len * esz > 32768 { pv.3 = false; break; }")
            w(u"                if len > 0 { core::ptr::copy_nonoverlapping(ptr as *const u8, pv.0.as_mut_ptr().add(used), len * esz); }")
            w(u"                pv.1[i] = (cap, len, esz); used += len * esz;")
            w(u"            } else { pv.1[i] = (0, 0, esz); }   // 비정상 삼중항 = 치환 안 함")
            w(u"        }")
            w(u"    } });")
        for j in r["rng"]:
            w(u"    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).")
            w(u"    let mut r%d = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a%d, r%d.as_mut_ptr(), 320); }" % (j, j, j))
        for (j, nb) in ARG_SNAP.get(r["idx"], []):
            w(u"    // ★간접 전달 인자 a%d(%dB) — IR 계약이 `dead_on_return` 이라 **피호출이 훼손해도 된다**." % (j, nb))
            w(u"    //   게임 호출이 먼저 훼손하면 내 사본이 다른 입력을 받는다 ⟹ StdRng 과 같은 이유로 되돌린다.")
            w(u"    let mut q%d = [0u8; %d]; if t { core::ptr::copy_nonoverlapping(a%d, q%d.as_mut_ptr(), %d); }" % (j, nb, j, j, nb))
        w(u"    let g = f(%s);" % call)
        w(u"    if !t { pop(%d); return g; }" % k)
        if r.get("selfr"):
            w(u"    // ★내 Vec 사본 버퍼에 안 들어간 호출은 **표본에서 뺀다** — 예전엔 `len=0` 으로")
            w(u"    //   떨어뜨려 **입력이 어긋난 채 비교**했고 그게 거짓 DIFF 였다(2026-09-12 차단).")
            w(u"    if !VB%d.with(|c| (&*c.get()).2) { S[%d].skip.fetch_add(1, Ordering::Relaxed); pop(%d); return g; }" % (k, k, k))
        w(u"    let n = S[%d].cmp.fetch_add(1, Ordering::Relaxed) + 1;" % k)
        for j in r["rng"]:
            w(u"    let mut p%d = [0u8; 320]; core::ptr::copy_nonoverlapping(a%d, p%d.as_mut_ptr(), 320); // 게임 호출 후 상태" % (j, j, j))
            w(u"    core::ptr::copy_nonoverlapping(r%d.as_ptr(), a%d as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로" % (j, j))
        for (j, nb) in ARG_SNAP.get(r["idx"], []):
            w(u"    let mut s%d = [0u8; %d]; core::ptr::copy_nonoverlapping(a%d, s%d.as_mut_ptr(), %d); // 게임 호출 후" % (j, nb, j, j, nb))
            w(u"    core::ptr::copy_nonoverlapping(q%d.as_ptr(), a%d as *mut u8, %d);                    // 내 사본 호출 전 = 호출 전 상태로" % (j, j, nb))
        sr2 = r.get("selfr")
        if sr2:
            ak, sz, vecs, esz = sr2
            w(u"    SP%d.with(|c| { let sp = &mut *c.get();" % k)
            w(u"        core::ptr::copy_nonoverlapping(a%d, sp.as_mut_ptr(), %d); });   // 게임 호출 후 상태" % (ak, sz))
            w(u"    SV%d.with(|c| { let sv = &*c.get();" % k)
            w(u"        core::ptr::copy_nonoverlapping(sv.as_ptr(), a%d as *mut u8, %d); }); // 내 사본 호출 전 = 호출 전 상태로" % (ak, sz))
            for (co, po, lo) in vecs:
                w(u"    // ★Vec 을 **빈 것**으로 — cap=0 이면 push 가 realloc 이 아니라 alloc 을 하므로")
                w(u"    //   **게임의 버퍼를 절대 해제하지 않는다**(순서는 IR 실측: cap@%#x · ptr@%#x · len@%#x)." % (co, po, lo))
                w(u"    VB%d.with(|c| { let vb = &*c.get();" % k)
                w(u"        core::ptr::write_unaligned((a%d as usize + %#x) as *mut usize, %d);  // cap = 내 버퍼 용량" % (ak, co, 12288 // max(esz,1)))
                w(u"        core::ptr::write_unaligned((a%d as usize + %#x) as *mut usize, vb.0.as_ptr() as usize); // ptr = 내 버퍼" % (ak, po))
                w(u"        core::ptr::write_unaligned((a%d as usize + %#x) as *mut usize, vb.1);  // len = 게임과 **같은** 개수" % (ak, lo))
                w(u"    });")
        hs = heap_specs(r["idx"]) if r.get("selfr") else []
        if hs:
            ak = r["selfr"][0]
            def _early_return():
                # ★조기 반환도 **게임 호출 후 상태**를 남겨야 한다 — self(SP) 뿐 아니라 RNG·간접인자도.
                w(u"        S[%d].skip.fetch_add(1, Ordering::Relaxed);" % k)
                w(u"        SP%d.with(|c| { let sp = &*c.get(); core::ptr::copy_nonoverlapping(sp.as_ptr(), a%d as *mut u8, %d); });" % (k, ak, r["selfr"][1]))
                for j in r["rng"]:
                    w(u"        core::ptr::copy_nonoverlapping(p%d.as_ptr(), a%d as *mut u8, 320);   // RNG = 게임 호출 후" % (j, j))
                for (j, nb) in ARG_SNAP.get(r["idx"], []):
                    w(u"        core::ptr::copy_nonoverlapping(s%d.as_ptr(), a%d as *mut u8, %d);" % (j, j, nb))
                w(u"        pop(%d); return g;" % k)
            w(u"    // ★★★힙 인식 스냅샷 — self 소유 Vec 들을 **내 힙 할당**으로 바꿔치기(내 사본의 drop/realloc 이 내 것에만 닿게).")
        for si, (boff, tags, vecs) in enumerate(hs):
            w(u"    let ptag_%d: u64 = %s;" % (si, (u"core::ptr::read_unaligned((a%d as usize + %#x) as *const u64)" % (ak, boff)) if tags else u"0"))
            if tags and None not in vecs:
                w(u"    if !(%d..=%d).contains(&ptag_%d) {   // untagged variant — 소유 필드를 모른다 ⟹ 표본 제외" % (tags[0], tags[1], si))
                _early_return()
                w(u"    }")
            w(u"    if !PV%d_%d.with(|c| (&*c.get()).3) {   // 스크래치에 안 들어갔다 = 표본 제외" % (k, si))
            _early_return()
            w(u"    }")
            w(u"    PV%d_%d.with(|c| { let pv = &*c.get(); let mut used = 0usize;" % (k, si))
            w(u"        for (i, &(off, esz, _)) in hs_vecs_%d_%d(ptag_%d).iter().enumerate() {" % (r["idx"], si, si))
            w(u"            if i >= pv.2 { break; }")
            w(u"            let (cap, len, _) = pv.1[i];")
            w(u"            let b = a%d as usize + %#x + off;" % (ak, boff))
            w(u"            if cap > 0 && len <= cap {")
            w(u"                // 여유를 둔다 — 내 사본이 push 해도 realloc 없이 들어가게(realloc 도 합법이지만 덜 흔들리게)")
            w(u"                let ncap = cap.max(len + 64);")
            w(u"                if let Ok(l) = std::alloc::Layout::from_size_align(ncap * esz, 8) {")
            w(u"                    let blk = std::alloc::alloc(l);")
            w(u"                    if !blk.is_null() {")
            w(u"                        if len > 0 { core::ptr::copy_nonoverlapping(pv.0.as_ptr().add(used), blk, len * esz); }   // ★게임 호출 전 내용")
            w(u"                        core::ptr::write_unaligned(b as *mut usize, ncap);")
            w(u"                        core::ptr::write_unaligned((b + 8) as *mut usize, blk as usize);")
            w(u"                        core::ptr::write_unaligned((b + 16) as *mut usize, len);")
            w(u"                    }")
            w(u"                }")
            w(u"                used += len * esz;")
            w(u"            } else if cap == 0 {")
            w(u"                // 빈 Vec(cap 0) — 게임 것도 댕글링이라 그대로 둬도 free 는 안 나지만, push 가 alloc 을 부르면")
            w(u"                // 그 결과는 내 것이다(아래 해제가 처리). 그대로 둔다.")
            w(u"            }")
            w(u"        }")
            w(u"    });")
        sn = r.get("sret_n") or 0
        if sn:
            # ★sret — 게임은 **호출자 버퍼**(a0)에 이미 썼다. 내 사본은 **스크래치**에 쓰게 해서
            #   게임 상태를 건드리지 않고 N 바이트를 대조한다. u64 배열 = 8바이트 정렬 보장.
            w(u"    let mut gb = [0u64; %d]; core::ptr::copy_nonoverlapping(a0, gb.as_mut_ptr() as *mut u8, %d); // 게임 출력 사본"
              % ((sn + 7) // 8, sn))
            w(u"    let mut mb = [0u64; %d];                                                   // 내 사본 전용 출력 버퍼"
              % ((sn + 7) // 8,))
            mcall = ", ".join(["mb.as_mut_ptr() as *const u8"] + ["a%d" % j for j in range(1, len(tys))])
            w(u"    let m = catch_unwind(AssertUnwindSafe(|| my_%d(%s)));" % (r["idx"], mcall))
        else:
            if BISECT_SKIP_MY and r["idx"] == 12:
                w(u"    let m: Result<(), Box<dyn std::any::Any + Send>> = Ok(());   // ★이분: 내 사본 호출 건너뜀")
            else:
                w(u"    let m = catch_unwind(AssertUnwindSafe(|| my_%d(%s)));" % (r["idx"], mycall))
        sdf = self_diff_of(r["idx"], r["name"]) if r.get("selfr") else None
        hs = heap_specs(r["idx"]) if r.get("selfr") else []
        if sdf:
            ak, sz, vecs, esz = r["selfr"]
            has_vec = bool(vecs)
            co, po, lo = vecs[0] if has_vec else (None, None, None)
            skips = sdf.get("skip") or []
            if r["rty"] == "()":
                w(u"    // ★★★상태 diff — 이 함수는 **반환이 void** 다. ABI 상 반환이 없는 것이지")
                w(u"    //   **출력이 없는 게 아니다** — 출력은 `&mut self` 에 있다 ⟹ 그걸 비교한다.")
            else:
                w(u"    // ★★★상태 diff — 이 함수의 **반환값은 exe 에 실재하지 않는다**(호출부가 결과를")
                w(u"    //   안 만져서 `AL` 이 미정규화 상태로 남는다) ⟹ 판정은 **self 부작용**으로 한다.")
            w(u"    //   ⚠되돌리기·해제 **전에** 떠서 비교한다 — 내 사본이 재할당했으면 해제 후엔 못 읽는다.")
            if has_vec:
                w(u"    let (m_ptr, m_len) = (core::ptr::read_unaligned((a%d as usize + %#x) as *const usize)," % (ak, po))
                w(u"                          core::ptr::read_unaligned((a%d as usize + %#x) as *const usize));" % (ak, lo))
            w(u"    SQ%d.with(|c| { let sq = &mut *c.get();" % k)
            w(u"        core::ptr::copy_nonoverlapping(a%d, sq.as_mut_ptr(), %d); });" % (ak, sz))
            w(u"    let sd: Option<String> = SP%d.with(|c| { let sp = &*c.get(); SQ%d.with(|c2| { let sq = &*c2.get();" % (k, k))
            w(u"        // ① 본체 바이트(설계상 다른 구간은 제외)")
            if hs:
                w(u"        // ★소유 Vec 삼중항(cap/ptr/len 24B)은 **동적 skip**(게임 것 vs 내 할당) — len·내용은 ②′에서 비교")
                for si, (boff, tags, vecs) in enumerate(hs):
                    w(u"        let gtag_%d: u64 = %s;" % (si, (u"core::ptr::read_unaligned(sp.as_ptr().add(%#x) as *const u64)" % boff) if tags else u"0"))
                    w(u"        let dyn_%d: &[(usize, usize, u8)] = hs_vecs_%d_%d(gtag_%d);" % (si, r["idx"], si, si))
            w(u"        for off in 0..%dusize {" % sz)
            for (so, sl) in skips:
                w(u"            if off >= %#x && off < %#x { continue; }" % (so, so + sl))
            for si, (boff, tags, vecs) in enumerate(hs):
                w(u"            if dyn_%d.iter().any(|&(o, _, _)| off >= %#x + o && off < %#x + o + 24) { continue; }" % (si, boff, boff))
            w(u"            if sp[off] != sq[off] {")
            w(u"                return Some(format!(\"self+{:#x}: g={:02x} m={:02x}\", off, sp[off], sq[off]));")
            w(u"            }")
            w(u"        }")
            for ei, (eoff, ety) in enumerate(ENUM_LIVE.get(r["idx"], [])):
                w(u"        // ①′ 열거형 필드 self+%#x `%s` 페이로드 — 타입 기반 live 맵(structlive)으로 variant 조건부 비교" % (eoff, ety.split("::")[-1]))
                w(u"        { let t = core::ptr::read_unaligned(sp.as_ptr().add(%#x) as *const u64);" % eoff)
                w(u"          if let Some(d) = enumlive_cmp_%d_%d(t, sp.as_ptr() as usize + %#x, sq.as_ptr() as usize + %#x) {" % (r["idx"], ei, eoff, eoff))
                w(u"              return Some(format!(\"self+{:#x}(tag {}){}\", %#x, t, d)); } }" % eoff)
            for si, (boff, tags, vecs) in enumerate(hs):
                w(u"        // ②′ 명세 %d 소유 Vec 의 len·내용 — 요소는 ELEM_LIVE(live id)로 살아있는 바이트만" % si)
                w(u"        for &(o, esz, lid) in dyn_%d {" % si)
                w(u"            let (gb, mb) = (sp.as_ptr().add(%#x + o), sq.as_ptr().add(%#x + o));" % (boff, boff))
                w(u"            // ★`Option<Vec>` 은 cap 을 니치로 쓴다(상위비트 = None). 그 경우 len/ptr 은 미초기화 — 읽지 않는다.")
                w(u"            let (gc, mc) = (core::ptr::read_unaligned(gb as *const usize), core::ptr::read_unaligned(mb as *const usize));")
                w(u"            let (gn, mn) = (gc >> 63 != 0, mc >> 63 != 0);")
                w(u"            if gn != mn { return Some(format!(\"self+{:#x}.opt: g={} m={}\", %#x + o, if gn { \"None\" } else { \"Some\" }, if mn { \"None\" } else { \"Some\" })); }" % boff)
                w(u"            if gn { continue; }")
                w(u"            let (gp, gl) = (core::ptr::read_unaligned(gb.add(8) as *const usize), core::ptr::read_unaligned(gb.add(16) as *const usize));")
                w(u"            let (mp, ml) = (core::ptr::read_unaligned(mb.add(8) as *const usize), core::ptr::read_unaligned(mb.add(16) as *const usize));")
                w(u"            if gl != ml { return Some(format!(\"self+{:#x}.len: g={} m={}\", %#x + o, gl, ml)); }" % boff)
                w(u"            if gl > 0 && gl < 4096 && gp > 0x1000 && mp > 0x1000 && gp != mp {")
                w(u"                for e in 0..gl {")
                w(u"                    if let Some(d) = elem_cmp(lid, esz, gp + e * esz, mp + e * esz) {")
                w(u"                        return Some(format!(\"self+{:#x}[{}]{}\", %#x + o, e, d));" % boff)
                w(u"                    }")
                w(u"                }")
                w(u"            }")
                w(u"        }")
            if has_vec:
                w(u"        // ② Vec 의 len (cap/ptr 은 버퍼가 달라 비교 대상이 아니다)")
                w(u"        let g_ptr = core::ptr::read_unaligned(sp.as_ptr().add(%#x) as *const usize);" % po)
                w(u"        let g_len = core::ptr::read_unaligned(sp.as_ptr().add(%#x) as *const usize);" % lo)
                w(u"        if g_len != m_len {")
                w(u"            return Some(format!(\"vec.len: g={} m={}\", g_len, m_len));")
                w(u"        }")
                el = sdf.get("elem_live")
                w(u"        // ③ Vec 의 **내용** (요소 %dB)" % esz)
                if el:
                    w(u"        //   ★요소는 열거형이라 **variant 별로 쓰는 칸이 다르다** — 안 쓰는 칸은")
                    w(u"        //   **재사용된 버퍼의 잔재**라 양쪽이 다른 게 정상이다(IR 실측 기반 live 범위만 본다).")
                    w(u"        //   (off, len, 이 tag 들에서만 live · 빈 것 = 항상)")
                    w(u"        const EL: &[(usize, usize, &[u8])] = &[%s];"
                      % ", ".join("(%d, %d, &[%s])" % (o, l, ", ".join("0x%02x" % t for t in tg))
                                  for (o, l, tg) in el))
                w(u"        if g_len > 0 && g_len < 4096 && g_ptr > 0x1000 && m_ptr > 0x1000 {")
                w(u"            for e in 0..g_len {")
                w(u"                let (gb, mb) = (g_ptr + e * %d, m_ptr + e * %d);" % (esz, esz))
                if el:
                    w(u"                let tag = *(gb as *const u8);")
                    w(u"                for &(o, l, tg) in EL {")
                    w(u"                    if !tg.is_empty() && !tg.contains(&tag) { continue; }")
                    w(u"                    for j in o..o + l {")
                    w(u"                        let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));")
                    w(u"                        if gv != mv {")
                    w(u"                            return Some(format!(\"vec[{}](tag {:#x})+{}: g={:02x} m={:02x}\", e, tag, j, gv, mv));")
                    w(u"                        }")
                    w(u"                    }")
                    w(u"                }")
                es = sdf.get("elem_str")
                if es:
                    w(u"                // ★★요소 안 `String` 은 **힙 포인터라 바이트 비교가 성립하지 않는다**")
                    w(u"                //   (게임은 exe 쪽, 내 사본은 내 DLL 쪽 할당) ⟹ **len + 내용**을 본다.")
                    w(u"                //   String 배치 = `Vec<u8>` 과 같다: cap@+0 · ptr@+8 · len@+16(IR 실측 정본).")
                    for (so, tg) in es:
                        w(u"                if %s {" % ("true" if not tg else
                                                        " || ".join("tag == %#04x" % t for t in tg)))
                        w(u"                    let (gp, gl) = (*((gb + %#x + 8) as *const usize), *((gb + %#x + 16) as *const usize));" % (so, so))
                        w(u"                    let (mp, ml) = (*((mb + %#x + 8) as *const usize), *((mb + %#x + 16) as *const usize));" % (so, so))
                        w(u"                    if gl != ml {")
                        w(u"                        return Some(format!(\"vec[{}]+{:#x}.len: g={} m={}\", e, %#x, gl, ml));" % so)
                        w(u"                    }")
                        w(u"                    if gl > 0 && gl < 65536 && gp > 0x1000 && mp > 0x1000 && gp != mp {")
                        w(u"                        for j in 0..gl {")
                        w(u"                            let (gv, mv) = (*((gp + j) as *const u8), *((mp + j) as *const u8));")
                        w(u"                            if gv != mv {")
                        w(u"                                return Some(format!(\"vec[{}]+{:#x}.str[{}]: g={:02x} m={:02x}\", e, %#x, j, gv, mv));" % so)
                        w(u"                            }")
                        w(u"                        }")
                        w(u"                    }")
                        w(u"                }")
                elif not el:
                    # ★09-13 정정: 옛 `else` 는 `if es:` 의 짝이라 elem_live 가 있어도 **전 바이트 루프가 한 번 더** 붙었다
                    #   (#54 TeamPlan::update 가 live 에서 뺀 +16 잔재로 계속 갈림). #14/#18 은 그 엄격 비교를 통과했으니 판정 유효.
                    w(u"                for j in 0..%d {" % esz)
                    w(u"                    let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));")
                    w(u"                    if gv != mv {")
                    w(u"                        return Some(format!(\"vec[{}]+{}: g={:02x} m={:02x}\", e, j, gv, mv));")
                    w(u"                    }")
                    w(u"                }")
                w(u"            }")
                w(u"        }")
            w(u"        None")
            w(u"    }) });")
            if es:
                w(u"    // ★내 사본이 **새로 push 한** 요소의 `String` 을 해제한다(내 DLL 의 할당이다).")
                w(u"    //   ⚠`pre` 미만 요소는 **게임 버퍼의 바이트 복사본**이라 그 ptr 은 **게임 소유**다 — 절대 해제 금지.")
                w(u"    //   String 의 할당 = `cap` 바이트 · align 1.")
                w(u"    {")
                w(u"        let pre = VB%d.with(|c| (&*c.get()).1);" % k)
                w(u"        if m_ptr > 0x1000 && m_len > pre && m_len < 4096 {")
                w(u"            for e in pre..m_len {")
                w(u"                let mb = m_ptr + e * %d;" % esz)
                w(u"                let tag = *(mb as *const u8);")
                for (so, tg) in es:
                    w(u"                if %s {" % ("true" if not tg else
                                                    " || ".join("tag == %#04x" % t for t in tg)))
                    w(u"                    let (c, p) = (*((mb + %#x) as *const usize), *((mb + %#x + 8) as *const usize));" % (so, so))
                    w(u"                    if c > 0 && c < (1 << 24) && p > 0x1000 {")
                    w(u"                        if let Ok(l) = std::alloc::Layout::from_size_align(c, 1) {")
                    w(u"                            std::alloc::dealloc(p as *mut u8, l);")
                    w(u"                        }")
                    w(u"                    }")
                    w(u"                }")
                w(u"            }")
                w(u"        }")
                w(u"    }")
        if hs:
            ak = r["selfr"][0]
            w(u"    // ★내 사본이 남긴 소유 Vec 을 해제한다 — 이 시점에 그 포인터는 **전부 내 것**이다")
            w(u"    //   (게임 것은 위에서 내 할당으로 바꿔치기됐고, 새로 만든 것은 내 사본이 할당했다).")
            w(u"    //   ⚠먼저 **내 사본이 새로 push 한 요소**의 String(ELEM_LIVE.str)을 해제한다 — pre-call len 미만은 게임 버퍼의 복사본.")
        for si, (boff, tags, vecs) in enumerate(hs):
            w(u"    { let t2: u64 = %s;" % ((u"core::ptr::read_unaligned((a%d as usize + %#x) as *const u64)" % (ak, boff)) if tags else u"0"))
            w(u"      PV%d_%d.with(|c| { let pv = &*c.get();" % (k, si))
            w(u"      for (i, &(off, esz, lid)) in hs_vecs_%d_%d(t2).iter().enumerate() {" % (r["idx"], si))
            w(u"          let b = a%d as usize + %#x + off;" % (ak, boff))
            w(u"          let (cap, ptr, len) = (core::ptr::read_unaligned(b as *const usize), core::ptr::read_unaligned((b + 8) as *const usize),")
            w(u"                                 core::ptr::read_unaligned((b + 16) as *const usize));")
            w(u"          if cap > 0 && cap < (1 << 20) && ptr > 0x1000 {")
            w(u"              let pre = if i < pv.2 { pv.1[i].1 } else { 0 };")
            w(u"              if BISECT_NO_STRFREE == 0 && lid != 0 && len > pre && len < 4096 { for e in pre..len { elem_free_str(lid, ptr + e * esz); } }")
            w(u"              if let Ok(l) = std::alloc::Layout::from_size_align(cap * esz, 8) { std::alloc::dealloc(ptr as *mut u8, l); }")
            w(u"          }")
            w(u"      } }); }")
        sr3 = r.get("selfr")
        if sr3:
            ak, sz, vecs, esz = sr3
            for (co, po, lo) in vecs:
                w(u"    // ★내 사본이 할당한 것을 해제한다(안 하면 호출당 누수). rlib 은 내 DLL 안에 링크돼 있어 알로케이터가 같다.")
                w(u"    { let c = core::ptr::read_unaligned((a%d as usize + %#x) as *const usize);" % (ak, co))
                w(u"      let pz = core::ptr::read_unaligned((a%d as usize + %#x) as *const usize);" % (ak, po))
                w(u"      let mine = VB%d.with(|c2| (&*c2.get()).0.as_ptr() as usize);" % k)
                w(u"      // ★내 버퍼면 해제하면 안 된다(thread_local 정적) — 재할당된 경우에만 해제.")
                w(u"      if c > 0 && pz > 0x1000 && pz != mine { if let Ok(l) = std::alloc::Layout::from_size_align(c * %d, 8) {" % esz)
                w(u"          std::alloc::dealloc(pz as *mut u8, l); } } }")
            w(u"    SP%d.with(|c| { let sp = &*c.get();" % k)
            w(u"        core::ptr::copy_nonoverlapping(sp.as_ptr(), a%d as *mut u8, %d); }); // 게임 호출 후 상태로 복구" % (ak, sz))
        for j in r["rng"]:
            w(u"    core::ptr::copy_nonoverlapping(p%d.as_ptr(), a%d as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)" % (j, j))
        for (j, nb) in ARG_SNAP.get(r["idx"], []):
            w(u"    core::ptr::copy_nonoverlapping(s%d.as_ptr(), a%d as *mut u8, %d);                    // 게임 호출 후 상태로 복구" % (j, j, nb))
        w(u"    match m {")
        if self_diff_of(r["idx"], r["name"]) and r.get("selfr"):
            # ★반환은 쓰레기라 **판정에 쓰지 않는다**. 다만 참고용으로 덤프에 남긴다.
            w(u"        Ok(_) => { if let Some(d) = sd {")
            _why = u"반환 void — 출력은 &mut self 다" if r["rty"] == "()" else u"반환은 exe 에 미실재 — 참고 g={:?}"
            if r["rty"] == "()":
                w(u"            note(%d, format!(\"#%02d %s 대조#{} **상태갈림**: {} | (%s) | %s\", n, d, %s)); } }"
                  % (k, r["idx"], r["name"], _why, " ".join(fmt), ", ".join(vals)))
            else:
                w(u"            note(%d, format!(\"#%02d %s 대조#{} **상태갈림**: {} | (%s) | %s\", n, d, g, %s)); } }"
                  % (k, r["idx"], r["name"], _why, " ".join(fmt), ", ".join(vals)))
        elif sn and r.get("sret_vec"):
            sv = r["sret_vec"]
            el = ", ".join("(%d, %d, &[%s])" % (o, l, ", ".join("0x%02x" % t for t in tg)) for (o, l, tg) in sv["elem_live"])
            w(u"        Ok(_) => { let (gl, ml) = (gb[%d] as usize, mb[%d] as usize); let (gp, mp) = (gb[%d] as usize, mb[%d] as usize);"
              % (sv["len"] // 8, sv["len"] // 8, sv["ptr"] // 8, sv["ptr"] // 8))
            w(u"            const EL: &[(usize, usize, &[u8])] = &[%s];" % el)
            w(u"            let d: Option<String> = if gl != ml { Some(format!(\"len g={} m={}\", gl, ml)) }")
            w(u"                else if gl > 0 && gl < 4096 && gp > 0x1000 && mp > 0x1000 { (|| { for e in 0..gl { let (eb, fb) = (gp + e * %d, mp + e * %d); let tag = *(eb as *const u8);"
              % (sv["esz"], sv["esz"]))
            w(u"                    for &(o, l, tg) in EL { if !tg.is_empty() && !tg.contains(&tag) { continue; } for j in o..o + l { let (gv, mv) = (*((eb + j) as *const u8), *((fb + j) as *const u8)); if gv != mv { return Some(format!(\"vec[{}]+{}: g={:02x} m={:02x}\", e, j, gv, mv)); } } } } None })() } else { None };")
            w(u"            if let Some(d) = d { note(%d, format!(\"#%02d %s 대조#{} 갈림(sret bump Vec len={}): {} | %s\", n, gl, d, %s)); } }"
              % (k, r["idx"], r["name"], " ".join(fmt[1:]), ", ".join(vals[1:])))
        elif sn and r.get("sret_enum"):
            # ★sret 열거형 — 태그(8B@0) 가 다르면 갈림, 같으면 variant 조건부 페이로드(enumlive · structlive 자동 생성)만 비교.
            w(u"        Ok(_) => { let (gt, mt) = (gb[0], mb[0]);")
            w(u"            let d = if gt != mt { Some(format!(\"tag g={} m={}\", gt, mt)) } else { enumlive_cmp_%d_0(gt, gb.as_ptr() as usize, mb.as_ptr() as usize).map(|x| format!(\"(tag {}){}\", gt, x)) };"
              % r["idx"])
            w(u"            if let Some(d) = d { note(%d, format!(\"#%02d %s 대조#{} 갈림(sret 열거형 %dB): {} | g={:02x?} m={:02x?} | %s\", n, d, &gb[..%d], &mb[..%d], %s)); } }"
              % (k, r["idx"], r["name"], sn, " ".join(fmt[1:]), (sn + 7) // 8, (sn + 7) // 8, ", ".join(vals[1:])))
        elif sn:
            # ★전 바이트가 아니라 **살아있는 구간만** 비교한다. 갈린 구간의 오프셋을 함께 남긴다.
            w(u"        Ok(_) => { if let Some(off) = live_eq(a0, mb.as_ptr() as *const u8, LIVE_%d) {"
              % r["idx"])
            w(u"            note(%d, format!(\"#%02d %s 대조#{} 갈림(sret %dB · +{:#x}): g={:02x?} m={:02x?} | %s\", n, off, &gb[..%d], &mb[..%d], %s)); } }"
              % (k, r["idx"], r["name"], sn,
                 " ".join(fmt[1:]), (sn + 7) // 8, (sn + 7) // 8, ", ".join(vals[1:])))
        elif r["rty"] in ("P8", "P64") and RET_LIVE.get(r["idx"]) is not None:
            # ★페어 반환 — 두 번째 칸은 **판별자에 따라 죽는다**(위 RET_LIVE 주석).
            arms = u" | ".join(str(t) for t in RET_LIVE[r["idx"]])
            w(u"        Ok(m) => { let bad = m.a != g.a || (matches!(g.a, %s) && m.b != g.b);" % arms)
            w(u"            if bad { note(%d, format!(\"#%02d %s 대조#{} 갈림: g={:?} m={:?} | %s\", n, g, m, %s)); } }"
              % (k, r["idx"], r["name"], " ".join(fmt), ", ".join(vals)))
        elif r["rty"] == "U24":
            # ★i24: 상위 8비트는 ABI 상 미정의 — 하위 24비트만 대조한다.
            w(u"        Ok(m) => { if (m & 0xffffff) != (g & 0xffffff) { note(%d, format!(\"#%02d %s 대조#{} 갈림(i24): g={:#x} m={:#x} | %s\", n, g & 0xffffff, m & 0xffffff, %s)); } }"
              % (k, r["idx"], r["name"], " ".join(fmt), ", ".join(vals)))
        else:
            w(u"        Ok(m) => { if m != g { note(%d, format!(\"#%02d %s 대조#{} 갈림: g={:?} m={:?} | %s\", n, g, m, %s)); } }"
              % (k, r["idx"], r["name"], " ".join(fmt), ", ".join(vals)))
        w(u"        Err(_) => { S[%d].pan.fetch_add(1, Ordering::Relaxed); }" % k)
        w(u"    }")
        w(u"    pop(%d);" % k)
        w(u"    g")
        w(u"}")
    w(u"")
    w(u"/// 이 명세 idx 가 sweep 으로 **설치돼 있나**(= 진입부 프로브를 걸면 안 되나).")
    w(u"pub fn is_installed_spec(idx: u8) -> bool {")
    w(u"    S.iter().any(|s| s.idx == idx && s.orig.load(Ordering::Relaxed) != 0)")
    w(u"}")
    w(u"pub fn any_installed() -> bool { S.iter().any(|s| s.orig.load(Ordering::Relaxed) != 0) }")
    w(u"")
    w(u"/// sweep 설치. `mask` 비트 k = `S[k]`. 반환 = (성공, 시도).")
    w(u"/// ⚠`orig` 는 **진입부 패치 전에** 저장된다(`hookw` 가 그 순서를 보장) — 패치 직후 다른 스레드가")
    w(u"///   들어와 `orig==0` 을 transmute 하면 널 호출이다(배경 sim 워커가 있으니 실재하는 경합).")
    w(u"pub unsafe fn install(mask: u64, log: &mut String) -> (usize, usize) {")
    w(u"    if mask == 0 {")
    w(u"        log.push_str(\"[sweep] 게이트 OFF (sweep20_on.txt 없음/0) — 한 곳도 안 걸었다\\n\");")
    w(u"        return (0, 0);")
    w(u"    }")
    w(u"    let unknown = mask & !((1u64 << S.len()) - 1);")
    w(u"    if unknown != 0 {")
    w(u"        // 「빠진 것을 모르는 상태」를 만들지 않는다 — 슬롯이 없는 비트를 켜면 조용히 무시되는 게 아니라 말한다.")
    w(u"        log.push_str(&format!(\"[sweep] ⚠mask 의 미지 비트 {:#x} 는 슬롯이 없어 무시했다(슬롯 {}개)\\n\", unknown, S.len()));")
    w(u"    }")
    w(u"    let w: [usize; %d] = [%s];" % (N, ", ".join("w_%d as usize" % r["idx"] for r in rows)))
    w(u"    let (mut ok, mut tried) = (0usize, 0usize);")
    w(u"    for i in 0..S.len() {")
    w(u"        if mask & (1u64 << i) == 0 { continue; }")
    w(u"        tried += 1;")
    # ★사이트가 있으면 **호출부 리다이렉트**로 건다(진입부 12B 를 못 빼는 함수). 원 함수 명령은 무손상.
    w(u"        let r = if S[i].sites.is_empty() {")
    w(u"            crate::hookw::install_wrap(S[i].rva, S[i].prolog, w[i], &S[i].orig)")
    w(u"        } else {")
    w(u"            crate::probe::install_cs_wrap(S[i].rva, S[i].sites, w[i], &S[i].orig)")
    w(u"        };")
    w(u"        match r {")
    w(u"            Ok(o) => { ok += 1; log.push_str(&format!(\"[sweep] bit{} #{:02} {} OK @{:#x} {} {:#x}{}\\n\", i, S[i].idx, S[i].name, S[i].rva,")
    w(u"                if S[i].sites.is_empty() { \"트램폴린\" } else { \"호출부스텁\" }, o,")
    w(u"                if S[i].sites.is_empty() { String::new() } else { format!(\" (사이트 {}곳 — 원 함수 명령 무손상)\", S[i].sites.len()) })); }")
    w(u"            Err(e) => log.push_str(&format!(\"[sweep] bit{} #{:02} {} 실패: {} @{:#x}\\n\", i, S[i].idx, S[i].name, e, S[i].rva)),")
    w(u"        }")
    w(u"    }")
    w(u"    (ok, tried)")
    w(u"}")
    w(u"")
    w(u"/// 「판 종료」 확정 직후 호출 — 다음 판의 델타 기준선.")
    w(u"pub fn mark_base() {")
    w(u"    for s in S.iter() {")
    w(u"        s.base_cmp.store(s.cmp.load(Ordering::Relaxed), Ordering::Relaxed);")
    w(u"        s.base_diff.store(s.diff.load(Ordering::Relaxed), Ordering::Relaxed);")
    w(u"    }")
    w(u"}")
    w(u"pub fn total_cmp() -> u64 { S.iter().map(|s| s.cmp.load(Ordering::Relaxed)).sum() }")
    w(u"")
    w(u"pub fn report(header: &str, gate: &str, inst: &str) -> String {")
    w(u"    let mut s = String::new();")
    w(u"    s.push_str(\"=== tfm2_judge_verify 2단계 · sweep (게임 원본 vs 내 링크사본 비트동일 대조) ===\\n\");")
    w(u"    s.push_str(header); s.push_str(\"\\n\\n\");")
    w(u"    s.push_str(&format!(\"--- 게이트: {}\\n--- 설치: {}\\n\\n\", gate, inst));")
    w(u"    s.push_str(\"--- 대조표 (DIFF=0 은 **표본 수와 함께** 읽어라 — 대조수 0 이면 판정 불성립)\\n\");")
    w(u"    s.push_str(\"    bit  idx  종류      호출수         대조수        DIFF     panic     skip   이번판델타(대조/DIFF)  name\\n\");")
    w(u"    for x in S.iter() {")
    w(u"        let kind = if x.orig.load(Ordering::Relaxed) != 0 { \"[sweep]\" } else { \"[미설치]\" };")
    w(u"        let (c, n, dd, pa, sk) = (x.calls.load(Ordering::Relaxed), x.cmp.load(Ordering::Relaxed),")
    w(u"                              x.diff.load(Ordering::Relaxed), x.pan.load(Ordering::Relaxed), x.skip.load(Ordering::Relaxed));")
    w(u"        s.push_str(&format!(\"   {:>3} {:>4}  {:<9} {:>12} {:>12} {:>10} {:>8} {:>8}   {:>10}/{:<8} {}\\n\",")
    w(u"            x.bit, x.idx, kind, c, n, dd, pa, sk,")
    w(u"            n.saturating_sub(x.base_cmp.load(Ordering::Relaxed)),")
    w(u"            dd.saturating_sub(x.base_diff.load(Ordering::Relaxed)), x.name));")
    w(u"    }")
    w(u"    s.push_str(\"    (호출수 = 진입 전부 · 대조수 = 최상위 진입만 = 실제 표본 수 · panic = 내 사본이 패닉한 횟수 · skip = 버퍼/변종 사유로 **표본에서 뺀** 횟수)\\n\");")
    w(u"    for x in S.iter() { if !x.caveat.is_empty() { s.push_str(&format!(\"    ⚠#{:02} {} caveat: {}\\n\", x.idx, x.name, x.caveat)); } }")
    w(u"    s.push_str(\"    1단계 발화수 대조: \");")
    # ★`stage1 == u64::MAX` = **무효 표식**. 숫자 0 으로 두면 「미발화」로 오독된다 —
    #   그게 이번 사고(틀린 주소의 측정치를 사실로 인용)의 재발 경로다.
    w(u"    for x in S.iter() { if x.stage1 == u64::MAX "
      u"{ s.push_str(&format!(\"#{:02} 무효(옛주소) \", x.idx)); } "
      u"else { s.push_str(&format!(\"#{:02} {} \", x.idx, x.stage1)); } }")
    w(u"    s.push_str(\"\\n    ⛔「무효(옛주소)」= 1단계를 **다른 함수**에서 쟀다 "
      u"— 정정된 주소로 1단계를 다시 돌려야 값이 생긴다.\");")
    w(u"    s.push_str(\"\\n\\n\");")
    w(u"    let g = FIRST.lock().unwrap_or_else(|e| e.into_inner());")
    w(u"    s.push_str(&format!(\"--- 첫 DIFF 덤프 {}건 (슬롯당 1건 · 상한 {})\\n\", g.len(), FIRST_MAX));")
    w(u"    if g.is_empty() { s.push_str(\"    (없음)\\n\"); }")
    w(u"    for (_, l) in g.iter() { s.push_str(\"    \"); s.push_str(l); s.push('\\n'); }")
    eh = [(r["idx"], ei, eoff, ety) for r in rows if r.get("selfr") for ei, (eoff, ety) in enumerate(ENUM_LIVE.get(r["idx"], []))]
    eh += [(slot, ei, eoff, ety) for slot, lst in sorted(PIN_ENUM_LIVE.items()) for ei, (eoff, ety) in enumerate(lst)]
    eh += [(r["idx"], 0, 0, r["sret_enum"]) for r in rows if r.get("sret_enum")]   # sret 열거형 히스토그램(09-13 · 첫 판은 미출력이었다)
    if eh:
        w(u"    s.push_str(\"\\n--- ENUM_LIVE variant 히스토그램 (페이로드 정밀 비교가 실제로 어느 variant 에 적용됐나 · 대조 1건 = 1)\\n\");")
        for (idx, ei, eoff, ety) in eh:
            w(u"    { let tot: u64 = EH_%d_%d.iter().map(|a| a.load(Ordering::Relaxed)).sum();" % (idx, ei))
            w(u"      s.push_str(&format!(\"    #%02d self+%#x %s: 총 {} — \", tot));" % (idx, eoff, ety.split("::")[-1]))
            w(u"      for &(t, nm) in EH_%d_%d_NAMES.iter() { let c = EH_%d_%d[t as usize].load(Ordering::Relaxed); if c > 0 { s.push_str(&format!(\"{}({})={} \", nm, if t == 0 { \"untagged\".to_string() } else { t.to_string() }, c)); } }" % (idx, ei, idx, ei))
            w(u"      let seen: Vec<&str> = EH_%d_%d_NAMES.iter().filter(|(t, _)| EH_%d_%d[*t as usize].load(Ordering::Relaxed) == 0).map(|(_, n)| *n).collect();" % (idx, ei, idx, ei))
            w(u"      s.push_str(&format!(\"\\n        미출현 {}: {}\\n\", seen.len(), seen.join(\" \"))); }")
    w(u"    s.push_str(\"\\n--- ★대조에서 빠진 명세 함수와 사유 (1단계 프로브는 그대로 유지된다)\\n\");")
    w(u"    for (i, nm, c, why) in EXCLUDED.iter() {")
    w(u"        s.push_str(&format!(\"    #{:02} {:<34} 발화 {:>12}  ← {}\\n\", i, nm, c, why));")
    w(u"    }")
    w(u"    s.push_str(\"\\n1단계 발화수는 probe20.txt(별도 파일)에 있다 — 섞어 읽지 말 것.\\n\");")
    w(u"    s")
    w(u"}")

    io.open(OUT, "w", encoding="utf-8").write("\n".join(L) + "\n")
    print(u"rlib = %s" % rlib)
    print(u"생성 %d함수 → %s" % (N, OUT))
    for k, r in enumerate(rows):
        print(u"  bit%d = %#-5x #%02d %-34s %#-9x 인자%d %-4s 발화 %12s  StdRng=%s"
              % (k, 1 << k, r["idx"], r["name"][:34], r["rva"], len(r["args"]), r["rty"],
                 (r["cnt_s"] if r["bad_cnt"] else "{:,}".format(r["cnt"])),
                 r["rng"] or u"없음"))
    print(u"\n제외 %d개:" % len(excl))
    for i, nm, c, why in excl:
        print(u"  #%02d %-34s 발화 %12s  ← %s" % (i, nm[:34], c, why))


if __name__ == "__main__":
    main()
