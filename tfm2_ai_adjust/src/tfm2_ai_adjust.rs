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
use std::sync::atomic::{AtomicBool, AtomicI16, AtomicI64, AtomicPtr, AtomicU8, AtomicU32, AtomicU64, AtomicUsize, Ordering};
#[path = "ui_inject_embed.rs"] mod uinj;   // ★내장 UI 가산주입(별도 tfm2_ui_inject 모드 불필요)
// ★★[07-30 유저 지시] 개인전술 화면 "AI 버튼" UI 주입 = **당분간 중단**.
//   false 면 uinj::install()/tick() 을 아예 호출하지 않는다 ⟹ ① 게임 UI 로더를 후킹하지 않고
//   ② ui_inject.txt 도 읽지 않으며 ③ LOADER/PARSER/ALLOC RVA 가 틀려 있어도 위험이 0이다.
//   (0.5.3 에서 LOADER 는 clone family 라 프롤로그로 변별 불가 = 오후킹 위험이 있던 자리 — 꺼두면 그 리스크도 같이 유예된다.)
//   재개하려면 이 한 줄만 true 로. 주입 목록은 mods/tfm2_ai_adjust/ui_inject.txt 그대로 보존돼 있다.
const UI_INJECT_ON: bool = false;
#[path = "knobs.rs"] mod knobs;   // ★자동생성: 편집기 항목 KNOBS 배열
#[path = "../../ui_kit/ui_kit.rs"] mod ui_kit;   // ★UI 조작(textedit/find_mut/set_visible)
use std::sync::Mutex;

use std::time::{Instant, SystemTime, UNIX_EPOCH};
include!("gb_kit.rs");
include!("rva_054.rs");   // ★0.5.4 마이그(2026-08-05): rva_053.rs → rva_054.rs. 구파일은 이력용 보존(참조 없음).
include!("mem_safety.rs");
include!("detour.rs");
include!("class_micro.rs");   // ★[08-07] 바이트패치 노브를 클래스별로 여는 마이크로 디투어(설계 = RE 부록 B)
include!("nexus_emg.rs");     // ★[08-08] "넥서스 비상" 발동 조건(쌍둥이 타워·2차 타워) 노브
include!("probe.rs");   // ★[08-04] 런타임 진단 프로브(probe=0 기본 OFF) — 정적 분석으로 안 뚫린 6건 계측
include!("serpen.rs");
include!("disc19_repro.rs");

const MOD_ID: &str = "tfm2_ai_adjust";
// facet#4 movepriority. 프롤로그 7push+sub0x50 = 14B 클린. rcx=출력ptr(rsi), rdx=subplan ptr(*=disc), 스택: r14(sim)@entry+0x28, r15(rh)@entry+0x30
const DD7_TAIL_OK: bool = true;   // ★engage-tail 재활성(2026-06-19): STAGE6 resolver/vt168 this=sim 수정(AV근본원인=rf(target)→rf(sim)). 디컴 confirm.
const INSTALL_DIAG_HOOKS: bool = false;   // ★성능(2026-06-22): 휴면 진단/캡처훅 미설치(프로덕션). ⚠⚠07-10 확정: true로 켜면 크래시 — 휴면훅(e88a0 @0x20e88a0 등)이 0.5.0_3 미재검증 stale이라 설치 즉시 AV(fault 0x20e9a2d=e88a0+0x118d, 3회 재현). mpcap/condcap/serpen_verify 검증은 KEEP훅(retreat/condgate/movepri/fc59a0=항상설치)만으로 충분 — 이 플래그 켤 필요 없음. 휴면훅 재활성하려면 RVA 전수 재마이그 선행.
const MIG_GB_CHANGED: bool = true;   // ★07-10 region D 정적완결로 넘어감(유저 지시 "추측으로 game=mine 맞춤"): gb_region_d_050 = ghidra-re 결정블록(0x22daff8~0x22db0b0) 완전일치=정적 game=mine 확정. gbrd_capture는 0.5.0_3 재작성완료(rbp맵 out@0x2b8/cnt0x110/da0x100/db0x108/d20x170/l2 0x1d0·dedc0 인라인 score/sim_scale)·detour 크래시 수정완료(안전슬롯 0x22dafea/orig14). ⚠단 런타임 detour raw=0 미해결(데모/리플레이/라이브 전부 0, movepri/condgate는 정상=훅인프라OK) → detour 실행경로/설치 재검증은 다음세션. 그때까지 훅 미설치(true)로 프로덕션 안전. **앵커 재추출완료**(REGIOND_HOOK 0x22daff8·EPILOGUE 0x22dbd22·FUNNEL 0x22dbc4e·DEDC0 인라인 0x22db05e; 203CB30/20C0690=vtable간접 소멸). **로직 재작업완료**=gb_region_d_050(결정트리). ⬜잔여=capture 디투어 새 RBP레이아웃(out@0x2b8·CNT@0x110·D2@0x170)·gbrd 하네스 macro_op(out[0]=9/action-Vec) 대조·스코어러 FUN_1420a5030(CNT/DB) 직접구현 → 이후 false 재활성+검증.
// facet#5 셀렉터(local_228) 신선포착: retreat_engage 내 df0c10 호출 직후 [rcx]=셀렉터(1=역할기반). 리턴前엔 액션코드로 덮임.
const ROSTER_BASE: usize = 0x1e0;        // plan_base + team*0x228 + 0x1e0
const ROSTER_STRIDE: usize = 0x228;
const ROSTER_N: usize = 5;
// 전투엔티티 오프셋
const E_POSX: usize = 0x648; const E_POSY: usize = 0x650;
const E_HP: usize = 0x658;   const E_MAXHP: usize = 0x610;
const E_SPEED: usize = 0x628;


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
// ★disc3(dd7700/CAND_FILTER) 완전대체 게이트(cfg dd7_repl). RNG-sync 검증완료(DIFF=0/21500, 2026-06-20) + writeback 배선 → 대체시 출력+RNG 둘다 비트동일=no-desync. cfg로 토글.
static DD7_REPL: AtomicBool = AtomicBool::new(false);
// ★disc9/11(EpicPoke/SerpenPoke) 대체 게이트(cfg poke_repl). ⚠출력재현만 검증(pokecmp DIFF=0), RNG-sync 미구현 → 켜면 desync. RNG-sync 구현후 활성. DD7_REPL과 분리(disc3만 안전하게 켜기 위함).
static POKE_REPL: AtomicBool = AtomicBool::new(false);
static DD7_REPL_RNG_N: AtomicU64 = AtomicU64::new(0);     // disc3 대체시 RNG writeback 적용 횟수(진단)
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
// disc별 최대 in-scope draw 관측(0이어야 안전). idx=disc.min(15)
static COND_DISC_MAXDRAW: [AtomicU64; 16] = [const { AtomicU64::new(0) }; 16];
static DD7_RNG_N: AtomicU64 = AtomicU64::new(0);     // my_dd7700_rng_final 예측 draw수(gen_range 호출수)
static DD7_RNG_LO: AtomicU64 = AtomicU64::new(0);    // 진단: 내 윈도우 lo
static DD7_RNG_HI: AtomicU64 = AtomicU64::new(0);    // 진단: 내 윈도우 hi
static DD7_RNG_I0: AtomicU64 = AtomicU64::new(0);    // 진단: 내 entry idx
static DD7_RNG_CMASK: AtomicU64 = AtomicU64::new(0); // 진단: candtable 5비트 non-null 마스크
static DD7_RNG_DBG: AtomicU64 = AtomicU64::new(0);   // 진단패킹: plan(8) | f(4)<<8 | ivar12==1<<12 | target!=0<<13 | reached_candfilter<<15
static DD7_RNG_CTAB: AtomicUsize = AtomicUsize::new(0); // 진단: candtable base(l80+0x1e0+other*0x28). exit 재독으로 dd7700이 수정하는지 확인
static DD7_RNG_PI14: AtomicUsize = AtomicUsize::new(0); // role record addr(side*0x228+geo+roleoff). exit 재독으로 dd7700이 iVar12/target 수정하는지 확인
static DD7_RNG_TH0: AtomicU64 = AtomicU64::new(0);      // entry tgt_handle
static DD7_PATH: AtomicU64 = AtomicU64::new(0);  // ★DIAG(disc0 my=4≠g=2 추격): 마지막 my_dd7700_code 리턴 경로. 1=early7 2=L2조기(6/4) 3=frontier-bail2 4=cover4/7 5=stage1-2 6=tail기타
// ★★[07-23] disc0 tail 오판(my=7 path=0 → game=2) 추격 계측. my_dd7700_code STAGE6 종단 세분 태그 + 진단입력.
//   my_dd7700_code = 캡처 비교 전용(실제 대체는 my_dd7700_full)이라 이 계측은 게임 동작 무영향.
//   TERM 태그: 40=STAGE1 role!=1 / 41=resolver / 42=target / 43=selfobj / 44=nexus / 45=anchor
//            / 46=GATE_C / 47=GATE_D / 48=GATE_E / 50=ref_self!=0 / 51=ref_self0_2 / 52=ref_self0_86dd
//            / 53=ref_self0_872d / 54=86c1_872d / 55=86c1_count<=3(2) / 56=86c1_86dd
static DD7_TERM: AtomicI64 = AtomicI64::new(-1);  // 마지막 my_dd7700_code STAGE6 종단 태그
static DD7_DBG: [AtomicI64; 11] = [const { AtomicI64::new(0) }; 11];  // [0]ivar2 [1]plan [2]bl [3]route8679 [4]term86dd [5]term872d
//   ★[07-23 3차] [10]=vt30_kind(GameMode 0=Moba/1=SingleLane/2=DeathMatch). ⚠**`my_f22e80_count`는 mode==2 경로만 재현**하는데
//     실전이 mode!=2면 원본은 **완전히 다른 알고리즘**(로컬시드 RNG `FUN_141b78380`, p4 draw **0회**)을 탄다 ⟹ count 전체가 무의미.
//     모드 기록엔 "실경기=kind0"이라 적혀 있어(L6027) **정면 배치** — 런타임 실측으로 판정해야 한다.
//   ★[07-23 2차] 잔여 DIFF 원인 분리용 추가: [6]count_survivors [7]near_cnt [8]n(selfobj+8) [9]*(e+n*0x18+0x38)
//   ⚠유력 가설 = **count_survivors 계측 아티팩트**: `my_f22e80_count`는 RNG를 소비해 산출하는데, mpcmp 캡처는
//     **리턴훅**(게임이 이미 RNG 전진시킨 뒤)에서 돌므로 진입시점 기준인 게임 원본과 원리적으로 어긋날 수 있다.
//     사실이면 실제 대체 경로(my_dd7700_full=진입시점)는 정확하고 mpcmp DIFF만 무의미(disc10/11 전례와 동일 부류).
static DD0_DIFF_N: AtomicU64 = AtomicU64::new(0);  // dd0diff.txt 덤프 카운터(300 상한)
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
// ★07-11 크래시대책①(§12.23/[[tfm2-mod-safety]]§8): disc5/6 대체 격리게이트(cfg mp_d56_repl, 기본0=관측만).
//   disc5/6=정규매치 미발화·스테이지1/희귀 전용 → 그 컨텍스트서의 대체=미검증 경로 유도 리스크만 있고 실익 0.
//   스테이지1 게이트(vt+0x30==1 감지) 배선·인게임 검증 후 기본 1 복귀 검토.
static MP_D56_REPL: AtomicBool = AtomicBool::new(false);
// ★07-11 크래시대책②: itemnet 스코어러(0x1b78420) fn+12 가드 스텁의 차단 카운트(BAD-arg 진입=바닐라라면 AV였을 횟수).
static ITEMNET_GUARD_HITS: AtomicU64 = AtomicU64::new(0);
static ITEMNET_GUARD_SEEN: AtomicU64 = AtomicU64::new(0);   // 로그 중복방지(마지막 로깅값)
// ★movepriority 출력계약 진단(capture모드): 진입시 *param_1 8qword 스냅 → 리턴서 diff = sub-judge가 쓴 오프셋. code-only(+0만)/aux 판별.
static MP_ENTRY: [AtomicU64; 8] = [AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0)];
static MP_ENTRY_PTR: AtomicUsize = AtomicUsize::new(0);
static MP_WS: [AtomicU64; 16] = [AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0)];
static MP_WS_INIT: AtomicBool = AtomicBool::new(false);
// ★disc9/11(EpicPoke/SerpenPoke) full-output 대체검증: capture시 aux입력 보관 → hook_return kind7서 내 full재현 vs 게임출력 byte대조(pokecmp.txt).
static MP_AUX_OP: AtomicUsize = AtomicUsize::new(0);   // 현 in-flight poke의 출력ptr(프레임 매칭). 비중첩이라 단일 static 안전.
static MP_AUX_P2: AtomicUsize = AtomicUsize::new(0);   // 서브저지 param_2(=subp+8): code0x13의 [+0x11]=byte[p2]
static MP_AUX_P6: AtomicUsize = AtomicUsize::new(0);   // 서브저지 param_6(=r15): epic P6(roster holder)
static MP_AUX_P3: AtomicU64 = AtomicU64::new(0);       // 0.5.0 epic P3(phase, r8): epic_poke_write 재현용
static MP_AUX_P5: AtomicUsize = AtomicUsize::new(0);   // 0.5.0 epic P5(SimState, r14): epic_poke_write 재현용
static MP_AUX_SF: AtomicUsize = AtomicUsize::new(0);   // disc12 SerpenBattle sf(p7_dd): +0x3ea/+0x3eb 분기
// ═════════════ ★[07-29] divergence tracer (cfg detlog=1) — 개입 지점별 (seed,tick,site,val) 해시 누적 → 두 sim 자동 대조 ═════════════
//   목적: 배경 pre-sim vs 관전 re-sim의 "첫 어긋난 사이트+시점"을 발산 1경기로 직접 지목(발산율 관찰 대체).
//   구조: seed슬롯 128 × tid슬롯 2 × 사이트 16 × tick버킷 32 의 xor-해시(u64). detour-safe(고정배열 atomic·무할당·무IO).
//   사이트: 0=dd7(disc0/1/3) 1=disc2/8 2=disc5/6 3=disc7 4=disc9 5=disc10 6=disc11 7=disc12 8=disc13 9=disc14
//          10=disc16 11=disc17 12=numbers후퇴발동 13=d12픽tag 14=d14픽tag 15=예비. 버킷=tick>>10(min31).
// ★[07-29 v2] 버킷 64틱(원인/결과 분리) + **IN/OUT 분리**(site 0~15=IN, 16~31=OUT).
//   판정: 같은 버킷서 **IN 일치인데 OUT 불일치 = 그 사이트가 범인**(동일입력 다른결정) / IN 불일치 = 상류 발산.
// ★[07-29 v3] **2채널 고해상도**: ch0=상태(판단 입력 = 엔티티 HP/좌표 누적) / ch1=RNG(idx,counter).
//   8틱 버킷 × 4096 = 32,768틱 커버. 목적 = **어느 채널이 먼저 갈리는지**(RNG 원인 vs 상태 원인) 직접 판별.
//   누적은 **wrapping_add**(xor는 같은 값 2회에 상쇄돼 무변화 tick이 지워짐).
// ★[07-29 v4] 채널: 0=상태(mp IN) 1=RNG 2=cond판정 3=recall판정 4=numbers후퇴 5=mp출력코드.
//   ★판정 규칙: **ch0·ch1(상태·RNG) 일치인데 2~5 중 하나가 불일치 = 그 판정면이 비결정 = 범인**.
const DL_SEEDS: usize = 32; const DL_SITES: usize = 8; const DL_BUCKETS: usize = 4096;
// mp 진입에서 본 World를 스레드별로 보관 → cond/recall/numbers 훅(World 인자 없음)에서 재사용.
thread_local! { static DL_WORLD_TL: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
#[inline] fn dl_world_tl() -> usize { DL_WORLD_TL.with(|c| c.get()) }
static DL_ON: AtomicBool = AtomicBool::new(false);
static DL_SEED: [AtomicU64; DL_SEEDS] = [const { AtomicU64::new(0) }; DL_SEEDS];
static DL_TID: [[AtomicU32; 2]; DL_SEEDS] = [const { [AtomicU32::new(0), AtomicU32::new(0)] }; DL_SEEDS];
static DL_H: [AtomicU64; DL_SEEDS * 2 * DL_SITES * DL_BUCKETS] = [const { AtomicU64::new(0) }; DL_SEEDS * 2 * DL_SITES * DL_BUCKETS];
#[inline] fn dl_mix(mut x: u64) -> u64 { x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9); x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb); x ^ (x >> 31) }
// world(=gchild)에서 seed/tick 읽어 기록. detlog off면 no-op. site<16.
unsafe fn dl_rec(world: usize, site: usize, val: u64) {
    if !DL_ON.load(Ordering::Relaxed) || site >= DL_SITES || !ptr_ok(world) { return; }
    let seed = rd_u64(world + 0xeb28).unwrap_or(0); if seed == 0 { return; }   // ★[08-06] 구 0xeaf8 → 0xeb28 (0.5.4: 구값은 exe에 0회)
    let tick = rd_u64(world + 0xeb00).unwrap_or(0); if tick > (1 << 40) { return; }
    // seed 슬롯: (seed>>3)%128 시작 선형 8칸(등록 or 빈칸 claim), 실패=드롭
    let start = ((seed >> 3) as usize) & (DL_SEEDS - 1);
    let mut slot = usize::MAX;
    for i in 0..8 {
        let s = (start + i) & (DL_SEEDS - 1);
        let cur = DL_SEED[s].load(Ordering::Relaxed);
        if cur == seed { slot = s; break; }
        if cur == 0 && DL_SEED[s].compare_exchange(0, seed, Ordering::Relaxed, Ordering::Relaxed).is_ok() { slot = s; break; }
    }
    if slot == usize::MAX { return; }
    // tid 슬롯 0/1 (첫 두 스레드만)
    let tid = GetCurrentThreadId();
    let mut ts = usize::MAX;
    for t in 0..2usize {
        let cur = DL_TID[slot][t].load(Ordering::Relaxed);
        if cur == tid { ts = t; break; }
        if cur == 0 && DL_TID[slot][t].compare_exchange(0, tid, Ordering::Relaxed, Ordering::Relaxed).is_ok() { ts = t; break; }
    }
    if ts == usize::MAX { return; }
    let b = ((tick >> 3) as usize).min(DL_BUCKETS - 1);   // ★8틱 버킷(고해상도)
    let idx = ((slot * 2 + ts) * DL_SITES + site) * DL_BUCKETS + b;
    DL_H[idx].fetch_add(dl_mix(tick.wrapping_mul(0x9e3779b97f4a7c15) ^ val.rotate_left(17) ^ 1), Ordering::Relaxed);
}
// 덤프(전용 스레드에서 5초마다): 2-tid seed 중 해시표 불일치를 detdiv.txt로.
fn dl_dump() {
    const SN: [&str; 16] = ["dd7(d0/1/3)", "d2/8", "d5/6", "d7", "d9", "d10", "d11", "d12", "d13", "d14", "d16", "d17", "numbers후퇴", "d12픽", "기타disc", "★RNG상태"];
    let mut out = String::from(
        "# detlog divergence tracer v2 — IN(입력상태)/OUT(모드결정) 분리 · 64틱 버킷\n\
         # ★★판정: 같은 버킷에서 **IN 일치 + OUT 불일치 = 그 사이트가 범인**(동일 입력에 다른 결정 = 모드 비결정).\n\
         #        IN 불일치 = 그 시점엔 이미 상류에서 상태가 갈린 것(결과) → 더 앞 버킷/사이트를 볼 것.\n\
         # 사이트: 0=dd7 1=d2/8 2=d5/6 3=d7 4=d9 5=d10 6=d11 7=d12 8=d13 9=d14 10=d16 11=d17 12=numbers후퇴 13=d12픽 14=기타 **15=RNG상태**\n\
         # ★site15 전용 해석: IN=mp진입시 rng(idx,counter) / OUT=처리후 rng. **IN✓ + OUT✗ = 모드가 draw를 다르게 소비(RNG 직격)**\n\
         #                    site15가 IN✗로 시작 = RNG가 mp 밖(recall 등)에서 이미 갈림.\n\n");
    let _ = SN;
    let mut any = false;
    for s in 0..DL_SEEDS {
        let seed = DL_SEED[s].load(Ordering::Relaxed);
        if seed == 0 || DL_TID[s][1].load(Ordering::Relaxed) == 0 { continue; }
        let h = |ts: usize, ch: usize, b: usize| DL_H[((s * 2 + ts) * DL_SITES + ch) * DL_BUCKETS + b].load(Ordering::Relaxed);
        // 채널별 최초 불일치 버킷(양쪽 다 기록이 있는 버킷만 비교 — 한쪽만 돌면 당연히 다르므로 제외)
        let first_mis = |ch: usize| -> Option<usize> {
            (0..DL_BUCKETS).find(|&b| {
                let (a, c) = (h(0, ch, b), h(1, ch, b));
                a != 0 && c != 0 && a != c
            })
        };
        let (fs, fr) = (first_mis(0), first_mis(1));
        let (fc, frc, fn_, fo) = (first_mis(2), first_mis(3), first_mis(4), first_mis(5));
        if fs.is_none() && fr.is_none() && fc.is_none() && frc.is_none() && fn_.is_none() && fo.is_none() { continue; }
        any = true;
        out.push_str(&format!("== seed {:#018x}  tid {}/{}\n", seed,
            DL_TID[s][0].load(Ordering::Relaxed), DL_TID[s][1].load(Ordering::Relaxed)));
        let f = |o: Option<usize>| match o { Some(b) => format!("tick {}~{}", b << 3, ((b + 1) << 3) - 1), None => "없음(일치)".into() };
        out.push_str(&format!("   상태(ch0) 최초 불일치 : {}\n", f(fs)));
        out.push_str(&format!("   RNG (ch1) 최초 불일치 : {}\n", f(fr)));
        out.push_str(&format!("   cond(ch2)/recall(ch3)/numbers(ch4)/mp출력(ch5) : {} / {} / {} / {}\n", f(fc), f(frc), f(fn_), f(fo)));
        // ★★스모킹건: 상태·RNG는 아직 일치인데 판정 채널이 먼저 갈린 경우 = 그 판정면이 비결정
        let base = fs.unwrap_or(usize::MAX).min(fr.unwrap_or(usize::MAX));
        for (ch, nm, v) in [(2, "condgate", fc), (3, "recall", frc), (4, "numbers후퇴", fn_), (5, "mp출력코드", fo)] {
            if let Some(b) = v { if b < base {
                out.push_str(&format!("   ★★범인 확정: **ch{} {}** @tick {}~{} — 상태·RNG 일치 시점인데 이 판정만 갈림 = 비결정\n",
                    ch, nm, b << 3, ((b + 1) << 3) - 1));
            }}
        }
        out.push_str(match (fs, fr) {
            (Some(a), Some(b)) if b < a => "   ★★판정: **RNG가 먼저 갈림** → 원인 = 모드의 RNG 소비 불일치(draw 수). recall/picker 등 writeback 사이트 조사.\n",
            (Some(a), Some(b)) if a < b => "   ★★판정: **상태가 먼저 갈림** → RNG는 따라간 결과. 원인 = 모드가 게임 메모리에 남기는 write(출력 잔재 등) 또는 판단 외 개입.\n",
            (Some(_), Some(_)) => "   ★★판정: 같은 버킷서 동시(8틱 이내) → 더 세밀한 추적 필요(둘 중 하나가 직전 tick 원인).\n",
            (None, Some(_)) => "   ★★판정: **RNG만 갈림**(상태 일치) → 모드 RNG 소비 불일치가 유일 원인.\n",
            (Some(_), None) => "   ★★판정: **상태만 갈림**(RNG 일치) → RNG 무관. 모드의 메모리 write/판단 외 개입이 원인.\n",
            _ => "",
        });
        out.push('\n');
    }
    if !any { out.push_str("(2-sim seed 중 개입축 불일치 없음 — 발산원은 기록 사이트 밖)\n"); }
    // ★LOG_ON 무관 직접 write(perf.txt 선례): detlog=1이면 log 플래그 없이도 기록(진단 자가완결).
    if let Some(p) = pth("detdiv.txt") { let _ = fs::write(p, &out); }
}
static MP_AUX_M18: AtomicU64 = AtomicU64::new(0);      // ★disc12 entry 스냅샷 mem+0x18: one-shot 리액티브 플래그(게임핸들러가 소비/클리어 → 리턴훅 재읽기=항상0이던 5.5% DIFF 원인)
static MP_AUX_M19: AtomicU64 = AtomicU64::new(0);      // ★disc12 entry 스냅샷 mem+0x19 (동일)
static MP_AUX_WLEN: AtomicU64 = AtomicU64::new(0);     // ★disc12 entry 스냅샷 W큐 len(*(gchild+0xeaf0+0x1a8)): 게임핸들러가 오더 소비→리턴훅 재읽기=0이던 0xc/0xd 아티팩트 해소(07-10)
static MP_AUX_WH: AtomicU64 = AtomicU64::new(0);       // ★disc12 entry 스냅샷 W큐 첫핸들(*(*(gchild+0xeaf0+0x1a0))), len==0이면 무효
static MP_AUX_C1A: AtomicU64 = AtomicU64::new(0);      // ★disc14 entry 스냅샷 cmd+0x1a: 다이브추적 pre-state(게임핸들러가 이번 틱 갱신 전 값 — B/C 경로 판별용)
static MP_AUX_RNG: AtomicUsize = AtomicUsize::new(0);  // disc12 SerpenBattle rng(r9): picker draw(replace-mode 잔여)
static MP_AUX_TP: AtomicUsize = AtomicUsize::new(0);   // disc12 SerpenBattle tp(p7p): engage게이트 인자
// ★disc12 SerpenBattle(body FUN_14235c440) 검증/대체 게이트. VERIFY=capture-compare(serpencmp.txt), REPL=완전대체.
//   ⚠기본 OFF: my_serpen_battle는 vt0x138/0x150 shadow-call(AV위험, §3) 포함 → 명시 활성시만 실행.
static SERPEN_VERIFY: AtomicBool = AtomicBool::new(false);
// ★disc12 my_serpen_battle 진단(serpendiag.txt). 0=인자·1=g0/plan·2=gchild/gvt·3=side·4=selfe·5=maxhp·6=0x14·7=0xc·8=7·9=0xd·10=2:3·11=0xe·12=총
//   +분기: 13=out_of_zone·14=in_zone·15=tag==1·16=tgt_full·17=sf3eb==3·18=sf3ea!=0·19=main-engage도달.
static SERPEN_DIAG: [AtomicU64; 28] = [
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),   // ★07-10 확장: [20]reactive&!have_order [21]reactive&order&tgt0 [22]main&have_order [23]예비
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),   // ★07-23 진입가드 세분: [24]geom0 [25]sim0 [26]mem0 [27]예비 ([0]=out0)
];
// ★disc12 0xe/0xc divergence 추적(2026-07-10): my_serpen_battle 마지막 호출의 경로 기록. serpen_verify 진단전용.
//   [0]최종emit코드 [1]have_order [2]ptr_ok(target) [3]m18 [4]m19 [5]engage_gate결과 [6]gate stage(0진입~10 true)
//   [7]/[8]stage별 aux(stage6=cnt/lane_id, stage9=best_idx/sim+0x8b0) [9]d2>>8 [10]thr>>8 [11]tick. 미스매치시 sgate.txt 덤프.
static SGT: [AtomicI64; 14] = [const { AtomicI64::new(-1) }; 14];   // +[12]tag(picker) [13]zenc=oz|tgtfull<<1|hp_pct<<8
#[inline] fn sgt(i: usize, v: i64) { SGT[i].store(v, Ordering::Relaxed); }
static SGT_N: AtomicU64 = AtomicU64::new(0);   // sgate.txt 미스매치 덤프 카운터
// ★disc9 EpicPoke 검증 미스매치 원인 특정용 경량 진단(SGT와 별개 배열). epic_poke_compute 각 게이트서 어디서 갈라지는지 기록.
//   [0]최종active [1]p3 [2]bvar1(gateflag) [3]disc(sub+0x58) [4]clane [5]cand(geom_resolve150 결과 ptr!=0)
//   [6]lane(sub+0x8d) [7]enemy_side [8]poke_enemy_list n [9]contested [10]phase_gate통과(p3>gate&&bvar1&1&&disc마스크) [11]reach매치여부.
static EGT: [AtomicI64; 12] = [const { AtomicI64::new(-1) }; 12];
#[inline] fn egt(i: usize, v: i64) { EGT[i].store(v, Ordering::Relaxed); }
static EGT_N: AtomicU64 = AtomicU64::new(0);   // egate.txt 미스매치 덤프 카운터
static SERPEN_ENTRY_N: AtomicU64 = AtomicU64::new(0);   // disc12 진입 카운터(serpendiag.txt 덤프 주기)
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
// ★dispatcher-레벨 e88a0 arg 재구성 검증: RNG=r9, e88a0_p4=r14(param5), e88a0_p7=*(r15+8)(param6[1]). count→gen_range(0,count) 예측 exit vs 실제 p4 exit.
static POKE_PIDX: AtomicU64 = AtomicU64::new(0);   // 예측 exit idx
static POKE_PCTR: AtomicU64 = AtomicU64::new(0);   // 예측 exit counter
static POKE_PCOUNT: AtomicI64 = AtomicI64::new(-1); // 예측 count(-1=계산실패)
static POKE_E88_OK: AtomicU64 = AtomicU64::new(0);
static POKE_E88_DIFF: AtomicU64 = AtomicU64::new(0);
// ★fc59a0 recall RNG score 캡처(cfg recallcap). facet#5 cVar6==1 RECALL-vs-roll 갭. RNG배율+최종score 검증(1차).
static RECALLCAP: AtomicBool = AtomicBool::new(false);
static RECALL_ARMED: AtomicU64 = AtomicU64::new(0);
static RECALL_FILE_INIT: AtomicBool = AtomicBool::new(false);
// ★recall(fc59a0) 완전대체 게이트(cfg recall_repl). score(mult) 재현·검증완료(recallcmp DIFF=0) + u32 RNG writeback. 켜면 fc59a0 skip→내 출력+RNG전진. ⚠RECALL 희귀=검증 기회캡처.
static RECALL_REPL: AtomicBool = AtomicBool::new(false);
static RECALL_REPL_N: AtomicU64 = AtomicU64::new(0);
static RECALL_REPL_PASS: AtomicU64 = AtomicU64::new(0);
const RECALL_ARM_MAX: u64 = 600;
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
static GB_TERM: AtomicUsize = AtomicUsize::new(0);   // ready_walk 미지 terminal vt+0x58 RVA(찾기용)
// ★per-site draw 카운터(F80320 +1 진단): [0]base [1]슬롯게이트 [2]+0x78 [3]+0x7d [4]+0x82 [5]list2. my_f80320가 매 호출 리셋+증가.
static GB_SITE: [AtomicU32; 6] = [AtomicU32::new(0),AtomicU32::new(0),AtomicU32::new(0),AtomicU32::new(0),AtomicU32::new(0),AtomicU32::new(0)];
// ★영역 D 출력검증(cfg gbrd, genbuild_body_D.md "런타임 캡처 빌드"): mid-func 0x20e42a3 캡처 → RegionD locals(rbp/r12/r13)
//   → gb_region_d 예측을 out ptr 키로 GBRD_MAP 저장. generic_build 리턴훅(kind14)이 같은 out ptr로 조회해 game out+0x58/+0x60 대조.
//   ★mid-func라 return 하이재킹 불가 → 저장만. gbrd=1이면 genbuild_body_capture(kind14 리턴)도 자동 무장. 순수 read+gb_region_d(순수)=게임호출0.
static GBRD: AtomicBool = AtomicBool::new(false);
static GBRD_INSTALL_OK: AtomicU8 = AtomicU8::new(0);   // ★07-10 진단: gbrd detour 설치 결과 0=미시도 1=OK 2=실패
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
static MP_OBSERVE: AtomicBool = AtomicBool::new(false);   // ★관찰전용 캡처(cfg mp_observe): my_movepriority 미실행(my=-99), game 출력만 기록.
// ⚠07-11 튜토리얼 AV(0x1b784a1) 규명(ghidra-re): 크래시=itemnet 빌드 스코어러 FUN_141b78420이 owner(팀AI블롭)+0x1558 모델을 NULL deref(rdx≈0x1558 저주소) — 우리 shadow-call 아님(콜러체인 disjoint·vtable 미탑재로 도달불가). 우리 mp_repl 대체출력이 튜토리얼(스테이지1) 미검증 컨텍스트를 바닐라 미진입 "빌드 재계획" 경로로 유도→튜토리얼엔 itemnet 에이전트 부재→게임 자폭. 최우선 용의자=disc4(정규 미발화·튜토리얼 첫 발화)+disc5/6 인라인 write.
// ⟹ ★mp_observe(캡처 my만 OFF)로는 불충분 — mp_repl 대체는 별개 게이트(L아래 MP_REPL)로 계속 돎. 스테이지1 안전 = mp_repl=0(또는 최소 d4_repl=0)로 대체 자체를 끄거나, 향후 driver 스테이지게이트(vt+0x30()==1) 감지→대체 passthrough 배선 필요.
static MP_ARMED: AtomicU64 = AtomicU64::new(0);
static MP_OK: AtomicU64 = AtomicU64::new(0);
static MP_DIFF: AtomicU64 = AtomicU64::new(0);
static MP_PEND: AtomicU64 = AtomicU64::new(0);
// ★[07-31] disc10·11 전용 카운터. 이 둘은 게임 `out+0`이 **enum tag 상수 0xb 고정**이라 code 비교 자체가 성립하지 않는다
//   (실제 판단결과는 payload `out+8`). 종전엔 이것들이 MP_DIFF 로 집계돼 **매 경기 ★DIFF 2,500여 건**을 만들어 냈다.
static MP_NA: AtomicU64 = AtomicU64::new(0);
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
static MP_SUB_ARMED: [AtomicU64; 18] = [   // ★07-10 16→18: disc16/17 별도 슬롯(구 min(15)로 합쳐 disc17이 disc16 밀어냄)
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),
    AtomicU64::new(0),AtomicU64::new(0)];
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
// ★★ judge 튜닝 계수 (cfg [튜닝] 섹션; 기본값=게임원본 → 안 건드리면 replay-identical 유지). 우리 대체 judge의 계수를 유저가 override.
static TUNE_ENGAGE_MULT: AtomicI64 = AtomicI64::new(100);  // engage role thr 배율%: 높을수록 교전 공격적(thr↑→roll>=thr 드묾→engage), 낮으면 소극적
static TUNE_TTD_MULT: AtomicI64 = AtomicI64::new(100);     // disc4 TTD 임계 배율%: 처치/갱킹 적극성
static TUNE_RECALL_BIAS: AtomicI64 = AtomicI64::new(0);    // recall score 가산: >0=자주 복귀(안전), <0=덜 복귀(공격적 체류)
static TUNE_GB_MULT: AtomicI64 = AtomicI64::new(100);      // generic_build 영역D 거리임계 배율%: 매크로 운영전환 거리 성향
// ★subplan별 공격성 배율(2026-07-02): 각 judge의 대표 후퇴/진입 게이트에 ×배율/100. 기본100=게임원본(replay-identical). >100=공격적(덜 후퇴). max(1)로 0나눗셈 차단.
static AGGR_LANE: AtomicI64 = AtomicI64::new(100);      // [3]라인전 dd7700 프론티어 후퇴 게이트(↑=덜 물러남)
static AGGR_OBJECT: AtomicI64 = AtomicI64::new(100);    // [9·11]오브젝트 poke 견제거리 게이트(↑=더 가까이서도 견제)
static AGGR_DEFENSE: AtomicI64 = AtomicI64::new(100);   // [14]넥서스수비 HP 후퇴 게이트(↑=낮은HP에도 버팀)
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
    if let Some(v) = tune_champ_lookup(key) { return v; }   // ★선수(챔피언)별 오버라이드 최우선(players/*.cfg bare key). CUR_CHAMP<0이면 즉시 skip(무비용).
    if let Some(v) = tune_class_lookup(key) { return v; }   // ★클래스별 오버라이드("{key}_class_<cls>"). CUR_CLASS<0이면 즉시 skip(무비용). (포지션 분기는 폐지)
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

// ════════════════ 포지션별 cfg 오버라이드 (top0/jungle1/mid2/bottom3/support4) ════════════════
// 모델: "키"=전역 기본값, "키_pos_<name>"=해당 포지션만 오버라이드(미지정 포지션=전역 폴백).
//   <name>∈top/jungle/mid/bottom/support (포지션 enum순서, 2026-06-29 Ghidra확정: 라벨/아이콘 테이블).
//   예) numbers_threat=50  numbers_threat_pos_jungle=80  → 정글만 80, 나머지 50. 모든 tune()/apos() 노브에 적용.
// 포지션 해석: self 엔티티를 검증된 plan_base(CAP_PB) 로스터(team*0x228+0x1e0, 5/team)서 슬롯 매칭.
//   ⚠"슬롯 idx==포지션"은 정적 미확정 → 런타임 프로브(cfg pos_probe=1, pos_probe.txt)로 실측확정 권장.
//   병렬 sim(rayon ~8스레드) 대응: CUR_POS = thread-local(스레드별 현재 판단 중인 선수의 포지션).
thread_local! { static CUR_POS: std::cell::Cell<i8> = const { std::cell::Cell::new(-1) }; }
#[inline] fn cur_pos() -> i8 { CUR_POS.with(|c| c.get()) }
#[inline] fn set_cur_pos(p: i8) { CUR_POS.with(|c| c.set(p)); }
// ★클래스(ChampionCategory 0..4)도 동일 모델: thread-local CUR_CLASS, judge 진입 가드서 세팅.
thread_local! { static CUR_CLASS: std::cell::Cell<i8> = const { std::cell::Cell::new(-1) }; }
#[inline] fn cur_class() -> i8 { CUR_CLASS.with(|c| c.get()) }
#[inline] fn set_cur_class(c: i8) { CUR_CLASS.with(|x| x.set(c)); }
// ★선수(챔피언)별 판단 오버라이드 = players/<선수>_<챔피언>.cfg. self 엔티티 char_id로만 세팅 → 병렬 sim 다른 챔피언 무간섭.
thread_local! { static CUR_CHAMP: std::cell::Cell<i16> = const { std::cell::Cell::new(-1) }; }
#[inline] fn cur_champ() -> i16 { CUR_CHAMP.with(|c| c.get()) }
#[inline] fn set_cur_champ(v: i16) { CUR_CHAMP.with(|c| c.set(v)); }
static CHAMP_ANY: AtomicBool = AtomicBool::new(false);   // players/*.cfg 존재 → champ 조회 활성(없으면 무비용)
static CHAMP_VERIFY: AtomicBool = AtomicBool::new(false);  // champ_verify=1: 매칭된 챔프 + 적용횟수 로그(champ_verify.txt)
// ★우리팀 게이트: champ 오버라이드를 "내가 관리하는 팀 소속 챔피언"에만 적용(이적/적팀/백그라운드sim 무간섭). 기본 ON.
static SELF_TEAM_ONLY: AtomicBool = AtomicBool::new(true);
static MANAGED_TEAM_ID: AtomicI64 = AtomicI64::new(-1);   // db.player_team_id() (리그 team_id). -1=미확정.
// 내 팀 소속 athlete_id 집합(리그단위, 이적시 자동 갱신). lock-free 게시.
type IdSet = std::collections::HashSet<u64>;
static MY_ATHLETES: AtomicPtr<IdSet> = AtomicPtr::new(std::ptr::null_mut());
static MY_ATHLETES_PREV: AtomicPtr<IdSet> = AtomicPtr::new(std::ptr::null_mut());
static ALL_ATHLETES: AtomicPtr<IdSet> = AtomicPtr::new(std::ptr::null_mut());   // 전체 db.athlete_ids() (오프셋 탐색용, 0 아닌 고유값)
static ALL_ATHLETES_PREV: AtomicPtr<IdSet> = AtomicPtr::new(std::ptr::null_mut());
static ROSTER_POLL: AtomicU64 = AtomicU64::new(0);
static TEAM_DIAG_DONE: AtomicBool = AtomicBool::new(false);
static POS_ANY: AtomicBool = AtomicBool::new(false);    // cfg에 _pos_ 키 존재 → skip_untuned 우회(대체경로 보존)
static POS_PROBE: AtomicBool = AtomicBool::new(false);  // pos_probe=1: 슬롯↔챔피언 매핑 로그(슬롯==포지션 실측)
static CLASS_PROBE: AtomicBool = AtomicBool::new(false);  // class_probe=1: 엔티티 category(클래스) 캐시 오프셋 탐색(class_probe.txt, 1회)
static CLASS_SHEET: AtomicBool = AtomicBool::new(false);  // class_sheet=1: 시트워크(ChampionInfoSheet mod Vec) category 오프셋 핀포인트(class_sheet.txt, 1회)

// l80(월드모델=*p6) 로스터(team*0x28+0x1e0, 5/team)서 selfe 슬롯 매칭. ★CAP_PB 불요(judge가 l80 직접보유)=bg스레드 안전.
//   (combat_balance/count_nearby_champs가 쓰는 동일 로스터 구조. CAP_PB는 메인스레드 탐지라 일정넘김서 미발화→사용폐기.)
unsafe fn slot_in_world(l80: usize, team: i64, selfe: usize) -> i8 {
    if team < 0 || team > 1 || !ptr_ok(l80) || !ptr_ok(selfe) { return -1; }
    let base = l80 + 0x1e0 + (team as usize) * 0x28;
    for k in 0..5usize {
        if rd_u64(base + k * 8).unwrap_or(0) as usize == selfe { return k as i8; }
    }
    -1
}
// per-class 맵 lookup: "{key}_class_{name}" (name∈melee/range/magician/util/assassin). CUR_CLASS<0 또는 키 미존재면 None.
#[inline] fn tune_class_lookup(key: &str) -> Option<i64> {
    let cls = cur_class();
    if cls < 0 || cls as usize >= 5 { return None; }
    let p = TUNE_PTR.load(Ordering::Acquire);
    if p.is_null() { return None; }
    let name = CLASS_NAMES[cls as usize];
    let (kb, nb) = (key.as_bytes(), name.as_bytes());
    let n = kb.len() + 7 + nb.len();   // "_class_" = 7
    let mut buf = [0u8; 96];
    if n > buf.len() { return None; }
    buf[..kb.len()].copy_from_slice(kb);
    buf[kb.len()..kb.len() + 7].copy_from_slice(b"_class_");
    buf[kb.len() + 7..n].copy_from_slice(nb);
    let s = unsafe { std::str::from_utf8_unchecked(&buf[..n]) };
    let r = unsafe { (*p).get(s).copied() };
    if r.is_some() && CLASS_VERIFY.load(Ordering::Relaxed) { CLASS_OVHIT[cls as usize].fetch_add(1, Ordering::Relaxed); }   // 검증: 클래스 전용값 실제 적용 카운트
    r
}
// ════════════════ 선수(챔피언)별 오버라이드 테이블 (lock-free 게시 = TUNE_PTR 패턴) ════════════════
// players/<선수>_<챔피언>.cfg 각 파일 = (champion_key, bare-key 오버라이드맵) 한 항목. CUR_CHAMP=인덱스.
type ChampTable = Vec<(String, TuneMap)>;
static CHAMP_TABLE: AtomicPtr<ChampTable> = AtomicPtr::new(std::ptr::null_mut());
static CHAMP_TABLE_PREV: AtomicPtr<ChampTable> = AtomicPtr::new(std::ptr::null_mut());   // 2세대 지연 free(reader 안전)
static CHAMP_CFG_MAP: [AtomicI16; 256] = [const { AtomicI16::new(0) }; 256];   // char_id(+0x5a8)→idx+1 캐시(0=미조회, -1=매치없음). 핫패스용.
static CHAMP_OVHIT: AtomicU64 = AtomicU64::new(0);   // 진단(champ_verify): champ 전용값 실제 적용 횟수
static CHAMP_SEEN: Mutex<Vec<(String, i16)>> = Mutex::new(Vec::new());   // 진단: 탐지된 (챔프명, 매칭idx) 1회씩
fn champ_table_publish(t: ChampTable) {
    let boxed = Box::into_raw(Box::new(t));
    let old = CHAMP_TABLE.swap(boxed, Ordering::AcqRel);
    let stale = CHAMP_TABLE_PREV.swap(old, Ordering::AcqRel);
    if !stale.is_null() { unsafe { drop(Box::from_raw(stale)); } }
}
// champ 전용 오버라이드 조회(bare key). CUR_CHAMP<0이면 즉시 skip(무비용).
// ★핫패스 early-out만 inline(148+ 사이트); 실제 Vec/HashMap 조회는 out-of-line(코드 폭증 방지).
#[inline] fn tune_champ_lookup(key: &str) -> Option<i64> {
    let c = cur_champ();
    if c < 0 { return None; }   // 공통 경로(players cfg 없거나 미매칭 챔프)=즉시 None, 무비용
    tune_champ_lookup_slow(c, key)
}
#[inline(never)] fn tune_champ_lookup_slow(c: i16, key: &str) -> Option<i64> {
    let p = CHAMP_TABLE.load(Ordering::Acquire);
    if p.is_null() { return None; }
    let t: &ChampTable = unsafe { &*p };
    let r = t.get(c as usize).and_then(|(_, m)| m.get(key).copied());
    if r.is_some() && CHAMP_VERIFY.load(Ordering::Relaxed) { CHAMP_OVHIT.fetch_add(1, Ordering::Relaxed); }
    r
}
// self 엔티티 char_id(+0x5a8)→CHAMP_TABLE 인덱스(캐시). champion_name 매칭. CHAMP_ANY 아니면 즉시 -1.
unsafe fn champ_idx_from_entity(champ: usize) -> i16 {
    if !CHAMP_ANY.load(Ordering::Relaxed) || !ptr_ok(champ) { return -1; }
    let cid = rd_i64(champ + 0x5a8).unwrap_or(-1);
    if cid >= 0 && (cid as usize) < 256 {
        let v = CHAMP_CFG_MAP[cid as usize].load(Ordering::Relaxed);
        if v == -1 { return -1; }        // 매치없음 캐시
        if v > 0 { return v - 1; }
    }
    let p = CHAMP_TABLE.load(Ordering::Acquire);
    if p.is_null() { return -1; }
    let t: &ChampTable = &*p;
    let nm = probe_ent_name(champ);
    let nm = nm.trim();
    let idx = if nm.is_empty() { -1i16 } else {
        t.iter().position(|(k, _)| k.eq_ignore_ascii_case(nm)).map(|i| i as i16).unwrap_or(-1)
    };
    if cid >= 0 && (cid as usize) < 256 {
        CHAMP_CFG_MAP[cid as usize].store(if idx >= 0 { idx + 1 } else { -1 }, Ordering::Relaxed);
    }
    if CHAMP_VERIFY.load(Ordering::Relaxed) && !nm.is_empty() {
        if let Ok(mut seen) = CHAMP_SEEN.lock() {
            if !seen.iter().any(|(n, _)| n == nm) { seen.push((nm.to_string(), idx)); }
        }
    }
    idx
}
// ★★[07-23 stale 수정] ~~0x698~~ = **0.4.x 잔재** → **0x810**(0.5.x). 라이브 결함이었음: team_gate가 쓰레기를 athlete_id로 읽어
//   "우리팀 게이트"가 상시 오작동(내 선수 오버라이드가 적팀에 새거나 내 팀에 미적용) 중이었다.
//   근거(ghidra-re 2026-07-23, 0.5.2 buildid 24310934 · 실바이트 대조 확인): struct B **생성자 = FUN_1422cb050(0x22cb050)** 의
//   필드 3연속 스토어 `[rsi+0x810]←id / [rsi+0x818]←0 / [rsi+0x820]←team`(= `48 89 be 10 08 00 00` / `48 c7 86 18 08 ...` /
//   `48 89 86 20 08 00 00`). 0.4.13_5 동일 생성자 FUN_1418b1c40의 `0x698/0x6a0/0x6a8`과 **1:1 동형** ⟹ 의미 승계.
//   이 3연속 패턴은 각 버전에서 **정확히 1건**(0.5.0_3 0x2079480 · 0.5.1 0x21d9810 · 0.5.2 0x22cb050) = 다중매치 없음.
//   팽창 정체 = `[+0x180]` 인라인 블록 0x210→0x298(+0x88) + 후속 +0xF0 = **+0x178** (0x698+0x178=0x810, 0x6a0/0x6a8→0x818/0x820과 동일 델타).
// ★★[08-06 실측 확정] ~~0x810~~ → **0x800**(0.5.4). 0.5.4에서 athlete 구조체가 −0x10 시프트했다.
//   실측 = 위 3연속 저장 패턴(`48 89 be <ID>` / `48 c7 86 <ID+8>` / `48 89 86 <ID+0x10>`)을
//   0.5.4 exe .text 전역 스캔 → **정확히 1건 @0x13cfa1d, ID=0x800, team=0x810** (v54\athid54.py).
//   ⚠**구 0x810 은 0.5.4에서 team 이다** — 그대로 두면 team_gate 가 athlete_id 자리에 team(0/1/2)을 읽어
//   MY_ATHLETES 와 절대 매칭되지 않는다 ⟹ **선수/클래스 오버라이드가 크래시 없이 조용히 전멸**한다.
//   (같은 결함이 0.4.x→0.5.x 전환 때 0x698 잔재로 한 번 있었다 — 같은 함정의 재발이다.)
const O_ATHLETE_ID: usize = 0x800;   // ★struct B(=p5/athlete)의 athlete_id. 0.4.x=0x698(offrank 2026-07-01: my_distinct=4/allhit=9)
// ★우리팀 게이트: champ 오버라이드를 내 팀 소속(athlete_id ∈ MY_ATHLETES)에만 적용. p5ath=athlete struct(B).
//   p5ath 없으면(0) 보수적으로 미적용(-1)=오버라이드 안 걺(적팀 누수 방지 우선). self_team_only=0이면 게이트 해제.
// ★[08-06 오프셋 판별] 후보 두 자리에서 읽은 값이 실제 athlete_id 집합에 맞는 횟수.
//   0x800 쪽이 크게 이기면 정정이 옳고, 0x810 쪽이 이기면 되돌려야 한다.
static OFF_HIT_800: AtomicU64 = AtomicU64::new(0);
static OFF_HIT_810: AtomicU64 = AtomicU64::new(0);
static OFF_SEEN: AtomicU64 = AtomicU64::new(0);
static OFF_SAMPLE: Mutex<Vec<(u64, u64)>> = Mutex::new(Vec::new());   // (at0x800, at0x810) 표본
static GATE_PASS: AtomicU64 = AtomicU64::new(0);   // 진단(champ_verify): 게이트 통과(내 팀) 횟수
static GATE_BLOCK: AtomicU64 = AtomicU64::new(0);   // 진단: 게이트 차단(타팀/적) 횟수
static GATE_IDS: Mutex<Vec<(u64, bool)>> = Mutex::new(Vec::new());   // 진단: cfg챔프가 본 (athlete_id, 내팀?) 고유쌍
#[inline] unsafe fn team_gate(idx: i16, p5ath: usize) -> i16 {
    // ★[08-06] 오프셋 판별 프로브 — **조기반환보다 앞**에 둔다(오버라이드가 없어도 돌아야 하므로).
    //   읽기 전용이고 champ_verify 켤 때만 돈다. 게임 동작·결정성에 영향 없음.
    if CHAMP_VERIFY.load(Ordering::Relaxed) && ptr_ok(p5ath) {
        let a = ALL_ATHLETES.load(Ordering::Acquire);
        if !a.is_null() && !(*a).is_empty() {
            let v800 = rd_u64(p5ath + 0x800).unwrap_or(u64::MAX);
            let v810 = rd_u64(p5ath + 0x810).unwrap_or(u64::MAX);
            OFF_SEEN.fetch_add(1, Ordering::Relaxed);
            if (*a).contains(&v800) { OFF_HIT_800.fetch_add(1, Ordering::Relaxed); }
            if (*a).contains(&v810) { OFF_HIT_810.fetch_add(1, Ordering::Relaxed); }
            if let Ok(mut g) = OFF_SAMPLE.lock() {
                if g.len() < 12 && !g.iter().any(|&(x, _)| x == v800) { g.push((v800, v810)); }
            }
        }
    }
    if idx < 0 || !SELF_TEAM_ONLY.load(Ordering::Relaxed) { return idx; }
    let p = MY_ATHLETES.load(Ordering::Acquire);
    if p.is_null() || (*p).is_empty() { return idx; }   // ★로스터 미확보(관리화면 방문 전)=게이트 보류→적용(내 오버라이드 안 죽게)
    let aid = if ptr_ok(p5ath) { rd_u64(p5ath + O_ATHLETE_ID).unwrap_or(u64::MAX) } else { u64::MAX };
    let mine = (*p).contains(&aid);
    if CHAMP_VERIFY.load(Ordering::Relaxed) {
        if mine { GATE_PASS.fetch_add(1, Ordering::Relaxed); } else { GATE_BLOCK.fetch_add(1, Ordering::Relaxed); }
        if let Ok(mut g) = GATE_IDS.lock() { if g.len() < 64 && !g.iter().any(|&(a,_)| a == aid) { g.push((aid, mine)); } }
    }
    if mine { idx } else { -1 }
}
// 전용 atomic 노브: per-champ 우선 → per-class → 전역 atomic. (포지션 분기 폐지)
#[inline] fn apos(global: &AtomicI64, key: &str) -> i64 {
    if let Some(v) = tune_champ_lookup(key) { return v; }
    tune_class_lookup(key).unwrap_or_else(|| global.load(Ordering::Relaxed))
}
#[inline] fn apos_u(global: &AtomicU64, key: &str) -> u64 {
    if let Some(v) = tune_champ_lookup(key) { if v >= 0 { return v as u64; } }
    match tune_class_lookup(key) { Some(v) if v >= 0 => v as u64, _ => global.load(Ordering::Relaxed) }
}
// judge 진입 RAII: self로 CUR_POS+CUR_CLASS 세팅 → drop서 이전값 복원(스레드 재사용·중첩 judge 안전).
struct PosGuard(i8, i8, i16);   // (prev_pos, prev_class, prev_champ)
impl Drop for PosGuard { fn drop(&mut self) { set_cur_pos(self.0); set_cur_class(self.1); set_cur_champ(self.2); } }
unsafe fn pos_enter_world(l80: usize, team: i64, selfe: usize, p5ath: usize) -> PosGuard {
    let g = PosGuard(cur_pos(), cur_class(), cur_champ());
    let pos = slot_in_world(l80, team, selfe);
    pos_record(selfe, pos);
    set_cur_pos(pos);
    set_cur_class(class_from_entity(selfe));   // ★클래스도 self 엔티티서 세팅(이름→category 맵)
    set_cur_champ(team_gate(champ_idx_from_entity(selfe), p5ath));   // ★선수(챔피언)별 오버라이드 + 우리팀 게이트(p5ath=athlete struct)
    g
}
// judge 공용(p5=lane ctx, p6=geom handle 규약): l80=*p6, sim=*l80, champ=dd7_slot128(sim, p5+0x818), team=p5+0x820.
//   engage/dd7700/defense 등 동일 규약 judge 진입부서 CUR_POS+CUR_CLASS 세팅 → 그 안 tune() 계수 전부 포지션/클래스 응답. 전부 fault-safe read.
// ★★[07-23 stale 수정·라이브 결함] ~~p5+0x6a8(team)·p5+0x6a0(handle)~~ = 0.4.x 잔재 → **+0x820/+0x818**. `apply_numbers_sp`·
//   `my_f22e80_count`와 동일한 "한 함수만 마이그 누락" 3번째 사례. **영향이 컸던 이유**: 이 함수가 dd7700(disc0/1/3)·engage 판단의
//   포지션/클래스/선수별 오버라이드 컨텍스트를 세팅하는 **단일 진입점**이라, 쓰레기 handle→champ 해석 실패→`pos=-1`·`cls=-1`·`ci=-1`
//   ⟹ ①그 judge들의 `[pos]`/클래스/선수별 오버라이드가 전부 **전역값으로 무단 폴백**(조용한 미적용) ②`pos_record()`도 불발이라
//   `POS_MAP`이 안 채워져 **recall(rc_*) 등 champ-기반 포지션 조회까지 연쇄 무력화**. 전역 레버는 정상이었음(tune 폴백 덕).
unsafe fn pos_enter_p56(p5: usize, p6: usize) -> PosGuard {
    let g = PosGuard(cur_pos(), cur_class(), cur_champ());
    let mut pos = -1i8; let mut cls = -1i8; let mut ci = -1i16;
    if ptr_ok(p5) && ptr_ok(p6) {
        let l80 = rd_u64(p6).unwrap_or(0) as usize;
        if ptr_ok(l80) {
            let sim = rd_u64(l80).unwrap_or(0) as usize;
            let team = rd_i64(p5 + 0x810).unwrap_or(-1);//  ★0.5.4 오프셋 이동 반영
            let champ = dd7_slot128(sim, rd_u64(p5 + 0x818).unwrap_or(0));
            pos = slot_in_world(l80, team, champ);
            pos_record(champ, pos);
            cls = class_from_entity(champ);
            ci = team_gate(champ_idx_from_entity(champ), p5);   // ★우리팀 게이트: p5=athlete struct(B)
        }
    }
    set_cur_pos(pos);
    set_cur_class(cls);
    set_cur_champ(ci);
    g
}
// ★char_id(entity+0x5a8) → 포지션 전역 맵. l80 보유 judge가 채우고(pos_record), l80 없는 judge(recall 등)는 챔피언으로 조회(pos_from_entity).
//   매치내 char_id 고유(프로브 18~27). 0=미상, 저장=pos+1. 라인전 한번 돌면 채워짐(recall은 그 뒤라 OK).
static POS_MAP: [AtomicU8; 256] = [const { AtomicU8::new(0) }; 256];
unsafe fn pos_record(champ: usize, pos: i8) {
    if pos < 0 || !ptr_ok(champ) { return; }
    let cid = rd_i64(champ + 0x5a8).unwrap_or(-1);
    if cid >= 0 && (cid as usize) < 256 { POS_MAP[cid as usize].store((pos + 1) as u8, Ordering::Relaxed); }
}
unsafe fn pos_from_entity(champ: usize) -> i8 {
    if !ptr_ok(champ) { return -1; }
    let cid = rd_i64(champ + 0x5a8).unwrap_or(-1);
    if cid >= 0 && (cid as usize) < 256 { let v = POS_MAP[cid as usize].load(Ordering::Relaxed); if v > 0 { return (v - 1) as i8; } }
    -1
}
// 엔티티(champion)만 있는 judge(recall 등)용: char_id 맵 조회로 CUR_POS 세팅 + 클래스 세팅.
unsafe fn pos_enter_ent(champ: usize, p5ath: usize) -> PosGuard {
    let g = PosGuard(cur_pos(), cur_class(), cur_champ());
    set_cur_pos(pos_from_entity(champ));
    set_cur_class(class_from_entity(champ));
    set_cur_champ(team_gate(champ_idx_from_entity(champ), p5ath));   // ★우리팀 게이트(p5ath=athlete struct, 없으면 보수적 미적용)
    g
}
// ★char_id(entity+0x5a8) → 클래스(0..4) 캐시 조회. 첫 조회시 champion_name→NAME_CLASS(typed 맵) 룩업 후 CLASS_MAP 캐시.
//   NAME_CLASS 미빌드 or 미상 챔프 = -1(전역 튜닝 폴백, 크래시無).
unsafe fn class_from_entity(champ: usize) -> i8 {
    if !ptr_ok(champ) { return -1; }
    let cid = rd_i64(champ + 0x5a8).unwrap_or(-1);
    if cid >= 0 && (cid as usize) < 256 {
        let v = CLASS_MAP[cid as usize].load(Ordering::Relaxed);
        if v == 255 { return -1; }            // 미상 캐시(반복 룩업 방지)
        if v > 0 { return (v - 1) as i8; }
    }
    // ★맵 미빌드면 캐시하지 말고 -1(전역) — 다음 프레임 재시도(맵은 post_update서 빌드). 빌드前 255 캐시=영구 미상 버그 방지.
    if !CLASS_BUILT.load(Ordering::Relaxed) { return -1; }
    // 첫 조회: 이름(String ptr@+0x250,len@+0x258) 읽어 NAME_CLASS 룩업
    let nm = probe_ent_name(champ);
    let cls = if nm.is_empty() { -1 } else {
        match NAME_CLASS.lock() { Ok(g) => g.as_ref().and_then(|m| m.get(&nm).copied()).unwrap_or(-1), Err(_) => -1 }
    };
    if cid >= 0 && (cid as usize) < 256 {
        CLASS_MAP[cid as usize].store(if cls >= 0 { (cls + 1) as u8 } else { 255 }, Ordering::Relaxed);
    }
    // 검증: 새 챔프 탐지 1회 기록(name,class) — 중복 제외
    if CLASS_VERIFY.load(Ordering::Relaxed) && !nm.is_empty() {
        if let Ok(mut seen) = CLASS_SEEN.lock() {
            if !seen.iter().any(|(n, _)| *n == nm) { seen.push((nm, cls)); }
        }
    }
    cls
}
// 챔피언명 읽기(+0x250 char*, null-term). 프로브 전용(≤16회). 실패시 "?".
// 엔티티 champion_name 정확 읽기: String{ptr@+0x250, len@+0x258} (null-term 아님 → len으로).
unsafe fn probe_ent_name(e: usize) -> String {
    let p = rd_u64(e + 0x250).unwrap_or(0) as usize;
    let len = rd_i64(e + 0x258).unwrap_or(0);
    if !ptr_ok(p) || len <= 0 || len > 64 { return String::new(); }
    let mut s = String::new();
    for i in 0..len as usize { let b = rd_u8(p + i); if b == 0 { break; } s.push(b as char); }
    s
}

// ════════════════ 클래스맵 (typed API: db.champion_info(name).category()) — 진짜 범용 ════════════════
// raw 오프셋 불필요. ClientDatabase.available_champions(전 챔프) × champion_info(name).category() → name→class(0..4).
// 바닐라/데이터/네이티브 모드챔프 전부 커버. post_update(InGame, 관리화면)서 1회 빌드 후 static 유지 → judge(detour)가 char_id 캐시로 조회.
const CLASS_NAMES: [&str; 5] = ["melee", "range", "magician", "util", "assassin"];
static NAME_CLASS: Mutex<Option<HashMap<String, i8>>> = Mutex::new(None);   // name→class(0..4), 1회 빌드
static CLASS_BUILT: AtomicBool = AtomicBool::new(false);
static CLASS_MAP: [AtomicU8; 256] = [const { AtomicU8::new(0) }; 256];   // char_id→class+1 캐시(0=미조회, 255=미상). judge 핫패스용.
#[path = "class_capable.rs"] mod class_capable;
#[path = "skip_groups.rs"] mod skip_groups;
use class_capable::CLASS_CAPABLE;
use skip_groups::SKIP_GROUP_KEYS;
static CLASS_ANY: AtomicBool = AtomicBool::new(false);   // cfg에 _class_ 오버라이드 존재 → 맵빌드 필요
// ── 런타임 검증(class_verify=1 → class_verify.txt): 클래스 탐지 + _class_ 오버라이드 실제 적용횟수 ──
const CLASS_KR: [&str; 5] = ["전사", "원거리", "마법사", "전투보조", "암살자"];
static CLASS_VERIFY: AtomicBool = AtomicBool::new(false);
static CLASS_OVHIT: [AtomicU64; 5] = [const { AtomicU64::new(0) }; 5];   // 클래스별 _class_ 값이 실제 적용된 횟수
static CLASS_SEEN: Mutex<Vec<(String, i8)>> = Mutex::new(Vec::new());    // 탐지된 챔프(name,class) 1회씩
static CLASS_VFLUSH: AtomicU64 = AtomicU64::new(0);   // post_update flush 스로틀
// 검증 덤프: 탐지된 챔프별 클래스 + 클래스별 오버라이드 적용횟수.
fn flush_class_verify() {
    let mut s = String::from("=== 클래스 적용 런타임 검증 (class_verify) ===\n");
    s.push_str("[탐지된 챔프 → 클래스]  (class_from_entity 가 champion_name→category 맵에서 판정)\n");
    if let Ok(seen) = CLASS_SEEN.lock() {
        if seen.is_empty() { s.push_str("  (아직 없음 — 경기 시뮬 전이거나 맵 미빌드)\n"); }
        for (nm, c) in seen.iter() {
            let kr = if (0..5).contains(c) { CLASS_KR[*c as usize] } else { "미상(-1, 전역폴백)" };
            s.push_str(&format!("  {:<18} → {}\n", nm, kr));
        }
    }
    s.push_str("\n[클래스별 _class_ 오버라이드 실제 적용 횟수]  (judge가 그 클래스 전용값을 읽은 횟수)\n");
    for i in 0..5 { s.push_str(&format!("  {:<10}({:<8}) : {}\n", CLASS_KR[i], CLASS_NAMES[i], CLASS_OVHIT[i].load(Ordering::Relaxed))); }
    s.push_str("\n※ 적용횟수>0 = 그 클래스 챔프가 cfg의 _class_ 값을 실제 사용 중(=적용 확인). 0 = 그 클래스에 _class_ 키 없음 or 해당 클래스 챔프 미등장.\n");
    // ★[08-07] 마이크로 디투어로 연 노브는 판단 재현을 거치지 않으므로 위 카운터에 안 잡힌다 — 따로 붙인다.
    s.push_str(&micro_summary());
    s.push_str(&nxe_summary());   // ★[08-08] 넥서스 비상 수비 — 판정/발동 횟수와 어느 상황이 채택됐는지
    if let Some(pp) = pth("class_verify.txt") { let _ = fs::write(pp, s.as_bytes()); }
}
// post_update서 typed로 빌드. cfg class_sheet=1이면 class_sheet.txt 덤프.
fn build_class_map(db: &ClientDatabase) {
    if CLASS_BUILT.load(Ordering::Relaxed) { return; }
    let mut map: HashMap<String, i8> = HashMap::new();
    let mut dump = String::from("=== 클래스맵 (typed champion_info().category()) ===\nenum: melee0 range1 magician2 util3 assassin4\n");
    for name in db.available_champions.iter() {
        if let Some(info) = db.champion_info(name) {
            let c: i8 = match info.category() {
                ChampionCategory::Melee => 0,
                ChampionCategory::Range => 1,
                ChampionCategory::Magician => 2,
                ChampionCategory::Util => 3,
                ChampionCategory::Assassin => 4,
            };
            dump.push_str(&format!("  {:<22} = {} ({})\n", name, c, CLASS_NAMES[c as usize]));
            map.insert(name.clone(), c);
        }
    }
    if map.is_empty() { return; }   // available_champions 아직 비었으면 다음 프레임 재시도
    let n = map.len();
    *NAME_CLASS.lock().unwrap_or_else(|e| e.into_inner()) = Some(map);
    CLASS_BUILT.store(true, Ordering::Relaxed);
    dump.push_str(&format!("\n총 {}개 챔프 클래스 등록(NAME_CLASS).\n", n));
    if CLASS_SHEET.load(Ordering::Relaxed) { if let Some(pp) = pth("class_sheet.txt") { let _ = fs::write(pp, dump.as_bytes()); } }
}

// ════════════════ 우리팀 게이트 — 관리팀 id + 내 팀 선발 athlete 집합 ════════════════
// db.player_team_id() = 관리팀 리그 id. db.team(id).last_starting = 선발 5명 athlete_id(개인전술 편집대상과 동일).
// 이적하면 last_starting에서 빠져 자동 제외 → 그 선수 챔피언에 내 오버라이드 안 걸림.
fn publish_idset(cell: &AtomicPtr<IdSet>, prev: &AtomicPtr<IdSet>, set: IdSet) {
    let boxed = Box::into_raw(Box::new(set));
    let old = cell.swap(boxed, Ordering::AcqRel);
    let stale = prev.swap(old, Ordering::AcqRel);
    if !stale.is_null() { unsafe { drop(Box::from_raw(stale)); } }
}
fn refresh_my_team(db: &ClientDatabase) {
    let tid = db.player_team_id();
    MANAGED_TEAM_ID.store(tid as i64, Ordering::Relaxed);
    // 내 팀 선발 athlete_id (Team.last_starting = 개인전술 편집대상 5명)
    let mut my: IdSet = std::collections::HashSet::new();
    let mut diag = String::new();
    if CHAMP_PROBE.load(Ordering::Relaxed) && !TEAM_DIAG_DONE.load(Ordering::Relaxed) {
        diag = format!("player_team_id = {}\nteam_ids(앞10) = {:?}\n", tid,
            db.team_ids().iter().take(10).collect::<Vec<_>>());
    }
    if let Some(team) = db.team(tid) {
        if !diag.is_empty() { diag += &format!("team({}) found. last_starting.len={}\n", tid, team.last_starting.len()); }
        for (i, slot) in team.last_starting.iter().enumerate() {
            if !diag.is_empty() { diag += &format!("  starting[{}] = {:?}\n", i, slot); }
            if let Some(aid) = slot { my.insert(*aid as u64); }
        }
    } else if !diag.is_empty() { diag += &format!("team({}) = None!\n", tid); }
    // 전체 athlete_id 집합(오프셋 탐색용)
    let all: IdSet = db.athlete_ids().iter().map(|&x| x as u64).collect();
    if !diag.is_empty() {
        diag += &format!("all_athletes.len={} (샘플 {:?})\n", all.len(), db.athlete_ids().iter().take(5).collect::<Vec<_>>());
        diag += &format!("my_starting.len={}\n", my.len());
        if let Some(pp) = pth("team_probe.txt") { let _ = fs::write(pp, diag.as_bytes()); }
        TEAM_DIAG_DONE.store(true, Ordering::Relaxed);
    }
    // MY_VEC(순서고정, 비트슬롯 매핑) — 프로브용
    { let mut mv: Vec<u64> = my.iter().copied().collect(); mv.sort_unstable();
      match MY_VEC.lock() { Ok(mut g) => *g = mv, Err(e) => *e.into_inner() = mv } }
    publish_idset(&MY_ATHLETES, &MY_ATHLETES_PREV, my);
    publish_idset(&ALL_ATHLETES, &ALL_ATHLETES_PREV, all);
}
// ★프로브(champ_probe=1): 판단 struct B(p5=plan-context/athlete)를 스캔해 MANAGED_TEAM_ID·MY_ATHLETES와 일치하는 오프셋 자동 태깅 → champ_probe.txt.
//   내 팀 챔프서 일관되게 ==MANAGED 인 오프셋 = team_id 필드. entity(selfe)로 챔프명 읽어 챔프당 1회.
static CHAMP_PROBE: AtomicBool = AtomicBool::new(false);
static MY_VEC: Mutex<Vec<u64>> = Mutex::new(Vec::new());   // 내 팀 athlete_id (순서고정, 비트슬롯 매핑)

// ════════════════ 선수(챔피언)별 cfg 로더 (players/<선수>_<챔피언>.cfg) ════════════════
static CHAMP_MTIME: AtomicU64 = AtomicU64::new(0);   // players 폴더 집계 mtime(파일 추가/수정 감지)
static CHAMP_POLL: AtomicU64 = AtomicU64::new(0);
static CHAMP_VFLUSH: AtomicU64 = AtomicU64::new(0);   // champ_verify 덤프 스로틀
// players/*.cfg 로드 → CHAMP_TABLE 게시. 각 파일 __champion= 헤더로 챔프 매핑(없으면 파일명 마지막 '_' 뒤). mtime 변경시만 재로드.
fn load_champ_cfgs(force: bool) {
    if !force && CHAMP_POLL.fetch_add(1, Ordering::Relaxed) % 30 != 0 { return; }
    let d = match pth("players") { Some(d) => d, None => return };
    let mut agg: u64 = 0;
    let mut files: Vec<PathBuf> = Vec::new();
    if let Ok(rd) = fs::read_dir(&d) {
        for e in rd.flatten() {
            let p = e.path();
            if p.extension().map(|x| x.eq_ignore_ascii_case("cfg")).unwrap_or(false) {
                agg = agg.wrapping_add(mtime_ms(&p)).wrapping_add(1);   // +1: 파일수 변화도 감지
                files.push(p);
            }
        }
    }
    if !force && agg == CHAMP_MTIME.load(Ordering::Relaxed) { return; }
    CHAMP_MTIME.store(agg, Ordering::Relaxed);
    let mut table: ChampTable = Vec::new();
    for p in &files {
        let txt = match fs::read_to_string(p) { Ok(t) => t, Err(_) => continue };
        let mut champ_key = String::new();
        let mut m: TuneMap = HashMap::default();
        for line in txt.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            if let Some((k, v)) = line.split_once('=') {
                let (k, v) = (k.trim(), v.trim());
                if k == "__champion" { champ_key = v.to_string(); continue; }   // 헤더: 매핑용 챔프키
                if k.starts_with("__") { continue; }   // __player 등 메타 무시
                let vv = v.split('#').next().unwrap_or("").trim();
                let parsed = match vv.strip_prefix("0x").or_else(|| vv.strip_prefix("0X")) {
                    Some(h) => i64::from_str_radix(h, 16),
                    None => vv.parse::<i64>(),
                };
                if let Ok(n) = parsed { m.insert(k.to_string(), n); }
            }
        }
        if champ_key.is_empty() {   // 폴백: 파일명 "선수_챔피언"의 챔피언부(마지막 '_' 뒤)
            if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
                champ_key = stem.rsplit_once('_').map(|(_, c)| c.to_string()).unwrap_or_else(|| stem.to_string());
            }
        }
        if !champ_key.is_empty() && !m.is_empty() { table.push((champ_key, m)); }
    }
    let any = !table.is_empty();
    champ_table_publish(table);
    for i in 0..256 { CHAMP_CFG_MAP[i].store(0, Ordering::Relaxed); }   // 재로드→char_id 캐시 무효화
    if let Ok(mut s) = CHAMP_SEEN.lock() { s.clear(); }
    CHAMP_ANY.store(any, Ordering::Relaxed);
}
// 진단(champ_verify=1 → champ_verify.txt): 매칭된 챔프 + 전용값 적용횟수.
fn flush_champ_verify() {
    let mut s = String::from("=== 선수(챔피언)별 오버라이드 런타임 검증 (champ_verify) ===\n");
    // ★사용 가능한 champion_name 키 전체 = players/*.cfg 의 __champion 에 넣을 정확한 문자열들.
    s.push_str("[사용 가능한 champion_name 키 (이 중 하나를 __champion 값으로)]\n");
    match NAME_CLASS.lock() {
        Ok(g) => match g.as_ref() {
            Some(m) if !m.is_empty() => {
                let mut names: Vec<&String> = m.keys().collect();
                names.sort();
                for nm in names { s.push_str(&format!("  {}\n", nm)); }
            }
            _ => s.push_str("  (아직 미빌드 — 경기 화면/시뮬 프레임 1회 필요)\n"),
        },
        Err(_) => s.push_str("  (lock 실패)\n"),
    }
    s.push('\n');
    let tp = CHAMP_TABLE.load(Ordering::Acquire);
    s.push_str("[로드된 players/*.cfg → champion_key]\n");
    if tp.is_null() { s.push_str("  (테이블 미빌드)\n"); }
    else { unsafe { let t = &*tp; if t.is_empty() { s.push_str("  (players 폴더 비어있음)\n"); }
        for (k, m) in t.iter() { s.push_str(&format!("  {:<20} : {} keys\n", k, m.len())); } } }
    s.push_str("\n[경기서 탐지된 챔프 → 매칭 인덱스]  (idx≥0=cfg 매칭, -1=오버라이드 없음)\n");
    if let Ok(seen) = CHAMP_SEEN.lock() {
        if seen.is_empty() { s.push_str("  (아직 없음 — 경기 시뮬 전이거나 CHAMP_ANY off)\n"); }
        for (nm, idx) in seen.iter() { s.push_str(&format!("  {:<20} → {}\n", nm, idx)); }
    }
    s.push_str(&format!("\n champ 전용값 실제 적용 총횟수 = {}\n", CHAMP_OVHIT.load(Ordering::Relaxed)));
    // ★[08-06] athlete_id 오프셋 판별 결과 — 오버라이드 유무와 무관하게 집계된다.
    {
        let seen = OFF_SEEN.load(Ordering::Relaxed);
        let h8 = OFF_HIT_800.load(Ordering::Relaxed);
        let h1 = OFF_HIT_810.load(Ordering::Relaxed);
        s.push_str(&format!("\n[★athlete_id 오프셋 판별 (08-06)]\n  관측 표본 = {}\n  +0x800 이 실제 id = {}\n  +0x810 이 실제 id = {}\n", seen, h8, h1));
        if let Ok(g) = OFF_SAMPLE.lock() { if !g.is_empty() {
            s.push_str("  표본(0x800 / 0x810) = ");
            for (a, b) in g.iter() { s.push_str(&format!("{}/{}  ", a, b)); }
            s.push('\n');
        } }
        s.push_str("  ※ 0x800 쪽이 크게 많으면 08-06 정정(0x810→0x800)이 옷다. 0x810 쪽이면 되돌려야 한다.\n");
        s.push_str("  ※ 둘 다 0인데 표본>0 이면 두 자리 다 athlete_id 가 아니다. 표본이 0 이면 로스터 미확보(관리화면 방문 필요).\n");
    }
    s.push_str(&format!("\n[우리팀 게이트 (self_team_only={})]\n  게이트 통과(내 팀)  = {}\n  게이트 차단(타팀/적) = {}\n",
        SELF_TEAM_ONLY.load(Ordering::Relaxed) as u8, GATE_PASS.load(Ordering::Relaxed), GATE_BLOCK.load(Ordering::Relaxed)));
    s.push_str("※ 통과>0 = 내 팀 챔프는 오버라이드 적용됨. 차단>0 = 같은/다른 챔프라도 타팀 소속은 차단됨(=게이트 작동).\n");
    if let Ok(g) = GATE_IDS.lock() { if !g.is_empty() {
        s.push_str("  [게이트가 본 athlete_id → 내팀?]  ");
        for (a, m) in g.iter() { s.push_str(&format!("{}:{}  ", a, if *m {"O"} else {"X"})); }
        s.push('\n');
    }}
    s.push_str("※ 적용>0 = 해당 챔프가 자기 cfg값을 실제 판단에 사용 중(=적용 확인). 타 챔프는 idx=-1로 무영향.\n");
    if let Some(pp) = pth("champ_verify.txt") { let _ = fs::write(pp, s.as_bytes()); }
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
// ★facet#5 역할 교전임계값 튜닝(검증된 값을 cfg로 조정 = 교전 공격성 다이얼). retreat_engage 내 4개 immediate low byte.
// roll<thr→교전(5), roll>=thr→퇴각(-1). thr↑=교전↑. high 3바이트 0이라 low byte만 패치(원자적).
// ★0.4.13: retreat refactor(0x1d474c0, 프레임오프셋 시프트만)됐으나 교전코어(df0c10→역할임계값→roll게이트) 바이트동일 검증(cmp_region.py).
//   RVA = df0c10_call(0x1fe4d33)+{0x40,0x58,0x6c,0x72}. roll게이트(cmp rax,r14;setge;neg;or 5)도 0.4.12와 동일.
const ROLE_THR: [(usize, u8); 4] = [(0x1d3602b, 100), (0x1d36043, 70), (0x1d36058, 50), (0x1d3605d, 30)]; // (imm32 RVA, 원본) 0.4.13_5(was 0x1fd0546/55e/72/78). 인코딩 cmp-imm32→mov-imm 변경: 100/70/30=mov r14d(imm@+2), 50=mov eax(imm@+1). RETREAT 새바디 df0c10콜 직후 역할래더(role4=100/3=70/2=50/else=30). 상위3바이트0 검증 통과
static ENGAGE_THR_MULT: AtomicI64 = AtomicI64::new(100);  // cfg %, 100=원본(검증), 다른값=공격성 조정
static MOVE_TAG: AtomicI64 = AtomicI64::new(1);       // cfg move_tag: 어느 tag를 Move로 볼지
static MOVE_OFF: AtomicI64 = AtomicI64::new(8);       // cfg move_off: x오프셋(y=x+8). 확인후 맞춤
static OV_ENABLED: AtomicBool = AtomicBool::new(false);
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
static DIAG_DONE: AtomicBool = AtomicBool::new(false);  // plan_base 자동탐지 1회
// dispatch가 준 확정 plan_state 주소들(distinct) — ground-truth set
static BOOTED: AtomicBool = AtomicBool::new(false);

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

// ───────── VEH 안전읽기 (item_editor/scrim 검증본 이식, 2026-06-21 perf 최적화) ─────────
//  rd_*의 per-read VirtualQuery(~1µs syscall)가 judge 핫루프서 호출당 수백회 = dd7700 218µs/call 주범.
//  대안: raw 읽기(rep movsb) + AV는 VEH가 잡아 landing으로 복구(성공경로 syscall 0 ~20ns) → 전 judge 동시 가속.
//  ★캐시 아님(stale 위험 없음): 매 읽기 즉시 실행, 폴트만 VEH로 흡수. cfg fast_read 게이트(off=기존 VirtualQuery).
//  SEH[]: 0=active 1=tid 2=land_rip 3=land_rsp 4=land_rbp 5=code_lo 6=code_hi 7=faults. (멀티모드: 각 모드 자기 SEH범위만 처리.)
#[repr(C)] struct ExceptionRecord { code: u32, _flags: u32, _rec: usize, _addr: usize, _np: u32, _p: u32, _params: [usize; 15] }
type VehHandler = extern "system" fn(*mut ExceptionPointers) -> i32;
extern "system" { fn AddVectoredExceptionHandler(first: u32, handler: VehHandler) -> usize; }
static mut SEH: [u64; 8] = [0u64; 8];
static SEH_INSTALLED: AtomicBool = AtomicBool::new(false);
static SEH_BUSY: AtomicBool = AtomicBool::new(false);
static FAST_READ: AtomicU8 = AtomicU8::new(0);   // cfg fast_read: 0=VirtualQuery / 1=VEH(spinlock 검증본) / 2=VEH(lockless 최속)
// ★[07-16 최적화] readable() 가속: fast_read=2일 때 VirtualQuery(syscall ~수백ns) 대신 VEH 프로브읽기(수ns).
//   프로브 = 그 주소를 실제로 lr_u8 읽기 시도 — 어차피 readable() 직후 같은 주소를 rd_*로 읽으므로 노출등가.
//   (PAGE_GUARD만 의미차: VQ는 무접촉 거부, 프로브는 접촉. 단 rd_* level2가 이미 동일 노출로 안정 운영중.)
//   문제시 fast_guard=0으로 롤백(VirtualQuery 복귀).
static FAST_GUARD: AtomicU8 = AtomicU8::new(1);
// ★[07-16 최적화] 핫패스 apply체인 세대게이트: cfg 리로드時만 apply_*(각각 tune() ~10회) 재실행.
//   기존엔 retreat 매콜 ~20 tune()로 sig만 재계산 → 세대 비교 1회로 대체(콜당 ~1µs 절감).
static CFG_GEN: AtomicU64 = AtomicU64::new(1);    // load_cfg 실리로드마다 +1 (1 시작: 첫 체인실행 보장)
static APPLY_GEN: AtomicU64 = AtomicU64::new(0);
/// ★[08-06 크래시수정] 적용 체인 배타 락. retreat 훅은 rayon 워커 여러 개에서 동시에 불린다 —
///   락이 없으면 두 스레드가 같은 사이트를 함께 패치하고, 한쪽이 VirtualProtect 로 보호를 되돌린
///   직후 다른 쪽이 쓰면서 **AV(write)** 가 난다(실사고: exe+0xCA0008 = th_skill_margin 사이트).
///   ⚠체인이 거기서 죽으면 **뒤쪽 묶음은 적용되지 않는다** — "일부 설정만 안 먹는다"로 보인다.
static APPLY_LOCK: AtomicBool = AtomicBool::new(false);  // apply체인이 마지막 완료(READY 상태 도달)한 세대
// ★[07-16] 행(hang) 진단 워치독 statics (본체 = mem_safety.rs 하단)
static HANG_DIAG: AtomicU8 = AtomicU8::new(0);        // cfg hang_diag: 0(기본)=OFF / 1=감시 ON. ★[07-19] 기본값 1→0: 진단은 opt-in. 구 기본1이면 hang_diag 키 없는 프리셋(8차~11차 전부) 로드 시 진단 부활 → judge_mark 마킹비용 재발
static HANG_SECS: AtomicI64 = AtomicI64::new(8);      // cfg hang_secs: STALL 판정 초(judge 정지+CPU바쁨)
static HANG_RUN_SECS: AtomicI64 = AtomicI64::new(30); // cfg hang_run_secs: RUNAWAY 판정 초(고속시뮬 연속)
static HANG_RUN_RATE: AtomicI64 = AtomicI64::new(5000); // cfg hang_run_rate: RUNAWAY로 볼 judge 콜/s (라이브 관전≪5000≪백그라운드 시뮬)
static HB_JUDGE: AtomicU64 = AtomicU64::new(0);       // judge 훅 하트비트(4훅 공용)
static HANG_WD_STARTED: AtomicBool = AtomicBool::new(false);
static INFLIGHT_SITE: [AtomicU64; 64] = [const { AtomicU64::new(0) }; 64];
static INFLIGHT_TS:   [AtomicU64; 64] = [const { AtomicU64::new(0) }; 64];
static INFLIGHT_TID:  [AtomicU64; 64] = [const { AtomicU64::new(0) }; 64];
static INFLIGHT_NEXT: AtomicUsize = AtomicUsize::new(0);
thread_local! { static INFLIGHT_SLOT: std::cell::Cell<usize> = const { std::cell::Cell::new(usize::MAX) }; }
// ★[07-16] 활동창 프로파일러 statics (본체 = mem_safety.rs 하단). ⚠배포(zip) cfg에선 adv_prof=0.
static ADV_PROF: AtomicU8 = AtomicU8::new(0);          // cfg adv_prof: 0(기본)=OFF / 1=활동창 프로파일 ON. ★[07-19] 기본값 1→0: 프로파일러는 활동중 100ms마다 전 활성스레드 Suspend/GetThreadContext/Resume — rayon 워커를 시뮬 내내 주기적으로 정지시킴(순수 손실). adv_prof 키 없는 프리셋 로드 시 자동 ON되던 것을 차단(진단=opt-in)
static ADV_PROF_MIN: AtomicI64 = AtomicI64::new(3000); // cfg adv_prof_min: 이 ms 미만 활동창은 로그 생략
static ADV_PROF_SEG: AtomicI64 = AtomicI64::new(15000); // cfg adv_prof_seg: 끝나지 않는 창의 중간 스냅샷 간격(ms)
static SITE_CNT: [AtomicU64; 5] = [const { AtomicU64::new(0) }; 5];   // judge 사이트별 누적 콜수(창 델타용)
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

// ★읽기 경로 직접 벤치(cfg read_bench=1, 1회): 확실히 readable한 주소를 각 경로로 N회 읽어 ns/read 측정.
//   게임 페이즈 무관 = perf.txt(첫50000콜창)의 경기페이즈 오염 없이 원시 per-read 비용 ground-truth.
static BENCH_DONE: AtomicBool = AtomicBool::new(false);

// ══════ 크래시 로거: 미처리 치명적 예외(AV 등, 프로세스 죽기 직전)만 crash_log.txt에 폴트위치 기록 (유저 원격진단) ══════
//   SetUnhandledExceptionFilter = "진짜 미처리"일 때만 호출 → false-positive 없음. VEH(seh_veh)와 별개.
//   ★크래시 문맥이라 alloc/format!/lock 금지(§3) → 고정 스택버퍼+수동 hex+WriteFile. 경로는 init서 UTF-16 프리컴퓨트.
extern "system" {
    fn SetUnhandledExceptionFilter(f: usize) -> usize;
    fn CreateFileW(name: *const u16, access: u32, share: u32, sec: usize, disp: u32, flags: u32, tmpl: usize) -> isize;
    fn WriteFile(h: isize, buf: *const u8, len: u32, written: *mut u32, ov: usize) -> i32;
    fn SetFilePointer(h: isize, lo: i32, hi: usize, method: u32) -> u32;
    fn CloseHandle(h: isize) -> i32;
    fn GetCurrentProcessId() -> u32;
}
static CRASH_MOD_BASE: AtomicU64 = AtomicU64::new(0);
static CRASH_PREV: AtomicU64 = AtomicU64::new(0);
static CRASH_INSTALLED: AtomicBool = AtomicBool::new(false);
static CRASH_LOGGED: AtomicBool = AtomicBool::new(false);
static mut CRASH_PATH: [u16; 600] = [0u16; 600];   // init서 채움(mod dir\crash_log.txt), 크래시 시 read-only
// ★★[2026-07-14] panic hook — UEF(SetUnhandledExceptionFilter)가 **못 잡는** 크래시 경로를 규명하기 위함.
//   근거: Rust는 panic이 `extern "C"` 경계(=우리 디투어)를 넘으면 **abort(__fastfail)** → UEF 우회 → crash_log.txt 안 남음.
//   실제 관측(07-14 크래시): crash_log 없음 + disc19 재현부 panic 0(rc=3만) → abort 경로 의심.
//   panic hook은 **unwind/abort 이전에** 호출되므로 catch_unwind로 잡히는 panic까지 전부 포착 → 발생지점 확정 가능.
//   ⚠훅 본문은 정상 스택에서 도는 일반코드라 alloc/format! 허용(VEH 핸들러와 다름).
static PANIC_N: AtomicU64 = AtomicU64::new(0);
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
    let team = rd_u64(p5 + 0x810).unwrap_or(2);   // 0.5.0(was 0x6a8, SimState +0x178)  ★0.5.4 오프셋 이동 반영
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
    let zbase = geo + tu*0x2e8;   // 0.5.0(was 0x228, geom stride +0xc0). zone 헤드/슬롯(0xf8/0xf9) 불변
    if rd_i32(zbase + slot_off).unwrap_or(0) != 1 { return false; }
    let handle = rd_u64(zbase + slot_off + 8).unwrap_or(0);
    if rhd0 == 0 || rhd1 == 0 { return false; }
    // 0.5.0 retreat: rvt+0x138 resolver(0x21aebf0)=dd7_slot128 동일 4단 chase(handle→entity). shadow-call 제거=AV방지(순수재현). rhd0=self holder.
    let tgt = dd7_slot128(rhd0, handle);
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
    // ★0.5.0_2 재작성(FUN_142241710 PATH B/A): dispatch code renumber ZONE 3→9 · STAND 8→10 · RECALL 7→11.
    //   구 dispatch_stand_attempt/shadow_fa1ea0 단순매핑(8/3) 폐기 → 2단 PATH 게이트 + resolver(my_fa1ea0=FUN_1422316f0).
    if cvar6 == 1 { return 11; }                  // RECALL (was 7)
    if cept == 1 || ce1 == 1 { return 7; }        // ★cept/ce1 0.5.0 재검증 필요(dispcmp game=-1/3), 임시 유지(별건)
    if cvar6 == 0 {
        // PATH B: zone slot(5, stride 0x20) flag==0 && dir==postag, 또는 za_sel>=-2. ★0.5.0: postag=2→za20 / postag=0→za70 (swap)
        let za_sel = if postag == 2 { za20 } else { za70 };
        let mut path_b = za_sel >= -2;
        if !path_b {
            for k in 0..5usize {
                if rd_u8(zone + 0xf8 + k * 0x20) == 0 && (rd_u8(zone + 0xf9 + k * 0x20) as i64) == postag { path_b = true; break; }
            }
        }
        if !path_b { return 9; }                  // ZONE (was 3), arg=postag
        // PATH A: zone slot flag==0 && val==1, 또는 za48=[zone+0x48]>=-2
        let mut path_a = za48 >= -2;
        if !path_a {
            for k in 0..5usize {
                if rd_u8(zone + 0xf8 + k * 0x20) == 0 && rd_u8(zone + 0xf9 + k * 0x20) == 1 { path_a = true; break; }
            }
        }
        if !path_a { return 9; }                  // arg=1
        // push10 시도: resolver=my_fa1ea0(=FUN_1422316f0, 288/288 DIFF=0). match→STAND(10) / 0xff실패→push없음=engage(-99)
        if shadow_fa1ea0(rh, geo, p5, postag) { return 10; }   // STAND (was 8)
        return -99;                               // resolver 실패 = engage 기본동작
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
// ★로컬 seed RNG (rand-0.8.5 StdRng::seed_from_u64) — disc4 A* branch A far 지터·FUN_142377e00 조기탈출용(sim-state 아닌 seed 생성).
//   seed_from_u64=PCG-XSH-RR로 32B key 생성(mul 0x5851f42d4c957f2d/inc 0xa17654e46fbe17f3). 이후 ChaCha12(counter 0, nonce 0). next_u64=2 u32 LE. gen_range=Lemire(rng_gen_range와 동일식).
struct LocalRng { key: [u32; 8], counter: u64, buf: [u32; 64], idx: usize }
impl LocalRng {
    fn seed_from_u64(seed: u64) -> Self {
        let mut st = seed;
        let mut key = [0u32; 8];
        for k in 0..8 {
            st = st.wrapping_mul(6364136223846793005).wrapping_add(11634580027462260723);   // 0x5851f42d4c957f2d / 0xa17654e46fbe17f3
            let xorshifted = (((st >> 18) ^ st) >> 27) as u32;
            let rot = (st >> 59) as u32;
            key[k] = xorshifted.rotate_right(rot);
        }
        LocalRng { key, counter: 0, buf: [0; 64], idx: 64 }
    }
    #[inline] fn next_u32(&mut self) -> u32 {
        if self.idx >= 64 {
            unsafe { chacha12_4block(&self.key, self.counter, 0, self.buf.as_mut_ptr()); }
            self.counter = self.counter.wrapping_add(4);
            self.idx = 0;
        }
        let v = self.buf[self.idx]; self.idx += 1; v
    }
    #[inline] fn next_u64(&mut self) -> u64 { let lo = self.next_u32() as u64; let hi = self.next_u32() as u64; lo | (hi << 32) }
    // gen_range(lo..=hi) 부호있는(unsigned wrapping range), Lemire widening-reject(rng_gen_range 동일 로직)
    fn gen_range_i64(&mut self, lo: i64, hi: i64) -> i64 {
        let range = (hi.wrapping_sub(lo) as u64).wrapping_add(1);
        if range == 0 { return self.next_u64() as i64; }
        let bits = 63 - range.leading_zeros() as u64;
        let zone = (range << (63 - bits) as u32).wrapping_sub(1);
        let mut guard = 0;
        loop {
            guard += 1; if guard > 64 { return lo; }
            let raw = self.next_u64();
            let prod = (raw as u128).wrapping_mul(range as u128);
            if zone < prod as u64 { continue; }
            return lo.wrapping_add((prod >> 64) as i64);
        }
    }
}
// ★write-back: 게임 RNG state를 fcd980과 동일하게 전진(되쓰기). 대체모드 RNG 동기화 핵심. 게임함수 콜 0.
//   read-only sim으로 최종 buf/idx/refills 구한 뒤 buf(refill시)+idx+counter 되쓰기. (step3 대체서 사용; step2 검증선 미호출)
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
fn append_log(s: &str) { if !LOG_ON.load(Ordering::Relaxed) { return; } if let Some(p) = pth("tfm2_ai_adjust.txt") {
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = write!(f,"{}",s); } } }
fn fresh_log(s: &str) { if !LOG_ON.load(Ordering::Relaxed) { return; } if let Some(p) = pth("tfm2_ai_adjust.txt") { let _ = fs::write(p, s); } }
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
// roster 포인터 중 첫 유효 엔티티 = 배열 진입 seed (미니언이라도 OK)
// 챔프 10명: (team(+0x8), entity). 챔프후보 중 "최장 연속 런"(stride 0x6a8) = 진짜 10명.
// (hp700/speed800 특수미니언은 챔프블록과 떨어져 고립 → 런에서 탈락)

// ── §E combat_effective_damage 재구현 (디컴파일 정확 이식). 검증: 게임함수와 비교 ──
// vt 글로벌(스탯-accessor vtable). atk vt[0x30]=스탯getter, vt[0x38]=데미지시트, tgt vt[0x30]=방어getter.
const COEF_MULT_PCT: i64 = 100;   // ★데미지 coef 배수%. 100=원본검증(dmgcmp OK 판정), 150=override데모. 검증위해 100.
// ★ 진단 하네스 스위치: false=실게임 안전, true=디버깅(데모/리플레이).
// TTD 검증 = 리턴-훅(실제 rax 반환 캡처). 재호출(g) 제거 → 재진입 부작용/크래시 원인 제거.
const HARNESS_ON: bool = true;

// ── TTD 리턴-훅 (리턴주소 스왑 트램폴린) ──
// 진입 훅에서: 입력 스냅샷 → my_ttd 계산 → 리턴주소를 thunk로 교체 + 프레임 push.
// 함수가 ret하면 thunk로 진입 → rax=실제 게임 TTD → ttd_return에서 game vs mine 로깅 → orig_ret로 복귀.
static RET_THUNK: AtomicUsize = AtomicUsize::new(0);   // 공용 리턴 thunk 코드 주소
static RET_STACK: Mutex<Vec<RetFrame>> = Mutex::new(Vec::new());
static RE_FILE_INIT: AtomicBool = AtomicBool::new(false);
type Getter1 = unsafe extern "C" fn(usize) -> i64;  // 역할게터 vt[0x68](data)->role
// 함수의 rdx 반환을 받는 시wim: shim(target,a,b,c,d) → target(a,b,c,d) 호출 후 rdx 리턴
static SHIM_RDX: AtomicUsize = AtomicUsize::new(0);
static SHIM_BOTH: AtomicUsize = AtomicUsize::new(0);   // ★단일호출 2값캡처(소환수 비멱등 게터 대응)
// ★engage pre-gate(0x2080760) 호출 shim: pregate_shim(target, p1, p2, p5, p6, arg9)→al.
//   0x2080760(rcx=p1, rdx=p2, r8=0, r9=0, [rsp+0x20]=p5, [rsp+0x28]=p6, [rsp+0x30]=arg9, [rsp+0x38]=0).
// ★[제거됨] build_pregate_shim/PREGATE_SHIM: 0x2080760을 머신코드 thunk로 직접호출했으나
//   스택인자 오프셋 off-by-8 버그(arg5/6를 8B 낮게 읽음)로 게임에 garbage roster ptr 전달→freeze.
//   완전대체 원칙(게임함수 호출X)에 따라 my_pregate(순수Rust 재현)로 대체. shim 영구 폐기.
// r9(소환수 게터가 읽는 테이블) 명시 버전. ~~엔게이지TTD=ability_table(0x3599b30), disc4=ATK_VT(0x35e4d00)~~
//   → ★**둘 다 0.5.2에선 `0x381e1e0` 하나로 통합**(ghidra-re 2026-07-23). 상세 = 아래 화이트리스트 주석.
unsafe fn probe_basedmg_r9(e: usize, local_80: usize, exe: usize, r9_addr: usize) -> (i64, i64) {
    // ★★[07-22] **desc 화이트리스트 가드** — 이 함수는 r9_addr를 **vt+0x28 다형 shadow-call의 this**로 쓴다.
    //   stale/미검증 주소가 들어오면 그 자리의 임의 바이트(문자열 등)를 vtable로 삼아 호출 → **non-canonical → AV**.
    //   실제 사고: `RVA_C8C_DMG_SHEET`가 0.5.2 미마이그(0.5.1값 `0x3830c58`=문자열 블롭) 상태로 방치돼 **disc14 크래시 2·3차**.
    //   ⟹ **0.5.2 검증 완료된 desc만 통과**시키고 나머지는 기존 "실패" 반환값 `(-1,-1)`로 조기탈출(호출부는 전부 이 값을
    //      dmg=0/skip으로 처리하므로 기능적으로 안전, 정확도만 저하).
    //   ~~⚠잠복 지뢰 2건(미확정이라 차단): `0x35e4d00`(ATK_VT) / `0x3599b30`(ability_table)~~
    //   → ✅**해소(ghidra-re 2026-07-23, 0.5.2)**: 둘 다 **`0x381e1e0`(= `RVA_C8C_DMG_SHEET`와 동일 값)** 으로 통합됐다.
    //     ① **ATK_VT = `0x381e1e0` 확신도 HIGH(실바이트 확인)**: 0.4.13_5 `FUN_14206e530`의 0.5.2 대응 = **`FUN_141b93830`(0x1b93830)**
    //        (imm 지문 + 엔티티 memdisp 카운트 전량 일치 + athlete 0x6a0/0x6a8→0x818/0x820 이동). 그 안의 두 사이트
    //        `0x1b93bb6`/`0x1b94354` = `4c 8d 0d ...`(lea r9 → RVA 0x381e1e0) 직후 `41 ff 52 28`(call [r10+0x28]) — 우리 재현과 인자열 일치.
    //        ⛔과거 오답 `0x38832a8`은 무관 함수 `FUN_142031110` 전용 클론.
    //     ② **ability_table = 별개 테이블이 아님(확신도 MED)**: `0x3599b30`은 **0.4.13_4에만** 존재했고 그 버전에서 플랜-AI 3함수가
    //        전부 그 하나를 공유 = ATK_VT와 같은 논리 desc의 CGU 클론. 0.5.2에서 `call [+0x28]`에 쓰이는 desc는 5개뿐인데
    //        (`0x381e1e0`·`0x38832a8`·`0x38a22b0`·`0x38c61b0`·`0x38d1918`) **slot0(drop) 빼고 size 0x6a8·align 8·메서드 7슬롯이 전부 바이트 동일**
    //        ⟹ 우리가 쓰는 slot `0x30`(=`0x141bebd80`)도 동일 = 어느 클론이든 동작·위험 동등. ⟹ **별도 상수 유지 근거 없음 = 통합**.
    //        (0.5.2 "엔게이지 TTD"의 정확한 call 사이트 특정은 못 함 — 모드에 그 재현 코드 자체가 없어 대조 기준 부재. 기능 등가값이라는 뜻.)
    //   desc 추가 시 **반드시 0.5.2 원본 호출부에서 실측 확인 후**에만 등록할 것(주소가 그럴듯해 보인다는 이유로 넣지 말 것).
    {
        let base = exe_base();
        if base == 0 { return (-1, -1); }
        // ★0.5.3 갱신(2026-07-29). ~~0.5.2 [0x381e1e0, 0x38d1918]~~ → 아래. 화이트리스트를 안 옮기면 **모든 호출이 차단**돼
        //   dmg=0 퇴화(크래시는 없음)하고, 반대로 구값을 그대로 통과시키면 0.5.3의 그 주소는 다른 데이터라 **AV**가 난다.
        //   둘 다 desc sanity {size=0x6a8, align=8, vt+0x30=**0xc7ead0**} 실측 통과 — 근거는 rva_054.rs 해당 상수 주석.
        //   ⚠0.5.4에서 vt+0x30 이 0xc51bc0 → 0xc7ead0 으로 옮겼다. desc 주소와 **같이** 갱신해야 한다.
        const OK_DESC_054: [usize; 2] = [0x3288e48, 0x327fba0];   // C8C(확정 08-05) / DISC7(확정 08-05)
        if !OK_DESC_054.contains(&r9_addr.wrapping_sub(base)) { return (-1, -1); }
    }
    let _ = exe;
    if !ptr_ok(e) { return (-1, -1); }
    let v480 = rd_u64(e + 0x480).unwrap_or(0) as usize;
    if !ptr_ok(v480) { return (-1, -1); }
    let inner = rd_u64(v480 + 0x10).unwrap_or(0) as usize;
    let buf = rd_u64(e + 0x478).unwrap_or(0) as usize;
    let aligned = (inner.wrapping_sub(1) & !0xf).wrapping_add(buf).wrapping_add(0x10);
    let gptr = rd_u64(v480 + 0x28).unwrap_or(0) as usize;
    // ★r9 = 호출부별 테이블. 챔피언 게터는 rcx만 쓰지만 소환수 게터는 r9을 읽음 → stale 값이면 소환수 base 깨짐(DIFF).
    let vt = r9_addr;
    if !ptr_ok(gptr) || !ptr_ok(aligned) { return (-2, -2); }
    // ★[07-15 하드닝] 사전게이트: gptr(=CALL할 game 게터)가 실제 .text 실행가능 코드가 아니면 CALL 금지.
    //   비정착 엔티티(비챔피언/사망/전이중)의 skill obj는 gptr에 garbage → 하드웨어 AV(catch_unwind 불가)의 정체.
    //   실패 시 (-1,-1)=기존 실패 센티널(caller가 0 처리) — 유효 게터는 항상 .text이므로 정상경로 무영향.
    if !code_ptr_ok(gptr) { return (-1, -1); }
    // ★단일호출 2값캡처(소환수 비멱등 게터 정확): 게터 1회 → (rax,rdx)=out[0],out[1].
    let both = SHIM_BOTH.load(Ordering::Relaxed);
    if both != 0 {
        let mut o = [0i64; 2];
        let s: ShimBoth = core::mem::transmute(both);
        s(o.as_mut_ptr() as usize, gptr, aligned, local_80, e, vt);
        return (o[0], o[1]);
    }
    // ★★[07-29 결정성 수정] 폴백(구 2호출) **비활성화**. 이 게터는 주석대로 **소환수에서 비멱등**(호출 자체가 게임 상태 전진)인데
    //   both-shim 실패 시 **같은 게터를 2회** 호출해 부작용을 2배로 일으켰다. 배경 워커와 관전 워커의 shim 준비 상태·호출 횟수가
    //   갈리면 게임 상태가 두 sim서 달라진다(판단 출력엔 안 드러나므로 계측에도 안 잡힘) ⟹ 배경≠관전 발산 잠재원.
    //   both-shim이 없으면 **정확도 대신 결정성**을 택해 미지값 반환(호출자는 -3/-1을 "미지"로 처리해 안전 폴백).
    let _ = (gptr, aligned, local_80, e, vt);
    (-3, -3)
}
// ★"능력2 없음" 빈 디스크립터 동등 const(+0x30 i32=-1, 게임 .rdata DEFAULT_AB2 대체 — churn 소멸).
//   genbuild_repro(my_f80320 skip82 / my_20c0690 slot3)가 사용: +0x30 먼저 읽고 -1이면 즉시 중단.
#[repr(C, align(8))]
struct DefAb2([u32; 16]);   // 0x40바이트 (idx12 = +0x30)
static DEFAULT_AB2_EMPTY: DefAb2 = DefAb2([0,0,0,0, 0,0,0,0, 0,0,0,0, 0xFFFF_FFFF,0,0,0]);
#[inline] fn default_ab2_ptr() -> usize { DEFAULT_AB2_EMPTY.0.as_ptr() as usize }

unsafe fn my_combat_dmg(atk: usize, tgt: usize, base: i64, dtype: u32, flag: i32, exe: usize) -> i64 {
    if flag != 0 && flag != 1 { return base; }
    // ★풀재현(2026-06-18): vtable getter 호출 제거 → 엔티티 오프셋 직접읽기(우리 손으로 다 구현).
    //   실측 getter: +0x38(0x18ba1c0)=`lea rax,[e+0x358]`(계수시트), +0x30(0x18ba210)=`copy e+0x600..0x640→out`.
    //   ⇒ sheet=e+0x358, 유효스탯블록=e+0x600(tb[0x10]=e+0x610,[0x18]=e+0x618방어,[0x20]=e+0x620마저).
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
# capture : 1=하네스 캡처 ON(RE). 원하는 경기 들어가서 1로 저장하면 그때부터 캡처
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

# ===== disc19 넥서스 방어 AI 성향 튜닝 =====
# ★[07-15] d19i_enable=1 이어야 아래 판단상수가 게임에 적용됨(로드시 disc19 코드 imm 바이트패치, disc19 전용).
#   0=게임 원본(무개입). 출력축(능력발행)·threat식·compFlag는 원본 그대로(다른 AI 무영향).
# --- 후퇴 결정(Gate2): threat를 현재HP%로 본 tr이 문턱 초과면 후퇴검토. HP 낮을수록 낮은 문턱 ---
# d19_retreat_hp : 후퇴 HP%문턱(45). 이 위로만 전투속행. 높이면 수비적(높은HP에도 후퇴)
# d19_sev_ratio_0: HP무관 기본 위협비율 문턱(49). 낮추면 작은 위협에도 후퇴(수비적)
# d19_sev_ratio_1/2/3: HP단계별 문턱(29/17/9). HP 낮을수록 이 값 적용
# d19_sev_hp_1/2/3: 위 단계 전환 HP% 경계(66/41/26)
# d19_ally_hp    : ally넥서스 위기 판정 HP%(50). 아군넥서스 이 아래면 지원 판단
# --- phase 진입(게임 진행도) ---
# d19_phase_threat: 위협판정 시작 phase(30). 낮추면 더 일찍 방어태세
# d19_phase_ally  : ally지원 판정 phase(39)
# --- ⚠아래 threat 계수는 공유함수(FUN_1420a3fd0, 6 AI 공유)라 현재 미적용(관찰전용). 켜면 전체 AI 영향이라 보류 ---
# d19_threat_mult / d19_range_* : 관찰용(게임 미반영)
d19i_enable = 0
d19_retreat_hp = 45
d19_sev_ratio_0 = 49
d19_sev_ratio_1 = 29
d19_sev_ratio_2 = 17
d19_sev_ratio_3 = 9
d19_sev_hp_1 = 66
d19_sev_hp_2 = 41
d19_sev_hp_3 = 26
d19_ally_hp = 50
d19_phase_threat = 30
d19_phase_ally = 39
d19_threat_mult = 100
d19_range_atkme = 100
d19_range_bld = 60
d19_range_other = 40
d19_range_idle = 80
";
fn mtime_ms(p: &PathBuf) -> u64 {
    fs::metadata(p).and_then(|m| m.modified()).ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map(|d| d.as_millis() as u64).unwrap_or(0)
}
static CFG_POLL_CTR: AtomicU64 = AtomicU64::new(0);   // ★성능: load_cfg 매프레임 stat 스로틀
fn load_cfg(force: bool) -> bool {
    let p = match pth("tfm2_ai_adjust.cfg") { Some(p) => p, None => return false };
    // ★성능(2026-06-22): post_update가 매 UI프레임 load_cfg(false) 호출 → fs::metadata(mtime)+exists syscall이 디스크 바쁠때 메인스레드 히치(관전 멈춤#2). force 아니면 30프레임당 1회만 stat. 핫리로드 ~0.5s 지연=무해, cfg값 동일.
    if !force && CFG_POLL_CTR.fetch_add(1, Ordering::Relaxed) % 30 != 0 { return false; }
    if !p.exists() { let _ = fs::write(&p, CFG_TEMPLATE); }
    let mt = mtime_ms(&p);
    if !force && mt == CFG_MTIME.load(Ordering::Relaxed) { return false; }
    CFG_MTIME.store(mt, Ordering::Relaxed);
    CFG_GEN.fetch_add(1, Ordering::Relaxed);   // ★[07-16] 실리로드 세대+1 → retreat 핫패스 apply체인 1회 재실행
    let txt = match fs::read_to_string(&p) { Ok(t) => t, Err(_) => return false };
    let mut new_tune: TuneMap = HashMap::default();   // ★lock-free + FNV 해셔: 파싱 누적 후 일괄 게시
    // ★[수정 07-31] subplan별 임계는 "키가 있을 때만" 덮어써서 **한 번 설정하면 되돌릴 수 없었다**
    //   (줄을 지우거나 -1로 바꿔도 구값·ANY=true가 게임 재시작까지 잔존 = 핫리로드 무효).
    //   txt를 이미 읽은 뒤라 여기서 초기화해도 실패 경로로 새는 일 없음. 매 리로드 = 파일 내용 그대로 재구성.
    for a in NUMBERS_THREAT_SP.iter() { a.store(-1, Ordering::Relaxed); }
    NUMBERS_THREAT_SP_ANY.store(false, Ordering::Relaxed);
    for line in txt.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        if let Some((k, v)) = line.split_once('=') {
            let (k, v) = (k.trim(), v.split('#').next().unwrap_or("").trim());
            let k: &str = match k {
                // ★[08-05] 옛 키 이름 → 정본 흡수.
                //   정본 = ec_(에픽) · sn_(세르펜) · nxd_(넥서스방어) · nx_repl · nx_*(넥서스 공수).
                //   근거 = 08-03 RE(plan-vs-subplan 두 enum 분리) + 08-05 감사(패닉 Location 파일명·Plan JT).
                //   ⚠08-05 오후에 잠깐 ep_/sp_/sr_ 로 바꿨다가 되돌렸다 — 그 사이 저장된 cfg도 읽히게 흡수한다.
                //   실제로 이름이 틀렸던 것은 oi_*(넥서스인데 objective 접두사) 하나뿐이었다.
                "ep_home_hi"|"sp_home_hi" => "sn_home_hi",
                "ep_home_lo"|"sp_home_lo" => "sn_home_lo",
                "ep_home_x1"|"sp_home_x1" => "sn_home_x1",
                "ep_home_y1"|"sp_home_y1" => "sn_home_y1",
                "ep_hp_crit"|"sp_hp_crit" => "sn_hp_crit",
                "ep_self_hp"|"sp_self_hp" => "sn_self_hp",
                // ★[08-07] 중복 사이트 정리 — ex_wait_* 와 lw_* 가 같은 주소를 각각 패치하고 있었다.
                //   lw_* 를 정본으로 삼고 옛 이름은 알리아스로 살린다(기존 cfg 무손실).
                "ex_wait_dist" => "lw_wait_dist", "ex_wait_back" => "lw_back",
                "sr_near_dist"  => "nxd_near_dist",  "sr_p3_gate"   => "nxd_p3_gate",
                "sr_pred_dist"  => "nxd_pred_dist",  "sr_prog_crit" => "nxd_prog_crit",
                "sr_prog_low"   => "nxd_prog_low",   "sr_ref_hp"    => "nxd_ref_hp",
                "sr_repl"|"nx1617_repl" => "nx_repl",
                "oi_enable" => "nx_enable", "oi_dn_count_gate" => "nx_dn_count_gate",
                "oi_dn_nexus_hp" => "nx_dn_nexus_hp", "oi_dn_hp_crit" => "nx_dn_hp_crit",
                "oi_dn_hp_low" => "nx_dn_hp_low", "oi_dn_near_dist" => "nx_dn_near_dist",
                "oi_dn_pred_dist" => "nx_dn_pred_dist", "oi_dn_lane_margin" => "nx_dn_vision_mem",
                "oi_an_count_gate" => "nx_an_count_gate", "oi_an_finish_hp" => "nx_an_finish_hp",
                "oi_an_cull_dist" => "nx_an_cull_dist",
                other => other,
            };   // ★07-11 파서버그 수정: 트레일링 #주석 미제거 → 토글 v=="1" 오판(mp_repl=1이 주석 달리면 false로 저장되던 원인). 타 파서 4곳과 동일화.
            match k {
                "enabled" => OV_ENABLED.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed),
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
                "engage_repl" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !ENGAGE_REPL.load(Ordering::Relaxed) { ENGAGE_REPL_N.store(0, Ordering::Relaxed); ENGAGE_REPL_PASS.store(0, Ordering::Relaxed); }
                    ENGAGE_REPL.store(on, Ordering::Relaxed);
                }
                "numbers_margin" => { if let Ok(n) = v.trim().parse::<i64>() { NUMBERS_MARGIN.store(n, Ordering::Relaxed); } }   // ★인원수 회피: 0=off, ≥1=적−아군≥n이면 후퇴
                "numbers_range" => { if let Ok(n) = v.trim().parse::<u64>() { if n > 0 { NUMBERS_RANGE.store(n, Ordering::Relaxed); } } }   // ★전력카운트 반경(한타)
                "numbers_range_move" => { if let Ok(n) = v.trim().parse::<i64>() { NUMBERS_RANGE_MOVE.store(n, Ordering::Relaxed); } }   // ★전력카운트 반경(라인전) -1=한타값 따름
                "numbers_threat" => { if let Ok(n) = v.trim().parse::<i64>() { NUMBERS_THREAT.store(n, Ordering::Relaxed); } }   // ★전력승산 임계(한타) 0~100(≥승산이면 후퇴)
                "numbers_min_enemy" => { if let Ok(n) = v.trim().parse::<i64>() { NUMBERS_MIN_ENEMY.store(n.max(1), Ordering::Relaxed); } }   // ★머릿수게이트(한타): 근처 적 ≥n 일때만 force후퇴. 1=현행, 2=1:1 제외.
                "numbers_min_enemy_move" => { if let Ok(n) = v.trim().parse::<i64>() { NUMBERS_MIN_ENEMY_MOVE.store(n, Ordering::Relaxed); } }   // ★머릿수게이트(라인전) -1=한타값 따름
                "numbers_threat_move" => { if let Ok(n) = v.trim().parse::<i64>() { NUMBERS_THREAT_MOVE.store(n, Ordering::Relaxed); } }   // ★전력승산 임계(라인전). -1=한타값 따름, 0=라인전 후퇴 안함(딜교 살림·와리가리 없음).
                "ally_tower_hp" => { if let Ok(n) = v.trim().parse::<i64>() { ALLY_TOWER_HP.store(n.clamp(0,100), Ordering::Relaxed); } }   // ★포탑 HP 전력가중(한타) 0~100
                "ally_tower_hp_move" => { if let Ok(n) = v.trim().parse::<i64>() { ALLY_TOWER_HP_MOVE.store(n, Ordering::Relaxed); } }   // ★포탑 HP 전력가중(라인전) -1=한타값 따름
                "ally_tower_dps" => { if let Ok(n) = v.trim().parse::<i64>() { ALLY_TOWER_DPS.store(n.clamp(0,100), Ordering::Relaxed); } }   // ★포탑 DPS 전력가중(한타) 0~100
                "ally_tower_dps_move" => { if let Ok(n) = v.trim().parse::<i64>() { ALLY_TOWER_DPS_MOVE.store(n, Ordering::Relaxed); } }   // ★포탑 DPS 전력가중(라인전) -1=한타값 따름
                "ally_tower_range" => { if let Ok(n) = v.trim().parse::<u64>() { if n > 0 { ALLY_TOWER_RANGE.store(n, Ordering::Relaxed); } } }   // ★아군 포탑 인식범위(한타)
                "ally_tower_range_move" => { if let Ok(n) = v.trim().parse::<i64>() { ALLY_TOWER_RANGE_MOVE.store(n, Ordering::Relaxed); } }   // ★아군 포탑 인식범위(라인전) -1=한타값
                kk if kk.starts_with("numbers_threat_sp") => {   // ★subplan별 개별 임계: numbers_threat_sp3=0(라인전 off), numbers_threat_sp9=80(에픽한타) 등. 미설정 subplan은 numbers_threat 폴백.
                    if let (Ok(idx), Ok(n)) = (kk["numbers_threat_sp".len()..].parse::<usize>(), v.trim().parse::<i64>()) {
                        if idx < 18 { NUMBERS_THREAT_SP[idx].store(n, Ordering::Relaxed); if n >= 0 { NUMBERS_THREAT_SP_ANY.store(true, Ordering::Relaxed); } }
                    }
                }
                "towercap" => { let on=v=="1"||v.eq_ignore_ascii_case("true"); if on { TOWERCAP_N.store(0, Ordering::Relaxed); NUM_MAXENEMY.store(0, Ordering::Relaxed); } TOWERCAP.store(on, Ordering::Relaxed); }   // ★캡처 toggle: on이면 항상 카운터 리셋(핫리로드 대응)
                "tower_threat" => { if let Ok(n) = v.trim().parse::<i64>() { TOWER_THREAT.store(n, Ordering::Relaxed); } }   // ★포탑 회피 강도 0~100(0=off, 100=tower_range 전체서 후퇴)
                "roam_diag" => { let on = v=="1"||v.eq_ignore_ascii_case("true"); if on && !ROAM_DIAG.load(Ordering::Relaxed) { for a in [&LANER_CALL_N,&LANER_RET_N,&LANER_RET_TOW,&LANER_RET_FRC,&LANER_RET_NUM,&TOW_UNDER_N,&TOW_W_SUM,&TOW_W_CNT,&FRC_W_SUM,&FRC_W_CNT] { a.store(0, Ordering::Relaxed); } } ROAM_DIAG.store(on, Ordering::Relaxed); }   // ★[07-16] 기지박힘 진단(roam_diag.txt). 켤때 카운터 리셋
                "judge_dump" => { let m = v.trim().parse::<u8>().unwrap_or(0).min(2); let prev = JUDGE_DUMP.load(Ordering::Relaxed); if m != 0 && prev == 0 { if let Ok(mut g)=JD_MATCHES.lock() { *g = None; } JD_MATCH_CNT.store(0, Ordering::Relaxed); } JUDGE_DUMP.store(m, Ordering::Relaxed); }   // ★[07-16] 판단 풀덤프(match_log/match_NN.txt) 0=off/1=관리팀/2=모든경기. 켤때 경기맵 리셋
                "tower_range" => { if let Ok(n) = v.trim().parse::<u64>() { if n > 0 { TOWER_RANGE.store(n, Ordering::Relaxed); } } }   // ★포탑 위험반경(threat=100 기준)
                "stat_influence" => { if let Ok(n) = v.trim().parse::<i64>() { STAT_INFLUENCE.store(n.clamp(0, 100), Ordering::Relaxed); } }   // ★성향스탯 보정강도 0~100(0=비트동일)
                "tgcap" => { TG_CAP.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★0.4.14 위협게이트 p2 로깅(tgcap.txt)
                "serpen_verify" => { SERPEN_VERIFY.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★disc12 SerpenBattle body 재현 검증(serpencmp=pokecmp.txt에 disc12 라인, shadow-call AV위험→기본OFF)
                "jungle_retreat_threat" => { if let Ok(n)=v.trim().parse::<i64>() { TG_MULT.store(n.clamp(0,200), Ordering::Relaxed); } }   // ★0.4.14 정글러 교전후퇴 p2배율 0~200(100=원본). <100=잘후퇴/>100=덜후퇴
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
                "recallcap" => {
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !RECALLCAP.load(Ordering::Relaxed) {
                        RECALL_ARMED.store(0, Ordering::Relaxed);
                        RECALL_FILE_INIT.store(false, Ordering::Relaxed);
                    }
                    RECALLCAP.store(on, Ordering::Relaxed);
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
                "e9jt" => { E9_JT.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }
                "d4ttd" => { let on=v=="1"||v.eq_ignore_ascii_case("true"); if on && !D4_TTD.load(Ordering::Relaxed) { D4_TTD_PASS.store(0,Ordering::Relaxed); D4_TTD_C8.store(0,Ordering::Relaxed); } D4_TTD.store(on, Ordering::Relaxed); }
                "d4_repl" => { D4_REPL.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // disc4 mp_repl 대체 토글(freeze 격리; 0=disc4만 passthrough)
                "d14_repl" => { D14_REPL.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★07-23 disc14 대체 토글(0=passthrough+캡처 → mpcmp에 disc14 판정줄 생성=재검증용)
                "d12_repl" => { D12_REPL.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★07-23 disc12 대체 토글(0=passthrough+캡처=격리·재검증)
                "detlog" => {   // ★[07-29] divergence tracer(개입 지점 해시 대조·detdiv.txt). ON시 1회 덤프 스레드 스폰(5초 주기, detour 밖 IO)
                    let on = v=="1"||v.eq_ignore_ascii_case("true");
                    if on && !DL_ON.swap(true, Ordering::Relaxed) {
                        std::thread::spawn(|| loop {
                            std::thread::sleep(std::time::Duration::from_secs(5));
                            let _ = std::panic::catch_unwind(dl_dump);
                        });
                    } else if !on { DL_ON.store(false, Ordering::Relaxed); }
                }
                "mp_d56_repl" => { MP_D56_REPL.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★07-11 크래시대책①: disc5/6 대체 격리(기본0=관측만; §12.23)
                "dcap" => { DISC1819_CAP.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★disc18/19 넥서스 game 출력 캡처(disc1819cap.txt, 완전재현 대조용; 기본0)
                "d19thr" => { D19_THREAT_SHADOW.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★disc19 위협점수 활성 게이트(기본OFF→threat=0=후반B경로). 켜면 my_disc19 HP사다리 활성. 계산=기본 순수(d19_threat_pure), 롤백=d19thrpure=0
                "d19thrpure" => { D19_THREAT_PURE.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★[07-12] threat 계산 순수/shadow 선택(기본1=순수 FUN_1420a3fd0 재현, vt0x28 base getter만 leaf shadow). 0=전체 shadow-call 롤백(AV위험 §3)
                "d19gate1" => { D19_GATE1.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★disc19 Gate1(조기홈복귀) 활성: FUN_14237d910/FUN_142090ec0/getter vt0x90 shadow-call(AV+재sim오염 위험 §3, 기본OFF=현행 완전보존). 켜면 Gate2 앞 Gate1 판정(cf0||bVar5||cf1→glen=1 홈복귀)
                "d19vis" => { D19_VIS_GATE.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★disc19 struct_threat 확률 시야 게이트(기본 ON): d19_building_reaches 후보건물에 FUN_14237d910 시야 롤(Gate1 bVar5 동일)→미노출이면 제외. OFF=구동작(visible=true→실desc 4건 오도달)
                "d19abil" => { D19_ABIL.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }
                "d19abil2" => { D19_ABIL2.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★[07-14] 2차 abil emitter(FUN_14236ddf0) 발행. 비-self 5사이트+Gate#1 스코어게이트 순수재현(self-target 사이트는 Gate#2/#3 폴리모픽 미해결로 defer). 기본 OFF
                "d19_lead" => { D19_LEAD.store(v!="0"&&!v.eq_ignore_ascii_case("false"), Ordering::Relaxed); }   // ★[07-15 진단] cand_main 리드보정(spd 감산) ON/OFF. 기본 ON. 0xf under 원인 격리용

                "d19_g1_shadow" => { D19_G1_SHADOW.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★시야롤 롤백 토글(기본 OFF=순수재현 d19_g1_pred_pure). ON=FUN_14237d910 shadow-call(AV위험 §3). 순수는 shadow와 17790건 비트동일 검증완료(mmN=0)
                "d19_us_shadow" => { D19_US_SHADOW.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★[07-12] usable_slot1/2 롤백 토글(기본 OFF=순수재현 d19_usable_slot1/2). ON=fce700/fbe950 shadow-call(AV위험 §3)
                "d19_bd_shadow" => { D19_BASEDMG_SHADOW.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★[07-12] vt0x28 base쌍 getter 롤백(기본 OFF=순수 d19_basedmg 6종). ON=probe_basedmg_r9 전체 shadow-call(AV위험 §3)
                "d19_bd_cmp" => { let on=v=="1"||v.eq_ignore_ascii_case("true"); if on { BD_CMP_OK.store(0,Ordering::Relaxed); BD_CMP_MM.store(0,Ordering::Relaxed); } D19_BD_CMP.store(on, Ordering::Relaxed); }
                // ★[07-31] SubPlan19 강제(검증 전용). my_disc17의 최종 반환을 0x13으로 고정 → disc19 핸들러 경로를 태운다.
                "force_sp19" => { FORCE_SP19.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }
                // ★[07-31] disc16/17 대체 A/B 토글. 0 = 그 둘만 passthrough(게임 원본 실행).
                // ⚠키 이름 `nx_`는 넥서스처럼 보이지만 **세르펜(disc16 SerpenHunt·17 SerpenPoke)** 스위치다(구라벨, L7164 참조).
            "nx_repl" => { D1617_REPL.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★[07-12] 순수 base쌍 vs shadow 비트동일 대조(bdcmp.txt, 검증 전용·shadow 호출 AV위험 §3)
                "d19_g1cap" => { G1CAP_ON.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★[07-12] compFlag vt0x170 오브젝티브 트리 빌더 concrete 타깃 캡처(g1cap.txt, 순수 read·기본 OFF). Gate1(d19gate1=1) 발화 시 수집→오프라인 RE
                "d19_g1cf_shadow" => { D19_G1CF_SHADOW.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★[07-12] compFlag 순수/shadow 선택(기본1=shadow FUN_142090ec0, 순수검증 전 안전). 0=순수 d19_g1_compflag_pure
                "d19_g1cf_cmp" => { let on=v=="1"||v.eq_ignore_ascii_case("true"); if on { G1CF_OK.store(0,Ordering::Relaxed); G1CF_MM.store(0,Ordering::Relaxed); } D19_G1CF_CMP.store(on, Ordering::Relaxed); }   // ★[07-12] compFlag 순수 vs shadow A/B(g1cfcmp.txt, 검증 전용·shadow 호출)
                "d19_g1cf_loop2" => { D19_G1CF_LOOP2.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★[07-12] 진단: loop2 threat항 격리(0=de40만)
                "d18abil" => { D18_ABIL.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★[07-12] disc18 Phase3/4 능력블록(0xf/0x10/0x11) 발행(기본 OFF=골격 미발행). 실발행+dcmp 보정은 §11.9.11 잔여
                "d19_threat_mult" => { if let Ok(n)=v.parse() { D19_THREAT_MULT.store(n, Ordering::Relaxed); } }   // ★[07-12] disc19 위협점수 배수%(주력 튜닝)
                "d19_retreat_hp" => { if let Ok(n)=v.parse() { D19_RETREAT_HP.store(n, Ordering::Relaxed); } }   // ★[07-12] go_detailed HP%문턱
                "d19_range_atkme" => { if let Ok(n)=v.parse() { D19_RANGE_ATKME.store(n, Ordering::Relaxed); } }
                "d19_range_bld" => { if let Ok(n)=v.parse() { D19_RANGE_BLD.store(n, Ordering::Relaxed); } }
                "d19_range_other" => { if let Ok(n)=v.parse() { D19_RANGE_OTHER.store(n, Ordering::Relaxed); } }
                "d19_range_idle" => { if let Ok(n)=v.parse() { D19_RANGE_IDLE.store(n, Ordering::Relaxed); } }
                "d7_repl" => { D7_REPL.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // disc7(Recall) 라이브대체 토글(기본0=원본. d7_hp_normal/selfheal/wp_dist2 반영엔 이게 켜져야)
                "d15_repl" => { D15_REPL.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // disc15(SerpenCheck) 라이브대체 토글(기본0=원본. d15_engage_hp_pct 반영엔 이게 켜져야·재현 미검증)
                "mp_observe" => { MP_OBSERVE.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // 관찰전용 캡처(my 미실행) — 튜토리얼/스테이지1 발화 관찰용
                "perf_measure" => { let on = v=="1"||v.eq_ignore_ascii_case("true"); if on { for i in 0..8 { PERF_NS[i].store(0,Ordering::Relaxed); PERF_CNT[i].store(0,Ordering::Relaxed); } } PERF_ON.store(on, Ordering::Relaxed); }   // judge별 시간측정→perf.txt
                "fast_read" => { let lvl = v.trim().parse::<u8>().unwrap_or(if v.eq_ignore_ascii_case("true"){2}else{0}); FAST_READ.store(lvl.min(2), Ordering::Relaxed); }   // ★rd_* 읽기 경로: 0=VirtualQuery / 1=VEH spinlock / 2=VEH lockless(최속). 문제시 낮춰서 롤백
                "fast_guard" => { FAST_GUARD.store(if v=="0" {0} else {1}, Ordering::Relaxed); }   // ★[07-16] readable() VEH프로브 가속(fast_read=2 전제). 0=VirtualQuery 롤백
                "hang_diag" => { HANG_DIAG.store(if v=="0" {0} else {1}, Ordering::Relaxed); }     // ★[07-16] 행 진단 워치독 ON/OFF
                "hang_secs" => { if let Ok(n)=v.parse() { HANG_SECS.store(n, Ordering::Relaxed); } }
                "hang_run_secs" => { if let Ok(n)=v.parse() { HANG_RUN_SECS.store(n, Ordering::Relaxed); } }
                "hang_run_rate" => { if let Ok(n)=v.parse() { HANG_RUN_RATE.store(n, Ordering::Relaxed); } }
                "adv_prof" => { ADV_PROF.store(if v=="0" {0} else {1}, Ordering::Relaxed); }       // ★[07-16] 활동창 프로파일러(adv_prof.txt). ⚠배포시 0
                "adv_prof_min" => { if let Ok(n)=v.parse() { ADV_PROF_MIN.store(n, Ordering::Relaxed); } }
                "adv_prof_seg" => { if let Ok(n)=v.parse() { ADV_PROF_SEG.store(n, Ordering::Relaxed); } }
                "read_bench" => { if v=="1"||v.eq_ignore_ascii_case("true") { unsafe { bench_reads(); } } }   // ★읽기경로 직접벤치 1회 → readbench.txt (per-read ns ground-truth)
                "log" => { LOG_ON.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★진단/로그 파일출력 마스터(기본 off=배포 깨끗). 1=plan_reimpl.txt·perf.txt·*cmp.txt 등 기록
                // ★[07-31] subplan별 후퇴 발동 누적 측정. `log`과 **독립** — 여러 판 연속 측정 시 mpcmp 등 무거운 로깅을 안 켜도 되게.
                "sp_seen" => { SP_SEEN_ON.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }
                // ★조합 테스트 결과가 DB 어디에 저장되는지 추적(스냅샷 diff). 찾고 나면 0으로.
                "ct_hunt" => { CT_HUNT.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }
                // ★구간 라벨(전술 이름 등). 값이 바뀌는 순간 직전 구간이 sp_seen_hist.txt 에 한 줄로 확정된다.
                "sp_seen_tag" => { *SP_TAG_CFG.lock().unwrap_or_else(|e| e.into_inner()) = v.trim().to_string(); }
                "call_ablate" => { CALL_ABLATE.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★오더 콜(0xb) 제거 ablation: 1=콜차단(retreat_engage 2 push nop), 0=원본복원. 콜 영향 검증용
                "lane_gate" => { if let Ok(n)=v.trim().parse::<u8>() { LANE_GATE.store(n.min(2), Ordering::Relaxed); } }   // ★오더 라인후보 게이트 ablation: 0=원본/1=OFF(후보0개)/2=ALL(후보다). 매크로 영향 검증용
                "type3_ablate" => { TYPE3_ABLATE.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★오더 transition type3 콜 차단: 1=차단(jae→jmp), 0=원본. 매크로 subplan 전환 영향 검증
                "skip_untuned" => { SKIP_UNTUNED.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★튜닝 안 한 judge는 원본 native 사용(속도↑·결과동일). 일정넘김 백그라운드 가속
                "class_verify" => { CLASS_VERIFY.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★런타임 검증: 클래스 탐지+오버라이드 적용횟수(class_verify.txt)
                "champ_verify" => { let on=v=="1"||v.eq_ignore_ascii_case("true"); if on && !CHAMP_VERIFY.load(Ordering::Relaxed) { CHAMP_OVHIT.store(0, Ordering::Relaxed); GATE_PASS.store(0,Ordering::Relaxed); GATE_BLOCK.store(0,Ordering::Relaxed); if let Ok(mut s)=CHAMP_SEEN.lock(){ s.clear(); } if let Ok(mut g)=GATE_IDS.lock(){ g.clear(); } } CHAMP_VERIFY.store(on, Ordering::Relaxed); }   // ★선수별 오버라이드 탐지+적용횟수+게이트(champ_verify.txt)
                "self_team_only" => { SELF_TEAM_ONLY.store(v=="1"||v.eq_ignore_ascii_case("true"), Ordering::Relaxed); }   // ★우리팀 챔피언에만 champ 오버라이드 적용(기본 1). 0=팀무관 전체적용(구 동작).
                // ★★ judge 튜닝 계수 (기본 engage/ttd/gb=100%, recall_bias=0). 안 적으면 게임원본=replay-identical.
                "t_engage" => { if let Ok(n) = v.parse::<i64>() { TUNE_ENGAGE_MULT.store(n, Ordering::Relaxed); } }
                "t_ttd"    => { if let Ok(n) = v.parse::<i64>() { TUNE_TTD_MULT.store(n, Ordering::Relaxed); } }
                "t_recall" => { if let Ok(n) = v.parse::<i64>() { TUNE_RECALL_BIAS.store(n, Ordering::Relaxed); } }
                "t_gb"     => { if let Ok(n) = v.parse::<i64>() { TUNE_GB_MULT.store(n, Ordering::Relaxed); } }
                "aggr_lane"    => { if let Ok(n) = v.parse::<i64>() { AGGR_LANE.store(n.max(1), Ordering::Relaxed); } }       // [3]라인전 공격성 배율%
                "aggr_object"  => { if let Ok(n) = v.parse::<i64>() { AGGR_OBJECT.store(n.max(1), Ordering::Relaxed); } }     // [9·11]오브젝트 공격성 배율%
                "aggr_defense" => { if let Ok(n) = v.parse::<i64>() { AGGR_DEFENSE.store(n.max(1), Ordering::Relaxed); } }    // [14]넥서스수비 공격성 배율%
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
                        for k in 0..18 { MP_SUB_ARMED[k].store(0, Ordering::Relaxed); }
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
    POS_ANY.store(new_tune.keys().any(|k| k.contains("_pos_")), Ordering::Relaxed);   // ★포지션 오버라이드 존재 → skip_untuned 우회(대체경로 보존)
    // ★[08-07] 클래스 오버라이드 정밀화. 구현은 "_class_ 문자열이 있으면 CLASS_ANY=참" 이었고,
    //   그 한 줄이 **효과 없는 클래스 키 하나로 skip_untuned 최적화를 통째로 끄는** 원인이었다
    //   (08-06 재생 멈춤 — bt_vision_mem_class_magician 등 20개는 전부 바이트패치 노브라 원래 안 먹던 값).
    let mut ov_live: Vec<String> = Vec::new();   // 실제로 먹는 오버라이드의 base 노브
    let mut ov_dead: Vec<String> = Vec::new();   // 바이트패치 노브 = 원리상 안 먹음
    {
        let mut bases: Vec<String> = new_tune.keys()
            .filter_map(|k| k.find("_class_").map(|i| k[..i].to_string())).collect();
        bases.sort(); bases.dedup();
        for b in bases {
            if CLASS_CAPABLE.contains(&b.as_str()) { ov_live.push(b); } else { ov_dead.push(b); }
        }
    }
    // 그룹 목록 밖의 유효 오버라이드 = 어느 판단이 읽는지 미상 → 보수적으로 전체 skip 해제.
    // ★[08-07] 단 **마이크로 디투어 노브는 제외한다**. 그건 게임 원본 코드 위에서 상수만 갈아끼우는
    //   방식이라 판단 재현이 아예 필요 없다 ⟹ skip_untuned 를 끌 이유가 없다.
    //   이 예외를 빼먹으면 "클래스 값을 넣으면 배속 재생이 멈춘다"는 08-06 사고가 **그대로 재발**한다.
    let ov_unknown: Vec<String> = ov_live.iter()
        .filter(|b| !SKIP_GROUP_KEYS.contains(&b.as_str()) && !is_micro_knob(b)).cloned().collect();
    CLASS_ANY.store(!ov_live.is_empty(), Ordering::Relaxed);   // 맵빌드는 유효 오버라이드가 있을 때만
    {   // 진단 로그 — 무엇이 먹고 무엇이 무시됐는지 남긴다(조용한 무시 금지)
        let mut s = String::from("=== 클래스별 값(_class_) 적용 결과 ===\n");
        let (mic, jud): (Vec<_>, Vec<_>) = ov_live.iter().partition(|b| is_micro_knob(b));
        s.push_str(&format!("적용됨({}) : {}\n", ov_live.len(), ov_live.join(", ")));
        if !jud.is_empty() {
            s.push_str(&format!("  · 판단 재현 경로({}) : {}\n", jud.len(),
                jud.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")));
        }
        if !mic.is_empty() {
            s.push_str(&format!("  · 마이크로 디투어 경로({}) : {}\n", mic.len(),
                mic.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")));
            s.push_str("    ↑ 원래 바이트패치라 전역이던 노브를, 상수 자리에서 클래스를 보고 값을 갈아끼우도록 연 것.\n\
                        \x20     실제 설치 결과는 class_micro.txt, 발화 횟수는 class_verify.txt 를 볼 것.\n\
                        \x20     ⚠설치/해제는 게임 재시작이 필요하다(값 변경은 재시작 없이 반영된다).\n");
        }
        s.push_str(&format!("무시됨({}) : {}\n", ov_dead.len(), ov_dead.join(", ")));
        s.push_str("  ↑ 무시 사유 = 바이트패치 전용 노브. exe 기계어 상수를 고치는 방식이라 선수별로 다를 수 없다.\n\
                    \x20    (이 중 일부는 마이크로 디투어로 열 수 있으나, 사이트에 판단 주체가 도달하지 않아\n\
                    \x20     현재는 불가로 판정된 것들이다 — 근거 = REPORT\\RE\\2026-08-07_테스트C-클래스노브-8종-*.md)\n");
        if !ov_unknown.is_empty() {
            s.push_str(&format!("판단 미상({}) : {}\n  ↑ 어느 판단이 읽는지 몰라 최적화(skip_untuned)를 전부 해제했다(느려짐).\n",
                ov_unknown.len(), ov_unknown.join(", ")));
        }
        if let Some(p) = pth("class_override.txt") { let _ = fs::write(&p, &s); }
    }
    tune_publish(new_tune);   // ★lock-free: 누적 테이블 일괄 게시(judge tune() 읽기 lock 제거)
    // ★skip_untuned: 튜닝 안 한 judge의 대체를 끔 → 원본 native 사용(결과 100% 동일·속도↑). 일정넘김 백그라운드 N경기 가속.
    //   판정 = default.txt(기준값) 대비 활성값 다름(=튜닝됨). condgate=계수없음→항상 끔. vis_window(CAND_FILTER 시야창)는 광범위→튜닝시 관련 judge 보존(보수적).
    // ★[08-07] CLASS_ANY 로 전체 skip 을 끄지 않는다 — 아래 g() 가 유효 오버라이드를 "튜닝됨" 으로
    //   취급해 **그 판단만** 재구현을 유지한다. 판단 미상 오버라이드가 있을 때만 구 동작(전체 해제)로 폴백.
    if SKIP_UNTUNED.load(Ordering::Relaxed) && ov_unknown.is_empty() && !CHAMP_ANY.load(Ordering::Relaxed) {   // ★_class_/players 오버라이드 있으면 skip 비활성(per-class/champ 대체가 native로 안 빠지게)
        COND_REPL.store(false, Ordering::Relaxed);
        if let Some(base) = read_baseline() {
            // ★[08-07] 클래스 오버라이드가 걸린 노브도 "튜닝됨" 으로 본다 → 그 판단만 재구현 유지.
            let g = |keys: &[&str]| keys.iter().any(|&k|
                ov_live.iter().any(|b| b == k)
                || match base.get(k) { Some(&b) => tune(k, b) != b, None => false });
            let vis_t = g(&["vis_window"]);
            let engage_t = vis_t || NUMBERS_MARGIN.load(Ordering::Relaxed) > 0 || TOWER_THREAT.load(Ordering::Relaxed) > 0 || g(&["t_engage","eng_role4","eng_role3","eng_role2","eng_role_def","engage_base","engage_thr_mult","stat_neutral","stat_pos_div","stat_judg_ref","stat_noise_shift"]);   // numbers/tower>0도 engage 대체 유지(override 동작 위함)
            let disc4_t  = vis_t || TOWER_THREAT.load(Ordering::Relaxed) > 0 || g(&["t_ttd","d4_dmg_scale","d4_div_base","d4_coef_scale","d4_coef_min","d4_coef_clamp","d4_coord_dist","d4_ttd_scale","tower_dps","d4_ward_dist2","d4_engage_r2","d4_ref_dist2","d4_close_hp","d4_threat_min","d4_pathlen_thr","d4_wcast_thr"]);   // ★포탑위협>0도 disc4 대체 유지(TTD 가산 위함)
            let recall_t = vis_t || g(&["t_recall","rc_u21_init","rc_ehp_t1","rc_ehp_t2","rc_ehp_t3","rc_ehp_v1","rc_ehp_v2","rc_norp_bonus","rc_ed_near","rc_ed_mid","rc_ed_far","rc_ed_near_pen","rc_ed_far_bonus","rc_ed_vfar_bonus","rc_ahp_t1","rc_ahp_t2","rc_u13_bonus","rc_ahp2_pen","rc_ad_near","rc_ad_mid","rc_ad_near_bonus","rc_ad_far_pen","rc_mult_bonus","rc_ally_hp_min","rc_rng_a_base","rc_rng_spread_div","rc_rng_center","rc_score_div","rc_join_weight","rc_join_adv","rc_join_rescue","rc_join_dnear","rc_join_dmid","rc_join_obj_mult","pf_edge_margin","pf_center_band","pf_diag_far","pf_diag_near","pf_band_width"]);   // ★[수정 07-16] pf_* 추가(poke_t서 이동. my_recall_mult geometry가 read)
            let gb_t     = vis_t || g(&["t_gb","gb_rbx_div","gb_r15_div","gb_r14_num","gb_cnt_skip","gb_da_thr","gb_cnt_move","gb_db_engage","gb_score_mult"]);
            let dd7_t    = vis_t || TOWER_THREAT.load(Ordering::Relaxed) > 0 || NUMBERS_MARGIN.load(Ordering::Relaxed) > 0 || NUMBERS_THREAT.load(Ordering::Relaxed) > 0 || NUMBERS_THREAT_MOVE.load(Ordering::Relaxed) >= 0 || NUMBERS_THREAT_SP_ANY.load(Ordering::Relaxed) || g(&["dd_frontier_mult","dd_lane_margin","dd_cover_count","dd_ratio_thr","dd_facet_thr","dd_near_dist","dd_main_near_dist","dd_gatee_dist","dd_ivar2_thr","dd_n_thr","dd_survivor_thr","dd_early_p3_thr","dd_cover_p3_thr","dd_f22e80_margin"]);   // ★포탑/전력/Move임계>0도 dd7_repl 유지(라이너 후퇴 override 위함)
            let poke_t   = vis_t || g(&["pk_home_lo","pk_home_hi","pk_home_x1","pk_home_y1","pk_hp_main","pk_hp_retreat","pk_smallact_split","pk_threat_mult","pk_zone_hp","pk_engage_dist","pk_obj_hp","poke_phase_gate","poke_active_min","poke_reach_bonus","poke_serpen_slot"]);   // ★[수정 07-16] pf_* 제거(→recall_t. pf_*는 my_recall_mult가 read=recall 소속인데 여기 있어 skip모드서 recall 강제off 잠복버그였음)
            let mp_misc_t = g(&["d8_slot_thr","sn_home_lo","sn_home_hi","sn_home_x1","sn_home_y1","sn_hp_crit","sn_self_hp","bt_home_lo","bt_home_hi","bt_home_x1","bt_home_y1","bt_hp_retreat",
                "nxd_prog_low","nxd_prog_crit","nxd_p3_gate","nxd_ref_hp","nxd_near_dist","nxd_pred_dist",
                // ★[07-23] 키 개명 동기: ~~ec_tgt_hp_low~~→**ec_self_hp_low**(07-23 개명·구 키는 死) + **disc16_home_hp 누락 보충**(신설 키가 이
                //   그룹에 없으면 skip_untuned=1에서 "그 키만 튜닝" 시 MP_REPL이 통째 꺼져 값이 무시되는 잠복버그). 死레버(ec_gate_tick·d13/d15_*)는 무해라 잔류.
                "ec_oz_hp","ec_iz_hp","ec_self_hp_low","ec_engage_dist2","ec_valid_hp","ec_gate_tick","ec_commit_hp","ec_count_hp","ec_count_radius","ec_vision_ticks",
                "disc16_home_hp","d15_engage_hp_pct","d13_engage_hp_pct"]);
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


// ── my_ttd 재구현 헬퍼 ──
type G2 = unsafe extern "C" fn(usize, usize) -> i64;
fn isqrt(n: u64) -> u64 {
    if n < 2 { return n; }
    let mut x = n; let mut y = (x + 1) / 2;
    while y < x { x = y; y = (x + n / x) / 2; }
    x
}
// vt0x90 = trivial 필드게터 전수확정(§11.9.1-G) → shadow-CALL 대신 vt90_get 순수 read (leaf는 rcx=dataptr만 사용).
unsafe fn vt560_threat(e: usize) -> i64 {
    let v = rd_u64(e + 0x560).unwrap_or(0) as usize;
    let a0 = rd_u64(e + 0x558).unwrap_or(0) as usize;
    if !ptr_ok(v) { return 0; }
    vt90_get(v, a0)
}

// ★engage RNG footprint 측정+예측검증: retreat 진입 (entry_rsp, state, idx0, ctr0, pred_out, pred_words) 스냅 → kind1 리턴서 실제 (out, words)와 대조. engfoot.txt.
//   pred_out/pred_words = my_engage_predict (engage 브랜치만; 비engage는 -777=skip).
static RE_SNAP: Mutex<Vec<(usize,usize,u64,u64,i64,i64,i64,i64)>> = Mutex::new(Vec::new());  // +count_a,count_b(진단)
static EFOOT_INIT: AtomicBool = AtomicBool::new(false);
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
    let counters: [&AtomicU64; 11] = [&RECALL_ARMED,&FC59_RAW,&FC59_ARM,&FC59_FILT,&RE_ARMED,&RE_LOGGED,&DISP_LOGGED,&DISP_OK,&DISP_DIFF,&FULL_OK,&FULL_DIFF];
    for c in counters { c.store(0, Ordering::Relaxed); }
    let flags: [&AtomicBool; 2] = [&RECALL_FILE_INIT,&RE_FILE_INIT];
    for f in flags { f.store(false, Ordering::Relaxed); }
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
    let ent = dd7_slot128(sim, rd_u64(p5 + 0x818).unwrap_or(0));   // 0.5.0(was 0x6a0, SimState self-handle +0x178)
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
#[inline(never)] unsafe fn my_retreat_dispatch(p5: usize, p6: usize, candidate: usize, rh: usize, robj: usize, rvt: usize) -> i64 {
    let team = rd_i64(p5 + 0x810).unwrap_or(-99);   // 0.5.0(was 0x6a8, SimState +0x178)  ★0.5.4 오프셋 이동 반영
    if team != 0 && team != 1 { return -99; }
    let geo2 = rd_u64(p6 + 0x10).unwrap_or(0) as usize;   // geo2 컨테이너 p6+0x10 불변
    let zone = geo2.wrapping_add((team as usize) * 0x2e8);   // 0.5.0(was 0x228, geom stride +0xc0). zone 헤드(0x20/0x48/0x70/0x179)는 불변
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
    // ★[07-18 크래시수정] rvt+0x58은 0.5.1에서 16B struct-return 게터(0x207a6c0, rcx=sret/rdx=this)로 바뀜.
    //   1인자 스칼라(Getter1)로 호출하면 rdx=미제어 leftover → [rdx+0xedc8] deref AV(세이브-재로드 크래시).
    //   쌍둥이(retreat_capture 인라인, 이 함수 아래)는 이미 비활성. 여기만 누락됐던 것 → 동일하게 제거. else path(l238=p5+0x4b8) 사용.
    let cvar4: i64 = -1;
    // ⚠robj+0xecd8(cVar4==0 경로)=정적 미확정(런타임 캡처 대상). else l238 p5+0x430→0x4b8(+0x88) 확정.
    let l238: u64 = if cvar4 == 0 { if readable(robj + 0xed18 + (team as usize) * 0x18, 8) { rd_u64(robj + 0xed18 + (team as usize) * 0x18).unwrap_or(0) } else { 0 } } else { rd_u64(p5 + 0x4b8).unwrap_or(0) };
    let cvar6 = ((l238 >> 16) & 0xff) as i64;
    my_dispatch_code(cvar6, ce_pt, ce_1, zone, postag, za20, za48, za70, rh, geo2, p5)
}
unsafe extern "C" fn retreat_capture(saved: usize, entry_rsp: usize) -> u64 {
    if !ptr_ok(entry_rsp) { return 1; }
    let _jm = judge_mark(1);   // ★행진단: 하트비트+in-flight(retreat)
    // ★[07-16 최적화] apply체인 세대게이트: 기존엔 매콜 imm 3형제가 sig 재계산용 tune() ~20회 소모(핫패스 낭비).
    //   cfg 실리로드(CFG_GEN+1)때만 체인 실행, READY 도달 후 세대 완료마킹(미완료시 다음 콜 재시도=기존 재시도 의미 보존).
    let cfg_gen = CFG_GEN.load(Ordering::Relaxed);
    if cfg_gen != APPLY_GEN.load(Ordering::Relaxed)
        && APPLY_LOCK.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok()
    {
        // ★[08-07] 마이크로 디투어를 **먼저** 설치한다 — 아래 apply_*_imm 들이 `micro_taken()` 으로
        //   자기 사이트를 건너뛸지 판단하기 때문. 순서가 뒤집히면 imm 패치가 우리 `E9` 를 덮어써
        //   게임이 엉뚱한 주소로 점프한다(상호배타가 깨지는 유일한 경로).
        install_class_micro();
        apply_call_ablate();  // ★오더 콜 ablation 패치 적용/복원 (want==applied면 즉시 return)
        apply_lane_gate();    // ★오더 라인후보 게이트 ablation (lane_gate 0/1/2)
        apply_type3_ablate(); // ★오더 transition type3 콜 ablation (매크로 전환 영향 검증)
        apply_objective_imm();// ★objective 원본상수 노출 (oi_* imm-patch)
        apply_vis_imm();      // ★[07-16] vis_window 부활 byte-patch (0x1caedd3 imm32, 기본600=무변화)
        apply_gb_imm();       // ★[07-16] GenericBuild 로밍/운영 byte-patch (경로A, gb_enable=0 기본=무변화)
        apply_sev_imm();      // ★[07-23 신설] 공유 위협 severity 사다리 byte-patch (sv_enable=0 기본=무변화)
        apply_visshort_imm(); // ★[08-03 신설] subplan별 개별 단기 시야창(120틱) byte-patch (전 키 기본 -1=무변화)
        apply_gank_imm();     // ★[08-03 신설] 라인개입(jng=1) 갱 셋업 타이밍/게이트 byte-patch (전 키 기본 -1=무변화)
        apply_plan_imm();     // ★[08-03 신설] plan 결정기 생성 게이트 byte-patch (전 키 기본 -1=무변화)
        apply_path_imm();     // ★[0.5.4 신설] 경로/거리 시스템 208사이트 (전 키 기본 -1=무변화)
        apply_auc_imm();      // ★[0.5.4 신설] 경매 중 강제귀환 12노브 (전 키 기본 -1=무변화)
        apply_an_imm();       // ★[0.5.4 신설] 판단14 넥서스공격 — 노브가 없던 유일한 판단 (전 키 기본 -1=무변화)
        apply_exec_imm();     // ★[08-03 신설] sub_plan 실행층 byte-patch — 판단력 오판 게이트·대기 거리·오더 유지 (전 키 기본 -1=무변화)
        apply_auction_imm();  // ★[08-03 신설] 판단력 노이즈(judge_noise_ratio)·battle.rs·line_defense 2회차·팀모드 취소마스크 (전 키 기본 -1=무변화)
        apply_new_imm();      // ★[08-05 신설] 적 위치추정 모델·시전 2차검열·1차 점수컷·경매 재선택·전역 궁 (전 키 기본 -1=무변화)
        apply_cast_imm();     // 시전 후보(평타·스킬 사거리/조건)·행동 실행층(해금레벨·재판단 간격) (전 키 기본 -1=무변화)
        apply_score_imm();    // 행동 점수 엔진(수적우세 배율·인식반경)·대기/안전/이동 실행층 (전 키 기본 -1=무변화)
        apply_score2_imm();   // 전투행동 점수 공식(구조물 인식반경·위험 사다리·보너스 상한) (전 키 기본 -1=무변화)
        apply_move_imm();     // ★[08-03 신설] 이동 계열 점수 cat0 도주·cat2 접근·cat4 추적 (전 키 기본 -1=무변화)
        apply_db_imm();       // ★[08-03 신설] death_battle 전투 후보 생성기 + 안전판정 게이트 (전 키 기본 -1=무변화)
        apply_pe_imm();       // ★[08-03 신설] 자리 평가 엔진 position_eval — risk 생성 자체 (전 키 기본 -1=무변화)
        apply_ldsc_imm();     // ★[08-03 신설] line_defense 후보 점수 함수 c66800 (전 키 기본 -1=무변화)
        apply_move2_imm();    // ★[08-04 신설] 이동 입력 생성기 c86560 + 우물탈출 (전 키 기본 -1=무변화)
        apply_bv_imm();       // ★[08-04 신설] buff_value 9함수 — 전투 실익 (전 키 기본 -1=무변화)
        apply_ae_imm();       // ★[08-04 신설] action_eval df5880 — 라인 수비 점수의 절반 (전 키 기본 -1=무변화)
        apply_th_imm();       // ★[08-04 신설] 위협 디스크립터 생산자 d07a60 (전 키 기본 -1=무변화)
        apply_rt_imm();
            apply_lt_imm();
            apply_nx_imm();
            apply_nxe();      // ★[08-08 신설] "넥서스 비상" 발동 조건 — 남은 구조물 N개 이하 / 2차 타워 파괴 (기본 -1=원본)
            apply_hd_imm();
            apply_d4_imm();
            apply_c3_imm();
            apply_lv_imm();
            apply_eh_imm();       // ★[08-04 신설] 위협감지·후퇴 d63d60 + 정글 진행 게이트 (전 키 기본 -1=무변화)
        apply_sim_unchunk();  // ★[07-16] 백그sim 병렬도(rayon split budget nop, sim_unchunk=0 기본=무변화)
        apply_fix_skill2();    // ★[08-04] 게임 결함 수정 스위치(fix_skill2_dmg, 기본 0=원본)
        apply_fix_hp_ratio();  // ★[08-04] 게임 결함 수정 스위치(fix_hp_ratio, 기본 0=원본)
        apply_lt_revive_join();// ★[08-04] 죽은 판단 되살리기(lt_revive_join, 기본 0=원본)
        // ★[08-04] 런타임 진단 프로브(probe=0 기본 OFF). **catch_unwind 필수** — 여기서 패닉이 나면
        //   SDK 콜백을 관통해 unwind되어 게임이 죽는다(실제로 한 번 그렇게 죽였다).
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| apply_probe()));
        write_guard_summary();   // ★[#26] 원본값 가드 결과 — blocked>0 이면 배선 주소가 틀린 것
        // ★[08-07] `micro_settled()` 추가 — 마이크로 디투어 설치가 아직 준비 관문에서 튕기는 중이면
        //   이 세대를 완료로 찍지 않는다. 안 그러면 체인이 다시 오지 않아 **설치가 영영 누락된다**:
        //   `CFG_GEN` 은 cfg 파싱 **시작**에 +1 되는데 `tune_publish` 는 파싱 **끝**이라, 그 틈에 체인이
        //   돌면 설치는 빈 튜닝 테이블을 보고 튕기는데 apply_*_imm 들은 (기본값으로) 정상 완료돼
        //   `APPLY_GEN` 이 저장돼 버린다. 첫 인게임 확인에서 실제로 이렇게 통째로 누락됐다.
        if exe_base() != 0 && READY_TICKS.load(Ordering::Relaxed) >= READY_MIN && micro_settled() {
            APPLY_GEN.store(cfg_gen, Ordering::Relaxed);   // READY 상태서 체인 완주 = 이 세대 완료
        }
        APPLY_LOCK.store(false, Ordering::Release);   // ★[08-06] 락 해제 — 실패해도 반드시 푼다
    }
    // ★[07-15] apply_disc19_imm은 로드시점(install_wrap 블록)에서만 호출 — 게임플레이중 패치=AV폴트. 여기(retreat_capture=게임플레이) 호출 제거.
    // ★새 sim 첫 호출이면 캡처 리셋(메뉴서 IN_MENU=true → 첫 sim 훅이 swap(false)+reset)
    if REPLAY_RESET.load(Ordering::Relaxed) && IN_MENU.swap(false, Ordering::Relaxed) { reset_captures(); }
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
        let team_units = rd_u64(p5 + 0x818).unwrap_or(0);   // 0.5.0(was 0x6a0, SimState +0x178). self-handle(u64)
        // ★0.5.0: rvt+0x138 resolver(0x21aebf0)=dd7_slot128과 동일 4단 SlotMap chase(handle→entity). shadow-call 제거=AV 크래시 방지(순수재현). robj=self holder.
        let cand = dd7_slot128(robj, team_units);
        if ptr_ok(cand) && readable(cand, 0x660) {
            let cnt = rd_i64(cand + 0x610).unwrap_or(0);
            let dep = rd_i64(cand + 0x658).unwrap_or(0);
            (cand, cnt, if cnt != 0 { dep * 100 / cnt } else { -1 })
        } else { (cand, -1, -1) }
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
        let now2: i64 = if false {   // ★0.5.0: now2 getter(rvt+0x28) shadow-call=게임함수 내부walk AV. 진단(tgate)용이라 비활성. (재검증시 slot/robj 정체 확정 후 복원)
            let g = rd_u64(rvt+0x28).unwrap_or(0) as usize;
            if ptr_ok(g) && ptr_ok(robj) { let f: Getter1 = core::mem::transmute(g); f(robj) } else { -1 }
        } else { -1 };
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
        let cvar4: i64 = if false {   // ★0.5.0: cvar4 getter(rvt+0x58) shadow-call=게임함수 내부walk AV. 진단(cvar6)용이라 비활성(-1→else path p5+0x4b8). (재검증시 복원)
            let g = rd_u64(rvt+0x58).unwrap_or(0) as usize;
            if ptr_ok(g) && ptr_ok(robj) { let f: Getter1 = core::mem::transmute(g); f(robj) & 0xff } else { -1 }
        } else { -1 };
        let l238: u64 = if cvar4==0 {
            if team>=0 && team<=1 && readable(robj+0xed18+(team as usize)*0x18, 8) { rd_u64(robj+0xed18+(team as usize)*0x18).unwrap_or(0) } else { 0 }   // ⚠robj+0xecd8=미확정(cVar4==0 소수경로)
        } else { rd_u64(p5 + 0x4b8).unwrap_or(0) };   // 0.5.0(was 0x86*8=0x430→0x4b8, else l238)
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
            if candidate != 0 && cand_cnt != 0 && my_lp != 0 {   // ★0.5.0: engage 재현 포팅 완료(threshold 0x888, e9probe 판별) → 재활성. my_engage_predict=RngSim read-only 예측(writeback無=desync無, engfoot 검증용 안전)
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

// ── 오더 transition_engine 타입3 콜 ablation (subplan 전환엔진, order*7+300 확률) ──
//   FUN_141e961d0 내 type3 push 게이트 2지점(0x1e9d318/0x1e9d59b, jae skip). 1=차단(jae→jmp, push0개), 0=원본.
//   RNG는 게이트 위라 보존. 1바이트 패치(0x73↔0xEB, 둘째 0x5f 검증). ★0xb콜과 별개 경로(다른 디스패처 핸들러→plan_state subplan/phase write 잠재=살아있을 수 있음).
static TYPE3_ABLATE: AtomicBool = AtomicBool::new(false);
static TYPE3_APPLIED: AtomicBool = AtomicBool::new(false);
const T3_GATE_A_RVA: usize = 0x1e9d318;   // jae 0x1e9d379 (원본 73 5f)
const T3_GATE_B_RVA: usize = 0x1e9d59b;   // jae 0x1e9d5fc (원본 73 5f)

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

// ════════════════ objective 원본상수 노출 (imm byte-patch) ════════════════
// ★"새 계산식 없음" 원칙: 게임 원본 함수(DefenseNexus 결정 0x2101a80 / AttackNexus 실행 0x232351e)를
//   그대로 실행시키되, 코드에 박힌 immediate 상수(임계값)만 cfg 값으로 덮어씀. reimpl(my_defense_nexus,
//   대체 스택)과 완전 독립 — 이쪽은 게임 함수 자체를 대체 안 함. nx_enable=0이면 게임 원본값 복원(=무개입).
// 각 타깃: (base+rva, imm_off_in_insn, width, prefix검증바이트). prefix 불일치=RVA어긋남→그 타깃 skip.
// 1B imm은 cmp qword의 sign-ext imm8 → 값 0..=0x7f로 clamp(음수화 방지). dist²는 movabs imm64=거리²+1.
static OBJIMM_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);
static VISIMM_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);   // vis_window byte-patch 서명
static GBIMM_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);    // GenericBuild(운영전환) 로밍 byte-patch 서명
static SEVIMM_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);   // ★[07-23 신설] 공유 위협 severity 사다리 byte-patch 서명
static VISSHORT_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF); // ★[08-03 신설] subplan별 개별 단기 시야창(120틱) byte-patch 서명
static GANKIMM_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);  // ★[08-03 신설] 라인개입 갱 셋업 타이밍/게이트 byte-patch 서명
static PLANIMM_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);  // ★[08-03 신설] plan 결정기(0xd452e0) 생성 게이트 byte-patch 서명
static EXECIMM_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);  // ★[08-03 신설] sub_plan 실행층(판단력 게이트·대기거리·오더유지) byte-patch 서명
static AUCTIMM_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);  // ★[08-03 신설] 판단력 노이즈(judge_noise_ratio)·battle.rs·line_defense 2회차·팀모드 취소마스크 byte-patch 서명
static NEWIMM_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);  // ★[08-05 신설] 적위치추정·시전2차검열·1차점수컷·경매재선택·전역궁 byte-patch 서명
static CASTIMM_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);  // 시전 후보 생성기·행동 실행층 byte-patch 서명
static SCOREIMM_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);  // 점수 엔진·이동 실행층 byte-patch 서명
static SCORE2_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);    // 전투행동 점수 공식 byte-patch 서명
static RDATA_ADV_OK: AtomicBool = AtomicBool::new(false);                 // 수적우세 .rdata 테이블 주소 검증 통과 여부
static MOVEIMM_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);   // 이동 계열 점수(cat0/2/4) byte-patch 서명
static DBIMM_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);     // death_battle 전투 후보 생성기 byte-patch 서명
static RDATA_ADV0_OK: AtomicBool = AtomicBool::new(false);                // cat0(도주) 배율 .rdata 테이블 주소 검증 통과 여부
static PEIMM_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);     // 자리 평가 엔진(position_eval) byte-patch 서명
static SP_VER2_HIST: [AtomicU64; 8] = [
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];
static SP_VER2_BIG: AtomicU64 = AtomicU64::new(0);
static MP_VER_SEEN: AtomicU64 = AtomicU64::new(0);
/// ★0.5.4 경매 진입 시 관측한 TeamPlan.version 분포(0~7 개별, 그 밖은 BIG).
static AUC_VER_HIST: [AtomicU64; 8] = [
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];
static AUC_VER_BIG: AtomicU64 = AtomicU64::new(0);
static ORIG_AUCTION: AtomicUsize = AtomicUsize::new(0);
static PATHIMM_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static AUCIMM_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static ANIMM_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);     // ★0.5.4 신설: 판단14 넥서스공격 byte-patch 서명
static LDSC_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);      // line_defense 후보 점수 함수 byte-patch 서명
static MOVE2_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);     // 이동 입력 생성기·우물탈출 byte-patch 서명
static BV_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);        // buff_value 9함수 byte-patch 서명
static AE_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);        // action_eval byte-patch 서명
static TH_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);        // 위협 디스크립터 생산자 byte-patch 서명
static RT_SIG: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_FFFF_FFFF);        // 위협감지·후퇴/정글 byte-patch 서명
static LT_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static NX_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static HD_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static D4_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static C3_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static LV_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static EH_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
// ════════════════ disc19(FUN_141c83700) 판단상수 노출 (imm byte-patch, disc19 전용) ════════════════
// ★"새 계산식 없음" 원칙: disc19 핸들러 코드에 박힌 판단 임계값(위협비율표·HP경계·phase·retreat·ally)만 cfg로 덮어씀.
//   출력축(abil 발행)·threat식(FUN_1420a3fd0 공유)·compFlag(FUN_14209a750 공유)는 무손댐 → 다른 AI 무영향, 게임 원본 발행.
//   ★컴파일러 경계변환(<→<=, >→>=) 반영: HP경계·phase는 V-1, tr>9·retreat2는 V+1로 인코딩값 도출(주석의 원본 imm이 근거).
//   d19i_enable=0이면 게임 원본 imm 복원(무개입). 전 사이트 REX+83+modrm 3바이트 prefix 검증 후 imm8 1바이트만 패치.
static D19IMM_DONE: AtomicBool = AtomicBool::new(false);
static ORIG_DISC18: AtomicUsize = AtomicUsize::new(0);   // game 원본 트램폴린(wrap이 호출)
static ORIG_DISC19: AtomicUsize = AtomicUsize::new(0);
// ★★[07-31] SubPlan 디스패처 계측 — "disc18/19 훅이 설치됐는데 한 번도 발화하지 않는" 원인 규명용.
//   ghidra-re 결론: 발화 게이트는 **"그 유닛의 SubPlan(unit+0x6b0)이 18/19인가" 하나뿐**이고,
//   Plan16/17(진입결정)은 실제로 발생하는데(REPL 로그 disc=16 42회·17 56회) SubPlan 승격에서 막히는 것으로 보인다.
//   ⟹ 실제 SubPlan 분포를 직접 세어 확정한다. **read-only 카운터만**(파일 IO 없음 — 매 호출 IO는 게임을 죽인다는 07-22 실증).
// ⛔**기본 false 확정** — 2026-07-31 인게임 크래시(AV, 2회 재현)로 즉시 차단. 원인 규명 전 재활성 금지.
//   ★단 크래시 전 5,269건을 세는 데는 성공했고 그 결과가 결론이 됐다: `18:`/`19:` 버킷 **0건**
//   (`subplan_dispatch: total=5269 other=0 | 0:155 1:95 2:3692 4:93 6:7 7:247 8:912 11:68`).
const SPDISP_PROBE: bool = false;
/// ★[0.5.4] 경매 진입 passthrough 프로브(`TeamPlan.version` 관측). 크래시 시 여기만 false.
const AUC_PROBE: bool = false;
//   ⛔[08-06] **목적 달성 후 OFF.** `version = 2` 를 두 판(395만·294만 관측, 전부 값 2)에서 확정했다.
//   ⚠끈 이유는 그것만이 아니다 — 이 프로브를 켠 두 판 모두 `imm_guard_summary` 가 `checked=10`(=로드시점
//     d19 패치만)으로, **retreat 훅의 매틱 apply 체인이 한 번도 안 돌았다**. 프로브 없던 직전 판은 756 이었다.
//   추정 원인: passthrough 래퍼가 반환값을 `usize`(rax)로만 되돌린다. 원본이 부동소수(xmm0)나 128비트
//     (rax:rdx)를 반환하면 **크래시 없이 반환값만 망가져** 경매 점수가 엉키고 후퇴 판단이 안 뜬다.
//   ★교훈: passthrough 훅은 인자만 맞추면 되는 게 아니라 **반환값 폭·레지스터까지** 맞아야 무해하다.
//     "크래시 안 남 = 무해" 가 아니다 — 조용한 동작 변화가 더 나쁘다.
// ★★[07-31] **SubPlan 19(DefenseNexus) 강제** — disc19 재현 검증 전용. cfg `force_sp19 = 1`.
//   왜 여기인가: ghidra-re 확정 — `MP_SAFE_DISC`에 16/17이 있어 **Plan16/17은 모드가 이미 완전대체**한다.
//   ⟹ 게임 원본(`0xdec6b0`)은 아예 실행되지 않으므로 **게임 byte-patch는 효과 0**이고,
//      진짜 제어점은 우리 재현부 `my_disc17`의 반환값이다(RE\2026-07-31_SubPlan승격게이트-패치사이트 §0·§5).
//   자연 상태에서 Plan17 56회가 전부 SubPlan 7로 떨어지는 이유 = `inBase && curHP<maxHP → 7`(유력).
//   ⚠검증 전용: 켜면 Plan17 챔프가 귀환 대신 넥서스 방어를 상시 수행 = AI 왜곡. **기본 OFF.**
static FORCE_SP19: AtomicBool = AtomicBool::new(false);
// 진단: 게이트가 실제로 도달·발동했는지. (cfg 파싱 실패 / my_disc17 미호출 / 발동했는데 게임에 전달 안 됨) 을 구분한다.
static D17_CALLS: AtomicU64 = AtomicU64::new(0);    // my_disc17 최종반환 지점 도달 수
static D17_FORCED: AtomicU64 = AtomicU64::new(0);   // 그중 0x13으로 강제한 수
// ★[07-31] disc16/17 대체 토글(A/B 실험용). **기본 true = 종전 동작 유지**. cfg `nx_repl = 0` 으로 끄면
//   그 두 disc 만 passthrough(게임 원본 실행) 가 되어, 같은 리플레이를 0/1 로 돌려 결과 차이를 볼 수 있다.
static D1617_REPL: AtomicBool = AtomicBool::new(true);
//   ★실험 계기 = 아래 `d17_calls`. 토글 OFF면 my_disc17 이 아예 안 불리므로 **d17_calls=0** 이 되어 A/B 구분이 자명하다.
static ORIG_SPDISP: AtomicUsize = AtomicUsize::new(0);
static SP_HIST: [AtomicU64; 40] = [const { AtomicU64::new(0) }; 40];
static SP_OTHER: AtomicU64 = AtomicU64::new(0);   // 40 이상/비정상 값
static SP_TOTAL: AtomicU64 = AtomicU64::new(0);
static DISC1819_CAP: AtomicBool = AtomicBool::new(false); // cfg dcap
static D18_N: AtomicU64 = AtomicU64::new(0);
static D19_N: AtomicU64 = AtomicU64::new(0);
static DISC_FILE_INIT: AtomicBool = AtomicBool::new(false);
// ── my_disc19(완전재현 Phase2-2) 대조 배선용 ──
static D19_THREAT_SHADOW: AtomicBool = AtomicBool::new(false);   // cfg d19thr: threat 활성 게이트(기본OFF→threat=0)
static D19_THREAT_PURE: AtomicBool = AtomicBool::new(true);      // cfg d19thrpure: 1=순수 d19_threat_pure(기본), 0=전체 shadow 롤백
static D19_US_SHADOW: AtomicBool = AtomicBool::new(false);       // cfg d19_us_shadow: usable_slot1/2 shadow 롤백(기본 OFF=순수)
static D19_BASEDMG_SHADOW: AtomicBool = AtomicBool::new(false);  // cfg d19_bd_shadow: vt0x28 base쌍 getter shadow 롤백(기본 OFF=순수, #4/미지 RVA는 자동 shadow 폴백)
static D19_BD_CMP: AtomicBool = AtomicBool::new(false);          // cfg d19_bd_cmp: 순수 base쌍을 shadow와 비트동일 대조(bdcmp.txt, 검증 전용)
static BD_CMP_OK: AtomicU64 = AtomicU64::new(0);
static BD_CMP_MM: AtomicU64 = AtomicU64::new(0);
static D19_CMP_N: AtomicU64 = AtomicU64::new(0);                 // dcmp 발화 카운터(200회당 1회 샘플)
static D19_CMP_INIT: AtomicBool = AtomicBool::new(false);
static D19_LAYOUT: AtomicU8 = AtomicU8::new(0xff);               // 마지막 감지 p6 레이아웃(0=직접 p6[0]=obj / 1=이중deref p6[0][0]=obj / 0xff=미해결)
// ── disc18(AttackNexus, FUN_141c81980) 완전재현 골격 대조 배선(관찰 전용, disc19 인프라 미러). 근거=§11.9.2(본체)+§11.9.11(콜리/게이트/band/zone). ──
//   ★이 단계 = 관찰/대조 전용(disc19와 동일 methodology). game sret 훼손 금지 → my_disc18은 별도 scratch에 계산. RNG-free라 재sim 무영향.
static D18_CMP_N: AtomicU64 = AtomicU64::new(0);                 // dcmp 발화 카운터(200회당 1회 샘플)
static D18_CMP_INIT: AtomicBool = AtomicBool::new(false);
static D18_LAYOUT: AtomicU8 = AtomicU8::new(0xff);               // 마지막 감지 p6 레이아웃
static D18_INRANGE: AtomicI64 = AtomicI64::new(-1);             // 계측: Phase1 in_range(적구조물 사거리내). -1=미계산
static D18_F0: AtomicI64 = AtomicI64::new(-1);                  // 계측: Phase2 zone flag0
static D18_F1: AtomicI64 = AtomicI64::new(-1);                  // 계측: Phase2 zone flag1
static D18_BAND: AtomicI64 = AtomicI64::new(-1);               // 계측: band(p3)
static D18_ABIL: AtomicBool = AtomicBool::new(false);          // cfg d18abil: Phase3/4 능력블록(0xf/0x10/0x11) 발행(기본 OFF=골격 미발행)
// ── Gate2 계측(순수 관찰: my_disc19가 Gate2 계산 시 값 캡처 → dcmp가 마지막호출 값을 라인에 append). 미도달=sentinel(thr=-1). ──
static D19_G2_THREAT: AtomicI64 = AtomicI64::new(-1);           // sentinel -1 = Gate2 미도달(phase<=0x1d/조기return)
static D19_G2_B: AtomicI64 = AtomicI64::new(0);                 // curHP (nx+0x658)
static D19_G2_HP: AtomicI64 = AtomicI64::new(0);               // hp_pct
static D19_G2_SEV: AtomicI64 = AtomicI64::new(0);              // sev 임계표 결과
static D19_G2_CV: AtomicI64 = AtomicI64::new(0);              // cvar7(disc17_pred, phase>=0x27)
static D19_G2_GD: AtomicI64 = AtomicI64::new(0);             // go_detailed as u8(0/1)
static D19_G2_PHASE: AtomicI64 = AtomicI64::new(0);         // Gate2 진입 phase
// ── Gate1 계측/토글(FUN_141c83700 bVar5+compFlag 조기홈복귀, Gate2보다 먼저). shadow-call 3종(FUN_14237d910 로컬RNG / FUN_142090ec0 TLS캐시 / getter vt0x90) = AV+재sim오염 위험(§3) → cfg d19gate1 격리, 기본 OFF=현행동작 완전보존. ──
static D19_GATE1: AtomicBool = AtomicBool::new(false);        // cfg d19gate1: Gate1 활성(기본 OFF)
static D19_VIS_GATE: AtomicBool = AtomicBool::new(true);      // cfg d19vis: struct_threat 후보건물 확률 시야 게이트(FUN_14237d910 shadow-call). 기본 ON=미노출건물 제외(실desc 4건 회귀 해소). Gate1 bVar5의 d19_g1_pred와 동일 롤(per-call 지역RNG=전역 sim-rng 미소비→재sim 안전). OFF=구동작(visible=true 가정)
static D19_G1_CF0: AtomicI64 = AtomicI64::new(-1);            // compFlag0(FUN_142090ec0 out+0x28 byte0). -1=게이트OFF/미도달
static D19_G1_CF1: AtomicI64 = AtomicI64::new(-1);            // compFlag1(out+0x29 byte1)
static D19_G1_BV:  AtomicI64 = AtomicI64::new(-1);            // bVar5(적웨이브 임박) as 0/1
static D19_G1_FIRED: AtomicI64 = AtomicI64::new(0);          // Gate1 발화(cf0||bv||cf1) → glen=1 홈복귀
// ── 전반A(4) 조기후퇴 게이트 계측(순수 관찰). Gate1/Gate2보다 앞선 홈복귀 경로(flag0||struct_threat||flag1). ──
//   이 게이트가 발화하면 emit+return이 Gate1/Gate2 계측 store보다 먼저라 기존 g1/g2가 전부 sentinel로 남았다(glen>=3 vs mlen=1 5건의 실체). 판정 직전 캡처.
static D19_A4_F0: AtomicI64 = AtomicI64::new(-1);            // flag0(zone f0). -1=게이트 미도달
static D19_A4_STRUCT: AtomicI64 = AtomicI64::new(-1);        // struct_threat(적건물 reach) as 0/1
static D19_A4_F1: AtomicI64 = AtomicI64::new(-1);            // flag1(zone f1)
static D19_A4_FIRED: AtomicI64 = AtomicI64::new(0);          // 전반A(4) 발화(f0||struct||f1) → tag3 후퇴 return(glen=1)
// ── d19_building_reaches 항별 계측(순수 관찰). struct_threat=true 유발한 첫 도달건물의 reach 각 항. -1=미발화(도달건물 없음). ──
//   과대판정(struct_threat 오발화) 원인항 특정용: R=base420+desc2+desc3term+vt90. d²<=R²이면 도달. 어느 항이 R을 과대하게 만드는지 대조.
static D19_BR_BASE: AtomicI64 = AtomicI64::new(-1);          // reach_base = rd_i64(b+0x420)  (format: base 및 rb 공용)
static D19_BR_D2:   AtomicI64 = AtomicI64::new(-1);          // d² = sqd(b, nexus)
static D19_BR_DESC2: AtomicI64 = AtomicI64::new(-1);         // desc[2] = rd_i64(desc+0x10)
static D19_BR_DESC3: AtomicI64 = AtomicI64::new(-1);         // desc3term = (lvl-1)*rd_i64(desc+0x18)
static D19_BR_VT90: AtomicI64 = AtomicI64::new(-1);          // vt0x90 필드게터 값
static D19_BR_R:    AtomicI64 = AtomicI64::new(-1);          // reach 총합 = base420 + desc_term
static D19_BR_BK:   AtomicI64 = AtomicI64::new(-1);          // b kind (b+0x68)
static D19_BR_BST:  AtomicI64 = AtomicI64::new(-1);          // b subtype (b+0x70)
static D19_BR_BID:  AtomicI64 = AtomicI64::new(-1);          // b id (b+0x5a8)
// ── 능력배정 발행블록(tag 0xf/0x10/0x11, FUN_141c83700 0x84c35~0x8525b) 토글. usable_slot1/2·vt90 range getter shadow-call(AV위험 §3) → cfg d19abil 격리, 기본 OFF=현행(tag5 완전일치) 완전보존. ON이면 tag5/tag3 뒤 능력사용 커맨드 추가발행(game disc19cmp 559건 미발행 교정). ──
static D19_ABIL: AtomicBool = AtomicBool::new(false);        // cfg d19abil: 능력배정 블록 활성(기본 OFF)
static D19_ABIL2: AtomicBool = AtomicBool::new(false);       // cfg d19abil2: 2차 abil emitter(FUN_14236ddf0) 활성(기본 OFF). 비-self 5사이트+Gate#1
static D19_LEAD: AtomicBool = AtomicBool::new(true);        // cfg d19_lead: cand_main 리드보정(기본 ON). 0xf under 격리용 진단
// ★[07-15] 0xf under 진단: cand_main/near 후보의 (target_id | slot0존재게이트<<16 | slot0inr<<17) 캡처. disc19cmp에 append.
static D19_CMDBG: [AtomicU64; 12] = [const { AtomicU64::new(0) }; 12];
static D19_CMDBG_N: AtomicUsize = AtomicUsize::new(0);
// ★[07-15] cand_main raw 5슬롯 로스터 진단: id<<8 | reject(0=수락/1=tag/2=null/3=시야/4=6a0/5=688). disc19cmp에 append.
static D19_RAWROST: [AtomicU64; 5] = [const { AtomicU64::new(0xffff_ff00) }; 5];
// ★[07-15] 1차 emitter concat3 slot0(0xf) 진단: id<<8 | code(0=emit / 1=dist / 2=team / 3=usable0 / 4=inrange). game 놓친타겟이 여기 emit(0)이면 1차서 발행中(2차대조아티팩트) / dist/inrange면 1차 사거리 / 부재면 concat3에 없음.
static D19_C1DBG: [AtomicU64; 128] = [const { AtomicU64::new(0) }; 128];
static D19_C1DBG_N: AtomicUsize = AtomicUsize::new(0);
// ★[07-15] RE 가설 확인: d19_scaled에서 P<0 발생수(0이면 SHR교정=no-op) / in_range 부호R<=0 발생수.
static D19_PNEG: AtomicU64 = AtomicU64::new(0);
// ★[07-15] slot2(0x11) 게이트 per-cand 진단: id<<8 | code(0=emit / 1=존재 / 2=tv / 3=inr / 4=score). 어느 게이트가 0x11 거부하나.
static D19_S2DBG: [AtomicU64; 12] = [const { AtomicU64::new(0) }; 12];
static D19_S2DBG_N: AtomicUsize = AtomicUsize::new(0);
// ★[07-15] 2차 slot1(0x10) 게이트 per-cand: id<<8|code(0=emit/1=존재/2=tv(+sel/tk/sm 상위비트)/3=inr/4=score).
static D19_S1DBG: [AtomicU64; 12] = [const { AtomicU64::new(0) }; 12];
static D19_S1DBG_N: AtomicUsize = AtomicUsize::new(0);
// ★[07-15] 1차 concat3 slot1(0x10)·slot2(0x11) 게이트 per-cand: id<<8|code(0=emit/1=slotvalid/2=usable/3=tv/4=inr).
static D19_C1S1: [AtomicU64; 24] = [const { AtomicU64::new(0) }; 24];
static D19_C1S1_N: AtomicUsize = AtomicUsize::new(0);
static D19_C1S2: [AtomicU64; 24] = [const { AtomicU64::new(0) }; 24];
static D19_C1S2_N: AtomicUsize = AtomicUsize::new(0);
// ★[07-15] 방어리스트(FUN_142376f00, g0+0xf0/+0x108) 3슬롯 발행 진단: id<<8|code(0=emit(≥1슬롯) / 1=invisible / 2=defflag / 3=team / 4=dist / 5=slot전부게이트). 놓친타겟(0x1ec류)이 여기 뜨면 방어경로 소스 확인.
static D19_DEFDBG: [AtomicU64; 32] = [const { AtomicU64::new(0) }; 32];
static D19_DEFDBG_N: AtomicUsize = AtomicUsize::new(0);
// ★[07-15] Path B 최근접구조물 0xf 진단: m=리스트수, best=*(best_s+0x5a8), gate 비트필드(b0=best!=0 b1=4a8!=-1 b2=s_in_range b3=unit_in_range b4=!dive b5=usable0 b6=hp>r15).
static D19_PB_M: AtomicU64 = AtomicU64::new(0xffff);
static D19_PB_BEST: AtomicU64 = AtomicU64::new(0xffff);
static D19_PB_GATE: AtomicU64 = AtomicU64::new(0);
// ★[07-15] usable_slot1 실패게이트 계측: 0=usable/1=type4/2=c8(시전중)/3=kind≠0xd/4=vtnull/5=cooldown/6=typescan/7=finalid.
static D19_US1_FAIL: AtomicU64 = AtomicU64::new(0xff);
// ★[07-15] usable_slot1 쿨다운 계측(us1f=5 진단): base(vt90)·div(vta8)·q·cd(*e+0xb8)·slot0x570 RVA.
static D19_US1_BASE: AtomicU64 = AtomicU64::new(0);
static D19_US1_DIV: AtomicU64 = AtomicU64::new(0);
static D19_US1_Q: AtomicU64 = AtomicU64::new(0);
static D19_US1_CD: AtomicU64 = AtomicU64::new(0);
static D19_US1_SLOTRVA: AtomicU64 = AtomicU64::new(0);
// ★[07-15] usable_slot2 계측(0x11 over 진단): fail게이트 + base/div/q/cd/lvl/slotRVA.
static D19_US2_FAIL: AtomicU64 = AtomicU64::new(0xff);
static D19_US2_BASE: AtomicU64 = AtomicU64::new(0);
static D19_US2_DIV: AtomicU64 = AtomicU64::new(0);
static D19_US2_Q: AtomicU64 = AtomicU64::new(0);
static D19_US2_CD: AtomicU64 = AtomicU64::new(0);
static D19_US2_LVL: AtomicU64 = AtomicU64::new(0);
static D19_US2_SLOTRVA: AtomicU64 = AtomicU64::new(0);
// ★[07-15] in_range 마지막 계산(R², d2, R) — 2차 slot1 경계건 정밀/계통 판별용.
static D19_INR_RSQ: AtomicU64 = AtomicU64::new(0);
static D19_INR_D2: AtomicU64 = AtomicU64::new(0);
static D19_INR_R: AtomicU64 = AtomicU64::new(0);
// 2차 slot1 in_range 탈락(code3) 스냅샷.
static D19_S1BR_RSQ: AtomicU64 = AtomicU64::new(0);
static D19_S1BR_D2: AtomicU64 = AtomicU64::new(0);
static D19_S1BR_R: AtomicU64 = AtomicU64::new(0);
static D19_S1BR_ID: AtomicU64 = AtomicU64::new(0);
// ★[07-14] 2차 emitter per-site/per-gate 계측(abil2_dbg.txt) — 0xf over / 0x11 under의 책임 게이트 특정용.
//   idx: 0=blk 1=ncm합 2=nen합 | 3~7=멤버십(tag유효/ptrok/vis/폴백/accept) | 8~10=slotN_ready
//   11~13=0xf(cmgate/lead발동/emit) | 14~19=0x10 cand(cmgate/tv/inr/c8/score/emit) | 20~21=0x10 near(tv/emit)
//   22~27=0x11 cand(cmgate/tv/inr/c8/score/emit) | 28~29=0x11 near(tv/emit) | 30=0x10 self emit | 31=0x11 self emit
//   32~35=descvt_90 구현별 hit(0x50fc80 / 0x23cbf20 / 0x214d210 / 미등재) | 36=descvt_90 nonzero 반환수
//   37~39=descvt_78 구현별 hit(0x50fc80=false / 0x19ec2c0=composite / 0x1e65a80=delegate)
//   40=Gate#2 호출수 | 41=Gate#2 pred단락(즉시true=무조건emit) | 42=Gate#2 컨테이너스캔 통과 | 43=Gate#2 false
static A2DBG: [AtomicI64; 48] = [const { AtomicI64::new(0) }; 48];
#[inline] fn a2(i: usize) { A2DBG[i].fetch_add(1, Ordering::Relaxed); }
// ★[07-15] 미등재 vtable 구현 RVA 런타임 캡처(모든 depth) — 정적 스캔이 못 찾은 7번째 pred 등을 실측 특정.
//   key = (slot_tag<<40 | rva). slot_tag: 0x78/0xc8/0x90/0x50/0x58/0x48. dedup 후 최대 16종 저장.
static UNREG: [AtomicUsize; 16] = [const { AtomicUsize::new(0) }; 16];
static UNREG_N: AtomicUsize = AtomicUsize::new(0);
#[inline] fn unreg_cap(slot: u16, rva: usize) {
    let key = ((slot as usize) << 40) | (rva & 0xff_ffff_ffff);
    let n = UNREG_N.load(Ordering::Relaxed).min(16);
    for i in 0..n { if UNREG[i].load(Ordering::Relaxed) == key { return; } }
    let idx = UNREG_N.fetch_add(1, Ordering::Relaxed);
    if idx < 16 { UNREG[idx].store(key, Ordering::Relaxed); }
}
// 진단: abil 블록 게이트별 통과수. [0]블록진입 [1]후보 [2]dist통과 [3]team통과 / slot0:[4]usable[5]emit0xf / slot1:[6]usable[7]emit0x10 / slot2:[8]usable[9]emit0x11 / [10]cand총순회 [11]self_team1
static ABIL_DBG: [AtomicI64; 16] = [const { AtomicI64::new(0) }; 16];
// 진단: my_disc19 진행 스테이지(panic 위치). 1=진입 2=nx해결 3=struct후 4=zone후 5=A4후 6=Gate1전 7=Gate2전 8=threat후 9=go_detailed후 10=phase_b진입. dcmp가 rc=-98시 기록.
static D19_STAGE: AtomicI64 = AtomicI64::new(0);
// ── 시야 확률롤(FUN_14237d910) 순수/shadow 전환(cfg d19_g1_shadow, 기본 OFF=순수). ─────────────────
//   순수재현(d19_g1_pred_pure, seed tick항=max(tick-r14,0)<<0x28[최신E])이 shadow(FUN_14237d910 shadow-call)와
//   17790건 비트동일(mmN=0)로 검증 완료(2026-07-11 vispure 병행계측). 검증 끝났으므로 순수를 기본으로 승격.
//   ON=shadow-call로 롤백(AV위험 §3 격리, 안전 대비용). 순수는 로컬RNG(전역 sim-rng 미소비→재sim 안전).
static D19_G1_SHADOW: AtomicBool = AtomicBool::new(false);   // cfg d19_g1_shadow: 시야롤을 shadow-call로 롤백(기본 OFF=순수재현)

// ════════════════════════════════════════════════════════════════════════════════════════════════
// disc19(DefenseNexus) 완전재현 Phase2-2 — my_disc19 (전반A + tag3 후퇴 emit + 후반B 골격) + dcmp 배선
//   근거: §11.9.3(제어흐름·필드맵)·§11.9.5(콜리·게이트)·§11.9.1(콜리)·§11.9.4(usability leaf)·§11.9.6(zone 산술).
//   ★이 단계 = 관찰/대조 전용. game sret 훼손 금지 → out은 disc19_dcmp가 넘기는 별도 scratch Vec.
//   ★RNG-free(전역 sim-rng 미소비)라 병렬계산=재sim 무영향. rd_*(safe VEH)만 사용, shadow-call은 d19thr 게이트(기본OFF).
//   TODO(남은 잔여): 후반B 9단계 본체 · zone 실계산(FUN_14209a750 §11.9.6) · 위협점수 순수재현(FUN_1420a3fd0) · usability leaf vt3슬롯.
// ════════════════════════════════════════════════════════════════════════════════════════════════
const D19_STRIDE: usize = 0x500;   // ★0.5.1(was 0.5.0_3 0x4f8). Command 구조체 +8 성장(verb앞 +0x4a0에 8B destY 삽입, tag +0x4f1→+0x4f9). ghidra 확정(imul→LEA+SHL 0x500).
const D19_HOME_HI: u64 = 0xe2900;   // 930048
const D19_HOME_LO: u64 = 0x7d00;    // 32000
// usable_slot2 순수재현(FUN_141fbe950) ★[07-12 의사코드 확정]: fce700 구조 + 차이 —
//   obj p=lvl>2?e+0x4e8:&DAT_14385e5e0(zero-init desc, p[6]=-1 → lvl<3이면 c8 게이트 실질 skip) /
//   base=항상 *(e+0x578)/*(e+0x580) / divisor만 lvl<3에서 *(e+0x598)/*(e+0x5a0) 쌍 교체 /
//   q 하한 1(q+=(q==0), fce700의 3과 다름) / 임계=*(e+0xc0) / 최종=p[6]!=-1 && lvl>=3.
const D19_SLOT2_EMPTY_RVA: usize = 0x38d1af0; // ⏸**0.5.3 미재핀 = 0.5.2값 유지**(2026-07-29): empty-descriptor(전 0)라 값지문 변별 불가. rd_u64(VEH 가드) 읽기 전용 = 크래시 없음, 재현 정확도만 저하.  // ⏸**0.5.3 미재핀 = 0.5.2값 유지**(2026-07-29): 이 desc는 전부 0으로 채워진 empty-descriptor라 .rdata 값지문으로 변별 불가(0 블록이 0.5.3에 6,230개). 사용처는 rd_u64(VEH 가드) 경유 읽기 전용 = **크래시 없음**, 재현 정확도만 저하.  // ★**0.5.2 확정**(ghidra-re 07-22, ~~0.5.1 0x3846d50~~). 사용처=disc19_repro(dcap 게이트 dev코드)라 프로덕션 무영향이나 재현 정확도 위해 반영. // 구:0.5.1(was 0.5.0_3 0x385e5e0). DAT_143846d50. ghidra-re HIGH 확정(disc19 핸들러 0x1e0ddb0: reach+0x5b0<5/<3 fallback, +0x30 guard=_UNK_143846d80)
// usable_slot1/2 shadow-call(FUN_141fce700 / FUN_141fbe950) — 롤백 전용(d19_us_shadow). this=self만, 반환 저비트=bool.
type D19Us = unsafe extern "C" fn(usize) -> u64;

// STATIC_TEMPLATE(슬롯2에서 *(self+0x5b0)<3일 때 desc/guard 소스). ★ghidra 확정: LEA R15,[0x14380d3f0](=DAT_14380d3f0).
//   절대주소 exe_base()+0x380d3f0. rip-rel disp 0x1b8833e 일치. guard=*(slot2+0x30), desc=slot2+0x28.
const D19_STATIC_TEMPLATE_RVA: usize = 0x38d1af0; // ⏸**0.5.3 미재핀 = 0.5.2값 유지**(2026-07-29): empty-descriptor(전 0)라 값지문 변별 불가. rd_u64(VEH 가드) 읽기 전용 = 크래시 없음, 재현 정확도만 저하.  // ⏸**0.5.3 미재핀 = 0.5.2값 유지**(2026-07-29): 이 desc는 전부 0으로 채워진 empty-descriptor라 .rdata 값지문으로 변별 불가(0 블록이 0.5.3에 6,230개). 사용처는 rd_u64(VEH 가드) 경유 읽기 전용 = **크래시 없음**, 재현 정확도만 저하. // ★**0.5.2 확정**(ghidra-re 07-22, ~~0.5.1 0x3846d50~~). SLOT2_EMPTY와 동일 객체(0.5.1서 통합된 단일 empty-descriptor)라 같은 값. // 구:0.5.1(was 0.5.0_3 0x380d3f0). ghidra-re HIGH 확정: LEA R14,[0x143846d50]@0x141e0f7cb(tag0x11 발행부). ★0.5.1서 SLOT2_EMPTY와 단일 empty-descriptor로 통합(0.5.0_3의 별개 2객체→1객체)

// ═══════════════════════════════════════════════════════════════════════════════════════════════
// disc19 2차 abil emitter FUN_14236ddf0(0x236ddf0) 완전재현 — §11.9.11-2 실디컴 확정(0.5.0_3 buildid 24125999)
//   후보소스 3종(cand_main via FUN_1422a1180 멤버십 / enemies_near 자팀근접 / self) × 7 emit 사이트.
//   ★이번 구현 = 비-self 5사이트(cand_main 0xf/0x10/0x11 + enemies_near 0x10/0x11) + Gate#1 스코어게이트 완전재현.
//   ★defer(후속): self-target(0x10 self·0x11 self+AoE) = Gate#2(FUN_1423802d0 pred=vtable슬롯0x78)·
//      Gate#3(FUN_1421283d0 폴리모픽 vt0x198/0x150·4-leaf) 런타임 슬롯해결 필요 → 추정재현 시 over-emit/AV 위험이라 보류.
//   변수: g0=*p6_pair(로스터베이스), sim_obj=*g0(geom gc), geom2=*(p6_pair+0x10)=teamdata, self_u=nx, side=flag, src=vt0x28(gc).
//   전부 순수(기존 헬퍼 재사용: geom_vt68/vtc0/vt28·d19_target_valid·d19_in_range·d19_threat_dmg·vtc8_get·vt90_get).
// ═══════════════════════════════════════════════════════════════════════════════════════════════
const D19_STATIC2_TEMPLATE_RVA: usize = 0x38d17b8; // ⏸**0.5.3 미재핀 = 0.5.2값 유지**(2026-07-29): empty-descriptor(전 0)라 값지문 변별 불가. rd_u64(VEH 가드) 읽기 전용 = 크래시 없음, 재현 정확도만 저하.  // ⏸**0.5.3 미재핀 = 0.5.2값 유지**(2026-07-29): 이 desc는 전부 0으로 채워진 empty-descriptor라 .rdata 값지문으로 변별 불가(0 블록이 0.5.3에 6,230개). 사용처는 rd_u64(VEH 가드) 경유 읽기 전용 = **크래시 없음**, 재현 정확도만 저하.  // ⏸**0.5.2 미확정=0.5.1값 유지**(ghidra-re 07-22: 2차 emitter 재식별 실패·0 desc라 값 변별 불가). 사용처=disc19_repro slot2_base(dcap 게이트 dev코드)=프로덕션 무영향. ⚠0.5.2 확정 이웃(SLOT2 0x38d1af0·DISC7시트 0x38d1918)과 같은 0x38d1 대역이나 **우연 일치일 수 있으므로 근접 추정 금지**. // 구:0.5.1(was 0.5.0_3 0x38aecc0). ghidra-re 확정: 2차 emitter FUN_14238b290 내 LEA R12,[0x1438d17b8]@0x14238b738 + F80320@0x14238a2ce 이중확증. STATIC(0x3846d50 통합)과 달리 별도 desc 유지

// ════════ Gate#3 FUN_1421283d0(0x1283d0) AoE/셰이프 게이트 — site7의 AoE 분기 전용 ════════
//   ★[07-15 확정] 셀 슬라이스 = vt+0x198 = RVA 0x19f03d0 = `{*(world+0xb178), *(world+0xb180)}` (ptr,len) trivial 게터.
//   셀→엔티티 = vt+0x150 = RVA 0x20ad690 = **기존 geom_resolve150 재사용**.
//   loop1(사거리 내 유효 적) → loop2(셀 내 적이 자기 4스킬로 caster를 역으로 때릴 수 있나).
const D19_TV7_RVA: usize = 0x32105a8;   // ★0.5.3(was 0.5.2 0x3863a28). .rdata 값지문 **선두 48B**(`07 00 00 00` + "8DataEffectDef::Nati…") 가 OLD/NEW 각 1건 = 유일 매칭. // 구 ★0.5.2(was 0.5.1 0x38b7d50). version-migrator 확정: 참조사이트 마스크시그 UNANIMOUS(2/2) + **값 sanity 완전일치**(선두 16B `07 00 00 00 38 44 61 74 ...`= u32==7 desc 헤더가 구값과 바이트동일). // ★0.5.1(was 0.5.0_3 0x38796f8). target_valid selector=7 desc. ghidra-re 확정: u32==7 desc+LEA 2회 유일후보(@0x142281e09/eba, 참조간격 0xb4=0.5.0_3 Gate#3와 바이트동일)

// ════ Gate1(FUN_141c83700 bVar5+compFlag, ghidra 실측 a48105662) — Gate2보다 먼저 판정하는 조기 홈복귀 ════
//   game: cf0!=0 || bVar5 || cf1!=0 → tag5 전부 스킵, tag3 홈복귀 1개만(glen=1). 전부 D19_GATE1 게이트(기본OFF)로 격리.
//   ⚠3 shadow-call: FUN_14237d910(확률술어, type!=0xd=로컬RNG/type==0xd=subtype 점프테이블), FUN_142090ec0(동적 위협그리드, TLS 메모캐시),
//     getter vt0x90(스킬사거리). 전부 exe_base()+rva + catch_unwind + ptr_ok/readable 가드. 재sim 무결성은 상위세션 런타임검증.
type Fn2090 = unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, u8) -> usize;
// ★★[0.5.3] 인자 3개 → **4개**(선두 삽입). ghidra-re 실측(2026-07-31, RE\2026-07-31_shadowcall-4종):
//   0.5.0_3 `(rcx=p5, rdx=p6, r8=e)` → 0.5.3 `(rcx=dead, rdx=p5, r8=p6, r9=e)`.
//   게임 콜사이트 `0x140ded006`도 4개를 세팅한다. ⛔3인자로 호출하면 인자가 한 칸씩 밀려 **즉시 AV**.
type F237d  = unsafe extern "C" fn(usize, usize, usize, usize) -> u64;
// ── vt0x170 오브젝티브 트리 빌더 concrete 타깃 캡처(compFlag 순수화 1차 블로커, cfg d19_g1cap 기본 OFF) ──
//   gameobj=*(*p6), vtbl=*(*p6+8), vt170=*(vtbl+0x170). 순수 read(호출 없음). distinct RVA+프롤로그 → g1cap.txt.
static D19_G1CF_SHADOW: AtomicBool = AtomicBool::new(false);   // ★[0.5.1] 기본 OFF=순수 d19_g1_compflag_pure(compFlag MM=0 검증완=DONE). 구 기본 ON은 shadow FUN_142090ec0 호출인데 0.5.1서 그 함수 0x2090ec0→0x236b6b0 재배치=stale이라 dcap=1 크래시 유발 → 순수로 전환(doctrine·안전). shadow 재활성하려면 disc19_repro.rs:2560 RVA 0.5.1 갱신 선행
static D19_G1CF_CMP: AtomicBool = AtomicBool::new(false);      // cfg d19_g1cf_cmp: 순수 vs shadow A/B 대조(g1cfcmp.txt)
static G1CF_OK: AtomicU64 = AtomicU64::new(0);
static G1CF_MM: AtomicU64 = AtomicU64::new(0);
static D19_G1CF_LOOP2: AtomicBool = AtomicBool::new(true);     // cfg d19_g1cf_loop2: loop2 threat항 격리(기본 ON). 진단용 OFF=de40만
// ── disc19 AI 성향 튜닝 계수(순수화 완료 후 개입지점). 전부 기본값=게임 원본 상수 → 미설정 시 비트동일 보존, 변경 시 튜닝. ──
static D19_THREAT_MULT: AtomicI64 = AtomicI64::new(100);   // cfg d19_threat_mult: 위협점수 배수%(주력). >100=수비적(일찍 후퇴), <100=공격적
static D19_RETREAT_HP:  AtomicI64 = AtomicI64::new(0x2d);  // cfg d19_retreat_hp: go_detailed HP%문턱(45). 높이면 높은 HP에도 후퇴(수비적)
static D19_RANGE_ATKME: AtomicI64 = AtomicI64::new(100);   // cfg d19_range_atkme: '나를 공격중' 적 위협 가중(100)
static D19_RANGE_BLD:   AtomicI64 = AtomicI64::new(0x3c);  // cfg d19_range_bld: '내 건물 공격중' 가중(60)
static D19_RANGE_OTHER: AtomicI64 = AtomicI64::new(0x28);  // cfg d19_range_other: '딴 대상 공격중' 가중(40)
static D19_RANGE_IDLE:  AtomicI64 = AtomicI64::new(0x50);  // cfg d19_range_idle: '비교전(놀고있음)' 가중(80)
// 진단: 마지막 pure 호출의 내부상태(불일치 특정용). [0]=de40cf0 [1]=loop2cf0 [2]=cf1 [3]=nobj [4]=na [5]=nb / [8..24]=obj cats(≤16)
static G1CF_DBG: [AtomicI64; 24] = [const { AtomicI64::new(0) }; 24];
// 진단: cat2 SPECIAL 5개 other-슬롯 덤프. 각 code = slotvalid*1e7+uok*1e6+dist*1e5+side*1e4+geom*1e3+fcf*1e2+same*10+guard. [5]=desc group low16
static G1CF_SLOT: [AtomicI64; 6] = [const { AtomicI64::new(0) }; 6];
static G1CAP_ON: AtomicBool = AtomicBool::new(false);          // cfg d19_g1cap
static G1CAP_CNT: AtomicU64 = AtomicU64::new(0);
static G1CAP_SET: [AtomicUsize; 16] = [const { AtomicUsize::new(0) }; 16];

// ── dcmp: my_disc19를 별도 scratch Vec에 계산 → game sret와 대조 → disc19cmp.txt 직접write(LOG_ON 무관) ──
//   ★★[2026-07-14] scratch를 **thread_local화**(구: 단일 전역 D19_SCRATCH/D18_SCRATCH).
//     구조: dcap=1이면 rayon 병렬 sim 워커 N개가 **같은 버퍼**에 write_bytes(memset)+커맨드 write+len 갱신 →
//     데이터 레이스(대조결과 오염 + 크래시 의심). handoff §5가 지시한 조치 = thread_local.
//     Cell<usize>(소멸자 없음) → 디투어서 TLS 접근 안전. 스레드당 20KB VirtualAlloc 1회(워커 수만큼, 해제 안 함=프로세스 수명).
thread_local! {
    static D19_SCRATCH_TL: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    static D18_SCRATCH_TL: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}
const D19_SCRATCH_SZ: usize = 0x20 + 16 * D19_STRIDE;   // 헤더 0x20 + 16 command

// ★스칼라(rax) 반환 replace detour: cap_fn(saved,entry_rsp)->i64. 반환값=RAX_SENT(=i64::MIN)면 passthrough(원본실행),
//   그 외면 그 값을 rax로 caller에 반환(원본 skip). install_detour와 saved레이아웃 동일(push rcx/rdx/r8/r9/r10/r11).
//   ★rng_repl off시 cap_fn이 항상 SENT 반환 → install_detour와 동일 동작(안전).
const RAX_SENT: i64 = i64::MIN;

// ★0.4.14 교전 위협게이트(FUN_1420a8680) 캡처/개입 — retreat_engage(정글러) 콜사이트 0x1feca43 래핑.
//   p2(rdx=[rbp+0x1d8])가 게이트로 가는 스칼라(<12면 좌표박스 정밀게이트 스킵=쉽게 후퇴). 게임 위협계산식(좌표박스/거리²/추격반경)은 그대로 두고 p2만 cfg 배율.
//   이 콜사이트만 래핑 → 공유헬퍼(7~8 judge) 中 retreat만 영향, 라이너 dd7700과 분리. RNG-free=시드동기화 무손상. mult=100=원본 비트동일.
static TG_CAP: AtomicBool = AtomicBool::new(false);   // cfg tgcap: p2 로깅 on/off (1단계 캡처)
static TG_N: AtomicI64 = AtomicI64::new(0);
static TG_MULT: AtomicI64 = AtomicI64::new(100);      // cfg jungle_retreat_threat 0~200(100=원본). <100=p2↓→더 잘 후퇴 / >100=p2↑→덜 후퇴(기지 다이브만)

// ══ dd7700 callee 재구현 (전부 PURE 포인터연산; 게임함수 호출 0) ══════════════
// slot+0x20(tick getter): ★07-10 정정 = *(sim+0xeac0) (구 0xed00 오식별 — 0.5.0 tick getter concrete 0x19f0620 asm 확정, 0xed00은 0xeb08 홀더선 구조체 밖 쓰레기 → 커버 count 윈도우 간헐 misfire 9/400 원인)
#[inline] unsafe fn dd7_slot20(sim: usize) -> i64 { rd_i64(sim + 0xeb00).unwrap_or(0) }
// ★sim 헤더 캐시(호출당 1회): slot48/a8가 매 호출 재읽던 base/count 필드. 호출간 캐싱 아님 — judge 1회 호출 내에서만 재사용(동기 스코프, stale 없음).
//   b6e8=엔티티arena base, c6f0=그 limit, t700=핸들테이블, c708=그 limit, b808=레코드배열 base, c810=그 count.
#[derive(Clone, Copy, Default)]
struct SimHdr { b6e8: usize, c6f0: u64, t700: usize, c708: u64, b808: usize, c810: u64, tick: i64, seed: u64 }
#[inline] unsafe fn sim_hdr(sim: usize) -> SimHdr {
    // 0.5.0 포팅: holder(SimulationStateP) 필드 +0x38 시프트. tick=+0xeac0(★07-10 asm 정정, 구 0xed00 오식별).
    SimHdr {
        b6e8: rd_u64(sim+0x720).unwrap_or(0) as usize,   // 0.5.0(was 0x6e8) entity arena base
        c6f0: rd_u64(sim+0x728).unwrap_or(0),            // 0.5.0(was 0x6f0) entity count
        t700: rd_u64(sim+0x738).unwrap_or(0) as usize,   // 0.5.0(was 0x700) handle table
        c708: rd_u64(sim+0x740).unwrap_or(0),            // 0.5.0(was 0x708) handle count
        b808: rd_u64(sim+0x840).unwrap_or(0) as usize,   // 0.5.0(was 0x808) record base
        c810: rd_u64(sim+0x848).unwrap_or(0),            // 0.5.0(was 0x810) record count
        tick: rd_i64(sim+0xeb00).unwrap_or(0),   // ★레버4: slot_a8 캐시 무효화 키(현재틱). ★07-10 정정 0xed00→0xeac0(tick getter 0x19f0620 asm 확정)
        seed: rd_u64(sim+0xeb28).unwrap_or(0),   // ★[07-29] 경기 식별자(주소 재사용 시 남의 경기 맵 반환 차단) ★[08-06] 구 0xeaf8 → 0xeb28
    }
}
// ★레버4: dd7_slot_a8 프레임 캐시. id→record 매핑을 (base,cnt,tick)당 1회 빌드(O(cnt)) 후 O(1) 조회.
//   rayon 워커별 thread_local(경기간 격리). 틱/배열base/cnt 변경시 자동 재빌드 = stale 차단.
// ★★[07-29 결정성 수정] 캐시 키에 **seed(경기 식별자)** 추가. 구 키 (base,cnt,tick)는 **경기를 구분하지 못했다**:
//   배경 워커는 경기를 연달아 sim하므로 A 종료→해제→B가 **같은 주소에 재할당**되면 base 동일·cnt 동일(로스터 10)·tick 동일
//   ⟹ 3키 전부 충돌 → **남의 경기 로스터 맵 반환**. 관전 워커는 경기 이력이 달라 충돌 양상이 달라져 **두 sim이 다른 값을 봄**
//   = 배경≠관전 발산. seed(World+0xeab8)는 경기마다 고유하므로 키에 넣으면 교차오염이 원천 차단된다.
struct A8Cache { base: usize, cnt: u64, tick: i64, seed: u64, map: HashMap<u64, usize, FnvBuild> }
thread_local! {
    static A8_CACHE: RefCell<A8Cache> = RefCell::new(A8Cache { base: 0, cnt: 0, tick: -1, seed: 0, map: HashMap::with_hasher(FnvBuild) });
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
        if cache.base != base || cache.cnt != cnt || cache.tick != h.tick || cache.seed != h.seed {   // ★seed 포함(경기 교차오염 차단)
            cache.base = base; cache.cnt = cnt; cache.tick = h.tick; cache.seed = h.seed;
            cache.map.clear();
            let mut k = 0u64;
            while k < cnt {
                let rec = base + (k as usize)*0x8d0;                  // 0.5.0(was 0x758, record stride +0x178)
                if rd_u64(rec+0x8b8).unwrap_or(0) != 0 {              // 0.5.0(was 0x740, in-record flag +0x178)
                    let rid = rd_u64(rec+0x8c0).unwrap_or(0);         // 0.5.0(was 0x748, in-record id +0x178)
                    cache.map.entry(rid).or_insert(rec);
                }
                k += 1;
            }
        }
        cache.map.get(&id).copied().unwrap_or(0)
    })
}
#[inline] unsafe fn dd7_slot_a8(sim: usize, id: u64) -> usize {
    dd7_slot_a8_h(&SimHdr {   // 0.5.0: holder +0x38
        c810: rd_u64(sim+0x848).unwrap_or(0),            // 0.5.0(was 0x810)
        b808: rd_u64(sim+0x840).unwrap_or(0) as usize,   // 0.5.0(was 0x808)
        tick: rd_i64(sim+0xeb00).unwrap_or(0),   // ★레버4: 캐시 키. ★07-10 정정 0xed00→0xeac0
        seed: rd_u64(sim+0xeb28).unwrap_or(0),   // ★[07-29] 경기 식별자(교차오염 차단) ★[08-06] 구 0xeaf8 → 0xeb28
        ..Default::default()
    }, id)
}
// slot+0x128 = entity_handle_deref(sim, handle): 2단계(0x820→0x700) → 엔티티(0x6a8)/0
// 0.5.0 포팅(게임 composed resolve @0x220262c 실측). sim=self_obj=SimulationStateP 홀더(per-athlete sim과 별개).
//   holder 필드 = 0.4.14 대비 +0x38 시프트 / L2 record(roster stride+in-record) = +0x178 / entity stride 0x6a8·node 0x10 = 불변.
unsafe fn dd7_slot128(sim: usize, h: u64) -> usize {
    if h >= rd_u64(sim+0x860).unwrap_or(0) { return 0; }               // 0.5.0(was 0x828, holder +0x38) L1 count
    let t1 = rd_u64(sim+0x858).unwrap_or(0) as usize;                  // 0.5.0(was 0x820) L1 handle-table ptr
    if !ptr_ok(t1) || rd_i32(t1 + (h as usize)*0x10).unwrap_or(0) != 1 { return 0; }
    let u1 = rd_u64(t1 + (h as usize)*0x10 + 8).unwrap_or(0);
    if u1 >= rd_u64(sim+0x848).unwrap_or(0) { return 0; }              // 0.5.0(was 0x810) L2 roster count
    let s808 = rd_u64(sim+0x840).unwrap_or(0) as usize;               // 0.5.0(was 0x808) L2 roster base ptr
    let lv2 = (u1 as usize)*0x8d0;                                     // 0.5.0(was 0x758, record stride +0x178)
    if !ptr_ok(s808) || rd_u8(s808+0x8b8+lv2) == 0 { return 0; }       // 0.5.0(was 0x740, in-record flag +0x178)
    let u2 = rd_u64(s808+lv2+0x8c0).unwrap_or(0);                     // 0.5.0(was 0x748, in-record handle +0x178)
    if u2 >= rd_u64(sim+0x740).unwrap_or(0) { return 0; }             // 0.5.0(was 0x708) L3 count
    let s700 = rd_u64(sim+0x738).unwrap_or(0) as usize;              // 0.5.0(was 0x700) L3 handle-table ptr
    let l3 = (u2 as usize)*0x10;
    if !ptr_ok(s700) || rd_i32(s700+l3).unwrap_or(0) != 1 { return 0; }
    let u3 = rd_u64(s700+l3+8).unwrap_or(0);
    if u3 >= rd_u64(sim+0x728).unwrap_or(0) { return 0; }             // 0.5.0(was 0x6f0) entity count
    (u3 as usize)*0x6a8 + rd_u64(sim+0x720).unwrap_or(0) as usize     // 0.5.0(was 0x6e8, entity base +0x38; stride 0x6a8 불변)
}
// ─── disc12 SerpenBattle 게임 vtable shadow-call 순수재현(gchild=판단 SimState, dd7_slot128과 동일 필드 레이아웃) ───
// vt0x150 재현 (게임 RVA 0x20ad690): 1-stage 핸들→엔티티 + 싱글턴 폴백. (dd7_slot128의 L3~entity + fallback)
#[inline] unsafe fn geom_resolve150(gc: usize, h: u64) -> usize {
    if h < rd_u64(gc+0x740).unwrap_or(0) {                      // L3 count
        let t3 = rd_u64(gc+0x738).unwrap_or(0) as usize;        // L3 table
        if ptr_ok(t3) && rd_i32(t3 + (h as usize)*0x10).unwrap_or(0) == 1 {
            let u = rd_u64(t3 + (h as usize)*0x10 + 8).unwrap_or(0);
            if u < rd_u64(gc+0x728).unwrap_or(0) {              // entity count
                return (u as usize)*0x6a8 + rd_u64(gc+0x720).unwrap_or(0) as usize;  // entity base+stride
            }
        }
    }
    // 인라인 싱글턴 폴백
    if rd_u64(gc+0x618).unwrap_or(u64::MAX) == h && rd_i32(gc+0x70).unwrap_or(-1) != -1 { return gc + 0x70; }
    0
}
// vt0xc0 재현 (게임 RVA 0x20b3790): 시야레코드 lookup. L2 roster 선형탐색 (in-use rec+0x8b8!=0, id rec+0x8c0==key). laneidx=rec+0x8b0.
#[inline] unsafe fn geom_vtc0(gc: usize, key: u64) -> usize {
    let base = rd_u64(gc+0x840).unwrap_or(0) as usize;
    let cnt = rd_u64(gc+0x848).unwrap_or(0) as usize;
    if !ptr_ok(base) { return 0; }
    for i in 0..cnt.min(4096) {
        let rec = base + i*0x8d0;
        if rd_u64(rec+0x8b8).unwrap_or(0) == 0 { continue; }
        if rd_u64(rec+0x8c0).unwrap_or(0) == key { return rec; }
    }
    0
}
// vt0x20 재현 (RVA 0x19f0610): getter → *(u64)(gc+0xeab8)
#[allow(dead_code)]
#[inline] unsafe fn geom_vt20(gc: usize) -> u64 { rd_u64(gc+0xeb28).unwrap_or(0) }
// vt0x28 재현 (RVA 0x19f0620): getter → *(i64)(gc+0xeac0) = 게임 틱(프레임카운터). compFlag loop2 tick게이트/site3 seed.
#[allow(dead_code)]
#[inline] unsafe fn geom_vt28(gc: usize) -> i64 { rd_i64(gc+0xeb00).unwrap_or(0) }
// vt0x50 재현 (RVA 0x2105f70): lea → gc+0xeb08 (서브객체 앵커 주소)
#[allow(dead_code)]
#[inline] unsafe fn geom_vt50(gc: usize) -> usize { gc + 0xeb48 }
// vt0x70 재현 (RVA 0x19f0470): 30×30 int32 점유/위협 그리드. (gc, side, gx=tx/32000, gy=ty/32000) → cell>0
#[inline] unsafe fn geom_vt70(gc: usize, side: usize, gx: usize, gy: usize) -> bool {
    if gx > 29 || gy > 29 || side >= 2 { return false; }
    rd_i32(gc + 0xb278 + side*0xe10 + gy*0x78 + gx*4).unwrap_or(0) > 0
}
// f6f720 mode=2 레인밴드 predicate(VOBJ, cx, cy). 맵경계 = *(VOBJ+8)+0x12b8(Xmax)/+0x12c0(Ymax).
unsafe fn dd7_f6f720_m2(vobj: usize, cx: u64, cy: u64) -> bool {
    // ★[08-05 감사] 이 mode 2 사본만 임계 4종을 **하드코딩**하고 있었다 — m0/m1은 tune()을 쓰는데
    //   여기만 안 써서 `pf_*` 4키가 mode 2 경로에서 통째로 무효였다(반쪽 노브). 같은 키로 배선한다.
    let pf_edge = tune("pf_edge_margin", 0x2ee00) as u64;    // 맵 가장자리 여유(원본 192000)
    let pf_band = tune("pf_center_band", 0xabe00) as u64;    // 중앙 밴드 폭(원본 704000)
    let pf_dnear = tune("pf_diag_near", 63999) as u64;       // 대각 근접(원본 pf_dnear)
    let pf_dfar  = tune("pf_diag_far", 96000) as u64;        // 대각 원거리(원본 pf_dfar)
    let m = rd_u64(vobj+8).unwrap_or(0) as usize;
    if !ptr_ok(m) { return true; }
    let ymax = rd_u64(m+0x12c0).unwrap_or(0);
    let u5 = ymax.wrapping_sub(cy);                       // uVar5 = Ymax - cy
    let mut u6 = if cx < u5 { u5 } else { cx };           // max(cx,u5)
    if u6 <= pf_edge { return true; }                    // <=192000 → true
    u6 = if u5 < cx { u5 } else { cx };                  // min(cx,u5)
    let xmax = rd_u64(m+0x12b8).unwrap_or(0);
    if u6 >= xmax.wrapping_sub(pf_edge) { return true; } // >= Xmax-192000 → true
    let h1 = xmax.wrapping_sub(pf_band) >> 1;            // (Xmax-704000)/2
    let cond1 = h1.wrapping_add(pf_band) < cx || cx < h1;
    let h2 = ymax.wrapping_sub(pf_band) >> 1;
    let cond2 = h2.wrapping_add(pf_band) < cy || cy < h2;
    if cond1 || cond2 {
        if pf_edge < cx {
            let u6b = cx.wrapping_sub(u5);
            if u5 <= cx { return pf_dnear < u6b; }
        }
        false
    } else {
        let u6c = u5.wrapping_sub(cx);
        let u4 = if cx <= u5 { u6c } else { cx.wrapping_sub(u5) };
        if (cx > u5 || u6c == 0) && pf_edge < cx && u4 < pf_dfar {
            return pf_dnear < (0u64).wrapping_sub(u6c);
        }
        false
    }
}
// 제곱거리(u64; 좌표 0~960000, 합 ~1.8e12 → u64 안전)
#[inline] fn sqd(x1:u64,y1:u64,x2:u64,y2:u64) -> u64 {
    let dx = if x1>=x2 {x1-x2} else {x2-x1}; let dy = if y1>=y2 {y1-y2} else {y2-y1};
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}
// f22e80 COUNT 재현: OTHER측 **적 챔피언 5명(역할 슬롯 0~4)** 순회, 슬롯마다 gen_range(p4,[wlo,whi]) draw + 필터 통과분 카운트.
//   ⚠[08-05 주석 정정] 구 주석의 "5빌딩"은 오기였다(값·동작은 동일해 영향 없음).
//   ★이 함수의 실체 = "생존자 카운트"가 아니라 **"기준점 반경 R 안에 지금 있을 수 있는 적"** 을
//     마지막 목격 시각·목격 위치·이동속도로 **위치 불확실성을 모델링해 거른 목록의 len**이다.
//     게임측 원본은 32B `Vec<*const Champion>`을 반환하고 호출자 13곳 중 대부분이 len만 쓴다.
//     근거 = RE\2026-08-05_cef270출력계약-big_goal-champion_side레이아웃-0.5.3.md
// this=sim(L80[0]). 슬롯함수는 dd7_slotXX(sim) 재구현 사용. tgt=(tgtx,tgty)=target좌표, k=150000.
unsafe fn my_f22e80_count(rng: &mut RngSim, l80: usize, geo: usize, p5: usize, p7: usize,
                          sim: usize, wlo: u64, whi: u64, tgtx: u64, tgty: u64, k: u64,
                          p6: usize, kind: i64) -> u64 {   // ★[07-23] p6(=L 도출)·kind(GameMode) 추가
    // ★★[07-23 마이그 누락 수정] 이 함수만 **0.4.x→0.5.0 마이그레이션에서 통째로 누락**돼 있었다(0.5.2 `0x2126610` 전수 disasm 확정).
    //   같은 값들이 모드 다른 곳엔 이미 올바르게 반영돼 있었다(dd7700 본체 `p5+0x820`·`geom_vtc0` 주석 `rec+0x8b0`가 증거)
    //   ⟹ 한 함수 안에서 side가 두 값으로 갈리고 있었다. **disc0 잔여 DIFF(my=6/7 vs game=2)의 진범**.
    //   ~~`p5+0x6a8`(0.4.x)~~ → **`p5+0x820`** (원본 `@142126846 mov rcx,[r14+0x820]`)
    let side = rd_i64(p5+0x810).unwrap_or(-1);//  ★0.5.4 오프셋 이동 반영
    if side != 0 && side != 1 { return 0; }
    let (s, other) = (side as usize, (1 - side) as usize);
    let hdr = sim_hdr(sim);   // ★호이스트: 빌딩루프 slot48/a8 재사용(non-_h 매호출 sim_hdr 재읽기 제거)
    let mut count: u64 = 0;
    // ══════════════════════════════════════════════════════════════════════════════════════
    // ★★[07-23] **mode != 2 경로 신설** (0.5.2 `0x2126610` 전수 disasm). ⚠**실전이 여기다.**
    //   원본의 mode 비교는 함수 전체에서 `!= 2` **한 종류뿐**(`@1421266bf`·`@1421269b7`)
    //   ⟹ **mode 0(Moba)·1(SingleLane)은 완전히 동일 경로**, mode 2(DeathMatch)만 별도. 2갈래면 충분.
    //   ⚠**인게임 실측 `kind=0`** ⟹ 기존 재현(mode2 전용)은 **실전에서 한 번도 맞은 적이 없다**.
    //   ⟹ 이것이 disc0 잔여 DIFF의 진짜 원인: 조기게이트 발화 시 원본은 **빈 Vec(COUNT=0)** 을 반환하고
    //     dd7700 GATE D(`near_cnt >= COUNT`)가 **무조건 참**이 되어 게임은 항상 2를 낸다.
    //     모드는 COUNT를 만들어 GATE D를 통과해 STAGE6까지 내려가 6/7을 냈다(TERM=52 26건).
    //   ★전역 RNG(p4) draw = **0회**(로컬 시드 RNG만 사용) — mode2와 근본적으로 다르다.
    if kind != 2 {
        let ta = rd_u64(sim + 0xeb28).unwrap_or(0);      // vt+0x20
        let tt = rd_u64(sim + 0xeb00).unwrap_or(0);      // vt+0x28 (tick)
        // L = u64[ u64[ u64[p6+8] + 8 ] + 0x12f8 ]  (Rules 스칼라)
        let l_host = rd_u64(rd_u64(rd_u64(p6 + 8).unwrap_or(0) as usize + 8).unwrap_or(0) as usize + 0x12f8).unwrap_or(0);
        // ── 조기 빈-Vec 게이트 (`@1421266c9`~`@1421267af`) ──
        if rd_u8(p5 + 0x414) == 1 {
            let b = rd_u64(p5 + 0x818).unwrap_or(0);
            let den = (10u64.wrapping_mul(l_host)).max(1);
            let (q, rem) = (tt / den, tt % den);
            let seed = ta ^ (b << 4) ^ (q << 40) ^ 0x1A75E;
            let mut r = LocalRng::seed_from_u64(seed);
            let r1 = r.gen_range_i64(3i64.wrapping_mul(l_host as i64), 6i64.wrapping_mul(l_host as i64)) as u64;
            let r2 = r.gen_range_i64(3i64.wrapping_mul(l_host as i64), 6i64.wrapping_mul(l_host as i64)) as u64;
            let d3 = r.gen_range_i64(0, 9999) as u64;
            let d4 = r.gen_range_i64(0, 9999) as u64;
            let sv = {
                let base = if tt >= 210u64.wrapping_mul(l_host) { tt - 210u64.wrapping_mul(l_host) } else { 0 };
                (base.wrapping_mul(6000) / (810u64.wrapping_mul(l_host)).max(1)).min(6750)
            };
            let s5 = rd_u64(p5 + 0x208).unwrap_or(0).min(100);
            let s6 = rd_u64(p5 + 0x210).unwrap_or(0).min(100);
            let s7 = rd_u64(p5 + 0x3f0).unwrap_or(0);
            let (av, bv) = (if s5 <= 100 { 100 - s5 } else { 0 }, if s6 <= 100 { 100 - s6 } else { 0 });
            let cv = if s7 <= 1000 { 1000 - s7 } else { 0 };
            let p0 = ((av.wrapping_mul(av).wrapping_mul(sv) as u128).wrapping_mul(0x68db8bb) >> 40) as u64;   // = a²·S/10000
            let v1 = ((bv.wrapping_mul(bv).wrapping_mul(4500) as u32 as u64 as u128).wrapping_mul(0x68db8bb) >> 40) as u64;
            let v2 = (((v1 as u32 as u64).wrapping_mul(cv as u32 as u64) as u128).wrapping_mul(0x83126f) >> 33) as u64;
            let p1 = (((v2 as u32 as u64).wrapping_mul(cv as u32 as u64) as u128).wrapping_mul(0x10624dd3) >> 38) as u64;
            let lim0 = (r1.wrapping_add((sv.wrapping_mul(l_host)) / 6000)).min(7u64.wrapping_mul(l_host));
            let ret0 = (d3 as u32) < (p0 as u32) && (rem as u32) < (lim0 as u32);
            let ret1 = (d4 as u32) < (p1 as u32) && (rem as u32) < (r2 as u32);
            if ret0 || ret1 { return 0; }   // ★빈 Vec = COUNT 0 (GATE D 무조건 트립 → 게임 2)
        }
        // ── 본체 루프 (`@1421268e9`~`@142126e30`) ──
        let (l3, l6, l1) = (3u64.wrapping_mul(l_host), (6u64.wrapping_mul(l_host)).max(1), l_host.max(1));
        let g = rd_i64(p5 + 0x218).unwrap_or(0) as u64;
        let xg = if g <= 100 { 100 - g } else { 0 };
        let kc = (((xg.wrapping_mul(xg) as u128).wrapping_mul(0x2d99999a4718) >> 43) as u32 as u64).wrapping_add(3000);
        let seedbase = ta ^ rd_u64(p5 + 0x818).unwrap_or(0);
        for u in 0..5usize {
            let bldg = rd_u64(l80 + 0x1e0 + other*0x28 + u*8).unwrap_or(0) as usize;
            if bldg == 0 { continue; }
            let id = rd_u64(bldg + 0x5a8).unwrap_or(0);
            if dd7_slot48_h(&hdr, s, id) { continue; }
            let sp = rd_i64(bldg + 0x628).unwrap_or(0) as u64;
            let seen = rd_u64(p7 + 0x290 + u*8).unwrap_or(0);
            let age = if tt >= seen { tt - seen } else { 0 };
            let accept = if age <= l3 {
                // FRESH: 최근 목격 → 실좌표로 판정
                let (px, py) = (rd_u64(p7 + 0x218 + u*0x10).unwrap_or(0), rd_u64(p7 + 0x220 + u*0x10).unwrap_or(0));
                let d = isqrt_u64(sqd(tgtx, tgty, px, py));
                age.wrapping_mul(sp) >= if d >= k { d - k } else { 0 }
            } else {
                // STALE: 미목격 시간에 비례한 반경 r 원판 내 무작위 추정 위치(로컬 RNG 3 draw)
                let r_rad = age.wrapping_mul(kc) / l1 + 40000;
                if r_rad > 300000 { continue; }
                let (lx, ly) = (rd_i64(bldg + 0x648).unwrap_or(0), rd_i64(bldg + 0x650).unwrap_or(0));
                let seed = ((tt / l6) << 40) ^ ((u as u64) << 8) ^ seedbase;
                let mut r = LocalRng::seed_from_u64(seed);
                let x1 = r.gen_range_i64(-1000, 1000);
                let x2 = r.gen_range_i64(-1000, 1000);
                let rr = r.gen_range_i64(0, r_rad as i64);
                let h = { let sq = isqrt_u64((x1.wrapping_mul(x1).wrapping_add(x2.wrapping_mul(x2))) as u64); if sq == 0 { 1 } else { sq as i64 } };
                let ex = (lx.wrapping_add(x1.wrapping_mul(rr) / h)).max(0) as u64;
                let ey = (ly.wrapping_add(rr.wrapping_mul(x2) / h)).max(0) as u64;
                let d = isqrt_u64(sqd(ex, ey, tgtx, tgty));
                d <= sp.wrapping_mul(l3).wrapping_add(k).wrapping_add(r_rad)
            };
            if accept { count += 1; }
        }
        return count;
    }
    // ══ 이하 mode == 2 (DeathMatch) 전용 — 기존 재현 유지 ══
    for u in 0..5usize {
        let bldg = rd_u64(l80 + 0x1e0 + (other*5 + u)*8).unwrap_or(0) as usize;
        if bldg == 0 { continue; }
        // ★★[07-23] **RNG draw 순서 교정** — 원본은 `vt[0xd0]`(=dd7_slot48_h) 필터를 **draw 이전에** 통과시킨다(`@142126985`).
        //   ~~모드는 draw를 무조건 먼저 하고 필터를 accept-test에 묶어놨다~~ ⟹ 한 빌딩이라도 필터에 걸리면
        //   **이후 모든 빌딩의 roll이 밀려 RNG 스트림이 오염**된다(대체 시 desync 직결).
        let id = rd_u64(bldg+0x5a8).unwrap_or(0);
        if dd7_slot48_h(&hdr, s, id) { continue; }   // ★draw 前 필터(원본 순서)
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
        if thra > thrb {   // pre-test (thra<=thrb면 바로 accept test)
            let h = dd7_slot_a8_h(&hdr, id);
            // ★[07-23] ~~`if !dd7_slot48_h(..) { continue }`(slot48!=0이면 accept test로)~~ → **무조건 continue**.
            //   원본(`@142126c3c`)은 h==0일 때 `vt[0xd0]`를 **재호출**하는데, 위 draw-前 필터에서 이미 false로 확정됐으므로
            //   그 값은 **항상 false ⇒ 무조건 reject**. 구 코드는 이 케이스를 accept로 뒤집고 있었다.
            if h == 0 { continue; }
            // ★[07-23] geo stride·레인 인덱스 오프셋 마이그 누락 수정:
            //   ~~`other*0x228`(0.4.x)~~ → **`other*0x2e8`**(원본 `@142126873 imul r8,r8,0x2e8`)
            //   ~~`rd_i32(h+0x738)`(0.4.x)~~ → **`rd_u32(h+0x8b0)`**(원본 `@142126ac4 mov eax,[rax+0x8b0]`)
            let lane = rd_u64(other*0x2e8 + geo + 0x1e0 + (rd_u32(h+0x8a0) as usize)*8).unwrap_or(0);//  ★0.5.4 오프셋 이동 반영
            if lane + 600 < s20 { continue; }   // 원본 `lane + 0x258 >= s20`이어야 통과
        }
        // accept test: lvar20*(local_100>>4) < e → reject
        // ★[07-23] ~~`dd7_slot48_h(..) ||`~~ **항 제거** — 위 draw-前 필터로 이미 걸러졌다(중복 판정 = 원본에 없음).
        if lvar20.wrapping_mul(local_100 >> 4) < e { continue; }
        // ACCEPT: push *(L80+0x1e0+other*0x28+u*8) if !=0
        if rd_u64(l80 + 0x1e0 + other*0x28 + u*8).unwrap_or(0) != 0 { count += 1; }
    }
    count
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


// vt+off(arg) 1인자 호출(node ctx vt+0x168 등 pure getter). dd7700 def_resolve(vt+0x140)와 동일 패턴.
#[inline] unsafe fn vt_call1(vt: usize, off: usize, a: usize) -> usize {
    let f = vt_slot(vt, off);
    if !ptr_ok(f) { return 0; }
    let g: VtPtrFn = core::mem::transmute(f);
    g(a)
}
// vt+off(this, arg) 2인자 호출(win64 rcx=this, rdx=arg). 읽기전용 리졸버(SerpenBattle self/target resolve vt0x138/0x150) 전용.
#[inline] unsafe fn vt_call2(vt: usize, off: usize, this: usize, arg: usize) -> usize {
    let f = vt_slot(vt, off);
    if !ptr_ok(f) { return 0; }
    let g: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(f);
    g(this, arg)
}

unsafe fn mem_eq(a: usize, b: usize, len: usize) -> bool {
    if len == 0 { return true; }
    if len > 4096 { return false; }
    // ★readable() VirtualQuery 2회 제거(engage e88a0 3중루프 1.78ms/call의 주범 — fast_read 무관했던 숨은비용): lockless lr_u8 비교, fault=불일치(readable-false와 동의미·비트동일·미스매치 조기탈출)
    for i in 0..len { match (lr_u8(a+i), lr_u8(b+i)) { (Some(x), Some(y)) if x == y => {}, _ => return false } }
    true
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
// ════ EpicPoke(disc9) FUN_141b21440 재현. 출력 {19,11,12,13,7,3,2}. df0c10 flag·fdae40만 stub. ════
// ════ EpicPoke(disc9) 0.5.0 FUN_1422e0d80 완전재작성(REWRITE, §12.13). out-struct writer, RNG-free. ════
//   sub=subplan+8(p2) · P3=phase(r8) · P5=SimState(side@+0x820) · P6=roster holder([P6]=l80,[P6+8]=vobj).
//   phase 게이트 통과시 적팀 리스트(FUN_1421797a0)에 대해 동적 reach² 근접 스캔 → contested. active 게이트 계산.
struct EpicOut { f0: u64, f8: u64, f10: u64, f18: u64, active: u8, gateflag: u8, contested: u8, clane: u8 }

// FUN_1421797a0 재현: side 6노드슬롯(0!=만) + 챔피언Vec 전량 → 엔티티 buf. 반환=개수(count 시맨틱 확정).
unsafe fn poke_enemy_list(l80: usize, side: usize, buf: &mut [usize]) -> usize {
    if !ptr_ok(l80) || side > 1 { return 0; }
    let mut n = 0usize;
    for &off in [0x180usize, 0x1a0, 0x1c0, 0x190, 0x1b0, 0x1d0].iter() {
        if n >= buf.len() { return n; }
        let e = rd_u64(l80 + off + side * 8).unwrap_or(0) as usize;
        if e != 0 { buf[n] = e; n += 1; }               // 노드=0 스킵
    }
    let vptr = rd_u64(l80 + 0x130 + side * 0x20).unwrap_or(0) as usize;   // begin
    let vlen = rd_u64(l80 + 0x148 + side * 0x20).unwrap_or(0) as usize;   // count(원소 수)
    if ptr_ok(vptr) && vlen <= 256 {
        for i in 0..vlen {
            if n >= buf.len() { return n; }
            buf[n] = rd_u64(vptr + i * 8).unwrap_or(0) as usize;   // 챔피언=전량 push(0 스킵 안 함)
            n += 1;
        }
    }
    n
}

// ════ disc9(실명 Battle — 구라벨 EpicPoke, §11.8) 0.5.0 재작성: phase 게이트 + contested 스캔 + active 게이트. None=가드실패(passthrough).
unsafe fn epic_poke_compute(sub: usize, p3: u64, p5: usize, p6: usize) -> Option<EpicOut> {
    for i in 0..12 { EGT[i].store(-1, Ordering::Relaxed); }   // ★진단 리셋(진입 직후)
    let l80 = rd_u64(p6).unwrap_or(0) as usize;             // ss = *P6
    if !ptr_ok(l80) { return None; }
    let _phase_gate = tune("poke_phase_gate", 0x31) as u64;  // ⚠[07-23] **死레버**: 0.5.2 원본에 대응 p3 게이트가 없어 무효(제거/숨김 대상)
    let _active_min = tune("poke_active_min", 0xb) as u64;   // ⚠[07-23] **死레버**: 위와 동일(0.5.2 게이트 부재)
    let reach_bonus = tune("poke_reach_bonus", 120000);      // ✅[재배선] 근접 도달거리 보너스(좌표)
    // ★★[07-23] **0.5.2 오프셋 이동 반영**(pokecmp `[★DIFF@+0x29]` 2814건의 직접 원인 — out+0x29 = gateflag raw).
    //   0.5.0 `sub+0x90/0x94/0x95`(raw subplan 기준) → **0.5.2 `sub+0xBE/0xC4/0xC5`**(sub=subplan+8 기준).
    //   시프트가 +0x36/+0x38로 **불균일** = 단순 삽입이 아니라 구조체 내부 재배치. ⬜0.5.1에서 바뀐 건지 0.5.2인지는 미확정.
    let bvar1 = rd_u8(sub + 0xBE);                          // gateflag  ★0x88→0xBE
    let disc = rd_u64(sub + 0x58).unwrap_or(0);
    let clane = rd_u8(sub + 0xC4);                          // i8, ==1이면 active 무효  ★0x8c→0xC4(07-23)
    egt(1, p3 as i64); egt(2, bvar1 as i64); egt(3, disc as i64); egt(4, clane as i64);   // ★진단
    let mut contested = 0u8;
    // phase 게이트: P3>0x31 && (bVar1&1) && (0x6f>>(disc&0x3f))&1  (disc ∈ {0,1,2,3,5,6})
    // ★★[07-23] **p3(phase) 게이트 삭제 + 전바이트 비교**: 0.5.2 원본 `0x22BA290`은 함수 전체에서 **r8(param_3)을 단 한 번도 읽지 않는다**
    //   (r8 등장 = `mov r8d,1`/`xor r8d,r8d`/`mov r8,[rdi+0x60]` 뿐). 0.5.0의 `p3>0x31`·`p3<0xb` 게이트가 **전량 삭제**됐다.
    //   ⟹ disc1(dd7700)·disc12·disc14와 **같은 부류**(0.5.2가 phase/level 류 사전 게이트를 걷어냄). 인접 arm `0x213448B`도 p3 없음으로 교차확증.
    //   ★`(bvar1 & 1) != 0` → **`bvar1 != 0`**(원본 `test bpl,bpl` = 전바이트).
    //   ⚠부수: `tune("poke_phase_gate")`·`tune("poke_active_min")`는 0.5.2에 대응 게이트가 없어 **死레버**가 된다(설정해도 무효).
    let pg_pass = bvar1 != 0 && ((0x6fu64 >> (disc & 0x3f)) & 1) != 0;
    egt(10, pg_pass as i64);   // ★진단
    if pg_pass {
        let sim = rd_u64(l80).unwrap_or(0) as usize;        // *ss
        let vt = rd_u64(l80 + 8).unwrap_or(0) as usize;     // ss.vt
        let handle = rd_u64(sub + 0x60).unwrap_or(0);
        let cand = if ptr_ok(sim) && ptr_ok(vt) { geom_resolve150(sim, handle) } else { 0 };   // vt+0x150 2인자 → 순수재현(shadow-call 제거=AV방지)
        egt(5, ptr_ok(cand) as i64);                        // ★진단
        let lane = rd_u8(sub + 0xC5);                       // i8 lane; 0xff(-1)이면 스캔 스킵  ★0x8d→0xC5(07-23)
        egt(6, lane as i64);                                // ★진단
        if ptr_ok(cand) && lane != 0xff {
            let cx = rd_u64(cand + 0x648).unwrap_or(0);
            let cy = rd_u64(cand + 0x650).unwrap_or(0);
            let enemy_side = 1u64.wrapping_sub(rd_u64(p5 + 0x810).unwrap_or(0)) as usize & 1;//  ★0.5.4 오프셋 이동 반영
            egt(7, enemy_side as i64);                       // ★진단
            let mut ebuf = [0usize; 40];
            let n = poke_enemy_list(l80, enemy_side, &mut ebuf);
            egt(8, n as i64);                                // ★진단
            for &e in ebuf.iter().take(n) {
                if !ptr_ok(e) || rd_i32(e + 0x68).unwrap_or(0) != 2 { continue; }   // type==2 챔피언만
                let elane = rd_u8(e + 0x128);
                let m = if lane.wrapping_sub(3) < 2 {          // lane ∈ {3,4}
                    elane == lane || elane.wrapping_sub(3) < 2
                } else { elane == lane };
                if !m { continue; }
                let reach2: u64 = if rd_i32(e + 0x4a8).unwrap_or(0) != -1 {   // alive
                    let r = rd_u64(e + 0x420).unwrap_or(0) as i64
                        + rd_u64(e + 0x488).unwrap_or(0) as i64
                        + (rd_u64(e + 0x5b0).unwrap_or(0) as i64 - 1) * (rd_u64(e + 0x490).unwrap_or(0) as i64)
                        + reach_bonus;
                    r.wrapping_mul(r) as u64
                } else { 14400000000u64 };                   // 120000² 기본
                let ex = rd_u64(e + 0x648).unwrap_or(0);
                let ey = rd_u64(e + 0x650).unwrap_or(0);
                let dx = if cx >= ex { cx - ex } else { ex - cx };
                let dy = if cy >= ey { cy - ey } else { ey - cy };
                if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) <= reach2 { contested = 1; egt(11, 1); break; }   // ★진단
            }
        }
    }
    // active 게이트
    let mut active;
    // ★[07-23] 0.5.2 최종식: `active = (gateflag==0) ? 0 : (clane==1) ? 0 : contested^1` — **p3 무관**(게이트 삭제 확증).
    active = contested ^ 1;
    if bvar1 == 0 { active = 0; }
    if clane == 1 { active = 0; }
    // ★[07-23] ~~`if p3 < active_min { active = 0 }`~~ **삭제**(0.5.2 원본에 대응 게이트 없음 — r8 미사용 확증).
    egt(0, active as i64); egt(9, contested as i64);   // ★진단 최종
    Some(EpicOut {
        f0: rd_u64(sub).unwrap_or(0),
        f8: rd_u64(sub + 8).unwrap_or(0),
        f10: disc,
        f18: rd_u64(sub + 0x60).unwrap_or(0),
        active, gateflag: bvar1, contested, clane,
    })
}

// out-struct(0x2d) 기록. false=가드실패(passthrough).
unsafe fn epic_poke_write(p1: usize, sub: usize, p3: u64, p5: usize, p6: usize) -> bool {
    if !writable(p1, 0x30) { return false; }
    match epic_poke_compute(sub, p3, p5, p6) {
        Some(o) => {
            std::ptr::write_unaligned(p1 as *mut u64, o.f0);
            std::ptr::write_unaligned((p1 + 0x08) as *mut u64, o.f8);
            std::ptr::write_unaligned((p1 + 0x10) as *mut u64, o.f10);
            std::ptr::write_unaligned((p1 + 0x18) as *mut u64, o.f18);
            std::ptr::write_unaligned((p1 + 0x20) as *mut u64, 0u64);
            std::ptr::write_unaligned((p1 + 0x28) as *mut u8, o.active);
            std::ptr::write_unaligned((p1 + 0x29) as *mut u8, o.gateflag);
            std::ptr::write_unaligned((p1 + 0x2a) as *mut u8, o.contested);
            std::ptr::write_unaligned((p1 + 0x2b) as *mut u8, 0u8);
            std::ptr::write_unaligned((p1 + 0x2c) as *mut u8, o.clane);
            true
        }
        None => false,
    }
}


// ★dd7700 action code 재현 (현 단계: 상단가드→7, 레인크립→4/7, 그외→2(기본/tail 미완)).
// 반환 -999 = 미예측(가드 실패 등). p3=param_3(r8).
// ⬜[07-23] `_p3`(param_3) = 0.5.2 원본에선 **판단 게이트로 쓰이지 않고**(게이트 2개 삭제 확인) f22e80 카운트 호출 `FUN_142126610`의 인자로만 전달된다.
//   모드의 `my_f22e80_count` 호출부는 현재 p3를 넘기지 않는다 → **원본과 인자 불일치 가능성(미확증)**. tail(STAGE2~6) 잔여 DIFF의 후보.
// ══════════════════════════════════════════════════════════════════════════════════════════════
// ★★[07-23] **disc0/1/3 대체 emit 구현 명세** (0.5.2 원본 `0x1b91e40` 전수 disasm 확정, ghidra-re)
//   ⬜미구현 — 편입하려면 이 함수를 **code 반환 → (code, Aux) 반환 or out-writer**로 바꿔야 한다(return 20+곳 종단 분류 필요).
//   ★실익: cfg의 `dd_frontier_mult=350` `dd_lane_margin=600` `dd_cover_count=0` `dd_ratio_thr=31` `dd_near_dist=110250000`이
//     기본값과 달라 **유저 튜닝이 들어가 있는데, emit이 없어 전혀 반영되지 않는 상태**다(편입 시 즉시 AI 판단이 바뀜).
//   ⚠`dd_early_p3_thr`·`dd_cover_p3_thr`는 0.5.2에서 대응 게이트 삭제 ⟹ **死레버**(설정편집기에서 숨김/제거 대상).
//
//   ── 종단별 write-set (f = u8[p2+0x116], lane == f 항등 증명됨) ──
//     T_G1     (code 7) : qword[+0]=7                                   ← +8 안 씀
//     T_G2_6   (code 6) : qword[+0]=6, byte[+8]=f
//     T_G2_4   (code 4) : qword[+0]=4, byte[+8]=f
//     T_COVER  (code 4|7): qword[+0]=code, **byte[+8]=2**               ← ★code 7인데 +8을 쓴다(f==2 보장 경로)
//     T_G7_6   (code 6) : qword[+0]=6, byte[+8]=lane(=f)
//     T_G7_7   (code 7) : qword[+0]=7                                   ← +8 안 씀
//     T_G8_7   (code 7) : qword[+0]=7                                   ← +8 안 씀
//     T_MAIN2  (code 2) : qword[+0]=2, byte[+8]=(i32[p5+0x8b0]==1) as u8, byte[+9]=f, byte[+0xa]=bl
//       LR = nav + team*0x2e8 + (lane==0 ? 0 : lane==2 ? 0x50 : 0x28),  nav = param_6[2], team = qword[p5+0x820]
//       ★★[07-23 정정, ghidra-re 전수 disasm] ~~"bl 4갈래: 빔→2 / 루프히트→(i64[LR+0x18]<0?2:0) / sf경로→1 / fallback→(i64[LR+0x10]<0x7d1)"~~
//         = **오기**. 뒤 2갈래(sf→1 / fallback)는 T_MAIN2가 아니라 **SF 경로(`0x1b9219e~`) 전용**이다.
//         진입 게이트 `0x1b9216e`: `plan(u8[p7+0x3f6]) == 8` **정확히 8**(&0xfe 아님 — 그 마스크는 COVER 게이트 `0x1b91ec9`에만) **AND** `sf(u8[p7+0x3f7]) == f`(`0x1b92191`).
//         ⟹ **MAIN BODY 진입 = `plan != 8 || sf != f`**. 두 경로는 **상호배타**라 같은 실행에서 함께 평가되지 않는다.
//       ⟹ ★**MAIN BODY의 bl은 정확히 2갈래**(현행 재현이 정답 — 수정 불요):
//         · HIT(`0x1b92509`)          → `(i64[LR+0x18] < 0) ? 2 : 0`   (`0x1b92583` SHR 0x3f / `0x1b9258b` ADD BL,BL)
//         · 빈리스트 or 루프 미히트   → `2`  (`0x1b92546` **단일 사이트** — `TEST RDI,RDI;JZ`(`0x1b923f6`)와 루프종료 `JZ`(`0x1b92437`)가 동일 타겟 = 별도 분기 아님)
//       ⚠원본에 bl **디폴트 대입은 없다**(4개 사이트 배타 대입, 모든 경로가 반드시 하나를 통과). 모드의 `local_58 = 2` 초기화는
//         MAIN BODY 한정으로 "미히트=2"와 값이 같아 **무해**하나, "원본도 디폴트 2"라고 적으면 안 된다.
//       ── 교차확인 완료(전부 모드와 일치, 재조사 금지) ──
//         · 근접 임계 `0x1b92482 SHR 8` + `0x1b92486 CMP 0x53d1ac0; JA` ⟹ 통과 `sqd8 <= 0x53d1ac0` ≡ 모드 `< 0x53d1ac1` ✔
//         · lane→오프셋 `f==0?0 : f==1?0x28 : 0x50`(`0x1b9253a`) ✔ / `thr` 조회 = `nav + (1-team)*0x2e8 + 0x1e0 + rlane*8`(=모드 `(rlane+0x3c)*8`) ✔
//         · 통과조건 `thr + 0x78 >= vt[+0x28](ctx)` ≡ 모드 `s20 <= thr + lane_margin`(리터럴 0x78 = `dd_lane_margin` 기본값) ✔
//         · 팀 인덱스 비대칭(후보/thr = **상대팀** `1-team` / LR = **자기팀** `team`) — 모드가 이미 정확히 구분 중 ✔
//       ⬜미채택 확장: 모드는 `plan == 8`이면 무조건 passthrough(L3452)라 **`plan==8 && sf!=f`(원본은 MAIN BODY)** 를 놓친다.
//         passthrough=바닐라 비트동일이라 **안전**하고 커버리지만 손해. 넓히려면 게이트를 `plan==8 && u8[p7+0x3f7]==f`로 바꾸면 되나,
//         이번 편입의 blast radius를 키우지 않으려 **보류**(별건).
//   ── ★구현 주의 3가지 ──
//     ①`+8`은 **반드시 byte 스토어**. qword로 쓰면 `+9..+0xf` 잔재를 0으로 덮어 원본과 갈린다
//       (실측 `code=7 +8=0x8`이 잔재 보존의 직접 증거 — 그 0x8은 콜러 스택 잔재다).
//     ②code 7 3종단(T_G1/T_G7_7/T_G8_7)은 `+8`을 **쓰면 안 된다**. "code 7=aux 없음"으로 뭉뚱그리면 T_COVER가 깨진다.
//     ③`+0x0b..+0x2f`는 **손대지 않는다**(원본도 잔재를 그대로 커밋 = 병합기가 code 2/4/6/7에서 0x30 통복사).
//       ⬜대안(크래시 방어 우선): `+0x10..+0x2f` 0-fill 시 무경계 JT가 index 0으로 in-bounds 보장되나 **바닐라와 비트동일은 깨짐**.
//       어느 쪽이 옳은지는 "소비자가 code 2/4/6/7에서 그 JT를 읽는지"에 달렸고 **미규명** → 편입 전 확인 권장.
//   ── 부수 확정 ── write-set은 **disc0/1/3 완전 동일**(dd7700은 `*p2`=discriminant를 읽지 않음, 디스패처 idx 선택에만 사용).
// ══════════════════════════════════════════════════════════════════════════════════════════════
unsafe fn my_dd7700_code(p2: usize, _p3: u64, p4: usize, p5: usize, p6: usize, p7: usize, skip_cover: bool) -> i64 {
    if rd_u8(p2+0x110) != 0 { DD7_PATH.store(1, Ordering::Relaxed); return 7; }                 // 상단 가드(0.5.0: p2+0x18→0x110)
    // ★신규 조기분기(0.5.0 신설, plan7 가드 직후): 0x2d<p3 && *(p2+0x112)!=0 && *(p2+0x115)==0 → code 6/4 (aux write=*(param1+1)=*(p2+0x116)는 full 소관)
    // ★★[07-23] **disc1 오답 328/400의 주범 수정**: ~~`(tune("dd_early_p3_thr",0x2d) as u64) < p3 &&`~~ **게이트 삭제**.
    //   0.5.2 원본(`0x1b91e40`)에 **`param_3`(p3/r8)을 비교하는 게이트가 하나도 없다**(0.5.0_3·0.5.1엔 `cmp r8,0x2e; jb` 존재).
    //   실측 p3 ≤ 0x2d라 이 게이트가 조기분기를 **항상 차단** → code 4·6이 전량 소실되고 tail로 새서 2만 반환했다
    //   (`my=2 path=0 → game=4` 264건 / `→ game=6` 64건 = 조기분기 산출물 그대로).
    //   disc0이 400/400이었던 이유도 설명됨: disc0에선 `0x112==1 && 0x115==0`이 성립 안 해 조기분기 자체가 발화하지 않았다.
    //   ★비교 강화도 함께: ~~`rd_u8(p2+0x112) != 0`~~ → **`== 1`**(0.5.2에서 `cmp ...,1`로 엄격해짐. 0.5.0_3·0.5.1은 `!=0`).
    //   ⟹ disc14의 `cmp 0x21`·`level>0x2d` 게이트 삭제와 **정확히 같은 부류**(0.5.2가 p3/level 류 사전 게이트를 걷어냄).
    if rd_u8(p2+0x112) == 1 && rd_u8(p2+0x115) == 0 {
        DD7_PATH.store(2, Ordering::Relaxed);
        return if rd_u8(p2+0x113) != 0 { 6 } else { 4 };
    }
    DD7_PATH.store(0, Ordering::Relaxed);
    let l80 = rd_u64(p6).unwrap_or(0) as usize;
    let vobj = rd_u64(p6+8).unwrap_or(0) as usize;
    let geo = rd_u64(p6+0x10).unwrap_or(0) as usize;
    if !ptr_ok(l80) || !ptr_ok(vobj) || !ptr_ok(geo) { return -999; }
    let sim = rd_u64(l80).unwrap_or(0) as usize;
    if !ptr_ok(sim) { return -999; }
    let plan = rd_u8(p7+0x3f6);                            // 0.5.0: p7+0x3e6→0x3ea
    let side = rd_i64(p5+0x810).unwrap_or(-1);            // 0.5.0: p5+0x6a8→0x820  ★0.5.4 오프셋 이동 반영
    let lane = rd_i32(p5+0x8a0).unwrap_or(-1);            // 0.5.0: p5+0x738→0x8b0  ★0.5.4 오프셋 이동 반영
    // ★[07-23] ~~`(tune("dd_cover_p3_thr",4) as u64) < p3 &&`~~ **게이트 삭제**(0.5.2 원본에 `cmp r8,4; jbe` 없음 — 위 조기분기와 동일 사유).
    //   이 게이트가 커버 블록(code 4/7)을 전량 차단하고 있었다.
    if !skip_cover {   // ★B cover dedup: full의 engage경로(skip_cover=true)는 full이 이미 cover 비fire 확인(동일 게이트) → code의 cover 재스캔 생략=비트동일
        let cvar10: i64 = rd_u8(p2+0x116) as i64;         // 0.5.0: p2+0x19→0x116
        // (plan&0xfe)==8 이고 *(p7+0x3eb)==cvar10 이면 LAB(기본)로
        let to_default = (plan & 0xfe) == 8 && (rd_u8(p7+0x3f7) as i64 == cvar10);  // 0.5.0: p7+0x3e7→0x3eb
        if !to_default && cvar10 == 2 {
            let s20 = dd7_slot20(sim);                        // ★호이스트: 현재틱(sim+0xed00)=호출 내 불변
            let lane_margin = tune("dd_lane_margin", 0x78);   // ★호이스트: 루프불변 튜닝계수
            let hdr = sim_hdr(sim);                           // ★호이스트: sim 헤더 1회 → cover 루프 slot48/a8 재사용
            // ★프론티어 게이트(07-10 0.5.0_3 재RE 정정): v=*(u8)(vobj+0x38), 발화 iff v∈{0,5,7,8}(비트마스크 0x1a1).
            //   구모델 "+0x28 ∉{1,2}"는 오식별(사실상 죽은코드)→게임만 bail(2)하고 우리는 커버(4)내던 my=4≠g=2 원인.
            // ★★[07-23 진범 수정] ~~`if prog <= s20 { return 2; }`~~ → **불리언 플래그 + 커버 블록만 스킵**.
            //   0.5.2 원본 `@1b91f7c`의 `jae 0x1b9216e`가 가리키는 **`0x1b9216e`는 MAIN BODY 진입점이지 반환 지점이 아니다**.
            //   함께 세팅되는 `r12b=2`도 반환코드가 아니라 MAIN에서 소비되는 변수(`@1b92175`)다.
            //   ⟹ 프론티어 bail = **"커버 블록만 포기하고 MAIN BODY로 폴스루"** — 최종 code는 MAIN이 정한다(2/6/7).
            //   실제로 **커버 블록의 모든 실패 경로가 MAIN으로 폴스루**한다(count 미달 `@1b92153`, `lane<3` `@1b91f8e` 동일).
            //   ⟹ 이것이 disc0 잔여 6/400(`my=2 game=7 path=3`)의 원인. 게이트 조건·오프셋·`30*l15`·`prog`·`s20`은 **전부 정확**했다
            //     (`v1 = u64[vobj+8]` 1단 역참조 확정 — serpen/f22e80과 달리 여기선 2단 아님).
            //   ★`my_dd7700_full`은 처음부터 불리언 플래그로 올바르게 구현돼 있었다(수정 불요) — `code`만 틀렸다.
            let vb = rd_u8(vobj+0x38);
            let frontier_bail = if vb <= 8 && (0x1a1u32 >> vb) & 1 == 1 {
                let v1 = rd_u64(vobj+8).unwrap_or(0) as usize;
                let u19 = rd_u64(v1+0x8a8).unwrap_or(0);
                let l15 = rd_i64(v1+0x12f8).unwrap_or(0);
                let l15x30 = (((l15 as u64) as u128).wrapping_mul(tune("dd_frontier_mult", 0x1e) as u128) * 100 / (AGGR_LANE.load(Ordering::Relaxed).max(1) as u128)) as u64;   // ★튜닝×[3]공격성배율: 프론티어 진척 배수(aggr↑=l15x30↓=prog↑=덜 후퇴)
                let prog = if l15x30 <= u19 { u19 - l15x30 } else { 0 };
                prog <= s20 as u64
            } else { false };
            if frontier_bail { DD7_PATH.store(3, Ordering::Relaxed); }   // ★진단 태그만(반환 아님 — 아래 커버 블록만 스킵하고 tail로 폴스루)
            if !frontier_bail && (lane as i64) >= tune("dd_cover_role_min", 3) && (side == 0 || side == 1) {
                let s = side as usize;
                let oidx = s*5 + (if lane==3 {1} else {0}) + 0x3f;
                let obj = rd_u64(l80 + oidx*8).unwrap_or(0) as usize;
                // ★0.5.0 홈박스 동적화(하드코딩 0/64000/896000/960000 삭제): base=[vobj+0x20], hb=base+0x6d70+side*0x20 → [hb]=x0/[+8]=y0/[+0x10]=x1/[+0x18]=y1
                let hb_base = rd_u64(vobj+0x20).unwrap_or(0) as usize;
                let hb = hb_base + 0x6d70 + s*0x20;
                let (xlo,ylo,xhi,yhi): (u64,u64,u64,u64) = (rd_u64(hb).unwrap_or(0), rd_u64(hb+8).unwrap_or(0), rd_u64(hb+0x10).unwrap_or(0), rd_u64(hb+0x18).unwrap_or(0));
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
                        let geo_side = geo + lv21*0x2e8;   // 0.5.0: geom stride 0x228→0x2e8
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
                                        let rlane = rd_i32(resolved+0x8a0).unwrap_or(0) as usize;  // 0.5.0: resolved+0x738→0x8b0  ★0.5.4 오프셋 이동 반영
                                        let thr = rd_i64(geo_side + (rlane+0x3c)*8).unwrap_or(0);
                                        q = if s20 <= thr + lane_margin { 1 } else { 0 };   // (s20/lane_margin = 호이스트됨)
                                    }
                                }
                            }
                            count += q;
                        }
                        if count >= tune("dd_cover_count", 2) as u64 {   // ★튜닝: 커버 발화 적군 카운트
                            let team_units = rd_u64(p5+0x818).unwrap_or(0);  // 0.5.0: p5+0x6a0→0x818
                            let team = dd7_slot128(sim, team_units);
                            if team != 0 {
                                let tcnt = rd_u64(team+0x610).unwrap_or(0);
                                if tcnt != 0 {
                                    let depth = rd_i64(team+0x658).unwrap_or(0);
                                    let ratio = (depth as u64).wrapping_mul(100) / tcnt;
                                    let mut code = 4i64;
                                    if (ratio as i64) < tune("dd_ratio_thr", 0x33) {   // ★튜닝: 커버 비율 임계(<51)
                                        let f = rd_i64(geo + 0x60 + s*0x2e8).unwrap_or(0);  // 0.5.0: geom stride 0x228→0x2e8
                                        code = if f > tune("dd_facet_thr", 999) { 7 } else { 4 };   // ★튜닝: 페이즈 게이트 임계(>999)
                                    }
                                    DD7_PATH.store(4, Ordering::Relaxed);
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
    // ★DD7_TAIL_OK=true(2026-07-10 검증): STAGE6 resolver/vt168 this=sim 수정으로 AV근본원인 제거 후, 0.5.0_3 인게임 데모전투서
    //   disc0=388/400(97%) MATCH·크래시無 확인(-999는 컨텍스트-ptr 실패 가드로 정상 폴백). 잔여=disc0 my=4≠g=2 12건(커버블록 임계 미세차).
    if !DD7_TAIL_OK { return -999; }
    if side != 0 && side != 1 { return 2; }
    let s = side as usize;
    let f = rd_u8(p2+0x116) as usize;                     // 0.5.0: p2+0x19→0x116
    let roleoff = if f==0 {0usize} else if f==1 {0x28} else {0x50};
    // STAGE 1: 레인활성 게이트. *(i32)(side*0x2e8 + GEO + roleoff) != 1 → 2
    let rolerec = s*0x2e8 + geo + roleoff;                // 0.5.0: geom stride 0x228→0x2e8
    if rd_i32(rolerec).unwrap_or(0) != 1 { DD7_TERM.store(40, Ordering::Relaxed); return 2; }   // ★readable VQ제거(rd_i32 None=0=fault흡수)
    // STAGE 2: 타깃 해석 = slot140(sim, *(rolerec+8)). 0이면 2. self=slot128(sim, *(p5+0x6a0)).
    let vtab = rd_u64(l80+8).unwrap_or(0) as usize;
    // ★★[07-30 0.5.3] **vt 슬롯 재시프트**: ~~0x1b8(0.5.1/0.5.2)~~ → **0x1c8**. ~~0x150(0.5.0_3)~~ → 0x1b8 → 0x1c8 로 두 번 이동했다.
    //   근거(ghidra-re 실측): 0.5.3에서 이 vtable에 **`+0x1b8`·`+0x1c0` 두 메서드가 신설 삽입**되어 **`≥0x1b8` 슬롯이 전부 +0x10 시프트**했다
    //     (`<0x1b8`인 0x28/0x118/0x138/0x140/0x150/0x168/0x1a0 등은 불변). 객체 크기도 `0xee88 → 0xeec8`.
    //     0.5.2 `vt+0x1b8`(=`0x2305520`, 핸들→엔티티 리졸버 61B)과 0.5.3 `vt+0x1c8`(=`0xee7d00`)이 **바이트 완전동일** ⟹ 이 슬롯이 후계.
    //   ⚠**이번 크래시의 진범**: 0.5.3 `vt+0x1b8`은 **sret 7인자 격자질의**(`0x12b9480`)로 바뀌었는데, 그것도 유효 코드포인터라
    //     `ptr_ok`를 통과해 그대로 shadow-call됐다 ⟹ `rdx`(=핸들 정수)가 `&self`로 오용되어 `fn+0x17b`에서 AV.
    //     실측 폴트 `exe+0x12b95fb`가 그 지점과 정확히 일치(재현 2/2). 07-23 주석이 경고한 "엉뚱한 함수 shadow-call" 사고가 그대로 재발한 것.
    //   ⚠**mpcap 전용 문제가 아니다**: disc0/1/3 재현은 대체 여부와 무관하게 매 판단마다 실행되므로 라이브 경로도 같은 지뢰를 밟을 수 있다
    //     (mpcap=1이 화이트리스트 밖 disc까지 돌려 노출을 키웠을 뿐). ⟹ 프로덕션 안전을 위해 필수 수정.
    let resolver = if ptr_ok(vtab) { rd_u64(vtab+0x1c8).unwrap_or(0) as usize } else { 0 };  // ★0.5.3 **0x1c8**(0.5.1/0.5.2=0x1b8, 0.5.0_3=0x150)
    if !ptr_ok(resolver) { DD7_TERM.store(41, Ordering::Relaxed); return 2; }
    let tgt_handle = rd_u64(rolerec+8).unwrap_or(0);
    let rf: G2 = core::mem::transmute(resolver);
    let target = if PERF_ON.load(Ordering::Relaxed) {
        let _t = Instant::now(); let r = rf(sim, tgt_handle as usize) as usize;
        DD7_RESOLVE_NS.fetch_add(_t.elapsed().as_nanos() as u64, Ordering::Relaxed); DD7_RESOLVE_N.fetch_add(1, Ordering::Relaxed); r
    } else { rf(sim, tgt_handle as usize) as usize };
    if !ptr_ok(target) { DD7_TERM.store(42, Ordering::Relaxed); return 2; }   // ★readable VQ제거(아래 tx/ty rd_u64 fault-safe)
    let selfobj = dd7_slot128(sim, rd_u64(p5+0x818).unwrap_or(0));  // 0.5.0: p5+0x6a0→0x818
    if !ptr_ok(selfobj) { DD7_TERM.store(43, Ordering::Relaxed); return 2; }   // ★readable VQ제거(panic가드=ptr_ok, 좌표 rd_u64 fault-safe)
    let (tx, ty) = (rd_u64(target+0x648).unwrap_or(0), rd_u64(target+0x650).unwrap_or(0));
    let (selfx, selfy) = (rd_u64(selfobj+0x648).unwrap_or(0), rd_u64(selfobj+0x650).unwrap_or(0));
    // STAGE 3: window(WLO/WHI) + COUNT = my_f22e80_count (RngSim는 entry state=STAGE1/2 RNG무소비라 일치).
    let count_survivors: u64 = {
        let a400 = rd_i64(p5+0x400).unwrap_or(0); let a218 = rd_i64(p5+0x218).unwrap_or(0);  // 0.5.0: C계수 p5+0x380→0x400, p5+0x218 불변
        // ★dd7700 정확식(0x1418aeea3): uVar20=(u64)(a400*a218)/1000(풀정밀 unsigned div). pre-shift(>>3)*magic 패턴 폐기(트리플floor로 t -1 오차→윈도우 widen→rejection어긋남).
        // ★★[07-23] **2단계 곱/나눗셈 누락 복원**: 원본은 `t0 = (a400*a218)/1000` 후 **`t = (t0 * *(p5+0x3f8)) / 1000`** 이다
        //   (`@14212667c..69c`). 이 항이 빠져 `a3f8 < 1000`이면 t가 과대 → **윈도우가 좁아져 roll 분포가 통째로 달라진다** → count 직결.
        //   ⚠0.5.2 신설이 아니라 **0.5.0_3에도 있던 것**(`0x141fd27e8 imul rax,[rbp+0x3f8]`) = 장기 재현 누락. clamp(100)는 **마지막에만**.
        let a3f8 = rd_i64(p5+0x3f0).unwrap_or(0);//  ★0.5.4 오프셋 이동 반영
        let t0 = (a400.wrapping_mul(a218) as u64) / 1000;
        let t = (t0.wrapping_mul(a3f8 as u64) / 1000).min(100);
        let half = 0x384u64.wrapping_sub(t.wrapping_mul(9)) >> 1;
        let (wlo, whi) = (0x3e8u64.wrapping_sub(half), 0x3e8u64.wrapping_add(half));
        // ★[07-23] kind(GameMode) 전달 — mode!=2면 f22e80가 전혀 다른 알고리즘(로컬RNG, p4 draw 0회)을 탄다.
        //   vtab은 STAGE2에서 이미 확보됨. mode!=2 경로는 RngSim을 쓰지 않으므로 None(=RNG state 불가)이어도 산출 가능.
        let f22_kind = disc4_vt30_kind(vtab);
        match RngSim::new(p4) {
            Some(mut r) => my_f22e80_count(&mut r, l80, geo, p5, p7, sim, wlo, whi, tx, ty, tune("dd_f22e80_margin", 150000) as u64, p6, f22_kind),
            None => { let mut dummy = RngSim { buf: [0;64], idx: 0, refills: 0, input: 0, state: 0 };
                      if f22_kind != 2 { my_f22e80_count(&mut dummy, l80, geo, p5, p7, sim, wlo, whi, tx, ty, tune("dd_f22e80_margin", 150000) as u64, p6, f22_kind) } else { 0 } }
        }
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
    if !ptr_ok(nexus) { DD7_TERM.store(44, Ordering::Relaxed); return 2; }   // ★readable VQ제거(panic가드=ptr_ok)
    if anchor == 0 { anchor = nexus; }   // len!=0 && anchor==0 → f9c6d0 보정 생략(추후)
    if !ptr_ok(anchor) { DD7_TERM.store(45, Ordering::Relaxed); return 2; }   // ★readable VQ제거(anchor 유효성만, 좌표 rd_u64 fault-safe)
    let (nx, ny) = (rd_u64(nexus+0x648).unwrap_or(0), rd_u64(nexus+0x650).unwrap_or(0));
    let (ax, ay) = (rd_u64(anchor+0x648).unwrap_or(0), rd_u64(anchor+0x650).unwrap_or(0));
    // GATE C: d(nexus,target) < d(nexus,anchor) → 2
    if sqd(nx,ny,tx,ty) < sqd(nx,ny,ax,ay) { DD7_TERM.store(46, Ordering::Relaxed); return 2; }
    // GATE D: near_cnt >= COUNT(f22e80) → 2
    if near_cnt >= count_survivors { DD7_TERM.store(47, Ordering::Relaxed); return 2; }
    // GATE E: (d(anchor,target)>>8) < 0x6ba9301 → 2
    if (sqd(ax,ay,tx,ty) >> 8) < tune("dd_gatee_dist", 0x6ba9301) as u64 { DD7_TERM.store(48, Ordering::Relaxed); return 2; }   // ★튜닝: GATE E 앵커-타깃 거리²(>>8)
    // ══ STAGE 6: 교전/귀환 결정 (코드 2/6/7). COUNT=count_survivors(f22e80 재현). ══
    let plan = rd_u8(p7+0x3f6);                           // 0.5.0: p7+0x3e6→0x3ea
    // iVar2: piVar26=side*0x2e8+geo; F!=0면 reindex(+0x50 if p4==2 else +0x28); +0x20
    // ★[07-23] STAGE6 roleoff 선택자 정정: ~~`(p4 as u32)==2`~~ → **`f==2`**.
    //   0.5.2 원본은 `cmp dword[rbp+0x70],2`이고 그 슬롯이 **f(=byte[p2+0x116], mode/facet)**다(p4가 아님).
    let piadj = s*0x2e8 + geo + (if f==0 {0usize} else if f==2 {0x50} else {0x28});  // 0.5.0: geom stride 0x228→0x2e8
    let ivar2 = rd_i32(piadj + 0x20).unwrap_or(0);
    // ★★[07-23 전면 교정] STAGE6 ref-path base·오프셋이 **0.5.0_3 잔재로 전부 stale**이었다(인게임 계측 TERM=52 24건 → 0.5.2 disasm 확정).
    //   ~~`r_self = sim + 0x860`(vt+0x168 모델)~~ → **`comp = sim + 0xeaf0`**(원본 `@141b92bb0`의 `vt->0x30(sim)` RDX 반환).
    //   `vt->0x30` 본체 = **3-instruction leaf**(`lea rdx,[rcx+0xeaf0]` / `xor eax,eax`|`mov eax,1`|`mov eax,2` / `ret`)
    //   ⟹ RDX는 **무조건 `sim+0xeaf0`**, RAX는 필드 무관 **컴파일타임 상수** = GameMode enum(0=Moba/1=SingleLane/2=DeathMatch).
    //   ★교차확증: `vt0x30(gchild) RDX = gchild+0xeaf0`은 serpen.rs L877이 이미 순수재현 중이고, dd7700의 `sim`과
    //     serpen의 `gchild`는 **동일 객체**(둘 다 `geom[0][0]`). f==2 오프셋(`+0x1a8`/`+0x1a0`)이 serpen W큐와 정확히 일치.
    //   오프셋도 전부 **+0x18 어긋나 있었다**: f==0 `~~+0x1c0/+0x1b8~~ → +0x1d8/+0x1d0` · f==2 `~~+0x190/+0x188~~ → +0x1a8/+0x1a0`.
    //   ⚠kind!=0(SingleLane/DeathMatch)은 객체 크기가 0xeb08이라 `+0xeaf0` 필드가 0x18B뿐 ⟹ `+0x1a8`/`+0x1d8` 읽기는 **OOB**.
    //     원본이 `flag != 0`이면 ref-path를 건너뛰는(`@141b92bb6`) 이유가 이것. 실경기는 kind0이라 평시 미발화지만 게이트는 재현한다.
    //   판별 = `disc4_vt30_kind`(기존 순수리드 재사용, shadow-call 없음 — CLAUDE.md §3 준수).
    let vt30_kind = disc4_vt30_kind(vtab);
    DD7_DBG[10].store(vt30_kind, Ordering::Relaxed);   // ★계측: 실전 GameMode 확인(f22e80 mode!=2 경로 판정용)
    let comp = sim + 0xeb30;   // = vt->0x30(sim) RDX (순수재현)
    let moba = vt30_kind == 0;
    let (bl, route_8679): (bool, bool) = match plan {
        0 => (f==2, f==0),
        1 => (f==0, f==0),
        _ => {
            let bl = if moba && rd_u64(comp+0x1a8).unwrap_or(0) != 0 { f==2 }
                     else { f==0 && moba && rd_u64(comp+0x1d8).unwrap_or(0) != 0 };
            (bl, f==0)
        }
    };
    let term_86dd = if (ivar2 as i64) > tune("dd_ivar2_thr", 2) { 7i64 } else { 6 };   // ★튜닝: STAGE6 진척단계 임계(iVar2>2→7)
    let term_872d = {                                    // 872d: anchor type2 + target → 7 else 2
        if rd_i32(anchor+0x68).unwrap_or(0) != 2 { 2i64 }
        else if rd_i64(anchor+0x88).unwrap_or(0) == 0 { 2 } else { 7 }
    };
    // ★계측: STAGE6 진단입력 저장(DIFF시 detour가 dd0diff.txt로 덤프)
    DD7_DBG[0].store(ivar2 as i64, Ordering::Relaxed); DD7_DBG[1].store(plan as i64, Ordering::Relaxed);
    DD7_DBG[2].store(bl as i64, Ordering::Relaxed); DD7_DBG[3].store(route_8679 as i64, Ordering::Relaxed);
    DD7_DBG[4].store(term_86dd, Ordering::Relaxed); DD7_DBG[5].store(term_872d, Ordering::Relaxed);
    DD7_DBG[6].store(count_survivors as i64, Ordering::Relaxed); DD7_DBG[7].store(near_cnt as i64, Ordering::Relaxed);
    DD7_DBG[8].store(-1, Ordering::Relaxed); DD7_DBG[9].store(-1, Ordering::Relaxed);   // ref-path 진입 시 갱신
    // 라우팅: ref 결정 → 869a, 아니면 86c1
    // ★★[07-23 교정] base·오프셋 stale 수정(위 comp 주석 참조) + **이중 역참조 복원**.
    //   원본 `@141b92bc2→bc9`: `RAX = [comp+0x1d0]`(Vec 데이터 포인터) → `RDX = [RAX]`(첫 원소 = 핸들) → `resolver(sim, RDX)`.
    //   ~~모드는 `[comp+0x1b8]`을 그대로 핸들로 넘겨 **deref 1회 부족**~~ → 포인터 자체를 핸들로 오인해 `e`가 엉뚱해졌다.
    //   ★serpen.rs의 W큐 재현(`ha = [we+0x1a0]` → `wh = [ha]`)이 **이미 올바른 2단 패턴**이라 교차확증됨.
    //   kind!=0이면 해당 필드가 OOB이므로 진입 자체를 막는다(원본 `flag!=0 → LAB_141b92bf5` 동치).
    let ref_h: Option<u64> = if route_8679 {                 // f==0 (LAB @141b92ba5)
        if moba && rd_u64(comp+0x1d8).unwrap_or(0) != 0 {
            let p = rd_u64(comp+0x1d0).unwrap_or(0) as usize;
            if ptr_ok(p) { Some(rd_u64(p).unwrap_or(0)) } else { None }
        } else { None }
    } else if f != 2 {
        None                                             // f==1 → 86c1(LAB_141b92bf5)
    } else {                                             // f==2 (LAB @141b92b7f)
        if moba && rd_u64(comp+0x1a8).unwrap_or(0) != 0 {
            let p = rd_u64(comp+0x1a0).unwrap_or(0) as usize;
            if ptr_ok(p) { Some(rd_u64(p).unwrap_or(0)) } else { None }
        } else { None }
    };
    if let Some(refv) = ref_h {
        // LAB_869a: e = resolver(sim, *(*(comp+0x1d0)))
        let e = rf(sim, refv as usize) as usize;   // ★resolver(this=sim, 핸들) (디컴 confirm: 옛 rf(target,..)=핸들deref에 엔티티 넘겨 AV였음 = 크래시 근본원인)
        if e != 0 {
            if rd_u8(selfobj) != 0 {
                DD7_TERM.store(50, Ordering::Relaxed);
                return if bl { term_86dd } else { term_872d };
            } else {
                // selfobj[0]==0: bl이면 (n<2 && COUNT<=3 && *(e+n*0x18+0x38)!=0)→2 else 86dd; 아니면 872d
                if bl {
                    let n = rd_u64(selfobj+8).unwrap_or(0);
                    DD7_DBG[8].store(n as i64, Ordering::Relaxed);
                    DD7_DBG[9].store(rd_u64(e + (n as usize)*0x18 + 0x38).unwrap_or(0) as i64, Ordering::Relaxed);   // ★계측: 종단 3조건 중 마지막
                    // ★[08-03 원본 순수화] `dd_n_thr`는 **원본에 대응 조건이 없다**(그 자리는 Rust 배열 bounds 패닉 가드).
        //   ⟹ 모드가 추가한 게이트를 제거하고 원본과 동일하게 항상 통과시킨다. 재추가 시 `(n as i64) < tune("dd_n_thr",2)` 복원.
        if true && count_survivors <= tune("dd_survivor_thr", 3) as u64 && rd_u64(e + (n as usize)*0x18 + 0x38).unwrap_or(0) != 0 { DD7_TERM.store(51, Ordering::Relaxed); return 2; }   // ★튜닝: 슬롯수/생존자수 임계
                    DD7_TERM.store(52, Ordering::Relaxed); return term_86dd;
                }
                DD7_TERM.store(53, Ordering::Relaxed); return term_872d;
            }
        }
        // e==0 → 86c1로 폴
    }
    // LAB_86c1: !bl → 872d; bl이면 COUNT<=3→2 else 86dd
    if !bl { DD7_TERM.store(54, Ordering::Relaxed); return term_872d; }
    if count_survivors <= tune("dd_survivor_thr", 3) as u64 { DD7_TERM.store(55, Ordering::Relaxed); return 2; }   // ★튜닝: 생존자수 임계
    DD7_TERM.store(56, Ordering::Relaxed); term_86dd
}

// ★dd7700(0x18ae610) 충실재현 — 전체출력(code@+0 + aux +8/+9/+0xa). Some(())=재현완료(out에 write) / None=passthrough(미포팅 경로/가드).
// 포팅범위(2026-06-19): early(7) + cover(4/7,[+8]=2) + else-branch main code-2(LAB_af3d9: [+8]=local_90 [+9]=F [+0xa]=local_58).
// 미포팅(None): plan==8(epic) 분기 / iVar12==1 engage(CAND_FILTER→6/7). day-11 plan=255 else-branch code-2 dominant.
unsafe fn my_dd7700_full(out: usize, p2: usize, p3: u64, p4: usize, p5: usize, p6: usize, p7: usize) -> Option<bool> {   // ★레버: Some(true)=engage(CAND_FILTER RNG소비)/Some(false)=cover·main(RNG 0 draw)→rng_final skip/None=passthrough
    let _pg = perf_guard(1);
    if !writable(out, 0x18) { return None; }
    let _posg = pos_enter_p56(p5, p6);   // ★포지션별 cfg: dd_* 라인전 계수 포지션 응답
    // EARLY GUARD: byte[param_2+0x110]!=0 → *out=7 (0.5.0: p2+0x18→0x110)
    if rd_u8(p2 + 0x110) != 0 { std::ptr::write_unaligned(out as *mut u64, 7u64); return Some(false); }
    // ★신규 조기분기(0.5.0 신설): *(p2+0x112)==1 && *(p2+0x115)==0 → out code 6/4 + aux(*(out+8)=*(p2+0x116))
    // ★★[07-23] **my_dd7700_code와 동일 수정을 full에도 반영**(code에만 적용돼 full이 stale이었음 = T_G2_6/T_G2_4 종단 전량 死):
    //   ~~`(tune("dd_early_p3_thr",0x2d) as u64) < p3 &&`~~ **게이트 삭제** — 0.5.2 원본 `0x1b91e40`에 p3(r8) 비교 게이트가 없다.
    //   실측 p3 ≤ 0x2d + cfg가 45(=0x2d)라 이 게이트가 조기분기를 **항상 차단**했다(근거 전문 = my_dd7700_code L3111 주석).
    //   ~~`!= 0`~~ → **`== 1`**(0.5.2에서 `cmp ...,1`로 엄격해짐). ⟹ `dd_early_p3_thr` = 死레버(설정편집기 제거 대상).
    if rd_u8(p2 + 0x112) == 1 && rd_u8(p2 + 0x115) == 0 {
        std::ptr::write_unaligned((out + 8) as *mut u8, rd_u8(p2 + 0x116));
        std::ptr::write_unaligned(out as *mut u64, if rd_u8(p2 + 0x113) != 0 { 6u64 } else { 4 });
        return Some(false);
    }
    let l80 = rd_u64(p6).unwrap_or(0) as usize;
    let vobj = rd_u64(p6 + 8).unwrap_or(0) as usize;
    let geo = rd_u64(p6 + 0x10).unwrap_or(0) as usize;
    if !ptr_ok(l80) || !ptr_ok(vobj) || !ptr_ok(geo) { return None; }
    let sim = rd_u64(l80).unwrap_or(0) as usize;   // robj==sim (l80[0])
    let vt  = rd_u64(l80 + 8).unwrap_or(0) as usize;
    if !ptr_ok(sim) || !ptr_ok(vt) { return None; }
    let plan = rd_u8(p7 + 0x3f6);                         // 0.5.0: p7+0x3e6→0x3ea
    let side = rd_i64(p5 + 0x810).unwrap_or(-1);         // 0.5.0: p5+0x6a8→0x820  ★0.5.4 오프셋 이동 반영
    if side != 0 && side != 1 { return None; }
    // ★라이너 포탑/인원수 보정(2026-06-23): self가 적포탑밑/수적열세면 code7(귀환) — early-guard(위 5168)와 동일 포맷(out+0=7, aux불요=검증된 dd7full 코드). dd7700=매라이너매프레임이라 라이너 다이브/불리교전 직접차단. RNG writeback(mp_capture 8102)은 별개 함수라 출력만 바꿔도 draw수 불변=RNG state 무손상(게임플레이만 의도분기). 기본(tower_threat=0&&numbers=0)=동작보존.
    // ★라이너 포탑/전력 후퇴는 my_dd7700_full **출력 후** mp_capture에서 적용(게임 base output code를 존중). 여기선 순수 게임판단 재현만.
    let s = side as usize;
    let lane = rd_i32(p5 + 0x8a0).unwrap_or(-1);         // 0.5.0: p5+0x738→0x8b0  ★0.5.4 오프셋 이동 반영
    let f = rd_u8(p2 + 0x116) as usize;   // F = byte[param_2+0x116] (0.5.0: 0x19→0x116)
    let s20 = dd7_slot20(sim);                        // ★호이스트: 현재틱(sim+0xed00)=호출 내 불변. 후보루프서 재읽기 제거
    let lane_margin = tune("dd_lane_margin", 0x78);   // ★호이스트: 루프불변 튜닝계수(per-candidate 조회 제거)
    let hdr = sim_hdr(sim);                           // ★호이스트: sim 헤더 1회 → cover/main 루프 slot48/a8 재사용

    // ── COVER BLOCK ──
    // ★★[07-23] **my_dd7700_code와 동일 수정을 full에도 반영**: ~~`if (tune("dd_cover_p3_thr",4) as u64) < p3 {`~~ **게이트 삭제**
    //   → 무조건 블록(0.5.2 원본에 `cmp r8,4; jbe` 없음. 근거 전문 = my_dd7700_code L3132 주석).
    //   ⚠**이게 disc0/1/3 편입의 핵심**: 이 게이트가 COVER 블록을 전량 차단해 왔고, 유저 튜닝 4종
    //   (`dd_frontier_mult`·`dd_lane_margin`·`dd_cover_count`·`dd_ratio_thr`)이 **전부 이 블록 안**이라
    //   화이트리스트만 풀었으면 "편입했는데 튜닝이 안 먹는" 상태가 됐을 것. ⟹ `dd_cover_p3_thr` = 死레버.
    {
        let cvar10 = f;   // (plan&0xfe)==8이면 p7[999]==F시 main으로; 아니면 cVar10=F
        let go_main = (plan & 0xfe) == 8 && rd_u8(p7 + 0x3f7) as usize == cvar10;  // 0.5.0: p7+0x3e7(999)→0x3eb
        if !go_main && cvar10 == 2 {
            // cVar10==2: frontier + lane + survivor 게이트. 4/7 출력 or main 폴.
            let mut cover_done = false;
            // ★frontier gate(07-10 재RE 정정): v=*(u8)(vobj+0x38), 발화 iff v∈{0,5,7,8}(마스크 0x1a1). prog<=slot20 → main(폴)
            let vb = rd_u8(vobj + 0x38);
            let frontier_bail = if vb <= 8 && (0x1a1u32 >> vb) & 1 == 1 {
                let v1 = rd_u64(vobj + 8).unwrap_or(0) as usize;
                let u19 = rd_u64(v1 + 0x8a8).unwrap_or(0);
                let l15 = rd_i64(v1 + 0x12f8).unwrap_or(0);
                let l15x30 = (((l15 as u64) as u128).wrapping_mul(tune("dd_frontier_mult", 0x1e) as u128) * 100 / (AGGR_LANE.load(Ordering::Relaxed).max(1) as u128)) as u64;   // ★튜닝×[3]공격성배율: 프론티어 진척 배수(aggr↑=l15x30↓=prog↑=덜 후퇴)
                let prog = if l15x30 <= u19 { u19 - l15x30 } else { 0 };
                prog <= s20 as u64
            } else { false };
            if !frontier_bail && (lane as i64) >= tune("dd_cover_role_min", 3) {
                let oidx = s * 5 + (if lane == 3 { 1 } else { 0 }) + 0x3f;
                let obj = rd_u64(l80 + oidx * 8).unwrap_or(0) as usize;
                // ★0.5.0 홈박스 동적화: base=[vobj+0x20], hb=base+0x6d70+side*0x20 → [hb]=x0/[+8]=y0/[+0x10]=x1/[+0x18]=y1
                let hb_base = rd_u64(vobj+0x20).unwrap_or(0) as usize; let hb = hb_base + 0x6d70 + s*0x20;
                let (xlo, ylo, xhi, yhi): (u64, u64, u64, u64) = (rd_u64(hb).unwrap_or(0), rd_u64(hb+8).unwrap_or(0), rd_u64(hb+0x10).unwrap_or(0), rd_u64(hb+0x18).unwrap_or(0));
                let proceed = obj == 0
                    || (rd_i32(obj + 0x68).unwrap_or(0) == 0xd && rd_i32(obj + 0x70).unwrap_or(0) == 1)
                    || { let ox = rd_u64(obj + 0x648).unwrap_or(0); let oy = rd_u64(obj + 0x650).unwrap_or(0);
                         xlo <= ox && ox <= xhi && ylo <= oy && oy <= yhi };
                if proceed {
                    let lv21 = 1 - s;
                    let mut cands = [0usize; 5]; let mut ncand = 0usize;   // ★Vec→스택배열(힙할당 제거, 후보 ≤5)
                    for k in 0..5usize { let c = rd_u64(l80 + 0x1e0 + lv21 * 0x28 + k * 8).unwrap_or(0) as usize; if c != 0 { cands[ncand] = c; ncand += 1; } }
                    if ncand != 0 {
                        let geo_side = geo + lv21 * 0x2e8;   // 0.5.0: geom stride 0x228→0x2e8
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
                                        let rlane = rd_i32(resolved + 0x8a0).unwrap_or(0) as usize;  // 0.5.0: resolved+0x738→0x8b0  ★0.5.4 오프셋 이동 반영
                                        let thr = rd_i64(geo_side + (rlane + 0x3c) * 8).unwrap_or(0);
                                        q = if s20 <= thr + lane_margin { 1 } else { 0 };   // (s20/lane_margin = 호이스트됨)
                                    }
                                }
                            }
                            count += q;
                        }
                        if count >= tune("dd_cover_count", 2) as u64 {   // ★튜닝: 커버 발화 적군 카운트
                            let team = dd7_slot128(sim, rd_u64(p5 + 0x818).unwrap_or(0));  // 0.5.0: p5+0x6a0→0x818
                            if team != 0 {
                                let tcnt = rd_u64(team + 0x610).unwrap_or(0);
                                if tcnt != 0 {
                                    let depth = rd_i64(team + 0x658).unwrap_or(0);
                                    let ratio = (depth as u64).wrapping_mul(100) / tcnt;
                                    let mut code = 4i64;
                                    if (ratio as i64) < tune("dd_ratio_thr", 0x33) { let fv = rd_i64(geo + 0x60 + s * 0x2e8).unwrap_or(0); code = if fv > tune("dd_facet_thr", 999) { 7 } else { 4 }; }   // ★튜닝: 커버 비율/페이즈 임계
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
    let self_handle = rd_u64(p5 + 0x818).unwrap_or(0);   // 0.5.0: p5+0x6a0→0x818
    let selfe = dd7_slot128(sim, self_handle);
    if !ptr_ok(selfe) { return None; }   // ★readable VQ제거(아래 selfx/selfy rd_u64 fault-safe, dd7700 메인바디 per-call)
    let local_90: u8 = (lane == 1) as u8;   // b8
    let other = 1 - s;
    let geom_other = other * 0x2e8 + geo;    // local_88 (threshold block, 0.5.0: 0x228→0x2e8)
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
                    let rlane = rd_i32(resolved + 0x8a0).unwrap_or(0) as usize;  // 0.5.0: resolved+0x738→0x8b0  ★0.5.4 오프셋 이동 반영
                    let thr = rd_i64(geom_other + (rlane + 0x3c) * 8).unwrap_or(0);
                    s20 <= thr + lane_margin   // (s20/lane_margin = 호이스트됨)
                }
            };
            if pass {
                let mut wp = s * 0x2e8 + geo;
                if f != 0 { wp += if f == 1 { 0x28 } else { 0x50 }; }
                let wpv = rd_i64(wp + 0x18).unwrap_or(0);
                local_58 = if wpv < 0 { 2 } else { 0 };
                break;
            }
        }
    }
    // LAB_aeea3: role-check. iVar12 = *(int)(side*0x228+geo + roleoff(F)). !=1 → af3d9(code2+aux); ==1 → engage(미포팅)
    let roleoff = if f == 0 { 0usize } else if f == 1 { 0x28 } else { 0x50 };
    let ivar12 = rd_i32(s * 0x2e8 + geo + roleoff).unwrap_or(0);
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
            return Some(false);  // ★[07-28 desync 수정] 0.5.2 실경기(GameMode 0)=native dd7700 전역 RNG **0 draw**(engage RNG소비자 f22e80=kind!=2에서 로컬 스택RNG). 구 CAND_FILTER(FUN_141fecbe0) per-cand 전역draw 모델=0.5.2 폐기 ⟹ rng_final(1~5 over-draw) skip=전역RNG불변=native일치. mpcmp 400/400은 code만봐 이 축 못잡음(=관전≠확정 진범). 근거=MIGRATION §7.2-A9 §2·§3 + ANA\ai_adjust-rng-desync-전수조사
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
    let plan = rd_u8(p7 + 0x3f6);                         // 0.5.0: p7+0x3e6→0x3ea
    let side = rd_i64(p5 + 0x810).unwrap_or(-1);         // 0.5.0: p5+0x6a8→0x820  ★0.5.4 오프셋 이동 반영
    if side != 0 && side != 1 { return None; }
    let s = side as usize;
    let lane = rd_i32(p5 + 0x8a0).unwrap_or(-1);         // 0.5.0: p5+0x738→0x8b0  ★0.5.4 오프셋 이동 반영
    let f = rd_u8(p2 + 0x116) as usize;                  // 0.5.0: p2+0x19→0x116
    // ★호이스트 + my_dd7700_full과 동일 튜닝값 사용(RNG-sync): cover-fire 예측이 실제 judge와 같은 임계를 써야 desync 없음.
    //   기본값(0x1e/0x78/2)에선 tune이 그대로 반환 → 검증된 DIFF=0 보존. 튜닝시 full과 일관.
    let s20 = dd7_slot20(sim);
    let lane_margin = tune("dd_lane_margin", 0x78);
    let frontier_mult = tune("dd_frontier_mult", 0x1e) as u64;
    let cover_count = tune("dd_cover_count", 2) as u64;
    let hdr = sim_hdr(sim);                            // ★호이스트: sim 헤더 1회 → cover 루프 slot48/a8 재사용
    DD7_RNG_DBG.store(plan as u64 | (f as u64) << 8, Ordering::Relaxed);
    // ── COVER BLOCK 검출: 발화시 main body 미도달 → CAND_FILTER 미실행(cover RNG 무소비) → None(0 draw). my_dd7700_full cover-fire와 동일조건. ──
    // ★★[07-23 누락 수정] ~~`if (tune("dd_cover_p3_thr",4) as u64) < p3 {`~~ **게이트 삭제** — code·full 양쪽은 같은 날 지웠는데
    //   **`rng_final`만 누락**돼 있었다. 0.5.2 원본 프롤로그(`0x1b91e40`~`0x1b91eb0`)에 **p3(r8)를 4와 비교하는 코드가 없다**
    //   (초기 게이트는 `byte[p2+0x110]/0x112/0x115/0x113`뿐, r8은 `[rbp-0x28]`에 저장만).
    //   방치 시 `p3<=4`에서 커버 fire 예측이 통째로 스킵돼 **RNG-sync 오예측** 위험(대체 경로 desync). ⬜인게임 미검증.
    {
        let go_main = (plan & 0xfe) == 8 && rd_u8(p7 + 0x3f7) as usize == f;  // 0.5.0: p7+0x3e7(999)→0x3eb
        if !go_main && f == 2 {
            let vb = rd_u8(vobj + 0x38);   // ★07-10 재RE 정정: +0x28→+0x38, 발화 iff v∈{0,5,7,8}(마스크 0x1a1)
            let frontier_bail = if vb <= 8 && (0x1a1u32 >> vb) & 1 == 1 {
                let v1 = rd_u64(vobj + 8).unwrap_or(0) as usize;
                let u19 = rd_u64(v1 + 0x8a8).unwrap_or(0);
                let l15 = rd_i64(v1 + 0x12f8).unwrap_or(0);
                let l15x30 = (((l15 as u64) as u128).wrapping_mul(frontier_mult as u128) * 100 / (AGGR_LANE.load(Ordering::Relaxed).max(1) as u128)) as u64;   // full과 동일 튜닝값 ×[3]공격성배율
                let prog = if l15x30 <= u19 { u19 - l15x30 } else { 0 };
                prog <= s20 as u64
            } else { false };
            if !frontier_bail && (lane as i64) >= tune("dd_cover_role_min", 3) {
                let oidx = s * 5 + (if lane == 3 { 1 } else { 0 }) + 0x3f;
                let obj = rd_u64(l80 + oidx * 8).unwrap_or(0) as usize;
                // ★0.5.0 홈박스 동적화: base=[vobj+0x20], hb=base+0x6d70+side*0x20 → [hb]=x0/[+8]=y0/[+0x10]=x1/[+0x18]=y1
                let hb_base = rd_u64(vobj+0x20).unwrap_or(0) as usize; let hb = hb_base + 0x6d70 + s*0x20;
                let (xlo, ylo, xhi, yhi): (u64, u64, u64, u64) = (rd_u64(hb).unwrap_or(0), rd_u64(hb+8).unwrap_or(0), rd_u64(hb+0x10).unwrap_or(0), rd_u64(hb+0x18).unwrap_or(0));
                let proceed = obj == 0
                    || (rd_i32(obj + 0x68).unwrap_or(0) == 0xd && rd_i32(obj + 0x70).unwrap_or(0) == 1)
                    || { let ox = rd_u64(obj + 0x648).unwrap_or(0); let oy = rd_u64(obj + 0x650).unwrap_or(0);
                         xlo <= ox && ox <= xhi && ylo <= oy && oy <= yhi };
                if proceed {
                    let lv21 = 1 - s;
                    let geo_side = geo + lv21 * 0x2e8;   // 0.5.0: geom stride 0x228→0x2e8
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
                                else { let rlane = rd_i32(resolved + 0x8a0).unwrap_or(0) as usize;  // 0.5.0: resolved+0x738→0x8b0  ★0.5.4 오프셋 이동 반영
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
    DD7_RNG_PI14.store(s * 0x2e8 + geo + roleoff, Ordering::Relaxed);   // role record addr (exit 재독용)
    if rd_i32(s * 0x2e8 + geo + roleoff).unwrap_or(0) != 1 { return None; }
    DD7_RNG_DBG.fetch_or(1 << 12, Ordering::Relaxed);         // iVar12==1
    // target resolve (vt[0x140](robj, *(pi14+8))). 0 → af3d9 early, CAND_FILTER 미도달
    let resolver = rd_u64(vt + 0x140).unwrap_or(0) as usize;
    if !ptr_ok(resolver) { return None; }
    let tgt_handle = rd_u64(s * 0x2e8 + geo + roleoff + 8).unwrap_or(0);
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
    let cand_base = rd_u64(p4 + 0x3c8)? as usize;   // 불변 추정(e9probe 미검증, engfoot로 확인)
    let cand_cnt = rd_u64(p4 + 0x3d0)?;             // facetcnt 불변(e9probe: p5+0x3d0=0)
    let threshold = rd_u64(p4 + 0x888)?;   // 0.5.0: p4+0x710→0x888(+0x178, e9probe 실측)
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
    let cand_base = rd_u64(p4 + 0x3c8)? as usize;   // 불변 추정(e9probe 미검증, engfoot로 확인)
    let cand_cnt = rd_u64(p4 + 0x3d0)?;             // facetcnt 불변(e9probe: p5+0x3d0=0)
    let threshold = rd_u64(p4 + 0x888)?;   // 0.5.0: p4+0x710→0x888(+0x178, e9probe 실측)
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
    // candidate = rvt[0x138](robj, [p5+0x818]) — 0.5.0: resolver rvt+0x128→0x138=dd7_slot128 순수재현(shadow-call 제거=AV방지)
    if !readable(p5 + 0x810, 8) { return None; }   // 0.5.0(was 0x6a8, SimState team +0x178)  ★0.5.4 오프셋 이동 반영
    let team_units = rd_u64(p5 + 0x818)?;           // 0.5.0(was 0x6a0, self-handle +0x178)
    let cand = dd7_slot128(robj, team_units);       // 0.5.0: rvt+0x138 resolver=dd7_slot128 동일 4단 chase
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
    // dist = rvt[0x28](robj) — 0.5.0: getter rvt+0x20→0x28 (pure getter, 슬롯만 교정)
    let g20 = vt_slot(rvt, 0x28); if !ptr_ok(g20) { return None; }
    let f20: VtPtrFn = core::mem::transmute(g20);
    let dist = f20(robj) as u64;
    if dist >= r15 { return Some(false); }          // 거리>=thr → al=0(FAIL)
    // 좌표 비교: dx=|candx - tX|, dy=|candy - tY|
    // ★0.5.0_2: tableC/D 정적표 폐기 → 런타임 컨테이너 순회로 목표좌표 획득.
    //   holder=[[p6+8]+0x20], base=[holder+0x68], count=[holder+0x70], stride0x28.
    //   entry중 byte[entry+0x20]==p1 매칭 → tX=[entry+q*0x10+0], tY=[entry+q*0x10+8] (q 스왑=오프셋선택).
    let candx = rd_u64(cand + 0x648)?;
    let candy = rd_u64(cand + 0x650)?;
    let pg_p7 = rd_u64(p6 + 8)? as usize;
    if !ptr_ok(pg_p7) { return None; }
    let holder = rd_u64(pg_p7 + 0x20)? as usize;
    if !ptr_ok(holder) { return None; }
    let cbase = rd_u64(holder + 0x68)? as usize;
    let ccount = rd_u64(holder + 0x70)?;
    if !ptr_ok(cbase) { return None; }
    let (mut tx, mut ty) = (0u64, 0u64);
    let mut found = false;
    let mut ci = 0u64;
    while ci < ccount && ci < 64 {                    // 안전 상한(순회 폭주 방지)
        let entry = cbase + (ci as usize) * 0x28;
        if !readable(entry + 0x20, 1) { break; }
        if rd_u8(entry + 0x20) as u64 == p1 {
            let qo = (q as usize) * 0x10;
            tx = rd_u64(entry + qo)?;
            ty = rd_u64(entry + qo + 8)?;
            found = true;
            break;
        }
        ci += 1;
    }
    if !found { return None; }                        // 매칭 엔트리 없음 → 보수 passthrough
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
    // ★★[07-28 desync 안전판 — dd7 3719식] **전 케이스 passthrough**(native가 engage 전체 수행 = 결정성 보존).
    //   0.5.2 재RE(engage ENTRY 0x1b94670) 확정 불일치 3건: ①roll 출력 5→**8** 변경(모드 5=매 케이스 오염)
    //   ②게이트 극성 정반대 — 0.5.2 신설 MAIN body(타겟커밋·추가 draw 2사이트·0x118B 구조체 출력)는 재현 불가인데
    //     현행 코드는 그 절반을 3-draw 모델+8B out으로 덮고, 재현 가능한 즉시-roll 절반은 passthrough = 정확히 반대
    //   ③count 오프셋 stale(0x3c8→0x450·0x440→0x4c0·p7+0x20→+0x30 등) = count 쓰레기 → under-draw.
    //   ⟹ 이 조합이 배경≠관전 25% desync의 주범(실측: recall/engage/cond만 켠 런서 발산·recall/cond 무죄 RE).
    //   재활성 조건(별건): 극성 반전(rcx<rdi 절반만 대체) + 5→8 + 오프셋 전면 재핀 + e88a0 alive(vt0x50) + thr=10 분기 → game==mine 재검증.
    //   부작용: engage emit 경유 튜닝(eng_role*/t_engage 등) 잠정 무력화(native 원본 동작).
    return Some(false);
    #[allow(unreachable_code)]
    {
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
    let s = vt_slot(rvt, 0x28); if !ptr_ok(s) { return None; }   // 0.5.0: getter rvt+0x20→0x28
    let g: VtPtrFn = core::mem::transmute(s);
    let dist = g(robj) as u64;
    let rcx = if dist >= baseline { dist - baseline } else { 0 };
    if rcx < rdi { return Some(false); }   // distance gate fail → retreat -1(e88a0만)
    Some(true)
    }   // ★[07-28] 안전판 unreachable 블록 끝
}
// ════ ★인원수(머릿수)·포탑 회피 — 게임 원본 AI에 없는 신규 항 (2026-06-22) ════
//   engage 교전(5) 결정시: ① 근처 적챔피언>아군+margin → 후퇴 ② self가 적 포탑 사거리내 → 후퇴.
//   ★self 위치 = engage_self_pos(p5+0x6a0 핸들 → dd7_slot128 챔피언resolve). 엔진 self_e는 RNG홀더라 +0x648=위치 아님(캡처확인).
//   ★RNG 무관: roll/writeback 다 소비 후 출력만 보정 → draw수 불변. 기본(margin=0·threat=0)=동작보존.
//   카운트=dd7700 검증 로스터(rh=*(p6)+0x1e0+team*0x28+k*8, 5슬롯). 포탑=l80 oidx 0x3c..0x45 type-13(obj+0x68==0xd), 적팀=obj+0x8==1-q(캡처확인 5/팀).
static NUMBERS_MARGIN: AtomicI64 = AtomicI64::new(0);     // cfg numbers_margin: 0=off, ≥1=적−아군≥margin이면 후퇴(단순 binary)
static NUMBERS_THREAT: AtomicI64 = AtomicI64::new(0);     // ★cfg numbers_threat 0~100(0=off): 일반교전 전력(force)승산 임계(폴백). numbers_threat≥승산이면 후퇴(강하면 적어도 싸움)
// ★subplan별 개별 임계: cfg numbers_threat_sp<N>(N=0..15, N=런타임 disc값). -1=미설정→폴백 numbers_threat. 실명(§11.8): 2=LineDefense / 3=LineAttack(라인전) / 4=LineSafe(정글) / 7=Recall / 8=Jungle / 9=Battle / 11=Hide / 13=EpicHunt / 14=EpicPoke. (구라벨 ForcePassive/PassiveLine/PassiveJungle/LineGanker/Cover/EpicHunt/SerpenHunt/AttackNexus/DefenseNexus=0.4.14 유산 오라벨)
static NUMBERS_THREAT_SP: [AtomicI64; 18] = [const { AtomicI64::new(-1) }; 18];   // ★[수정 07-16] 16→18: sp16/17(SerpenHunt/Poke) idx가 배열 초과로 저장/read 불가였음
static NUMBERS_THREAT_SP_ANY: AtomicBool = AtomicBool::new(false);   // subplan별 임계 하나라도 설정됨 → dd7700 호출 게이트 활성
// ★[수정 07-31] 16→18: `apply_numbers_sp`가 `.min(15)`로 클램프해 **disc15/16/17 발동이 슬롯15에 뭉개져** 있었다
//   (실측 sp_seen.txt `subplan 15 = 4127` = 세르펜 3종 합계). NUMBERS_THREAT_SP와 같은 18칸으로 맞춰 disc별 분리.
//   ※진단 전용(게임 동작 무영향) — 주석의 "dead"는 오기였다(apply_numbers_sp가 실제로 쓴다).
static SP_SEEN: [AtomicU64; 18] = [const { AtomicU64::new(0) }; 18];
static SP_SEEN_FRAME: AtomicU64 = AtomicU64::new(0);   // post_update 프레임 스로틀 카운터
static SP_SEEN_LAST: AtomicU64 = AtomicU64::new(0);    // 직전 덤프 시점의 총합(변화 없으면 재기록 생략)
// ★★[2026-07-31 확장] **장기 누적 측정 모드** — 유저가 팀전술을 바꿔가며 여러 판을 연속으로 돌리는 용도.
//   설계 요점 4가지:
//    ① **`log` 와 분리된 전용 키 `sp_seen`** — `log=1` 은 mpcmp append 등 무거운 로깅까지 같이 켜서
//       수십 판 연속 측정엔 부담이다. 이 지표만 필요하면 `sp_seen=1` 하나면 된다(write도 LOG_ON 무관 직접).
//    ② **게임 재시작에도 누적 유지** — 프로세스 statics는 0으로 시작하므로 `sp_seen_acc.txt` 에서 이전 총계를
//       읽어 `SP_BASE` 로 이어붙인다. 표시 총계 = BASE + 이번 실행분.
//    ③ **`sp_seen_tag` 로 구간 분리** — 전술을 바꿀 때 이 값을 바꾸면, 직전 구간이 `sp_seen_hist.txt` 에
//       한 줄로 확정되고 카운터가 리베이스된다 ⟹ **전술별 비교표**가 자동으로 쌓인다.
//       (씬 전환으로 경기 경계를 추측하지 않는다 — 배경 sim도 카운터를 올리므로 추측은 틀린다. 라벨은 유저가 준다.)
//    ④ detour는 여전히 **원자 카운터만** 올린다(파일IO 금지 규칙 유지).
static SP_SEEN_ON: AtomicBool = AtomicBool::new(false);          // cfg sp_seen (log과 독립)
static SP_BASE: [AtomicU64; 18] = [const { AtomicU64::new(0) }; 18];   // 이전 실행들에서 이어받은 누적
static SP_SEG: [AtomicU64; 18] = [const { AtomicU64::new(0) }; 18];    // 현재 태그 구간 시작 시점의 SP_SEEN 스냅샷
static SP_ACC_LOADED: AtomicBool = AtomicBool::new(false);
static SP_SEG_LOADED: AtomicBool = AtomicBool::new(false);   // acc 파일에 구간 시작점(s<N>)이 있었는가
static SP_RUNS: AtomicU64 = AtomicU64::new(0);                   // 게임 실행(프로세스) 횟수
static SP_LABELS: [&str; 18] = ["라인전0","라인전1","LineDefense","라인전3","LineSafe정글","귀환5","교전6","Recall이동",
    "갱커버","Battle견제","오브배틀","Hide견제","EpicCheck","EpicHunt","EpicPoke","SerpenCheck","SerpenHunt","SerpenPoke"];
fn sp_tag_cfg() -> String { SP_TAG_CFG.lock().unwrap_or_else(|e| e.into_inner()).clone() }
fn sp_tag_cur() -> String { SP_TAG_CUR.lock().unwrap_or_else(|e| e.into_inner()).clone() }
static SP_TAG_CFG: Mutex<String> = Mutex::new(String::new());   // cfg sp_seen_tag (유저가 적는 전술 라벨 / "auto"=인게임 팀전술 자동)
static SP_TAG_CUR: Mutex<String> = Mutex::new(String::new());   // 현재 진행 중인 구간의 라벨
// ★★[07-31 5차] **조합 테스트 결과 저장처 추적기** (cfg `ct_hunt=1`)
//   전제: 인게임에서 조합 테스트 **리플레이를 볼 수 있다** ⟹ 어딘가에는 반드시 저장된다.
//   지금까지 배제된 곳: `MatchType::Practice`(0건) · `MatchInfo.is_practice/is_room_practice`(전부 false)
//                     · `Team.strategy`(화면과 7/12 불일치) · `match_replays` 최대 id(리그 경기만 잡힘).
//   ⚠"최대 id = 최신"이라는 가정 자체가 미검증이다 — 조합 테스트가 **고정 슬롯을 덮어쓰거나** 낮은 키를 쓰면 그 방식으론 영원히 못 본다.
//   ⟹ 추측을 접고 **스냅샷 diff**로 간다: 조합 테스트 실행 전후로 DB를 비교해 **무엇이 새로 생기거나 바뀌었는지**를 그대로 보고.
//      (어느 컨테이너인지 모르는 상태에서 "변한 것"만 잡아내는 방식이라 저장처를 몰라도 답이 나온다.)
static CT_HUNT: AtomicBool = AtomicBool::new(false);
static CT_TICK: AtomicU64 = AtomicU64::new(0);
static CT_SEEN_REPLAYS: Mutex<Option<std::collections::HashSet<u64>>> = Mutex::new(None);
static CT_SEEN_SIGS: Mutex<Option<HashMap<u64, String>>> = Mutex::new(None);   // id → blue/red 전술 서명(값 변경 감지)
static CT_LAST_COUNTS: Mutex<Option<(usize, usize)>> = Mutex::new(None);       // (matches, match_replays)
static CT_LOG_N: AtomicU64 = AtomicU64::new(0);

fn ct_hunt_scan(r: &ClientDatabase) {
    let mut out = String::new();
    // ① MatchType 변종 분포 — 조합 테스트 전용 variant 가 있는지 확인(최초 1회만 기록)
    if CT_LOG_N.fetch_add(1, Ordering::Relaxed) == 0 {
        let mut kinds: HashMap<String, u32> = HashMap::new();
        for (mt, _mi) in r.matches.iter() {
            let d = format!("{:?}", mt);
            let name = d.split(|c: char| c == ' ' || c == '{' || c == '(').next().unwrap_or("?").to_string();
            *kinds.entry(name).or_insert(0) += 1;
        }
        let mut ks: Vec<_> = kinds.into_iter().collect();
        ks.sort_by(|a, b| b.1.cmp(&a.1));
        out.push_str("=== 조합테스트 저장처 추적 (ct_hunt) ===\n[최초 스캔] MatchType 분포:\n");
        for (k, v) in ks { out.push_str(&format!("   {:<28} {}\n", k, v)); }
        out.push_str("\n이후로는 '변화'만 기록합니다. 조합 테스트를 한 판 돌리세요.\n\n");
    }

    // ② 현재 replay 집합 + 각 replay 의 전술 서명
    let mut cur: std::collections::HashSet<u64> = std::collections::HashSet::new();
    let mut sigs: HashMap<u64, String> = HashMap::new();
    for (_k, rep) in r.match_replays.iter() {
        let id = rep.id as u64;
        cur.insert(id);
        sigs.insert(id, format!("B[{}] R[{}]", sp_strat_sig(&rep.blue_strategy), sp_strat_sig(&rep.red_strategy)));
    }
    let counts = (r.matches.len(), r.match_replays.len());

    let mut prev_keys = CT_SEEN_REPLAYS.lock().unwrap_or_else(|e| e.into_inner());
    let mut prev_sigs = CT_SEEN_SIGS.lock().unwrap_or_else(|e| e.into_inner());
    let mut prev_cnt = CT_LAST_COUNTS.lock().unwrap_or_else(|e| e.into_inner());

    if let (Some(pk), Some(ps), Some(pc)) = (prev_keys.as_ref(), prev_sigs.as_ref(), prev_cnt.as_ref()) {
        if *pc != counts {
            out.push_str(&format!("[개수변화] matches {}→{} · match_replays {}→{}\n", pc.0, counts.0, pc.1, counts.1));
        }
        // 새로 생긴 replay
        for id in cur.difference(pk) {
            out.push_str(&format!("[★새 replay] id={}\n     {}\n", id, sigs.get(id).map(|s| s.as_str()).unwrap_or("?")));
        }
        // 사라진 replay
        for id in pk.difference(&cur) { out.push_str(&format!("[사라짐] id={}\n", id)); }
        // ★기존 id인데 내용이 바뀐 것 = "고정 슬롯 덮어쓰기" 가설의 결정적 증거
        for (id, s) in sigs.iter() {
            if let Some(old) = ps.get(id) {
                if old != s { out.push_str(&format!("[★내용변경] id={} (같은 슬롯을 덮어씀!)\n     이전 {}\n     이후 {}\n", id, old, s)); }
            }
        }
    }
    *prev_keys = Some(cur); *prev_sigs = Some(sigs); *prev_cnt = Some(counts);

    if !out.is_empty() {
        if let Some(p) = pth("ct_hunt.txt") {
            if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = write!(f, "{}", out); }
        }
    }
}

static SP_STRAT_SIG: Mutex<String> = Mutex::new(String::new());   // 인게임 팀전술 12필드 서명(자동 라벨용)
static SP_STRAT_RAW: Mutex<String> = Mutex::new(String::new());   // 검증용: Team+0x318 원시 24B hex + 해독

// ★[신설 07-31] **인게임 팀전술(Strategy)을 읽어 구간 라벨을 자동 생성**.
//   소스 = `Team+0x318`(현재 전술 24B·`+0x300`=직전). 12 서브필드 byte 오프셋은 기존 `STRAT_OFFS_ROT` 와 동일 매핑을 재사용.
//   ⚠이 오프셋은 0.4.14 실측 유래다 — 0.5.3 유효성은 **런타임 자가검증**으로 확인한다:
//     각 필드가 `STRAT_VC` 범위(2~3)를 넘으면 `범위초과` 로 표시해 오프셋 오류를 즉시 드러낸다.
//     원시 24B hex도 같이 기록하므로, 유저가 인게임에서 전술을 바꿨을 때 어느 바이트가 움직이는지 대조할 수 있다.
//   ⚠★알려진 RE 결론: **sim 판단함수는 Strategy를 읽지 않는다**([[tfm2-team-strategy-tactics]] 확정).
//     ⟹ 전술을 바꿔도 아래 분포가 안 변하는 것이 "예상되는" 결과이고, 이 측정은 그 결론의 라이브 검증이 된다.
const STRAT_FIELD_NAMES: [&str; 12] = ["foc","jng","srp","srt","bld","bat","mor","twr","def","fin","wav","end"];
// ★★[07-31 2차] **소스 정정** — 유저가 바꾸는 것은 관리팀 Strategy가 아니라 **연습경기 블루/레드 전술**이었다
//   (스크린샷 = 블루/레드 12개 드롭다운 = practice match 설정). 그 자리는 `MatchReplayData` 의 blue@+0x78 / red@+0x90.
//   ⚠단 그 오프셋은 0.4.14 유래이고 **0.5.0에서 구조 stride가 바뀐 뒤 재검증이 안 됐다**(메모리에 명시된 미검증 항목).
//   ⟹ 추측으로 또 틀리지 않도록, 확정 전까지는 **원시 바이트를 넓게 덤프**해서 인게임 화면과 직접 대조한다.
//   판별 지문: 유저 화면에서 블루와 레드가 **초반세르펜(srp) 한 필드만** 다르다(블루=되도록포기 / 레드=무조건시도)
//   ⟹ 두 24B 블록이 **정확히 1바이트만 다른** 지점을 찾으면 그게 blue/red strategy 쌍이다.
static SP_PROBE_DONE: AtomicBool = AtomicBool::new(false);
static SP_PROBE_TRY: AtomicU64 = AtomicU64::new(0);   // 120프레임(≈2초)마다 1회만 재시도

// ★★★[07-31 3차·확정] **원시 오프셋 추적은 애초에 불필요했다.**
//   rustc 프로버(E0027)로 SDK 타입을 직접 물어보니 전부 공개 필드였다:
//     `MatchReplayData { id, blue_team_id, red_team_id, blue_ban, red_ban, blue_team, red_team,
//                        blue_strategy, red_strategy, seed, blue_team_win, game_tick, is_brief, … }`
//     `game_core::Strategy { focused, early_jungle, early_serpen, early_serpen_top, object_buildup,
//                            object_battle, morgard_use, tower_press, morgard_defense, object_finish,
//                            minion_wave, game_finish }`  ← 인게임 12개 드롭다운과 1:1
//   ⟹ `+0x78/+0x90` 하드코딩·24B 해독·바이트 덤프 전부 폐기하고 **SDK 필드 직독**으로 간다
//      (버전이 올라가도 오프셋 재핀이 필요 없다 = 마이그 부담 0).
//   ⚠또한 `MatchInfo` 에는 `is_practice`/`is_room_practice` **불리언**이 따로 있다 —
//     구 필터 `MatchType::Practice{..}` 로 0건이 나온 원인이 이것일 수 있다(맵 키 타입과 별개 플래그).
fn sp_strat_sig(s: &game_core::Strategy) -> String {
    format!("foc{:?}/jng{:?}/srp{:?}/srt{:?}/bld{:?}/bat{:?}/mor{:?}/twr{:?}/def{:?}/fin{:?}/wav{:?}/end{:?}",
        s.focused, s.early_jungle, s.early_serpen, s.early_serpen_top, s.object_buildup, s.object_battle,
        s.morgard_use, s.tower_press, s.morgard_defense, s.object_finish, s.minion_wave, s.game_finish)
}
// MatchReplayData 앞부분을 통째로 떠서 blue/red strategy 후보를 자동 탐색한다.
const O_TEAM_STRAT: usize = 0x318;   // Team+0x318 = 현재 Strategy(24B)
// 누적 파일 로드: `d<N>=<v>` / `runs=<n>` / `tag=<s>` 줄만 읽는다(포맷 단순 = 손으로 편집·초기화 가능).
fn sp_acc_load() {
    if SP_ACC_LOADED.swap(true, Ordering::Relaxed) { return; }
    if let Some(p) = pth("sp_seen_acc.txt") {
        if let Ok(t) = fs::read_to_string(&p) {
            for line in t.lines() {
                let line = line.trim();
                let (k, v) = match line.split_once('=') { Some(kv) => kv, None => continue };
                let (k, v) = (k.trim(), v.trim());
                if let Some(n) = k.strip_prefix('d') {
                    if let (Ok(i), Ok(val)) = (n.parse::<usize>(), v.parse::<u64>()) {
                        if i < 18 { SP_BASE[i].store(val, Ordering::Relaxed); }
                    }
                } else if let Some(n) = k.strip_prefix('s') {
                    // ★구간 시작점도 영속화 — 없으면 재시작할 때마다 "이번구간"이 0부터 다시 세어져
                    //   전술 하나를 여러 세션에 걸쳐 돌릴 때 구간 통계가 쪼개진다.
                    if let (Ok(i), Ok(val)) = (n.parse::<usize>(), v.parse::<u64>()) {
                        if i < 18 { SP_SEG[i].store(val, Ordering::Relaxed); SP_SEG_LOADED.store(true, Ordering::Relaxed); }
                    }
                } else if k == "runs" {
                    if let Ok(n) = v.parse::<u64>() { SP_RUNS.store(n, Ordering::Relaxed); }
                } else if k == "tag" {
                    *SP_TAG_CUR.lock().unwrap_or_else(|e| e.into_inner()) = v.to_string();
                }
            }
        }
    }
    // 구간 시작점 기록이 없던 파일(구 포맷)·최초 실행이면 "지금까지의 누적"을 구간 시작점으로 잡는다.
    if !SP_SEG_LOADED.load(Ordering::Relaxed) {
        for d in 0..18usize { SP_SEG[d].store(SP_BASE[d].load(Ordering::Relaxed), Ordering::Relaxed); }
    }
    SP_RUNS.fetch_add(1, Ordering::Relaxed);   // 이번 프로세스 = 1회 실행
    if sp_tag_cur().is_empty() { *SP_TAG_CUR.lock().unwrap_or_else(|e| e.into_inner()) = sp_tag_cfg(); }
}

// 구간 확정: 현재 태그 구간의 델타를 sp_seen_hist.txt 에 한 줄 append 하고 리베이스.
fn sp_seg_close(tag: &str) {
    let mut body = String::new();
    let mut tot = 0u64;
    for d in 0..18usize {
        let cur = SP_BASE[d].load(Ordering::Relaxed) + SP_SEEN[d].load(Ordering::Relaxed);
        let seg = cur.saturating_sub(SP_SEG[d].load(Ordering::Relaxed));
        SP_SEG[d].store(cur, Ordering::Relaxed);   // 리베이스
        tot += seg;
        if seg > 0 { body.push_str(&format!(" sp{}={}", d, seg)); }
    }
    if tot == 0 { return; }   // 빈 구간은 기록하지 않음
    if let Some(p) = pth("sp_seen_hist.txt") {
        let line = format!("[{}] 합계={}{}\n", if tag.is_empty() { "(라벨없음)" } else { tag }, tot, body);
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = write!(f, "{}", line); }
    }
}

fn sp_seen_flush() {
    if !SP_SEEN_ON.load(Ordering::Relaxed) { return; }
    if SP_SEEN_FRAME.fetch_add(1, Ordering::Relaxed) % 300 != 0 { return; }
    sp_acc_load();
    // ★태그가 바뀌었으면(=유저가 전술을 교체했으면) 직전 구간을 확정하고 새 구간 시작
    //   cfg가 비었거나 "auto" 면 **인게임 팀전술 서명**을 라벨로 쓴다 ⟹ 유저는 게임에서 전술만 바꾸면 된다.
    //   ⛔[07-31 4차] `auto`(=DB replay 전술로 자동 구간 분리)는 **조합 테스트에 대해 무효**다.
    //     조합 테스트 경기는 `match_replays` 에 저장되지 않아 항상 리그 경기의 전술만 읽히고,
    //     그러면 유저가 전술을 바꿔도 라벨이 안 바뀌어 **구간이 통째로 뭉개진다**(실제로 1차 측정 25만건이 그렇게 날아갔다).
    //     ⟹ auto 는 "(auto-무효)" 고정 라벨로 떨어뜨려 조용히 잘못 나뉘는 일이 없게 한다. 구간 분리는 수동 라벨이 정본.
    let cfg_tag = sp_tag_cfg();
    let want = if cfg_tag.is_empty() || cfg_tag.eq_ignore_ascii_case("auto") {
        "(auto-무효·sp_seen_tag에 전술이름을 직접 적으세요)".to_string()
    } else { cfg_tag };
    let cur = sp_tag_cur();
    if want != cur {
        sp_seg_close(&cur);
        *SP_TAG_CUR.lock().unwrap_or_else(|e| e.into_inner()) = want;
    }
    let mut total = 0u64;
    let mut seg_total = 0u64;
    let mut rows = String::new();
    let mut acc = String::new();
    for d in 0..18usize {
        let run = SP_SEEN[d].load(Ordering::Relaxed);
        let cum = SP_BASE[d].load(Ordering::Relaxed) + run;
        let seg = cum.saturating_sub(SP_SEG[d].load(Ordering::Relaxed));
        total += cum; seg_total += seg;
        acc.push_str(&format!("d{}={}\ns{}={}\n", d, cum, d, SP_SEG[d].load(Ordering::Relaxed)));
        if cum > 0 {
            rows.push_str(&format!("  sp{:<2} {:<12} 누적 {:>9}   이번구간 {:>9}   이번실행 {:>9}\n",
                d, SP_LABELS[d], cum, seg, run));
        }
    }
    if total == 0 || total == SP_SEEN_LAST.swap(total, Ordering::Relaxed) { return; }
    let tag = sp_tag_cur();
    let s = format!("=== numbers_sp 후퇴발동 누적 측정 ===\n\
        게임 실행 횟수 = {}회 | 현재 구간 라벨 = {}\n\
        ※구간은 cfg `sp_seen_tag` 로 나눕니다 — 전술을 바꿀 때 이 값을 바꾸면 직전 구간이 sp_seen_hist.txt 에 확정됩니다.\n\
        ※'누적'=게임 재시작 포함 전체 / '이번구간'=라벨을 바꾼 뒤 / '이번실행'=이번 게임 실행분\n\n\
        {}\n  {:<16} 누적 {:>9}   이번구간 {:>9}\n\n\
        ── 참고: DB 최신 replay 의 전술(리그 경기) ──\n  {}\n\
        ⚠**조합 테스트로 돌린 경기는 여기 안 잡힙니다**(match_replays 에는 리그 경기만 쌓임).\n\
          따라서 이 줄로 조합 테스트의 전술을 판단하지 마세요 — 구간 구분은 위 `sp_seen_tag` 수동 라벨을 쓰세요.\n",
        SP_RUNS.load(Ordering::Relaxed), if tag.is_empty() { "(없음)" } else { &tag },
        rows, "합계", total, seg_total,
        SP_STRAT_RAW.lock().unwrap_or_else(|e| e.into_inner()).clone());
    if let Some(p) = pth("sp_seen.txt") { let _ = fs::write(p, &s); }   // ★LOG_ON 무관 직접 write(측정 자가완결·perf.txt와 동일 방침)
    if let Some(p) = pth("sp_seen_acc.txt") {
        let _ = fs::write(p, format!("# 누적 카운터(자동 생성). 초기화하려면 이 파일을 지우세요.\nruns={}\ntag={}\n{}",
            SP_RUNS.load(Ordering::Relaxed), tag, acc));
    }
}
// ★dd7700 출력코드(게임 판단)별 임계 — code 2(Move/라인워크)만 따로. -1=폴백(numbers_threat=지금과 동일).
//   dd7700이 PassiveLine 전용이라 subplan으론 라인전↔딜교/갱 구분불가 → 게임이 내는 출력 code(2=라인워크 / 4·6·7=교전·귀환)로 구분.
static NUMBERS_THREAT_MOVE: AtomicI64 = AtomicI64::new(0);   // ★기본 0=라인워크 보존(cfg에 키 없어도). numbers_threat=0이면 무관(원본보존).
// ★아군 포탑 전력 가산: 근처 아군포탑 force(curHP×tower_dps)를 아군 force에 더함 → 타워밑 승산↑ → 덜 빠짐(타워 끼고 버티기).
// HP기여·DPS기여 따로 + 라인전(_move)·한타 따로(전력측정 numbers_threat/move와 동일패턴). -1(_move)=폴백(한타값 따름).
static ALLY_TOWER_HP: AtomicI64 = AtomicI64::new(0);         // 포탑 HP 전력가중(한타) 0~100 → 아군 ΣHP에 가산
static ALLY_TOWER_HP_MOVE: AtomicI64 = AtomicI64::new(-1);   // 포탑 HP 전력가중(라인전 Move) -1=폴백
static ALLY_TOWER_DPS: AtomicI64 = AtomicI64::new(0);        // 포탑 DPS 전력가중(한타) 0~100 → 아군 Σ공격에 가산
static ALLY_TOWER_DPS_MOVE: AtomicI64 = AtomicI64::new(-1);  // 포탑 DPS 전력가중(라인전 Move) -1=폴백
static ALLY_TOWER_RANGE: AtomicU64 = AtomicU64::new(150000); // 아군 포탑 인식범위(한타)
static ALLY_TOWER_RANGE_MOVE: AtomicI64 = AtomicI64::new(-1); // 아군 포탑 인식범위(라인전). -1=한타값
static ALLY_TOWER_THP: AtomicU64 = AtomicU64::new(0);        // 진단: 포탑 HP기여(아군 ΣHP에 더한 값)
static ALLY_TOWER_TATK: AtomicU64 = AtomicU64::new(0);       // 진단: 포탑 DPS기여(Σ공격에 더한 값)
static ALLY_TOWER_CNT: AtomicU64 = AtomicU64::new(0);        // 진단: 인식범위 내 아군 포탑 수
static SEEN_C2: AtomicU64 = AtomicU64::new(0);    // 진단: 게임이 Move(2/라인워크) 낸 force판정 진입
static SEEN_C67: AtomicU64 = AtomicU64::new(0);   // 진단: 게임이 교전/귀환(4/6/7) 낸 진입
static RET_C2: AtomicU64 = AtomicU64::new(0);     // 진단: Move(라인워크)를 numbers가 후퇴로 덮은 횟수(=멀뚱멀뚱 주범)
static RET_C67: AtomicU64 = AtomicU64::new(0);    // 진단: 교전/귀환 출력에 numbers 후퇴
// ★라인전↔한타 구분(subplan은 97% 0이라 무용 → 근처 적 수로 구분): force 후퇴는 근처 적 챔프 ≥ numbers_min_enemy 일때만.
//   1=현행(1:1 라인전도 후퇴) / 2=라인전(1:1) 제외하고 갱·한타(2명+)만 후퇴 = 라인전 미니언/딜교 살림.
static NUMBERS_MIN_ENEMY: AtomicI64 = AtomicI64::new(1);
static NUMBERS_MIN_ENEMY_MOVE: AtomicI64 = AtomicI64::new(-1);   // 라인전 전용 머릿수게이트. -1=한타값 따름
static FRC_E1: AtomicU64 = AtomicU64::new(0);   // 진단: force후퇴 중 근처적 ≤1 (라인전류)
static FRC_E2: AtomicU64 = AtomicU64::new(0);   // 진단: force후퇴 중 근처적 ≥2 (한타류)
static NUMBERS_RANGE: AtomicU64 = AtomicU64::new(150000); // cfg numbers_range: 전력카운트 반경(한타)
static NUMBERS_RANGE_MOVE: AtomicI64 = AtomicI64::new(-1); // cfg numbers_range_move: 전력카운트 반경(라인전 Move). -1=폴백(한타값)
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
    let selfe = dd7_slot128(sim, rd_u64(p5 + 0x818)?);   // 0.5.0: p5+0x6a0→0x818
    if !ptr_ok(selfe) || !readable(selfe + 0x650, 8) { return None; }
    Some((rd_u64(selfe + 0x648)?, rd_u64(selfe + 0x650)?))
}
unsafe fn count_nearby_champs(rh: usize, team: i64, sx: u64, sy: u64) -> Option<(u32, u32)> {
    if !ptr_ok(rh) || team < 0 || team > 1 { return None; }
    let q = team as usize;
    let r = apos_u(&NUMBERS_RANGE, "numbers_range"); let r2 = r.wrapping_mul(r);
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
// ★아군 포탑 전력 기여(HP·DPS 분리, 라인전/한타 분리): 반환 (thp, tatk) = (아군 ΣHP에 더할 포탑HP기여, 아군 Σ공격에 더할 포탑DPS기여).
//   thp = Σ포탑curHP × hp_w/100, tatk = 포탑수 × tower_dps × dps_w/100. base_code==2(라인워크)면 _move 가중치(-1=폴백→한타값). 포탑 슬롯=laner① 동일(고정 0x170~0x1d0 + Vec 0x130, team=q=아군).
unsafe fn ally_tower_contrib(rh: usize, team: i64, sx: u64, sy: u64, base_code: u8) -> (u128, u128) {
    if team < 0 || team > 1 { return (0, 0); }
    let pick = |base: i64, mv: i64| -> i64 { if base_code == 2 { if mv >= 0 { mv } else { base } } else { base } };
    let hp_w = pick(apos(&ALLY_TOWER_HP, "ally_tower_hp"), apos(&ALLY_TOWER_HP_MOVE, "ally_tower_hp_move")).clamp(0, 100) as u128;
    let dps_w = pick(apos(&ALLY_TOWER_DPS, "ally_tower_dps"), apos(&ALLY_TOWER_DPS_MOVE, "ally_tower_dps_move")).clamp(0, 100) as u128;
    if hp_w == 0 && dps_w == 0 { return (0, 0); }
    let q = team as usize;
    let r_base = apos_u(&ALLY_TOWER_RANGE, "ally_tower_range");
    let r = if base_code == 2 { let m = apos(&ALLY_TOWER_RANGE_MOVE, "ally_tower_range_move"); if m >= 0 { m as u64 } else { r_base } } else { r_base };   // ★포탑 인식범위 base_code별(라인전 _move, -1=한타값)
    let r2 = r.wrapping_mul(r);
    let dps = tune("tower_dps", 8000).max(0) as u128;   // 포탑 공격 추정(disc4 포탑딜과 동일 계수)
    let (mut sum_hp, mut cnt) = (0u128, 0u128);
    let mut acc = |t: usize| {
        if t < 0x10000 { return; }
        let hp = rd_u64(t + 0x658).unwrap_or(0); if hp == 0 { return; }
        let tx = rd_u64(t + 0x648).unwrap_or(0); let ty = rd_u64(t + 0x650).unwrap_or(0);
        let dx = if tx >= sx { tx - sx } else { sx - tx };
        let dy = if ty >= sy { ty - sy } else { sy - ty };
        if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) < r2 { sum_hp = sum_hp.wrapping_add(hp as u128); cnt += 1; }
    };
    for &off in &[0x170usize, 0x180, 0x190, 0x1a0, 0x1b0, 0x1c0, 0x1d0] { acc(rd_u64(rh + off + q*8).unwrap_or(0) as usize); }
    let vb = rd_u64(rh + 0x130 + q*0x20).unwrap_or(0) as usize;
    let vl = rd_u64(rh + 0x148 + q*0x20).unwrap_or(0);
    if ptr_ok(vb) && vl <= 32 { for i in 0..vl as usize { acc(rd_u64(vb + i*8).unwrap_or(0) as usize); } }
    let thp = sum_hp.wrapping_mul(hp_w) / 100;
    let tatk = cnt.wrapping_mul(dps).wrapping_mul(dps_w) / 100;
    ALLY_TOWER_THP.store(thp.min(u128::from(u64::MAX)) as u64, Ordering::Relaxed);    // 진단: HP기여 절대값
    ALLY_TOWER_TATK.store(tatk.min(u128::from(u64::MAX)) as u64, Ordering::Relaxed);  // 진단: DPS기여 절대값
    ALLY_TOWER_CNT.store(cnt as u64, Ordering::Relaxed);                              // 진단: 포탑 수
    (thp, tatk)
}
unsafe fn combat_balance(rh: usize, team: i64, sx: u64, sy: u64, base_code: u8) -> Option<(i64, u32, u32)> {
    if !ptr_ok(rh) || team < 0 || team > 1 { return None; }
    let q = team as usize;
    let r_base = apos_u(&NUMBERS_RANGE, "numbers_range");
    let r = if base_code == 2 { let m = apos(&NUMBERS_RANGE_MOVE, "numbers_range_move"); if m >= 0 { m as u64 } else { r_base } } else { r_base };   // ★전력카운트 반경 base_code별(라인전 _move, -1=폴백)
    let r2 = r.wrapping_mul(r);
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
    let (thp, tatk) = ally_tower_contrib(rh, team, sx, sy, base_code);   // ★아군 포탑 HP·DPS 분리 기여
    let (ahp, aatk) = (ahp.wrapping_add(thp), aatk.wrapping_add(tatk));   // 포탑HP→ΣHP, 포탑DPS→Σ공격
    let (ehp, eatk, en) = team_force(1 - q);
    let f_ally = ahp.wrapping_mul(aatk);
    let f_enemy = ehp.wrapping_mul(eatk);
    let w: i64 = if f_enemy == 0 { 9999 } else if f_ally == 0 { 0 }
                 else { (f_ally.wrapping_mul(100) / f_enemy).min(9999) as i64 };
    Some((w, an, en))
}
static D4_NUM_OVR_N: AtomicU64 = AtomicU64::new(0);   // 진단: disc4 인원수 후퇴 override 발동
// ★다이브게이트식 "교전중 챔피언 수": engage-list(cont+0xf0+team*0x20, count@+0x108) 중 로스터(챔피언)만 카운트.
//   (구 ttd_capture와 동일 구조였음.) 로스터-근접 폴백 아님 = 실제 교전중 적 챔프수(한타 규모 반영). team-wide.
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
        + &format!("     OUT5={} near(ally={} enemy={}) maxNearE={} D4_OVR={} D4_C8={} TOW_HIT={} TOW_MAX={} LRET={}(T{}F{}N{} W{}) forceRet[적≤1={} 적≥2={}] min_enemy={}\n", ENG_OUT5_N.load(Ordering::Relaxed), NUM_LASTCNT.load(Ordering::Relaxed)>>32, NUM_LASTCNT.load(Ordering::Relaxed)&0xffffffff, NUM_MAXENEMY.load(Ordering::Relaxed), D4_NUM_OVR_N.load(Ordering::Relaxed), D4_TTD_C8.load(Ordering::Relaxed), TOWER_HIT_N.load(Ordering::Relaxed), TOWER_HIT_MAX.load(Ordering::Relaxed), LANER_RET_N.load(Ordering::Relaxed), LANER_RET_TOW.load(Ordering::Relaxed), LANER_RET_FRC.load(Ordering::Relaxed), LANER_RET_NUM.load(Ordering::Relaxed), LANER_RET_W.load(Ordering::Relaxed), FRC_E1.load(Ordering::Relaxed), FRC_E2.load(Ordering::Relaxed), NUMBERS_MIN_ENEMY.load(Ordering::Relaxed));
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
    // ★dd7700 출력 code별 분포: 게임이 Move(2/라인워크) vs 교전·귀환(4/6/7) 낸 비율 + numbers가 각각 후퇴로 덮은 횟수. RET[Move]가 멀뚱멀뚱 주범 → move_thr=0이면 0으로 떨어져야.
    rost.push_str(&format!("     CODE seen[Move={} 교전={}] ret[Move={} 교전={}] move_thr={} | allyTower[hp+={} dps+={} 탑{}개](w_hp{}/w_dps{})\n", SEEN_C2.load(Ordering::Relaxed), SEEN_C67.load(Ordering::Relaxed), RET_C2.load(Ordering::Relaxed), RET_C67.load(Ordering::Relaxed), NUMBERS_THREAT_MOVE.load(Ordering::Relaxed), ALLY_TOWER_THP.load(Ordering::Relaxed), ALLY_TOWER_TATK.load(Ordering::Relaxed), ALLY_TOWER_CNT.load(Ordering::Relaxed), ALLY_TOWER_HP.load(Ordering::Relaxed), ALLY_TOWER_DPS.load(Ordering::Relaxed)));
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
#[inline(never)] unsafe fn my_engage_predict(p2: usize, p5: usize, p6: usize, p9: usize, self_e: usize) -> Option<(i64, i64, i64, i64)> {
    if !ptr_ok(p5) || !ptr_ok(p6) || !ptr_ok(self_e) { return None; }   // ★readable VQ제거(p6+8/self_e+0x138은 rd_u64?/미사용)
    let _posg = pos_enter_p56(p5, p6);   // ★포지션별 cfg: t_engage/eng_role* 포지션 응답
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
                (match rd_u64(obj + 0x188)? { 4 => tune("eng_role4", 100), 3 => tune("eng_role3", 70), 2 => tune("eng_role2", 50), _ => tune("eng_role_def", 30) }) * apos(&TUNE_ENGAGE_MULT, "t_engage") / 100
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
#[inline(never)] unsafe fn my_engage_emit(p2: usize, p5: usize, p6: usize, p9: usize, self_e: usize) -> Option<i64> {
    let _pg = perf_guard(5);
    if !ptr_ok(p5) || !ptr_ok(p6) || !ptr_ok(self_e) || !writable(self_e, 0x108) { PT_OTHER.fetch_add(1, Ordering::Relaxed); return None; }   // ★readable VQ제거(writable=RNG-sync 유지)
    let _posg = pos_enter_p56(p5, p6);   // ★포지션별 cfg: t_engage/eng_role* 포지션 응답
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
                (match rd_u64(obj + 0x188)? { 4 => tune("eng_role4", 100), 3 => tune("eng_role3", 70), 2 => tune("eng_role2", 50), _ => tune("eng_role_def", 30) }) * apos(&TUNE_ENGAGE_MULT, "t_engage") / 100
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
    let threshold = rd_u64(p3 + 0x888)?;   // 0.5.0: p3(SimState)+0x710→0x888(+0x178, e9probe 실측 1303/968). facetcnt(0x3d0)/vec(0x3c8)/jt(0x440)는 불변(e9probe: p5+0x3d0=0)
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


// ── fc59a0 base-score uVar21 + mult 완전재현 (disasm FUN_141d5b5d0) ──
// 빌더 f260f0(FUN_141d874b0 적위협=f6f720위치술어+cand_valid)/f26fd0(FUN_141d88200 아군오브젝트 geo술어+HP%>40).
// 반환 Some(mult)=게임 out[2]. None=계산불가. mult=(local_b0+1<=local_d0)?uVar21:uVar21+0x14. (RNG불필요, 진입시 결정론.)
const RECALL_MULT_NONE: i64 = -888888;
// 반환 (mult, u21f, b0, d0): u21f=mult분기 前 base, b0/d0=카운트 (mult 검증 로깅용).
// 반환 5번째=rng_drawn(true=full path 도달=RNG 1 u32 draw 소비; false=early-out=out{0,0,0} 무RNG).
// ★진단(fc59a0 u21 과대 원인추적): my_recall_mult 블록1 입력 raw 덤프. recallcap 로그서 출력.
static RC_D_NE: AtomicU64 = AtomicU64::new(0);
static RC_D_NEHP: AtomicI64 = AtomicI64::new(0);
static RC_D_NEMAX: AtomicI64 = AtomicI64::new(0);
static RC_D_EHP: AtomicI64 = AtomicI64::new(0);
static RC_D_NA: AtomicU64 = AtomicU64::new(0);
static RC_D_SELF: AtomicU64 = AtomicU64::new(0);
unsafe fn my_recall_mult(sim: usize, p4: usize, mode: u8) -> Option<(i64,i64,i64,i64,bool)> {
    let _pg = perf_guard(4);
    let _posg = pos_enter_ent(sim, sim);   // ★포지션별 cfg + 우리팀 게이트(sim=athlete struct B, athlete_id@+0x698). t_recall/rc_* 복귀 계수
    let team = rd_u64(sim + 0x810).unwrap_or(9);   // 0.5.0(was 0x6a8, SimState +0x178). athlete레코드 team  ★0.5.4 오프셋 이동 반영
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
    let enemy_base = l78 + (other as usize)*0x28 + 0x1e0;      // lane 컨테이너 stride 0x28 = 불변
    let lvar16 = (other as usize)*0x2e8 + geo;                 // 0.5.0(was 0x228, geom 컨테이너 stride +0xc0)
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
    let ally_base = l78 + (team as usize)*0x28 + 0x1e0;        // lane 컨테이너 stride 0x28 = 불변
    let pred_base = geo + (team as usize)*0x2e8;               // 0.5.0(was 0x228, geom 컨테이너 stride +0xc0)
    let ally_hp_min = tune("rc_ally_hp_min", 0x28);          // ★호이스트: 아군 유효 HP% 하한(루프불변)
    let mut allies = [0usize; 5]; let mut b0 = 0usize;       // ★Vec→스택배열(≤5, 힙할당 제거)
    for k in 0..5usize {
        if rd_u8(pred_base + 0xf8 + k*0x20) != 0 || rd_u8(pred_base + 0xf9 + k*0x20) != mode { continue; }
        let c = rd_u64(ally_base + k*8).unwrap_or(0) as usize;
        if c == 0 { continue; }
        let mx = rd_u64(c+0x610).unwrap_or(0); if mx == 0 { continue; }
        if (rd_u64(c+0x658).unwrap_or(0).wrapping_mul(100) / mx) as i64 > ally_hp_min { allies[b0]=c; b0+=1; }
    }
    let self_ref = dd7_slot128(self_obj, rd_u64(sim + 0x818).unwrap_or(0));   // 0.5.0(was 0x6a0, SimState self-entity 핸들 +0x178)
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
    RC_D_NE.store(ne as u64, Ordering::Relaxed); RC_D_NEHP.store(rd_u64(ne+0x658).unwrap_or(0) as i64, Ordering::Relaxed);
    RC_D_NEMAX.store(emx as i64, Ordering::Relaxed); RC_D_EHP.store(ehp, Ordering::Relaxed);
    RC_D_NA.store(na as u64, Ordering::Relaxed); RC_D_SELF.store(self_ref as u64, Ordering::Relaxed);
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
    let base_mult = if (b0 as i64) + 1 <= d0 as i64 { u21f } else { u21f + tune("rc_mult_bonus", 0x14) };
    // ★합류 이득 항(신규, rc_join_weight=0이면 비활성=기존 동작·검증 무영향). recall을 전략적 합류 이동기로.
    //   ★가산이 아니라 max: 체력기반(base_mult)와 합류기반(join_score) 중 강한 쪽만 채택.
    //   → "체력만으로 위험" OR "합류만으로 이득" 중 하나라도 임계 넘으면 복귀, 둘 다 애매하면 복귀 안 함(어정쩡 합산 제거).
    //   수적우위→승산 한타 합류 / 수적열세→열세 아군 구원, self↔합류대상 거리로 감가, 리콜포인트(거점/오브젝트)면 가중.
    // ★[08-03 원본 순수화] 합류 이득은 **게임 원본에 없는 모드 신규 판단** ⟹ 강제 0(비활성).
    //   재추가 시 `tune("rc_join_weight", 0)`으로 되돌리면 복구(아래 로직 그대로 보존).
    let join_w = 0i64;   // was: tune("rc_join_weight", 0)
    let mult = if join_w == 0 { base_mult } else {
        let sit = if (b0 as i64) + 1 > d0 as i64 {
            ((b0 as i64) + 1 - d0 as i64) * tune("rc_join_adv", 10)      // 승산 한타 합류
        } else {
            (d0 as i64 - (b0 as i64)) * tune("rc_join_rescue", 6)        // 열세 아군 구원
        };
        let jd = isqrt(sqd(srx, sry, rd_u64(na + 0x648).unwrap_or(0), rd_u64(na + 0x650).unwrap_or(0)));
        let distf = if jd < tune("rc_join_dnear", 80000) as u64 { 3 }
                    else if jd < tune("rc_join_dmid", 160000) as u64 { 2 } else { 1 };
        let objf = if rp != 0 { tune("rc_join_obj_mult", 2) } else { 1 };
        let join_score = sit * distf * objf * join_w / 10;             // 합류 이득만으로의 복귀 점수
        base_mult.max(join_score)                                      // ★둘 중 강한 쪽
    };
    Some((mult, u21f, b0 as i64, d0 as i64, true))   // full path → RNG 1 draw 소비
}
// ★fc59a0(recall) 완전대체: out[0]=score(=m*mult/100), out[4]=bool(p6<=score), out[8]=mult. early-out(무RNG)=out{0,0,0}. full=u32 gen_range writeback(m=100-uv7..100+uv7, uv7=(1000-A)/20, A=sim[0x218]). 실패/미지→None(passthrough).
#[inline(never)] unsafe fn my_fc59a0_full(out: usize, prng: usize, sim: usize, p4: usize, mode: u8, p6: i64) -> Option<()> {
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
    if a < 0 || a > tune("rc_rng_a_base", 1000) { return None; }            // 범위밖 = 미지(게임 clamp 불확실) → passthrough
    let uv7 = ((tune("rc_rng_a_base", 1000) - a) / tune("rc_rng_spread_div", 20).max(1)) as u64;
    let m = rng_advance_writeback_u32(prng, tune("rc_rng_center", 100) - uv7 as i64, 2*uv7 + 1)?;
    let score = (((m * mult) / tune("rc_score_div", 100).max(1)) + apos(&TUNE_RECALL_BIAS, "t_recall")) as i32;   // ★튜닝: recall score 가산(>0=자주복귀, 포지션별)
    std::ptr::write_unaligned(out as *mut i32, score);
    std::ptr::write_unaligned((out + 4) as *mut u8, if p6 <= score as i64 { 1u8 } else { 0u8 });
    std::ptr::write_unaligned((out + 8) as *mut i32, mult as i32);
    // ★[07-29 detlog ch3] recall 판정 기록(score·mult·draw결과). ch0/ch1 일치인데 여기만 갈리면 recall 재현이 비결정.
    if DL_ON.load(Ordering::Relaxed) { dl_rec(dl_world_tl(), 3, (score as u64) ^ (mult as u64).rotate_left(21) ^ (m as u64).rotate_left(37)); }
    Some(())
}

// ★fc59a0 recall RNG score 캡처: 진입시 A=sim[0x218]로 RNG배율 m 예측(read-only) + my_recall_mult(base score), 리턴훅 kind:5서 게임 출력(score/bool/mult)과 대조.
//   facet#5 retreat_engage → f28a50 → fc59a0 체인. score=(m*mult)/100. base score(uVar21)→mult 재현 완료.
unsafe extern "C" fn fc59a0_capture(saved: usize, entry_rsp: usize) -> i64 {
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return RAX_SENT; }
    let _jm = judge_mark(2);   // ★행진단: 하트비트+in-flight(recall)
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
    let pre = format!("[recall #{}] A={} thr={} my_m={} mode={} my_mult={} u21={} b0={} d0={} | DIAG ne=0x{:x} ne_hp={} ne_max={} ehp={} na=0x{:x} self=0x{:x}", RECALL_ARMED.load(Ordering::Relaxed), a, p6, my_m, mode, my_mult, my_u21, my_b0, my_d0, RC_D_NE.load(Ordering::Relaxed), RC_D_NEHP.load(Ordering::Relaxed), RC_D_NEMAX.load(Ordering::Relaxed), RC_D_EHP.load(Ordering::Relaxed), RC_D_NA.load(Ordering::Relaxed), RC_D_SELF.load(Ordering::Relaxed));
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

// ── generic_build 스코어러 재현 모듈 (my_f80320/F80Ctx) ──
include!("genbuild_repro.rs");

// ── facet#1 condgate 재현: subplan별 목표커밋 bool. 리프 vtable=섀도우호출(getter, 부작용無 추정). -99=미재현(poke/gank-else).
type VtPtrFn = unsafe extern "C" fn(usize) -> usize;          // rvt[0x20]timing / rvt[0x168]ctx (1 arg)
type VtPtr2Fn = unsafe extern "C" fn(usize, usize) -> usize;  // rvt[0x128]deref / rvt[0x140]check (2 arg)
unsafe fn vt_slot(rvt: usize, off: usize) -> usize { rd_u64(rvt+off).unwrap_or(0) as usize }   // ★readable VQ→rd_u64(모든 vtable 조회, per-frame 최고빈도). fault시 0=동일
// poke 후보 유효성(FUN_141d880e0/gather 공용): c48!=0 || (a8!=0 && rvt[0x20]timing <= *(lvar16+0x1e0+a8[0x738]*8)+0x78)
unsafe fn cand_valid(robj: usize, _rvt: usize, team: u64, lvar16: usize, cand: usize) -> Option<bool> {
    let uv8 = rd_u64(cand + 0x5a8)? as usize;               // entity char_id = 불변
    if geom_vt68(robj, team as usize, uv8 as u64) { return Some(true); }   // vt0x68 now-visible → 순수재현(shadow-call 제거=AV방지)
    let a8 = geom_vtc0(robj, uv8 as u64);                   // vt0xc0 시야레코드 → 순수재현
    if a8 == 0 { return Some(false); }
    let idx = rd_u32(a8 + 0x8a0) as usize;                  // 0.5.0(was 0x738, a8=SimState계열 +0x178)  ★0.5.4 오프셋 이동 반영
    let lv = rd_u64(lvar16 + 0x1e0 + idx*8)?;               // geom +0x1e0/idx*8 = 불변
    let timing = rd_i64(robj + 0xeb00).unwrap_or(0) as u64; // vt0x28 tick → 순수재현
    Some(timing <= lv + 0x78)
}

// condgate poke A/B(disc12=A FUN_14235bfb0 / disc14=B FUN_141c87c20) goal-commit bool. RNG-free.
//   early(flags+0x3ea: A!=0/B!=1→true)·gather(flags+0x3eb==1 근접적 150000², ⬜best-effort 생략)·TAIL 타이밍(scale*15<slack).
//   flags=r11(param6)·gc=rdx(param2)·side=*(r9+0x820)·agent=rh_slot([0]={obj,vt}·[1]=marker·[2]=pool). tail off: A=+0x1a8/0x1a0/0x1b0, B=+0x1d8/0x1d0/0x1e0.
unsafe fn condgate_poke_bool(flags: usize, gc: usize, r9: usize, rh_slot: usize, is_b: bool) -> i64 {
    if !ptr_ok(flags) || !ptr_ok(gc) { return -99; }
    let early = rd_u8(flags + 0x3ea);
    if (!is_b && early != 0) || (is_b && early != 1) { return 1; }   // early → 커밋(true)
    let agent0 = rd_u64(rh_slot).unwrap_or(0) as usize;
    if !ptr_ok(agent0) { return -99; }
    let obj = rd_u64(agent0).unwrap_or(0) as usize;
    let vt = rd_u64(agent0 + 8).unwrap_or(0) as usize;
    if !ptr_ok(obj) || !ptr_ok(vt) { return -99; }
    // ── gather(flags+0x3eb==1): 오더타겟 반경 최근접 적. 멀면(>150000²) or 無 → 커밋(true), 가까우면 TAIL ──
    let side = rd_u64(r9 + 0x810).unwrap_or(2) as usize;//  ★0.5.4 오프셋 이동 반영
    if rd_u8(flags + 0x3eb) == 1 && side < 2 {
        let agent1 = rd_u64(rh_slot + 8).unwrap_or(0) as usize;
        let agent2 = rd_u64(rh_slot + 0x10).unwrap_or(0) as usize;
        let filter = if is_b { 5u8 } else { 4u8 };
        // 마커룩업: *(agent1+0x20) 컨테이너(base@+0x68/cnt@+0x70 stride0x28), +0x20==filter → side별 좌표
        let (mut tx, mut ty) = (0u64, 0u64);
        let cont = rd_u64(agent1 + 0x20).unwrap_or(0) as usize;
        if ptr_ok(cont) {
            let mbase = rd_u64(cont + 0x68).unwrap_or(0) as usize;
            let mcnt = rd_u64(cont + 0x70).unwrap_or(0) as usize;
            for i in 0..mcnt.min(64) {
                if rd_u8(mbase + i * 0x28 + 0x20) == filter {
                    let so = if side != 0 { 0x10 } else { 0 };
                    tx = rd_u64(mbase + i * 0x28 + so).unwrap_or(0);
                    ty = rd_u64(mbase + i * 0x28 + so + 8).unwrap_or(0);
                    break;
                }
            }
        }
        // vt0x70 유효존 게이트: true면 적 gather. 순수재현(shadow-call 제거=AV방지). ⬜FUN_141fddab0 sub-check 생략(best-effort)
        if geom_vt70(obj, side, (tx / 32000) as usize, (ty / 32000) as usize) {
            let eside = 1 - side;
            let (mut best, mut bestd) = (0usize, u64::MAX);
            for k in 0..5usize {
                let e = rd_u64(agent0 + 0x1e0 + eside * 0x28 + k * 8).unwrap_or(0) as usize;
                if e == 0 { continue; }
                let id = rd_u64(e + 0x5a8).unwrap_or(0) as usize;
                let pass = if geom_vt68(obj, side, id as u64) { true } else {   // vt0x68 now-visible → 순수재현
                    let ro = geom_vtc0(obj, id as u64);                          // vt0xc0 시야레코드 → 순수재현
                    if ro == 0 { false } else {
                        let sv = rd_u64(agent2 + eside * 0x2e8 + 0x1e0 + rd_u32(ro + 0x8a0) as usize * 8).unwrap_or(0);//  ★0.5.4 오프셋 이동 반영
                        !(sv.wrapping_add(0x78) < rd_i64(obj + 0xeb00).unwrap_or(0) as u64)   // vt0x28 tick reach 도달 → 순수재현
                    }
                };
                if !pass { continue; }
                let d = sqd(tx, ty, rd_u64(e + 0x648).unwrap_or(0), rd_u64(e + 0x650).unwrap_or(0));
                if d < bestd { bestd = d; best = e; }
            }
            if best == 0 || bestd > 0x53d1ac101 { return 1; }   // qualifying 적 無 or 최근접>150000² → 커밋
            // else 가까운 적 있음 → TAIL
        }
    }
    // ── TAIL 타이밍 ──
    let (len_off, harr_off, tick_off) = if is_b { (0x1d8usize, 0x1d0usize, 0x1e0usize) } else { (0x1a8usize, 0x1a0usize, 0x1b0usize) };
    // 진행중 라인오더 → 미커밋. vt0x150(obj, *(*(gc+harr_off))) → 순수재현(geom_resolve150; ★Ghidra 확증: rdx=핸들배열 첫원소, len가드=gc+len_off).
    if rd_u64(gc + len_off).unwrap_or(0) != 0 {
        let harr = rd_u64(gc + harr_off).unwrap_or(0) as usize;
        if ptr_ok(harr) && geom_resolve150(obj, rd_u64(harr).unwrap_or(0)) != 0 { return 0; }
    }
    let base_tick = rd_u64(gc + tick_off).unwrap_or(0);
    let cur = rd_i64(obj + 0xeb00).unwrap_or(0) as u64;   // vt0x28 tick → 순수재현
    let slack = if cur <= base_tick { base_tick - cur } else { 0 };
    let agent1 = rd_u64(rh_slot + 8).unwrap_or(0) as usize;
    let a1p = rd_u64(agent1 + 8).unwrap_or(0) as usize;
    let scale = rd_i64(a1p + 0x12f8).unwrap_or(0).wrapping_mul(15);
    if (scale as u64) < slack { 1 } else { 0 }   // 잔여시간 충분 → 커밋
}
// ════ condgate 0.5.0_3 재작성(FUN_210d780 병합/재번호). disc=*ctx(0.5.0 raw). rdx=param2(battle gc). RNG-free. ════
//   disc0/1/3/4/7/16/17→0·2→1·5/9→ctx+0x60==7·6→ctx+0x70==7·8=recall·10=ganker·11=cover·13/15=battle(rdx)·12/14=poke(-99 passthrough).
#[inline(never)] unsafe fn my_condgate_050(ctx: usize, rdx: usize, r9: usize, rh_slot: usize, r11: usize) -> i64 {
    if !ptr_ok(ctx) { return -99; }
    let disc = match rd_u64(ctx) { Some(v) => v, None => return -99 };
    match disc {
        0 | 1 | 3 | 4 | 7 | 16 | 17 => return 0,
        2 => return 1,
        5 | 9 => return if rd_u32(ctx + 0x60) == 7 { 1 } else { 0 },   // data-var (구 ctx+0x58)
        6 => return if rd_u32(ctx + 0x70) == 7 { 1 } else { 0 },
        _ => {}
    }
    let p = rd_u64(rh_slot).unwrap_or(0) as usize;   // agent {obj,vt}
    if !ptr_ok(p) { return -99; }
    let robj = rd_u64(p).unwrap_or(0) as usize;
    let rvt = rd_u64(p + 8).unwrap_or(0) as usize;
    if !ptr_ok(robj) || !ptr_ok(rvt) { return -99; }
    match disc {
        8 => {  // ActiveRecall: ent=vt0x138(obj,*(r9+0x818)); ent.cur>=ent.max
            if !ptr_ok(r9) { return -99; }
            let ent = dd7_slot128(robj, rd_u64(r9 + 0x818).unwrap_or(0));   // vt0x138 2단 resolve → 순수재현(shadow-call 제거=AV방지)
            if !ptr_ok(ent) { return -99; }
            let cur = match rd_u64(ent + 0x658) { Some(v) => v, None => return -99 };
            let max = match rd_u64(ent + 0x610) { Some(v) => v, None => return -99 };
            if cur >= max { 1 } else { 0 }
        }
        11 => {  // Cover: vt0x28(obj)tick >= ctx+0x20 → 순수재현(shadow-call 제거=AV방지)
            if (rd_i64(robj + 0xeb00).unwrap_or(0) as u64) >= rd_u64(ctx + 0x20).unwrap_or(u64::MAX) { 1 } else { 0 }
        }
        10 => {  // LineGanker: vt0x28(obj)tick → 순수재현
            if (rd_i64(robj + 0xeb00).unwrap_or(0) as u64) >= rd_u64(ctx + 0x28).unwrap_or(u64::MAX) { return 1; }
            match rd_u8(ctx + 0x31) {
                6 => if (rd_i64(robj + 0xeb00).unwrap_or(0) as u64) >= rd_u64(ctx + 0x20).unwrap_or(u64::MAX) { 1 } else { 0 },
                8 => 1,
                _ => 0,
            }
        }
        13 => {  // Battle A: rdx+0x1a8==0→1; vt0x150(obj,*(*(rdx+0x1a0)))==0. (vt0x30 panic게이트=None가드 생략)
            if !ptr_ok(rdx) { return -99; }
            if rd_u64(rdx + 0x1a8).unwrap_or(0) == 0 { return 1; }
            let pp = rd_u64(rdx + 0x1a0).unwrap_or(0) as usize;
            if !ptr_ok(pp) { return -99; }
            if geom_resolve150(robj, rd_u64(pp).unwrap_or(0)) == 0 { 1 } else { 0 }   // vt0x150 → 순수재현(shadow-call 제거=AV방지)
        }
        15 => {  // Battle B: rdx+0x1d8/0x1d0
            if !ptr_ok(rdx) { return -99; }
            if rd_u64(rdx + 0x1d8).unwrap_or(0) == 0 { return 1; }
            let pp = rd_u64(rdx + 0x1d0).unwrap_or(0) as usize;
            if !ptr_ok(pp) { return -99; }
            if geom_resolve150(robj, rd_u64(pp).unwrap_or(0)) == 0 { 1 } else { 0 }   // vt0x150 → 순수재현(shadow-call 제거=AV방지)
        }
        12 => condgate_poke_bool(r11, rdx, r9, rh_slot, false),   // Poke A(EpicHuntPoke 계열)
        14 => condgate_poke_bool(r11, rdx, r9, rh_slot, true),    // Poke B(SerpenHuntPoke 계열)
        _ => -99,
    }
}
// ★facet#1 condgate 캡처: 진입시 my_condgate 계산 → 리턴훅 kind:6서 게임 al(retval&0xff)과 대조.
unsafe extern "C" fn condgate_capture(saved: usize, entry_rsp: usize) -> i64 {
    // ★07-12 panic-safe: replace_rax 훅 본문 catch_unwind 래핑(§1 일관성, retreat/mp/fc59a0와 동일). 패닉시 RAX_SENT passthrough.
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> i64 {
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return RAX_SENT; }
    let _jm = judge_mark(3);   // ★행진단: 하트비트+in-flight(condgate)
    // ★완전대체(cond_repl): RNG-free judge → sync 불필요. my_condgate(≠-99 확신케이스)로 게임출력 대체(원본 skip).
    //   -99(poke/gank 미재현)는 passthrough(게임 원본). my=al값(0..255) → rax 저바이트=al → 게임이 우리 커밋값 사용.
    if COND_REPL.load(Ordering::Relaxed) {
        let ctx = rd_u64(saved + 0x28).unwrap_or(0) as usize;
        if ptr_ok(ctx) && readable(ctx, 8) {
            let r9 = rd_u64(saved + 0x10).unwrap_or(0) as usize;
            let rh_slot = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;
            let r11c = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize;
            TD_RET.store(i64::MIN, Ordering::Relaxed);
            let rdx_c = rd_u64(saved + 0x20).unwrap_or(0) as usize;
            let my = my_condgate_050(ctx, rdx_c, r9, rh_slot, r11c);
            if my != -99 {
                let n = COND_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                if n == 1 || n % 300 == 0 {
                    let disc = rd_u64(ctx).unwrap_or(0);
                    let pass = COND_REPL_PASS.load(Ordering::Relaxed);
                    if !COND_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("condcmp.txt", "=== facet#1 condgate ④ 완전대체(cond_repl=1) ===\n"); }
                    append_named("condcmp.txt", &format!("[cond REPL #{}] disc={} my={} (대체) | passthrough누적={}\n", n, disc, my & 0xff, pass));
                }
                // ★[07-29 detlog ch2] cond 판정 기록 — 최대 판정면(경기당 3만+ 호출)인데 그간 **미계측**이었다.
                //   ch0/ch1(상태·RNG) 일치인데 이 채널만 갈리면 = condgate 재현이 비결정 = 범인.
                if DL_ON.load(Ordering::Relaxed) { dl_rec(dl_world_tl(), 2, (my as u64) ^ rd_u64(ctx).unwrap_or(0).rotate_left(11)); }
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
    let rdx_c = rd_u64(saved + 0x20).unwrap_or(0) as usize;
    let my = my_condgate_050(ctx, rdx_c, r9, rh_slot, r11c);
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
    })).unwrap_or(RAX_SENT)
}

// ── DefenseNexus(disc=14) movepri judge = FUN_142068670 충실 재현. 출력 {7, 18}. ──
// vt+0x140 라우트노드 핸들→엔티티 리졸버(섀도우-CALL 안전, dd7700과 동일 패턴).
#[inline] unsafe fn def_resolve(sim: usize, _vobj: usize, handle: u64) -> usize {
    // ★07-12 shadow-call 순수화: vt0x150(=게임 0x1420ad690)=geom_resolve150 바이트동일 등치확정(6 vtable사본, capstone+ghidra). vobj 드롭, stale-candidate AV 근절.
    geom_resolve150(sim, handle)
}

// ════ EpicBattle(disc10) 0.5.0 FUN_1422d3950 완전재작성(REWRITE). 순수 7-엔트리 가중치 테이블, RNG-free. ════
//   mode=byte(subp+0x30)=rd_u8(p2+0x28)(p2=subp+8) · side=*(p5+0x820) · gs=*(p6)(버킷@+0x2170,{obj,vtbl}) · ctx=*(p6+8).
//   idx = gs.word[0x2170 + (msel*2+side)*8] (0..6). 반환 i64 weight (emit서 out+0=0xb/+8=weight/+0x10=1/+0x12=0). -99=passthrough.
#[inline] unsafe fn my_epic_battle(p2: usize, p5: usize, p6: usize, _p7: usize) -> i64 {
    let gs = rd_u64(p6).unwrap_or(0) as usize;            // arg4: 버킷 인덱스 + {obj,vtbl}
    if !ptr_ok(gs) { return -99; }
    let mode = rd_u8(p2 + 0x28);                           // arg1 = byte(subp+0x30)
    let side = rd_u64(p5 + 0x810).unwrap_or(2);            // arg3  ★0.5.4 오프셋 이동 반영
    if side > 1 { return -99; }
    let msel: usize = match mode { 0 => 0, 1 => 1, _ => 2 };
    let idx = rd_u64(gs + 0x2170 + (msel * 2 + side as usize) * 8).unwrap_or(99) as usize;
    if idx > 6 { return -99; }
    let tbl: [i64; 7] = match (msel, side) {
        (0, 0) => [16, 6, 3, 3, 3, 2, 2],
        (0, 1) => [2, 3, 6, 6, 6, 16, 16],
        (2, 0) => [0x15, 0x14, 0xf, 0xf, 0xf, 9, 7],
        (2, 1) => [9, 0xf, 0x14, 0x14, 0x14, 0x15, 0x17],
        (1, _) => {   // 위치의존 동적 테이블: cond = ((맵경계 - self_y) < self_x)
            let obj = rd_u64(gs).unwrap_or(0) as usize;          // gs[0]
            let vt = rd_u64(gs + 8).unwrap_or(0) as usize;       // gs[1]
            if !ptr_ok(obj) || !ptr_ok(vt) { return -99; }
            // ⛔★★[07-23] **살아있던 위험 shadow-call 제거**(disc10 pending 191건 = mode==1 호출 전량의 원인).
            //   ~~`vt_call1(vt, 0x138, obj)`~~ 문제 2중: ①슬롯 stale(0.5.1부터 0x50 이상 **+0x68** ⟹ `0x138 → 0x1a0`;
            //   원본 `0x2391ef7 call qword ptr [rax+0x1a0]`로 확증) ②원본은 **2인자 리졸버**(`rcx=g0[0]`, `rdx=sim[0x818]`=self handle)인데
            //   `vt_call1`은 1인자라 **rdx가 미정의** 상태로 호출됐다. ⟹ 엉뚱한 슬롯을 쓰레기 인자로 호출 = `RVA_C8C_DMG_SHEET` stale AV와
            //   **동일 클래스의 잠복 크래시원**(지금까지는 `!ptr_ok` 필터에 걸려 조용했을 뿐, 그래서 mode1이 전량 -99로 빠졌다).
            //   ⟹ **`dd7_slot128` 순수재현으로 교체**(vt[0x1a0]의 비트동일 재현, disc14에서 검증 통과 중) = shadow-call 자체가 소멸.
            let e = dd7_slot128(obj, rd_u64(p5 + 0x818).unwrap_or(0));
            if !ptr_ok(e) { return -99; }
            let _ = vt;   // (구 shadow-call 인자 — 순수재현 전환으로 미사용)
            let ctx = rd_u64(p6 + 8).unwrap_or(0) as usize;      // arg5
            let ctx1 = rd_u64(ctx + 8).unwrap_or(0) as usize;
            if !ptr_ok(ctx1) { return -99; }
            let mapb = rd_u64(ctx1 + 0x12c0).unwrap_or(0);
            let ey = rd_u64(e + 0x650).unwrap_or(0);
            let ex = rd_u64(e + 0x648).unwrap_or(0);
            let cond: i64 = if mapb.wrapping_sub(ey) < ex { 1 } else { 0 };
            if side == 0 {
                [cond * 4 + 0x11, cond * 3 + 0xb, cond * 3 + 0xb, cond * 3 + 0xb, cond * 3 + 0xb, cond * 5 + 4, cond * 5 + 4]
            } else {
                [cond * 5 + 4, cond * 3 + 0xb, cond * 3 + 0xb, cond * 3 + 0xb, cond * 3 + 0xb, cond * 4 + 0x11, cond * 4 + 0x11]
            }
        }
        _ => return -99,
    };
    tbl[idx]
}
// ════════════════════════════════════════════════════════════════════════════
// SerpenBattle(disc12) 콜리 재구현 리프 — §12.16, bottom-up. 전부 RNG-free.
// ════════════════════════════════════════════════════════════════════════════
// SerpenBattle 데이터 테이블 (0.5.0_3 확정, 전부 i64×18, ghidra content-search). 인덱스=role(desc/plan+0x38), 0..8 유효.
//   ① engage REQUIRED (abs 0x1438688b8): 게이트통과 = count >= SERPEN_REQUIRED[role]
const SERPEN_REQUIRED: [i64; 18] = [2, 2, 1, 2, 1, 2, 1, 2, 2, 3, 1, 3, 1, 3, 2, 3, 3, 3];
//   ③ laneBias (abs 0x1438ad828, i64×5): 슬롯별 거리 bias(FUN_14229a970 최근접 교전적 선택)
const SERPEN_LANE_BIAS: [i64; 5] = [60000, 0, 40000, 60000, 20000];
// ════ FUN_141c8c520 (RVA 0x1c8c520) 다이버 후보 선별기 재현(07-10 정밀RE 2패스, RNG-free) — disc14 threat B-경로 ════
//   시그(게임): (out{tag,vec}, level, ss, geom, self_ent). 반환 tag: 0=다이버 확보(콜러 cmd+0x1a=1·vec복제·7) / 1=후보0 / 5=자기 구조물 보호권 / 4=위험후보 미푸시.
//   검증경로는 tag만 사용(divers 등록은 라이브 대체시). 헬퍼 재사용: c932c0≈serpen_pred_b, 데미지코어≈serpen_dmg_core, fbe670≈serpen_skill_type3, base쌍≈probe_basedmg_r9.
const C8C_ROLE_OFF: [usize; 8] = [0x13f8, 0x1400, 0x13f8, 0x1400, 0x13f8, 0x1408, 0x13f8, 0x13f8];   // DAT_143812f70 모드별 스칼라 오프셋표

// ⚠[보존] disc4 AV근본해결 A·B 07-12 배포·라이브검증대기(CURRENT.md), dead지만 미검증 pending — 삭제금지·재활성대기
// ════ disc4 (LineSafe) 0.5.0_3 핸들러 FUN_1420f8df0→0x141c6f260 재구현(out-writer). 출력{2,4,6,7}. ════
//   시그: (out, mem=subp+8, param3=phase/count, athlete=param5, geom=param6, ctx=param7/tp). RNG=candidate count(간접).
//   ⬜v1 스켈레톤: abort(7)·근접교전(4/7)·수렴기본(2) 확정. 메인분기③·commit candidate(6/7 RNG)=TODO(검증재개시 수렴).
// FUN_1422a7910 최근접(argmin) 선정: 배열[start..end) 엔티티 중 앵커(laneType별 DAT)에 최근접 slot 반환. team0=축스왑. 07-10 순수RE.
const D4_ANCHOR_X: [i64; 3] = [820000, 817000, 880000];   // DAT_1438ad8f0 / DAT_143812fb8
const D4_ANCHOR_Y: [i64; 3] = [80000, 144000, 144000];    // DAT_1438ad908 / DAT_143812fd0
unsafe fn disc4_argmin(start: usize, end: usize, lanetype_obj: usize, team: u64) -> usize {
    if start == 0 || end <= start { return 0; }
    let t = (rd_u8(lanetype_obj + 0x19) as usize).min(2);
    let (ax, ay) = (D4_ANCHOR_X[t], D4_ANCHOR_Y[t]);
    let mut best = rd_u64(start).unwrap_or(0) as usize;
    let mut bestd = u64::MAX;
    let mut p = start;
    while p < end {
        let e = rd_u64(p).unwrap_or(0) as usize;
        if e != 0 {
            let (ex, ey) = (rd_i64(e + 0x648).unwrap_or(0), rd_i64(e + 0x650).unwrap_or(0));
            // team1: (|ex-ax|,|ey-ay|) / team0: 축스왑 (|ex-ay|,|ey-ax|)
            let d = if team == 1 { idist2(ex, ey, ax, ay) } else { idist2(ex, ey, ay, ax) };
            if d < bestd { bestd = d; best = e; }
        }
        p += 8;
    }
    best
}
// ★07-12 disc4 ward fallback argmin(FUN_1422a7910 실물): element[0]=seed(line색인)·[1..]=mode색인, candptr+8부터 순회. 최근접 슬롯의 엔티티 반환.
unsafe fn disc4_lead_argmin(candptr: usize, candlen: usize, line: usize, mode: usize, side: u64) -> usize {
    if candptr == 0 || candlen == 0 { return 0; }
    let dist = |e: usize, idx: usize| -> u64 {
        if e == 0 { return u64::MAX; }                          // 게임은 널체크 없이 deref(밀집배열 전제); 우린 안전가드
        let t = idx.min(2);
        let (ax, ay) = (D4_ANCHOR_X[t], D4_ANCHOR_Y[t]);
        let (ex, ey) = (rd_i64(e + 0x648).unwrap_or(0), rd_i64(e + 0x650).unwrap_or(0));
        if side == 1 { idist2(ex, ey, ax, ay) } else { idist2(ex, ey, ay, ax) }   // side!=1 축스왑
    };
    let mut best_slot = candptr;                                // element[0] 슬롯(seed)
    let mut best_dist = dist(rd_u64(candptr).unwrap_or(0) as usize, line);   // seed=line 색인
    let mut k = 1usize;
    while k < candlen {                                         // element[1..] = mode 색인
        let slot = candptr + k * 8;
        let d = dist(rd_u64(slot).unwrap_or(0) as usize, mode);
        if d < best_dist { best_dist = d; best_slot = slot; }
        k += 1;
    }
    rd_u64(best_slot).unwrap_or(0) as usize                     // *best_slot = 엔티티
}
#[inline(never)] unsafe fn my_disc4_050(out: usize, mem: usize, param3: i64, athlete: usize, geom: usize, ctx: usize) -> i64 {
    if !ptr_ok(out) || !ptr_ok(mem) || !ptr_ok(athlete) || !ptr_ok(geom) || !ptr_ok(ctx) { return -99; }
    // ① mem+0x18 abort → code7
    if rd_u8(mem + 0x18) != 0 { wr_u64(out, 7); return 7; }
    let g0 = rd_u64(geom).unwrap_or(0) as usize;          // geom0(objective 슬롯배열)
    if !ptr_ok(g0) { return -99; }
    let gchild = rd_u64(g0).unwrap_or(0) as usize;
    let gvt = rd_u64(g0 + 8).unwrap_or(0) as usize;
    if !ptr_ok(gchild) || !ptr_ok(gvt) { return -99; }
    let side = rd_u64(athlete + 0x810).unwrap_or(2);//  ★0.5.4 오프셋 이동 반영
    if side > 1 { return -99; }
    let role = rd_u32(athlete + 0x8a0);//  ★0.5.4 오프셋 이동 반영
    let b23 = rd_u8(ctx + 0x3ea);                          // 오더타입
    let mode = rd_u8(mem + 0x19);                          // 0/1/2
    // ★07-10 전면 재작성(§①~⑥ 완전RE, vt순수화). ★표본 미발화=검증불가, disasm 정적완결. 일부 콜리 인자매핑은 최선(★주석).
    let geom2 = rd_u64(geom + 0x10).unwrap_or(0) as usize;   // gb(레인그리드)
    let plan = rd_u64(geom + 8).unwrap_or(0) as usize;       // geom[1]
    if !ptr_ok(geom2) || !ptr_ok(plan) { return -99; }
    let lane = rd_u32(athlete + 0x8a0) as usize;             // athlete lane index  ★0.5.4 오프셋 이동 반영
    let selfid = rd_u64(athlete + 0x818).unwrap_or(0) as usize;
    let kind = disc4_vt30_kind(gvt);                          // vt0x30 kind(순수)
    let comp = gchild + 0xeb30;                               // vt0x30 comp(순수: 주소, deref 아님)
    // ── ① 근접교전(count>4 & (t3ea&0xfe)==8→tp+0x3eb==mode시 skip & mode==2 & 프론티어 & lane>2 & objective bbox) ──
    let skip1 = param3 > 4 && (b23 & 0xfe) == 8 && rd_u8(ctx + 0x3eb) == mode;
    if !skip1 && param3 > 4 && mode == 2 {
        // 프론티어 게이트(disc16 동종: plan+0x38∈{0,5,7,8}(0x1a1)이면 (plan+0x8a8)-30*(plan+0x12f8) <= vt28 → skip). ★state=plan 최선매핑.
        let fb = rd_u8(plan + 0x38);
        let frontier_bail = fb <= 8 && (0x1a1u32 >> fb) & 1 == 1 && {
            // ★07-12 정정: prog만 중간 deref *(plan+8)=cfgroot 경유(disasm 0x141c6f328~366). fb·bbox는 plan 직접(정확).
            let cfgroot = rd_u64(plan + 8).unwrap_or(0) as usize;
            let prog = rd_i64(cfgroot + 0x8a8).unwrap_or(0) - 30 * rd_i64(cfgroot + 0x12f8).unwrap_or(0);
            prog <= rd_i64(gchild + 0xeb00).unwrap_or(0)
        };
        if !frontier_bail && (lane as i64) >= tune("dd_cover_role_min", 3) {
            let s = side as usize;
            let oidx = 0x1f8 + s * 0x28 + if lane == 3 { 8 } else { 0 };
            let obj = rd_u64(g0 + oidx).unwrap_or(0) as usize;
            let box_ok = obj == 0 || (rd_i32(obj + 0x68).unwrap_or(0) != 0xd || rd_i32(obj + 0x70).unwrap_or(0) != 1) || {
                // bbox: *(geom[1][4]+0x6d70..)+side*0x20 (geom[1][4]=*(plan+0x20))
                let m = rd_u64(plan + 0x20).unwrap_or(0) as usize + s * 0x20;
                let (ox, oy) = (rd_u64(obj + 0x648).unwrap_or(0), rd_u64(obj + 0x650).unwrap_or(0));
                ox >= rd_u64(m + 0x6d70).unwrap_or(u64::MAX) && ox <= rd_u64(m + 0x6d80).unwrap_or(0)
                    && oy >= rd_u64(m + 0x6d78).unwrap_or(u64::MAX) && oy <= rd_u64(m + 0x6d88).unwrap_or(0)
            };
            if box_ok {
                let enemy = 1 - s;
                let mut cnt = 0i64;
                for i in 0..5usize {
                    let e = rd_u64(g0 + 0x1e0 + enemy * 0x28 + i * 8).unwrap_or(0) as usize;
                    if e == 0 { continue; }
                    if !dd7_f6f720_m2(plan, rd_u64(e + 0x648).unwrap_or(0), rd_u64(e + 0x650).unwrap_or(0)) { continue; }   // ★FUN_14236db90 레인게이트=dd7_f6f720_m2(동일상수), vobj=plan 최선
                    let id = rd_u64(e + 0x5a8).unwrap_or(0);
                    let team = geom_vt68(gchild, s, id) as i64;
                    if team != 0 { cnt += 1; }
                    else {
                        let le = geom_vtc0(gchild, id);
                        if le != 0 {
                            let rl = rd_u32(le + 0x8a0) as usize;//  ★0.5.4 오프셋 이동 반영
                            if rd_i64(gchild + 0xeb00).unwrap_or(0) <= rd_i64(geom2 + enemy * 0x2e8 + 0x1e0 + rl * 8).unwrap_or(0) + 0x78 { cnt += 1; }
                        }
                    }
                }
                if cnt >= tune("d4_threat_min", 2) {
                    let selfe = dd7_slot128(gchild, selfid as u64);
                    let (mx, cu) = if ptr_ok(selfe) { (rd_i64(selfe + 0x610).unwrap_or(1).max(1), rd_i64(selfe + 0x658).unwrap_or(0)) } else { (1, 0) };
                    let hp_pct = cu.wrapping_mul(100) / mx;
                    let code: i64 = if hp_pct >= tune("d4_close_hp", 51) { 4 } else if rd_i64(geom2 + s * 0x2e8 + 0x60).unwrap_or(0) >= 1000 { 7 } else { 4 };
                    wr_u64(out, code as u64); wr_u8(out + 8, 2); return code;
                }
            }
        }
    }
    // ── ②③ 메인/특수경로 → line/subcode ──
    let self_ent = dd7_slot128(gchild, selfid as u64);
    let bstat = (role == 1) as u8;
    if !ptr_ok(self_ent) { wr_u64(out, 2); wr_u8(out + 8, bstat); return 2; }
    let (sx, sy) = (rd_u64(self_ent + 0x648).unwrap_or(0), rd_u64(self_ent + 0x650).unwrap_or(0));
    let s = side as usize;
    let t3eb = rd_u8(ctx + 0x3eb) as usize;
    let special = b23 == 8 && t3eb == mode as usize;
    let (line, subcode): (usize, u8) = if special {
        // B 특수경로: ref=obj0[own*5+0x3d]=*(g0+own*0x28+0x1e8). ref==0→subcode=(gb[own,t3eb]+0x10<0x7d1). ref≠0→d2/cell/team 조합.
        let refp = rd_u64(g0 + s * 0x28 + 0x1e8).unwrap_or(0) as usize;
        let sc = if refp == 0 {
            (rd_i64(geom2 + s * 0x2e8 + 0x10).unwrap_or(0) < 0x7d1) as u8   // ★line offset 미세: t3eb 라인
        } else {
            let d2 = idist2(sx as i64, sy as i64, rd_i64(refp + 0x648).unwrap_or(0), rd_i64(refp + 0x650).unwrap_or(0));
            let team = geom_vt68(gchild, 1 - s, rd_u64(refp + 0x5a8).unwrap_or(0)) as i64;
            if d2 > tune("d4_ref_dist2", 0x9502F9000) as u64 || team != 0 { (rd_i64(geom2 + s * 0x2e8 + 0x10).unwrap_or(0) < 0x7d1) as u8 } else { 1 }
        };
        (t3eb, sc)
    } else {
        // A 메인경로: candidate=적5슬롯 non-null 첫 교전적. subcode=(gb[own,line]+0x18<0?2:0), 없으면 2.
        let enemy = 1 - s;
        let mut engaged = false;
        for i in 0..5usize {
            let e = rd_u64(g0 + 0x1e0 + enemy * 0x28 + i * 8).unwrap_or(0) as usize;
            if e == 0 { continue; }
            let d2 = idist2(sx as i64, sy as i64, rd_i64(e + 0x648).unwrap_or(0), rd_i64(e + 0x650).unwrap_or(0));
            if (d2 >> 8) >= tune("d4_engage_r2", 0x53D1AC1) as u64 { continue; }
            let id = rd_u64(e + 0x5a8).unwrap_or(0);
            let team = geom_vt68(gchild, enemy, id) as i64;
            let eng = team != 0 || {
                let le = geom_vtc0(gchild, id);
                le != 0 && rd_i64(gchild + 0xeb00).unwrap_or(0) <= rd_i64(geom2 + enemy * 0x2e8 + 0x1e0 + (rd_u32(le + 0x8a0) as usize) * 8).unwrap_or(0) + 0x78//  ★0.5.4 오프셋 이동 반영
            };
            if eng { engaged = true; break; }
        }
        let sc = if engaged { (rd_i64(geom2 + s * 0x2e8 + 0x18).unwrap_or(0) < 0) as u8 * 2 } else { 2 };
        (mode as usize, sc)
    };
    let code2 = |out: usize| -> i64 { wr_u64(out, 2); wr_u8(out + 8, (lane == 1) as u8); wr_u8(out + 9, line as u8); wr_u8(out + 0xa, subcode); 2 };
    // ── ③ tail: 레인상태 게이트 + tgt + pathlen + 위협카운트 + ward/base ──
    let loff = match line { 1 => 0x28usize, 2 => 0x50, _ => 0 };
    let slot = geom2 + s * 0x2e8 + loff;
    if rd_i32(slot).unwrap_or(0) != 1 { return code2(out); }
    let tgt = geom_resolve150(gchild, rd_u32(slot + 8) as u64);
    if !ptr_ok(tgt) { return code2(out); }
    let (tx, ty) = (rd_i64(tgt + 0x648).unwrap_or(0), rd_i64(tgt + 0x650).unwrap_or(0));
    let uvar3 = disc4_pathlen(ctx, mem, athlete, geom, tx as u64, ty as u64);   // A* path.len
    let mut uvar30 = 0i64;
    for i in 0..5usize {
        let sl = rd_u64(g0 + 0x1e0 + s * 0x28 + i * 8).unwrap_or(0) as usize;
        if sl == 0 { continue; }
        let (px, py) = (rd_i64(sl + 0x648).unwrap_or(0), rd_i64(sl + 0x650).unwrap_or(0));
        if (idist2(px, py, sx as i64, sy as i64) >> 8) < tune("d4_engage_r2", 0x53D1AC1) as u64 || (idist2(px, py, tx, ty) >> 8) < tune("d4_engage_r2", 0x53D1AC1) as u64 { uvar30 += 1; }
    }
    // ── ④ lead 결정(★07-12 정정: ward슬롯→argmin→base2 fallback, disasm fc45~fd7b) + 거리게이트(가드제거) ──
    let mut lead = rd_u64(g0 + s * 8 + line * 0x20 + 0x180).unwrap_or(0) as usize;
    if lead == 0 { lead = rd_u64(g0 + s * 8 + line * 0x20 + 0x190).unwrap_or(0) as usize; }
    if lead == 0 {
        let candptr = rd_u64(g0 + s * 0x20 + 0x130).unwrap_or(0) as usize;
        let candlen = rd_u64(g0 + s * 0x20 + 0x148).unwrap_or(0) as usize;
        lead = disc4_lead_argmin(candptr, candlen, line, mode as usize, side);
    }
    let base = rd_u64(g0 + (s + 0x2e) * 8).unwrap_or(0) as usize;   // base2 = *(g0+s*8+0x170), 항상
    if !ptr_ok(base) { return code2(out); }
    if lead == 0 { lead = base; }                                  // 최종 fallback
    let (bx, by) = (rd_i64(base + 0x648).unwrap_or(0), rd_i64(base + 0x650).unwrap_or(0));
    let (lx, ly) = (rd_i64(lead + 0x648).unwrap_or(0), rd_i64(lead + 0x650).unwrap_or(0));
    // ── ④ g2 (가드 제거): dist²(tgt,base2) < dist²(base2,lead) ──
    if idist2(tx, ty, bx, by) < idist2(bx, by, lx, ly) { return code2(out); }
    if uvar30 >= uvar3 { return code2(out); }   // g3
    // g4 (가드 제거): dist²(tgt,lead)>>8 < 0x6ba9301
    if (idist2(tx, ty, lx, ly) >> 8) < tune("d4_ward_dist2", 0x6BA9301) as u64 { return code2(out); }
    // ── ⑤ W-cast: iVar1, bW(t3ea분기·Wobj순수) ──
    let ivar1 = rd_i32(slot + 0x20).unwrap_or(0);
    let busy = kind != 0;
    let w_a_ok = rd_u64(comp + 0x1a8).unwrap_or(0) != 0;   // Wobj[0x1a8]=*(gchild+0xec98)
    let w_b_ok = rd_u64(comp + 0x1d8).unwrap_or(0) != 0;   // Wobj[0x1d8]=*(gchild+0xecc8)
    let bw: bool = if b23 == 1 { line == 0 }
        else if b23 == 0 { line == 2 }
        else if !busy && w_a_ok { line == 2 }
        else { !busy && w_b_ok && line == 0 };
    // ── ⑥ 최종 {2,4,6,7} ──
    let emit67 = |out: usize| -> i64 { let c: i64 = if ivar1 > tune("d4_wcast_thr", 2) as i32 { 7 } else { 6 }; wr_u64(out, c as u64); if c == 6 { wr_u8(out + 8, line as u8); } c };
    let tower_check = |out: usize| -> i64 {
        if lead == 0 || rd_i32(lead + 0x68).unwrap_or(0) != 2 || rd_i64(lead + 0x88).unwrap_or(0) == 0 { code2(out) } else { wr_u64(out, 7); 7 }
    };
    // W-target 재검
    let wtgt: Option<usize> = if line == 2 { if busy || !w_a_ok { None } else { Some(rd_u64(comp + 0x1a0).unwrap_or(0) as usize) } }   // Wobj[0x1a0]=gchild+0xec90
        else if line == 1 { None }
        else { if busy || !w_b_ok { None } else { Some(rd_u64(comp + 0x1d0).unwrap_or(0) as usize) } };   // Wobj[0x1d0]=gchild+0xecc0
    let wvalid: Option<usize> = wtgt.and_then(|wt| {
        if !ptr_ok(wt) { return None; }
        let e = geom_resolve150(gchild, rd_u64(wt).unwrap_or(0));
        if e == 0 { None } else { Some(e) }
    });
    if let Some(we) = wvalid {
        if rd_u8(self_ent) != 0 {
            if bw { emit67(out) } else { tower_check(out) }
        } else {
            let k = rd_u64(self_ent + 8).unwrap_or(0) as usize;
            if !bw { tower_check(out) }
            else if uvar3 > tune("d4_pathlen_thr", 3) { emit67(out) }
            else if rd_u64(we + 0x38 + k * 0x18).unwrap_or(0) != 0 { code2(out) }
            else { emit67(out) }
        }
    } else if !bw { tower_check(out) }
    else if uvar3 <= tune("d4_pathlen_thr", 3) { code2(out) }
    else { emit67(out) }
}

// ════ disc14(실명 EpicPoke — 구라벨 DefenseNexus, §11.8; 진짜 DefenseNexus=disc19 disp3 0x141c83700 미대체) 0.5.0_3 핸들러 0x141c88090 재구현(out-writer). 출력{0x14,0xf,7,0x11,0x10,2,3}. RNG-free. ════
//   콜리=SerpenBattle 헬퍼 재사용(engage게이트 disc5·reposition mode0·self resolve). 시그: (out,cmd=subp+8,level,sim,geom,tp,sf).
//   ⬜v1: dive·phase1·phase3·engage게이트 확정. threat 서브블록(level>45 다이브추적·defN/atkN)=TODO(검증재개시).
const DEF_FORM_LUT: [u8; 9] = [0, 2, 0, 2, 1, 2, 2, 0, 0];   // role→formation (role LUT @0x1438b5adc)
// ════ disc17 (idx15 핸들러 0x141c77f20) 재현. 출력{7, 0x13}, out+0=code만, RNG-free. 07-10 정밀RE. ════
const DISC17_PAT: [&[u8]; 6] = [&[0u8, 1, 2], &[2], &[0], &[2], &[1], &[1, 2]];   // 전략바이트 s→요구 역할집합(role_ok = AND near[p[j]])
// ════ disc4 보조: sim vtable vt0x30 kind 판별(모노모프 6사본 RVA→0/1/2, 07-10 순수RE 확정). ════
// ★★[0.5.2 07-22] kind 판별 = **런타임 도출 1차 + 상수표 2차(폴백)**로 전환 (종전=상수표 단독).
//   ⚠전환 이유(실제 사고): 상수표는 패치마다 100% 이동하는데 stale이면 `_=>0` 폴백으로 조용히 kind0 오판 →
//     **크래시 없이 기능만 사망**해 로그를 안 보면 발각되지 않는다. 실제로 소스 상수가 **0.5.0_3 값**이었고
//     0.5.1 마이그 때 누락돼 **0.5.1·0.5.2 두 버전 연속** movepri 대체가 죽어 있었다(0.5.2 인게임 실측:
//     `[mp STAGE-GATE] stage!=2 skip 누적=15,209,000` = 1,520만 회 전량 skip). 0.5.1 "인게임 검증완"은 이 항목엔 무효.
//   ★런타임 도출 근거(0.5.0_3·0.5.1·0.5.2 3버전 18/18 바이트 완전동일 실측 검증):
//     vtable+0x30 슬롯이 가리키는 본체가 kind를 **상수로 반환**한다 —
//       48 8d 91 f0 ea 00 00   lea rdx,[rcx+0xeaf0]   ← 신원검증 프리픽스(소스 `comp = gchild+0xeaf0`와 일치)
//       31 c0 / b8 01 00 00 00 / b8 02 00 00 00       ← xor eax,eax=0 · mov eax,1 · mov eax,2 = 곧 kind
//       c3                     ret
//     ⟹ kind가 exe 안에 박혀 있어 **매핑이 뒤바뀔 여지가 없다**(07-11 §12.23 튜토리얼 오발화 크래시 원천차단).
//   판별 실패(패턴 불일치·읽기 실패)는 상수표로 폴백하고, 그것도 미상이면 종전대로 0(보수) — fail-safe 불변.
const VT30_CACHE_N: usize = 8;   // 실측 vtable 6개(3 kind × 2 trait) + 여유 2
static VT30_CACHE: [(AtomicUsize, AtomicI64); VT30_CACHE_N] = [
    (AtomicUsize::new(0), AtomicI64::new(-1)), (AtomicUsize::new(0), AtomicI64::new(-1)),
    (AtomicUsize::new(0), AtomicI64::new(-1)), (AtomicUsize::new(0), AtomicI64::new(-1)),
    (AtomicUsize::new(0), AtomicI64::new(-1)), (AtomicUsize::new(0), AtomicI64::new(-1)),
    (AtomicUsize::new(0), AtomicI64::new(-1)), (AtomicUsize::new(0), AtomicI64::new(-1)),
];
// vtable+0x30 본체 바이트를 읽어 kind를 뽑는다. 실패=None. rd_* = VEH 경유 안전읽기(stale ptr 세그폴트 방지).
#[inline] unsafe fn vt30_probe(gvt: usize) -> Option<i64> {
    if !ptr_ok(gvt) { return None; }
    let f = rd_u64(gvt.wrapping_add(0x30))? as usize;
    if !ptr_ok(f) { return None; }
    // lea rdx,[rcx+0xeaf0] = 48 8d 91 | f0 ea 00 00
    if rd_u8(f) != 0x48 || rd_u8(f + 1) != 0x8d || rd_u8(f + 2) != 0x91 { return None; }
    if rd_u32(f + 3) != 0xeb30 { return None; }
    match rd_u8(f + 7) {
        0x31 if rd_u8(f + 8) == 0xc0 => Some(0),                                  // xor eax,eax
        0xb8 => { let v = rd_u32(f + 8); if v <= 2 { Some(v as i64) } else { None } }   // mov eax,imm32
        _ => None,
    }
}
// 1,520만 회/경기 호출되는 핫패스라 gvt→kind를 락프리 캐시(실패도 -2로 캐시해 매회 재프로브 방지).
#[inline] unsafe fn vt30_kind_cached(gvt: usize) -> Option<i64> {
    for s in VT30_CACHE.iter() {
        match s.0.load(Ordering::Relaxed) {
            0 => break,                       // 이후는 전부 빈 슬롯
            g if g == gvt => { let v = s.1.load(Ordering::Relaxed); return if v >= 0 { Some(v) } else { None }; }
            _ => {}
        }
    }
    let kind = vt30_probe(gvt);
    for s in VT30_CACHE.iter() {   // 첫 빈 슬롯에 등록(경합 시 CAS 실패한 쪽은 다음 슬롯으로)
        if s.0.compare_exchange(0, gvt, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
            s.1.store(kind.unwrap_or(-2), Ordering::Relaxed); break;
        }
        if s.0.load(Ordering::Relaxed) == gvt { break; }   // 다른 스레드가 이미 등록
    }
    kind
}
#[inline] unsafe fn disc4_vt30_kind(gvt: usize) -> i64 {
    if let Some(k) = vt30_kind_cached(gvt) { return k; }   // 1차: 런타임 도출(패치 내성)
    let b = exe_base();
    if b == 0 { return 0; }
    match gvt.wrapping_sub(b) {   // 2차: 확정 상수표(런타임 도출 실패 시 폴백)
        // ⚠검증 시 주의: k0 arm은 `_ => 0`과 결과가 같아 **컴파일러가 제거**한다 → dll 바이트에 0x383cd68/0x38c5d78이
        //   **없는 것이 정상**(k1·k2 4개만 링크됨). 07-22 배포본에서 실제로 확인 — stale로 오판하지 말 것.
        0x383cd68 | 0x38c5d78 => 0,   // ★0.5.2 확정(ghidra-re 07-22 + 바이트 실측). ~~0.5.0_3 0x37d9ee0|0x386b080~~ / 0.5.1=0x38942f8|0x38a66d8
        0x383d080 | 0x38c5aa0 => 1,   // stage1=튜토리얼/축소 컨텍스트. ~~0.5.0_3 0x37da190|0x386ae10~~ / 0.5.1=0x3894610|0x38a6400
        0x383d358 | 0x38c57c8 => 2,   // stage2=정규+백그라운드 풀매치(movepri 대체 허용 조건). ~~0.5.0_3 0x37da400|0x386aba0~~ / 0.5.1=0x38948e8|0x38a6128
        _ => 0,   // 미상 사본 → kind0 보수(타깃경로 활성)
    }
}
#[inline] fn idist2(ax: i64, ay: i64, bx: i64, by: i64) -> u64 {
    let dx = (ax - bx).unsigned_abs(); let dy = (ay - by).unsigned_abs();
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}
// ════ FUN_141fd2790→FUN_142312bd0 A* 레인노드 안전열거기 path.len 재현(07-10 완전RE, ★표본미발화=검증불가). ════
//   반환 = 필터통과 레인노드(0..4) 수. branch A(kind!=2)=로컬RNG 지터, branch B(kind==2)=전역RNG 1소비. FUN_142377e00 조기탈출=code8→len0.
//   vt0x20/0x30=순수raw(posA=*(gchild+0xeab8), kind=RVA판별, comp=gchild+0xeaf0). vt68/vtc0=shadow(비다형·검증완료).
unsafe fn disc4_pathlen(tp: usize, rng4: usize, athlete: usize, geom: usize, tgtx: u64, tgty: u64) -> i64 {
    let g0 = rd_u64(geom).unwrap_or(0) as usize;               // MAP
    let geom1 = rd_u64(geom + 8).unwrap_or(0) as usize;
    let geom2 = rd_u64(geom + 0x10).unwrap_or(0) as usize;     // S[1]=param_6[2]
    if !ptr_ok(g0) || !ptr_ok(geom1) { return 0; }
    let gchild = rd_u64(g0).unwrap_or(0) as usize;
    let gvt = rd_u64(g0 + 8).unwrap_or(0) as usize;
    if !ptr_ok(gchild) || !ptr_ok(gvt) { return 0; }
    let team = rd_u64(athlete + 0x810).unwrap_or(2);//  ★0.5.4 오프셋 이동 반영
    if team > 1 { return 0; }
    let side = (1 - team) as usize;
    let kind = disc4_vt30_kind(gvt);
    let posb = rd_i64(gchild + 0xeb00).unwrap_or(0);           // vt0x28(tick 필드) = posB
    let n = rd_i64(rd_u64(geom1 + 8).unwrap_or(0) as usize + 0x12f8).unwrap_or(0);
    let j = rd_i64(athlete + 0x218).unwrap_or(0).min(100);
    let self_id = rd_u64(athlete + 0x818).unwrap_or(0);
    // FUN_142377e00 조기탈출(kind==0 && athlete[0x414]==1) → len0
    if kind == 0 && rd_u32(athlete + 0x414) == 1 {
        let posa = rd_i64(gchild + 0xeb28).unwrap_or(0);
        let jud1 = rd_i64(athlete + 0x208).unwrap_or(0).min(100);
        let jud2 = rd_i64(athlete + 0x210).unwrap_or(0).min(100);
        let bail = disc4_earlyexit(posa, self_id, posb, n, jud1, jud2, rd_i64(athlete + 0x3f0).unwrap_or(0));
        if (self_id | (bail as u64)) & 1 != 0 { return 0; }
    }
    let mut len = 0i64;
    for lane in 0..5usize {
        let node = rd_u64(g0 + 0x1e0 + side * 0x28 + lane * 8).unwrap_or(0) as usize;
        if node == 0 { continue; }
        let id = rd_u64(node + 0x5a8).unwrap_or(0);
        if geom_vt68(gchild, team as usize, id) { continue; }   // vt68 안전판정 nonzero=위험 skip
        let w = rd_i64(node + 0x628).unwrap_or(0);
        let (nx, ny) = (rd_i64(node + 0x648).unwrap_or(0), rd_i64(node + 0x650).unwrap_or(0));
        let wp = rd_i64(tp + 0x290 + lane * 8).unwrap_or(0);
        let mut pushed = false;
        if kind != 2 {
            let diff = posb.saturating_sub(wp);
            if diff > 3 * n {   // ── far: 로컬RNG 지터 ──
                let scaled = (((100 - j) * (100 - j)) as u64).wrapping_mul(0x2d99999a4718) >> 43;
                let threshold = diff.wrapping_mul(scaled as i64 + 3000) / n.max(1) + 40000;
                if threshold > 300000 { continue; }
                let posa = rd_i64(gchild + 0xeb28).unwrap_or(0);
                let seed = (((posb / (6 * n).max(1)) as u64) << 40) ^ (posa as u64) ^ ((lane as u64) << 8) ^ self_id;
                let mut r = LocalRng::seed_from_u64(seed);
                let jx = r.gen_range_i64(-1000, 1000);
                let jy = r.gen_range_i64(-1000, 1000);
                let rr = r.gen_range_i64(0, threshold);
                let mag2 = (jx.wrapping_mul(jx) + jy.wrapping_mul(jy)) as u64;
                let dist = (mag2.isqrt() as i64).max(1);
                let fx = (nx + jx.wrapping_mul(rr) / dist).max(0);
                let fy = (ny + rr.wrapping_mul(jy) / dist).max(0);
                let dtt = idist2(tgtx as i64, tgty as i64, fx, fy).isqrt() as i64;
                pushed = dtt <= threshold + 3 * w * n + 150000;
            } else {   // ── near: RNG 없음 ──
                let wx = rd_i64(tp + 0x218 + lane * 0x10).unwrap_or(0);
                let wy = rd_i64(tp + 0x218 + lane * 0x10 + 8).unwrap_or(0);
                let dd = idist2(wx, wy, tgtx as i64, tgty as i64).isqrt() as i64;
                pushed = (dd - 150000).max(0) <= diff * w;
            }
        } else {   // ── branch B kind==2: 전역RNG 1소비 ──
            let mut dr = 0u32;
            let draw = rng_gen_range(rng4, (-1000i64) as u64, 1000, &mut dr).unwrap_or(0) as i64;
            let beyond = posb.saturating_sub(wp);   // vt28=castR=posb
            let wx = rd_i64(tp + 0x218 + lane * 0x10).unwrap_or(0);
            let wy = rd_i64(tp + 0x218 + lane * 0x10 + 8).unwrap_or(0);
            let dd = idist2(wx, wy, tgtx as i64, tgty as i64).isqrt() as i64;
            let lhs = beyond.wrapping_mul(draw.wrapping_mul(w) / 1000);
            let rhs = (dd - 150000).max(0);
            let mut ok = true;
            if rd_i64(tp + 0x268 + lane * 8).unwrap_or(0) < wp {
                let ent = geom_vtc0(gchild, id);
                if ent == 0 { ok = geom_vt68(gchild, team as usize, id); }
                else {
                    let rlane = rd_u32(ent + 0x8a0) as usize;//  ★0.5.4 오프셋 이동 반영
                    let base = rd_i64(geom2 + side * 0x2e8 + 0x1e0 + rlane * 8).unwrap_or(0);
                    ok = posb <= base + 600;
                }
            }
            if ok { pushed = rhs <= lhs; }
        }
        if pushed { len += 1; }
    }
    len
}
// FUN_142377e00 조기탈출 coin-flip(로컬RNG 거부표본). 판단력 100이면 미발화. 07-10 완전RE.
unsafe fn disc4_earlyexit(posa: i64, self_id: u64, posb: i64, unit: i64, _jud1: i64, _jud2: i64, jud: i64) -> bool {
    let u = unit.max(1);
    let q = posb / (10 * u);
    let seed = (posa as u64) ^ ((self_id) << 4) ^ ((q as u64) << 40) ^ 0x1a75e;
    let mut r = LocalRng::seed_from_u64(seed);
    let d0 = r.gen_range_i64(3 * u, 6 * u);
    let _d1 = r.gen_range_i64(3 * u, 6 * u);   // 뽑고 버림(상태전진)
    let roll = ((r.next_u64() as u128).wrapping_mul(10000) >> 64) as i64;   // lemire uniform[0,10000)
    let _ = ((r.next_u64() as u128).wrapping_mul(10000) >> 64) as i64;      // 두번째 roll 미사용(상태전진)
    let reach = (((posb - 210 * u).max(0)).wrapping_mul(6000) / (810 * u)).clamp(0, 6750);
    let jr = (100 - jud).clamp(0, 100);
    let thr1 = jr.wrapping_mul(jr).wrapping_mul(reach) / 10000;
    if roll < thr1 {
        let reach_dist = (d0 + reach.wrapping_mul(u) / 6000).min(7 * u);
        (posb % (10 * u)) < reach_dist
    } else {
        false
    }
}

// facet#4 movepriority 재현(code@출력+0). Stage1=상수+data-var+AttackNexus.
// Stage2b: 3(dd7700)/9(EpicPoke)/10(EpicBattle)/11(SerpenPoke)/12(SerpenBattle)/14(DefNexus) 전부 재현.
// ★disc4 메인경로(좌표게이트+첫 TTD루프) 토글 + 진단카운터. d4ttd=1이면 my_disc4가 TTD경로 사용(기본off=late7 단순화).
static D4_TTD: AtomicBool = AtomicBool::new(false);
static D4_REPL: AtomicBool = AtomicBool::new(true);     // disc4 mp_repl 대체 토글(cfg d4_repl; false=passthrough 격리)
static D7_REPL: AtomicBool = AtomicBool::new(false);    // disc7(Recall) mp_repl 라이브대체 토글(cfg d7_repl; 기본0=원본. 신모델 인게임 400/400 확인 전까지 격리)
// ★★[07-23] disc14 대체 토글(cfg d14_repl; 기본1=대체). **0으로 두면 passthrough+캡처 → mpcmp에 disc14 판정줄 생성**.
//   도입 사유: **대체되면 리턴훅이 안 돌아 game↔mine 비교 자체가 성립하지 않는다**(07-23 실측: disc14 대체 971회 발화했으나
//   mpcmp 판정줄 0건. 판정줄이 나온 subplan 0·7·12는 전부 passthrough 호출이었다). ⟹ 재검증하려면 일시적으로 꺼야 한다.
//   07-23에 콜리 3종(engage_gate·rng_pick·reposition_fight)을 수정해 disc14 라이브 동작이 바뀌었고,
//   기존 "400/400 DIFF=0"은 수정 전 기준이라 무효 상태 ⟹ 이 토글로 재검증한다.
static D14_REPL: AtomicBool = AtomicBool::new(true);    // cfg d14_repl; false=disc14만 passthrough(검증용)
// ★[07-23] disc12 대체 토글(cfg d12_repl; 기본1=대체 / 0=passthrough+캡처). 편입 직후라 문제 시 재빌드 없이 격리·재검증용.
static D12_REPL: AtomicBool = AtomicBool::new(true);
static D15_REPL: AtomicBool = AtomicBool::new(false);   // disc15(SerpenCheck) 라이브대체 토글(cfg d15_repl; 기본0=원본. 재현 미검증·표본부족 opt-in)
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
    let _posg = pos_enter_p56(p5, p6);   // ★포지션별 cfg: t_ttd/d4_* 갱킹·처치 계수 포지션 응답
    let _ = subp;
    if D4FREEZE.load(Ordering::Relaxed) { D4_CN.store(D4_CALLN.fetch_add(1, Ordering::Relaxed)+1, Ordering::Relaxed); }
    d4stage!(&format!("ENTER subp={:#x} p5={:#x} p6={:#x}", subp, p5, p6));
    if !ptr_ok(p5) || !ptr_ok(p6) { d4stage!("EXIT -99 badp5p6"); return -99; }   // ★readable VQ제거(p5+0x6a0 unwrap_or)
    let l80 = rd_u64(p6).unwrap_or(0) as usize;
    if !ptr_ok(l80) { return -99; }
    let obj = rd_u64(l80).unwrap_or(0) as usize;
    let vt = rd_u64(l80 + 8).unwrap_or(0) as usize;
    if !ptr_ok(obj) || !ptr_ok(vt) { return -99; }
    // ★07-10: vt슬롯 0x128 직접호출 제거(0.5.0선 4인자 뮤테이터=AV) → dd7_slot128(=vt0x138 리졸버 순수재현). ⚠이 함수(구 my_disc4)는 0.4.x 오프셋 잔재(dead, d4_repl 라우팅=050).
    let handle = rd_u64(p5 + 0x6a0).unwrap_or(0) as usize;
    let target = dd7_slot128(obj, handle as u64);
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
    // ⛔★[07-23 감사] **0.5.2 stale — 부활 금지**. 구 슬롯 `0x168`은 0.5.2에서 **`0x1d0`으로 시프트**(≥0x50 일괄 +0x68).
    //   0.5.2의 `vt+0x168` = `0x2302700` = **쓰기 있는 대형 함수**(`mov [rdx+0x660]` + 0x6a8B memcpy)인데
    //   `vt_call1`은 **rdx를 전달하지 않는다** ⟹ 되살리면 **미정의 rdx로 즉시 힙 파손**(07-22형 크래시 클래스).
    //   현재는 호출자 부재로 도달 불가(disc4는 `my_disc4_050`으로 라우팅) = 死코드라 무해.
    //   되살리려면 반드시 슬롯 `0x1d0` + rdx 규약 재확인부터 할 것.
    let vec = vt_call1(vt, 0x168, obj);
    if !ptr_ok(vec) { return 0; }
    let cnt = rd_u64(vec + 0x190).unwrap_or(0) as usize;
    let ptr = rd_u64(vec + 0x188).unwrap_or(0) as usize;
    if !ptr_ok(ptr) || cnt > 64 { return 0; }
    let r9 = exe + 0x381e1e0;                               // base getter r9(ATK_VT). ★0.5.2(was 0.4.x 0x35e4d00 stale) — ghidra-re 07-23 실바이트 확정
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
    let k = apos(&STAT_INFLUENCE, "stat_influence");
    if k <= 0 || !ptr_ok(p5) { return (0, 0); }
    let aggr = rd_i64(p5 + 0x230).unwrap_or(50).clamp(0, 100);   // 공격성
    let ego  = rd_i64(p5 + 0x238).unwrap_or(50).clamp(0, 100);   // 에고
    let judg = rd_i64(p5 + 0x218).unwrap_or(100).clamp(0, 200);  // 판단력(effective, 100초과 가능)
    // ★비대칭 가중: 위(공격적,>50)는 절반(100/100=+50=많이안뺌, "절대안뺌"=+100은 150/150 필요=상한초과) / 아래(소심,<50)는 그대로(0/0=−100=호각도뺌). 50/50=0.
    let ca = { let d = aggr - tune("stat_neutral", 50); if d > 0 { d / tune("stat_pos_div", 2).max(1) } else { d } };
    let ce = { let d = ego  - tune("stat_neutral", 50); if d > 0 { d / tune("stat_pos_div", 2).max(1) } else { d } };
    let stat_adj = (ca + ce) * k / 100;                          // 공격성·에고↑ → +adj → eff임계↓ → 덜 후퇴(다이브)
    let amp = ((tune("stat_judg_ref", 100) - judg).max(0)) * k / 100;                   // 판단력↓ → 노이즈 진폭↑. ≥100=0(완벽판단)
    let jnoise = if amp > 0 {
        let tick = dd7_slot20(sim) as u64;
        // ★★[07-29 결정성 수정] seed = ~~핸들(p5+0x818)~~ → **athlete_id(p5+0x810)**.
        //   핸들 = SlotMap (idx|generation)인데 **generation은 슬롯 재사용 횟수에 의존** — 배경 워커는 World를 재사용하며
        //   경기를 연속 sim하므로 관전 sim과 generation이 어긋날 수 있음 ⟹ 같은 선수의 jnoise가 sim마다 달라짐
        //   = 후퇴 판정 갈림 = **배경≠관전 위치 발산**(d9/11/12 arm 활성 시 apply_numbers_sp 발화 증가로 표면화).
        //   athlete_id(+0x810)=per-athlete 고유 DB id·양 sim 동일(DONE.md +0x810조인 확정) ⟹ 탈상관 유지+결정성 확보.
        let handle = rd_u64(p5 + 0x810).unwrap_or(0);
        (stat_hash(tick >> (tune("stat_noise_shift", 5) as u32), handle) % (2 * amp as u64 + 1)) as i64 - amp   // [-amp,+amp] 결정론(~0.5s 코히런트=프레임 깜빡임 방지)
    } else { 0 };
    (stat_adj, jnoise)
}
unsafe fn tower_threat_acc(p6: usize, team: i64, selfe: usize) -> u64 {
    if team < 0 || team > 1 || !ptr_ok(selfe) { return 0; }
    let l80 = match rd_u64(p6) { Some(v) if ptr_ok(v as usize) => v as usize, _ => return 0 };
    let _pg = pos_enter_world(l80, team, selfe, 0);   // ★포지션별 cfg (disc4 TTD 경로도 per-pos). p5 없음→우리팀 게이트시 per-champ 미적용(정글러 tower는 라이너 경로가 커버)
    let threat = apos(&TOWER_THREAT, "tower_threat");
    if threat <= 0 { return 0; }
    let sx = rd_u64(selfe + 0x648).unwrap_or(0); let sy = rd_u64(selfe + 0x650).unwrap_or(0);
    let et = (1 - team) as usize;   // 적팀
    let trange = apos_u(&TOWER_RANGE, "tower_range"); let tr2 = trange.wrapping_mul(trange);
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
// ★[07-16] 기지박힘(roam_diag) 진단: 후퇴 사유별 승산분포 + 포탑 체류율. 실제 시뮬(judge 발화) 때만 채워짐.
static LANER_CALL_N: AtomicU64 = AtomicU64::new(0);    // laner_should_retreat 총 호출(=라이너 판단 횟수)
static TOW_UNDER_N: AtomicU64 = AtomicU64::new(0);     // 적포탑 사거리내(under) 판정 횟수 = 포탑/기지 근처 체류
static TOW_W_SUM: AtomicU64 = AtomicU64::new(0);       // 포탑후퇴 시 승산 누적(평균=SUM/CNT)
static TOW_W_CNT: AtomicU64 = AtomicU64::new(0);
static FRC_W_SUM: AtomicU64 = AtomicU64::new(0);       // 전력후퇴 시 승산 누적
static FRC_W_CNT: AtomicU64 = AtomicU64::new(0);
static ROAM_DIAG: AtomicBool = AtomicBool::new(false); // cfg roam_diag: 기지박힘 진단 flush ON/OFF
static ROAM_DIAG_LAST: AtomicU64 = AtomicU64::new(0);  // flush 스로틀
// ★라이너(dd7700/PassiveLine) 후퇴 판단 — 전력(force)승산 기반(2026-06-23 정식형 DPS×HP). 승산=combat_balance(force_ally×100/force_enemy, force=ΣHP×Σ공격). ①포탑밑 AND tower_threat≥승산 ②일반교전 numbers_threat≥승산(적없으면 승산9999=자동 근접게이트) ③단순머릿수 margin. dd7700=매라이너매프레임=라이너 다이브/불리교전 직접차단(disc4=정글러와 별개). l80=*p6, et=적팀. 기본(전부0)=원본동작.
// ★[07-16] 기지박힘(양팀 안 나옴) 진단 flush — LOG_ON 무관 직접 write. roam_diag=1일 때 라이너 판단 2000회마다.
//   실제 시뮬(일정넘김·라이브 judge) 때만 채워짐(재생 중엔 judge=0이라 안 참). 어느 회피 게이트가 후퇴를 지배하는지 규명.
unsafe fn roam_diag_flush() {
    let call = LANER_CALL_N.load(Ordering::Relaxed);
    let ret  = LANER_RET_N.load(Ordering::Relaxed);
    let tow  = LANER_RET_TOW.load(Ordering::Relaxed);
    let frc  = LANER_RET_FRC.load(Ordering::Relaxed);
    let num  = LANER_RET_NUM.load(Ordering::Relaxed);
    let under= TOW_UNDER_N.load(Ordering::Relaxed);
    let tws  = TOW_W_SUM.load(Ordering::Relaxed); let twc = TOW_W_CNT.load(Ordering::Relaxed);
    let frs  = FRC_W_SUM.load(Ordering::Relaxed); let frc_c = FRC_W_CNT.load(Ordering::Relaxed);
    let pct = |a: u64, b: u64| if b > 0 { a * 100 / b } else { 0 };
    let avg = |s: u64, c: u64| if c > 0 { s / c } else { 0 };
    let s = format!(
        "=== roam_diag (기지박힘 진단) tow_threat={} tow_range={} num_threat={} ===\n\
         라이너 판단 {}회 → 후퇴 {}회({}%) | 사유: 포탑 {} / 전력 {} / 머릿수 {}\n\
         포탑 근처(under) 체류 {}회({}% of 판단) → 그중 후퇴 {}회({}% of under)\n\
         승산 평균(100=호각): 포탑후퇴 시 {} / 전력후퇴 시 {}  ※승산<임계면 후퇴. tow_threat={}이 승산보다 크면 항상 후퇴\n",
        TOWER_THREAT.load(Ordering::Relaxed), TOWER_RANGE.load(Ordering::Relaxed), NUMBERS_THREAT.load(Ordering::Relaxed),
        call, ret, pct(ret, call), tow, frc, num,
        under, pct(under, call), tow, pct(tow, under),
        avg(tws, twc), avg(frs, frc_c), TOWER_THREAT.load(Ordering::Relaxed));
    if let Some(p) = pth("roam_diag.txt") { let _ = fs::write(p, s); }
}
unsafe fn laner_should_retreat(p6: usize, team: i64, selfe: usize, p5: usize, base_code: u8, disc: i64) -> bool {
    if team < 0 || team > 1 || !ptr_ok(selfe) { return false; }
    if ROAM_DIAG.load(Ordering::Relaxed) { let n = LANER_CALL_N.fetch_add(1, Ordering::Relaxed); if n % 2000 == 1999 { roam_diag_flush(); } }   // ★기지박힘 진단
    let l80 = match rd_u64(p6) { Some(v) if ptr_ok(v as usize) => v as usize, _ => return false };
    let _pg = pos_enter_world(l80, team, selfe, p5);   // ★포지션별 cfg + 우리팀 게이트(p5=athlete struct). 이하 apos()/tune()이 이 선수 오버라이드 적용
    let sx = rd_u64(selfe + 0x648).unwrap_or(0); let sy = rd_u64(selfe + 0x650).unwrap_or(0);
    let et = (1 - team) as usize;
    let threat = apos(&TOWER_THREAT, "tower_threat");
    // ★subplan(disc)별 개별 임계 우선: NUMBERS_THREAT_SP[disc]≥0이면 그 값, -1(미설정)이면 공통 폴백(dd7700 출력 code별: 2=Move→numbers_threat_move / 4·6·7=numbers_threat).
    let nthreat = {
        let sp = if (0..18).contains(&disc) { NUMBERS_THREAT_SP[disc as usize].load(Ordering::Relaxed) } else { -1 };
        if sp >= 0 { sp }
        else if base_code == 2 { let m = apos(&NUMBERS_THREAT_MOVE, "numbers_threat_move"); if m >= 0 { m } else { apos(&NUMBERS_THREAT, "numbers_threat") } }
        else { apos(&NUMBERS_THREAT, "numbers_threat") }
    };
    let margin = apos(&NUMBERS_MARGIN, "numbers_margin");
    let (w, ally, enemy) = combat_balance(l80, team, sx, sy, base_code).unwrap_or((9999, 1, 0));   // 전력승산 + 머릿수(공용, 포탑 base_code별)
    if threat > 0 || nthreat > 0 { LANER_RET_W.store(w, Ordering::Relaxed); }   // 진단 샘플
    let (stat_adj, jnoise) = stat_modifiers(p5, rd_u64(l80).unwrap_or(0) as usize);   // ★성향보정: 공격성+에고=임계시프트, 판단력=노이즈. k0/중립=0=현행
    // ① 포탑: self가 적포탑 사거리내 AND tower_threat ≥ 전력승산 → 후퇴. ★threat=100→호각싸움도 수비, 0→미적용.
    if threat > 0 {
        let trange = apos_u(&TOWER_RANGE, "tower_range"); let tr2 = trange.wrapping_mul(trange);
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
        if under { TOW_UNDER_N.fetch_add(1, Ordering::Relaxed); }   // ★진단: 포탑/기지 근처 체류
        if under && (threat - stat_adj + jnoise) >= w {   // ★포탑밑 + (성향보정)tower_threat가 전력승산 이상 = 불리 → 후퇴
            LANER_RET_N.fetch_add(1, Ordering::Relaxed); LANER_RET_TOW.fetch_add(1, Ordering::Relaxed);
            TOW_W_SUM.fetch_add((w.max(0) as u64).min(100000), Ordering::Relaxed); TOW_W_CNT.fetch_add(1, Ordering::Relaxed);   // ★진단: 포탑후퇴 승산
            return true;
        }
    }
    // ② 일반교전 전력(force): subplan별 numbers_threat ≥ 전력승산 → 후퇴. ★근처 적 ≥ numbers_min_enemy 일때만(라인전 1:1 제외용). 적없으면 w=9999라 미발동.
    let min_e = { let m = apos(&NUMBERS_MIN_ENEMY_MOVE, "numbers_min_enemy_move"); if base_code == 2 && m >= 0 { m.max(1) } else { apos(&NUMBERS_MIN_ENEMY, "numbers_min_enemy") } };   // ★머릿수게이트 base_code별(라인전 _move, -1=한타값)
    if nthreat > 0 { if base_code == 2 { SEEN_C2.fetch_add(1, Ordering::Relaxed); } else { SEEN_C67.fetch_add(1, Ordering::Relaxed); } }   // 진단: code별 force 진입
    if nthreat > 0 && (enemy as i64) >= min_e && (nthreat - stat_adj + jnoise) >= w {   // ★머릿수게이트 + (성향보정)code별 numbers_threat ≥ 전력승산
        if enemy >= 2 { FRC_E2.fetch_add(1, Ordering::Relaxed); } else { FRC_E1.fetch_add(1, Ordering::Relaxed); }
        if base_code == 2 { RET_C2.fetch_add(1, Ordering::Relaxed); } else { RET_C67.fetch_add(1, Ordering::Relaxed); }
        LANER_RET_N.fetch_add(1, Ordering::Relaxed); LANER_RET_FRC.fetch_add(1, Ordering::Relaxed);
        FRC_W_SUM.fetch_add((w.max(0) as u64).min(100000), Ordering::Relaxed); FRC_W_CNT.fetch_add(1, Ordering::Relaxed);   // ★진단: 전력후퇴 승산
        return true;
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
    // ⛔★[07-23 감사] **0.5.2 stale — 부활 금지**. 구 슬롯 `0x168`은 0.5.2에서 **`0x1d0`으로 시프트**(≥0x50 일괄 +0x68).
    //   0.5.2의 `vt+0x168` = `0x2302700` = **쓰기 있는 대형 함수**(`mov [rdx+0x660]` + 0x6a8B memcpy)인데
    //   `vt_call1`은 **rdx를 전달하지 않는다** ⟹ 되살리면 **미정의 rdx로 즉시 힙 파손**(07-22형 크래시 클래스).
    //   현재는 호출자 부재로 도달 불가(disc4는 `my_disc4_050`으로 라우팅) = 死코드라 무해.
    //   되살리려면 반드시 슬롯 `0x1d0` + rdx 규약 재확인부터 할 것.
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
// ⛔★[07-23 감사] **호출자 없음 = 死코드**(disc4는 `my_disc4_050`으로 라우팅). 이 함수와 `disc4_ttd_acc`가 유일한 ATK_VT 소비처였고,
//   둘 다 도달 불가라 `0x35e4d00` stale은 실害 0이었다(desc 화이트리스트 가드까지 이중 차단).
// ⚠**되살릴 때 필수 수정 3건**(0.5.2 원본 = `FUN_141b93830`, ghidra-re 07-23):
//   ① 능력 게터2 슬롯 `vth+0x40` → **`vth+0x90`**(0.4.x→0.5.x 이동). 게터1 `vth+0x30`은 불변·4인자 `(buf,sim,target,desc)` 그대로.
//   ② 게터2 호출규약 = `rcx=sret, rdx=buf, r8=sim, r9=target, [rsp+0x20]=desc`(5인자·desc가 스택) — `Vt40Fn` 시그니처 자체는 유효.
//   ③ 반환 검사 위치 이동: 0.5.2는 `cmp dword[sret+0x48],-1` / `cmp dword[sret+0x80],0` (현 코드는 `out[0]` / `out+0x40`) → 재확인 필수.
//   그리고 `disc4_ttd_acc`의 vt 슬롯 `0x168`→`0x1d0` 문제(그 함수 주석 참조)도 함께 고쳐야 한다.
unsafe fn disc4_subplan_r13b(target: usize, sim: usize, exe: usize) -> i32 {
    let disc: i32 = 4;
    if rd_i32(target + 0x3d8).unwrap_or(0) > 0 { return disc; }   // 3d8>0 → 2nd_dispatch, r13b=disc
    let atkvt = exe + 0x381e1e0;   // ★0.5.2(was 0.4.x 0x35e4d00 stale) — ghidra-re 07-23 실바이트 확정
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
#[inline(never)] unsafe fn my_disc4_main(subp: usize, p6: usize, obj: usize, vt: usize, target: usize, team: i64, hp: u64, maxhp: u64) -> i64 {
    let exe = exe_base();
    if exe == 0 { return disc4_late7(hp, maxhp); }
    let sim = rd_u64(p6 + 8).unwrap_or(0) as usize;
    let cfg_root = rd_u64(p6 + 8).unwrap_or(0) as usize;
    let cfg_b = rd_u64(cfg_root + 8).unwrap_or(0) as usize;
    let cfg_thr = (rd_u64(cfg_b + 0x12f8).unwrap_or(0) as i128 * apos(&TUNE_TTD_MULT, "t_ttd") as i128 / 100) as u64;   // ★튜닝: disc4 TTD 임계 배율(t_ttd%; 포지션별). 높을수록 처치판단 빡빡
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
#[inline(never)] unsafe fn my_movepriority(disc: u64, r15: usize, r14: usize, subp: usize, r8: u64, r9: usize, p7_dd: usize, p7p: usize) -> i64 {
    let _pg = perf_guard(6);
    match disc {
        // ★0.5.0_3 인라인 라우팅(movepri 디스패처 0x1419e4a50, ad6f RE + mpcmp 실측)
        2 | 8 => return 7,                              // 인라인 상수7 (블록 0x1419e4aa8 공유, aux 없음 disasm 확정 07-11). 라이브 대체=mp_capture disc2||8 분기
        6 => return 0xa,                               // 인라인 상수0xa (0x1419e4ba3). ✅aux 해소(07-11): 라이브 대체=mp_write_disc6(전체 write-set)
        5 => return rd_i64(subp + 8).unwrap_or(-99),   // data-var = *(subp+8) (0x1419e4ca2). ✅aux 해소(07-11): 라이브 대체=mp_write_disc5(전체 write-set)
        7 => return my_disc7(r15, r14, subp),   // ★07-11 완전재현(§12.22): zone-box·웨이포인트TTD·스캔2/3·r12b임계전환. 구 "hp%>=0x29→8" 단순모델(397/400) supersede.
        // ★disc3 충돌 해소(a6e4): disc0/1/3 = 동일 핸들러 0x141c76ed0 = dd7700(0.5.0_2 0x2100a30서 MOVED). my_dd7700_code가 이 함수 reimpl({2,4,6,7}, 선두 selector 포함) → 아래 통합.
        4 => {   // ★07-10 0.5.0_3 REWRITE 배선: my_disc4_050(0x1c6f260 재구현, out-writer)을 scratch에 쓰고 code만 반환(게임 out 무오염). 구 my_disc4=0.4.x 오프셋(p5+0x6a0/vt0x128) stale.
            let mut scratch = [0u64; 8];
            return my_disc4_050(scratch.as_mut_ptr() as usize, subp + 8, r8 as i64, r14, r15, p7p);   // ★ctx=tp=param7=p7p(arg7). 구 p7_dd(arg8) 정정.
        }
        0 | 1 | 3 => return my_dd7700_code(subp, r8, r9, r14, r15, p7_dd, false),  // ★disc0/1/3 공유핸들러 0x141c76ed0=dd7700. ★07-10 검증수정: idx1 케이스블록만 add rdx,8 안함→param_2=subp(NOT subp+8, subp+8이면 early-7게이트가 subp+0x118 읽어 항상7). p5/p6는 원순서 유지: 디스패처 프롤로그 완전추적(ghidra-re)=param_5(엔티티)=R15=arg5=[entry_rsp+0x28]=캡처변수 r14, param_6(컨텍스트triple)=R14=arg6=[entry_rsp+0x30]=캡처변수 r15. p7=arg8=[entry_rsp+0x40]=p7_dd(정확). ★잠깐 스왑했다 되돌림(변수명 r14/r15가 arg5/arg6로 이미 정렬).
        14 => {   // ★07-10 0.5.0_3 REWRITE 배선: my_defense_nexus_050(0x1c88090 재구현, out-writer) scratch 검증. tp=p7p(threat +0xc0/+0xd0)/sf=p7_dd(plan +0x3ea/+0x3eb). 구 my_defense_nexus=0.4.x stale.
            let mut scratch = [0u64; 8];
            // ★[07-23] rng·live 인자 신설. **검증(리턴훅) 경로 = `live=false` 고정** — 게임이 이미 picker draw를 소비했으므로
            //   여기서 true면 이중 소비 desync. rng(=r9=P4)는 넘기되 소비하지 않는다(live 게이트가 막음).
            return my_defense_nexus_050(scratch.as_mut_ptr() as usize, subp + 8, r8 as i64, r9, r14, r15, p7p, p7_dd, false);
        }
        // ★★[07-23] **code 지표 오배선 수정**: 게임 disc9는 out+0을 `movups`로 **`*(u64)(sub+0) = *(subp+8)` 통복사**한다(분기 없음).
        //   그런데 여기서 `o.active`(0/1)를 반환해 하네스(`detour.rs` `rd_i64(op)`)의 game_code(=out+0)와 **서로 다른 량을 비교**하고 있었다
        //   ⟹ mpcmp의 disc9 "2301 OK / 699 DIFF(77%)"는 **범주 오류**(둘 다 0/1 소값 도메인이라 우연 일치가 섞인 결과).
        //   `active`는 code가 아니라 **out+0x28 필드**이므로 검증은 `pokecmp`(full-output 바이트 대조)로만 해야 한다.
        //   disc5가 이미 정답 형태(`rd_i64(subp+8)`)라 그와 동일하게 맞춘다.
        9 => return rd_i64(subp + 8).unwrap_or(-99),   // ★0.5.2 EpicPoke: out+0 = *(subp+8) 통복사. 실대체·active는 emit의 epic_poke_write

        11 => return my_serpen_poke(subp + 8, r8, r14, r15, p7p, p7_dd), // ★0.5.0 SerpenPoke: char 반환
        10 => return my_epic_battle(subp + 8, r14, r15, p7p),           // EpicBattle: p2=subp+8, p5=r14(lanectx), p6=r15(geom), p7=p7p(threat)
        12 => return -99,   // SerpenBattle=out-writer(복합 aux) → code-return 부적합. 전용 capture(SERPEN_VERIFY) 브랜치서 my_serpen_battle 직접호출.
        16 => {   // ★07-10 disc16(idx14 인라인) REWRITE. out-writer지만 code {7,0x12,2}=mpcmp 직접검증. scratch out(게임 out 무오염).
            let mut scratch = [0u64; 8];
            return my_disc16(scratch.as_mut_ptr() as usize, r14, r15, subp);   // sim=param5=r14(엔티티), geom=param6=r15(컨텍스트), subplan=subp(raw, idx14 add rdx,8 안함)
        }
        17 => {   // ★07-10 disc17(idx15 핸들러 0x141c77f20) REWRITE. code {7,0x13}=mpcmp 직접검증. add rdx,8 있음→subplan=subp+8, param3=r8.
            let mut scratch = [0u64; 8];
            return my_disc17(scratch.as_mut_ptr() as usize, subp + 8, r8, r14, r15);   // sim=param5=r14, geom=param6=r15
        }
        15 => {   // ★07-10 disc15(idx13 핸들러 0x235d230) 정적완결(★표본 미발화=검증불가). code {7,0xB,0x10}. add rdx,8 있음. rng=r9, tp=p7p(arg7).
            let mut scratch = [0u64; 8];
            return my_disc15(scratch.as_mut_ptr() as usize, subp + 8, r8, r9, r14, r15, p7p);   // sim=param5=r14, geom=param6=r15, tp=param7=p7p
        }
        13 => {   // ★07-10 disc13(idx11 핸들러 0x1422d6d30, AttackNexus) 완전재RE(구 {7,0x11,2} 폐기). code {7,0xb,0xd}. add rdx,8 있음. rng=r9, ctx=p7p.
            let mut scratch = [0u64; 8];
            return my_disc13(scratch.as_mut_ptr() as usize, subp + 8, r8, r9, r14, r15, p7p);   // sim=param5=r14, geom=param6=r15, ctx=param7=p7p
        }
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
            let arg = rd_u64(r14 + 0x818).unwrap_or(0) as usize;   // 0.5.0: r14(SimState)+0x6a0→0x818
            let s = vt_slot(rvt, 0x138); if !ptr_ok(s) { return -99; }   // 0.5.0: vt 0x128→0x138
            let f: VtPtr2Fn = core::mem::transmute(s);
            let ent = f(robj, arg);
            if !ptr_ok(ent) || !readable(ent + 0x658, 8) || !readable(ent + 0x648, 8) { return -99; }
            let team = rd_u64(r14 + 0x810).unwrap_or(2);           // 0.5.0: r14+0x6a8→0x820  ★0.5.4 오프셋 이동 반영
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
// ★disc5(실명 LineTotal) 인라인 idx3(0x1419e4ca2) write-set 재현(07-11 disasm 확정, raw subp 기준·call/RNG 없음·straight-line).
//   out[0..0x10]=subp[8..0x18](∴out+0 code=*(u64)(subp+8))·out[0x10..0x20]=subp[0x60..0x70]·out+0x20=0·
//   out+0x28=(r8>=0xb)&subp[0x90]&(subp[0x93]!=1)·out+0x29=subp[0x90]·out+0x2a=0u16·out+0x2c=subp[0x93]. +0x2d~ 미터치(게임도 동일).
// ⛔★[07-22] **0.5.2 기준 이미 틀림 — 켜기 전 반드시 수정**: 0.5.2에서 disc5·disc6 원본의 `param_3(r8) >= 0xb` 게이트가
//   **양쪽 모두 삭제**됐다(0.5.0_3·0.5.1엔 존재). 0.5.2 원본: disc5 `out+0x28 = (byte[subp+0x93]!=1) & byte[subp+0x90]`
//   / disc6 `out+0x29 = byte[subp+0xa0]` — **둘 다 r8 항 없음**. 아래 재현은 r8 항이 잔존해 출력이 어긋난다.
//   현재 `MP_D56_REPL=0`(+ MP_SAFE_DISC 미포함)이라 이중으로 inert지만, 활성화 시 out 오염 → 07-22형 크래시 클래스.
unsafe fn mp_write_disc5(out: usize, subp: usize, r8: u64) -> bool {
    if !writable(out, 0x2d) || !readable(subp, 0x94) { return false; }
    for i in 0..0x10usize { std::ptr::write_unaligned((out + i) as *mut u8, rd_u8(subp + 8 + i)); }
    for i in 0..0x10usize { std::ptr::write_unaligned((out + 0x10 + i) as *mut u8, rd_u8(subp + 0x60 + i)); }
    std::ptr::write_unaligned((out + 0x20) as *mut u64, 0u64);
    let (b90, b93) = (rd_u8(subp + 0x90), rd_u8(subp + 0x93));
    std::ptr::write_unaligned((out + 0x28) as *mut u8, ((r8 >= 0xb) as u8) & b90 & ((b93 != 1) as u8));
    std::ptr::write_unaligned((out + 0x29) as *mut u8, b90);
    std::ptr::write_unaligned((out + 0x2a) as *mut u16, 0u16);
    std::ptr::write_unaligned((out + 0x2c) as *mut u8, b93);
    true
}
// ★disc6(실명 LineWait, 0.5.0 신설) 인라인 idx4(0x1419e4ba3) write-set 재현(07-11 disasm 확정, raw subp 기준).
//   out[8..0x18]=subp[8..0x18]·out[0x18..0x28]=subp[0x70..0x80]·out+0x28=subp[0xa3]·out+0x29=(r8>=0xb)&subp[0xa0]·
//   out+0x2a=*(u16)(subp+0xa4)·out+0=0xa(마지막 write=게임 순서 유지). +0x2c~ 미터치.
unsafe fn mp_write_disc6(out: usize, subp: usize, r8: u64) -> bool {
    if !writable(out, 0x2c) || !readable(subp, 0xa6) { return false; }
    for i in 0..0x10usize { std::ptr::write_unaligned((out + 8 + i) as *mut u8, rd_u8(subp + 8 + i)); }
    for i in 0..0x10usize { std::ptr::write_unaligned((out + 0x18 + i) as *mut u8, rd_u8(subp + 0x70 + i)); }
    std::ptr::write_unaligned((out + 0x28) as *mut u8, rd_u8(subp + 0xa3));
    std::ptr::write_unaligned((out + 0x29) as *mut u8, ((r8 >= 0xb) as u8) & rd_u8(subp + 0xa0));
    std::ptr::write_unaligned((out + 0x2a) as *mut u16, (rd_u8(subp + 0xa4) as u16) | ((rd_u8(subp + 0xa5) as u16) << 8));
    std::ptr::write_unaligned(out as *mut u64, 0xau64);
    true
}
// ★movepriority disc 0/1 인라인 출력 완전재현(dispatcher idx4 @0x1c38d81). p1=출력sret, p2=subplan(rdx), p3=param_3(r8).
//   [p1]=disc, [p1+8]=[p2+8], [p1+0x10..1f]=[p2+0x58..67](16B), [p1+0x20]=(p3>=0xb)&[p2+0x88], [p1+0x21]=[p2+0x8b]. +0x22~ 미터치(게임도 동일).
#[allow(dead_code)]   // ★07-11: 0.4.x 레거시 계약(0.5.0 disc6 계약은 mp_write_disc6) — 참조 보존, 호출자 폐기
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
#[allow(dead_code)]   // ★07-11: 구 disc8 좌석 폐기 — sub_a60 로직의 0.5.0 현행 좌석=disc11 Hide(§12.13.1 DONE). 참조 보존
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
// ★07-11 크래시 근본대책(§12.23 + §11.10/§11.9 스테이지1 규명): movepri 대체는 정규 매치 sim(stage2)에서만.
//   stage = sim 객체의 vtable "타입"이 인코딩(필드 아님): stage1=튜토리얼/축소 컨텍스트(itemnet 에이전트 부재라
//   대체출력이 게임을 미검증 "빌드 재계획" 경로로 유도→게임 자신이 NULL deref, §8 부류), stage2=정규+백그라운드 풀매치.
//   ★[0.5.2 07-22] 판별방식 전환: ~~vtable RVA 상수 비교~~ → **런타임 도출(vt+0x30 본체의 mov eax,kind 파싱)** 1차
//   + 상수표 2차 폴백 = disc4_vt30_kind() 참조(그 주석에 전환 사유·3버전 실측근거). shadow-call 無는 불변.
//   체인 미해석/미상 사본도 passthrough = fail-safe(바닐라 비트동일). 검증된 대체 컨텍스트(라이브·시즌sim)는 전부 stage2.
// ★[07-16] 판단 풀덤프 진단 — 관리팀 경기 1개의 매 프레임 10명 판단(subplan) 로그. 실제 시뮬때만 발화(재생 X).
//   ghidra 확정(0.5.1): P5=arg5(entry_rsp+0x28), P6=arg6(entry_rsp+0x30). sim=*(*P6)(경기잠금키),
//   handle=*(P5+0x818), team=*(P5+0x820), athlete_id=*(P5+0x810)(0.5.1 추론), tick=*(sim+0xeac0),
//   entity=dd7_slot128(sim,handle). 한 경기=rayon 워커 1스레드 순차라 thread-safe 단순.
static JUDGE_DUMP: AtomicU8 = AtomicU8::new(0);       // cfg judge_dump: 0=off / 1=관리팀 경기 / 2=모든 경기(필터無, 0x810검증용)
static JD_MATCHES: Mutex<Option<HashMap<usize, (u32, i64)>>> = Mutex::new(None);   // sim → (경기번호, last_tick). tick 리셋=새 경기(sim 재사용 분리)
static JD_MATCH_CNT: AtomicU32 = AtomicU32::new(0);   // 경기 순번 발급
static JD_DIR_INIT: AtomicBool = AtomicBool::new(false);   // match_log 폴더 1회 생성
const JD_MAX_MATCHES: usize = 64;                     // 동시 로그 경기 상한(폭주 방지)
// ★★[08-05 감사 정정] 이 함수가 받는 값은 **Plan**(movepri `0xc559e0` arg2, 0~17)인데
//   예전엔 **SubPlan 이름표**를 붙이고 있었다 = 라벨 버그. 그 탓에 judge_dump 로그의 "disc10 결사전"이
//   실제로는 **plan 10 = LineGanker**였고, "결사전이 일반 경기에서도 돈다"는 오판을 낳을 뻔했다.
//   ⚠**확인된 것만 이름을 쓴다.** 08-03 RE(plan-vs-subplan 두 enum 분리) + 08-05 감사에서
//   핸들러 RVA·패닉 Location 파일명으로 확정한 값뿐이고, 나머지는 추측하지 않고 번호로 남긴다
//   (오늘 접두사 개명 사고의 원인이 "확신 없는 이름을 정본처럼 쓴 것"이었다).
/// ★0.5.4: **Plan enum 번호가 통째로 −2 시프트**됐다.
///   디스패처 인덱스식에서 바이어스가 빠진 결과다: `idx = disc>=2 ? disc-2 : 1` → `idx = disc`.
///   이름 배열 16개(ForcePassive…DefenseNexus)는 순서·내용 그대로라 **변형이 사라진 게 아니라 번호만 밀렸다**.
///   (Battle 9→7, LineGankCover 11→9, AttackNexus 16→14, DefenseNexus 17→15, DeathMatchBattle 6→4 …)
///
///   모드 전역이 **0.5.3 번호 체계**로 쓰여 있으므로(`disc == 9 || disc == 11` 같은 비교가 20곳 넘는다),
///   흩어진 비교문을 하나씩 고치는 대신 **읽는 즉시 0.5.3 번호로 되돌린다**. 되돌리기도 이 한 줄이다.
///   ⚠SubPlan 번호는 **안 밀렸다**(디스패처 인덱스식·arm 순서·카테고리표 전부 불변). condgate 등
///     SubPlan 계열 disc 에는 절대 적용하지 말 것 — 두 번호공간을 같이 밀면 전부 어긋난다.
#[inline] fn plan_disc_053(raw: u64) -> u64 { raw.wrapping_add(2) }

fn plan_name(d: u64) -> &'static str {
    match d {
        4|5 => "단일라인",                 // old\single_line.rs — 일반 경기 발화 0
        6   => "결사전",                   // DeathMatchBattle — 일반 경기 발화 0
        7   => "정글",                     // passive_jungle 0xdff660
        10  => "라인갱커",                 // LineGanker (구 라벨 "결사전"이 여기서 나왔다)
        12  => "모르가드 사냥·견제",        // old\epic\hunt_and_poke.rs
        14  => "세르펜 사냥·견제",          // old\serpen\hunt_and_poke.rs
        16  => "넥서스공격",               // attack_nexus (인라인)
        17  => "넥서스방어",               // old\defense_nexus.rs
        _   => "plan?(미확정)",
    }
}
// out+0 이동명령 코드 = 실제 행동(disc보다 이게 진짜). concat 0x1438db490 확정.
fn move_name(c: u64) -> &'static str {
    match c { 0=>"히트맨",1=>"스킬",2=>"전진",3=>"도망",4=>"귀환",5=>"주변",6=>"포지셔닝",7=>"추적",8=>"라인",
        9=>"라인공격",10=>"라인대기",11=>"결사전",_=>"?" }
}
unsafe fn judge_dump_capture(saved: usize, entry_rsp: usize) {
    let mode = JUDGE_DUMP.load(Ordering::Relaxed);
    let p5 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;
    let p6 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;
    if !ptr_ok(p5) || !ptr_ok(p6) { return; }
    let l80 = rd_u64(p6).unwrap_or(0) as usize;
    if !ptr_ok(l80) { return; }
    let sim = rd_u64(l80).unwrap_or(0) as usize;
    if !ptr_ok(sim) || !readable(sim + 0xeb00, 8) { return; }
    let tick = rd_i64(sim + 0xeb00).unwrap_or(-1);   // 경기 진행 틱(단조증가). 리셋=새 경기.
    let aid = rd_u64(p5 + 0x810).unwrap_or(u64::MAX);   // 0.5.1 추론 오프셋(MY_ATHLETES 매칭되면 검증됨)
    let mine = { let p = MY_ATHLETES.load(Ordering::Acquire); !p.is_null() && (*p).contains(&aid) };
    // 경기별 번호 조회/등록: mode1=관리팀 경기만 / mode2=모든 경기. ★sim 재사용 분리: 같은 sim인데 tick이 확 줄면(리셋) 새 경기=새 파일.
    let mnum = {
        let mut g = JD_MATCHES.lock().unwrap_or_else(|e| e.into_inner());
        let map = g.get_or_insert_with(HashMap::new);
        match map.get_mut(&sim) {
            Some(slot) => {
                if tick < slot.1 - 600 {   // tick 리셋(600틱 이상 되감김) = 새 경기가 같은 sim 주소 재사용
                    let n = JD_MATCH_CNT.fetch_add(1, Ordering::Relaxed);
                    slot.0 = n; slot.1 = tick; n
                } else { if tick > slot.1 { slot.1 = tick; } slot.0 }
            }
            None => {
                if mode == 1 && !mine { return; }   // 관리팀 모드: 내 선수 발화 전엔 이 경기 등록 안 함
                if map.len() >= JD_MAX_MATCHES { return; }
                let n = JD_MATCH_CNT.fetch_add(1, Ordering::Relaxed);
                map.insert(sim, (n, tick)); n
            }
        }
    };
    if !JD_DIR_INIT.swap(true, Ordering::Relaxed) { if let Some(d) = pth("match_log") { let _ = fs::create_dir_all(&d); } }
    let handle = rd_u64(p5 + 0x818).unwrap_or(0);
    let team = rd_i64(p5 + 0x810).unwrap_or(-1);//  ★0.5.4 오프셋 이동 반영
    let p2 = rd_u64(saved + 0x20).unwrap_or(0) as usize;
    let disc = if ptr_ok(p2) && readable(p2, 8) { plan_disc_053(std::ptr::read_unaligned(p2 as *const u64)) } else { 0 };
    //   ★0.5.4: Plan 번호 −2 시프트 → 읽는 즉시 0.5.3 번호로 되돌린다(plan_disc_053).
    let ent = dd7_slot128(sim, handle);
    if !ptr_ok(ent) || !readable(ent + 0x660, 8) { return; }
    let cid = rd_i64(ent + 0x5a8).unwrap_or(-1);
    let x = rd_i64(ent + 0x648).unwrap_or(0); let y = rd_i64(ent + 0x650).unwrap_or(0);
    let hp = rd_i64(ent + 0x658).unwrap_or(0); let mhp = rd_i64(ent + 0x610).unwrap_or(1);
    let hpp = if mhp > 0 { hp * 100 / mhp } else { -1 };
    // ★[ghidra확정] 이동명령 kind = ent+0x6b0(int32, order-type enum {4,6,7,8,0xa..0x14}, 0/1=대기).
    //   플랜 상태 = ent+0x598(int32, subplan transition, 0/1=idle). 값체계 미확정 → raw로 관찰 후 이름매핑.
    let slot = if readable(ent + 0x708, 4) { rd_u32(ent + 0x708) as i64 } else { -1 };   // ★0.5.4(was 0x6b0)
    let plan = if readable(ent + 0x5e8, 4) { rd_u32(ent + 0x5e8) as i64 } else { -1 };   // ★0.5.4(was 0x598)
    //   ⚠★Plan **번호도 −2 시프트**됐다(Battle 9→7, DefenseNexus 17→15). 값을 해석해 쓰는 쪽은 같이 고칠 것.
    let line = format!("t{:>6} team{} cid{:<4}{} disc{}({}) 명령{} 플랜{} pos({},{}) hp{}%\n",
        tick, team, cid, if mine { "★" } else { " " }, disc, plan_name(disc), slot, plan, x, y, hpp);
    // 경기별 파일 (경기당 1스레드 순차라 파일단위 append 안전, lock 불필요)
    if let Some(p) = pth(&format!("match_log/match_{:02}.txt", mnum)) {
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = f.write_all(line.as_bytes()); }
    }
}
const MP_STAGE_DIAG: bool = false;  // [07-22] gvt 오프셋 자동탐색 진단 — **역할 완료**(체인 `g0+8` 정상 확인) → OFF.
// ★★[07-22 계측] MP_KIND_TALLY — "실경기 sim = kind0" 실측 후속. 게이트는 그대로 두고(대체 OFF 유지) **kind 분포만 수집**.
//   배경: `kind==2` 게이트는 실경기서 항상 false(= 사실상 대체 사망, 최소 0.5.1부터). 실경기=kind0은 2중 실측 확증
//   (0.5.1 comp_test engine_vt 0x38a66d8 / 0.5.2 movepri gvt 0x38c5d78, size 0xee88 교차) — 단 **튜토리얼/데모의 kind는 실측 0건**.
//   ⟹ kind0 허용으로 게이트를 열려면 "튜토리얼이 kind0이 아님"을 먼저 실측해야 한다(섞여 있으면 07-11 §12.23 크래시 재발).
//   이 계측은 그 확인 전용. **배포 전 false**(디버그 체크리스트). 목표 데이터가 모이면 게이트 전환 판단 후 제거.
// ⛔[07-22] **OFF 확정·재활성 금지**: 이 계측이 게임 크래시를 유발했다(튜토리얼→관리화면). 원인 = 전환 감지가
//   단일 스레드를 가정했는데 실제로는 **rayon 워커가 family A/B sim을 병렬 처리** → 매 ms 수십 번 "전환" 오인
//   → append_named 동기 IO 폭주(4.7MB/1판) → 게임 사망. 재계측이 필요하면 **파일 IO 없이 원자 카운터만** 누적하고
//   post_update(단일 스레드)에서 주기 스냅샷 1줄만 쓸 것.
const MP_KIND_TALLY: bool = false;
// ★[07-22 2차] 계측축을 kind → **gvt 주소(vtable 사본)** 로 변경.
//   1차 계측 결과: 튜토리얼도 kind0(270만 회 전량) ⟹ **kind만으로는 실경기/튜토리얼 구분 불가 = kind0 허용 게이트는 위험**.
//   단 gvt 주소는 갈렸다: 실경기=0x38c5d78(family B=live) / 튜토리얼 첫등장=0x383cd68(family A=serde 템플릿).
//   ⟹ family가 진짜 판별축인지 확정하려면 **주소별** 히스토그램이 필요(1차는 gvt를 첫등장에만 찍어 전환을 놓쳤음).
const GVT_SLOTS: usize = 8;
static GVT_HIST: [(AtomicUsize, AtomicU64); GVT_SLOTS] = [
    (AtomicUsize::new(0), AtomicU64::new(0)), (AtomicUsize::new(0), AtomicU64::new(0)),
    (AtomicUsize::new(0), AtomicU64::new(0)), (AtomicUsize::new(0), AtomicU64::new(0)),
    (AtomicUsize::new(0), AtomicU64::new(0)), (AtomicUsize::new(0), AtomicU64::new(0)),
    (AtomicUsize::new(0), AtomicU64::new(0)), (AtomicUsize::new(0), AtomicU64::new(0)),
];
static GVT_LAST: AtomicUsize = AtomicUsize::new(0);
static GVT_TOTAL: AtomicU64 = AtomicU64::new(0);
fn gvt_hist_dump() -> String {
    let b = unsafe { exe_base() };
    let mut s = String::new();
    for (a, c) in GVT_HIST.iter() {
        let a = a.load(Ordering::Relaxed);
        if a == 0 { break; }
        s.push_str(&format!(" 0x{:x}={}", a.wrapping_sub(b), c.load(Ordering::Relaxed)));
    }
    s
}
// 게이트 판정 전에 호출 — 전 컨텍스트를 집계(게이트 통과분만 세면 kind0이 안 잡힘).
#[inline] unsafe fn mp_kind_tally(k: Option<i64>, gvt: usize) {
    if !MP_KIND_TALLY { return; }
    // gvt별 카운터(슬롯 선점식, 락프리)
    for (a, c) in GVT_HIST.iter() {
        match a.load(Ordering::Relaxed) {
            0 => { if a.compare_exchange(0, gvt, Ordering::Relaxed, Ordering::Relaxed).is_ok() { c.fetch_add(1, Ordering::Relaxed); break; } }
            g if g == gvt => { c.fetch_add(1, Ordering::Relaxed); break; }
            _ => {}
        }
    }
    let n = GVT_TOTAL.fetch_add(1, Ordering::Relaxed) + 1;
    // ★gvt가 바뀌는 순간 = 컨텍스트 전환. 이걸 잡는 게 이번 계측의 목적(1차 설계 결함 보완).
    let prev = GVT_LAST.swap(gvt, Ordering::Relaxed);
    if prev != gvt {
        append_named("mp_kind_hist.txt", &format!(
            "[{}ms] {} gvt_rva=0x{:x} kind={:?}{}  | 주소별:{}\n",
            now_ms(), if prev == 0 { "★첫등장" } else { "↔★전환 " }, gvt.wrapping_sub(exe_base()), k,
            if prev == 0 { String::new() } else { format!(" (직전 0x{:x})", prev.wrapping_sub(exe_base())) },
            gvt_hist_dump()));
    } else if n % 50000 == 0 {
        append_named("mp_kind_hist.txt", &format!("[{}ms] ...주소별:{}\n", now_ms(), gvt_hist_dump()));
    }
}
static MP_STAGE_SKIP: AtomicU64 = AtomicU64::new(0);
#[inline] unsafe fn mp_stage2_ok(entry_rsp: usize) -> bool {
    let p6 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;     // param_6 = 컨텍스트 triple(geom)
    if !ptr_ok(p6) { return false; }
    let g0 = rd_u64(p6).unwrap_or(0) as usize;                    // geom[0]
    if !ptr_ok(g0) { return false; }
    let gvt = rd_u64(g0 + 8).unwrap_or(0) as usize;               // geom[0][1] = sim vtable
    if !ptr_ok(gvt) { return false; }
    mp_kind_tally(vt30_kind_cached(gvt), gvt);   // (계측 OFF 기본 — MP_KIND_TALLY 주석 참조)
    // ★★[07-22] 게이트 축 전환: ~~`kind == 2`(컨텍스트 추측)~~ → **sim 신원검증만**.
    //   폐기 근거(기록 재조사로 확정):
    //     ①§12.23.2 원문 = "**스테이지 게이트는 이 크래시엔 무효**(stage1만 차단인데 크래시는 stage2서 발생)
    //       → itemnet 가드가 유일 방어선". 즉 컨텍스트 축은 도입 당시부터 이 크래시를 못 막는다고 적혀 있었다.
    //     ②진짜 원인 = **disc4 재현 출력이 원본과 3곳 상이**(frontier_bail 중간 deref 누락 / ward argmin·base2 fallback 미구현
    //       / g2·g4 거리게이트의 원본에 없는 `ward!=0` 가드). `d4_repl=1`이면 100% 재현·`=0`이면 Bo3 완주 무크래시로 확정.
    //       07-12에 3곳 비트동일 수정 완료. ⟹ 위험의 축은 **컨텍스트가 아니라 "재현이 비트동일한가"**.
    //     ③소스에 남아 있던 "stage1=튜토리얼이라 위험" 주석은 07-11 밤~07-12 정정 **이전 문구**가 잔존한 것(폐기).
    //     ④실측상 실경기=kind0·튜토리얼도 kind0·family A/B는 병렬 동시 실행 ⟹ kind/vtable로는 컨텍스트 구분 자체가 불가.
    //   ⟹ 위험관리는 **disc별 토글(라이브 DIFF=0 실증분만 ON)** + **itemnet 가드**로 이관한다.
    //   여기 남기는 검증 = "정상 sim 객체인가"(vt+0x30 kind 도출 성공). 도출 실패(미상 vtable·읽기 실패)=불허 = fail-safe 유지.
    if vt30_kind_cached(gvt).is_some() { return true; }
    // 비정규 sim서 대체 skip — 발화 자체가 희귀 전제라 초회+간헐 로깅(감지용)
    let n = MP_STAGE_SKIP.fetch_add(1, Ordering::Relaxed) + 1;
    if n == 1 || n % 200 == 0 {
        // ★[07-22 임시진단·MP_STAGE_DIAG] 0.5.2서 stage 게이트 100% skip → gvt 도출 체인(+0x30→[0]→+8)이
        //   0.5.2에서 유효한지 확인. vt30_probe로 p6/g0/saved 주변을 훑어 **kind 패턴이 나오는 실제 오프셋을 자동탐색**.
        //   ⚠배포 전 제거(이 블록 + MP_STAGE_DIAG 상수). 초회+200회마다만 도므로 핫패스 영향 없음.
        let mut hit = String::new();
        if MP_STAGE_DIAG {
            let b = exe_base();
            for (lbl, root) in [("p6", p6), ("g0", g0)] {
                for off in (0..0x60usize).step_by(8) {
                    let c = match rd_u64(root.wrapping_add(off)) { Some(v) => v as usize, None => continue };
                    if !ptr_ok(c) { continue; }
                    if let Some(k) = vt30_probe(c) {   // c 자체가 vtable인 경우
                        hit.push_str(&format!(" [{}+0x{:x}]=vt0x{:x}:k{}", lbl, off, c.wrapping_sub(b), k));
                    }
                    if let Some(c2) = rd_u64(c.wrapping_add(8)) {   // c[1]이 vtable인 경우(현 체인 형태)
                        let c2 = c2 as usize;
                        if ptr_ok(c2) { if let Some(k) = vt30_probe(c2) {
                            hit.push_str(&format!(" [{}+0x{:x}][1]=vt0x{:x}:k{}", lbl, off, c2.wrapping_sub(b), k)); } }
                    }
                }
            }
            if hit.is_empty() { hit.push_str(" (탐색범위 내 kind패턴 0건)"); }
        }
        // ★[07-22] 이 경로 = **vt+0x30 kind 도출 실패(미상 sim vtable)만** 도달(정상 sim은 위에서 통과).
        //   종전 "stage!=2 전량 skip"과 달리 여기 누적이 늘면 = 미지의 sim 사본 등장 = 조사 필요 신호.
        append_named("mpcmp.txt", &format!(
            "[mp GATE] sim 신원검증 실패 skip 누적={} p6=0x{:x} g0=0x{:x} gvt=0x{:x} gvt_rva=0x{:x} probe={:?}{}\n",
            n, p6, g0, gvt, gvt.wrapping_sub(exe_base()), vt30_probe(gvt), hit));
    }
    false
}
/// ★[0.5.4 프로브] 경매 진입부 passthrough 래퍼 — `TeamPlan.version`(3번째 인자)만 세고 원본을 부른다.
///   version 은 `>=2` 게이트로 0.5.4 신규 판단들(경매 강제귀환·점수식 넥서스 게이트)을 여닫는데
///   정적 분석으로는 값을 못 밝혔다. 이 값만 알면 그 노브들의 기본값·설명이 확정된다.
///   ⚠읽기 전용. 원본을 **항상** 그대로 호출하므로 게임 동작·결정성에 영향 없다.
unsafe extern "C" fn auction_probe_capture(p1: usize, p2: usize, p3: usize, p4: usize, p5: usize, p6: usize,
                                           p7: usize, p8: usize, p9: usize, p10: usize, p11: usize, p12: usize) -> usize {
    if p3 < AUC_VER_HIST.len() { AUC_VER_HIST[p3].fetch_add(1, Ordering::Relaxed); }
    else { AUC_VER_BIG.fetch_add(1, Ordering::Relaxed); }
    let orig = ORIG_AUCTION.load(Ordering::Relaxed);
    let f: extern "C" fn(usize,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize) -> usize
        = core::mem::transmute(orig);
    f(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12)
}

unsafe extern "C" fn mp_capture(saved: usize, entry_rsp: usize) -> i64 {
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return 1; }
    let _jm = judge_mark(4);   // ★행진단: 하트비트+in-flight(movepri)
    if JUDGE_DUMP.load(Ordering::Relaxed) != 0 { let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| judge_dump_capture(saved, entry_rsp))); }   // ★판단 풀덤프(관리팀 경기 1개)
    if probe_on() { let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| probe_collect(saved, entry_rsp))); }   // ★[08-04] 진단 프로브 패시브 수집(IO 없음)
    // ★완전대체(mp_repl): disc 0/1 인라인출력 재현→대체(원본 dispatcher skip, rax=rcx=sret). 그 외 disc=passthrough(원본+capture).
    //   ★07-11: mp_stage2_ok 게이트 — stage!=2(튜토리얼 등)면 대체 전체 OFF(관측 캡처는 유지).
    if MP_REPL.load(Ordering::Relaxed) && mp_stage2_ok(entry_rsp) {
        let p1 = rd_u64(saved + 0x28).unwrap_or(0) as usize;
        let p2 = rd_u64(saved + 0x20).unwrap_or(0) as usize;
        // ★[07-19 최적화] readable(p2,8)(fast_guard=0시 VQ syscall)+raw read 2단 → rd_u64 1회 병합.
        //   폴트=None=기존 readable-false와 동일 분기(캠페인 B-2 패턴, 비트동일).
        let disc_rd = if ptr_ok(p1) && ptr_ok(p2) { rd_u64(p2).map(plan_disc_053) } else { None };
        //   ★0.5.4: Plan 번호 −2 시프트 → 0.5.3 번호로 되돌려서 아래 `disc == N` 비교를 그대로 쓴다.
        // ★★[07-22] MP_SAFE_DISC 화이트리스트 — **0.5.2 원본 write-set을 실제 disasm으로 대조해 "완전 일치" 확증된 disc만** 대체.
        //   도입 사유(실측 크래시 2026-07-22 18:59, 덤프 확증):
        //     out 버퍼 = 콜러 **스택 슬롯**(`[rbp+0x360]`, 0x30B, **제로화 안 됨**) → 반환 후 병합기 `FUN_141daf160`이
        //     **0x30 통째로** 영속 MovePriority(`agent+0x6b0`)에 복사한다. ⟹ 모드가 out의 **일부만** 쓰면 나머지는
        //     **직전 호출의 스택 잔재**가 그대로 커밋된다. 그 뒤 tag가 0/1/9인 상황에서 실행기 `FUN_142388fd0`이
        //     `byte[MP+0x2c]`(테이블 41엔트리)·`qword[MP+0x10]`(8엔트리)로 **상한체크 없이** 점프 → 명령 중간 착지 → `.text` 쓰기 AV.
        //     게임 원본이 안전한 이유 = 원본도 일부만 쓰지만 **그 variant가 실제로 쓰는 필드는 전부** 쓰기 때문.
        //   ⟹ "재현이 out write-set을 원본과 정확히 일치시키는가"가 유일한 안전 기준이다(컨텍스트·kind는 무관 — 폐기됨).
        //   현행 확증(ghidra-re 07-22, 0.5.2 실disasm + 0.5.0_3/0.5.1 3버전 대조):
        //     ✅disc2·8  = 원본 `mov [rsi],7` 2명령·3버전 바이트동일 / 모드 `wr_u64(p1,7)` **비트동일**
        //     ✅disc16   = 인라인 0x21342a4 / 모드 완전일치(code2시 +8,+9=byte[subplan+0x10],+0xa)
        //     ✅disc17   = 0x1b92e40, 원본도 out+0 단일 write / 모드 완전일치
        //     ⛔disc14  = **이번 크래시 최유력 원인**. 원본은 code별로 +8/+9/+0xa/+0x10/+0x11을 쓰는데 모드는 **out+0만** 기록
        //                 (소스 주석 "code-only·aux 미터치(게임도 동일)"는 **오기** — 0.5.0_3에서도 이미 틀렸음)
        //     ⛔disc12  = out은 일치하나 **payload Vec(+0..0x17)·flag(+0x1a) 재현 없음**(stale 힙포인터 순회 위험)
        //     ⛔disc12·14 공통 = 0.5.2서 `tick<0x21 → code 3` 분기 **삭제**됐는데 모드는 code3을 여전히 출력(초과)
        //     ⬜disc10·13 = **미감사**(disc13은 emit {7,0x11,2} vs 재현주석 {7,0xb,0xd} 불일치 의심) → 보수적으로 제외
        //   ⚠disc를 추가하려면 **반드시 0.5.2 원본 핸들러를 disasm해 write-set을 대조한 뒤에만** 넣을 것.
        //     "과거 400/400 검증·DIFF=0" 기록은 **0.5.0_3 시절 것**이라 근거로 쓸 수 없다(그 가정이 이 크래시를 냈다).
        //     ✅disc14  = **07-22 수정 후 편입**. 원본 `0x2118ef0` 전수 disasm으로 write-set 확정하고 재현을 일치시킴:
        //                 ①emit이 재현 결과를 버리던 것(scratch)을 **p1 직접 기록**으로 교체(+ out 0x30 제로화)
        //                 ②`sf` 오프셋 **0x3ea/0x3eb → 0x3f6/0x3f7**(0.5.2 시프트, 콜리 FUN_142117ae0 디컴 2점 확증)
        //                 ③**code 3 분기 삭제**(0.5.2서 `cmp rbx,0x21` 게이트 제거됨) ④**다이브추적 `level>0x2d` 게이트 삭제**(0.5.2서 제거)
        //                 ⑤**캐시 무효화 write 추가**(`payload+0x1a=0`·`+0x10=0`, cap/ptr 불변 — 순수 store라 안전)
        //                 힙 재구축 경로는 재현하지 않음(모든 출구가 code 7 수렴이라 출력 비트동일 + 게임 힙 소유권 금지).
        //                 ⬜첫 검증 = 6개 code(특히 0x14/2)의 out `+0..+0x11` game↔mine 대조 권장.
        //   ⛔**disc14 재제외(07-22 2차 크래시)**: 위 ①~⑤ 수정 후 편입했다가 **AV 재발**(RIP=exe+0x1c8f785, access=읽기,
        //     **faultAddr=0xffffffffffffffff(-1)**, 모드 crash_log 포착·덤프 `...77968.dmp`).
        //     ★**`faultAddr=-1`은 언더플로/NULL이 아니라 non-canonical 표식**: Windows가 non-canonical 접근 #GP를
        //       `AV(0, 0xffffffffffffffff)`로 변환 보고한다 ⟹ "레지스터에 쓰레기값이 들어가 그걸 주소로 썼다"를 먼저 의심할 것.
        //     실측(덤프 파싱): 폴트 = `0x1c8f770` 내 `call qword[r9+0x30]`, **R9 = exe+0x3830c58 = Rust 타입명 문자열 블롭 중간**
        //       (정상값은 `0x38c5d78` kind0 vtable). exe 전역에 그 주소 절대포인터·rip-rel **0건** ⟹ **순수 쓰레기값**.
        //       폴트 지점 성격 = 아이템/이펙트 **스탯 모디파이어 합산기**(상류 `0x1acad70`) = **disc14가 아니라 하류 소비 지점**.
        //     ~~"유력 원인 = ⑤ 캐시 무효화 write(len=0) 언더플로"~~ = **오판 정정(07-22)**: ⑤는 원본 `0x2119464`와 조건·대상·
        //       **순서(+0x1a byte→+0x10 qword)**·폴스루까지 **정확히 일치 = 무죄**. 언더플로 기각 3점: ①소비자 `0x1acad70`은
        //       진입 즉시 `TEST R12,R12; JZ`로 len==0 가드 ②**원본 스스로 len=0을 만들고 폴스루** ③언더플로 산술은 힙주소만
        //       낳지 `.rdata 0x3830c58`을 만들 수 없다. ⑥(out 제로화)도 0-write로 문자열 포인터 생성 불가 = 직접 원인 아님.
        //     ⟹ ★**"⑤·⑥만 되돌리면 안전"은 근거 없음**(그대로 재시도 시 재발 공산). 유력 = **①(재현 결과를 실제 out에 커밋)**
        //       — 이전엔 scratch에 버려 게임 무영향이었고 ①이 disc14 판단을 **사상 처음 실제 MovePriority에 반영**시켰다.
        //       + **모드 helper 내부 필드 오프셋의 0.5.2 미검증**(`geom_vt50=gc+0xeb08`·`s_ctx=gchild+0xeaf0` 등. shadow-call이
        //       아니라 순수 오프셋 계산이지만 stale이면 obj가 쓰레기 → 하류에서 문자열을 vtable로 호출 = 관측과 정합).
        //     ⚠부수 결함: ⑤ 코드의 `vlen<=16` 가드는 **원본에 없다**(vlen>16시 원본은 순회해 code7 가능) = 비트동일 위반.
        //     ⟹ 교훈: **명세에 없는 "권장/개선"을 재현에 얹지 말 것**(원본이 잔재를 남기면 잔재를 남기는 게 맞다) +
        //       **과거 검증된 재현을 새 버전에서 켤 때는 그 재현이 딛고 선 오프셋 체계 전체가 미검증임을 전제**할 것.
        //     재편입 조건(상세=MIGRATION §7.2-A4): ①되돌린 shadow 비교로 code별 out `+0..+0x2f` **0x30 전체** game↔mine 대조
        //       (지점=병합기 `0x1daf160` 진입 직전) → `vlen<=16` 제거 → helper 오프셋 0.5.2 검증 → shadow-call 게이트 규명
        //       → **한 항목씩** 재편입(5개 동시 적용이 이번 원인 분리를 불가능하게 만들었다).
        //   ✅**disc14 편입(07-23)** — 2차 제외 후 재편입. 근거가 이번엔 **실측**이다:
        //     ①~~**code 재현 400/400 OK·DIFF 0**(mpcmp 실측, C8C 복구 상태) ⟹ 판단 로직이 0.5.2에서 정확.~~
        //       → ★정정(0.5.2, 07-23 후반): ~~이 400/400은 무효 · ⬜재검증 대기.~~ disc12 편입작업으로 공유 콜리 3종을
        //       고쳤고(`serpen_engage_gate` tick 사전게이트 삭제 + UNIT 출처 ctx→mapobj / `serpen_rng_pick` 가용게이트 신설 + 상한완화 /
        //       `serpen_reposition_fight` tick 사전게이트 삭제 + 앵커 tag `4`→p4) 이들이 라이브 disc14 경로에서도 쓰인다.
        //       → ✅✅**재검증 통과(0.5.2, 07-23, dll 854B23F3): `d14_repl=0` passthrough+캡처로 subplan14 OK 400 / DIFF 0 / 크래시0**
        //       ⟹ 콜리 3종 수정이 전부 옳았음 실측 확인 = "400/400 검증완료" 정당 복원(정본 = MIGRATION §7.2-A8).
        //     ②**원본 write-set 실측이 명세와 정합**(mpws: `[disc=14] 0b1(+0)` / `0b11(+0,+8)` = 명세의 code 0x11→+0만·code 0x10→+0,+8과 일치).
        //     ③재현 함수 `my_defense_nexus_050`이 code별 명세 필드를 정확히 기록(0x14→+0/+8/+0x10/+0x11, 2→+0/+8/+9/+0xa 등).
        //     ④2·3차 크래시의 진범은 **C8C stale shadow-call**이었고 확정·복구됨(3차가 "대체 없이도 죽음"으로 ①~⑥ 무죄를 증명).
        //   적용된 수정: ①emit→p1 직결(+out 0x30 제로화) ②sf 0x3ea/0x3eb→0x3f6/0x3f7 ③code3 삭제 ④level>0x2d 게이트 삭제 ⑤캐시 무효화 write.
        //   ⚠out 제로화는 **원본과 비트동일은 아니다**(원본은 미기록 필드에 스택 잔재를 남김). 잔재 재현은 원리적으로 불가(직전 호출 의존)하고,
        //     0은 결정적이며 무경계 LUT(+0x2c/+0x10)에서 유효 인덱스라 **안전 우위**로 택했다. 병합기 특수 arm(code 0x14)이 +0x12/+0x16을
        //     복사하므로 그 두 필드에서 원본과 값이 갈릴 수 있음 — ⬜code 0x14 실관측 시 재확인 대상.
        //   ⬜미관측 code = **0x14 / 2 / 0xf**(이번 표본은 0x10·0x11만). 재현 소스는 명세와 일치하나 런타임 대조는 미완.
        //   ✅**disc9·10·11 편입(07-23)** — 0.5.2 원본 대조 후 수정 → **실측 검증 통과**:
        //     ✅disc9  = 오프셋 3종 마이그(`0x88/0x8c/0x8d` → **`0xBE/0xC4/0xC5`**) + p3 게이트 삭제 + code 지표 재배선
        //               ⟹ mpcmp **3000/3000 OK**(수정 전 77%) · pokecmp `[★DIFF@+0x29]` **2814 → 0**.
        //               write-set: `epic_poke_write`가 `+0x00~+0x2C` 전량 기록 = 원본 `0b101111`(+0,+8,+0x10,+0x18,+0x28) 커버 ✓
        //     ✅disc10 = 위험 shadow-call `vt_call1(vt,0x138,obj)`(슬롯 stale + rdx 미전달) → **`dd7_slot128` 순수재현**으로 교체
        //               ⟹ pending **191 → 0**, pokecmp OK. write-set: `+0=0xb, +8=weight, +0x10=1(u16), +0x12=0` = 원본 `0b111` 일치 ✓
        //     ✅disc11 = **원래 정상이었음**(제 오진). mpcmp가 disc11의 `out+0`(상수 `0xb`)을 판단값으로 읽은 계측 오배선이었고,
        //               진짜 지표인 pokecmp 바이트대조는 오답 0건. 오프셋 8종 전부 유효·로직 차집합 없음. write-set 0x14 커버 ✓
        //   ⚠**disc10·11의 mpcmp OK/DIFF는 무의미**(`out+0`이 `0xb` 고정이라 code 비교가 성립 안 함). 이 둘의 정본 지표는 **pokecmp**다.
        //   ✅**disc0·1·3 편입(07-23)** — ~~"emit 분기가 없어 편입 불가(대체 코드 자체가 부재)"~~ = **오진 정정**.
        //     emit은 `my_dd7700_full`로 **이미 완비**돼 있었고(T_G1/T_G2_6/T_G2_4/T_COVER/T_MAIN2), 배선 arm(아래 `disc==0||1||3`)도 완비,
        //     cfg도 `mp_repl=1`·`dd7_repl=1`이었다. 유일한 차단 = **이 화이트리스트에 0/1/3이 없어 위 `filter`가 떨군 것** = arm 전체가 死코드.
        //   ⚠단, 화이트리스트만 풀면 **유저 튜닝이 여전히 안 먹는다**. `my_dd7700_full`이 오늘 삭제 확정된 게이트 2개를 아직 들고 있었기 때문
        //     (`my_dd7700_code`에만 반영하고 full은 stale). 동반 수정 = ①`dd_early_p3_thr < p3` 삭제 + `!=0`→`==1`(L3349 부근)
        //     ②`dd_cover_p3_thr < p3` 삭제(COVER 블록 전체가 막혀 있었음). ⟹ 그 둘은 **확정 死레버**(설정편집기 제거 대상).
        //     ★실익: `dd_frontier_mult=350`·`dd_lane_margin=600`·`dd_cover_count=0`·`dd_ratio_thr=31`이 **전부 COVER 블록 안**이라 이제야 살아난다.
        //     (`dd_near_dist`는 COVER가 아니라 `my_dd7700_code` STAGE4(engage) 소관 → engage가 code 2를 낼 때만 반영 = **부분 적용**.)
        //   write-set 대조(07-23 ghidra-re 전수 disasm `0x1b91e40` vs `my_dd7700_full`) — 07-22 안전기준 통과:
        //     ✅T_G1 `+0`만 / ✅T_G2_6·T_G2_4 `+0,+8`(byte) / ✅T_COVER `+0,+8=2`(code 7인데 +8을 쓴다 — "code7=aux없음"으로 뭉뚱그리면 깨짐)
        //     ✅T_MAIN2 `+0,+8,+9,+0xa`(bl 2갈래 = 원본과 일치 확정, 상세=my_dd7700_full 위 명세블록의 07-23 정정)
        //     ✅SF경로(`plan==8`)·engage code 6/7 = `None` **passthrough** = 바닐라 비트동일(미포팅이 곧 안전)
        //     ✅`+0x0b..+0x2f` **미터치**(disc14와 달리 0-fill 안 함) — 원본도 잔재를 그대로 커밋하므로 미터치가 곧 비트동일.
        //   ⬜첫 인게임 검증 지표: `mpws.txt`의 `[disc=0/1/3]`이 **`0b11`(+0,+8) 이하**여야 함. bit2(+0x10) 이상이 켜지면 명세 불완전 ⟹ 즉시 화이트리스트에서 제외.
        //   ⬜disc12는 arm이 `-99` 하드코딩이라 미실행 + 테일 모델 상이 → 별도 과제.
        //   ✅**disc12 편입(07-23)** — 관문 4종 전부 해소 후 활성화. 근거:
        //     ①**테일 조건표 전문 확보**(0.5.2 `0x238f130` 전수 disasm, 정본=`ANA\disc12-epiccheck-tail-spec.md`)
        //     ②**재현 13건 수정 완료**(lane 오독·code3 삭제·[C]홈코너·[D]threat·[E]다이브·[F]HP소스·role7_count 재작성
        //       + 콜리 `rng_pick` 2건·`engage_gate` 4건). ③**콜리 3종 0.5.2 델타 확정**(reposition_fight 포함, RNG-free 실증)
        //     ④**write-set 07-22 안전기준 충족** — 종단 6종(0x14/0xc/7/2/0xd/0xe) 원본 대조 완료.
        //       ★인게임 실측 교차확증: `mpws.txt`의 게임 원본 `[disc=12]`가 `0b1`(+0만)·`0b11`(+0,+8)로 관측 = 명세 부합.
        //     ⚠**미구현 잔여 = [E] 재구축 경로**(`st.track==0` && 후보 있음) → 콜리 `0x1bd73a0` 미전개로 `st.len` 산출 불가라
        //       **`-99` passthrough**(게임 원본 수행) = 바닐라 비트동일 = 안전. 그 경로는 출구 3개 전부 code 7 수렴이라 출력 손실 없음.
        //     ⚠**picker `0x2135350` draw 수 동일성**이 desync 방지의 핵심 — (c)가용게이트 신설·임의상한 완화로 `n` 산출을 원본과 일치시킴.
        //   ★cfg `d12_repl`(기본1)로 재빌드 없이 격리 가능(d14_repl과 동일 패턴).
        const MP_SAFE_DISC: [u64; 12] = [0, 1, 2, 3, 8, 9, 10, 11, 12, 14, 16, 17];
        //   비화이트리스트 disc = None으로 떨궈 대체 블록 전체를 건너뛴다 = 원본 passthrough(기존 readable-false 분기와 동일 = 비트동일).
        //   ★[07-23] `d14_repl=0`이면 disc14를 여기서 떨궈 **passthrough+캡처**로 보낸다(= mpcmp에 disc14 판정줄 생성).
        //     대체 경로는 리턴훅을 안 거쳐 비교가 불가능하므로, 재검증에는 이 우회가 필수다.
        let disc_rd = disc_rd.filter(|d| MP_SAFE_DISC.contains(d)
            && !(*d == 14 && !D14_REPL.load(Ordering::Relaxed))
            && !(*d == 12 && !D12_REPL.load(Ordering::Relaxed)));
        if let Some(disc) = disc_rd {
            // ★[07-28 위치잔차 완결] out 0x30 제로화 — **잔재를 안 채우는 arm에만 한정**(disc0/1/3 dd7700 · disc12 · disc14).
            //   RE 확정(dd7700 전수 asm): native는 +0/+8/+9/+0xa만 쓰고 +0xb..+0x2f=콜러 스택 잔재(memset 없음).
            //   병합기 0x1daf160이 0x30 verbatim 커밋하나 그 바이트는 **dead**(다운스트림 미read)라 native는 스레드마다 잔재
            //   달라도 결정적. 모드는 **Rust 디투어 스택 잔재**라 두 sim(배경/관전)서 값이 달라 **비결정**(=위치 발산).
            //   native 잔재 재현은 불가(임의 스크래치)+불필요(dead) ⟹ 결정적 0으로 밀어 모드 자기결정성 확보.
            //   ⚠**전 arm 무차별 0-fill 금지**(07-28 RE 지적): disc9(`0x22ba290` +0~+0x2c 10필드)·disc11(`0x21343fa`+`0x23903a0`,
            //     +0/+8/+0x10/+0x12)은 **재현 write-set이 native와 이미 완전일치**라 out이 게임 콜러 스택인 이상 미기록분도
            //     native와 동일 잔재가 남는다 ⟹ 그걸 0으로 밀면 **바닐라 비트동일이 되레 깨진다**. passthrough(게이트 off) 시에도 동일.
            // ★★[07-29 복원] **전 arm 0-fill**(구: dd7/d12/d14 한정). 근거 = detlog tracer v2 실측:
            //   모드 판단은 전부 결정적(같은 IN→같은 OUT, IN✓OUT✗ 0건)인데 **입력 상태(HP/좌표)가 이른 tick부터 갈림**
            //   ⟹ 발산원은 "판단"이 아니라 **모드가 out 0x30에 남기는 미기록 잔재**(병합기 0x1daf160이 통째로
            //   MovePriority(agent+0x6b0)에 커밋 → 후속 프레임이 그 바이트를 읽으면 두 sim이 갈림).
            //   실측 이력도 일치: 전 arm 0-fill 시 챔프 발산 0/10 → dd7/d12/d14로 축소하자 25~33% 재발.
            //   ⚠"poke arm은 write-set이 native와 같으니 0-fill이 바닐라 충실도를 깬다"(RE 조언)는 이론적으로 옳으나,
            //     **결정성(관전==확정)이 우선**이고 대상 바이트는 판단 출력이 아닌 잔재라 sim 의미론 손실이 없다.
            for i in 0..6 { let _ = wr_u64(p1 + i * 8, 0); }
            // ★[07-29 detlog v2] mp 진입 **IN 기록** = 판단 입력 상태(disc·side·self HP/좌표·athlete_id) 해시.
            //   포인터 아닌 **값 기반**이라 두 sim 상태가 같으면 반드시 같은 해시 ⟹ IN 일치 = 상류 미발산 증명.
            let dl_site = match disc { 0 | 1 | 3 => 0, 2 | 8 => 1, 5 | 6 => 2, 7 => 3, 9 => 4, 10 => 5, 11 => 6, 12 => 7, 13 => 8, 14 => 9, 16 => 10, 17 => 11, _ => 14 };
            let mut dl_world = 0usize;
            let dl_rng = rd_u64(saved + 0x10).unwrap_or(0) as usize;   // P4 = RNG state(공용)
            if DL_ON.load(Ordering::Relaxed) {
                let r15d = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;
                let g0d = if ptr_ok(r15d) { rd_u64(r15d).unwrap_or(0) as usize } else { 0 };
                dl_world = if ptr_ok(g0d) { rd_u64(g0d).unwrap_or(0) as usize } else { 0 };
                let r14d = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;
                let sided = rd_i64(r14d + 0x810).unwrap_or(-1);//  ★0.5.4 오프셋 이동 반영
                let selfd = dd7_slot128(dl_world, rd_u64(r14d + 0x818).unwrap_or(0));
                let (hp, px, py) = if ptr_ok(selfd) {
                    (rd_i64(selfd + 0x658).unwrap_or(0) as u64, rd_u64(selfd + 0x648).unwrap_or(0), rd_u64(selfd + 0x650).unwrap_or(0))
                } else { (0, 0, 0) };
                let inh = (disc as u64) ^ ((sided as u64) << 8) ^ hp.rotate_left(13) ^ px.rotate_left(29) ^ py.rotate_left(43)
                          ^ rd_u64(r14d + 0x810).unwrap_or(0).rotate_left(7);   // athlete_id = 양 sim 동일 고유값
                dl_rec(dl_world, 0, inh ^ (dl_site as u64) << 56);   // ★ch0 = 상태(전 disc 통합)
                DL_WORLD_TL.with(|c| c.set(dl_world));               // cond/recall/numbers 훅용 World 전달
                // ★[07-29 v3] **RNG 스트림 추적**(site15=진입 상태 / site31=처리 후 상태).
                //   판단(IN/OUT)이 전부 결정적인데 상태(HP/좌표)가 갈린다 ⟹ 다음 용의자 = RNG 소비 불일치.
                //   판정: site15 IN✓ + site31 OUT✗ = **모드가 이 호출에서 draw를 다르게 소비**(직격 증거) /
                //         site15부터 IN✗ = RNG가 mp 밖(recall 등)에서 이미 갈림.
                if ptr_ok(dl_rng) {
                    dl_rec(dl_world, 1, rd_u64(dl_rng + 0x100).unwrap_or(0) ^ rd_u64(dl_rng + 0x130).unwrap_or(0).rotate_left(23));   // ★ch1 = RNG(idx,counter)
                }
            }
            if disc == 2 || disc == 8 {
                // ★07-11 배선: disc2(실명 LineDefense)/disc8(실명 Jungle — 구라벨 Cover) = 인라인 상수7 공유블록(0x1419e4aa8, JT idx0/idx6 동일타겟) 완전대체.
                //   write-set disasm 확정(07-11): out+0=7 단독(aux·call·RNG 없음·straight-line). disc8=0.5.0_3 인게임 400/400 검증(§12.20)·disc2=미발화(정적확정).
                //   (구 0.4.x 갱커버 sub_a60 로직의 현행 좌석=disc11 Hide(§12.13.1 DONE) — 구 disc8 레거시 블록은 07-11 폐기.)
                if wr_u64(p1, 7u64) {   // ★단일 write=부분쓰기위험0 → wr_u64(writable VQ 제거)
                    let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                    if disc == 2 || n % 500 == 0 {   // ★disc2=미발화 disc → 발화하면 전건 기록(감지용), disc8=고빈도라 500모듈로
                        if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                        append_named("mpcmp.txt", &format!("[mp REPL #{}] disc={}(인라인 상수7) code=7\n", n, disc));
                    }
                    apply_numbers_sp(disc as i64, entry_rsp, p1);
                    if DL_ON.load(Ordering::Relaxed) {
                        // ★[07-29 v5] ch5 = **출력 0x30 전량**(코드+aux 전부). 전 arm 0-fill이 걸려 잔재가 없으므로
                        //   전체 해싱해도 오탐 없음(구 "+8 오탐"의 원인=잔재는 제거됨). dd7 +8/+9/+0xa, poke +0x28~+0x2c,
                        //   d12/d14 +8/+0x10/+0x11 같은 **aux write가 그간 전부 미계측**이었다.
                        let mut oh = (dl_site as u64) << 56;
                        for i in 0..6 { oh ^= rd_u64(p1 + i * 8).unwrap_or(0).rotate_left((i * 7) as u32); }
                        dl_rec(dl_world, 5, oh);
                    }   // ★subplan별 numbers 후퇴(disc2/8)
                    return 0;   // HANDLED → rax=rcx=p1(sret), 원본 skip
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);
            } else if (disc == 5 || disc == 6) && MP_D56_REPL.load(Ordering::Relaxed) {
                // ★07-11 배선: disc5(실명 LineTotal, idx3 0x1419e4ca2)/disc6(실명 LineWait 0.5.0 신설, idx4 0x1419e4ba3) 인라인 풀아웃 완전대체.
                //   write-set=mp_write_disc5/6(07-11 disasm 확정, raw subp 기준·call/RNG 없음). ⚠둘 다 미발화 disc(§12.20 검증목록 부재) — 발화 시 게임 write-set 비트동일 대체.
                //   ★07-11 크래시대책①: cfg mp_d56_repl(기본0) 격리 — disc5/6=스테이지1/희귀 전용이라 대체=§8(미검증 컨텍스트 유도) 리스크만 존재. 기본=관측 passthrough(아래 캡처가 발화 로깅).
                let r8 = rd_u64(saved + 0x18).unwrap_or(0);
                let ok = if disc == 5 { mp_write_disc5(p1, p2, r8) } else { mp_write_disc6(p1, p2, r8) };
                if ok {
                    let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                    {   // ★disc5/6=미발화 disc → 발화 시 전건 기록(감지용; 발화=0 전제라 I/O 부담 없음)
                        if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                        append_named("mpcmp.txt", &format!("[mp REPL #{}] disc={}(인라인 풀아웃) 대체\n", n, disc));
                    }
                    apply_numbers_sp(disc as i64, entry_rsp, p1);
                    if DL_ON.load(Ordering::Relaxed) {
                        // ★[07-29 v5] ch5 = **출력 0x30 전량**(코드+aux 전부). 전 arm 0-fill이 걸려 잔재가 없으므로
                        //   전체 해싱해도 오탐 없음(구 "+8 오탐"의 원인=잔재는 제거됨). dd7 +8/+9/+0xa, poke +0x28~+0x2c,
                        //   d12/d14 +8/+0x10/+0x11 같은 **aux write가 그간 전부 미계측**이었다.
                        let mut oh = (dl_site as u64) << 56;
                        for i in 0..6 { oh ^= rd_u64(p1 + i * 8).unwrap_or(0).rotate_left((i * 7) as u32); }
                        dl_rec(dl_world, 5, oh);
                    }   // ★subplan별 numbers 후퇴(disc5/6)
                    return 0;   // HANDLED → rax=rcx=p1(sret), 원본 skip
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);
            } else if false {   // (구 disc8 레거시 블록 — 07-11 위 disc2||8 분기로 대체·항불진입)
                // ★disc 8: vt[0x128]게이트 + 헬퍼(0x142078a60) → 공유write(0x1c38edb): code10+서브코드(+8)+플래그(+0x10=1,+0x12=0).
                let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;   // p5 lanectx
                let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;   // p6 geom
                let rh = rd_u64(r15).unwrap_or(0) as usize;                 // *[p6] = 로스터 struct
                let mut done = false;
                // ★★07-10 비활성(0.4.x stale 레거시 = mp_repl 크래시 0x20ac708 원인): vt슬롯 0x128(0.5.0선 4인자 뮤테이터 0x20ac6c0, 리졸버는 0x138로 시프트)을
                //   2인자 호출→R8 쓰레기 AV. 핸들 +0x6a0/팀 +0x6a8도 0.5.0(+0x818/+0x820)과 불일치. disc8 out 인코딩 0.5.0 재RE 후 재활성 — 그때까지 passthrough(게임 핸들러 실행).
                if false && ptr_ok(rh) && ptr_ok(r14) && readable(rh, 16) {
                    let robj = rd_u64(rh).unwrap_or(0) as usize;
                    let cand = dd7_slot128(robj, rd_u64(r14 + 0x818).unwrap_or(0));   // (재활성시) vt0x138 순수재현 + 0.5.0 핸들오프셋
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
                if done { apply_numbers_sp(disc as i64, entry_rsp, p1); return 0; }   // ★subplan별 numbers 후퇴(disc8)
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);
            } else if disc == 7 && D7_REPL.load(Ordering::Relaxed) {
                // ★disc7(Recall) 라이브 대체(§12.22 my_disc7). 기본 D7_REPL=false(원본). code {7,8}·code8만 aux(+8=P48=payload+0x48/+0x10=role=payload+0x60 u8/+0x11=0). add rdx,8→payload=p2+8.
                let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;   // arg5 gholder
                let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;   // arg6 ctxpair
                let code = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_disc7(r15, r14, p2))).unwrap_or(-99);
                if code != -99 && wr_u64(p1, code as u64) {   // ★probe+write: 첫필드 wr_*로 writability확인→성공시 aux raw
                    if code == 8 {
                        let pl = p2 + 8;
                        std::ptr::write_unaligned((p1 + 8) as *mut u64, rd_u64(pl + 0x48).unwrap_or(0));
                        std::ptr::write_unaligned((p1 + 0x10) as *mut u8, rd_u8(pl + 0x60));
                        std::ptr::write_unaligned((p1 + 0x11) as *mut u8, 0u8);
                    }
                    let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                    if n % 500 == 0 {
                        if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                        append_named("mpcmp.txt", &format!("[mp REPL #{}] disc=7(Recall) code={}\n", n, code));
                    }
                    apply_numbers_sp(disc as i64, entry_rsp, p1);
                    if DL_ON.load(Ordering::Relaxed) {
                        // ★[07-29 v5] ch5 = **출력 0x30 전량**(코드+aux 전부). 전 arm 0-fill이 걸려 잔재가 없으므로
                        //   전체 해싱해도 오탐 없음(구 "+8 오탐"의 원인=잔재는 제거됨). dd7 +8/+9/+0xa, poke +0x28~+0x2c,
                        //   d12/d14 +8/+0x10/+0x11 같은 **aux write가 그간 전부 미계측**이었다.
                        let mut oh = (dl_site as u64) << 56;
                        for i in 0..6 { oh ^= rd_u64(p1 + i * 8).unwrap_or(0).rotate_left((i * 7) as u32); }
                        dl_rec(dl_world, 5, oh);
                    }   // ★subplan별 numbers 후퇴(disc7 귀환)
                    return 0;   // HANDLED → rax=rcx=p1(sret)
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);
            } else if disc == 14 {
                // ⛔★★[07-22 정정] ~~"code-only(디컴 *param_1=code만) … +0만 write, aux 미터치(**게임도 동일**)"~~ = **오기(誤記)**.
                //   0.5.2 원본 헬퍼 `0x2118ef0` 실disasm: code0x14 → +8(q)=0·**+0x10(b)=1**·+0x11(b)=byte[payload+0x18] /
                //   code0xf·0x10 → +8(b)=0 / code2 → +8(b)=0·+9(b)=al·+0xa(b)=2. 즉 **게임은 aux를 쓴다**(0.5.0_3 0x1c88090도 이미 그랬음).
                //   이 오기가 "게임도 +0만 쓴다"는 잘못된 전제를 만들어 **2026-07-22 AV 크래시**를 통과시켰다:
                //   out은 콜러 **스택 슬롯**(제로화 안 됨)이고 병합기 0x1daf160이 **0x30을 통째로** MovePriority(agent+0x6b0)에 복사하므로,
                //   +0만 쓰면 나머지는 **직전 호출 잔재**가 커밋 → tag 0/1/9에서 `byte[MP+0x2c]`(41엔트리 무경계 테이블) 오착지 → `.text` 쓰기 AV.
                //   ⟹ ~~**disc14는 MP_SAFE_DISC에서 제외(현재 대체 불가)**~~ → ★**편입 성공(07-23, 정본=MIGRATION §7.2-A7)**: 아래 5건 수정 후 code ~~**400/400 DIFF=0**~~(→~~07-23 후반 공유 콜리 3종 수정으로 무효·⬜재검증 대기~~ → ✅✅**재검증 통과**(0.5.2, 07-23, dll 854B23F3, `d14_repl=0` passthrough+캡처 400/400 DIFF0·크래시0, 정본=MIGRATION §7.2-A8). 편입=활성 유지)·인게임 641회 발화·크래시0. (①emit→p1 직결 ②sf 0x3f6/0x3f7 ③code3 분기 삭제 ④level>0x2d 게이트 삭제 ⑤캐시 무효화 write) ⚠2·3차 크래시의 진범은 이 항목들이 아니라 **C8C stale shadow-call**이었다. 이하 '재활성 조건'은 이력.
                //      **code 3 분기 제거**(0.5.2서 `tick<0x21 → code3` 삭제됨) 후 0.5.2 원본과 write-set 재대조.
                // (원 서술 보존) disc14 = 실명 EpicPoke(구라벨 DefenseNexus). my_movepriority→my_defense_nexus_050.
                let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;
                let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;
                let r8 = rd_u64(saved + 0x18).unwrap_or(0);
                let r9 = rd_u64(saved + 0x10).unwrap_or(0) as usize;        // P4 = RNG state (★07-23 `_r9`→사용: RNG 홀 수정)
                let p7_dd = rd_u64(entry_rsp + 0x40).unwrap_or(0) as usize;
                let p7p = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize;
                // ★★[07-22] **단일 write 폐기 → 실제 out(p1) 직접 기록**. 이 `wr_u64(p1, code)` 단독이 크래시 원인이었다:
                //   `my_defense_nexus_050`은 **이미 out-writer**로서 code별 완전 write-set(+0/+8/+9/+0xa/+0x10/+0x11)을 스스로 쓴다.
                //   그런데 라우팅이 그 결과를 버리는 로컬 `scratch`에 쓰게 해놓고(검증기 잔재 "게임 out 무오염") emit은 +0만 기록 →
                //   out 나머지(0x30 버퍼, **콜러 스택이라 제로화 안 됨**)에 직전 호출 잔재가 남고 병합기 `0x1daf160`이 그걸 통째 커밋.
                //   ⟹ my_movepriority(14,…) 경유(scratch)를 **건너뛰고 재현 함수를 p1으로 직접 호출**한다.
                //   ★0-fill 선행: 원본도 미기록 필드엔 잔재를 남기지만(그 필드는 병합기 기준 dead), 0이 잔재보다 엄격히 안전·결정적.
                //   -99 조기반환 경로는 전부 `ptr_ok` 가드 구간(어떤 wr_* 보다 앞)이라 p1 전달해도 부분오염 없음(원본 대조 확인).
                for i in 0..6 { let _ = wr_u64(p1 + i * 8, 0); }   // out 0x30 제로화
                // ★★[07-23] **RNG 홀 수정 = 여기가 `live=true` 유일 지점**(대체 경로). 원본 picker(`0x2135350`)의 Lemire draw를
                //   재현이 소비하지 않아 RNG 스트림이 게임보다 뒤처지던 desync를 막는다. 검증 경로(my_movepriority disc14 arm)는 false.
                //   ⚠`code == -99`(passthrough) 경로는 전부 draw **이전**의 ptr_ok 가드라 이중 소비 없음(serpen.rs 사이트 주석 참조).
                let code = my_defense_nexus_050(p1, p2 + 8, r8 as i64, r9, r14, r15, p7p, p7_dd, true);
                if code != -99 {   // ★wr_u64(p1, code) 금지 — 재현 함수가 이미 +0 포함 전 필드를 기록했다
                    let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                    if n % 500 == 0 {
                        if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                        append_named("mpcmp.txt", &format!("[mp REPL #{}] disc=14(EpicPoke·code-only) code={}\n", n, code));
                    }
                    apply_numbers_sp(disc as i64, entry_rsp, p1);
                    if DL_ON.load(Ordering::Relaxed) {
                        // ★[07-29 v5] ch5 = **출력 0x30 전량**(코드+aux 전부). 전 arm 0-fill이 걸려 잔재가 없으므로
                        //   전체 해싱해도 오탐 없음(구 "+8 오탐"의 원인=잔재는 제거됨). dd7 +8/+9/+0xa, poke +0x28~+0x2c,
                        //   d12/d14 +8/+0x10/+0x11 같은 **aux write가 그간 전부 미계측**이었다.
                        let mut oh = (dl_site as u64) << 56;
                        for i in 0..6 { oh ^= rd_u64(p1 + i * 8).unwrap_or(0).rotate_left((i * 7) as u32); }
                        dl_rec(dl_world, 5, oh);
                    }   // ★subplan별 numbers 후퇴(disc14 EpicPoke)
                    return 0;   // HANDLED
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);
            } else if disc == 13 {
                // ★disc13(실명 EpicHunt — 구라벨 AttackNexus) 인라인(0x1c38c98) 완전대체. 출력계약 disasm확정:
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
                        append_named("mpcmp.txt", &format!("[mp REPL #{}] disc=13(EpicHunt) code={}\n", n, code));
                    }
                    apply_numbers_sp(disc as i64, entry_rsp, p1);
                    if DL_ON.load(Ordering::Relaxed) {
                        // ★[07-29 v5] ch5 = **출력 0x30 전량**(코드+aux 전부). 전 arm 0-fill이 걸려 잔재가 없으므로
                        //   전체 해싱해도 오탐 없음(구 "+8 오탐"의 원인=잔재는 제거됨). dd7 +8/+9/+0xa, poke +0x28~+0x2c,
                        //   d12/d14 +8/+0x10/+0x11 같은 **aux write가 그간 전부 미계측**이었다.
                        let mut oh = (dl_site as u64) << 56;
                        for i in 0..6 { oh ^= rd_u64(p1 + i * 8).unwrap_or(0).rotate_left((i * 7) as u32); }
                        dl_rec(dl_world, 5, oh);
                    }   // ★subplan별 numbers 후퇴(disc13 EpicHunt)
                    return 0;   // HANDLED → rax=rcx=p1(sret)
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);
            } else if disc == 15 && D15_REPL.load(Ordering::Relaxed) {
                // ★[07-16] disc15(SerpenCheck) 라이브 대체 — d15_repl=1 opt-in(재현 미검증·표본부족이라 기본 OFF). 출력 {7,0xb,0x10}. code0xb만 aux(+8=u64,+0x10=1). my_disc15에 real 인자(rng=r9,tp=p7p) 전달.
                let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;
                let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;
                let r8 = rd_u64(saved + 0x18).unwrap_or(0);
                let r9 = rd_u64(saved + 0x10).unwrap_or(0) as usize;
                let p7p = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize;
                let mut scratch = [0u64; 8];
                let code = my_disc15(scratch.as_mut_ptr() as usize, p2 + 8, r8, r9, r14, r15, p7p);
                if code != -99 && wr_u64(p1, code as u64) {
                    if code == 0xb {
                        std::ptr::write_unaligned((p1 + 8) as *mut u64, scratch[1]);
                        std::ptr::write_unaligned((p1 + 0x10) as *mut u8, 1u8);
                        std::ptr::write_unaligned((p1 + 0x11) as *mut u8, 0u8);
                        std::ptr::write_unaligned((p1 + 0x12) as *mut u8, 0u8);
                    } else if code == 0x10 {
                        std::ptr::write_unaligned((p1 + 8) as *mut u8, 0u8);
                    }
                    let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                    if n % 500 == 0 {
                        if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                        append_named("mpcmp.txt", &format!("[mp REPL #{}] disc=15(SerpenCheck·opt-in) code={}\n", n, code));
                    }
                    apply_numbers_sp(disc as i64, entry_rsp, p1);
                    if DL_ON.load(Ordering::Relaxed) {
                        // ★[07-29 v5] ch5 = **출력 0x30 전량**(코드+aux 전부). 전 arm 0-fill이 걸려 잔재가 없으므로
                        //   전체 해싱해도 오탐 없음(구 "+8 오탐"의 원인=잔재는 제거됨). dd7 +8/+9/+0xa, poke +0x28~+0x2c,
                        //   d12/d14 +8/+0x10/+0x11 같은 **aux write가 그간 전부 미계측**이었다.
                        let mut oh = (dl_site as u64) << 56;
                        for i in 0..6 { oh ^= rd_u64(p1 + i * 8).unwrap_or(0).rotate_left((i * 7) as u32); }
                        dl_rec(dl_world, 5, oh);
                    }
                    return 0;
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);
            } else if (disc == 16 || disc == 17) && D1617_REPL.load(Ordering::Relaxed) {
                // ★★[07-31] disc16/17 **대체 실효성 A/B 실험용 토글**(cfg `nx_repl`, 기본 1=종전 동작).
                //   배경: `force_sp19`로 my_disc17 반환을 9,217회 전부 0x13(SubPlan19)으로 강제했는데
                //         **게임 disc19 핸들러가 한 번도 안 돌았고 AI 행동도 눈에 띄게 안 바뀌었다**
                //         ⟹ "disc16/17 대체가 게임에 반영되지 않을 수 있다"는 의심(추정).
                //   실험: 같은 리플레이(=시드 재시뮬레이션이라 결정론적)를 이 값 0/1로 각각 관전해
                //         결과가 갈리면 대체가 실효 있는 것, 같으면 무효인 것.
                //   ⚠전체 `mp_repl`을 끄면 disc0/1/3/9/11 효과까지 섞여 인과를 못 가린다 ⟹ **16/17만** 격리한다.
                // ★07-11 배선: disc16(실명 SerpenHunt, idx14 인라인)/disc17(실명 SerpenPoke, 0x1c77f20) 라이브 대체 — 재현 400/400 검증완료(§12.20), out 계약 disasm 확정(07-11):
                //   disc16: 7/0x12=code-only, 2=code+aux3(+8=0,+9=byte[subp+0x10 raw],+0xa=2 — 0.4.x rh-slot 계약 유지 확인) / disc17: 항상 code-only {7,0x13}. 둘 다 RNG-free.
                let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;
                let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;
                let r8 = rd_u64(saved + 0x18).unwrap_or(0);
                let r9 = rd_u64(saved + 0x10).unwrap_or(0) as usize;
                let p7_dd = rd_u64(entry_rsp + 0x40).unwrap_or(0) as usize;
                let p7p = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize;
                let code = my_movepriority(disc, r15, r14, p2, r8, r9, p7_dd, p7p);
                if code != -99 && wr_u64(p1, code as u64) {
                    if disc == 16 && code == 2 {
                        std::ptr::write_unaligned((p1 + 8) as *mut u8, 0u8);
                        std::ptr::write_unaligned((p1 + 9) as *mut u8, rd_u8(p2 + 0x10));
                        std::ptr::write_unaligned((p1 + 0xa) as *mut u8, 2u8);
                    }
                    let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                    if n % 500 == 0 {
                        if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                        append_named("mpcmp.txt", &format!("[mp REPL #{}] disc={}({}) code={}\n", n, disc, if disc == 16 { "SerpenHunt16" } else { "SerpenPoke17" }, code));
                    }
                    apply_numbers_sp(disc as i64, entry_rsp, p1);
                    if DL_ON.load(Ordering::Relaxed) {
                        // ★[07-29 v5] ch5 = **출력 0x30 전량**(코드+aux 전부). 전 arm 0-fill이 걸려 잔재가 없으므로
                        //   전체 해싱해도 오탐 없음(구 "+8 오탐"의 원인=잔재는 제거됨). dd7 +8/+9/+0xa, poke +0x28~+0x2c,
                        //   d12/d14 +8/+0x10/+0x11 같은 **aux write가 그간 전부 미계측**이었다.
                        let mut oh = (dl_site as u64) << 56;
                        for i in 0..6 { oh ^= rd_u64(p1 + i * 8).unwrap_or(0).rotate_left((i * 7) as u32); }
                        dl_rec(dl_world, 5, oh);
                    }   // ★subplan별 numbers 후퇴(disc16/17 SerpenHunt·Poke)
                    return 0;   // HANDLED → rax=rcx=p1(sret)
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);
            } else if disc == 4 {
                // ★disc4(실명 LineSafe — 구라벨 PassiveJungle, 0x206e530) 완전대체. cfg d4_repl=0이면 passthrough(freeze 격리). coord_pass subp+8 수정됨. d4freeze=1이면 my_disc4 단계별 d4last.txt.
                if D4_REPL.load(Ordering::Relaxed) {
                    let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;
                    let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;
                    let code = my_movepriority(4, r15, r14, p2, 0, 0, 0, 0);
                    if code != -99 && write_disc4_aux(p1, code, p2 + 8) {
                        let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                        if n % 500 == 0 {
                            if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                            append_named("mpcmp.txt", &format!("[mp REPL #{}] disc=4(LineSafe) code={}\n", n, code));
                        }
                        apply_numbers_sp(disc as i64, entry_rsp, p1);
                    if DL_ON.load(Ordering::Relaxed) {
                        // ★[07-29 v5] ch5 = **출력 0x30 전량**(코드+aux 전부). 전 arm 0-fill이 걸려 잔재가 없으므로
                        //   전체 해싱해도 오탐 없음(구 "+8 오탐"의 원인=잔재는 제거됨). dd7 +8/+9/+0xa, poke +0x28~+0x2c,
                        //   d12/d14 +8/+0x10/+0x11 같은 **aux write가 그간 전부 미계측**이었다.
                        let mut oh = (dl_site as u64) << 56;
                        for i in 0..6 { oh ^= rd_u64(p1 + i * 8).unwrap_or(0).rotate_left((i * 7) as u32); }
                        dl_rec(dl_world, 5, oh);
                    }   // ★subplan별 numbers 후퇴(disc4 정글)
                        return 0;   // HANDLED → rax=rcx=p1(sret)
                    }
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);   // d4_repl=0 또는 write실패 → passthrough(게임 dispatcher 실행)
            } else if (disc == 9 || disc == 11) && POKE_REPL.load(Ordering::Relaxed) {
                // ★disc9/11(실명 Battle/Hide — 구라벨 Epic/SerpenPoke, §11.8) 0.5.0 완전재작성 대체(RNG-free, §12.13): 0.5.0 poke는 RNG draw 없음(0.4.14 e88a0/writeback 유산 제거=desync 위험 소멸).
                //   disc9(Battle)=epic_poke_write(out-struct 0x2d 직접기록) · disc11(Hide)=char 반환→write_serpen_out(out+0=0xb/+8=char/+0x10=1) 래핑.
                let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;   // P5 = SimState
                let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;   // P6 = roster holder
                let r8 = rd_u64(saved + 0x18).unwrap_or(0);                 // P3 = phase(epic)
                let handled = if disc == 9 {
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| epic_poke_write(p1, p2 + 8, r8, r14, r15))).unwrap_or(false)
                } else {
                    let c = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_serpen_poke(p2 + 8, r8, r14, r15, 0, 0))).unwrap_or(-99);
                    writable(p1, 0x14) && write_serpen_out(p1, c)
                };
                if handled {
                    let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                    if n % 500 == 0 {
                        if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                        append_named("mpcmp.txt", &format!("[mp REPL #{}] disc={}({}) poke-rewrite(RNG-free)\n", n, disc, if disc==9 {"Battle9"} else {"Hide11"}));
                    }
                    apply_numbers_sp(disc as i64, entry_rsp, p1);
                    if DL_ON.load(Ordering::Relaxed) {
                        // ★[07-29 v5] ch5 = **출력 0x30 전량**(코드+aux 전부). 전 arm 0-fill이 걸려 잔재가 없으므로
                        //   전체 해싱해도 오탐 없음(구 "+8 오탐"의 원인=잔재는 제거됨). dd7 +8/+9/+0xa, poke +0x28~+0x2c,
                        //   d12/d14 +8/+0x10/+0x11 같은 **aux write가 그간 전부 미계측**이었다.
                        let mut oh = (dl_site as u64) << 56;
                        for i in 0..6 { oh ^= rd_u64(p1 + i * 8).unwrap_or(0).rotate_left((i * 7) as u32); }
                        dl_rec(dl_world, 5, oh);
                    }   // ★subplan별 numbers 후퇴(disc9/11 오브젝트견제)
                    return 0;   // HANDLED → rax=rcx=p1(sret)
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);
            } else if (disc == 0 || disc == 1 || disc == 3) && DD7_REPL.load(Ordering::Relaxed) {
                // ★07-10: 0.5.0 renumber — disc0/1/3 전부 idx1 dd7700(동일 핸들러·base=subp raw). 구 disc3 단독+p2+8은 0.4.x 잔재 정정.
                // ★dd7700(PassiveLine) 완전대체(2026-06-20 RNG-sync 검증완료 DIFF=0/21500): my_dd7700_full(전체출력, dd7full DIFF=0) + my_dd7700_rng_final writeback(p4 RNG 전진=skip시 no-desync). None(engage 6/7·plan8 rare)→passthrough. panic-safe.
                let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;
                let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;
                let r8 = rd_u64(saved + 0x18).unwrap_or(0);
                let r9 = rd_u64(saved + 0x10).unwrap_or(0) as usize;   // = dd7700 param_4 = RNG state
                let p7_dd = rd_u64(entry_rsp + 0x40).unwrap_or(0) as usize;
                // ★위치잔차 0-fill = 디투어 진입부(위)로 통합(전 arm 공통). 여기 중복 제거.
                // ① 출력 재현(p4 read-only). 성공시에만 대체(실패=passthrough → 원본 dd7700이 출력+RNG 자체수행).
                let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_dd7700_full(p1, p2, r8, r9, r14, r15, p7_dd))).unwrap_or(None);   // ★07-10: base=subp raw(0.5.0 idx1 add rdx,8 안 함 — 구 p2+8=0.4.x 잔재)
                if let Some(consumes_rng) = res {
                    // ②★레버: engage(consumes_rng=true)만 rng_final 호출. cover·main(false)=RNG 0 draw 확정 → rng_final 통째 skip(중복 cover검출/role/sim_hdr 1회 제거 = native급 단일순회). draw 0이라 state 불변=비트동일.
                    if consumes_rng {
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        if let Some((fidx, refills, buf)) = my_dd7700_rng_final(r9, p2, r8, r14, r15, p7_dd) {   // ★07-10: base=subp raw
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
                    // ③ ★base output code 존중 후퇴: 게임이 낸 out code(p1) 읽고 Move(2)=라인워크면 numbers_threat_move, 교전/귀환(4/6/7)이면 numbers_threat.
                    // ★[수정 07-31] 게이트에 `NUMBERS_THREAT_SP_ANY` 추가 — 구 게이트는 전역 numbers/tower 키가 하나도 없으면 닫혀서
                    //   **`numbers_threat_sp0/1/3`만 설정한 유저는 라인전 경로가 통째로 미실행**이었다(apply_numbers_sp 게이트와 불일치).
                    if TOWER_THREAT.load(Ordering::Relaxed) > 0 || NUMBERS_THREAT.load(Ordering::Relaxed) > 0 || NUMBERS_THREAT_MOVE.load(Ordering::Relaxed) >= 0 || NUMBERS_MARGIN.load(Ordering::Relaxed) > 0 || NUMBERS_THREAT_SP_ANY.load(Ordering::Relaxed) || CLASS_ANY.load(Ordering::Relaxed) || POS_PROBE.load(Ordering::Relaxed) || CLASS_PROBE.load(Ordering::Relaxed) {
                        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            let base_code = rd_u8(p1);
                            let l80 = rd_u64(r15).unwrap_or(0) as usize;
                            let sim = rd_u64(l80).unwrap_or(0) as usize;
                            let side = rd_i64(r14 + 0x810).unwrap_or(-1);   // ★07-10: 0.5.0 오프셋(구 0x6a8=0.4.x)  ★0.5.4 오프셋 이동 반영
                            let selfe = dd7_slot128(sim, rd_u64(r14 + 0x818).unwrap_or(0));   // ★07-10: 0.5.0 핸들(구 0x6a0)
                            // ★[수정 07-31] disc 하드코딩 `3` → 실제 `disc`. 이 아암은 disc **0/1/3** 공용인데 항상 3으로 넘겨
                            //   `numbers_threat_sp0`·`sp1`이 영구 무반영이었고 진단 집계도 전부 슬롯3에 뭉개졌다.
                            //   ※현행 cfg·default.txt에 `sp0/1/3` 줄이 없어(전부 -1=폴백) **오늘 시점 동작 변화는 0**.
                            if ptr_ok(selfe) && laner_should_retreat(r15, side, selfe, r14, base_code, disc as i64) {
                                std::ptr::write_unaligned(p1 as *mut u64, 7u64);
                                SP_SEEN[(disc as usize).min(17)].fetch_add(1, Ordering::Relaxed);
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
            } else if disc == 10 {
                // ★disc10(실명 DeathBattle — 구라벨 EpicBattle, §11.8) 0.5.0 대체(재작성): my_epic_battle=weight(i64) → out+0=0xb/+8=weight/+0x10=1(u16)/+0x12=0. RNG-free.
                let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;   // P5 SimState(side@+0x820)
                let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;   // P6 geom(gs=*P6)
                let w = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_epic_battle(p2 + 8, r14, r15, 0))).unwrap_or(-99);
                if w != -99 && writable(p1, 0x14) {
                    wr_u64(p1, 0xbu64);
                    std::ptr::write_unaligned((p1 + 8) as *mut u64, w as u64);
                    std::ptr::write_unaligned((p1 + 0x10) as *mut u16, 1u16);
                    std::ptr::write_unaligned((p1 + 0x12) as *mut u8, 0u8);
                    let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                    if n % 500 == 0 {
                        if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                        append_named("mpcmp.txt", &format!("[mp REPL #{}] disc=10(EpicBattle) weight={}\n", n, w));
                    }
                    apply_numbers_sp(disc as i64, entry_rsp, p1);
                    if DL_ON.load(Ordering::Relaxed) {
                        // ★[07-29 v5] ch5 = **출력 0x30 전량**(코드+aux 전부). 전 arm 0-fill이 걸려 잔재가 없으므로
                        //   전체 해싱해도 오탐 없음(구 "+8 오탐"의 원인=잔재는 제거됨). dd7 +8/+9/+0xa, poke +0x28~+0x2c,
                        //   d12/d14 +8/+0x10/+0x11 같은 **aux write가 그간 전부 미계측**이었다.
                        let mut oh = (dl_site as u64) << 56;
                        for i in 0..6 { oh ^= rd_u64(p1 + i * 8).unwrap_or(0).rotate_left((i * 7) as u32); }
                        dl_rec(dl_world, 5, oh);
                    }
                    return 0;   // HANDLED
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);
            } else if disc == 12 {
                // ⛔★[07-22] **MP_SAFE_DISC 제외 — 현재 대체 불가**. 0.5.2 원본 헬퍼 `0x238f130` 실disasm 결과 2건 불일치:
                //   ①**payload write 전량 누락**: 원본은 `payload+0x1a`(bool)·`payload+0x10`(len)·`movups payload+0..0xf`={cap,ptr}
                //     = **Vec{cap,ptr,len}** 갱신 + 기존 Vec `__rust_dealloc`을 수행하는데 재현은 out만 쓴다
                //     ⟹ 다음 틱 FLEE 판정이 **stale 힙 포인터/길이로 진행**(해제된 엔티티 순회 위험).
                //   ②**code 3 초과**: 0.5.2서 `tick<0x21 → code3` 분기가 **삭제**됨(0.5.0_3·0.5.1엔 존재). 0.5.2 출력집합={0x14,0xc,7,0xd,0xe,2}.
                //   ⟹ 아래 "검증 DIFF=0"은 **0.5.0_3 시절 기록이라 0.5.2에선 무효**(code3 경로 한정). 재활성 = payload Vec 재현 + code3 제거 후 재대조.
                // (원 서술) ★disc12(실명 EpicCheck — 구라벨 SerpenBattle, §11.8) 0.5.0 라이브 대체(07-10): my_serpen_battle(~~검증 DIFF=0~~, shadow-free)로 out-struct 재현.
                //   entry 스냅샷(m18/m19/W큐)은 캡처 경로가 채우나 mp_repl은 캡처 미도달 → 여기서 라이브 mem에서 직접 채움(entry시점=게임핸들러 소비 전이라 live==snapshot).
                //   ①(fe2500 RNG 소비)는 my_serpen_battle 내부 serpen_rng_pick(live=true)이 Lemire 드로우로 스트림 전진. ②(FLEE mem+0x1a write)=미구현 TODO(아래 주석).
                let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;   // P5 = SimState(sim)
                let r15 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;   // P6 = geom(roster holder)
                let r8  = rd_u64(saved + 0x18).unwrap_or(0);                // P3 = tick
                let r9  = rd_u64(saved + 0x10).unwrap_or(0) as usize;       // P4 = rng state
                let p7_dd = rd_u64(entry_rsp + 0x40).unwrap_or(0) as usize; // P8 = sf(+0x3ea/+0x3eb)
                let p7p   = rd_u64(entry_rsp + 0x38).unwrap_or(0) as usize; // P7 = tp(engage 게이트 인자)
                // ★★[07-28 결정성 수정] entry 스냅샷을 **로컬로 계산해 인자 직결**(구: MP_AUX_* 전역 store→함수가 load).
                //   전역 경유는 배경 rayon 워커 다수 + 관전 sim 동시 진입 시 lost-update(남의 경기 값 read) ⟹ 리액티브 분기 갈림 = 비결정 발산.
                let aux_m18 = rd_u8(p2 + 8 + 0x18) as u64;
                let aux_m19 = rd_u8(p2 + 8 + 0x19) as u64;
                let (aux_wl, aux_wh) = {
                    let g0e = rd_u64(r15).unwrap_or(0) as usize;
                    let gce = if ptr_ok(g0e) { rd_u64(g0e).unwrap_or(0) as usize } else { 0 };
                    let (mut wl, mut wh) = (0u64, 0u64);
                    if ptr_ok(gce) {
                        let we = gce + 0xeb30;
                        wl = rd_u64(we + 0x1a8).unwrap_or(0);
                        if wl != 0 {
                            let ha = rd_u64(we + 0x1a0).unwrap_or(0) as usize;
                            if ptr_ok(ha) { wh = rd_u64(ha).unwrap_or(0); } else { wl = 0; }
                        }
                    }
                    (wl, wh)
                };
                // ⬜TODO ②(FLEE 상태추적 mem write): FLEE 신규진입시 mem+0x1a=1 + threat 리스트 heap Vec 클론(+0/+8/+0x10), 유지실패시 mem+0x1a=0/len=0.
                //    현 my_serpen_battle에 FLEE 분기 자체가 부재(검증 DIFF=0 출력은 mem+0x1a 무의존) → 잘못된 조건에 플래그 write시 다음틱 FLEE 오판 위험.
                //    ⇒ 1차 배선에선 미구현(안전). 결과: 게임이 FLEE 진입/이탈하는 틱에서 다음 틱 FLEE 유지판정이 어긋날 수 있음(즉시 출력은 무영향).
                let code = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_serpen_battle(p1, p2 + 8, r8 as i64, r9, r14, r15, p7p, p7_dd, true, Some((aux_m18, aux_m19, aux_wl, aux_wh))))).unwrap_or(-99);
                if code != -99 {
                    let n = MP_REPL_N.fetch_add(1, Ordering::Relaxed) + 1;
                    if n % 500 == 0 {
                        if !MP_FILE_INIT.swap(true, Ordering::Relaxed) { write_named("mpcmp.txt", "=== facet#4 movepriority ④ 완전대체(mp_repl) ===\n"); }
                        append_named("mpcmp.txt", &format!("[mp REPL #{}] disc=12(SerpenBattle) code={}\n", n, code));
                    }
                    apply_numbers_sp(disc as i64, entry_rsp, p1);
                    if DL_ON.load(Ordering::Relaxed) {
                        // ★[07-29 v5] ch5 = **출력 0x30 전량**(코드+aux 전부). 전 arm 0-fill이 걸려 잔재가 없으므로
                        //   전체 해싱해도 오탐 없음(구 "+8 오탐"의 원인=잔재는 제거됨). dd7 +8/+9/+0xa, poke +0x28~+0x2c,
                        //   d12/d14 +8/+0x10/+0x11 같은 **aux write가 그간 전부 미계측**이었다.
                        let mut oh = (dl_site as u64) << 56;
                        for i in 0..6 { oh ^= rd_u64(p1 + i * 8).unwrap_or(0).rotate_left((i * 7) as u32); }
                        dl_rec(dl_world, 5, oh);
                    }   // ★[수정 07-16] disc12(EpicCheck) numbers 후퇴 override(disc9/11 패턴 동일). sp12 미배선 해소
                    return 0;   // HANDLED → rax=rcx=p1(sret), 원본 dispatcher skip
                }
                MP_REPL_PASS.fetch_add(1, Ordering::Relaxed);   // -99(재현불가) → passthrough(게임 원본 실행; 아직 미드로우 지점이라 RNG double-draw 없음)
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
    let di = (disc as usize).min(17);   // ★07-10 min(15)→min(17): disc16/17 별도 슬롯
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
    // ★mp_observe=1: my 재현 스킵(전부 pending 기록) — 미검증 스테이지 컨텍스트서 shadow-call/raw-read AV 차단. 평시(=0)엔 panic만 가드(AV는 vt_call 정합성에 의존=기검증 컨텍스트 전제).
    let my = if MP_OBSERVE.load(Ordering::Relaxed) { -99 }
        else { std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_movepriority(disc, r15, r14, subp, r8, r9, p7_dd, p7p))).unwrap_or(-99) };
    let diag: i64 = if disc == 14 { DEF_DIAG.load(Ordering::Relaxed) as i64 } else { -99 };
    let orig_ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    if !ptr_ok(orig_ret) || !readable(entry_rsp, 8) { return 1; }
    let pre = if disc == 0 || disc == 1 || disc == 3 {   // ★DIAG: dd7700 리턴경로 태그(disc0 my=4≠g=2 추격)
        format!("[mp #{}] subplan={} my={} path={}", MP_ARMED.load(Ordering::Relaxed), disc, my, DD7_PATH.load(Ordering::Relaxed))
    } else { format!("[mp #{}] subplan={} my={}", MP_ARMED.load(Ordering::Relaxed), disc, my) };
    // ★출력계약 진단: 진입시 *out 8qword 스냅 → 리턴(kind7)서 diff = sub-judge write-set.
    if kind == 7 && readable(out, 0x40) {
        for k in 0..8usize { MP_ENTRY[k].store(rd_u64(out + k*8).unwrap_or(0), Ordering::Relaxed); }
        MP_ENTRY_PTR.store(out, Ordering::Relaxed);
        // ★disc9/11 full-output 검증용 aux입력 보관(hook_return kind7서 write_poke_aux 재현 대조).
        if disc == 9 || disc == 11 || disc == 10 || disc == 12 { MP_AUX_OP.store(out, Ordering::Relaxed); MP_AUX_P2.store(subp + 8, Ordering::Relaxed); MP_AUX_P6.store(r15, Ordering::Relaxed); MP_AUX_P3.store(r8, Ordering::Relaxed); MP_AUX_P5.store(r14, Ordering::Relaxed); }
        if disc == 12 {
            MP_AUX_SF.store(p7_dd, Ordering::Relaxed); MP_AUX_RNG.store(r9, Ordering::Relaxed); MP_AUX_TP.store(p7p, Ordering::Relaxed);   // ★disc12 SerpenBattle 추가입력(sf/rng/tp)
            MP_AUX_M18.store(rd_u8(subp + 8 + 0x18) as u64, Ordering::Relaxed);   // ★one-shot 리액티브 플래그 entry 스냅샷(게임핸들러 실행 전 값)
            MP_AUX_M19.store(rd_u8(subp + 8 + 0x19) as u64, Ordering::Relaxed);
            // ★W 오더큐 entry 스냅샷(07-10): 게임핸들러가 오더 소비 → 리턴훅 시점 len=0으로 target-null 오판(0xc vs 0xd 아티팩트). m18/19와 동일 패턴.
            {
                let g0e = rd_u64(r15).unwrap_or(0) as usize;
                let gce = if ptr_ok(g0e) { rd_u64(g0e).unwrap_or(0) as usize } else { 0 };
                let (mut wl, mut wh) = (0u64, 0u64);
                if ptr_ok(gce) {
                    let we = gce + 0xeb30;
                    wl = rd_u64(we + 0x1a8).unwrap_or(0);
                    if wl != 0 {
                        let ha = rd_u64(we + 0x1a0).unwrap_or(0) as usize;
                        if ptr_ok(ha) { wh = rd_u64(ha).unwrap_or(0); } else { wl = 0; }
                    }
                }
                MP_AUX_WLEN.store(wl, Ordering::Relaxed);
                MP_AUX_WH.store(wh, Ordering::Relaxed);
            }
            let sn = SERPEN_ENTRY_N.fetch_add(1, Ordering::Relaxed) + 1;
            if sn % 200 == 0 {   // ★진단 덤프(SERPEN_VERIFY 무관하게 disc12 발화 확인)
                let mut s = format!("disc12 entry={} SERPEN_VERIFY={} MPCAP={}\n인자: sim(athlete)=0x{:x} geom=0x{:x} sf=0x{:x} mem(subp+8)=0x{:x}\n",
                    sn, SERPEN_VERIFY.load(Ordering::Relaxed), MPCAP.load(Ordering::Relaxed), r14, r15, p7_dd, subp + 8);
                s.push_str("DIAG[0=out0가드/1g0/2gchild/3side/4selfe/5maxhp/6=0x14/7=0xc/8=7/9=0xd/10=2:3/11=0xe/12총/13ozone/14izone/15tag1/16tgtfull/17sf3eb3/18sf3ea/19engage/20react무오더/21react타겟0/22main오더有/23[E]재구축passthru/24geom0/25sim0/26mem0]:\n");
                for i in 0..27 { s.push_str(&format!("[{}]={} ", i, SERPEN_DIAG[i].load(Ordering::Relaxed))); }
                write_named("serpendiag.txt", &s);
            }
        }
        else if disc == 4 { MP_AUX_OP.store(out, Ordering::Relaxed); MP_AUX_P2.store(subp + 8, Ordering::Relaxed); MP_AUX_P6.store(r15, Ordering::Relaxed); }   // ★disc4 param_2=subp+8(디스패처 add rdx,8 확인). aux=*(p2+0x48)active/*(p2+0x60)facet
        else if disc == 14 { MP_AUX_C1A.store(rd_u8(subp + 8 + 0x1a) as u64, Ordering::Relaxed); }   // ★disc14 다이브추적 pre-state 스냅샷(cmd+0x1a, 게임 갱신 전)
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



// ── 결정 ──
#[derive(Clone, Debug)]
struct PlanAi;
impl ModPlayerInputAi for PlanAi {
    fn clone_box(&self) -> Box<dyn ModPlayerInputAi> { Box::new(self.clone()) }
    fn id(&self) -> &str { "tfm2_ai_adjust_ai" }
    fn think(&mut self, ctx: &mut PlayerAiContext<'_,'_,'_>, base_input: Option<Input>) -> PlayerInputDecision {

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

// ════════════════ Phase 3: 인게임 편집 모달 (선수별 판단 편집기) ════════════════
// ★07-11: 인게임 모달 비활성(유저요청). 넥서스(oi_*)·numbers_threat_sp 섹션은 편집기에 추가돼 있으나 진입만 차단.
//   cfg 텍스트 편집은 무관하게 유효. 재활성 시 true로.
const AIADJ_MODAL_ENABLED: bool = false;
static AIADJ_FH_LEN: AtomicUsize = AtomicUsize::new(usize::MAX);
static AIADJ_MODAL_OPEN: AtomicBool = AtomicBool::new(false);
static AIADJ_SEL_PLAYER: AtomicI64 = AtomicI64::new(-1);
static AIADJ_LOADED_PLAYER: AtomicI64 = AtomicI64::new(-2);
static AIADJ_SAVE_REQ: AtomicBool = AtomicBool::new(false);
static AIADJ_RESET_IDX: AtomicI64 = AtomicI64::new(-1);   // 기본값 버튼 클릭한 항목(전역값 리셋)
static AIADJ_MSG: Mutex<String> = Mutex::new(String::new());
fn ui_set_visible(root: &mut Node, id: &str, on: bool) -> bool {
    if root.id == id { root.visible = on; return true; }
    for c in root.child.iter_mut() { if ui_set_visible(c, id, on) { return true; } }
    false
}
// ── 라이브 노드 읽기(item_tactics 방식): 선수명(라벨)+챔프(아이콘 이미지소스) ──
fn ui_find<'a>(n: &'a Node, id: &str) -> Option<&'a Node> {
    if n.id.as_str() == id { return Some(n); }
    for c in n.child.iter() { if let Some(x) = ui_find(c, id) { return Some(x); } }
    None
}
unsafe fn runner_base(n: &Node) -> usize {
    let any: &dyn std::any::Any = n.runner.as_any();
    let parts: [usize; 2] = std::mem::transmute::<*const dyn std::any::Any, [usize; 2]>(any as *const dyn std::any::Any);
    parts[0]
}
unsafe fn read_runner_string(n: &Node, type_sub: &str, off: usize) -> Option<String> {
    if !n.runner.type_name().contains(type_sub) { return None; }
    let dp = runner_base(n);
    let len = std::ptr::read_unaligned((dp + off) as *const u64) as usize;
    let ptr = std::ptr::read_unaligned((dp + off + 8) as *const u64) as *const u8;
    if ptr.is_null() || len == 0 || len > 512 || !readable(ptr as usize, len) { return None; }
    Some(String::from_utf8_lossy(std::slice::from_raw_parts(ptr, len)).to_string())
}
unsafe fn row_champion(row: &Node) -> Option<String> {
    let icon = ui_find(row, "icon")?;
    let src = read_runner_string(icon, "ImageRunner", 0)?;   // "…/champions/{champ}#sheet"
    let a = src.find("champions/")? + "champions/".len();
    let rest = &src[a..];
    let end = rest.find('#').unwrap_or(rest.len());
    let champ = rest[..end].trim();
    if champ.is_empty() { None } else { Some(champ.to_string()) }
}
unsafe fn row_player(row: &Node) -> Option<String> {
    let nm = ui_find(row, "name")?;
    read_runner_string(nm, "LabelRunner", 352)
}
fn sanitize(s: &str) -> String {
    s.chars().map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' }).collect()
}
// 전역값 맵(default.txt 위에 tfm2_ai_adjust.cfg 오버레이) — 프리필/변경판정용.
static AIADJ_GLOBAL: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);
fn build_global_map() -> HashMap<String, String> {
    let mut m: HashMap<String, String> = HashMap::default();
    for p in [pth("config/default.txt"), pth("tfm2_ai_adjust.cfg")] {   // 활성이 기본 오버라이드
        if let Some(p) = p { if let Ok(txt) = fs::read_to_string(&p) {
            for line in txt.lines() {
                let t = line.trim();
                if t.is_empty() || t.starts_with('#') { continue; }
                if let Some((k, v)) = t.split_once('=') { let v = v.split('#').next().unwrap_or("").trim();
                    if !v.is_empty() { m.insert(k.trim().to_string(), v.to_string()); } }
            }
        } }
    }
    m
}
// 현재 편집중 선수 챔피언의 클래스(melee/range/…). post_update서 db.champion_info로 세팅.
static AIADJ_CUR_CLASS: Mutex<String> = Mutex::new(String::new());
fn cur_class_name() -> String { AIADJ_CUR_CLASS.lock().map(|c| c.clone()).unwrap_or_default() }
// 전역 유효값 = 클래스오버라이드(키_class_<cls>) ?? 전역. DLL 적용순서(champ>class>global)와 일치.
fn eff_global<'a>(m: &'a HashMap<String, String>, cls: &str, key: &str) -> Option<&'a String> {
    if !cls.is_empty() { if let Some(v) = m.get(&format!("{}_class_{}", key, cls)) { return Some(v); } }
    m.get(key)
}
fn global_get(key: &str) -> Option<String> {
    let cls = cur_class_name();
    AIADJ_GLOBAL.lock().unwrap_or_else(|e| e.into_inner()).as_ref().and_then(|m| eff_global(m, &cls, key).cloned())
}
// 파일 키 = athleteID_champion (이름중복 방지). athlete_id = db.player_team().last_starting[row].
unsafe fn cur_target(u: &GameUI, ath: &[i64; 5]) -> Option<(i64, String, PathBuf)> {
    let n = AIADJ_SEL_PLAYER.load(Ordering::Relaxed);
    if n < 0 || n >= 5 { return None; }
    let row = ui_find(&u.root, &format!("row{}", n))?;
    let champ = row_champion(row)?;
    let aid = ath[n as usize];
    if aid < 0 { return None; }
    let d = dir()?.join("players");
    let _ = fs::create_dir_all(&d);
    Some((aid, champ.clone(), d.join(format!("{}_{}.cfg", aid, sanitize(&champ)))))
}
// 모달 열릴 때(또는 선수 변경시): 개별 ?? 전역 ?? 빈칸 프리필 + 타이틀.
unsafe fn editor_load(u: &mut GameUI, ath: &[i64; 5]) {
    *AIADJ_GLOBAL.lock().unwrap_or_else(|e| e.into_inner()) = Some(build_global_map());
    let n = AIADJ_SEL_PLAYER.load(Ordering::Relaxed);
    let (player, champ) = if n >= 0 && n < 5 {
        match ui_find(&u.root, &format!("row{}", n)) {
            Some(r) => (row_player(r).unwrap_or_default(), row_champion(r).unwrap_or_default()),
            None => (String::new(), String::new()),
        }
    } else { (String::new(), String::new()) };
    let title = format!("AI 판단 설정 — {} ({})", if player.is_empty() { "선수".to_string() } else { player }, champ);
    // 개별 오버라이드 파싱
    let mut pp: HashMap<String, String> = HashMap::default();
    if let Some((_, _, path)) = cur_target(u, ath) {
        if let Ok(txt) = fs::read_to_string(&path) {
            for line in txt.lines() { let t = line.trim();
                if t.is_empty() || t.starts_with('#') || t.starts_with("__") { continue; }
                if let Some((k, v)) = t.split_once('=') { pp.insert(k.trim().to_string(), v.split('#').next().unwrap_or("").trim().to_string()); }
            }
        }
    }
    if let Some(tn) = ui_kit::find_mut(&mut u.root, "aiadj_title") { ui_kit::label_set(tn, &title); }
    for (i, key) in knobs::KNOBS.iter().enumerate() {
        let val = pp.get(*key).cloned().or_else(|| global_get(key)).unwrap_or_default();   // 개별 ?? 전역 ?? 빈칸
        if let Some(f) = ui_kit::find_mut(&mut u.root, &format!("aiadj_f{}", i)) { ui_kit::textedit_set(f, &val); }
    }
    ui_kit::scroll_set_by_id(&mut u.root, "aiadj_scroll", 0.0);   // ★열 때마다 스크롤 맨 위로
    if let Ok(mut m) = AIADJ_MSG.lock() { m.clear(); }
}
// 저장: players/<athleteID>_<champ>.cfg. 전역과 다른 값만 저장(같으면 전역 따름). 기능영속 write=디버그로거와 분리.
unsafe fn editor_save(u: &mut GameUI, ath: &[i64; 5]) {
    let Some((aid, champ, path)) = cur_target(u, ath) else {
        if let Ok(mut m) = AIADJ_MSG.lock() { *m = "저장 실패: 선수/챔프 인식 불가(경기전 화면?)".into(); } return;
    };
    let n = AIADJ_SEL_PLAYER.load(Ordering::Relaxed);
    let player = if n >= 0 && n < 5 { ui_find(&u.root, &format!("row{}", n)).and_then(|r| row_player(r)).unwrap_or_default() } else { String::new() };
    let mut s = format!("# {} / {} 판단 오버라이드 (인게임 편집기)\n__athlete = {}\n__player = {}\n__champion = {}\n\n", player, champ, aid, player, champ);
    let mut cnt = 0;
    for (i, key) in knobs::KNOBS.iter().enumerate() {
        if let Some(f) = ui_kit::find_mut(&mut u.root, &format!("aiadj_f{}", i)) {
            if let Some(v) = ui_kit::textedit_get(f) {
                let v = v.trim();
                if !v.is_empty() && v.parse::<i64>().is_ok() && global_get(key).as_deref() != Some(v) {
                    s.push_str(&format!("{} = {}\n", key, v)); cnt += 1;   // 전역과 다를때만 = 진짜 오버라이드
                }
            }
        }
    }
    let r = fs::write(&path, s.as_bytes());
    if let Ok(mut m) = AIADJ_MSG.lock() {
        *m = match r { Ok(_) => format!("저장됨: {} — 오버라이드 {}항목", champ, cnt),
                       Err(e) => format!("저장 오류: {}", e) };
    }
}
fn ensure_ai_clicks(u: &mut GameUI) {
    let cur = u.filter_handler.len();
    let prev = AIADJ_FH_LEN.load(Ordering::Relaxed);
    if prev == usize::MAX || cur < prev {
        let filter: std::rc::Rc<dyn Fn(&UIEvent) -> bool> = std::rc::Rc::new(|e: &UIEvent| {
            if let UIEvent::Click { path, .. } = e {
                // 기본값 리셋 버튼 aiadj_r{i}
                let last = path.rsplit('.').next().unwrap_or("");
                if let Some(num) = last.strip_prefix("aiadj_r") { if let Ok(i) = num.parse::<i64>() { AIADJ_RESET_IDX.store(i, Ordering::Relaxed); return true; } }
                for n in 0..5i64 {
                    if path.ends_with(&format!(".aiadj_btn{}", n)) {
                        AIADJ_SEL_PLAYER.store(n, Ordering::Relaxed);
                        AIADJ_MODAL_OPEN.store(true, Ordering::Relaxed);
                        return true;
                    }
                }
                if path.ends_with(".aiadj_save") { AIADJ_SAVE_REQ.store(true, Ordering::Relaxed); return true; }
                if path.ends_with(".aiadj_close") || path.ends_with(".aiadj_dim") { AIADJ_MODAL_OPEN.store(false, Ordering::Relaxed); return true; }
            }
            false
        });
        let handler: std::rc::Rc<dyn Fn(&mut UIEventHandlerContext<(), UIOutEvent>)> = std::rc::Rc::new(|_c| {});
        u.filter_handler.push((filter, handler));
    }
    AIADJ_FH_LEN.store(u.filter_handler.len(), Ordering::Relaxed);
}
fn ai_ui_tick(u: &mut GameUI, ath: &[i64; 5]) {
    if !AIADJ_MODAL_ENABLED { ui_set_visible(&mut u.root, "aiadj_modal", false); return; }   // ★유저요청 07-11: 모달 진입 차단(클릭핸들러 미등록+항상 숨김)
    ensure_ai_clicks(u);
    let open = AIADJ_MODAL_OPEN.load(Ordering::Relaxed);
    ui_set_visible(&mut u.root, "aiadj_modal", open);
    if !open { AIADJ_LOADED_PLAYER.store(-2, Ordering::Relaxed); return; }
    let sel = AIADJ_SEL_PLAYER.load(Ordering::Relaxed);
    if AIADJ_LOADED_PLAYER.load(Ordering::Relaxed) != sel { unsafe { editor_load(u, ath); } AIADJ_LOADED_PLAYER.store(sel, Ordering::Relaxed); }
    if AIADJ_SAVE_REQ.swap(false, Ordering::Relaxed) { unsafe { editor_save(u, ath); } }
    let ridx = AIADJ_RESET_IDX.swap(-1, Ordering::Relaxed);   // ★기본값 버튼: 그 항목을 전역값으로
    if ridx >= 0 && (ridx as usize) < knobs::KNOBS.len() {
        let gv = global_get(knobs::KNOBS[ridx as usize]).unwrap_or_default();
        if let Some(f) = ui_kit::find_mut(&mut u.root, &format!("aiadj_f{}", ridx)) { ui_kit::textedit_set(f, &gv); }
    }
    // ★변경(전역과 다른) 필드 색 + ★호버 항목 감지(커서 UI좌표 vs 행 rect) — aiadj_list 1회 순회.
    let (gx, gy) = ui_kit::cursor_to_game();
    let teal = ui_kit::Rgba::hex(0x37d5b3ff);
    let gray = ui_kit::Rgba::hex(0x9aa0b0ff);
    let mut hover_idx: i64 = -1;
    let cls = cur_class_name();
    if let Some(list) = ui_kit::find_mut(&mut u.root, "aiadj_list") {
        let gmap = AIADJ_GLOBAL.lock().unwrap_or_else(|e| e.into_inner());
        for row in list.child.iter_mut() {
            let i = match row.id.as_str().strip_prefix("kbrow_").and_then(|r| r.parse::<usize>().ok()) { Some(i) if i < knobs::KNOBS.len() => i, _ => continue };
            let rc = row.rect;   // UI좌표 rect
            if gx >= rc.x && gx < rc.x + rc.w && gy >= rc.y && gy < rc.y + rc.h { hover_idx = i as i64; }
            let mut val = String::new();
            for c in row.child.iter() { if c.id.as_str().starts_with("aiadj_f") { if let Some(v) = ui_kit::textedit_get(c) { val = v; } break; } }
            let g = gmap.as_ref().and_then(|m| eff_global(m, &cls, knobs::KNOBS[i]));
            let ov = !val.trim().is_empty() && g.map(|x| x.as_str()) != Some(val.trim());
            for c in row.child.iter_mut() { if c.id.as_str().starts_with("kl_") { ui_kit::label_set_color(c, if ov { teal } else { gray }); break; } }
        }
    }
    // ★하단 도움말바: 호버한 항목 설명 > 저장메시지 > 기본 (코드 툴팁)
    let msg = AIADJ_MSG.lock().map(|m| m.clone()).unwrap_or_default();
    let hint_text = if hover_idx >= 0 && (hover_idx as usize) < knobs::DESCS.len() { knobs::DESCS[hover_idx as usize].to_string() }
        else if !msg.is_empty() { msg }
        else { "빈칸=전역 따름 · 항목에 마우스 올리면 여기에 설명".to_string() };
    if let Some(h) = ui_kit::find_mut(&mut u.root, "aiadj_hint") { ui_kit::label_set(h, &hint_text); }
}
struct CfgExt;
impl ModExtension for CfgExt {
    fn on_init(&self, _s: &mut Scene, _u: &mut GameUI, _a: &mut Assets) {
        if UI_INJECT_ON { unsafe { let _ = uinj::install(); } }   // 백업 설치(init서 이미 됐으면 no-op). ★07-30 OFF
    }
    fn post_update(&self, _s: &mut Scene, u: &mut GameUI, _a: &mut Assets, _dt: f32) {
        if UI_INJECT_ON { unsafe { uinj::tick(u); } }   // ★UI 조각 주입(mods/*/ui_inject.txt 스캔) + 모달 배경차단. scrim이 이미 후킹했으면 무동작(그쪽이 주입).
        sp_seen_flush();   // ★[07-31] subplan별 후퇴 발동 덤프(log=1일 때만·300프레임 스로틀). detour에서 옮겨온 것 — 위 함수 주석 참조.
        // ★[08-04] 진단 프로브 스냅샷 — **파일을 쓰는 건 여기 한 곳뿐**(핫패스는 원자 카운터만).
        //   07-22에 계측 하나가 병렬 워커에서 동기 IO를 폭주시켜 게임을 죽인 전례가 있다(probe.rs 상단 주석).
        unsafe { let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| probe_snapshot())); }
        // ★Phase3: 선발 athlete_id(row N = last_starting[N]) 추출 → 편집기. panic=post_update크래시라 catch_unwind로 격리.
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut ath = [-1i64; 5];
            if let Scene::InGame { data } = &*_s {
                let db = data.db();
                if let Some(team) = db.try_player_team() {   // 관리팀 없는 씬서 panic 회피
                    for (i, slot) in team.last_starting.iter().take(5).enumerate() { ath[i] = match slot { Some(a) => *a as i64, None => -1 }; }
                    // ⛔[07-31 폐기] 구 `sp_strat_read(team+0x318)` raw 판독 제거 —
                    //   ①오프셋이 오답이었고(사진 대조 5필드 불일치) ②SDK에 `Team.strategy` 공개 필드가 있어 불필요하며
                    //   ③SP_STRAT_RAW 슬롯을 SDK 판독본과 번갈아 덮어써 출력이 섞였다. 판독은 아래 전술 상태 블록 1곳으로 일원화.
                }
                // ★모달 열림 → 선택 선수 챔피언 클래스 계산(전역 프리필/색/기본값이 클래스오버라이드 반영)
                if AIADJ_MODAL_OPEN.load(Ordering::Relaxed) {
                    let n = AIADJ_SEL_PLAYER.load(Ordering::Relaxed);
                    let champ = if n >= 0 && n < 5 { ui_find(&u.root, &format!("row{}", n)).and_then(|r| unsafe { row_champion(r) }) } else { None };
                    let cls = champ.and_then(|c| db.champion_info(&c)).map(|info| match info.category() {
                        ChampionCategory::Melee => "melee", ChampionCategory::Range => "range",
                        ChampionCategory::Magician => "magician", ChampionCategory::Util => "util", ChampionCategory::Assassin => "assassin",
                    }.to_string()).unwrap_or_default();
                    *AIADJ_CUR_CLASS.lock().unwrap_or_else(|e| e.into_inner()) = cls;
                }
            }
            ai_ui_tick(u, &ath);   // ★AI버튼 클릭→모달, 편집/저장
        }));
        if !BOOTED.swap(true, Ordering::Relaxed) {
            append_log(&format!("[{}ms] [ext] post_update 가동. cfg 핫리로드 활성.\n", now_ms()));
        }
        IN_MENU.store(true, Ordering::Relaxed);   // 메뉴/모달 프레임 표시 → 다음 sim 첫 훅이 리셋 트리거
        load_champ_cfgs(false);   // ★선수(챔피언)별 players/*.cfg 핫리로드(30프레임당 stat, mtime변경시만 파싱)
        if CHAMP_VERIFY.load(Ordering::Relaxed) && CHAMP_VFLUSH.fetch_add(1, Ordering::Relaxed) % 120 == 0 { flush_champ_verify(); }
        if (CLASS_ANY.load(Ordering::Relaxed) || CHAMP_VERIFY.load(Ordering::Relaxed) || CHAMP_ANY.load(Ordering::Relaxed)) && !CLASS_BUILT.load(Ordering::Relaxed) {
            if let Scene::InGame { data } = _s {   // ★클래스맵 1회 빌드(typed API). 관리화면 프레임이면 충분.
                let r: &ClientDatabase = &*data.db();
                build_class_map(r);
            }
        }
        // ★관리팀 id + 내 팀 athlete 집합 캡처(우리팀 게이트용, self_team_only). InGame 프레임서 갱신(이적 반영). 60프레임당 1회(가벼움).
        // ★관리팀 id/내 팀 athlete 캡처. MANAGED<0이면 InGame 매 프레임(부트스트랩=관리화면서 즉시 확보→일정넘김 전에 세팅), 이후 120프레임당 갱신(이적 반영).
        if SELF_TEAM_ONLY.load(Ordering::Relaxed) || CHAMP_ANY.load(Ordering::Relaxed) {
            let need = MANAGED_TEAM_ID.load(Ordering::Relaxed) < 0 || ROSTER_POLL.fetch_add(1, Ordering::Relaxed) % 120 == 0;
            if need { if let Scene::InGame { data } = _s { refresh_my_team(&*data.db()); } }
        }
        // ★클래스 검증 덤프(class_verify=1): ~120프레임마다 class_verify.txt 갱신(라이브)
        if CLASS_VERIFY.load(Ordering::Relaxed) && CLASS_VFLUSH.fetch_add(1, Ordering::Relaxed) % 120 == 0 {
            flush_class_verify();
        }
        // ★★[07-31 3차] 연습경기 전술을 **SDK 필드로 직독** → sp_seen 구간 자동 라벨.
        //   구 방식(원시 24B + 오프셋 가정 + 바이트 덤프)은 전량 폐기 — `MatchReplayData.blue_strategy`가 그냥 공개 필드였다.
        //   경기가 진행될수록 replay가 늘어나므로 **가장 최근(game_tick이 살아있는) 연습경기 replay**를 라벨 소스로 삼는다.
        // ★조합테스트 저장처 추적 — 30프레임(≈0.5초)마다 스냅샷 비교. 경기 직후 변화를 놓치지 않게 촘촘히.
        if CT_HUNT.load(Ordering::Relaxed) && CT_TICK.fetch_add(1, Ordering::Relaxed) % 30 == 0 {
            if let Scene::InGame { data } = _s {
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    ct_hunt_scan(&*data.db());
                }));
            }
        }
        if SP_SEEN_ON.load(Ordering::Relaxed) && SP_PROBE_TRY.fetch_add(1, Ordering::Relaxed) % 60 == 0 {
            let mut why = String::from("=== 연습경기 전술 판독 상태 ===\n");
            if let Scene::InGame { data } = _s {
                let r: &ClientDatabase = &*data.db();
                // ★★[07-31 4차·접근 전환] UI가 전술을 **어디에 쓰는지** 쫓는 것을 포기한다.
                //   실패한 후보: `MatchType::Practice`(0건) · `MatchInfo.is_practice/is_room_practice`(0건) ·
                //                `Team.strategy`(화면과 12개 중 7개 불일치) ⟹ 그 화면은 이 셋 중 어디에도 안 쓴다.
                //   대신 **경기가 끝나면 생기는 `match_replays` 를 직접 훑어 가장 최근 것**을 본다.
                //   근거: 그 안의 `blue_strategy/red_strategy` = **sim 이 실제로 소비한 전술**이므로,
                //         "유저가 UI에서 뭘 눌렀는가"보다 오히려 정확한 라벨이다(중간 변환이 있어도 결과값을 본다).
                //   ⟹ 매치 타입 필터 자체가 불필요해지고, 저장 위치 RE 없이 목적(전술별 구간 분리)이 달성된다.
                let n_match = r.matches.len() as u32;
                let n_prac = r.match_replays.len() as u32;   // 재해석: 전체 replay 수
                let mut n_hit = 0u32;
                let mut best: Option<(u64, String, String)> = None;   // (id, blue_sig, red_sig)
                for (_k, rep) in r.match_replays.iter() {
                    n_hit += 1;
                    let id = rep.id as u64;
                    if best.as_ref().map_or(true, |b| id >= b.0) {
                        best = Some((id, sp_strat_sig(&rep.blue_strategy), sp_strat_sig(&rep.red_strategy)));
                    }
                }
                why.push_str(&format!("Scene=InGame  matches={} match_replays={} 훑음={}\n", n_match, n_prac, n_hit));
                // ★★유저 질문("다시보기에만 있으면 의미 없다")에 대한 직접 실험 —
                //   후보 2곳을 **동시에** 찍어서, 전술 창에서 드롭다운을 하나 바꿨을 때 **어느 쪽이 움직이는지** 본다.
                //   ①팀 영속 전술: `Team.strategy` / `Team.last_strategy` (SDK 필드 — 구 raw `+0x318` 은 오답이었다)
                //   ②경기 입력: `MatchReplayData.blue_strategy` / `.red_strategy`
                //   RE 메모리는 "②가 라이브+리플레이 공통 sim 입력"이라 하지만 0.5.3 재검증은 없다 ⟹ 추측 대신 관측한다.
                if let Some(team) = r.try_player_team() {
                    why.push_str(&format!("\n[①팀 영속] Team.strategy      {}\n           Team.last_strategy {}\n",
                        sp_strat_sig(&team.strategy), sp_strat_sig(&team.last_strategy)));
                } else {
                    why.push_str("\n[①팀 영속] player_team 없음(관리팀 미확정 씬)\n");
                }
                why.push_str("\n[②경기 입력] match_replays 중 id 최대(=가장 최근) 것\n");
                if let Some((id, b, rd)) = best {
                    why.push_str(&format!("replay id={}\n  BLUE {}\n  RED  {}\n", id, b, rd));
                    // ★이 값이 sim 이 실제 소비한 전술 ⟹ sp_seen 구간 라벨 소스로 사용.
                    *SP_STRAT_SIG.lock().unwrap_or_else(|e| e.into_inner()) = format!("B[{}] R[{}]", b, rd);
                    *SP_STRAT_RAW.lock().unwrap_or_else(|e| e.into_inner()) = format!("BLUE {}\n  RED  {}", b, rd);
                    SP_PROBE_DONE.store(true, Ordering::Relaxed);
                } else {
                    why.push_str("⚠match_replays 가 비어 있음 — 경기를 한 번 돌리면 잡힙니다.\n");
                }
            } else {
                why.push_str("Scene≠InGame (메인메뉴 등)\n");
            }
            if let Some(p) = pth("sp_strat_probe_status.txt") { let _ = fs::write(p, &why); }
        }
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
        // ★[07-15] apply_disc19_imm 호출 제거 — patch_imm_bytes 쓰기가 exe+0x1c83e6a에서 AV(0xc0000005 write)폴트.
        //   원인 조사(VirtualProtect 실패/게임 무결성 재보호/동시실행 race) 후 VEH-safe 쓰기로 재배선 예정. 그 전엔 미호출=안전.
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
                let s = format!("move={} move_tag={} move_off={:#x} move_x={} move_y={} MOVE_HANDLED={} | engage_base(cfg)={} gate_imm8={} | engage_thr_mult(cfg)={} ROLE_THR[{}]\nengage_repl(entry): on={} N={} pass={} (PT gate={} count={} other={})\nfc59a0[recall]: raw={} arm={} filt={} recallcap={}\ngenbuild[body]: raw={} arm={} gbbody={}\ngb[region_d]: raw={} armed={} badptr={} panic={} gbrd={} (OK={} DIFF={} NP={} Dvpush={}) | gbrepl={} replaced={} chk(M={} X={}) gbskip={} skipped={}\nInput tag별 첫샘플(머리 9 qword; 좌표같은 값 있는 오프셋이 Move의 x/y):\n{}replace={} repl_handled={} ready_ticks={}\n=== 광범위 커밋(FUN_141a49fa0, 매프레임 최종Input) total={} ===\n{}",
                    MOVE_ON.load(Ordering::Relaxed) as u8, MOVE_TAG.load(Ordering::Relaxed), MOVE_OFF.load(Ordering::Relaxed), MOVE_X.load(Ordering::Relaxed), MOVE_Y.load(Ordering::Relaxed), MOVE_HANDLED.load(Ordering::Relaxed),
                    ENGAGE_BASE.load(Ordering::Relaxed), gate_imm, ENGAGE_THR_MULT.load(Ordering::Relaxed), thr_live,
                    ENGAGE_REPL.load(Ordering::Relaxed) as u8, ENGAGE_REPL_N.load(Ordering::Relaxed), ENGAGE_REPL_PASS.load(Ordering::Relaxed), PT_GATE.load(Ordering::Relaxed), PT_COUNT.load(Ordering::Relaxed), PT_OTHER.load(Ordering::Relaxed),
                    FC59_RAW.load(Ordering::Relaxed), FC59_ARM.load(Ordering::Relaxed), FC59_FILT.load(Ordering::Relaxed), RECALLCAP.load(Ordering::Relaxed) as u8,
                    GBB_RAW.load(Ordering::Relaxed), GBB_ARMED.load(Ordering::Relaxed), GBBODY.load(Ordering::Relaxed) as u8,
                    GBRD_RAW.load(Ordering::Relaxed), GBRD_ARMED.load(Ordering::Relaxed), GBRD_BADPTR.load(Ordering::Relaxed), GBRD_PANIC.load(Ordering::Relaxed), GBRD.load(Ordering::Relaxed) as u8, GBRD_OK.load(Ordering::Relaxed), GBRD_DIFF.load(Ordering::Relaxed), GBRD_NP.load(Ordering::Relaxed), GBRD_VPUSH.load(Ordering::Relaxed),
                    GBREPL.load(Ordering::Relaxed) as u8, GBREPL_N.load(Ordering::Relaxed), GBREPL_MATCH.load(Ordering::Relaxed), GBREPL_MISMATCH.load(Ordering::Relaxed), GBSKIP.load(Ordering::Relaxed) as u8, GBSKIP_N.load(Ordering::Relaxed),
                    tags, REPL_ON.load(Ordering::Relaxed) as u8, REPL_HANDLED.load(Ordering::Relaxed), READY_TICKS.load(Ordering::Relaxed),
                    COMMIT_TOTAL.load(Ordering::Relaxed), ctags);
                let s = format!("{}call_ablate: cfg={} applied={} blocked(콜0xb 발화·차단) A={} B={} 합계={}\n", s, CALL_ABLATE.load(Ordering::Relaxed) as u8, CALL_ABLATE_APPLIED.load(Ordering::Relaxed) as u8, CALL_BLOCKED_A.load(Ordering::Relaxed), CALL_BLOCKED_B.load(Ordering::Relaxed), CALL_BLOCKED_A.load(Ordering::Relaxed)+CALL_BLOCKED_B.load(Ordering::Relaxed));
                let s = format!("{}lane_gate: cfg={} applied={} (0=원본/1=후보0개/2=후보다)\n", s, LANE_GATE.load(Ordering::Relaxed), LANE_GATE_APPLIED.load(Ordering::Relaxed));
                let s = format!("{}type3_ablate: cfg={} applied={} (transition 타입3콜 차단)\n", s, TYPE3_ABLATE.load(Ordering::Relaxed) as u8, TYPE3_APPLIED.load(Ordering::Relaxed) as u8);
                // ★[07-31] SubPlan 실측 분포 — disc18/19(=SubPlan 18/19) 발화 여부의 직접 지표.
                //   0이 아닌 버킷만 찍는다. **18/19가 0이면** 그 경기에서 넥서스 SubPlan 자체가 생성되지 않은 것
                //   ⟹ 훅·주소 문제가 아니라 **국면 미도달**이라는 뜻(Plan16/17은 나와도 승격 게이트에서 막힐 수 있다).
                let sp_line = {
                    let mut v: Vec<String> = Vec::new();
                    for i in 0..SP_HIST.len() {
                        let c = SP_HIST[i].load(Ordering::Relaxed);
                        if c != 0 { v.push(format!("{}:{}", i, c)); }
                    }
                    format!("subplan_dispatch: total={} other={} | {}\n",
                        SP_TOTAL.load(Ordering::Relaxed), SP_OTHER.load(Ordering::Relaxed),
                        if v.is_empty() { "(발화 0 = 디스패처 자체 미도달)".to_string() } else { v.join(" ") })
                };
                let s = format!("{}{}", s, sp_line);
                // ★[07-31] FORCE_SP19 진단 — 셋을 구분한다:
                //   cfg=0        → cfg 파싱 실패 or 미설정
                //   calls=0      → my_disc17 최종반환 미도달(= disc17 대체가 다른 경로로 감)
                //   forced>0인데 disc19 훅 미발화 → 우리가 쓴 SubPlan 이 게임에 전달되지 않음
                let s = format!("{}force_sp19: cfg={} d17_calls={} d17_forced={} | nx_repl={} (0=disc16/17 passthrough=A/B 실험 OFF측)\n", s,
                    FORCE_SP19.load(Ordering::Relaxed) as u8,
                    D17_CALLS.load(Ordering::Relaxed), D17_FORCED.load(Ordering::Relaxed),
                    D1617_REPL.load(Ordering::Relaxed) as u8);
                write_named("repl_status.txt", &s);
            }
        }
        // plan_base 자동탐지 (1회, 메인스레드 = 안전)
        if !DIAG_DONE.load(Ordering::Relaxed) { unsafe { try_find_plan_base(); } }
        if load_cfg(false) {
            // (dmgcmp 재측정 트리거 제거 — 하드코딩 테스트라 1회만)
            append_log(&format!("[{}ms] ↻ cfg: enabled={} team={} x={} y={} coef_mult={}%\n", now_ms(),
                OV_ENABLED.load(Ordering::Relaxed) as u8, OV_TEAM.load(Ordering::Relaxed),
                OV_X.load(Ordering::Relaxed), OV_Y.load(Ordering::Relaxed), OV_COEF_MULT.load(Ordering::Relaxed)));
        }
        // ★07-11 크래시대책② 관측: itemnet 가드 차단 카운트를 **LOG_ON 무관 직접 파일 write**(perf.txt 방식).
        //   append_log가 반복적으로 죽는(LOG_ON 리셋 추정) 로그 인프라를 우회 — 가드 발동을 확실히 파일로 남긴다.
        //   매 post_update 무조건 갱신(누적값 항상 기록·H=0이어도 "설치됨+미발동" 증거). itemnet_guard.txt 단일파일 truncate write.
        {
            let h = ITEMNET_GUARD_HITS.load(Ordering::Relaxed);
            ITEMNET_GUARD_SEEN.store(h, Ordering::Relaxed);
            // ★[0.5.4 프로브] TeamPlan.version 실측 분포 — **LOG_ON 무관 직접 write**.
            //   앞서 진단 블록(write_named)에 넣었다가 cfg log=1 이 아니면 안 찍혀서 놓쳤다.
            //   `>=2` 게이트가 exe 전역 8곳(경매 강제귀환·점수식 넥서스 게이트 등)을 여닫는데
            //   정적으로는 값을 못 밝혔다(팩토리가 정적 호출 0건). 이 파일이 그 답이다.
            //   ⚠"난이도"가 아니다 — 게임의 Difficulty{Easy,Normal,Hard}와는 별개 필드다.
            if let Some(p) = pth("teamplan_version.txt") {
                let mut v: Vec<String> = Vec::new();
                for i in 0..AUC_VER_HIST.len() {
                    let c = AUC_VER_HIST[i].load(Ordering::Relaxed);
                    if c != 0 { v.push(format!("{}:{}", i, c)); }
                }
                let b = AUC_VER_BIG.load(Ordering::Relaxed);
                if b != 0 { v.push(format!("(8이상):{}", b)); }
                let _ = fs::write(p, format!(
                    "TeamPlan.version = {}   (경매 진입 3번째 인자 실측)\n\
                     훅 설치 = {}\n\
                     ※ 2 이상이면 0.5.4 신규 판단(경매 강제귀환·점수식 넥서스 게이트)이 켜져 있다.\n",
                    if v.is_empty() { "(관측 0 — 훅 미설치 또는 경매 미도달)".to_string() } else { v.join(" ") },
                    if ORIG_AUCTION.load(Ordering::Relaxed) != 0 { "OK" } else { "실패" }));
            }
            if let Some(p) = pth("itemnet_guard.txt") {
                let _ = fs::write(p, format!("itemnet 가드 차단 누적 = {}  (마지막 갱신 {}ms; 0=미발동/설치됨, >0=AV였을 진입을 실제 차단)\n", h, now_ms()));
            }
        }
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    fresh_log(&format!("[{}ms] === plan_reimpl Phase1 INIT (월드접근 검증) ===\n", now_ms()));
    unsafe {
        seh_install();   // ★VEH 안전읽기 핸들러 등록(fast_read 경로용; off여도 무해=우리 폴트범위만 처리)
        crash_install(); // ★크래시 로거 등록(미처리 치명적 예외 시 crash_log.txt에 폴트위치 기록; 상시 ON, 정상경로 무비용)
        panic_hook_install();   // ★[07-14] panic hook(panic_log.txt) — UEF가 못 잡는 abort(extern"C" 경계 panic) 경로 규명용. 상시 ON, 정상경로 무비용.
        hang_watchdog_install();   // ★[07-16] 행(hang) 진단 워치독(hang_diag.txt) — 일정넘김 멈춤 규명용. 독립 스레드, hang_diag=0 OFF.
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
        // ★0.4.14 위협게이트(FUN_1420a8680) 콜사이트 래핑 — 정글러 교전후퇴 p2 캡처/배율. target sanity가드(≠FUN_1420a8680→Err skip)라 stale시 무크래시. mult=100&tgcap=0=원본 비트동일.
        match install_threatgate_hook() {
            Ok(())=>append_log("[hook] threatgate(@0x1feca43→FUN_1420a8680) OK\n"),
            Err(e)=>append_log(&format!("[hook] threatgate 실패: {}\n", e)),
        }
        let _ = (install_replace_detour as *const (), install_move_post_hook as *const (), install_commit_hook as *const ());
        if HARNESS_ON {
            // ★fc59a0 recall RNG score(0x2080e20, 12B=push8). cfg recallcap=1 검증(리턴훅 kind:5) / recall_repl=1 완전대체(replace-rax: SENT=passthrough, 그외=out ptr로 skip).
            match install_replace_detour_rax(RVA_FC59A0, 12, fc59a0_capture as *const () as usize) {
                Ok(())=>append_log("[hook] fc59a0 recall score(@0x2080e20, 12B, replace-rax) OK\n"),
                Err(e)=>append_log(&format!("[hook] fc59a0 실패: {}\n", e)),
            }
            // ★generic_build 본체(0x20def90, 12B=push8) 디스패치/출력 캡처. cfg gbbody=1. 리턴훅 kind:14. (task#23)
            if INSTALL_DIAG_HOOKS {
            match install_detour(RVA_GENERIC_BUILD, 12, genbuild_body_capture as *const () as usize) {
                Ok(())=>append_log("[hook] generic_build body 출력캡처(@0x20def90, 12B) OK\n"),
                Err(e)=>append_log(&format!("[hook] generic_build body 실패: {}\n", e)),
            }
            }
            // ★영역 D: 0x42a3 캡처/검증/skip 디투어(handled→funnel skip / passthrough→capture+verify). cfg gbrd=verify·gbskip=진짜skip. cap_fn i64.
            if !MIG_GB_CHANGED {
            match install_detour_d_skip(RVA_GB_REGIOND_HOOK, ORIG_LEN_GB_REGIOND, gbrd_capture as *const () as usize, RVA_GB_FUNNEL) {
                Ok(())=>{ GBRD_INSTALL_OK.store(1, Ordering::Relaxed); if let Some(p)=pth("gbinstall.txt"){ let _=fs::write(p, format!("gbrd detour OK @0x{:x} len={}\n", RVA_GB_REGIOND_HOOK, ORIG_LEN_GB_REGIOND)); } append_log("[hook] gbrd/gbskip 영역D(@0x22dafea, 14B) OK\n"); }
                Err(e)=>{ GBRD_INSTALL_OK.store(2, Ordering::Relaxed); if let Some(p)=pth("gbinstall.txt"){ let _=fs::write(p, format!("gbrd detour FAIL: {}\n", e)); } append_log(&format!("[hook] gbrd/gbskip 실패: {}\n", e)); }
            }
            } else { let _=(install_detour_d_skip as *const(), gbrd_capture as *const(), RVA_GB_REGIOND_HOOK, RVA_GB_FUNNEL, ORIG_LEN_GB_REGIOND); append_log("[hook] gbrd/gbskip 영역D SKIP (MIG_GB_CHANGED=true, 0.4.14 generic_build region D 재추출 대기)\n"); }
            // ★0.4.13 마이그완료: facet#1 condgate(@0x1be1290, 15B). cfg condcap=1. 리턴훅 kind:6.
            // ★replace-detour(rax): cond_repl=0이면 cap_fn이 SENT→passthrough(install_detour와 동일). cond_repl=1이면 my_condgate(≠-99)로 완전대체.
            match install_replace_detour_rax(RVA_CONDGATE, 15, condgate_capture as *const () as usize) {
                Ok(())=>append_log("[hook] facet#1 condgate(@0x1c383f0, 15B, replace-rax) OK\n"),
                Err(e)=>append_log(&format!("[hook] condgate 실패: {}\n", e)),
            }
            // ★facet#4 movepriority 관측(0x1c08420, 14B=7push+sub0x50). cfg mpcap=1. 리턴훅 kind:7.
            // ★replace-detour(sret rax=rcx): mp_repl=0이면 cap_fn이 1→passthrough(install_detour와 동일). mp_repl=1이면 disc0/1 완전대체.
            match install_replace_detour(RVA_MOVEPRI, 12, mp_capture as *const () as usize) {   // ★0.5.3: 13→**12**(프롤로그 push6→push4로 축소 = 경계가 0,2,3,4,5,9,12,20. 13은 `mov rax,[rsp+0xb8]` 한복판을 잘라 즉사) / 구 0.5.0: 14→13
                Ok(())=>append_log("[hook] facet#4 movepriority(@0xc559e0 0.5.3, 12B, replace-sret) OK\n"),
                Err(e)=>append_log(&format!("[hook] movepriority 실패: {}\n", e)),
            }
            // ★07-11 크래시대책②(§12.23): itemnet 스코어러 NULL-모델 가드(fn+12, scrim 프롤로그검증 비간섭).
            //   실패해도 기능 무영향(가드 부재=기존과 동일) — 단 크래시 원천차단이 빠지므로 로그로 노출.
            //   ★설치 결과를 LOG_ON 무관 직접write(append_log 인프라가 죽어도 설치여부 확증 — 무크래시가 "가드덕"인지 "트리거미도달"인지 구분용).
            match install_itemnet_guard() {
                Ok(())=>{ append_log("[hook] itemnet NULL-모델 가드(@0x1b78420+12, 15B) OK\n");
                    if let Some(p)=pth("itemnet_guard.txt"){ let _=fs::write(p, "itemnet 가드 설치 OK (readable 검사판) — 차단 누적 0 (아직 미발동)\n"); } }
                Err(e)=>{ append_log(&format!("[hook] itemnet 가드 실패(크래시대책② 미설치): {}\n", e));
                    if let Some(p)=pth("itemnet_guard.txt"){ let _=fs::write(p, format!("★itemnet 가드 설치 실패: {} (크래시 원천차단 부재 — 재현시 크래시 가능)\n", e)); } }
            }
            // ★disc18/19(진짜 넥서스) 완전재현 Phase2-1: 캡처 wrap(game 원본 관찰). wrap은 game 원본 항상 호출=passthrough라
            //   dcap=0이면 기능 무영향(덤프만 skip). 프롤로그 push8 신원검증+catch_unwind로 안전. RNG-free라 재sim 무영향.
            match install_wrap(RVA_DISC18_HANDLER, 12, disc18_capture as *const () as usize) {
                Ok(orig)=>{ ORIG_DISC18.store(orig, Ordering::Relaxed); append_log("[hook] disc18 캡처wrap(@0x1c7ca20 0.5.1) OK\n"); }
                Err(e)=>append_log(&format!("[hook] disc18 wrap 실패: {}\n", e)),
            }
            match install_wrap(RVA_DISC19_HANDLER, 12, disc19_capture as *const () as usize) {
                Ok(orig)=>{ ORIG_DISC19.store(orig, Ordering::Relaxed); append_log("[hook] disc19 캡처wrap(@0x2380820 0.5.2) OK\n"); }
                Err(e)=>append_log(&format!("[hook] disc19 wrap 실패: {}\n", e)),
            }
            // ⛔★[07-31] SubPlan 디스패처 계측 wrap — **크래시로 즉시 OFF**(아래 SPDISP_PROBE=false).
            //   증상: 설치 자체는 성공(`hooks.txt` stub tag=0xd98740 등재)했으나 경기 진입 후 **AV `0xc0000005`**
            //         `RIP=exe+0xc4225e` · `faultAddr=0x0`(null 역참조) · 콜러 `exe+0xca92ed`, 2회 재현.
            //   ⟹ 원인 규명 전에는 켜지 말 것. 후보 = ①`0xd98740` 12B 구간으로 **점프해 들어오는 내부 분기**가 있어
            //      트램폴린이 그 경로를 깨뜨림 ②install_wrap 의 7인자 전달이 이 함수 규약과 불일치.
            //   ★교훈: passthrough·read-only 라도 **트램폴린을 새로 박는 것 자체가 위험**하다(§3 메모리안전).
            // ★[0.5.4 프로브] 경매 진입 래퍼 — `TeamPlan.version` 관측 전용(passthrough).
            //   위 SPDISP_PROBE 블록 **밖**에 둔다: 07-31 크래시는 `d98740` 한정이고,
            //   경매(`eacf10`)는 안전 실증된 disc18(`da1850`)과 측정 가능한 전 항목이 동일하다 —
            //   선두 12B 바이트 완전동일(push8) · 12인자 extern "C" 동형 · 호출부 1곳 ·
            //   **테일콜 진입 0 · 선두 12B 내부 진입 0**(v54\jmpin2.py 전역 스캔).
            //   ⚠크래시가 나면 이 상수 하나만 false 로 되돌리면 된다.
            if AUC_PROBE {
                if let Ok(o) = install_wrap(RVA_AUCTION, 12, auction_probe_capture as *const () as usize) {
                    ORIG_AUCTION.store(o, Ordering::Relaxed);
                }
            }
            if SPDISP_PROBE {
                match install_wrap(RVA_SUBPLAN_DISPATCH, 12, subplan_dispatch_capture as *const () as usize) {
                    Ok(orig)=>{ ORIG_SPDISP.store(orig, Ordering::Relaxed); append_log("[hook] SubPlan 디스패처 계측wrap(@0xd98740) OK\n"); }
                    Err(e)=>append_log(&format!("[hook] SubPlan 디스패처 wrap 실패: {}\n", e)),
                }
            }
            // ★[07-15] disc19 판단상수 imm-patch = 로드시점 1회(여기 = install_wrap 성공지점 = sim 실행 전 = .text 쓰기 안전).
            //   cfg 먼저 로드해 사용자값 반영(config=재시작이라 로드시 1회면 충분). 게임플레이중 재패치는 AV폴트라 안 함.
            load_cfg(true);
            apply_disc19_imm();
        } else { let _ = (RVA_FC59A0, fc59a0_capture as *const ()); let _ = (RVA_GENERIC_BUILD, genbuild_body_capture as *const ()); let _ = (RVA_CONDGATE, condgate_capture as *const ()); let _ = (RVA_MOVEPRI, mp_capture as *const ()); }
    }
    load_cfg(true);
    load_champ_cfgs(true);   // ★선수(챔피언)별 players/*.cfg 초기 로드
    unsafe {   // ★UI 주입 훅 설치(가능한 한 일찍). prologue mismatch면 안전 무동작.
        if let Some(d) = dir() { if let Ok(mut g) = uinj::LOG_DIR.lock() { *g = d.to_string_lossy().into_owned(); } }
        uinj::DBG.store(false, Ordering::Relaxed);   // 배포=false(uinj_log.txt 안 씀). 주입진단 필요시 true.
        if UI_INJECT_ON { let _ = uinj::install(); }   // ★07-30: 유저 지시로 OFF (UI_INJECT_ON 주석 참조)
        // ★[07-30] 스텁 인벤토리 덤프 — 모든 훅/스텁 설치가 끝난 뒤 1회. LOG_ON 과 무관하게 hooks.txt 로 남긴다.
        //   목적: 크래시 이벤트로그가 `module=unknown`(동적 RWX 스텁)일 때, 그 절대주소를 이 표와 대조해
        //         **어느 훅의 트램폴린인지 사후 특정**. crash_log.txt 쪽은 stub_lookup 으로 자동 표기되지만,
        //         프로세스가 UEF 도달 전에 죽는 경우(abort/__fastfail)엔 WER 주소밖에 안 남으므로 이 표가 유일한 단서다.
        stub_dump();
    }
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(CfgExt);
    reg.add_player_input_ai(PlanAi);
    reg
}
declare_mod!(init);
