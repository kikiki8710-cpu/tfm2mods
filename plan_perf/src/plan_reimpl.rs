// plan_reimpl.rs — Phase 1: 월드 접근 복원 + 검증 (행동 무변경)
// 빌드: build_mod.bat "...\plan_reimpl.rs" → plan_reimpl.dll
// 파일(mods\plan_reimpl\): plan_reimpl.cfg(설정) / plan_reimpl.txt(로그) / world.txt(검증덤프)
//
// 목표: think 안에서 plan_v2의 월드(양팀 로스터 10챔프)에 접근하고, ctx의 self와 대조해 브리지 검증.
//   - dispatch(facet#5, 0x1c08770) 후킹: rdx=plan_state+0x500 → plan_base=rdx-0x500 캡처(글로벌).
//     plan_base는 매치당 공유(상수) → 글로벌 last-seen으로 충분.
//   - 로스터: plan_base + team*0x228 + 0x1e0, 팀당 5 (*(roster+i*8)=전투엔티티, speed>0).
//   - think: CAP_PB→로스터 열거, ctx.team()/hp()와 self 매칭 확인. 행동은 그대로(검증 단계).
//   - override는 cfg(enabled, 기본 OFF)로 게이트. Phase 2부터 결정 재구현 채움.

use mod_api::*;
use std::path::PathBuf;
use std::collections::HashMap;
use std::cell::RefCell;   // ★레버4: slot_a8 프레임 thread_local 캐시
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicPtr, AtomicU8, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "plan_perf";   // ★테스트 모드: plan_reimpl 복제본(compute 0.5배 최적화 실험). plan_reimpl과 동시 활성 금지(같은 함수 후킹).
const RVA_DISPATCH: usize = 0x1b098e0;   // 0.4.13_5(was 0x1b7ab30). plan_judge_dispatch(facet#5) prologue 17B; rdx=plan_state+0x500
const RVA_RETREAT:  usize = 0x1d35820;   // 0.4.13_5(was 0x1fcfda0). retreat_engage(#5 disc4) 12B; 콜그래프 score=10. replace detour는 MIG_CHANGED로 보류
// ★plan_lane_predicate(0x2080760) = my_lane_predicate로 순수재현 완료(2026-06-19, DIFF=0) → RVA_LANE_PRED/LanePred 제거(churn 소멸).
const RVA_DRIVER:   usize = 0x1e9f280;   // 0.4.13_5(was 0x1b6ccf0). plan_think_driver 12B(push8); per-player. 콜그래프 score=4. args rcx/rdx/r8/r9, arg5=[rsp+0x28]
// facet#2 position: driver 내 FUN_141917430(generic_build, 이동좌표 최종화) 호출지점. 직후 rcx(=[RBP+0xd0])=최종 Move{tag,x,y}.
const RVA_F2_BUILD_CALL: usize = 0x1c9b93f;  // 0.4.13_5(was 0x1b6e806). CALL generic_build (E8 rel32). DRIVER 새바디서 재추출. 8인자. next=+5
const RVA_GENERIC_BUILD: usize = 0x20a4830;  // 0.4.13_5(was 0x1bf5980). 콜그래프 score=10(F80320×7). B2 로직변경, repro 재검증요
// ★facet#2 레인워크 waypoint 선택 = FUN_141dd7700. 프롤로그 12B(8 push). param_2=rdx,param_5/6/7=[entry+0x28/0x30/0x38].
// param_7=챔피언, param_6=지오메트리(*p6=L80,p6[1]=VOBJ,p6[2]=GEO), param_5=lane(side@+0x6a8,lane_kind@+0x738).
const RVA_DD7700: usize = 0x18c6490;  // 0.4.13_5(was 0x19e5e10). mask-sig 유일매치=바디동일. PassiveLinePlan::sub_plan
// ★시드 PRNG gen_range(rand-0.8.5). fcd980=roll[lo,hi](rcx=state, rdx=&{lo,hi})→rax. 프롤로그 12B 가정.
const RVA_FCD980: usize = 0x18b1550;  // 0.4.13_5(was 0x18a1c30). FCDAF0-0x170 쌍 확정
// ★facet#5 교전롤: retreat_engage 내 fcd980 호출(0x141fabfef)의 복귀주소. 이걸로 롤 호출을 식별.
const RVA_ROLL_RET: usize = 0x1d37b98;  // 0.4.13_5(was 0x1fd20ae). RETREAT 새바디 fcd980콜(0x20715ef)+5
// ★드라이버 페이즈게이트: fcdaf0=gen_range[0,1000](13B 프롤로그). 페이즈게이트 롤 호출(0x1d4f889)의 복귀주소=0x1d4f88e.
//   A=ms[0x218]→RBX, B=ms[0x238]→RDI, C=ms[0x380]→RSI (비휘발성, fcdaf0 호출 넘어 보존). rng=rcx.
const RVA_FCDAF0: usize = 0x18b16c0;  // 0.4.13_5(was 0x18a1da0). 콜그래프(F80320=FCDAF0×9)확정, fcd980+0x170
const RVA_PG_ROLL_RET: usize = 0x1c9b306;  // 0.4.13_5(was 0x1b6e1c8). DRIVER 새바디 fcdaf0콜(0x1ea07c9)+5
const RVA_TRANS: usize = 0x1af2920;   // 0.4.13_5(was 0x1b64db0). 상수지문3.14+프롤로그92%. B2 로직변경(캡처훅 휴면이라 안전). handle_interact_battle. 12B(push8). rcx=S(+0x4ce=phase,+0x500=subplan)
// ★retreat_engage(facet#5) 0.4.13 유력후보 = 0x1fb7770 (LegacyPlanHandler::get_small_action, push8+0x2e68 프레임). RVA_RETREAT/FC59A0/ROLL_RET/FA1EA0/LANE_PRED/DF0C10 콜그래프 검증 후 갱신요(MIG0413=false 전).
const RVA_FC59A0: usize = 0x1d42db0;  // 0.4.13_5(was 0x1fe3220). 상수지문2.2+프롤로그90%. B2 로직변경(캡처훅 휴면이라 안전). recall_rng_score 12B(push8). rcx=out, rdx=RNG state
const RVA_CAND_FILTER: usize = 0x1f9f7c0;  // 0.4.13_5(was 0x1f4ec60). GB의 유일 비-F80320 fcdaf0-callee+push8 프롤로그. transition RNG 후보필터. rcx=out Vec, rdx=14필드 ctx, r8=param3. fcdaf0 소비.
const RVA_E88A0: usize = 0x1e6a9c0;  // disc9/11 poke 후보선택자. 인라인 gen_range(0,count)=serpen RNG소스. r8=RNG, r9=param_4(cand리스트+0x3c8/cnt+0x3d0/thr+0x710), stack p7[4]=비교집합. 8push 프롤로그(12B).
// ★facet#5 engage draw1: FUN_1420e9a30 = 후보 gather + u64 gen_range(0,count_a) 1뽑기. rdx=RNG state(param2), r8=param3(facetcnt@+0x3d0/thr@+0x710/+0x440=K소스), arg6=ARG_CONT(gather: [+0x20]→[+8]base/[+0x10]len). 8push 프롤로그(12B). cand getter=필드읽기(vt0x60=+0x180thr/vt0x68=+0x188pri/vt0x98=+0x190 facet u32).
const RVA_E9A30: usize = 0x20e9a30;
const RVA_PREGATE: usize = 0x1d426f0;  // engage pre-gate(거리 eligibility, RNG-free). al=0이면 retreat -1(roll 전). ★my_pregate에서 순수Rust 재현(호출X).
// ★pre-gate(0x2080760) 상수 테이블(.rdata, RVA). p1(lane)<4만 사용. tableA[0..4]=[0,1,3,2](인덱스 변환), tableC/D=후보 비교좌표.
const RVA_TABLE_A: usize = 0x35eef90;  // r15 threshold 인덱스 변환표: [0,1,3,2,...]
const RVA_TABLE_C: usize = 0x35eefb0;  // q==0→x비교 / q==1→y비교 좌표표
const RVA_TABLE_D: usize = 0x35eefd0;  // q==0→y비교 / q==1→x비교 좌표표
// ── generic_build 옵션 스코어러(0x1f80320) 검증 대상 + 서브함수(oracle 호출용) ──
const RVA_F80320: usize = 0x1fe4990;   // 0.4.13_5(was 0x1f80320). 콜그래프 score=8(FCDAF0×9). RNG-가중 옵션 스코어러. 7인자. ★데미지경로 f5db30→COMBAT_FN×6+E1B330로 변경.
const RVA_DEC1F0: usize = 0x1e3e050;   // 0.4.13_5(was 0x1dec1f0) mask-sig 유일=바디동일. type3체크→role 점프 조기탈출 (char)
const RVA_DFD1E0: usize = 0x1e4ff20;   // 0.4.13_5(was 0x1dfd1e0) 바디동일. 슬롯가용 술어 +0xb8 (char)
const RVA_DEC4D0: usize = 0x1e3e330;   // 0.4.13_5(was 0x1dec4d0) 바디동일. 슬롯가용 술어 +0xc0 (char)
const RVA_DFB1A0: usize = 0x1e4dec0;   // 0.4.13_5(was 0x1dfb1a0) 바디동일. 슬롯가용 술어 +0xc8 (char)
const RVA_A1DA50: usize = 0x1a24720;   // 0.4.13_5(was 0x1a1da50) 바디동일. 아군풀 수집(param1<30 분기)
const RVA_E1B330: usize = 0x1e6fc30;   // 0.4.13_5(was 0x1e1b330) 바디동일. param1>=30 특수 스코어(F80320이 ×1 호출)
// ── 영역 D callee 검증 대상(genbuild_body_D.md): 함수시작 detour로 game retval(rax,u64) vs my_203cb30/my_20c0690 DIFF=0 ──
const RVA_GB_203CB30: usize = 0x203cb30;  // 0.4.13_5. 단일 엔티티 종합점수(3슬롯). rcx=rh, rdx=a(점수대상), r8=S → rax. prologue 8push=12B(rip-rel無, ghidra-re 2026-06-21)
const RVA_GB_20C0690: usize = 0x20c0690;  // 0.4.13_5. post 점수(1슬롯). rcx=&{[0]=rh,[8]=a,[0x10]=S}, rdx=desc → rax. prologue push7+sub=14B(rip-rel無)
const ORIG_LEN_GB_203CB30: usize = 12;    // push R15/R14/R13/R12/RSI/RDI/RBP/RBX = 정확히 12B 경계
const ORIG_LEN_GB_20C0690: usize = 14;    // push7(10B)+SUB RSP,0x40(4B) = 14B 경계(12 미달→SUB 포함)
// ── 영역 D 출력검증(gb_region_d): mid-func 캡처 detour 지점(genbuild_body_D.md "런타임 캡처 빌드") ──
const RVA_GB_REGIOND_HOOK: usize = 0x20e42a3;  // 0.4.13_5. 결정게이트 [0x108]vs[0x158] 비교점. 여기서 RegionD 입력로컬 전부 set됨.
const ORIG_LEN_GB_REGIOND: usize = 15;         // shr rbx,2[4B]/shr r14,2[4B]/mov rcx,[rbp+0x108][7B] = 0x42b2 경계. rip-rel無(ghidra-re 확정).
const RVA_GB_DEDC0: usize = 0x20dedc0;          // FUN_1420dedc0 타이밍/상태 게이트(asm 다수 call). shadow-call: rcx=out([rbp+0x290]), r8=[rbp+0x320]값(asm 0x44c4). 21 NP(out+0x40==0&&b_logic) 오라클 해결용.
const RVA_GB_EPILOGUE: usize = 0x20df5da;        // generic_build 유일 공통출구 에필로그(단일 ret 0x20df5f4 수렴; funnel 0x4a1a는 우회경로多=부적합). 100% inline 대체 hook 지점.
const ORIG_LEN_GB_EPILOGUE: usize = 15;          // movaps xmm6,[rbp+0x2a0](8B)+add rsp,0x338(7B)=15. rip-rel無, 중간진입 0건(asm확정). rbp 유효(out=[rbp+0x290]).
const RVA_GB_FUNNEL: usize = 0x20e4a1a;           // region D 공통출구(arena cleanup→에필로그). gbskip 진짜skip시 0x42a3서 여기로 jump(게임 region D 미실행). rsp는 고정프레임이라 0x42a3==funnel-entry.
// facet#1 condgate(목표커밋 bool). 프롤로그 push3+sub0x40+mov rsi = 15B 클린. rcx=subplan_ctx(*=disc), r9=reg, 스택: rh_slot@entry+0x28, r11@+0x38, rsi@+0x40
const RVA_CONDGATE: usize = 0x1b08e10;   // 0.4.13_5(was 0x1b7a070). ✅확정(condcap 4000샘플 DIFF=0, my_condgate==game). DRIVER단독호출 callee+프롤로그경계OK
// facet#4 movepriority. 프롤로그 7push+sub0x50 = 14B 클린. rcx=출력ptr(rsi), rdx=subplan ptr(*=disc), 스택: r14(sim)@entry+0x28, r15(rh)@entry+0x30
// ★0.4.13 마이그: exe↔exe 매칭으로 전 RVA 갱신완료(mig_exe2exe.py + 0.4.12백업). MIG0413=false → unchanged 캡처훅 활성.
// MIG_CHANGED=true: 0.4.13서 로직변경된 함수(retreat 리팩터/ttd/fa1ea0/generic_build/df0c10콜사이트/데미지검증) 훅은 캡처오프셋 stale 가능 → 별도 보류.
const MIG0413: bool = false;
const MIG_CHANGED: bool = true;
// ★dd7700 캡처(레인워크 재현 my_dd7700_code). 0.4.13_5 Ghidra대조: 바디동일(vtable/args/오프셋 동일, 마이그회귀 아님).
//   크래시 = engage-tail(STAGE3-6) 재현의 잠복 AV(교전tail은 0.4.12서도 미캡처=미검증이었음). 초기코드(2/4/7)는 검증됨.
//   ⟹ DD7700_CAP_OK=true(캡처 재활성) + DD7_TAIL_OK=false(tail 게이트=AV차단, tail은 -999 미예측)로 안전 캡처.
const DD7700_CAP_OK: bool = true;
const DD7_TAIL_OK: bool = true;   // ★engage-tail 재활성(2026-06-19): STAGE6 resolver/vt168 this=sim 수정(AV근본원인=rf(target)→rf(sim)). 디컴 confirm.
// MIG_DMG=true: 데미지검증 비교(think 내, combat 0x1be1e90 vs my_combat_dmg)만 선택활성. MIG_CHANGED와 분리 — 나머지 변경훅(retreat replace/move/commit, 프레임시프트 stale)은 계속 보류. combat/vtable RVA는 #1 TTD에서 라이브 확정됨. 순수 read-only 비교(행동무변경).
const MIG_DMG: bool = false;   // ★3차핫픽스: vtable(ATK/TGT/DEFAULT_AB2 2차주소) stale + TTD damage경로 변경 → 데미지검증 OFF(재RE 후 활성)
// MIG_TTD=true: TTD(0x1b6df40) 캡처만 선택활성(프롤로그 install_detour-안전 검증완료, 핫픽스=프리핫픽스 동일). my_ttd(0.4.12기준) 재검증용. df0c10콜사이트는 계속 MIG_CHANGED 보류.
const MIG_TTD: bool = false;   // ★0.4.13_5: TTD RVA tentative + 데미지경로 f5db30→COMBAT_FN×6 변경 + DEFAULT_AB2/ABILITY_TABLE 미해결 → TTD훅/my_ttd repro 비활성(Ghidra 확정+repro 재유도 후 재활성).
const INSTALL_DIAG_HOOKS: bool = false;   // ★성능(2026-06-22): 휴면 진단/캡처훅 미설치(프로덕션). RNG/판단 핫패스 트램폴린 제거=일정넘김 가속. cfg=0이라 passthrough였던 걸 미설치로=비트동일. 검증재개시 true 재빌드. KEEP(밖)=retreat/condgate/movepri/fc59a0/gbrd.
const MIG_GB_CHANGED: bool = true;   // ★0.4.14: generic_build(0x20a4830) 로직변경(유사도 0.64) → region D 내부주소(GB_REGIOND_HOOK/EPILOGUE/FUNNEL/DEDC0/203CB30/20C0690)가 전부 stale. install_detour_d_skip(무조건설치=즉사위험) 보류. 0.4.14 새바디 region D 재추출 후 false 재활성(작업#4).
const RVA_MOVEPRI: usize = 0x1b09590;   // 0.4.13_5(was 0x1b7a7e0). mask-sig 유일매치=바디동일. dispatch 바로앞 인접(+0x350)
// ★cVar6==0 STAND vs roll 게이트: fa1ea0(액션큐)≠0xff면 STAND(8), ==0xff면 교전롤.
//   ⟹ my_fa1ea0(순수재현, 288/288 DIFF0 검증)로 완전대체 → 게임콜 RVA_FA1EA0 제거(churn 소멸, 2026-06-19).
// ★ChaCha12 refill 디스패처 FUN_1421bbc10(rcx=input, rdx=6, r8=output버퍼). 프롤로그 12B(push r14/rsi/rdi/rbx+sub rsp,0x168).
const RVA_CHACHA: usize = 0x2245cf0;  // 0.4.13_5(was 0x2220f70). mask-sig 유일매치=바디동일
// 광범위 이동/행동 커밋: driver가 매프레임 champion+0x590에 최종 Input을 쓰는 단일 지점(FUN_141a49fa0).
// 0x141d50341 LEA RCX,[champ+0x590]; 0x141d5034f LEA RDX,[RBP+0x200]=최종Input; 0x141d5035d CALL. dump=rdx(Input 0x90).
const RVA_COMMIT_CALL: usize = 0x1c9bdca;     // 0.4.13_5(was 0x1b6ec93). DRIVER 새바디 CALL commit_fn (E8). rdx=&Input
const RVA_COMMIT_FN: usize = 0x1d9b140;        // 0.4.13_5(was 0x1cbc9f0). mask-sig 유일매치. commit_fn
// 페이즈 게이트 threshold = objective*9 + min(B,100)*2 + BASE(=100). driver 0x141d4f8ca: ADD EAX,0x64 (83 C0 64).
// imm8(베이스)을 패치 = 교전 공격성 다이얼. 낮추면 threshold↓ → score>=threshold 자주 → active 전환 빨라짐.
const RVA_ENGAGE_GATE: usize = 0x1c9b33d;     // 0.4.13_5(was 0x1b6e204). DRIVER 새바디 ADD EAX,0x64 (83 C0 64 유일확인); imm8 at +2
// facet#5 셀렉터(local_228) 신선포착: retreat_engage 내 df0c10 호출 직후 [rcx]=셀렉터(1=역할기반). 리턴前엔 액션코드로 덮임.
const RVA_DF0C10_CALL: usize = 0x1d35ff1;     // 0.4.13_5(was 0x1fd0503). RETREAT 새바디 CALL df0c10 (E8). rcx=&local_228, 7인자
const RVA_DF0C10_FN: usize = 0x1e6a9c0;        // 0.4.13_5(was 0x1b2eac0). mask-sig 유일매치. df0c10 getter probe
const RVA_T9360:    usize = 0x1dd9360;   // ⚠0.4.13_5 미해결(4후보 무효, Ghidra요). 현재 비활성(let _=). FUN_141dd9360 per-player AI. rdx=AI구조체(subplan@+0x1870), r9=athlete
const ROSTER_BASE: usize = 0x1e0;        // plan_base + team*0x228 + 0x1e0
const ROSTER_STRIDE: usize = 0x228;
const ROSTER_N: usize = 5;
// 전투엔티티 오프셋
const E_POSX: usize = 0x648; const E_POSY: usize = 0x650;
const E_HP: usize = 0x658;   const E_MAXHP: usize = 0x610;
const E_SPEED: usize = 0x628; const E_ALIVE: usize = 0x4a8;
const E_NAME: usize = 0x250;  // char* champion_name (null-term, 직접)
// athlete(FUN_141dd9360 param_4) 오프셋
const A_NAME: usize = 0x398;  // athlete char* champion_name
const A_TEAM: usize = 0x6a8;  // athlete team (0/1)
// AI구조체(FUN_141dd9360 param_2) — plan_state 임베드, subplan@+0x1878(flag=-1) 또는 +0x1870
const AI_FLAG: usize = 0x1370;   // param_2[0x4dc]: ==-1이면 subplan@+0x1878
const AI_SUB_A: usize = 0x1878;
const AI_SUB_B: usize = 0x1870;

// ── name→subplan 맵 (FUN_141dd9360 훅이 채움, think이 조회) ──
#[derive(Clone, Copy)]
struct SubEnt { used: bool, team: i64, subplan: i64, nlen: usize, name: [u8; 24] }
static SUBMAP: Mutex<[SubEnt; 16]> = Mutex::new([SubEnt { used: false, team: -1, subplan: -1, nlen: 0, name: [0u8; 24] }; 16]);
fn submap_set(team: i64, name: &[u8], subplan: i64) {
    if name.is_empty() || name.len() > 24 { return; }
    if let Ok(mut m) = SUBMAP.lock() {
        for e in m.iter_mut() {
            if e.used && e.team == team && e.nlen == name.len() && &e.name[..e.nlen] == name { e.subplan = subplan; return; }
        }
        for e in m.iter_mut() {
            if !e.used { e.used = true; e.team = team; e.subplan = subplan; e.nlen = name.len(); e.name[..name.len()].copy_from_slice(name); return; }
        }
    }
}
fn submap_get(team: i64, name: &[u8]) -> Option<i64> {
    if let Ok(m) = SUBMAP.lock() {
        for e in m.iter() {
            if e.used && e.team == team && e.nlen == name.len() && &e.name[..e.nlen] == name { return Some(e.subplan); }
        }
    }
    None
}

// ── CONFIG (런타임 튜너블; 파일 로드). 기본=안전(OFF). ──
// ★ replacement: retreat_engage 결정을 우리 코드로 대체. 기본 OFF(원본 통과). cfg replace=1로 켬.
//   1단계: 검증된 -1(퇴각, candidate!=0 && cnt!=0 && lane_pred==0)만 대체, 나머지 fall-through.
static REPL_ON: AtomicBool = AtomicBool::new(false);
static REPL_HANDLED: AtomicU64 = AtomicU64::new(0);   // 대체 처리 카운트(진단)
static REPL_OUT: AtomicI64 = AtomicI64::new(-1);      // ★대체 출력값(override 테스트). -1=원본동일(퇴각), 5=교전, 7=귀환 등
static READY_TICKS: AtomicU64 = AtomicU64::new(0);    // post_update 틱카운트(로딩중 게임함수 호출 방지용)
const READY_MIN: u64 = 200;                           // 이 틱수 지나야 훅에서 게임함수 호출(런칭 크래시 완화)
// ── facet#2 이동(position) override: driver memcpy(0x1d4ec17) 직전 Input(tag@0,x@8,y@0x10) 가로채기 ──
static MOVE_ON: AtomicBool = AtomicBool::new(false);  // cfg move=1: tag==1(Move) Input의 x/y를 강제
static MOVE_X: AtomicI64 = AtomicI64::new(336000);    // cell-center 좌표(cell*32000+16000). 맵중앙~336000
static MOVE_Y: AtomicI64 = AtomicI64::new(336000);
static MOVE_HANDLED: AtomicU64 = AtomicU64::new(0);   // 이동 override 적용 횟수
static TAG_COUNTS: [AtomicU64; 16] = [const { AtomicU64::new(0) }; 16]; // Input tag별 카운트(훅 발동확인)
// tag별 첫 샘플(struct 머리 9 qword +0~+0x40) — 좌표(16000~672000) 있는 곳이 Move
static TAG_SAMP: [[AtomicI64; 18]; 16] = [const { [const { AtomicI64::new(i64::MIN) }; 18] }; 16]; // 전체 0x90 struct
// 광범위 커밋(FUN_141a49fa0 @0x1d5035d) dump: 매프레임 최종 Input. 월드좌표 Move가 여기 흐르는지 확인.
static COMMIT_TOTAL: AtomicU64 = AtomicU64::new(0);
static COMMIT_TAGCOUNT: [AtomicU64; 16] = [const { AtomicU64::new(0) }; 16];
static COMMIT_SAMP: [[AtomicI64; 18]; 16] = [const { [const { AtomicI64::new(i64::MIN) }; 18] }; 16];
// 페이즈 게이트 threshold 베이스 패치: -1=원본(100) 유지, 0..127=imm8 덮어씀. cfg engage_base.
static ENGAGE_BASE: AtomicI64 = AtomicI64::new(-1);
static ENGAGE_ORIG: AtomicI64 = AtomicI64::new(-1);  // 최초 원본 imm8 백업(복원용)
// facet#5 셀렉터(local_228) df0c10 직후 신선값. retreat_capture 진입시 -777 리셋, df0c10훅서 셋, 리턴훅서 사용.
static SEL228_FRESH: AtomicI64 = AtomicI64::new(-777);
// ★facet#2 FUN_141dd7700 param 캡처(1단계: param체인 검증 + 후보 waypoint 엔티티 좌표 덤프)
static DD7700_N: AtomicU64 = AtomicU64::new(0);
static DD7700_P2: AtomicUsize = AtomicUsize::new(0);
static DD7700_P5: AtomicUsize = AtomicUsize::new(0);
static DD7700_P6: AtomicUsize = AtomicUsize::new(0);
static DD7700_P7: AtomicUsize = AtomicUsize::new(0);
static DD7700_P3: AtomicUsize = AtomicUsize::new(0);   // ★my_dd7700_full 검증용(param_3 count gate)
static DD7700_P4: AtomicUsize = AtomicUsize::new(0);   // ★param_4(r9 reindex/RNG)
static DD7F_OK: AtomicU64 = AtomicU64::new(0);         // dd7700 full-output 대조 OK/DIFF/passthrough
static DD7F_DIFF: AtomicU64 = AtomicU64::new(0);
static DD7F_PASS: AtomicU64 = AtomicU64::new(0);
static DD7F_INIT: AtomicBool = AtomicBool::new(false);
// ★my_dd7700_full을 capture시점(입력 정확)에 계산→여기 저장. hook_return은 DD7700_MY_OP==op 게이트로 대조(static staleness 회피).
static DD7700_MY: [AtomicU64; 2] = [AtomicU64::new(0), AtomicU64::new(0)];  // 내 출력 16B(out pre-state + my writes)
static DD7700_MY_OP: AtomicUsize = AtomicUsize::new(0);    // 계산한 호출의 out ptr(=hook_return op와 매칭)
static DD7700_MY_RES: AtomicU8 = AtomicU8::new(2);         // 0=None(passthrough) 1=Some(재현) 2=invalid
// ★disc3(dd7700/CAND_FILTER) 완전대체 게이트(cfg dd7_repl). RNG-sync 검증완료(DIFF=0/21500, 2026-06-20) + writeback 배선 → 대체시 출력+RNG 둘다 비트동일=no-desync. cfg로 토글.
static DD7_REPL: AtomicBool = AtomicBool::new(false);
// ★disc9/11(EpicPoke/SerpenPoke) 대체 게이트(cfg poke_repl). ⚠출력재현만 검증(pokecmp DIFF=0), RNG-sync 미구현 → 켜면 desync. RNG-sync 구현후 활성. DD7_REPL과 분리(disc3만 안전하게 켜기 위함).
static POKE_REPL: AtomicBool = AtomicBool::new(false);
static DD7_REPL_RNG_N: AtomicU64 = AtomicU64::new(0);     // disc3 대체시 RNG writeback 적용 횟수(진단)
// ★dd7700 RNG-sync 검증: capture시점 my_dd7700_rng_final로 exit RNG state 예측 → hook_return서 실제 exit과 per-call 대조(타이밍무관).
static DD7_RNG_P4: AtomicUsize = AtomicUsize::new(0);     // RNG state ptr(=p4), exit read + op매칭 게이트
static DD7_RNG_PIDX: AtomicU64 = AtomicU64::new(0);       // 예측 final idx
static DD7_RNG_PCTR: AtomicU64 = AtomicU64::new(0);       // 예측 final counter
static DD7_RNG_VALID: AtomicBool = AtomicBool::new(false);
static DD7RNG_OK: AtomicU64 = AtomicU64::new(0);
static DD7RNG_DIFF: AtomicU64 = AtomicU64::new(0);
static DD7RNG_INIT: AtomicBool = AtomicBool::new(false);
// ★facet#1 condgate in-scope RNG draw 카운트(cond_repl 안전 재확인): condgate 진입~리턴 동안 fcd980/fcdaf0/e88a0/e9a30 호출수.
//   replaced disc(my≠-99)가 0 draw면 RNG-free=skip 안전. >0이면 desync위험(writeback 필요).
static COND_INSCOPE: AtomicBool = AtomicBool::new(false);
static COND_IS_DRAWS: AtomicU64 = AtomicU64::new(0);     // in-scope RNG 함수호출수(전체)
static COND_IS_DEF: AtomicU64 = AtomicU64::new(0);       // fcd980+fcdaf0(항상 실제 draw)
static COND_IS_E88: AtomicU64 = AtomicU64::new(0);       // e88a0 실제 draw(count>0)만
static COND_IS_E9: AtomicU64 = AtomicU64::new(0);        // e9a30 호출(count불명)
static CONDRNG_INIT: AtomicBool = AtomicBool::new(false);
static COND_CUR_DISC: AtomicI64 = AtomicI64::new(-1);    // 현재 condgate disc(caller-trace용)
static COND_LEAK: AtomicU64 = AtomicU64::new(0);         // COND_INSCOPE 누수 횟수(진입시 이미 true=이전 미종료)
static COND_SITE_N: AtomicU64 = AtomicU64::new(0);       // caller-trace 로그수
static COND_SITE_INIT: AtomicBool = AtomicBool::new(false);
// condgate in-scope RNG draw의 caller(복귀주소) 로깅 → 어느 함수/사이트가 draw하나 특정.
unsafe fn cond_site_log(which: &str, orig_ret: usize) {
    if COND_SITE_N.fetch_add(1, Ordering::Relaxed) >= 3000 { return; }
    let base = exe_base();
    let rva = if base != 0 && orig_ret >= base { orig_ret - base } else { orig_ret };
    if !COND_SITE_INIT.swap(true, Ordering::Relaxed) { write_named("condrng_sites.txt", "=== condgate in-scope RNG draw caller RVA (disc별 어느 사이트가 fcd980/fcdaf0 호출) ===\n"); }
    append_named("condrng_sites.txt", &format!("disc={} {} callerRVA={:#x}\n", COND_CUR_DISC.load(Ordering::Relaxed), which, rva));
}
// disc별 최대 in-scope draw 관측(0이어야 안전). idx=disc.min(15)
static COND_DISC_MAXDRAW: [AtomicU64; 16] = [const { AtomicU64::new(0) }; 16];
// ★dd7700 in-scope RNG draw 카운트(진단): 진입~리턴 동안 fcd980/fcdaf0 실제 호출수 vs 내 예측 N.
static DD7_INSCOPE: AtomicBool = AtomicBool::new(false);
static DD7_IS_980: AtomicU64 = AtomicU64::new(0);   // in-scope fcd980 호출수
static DD7_IS_AF0: AtomicU64 = AtomicU64::new(0);   // in-scope fcdaf0 호출수
static DD7_RNG_N: AtomicU64 = AtomicU64::new(0);     // my_dd7700_rng_final 예측 draw수(gen_range 호출수)
static DD7_RNG_LO: AtomicU64 = AtomicU64::new(0);    // 진단: 내 윈도우 lo
static DD7_RNG_HI: AtomicU64 = AtomicU64::new(0);    // 진단: 내 윈도우 hi
static DD7_RNG_I0: AtomicU64 = AtomicU64::new(0);    // 진단: 내 entry idx
static DD7_RNG_CMASK: AtomicU64 = AtomicU64::new(0); // 진단: candtable 5비트 non-null 마스크
static DD7_RNG_DBG: AtomicU64 = AtomicU64::new(0);   // 진단패킹: plan(8) | f(4)<<8 | ivar12==1<<12 | target!=0<<13 | reached_candfilter<<15
static DD7_RNG_CTAB: AtomicUsize = AtomicUsize::new(0); // 진단: candtable base(l80+0x1e0+other*0x28). exit 재독으로 dd7700이 수정하는지 확인
static DD7CF_N: AtomicU64 = AtomicU64::new(0);          // dd7700-호출 CAND_FILTER ground-truth 로그수
static DD7CF_INIT: AtomicBool = AtomicBool::new(false);
static DD7_RNG_PI14: AtomicUsize = AtomicUsize::new(0); // role record addr(side*0x228+geo+roleoff). exit 재독으로 dd7700이 iVar12/target 수정하는지 확인
static DD7_RNG_TH0: AtomicU64 = AtomicU64::new(0);      // entry tgt_handle
static DD7700_DUMP: AtomicBool = AtomicBool::new(false);
// ★STEP5 목표선택 repro 결과(hook live 1회). goalkind: 1=endpoint(nexus), 2=near, 0=미산출/단순경로
static DD7700_R_DONE: AtomicBool = AtomicBool::new(false);
static DD7700_R_IV12: AtomicI64 = AtomicI64::new(-99);
static DD7700_R_TX: AtomicI64 = AtomicI64::new(0); static DD7700_R_TY: AtomicI64 = AtomicI64::new(0);
static DD7700_R_DSELF: AtomicI64 = AtomicI64::new(-1); static DD7700_R_DNEAR: AtomicI64 = AtomicI64::new(-1);
static DD7700_R_GOALX: AtomicI64 = AtomicI64::new(0); static DD7700_R_GOALY: AtomicI64 = AtomicI64::new(0);
static DD7700_R_GOALKIND: AtomicI64 = AtomicI64::new(0);
// ★dd7700 게임출력 캡처(action code) + vtable슬롯/테이블 식별. cfg dd7cap=1.
// dd7700은 param_1(out ptr)를 그대로 리턴 → 리턴훅 retval==out ptr. game action code=*retval(+0=i64, 바이트 +8/+9/+10).
static DD7CAP: AtomicBool = AtomicBool::new(false);
static DD7_ARMED: AtomicU64 = AtomicU64::new(0);     // 리턴훅 무장 호출 수(상한)
static DD7_LOGGED: AtomicU64 = AtomicU64::new(0);    // dd7cmp.txt 기록 수
const DD7_ARM_MAX: u64 = 800;
static DD7_FILE_INIT: AtomicBool = AtomicBool::new(false);
// L80[1] vtable + 슬롯 함수주소(1회 캡처; post_update서 RVA로 덤프). 슬롯: +0x20/+0x48/+0xa8/+0x128/+0x140/+0x168.
static DD7_VT: AtomicUsize = AtomicUsize::new(0);
static DD7_S20: AtomicUsize = AtomicUsize::new(0);
static DD7_S48: AtomicUsize = AtomicUsize::new(0);
static DD7_SA8: AtomicUsize = AtomicUsize::new(0);
static DD7_S128: AtomicUsize = AtomicUsize::new(0);
static DD7_S140: AtomicUsize = AtomicUsize::new(0);
static DD7_S168: AtomicUsize = AtomicUsize::new(0);
static DD7_CALLEE_DUMP: AtomicBool = AtomicBool::new(false);
static DD7_DEEP: AtomicU64 = AtomicU64::new(0);  // tail STAGE1/2/C/E 통과 = 6/7 후보(deep) 케이스 수
// ★PRNG gen_range 검증: fcd980 호출마다 my_gen_range(read-only 시뮬) vs 실제 반환 대조. cfg rngcap=1.
static RNGCAP: AtomicBool = AtomicBool::new(false);
static RNG_ARMED: AtomicU64 = AtomicU64::new(0);
static RNG_LOGGED: AtomicU64 = AtomicU64::new(0);
static RNG_FILE_INIT: AtomicBool = AtomicBool::new(false);
const RNG_ARM_MAX: u64 = 1200;
// ★④ RNG write-back 상태전이 검증: 내 예측 after-state(idx,counter) == 게임 실제 (rng_advance_writeback 정확성 사전검증).
static RNGST_OK: AtomicU64 = AtomicU64::new(0);
static RNGST_DIFF: AtomicU64 = AtomicU64::new(0);
// ★④ step3: cfg rng_repl=1이면 교전롤 fcd980(ret=ROLL_RET) 호출을 우리 rng_advance_writeback로 대체(실전 RNG-sync 검증). 기본 off.
static RNG_REPL: AtomicBool = AtomicBool::new(false);
static RNG_REPL_N: AtomicU64 = AtomicU64::new(0);   // 대체 발동 횟수
// ★④ Stage B: facet#5 engage ENTRY 완전대체(cfg engage_repl=1, +replace=1 필수). my_engage_emit(출력+RNG writeback). 검증 2500/2500 diverse. 기본 off.
static ENGAGE_REPL: AtomicBool = AtomicBool::new(false);
static ENGAGE_REPL_N: AtomicU64 = AtomicU64::new(0);   // engage entry 대체 발동
static ENGAGE_REPL_PASS: AtomicU64 = AtomicU64::new(0); // 가드실패 passthrough
// passthrough 사유 분류(100% vs 갭 판별)
static PT_GATE: AtomicU64 = AtomicU64::new(0);   // engage_reaches_roll != Some(true) (게이트 발화/불확실=정상 위임)
static PT_COUNT: AtomicU64 = AtomicU64::new(0);  // my_e9a30_count None (jtv/cand_get 미해결=재현갭)
static PT_OTHER: AtomicU64 = AtomicU64::new(0);  // 그외(ptr가드/pick/thr None)
// ★④ condgate 완전대체(cfg cond_repl=1): my_condgate(≠-99)로 게임 condgate 출력 대체(원본 skip).
//   ✅✅2026-06-21 정정: condgate는 **RNG-FREE 확정**(ghidra-re 정적 depth-12 BFS, 정확 .pdata경계: 12핸들러+poke 모두 RNG호출0, macro 서브시스템 미호출).
//   옛 "RNG-FREE 아님(disc draw)" 판정은 오판=①런타임 in-scope 측정 confound(같은 plan프레임 다른 macro draw 혼입) ②BFS 경계오버런(poke 직후 인접함수=별개 vtable함수가 e88a0 호출).
//   ⟹ cond_repl=1 SKIP 안전(desync無). **gold-standard 검증완료: cond_repl=1 단독 다시보기=원본 비트동일, COND_REPL 32700+, passthrough=0(100% 우리것).**
static COND_REPL: AtomicBool = AtomicBool::new(false);
static COND_REPL_N: AtomicU64 = AtomicU64::new(0);     // 대체 발동(my≠-99=우리것)
static COND_REPL_PASS: AtomicU64 = AtomicU64::new(0);  // passthrough(my=-99=게임원본). 0에 가까우면 100% 우리것
static COND_PASS_DISC: [AtomicU64; 16] = [             // passthrough시 disc 분포(어느 핸들러가 -99 내나)
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0)];
// ★④ movepriority 완전대체(cfg mp_repl=1): disc 0/1 인라인출력 재현→대체(원본 dispatcher skip). 그 외 disc=passthrough(capture). 기본 off.
static MP_REPL: AtomicBool = AtomicBool::new(false);
static MP_REPL_N: AtomicU64 = AtomicU64::new(0);    // disc0/1 대체 발동
static MP_REPL_PASS: AtomicU64 = AtomicU64::new(0); // 그 외 disc(미대체 passthrough)
// ★movepriority 출력계약 진단(capture모드): 진입시 *param_1 8qword 스냅 → 리턴서 diff = sub-judge가 쓴 오프셋. code-only(+0만)/aux 판별.
static MP_ENTRY: [AtomicU64; 8] = [AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0)];
static MP_ENTRY_PTR: AtomicUsize = AtomicUsize::new(0);
static MP_WS: [AtomicU64; 16] = [AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0)];
static MP_WS_INIT: AtomicBool = AtomicBool::new(false);
// ★disc9/11(EpicPoke/SerpenPoke) full-output 대체검증: capture시 aux입력 보관 → hook_return kind7서 내 full재현 vs 게임출력 byte대조(pokecmp.txt).
static MP_AUX_OP: AtomicUsize = AtomicUsize::new(0);   // 현 in-flight poke의 출력ptr(프레임 매칭). 비중첩이라 단일 static 안전.
static MP_AUX_P2: AtomicUsize = AtomicUsize::new(0);   // 서브저지 param_2(=subp+8): code0x13의 [+0x11]=byte[p2]
static MP_AUX_P6: AtomicUsize = AtomicUsize::new(0);   // 서브저지 param_6(=r15): serpen code3/2 uVar6 룩업
static POKE_OK: AtomicU64 = AtomicU64::new(0);
static POKE_DIFF: AtomicU64 = AtomicU64::new(0);
static POKE_INIT: AtomicBool = AtomicBool::new(false);
// ★disc9/11 RNG footprint 측정(ground-truth): mp_capture 진입시 p4(=r9=RNG) idx/counter 스냅 → kind7 리턴서 exit delta=실제 draw 소비. 디스패처 비재귀=단일static 안전.
static POKE_RNG_P4: AtomicUsize = AtomicUsize::new(0);
static POKE_RNG_I0: AtomicU64 = AtomicU64::new(0);
static POKE_RNG_C0: AtomicU64 = AtomicU64::new(0);
static POKE_RNG_GUARD: AtomicU8 = AtomicU8::new(0);   // early-guard(*(p2sj)!=0||*(p2sj+1)!=0) — 0draw 경로
static POKE_RNG_PLAN: AtomicI64 = AtomicI64::new(-1);
static POKERNG_INIT: AtomicBool = AtomicBool::new(false);
static POKE_RNG_N_CTR: AtomicU64 = AtomicU64::new(0);
// ★disc11 RNG 소스 추적: dispatcher disc11 진입~리턴 윈도우 동안 fcd980/fcdaf0 호출의 caller RVA를 로깅(어느 함수가 gen_range 호출하는지 직접 특정).
static POKE_INSCOPE: AtomicBool = AtomicBool::new(false);
static POKE_RET_N: AtomicU64 = AtomicU64::new(0);
static POKERET_INIT: AtomicBool = AtomicBool::new(false);
// ★dispatcher-레벨 e88a0 arg 재구성 검증: RNG=r9, e88a0_p4=r14(param5), e88a0_p7=*(r15+8)(param6[1]). count→gen_range(0,count) 예측 exit vs 실제 p4 exit.
static POKE_PIDX: AtomicU64 = AtomicU64::new(0);   // 예측 exit idx
static POKE_PCTR: AtomicU64 = AtomicU64::new(0);   // 예측 exit counter
static POKE_PCOUNT: AtomicI64 = AtomicI64::new(-1); // 예측 count(-1=계산실패)
static POKE_E88_OK: AtomicU64 = AtomicU64::new(0);
static POKE_E88_DIFF: AtomicU64 = AtomicU64::new(0);
// ★드라이버 페이즈게이트 캡처(cfg pgcap). 기본 off=배포안전.
static PGCAP: AtomicBool = AtomicBool::new(false);
static PG_ARMED: AtomicU64 = AtomicU64::new(0);
static PG_FILE_INIT: AtomicBool = AtomicBool::new(false);
const PG_ARM_MAX: u64 = 400;   // 안정 복원(실제경기 오래재생됐던 pgcap-ms덤프 버전과 동일)
// ★페이즈게이트 A/B/C override (cfg pg_a/pg_b/pg_c, -1=무override). 스텁 저장슬롯 덮어쓰기 → 드라이버 threshold 변조.
static PG_OV_A: AtomicI64 = AtomicI64::new(-1);
static PG_OV_B: AtomicI64 = AtomicI64::new(-1);
static PG_OV_C: AtomicI64 = AtomicI64::new(-1);
// ★subplan_transition_engine(0x1d45290) 엔트리 캡처(cfg tecap). phase=S[0x4ce]별 분포 + 입력 덤프. 기본 off.
static TECAP: AtomicBool = AtomicBool::new(false);
static TE_ARMED: AtomicU64 = AtomicU64::new(0);
static TE_FILE_INIT: AtomicBool = AtomicBool::new(false);
const TE_ARM_MAX: u64 = 600;
static TE_PHASE_HIST: [AtomicU64; 16] = [
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0)];
static TE_SUB_HIST: [AtomicU64; 16] = [
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0)];
static TE_CALLS: AtomicU64 = AtomicU64::new(0);
// 챔피언별 마지막 subplan 추적 → 프레임간 변화=실제 전환 이벤트 포착(출력은 함수 밖서 적용되므로).
static TE_TRACK: Mutex<Vec<(usize, i64, [i64;7])>> = Mutex::new(Vec::new());  // (champ, sub, [phase,gate,subdisc,ath228,team,lane,a2] = 결정프레임 입력; 0.4.13: r9=athlete)
static TE_TRANS_N: AtomicU64 = AtomicU64::new(0);
// ★fc59a0 recall RNG score 캡처(cfg recallcap). facet#5 cVar6==1 RECALL-vs-roll 갭. RNG배율+최종score 검증(1차).
static RECALLCAP: AtomicBool = AtomicBool::new(false);
static RECALL_ARMED: AtomicU64 = AtomicU64::new(0);
static RECALL_FILE_INIT: AtomicBool = AtomicBool::new(false);
// ★recall(fc59a0) 완전대체 게이트(cfg recall_repl). score(mult) 재현·검증완료(recallcmp DIFF=0) + u32 RNG writeback. 켜면 fc59a0 skip→내 출력+RNG전진. ⚠RECALL 희귀=검증 기회캡처.
static RECALL_REPL: AtomicBool = AtomicBool::new(false);
static RECALL_REPL_N: AtomicU64 = AtomicU64::new(0);
static RECALL_REPL_PASS: AtomicU64 = AtomicU64::new(0);
const RECALL_ARM_MAX: u64 = 600;
// ★CAND_FILTER(0x1f4ec60) white-box 검증 캡처(cfg candcap). 진입 RNG스냅샷+cand_filter_repro 예측 → 리턴훅 kind6서 게임 출력Vec 대조.
static CANDCAP: AtomicBool = AtomicBool::new(false);
static CAND_ARMED: AtomicU64 = AtomicU64::new(0);
static CAND_RAW: AtomicU64 = AtomicU64::new(0);
static CAND_FILT: AtomicU64 = AtomicU64::new(0);
static CAND_LOGGED: AtomicU64 = AtomicU64::new(0);
static CAND_FILE_INIT: AtomicBool = AtomicBool::new(false);
const CAND_ARM_MAX: u64 = 600;
static CAND_PRED: Mutex<(usize, Vec<usize>)> = Mutex::new((0, Vec::new()));   // (out_ptr, 예측 Vec) 단일슬롯(호출 비중첩)
// ★generic_build 스코어러(0x1f80320) white-box 검증(cfg gbcap). entry RNG스냅샷+my_f80320 예측 → 리턴훅 kind11서 game score+draw 대조.
static GBCAP: AtomicBool = AtomicBool::new(false);
static GB_ARMED: AtomicU64 = AtomicU64::new(0);
static GB_RAW: AtomicU64 = AtomicU64::new(0);
static GB_LOGGED: AtomicU64 = AtomicU64::new(0);
static GB_FILE_INIT: AtomicBool = AtomicBool::new(false);
const GB_ARM_MAX: u64 = 800;
// ★generic_build 본체(0x20def90) 디스패치/출력 캡처(cfg gbbody). 진입(disc,param2,team) 스냅 + 리턴훅 kind:14서 out kind@+0x58/arg@+0x60/action Vec 읽기. 게임호출 제로(순수 read).
static GBBODY: AtomicBool = AtomicBool::new(false);
static GBB_ARMED: AtomicU64 = AtomicU64::new(0);
static GBB_RAW: AtomicU64 = AtomicU64::new(0);
static GBB_FILE_INIT: AtomicBool = AtomicBool::new(false);
const GBB_ARM_MAX: u64 = 100000;   // ★gbrepl 대체모드서 리턴훅 무장범위(=덮어쓸 수 있는 generic_build 호출 수). verify(gbbody/gbrd)엔 별도 GBRD_ARM_MAX/GBB_SEEN 스로틀이 더 좁게 작용.
const GBB_PER_KEY: u32 = 24;                          // unique (disc,param2)별 캡처 상한(분포 골고루)
static GBB_SEEN: Mutex<Vec<(u64,u32)>> = Mutex::new(Vec::new());
static GBB_OK: AtomicU64 = AtomicU64::new(0);      // my_generic_build 예측 일치
static GBB_DIFF: AtomicU64 = AtomicU64::new(0);    // 예측 불일치
static GBB_NOPRED: AtomicU64 = AtomicU64::new(0);  // None(미예측=메인빌드/B/C/D)
static F80_INSCOPE: AtomicBool = AtomicBool::new(false);   // 0x1f80320 실행 중 표시(fcdaf0 카운트 게이트)
static F80_DRAWS: AtomicU64 = AtomicU64::new(0);           // INSCOPE 중 fcdaf0 호출 수 = 게임 실제 draw
static GB_DRAW_OK: AtomicU64 = AtomicU64::new(0);
static GB_DRAW_DIFF: AtomicU64 = AtomicU64::new(0);
static GB_TERM: AtomicUsize = AtomicUsize::new(0);   // ready_walk 미지 terminal vt+0x58 RVA(찾기용)
static GB_SCORE_OK: AtomicU64 = AtomicU64::new(0);
static GB_SCORE_DIFF: AtomicU64 = AtomicU64::new(0);
// ★per-site draw 카운터(F80320 +1 진단): [0]base [1]슬롯게이트 [2]+0x78 [3]+0x7d [4]+0x82 [5]list2. my_f80320가 매 호출 리셋+증가.
static GB_SITE: [AtomicU32; 6] = [AtomicU32::new(0),AtomicU32::new(0),AtomicU32::new(0),AtomicU32::new(0),AtomicU32::new(0),AtomicU32::new(0)];
// ★영역 D callee(0x203cb30 단일종합점수 / 0x20c0690 post점수) white-box 검증(cfg gbcallee, task#2).
//   진입서 entity ptr 인자 캡처 → my_203cb30/my_20c0690 예측 → 리턴훅 kind:20서 game retval(rax,u64) 대조.
//   순수 점수함수(RNG미소비). resolver/norm은 SHIM oracle(gb_resolver/gb_norm), my_combat_dmg=순수.
static GBCALLEE: AtomicBool = AtomicBool::new(false);
static GBC_ARMED: AtomicU64 = AtomicU64::new(0);
static GBC_RAW: AtomicU64 = AtomicU64::new(0);
static GBC_LOGGED: AtomicU64 = AtomicU64::new(0);
static GBC_FILE_INIT: AtomicBool = AtomicBool::new(false);
static GBC203_OK: AtomicU64 = AtomicU64::new(0);
static GBC203_DIFF: AtomicU64 = AtomicU64::new(0);
static GBC690_OK: AtomicU64 = AtomicU64::new(0);
static GBC690_DIFF: AtomicU64 = AtomicU64::new(0);
// ★진단(호출됨? 아니면 가드탈락?): per-fn 최상단 raw(모든 진입, READY/cfg게이트 前) + badptr/panic 카운터.
static GBC203_RAW: AtomicU64 = AtomicU64::new(0);
static GBC690_RAW: AtomicU64 = AtomicU64::new(0);
static GBC_BADPTR: AtomicU64 = AtomicU64::new(0);
static GBC_PANIC: AtomicU64 = AtomicU64::new(0);
const GBC_ARM_MAX: u64 = 800;
// ★영역 D 출력검증(cfg gbrd, genbuild_body_D.md "런타임 캡처 빌드"): mid-func 0x20e42a3 캡처 → RegionD locals(rbp/r12/r13)
//   → gb_region_d 예측을 out ptr 키로 GBRD_MAP 저장. generic_build 리턴훅(kind14)이 같은 out ptr로 조회해 game out+0x58/+0x60 대조.
//   ★mid-func라 return 하이재킹 불가 → 저장만. gbrd=1이면 genbuild_body_capture(kind14 리턴)도 자동 무장. 순수 read+gb_region_d(순수)=게임호출0.
static GBRD: AtomicBool = AtomicBool::new(false);
static GBRD_RAW: AtomicU64 = AtomicU64::new(0);     // 0x42a3 전체진입(READY/cfg게이트 前) — "도달함?" 판정
static GBRD_ARMED: AtomicU64 = AtomicU64::new(0);   // GBRD_MAP에 store된 예측 수
static GBRD_BADPTR: AtomicU64 = AtomicU64::new(0);
static GBRD_PANIC: AtomicU64 = AtomicU64::new(0);
static GBRD_OK: AtomicU64 = AtomicU64::new(0);      // gb_region_d == game (kind+arg)
static GBRD_DIFF: AtomicU64 = AtomicU64::new(0);
static GBRD_NP: AtomicU64 = AtomicU64::new(0);      // gb_region_d None(미확정 분기 sil!=1/idle/0x4659)
static GBRD_VPUSH: AtomicU64 = AtomicU64::new(0);   // 영역 D가 action Vec에 push한(delta>0) 케이스 수(action Vec 검증 진단)
static GBRD_FILE_INIT: AtomicBool = AtomicBool::new(false);
const GBRD_ARM_MAX: u64 = 4000;
// out ptr → (예측 Option<(kind,arg)>, locals 덤프, 영역D진입시 action Vec len). kind14 리턴훅서 find+remove(같은 invocation 내 store→consume).
//   entry_vlen = 0x42a3시 out+0x78(=A/B/C가 쌓은 len). 리턴서 최종 len과 비교 → 영역 D push delta 진단(action Vec 검증).
static GBRD_MAP: Mutex<Vec<(usize, Option<(i64,u64,u16)>, String, u64)>> = Mutex::new(Vec::new());
// ★영역 D 한정 대체모드(cfg gbrepl): live locals서 gb_region_d 계산(0x42a3) → 함수리턴(kind14)서 game out+0x58/+0x60을
//   내 결정으로 덮어씀. 제어흐름 hijack無(리턴-overwrite=안전). gb_region_d==game(DIFF=0)이라 무수정시 게임동작 동일(메커니즘 투명성 증명),
//   gb_region_d 튜닝시 게임 AI가 그 결정 채택. ⚠v1 한계: 리턴훅 무장(GBB_ARM_MAX) 범위까지만(=조기~중반 다수). 전건 대체는 inline skip 필요(후속).
static GBREPL: AtomicBool = AtomicBool::new(false);
static GBREPL_N: AtomicU64 = AtomicU64::new(0);   // 실제 덮어쓴 횟수
// ★대체 충실성 체크(cfg gbreplchk): 덮어쓰기 없이 에필로그서 pred vs game out+0x58/0x60 대조(전케이스, 미cap) → match/mismatch+로그.
//   체크전용서 게임이 같으면=hook 투명(메커니즘OK), mismatch=gb_region_d 미검증오류. 다르면=메커니즘 side effect.
static GBREPLCHK: AtomicBool = AtomicBool::new(false);
static GBREPL_MATCH: AtomicU64 = AtomicU64::new(0);
static GBREPL_MISMATCH: AtomicU64 = AtomicU64::new(0);
static GBREPLCHK_FILE_INIT: AtomicBool = AtomicBool::new(false);
// ★진짜 skip 대체(cfg gbskip): region D RNG-free라 0x42a3서 gb_region_d 계산→out기록→funnel jump=게임 region D 미실행(진짜 계산대체).
//   overwrite(게임실행+덮어쓰기)와 달리 게임 region D 건너뜀. push≠0/None은 passthrough(게임실행=Vec보존). install_detour_d_skip 필요.
static GBSKIP: AtomicBool = AtomicBool::new(false);
static GBSKIP_N: AtomicU64 = AtomicU64::new(0);
// ★dedc0 timing분기(out+0x40==0 && b_logic) 오라클 해결(cfg gbdedc0): my_dedc0가 None인 21 NP에서만 FUN_1420dedc0 shadow-call(getter=leaf 오라클, resolver/norm과 동급). 게임함수콜=AV위험 cfg게이트(기본OFF).
static GBDEDC0: AtomicBool = AtomicBool::new(false);
// ★facet#4 movepriority 관측(cfg mpcap). disc→출력코드 분포. 별도 judge 10개라 우선 관측.
static MPCAP: AtomicBool = AtomicBool::new(false);
static MP_ARMED: AtomicU64 = AtomicU64::new(0);
static MP_OK: AtomicU64 = AtomicU64::new(0);
static MP_DIFF: AtomicU64 = AtomicU64::new(0);
static MP_PEND: AtomicU64 = AtomicU64::new(0);
static MP_FILE_INIT: AtomicBool = AtomicBool::new(false);
// ★DefenseNexus(subplan=14) 7-watcher: per-subplan 캡(400) 무관, 무강제·무제한 관측. game!=18(=7)만 로깅.
static DEFWATCH: AtomicBool = AtomicBool::new(false);
static DEF_DIAG: AtomicU64 = AtomicU64::new(0);    // my_defense_nexus가 매 호출 채움(hp%/home/near/pred/side/nexus_hp%)
static POKE_DIAG: AtomicI64 = AtomicI64::new(-1);  // my_poke_helper가 매 호출 채움(분기/cnt/f50full vs f50low/nvalid/nearest) — serpent poke DIFF 진단
// poke_timing_branch 내부값 진단 (serpent timing return-1 갭): cond/target/timing/gap/thr*15/ret
static TD_COND: AtomicI64 = AtomicI64::new(-1);
static TD_TGT: AtomicI64 = AtomicI64::new(0);
static TD_TIM: AtomicI64 = AtomicI64::new(0);
static TD_GAP: AtomicI64 = AtomicI64::new(0);
static TD_THR: AtomicI64 = AtomicI64::new(0);
static TD_RET: AtomicI64 = AtomicI64::new(-1);
static TD_A0: AtomicI64 = AtomicI64::new(0);     // [ctx+off_a] (cond sub-path1 게이트값)
static TD_V140: AtomicI64 = AtomicI64::new(0);   // vt140(robj,arg) 결과 (a0!=0일때만; i64::MIN=미계산)
static DEFW_ARMED: AtomicU64 = AtomicU64::new(0);  // kind8 무장 카운트(상한 200000=폭주방지)
static DEFW_N: AtomicU64 = AtomicU64::new(0);      // defwatch.txt 기록 카운트(상한 1000)
static DEFW_INIT: AtomicBool = AtomicBool::new(false);
const MP_SUB_CAP: u64 = 400;
static MP_SUB_ARMED: [AtomicU64; 16] = [
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0)];
// ★④ movepriority 출력계약 덤프(disc별 게임 출력구조 head): 어느 핸들러가 code만/aux도 쓰나 → replace 재현범위 결정.
static MPOUT_CNT: [AtomicU64; 16] = [
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0)];
static MPOUT_INIT: AtomicBool = AtomicBool::new(false);
static FC59_RAW: AtomicU64 = AtomicU64::new(0);   // fc59a0 진입 raw 카운트(필터 무관, 호출여부 진단)
static FC59_ARM: AtomicU64 = AtomicU64::new(0);   // 리턴훅 무장 성공 카운트
static FC59_FILT: AtomicU64 = AtomicU64::new(0);  // 진입했으나 필터로 return
// ★facet#1 condgate 검증(cfg condcap). my_condgate vs 게임 al. Stage1=dispatch+단순핸들러(poke=pending).
static CONDCAP: AtomicBool = AtomicBool::new(false);
static COND_ARMED: AtomicU64 = AtomicU64::new(0);
static COND_OK: AtomicU64 = AtomicU64::new(0);
static COND_DIFF: AtomicU64 = AtomicU64::new(0);
static COND_PEND: AtomicU64 = AtomicU64::new(0);   // poke 등 미재현(-99)
static COND_FILE_INIT: AtomicBool = AtomicBool::new(false);
const COND_ARM_MAX: u64 = 12000;
const COND_SUB_CAP: u64 = 500;   // subplan(disc)당 최대 캡 → 희귀 핸들러도 골고루 잡힘
static COND_SUB_ARMED: [AtomicU64; 16] = [
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0)];
// ★시드 회전(cfg seed_rotate): 매 프레임 practice replay seed(base+688)를 회전값으로 덮어씀 → 일반 다시보기가 매번 다른 시드 = 다양한 경기.
static SEED_ROTATE: AtomicBool = AtomicBool::new(false);
static SEED_ROT: AtomicU64 = AtomicU64::new(0);
static CUR_SEED: AtomicU64 = AtomicU64::new(0);   // 현재 practice replay에 적용된 시드(메뉴서만 갱신 → sim중 freeze = sim 실제시드). condgate 로그에 찍힘.
static SEED_SET: AtomicU64 = AtomicU64::new(0);   // cfg seed_set: !=0면 회전 대신 이 시드로 고정(DIFF 경기 재현용)
static LAST_AI_FRAME: AtomicU64 = AtomicU64::new(0);   // condgate 마지막 발화 프레임(READY_TICKS). 갭>60=메뉴(회전ON), 갭작음=경기중(회전OFF=시드freeze). post_update가 sim중에도 돌아 IN_MENU 신뢰불가 → 갭휴리스틱.
// 원본 시드 백업 (base, orig_seed) — 끄면 복원해서 세이브 보호.
static SEED_BAK: Mutex<Vec<(usize, u64)>> = Mutex::new(Vec::new());
const O_SEED_REPLAY: usize = 688;
// ★전술 회전(cfg strat_rotate): 메뉴 프레임마다 practice replay 팀전술(24B Strategy: blue@base+0x78/red@base+0x90)을 무작위화 → 다시보기마다 팀 전략 변화 = AI 행동 다양성. (seed_rotate와 병행 가능). 끄면 복원(세이브 보호).
//   12 서브필드(foc/jng/srp/srt/bld/bat/mor/twr/def/fin/wav/end) byte오프셋=STRAT_OFFS_ROT, 변형수=STRAT_VC(bld/mor는 split회피 위해 0/1로 제한). tfm2_scrim O_BLUE_STRAT/O_RED_STRAT/STRAT_OFFS와 동일 매핑.
static STRAT_ROTATE: AtomicBool = AtomicBool::new(false);
static STRAT_ROT_N: AtomicU64 = AtomicU64::new(0);
static STRAT_BAK: Mutex<Vec<(usize, [u8;24], [u8;24])>> = Mutex::new(Vec::new());   // (base, blue24, red24)
static STRAT_CUR: Mutex<([u8;12],[u8;12])> = Mutex::new(([0u8;12],[0u8;12]));   // 현 회전 strat(blue,red) 12필드 — seedstrat.txt 로깅용(strat_rotate ON시만 갱신)
static STRAT_SET: Mutex<Option<([u8;12],[u8;12])>> = Mutex::new(None);   // cfg strat_set: Some면 회전 대신 이 고정 strat 주입(seed_set과 함께 code7 매치 재현)
const O_BLUE_STRAT: usize = 0x78;
const O_RED_STRAT: usize = 0x90;
const STRAT_OFFS_ROT: [usize; 12] = [17,18,19,20,0,12,4,13,14,15,16,21];
const STRAT_VC: [u8; 12] = [3,3,3,3,2,2,2,2,2,2,2,3];   // 변형수(foc/jng/srp/srt=3, bld/mor=2(split회피), bat/twr/def/fin/wav=2, end=3)
// ★per-replay 리셋(cfg replay_reset): 메뉴 갔다가 새 sim 첫 훅 시점에 모든 캡처상태 초기화 → 다시보기마다 fresh 로그.
static REPLAY_RESET: AtomicBool = AtomicBool::new(false);
static IN_MENU: AtomicBool = AtomicBool::new(false);   // post_update가 매 메뉴프레임 true; 첫 sim 훅이 swap(false)+reset
// ★facet#5 교전롤 예측: 롤 fcd980(복귀주소=RVA_ROLL_RET) 시점 상태로 예측. retreat_engage 진입시 VALID=false 리셋.
static PRED_ROLL: AtomicI64 = AtomicI64::new(-999);
static PRED_ROLL_VALID: AtomicBool = AtomicBool::new(false);
// ★★ judge 튜닝 계수 (cfg [튜닝] 섹션; 기본값=게임원본 → 안 건드리면 replay-identical 유지). 우리 대체 judge의 계수를 유저가 override.
static TUNE_ENGAGE_MULT: AtomicI64 = AtomicI64::new(100);  // engage role thr 배율%: 높을수록 교전 공격적(thr↑→roll>=thr 드묾→engage), 낮으면 소극적
static TUNE_TTD_MULT: AtomicI64 = AtomicI64::new(100);     // disc4 TTD 임계 배율%: 처치/갱킹 적극성
static TUNE_RECALL_BIAS: AtomicI64 = AtomicI64::new(0);    // recall score 가산: >0=자주 복귀(안전), <0=덜 복귀(공격적 체류)
static TUNE_GB_MULT: AtomicI64 = AtomicI64::new(100);      // generic_build 영역D 거리임계 배율%: 매크로 운영전환 거리 성향
// ★★ 세밀 계수 테이블: cfg의 명시 arm에 없는 key(=세밀 튜닝 계수)를 저장. tune("key",게임원본기본)으로 judge 계산식 매직넘버 override.
//   새 계수 노출 = my_*에 tune("key",orig) 1줄 + cfg 1줄. 미설정 key는 기본값(=replay-identical).
// ★성능: lock-free 읽기(atomic swap) + 빠른 해셔. judge 핫패스 tune()은 lock 없는 atomic load+get.
//   ★FNV-1a 해셔: std 기본 SipHash(암호학적·느림 ~40ns/lookup)는 judge 핫루프 tune()서 과함 → FNV ~8ns.
#[derive(Clone, Copy, Default)] struct FnvBuild;
struct FnvHasher(u64);
impl std::hash::BuildHasher for FnvBuild { type Hasher = FnvHasher; #[inline] fn build_hasher(&self) -> FnvHasher { FnvHasher(0xcbf29ce484222325) } }
impl std::hash::Hasher for FnvHasher {
    #[inline] fn finish(&self) -> u64 { self.0 }
    #[inline] fn write(&mut self, bytes: &[u8]) { let mut h = self.0; for &b in bytes { h ^= b as u64; h = h.wrapping_mul(0x100000001b3); } self.0 = h; }
}
type TuneMap = HashMap<String, i64, FnvBuild>;
static TUNE_PTR: AtomicPtr<TuneMap> = AtomicPtr::new(std::ptr::null_mut());
static TUNE_PREV: AtomicPtr<TuneMap> = AtomicPtr::new(std::ptr::null_mut());   // ★누수상한: 직전 old 보관(2세대 지연 free)
#[inline] fn tune(key: &str, default: i64) -> i64 {
    let p = TUNE_PTR.load(Ordering::Acquire);
    if p.is_null() { default } else { unsafe { (*p).get(key).copied().unwrap_or(default) } }
}
// cfg 로드 끝에서 새 테이블 게시. ★누수상한(2세대 지연 free): 옛 테이블 즉시 free는 reader(judge가 tune() 읽는 중)
//   use-after-free 위험 → 직전 old는 TUNE_PREV에 보관하고 그 전 세대(N-2)만 free. reader는 judge 1회(µs)내 끝나고
//   게시는 cfg mtime변경(초)마다라 2세대차면 reader 없음 = 안전. 살아있는 테이블 ≤2개로 바운드(무한누수 제거).
fn tune_publish(map: TuneMap) {
    let boxed = Box::into_raw(Box::new(map));
    let old = TUNE_PTR.swap(boxed, Ordering::AcqRel);
    let stale = TUNE_PREV.swap(old, Ordering::AcqRel);   // N-2 세대를 회수
    if !stale.is_null() { unsafe { drop(Box::from_raw(stale)); } }
}
// ★★ judge 성능 측정(cfg perf_measure=1): 각 judge 진입~출구 누적 ns/호출수 → perf.txt. 어느 대체 judge가 무거운지 식별용.
static PERF_NS: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];
static PERF_CNT: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];
static PERF_ON: AtomicBool = AtomicBool::new(false);
static PERF_FLUSH: AtomicU64 = AtomicU64::new(0);
const PERF_NAMES: [&str; 8] = ["condgate","dd7700","disc4","gb_regionD","recall","engage","movepri","poke"];
static DD7_CODE_NS: AtomicU64 = AtomicU64::new(0);   // 진단: dd7700 engage경로 my_dd7700_code(STAGE6) 누적ns
static DD7_CODE_N: AtomicU64 = AtomicU64::new(0);
static DD7_RESOLVE_NS: AtomicU64 = AtomicU64::new(0);   // 진단: dd7700 STAGE2 게임 resolver(vt0x140) 누적ns
static DD7_RESOLVE_N: AtomicU64 = AtomicU64::new(0);
struct PerfGuard { idx: usize, t: Instant }
impl Drop for PerfGuard {
    fn drop(&mut self) {
        PERF_NS[self.idx].fetch_add(self.t.elapsed().as_nanos() as u64, Ordering::Relaxed);
        PERF_CNT[self.idx].fetch_add(1, Ordering::Relaxed);
        if PERF_FLUSH.fetch_add(1, Ordering::Relaxed) % 50000 == 49999 {
            let mut s = String::from("=== judge perf (총ns / 호출수 / 평균ns/call) ===\n");
            for i in 0..8 {
                let ns = PERF_NS[i].load(Ordering::Relaxed);
                let c = PERF_CNT[i].load(Ordering::Relaxed);
                s.push_str(&format!("{:11} {:>16} / {:>9} / {:>7}\n", PERF_NAMES[i], ns, c, ns / c.max(1)));
            }
            s.push_str(&format!("{:11} {:>16} / {:>9} / {:>7}\n", "dd7_code", DD7_CODE_NS.load(Ordering::Relaxed), DD7_CODE_N.load(Ordering::Relaxed), DD7_CODE_NS.load(Ordering::Relaxed) / DD7_CODE_N.load(Ordering::Relaxed).max(1)));
            s.push_str(&format!("{:11} {:>16} / {:>9} / {:>7}\n", "dd7_resolve", DD7_RESOLVE_NS.load(Ordering::Relaxed), DD7_RESOLVE_N.load(Ordering::Relaxed), DD7_RESOLVE_NS.load(Ordering::Relaxed) / DD7_RESOLVE_N.load(Ordering::Relaxed).max(1)));
            if let Some(p) = pth("perf.txt") { let _ = fs::write(p, &s); }   // ★LOG_ON 무관 직접write: perf_measure=1이면 log 플래그 없이도 perf.txt 기록(측정 자가완결)
        }
    }
}
#[inline] fn perf_guard(idx: usize) -> Option<PerfGuard> {
    if PERF_ON.load(Ordering::Relaxed) { Some(PerfGuard { idx, t: Instant::now() }) } else { None }
}
static ROLL_LOGGED: AtomicU64 = AtomicU64::new(0);
// ★ChaCha refill 검증: 1421bbc10 리턴훅서 내 재현버퍼(MY_CHACHA) vs 게임버퍼 워드대조. rngcap=1 게이트 공유.
static CHACHA_ARMED: AtomicU64 = AtomicU64::new(0);
static CHACHA_LOGGED: AtomicU64 = AtomicU64::new(0);
static CHACHA_FILE_INIT: AtomicBool = AtomicBool::new(false);
const CHACHA_ARM_MAX: u64 = 64;
static MY_CHACHA: Mutex<[u32; 64]> = Mutex::new([0u32; 64]);
// ★facet#5 역할 교전임계값 튜닝(검증된 값을 cfg로 조정 = 교전 공격성 다이얼). retreat_engage 내 4개 immediate low byte.
// roll<thr→교전(5), roll>=thr→퇴각(-1). thr↑=교전↑. high 3바이트 0이라 low byte만 패치(원자적).
// ★0.4.13: retreat refactor(0x1d474c0, 프레임오프셋 시프트만)됐으나 교전코어(df0c10→역할임계값→roll게이트) 바이트동일 검증(cmp_region.py).
//   RVA = df0c10_call(0x1fe4d33)+{0x40,0x58,0x6c,0x72}. roll게이트(cmp rax,r14;setge;neg;or 5)도 0.4.12와 동일.
const ROLE_THR: [(usize, u8); 4] = [(0x1d3602b, 100), (0x1d36043, 70), (0x1d36058, 50), (0x1d3605d, 30)]; // (imm32 RVA, 원본) 0.4.13_5(was 0x1fd0546/55e/72/78). 인코딩 cmp-imm32→mov-imm 변경: 100/70/30=mov r14d(imm@+2), 50=mov eax(imm@+1). RETREAT 새바디 df0c10콜 직후 역할래더(role4=100/3=70/2=50/else=30). 상위3바이트0 검증 통과
static ENGAGE_THR_MULT: AtomicI64 = AtomicI64::new(100);  // cfg %, 100=원본(검증), 다른값=공격성 조정
static MOVE_TAG: AtomicI64 = AtomicI64::new(1);       // cfg move_tag: 어느 tag를 Move로 볼지
static MOVE_OFF: AtomicI64 = AtomicI64::new(8);       // cfg move_off: x오프셋(y=x+8). 확인후 맞춤
static OV_ENABLED: AtomicBool = AtomicBool::new(false);
static DMGCAP: AtomicBool = AtomicBool::new(false);   // cfg dmgcap=1: 데미지검증(combat vs my_combat_dmg) 비교 ON. 라이브매치 think()서 16샘플 dmgcmp.txt. 기본 OFF(휴면).
static OV_TEAM: AtomicI64 = AtomicI64::new(0);
static OV_X: AtomicU64 = AtomicU64::new(480000);
static OV_Y: AtomicU64 = AtomicU64::new(480000);
static OV_COEF_MULT: AtomicI64 = AtomicI64::new(100); // 데미지 coef(+0xd8) 배수 %. 100=원본
static CFG_MTIME: AtomicU64 = AtomicU64::new(0);
// ★ 캡처 마스터 게이트: cfg capture=1로 켜야 TTD/RE 하네스 무장. 0→1 전환시 카운터·파일 리셋.
// (데모화면 배경전투가 예산 소진하는 문제 해결 — 원하는 경기에서 1로 켜면 그때부터 캡처)
static CAP_ON: AtomicBool = AtomicBool::new(false);

// ── 런타임 캡처 (훅은 raw 값만 저장 = 초경량. 탐지는 메인스레드에서) ──
static CAP_PB: AtomicUsize = AtomicUsize::new(0);       // 진짜 plan_base (로스터 +0x1e0 보유)
static CAP_PB_RAW: AtomicUsize = AtomicUsize::new(0);   // retreat_engage 경로 plan_base 후보 (검증 전)
static CAP_PSTATE: AtomicUsize = AtomicUsize::new(0);   // 플레이어별 plan_state (subplan +0x500)
static CAP_RCX: AtomicUsize = AtomicUsize::new(0);
static CAP_RDX: AtomicUsize = AtomicUsize::new(0);
static CAP_R8: AtomicUsize = AtomicUsize::new(0);
static CAP_R9: AtomicUsize = AtomicUsize::new(0);
static DISP_TID: AtomicU64 = AtomicU64::new(0);
static DISP_HITS: AtomicU64 = AtomicU64::new(0);
static DIAG_DONE: AtomicBool = AtomicBool::new(false);  // plan_base 자동탐지 1회
static DISPREG_DONE: AtomicBool = AtomicBool::new(false); // dispatch 레지스터 진단 1회
// dispatch가 준 확정 plan_state 주소들(distinct) — ground-truth set
static PSTATE_SET: [AtomicUsize; 16] = [const { AtomicUsize::new(0) }; 16];
static PSTATE_CNT: AtomicU64 = AtomicU64::new(0);
static PSTATES_DUMP_DONE: AtomicBool = AtomicBool::new(false);
static AISCAN_DONE: AtomicBool = AtomicBool::new(false);  // AI구조체(param_2)에서 entity 링크 탐색
static CAP_T9_RDX: AtomicUsize = AtomicUsize::new(0);    // FUN_141dd9360 param_2 (AI구조체)
static CAP_T9_R9: AtomicUsize = AtomicUsize::new(0);     // FUN_141dd9360 param_4 (athlete)
static T9_DONE: AtomicBool = AtomicBool::new(false);
static T9_CTR: AtomicU64 = AtomicU64::new(0);   // 맵갱신 스로틀
static EXT_CTR: AtomicU64 = AtomicU64::new(0);  // submap.txt 쓰기 스로틀
static CAP_DRV_RCX: AtomicUsize = AtomicUsize::new(0);
static CAP_DRV_RDX: AtomicUsize = AtomicUsize::new(0);
static CAP_DRV_R8: AtomicUsize = AtomicUsize::new(0);
static CAP_DRV_R9: AtomicUsize = AtomicUsize::new(0);
static CAP_DRV_A5: AtomicUsize = AtomicUsize::new(0);
static DRVDUMP_DONE: AtomicBool = AtomicBool::new(false);
static PSLINK_N: AtomicU64 = AtomicU64::new(0);          // entity→plan_state 링크 탐색 카운터
static LAST_TICK: AtomicU64 = AtomicU64::new(0);         // 새 경기(tick 리셋) 감지용
static VERIFY_N: AtomicU64 = AtomicU64::new(0);
static BOOTED: AtomicBool = AtomicBool::new(false);
const VERIFY_LIMIT: u64 = 10;

type BOOL = i32; type DWORD = u32; type HMODULE = usize;
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(flags: u32, addr: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, sz: DWORD) -> DWORD;
    fn GetCurrentThreadId() -> DWORD;
    fn VirtualAlloc(addr: usize, sz: usize, typ: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old: *mut u32) -> BOOL;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, sz: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
}

// ★exe base 캐시: GetModuleHandleW(null)=exe 이미지베이스. 프로세스 수명 내내 고정(미언로드/미재배치)이라 1회면 충분.
//   매 draw/judge detour서 반복 호출하던 걸 캐시 1회로 대체. 메모리영역 캐싱과 달리 base 불변 → 100% 안전.
static EXE_BASE: AtomicU64 = AtomicU64::new(0);
#[inline] unsafe fn exe_base() -> usize {
    let v = EXE_BASE.load(Ordering::Relaxed) as usize;
    if v != 0 { return v; }
    let b = GetModuleHandleW(core::ptr::null());
    EXE_BASE.store(b as u64, Ordering::Relaxed);
    b
}

#[repr(C)] #[derive(Default)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32,
    region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32 }

// ───────── VEH 안전읽기 (item_editor/scrim 검증본 이식, 2026-06-21 perf 최적화) ─────────
//  rd_*의 per-read VirtualQuery(~1µs syscall)가 judge 핫루프서 호출당 수백회 = dd7700 218µs/call 주범.
//  대안: raw 읽기(rep movsb) + AV는 VEH가 잡아 landing으로 복구(성공경로 syscall 0 ~20ns) → 전 judge 동시 가속.
//  ★캐시 아님(stale 위험 없음): 매 읽기 즉시 실행, 폴트만 VEH로 흡수. cfg fast_read 게이트(off=기존 VirtualQuery).
//  SEH[]: 0=active 1=tid 2=land_rip 3=land_rsp 4=land_rbp 5=code_lo 6=code_hi 7=faults. (멀티모드: 각 모드 자기 SEH범위만 처리.)
#[repr(C)] struct ExceptionRecord { code: u32, _flags: u32, _rec: usize, _addr: usize, _np: u32, _p: u32, _params: [usize; 15] }
#[repr(C)] struct ExceptionPointers { rec: *mut ExceptionRecord, ctx: *mut core::ffi::c_void }
type VehHandler = extern "system" fn(*mut ExceptionPointers) -> i32;
extern "system" { fn AddVectoredExceptionHandler(first: u32, handler: VehHandler) -> usize; }
static mut SEH: [u64; 8] = [0u64; 8];
static SEH_INSTALLED: AtomicBool = AtomicBool::new(false);
static SEH_BUSY: AtomicBool = AtomicBool::new(false);
static FAST_READ: AtomicU8 = AtomicU8::new(0);   // cfg fast_read: 0=VirtualQuery / 1=VEH(spinlock 검증본) / 2=VEH(lockless 최속)
// ★lockless VEH 읽기(level2): load 1개가 함수 첫 명령(스택 무변경) → 폴트시 VEH가 RIP만 land로(스핀락·SEH[]상태·rsp복원 전부 불요).
//   land의 ret가 바로 정상복귀(eax=0=fault). 공유 가변상태 0 = 스레드안전. ~5-8ns/read.
core::arch::global_asm!(
    ".globl pr_rd8", ".globl pr_rd8_f", ".globl pr_rd8_l",
    ".globl pr_rd4", ".globl pr_rd4_f", ".globl pr_rd4_l",
    ".globl pr_rd1", ".globl pr_rd1_f", ".globl pr_rd1_l",
    "pr_rd8:", "pr_rd8_f:", "mov rax, qword ptr [rcx]", "mov qword ptr [rdx], rax", "mov eax, 1", "ret",
    "pr_rd8_l:", "xor eax, eax", "ret",
    "pr_rd4:", "pr_rd4_f:", "mov eax, dword ptr [rcx]", "mov dword ptr [rdx], eax", "mov eax, 1", "ret",
    "pr_rd4_l:", "xor eax, eax", "ret",
    "pr_rd1:", "pr_rd1_f:", "movzx eax, byte ptr [rcx]", "mov byte ptr [rdx], al", "mov eax, 1", "ret",
    "pr_rd1_l:", "xor eax, eax", "ret",
);
extern "C" {
    fn pr_rd8(addr: usize, out: *mut u64) -> u32;
    fn pr_rd4(addr: usize, out: *mut u32) -> u32;
    fn pr_rd1(addr: usize, out: *mut u8) -> u32;
    static pr_rd8_f: u8; static pr_rd8_l: u8;
    static pr_rd4_f: u8; static pr_rd4_l: u8;
    static pr_rd1_f: u8; static pr_rd1_l: u8;
}
#[inline] unsafe fn lr_u64(a: usize) -> Option<u64> { let mut o=0u64; if pr_rd8(a, &mut o)!=0 {Some(o)} else {None} }
#[inline] unsafe fn lr_i32(a: usize) -> Option<i32> { let mut o=0u32; if pr_rd4(a, &mut o)!=0 {Some(o as i32)} else {None} }
#[inline] unsafe fn lr_u8(a: usize)  -> Option<u8>  { let mut o=0u8;  if pr_rd1(a, &mut o)!=0 {Some(o)} else {None} }
// ★lockless VEH 쓰기(B-3, 2026-06-23): pr_rd* 미러. rcx=addr, rdx=value, 성공=eax 1/폴트=eax 0(land). 스택 무변경=land ret 정상복귀, 공유상태 0=스레드안전(rd와 동일 land경로). seh_install 무조건(init).
core::arch::global_asm!(
    ".globl pr_wr8", ".globl pr_wr8_f", ".globl pr_wr8_l",
    ".globl pr_wr4", ".globl pr_wr4_f", ".globl pr_wr4_l",
    ".globl pr_wr1", ".globl pr_wr1_f", ".globl pr_wr1_l",
    "pr_wr8:", "pr_wr8_f:", "mov qword ptr [rcx], rdx", "mov eax, 1", "ret",
    "pr_wr8_l:", "xor eax, eax", "ret",
    "pr_wr4:", "pr_wr4_f:", "mov dword ptr [rcx], edx", "mov eax, 1", "ret",
    "pr_wr4_l:", "xor eax, eax", "ret",
    "pr_wr1:", "pr_wr1_f:", "mov byte ptr [rcx], dl", "mov eax, 1", "ret",
    "pr_wr1_l:", "xor eax, eax", "ret",
);
extern "C" {
    fn pr_wr8(addr: usize, val: u64) -> u32;
    fn pr_wr4(addr: usize, val: u32) -> u32;
    fn pr_wr1(addr: usize, val: u8) -> u32;
    static pr_wr8_f: u8; static pr_wr8_l: u8;
    static pr_wr4_f: u8; static pr_wr4_l: u8;
    static pr_wr1_f: u8; static pr_wr1_l: u8;
}
#[inline] unsafe fn lw_u64(a: usize, v: u64) -> bool { pr_wr8(a, v) != 0 }
#[inline] unsafe fn lw_u32(a: usize, v: u32) -> bool { pr_wr4(a, v) != 0 }
#[allow(dead_code)] #[inline] unsafe fn lw_u8(a: usize, v: u8) -> bool { pr_wr1(a, v) != 0 }
// ★쓰기 디스패처(rd_* 미러): fast_read=2=lockless VEH(최속,VQ0) / 0·1=writable VQ+raw(폴백). 둘다 AV-safe. 합법주소=동일write·불법=무쓰기 → writable가드와 동의미(valid sim서 비트동일).
unsafe fn wr_u64(a: usize, v: u64) -> bool { if a < 0x10000 { return false; } match FAST_READ.load(Ordering::Relaxed) { 2 => lw_u64(a, v), 3 => { std::ptr::write_unaligned(a as *mut u64, v); true }, _ => if writable(a, 8) { std::ptr::write_unaligned(a as *mut u64, v); true } else { false } } }
unsafe fn wr_u32(a: usize, v: u32) -> bool { if a < 0x10000 { return false; } match FAST_READ.load(Ordering::Relaxed) { 2 => lw_u32(a, v), 3 => { std::ptr::write_unaligned(a as *mut u32, v); true }, _ => if writable(a, 4) { std::ptr::write_unaligned(a as *mut u32, v); true } else { false } } }
#[allow(dead_code)]
unsafe fn wr_u8(a: usize, v: u8) -> bool { if a < 0x10000 { return false; } match FAST_READ.load(Ordering::Relaxed) { 2 => lw_u8(a, v), 3 => { std::ptr::write_unaligned(a as *mut u8, v); true }, _ => if writable(a, 1) { std::ptr::write_unaligned(a as *mut u8, v); true } else { false } } }

// ★읽기 경로 직접 벤치(cfg read_bench=1, 1회): 확실히 readable한 주소를 각 경로로 N회 읽어 ns/read 측정.
//   게임 페이즈 무관 = perf.txt(첫50000콜창)의 경기페이즈 오염 없이 원시 per-read 비용 ground-truth.
static BENCH_DONE: AtomicBool = AtomicBool::new(false);
unsafe fn bench_reads() {
    if BENCH_DONE.swap(true, Ordering::Relaxed) { return; }
    let probe = core::ptr::addr_of!(SEH) as usize;   // 우리 static = 확실히 읽기가능
    let n = 300_000u64;
    let mut acc = 0u64;
    let t = Instant::now(); for _ in 0..n { if readable(probe,8) { acc = acc.wrapping_add(std::ptr::read_unaligned(probe as *const u64)); } } let vq = t.elapsed().as_nanos() as u64 / n;
    let t = Instant::now(); for _ in 0..n { acc = acc.wrapping_add(safe_rd_u64(probe).unwrap_or(0)); } let l1 = t.elapsed().as_nanos() as u64 / n;
    let t = Instant::now(); for _ in 0..n { acc = acc.wrapping_add(lr_u64(probe).unwrap_or(0)); } let l2 = t.elapsed().as_nanos() as u64 / n;
    write_named("readbench.txt", &format!("=== read path 벤치 (acc={} reads/path={}) ===\nVirtualQuery(level0): {} ns/read\nVEH spinlock(level1): {} ns/read\nVEH lockless(level2): {} ns/read\n", acc, n, vq, l1, l2));
}
extern "system" fn seh_veh(p: *mut ExceptionPointers) -> i32 {
    const CONTINUE_EXECUTION: i32 = -1; const CONTINUE_SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return CONTINUE_SEARCH; }
        let rec = (*p).rec;
        if rec.is_null() || (*rec).code != 0xC0000005 { return CONTINUE_SEARCH; }   // ACCESS_VIOLATION만
        let ctx = (*p).ctx as usize;
        if ctx == 0 { return CONTINUE_SEARCH; }
        let rip = *((ctx + 0xF8) as *const u64) as usize;                           // CONTEXT.Rip@0xF8
        // ── lockless 읽기(level2): 고정 load주소 → land로 (상태/rsp복원 불요, 스택 무변경이라 land의 ret가 정상복귀) ──
        let land = if rip == core::ptr::addr_of!(pr_rd8_f) as usize { core::ptr::addr_of!(pr_rd8_l) as usize }
                   else if rip == core::ptr::addr_of!(pr_rd4_f) as usize { core::ptr::addr_of!(pr_rd4_l) as usize }
                   else if rip == core::ptr::addr_of!(pr_rd1_f) as usize { core::ptr::addr_of!(pr_rd1_l) as usize }
                   else if rip == core::ptr::addr_of!(pr_wr8_f) as usize { core::ptr::addr_of!(pr_wr8_l) as usize }   // ★B-3 쓰기 land(읽기와 동일 경로)
                   else if rip == core::ptr::addr_of!(pr_wr4_f) as usize { core::ptr::addr_of!(pr_wr4_l) as usize }
                   else if rip == core::ptr::addr_of!(pr_wr1_f) as usize { core::ptr::addr_of!(pr_wr1_l) as usize }
                   else { 0 };
        if land != 0 { *((ctx + 0xF8) as *mut u64) = land as u64; return CONTINUE_EXECUTION; }
        // ── spinlock 읽기(level1): SEH[] 활성 + 우리스레드 + 우리 asm범위 ──
        let g = core::ptr::addr_of!(SEH) as *const u64;
        if *g.add(0) == 0 { return CONTINUE_SEARCH; }                                // inactive
        if *g.add(1) != GetCurrentThreadId() as u64 { return CONTINUE_SEARCH; }      // 다른 스레드
        if (rip as u64) < *g.add(5) || (rip as u64) >= *g.add(6) { return CONTINUE_SEARCH; }  // 범위 밖
        *((ctx + 0xF8) as *mut u64) = *g.add(2);                                     // Rip → land
        *((ctx + 0x98) as *mut u64) = *g.add(3);                                     // Rsp 복원
        *((ctx + 0xA0) as *mut u64) = *g.add(4);                                     // Rbp 복원
        let gm = core::ptr::addr_of_mut!(SEH) as *mut u64;
        *gm.add(7) += 1;
        CONTINUE_EXECUTION
    }
}
fn seh_install() { if SEH_INSTALLED.swap(true, Ordering::Relaxed) { return; } unsafe { AddVectoredExceptionHandler(1, seh_veh); } }
#[inline(never)]
unsafe fn safe_copy(dst: *mut u8, src: *const u8, len: usize) -> bool {
    if !SEH_INSTALLED.load(Ordering::Relaxed) { return false; }
    while SEH_BUSY.swap(true, Ordering::Acquire) { core::hint::spin_loop(); }
    let g = core::ptr::addr_of_mut!(SEH) as *mut u64;
    *g.add(1) = GetCurrentThreadId() as u64;
    let ok: u64;
    core::arch::asm!(
        "lea rax, [rip + 200f]", "mov [{g} + 40], rax",   // code_lo = SEH[5]
        "lea rax, [rip + 201f]", "mov [{g} + 48], rax",   // code_hi = SEH[6]
        "lea rax, [rip + 202f]", "mov [{g} + 16], rax",   // land_rip = SEH[2]
        "mov [{g} + 24], rsp", "mov [{g} + 32], rbp",      // land_rsp/rbp = SEH[3]/[4]
        "mov qword ptr [{g} + 0], 1", "cld",               // active
        "200:", "rep movsb", "201:", "mov {ok}, 1", "jmp 203f",
        "202:", "mov {ok}, 0", "203:", "mov qword ptr [{g} + 0], 0",   // inactive
        g = in(reg) g, ok = out(reg) ok,
        inout("rcx") len => _, inout("rdi") dst => _, inout("rsi") src => _, out("rax") _,
    );
    SEH_BUSY.store(false, Ordering::Release);
    ok != 0
}
#[inline] unsafe fn safe_rd_u64(a: usize) -> Option<u64> { let mut b=[0u8;8]; if safe_copy(b.as_mut_ptr(), a as *const u8, 8){Some(u64::from_le_bytes(b))}else{None} }
#[inline] unsafe fn safe_rd_i32(a: usize) -> Option<i32> { let mut b=[0u8;4]; if safe_copy(b.as_mut_ptr(), a as *const u8, 4){Some(i32::from_le_bytes(b))}else{None} }
#[inline] unsafe fn safe_rd_u8(a: usize) -> Option<u8> { let mut b=[0u8;1]; if safe_copy(b.as_mut_ptr(), a as *const u8, 1){Some(b[0])}else{None} }

// ⚠ readable/writable는 매 호출 VirtualQuery (point-in-time 검증). 이걸 영역캐시로 가속하려던 시도는
//   ★실패(2026-06-21): commit 영역 [s,e)를 캐시했다가 게임이 그 영역 sub-page를 나중에 decommit(힙
//   sub-block free)하면 캐시가 여전히 [s,e) 전체를 readable로 보증 → judge가 decommit된 주소 읽기 → AV.
//   경기 시작 직후(alloc/free 격렬, ready_ticks~31) 크래시 실측. ⟹ VirtualQuery 결과는 시간을 가로질러
//   캐시 불가(thread_local/Mutex 무관). 가속은 캐시 대신 '읽기 횟수 자체'를 줄여서(객체범위 1회검증+raw읽기,
//   같은 동기 스코프) 달성 — 예: dd7_slot_a8. readable/writable 본체는 원본(매콜 VirtualQuery) 유지.
unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const RD: u32 = 0x02|0x04|0x20|0x40; const GUARD: u32 = 0x01|0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}
// ★쓰기가능 검사(대체모드 RNG 되쓰기 안전가드): PAGE_READWRITE/WRITECOPY/EXECUTE_READWRITE/EXECUTE_WRITECOPY.
unsafe fn writable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const WR: u32 = 0x04|0x08|0x40|0x80; const GUARD: u32 = 0x01|0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & WR == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}
// ★ 연속 바이트범위를 1회 VirtualQuery로 검증(핫루프서 per-field readable 폭증 회피용).
//   검증 성공시 [addr,addr+len)이 한 commit-readable 영역 안 → 그 범위 내 raw read_unaligned 안전(같은 동기 스코프).
//   ⚠캐시 아님: 매 호출 즉시 검증(시간을 가로지르지 않음=stale 위험 없음).
#[inline] unsafe fn rd_span_ok(addr: usize, len: usize) -> bool { readable(addr, len) }
// ★fast_read: 2=lockless VEH(최속) / 1=spinlock VEH(검증본) / 0=VirtualQuery. 셋다 AV-safe·동일결과.
unsafe fn rd_u64(a: usize) -> Option<u64> { if a < 0x10000 { return None; } match FAST_READ.load(Ordering::Relaxed) { 2 => lr_u64(a), 3 => Some(std::ptr::read_unaligned(a as *const u64)), 1 => safe_rd_u64(a), _ => if readable(a,8){Some(std::ptr::read_unaligned(a as *const u64))}else{None} } }
unsafe fn rd_i64(a: usize) -> Option<i64> { if a < 0x10000 { return None; } match FAST_READ.load(Ordering::Relaxed) { 2 => lr_u64(a).map(|v| v as i64), 3 => Some(std::ptr::read_unaligned(a as *const i64)), 1 => safe_rd_u64(a).map(|v| v as i64), _ => if readable(a,8){Some(std::ptr::read_unaligned(a as *const i64))}else{None} } }
unsafe fn rd_i32(a: usize) -> Option<i32> { if a < 0x10000 { return None; } match FAST_READ.load(Ordering::Relaxed) { 2 => lr_i32(a), 3 => Some(std::ptr::read_unaligned(a as *const i32)), 1 => safe_rd_i32(a), _ => if readable(a,4){Some(std::ptr::read_unaligned(a as *const i32))}else{None} } }
unsafe fn rd_u8(a: usize) -> u8 { if a < 0x10000 { return 0; } match FAST_READ.load(Ordering::Relaxed) { 2 => lr_u8(a).unwrap_or(0), 3 => std::ptr::read_unaligned(a as *const u8), 1 => safe_rd_u8(a).unwrap_or(0), _ => if readable(a,1){std::ptr::read_unaligned(a as *const u8)}else{0} } }
// ★디스패치 루프 candEnt 유효성 (decomp 474-478): candEnt=*(rh+0x180+team*8+type*0x20)(0이면 +0x190),
//   유효 = candEnt!=0 && *(candEnt+0x68)i32==2 && *(candEnt+0x128)byte>4. -1=가드스킵.
unsafe fn cand_ent_valid(rh: usize, team: i64, ty: i64) -> i32 {
    if team<0 || team>1 || ty<0 || !ptr_ok(rh) { return -1; }
    let (tm, t) = (team as usize, ty as usize);
    let mut ce = rd_u64(rh + 0x180 + tm*8 + t*0x20).unwrap_or(0) as usize;
    if ce == 0 { ce = rd_u64(rh + 0x190 + tm*8 + t*0x20).unwrap_or(0) as usize; }
    if !ptr_ok(ce) { return 0; }   // ★성능(B-1): readable 가드 제거 — 본문 rd_i32(ce+0x68)/rd_u8(ce+0x128)이 fault-safe(None→-1/0). 불가독시 -1!=2 false → 0반환 = 가드의 0반환과 비트동일.
    if rd_i32(ce+0x68).unwrap_or(-1)==2 && (rd_u8(ce+0x128) as i64) > 4 { 1 } else { 0 }
}
// ★STAND vs ZONE 분기로직 (decomp 959-1022). zone=GEO+team*0x228. true=STAND-attempt(→8), false=ZONE(→3).
//   slot k: lock@zone+0xf8+k*0x20, type@zone+0xf9+k*0x20. postag=cVar4∈{0,2}. za48/za20/za70=alive i32.
unsafe fn dispatch_stand_attempt(zone: usize, postag: i64, za20: i64, za48: i64, za70: i64) -> bool {
    if !ptr_ok(zone) || !readable(zone+0x178+1, 1) { return false; }
    let l0 = rd_u8(zone+0xf8) as i64; let t0 = rd_u8(zone+0xf9) as i64;
    // fab7ba: (∃ slot1-4: lock==0 && type==1) OR za48 > -3
    let mut type1 = false;
    for k in 1..5usize { if rd_u8(zone+0xf8+k*0x20) as i64==0 && rd_u8(zone+0xf9+k*0x20) as i64==1 { type1=true; break; } }
    let fab7ba = type1 || za48 > -3;
    if l0==0 && t0==postag { return fab7ba; }   // slot0 matches postag → branch1
    // else branch2
    let mut reached = false;
    for k in 1..5usize { if rd_u8(zone+0xf8+k*0x20) as i64==0 && rd_u8(zone+0xf9+k*0x20) as i64==postag { reached=true; break; } }
    if !reached { let alive = if postag==0 { za20 } else { za70 }; if alive > -3 { reached=true; } }
    if !reached { return false; }   // → ZONE
    // LAB_141fab7a0: slot0 re-check
    if l0 != 0 { return fab7ba; }
    if t0 != 1 { return fab7ba; }
    true   // fab883 직접 → STAND-attempt
}
// ★fa1ea0(FUN_1420676c0) 순수재현 — STAND-attempt시 액션큐에 유효행동 있나. ≠0xff(매치)면 STAND, ==0xff면 교전롤.
//   원본 shadow는 게임 fa1ea0(RVA_FA1EA0=churn)을 q={0,2},acts={1,postag}로 호출했음 →
//   ① 매버전 주소이동(churn) ② 한타(큐 non-empty)서 가드없는 deref로 세그폴트 위험.
//   ⟹ my_fa1ea0로 완전대체(전부 guarded read; 게임함수콜=def_resolve(vt+0x140 런타임resolve, churn無)뿐).
//   앵커 DAT: DAT_1435eef60[a]/DAT_1435eef78[a] (a=action byte∈{0,1,2}). exe서 추출(const).
const FA_ANC60: [u64; 3] = [820000, 817000, 880000];   // DAT_1435eef60
const FA_ANC78: [u64; 3] = [80000, 144000, 144000];    // DAT_1435eef78
// 액션큐 [1, postag]에 대해 하나라도 매치하면 true(게임 ≠0xff), 없으면 false(0xff).
unsafe fn my_fa1ea0(rh: usize, geo: usize, p5: usize, postag: i64) -> bool {
    if !ptr_ok(rh) || !ptr_ok(geo) || !ptr_ok(p5) { return false; }
    let team = rd_u64(p5 + 0x6a8).unwrap_or(2);
    if team > 1 { return false; }              // 게임 bounds-panic 회피(매치중 0/1)
    let tu = team as usize;
    let rhd0 = rd_u64(rh).unwrap_or(0) as usize;       // *puVar5 = resolve this
    let rhd1 = rd_u64(rh + 8).unwrap_or(0) as usize;   // puVar5[1] = resolve vtable
    // 큐: acts[0]=1, acts[1]=postag (shadow의 q={0,2})
    for &action in [1i64, postag].iter() {
        if my_fa1ea0_one(rh, rhd0, rhd1, geo, tu, action & 0xff) { return true; }
    }
    false
}
// fa1ea0 1회 루프바디 — true=이 액션 매치(decomp goto LAB_1420676df).
unsafe fn my_fa1ea0_one(rh: usize, rhd0: usize, rhd1: usize, geo: usize, tu: usize, action: i64) -> bool {
    let a = (action & 0xff) as usize;
    if a > 2 { return false; }
    // ① 1차 웨이포인트 lv13 (puVar5[team+0x30/0x34/0x38]; 0이면 byte오프셋 fallback)
    let (prim_idx, prim_off) = match action { 0 => (0x30usize, 0x190usize), 1 => (0x34, 0x1b0), _ => (0x38, 0x1d0) };
    let mut lv13 = rd_u64(rh + (tu + prim_idx)*8).unwrap_or(0);
    if lv13 == 0 { lv13 = rd_u64(rh + tu*8 + prim_off).unwrap_or(0); }
    // ② 웨이포인트 선택(count=puVar5[team*4+0x29]; lv13 우선, 아니면 nearest-loop/배열0번)
    let count = rd_u64(rh + (tu*4 + 0x29)*8).unwrap_or(0);
    let mut wp = if lv13 != 0 { lv13 }
        else if count == 0 { 0 }
        else {
            let arr = rd_u64(rh + (tu*4 + 0x26)*8).unwrap_or(0) as usize;
            if count == 1 { if ptr_ok(arr) { rd_u64(arr).unwrap_or(0) } else { 0 } }
            else { fa_nearest(arr, count, tu, a) }
        };
    if wp == 0 { wp = rd_u64(rh + (tu + 0x2e)*8).unwrap_or(0); }   // puVar5[team+0x2e] fallback
    if wp == 0 { return false; }                                   // 게임 panic(FUN_1429404e0) → 매치불가
    let wp = wp as usize;
    // ③ zone 슬롯 iVar2 (action별 +0/0x28/0x50), handle=*(slot+8)
    let slot_off = match action { 0 => 0usize, 1 => 0x28, _ => 0x50 };
    let zbase = geo + tu*0x228;
    if rd_i32(zbase + slot_off).unwrap_or(0) != 1 { return false; }
    let handle = rd_u64(zbase + slot_off + 8).unwrap_or(0);
    if rhd0 == 0 || rhd1 == 0 { return false; }
    let tgt = def_resolve(rhd0, rhd1, handle);          // (*(rhd[1]+0x140))(rhd[0], handle) — 런타임resolve(churn無)
    if tgt == 0 { return false; }
    // ④ 웨이포인트-타겟 거리² >>6 > 0x1c8591a8 (멀어야 zone 디스패치 후보). ★B-2: readable VQ→rd_u64 None=false(폴트세이프, 동의미)
    let ty = match rd_u64(tgt+0x650) { Some(v) => v, None => return false };
    let wy = match rd_u64(wp+0x650)  { Some(v) => v, None => return false };
    let (tx, wx) = (rd_u64(tgt+0x648).unwrap_or(0), rd_u64(wp+0x648).unwrap_or(0));
    if (sqd(wx, wy, tx, ty) >> 6) <= 0x1c8591a8 { return false; }
    // ⑤ 5 zone 서브슬롯: lock@+0xf8+k*0x20==0 && type@+0xf9==action && sub=puVar5[team*5+0x3c+k]!=0
    //    && 거리²(tgt,sub) < 0x17d784001 → 매치(STAND)
    for k in 0..5usize {
        if rd_u8(zbase + 0xf8 + k*0x20) as i64 != 0 { continue; }
        if rd_u8(zbase + 0xf9 + k*0x20) as i64 != action { continue; }
        let sub = rd_u64(rh + (tu*5 + 0x3c + k)*8).unwrap_or(0) as usize;
        if sub == 0 { continue; }
        let sy = match rd_u64(sub+0x650) { Some(v) => v, None => continue };   // ★B-2: readable VQ→rd_u64
        let sx = rd_u64(sub+0x648).unwrap_or(0);
        if sqd(tx, ty, sx, sy) < 0x17d784001 { return true; }
    }
    false
}
// nearest-loop: 앵커(team별 eef60/eef78 X·Y 교차)에 거리² 최소인 배열원소 반환. lv13!=0이면 미호출(결과 무시됨).
unsafe fn fa_nearest(arr: usize, count: u64, tu: usize, a: usize) -> u64 {
    if !ptr_ok(arr) || a > 2 { return 0; }
    let (ax, ay) = if tu == 1 { (FA_ANC60[a], FA_ANC78[a]) } else { (FA_ANC78[a], FA_ANC60[a]) };
    let dist_at = |p: usize| -> u64 {
        let e = rd_u64(p).unwrap_or(0) as usize;
        match rd_u64(e+0x650) { Some(ey) => sqd(ax, ay, rd_u64(e+0x648).unwrap_or(0), ey), None => u64::MAX }   // ★B-2: readable VQ→rd_u64
    };
    let mut best_ptr = arr;
    let mut best_d = dist_at(arr);
    let iters = (count - 1) & 0x1fffffffffffffff;
    let mut p = arr + 8; let mut i = 0u64;
    while i < iters { let d = dist_at(p); if d < best_d { best_ptr = p; best_d = d; } p += 8; i += 1; }
    rd_u64(best_ptr).unwrap_or(0)
}
// ★fa1ea0 STAND-attempt 판정 = my_fa1ea0 순수재현(288/288 DIFF0 검증완료, game vs mine true/false·team0/1 일치).
//   게임 fa1ea0 콜·RVA_FA1EA0·fa1cmp 대조 스캐폴드는 검증 후 제거됨(2026-06-19, churn 소멸 + 세그폴트 위험 영구제거).
#[inline] unsafe fn shadow_fa1ea0(rh: usize, geo: usize, p5: usize, postag: i64) -> bool {
    my_fa1ea0(rh, geo, p5, postag)
}
// ★통합 디스패치 코드 예측: 7=RECALL/8=STAND/3=ZONE, -99=교전롤/none.
//   candEnt유효 → 디스패치루프 recall(7). cVar6==1 → post-loop recall(7, fc59a0 RNG게이트 미반영=TODO).
//   cVar6==0 → STAND-attempt면 fa1ea0≠0xff?8:roll(-99), 아니면 ZONE(3). cVar6==2 battle-poke → roll(-99).
unsafe fn my_dispatch_code(cvar6: i64, cept: i32, ce1: i32, zone: usize, postag: i64, za20: i64, za48: i64, za70: i64, rh: usize, geo: usize, p5: usize) -> i64 {
    if cept==1 || ce1==1 || cvar6==1 { return 7; }
    if cvar6==0 {
        if dispatch_stand_attempt(zone, postag, za20, za48, za70) {
            return if shadow_fa1ea0(rh, geo, p5, postag) { 8 } else { -99 };
        }
        return 3;
    }
    -99   // cVar6==2(battle-poke→roll) / 기타
}
// ── 시드 PRNG(rand-0.8.5 ChaCha12 StdRng) 재현 ──
// 상태 레이아웃(FUN_141fcdaf0/1421bbc10 디컴파일): byte 0..0x100 = 출력버퍼(64×u32),
//   *(state+0x100) = idx(4바이트 단위, 0..0x40; >=0x3f면 refill 필요). 각 draw = u64(2워드), idx+=2.
// read-only 시뮬: 게임 상태 안 건드리고 로컬 idx 추적. refill 경계(idx>=0x3f)는 1단계 미지원(None).
// next_u64: 로컬버퍼/idx서 u64. idx>=0x3f면 내 ChaCha12로 버퍼 재생성(refill). FUN_141fcdaf0 엣지 그대로:
//   idx<0x3f: buf[idx]|buf[idx+1]<<32, idx+=2. idx==0x3f: old=buf[0x3f]; refill; (new buf[0]<<32)|old, idx=1.
//   idx>=0x40: refill; buf[0]|buf[1]<<32, idx=2. refill counter = *(input+0x20) + 4*refills.
// ★레버1(lazy on-demand): buf를 매 draw마다 64워드 통째복사(rd_u32×64) 안 함 → refill 전(refills==0)엔
//   필요한 워드만 state서 직접 rd_u32, refill 후(refills>0)엔 로컬 buf(chacha결과)서. 비트동일(같은워드·순서·refill경계).
//   draw당 rd_u32 64회→~2회. state=RNG버퍼 베이스(워드 j = state+j*4).
unsafe fn rng_next_u64(buf: &mut [u32;64], idx: &mut u64, refills: &mut u64, input: usize, state: usize) -> Option<u64> {
    let i = *idx;
    if i < 0x3f {
        let (w0, w1) = if *refills == 0 { (rd_u32(state + (i as usize)*4), rd_u32(state + (i as usize + 1)*4)) }
                       else { (buf[i as usize], buf[i as usize + 1]) };
        *idx = i + 2; return Some((w0 as u64) | ((w1 as u64) << 32));
    }
    let mut key = [0u32; 8]; for k in 0..8 { key[k] = rd_u32(input + k*4); }
    let counter = rd_u64(input + 0x20)?; let nonce = rd_u64(input + 0x28).unwrap_or(0);
    let base = counter.wrapping_add(4u64.wrapping_mul(*refills));
    let old63 = (if *refills == 0 { rd_u32(state + 0x3f*4) } else { buf[0x3f] }) as u64;
    chacha12_4block(&key, base, nonce, buf.as_mut_ptr());   // ★레버3: 4블록 refill을 SIMD 1회(또는 스칼라 fallback)
    *refills += 1;
    if i == 0x3f { *idx = 1; Some(((buf[0] as u64) << 32) | old63) }
    else { *idx = 2; Some((buf[0] as u64) | ((buf[1] as u64) << 32)) }
}
// gen_range(state, lo, hi) = Lemire widening-multiply 거부샘플링(FUN_141fcd980 signed, refill 지원).
// 전부 unsigned wrapping(signed 범위 lo>hi(unsigned)여도 wrapping; lo>hi bail 금지). state=RNG상태(버퍼@0,idx@0x100,input@0x110).
unsafe fn rng_gen_range(state: usize, lo: u64, hi: u64, draws: &mut u32) -> Option<u64> {
    let mut buf = [0u32; 64];   // ★레버1: refill 캐시용(초기 미사용, rng_next_u64가 lazy로 채움)
    let mut idx = rd_u64(state + 0x100)?;
    let mut refills = 0u64;
    let input = state + 0x110;
    let range = hi.wrapping_sub(lo).wrapping_add(1);
    if range == 0 { *draws = 1; return rng_next_u64(&mut buf, &mut idx, &mut refills, input, state); }
    let bits = 63 - range.leading_zeros() as u64;
    let zone = (range << (63 - bits) as u32).wrapping_sub(1);
    let mut guard = 0;
    loop {
        guard += 1; if guard > 64 { return None; }
        *draws += 1;
        let raw = rng_next_u64(&mut buf, &mut idx, &mut refills, input, state)?;
        let prod = (raw as u128).wrapping_mul(range as u128);
        if zone < prod as u64 { continue; }
        return Some(lo.wrapping_add((prod >> 64) as u64));
    }
}
// ★write-back용: gen_range 결과 + 최종(idx, refills) 반환(rng_gen_range와 동일 로직, 상태전이 노출).
//   대체모드 RNG 동기화: 예측 after-state(idx, counter+4*refills) 산출 → 검증/되쓰기.
unsafe fn rng_gen_range_st(state: usize, lo: u64, hi: u64, draws: &mut u32) -> Option<(u64, u64, u64)> {
    let mut buf = [0u32; 64];   // ★레버1: lazy(refill 캐시용)
    let mut idx = rd_u64(state + 0x100)?;
    let mut refills = 0u64;
    let input = state + 0x110;
    let range = hi.wrapping_sub(lo).wrapping_add(1);
    if range == 0 { *draws = 1; let v = rng_next_u64(&mut buf, &mut idx, &mut refills, input, state)?; return Some((v, idx, refills)); }
    let bits = 63 - range.leading_zeros() as u64;
    let zone = (range << (63 - bits) as u32).wrapping_sub(1);
    let mut guard = 0;
    loop {
        guard += 1; if guard > 64 { return None; }
        *draws += 1;
        let raw = rng_next_u64(&mut buf, &mut idx, &mut refills, input, state)?;
        let prod = (raw as u128).wrapping_mul(range as u128);
        if zone < prod as u64 { continue; }
        return Some((lo.wrapping_add((prod >> 64) as u64), idx, refills));
    }
}
// ★write-back: 게임 RNG state를 fcd980과 동일하게 전진(되쓰기). 대체모드 RNG 동기화 핵심. 게임함수 콜 0.
//   read-only sim으로 최종 buf/idx/refills 구한 뒤 buf(refill시)+idx+counter 되쓰기. (step3 대체서 사용; step2 검증선 미호출)
unsafe fn rng_advance_writeback(state: usize, lo: u64, hi: u64) -> Option<u64> {
    // ★레버2(이중시뮬 제거): lazy rng_next_u64를 단일 패스로 돌려 result+최종 buf+idx+refills 동시획득
    //   (기존엔 idx 구한 뒤 buf 위해 chacha 20R×4를 처음부터 재시뮬). buf는 refill 시 rng_next_u64가 채움 → 그대로 writeback.
    // ★B-3: writable VQ가드 제거 → wr_*(폴트세이프). valid sim서 비트동일(합법=동일write, 불법=무쓰기+None).
    let input = state + 0x110;
    let before_counter = rd_u64(input + 0x20)?;
    let mut buf = [0u32; 64];
    let mut idx = rd_u64(state + 0x100)?;
    let mut refills = 0u64;
    let range = hi.wrapping_sub(lo).wrapping_add(1);
    let result = if range == 0 {
        rng_next_u64(&mut buf, &mut idx, &mut refills, input, state)?
    } else {
        let bits = 63 - range.leading_zeros() as u64;
        let zone = (range << (63 - bits) as u32).wrapping_sub(1);
        let mut g = 0;
        loop {
            g += 1; if g > 64 { return None; }
            let raw = rng_next_u64(&mut buf, &mut idx, &mut refills, input, state)?;
            let prod = (raw as u128).wrapping_mul(range as u128);
            if zone < prod as u64 { continue; }
            break lo.wrapping_add((prod >> 64) as u64);
        }
    };
    if refills > 0 {
        for i in 0..64 { if !wr_u32(state + i*4, buf[i]) { return None; } }
        if !wr_u64(input + 0x20, before_counter.wrapping_add(4u64.wrapping_mul(refills))) { return None; }
    }
    if !wr_u64(state + 0x100, idx) { return None; }
    Some(result)
}
// ★u32 gen_range write-back (recall fc59a0용). rng_gen_range_u32와 동일 메커니즘(1 u32워드/draw, idx+=1, refill at idx>=0x40→idx=0, counter+=4). idx+counter(+refill시 buf) 되쓰기 → 게임 state 전진. 반환=sample(lo+high32).
unsafe fn rng_advance_writeback_u32(state: usize, lo: i64, range: u64) -> Option<i64> {
    if range == 0 { return None; }   // ★B-3: writable VQ가드 제거(아래 wr_*가 폴트세이프, 비트동일)
    let mut buf = [0u32; 64];   // ★레버1: lazy(refill 캐시용)
    let mut idx = rd_u64(state + 0x100)? as usize;
    let mut refills = 0u64;
    let input = state + 0x110;
    let before_counter = rd_u64(input + 0x20)?;
    let mut key = [0u32; 8]; for k in 0..8 { key[k] = rd_u32(input + k*4); }
    let nonce = rd_u64(input + 0x28).unwrap_or(0);
    let mut iv: i32 = 0x1f; while (range >> iv) == 0 { iv -= 1; if iv < 0 { return None; } }
    let shift = ((!iv) & 0x1f) as u32;
    let zone = ((range << shift).wrapping_sub(1)) & 0xffff_ffff;
    let mut guard = 0;
    let result = loop {
        guard += 1; if guard > 256 { return None; }
        if idx >= 0x40 {
            let base = before_counter.wrapping_add(4u64.wrapping_mul(refills));
            chacha12_4block(&key, base, nonce, buf.as_mut_ptr());   // ★레버3: 4블록 refill SIMD
            refills += 1; idx = 0;
        }
        let raw = (if refills == 0 { rd_u32(state + idx*4) } else { buf[idx] }) as u64; idx += 1;   // ★레버1: refill 전엔 state 직접
        let prod = raw.wrapping_mul(range);
        if zone < (prod & 0xffff_ffff) { continue; }
        break lo + (prod >> 32) as i64;
    };
    if refills > 0 {
        for i in 0..64 { if !wr_u32(state + i*4, buf[i]) { return None; } }
        if !wr_u64(input + 0x20, before_counter.wrapping_add(4u64.wrapping_mul(refills))) { return None; }
    }
    if !wr_u64(state + 0x100, idx as u64) { return None; }
    Some(result)
}
// 다중 draw용 상태유지 RNG 시뮬(f22e80처럼 한 함수서 여러번 gen_range). 게임상태 무변조(로컬 buf/idx/refills).
// ★레버1: state 보관해 buf 64복사 없이 lazy. buf는 refill 시에만 채워짐.
struct RngSim { buf: [u32;64], idx: u64, refills: u64, input: usize, state: usize }
impl RngSim {
    unsafe fn new(state: usize) -> Option<RngSim> {
        let idx = rd_u64(state+0x100)?;
        Some(RngSim{ buf:[0u32;64], idx, refills:0, input: state+0x110, state })
    }
    unsafe fn gen_range(&mut self, lo: u64, hi: u64) -> Option<u64> {
        let range = hi.wrapping_sub(lo).wrapping_add(1);
        if range==0 { return rng_next_u64(&mut self.buf,&mut self.idx,&mut self.refills,self.input,self.state); }
        let bits = 63 - range.leading_zeros() as u64;
        let zone = (range << (63-bits) as u32).wrapping_sub(1);
        let mut g=0;
        loop { g+=1; if g>64 { return None; }
            let raw = rng_next_u64(&mut self.buf,&mut self.idx,&mut self.refills,self.input,self.state)?;
            let prod=(raw as u128).wrapping_mul(range as u128);
            if zone < prod as u64 { continue; }
            return Some(lo.wrapping_add((prod>>64) as u64));
        }
    }
}
// ── fc59a0식 u32 gen_range: 1 u32워드/draw, raw32*range, 32비트 Lemire rejection. read-only(게임 RNG 무변조). ──
//   rand-0.8.5 next_u32 경로(rng_next_u64는 2워드 u64경로라 별개). 반환 = lo + sample(∈[0,range)). range=0→None(빈범위).
unsafe fn rng_gen_range_u32(state: usize, lo: i64, range: u64) -> Option<i64> {
    if range == 0 { return None; }
    let mut buf = [0u32; 64];   // ★레버1: lazy(refill 캐시용)
    let mut idx = rd_u64(state + 0x100)? as usize;
    let mut refills = 0u64;
    let input = state + 0x110;
    let mut key = [0u32; 8]; for k in 0..8 { key[k] = rd_u32(input + k*4); }
    let counter = rd_u64(input + 0x20)?; let nonce = rd_u64(input + 0x28).unwrap_or(0);
    // MSB 위치 (게임: iVar12=0x1f; for(;R>>iVar12==0;iVar12--))
    let mut iv: i32 = 0x1f;
    while (range >> iv) == 0 { iv -= 1; if iv < 0 { return None; } }
    let shift = ((!iv) & 0x1f) as u32;                       // = 31 - iv
    let zone = ((range << shift).wrapping_sub(1)) & 0xffff_ffff;
    let mut guard = 0;
    loop {
        guard += 1; if guard > 256 { return None; }
        if idx >= 0x40 {                                      // refill (ChaCha 4블록), idx=0
            let base = counter.wrapping_add(4u64.wrapping_mul(refills));
            chacha12_4block(&key, base, nonce, buf.as_mut_ptr());   // ★레버3: 4블록 refill SIMD
            refills += 1; idx = 0;
        }
        let raw = (if refills == 0 { rd_u32(state + idx*4) } else { buf[idx] }) as u64;   // ★레버1: refill 전엔 state 직접
        idx += 1;
        let prod = raw.wrapping_mul(range);                   // u32*u32 → u64
        if zone < (prod & 0xffff_ffff) { continue; }          // rejection (게임: zone < low32(prod) → continue)
        return Some(lo + (prod >> 32) as i64);                // sample = high32(prod) ∈ [0,range)
    }
}
// 정수 제곱근(f22e80 inline: <0xf4241면 Newton, else 비트길이 이진탐색. 결과는 floor(sqrt)).
fn isqrt_u64(n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut x = n; let mut y = (x + 1) >> 1;
    while y < x { x = y; y = (x + n / x) >> 1; }
    x
}
unsafe fn rd_u32(a: usize) -> u32 { if a < 0x10000 { return 0; } match FAST_READ.load(Ordering::Relaxed) { 2 => lr_i32(a).map(|v| v as u32).unwrap_or(0), 3 => std::ptr::read_unaligned(a as *const u32), 1 => safe_rd_i32(a).map(|v| v as u32).unwrap_or(0), _ => if readable(a,4){std::ptr::read_unaligned(a as *const u32)}else{0} } }   // ★fast_read=2 분기 누락 수정(RNG 64워드/draw VQ→lockless): 단일 최대 효과
// ── ChaCha12 블록 재현 (rand-0.8.5 StdRng refill = FUN_1421bbc10) ──
#[inline] fn chacha_qr(s: &mut [u32;16], a:usize,b:usize,c:usize,d:usize){
    s[a]=s[a].wrapping_add(s[b]); s[d]^=s[a]; s[d]=s[d].rotate_left(16);
    s[c]=s[c].wrapping_add(s[d]); s[b]^=s[c]; s[b]=s[b].rotate_left(12);
    s[a]=s[a].wrapping_add(s[b]); s[d]^=s[a]; s[d]=s[d].rotate_left(8);
    s[c]=s[c].wrapping_add(s[d]); s[b]^=s[c]; s[b]=s[b].rotate_left(7);
}
fn chacha12_block(key:&[u32;8], counter:u64, nonce:u64, out:&mut [u32;16]){
    let mut s=[0x61707865u32,0x3320646e,0x79622d32,0x6b206574,
               key[0],key[1],key[2],key[3],key[4],key[5],key[6],key[7],
               counter as u32,(counter>>32) as u32, nonce as u32,(nonce>>32) as u32];
    let init=s;
    for _ in 0..6 {   // 6 더블라운드 = 12 라운드
        chacha_qr(&mut s,0,4,8,12); chacha_qr(&mut s,1,5,9,13); chacha_qr(&mut s,2,6,10,14); chacha_qr(&mut s,3,7,11,15);
        chacha_qr(&mut s,0,5,10,15); chacha_qr(&mut s,1,6,11,12); chacha_qr(&mut s,2,7,8,13); chacha_qr(&mut s,3,4,9,14);
    }
    for i in 0..16 { out[i]=s[i].wrapping_add(init[i]); }
}
// ★레버3: ChaCha12 4블록 SIMD(SSE2 __m128i, 4블록=4레인 동시). x86_64는 SSE2 baseline이라 runtime-detect 불필요.
//   refill=4블록(base+0..3)을 1회 벡터연산(스칼라 4회 대비). init()의 self-test가 스칼라와 비트동일 확인 후 USE_SIMD_CHACHA=true.
static USE_SIMD_CHACHA: AtomicBool = AtomicBool::new(false);
#[cfg(target_arch = "x86_64")]
unsafe fn chacha12_4block_sse2(key:&[u32;8], base:u64, nonce:u64, out: *mut u32){
    use core::arch::x86_64::*;
    #[inline] unsafe fn vqr(v: &mut [__m128i;16], a:usize,b:usize,c:usize,d:usize){
        let (mut va,mut vb,mut vc,mut vd)=(v[a],v[b],v[c],v[d]);
        va=_mm_add_epi32(va,vb); vd=_mm_xor_si128(vd,va); vd=_mm_or_si128(_mm_slli_epi32::<16>(vd),_mm_srli_epi32::<16>(vd));
        vc=_mm_add_epi32(vc,vd); vb=_mm_xor_si128(vb,vc); vb=_mm_or_si128(_mm_slli_epi32::<12>(vb),_mm_srli_epi32::<20>(vb));
        va=_mm_add_epi32(va,vb); vd=_mm_xor_si128(vd,va); vd=_mm_or_si128(_mm_slli_epi32::<8>(vd),_mm_srli_epi32::<24>(vd));
        vc=_mm_add_epi32(vc,vd); vb=_mm_xor_si128(vb,vc); vb=_mm_or_si128(_mm_slli_epi32::<7>(vb),_mm_srli_epi32::<25>(vb));
        v[a]=va; v[b]=vb; v[c]=vc; v[d]=vd;
    }
    let b0=base; let b1=base.wrapping_add(1); let b2=base.wrapping_add(2); let b3=base.wrapping_add(3);
    let mut v = [
        _mm_set1_epi32(0x61707865u32 as i32), _mm_set1_epi32(0x3320646eu32 as i32),
        _mm_set1_epi32(0x79622d32u32 as i32), _mm_set1_epi32(0x6b206574u32 as i32),
        _mm_set1_epi32(key[0] as i32), _mm_set1_epi32(key[1] as i32), _mm_set1_epi32(key[2] as i32), _mm_set1_epi32(key[3] as i32),
        _mm_set1_epi32(key[4] as i32), _mm_set1_epi32(key[5] as i32), _mm_set1_epi32(key[6] as i32), _mm_set1_epi32(key[7] as i32),
        _mm_setr_epi32(b0 as u32 as i32, b1 as u32 as i32, b2 as u32 as i32, b3 as u32 as i32),
        _mm_setr_epi32((b0>>32) as u32 as i32, (b1>>32) as u32 as i32, (b2>>32) as u32 as i32, (b3>>32) as u32 as i32),
        _mm_set1_epi32(nonce as u32 as i32), _mm_set1_epi32((nonce>>32) as u32 as i32),
    ];
    let init = v;
    for _ in 0..6 {
        vqr(&mut v,0,4,8,12); vqr(&mut v,1,5,9,13); vqr(&mut v,2,6,10,14); vqr(&mut v,3,7,11,15);
        vqr(&mut v,0,5,10,15); vqr(&mut v,1,6,11,12); vqr(&mut v,2,7,8,13); vqr(&mut v,3,4,9,14);
    }
    let mut tmp=[0u32;4];
    for i in 0..16 {
        let s = _mm_add_epi32(v[i], init[i]);
        _mm_storeu_si128(tmp.as_mut_ptr() as *mut __m128i, s);
        *out.add(i)=tmp[0]; *out.add(16+i)=tmp[1]; *out.add(32+i)=tmp[2]; *out.add(48+i)=tmp[3];
    }
}
// 4블록 refill 디스패처: USE_SIMD면 SSE2 1회, 아니면 스칼라 chacha12_block 4회. out=최소 64 u32.
#[inline] unsafe fn chacha12_4block(key:&[u32;8], base:u64, nonce:u64, out: *mut u32){
    #[cfg(target_arch = "x86_64")]
    { if USE_SIMD_CHACHA.load(Ordering::Relaxed) { chacha12_4block_sse2(key, base, nonce, out); return; } }
    for b in 0..4u64 { let mut blk=[0u32;16]; chacha12_block(key, base.wrapping_add(b), nonce, &mut blk); for w in 0..16 { *out.add(b as usize*16 + w) = blk[w]; } }
}
// self-test: SIMD 4블록 == 스칼라 4블록 비트동일이면 true(→USE_SIMD_CHACHA). 다양한 시드+counter carry 케이스.
fn chacha_simd_selftest() -> bool {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let cases: [([u32;8], u64, u64); 4] = [
            ([0,0,0,0,0,0,0,0], 0, 0),
            ([1,2,3,4,5,6,7,8], 0xffff_fffeu64, 0x1234_5678_9abc_def0),
            ([0xdead_beef,0x1234_5678,0,0xffff_ffff,7,0x8000_0000,3,0x5555_5555], 0x0000_0000_ffff_ffff, 0),
            ([9,8,7,6,5,4,3,2], 0xffff_ffff_ffff_fffd, 0xdead),
        ];
        for (key, base, nonce) in cases.iter() {
            let mut scal=[0u32;64];
            for b in 0..4u64 { let mut blk=[0u32;16]; chacha12_block(key, base.wrapping_add(b), *nonce, &mut blk); for w in 0..16 { scal[b as usize*16+w]=blk[w]; } }
            let mut simd=[0u32;64];
            chacha12_4block_sse2(key, *base, *nonce, simd.as_mut_ptr());
            if scal != simd { return false; }
        }
        return true;
    }
    #[allow(unreachable_code)] { false }
}
// input ptr(rcx): key[8]@+0, counter u64@+0x20, nonce u64@+0x28. 4블록(counter+0..3) → out[64].
// 버퍼 워드순서는 SIMD변형에 따라 다를 수 있음 → implement-and-diff로 확정(초기추정=block-sequential).
unsafe fn chacha_reproduce(input: usize, out: &mut [u32; 64]) -> bool {
    if !readable(input, 0x30) { return false; }
    let mut key = [0u32; 8];
    for i in 0..8 { key[i] = rd_u32(input + i*4); }
    let counter = rd_u64(input + 0x20).unwrap_or(0);
    let nonce = rd_u64(input + 0x28).unwrap_or(0);
    for b in 0..4u64 {
        let mut blk = [0u32; 16];
        chacha12_block(&key, counter.wrapping_add(b), nonce, &mut blk);
        for w in 0..16 { out[(b as usize)*16 + w] = blk[w]; }
    }
    true
}
// 유저공간 포인터로 그럴듯한 범위 (오버플로/쓰레기 산술 방지)
fn ptr_ok(a: usize) -> bool { a >= 0x10000 && a < 0x0001_0000_0000_0000 }
// p가 가리키는 곳이 정확히 nb 문자열(+null종료)인가
unsafe fn str_eq_at(p: usize, nb: &[u8]) -> bool {
    if nb.is_empty() || !ptr_ok(p) || !readable(p, nb.len()+1) { return false; }
    for k in 0..nb.len() { if std::ptr::read_unaligned((p+k) as *const u8) != nb[k] { return false; } }
    std::ptr::read_unaligned((p+nb.len()) as *const u8) == 0  // null 종료(접두사 오탐 방지)
}
// 힙할당 없이 null종료 문자열을 buf에 읽음 → 길이
unsafe fn read_name_at(p: usize, buf: &mut [u8; 24]) -> usize {
    if !ptr_ok(p) || !readable(p, 1) { return 0; }
    let mut n = 0usize;
    while n < 24 {
        if !readable(p+n, 1) { break; }
        let b = std::ptr::read_unaligned((p+n) as *const u8);
        if b == 0 { break; }
        buf[n] = b; n += 1;
    }
    n
}
// p의 null종료 문자열을 최대 31바이트 읽어 String으로
unsafe fn cstr(p: usize) -> String {
    if !ptr_ok(p) || !readable(p, 1) { return "?".into(); }
    let mut v = Vec::new();
    for k in 0..31usize {
        if !readable(p+k, 1) { break; }
        let b = std::ptr::read_unaligned((p+k) as *const u8);
        if b == 0 { break; }
        v.push(b);
    }
    String::from_utf8_lossy(&v).into_owned()
}

fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }
fn dir() -> Option<PathBuf> { unsafe {
    let mut h: HMODULE = 0;
    if GetModuleHandleExW(0x4|0x2, dir as *const () as *const u16, &mut h) == 0 || h == 0 { return None; }
    let mut b = [0u16; 4096];
    let n = GetModuleFileNameW(h, b.as_mut_ptr(), b.len() as DWORD);
    if n == 0 { return None; }
    let mut p = PathBuf::from(String::from_utf16_lossy(&b[..n as usize])); p.pop(); Some(p)
}}
fn pth(name: &str) -> Option<PathBuf> { dir().map(|mut p| { p.push(name); p }) }
// ★배포: 모든 진단/로그 파일출력 마스터 스위치. cfg log=1 일때만 기록(기본 off=배포 깨끗). 출력만 막음=캡처계산은 유지(load-bearing 안전). cfg템플릿 생성(아래 CFG_TEMPLATE)은 별개=기능 유지.
static LOG_ON: AtomicBool = AtomicBool::new(false);
// ★skip_untuned: 튜닝 안 한 judge는 대체 끄고 원본 native 사용(결과 100% 동일·속도↑). 일정넘김 백그라운드 N경기 가속.
//   "튜닝됨" 판정 = config/default.txt(기준값)와 활성 cfg값 비교(하드코딩 default 없음=오류방지). condgate는 계수 없어 항상 untuned.
static SKIP_UNTUNED: AtomicBool = AtomicBool::new(false);
fn read_baseline() -> Option<std::collections::HashMap<String, i64>> {
    let p = pth("config/default.txt")?;
    let txt = fs::read_to_string(&p).ok()?;
    let mut m = std::collections::HashMap::new();
    for line in txt.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') { continue; }
        if let Some((k, v)) = t.split_once('=') {
            let v = v.split('#').next().unwrap_or("").trim();
            let parsed = match v.strip_prefix("0x").or_else(|| v.strip_prefix("0X")) {
                Some(h) => i64::from_str_radix(h, 16).ok(),
                None => v.parse::<i64>().ok(),
            };
            if let Some(n) = parsed { m.insert(k.trim().to_string(), n); }
        }
    }
    Some(m)
}
fn append_log(s: &str) { if !LOG_ON.load(Ordering::Relaxed) { return; } if let Some(p) = pth("plan_reimpl.txt") {
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = write!(f,"{}",s); } } }
fn fresh_log(s: &str) { if !LOG_ON.load(Ordering::Relaxed) { return; } if let Some(p) = pth("plan_reimpl.txt") { let _ = fs::write(p, s); } }
fn write_named(name: &str, s: &str) { if !LOG_ON.load(Ordering::Relaxed) { return; } if let Some(p) = pth(name) { let _ = fs::write(p, s); } }
fn append_named(name: &str, s: &str) { if !LOG_ON.load(Ordering::Relaxed) { return; } if let Some(p) = pth(name) {
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = write!(f,"{}",s); } } }

// ── 로스터 열거: plan_base → [(team, idx, entity)] (유효 챔피언만) ──
unsafe fn roster(pb: usize) -> Vec<(usize, usize, usize)> {
    let mut v = Vec::new();
    if !ptr_ok(pb) { return v; }
    for team in 0..2usize {
        let base = pb + team*ROSTER_STRIDE + ROSTER_BASE;
        if !readable(base, ROSTER_N*8) { continue; }
        for i in 0..ROSTER_N {
            let e = rd_u64(base + i*8).unwrap_or(0) as usize;
            if e <= 0x10000 || !readable(e, 0x740) { continue; }
            if rd_i64(e + E_SPEED).unwrap_or(0) <= 0 { continue; }
            v.push((team, i, e));
        }
    }
    v
}

// ── 월드 모델: 연속 엔티티배열(stride 0x6a8)에서 챔프 10명 열거 ──
const STRIDE: usize = 0x6a8;
unsafe fn is_champion(e: usize) -> bool {
    if !ptr_ok(e) || !readable(e, 0x740) { return false; }
    let mx = rd_i64(e+E_MAXHP).unwrap_or(0);
    let sp = rd_i64(e+E_SPEED).unwrap_or(0);
    let x  = rd_i64(e+E_POSX).unwrap_or(-1);
    let y  = rd_i64(e+E_POSY).unwrap_or(-1);
    mx >= 600 && mx <= 1900 && sp > 0 && x >= 0 && x < 1_000_000 && y >= 0 && y < 1_000_000
}
// roster 포인터 중 첫 유효 엔티티 = 배열 진입 seed (미니언이라도 OK)
unsafe fn find_seed(pb: usize) -> usize {
    if !ptr_ok(pb) { return 0; }
    for team in 0..2usize {
        let base = pb + team*ROSTER_STRIDE + ROSTER_BASE;
        if !readable(base, ROSTER_N*8) { continue; }
        for i in 0..ROSTER_N {
            let e = rd_u64(base + i*8).unwrap_or(0) as usize;
            if ptr_ok(e) && readable(e, 0x740) { return e; }
        }
    }
    0
}
// 챔프 10명: (team(+0x8), entity). 챔프후보 중 "최장 연속 런"(stride 0x6a8) = 진짜 10명.
// (hp700/speed800 특수미니언은 챔프블록과 떨어져 고립 → 런에서 탈락)
unsafe fn champions(pb: usize) -> Vec<(i64, usize)> {
    let s = find_seed(pb);
    if s == 0 { return Vec::new(); }
    let lo = s.saturating_sub(60*STRIDE);
    let cand: Vec<usize> = (0..121usize).map(|j| lo + j*STRIDE).filter(|&e| is_champion(e)).collect();
    let (mut best_i, mut best_len, mut i) = (0usize, 0usize, 0usize);
    while i < cand.len() {
        let mut j = i;
        while j+1 < cand.len() && cand[j+1] == cand[j] + STRIDE { j += 1; }
        let len = j - i + 1;
        if len > best_len { best_len = len; best_i = i; }
        i = j + 1;
    }
    cand[best_i..best_i+best_len].iter().map(|&e| (rd_i64(e+0x8).unwrap_or(-1), e)).collect()
}

// ── §E combat_effective_damage 재구현 (디컴파일 정확 이식). 검증: 게임함수와 비교 ──
// vt 글로벌(스탯-accessor vtable). atk vt[0x30]=스탯getter, vt[0x38]=데미지시트, tgt vt[0x30]=방어getter.
// ★2nd-hotfix 재마이그(2026-06-18, ghidra_beta 2nd-hotfix exe 로딩 확인): 메인TTD(0x1b6df40)의 combat(0x1be1e90)
//   호출지점 4곳 전부 LEA RDX,[0x14356ed28]; MOV R9,RDX → atk(rdi)·tgt(r9) 둘다 0x356ed28 (단일 universal accessor vt).
//   xrefs_to(0x14356ed28) = 전 데미지함수(FUN_141cfe993 등)가 동일 참조 → universal 확정. (구 0.4.12: atk=0x34b2d48/tgt=0x355e900 stale)
const RVA_ATK_VT: usize = 0x356ed28;   // ⚠0.4.13_5 미해결: vt[0]=base getter→new 0x18c3090 가리키는 슬롯 7개 동일내용. 데미지코드 LEA슬롯으로 확정요(Ghidra). MIG_DMG off라 미사용
const RVA_TGT_VT: usize = 0x356ed28;   // ⚠동상(atk와 동일 vt)
const RVA_COMBAT_FN: usize = 0x1fdb5b0;   // 0.4.13_5(was 0x1f76df0). mask-sig 유일매치=바디동일. ★F80320 새 데미지경로가 이걸 ×6 호출. MIG_DMG off
const COEF_MULT_PCT: i64 = 100;   // ★데미지 coef 배수%. 100=원본검증(dmgcmp OK 판정), 150=override데모. 검증위해 100.
// ★ 진단 하네스 스위치: false=실게임 안전, true=디버깅(데모/리플레이).
// TTD 검증 = 리턴-훅(실제 rax 반환 캡처). 재호출(g) 제거 → 재진입 부작용/크래시 원인 제거.
const HARNESS_ON: bool = true;
const RVA_TTD: usize = 0x1d307f0; // ⚠0.4.13_5 tentative(was 0x1e1c7c0, 상수지문+callers=4+프롤로그경계OK, Ghidra확인요). MIG_TTD off(데미지경로 변경+DEFAULT_AB2 미해결로 repro 재유도 필요). plan_score_survival_ttd
type TtdFn = unsafe extern "C" fn(usize, usize, usize, usize, usize, usize) -> i64;
static TTD_N: AtomicU64 = AtomicU64::new(0);

// ── TTD 리턴-훅 (리턴주소 스왑 트램폴린) ──
// 진입 훅에서: 입력 스냅샷 → my_ttd 계산 → 리턴주소를 thunk로 교체 + 프레임 push.
// 함수가 ret하면 thunk로 진입 → rax=실제 게임 TTD → ttd_return에서 game vs mine 로깅 → orig_ret로 복귀.
static RET_THUNK: AtomicUsize = AtomicUsize::new(0);   // 공용 리턴 thunk 코드 주소
static TTD_ARMED: AtomicU64 = AtomicU64::new(0);       // 총 무장(리턴훅 건) 호출 수
static TTD_NONEMPTY: AtomicU64 = AtomicU64::new(0);    // 그중 +0xf0 적리스트 비지않은 호출 수
static TTD_FILE_INIT: AtomicBool = AtomicBool::new(false); // ttdcmp.txt 첫쓰기=truncate
const TTD_ARM_MAX: u64 = 6000;      // 무장 상한(빈것 포함; ENGAGE 샘플 확보 위해 상향 — 16 engage 모일때까지 매치 후반 캡처)
const TTD_NONEMPTY_MAX: u64 = 16;   // 실교전 샘플 목표
// kind: 0=TTD(game=retval vs mine), 1=RE(retreat_engage: 결정=*retval). pre=로그프리픽스.
struct RetFrame { key: usize, orig_ret: usize, mine: i64, kind: u8, pre: String, p5: usize, p6: usize, disp_pred: i64 }
static RET_STACK: Mutex<Vec<RetFrame>> = Mutex::new(Vec::new());
static RE_FILE_INIT: AtomicBool = AtomicBool::new(false);

// thunk: rax=retval로 진입, rsp=key+8. hook_return(retval, key)→orig_ret 호출 후 rax복원·orig_ret 점프.
unsafe fn build_ret_thunk() {
    let handler = hook_return as *const () as usize;
    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(&[0x50]);                 // push rax            (retval 보존; rsp=key=ESP0)
    code.extend_from_slice(&[0x48,0x89,0xC1]);       // mov rcx, rax        (arg1=retval)
    code.extend_from_slice(&[0x48,0x89,0xE2]);       // mov rdx, rsp        (arg2=key=ESP0)
    code.extend_from_slice(&[0x4C,0x8B,0x84,0x24,0x50,0xFF,0xFF,0xFF]); // mov r8,[rsp-0xb0]  (arg3=e1; RE=local_b0 게임임계값@ESP0-0xb0)
    code.extend_from_slice(&[0x4C,0x8B,0x8C,0x24,0xD8,0xFD,0xFF,0xFF]); // mov r9,[rsp-0x228] (arg4=e2; RE=local_228 셀렉터@ESP0-0x228)
    code.extend_from_slice(&[0x48,0x8B,0x84,0x24,0xE8,0xFD,0xFF,0xFF]); // mov rax,[rsp-0x218] (tmp=local_218 idx)
    code.extend_from_slice(&[0x4C,0x8B,0x94,0x24,0xB0,0xFF,0xFF,0xFF]); // mov r10,[rsp-0x50]  (tmp=local_50 df1da0반환)
    code.extend_from_slice(&[0x48,0x83,0xEC,0x38]);  // sub rsp,0x38        (16정렬 + shadow + 2 stack args)
    code.extend_from_slice(&[0x48,0x89,0x44,0x24,0x20]); // mov [rsp+0x20],rax  (arg5=local_218)
    code.extend_from_slice(&[0x4C,0x89,0x54,0x24,0x28]); // mov [rsp+0x28],r10  (arg6=local_50)
    code.extend_from_slice(&[0x48,0xB8]); code.extend_from_slice(&handler.to_le_bytes()); // movabs rax,handler
    code.extend_from_slice(&[0xFF,0xD0]);            // call rax
    code.extend_from_slice(&[0x48,0x83,0xC4,0x38]);  // add rsp,0x38
    code.extend_from_slice(&[0x49,0x89,0xC2]);       // mov r10, rax        (orig_ret)
    code.extend_from_slice(&[0x58]);                 // pop rax             (retval 복원; rsp=key+8)
    code.extend_from_slice(&[0x41,0xFF,0xE2]);       // jmp r10
    let m = VirtualAlloc(0, 64, 0x1000|0x2000, 0x40);
    if m != 0 { core::ptr::copy_nonoverlapping(code.as_ptr(), m as *mut u8, code.len()); RET_THUNK.store(m, Ordering::Relaxed); }
}
type Getter1 = unsafe extern "C" fn(usize) -> i64;  // 역할게터 vt[0x68](data)->role
// 공용 리턴 thunk 핸들러: retval=반환값, key=ESP0, e1=local_b0(게임임계값), e2=local_228(셀렉터), e3=local_218(idx), e4=local_50(df1da0반환).
unsafe extern "C" fn hook_return(retval: i64, key: usize, e1: i64, e2: i64, e3: i64, e4: i64) -> usize {
    let frame = if let Ok(mut st) = RET_STACK.lock() {
        st.iter().rposition(|f| f.key == key).map(|p| st.remove(p))
    } else { None };
    match frame {
        Some(f) => {
            // ★panic-safe(mod-safety): 리턴훅 verify/logging panic이 FFI UB로 게임 크래시 → catch_unwind 차단.
            //   orig_ret은 먼저 추출해 패닉시에도 정상 복귀(게임 흐름 유지). 전 kind(0/1/2/3/5/9/11) 공통보호.
            let ret = f.orig_ret;
            let hr = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if f.kind == 0 {
                let verdict = if retval == f.mine { "OK" } else { "DIFF" };
                let s = format!("{}game={} mine={} [{}]\n", f.pre, retval, f.mine, verdict);
                if !TTD_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("ttdcmp.txt", &s); }
                else { append_named("ttdcmp.txt", &s); }
            } else if f.kind == 2 {
                // dd7700: 함수가 param_1을 리턴 → retval == out ptr. game action code = *retval(+0=i64), 상태바이트 +8/+9/+10.
                let op = retval as usize;
                let code = if ptr_ok(op) { rd_i64(op).unwrap_or(-99) } else { -99 };
                let rdb = |o: usize| if readable(op+o,1) { std::ptr::read_unaligned((op+o) as *const u8) as i64 } else { -1 };
                let (b8, b9, b10) = (rdb(8), rdb(9), rdb(0xa));
                let pred = f.mine;  // -999=미예측(로깅만)
                let verdict = if pred == -999 { "(미예측)" } else if pred == code { "OK" } else { "★DIFF" };
                let n = DD7_LOGGED.fetch_add(1, Ordering::Relaxed);
                if n < 600 {
                    let s = format!("{} → game_code={} (b8={} b9={} b10={}) [{}]\n", f.pre, code, b8, b9, b10, verdict);
                    if !DD7_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("dd7cmp.txt", "=== dd7700 game action code 캡처 (code @+0, 바이트 +8/+9/+10) ===\n"); }
                    append_named("dd7cmp.txt", &s);
                }
                // ★dd7700 full-output 대체검증: capture시점 계산한 DD7700_MY(내출력) vs 게임 op byte대조([0..0x10]). op매칭 게이트로 static staleness 회피.
                if ptr_ok(op) && readable(op, 0x10) && DD7700_MY_OP.load(Ordering::Relaxed) == op {
                    let res = DD7700_MY_RES.load(Ordering::Relaxed);
                    if res == 1 {
                        let mut buf = [0u8; 0x10];
                        buf[0..8].copy_from_slice(&DD7700_MY[0].load(Ordering::Relaxed).to_le_bytes());
                        buf[8..16].copy_from_slice(&DD7700_MY[1].load(Ordering::Relaxed).to_le_bytes());
                        let mut diff_at: i64 = -1;
                        for i in 0..0x10usize { if buf[i] != rd_u8(op + i) { diff_at = i as i64; break; } }
                        let ok = diff_at < 0;
                        if ok { DD7F_OK.fetch_add(1, Ordering::Relaxed); } else { DD7F_DIFF.fetch_add(1, Ordering::Relaxed); }
                        let m = DD7F_OK.load(Ordering::Relaxed) + DD7F_DIFF.load(Ordering::Relaxed);
                        if !ok || m % 200 == 0 || m <= 8 {
                            if !DD7F_INIT.swap(true, Ordering::Relaxed) { write_named("dd7full.txt", "=== dd7700 full-output 대체검증: DD7700_MY(capture계산) vs 게임 op ([0..0x10]) ===\n"); }
                            let verdict = if ok { "OK".to_string() } else { format!("★DIFF@+0x{:x}", diff_at) };
                            let mb: [u8;0x10] = core::array::from_fn(|i| buf[i]);
                            let gb: [u8;0x10] = core::array::from_fn(|i| rd_u8(op + i));
                            append_named("dd7full.txt", &format!("[dd7f {}] code={} [{}] OK={} DIFF={} PASS={}\n  my  ={:02x?}\n  game={:02x?}\n",
                                m, code, verdict, DD7F_OK.load(Ordering::Relaxed), DD7F_DIFF.load(Ordering::Relaxed), DD7F_PASS.load(Ordering::Relaxed), mb, gb));
                        }
                    } else if res == 0 { DD7F_PASS.fetch_add(1, Ordering::Relaxed); }
                }
                // ★RNG-sync 검증: my_dd7700_rng_final 예측 exit(idx,counter) vs 실제 게임 exit. per-call(타이밍무관). + in-scope 실제 draw수(fcd980/fcdaf0) vs 내 N.
                DD7_INSCOPE.store(false, Ordering::Relaxed);   // in-scope 윈도우 닫기
                // ★RNG-sync는 실제 skip할 케이스(my_dd7700_full=Some)만 의미있음. None(plan8/engage6·7)=passthrough→dd7700 실행→RNG자동. 그것만 검증.
                if DD7_RNG_VALID.load(Ordering::Relaxed) && DD7700_MY_RES.load(Ordering::Relaxed) == 1 {
                    let p4 = DD7_RNG_P4.load(Ordering::Relaxed);
                    if ptr_ok(p4) && readable(p4 + 0x138, 8) {
                        let i1 = rd_u64(p4 + 0x100).unwrap_or(0);
                        let c1 = rd_u64(p4 + 0x130).unwrap_or(0);
                        let (pidx, pctr) = (DD7_RNG_PIDX.load(Ordering::Relaxed), DD7_RNG_PCTR.load(Ordering::Relaxed));
                        let (myn, g980, gaf0) = (DD7_RNG_N.load(Ordering::Relaxed), DD7_IS_980.load(Ordering::Relaxed), DD7_IS_AF0.load(Ordering::Relaxed));
                        let ok = pidx == i1 && pctr == c1;
                        if ok { DD7RNG_OK.fetch_add(1, Ordering::Relaxed); } else { DD7RNG_DIFF.fetch_add(1, Ordering::Relaxed); }
                        let m = DD7RNG_OK.load(Ordering::Relaxed) + DD7RNG_DIFF.load(Ordering::Relaxed);
                        if !ok || m % 500 == 0 || m <= 8 {
                            if !DD7RNG_INIT.swap(true, Ordering::Relaxed) { write_named("dd7rng.txt", "=== dd7700 RNG-sync 검증: 예측 exit vs 실제 + in-scope draw + 윈도우/cmask 진단 ===\n"); }
                            let dbg = DD7_RNG_DBG.load(Ordering::Relaxed);
                            let ctab = DD7_RNG_CTAB.load(Ordering::Relaxed);   // exit 재독: dd7700이 candtable 수정했나?
                            let mut excm = 0u64; if ptr_ok(ctab) { for l in 0..5usize { if rd_u64(ctab + l*8).unwrap_or(0) != 0 { excm |= 1 << l; } } }
                            let pi14 = DD7_RNG_PI14.load(Ordering::Relaxed);   // role record exit 재독: dd7700이 iVar12/target 수정했나?
                            let (iv12_exit, tgt_exit) = if ptr_ok(pi14) { (rd_i32(pi14).unwrap_or(-99), rd_u64(pi14 + 8).unwrap_or(0)) } else { (-99, 0) };
                            let tgt_entry = DD7_RNG_TH0.load(Ordering::Relaxed);
                            append_named("dd7rng.txt", &format!("[dd7rng {}] gameCODE={} my(idx={} ctr={}) game(idx={} ctr={}) [{}] | myN={} fcd980={} fcdaf0={} | lo={} hi={} i0={} cmask=0b{:05b} exitcmask=0b{:05b} | plan={} f={} iv12={} tgt={} reached={} | OK={} DIFF={}\n",
                                m, code, pidx, pctr, i1, c1, if ok {"OK"} else {"★DIFF"}, myn, g980, gaf0,
                                DD7_RNG_LO.load(Ordering::Relaxed), DD7_RNG_HI.load(Ordering::Relaxed), DD7_RNG_I0.load(Ordering::Relaxed), DD7_RNG_CMASK.load(Ordering::Relaxed), excm,
                                dbg & 0xff, (dbg >> 8) & 0xf, (dbg >> 12) & 1, (dbg >> 13) & 1, (dbg >> 15) & 1,
                                DD7RNG_OK.load(Ordering::Relaxed), DD7RNG_DIFF.load(Ordering::Relaxed)));
                            let mycode = (DD7700_MY[0].load(Ordering::Relaxed) & 0xff) as i64;   // my_dd7700_full 출력코드
                            append_named("dd7rng.txt", &format!("        ↳ role-record exit재독: iv12_exit={} tgt_entry={:#x} tgt_exit={:#x} {} | myCODE={} (game={}) {}\n",
                                iv12_exit, tgt_entry, tgt_exit, if iv12_exit != 1 || tgt_entry != tgt_exit { "★role CHANGED" } else { "role동일" },
                                mycode, code, if mycode == code { "출력일치(=rng만 over예측)" } else { "★출력도DIFF(my_dd7700_full도 틀림)" }));
                        }
                    }
                    DD7_RNG_VALID.store(false, Ordering::Relaxed);
                }
            } else if f.kind == 3 {
                // PRNG: retval = 실제 gen_range 반환. f.mine = read-only 시뮬 예측(-888=refill경계).
                let actual = retval; let pred = f.mine;
                let verdict = if pred == -888 { "refill경계" } else if pred == actual { "OK" } else { "★DIFF" };
                // ★④ write-back 상태전이 검증: 게임 실제 after-state(idx@+0x100, counter@+0x130) vs 내 예측.
                //   f.p5=state, f.p6=예측 after-idx, f.disp_pred=예측 after-counter. (게임 fcd980 실행 후 시점)
                let st_v = if f.p5 != 0 && pred != -888 {
                    let game_idx = rd_u64(f.p5 + 0x100).unwrap_or(u64::MAX);
                    let game_cnt = rd_u64(f.p5 + 0x130).unwrap_or(u64::MAX);
                    let (my_idx, my_cnt) = (f.p6 as u64, f.disp_pred as u64);
                    if game_idx == my_idx && game_cnt == my_cnt {
                        RNGST_OK.fetch_add(1, Ordering::Relaxed);
                        format!(" | ST-OK[idx={} cnt={}]", game_idx, game_cnt)
                    } else {
                        RNGST_DIFF.fetch_add(1, Ordering::Relaxed);
                        format!(" | ★ST-DIFF game[idx={} cnt={}] my[idx={} cnt={}]", game_idx, game_cnt, my_idx, my_cnt)
                    }
                } else { String::new() };
                let n = RNG_LOGGED.fetch_add(1, Ordering::Relaxed);
                if n < 800 {
                    let s = format!("{} → game={} mine={} [{}]{} (ST-OK={} ST-DIFF={})\n", f.pre, actual, pred, verdict, st_v, RNGST_OK.load(Ordering::Relaxed), RNGST_DIFF.load(Ordering::Relaxed));
                    if !RNG_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("rngcmp.txt", "=== PRNG gen_range game==mine 검증 (결과 + ④ write-back 상태전이 ST) ===\n"); }
                    append_named("rngcmp.txt", &s);
                }
            } else if f.kind == 4 {
                // ChaCha refill: f.p5 = 게임 output 버퍼(256B). MY_CHACHA(내 재현)와 64워드 대조.
                let outp = f.p5;
                let n = CHACHA_LOGGED.fetch_add(1, Ordering::Relaxed);
                if n < 32 {
                    let mine = MY_CHACHA.lock().map(|m| *m).unwrap_or([0u32;64]);
                    let mut nmatch = 0; let mut firstdiff = String::new();
                    // 게임버퍼 어느 워드가 내것 어디와 맞는지: 정렬일치 카운트 + 처음 8워드 양쪽 덤프
                    for i in 0..64 { if rd_u32(outp + i*4) == mine[i] { nmatch += 1; } }
                    for i in 0..8 { firstdiff.push_str(&format!("  [{}] game={:08x} mine={:08x}\n", i, rd_u32(outp+i*4), mine[i])); }
                    let s = format!("[chacha #{}] 정렬일치 {}/64\n{}", n, nmatch, firstdiff);
                    if !CHACHA_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("chacha.txt", "=== ChaCha12 refill 버퍼 대조 (정렬일치=내 block-sequential 추정 맞은 워드수) ===\n"); }
                    append_named("chacha.txt", &s);
                }
            } else if f.kind == 5 {
                // fc59a0 recall score: 게임출력 *p5 = score@+0(i32), bool@+4(u8), mult@+8(i32). f.mine=내 RNG배율 m, f.disp_pred=threshold.
                let op = f.p5;
                let score = rd_i32(op).unwrap_or(-999) as i64;
                let gbool = if readable(op+4,1) { std::ptr::read_unaligned((op+4) as *const u8) as i64 } else { -1 };
                let mult  = rd_i32(op+8).unwrap_or(-999) as i64;
                let my_m = f.mine; let thr = f.disp_pred;
                let my_mult = f.p6 as i64;                     // 내 base-score 재현 mult
                let mok = my_mult != RECALL_MULT_NONE && my_mult == mult;   // ★base-score 검증(게임 mult 대조)
                let mtag = if my_mult == RECALL_MULT_NONE { "mult:N/A".to_string() }
                           else if mok { "mult:OK".to_string() } else { format!("★mult-DIFF(my={} game={})", my_mult, mult) };
                let n = RECALL_ARMED.load(Ordering::Relaxed);
                if n <= RECALL_ARM_MAX {
                    let (verdict, detail) = if mult == 0 && score == 0 {
                        ("early-out".to_string(), "(후보없음/조기반환, RNG미소비)".to_string())
                    } else {
                        let pred_score = (my_m * mult) / 100;          // score = (m*mult)/100 검증
                        let pred_bool = (thr <= pred_score) as i64;
                        let sok = pred_score == score;
                        let bok = pred_bool == gbool;
                        let v = if sok && bok { "OK" } else if sok { "bool-DIFF" } else { "★score-DIFF" };
                        (v.to_string(), format!("game_score={} pred={}({}*{}/100) | game_bool={} pred_bool={}", score, pred_score, my_m, mult, gbool, pred_bool))
                    };
                    let s = format!("{} → mult={} [{}] {} [{}]\n", f.pre, mult, mtag, detail, verdict);
                    if !RECALL_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("recallcmp.txt", "=== fc59a0 recall RNG score 검증: score=(m*mult)/100, bool=(thr<=score). m=내 read-only RNG draw 재현 ===\n"); }
                    append_named("recallcmp.txt", &s);
                }
            } else if f.kind == 6 {
                // facet#1 condgate: 게임 al = retval&0xff. f.mine=my_condgate(-99=pending poke/gank).
                let game_al = (retval & 0xff) as i64;
                let my = f.mine;
                // ★in-scope RNG draw 측정(cond_repl 안전 재확인): condgate가 RNG 소비했나? replaced disc(my≠-99)가 0이어야 skip 안전.
                COND_INSCOPE.store(false, Ordering::Relaxed);
                let draws = COND_IS_DRAWS.load(Ordering::Relaxed);
                let di6 = (f.p5).min(15);
                if draws > COND_DISC_MAXDRAW[di6].load(Ordering::Relaxed) { COND_DISC_MAXDRAW[di6].store(draws, Ordering::Relaxed); }
                let def = COND_IS_DEF.load(Ordering::Relaxed);   // fcd980+fcdaf0=항상 실제 draw
                let e88 = COND_IS_E88.load(Ordering::Relaxed);   // e88a0 실제 draw(count>0)
                let e9 = COND_IS_E9.load(Ordering::Relaxed);     // e9a30 호출
                let real = def + e88;   // 확실한 실제 draw(e9는 count불명이라 별도)
                if draws > 0 {  // RNG 함수 호출한 condgate 케이스. real>0=확실히 desync위험.
                    if !CONDRNG_INIT.swap(true, Ordering::Relaxed) { write_named("condrng.txt", "=== facet#1 condgate in-scope RNG: def(fcd980/af0=실제) e88(count>0) e9(호출). replaced(my≠-99)+real>0=desync위험 ===\n"); }
                    append_named("condrng.txt", &format!("disc={} my={} def={} e88={} e9={} real={} | LEAK누적={} [{}]\n", f.p5, my, def, e88, e9, real, COND_LEAK.load(Ordering::Relaxed),
                        if my != -99 && real > 0 {"★REPLACED+REAL_RNG=desync확실(or누수)"} else if my != -99 && e9 > 0 {"replaced+e9호출(count확인필요)"} else if my == -99 {"passthrough(안전)"} else {"replaced(RNG=0)"}));
                }
                if my == -99 { COND_PEND.fetch_add(1, Ordering::Relaxed); }
                else if my == game_al { COND_OK.fetch_add(1, Ordering::Relaxed); }
                else { COND_DIFF.fetch_add(1, Ordering::Relaxed); }
                let n = COND_ARMED.load(Ordering::Relaxed);
                if n <= COND_ARM_MAX {
                    let verdict = if my == -99 { "pending" } else if my == game_al { "OK" } else { "★DIFF" };
                    let s = format!("{} → game={} [{}] | OK={} DIFF={} PEND={}\n", f.pre, game_al, verdict, COND_OK.load(Ordering::Relaxed), COND_DIFF.load(Ordering::Relaxed), COND_PEND.load(Ordering::Relaxed));
                    if !COND_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("condcmp.txt", "=== facet#1 condgate: my_condgate vs 게임 al (subplan별, -99=pending) ===\n"); }
                    append_named("condcmp.txt", &s);
                }
            } else if f.kind == 7 {
                // facet#4 movepriority: 출력구조체 *p5. code@+0, 필드 +8/+0x10/+0x20/+0x21.
                let op = f.p5;
                let code = rd_i64(op).unwrap_or(-999);
                let my = f.mine;
                // ★④ 출력계약 덤프: disc별 게임 출력구조 head(6 qword + key byte). aux 비-0 오프셋 식별 → replace 재현범위.
                {
                    let di = (f.p6 as usize).min(15);
                    if MPOUT_CNT[di].fetch_add(1, Ordering::Relaxed) < 6 && readable(op, 0x30) {
                        let q: [i64;6] = core::array::from_fn(|k| rd_i64(op + k*8).unwrap_or(0));
                        let b12 = rd_u8(op + 0x12); let b21 = rd_u8(op + 0x21);
                        if !MPOUT_INIT.swap(true, Ordering::Relaxed) { write_named("mpout.txt", "=== ④ movepriority 출력계약: disc별 게임 출력구조 (code@+0, aux 비0 오프셋 식별) ===\n"); }
                        append_named("mpout.txt", &format!("[disc={} code={}] +8={:#x} +0x10={:#x} +0x18={:#x} +0x20={:#x} +0x28={:#x} | b+0x12={} b+0x21={}\n",
                            f.p6, code, q[1], q[2], q[3], q[4], q[5], b12, b21));
                    }
                }
                // ★출력계약 write-set: 진입스냅(MP_ENTRY) vs 현재 *op = sub-judge가 쓴 qword오프셋 비트마스크. code-only(=0b1)/aux 판별.
                if MP_ENTRY_PTR.load(Ordering::Relaxed) == op && readable(op, 0x40) {
                    let mut ws = 0u64;
                    for k in 0..8usize { if rd_u64(op + k*8).unwrap_or(0) != MP_ENTRY[k].load(Ordering::Relaxed) { ws |= 1 << k; } }
                    let di = (f.p6 as usize).min(15);
                    let prev = MP_WS[di].fetch_or(ws, Ordering::Relaxed);
                    if (prev | ws) != prev {   // 새 비트 발견 → 로그
                        if !MP_WS_INIT.swap(true, Ordering::Relaxed) { write_named("mpws.txt", "=== movepriority sub-judge write-set (qword offset 비트: bit0=+0(code) bit1=+8 bit2=+0x10 ...) code-only=0b1 ===\n"); }
                        append_named("mpws.txt", &format!("[disc={}] write-set=0b{:08b} (오프셋: {})\n", f.p6, prev|ws,
                            (0..8).filter(|k| (prev|ws)>>k & 1 == 1).map(|k| format!("+0x{:x}", k*8)).collect::<Vec<_>>().join(",")));
                    }
                }
                if my == -99 { MP_PEND.fetch_add(1, Ordering::Relaxed); }
                else if my == code { MP_OK.fetch_add(1, Ordering::Relaxed); }
                else { MP_DIFF.fetch_add(1, Ordering::Relaxed); }
                let n = MP_ARMED.load(Ordering::Relaxed);
                if n <= 30000 {
                    let verdict = if my == -99 { "pending" } else if my == code { "OK" } else { "★DIFF" };
                    let s = format!("{} → game_code={} [{}] | OK={} DIFF={} PEND={}\n", f.pre, code, verdict, MP_OK.load(Ordering::Relaxed), MP_DIFF.load(Ordering::Relaxed), MP_PEND.load(Ordering::Relaxed));
                    if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority: my_movepriority vs 게임코드 (subplan별, -99=pending) ===\n"); }
                    append_named("mpcmp.txt", &s);
                }
                // ★disc9/11 full-output 검증: write_poke_aux(진입스냅+내writes) vs 게임출력 op byte대조([0..0x22]). 진입스냅(MP_ENTRY)서 시작→게임도 동일진입상태서 write하므로 동일하면 비트동일.
                if (f.p6 == 9 || f.p6 == 11 || f.p6 == 10 || f.p6 == 12 || f.p6 == 4) && MP_AUX_OP.load(Ordering::Relaxed) == op && my != -99 && readable(op, 0x28) {
                    // base=게임출력(op=호출자state+게임writes). 내 writes 적용후 op와 대조 → 내가 쓰는 필드 값일치 검증(MP_ENTRY 스냅 staleness 회피).
                    // 미기록 필드는 op그대로 유지→trivial match(대체모드서도 양쪽 caller값 유지). write-set 완전성은 disasm로 보장. disc9/11=poke, disc10/12=battle(미검증=dead).
                    let mut buf = [0u8; 0x28];
                    for i in 0..0x28usize { buf[i] = rd_u8(op + i); }
                    let p2sj = MP_AUX_P2.load(Ordering::Relaxed);
                    let p6 = MP_AUX_P6.load(Ordering::Relaxed);
                    let bufp = buf.as_mut_ptr() as usize;
                    let wrote = if f.p6 == 4 { write_disc4_aux(bufp, my, p2sj) } else if f.p6 == 9 || f.p6 == 11 { write_poke_aux(bufp, f.p6 == 9, my, p2sj, p6) } else { write_battle_aux(bufp, my, p2sj) };
                    if wrote {
                        let mut diff_at: i64 = -1;
                        for i in 0..0x22usize { if buf[i] != rd_u8(op + i) { diff_at = i as i64; break; } }
                        let ok = diff_at < 0;
                        if ok { POKE_OK.fetch_add(1, Ordering::Relaxed); } else { POKE_DIFF.fetch_add(1, Ordering::Relaxed); }
                        let n2 = POKE_OK.load(Ordering::Relaxed) + POKE_DIFF.load(Ordering::Relaxed);
                        if !ok || n2 % 500 == 0 || n2 <= 5 {
                            if !POKE_INIT.swap(true, Ordering::Relaxed) { write_named("pokecmp.txt", "=== disc9/11 EpicPoke/SerpenPoke full-output 대체검증: write_poke_aux(내재현) vs 게임출력 byte대조 ([0..0x22]) ===\n"); }
                            let verdict = if ok { "OK".to_string() } else { format!("★DIFF@+0x{:x}", diff_at) };
                            let mb: [u8;0x22] = core::array::from_fn(|i| buf[i]);
                            let gb: [u8;0x22] = core::array::from_fn(|i| rd_u8(op + i));
                            append_named("pokecmp.txt", &format!("[poke {}] disc={} code={} [{}] OK={} DIFF={}\n  my  ={:02x?}\n  game={:02x?}\n",
                                n2, f.p6, my, verdict, POKE_OK.load(Ordering::Relaxed), POKE_DIFF.load(Ordering::Relaxed), mb, gb));
                        }
                    }
                }
                // ★disc9/11 RNG footprint 측정: 진입 스냅(POKE_RNG_*) vs exit p4 → 실제 RNG 소비(words/refills). draw 분포·early-guard 상관 파악 → my_poke_rng_final 모델링용.
                if (f.p6 == 9 || f.p6 == 11) {
                    let p4 = POKE_RNG_P4.load(Ordering::Relaxed);
                    if p4 != 0 && readable(p4 + 0x138, 8) {
                        let i0 = POKE_RNG_I0.load(Ordering::Relaxed);
                        let c0 = POKE_RNG_C0.load(Ordering::Relaxed);
                        let i1 = rd_u64(p4 + 0x100).unwrap_or(0);
                        let c1 = rd_u64(p4 + 0x130).unwrap_or(0);
                        let refills = c1.wrapping_sub(c0) / 4;
                        let words = (i1 as i64 + 64 * refills as i64) - i0 as i64;   // 소비 u32 워드수(refill 보정)
                        let n2 = POKE_RNG_N_CTR.fetch_add(1, Ordering::Relaxed);
                        // ★재구성 검증: 예측 exit(POKE_PIDX/PCTR, reconstructed args) vs 실제 p4 exit(i1,c1).
                        let pcount = POKE_PCOUNT.load(Ordering::Relaxed);
                        let (pidx, pctr) = (POKE_PIDX.load(Ordering::Relaxed), POKE_PCTR.load(Ordering::Relaxed));
                        let e_ok = pcount >= 0 && i1 == pidx && c1 == pctr;
                        if pcount >= 0 { if e_ok { POKE_E88_OK.fetch_add(1, Ordering::Relaxed); } else { POKE_E88_DIFF.fetch_add(1, Ordering::Relaxed); } }
                        if !POKERNG_INIT.swap(true, Ordering::Relaxed) { write_named("pokerng.txt", "=== disc9/11 RNG: p4 delta + e88a0 재구성 검증(예측 exit vs 실제). eOK=재구성정확 ===\n"); }
                        if n2 < 4000 || !e_ok {
                            append_named("pokerng.txt", &format!("[pokerng {}] disc={} code={} plan={} | i0={} i1={} refills={} words={} | myCount={} pred(idx={} ctr={}) e88[{}] eOK={} eDIFF={}\n",
                                n2, f.p6, code, POKE_RNG_PLAN.load(Ordering::Relaxed), i0, i1, refills, words,
                                pcount, pidx, pctr, if pcount<0 {"n/a"} else if e_ok {"OK"} else {"★DIFF"},
                                POKE_E88_OK.load(Ordering::Relaxed), POKE_E88_DIFF.load(Ordering::Relaxed)));
                        }
                        POKE_RNG_P4.store(0, Ordering::Relaxed);
                    }
                }
                if f.p6 == 9 || f.p6 == 11 { POKE_INSCOPE.store(false, Ordering::Relaxed); }   // ★RNG caller 추적 윈도우 종료(p4 무관 항상 해제)
                if f.p6 == 14 && code != 18 { defwatch_log(code, my, f.disp_pred); }   // 캡내 disc14의 7-케이스도 watcher 기록
                if f.p6 == 9 && my == 7 && code != 7 {   // ★epic 7-DIFF 진단: 어느 7-출구가 과발동했나
                    let dn = EPICDIAG_N.fetch_add(1, Ordering::Relaxed);
                    if dn < 200 {
                        let d = EPIC_DIAG.load(Ordering::Relaxed);
                        let s = format!("[epicdiff #{}] game={} my=7 | reason={} hp%={} obj_full={} not_home={} side={} self_z7={} other_z7={} obj_hp={} thr_lt={}\n",
                            dn, code, d & 0xf, (d>>4)&0xff, (d>>12)&1, (d>>13)&1, (d>>14)&1, (d>>16)&0xf, (d>>20)&0xf, (d>>24)&0xff, (d>>32)&1);
                        if !EPICDIAG_INIT.swap(true, Ordering::Relaxed) { write_named("epicdiff.txt", "=== EpicPoke 7-DIFF 진단 (reason 1~5 = 어느 return-7) ===\n"); }
                        append_named("epicdiff.txt", &s);
                    }
                }
                if f.p6 == 9 && my == 13 {   // ★engage(13) 진단: 2 DIFF가 champ999==1로 갈리나
                    let en = ENGDIAG_N.fetch_add(1, Ordering::Relaxed);
                    if en < 300 {
                        let d = ENG_DIAG.load(Ordering::Relaxed);
                        let dsq = ENG_DIST.load(Ordering::Relaxed);
                        let thr = 0x53d1ac101u64;   // dist²<thr% : 100=임계바로아래, 작을수록 멀리(=fdae40 의심)
                        let verdict = if code == 13 { "OK" } else { "★DIFF" };
                        let s = format!("[eng #{}] game={} my=13 [{}] champ999={} champ3e6={} side={} dist²={} ({}%of임계)\n",
                            en, code, verdict, (d>>16)&0xff, (d>>24)&0xff, (d>>14)&1, dsq, dsq.saturating_mul(100)/thr);
                        if !ENGDIAG_INIT.swap(true, Ordering::Relaxed) { write_named("epiceng.txt", "=== EpicPoke engage(13) 진단: champ999/champ3e6 (fdae40 게이트) ===\n"); }
                        append_named("epiceng.txt", &s);
                    }
                }
                if f.p6 == 9 && my == 11 && code != 11 {   // ★epic my=11 DIFF 진단: 어느 게이트서 갈렸나
                    let d = EPIC11_DIAG.load(Ordering::Relaxed);
                    let s = format!("[epic11] game={} my=11 | reason={} fdae40={} node2[c1={} c2={} c3={} c4={} c5={} heq={}] zone_app={} side={} flag={} champ999={} champ3e6={} zsf={} zot={} zhp={}\n",
                        code, d&7, (d>>3)&1, (d>>4)&1, (d>>5)&1, (d>>6)&1, (d>>7)&1, (d>>8)&1, (d>>9)&1, (d>>10)&1, (d>>11)&1, (d>>12)&1, (d>>16)&0xff, (d>>24)&0xff, (d>>32)&0xff, (d>>40)&0xff, (d>>48)&0xff);
                    append_named("epic11.txt", &s);
                }
            } else if f.kind == 8 {
                // DefenseNexus 7-watcher(무제한): game!=18(=7) 케이스만 기록
                let code = rd_i64(f.p5).unwrap_or(-999);
                if code != 18 { defwatch_log(code, f.mine, f.disp_pred); }
            } else if f.kind == 9 {
                // ★CAND_FILTER(0x1f4ec60): 게임 출력 Vec @ p5(out_ptr): ptr@0, tag@8, cap@0x10, len@0x18.
                //   f.mine=my_len, f.p6=my_sum(요소합). CAND_PRED에 예측Vec 보관.
                let op = f.p5;
                let g_ptr = rd_u64(op).unwrap_or(0) as usize;
                let g_len = rd_u64(op + 0x18).unwrap_or(0) as usize;
                let my_len = f.mine as usize;
                let my_sum = f.p6;
                let mut g_sum = 0usize; let mut g_elems = String::new();
                let ok_ptr = ptr_ok(g_ptr) && g_len <= 16;   // 5레인이라 len<=5; 16은 안전상한
                if ok_ptr {
                    for i in 0..g_len {
                        let e = rd_u64(g_ptr + i*8).unwrap_or(0) as usize;
                        g_sum = g_sum.wrapping_add(e);
                        if i < 6 { g_elems.push_str(&format!(" {:#x}", e)); }
                    }
                }
                let mine_vec = CAND_PRED.lock().ok().map(|g| if g.0==op { g.1.clone() } else { Vec::new() }).unwrap_or_default();
                let len_ok = my_len == g_len;
                let sum_ok = my_sum == g_sum;
                let verdict = if !ok_ptr { "skip(badptr)" } else if len_ok && sum_ok { "OK" } else { "★DIFF" };
                let n = CAND_LOGGED.fetch_add(1, Ordering::Relaxed);
                if n < 400 {
                    let mine_elems: String = mine_vec.iter().take(6).map(|&x| format!(" {:#x}", x)).collect();
                    let s = format!("{} → game_len={} game_sum={:#x} game[{}] mine[{}] [{}]\n",
                        f.pre, g_len, g_sum, g_elems, mine_elems, verdict);
                    if !CAND_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("candcmp.txt", "=== CAND_FILTER(0x1f4ec60) white-box 검증: game out Vec(len/요소합) vs cand_filter_repro ===\n"); }
                    append_named("candcmp.txt", &s);
                }
            } else if f.kind == 11 {
                // ★0x1f80320: game_draws = F80_DRAWS(스코프 내 fcdaf0 호출수) vs my_draws=f.p6. score는 부분값(참고).
                F80_INSCOPE.store(false, Ordering::Relaxed);
                let game_draws = F80_DRAWS.load(Ordering::Relaxed);
                let my_draws = f.p6 as u64;
                let game = retval as u64;
                let my_score = f.mine as u64;
                let dok = game_draws == my_draws;
                let sok = game == my_score;
                let verdict = if dok && sok { "OK" } else if dok { "score★DIFF" } else { "draw★DIFF" };
                if dok { GB_DRAW_OK.fetch_add(1, Ordering::Relaxed); } else { GB_DRAW_DIFF.fetch_add(1, Ordering::Relaxed); }
                if sok { GB_SCORE_OK.fetch_add(1, Ordering::Relaxed); } else { GB_SCORE_DIFF.fetch_add(1, Ordering::Relaxed); }
                let n = GB_LOGGED.fetch_add(1, Ordering::Relaxed);
                if n < 400 {
                    let s = format!("{} → game_draws={} my_draws={} game_score={} my_score={} [{}] (drawOK={} scoreOK={} scoreDIFF={})\n",
                        f.pre, game_draws, my_draws, game, my_score, verdict,
                        GB_DRAW_OK.load(Ordering::Relaxed), GB_SCORE_OK.load(Ordering::Relaxed), GB_SCORE_DIFF.load(Ordering::Relaxed));
                    if !GB_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("gbcmp.txt", "=== 0x1f80320 검증: game vs my (draw + score) ===\n"); }
                    append_named("gbcmp.txt", &s);
                }
            } else if f.kind == 12 {
                // ★FUN_1420e88a0 RNG exit 검증: my count→gen_range(0,count) 예측 exit(idx/counter) vs 실제 rng exit.
                let rng = f.p5;
                let (gi, gc) = (rd_u64(rng + 0x100).unwrap_or(0), rd_u64(rng + 0x130).unwrap_or(0));
                let (pidx, pctr) = (f.p6 as u64, f.disp_pred as u64);
                let ok = gi == pidx && gc == pctr;
                if ok { E88_OK.fetch_add(1, Ordering::Relaxed); } else { E88_DIFF.fetch_add(1, Ordering::Relaxed); }
                let n = E88_OK.load(Ordering::Relaxed) + E88_DIFF.load(Ordering::Relaxed);
                if !ok || n % 500 == 0 || n <= 20 {
                    if !E88_CMP_INIT.swap(true, Ordering::Relaxed) { write_named("e88acmp.txt", "=== FUN_1420e88a0 RNG-sync: my count→gen_range(0,count) exit vs 실제 rng exit ===\n"); }
                    let verdict = if ok { "OK" } else { "★DIFF" };
                    append_named("e88acmp.txt", &format!("{} → game(idx={} ctr={}) my(idx={} ctr={}) [{}] OK={} DIFF={}\n",
                        f.pre, gi, gc, pidx, pctr, verdict, E88_OK.load(Ordering::Relaxed), E88_DIFF.load(Ordering::Relaxed)));
                }
                // ★선택출력 검증: my (out0,out2) vs 게임 out([out0],[out+0x10])
                if let Ok(mut pk) = E88_PICK.lock() {
                    if let Some(pos) = pk.iter().rposition(|x| x.0 == f.key) {
                        let (_, out_ptr, my_o0, my_o2) = pk.remove(pos);
                        if readable(out_ptr + 0x18, 8) {
                            let g_o0 = rd_u64(out_ptr).unwrap_or(0);
                            let g_o2 = rd_u64(out_ptr + 0x10).unwrap_or(0) as i64;
                            // out0만 비교(둘다 0이면 out2 무의미). out0==1이면 out2도 비교.
                            let ok = my_o0 == g_o0 && (my_o0 == 0 || my_o2 == g_o2);
                            if ok { E88P_OK.fetch_add(1, Ordering::Relaxed); } else { E88P_DIFF.fetch_add(1, Ordering::Relaxed); }
                            let pn = E88P_OK.load(Ordering::Relaxed) + E88P_DIFF.load(Ordering::Relaxed);
                            if !ok || pn % 500 == 0 || pn <= 30 {
                                if !E88P_INIT.swap(true, Ordering::Relaxed) { write_named("e88pick.txt", "=== FUN_1420e88a0 선택출력 검증: my(out0,out2) vs game ===\n"); }
                                append_named("e88pick.txt", &format!("my(o0={} o2={}) game(o0={} o2={}) [{}] OK={} DIFF={}\n",
                                    my_o0, my_o2, g_o0, g_o2, if ok {"OK"} else {"★DIFF"}, E88P_OK.load(Ordering::Relaxed), E88P_DIFF.load(Ordering::Relaxed)));
                            }
                        }
                    }
                }
            } else if f.kind == 13 {
                // ★FUN_1420e9a30(engage draw1) RNG exit 검증: my count→gen_range(0,count) 예측 exit vs 실제 rng exit.
                let rng = f.p5;
                let (gi, gc) = (rd_u64(rng + 0x100).unwrap_or(0), rd_u64(rng + 0x130).unwrap_or(0));
                let (pidx, pctr) = (f.p6 as u64, f.disp_pred as u64);
                let ok = gi == pidx && gc == pctr;
                if ok { E9_OK.fetch_add(1, Ordering::Relaxed); } else { E9_DIFF.fetch_add(1, Ordering::Relaxed); }
                let n = E9_OK.load(Ordering::Relaxed) + E9_DIFF.load(Ordering::Relaxed);
                if !ok || n % 500 == 0 || n <= 40 {
                    if !E9_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("e9a30.txt", "=== FUN_1420e9a30 draw1 검증 ===\n"); }
                    let verdict = if ok { "OK" } else { "★DIFF" };
                    append_named("e9a30rng.txt", &format!("{} → game(idx={} ctr={}) my(idx={} ctr={}) [{}] OK={} DIFF={}\n",
                        f.pre, gi, gc, pidx, pctr, verdict, E9_OK.load(Ordering::Relaxed), E9_DIFF.load(Ordering::Relaxed)));
                }
            } else if f.kind == 14 {
                // ★generic_build 본체(0x20def90) 출력: out struct kind@+0x58 / arg@+0x60 / action sub-Vec(+0x70 ptr, +0x78 len, entry stride 0x18, word=code).
                let out = f.p5;
                let mbase = exe_base();
                let kind = rd_i64(out + 0x58).unwrap_or(-99);
                let arg = rd_u64(out + 0x60).unwrap_or(0) as usize;
                let argr = if arg > mbase && arg < mbase + 0x10000000 { format!("rva+{:#x}", arg - mbase) } else { format!("{:#x}", arg) };
                if GBBODY.load(Ordering::Relaxed) {
                    let sentinel = rd_i64(out).unwrap_or(-99);
                    let hdr8d = (rd_u8(out + 0x8d) as u32) | ((rd_u8(out + 0x8e) as u32) << 8);
                    let (h89, h8a, h8b, h8f) = (rd_u8(out + 0x89), rd_u8(out + 0x8a), rd_u8(out + 0x8b), rd_u8(out + 0x8f));
                    let vlen = rd_u64(out + 0x78).unwrap_or(0);
                    let vptr = rd_u64(out + 0x70).unwrap_or(0) as usize;
                    let mut vcodes = String::new();
                    for i in 0..vlen.min(8) {
                        let p = vptr + (i as usize) * 0x18;
                        let code = (rd_u8(p) as u16) | ((rd_u8(p + 1) as u16) << 8);
                        vcodes.push_str(&format!("{:#x},", code));
                    }
                    let (mk, ma) = (f.mine, f.p6 as u64);   // my_generic_build 예측 (kind, arg)
                    let verdict = if mk == -99 { GBB_NOPRED.fetch_add(1, Ordering::Relaxed); "미예측".to_string() }
                        else if mk == kind && ma == (arg as u64) { GBB_OK.fetch_add(1, Ordering::Relaxed); "OK".to_string() }
                        else { GBB_DIFF.fetch_add(1, Ordering::Relaxed); format!("★DIFF(my k={} a={:#x})", mk, ma) };
                    let s = format!("{} → kind={} arg={} [{}] (OK={} DIFF={} NP={}) sent={} hdr8d={:#x} h89/8a/8b/8f={}/{}/{}/{} vlen={} v=[{}]\n",
                        f.pre, kind, argr, verdict, GBB_OK.load(Ordering::Relaxed), GBB_DIFF.load(Ordering::Relaxed), GBB_NOPRED.load(Ordering::Relaxed),
                        sentinel, hdr8d, h89, h8a, h8b, h8f, vlen, vcodes);
                    if !GBB_FILE_INIT.swap(true, Ordering::Relaxed) {
                        write_named("gbbody.txt", "=== generic_build 본체(0x20def90) 출력 캡처: (disc,p2,team) → (kind@+0x58, arg@+0x60, action Vec) ===\n");
                    }
                    append_named("gbbody.txt", &s);
                }
                // ★gbrd: 0x20e42a3 mid-func 캡처가 저장한 gb_region_d 예측을 out ptr로 조회 → game kind/arg 대조 → gbrdcmp.txt.
                //   같은 invocation서 0x42a3(store) → 함수리턴(여기서 consume). 0x42a3 미도달 invocation은 맵에 없음(=영역D 깊은분기 우회, 1차 무방).
                let gbrd_ent = if let Ok(mut m) = GBRD_MAP.lock() {
                    m.iter().position(|x| x.0 == out).map(|p| m.remove(p))
                } else { None };
                if let Some((_, pred, dump, entry_vlen)) = gbrd_ent {
                    // ★action Vec 검증: 영역 D delta = 최종 len − entry_vlen = 영역 D가 push한 코드. (out+0x78 len, out+0x70 ptr, stride 0x18, word=code)
                    let fvlen = rd_u64(out + 0x78).unwrap_or(0);
                    let dn = fvlen.saturating_sub(entry_vlen);   // 영역 D push 개수
                    if dn > 0 { GBRD_VPUSH.fetch_add(1, Ordering::Relaxed); }
                    // game 영역 D push: dn==0→0 / dn==1→그 코드 / dn>1→0xffff(예상밖)
                    let vptr = rd_u64(out + 0x70).unwrap_or(0) as usize;
                    let game_push: u16 = if dn == 0 { 0 } else if dn == 1 && ptr_ok(vptr) {
                        (rd_u8(vptr + (entry_vlen as usize) * 0x18) as u16) | ((rd_u8(vptr + (entry_vlen as usize) * 0x18 + 1) as u16) << 8)
                    } else { 0xffff };
                    if GBRD.load(Ordering::Relaxed) {   // verify 로깅은 gbrd일 때만(gbrepl 단독시 로그폭증 방지)
                        let ga = arg as u64;
                        let verdict = match pred {
                            Some((pk, pa, ppush)) => if pk == kind && pa == ga && ppush == game_push {
                                GBRD_OK.fetch_add(1, Ordering::Relaxed); "OK".to_string()
                            } else {
                                GBRD_DIFF.fetch_add(1, Ordering::Relaxed); format!("★DIFF(my k={} a={:#x} push={:#x})", pk, pa, ppush)
                            },
                            None => { GBRD_NP.fetch_add(1, Ordering::Relaxed); "미예측(영역D 분기 TODO)".to_string() }
                        };
                        let mut dcodes = String::new();
                        if dn > 0 && ptr_ok(vptr) {
                            for i in entry_vlen..fvlen.min(entry_vlen + 8) {
                                let p = vptr + (i as usize) * 0x18;
                                let code = (rd_u8(p) as u16) | ((rd_u8(p + 1) as u16) << 8);
                                dcodes.push_str(&format!("{:#x},", code));
                            }
                        }
                        let s = format!("[gbrd] game kind={} arg={} push={:#x} [{}] (OK={} DIFF={} NP={}) Dvec(d={} ev={} [{}]) | {}\n",
                            kind, argr, game_push, verdict, GBRD_OK.load(Ordering::Relaxed), GBRD_DIFF.load(Ordering::Relaxed), GBRD_NP.load(Ordering::Relaxed),
                            dn, entry_vlen, dcodes, dump);
                        if !GBRD_FILE_INIT.swap(true, Ordering::Relaxed) {
                            write_named("gbrdcmp.txt", "=== 영역 D gb_region_d 검증: 캡처 locals → 예측 vs game out (kind/arg) + Dvec(영역D push delta) ===\n");
                        }
                        append_named("gbrdcmp.txt", &s);
                    }
                    // ★대체(gbrepl)는 에필로그 hook(gbrd_epilogue_apply)이 100% inline 처리 → kind14서 제거.
                    let _ = pred;
                }
            } else if f.kind == 20 {
                // ★영역 D callee(0x203cb30/0x20c0690) 순수 점수: game retval(rax,u64) vs my(f.mine). p5=함수id(203/690).
                let game = retval as u64;
                let mine = f.mine as u64;
                let ok = game == mine;
                let is203 = f.p5 == 203;
                if is203 { if ok { GBC203_OK.fetch_add(1, Ordering::Relaxed); } else { GBC203_DIFF.fetch_add(1, Ordering::Relaxed); } }
                else     { if ok { GBC690_OK.fetch_add(1, Ordering::Relaxed); } else { GBC690_DIFF.fetch_add(1, Ordering::Relaxed); } }
                let n = GBC_LOGGED.fetch_add(1, Ordering::Relaxed);
                if !ok || n < 400 {
                    let s = format!("{} → game={} [{}] (203: OK={} DIFF={} | 690: OK={} DIFF={})\n",
                        f.pre, game, if ok {"OK"} else {"★DIFF"},
                        GBC203_OK.load(Ordering::Relaxed), GBC203_DIFF.load(Ordering::Relaxed),
                        GBC690_OK.load(Ordering::Relaxed), GBC690_DIFF.load(Ordering::Relaxed));
                    if !GBC_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("gbcalleecmp.txt", "=== 영역 D callee 검증: game retval vs my_203cb30/my_20c0690 (cfg gbcallee, kind20) ===\n"); }
                    append_named("gbcalleecmp.txt", &s);
                }
            } else {
                // RE: retval=puVar3(출력ptr) → 결정=*retval. e1=game임계값(local_b0), e2=셀렉터, e3=idx, e4=df1da0반환.
                let decision = if ptr_ok(retval as usize) { rd_i64(retval as usize).unwrap_or(0) } else { 0 };
                let pred = f.mine;                          // -1=retreat, 0=none, 9999=proceed(예측없음)
                let has_pred = pred != 9999;
                let violation = has_pred && decision != pred;
                // ★2단계 검증: 임계값 충실재현(결정론적, RNG無). 디컴 332-392:
                //   local_228==1: arr=[[[p6+8]+0x20]+8]; data=[arr+idx*0x10]; vt=[arr+8+idx*0x10]; role=(*[vt+0x68])(data); {4:100,3:70,2:50,_:30}
                //   else: (local_50==1)?(p5[0x7a]?0:10):0
                let sel = SEL228_FRESH.load(Ordering::Relaxed);  // df0c10훅이 잡은 신선 셀렉터(e2는 리턴때 덮임)
                let my_thr: i64 = if (sel as i32) == 1 {
                    let a = rd_u64(f.p6 + 8).unwrap_or(0) as usize;
                    let b = rd_u64(a + 0x20).unwrap_or(0) as usize;
                    let arr = rd_u64(b + 8).unwrap_or(0) as usize;
                    let idx = e3 as usize;
                    if ptr_ok(arr) && idx < 64 && readable(arr + idx*0x10, 0x10) {
                        let data = rd_u64(arr + idx*0x10).unwrap_or(0) as usize;
                        let vt = rd_u64(arr + 8 + idx*0x10).unwrap_or(0) as usize;
                        if ptr_ok(data) && ptr_ok(vt) && readable(vt + 0x68, 8) {
                            let g = rd_u64(vt + 0x68).unwrap_or(0) as usize;
                            if ptr_ok(g) {
                                let gf: Getter1 = core::mem::transmute(g);
                                match gf(data) { 4 => 100, 3 => 70, 2 => 50, _ => 30 }
                            } else { -777 }
                        } else { -777 }
                    } else { -777 }
                } else {
                    if e4 == 1 { if rd_i64(f.p5 + 0x7a*8).unwrap_or(0) != 0 { 0 } else { 10 } } else { 0 }
                };
                // e1(게임 local_b0)은 2차게이트 우회시 garbage. 유효 임계값(0/10/30/50/70/100)일때만 비교.
                let e1_valid = matches!(e1, 0|10|30|50|70|100);
                let thr_v = if my_thr == -777 { "thr:가드스킵" }
                    else if !e1_valid { "thr:bypass(game garbage)" }
                    else if e1 == my_thr { "thr:OK✓" } else { "thr:★DIFF" };
                let n = RE_LOGGED.load(Ordering::Relaxed);
                // ★교전롤 → out 예측: roll>=thr→-1(퇴각), roll<thr→5(교전). (검증됨 273/273)
                let roll_v = if PRED_ROLL_VALID.load(Ordering::Relaxed) && (decision == 5 || decision == -1) && matches!(my_thr, 0|10|30|50|70|100) {
                    let roll = PRED_ROLL.load(Ordering::Relaxed);
                    let my_out = if roll >= my_thr { -1 } else { 5 };
                    ROLL_LOGGED.fetch_add(1, Ordering::Relaxed);
                    format!(" | ★ROLL roll={} thr={} my_out={} [{}]{}", roll, my_thr, my_out,
                        if my_out == decision { "ROLL-OK✓" } else { "ROLL-★DIFF" },
                        if RNG_REPL.load(Ordering::Relaxed) { format!(" (REPL N={})", RNG_REPL_N.load(Ordering::Relaxed)) } else { String::new() })
                } else { String::new() };
                let _ = violation;
                let dtag = if decision == 5 { "ENGAGE(5)" } else if decision == 7 { "RECALL(7)" } else if decision == -1 { "RETREAT(-1)" } else if decision == 0 { "NONE(0)" } else if decision == 3 { "ZONE(3)" } else if decision == 8 { "STAND(8)" } else { "OTHER" };
                let pv = if has_pred { if decision == pred { "PRED-OK" } else { "★PRED-VIOLATION" } } else { "proceed" };
                // ★my_dispatch_code 라이브 검증: f.disp_pred(진입시 예측 7/8/3) vs 실제 decision
                // ★완전정복 갭측정: my_dispatch_code 예측(7/8/3)이 실제 디스패치 출력과 일치? 아니면 roll/none으로 빠짐(mispredict)?
                // ★2026-06-19 수정: disp_pred는 "dispatch 도달시" 조건부 예측 → proceed(lp_pred=9999) 케이스만 MISPREDICT 집계.
                //   (lane_pred=0 퇴각 케이스는 my_full이 lp_pred로 -1 산출=정답이므로 disp_pred 무관 → 오집계 방지.)
                let is_misp = f.mine == 9999 && matches!(f.disp_pred, 3|7|8) && !matches!(decision, 3|7|8);
                let disp_v = if matches!(decision, 3|7|8) {
                    let md = f.disp_pred;
                    if md == decision { DISP_OK.fetch_add(1, Ordering::Relaxed); format!(" | ★DISP mydisp={} [DISP-OK✓]", md) }
                    else { DISP_DIFF.fetch_add(1, Ordering::Relaxed); format!(" | ★DISP mydisp={} [DISP-★DIFF]", md) }
                } else if is_misp {
                    DISP_DIFF.fetch_add(1, Ordering::Relaxed);
                    format!(" | ★DISP mydisp={} actual={} [★MISPREDICT(예측디스패치→실제roll/none)]", f.disp_pred, decision)
                } else { String::new() };
                // ★통합 출력 예측 my_full = lp_pred(lane/none) + dispatch + roll → 전 출력(-1/0/3/5/7/8) game==mine 측정.
                //   f.mine=lp_pred(0=none/-1=lane퇴각/9999=proceed). proceed면 disp_pred(3/7/8) or roll(5/-1).
                let roll_out: i64 = if PRED_ROLL_VALID.load(Ordering::Relaxed) && matches!(my_thr, 0|10|30|50|70|100) {
                    if PRED_ROLL.load(Ordering::Relaxed) >= my_thr { -1 } else { 5 }
                } else { -777 };
                let my_full: i64 =
                    if f.mine == 0 { 0 } else if f.mine == -1 { -1 }
                    else if matches!(f.disp_pred, 3|7|8) { f.disp_pred } else { roll_out };
                let full_v = if my_full == -777 { String::new() }
                    else if my_full == decision { FULL_OK.fetch_add(1, Ordering::Relaxed); String::new() }
                    else { FULL_DIFF.fetch_add(1, Ordering::Relaxed); format!(" [★FULL-DIFF myfull={} act={}]", my_full, decision) };
                let is_full_diff = !full_v.is_empty();
                let s = format!("{} → out={} [{} {}] | sel_fresh={} idx={} game_thr={} my_thr={} [{}]{}{}{}\n", f.pre, decision, dtag, pv, sel, e3, e1, my_thr, thr_v, roll_v, disp_v, full_v);
                // ★디스패치(3/7/8) + cVar6==2 + ★FULL-DIFF(통합예측 틀린것=진짜갭) → dispcmp.txt 고캡(2000).
                let is_cv2 = f.pre.contains("cVar6=2 ");
                if matches!(decision, 3|7|8) || is_cv2 || is_full_diff {
                    if DISP_LOGGED.fetch_add(1, Ordering::Relaxed) < 2000 { append_named("dispcmp.txt", &s); }
                } else if (n < 60 || (sel as i32) == 1) && n < 120 {
                    RE_LOGGED.fetch_add(1, Ordering::Relaxed);
                    append_named("recmp.txt", &s);
                }
                // ★engage footprint 측정: 진입 스냅 → 출력별 총 RNG delta(words = refills*64 + i1 - i0). engfoot.txt.
                if let Ok(mut sn) = RE_SNAP.lock() {
                    if let Some(pos) = sn.iter().rposition(|x| x.0 == f.key) {
                        let (_, state, i0, c0, pred_out, pred_words, pca, pcb) = sn.remove(pos);
                        if readable(state + 0x138, 8) {
                            let i1 = rd_u64(state + 0x100).unwrap_or(0);
                            let c1 = rd_u64(state + 0x130).unwrap_or(0);
                            let refills = c1.wrapping_sub(c0) / 4;
                            let words = refills.wrapping_mul(64).wrapping_add(i1).wrapping_sub(i0) as i64;
                            // ★engage 예측 검증: pred_out/pred_words(my_engage_predict) vs 실제 (decision, words). pred=-777=비engage(skip).
                            if pred_out != -777 {
                                let ok = pred_out == decision && pred_words == words;
                                if ok { EP_OK.fetch_add(1, Ordering::Relaxed); } else { EP_DIFF.fetch_add(1, Ordering::Relaxed); }
                                let pn = EP_OK.load(Ordering::Relaxed) + EP_DIFF.load(Ordering::Relaxed);
                                if !ok || pn % 500 == 0 || pn <= 40 {
                                    if !EFOOT_INIT.swap(true, Ordering::Relaxed) { write_named("engfoot.txt", "=== engage 예측검증: my_engage_predict(out,words) vs 실제(decision,words). gate early-exit은 DIFF로 노출 ===\n"); }
                                    let roll_fired = PRED_ROLL_VALID.load(Ordering::Relaxed);
                                    append_named("engfoot.txt", &format!("[ep {}] my(out={} words={}) game(out={} words={}) [{}] | i0={} i1={} refills={} ca={} cb={} roll_fired={} EP_OK={} EP_DIFF={}\n",
                                        pn, pred_out, pred_words, decision, words, if ok {"OK"} else {"★DIFF"}, i0, i1, refills, pca, pcb, roll_fired, EP_OK.load(Ordering::Relaxed), EP_DIFF.load(Ordering::Relaxed)));
                                }
                            }
                        }
                    }
                }
            }
            }));
            if hr.is_err() {
                let c = HR_PANIC.fetch_add(1, Ordering::Relaxed);
                if c < 30 { append_named("recmp.txt", &format!("[★PANIC caught] hook_return kind={} — verify 건너뜀(orig_ret 정상복귀)\n", f.kind)); }
            }
            ret
        }
        None => 0,
    }
}
// 함수의 rdx 반환을 받는 시wim: shim(target,a,b,c,d) → target(a,b,c,d) 호출 후 rdx 리턴
static SHIM_RDX: AtomicUsize = AtomicUsize::new(0);
static SHIM_BOTH: AtomicUsize = AtomicUsize::new(0);   // ★단일호출 2값캡처(소환수 비멱등 게터 대응)
type Shim5 = unsafe extern "C" fn(usize, usize, usize, usize, usize) -> i64;
type Getter4 = unsafe extern "C" fn(usize, usize, usize, usize) -> i64;
type ShimBoth = unsafe extern "C" fn(usize, usize, usize, usize, usize, usize);  // (out[2], getter, a,b,c,d)
// ★게터를 1회만 호출 → (rax,rdx) 둘 다 out[0],out[1]에 기록. 비멱등 게터(소환수)도 정확.
unsafe fn build_shim_both() {
    let code: [u8; 43] = [
        0x53,                       // push rbx
        0x48,0x89,0xCB,             // mov rbx, rcx        (out)
        0x49,0x89,0xD2,             // mov r10, rdx        (getter)
        0x4C,0x89,0xC1,             // mov rcx, r8         (getter arg1=a)
        0x4C,0x89,0xCA,             // mov rdx, r9         (getter arg2=b)
        0x4C,0x8B,0x44,0x24,0x30,   // mov r8, [rsp+0x30]  (arg3=c, +8 for pushed rbx)
        0x4C,0x8B,0x4C,0x24,0x38,   // mov r9, [rsp+0x38]  (arg4=d)
        0x48,0x83,0xEC,0x20,        // sub rsp,0x20        (shadow, 16-align 유지)
        0x41,0xFF,0xD2,             // call r10            (getter 1회)
        0x48,0x83,0xC4,0x20,        // add rsp,0x20
        0x48,0x89,0x03,             // mov [rbx], rax      (out[0]=base1)
        0x48,0x89,0x53,0x08,        // mov [rbx+8], rdx    (out[1]=base2)
        0x5B,                       // pop rbx
        0xC3,                       // ret
    ];
    let m = VirtualAlloc(0, 64, 0x1000|0x2000, 0x40);
    if m != 0 { core::ptr::copy_nonoverlapping(code.as_ptr(), m as *mut u8, code.len()); SHIM_BOTH.store(m, Ordering::Relaxed); }
}
unsafe fn build_shim_rdx() {
    let code: [u8; 32] = [
        0x49,0x89,0xCA,             // mov r10, rcx (target)
        0x48,0x89,0xD1,             // mov rcx, rdx (a)
        0x4C,0x89,0xC2,             // mov rdx, r8  (b)
        0x4D,0x89,0xC8,             // mov r8, r9   (c)
        0x4C,0x8B,0x4C,0x24,0x28,   // mov r9, [rsp+0x28] (d)
        0x48,0x83,0xEC,0x28,        // sub rsp,0x28
        0x41,0xFF,0xD2,             // call r10
        0x48,0x83,0xC4,0x28,        // add rsp,0x28
        0x48,0x89,0xD0,             // mov rax, rdx
        0xC3,                       // ret
    ];
    let m = VirtualAlloc(0, 64, 0x1000|0x2000, 0x40);
    if m != 0 { core::ptr::copy_nonoverlapping(code.as_ptr(), m as *mut u8, code.len()); SHIM_RDX.store(m, Ordering::Relaxed); }
}
// ★engage pre-gate(0x2080760) 호출 shim: pregate_shim(target, p1, p2, p5, p6, arg9)→al.
//   0x2080760(rcx=p1, rdx=p2, r8=0, r9=0, [rsp+0x20]=p5, [rsp+0x28]=p6, [rsp+0x30]=arg9, [rsp+0x38]=0).
// ★[제거됨] build_pregate_shim/PREGATE_SHIM: 0x2080760을 머신코드 thunk로 직접호출했으나
//   스택인자 오프셋 off-by-8 버그(arg5/6를 8B 낮게 읽음)로 게임에 garbage roster ptr 전달→freeze.
//   완전대체 원칙(게임함수 호출X)에 따라 my_pregate(순수Rust 재현)로 대체. shim 영구 폐기.
// 엔티티 e의 base-dmg 게터 호출 → (rax=물리?, rdx=마법?)
unsafe fn probe_basedmg(e: usize, local_80: usize, exe: usize) -> (i64, i64) {
    probe_basedmg_r9(e, local_80, exe, exe + RVA_ABILITY_TABLE)
}
// r9(소환수 게터가 읽는 테이블) 명시 버전. 엔게이지TTD=ability_table(0x3599b30), disc4=ATK_VT(0x35e4d00).
unsafe fn probe_basedmg_r9(e: usize, local_80: usize, exe: usize, r9_addr: usize) -> (i64, i64) {
    let _ = exe;
    let v480 = rd_u64(e + 0x480).unwrap_or(0) as usize;
    if !ptr_ok(v480) { return (-1, -1); }
    let inner = rd_u64(v480 + 0x10).unwrap_or(0) as usize;
    let buf = rd_u64(e + 0x478).unwrap_or(0) as usize;
    let aligned = (inner.wrapping_sub(1) & !0xf).wrapping_add(buf).wrapping_add(0x10);
    let gptr = rd_u64(v480 + 0x28).unwrap_or(0) as usize;
    // ★r9 = 호출부별 테이블. 챔피언 게터는 rcx만 쓰지만 소환수 게터는 r9을 읽음 → stale 값이면 소환수 base 깨짐(DIFF).
    let vt = r9_addr;
    if !ptr_ok(gptr) || !ptr_ok(aligned) { return (-2, -2); }
    // ★단일호출 2값캡처(소환수 비멱등 게터 정확): 게터 1회 → (rax,rdx)=out[0],out[1].
    let both = SHIM_BOTH.load(Ordering::Relaxed);
    if both != 0 {
        let mut o = [0i64; 2];
        let s: ShimBoth = core::mem::transmute(both);
        s(o.as_mut_ptr() as usize, gptr, aligned, local_80, e, vt);
        return (o[0], o[1]);
    }
    // 폴백(구 2호출, 멱등 챔프엔 정확): both-shim 빌드실패시.
    let getter: Getter4 = core::mem::transmute(gptr);
    let rax = getter(aligned, local_80, e, vt);
    let shim = SHIM_RDX.load(Ordering::Relaxed);
    let rdx = if shim != 0 { let s: Shim5 = core::mem::transmute(shim); s(gptr, aligned, local_80, e, vt) } else { -3 };
    (rax, rdx)
}
type Vt30 = unsafe extern "C" fn(*mut u8, usize);
type Vt38 = unsafe extern "C" fn(usize) -> usize;
type CombatFn = unsafe extern "C" fn(usize, usize, usize, usize, i64, u32, i32) -> i64;

unsafe fn my_combat_dmg(atk: usize, tgt: usize, base: i64, dtype: u32, flag: i32, exe: usize) -> i64 {
    if flag != 0 && flag != 1 { return base; }
    // ★풀재현(2026-06-18): vtable getter 호출 제거 → 엔티티 오프셋 직접읽기(우리 손으로 다 구현).
    //   실측 getter: +0x38(0x18ba1c0)=`lea rax,[e+0x358]`(계수시트), +0x30(0x18ba210)=`copy e+0x600..0x640→out`.
    //   ⇒ sheet=e+0x358, 유효스탯블록=e+0x600(tb[0x10]=e+0x610,[0x18]=e+0x618방어,[0x20]=e+0x620마저).
    let _ = (RVA_ATK_VT, RVA_TGT_VT);  // vtable 더이상 불필요
    let sheet = atk + 0x358;             // 계수시트(coef-sheet, getter=lea[e+0x358])
    let local_a8 = rd_i64(tgt + 0x610).unwrap_or(0);
    let local_a0 = rd_i64(tgt + 0x618).unwrap_or(0);   // phys armor
    let local_98 = rd_i64(tgt + 0x620).unwrap_or(0);   // magic resist
    let local_60 = rd_i64(atk + 0x610).unwrap_or(0);   // 공격 스탯
    let s = |o: usize| rd_i64(sheet + o).unwrap_or(0);
    let coef = s(0xd8);
    let mut p5 = base;
    let uvar6: i64;
    if dtype.wrapping_sub(2) < 2 {
        uvar6 = ((s(0xf0) + 100) * p5) / 100;
    } else {
        let mut amp = false;
        if dtype == 0 { p5 += (s(0xd0) * local_a8) / 100; }
        else { p5 += (s(0xe0) * local_a8) / 100; if (dtype & 6) == 2 { amp = true; } }
        uvar6 = if amp { ((s(0xf0) + 100) * p5) / 100 } else { (coef * local_60) / 100 + p5 };
    }
    // ★ override 데모: 계산된 데미지에 유저 배수(100=원본). 항상 보임.
    let uvar6 = uvar6 * COEF_MULT_PCT / 100;
    let (resist, stat) = if flag == 0 { (s(0xa8), local_a0) } else { (s(0xb0), local_98) };
    let lv4 = if (resist as u64) < 0x65 { 100 - resist } else { 0 };
    let a = ((lv4 * stat) as u64) >> 2;
    let b = ((a as u128 * 0x28f5c28f5c28f5c3u128) >> 64) as u64;
    let uvar5 = (b >> 2) + 100;
    let r = if uvar5 != 0 { ((uvar6 as u64) * 100) / uvar5 } else { 0 };
    (r + (r == 0) as u64) as i64
}

// ── 로스터 시그니처: 후보 c가 진짜 plan_base인지 점수(유효 챔프 수, 0..10) ──
unsafe fn roster_sig(c: usize) -> usize {
    if !ptr_ok(c) { return 0; }
    let mut cnt = 0;
    for team in 0..2usize {
        let base = c + team*ROSTER_STRIDE + ROSTER_BASE;
        if !readable(base, ROSTER_N*8) { continue; }
        for i in 0..ROSTER_N {
            let e = rd_u64(base + i*8).unwrap_or(0) as usize;
            if e <= 0x10000 || !readable(e, 0x740) { continue; }
            let sp = rd_i64(e + E_SPEED).unwrap_or(0);
            let x  = rd_i64(e + E_POSX).unwrap_or(-1);
            let hp = rd_i64(e + E_HP).unwrap_or(-1);
            if sp > 0 && x >= 0 && x < 2_000_000 && hp > 0 && hp < 1_000_000 { cnt += 1; }
        }
    }
    cnt
}

// ── CONFIG 파일 ──
const CFG_TEMPLATE: &str = "\
# plan_reimpl 설정 — 저장하면 게임 중에도 즉시 반영(핫리로드).
# enabled : 1=오버라이드 ON, 0=OFF(원본 그대로, 안전 기본값)
# team    : 오버라이드 대상 팀 (0 / 1 / -1=양팀)
# x, y    : 강제 이동 목표 좌표 (맵중앙 = 480000,480000)
# coef_mult: 재구현 데미지 coef 배수(%). 100=게임과동일, 150=1.5배 (override 데모)
# capture : 1=하네스 캡처 ON(TTD/RE). 원하는 경기 들어가서 1로 저장하면 그때부터 캡처
#           (0→1 전환시 카운터·로그파일 리셋 → 데모화면 배경전투에 예산 안뺏김). 끝나면 0.
# replace : 1=retreat_engage 결정을 우리코드로 대체(★실제 정글AI 행동변경). 기본0=원본통과.
#           1단계=검증된 퇴각경로만 대체. 위험하니 데모/리플레이서 먼저.
# repl_out : replace=1일때 우리가 대체하는 케이스의 출력값. -1=원본동일(퇴각), 5=교전, 7=귀환.
#           ★ -1이면 행동 동일(메커니즘 테스트), 5로 바꾸면 그 정글러가 교전하게 됨(override 시연).
# move    : 1=facet#2 이동 override(★모든 챔피언 이동타깃 강제). tag==1(Move) 결정의 x/y를 아래로 덮어씀.
# move_x/y: 강제 이동 목표(cell-center 좌표=cell*32000+16000, 맵중앙~336000). move=1일때만.
enabled = 0
team = 0
x = 480000
y = 480000
coef_mult = 100
capture = 0
replace = 0
repl_out = -1
move = 0
move_x = 336000
move_y = 336000
";
fn mtime_ms(p: &PathBuf) -> u64 {
    fs::metadata(p).and_then(|m| m.modified()).ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map(|d| d.as_millis() as u64).unwrap_or(0)
}
static CFG_POLL_CTR: AtomicU64 = AtomicU64::new(0);   // ★성능: load_cfg 매프레임 stat 스로틀
fn load_cfg(force: bool) -> bool {
    let p = match pth("plan_reimpl.cfg") { Some(p) => p, None => return false };
    // ★성능(2026-06-22): post_update가 매 UI프레임 load_cfg(false) 호출 → fs::metadata(mtime)+exists syscall이 디스크 바쁠때 메인스레드 히치(관전 멈춤#2). force 아니면 30프레임당 1회만 stat. 핫리로드 ~0.5s 지연=무해, cfg값 동일.
    if !force && CFG_POLL_CTR.fetch_add(1, Ordering::Relaxed) % 30 != 0 { return false; }
    if !p.exists() { let _ = fs::write(&p, CFG_TEMPLATE); }
    let mt = mtime_ms(&p);
    if !force && mt == CFG_MTIME.load(Ordering::Relaxed) { return false; }
    CFG_MTIME.store(mt, Ordering::Relaxed);
    let txt = match fs::read_to_string(&p) { Ok(t) => t, Err(_) => return false };
    let mut new_tune: TuneMap = HashMap::default();   // ★lock-free + FNV 해셔: 파싱 누적 후 일괄 게시
    for line in txt.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        if let Some((k, v)) = line.split_once('=') {
            let (k, v) = (k.trim(), v.trim());
            match k {
                "enabled" => OV_ENABLED.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed),
                "dmgcap" => DMGCAP.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed),
                "team" => { if let Ok(n)=v.parse() { OV_TEAM.store(n, Ordering::Relaxed); } }
                "x" => { if let Ok(n)=v.parse() { OV_X.store(n, Ordering::Relaxed); } }
                "y" => { if let Ok(n)=v.parse() { OV_Y.store(n, Ordering::Relaxed); } }
                "coef_mult" => { if let Ok(n)=v.parse() { OV_COEF_MULT.store(n, Ordering::Relaxed); } }
                "capture" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    // 0→1 전환: 카운터·파일 리셋 (원하는 경기에서 깨끗하게 캡처 시작)
                    if on && !CAP_ON.load(Ordering::Relaxed) {
                        RE_ARMED.store(0, Ordering::Relaxed);
                        RE_LOGGED.store(0, Ordering::Relaxed);
                        DISP_LOGGED.store(0, Ordering::Relaxed);
                        DISP_OK.store(0, Ordering::Relaxed);
                        DISP_DIFF.store(0, Ordering::Relaxed);
                        write_named("dispcmp.txt", "=== 디스패치(3/7/8) 캡처 — my_dispatch_code 라이브검증(DISP-OK/DIFF) ===\n");
                        TTD_ARMED.store(0, Ordering::Relaxed);
                        TTD_NONEMPTY.store(0, Ordering::Relaxed);
                        RE_FILE_INIT.store(false, Ordering::Relaxed);
                        TTD_FILE_INIT.store(false, Ordering::Relaxed);
                        // recmp.txt 리셋 + 헤더(켜짐 확인; 이후 out≠-1만 append)
                        write_named("recmp.txt", "=== RE capture ON — 결과≠-1(교전5/귀환7)만 기록 ===\n");
                    }
                    CAP_ON.store(on, Ordering::Relaxed);
                }
                "replace" => REPL_ON.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed),
                "disppred" => DISPPRED.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed),
                "repl_out" => { if let Ok(n)=v.parse() { REPL_OUT.store(n, Ordering::Relaxed); } }
                "move" => MOVE_ON.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed),
                "move_x" => { if let Ok(n)=v.parse() { MOVE_X.store(n, Ordering::Relaxed); } }
                "move_y" => { if let Ok(n)=v.parse() { MOVE_Y.store(n, Ordering::Relaxed); } }
                "move_tag" => { if let Ok(n)=v.parse() { MOVE_TAG.store(n, Ordering::Relaxed); } }
                "move_off" => { if let Ok(n)=v.parse() { MOVE_OFF.store(n, Ordering::Relaxed); } }
                "engage_base" => { if let Ok(n)=v.parse() { ENGAGE_BASE.store(n, Ordering::Relaxed); } }
                "engage_thr_mult" => { if let Ok(n)=v.parse() { ENGAGE_THR_MULT.store(n, Ordering::Relaxed); } }
                "dd7cap" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !DD7CAP.load(Ordering::Relaxed) {
                        DD7_ARMED.store(0, Ordering::Relaxed);
                        DD7_LOGGED.store(0, Ordering::Relaxed);
                        DD7_FILE_INIT.store(false, Ordering::Relaxed);
                    }
                    DD7CAP.store(on, Ordering::Relaxed);
                }
                "rngcap" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !RNGCAP.load(Ordering::Relaxed) {
                        RNG_ARMED.store(0, Ordering::Relaxed);
                        RNG_LOGGED.store(0, Ordering::Relaxed);
                        RNG_FILE_INIT.store(false, Ordering::Relaxed);
                        CHACHA_ARMED.store(0, Ordering::Relaxed);
                        CHACHA_LOGGED.store(0, Ordering::Relaxed);
                        CHACHA_FILE_INIT.store(false, Ordering::Relaxed);
                    }
                    RNGCAP.store(on, Ordering::Relaxed);
                }
                "rng_repl" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !RNG_REPL.load(Ordering::Relaxed) { RNG_REPL_N.store(0, Ordering::Relaxed); }
                    RNG_REPL.store(on, Ordering::Relaxed);
                }
                "engage_repl" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !ENGAGE_REPL.load(Ordering::Relaxed) { ENGAGE_REPL_N.store(0, Ordering::Relaxed); ENGAGE_REPL_PASS.store(0, Ordering::Relaxed); }
                    ENGAGE_REPL.store(on, Ordering::Relaxed);
                }
                "numbers_margin" => { if let Ok(n) = v.trim().parse::<i64>() { NUMBERS_MARGIN.store(n, Ordering::Relaxed); } }   // ★인원수 회피: 0=off, ≥1=적−아군≥n이면 후퇴
                "numbers_range" => { if let Ok(n) = v.trim().parse::<u64>() { if n > 0 { NUMBERS_RANGE.store(n, Ordering::Relaxed); } } }   // ★인원수 카운트 근접반경
                "numbers_threat" => { if let Ok(n) = v.trim().parse::<i64>() { NUMBERS_THREAT.store(n, Ordering::Relaxed); } }   // ★일반교전 전력(force)승산 임계 0~100(≥승산이면 후퇴)
                "towercap" => { let on=v=="1"||v.eq_ignore_ascii_case("true"); if on { TOWERCAP_N.store(0, Ordering::Relaxed); NUM_MAXENEMY.store(0, Ordering::Relaxed); } TOWERCAP.store(on, Ordering::Relaxed); }   // ★캡처 toggle: on이면 항상 카운터 리셋(핫리로드 대응)
                "tower_threat" => { if let Ok(n) = v.trim().parse::<i64>() { TOWER_THREAT.store(n, Ordering::Relaxed); } }   // ★포탑 회피 강도 0~100(0=off, 100=tower_range 전체서 후퇴)
                "tower_range" => { if let Ok(n) = v.trim().parse::<u64>() { if n > 0 { TOWER_RANGE.store(n, Ordering::Relaxed); } } }   // ★포탑 위험반경(threat=100 기준)
                "stat_influence" => { if let Ok(n) = v.trim().parse::<i64>() { STAT_INFLUENCE.store(n.clamp(0, 100), Ordering::Relaxed); } }   // ★성향스탯 보정강도 0~100(0=비트동일)
                "cond_repl" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !COND_REPL.load(Ordering::Relaxed) { COND_REPL_N.store(0, Ordering::Relaxed); }
                    COND_REPL.store(on, Ordering::Relaxed);
                }
                "mp_repl" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !MP_REPL.load(Ordering::Relaxed) { MP_REPL_N.store(0, Ordering::Relaxed); MP_REPL_PASS.store(0, Ordering::Relaxed); }
                    MP_REPL.store(on, Ordering::Relaxed);
                }
                "dd7_repl" => { DD7_REPL.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }
                "poke_repl" => { POKE_REPL.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }
                "recall_repl" => { let on=v=="1"||v.eq_ignore_ascii_case("true"); if on && !RECALL_REPL.load(Ordering::Relaxed) { RECALL_REPL_N.store(0,Ordering::Relaxed); RECALL_REPL_PASS.store(0,Ordering::Relaxed); } RECALL_REPL.store(on, Ordering::Relaxed); }
                "pgcap" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !PGCAP.load(Ordering::Relaxed) {
                        PG_ARMED.store(0, Ordering::Relaxed);
                        PG_FILE_INIT.store(false, Ordering::Relaxed);
                    }
                    PGCAP.store(on, Ordering::Relaxed);
                }
                "pg_a" => { if let Ok(n)=v.parse() { PG_OV_A.store(n, Ordering::Relaxed); } }
                "pg_b" => { if let Ok(n)=v.parse() { PG_OV_B.store(n, Ordering::Relaxed); } }
                "pg_c" => { if let Ok(n)=v.parse() { PG_OV_C.store(n, Ordering::Relaxed); } }
                "tecap" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !TECAP.load(Ordering::Relaxed) {
                        TE_ARMED.store(0, Ordering::Relaxed);
                        TE_CALLS.store(0, Ordering::Relaxed);
                        TE_TRANS_N.store(0, Ordering::Relaxed);
                        TE_FILE_INIT.store(false, Ordering::Relaxed);
                        if let Ok(mut tr) = TE_TRACK.lock() { tr.clear(); }
                        for k in 0..16 { TE_PHASE_HIST[k].store(0, Ordering::Relaxed); TE_SUB_HIST[k].store(0, Ordering::Relaxed); }
                    }
                    TECAP.store(on, Ordering::Relaxed);
                }
                "recallcap" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !RECALLCAP.load(Ordering::Relaxed) {
                        RECALL_ARMED.store(0, Ordering::Relaxed);
                        RECALL_FILE_INIT.store(false, Ordering::Relaxed);
                    }
                    RECALLCAP.store(on, Ordering::Relaxed);
                }
                "candcap" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !CANDCAP.load(Ordering::Relaxed) {
                        CAND_ARMED.store(0, Ordering::Relaxed);
                        CAND_LOGGED.store(0, Ordering::Relaxed);
                        CAND_FILE_INIT.store(false, Ordering::Relaxed);
                    }
                    CANDCAP.store(on, Ordering::Relaxed);
                }
                "gbcap" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !GBCAP.load(Ordering::Relaxed) {
                        GB_ARMED.store(0, Ordering::Relaxed);
                        GB_LOGGED.store(0, Ordering::Relaxed);
                        GB_FILE_INIT.store(false, Ordering::Relaxed);
                    }
                    GBCAP.store(on, Ordering::Relaxed);
                }
                "gbbody" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !GBBODY.load(Ordering::Relaxed) {
                        GBB_ARMED.store(0, Ordering::Relaxed);
                        GBB_FILE_INIT.store(false, Ordering::Relaxed);
                        if let Ok(mut sv) = GBB_SEEN.lock() { sv.clear(); }
                    }
                    GBBODY.store(on, Ordering::Relaxed);
                }
                "gbcallee" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !GBCALLEE.load(Ordering::Relaxed) {
                        GBC_ARMED.store(0, Ordering::Relaxed);
                        GBC_LOGGED.store(0, Ordering::Relaxed);
                        GBC_FILE_INIT.store(false, Ordering::Relaxed);
                        GBC203_OK.store(0, Ordering::Relaxed); GBC203_DIFF.store(0, Ordering::Relaxed);
                        GBC690_OK.store(0, Ordering::Relaxed); GBC690_DIFF.store(0, Ordering::Relaxed);
                    }
                    GBCALLEE.store(on, Ordering::Relaxed);
                }
                "gbrd" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !GBRD.load(Ordering::Relaxed) {
                        GBRD_ARMED.store(0, Ordering::Relaxed);
                        GBRD_OK.store(0, Ordering::Relaxed); GBRD_DIFF.store(0, Ordering::Relaxed); GBRD_NP.store(0, Ordering::Relaxed);
                        GBRD_FILE_INIT.store(false, Ordering::Relaxed);
                        if let Ok(mut m) = GBRD_MAP.lock() { m.clear(); }
                        // gbrd는 kind14 리턴캡처에 의존 → gbbody seen 풀도 리셋(스로틀 신선화).
                        if let Ok(mut sv) = GBB_SEEN.lock() { sv.clear(); }
                        GBB_ARMED.store(0, Ordering::Relaxed);
                    }
                    GBRD.store(on, Ordering::Relaxed);
                }
                "gbrepl" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !GBREPL.load(Ordering::Relaxed) {
                        GBREPL_N.store(0, Ordering::Relaxed);
                        GBRD_FILE_INIT.store(false, Ordering::Relaxed);
                        GBRD_OK.store(0, Ordering::Relaxed); GBRD_DIFF.store(0, Ordering::Relaxed); GBRD_NP.store(0, Ordering::Relaxed);
                        if let Ok(mut m) = GBRD_MAP.lock() { m.clear(); }
                        GBB_ARMED.store(0, Ordering::Relaxed);
                        if let Ok(mut sv) = GBB_SEEN.lock() { sv.clear(); }
                    }
                    GBREPL.store(on, Ordering::Relaxed);
                }
                "gbdedc0" => { GBDEDC0.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }
                "gbskip" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !GBSKIP.load(Ordering::Relaxed) { GBSKIP_N.store(0, Ordering::Relaxed); }
                    GBSKIP.store(on, Ordering::Relaxed);
                }
                "gbreplchk" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !GBREPLCHK.load(Ordering::Relaxed) {
                        GBREPL_MATCH.store(0, Ordering::Relaxed); GBREPL_MISMATCH.store(0, Ordering::Relaxed);
                        GBREPLCHK_FILE_INIT.store(false, Ordering::Relaxed);
                        if let Ok(mut m) = GBRD_MAP.lock() { m.clear(); }
                    }
                    GBREPLCHK.store(on, Ordering::Relaxed);
                }
                "e9a30cap" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !E9_CAP.load(Ordering::Relaxed) {
                        E9_N.store(0, Ordering::Relaxed); E9_OK.store(0, Ordering::Relaxed);
                        E9_DIFF.store(0, Ordering::Relaxed); E9_FILE_INIT.store(false, Ordering::Relaxed);
                    }
                    E9_CAP.store(on, Ordering::Relaxed);
                }
                "e9jt" => { E9_JT.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }
                "d4ttd" => { let on=v=="1"||v.eq_ignore_ascii_case("true"); if on && !D4_TTD.load(Ordering::Relaxed) { D4_TTD_PASS.store(0,Ordering::Relaxed); D4_TTD_C8.store(0,Ordering::Relaxed); } D4_TTD.store(on, Ordering::Relaxed); }
                "d4_repl" => { D4_REPL.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // disc4 mp_repl 대체 토글(freeze 격리; 0=disc4만 passthrough)
                "perf_measure" => { let on = v=="1"||v.eq_ignore_ascii_case("true"); if on { for i in 0..8 { PERF_NS[i].store(0,Ordering::Relaxed); PERF_CNT[i].store(0,Ordering::Relaxed); } } PERF_ON.store(on, Ordering::Relaxed); }   // judge별 시간측정→perf.txt
                "fast_read" => { let lvl = v.trim().parse::<u8>().unwrap_or(if v.eq_ignore_ascii_case("true"){2}else{0}); FAST_READ.store(lvl.min(2), Ordering::Relaxed); }   // ★rd_* 읽기 경로: 0=VirtualQuery / 1=VEH spinlock / 2=VEH lockless(최속). 문제시 낮춰서 롤백
                "read_bench" => { if v=="1"||v.eq_ignore_ascii_case("true") { unsafe { bench_reads(); } } }   // ★읽기경로 직접벤치 1회 → readbench.txt (per-read ns ground-truth)
                "log" => { LOG_ON.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★진단/로그 파일출력 마스터(기본 off=배포 깨끗). 1=plan_reimpl.txt·perf.txt·*cmp.txt 등 기록
                "call_ablate" => { CALL_ABLATE.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★오더 콜(0xb) 제거 ablation: 1=콜차단(retreat_engage 2 push nop), 0=원본복원. 콜 영향 검증용
                "lane_gate" => { if let Ok(n)=v.trim().parse::<u8>() { LANE_GATE.store(n.min(2), Ordering::Relaxed); } }   // ★오더 라인후보 게이트 ablation: 0=원본/1=OFF(후보0개)/2=ALL(후보다). 매크로 영향 검증용
                "type3_ablate" => { TYPE3_ABLATE.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★오더 transition type3 콜 차단: 1=차단(jae→jmp), 0=원본. 매크로 subplan 전환 영향 검증
                "skip_untuned" => { SKIP_UNTUNED.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★튜닝 안 한 judge는 원본 native 사용(속도↑·결과동일). 일정넘김 백그라운드 가속
                // ★★ judge 튜닝 계수 (기본 engage/ttd/gb=100%, recall_bias=0). 안 적으면 게임원본=replay-identical.
                "t_engage" => { if let Ok(n) = v.parse::<i64>() { TUNE_ENGAGE_MULT.store(n, Ordering::Relaxed); } }
                "t_ttd"    => { if let Ok(n) = v.parse::<i64>() { TUNE_TTD_MULT.store(n, Ordering::Relaxed); } }
                "t_recall" => { if let Ok(n) = v.parse::<i64>() { TUNE_RECALL_BIAS.store(n, Ordering::Relaxed); } }
                "t_gb"     => { if let Ok(n) = v.parse::<i64>() { TUNE_GB_MULT.store(n, Ordering::Relaxed); } }
                "d4freeze" => { D4FREEZE.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); D4_CALLN.store(0, Ordering::Relaxed); }   // my_disc4 단계별 truncate-write(d4last.txt)
                "condcap" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !CONDCAP.load(Ordering::Relaxed) {
                        COND_ARMED.store(0, Ordering::Relaxed); COND_OK.store(0, Ordering::Relaxed);
                        COND_DIFF.store(0, Ordering::Relaxed); COND_PEND.store(0, Ordering::Relaxed);
                        COND_FILE_INIT.store(false, Ordering::Relaxed);
                        for k in 0..16 { COND_SUB_ARMED[k].store(0, Ordering::Relaxed); }
                    }
                    CONDCAP.store(on, Ordering::Relaxed);
                }
                "mpcap" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !MPCAP.load(Ordering::Relaxed) {
                        MP_ARMED.store(0, Ordering::Relaxed); MP_FILE_INIT.store(false, Ordering::Relaxed);
                        MP_OK.store(0, Ordering::Relaxed); MP_DIFF.store(0, Ordering::Relaxed); MP_PEND.store(0, Ordering::Relaxed);
                        for k in 0..16 { MP_SUB_ARMED[k].store(0, Ordering::Relaxed); }
                    }
                    MPCAP.store(on, Ordering::Relaxed);
                }
                "defwatch" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !DEFWATCH.load(Ordering::Relaxed) {
                        DEFW_ARMED.store(0, Ordering::Relaxed); DEFW_N.store(0, Ordering::Relaxed); DEFW_INIT.store(false, Ordering::Relaxed);
                    }
                    DEFWATCH.store(on, Ordering::Relaxed);
                }
                "replay_reset" => {
                    REPLAY_RESET.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed);
                }
                "seed_set" => {
                    let sv = v.trim();
                    let parsed = if let Some(h) = sv.strip_prefix("0x").or_else(|| sv.strip_prefix("0X")) {
                        u64::from_str_radix(h, 16).ok()
                    } else { sv.parse::<u64>().ok() };
                    SEED_SET.store(parsed.unwrap_or(0), Ordering::Relaxed);
                }
                "strat_set" => {
                    // "b0,..,b11;r0,..,r11" (12;12) → 고정 strat 주입. "0"/빈값 → 해제(회전/복원).
                    let sv = v.trim();
                    let parsed: Option<([u8;12],[u8;12])> = if sv == "0" || sv.is_empty() { None }
                        else if let Some((bs, rs)) = sv.split_once(';') {
                            let pb: Vec<u8> = bs.split(',').filter_map(|x| x.trim().parse::<u8>().ok()).collect();
                            let pr: Vec<u8> = rs.split(',').filter_map(|x| x.trim().parse::<u8>().ok()).collect();
                            if pb.len()==12 && pr.len()==12 { let mut b=[0u8;12]; let mut r=[0u8;12]; for i in 0..12 { b[i]=pb[i]; r[i]=pr[i]; } Some((b,r)) } else { None }
                        } else { None };
                    if let Ok(mut s) = STRAT_SET.lock() { *s = parsed; }
                }
                "seed_rotate" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if !on && SEED_ROTATE.load(Ordering::Relaxed) {
                        // on→off: 백업한 원본 시드 전부 복원(세이브 보호) 후 백업 비움
                        if let Ok(mut bak) = SEED_BAK.lock() {
                            for &(base, orig) in bak.iter() {
                                unsafe { if readable(base + O_SEED_REPLAY, 8) { std::ptr::write_unaligned((base + O_SEED_REPLAY) as *mut u64, orig); } }
                            }
                            bak.clear();
                        }
                        SEED_ROT.store(0, Ordering::Relaxed);
                    }
                    SEED_ROTATE.store(on, Ordering::Relaxed);
                }
                "strat_rotate" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if !on && STRAT_ROTATE.load(Ordering::Relaxed) {
                        // on→off: 백업한 원본 24B 전술(blue/red) 복원(세이브 보호)
                        if let Ok(mut bak) = STRAT_BAK.lock() {
                            for &(base, blue, red) in bak.iter() {
                                unsafe { if readable(base + O_RED_STRAT + 24, 1) {
                                    for i in 0..24 { std::ptr::write_unaligned((base + O_BLUE_STRAT + i) as *mut u8, blue[i]); std::ptr::write_unaligned((base + O_RED_STRAT + i) as *mut u8, red[i]); }
                                } }
                            }
                            bak.clear();
                        }
                        STRAT_ROT_N.store(0, Ordering::Relaxed);
                    }
                    STRAT_ROTATE.store(on, Ordering::Relaxed);
                }
                _ => {   // ★미지 key = 세밀 튜닝 계수 → TUNE_TABLE (10진수 또는 0x16진수 지원)
                    let parsed = match v.strip_prefix("0x").or_else(|| v.strip_prefix("0X")) {
                        Some(h) => i64::from_str_radix(h, 16),
                        None => v.parse::<i64>(),
                    };
                    if let Ok(n) = parsed { new_tune.insert(k.to_string(), n); }
                }
            }
        }
    }
    tune_publish(new_tune);   // ★lock-free: 누적 테이블 일괄 게시(judge tune() 읽기 lock 제거)
    // ★skip_untuned: 튜닝 안 한 judge의 대체를 끔 → 원본 native 사용(결과 100% 동일·속도↑). 일정넘김 백그라운드 N경기 가속.
    //   판정 = default.txt(기준값) 대비 활성값 다름(=튜닝됨). condgate=계수없음→항상 끔. vis_window(CAND_FILTER 시야창)는 광범위→튜닝시 관련 judge 보존(보수적).
    if SKIP_UNTUNED.load(Ordering::Relaxed) {
        COND_REPL.store(false, Ordering::Relaxed);
        if let Some(base) = read_baseline() {
            let g = |keys: &[&str]| keys.iter().any(|&k| match base.get(k) { Some(&b) => tune(k, b) != b, None => false });
            let vis_t = g(&["vis_window"]);
            let engage_t = vis_t || NUMBERS_MARGIN.load(Ordering::Relaxed) > 0 || TOWER_THREAT.load(Ordering::Relaxed) > 0 || g(&["t_engage","eng_role4","eng_role3","eng_role2","eng_role_def","engage_base","engage_thr_mult"]);   // numbers/tower>0도 engage 대체 유지(override 동작 위함)
            let disc4_t  = vis_t || TOWER_THREAT.load(Ordering::Relaxed) > 0 || g(&["t_ttd","d4_dmg_scale","d4_div_base","d4_coef_scale","d4_coef_min","d4_coef_clamp","d4_coord_dist","d4_ttd_scale","tower_dps"]);   // ★포탑위협>0도 disc4 대체 유지(TTD 가산 위함)
            let recall_t = vis_t || g(&["t_recall","rc_u21_init","rc_ehp_t1","rc_ehp_t2","rc_ehp_t3","rc_ehp_v1","rc_ehp_v2","rc_norp_bonus","rc_ed_near","rc_ed_mid","rc_ed_far","rc_ed_near_pen","rc_ed_far_bonus","rc_ed_vfar_bonus","rc_ahp_t1","rc_ahp_t2","rc_u13_bonus","rc_ahp2_pen","rc_ad_near","rc_ad_mid","rc_ad_near_bonus","rc_ad_far_pen","rc_mult_bonus","rc_ally_hp_min"]);
            let gb_t     = vis_t || g(&["t_gb","gb_rbx_div","gb_r15_div","gb_r14_num"]);
            let dd7_t    = vis_t || TOWER_THREAT.load(Ordering::Relaxed) > 0 || NUMBERS_MARGIN.load(Ordering::Relaxed) > 0 || NUMBERS_THREAT.load(Ordering::Relaxed) > 0 || g(&["dd_frontier_mult","dd_lane_margin","dd_cover_count","dd_ratio_thr","dd_facet_thr","dd_near_dist","dd_main_near_dist","dd_gatee_dist","dd_ivar2_thr","dd_n_thr","dd_survivor_thr"]);   // ★포탑/인원수>0도 dd7_repl 유지(라이너 후퇴 override 위함)
            let poke_t   = vis_t || g(&["pf_edge_margin","pf_center_band","pf_diag_far","pf_diag_near","pf_band_width","pk_home_lo","pk_home_hi","pk_home_x1","pk_home_y1","pk_hp_main","pk_hp_retreat","pk_smallact_split","pk_threat_mult","pk_zone_hp","pk_engage_dist","pk_obj_hp"]);
            let mp_misc_t = g(&["d8_slot_thr","dn_lane_margin","dn_pred_dist","dn_near_dist","dn_home_lo","dn_home_hi","dn_home_x1","dn_home_y1","dn_hp_crit","dn_hp_low","dn_count_gate","dn_nexus_hp","bt_home_lo","bt_home_hi","bt_home_x1","bt_home_y1","bt_hp_retreat"]);
            if !engage_t { ENGAGE_REPL.store(false, Ordering::Relaxed); }
            if !disc4_t  { D4_REPL.store(false, Ordering::Relaxed); }
            if !recall_t { RECALL_REPL.store(false, Ordering::Relaxed); }
            if !gb_t     { GBSKIP.store(false, Ordering::Relaxed); }
            if !dd7_t    { DD7_REPL.store(false, Ordering::Relaxed); }
            if !poke_t   { POKE_REPL.store(false, Ordering::Relaxed); }
            if !(dd7_t || poke_t || disc4_t || mp_misc_t) { MP_REPL.store(false, Ordering::Relaxed); }   // 이동판단 하위 전부 untuned면 통째 원본
        }
    }
    // ★engage 레버(0.4.13 재RE·검증완료): engage_thr_mult(ROLE_THR 4 imm)/engage_base(ENGAGE_GATE 83 C0 imm8) 둘 다 sanity보호 정적패치 → MIG_CHANGED 무관 적용. retreat replace detour(아래 3722)는 프레임시프트 의존이라 별도 보류 유지.
    unsafe { apply_engage_base(); apply_engage_thr_mult(); }
    true
}

// ── dispatch 훅: 초경량 — raw 레지스터/포인터만 저장. 무거운 작업 금지(핫패스). ──
// saved 레이아웃: [0]=r11 [8]=r10 [0x10]=r9 [0x18]=r8 [0x20]=rdx [0x28]=rcx
unsafe extern "C" fn dispatch_capture(saved: usize, _entry_rsp: usize) {
    let r9  = rd_u64(saved+0x10).unwrap_or(0) as usize;
    let r8  = rd_u64(saved+0x18).unwrap_or(0) as usize;
    let rdx = rd_u64(saved+0x20).unwrap_or(0) as usize;
    let rcx = rd_u64(saved+0x28).unwrap_or(0) as usize;
    if rdx < 0x10500 { return; }
    let ps = rdx - 0x500;
    CAP_R9.store(r9, Ordering::Relaxed);
    CAP_R8.store(r8, Ordering::Relaxed);
    CAP_RDX.store(rdx, Ordering::Relaxed);
    CAP_RCX.store(rcx, Ordering::Relaxed);
    CAP_PSTATE.store(ps, Ordering::Relaxed);
    DISP_TID.store(GetCurrentThreadId() as u64, Ordering::Relaxed);
    DISP_HITS.fetch_add(1, Ordering::Relaxed);
    // distinct plan_state set 채우기 (16개 차면 중단)
    if PSTATE_CNT.load(Ordering::Relaxed) < 16 {
        let mut empty = usize::MAX;
        let mut found = false;
        for i in 0..16usize {
            let v = PSTATE_SET[i].load(Ordering::Relaxed);
            if v == ps { found = true; break; }
            if v == 0 && empty == usize::MAX { empty = i; }
        }
        if !found && empty != usize::MAX {
            PSTATE_SET[empty].store(ps, Ordering::Relaxed);
            PSTATE_CNT.fetch_add(1, Ordering::Relaxed);
        }
    }
}

// ── my_ttd 재구현 헬퍼 ──
type G2 = unsafe extern "C" fn(usize, usize) -> i64;
type G3 = unsafe extern "C" fn(usize, usize, usize) -> i64;
fn isqrt(n: u64) -> u64 {
    if n < 2 { return n; }
    let mut x = n; let mut y = (x + 1) / 2;
    while y < x { x = y; y = (x + n / x) / 2; }
    x
}
unsafe fn vt560_threat(e: usize) -> i64 {
    let v = rd_u64(e + 0x560).unwrap_or(0) as usize;
    let g = rd_u64(v + 0x90).unwrap_or(0) as usize;
    let a0 = rd_u64(e + 0x558).unwrap_or(0) as usize;
    if !ptr_ok(g) { return 0; }
    let f: G2 = core::mem::transmute(g); f(a0, e)
}
unsafe fn vt480_reach(e: usize, ref_e: usize) -> i64 {
    let v = rd_u64(e + 0x480).unwrap_or(0) as usize;
    if !ptr_ok(v) { return 0; }
    let g = rd_u64(v + 0x90).unwrap_or(0) as usize;
    let inner = rd_u64(v + 0x10).unwrap_or(0) as usize;
    let buf = rd_u64(e + 0x478).unwrap_or(0) as usize;
    let aligned = (inner.wrapping_sub(1) & !0xf).wrapping_add(buf).wrapping_add(0x10);
    if !ptr_ok(g) || !ptr_ok(aligned) { return 0; }
    let f: G3 = core::mem::transmute(g); f(aligned, e, ref_e)
}

// ── FUN_141f2a7d0 재구현: cand_list 적들의 base (ttr,threat) entry 빌더 ──
// 위협=미리계산 테이블(cont+0x230) 조회, 사거리/ttr=실시간(3변형). 적마다 3 entry emit.
// ★DEFAULT_AB2 churn제거(2026-06-19): 옛 게임 .rdata 주소(0x1435d83d0, was 0x35be780)는 매버전 이동.
//   라이브 의미 = "능력2 없음" 빈 디스크립터 = (+0x00~0x28 전부 0, +0x30 i32=-1). +0x38부터 exe포인터가 있으나
//   라이브 코드(my_f80320 skip82 / reach_ab2)는 +0x30 먼저읽고 -1이면 즉시 중단 → 절대 안 읽음.
//   ⟹ 동등 const static로 대체. 게임 .rdata 주소참조 0 = churn 소멸. (검증: scan_data exe덤프 +0x30=0xffffffff)
#[repr(C, align(8))]
struct DefAb2([u32; 16]);   // 0x40바이트 (idx12 = +0x30)
static DEFAULT_AB2_EMPTY: DefAb2 = DefAb2([0,0,0,0, 0,0,0,0, 0,0,0,0, 0xFFFF_FFFF,0,0,0]);
#[inline] fn default_ab2_ptr() -> usize { DEFAULT_AB2_EMPTY.0.as_ptr() as usize }
// 로스터-base FUN_141f56160(=옛 0x1bbc080)의 ab2 reach 폴백 = `if enemy+0x5b0<3: &DEFAULT_AB2`. reach_ab2 패턴 정확일치.
// ★3차 TTD(0x1e1c7c0) 데미지경로 변경: 옛 combat(0x1be1e90) 이중호출(flag0/1) → FUN_141f5db30 단일호출로 리팩터.
// f5db30=combat과 데미지공식 동일(0xd0/0xe0/0xf0 타입스케일+0xa8/0xb0 방어경감), 두 항을 내부합산+첫base는 스킬데이터서.
// 콜사이트(0x1e1c8d6): f5db30(rcx=e+0x478, rdx=sim(local_80), r8=e, r9=&0x143599b30 능력테이블, [rsp+0x20]=target).
// ⇒ f5db30 직접호출(순수함수, 옛 combat 직접호출과 동일방식)로 stale ATK_VT/TGT_VT 우회.
const RVA_F5DB30: usize = 0x1f5db30;        // ⚠0.4.13_5 미해결(F80320이 더이상 호출안함, 데미지 아키텍처 변경. 메모이즈래퍼 new 0x18960a0 디컴파일로 확정요). 현재 미사용
const RVA_ABILITY_TABLE: usize = 0x3599b30; // ⚠0.4.13_5 미해결([0]=함수 유일매핑실패). MIG_TTD off라 미사용. (f5db30 param_4)
type F5db30Fn = unsafe extern "C" fn(usize, usize, usize, usize, usize) -> i64;
// 한 공격유형의 사거리: enemy.420 + off_a + (enemy.5b0-1)*off_b + getter(vt+0x90) + ref/enemy(458/668)항
unsafe fn reach_variant(enemy: usize, ref_e: usize, off_a: usize, off_b: usize, vt_off: usize, buf_off: usize) -> i64 {
    let vt = rd_u64(enemy + vt_off).unwrap_or(0) as usize;
    let lvar8 = if ptr_ok(vt) {
        let inner = rd_u64(vt + 0x10).unwrap_or(0) as usize;
        let buf = rd_u64(enemy + buf_off).unwrap_or(0) as usize;
        let aligned = (inner.wrapping_sub(1) & !0xf).wrapping_add(buf).wrapping_add(0x10);
        let g = rd_u64(vt + 0x90).unwrap_or(0) as usize;
        if readable(g, 4) && readable(aligned, 8) { let f: G3 = core::mem::transmute(g); f(aligned, enemy, ref_e) } else { 0 }
    } else { 0 };
    let lvar18 = rd_i64(enemy + off_a).unwrap_or(0);
    let lvar23 = rd_i64(enemy + off_b).unwrap_or(0);
    let lvar5 = rd_i64(enemy + 0x5b0).unwrap_or(0);
    let lvar10 = rd_i64(enemy + 0x420).unwrap_or(0);
    let ref_term = (rd_i32(ref_e + 0x458).unwrap_or(0) as i64 + 100) * rd_i64(ref_e + 0x668).unwrap_or(0) / 100;
    let en_term  = (rd_i32(enemy + 0x458).unwrap_or(0) as i64 + 100) * rd_i64(enemy + 0x668).unwrap_or(0) / 100;
    lvar10 + lvar18 + (lvar5 - 1) * lvar23 + lvar8 + ref_term + en_term
}
// 능력2 사거리(plVar9 디스크립터 기반): enemy.420 + plv[2] + plv[3]*(5b0-1) + getter + 항
unsafe fn reach_ab2(enemy: usize, ref_e: usize, exe: usize) -> i64 {
    let _ = exe;   // DEFAULT_AB2 const화 후 exe 미사용(시그니처는 호출부 호환 위해 유지)
    let lvar5b0 = rd_i64(enemy + 0x5b0).unwrap_or(0);
    let plv = if lvar5b0 >= 3 { enemy + 0x4e8 } else { default_ab2_ptr() };
    if rd_i32(plv + 0x30).unwrap_or(-1) == -1 { return 0; }  // plVar9[6] flag == -1 → 0
    let buf = rd_u64(plv).unwrap_or(0) as usize;
    let vt  = rd_u64(plv + 8).unwrap_or(0) as usize;
    let lvar10 = if ptr_ok(vt) {
        let inner = rd_u64(vt + 0x10).unwrap_or(0) as usize;
        let aligned = buf.wrapping_add(inner.wrapping_sub(1) & !0xf).wrapping_add(0x10);
        let g = rd_u64(vt + 0x90).unwrap_or(0) as usize;
        if readable(g, 4) && readable(aligned, 8) { let f: G3 = core::mem::transmute(g); f(aligned, enemy, ref_e) } else { 0 }
    } else { 0 };
    let lvar18 = rd_i64(plv + 0x10).unwrap_or(0);
    let lvar23 = rd_i64(plv + 0x18).unwrap_or(0);
    let en420 = rd_i64(enemy + 0x420).unwrap_or(0);
    let ref_term = (rd_i32(ref_e + 0x458).unwrap_or(0) as i64 + 100) * rd_i64(ref_e + 0x668).unwrap_or(0) / 100;
    let en_term  = (rd_i32(enemy + 0x458).unwrap_or(0) as i64 + 100) * rd_i64(enemy + 0x668).unwrap_or(0) / 100;
    en420 + lvar18 + lvar23 * (lvar5b0 - 1) + lvar10 + en_term + ref_term
}
// 상태효과 ttr보너스 uVar26: enemy+0x2b0(배열ptr)/+0x2b8(개수), stride 0x28.
// type∈{2,3,4,5} 스킵, 나머지는 val=(type==10?+0x20:+0x8) max 누적. 유효없으면 0.
// (행동방해 효과 잔여시간 = 적 도달 지연 → 모든 ttr 변형에 가산)
unsafe fn status_bonus(enemy: usize) -> i64 {
    let cnt = rd_i64(enemy + 0x2b8).unwrap_or(0);
    if cnt <= 0 { return 0; }
    let arr = rd_u64(enemy + 0x2b0).unwrap_or(0) as usize;
    if !ptr_ok(arr) { return 0; }
    let cnt = cnt.min(64) as usize;  // 안전 바운드
    let mut found = false;
    let mut maxv = 0i64;
    for i in 0..cnt {
        let e = arr + i*0x28;
        if !readable(e, 0x28) { break; }
        let ty = rd_i32(e).unwrap_or(0);
        if (2..=5).contains(&ty) { continue; }   // type 2~5 스킵
        let val = if ty == 10 { rd_i64(e + 0x20).unwrap_or(0) } else { rd_i64(e + 0x8).unwrap_or(0) };
        if !found { maxv = val; found = true; } else if val > maxv { maxv = val; }
    }
    if found { maxv } else { 0 }
}
// cand_list 적들의 base entry 빌더. enemy=roster[1-team][cand_slot], ref=roster[team][ref_slot].
unsafe fn my_f2a7d0_base(cont: usize, team: usize, ref_slot: usize, ref_e: usize, p4: usize, p6: usize, exe: usize) -> Vec<(i64, i64)> {
    let mut out = Vec::new();
    let et = 1 - team;
    let cand_start = rd_u64(p4).unwrap_or(0) as usize;
    let cand_cnt = rd_u64(p4 + 0x18).unwrap_or(0) as usize;
    if !ptr_ok(cand_start) || cand_cnt == 0 || cand_cnt > 16 { return out; }
    let p6deref = if ptr_ok(p6) { rd_i64(p6).unwrap_or(1) } else { 1 };
    let ref_sp = rd_i64(ref_e + 0x628).unwrap_or(0);
    for i in 0..cand_cnt {
        let eslot = rd_i64(cand_start + i*8).unwrap_or(-1);
        if eslot < 0 || eslot > 4 { continue; }
        let es = eslot as usize;
        let enemy = rd_u64(cont + 0x1e0 + et*0x28 + es*8).unwrap_or(0) as usize;
        if !ptr_ok(enemy) { continue; }
        // 위협 테이블: cont+0x230 + enemy_slot*800 + enemy_team*4000, +{0x190,0x1b8,0x1e0}+ref_slot*8
        let tbase = cont + 0x230 + es*800 + et*4000;
        let thr_basic = rd_i64(tbase + 0x190 + ref_slot*8).unwrap_or(0);
        let thr_ab1   = rd_i64(tbase + 0x1b8 + ref_slot*8).unwrap_or(0);
        let thr_ab2   = rd_i64(tbase + 0x1e0 + ref_slot*8).unwrap_or(0);
        // 사거리 (죽은적: basic=0)
        let alive = rd_i32(enemy + 0x4a8).unwrap_or(-1) != -1;
        let r_basic = if alive { reach_variant(enemy, ref_e, 0x488, 0x490, 0x480, 0x478) } else { 0 };
        let r_ab1 = if rd_i32(enemy + 0x4e0).unwrap_or(-1) != -1 {
            reach_variant(enemy, ref_e, 0x4c0, 0x4c8, 0x4b8, 0x4b0)
        } else { 0 };
        let r_ab2 = reach_ab2(enemy, ref_e, exe);
        // 거리
        let dx = (rd_i64(enemy + 0x648).unwrap_or(0) - rd_i64(ref_e + 0x648).unwrap_or(0)).abs();
        let dy = (rd_i64(enemy + 0x650).unwrap_or(0) - rd_i64(ref_e + 0x650).unwrap_or(0)).abs();
        let dist = isqrt((dx*dx + dy*dy) as u64) as i64;
        // 이속 (param_6 deref==0 → 상대이속, 아니면 절대)
        let mut sp = if p6deref == 0 { (rd_i64(enemy + 0x628).unwrap_or(0) - ref_sp).max(0) } else { rd_i64(enemy + 0x628).unwrap_or(0) };
        sp += (sp == 0) as i64;
        // 상태효과 ttr 보너스 uVar26 (적 행동방해 효과 잔여시간)
        let bonus = status_bonus(enemy);
        let ttr_basic = (dist - r_basic).max(0).saturating_add(sp - 1) / sp + bonus;
        let ttr_ab1   = (dist - r_ab1).max(0).saturating_add(sp - 1) / sp + bonus;
        let ttr_ab2   = (dist - r_ab2).max(0).saturating_add(sp - 1) / sp + bonus;
        out.push((ttr_basic, thr_basic));
        out.push((ttr_ab1, thr_ab1));
        out.push((ttr_ab2, thr_ab2));
    }
    out
}

// 적/공격자 e 한 명의 (ttr, threat) 점수. with_reach=false → ttr=0 (param_5 attacker).
unsafe fn ttd_score_one(e: usize, ref_e: usize, local_80: usize, threat_scalar: i64, with_reach: bool, exe: usize) -> (i64, i64) {
    // ★풀재현(2026-06-18, 우리 손으로 전부): f5db30 게임호출 제거 → my_combat_dmg 2항(공식+계수시트+유효스탯 전부 Rust 직접).
    // f5db30 prologue 확정: 스킬 base게터([skill_vt+0x28])가 (rax,rdx)=base1,base2 2값반환 → term1(flag0)+term2(flag1).
    //   (decompile "param_2"=게터 rdx반환=base2, 함수인자 아님; 함수 param_2=sim은 미사용.) = 옛 combat 이중호출 경로와 동일식.
    let (pb, mb) = probe_basedmg(e, local_80, exe);   // 스킬 base 2값(rax,rdx). 챔프별 순수게터=섀도우호출(불가피 경계).
    let dtype = rd_i32(e + 0x4a4).unwrap_or(0) as u32;
    let dmg = my_combat_dmg(e, ref_e, pb, dtype, 0, exe) + my_combat_dmg(e, ref_e, mb, dtype, 1, exe);
    let _ = (RVA_F5DB30, RVA_ABILITY_TABLE, RVA_COMBAT_FN);  // f5db30 직접호출 제거(미사용 경고억제)
    let coef = vt560_threat(e);
    let denom = (rd_i32(e + 0x3e4).unwrap_or(0) as i64 + 100).max(1);
    let rate = (coef * 100 / denom).max(3);
    let threat = threat_scalar * dmg / rate;
    if !with_reach { return (0, threat); }
    let reach = rd_i64(e + 0x420).unwrap_or(0)
        + rd_i64(e + 0x488).unwrap_or(0)
        + vt480_reach(e, ref_e)
        + (rd_i64(e + 0x5b0).unwrap_or(0) - 1) * rd_i64(e + 0x490).unwrap_or(0)
        + (rd_i32(e + 0x458).unwrap_or(0) as i64 + 100) * rd_i64(e + 0x668).unwrap_or(0) / 100
        + (rd_i32(ref_e + 0x458).unwrap_or(0) as i64 + 100) * rd_i64(ref_e + 0x668).unwrap_or(0) / 100;
    let dx = (rd_i64(e + 0x648).unwrap_or(0) - rd_i64(ref_e + 0x648).unwrap_or(0)).abs();
    let dy = (rd_i64(e + 0x650).unwrap_or(0) - rd_i64(ref_e + 0x650).unwrap_or(0)).abs();
    let dist = isqrt((dx * dx + dy * dy) as u64) as i64;
    let sp = { let s = rd_i64(e + 0x628).unwrap_or(0); if s == 0 { 1 } else { s } };
    let ttr = (dist - reach).max(0) / sp;
    (ttr, threat)
}
// TTD 재구현 (스냅샷 기반). base=FUN_141f2a7d0 로스터 적 entry, enemies=cont+0xf0 교전중 적.
unsafe fn my_ttd_snap(ref_e: usize, local_80: usize, threat_scalar: i64, base: &[(i64,i64)], enemies: &[usize], attacker: usize, exe: usize) -> i64 {
    if !ptr_ok(ref_e) { return -2; }
    let mut list: Vec<(i64, i64)> = Vec::new();
    // ① FUN_141f2a7d0 base entry들 (로스터 적, 위협=테이블)
    list.extend_from_slice(base);
    // ② param_5 옵션 공격자 (ttr=0)
    if attacker != 0 && rd_i32(attacker + 0x4a8).unwrap_or(-1) != -1 {
        list.push(ttd_score_one(attacker, ref_e, local_80, threat_scalar, false, exe));
    }
    // ③ cont+0xf0 교전중 적 (위협=combat_effective_damage)
    for &en in enemies {
        if ptr_ok(en) && readable(en, 0x740) && rd_i32(en + 0x4a8).unwrap_or(-1) != -1 {
            list.push(ttd_score_one(en, ref_e, local_80, threat_scalar, true, exe));
        }
    }
    list.push((9999999999, 0));
    list.sort_by_key(|&(t, _)| t);
    let budget = rd_i64(ref_e + 0x658).unwrap_or(0) * 100;
    let (mut acc_t, mut acc_thr, mut acc_dmg) = (0i64, 0i64, 0i64);
    let mut last_t = 0i64;
    for &(t, thr) in &list {
        let seg_plus = (t - acc_t) * acc_thr * 100 / 60 + acc_dmg;
        if budget <= seg_plus {
            return (budget - acc_dmg) / acc_thr.max(1) + acc_t;
        }
        acc_thr += thr; acc_t = t; acc_dmg = seg_plus; last_t = t;
    }
    if last_t != 0 { last_t } else { 9999999999 }
}

// ── TTD 리턴-훅 진입 캡처: 입력 스냅샷 → my_ttd 계산 → 리턴주소를 thunk로 스왑 + 프레임 push. ──
// 재호출(g) 제거. 실제 게임 반환값은 함수가 ret할 때 thunk(ttd_return)가 rax로 캡처.
// 적집합 = +0xf0 동적 "교전중 적" 리스트만 (로스터 폴백 제거). 빈 리스트 → my_ttd=9999999999.
unsafe extern "C" fn ttd_capture(saved: usize, entry_rsp: usize) {
    let thunk = RET_THUNK.load(Ordering::Relaxed);
    if thunk == 0 || !CAP_ON.load(Ordering::Relaxed) { return; }  // cfg capture=1일 때만 무장
    // 무장 상한: 실교전 샘플 목표치 달성 or 총무장 상한 → 무장 중단(정상 통과).
    if TTD_NONEMPTY.load(Ordering::Relaxed) >= TTD_NONEMPTY_MAX
        || TTD_ARMED.load(Ordering::Relaxed) >= TTD_ARM_MAX { return; }
    let p1 = rd_u64(saved+0x28).unwrap_or(0) as usize; // rcx container
    let p2 = rd_u64(saved+0x20).unwrap_or(0) as usize; // rdx team
    let p3 = rd_u64(saved+0x18).unwrap_or(0) as usize; // r8 slot
    let p4 = rd_u64(saved+0x10).unwrap_or(0) as usize; // r9 cand_list
    let p5 = rd_u64(entry_rsp+0x28).unwrap_or(0) as usize; // attacker(param_5)
    let p6 = rd_u64(entry_rsp+0x30).unwrap_or(0) as usize; // param_6 (이속모드 플래그ptr)
    if !ptr_ok(p1) || p2 > 1 || p3 > 4 { return; }
    let exe = exe_base();
    if exe == 0 { return; }
    let cont = rd_u64(p1).unwrap_or(0) as usize;
    if !ptr_ok(cont) { return; }
    // 입력 스냅샷
    let ref_e = rd_u64(cont + 0x1e0 + p2*0x28 + p3*8).unwrap_or(0) as usize;
    if !ptr_ok(ref_e) { return; }
    let local_80 = rd_u64(p1 + 8).unwrap_or(0) as usize;
    let c1 = rd_u64(local_80 + 8).unwrap_or(0) as usize;
    let scalar = rd_i64(c1 + 0x12f8).unwrap_or(0);
    let et = 1 - p2;
    let f0start = rd_u64(cont + 0xf0 + et*0x20).unwrap_or(0) as usize;
    let f0cnt = rd_u64(cont + 0x108 + et*0x20).unwrap_or(0) as usize;
    // 적집합 = +0xf0 리스트만
    let mut enemies: Vec<usize> = Vec::new();
    if ptr_ok(f0start) && f0cnt > 0 && f0cnt <= 32 {
        for i in 0..f0cnt { let en = rd_u64(f0start + i*8).unwrap_or(0) as usize; if ptr_ok(en) { enemies.push(en); } }
    }
    let nonempty = !enemies.is_empty();
    // ★ base = FUN_141f2a7d0 로스터 적 entry (cand_list 기반). 이게 빈-적 케이스 유한 TTD의 원천.
    let base = my_f2a7d0_base(cont, p2, p3, ref_e, p4, p6, exe);
    // my_ttd 계산 (스냅샷). getter/combat/테이블은 순수 읽기 = 안전.
    let mine = my_ttd_snap(ref_e, local_80, scalar, &base, &enemies, p5, exe);
    // 로그 프리픽스 (반환시 game/mine verdict 덧붙임)
    let n = TTD_ARMED.fetch_add(1, Ordering::Relaxed);
    if nonempty { TTD_NONEMPTY.fetch_add(1, Ordering::Relaxed); }
    let refname = cstr(rd_u64(ref_e + E_NAME).unwrap_or(0) as usize);
    let mut enames = String::new();
    for &en in &enemies { enames.push_str(&format!("{}(av{}) ", cstr(rd_u64(en+E_NAME).unwrap_or(0) as usize), rd_i32(en+0x4a8).unwrap_or(-9))); }
    // ── 진단(첫 12회): base(FUN_141f2a7d0) + p5 경로 분해 ──
    let mut dbg = String::new();
    if n < 12 {
        let cand_start = rd_u64(p4).unwrap_or(0) as usize;
        let cand_cnt = rd_u64(p4 + 0x18).unwrap_or(0) as usize;
        dbg.push_str(&format!("\n   cand: start=0x{:x} cnt={} slots=[", cand_start, cand_cnt));
        if ptr_ok(cand_start) && cand_cnt <= 16 { for i in 0..cand_cnt { dbg.push_str(&format!("{} ", rd_i64(cand_start+i*8).unwrap_or(-1))); } }
        dbg.push_str(&format!("] p6=0x{:x} *p6={} refhp={} refsp={}",
            p6, if ptr_ok(p6) { rd_i64(p6).unwrap_or(-999) } else { -999 },
            rd_i64(ref_e + 0x658).unwrap_or(-1), rd_i64(ref_e + 0x628).unwrap_or(-1)));
        dbg.push_str(&format!("\n   base {} entries:", base.len()));
        for (k, &(t, thr)) in base.iter().enumerate().take(9) { dbg.push_str(&format!(" [{}](ttr={},thr={})", k, t, thr)); }
        let p5alive = if p5 != 0 { rd_i32(p5 + 0x4a8).unwrap_or(-1) } else { -999 };
        dbg.push_str(&format!("\n   p5=0x{:x} p5alive={} scalar={}", p5, p5alive, scalar));
        if p5 != 0 && p5alive != -1 {
            let (pb, mb) = probe_basedmg(p5, local_80, exe);
            let dtype = rd_i32(p5 + 0x4a4).unwrap_or(0) as u32;
            let combat: CombatFn = core::mem::transmute(exe + RVA_COMBAT_FN);
            let avt = exe + RVA_ATK_VT; let tvt = exe + RVA_TGT_VT;
            let d0 = combat(p5, avt, ref_e, tvt, pb, dtype, 0);
            let d1 = combat(p5, avt, ref_e, tvt, mb, dtype, 1);
            let coef = vt560_threat(p5);
            let denom = (rd_i32(p5 + 0x3e4).unwrap_or(0) as i64 + 100).max(1);
            let rate = (coef * 100 / denom).max(3);
            let threat = scalar * (d0 + d1) / rate;
            dbg.push_str(&format!("\n   ATK: pb={} mb={} dtype={} dmg={}(+{}) coef={} 3e4={} rate={} → threat={}",
                pb, mb, dtype, d0, d1, coef, rd_i32(p5 + 0x3e4).unwrap_or(-1), rate, threat));
        }
    }
    let pre = format!("[ttd #{} {}] team={} slot={} ref={} 적{}=[{}]{} → ",
        n, if nonempty {"ENGAGE"} else {"empty"}, p2, p3, refname, enemies.len(), enames, dbg);
    // ★ 프레임 push (먼저!) → 리턴주소 스왑. push 성공해야만 스왑(thunk 도달시 프레임 보장).
    let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    if !ptr_ok(orig_ret) { return; }
    let pushed = if let Ok(mut st) = RET_STACK.lock() {
        if st.len() < 64 { st.push(RetFrame { key: entry_rsp, orig_ret, mine, kind: 0, pre, p5: 0, p6: 0, disp_pred: -99 }); true } else { false }
    } else { false };
    if !pushed { return; }
    // 리턴주소 슬롯(*entry_rsp)을 thunk로 교체 → 함수 ret시 thunk 진입.
    if readable(entry_rsp, 8) {
        core::ptr::write_unaligned(entry_rsp as *mut usize, thunk);
    } else {
        // 스왑 실패 → 방금 push한 프레임 제거(고아 방지)
        if let Ok(mut st) = RET_STACK.lock() { if let Some(p) = st.iter().rposition(|f| f.key == entry_rsp) { st.remove(p); } }
    }
}

// ── FUN_141dd9360 훅: 초경량 — rdx(AI구조체), r9(athlete) 저장 ──
unsafe extern "C" fn t9360_capture(saved: usize, _entry_rsp: usize) {
    let r9  = rd_u64(saved+0x10).unwrap_or(0) as usize; // athlete
    let rdx = rd_u64(saved+0x20).unwrap_or(0) as usize; // AI구조체
    if ptr_ok(rdx) { CAP_T9_RDX.store(rdx, Ordering::Relaxed); }
    if ptr_ok(r9)  { CAP_T9_R9.store(r9, Ordering::Relaxed); }
    if !ptr_ok(r9) || !ptr_ok(rdx) { return; }
    // 스로틀: 매 틱·매 플레이어 갱신은 과해 (subplan 천천히 바뀜) → 1/8만
    if T9_CTR.fetch_add(1, Ordering::Relaxed) & 7 != 0 { return; }
    // name→subplan 맵 갱신. athlete name = Rust &str/String (ptr@+0x398, len@+0x3a0 또는 +0x3a8)
    let np = rd_u64(r9 + A_NAME).unwrap_or(0) as usize;
    let la = rd_u64(r9 + 0x3a0).unwrap_or(0) as usize;
    let lb = rd_u64(r9 + 0x3a8).unwrap_or(0) as usize;
    let want = if (1..=24).contains(&la) { la } else if (1..=24).contains(&lb) { lb } else { 0 };
    let mut buf = [0u8; 24];
    let mut nlen = 0usize;
    if want > 0 && ptr_ok(np) && readable(np, want) {  // 한 번의 readable + 일괄 읽기
        for k in 0..want { buf[k] = std::ptr::read_unaligned((np+k) as *const u8); }
        nlen = want;
    } else {
        nlen = read_name_at(np, &mut buf); // 폴백: null 종료
    }
    if nlen == 0 { return; }
    let team = rd_i64(r9 + A_TEAM).unwrap_or(-1);
    let flag = rd_i64(rdx + AI_FLAG).unwrap_or(0);
    let sp = if flag == -1 { rd_i64(rdx + AI_SUB_A).unwrap_or(-99) } else { rd_i64(rdx + AI_SUB_B).unwrap_or(-99) };
    if (0..=14).contains(&sp) { submap_set(team, &buf[..nlen], sp); }
}

// ── plan_think_driver 훅: 초경량 — per-player 인자만 저장 (entity/plan_state 식별용) ──
unsafe extern "C" fn driver_capture(saved: usize, entry_rsp: usize) {
    let r9  = rd_u64(saved+0x10).unwrap_or(0) as usize;
    let r8  = rd_u64(saved+0x18).unwrap_or(0) as usize;
    let rdx = rd_u64(saved+0x20).unwrap_or(0) as usize;
    let rcx = rd_u64(saved+0x28).unwrap_or(0) as usize;
    let a5  = rd_u64(entry_rsp+0x28).unwrap_or(0) as usize;
    CAP_DRV_RCX.store(rcx, Ordering::Relaxed);
    CAP_DRV_RDX.store(rdx, Ordering::Relaxed);
    CAP_DRV_R8.store(r8, Ordering::Relaxed);
    CAP_DRV_R9.store(r9, Ordering::Relaxed);
    CAP_DRV_A5.store(a5, Ordering::Relaxed);
}

// ★engage RNG footprint 측정+예측검증: retreat 진입 (entry_rsp, state, idx0, ctr0, pred_out, pred_words) 스냅 → kind1 리턴서 실제 (out, words)와 대조. engfoot.txt.
//   pred_out/pred_words = my_engage_predict (engage 브랜치만; 비engage는 -777=skip).
static RE_SNAP: Mutex<Vec<(usize,usize,u64,u64,i64,i64,i64,i64)>> = Mutex::new(Vec::new());  // +count_a,count_b(진단)
static EFOOT_INIT: AtomicBool = AtomicBool::new(false);
static EFOOT_N: AtomicU64 = AtomicU64::new(0);
static EP_OK: AtomicU64 = AtomicU64::new(0);    // engage 예측 (out+words) 일치
static EP_DIFF: AtomicU64 = AtomicU64::new(0);
// ── facet#5 RE 하네스: retreat_engage 출력(*param_1: 5=교전/-1=퇴각) + param_7(임계값) 캡처 ──
static RE_ARMED: AtomicU64 = AtomicU64::new(0);   // 총 무장(전 경기 동안; 의미있는것만 로깅)
static RE_LOGGED: AtomicU64 = AtomicU64::new(0);  // roll/retreat 샘플 로깅수 → recmp.txt
static RE_PANIC: AtomicU64 = AtomicU64::new(0);    // ★capture 경로 패닉 차단수(catch_unwind) → recmp.txt 진단
static HR_PANIC: AtomicU64 = AtomicU64::new(0);    // ★hook_return 패닉 차단수
// ★dispatch 예측 게이트(cfg disppred, 기본 OFF). 0.4.13_5 리팩터 retreat의 dispatch(3/7/8) 예측 블록.
//   shadow_fa1ea0=my_fa1ea0(순수, guarded) → 게임콜 無·세그폴트 위험 없음. disppred=1로 STAND(8) 예측을
//   DISP-OK/DIFF로 end-to-end 검증 가능. (fa1ea0 직접대조 fa1cmp는 288/288 DIFF0 검증완료 후 제거됨.)
static DISPPRED: AtomicBool = AtomicBool::new(false);
static DISP_LOGGED: AtomicU64 = AtomicU64::new(0);  // 디스패치(3/7/8) 로깅수 → dispcmp.txt
static DISP_OK: AtomicU64 = AtomicU64::new(0);      // my_dispatch_code == 실제 out 횟수
static DISP_DIFF: AtomicU64 = AtomicU64::new(0);    // my_dispatch_code != 실제 out 횟수
static FULL_OK: AtomicU64 = AtomicU64::new(0);      // 통합예측 my_full == 실제 out
static FULL_DIFF: AtomicU64 = AtomicU64::new(0);    // 통합예측 my_full != 실제 out
const RE_ARM_MAX: u64 = 200000;   // late-game 교전까지 무장. 로깅=디스패치 우선(dispcmp.txt)

// ── retreat_engage 훅(반환 0=대체처리·원본스킵 / 1=fall-through·원본실행) ──
// plan_base 캡처(항상) + replacement(cfg replace=1) + RE 리턴훅 하네스(cfg capture=1)
// 스택: rcx=param_1(출력 sret), [entry_rsp+0x30]=arg6/p6, [+0x38]=param_7, [+0x28]=p5, [+0x48]=p9
// ── per-replay 캡처 리셋: 모든 카운터/파일init/히스토그램/추적기 초기화 + dispcmp truncate ──
unsafe fn reset_captures() {
    let counters: [&AtomicU64; 23] = [&DD7_ARMED,&DD7_LOGGED,&RNG_ARMED,&RNG_LOGGED,&PG_ARMED,&TE_ARMED,&TE_CALLS,&TE_TRANS_N,&RECALL_ARMED,&FC59_RAW,&FC59_ARM,&FC59_FILT,&ROLL_LOGGED,&CHACHA_ARMED,&CHACHA_LOGGED,&TTD_ARMED,&RE_ARMED,&RE_LOGGED,&DISP_LOGGED,&DISP_OK,&DISP_DIFF,&FULL_OK,&FULL_DIFF];
    for c in counters { c.store(0, Ordering::Relaxed); }
    let flags: [&AtomicBool; 8] = [&DD7_FILE_INIT,&RNG_FILE_INIT,&PG_FILE_INIT,&TE_FILE_INIT,&RECALL_FILE_INIT,&CHACHA_FILE_INIT,&TTD_FILE_INIT,&RE_FILE_INIT];
    for f in flags { f.store(false, Ordering::Relaxed); }
    for k in 0..16 { TE_PHASE_HIST[k].store(0, Ordering::Relaxed); TE_SUB_HIST[k].store(0, Ordering::Relaxed); }
    if let Ok(mut tr) = TE_TRACK.lock() { tr.clear(); }
    write_named("dispcmp.txt", "=== 디스패치 캡처 (per-replay 리셋) ===\n");
    // ★seed+strat 기록(seedstrat.txt): 현 리플레이의 시드(CUR_SEED, sim중 freeze=실제시드)+팀전술 12필드.
    //   유용한 리플레이 식별 후 seed_set=이값 으로 시드 재현(strat은 strat_rotate OFF+수동셋 필요). 매 sim-start overwrite=현 리플레이값만.
    {
        let seed = CUR_SEED.load(Ordering::Relaxed);
        let (b, rd) = STRAT_CUR.lock().map(|g| *g).unwrap_or(([0u8;12],[0u8;12]));
        const NM: [&str; 12] = ["foc","jng","srp","srt","bld","bat","mor","twr","def","fin","wav","end"];
        let mut s = format!("seed=0x{:x}\n", seed);
        s.push_str("blue:"); for f in 0..12 { s.push_str(&format!(" {}={}", NM[f], b[f])); } s.push('\n');
        s.push_str("red: "); for f in 0..12 { s.push_str(&format!(" {}={}", NM[f], rd[f])); } s.push('\n');
        write_named("seedstrat.txt", &s);
    }
}

// ════ plan_lane_predicate(0x2080760) 순수재현 — churn제거(RVA_LANE_PRED + DAT 3개 const화) ════
// 완전대체 1단계(2026-06-19). 디컴+exe값추출 확정. DAT값=기존 POKE_ANC_A/B[0..3]와 동일.
const LANE_PRED_IDX: [usize; 4] = [0, 1, 3, 2];                   // DAT_1435eef90 (lane→threshold idx)
const LANE_ANC_B: [u64; 4] = [496000, 176000, 256000, 351000];   // DAT_1435eefb0
const LANE_ANC_D: [u64; 4] = [752000, 592000, 448000, 800000];   // DAT_1435eefd0
//   self=dd7_slot128(sim,p5[0x6a0]), now=dd7_slot20(sim), thr=p9[0x360+team*0x20+IDX[lane]*8].
//   now>=thr→false. else 앵커거리 q=isqrt(dist²)/speed → (now+q+hostscalar)<thr. (param_8=0)
unsafe fn my_lane_predicate(lane: u8, team: u64, p5: usize, p6: usize, p9: usize) -> bool {
    if lane > 3 || team > 1 { return false; }
    let l80 = rd_u64(p6).unwrap_or(0) as usize;
    if !ptr_ok(l80) { return false; }
    let sim = rd_u64(l80).unwrap_or(0) as usize;
    if !ptr_ok(sim) || !ptr_ok(p9) { return false; }
    let ent = dd7_slot128(sim, rd_u64(p5 + 0x6a0).unwrap_or(0));
    if ent == 0 || !readable(ent + 0x650, 8) || !readable(ent + 0x628, 8) { return false; }
    let thr = rd_u64(p9 + 0x360 + (team as usize)*0x20 + LANE_PRED_IDX[lane as usize]*8).unwrap_or(0);
    let now = dd7_slot20(sim) as u64;
    if now >= thr { return false; }
    let ex = rd_u64(ent + 0x648).unwrap_or(0);
    let ey = rd_u64(ent + 0x650).unwrap_or(0);
    let (ax, ay) = if team == 0 { (LANE_ANC_B[lane as usize], LANE_ANC_D[lane as usize]) }
                   else { (LANE_ANC_D[lane as usize], LANE_ANC_B[lane as usize]) };
    let dist = isqrt(sqd(ax, ay, ex, ey));
    let speed = rd_u64(ent + 0x628).unwrap_or(0);
    if speed == 0 { return false; }
    let q = dist / speed;
    let host = rd_u64(p6 + 8).unwrap_or(0) as usize;
    let hostb = rd_u64(host + 8).unwrap_or(0) as usize;
    let hostsc = rd_i64(hostb + 0x12f8).unwrap_or(0);
    ((now as i64).wrapping_add(q as i64).wrapping_add(hostsc) as u64) < thr
}

// ★facet#5 dispatch 출력코드(RNG-free) 함수화(retreat_capture disppred 블록 로직). 반환 0/3/7/8 or -99(engage→roll/Stage B). 게임 vtable getter(vt0x38=cVar4 섀도우, SAFE).
unsafe fn my_retreat_dispatch(p5: usize, p6: usize, candidate: usize, rh: usize, robj: usize, rvt: usize) -> i64 {
    let team = rd_i64(p5 + 0x6a8).unwrap_or(-99);
    if team != 0 && team != 1 { return -99; }
    let geo2 = rd_u64(p6 + 0x10).unwrap_or(0) as usize;
    let zone = geo2.wrapping_add((team as usize) * 0x228);
    if !ptr_ok(zone) || !readable(zone + 0x179, 1) || !ptr_ok(rh) || !ptr_ok(robj) || !ptr_ok(rvt) || candidate == 0 || !readable(candidate + 0x658, 8) { return -99; }
    let plv28 = rd_u64(p6 + 8).unwrap_or(0) as usize;
    let host = rd_u64(plv28 + 8).unwrap_or(0) as usize;
    let l80_600 = rd_u64(host + 0x12c0).unwrap_or(0);
    let cx = rd_u64(candidate + 0x648).unwrap_or(0);
    let cy = rd_u64(candidate + 0x650).unwrap_or(0);
    let postag: i64 = if l80_600.wrapping_sub(cy) < cx { 2 } else { 0 };
    let za20 = rd_i32(zone + 0x20).unwrap_or(-99) as i64;
    let za48 = rd_i32(zone + 0x48).unwrap_or(-99) as i64;
    let za70 = rd_i32(zone + 0x70).unwrap_or(-99) as i64;
    let ce_pt = cand_ent_valid(rh, team, postag);
    let ce_1 = cand_ent_valid(rh, team, 1);
    let cvar4: i64 = { let g = rd_u64(rvt + 0x38).unwrap_or(0) as usize; if ptr_ok(g) && ptr_ok(robj) { let f: Getter1 = core::mem::transmute(g); f(robj) & 0xff } else { -1 } };
    let l238: u64 = if cvar4 == 0 { if readable(robj + 0xecd8 + (team as usize) * 0x18, 8) { rd_u64(robj + 0xecd8 + (team as usize) * 0x18).unwrap_or(0) } else { 0 } } else { rd_u64(p5 + 0x86 * 8).unwrap_or(0) };
    let cvar6 = ((l238 >> 16) & 0xff) as i64;
    my_dispatch_code(cvar6, ce_pt, ce_1, zone, postag, za20, za48, za70, rh, geo2, p5)
}
unsafe extern "C" fn retreat_capture(saved: usize, entry_rsp: usize) -> u64 {
    if !ptr_ok(entry_rsp) { return 1; }
    apply_call_ablate();  // ★오더 콜 ablation 패치 적용/복원 (want==applied면 즉시 return, 핫패스 부담無)
    apply_lane_gate();    // ★오더 라인후보 게이트 ablation (lane_gate 0/1/2)
    apply_type3_ablate(); // ★오더 transition type3 콜 ablation (매크로 전환 영향 검증)
    // ★새 sim 첫 호출이면 캡처 리셋(메뉴서 IN_MENU=true → 첫 sim 훅이 swap(false)+reset)
    if REPLAY_RESET.load(Ordering::Relaxed) && IN_MENU.swap(false, Ordering::Relaxed) { reset_captures(); }
    SEL228_FRESH.store(-777, Ordering::Relaxed);  // 진입시 리셋; df0c10훅이 신선 셀렉터로 갱신(없으면 -777=early-exit)
    PRED_ROLL_VALID.store(false, Ordering::Relaxed);  // 교전롤 예측 리셋(롤 fcd980 호출시 갱신)
    let arg6 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;
    if ptr_ok(arg6) {
        let pb = rd_u64(arg6 + 0x18).unwrap_or(0) as usize;
        if ptr_ok(pb) { CAP_PB_RAW.store(pb, Ordering::Relaxed); }
    }
    let cap_on = HARNESS_ON && CAP_ON.load(Ordering::Relaxed);
    let repl_on = REPL_ON.load(Ordering::Relaxed);
    if !cap_on && !repl_on && !ENGAGE_REPL.load(Ordering::Relaxed) { return 1; }  // 할 일 없음 → 원본 통과
    // ★ 로딩중(게임 미안정) 게임함수 호출 방지 — 런칭 크래시 완화. 안정 전엔 원본 통과.
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return 1; }
    let p1 = rd_u64(saved + 0x28).unwrap_or(0) as usize;   // rcx = param_1 (출력 sret)
    let p2 = rd_u64(saved + 0x20).unwrap_or(0) as usize;   // rdx = param_2 (entity/roster desc)
    let p5 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize; // arg5 (config)
    let p6 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize; // arg6 (로스터 2-ptr desc)
    let p7 = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize; // arg7 = param_7
    let p9 = rd_u64(entry_rsp + 0x48).unwrap_or(0) as usize; // arg9 (plan_lane_predicate 테이블)
    let self_e = rd_u64(saved + 0x10).unwrap_or(0) as usize; // r9 = param_4 (self/AI 엔티티)
    if !ptr_ok(p1) { return 1; }
    // ── 공통: first-part(candidate-resolve) + lane게이트(plan_lane_predicate) 충실 호출 ──
    let exe = exe_base();
    let rh = rd_u64(p6).unwrap_or(0) as usize;
    let robj = rd_u64(rh).unwrap_or(0) as usize;
    let rvt = rd_u64(rh + 8).unwrap_or(0) as usize;
    let guards_ok = exe != 0 && ptr_ok(p2) && ptr_ok(p5) && ptr_ok(p6) && ptr_ok(p9)
        && readable(p2 + 0x60, 1) && readable(p2 + 0x48, 8) && ptr_ok(rh) && readable(rh, 16)
        && ptr_ok(robj) && ptr_ok(rvt);
    // CALL A: candidate-resolve roster_vt[0x128](roster_obj, [p5+0x6a0]) — SAFE(순수 selector)
    let (candidate, cand_cnt, depth_ratio): (usize, i64, i64) = if guards_ok {
        let team_units = rd_u64(p5 + 0x6a0).unwrap_or(0) as usize;
        let g = rd_u64(rvt + 0x128).unwrap_or(0) as usize;
        if ptr_ok(g) {
            let f: G2 = core::mem::transmute(g);
            let cand = f(robj, team_units) as usize;
            if ptr_ok(cand) && readable(cand, 0x660) {
                let cnt = rd_i64(cand + 0x610).unwrap_or(0);
                let dep = rd_i64(cand + 0x658).unwrap_or(0);
                (cand, cnt, if cnt != 0 { dep * 100 / cnt } else { -1 })
            } else { (cand, -1, -1) }
        } else { (0, -1, -1) }
    } else { (0, -1, -1) };
    // CALL G: plan_lane_predicate((u8)[p2+0x60], [p2+0x48], 0,0, p5, p6, p9, 0) — SAFE. 0이면 -1.
    let my_lp: i32 = if guards_ok {
        let lane = std::ptr::read_unaligned((p2 + 0x60) as *const u8);
        let team = rd_u64(p2 + 0x48).unwrap_or(0);
        // ★완전대체 1a 완료(2026-06-19): my_lane_predicate 순수재현 → 게임 plan_lane_predicate와 DIFF=0(~17.6k샘플) 검증완료 → shadow+RVA_LANE_PRED 제거(churn 소멸).
        my_lane_predicate(lane, team, p5, p6, p9) as i32
    } else { -9 };
    // ── ★ Stage B: facet#5 engage ENTRY 완전대체 (cfg engage_repl, replace와 독립). ──
    //   engage 브랜치(candidate≠0 & cand_cnt≠0 & my_lp≠0 & dispatch==-99)만 my_engage_emit으로 대체(출력+RNG writeback).
    //   검증 2500/2500 diverse. None(가드실패)→passthrough(desync 방지). 게이트 early-exit은 empirically 0발화.
    if ENGAGE_REPL.load(Ordering::Relaxed) && guards_ok && readable(p1, 8) && candidate != 0 && cand_cnt != 0 && my_lp != 0 {
        let d = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_retreat_dispatch(p5, p6, candidate, rh, robj, rvt))).unwrap_or(0);
        if d == -99 {  // engage
            let self_e = rd_u64(saved + 0x10).unwrap_or(0) as usize;  // r9=param4=RNG state
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_engage_emit(p2, p5, p6, p9, self_e))).unwrap_or(None) {
                Some(out) => {
                    core::ptr::write_unaligned(p1 as *mut i64, out);
                    ENGAGE_REPL_N.fetch_add(1, Ordering::Relaxed);
                    return 0;  // handled → 원본 skip
                }
                None => { ENGAGE_REPL_PASS.fetch_add(1, Ordering::Relaxed); }  // 가드실패 → passthrough
            }
        }
    }
    // ── ★ REPLACEMENT 1단계: 검증된 퇴각경로만 우리 출력으로 대체(원본 스킵). ──
    // candidate!=0 && cnt!=0 && lane_pred==0 → 게임도 lane게이트서 -1(df0c10/RNG 도달 前) → desync 없음.
    if repl_on && guards_ok && readable(p1, 8) {
        // ★Stage A 충실대체(RNG-free 경로): candidate없음→0, lane_pred==0→-1, proceed시 dispatch(0/3/7/8).
        //   engage(-99=roll)는 Stage B(roll writeback+engage-target idx 재현) 미완 → passthrough.
        let out: Option<i64> =
            if candidate == 0 || cand_cnt == 0 { Some(0) }
            else if my_lp == 0 { Some(-1) }
            else {
                let d = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_retreat_dispatch(p5, p6, candidate, rh, robj, rvt))).unwrap_or(-99);
                if matches!(d, 0 | 3 | 7 | 8) { Some(d) } else { None }  // -99=engage → 별도 ENGAGE_REPL 블록서 처리(아래)
            };
        if let Some(o) = out {
            let rv = REPL_OUT.load(Ordering::Relaxed);
            let final_out = if rv != -1 { rv } else { o };   // REPL_OUT≠-1=수동 override(테스트), else=충실값
            core::ptr::write_unaligned(p1 as *mut i64, final_out);
            REPL_HANDLED.fetch_add(1, Ordering::Relaxed);
            return 0;  // handled → 원본 실행 안 함
        }
        // engage path → fall through (capture/passthrough)
    }
    // ── CAPTURE 하네스 (cfg capture=1) → 원본 실행시키고 리턴훅으로 검증 ──
    if !cap_on { return 1; }
    let thunk = RET_THUNK.load(Ordering::Relaxed);
    if thunk == 0 || RE_ARMED.load(Ordering::Relaxed) >= RE_ARM_MAX { return 1; }
    let n = RE_ARMED.fetch_add(1, Ordering::Relaxed);
    // ★panic-safe(mod-safety): 리팩터된 0.4.13_5 retreat의 capture/dispatch 경로 panic(인덱스/unwrap 등)이
    //   FFI UB로 게임 크래시 → catch_unwind로 차단. 패닉 케이스만 건너뛰고(passthrough=1) 게임 계속.
    let cap_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> u64 {
    // param_7 raw 덤프 (필드 오프셋 0,8,0x10,0x18,0x20,0x28,0x30,0x38,0x40,0x48,0x50,0x58,0x60,0x68,0x70)
    let mut p7s = String::new();
    if ptr_ok(p7) {
        for o in [0usize,8,0x10,0x18,0x20,0x28,0x30,0x38,0x40,0x48,0x50,0x58,0x60,0x68,0x70] {
            p7s.push_str(&format!("{:#x}:{} ", o, rd_i64(p7 + o).unwrap_or(-1)));
        }
    } else { p7s.push_str("(null)"); }
    // arg5 config 일부
    let cfg = if ptr_ok(p5) {
        format!("cfg[0x46]={} [0x47]={} [0x7a]={} [0xd4]={} [0xd5]={}",
            rd_i64(p5+0x46*8).unwrap_or(-1), rd_i64(p5+0x47*8).unwrap_or(-1), rd_i64(p5+0x7a*8).unwrap_or(-1),
            rd_i64(p5+0xd4*8).unwrap_or(-1), rd_i64(p5+0xd5*8).unwrap_or(-1))
    } else { "cfg(null)".into() };
    // 예측: ①candidate==0 or cand_cnt==0 → 0(first-part none) ②lane_pred==0 → -1 ③else proceed(9999)
    let lp_pred: i64 = if candidate == 0 || cand_cnt == 0 { 0 }
        else if my_lp == 0 { -1 }
        else { 9999 };
    // ★중간디스패치 판별자 수집 (오프셋 정정 2026-06-16, decomp 380~1042 재추적):
    //   - team = param_5[0xd5] = *(p5+0x6a8)  (zone블록 인덱스)
    //   - zone블록 = param_6[2](=[p6+0x10]) + team*0x228  ← 이전 버그: team*0x228 누락
    //   - 5쌍 플래그: lock@zone+0xf8+k*0x20(==0 잠금해제), type@zone+0xf9+k*0x20(위치태그 매칭)
    //   - param_7 = 5슬롯×0x18 디스패치 디스크립터: gate@+0(byte), recIdx@+8(u64), distThr@+0x10(u64)
    let (disp, my_disp) = if !DISPPRED.load(Ordering::Relaxed) {
        // ★dispatch 예측 OFF(기본): 비용절감(dispatch 진단블록 스킵). -1/roll/engage는 그대로 검증됨.
        //   (shadow_fa1ea0는 이제 순수 my_fa1ea0라 disppred=1도 세그폴트 위험 없음 — OFF는 단지 기본 휴면.)
        ("(disppred off)".to_string(), -99i64)
    } else {
        let alive = rd_i32(self_e+0x48).unwrap_or(-99);
        let team = rd_i64(p5 + 0x6a8).unwrap_or(-99);   // = param_5[0xd5]
        let geo2 = rd_u64(p6 + 0x10).unwrap_or(0) as usize;  // = param_6[2]
        let zone = if team==0 || team==1 { geo2.wrapping_add((team as usize)*0x228) } else { 0 };
        let mut zf = String::new();
        if ptr_ok(zone) && readable(zone+0x178+1, 1) {
            for k in 0..5usize { zf.push_str(&format!("{}:{},{} ", k, rd_u8(zone+0xf8+k*0x20), rd_u8(zone+0xf9+k*0x20))); }
        } else { zf.push_str("(zone bad)"); }
        let mut sl = String::new();
        if ptr_ok(p7) && readable(p7+0x70+8, 8) {
            for k in 0..5usize {
                let b = p7 + k*0x18;
                sl.push_str(&format!("{}:g{}/i{}/t{} ", k, rd_u8(b), rd_i64(b+8).unwrap_or(-1), rd_i64(b+0x10).unwrap_or(-1)));
            }
        } else { sl.push_str("(p7 bad)"); }
        let win = rd_i64(p2 + 0xb*8).unwrap_or(-99);
        // hp_thr = 0x3c - (min(p5[0x46],100)*0x67 >> 9)  (60 - s46*103/512)
        let s46 = rd_i64(p5+0x46*8).unwrap_or(0).min(100);
        let hp_thr = 0x3c - ((s46*0x67) >> 9);
        // ── ★my_dispatch_diag: 포팅 조각 검증 (decomp 446/950 postag, f26ad0 PORT, zone매칭) ──
        //   plVar28=param_6[1]=*(p6+8); host=local_80=*(plVar28+8); local_80[600]=*(host+0x12c0)
        //   postag cVar4 = ((local_80_600 - cand_y) < cand_x) ? 2 : 0   (unsigned)
        //   f26ad0 recall_count = Σ slot0..4 [zf_lock==0 && zf_type==postag && ally(rh+0x1e0+team*0x28+slot*8).hp%>=41]
        let plv28 = rd_u64(p6 + 8).unwrap_or(0) as usize;
        let host  = rd_u64(plv28 + 8).unwrap_or(0) as usize;
        let l80_600 = rd_u64(host + 0x12c0).unwrap_or(0);
        let (postag, rcnt, mpost, m1): (i64, i64, i32, i32) =
            if candidate != 0 && readable(candidate+0x650,8) && team>=0 && team<=1 && ptr_ok(zone) && ptr_ok(rh) {
                let cx = rd_u64(candidate+0x648).unwrap_or(0);
                let cy = rd_u64(candidate+0x650).unwrap_or(0);
                let ptag: i64 = if l80_600.wrapping_sub(cy) < cx { 2 } else { 0 };
                let (mut cnt, mut mp, mut mo) = (0i64, 0i32, 0i32);
                for k in 0..5usize {
                    let lock = rd_u8(zone+0xf8+k*0x20) as i64;
                    let typ  = rd_u8(zone+0xf9+k*0x20) as i64;
                    if lock==0 && typ==ptag {
                        mp=1;
                        let ally = rd_u64(rh+0x1e0 + (team as usize)*0x28 + k*8).unwrap_or(0) as usize;
                        if ptr_ok(ally) && readable(ally+0x658,8) {
                            let mx = rd_i64(ally+0x610).unwrap_or(0);
                            if mx>0 && rd_i64(ally+0x658).unwrap_or(0)*100/mx >= 41 { cnt+=1; }
                        }
                    }
                    if k>=1 && lock==0 && typ==1 { mo=1; }
                }
                (ptag, cnt, mp, mo)
            } else { (-1,-1,-1,-1) };
        // ★타이밍 게이트(decomp 483-488): gap=max(0,now2-distThr) <= ctx_scalar*3 → 디스패치루프 진입(RECALL후보)
        //   now2 = roster_vt[0x20](robj) 섀도우CALL(SAFE), ctx_scalar = host[0x12f8], distThr = p7 slot+0x10
        let now2: i64 = {
            let g = rd_u64(rvt+0x20).unwrap_or(0) as usize;
            if ptr_ok(g) && ptr_ok(robj) { let f: Getter1 = core::mem::transmute(g); f(robj) } else { -1 }
        };
        let ctxs = rd_i64(host+0x12f8).unwrap_or(-1);
        let dthr = rd_i64(p7+0x10).unwrap_or(-1);
        let gap = if now2 > dthr { now2 - dthr } else { 0 };
        let tgate: i64 = if now2>=0 && ctxs>=0 && dthr>=0 { if gap <= ctxs*3 { 1 } else { 0 } } else { -1 };
        // ★candEnt 유효성 (decomp 474-478): type=postag(local_238[0]) / type=1(local_238[1])
        let ce_pt = cand_ent_valid(rh, team, postag);
        let ce_1  = cand_ent_valid(rh, team, 1);
        // ★★ cVar6 마스터 셀렉터 (decomp 750-763): cVar4=rvt[0x38](robj) 섀도우CALL;
        //   cVar4==0 → local_238=robj[0xecd8+team*0x18](rvt[0x58]=FUN_141976a30), else param_5[0x86]; cVar6=byte2(local_238)
        //   가설: cVar6==1→RECALL, ==0→STAND/ZONE, ==2→battle-poke
        let cvar4: i64 = {
            let g = rd_u64(rvt+0x38).unwrap_or(0) as usize;
            if ptr_ok(g) && ptr_ok(robj) { let f: Getter1 = core::mem::transmute(g); f(robj) & 0xff } else { -1 }
        };
        let l238: u64 = if cvar4==0 {
            if team>=0 && team<=1 && readable(robj+0xecd8+(team as usize)*0x18, 8) { rd_u64(robj+0xecd8+(team as usize)*0x18).unwrap_or(0) } else { 0 }
        } else { rd_u64(p5 + 0x86*8).unwrap_or(0) };
        let cvar6 = ((l238 >> 16) & 0xff) as i64;
        // ★STAND vs ZONE 판별: zoneblk alive 필드 (decomp 969 zoneblk[9]=+0x48, 1021 plVar28b[4]=+0x70/+0x20 i32 ≥ -3)
        let (za20, za48, za70) = if ptr_ok(zone) {
            (rd_i32(zone+0x20).unwrap_or(-99), rd_i32(zone+0x48).unwrap_or(-99), rd_i32(zone+0x70).unwrap_or(-99))
        } else { (-99,-99,-99) };
        let _ = (alive, win, hp_thr, tgate, now2, mpost, m1);
        // ★통합 디스패치 예측 (검증 76/76): 7=RECALL/8=STAND/3=ZONE, -99=cVar6 기타(roll/none)
        let mydisp = my_dispatch_code(cvar6, ce_pt, ce_1, zone, postag, za20 as i64, za48 as i64, za70 as i64, rh, geo2, p5);
        (format!("team={} hp%={} ★cVar6={} cVar4={} postag={} rcnt={} cePt={} ce1={} ★mydisp={} za[20:{} 48:{} 70:{}] zf[{}] slots[{}]",
            team, depth_ratio, cvar6, cvar4, postag, rcnt, ce_pt, ce_1, mydisp, za20, za48, za70, zf.trim(), sl.trim()), mydisp)
    };
    let pre = format!("[re #{}] {} cand={}(cnt={} ratio={}) lane_pred={} 예측={} | DISP {}\n   p7=[{}]",
        n, cfg, if candidate != 0 {"O"} else {"0"}, cand_cnt, depth_ratio, my_lp, lp_pred, disp, p7s);
    let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    if !ptr_ok(orig_ret) { return 1; }
    let pushed = if let Ok(mut st) = RET_STACK.lock() {
        if st.len() < 64 { st.push(RetFrame { key: entry_rsp, orig_ret, mine: lp_pred, kind: 1, pre, p5, p6, disp_pred: my_disp }); true } else { false }
    } else { false };
    if !pushed { return 1; }
    // ★engage footprint+예측: 진입 RNG 스냅 + engage 브랜치면 my_engage_predict(out,words) 저장. kind1 리턴서 실제와 대조.
    if ptr_ok(self_e) && readable(self_e + 0x138, 8) {
        let i0 = rd_u64(self_e + 0x100).unwrap_or(0);
        let c0 = rd_u64(self_e + 0x130).unwrap_or(0);
        // engage 브랜치 판별: candidate!=0 && cand_cnt!=0 && my_lp!=0 && dispatch==-99(=engage)
        let (pred_out, pred_words, pca, pcb): (i64, i64, i64, i64) =
            if candidate != 0 && cand_cnt != 0 && my_lp != 0 {
                let d = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_retreat_dispatch(p5, p6, candidate, rh, robj, rvt))).unwrap_or(-99);
                if d == -99 {
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_engage_predict(p2, p5, p6, p9, self_e))).unwrap_or(None) {
                        Some((o, w, ca, cb)) => (o, w, ca, cb), None => (-777, -777, -1, -1)
                    }
                } else { (-777, -777, -1, -1) }
            } else { (-777, -777, -1, -1) };
        if let Ok(mut sn) = RE_SNAP.lock() { if sn.len() < 64 { sn.push((entry_rsp, self_e, i0, c0, pred_out, pred_words, pca, pcb)); } }
    }
    if readable(entry_rsp, 8) { core::ptr::write_unaligned(entry_rsp as *mut usize, thunk); }
    else if let Ok(mut st) = RET_STACK.lock() { if let Some(p) = st.iter().rposition(|f| f.key == entry_rsp) { st.remove(p); } }
    1  // fall-through (원본 실행 → 리턴훅 검증)
    }));
    match cap_result {
        Ok(v) => v,
        Err(_) => {
            let c = RE_PANIC.fetch_add(1, Ordering::Relaxed);
            if c < 30 { append_named("recmp.txt", &format!("[★PANIC caught] retreat_capture re#{} — engage/dispatch 케이스 추정, 건너뜀(게임 계속)\n", n)); }
            1  // passthrough: 게임 원본 실행
        }
    }
}

// ── plan_base 자동탐지 (메인스레드 post_update에서 호출; 핫패스 아님) ──
// ① retreat 경로(검증됨) 우선 ② 실패 시 레지스터/pstate 스캔 폴백
unsafe fn try_find_plan_base() {
    // ① retreat_engage 경로
    let raw = CAP_PB_RAW.load(Ordering::Relaxed);
    if ptr_ok(raw) {
        let sc = roster_sig(raw);
        if sc >= 6 {
            CAP_PB.store(raw, Ordering::Relaxed);
            DIAG_DONE.store(true, Ordering::Relaxed);
            let mut s = format!("[{}ms] === plan_base = retreat_engage 경로 (score {}) ===\n★plan_base=0x{:x}\n", now_ms(), sc, raw);
            for (t,i,e) in &roster(raw) {
                s.push_str(&format!("  t{} #{} e=0x{:x} pos=({},{}) hp={}/{} speed={}\n",
                    t,i,e, rd_i64(e+E_POSX).unwrap_or(0), rd_i64(e+E_POSY).unwrap_or(0),
                    rd_i64(e+E_HP).unwrap_or(-1), rd_i64(e+E_MAXHP).unwrap_or(-1), rd_i64(e+E_SPEED).unwrap_or(-1)));
            }
            write_named("diag.txt", &s);
            append_log(&format!("[{}ms] ★plan_base(retreat) 0x{:x} score={}\n", now_ms(), raw, sc));
            return;
        }
    }

    // ② 폴백: 레지스터/pstate 스캔
    let rcx = CAP_RCX.load(Ordering::Relaxed);
    let rdx = CAP_RDX.load(Ordering::Relaxed);
    let r8  = CAP_R8.load(Ordering::Relaxed);
    let r9  = CAP_R9.load(Ordering::Relaxed);
    let pstate = CAP_PSTATE.load(Ordering::Relaxed);
    if rdx == 0 || pstate == 0 { return; }

    let mut cands: Vec<(String, usize)> = Vec::new();
    for (nm, v) in [("rcx",rcx),("rdx",rdx),("r8",r8),("r9",r9)] {
        if ptr_ok(v) { cands.push((nm.to_string(), v)); }
        for o in [0x8usize,0x10,0x18,0x20,0x28,0x30,0x38,0x40] {
            if let Some(p) = rd_u64(v.wrapping_add(o)) { cands.push((format!("*({}+0x{:x})", nm, o), p as usize)); }
        }
    }
    if ptr_ok(pstate) && readable(pstate, 0x800) {
        let mut o = 0usize;
        while o < 0x800 {
            if let Some(p) = rd_u64(pstate.wrapping_add(o)) { cands.push((format!("pstate+0x{:x}", o), p as usize)); }
            o += 8;
        }
    }
    let mut hits: Vec<(String, usize, usize)> = Vec::new();
    for (prov, c) in cands {
        let sc = roster_sig(c);
        if sc >= 6 { hits.push((prov, c, sc)); }
    }
    if hits.is_empty() { return; }
    hits.sort_by(|a,b| b.2.cmp(&a.2));
    let best = hits[0].1;
    CAP_PB.store(best, Ordering::Relaxed);
    DIAG_DONE.store(true, Ordering::Relaxed);

    let mut s = format!("[{}ms] === plan_base = 스캔 폴백 (retreat 경로 실패) ===\n", now_ms());
    s.push_str(&format!("regs: rcx=0x{:x} rdx=0x{:x} r8=0x{:x} r9=0x{:x} pstate=0x{:x}\n", rcx,rdx,r8,r9,pstate));
    for (prov, addr, sc) in hits.iter().take(16) {
        s.push_str(&format!("  [score {}] {} = 0x{:x}\n", sc, prov, addr));
    }
    s.push_str(&format!("\n★채택 plan_base=0x{:x}\n", best));
    for (t,i,e) in &roster(best) {
        s.push_str(&format!("  t{} #{} e=0x{:x} pos=({},{}) hp={}/{} speed={}\n",
            t,i,e, rd_i64(e+E_POSX).unwrap_or(0), rd_i64(e+E_POSY).unwrap_or(0),
            rd_i64(e+E_HP).unwrap_or(-1), rd_i64(e+E_MAXHP).unwrap_or(-1), rd_i64(e+E_SPEED).unwrap_or(-1)));
    }
    write_named("diag.txt", &s);
    append_log(&format!("[{}ms] ★plan_base 탐지 완료 0x{:x} (diag.txt)\n", now_ms(), best));
}

// ── 오더 라인-합류후보 게이트 ablation (update_state RNG accept gate, 확률 order/100) ──
//   FUN_1420d9720(0x20d9720) 5라인 루프 게이트 0x20d9bf9(JBE skip; rng<thr=후보push). thr=min(order,100)*10.
//   cfg lane_gate: 0=원본 / 1=OFF(후보0개,항상skip) / 2=ALL(후보다,게이트NOP=fall-through). 결과비교로 라인후보가 실제행동에 닿는지 검증.
//   RNG는 게이트 위라 보존. 6B in-place. 안전검증=현재바이트 원본/OFF/ALL 중 하나 아니면 중단.
static LANE_GATE: AtomicU8 = AtomicU8::new(0);
static LANE_GATE_APPLIED: AtomicU8 = AtomicU8::new(255);
const LANE_GATE_RVA: usize = 0x20d9bf9;
const LANE_GATE_ORIG: [u8; 6] = [0x0f,0x86,0x41,0xff,0xff,0xff];  // JBE 0x20d9b40 (rel32)
const LANE_GATE_OFF:  [u8; 6] = [0xe9,0x42,0xff,0xff,0xff,0x90];  // JMP 0x20d9b40 + NOP (항상 skip=후보0)
const LANE_GATE_ALL:  [u8; 6] = [0x0f,0x1f,0x44,0x00,0x00,0x90];  // 6B NOP (항상 fall-through=후보다)
unsafe fn patch6(addr: usize, bytes: &[u8; 6]) {
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(addr, 6, RWX, &mut old) == 0 { return; }
    core::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, 6);
    VirtualProtect(addr, 6, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, 6);
}
unsafe fn apply_lane_gate() {
    let want = LANE_GATE.load(Ordering::Relaxed);
    if want == LANE_GATE_APPLIED.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 { return; }
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let addr = base + LANE_GATE_RVA;
    if !readable(addr, 6) { return; }
    let cur: [u8; 6] = [rd_u8(addr),rd_u8(addr+1),rd_u8(addr+2),rd_u8(addr+3),rd_u8(addr+4),rd_u8(addr+5)];
    if !(cur == LANE_GATE_ORIG || cur == LANE_GATE_OFF || cur == LANE_GATE_ALL) {
        write_named("lane_gate.txt", &format!("ABORT cur={:02x?} (RVA mismatch?)\n", cur));
        return;
    }
    let target: &[u8; 6] = match want { 1 => &LANE_GATE_OFF, 2 => &LANE_GATE_ALL, _ => &LANE_GATE_ORIG };
    patch6(addr, target);
    LANE_GATE_APPLIED.store(want, Ordering::Relaxed);
    write_named("lane_gate.txt", &format!("lane_gate={} APPLIED @ {:#x} bytes={:02x?}\n", want, addr, target));
}

// ── 오더 transition_engine 타입3 콜 ablation (subplan 전환엔진, order*7+300 확률) ──
//   FUN_141e961d0 내 type3 push 게이트 2지점(0x1e9d318/0x1e9d59b, jae skip). 1=차단(jae→jmp, push0개), 0=원본.
//   RNG는 게이트 위라 보존. 1바이트 패치(0x73↔0xEB, 둘째 0x5f 검증). ★0xb콜과 별개 경로(다른 디스패처 핸들러→plan_state subplan/phase write 잠재=살아있을 수 있음).
static TYPE3_ABLATE: AtomicBool = AtomicBool::new(false);
static TYPE3_APPLIED: AtomicBool = AtomicBool::new(false);
const T3_GATE_A_RVA: usize = 0x1e9d318;   // jae 0x1e9d379 (원본 73 5f)
const T3_GATE_B_RVA: usize = 0x1e9d59b;   // jae 0x1e9d5fc (원본 73 5f)
unsafe fn apply_type3_ablate() {
    let want = TYPE3_ABLATE.load(Ordering::Relaxed);
    if want == TYPE3_APPLIED.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 { return; }
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let sites = [base + T3_GATE_A_RVA, base + T3_GATE_B_RVA];
    // 안전검증: 둘째 바이트 0x5f, 첫바이트 want면 0x73(원본)·아니면 0xEB(패치)
    for &addr in sites.iter() {
        if !readable(addr, 2) { return; }
        let (b0, b1) = (rd_u8(addr), rd_u8(addr + 1));
        let ok = b1 == 0x5f && (if want { b0 == 0x73 } else { b0 == 0xEB });
        if !ok { write_named("type3_ablate.txt", &format!("ABORT @{:#x} {:02x}{:02x} want={} (RVA mismatch?)\n", addr, b0, b1, want)); return; }
    }
    let newb: u8 = if want { 0xEB } else { 0x73 };
    for &addr in sites.iter() {
        let mut old: u32 = 0;
        if VirtualProtect(addr, 1, 0x40, &mut old) == 0 { continue; }
        core::ptr::write_unaligned(addr as *mut u8, newb);
        VirtualProtect(addr, 1, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), addr, 1);
    }
    TYPE3_APPLIED.store(want, Ordering::Relaxed);
    write_named("type3_ablate.txt", &format!("type3_ablate={} APPLIED @ {:#x}/{:#x} (jae→jmp 차단)\n", want, sites[0], sites[1]));
}

// ── 오더 콜(0xb) ablation + 발화 카운터 ──
//   call_ablate=1 → push 2지점을 카운터스텁으로 점프(콜 차단 + 발화 횟수 카운트), =0 → 원본 복원.
//   스텁: push rcx; lock inc [CALL_BLOCKED]; pop rcx; jmp [rip](합류점). RAX 등 무손상(RCX만 push/pop 보존).
//   패치=각 지점 14바이트(FF 25 00000000 + 8B stub절대주소=jmp qword[rip]). RNG·합류 레지스터 보존. RVA불일치 시 중단.
static CALL_ABLATE: AtomicBool = AtomicBool::new(false);
static CALL_ABLATE_APPLIED: AtomicBool = AtomicBool::new(false);
static CALL_BLOCKED_A: AtomicU64 = AtomicU64::new(0);   // push A(0xb) 발화·차단 횟수
static CALL_BLOCKED_B: AtomicU64 = AtomicU64::new(0);   // push B(0xb) 발화·차단 횟수
const CALL_PUSH_A_RVA: usize = 0x2070ce9;  // mov byte[rax+rcx*8],0xb (push A) → 합류 0x2070d01
const CALL_PUSH_B_RVA: usize = 0x2071752;  // (push B) → 합류 0x207176c
const CALL_JOIN_A_RVA: usize = 0x2070d01;
const CALL_JOIN_B_RVA: usize = 0x207176c;
const CALL_ORIG_A: [u8; 14] = [0xC6,0x04,0xC8,0x0B, 0x88,0x5C,0xC8,0x01, 0x48,0xC7,0x44,0xC8,0x08,0x00];
const CALL_ORIG_B: [u8; 14] = [0xC6,0x04,0xC8,0x0B, 0x44,0x88,0x6C,0xC8,0x01, 0x48,0xC7,0x44,0xC8,0x08];
unsafe fn build_call_stub(counter_addr: usize, join_addr: usize) -> usize {
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 64, MEM_CR, RWX);
    if stub == 0 { return 0; }
    let mut s: Vec<u8> = Vec::new();
    s.push(0x51);                                          // push rcx
    s.extend_from_slice(&[0x48,0xb9]); s.extend_from_slice(&counter_addr.to_le_bytes());  // movabs rcx, &counter
    s.extend_from_slice(&[0xf0,0x48,0xff,0x01]);           // lock inc qword [rcx]
    s.push(0x59);                                          // pop rcx
    s.extend_from_slice(&[0xff,0x25,0x00,0x00,0x00,0x00]); // jmp qword [rip+0]
    s.extend_from_slice(&join_addr.to_le_bytes());         // 합류점 절대주소
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    stub
}
unsafe fn patch14(addr: usize, bytes: &[u8; 14]) {
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(addr, 14, RWX, &mut old) == 0 { return; }
    core::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, 14);
    VirtualProtect(addr, 14, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, 14);
}
unsafe fn apply_call_ablate() {
    let want = CALL_ABLATE.load(Ordering::Relaxed);
    if want == CALL_ABLATE_APPLIED.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 { return; }
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }  // 게임 안정 후에만
    let a_addr = base + CALL_PUSH_A_RVA;
    let b_addr = base + CALL_PUSH_B_RVA;
    if !readable(a_addr, 14) || !readable(b_addr, 14) { return; }
    // 안전검증: 켤때 원본(C6 04..) / 끌때 패치(FF 25..) 상태인지 확인 후에만 적용 (RVA 오류 크래시 방지)
    let (a0, a1, b0, b1) = (rd_u8(a_addr), rd_u8(a_addr + 1), rd_u8(b_addr), rd_u8(b_addr + 1));
    let chk = if want { a0==0xC6 && a1==0x04 && b0==0xC6 && b1==0x04 } else { a0==0xFF && a1==0x25 && b0==0xFF && b1==0x25 };
    if !chk { write_named("call_ablate.txt", &format!("ABORT A={:02x}{:02x} B={:02x}{:02x} want={} (RVA mismatch?)\n", a0,a1,b0,b1,want)); return; }
    if want {
        let sa = build_call_stub(&CALL_BLOCKED_A as *const _ as usize, base + CALL_JOIN_A_RVA);
        let sb = build_call_stub(&CALL_BLOCKED_B as *const _ as usize, base + CALL_JOIN_B_RVA);
        if sa == 0 || sb == 0 { write_named("call_ablate.txt", "ABORT stub alloc fail\n"); return; }
        let mut pa = [0u8; 14]; pa[0]=0xff; pa[1]=0x25; pa[6..14].copy_from_slice(&sa.to_le_bytes());
        let mut pb = [0u8; 14]; pb[0]=0xff; pb[1]=0x25; pb[6..14].copy_from_slice(&sb.to_le_bytes());
        patch14(a_addr, &pa); patch14(b_addr, &pb);
        write_named("call_ablate.txt", &format!("call_ablate=ON (콜차단+카운트) @ {:#x}/{:#x} stubs {:#x}/{:#x}\n", a_addr, b_addr, sa, sb));
    } else {
        patch14(a_addr, &CALL_ORIG_A); patch14(b_addr, &CALL_ORIG_B);
        write_named("call_ablate.txt", &format!("call_ablate=OFF (원본복원) @ {:#x}/{:#x}\n", a_addr, b_addr));
    }
    CALL_ABLATE_APPLIED.store(want, Ordering::Relaxed);
}

unsafe fn install_detour(rva: usize, orig_len: usize, cap_fn: usize) -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, orig_len+4) { return Err("fn unreadable"); }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);        // mov r10, rsp
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]); // push rcx rdx r8 r9 r10 r11
    s.extend_from_slice(&[0x48,0x89,0xe1]);        // mov rcx, rsp (saved)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);        // mov rdx, r10 (entry_rsp)
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);   // sub rsp,0x28
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff,0xd0]);             // call rax
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);   // add rsp,0x28
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]); // pop r11 r10 r9 r8 rdx rcx
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes());
    s.extend_from_slice(&[0xff,0xe0]);             // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(())
}

// ★스칼라(rax) 반환 replace detour: cap_fn(saved,entry_rsp)->i64. 반환값=RAX_SENT(=i64::MIN)면 passthrough(원본실행),
//   그 외면 그 값을 rax로 caller에 반환(원본 skip). install_detour와 saved레이아웃 동일(push rcx/rdx/r8/r9/r10/r11).
//   ★rng_repl off시 cap_fn이 항상 SENT 반환 → install_detour와 동일 동작(안전).
const RAX_SENT: i64 = i64::MIN;
unsafe fn install_replace_detour_rax(rva: usize, orig_len: usize, cap_fn: usize) -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, orig_len+4) { return Err("fn unreadable"); }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);        // mov r10, rsp
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]); // push rcx rdx r8 r9 r10 r11
    s.extend_from_slice(&[0x48,0x89,0xe1]);        // mov rcx, rsp (saved)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);        // mov rdx, r10 (entry_rsp)
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);   // sub rsp,0x28
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes()); // movabs rax, cap_fn
    s.extend_from_slice(&[0xff,0xd0]);             // call rax  (→ rax: RAX_SENT=passthrough / else=반환값)
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);   // add rsp,0x28
    s.extend_from_slice(&[0x49,0xbb]); s.extend_from_slice(&(RAX_SENT as u64).to_le_bytes()); // movabs r11, sentinel
    s.extend_from_slice(&[0x4c,0x39,0xd8]);        // cmp rax, r11
    s.extend_from_slice(&[0x74,0x0b]);             // je +0x0b → fallthrough (HANDLED 11B 스킵)
    // ── HANDLED (11B): pop regs(rax=반환값 보존) → ret (caller로 복귀) ──
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]); // pop r11 r10 r9 r8 rdx rcx (10B)
    s.extend_from_slice(&[0xc3]);                  // ret (1B)
    // ── FALLTHROUGH: regs복원 → 원본 prologue → fn+len ──
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]); // pop r11 r10 r9 r8 rdx rcx
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes()); // movabs rax, fn+len
    s.extend_from_slice(&[0xff,0xe0]);             // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(())
}

// ── 페이즈게이트 전용 디투어: 휘발성 + rbx/rdi/rsi(=A/B/C) 저장. saved 레이아웃:
//    +0=rcx +8=rdx +0x10=r8 +0x18=r9 +0x20=r10(entry_rsp) +0x28=r11 +0x30=rbx +0x38=rdi +0x40=rsi ──
unsafe fn install_detour_pg(rva: usize, orig_len: usize, cap_fn: usize) -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, orig_len+4) { return Err("fn unreadable"); }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);   // mov r10, rsp (entry_rsp)
    // push rsi rdi rbx r11 r10 r9 r8 rdx rcx  (rcx 마지막=saved+0)
    s.extend_from_slice(&[0x56,0x57,0x53,0x41,0x53,0x41,0x52,0x41,0x51,0x41,0x50,0x52,0x51]);
    s.extend_from_slice(&[0x48,0x89,0xe1]);   // mov rcx, rsp (saved)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);   // mov rdx, r10 (entry_rsp)
    s.extend_from_slice(&[0x48,0x83,0xec,0x20]);   // sub rsp,0x20 (shadow, 16-align)
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff,0xd0]);        // call rax
    s.extend_from_slice(&[0x48,0x83,0xc4,0x20]);   // add rsp,0x20
    // pop rcx rdx r8 r9 r10 r11 rbx rdi rsi
    s.extend_from_slice(&[0x59,0x5a,0x41,0x58,0x41,0x59,0x41,0x5a,0x41,0x5b,0x5b,0x5f,0x5e]);
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes());
    s.extend_from_slice(&[0xff,0xe0]);        // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(())
}
// ── 영역 D 캡처 전용 디투어(mid-func 0x20e42a3): rbp/r12/r13 캡처 + ★강제 16정렬보정(and rsp,-16) ──
//   mid-func는 rsp 16-정렬(함수진입 16k-8과 다름) → push개수만으론 call서 8B 어긋남 → cap_fn movaps 폴트(genbuild_body_D.md "유일 crash지점").
//   해결: 전 reg save 후 rbx에 rsp백업 → and rsp,-16(어느쪽 정렬이든 robust) → call → mov rsp,rbx 복원(rbx=non-vol, cap_fn 보존).
//   saved 레이아웃: +0=rcx +8=rdx +0x10=r8 +0x18=r9 +0x20=r10(entry_rsp) +0x28=r11 +0x30=rbx +0x38=rbp +0x40=r12 +0x48=r13.
//   ★r14는 save안함(cap_fn=Win64가 non-vol 보존) → 원본 shr r14,2 정상. rbp/rbx도 pop으로 복원 후 원본 mov rcx,[rbp+0x108] 정상.
unsafe fn install_detour_d(rva: usize, orig_len: usize, cap_fn: usize) -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, orig_len+4) { return Err("fn unreadable"); }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);   // mov r10, rsp (entry_rsp)
    // push rax r13 r12 rbp rbx r11 r10 r9 r8 rdx rcx  (rcx 마지막=saved+0 ; rax=saved+0x50 ★에필로그 hook서 rax=반환값이라 보존 필수)
    s.extend_from_slice(&[0x50, 0x41,0x55, 0x41,0x54, 0x55, 0x53, 0x41,0x53, 0x41,0x52, 0x41,0x51, 0x41,0x50, 0x52, 0x51]);
    s.extend_from_slice(&[0x48,0x89,0xe1]);   // mov rcx, rsp (saved=arg1)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);   // mov rdx, r10 (entry_rsp=arg2)
    s.extend_from_slice(&[0x48,0x89,0xe3]);   // mov rbx, rsp (정렬복원 홀더; cap_fn이 rbx 보존)
    s.extend_from_slice(&[0x48,0x83,0xe4,0xf0]); // and rsp,-16 (강제 16정렬 — crash 방지 핵심)
    s.extend_from_slice(&[0x48,0x83,0xec,0x20]); // sub rsp,0x20 (shadow; 0x20=16정렬 유지)
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff,0xd0]);        // call rax
    s.extend_from_slice(&[0x48,0x89,0xdc]);   // mov rsp, rbx (정렬보정 되돌림)
    // pop rcx rdx r8 r9 r10 r11 rbx rbp r12 r13 rax  (rax 마지막=복원, 반환값 보존)
    s.extend_from_slice(&[0x59, 0x5a, 0x41,0x58, 0x41,0x59, 0x41,0x5a, 0x41,0x5b, 0x5b, 0x5d, 0x41,0x5c, 0x41,0x5d, 0x58]);
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    // ★rax-보존 점프백: movabs rax는 반환값을 깸 → jmp qword [rip+0] + 8B 타깃(레지스터 무클로버). 에필로그 hook(rax=retval) 충실성 필수.
    s.extend_from_slice(&[0xff,0x25,0x00,0x00,0x00,0x00]); s.extend_from_slice(&ret_addr.to_le_bytes());  // jmp [rip+0]; .quad fn+orig_len
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(())
}
// ── ★영역 D 진짜 skip 디투어(mid-func 0x42a3): cap_fn(saved,entry_rsp)->i64. SENT=passthrough(원본 region D 실행, fn+orig_len 복귀) / else=out ptr=HANDLED.
//    HANDLED: rax=cap반환(out=sret 반환값) 유지하고 funnel(funnel_rva)로 jmp = 게임 region D 미실행. region D=RNG-free라 skip 무desync.
//    saved 레이아웃 = install_detour_d 동일(+0x38=rbp/+0x40=r12/+0x48=r13). 정렬보정 and rsp,-16. rax: passthrough=복원 / handled=cap반환(out) 유지.
unsafe fn install_detour_d_skip(rva: usize, orig_len: usize, cap_fn: usize, funnel_rva: usize) -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, orig_len+4) { return Err("fn unreadable"); }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;          // passthrough 복귀 (0x42b2)
    let funnel_addr = mbase + funnel_rva;        // handled jump (0x20e4a1a)
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);   // mov r10, rsp (entry_rsp)
    // push rax r13 r12 rbp rbx r11 r10 r9 r8 rdx rcx  (rax=highest/마지막pop ; rcx=saved+0)
    s.extend_from_slice(&[0x50, 0x41,0x55, 0x41,0x54, 0x55, 0x53, 0x41,0x53, 0x41,0x52, 0x41,0x51, 0x41,0x50, 0x52, 0x51]);
    s.extend_from_slice(&[0x48,0x89,0xe1]);   // mov rcx, rsp (saved=arg1)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);   // mov rdx, r10 (entry_rsp=arg2)
    s.extend_from_slice(&[0x48,0x89,0xe3]);   // mov rbx, rsp (정렬복원 홀더)
    s.extend_from_slice(&[0x48,0x83,0xe4,0xf0]); // and rsp,-16
    s.extend_from_slice(&[0x48,0x83,0xec,0x20]); // sub rsp,0x20
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff,0xd0]);        // call rax  (rax=SENT / out)
    s.extend_from_slice(&[0x48,0x89,0xdc]);   // mov rsp, rbx
    s.extend_from_slice(&[0x49,0xbb]); s.extend_from_slice(&(RAX_SENT as u64).to_le_bytes()); // movabs r11, SENT
    s.extend_from_slice(&[0x4c,0x39,0xd8]);   // cmp rax, r11
    s.extend_from_slice(&[0x74, 0x22]);       // je +0x22 → PASSTHROUGH (HANDLED 블록 34B 스킵)
    // ── HANDLED (34B): rax=out(cap반환) 유지. pop rcx..r13(10, rax제외) → add rsp,8(rax슬롯 폐기) → jmp funnel ──
    s.extend_from_slice(&[0x59, 0x5a, 0x41,0x58, 0x41,0x59, 0x41,0x5a, 0x41,0x5b, 0x5b, 0x5d, 0x41,0x5c, 0x41,0x5d]); // pop rcx rdx r8 r9 r10 r11 rbx rbp r12 r13 (16B)
    s.extend_from_slice(&[0x48,0x83,0xc4,0x08]); // add rsp,8 (saved rax슬롯 폐기, rax=out 유지) (4B)
    s.extend_from_slice(&[0xff,0x25,0x00,0x00,0x00,0x00]); s.extend_from_slice(&funnel_addr.to_le_bytes()); // jmp [rip+0]; .quad funnel (14B)
    // ── PASSTHROUGH: pop rcx..r13 rax(11, rax복원) → 원본 15B → jmp fn+orig_len ──
    s.extend_from_slice(&[0x59, 0x5a, 0x41,0x58, 0x41,0x59, 0x41,0x5a, 0x41,0x5b, 0x5b, 0x5d, 0x41,0x5c, 0x41,0x5d, 0x58]); // pop ...r13 rax (17B)
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0xff,0x25,0x00,0x00,0x00,0x00]); s.extend_from_slice(&ret_addr.to_le_bytes()); // jmp [rip+0]; .quad fn+orig_len
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(())
}
// ── 조건부 디투어: cap_fn 반환값 rax==0(handled)→*p1 이미씀, rax=p1로 caller에 즉시 RET(원본 스킵).
//    rax==1(fall-through)→원본 prologue 실행 후 fn+12로(원본 정상실행). 출력 sret=rcx=param_1. ──
unsafe fn install_replace_detour(rva: usize, orig_len: usize, cap_fn: usize) -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, orig_len+4) { return Err("fn unreadable"); }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);        // mov r10, rsp  (r10=ESP0=retaddr슬롯)
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]); // push rcx rdx r8 r9 r10 r11
    s.extend_from_slice(&[0x48,0x89,0xe1]);        // mov rcx, rsp (saved)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);        // mov rdx, r10 (entry_rsp)
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);   // sub rsp,0x28
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes()); // movabs rax, cap_fn
    s.extend_from_slice(&[0xff,0xd0]);             // call rax  (→ rax: 0=handled / 1=fallthrough)
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);   // add rsp,0x28
    s.extend_from_slice(&[0x48,0x85,0xc0]);        // test rax, rax
    s.extend_from_slice(&[0x75,0x0e]);             // jnz +0x0e → fallthrough (handled블록 14B 스킵)
    // ── HANDLED (14B): regs복원 → rax=rcx(=p1) → ret (caller로 복귀, *p1 이미 씀) ──
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]); // pop r11 r10 r9 r8 rdx rcx (rsp=ESP0)
    s.extend_from_slice(&[0x48,0x89,0xc8]);        // mov rax, rcx  (반환값 = param_1 sret)
    s.extend_from_slice(&[0xc3]);                  // ret  ([ESP0]=caller retaddr pop, 복귀)
    // ── FALLTHROUGH: regs복원 → 원본 prologue → fn+12 ──
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]); // pop r11 r10 r9 r8 rdx rcx (rsp=ESP0)
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes()); // movabs rax, fn+12
    s.extend_from_slice(&[0xff,0xe0]);             // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(())
}

// ── facet#2 이동 override 핸들러: driver memcpy 직전 Input(src=rdx) 가로채기 ──
// tag@+0==1(Move)이면 x@+8/y@+0x10를 cfg값으로 덮어씀. tag별 카운트(훅 발동확인).
unsafe extern "C" fn move_override(src: usize) {
    if !ptr_ok(src) || !readable(src, 0x90) { return; }
    let tag = std::ptr::read_unaligned(src as *const i64);
    let b = if (0..16).contains(&tag) { tag as usize } else { 15 };
    TAG_COUNTS[b].fetch_add(1, Ordering::Relaxed);
    // tag별 첫 샘플: struct 머리 9 qword 덤프 (좌표 위치 찾기)
    if TAG_SAMP[b][0].load(Ordering::Relaxed) == i64::MIN {
        for k in 0..18usize { TAG_SAMP[b][k].store(std::ptr::read_unaligned((src + k*8) as *const i64), Ordering::Relaxed); }
    }
    if MOVE_ON.load(Ordering::Relaxed) && tag == MOVE_TAG.load(Ordering::Relaxed) {
        let off = MOVE_OFF.load(Ordering::Relaxed) as usize;
        if off + 16 <= 0x90 {
            std::ptr::write_unaligned((src + off) as *mut i64, MOVE_X.load(Ordering::Relaxed));
            std::ptr::write_unaligned((src + off + 8) as *mut i64, MOVE_Y.load(Ordering::Relaxed));
            MOVE_HANDLED.fetch_add(1, Ordering::Relaxed);
        }
    }
}
// rel32 도달범위(±2GB) 내 target 근처에 RWX 할당 (CALL rel32 재지정용)
unsafe fn alloc_near(target: usize, size: usize) -> usize {
    let base = target & !0xffff;
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let mut step = 1usize;
    while step < 0x7000 {  // ~1.75GB 까지 64KB 스텝
        for dir in [1isize, -1isize] {
            let addr = base.wrapping_add((dir * (step as isize) * 0x10000) as usize);
            if addr >= 0x10000 {
                let p = VirtualAlloc(addr, size, MEM_CR, RWX);
                if p != 0 { return p; }
            }
        }
        step += 1;
    }
    0
}
// ── driver memcpy(0x1d4ec17) 호출지점 패치: rel32만 우리 스텁으로(원자적 4B). 스텁=override 후 jmp memcpy. ──
unsafe fn install_move_hook() -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let call_site = mbase + 0x1d4ec17;       // CALL 0x14286ba4d (memcpy)
    if !readable(call_site, 5) { return Err("call_site unreadable"); }
    if std::ptr::read_unaligned(call_site as *const u8) != 0xE8 { return Err("not a CALL(E8)"); }
    let next = call_site + 5;                 // 0x141d4ec1c (return target)
    let rel0 = std::ptr::read_unaligned((call_site + 1) as *const i32);
    let memcpy = (next as i64 + rel0 as i64) as usize;  // 원래 memcpy 주소
    if !ptr_ok(memcpy) { return Err("memcpy resolve"); }
    let stub = alloc_near(next, 128);
    if stub == 0 { return Err("alloc_near"); }
    let new_rel = stub as i64 - next as i64;
    if new_rel > 0x7f00_0000 || new_rel < -0x7f00_0000 { return Err("stub too far"); }
    // 스텁: rcx=dst,rdx=src,r8=size. override(src) 호출 후 memcpy로 tail-jmp(→ret시 ec1c복귀).
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x51, 0x52, 0x41, 0x50]);            // push rcx; push rdx; push r8
    s.extend_from_slice(&[0x48, 0x89, 0xD1]);                 // mov rcx, rdx (arg=src Input)
    s.extend_from_slice(&[0x48, 0x83, 0xEC, 0x20]);           // sub rsp,0x20 (shadow)
    s.extend_from_slice(&[0x48, 0xB8]); s.extend_from_slice(&(move_override as usize).to_le_bytes());
    s.extend_from_slice(&[0xFF, 0xD0]);                       // call rax (move_override)
    s.extend_from_slice(&[0x48, 0x83, 0xC4, 0x20]);           // add rsp,0x20
    s.extend_from_slice(&[0x41, 0x58, 0x5A, 0x59]);           // pop r8; pop rdx; pop rcx
    s.extend_from_slice(&[0x48, 0xB8]); s.extend_from_slice(&memcpy.to_le_bytes());
    s.extend_from_slice(&[0xFF, 0xE0]);                       // jmp rax (memcpy → ret to ec1c)
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());
    // rel32(call_site+1) 패치: 4-정렬이면 원자적. RWX→write→복원.
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(call_site, 5, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    std::ptr::write_unaligned((call_site + 1) as *mut i32, new_rel as i32);
    VirtualProtect(call_site, 5, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), call_site, 5);
    Ok(())
}

// ── facet#2 진짜 이동훅: driver의 FUN_141917430(이동좌표 최종화) 호출지점(0x1d4fecf)을 POST-래퍼로 ──
// 래퍼 = 원본 FUN_141917430을 (인자 그대로 복제) 호출 → 직후 outptr(=rcx, [RBP+0xd0])이 최종 Move{tag,x,y}
//        → move_override(outptr) (9 qword 덤프 + cfg move=1이면 x/y 강제). 호출지점 한정이라 다른 caller엔 무영향.
unsafe fn install_move_post_hook() -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let call_site = mbase + RVA_F2_BUILD_CALL;   // CALL FUN_141917430
    if !readable(call_site, 5) { return Err("call_site unreadable"); }
    if std::ptr::read_unaligned(call_site as *const u8) != 0xE8 { return Err("not a CALL(E8)"); }
    let next = call_site + 5;                      // 0x141d4fed4 (driver 복귀지점)
    let rel0 = std::ptr::read_unaligned((call_site + 1) as *const i32);
    let target = (next as i64 + rel0 as i64) as usize;  // 실제 FUN_141917430 주소
    if target != mbase + RVA_GENERIC_BUILD { return Err("target mismatch (not generic_build)"); }
    let stub = alloc_near(next, 160);
    if stub == 0 { return Err("alloc_near"); }
    let new_rel = stub as i64 - next as i64;
    if new_rel > 0x7f00_0000 || new_rel < -0x7f00_0000 { return Err("stub too far"); }
    // 래퍼 스텁. 진입 rsp=S(%16==8), [S]=복귀주소, rcx=outptr, rdx/r8/r9=arg2~4, stack arg5~8=[S+0x28..0x40].
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x55]);                          // push rbp
    s.extend_from_slice(&[0x48, 0x89, 0xE5]);             // mov rbp, rsp        (rbp=S-8)
    s.extend_from_slice(&[0x53]);                          // push rbx
    s.extend_from_slice(&[0x48, 0x89, 0xCB]);             // mov rbx, rcx        (rbx=outptr, call 넘어 보존)
    s.extend_from_slice(&[0x48, 0x83, 0xEC, 0x48]);       // sub rsp, 0x48       (shadow0x20+arg0x20+align8)
    // stack arg5~8 복제: [rbp+0x30..0x48] → [rsp+0x20..0x38] (원래 호출이 넘기던 값 그대로)
    s.extend_from_slice(&[0x48, 0x8B, 0x45, 0x30, 0x48, 0x89, 0x44, 0x24, 0x20]); // mov rax,[rbp+0x30]; mov [rsp+0x20],rax
    s.extend_from_slice(&[0x48, 0x8B, 0x45, 0x38, 0x48, 0x89, 0x44, 0x24, 0x28]); // arg6
    s.extend_from_slice(&[0x48, 0x8B, 0x45, 0x40, 0x48, 0x89, 0x44, 0x24, 0x30]); // arg7
    s.extend_from_slice(&[0x48, 0x8B, 0x45, 0x48, 0x48, 0x89, 0x44, 0x24, 0x38]); // arg8
    // rcx/rdx/r8/r9 그대로(arg1~4). 원본 호출.
    s.extend_from_slice(&[0x48, 0xB8]); s.extend_from_slice(&target.to_le_bytes());  // movabs rax, FUN_141917430
    s.extend_from_slice(&[0xFF, 0xD0]);                   // call rax
    // 복귀: rax=리턴값(sret→outptr). move_override(outptr) 실행 (rax 보존).
    s.extend_from_slice(&[0x50]);                          // push rax           (리턴값 보존)
    s.extend_from_slice(&[0x48, 0x89, 0xD9]);             // mov rcx, rbx        (arg=outptr)
    s.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]);       // sub rsp, 0x28       (shadow+align)
    s.extend_from_slice(&[0x48, 0xB8]); s.extend_from_slice(&(move_override as usize).to_le_bytes()); // movabs rax,move_override
    s.extend_from_slice(&[0xFF, 0xD0]);                   // call rax
    s.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28]);       // add rsp, 0x28
    s.extend_from_slice(&[0x58]);                          // pop rax            (리턴값 복원)
    s.extend_from_slice(&[0x48, 0x8D, 0x65, 0xF8]);       // lea rsp, [rbp-8]    (저장된 rbx 위치)
    s.extend_from_slice(&[0x5B]);                          // pop rbx
    s.extend_from_slice(&[0x5D]);                          // pop rbp
    s.extend_from_slice(&[0xC3]);                          // ret                (→ 복귀주소 next)
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(call_site, 5, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    std::ptr::write_unaligned((call_site + 1) as *mut i32, new_rel as i32);
    VirtualProtect(call_site, 5, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), call_site, 5);
    Ok(())
}

// 광범위 커밋 dump: 매프레임 최종 Input(rdx) tag별 첫샘플 18 qword. (override 없음, 관측전용)
unsafe extern "C" fn commit_dump(src: usize) {
    if !ptr_ok(src) || !readable(src, 0x90) { return; }
    COMMIT_TOTAL.fetch_add(1, Ordering::Relaxed);
    let tag = std::ptr::read_unaligned(src as *const i64);
    let b = if (0..16).contains(&tag) { tag as usize } else { 15 };
    COMMIT_TAGCOUNT[b].fetch_add(1, Ordering::Relaxed);
    if COMMIT_SAMP[b][0].load(Ordering::Relaxed) == i64::MIN {
        for k in 0..18usize { COMMIT_SAMP[b][k].store(std::ptr::read_unaligned((src + k*8) as *const i64), Ordering::Relaxed); }
    }
}
// 페이즈 게이트 threshold 베이스(imm8) 패치. cfg engage_base>=0이면 적용(핫리로드). -1이면 원본 복원.
unsafe fn apply_engage_base() {
    let mbase = exe_base();
    if mbase == 0 { return; }
    let site = mbase + RVA_ENGAGE_GATE;
    if !readable(site, 3) { return; }
    // sanity: 83 C0 ?? (ADD EAX, imm8)
    if std::ptr::read_unaligned(site as *const u8) != 0x83 || std::ptr::read_unaligned((site+1) as *const u8) != 0xC0 { return; }
    let imm_site = site + 2;
    // 최초 1회 원본 백업
    if ENGAGE_ORIG.load(Ordering::Relaxed) < 0 {
        ENGAGE_ORIG.store(std::ptr::read_unaligned(imm_site as *const u8) as i64, Ordering::Relaxed);
    }
    let want = ENGAGE_BASE.load(Ordering::Relaxed);
    let new_imm: u8 = if want < 0 { ENGAGE_ORIG.load(Ordering::Relaxed) as u8 }  // -1=원본 복원
                      else { want.clamp(0, 127) as u8 };
    if std::ptr::read_unaligned(imm_site as *const u8) == new_imm { return; }  // 변화없으면 skip
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(imm_site, 1, RWX, &mut old) == 0 { return; }
    std::ptr::write_unaligned(imm_site as *mut u8, new_imm);
    VirtualProtect(imm_site, 1, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), imm_site, 1);
}
// ── 광범위 커밋 훅: CALL FUN_141a49fa0(0x1d5035d) rel32 재지정 → commit_dump(rdx) 후 jmp 원본 ──
unsafe fn install_commit_hook() -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let call_site = mbase + RVA_COMMIT_CALL;
    if !readable(call_site, 5) { return Err("call_site unreadable"); }
    if std::ptr::read_unaligned(call_site as *const u8) != 0xE8 { return Err("not a CALL(E8)"); }
    let next = call_site + 5;
    let rel0 = std::ptr::read_unaligned((call_site + 1) as *const i32);
    let target = (next as i64 + rel0 as i64) as usize;
    if target != mbase + RVA_COMMIT_FN { return Err("target mismatch (not commit fn)"); }
    let stub = alloc_near(next, 128);
    if stub == 0 { return Err("alloc_near"); }
    let new_rel = stub as i64 - next as i64;
    if new_rel > 0x7f00_0000 || new_rel < -0x7f00_0000 { return Err("stub too far"); }
    // 스텁: rcx=champ+0x590, rdx=&Input. commit_dump(rdx) 후 jmp FUN_141a49fa0(→ret시 0x141d50362 복귀).
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x51, 0x52, 0x41, 0x50, 0x41, 0x51]);  // push rcx; push rdx; push r8; push r9
    s.extend_from_slice(&[0x48, 0x89, 0xD1]);                    // mov rcx, rdx (arg=Input)
    s.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]);              // sub rsp,0x28 (shadow+align)
    s.extend_from_slice(&[0x48, 0xB8]); s.extend_from_slice(&(commit_dump as *const () as usize).to_le_bytes());
    s.extend_from_slice(&[0xFF, 0xD0]);                          // call rax
    s.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28]);              // add rsp,0x28
    s.extend_from_slice(&[0x41, 0x59, 0x41, 0x58, 0x5A, 0x59]);  // pop r9; pop r8; pop rdx; pop rcx
    s.extend_from_slice(&[0x48, 0xB8]); s.extend_from_slice(&target.to_le_bytes());  // movabs rax, FUN_141a49fa0
    s.extend_from_slice(&[0xFF, 0xE0]);                          // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(call_site, 5, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    std::ptr::write_unaligned((call_site + 1) as *mut i32, new_rel as i32);
    VirtualProtect(call_site, 5, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), call_site, 5);
    Ok(())
}

// ★facet#5 역할 교전임계값 스케일 패치(cfg engage_thr_mult). mult=100→원본 복원. 각 immediate low byte 1개.
unsafe fn apply_engage_thr_mult() {
    let mult = ENGAGE_THR_MULT.load(Ordering::Relaxed);
    if mult < 0 { return; }
    let mbase = exe_base();
    if mbase == 0 { return; }
    // 오프셋 sanity: 각 site의 imm32 상위3바이트가 0이어야(작은값). 아니면 잘못된 오프셋→중단.
    for &(rva, _) in &ROLE_THR {
        let s = mbase + rva;
        if !readable(s, 4) { return; }
        if std::ptr::read_unaligned((s+1) as *const u8) != 0
            || std::ptr::read_unaligned((s+2) as *const u8) != 0
            || std::ptr::read_unaligned((s+3) as *const u8) != 0 { return; }
    }
    const RWX: u32 = 0x40;
    for &(rva, orig) in &ROLE_THR {
        let s = mbase + rva;
        let new = ((orig as i64) * mult / 100).clamp(0, 255) as u8;
        if std::ptr::read_unaligned(s as *const u8) == new { continue; }
        let mut old: u32 = 0;
        if VirtualProtect(s, 1, RWX, &mut old) == 0 { continue; }
        std::ptr::write_unaligned(s as *mut u8, new);
        VirtualProtect(s, 1, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), s, 1);
    }
}
// facet#5 셀렉터 신선포착: df0c10 출력버퍼(rcx=&local_228)에서 셀렉터 읽어 전역에 저장(역할기반 판정용).
unsafe extern "C" fn df0c10_post(out_ptr: usize) {
    if ptr_ok(out_ptr) && readable(out_ptr, 8) {
        SEL228_FRESH.store(std::ptr::read_unaligned(out_ptr as *const i64), Ordering::Relaxed);
    }
}
// ── df0c10 호출(0x1faa433) POST-래퍼: 원본 df0c10 호출(인자충실복제) 후 df0c10_post(rcx=&셀렉터) ──
unsafe fn install_df0c10_hook() -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let call_site = mbase + RVA_DF0C10_CALL;
    if !readable(call_site, 5) { return Err("call_site unreadable"); }
    if std::ptr::read_unaligned(call_site as *const u8) != 0xE8 { return Err("not a CALL(E8)"); }
    let next = call_site + 5;
    let rel0 = std::ptr::read_unaligned((call_site + 1) as *const i32);
    let target = (next as i64 + rel0 as i64) as usize;
    if target != mbase + RVA_DF0C10_FN { return Err("target mismatch (not df0c10)"); }
    let stub = alloc_near(next, 160);
    if stub == 0 { return Err("alloc_near"); }
    let new_rel = stub as i64 - next as i64;
    if new_rel > 0x7f00_0000 || new_rel < -0x7f00_0000 { return Err("stub too far"); }
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x55]);                          // push rbp
    s.extend_from_slice(&[0x48, 0x89, 0xE5]);             // mov rbp, rsp
    s.extend_from_slice(&[0x53]);                          // push rbx
    s.extend_from_slice(&[0x48, 0x89, 0xCB]);             // mov rbx, rcx (&local_228 보존)
    s.extend_from_slice(&[0x48, 0x83, 0xEC, 0x48]);       // sub rsp, 0x48
    s.extend_from_slice(&[0x48, 0x8B, 0x45, 0x30, 0x48, 0x89, 0x44, 0x24, 0x20]); // arg5
    s.extend_from_slice(&[0x48, 0x8B, 0x45, 0x38, 0x48, 0x89, 0x44, 0x24, 0x28]); // arg6
    s.extend_from_slice(&[0x48, 0x8B, 0x45, 0x40, 0x48, 0x89, 0x44, 0x24, 0x30]); // arg7
    s.extend_from_slice(&[0x48, 0x8B, 0x45, 0x48, 0x48, 0x89, 0x44, 0x24, 0x38]); // arg8(미사용, 무해)
    s.extend_from_slice(&[0x48, 0xB8]); s.extend_from_slice(&target.to_le_bytes());  // movabs rax, df0c10
    s.extend_from_slice(&[0xFF, 0xD0]);                   // call rax
    s.extend_from_slice(&[0x50]);                          // push rax (리턴값 보존)
    s.extend_from_slice(&[0x48, 0x89, 0xD9]);             // mov rcx, rbx (&local_228)
    s.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]);       // sub rsp, 0x28
    s.extend_from_slice(&[0x48, 0xB8]); s.extend_from_slice(&(df0c10_post as *const () as usize).to_le_bytes());
    s.extend_from_slice(&[0xFF, 0xD0]);                   // call df0c10_post
    s.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28]);       // add rsp, 0x28
    s.extend_from_slice(&[0x58]);                          // pop rax
    s.extend_from_slice(&[0x48, 0x8D, 0x65, 0xF8]);       // lea rsp, [rbp-8]
    s.extend_from_slice(&[0x5B]);                          // pop rbx
    s.extend_from_slice(&[0x5D]);                          // pop rbp
    s.extend_from_slice(&[0xC3]);                          // ret (→ 0x1faa438)
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(call_site, 5, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    std::ptr::write_unaligned((call_site + 1) as *mut i32, new_rel as i32);
    VirtualProtect(call_site, 5, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), call_site, 5);
    Ok(())
}

// ══ dd7700 callee 재구현 (전부 PURE 포인터연산; 게임함수 호출 0) ══════════════
// slot+0x20: *(sim+0xed00) 스칼라(프론티어 진행도)
#[inline] unsafe fn dd7_slot20(sim: usize) -> i64 { rd_i64(sim + 0xed00).unwrap_or(0) }
// ★sim 헤더 캐시(호출당 1회): slot48/a8가 매 호출 재읽던 base/count 필드. 호출간 캐싱 아님 — judge 1회 호출 내에서만 재사용(동기 스코프, stale 없음).
//   b6e8=엔티티arena base, c6f0=그 limit, t700=핸들테이블, c708=그 limit, b808=레코드배열 base, c810=그 count.
#[derive(Clone, Copy, Default)]
struct SimHdr { b6e8: usize, c6f0: u64, t700: usize, c708: u64, b808: usize, c810: u64, tick: i64 }
#[inline] unsafe fn sim_hdr(sim: usize) -> SimHdr {
    SimHdr {
        b6e8: rd_u64(sim+0x6e8).unwrap_or(0) as usize,
        c6f0: rd_u64(sim+0x6f0).unwrap_or(0),
        t700: rd_u64(sim+0x700).unwrap_or(0) as usize,
        c708: rd_u64(sim+0x708).unwrap_or(0),
        b808: rd_u64(sim+0x808).unwrap_or(0) as usize,
        c810: rd_u64(sim+0x810).unwrap_or(0),
        tick: rd_i64(sim+0xed00).unwrap_or(0),   // ★레버4: slot_a8 캐시 무효화 키(현재틱)
    }
}
// ★레버4: dd7_slot_a8 프레임 캐시. id→record 매핑을 (base,cnt,tick)당 1회 빌드(O(cnt)) 후 O(1) 조회.
//   rayon 워커별 thread_local(경기간 격리). 틱/배열base/cnt 변경시 자동 재빌드 = stale 차단.
struct A8Cache { base: usize, cnt: u64, tick: i64, map: HashMap<u64, usize, FnvBuild> }
thread_local! {
    static A8_CACHE: RefCell<A8Cache> = RefCell::new(A8Cache { base: 0, cnt: 0, tick: -1, map: HashMap::with_hasher(FnvBuild) });
}
// slot+0x48(sim, sub<2, id): handle→0x6a8 rec, return *(rec+0x38+sub*0x18)==0. 본체=_h(미리읽은 헤더), sim버전=얇은 래퍼(필요 4필드만 읽음).
unsafe fn dd7_slot48_h(h: &SimHdr, sub: usize, id: u64) -> bool {
    if id < h.c708 {
        if rd_i32(h.t700 + (id as usize)*0x10).unwrap_or(0) == 1 {
            let u = rd_u64(h.t700 + (id as usize)*0x10 + 8).unwrap_or(0);
            if u < h.c6f0 && sub < 2 {
                let rec = (u as usize)*0x6a8 + h.b6e8;
                return rd_u64(rec + 0x38 + sub*0x18).unwrap_or(1) == 0;
            }
        }
    }
    false
}
#[inline] unsafe fn dd7_slot48(sim: usize, sub: usize, id: u64) -> bool {
    dd7_slot48_h(&SimHdr {
        c708: rd_u64(sim+0x708).unwrap_or(0),
        t700: rd_u64(sim+0x700).unwrap_or(0) as usize,
        c6f0: rd_u64(sim+0x6f0).unwrap_or(0),
        b6e8: rd_u64(sim+0x6e8).unwrap_or(0) as usize,
        ..Default::default()
    }, sub, id)
}
// slot+0xa8(sim, id): [sim+0x808]배열(cnt[sim+0x810],stride0x758) 선형탐색 → +0x740!=0 && +0x748==id 레코드/0
unsafe fn dd7_slot_a8_h(h: &SimHdr, id: u64) -> usize {
    let cnt = h.c810;
    if cnt == 0 || cnt > 4096 { return 0; }
    let base = h.b808;
    if !ptr_ok(base) { return 0; }
    // ★레버4: 프레임(base,cnt,tick) thread_local 해시 → O(1) 조회(기존 O(cnt) 선형탐색 제거 = dd7700/f22e80 핫스팟).
    //   비트동일: 맵은 +0x740!=0 레코드의 (id→rec)를 k오름차순 첫매칭(or_insert) = 선형탐색 첫매칭과 동일.
    //   base/cnt/tick 변경시 재빌드 = stale 차단. rd_u64=fault-safe(빌드 중 폴트=해당 레코드 skip, 선형과 동일).
    A8_CACHE.with(|c| {
        let mut cache = c.borrow_mut();
        if cache.base != base || cache.cnt != cnt || cache.tick != h.tick {
            cache.base = base; cache.cnt = cnt; cache.tick = h.tick;
            cache.map.clear();
            let mut k = 0u64;
            while k < cnt {
                let rec = base + (k as usize)*0x758;
                if rd_u64(rec+0x740).unwrap_or(0) != 0 {
                    let rid = rd_u64(rec+0x748).unwrap_or(0);
                    cache.map.entry(rid).or_insert(rec);
                }
                k += 1;
            }
        }
        cache.map.get(&id).copied().unwrap_or(0)
    })
}
#[inline] unsafe fn dd7_slot_a8(sim: usize, id: u64) -> usize {
    dd7_slot_a8_h(&SimHdr {
        c810: rd_u64(sim+0x810).unwrap_or(0),
        b808: rd_u64(sim+0x808).unwrap_or(0) as usize,
        tick: rd_i64(sim+0xed00).unwrap_or(0),   // ★레버4: 캐시 키
        ..Default::default()
    }, id)
}
// slot+0x128 = entity_handle_deref(sim, handle): 2단계(0x820→0x700) → 엔티티(0x6a8)/0
unsafe fn dd7_slot128(sim: usize, h: u64) -> usize {
    if h >= rd_u64(sim+0x828).unwrap_or(0) { return 0; }
    let t1 = rd_u64(sim+0x820).unwrap_or(0) as usize;
    if !ptr_ok(t1) || rd_i32(t1 + (h as usize)*0x10).unwrap_or(0) != 1 { return 0; }
    let u1 = rd_u64(t1 + (h as usize)*0x10 + 8).unwrap_or(0);
    if u1 >= rd_u64(sim+0x810).unwrap_or(0) { return 0; }
    let s808 = rd_u64(sim+0x808).unwrap_or(0) as usize;
    let lv2 = (u1 as usize)*0x758;
    if !ptr_ok(s808) || rd_u8(s808+0x740+lv2) == 0 { return 0; }
    let u2 = rd_u64(s808+lv2+0x748).unwrap_or(0);
    if u2 >= rd_u64(sim+0x708).unwrap_or(0) { return 0; }
    let s700 = rd_u64(sim+0x700).unwrap_or(0) as usize;
    let l3 = (u2 as usize)*0x10;
    if !ptr_ok(s700) || rd_i32(s700+l3).unwrap_or(0) != 1 { return 0; }
    let u3 = rd_u64(s700+l3+8).unwrap_or(0);
    if u3 >= rd_u64(sim+0x6f0).unwrap_or(0) { return 0; }
    (u3 as usize)*0x6a8 + rd_u64(sim+0x6e8).unwrap_or(0) as usize
}
// f6f720 mode=2 레인밴드 predicate(VOBJ, cx, cy). 맵경계 = *(VOBJ+8)+0x12b8(Xmax)/+0x12c0(Ymax).
unsafe fn dd7_f6f720_m2(vobj: usize, cx: u64, cy: u64) -> bool {
    let m = rd_u64(vobj+8).unwrap_or(0) as usize;
    if !ptr_ok(m) { return true; }
    let ymax = rd_u64(m+0x12c0).unwrap_or(0);
    let u5 = ymax.wrapping_sub(cy);                       // uVar5 = Ymax - cy
    let mut u6 = if cx < u5 { u5 } else { cx };           // max(cx,u5)
    if u6 <= 0x2ee00 { return true; }                    // <=192000 → true
    u6 = if u5 < cx { u5 } else { cx };                  // min(cx,u5)
    let xmax = rd_u64(m+0x12b8).unwrap_or(0);
    if u6 >= xmax.wrapping_sub(0x2ee00) { return true; } // >= Xmax-192000 → true
    let h1 = xmax.wrapping_sub(0xabe00) >> 1;            // (Xmax-704000)/2
    let cond1 = h1.wrapping_add(0xabe00) < cx || cx < h1;
    let h2 = ymax.wrapping_sub(0xabe00) >> 1;
    let cond2 = h2.wrapping_add(0xabe00) < cy || cy < h2;
    if cond1 || cond2 {
        if 0x2ee00 < cx {
            let u6b = cx.wrapping_sub(u5);
            if u5 <= cx { return 63999 < u6b; }
        }
        false
    } else {
        let u6c = u5.wrapping_sub(cx);
        let u4 = if cx <= u5 { u6c } else { cx.wrapping_sub(u5) };
        if (cx > u5 || u6c == 0) && 0x2ee00 < cx && u4 < 96000 {
            return 63999 < (0u64).wrapping_sub(u6c);
        }
        false
    }
}
// 제곱거리(u64; 좌표 0~960000, 합 ~1.8e12 → u64 안전)
#[inline] fn sqd(x1:u64,y1:u64,x2:u64,y2:u64) -> u64 {
    let dx = if x1>=x2 {x1-x2} else {x2-x1}; let dy = if y1>=y2 {y1-y2} else {y2-y1};
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}
// f22e80 COUNT 재현: OTHER측 5빌딩 순회, 빌딩마다 gen_range(p4,[wlo,whi]) draw + 필터 통과분 카운트.
// this=sim(L80[0]). 슬롯함수는 dd7_slotXX(sim) 재구현 사용. tgt=(tgtx,tgty)=target좌표, k=150000.
unsafe fn my_f22e80_count(rng: &mut RngSim, l80: usize, geo: usize, p5: usize, p7: usize,
                          sim: usize, wlo: u64, whi: u64, tgtx: u64, tgty: u64, k: u64) -> u64 {
    let side = rd_i64(p5+0x6a8).unwrap_or(-1);
    if side != 0 && side != 1 { return 0; }
    let (s, other) = (side as usize, (1 - side) as usize);
    let hdr = sim_hdr(sim);   // ★호이스트: 빌딩루프 slot48/a8 재사용(non-_h 매호출 sim_hdr 재읽기 제거)
    let mut count: u64 = 0;
    for u in 0..5usize {
        let bldg = rd_u64(l80 + 0x1e0 + (other*5 + u)*8).unwrap_or(0) as usize;
        if bldg == 0 { continue; }
        let mul = rd_u64(bldg+0x628).unwrap_or(0);
        let roll = match rng.gen_range(wlo, whi) { Some(v)=>v, None=>return count };  // 빌딩당 RNG draw
        let s20 = dd7_slot20(sim) as u64;
        let local_100 = (((roll.wrapping_mul(mul)) >> 3) as u128).wrapping_mul(0x20c49ba5e353f7cf) >> 64;
        let local_100 = local_100 as u64;
        let thra = rd_u64(p7+0x290+u*8).unwrap_or(0);
        let lvar20 = if thra <= s20 { s20 - thra } else { 0 };
        let ptx = rd_u64(p7+0x218+u*0x10).unwrap_or(0);
        let pty = rd_u64(p7+0x220+u*0x10).unwrap_or(0);
        let e = { let isq = isqrt_u64(sqd(ptx,pty,tgtx,tgty)); if k <= isq { isq - k } else { 0 } };
        let thrb = rd_u64(p7+0x268+u*8).unwrap_or(0);
        let id = rd_u64(bldg+0x5a8).unwrap_or(0);
        if thra > thrb {   // pre-test (thra<=thrb면 바로 accept test)
            let h = dd7_slot_a8_h(&hdr, id);
            if h == 0 {
                if !dd7_slot48_h(&hdr, s, id) { continue; }   // slot48==0 → reject
                // slot48!=0 → accept test로
            } else {
                let lane = rd_u64(other*0x228 + geo + 0x1e0 + (rd_i32(h+0x738).unwrap_or(0) as usize)*8).unwrap_or(0);
                if lane + 600 < s20 { continue; }   // 너무 멀면 reject
            }
        }
        // accept test: slot48!=0 OR lvar20*(local_100>>4) < e → reject
        if dd7_slot48_h(&hdr, s, id) || lvar20.wrapping_mul(local_100 >> 4) < e { continue; }
        // ACCEPT: push *(L80+0x1e0+other*0x28+u*8) if !=0
        if rd_u64(l80 + 0x1e0 + other*0x28 + u*8).unwrap_or(0) != 0 { count += 1; }
    }
    count
}
// ══════════════════════════════════════════════════════════════════════════
// disc 9/11 (epic/serpen poke) judge 재현 — 리프 헬퍼 (bottom-up). 추적=poke_judge_repro.md
// ══════════════════════════════════════════════════════════════════════════
// 앵커 테이블 DAT_1435d9260/9290 (mode 0..5). epic(m4)=(288000,288000), serpen(m5)=(672000,672000).
const POKE_ANC_A: [u64; 6] = [752000, 592000, 448000, 800000, 288000, 672000]; // DAT_1435d9260
const POKE_ANC_B: [u64; 6] = [496000, 176000, 256000, 351000, 288000, 672000]; // DAT_1435d9290
// fe2b10/cc0 앵커: uVar3=anchor_x(selfx와 짝), uVar2=anchor_y. side!=0→(A,B), side==0→(B,A) swap.
#[inline] fn poke_anchor(mode: u8, side: i64) -> (u64, u64) {
    let m = (mode as usize).min(5);
    if side == 0 { (POKE_ANC_B[m], POKE_ANC_A[m]) } else { (POKE_ANC_A[m], POKE_ANC_B[m]) }
}
// FUN_141db8960 레인밴드 predicate. mode0(serpen)=아래, mode2(epic)=기존 dd7_f6f720_m2.
unsafe fn poke_f6f720_m0(node: usize, x: u64, y: u64) -> bool {
    let m = rd_u64(node + 8).unwrap_or(0) as usize;
    if !ptr_ok(m) { return true; }
    let ymax = rd_u64(m + 0x12c0).unwrap_or(0);
    let xmax = rd_u64(m + 0x12b8).unwrap_or(0);
    let edge = tune("pf_edge_margin", 0x2ee00) as u64;   // ★튜닝: 맵 가장자리 마진 거리
    let band = tune("pf_center_band", 0xabe00) as u64;   // ★튜닝: 중앙대각 밴드폭
    let u5 = ymax.wrapping_sub(y);
    let u6 = if x < u5 { u5 } else { x };          // max(x, u5)
    if u6 <= edge { return true; }
    let u6m = if u5 < x { u5 } else { x };          // min(x, u5)
    if u6m >= xmax.wrapping_sub(edge) { return true; }
    let h1 = xmax.wrapping_sub(band) >> 1;
    let cond_a = h1.wrapping_add(band) < x || x < h1;
    let h2 = ymax.wrapping_sub(band) >> 1;
    let cond_b = h2.wrapping_add(band) < y || y < h2;
    let uvar6 = u5.wrapping_sub(x);                 // u5 - x
    let mut uvar4: u64;
    if cond_a || cond_b {
        if u5 < edge + 1 || u5 < x { return false; }
        uvar4 = x.wrapping_sub(u5);                 // x - u5
    } else {
        uvar4 = if x <= u5 { uvar6 } else { x.wrapping_sub(u5) };
        if u5 < edge + 1 { return false; }
        if u5 < x { return false; }
        if (tune("pf_diag_far", 95999) as u64) < uvar4 { return false; }   // ★튜닝: 대각밴드 원거리 컷
        uvar4 = (0u64).wrapping_sub(uvar6);         // -(u5-x)
    }
    if x < u5 { uvar4 = uvar6; }
    (tune("pf_diag_near", 63999) as u64) < uvar4   // ★튜닝: 대각밴드 근거리 컷
}
// FUN_141db8960 mode1(밴드): dy=mapY-y. NOT-qual = max(x,dy)>0x2ee00 && min(x,dy)<mapX-0x2ee00 && |dy-x|>=64000.
unsafe fn poke_f6f720_m1(node: usize, x: u64, y: u64) -> bool {
    let m = rd_u64(node + 8).unwrap_or(0) as usize;
    if !ptr_ok(m) { return true; }
    let ymax = rd_u64(m + 0x12c0).unwrap_or(0);
    let xmax = rd_u64(m + 0x12b8).unwrap_or(0);
    let edge = tune("pf_edge_margin", 0x2ee00) as u64;   // ★튜닝: 맵 가장자리 마진 거리
    let dy = ymax.wrapping_sub(y);
    let big = if x < dy { dy } else { x };
    if big <= edge { return true; }
    let small = if dy < x { dy } else { x };
    if small >= xmax.wrapping_sub(edge) { return true; }
    let d = if dy < x { x.wrapping_sub(dy) } else { dy.wrapping_sub(x) };
    d < tune("pf_band_width", 64000) as u64   // ★튜닝: 밴드폭(대각 |dy-x| 컷)
}
#[inline] unsafe fn poke_f6f720(node: usize, x: u64, y: u64, mode: u8) -> bool {
    match mode { 0 => poke_f6f720_m0(node, x, y), 1 => poke_f6f720_m1(node, x, y), 2 => dd7_f6f720_m2(node, x, y), _ => true }
}
// FUN_141fe2770: side 5슬롯서 HP%>=hpthr & dist²<=R² 아군 카운트.
unsafe fn poke_count_allies(p5: usize, p6: usize, ax: u64, ay: u64, r: u64, hpthr: u64) -> u64 {
    let side = rd_i64(p5 + 0x6a8).unwrap_or(-1);
    if side < 0 || side > 1 { return 0; }
    let base = rd_u64(p6).unwrap_or(0) as usize;
    if !ptr_ok(base) { return 0; }
    let r2 = r.wrapping_mul(r);
    let mut count = 0u64;
    for k in 0..5usize {
        let ent = rd_u64(base + 0x1e0 + (side as usize) * 0x28 + k * 8).unwrap_or(0) as usize;
        if ent == 0 { continue; }   // ★readable VQ제거(본문 rd_* fault-safe=valid sim 비트동일, poke 핫루프)
        let maxhp = rd_u64(ent + 0x610).unwrap_or(0);
        if maxhp == 0 { continue; }
        let hp = rd_u64(ent + 0x658).unwrap_or(0).wrapping_mul(100) / maxhp;
        if hp >= hpthr && sqd(rd_u64(ent + 0x648).unwrap_or(0), rd_u64(ent + 0x650).unwrap_or(0), ax, ay) <= r2 {
            count += 1;
        }
    }
    count
}
// FUN_141fe3180: OTHER 5슬롯서 HP%>=hpthr & dist²<=R² & vt필터(slot48||(slot_a8&&slot20<=lane+0x78)) 적 카운트.
unsafe fn poke_count_enemies(p5: usize, p6: usize, ax: u64, ay: u64, r: u64, hpthr: u64) -> u64 {
    let side = rd_i64(p5 + 0x6a8).unwrap_or(-1);
    if side < 0 || side > 1 { return 0; }
    let (s, other) = (side as usize, 1 - side as usize);
    let base = rd_u64(p6).unwrap_or(0) as usize;
    let geom = rd_u64(p6 + 0x10).unwrap_or(0) as usize;
    if !ptr_ok(base) || !ptr_ok(geom) { return 0; }
    let sim = rd_u64(base).unwrap_or(0) as usize;
    if !ptr_ok(sim) { return 0; }
    let r2 = r.wrapping_mul(r);
    let mut count = 0u64;
    for k in 0..5usize {
        let ent = rd_u64(base + 0x1e0 + other * 0x28 + k * 8).unwrap_or(0) as usize;
        if ent == 0 { continue; }   // ★readable VQ제거(본문 rd_* fault-safe=valid sim 비트동일, poke 핫루프)
        let maxhp = rd_u64(ent + 0x610).unwrap_or(0);
        if maxhp == 0 { continue; }
        let hp = rd_u64(ent + 0x658).unwrap_or(0).wrapping_mul(100) / maxhp;
        if hp < hpthr { continue; }
        if sqd(rd_u64(ent + 0x648).unwrap_or(0), rd_u64(ent + 0x650).unwrap_or(0), ax, ay) > r2 { continue; }
        let id = rd_u64(ent + 0x5a8).unwrap_or(0);
        if dd7_slot48(sim, s, id) { count += 1; continue; }   // cVar10 != 0
        let ha8 = dd7_slot_a8(sim, id);
        if ha8 == 0 { continue; }
        let lane = rd_i64(other * 0x228 + geom + 0x1e0 + (rd_i32(ha8 + 0x738).unwrap_or(0) as usize) * 8).unwrap_or(0);
        if dd7_slot20(sim) <= lane + 0x78 { count += 1; }
    }
    count
}
// FUN_141fe2cc0: 앵커서 먼 아군 존재 + zone진척 임계. poke 예측자 보조. nonzero→true.
unsafe fn poke_fe2cc0(p5: usize, p6: usize, mode: u8) -> bool {
    let bvar14: i32 = if mode == 4 { 2 } else if mode == 5 { 0 } else { -1 };
    let vobj = rd_u64(p6 + 8).unwrap_or(0) as usize;
    if !ptr_ok(vobj) { return false; }
    if bvar14 == 0 {
        if rd_u8(vobj + 0x28).wrapping_sub(4) > 0xfc { return false; }   // vobj+0x28 byte ∈ {1,2,3} → 0
    } else if bvar14 != 2 { return false; }
    let side = rd_i64(p5 + 0x6a8).unwrap_or(-1);
    if side < 0 || side > 1 { return false; }
    let s = side as usize;
    let base = rd_u64(p6).unwrap_or(0) as usize;
    let geom = rd_u64(p6 + 0x10).unwrap_or(0) as usize;
    if !ptr_ok(base) || !ptr_ok(geom) { return false; }
    let (ax, ay) = poke_anchor(mode, side);
    let mut far_cnt = 0u64;
    for k in 0..5usize {
        let ent = rd_u64(base + 0x1e0 + s * 0x28 + k * 8).unwrap_or(0) as usize;
        if ent == 0 { continue; }   // ★readable VQ제거(본문 rd_* fault-safe=valid sim 비트동일, poke 핫루프)
        let maxhp = rd_u64(ent + 0x610).unwrap_or(0);
        if maxhp == 0 { continue; }
        let hp = rd_u64(ent + 0x658).unwrap_or(0).wrapping_mul(100) / maxhp;
        if hp < 0x28 { continue; }
        let (ex, ey) = (rd_u64(ent + 0x648).unwrap_or(0), rd_u64(ent + 0x650).unwrap_or(0));
        if !poke_f6f720(vobj, ex, ey, bvar14 as u8) { continue; }
        if (sqd(ax, ay, ex, ey) >> 8) > 0xe8d4a50 { far_cnt += 1; }
    }
    if far_cnt == 0 { return false; }
    let (blk, slot) = if bvar14 == 0 { (s * 0x228 + geom, 0x2170usize) }
                      else if bvar14 == 2 { (s * 0x228 + geom + 0x50, 0x2190) }
                      else { (s * 0x228 + geom + 0x28, 0x2180) };
    if rd_u64(base + slot + s * 8).unwrap_or(0) < 2 {
        let prog = rd_i64(blk + 0x10).unwrap_or(0);
        if prog < 3000 {
            let iv = rd_i32(blk + 0x20).unwrap_or(0);
            let thr = if prog < 1000 { 4 } else { 2 };
            return iv < thr;
        }
        return true;
    }
    true
}

// FUN_141fe2b10: poke 안전성 예측자. self 레인밴드 + 앵커거리 + fe2cc0 + 적0 & 아군>=임계.
const POKE_ALLY_THR: [u64; 6] = [4, 2, 2, 3, 4, 4]; // DAT_1435d9230 (vobj+0x28 byte)
unsafe fn poke_fe2b10(p3: u64, p5: usize, p6: usize, mode: u8) -> bool {
    if p3 <= 0x18 { return false; }
    let bvar14: i32 = if mode == 4 { 2 } else if mode == 5 { 0 } else { -1 };
    let l80 = rd_u64(p6).unwrap_or(0) as usize;
    if !ptr_ok(l80) { return false; }
    let vobj = rd_u64(p6 + 8).unwrap_or(0) as usize;       // p6[1] = node(f6f720)
    if !ptr_ok(vobj) { return false; }
    if bvar14 == 0 {
        if rd_u8(vobj + 0x28).wrapping_sub(4) > 0xfc { return false; }
    } else if bvar14 != 2 { return false; }
    let sim = rd_u64(l80).unwrap_or(0) as usize;           // l80[0]
    if !ptr_ok(sim) { return false; }
    let selfe = dd7_slot128(sim, rd_u64(p5 + 0x6a0).unwrap_or(0));
    if !ptr_ok(selfe) || !readable(selfe + 0x650, 8) { return false; }
    let side = rd_i64(p5 + 0x6a8).unwrap_or(-1);
    if side < 0 || side > 1 { return false; }
    let (ax, ay) = poke_anchor(mode, side);
    let (sx, sy) = (rd_u64(selfe + 0x648).unwrap_or(0), rd_u64(selfe + 0x650).unwrap_or(0));
    if !poke_f6f720(vobj, sx, sy, bvar14 as u8) { return false; }
    if (sqd(ax, ay, sx, sy) >> 8) <= 0xe8d4a50 { return false; }   // 0xe8d4a50 < dist²>>8 필요
    if !poke_fe2cc0(p5, p6, mode) { return false; }
    let allies = poke_count_allies(p5, p6, ax, ay, 180000, 0x28);
    let enemies = poke_count_enemies(p5, p6, ax, ay, 180000, 0x28);
    let vbyte = (rd_u8(vobj + 0x28) as usize).min(5);
    enemies == 0 && POKE_ALLY_THR[vbyte] <= allies
}

// vt+off(arg) 1인자 호출(node ctx vt+0x168 등 pure getter). dd7700 def_resolve(vt+0x140)와 동일 패턴.
#[inline] unsafe fn vt_call1(vt: usize, off: usize, a: usize) -> usize {
    let f = vt_slot(vt, off);
    if !ptr_ok(f) { return 0; }
    let g: VtPtrFn = core::mem::transmute(f);
    g(a)
}
// 노드 resolve: vt+0x168(sim)=node ctx; (+field!=0 && resolve(*(node+koff))). epic koff=(400,0x188), serpen=(0x1c0,0x1b8).
unsafe fn poke_node_resolve(sim: usize, vt: usize, chk_off: usize, ptr_off: usize) -> usize {
    let node = vt_call1(vt, 0x168, sim);
    if !ptr_ok(node) || rd_u64(node + chk_off).unwrap_or(0) == 0 { return 0; }
    let pp = rd_u64(node + ptr_off).unwrap_or(0) as usize;   // *(node+ptr_off)
    if !ptr_ok(pp) { return 0; }
    let h = rd_u64(pp).unwrap_or(0);                          // **(node+ptr_off)
    def_resolve(sim, vt, h)
}
// ★존타입 그리드 30×30 (맵 정적데이터, .rdata 0x1435 87f68 → const복사로 churn제거+버그수정, 2026-06-19).
//   값=존타입 0~26(중앙 7=에픽존). 옛 코드는 stale 0x352b720(=포인터테이블) 읽어 EpicBattle zone(5) 깨져있었음.
//   게임 인덱싱(FUN@0x1ff6ef0): grid[yc*0xf0 + xc*8], xc/yc=pos/32000 cap29. ⟹ ZONE_GRID[yc*30+xc].
static ZONE_GRID: [u8; 900] = [
    17,17,17,17,17,17,17,18,18,18,18,18,18,18,20,20,20,20,20,20,20,20,26,26,26,26,26,26,26,26,  // row 0
    17,17,17,17,17,17,17,18,18,18,18,18,18,18,20,20,20,20,20,20,20,20,26,26,26,26,26,26,26,26,  // row 1
    17,17,17,17,17,17,17,18,18,18,18,18,18,18,20,20,20,20,20,20,20,20,26,26,26,26,26,26,26,26,  // row 2
    17,17,17,17,17,17,17,18,18,18,18,18,18,10,10,20,20,20,20,20,20,20,26,26,26,26,26,26,26,26,  // row 3
    17,17,17,17,7,7,7,18,18,18,18,18,10,10,10,11,11,11,11,11,11,11,26,26,26,26,26,26,26,26,  // row 4
    17,17,17,17,7,7,7,7,7,7,7,7,10,10,10,11,11,11,11,11,11,11,26,26,26,26,26,26,26,26,  // row 5
    17,17,17,17,7,7,7,7,7,7,7,7,7,10,10,10,11,11,11,11,11,12,26,26,26,26,26,26,26,26,  // row 6
    22,22,22,22,22,7,7,7,7,7,7,7,7,10,10,10,10,10,11,11,12,12,26,26,26,26,26,26,26,26,  // row 7
    22,22,22,22,22,7,7,7,7,7,7,7,7,10,10,10,10,10,11,12,12,12,12,12,21,21,25,25,25,25,  // row 8
    22,22,22,22,22,7,7,7,7,7,7,7,7,7,10,10,10,10,3,12,12,12,12,21,21,21,25,25,25,25,  // row 9
    22,22,22,22,22,7,7,7,7,7,7,7,7,7,10,10,10,3,3,3,12,12,21,21,21,21,25,25,25,25,  // row 10
    22,22,22,22,22,7,7,7,7,7,7,7,7,7,10,10,3,3,3,3,3,21,21,21,21,21,25,25,25,25,  // row 11
    22,22,22,22,22,7,7,7,7,7,7,7,7,7,7,3,3,3,3,3,15,15,21,21,21,21,25,25,25,25,  // row 12
    22,22,22,13,13,13,13,13,13,7,7,7,7,7,6,6,3,3,3,15,15,15,15,21,21,21,25,25,25,25,  // row 13
    5,5,5,13,13,13,13,13,13,13,13,13,7,6,6,6,6,3,15,15,15,15,15,15,15,21,25,25,25,25,  // row 14
    5,5,5,5,1,1,13,13,13,13,13,13,4,6,6,6,6,15,15,15,15,15,15,15,15,21,25,25,25,25,  // row 15
    5,5,5,5,1,1,1,13,13,13,13,4,4,4,6,6,2,2,15,15,15,15,15,15,15,15,23,23,23,23,  // row 16
    5,5,5,5,1,1,1,13,13,13,4,4,4,4,4,14,2,2,2,15,15,15,15,15,15,15,23,23,23,23,  // row 17
    5,5,5,5,1,1,1,1,1,4,4,4,4,4,14,14,14,2,2,2,15,15,15,15,15,15,23,23,23,23,  // row 18
    5,5,5,5,1,1,1,1,19,19,4,4,4,14,14,14,14,14,2,2,2,15,15,15,15,15,23,23,23,23,  // row 19
    5,5,5,5,1,1,1,19,19,19,19,4,14,14,14,14,14,14,14,2,2,2,15,15,15,15,23,23,23,23,  // row 20
    5,5,5,5,1,1,19,19,19,19,19,24,14,14,14,14,14,14,14,14,2,2,2,15,15,15,23,23,23,23,  // row 21
    0,0,0,0,0,0,0,0,19,19,24,24,24,14,14,14,14,14,14,14,14,2,2,2,2,2,23,23,23,23,  // row 22
    0,0,0,0,0,0,0,0,19,24,24,24,24,24,14,14,14,14,14,14,14,14,2,2,2,2,16,16,16,16,  // row 23
    0,0,0,0,0,0,0,0,24,24,24,24,24,24,14,14,14,14,14,14,14,14,2,2,2,2,16,16,16,16,  // row 24
    0,0,0,0,0,0,0,0,24,24,24,24,24,24,24,24,14,14,14,14,14,14,2,2,2,2,16,16,16,16,  // row 25
    0,0,0,0,0,0,0,0,9,9,9,9,9,9,9,9,8,8,8,8,8,8,8,16,16,16,16,16,16,16,  // row 26
    0,0,0,0,0,0,0,0,9,9,9,9,9,9,9,9,8,8,8,8,8,8,8,16,16,16,16,16,16,16,  // row 27
    0,0,0,0,0,0,0,0,9,9,9,9,9,9,9,9,8,8,8,8,8,8,8,16,16,16,16,16,16,16,  // row 28
    0,0,0,0,0,0,0,0,9,9,9,9,9,9,9,9,8,8,8,8,8,8,8,16,16,16,16,16,16,16,  // row 29
];
// zone7 카운트: vec_side 5슬롯서 ZONE_GRID==7 & vt필터(필터는 filter_side로 dd7_slot48) 통과 카운트.
// ★게임: OTHER카운트=vec_side=other/geo=other이나 dd7_slot48는 SELF side. SELF카운트=둘다self.
unsafe fn poke_zone7_count(l80: usize, vec_side: usize, filter_side: usize, sim: usize, geom: usize) -> u64 {
    if !ptr_ok(l80) { return 0; }
    let geo_side = vec_side * 0x228 + geom;
    let mut count = 0u64;
    for k in 0..5usize {
        let ent = rd_u64(l80 + 0x1e0 + vec_side * 0x28 + k * 8).unwrap_or(0) as usize;
        if ent == 0 { continue; }   // ★readable VQ제거(본문 rd_* fault-safe=valid sim 비트동일, poke 핫루프)
        let xc = (rd_u64(ent + 0x648).unwrap_or(0) / 32000).min(0x1d) as usize;
        let yc = (rd_u64(ent + 0x650).unwrap_or(0) / 32000).min(0x1d) as usize;
        if ZONE_GRID[yc * 30 + xc] != 7 { continue; }
        let id = rd_u64(ent + 0x5a8).unwrap_or(0);
        let pass = if dd7_slot48(sim, filter_side, id) { true } else {
            let ha8 = dd7_slot_a8(sim, id);
            if ha8 == 0 { false } else {
                let lane = rd_i64(geo_side + 0x1e0 + (rd_i32(ha8 + 0x738).unwrap_or(0) as usize) * 8).unwrap_or(0);
                dd7_slot20(sim) <= lane + 0x78
            }
        };
        if pass { count += 1; }
    }
    count
}
// ── fdae40(engage 예측자) callee들 ──
// FUN_141fe2980: 노드resolve + vt0x48필터 + 위협시간(reach+threatScale<reachSelf? & threat<=scale*0x14). →bool.
unsafe fn poke_fe2980(side: usize, l78: usize, vobj: usize, vbyte: u8, threat_a: u64, threat_b: u64, reach_a: u64, reach_b: u64, mode9: u8) -> bool {
    let lvar1 = rd_i64(vobj + 0x12f8).unwrap_or(0);
    let sim = rd_u64(l78).unwrap_or(0) as usize;
    let vt = rd_u64(l78 + 8).unwrap_or(0) as usize;
    if !ptr_ok(sim) || !ptr_ok(vt) { return false; }
    let (byte_ok, chk, ptr, threat, reach) = if mode9 == 0 {
        (vbyte.wrapping_sub(1) > 2, 400usize, 0x188usize, threat_a, reach_a)   // not in {1,2,3}
    } else {
        (vbyte.wrapping_sub(1) > 1, 0x1c0, 0x1b8, threat_b, reach_b)            // not in {1,2}
    };
    if !byte_ok { return false; }
    let reach_self = dd7_slot20(sim) as u64;                  // vt+0x20
    let e = poke_node_resolve(sim, vt, chk, ptr);
    if e == 0 { return false; }
    if !dd7_slot48(sim, side, rd_u64(e + 0x5a8).unwrap_or(0)) { return false; }   // vt+0x48==0 → 0
    if reach.wrapping_add(lvar1 as u64) < reach_self { return false; }
    if rd_u64(e + 0x610).unwrap_or(0) <= rd_u64(e + 0x658).unwrap_or(0) { return false; }   // e 풀피 → 0
    threat <= (lvar1 as u64).wrapping_mul(0x14)
}

unsafe fn mem_eq(a: usize, b: usize, len: usize) -> bool {
    if len == 0 { return true; }
    if len > 4096 { return false; }
    // ★readable() VirtualQuery 2회 제거(engage e88a0 3중루프 1.78ms/call의 주범 — fast_read 무관했던 숨은비용): lockless lr_u8 비교, fault=불일치(readable-false와 동의미·비트동일·미스매치 조기탈출)
    for i in 0..len { match (lr_u8(a+i), lr_u8(b+i)) { (Some(x), Some(y)) if x == y => {}, _ => return false } }
    true
}
// ── df0c10(FUN_141e46f90) 적격후보 존재 flag 완전재현 (disasm 0x141e46f90 전구간 검증) ──
// ★핵심정정: getter vt+0x50(0x985d00)=`mov rax,rcx`(=obj자신), vt+0x78(0x985de0)=`lea rax,[rcx+0x30]` — FUN_141985c80
//   (문자열빌더)가 아니라 trivial 필드읽기. key=String{*(obj+8),*(obj+0x10)}, keylist=Vec@obj+0x30. → 순수재현.
// 매핑(epic judge FUN_141b21440 disasm): POOL ctx=judge param_5=p5, self_obj=*(*(p6)), EXISTING holder=*(self_obj+0x20).
// flag = ∃ c∈POOL, K∈keylist(c), e=첫 EXISTING(key(e)==K len+memcmp): prio(e)>maxprio && thresh(e)<=thr.
//   maxprio = max(0, max prio(p) over POOL where prio<=3). RNG는 N>0일때 winner 인덱스만 뽑음(flag와 무관).
// ★EXISTING 출처 정정(2026-06-17): df0c10 [RBP+0x90]=arg7=puVar3=param_6[1]=vobj (RBP=rsp-0x58).
//   EXISTING holder=*(vobj+0x20). (이전 self_obj+0x20은 오계산 → 항상 ex_len=0이었음.) POOL ctx=arg4=p5.
unsafe fn poke_df0c10_flag(p5: usize, p6: usize) -> bool {
    let vobj = rd_u64(p6 + 8).unwrap_or(0) as usize;   // param_6[1]
    df0c_flag_core(p5, vobj)
}
// pool_ctx: POOL=*(pool_ctx+0x3c8). ex_src: EXISTING holder=*(ex_src+0x20). df0c10-시점/디스패처-시점 공용.
unsafe fn df0c_flag_core(pool_ctx: usize, ex_src: usize) -> bool {
    let pool_ptr = rd_u64(pool_ctx + 0x3c8).unwrap_or(0) as usize;
    let pool_len = rd_u64(pool_ctx + 0x3d0).unwrap_or(0) as usize;
    let thr = rd_u64(pool_ctx + 0x710).unwrap_or(0);
    if !ptr_ok(pool_ptr) || pool_len == 0 || pool_len > 256 { return false; }
    if !ptr_ok(ex_src) { return false; }
    let holder = rd_u64(ex_src + 0x20).unwrap_or(0) as usize;
    if !ptr_ok(holder) { return false; }
    let ex_ptr = rd_u64(holder + 8).unwrap_or(0) as usize;
    let ex_len = rd_u64(holder + 0x10).unwrap_or(0) as usize;
    if !ptr_ok(ex_ptr) || ex_len == 0 || ex_len > 256 { return false; }
    // maxprio = POOL 중 prio<=3 최대 (default 0)
    let mut maxprio: u64 = 0;
    for i in 0..pool_len {
        let obj = rd_u64(pool_ptr + i*0x10).unwrap_or(0) as usize;
        if !ptr_ok(obj) { continue; }
        let pr = rd_u64(obj + 0x188).unwrap_or(0);
        if pr <= 3 && pr > maxprio { maxprio = pr; }
    }
    for ci in 0..pool_len {
        let c_obj = rd_u64(pool_ptr + ci*0x10).unwrap_or(0) as usize;
        if !ptr_ok(c_obj) { continue; }
        let kl_ptr = rd_u64(c_obj + 0x38).unwrap_or(0) as usize;   // keylist Vec ptr (@obj+0x30 +8)
        let kl_len = rd_u64(c_obj + 0x40).unwrap_or(0) as usize;   // keylist Vec len (+0x10)
        if !ptr_ok(kl_ptr) || kl_len == 0 || kl_len > 256 { continue; }
        for ki in 0..kl_len {
            let k = kl_ptr + ki*0x18;                               // K entry (stride 0x18)
            let k_ptr = rd_u64(k + 8).unwrap_or(0) as usize;
            let k_len = rd_u64(k + 0x10).unwrap_or(0) as usize;
            for j in 0..ex_len {
                let e_obj = rd_u64(ex_ptr + j*0x10).unwrap_or(0) as usize;
                if !ptr_ok(e_obj) { continue; }
                let ke_len = rd_u64(e_obj + 0x10).unwrap_or(0) as usize;
                if ke_len != k_len { continue; }
                let ke_ptr = rd_u64(e_obj + 8).unwrap_or(0) as usize;
                if !mem_eq(ke_ptr, k_ptr, k_len) { continue; }
                // 첫 key-match e: 적격검사 후 무조건 다음 K로 (break)
                if rd_u64(e_obj + 0x188).unwrap_or(0) > maxprio
                    && rd_u64(e_obj + 0x180).unwrap_or(0) <= thr { return true; }
                break;
            }
        }
    }
    false
}
// ── engage-block 예측자 FUN_141fdae40 완전재현 (disasm 0x141fdae40 검증) ──
// 게이트: p3<0x18 / champ플래그(mode별) / fe2ff0(더매력레인 존재)≠-1 / vec빈 / 밴드count<2 / vt128==0 / d2fa40최소레인≠지정(p5+0x738).
// 터미널: fe2980!=0→block(1); else vt20<=thr && vt50!=0 → fa6730; else block(1). 게이트탈락=0(engage허용).
const POKE_BASE: [u64;5] = [60000, 0, 40000, 60000, 20000];   // DAT_1435d9178/14359f2d0 qword[slot]

// 밴드 predicate(disasm확정). dy=mapY-Y. mode4:(big,small)=(X,dy) / mode5:(big,small)=(dy,X).
// NOT-qual = big>0x2ee00 && big>small && small<mapX-0x2ee00 && (big-small)>=64000.
#[inline] fn poke_band_qual(x: u64, y: u64, mapx: u64, mapy: u64, mode: u8) -> bool {
    let dy = mapy.wrapping_sub(y);
    let (big, small) = if mode == 4 { (x, dy) } else { (dy, x) };
    !(big > 0x2ee00 && big > small && small < mapx.wrapping_sub(0x2ee00) && big.wrapping_sub(small) >= 64000)
}

// FUN_141fe2ff0: 패턴(레인타입 2바이트) 중 "더 매력적" 레인 탐색. 발견시 그 타입(0/1/2), 없으면 -1.
unsafe fn poke_fe2ff0(p5: usize, p6: usize, pat: &[u8]) -> i32 {
    let side = rd_u64(p5 + 0x6a8).unwrap_or(0);
    if side >= 2 { return -1; }                              // game panics
    let l78 = rd_u64(p6).unwrap_or(0) as usize;              // p6[0] container
    let vobj = rd_u64(p6 + 8).unwrap_or(0) as usize;         // p6[1]
    let lane_arr = rd_u64(p6 + 0x10).unwrap_or(0) as usize;  // p6[2] (stride 0x228)
    let vbyte = rd_u8(vobj + 0x28);
    let lane9 = (side as usize) * 0x228;
    let mut best: i64 = 0; let mut cv: i32 = -1;
    for &b in pat {
        let (lv12, lv11_off, enter) = if b == 0 {
            (0x2170usize, 0usize, vbyte.wrapping_sub(1) > 2)
        } else if b == 2 {
            (0x2190, 0x50, true)
        } else {                                             // b == 1
            (0x2180, 0x28, vbyte.wrapping_sub(1) > 1)
        };
        if !enter { continue; }
        let uv4 = rd_u64(l78 + (side as usize)*8 + lv12).unwrap_or(0);
        let lv11 = lane_arr + lane9 + lv11_off;
        let lane_val = rd_i64(lv11 + 0x10).unwrap_or(0);
        let lane_cnt = rd_i32(lv11 + 0x20).unwrap_or(0);
        if uv4 < 3 || lane_val < 2000 || lane_cnt < 2 {
            let score = (lane_cnt as i64)*1000 + lane_val + (uv4 as i64)*10000;
            if cv == -1 || score < best { best = score; cv = b as i32; }
        }
    }
    cv
}

// FUN_141fa6730: 적측(1-side) 5레인 중 위협충분 존재? 도달<위협이면 block(true). vt0xa8/0x48/0x20 리졸버.
unsafe fn poke_fa6730(l78: usize, self_obj: usize, _self_vt: usize, p6_2: usize, p5: usize, p8: usize, anchor: u64, vt20: u64) -> bool {
    let side = rd_u64(p5 + 0x6a8).unwrap_or(0);
    let other = 1u64.wrapping_sub(side);
    let lane_base = l78 + (other as usize)*0x28 + 0x1e0;
    for s in 0..5usize {
        let lane = rd_u64(lane_base + s*8).unwrap_or(0) as usize;
        if lane == 0 { continue; }
        let max = rd_u64(lane + 0x610).unwrap_or(0); if max == 0 { continue; }
        let hppct = rd_u64(lane + 0x658).unwrap_or(0).wrapping_mul(100) / max;
        if hppct <= 0x31 { continue; }                       // 0x31 < hppct 필요
        let lane_id = rd_u64(lane + 0x5a8).unwrap_or(0);
        let a8 = dd7_slot_a8(self_obj, lane_id);
        let c48 = dd7_slot48(self_obj, side as usize, lane_id);   // cVar16==0 ⟺ !dd7_slot48
        let gate = !c48 && (a8 == 0 || {
            let idx = rd_u32(a8 + 0x738) as usize;
            let lane_state = rd_u64((other as usize)*0x228 + p6_2 + 0x1e0 + idx*8).unwrap_or(0);
            lane_state + 0x78 < dd7_slot20(self_obj) as u64
        });
        if gate {
            let ex = rd_u64(p8 + 0x218 + s*0x10).unwrap_or(0);
            let ey = rd_u64(p8 + 0x220 + s*0x10).unwrap_or(0);
            let dx = anchor.abs_diff(ex); let dy = anchor.abs_diff(ey);
            let d = isqrt(dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)));
            let threat = if d < 150000 { 0 } else { d - 150000 };
            let champ_reach = rd_u64(p8 + 0x2b8 + s*8).unwrap_or(0);
            let reach = if vt20 < champ_reach { 0 } else { vt20 - champ_reach };
            let mult = rd_u64(lane + 0x628).unwrap_or(0);
            if threat <= reach.wrapping_mul(mult) { return true; }
        }
    }
    false
}

unsafe fn poke_fdae40(p8: usize, p3: u64, p5: usize, p6: usize, p7: usize, mode: u8) -> bool {
    if p3 < 0x18 { return false; }
    let f3e6 = rd_u8(p8 + 0x3e6);
    let f3e7 = rd_u8(p8 + 0x3e7);                            // off 999
    let pat: &[u8] = if mode == 5 {
        if f3e7 != 1 || f3e6 != 1 { return false; }
        &[2u8, 1]
    } else if mode == 4 {
        if f3e6 != 0 || f3e7 != 1 { return false; }
        &[0u8, 1]
    } else { return false; };
    if poke_fe2ff0(p5, p6, pat) != -1 { return false; }     // 더 매력적 레인 → engage 허용
    let side = rd_u64(p5 + 0x6a8).unwrap_or(0);
    if side > 1 { return false; }
    let l78 = rd_u64(p6).unwrap_or(0) as usize;
    let vobj = rd_u64(p6 + 8).unwrap_or(0) as usize;
    let mapstate = rd_u64(vobj + 8).unwrap_or(0) as usize;
    if !ptr_ok(l78) || !ptr_ok(mapstate) { return false; }
    let mapx = rd_u64(mapstate + 0x12b8).unwrap_or(0);
    let mapy = rd_u64(mapstate + 0x12c0).unwrap_or(0);
    let lane_base = l78 + (side as usize)*0x28 + 0x1e0;
    // vec빈 + 밴드count
    let mut nonzero = 0u32; let mut bandcount = 0u32;
    for k in 0..5usize {
        let slot = rd_u64(lane_base + k*8).unwrap_or(0) as usize;
        if slot == 0 { continue; }
        nonzero += 1;
        let max = rd_u64(slot + 0x610).unwrap_or(0); if max == 0 { continue; }
        let hppct = rd_u64(slot + 0x658).unwrap_or(0).wrapping_mul(100) / max;
        if hppct < 0x28 { continue; }
        let sx = rd_u64(slot + 0x648).unwrap_or(0);
        let sy = rd_u64(slot + 0x650).unwrap_or(0);
        if poke_band_qual(sx, sy, mapx, mapy, mode) { bandcount += 1; }
    }
    if nonzero == 0 || bandcount < 2 { return false; }
    let self_obj = rd_u64(l78).unwrap_or(0) as usize;
    let self_vt = rd_u64(l78 + 8).unwrap_or(0) as usize;
    if !ptr_ok(self_obj) || !ptr_ok(self_vt) { return false; }
    if dd7_slot128(self_obj, rd_u64(p5 + 0x6a0).unwrap_or(0)) == 0 { return false; }   // vt128==0
    let anchor = if mode == 4 { 288000u64 } else { 672000u64 };
    // chosen = 첫 qualifying 레인
    let mut chosen: Option<(usize, u64, u64)> = None;
    for k in 0..5usize {
        let slot = rd_u64(lane_base + k*8).unwrap_or(0) as usize;
        if slot == 0 { continue; }
        let max = rd_u64(slot + 0x610).unwrap_or(0); if max == 0 { continue; }
        let hppct = rd_u64(slot + 0x658).unwrap_or(0).wrapping_mul(100) / max;
        if hppct < 0x28 { continue; }
        let sx = rd_u64(slot + 0x648).unwrap_or(0);
        let sy = rd_u64(slot + 0x650).unwrap_or(0);
        if poke_band_qual(sx, sy, mapx, mapy, mode) { chosen = Some((k, sx, sy)); break; }
    }
    let (cidx, cx, cy) = match chosen { Some(c) => c, None => return false };
    let score = |sx: u64, sy: u64, idx: usize| -> u64 {
        let dx = anchor.abs_diff(sx); let dy = anchor.abs_diff(sy);
        isqrt(dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))).wrapping_add(POKE_BASE[idx])
    };
    // d2fa40: {chosen} ∪ qualifying(cidx+1..4) 중 최소-score 레인
    let mut min_score = score(cx, cy, cidx);
    let mut min_idx = cidx as u64;
    for k in (cidx+1)..5usize {
        let slot = rd_u64(lane_base + k*8).unwrap_or(0) as usize;
        if slot == 0 { continue; }
        let max = rd_u64(slot + 0x610).unwrap_or(0); if max == 0 { continue; }
        let hppct = rd_u64(slot + 0x658).unwrap_or(0).wrapping_mul(100) / max;
        if hppct < 0x28 { continue; }
        let sx = rd_u64(slot + 0x648).unwrap_or(0);
        let sy = rd_u64(slot + 0x650).unwrap_or(0);
        if !poke_band_qual(sx, sy, mapx, mapy, mode) { continue; }
        let sc = score(sx, sy, k);
        if sc < min_score { min_score = sc; min_idx = k as u64; }
    }
    if min_idx != rd_u32(p5 + 0x738) as u64 { return false; }   // 최소레인≠지정 → engage 허용
    // ── 터미널 ──
    let vbyte = rd_u8(vobj + 0x28);
    let cv = poke_fe2980(side as usize, l78, mapstate, vbyte,
        rd_u64(p7 + 0x88).unwrap_or(0), rd_u64(p7 + 0xc0).unwrap_or(0),
        rd_u64(p8 + 0x80).unwrap_or(0), rd_u64(p8 + 0x88).unwrap_or(0),
        if mode == 5 { 1 } else { 0 });
    if cv { return true; }                                   // fe2980!=0 → block
    let vt20 = dd7_slot20(self_obj) as u64;
    let thr_base = if mode == 5 { rd_u64(p8 + 0x88).unwrap_or(0) } else { rd_u64(p8 + 0x80).unwrap_or(0) };
    let threat_scale = rd_i64(mapstate + 0x12f8).unwrap_or(0);
    let thr = thr_base.wrapping_add((threat_scale as u64).wrapping_mul(2));
    let s50 = vt_slot(self_vt, 0x50);
    let vt50 = if ptr_ok(s50) {
        let f: VtPtr4Fn = core::mem::transmute(s50);
        f(self_obj, side as usize, (anchor/32000) as usize, (anchor/32000) as usize)
    } else { 0 };
    if vt20 <= thr && vt50 != 0 {
        return poke_fa6730(l78, self_obj, self_vt, rd_u64(p6 + 0x10).unwrap_or(0) as usize, p5, p8, anchor, vt20);
    }
    true                                                     // block
}

// ★df0c10 후보 getter RVA 캡처 (완전재현 선결). pool=df0c10 param_4=judge p5. 후보={obj,vt} stride0x10.
static DF0CGP_DONE: AtomicBool = AtomicBool::new(false);
unsafe fn df0c_getter_probe(pool: usize, disc: u64) {
    if DF0CGP_DONE.load(Ordering::Relaxed) { return; }
    if !ptr_ok(pool) || !readable(pool + 0x3d0, 8) { return; }
    let base = exe_base();
    if base == 0 { return; }
    let count = rd_u64(pool + 0x3d0).unwrap_or(0);
    let cbase = rd_u64(pool + 0x3c8).unwrap_or(0) as usize;
    let thr = rd_u64(pool + 0x710).unwrap_or(0);
    if !ptr_ok(cbase) || count == 0 || count >= 64 { return; }
    let mut s = format!("[df0cgp] disc={} count={} cbase={:#x} thr(p5+0x710)={}\n", disc, count, cbase, thr);
    for i in 0..(count as usize).min(8) {
        let obj = rd_u64(cbase + i * 0x10).unwrap_or(0) as usize;
        let vt = rd_u64(cbase + i * 0x10 + 8).unwrap_or(0) as usize;
        if !ptr_ok(vt) { continue; }
        let g = |off: usize| -> u64 { rd_u64(vt + off).unwrap_or(0).wrapping_sub(base as u64) };
        s.push_str(&format!("  cand{} obj={:#x} vt_rva={:#x} | g50={:#x} g60={:#x} g68={:#x} g78={:#x}\n",
            i, obj.wrapping_sub(base), (vt as u64).wrapping_sub(base as u64), g(0x50), g(0x60), g(0x68), g(0x78)));
    }
    write_named("df0cgp.txt", &s);
    DF0CGP_DONE.store(true, Ordering::Relaxed);
}
// ★df0c10 직접 진입훅(0x2068b10, 12B=8push). R9=param_4=후보풀(게임이 여기서 읽으니 유효). poke 콜러만 1회 캡처.
unsafe extern "C" fn df0c10_entry_probe(saved: usize, entry_rsp: usize) {
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN || DF0CGP_DONE.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 { return; }
    let ret = rd_u64(entry_rsp).unwrap_or(0).wrapping_sub(base as u64);   // 콜러 RVA
    let in_epic = ret >= 0x1b21440 && ret < 0x1b21d00;
    let in_serpen = ret >= 0x1b224f0 && ret < 0x1b22e00;
    if !in_epic && !in_serpen { return; }   // poke judge 콜러만
    let pool = rd_u64(saved + 0x10).unwrap_or(0) as usize;   // R9 = param_4(POOL ctx)
    df0c_getter_probe(pool, if in_epic { 9 } else { 11 });
}

// ★epic 7-진단: my_epic_poke가 return 7할 때 reason+상태 패킹. DIFF(my=7,game≠7)시 핸들러서 로깅.
static EPIC_DIAG: AtomicU64 = AtomicU64::new(0);
static EPICDIAG_N: AtomicU64 = AtomicU64::new(0);
static EPICDIAG_INIT: AtomicBool = AtomicBool::new(false);
static ENG_DIAG: AtomicU64 = AtomicU64::new(0);    // my=13(engage) 진단: champ999/champ3e6/side
static ENG_DIST: AtomicU64 = AtomicU64::new(0);    // my=13 dist² (임계 0x53d1ac101 대비 거리)
static EPIC11_DIAG: AtomicU64 = AtomicU64::new(0); // my=11 진단: reason+node2 5조건+zone+fdae40+flag
static ENGDIAG_N: AtomicU64 = AtomicU64::new(0);
static ENGDIAG_INIT: AtomicBool = AtomicBool::new(false);
#[inline] fn epic_d(reason: u64, hp: u64, obj_full: bool, not_home: bool, side: i64, sz7: u64, oz7: u64, ohp: u64, thr_lt: bool) -> u64 {
    (reason & 0xf) | (hp.min(255) << 4) | ((obj_full as u64) << 12) | ((not_home as u64) << 13)
    | (((side & 1) as u64) << 14) | (sz7.min(15) << 16) | (oz7.min(15) << 20) | (ohp.min(255) << 24) | ((thr_lt as u64) << 32)
}
// ════ EpicPoke(disc9) FUN_141b21440 재현. 출력 {19,11,12,13,7,3,2}. df0c10 flag·fdae40만 stub. ════
unsafe fn my_epic_poke(p2: usize, p3: u64, p5: usize, p6: usize, p7: usize, p8: usize) -> i64 {
    let _pg = perf_guard(7);
    let l80 = rd_u64(p6).unwrap_or(0) as usize;
    if !ptr_ok(l80) { return -99; }
    let sim = rd_u64(l80).unwrap_or(0) as usize;
    let vt = rd_u64(l80 + 8).unwrap_or(0) as usize;
    if !ptr_ok(sim) || !ptr_ok(vt) { return -99; }
    let self_handle = rd_u64(p5 + 0x6a0).unwrap_or(0);
    let selfe = dd7_slot128(sim, self_handle);
    if !ptr_ok(selfe) { return -99; }   // ★readable VQ제거(poke selfe, 좌표 rd_u64 fault-safe)
    // 이른 분기: subplan flags
    if rd_u8(p2) != 0 || rd_u8(p2 + 1) != 0 {
        let e = poke_node_resolve(sim, vt, 400, 0x188);
        if e == 0 { EPIC11_DIAG.store(3, Ordering::Relaxed); }   // reason3: 이른분기 node==0
        return if e != 0 { 0x13 } else { 0xb };   // 19 / 11
    }
    let maxhp = rd_u64(selfe + 0x610).unwrap_or(0);
    if maxhp == 0 { return -99; }
    let curhp = rd_u64(selfe + 0x658).unwrap_or(0);
    let hp_pct = curhp.wrapping_mul(100) / maxhp;
    let flag = poke_df0c10_flag(p5, p6);                      // ★완전재현(disasm)
    let local_50 = poke_node_resolve(sim, vt, 400, 0x188);
    if local_50 == 0 { EPIC11_DIAG.store(1, Ordering::Relaxed); return 0xb; }   // ★readable VQ제거(obj_full/좌표 rd_u64 fault-safe)
    let side = rd_i64(p5 + 0x6a8).unwrap_or(-1);
    if side < 0 || side > 1 { return -99; }
    let (s, other) = (side as usize, 1 - side as usize);
    let selfx = rd_u64(selfe + 0x648).unwrap_or(0);
    let selfy = rd_u64(selfe + 0x650).unwrap_or(0);
    let xup = if side == 0 { tune("pk_home_lo", 64000) as u64 } else { tune("pk_home_hi", 960000) as u64 };   // ★튜닝: X 홈경계
    let yup = if side == 0 { tune("pk_home_hi", 960000) as u64 } else { tune("pk_home_lo", 64000) as u64 };   // ★튜닝: Y 홈경계
    let home_x1 = tune("pk_home_x1", 0xd9c60) as u64;   // ★호이스트: 1·2차 home 체크 공용(중복 tune 조회 제거)
    let home_y1 = tune("pk_home_y1", 0xdac00) as u64;   //   (+죽은 home_lo/home_hi 제거)
    let obj_full = rd_u64(local_50 + 0x658).unwrap_or(0) == rd_u64(local_50 + 0x610).unwrap_or(0);
    let not_home = (selfx < home_x1 && side != 0) || xup < selfx || (side == 0 && selfy < home_y1);   // ★튜닝: 홈판정 X/Y 안쪽경계
    if not_home {
        if !((tune("pk_hp_main", 0x32) < hp_pct as i64 && !flag) || !obj_full) {   // ★튜닝: poke 진입 HP%(>50)
            EPIC_DIAG.store(epic_d(1, hp_pct, obj_full, not_home, side, 0, 0, 0, false), Ordering::Relaxed); return 7;
        }
    } else if obj_full && (flag || (hp_pct as i64) < tune("pk_hp_retreat", 0x33) || (curhp < maxhp && selfy <= yup)) {   // ★튜닝: 귀환 HP%(<51)
        EPIC_DIAG.store(epic_d(2, hp_pct, obj_full, not_home, side, 0, 0, 0, false), Ordering::Relaxed); return 7;
    }
    // LAB_2167b
    if rd_u8(p8 + 0x3e6) != 0 {
        EPIC_DIAG.store(epic_d(3, hp_pct, obj_full, not_home, side, 0, 0, 0, false), Ordering::Relaxed); return 7;
    }
    if rd_u8(p8 + 999) == 3 {
        // poke 게이트: 셀렉터 = (vt+0x38==0)? *(u32)(sim+0xecc8+side*0x18) : *(u32)(p5+0x420)
        let sel = if rd_u8(sim + 0xed69) == 0 {
            rd_i32(sim + 0xecc8 + s * 0x18).unwrap_or(-1) as u32
        } else {
            rd_i32(p5 + 0x420).unwrap_or(-1) as u32
        };
        let lane = rd_i32(p5 + 0x738).unwrap_or(-1) as u32;
        if sel < 5 && lane == sel && poke_fe2b10(p3, p5, p6, 4) {
            return if p3 < tune("pk_smallact_split", 0x21) as u64 { 3 } else { 2 };   // ★튜닝: 소액션 코드 분기 임계(p3<0x21→3)
        }
        return 0xc;   // 12 (poke-fail)
    }
    // zone-count 분기 (param_8+0x3e7 != 3)
    let t88 = rd_u64(p7 + 0x88).unwrap_or(0);
    let t98 = rd_u64(p7 + 0x98).unwrap_or(0);
    let selfe2 = dd7_slot128(sim, self_handle);
    if !ptr_ok(selfe2) { return -99; }   // ★readable VQ제거(poke selfe2)
    let s2x = rd_u64(selfe2 + 0x648).unwrap_or(0);
    let s2y = rd_u64(selfe2 + 0x650).unwrap_or(0);
    // ★2차 home 체크 (disasm 0x21742~): s2curhp<s2maxhp HP체크는 ALL side 적용(디컴파일 precedence 오역 정정).
    let s2cur = rd_u64(selfe2 + 0x658).unwrap_or(0);
    let s2mx0 = rd_u64(selfe2 + 0x610).unwrap_or(0);
    if s2x <= xup && (side == 0 || s2x >= home_x1)
       && s2y <= yup && (side != 0 || s2y >= home_y1)
       && s2cur < s2mx0 {
        EPIC_DIAG.store(epic_d(4, hp_pct, obj_full, not_home, side, (s2x/32000).min(31), (s2y/32000).min(31), 0, false), Ordering::Relaxed); return 7;
    }
    let s2max = rd_u64(selfe2 + 0x610).unwrap_or(0);
    if s2max == 0 { return -99; }
    let tmin = if t88 < t98 { t88 } else { t98 };
    let vobj = rd_u64(p6 + 8).unwrap_or(0) as usize;
    let m = rd_u64(vobj + 8).unwrap_or(0) as usize;
    let threat_scale = rd_i64(m + 0x12f8).unwrap_or(0);
    let geom = rd_u64(p6 + 0x10).unwrap_or(0) as usize;
    let (mut zone_app, mut zsf, mut zot, mut zhp) = (false, 0u64, 0u64, 0u64);
    if (threat_scale as u64).wrapping_mul(tune("pk_threat_mult", 5) as u64) < tmin {   // ★튜닝: 위협 스케일 배수
        zone_app = true;
        zhp = rd_u64(selfe2 + 0x658).unwrap_or(0).wrapping_mul(100) / s2max;
        zot = poke_zone7_count(l80, other, s, sim, geom);
        zsf = poke_zone7_count(l80, s, s, sim, geom);
        if (zhp as i64) < tune("pk_zone_hp", 0x15) && zsf < zot {   // ★튜닝: zone 진입 HP%(<21)
            EPIC_DIAG.store(epic_d(5, hp_pct, obj_full, not_home, side, zsf, zot, zhp, true), Ordering::Relaxed); return 7;
        }
    }
    let fd = poke_fdae40(p8, p3, p5, p6, p7, 4);
    if !fd {                                                  // engage 거리체크
        let (lx, ly) = (rd_u64(local_50 + 0x648).unwrap_or(0), rd_u64(local_50 + 0x650).unwrap_or(0));
        let dsq = sqd(selfx, selfy, lx, ly);
        if dsq < tune("pk_engage_dist", 0x53d1ac101) as u64 {   // ★튜닝: 교전 거리² 임계
            ENG_DIST.store(dsq, Ordering::Relaxed);
            ENG_DIAG.store(((side & 1) as u64) << 14 | (rd_u8(p8 + 999) as u64) << 16 | (rd_u8(p8 + 0x3e6) as u64) << 24, Ordering::Relaxed);
            return 0xd;   // 13
        }
    }
    // ★EPIC11 진단: my=11(fallthrough) 상태 — node2 5조건/zone/fdae40/flag 캡처
    let c1=(s2x<=xup) as u64; let c2=(side==0||s2x>=0xd9c60) as u64; let c3=(s2y<=yup) as u64;
    let c4=(side!=0||s2y>=0xdac00) as u64; let c5=(s2cur<s2mx0) as u64; let heq=(s2cur==s2mx0) as u64;
    EPIC11_DIAG.store(2 | (fd as u64)<<3 | c1<<4 | c2<<5 | c3<<6 | c4<<7 | c5<<8 | heq<<9
        | (zone_app as u64)<<10 | ((side&1) as u64)<<11 | (flag as u64)<<12
        | (rd_u8(p8+999) as u64)<<16 | (rd_u8(p8+0x3e6) as u64)<<24
        | (zsf.min(255))<<32 | (zot.min(255))<<40 | (zhp.min(255))<<48, Ordering::Relaxed);
    0xb   // 11
}

// 노드거리² 카운트(serpent zone-count): vec_side 5슬롯서 vt필터통과 & dist²(ent,node)<0x53d1ac101 카운트.
unsafe fn poke_dist_count(l80: usize, vec_side: usize, filter_side: usize, sim: usize, geom: usize, node: usize) -> u64 {
    if !ptr_ok(l80) || !ptr_ok(node) { return 0; }   // ★readable VQ제거(nx/ny rd_u64 fault-safe)
    let (nx, ny) = (rd_u64(node + 0x648).unwrap_or(0), rd_u64(node + 0x650).unwrap_or(0));
    let geo_side = vec_side * 0x228 + geom;
    let mut count = 0u64;
    for k in 0..5usize {
        let ent = rd_u64(l80 + 0x1e0 + vec_side * 0x28 + k * 8).unwrap_or(0) as usize;
        if ent == 0 { continue; }   // ★readable VQ제거(본문 rd_* fault-safe=valid sim 비트동일, poke 핫루프)
        let id = rd_u64(ent + 0x5a8).unwrap_or(0);
        let pass = if dd7_slot48(sim, filter_side, id) { true } else {
            let ha8 = dd7_slot_a8(sim, id);
            if ha8 == 0 { false } else {
                let lane = rd_i64(geo_side + 0x1e0 + (rd_i32(ha8 + 0x738).unwrap_or(0) as usize) * 8).unwrap_or(0);
                dd7_slot20(sim) <= lane + 0x78
            }
        };
        if pass && sqd(rd_u64(ent + 0x648).unwrap_or(0), rd_u64(ent + 0x650).unwrap_or(0), nx, ny) < 0x53d1ac101 { count += 1; }
    }
    count
}

// ════ SerpenPoke(disc11) FUN_141b224f0 재현. 출력 {19,14,15,16,7,3,2}. df0c10 flag·fdae40 stub. ════
unsafe fn my_serpen_poke(p2: usize, p3: u64, p5: usize, p6: usize, p7: usize, p8: usize) -> i64 {
    let _pg = perf_guard(7);
    let l80 = rd_u64(p6).unwrap_or(0) as usize;
    if !ptr_ok(l80) { return -99; }
    let sim = rd_u64(l80).unwrap_or(0) as usize;
    let vt = rd_u64(l80 + 8).unwrap_or(0) as usize;
    if !ptr_ok(sim) || !ptr_ok(vt) { return -99; }
    let self_handle = rd_u64(p5 + 0x6a0).unwrap_or(0);
    let selfe = dd7_slot128(sim, self_handle);
    if !ptr_ok(selfe) { return -99; }   // ★readable VQ제거(poke selfe, 좌표 rd_u64 fault-safe)
    if rd_u8(p2) != 0 || rd_u8(p2 + 1) != 0 {   // 이른 분기 (serpent: 0x1c0/0x1b8)
        let e = poke_node_resolve(sim, vt, 0x1c0, 0x1b8);
        return if e != 0 { 0x13 } else { 0xe };   // 19 / 14
    }
    let maxhp = rd_u64(selfe + 0x610).unwrap_or(0);
    if maxhp == 0 { return -99; }
    let curhp = rd_u64(selfe + 0x658).unwrap_or(0);
    let hp_pct = curhp.wrapping_mul(100) / maxhp;
    let local_50 = poke_node_resolve(sim, vt, 0x1c0, 0x1b8);
    if local_50 == 0 { return 0xe; }   // ★readable VQ제거(좌표 rd_u64 fault-safe)
    let side = rd_i64(p5 + 0x6a8).unwrap_or(-1);
    if side < 0 || side > 1 { return -99; }
    let (s, other) = (side as usize, 1 - side as usize);
    let selfx = rd_u64(selfe + 0x648).unwrap_or(0);
    let selfy = rd_u64(selfe + 0x650).unwrap_or(0);
    let xup = if side == 0 { tune("pk_home_lo", 64000) as u64 } else { tune("pk_home_hi", 960000) as u64 };   // ★튜닝: X 홈경계
    let yup = if side == 0 { tune("pk_home_hi", 960000) as u64 } else { tune("pk_home_lo", 64000) as u64 };   // ★튜닝: Y 홈경계
    let home_x1 = tune("pk_home_x1", 0xd9c60) as u64;   // ★호이스트: 1·2차 home 체크 공용(중복 tune 조회 제거)
    let home_y1 = tune("pk_home_y1", 0xdac00) as u64;
    let flag = poke_df0c10_flag(p5, p6);                      // ★완전재현(disasm)
    let obj_full = rd_u64(local_50 + 0x658).unwrap_or(0) == rd_u64(local_50 + 0x610).unwrap_or(0);
    let not_home = (selfx < home_x1 && side != 0) || xup < selfx || (side == 0 && selfy < home_y1);   // ★튜닝: 홈판정 안쪽경계
    let c3e6_1 = rd_u8(p8 + 0x3e6) == 1;
    let j226fa = (hp_pct as i64) > tune("pk_hp_main", 0x32) && !flag && c3e6_1;   // ★튜닝: poke 진입 HP%(>50). j226fa→main
    let to_main = if not_home {
        if obj_full { j226fa } else { c3e6_1 }
    } else if obj_full {
        if maxhp <= curhp || yup < selfy { j226fa } else { false }   // branch B obj_full
    } else { c3e6_1 };
    if !to_main { return 7; }
    // LAB_227df (main)
    if rd_u8(p8 + 999) == 3 {
        let sel = if rd_u8(sim + 0xed69) == 0 {
            rd_i32(sim + 0xecc8 + s * 0x18).unwrap_or(-1) as u32
        } else {
            rd_i32(p5 + 0x420).unwrap_or(-1) as u32
        };
        let lane = rd_i32(p5 + 0x738).unwrap_or(-1) as u32;
        if sel < 5 && lane == sel && poke_fe2b10(p3, p5, p6, 5) {
            return if p3 < tune("pk_smallact_split", 0x21) as u64 { 3 } else { 2 };   // ★튜닝: 소액션 코드 분기 임계(p3<0x21→3)
        }
        return 0xf;   // 15 (poke-fail)
    }
    // zone-count (champ+999 != 3): serpent=노드거리²
    let t_c0 = rd_u64(p7 + 0xc0).unwrap_or(0);
    let t_d0 = rd_u64(p7 + 0xd0).unwrap_or(0);
    let selfe2 = dd7_slot128(sim, self_handle);
    if !ptr_ok(selfe2) { return -99; }   // ★readable VQ제거(poke selfe2)
    let s2x = rd_u64(selfe2 + 0x648).unwrap_or(0);
    let s2y = rd_u64(selfe2 + 0x650).unwrap_or(0);
    let s2cur = rd_u64(selfe2 + 0x658).unwrap_or(0);
    let s2mx = rd_u64(selfe2 + 0x610).unwrap_or(0);
    // ★2차 home (reason-4 교훈: HP체크 모든 side)
    if s2x <= xup && (side == 0 || s2x >= home_x1)
       && s2y <= yup && (side != 0 || s2y >= home_y1)
       && s2cur < s2mx {
        return 7;
    }
    if s2mx == 0 { return -99; }
    let obj_hp = s2cur.wrapping_mul(100) / s2mx;        // local_60
    let tmin = if t_c0 < t_d0 { t_c0 } else { t_d0 };
    let vobj = rd_u64(p6 + 8).unwrap_or(0) as usize;
    let m = rd_u64(vobj + 8).unwrap_or(0) as usize;
    let threat_scale = rd_i64(m + 0x12f8).unwrap_or(0);
    let geom = rd_u64(p6 + 0x10).unwrap_or(0) as usize;
    if tmin <= (threat_scale as u64).wrapping_mul(tune("pk_threat_mult", 5) as u64) {   // ★튜닝: 위협 스케일 배수 → fdae40
        return if !poke_fdae40(p8, p3, p5, p6, p7, 5) { 0x10 } else { 0xe };   // 16 / 14
    }
    let node = poke_node_resolve(sim, vt, 0x1c0, 0x1b8);
    if node != 0 {
        let other_c = poke_dist_count(l80, other, s, sim, geom, node);
        let self_c = poke_dist_count(l80, s, s, sim, geom, node);
        if tune("pk_obj_hp", 0x14) < obj_hp as i64 || other_c <= self_c {   // ★튜닝: 오브젝트 HP%(>20) → fdae40
            return if !poke_fdae40(p8, p3, p5, p6, p7, 5) { 0x10 } else { 0xe };   // 16 / 14
        }
    }
    7   // LAB_22d44
}

// ★SerpenPoke code3/2의 uVar6: cVar5=byte[param_6[1]+0x28]; cVar5∈{1,2,3}→DAT룩업(둘다 2), else 0. (DAT_1435f0318[0]=2, DAT_1435f2660[1]=2 라이브확정)
unsafe fn serpen_uvar6(p6: usize) -> u8 {
    let pp = rd_u64(p6 + 8).unwrap_or(0) as usize;
    if pp == 0 { return 0; }
    let cv5 = rd_u8(pp + 0x28);
    if (1..=3).contains(&cv5) { 2 } else { 0 }
}

// ★EpicPoke/SerpenPoke 출력 struct 전체 재현(code→aux 결정적, disasm+mpout 정답확정). p1=출력ptr, p2sj=서브저지 param_2(=subp+8), p6=param_6(=r15).
//   [+0]=code(qword) 항상. code별 aux(게임 write-set와 동일; 미기록 필드는 호출자/stale 그대로):
//     0x13: [+8]q=0, [+0x10]b=(epic?0:1), [+0x11]b=byte[p2sj]  /  0xb/0xc/0xe/0xf: [+8]b=0
//     3: [+8]b=(epic?2:uVar6)  /  2: [+8]b=0,[+9]b=(epic?2:uVar6),[+0xa]b=2  /  7,0xd,0x10: +0만
//   반환 false = 미지 code(대체불가→passthrough). 버퍼검증시 p1=스택버퍼ptr(진입스냅 복사후 호출).
// ★disc4 full-output writer: [p1]=code. code!=7이면 aux([p1+8]=active(*(p2+0x48)), [p1+0x10]=facet byte(*(p2+0x60)), [p1+0x11]=0). code7=코드만(0x206ee7b 직행, aux 미터치). p2sj=subp(disc4는 rdx직접 사용).
unsafe fn write_disc4_aux(p1: usize, code: i64, p2sj: usize) -> bool {
    if !writable(p1, 0x18) || !ptr_ok(p2sj) || !readable(p2sj + 0x68, 8) { return false; }
    std::ptr::write_unaligned(p1 as *mut u64, code as u64);
    if code != 7 {
        std::ptr::write_unaligned((p1 + 8) as *mut u64, rd_u64(p2sj + 0x48).unwrap_or(0));
        std::ptr::write_unaligned((p1 + 0x10) as *mut u8, rd_u8(p2sj + 0x60));
        std::ptr::write_unaligned((p1 + 0x11) as *mut u8, 0u8);
    }
    true
}
unsafe fn write_poke_aux(p1: usize, is_epic: bool, code: i64, p2sj: usize, p6: usize) -> bool {
    std::ptr::write_unaligned(p1 as *mut u64, code as u64);
    match code {
        0x13 => {
            std::ptr::write_unaligned((p1 + 8) as *mut u64, 0u64);
            std::ptr::write_unaligned((p1 + 0x10) as *mut u8, if is_epic { 0u8 } else { 1u8 });
            std::ptr::write_unaligned((p1 + 0x11) as *mut u8, rd_u8(p2sj));
        }
        0xb | 0xc | 0xe | 0xf => { std::ptr::write_unaligned((p1 + 8) as *mut u8, 0u8); }
        3 => { let v = if is_epic { 2u8 } else { serpen_uvar6(p6) }; std::ptr::write_unaligned((p1 + 8) as *mut u8, v); }
        2 => {
            std::ptr::write_unaligned((p1 + 8) as *mut u8, 0u8);
            let v = if is_epic { 2u8 } else { serpen_uvar6(p6) };
            std::ptr::write_unaligned((p1 + 9) as *mut u8, v);
            std::ptr::write_unaligned((p1 + 0xa) as *mut u8, 2u8);
        }
        7 | 0xd | 0x10 => {}   // +0만
        _ => return false,
    }
    true
}

// ★EpicBattle/SerpenBattle(disc10/12) 출력 struct 재현. ⚠정상매치 dead=미검증(disasm only). p2sj=서브저지 param_2(=subp+8).
//   code별: 0xa→[+8]q=*(param_2+8),[+0x10]w=1,[+0x12]b=0 / 0xc(epic)·0xf(serpen)=failcode→[+8]b=0 / 7→+0만.
unsafe fn write_battle_aux(p1: usize, code: i64, p2sj: usize) -> bool {
    std::ptr::write_unaligned(p1 as *mut u64, code as u64);
    match code {
        0xa => {
            std::ptr::write_unaligned((p1 + 8) as *mut u64, rd_u64(p2sj + 8).unwrap_or(0));
            std::ptr::write_unaligned((p1 + 0x10) as *mut u16, 1u16);
            std::ptr::write_unaligned((p1 + 0x12) as *mut u8, 0u8);
        }
        0xc | 0xf => { std::ptr::write_unaligned((p1 + 8) as *mut u8, 0u8); }
        7 => {}
        _ => return false,
    }
    true
}

// ★dd7700 action code 재현 (현 단계: 상단가드→7, 레인크립→4/7, 그외→2(기본/tail 미완)).
// 반환 -999 = 미예측(가드 실패 등). p3=param_3(r8).
unsafe fn my_dd7700_code(p2: usize, p3: u64, p4: usize, p5: usize, p6: usize, p7: usize, skip_cover: bool) -> i64 {
    if rd_u8(p2+0x18) != 0 { return 7; }                  // 상단 가드
    let l80 = rd_u64(p6).unwrap_or(0) as usize;
    let vobj = rd_u64(p6+8).unwrap_or(0) as usize;
    let geo = rd_u64(p6+0x10).unwrap_or(0) as usize;
    if !ptr_ok(l80) || !ptr_ok(vobj) || !ptr_ok(geo) { return -999; }
    let sim = rd_u64(l80).unwrap_or(0) as usize;
    if !ptr_ok(sim) { return -999; }
    let plan = rd_u8(p7+0x3e6);
    let side = rd_i64(p5+0x6a8).unwrap_or(-1);
    let lane = rd_i32(p5+0x738).unwrap_or(-1);
    if 4 < p3 && !skip_cover {   // ★B cover dedup: full의 engage경로(skip_cover=true)는 full이 이미 cover 비fire 확인(동일 게이트) → code의 cover 재스캔 생략=비트동일
        let cvar10: i64 = rd_u8(p2+0x19) as i64;
        // (plan&0xfe)==8 이고 *(p7+0x3e7)==cvar10 이면 LAB_141dd7aab(기본)로
        let to_default = (plan & 0xfe) == 8 && (rd_u8(p7+0x3e7) as i64 == cvar10);
        if !to_default && cvar10 == 2 {
            let s20 = dd7_slot20(sim);                        // ★호이스트: 현재틱(sim+0xed00)=호출 내 불변
            let lane_margin = tune("dd_lane_margin", 0x78);   // ★호이스트: 루프불변 튜닝계수
            let hdr = sim_hdr(sim);                           // ★호이스트: sim 헤더 1회 → cover 루프 slot48/a8 재사용
            // 프론티어 게이트 (VOBJ+0x28 byte ∉{1,2}일때)
            let vb = rd_u8(vobj+0x28);
            if vb.wrapping_sub(1) > 1 {
                let v1 = rd_u64(vobj+8).unwrap_or(0) as usize;
                let u19 = rd_u64(v1+0x8a8).unwrap_or(0);
                let l15 = rd_i64(v1+0x12f8).unwrap_or(0);
                let l15x30 = (l15 as u64).wrapping_mul(tune("dd_frontier_mult", 0x1e) as u64);   // ★튜닝: 프론티어 진척 배수(×30)
                let prog = if l15x30 <= u19 { u19 - l15x30 } else { 0 };
                if prog <= s20 as u64 { return 2; }   // bail → 기본
            }
            if lane > 2 && (side == 0 || side == 1) {
                let s = side as usize;
                let oidx = s*5 + (if lane==3 {1} else {0}) + 0x3f;
                let obj = rd_u64(l80 + oidx*8).unwrap_or(0) as usize;
                let (xlo,xhi,ylo,yhi): (u64,u64,u64,u64) = if side==0 {(0,64000,896000,960000)} else {(892000,960000,0,64000)};
                let proceed = obj==0
                    || (rd_i32(obj+0x68).unwrap_or(0)==0xd && rd_i32(obj+0x70).unwrap_or(0)==1)
                    || { let ox=rd_u64(obj+0x648).unwrap_or(0); let oy=rd_u64(obj+0x650).unwrap_or(0);
                         xlo<=ox && ox<=xhi && ylo<=oy && oy<=yhi };
                if proceed {
                    let lv21 = 1 - s;
                    let mut cands = [0usize; 5]; let mut ncand = 0usize;   // ★Vec→스택배열(힙할당 제거)
                    for k in 0..5usize {
                        let c = rd_u64(l80 + 0x1e0 + lv21*0x28 + k*8).unwrap_or(0) as usize;
                        if c != 0 { cands[ncand] = c; ncand += 1; }
                    }
                    if ncand != 0 {
                        let geo_side = geo + lv21*0x228;
                        let mut count: u64 = 0;
                        for &c in &cands[..ncand] {
                            let cx = rd_u64(c+0x648).unwrap_or(0);
                            let cy = rd_u64(c+0x650).unwrap_or(0);
                            let mut q: u64 = 0;
                            if dd7_f6f720_m2(vobj, cx, cy) {
                                let id = rd_u64(c+0x5a8).unwrap_or(0);
                                let empty = dd7_slot48_h(&hdr, s, id);
                                q = 1;
                                if !empty {
                                    let resolved = dd7_slot_a8_h(&hdr, id);   // ★empty시 slot_a8 O(n)스캔 생략
                                    if resolved == 0 { q = 0; }
                                    else {
                                        let rlane = rd_i32(resolved+0x738).unwrap_or(0) as usize;
                                        let thr = rd_i64(geo_side + (rlane+0x3c)*8).unwrap_or(0);
                                        q = if s20 <= thr + lane_margin { 1 } else { 0 };   // (s20/lane_margin = 호이스트됨)
                                    }
                                }
                            }
                            count += q;
                        }
                        if count >= tune("dd_cover_count", 2) as u64 {   // ★튜닝: 커버 발화 적군 카운트
                            let team_units = rd_u64(p5+0x6a0).unwrap_or(0);
                            let team = dd7_slot128(sim, team_units);
                            if team != 0 {
                                let tcnt = rd_u64(team+0x610).unwrap_or(0);
                                if tcnt != 0 {
                                    let depth = rd_i64(team+0x658).unwrap_or(0);
                                    let ratio = (depth as u64).wrapping_mul(100) / tcnt;
                                    let mut code = 4i64;
                                    if (ratio as i64) < tune("dd_ratio_thr", 0x33) {   // ★튜닝: 커버 비율 임계(<51)
                                        let f = rd_i64(geo + 0x60 + s*0x228).unwrap_or(0);
                                        code = if f > tune("dd_facet_thr", 999) { 7 } else { 4 };   // ★튜닝: 페이즈 게이트 임계(>999)
                                    }
                                    return code;
                                }
                            }
                            return -999;   // team 해석 실패 = 가드(게임은 panic 경로)
                        }
                    }
                }
            }
        }
    }
    // ══ 교전 tail (branch B 경로; plan!=8). STAGE1/2/4/5 결정론 골격 재현 ══
    // ★DD7_TAIL_OK=false: engage-tail 재현에 잠복 AV(0.4.13_5 교전서 크래시 격리확정) → 게이트. 캡처는 actual code만(미예측 -999).
    //   tail AV 디버그(replay+SEH/instrument) 후 true로 해제. 초기 code(2/4/7)는 위에서 이미 검증됨.
    if !DD7_TAIL_OK { return -999; }
    // (STAGE3 COUNT/STAGE6 6/7 = f22e80 재현 필요 → Ghidra 복구 대기. 그 전까진 deep zone서 보수적 2.)
    if side != 0 && side != 1 { return 2; }
    let s = side as usize;
    let f = rd_u8(p2+0x19) as usize;
    let roleoff = if f==0 {0usize} else if f==1 {0x28} else {0x50};
    // STAGE 1: 레인활성 게이트. *(i32)(side*0x228 + GEO + roleoff) != 1 → 2
    let rolerec = s*0x228 + geo + roleoff;
    if rd_i32(rolerec).unwrap_or(0) != 1 { return 2; }   // ★readable VQ제거(rd_i32 None=0=fault흡수)
    // STAGE 2: 타깃 해석 = slot140(sim, *(rolerec+8)). 0이면 2. self=slot128(sim, *(p5+0x6a0)).
    let vtab = rd_u64(l80+8).unwrap_or(0) as usize;
    let resolver = if ptr_ok(vtab) { rd_u64(vtab+0x140).unwrap_or(0) as usize } else { 0 };
    if !ptr_ok(resolver) { return 2; }
    let tgt_handle = rd_u64(rolerec+8).unwrap_or(0);
    let rf: G2 = core::mem::transmute(resolver);
    let target = if PERF_ON.load(Ordering::Relaxed) {
        let _t = Instant::now(); let r = rf(sim, tgt_handle as usize) as usize;
        DD7_RESOLVE_NS.fetch_add(_t.elapsed().as_nanos() as u64, Ordering::Relaxed); DD7_RESOLVE_N.fetch_add(1, Ordering::Relaxed); r
    } else { rf(sim, tgt_handle as usize) as usize };
    if !ptr_ok(target) { return 2; }   // ★readable VQ제거(아래 tx/ty rd_u64 fault-safe)
    let selfobj = dd7_slot128(sim, rd_u64(p5+0x6a0).unwrap_or(0));
    if !ptr_ok(selfobj) { return 2; }   // ★readable VQ제거(panic가드=ptr_ok, 좌표 rd_u64 fault-safe)
    let (tx, ty) = (rd_u64(target+0x648).unwrap_or(0), rd_u64(target+0x650).unwrap_or(0));
    let (selfx, selfy) = (rd_u64(selfobj+0x648).unwrap_or(0), rd_u64(selfobj+0x650).unwrap_or(0));
    // STAGE 3: window(WLO/WHI) + COUNT = my_f22e80_count (RngSim는 entry state=STAGE1/2 RNG무소비라 일치).
    let count_survivors: u64 = {
        let a380 = rd_i64(p5+0x380).unwrap_or(0); let a218 = rd_i64(p5+0x218).unwrap_or(0);
        // ★dd7700 정확식(0x1418aeea3): uVar20=(u64)(a380*a218)/1000(풀정밀 unsigned div). pre-shift(>>3)*magic 패턴 폐기(트리플floor로 t -1 오차→윈도우 widen→rejection어긋남).
        let t = ((a380.wrapping_mul(a218) as u64) / 1000).min(100);
        let half = 0x384u64.wrapping_sub(t.wrapping_mul(9)) >> 1;
        let (wlo, whi) = (0x3e8u64.wrapping_sub(half), 0x3e8u64.wrapping_add(half));
        match RngSim::new(p4) { Some(mut r) => my_f22e80_count(&mut r, l80, geo, p5, p7, sim, wlo, whi, tx, ty, 150000), None => 0 }
    };
    // STAGE 4: NEAR 카운트(자기편 5칸 중 self/target 근접). GATE D서 COUNT와 비교.
    let mut near_cnt: u64 = 0;
    for k in 0..5usize {
        let c = rd_u64(l80 + s*0x28 + 0x1e0 + k*8).unwrap_or(0) as usize;
        if c == 0 { continue; }   // ★readable VQ제거(STAGE4 근접루프, 좌표 rd_u64 fault-safe)
        let (cx, cy) = (rd_u64(c+0x648).unwrap_or(0), rd_u64(c+0x650).unwrap_or(0));
        let near_d = tune("dd_near_dist", 0x53d1ac0) as u64;   // ★튜닝: 근접 카운트 거리²(>>8)
        if (sqd(cx,cy,selfx,selfy)>>8) <= near_d || (sqd(cx,cy,tx,ty)>>8) <= near_d { near_cnt += 1; }
    }
    // STAGE 5: 앵커 + 거리게이트 C/D/E
    let mut anchor = rd_u64(l80 + s*8 + f*0x20 + 0x180).unwrap_or(0) as usize;
    if anchor == 0 { anchor = rd_u64(l80 + s*8 + f*0x20 + 0x190).unwrap_or(0) as usize; }
    let nexus = rd_u64(l80 + (s + 0x2e)*8).unwrap_or(0) as usize;
    if !ptr_ok(nexus) { return 2; }   // ★readable VQ제거(panic가드=ptr_ok)
    if anchor == 0 { anchor = nexus; }   // len!=0 && anchor==0 → f9c6d0 보정 생략(추후)
    if !ptr_ok(anchor) { return 2; }   // ★readable VQ제거(anchor 유효성만, 좌표 rd_u64 fault-safe)
    let (nx, ny) = (rd_u64(nexus+0x648).unwrap_or(0), rd_u64(nexus+0x650).unwrap_or(0));
    let (ax, ay) = (rd_u64(anchor+0x648).unwrap_or(0), rd_u64(anchor+0x650).unwrap_or(0));
    // GATE C: d(nexus,target) < d(nexus,anchor) → 2
    if sqd(nx,ny,tx,ty) < sqd(nx,ny,ax,ay) { return 2; }
    // GATE D: near_cnt >= COUNT(f22e80) → 2
    if near_cnt >= count_survivors { return 2; }
    // GATE E: (d(anchor,target)>>8) < 0x6ba9301 → 2
    if (sqd(ax,ay,tx,ty) >> 8) < tune("dd_gatee_dist", 0x6ba9301) as u64 { return 2; }   // ★튜닝: GATE E 앵커-타깃 거리²(>>8)
    // ══ STAGE 6: 교전/귀환 결정 (코드 2/6/7). COUNT=count_survivors(f22e80 재현). ══
    DD7_DEEP.fetch_add(1, Ordering::Relaxed);
    let plan = rd_u8(p7+0x3e6);
    // iVar2: piVar26=side*0x228+geo; F!=0면 reindex(+0x50 if p4==2 else +0x28); +0x20
    let piadj = s*0x228 + geo + (if f==0 {0usize} else if (p4 as u32)==2 {0x50} else {0x28});
    let ivar2 = rd_i32(piadj + 0x20).unwrap_or(0);
    let r_self = sim + 0x860;   // ★vt+0x168(this=sim)=sim+0x860 (디컴 confirm: plVar22=plStack_88=sim, NOT self/target). 옛 selfobj는 mis-RE
    let (bl, route_8679): (bool, bool) = match plan {
        0 => (f==2, f==0),
        1 => (f==0, f==0),
        _ => {
            let bl = if rd_u64(r_self+0x190).unwrap_or(0) != 0 { f==2 }
                     else { f==0 && rd_u64(r_self+0x1c0).unwrap_or(0) != 0 };
            (bl, f==0)
        }
    };
    let term_86dd = if (ivar2 as i64) > tune("dd_ivar2_thr", 2) { 7i64 } else { 6 };   // ★튜닝: STAGE6 진척단계 임계(iVar2>2→7)
    let term_872d = {                                    // 872d: anchor type2 + target → 7 else 2
        if rd_i32(anchor+0x68).unwrap_or(0) != 2 { 2i64 }
        else if rd_i64(anchor+0x88).unwrap_or(0) == 0 { 2 } else { 7 }
    };
    // 라우팅: ref 결정 → 869a, 아니면 86c1
    let ref_pp: Option<usize> = if route_8679 {
        let r = sim + 0x860;   // ★vt+0x168(this=sim) (디컴 confirm; 옛 target은 mis-RE)                          // LAB_8679: vt+0x168(target)
        if rd_u64(r+0x1c0).unwrap_or(0) == 0 { None } else { Some(r+0x1b8) }
    } else if f != 2 {
        None                                             // LAB_8598: f!=2 → 86c1
    } else {
        let r = sim + 0x860;   // ★vt+0x168(this=sim) (디컴 confirm; 옛 target은 mis-RE)
        if rd_u64(r+0x190).unwrap_or(0) == 0 { None } else { Some(r+0x188) }
    };
    if let Some(refp) = ref_pp {
        // LAB_869a: e = resolver(target, *ref)
        let refv = rd_u64(refp).unwrap_or(0);
        let e = rf(sim, refv as usize) as usize;   // ★resolver(this=sim, *ref) (디컴 confirm: 옛 rf(target,..)=핸들deref에 엔티티 넘겨 AV였음 = 크래시 근본원인)
        if e != 0 {
            if rd_u8(selfobj) != 0 {
                return if bl { term_86dd } else { term_872d };
            } else {
                // selfobj[0]==0: bl이면 (n<2 && COUNT<=3 && *(e+n*0x18+0x38)!=0)→2 else 86dd; 아니면 872d
                if bl {
                    let n = rd_u64(selfobj+8).unwrap_or(0);
                    if (n as i64) < tune("dd_n_thr", 2) && count_survivors <= tune("dd_survivor_thr", 3) as u64 && rd_u64(e + (n as usize)*0x18 + 0x38).unwrap_or(0) != 0 { return 2; }   // ★튜닝: 슬롯수/생존자수 임계
                    return term_86dd;
                }
                return term_872d;
            }
        }
        // e==0 → 86c1로 폴
    }
    // LAB_86c1: !bl → 872d; bl이면 COUNT<=3→2 else 86dd
    if !bl { return term_872d; }
    if count_survivors <= tune("dd_survivor_thr", 3) as u64 { return 2; }   // ★튜닝: 생존자수 임계
    term_86dd
}

// ★dd7700(0x18ae610) 충실재현 — 전체출력(code@+0 + aux +8/+9/+0xa). Some(())=재현완료(out에 write) / None=passthrough(미포팅 경로/가드).
// 포팅범위(2026-06-19): early(7) + cover(4/7,[+8]=2) + else-branch main code-2(LAB_af3d9: [+8]=local_90 [+9]=F [+0xa]=local_58).
// 미포팅(None): plan==8(epic) 분기 / iVar12==1 engage(CAND_FILTER→6/7). day-11 plan=255 else-branch code-2 dominant.
unsafe fn my_dd7700_full(out: usize, p2: usize, p3: u64, p4: usize, p5: usize, p6: usize, p7: usize) -> Option<bool> {   // ★레버: Some(true)=engage(CAND_FILTER RNG소비)/Some(false)=cover·main(RNG 0 draw)→rng_final skip/None=passthrough
    let _pg = perf_guard(1);
    if !writable(out, 0x18) { return None; }
    // EARLY GUARD: byte[param_2+0x18]!=0 → *out=7
    if rd_u8(p2 + 0x18) != 0 { std::ptr::write_unaligned(out as *mut u64, 7u64); return Some(false); }
    let l80 = rd_u64(p6).unwrap_or(0) as usize;
    let vobj = rd_u64(p6 + 8).unwrap_or(0) as usize;
    let geo = rd_u64(p6 + 0x10).unwrap_or(0) as usize;
    if !ptr_ok(l80) || !ptr_ok(vobj) || !ptr_ok(geo) { return None; }
    let sim = rd_u64(l80).unwrap_or(0) as usize;   // robj==sim (l80[0])
    let vt  = rd_u64(l80 + 8).unwrap_or(0) as usize;
    if !ptr_ok(sim) || !ptr_ok(vt) { return None; }
    let plan = rd_u8(p7 + 0x3e6);
    let side = rd_i64(p5 + 0x6a8).unwrap_or(-1);
    if side != 0 && side != 1 { return None; }
    // ★라이너 포탑/인원수 보정(2026-06-23): self가 적포탑밑/수적열세면 code7(귀환) — early-guard(위 5168)와 동일 포맷(out+0=7, aux불요=검증된 dd7full 코드). dd7700=매라이너매프레임이라 라이너 다이브/불리교전 직접차단. RNG writeback(mp_capture 8102)은 별개 함수라 출력만 바꿔도 draw수 불변=RNG state 무손상(게임플레이만 의도분기). 기본(tower_threat=0&&numbers=0)=동작보존.
    if TOWER_THREAT.load(Ordering::Relaxed) > 0 || NUMBERS_MARGIN.load(Ordering::Relaxed) > 0 || NUMBERS_THREAT.load(Ordering::Relaxed) > 0 {
        let selfobj = dd7_slot128(sim, rd_u64(p5 + 0x6a0).unwrap_or(0));
        if ptr_ok(selfobj) && laner_should_retreat(p6, side, selfobj, p5) {
            std::ptr::write_unaligned(out as *mut u64, 7u64);
            return Some(false);
        }
    }
    let s = side as usize;
    let lane = rd_i32(p5 + 0x738).unwrap_or(-1);
    let f = rd_u8(p2 + 0x19) as usize;   // F = byte[param_2+0x19]
    let s20 = dd7_slot20(sim);                        // ★호이스트: 현재틱(sim+0xed00)=호출 내 불변. 후보루프서 재읽기 제거
    let lane_margin = tune("dd_lane_margin", 0x78);   // ★호이스트: 루프불변 튜닝계수(per-candidate 조회 제거)
    let hdr = sim_hdr(sim);                           // ★호이스트: sim 헤더 1회 → cover/main 루프 slot48/a8 재사용

    // ── COVER BLOCK (4 < param_3) ──
    if 4 < p3 {
        let cvar10 = f;   // (plan&0xfe)==8이면 p7[999]==F시 main으로; 아니면 cVar10=F
        let go_main = (plan & 0xfe) == 8 && rd_u8(p7 + 999) as usize == cvar10;
        if !go_main && cvar10 == 2 {
            // cVar10==2: frontier + lane + survivor 게이트. 4/7 출력 or main 폴.
            let mut cover_done = false;
            // frontier gate: vobj+0x28 ∉ {1,2}일때 prog<=slot20 → main(폴)
            let vb = rd_u8(vobj + 0x28);
            let frontier_bail = if vb.wrapping_sub(1) > 1 {
                let v1 = rd_u64(vobj + 8).unwrap_or(0) as usize;
                let u19 = rd_u64(v1 + 0x8a8).unwrap_or(0);
                let l15 = rd_i64(v1 + 0x12f8).unwrap_or(0);
                let l15x30 = (l15 as u64).wrapping_mul(tune("dd_frontier_mult", 0x1e) as u64);   // ★튜닝: 프론티어 진척 배수(×30)
                let prog = if l15x30 <= u19 { u19 - l15x30 } else { 0 };
                prog <= s20 as u64
            } else { false };
            if !frontier_bail && lane > 2 {
                let oidx = s * 5 + (if lane == 3 { 1 } else { 0 }) + 0x3f;
                let obj = rd_u64(l80 + oidx * 8).unwrap_or(0) as usize;
                let (xlo, xhi, ylo, yhi): (u64, u64, u64, u64) = if side == 0 { (0, 64000, 896000, 960000) } else { (892000, 960000, 0, 64000) };
                let proceed = obj == 0
                    || (rd_i32(obj + 0x68).unwrap_or(0) == 0xd && rd_i32(obj + 0x70).unwrap_or(0) == 1)
                    || { let ox = rd_u64(obj + 0x648).unwrap_or(0); let oy = rd_u64(obj + 0x650).unwrap_or(0);
                         xlo <= ox && ox <= xhi && ylo <= oy && oy <= yhi };
                if proceed {
                    let lv21 = 1 - s;
                    let mut cands = [0usize; 5]; let mut ncand = 0usize;   // ★Vec→스택배열(힙할당 제거, 후보 ≤5)
                    for k in 0..5usize { let c = rd_u64(l80 + 0x1e0 + lv21 * 0x28 + k * 8).unwrap_or(0) as usize; if c != 0 { cands[ncand] = c; ncand += 1; } }
                    if ncand != 0 {
                        let geo_side = geo + lv21 * 0x228;
                        let mut count: u64 = 0;
                        for &c in &cands[..ncand] {
                            let cx = rd_u64(c + 0x648).unwrap_or(0);
                            let cy = rd_u64(c + 0x650).unwrap_or(0);
                            let mut q: u64 = 0;
                            if dd7_f6f720_m2(vobj, cx, cy) {
                                let id = rd_u64(c + 0x5a8).unwrap_or(0);
                                let empty = dd7_slot48_h(&hdr, s, id);
                                q = 1;
                                if !empty {
                                    let resolved = dd7_slot_a8_h(&hdr, id);   // ★empty시 slot_a8 O(n)스캔 생략
                                    if resolved == 0 { q = 0; }
                                    else {
                                        let rlane = rd_i32(resolved + 0x738).unwrap_or(0) as usize;
                                        let thr = rd_i64(geo_side + (rlane + 0x3c) * 8).unwrap_or(0);
                                        q = if s20 <= thr + lane_margin { 1 } else { 0 };   // (s20/lane_margin = 호이스트됨)
                                    }
                                }
                            }
                            count += q;
                        }
                        if count >= tune("dd_cover_count", 2) as u64 {   // ★튜닝: 커버 발화 적군 카운트
                            let team = dd7_slot128(sim, rd_u64(p5 + 0x6a0).unwrap_or(0));
                            if team != 0 {
                                let tcnt = rd_u64(team + 0x610).unwrap_or(0);
                                if tcnt != 0 {
                                    let depth = rd_i64(team + 0x658).unwrap_or(0);
                                    let ratio = (depth as u64).wrapping_mul(100) / tcnt;
                                    let mut code = 4i64;
                                    if (ratio as i64) < tune("dd_ratio_thr", 0x33) { let fv = rd_i64(geo + 0x60 + s * 0x228).unwrap_or(0); code = if fv > tune("dd_facet_thr", 999) { 7 } else { 4 }; }   // ★튜닝: 커버 비율/페이즈 임계
                                    std::ptr::write_unaligned(out as *mut u64, code as u64);
                                    std::ptr::write_unaligned((out + 8) as *mut u8, 2u8);   // cover [+8]=2
                                    cover_done = true;
                                } else { return None; }   // team panic 경로
                            } else { return None; }
                        }
                    }
                }
            }
            if cover_done { return Some(false); }
            // else: bail → main body (LAB_ae9bb)
        }
        // go_main / cvar10!=2 / cover bail → main
    }

    // ── MAIN BODY (LAB_ae9bb) ──
    if plan == 8 { return None; }   // epic 분기(uVar11=='\b') 미포팅 (day-11 plan=255)
    // else-branch (LAB_aeb5a): plan != 8
    let self_handle = rd_u64(p5 + 0x6a0).unwrap_or(0);
    let selfe = dd7_slot128(sim, self_handle);
    if !ptr_ok(selfe) { return None; }   // ★readable VQ제거(아래 selfx/selfy rd_u64 fault-safe, dd7700 메인바디 per-call)
    let local_90: u8 = (lane == 1) as u8;   // b8
    let other = 1 - s;
    let geom_other = other * 0x228 + geo;    // local_88 (threshold block)
    let (selfx, selfy) = (rd_u64(selfe + 0x648).unwrap_or(0), rd_u64(selfe + 0x650).unwrap_or(0));
    let mut local_58: u8 = 2;                // b10 (no-match 기본=2)
    let uvar18: u8 = f as u8;                // b9 = F
    // (1-side) 후보 순회: self근접 + (empty || resolved&threshold) → 매치시 local_58=웨이포인트 sign
    let main_near = tune("dd_main_near_dist", 0x53d1ac1) as u64;   // ★호이스트: 루프불변
    let mut cands = [0usize; 5]; let mut ncand = 0usize;          // ★Vec→스택배열
    for k in 0..5usize { let c = rd_u64(l80 + 0x1e0 + other * 0x28 + k * 8).unwrap_or(0) as usize; if c != 0 { cands[ncand] = c; ncand += 1; } }
    for &c in &cands[..ncand] {
        let (cx, cy) = (rd_u64(c + 0x648).unwrap_or(0), rd_u64(c + 0x650).unwrap_or(0));
        if (sqd(cx, cy, selfx, selfy) >> 8) < main_near {
            let id = rd_u64(c + 0x5a8).unwrap_or(0);
            let empty = dd7_slot48_h(&hdr, s, id);
            let pass = empty || {   // ★empty시 slot_a8 O(n)스캔 생략(단락평가)
                let resolved = dd7_slot_a8_h(&hdr, id);
                resolved != 0 && {
                    let rlane = rd_i32(resolved + 0x738).unwrap_or(0) as usize;
                    let thr = rd_i64(geom_other + (rlane + 0x3c) * 8).unwrap_or(0);
                    s20 <= thr + lane_margin   // (s20/lane_margin = 호이스트됨)
                }
            };
            if pass {
                let mut wp = s * 0x228 + geo;
                if f != 0 { wp += if f == 1 { 0x28 } else { 0x50 }; }
                let wpv = rd_i64(wp + 0x18).unwrap_or(0);
                local_58 = if wpv < 0 { 2 } else { 0 };
                break;
            }
        }
    }
    // LAB_aeea3: role-check. iVar12 = *(int)(side*0x228+geo + roleoff(F)). !=1 → af3d9(code2+aux); ==1 → engage(미포팅)
    let roleoff = if f == 0 { 0usize } else if f == 1 { 0x28 } else { 0x50 };
    let ivar12 = rd_i32(s * 0x228 + geo + roleoff).unwrap_or(0);
    if ivar12 == 1 {
        // ── ENGAGE 경로(CAND_FILTER → af3d9 code2 / af65b code6/7) ──
        // +0 코드 = my_dd7700_code(검증된 STAGE6, dd7cmp 40/40). code2(dominant)는 candidate-loop aux(local_90/uvar18/local_58).
        // code 6/7(rare engage decision)은 aux(cVar10 route)가 engage블록 내부값이라 None(passthrough). dd7full로 검증.
        let code = if PERF_ON.load(Ordering::Relaxed) {
            let _t = Instant::now(); let c = my_dd7700_code(p2, p3, p4, p5, p6, p7, true);   // ★skip_cover=true(full이 cover 비fire 확인됨)
            DD7_CODE_NS.fetch_add(_t.elapsed().as_nanos() as u64, Ordering::Relaxed); DD7_CODE_N.fetch_add(1, Ordering::Relaxed); c
        } else { my_dd7700_code(p2, p3, p4, p5, p6, p7, true) };
        if code == 2 {
            std::ptr::write_unaligned(out as *mut u64, 2u64);
            std::ptr::write_unaligned((out + 8) as *mut u8, local_90);
            std::ptr::write_unaligned((out + 9) as *mut u8, uvar18);
            std::ptr::write_unaligned((out + 0xa) as *mut u8, local_58);
            return Some(true);   // ★engage 경로 = CAND_FILTER RNG 소비 → rng_final 필요
        }
        return None;   // engage 6/7 passthrough(rare)
    }
    // LAB_af3d9: code 2 + aux (iVar12 != 1 직접경로)
    std::ptr::write_unaligned(out as *mut u64, 2u64);
    std::ptr::write_unaligned((out + 8) as *mut u8, local_90);
    std::ptr::write_unaligned((out + 9) as *mut u8, uvar18);
    std::ptr::write_unaligned((out + 0xa) as *mut u8, local_58);
    Some(false)   // ★main 경로(iVar12!=1) = RNG 0 draw → rng_final skip
}

// ★dd7700 RNG 소비 재현(대체모드 RNG-sync). dd7700의 유일 RNG소비 = CAND_FILTER(FUN_141fecbe0): iVar12==1 && target!=0 일때
//   non-null cand(레인0..5, team=1-side)당 gen_range(lo,hi) 1회. lo/hi = ego*tactic 윈도우(STAGE3 동일식).
//   반환 Some((final_idx, refills, buf)) = p4 RNG의 예측 after-state / None = draw없음(상태불변). 게임함수=resolver(vt0x140) 1회(RNG-free, churn-free).
unsafe fn my_dd7700_rng_final(p4: usize, p2: usize, p3: u64, p5: usize, p6: usize, p7: usize) -> Option<(u64, u64, [u32; 64])> {
    DD7_RNG_N.store(0, Ordering::Relaxed);
    if !ptr_ok(p4) { return None; }   // ★readable VQ제거(p4=RNG state, 본문 RngSim/wr_* fault-safe·per-dd7700)
    if rd_u8(p2 + 0x18) != 0 { return None; }                 // early guard: no RNG
    let l80 = rd_u64(p6).unwrap_or(0) as usize;
    let vobj = rd_u64(p6 + 8).unwrap_or(0) as usize;
    let geo = rd_u64(p6 + 0x10).unwrap_or(0) as usize;
    if !ptr_ok(l80) || !ptr_ok(vobj) || !ptr_ok(geo) { return None; }
    let sim = rd_u64(l80).unwrap_or(0) as usize;
    let vt = rd_u64(l80 + 8).unwrap_or(0) as usize;
    if !ptr_ok(sim) || !ptr_ok(vt) { return None; }
    let plan = rd_u8(p7 + 0x3e6);
    let side = rd_i64(p5 + 0x6a8).unwrap_or(-1);
    if side != 0 && side != 1 { return None; }
    let s = side as usize;
    let lane = rd_i32(p5 + 0x738).unwrap_or(-1);
    let f = rd_u8(p2 + 0x19) as usize;
    // ★호이스트 + my_dd7700_full과 동일 튜닝값 사용(RNG-sync): cover-fire 예측이 실제 judge와 같은 임계를 써야 desync 없음.
    //   기본값(0x1e/0x78/2)에선 tune이 그대로 반환 → 검증된 DIFF=0 보존. 튜닝시 full과 일관.
    let s20 = dd7_slot20(sim);
    let lane_margin = tune("dd_lane_margin", 0x78);
    let frontier_mult = tune("dd_frontier_mult", 0x1e) as u64;
    let cover_count = tune("dd_cover_count", 2) as u64;
    let hdr = sim_hdr(sim);                            // ★호이스트: sim 헤더 1회 → cover 루프 slot48/a8 재사용
    DD7_RNG_DBG.store(plan as u64 | (f as u64) << 8, Ordering::Relaxed);
    // ── COVER BLOCK(4<p3) 검출: 발화시 main body 미도달 → CAND_FILTER 미실행(cover RNG 무소비) → None(0 draw). my_dd7700_full cover-fire와 동일조건. ──
    if 4 < p3 {
        let go_main = (plan & 0xfe) == 8 && rd_u8(p7 + 999) as usize == f;
        if !go_main && f == 2 {
            let vb = rd_u8(vobj + 0x28);
            let frontier_bail = if vb.wrapping_sub(1) > 1 {
                let v1 = rd_u64(vobj + 8).unwrap_or(0) as usize;
                let u19 = rd_u64(v1 + 0x8a8).unwrap_or(0);
                let l15 = rd_i64(v1 + 0x12f8).unwrap_or(0);
                let l15x30 = (l15 as u64).wrapping_mul(frontier_mult);   // full과 동일 튜닝값
                let prog = if l15x30 <= u19 { u19 - l15x30 } else { 0 };
                prog <= s20 as u64
            } else { false };
            if !frontier_bail && lane > 2 {
                let oidx = s * 5 + (if lane == 3 { 1 } else { 0 }) + 0x3f;
                let obj = rd_u64(l80 + oidx * 8).unwrap_or(0) as usize;
                let (xlo, xhi, ylo, yhi): (u64, u64, u64, u64) = if side == 0 { (0, 64000, 896000, 960000) } else { (892000, 960000, 0, 64000) };
                let proceed = obj == 0
                    || (rd_i32(obj + 0x68).unwrap_or(0) == 0xd && rd_i32(obj + 0x70).unwrap_or(0) == 1)
                    || { let ox = rd_u64(obj + 0x648).unwrap_or(0); let oy = rd_u64(obj + 0x650).unwrap_or(0);
                         xlo <= ox && ox <= xhi && ylo <= oy && oy <= yhi };
                if proceed {
                    let lv21 = 1 - s;
                    let geo_side = geo + lv21 * 0x228;
                    let mut count = 0u64; let mut any = false;
                    for k in 0..5usize {
                        let c = rd_u64(l80 + 0x1e0 + lv21 * 0x28 + k * 8).unwrap_or(0) as usize;
                        if c == 0 { continue; }
                        any = true;
                        let cx = rd_u64(c + 0x648).unwrap_or(0); let cy = rd_u64(c + 0x650).unwrap_or(0);
                        if dd7_f6f720_m2(vobj, cx, cy) {
                            let id = rd_u64(c + 0x5a8).unwrap_or(0);
                            let empty = dd7_slot48_h(&hdr, s, id);
                            let mut q = true;
                            if !empty {
                                let resolved = dd7_slot_a8_h(&hdr, id);   // ★empty시 slot_a8 O(n)스캔 생략
                                if resolved == 0 { q = false; }
                                else { let rlane = rd_i32(resolved + 0x738).unwrap_or(0) as usize;
                                    let thr = rd_i64(geo_side + (rlane + 0x3c) * 8).unwrap_or(0);
                                    q = s20 <= thr + lane_margin; }   // full과 동일 튜닝값(+호이스트)
                            }
                            if q { count += 1; }
                        }
                    }
                    if any && count >= cover_count { return None; }   // cover fires(4/7) → main 미도달 → 0 draw (full과 동일 튜닝값)
                }
            }
        }
    }
    if plan == 8 { return None; }                             // plan8 분기 별도(passthrough)
    // role check (iVar12). !=1 → CAND_FILTER 미도달
    let roleoff = if f == 0 { 0usize } else if f == 1 { 0x28 } else { 0x50 };
    DD7_RNG_PI14.store(s * 0x228 + geo + roleoff, Ordering::Relaxed);   // role record addr (exit 재독용)
    if rd_i32(s * 0x228 + geo + roleoff).unwrap_or(0) != 1 { return None; }
    DD7_RNG_DBG.fetch_or(1 << 12, Ordering::Relaxed);         // iVar12==1
    // target resolve (vt[0x140](robj, *(pi14+8))). 0 → af3d9 early, CAND_FILTER 미도달
    let resolver = rd_u64(vt + 0x140).unwrap_or(0) as usize;
    if !ptr_ok(resolver) { return None; }
    let tgt_handle = rd_u64(s * 0x228 + geo + roleoff + 8).unwrap_or(0);
    DD7_RNG_TH0.store(tgt_handle, Ordering::Relaxed);         // entry tgt_handle
    let rf: G2 = core::mem::transmute(resolver);
    let target = rf(sim, tgt_handle as usize) as usize;
    if target == 0 { return None; }
    DD7_RNG_DBG.fetch_or(1 << 13 | 1 << 15, Ordering::Relaxed);   // target!=0 + reached CAND_FILTER
    // CAND_FILTER 도달: lo/hi 윈도우(STAGE3 동일)
    let a380 = rd_i64(p5 + 0x380).unwrap_or(0);
    let a218 = rd_i64(p5 + 0x218).unwrap_or(0);
    // ★dd7700 정확식(0x1418aeea3): uVar20=(u64)(a380*a218)/1000(풀정밀). pre-shift(>>3)*magic>>64>>4 패턴은 트리플floor로 t를 최대 1 과소→half 과대→윈도우 widen→동일draw수에도 rejection어긋나 exit DIFF. 정확 /1000로 교체.
    let t = ((a380.wrapping_mul(a218) as u64) / 1000).min(100);
    let half = 0x384u64.wrapping_sub(t.wrapping_mul(9)) >> 1;
    let (lo, hi) = (0x3e8u64.wrapping_sub(half), 0x3e8u64.wrapping_add(half));
    // non-null cand 수(레인 0..5, candtable=l80+0x1e0+(1-s)*0x28). 각 1 draw.
    let other = 1 - s;
    DD7_RNG_CTAB.store(l80 + 0x1e0 + other * 0x28, Ordering::Relaxed);   // 진단: exit 재독용
    let mut rng = RngSim::new(p4)?;
    let mut n = 0u64;
    let mut cmask = 0u64;
    for l in 0..5usize {
        let cand = rd_u64(l80 + 0x1e0 + other * 0x28 + l * 8).unwrap_or(0);
        if cand != 0 { rng.gen_range(lo, hi)?; n += 1; cmask |= 1 << l; }
    }
    DD7_RNG_N.store(n, Ordering::Relaxed);
    DD7_RNG_LO.store(lo, Ordering::Relaxed); DD7_RNG_HI.store(hi, Ordering::Relaxed);
    DD7_RNG_I0.store(rd_u64(p4 + 0x100).unwrap_or(0), Ordering::Relaxed); DD7_RNG_CMASK.store(cmask, Ordering::Relaxed);
    Some((rng.idx, rng.refills, rng.buf))
}

// ★FUN_1420e88a0 필터 재현 → count(=draw 여부/range). 게터 전부 필드읽기(e88a0.txt+capstone 확정).
//   count = 후보 sublist엔트리 중 비교집합(p7[4])과 identity(memcmp)일치 & 우선순위(>local_50) & thr(≤param_4[0x710]) 통과 수.
//   local_50 = 후보(cand리스트 param_4+0x3c8) 중 priority(*+0x188)<4 의 최대(없으면 0).
unsafe fn my_e88a0_count(p4: usize, p7: usize) -> Option<u64> {
    let cand_base = rd_u64(p4 + 0x3c8)? as usize;
    let cand_cnt = rd_u64(p4 + 0x3d0)?;
    let threshold = rd_u64(p4 + 0x710)?;
    if cand_base == 0 || cand_cnt == 0 || cand_cnt > 64 { return Some(0); }
    // local_50 = max{ priority<4 } over candidates, else 0
    let mut local_50: u64 = 0;
    for i in 0..cand_cnt as usize {
        let obj = rd_u64(cand_base + i * 0x10)? as usize;
        if !ptr_ok(obj) { continue; }   // ★readable VQ제거(직후 rd_u64(obj+0x188)? fault-safe)
        let pri = rd_u64(obj + 0x188)?;
        if pri < 4 && pri > local_50 { local_50 = pri; }
    }
    // 비교집합(local_58 = param_7[4] = *(p7+0x20)): base=*(+8), cnt=*(+0x10)
    if !ptr_ok(p7) { return Some(0); }   // ★readable VQ제거(직후 rd_u64(p7+0x20)? fault-safe)
    let local_58 = rd_u64(p7 + 0x20)? as usize;
    if !ptr_ok(local_58) { return Some(0); }   // ★readable VQ제거(직후 rd_u64(local_58+8/0x10)? fault-safe)
    let cmp_base = rd_u64(local_58 + 8)? as usize;
    let cmp_cnt = rd_u64(local_58 + 0x10)?;
    if cmp_base == 0 || cmp_cnt > 256 { return Some(0); }
    let mut count: u64 = 0;
    for i in 0..cand_cnt as usize {
        let obj = rd_u64(cand_base + i * 0x10)? as usize;
        if !ptr_ok(obj) { continue; }   // ★readable VQ제거(직후 rd_u64(obj+0x38/0x40)? fault-safe)
        let sub_base = rd_u64(obj + 0x38)? as usize;   // vt[0x78]=obj+0x30 → +8
        let sub_cnt = rd_u64(obj + 0x40)?;             // +0x10
        if sub_base == 0 || sub_cnt > 256 { continue; }
        for j in 0..sub_cnt as usize {
            let entry = sub_base + j * 0x18;
            if !readable(entry + 0x18, 8) { break; }
            let id_ptr = rd_u64(entry + 8)? as usize;
            let id_len = rd_u64(entry + 0x10)? as usize;
            // 비교집합과 매칭(첫 매치서 break)
            for k in 0..cmp_cnt as usize {
                let cobj = rd_u64(cmp_base + k * 0x10)? as usize;
                if !ptr_ok(cobj) { continue; }   // ★readable VQ제거(직후 rd_u64(cobj+8/0x10)? fault-safe)
                let cid_ptr = rd_u64(cobj + 8)? as usize;
                let cid_len = rd_u64(cobj + 0x10)? as usize;
                if cid_len == id_len && mem_eq(cid_ptr, id_ptr, id_len) {
                    let cpri = rd_u64(cobj + 0x188)?;
                    if local_50 < cpri {
                        let cthr = rd_u64(cobj + 0x180)?;
                        if cthr <= threshold { count += 1; }
                    }
                    break;
                }
            }
        }
    }
    Some(count)
}
// ★FUN_1420e88a0 선택 출력 재현 → (out0, out1=cand_i, out2=cmp_k). out0=count>0?1:0. count>0이면 gen_range(0,count)로 매치 1개 선택.
//   매치리스트=(cand_i, cmp_k) 발견순(my_e88a0_count과 동일루프). 선택=gen_range(0,count-1) 결과 인덱스. RngSim(read-only, 게임 RNG 무변조).
unsafe fn my_e88a0_pick(p4: usize, p7: usize, rng_state: usize) -> Option<(u64, i64, i64, u64)> {
    let cand_base = rd_u64(p4 + 0x3c8)? as usize;
    let cand_cnt = rd_u64(p4 + 0x3d0)?;
    let threshold = rd_u64(p4 + 0x710)?;
    if cand_base == 0 || cand_cnt == 0 || cand_cnt > 64 { return Some((0, 0, 0, 0)); }
    let mut local_50: u64 = 0;
    for i in 0..cand_cnt as usize {
        let obj = rd_u64(cand_base + i * 0x10)? as usize;
        if !ptr_ok(obj) { continue; }   // ★readable VQ제거(직후 rd_u64(obj+0x188)? fault-safe)
        let pri = rd_u64(obj + 0x188)?;
        if pri < 4 && pri > local_50 { local_50 = pri; }
    }
    if !ptr_ok(p7) { return Some((0, 0, 0, 0)); }   // ★readable VQ제거(직후 rd_u64(p7+0x20)? fault-safe)
    let local_58 = rd_u64(p7 + 0x20)? as usize;
    if !ptr_ok(local_58) { return Some((0, 0, 0, 0)); }   // ★readable VQ제거
    let cmp_base = rd_u64(local_58 + 8)? as usize;
    let cmp_cnt = rd_u64(local_58 + 0x10)?;
    if cmp_base == 0 || cmp_cnt > 256 { return Some((0, 0, 0, 0)); }
    let mut matched: Vec<(i64, i64)> = Vec::new();
    for i in 0..cand_cnt as usize {
        let obj = rd_u64(cand_base + i * 0x10)? as usize;
        if !ptr_ok(obj) { continue; }   // ★readable VQ제거(직후 rd_u64(obj+0x38/0x40)? fault-safe)
        let sub_base = rd_u64(obj + 0x38)? as usize;
        let sub_cnt = rd_u64(obj + 0x40)?;
        if sub_base == 0 || sub_cnt > 256 { continue; }
        for j in 0..sub_cnt as usize {
            let entry = sub_base + j * 0x18;
            if !readable(entry + 0x18, 8) { break; }
            let id_ptr = rd_u64(entry + 8)? as usize;
            let id_len = rd_u64(entry + 0x10)? as usize;
            for k in 0..cmp_cnt as usize {
                let cobj = rd_u64(cmp_base + k * 0x10)? as usize;
                if !ptr_ok(cobj) { continue; }   // ★readable VQ제거(직후 rd_u64(cobj+8/0x10)? fault-safe)
                let cid_ptr = rd_u64(cobj + 8)? as usize;
                let cid_len = rd_u64(cobj + 0x10)? as usize;
                if cid_len == id_len && mem_eq(cid_ptr, id_ptr, id_len) {
                    let cpri = rd_u64(cobj + 0x188)?;
                    if local_50 < cpri {
                        let cthr = rd_u64(cobj + 0x180)?;
                        if cthr <= threshold { matched.push((i as i64, k as i64)); }
                    }
                    break;
                }
            }
        }
    }
    let count = matched.len() as u64;
    if count == 0 { return Some((0, 0, 0, 0)); }
    let mut sim = RngSim::new(rng_state)?;
    let picked = sim.gen_range(0, count - 1)? as usize;
    if picked >= matched.len() { return Some((1, 0, 0, count)); }
    let (ci, ck) = matched[picked];
    Some((1, ci, ck, count))
}
// ★pre-gate FUN_2080760 순수 Rust 재현 (게임함수 호출X; leaf 게터 rvt[0x128]/rvt[0x20] + 메모리읽기만).
//   디컴 분기: candidate==0→false / q>1·p1>=4·D==0 = panic분기(정상엔 불발)→None(보수 passthrough) / dist>=r15→false /
//   else al = (r15 > acc), acc = dist2 + isqrt(dx²+dy²)/D + scale(=[[p6+8]+8]+0x12f8), arg8=0.
//   dx=|candx - tX[p1]|, dy=|candy - tY[p1]|; q==0:(tX=tableC,tY=tableD) q==1:(tX=tableD,tY=tableC). r15=[p9+q*0x20+tableA[p1]*8+0x360].
unsafe fn my_pregate(p2: usize, p5: usize, p6: usize, p9: usize, robj: usize, rvt: usize) -> Option<bool> {
    let base = exe_base();
    if base == 0 { return None; }
    // candidate = rvt[0x128](robj, [p5+0x6a0])
    if !readable(p5 + 0x6a8, 8) { return None; }
    let team_units = rd_u64(p5 + 0x6a0)? as usize;
    let g128 = vt_slot(rvt, 0x128); if !ptr_ok(g128) { return None; }
    let f128: G2 = core::mem::transmute(g128);
    let cand = f128(robj, team_units) as usize;
    if cand == 0 { return Some(false); }            // candidate null → al=0(FAIL)
    if !ptr_ok(cand) || !readable(cand + 0x658, 8) { return None; }
    let q = rd_u64(p2 + 0x48)?;                      // [p2+0x48] team
    let p1 = rd_u8(p2 + 0x60) as u64;               // [p2+0x60] lane
    if q > 1 || p1 >= 4 { return None; }            // panic 분기(out-of-bounds/unreachable) → 보수 passthrough
    // r15 threshold = [p9 + q*0x20 + tableA[p1]*8 + 0x360]
    let ta = rd_u64(base + RVA_TABLE_A + (p1 as usize) * 8)?;  // tableA[p1] ∈ {0,1,3,2}
    let r15_off = (q as usize) * 0x20 + (ta as usize) * 8 + 0x360;
    if !readable(p9 + r15_off, 8) { return None; }
    let r15 = rd_u64(p9 + r15_off)?;
    // dist = rvt[0x20](robj)
    let g20 = vt_slot(rvt, 0x20); if !ptr_ok(g20) { return None; }
    let f20: VtPtrFn = core::mem::transmute(g20);
    let dist = f20(robj) as u64;
    if dist >= r15 { return Some(false); }          // 거리>=thr → al=0(FAIL)
    // 좌표 비교: dx=|candx - tX[p1]|, dy=|candy - tY[p1]|
    let candx = rd_u64(cand + 0x648)?;
    let candy = rd_u64(cand + 0x650)?;
    let (rva_tx, rva_ty) = if q == 0 { (RVA_TABLE_C, RVA_TABLE_D) } else { (RVA_TABLE_D, RVA_TABLE_C) };
    let tx = rd_u64(base + rva_tx + (p1 as usize) * 8)?;
    let ty = rd_u64(base + rva_ty + (p1 as usize) * 8)?;
    let dx = if candx >= tx { candx - tx } else { tx - candx };
    let dy = if candy >= ty { candy - ty } else { ty - candy };
    let sq = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
    let s = isqrt_u64(sq);
    let d = rd_u64(cand + 0x628)?;                   // D
    if d == 0 { return None; }                       // div-by-zero panic 분기 → 보수
    let quo = s / d;
    let dist2 = f20(robj) as u64;                    // rvt[0x20](robj) 다시
    let p7 = rd_u64(p6 + 8)? as usize;               // [p6+8]
    if !ptr_ok(p7) { return None; }   // ★readable VQ제거(직후 rd_u64(p7+8)?)
    let sub = rd_u64(p7 + 8)? as usize;              // [p7+8]
    if !ptr_ok(sub) { return None; }   // ★readable VQ제거(직후 rd_u64(sub+0x12f8)?)
    let scale = rd_u64(sub + 0x12f8)?;
    let acc = dist2.wrapping_add(quo).wrapping_add(scale);  // arg8=0
    Some(r15 > acc)                                  // al = seta(r15 > acc)
}
// ★engage 게이트 재현: roll에 깨끗이 도달하는지 판정(zero-edge bit-level). 둘 중 하나라도 fire/불확실 → false(emit서 passthrough).
//   pre-gate(0x2080760): my_pregate 순수Rust 재현(게임함수 호출X). false면 retreat -1(roll전, 0 RNG). distance gate: 재현.
//   None=가드 계산실패(보수적 passthrough). Some(true)=roll도달. Some(false)=게이트 fire(passthrough해야 게임이 정확 처리).
unsafe fn engage_reaches_roll(p2: usize, p5: usize, p6: usize, p9: usize) -> Option<bool> {
    if !ptr_ok(p2) || !ptr_ok(p5) || !ptr_ok(p6) || !ptr_ok(p9) { return None; }   // ★readable VQ제거
    let rh = rd_u64(p6).unwrap_or(0) as usize;
    if !ptr_ok(rh) { return None; }   // ★readable VQ제거(직후 rd_u64(rh)/rd_u64(rh+8))
    let robj = rd_u64(rh).unwrap_or(0) as usize;
    let rvt = rd_u64(rh + 8).unwrap_or(0) as usize;
    if !ptr_ok(robj) || !ptr_ok(rvt) { return None; }
    if !readable(p2 + 0x61, 1) || !readable(p2 + 0x48, 8) { return None; }
    // ── pre-gate: my_pregate(순수Rust 재현) → al ──
    let pg = my_pregate(p2, p5, p6, p9, robj, rvt)?;
    if !pg { return Some(false); }   // pre-gate fail → retreat -1(0 RNG)
    // ── distance gate(재현) ──
    let p7 = rd_u64(p6 + 8).unwrap_or(0) as usize;
    if !ptr_ok(p7) { return None; }   // ★readable VQ제거(직후 rd_u64(p7+8)?)
    let baseline = rd_u64(p2 + 0x58)?;        // [p2+0x58]
    let b = rd_u64(p5 + 0x238)?;              // [p5+0x238]
    let bm = if b > 100 { 100u64 } else { b };
    let edi: u64 = 4u64.wrapping_sub(((41u64.wrapping_mul(bm)) & 0xffff) >> 11);
    let sub = rd_u64(p7 + 8).unwrap_or(0) as usize;
    if !ptr_ok(sub) { return None; }   // ★readable VQ제거(직후 rd_u64(sub+0x12f8)?)
    let scale = rd_u64(sub + 0x12f8)?;
    let rdi = edi.wrapping_mul(scale);
    let s = vt_slot(rvt, 0x20); if !ptr_ok(s) { return None; }
    let g: VtPtrFn = core::mem::transmute(s);
    let dist = g(robj) as u64;
    let rcx = if dist >= baseline { dist - baseline } else { 0 };
    if rcx < rdi { return Some(false); }   // distance gate fail → retreat -1(e88a0만)
    Some(true)
}
// ════ ★인원수(머릿수)·포탑 회피 — 게임 원본 AI에 없는 신규 항 (2026-06-22) ════
//   engage 교전(5) 결정시: ① 근처 적챔피언>아군+margin → 후퇴 ② self가 적 포탑 사거리내 → 후퇴.
//   ★self 위치 = engage_self_pos(p5+0x6a0 핸들 → dd7_slot128 챔피언resolve). 엔진 self_e는 RNG홀더라 +0x648=위치 아님(캡처확인).
//   ★RNG 무관: roll/writeback 다 소비 후 출력만 보정 → draw수 불변. 기본(margin=0·threat=0)=동작보존.
//   카운트=dd7700 검증 로스터(rh=*(p6)+0x1e0+team*0x28+k*8, 5슬롯). 포탑=l80 oidx 0x3c..0x45 type-13(obj+0x68==0xd), 적팀=obj+0x8==1-q(캡처확인 5/팀).
static NUMBERS_MARGIN: AtomicI64 = AtomicI64::new(0);     // cfg numbers_margin: 0=off, ≥1=적−아군≥margin이면 후퇴(단순 binary)
static NUMBERS_THREAT: AtomicI64 = AtomicI64::new(0);     // ★cfg numbers_threat 0~100(0=off): 일반교전 전력(force)승산 임계. numbers_threat≥승산이면 후퇴(강하면 적어도 싸움)
static NUMBERS_RANGE: AtomicU64 = AtomicU64::new(150000); // cfg numbers_range: 챔피언 근접반경(머릿수·전력 카운트 공용)
static NUMBERS_OVR_N: AtomicU64 = AtomicU64::new(0);
static TOWER_THREAT: AtomicI64 = AtomicI64::new(0);       // cfg tower_threat 0~100(0=off). 유효사거리 = tower_range×threat/100
static TOWER_RANGE: AtomicU64 = AtomicU64::new(140000);   // cfg tower_range: threat=100일때 포탑 위험반경
static STAT_INFLUENCE: AtomicI64 = AtomicI64::new(0);     // ★cfg stat_influence 0~100(0=off=비트동일): 성향스탯 보정강도(공격성/에고=결정론 임계시프트, 판단력=결정론 해시노이즈). 중립=공격성50·에고50·판단력100=현행
static TOWER_OVR_N: AtomicU64 = AtomicU64::new(0);
static ENG_OUT5_N: AtomicU64 = AtomicU64::new(0);   // 진단: base_out==5(교전) 횟수
static NUM_LASTCNT: AtomicU64 = AtomicU64::new(0);  // 진단: 마지막 카운트 (ally<<32)|enemy
static NUM_MAXENEMY: AtomicU64 = AtomicU64::new(0); // 진단: 본 적군수 최대
// self 챔피언 위치 resolve (self_e는 RNG홀더라 위치 아님 → p5+0x6a0 핸들을 dd7_slot128로).
unsafe fn engage_self_pos(p6: usize, p5: usize) -> Option<(u64, u64)> {
    let rh = rd_u64(p6)? as usize;
    let sim = rd_u64(rh)? as usize;
    let selfe = dd7_slot128(sim, rd_u64(p5 + 0x6a0)?);
    if !ptr_ok(selfe) || !readable(selfe + 0x650, 8) { return None; }
    Some((rd_u64(selfe + 0x648)?, rd_u64(selfe + 0x650)?))
}
unsafe fn count_nearby_champs(rh: usize, team: i64, sx: u64, sy: u64) -> Option<(u32, u32)> {
    if !ptr_ok(rh) || team < 0 || team > 1 { return None; }
    let q = team as usize;
    let r = NUMBERS_RANGE.load(Ordering::Relaxed); let r2 = r.wrapping_mul(r);
    let cnt = |t: usize| -> u32 {
        let mut n = 0u32;
        for k in 0..5usize {
            let c = rd_u64(rh + 0x1e0 + t*0x28 + k*8).unwrap_or(0) as usize;
            if c == 0 || rd_u64(c + 0x658).unwrap_or(0) == 0 { continue; }   // ★readable VQ제거: rd_u64 None/0이 fault·사망 동시흡수(중복read도 제거, 비트동일)
            let cx = rd_u64(c + 0x648).unwrap_or(0); let cy = rd_u64(c + 0x650).unwrap_or(0);
            let dx = if cx >= sx { cx - sx } else { sx - cx };
            let dy = if cy >= sy { cy - sy } else { sy - cy };
            if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) < r2 { n += 1; }
        }
        n
    };
    Some((cnt(q), cnt(1 - q)))        // (아군, 적군)
}
// ★전력(force) 승산 — 근처 양팀 챔프의 (ΣHP)×(Σ공격)로 Lanchester 전투력 비교(2026-06-23 정식형, DPS×HP 전투추정 = 유저요구 (b)).
//   유효HP=curhp(+0x658), DPS=공격스탯(+0x610, my_combat_dmg DIFF=0 검증오프셋). 머릿수는 Σ에 자연가중(2배면 force 4배=Lanchester square = 한타서 수적우세 초선형). 세기(HP·공격)도 반영 → "적어도 강하면 싸움".
//   승산 = force_ally×100/force_enemy (100=호각, >100=우세, 적 없으면 9999=무패). 반환 (승산, 아군수, 적군수). u128로 오버플로 차단.
unsafe fn combat_balance(rh: usize, team: i64, sx: u64, sy: u64) -> Option<(i64, u32, u32)> {
    if !ptr_ok(rh) || team < 0 || team > 1 { return None; }
    let q = team as usize;
    let r = NUMBERS_RANGE.load(Ordering::Relaxed); let r2 = r.wrapping_mul(r);
    let team_force = |t: usize| -> (u128, u128, u32) {
        let (mut hp, mut atk, mut n) = (0u128, 0u128, 0u32);
        for k in 0..5usize {
            let c = rd_u64(rh + 0x1e0 + t*0x28 + k*8).unwrap_or(0) as usize;
            if c == 0 { continue; }   // ★readable VQ제거(아래 chp==0이 fault·사망 동시흡수=비트동일)
            let chp = rd_u64(c + 0x658).unwrap_or(0);
            if chp == 0 { continue; }   // 사망(curhp=0)/fault skip
            let cx = rd_u64(c + 0x648).unwrap_or(0); let cy = rd_u64(c + 0x650).unwrap_or(0);
            let dx = if cx >= sx { cx - sx } else { sx - cx };
            let dy = if cy >= sy { cy - sy } else { sy - cy };
            if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) >= r2 { continue; }
            hp += chp as u128;
            atk += rd_i64(c + 0x610).unwrap_or(0).max(0) as u128;   // 공격스탯(my_combat_dmg 검증 = 게임 데미지식 입력)
            n += 1;
        }
        (hp, atk, n)
    };
    let (ahp, aatk, an) = team_force(q);
    let (ehp, eatk, en) = team_force(1 - q);
    let f_ally = ahp.wrapping_mul(aatk);
    let f_enemy = ehp.wrapping_mul(eatk);
    let w: i64 = if f_enemy == 0 { 9999 } else if f_ally == 0 { 0 }
                 else { (f_ally.wrapping_mul(100) / f_enemy).min(9999) as i64 };
    Some((w, an, en))
}
static D4_NUM_OVR_N: AtomicU64 = AtomicU64::new(0);   // 진단: disc4 인원수 후퇴 override 발동
// ★다이브게이트식 "교전중 챔피언 수": engage-list(cont+0xf0+team*0x20, count@+0x108) 중 로스터(챔피언)만 카운트.
//   ttd_capture(L2731)와 동일 구조. 로스터-근접 폴백 아님 = 실제 교전중 적 챔프수(한타 규모 반영). team-wide.
#[allow(dead_code)]
unsafe fn champ_combat_counts(l80: usize, team: i64) -> Option<(u32, u32)> {
    if !ptr_ok(l80) || team < 0 || team > 1 { return None; }
    let q = team as usize;
    let count_champ = |t: usize| -> u32 {
        let start = rd_u64(l80 + 0xf0 + t*0x20).unwrap_or(0) as usize;
        let cnt = rd_u64(l80 + 0x108 + t*0x20).unwrap_or(0) as usize;
        if !ptr_ok(start) || cnt == 0 || cnt > 64 { return 0; }
        let mut n = 0u32;
        for i in 0..cnt {
            let en = rd_u64(start + i*8).unwrap_or(0) as usize;
            if en == 0 { continue; }
            let mut is_champ = false;   // 로스터 멤버십 = 챔피언(미니언 제외)
            for k in 0..5usize { if rd_u64(l80 + 0x1e0 + t*0x28 + k*8).unwrap_or(0) as usize == en { is_champ = true; break; } }
            if is_champ { n += 1; }
        }
        n
    };
    Some((count_champ(q), count_champ(1 - q)))   // (아군 교전챔프, 적 교전챔프)
}
// ★disc4 교전-커밋 보정: code8(전진/추격)일 때, self(target) 근처 적챔피언>아군+margin이면 7(홀드/귀환)로. disc4=RNG-free라 출력만 바꿔도 안전.
unsafe fn disc4_engage_or_hold(code: i64, p6: usize, team: i64, target: usize) -> i64 {
    if code != 8 { return code; }
    let margin = NUMBERS_MARGIN.load(Ordering::Relaxed);
    if margin <= 0 { return code; }
    let rh = match rd_u64(p6) { Some(v) if ptr_ok(v as usize) => v as usize, _ => return code };
    let sx = rd_u64(target + 0x648).unwrap_or(0); let sy = rd_u64(target + 0x650).unwrap_or(0);   // target=self(disc4)
    match count_nearby_champs(rh, team, sx, sy) {
        Some((ally, enemy)) => {
            NUM_LASTCNT.store(((ally as u64) << 32) | (enemy as u64), Ordering::Relaxed);
            if (enemy as u64) > NUM_MAXENEMY.load(Ordering::Relaxed) { NUM_MAXENEMY.store(enemy as u64, Ordering::Relaxed); }
            if (enemy as i64) - (ally as i64) >= margin { D4_NUM_OVR_N.fetch_add(1, Ordering::Relaxed); 7 } else { code }
        }
        None => code,
    }
}
// self가 적 포탑 유효사거리 안인가: l80 type-13 구조물 중 적팀(obj+0x8==1-q)만 거리²<eff².
unsafe fn is_under_enemy_tower(p6: usize, p2: usize, sx: u64, sy: u64) -> bool {
    let threat = TOWER_THREAT.load(Ordering::Relaxed);
    if threat <= 0 { return false; }
    let rh = match rd_u64(p6) { Some(v) if ptr_ok(v as usize) => v as usize, _ => return false };
    let q = match rd_u64(p2 + 0x48) { Some(v) if v <= 1 => v as i64, _ => return false };
    let eff = TOWER_RANGE.load(Ordering::Relaxed).wrapping_mul(threat.min(100) as u64) / 100;
    let eff2 = eff.wrapping_mul(eff);
    for oidx in 0x3c..0x46usize {                          // 캡처: type-13 구조물 oidx 0x3c..0x45
        let obj = rd_u64(rh + oidx*8).unwrap_or(0) as usize;
        if obj < 0x10000 { continue; }   // ★readable VQ제거(아래 rd_i32 -99이 fault흡수=비트동일)
        if rd_i32(obj + 0x68).unwrap_or(-99) != 0xd { continue; }       // 구조물(type13)
        if rd_i64(obj + 0x8).unwrap_or(-1) != 1 - q { continue; }       // 적팀 포탑만(+8=team)
        let ox = rd_u64(obj + 0x648).unwrap_or(0); let oy = rd_u64(obj + 0x650).unwrap_or(0);
        let dx = if ox >= sx { ox - sx } else { sx - ox };
        let dy = if oy >= sy { oy - sy } else { sy - oy };
        if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) < eff2 { return true; }
    }
    false
}
// engage 출력 보정(인원수+포탑): 교전(5)인데 불리/적포탑이면 후퇴(-1). 후퇴→교전 절대 안 바꿈(보수). self_pos 1회 계산 공유.
unsafe fn engage_situational_override(base_out: i64, p6: usize, p2: usize, p5: usize) -> i64 {
    if base_out != 5 { return base_out; }
    ENG_OUT5_N.fetch_add(1, Ordering::Relaxed);   // 진단: 교전(5) 결정 횟수
    let margin = NUMBERS_MARGIN.load(Ordering::Relaxed);
    let threat = TOWER_THREAT.load(Ordering::Relaxed);
    if margin <= 0 && threat <= 0 { return base_out; }    // 둘 다 off → 동작보존
    let (sx, sy) = match engage_self_pos(p6, p5) { Some(p) => p, None => return base_out };
    if margin > 0 {
        let rh = rd_u64(p6).unwrap_or(0) as usize;
        let team = rd_u64(p2 + 0x48).map(|v| v as i64).unwrap_or(-1);
        if let Some((ally, enemy)) = count_nearby_champs(rh, team, sx, sy) {
            NUM_LASTCNT.store(((ally as u64) << 32) | (enemy as u64), Ordering::Relaxed);   // 진단
            if (enemy as u64) > NUM_MAXENEMY.load(Ordering::Relaxed) { NUM_MAXENEMY.store(enemy as u64, Ordering::Relaxed); }
            if (enemy as i64) - (ally as i64) >= margin { NUMBERS_OVR_N.fetch_add(1, Ordering::Relaxed); return -1; }
        }
    }
    if threat > 0 && is_under_enemy_tower(p6, p2, sx, sy) { TOWER_OVR_N.fetch_add(1, Ordering::Relaxed); return -1; }
    base_out
}
// ════ ★포탑 데이터 런타임 캡처 (cfg towercap=1, 일회성 진단) ════
//   engage 컨텍스트 l80(=rh=*(p6))서 type-13(obj+0x68==0xd) 구조물 스캔 → oidx/위치/팀필드후보(+8,+6a8)/self거리 덤프.
//   목적: 적 포탑 식별(아군제외) + 사거리 추정. 확정 후 포탑 회피항 구현. LOG_ON 무관 직접write(명시적 진단).
static TOWERCAP: AtomicBool = AtomicBool::new(false);
static TOWERCAP_N: AtomicU64 = AtomicU64::new(0);
unsafe fn tower_capture(p6: usize, p2: usize, p5: usize, self_e: usize) {
    if !TOWERCAP.load(Ordering::Relaxed) { return; }
    let n = TOWERCAP_N.fetch_add(1, Ordering::Relaxed);
    if n < 600 {   // ★engage 경로(전 유닛·매프레임)서 로스터-근접 카운트 샘플(max 추적, 경기편차 무관, 비용상한)
        if let (Some(rh), Some(tm), Some((sx, sy))) = (rd_u64(p6), rd_u64(p2 + 0x48), engage_self_pos(p6, p5)) {
            if let Some((ally, enemy)) = count_nearby_champs(rh as usize, tm as i64, sx, sy) {
                NUM_LASTCNT.store(((ally as u64) << 32) | (enemy as u64), Ordering::Relaxed);
                if (enemy as u64) > NUM_MAXENEMY.load(Ordering::Relaxed) { NUM_MAXENEMY.store(enemy as u64, Ordering::Relaxed); }
            }
        }
    }
    if n >= 120 || n % 12 != 0 { return; }   // 덤프: 매 12번째, 최대 ~10샘플(런 전반 분포)
    let cp = engage_self_pos(p6, p5);
    let _ = (p2, self_e);
    // ★상태 진단: cfg 로드값(eng_role3=84면 AI개선3 로드됨) + override 카운터 + engage 통계(roll도달 vs passthrough)
    let s = format!("[{}] eng_role3_loaded={} NUM_MARGIN={} TOW_THREAT={} ENG_REPL={} | OVR num={} tow={} | ENG_N={} PT(gate={} count={} other={}) champ={:?}\n",
        n, tune("eng_role3", 70), NUMBERS_MARGIN.load(Ordering::Relaxed), TOWER_THREAT.load(Ordering::Relaxed),
        ENGAGE_REPL.load(Ordering::Relaxed) as u8, NUMBERS_OVR_N.load(Ordering::Relaxed), TOWER_OVR_N.load(Ordering::Relaxed),
        ENGAGE_REPL_N.load(Ordering::Relaxed), PT_GATE.load(Ordering::Relaxed), PT_COUNT.load(Ordering::Relaxed), PT_OTHER.load(Ordering::Relaxed), cp)
        + &format!("     OUT5={} near(ally={} enemy={}) maxNearE={} D4_OVR={} D4_C8={} TOW_HIT={} TOW_MAX={} LRET={}(T{}F{}N{} W{})\n", ENG_OUT5_N.load(Ordering::Relaxed), NUM_LASTCNT.load(Ordering::Relaxed)>>32, NUM_LASTCNT.load(Ordering::Relaxed)&0xffffffff, NUM_MAXENEMY.load(Ordering::Relaxed), D4_NUM_OVR_N.load(Ordering::Relaxed), D4_TTD_C8.load(Ordering::Relaxed), TOWER_HIT_N.load(Ordering::Relaxed), TOWER_HIT_MAX.load(Ordering::Relaxed), LANER_RET_N.load(Ordering::Relaxed), LANER_RET_TOW.load(Ordering::Relaxed), LANER_RET_FRC.load(Ordering::Relaxed), LANER_RET_NUM.load(Ordering::Relaxed), LANER_RET_W.load(Ordering::Relaxed));
    // ★TOWSCAN: 타워 RE(aff768e) 확정 구조물 슬롯 검증 — l80+{0x170,0x180,0x190,0x1a0,0x1b0,0x1c0,0x1d0}+team*8(고정) + l80+0x130+team*0x20(Vec base/len@+0x148). 적팀(1-q) 각 슬롯 type(+0x68)/flag(+0x70)/pos(+0x648,+0x650)/hp(+0x658)/eff(+0x4b0) 덤프. → 핸들vs포인터·type enum·좌표스케일·effect슬롯·고정vs이동 확정.
    let mut rost = String::from("     TOW");
    if let (Some(rhv), Some(qv)) = (rd_u64(p6), rd_u64(p2 + 0x48)) {
        let rh = rhv as usize; let q = qv as usize;
        if ptr_ok(rh) && q <= 1 {
            let et = 1 - q;
            for &off in &[0x170usize, 0x180, 0x190, 0x1a0, 0x1b0, 0x1c0, 0x1d0] {
                let e = rd_u64(rh + off + et*8).unwrap_or(0) as usize;
                if e == 0 { rost.push_str(&format!(" {:x}=0", off)); }
                else if !ptr_ok(e) { rost.push_str(&format!(" {:x}=H{:x}", off, e)); }   // 핸들(작은값)?
                else { rost.push_str(&format!(" {:x}=t{}f{}({},{}h{}e{:x})", off, rd_i32(e+0x68).unwrap_or(-9), rd_i32(e+0x70).unwrap_or(-9), rd_u64(e+0x648).unwrap_or(0), rd_u64(e+0x650).unwrap_or(0), rd_u64(e+0x658).unwrap_or(0), rd_u64(e+0x4b0).unwrap_or(0)&0xffffff)); }
            }
            let vbase = rd_u64(rh + 0x130 + et*0x20).unwrap_or(0) as usize;
            let vlen = rd_u64(rh + 0x148 + et*0x20).unwrap_or(0);
            rost.push_str(&format!(" |Vec(l{}):", vlen));
            if ptr_ok(vbase) && vlen > 0 && vlen <= 16 {
                for i in 0..vlen as usize {
                    let e = rd_u64(vbase + i*8).unwrap_or(0) as usize;
                    if ptr_ok(e) { rost.push_str(&format!(" t{}({},{}h{})", rd_i32(e+0x68).unwrap_or(-9), rd_u64(e+0x648).unwrap_or(0), rd_u64(e+0x650).unwrap_or(0), rd_u64(e+0x658).unwrap_or(0))); }
                }
            }
        }
    }
    rost.push('\n');
    let s = s + &rost;
    if let Some(p) = pth("towercap.txt") {
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = f.write_all(s.as_bytes()); }
    }
}
// ★facet#5 engage 출력+RNG footprint 예측 (entry대체용). footprint = draw1(e9a30 count_a) + draw2(e88a0 count_b) + roll(gen_range0,100).
//   인자 재구성(rbp산술 확정): e9a30/e88a0 p3=p5(arg5), arg_cont=p7=[p6+8](=r15, arg6), RNG=self_e(param4).
//   ★e9a30 draw1: 전엔 0가정했으나 count_a>0 케이스(u64 gen_range range=count_a) 존재 → footprint에 포함(2026-06-20 DIFF수정).
//   thr=(out0==1)? ladder(r15_array[out2] priority{4:100/3:70/2:50/_:30}) : 0. out=roll>=thr?-1:5.
//   ★게이트(engage_reaches_roll)로 zero-edge: roll 미도달이면 None(passthrough).
//   반환: (out, rng_words, count_a, count_b) — words=refills*64+exit_idx-entry_idx (engfoot 실제와 대조 + 진단).
unsafe fn my_engage_predict(p2: usize, p5: usize, p6: usize, p9: usize, self_e: usize) -> Option<(i64, i64, i64, i64)> {
    if !ptr_ok(p5) || !ptr_ok(p6) || !ptr_ok(self_e) { return None; }   // ★readable VQ제거(p6+8/self_e+0x138은 rd_u64?/미사용)
    // ★게이트 가드: pre-gate(실제호출) + distance gate(재현) → roll 미도달/불확실이면 None(검증서 skip, emit서 passthrough).
    if engage_reaches_roll(p2, p5, p6, p9) != Some(true) { return None; }
    let p7 = rd_u64(p6 + 8)? as usize;   // r15
    if !ptr_ok(p7) { return None; }
    let count_a = my_e9a30_count(p5, p7)?;   // draw1(e9a30) gather count
    // ★E9_JT off: jumptable 미적용=과대추정 → count_a>0이면 passthrough(my_count>=real, count_a==0=무draw 확정=안전).
    //   E9_JT on: count_a 정확(jumptable carry 적용) → count_a>0도 draw 모델링해 대체(100%).
    if !E9_JT.load(Ordering::Relaxed) && count_a > 0 { return None; }
    let (out0, _o1, out2, count_b) = my_e88a0_pick(p5, p7, self_e)?;   // ★count_b=pick의 matched.len() 재사용(중복 e88a0_count 3중루프 제거)
    // thr ladder: cand = [[p7+0x20]+8] + out2*0x10 → *(obj+0x188) → ladder
    let thr: i64 = if out0 == 1 {
        let a = rd_u64(p7 + 0x20)? as usize;
        if !ptr_ok(a) { return None; }   // ★readable VQ제거(직후 rd_u64(a+8)? fault-safe)
        let arr = rd_u64(a + 8)? as usize;
        let o2 = out2 as usize;
        if ptr_ok(arr) && o2 < 64 {   // ★readable VQ제거(본문 rd_u64(arr+o2*0x10)?, else→None passthrough 동치)
            let obj = rd_u64(arr + o2 * 0x10)? as usize;
            if ptr_ok(obj) {   // ★readable VQ제거(본문 rd_u64(obj+0x188)?, else→None passthrough 동치)
                (match rd_u64(obj + 0x188)? { 4 => tune("eng_role4", 100), 3 => tune("eng_role3", 70), 2 => tune("eng_role2", 50), _ => tune("eng_role_def", 30) }) * TUNE_ENGAGE_MULT.load(Ordering::Relaxed) / 100
            } else { return None; }
        } else { return None; }
    } else { 0 };
    // RNG footprint: e9a30 draw1(count_a) + e88a0 draw2(count_b) + roll(0,100). RngSim read-only, 순서대로.
    //   (E9_JT off면 count_a>0은 위에서 passthrough됐으므로 여기 count_a는 0 또는 정확)
    let mut sim = RngSim::new(self_e)?;
    let i0 = sim.idx;
    if count_a > 0 { sim.gen_range(0, count_a - 1)?; }   // ★draw1(jumptable 정확 count)
    if count_b > 0 { sim.gen_range(0, count_b - 1)?; }
    let roll = sim.gen_range(0, 100)? as i64;   // gen_range(0,100): range=101(0..100 inclusive)
    let words = (sim.refills.wrapping_mul(64).wrapping_add(sim.idx).wrapping_sub(i0)) as i64;
    let out = if roll >= thr { -1 } else { 5 };
    Some((out, words, count_a as i64, count_b as i64))
}
// ★facet#5 engage entry 완전대체 EMIT: my_engage_predict와 동일 계산 + 게임 RNG state(self_e) writeback(e88a0 draw + roll). 반환=out(-1/5).
//   writeback: RngSim로 2 draw 시뮬 후 최종 buf(refill시)+counter+idx를 self_e에 되쓰기 → 게임이 e88a0+roll 소비한 것과 동일 state.
//   ⚠게이트 early-exit 미반영(empirically 0/2500diverse). 가드: 계산 실패시 None→passthrough.
unsafe fn my_engage_emit(p2: usize, p5: usize, p6: usize, p9: usize, self_e: usize) -> Option<i64> {
    let _pg = perf_guard(5);
    if !ptr_ok(p5) || !ptr_ok(p6) || !ptr_ok(self_e) || !writable(self_e, 0x108) { PT_OTHER.fetch_add(1, Ordering::Relaxed); return None; }   // ★readable VQ제거(writable=RNG-sync 유지)
    tower_capture(p6, p2, p5, self_e);   // ★포탑 데이터 캡처(cfg towercap=1일때만, 일회성)
    // ★게이트 가드: roll 미도달/불확실 → None(passthrough). 게이트 fire시 RNG footprint가 다르므로 원본 처리에 위임.
    if engage_reaches_roll(p2, p5, p6, p9) != Some(true) { PT_GATE.fetch_add(1, Ordering::Relaxed); return None; }
    let p7 = rd_u64(p6 + 8)? as usize;
    if !ptr_ok(p7) { PT_OTHER.fetch_add(1, Ordering::Relaxed); return None; }
    let count_a = match my_e9a30_count(p5, p7) { Some(c) => c, None => { PT_COUNT.fetch_add(1, Ordering::Relaxed); return None; } };   // draw1(e9a30) gather count
    // ★E9_JT off: 과대추정→count_a>0 passthrough(desync방지). on: 정확→draw 모델링해 writeback(100%).
    if !E9_JT.load(Ordering::Relaxed) && count_a > 0 { return None; }
    let (out0, _o1, out2, count_b) = match my_e88a0_pick(p5, p7, self_e) { Some(x) => x, None => { PT_OTHER.fetch_add(1, Ordering::Relaxed); return None; } };   // ★count_b 재사용(중복 루프 제거)
    let thr: i64 = if out0 == 1 {
        let a = rd_u64(p7 + 0x20)? as usize;
        if !ptr_ok(a) { return None; }   // ★readable VQ제거(직후 rd_u64(a+8)? fault-safe)
        let arr = rd_u64(a + 8)? as usize;
        let o2 = out2 as usize;
        if ptr_ok(arr) && o2 < 64 {   // ★readable VQ제거(본문 rd_u64(arr+o2*0x10)?, else→None passthrough 동치)
            let obj = rd_u64(arr + o2 * 0x10)? as usize;
            if ptr_ok(obj) {   // ★readable VQ제거(본문 rd_u64(obj+0x188)?, else→None passthrough 동치)
                (match rd_u64(obj + 0x188)? { 4 => tune("eng_role4", 100), 3 => tune("eng_role3", 70), 2 => tune("eng_role2", 50), _ => tune("eng_role_def", 30) }) * TUNE_ENGAGE_MULT.load(Ordering::Relaxed) / 100
            } else { return None; }
        } else { return None; }
    } else { 0 };
    // RNG writeback: e9a30 draw1(count_a) + e88a0 draw2(count_b) + roll(0,100) → self_e state 전진(순서대로)
    let mut sim = RngSim::new(self_e)?;
    if count_a > 0 { sim.gen_range(0, count_a - 1)?; }   // ★draw1(jumptable 정확 count)
    if count_b > 0 { sim.gen_range(0, count_b - 1)?; }
    let roll = sim.gen_range(0, 100)? as i64;
    let input = self_e + 0x110;
    if sim.refills > 0 {
        let before_counter = rd_u64(input + 0x20)?;
        for i in 0..64 { std::ptr::write_unaligned((self_e + i * 4) as *mut u32, sim.buf[i]); }
        std::ptr::write_unaligned((input + 0x20) as *mut u64, before_counter.wrapping_add(4u64.wrapping_mul(sim.refills)));
    }
    std::ptr::write_unaligned((self_e + 0x100) as *mut u64, sim.idx);
    let base_out = if roll >= thr { -1 } else { 5 };   // 원본 결정(roll vs 임계)
    Some(engage_situational_override(base_out, p6, p2, p5))   // ★인원수+포탑 보정(둘다 0이면 그대로=동작보존)
}
// ★facet#5 engage draw1 (FUN_1420e9a30) gather count 재현 (tentative: 조건1-3 + LOOP1 pre-gate, jumptable 필터는 1차에선 미적용).
//   조건: facetcnt(=[p3+0x3d0])<=2 AND vt0x60(cand+0x180)<=thr([p3+0x710]) AND vt0x68(cand+0x188)==0.
//   LOOP1 pre-gate: [p3+0x3c8] vec(len [p3+0x3d0]) 中 priority(+0x188)<4 있으면 → return 0(무RNG).
//   ⚠jumptable(K=[p3+0x440].vt0x20, facet코드 +0x190 필터)는 미반영 → 과대추정 가능. 1차 측정으로 확정 후 정밀화.
static E9_JT: AtomicBool = AtomicBool::new(false);  // jumptable 필터 적용(cfg e9jt). on=정확count, off=과대추정+가드
// ★후보 게터 에뮬레이트(vtable별 오프셋 상이 대응, 호출X 순수읽기). vt[slot]=`mov rax/eax,[rcx+disp]; ret` 파싱→*(obj+disp).
//   확정: thr(vt0x60)/pri(vt0x68)=전타입 0x180/0x188 / fc(vt0x98)=타입별 0x190(0x355a5f0)·0x1a0(718/840)·0x1b8(968). 알수없는패턴→None(보수).
unsafe fn cand_get(obj: usize, vt: usize, slot: usize) -> Option<u64> {
    if !ptr_ok(vt) || !readable(vt + slot, 8) { return None; }
    let fp = rd_u64(vt + slot)? as usize;
    if !ptr_ok(fp) || !readable(fp, 7) { return None; }
    let b0 = rd_u8(fp); let b1 = rd_u8(fp + 1); let b2 = rd_u8(fp + 2);
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x81 { let d = rd_u32(fp + 3) as usize; return rd_u64(obj + d); }       // mov rax,[rcx+disp32]
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x41 { let d = rd_u8(fp + 3) as usize; return rd_u64(obj + d); }         // mov rax,[rcx+disp8]
    if b0 == 0x8b && b1 == 0x81 { let d = rd_u32(fp + 2) as usize; return Some(rd_u32(obj + d) as u64); }          // mov eax,[rcx+disp32]
    if b0 == 0x8b && b1 == 0x41 { let d = rd_u8(fp + 2) as usize; return Some(rd_u32(obj + d) as u64); }           // mov eax,[rcx+disp8]
    None
}
type E9JtFn = unsafe extern "C" fn(usize) -> u64;       // rcx=VBUF → raw_JT(eax)
type E9VFn = unsafe extern "C" fn(*mut u8, usize);      // rcx=out(sret), rdx=VBUF
// ★e9a30 jumptable JT/v 게터 호출 (순수확인됨: JT_getter=*(VBUF+0x1b8)단순읽기·v_getter=sret복사 out[0x10]=*(VBUF+0x128), 게임상태 write/RNG 0=더블콜안전).
//   체인: obj440=*(p3+0x440)=vtable객체본체(fn ptr=*(obj440+0x20)JT·*(obj440+0x30)v), VBUF=((*(obj440+0x10)-1)&~0xf)+*(p3+0x438)+0x10. reg-arg호출(스택0=shim불요). 실패→None(보수).
unsafe fn my_e9a30_jt_v(p3: usize) -> Option<(u32, u64)> {
    if !readable(p3 + 0x448, 8) { return None; }
    let obj440 = rd_u64(p3 + 0x440)? as usize;
    let buf438 = rd_u64(p3 + 0x438)? as usize;
    if !ptr_ok(obj440) || !ptr_ok(buf438) || !readable(obj440 + 0x38, 8) { return None; }
    let olen = rd_u64(obj440 + 0x10)?;
    let vbuf = (((olen as usize).wrapping_sub(1)) & !0xfusize).wrapping_add(buf438).wrapping_add(0x10);
    if !ptr_ok(vbuf) || !readable(vbuf, 0x200) { return None; }   // 게터 read범위 보수 커버(JT=+0x1b8, v=+0x118..0x158)
    let jt_fp = rd_u64(obj440 + 0x20)? as usize;
    let v_fp = rd_u64(obj440 + 0x30)? as usize;
    if !ptr_ok(jt_fp) || !ptr_ok(v_fp) || !readable(jt_fp, 4) || !readable(v_fp, 4) { return None; }
    let jt_fn: E9JtFn = core::mem::transmute(jt_fp);
    let raw_jt = (jt_fn(vbuf) & 0xffffffff) as u32;
    let v_fn: E9VFn = core::mem::transmute(v_fp);
    let mut tmp = [0u8; 0x60];
    v_fn(tmp.as_mut_ptr(), vbuf);
    let v = u64::from_le_bytes(tmp[0x10..0x18].try_into().ok()?);
    Some((raw_jt, v))
}
// ★jumptable carry 규칙(ghidra-re 디코드, raw_jt별). cif=fc∈{2,3,5}, cnt02=cnt∈{0,2}.
#[inline] fn e9a30_carry(raw_jt: u32, v: u64, fc: u32, cnt: u64) -> bool {
    let cif = fc == 2 || fc == 3 || fc == 5;
    let cnt02 = (cnt & !2u64) == 0;
    match raw_jt {
        1 => fc < 2,
        2 => fc == 4,
        4 => if cnt02 { fc < 2 } else { cif },
        0 => if v < 0x60e { if cnt02 { fc < 2 } else { cif } }
             else if v < 0x708 { if cnt == 2 { fc < 2 } else { cif } }
             else { cif },
        3 => if v < 0x60e { fc == 4 } else { cif },
        _ => cif,   // 방어(관측상 raw_jt∈0..4)
    }
}
unsafe fn my_e9a30_count(p3: usize, arg_cont: usize) -> Option<u64> {
    if !ptr_ok(p3) { return None; }   // ★readable VQ제거(이후 rd_u64(p3+...)? fault-safe)
    let facetcnt = rd_u64(p3 + 0x3d0)?;
    if facetcnt > 2 { return Some(0); }   // 조건1: facetcnt>2 → 전부 skip
    let threshold = rd_u64(p3 + 0x710)?;
    // LOOP1 pre-gate: [p3+0x3c8] vec 中 priority<4 있으면 return 0
    let l1_base = rd_u64(p3 + 0x3c8)? as usize;
    let l1_cnt = rd_u64(p3 + 0x3d0)?;   // == facetcnt
    if l1_base != 0 && l1_cnt <= 64 {
        for i in 0..l1_cnt as usize {
            let obj = rd_u64(l1_base + i*0x10)? as usize;
            if !ptr_ok(obj) { continue; }   // ★readable VQ제거(직후 rd_u64(obj+0x188)? fault-safe)
            if rd_u64(obj + 0x188)? < 4 { return Some(0); }
        }
    }
    // gather set: arg_cont → [+0x20] → [+8]base/[+0x10]len
    if !ptr_ok(arg_cont) { return Some(0); }   // ★readable VQ제거(직후 rd_u64(arg_cont+0x20)?)
    let sub = rd_u64(arg_cont + 0x20)? as usize;
    if !ptr_ok(sub) { return Some(0); }   // ★readable VQ제거(직후 rd_u64(sub+8/0x10)?)
    let base = rd_u64(sub + 8)? as usize;
    let len = rd_u64(sub + 0x10)?;
    if base == 0 || len > 256 { return Some(0); }
    // ★jumptable 정밀화: E9_JT on이면 JT/v 게터로 carry규칙(정확). off면 과대추정(필터①②③만).
    //   ★lazy: survivor(①②③통과) 처음 만났을 때만 jtv 호출 → survivor 없으면(count=0) jtv 불요=대체가능(passthrough 회수).
    let want_jt = E9_JT.load(Ordering::Relaxed);
    let mut jtv: Option<(u32, u64)> = None;
    let mut jtv_init = false;
    let mut count: u64 = 0;
    for i in 0..len as usize {
        let obj = rd_u64(base + i*0x10)? as usize;
        let vt = rd_u64(base + i*0x10 + 8)? as usize;   // 후보 vtable (게터 오프셋 타입별 상이)
        if !ptr_ok(obj) || !ptr_ok(vt) { continue; }   // ★readable VQ제거(직후 rd_u64(obj+0x180)?)
        let thr = rd_u64(obj + 0x180)?;       // vt0x60 (전타입 0x180)
        if thr > threshold { continue; }       // 조건2
        let pri = rd_u64(obj + 0x188)?;       // vt0x68 (전타입 0x188)
        if pri != 0 { continue; }              // 조건3
        if want_jt {                           // jumptable carry(facet code = vt0x98, 타입별 오프셋)
            if !jtv_init { jtv_init = true; jtv = my_e9a30_jt_v(p3); }
            let (raw_jt, vval) = match jtv { Some(x) => x, None => return None };  // survivor 있는데 jtv불명 → 불확실 → passthrough
            let fc = match cand_get(obj, vt, 0x98) { Some(f) => f as u32, None => return None };
            if !e9a30_carry(raw_jt, vval, fc, facetcnt) { continue; }
        }
        count += 1;
    }
    Some(count)
}
// ★count→gen_range(0,count) exit 예측(count>0일때만 1 draw; rejection=RngSim.gen_range 동일). (idx, refills) 반환.
unsafe fn my_e88a0_exit(rng_state: usize, count: u64) -> Option<(u64, u64)> {
    let i0 = rd_u64(rng_state + 0x100)?;
    if count == 0 { return Some((i0, 0)); }   // draw 없음
    let mut rng = RngSim::new(rng_state)?;
    rng.gen_range(0, count - 1)?;              // range = count
    Some((rng.idx, rng.refills))
}

// ★disc11 RNG caller 추적: POKE_INSCOPE 윈도우 중 fcd980/fcdaf0 호출의 caller RVA 기록(어느 함수가 serpen gen_range 호출하나 직접 특정). 2000개 캡.
unsafe fn poke_ret_log(which: &str, orig_ret: usize) {
    let n = POKE_RET_N.fetch_add(1, Ordering::Relaxed);
    if n >= 2000 { return; }
    let base = exe_base();
    let rva = if base != 0 && orig_ret >= base { orig_ret - base } else { orig_ret };
    if !POKERET_INIT.swap(true, Ordering::Relaxed) { write_named("pokerng_rets.txt", "=== disc9/11 in-scope gen_range caller RVA (어느 함수가 serpen draw 호출하나) ===\n"); }
    append_named("pokerng_rets.txt", &format!("[{}] callerRVA={:#x}\n", which, rva));
}
// ★FUN_1420e88a0(poke 후보선택자) 입력+후보객체 vtable 덤프: 게터(vt[0x50/0x60/0x68/0x78]) 해결 + 필터 재현용 구조 파악. 첫 40콜 e88a0.txt.
static E88_N: AtomicU64 = AtomicU64::new(0);
static E88_OK: AtomicU64 = AtomicU64::new(0);
static E88_DIFF: AtomicU64 = AtomicU64::new(0);
static E88_CMP_INIT: AtomicBool = AtomicBool::new(false);
// ★e88a0 선택출력 검증: 진입시 my_e88a0_pick(out0,out2) 예측 → 리턴서 게임 out([out0],[out+0x10]) 대조. (entry_rsp, out_ptr, my_out0, my_out2)
static E88_PICK: Mutex<Vec<(usize, usize, u64, i64)>> = Mutex::new(Vec::new());
static E88P_OK: AtomicU64 = AtomicU64::new(0);
static E88P_DIFF: AtomicU64 = AtomicU64::new(0);
static E88P_INIT: AtomicBool = AtomicBool::new(false);
// ★FUN_1420e88a0 RNG-sync 검증: 진입시 my_e88a0_count→gen_range(0,count) exit 예측 → 리턴훅 kind12서 실제 rng exit과 per-call 대조(타이밍무관). e88acmp.txt.
unsafe extern "C" fn e88a0_capture(saved: usize, entry_rsp: usize) {
    if COND_INSCOPE.load(Ordering::Relaxed) {   // condgate in-scope: e88a0 실제 draw(count>0)만 카운트
        let p4c = rd_u64(saved + 0x10).unwrap_or(0) as usize;
        let p7c = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize;
        if ptr_ok(p4c) && readable(p4c + 0x718, 8) {
            let cc = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_e88a0_count(p4c, p7c))).unwrap_or(None).unwrap_or(0);
            if cc > 0 { COND_IS_E88.fetch_add(1, Ordering::Relaxed); COND_IS_DRAWS.fetch_add(1, Ordering::Relaxed); }
        } else { COND_IS_E88.fetch_add(1, Ordering::Relaxed); COND_IS_DRAWS.fetch_add(1, Ordering::Relaxed); }  // 불명=보수적 카운트
    }
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    if E88_N.load(Ordering::Relaxed) >= 8000 { return; }
    let thunk = RET_THUNK.load(Ordering::Relaxed);
    if thunk == 0 { return; }
    let rng = rd_u64(saved + 0x18).unwrap_or(0) as usize;   // r8 = param_3 RNG state
    let p4 = rd_u64(saved + 0x10).unwrap_or(0) as usize;    // r9 = param_4 (cand holder)
    let p7 = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize; // stack param_7 (compare holder)
    if !ptr_ok(rng) || !readable(rng + 0x138, 8) || !ptr_ok(p4) || !readable(p4 + 0x718, 8) { return; }
    let count = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_e88a0_count(p4, p7))).unwrap_or(None) { Some(c) => c, None => return };
    let i0 = rd_u64(rng + 0x100).unwrap_or(0);
    let c0 = rd_u64(rng + 0x130).unwrap_or(0);
    let (pidx, prf) = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_e88a0_exit(rng, count))).unwrap_or(None) { Some(v) => v, None => (i0, 0) };
    let pctr = c0.wrapping_add(4u64.wrapping_mul(prf));
    // ★선택출력 예측: my_e88a0_pick(out0,out2) → 리턴서 게임 out 대조. out ptr=rcx(saved+0x28).
    let out_ptr = rd_u64(saved + 0x28).unwrap_or(0) as usize;
    if ptr_ok(out_ptr) {
        if let Some((my_o0, _my_o1, my_o2, _cnt)) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_e88a0_pick(p4, p7, rng))).unwrap_or(None) {
            if let Ok(mut pk) = E88_PICK.lock() { if pk.len() < 64 { pk.push((entry_rsp, out_ptr, my_o0, my_o2)); } }
        }
    }
    let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    if !ptr_ok(orig_ret) || !readable(entry_rsp, 8) { return; }
    let pre = format!("[e88a0 #{}] count={} i0={} c0={}", E88_N.load(Ordering::Relaxed), count, i0, c0);
    let pushed = if let Ok(mut st) = RET_STACK.lock() {
        if st.len() < 64 { st.push(RetFrame{ key: entry_rsp, orig_ret, mine: count as i64, kind: 12, pre, p5: rng, p6: pidx as usize, disp_pred: pctr as i64 }); true } else { false }
    } else { false };
    if pushed { core::ptr::write_unaligned(entry_rsp as *mut usize, thunk); E88_N.fetch_add(1, Ordering::Relaxed); }
}
// ── facet#5 engage draw1: FUN_1420e9a30 캡처(gather 입력 덤프 + 실제 draw1 RNG footprint 측정). cfg e9a30cap=1. 리턴훅 kind:13. ──
static E9_N: AtomicU64 = AtomicU64::new(0);
static E9_OK: AtomicU64 = AtomicU64::new(0);
static E9_DIFF: AtomicU64 = AtomicU64::new(0);
static E9_CAP: AtomicBool = AtomicBool::new(false);
static E9_FILE_INIT: AtomicBool = AtomicBool::new(false);
const E9_CAP_MAX: u64 = 4000;
unsafe extern "C" fn e9a30_capture(saved: usize, entry_rsp: usize) {
    if COND_INSCOPE.load(Ordering::Relaxed) { COND_IS_DRAWS.fetch_add(1, Ordering::Relaxed); COND_IS_E9.fetch_add(1, Ordering::Relaxed); }   // condgate in-scope(e9a30 호출)
    if !E9_CAP.load(Ordering::Relaxed) || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    if E9_N.load(Ordering::Relaxed) >= E9_CAP_MAX { return; }
    let thunk = RET_THUNK.load(Ordering::Relaxed);
    if thunk == 0 { return; }
    // ★engage 콜사이트(retreat_engage 0x206fa08의 복귀=0x206fa0d)만 필터링. e9a30는 5콜러(0x18ac7af/0x1eb4afb/0x206fa08/0x20dcceb/0x20e8ee3)라 다른 콜러 배제.
    {
        let orig_ret0 = rd_u64(entry_rsp).unwrap_or(0) as usize;
        let base0 = exe_base();
        if base0 == 0 || orig_ret0 != base0 + 0x206fa0d { return; }
    }
    let rng = rd_u64(saved + 0x20).unwrap_or(0) as usize;     // rdx = param2 = RNG state
    let p3  = rd_u64(saved + 0x18).unwrap_or(0) as usize;     // r8  = param3
    let arg_cont = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;  // arg6 = ARG_CONT(=[rbp+0xb8])
    if !ptr_ok(rng) || !readable(rng + 0x138, 8) || !ptr_ok(p3) || !readable(p3 + 0x718, 8) { return; }
    let i0 = rd_u64(rng + 0x100).unwrap_or(0);
    let c0 = rd_u64(rng + 0x130).unwrap_or(0);
    // tentative count (조건1-3 + pre-gate, e9jt에 따라 jumptable 적용)
    let count = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_e9a30_count(p3, arg_cont))).unwrap_or(None) { Some(c) => c, None => 0 };
    // ★range 역산 스위프: count>0이면 range 1..16 각각의 gen_range exit idx 로깅 → e9a30rng의 게임 gi와 c0로 대조해 진짜 range 특정.
    if count > 0 {
        let mut sw = String::new();
        for r in 1..=16u64 {
            let ex = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Option<u64> {
                let mut s = RngSim::new(rng)?; s.gen_range(0, r - 1)?; Some(s.refills.wrapping_mul(64).wrapping_add(s.idx))
            })).unwrap_or(None);
            sw.push_str(&format!("r{}={} ", r, ex.map(|x| x as i64).unwrap_or(-1)));
        }
        append_named("e9sweep.txt", &format!("[c0={} i0={} myc={}] {}\n", c0, i0, count, sw.trim()));
        // ★per-candidate carry 브레이크다운: raw_JT/v + filter①②③ 생존자별 fc·carry → off-by-one 정밀화용.
        let brk = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Option<String> {
            let (raw_jt, vval) = my_e9a30_jt_v(p3)?;
            let facetcnt = rd_u64(p3 + 0x3d0)?;
            let threshold = rd_u64(p3 + 0x710)?;
            let sub = rd_u64(arg_cont + 0x20)? as usize;
            let gb = rd_u64(sub + 8)? as usize;
            let glen = rd_u64(sub + 0x10)?;
            let mut s = format!("JT={} v={:#x} cnt={} thr={} glen={} surv[", raw_jt, vval, facetcnt, threshold, glen);
            let mut carry_n = 0u64;
            if ptr_ok(gb) && glen <= 256 {
                for i in 0..glen as usize {
                    let obj = rd_u64(gb + i*0x10)? as usize;
                    let vt = rd_u64(gb + i*0x10 + 8)? as usize;
                    if !ptr_ok(obj) || !ptr_ok(vt) { continue; }   // ★readable VQ제거(직후 rd_u64(obj+0x180)?)
                    if rd_u64(obj + 0x180)? > threshold { continue; }   // ②
                    if rd_u64(obj + 0x188)? != 0 { continue; }          // ③
                    let fc = cand_get(obj, vt, 0x98).unwrap_or(999) as u32;   // ★타입별 오프셋
                    let c = e9a30_carry(raw_jt, vval, fc, facetcnt);
                    if c { carry_n += 1; }
                    s.push_str(&format!("f{}{} ", fc, if c {"+"} else {"-"}));
                }
            }
            s.push_str(&format!("] carry={}", carry_n));
            Some(s)
        })).unwrap_or(None).unwrap_or_else(|| "brk_fail".into());
        append_named("e9break.txt", &format!("[c0={} i0={}] {}\n", c0, i0, brk));
    }
    // 예측 exit (count>0이면 1 u64 draw)
    let (pidx, prf) = if count > 0 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Option<(u64,u64)> {
            let mut sim = RngSim::new(rng)?; sim.gen_range(0, count - 1)?; Some((sim.idx, sim.refills))
        })).unwrap_or(None) { Some(v) => v, None => (i0, 0) }
    } else { (i0, 0) };
    let pctr = c0.wrapping_add(4u64.wrapping_mul(prf));
    // ── 덤프(첫 ~40콜 + 모든 count>0): gather 입력 상세 + [p3+0x440] vtable(K 해결용) ──
    let n = E9_N.load(Ordering::Relaxed);
    if n < 40 || count > 0 {
        let facetcnt = rd_u64(p3 + 0x3d0).unwrap_or(0);
        let thr = rd_u64(p3 + 0x710).unwrap_or(0);
        let kobj = rd_u64(p3 + 0x440).unwrap_or(0) as usize;
        let kvt = if ptr_ok(kobj) { rd_u64(kobj).unwrap_or(0) as usize } else { 0 };
        let base = exe_base();
        let kvt_rva = if base != 0 && kvt >= base { kvt - base } else { kvt };
        // ★jumptable 체인 read-only 검증(정정: obj440이 vtable객체 자체 → fn ptr=*(obj440+0x20/0x30) 직접). 호출X.
        let kslot20 = if ptr_ok(kobj) && readable(kobj + 0x20, 8) { rd_u64(kobj + 0x20).unwrap_or(0) as usize } else { 0 };  // JT게터 fn
        let kslot30 = if ptr_ok(kobj) && readable(kobj + 0x30, 8) { rd_u64(kobj + 0x30).unwrap_or(0) as usize } else { 0 };  // v게터 fn
        let buf438 = rd_u64(p3 + 0x438).unwrap_or(0) as usize;
        let olen = if ptr_ok(kobj) && readable(kobj + 0x10, 8) { rd_u64(kobj + 0x10).unwrap_or(0) } else { 0 };
        let vbuf = if ptr_ok(buf438) && olen > 0 { (((olen as usize).wrapping_sub(1)) & !0xfusize).wrapping_add(buf438).wrapping_add(0x10) } else { 0 };
        let s20rva = if base != 0 && kslot20 >= base { kslot20 - base } else { kslot20 };
        let s30rva = if base != 0 && kslot30 >= base { kslot30 - base } else { kslot30 };
        // 함수 ptr은 .text(code) 영역이어야 함 = ptr_ok + executable readable
        let chain_ok = ptr_ok(kobj) && ptr_ok(kslot20) && ptr_ok(kslot30) && ptr_ok(buf438) && vbuf != 0 && readable(vbuf, 8) && readable(kslot20, 4) && readable(kslot30, 4);
        // LOOP1 set priorities
        let mut l1 = String::new();
        let l1b = rd_u64(p3 + 0x3c8).unwrap_or(0) as usize;
        if ptr_ok(l1b) && facetcnt <= 64 {
            for i in 0..facetcnt as usize {
                let o = rd_u64(l1b + i*0x10).unwrap_or(0) as usize;
                if ptr_ok(o) && readable(o+0x190,8) { l1.push_str(&format!("p{} ", rd_u64(o+0x188).unwrap_or(999))); }
            }
        }
        // gather set: 각 cand thr/pri/facet + vtable 확인
        let mut gs = String::new(); let mut glen = 0u64;
        if ptr_ok(arg_cont) && readable(arg_cont+0x28,8) {
            let sub = rd_u64(arg_cont+0x20).unwrap_or(0) as usize;
            if ptr_ok(sub) && readable(sub+0x18,8) {
                let gb = rd_u64(sub+8).unwrap_or(0) as usize;
                glen = rd_u64(sub+0x10).unwrap_or(0);
                if ptr_ok(gb) && glen <= 64 {
                    for i in 0..glen as usize {
                        let o = rd_u64(gb+i*0x10).unwrap_or(0) as usize;
                        let v = rd_u64(gb+i*0x10+8).unwrap_or(0) as usize;
                        let vrva = if base!=0 && v>=base { v-base } else { v };
                        if ptr_ok(o) && readable(o+0x194,4) {
                            gs.push_str(&format!("[t={} p={} f={} vt={:#x}] ", rd_u64(o+0x180).unwrap_or(0), rd_u64(o+0x188).unwrap_or(0), rd_u32(o+0x190), vrva));
                        }
                    }
                }
            }
        }
        if !E9_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("e9a30.txt", "=== FUN_1420e9a30 gather 덤프: facetcnt/thr/K-vtable/LOOP1 pri/gather cands(t=thr p=pri f=facet vt=) + my_count + draw1 footprint + jumptable체인(s20/s30/vbuf/chain_ok) ===\n"); }
        append_named("e9a30.txt", &format!("[e9 #{}] facetcnt={} thr={} Kobj_vt={:#x} | LOOP1[{}] | glen={} gather: {} | my_count={} i0={} c0={} pred(idx={} ctr={}) | JT게터s20={:#x} v게터s30={:#x} buf438ok={} olen={} vbuf={:#x} chain_ok={}\n",
            n, facetcnt, thr, kvt_rva, l1.trim(), glen, gs.trim(), count, i0, c0, pidx, pctr, s20rva, s30rva, ptr_ok(buf438), olen, vbuf, chain_ok));
    }
    let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    if !ptr_ok(orig_ret) || !readable(entry_rsp, 8) { return; }
    let pre = format!("[e9 #{}] count={} i0={} c0={}", n, count, i0, c0);
    let pushed = if let Ok(mut st) = RET_STACK.lock() {
        if st.len() < 64 { st.push(RetFrame{ key: entry_rsp, orig_ret, mine: count as i64, kind: 13, pre, p5: rng, p6: pidx as usize, disp_pred: pctr as i64 }); true } else { false }
    } else { false };
    if pushed { core::ptr::write_unaligned(entry_rsp as *mut usize, thunk); E9_N.fetch_add(1, Ordering::Relaxed); }
}
// ★PRNG gen_range 검증 훅: fcd980(rcx=state, rdx=&{lo,hi})→rax=roll. 진입시 read-only 시뮬→리턴훅 kind:3서 실제와 대조.
unsafe extern "C" fn fcd980_capture(saved: usize, entry_rsp: usize) -> i64 {
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return RAX_SENT; }
    if DD7_INSCOPE.load(Ordering::Relaxed) { DD7_IS_980.fetch_add(1, Ordering::Relaxed); }   // dd7700 in-scope draw 진단
    if COND_INSCOPE.load(Ordering::Relaxed) { COND_IS_DRAWS.fetch_add(1, Ordering::Relaxed); COND_IS_DEF.fetch_add(1, Ordering::Relaxed); cond_site_log("fcd980", rd_u64(entry_rsp).unwrap_or(0) as usize); }   // condgate in-scope(fcd980=실제draw)+caller추적
    if POKE_INSCOPE.load(Ordering::Relaxed) { poke_ret_log("fcd980", rd_u64(entry_rsp).unwrap_or(0) as usize); }   // disc11 RNG caller 추적
    // ★facet#5 교전롤: 이 fcd980 호출의 복귀주소가 롤(RVA_ROLL_RET)이면 그 시점 RNG상태로 롤 예측 저장.
    //   ★step3: RNG_REPL on이면 우리 rng_advance_writeback로 롤 소비(게임 state 전진)+우리 롤 반환(원본 skip)=실전 RNG-sync.
    {
        let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
        let base = exe_base();
        if base != 0 && orig_ret == base + RVA_ROLL_RET {
            let st = rd_u64(saved + 0x28).unwrap_or(0) as usize;
            let rp = rd_u64(saved + 0x20).unwrap_or(0) as usize;
            if ptr_ok(st) && ptr_ok(rp) && readable(st + 0x100, 8) && readable(rp + 8, 8) {
                let (rlo, rhi) = (rd_u64(rp).unwrap_or(0), rd_u64(rp + 8).unwrap_or(0));
                if RNG_REPL.load(Ordering::Relaxed) {
                    // ★대체: write-back으로 게임 state 전진 + 우리 롤 반환(원본 fcd980 skip). read-only 예측과 동일값.
                    if let Some(roll) = rng_advance_writeback(st, rlo, rhi) {
                        PRED_ROLL.store(roll as i64, Ordering::Relaxed);
                        PRED_ROLL_VALID.store(true, Ordering::Relaxed);
                        RNG_REPL_N.fetch_add(1, Ordering::Relaxed);
                        return roll as i64;   // HANDLED → caller에 우리 롤
                    }
                    // writeback 실패 → passthrough(아래 read-only 예측만)
                }
                if CAP_ON.load(Ordering::Relaxed) {
                    let mut d = 0u32;
                    if let Some(roll) = rng_gen_range(st, rlo, rhi, &mut d) {
                        PRED_ROLL.store(roll as i64, Ordering::Relaxed);
                        PRED_ROLL_VALID.store(true, Ordering::Relaxed);
                    }
                }
            }
        }
    }
    if !RNGCAP.load(Ordering::Relaxed) || RNG_ARMED.load(Ordering::Relaxed) >= RNG_ARM_MAX { return RAX_SENT; }
    let thunk = RET_THUNK.load(Ordering::Relaxed);
    if thunk == 0 { return RAX_SENT; }
    let state = rd_u64(saved + 0x28).unwrap_or(0) as usize;   // rcx = RNG state
    let rangep = rd_u64(saved + 0x20).unwrap_or(0) as usize;  // rdx = &{lo,hi}
    if !ptr_ok(state) || !ptr_ok(rangep) || !readable(state + 0x100, 8) || !readable(rangep + 8, 8) { return RAX_SENT; }
    let lo = rd_u64(rangep).unwrap_or(0);
    let hi = rd_u64(rangep + 8).unwrap_or(0);
    let idx0 = rd_u64(state + 0x100).unwrap_or(0);
    let before_counter = rd_u64(state + 0x130).unwrap_or(0);   // input+0x20 = state+0x110+0x20
    let mut draws = 0u32;
    // ★rng_gen_range_st: 결과 + 예측 after-state(idx, refills). write-back 정확성 검증용.
    let (pred, my_after_idx, my_refills): (i64, u64, u64) = match rng_gen_range_st(state, lo, hi, &mut draws) {
        Some((v, fi, rf)) => (v as i64, fi, rf), None => (-888, 0, 0)
    };
    let my_after_counter = before_counter.wrapping_add(4u64.wrapping_mul(my_refills));
    let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    if !ptr_ok(orig_ret) || !readable(entry_rsp, 8) { return RAX_SENT; }
    let pre = format!("[rng #{}] lo={} hi={} idx0={} draws={} my_after[idx={} rf={}]", RNG_ARMED.load(Ordering::Relaxed), lo, hi, idx0, draws, my_after_idx, my_refills);
    let pushed = if let Ok(mut st) = RET_STACK.lock() {
        if st.len() < 64 { st.push(RetFrame{ key: entry_rsp, orig_ret, mine: pred, kind: 3, pre, p5: state, p6: my_after_idx as usize, disp_pred: my_after_counter as i64 }); true } else { false }
    } else { false };
    if pushed {
        core::ptr::write_unaligned(entry_rsp as *mut usize, thunk);
        RNG_ARMED.fetch_add(1, Ordering::Relaxed);
    }
    RAX_SENT   // passthrough (원본 fcd980 실행)
}

// ★드라이버 페이즈게이트 재현·검증 훅 (install_detour_pg). fcdaf0 호출 중 복귀주소=0x1d4f88e만 캡처.
//   A=rbx(saved+0x30) B=rdi(+0x38) C=rsi(+0x40) rng=rcx(+0). threshold=min(min(A*C/1000,100)*9+min(B,100)*2+100,1000).
//   roll=rng_gen_range(rng,0,1000)(read-only, PRNG 800/800 검증됨). transition=roll<threshold. → pgcmp.txt.
unsafe extern "C" fn fcdaf0_pg_capture(saved: usize, entry_rsp: usize) {
    if DD7_INSCOPE.load(Ordering::Relaxed) { DD7_IS_AF0.fetch_add(1, Ordering::Relaxed); }   // dd7700 in-scope draw 진단
    if COND_INSCOPE.load(Ordering::Relaxed) { COND_IS_DRAWS.fetch_add(1, Ordering::Relaxed); COND_IS_DEF.fetch_add(1, Ordering::Relaxed); cond_site_log("fcdaf0", rd_u64(entry_rsp).unwrap_or(0) as usize); }   // condgate in-scope(fcdaf0=실제draw)+caller추적
    if POKE_INSCOPE.load(Ordering::Relaxed) { poke_ret_log("fcdaf0", rd_u64(entry_rsp).unwrap_or(0) as usize); }   // disc11 RNG caller 추적
    let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    let base = exe_base();
    // ★0x1f80320 draw 카운트: 복귀주소가 0x1f80320의 9개 fcdaf0 호출 사이트 중 하나일 때만(외부 fcdaf0 오염 배제 = ground truth).
    if F80_INSCOPE.load(Ordering::Relaxed) && base != 0 {
        const F80_RETS: [usize; 9] = [0x205e429, 0x205e4dd, 0x205e58d, 0x205e63d, 0x205e6fd, 0x205e81d, 0x205e94d, 0x205ea4f, 0x205ea9c];  // 0.4.13_5 재추출
        let r = orig_ret.wrapping_sub(base);
        if F80_RETS.contains(&r) { F80_DRAWS.fetch_add(1, Ordering::Relaxed); }
    }
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    if base == 0 || orig_ret != base + RVA_PG_ROLL_RET { return; }   // 빠른필터: 페이즈게이트 롤만
    // ★A/B/C override: 스텁이 저장한 슬롯(rbx@+0x30/rdi@+0x38/rsi@+0x40) 덮어쓰면 pop이 복원→드라이버가 그 값으로 threshold.
    let (ov_a, ov_b, ov_c) = (PG_OV_A.load(Ordering::Relaxed), PG_OV_B.load(Ordering::Relaxed), PG_OV_C.load(Ordering::Relaxed));
    let override_on = ov_a>=0 || ov_b>=0 || ov_c>=0;
    if ov_a>=0 { core::ptr::write_unaligned((saved + 0x30) as *mut i64, ov_a); }
    if ov_b>=0 { core::ptr::write_unaligned((saved + 0x38) as *mut i64, ov_b); }
    if ov_c>=0 { core::ptr::write_unaligned((saved + 0x40) as *mut i64, ov_c); }
    if !PGCAP.load(Ordering::Relaxed) { return; }   // override는 위에서 이미 적용; 로깅은 PGCAP일때만
    if PG_ARMED.load(Ordering::Relaxed) >= PG_ARM_MAX { return; }
    let rng = rd_u64(saved + 0).unwrap_or(0) as usize;     // rcx
    let a = rd_i64(saved + 0x30).unwrap_or(0);             // rbx = A (override 적용 후)
    let b = rd_i64(saved + 0x38).unwrap_or(0);             // rdi = B
    let c = rd_i64(saved + 0x40).unwrap_or(0);             // rsi = C
    let obj = (((a as i128) * (c as i128)) / 1000).max(0).min(100) as i64;
    let bb = b.max(0).min(100);
    let thr = (obj*9 + bb*2 + 100).min(1000);
    let mut d = 0u32;
    let roll = if ptr_ok(rng) && readable(rng+0x100, 8) {
        rng_gen_range(rng, 0, 1000, &mut d).map(|v| v as i64).unwrap_or(-1)
    } else { -1 };
    let trans = if roll>=0 { if roll < thr {1} else {0} } else {-1};
    let n = PG_ARMED.fetch_add(1, Ordering::Relaxed);
    // ★0.4.13_5: ms(athlete) = 드라이버(0x1e9f280) [rbp+0x338] = [entry_rsp+0x3c0] (프롤로그 8push+sub0x408+lea rbp,[rsp+0x80] → rbp=entry_rsp-0x3c8, ms=rbp+0x338). Ghidra 0x141ea077b 확정: MOV RAX,[RBP+0x338]; MOV RBX,[RAX+0x218]/RDI,[+0x238]/RSI,[+0x380]. (3차핫픽스 sub0x448=[+0x400]은 stale)
    //   성향 스탯블록 덤프 → 선수단 UI 대조용. ms_ok = ms[0x218]==A && ms[0x238]==B (포인터 정확성 자체검증).
    let ms = rd_u64(entry_rsp + 0x3c0).unwrap_or(0) as usize;
    let (s218, s220, s228, s230, s238, team, lane, ms_ok) = if ptr_ok(ms) && readable(ms+0x238,8) {
        let a218 = rd_i64(ms+0x218).unwrap_or(-1); let a238 = rd_i64(ms+0x238).unwrap_or(-1);
        (a218, rd_i64(ms+0x220).unwrap_or(-1), rd_i64(ms+0x228).unwrap_or(-1),
         rd_i64(ms+0x230).unwrap_or(-1), a238,
         rd_i64(ms+0x6a8).unwrap_or(-1), rd_i32(ms+0x738).unwrap_or(-1) as i64,
         (a218==a && a238==b) as i32)
    } else { (-1,-1,-1,-1,-1,-1,-1,0) };
    if !PG_FILE_INIT.swap(true, Ordering::Relaxed) {
        write_named("pgcmp.txt", "=== 드라이버 페이즈게이트: A=ms[0x218] B=ms[0x238] C=ms[0x380]. +성향슬롯 덤프(0x218/0x220order/0x228/0x230aggr/0x238ego) team/lane, ms_ok=포인터검증 ===\n");
    }
    append_named("pgcmp.txt", &format!("[pg #{}] A={} B={} C={} obj={} thr={} roll={} trans={} | ms_ok={} t={} lane={} | s218={} s220(order)={} s228={} s230(aggr)={} s238(ego)={}\n",
        n, a, b, c, obj, thr, roll, trans, ms_ok, team, lane, s218, s220, s228, s230, s238));
}

// ★subplan_transition_engine(0x1d45290) 엔트리 캡처: phase=S[0x4ce]별 분포 + 입력덤프 → tecmp.txt.
//   args: rcx=S(champion/situation), rdx=A2(trigger code), r8=A3, r9=CAND(후보 subplan홀더, +0x430=disc/+0x420=payload/+0x228=확률스칼라).
//   출력(새 subplan)은 로컬 [rbp+0x4f0..0x508]에서 계산 → caller가 적용. 본 캡처는 입력+분포만(어떤 phase가 라이브인지 확인).
unsafe extern "C" fn trans_capture(saved: usize, _entry_rsp: usize) {
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    if !TECAP.load(Ordering::Relaxed) { return; }
    // install_detour saved 레이아웃(push rcx,rdx,r8,r9,r10,r11 역순): rcx@+0x28, rdx@+0x20, r8@+0x18, r9@+0x10
    let s = rd_u64(saved + 0x28).unwrap_or(0) as usize;       // rcx = S champion
    let a2 = rd_i64(saved + 0x20).unwrap_or(0);               // rdx = param2 (틱/카운트)
    let a3 = rd_i64(saved + 0x18).unwrap_or(0);               // r8  = param3
    let ath = rd_u64(saved + 0x10).unwrap_or(0) as usize;     // r9  = athlete A (★0.4.13: CAND 아님!)
    if !ptr_ok(s) || !readable(s + 0x4ce, 1) || !readable(s + 0x500, 8) { return; }
    let phase = std::ptr::read_unaligned((s + 0x4ce) as *const u8) as i64;
    let subdisc = if readable(s + 0x4cf, 1) { std::ptr::read_unaligned((s + 0x4cf) as *const u8) as i64 } else { -1 }; // 목표좌표 select
    let cur_sub = std::ptr::read_unaligned((s + 0x500) as *const u64) as i64;
    // 분포 히스토그램(모든 호출, 경량)
    TE_CALLS.fetch_add(1, Ordering::Relaxed);
    TE_PHASE_HIST[(phase as usize).min(15)].fetch_add(1, Ordering::Relaxed);
    TE_SUB_HIST[(cur_sub as usize).min(15)].fetch_add(1, Ordering::Relaxed);
    // ── 전환 이벤트 추적: 같은 챔피언(S ptr)의 subplan이 프레임간 바뀌면 = 실제 전환 발생 ──
    // 결정프레임 입력 스냅샷: 전환검출시 이전프레임(=함수가 새 subplan을 결정한 프레임)의 입력을 로그.
    let gate = if readable(s + 0x4d0, 1) { std::ptr::read_unaligned((s + 0x4d0) as *const u8) as i64 } else { -1 };
    // ★0.4.13 athlete 필드: [0x228]=확률스칼라(score=min(.,100)), [0x6a8]=team∈{0,1}(후보풀 인덱스+stage2 bit0), [0x738]=lane_kind(0~4)
    let (ath228, team, lane) = if ptr_ok(ath) && readable(ath + 0x738, 4) {
        (rd_i64(ath + 0x228).unwrap_or(-1), rd_i64(ath + 0x6a8).unwrap_or(-1), rd_i32(ath + 0x738).unwrap_or(-1) as i64)
    } else { (-1, -1, -1) };
    let inputs = [phase, gate, subdisc, ath228, team, lane, a2];
    if let Ok(mut tr) = TE_TRACK.lock() {
        let mut found = false;
        for e in tr.iter_mut() {
            if e.0 == s {
                found = true;
                if e.1 != cur_sub {
                    let tn = TE_TRANS_N.fetch_add(1, Ordering::Relaxed);
                    let d = e.2;  // 이전프레임 = 결정프레임 입력
                    append_named("tecmp.txt", &format!("  >>> [TRANS #{}] champ={:#x} {}->{} | DEC[phase={} gate={} subdisc={} ath228={} team={} lane={} A2={}]\n",
                        tn, s, e.1, cur_sub, d[0], d[1], d[2], d[3], d[4], d[5], d[6]));
                    e.1 = cur_sub;
                }
                e.2 = inputs;
                break;
            }
        }
        if !found && tr.len() < 64 { tr.push((s, cur_sub, inputs)); }
    }
    // 상세 덤프(처음 TE_ARM_MAX회)
    let n = TE_ARMED.load(Ordering::Relaxed);
    if n >= TE_ARM_MAX { return; }
    TE_ARMED.fetch_add(1, Ordering::Relaxed);
    if !TE_FILE_INIT.swap(true, Ordering::Relaxed) {
        write_named("tecmp.txt", "=== transition_engine(0.4.13) 입력캡처: phase=S[0x4ce], subdisc=S[0x4cf], cur_sub=S[0x500], gate=S[0x4d0], r9=athlete{228=scalar,6a8=team,738=lane}, A2=rdx, A3=r8 ===\n");
    }
    append_named("tecmp.txt", &format!(
        "[te #{}] phase={} subdisc={} cur_sub={} gate={} | ath228={} team={} lane={} | A2={} A3={}\n",
        n, phase, subdisc, cur_sub, gate, ath228, team, lane, a2, a3));
    // 50회마다 누적 히스토그램 스냅샷(클린 종료 없이도 분포 확보)
    if n % 50 == 49 {
        let mut ph = String::new();
        for k in 0..16 { let v = TE_PHASE_HIST[k].load(Ordering::Relaxed); if v>0 { ph.push_str(&format!("p{}={} ", k, v)); } }
        let mut sb = String::new();
        for k in 0..16 { let v = TE_SUB_HIST[k].load(Ordering::Relaxed); if v>0 { sb.push_str(&format!("s{}={} ", k, v)); } }
        append_named("tecmp.txt", &format!("--- HIST@{} calls={} | PHASE {}| SUBPLAN {}\n", n+1, TE_CALLS.load(Ordering::Relaxed), ph, sb));
    }
}

// ── fc59a0 base-score uVar21 + mult 완전재현 (disasm FUN_141d5b5d0) ──
// 빌더 f260f0(FUN_141d874b0 적위협=f6f720위치술어+cand_valid)/f26fd0(FUN_141d88200 아군오브젝트 geo술어+HP%>40).
// 반환 Some(mult)=게임 out[2]. None=계산불가. mult=(local_b0+1<=local_d0)?uVar21:uVar21+0x14. (RNG불필요, 진입시 결정론.)
const RECALL_MULT_NONE: i64 = -888888;
// 반환 (mult, u21f, b0, d0): u21f=mult분기 前 base, b0/d0=카운트 (mult 검증 로깅용).
// 반환 5번째=rng_drawn(true=full path 도달=RNG 1 u32 draw 소비; false=early-out=out{0,0,0} 무RNG).
unsafe fn my_recall_mult(sim: usize, p4: usize, mode: u8) -> Option<(i64,i64,i64,i64,bool)> {
    let _pg = perf_guard(4);
    let team = rd_u64(sim + 0x6a8).unwrap_or(9);
    if team > 1 { return None; }
    let other = 1u64.wrapping_sub(team);
    let l78 = rd_u64(p4).unwrap_or(0) as usize;
    let vobj_f6 = rd_u64(p4 + 8).unwrap_or(0) as usize;       // p4[1] = f6f720 위치술어 vobj
    let geo = rd_u64(p4 + 0x10).unwrap_or(0) as usize;        // p4[2] = geo
    if !ptr_ok(l78) || !ptr_ok(vobj_f6) || !ptr_ok(geo) { return None; }
    let self_obj = rd_u64(l78).unwrap_or(0) as usize;         // container[0][0]
    let rvt = rd_u64(l78 + 8).unwrap_or(0) as usize;          // container[0][1] = cand_valid vtable
    if !ptr_ok(self_obj) || !ptr_ok(rvt) { return None; }
    // 빌더1 적위협: (1-team) 레인 5슬롯 → f6f720(vobj,x,y,mode) && cand_valid
    let enemy_base = l78 + (other as usize)*0x28 + 0x1e0;
    let lvar16 = (other as usize)*0x228 + geo;
    let mut enemies = [0usize; 5]; let mut d0 = 0usize;       // ★Vec→스택배열(후보 ≤5, 힙할당 제거)
    for k in 0..5usize {
        let c = rd_u64(enemy_base + k*8).unwrap_or(0) as usize;
        if c == 0 { continue; }
        let (cx, cy) = (rd_u64(c+0x648).unwrap_or(0), rd_u64(c+0x650).unwrap_or(0));
        if !poke_f6f720(vobj_f6, cx, cy, mode) { continue; }
        match cand_valid(self_obj, rvt, team, lvar16, c) { Some(true)=>{ enemies[d0]=c; d0+=1; }, Some(false)=>{}, None=>return None }
    }
    if d0 == 0 { return Some((0,0,0,0,false)); }              // 적위협無 → score 0 (early-out, 무RNG)
    // 빌더2 아군오브젝트: team 레인 5슬롯 → geo술어(+0xf8==0 && +0xf9==mode) && HP%>40
    let ally_base = l78 + (team as usize)*0x28 + 0x1e0;
    let pred_base = geo + (team as usize)*0x228;
    let ally_hp_min = tune("rc_ally_hp_min", 0x28);          // ★호이스트: 아군 유효 HP% 하한(루프불변)
    let mut allies = [0usize; 5]; let mut b0 = 0usize;       // ★Vec→스택배열(≤5, 힙할당 제거)
    for k in 0..5usize {
        if rd_u8(pred_base + 0xf8 + k*0x20) != 0 || rd_u8(pred_base + 0xf9 + k*0x20) != mode { continue; }
        let c = rd_u64(ally_base + k*8).unwrap_or(0) as usize;
        if c == 0 { continue; }
        let mx = rd_u64(c+0x610).unwrap_or(0); if mx == 0 { continue; }
        if (rd_u64(c+0x658).unwrap_or(0).wrapping_mul(100) / mx) as i64 > ally_hp_min { allies[b0]=c; b0+=1; }
    }
    let self_ref = dd7_slot128(self_obj, rd_u64(sim + 0x6a0).unwrap_or(0));
    if b0 == 0 || b0 + 1 < d0 || self_ref == 0 { return Some((0,0,b0 as i64,d0 as i64,false)); }   // early-out, 무RNG
    // 최근접 적 (self_ref 기준)
    let (srx, sry) = (rd_u64(self_ref+0x648).unwrap_or(0), rd_u64(self_ref+0x650).unwrap_or(0));
    let mut ne = enemies[0]; let mut nd = sqd(srx,sry,rd_u64(ne+0x648).unwrap_or(0),rd_u64(ne+0x650).unwrap_or(0));
    for &e in &enemies[1..d0] { let d = sqd(srx,sry,rd_u64(e+0x648).unwrap_or(0),rd_u64(e+0x650).unwrap_or(0)); if d < nd { nd=d; ne=e; } }
    let (ex, ey) = (rd_u64(ne+0x648).unwrap_or(0), rd_u64(ne+0x650).unwrap_or(0));
    // 최근접 아군오브젝트 (적 기준)
    let mut na = allies[0]; let mut ad = sqd(ex,ey,rd_u64(na+0x648).unwrap_or(0),rd_u64(na+0x650).unwrap_or(0));
    for &a in &allies[1..b0] { let d = sqd(ex,ey,rd_u64(a+0x648).unwrap_or(0),rd_u64(a+0x650).unwrap_or(0)); if d < ad { ad=d; na=a; } }
    // 블록1: 적 HP%
    let emx = rd_u64(ne+0x610).unwrap_or(0); if emx == 0 { return None; }
    let ehp = (rd_u64(ne+0x658).unwrap_or(0).wrapping_mul(100) / emx) as i64;
    // ★튜닝(recall 적 HP% 블록): u21 초기값 + ehp 밴드 임계/값
    let mut u21: i64 = tune("rc_u21_init", -40);
    let ehp_t1 = tune("rc_ehp_t1", 0x50);   // 적 HP% 상한(이하서 가산)
    let ehp_t2 = tune("rc_ehp_t2", 0x3c);
    let ehp_t3 = tune("rc_ehp_t3", 0x28);
    let ehp_v2 = tune("rc_ehp_v2", 0x50);
    if ehp < ehp_t1 {
        if ehp < ehp_t2 { u21 = (if ehp < ehp_t3 { tune("rc_ehp_v1", 0x5a) } else { ehp_v2 }) - ehp; }
        else { u21 = (ehp_v2 - ehp) >> 1; }
    }
    // 블록2: 리콜포인트 → 적 거리. recall_point = l78[mode*4 - team + 0x31], 0이면 +0x33.
    let ri = (mode as i64)*4 - team as i64;
    let mut rp = rd_u64(l78 + ((ri + 0x31) as usize)*8).unwrap_or(0) as usize;
    if rp == 0 { rp = rd_u64(l78 + ((ri + 0x33) as usize)*8).unwrap_or(0) as usize; }
    if rp == 0 { u21 += tune("rc_norp_bonus", 0x23); }   // ★튜닝: 리콜포인트 없을때 가산
    else {
        let d = isqrt(sqd(rd_u64(rp+0x648).unwrap_or(0), rd_u64(rp+0x650).unwrap_or(0), ex, ey));
        // ★튜닝(recall 리콜포인트→적 거리 밴드): 임계 + 각 밴드 가감
        if d < tune("rc_ed_near", 130000) as u64 { u21 -= tune("rc_ed_near_pen", 0x3c); }
        else if d < tune("rc_ed_mid", 160000) as u64 {}
        else if d < tune("rc_ed_far", 200000) as u64 { u21 += tune("rc_ed_far_bonus", 0x14); }
        else { u21 += tune("rc_ed_vfar_bonus", 0x28); }
    }
    // 블록3: 아군오브젝트 HP% + obj→적 거리
    let amx = rd_u64(na+0x610).unwrap_or(0); if amx == 0 { return None; }
    let ahp = (rd_u64(na+0x658).unwrap_or(0).wrapping_mul(100) / amx) as i64;
    // ★튜닝(recall 아군 HP% 블록): u13 보너스 + ahp 밴드 임계/패널티
    let mut u13 = u21 + tune("rc_u13_bonus", 10);
    if ahp < tune("rc_ahp_t1", 0x46) { u13 = u21; }
    if ahp < tune("rc_ahp_t2", 0x32) { u13 = u21 - tune("rc_ahp2_pen", 0x1e); }
    let ad2 = isqrt(sqd(rd_u64(na+0x648).unwrap_or(0), rd_u64(na+0x650).unwrap_or(0), ex, ey));
    // ★튜닝(recall 아군→적 거리 밴드)
    let u21f = if ad2 < tune("rc_ad_near", 80000) as u64 { u13 + tune("rc_ad_near_bonus", 0xf) }
               else if ad2 < tune("rc_ad_mid", 0x1d4c1) as u64 { u13 }
               else { u13 - tune("rc_ad_far_pen", 0x19) };
    let mult = if (b0 as i64) + 1 <= d0 as i64 { u21f } else { u21f + tune("rc_mult_bonus", 0x14) };
    Some((mult, u21f, b0 as i64, d0 as i64, true))   // full path → RNG 1 draw 소비
}
// ★fc59a0(recall) 완전대체: out[0]=score(=m*mult/100), out[4]=bool(p6<=score), out[8]=mult. early-out(무RNG)=out{0,0,0}. full=u32 gen_range writeback(m=100-uv7..100+uv7, uv7=(1000-A)/20, A=sim[0x218]). 실패/미지→None(passthrough).
unsafe fn my_fc59a0_full(out: usize, prng: usize, sim: usize, p4: usize, mode: u8, p6: i64) -> Option<()> {
    if !writable(out, 0x10) { return None; }
    let (mult, _u21, _b0, _d0, rng_drawn) = my_recall_mult(sim, p4, mode)?;
    if !rng_drawn {
        // early-out: out = {0,0,0}, RNG 무소비
        std::ptr::write_unaligned(out as *mut i32, 0i32);
        std::ptr::write_unaligned((out + 4) as *mut u8, 0u8);
        std::ptr::write_unaligned((out + 8) as *mut i32, 0i32);
        return Some(());
    }
    // full path: 1 u32 gen_range draw + writeback
    let a = rd_i64(sim + 0x218).unwrap_or(-1);
    if a < 0 || a > 1000 { return None; }            // 범위밖 = 미지(게임 clamp 불확실) → passthrough
    let uv7 = ((1000 - a) / 20) as u64;
    let m = rng_advance_writeback_u32(prng, 100 - uv7 as i64, 2*uv7 + 1)?;
    let score = (((m * mult) / 100) + TUNE_RECALL_BIAS.load(Ordering::Relaxed)) as i32;   // ★튜닝: recall score 가산(>0=자주복귀)
    std::ptr::write_unaligned(out as *mut i32, score);
    std::ptr::write_unaligned((out + 4) as *mut u8, if p6 <= score as i64 { 1u8 } else { 0u8 });
    std::ptr::write_unaligned((out + 8) as *mut i32, mult as i32);
    Some(())
}

// ★fc59a0 recall RNG score 캡처: 진입시 A=sim[0x218]로 RNG배율 m 예측(read-only) + my_recall_mult(base score), 리턴훅 kind:5서 게임 출력(score/bool/mult)과 대조.
//   facet#5 retreat_engage → f28a50 → fc59a0 체인. score=(m*mult)/100. base score(uVar21)→mult 재현 완료.
unsafe extern "C" fn fc59a0_capture(saved: usize, entry_rsp: usize) -> i64 {
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return RAX_SENT; }
    if RECALLCAP.load(Ordering::Relaxed) { FC59_RAW.fetch_add(1, Ordering::Relaxed); }   // ★성능: 진단캡처 켜졌을때만(프로덕션 캐시라인 바운싱 제거)
    // ★recall 완전대체: my_fc59a0_full로 출력+RNG writeback → 원본 skip(rax=out ptr). 실패/미지→passthrough.
    if RECALL_REPL.load(Ordering::Relaxed) {
        let p1   = rd_u64(saved + 0x28).unwrap_or(0) as usize;   // rcx = out
        let prng = rd_u64(saved + 0x20).unwrap_or(0) as usize;   // rdx = RNG state
        let sim  = rd_u64(saved + 0x18).unwrap_or(0) as usize;   // r8  = sim
        let p4   = rd_u64(saved + 0x10).unwrap_or(0) as usize;   // r9  = cand src
        let mode = rd_u8(entry_rsp + 0x28);                      // arg5 = mode
        let p6   = rd_i32(entry_rsp + 0x30).unwrap_or(0) as i64; // arg6 = threshold
        if ptr_ok(p1) && ptr_ok(prng) && ptr_ok(sim) && ptr_ok(p4) && readable(prng + 0x130, 8) && readable(sim + 0x220, 8) {
            let done = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_fc59a0_full(p1, prng, sim, p4, mode, p6))).unwrap_or(None).is_some();
            if done { RECALL_REPL_N.fetch_add(1, Ordering::Relaxed); return p1 as i64; }   // HANDLED → rax=out, 원본 skip
        }
        RECALL_REPL_PASS.fetch_add(1, Ordering::Relaxed);   // 미지/실패 → passthrough(원본 RNG소비)
    }
    if !RECALLCAP.load(Ordering::Relaxed) { return RAX_SENT; }
    if RECALL_ARMED.load(Ordering::Relaxed) >= RECALL_ARM_MAX { return RAX_SENT; }
    let thunk = RET_THUNK.load(Ordering::Relaxed);
    if thunk == 0 { FC59_FILT.fetch_add(1, Ordering::Relaxed); return RAX_SENT; }
    let p1   = rd_u64(saved + 0x28).unwrap_or(0) as usize;   // rcx = out ptr
    let prng = rd_u64(saved + 0x20).unwrap_or(0) as usize;   // rdx = RNG state
    let sim  = rd_u64(saved + 0x18).unwrap_or(0) as usize;   // r8  = sim_state
    let p6   = rd_i32(entry_rsp + 0x30).unwrap_or(0) as i64; // stack arg6 = threshold
    if !ptr_ok(p1) || !ptr_ok(prng) || !ptr_ok(sim) { FC59_FILT.fetch_add(1, Ordering::Relaxed); return RAX_SENT; }
    if !readable(prng + 0x130, 8) || !readable(sim + 0x218, 8) { FC59_FILT.fetch_add(1, Ordering::Relaxed); return RAX_SENT; }
    let a = rd_i64(sim + 0x218).unwrap_or(-1);               // 전술/공격성 A
    // RNG 배율 m = gen_range(100-uVar7 .. 100+uVar7), uVar7=(1000-A)/20. read-only 예측.
    let my_m: i64 = if a >= 0 && a <= 1000 {
        let uv7 = ((1000 - a) / 20) as u64;
        rng_gen_range_u32(prng, 100 - uv7 as i64, 2*uv7 + 1).unwrap_or(-777)
    } else { -777 };
    let p4 = rd_u64(saved + 0x10).unwrap_or(0) as usize;     // r9 = cand src
    let mode = rd_u8(entry_rsp + 0x28);                      // stack arg5 = p5 byte (lane/objective type)
    let (my_mult, my_u21, my_b0, my_d0, _rng) = my_recall_mult(sim, p4, mode).unwrap_or((RECALL_MULT_NONE,0,0,0,false));
    let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    if !ptr_ok(orig_ret) || !readable(entry_rsp, 8) { return RAX_SENT; }
    let pre = format!("[recall #{}] A={} thr={} my_m={} mode={} my_mult={} u21={} b0={} d0={}", RECALL_ARMED.load(Ordering::Relaxed), a, p6, my_m, mode, my_mult, my_u21, my_b0, my_d0);
    let pushed = if let Ok(mut st) = RET_STACK.lock() {
        if st.len() < 64 { st.push(RetFrame{ key: entry_rsp, orig_ret, mine: my_m, kind: 5, pre, p5: p1, p6: my_mult as usize, disp_pred: p6 }); true } else { false }
    } else { false };
    if pushed {
        core::ptr::write_unaligned(entry_rsp as *mut usize, thunk);
        RECALL_ARMED.fetch_add(1, Ordering::Relaxed);
        FC59_ARM.fetch_add(1, Ordering::Relaxed);
    } else { FC59_FILT.fetch_add(1, Ordering::Relaxed); }
    RAX_SENT   // 캡처는 passthrough(원본 실행 후 kind5 리턴훅 검증)
}

// ── CAND_FILTER white-box 재현 모듈 (cand_filter_repro/load_ctx/CandCtx/trans_should_commit) ──
include!("cand_filter_repro.rs");

// ★CAND_FILTER(0x1f4ec60) white-box 검증 캡처: 진입시 RNG 미소비 상태로 cand_filter_repro 예측(read-only)
//   → 리턴훅 kind9서 게임 출력 Vec(len/요소합)과 대조. fcdaf0=RngSim 재현, getter=dd7_slot*.
unsafe extern "C" fn cand_filter_capture(saved: usize, entry_rsp: usize) {
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    CAND_RAW.fetch_add(1, Ordering::Relaxed);
    // ★dd7700-호출 CAND_FILTER ground-truth(candcap 무관): 실제 ctx.rng/lo/hi/lane/draws/candtable → 내 dd7700 RNG예측 가정 검증.
    if DD7_REPL.load(Ordering::Relaxed) || true {
        let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
        let base = exe_base();
        if base != 0 && DD7CF_N.load(Ordering::Relaxed) < 80 {
            let rva = orig_ret.wrapping_sub(base);
            if rva >= 0x18ae610 && rva < 0x18b0000 {   // dd7700 본체 범위 = dd7700이 호출한 CAND_FILTER
                let ctx_ptr = rd_u64(saved + 0x20).unwrap_or(0) as usize;
                if let Some(c) = load_ctx(ctx_ptr) {
                    let team_raw = rd_i64(c.athlete + 0x6a8).unwrap_or(-1);
                    let (cmask, rngidx) = if team_raw == 0 || team_raw == 1 {
                        let team = (1 - team_raw) as usize;
                        let mut m = 0u64; for l in 0..5usize { if rd_u64(c.rhd + 0x1e0 + (team*5+l)*8).unwrap_or(0) != 0 { m |= 1 << l; } }
                        (m, rd_u64(c.rng + 0x100).unwrap_or(0))
                    } else { (0xff, 0) };
                    let mut dr = 0u32; let _ = cand_filter_repro(&c, &mut dr);
                    // ★윈도우식 직접검증: ctx의 param_5(=c.athlete2)로 내 lo/hi 재계산 → 실측 c.lo/c.hi와 대조. match면 my_dd7700_rng_final 윈도우식 정확.
                    let (my_lo, my_hi) = {
                        let a380 = rd_i64(c.athlete2 + 0x380).unwrap_or(0);
                        let a218 = rd_i64(c.athlete2 + 0x218).unwrap_or(0);
                        let t = ((a380.wrapping_mul(a218) as u64) / 1000).min(100);
                        let half = 0x384u64.wrapping_sub(t.wrapping_mul(9)) >> 1;
                        (0x3e8u64.wrapping_sub(half) as i64, 0x3e8u64.wrapping_add(half) as i64)
                    };
                    let wmatch = if my_lo == c.lo && my_hi == c.hi { "OK" } else { "★MISS" };
                    let n = DD7CF_N.fetch_add(1, Ordering::Relaxed);
                    if !DD7CF_INIT.swap(true, Ordering::Relaxed) { write_named("dd7cf.txt", "=== dd7700-호출 CAND_FILTER ground-truth (실측 ctx; 내 가정=rng:p4 lane:0..5 와 대조) + 윈도우식 직접검증(my vs real lo/hi) ===\n"); }
                    append_named("dd7cf.txt", &format!("[dd7cf {}] retRVA={:#x} rng={:#x} rngIdx={} lo={} hi={} lane={}..{} repro_draws={} cmask05=0b{:05b} team_raw={} | win: my({},{}) real({},{}) {}\n",
                        n, rva, c.rng, rngidx, c.lo, c.hi, c.lane_start, c.lane_end, dr, cmask, team_raw, my_lo, my_hi, c.lo, c.hi, wmatch));
                }
            }
        }
    }
    if !CANDCAP.load(Ordering::Relaxed) { return; }
    if CAND_ARMED.load(Ordering::Relaxed) >= CAND_ARM_MAX { return; }
    let thunk = RET_THUNK.load(Ordering::Relaxed);
    if thunk == 0 { return; }
    let out_ptr = rd_u64(saved + 0x28).unwrap_or(0) as usize;   // rcx = out Vec
    let ctx_ptr = rd_u64(saved + 0x20).unwrap_or(0) as usize;   // rdx = 14필드 ctx
    if !ptr_ok(out_ptr) || !ptr_ok(ctx_ptr) { CAND_FILT.fetch_add(1, Ordering::Relaxed); return; }
    let ctx = match load_ctx(ctx_ptr) { Some(c) => c, None => { CAND_FILT.fetch_add(1, Ordering::Relaxed); return; } };
    let mut draws = 0u32;
    let pred = match cand_filter_repro(&ctx, &mut draws) { Some(v) => v, None => { CAND_FILT.fetch_add(1, Ordering::Relaxed); return; } };
    let my_len = pred.len();
    let my_sum = pred.iter().fold(0usize, |a, &x| a.wrapping_add(x));
    let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    if !ptr_ok(orig_ret) || !readable(entry_rsp, 8) { return; }
    if let Ok(mut g) = CAND_PRED.lock() { *g = (out_ptr, pred); }
    let pre = format!("[cand #{}] lane={}..{} lo={} hi={} draws={} my_len={} my_sum={:#x}",
        CAND_ARMED.load(Ordering::Relaxed), ctx.lane_start, ctx.lane_end, ctx.lo, ctx.hi, draws, my_len, my_sum);
    let pushed = if let Ok(mut st) = RET_STACK.lock() {
        if st.len() < 64 { st.push(RetFrame{ key: entry_rsp, orig_ret, mine: my_len as i64, kind: 9, pre, p5: out_ptr, p6: my_sum, disp_pred: 0 }); true } else { false }
    } else { false };
    if pushed { core::ptr::write_unaligned(entry_rsp as *mut usize, thunk); CAND_ARMED.fetch_add(1, Ordering::Relaxed); }
    else { CAND_FILT.fetch_add(1, Ordering::Relaxed); }
}

// ── generic_build 스코어러 재현 모듈 (my_f80320/F80Ctx) ──
include!("genbuild_repro.rs");

// ★0x1f80320 스코어러 white-box 검증 캡처: 진입 RNG 미소비 상태로 my_f80320 예측 → 리턴훅 kind11서 game score 대조.
unsafe extern "C" fn f80320_capture(saved: usize, entry_rsp: usize) {
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    GB_RAW.fetch_add(1, Ordering::Relaxed);
    if !GBCAP.load(Ordering::Relaxed) { return; }
    if GB_ARMED.load(Ordering::Relaxed) >= GB_ARM_MAX { return; }
    let thunk = RET_THUNK.load(Ordering::Relaxed);
    if thunk == 0 { return; }
    let mbase = exe_base();
    if mbase == 0 { return; }
    let ctx = F80Ctx {
        p1:      rd_u64(saved + 0x28).unwrap_or(0),           // rcx
        rng:     rd_u64(saved + 0x20).unwrap_or(0) as usize,  // rdx
        p3:      rd_u64(saved + 0x18).unwrap_or(0) as usize,  // r8
        athlete: rd_u64(saved + 0x10).unwrap_or(0) as usize,  // r9
        p5:      rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize,
        p6:      rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize,
        p7:      rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize,
    };
    if !ptr_ok(ctx.rng) || !ptr_ok(ctx.p3) || !ptr_ok(ctx.p5) || !ptr_ok(ctx.athlete) { return; }
    // ★빈-능력 후보 skill 게터(vt+0x90/+0xa8) 프로브: 모든 list1 후보 중 능력리스트 빈 것만 덤프.
    let grw = GB_RAW.load(Ordering::Relaxed);
    if grw % 8 == 1 {
        let rva = |p: usize| -> i64 { if p > mbase && p < mbase + 0x10000000 { (p - mbase) as i64 } else { p as i64 } };
        let l1b = rd_u64(ctx.p6).unwrap_or(0) as usize; let l1n = rd_u64(ctx.p6+0x18).unwrap_or(0).min(5);
        for i in 0..l1n {
            let elem = rd_u64(l1b + (i as usize)*8).unwrap_or(0) as usize;
            if !ptr_ok(elem) || !readable(elem, 0x600) { continue; }
            let abn = rd_u64(elem+0x2b8).unwrap_or(99);
            if abn != 0 { continue; }   // 빈-능력만
            let mut s = format!("[empty elem={:#x} r={} 3e8={} b8={} c0={} c8={} 4e0={}]",
                rva(elem), rd_i32(elem+0x68).unwrap_or(-1), rd_i32(elem+0x3e8).unwrap_or(-1),
                rd_u64(elem+0xb8).unwrap_or(0), rd_u64(elem+0xc0).unwrap_or(0), rd_u64(elem+0xc8).unwrap_or(0),
                rd_i32(elem+0x4e0).unwrap_or(-99));
            for (lbl, doff, voff) in [("s1",0x568usize,0x570usize),("s2",0x578,0x580),("s3",0x588,0x590)] {
                let vt = rd_u64(elem+voff).unwrap_or(0) as usize; let data = rd_u64(elem+doff).unwrap_or(0) as usize;
                let g90 = rd_u64(vt+0x90).unwrap_or(0) as usize; let ga8 = rd_u64(vt+0xa8).unwrap_or(0) as usize;
                // emulate 결과도 함께 (내가 계산하는 값)
                let e90 = emulate_getter(g90, data).unwrap_or(-999); let ea8 = emulate_getter(ga8, data).unwrap_or(-999);
                s.push_str(&format!(" {}[+90={:#x}→{} +a8={:#x}→{}]", lbl, rva(g90), e90, rva(ga8), ea8));
            }
            s.push('\n');
            append_named("gbready.txt", &s);
        }
    }
    let entry_idx = rd_u64(ctx.rng + 0x100).unwrap_or(0) as i64;
    // my_f80320 = 순수-read(게임호출 제로). 1차=draw 정렬 검증(score는 list1 부분값).
    // ★panic-safe: my_f80320 내부 panic(div0/overflow 등)이 FFI UB로 게임 크래시 → catch_unwind로 차단.
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_f80320(&ctx, mbase)));
    let (my_score, my_draws) = match res { Ok(Some(v)) => v, _ => return };
    let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    if !ptr_ok(orig_ret) || !readable(entry_rsp, 8) { return; }
    // 진단: list1 각 후보의 능력 타입 멀티셋(#333류 +draw 원인 규명용)
    let mut abty = String::new();
    {
        let l1b = rd_u64(ctx.p6).unwrap_or(0) as usize; let l1n = rd_u64(ctx.p6+0x18).unwrap_or(0).min(6);
        for i in 0..l1n {
            let e = rd_u64(l1b + (i as usize)*8).unwrap_or(0) as usize;
            if !ptr_ok(e) { continue; }
            let ab = rd_u64(e+0x2b0).unwrap_or(0) as usize; let abn = rd_u64(e+0x2b8).unwrap_or(0).min(12);
            abty.push_str(&format!(" c{}r{}[", i, rd_i32(e+0x68).unwrap_or(-1)));
            for k in 0..abn { abty.push_str(&format!("{},", rd_i32(ab + (k as usize)*0x28).unwrap_or(-1))); }
            abty.push(']');
        }
    }
    // 빈-능력 첫 후보 정밀진단: s5b0/b8/c0/c8/thr + gb술어 결과 + 슬롯 pass
    let mut slotdbg = String::new();
    {
        let l1b = rd_u64(ctx.p6).unwrap_or(0) as usize;
        if rd_u64(ctx.p6+0x18).unwrap_or(0) > 0 {
            let e = rd_u64(l1b).unwrap_or(0) as usize;
            if ptr_ok(e) && readable(e, 0x600) && rd_u64(e+0x2b8).unwrap_or(99)==0 {
                let p3_1 = rd_u64(ctx.p3+8).unwrap_or(0) as usize;
                let thr = rd_i64(rd_u64(p3_1+8).unwrap_or(0) as usize + 0x12f8).unwrap_or(0);
                let r13 = rd_i32(e+0x68).unwrap_or(0)==0xd;
                let b8=rd_i64(e+0xb8).unwrap_or(0); let c0=rd_i64(e+0xc0).unwrap_or(0); let c8=rd_i64(e+0xc8).unwrap_or(0);
                let p1r=gb_dfd1e0(e,mbase); let p2r=gb_dec4d0(e,mbase); let p3r=gb_dfb1a0(e,mbase);
                let g1=p1r.unwrap_or(false)|| !r13 || b8<=thr;
                let g2=p2r.unwrap_or(false)|| !r13 || c0<=thr;
                let g3=p3r.unwrap_or(false)|| !r13 || c8<=thr;
                slotdbg=format!(" |s5b0={} thr={} b8={} c0={} c8={} d1={:?}/g{} d2={:?}/g{} d3={:?}/g{}",
                    rd_u64(e+0x5b0).unwrap_or(0), thr, b8, c0, c8, p1r, g1 as u8, p2r, g2 as u8, p3r, g3 as u8);
            }
        }
    }
    let sites = format!(" SITE[base={} slot={} t78={} t7d={} t82={} l2={}]",
        GB_SITE[0].load(Ordering::Relaxed), GB_SITE[1].load(Ordering::Relaxed), GB_SITE[2].load(Ordering::Relaxed),
        GB_SITE[3].load(Ordering::Relaxed), GB_SITE[4].load(Ordering::Relaxed), GB_SITE[5].load(Ordering::Relaxed));
    let pre = format!("[gb #{}] p1={} l1={} l2={} my_draws={} my_pscore={}{}{}{}",
        GB_ARMED.load(Ordering::Relaxed), ctx.p1,
        rd_u64(ctx.p6 + 0x18).unwrap_or(0), rd_u64(ctx.p7 + 0x18).unwrap_or(0), my_draws, my_score, abty, slotdbg, sites);
    let pushed = if let Ok(mut st) = RET_STACK.lock() {
        if st.len() < 64 { st.push(RetFrame{ key: entry_rsp, orig_ret, mine: my_score as i64, kind: 11, pre, p5: ctx.rng, p6: my_draws as usize, disp_pred: entry_idx }); true } else { false }
    } else { false };
    if pushed {
        // ★게임 0x1f80320 본체의 fcdaf0 draw를 카운트하도록 스코프 시작(my_f80320은 RngSim이라 fcdaf0 미호출=무영향).
        F80_DRAWS.store(0, Ordering::Relaxed);
        F80_INSCOPE.store(true, Ordering::Relaxed);
        core::ptr::write_unaligned(entry_rsp as *mut usize, thunk);
        GB_ARMED.fetch_add(1, Ordering::Relaxed);
    }
}

// ★영역 D callee 0x203cb30 검증 캡처(task#2): 진입 entity ptr(rcx=rh/rdx=a/r8=s) → my_203cb30 예측 → 리턴훅 kind20서 game retval(rax) 대조.
//   순수 점수함수(RNG미소비)라 함수시작 detour만으로 game==mine. install_detour saved: rcx@+0x28, rdx@+0x20, r8@+0x18.
unsafe extern "C" fn gb203_capture(saved: usize, entry_rsp: usize) {
    GBC203_RAW.fetch_add(1, Ordering::Relaxed);   // ★최상단: 모든 진입(게이트 前). "호출됨?" 판정용.
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    GBC_RAW.fetch_add(1, Ordering::Relaxed);
    if !GBCALLEE.load(Ordering::Relaxed) { return; }
    if GBC_ARMED.load(Ordering::Relaxed) >= GBC_ARM_MAX { return; }
    let thunk = RET_THUNK.load(Ordering::Relaxed);
    if thunk == 0 { return; }
    let mbase = exe_base();
    if mbase == 0 { return; }
    let rh = rd_u64(saved + 0x28).unwrap_or(0) as usize;   // rcx = resolver 핸들
    let a  = rd_u64(saved + 0x20).unwrap_or(0) as usize;   // rdx = 점수대상 엔티티
    let s  = rd_u64(saved + 0x18).unwrap_or(0) as usize;   // r8  = S(combat 보조)
    if !ptr_ok(rh) || !ptr_ok(a) || !ptr_ok(s) { GBC_BADPTR.fetch_add(1, Ordering::Relaxed); return; }
    let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    if !ptr_ok(orig_ret) || !readable(entry_rsp, 8) { GBC_BADPTR.fetch_add(1, Ordering::Relaxed); return; }
    // ★panic-safe(mod-safety): my_203cb30 내부 panic(div0/overflow/stale ptr)=FFI UB → catch_unwind 차단.
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_203cb30(rh, a, s, mbase)));
    let mine = match res { Ok(v) => v, _ => { GBC_PANIC.fetch_add(1, Ordering::Relaxed); return; } };
    let pre = format!("[gbc203 #{}] a={:#x} my={}", GBC_ARMED.load(Ordering::Relaxed), a, mine);
    let pushed = if let Ok(mut st) = RET_STACK.lock() {
        if st.len() < 64 { st.push(RetFrame{ key: entry_rsp, orig_ret, mine: mine as i64, kind: 20, pre, p5: 203, p6: 0, disp_pred: -99 }); true } else { false }
    } else { false };
    if pushed {
        core::ptr::write_unaligned(entry_rsp as *mut usize, thunk);
        GBC_ARMED.fetch_add(1, Ordering::Relaxed);
    }
}
// ★영역 D callee 0x20c0690 검증 캡처(task#2): rcx=&{[0]=rh,[8]=a,[0x10]=S} 구조체 ptr → my_20c0690 예측 → 리턴훅 kind20.
unsafe extern "C" fn gb690_capture(saved: usize, entry_rsp: usize) {
    GBC690_RAW.fetch_add(1, Ordering::Relaxed);   // ★최상단: 모든 진입(게이트 前).
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    GBC_RAW.fetch_add(1, Ordering::Relaxed);
    if !GBCALLEE.load(Ordering::Relaxed) { return; }
    if GBC_ARMED.load(Ordering::Relaxed) >= GBC_ARM_MAX { return; }
    let thunk = RET_THUNK.load(Ordering::Relaxed);
    if thunk == 0 { return; }
    let mbase = exe_base();
    if mbase == 0 { return; }
    let ctxp = rd_u64(saved + 0x28).unwrap_or(0) as usize;   // rcx = &{rh,a,s}
    if !ptr_ok(ctxp) || !readable(ctxp, 0x18) { GBC_BADPTR.fetch_add(1, Ordering::Relaxed); return; }
    let rh = rd_u64(ctxp).unwrap_or(0) as usize;
    let a  = rd_u64(ctxp + 8).unwrap_or(0) as usize;
    let s  = rd_u64(ctxp + 0x10).unwrap_or(0) as usize;
    if !ptr_ok(a) || !ptr_ok(s) { GBC_BADPTR.fetch_add(1, Ordering::Relaxed); return; }
    let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    if !ptr_ok(orig_ret) || !readable(entry_rsp, 8) { GBC_BADPTR.fetch_add(1, Ordering::Relaxed); return; }
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_20c0690(rh, a, s, mbase)));
    let mine = match res { Ok(v) => v, _ => { GBC_PANIC.fetch_add(1, Ordering::Relaxed); return; } };
    let pre = format!("[gbc690 #{}] a={:#x} my={}", GBC_ARMED.load(Ordering::Relaxed), a, mine);
    let pushed = if let Ok(mut st) = RET_STACK.lock() {
        if st.len() < 64 { st.push(RetFrame{ key: entry_rsp, orig_ret, mine: mine as i64, kind: 20, pre, p5: 690, p6: 0, disp_pred: -99 }); true } else { false }
    } else { false };
    if pushed {
        core::ptr::write_unaligned(entry_rsp as *mut usize, thunk);
        GBC_ARMED.fetch_add(1, Ordering::Relaxed);
    }
}
// ★영역 D 출력검증 캡처(cfg gbrd): mid-func 0x20e42a3서 RegionD 입력로컬(rbp/r12/r13) → gb_region_d 예측 → out ptr 키로 GBRD_MAP 저장.
//   ★mid-func라 return 하이재킹 불가(entry_rsp=함수 리턴슬롯 아님) → 저장만. 대조는 generic_build 리턴훅(kind14)이 같은 out ptr로 수행.
//   saved: rbp@+0x38, r12@+0x40, r13@+0x48 (install_detour_d 레이아웃). 순수 read + gb_region_d(순수) = 게임호출 제로. panic-safe.
type Dedc0Fn = unsafe extern "C" fn(usize, usize, usize) -> u8;   // FUN_1420dedc0(out, _, bundle) → al(bool)
// ★반환 i64(install_detour_d_skip용): RAX_SENT=passthrough(게임 region D 실행) / out ptr=HANDLED(skip, 우리출력 기록후 funnel jump).
//   verify(gbrd)/overwrite(gbrepl/chk)는 항상 SENT(passthrough+capture). skip(gbskip)만 Some&&push==0시 out 반환(진짜 계산대체).
unsafe extern "C" fn gbrd_capture(saved: usize, _entry_rsp: usize) -> i64 {
    if GBRD.load(Ordering::Relaxed) || GBREPL.load(Ordering::Relaxed) || GBREPLCHK.load(Ordering::Relaxed) { GBRD_RAW.fetch_add(1, Ordering::Relaxed); }   // ★성능: 진단캡처 켜졌을때만(프로덕션 캐시라인 바운싱 제거)
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return RAX_SENT; }
    if !GBRD.load(Ordering::Relaxed) && !GBREPL.load(Ordering::Relaxed) && !GBREPLCHK.load(Ordering::Relaxed) && !GBSKIP.load(Ordering::Relaxed) { return RAX_SENT; }
    // ★대체/체크/skip은 store 캡 없음(매 호출). verify(GBRD-only)만 4000 캡.
    if !GBREPL.load(Ordering::Relaxed) && !GBREPLCHK.load(Ordering::Relaxed) && !GBSKIP.load(Ordering::Relaxed) && GBRD_ARMED.load(Ordering::Relaxed) >= GBRD_ARM_MAX { return RAX_SENT; }
    let mbase = exe_base();
    if mbase == 0 { return RAX_SENT; }
    let rbp = rd_u64(saved + 0x38).unwrap_or(0) as usize;   // 게임 rbp(프레임베이스)
    let r12 = rd_u64(saved + 0x40).unwrap_or(0);            // self보간거리²
    let r13 = rd_u64(saved + 0x48).unwrap_or(0);            // A*경로거리²
    if !ptr_ok(rbp) || !readable(rbp + 0x290, 8) { GBRD_BADPTR.fetch_add(1, Ordering::Relaxed); return RAX_SENT; }
    let out = rd_u64(rbp + 0x290).unwrap_or(0) as usize;    // out struct ptr (= kind14 f.p5와 동일 키)
    if !ptr_ok(out) || !readable(out + 0x8b, 1) { GBRD_BADPTR.fetch_add(1, Ordering::Relaxed); return RAX_SENT; }
    let ath = rd_u64(rbp + 0x280).unwrap_or(0) as usize;    // [0x280]=대상 athlete 엔티티
    let t_p = rd_u64(rbp + 0x288).unwrap_or(0) as usize;    // [0x288]=resolved target
    let arg_src = if ptr_ok(ath) { rd_u64(ath + 0x5a8).unwrap_or(0) } else { 0 };   // 모든 kind arg = [[0x280]+0x5a8]
    let t_arg = if ptr_ok(t_p) { rd_u64(t_p + 0x5a8).unwrap_or(0) } else { 0 };     // T+0x5a8 (kind3 0x441c용)
    // ★dedc0 게이트 해결: 순수 my_dedc0(out+0x40!=0 등) → None(out+0x40==0 && b_logic, vtable timing)이고 gbdedc0면 dedc0(0x20dedc0) 오라클 shadow-call.
    let dedc0_g: Option<bool> = {
        let b = my_dedc0(rd_i64(out + 0x40).unwrap_or(0), rd_u8(out + 0x88), rd_u8(out + 0x8d));
        if b.is_none() && GBDEDC0.load(Ordering::Relaxed) {
            let r8 = rd_u64(rbp + 0x320).unwrap_or(0) as usize;   // [rbp+0x320] 값 = dedc0 3rd arg(asm 0x44c4 mov r8,[rbp+0x320])
            if ptr_ok(r8) {
                let f: Dedc0Fn = core::mem::transmute(mbase + RVA_GB_DEDC0);
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(out, 0, r8))) { Ok(v) => Some(v != 0), _ => None }
            } else { None }
        } else { b }
    };
    let d = RegionD {
        r12, r13,
        l108: rd_i64(rbp + 0x108).unwrap_or(0),
        l158: rd_i64(rbp + 0x158).unwrap_or(0),
        l120: rd_i64(rbp + 0x120).unwrap_or(0),
        l140: rd_i64(rbp + 0x140).unwrap_or(0),
        l270: rd_i64(rbp + 0x270).unwrap_or(0),
        l27e: rd_u8(rbp + 0x27e),
        l27f: rd_u8(rbp + 0x27f),
        out_8b: rd_u8(out + 0x8b),
        l130: rd_i64(rbp + 0x130).unwrap_or(0),
        arg_src, t_arg,
        param2: rd_u64(rbp + 0x238).unwrap_or(0),   // [0x238]=param2 (D.md정정; gb_region_d 미사용=진단용)
        o40: rd_i64(out + 0x40).unwrap_or(0),        // dedc0 타이밍게이트
        o88: rd_u8(out + 0x88),                      // dedc0 facet5 토글
        o8d: rd_u8(out + 0x8d),                      // dedc0 상태바이트 b
        o60: rd_u64(out + 0x60).unwrap_or(0),        // kind4 미기록시 유지되는 arg
        l_e0: rd_i64(rbp + 0xe0).unwrap_or(0),       // [0xe0] 0x44a6 카운트
        l258: rd_u8(rbp + 0x258),                    // [0x258] 0x4724 플래그
        l148: rd_i64(rbp + 0x148).unwrap_or(0),      // [0x148] param2 사본
        l64: rd_u8(rbp + 0x64),                      // [0x64] 0x4628 플래그
        dedc0: dedc0_g,                              // 해결된 게이트(순수 or 오라클)
    };
    let pred = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| gb_region_d(&d, mbase))) {
        Ok(v) => v, _ => { GBRD_PANIC.fetch_add(1, Ordering::Relaxed); return RAX_SENT; }
    };
    // ★성능(2026-06-22): dump(23필드 format! String할당)+dl/sil/entry_vlen은 verify/overwrite(GBRD/GBREPL/GBREPLCHK)서만 GBRD_MAP에 소비됨 → gate 안으로 이동. gbskip 프로덕션(이 셋=0)서 매 region D String 힙할당 제거(gbskip 최대 단일낭비). 비트동일(진단데이터일뿐, 게임출력 무관).
    let entry_vlen = rd_u64(out + 0x78).unwrap_or(0);   // 영역 D 진입시 action Vec len(A/B/C 누적). 리턴서 delta=D push. (cheap; gbskip 경로 미사용이나 1회 read라 유지)
    if GBRD.load(Ordering::Relaxed) || GBREPL.load(Ordering::Relaxed) || GBREPLCHK.load(Ordering::Relaxed) {
        let dl = (d.r12 <= d.r13 && d.l120 <= d.l140) as u8;
        let sil = (((((d.l270 >= 0x32) as u8) | d.l27e) & d.l27f) ^ 1) & 1;
        let dump = format!("r12={} r13={} l108={} l158={} l120={} l140={} lE0={} l258={} l64={} l148={} l270={} o40={} o88={} o8d={} ddc={:?} out8b={} l130={} p2={} dl={} sil={} arg={:#x} o60={:#x} targ={:#x}",
            d.r12, d.r13, d.l108, d.l158, d.l120, d.l140, d.l_e0, d.l258, d.l64, d.l148, d.l270, d.o40, d.o88, d.o8d, dedc0_g, d.out_8b, d.l130, d.param2, dl, sil, d.arg_src, d.o60, d.t_arg);
        if let Ok(mut m) = GBRD_MAP.lock() {
            if let Some(e) = m.iter_mut().find(|x| x.0 == out) { *e = (out, pred, dump, entry_vlen); }
            else if m.len() < 256 { m.push((out, pred, dump, entry_vlen)); }
        }
    }
    GBRD_ARMED.fetch_add(1, Ordering::Relaxed);
    // ★진짜 skip(gbskip): region D는 RNG-free → 게임 region D 건너뛰고 우리출력만 기록 + funnel jump(스텁이 rax=out으로 점프).
    //   push≠0(게임이 action Vec 빌드)/None(dedc0 timing)은 passthrough(게임 region D 실행=Vec/그 결정 보존). 98.5%(no-push)는 진짜 우리계산 대체.
    if GBSKIP.load(Ordering::Relaxed) {
        if let Some((k, a, push)) = pred {
            if push == 0 && readable(out + 0x58, 16) {
                core::ptr::write_unaligned((out + 0x58) as *mut i64, k);
                core::ptr::write_unaligned((out + 0x60) as *mut u64, a);
                GBSKIP_N.fetch_add(1, Ordering::Relaxed);
                return out as i64;   // HANDLED → 스텁이 funnel(0x20e4a1a)로 jump(게임 region D 미실행)
            }
        }
    }
    RAX_SENT   // passthrough (게임 region D 정상 실행)
}
// ★영역 D 한정 대체 100% inline(cfg gbrepl): generic_build 유일 공통출구 에필로그(0x20df5da)서 0x42a3 저장 pred를 out ptr로 조회 →
//   game out+0x58/+0x60 덮어씀. region D 전체 실행(action Vec push 보존) 후 ret 직전 = 모든 영역D 경로 통과(funnel 우회경로 포함) = 100%.
//   리턴-overwrite(RET_STACK캡)와 달리 inline=전건. pred None(dedc0 timing)/non-0x42a3(A/B결정)은 GBRD_MAP miss→게임유지(passthrough).
//   saved: rbp@+0x38(install_detour_d). out=[rbp+0x290]. panic-safe(catch_unwind). gbrepl off시 즉시 return(에필로그=매콜이라 비용가드).
unsafe extern "C" fn gbrd_epilogue_apply(saved: usize, _entry_rsp: usize) {
    let repl = GBREPL.load(Ordering::Relaxed);
    let chk = GBREPLCHK.load(Ordering::Relaxed);
    if !repl && !chk { return; }
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let rbp = match rd_u64(saved + 0x38) { Some(v) => v as usize, None => return };
        if !ptr_ok(rbp) || !readable(rbp + 0x290, 8) { return; }
        let out = rd_u64(rbp + 0x290).unwrap_or(0) as usize;
        if !ptr_ok(out) || !readable(out + 0x58, 16) { return; }
        // 에필로그 = region D 실행 후 = out+0x58/0x60이 게임 최종값. pred와 대조(충실성) 후, repl이면 덮어씀.
        let gk = rd_i64(out + 0x58).unwrap_or(-99);
        let ga = rd_u64(out + 0x60).unwrap_or(0);
        let ent = if let Ok(mut m) = GBRD_MAP.lock() {
            m.iter().position(|x| x.0 == out).map(|p| m.remove(p))
        } else { None };
        if let Some((_, pred, dump, _ev)) = ent {
            if let Some((k, a, _push)) = pred {
                if k == gk && a == ga {
                    GBREPL_MATCH.fetch_add(1, Ordering::Relaxed);
                } else {
                    let n = GBREPL_MISMATCH.fetch_add(1, Ordering::Relaxed) + 1;
                    if n <= 200 {
                        let s = format!("[gbreplchk] my(k={} a={:#x}) vs game(k={} a={:#x}) | {}\n", k, a, gk, ga, dump);
                        if !GBREPLCHK_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("gbreplchk.txt", "=== 대체 충실성 체크: gb_region_d(my) vs game out+0x58/0x60 (전 케이스, 미cap). 덮어쓰기前 대조 ===\n"); }
                        append_named("gbreplchk.txt", &s);
                    }
                }
                if repl {
                    core::ptr::write_unaligned((out + 0x58) as *mut i64, k);
                    core::ptr::write_unaligned((out + 0x60) as *mut u64, a);
                    GBREPL_N.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
    }));
}
// ★generic_build 본체(0x20def90, task#23) 출력 캡처: 진입서 (disc,param2,team) 스냅 + out포인터 저장 → 리턴훅(kind14)서
//   out struct kind@+0x58/arg@+0x60/action Vec 읽기. 매프레임 수백만콜 → unique (disc,param2) 키별 GBB_PER_KEY개만 arm.
//   순수 read(게임호출 제로)라 안전. install_detour saved: rcx@+0x28(out), rdx@+0x20(param2), r9@+0x10(athlete). arg7=S champion@entry_rsp+0x38.
unsafe extern "C" fn genbuild_body_capture(saved: usize, entry_rsp: usize) {
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    GBB_RAW.fetch_add(1, Ordering::Relaxed);
    if !GBBODY.load(Ordering::Relaxed) && !GBRD.load(Ordering::Relaxed) { return; }  // gbrd=verify(kind14). 대체(gbrepl)는 에필로그 hook이 처리하므로 여기 무장 불요.
    if GBB_ARMED.load(Ordering::Relaxed) >= GBB_ARM_MAX { return; }
    let thunk = RET_THUNK.load(Ordering::Relaxed);
    if thunk == 0 { return; }
    let out     = rd_u64(saved + 0x28).unwrap_or(0) as usize;        // rcx = out(0x90B sret)
    let param2  = rd_u64(saved + 0x20).unwrap_or(0);                 // rdx = param2
    let athlete = rd_u64(saved + 0x10).unwrap_or(0) as usize;        // r9  = athlete A
    let s_champ = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize;    // arg7 = S champion
    if !ptr_ok(out) || !ptr_ok(s_champ) || !ptr_ok(athlete) { return; }
    let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    if !ptr_ok(orig_ret) || !readable(entry_rsp, 8) { return; }
    let team = rd_u64(athlete + 0x6a8).unwrap_or(99) as i64;
    // disc-2D = (byte[S+0x3e8]<<16) | word[S+0x3e6]
    let dword = rd_u8(s_champ + 0x3e8) as u32;
    let dlo   = (rd_u8(s_champ + 0x3e6) as u32) | ((rd_u8(s_champ + 0x3e7) as u32) << 8);
    let disc  = (dword << 16) | dlo;
    // unique (disc,param2) 키별 상한 → 분포 골고루
    let key = ((disc as u64) << 20) | (param2 & 0xfffff);
    // ★gbrd 페어링: GBRD on이면 throttle 우회(모든 리턴 무장) → 0x42a3 store를 그 invocation 리턴이 1:1 consume
    //   = out-key 충돌(다른 invocation 출력이 재사용 out슬롯에 lingering) 제거. gbbody-only면 기존 (disc,p2) throttle.
    let ok = if GBRD.load(Ordering::Relaxed) { true } else if let Ok(mut sv) = GBB_SEEN.lock() {
        if let Some(e) = sv.iter_mut().find(|x| x.0 == key) {
            if e.1 >= GBB_PER_KEY { false } else { e.1 += 1; true }
        } else if sv.len() < 8192 { sv.push((key, 1)); true } else { false }
    } else { false };
    if !ok { return; }
    let (sil, mid, hi) = (disc & 0xff, (disc >> 8) & 0xff, (disc >> 16) & 0xff);
    // ★my_generic_build 예측(영역 A early-exit). rh_chain=arg5 value(*=rhd). panic-safe(catch_unwind).
    let gctx = GBCtx {
        param2, athlete, rh_chain: rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize, s_champ,
        o_30: rd_i64(out + 0x30).unwrap_or(0), o_38: rd_i64(out + 0x38).unwrap_or(0), o_60: rd_u64(out + 0x60).unwrap_or(0),
    };
    let (mine, parg): (i64, u64) = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_generic_build(&gctx, 0))) {
        Ok(Some((k, a))) => (k, a), _ => (-99, 0),
    };
    let pre = format!("[gbb #{}] disc={:#x}(lo={} mid={} hi={}) p2={} team={}",
        GBB_ARMED.load(Ordering::Relaxed), disc, sil, mid, hi, param2 as i64, team);
    let pushed = if let Ok(mut st) = RET_STACK.lock() {
        if st.len() < 64 { st.push(RetFrame{ key: entry_rsp, orig_ret, mine, kind: 14, pre, p5: out, p6: parg as usize, disp_pred: 0 }); true } else { false }
    } else { false };
    if pushed {
        core::ptr::write_unaligned(entry_rsp as *mut usize, thunk);
        GBB_ARMED.fetch_add(1, Ordering::Relaxed);
    }
}

// ── facet#1 condgate 재현: subplan별 목표커밋 bool. 리프 vtable=섀도우호출(getter, 부작용無 추정). -99=미재현(poke/gank-else).
type VtPtrFn = unsafe extern "C" fn(usize) -> usize;          // rvt[0x20]timing / rvt[0x168]ctx (1 arg)
type VtPtr2Fn = unsafe extern "C" fn(usize, usize) -> usize;  // rvt[0x128]deref / rvt[0x140]check (2 arg)
unsafe fn vt_slot(rvt: usize, off: usize) -> usize { rd_u64(rvt+off).unwrap_or(0) as usize }   // ★readable VQ→rd_u64(모든 vtable 조회, per-frame 최고빈도). fault시 0=동일
unsafe fn rd_u8g(a: usize) -> i64 { if a < 0x10000 { return -1; } match FAST_READ.load(Ordering::Relaxed) { 2 => lr_u8(a).map(|v| v as i64).unwrap_or(-1), 1 => safe_rd_u8(a).map(|v| v as i64).unwrap_or(-1), _ => if readable(a,1){ std::ptr::read_unaligned(a as *const u8) as i64 } else { -1 } } }   // ★readable VQ→lockless(fault시 -1 유지)
// poke(epic/serpent)의 f3e7!=1 timing 브랜치(LAB_141fbe5e7/f5e1c0). off_a=ctx ==0체크, off_b=rvt[0x140]arg, off_c=timing타겟.
unsafe fn poke_timing_branch(robj: usize, rvt: usize, rh_slot: usize, off_a: usize, off_b: usize, off_c: usize) -> i64 {
    let s = vt_slot(rvt, 0x168); if !ptr_ok(s) { return -99; }
    let g: VtPtrFn = core::mem::transmute(s);
    let ctx = g(robj);
    if !ptr_ok(ctx) || !readable(ctx + off_c, 8) || !readable(ctx + off_a, 8) { return -99; }
    let a0 = std::ptr::read_unaligned((ctx + off_a) as *const u64);
    TD_A0.store(a0 as i64, Ordering::Relaxed);
    let cond = if a0 == 0 { TD_V140.store(i64::MIN, Ordering::Relaxed); true } else {
        let inner = rd_u64(ctx + off_b).unwrap_or(0) as usize;
        let arg = rd_u64(inner).unwrap_or(0) as usize;
        let s2 = vt_slot(rvt, 0x140); if !ptr_ok(s2) { return -99; }
        let h: VtPtr2Fn = core::mem::transmute(s2);
        let v = h(robj, arg);
        TD_V140.store(v as i64, Ordering::Relaxed);
        v == 0
    };
    TD_COND.store(cond as i64, Ordering::Relaxed);
    if !cond {
        TD_TGT.store(-1,Ordering::Relaxed); TD_TIM.store(-1,Ordering::Relaxed); TD_GAP.store(-1,Ordering::Relaxed); TD_THR.store(-1,Ordering::Relaxed); TD_RET.store(0, Ordering::Relaxed);
        return 0;
    }
    let st = vt_slot(rvt, 0x20); if !ptr_ok(st) { return -99; }
    let ft: VtPtrFn = core::mem::transmute(st);
    let timing = ft(robj) as u64;
    let target = std::ptr::read_unaligned((ctx + off_c) as *const u64);
    let gap = if timing <= target { target - timing } else { 0 };
    let a = rd_u64(rh_slot + 8).unwrap_or(0) as usize;
    let b = rd_u64(a + 8).unwrap_or(0) as usize;
    let thr_base = rd_i64(b + 0x12f8).unwrap_or(0);
    let thr15 = thr_base.wrapping_mul(15);
    let ret = if (thr15 as u64) < gap { 1 } else { 0 };
    TD_TGT.store(target as i64, Ordering::Relaxed); TD_TIM.store(timing as i64, Ordering::Relaxed);
    TD_GAP.store(gap as i64, Ordering::Relaxed); TD_THR.store(thr15, Ordering::Relaxed); TD_RET.store(ret, Ordering::Relaxed);
    ret
}
type VtPtr3Fn = unsafe extern "C" fn(usize, usize, usize) -> i64;   // rvt[0x48](robj,team,x) / rvt[0x50](robj,team,9,9)는 4인자
type VtPtr4Fn = unsafe extern "C" fn(usize, usize, usize, usize) -> i64;  // rvt[0x50](robj,team,9,9)
// poke 후보 유효성(FUN_141d880e0/gather 공용): c48!=0 || (a8!=0 && rvt[0x20]timing <= *(lvar16+0x1e0+a8[0x738]*8)+0x78)
unsafe fn cand_valid(robj: usize, rvt: usize, team: u64, lvar16: usize, cand: usize) -> Option<bool> {
    let uv8 = rd_u64(cand + 0x5a8)? as usize;
    let s48 = vt_slot(rvt, 0x48); if !ptr_ok(s48) { return None; }
    let f48: VtPtr3Fn = core::mem::transmute(s48);
    let c48 = f48(robj, team as usize, uv8);
    if (c48 as u8) != 0 { return Some(true); }   // ★vt48=char(AL)반환 — 상위 garbage 무시(저바이트만). 전체 i64 비교는 과대포함 버그.
    let sa8 = vt_slot(rvt, 0xa8); if !ptr_ok(sa8) { return None; }
    let fa8: VtPtr2Fn = core::mem::transmute(sa8);
    let a8 = fa8(robj, uv8);
    if a8 == 0 { return Some(false); }
    let idx = rd_u32(a8 + 0x738) as usize;
    let lv = rd_u64(lvar16 + 0x1e0 + idx*8)?;
    let st = vt_slot(rvt, 0x20); if !ptr_ok(st) { return None; }
    let ft: VtPtrFn = core::mem::transmute(st);
    let timing = ft(robj) as u64;
    Some(timing <= lv + 0x78)
}
// FUN_141d880e0 재현: obj(에픽/뱀) 근처 HP%>=0x28 & 거리<=180000² 후보 중 cand_valid count.
unsafe fn my_count_near_obj(robj: usize, rvt: usize, rh: usize, team: u64, uv19: u64, lvar16: usize, obj: u64) -> i64 {
    let mut cnt: i64 = 0;
    for k in 0..5usize {
        let cand = rd_u64(rh + k*8 + (uv19 as usize).wrapping_mul(0x28) + 0x1e0).unwrap_or(0) as usize;
        if cand == 0 { continue; }
        let max = rd_u64(cand + 0x610).unwrap_or(0);
        if max == 0 { return -99; }
        let hp = rd_u64(cand + 0x658).unwrap_or(0).wrapping_mul(100) / max;
        if hp < 0x28 { continue; }
        let cx = rd_u64(cand + 0x648).unwrap_or(0);
        let cy = rd_u64(cand + 0x650).unwrap_or(0);
        let dx = if obj >= cx { obj - cx } else { cx - obj };
        let dy = if obj >= cy { obj - cy } else { cy - obj };
        if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) > 180000u64*180000 { continue; }
        match cand_valid(robj, rvt, team, lvar16, cand) { Some(true)=>cnt+=1, Some(false)=>{}, None=>return -99 }
    }
    cnt
}
// poke f3e7==1 헬퍼/gather 브랜치. obj=오브젝트좌표, (ta,tb,tc)=timing브랜치 ctx오프셋.
// ★진단(POKE_DIAG): 결정은 기존 unmasked(f50r!=0) 그대로 유지(=DIFF 재현). f50 풀-i64 vs AL저바이트·분기·cnt·nvalid·nearest 기록.
#[inline]
fn pack_poke_diag(branch: i64, cnt: i64, f50_full_nz: bool, f50_low: u8, nvalid: i64, nearest: Option<u64>) -> i64 {
    let near_some = nearest.is_some() as i64;
    let near_q = nearest.map(|d| ((d >> 8).min(0xFFFF_FFFF)) as i64).unwrap_or(0);
    (branch & 0xf)
        | ((cnt & 0xff) << 4)
        | ((f50_full_nz as i64) << 12)
        | ((f50_low as i64) << 13)
        | ((nvalid & 0x7) << 21)
        | (near_some << 24)
        | (near_q << 25)
}
// 레인셀렉터 FUN_141c50ed0(핫픽스) 충실재현 — cnt==0 & f50_AL==0 경로. 반환=선택 레인코드 or 0xff(없음).
// in: r9=champ([+0x6a8]=team), rh_slot=컨테이너([ ]/[+8]/[+0x10]), lanes=레인코드배열(epic[0,1]/serpent[2,1]).
// 각 레인: count게이트([rh_slot+8]의 +0x28) + score게이트(r12<3 ∥ [blk+0x10]<2000 ∥ [blk+0x20]<2) 통과시
// score=[rcx_arr+off]*10000+[blk+0x10]+(i32)[blk+0x20]*1000 → 최소score 레인 선택(al=0xff 시작).
unsafe fn poke_lane_sel(r9: usize, rh_slot: usize, lanes: &[u8]) -> u8 {
    if lanes.is_empty() { return 0xff; }
    let team = rd_u64(r9 + 0x6a8).unwrap_or(99);
    if team > 1 { return 0xff; }                                  // team>=2 = 게임 패닉경로, 가드
    let r10 = rd_u64(rh_slot + 8).unwrap_or(0) as usize;
    let base = rd_u64(rh_slot + 0x10).unwrap_or(0) as usize;
    let arr = rd_u64(rh_slot).unwrap_or(0) as usize;
    let rcx_arr = arr + (team as usize) * 8;
    let cnt_b = rd_u8(r10 + 0x28);                               // u8 카운트(unreadable시 0)
    let blkbase = base + (team as usize) * 0x228;
    let mut al: i32 = 0xff;
    let mut best: i64 = 0;
    for &lc in lanes {
        let (blk, off): (usize, usize);
        if lc == 0 {
            blk = blkbase; off = 0x2170;
            if cnt_b.wrapping_sub(1) < 3 { continue; }            // 0x50fc1: (cnt-1)<3 → skip
        } else if lc == 2 {
            blk = blkbase + 0x50; off = 0x2190;                  // count게이트 없음
        } else {
            blk = blkbase + 0x28; off = 0x2180;
            if cnt_b.wrapping_sub(1) < 2 { continue; }            // 0x50fa3: (cnt-1)<2 → skip
        }
        let r12 = rd_i64(rcx_arr + off).unwrap_or(0);
        let r15v = rd_i64(blk + 0x10).unwrap_or(0);
        let b20 = rd_i32(blk + 0x20).unwrap_or(0) as i64;
        // score게이트: (r12<3 unsigned) ∥ (r15v<2000 signed) ∥ (b20<2 signed) 이면 처리, 아니면 skip
        if !((r12 as u64) < 3 || r15v < 0x7d0 || b20 < 2) { continue; }
        let score = r12.wrapping_mul(10000).wrapping_add(r15v).wrapping_add(b20.wrapping_mul(1000));
        if al == 0xff || score < best { al = lc as i32; best = score; }   // 최소 score 선택
    }
    al as u8
}
unsafe fn my_poke_helper(robj: usize, rvt: usize, rh_slot: usize, r9: usize, obj: u64, ta: usize, tb: usize, tc: usize, lanes: &[u8], f50a: usize) -> i64 {
    let team = rd_u64(r9 + 0x6a8).unwrap_or(0);
    if team > 1 { return -99; }
    let uv19 = 1u64.wrapping_sub(team);
    let rh = rd_u64(rh_slot).unwrap_or(0) as usize;
    let p5_2 = rd_u64(rh_slot + 0x10).unwrap_or(0) as usize;
    let lvar16 = (uv19 as usize).wrapping_mul(0x228).wrapping_add(p5_2);
    let cnt = my_count_near_obj(robj, rvt, rh, team, uv19, lvar16, obj);
    if cnt < 0 { POKE_DIAG.store(7, Ordering::Relaxed); return -99; }
    let s50 = vt_slot(rvt, 0x50); if !ptr_ok(s50) { POKE_DIAG.store(8, Ordering::Relaxed); return -99; }
    let f50: VtPtr4Fn = core::mem::transmute(s50);
    let f50r = f50(robj, team as usize, f50a, f50a);   // ★f50 인자 caller별로 다름: epic=9, serpent=0x15 (asm 확인). 틀린값 쓰면 f50 반환 갈려 timing/nearest 오선택 → DIFF.
    let f50_full_nz = f50r != 0;        // cnt==0 경로(branch A): 게임은 AL≠0→1, AL==0→레인셀렉터. 레인셀렉터 미구현이라 full-i64로 1 반환(관측 752/752 OK, 충실재현 TODO).
    let f50_al_nz = (f50r as u8) != 0;  // ★cnt>0 경로(0x912dd test al,al): 게임은 AL(char)만 본다. c48과 동일 char-mask. ← serpent branch E DIFF 픽스.
    let f50_low = f50r as u8;
    let _ = f50_full_nz;
    if cnt == 0 {
        // 0x91395: f50_AL!=0 → 1
        if f50_al_nz { POKE_DIAG.store(pack_poke_diag(1, cnt, f50_al_nz, f50_low, 0, None), Ordering::Relaxed); return 1; }
        // 0x913b1: 레인셀렉터 → !=0xff면 1
        let lane = poke_lane_sel(r9, rh_slot, lanes);
        if lane != 0xff { POKE_DIAG.store(pack_poke_diag(0xA, cnt, f50_al_nz, lane, 0, None), Ordering::Relaxed); return 1; }
        // lane==0xff → 0x912bc loop → f50_AL==0이라 즉시 timing(0x912dd je)
        POKE_DIAG.store(pack_poke_diag(0xB, cnt, f50_al_nz, lane, 0, None), Ordering::Relaxed);
        return poke_timing_branch(robj, rvt, rh_slot, ta, tb, tc);
    }
    // gather (LAB): f50_AL==0 → timing (★AL 마스크 — 기존 full-i64 비교가 serpent branch E 오발화의 원인)
    if !f50_al_nz {
        POKE_DIAG.store(pack_poke_diag(3, cnt, f50_al_nz, f50_low, 0, None), Ordering::Relaxed);
        return poke_timing_branch(robj, rvt, rh_slot, ta, tb, tc);
    }
    // 최근접 유효후보까지 거리; >150000²면 true, else timing. 유효후보無면 true.
    let mut nearest: Option<u64> = None;
    let mut nvalid = 0i64;
    for k in 0..5usize {
        let cand = rd_u64(rh + k*8 + (uv19 as usize).wrapping_mul(0x28) + 0x1e0).unwrap_or(0) as usize;
        if cand == 0 { continue; }
        match cand_valid(robj, rvt, team, lvar16, cand) {
            Some(true) => {
                nvalid += 1;
                let cx = rd_u64(cand + 0x648).unwrap_or(0);
                let cy = rd_u64(cand + 0x650).unwrap_or(0);
                let dx = if obj >= cx { obj - cx } else { cx - obj };
                let dy = if obj >= cy { obj - cy } else { cy - obj };
                let d = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
                if nearest.map_or(true, |n| d < n) { nearest = Some(d); }
            }
            Some(false) => {}
            None => { POKE_DIAG.store(9, Ordering::Relaxed); return -99; }
        }
    }
    match nearest {
        None => { POKE_DIAG.store(pack_poke_diag(4, cnt, f50_full_nz, f50_low, nvalid, None), Ordering::Relaxed); 1 }   // 유효후보 없음 → true
        Some(d) => if d > 22500000000 {
                POKE_DIAG.store(pack_poke_diag(5, cnt, f50_full_nz, f50_low, nvalid, Some(d)), Ordering::Relaxed); 1
            } else {
                POKE_DIAG.store(pack_poke_diag(6, cnt, f50_full_nz, f50_low, nvalid, Some(d)), Ordering::Relaxed);
                poke_timing_branch(robj, rvt, rh_slot, ta, tb, tc)
            },
    }
}
unsafe fn my_condgate(ctx: usize, r9: usize, rh_slot: usize, r11: usize) -> i64 {
    let _pg = perf_guard(0);
    if !ptr_ok(ctx) { return -99; }
    let disc = match rd_u64(ctx) { Some(v) => v, None => return -99 };   // ★성능(B-2): readable(ctx,8)+raw → rd_u64(VEH, fast_read=2 VQ0). 불가독시 None→-99 = 가드와 비트동일. 매 condgate 호출=최고핫.
    match disc {
        2 => return 1,                                        // ForcePassive = true
        3 | 4 | 13 | 14 => return 0,                          // Passive/Nexus = false
        0 | 1 | 6 => return if rd_i32(ctx + 0x58).unwrap_or(-1) == 7 { 1 } else { 0 },  // data-var (disc<2→idx4 catch도 동일핸들러)
        _ => {}
    }
    let p = rd_u64(rh_slot).unwrap_or(0) as usize;            // rh = *r10
    if !ptr_ok(p) { return -99; }
    let robj = rd_u64(p).unwrap_or(0) as usize;
    let rvt  = rd_u64(p + 8).unwrap_or(0) as usize;
    if !ptr_ok(robj) || !ptr_ok(rvt) { return -99; }
    match disc {
        5 => {  // ActiveRecall: HP풀? rvt[0x128](robj,[r9+0x6a0])=ent; ent[0x658]curHP >= ent[0x610]maxHP
            if !ptr_ok(r9) { return -99; }
            let arg = rd_u64(r9 + 0x6a0).unwrap_or(0) as usize;
            let s = vt_slot(rvt, 0x128); if !ptr_ok(s) { return -99; }
            let f: VtPtr2Fn = core::mem::transmute(s);
            let ent = f(robj, arg);
            if !ptr_ok(ent) { return -99; }   // ★성능(B-2): readable×2 제거, 아래 rd_u64이 정확히 같은 주소 fault-safe(None→-99=동일).
            let cur = match rd_u64(ent + 0x658) { Some(v) => v, None => return -99 };
            let max = match rd_u64(ent + 0x610) { Some(v) => v, None => return -99 };
            if cur >= max { 1 } else { 0 }
        }
        8 => {  // Cover: rvt[0x20](robj)timing >= ctx[0x20] (unsigned setae)
            let s = vt_slot(rvt, 0x20); if !ptr_ok(s) { return -99; }
            let f: VtPtrFn = core::mem::transmute(s);
            let t = f(robj) as u64;
            if t >= rd_u64(ctx + 0x20).unwrap_or(u64::MAX) { 1 } else { 0 }
        }
        7 => {  // LineGanker: timing >= ctx[0x28] → 1; else gank-position(idx2 분기, 0x1be1380 재현)
            let s = vt_slot(rvt, 0x20); if !ptr_ok(s) { return -99; }
            let f: VtPtrFn = core::mem::transmute(s);
            let t = f(robj) as u64;
            if t >= rd_u64(ctx + 0x28).unwrap_or(u64::MAX) { return 1; }
            // else(t<ctx[0x28]): idx2=(ctx[0x31]>=6)?ctx[0x31]-6:3. idx2==0→vt20>=ctx[0x20]?1:0 / idx2==2→1 / else→0
            let v31 = rd_u8(ctx + 0x31);
            let idx2: u8 = if v31 >= 6 { v31.wrapping_sub(6) } else { 3 };
            if idx2 == 0 {
                let t2 = f(robj) as u64;
                if t2 >= rd_u64(ctx + 0x20).unwrap_or(u64::MAX) { 1 } else { 0 }
            } else if idx2 == 2 { 1 } else { 0 }
        }
        10 => {  // EpicBattle: ctx168[0x190]==0→1; else rvt[0x140](robj,[ctx168[0x188]])==0
            let s = vt_slot(rvt, 0x168); if !ptr_ok(s) { return -99; }
            let g: VtPtrFn = core::mem::transmute(s);
            let gc = g(robj);
            if !ptr_ok(gc) || !readable(gc + 0x198, 8) { return -99; }
            if std::ptr::read_unaligned((gc + 0x190) as *const u64) == 0 { return 1; }
            let inner = rd_u64(gc + 0x188).unwrap_or(0) as usize;
            let arg = rd_u64(inner).unwrap_or(0) as usize;
            let s2 = vt_slot(rvt, 0x140); if !ptr_ok(s2) { return -99; }
            let h: VtPtr2Fn = core::mem::transmute(s2);
            if h(robj, arg) == 0 { 1 } else { 0 }
        }
        12 => {  // SerpenBattle: ctx168[0x1c0]==0→1; else rvt[0x140](robj,[ctx168[0x1b8]])==0
            let s = vt_slot(rvt, 0x168); if !ptr_ok(s) { return -99; }
            let g: VtPtrFn = core::mem::transmute(s);
            let gc = g(robj);
            if !ptr_ok(gc) || !readable(gc + 0x1c8, 8) { return -99; }
            if std::ptr::read_unaligned((gc + 0x1c0) as *const u64) == 0 { return 1; }
            let inner = rd_u64(gc + 0x1b8).unwrap_or(0) as usize;
            let arg = rd_u64(inner).unwrap_or(0) as usize;
            let s2 = vt_slot(rvt, 0x140); if !ptr_ok(s2) { return -99; }
            let h: VtPtr2Fn = core::mem::transmute(s2);
            if h(robj, arg) == 0 { 1 } else { 0 }
        }
        9 => {  // EpicHuntPoke (FUN_141fbe220): f3e6!=0→true; f3e7==1→helper/gather; else timing 브랜치
            if !ptr_ok(r11) { return -99; }
            if rd_u8g(r11 + 0x3e6) != 0 { return 1; }
            if rd_u8g(r11 + 0x3e7) == 1 { my_poke_helper(robj, rvt, rh_slot, r9, 0x46500, 0x190, 0x188, 0x198, &[0, 1], 9) }
            else { poke_timing_branch(robj, rvt, rh_slot, 0x190, 0x188, 0x198) }
        }
        11 => { // SerpenHuntPoke (FUN_141f5de90): f3e6!=1→true; f3e7==1→helper/gather; else timing 브랜치(serpent ctx오프셋)
            if !ptr_ok(r11) { return -99; }
            if rd_u8g(r11 + 0x3e6) != 1 { return 1; }
            if rd_u8g(r11 + 0x3e7) == 1 { my_poke_helper(robj, rvt, rh_slot, r9, 0xa4100, 0x1c0, 0x1b8, 0x1c8, &[2, 1], 0x15) }
            else { poke_timing_branch(robj, rvt, rh_slot, 0x1c0, 0x1b8, 0x1c8) }
        }
        _ => -99,
    }
}

// ★facet#1 condgate 캡처: 진입시 my_condgate 계산 → 리턴훅 kind:6서 게임 al(retval&0xff)과 대조.
unsafe extern "C" fn condgate_capture(saved: usize, entry_rsp: usize) -> i64 {
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return RAX_SENT; }
    // ★완전대체(cond_repl): RNG-free judge → sync 불필요. my_condgate(≠-99 확신케이스)로 게임출력 대체(원본 skip).
    //   -99(poke/gank 미재현)는 passthrough(게임 원본). my=al값(0..255) → rax 저바이트=al → 게임이 우리 커밋값 사용.
    if COND_REPL.load(Ordering::Relaxed) {
        let ctx = rd_u64(saved + 0x28).unwrap_or(0) as usize;
        if ptr_ok(ctx) && readable(ctx, 8) {
            let r9 = rd_u64(saved + 0x10).unwrap_or(0) as usize;
            let rh_slot = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;
            let r11c = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize;
            TD_RET.store(i64::MIN, Ordering::Relaxed);
            let my = my_condgate(ctx, r9, rh_slot, r11c);
            if my != -99 {
                let n = COND_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                if n == 1 || n % 300 == 0 {
                    let disc = rd_u64(ctx).unwrap_or(0);
                    let pass = COND_REPL_PASS.load(Ordering::Relaxed);
                    if !COND_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("condcmp.txt", "=== facet#1 condgate ④ 완전대체(cond_repl=1) ===\n"); }
                    append_named("condcmp.txt", &format!("[cond REPL #{}] disc={} my={} (대체) | passthrough누적={}\n", n, disc, my & 0xff, pass));
                }
                return my & 0xff;   // HANDLED: al=my → 원본 skip(게임이 우리 결정 커밋)
            } else {
                // ★passthrough(my=-99=방어가드/dead-path): 게임원본 실행. disc 분포 기록 → 100%여부 측정.
                let pn = COND_REPL_PASS.fetch_add(1, Ordering::Relaxed) + 1;
                let d = (rd_u64(ctx).unwrap_or(99) as usize).min(15);
                COND_PASS_DISC[d].fetch_add(1, Ordering::Relaxed);
                if pn <= 20 || pn % 500 == 0 {
                    if !COND_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("condcmp.txt", "=== facet#1 condgate ④ 완전대체(cond_repl=1) ===\n"); }
                    append_named("condcmp.txt", &format!("[cond PASSTHROUGH #{}] disc={} (my=-99 → 게임원본)\n", pn, rd_u64(ctx).unwrap_or(99)));
                }
            }
        }
    }
    if !CONDCAP.load(Ordering::Relaxed) { return RAX_SENT; }
    // ★새 sim 시작 감지(메뉴 갭 후 첫 AI프레임) → COND 카운터만 리셋(per-replay fresh 캡). 파일은 COND_FILE_INIT 유지로 누적. 프레임갭 휴리스틱(IN_MENU는 sim중 토글돼 신뢰불가).
    let cur_frame = READY_TICKS.load(Ordering::Relaxed);
    let prev_frame = LAST_AI_FRAME.swap(cur_frame, Ordering::Relaxed);
    if REPLAY_RESET.load(Ordering::Relaxed) && cur_frame.wrapping_sub(prev_frame) > 60 {
        COND_ARMED.store(0, Ordering::Relaxed); COND_OK.store(0, Ordering::Relaxed);
        COND_DIFF.store(0, Ordering::Relaxed); COND_PEND.store(0, Ordering::Relaxed);
        for i in 0..16 { COND_SUB_ARMED[i].store(0, Ordering::Relaxed); }
    }
    if COND_ARMED.load(Ordering::Relaxed) >= COND_ARM_MAX { return RAX_SENT; }
    let thunk = RET_THUNK.load(Ordering::Relaxed);
    if thunk == 0 { return RAX_SENT; }
    let ctx = rd_u64(saved + 0x28).unwrap_or(0) as usize;        // rcx = param_1(subplan ctx)
    let r9  = rd_u64(saved + 0x10).unwrap_or(0) as usize;        // r9
    let rh_slot = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize; // [rsp+0x80] = r10 stack arg
    if !ptr_ok(ctx) || !readable(ctx, 8) { return RAX_SENT; }
    let disc = std::ptr::read_unaligned(ctx as *const u64);
    // subplan별 캡: 흔한 disc가 다 채워도 희귀 핸들러 잡히게
    let di = (disc as usize).min(15);
    if COND_SUB_ARMED[di].load(Ordering::Relaxed) >= COND_SUB_CAP { return RAX_SENT; }
    let r11c = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize;   // param_6 = champion (poke 플래그)
    TD_RET.store(i64::MIN, Ordering::Relaxed);   // 스테일방지: poke_timing_branch 호출됐을때만 TD 디코드
    let my = my_condgate(ctx, r9, rh_slot, r11c);
    let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    if !ptr_ok(orig_ret) || !readable(entry_rsp, 8) { return RAX_SENT; }
    let mut pre = format!("[cond #{}] seed=0x{:x} subplan={} my={}", COND_ARMED.load(Ordering::Relaxed), CUR_SEED.load(Ordering::Relaxed), disc, my);
    // ★poke(9/11) 브랜치 진단: FUN_141fbe220/f5de90의 분기키 param_6[0x3e6](early-true)/[0x3e7](active)/param_2(<0x18?)
    if disc == 9 || disc == 11 {
        let p2 = rd_i64(saved + 0x20).unwrap_or(-1);          // rdx = param_2
        let r11 = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize; // param_6 = champion
        let f3e6 = if readable(r11 + 0x3e6, 1) { std::ptr::read_unaligned((r11 + 0x3e6) as *const u8) as i64 } else { -1 };
        let f3e7 = if readable(r11 + 0x3e7, 1) { std::ptr::read_unaligned((r11 + 0x3e7) as *const u8) as i64 } else { -1 };
        pre.push_str(&format!(" POKE[p2={} f3e6={} f3e7={}]", p2, f3e6, f3e7));
        // ★f3e7==1 → my_poke_helper가 방금 POKE_DIAG 채움. 분기/f50full vs f50low(=AL저바이트) 디코드.
        if f3e7 == 1 {
            let pd = POKE_DIAG.load(Ordering::Relaxed);
            if pd >= 0 {
                let br = pd & 0xf;
                let cnt = ((pd >> 4) & 0xff) as i8 as i64;
                let ffull = (pd >> 12) & 1;
                let flow = (pd >> 13) & 0xff;
                let nval = (pd >> 21) & 0x7;
                let nsome = (pd >> 24) & 1;
                let near: i64 = if nsome == 1 { ((pd >> 25) & 0xFFFF_FFFF) << 8 } else { -1 };
                let brc = match br { 1=>"A:cnt0&f50AL→1", 2=>"B:cnt0&!f50→pend", 3=>"C:!f50AL→timing", 4=>"D:nearNone→1", 5=>"E:near>thr→1", 6=>"F:near≤thr→timing", 7=>"cnt<0", 8=>"s50fail", 9=>"candNone", 0xa=>"laneSel→1", 0xb=>"lane0xff→timing", _=>"?" };
                pre.push_str(&format!(" PD[{} cnt={} f50full={} f50low={} nval={} near={}]", brc, cnt, ffull, flow, nval, near));
            }
        }
        // ★poke_timing_branch 호출됐으면(TD_RET!=sentinel) 내부값 디코드 — serpent timing return-1 갭 진단
        if TD_RET.load(Ordering::Relaxed) != i64::MIN {
            pre.push_str(&format!(" TD[cond={} a0={} v140={} tgt={} tim={} gap={} thr15={} ret={}]",
                TD_COND.load(Ordering::Relaxed), TD_A0.load(Ordering::Relaxed), TD_V140.load(Ordering::Relaxed),
                TD_TGT.load(Ordering::Relaxed), TD_TIM.load(Ordering::Relaxed),
                TD_GAP.load(Ordering::Relaxed), TD_THR.load(Ordering::Relaxed), TD_RET.load(Ordering::Relaxed)));
        }
    }
    let pushed = if let Ok(mut st) = RET_STACK.lock() {
        if st.len() < 64 { st.push(RetFrame{ key: entry_rsp, orig_ret, mine: my, kind: 6, pre, p5: disc as usize, p6: 0, disp_pred: -99 }); true } else { false }
    } else { false };
    if pushed {
        core::ptr::write_unaligned(entry_rsp as *mut usize, thunk);
        // ★누수 검출: 새 condgate 진입인데 COND_INSCOPE가 이미 true면 이전 윈도우가 안 닫힘(kind-6 미발화)=누수.
        //   누수 윈도우의 draw는 옆 judge 것이 오귀속된 것 → condgate RNG측정 신뢰불가 신호.
        if COND_INSCOPE.swap(false, Ordering::Relaxed) { COND_LEAK.fetch_add(1, Ordering::Relaxed); }
        COND_IS_DRAWS.store(0, Ordering::Relaxed);   // ★in-scope RNG draw 측정 시작(cond_repl 안전 재확인)
        COND_IS_DEF.store(0, Ordering::Relaxed); COND_IS_E88.store(0, Ordering::Relaxed); COND_IS_E9.store(0, Ordering::Relaxed);
        COND_CUR_DISC.store(disc as i64, Ordering::Relaxed);
        COND_INSCOPE.store(true, Ordering::Relaxed);
        COND_ARMED.fetch_add(1, Ordering::Relaxed);
        COND_SUB_ARMED[di].fetch_add(1, Ordering::Relaxed);
    }
    RAX_SENT   // passthrough (원본 condgate 실행 → 리턴훅 검증)
}

// ── DefenseNexus(disc=14) movepri judge = FUN_142068670 충실 재현. 출력 {7, 18}. ──
// vt+0x140 라우트노드 핸들→엔티티 리졸버(섀도우-CALL 안전, dd7700과 동일 패턴).
#[inline] unsafe fn def_resolve(sim: usize, vobj: usize, handle: u64) -> usize {
    let res = vt_slot(vobj, 0x140);
    if !ptr_ok(res) { return 0; }
    let rf: G2 = core::mem::transmute(res);
    rf(sim, handle as usize) as usize
}
// 상태 시퀀스 워크(DAT_14357be08/be38). vb=*(u8)(vobj+0x28). bVar19/20/21 = 3 웨이포인트 nexus근접.
// 각 워크 = seq에 대한 AND-reduce(브레이크조건=cond의 부정이라 동치). 반환 = 최종 bVar20.
unsafe fn my_def_state_walk(vb: usize, bv19: bool, bv20: bool, bv21: bool) -> bool {
    let seq: &[u8] = match vb {
        0 | 4 | 5 => &[0u8, 1, 2],
        1 | 2     => &[2u8],
        3         => &[1u8, 2],
        _         => return false,   // 범위밖(게임은 게이트로 미발생) — 보수적 false
    };
    let all = |f: &dyn Fn(u8) -> bool| seq.iter().all(|&x| f(x));
    if bv21 {
        if bv20 {
            if !bv19 { all(&|x| x != 0) } else { true }   // bv21&bv20&bv19 → 워크없이 true
        } else if bv19 {
            all(&|x| x != 1)
        } else {
            all(&|x| x == 2)
        }
    } else if bv20 {
        if bv19 { all(&|x| x != 2) } else { all(&|x| x == 1) }
    } else if bv19 {
        all(&|x| x == 0)
    } else {
        false
    }
}
// FUN_142093b80 predicate 재현(read-only, nonzero→true). 부수효과(cursor write) 생략.
// Case A: OTHER측 후보 중 vt0x48/0xa8 필터통과 & nexus<240000. Case B: de0860 머지리스트에 +0x68/0x88/0x90 매칭.
unsafe fn my_def_093b80(p5: usize, p6: usize) -> bool {
    let side = rd_i64(p5 + 0x6a8).unwrap_or(-1);
    if side != 0 && side != 1 { return false; }
    let s = side as usize; let other = 1 - s;
    let l80 = rd_u64(p6).unwrap_or(0) as usize;
    if !ptr_ok(l80) { return false; }
    let nexus = rd_u64(l80 + (s + 0x2e) * 8).unwrap_or(0) as usize;
    if nexus == 0 || !readable(nexus + 0x650, 8) { return false; }
    let sim = rd_u64(l80).unwrap_or(0) as usize;
    let vobj = rd_u64(l80 + 8).unwrap_or(0) as usize;
    let geom = rd_u64(p6 + 0x10).unwrap_or(0) as usize;
    if !ptr_ok(sim) || !ptr_ok(vobj) || !ptr_ok(geom) { return false; }
    let nid = rd_u64(nexus + 0x5a8).unwrap_or(0);
    let (nx, ny) = (rd_u64(nexus + 0x648).unwrap_or(0), rd_u64(nexus + 0x650).unwrap_or(0));
    // Case A
    for k in 0..5usize {
        let c = rd_u64(l80 + 0x1e0 + other * 0x28 + k * 8).unwrap_or(0) as usize;
        if c == 0 || !readable(c + 0x650, 8) { continue; }
        let id = rd_u64(c + 0x5a8).unwrap_or(0);
        let ha8 = dd7_slot_a8(sim, id);
        let mut pass = dd7_slot48(sim, s, id);   // cVar12 != 0
        if !pass && ha8 != 0 {
            let lane = rd_i64(other * 0x228 + geom + 0x1e0 + (rd_i32(ha8 + 0x738).unwrap_or(0) as usize) * 8).unwrap_or(0);
            if dd7_slot20(sim) <= lane + tune("dn_lane_margin", 0x78) { pass = true; }   // ★튜닝: 레인 진척 마진(+120)
        }
        if pass {
            let (cx, cy) = (rd_u64(c + 0x648).unwrap_or(0), rd_u64(c + 0x650).unwrap_or(0));
            if sqd(cx, cy, nx, ny) < tune("dn_pred_dist", 0xd693a4001) as u64 { return true; }   // ★튜닝: 술어 넥서스 근접 거리²(240000²)
        }
    }
    // Case B: de0860 머지(3 배열, OTHER측 geom 0x20스트라이드)
    let off = other * 0x20;
    for &(po, co) in &[(0x10usize, 0x28usize), (0x50, 0x68), (0x90, 0xa8)] {
        let base = rd_u64(l80 + po + off).unwrap_or(0) as usize;
        let cnt = rd_u64(l80 + co + off).unwrap_or(0);
        if base == 0 || cnt == 0 || cnt > 4096 { continue; }
        for j in 0..cnt as usize {
            let ent = rd_u64(base + j * 8).unwrap_or(0) as usize;
            if ent == 0 { continue; }
            if rd_i32(ent + 0x68).unwrap_or(0) == 1 && rd_i32(ent + 0x88).unwrap_or(0) == 1
               && rd_u64(ent + 0x90).unwrap_or(0) == nid {
                return true;
            }
        }
    }
    false
}
// FUN_142068670 본체. p3=count gate(=r8, dd7서 항상 0x27), p5=lane ctx, p6=geom handle.
unsafe fn my_defense_nexus(p3: u64, p5: usize, p6: usize) -> i64 {
    let l80 = rd_u64(p6).unwrap_or(0) as usize;
    if !ptr_ok(l80) { return -99; }
    let sim = rd_u64(l80).unwrap_or(0) as usize;
    let vobj = rd_u64(l80 + 8).unwrap_or(0) as usize;
    let geom = rd_u64(p6 + 0x10).unwrap_or(0) as usize;
    if !ptr_ok(sim) || !ptr_ok(vobj) || !ptr_ok(geom) { return -99; }
    let side = rd_i64(p5 + 0x6a8).unwrap_or(-1);
    if side != 0 && side != 1 { return -99; }
    let s = side as usize; let other = 1 - s;
    // self 엔티티 = vt+0x128(sim, self_handle) → 재현 dd7_slot128
    let selfe = dd7_slot128(sim, rd_u64(p5 + 0x6a0).unwrap_or(0));
    if !ptr_ok(selfe) || !readable(selfe + 0x658, 8) || !readable(selfe + 0x650, 8) { return -99; }
    let maxhp = rd_u64(selfe + 0x610).unwrap_or(0);
    if maxhp == 0 { return -99; }
    let curhp = rd_u64(selfe + 0x658).unwrap_or(0);
    let hp_pct = curhp.wrapping_mul(100) / maxhp;
    // home region bool (X상한 uVar16, Y상한 uVar11)
    let x = rd_u64(selfe + 0x648).unwrap_or(0);
    let y = rd_u64(selfe + 0x650).unwrap_or(0);
    let xup = if side == 0 { tune("dn_home_lo", 64000) as u64 } else { tune("dn_home_hi", 960000) as u64 };   // ★튜닝: X 홈경계
    let home = if (x < tune("dn_home_x1", 0xd9c60) as u64 && side != 0) || x > xup {
        false
    } else {
        let yup = if side == 0 { tune("dn_home_hi", 960000) as u64 } else { tune("dn_home_lo", 64000) as u64 };   // ★튜닝: Y 홈경계
        (y >= tune("dn_home_y1", 0xdac00) as u64 || side != 0) && y <= yup   // ★튜닝: 홈판정 Y 안쪽경계
    };
    // nexus(side) — 게임은 null시 panic; 가드
    let nexus = rd_u64(l80 + (s + 0x2e) * 8).unwrap_or(0) as usize;
    if !ptr_ok(nexus) || !readable(nexus + 0x650, 8) { return -99; }
    let (nx, ny) = (rd_u64(nexus + 0x648).unwrap_or(0), rd_u64(nexus + 0x650).unwrap_or(0));
    // OTHER측 geom 블록서 웨이포인트 3개 리졸브 → nexus 120000 근접 bool
    let oblk = geom + other * 0x228;
    let lv12 = if rd_u8(oblk) != 0 { def_resolve(sim, vobj, rd_u64(oblk + 8).unwrap_or(0)) } else { 0 };
    let lv13 = if rd_i32(oblk + 0x28).unwrap_or(0) == 1 { def_resolve(sim, vobj, rd_u64(oblk + 0x30).unwrap_or(0)) } else { 0 };
    let lv10 = if rd_i32(oblk + 0x50).unwrap_or(0) == 1 { def_resolve(sim, vobj, rd_u64(oblk + 0x58).unwrap_or(0)) } else { 0 };
    let near_d = tune("dn_near_dist", 0x35a4e9001) as u64;   // ★튜닝: 넥서스 근접 거리²(120000²)
    let near120 = |e: usize| -> bool {
        e != 0 && readable(e + 0x650, 8)
            && sqd(rd_u64(e + 0x648).unwrap_or(0), rd_u64(e + 0x650).unwrap_or(0), nx, ny) < near_d
    };
    let bv19 = near120(lv12);
    let bv20 = near120(lv13);
    let bv21 = near120(lv10);
    let vb = rd_u8(vobj + 0x28) as usize;
    let bvar20 = my_def_state_walk(vb, bv19, bv20, bv21);
    // OTHER측 후보 vec서 nexus 120000내 후보 존재? (게임: 첫 dist²<=임계서 정지)
    let mut found_near = false;
    for k in 0..5usize {
        let c = rd_u64(l80 + 0x1e0 + other * 0x28 + k * 8).unwrap_or(0) as usize;
        if c == 0 || !readable(c + 0x650, 8) { continue; }
        // 게임: while(14400000000 < dist²) → 정지=dist²<=14400000000 = (dist² < 0x35a4e9001)
        if sqd(rd_u64(c + 0x648).unwrap_or(0), rd_u64(c + 0x650).unwrap_or(0), nx, ny) < near_d {   // ★튜닝: 넥서스 근접 거리²(120000², 위 near_d 공유)
            found_near = true; break;
        }
    }
    let near_gate = bvar20 && found_near;
    // predicate·nexus HP%는 read-only 결정론 → 진단 위해 1회 선계산(출력 동일, c48 short-circuit과 등가).
    let pred = my_def_093b80(p5, p6);
    let nmax = rd_u64(nexus + 0x610).unwrap_or(0);
    let nmax_d = if nmax == 0 { 1 } else { nmax };
    let nhp_pct = rd_u64(nexus + 0x658).unwrap_or(0).wrapping_mul(100) / nmax_d;
    // ★DEF_DIAG: 7-watcher가 읽을 진단 패킹(hp%/home/near/side/pred/nexus_hp%)
    DEF_DIAG.store(
        hp_pct.min(255)
        | ((home as u64) << 8)
        | ((near_gate as u64) << 9)
        | ((s as u64) << 10)
        | ((pred as u64) << 11)
        | (nhp_pct.min(255) << 16),
        Ordering::Relaxed);
    // ── 결정. param_3>=0x27 → c48경로, <0x27 → ca0경로 (firstcond는 결과 무영향: 양분기 동일목적지) ──
    let hp_crit = tune("dn_hp_crit", 0x15);   // ★튜닝: 위급 HP%(<21)
    let hp_low = tune("dn_hp_low", 0x1f);     // ★튜닝: 저 HP%(<31)
    if p3 < tune("dn_count_gate", 0x27) as u64 {   // ★튜닝: 카운트 게이트(<0x27→ca0)
        // ca0
        if near_gate { return if (hp_pct as i64) < hp_crit && !home { 7 } else { 18 }; }
        // d06
        if (hp_pct as i64) < hp_low { return 7; }
        return if curhp < maxhp && home { 7 } else { 18 };
    }
    // c48: nexus HP% <=50 && predicate → 적극수비(18)
    if (nhp_pct as i64) <= tune("dn_nexus_hp", 0x32) && pred { return 18; }   // ★튜닝: 넥서스 HP%(<=50) 적극수비
    // ce4
    if near_gate || pred { return if (hp_pct as i64) < hp_crit && !home { 7 } else { 18 }; }
    // d06
    if (hp_pct as i64) < hp_low { return 7; }
    if curhp < maxhp && home { 7 } else { 18 }
}

// ════ EpicBattle(disc10) FUN_141ea5340 / SerpenBattle(disc12) FUN_14201a3c0 movepri judge 재현. ════
//   디컴+디스어셈(0x141ea5340) 확정. 출력 {7, 0xa(10), failcode}. 두 함수 거의 동일(오프셋만 차이):
//     self = dd7_slot128(sim, p5[0x6a0]) (champion, +0x610 maxhp/+0x658 curhp/+0x648,0x650 pos)
//     obj  = poke_node_resolve(sim, vt, off_a, off_b)  (vt168 node→def_resolve)
//     home = self.pos ∈ side 홈영역(64000/960000/892000/896000 경계)
//     sel  = poke_df0c10_flag(p5,p6)  (df0c10 셀렉터≠0)
//     ret  = obj_full && ((curhp<maxhp && home) || hp%<51 || sel) → 7
//            else 타이밍게이트: [[p6+8][8]+0x12f8]+p7[t_add] > p7[t_cmp] && *p2!=0 → 0xa(+waypoint) else failcode
//   Epic: off_a/b=0x190/0x188, timing t_add/cmp=0x98/0xa0, failcode=0xc(12)
//   Serpen: off_a/b=0x1c0/0x1b8, timing=0xd0/0xd8, failcode=0xf(15)
unsafe fn my_battle_judge(p2: usize, p5: usize, p6: usize, p7: usize,
                          off_a: usize, off_b: usize, t_add: usize, t_cmp: usize, failcode: i64) -> i64 {
    let l80 = rd_u64(p6).unwrap_or(0) as usize;
    if !ptr_ok(l80) { return -99; }
    let sim = rd_u64(l80).unwrap_or(0) as usize;
    let vt  = rd_u64(l80 + 8).unwrap_or(0) as usize;
    if !ptr_ok(sim) || !ptr_ok(vt) { return -99; }
    let selfe = dd7_slot128(sim, rd_u64(p5 + 0x6a0).unwrap_or(0));
    if !ptr_ok(selfe) || !readable(selfe + 0x658, 8) || !readable(selfe + 0x648, 8) { return -99; }
    let maxhp = rd_u64(selfe + 0x610).unwrap_or(0);
    if maxhp == 0 { return -99; }
    let curhp = rd_u64(selfe + 0x658).unwrap_or(0);
    let hp_pct = curhp.wrapping_mul(100) / maxhp;
    let obj = poke_node_resolve(sim, vt, off_a, off_b);
    let obj_full = obj != 0 && readable(obj + 0x658, 8) && readable(obj + 0x610, 8)
        && rd_u64(obj + 0x658).unwrap_or(0) == rd_u64(obj + 0x610).unwrap_or(1);
    // home: self.pos ∈ side 홈영역 (x upper=side0?64000:960000, y upper=side0?960000:64000)
    let side = rd_i64(p5 + 0x6a8).unwrap_or(-1);
    let x = rd_u64(selfe + 0x648).unwrap_or(0);
    let y = rd_u64(selfe + 0x650).unwrap_or(0);
    let xb = if side == 0 { tune("bt_home_lo", 0xfa00) as u64 } else { tune("bt_home_hi", 0xea600) as u64 };   // ★튜닝: X 홈경계
    let yb = if side == 0 { tune("bt_home_hi", 0xea600) as u64 } else { tune("bt_home_lo", 0xfa00) as u64 };   // ★튜닝: Y 홈경계
    let home = if x > xb || (x < tune("bt_home_x1", 0xd9c60) as u64 && side != 0) { false }
               else { (y >= tune("bt_home_y1", 0xdac00) as u64 || side != 0) && y <= yb };   // ★튜닝: 홈판정 안쪽경계
    let sel = poke_df0c10_flag(p5, p6);   // df0c10 셀렉터 != 0
    if obj_full && ((curhp < maxhp && home) || (hp_pct as i64) < tune("bt_hp_retreat", 51) || sel) { return 7; }   // ★튜닝: 귀환 HP%(<51)
    // 타이밍게이트
    let host = rd_u64(p6 + 8).unwrap_or(0) as usize;
    let hostvt = rd_u64(host + 8).unwrap_or(0) as usize;
    let timing = rd_i64(hostvt + 0x12f8).unwrap_or(0).wrapping_add(rd_i64(p7 + t_add).unwrap_or(0));
    if timing > rd_i64(p7 + t_cmp).unwrap_or(0) && ptr_ok(p2) && rd_u8(p2) != 0 { 0xa } else { failcode }
}
#[inline] unsafe fn my_epic_battle(p2: usize, p5: usize, p6: usize, p7: usize) -> i64 {
    my_battle_judge(p2, p5, p6, p7, 0x190, 0x188, 0x98, 0xa0, 0xc)
}
#[inline] unsafe fn my_serpen_battle(p2: usize, p5: usize, p6: usize, p7: usize) -> i64 {
    my_battle_judge(p2, p5, p6, p7, 0x1c0, 0x1b8, 0xd0, 0xd8, 0xf)
}

// facet#4 movepriority 재현(code@출력+0). Stage1=상수+data-var+AttackNexus.
// Stage2b: 3(dd7700)/9(EpicPoke)/10(EpicBattle)/11(SerpenPoke)/12(SerpenBattle)/14(DefNexus) 전부 재현.
// ★disc4 메인경로(좌표게이트+첫 TTD루프) 토글 + 진단카운터. d4ttd=1이면 my_disc4가 TTD경로 사용(기본off=late7 단순화).
static D4_TTD: AtomicBool = AtomicBool::new(false);
static D4_REPL: AtomicBool = AtomicBool::new(true);     // disc4 mp_repl 대체 토글(cfg d4_repl; false=passthrough 격리)
static D4FREEZE: AtomicBool = AtomicBool::new(false);   // my_disc4 단계별 truncate-write 진단(cfg d4freeze → d4last.txt)
static D4_CALLN: AtomicU64 = AtomicU64::new(0);         // my_disc4 호출 카운터
static D4_CN: AtomicU64 = AtomicU64::new(0);            // 현재 호출번호(d4stage 공유)
// ★freeze 진단: 매 단계 d4last.txt에 truncate-write. freeze 직전 마지막 줄 = hang 도달 단계 + 입력. 내부 shadow-call hang이면 그 stage가 마지막.
// ★성능: 매크로로 lazy화 — D4FREEZE off시 인자(format!) 자체를 평가 안 함(이전엔 함수 인자라 매 disc4 호출마다 format! alloc 발생=경기 저하 주범).
macro_rules! d4stage {
    ($e:expr) => { if D4FREEZE.load(Ordering::Relaxed) { write_named("d4last.txt", &format!("call#{} {}\n", D4_CN.load(Ordering::Relaxed), $e)); } };
}
static D4_TTD_PASS: AtomicU64 = AtomicU64::new(0);   // 좌표게이트 통과(첫TTD루프 실행) 횟수
static D4_TTD_C8: AtomicU64 = AtomicU64::new(0);     // TTD>cfg → code8 횟수
static D4_3RD: AtomicU64 = AtomicU64::new(0);        // 2nd ally매치→3rd_dispatch TTD 발화 횟수
static D4_DIAG_N: AtomicU64 = AtomicU64::new(0);     // late7 borderline 진단 카운터
static D4_DIAG: Mutex<String> = Mutex::new(String::new());   // late7 borderline r13b필드 덤프(d4diag.txt)
// ★disc4(0x206e530, PassiveJungle) 출력코드 재현 first-cut(RNG-free, disasm-only). p2=subp, p5=r14(athlete), p6=r15(geom).
//   resolve: vt[0x128](sim=*(*(p6)), handle=*(p5+0x6a0))→target(==0=게임panic→passthrough). code7=target 홈리전&HP-low. else 메인→code8(지배적; code3/late7=미세조정 대상).
unsafe fn my_disc4(subp: usize, p5: usize, p6: usize) -> i64 {
    let _pg = perf_guard(2);
    let _ = subp;
    if D4FREEZE.load(Ordering::Relaxed) { D4_CN.store(D4_CALLN.fetch_add(1, Ordering::Relaxed)+1, Ordering::Relaxed); }
    d4stage!(&format!("ENTER subp={:#x} p5={:#x} p6={:#x}", subp, p5, p6));
    if !ptr_ok(p5) || !ptr_ok(p6) { d4stage!("EXIT -99 badp5p6"); return -99; }   // ★readable VQ제거(p5+0x6a0 unwrap_or)
    let l80 = rd_u64(p6).unwrap_or(0) as usize;
    if !ptr_ok(l80) { return -99; }
    let obj = rd_u64(l80).unwrap_or(0) as usize;
    let vt = rd_u64(l80 + 8).unwrap_or(0) as usize;
    if !ptr_ok(obj) || !ptr_ok(vt) { return -99; }
    let s = vt_slot(vt, 0x128); if !ptr_ok(s) { return -99; }
    let f: VtPtr2Fn = core::mem::transmute(s);
    let handle = rd_u64(p5 + 0x6a0).unwrap_or(0) as usize;
    let target = f(obj, handle);
    if !ptr_ok(target) { d4stage!("EXIT -99 badtarget"); return -99; }   // ★readable VQ제거(target==0=panic가드는 ptr_ok, 좌표/hp rd_u64)
    d4stage!(&format!("target={:#x}", target));
    let team = rd_i64(p5 + 0x6a8).unwrap_or(-1);
    if team != 0 && team != 1 { return -99; }
    // ── code7(early): target 홈리전(x/y) AND hp-low(hp<maxhp) ──
    let x = rd_u64(target + 0x648).unwrap_or(0);
    let y = rd_u64(target + 0x650).unwrap_or(0);
    let xb: u64 = if team == 0 { 0xfa00 } else { 0xea600 };
    let yb: u64 = if team == 0 { 0xea600 } else { 0xfa00 };
    let x_home = x <= xb && (x >= 0xd9c60 || team == 0);
    let y_home = y <= yb && (y >= 0xdac00 || team != 0);
    let hp = rd_u64(target + 0x658).unwrap_or(0);
    let maxhp = rd_u64(target + 0x610).unwrap_or(0);
    if x_home && y_home && hp < maxhp {
        d4stage!("EXIT 7 early-home");
        return 7;
    }
    // ── 메인 경로: 좌표게이트+첫TTD루프+SUBPLAN+2nd/3rd dispatch+late7 (0x206e530 disasm 충실재현). d4ttd=1일때만. ──
    if D4_TTD.load(Ordering::Relaxed) {
        d4stage!(&format!("→main hp={} maxhp={}", hp, maxhp));
        let c = my_disc4_main(subp, p6, obj, vt, target, team, hp, maxhp);
        d4stage!(&format!("EXIT {} main", c));
        return c;
    }
    // ── (D4_TTD off) late-code7 단순화 (좌표게이트/TTD/2nd·3rd 미발화 근사): hp_pct>=21→8 / <21→7. ──
    let hp_pct = if maxhp != 0 { hp.saturating_mul(100) / maxhp } else { hp.saturating_mul(100) / 7 };
    let c = if hp_pct >= 21 { 8 } else { 7 };
    d4stage!(&format!("EXIT {} simple", c));
    c
}
#[inline] fn disc4_late7(hp: u64, maxhp: u64) -> i64 {
    let hp_pct = if maxhp != 0 { hp.saturating_mul(100) / maxhp } else { hp.saturating_mul(100) / 7 };
    if hp_pct >= 21 { 8 } else { 7 }   // thr=21(disc4). r13b 능력게이트 thr=41 케이스=Stage B
}
// disc4 TTD 누적 루프 (vt168 0x180 벡터 순회 → Σ contrib/coef). 첫/3rd dispatch 공용. (FUN_14206e530 0.4.13_5 disasm).
unsafe fn disc4_ttd_acc(obj: usize, vt: usize, target: usize, sim: usize, exe: usize) -> u64 {
    let vec = vt_call1(vt, 0x168, obj);
    if !ptr_ok(vec) { return 0; }
    let cnt = rd_u64(vec + 0x190).unwrap_or(0) as usize;
    let ptr = rd_u64(vec + 0x188).unwrap_or(0) as usize;
    if !ptr_ok(ptr) || cnt > 64 { return 0; }
    let r9 = exe + 0x35e4d00;                               // base getter r9(ATK_VT)
    // ★튜닝 계수는 루프불변 → 루프 밖 1회 조회(핫루프 tune() SipHash×최대320회 제거 = disc4 대폭 가속).
    let t_dmg_scale = tune("d4_dmg_scale", 1000) as u64;
    let t_div_base  = tune("d4_div_base", 100);
    let t_coef_scale = tune("d4_coef_scale", 100);
    let t_coef_min  = tune("d4_coef_min", 4);
    let t_coef_clamp = tune("d4_coef_clamp", 3);
    let mut acc: u64 = 0;
    for i in 0..cnt {
        let handle = rd_u64(ptr + i*8).unwrap_or(0);
        let e = def_resolve(obj, vt, handle);
        if e == 0 || rd_i32(e + 0x4a8).unwrap_or(-1) == -1 { continue; }
        let (pb, mb) = probe_basedmg_r9(e, sim, exe, r9);
        let contrib: u64 = if pb >= 0 && mb >= 0 && (pb | mb) != 0 {
            let dtype = rd_i32(e + 0x4a4).unwrap_or(0) as u32;
            let dmg = my_combat_dmg(e, target, pb, dtype, 0, exe) + my_combat_dmg(e, target, mb, dtype, 1, exe);
            (dmg.max(0) as u64).wrapping_mul(t_dmg_scale)
        } else { 0 };
        let dps = vt560_threat(e);
        let mut div = rd_i32(e + 0x3e4).unwrap_or(0) as i64 + t_div_base;
        if div < 2 { div = 1; }
        let mut coef = dps.wrapping_mul(t_coef_scale) / div;
        if coef < t_coef_min { coef = t_coef_clamp; }
        if coef > 0 { acc = acc.wrapping_add(contrib / coef as u64); }
    }
    acc
}
// disc4 좌표게이트: sel=*(subp+0x48)!=0; sel==0→x=A(0x35ef020)/y=B(0x35eeff0). dx²+dy²<14400000001 → 첫TTD루프 실행(true).
// ★포탑 위협 = self 생존 TTD 가산 (cfg tower_threat>0). 게임 다이브-TTD(disc4_ttd_acc)의 적집합엔 포탑 없음(로스터+교전리스트=챔피언만, 타워RE aff768e + 런타임 TOWSCAN 확정) → 포탑밑 무한생존 평가→다이브사망. 이 항이 막음. RNG무소비=desync無. 기본 tower_threat=0=원본동작.
//   포탑 enumerate: l80(=*p6)+{0x180,0x190,0x1a0,0x1b0,0x1c0,0x1d0}+et*8(고정 6) + l80+0x130+et*0x20 Vec(len@+0x148). type(+0x68)==2=포탑(넥서스 t3 제외). pos=+0x648/+0x650. 직접포인터(TOWSCAN 확인). et=적팀(1-self_team).
static TOWER_HIT_N: AtomicU64 = AtomicU64::new(0);     // 진단: 사거리내 적포탑 발견 호출수
static TOWER_HIT_MAX: AtomicU64 = AtomicU64::new(0);   // 진단: 한번에 본 최대 포탑수
unsafe fn tower_in_range(tw: usize, sx: u64, sy: u64, tr2: u64) -> bool {
    if !ptr_ok(tw) || rd_i32(tw + 0x68).unwrap_or(-1) != 2 { return false; }   // type 2 = 포탑만(넥서스 type3 제외)
    let tx = rd_u64(tw + 0x648).unwrap_or(0); let ty = rd_u64(tw + 0x650).unwrap_or(0);
    let dx = if sx >= tx { sx - tx } else { tx - sx };
    let dy = if sy >= ty { sy - ty } else { ty - sy };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) < tr2
}
// ★성향스탯 보정 (공격성+에고 = 결정론 임계시프트 / 판단력 = 결정론 해시노이즈). STAT_INFLUENCE=0 또는 중립(공격성50·에고50·판단력≥100)이면 (0,0)=비트동일.
//   판단력 노이즈 = (tick>>5, 핸들) splitmix 해시 → ★게임RNG 무소비(draw수 불변=sim 기계적 유효·replay 재현가능). k>0서 출력변화는 의도(성향반영). p5=athlete(★라이너 추가판단 한정 — 정글러는 유저지시로 미반영: +0x218판단력/+0x230공격성/+0x238에고/+0x6a0핸들).
#[inline]
fn stat_hash(a: u64, b: u64) -> u64 {
    let mut x = a.wrapping_mul(0x9e3779b97f4a7c15).wrapping_add(b.wrapping_mul(0xbf58476d1ce4e5b9));
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}
unsafe fn stat_modifiers(p5: usize, sim: usize) -> (i64, i64) {
    let k = STAT_INFLUENCE.load(Ordering::Relaxed);
    if k <= 0 || !ptr_ok(p5) { return (0, 0); }
    let aggr = rd_i64(p5 + 0x230).unwrap_or(50).clamp(0, 100);   // 공격성
    let ego  = rd_i64(p5 + 0x238).unwrap_or(50).clamp(0, 100);   // 에고
    let judg = rd_i64(p5 + 0x218).unwrap_or(100).clamp(0, 200);  // 판단력(effective, 100초과 가능)
    // ★비대칭 가중: 위(공격적,>50)는 절반(100/100=+50=많이안뺌, "절대안뺌"=+100은 150/150 필요=상한초과) / 아래(소심,<50)는 그대로(0/0=−100=호각도뺌). 50/50=0.
    let ca = { let d = aggr - 50; if d > 0 { d / 2 } else { d } };
    let ce = { let d = ego  - 50; if d > 0 { d / 2 } else { d } };
    let stat_adj = (ca + ce) * k / 100;                          // 공격성·에고↑ → +adj → eff임계↓ → 덜 후퇴(다이브)
    let amp = ((100 - judg).max(0)) * k / 100;                   // 판단력↓ → 노이즈 진폭↑. ≥100=0(완벽판단)
    let jnoise = if amp > 0 {
        let tick = dd7_slot20(sim) as u64;
        let handle = rd_u64(p5 + 0x6a0).unwrap_or(0);
        (stat_hash(tick >> 5, handle) % (2 * amp as u64 + 1)) as i64 - amp   // [-amp,+amp] 결정론(~0.5s 코히런트=프레임 깜빡임 방지)
    } else { 0 };
    (stat_adj, jnoise)
}
unsafe fn tower_threat_acc(p6: usize, team: i64, selfe: usize) -> u64 {
    let threat = TOWER_THREAT.load(Ordering::Relaxed);
    if threat <= 0 || team < 0 || team > 1 || !ptr_ok(selfe) { return 0; }
    let l80 = match rd_u64(p6) { Some(v) if ptr_ok(v as usize) => v as usize, _ => return 0 };
    let sx = rd_u64(selfe + 0x648).unwrap_or(0); let sy = rd_u64(selfe + 0x650).unwrap_or(0);
    let et = (1 - team) as usize;   // 적팀
    let trange = TOWER_RANGE.load(Ordering::Relaxed); let tr2 = trange.wrapping_mul(trange);
    let per = tune("tower_dps", 8000).max(0) as u64;   // ★튜닝: 포탑1개당 acc기여(threat=100 기준). self TTD↓ 강도(클수록 포탑밑 다이브 더 자제).
    let mut hits = 0u64;
    for &off in &[0x180usize, 0x190, 0x1a0, 0x1b0, 0x1c0, 0x1d0] {   // 고정 6 포탑슬롯
        if tower_in_range(rd_u64(l80 + off + et*8).unwrap_or(0) as usize, sx, sy, tr2) { hits += 1; }
    }
    let vbase = rd_u64(l80 + 0x130 + et*0x20).unwrap_or(0) as usize;   // 동적 포탑 Vec
    let vlen = rd_u64(l80 + 0x148 + et*0x20).unwrap_or(0);
    if ptr_ok(vbase) && vlen <= 32 {
        for i in 0..vlen as usize { if tower_in_range(rd_u64(vbase + i*8).unwrap_or(0) as usize, sx, sy, tr2) { hits += 1; } }
    }
    if hits > 0 { TOWER_HIT_N.fetch_add(1, Ordering::Relaxed); if hits > TOWER_HIT_MAX.load(Ordering::Relaxed) { TOWER_HIT_MAX.store(hits, Ordering::Relaxed); } }   // 진단
    hits.wrapping_mul(per).wrapping_mul(threat as u64) / 100   // 사거리내 포탑수 × per × threat/100 → acc↑ → ttd↓ → code8 자제. ★정글러는 성향스탯 미반영(유저지시 — 원래 disc4 판단 유지)
}
static LANER_RET_N: AtomicU64 = AtomicU64::new(0);     // 진단: 라이너 후퇴(code7) override 발동
static LANER_RET_TOW: AtomicU64 = AtomicU64::new(0);   // 그중 포탑사유
static LANER_RET_NUM: AtomicU64 = AtomicU64::new(0);   // 그중 단순머릿수(binary margin)사유
static LANER_RET_FRC: AtomicU64 = AtomicU64::new(0);   // ★그중 일반교전 전력(force)사유
static LANER_RET_W: AtomicI64 = AtomicI64::new(-1);    // 진단: 마지막 전력승산 샘플(100=호각)
// ★라이너(dd7700/PassiveLine) 후퇴 판단 — 전력(force)승산 기반(2026-06-23 정식형 DPS×HP). 승산=combat_balance(force_ally×100/force_enemy, force=ΣHP×Σ공격). ①포탑밑 AND tower_threat≥승산 ②일반교전 numbers_threat≥승산(적없으면 승산9999=자동 근접게이트) ③단순머릿수 margin. dd7700=매라이너매프레임=라이너 다이브/불리교전 직접차단(disc4=정글러와 별개). l80=*p6, et=적팀. 기본(전부0)=원본동작.
unsafe fn laner_should_retreat(p6: usize, team: i64, selfe: usize, p5: usize) -> bool {
    if team < 0 || team > 1 || !ptr_ok(selfe) { return false; }
    let l80 = match rd_u64(p6) { Some(v) if ptr_ok(v as usize) => v as usize, _ => return false };
    let sx = rd_u64(selfe + 0x648).unwrap_or(0); let sy = rd_u64(selfe + 0x650).unwrap_or(0);
    let et = (1 - team) as usize;
    let threat = TOWER_THREAT.load(Ordering::Relaxed);
    let nthreat = NUMBERS_THREAT.load(Ordering::Relaxed);
    let margin = NUMBERS_MARGIN.load(Ordering::Relaxed);
    let (w, ally, enemy) = combat_balance(l80, team, sx, sy).unwrap_or((9999, 1, 0));   // 전력승산 + 머릿수(공용)
    if threat > 0 || nthreat > 0 { LANER_RET_W.store(w, Ordering::Relaxed); }   // 진단 샘플
    let (stat_adj, jnoise) = stat_modifiers(p5, rd_u64(l80).unwrap_or(0) as usize);   // ★성향보정: 공격성+에고=임계시프트, 판단력=노이즈. k0/중립=0=현행
    // ① 포탑: self가 적포탑 사거리내 AND tower_threat ≥ 전력승산 → 후퇴. ★threat=100→호각싸움도 수비, 0→미적용.
    if threat > 0 {
        let trange = TOWER_RANGE.load(Ordering::Relaxed); let tr2 = trange.wrapping_mul(trange);
        let mut under = false;
        for &off in &[0x180usize, 0x190, 0x1a0, 0x1b0, 0x1c0, 0x1d0] {
            if tower_in_range(rd_u64(l80 + off + et*8).unwrap_or(0) as usize, sx, sy, tr2) { under = true; break; }
        }
        if !under {
            let vbase = rd_u64(l80 + 0x130 + et*0x20).unwrap_or(0) as usize;
            let vlen = rd_u64(l80 + 0x148 + et*0x20).unwrap_or(0);
            if ptr_ok(vbase) && vlen <= 32 {
                for i in 0..vlen as usize { if tower_in_range(rd_u64(vbase + i*8).unwrap_or(0) as usize, sx, sy, tr2) { under = true; break; } }
            }
        }
        if under && (threat - stat_adj + jnoise) >= w {   // ★포탑밑 + (성향보정)tower_threat가 전력승산 이상 = 불리 → 후퇴
            LANER_RET_N.fetch_add(1, Ordering::Relaxed); LANER_RET_TOW.fetch_add(1, Ordering::Relaxed); return true;
        }
    }
    // ② 일반교전 전력(force): numbers_threat ≥ 전력승산 → 후퇴. 적없으면 w=9999라 미발동(자동 근접게이트). ★강하면 적어도 싸움(force=머릿수+세기 동시반영).
    if nthreat > 0 && (nthreat - stat_adj + jnoise) >= w {   // ★(성향보정)numbers_threat ≥ 전력승산
        LANER_RET_N.fetch_add(1, Ordering::Relaxed); LANER_RET_FRC.fetch_add(1, Ordering::Relaxed); return true;
    }
    // ③ 단순 머릿수(binary, 하위호환): 근처 적챔프 − 아군챔프 ≥ margin
    if margin > 0 && (enemy as i64 - ally as i64) >= margin {
        LANER_RET_N.fetch_add(1, Ordering::Relaxed); LANER_RET_NUM.fetch_add(1, Ordering::Relaxed); return true;
    }
    false
}
// ★disc4 좌표게이트: sel=*(subp+0x48)!=0; sel==0→x=A(0x35ef020)/y=B(0x35eeff0). dx²+dy²<14400000001 → 첫TTD루프 실행(true).
unsafe fn disc4_coord_pass(subp: usize, target: usize, exe: usize) -> bool {
    let disc: usize = 4;
    let sel = rd_u64(subp + 0x48).unwrap_or(0) != 0;
    let tab_a = exe + 0x35ef020; let tab_b = exe + 0x35eeff0;
    let (x_tbl, y_tbl) = if !sel { (tab_a, tab_b) } else { (tab_b, tab_a) };
    let tx = rd_u64(x_tbl + disc*8).unwrap_or(0);
    let ty = rd_u64(y_tbl + disc*8).unwrap_or(0);
    let txv = rd_u64(target + 0x648).unwrap_or(0);
    let tyv = rd_u64(target + 0x650).unwrap_or(0);
    let dx = (if txv >= tx { txv - tx } else { tx - txv }) as u128;
    let dy = (if tyv >= ty { tyv - ty } else { ty - tyv }) as u128;
    dx*dx + dy*dy < tune("d4_coord_dist", 14400000001) as u128   // ★튜닝: 좌표게이트 거리²(갱킹 활동범위, 기본 120000²+1)
}
// disc4 2nd_dispatch ally매치: vt168 0x180 벡터서 핸들 def_resolve→e가 *(e+0x68)==4 && *(e+0x88)==1 && *(e+0x90)==*(target+0x5a8) 인 게 하나라도 있으면 true.
unsafe fn disc4_ally_match(obj: usize, vt: usize, target: usize) -> bool {
    let vec = vt_call1(vt, 0x168, obj);
    if !ptr_ok(vec) { return false; }
    let cnt = rd_u64(vec + 0x190).unwrap_or(0) as usize;
    let ptr = rd_u64(vec + 0x188).unwrap_or(0) as usize;
    if !ptr_ok(ptr) || cnt > 64 { return false; }
    let tkey = rd_i64(target + 0x5a8).unwrap_or(i64::MIN);
    for i in 0..cnt {
        let handle = rd_u64(ptr + i*8).unwrap_or(0);
        let e = def_resolve(obj, vt, handle);
        if e == 0 { continue; }
        if rd_i32(e + 0x68).unwrap_or(0) != 4 { continue; }
        if rd_i32(e + 0x88).unwrap_or(0) != 1 { continue; }
        if rd_i64(e + 0x90).unwrap_or(i64::MIN) != tkey { continue; }
        return true;
    }
    false
}
type Vt40Fn = unsafe extern "C" fn(usize, usize, usize, usize, usize);   // (out_sret, buf, sim, target, atkvt) → void
// disc4 SUBPLAN 능력게이트 → late-code7 thr 결정용 r13b 반환 (4=disc/1=충족 → thr21 / 0 → thr41). (FUN_14206e530 0x206e9fa~).
//   target=self(jungler). r13b=0(thr41) = 능력2 dummy(*(dpi+0x30)==-1) 또는 vt40 out<=0. vt30/vt40 = self 능력reach getter shadow-call(guarded).
unsafe fn disc4_subplan_r13b(target: usize, sim: usize, exe: usize) -> i32 {
    let disc: i32 = 4;
    if rd_i32(target + 0x3d8).unwrap_or(0) > 0 { return disc; }   // 3d8>0 → 2nd_dispatch, r13b=disc
    let atkvt = exe + 0x35e4d00;
    let mh = rd_i64(target + 0x610).unwrap_or(0) - rd_i64(target + 0x658).unwrap_or(0);   // maxhp-hp
    // ── 능력1 (vth=*(target+0x4b8)) ──
    if rd_i32(target + 0x4e0).unwrap_or(-1) != -1 {
        let vth = rd_u64(target + 0x4b8).unwrap_or(0) as usize;
        let buf0 = rd_u64(target + 0x4b0).unwrap_or(0) as usize;
        if ptr_ok(vth) && ptr_ok(buf0) {
            let inner = rd_u64(vth + 0x10).unwrap_or(0) as usize;
            let buf = buf0.wrapping_add(inner.wrapping_sub(1) & !0xf).wrapping_add(0x10);
            let g30 = rd_u64(vth + 0x30).unwrap_or(0) as usize;
            if readable(g30, 4) && readable(buf, 8) {
                let f: Getter4 = core::mem::transmute(g30);
                if mh.min(f(buf, sim, target, atkvt)) != 0 { return disc; }   // 충족 A
                let g40 = rd_u64(vth + 0x40).unwrap_or(0) as usize;
                if readable(g40, 4) {
                    let mut out = [0u64; 32];
                    let f40: Vt40Fn = core::mem::transmute(g40);
                    f40(out.as_mut_ptr() as usize, buf, sim, target, atkvt);
                    let o0 = out[0] as i32;
                    let o40 = std::ptr::read_unaligned((out.as_ptr() as usize + 0x40) as *const i32);
                    if o0 != -1 && o40 > 0 { return disc; }   // 충족 B
                }
            }
        }
    }
    // ── 능력2 (0x206eb13): dpi 선택 → dummy면 r13b=0, else vt30/vt40 ──
    let dpi = if rd_i64(target + 0x5b0).unwrap_or(0) >= 3 { target + 0x4e8 } else { exe + 0x35e5730 };
    if rd_i32(dpi + 0x30).unwrap_or(-1) == -1 { return 0; }   // dummy/플래그 -1 → r13b=0 → thr41
    let dvt = rd_u64(dpi + 8).unwrap_or(0) as usize;
    let dbuf0 = rd_u64(dpi).unwrap_or(0) as usize;
    if ptr_ok(dvt) && ptr_ok(dbuf0) {
        let inner = rd_u64(dvt + 0x10).unwrap_or(0) as usize;
        let dbuf = dbuf0.wrapping_add(inner.wrapping_sub(1) & !0xf).wrapping_add(0x10);
        let g30 = rd_u64(dvt + 0x30).unwrap_or(0) as usize;
        if readable(g30, 4) && readable(dbuf, 8) {
            let f: Getter4 = core::mem::transmute(g30);
            if mh.min(f(dbuf, sim, target, atkvt)) != 0 { return disc; }   // 충족 → r13b=disc
        }
        let g40 = rd_u64(dvt + 0x40).unwrap_or(0) as usize;
        if readable(g40, 4) {
            let mut out = [0u64; 32];
            let f40: Vt40Fn = core::mem::transmute(g40);
            f40(out.as_mut_ptr() as usize, dbuf, sim, target, atkvt);
            let o0 = out[0] as i32;
            let o40 = std::ptr::read_unaligned((out.as_ptr() as usize + 0x40) as *const i32);
            return if o0 != -1 && o40 > 0 { 1 } else { 0 };   // vt40 충족→1(thr21) / 아니면 0(thr41)
        }
    }
    0
}
// disc4 메인경로 완전재현 (FUN_14206e530): 좌표게이트+첫TTD → SUBPLAN→2nd ally매치→3rd TTD → late7. 출력 7/8.
unsafe fn my_disc4_main(subp: usize, p6: usize, obj: usize, vt: usize, target: usize, team: i64, hp: u64, maxhp: u64) -> i64 {
    let exe = exe_base();
    if exe == 0 { return disc4_late7(hp, maxhp); }
    let sim = rd_u64(p6 + 8).unwrap_or(0) as usize;
    let cfg_root = rd_u64(p6 + 8).unwrap_or(0) as usize;
    let cfg_b = rd_u64(cfg_root + 8).unwrap_or(0) as usize;
    let cfg_thr = (rd_u64(cfg_b + 0x12f8).unwrap_or(0) as i128 * TUNE_TTD_MULT.load(Ordering::Relaxed) as i128 / 100) as u64;   // ★튜닝: disc4 TTD 임계 배율(t_ttd%; 높을수록 처치판단 빡빡)
    d4stage!(&format!("main cfg_thr={} sim={:#x}", cfg_thr, sim));
    // [좌표게이트 + 첫 TTD 루프]
    // ★게임 disc4 핸들러 rdx = 디스패처 param_2 + 8 (디스패처 add rdx,8; write_disc4_aux도 p2+8 사용). 좌표게이트 sel=*(rdx+0x48)이므로 subp(=디스패처param_2)+8 필요 — 기존 subp 기준이면 8 어긋나 잘못된 sel→첫TTD 오판→오답 code8→caller waypoint검증 실패 hang.
    d4stage!("coord?");
    if disc4_coord_pass(subp.wrapping_add(8), target, exe) {
        D4_TTD_PASS.fetch_add(1, Ordering::Relaxed);
        d4stage!("ttd1-acc(shadow vt168/def_resolve/combat/vt90)");
        let rbx = disc4_ttd_acc(obj, vt, target, sim, exe).wrapping_add(tower_threat_acc(p6, team, target));   // ★포탑위협 가산(target=self): self가 적포탑 사거리내면 acc↑→ttd↓
        let ttd: u64 = if rbx == 0 { u64::MAX } else { hp.wrapping_mul(tune("d4_ttd_scale", 1000) as u64) / rbx };   // ★튜닝: TTD 분자 스케일
        d4stage!(&format!("ttd1={} rbx={} cfg={}", ttd, rbx, cfg_thr));
        if ttd > cfg_thr { D4_TTD_C8.fetch_add(1, Ordering::Relaxed); return disc4_engage_or_hold(8, p6, team, target); }   // ★인원수 보정
    }
    // [SUBPLAN_BRANCH → 항상 2nd_dispatch → ally매치]
    d4stage!("ally?(shadow vt168/def_resolve)");
    if disc4_ally_match(obj, vt, target) {
        // [3rd_dispatch TTD] 좌표게이트 없음, acc==0→1(첫루프의 MAX와 다름).
        d4stage!("3rd-acc(shadow)");
        let mut acc = disc4_ttd_acc(obj, vt, target, sim, exe).wrapping_add(tower_threat_acc(p6, team, target));   // ★포탑위협 가산
        if acc == 0 { acc = 1; }
        let ttd3 = hp.wrapping_mul(tune("d4_ttd_scale", 1000) as u64) / acc;   // ★튜닝: 3rd TTD 분자 스케일
        let c = if ttd3 > cfg_thr { disc4_engage_or_hold(8, p6, team, target) } else { 7 };   // ★인원수 보정
        let n3 = D4_3RD.fetch_add(1, Ordering::Relaxed);
        if n3 < 300 {
            write_named("d4ttd.txt", &format!("좌표게이트통과누적={} 3rd_dispatch누적={}\n마지막3rd: acc={} hp={} ttd3={} cfg={} → code{}\n",
                D4_TTD_PASS.load(Ordering::Relaxed), n3+1, acc, hp, ttd3, cfg_thr, c));
        }
        return c;
    }
    // [late-code7] (2nd 무매치): ★vt30/vt40 능력게이트 shadow-call이 day-11 call#1308서 게임함수 내부 hang = freeze 유일 범인(d4last.txt "subplan-r13b" EXIT없음).
    //   → late7은 게임 dispatcher에 위임(passthrough=-99). disc4 RNG-free라 무desync. subplan_r13b/thr/hp_pct 미실행 = hang 원천차단. game==mine 유지(late7만 게임이 정확처리).
    //   향후 vt30/vt40 순수재현(챔피언 능력 reach) 완료시 late7도 우리코드 복귀 가능.
    let _ = (subp, hp, maxhp);
    d4stage!("EXIT -99 late7-passthrough");
    -99
}
unsafe fn my_movepriority(disc: u64, r15: usize, r14: usize, subp: usize, r8: u64, r9: usize, p7_dd: usize, p7p: usize) -> i64 {
    let _pg = perf_guard(6);
    match disc {
        2 | 5 => return 7,                  // ForcePassive/ActiveRecall 인라인
        0 | 1 | 6 => return disc as i64,    // data-var: [rsi]=r11=[rdx]=disc
        7 | 8 => return 0xa,                // Ganker/Cover: 핸들러 공유 0x1c086cb [rsi]=0xa 하드코딩 → 코드10
        4 => return my_disc4(subp, r14, r15),   // PassiveJungle(0x206e530) first-cut: code7(홈+HP)/code8(메인). code3/late7 미세조정 대상
        3 => return my_dd7700_code(subp + 8, r8, r9, r14, r15, p7_dd, false),  // PassiveLine: 디스패처가 rdx+8 포워딩(cover 정상)
        14 => return my_defense_nexus(r8, r14, r15),                    // DefenseNexus 충실재현(p3=r8 count gate)
        9 => return my_epic_poke(subp + 8, r8, r14, r15, p7p, p7_dd),   // EpicPoke: p7=arg7(entry+0x38), p8=arg8(entry+0x40=p7_dd)
        11 => return my_serpen_poke(subp + 8, r8, r14, r15, p7p, p7_dd), // SerpenPoke (동일 9인자 포워딩)
        10 => return my_epic_battle(subp + 8, r14, r15, p7p),           // EpicBattle: p2=subp+8, p5=r14(lanectx), p6=r15(geom), p7=p7p(threat)
        12 => return my_serpen_battle(subp + 8, r14, r15, p7p),         // SerpenBattle (동일 매핑)
        _ => {}
    }
    let rh = rd_u64(r15).unwrap_or(0) as usize;
    if !ptr_ok(rh) { return -99; }
    let robj = rd_u64(rh).unwrap_or(0) as usize;
    let rvt  = rd_u64(rh + 8).unwrap_or(0) as usize;
    if !ptr_ok(robj) || !ptr_ok(rvt) { return -99; }
    match disc {
        13 => {  // AttackNexus 인라인: 홈리전&HP안풀→7, else rh[(1-team)*0x20+0x148]==0→0x11 / else→2
            if !ptr_ok(r14) { return -99; }
            let arg = rd_u64(r14 + 0x6a0).unwrap_or(0) as usize;
            let s = vt_slot(rvt, 0x128); if !ptr_ok(s) { return -99; }
            let f: VtPtr2Fn = core::mem::transmute(s);
            let ent = f(robj, arg);
            if !ptr_ok(ent) || !readable(ent + 0x658, 8) || !readable(ent + 0x648, 8) { return -99; }
            let team = rd_u64(r14 + 0x6a8).unwrap_or(2);
            if team > 1 { return -99; }
            let x = rd_u64(ent + 0x648).unwrap_or(0);
            let y = rd_u64(ent + 0x650).unwrap_or(0);
            let r10 = if team == 0 { 0xfa00u64 } else { 0xea600 };
            let cond_x = ((x >= 0xd9c60) || team == 0) && (x <= r10);
            let mut home = false;
            if cond_x {
                // ★FIX(disasm 0x1c38d06 cmove rcx,r8): y_bound = team==0?0xea600:0xfa00 (x_bound과 교차). 기존 swap버그=team0 home 영영false.
                let rcy = if team != 0 { 0xfa00u64 } else { 0xea600 };
                let cond_y = ((y >= 0xdac00) || team != 0) && (y <= rcy);
                if cond_y {
                    let cur = rd_u64(ent + 0x658).unwrap_or(0);
                    let max = rd_u64(ent + 0x610).unwrap_or(0);
                    if cur < max { home = true; }
                }
            }
            if home { return 7; }
            let idx = 1u64.wrapping_sub(team) as usize;
            let v = rd_u64(rh + idx*0x20 + 0x148).unwrap_or(0);
            if v == 0 { 0x11 } else { 2 }
        }
        _ => -99,   // 9/11(epic/serpen poke) judges = Stage 2c 미완
    }
}

// ★DefenseNexus 7-watcher 로깅(kind7 disc14 & kind8 공용). game!=18 케이스만 무제한 기록.
unsafe fn defwatch_log(code: i64, mine: i64, diag: i64) {
    let dn = DEFW_N.fetch_add(1, Ordering::Relaxed);
    if dn >= 1000 { return; }
    let d = diag as u64;
    let (hp, home, near, side, pred, nhp) = (d & 0xff, (d>>8)&1, (d>>9)&1, (d>>10)&1, (d>>11)&1, (d>>16)&0xff);
    let verdict = if mine == code { "OK✓" } else { "★MISS" };
    let s = format!("[defw #{}] game={} my={} [{}] hp%={} nexus_hp%={} home={} near={} pred={} side={}\n",
        dn, code, mine, verdict, hp, nhp, home, near, pred, side);
    if !DEFW_INIT.swap(true, Ordering::Relaxed) { write_named("defwatch.txt", "=== DefenseNexus(subplan=14) 7-watcher: game!=18(=7) 케이스만 (무제한·무강제) ===\n"); }
    append_named("defwatch.txt", &s);
}

// ★facet#4 movepriority 검증 캡처: my_movepriority vs 게임 출력코드(*rsi[0]). kind7=정상(캡), kind8=DefNexus 7-watcher(무제한).
// ★movepriority disc 0/1 인라인 출력 완전재현(dispatcher idx4 @0x1c38d81). p1=출력sret, p2=subplan(rdx), p3=param_3(r8).
//   [p1]=disc, [p1+8]=[p2+8], [p1+0x10..1f]=[p2+0x58..67](16B), [p1+0x20]=(p3>=0xb)&[p2+0x88], [p1+0x21]=[p2+0x8b]. +0x22~ 미터치(게임도 동일).
unsafe fn mp_write_disc01(p1: usize, p2: usize, p3: u64) -> bool {
    if !readable(p2, 0x90) { return false; }   // ★p2 read 가드 유지(writable은 아래 probe로 대체)
    let disc = rd_u64(p2).unwrap_or(0);
    if !wr_u64(p1, disc) { return false; }   // ★probe+write 첫필드(writable VQ제거)→성공시 나머지 raw, 실패=passthrough(RNG무관이라 안전)
    std::ptr::write_unaligned((p1 + 8) as *mut u64, rd_u64(p2 + 8).unwrap_or(0));
    std::ptr::write_unaligned((p1 + 0x10) as *mut u64, rd_u64(p2 + 0x58).unwrap_or(0));
    std::ptr::write_unaligned((p1 + 0x18) as *mut u64, rd_u64(p2 + 0x60).unwrap_or(0));
    let f20 = (if p3 >= 0xb { 1u8 } else { 0 }) & rd_u8(p2 + 0x88);
    std::ptr::write_unaligned((p1 + 0x20) as *mut u8, f20);
    std::ptr::write_unaligned((p1 + 0x21) as *mut u8, rd_u8(p2 + 0x8b));
    true
}
// ★movepriority disc 7/8 서브코드 헬퍼 FUN_142078a60 재현. 웨이포인트선택 + 타입체크 → 서브코드(+8값).
//   p1=byte[subplan+0x28], team=[p5+0x6a8], rh=*[p6](로스터 struct: [0]=obj/[1]=vt/[team+0x30..]=웨이포인트), p5arg=[p6+8], cand=vt[0x128]결과(=헬퍼 lVar3, 게이트가 resolve).
unsafe fn my_mp_sub_a60(p1: u8, team: u64, rh: usize, p5arg: usize, cand: usize) -> Option<i64> {
    if team > 1 { return None; }
    let tu = team as usize;
    let (idx, off) = match p1 { 0 => (0x30usize, 400usize), 1 => (0x34, 0x1b0), _ => (0x38, 0x1d0) };
    let mut lv6 = rd_u64(rh + (tu + idx)*8).unwrap_or(0);
    if lv6 == 0 { lv6 = rd_u64(rh + tu*8 + off).unwrap_or(0); }
    if lv6 == 0 {   // 웨이포인트 null → p1/team 코드
        let (c4, c2): (i64, i64) = match p1 { 0 => (2, 0x10), 1 => (4, 0x11), _ => (9, 0x15) };
        return Some(if team == 0 { c4 } else { c2 });
    }
    let lv6 = lv6 as usize;
    if cand == 0 { return None; }   // 게임 panic 경로
    let iv5 = rd_i32(lv6 + 0x68).unwrap_or(0);
    let p128 = rd_u8(lv6 + 0x128);
    let p88 = rd_u64(lv6 + 0x88).unwrap_or(0);
    let p128_thr = tune("d8_slot_thr", 5);   // ★튜닝: 슬롯 우선순위 임계(p128<5)
    match p1 {
        0 => { if iv5 != 2 { return None; } let cond = (p128 as i64) < p128_thr && p88 != 0; Some(if cond { if team==0 {6} else {3} } else if team==0 {3} else {6}) }
        1 => {
            if !ptr_ok(p5arg) { return None; }   // ★readable VQ제거(cand는 위 cand==0 가드됨, 좌표 rd_u64)
            let m = rd_u64(p5arg + 8).unwrap_or(0) as usize;
            let ydiff = rd_u64(m + 0x12c0).unwrap_or(0).wrapping_sub(rd_u64(cand + 0x650).unwrap_or(0));
            let near = ydiff < rd_u64(cand + 0x648).unwrap_or(0);
            if iv5 == 2 && (p128 as i64) >= p128_thr {   // ★튜닝: 슬롯 임계 이상(원본 p128>4)
                if near { Some(if team != 0 { 0x12 } else { 0xd }) } else { Some(if team != 0 { 0xc } else { 8 }) }
            } else { Some(if near { 0xe } else { 0xb }) }
        }
        _ => { if iv5 != 2 { return None; } let cond = (p128 as i64) < p128_thr && p88 != 0; let b = if cond { team == 0 } else { team != 0 }; Some(if b { 0x14 } else { 0xf }) }
    }
}
unsafe extern "C" fn mp_capture(saved: usize, entry_rsp: usize) -> i64 {
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return 1; }
    // ★완전대체(mp_repl): disc 0/1 인라인출력 재현→대체(원본 dispatcher skip, rax=rcx=sret). 그 외 disc=passthrough(원본+capture).
    if MP_REPL.load(Ordering::Relaxed) {
        let p1 = rd_u64(saved + 0x28).unwrap_or(0) as usize;
        let p2 = rd_u64(saved + 0x20).unwrap_or(0) as usize;
        if ptr_ok(p1) && ptr_ok(p2) && readable(p2, 8) {
            let disc = std::ptr::read_unaligned(p2 as *const u64);
            if disc == 0 || disc == 1 || disc == 6 {
                // 인라인 케이스(idx4): 전체출력(code+ptr+16B블롭+2플래그) 재현.
                let p3 = rd_u64(saved + 0x18).unwrap_or(0);
                if mp_write_disc01(p1, p2, p3) {
                    let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                    if n == 1 || n % 500 == 0 {
                        if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                        append_named("mpcmp.txt", &format!("[mp REPL #{}] disc={} (전체출력 대체) | passthrough={}\n", n, disc, MP_REPL_PASS.load(Ordering::Relaxed)));
                    }
                    return 0;   // HANDLED → rax=rcx=p1(sret), 원본 skip
                }
            } else if disc == 2 || disc == 5 {
                // ★disc 2/5 인라인(0x1c38c8c): [param_1]=7 (code-only). aux 미터치(게임도 동일).
                if wr_u64(p1, 7u64) {   // ★B-3: 단일 write라 부분쓰기위험0 → wr_u64(writable VQ 제거, valid sim 비트동일)
                    MP_REPL_N.fetch_add(1, Ordering::Relaxed);
                    return 0;
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);
            } else if disc == 8 {
                // ★disc 8: vt[0x128]게이트 + 헬퍼(0x142078a60) → 공유write(0x1c38edb): code10+서브코드(+8)+플래그(+0x10=1,+0x12=0).
                let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;   // p5 lanectx
                let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;   // p6 geom
                let rh = rd_u64(r15).unwrap_or(0) as usize;                 // *[p6] = 로스터 struct
                let mut done = false;
                if ptr_ok(rh) && ptr_ok(r14) && readable(rh, 16) {
                    let robj = rd_u64(rh).unwrap_or(0) as usize;
                    let rvt = rd_u64(rh + 8).unwrap_or(0) as usize;
                    let gate = vt_slot(rvt, 0x128);
                    let cand = if ptr_ok(gate) && ptr_ok(robj) { let g: VtPtr2Fn = core::mem::transmute(gate); g(robj, rd_u64(r14 + 0x6a0).unwrap_or(0) as usize) } else { 0 };
                    if cand != 0 {
                        let p1c = rd_u8(p2 + 0x28);
                        let team = rd_u64(r14 + 0x6a8).unwrap_or(2);
                        let p5arg = rd_u64(r15 + 8).unwrap_or(0) as usize;
                        if let Some(sub) = my_mp_sub_a60(p1c, team, rh, p5arg, cand) {
                            if wr_u64(p1, 0xau64) {   // ★probe+write: 첫필드 wr_*로 writability확인→성공시 나머지 raw(같은 sret 할당=안전, writable VQ 제거)
                                std::ptr::write_unaligned((p1 + 8) as *mut u64, sub as u64);
                                std::ptr::write_unaligned((p1 + 0x10) as *mut u16, 1u16);
                                std::ptr::write_unaligned((p1 + 0x12) as *mut u8, 0u8);
                                let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                                if n % 500 == 0 {
                                    if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                                    append_named("mpcmp.txt", &format!("[mp REPL #{}] disc=8 code=10 sub={}\n", n, sub));
                                }
                                done = true;
                            }
                        }
                    }
                }
                if done { return 0; }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);
            } else if disc == 14 {
                // ★DefenseNexus: code-only(디컴 *param_1=code만). my_defense_nexus(검증됨) → +0만 write, aux 미터치(게임도 동일).
                let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;
                let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;
                let r8 = rd_u64(saved + 0x18).unwrap_or(0);
                let r9 = rd_u64(saved + 0x10).unwrap_or(0) as usize;
                let p7_dd = rd_u64(entry_rsp + 0x40).unwrap_or(0) as usize;
                let p7p = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize;
                let code = my_movepriority(14, r15, r14, p2, r8, r9, p7_dd, p7p);
                if code != -99 && wr_u64(p1, code as u64) {   // ★B-3: 단일 write → wr_u64(writable VQ 제거)
                    let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                    if n % 500 == 0 {
                        if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                        append_named("mpcmp.txt", &format!("[mp REPL #{}] disc=14(code-only) code={}\n", n, code));
                    }
                    return 0;   // HANDLED
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);
            } else if disc == 13 {
                // ★AttackNexus 인라인(0x1c38c98) 완전대체. 출력계약 disasm확정:
                //   code7(홈+HP안풀)→[p1]=7 only / code0x11(적구조물==0)→[p1]=0x11 only /
                //   code2(else)→[p1]=2 + [p1+8]=0(u8),[p1+9]=byte[subplan+0x10],[p1+0xa]=2(u8). aux는 code2에서만 write(게임도 동일).
                let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;   // p5 lanectx
                let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;   // p6 geom
                let code = my_movepriority(13, r15, r14, p2, 0, 0, 0, 0);
                if code != -99 && wr_u64(p1, code as u64) {   // ★probe+write: 첫필드 wr_*로 writability확인→성공시 나머지 raw(writable VQ 제거)
                    if code == 2 {
                        std::ptr::write_unaligned((p1 + 8) as *mut u8, 0u8);
                        std::ptr::write_unaligned((p1 + 9) as *mut u8, rd_u8(p2 + 0x10));
                        std::ptr::write_unaligned((p1 + 0xa) as *mut u8, 2u8);
                    }
                    let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                    if n % 500 == 0 {
                        if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                        append_named("mpcmp.txt", &format!("[mp REPL #{}] disc=13(AttackNexus) code={}\n", n, code));
                    }
                    return 0;   // HANDLED → rax=rcx=p1(sret)
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);
            } else if disc == 4 {
                // ★disc4(PassiveJungle/0x206e530) 완전대체. cfg d4_repl=0이면 passthrough(freeze 격리). coord_pass subp+8 수정됨. d4freeze=1이면 my_disc4 단계별 d4last.txt.
                if D4_REPL.load(Ordering::Relaxed) {
                    let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;
                    let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;
                    let code = my_movepriority(4, r15, r14, p2, 0, 0, 0, 0);
                    if code != -99 && write_disc4_aux(p1, code, p2 + 8) {
                        let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                        if n % 500 == 0 {
                            if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                            append_named("mpcmp.txt", &format!("[mp REPL #{}] disc=4(PassiveJungle) code={}\n", n, code));
                        }
                        return 0;   // HANDLED → rax=rcx=p1(sret)
                    }
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);   // d4_repl=0 또는 write실패 → passthrough(게임 dispatcher 실행)
            } else if (disc == 9 || disc == 11) && POKE_REPL.load(Ordering::Relaxed) {
                // ★EpicPoke/SerpenPoke 완전대체(2026-06-20 RNG-sync 검증완료): my_movepriority(출력코드, pokecmp DIFF=0) + write_poke_aux(full output) + FUN_1420e88a0 draw writeback(p4=r9, e88a0_p4=r14=param5, e88a0_p7=*(r15+8)=param6[1]. 재구성검증 eDIFF=0/3998). count계산 실패→passthrough(desync 방지).
                let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;
                let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;
                let r8 = rd_u64(saved + 0x18).unwrap_or(0);
                let r9 = rd_u64(saved + 0x10).unwrap_or(0) as usize;   // = poke param_4 = RNG state
                let p7_dd = rd_u64(entry_rsp + 0x40).unwrap_or(0) as usize;
                let p7p = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize;
                // ① RNG draw 재현 가능여부(count) 먼저 — 실패시 대체 안 함(원본이 RNG소비).
                let e88_p7 = rd_u64(r15 + 8).unwrap_or(0) as usize;
                let cnt_opt = if ptr_ok(r9) && readable(r9 + 0x108, 8) && ptr_ok(r14) && readable(r14 + 0x718, 8) {
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_e88a0_count(r14, e88_p7))).unwrap_or(None)
                } else { None };
                if let Some(cnt0) = cnt_opt {
                    // ★disc11 serpen은 plan==1서만 e88a0 draw. plan!=1이면 무draw → cnt=0(pokerng eDIFF 수정, replay desync 해결).
                    let plan_v = if readable(p7_dd + 0x3e6, 1) { rd_u8(p7_dd + 0x3e6) } else { 255 };
                    let cnt = if disc == 11 && plan_v != 1 { 0 } else { cnt0 };
                    let code = my_movepriority(disc, r15, r14, p2, r8, r9, p7_dd, p7p);
                    if code != -99 && writable(p1, 0x18) && write_poke_aux(p1, disc == 9, code, p2 + 8, r15) {
                        // ② RNG-sync: count>0이면 gen_range(0,count-1) 만큼 p4(r9) writeback(=FUN_1420e88a0 소비 대체). count==0=무draw.
                        if cnt > 0 {
                            if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| rng_advance_writeback(r9, 0, cnt - 1))).unwrap_or(None).is_some() {
                                DD7_REPL_RNG_N.fetch_add(1, Ordering::Relaxed);
                            }
                        }
                        let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                        if n % 500 == 0 {
                            if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                            append_named("mpcmp.txt", &format!("[mp REPL #{}] disc={}({}) code={} cnt={} rngWB={}\n", n, disc, if disc==9 {"EpicPoke"} else {"SerpenPoke"}, code, cnt, DD7_REPL_RNG_N.load(Ordering::Relaxed)));
                        }
                        return 0;   // HANDLED → rax=rcx=p1(sret)
                    }
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);
            } else if disc == 3 && DD7_REPL.load(Ordering::Relaxed) {
                // ★dd7700(PassiveLine) 완전대체(2026-06-20 RNG-sync 검증완료 DIFF=0/21500): my_dd7700_full(전체출력, dd7full DIFF=0) + my_dd7700_rng_final writeback(p4 RNG 전진=skip시 no-desync). None(engage 6/7·plan8 rare)→passthrough. panic-safe.
                let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;
                let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;
                let r8 = rd_u64(saved + 0x18).unwrap_or(0);
                let r9 = rd_u64(saved + 0x10).unwrap_or(0) as usize;   // = dd7700 param_4 = RNG state
                let p7_dd = rd_u64(entry_rsp + 0x40).unwrap_or(0) as usize;
                // ① 출력 재현(p4 read-only). 성공시에만 대체(실패=passthrough → 원본 dd7700이 출력+RNG 자체수행).
                let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_dd7700_full(p1, p2 + 8, r8, r9, r14, r15, p7_dd))).unwrap_or(None);
                if let Some(consumes_rng) = res {
                    // ②★레버: engage(consumes_rng=true)만 rng_final 호출. cover·main(false)=RNG 0 draw 확정 → rng_final 통째 skip(중복 cover검출/role/sim_hdr 1회 제거 = native급 단일순회). draw 0이라 state 불변=비트동일.
                    if consumes_rng {
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        if let Some((fidx, refills, buf)) = my_dd7700_rng_final(r9, p2 + 8, r8, r14, r15, p7_dd) {
                            // ★B-3: writable VQ가드 제거 → wr_*(폴트세이프). 합법=동일write+카운터, 불법=무쓰기(valid sim 비트동일).
                            let mut ok = true;
                            if refills > 0 {
                                for i in 0..64usize { if !wr_u32(r9 + i*4, buf[i]) { ok = false; break; } }
                                if ok { let c0 = rd_u64(r9 + 0x130).unwrap_or(0); ok = wr_u64(r9 + 0x130, c0.wrapping_add(4u64.wrapping_mul(refills))); }
                            }
                            if ok && wr_u64(r9 + 0x100, fidx) {
                                DD7_REPL_RNG_N.fetch_add(1, Ordering::Relaxed);
                            }
                        }
                    }));
                    }
                    let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                    if n % 500 == 0 {
                        if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                        append_named("mpcmp.txt", &format!("[mp REPL #{}] disc=3(dd7700/PassiveLine) rngWB={}\n", n, DD7_REPL_RNG_N.load(Ordering::Relaxed)));
                    }
                    return 0;   // HANDLED
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);
            } else if disc == 10 || disc == 12 {
                // ★EpicBattle/SerpenBattle 대체 (⚠정상매치 dead=미검증, disasm only). my_movepriority(disc10→my_epic_battle/disc12→my_serpen_battle) + write_battle_aux.
                let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;
                let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;
                let r8 = rd_u64(saved + 0x18).unwrap_or(0);
                let r9 = rd_u64(saved + 0x10).unwrap_or(0) as usize;
                let p7_dd = rd_u64(entry_rsp + 0x40).unwrap_or(0) as usize;
                let p7p = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize;
                let code = my_movepriority(disc, r15, r14, p2, r8, r9, p7_dd, p7p);
                if code != -99 && writable(p1, 0x18) && write_battle_aux(p1, code, p2 + 8) {
                    let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                    if n % 500 == 0 {
                        if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                        append_named("mpcmp.txt", &format!("[mp REPL #{}] disc={}({}) code={}\n", n, disc, if disc==10 {"EpicBattle"} else {"SerpenBattle"}, code));
                    }
                    return 0;   // HANDLED
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);
            } else {
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);   // 그 외 disc = 미대체(원본실행) — aux 재현 필요(grind)
            }
        }
    }
    if !MPCAP.load(Ordering::Relaxed) && !DEFWATCH.load(Ordering::Relaxed) { return 1; }
    let thunk = RET_THUNK.load(Ordering::Relaxed);
    if thunk == 0 { return 1; }
    let out  = rd_u64(saved + 0x28).unwrap_or(0) as usize;   // rcx = 출력ptr(rsi)
    let subp = rd_u64(saved + 0x20).unwrap_or(0) as usize;   // rdx = subplan ptr
    if !ptr_ok(out) || !ptr_ok(subp) || !readable(subp, 8) { return 1; }
    let disc = std::ptr::read_unaligned(subp as *const u64);
    let di = (disc as usize).min(15);
    // 정상 캡처(캡 적용) vs DefNexus 7-watcher(무제한). 둘 다 불가시 skip.
    let sub_cap = if di == 9 || di == 11 { 3000 } else { MP_SUB_CAP };   // ★disc9/11=combat상태 잡게 캡↑(초반 poke자세 12 홍수 통과)
    let normal = MPCAP.load(Ordering::Relaxed) && MP_ARMED.load(Ordering::Relaxed) < 30000 && MP_SUB_ARMED[di].load(Ordering::Relaxed) < sub_cap;
    let watch14 = DEFWATCH.load(Ordering::Relaxed) && disc == 14 && DEFW_ARMED.load(Ordering::Relaxed) < 200000;
    if !normal && !watch14 { return 1; }
    let kind: u8 = if normal { 7 } else { 8 };
    let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;   // arg5 = dd7700/DefNexus p5 (lane ctx)
    let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;   // arg6 = p6 (geom handle)
    let r8  = rd_u64(saved + 0x18).unwrap_or(0);                // r8 = p3 (count gate; dd7서 항상 0x27)
    let r9  = rd_u64(saved + 0x10).unwrap_or(0) as usize;       // r9 = dd7700 p4 (STAGE6 reindex/RNG)
    let p7_dd = rd_u64(entry_rsp + 0x40).unwrap_or(0) as usize; // arg8(=dispatcher r10) = dd7700 p7(champion) = poke p8
    let p7p   = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize; // arg7(=dispatcher rcx) = poke p7(threat ctx)
    let my = my_movepriority(disc, r15, r14, subp, r8, r9, p7_dd, p7p);
    let diag: i64 = if disc == 14 { DEF_DIAG.load(Ordering::Relaxed) as i64 } else { -99 };
    let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    if !ptr_ok(orig_ret) || !readable(entry_rsp, 8) { return 1; }
    let pre = format!("[mp #{}] subplan={} my={}", MP_ARMED.load(Ordering::Relaxed), disc, my);
    // ★출력계약 진단: 진입시 *out 8qword 스냅 → 리턴(kind7)서 diff = sub-judge write-set.
    if kind == 7 && readable(out, 0x40) {
        for k in 0..8usize { MP_ENTRY[k].store(rd_u64(out + k*8).unwrap_or(0), Ordering::Relaxed); }
        MP_ENTRY_PTR.store(out, Ordering::Relaxed);
        // ★disc9/11 full-output 검증용 aux입력 보관(hook_return kind7서 write_poke_aux 재현 대조).
        if disc == 9 || disc == 11 || disc == 10 || disc == 12 { MP_AUX_OP.store(out, Ordering::Relaxed); MP_AUX_P2.store(subp + 8, Ordering::Relaxed); MP_AUX_P6.store(r15, Ordering::Relaxed); }
        else if disc == 4 { MP_AUX_OP.store(out, Ordering::Relaxed); MP_AUX_P2.store(subp + 8, Ordering::Relaxed); MP_AUX_P6.store(r15, Ordering::Relaxed); }   // ★disc4 param_2=subp+8(디스패처 add rdx,8 확인). aux=*(p2+0x48)active/*(p2+0x60)facet
        // ★disc9/11 RNG footprint: 진입 p4(=r9) idx/counter 스냅 (리턴서 delta=실제 draw). early-guard·plan도 보관(0draw 경로 식별).
        if (disc == 9 || disc == 11) && ptr_ok(r9) && readable(r9 + 0x138, 8) {
            POKE_RNG_P4.store(r9, Ordering::Relaxed);
            POKE_RNG_I0.store(rd_u64(r9 + 0x100).unwrap_or(0), Ordering::Relaxed);
            POKE_RNG_C0.store(rd_u64(r9 + 0x130).unwrap_or(0), Ordering::Relaxed);
            let guard = (rd_u8(subp + 8) != 0 || rd_u8(subp + 9) != 0) as u8;
            POKE_RNG_GUARD.store(guard, Ordering::Relaxed);
            POKE_RNG_PLAN.store(if readable(p7_dd + 0x3e6, 1) { rd_u8(p7_dd + 0x3e6) as i64 } else { -1 }, Ordering::Relaxed);
            POKE_INSCOPE.store(true, Ordering::Relaxed);   // ★RNG caller 추적 윈도우 시작(서브저지 실행 중 fcd980/fcdaf0 caller RVA 로깅)
            // ★e88a0 arg 재구성 검증: e88a0_p4=r14(param5), e88a0_p7=*(r15+8)(param6[1]). count→예측 exit(RNG=r9). kind7서 실제 p4 exit과 대조.
            POKE_PCOUNT.store(-1, Ordering::Relaxed);
            let e88_p7 = rd_u64(r15 + 8).unwrap_or(0) as usize;
            if ptr_ok(r14) && readable(r14 + 0x718, 8) {
                if let Some(cnt0) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_e88a0_count(r14, e88_p7))).unwrap_or(None) {
                    // ★disc11 serpen은 plan==1 브랜치서만 e88a0 gather+draw. plan!=1(255 등)이면 게임 무draw → cnt=0(pokerng eDIFF=9 수정).
                    let plan_v = if readable(p7_dd + 0x3e6, 1) { rd_u8(p7_dd + 0x3e6) } else { 255 };
                    let cnt = if disc == 11 && plan_v != 1 { 0 } else { cnt0 };
                    let c0 = rd_u64(r9 + 0x130).unwrap_or(0);
                    let (pidx, prf) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_e88a0_exit(r9, cnt))).unwrap_or(None).unwrap_or((rd_u64(r9+0x100).unwrap_or(0), 0));
                    POKE_PCOUNT.store(cnt as i64, Ordering::Relaxed);
                    POKE_PIDX.store(pidx, Ordering::Relaxed);
                    POKE_PCTR.store(c0.wrapping_add(4u64.wrapping_mul(prf)), Ordering::Relaxed);
                }
            }
        } else if disc == 9 || disc == 11 { POKE_RNG_P4.store(0, Ordering::Relaxed); }
    }
    let pushed = if let Ok(mut st) = RET_STACK.lock() {
        if st.len() < 64 { st.push(RetFrame{ key: entry_rsp, orig_ret, mine: my, kind, pre, p5: out, p6: disc as usize, disp_pred: diag }); true } else { false }
    } else { false };
    if pushed {
        core::ptr::write_unaligned(entry_rsp as *mut usize, thunk);
        if kind == 7 { MP_ARMED.fetch_add(1, Ordering::Relaxed); MP_SUB_ARMED[di].fetch_add(1, Ordering::Relaxed); }
        else { DEFW_ARMED.fetch_add(1, Ordering::Relaxed); }
    }
    1   // passthrough (원본 dispatcher 실행 → 리턴훅 검증)
}

// ★ChaCha refill 훅: 1421bbc10(rcx=input, r8=output버퍼). 진입시 내 ChaCha12 재현→MY_CHACHA, 리턴훅 kind:4서 게임버퍼와 워드대조.
unsafe extern "C" fn chacha_capture(saved: usize, entry_rsp: usize) {
    // ★disc11 RNG 소스 특정: refill(FUN_14222f3c0=RVA_CHACHA) caller = 인라인 gen_range 함수(또는 fcdaf0/fcd980). POKE_INSCOPE 윈도우중 caller RVA 로깅 → serpen draw 함수 직접 특정. rngcap 게이트 전에 실행.
    if POKE_INSCOPE.load(Ordering::Relaxed) { poke_ret_log("refill", rd_u64(entry_rsp).unwrap_or(0) as usize); }
    if !RNGCAP.load(Ordering::Relaxed) || CHACHA_ARMED.load(Ordering::Relaxed) >= CHACHA_ARM_MAX { return; }
    let thunk = RET_THUNK.load(Ordering::Relaxed);
    if thunk == 0 { return; }
    let input = rd_u64(saved + 0x28).unwrap_or(0) as usize;   // rcx = ChaCha input(key/counter/nonce)
    let outp = rd_u64(saved + 0x18).unwrap_or(0) as usize;    // r8 = output 버퍼(256B)
    if !ptr_ok(input) || !ptr_ok(outp) || !readable(outp, 0x100) { return; }
    let mut buf = [0u32; 64];
    if !chacha_reproduce(input, &mut buf) { return; }
    if let Ok(mut m) = MY_CHACHA.lock() { *m = buf; } else { return; }
    let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    if !ptr_ok(orig_ret) || !readable(entry_rsp, 8) { return; }
    let pushed = if let Ok(mut st) = RET_STACK.lock() {
        if st.len() < 64 { st.push(RetFrame{ key: entry_rsp, orig_ret, mine: 0, kind: 4, pre: String::new(), p5: outp, p6: 0, disp_pred: -99 }); true } else { false }
    } else { false };
    if pushed {
        core::ptr::write_unaligned(entry_rsp as *mut usize, thunk);
        CHACHA_ARMED.fetch_add(1, Ordering::Relaxed);
    }
}

// ★facet#2 FUN_141dd7700 진입 캡처(경량): param_2/5/6/7 저장. install_detour 컨벤션(saved/entry_rsp).
unsafe extern "C" fn dd7700_capture(saved: usize, entry_rsp: usize) {
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let p2 = rd_u64(saved + 0x20).unwrap_or(0) as usize;       // rdx = param_2
    let p5 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;   // param_5 (lane ctx)
    let p6 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;   // param_6 (geometry)
    let p7 = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize;   // param_7 (champion)
    if !ptr_ok(p5) || !ptr_ok(p6) || !ptr_ok(p7) { return; }
    DD7700_P2.store(p2, Ordering::Relaxed);
    DD7700_P5.store(p5, Ordering::Relaxed);
    DD7700_P6.store(p6, Ordering::Relaxed);
    DD7700_P7.store(p7, Ordering::Relaxed);
    DD7700_P3.store(rd_u64(saved + 0x18).unwrap_or(0) as usize, Ordering::Relaxed);
    DD7700_P4.store(rd_u64(saved + 0x10).unwrap_or(0) as usize, Ordering::Relaxed);
    let n = DD7700_N.fetch_add(1, Ordering::Relaxed);
    // ── L80[1] vtable + 슬롯 1회 캡처 (dd7700/f22e80 간접호출 타깃 = 재구현 대상) ──
    if DD7_VT.load(Ordering::Relaxed) == 0 {
        let l80 = rd_u64(p6).unwrap_or(0) as usize;
        if ptr_ok(l80) {
            let vt = rd_u64(l80 + 8).unwrap_or(0) as usize;
            if ptr_ok(vt) {
                DD7_S20.store(rd_u64(vt+0x20).unwrap_or(0) as usize, Ordering::Relaxed);
                DD7_S48.store(rd_u64(vt+0x48).unwrap_or(0) as usize, Ordering::Relaxed);
                DD7_SA8.store(rd_u64(vt+0xa8).unwrap_or(0) as usize, Ordering::Relaxed);
                DD7_S128.store(rd_u64(vt+0x128).unwrap_or(0) as usize, Ordering::Relaxed);
                DD7_S140.store(rd_u64(vt+0x140).unwrap_or(0) as usize, Ordering::Relaxed);
                DD7_S168.store(rd_u64(vt+0x168).unwrap_or(0) as usize, Ordering::Relaxed);
                DD7_VT.store(vt, Ordering::Relaxed);
            }
        }
    }
    // ── 게임 action code 캡처: 리턴훅 무장(kind:2). dd7700이 param_1을 리턴→retval==out ptr. ──
    if DD7CAP.load(Ordering::Relaxed) && DD7_ARMED.load(Ordering::Relaxed) < DD7_ARM_MAX {
        let thunk = RET_THUNK.load(Ordering::Relaxed);
        if thunk != 0 {
            let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
            if ptr_ok(orig_ret) && readable(entry_rsp, 8) {
                // ★재현: my_dd7700_code = 게임함수 호출 없이 action code 예측. -999=미예측.
                let p3 = rd_u64(saved+0x18).unwrap_or(0);
                let p4 = rd_u64(saved+0x10).unwrap_or(0) as usize;   // r9 (STAGE6 reindex; RNG state도 겸함)
                // ★panic-safe(mod-safety): my_dd7700_code panic 차단(AV는 못 잡지만 tail 게이트로 회피중).
                let pred: i64 = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_dd7700_code(p2, p3, p4, p5, p6, p7, false))).unwrap_or(-999);
                // ★my_dd7700_full을 capture시점(입력 정확)에 계산→DD7700_MY 저장(out pre-state base + my writes). out=param_1=rcx.
                {
                    let outp = rd_u64(saved + 0x28).unwrap_or(0) as usize;
                    if ptr_ok(outp) && readable(outp, 0x18) {
                        let mut scratch = [0u8; 0x18];
                        for i in 0..0x18usize { scratch[i] = rd_u8(outp + i); }
                        let sp = scratch.as_mut_ptr() as usize;
                        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_dd7700_full(sp, p2, p3, p4, p5, p6, p7))).unwrap_or(None);
                        DD7700_MY[0].store(u64::from_le_bytes(scratch[0..8].try_into().unwrap()), Ordering::Relaxed);
                        DD7700_MY[1].store(u64::from_le_bytes(scratch[8..16].try_into().unwrap()), Ordering::Relaxed);
                        DD7700_MY_OP.store(outp, Ordering::Relaxed);
                        DD7700_MY_RES.store(if res.is_some() { 1 } else { 0 }, Ordering::Relaxed);
                    } else { DD7700_MY_RES.store(2, Ordering::Relaxed); }
                }
                // ★RNG-sync 예측: dd7700이 소비할 RNG exit state(idx,counter) 예측 → hook_return서 실제 exit과 per-call 대조.
                {
                    DD7_RNG_VALID.store(false, Ordering::Relaxed);
                    if ptr_ok(p4) && readable(p4 + 0x138, 8) {
                        let c0 = rd_u64(p4 + 0x130).unwrap_or(0);
                        let i0 = rd_u64(p4 + 0x100).unwrap_or(0);
                        let (pidx, pctr) = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_dd7700_rng_final(p4, p2, p3, p5, p6, p7))).unwrap_or(None) {
                            Some((fidx, refills, _buf)) => (fidx, c0.wrapping_add(4u64.wrapping_mul(refills))),
                            None => (i0, c0),
                        };
                        DD7_RNG_P4.store(p4, Ordering::Relaxed);
                        DD7_RNG_PIDX.store(pidx, Ordering::Relaxed);
                        DD7_RNG_PCTR.store(pctr, Ordering::Relaxed);
                        DD7_RNG_VALID.store(true, Ordering::Relaxed);
                    }
                }
                let f = if readable(p2+0x19,1){ std::ptr::read_unaligned((p2+0x19) as *const u8) as i64 } else {-1};
                let plan = if readable(p7+0x3e6,1){ std::ptr::read_unaligned((p7+0x3e6) as *const u8) as i64 } else {-1};
                let side = rd_i64(p5+0x6a8).unwrap_or(-1);
                let lane = rd_i32(p5+0x738).unwrap_or(-1);
                let nm = cstr(rd_u64(p7+0x250).unwrap_or(0) as usize);
                let pre = format!("[dd7 #{}] {} side={} F={} lane={} plan={} p3={} mine={}",
                    DD7_ARMED.load(Ordering::Relaxed), nm, side, f, lane, plan, p3, pred);
                let pushed = if let Ok(mut st) = RET_STACK.lock() {
                    if st.len() < 64 { st.push(RetFrame{ key: entry_rsp, orig_ret, mine: pred, kind: 2, pre, p5, p6, disp_pred: -99 }); true } else { false }
                } else { false };
                if pushed {
                    core::ptr::write_unaligned(entry_rsp as *mut usize, thunk);
                    DD7_ARMED.fetch_add(1, Ordering::Relaxed);
                    // ★in-scope RNG 카운트 윈도우 시작(dd7700 실행 중 fcd980/fcdaf0 호출수 측정).
                    DD7_IS_980.store(0, Ordering::Relaxed); DD7_IS_AF0.store(0, Ordering::Relaxed);
                    DD7_INSCOPE.store(true, Ordering::Relaxed);
                }
            }
        }
    }
    // ★STEP5 목표선택 repro: live 1회(N==250 호출). vtable 타깃해석 = 섀도우안전(핸들→엔티티).
    if n == 250 && !DD7700_R_DONE.swap(true, Ordering::Relaxed) {
        let side = rd_i64(p5+0x6a8).unwrap_or(-1);
        if side < 0 || side > 1 { return; }
        let s = side as usize;
        if !readable(p2+0x19, 1) { return; }
        let f = std::ptr::read_unaligned((p2+0x19) as *const u8);
        let fu = f as usize;
        let l80 = rd_u64(p6).unwrap_or(0) as usize;
        let geo = rd_u64(p6+0x10).unwrap_or(0) as usize;
        if !ptr_ok(l80) || !ptr_ok(geo) { return; }
        let georec = geo + s*0x228;
        let foff = if f==1 {0x28usize} else if f==2 {0x50} else {0};
        let pi14 = georec + foff;
        let iv12 = rd_i32(pi14).unwrap_or(-99) as i64;
        DD7700_R_IV12.store(iv12, Ordering::Relaxed);
        let endp = rd_u64(l80 + s*8 + 0x170).unwrap_or(0) as usize;
        let near0 = rd_u64(l80 + s*8 + fu*32 + 0x180).unwrap_or(0) as usize;
        let near = if near0 != 0 { near0 } else { rd_u64(l80 + s*8 + fu*32 + 0x190).unwrap_or(0) as usize };
        // iv12==1: 타깃해석 + STEP5 거리비교. else: 단순경로(목표=near, 보고용).
        if iv12 == 1 && ptr_ok(endp) && ptr_ok(near) {
            let root = rd_u64(l80).unwrap_or(0) as usize;
            let a0 = rd_u64(l80+8).unwrap_or(0) as usize;
            let handle = rd_u64(pi14+8).unwrap_or(0) as usize;
            let resolver = if ptr_ok(a0) { rd_u64(a0+0x140).unwrap_or(0) as usize } else { 0 };
            if ptr_ok(resolver) && ptr_ok(root) {
                let rf: G2 = core::mem::transmute(resolver);
                let tent = rf(root, handle) as usize;   // 타깃 엔티티 해석
                if ptr_ok(tent) && readable(tent+0x650, 8) && readable(endp+0x650, 8) && readable(near+0x650, 8) {
                    let tx = rd_i64(tent+0x648).unwrap_or(0); let ty = rd_i64(tent+0x650).unwrap_or(0);
                    let epx = rd_i64(endp+0x648).unwrap_or(0); let epy = rd_i64(endp+0x650).unwrap_or(0);
                    let nx = rd_i64(near+0x648).unwrap_or(0); let ny = rd_i64(near+0x650).unwrap_or(0);
                    let dself = (epx-tx)*(epx-tx) + (epy-ty)*(epy-ty);
                    let dnear = (epx-nx)*(epx-nx) + (epy-ny)*(epy-ny);
                    DD7700_R_TX.store(tx, Ordering::Relaxed); DD7700_R_TY.store(ty, Ordering::Relaxed);
                    DD7700_R_DSELF.store(dself, Ordering::Relaxed); DD7700_R_DNEAR.store(dnear, Ordering::Relaxed);
                    // d_self<d_near → 목표=endpoint(nexus), else → near
                    if dself < dnear { DD7700_R_GOALX.store(epx, Ordering::Relaxed); DD7700_R_GOALY.store(epy, Ordering::Relaxed); DD7700_R_GOALKIND.store(1, Ordering::Relaxed); }
                    else { DD7700_R_GOALX.store(nx, Ordering::Relaxed); DD7700_R_GOALY.store(ny, Ordering::Relaxed); DD7700_R_GOALKIND.store(2, Ordering::Relaxed); }
                }
            }
        } else if ptr_ok(near) && readable(near+0x650, 8) {
            // 단순경로: 목표=near (보고용)
            DD7700_R_GOALX.store(rd_i64(near+0x648).unwrap_or(0), Ordering::Relaxed);
            DD7700_R_GOALY.store(rd_i64(near+0x650).unwrap_or(0), Ordering::Relaxed);
            DD7700_R_GOALKIND.store(2, Ordering::Relaxed);
        }
    }
}

// ── 결정 ──
#[derive(Clone, Debug)]
struct PlanAi;
impl ModPlayerInputAi for PlanAi {
    fn clone_box(&self) -> Box<dyn ModPlayerInputAi> { Box::new(self.clone()) }
    fn id(&self) -> &str { "plan_reimpl_ai" }
    fn think(&mut self, ctx: &mut PlayerAiContext<'_,'_,'_>, base_input: Option<Input>) -> PlayerInputDecision {
        // ── Phase2: facet#5 데미지 재구현 검증 (HARNESS_ON일 때만; 재진입 게임함수 호출 = 실게임 위험) ──
        if HARNESS_ON && DMGCAP.load(Ordering::Relaxed) && VERIFY_N.load(Ordering::Relaxed) < 16 {
            unsafe {
                let exe = exe_base();
                let pb = CAP_PB.load(Ordering::Relaxed);
                let champs = champions(pb);
                if champs.len() >= 8 && exe != 0 {
                    let name = ctx.champion_name().to_string();
                    let nb = name.as_bytes();
                    let mt = ctx.team() as i64;
                    let mut me = 0usize;
                    let mut en = 0usize;
                    for &(t, e) in &champs {
                        if me == 0 && t == mt { if let Some(np) = rd_u64(e + E_NAME) { if str_eq_at(np as usize, nb) { me = e; } } }
                        if en == 0 && t != mt { en = e; }
                    }
                    if me != 0 && en != 0 && MIG_DMG {   // 데미지검증: combat(0x1be1e90)+ATK/TGT_VT(0x356ed28) 라이브확정(#1 TTD) → 비교활성
                        let v = VERIFY_N.fetch_add(1, Ordering::Relaxed);
                        if v < 16 {
                            let dtype = rd_i32(me + 0x4a4).unwrap_or(0) as u32;
                            let combat: CombatFn = core::mem::transmute(exe + RVA_COMBAT_FN);
                            let avt = exe + RVA_ATK_VT;
                            let tvt = exe + RVA_TGT_VT;
                            let base = 1000i64;
                            let g0 = combat(me, avt, en, tvt, base, dtype, 0);
                            let g1 = combat(me, avt, en, tvt, base, dtype, 1);
                            let m0 = my_combat_dmg(me, en, base, dtype, 0, exe);
                            let m1 = my_combat_dmg(me, en, base, dtype, 1, exe);
                            let cm = COEF_MULT_PCT;
                            let tag = |g: i64, m: i64| if cm == 100 { if g==m {"OK(검증)"} else {"MISMATCH"} } else { "override" };
                            let s = format!("[d#{}] {} dtype={} base={} coef_mult={}%\n  phys: game={} mine={} [{}]\n  magic: game={} mine={} [{}]\n",
                                v, name, dtype, base, cm, g0, m0, tag(g0,m0), g1, m1, tag(g1,m1));
                            if v == 0 { write_named("dmgcmp.txt", &s); } else { append_named("dmgcmp.txt", &s); }
                        }
                    }
                }
            }
        }

        // ★facet#2 오라클: base_input Move{x,y} 실측(타워/넥서스 직접좌표인지, 셀센터 중간노드인지). 첫 40개.
        {
            static BASE_LOG_N: AtomicU64 = AtomicU64::new(0);
            if let Some(Input::Move{x, y}) = &base_input {
                let n = BASE_LOG_N.load(Ordering::Relaxed);
                if n < 40 {
                    BASE_LOG_N.fetch_add(1, Ordering::Relaxed);
                    let s = format!("[base #{}] {} team={} -> Move({},{})\n", n, ctx.champion_name(), ctx.team(), *x, *y);
                    if n == 0 { write_named("baseinp.txt", &s); } else { append_named("baseinp.txt", &s); }
                }
            }
        }
        // ── override (cfg, 기본 OFF) — Phase2부터 재구현 결정으로 교체 ──
        if OV_ENABLED.load(Ordering::Relaxed) {
            let want = OV_TEAM.load(Ordering::Relaxed);
            if (want < 0 || ctx.team() as i64 == want) && matches!(base_input, Some(Input::Move{..})) {
                return PlayerInputDecision::Replace(Input::Move {
                    x: OV_X.load(Ordering::Relaxed), y: OV_Y.load(Ordering::Relaxed) });
            }
        }
        match base_input { Some(i)=>PlayerInputDecision::Replace(i), None=>PlayerInputDecision::Pass }
    }
}

struct CfgExt;
impl ModExtension for CfgExt {
    fn post_update(&self, _s: &mut Scene, _u: &mut GameUI, _a: &mut Assets, _dt: f32) {
        if !BOOTED.swap(true, Ordering::Relaxed) {
            append_log(&format!("[{}ms] [ext] post_update 가동. cfg 핫리로드 활성.\n", now_ms()));
        }
        IN_MENU.store(true, Ordering::Relaxed);   // 메뉴/모달 프레임 표시 → 다음 sim 첫 훅이 리셋 트리거
        // ★시드 회전: 메뉴 프레임에서만(AI 갭>60) practice replay seed 덮어씀 → 경기중엔 동결되어 CUR_SEED=sim 실제시드. 끄면 복원.
        let ai_gap = READY_TICKS.load(Ordering::Relaxed).wrapping_sub(LAST_AI_FRAME.load(Ordering::Relaxed));
        if SEED_ROTATE.load(Ordering::Relaxed) && ai_gap > 60 {
            if let Scene::InGame { data } = _s {
                let db = data.db();
                let r: &ClientDatabase = &*db;
                let n = SEED_ROT.fetch_add(1, Ordering::Relaxed);
                // ★균일 시드 v: SEED_SET 있으면 고정(재현), 없으면 회전(메뉴 프레임마다 변화=다양성). 모든 replay 동일 → sim 실제시드 = CUR_SEED = v.
                let ss = SEED_SET.load(Ordering::Relaxed);
                let v = if ss != 0 { ss } else { n.wrapping_mul(0x9E3779B97F4A7C15).wrapping_add(0x1234567) };
                CUR_SEED.store(v, Ordering::Relaxed);   // post_update=메뉴전용이라 sim중엔 freeze → sim 시드와 일치
                if let Ok(mut bak) = SEED_BAK.lock() {
                    for (mt, mi) in r.matches.iter() {
                        match mt { MatchType::Practice { .. } => {}, _ => continue };
                        for &k in mi.replays.iter() {
                            if let Some(rep) = r.match_replays.get(&k) {
                                let base = rep as *const _ as usize;
                                unsafe {
                                    if !readable(base + O_SEED_REPLAY, 8) { continue; }
                                    // 원본 1회 백업(off시 복원=세이브보호)
                                    if !bak.iter().any(|e| e.0 == base) {
                                        let s = std::ptr::read_unaligned((base + O_SEED_REPLAY) as *const u64);
                                        bak.push((base, s));
                                    }
                                    std::ptr::write_unaligned((base + O_SEED_REPLAY) as *mut u64, v);
                                }
                            }
                        }
                    }
                }
            }
        }
        // ★전술 회전: 메뉴 프레임에서 practice replay 24B 팀전술 무작위화(seed_rotate와 병행). 끄면 cfg에서 복원.
        let strat_set = STRAT_SET.lock().ok().and_then(|g| *g);
        if (STRAT_ROTATE.load(Ordering::Relaxed) || strat_set.is_some()) && ai_gap > 60 {
            if let Scene::InGame { data } = _s {
                let r: &ClientDatabase = &*data.db();
                let n = STRAT_ROT_N.fetch_add(1, Ordering::Relaxed);
                // 주입할 strat 12필드(blue/red): strat_set 있으면 고정 주입(code7 매치 재현), 없으면 회전.
                let (b, rd) = if let Some((sb, sr)) = strat_set {
                    (sb, sr)
                } else {
                    let mut b = [0u8; 12]; let mut rd = [0u8; 12];
                    for f in 0..12usize {
                        let bh = n.wrapping_mul(0x9E3779B97F4A7C15).wrapping_add((f as u64).wrapping_mul(0x100000001b3));
                        b[f]  = ((bh >> 23) % (STRAT_VC[f] as u64)) as u8;
                        rd[f] = ((bh.wrapping_add(0x1234567) >> 23) % (STRAT_VC[f] as u64)) as u8;
                    }
                    (b, rd)
                };
                if let Ok(mut sc) = STRAT_CUR.lock() { *sc = (b, rd); }
                if let Ok(mut bak) = STRAT_BAK.lock() {
                    for (mt, mi) in r.matches.iter() {
                        match mt { MatchType::Practice { .. } => {}, _ => continue };
                        for &k in mi.replays.iter() {
                            if let Some(rep) = r.match_replays.get(&k) {
                                let base = rep as *const _ as usize;
                                unsafe {
                                    if !readable(base + O_RED_STRAT + 24, 1) { continue; }
                                    if !bak.iter().any(|e| e.0 == base) {
                                        let mut bb = [0u8; 24]; let mut rr = [0u8; 24];
                                        for i in 0..24 { bb[i] = rd_u8(base + O_BLUE_STRAT + i); rr[i] = rd_u8(base + O_RED_STRAT + i); }
                                        bak.push((base, bb, rr));
                                    }
                                    for f in 0..12usize {
                                        std::ptr::write_unaligned((base + O_BLUE_STRAT + STRAT_OFFS_ROT[f]) as *mut u8, b[f]);
                                        std::ptr::write_unaligned((base + O_RED_STRAT + STRAT_OFFS_ROT[f]) as *mut u8, rd[f]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // ── ready 틱 증가(로딩중 게임함수 호출 방지) + 상태 주기 기록 ──
        READY_TICKS.fetch_add(1, Ordering::Relaxed);
        {
            static STATUS_CTR: AtomicU64 = AtomicU64::new(0);
            if STATUS_CTR.fetch_add(1, Ordering::Relaxed) % 30 == 0 {
                // tag별 struct머리 9 qword 덤프. 좌표(대략 16000~700000)가 있는 오프셋이 Move의 x/y.
                let mut tags = String::new();
                for t in 0..16usize {
                    let c = TAG_COUNTS[t].load(Ordering::Relaxed);
                    if c > 0 {
                        tags.push_str(&format!("  tag{}(cnt={}): ", t, c));
                        for k in 0..18usize { let v=TAG_SAMP[t][k].load(Ordering::Relaxed); if v!=0 { tags.push_str(&format!("+{:#x}={} ", k*8, v)); } }
                        tags.push('\n');
                    }
                }
                // 광범위 커밋(FUN_141a49fa0) tag별 첫샘플: 매프레임 최종 Input. 월드좌표(16000~960000)가 어디 있나.
                let mut ctags = String::new();
                for t in 0..16usize {
                    let c = COMMIT_TAGCOUNT[t].load(Ordering::Relaxed);
                    if c > 0 {
                        ctags.push_str(&format!("  ctag{}(cnt={}): ", t, c));
                        for k in 0..18usize { let v=COMMIT_SAMP[t][k].load(Ordering::Relaxed); if v!=0 { ctags.push_str(&format!("+{:#x}={} ", k*8, v)); } }
                        ctags.push('\n');
                    }
                }
                // 페이즈 게이트 베이스 imm8 readback(패치 확인)
                let gate_imm: i64 = unsafe {
                    let mb = exe_base();
                    if mb != 0 && readable(mb + RVA_ENGAGE_GATE + 2, 1) { std::ptr::read_unaligned((mb + RVA_ENGAGE_GATE + 2) as *const u8) as i64 } else { -1 }
                };
                // ★역할 교전임계값 4 imm8 라이브 readback(engage_thr_mult 패치 확인). orig→live.
                let thr_live: String = unsafe {
                    let mb = exe_base();
                    ROLE_THR.iter().map(|&(rva, orig)| {
                        let b = if mb != 0 && readable(mb + rva, 1) { std::ptr::read_unaligned((mb + rva) as *const u8) as i64 } else { -1 };
                        format!("{}→{}", orig, b)
                    }).collect::<Vec<_>>().join(",")
                };
                let s = format!("move={} move_tag={} move_off={:#x} move_x={} move_y={} MOVE_HANDLED={} | engage_base(cfg)={} gate_imm8={} | engage_thr_mult(cfg)={} ROLE_THR[{}]\nengage_repl(entry): on={} N={} pass={} (PT gate={} count={} other={}) | rng_repl(roll): on={} N={}\nfc59a0[recall]: raw={} arm={} filt={} recallcap={}\ncand[filter]: raw={} arm={} candcap={} | genbuild[scorer]: raw={} arm={} gbcap={} term={:#x} | genbuild[body]: raw={} arm={} gbbody={} | gb[callee]: 203raw={} 690raw={} armed={} badptr={} panic={} gbcallee={} 203(OK={} DIFF={}) 690(OK={} DIFF={})\ngb[region_d]: raw={} armed={} badptr={} panic={} gbrd={} (OK={} DIFF={} NP={} Dvpush={}) | gbrepl={} replaced={} chk(M={} X={}) gbskip={} skipped={}\nInput tag별 첫샘플(머리 9 qword; 좌표같은 값 있는 오프셋이 Move의 x/y):\n{}replace={} repl_handled={} ready_ticks={}\n=== 광범위 커밋(FUN_141a49fa0, 매프레임 최종Input) total={} ===\n{}",
                    MOVE_ON.load(Ordering::Relaxed) as u8, MOVE_TAG.load(Ordering::Relaxed), MOVE_OFF.load(Ordering::Relaxed), MOVE_X.load(Ordering::Relaxed), MOVE_Y.load(Ordering::Relaxed), MOVE_HANDLED.load(Ordering::Relaxed),
                    ENGAGE_BASE.load(Ordering::Relaxed), gate_imm, ENGAGE_THR_MULT.load(Ordering::Relaxed), thr_live,
                    ENGAGE_REPL.load(Ordering::Relaxed) as u8, ENGAGE_REPL_N.load(Ordering::Relaxed), ENGAGE_REPL_PASS.load(Ordering::Relaxed), PT_GATE.load(Ordering::Relaxed), PT_COUNT.load(Ordering::Relaxed), PT_OTHER.load(Ordering::Relaxed), RNG_REPL.load(Ordering::Relaxed) as u8, RNG_REPL_N.load(Ordering::Relaxed),
                    FC59_RAW.load(Ordering::Relaxed), FC59_ARM.load(Ordering::Relaxed), FC59_FILT.load(Ordering::Relaxed), RECALLCAP.load(Ordering::Relaxed) as u8,
                    CAND_RAW.load(Ordering::Relaxed), CAND_ARMED.load(Ordering::Relaxed), CANDCAP.load(Ordering::Relaxed) as u8,
                    GB_RAW.load(Ordering::Relaxed), GB_ARMED.load(Ordering::Relaxed), GBCAP.load(Ordering::Relaxed) as u8, GB_TERM.load(Ordering::Relaxed),
                    GBB_RAW.load(Ordering::Relaxed), GBB_ARMED.load(Ordering::Relaxed), GBBODY.load(Ordering::Relaxed) as u8,
                    GBC203_RAW.load(Ordering::Relaxed), GBC690_RAW.load(Ordering::Relaxed), GBC_ARMED.load(Ordering::Relaxed), GBC_BADPTR.load(Ordering::Relaxed), GBC_PANIC.load(Ordering::Relaxed), GBCALLEE.load(Ordering::Relaxed) as u8, GBC203_OK.load(Ordering::Relaxed), GBC203_DIFF.load(Ordering::Relaxed), GBC690_OK.load(Ordering::Relaxed), GBC690_DIFF.load(Ordering::Relaxed),
                    GBRD_RAW.load(Ordering::Relaxed), GBRD_ARMED.load(Ordering::Relaxed), GBRD_BADPTR.load(Ordering::Relaxed), GBRD_PANIC.load(Ordering::Relaxed), GBRD.load(Ordering::Relaxed) as u8, GBRD_OK.load(Ordering::Relaxed), GBRD_DIFF.load(Ordering::Relaxed), GBRD_NP.load(Ordering::Relaxed), GBRD_VPUSH.load(Ordering::Relaxed),
                    GBREPL.load(Ordering::Relaxed) as u8, GBREPL_N.load(Ordering::Relaxed), GBREPL_MATCH.load(Ordering::Relaxed), GBREPL_MISMATCH.load(Ordering::Relaxed), GBSKIP.load(Ordering::Relaxed) as u8, GBSKIP_N.load(Ordering::Relaxed),
                    tags, REPL_ON.load(Ordering::Relaxed) as u8, REPL_HANDLED.load(Ordering::Relaxed), READY_TICKS.load(Ordering::Relaxed),
                    COMMIT_TOTAL.load(Ordering::Relaxed), ctags);
                let s = format!("{}call_ablate: cfg={} applied={} blocked(콜0xb 발화·차단) A={} B={} 합계={}\n", s, CALL_ABLATE.load(Ordering::Relaxed) as u8, CALL_ABLATE_APPLIED.load(Ordering::Relaxed) as u8, CALL_BLOCKED_A.load(Ordering::Relaxed), CALL_BLOCKED_B.load(Ordering::Relaxed), CALL_BLOCKED_A.load(Ordering::Relaxed)+CALL_BLOCKED_B.load(Ordering::Relaxed));
                let s = format!("{}lane_gate: cfg={} applied={} (0=원본/1=후보0개/2=후보다)\n", s, LANE_GATE.load(Ordering::Relaxed), LANE_GATE_APPLIED.load(Ordering::Relaxed));
                let s = format!("{}type3_ablate: cfg={} applied={} (transition 타입3콜 차단)\n", s, TYPE3_ABLATE.load(Ordering::Relaxed) as u8, TYPE3_APPLIED.load(Ordering::Relaxed) as u8);
                write_named("repl_status.txt", &s);
            }
        }
        // plan_base 자동탐지 (1회, 메인스레드 = 안전)
        if !DIAG_DONE.load(Ordering::Relaxed) { unsafe { try_find_plan_base(); } }
        // ★facet#2 FUN_141dd7700 덤프: param체인 + 후보 waypoint + STEP5 목표선택 repro (N>300후 1회, repro N==250 후)
        if !DD7700_DUMP.load(Ordering::Relaxed) && DD7700_N.load(Ordering::Relaxed) > 300 {
            unsafe {
                DD7700_DUMP.store(true, Ordering::Relaxed);
                let p2 = DD7700_P2.load(Ordering::Relaxed); let p5 = DD7700_P5.load(Ordering::Relaxed);
                let p6 = DD7700_P6.load(Ordering::Relaxed); let p7 = DD7700_P7.load(Ordering::Relaxed);
                let rb = |a: usize| if readable(a,1) { std::ptr::read_unaligned(a as *const u8) as i64 } else { -1 };
                let name = cstr(rd_u64(p7+0x250).unwrap_or(0) as usize);
                let plan_byte = rb(p7+0x3e6);
                let side = rd_i64(p5+0x6a8).unwrap_or(-1);
                let lane_kind = rd_i32(p5+0x738).unwrap_or(-1);
                let f = rb(p2+0x19);
                let l80 = rd_u64(p6).unwrap_or(0) as usize; let vobj = rd_u64(p6+8).unwrap_or(0) as usize; let geo = rd_u64(p6+0x10).unwrap_or(0) as usize;
                let sidx = if side>=0 {side as usize} else {0}; let fidx = if f>=0 {f as usize} else {0};
                let cbase = l80 + sidx*8 + fidx*32;
                let near = rd_u64(cbase + 0x180).unwrap_or(0) as usize;
                let far  = rd_u64(cbase + 0x190).unwrap_or(0) as usize;
                let endp = rd_u64(l80 + sidx*8 + 0x170).unwrap_or(0) as usize;
                let co = |e: usize| if ptr_ok(e) && readable(e+0x658,8) { format!("0x{:x} pos=({},{}) hp={}/{} name=\"{}\"", e, rd_i64(e+0x648).unwrap_or(-1), rd_i64(e+0x650).unwrap_or(-1), rd_i64(e+0x658).unwrap_or(-1), rd_i64(e+0x610).unwrap_or(-1), cstr(rd_u64(e+0x250).unwrap_or(0) as usize)) } else { format!("0x{:x} (non-entity)", e) };
                // STEP5 목표선택 repro 결과(hook live N==250)
                let athlete_name = cstr(rd_u64(p7+0x398).unwrap_or(0) as usize);  // p7=athlete, 이름은 +0x398
                let s = format!("[{}ms] === FUN_141dd7700 param캡처 (N={}) ===\nchamp(p7)=0x{:x} name(+0x250)=\"{}\" athlete_name(+0x398)=\"{}\" plan_byte={}\nside(p5+0x6a8)={} lane_kind(p5+0x738)={} F(p2+0x19)={}\nL80=*p6=0x{:x} VOBJ=p6[1]=0x{:x} GEO=p6[2]=0x{:x}\n후보 waypoint(side={} F={}):\n  near(L80+s*8+F*32+0x180): {}\n  far(+0x190): {}\n  endpoint(L80+s*8+0x170): {}\n=== ★STEP5 목표선택 repro (live) ===\niv12(*piVar14)={} target=({},{}) d_self={} d_near={}\n  → 예측목표 kind={}(1=nexus/2=near) pos=({},{})\n",
                    now_ms(), DD7700_N.load(Ordering::Relaxed), p7, name, athlete_name, plan_byte, side, lane_kind, f, l80, vobj, geo, side, f, co(near), co(far), co(endp),
                    DD7700_R_IV12.load(Ordering::Relaxed), DD7700_R_TX.load(Ordering::Relaxed), DD7700_R_TY.load(Ordering::Relaxed), DD7700_R_DSELF.load(Ordering::Relaxed), DD7700_R_DNEAR.load(Ordering::Relaxed),
                    DD7700_R_GOALKIND.load(Ordering::Relaxed), DD7700_R_GOALX.load(Ordering::Relaxed), DD7700_R_GOALY.load(Ordering::Relaxed));
                write_named("dd7700.txt", &s);
                append_log(&format!("[{}ms] ★dd7700.txt (facet#2 param캡처) | tail deep(6/7후보) 케이스={}\n", now_ms(), DD7_DEEP.load(Ordering::Relaxed)));
            }
        }
        // ★dd7700 callee 식별: L80[1] vtable 슬롯 함수 + 정적 테이블 값 → dd7callees.txt (RVA로, 디컴파일용)
        if !DD7_CALLEE_DUMP.load(Ordering::Relaxed) && DD7_VT.load(Ordering::Relaxed) != 0 {
            unsafe {
                DD7_CALLEE_DUMP.store(true, Ordering::Relaxed);
                let base = exe_base() as usize;
                let rva = |a: usize| if a >= base && base != 0 { (a - base) as i64 } else { -1 };
                let vt = DD7_VT.load(Ordering::Relaxed);
                let rdt = |r: usize, i: usize| if base!=0 { rd_i64(base + r + i*8).unwrap_or(-1) } else { -1 };
                let mut s = format!("=== dd7700 callee 식별 (RVA = addr - module_base) ===\nL80[1] vtable @ RVA {:#x} (abs 0x{:x})\n", rva(vt), vt);
                for (name, a) in [("+0x20", DD7_S20.load(Ordering::Relaxed)), ("+0x48", DD7_S48.load(Ordering::Relaxed)),
                    ("+0xa8", DD7_SA8.load(Ordering::Relaxed)), ("+0x128(candidate-resolve)", DD7_S128.load(Ordering::Relaxed)),
                    ("+0x140(handle→entity)", DD7_S140.load(Ordering::Relaxed)), ("+0x168", DD7_S168.load(Ordering::Relaxed))] {
                    s.push_str(&format!("  slot {} → FUN @ RVA {:#x} (abs 0x{:x})\n", name, rva(a), a));
                }
                s.push_str("=== 정적 테이블 (F=0,1,2) ===\n");
                s.push_str(&format!("DAT_14356d930[0..3] = {} {} {}\n", rdt(0x356d930,0), rdt(0x356d930,1), rdt(0x356d930,2)));
                s.push_str(&format!("DAT_14356d948[0..3] = {} {} {}\n", rdt(0x356d948,0), rdt(0x356d948,1), rdt(0x356d948,2)));
                s.push_str(&format!("DAT_143544e00[0..3] = {} {} {}\n", rdt(0x3544e00,0), rdt(0x3544e00,1), rdt(0x3544e00,2)));
                s.push_str(&format!("DAT_143544e18[0..3] = {} {} {}\n", rdt(0x3544e18,0), rdt(0x3544e18,1), rdt(0x3544e18,2)));
                s.push_str(&format!("DAT_143539308 grid[row0 c0..4] = {} {} {} {} {}\n", rdt(0x3539308,0), rdt(0x3539308,1), rdt(0x3539308,2), rdt(0x3539308,3), rdt(0x3539308,4)));
                write_named("dd7callees.txt", &s);
                append_log(&format!("[{}ms] ★dd7callees.txt (vtable슬롯 RVA + 테이블)\n", now_ms()));
            }
        }
        // FUN_141dd9360: athlete(r9)에 champion 이름이 있나 + AI구조체(rdx)+0x1870 subplan (1회)
        if !T9_DONE.load(Ordering::Relaxed) {
            unsafe {
                let rdx = CAP_T9_RDX.load(Ordering::Relaxed);
                let r9 = CAP_T9_R9.load(Ordering::Relaxed);
                if rdx != 0 && r9 != 0 {
                    T9_DONE.store(true, Ordering::Relaxed);
                    let sub1 = rd_i64(rdx + 0x1870).unwrap_or(-99); // plan_state+0x500 = param_2+0x1870
                    let sub2 = rd_i64(rdx + 0x1378 + 0x500).unwrap_or(-99); // 대체 후보
                    let mut s = format!("[{}ms] === FUN_141dd9360: AI구조체/athlete ===\nrdx(AI)=0x{:x} r9(athlete)=0x{:x}\n  rdx+0x1870={} rdx+0x1878={}\n",
                        now_ms(), rdx, r9, sub1, sub2);
                    // athlete(r9) 0..0x800 스캔: 이름 문자열(char*) / entity 링크
                    s.push_str("athlete 스캔:\n");
                    let mut o = 0usize;
                    while o < 0x800 {
                        if let Some(p) = rd_u64(r9+o) {
                            let pu = p as usize;
                            // char* → 소문자 문자열?
                            let cs = cstr(pu);
                            if cs.len()>=3 && cs.len()<=24 && cs!="?" && cs.bytes().all(|b| b.is_ascii_lowercase()||b==b'_'||b.is_ascii_digit()) {
                                s.push_str(&format!("  r9+0x{:x} → \"{}\"\n", o, cs));
                            }
                            if is_champion(pu) {
                                s.push_str(&format!("  r9+0x{:x} → ENTITY name=\"{}\"\n", o, cstr(rd_u64(pu+E_NAME).unwrap_or(0) as usize)));
                            }
                        }
                        o += 8;
                    }
                    // athlete 자체 +0x250(엔티티와 같은 위치)도 직접 확인
                    s.push_str(&format!("  r9+0x250 직접→ \"{}\"\n", cstr(rd_u64(r9+0x250).unwrap_or(0) as usize)));
                    write_named("ai3.txt", &s);
                    append_log(&format!("[{}ms] ★ai3.txt (athlete 스캔)\n", now_ms()));
                }
            }
        }
        // AI구조체(param_2 = plan_state - 0x1370) 스캔: entity 포인터 링크 찾기 (1회)
        if DIAG_DONE.load(Ordering::Relaxed) && !AISCAN_DONE.load(Ordering::Relaxed) {
            unsafe {
                let ps = CAP_PSTATE.load(Ordering::Relaxed);
                if ps != 0 {
                    AISCAN_DONE.store(true, Ordering::Relaxed);
                    let subplan = rd_i64(ps + 0x500).unwrap_or(-1);
                    let p2 = ps.wrapping_sub(0x1370); // param_2 추정 시작
                    let mut s = format!("[{}ms] === AI구조체 스캔 ===\nplan_state=0x{:x} subplan(+0x500)={} param_2추정=0x{:x}\n",
                        now_ms(), ps, subplan, p2);
                    // ps 기준 [-0x1400, +0x2800] 범위에서 entity 포인터 / 이름링크 찾기
                    let lo = ps.wrapping_sub(0x1400);
                    let mut o = 0usize;
                    while o < 0x3c00 {
                        let a = lo + o;
                        if let Some(p) = rd_u64(a) {
                            let pu = p as usize;
                            if is_champion(pu) {
                                let rel = a as i64 - ps as i64;
                                s.push_str(&format!("  ps{:+#x} (param_2+0x{:x}) → ENTITY 0x{:x} name=\"{}\" team={}\n",
                                    rel, a.wrapping_sub(p2), pu, cstr(rd_u64(pu+E_NAME).unwrap_or(0) as usize), rd_i64(pu+0x8).unwrap_or(-1)));
                            }
                        }
                        o += 8;
                    }
                    write_named("ai2.txt", &s);
                    append_log(&format!("[{}ms] ★ai2.txt (AI구조체 스캔)\n", now_ms()));
                }
            }
        }
        // plan_state set 덤프: 수집된 plan_state들 정렬+stride+subplan (배열구조인지 확인) (1회, ≥8개 모이면)
        if !PSTATES_DUMP_DONE.load(Ordering::Relaxed) && PSTATE_CNT.load(Ordering::Relaxed) >= 8 {
            unsafe {
                PSTATES_DUMP_DONE.store(true, Ordering::Relaxed);
                let mut v: Vec<usize> = (0..16).map(|i| PSTATE_SET[i].load(Ordering::Relaxed)).filter(|&a| a!=0).collect();
                v.sort();
                let mut s = format!("[{}ms] === 수집 plan_state {}개 (정렬) ===\n", now_ms(), v.len());
                for (k, &a) in v.iter().enumerate() {
                    let sub = rd_i64(a+0x500).unwrap_or(-1);
                    let stride = if k>0 { a as i64 - v[k-1] as i64 } else { 0 };
                    s.push_str(&format!("  [{:2}] 0x{:x}  subplan={:<2}  Δ=0x{:x}\n", k, a, sub, stride));
                }
                write_named("pstates.txt", &s);
                append_log(&format!("[{}ms] ★pstates.txt ({}개)\n", now_ms(), v.len()));
            }
        }
        // driver 인자 진단: 어느 인자가 entity / plan_state 인가 (1회)
        if false && DIAG_DONE.load(Ordering::Relaxed) && !DRVDUMP_DONE.load(Ordering::Relaxed) {
            unsafe {
                let rcx = CAP_DRV_RCX.load(Ordering::Relaxed);
                let rdx = CAP_DRV_RDX.load(Ordering::Relaxed);
                let r8  = CAP_DRV_R8.load(Ordering::Relaxed);
                let r9  = CAP_DRV_R9.load(Ordering::Relaxed);
                let a5  = CAP_DRV_A5.load(Ordering::Relaxed);
                if rcx != 0 || rdx != 0 {
                    DRVDUMP_DONE.store(true, Ordering::Relaxed);
                    let mut s = format!("[{}ms] === driver 인자 진단 ===\nrcx=0x{:x} rdx=0x{:x} r8=0x{:x} r9=0x{:x} arg5=0x{:x}\n",
                        now_ms(), rcx, rdx, r8, r9, a5);
                    for (nm, v) in [("rcx",rcx),("rdx",rdx),("r8",r8),("r9",r9),("arg5",a5)] {
                        if !ptr_ok(v) { s.push_str(&format!("  {} = 0x{:x} (non-ptr)\n", nm, v)); continue; }
                        if is_champion(v) {
                            s.push_str(&format!("  {} = ENTITY 0x{:x} name=\"{}\" team={}\n", nm, v,
                                cstr(rd_u64(v+E_NAME).unwrap_or(0) as usize), rd_i64(v+0x8).unwrap_or(-1)));
                        }
                        let sp = rd_i64(v+0x500).unwrap_or(-99);
                        if sp >= 2 && sp <= 14 { s.push_str(&format!("  {} = PLAN_STATE 0x{:x} (+0x500={})\n", nm, v, sp)); }
                        // 깊은 스캔(0..0x400): 엔티티 직접 / 1홉 / 이름링크
                        let mut o = 0usize;
                        while o < 0x400 {
                            if let Some(p) = rd_u64(v+o) {
                                let pu = p as usize;
                                if is_champion(pu) {
                                    s.push_str(&format!("  {}+0x{:x} → ENTITY 0x{:x} name=\"{}\" team={}\n", nm, o, pu,
                                        cstr(rd_u64(pu+E_NAME).unwrap_or(0) as usize), rd_i64(pu+0x8).unwrap_or(-1)));
                                } else if ptr_ok(pu) {
                                    // 1홉: *(p+o2)가 엔티티?
                                    for o2 in [0x0usize,0x8,0x10,0x18,0x20,0x28,0x30,0x38,0x40] {
                                        if let Some(q) = rd_u64(pu+o2) {
                                            if is_champion(q as usize) {
                                                s.push_str(&format!("  {}+0x{:x}→+0x{:x} → ENTITY 0x{:x} name=\"{}\"\n",
                                                    nm, o, o2, q, cstr(rd_u64((q as usize)+E_NAME).unwrap_or(0) as usize)));
                                            }
                                        }
                                    }
                                }
                            }
                            o += 8;
                        }
                    }
                    write_named("drvdump.txt", &s);
                    append_log(&format!("[{}ms] ★drvdump.txt 작성\n", now_ms()));
                }
            }
        }
        // dispatch 레지스터 진단: 어느 reg가 self 엔티티인가 (entity→plan_state 맵 위해) (1회)
        if DIAG_DONE.load(Ordering::Relaxed) && !DISPREG_DONE.load(Ordering::Relaxed) {
            unsafe {
                let rcx = CAP_RCX.load(Ordering::Relaxed);
                let r8  = CAP_R8.load(Ordering::Relaxed);
                let r9  = CAP_R9.load(Ordering::Relaxed);
                let rdx = CAP_RDX.load(Ordering::Relaxed);
                if rdx != 0 {
                    DISPREG_DONE.store(true, Ordering::Relaxed);
                    let pstate = CAP_PSTATE.load(Ordering::Relaxed);
                    let subplan = rd_i64(rdx).unwrap_or(-1);
                    let mut s = format!("[{}ms] === dispatch reg 진단 ===\nrcx=0x{:x} r8=0x{:x} r9=0x{:x} rdx=0x{:x} pstate=0x{:x} subplan(*rdx)={}\n",
                        now_ms(), rcx, r8, r9, rdx, pstate, subplan);
                    for (nm, v) in [("rcx",rcx),("r8",r8),("r9",r9)] {
                        // 직접 엔티티?
                        if is_champion(v) {
                            s.push_str(&format!("  {} = ENTITY 0x{:x} name=\"{}\" team={} hp={}/{}\n", nm, v,
                                cstr(rd_u64(v+E_NAME).unwrap_or(0) as usize), rd_i64(v+0x8).unwrap_or(-1),
                                rd_i64(v+E_HP).unwrap_or(-1), rd_i64(v+E_MAXHP).unwrap_or(-1)));
                        } else if ptr_ok(v) {
                            // 얕은 deref가 엔티티?
                            for o in [0x0usize,0x8,0x10,0x18,0x20,0x28,0x30,0x38,0x40] {
                                if let Some(p) = rd_u64(v+o) {
                                    if is_champion(p as usize) {
                                        s.push_str(&format!("  {}+0x{:x} → ENTITY 0x{:x} name=\"{}\"\n", nm, o, p,
                                            cstr(rd_u64((p as usize)+E_NAME).unwrap_or(0) as usize)));
                                    }
                                }
                            }
                        }
                    }
                    // plan_state 전체 덤프 (0..0x540): 각 qword를 포인터/문자열/챔프/좌표로 주석
                    if ptr_ok(pstate) {
                        s.push_str("--- plan_state dump 0..0x540 (i32쌍=좌표/플래그 후보) ---\n");
                        let mut o = 0usize;
                        while o < 0x540 {
                            if let Some(v) = rd_u64(pstate+o) {
                                let lo = (v & 0xffffffff) as i32;
                                let hi = (v >> 32) as i32;
                                let vu = v as usize;
                                let mut ann = String::new();
                                if ptr_ok(vu) && readable(vu, 1) {
                                    let cs = cstr(vu);
                                    if cs.len() >= 2 && cs != "?" && cs.bytes().all(|b| b.is_ascii_graphic()) {
                                        ann.push_str(&format!(" str=\"{}\"", cs));
                                    }
                                    if is_champion(vu) {
                                        ann.push_str(&format!(" CHAMP=\"{}\"", cstr(rd_u64(vu+E_NAME).unwrap_or(0) as usize)));
                                    }
                                    // vu가 가리키는 곳의 +0x250이 이름이면(=엔티티성)
                                    if let Some(np) = rd_u64(vu+E_NAME) {
                                        let nm = cstr(np as usize);
                                        if nm.len()>=2 && nm!="?" && nm.bytes().all(|b| b.is_ascii_lowercase()||b==b'_') {
                                            ann.push_str(&format!(" [+0x250→\"{}\"]", nm));
                                        }
                                    }
                                }
                                let mark = if o==0x500 {" <SUBPLAN"} else {""};
                                s.push_str(&format!("  +0x{:03x}: 0x{:016x}  [{:>10},{:>10}]{}{}\n", o, v, lo, hi, mark, ann));
                            }
                            o += 8;
                        }
                    }
                    write_named("dispreg.txt", &s);
                    append_log(&format!("[{}ms] ★dispreg.txt 작성\n", now_ms()));
                }
            }
        }
        if load_cfg(false) {
            // (dmgcmp 재측정 트리거 제거 — 하드코딩 테스트라 1회만)
            append_log(&format!("[{}ms] ↻ cfg: enabled={} team={} x={} y={} coef_mult={}%\n", now_ms(),
                OV_ENABLED.load(Ordering::Relaxed) as u8, OV_TEAM.load(Ordering::Relaxed),
                OV_X.load(Ordering::Relaxed), OV_Y.load(Ordering::Relaxed), OV_COEF_MULT.load(Ordering::Relaxed)));
        }
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    fresh_log(&format!("[{}ms] === plan_reimpl Phase1 INIT (월드접근 검증) ===\n", now_ms()));
    unsafe {
        seh_install();   // ★VEH 안전읽기 핸들러 등록(fast_read 경로용; off여도 무해=우리 폴트범위만 처리)
        // ★레버3: ChaCha12 SIMD self-test → 스칼라와 비트동일일 때만 활성(아니면 스칼라 fallback). 어떤 머신서도 안전.
        let simd_ok = chacha_simd_selftest();
        USE_SIMD_CHACHA.store(simd_ok, Ordering::Relaxed);
        append_log(&format!("[perf] chacha SIMD self-test: {}\n", if simd_ok {"PASS -> SIMD ON"} else {"FAIL -> scalar fallback"}));
        build_shim_rdx();
        build_shim_both();   // ★소환수 비멱등 게터용 단일호출 2값캡처
        // build_pregate_shim 제거: my_pregate(순수Rust)로 대체
        if HARNESS_ON { build_ret_thunk(); }  // 공용 리턴 thunk (TTD+RE 둘 다 사용; 훅설치 前)
        // ★3차 retreat replace 분리활성(2026-06-18): retreat_engage(0x1fcfda0) 프롤로그 8push=12B 경계OK·rip-rel無 검증, args(rcx=out/rdx=p2(+0x48읽음)/r9=self) 3차서도 동일(리팩터는 뒷부분만). 기본 replace=0/capture=0이면 retreat_capture 즉시 return1=inert라 안전. 콜리 lane_pred(0x1fe2b60)/roster vt 3차갱신.
        match install_replace_detour(RVA_RETREAT, 12, retreat_capture as *const () as usize) {
            Ok(())=>append_log("[hook] retreat_engage replace(0x1fcfda0,12B) OK\n"),
            Err(e)=>append_log(&format!("[hook] retreat 실패: {}\n", e)),
        }
        // ★3차 commit 마이그완료(드라이버 +0x590 콜 스캔): COMMIT_CALL 0x1b6ec93 / COMMIT_FN 0x1cbc9f0. sanity가드(target≠COMMIT_FN→Err)+commit_dump 관측전용 → 분리활성.
        match install_commit_hook() {
            Ok(())=>append_log("[hook] commit(commit_fn @0x1b6ec93) OK\n"),
            Err(e)=>append_log(&format!("[hook] commit 실패: {}\n", e)),
        }
        // ★3차 B2 generic_build: move-post 훅 분리활성(retreat/commit과 분리). F2_BUILD_CALL(0x1b6e806)+generic_build(0x1bf5980) 3차갱신, 콜사이트 8인자(4reg+4stack@rsp+0x20~38)·rcx=outptr ABI확인. move_override 기본 read-only(MOVE_ON off=캡처만). target sanity가드 자체보호 → 안전.
        match install_move_post_hook() {
            Ok(())=>append_log("[hook] move-post(generic_build @0x1b6e806, 8arg) OK\n"),
            Err(e)=>append_log(&format!("[hook] move-post 실패: {}\n", e)),
        }
        let _ = install_move_hook as *const ();
        let _ = (install_replace_detour as *const (), install_move_post_hook as *const (), install_commit_hook as *const ());
        if HARNESS_ON {
            // ★TTD(0x1b6df40) — 프롤로그 install_detour-안전(8push=12B, rip-rel無, 핫픽스=프리핫픽스 동일). MIG_TTD로 선택활성(my_ttd 재검증).
            if MIG_TTD {
            if RET_THUNK.load(Ordering::Relaxed) == 0 {
                append_log("[hook] ret thunk alloc 실패 — TTD 훅 미설치\n");
            } else {
                match install_detour(RVA_TTD, 12, ttd_capture as *const () as usize) {
                    Ok(())=>append_log("[hook] plan_score_survival_ttd(0x1e1c7c0,12B) OK — TTD 리턴훅\n"),
                    Err(e)=>append_log(&format!("[hook] ttd 실패: {}\n", e)),
                }
            }
            } // end if MIG_TTD
            // ★df0c10 콜사이트 — 0.4.13_5 재활성 시도했으나 리플레이 재생중 크래시(2026-06-19) → 격리 위해 다시 보류.
            //   콜사이트 RVA(0x206fa33→df0c10 0x20e88a0)·ROLL_RET·FCD980은 Ghidra 검증완료. 투명래퍼 자체에 문제 추정(arg복제 or df0c10 새시그).
            //   dispatch(3/7/8) 검증은 df0c10 불필요(args기반)라 이 훅 없이 capture=1로 진행. roll(5/-1)은 래퍼 안전화 후 별도.
            if !MIG_CHANGED {
            match install_df0c10_hook() {
                Ok(())=>append_log("[hook] df0c10 셀렉터포착(@0x206fa33) OK\n"),
                Err(e)=>append_log(&format!("[hook] df0c10 실패: {}\n", e)),
            }
            } // end if !MIG_CHANGED (df0c10 콜사이트, 크래시 격리중)
            // ★unchanged 캡처훅(0.4.13 RVA 갱신완료) — MIG0413=false로 활성
            if !MIG0413 {
            // facet#2 FUN_141dd7700 param 캡처(1단계: param체인·후보좌표 덤프)
            // ★0.4.13_5: DD7700_CAP_OK=false로 설치 차단(my_dd7700_code 재현 stale+과부하 크래시). 재유도 후 해제.
            if DD7700_CAP_OK && INSTALL_DIAG_HOOKS {
            match install_detour(RVA_DD7700, 12, dd7700_capture as *const () as usize) {
                Ok(())=>append_log("[hook] dd7700 param캡처(@0x19e5e10, 12B) OK\n"),
                Err(e)=>append_log(&format!("[hook] dd7700 실패: {}\n", e)),
            }
            } else { append_log("[hook] dd7700 캡처 SKIP (DD7700_CAP_OK=false, 0.4.13_5 재현 stale)\n"); }
            // ★PRNG gen_range 검증(fcd980, 프롤로그 13B = 6push+sub rsp,0x28)
            // ★replace-detour(rax): rng_repl=0이면 cap_fn이 SENT 반환→passthrough(install_detour와 동일). rng_repl=1이면 교전롤 대체.
            if INSTALL_DIAG_HOOKS {
            match install_replace_detour_rax(RVA_FCD980, 13, fcd980_capture as *const () as usize) {
                Ok(())=>append_log("[hook] fcd980 gen_range(@0x189ae20, 13B, replace-rax) OK\n"),
                Err(e)=>append_log(&format!("[hook] fcd980 실패: {}\n", e)),
            }
            }   // ★INSTALL_DIAG_HOOKS=false면 미설치(RNG 자급자족, rng_repl=0)
            // ★드라이버 페이즈게이트(fcdaf0 gen_range[0,1000], 13B). 커스텀 스텁(rbx/rdi/rsi=A/B/C 저장). cfg pgcap=1일때만 작동.
            if INSTALL_DIAG_HOOKS {
            match install_detour_pg(RVA_FCDAF0, 13, fcdaf0_pg_capture as *const () as usize) {
                Ok(())=>append_log("[hook] fcdaf0 phase-gate(@0x18a1da0, 13B, pg-stub) OK\n"),
                Err(e)=>append_log(&format!("[hook] fcdaf0 실패: {}\n", e)),
            }
            }   // ★미설치시 pgcap=0 passthrough였음(비트동일)
            // ★ChaCha12 refill 검증(1421bbc10, 프롤로그 12B = 4push+sub rsp,0x168)
            if INSTALL_DIAG_HOOKS {
            match install_detour(RVA_CHACHA, 12, chacha_capture as *const () as usize) {
                Ok(())=>append_log("[hook] chacha refill(@0x2220f70, 12B) OK\n"),
                Err(e)=>append_log(&format!("[hook] chacha 실패: {}\n", e)),
            }
            }   // ★미설치시 rngcap=0 passthrough였음. 모든 refill 트램폴린 제거(최대효과)
            // ★subplan_transition_engine 입력캡처(0x1d45290, 12B=push8). cfg tecap=1일때만 로깅. phase분포·후보덤프.
            if INSTALL_DIAG_HOOKS {
            match install_detour(RVA_TRANS, 12, trans_capture as *const () as usize) {
                Ok(())=>append_log("[hook] transition_engine 입력캡처(@0x1b64db0, 12B) OK\n"),
                Err(e)=>append_log(&format!("[hook] transition_engine 실패: {}\n", e)),
            }
            }
            // ★fc59a0 recall RNG score(0x2080e20, 12B=push8). cfg recallcap=1 검증(리턴훅 kind:5) / recall_repl=1 완전대체(replace-rax: SENT=passthrough, 그외=out ptr로 skip).
            match install_replace_detour_rax(RVA_FC59A0, 12, fc59a0_capture as *const () as usize) {
                Ok(())=>append_log("[hook] fc59a0 recall score(@0x2080e20, 12B, replace-rax) OK\n"),
                Err(e)=>append_log(&format!("[hook] fc59a0 실패: {}\n", e)),
            }
            // ★CAND_FILTER white-box 검증(0x1f4ec60, 12B=push8). cfg candcap=1. 리턴훅 kind:9.
            if INSTALL_DIAG_HOOKS {
            match install_detour(RVA_CAND_FILTER, 12, cand_filter_capture as *const () as usize) {
                Ok(())=>append_log("[hook] CAND_FILTER white-box검증(@0x1f4ec60, 12B) OK\n"),
                Err(e)=>append_log(&format!("[hook] CAND_FILTER 실패: {}\n", e)),
            }
            }
            // ★FUN_1420e88a0(poke 후보선택자) 입력+vtable 덤프(게터 해결). 첫 40콜 e88a0.txt(always-on cap).
            if INSTALL_DIAG_HOOKS {
            match install_detour(RVA_E88A0, 12, e88a0_capture as *const () as usize) {
                Ok(())=>append_log("[hook] e88a0 poke선택자 캡처(@0x20e88a0, 12B) OK\n"),
                Err(e)=>append_log(&format!("[hook] e88a0 실패: {}\n", e)),
            }
            }   // poke_repl이 mp_capture 내부 my_e88a0_count로 자급
            // ★generic_build 스코어러 white-box 검증(0x1f80320, 12B=push8). cfg gbcap=1. 리턴훅 kind:11.
            if INSTALL_DIAG_HOOKS {
            match install_detour(RVA_F80320, 12, f80320_capture as *const () as usize) {
                Ok(())=>append_log("[hook] genbuild scorer white-box검증(@0x1f80320, 12B) OK\n"),
                Err(e)=>append_log(&format!("[hook] genbuild scorer 실패: {}\n", e)),
            }
            }
            // ★generic_build 본체(0x20def90, 12B=push8) 디스패치/출력 캡처. cfg gbbody=1. 리턴훅 kind:14. (task#23)
            if INSTALL_DIAG_HOOKS {
            match install_detour(RVA_GENERIC_BUILD, 12, genbuild_body_capture as *const () as usize) {
                Ok(())=>append_log("[hook] generic_build body 출력캡처(@0x20def90, 12B) OK\n"),
                Err(e)=>append_log(&format!("[hook] generic_build body 실패: {}\n", e)),
            }
            }
            // ★영역 D callee 검증(0x203cb30 단일종합점수 12B / 0x20c0690 post점수 14B). cfg gbcallee=1. 리턴훅 kind:20. (task#2)
            if INSTALL_DIAG_HOOKS {
            match install_detour(RVA_GB_203CB30, ORIG_LEN_GB_203CB30, gb203_capture as *const () as usize) {
                Ok(())=>append_log("[hook] gb203 영역D callee 검증(@0x203cb30, 12B) OK\n"),
                Err(e)=>append_log(&format!("[hook] gb203 실패: {}\n", e)),
            }
            match install_detour(RVA_GB_20C0690, ORIG_LEN_GB_20C0690, gb690_capture as *const () as usize) {
                Ok(())=>append_log("[hook] gb690 영역D callee 검증(@0x20c0690, 14B) OK\n"),
                Err(e)=>append_log(&format!("[hook] gb690 실패: {}\n", e)),
            }
            }
            // ★영역 D: 0x42a3 캡처/검증/skip 디투어(handled→funnel skip / passthrough→capture+verify). cfg gbrd=verify·gbskip=진짜skip. cap_fn i64.
            if !MIG_GB_CHANGED {
            match install_detour_d_skip(RVA_GB_REGIOND_HOOK, ORIG_LEN_GB_REGIOND, gbrd_capture as *const () as usize, RVA_GB_FUNNEL) {
                Ok(())=>append_log("[hook] gbrd/gbskip 영역D(@0x20e42a3 mid-func, 15B, align-fix, skip→funnel) OK\n"),
                Err(e)=>append_log(&format!("[hook] gbrd/gbskip 실패: {}\n", e)),
            }
            } else { let _=(install_detour_d_skip as *const(), gbrd_capture as *const(), RVA_GB_REGIOND_HOOK, RVA_GB_FUNNEL, ORIG_LEN_GB_REGIOND); append_log("[hook] gbrd/gbskip 영역D SKIP (MIG_GB_CHANGED=true, 0.4.14 generic_build region D 재추출 대기)\n"); }
            // ★영역 D 한정 대체 100% inline: 에필로그 hook(0x20df5da, 15B). cfg gbrepl=1. 0x42a3 저장 pred로 out+0x58/0x60 덮어씀(전건).
            if INSTALL_DIAG_HOOKS {
            match install_detour_d(RVA_GB_EPILOGUE, ORIG_LEN_GB_EPILOGUE, gbrd_epilogue_apply as *const () as usize) {
                Ok(())=>append_log("[hook] gbrepl 영역D 대체(@0x20df5da epilogue, 15B, align-fix) OK\n"),
                Err(e)=>append_log(&format!("[hook] gbrepl 실패: {}\n", e)),
            }
            }   // gbrepl=0(gbskip이 활성), 에필로그 트램폴린(매 region D) 제거
            // ★facet#5 engage draw1: FUN_1420e9a30 gather+pick 캡처(0x20e9a30, 12B=push8). cfg e9a30cap=1. 리턴훅 kind:13.
            if INSTALL_DIAG_HOOKS {
            match install_detour(RVA_E9A30, 12, e9a30_capture as *const () as usize) {
                Ok(())=>append_log("[hook] e9a30 engage draw1 캡처(@0x20e9a30, 12B) OK\n"),
                Err(e)=>append_log(&format!("[hook] e9a30 실패: {}\n", e)),
            }
            }
            } // end if !MIG0413 (미마이그 훅)
            // ★0.4.13 마이그완료: facet#1 condgate(@0x1be1290, 15B). cfg condcap=1. 리턴훅 kind:6.
            // ★replace-detour(rax): cond_repl=0이면 cap_fn이 SENT→passthrough(install_detour와 동일). cond_repl=1이면 my_condgate(≠-99)로 완전대체.
            match install_replace_detour_rax(RVA_CONDGATE, 15, condgate_capture as *const () as usize) {
                Ok(())=>append_log("[hook] facet#1 condgate(@0x1c383f0, 15B, replace-rax) OK\n"),
                Err(e)=>append_log(&format!("[hook] condgate 실패: {}\n", e)),
            }
            // ★facet#4 movepriority 관측(0x1c08420, 14B=7push+sub0x50). cfg mpcap=1. 리턴훅 kind:7.
            // ★replace-detour(sret rax=rcx): mp_repl=0이면 cap_fn이 1→passthrough(install_detour와 동일). mp_repl=1이면 disc0/1 완전대체.
            match install_replace_detour(RVA_MOVEPRI, 14, mp_capture as *const () as usize) {
                Ok(())=>append_log("[hook] facet#4 movepriority(@0x1c38c30, 14B, replace-sret) OK\n"),
                Err(e)=>append_log(&format!("[hook] movepriority 실패: {}\n", e)),
            }
            // ★df0c10 후보 getter 1회 캡처용 진입훅(0x2068b10, 12B). poke 콜러 1회만 발동(DF0CGP_DONE).
            if INSTALL_DIAG_HOOKS {
            match install_detour(RVA_DF0C10_FN, 12, df0c10_entry_probe as *const () as usize) {
                Ok(())=>append_log("[hook] df0c10 getter probe(@0x1b2eac0, 12B) OK\n"),
                Err(e)=>append_log(&format!("[hook] df0c10 probe 실패: {}\n", e)),
            }
            }
        } else { let _ = (RVA_TTD, ttd_capture as *const ()); let _ = install_df0c10_hook as *const (); let _ = (RVA_DD7700, dd7700_capture as *const ()); let _ = (RVA_FCD980, fcd980_capture as *const ()); let _ = (RVA_CHACHA, chacha_capture as *const ()); let _ = (RVA_TRANS, trans_capture as *const ()); let _ = (RVA_FC59A0, fc59a0_capture as *const ()); let _ = (RVA_CAND_FILTER, cand_filter_capture as *const ()); let _ = (RVA_F80320, f80320_capture as *const ()); let _ = (RVA_GENERIC_BUILD, genbuild_body_capture as *const ()); let _ = (RVA_GB_203CB30, gb203_capture as *const ()); let _ = (RVA_GB_20C0690, gb690_capture as *const (), ORIG_LEN_GB_203CB30, ORIG_LEN_GB_20C0690); let _ = (RVA_CONDGATE, condgate_capture as *const ()); let _ = (RVA_MOVEPRI, mp_capture as *const ()); }
        // 보류 훅 (subplan 작업 재개 시 활성): dispatch / t9360 / driver
        let _ = (RVA_DISPATCH, dispatch_capture as *const ());
        let _ = (RVA_DRIVER, driver_capture as *const ());
        let _ = (RVA_T9360, t9360_capture as *const ());
    }
    load_cfg(true);
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(CfgExt);
    reg.add_player_input_ai(PlanAi);
    reg
}
declare_mod!(init);
