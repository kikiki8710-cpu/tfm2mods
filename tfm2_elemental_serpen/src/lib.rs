//! tfm2_elemental_serpen — 다속성 세르펜 + 장로(처형) 모드
//! ===========================================================================
//! 최종 목표:
//!   - 세르펜이 리스폰마다 랜덤 속성(화염/대지/바람/바다/마법공학/화학공학)으로 등장,
//!     속성별로 다른 팀 영구 버프 부여. N번째 이후는 장로 세르펜(저체력 챔피언 처형).
//!   - 속성 정의는 config/*.cfg (파일 추가 = 속성 추가). 버프 필드 = SDK BuffState.
//!
//! ── Stage 1 (이 파일): 런타임 프로브 ──
//!   세르펜 per-tick 핸들러(FUN_1422bdda0, 0.5.0_3 RVA 0x22bdda0)를 트램폴린 detour.
//!   목적 = (a) detour 안전성 실증(경기 안 죽음) (b) 핸들러 진입 실측.
//!   ⚠ rcx=world/sim 핸들(엔티티 아님). 세르펜 엔티티는 함수 내부 (*(rdx+0x150))(rcx,a5) lookup.
//!   위험 프로브(엔티티 shadow-lookup / GameCtx tick)는 cfg 게이트 기본 OFF.
//!
//! 안전: detour 본문 catch_unwind, raw r/w는 SEH(VEH) 보호, 배경 sim 병렬 진입 대비
//!   공유상태 Atomic + 로그 Mutex poison-safe. item_editor 검증본 이식.
//! ===========================================================================
#![allow(unused_imports, unused_variables)]
use mod_api::*;
// 공용 UI 모듈(복사 금지, #[path] import 규약). find()로 "game_time" 노드 존재 = 경기 화면 판정에 사용.
#[path = "C:/tfm2mods/ui_kit/ui_kit.rs"]
mod ui_kit;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicPtr, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_elemental_serpen";

// ── 세르펜 per-tick 핸들러 (0.5.0_3, RVA = abs − 0x140000000) ──
//   FUN_1422bdda0. 프롤로그 12B(PUSH 8개)=온전한 경계, relocatable OK, 재진입=+0xc(SUB RSP).
//   시그니처 = extern "win64" fn(rcx,rdx,r8,r9,a5,a6) → void. rcx≠엔티티(world/sim 핸들).
const SERPEN_RVA: usize = 0x1328950; // 0.5.4 (구0.5.3=0x1535810) kind6. ★clone 함정 2패치 연속 재발(0.5.3 오답=0x1c70e90 / 0.5.4 동점후보=0x13273e0, 둘 다 kind5 Epic). 확정 근거 = 함수+0x73 `cmp dword [rax+0x68], 6`. 프롤로그 8push + sub rsp,0x368.
// ★0.5.3 확증: 프롤로그 12B 바이트 완전동일 + 함수 +77 명령이 `mov rax,[rbx+0x1c8]`(0.5.2는 +0x1b8)로
//   1:1 대응, 간접 call 19건의 함수내 오프셋이 +1098까지 완전일치. 크기 3863→3938.
// ★0.5.2 kind6 확증: 스켈레톤 L1 UNIQUE(크기 0xf17 동일, 전 mem-disp/imm 일치) + 함수 내 상대오프셋
//   +0x73 에 `cmp dword ptr [rax+0x68], 6` 존재(구/신 동일 위치). 엔티티오프셋 참조 15개 전부 일치.
const SERPEN_PROLOGUE: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
const ENTITY_KIND_OFF: usize = 0x68; // ==6 → 세르펜
const SERPEN_KIND: i32 = 6;
const O_ENTITY_ID: usize = 0x5a8;
const O_CUR_HP: usize = 0x658;
// ★0.5.3 이동 확정(2026-07-29): SERPEN 함수 +77 에서 `mov rax,[rbx+0x1b8]` → `[rbx+0x1c8]` 로 바뀜
//   (앞뒤 명령 문맥 동일 = 같은 명령의 disp만 변경). DMGB 콜러 사상에서도 +0x10 일치.
//   ⚠이 값은 읽어서 **함수포인터로 호출**하므로 틀리면 즉시 크래시 — 엔티티 구조체 나머지는 전부 불변.
const O_ENTITY_ACCESSOR: usize = 0x1c8; // 0.5.3 (구0.5.2/0.5.1=0x1b8) rdx+0x1c8 = id→entity 리졸버 함수포인터
const O_SERPEN_TEMPLATE: usize = 0xb0;  // 세르펜 엔티티+0xb0 = 처치 시 뿌릴 이펙트 템플릿(0x120)
const O_SPRITE_NAME_PTR: usize = 0x250; // 세르펜 엔티티+0x250 = 스프라이트 이름 (ptr, len@+0x258) "serpen_monster"
const O_SPRITE_NAME_LEN: usize = 0x258;

static SERPEN_TRAMP: AtomicUsize = AtomicUsize::new(0);
static SERPEN_HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
static ENTER_COUNT: AtomicU64 = AtomicU64::new(0);
static PROBE_LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
static LOG_FLUSHED_LEN: AtomicUsize = AtomicUsize::new(0);
static SAVED_GAMECTX: AtomicUsize = AtomicUsize::new(0);

// cfg 게이트 (serpen_probe.cfg, on_init 1회 로드) — 위험 프로브 기본 OFF
static CFG_PROBE_LOG: AtomicBool = AtomicBool::new(false);     // 진단 로그 파일(배포 기본 off, serpen_probe.cfg로 on)
// 대조실험용(07-24 클론분기 근원 조사): 0이면 배정·waves 기록·킬 추적은 그대로 두고 **sim 개입(템플릿
//   스탯 write)만 억제**. execute=0과 함께 쓰면 모드의 sim 개입이 0 — 그때도 ⚠킬재기록이 나오면
//   분기 근원 = 게임 자체. 기본 1(정상 동작).
static CFG_STAT_WRITE: AtomicBool = AtomicBool::new(true);
// ★결정론 실험(07-24): thread-local RandomState 게터를 고정키(0,0)로 대체 → 병렬 sim HashMap 순회
//   결정론화 = 클론 간 킬타이밍 분기 제거 가설 검증. 기본 OFF(게임 동작 변경 개입 = 검증 전용).
static TEMPLATE_WRITES: AtomicU64 = AtomicU64::new(0);
static CFG_NAME_SWAP: AtomicBool = AtomicBool::new(false); // sim 세르펜 이름(+0x250)을 serpen_<attr>_monster로 교체

// 인월드 엔티티 스프라이트 렌더러 FUN_141136600(0.5.0_3). RVO 반환 + 7인자.
//   rcx=반환버퍼, rdx=view-entity, r8=asset ctx, r9=p4, +스택 p5~7.
//   view-entity: +0x58/+0x60=스프라이트이름, +0x164=위치(2×f32), +0xa0/+0xa8=상태Vec(0x28, 이름+0x8/+0x10).
// ⬜0.5.2 미확정 + ★이미 0.5.1에서도 죽어 있던 상수(마이그 중 발견, 2026-07-22):
//   0.5.1 exe의 0x1136600 실제 바이트는 `ff90 4883c428 5b5d c3`(= call [rax+..]; add rsp,0x28; pop; ret)
//   = 함수 시작이 아니라 어느 함수의 **꼬리 조각**이다. 아래 RENDER_PROLOGUE(8push)와 애초에 불일치라
//   install 시 프롤로그 검증에서 항상 false → 훅이 설치된 적이 없다(cfg `render_probe = 0`이라 무증상).
//   ⇒ 0.5.2 값(마스크시그 후보 0x156ce40)도 같은 성질이라 채택 보류. 되살리려면 ghidra-re로 진짜
//     스프라이트 렌더러 함수시작을 다시 잡아야 한다.


// sheet 룩업 FUN_14051bbc0 (0.5.0_3): fn(rcx=ctx, rdx=key_ptr, r8=key_len)→handle.
//   인월드 세르펜 스프라이트가 이 키("...ingame/serpen#sheet")로 룩업(캐시미스 시)되므로
//   여기서 콜스택 잡으면 인월드 렌더 콜러 + 엔티티 접근 경로 확정. ⚠ draft_overlay와 동일 함수.
// ⬜0.5.2 미확정 + ★RENDER_RVA와 같은 부류로 0.5.1에서도 이미 죽어 있던 상수(2026-07-22 발견):
//   0.5.1 exe의 0x51bbc0 실제 바이트는 `0f8589000000 488b96c00000..`(jne + mov) = 함수 시작이 아니다.
//   아래 SHEET_PROLOGUE(8push)와 불일치 → 훅 미설치(cfg에 sheet 프로브 없음·기본 false라 무증상).
//   0.5.2 후보는 완전동일 사본이 2개(0x820b70 / 0xc1f1f0) 나와 정적으로 못 가림 ⇒ 보류.

// 세르펜 템플릿 스탯블록 = 세르펜 엔티티 + 0xb0(템플릿) + 0x58(스탯) = +0x108.
//   ★물리 배치(2026-07-18 RE): idx0~14 = i32(연속 4B) / entry+0x94 = 4B 패딩 / idx15~31 = i64(8B, +0x98부터).
const TMPL_STAT_OFF: usize = O_SERPEN_TEMPLATE + 0x58; // 0x108 = 블록 베이스
const NUM_STATS: usize = 32;
// 논리 스탯 인덱스 → 블록 내 바이트 오프셋 (i32 0~14 / i64 15~31). i64 여부는 idx>=15.
// ★★스탯블록 물리 배치 (2026-07-19 디스어셈 실측으로 **정정**, 합산함수 FUN_141f097b0 0x1f097b0)
//   블록 = 이펙트엔트리+0x58. 블록크기 0xc8 → 0x58+0xc8 = 0x120 = 엔트리 stride(무결성 확인).
//     idx0~14  : i32  @ 0x00~0x38       (attack … skill_cooldown_mult)
//     idx15~26 : i64  @ 0x40~0x98       (damage_reflect … dot_amplify)
//     ★cc_immune : bool 1B @ 0xa0       ← **숫자 인덱스가 아니지만 메모리 한 칸을 차지한다**
//     idx27~29 : **i32** @ 0xa4,0xa8,0xac (ult_cooldown_mult, radius_mult, crit_chance)
//     idx30~31 : i64  @ 0xb0,0xb8       (base_attack_damaged_reduce, skill_damaged_reduce)
//     undying/ignore_wall : bool @ 0xc0/0xc1
//   ⚠구 공식 `i<15 ? i*4 : 0x40+(i-15)*8`은 idx26까지만 맞았다. idx27을 0xa0에 i64로 쓰면
//     최하위 바이트가 **cc_immune을 켜서 CC 완전면역을 몰래 부여**하고, 진짜 궁쿨감(0xa4)엔 0이
//     들어가 효과가 없었다(= "궁 쿨감 적용 안 됨" 제보의 정체). crit_chance(29)도 0xb0에 쓰여
//     base_attack_damaged_reduce를 덮고 정작 크리(0xac)는 0이었다.
//   근거: 0x141f098d4 `OR R9B,[RSI-0x21]`(0xa0=bool) / 0x141f098d8 `MOVQ XMM12,[RSI-0x1d]`+PADDD
//         (0xa4·0xa8 = i32 2개) / 0x141f098e3 `ADD R8D,[RSI-0x15]`(0xac) /
//         0x141f098e7 `MOVDQU [RSI-0x11]`+PADDQ(0xb0·0xb8 = i64 2개).
//   합산 후 entity 오프셋: idx27→+0x454, idx29(crit)→+0x45c. 궁 쿨다운 공식(실측):
//     궁 CD = raw*100 / (100 + skill_cooldown_mult[+0x3e8] + ult_cooldown_mult[+0x454])
#[inline] fn stat_off(i: usize) -> usize {
    if i < 15 { i * 4 }
    else if i < 27 { 0x40 + (i - 15) * 8 }
    else if i < 30 { 0xa4 + (i - 27) * 4 }   // 27,28,29 = i32
    else { 0xb0 + (i - 30) * 8 }             // 30,31 = i64
}
// 해당 인덱스가 4바이트(i32)인가? (i64는 8바이트 write)
#[inline] fn stat_is_i32(i: usize) -> bool { i < 15 || (27..30).contains(&i) }
// 블록 base에서 논리 인덱스 i의 값을 읽는다(i64는 하위 표시용 i32로 절단).
unsafe fn stat_read(base: usize, i: usize) -> Option<i32> {
    if stat_is_i32(i) { safe_read_i32(base + stat_off(i)) }
    else { safe_read_u64(base + stat_off(i)).map(|v| v as i32) }
}
// ★0.5.2 struct 불변 확인(2026-07-22, exe↔exe .text disp 센서스 = 실제 mem-operand 변위만 집계):
//   provider 계열 0xeab8/0xeac0/0xecc0/0xecc8/0xecd0/0xecd8/0xed18/0xed20/0xed28/0xed50/0xed58
//   = 사용횟수가 OLD와 **11/11 정확히 동일**(7,78,6,4,13,4,10,11,7,12,1) ⇒ provider struct 시프트 없음.
//   엔티티 계열도 SERPEN 함수 본문에서 +0x68(kind)/+0x5a8/+0x658 참조 15개가 구/신 완전 일치.
//   스탯블록 물리배치 근거함수(0.5.1 0x1f097b0 → 0.5.2 0x220b470)의 배치 결정 4명령어
//   (`or r9b,[rsi-0x21]` / `movq xmm12,[rsi-0x1d]` / `add r8d,[rsi-0x15]` / `movdqu [rsi-0x11]`)도 동일
//   ⇒ stat_off()/TMPL_STAT_OFF 그대로 유효.
//   ⚠단 ClientDatabase 계열(0x1338/0x1340/0x1598/0x1630/0x1678/0x1680/0x2970/0x1dc0)은 이 방법으로
//     **판정 불가**(값이 너무 흔해 무관 구조체 사용분에 묻힘) → 아래 각 상수 주석 참조·런타임 검증 필요.
// ★★0.5.3 구조체 시프트(2026-07-29 실측): provider(World) 의 **0xea00~0xf000 대역이 통째로 +0x40** 이동.
//   근거 = MOBATICK 확정쌍(0x230c290→0xeeeac0) 본문 disp 히스토그램에서 이 대역 0.5.2 오프셋 전부가
//   0.5.3에 +0x40 위치에 **개수까지 일치**로 존재하고 +0 위치엔 부재(0xece8/0xecf0/0xecf8/0xed00/0xed08/
//   0xed18/0xed20/0xed80 등 8종이 +0 카운트 0). 계열 합산 n=25 중 +0x40:21(2위 4).
//   SIM_TICK 은 사용함수 11개 중 10개가 +0x40 확증(+0 은 0개) = 단독으로도 결정적.
//   ⚠ 그 아래 대역(엔티티 0x40~0x400 n=4852 / World 슬롯맵 0x400~0x1000 n=1024 / db·Game)은 **전부 불변**
//     ⇒ 0x40 삽입 지점은 0xe000~0xea00 사이. 저역까지 같이 밀지 말 것.
const SEED_OFF: usize = 0xeb28; // 0.5.4 (구0.5.3=0xeaf8) provider + 0xeb28 = 경기 시드(u64, 불변). 근거=seedctor 0x14e16d0 의 `mov [rsi+0xeb28],rax`

// config key → 이펙트 스탯블록 i32 인덱스 (엔트리+0x58 기준, idx*4)
const STAT_KEYS: &[(&str, usize)] = &[
    ("attack", 0), ("attack_mult", 1), ("magic_power", 2), ("magic_power_mult", 3),
    ("defence", 4), ("defence_mult", 5), ("hp", 6), ("hp_regen", 7),
    ("magic_resistance", 8), ("magic_resistance_mult", 9), ("vamp", 10), ("hp_mult", 11),
    ("move_speed_mult", 12), ("attack_speed_mult", 13), ("skill_cooldown_mult", 14),
    ("damage_reflect", 15), ("damaged_amplify", 16), ("defence_penetration", 17),
    ("magic_resistance_penetration", 18), ("toughness", 19), ("heal_reduce", 20),
    ("range", 21), ("base_attack_enemy_max_hp_damage", 22), ("self_max_hp_damage", 23),
    ("skill_enemy_max_hp_damage", 24), ("damaged_reduce", 25), ("dot_amplify", 26),
    ("ult_cooldown_mult", 27), ("radius_mult", 28), ("crit_chance", 29),
    ("base_attack_damaged_reduce", 30), ("skill_damaged_reduce", 31),
];

// 스탯 퍼센트 여부(데이터) — 인덱스 = STAT_KEYS와 동일. 표시명은 i18n(text/<lang>.txt, key=`stat.<STAT_KEYS.0>`).
const STAT_PCT: [bool; NUM_STATS] = [
    false, true, false, true, false, true, false, false,
    false, true, false, true, true, true, true,
    false, false, false, false, false, false,
    false, false, false, false, false, false,
    true, true, false, false, false,
];
// 하나의 세르펜 속성 정의 (config 파일 1개)
#[derive(Clone, Default)]
struct Attr {
    name: String,
    display_name: String,
    sprite: String,
    name_buf: String, // "{sprite}_monster" — 세르펜 엔티티 스프라이트 이름으로 교체할 값
    anim_key: String, // "asset/<modid>/aseprite_resources/ingame/{sprite}#anim" (base→modid 교체용)
    sheet_key: String,
    weight: u32,
    stats: [i32; NUM_STATS],
    execute_hp_pct: i32, // >0 = 장로(처형 임계 %)
    execute_duration: u64, // 처형 능력 지속시간(sim tick). 0 = 무제한
    // ★생산자 seam용 단축 에셋 베이스 키 `asset/<modid>/s/<한글자>` (2026-07-19).
    //   리졸버 반환 String을 제자리 치환하려면 바닐라 베이스키(43자)보다 짧아야 한다.
    short_key: String,
}
// 경기(키=경기 시드, 고유·불변)별 세르펜 배정 상태.
//   세르펜 식별 = 엔티티 id(a5). a5가 바뀌면 새 세르펜(리스폰)으로 판정.
#[derive(Default)]
//   ★세르펜은 한 경기에 여러 마리(실측 2마리: a5=0xf3/0x1c9)가 동시에 살아서 번갈아 detour된다.
//   마리별로 속성을 주면 전역 CURRENT_ATTR이 1ms 간격으로 진동 = 화면 색 깜빡임(실측).
//   → "한 웨이브 = 한 속성"(롤 드래곤과 동일). 전멸(WAVE_GAP_MS 무활동) 후 재등장 = 새 웨이브.
//   ★2026-07-17 RE 반영: 웨이브는 게임 캠프 필드(respawn_count/next_respawn_tick)에서 직접 온다.
//   waves = 웨이브인덱스 → (spawn_tick, 속성). serpen_logs[i] = 웨이브 i의 죽음(1:1)이라 매칭 불필요.
//   ★waves_by_tick = 웨이브 시작 sim_tick → 속성. 뒤로감기하면 게임이 같은 seed로 sim을 tick 7200부터
//   재시뮬한다(실측 타임라인: 7200→16282→26154 후 다시 7200→16282). 속성을 spawn_count(누적)로 정하면
//   같은 tick 웨이브가 재시뮬 때 다른 색이 된다(=뒤로감기 시 색 불일치 원인). tick을 키로 쓰면 재현됨.
//   kills = 게임이 직접 쌓는 세르펜 처치 이력 (team, tick). ★훅 불필요 — provider에 이미 있다.
struct WorldState {
    waves: HashMap<u64, (u64, i32)>, // 웨이브idx → (spawn_tick, 속성idx). -1=장로
    kills: Vec<(u64, u64, u64)>,     // (team, tick, kill_index) — index = serpen_logs 위치(장로판정 정본)
    current: i32,                    // 지금 살아있는 웨이브의 속성
    wave_idx: u64, spawn_tick: u64,  // 현재 웨이브(캠프 필드 원본)
    rcx: u64, sim_tick: u64, last_ms: u64, hits: u64, // 진단
    logged_mask: u64,                // 진단: ◆sim웨이브전이를 로그한 웨이브 비트셋 (Option 페어는 진행도 다른
    rb_mask: u64,                    //   클론끼리 핑퐁 → 초당 수백 줄 실사고(07-24) → 웨이브당 1회 비트마스크로)
    last_kill_tid: u32,              // 진단(07-24 FP조사): 이 파티션 kills를 마지막으로 쓴 스레드 id.
                                     //   킬재기록 시 현재 tid와 비교 = "다른 스레드 동시" vs "같은 스레드 재계산" 판별
}
// ★챔피언 구성 지문 (07-24 ghidra-re 확정 레시피): (팀, 챔피언 name) 페어 정렬 합성 해시.
//   엔티티 name String = cap@+0x248/ptr@+0x250/len@+0x258 (0.5.2 정적 확정, ctor 3필드 기록 사이트 근거).
//   player→champ 체인 = find_player 0x2306870 본문으로 0.5.2 불변 실증(+0x840/848 dense·+0x820 team·
//   +0x8b8 tag·+0x8c0 key → slots +0x738/740 [idx*0x10+8]=dense_idx → champs +0x720/728 stride 0x6a8).
unsafe fn world_fingerprint(w: usize) -> Option<u64> {
    let pp = safe_read_u64(w + W_PLAYER_DENSE)? as usize;
    let pn = safe_read_u64(w + W_PLAYER_DENSE + 8)? as usize;
    if pp < 0x10000 || pn == 0 || pn > 16 { return None; }
    let sp = safe_read_u64(w + W_CHAMP_SLOTS)? as usize;
    let sn = safe_read_u64(w + W_CHAMP_SLOTS + 8)? as usize;
    let cp = safe_read_u64(w + W_CHAMP_DENSE)? as usize;
    let cn = safe_read_u64(w + W_CHAMP_DENSE + 8)? as usize;
    if sp < 0x10000 || cp < 0x10000 || cn == 0 || cn > 4096 { return None; }
    let mut pairs: [u64; 16] = [0; 16];
    let mut np_cnt = 0usize;
    for i in 0..pn {
        let p = pp + i * PLAYER_STRIDE;
        let Some(team) = safe_read_u64(p + P_TEAM) else { continue };
        if team > 1 { continue; }
        if safe_read_u64(p + P_CHAMP_TAG).unwrap_or(0) == 0 { continue; } // 챔피언 미배정
        let Some(key) = safe_read_u64(p + P_CHAMP_KEY) else { continue };
        let idx = (key & 0xffff_ffff) as usize;
        if idx >= sn { continue; }
        let Some(dense) = safe_read_u64(sp + idx * 0x10 + 8) else { continue };
        if dense as usize >= cn { continue; }
        let ent = cp + dense as usize * CHAMP_STRIDE;
        if safe_read_i32(ent + ENTITY_KIND_OFF) != Some(CHAMP_KIND as i32) { continue; }
        let Some(nl) = safe_read_u64(ent + 0x258) else { continue };
        let nl = (nl as usize).min(32);
        let Some(nptr) = safe_read_u64(ent + 0x250) else { continue };
        if (nptr as usize) < 0x10000 || nl == 0 { continue; }
        let mut nb = [0u8; 32];
        if !safe_copy(nb.as_mut_ptr(), nptr as usize as *const u8, nl) { continue; }
        let mut h = 0xcbf2_9ce4_8422_2325u64; // FNV-1a
        for &b in &nb[..nl] { h = (h ^ b as u64).wrapping_mul(0x1000_0000_01b3); }
        if np_cnt < 16 { pairs[np_cnt] = (team << 62) | (h >> 2); np_cnt += 1; }
    }
    if np_cnt < 4 { return None; } // 드래프트 미완/판독 실패 → 미확정(다음 틱 재시도)
    pairs[..np_cnt].sort_unstable();
    let mut h = 0x9e37_79b9_7f4a_7c15u64;
    for &v in &pairs[..np_cnt] { h = splitmix64(h ^ v); }
    Some(h | 1) // 0 회피(0 = 미확정 sentinel)
}
// fp 캐시 경유 조회. 검증 = World ptr + champ dense ptr(재할당별 고유) 동시 일치 — 같은 seed 다른
//   세트가 ptr를 재사용해도 dense ptr까지 겹칠 확률은 무시 가능. 미스 시 재계산(~100 VEH read, 세트당 1회꼴).
unsafe fn fp_for_world(w: usize) -> Option<u64> {
    let chk = safe_read_u64(w + W_CHAMP_DENSE).unwrap_or(0);
    if chk < 0x10000 { return None; }
    let slot = ((w >> 4) & (KC_SLOTS - 1)) as usize;
    if FP_ADDR[slot].load(Ordering::Relaxed) == w as u64 && FP_CHK[slot].load(Ordering::Relaxed) == chk {
        let v = FP_VAL[slot].load(Ordering::Relaxed);
        if v != 0 { return Some(v); }
    }
    let fp = world_fingerprint(w);
    match fp {
        Some(v) => {
            FP_ADDR[slot].store(w as u64, Ordering::Relaxed);
            FP_CHK[slot].store(chk, Ordering::Relaxed);
            FP_VAL[slot].store(v, Ordering::Relaxed);
            Some(v)
        }
        None => { FP_FAIL_N.fetch_add(1, Ordering::Relaxed); None }
    }
}
// ★렌더측 fp 도출 (메인스레드, post_update): 런처 게이트가 저장한 out Game → provider → fp.
//   Game→provider 오프셋은 0x1dc0(0.5.2 추정)/0x1660(0.5.1 채록) 두 후보를 provider+0xeab8==RENDER_SEED
//   자기검증으로 선별(틀린 오프셋은 seed 불일치로 자연 탈락 → 조용히 미채택 = 안전).
//   ⚠db→provider 링크는 존재하지 않음 확정(2026-07-17 RE)이라 db 경유 불가 — 런처 rcx가 유일 경로.
fn resolve_render_fp() {
    if RENDER_FP.load(Ordering::Relaxed) != 0 { return; } // 확정됨(런처 게이트가 세트 전환마다 리셋)
    let game = LAUNCH_GAME.load(Ordering::Relaxed) as usize;
    if game < 0x10000 { return; }
    let rs = RENDER_SEED.load(Ordering::Relaxed);
    if rs == 0 { return; }
    for off in [GAME_PROVIDER_OFF, 0x1660usize] {
        let Some(prov) = (unsafe { safe_read_u64(game + off) }) else { continue };
        let prov = prov as usize;
        if prov < 0x10000 || prov >= (1usize << 47) { continue; }
        if unsafe { safe_read_u64(prov + SEED_OFF) } != Some(rs) { continue; } // 자기검증
        if let Some(fp) = unsafe { fp_for_world(prov) } {
            RENDER_FP.store(fp, Ordering::Relaxed);
            RENDER_PROV.store(prov as u64, Ordering::Relaxed);
            if CFG_PROBE_LOG.load(Ordering::Relaxed) {
                log_push(format!("[{}ms] ★렌더fp 확정 seed={:#x} fp={:04x} (Game+{:#x} provider={:#x})",
                    now_ms(), rs, fp & 0xffff, off, prov));
            }
        }
        return; // seed 검증 통과 오프셋을 찾았으면 fp 미확정이라도 종료(다음 프레임 재시도)
    }
}
// 렌더측 화면 파티션 선택: RENDER_FP 확정 시 그 파티션, 미확정이면 그 seed 파티션이 유일할 때만.
fn pick_live<'a>(m: &'a HashMap<(u64, u64), WorldState>, ls: u64) -> Option<&'a WorldState> {
    if ls == 0 { return None; }
    let rf = RENDER_FP.load(Ordering::Relaxed);
    if rf != 0 {
        if let Some(ws) = m.get(&(ls, rf)) { return Some(ws); }
    }
    // ★리플레이 대응(2026-07-26): fp 지문이 없으면(리플레이 out Game이 스택이라 stale) 화면 처치
    //   카운터로 파티션을 특정한다 — "화면에서 실제 죽는 세르펜"이 곧 화면 경기라는 정의(SCREEN_KILLS 정신).
    //   그 seed 파티션들 중 (B,R) 처치수(played 이하)가 화면 카운터와 정확히 일치하는 유일 파티션.
    let scb = SERPEN_CNT_ONSCREEN[0].load(Ordering::Relaxed);
    let scr = SERPEN_CNT_ONSCREEN[1].load(Ordering::Relaxed);
    let played = PLAYED_TICK.load(Ordering::Relaxed);
    if scb + scr > 0 {
        let mut hit: Option<&WorldState> = None;
        let mut dup = false;
        for ((s, _), w) in m.iter().filter(|((s, _), _)| *s == ls) {
            let _ = s;
            let kb = w.kills.iter().filter(|(t, k, _)| *t == 0 && *k <= played).count() as u64;
            let kr = w.kills.iter().filter(|(t, k, _)| *t == 1 && *k <= played).count() as u64;
            if kb == scb && kr == scr {
                if hit.is_some() { dup = true; break; }
                hit = Some(w);
            }
        }
        if !dup { if let Some(w) = hit { return Some(w); } }
    }
    let mut it = m.iter().filter(|((s, _), _)| *s == ls).map(|(_, ws)| ws);
    let first = it.next()?;
    if it.next().is_some() { None } else { Some(first) } // 복수 파티션 + 매칭 실패 = 조회 포기(오염보다 안전)
}
// ★전멸 판정 = wall-clock 기준. (검증됨: "대지-바람-화염" 3웨이브가 실제와 정확히 일치)
//   ⚠sim tick 기준으로 바꿨다가 대실패 — sim_tick이 단조증가가 아니다(실측: 32127→32035→32143…
//   같은 경기 detour가 여러 스레드로 들어와 tick이 레이스). tick 역행마다 생존목록이 비워져
//   웨이브가 매 틱 폭증 → elder_after 초과 → 전부 장로 → 색 깨짐. 재시도 금지.
// ★sim tick = provider+0xeac0 (seed +0xeab8 바로 옆. GameCtx::tick()이 읽는 그 필드 — 단 vtable
//   호출은 detour서 AV라 raw read만). 0.5.1 유효(인접 seed가 실증됨).
const SIM_TICK_OFF: usize = 0xeb30; // 0.5.4 (구0.5.3=0xeb00) — provider 구조체 상위대역 0.5.4에서 **+0x30 시프트**(0.5.2→3은 +0x40)
// ★★2026-07-17 행단위 RE 정본 (0.5.1 실측) — 추측 heuristic 전면 대체.
//   provider = `World`(0xeaf0) + 인라인 `MobaMode`(@+0xeaf0). 아래는 전부 provider(=detour rcx) 기준.
//   세르펜 캠프 = JungleCampState(0x30) @ +0xecb8 (jungle_runner 10슬롯 중 serpen, ty=5).
//   ⇒ 세르펜은 **경기당 정확히 1마리**(ty5 스폰좌표 1개로 정적 확증) → "여러 마리" 가정 불필요.
const CAMP_SPAWN_TICK: usize = 0xed40; // 0.5.4 (구0.5.3=0xed10) next_respawn_tick = **이 웨이브의 스폰 tick**(웨이브 내내 불변)
const CAMP_WAVE_IDX: usize = 0xed48;   // 0.5.4 (구0.5.3=0xed18) respawn_count = **웨이브 인덱스(0-based)**
// 처치 로그: serpen_logs[i] = **웨이브 i의 죽음**(1:1) → 처치↔웨이브 순서 매칭 로직 불필요.
//   entry 16B { team:u64, tick:u64 }, tick 축 = World.tick(+0xeac0) = played_tick과 1:1.
// ★★장로 처형 (2026-07-17 RE). 게임의 처형 = 전용 kill 함수 없이 **entity+0x658 = 0** raw write.
//   판정식(Inquisitor ult 0x22a9740과 동일): curHP(+0x658) <= maxHP(**+0x610**) × thr / 100
//   ⚠기준 HP는 +0x5c8(base-stat)이 아니라 **+0x610(effective)**.
//   ★킬 크레딧: 처형 자체엔 없다. 데미지 적용(0x1f147e0)이 남기는 **entity+0x670 + team*8 = 180틱
//   윈도우**가 원천 → "그 팀이 최근 때렸을 때만" 처형해야 킬/골드가 그 팀에 정상 귀속된다
//   (= 게임 계약 "처형은 가해자의 데미지 기록 뒤에 온다" + LoL 장로 시맨틱과도 일치).
const O_EXEC_MAXHP: usize = 0x610;      // 처형 판정 기준 HP
//   ★HP write만으론 부족했던 필드 ①: baseHP를 안 깎으면 regen/재계산이 curHP를 되살려 사망 무효화.
//   ★HP write만으론 부족했던 필드 ②: 죽음 pass가 "이번 틱 피격됨" 마커로 소비. 누락=무한 재시뮬.
const O_DMG_WINDOW: usize = 0x670;      // +team*8, 0이 아니면 그 팀이 최근 180틱 내 피해를 입힘
const CHAMP_KIND: u64 = 0xd;
// World 슬롯맵 (0.5.1 확정, find_player 0x21f7570 근거)
const W_CHAMP_DENSE: usize = 0x720;     // ptr / +0x728 len (stride 0x6a8)
const W_CHAMP_SLOTS: usize = 0x738;     // ptr / +0x740 len (stride 0x10: [0]=u32 occupied, [8]=dense_idx)
const W_PLAYER_DENSE: usize = 0x840;    // ptr / +0x848 len (stride 0x8c0, 0.5.4) — World 오프셋은 불변
const P_TEAM: usize = 0x810;            // 0.5.4 (구0.5.3=0x820) player+0x810 = team(u64). ⚠athlete 레이아웃 0.5.4에서 **-0x10 시프트**
const P_CHAMP_TAG: usize = 0x8a8;       // 0.5.4 (구0.5.3=0x8b8) Option tag (0=챔피언 없음)
const P_CHAMP_KEY: usize = 0x8b0;       // 0.5.4 (구0.5.3=0x8c0) champion slotmap key
const CHAMP_STRIDE: usize = 0x6a8;
const PLAYER_STRIDE: usize = 0x8c0;
// 0.5.4 (구0.5.3=0x8d0) MobaMode::tick — 매 틱 호출(rcx=World). 프롤로그 12B 순수 push(그 뒤 mov eax,imm 5B까지 17B 안전,
//   단 그 다음 `call __chkstk`는 상대콜이라 스틸 금지) → 12B 스틸.
// ★0.5.3 확정(2026-07-29, 독립 2방법 일치): ①문자열 `"game_core::simulation::game"` 를 LEA하는 함수가
//   두 exe 각각 **유일**(0.5.2=0x230c290 / 0.5.3=0xeeeac0) ②콜그래프 전파투표 43표(2위 21)
//   ③provider 오프셋 교차검증 — 이 후보만 0xed2x/0xed3x/0xed5x 대역을 참조(다른 후보는 0건).
//   프롤로그 12B 바이트 동일(chkstk imm만 0x19c8→0x1b08). 크기 48761→42668.
const MOBATICK_RVA: usize = 0x13ee0a0; // 0.5.4 (구0.5.3=0xeeeac0)
const MOBATICK_PROLOGUE: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
const KILLS_BLUE_OFF: usize = 0xedc0;  // 0.5.4 (구0.5.3=0xed90) serpen_count[0]
const KILLS_RED_OFF: usize = 0xedc8;   // 0.5.4 (구0.5.3=0xed98) serpen_count[1]
// ★db → 화면 경기 provider 정석 3-deref (db 128KB 스캔 폐기: VEH 폴트 25만의 주범이었음)
// ⬜0.5.2 정적 미검증(값 유지) — ClientDatabase raw 오프셋군. disp 센서스로는 판정 불가(위 SEED_OFF 주석).
//   런타임 불변식으로 자기검증됨: 0 ≤ PLAYED_TICK ≤ events.len(EV_LEN_OFF). 어긋나면 조용히 미채택.
//   ★교차확인 권장: crm·Spectator_Chat 이 같은 ClientDatabase 오프셋군을 쓰므로 그쪽 0.5.2 마이그 결과와 대조.
const GAME_PROVIDER_OFF: usize = 0x1dc0; // Game + 0x1dc0 = provider data ptr (== detour rcx) ⬜0.5.2 런타임검증 대기
// ★게임이 직접 관리하는 세르펜 처치 이력 (ghidra-re 0.5.1 확정, 리워드 배포 FUN@0x21fcf90 분석):
//   provider + 0xed18/0xed20/0xed28 = Vec<{team:u64, tick:u64}> 의 cap/ptr/len. team 0=blue 1=red.
//   처치 tick이 함께 저장되므로 played_tick 이하만 집계하면 sim 선행·뒤로감기가 자동 정합된다.
//   (+0xed50/+0xed58 = 팀별 처치수, +0xed30/+0xed38 = 팀별 버프 잔여틱)
const KILLS_PTR_OFF: usize = 0xed90; // 0.5.4 (구0.5.3=0xed60) — cap=0xed88
const KILLS_LEN_OFF: usize = 0xed98; // 0.5.4 (구0.5.3=0xed68)
// ★실측 확정(2026-07-16): played_tick과 sim tick은 **같은 축(1:1)**.
//   근거: 첫 웨이브 sim_tick=7200 → played=7281에 화면에 세르펜이 보였고(유저 확인), 다음 웨이브
//   16185는 아직 안 보였음. sim_tick=16382가 played=7281보다 앞선 건 sim이 재생을 앞질러 계산하기 때문.
//   ⚠문서·아틀라스의 "sim 30/s vs played 60/s(2배)" 기록은 실측과 불일치 → 1로 확정.
const ELDER_IDX: i32 = -1;
const VANILLA_ATTR: i32 = -2; // 이 웨이브는 색·버프 없음(바닐라). 원소/장로 스위치 off 시.
// ★온오프 스위치 (serpen.cfg): 둘다 off면 바닐라. 기본 on.
static CFG_ELEMENTAL: AtomicBool = AtomicBool::new(true); // 원소 세르펜(색+버프) 사용
static CFG_ELDER: AtomicBool = AtomicBool::new(true);     // 장로 세르펜(색+버프+처형) 사용

// ★2026-08-02: 속성 풀/장로 = `Mutex` → **불변 스냅샷 + AtomicPtr(락 없음)**.
//   이 둘은 `load_attrs()` 에서만 세팅되고 그 뒤로는 읽기 전용인데, 매 틱 detour(8 sim 워커)와
//   매 프레임 렌더 경로(`keyres_swap`)가 각각 Mutex 를 잡아 서로 경합했다 —
//   외부 샘플러 실측에서 park 대기(전체 busy 샘플의 15~21%)의 최대 호출자가 이 모드였다.
//   교체본은 읽기가 **원자 load 1회**뿐이라 경합이 구조적으로 사라진다.
//   ⚠구 스냅샷은 **의도적으로 leak** 한다 — detour 가 `&'static` 로 들고 있을 수 있어 안전한 해제
//     시점을 알 수 없고, 재발행은 초기화/cfg 재적용에 한정되어 누수량이 유한하다(수 KB 단위).
//   ⚠이 전환으로 기존 락 순서(POOL→ELDER→WORLDS→…)에서 앞 두 항목이 소멸한다(데드락 표면 감소).
struct AttrSnap { pool: Vec<Attr>, elder: Option<Attr> }
static ATTR_SNAP: AtomicPtr<AttrSnap> = AtomicPtr::new(core::ptr::null_mut());
#[inline]
fn attrs() -> Option<&'static AttrSnap> {
    let p = ATTR_SNAP.load(Ordering::Acquire);
    if p.is_null() { None } else { Some(unsafe { &*p }) }
}
fn publish_attrs(pool: Vec<Attr>, elder: Option<Attr>) {
    let b: &'static mut AttrSnap = Box::leak(Box::new(AttrSnap { pool, elder }));
    ATTR_SNAP.store(b as *mut AttrSnap, Ordering::Release);
}
static ELDER_AFTER: AtomicU32 = AtomicU32::new(3); // 내부 0-based(=config 4번째부터). config에서 1-based로 입력.
// ★★키 = (seed, fp) 파티션 (07-24 버그수정): 게임이 Bo 시리즈 세트들에 같은 seed를 재사용함이
//   실측 확정(같은 seed 웨이브 스폰 24204/28703 이중 타임라인·같은 웨이브 팀 뒤집힘 킬·킬재기록 3948회).
//   fp = 챔피언 구성 지문(world_fingerprint) — 세트마다 밴픽/사이드가 달라 fp가 갈리고, 같은 세트의
//   재시뮬 클론은 같은 드래프트라 같은 fp = 원하는 파티션 성질. (RE 근거: 세트 인덱스는 런처 인자에도
//   World에도 없음 — dl=[db+0x738] 모드셀렉터, r9d=World+0xeae9 플래그로 확정, 07-24 ghidra-re)
static WORLDS: Mutex<Option<HashMap<(u64, u64), WorldState>>> = Mutex::new(None);
// fp 캐시: World ptr direct-map. 검증키 = champ dense ptr(할당별 고유 → ptr 재사용 오염 방지, 매 틱 1read)
static FP_ADDR: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];
static FP_CHK: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];
static FP_VAL: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];
static FP_FAIL_N: AtomicU64 = AtomicU64::new(0);   // 지문 계산 실패 수(진단)
static RENDER_FP: AtomicU64 = AtomicU64::new(0);   // 화면 세트의 fp (0=미확정)
static LAUNCH_GAME: AtomicU64 = AtomicU64::new(0); // 런처 게이트 적중 시 rcx=out Game (렌더 fp 도출용)
static RENDER_PROV: AtomicU64 = AtomicU64::new(0); // 도출된 화면 provider(진단)
static ATTRS_LOADED: AtomicBool = AtomicBool::new(false);
// 관전 경기 세르펜 속성 전역: skia 커맨드 키 교체가 이걸 사용. -2=미설정.
static CURRENT_ATTR: AtomicI32 = AtomicI32::new(-2);
// 방금 죽은 웨이브의 속성 — 죽는 모션은 이걸로 그린다(처치 즉시 CURRENT_ATTR이 다음으로 넘어가므로)
static PREV_ATTR: AtomicI32 = AtomicI32::new(-2);
static RENDER_TID: AtomicU32 = AtomicU32::new(0);
// 메인(클라이언트) 스레드 id — post_update/on_init에서 기록. 관전 경기 sim은 메인 스레드에서 tick.
static MAIN_TID: AtomicU32 = AtomicU32::new(0);

// ── 관전 경기 provider 캡처 (item_tactics 검증 스폰클로저 훅, ⚠ item_tactics와 동일 함수) ──
//   라이브(관전) sim 전용 스레드 스폰 클로저: 0x473040(lineup)/0x4724a0(variant). rcx=env.
//   env+0x10=ArcInner(Arc<RwLock<Game>>), Game=inner+0x20, provider=*(Game+0x1660).
//   세르펜 detour rcx(=provider) == LIVE_PROVIDER면 관전 경기 → CURRENT_ATTR 세팅.
const SPAWN_HOOKS: [usize; 2] = [0xb31bb0, 0xb30f90]; // 0.5.4 (구0.5.3=0xabdf60/0xabd340). 프롤로그 12B 동일·크기 2073=2073·콜러 컨테이너 지문(size367/+0xa1, size431/+0xc9) 완전일치.
const SPAWN_PROLOGUE: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
static LIVE_PROVIDER: AtomicUsize = AtomicUsize::new(0);
// ★★★재생할 경기를 "고르는 지점" = Game 런처 0x20588a0 (2026-07-17 RE, 유저 착안).
//   배경 리그 30~40경기가 동시에 sim을 도는데 화면엔 하나만 재생된다 → 그걸 고르는 코드가 반드시 있다.
//   클라 씬빌더(0x722ca0)가 이 런처를 부르며 **seed를 인자로 직접 넘긴다**:
//     rcx = out Game / edx = 셀렉터 / **r8 = seed(순수 u64)** / r9d = 0
//   콜사이트 9곳 중 화면 경기 = 아래 2개(retaddr로 식별). 0x2061132는 배경 리그 → 자동 배제.
//   ⇒ db 역탐색 불필요(db→provider 링크는 존재하지 않음이 확정됨: GameView는 순수 렌더 상태).
// ★0.5.3 확정(2026-07-29, 독립 2방법 일치): 씬빌더(0x74d510→0x997740)에서 **정확히 2회** +
//   리플레이핸들러(0x1554930→0x229a410)에서 **1회** 불리는 타깃이 0.5.3 전체에서 0xeb8810 **유일**.
//   ghidra 별도 검증도 동일(콜사이트 총 9곳 = 0.5.2와 동수·동성격). 프롤로그 12B 바이트 동일.
const LAUNCHER_RVA: usize = 0x13b53d0; // 0.5.4 (구0.5.3=0xeb8810)
const LAUNCHER_PROLOGUE: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
// ★RET_A/B 재도출 방식 주의(0.5.2): 컨테이너 0x722ca0→0x74d510 은 본문이 84명령어 줄어(14769→14685)
//   **단순 컨테이너+오프셋 델타도, 콜 서수(ordinal) 매핑도 둘 다 오답**을 낸다(각각 0x759d72 / 0x759fc1).
//   정답 = 컨테이너 명령어열 difflib 정렬 후 해당 call 명령어의 신주소. 두 사이트 모두 'equal' 구간에
//   떨어졌고 정렬된 call의 타깃이 위 LAUNCHER_RVA와 자기일치 ⇒ 교차검증됨.
// ★0.5.3 재도출(2026-07-29): 위 확정 런처의 xref 콜사이트를 씬빌더 본문에서 직접 열거해 얻음
//   (컨테이너+오프셋 델타나 콜 서수 매핑을 쓰지 말 것 — 0.5.2 때 둘 다 오답이었다).
//   자기일치 확인: 세 사이트의 E8 타깃이 전부 LAUNCHER_RVA(0xeb8810)로 재계산됨.
const LAUNCHER_RET_A: usize = 0x9e2079; // 0.5.4 (구0.5.3=0x9a3287) 화면 경기 경로 A = 콜사이트 0x9e2074+5 (씬빌더 0x9d5f20 +0xc154)
const LAUNCHER_RET_B: usize = 0x9e6feb; // 0.5.4 (구0.5.3=0x9a7b03) 화면 경기 경로 B = 콜사이트 0x9e6fe6+5 (씬빌더 0x9d5f20 +0x110c6)
// ★리플레이(다시보기) 진입 경로 C — pause 메뉴 replay_match_slot 매치런치 핸들러(entry 0x1554930)의
//   런처 콜사이트 0x1555210 + 5(E8 rel32) = retaddr. 이 게이트로 리플레이도 화면 경기 seed를 확정한다.
//   (ghidra-re 2026-07-26: World 생성은 런처 0x1d96870을 반드시 경유·간접호출 전무 → 콜사이트+5=retaddr)
const LAUNCHER_RET_C: usize = 0x1d147e4; // 0.5.4 (구0.5.3=0x229ad94) 리플레이 경로 = 콜사이트 0x1d147df+5 (핸들러 0x1d13e60 +0x97f, 크기 5272=5272)
// ★comp_test(조합테스트) 다시보기 경로 D — comp_test는 정규 리플레이 핸들러(0x1d13e60)를 타지 않고
//   전용 재생 빌더 0x2323aa0(training_ui.rs, CompTestHistoryEntry의 seed로 재시뮬)을 탄다.
//   경로: comp_test 팝업 다시보기 버튼 → 0x2326820 → 0x2323aa0 → 런처 콜사이트 0x2323ff9(+5=retaddr).
//   (ghidra-re 2026-08-08: 런처 콜사이트 exe 바이트스캔 전수 9건 중 유일한 comp_test 화면 재생 경로.
//    ⚠0x235c382는 comp_test 백그라운드 sim 본체 추정 = 화이트리스트 금지.
//    전문 = REPORT\tfm2_elemental_serpen\RE\2026-08-08_comptest-다시보기-런처콜사이트.md)
const LAUNCHER_RET_D: usize = 0x2323ffe; // 0.5.4 comp_test 다시보기 = 콜사이트 0x2323ff9+5 (재생 빌더 0x2323aa0 +0x559)
static RENDER_SEED: AtomicU64 = AtomicU64::new(0);   // ★화면 경기 seed (이게 정답 게이트)
static LAUNCH_N: AtomicU64 = AtomicU64::new(0);      // 런처 총 발화수
static LAUNCH_HIT: AtomicU64 = AtomicU64::new(0);    // 그중 화면 경기(retaddr 일치)
static LAUNCH_LAST_RVA: AtomicU64 = AtomicU64::new(0); // 최근 retaddr rva(진단)
// ★track_kills 변화감지 캐시(성능): World 주소로 direct-map. 처치 이력이 그대로면 배열 재판독과
//   전역 WORLDS 락을 통째로 건너뛴다. 슬롯 충돌 시엔 서명 불일치 → 그냥 한 번 더 읽을 뿐(무해).
// 스프라이트 위치 보정 스위치/미세조정 (serpen.cfg) — 앵커 규칙 실측용이자 최종 조정 손잡이.
static SPR_CENTER_FIX: AtomicBool = AtomicBool::new(true);
static SPR_OFF_X: AtomicI32 = AtomicI32::new(0);
static SPR_OFF_Y: AtomicI32 = AtomicI32::new(0);
const KC_SLOTS: usize = 64;
#[allow(clippy::declare_interior_mutable_const)]
const KC_ZERO: AtomicU64 = AtomicU64::new(0);
static KC_ADDR: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];
static KC_SIG: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];
// exec_amp "장로버프 활성팀" 캐시 — 키=(처치수, 1초버킷), 값=팀 비트마스크. 타격마다 재계산 방지.
static EA_ADDR: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];
static EA_KEY: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];
static EA_MASK: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];
// 진단: 런처 호출처 distinct 목록 (rva, 횟수) — 다시보기 진입 경로 식별용
static LAUNCH_RVAS: Mutex<Option<Vec<(u64, u64)>>> = Mutex::new(None);
// ★item_tactics 검증 게이트: 스폰 클로저(화면 sim 전용 스레드)가 캡처한 tid.
//   세르펜 detour(=sim 틱)도 같은 sim 스레드서 도므로 tid 일치 = 화면 경기. 배경(rayon 워커)은 불일치 배제.
static LIVE_TID: AtomicU64 = AtomicU64::new(0);
static LOG_N: AtomicU64 = AtomicU64::new(0); // detour 로그 총량 제한(다경기 환경서 상태변화 로그 무력 → 전역 카운터)
// ★재생 커서(화면에 지금 보이는 프레임). TFM2는 sim을 먼저 다 돌리고 그 프레임을 천천히 재생하므로,
//   sim detour 시점 속성 ≠ 화면 프레임 속성(실측: 뒤로감기 시 색 불일치). 색은 played_tick 기준이어야 함.
//   출처 = Spectator_Chat 검증 오프셋(0.5.0_3 = 0.5.1 동일, 재도출 금지): scene(db+0x1338)+8+0x258.
// ★ghidra-re 0.5.1 확정(2026-07-16): 오프셋은 원래 맞았고 판정식이 틀렸었다.
//   ClientScene은 niche enum이고 InGame이 untagged variant → **tag < 3**(0/1/2)이 InGame.
//   게임 관용구: `sub rcx,3; mov r8d,5; cmovb rcx,r8` = if tag<3 {InGame} else {tag-3}.
//   tag표: 3=Main 4=Lineup 5=StadiumEntrance 6=MatchResult 7=LockerRoom 9=Prologue …
//   (구 "InGame==9"는 Prologue 분기를 오독한 것. 실측 tag=7=LockerRoom = 경기 끝난 뒤라 정상이었음.)
//   ⚠mod_api의 `Scene::InGame`은 별개 enum(세션 진행중) — 경기 화면 여부가 아니다.
const LIVE_PLAYED_OFF: usize = 0x1598; // db+0x1598 = enum payload 무관필드(쓰레기, 진단용)
// ★★현재 재생 tick 정본(2026-07-18 seek 역추적 RE 확정):
//   ClientScene::Game payload(db+0x1340) 안에 활성 재생뷰 view#2가 임베드(payload+0x13d8=db+0x2718).
//   그 view.played_tick(+0x258) = **db+0x2970** — seek/앞뒤가 전진시키는 정밀 tick. seconds=+0x258/tickrate=db+0x2968.
//   (구후보: db+0x1598=payload 무관필드 / db+0xBA0=유휴 보조뷰 view#1. 둘 다 쓰레기였다.)
//   불변식: 0 ≤ db+0x2970 ≤ events.len(db+0x1680). 이걸로 유효성 검증.
// ★라이브 관전 활성뷰 후보 = view#2(db+0x13d8), 커서 = db+0x1630 (2026-07-18 RE, 미측정 필드).
//   화면 tick = game_view+0x258. view#1(db+0xBA0)·view#3(db+0x2970)은 유휴 → 라이브는 view#2 유력.
//   game_time 라벨(정답)과 대조해 일치하면 이 정밀 tick 채택.
// ★0.5.3 검증(2026-07-29): db(ClientDatabase) 계열은 **전부 불변**. 계열 합산 n=169 중 +0:136(2위 +0x10:102),
//   개별로도 0x1338(n=40)·0x1340(27)·0x1598(15)·0x1630(10)·0x1670(9)·0x1680(12)·0x2718·0x2968·0x2970·
//   0xba0·0x13d8 전부 +0 우세 ⇒ 값 유지.
//   ⬜단 EV_PTR_OFF(0x1678)만 사용처 5개로 표본이 얇고 +0x10:5 vs +0:4 로 근소 — 읽기전용이고
//     불변식(0 ≤ played ≤ events.len)으로 자기검증되므로 틀려도 조용히 미채택. 인게임서 재생커서 동기가
//     안 맞으면 여기부터 의심할 것.
const VIEW2_TICK_OFF: usize = 0x1630;
const EV_PTR_OFF: usize = 0x1678;      // events Vec ptr (cap@0x1670 / len@0x1680)
const EV_LEN_OFF: usize = 0x1680;
const SCENE_TAG_OFF: usize = 0x1338;   // u32. <3 → InGame
static PLAYED_TICK: AtomicU64 = AtomicU64::new(0);
static PLAYED_SRC: AtomicU64 = AtomicU64::new(0); // 0=미확보 1=SDK game_view 2=raw(라이브)
static SCENE_TAG: AtomicU64 = AtomicU64::new(0xffff);
static LAST_SCENE_TAG: AtomicU64 = AtomicU64::new(0xffff); // 전이 감지용
static TAG_LOG_N: AtomicU64 = AtomicU64::new(0);
static IN_MATCH: AtomicBool = AtomicBool::new(false); // game_time 노드 존재 = 경기 화면
// ★세르펜 툴팁 주입 (ghidra-re 0.5.1 확정)
//   ARG_STR(0xb4fda0) = i18n 치환 빌더 `arg(key, &String value)`. 컨테이너가 {Stats} 치환에 쓴다.
//   게임은 값 문자열을 **clone**해 가고(alloc+memcpy) free하지 않으므로, 호출 직전 String의
//   {cap,ptr,len}만 우리 것으로 바꿨다가 호출 후 되돌리면 안전하다(이중해제·leak 없음).
//   ⚠호출 후 반드시 원복 — 컨테이너가 자기 버퍼를 drop하기 때문.
//   팀 판정은 노드 호버 상태(+0x262)로 post_update에서 미리 정한다(프레임 로컬 의존 0).
static HOVER_TEAM: AtomicI32 = AtomicI32::new(-1); // -1=없음 0=blue 1=red
static TOOLTIP_TEXT: Mutex<String> = Mutex::new(String::new());
static GAME_TIP_TEXT: Mutex<String> = Mutex::new(String::new()); // 게임 원본 툴팁 텍스트(호버 감지 진단)
static GAME_TIME_TEXT: Mutex<String> = Mutex::new(String::new()); // 화면 경기 시각("06:42") = 재생 커서 원본
static DB_PLAYED_RAW: AtomicU64 = AtomicU64::new(0); // db+0x1598 원본(진단·정답 대조용)
static VIEW_TICK_DIAG: AtomicU64 = AtomicU64::new(0); // db+0xBA0 원본(진단·정답 대조용)
// 3-deref 단계 진단: 0=경기화면아님 1=db+0x1340 무효 2=Game+0x1dc0 무효 3=seed=0 9=성공
// 툴팁 본문이 비는 원인 진단: 0=정상 1=LIVE_SEED 미확보 2=WORLDS에 경기 없음 3=재생된 처치 없음
static TIP_FAIL: AtomicU64 = AtomicU64::new(9);
static TIP_KILLS: AtomicU64 = AtomicU64::new(0);   // 호버 시점 그 경기 총 처치수
static TIP_PLAYED: AtomicU64 = AtomicU64::new(0);  // 호버 시점 재생 커서
static TIP_WAVES: AtomicU64 = AtomicU64::new(0);   // 호버 시점 웨이브 수
static TEAM_BY_RECT: AtomicU64 = AtomicU64::new(u64::MAX); // 커서 rect 판정 결과(-1=미적중)
static TIP_SWAPS: AtomicU64 = AtomicU64::new(0);
static NODE_DUMP_DONE: AtomicBool = AtomicBool::new(false);
static TIP_SEEN: AtomicU64 = AtomicU64::new(0); // 세르펜 툴팁이 화면에 뜬 프레임 수
static TIP_NODE_LIVE: AtomicBool = AtomicBool::new(false); // 주입된 serpen_tip 노드가 인스턴스로 살아있나
// ═══ 툴팁 v3: 우리 소유 패널 가산주입 (게임 소유물 불침범 — 근본 해결) ═══
//   실패 원인 공통점 = 게임 소유물 침범(arg_str 훅=게임 함수, tooltip 노드=게임이 매프레임 재조립).
//   → 인게임 레이아웃에 **모드 소유 라벨**을 가산주입(draft_overlay/scrim 검증 프레임워크)하고,
//   호버 감지는 게임 툴팁 텍스트 read-only 관찰로(읽기는 안전), 내용은 우리 라벨에 label_set.
//   RVA(0.5.1, item_tactics ui_inject.rs서 마이그 완료된 값 재사용):
// ★asset-get copy 분화 대응(0.5.2 재확인): 이 게터는 바이트동일 모노모픽 copy가 다수(0.5.2에서
//   스켈레톤 동일 후보 26개, stride 0x230)라 **RVA만 스왑하면 엉뚱한 copy를 훅해 조용히 미발화**한다.
//   확정 방법 = 우리가 실제로 매칭하는 경로 문자열의 string-xref. serpen 은 "…ui/layout/ingame" 을 본다.
//   0.5.2 실측: "asset/base/ui/layout/ingame" 을 LEA 후 호출하는 게터 = 0x5ac950 (14회),
//               "asset/base/ui/layout/main" 도 동일하게 0x5ac950 (17회) ⇒ main/ingame 계열 같은 copy.
//               (0.5.1도 동일 구조였고 그때 값이 0x40f3d0 이었다. 부차 copy: 0.5.1 0x248e2c0 → 0.5.2 0x24956f0.)
// ★★0.5.3 (2026-07-29) — ⚠**자동매칭(_MIGRATE_053.md)의 UILOADER "확정 0x91ab0" 은 오답**이었다.
//   0x91ab0 은 skel 유일(copy 1개)인 무관 함수. 정답은 위 주석의 확정 방법(경로 문자열 LEA→직후 call)대로
//   재도출한 **0x2e1550**(0.5.3에서 30-copy 군집의 그 copy). 0.5.2에 같은 방법을 돌리면 0x5ac950 이
//   그대로 재현되어(ingame 13회·main 17회) 방법 자체가 검증됨. 콜러 사상 투표도 독립적으로 193/194 로 동일 결론.
//   ⇒ 자동매칭 값을 그냥 썼으면 **엉뚱한 copy를 훅해 UI 주입이 조용히 미발화**했을 것(소스가 경고한 그 함정).
const UILOADER_RVA: usize = 0x2e35d0;  // 0.5.4 (구0.5.3=0x2e1550) 제네릭 asset-get(main/ingame 계열). ⚠item_tactics 등과 공유 → 체이닝 필수
const UIPARSER_RVA: usize = 0x1a3ce0;  // 0.5.4 (구0.5.3=0x1a6530) .ui 텍스트 → NodeTemplate. 콜러 사상 3/3 일치.
// ★★0.5.3: 2인자 alloc(size, align) shim 이 **LTO 인라인으로 소멸**했다(0.5.2 0x25c4d30 의 어떤 부분열도
//   0.5.3 이미지에 0회 등장 / 실할당자 참조 함수가 5개 → 10,644개로 폭증 = 호출처마다 인라인).
//   ⇒ shim 이 align<=0x10 에서 tail-jmp 하던 **실할당자를 직접 호출**한다(의미 완전 동일).
//   실할당자 = GetProcessHeap() → HeapAlloc(rcx=heap, rdx=flags, r8=size) thunk.
//   0.5.2 0x25d9640 과 **바이트 동일**(rip-rel 델타만 차이)이고 HeapAlloc IAT 참조 유일 코드라 오인 불가.
//   ⚠인자 3개다 — 2인자 그대로 두면 rdx=8=HEAP_ZERO_MEMORY·r8=미초기화가 되어 랜덤 크래시.
const UIALLOC_RVA: usize = 0x29bb920;  // 0.5.4 (구0.5.3=0x28f7df0) 실할당자 직접 호출(rcx무시, rdx=flags, r8=size)
const NT_SIZE: usize = 0x90;
// ★장로 버프 표시 — 게임의 `#blue_morgard_buff:color`(ingame.ui:187)를 그대로 베낌.
//   원본: x:380 y:9 210x32 / visible:false / ignore_event:true / back_color #1f2230 / color #4a4c56
//         rounding 8 / #fx:canvas 100% / #icon:image 8,6 20x20 source=icons/morgard color #5b73ff
//         / #text:label @main#bold_label 34,1 168x30 size16 align_y:Center fit_width
//   우리 것: 모르가드 **오른쪽**(blue x=596=380+210+6 / red는 anchor_x:1 기준 -616), 아이콘=세르펜(금색).
//   ⚠조각 루트는 `#` 없이, 자식만 `#` (파서 규약). 주석/세미콜론 규칙 위반 시 조용한 미주입.
//   ⚠폭 주의: 모르가드 버프 x380~590, **blue_stat.serpen이 x=633**(rect 실측)부터.
//   210px로 넣었더니 세르펜 카운터를 통째로 덮었다 → 폭을 좁혀 모르가드 오른쪽에 밀착시킨다.
//   현행: blue x=538 / red x=-548, width=77, 텍스트만 "장로 1:30".
const ELDER_BUFF_FRAG_BLUE: &str = "blue_elder_buff:color {\nx: 538px;\ny: 9px;\nwidth: 77px;\nheight: 32px;\nvisible: false;\nignore_event: true;\nback_color: #1f2230ff;\ncolor: #ffc84aff;\nrounding: Uniform {\nrounding: 8;\n}\n\n#text:label {\n@\"asset/base/style/main#bold_label\";\nx: 6px;\ny: 1px;\nwidth: 66px;\nheight: 30px;\nsize: 16;\nalign_y: Center;\nfit_width: true;\n}\n}";
const ELDER_BUFF_FRAG_RED: &str = "red_elder_buff:color {\nx: -548px;\ny: 9px;\nanchor_x: 1;\npivot_x: 1;\nwidth: 77px;\nheight: 32px;\nvisible: false;\nignore_event: true;\nback_color: #1f2230ff;\ncolor: #ffc84aff;\nrounding: Uniform {\nrounding: 8;\n}\n\n#text:label {\n@\"asset/base/style/main#bold_label\";\nx: 6px;\ny: 1px;\nwidth: 66px;\nheight: 30px;\nsize: 16;\nalign_y: Center;\nfit_width: true;\n}\n}";
static UI_BASE: AtomicUsize = AtomicUsize::new(0);
static UILOADER_TRAMP: AtomicUsize = AtomicUsize::new(0);
static UIINJ_INSTALLED: AtomicBool = AtomicBool::new(false);
static LAST_INGAME_R: AtomicUsize = AtomicUsize::new(0);
static INJ_OK_N: AtomicU64 = AtomicU64::new(0);   // 주입 성공 횟수
static INGAME_SEEN: AtomicU64 = AtomicU64::new(0); // 로더에 ingame 경로가 지나간 횟수
static CFG_TIP_PANEL: AtomicBool = AtomicBool::new(true); // v3(안전 경로)

type UiLoaderFn = extern "win64" fn(usize, *const u8, usize) -> usize;
extern "win64" fn uiloader_detour(am: usize, path: *const u8, len: usize) -> usize {
    let t = UILOADER_TRAMP.load(Ordering::Relaxed);
    if t == 0 { return 0; }
    let r = unsafe { core::mem::transmute::<usize, UiLoaderFn>(t)(am, path, len) };
    if !path.is_null() && r > 0x10000 && len < 200 && CFG_TIP_PANEL.load(Ordering::Relaxed) {
        let s = unsafe { core::slice::from_raw_parts(path, len) };
        if s.ends_with(b"ui/layout/ingame") {
            INGAME_SEEN.fetch_add(1, Ordering::Relaxed);
            if r != LAST_INGAME_R.load(Ordering::Relaxed) {
                // ★장로 버프 표시 2개(blue/red) 주입 — 모르가드 버프와 같은 형식·위치(그 오른쪽)
                let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
                    let b = append_frag(r, ELDER_BUFF_FRAG_BLUE, b"blue_elder_buff");
                    let d = append_frag(r, ELDER_BUFF_FRAG_RED, b"red_elder_buff");
                    b && d
                })).unwrap_or(false);
                if ok { LAST_INGAME_R.store(r, Ordering::Relaxed); INJ_OK_N.fetch_add(1, Ordering::Relaxed); }
            }
        }
    }
    r
}
// 템플릿 트리서 id 탐색 (NodeTemplate: id ptr@+0x08 len@+0x10, child {cap@+0x48, ptr@+0x50, len@+0x58})
unsafe fn find_tmpl(node: usize, target: &[u8], depth: usize) -> usize {
    if node <= 0x10000 || depth > 12 { return 0; }
    let idptr = *((node + 0x08) as *const usize);
    let idlen = *((node + 0x10) as *const usize);
    if idlen == target.len() && idptr > 0x10000 {
        if core::slice::from_raw_parts(idptr as *const u8, idlen) == target { return node; }
    }
    let cptr = *((node + 0x50) as *const usize);
    let clen = *((node + 0x58) as *const usize);
    if cptr > 0x10000 && clen < 1000 {
        for i in 0..clen {
            let f = find_tmpl(cptr + i * NT_SIZE, target, depth + 1);
            if f != 0 { return f; }
        }
    }
    0
}
// 조각 1개를 템플릿 컨테이너에 append (게임 alloc으로 새 배열 → 기존+조각 memcpy → ptr/cap/len 교체.
//   옛 배열 free 금지 = leak이 정답: 게임 free는 startup 경합서 use-after-free 이력).
unsafe fn append_frag(r: usize, frag: &str, id: &[u8]) -> bool {
    if find_tmpl(r, id, 0) != 0 { return true; } // 멱등
    let base = UI_BASE.load(Ordering::Relaxed);
    if base == 0 { return false; }
    let parser: extern "win64" fn(*mut u8, *const u8, usize) = core::mem::transmute(base + UIPARSER_RVA);
    let mut out = [0u8; 0x400];
    parser(out.as_mut_ptr(), frag.as_ptr(), frag.len());
    let my = out.as_ptr().add(0x10) as usize;
    if *(my as *const usize) == usize::MAX {
        log_push(format!("[{}ms] ★조각 parse ERR: {}", now_ms(), String::from_utf8_lossy(id)));
        return false;
    }
    let ptr = *((r + 0x50) as *const usize);
    let len = *((r + 0x58) as *const usize);
    if len > 2000 { return false; }
    // 0.5.3: shim 소멸 → 실할당자 직접. 0.5.2 shim 의 align<=0x10 경로와 동일한 레지스터 상태로 부른다
    //   (rcx=size는 GetProcessHeap이 무시 / rdx=0=flags / r8=size). 실패 시 0 반환도 그대로.
    let galloc: extern "win64" fn(usize, usize, usize) -> usize = core::mem::transmute(base + UIALLOC_RVA);
    let asz = (len + 1) * NT_SIZE;
    let np = galloc(asz, 0, asz);
    if np == 0 { return false; }
    if ptr > 0x10000 && len != 0 { core::ptr::copy_nonoverlapping(ptr as *const u8, np as *mut u8, len * NT_SIZE); }
    core::ptr::copy_nonoverlapping(my as *const u8, (np + len * NT_SIZE) as *mut u8, NT_SIZE);
    *((r + 0x50) as *mut usize) = np;
    *((r + 0x48) as *mut usize) = len + 1;
    *((r + 0x58) as *mut usize) = len + 1;
    true
}
// 체이닝 install (item_tactics ui_inject.rs 검증 패턴): 현재 진입부 12B를 그대로 트램폴린에 저장 —
//   원본 프롤로그든 다른 모드 jmp든 무관 → 같은 함수를 후킹하는 모드들과 순서무관 공존.
fn install_uiloader_hook() {
    if UIINJ_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    unsafe {
        let base = GetModuleHandleW(core::ptr::null());
        if base == 0 { return; }
        UI_BASE.store(base, Ordering::Relaxed);
        let fn_addr = base + UILOADER_RVA;
        let mut cur = [0u8; 12];
        core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
        if cur[0] == 0x48 && cur[1] == 0xb8 {
            let tgt = usize::from_le_bytes(cur[2..10].try_into().unwrap());
            if tgt == uiloader_detour as usize { return; } // 이미 내 훅
        }
        let stub = VirtualAlloc(0, 64, 0x3000, 0x40);
        if stub == 0 { return; }
        let mut s: Vec<u8> = Vec::new();
        s.extend_from_slice(&cur);
        s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&(fn_addr + 0xc).to_le_bytes());
        s.extend_from_slice(&[0xff, 0xe0]);
        core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
        UILOADER_TRAMP.store(stub, Ordering::Relaxed);
        let mut patch = [0u8; 12];
        patch[0] = 0x48; patch[1] = 0xb8;
        patch[2..10].copy_from_slice(&(uiloader_detour as usize).to_le_bytes());
        patch[10] = 0xff; patch[11] = 0xe0;
        let mut old = 0u32;
        if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 { return; }
        core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
        VirtualProtect(fn_addr, 12, old, &mut old);
        log_push(format!("[{}ms] ui loader 체이닝 훅 OK fn={:#x} (RVA {:#x})", now_ms(), fn_addr, UILOADER_RVA));
    }
}

// tooltip 노드 텍스트 직접 덮기(LabelRunner text +0x160)는 소유권 경합으로 크래시(2026-07-17 실측)
//   = 게임이 매 프레임 재조립하는 공유 노드라 부적합 → 이 경로는 안 씀.
// ★정정(2026-07-26): ~~arg_str 훅=게임시작 즉시 크래시·재시도금지(0.5.1 RVA 0xb4fda0)~~
//   → 그 크래시는 **구 0.5.1 RVA(0xb4fda0) 한정**. 현행 **0.5.2 RVA(0xfef190)에서는 크래시 없이
//   안전 동작** = 실제 설치·인게임 검증완(install_arg_str_hook, ~L2406). 툴팁 주입 정답 경로 확정.
//   (구 실패 원인엔 프롤로그 미대조로 잘못된 12B를 패치한 것도 있었음 — 현행은 정상 seam.)
static EV_PTR: AtomicU64 = AtomicU64::new(0);
static EV_LEN: AtomicU64 = AtomicU64::new(0);
static SPAWN_INSTALLED: AtomicBool = AtomicBool::new(false);
static SPAWN_CAP_N: AtomicU64 = AtomicU64::new(0);

// 스폰 클로저 진입 콜백 (asm 스텁이 saved=rsp로 호출): saved+0=rcx=env.
unsafe extern "C" fn cap_spawn(saved: *mut u64, _rsp_entry: usize) -> u64 {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved.is_null() { return; }
        let env = *saved as usize;
        if env < 0x10000 { return; }
        SPAWN_CAP_N.fetch_add(1, Ordering::Relaxed);
        // ⚠이 훅이 잡는 provider/tid는 화면 경기와 무관함이 실측됨(경기 목록 ★LIVE 매칭 0건).
        //   틀린 lp가 게이트를 굳혀 CURRENT_ATTR을 고정시키므로 LIVE_PROVIDER는 세팅하지 않는다.
        //   LIVE_TID는 진단 표시용으로만 남김(게이트 미사용).
        LIVE_TID.store(GetCurrentThreadId() as u64, Ordering::Relaxed);
    }));
    0
}
// ★★★런처 진입 콜백: r8 = seed, retaddr로 화면 경기 여부 판정.
//   asm 스텁이 push한 순서(install_stub_generic 참조): rcx,rdx,r8,r9,r10,r11,rbx,rdi,rsi,r12 = 10개.
//   → saved[0]=rcx, saved[2]=r8(seed), saved[10]=원래 [rsp] = **retaddr**.
unsafe extern "C" fn cap_launcher(saved: *mut u64, _rsp: usize) -> u64 {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved.is_null() { return; }
        LAUNCH_N.fetch_add(1, Ordering::Relaxed);
        let seed = *saved.add(2);      // r8
        let ret = *saved.add(10) as usize; // return address
        let base = GetModuleHandleW(core::ptr::null());
        if base == 0 || ret <= base { return; }
        let rva = ret - base;
        LAUNCH_LAST_RVA.store(rva as u64, Ordering::Relaxed);
        // ★진단: 런처 호출처(retaddr)를 distinct로 수집 — 다시보기 진입 경로의 retaddr를 알아내
        //   게이트에 추가하기 위함(라이브 기준 A/B만으론 다시보기를 못 잡을 수 있음).
        let mut new_rva = false;
        {
            let mut g = LAUNCH_RVAS.lock().unwrap_or_else(|e| e.into_inner());
            let v = g.get_or_insert_with(Vec::new);
            match v.iter_mut().find(|(r, _)| *r == rva as u64) {
                Some((_, c)) => *c += 1,
                None => if v.len() < 16 { v.push((rva as u64, 1)); new_rva = true; },
            }
        } // ⚠log_push는 락 밖에서 (LAUNCH_RVAS→PROBE_LOG 중첩 회피)
        // ◆진단(07-24 제보): 신규 retaddr 최초 발화 타임스탬프 — 세트 진입 시각과 대조해 미커버 경로 식별
        if new_rva && CFG_PROBE_LOG.load(Ordering::Relaxed) {
            log_push(format!("[{}ms] ◆런처 신규 retaddr rva={:#x} seed={:#x}{}", now_ms(), rva, seed,
                if rva == LAUNCHER_RET_A || rva == LAUNCHER_RET_B || rva == LAUNCHER_RET_C || rva == LAUNCHER_RET_D { " ★게이트" } else { "" }));
        }
        if rva == LAUNCHER_RET_A || rva == LAUNCHER_RET_B || rva == LAUNCHER_RET_C || rva == LAUNCHER_RET_D {
            RENDER_SEED.store(seed, Ordering::Relaxed); // ★화면에 재생할 경기 확정
            LIVE_SEED.store(seed, Ordering::Relaxed);   // 사이드테이블 게이트도 이걸로(3-deref 불필요)
            // ★(07-24) 화면 세트 fp 도출용: rcx=out Game 저장 + fp 미확정으로 리셋. 런처가 리턴해 Game이
            //   채워진 뒤(다음 프레임 post_update, 메인스레드) resolve_render_fp가 provider→fp를 계산한다.
            LAUNCH_GAME.store(*saved.add(0) as usize as u64, Ordering::Relaxed);
            RENDER_FP.store(0, Ordering::Relaxed);
            RENDER_PROV.store(0, Ordering::Relaxed);
            LAUNCH_HIT.fetch_add(1, Ordering::Relaxed);
            log_push(format!("[{}ms] ★★재생경기 선택 포착: seed={:#x} (retaddr rva={:#x})", now_ms(), seed, rva));
        }
    }));
    0
}
fn install_launcher_hook() {
    if LAUNCHER_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    let ok = unsafe { install_stub_generic(LAUNCHER_RVA, 12, cap_launcher as usize, &LAUNCHER_PROLOGUE) };
    log_push(format!("[{}ms] launcher hook {:#x} = {}", now_ms(), LAUNCHER_RVA, if ok { "OK" } else { "실패(프롤로그 mismatch)" }));
}
static LAUNCHER_INSTALLED: AtomicBool = AtomicBool::new(false);

// ★★재생기 렌더 스텝 FUN_140872950 = 매 렌더마다 rcx=game_view를 받아 그 뷰의 played_tick(+0x258)/초(+0x250)
//   을 갱신. game_time 라벨이 읽는 그 값. ClientData의 game_view 3개(활성 1 + 유휴 2)가 각각 렌더되는데,
//   활성(화면) 뷰의 +0x258이 진짜 화면 tick. 유휴 뷰는 tick이 ~6/18로 작다 → 프레임 최대값이 활성 뷰.
//   (2026-07-18: 고정 오프셋 뷰 3개 전부 유휴/리플레이라 실패 → 렌더 시점에만 활성 뷰를 알 수 있음.)
const RENDER_STEP_RVA: usize = 0xaa06c0; // 0.5.4 (구0.5.3=0x960df0). 프롤로그 12B 동일·크기 4575=4575·문자열 3/3·VIEW_TICK_REL(+0x258) 불변 확인.
const RENDER_STEP_PROLOGUE: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
const VIEW_TICK_REL: usize = 0x258; // game_view.played_tick
static RENDER_TICK: AtomicU64 = AtomicU64::new(0); // 프레임 내 최대 뷰 tick(= 활성 뷰). post_update가 swap(0)로 소비.
static RENDER_STEP_INSTALLED: AtomicBool = AtomicBool::new(false);
static RENDER_HOOK_N: AtomicU64 = AtomicU64::new(0);
unsafe extern "C" fn cap_render(saved: *mut u64, _rsp: usize) -> u64 {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved.is_null() { return; }
        let view = *saved.add(0) as usize; // rcx = game_view
        if view < 0x10000 { return; }
        if let Some(tick) = safe_read_u64(view + VIEW_TICK_REL) {
            if tick > 0 && tick < 10_000_000 {
                RENDER_TICK.fetch_max(tick, Ordering::Relaxed); // 3뷰 중 최대 = 활성(화면) 뷰
                RENDER_HOOK_N.fetch_add(1, Ordering::Relaxed);
            }
        }
    }));
    0
}
fn install_render_step_hook() {
    if RENDER_STEP_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    let ok = unsafe { install_stub_generic(RENDER_STEP_RVA, 12, cap_render as usize, &RENDER_STEP_PROLOGUE) };
    log_push(format!("[{}ms] ★렌더스텝 훅 {:#x} = {}", now_ms(), RENDER_STEP_RVA, if ok { "OK" } else { "실패(프롤로그 mismatch)" }));
}
// runner_ctor FUN_1419c9470(0x19c9470): 화면 경기(관전+직접플레이) sim Game 생성 시만 발화(배경 리그 제외).
//   rcx=out슬롯=sim Game. provider=*(Game+0x1660). item_tactics 검증 지점.
const RUNNER_CTOR_RVA: usize = 0x13b7050; // 0.5.4 (구0.5.3=0xeba490). 전파투표 1위 + ghidra 독립확인(콜사이트 6곳 컨테이너까지 완전대응)·프롤로그 12B 동일.
const RUNNER_PROLOGUE: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
static RCTOR_N: AtomicU64 = AtomicU64::new(0);

// runner_ctor 진입: game=*saved(rcx) → provider=*(game+0x1660) → LIVE_PROVIDER.
unsafe extern "C" fn cap_runner_ctor(saved: *mut u64, _rsp: usize) -> u64 {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved.is_null() { return; }
        let game = *saved as usize;
        if game < 0x10000 || game >= (1usize << 47) { return; }
        RCTOR_N.fetch_add(1, Ordering::Relaxed);
        LIVE_TID.store(GetCurrentThreadId() as u64, Ordering::Relaxed); // ★화면 sim 전용 스레드
        if let Some(prov) = safe_read_u64(game + 0x1dc0) {
            if prov > 0x10000 && (prov as usize) < (1usize << 47) {
                LIVE_PROVIDER.store(prov as usize, Ordering::Relaxed);
            }
        }
    }));
    0
}
// asm 스텁 트램폴린(레지스터 저장→cap_fn→복원→원본명령→복귀). 원본 인자 무변경.
unsafe fn install_stub_generic(rva: usize, orig_len: usize, cap_fn: usize, prologue: &[u8]) -> bool {
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { return false; }
    let fn_addr = base + rva;
    for i in 0..prologue.len() { if *((fn_addr + i) as *const u8) != prologue[i] { return false; } }
    let stub = VirtualAlloc(0, 256, 0x3000, 0x40);
    if stub == 0 { return false; }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x41, 0x54, 0x56, 0x57, 0x53, 0x41, 0x53, 0x41, 0x52, 0x41, 0x51, 0x41, 0x50, 0x52, 0x51]);
    s.extend_from_slice(&[0x48, 0x89, 0xe1]);       // mov rcx, rsp
    s.extend_from_slice(&[0x48, 0x89, 0xe3]);       // mov rbx, rsp
    s.extend_from_slice(&[0x48, 0x83, 0xe4, 0xf0]); // and rsp, -16
    s.extend_from_slice(&[0x48, 0x83, 0xec, 0x20]); // sub rsp, 0x20
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff, 0xd0]);             // call rax
    s.extend_from_slice(&[0x48, 0x89, 0xdc]);       // mov rsp, rbx
    s.extend_from_slice(&[0x59, 0x5a, 0x41, 0x58, 0x41, 0x59, 0x41, 0x5a, 0x41, 0x5b, 0x5b, 0x5f, 0x5e, 0x41, 0x5c]);
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    let mut old = 0u32;
    if VirtualProtect(fn_addr, orig_len, 0x40, &mut old) == 0 { return false; }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    true
}
fn install_spawn_hooks() {
    if SPAWN_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    for &rva in SPAWN_HOOKS.iter() {
        let ok = unsafe { install_stub_generic(rva, 12, cap_spawn as usize, &SPAWN_PROLOGUE) };
        log_push(format!("[{}ms] spawn hook {:#x} = {}", now_ms(), rva, if ok { "OK" } else { "실패" }));
    }
    let rc = unsafe { install_stub_generic(RUNNER_CTOR_RVA, 12, cap_runner_ctor as usize, &RUNNER_PROLOGUE) };
    log_push(format!("[{}ms] runner_ctor hook {:#x} = {}", now_ms(), RUNNER_CTOR_RVA, if rc { "OK" } else { "실패" }));
}

// ── Skia flush dispatcher (tfm2_fog 프로덕션 검증 seam) ──
//   FUN_1409cd2b0: (rcx=ctx, rdx=state, r8=SkCanvas/Assets, r9=RenderCommand, [stack]=alpha f32).
//   tag3/4(이미지) 커맨드 +0x10=텍스처 키 ptr / +0x18=키 len. 키 교체 = 다른 텍스처로 그려짐.
//   텍스처 미로드 시 NULL→no draw (graceful, 크래시 아님 — fog 실증).
//   ⚠ tfm2_fog와 동일 함수 후킹 → fog와 동시 활성 금지.
//   ★0.5.0_3 진짜 dispatcher = 0x9d3470 (ghidra 구조규명, fog의 0x9cd2b0은 0.4.14 stale값).
//   프롤로그 8push(12B relocatable) + sub rsp,0xa18. 재진입 +0xc.
//   tag3/4(이미지) = 키ptr@+0x10 / 키len@+0x18. 미로드 시 텍스처조회(0x9ddb70) NULL→no draw(크래시 X).

// 인월드 draw 빌더 FUN_140414800(0.5.0_3, 렌더 스레드·on-screen 매치만). 프롤로그 8push(12B).
//   param_4(r9): +0x230=data / +0x238=vtable. (*(vtable+0x48))(data)→game obj → obj+0x1660→provider(+0xeab8=seed) 후보.
// ── 이펙트 push 범용함수 (FUN_141fcdcc0, 0.5.0_3) ──
//   fn(rcx=대상 Entity, rdx=Effect(0x120: name_len@+0, name@+4)). hot+병렬.
//   name "serpen" prefix일 때만 콜스택 캡처 → 세르펜 버프 어플라이어(콜러) 특정.
// 0.5.1 = 0x1f15940 (구0.5.0_3=0x1fcdcc0, migrate_rva.py 마스크시그 유일매치 2026-07-16).
//   ★세르펜 팀버프의 실제 적용 경로 = 이 effect push. GameSetting 어플라이어(0x21df4f0)는
//   morgard(epic_minion_buff) 전용임이 0.5.1 실측으로 확정됨(serpen_permanent_buff 무발화).
//   교차확증: ghidra-re의 어플라이어 분석에도 "FUN_141f15940으로 적용"으로 같은 주소가 등장.

// ── objective 팀버프-from-GameSetting 어플라이어 (FUN_141f47970, 0.5.0_3) ──
//   kind 점프테이블 kind1 → 0x1f47970. GameSetting 버프를 팀 챔피언에 뿌리는 유일 함수.
//   ★serpen(serpen_permanent_buff) vs morgard(epic_minion_buff) 판별 프로브 대상.
//   시그니처 fn(rcx,rdx,r8=ScoreState,r9=GameSetting container[+8=GameSetting]).
//   버프소스 name_len@GS+0xe38 / name@GS+0xe3c / 스탯블록@GS+0xe90.
//   ⚠migrate_rva.py는 NONE — 로직은 98.87% 동일이고 vtable 슬롯이 +0x68 밀린 레이아웃 변경뿐인데,
//   마스크시그가 구조체 변위를 고정바이트로 남겨 초반 창에서 불일치. 프롤로그 12B·시그니처·
//   GS 오프셋(+0xe38/+0xe3c/+0xe90)은 전부 0.5.0_3과 동일.

// ───────────────────────── WinAPI ─────────────────────────
type HMODULE = isize; type DWORD = u32; type BOOL = i32;
const PAGE_READWRITE: u32 = 0x04;
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleExW(f: DWORD, name: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, n: DWORD) -> DWORD;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old_protect: *mut u32) -> BOOL;
    fn AddVectoredExceptionHandler(first: u32, handler: VehHandler) -> usize;
    fn GetCurrentThreadId() -> u32;
    fn GetModuleHandleW(name: *const u16) -> usize; // null=exe base
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, size: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
}

// ───────────────────────── SEH 안전 r/w (item_editor 검증본) ─────────────────────────
//  SEH[]: 0=active 1=tid 2=land_rip 3=land_rsp 4=land_rbp 5=code_lo 6=code_hi 7=faults
#[repr(C)]
struct ExceptionRecord { code: u32, flags: u32, rec: usize, addr: usize, nparams: u32, _p: u32, params: [usize; 15] }
#[repr(C)]
struct ExceptionPointers { rec: *mut ExceptionRecord, ctx: *mut core::ffi::c_void }
type VehHandler = extern "system" fn(*mut ExceptionPointers) -> i32;

// ★2026-08-02 전환: 전역 `SEH[8]` + `SEH_BUSY` 스핀락 → **스레드별 TLS**.
//   구: safe_copy 가 전역 상태 하나를 공유하느라 `while SEH_BUSY.swap(true) { spin_loop() }` 로
//   **모든 rayon 워커를 직렬화**했다. 일정넘김은 배경 경기 sim이 8스레드로 도는데 세르펜 detour가
//   매 틱 safe_read/safe_write 를 수십 회 부르므로 이 스핀락이 곧 모드 최대 비용원이었다
//   (외부 샘플러 실측: sim 워커 CPU 샘플의 24~37%가 이 모드 호출 체인).
//   VEH 핸들러는 **폴트난 바로 그 스레드 위에서** 실행되므로 자기 TLS를 읽으면 된다
//   ⇒ 락 불필요 + tid 대조도 불필요(TLS 자체가 스레드 스코프라 구조적으로 보장).
//   ⚠VEH 안전 4요건 유지 = Cell 배열 + `const` 초기화 + **Drop 없음** + `try_with`
//     ⇒ 핸들러 안에 할당·락·패닉 경로가 없다. CLAUDE.md §3 / [[tfm2-mod-safety]] §2 정정.
//   ⚠§4 "후킹 경로 thread_local 금지"와 의도적으로 다른 판단(§2가 명시한 예외). 참조 구현 =
//     `tfm2_item_tactics\src\lib.rs` L195~277 (2026-07-22 전환·프로덕션 검증본)을 그대로 이식.
//   레이아웃은 구 [u64;8]과 동일(asm 오프셋 그대로). idx1(구 tid)은 미사용으로 남긴다.
#[repr(C)]
struct SehTls { v: [core::cell::Cell<u64>; 8] }
thread_local! {
    static SEH_T: SehTls = const { SehTls { v: [const { core::cell::Cell::new(0) }; 8] } };
}
#[inline(always)]
fn seh_ptr() -> *mut u64 {
    // Cell<u64>는 repr(transparent) → [Cell<u64>;8]과 [u64;8]은 레이아웃 동일.
    SEH_T.with(|s| s.v.as_ptr() as *mut u64)
}
static SEH_INSTALLED: AtomicBool = AtomicBool::new(false);
// 구 SEH[7](전역 배열 슬롯)의 대체 — TLS 전환으로 폴트 카운터가 스레드별이 되므로 합산은 여기서.
static SEH_FAULTS: AtomicU64 = AtomicU64::new(0);

extern "system" fn seh_veh(p: *mut ExceptionPointers) -> i32 {
    const CONTINUE_EXECUTION: i32 = -1;
    const CONTINUE_SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return CONTINUE_SEARCH; }
        let rec = (*p).rec;
        if rec.is_null() || (*rec).code != 0xC0000005 { return CONTINUE_SEARCH; }
        // ★TLS 전환: 이 핸들러는 폴트난 그 스레드에서 도므로 자기 TLS가 곧 그 스레드의 상태
        //   (구 tid 대조 불필요). try_with = TLS 소멸중이면 조용히 패스(패닉 금지 요건).
        let Ok(g) = SEH_T.try_with(|s| s.v.as_ptr() as *mut u64) else { return CONTINUE_SEARCH; };
        if *g.add(0) == 0 { return CONTINUE_SEARCH; }
        let ctx = (*p).ctx as usize;
        if ctx == 0 { return CONTINUE_SEARCH; }
        let rip = *((ctx + 0xF8) as *const u64);
        if rip < *g.add(5) || rip >= *g.add(6) { return CONTINUE_SEARCH; }
        *((ctx + 0xF8) as *mut u64) = *g.add(2);
        *((ctx + 0x98) as *mut u64) = *g.add(3);
        *((ctx + 0xA0) as *mut u64) = *g.add(4);
        SEH_FAULTS.fetch_add(1, Ordering::Relaxed);   // 원자 증가 = 할당·락 없음(VEH 안전)
        CONTINUE_EXECUTION
    }
}
fn seh_install() {
    if SEH_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    unsafe { AddVectoredExceptionHandler(1, seh_veh); }
}
#[inline(never)]
unsafe fn safe_copy(dst: *mut u8, src: *const u8, len: usize) -> bool {
    if !SEH_INSTALLED.load(Ordering::Relaxed) { return false; }
    // ★락 없음: 상태가 스레드별이라 워커끼리 경합하지 않는다(구 SEH_BUSY 스핀락 제거).
    let g = seh_ptr();
    let ok: u64;
    core::arch::asm!(
        "lea rax, [rip + 200f]",
        "mov [{g} + 40], rax",
        "lea rax, [rip + 201f]",
        "mov [{g} + 48], rax",
        "lea rax, [rip + 202f]",
        "mov [{g} + 16], rax",
        "mov [{g} + 24], rsp",
        "mov [{g} + 32], rbp",
        "mov qword ptr [{g} + 0], 1",
        "cld",
        "200:",
        "rep movsb",
        "201:",
        "mov {ok}, 1",
        "jmp 203f",
        "202:",
        "mov {ok}, 0",
        "203:",
        "mov qword ptr [{g} + 0], 0",
        g = in(reg) g,
        ok = out(reg) ok,
        inout("rcx") len => _,
        inout("rdi") dst => _,
        inout("rsi") src => _,
        out("rax") _,
    );
    ok != 0
}
unsafe fn safe_read_u64(addr: usize) -> Option<u64> {
    let mut b = [0u8; 8];
    if safe_copy(b.as_mut_ptr(), addr as *const u8, 8) { Some(u64::from_le_bytes(b)) } else { None }
}
unsafe fn safe_read_i32(addr: usize) -> Option<i32> {
    let mut b = [0u8; 4];
    if safe_copy(b.as_mut_ptr(), addr as *const u8, 4) { Some(i32::from_le_bytes(b)) } else { None }
}
// ★2026-08-02: 기본 경로에서 VirtualProtect 제거.
//   이 함수의 실제 대상은 전부 **게임 힙**(엔티티 템플릿 스탯·스프라이트 이름 ptr/len·툴팁 문자열
//   슬롯)이라 이미 쓰기 가능하다. 그런데 write 1회마다 VirtualProtect 2회(=시스템콜 2회)를 돌았고,
//   매 틱 32스탯 루프가 이걸 곱해 **틱당 ~64 시스템콜**이 됐다. 페이지 보호 변경은 프로세스 전역 +
//   타 코어 TLB 무효화라 **다른 sim 스레드까지 같이 느려진다**
//   (외부 샘플러 실측: NtProtectVirtualMemory 가 busy CPU 샘플의 7.9%, 그 호출자가 이 모드).
//   ⇒ 먼저 그냥 쓰고(SEH 보호), **실패했을 때만** VirtualProtect 후 재시도하고 원복한다.
//     읽기전용 페이지에 쓰는 호출자가 나중에 생겨도 동작은 그대로다(폴백이 받아냄).
unsafe fn safe_write_bytes(addr: usize, data: &[u8]) -> bool {
    if data.is_empty() || data.len() > 4096 || addr < 0x10000 { return false; }
    if safe_copy(addr as *mut u8, data.as_ptr(), data.len()) { return true; }
    let mut old = 0u32;
    if VirtualProtect(addr, data.len(), PAGE_READWRITE, &mut old) == 0 { return false; }
    let ok = safe_copy(addr as *mut u8, data.as_ptr(), data.len());
    let mut o2 = 0u32;
    VirtualProtect(addr, data.len(), old, &mut o2);
    ok
}
unsafe fn safe_write_i32(addr: usize, v: i32) -> bool { safe_write_bytes(addr, &v.to_le_bytes()) }
unsafe fn safe_write_u64(addr: usize, v: u64) -> bool { safe_write_bytes(addr, &v.to_le_bytes()) }

// ───────────────────────── 파일/로그 ─────────────────────────
fn dll_path() -> Option<PathBuf> {
    unsafe {
        let addr = dll_path as *const () as usize;
        let mut h: HMODULE = 0;
        if GetModuleHandleExW(0x4 | 0x2, addr as *const u16, &mut h) == 0 || h == 0 { return None; }
        let mut buf = [0u16; 4096];
        let n = GetModuleFileNameW(h, buf.as_mut_ptr(), buf.len() as DWORD);
        if n == 0 { return None; }
        Some(PathBuf::from(String::from_utf16_lossy(&buf[..n as usize])))
    }
}
fn mod_dir() -> Option<PathBuf> { dll_path()?.parent().map(|p| p.to_path_buf()) }
fn write_log(name: &str, content: &str) {
    if let Some(p) = mod_dir().map(|d| d.join(name)) { let _ = fs::write(p, content); }
}
fn read_text(name: &str) -> Option<String> { fs::read_to_string(mod_dir()?.join(name)).ok() }
fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }
// poison-safe 로그 push (배경 sim 병렬 진입 대비)
fn log_push(s: String) {
    let mut v = PROBE_LOG.lock().unwrap_or_else(|e| e.into_inner());
    // 순환버퍼(07-24 2차): 만석 시 가장 오래된 줄 폐기 — 1차 진단서 2.6분만에 만석→이후 30분 유실 교훈
    if v.len() >= 600 { v.remove(0); }
    v.push(s);
}
fn probe_flush() {
    // ★ flush는 항상 (진입 로그는 CFG_PROBE_LOG 게이트지만, 속성/config 로그는 무조건 기록)
    // ⚠락 순서 = detour와 동일(WORLDS→PROBE_LOG). 역전하면 데드락.
    //   ★2026-08-02: POOL/ELDER는 무락 스냅샷(ATTR_SNAP)이 되어 락 순서에서 빠졌다.
    let now = now_ms();
    // ★성능: 스로틀을 덤프 생성 "앞"으로 옮김(2026-07-19). 이전엔 거대한 진단 문자열(락 3개 +
    //   format! 수십 개 + 경기목록 순회)을 **매 프레임** 만들고 파일 쓰기만 1초로 걸러서,
    //   초당 60번 만든 덤프 중 59번을 그냥 버렸다. 아래 조건은 실제 flush 조건과 동일하게 유지.
    {
        let n = PROBE_LOG.lock().unwrap_or_else(|e| e.into_inner()).len();
        if n == 0 { return; }
        if n == LOG_FLUSHED_LEN.load(Ordering::Relaxed)
            && now.saturating_sub(LAST_FLUSH_MS.load(Ordering::Relaxed)) < 1000 { return; }
    } // ⚠guard drop 필수 — 아래에서 WORLDS→PROBE_LOG 순으로 다시 잡는다(중첩 시 락순서 역전).
    let dump = {
        // ★2026-08-02: POOL/ELDER 락 → 무락 스냅샷(ATTR_SNAP). 미발행(=cfg 로드 전)이면 빈 값으로 표기.
        let sn = attrs();
        let pool: &[Attr] = sn.map(|s| s.pool.as_slice()).unwrap_or(&[]);
        let elder: Option<&Attr> = sn.and_then(|s| s.elder.as_ref());
        let mut wg = WORLDS.lock().unwrap_or_else(|e| e.into_inner());
        let lp = LIVE_PROVIDER.load(Ordering::Relaxed);
        let lt = LIVE_TID.load(Ordering::Relaxed);
        let ls = LIVE_SEED.load(Ordering::Relaxed);
        let mut s = format!("[게이트] ★LIVE_SEED={:#x} RENDER_FP={:04x} prov={:#x} fp실패={} LIVE_PROVIDER={:#x} RENDER_TID={} LIVE_TID={} rctor_n={} spawn_n={} CURRENT_ATTR={}\n",
            ls, RENDER_FP.load(Ordering::Relaxed) & 0xffff, RENDER_PROV.load(Ordering::Relaxed),
            FP_FAIL_N.load(Ordering::Relaxed), lp, RENDER_TID.load(Ordering::Relaxed), lt,
            RCTOR_N.load(Ordering::Relaxed), SPAWN_CAP_N.load(Ordering::Relaxed), CURRENT_ATTR.load(Ordering::Relaxed));
        let played = PLAYED_TICK.load(Ordering::Relaxed);
        let live_ws = wg.as_ref().and_then(|w| pick_live(w, ls));
        let lsim = live_ws.map(|w| w.sim_tick).unwrap_or(0);
        s.push_str(&format!("[재생] played_tick={} src={} 조회={} | 화면경기 sim_tick={} 비율(played/sim)={:.2}\n",
            played,
            match PLAYED_SRC.load(Ordering::Relaxed) { 1 => "SDK", 3 => "frames미준비", 4 => "경기화면아님", 6 => "game_time라벨(폴백)", 8 => "★★렌더스텝(정밀·화면일치)", _ => "미확보" },
            match PLAYED_RESOLVED.load(Ordering::Relaxed) { 1 => "성공", 2 => "구간없음", _ => "미시도" },
            lsim, if lsim > 0 { played as f64 / lsim as f64 } else { 0.0 }));
        // ⚠★여기서 kill_counts() 호출 금지 — 이 블록은 WORLDS 락을 쥐고 있는데 kill_counts()가
        //   WORLDS를 재잠금 → 같은 스레드 데드락 = 게임 프리즈. LIVE_SEED==0(메뉴)일 땐 조기반환이라
        //   멀쩡하다가 경기 식별 직후 첫 flush에서 멈추는 패턴(2026-07-17 "멈춘다" 실측 원인).
        //   → 이미 쥔 live_ws로 인라인 계산.
        let (kb, kr) = live_ws.map(|w| (
            w.kills.iter().filter(|(t, k, _)| *t == 0 && *k <= played).count(),
            w.kills.iter().filter(|(t, k, _)| *t == 1 && *k <= played).count(),
        )).unwrap_or((0, 0));
        s.push_str(&format!("[툴팁v3] ingame로드감지={}회 주입성공={}회 패널인스턴스={} 호버프레임={}회 갱신={}회 팀별처치(B/R)=({}, {})\n",
            INGAME_SEEN.load(Ordering::Relaxed), INJ_OK_N.load(Ordering::Relaxed),
            TIP_NODE_LIVE.load(Ordering::Relaxed), TIP_SEEN.load(Ordering::Relaxed),
            TIP_SWAPS.load(Ordering::Relaxed), kb, kr));
        // ★게임이 직접 세는 팀별 처치수(+0xed50/+0xed58)와 우리 Vec 읽기를 대조 — 툴팁 스택수가
        //   2인데 우리 kills가 1건뿐인 원인 규명용(Vec 오프셋/갱신 문제 구분).
        // ★provider = ws.rcx(세르펜 detour rcx). LIVE_PROVIDER는 무효라 쓰레기가 나왔다.
        let sp = live_ws.map(|w| w.rcx as usize).unwrap_or(0);
        let (gcb, gcr, vlen) = if sp >= 0x10000 {
            (unsafe { safe_read_u64(sp + KILLS_BLUE_OFF) }.unwrap_or(u64::MAX),
             unsafe { safe_read_u64(sp + KILLS_RED_OFF) }.unwrap_or(u64::MAX),
             unsafe { safe_read_u64(sp + KILLS_LEN_OFF) }.unwrap_or(u64::MAX))
        } else { (u64::MAX, u64::MAX, u64::MAX) };
        s.push_str(&format!("[게임카운터] blue={} red={} | Vec.len={} (우리 kills={}건)\n",
            gcb as i64, gcr as i64, vlen as i64, live_ws.map(|w| w.kills.len()).unwrap_or(0)));
        // ★재생커서 3자 대조: game_time 라벨(정답) vs db+0x1598 vs db+0xBA0 — 어느 db 필드가
        //   화면 tick과 일치하는지 판별(일치하면 문자열 파싱 대신 그 직접 tick 필드로 교체).
        s.push_str(&format!("[재생커서] played={} game_time={:?} | db+0x1630={} db+0x1598={} events.len={} sim={}\n",
            PLAYED_TICK.load(Ordering::Relaxed),
            GAME_TIME_TEXT.lock().unwrap_or_else(|e| e.into_inner()).clone(),
            VIEW_TICK_DIAG.load(Ordering::Relaxed) as i64, DB_PLAYED_RAW.load(Ordering::Relaxed) as i64,
            EV_LEN.load(Ordering::Relaxed), lsim));
        s.push_str(&format!("[툴팁진단] 실패이유={} (호버시점 처치={}건 played={} 웨이브={}개)\n",
            match TIP_FAIL.load(Ordering::Relaxed) {
                0 => "정상(본문생성됨)", 1 => "★LIVE_SEED 미확보", 2 => "★WORLDS에 화면경기 없음",
                3 => "★그 팀의 화면 처치 0건 또는 웨이브매칭 실패", _ => "미시도(호버 안함)" },
            TIP_KILLS.load(Ordering::Relaxed), TIP_PLAYED.load(Ordering::Relaxed), TIP_WAVES.load(Ordering::Relaxed)));
        s.push_str(&format!("[툴팁생성시점] 판정팀={} 그팀의화면처치수={} (0이면 '없음' 표시가 정상)
",
            match TIP_TEAM_AT.load(Ordering::Relaxed) { 0 => "blue", 1 => "red", _ => "미생성" },
            TIP_WANT.load(Ordering::Relaxed)));
        s.push_str(&format!("[툴팁훅] 호버감지={}회 arg_str교체={}회 판정팀={} (rect판정={}) | 게임툴팁={:?}\n  주입본문={:?}\n",
            TIP_SEEN.load(Ordering::Relaxed), TIP_SWAPS.load(Ordering::Relaxed),
            match HOVER_TEAM.load(Ordering::Relaxed) { 0 => "blue", 1 => "red", 2 => "모호(양팀표시)", _ => "호버안함" },
            match TEAM_BY_RECT.load(Ordering::Relaxed) as i64 { 0 => "blue", 1 => "red", _ => "미적중(폴백사용)" },
            GAME_TIP_TEXT.lock().unwrap_or_else(|e| e.into_inner()).replace('\n', "⏎"),
            TOOLTIP_TEXT.lock().unwrap_or_else(|e| e.into_inner()).replace('\n', "⏎")));
        s.push_str(&format!("[scene] ★game_time노드={} (경기화면판정) tag={} events.ptr={:#x} len={}\n",
            IN_MATCH.load(Ordering::Relaxed), SCENE_TAG.load(Ordering::Relaxed) as i64,
            EV_PTR.load(Ordering::Relaxed), EV_LEN.load(Ordering::Relaxed)));
        if let Some(w) = live_ws {
            let mut tl: Vec<(u64, u64, i32)> = w.waves.iter().map(|(i, (t, a))| (*i, *t, *a)).collect();
            tl.sort_by_key(|(i, _, _)| *i);
            let tls: Vec<String> = tl.iter().map(|(i, t, a)| format!("#{}@{}→{}", i, t, a)).collect();
            s.push_str(&format!("[웨이브] (웨이브idx@spawn_tick→속성idx) {}\n", tls.join(" ")));
            // 화면 경기 처치 이력 원본(검증용): 처치 tick이 어느 웨이브 구간에 드는지 눈으로 대조
            let kl: Vec<String> = w.kills.iter()
                .map(|(t, k, ki)| format!("{}#{}@{}{}", if *t == 0 { "B" } else { "R" }, ki, k,
                    if *k <= played { "" } else { "(미재생)" })).collect();
            s.push_str(&format!("[처치이력] {}\n",
                if kl.is_empty() { "없음".into() } else { kl.join(" ") }));
            { // ★화면 자체 이력(07-24, 툴팁·장로버프의 유일 소스) — 위 sim측 처치이력과 대조용
                let skg = SCREEN_KILLS.lock().unwrap_or_else(|e| e.into_inner());
                let sks: Vec<String> = skg.as_ref().map(|v| v.iter()
                    .map(|(t, ki, gt)| format!("{}#{}@{}", if *t == 0 { "B" } else { "R" }, ki, gt)).collect())
                    .unwrap_or_default();
                s.push_str(&format!("[화면이력] {}\n", if sks.is_empty() { "없음".into() } else { sks.join(" ") }));
            }
            // ★툴팁 미리보기: serpen_logs[i] = 웨이브 i (1:1) → 매칭 불필요. 재생 기준(played 이하)만.
            let nm = |i: i32| -> String {
                if i == ELDER_IDX { elder.as_ref().map(|a| a.display_name.clone()).unwrap_or_else(|| "?".into()) }
                else { pool.get(i as usize).map(|a| a.display_name.clone()).unwrap_or_else(|| format!("idx{}", i)) }
            };
            for team in 0..2u64 {
                let mut cnt: HashMap<i32, u32> = HashMap::new();
                for (t, ktick, ki) in w.kills.iter() {
                    if *ktick > played { continue; } // 재생 기준(미래 처치 제외)
                    if *t != team { continue; }
                    if let Some(&(_, a)) = w.waves.get(ki) { *cnt.entry(a).or_insert(0) += 1; } // 색=처치 인덱스의 웨이브
                }
                let mut list: Vec<String> = cnt.iter().map(|(a, c)| format!("{} x{}", nm(*a), c)).collect();
                list.sort();
                s.push_str(&format!("[툴팁:{}] {}\n", if team == 0 { "blue" } else { "red" },
                    if list.is_empty() { "처치 없음".into() } else { list.join(", ") }));
            }
        }
        s.push_str(&format!("[db] InGame={} db={:#x} (db→provider 링크 없음 확정 → 런처 훅으로 식별)\n",
            DB_INGAME.load(Ordering::Relaxed), DB_PTR.load(Ordering::Relaxed)));
        s.push_str(&format!("[☠처형] 임계={}% 지속={}틱 증폭훅={} 처형={}회 | 훅호출={}회 음수delta={}회 임계권진단={}회\n",
            EXEC_THR_PCT.load(Ordering::Relaxed), EXEC_DURATION.load(Ordering::Relaxed),
            if DMGA_TRAMP.load(Ordering::Relaxed) != 0 && DMGB_TRAMP.load(Ordering::Relaxed) != 0 { "A+B설치됨" }
            else if DMGA_TRAMP.load(Ordering::Relaxed) != 0 { "★A만(B실패)" }
            else { "★미설치" },
            AMP_FIRE_N.load(Ordering::Relaxed), AMP_CALL_N.load(Ordering::Relaxed),
            AMP_SEEN_N.load(Ordering::Relaxed), EXEC_CAND_N.load(Ordering::Relaxed)));
        s.push_str(&format!("[☠처형/화면] 화면경기 발화={}회 (0이면 화면 경기에서 조건 미성립 = 원인 그쪽)\n",
            AMP_FIRE_LIVE_N.load(Ordering::Relaxed)));
        s.push_str(&format!("[☠처형탈락] 비챔대상={} disc={} 팀={} TLS월드={} dense={} 창없음={} 임계초과={} 공격자비챔={}\n",
            AMP_REJ[0].load(Ordering::Relaxed), AMP_REJ[1].load(Ordering::Relaxed),
            AMP_REJ[2].load(Ordering::Relaxed), AMP_REJ[3].load(Ordering::Relaxed),
            AMP_REJ[4].load(Ordering::Relaxed), AMP_REJ[5].load(Ordering::Relaxed),
            AMP_REJ[6].load(Ordering::Relaxed), AMP_REJ[8].load(Ordering::Relaxed)));
        s.push_str(&format!("[장로UI] 잔여틱(B/R)=({}, {}) 표시갱신={}회 노드미발견={}회\n",
            ELDER_LEFT_B.load(Ordering::Relaxed), ELDER_LEFT_R.load(Ordering::Relaxed),
            ELDER_UI_N.load(Ordering::Relaxed), ELDER_NODE_MISS.load(Ordering::Relaxed)));
        let bsrc = |team: usize| -> String {
            let v = LAST_BUFF_SRC[team].load(Ordering::Relaxed);
            if v == 0 { "없음".into() } else { format!("처치#{} kill={}", (v >> 32) & 0xff, v & 0xffffffff) }
        };
        s.push_str(&format!("[장로UI출처] 블루={} | 레드={} | 버프ON로그={}회\n",
            bsrc(0), bsrc(1), ELDER_ANOMALY_N.load(Ordering::Relaxed)));
        s.push_str(&format!("[화면카운터] 세르펜 화면처치수 B={} R={} | 장로버프시작(game_time) B={} R={}\n",
            SERPEN_CNT_ONSCREEN[0].load(Ordering::Relaxed), SERPEN_CNT_ONSCREEN[1].load(Ordering::Relaxed),
            ELDER_BUFF_START[0].load(Ordering::Relaxed), ELDER_BUFF_START[1].load(Ordering::Relaxed)));
        s.push_str(&format!("[★런처] 발화={}회 화면경기적중={}회 RENDER_SEED={:#x} 최근retaddr_rva={:#x}\n",
            LAUNCH_N.load(Ordering::Relaxed), LAUNCH_HIT.load(Ordering::Relaxed),
            RENDER_SEED.load(Ordering::Relaxed), LAUNCH_LAST_RVA.load(Ordering::Relaxed)));
        { // 런처 호출처 distinct — 다시보기에서 적중=0이면 여기 목록 중 하나가 다시보기 경로다
            let g = LAUNCH_RVAS.lock().unwrap_or_else(|e| e.into_inner());
            let list: Vec<String> = g.as_ref().map(|v| v.iter()
                .map(|(r, c)| format!("{:#x}×{}{}", r, c,
                    if *r == LAUNCHER_RET_A as u64 || *r == LAUNCHER_RET_B as u64 || *r == LAUNCHER_RET_C as u64 || *r == LAUNCHER_RET_D as u64 { "★게이트" } else { "" }))
                .collect()).unwrap_or_default();
            s.push_str(&format!("[런처호출처] {}\n", if list.is_empty() { "없음".into() } else { list.join(" ") }));
        }
        if let Some(w) = wg.as_mut() {
            // 죽은 경기 정리. ⚠화면 경기(LIVE_SEED)는 절대 지우지 않는다 — sim이 화면보다 앞서
            //   끝나면 detour가 더 안 오지만(last_ms 정지) 화면은 그 프레임을 재생 중이라 waves가 필요.
            w.retain(|k, ws| k.0 == ls || now.saturating_sub(ws.last_ms) < 60_000);
            let mut list: Vec<(&(u64, u64), &WorldState)> = w.iter().collect();
            list.sort_by_key(|(_, ws)| core::cmp::Reverse(ws.last_ms));
            s.push_str(&format!("[경기별] 총 {}파티션 (★=화면 seed, fp=세트지문 하위16비트)\n", list.len()));
            let rf = RENDER_FP.load(Ordering::Relaxed);
            for ((seed, fp), ws) in list.iter().take(8) {
                let live = if ls != 0 && *seed == ls {
                    if rf != 0 && *fp == rf { "★" } else { "☆" } // ☆=화면 seed지만 다른 세트 파티션
                } else { " " };
                let nm = |i: i32| -> String {
                    if i == ELDER_IDX { elder.as_ref().map(|a| a.display_name.clone()).unwrap_or_else(|| "?".into()) }
                    else { pool.get(i as usize).map(|a| a.display_name.clone()).unwrap_or_else(|| "?".into()) }
                };
                s.push_str(&format!("  {}seed={:#x} fp={:04x} 웨이브#{} spawn_tick={} 색='{}' sim_tick={} 처치={}건 최근={}ms전\n",
                    live, seed, *fp & 0xffff, ws.wave_idx, ws.spawn_tick, nm(ws.current), ws.sim_tick,
                    ws.kills.len(), now.saturating_sub(ws.last_ms)));
            }
        }
        s
    };
    let v = PROBE_LOG.lock().unwrap_or_else(|e| e.into_inner());
    let n = v.len();
    if n == 0 { return; }
    // 경기별 상태는 계속 변하므로, 로그 줄수가 200줄 만석으로 고정된 뒤에도 1초 주기로 flush
    if n == LOG_FLUSHED_LEN.load(Ordering::Relaxed)
        && now.saturating_sub(LAST_FLUSH_MS.load(Ordering::Relaxed)) < 1000 { return; }
    LOG_FLUSHED_LEN.store(n, Ordering::Relaxed);
    LAST_FLUSH_MS.store(now, Ordering::Relaxed);
    // seh_faults = 구 전역 SEH[7] → TLS 전환(2026-08-02)으로 전 스레드 합산 카운터에서 읽는다.
    let head = format!("enter_count={} seh_faults={}\n{}",
        ENTER_COUNT.load(Ordering::Relaxed), SEH_FAULTS.load(Ordering::Relaxed), dump);
    write_log("serpen_probe.txt", &(head + &v.join("\n")));
}
static LAST_FLUSH_MS: AtomicU64 = AtomicU64::new(0);

// ───────────────────────── cfg 로드 ─────────────────────────
fn load_cfg() {
    let Some(txt) = read_text("serpen_probe.cfg") else { return; };
    for line in txt.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let Some((k, v)) = line.split_once('=') else { continue; };
        let (k, v) = (k.trim(), v.split('#').next().unwrap_or("").trim()); // 인라인 주석(#) 제거
        let on = v == "1" || v.eq_ignore_ascii_case("true");
        match k {
            // 실기능 스위치(원소/장로 온오프·elder_after)의 정본은 config/serpen.cfg(load_attrs) — 여기선 진단만.
            "probe_log" => CFG_PROBE_LOG.store(on, Ordering::Relaxed), // 진단 로그 파일(기본 off)
            // 장로 버프 UI 표시 지연(tick): 처치 후 사망 연출 여유분만큼 늦게 띄워 "장로 살아있는데 버프 뜸" 방지.
            //   버프 지속시간은 처치 기준 그대로(표시 시작만 지연). 기본 90(3초).
            "buff_show_delay_tick" => BUFF_SHOW_DELAY.store(v.parse().unwrap_or(90), Ordering::Relaxed),
            _ => {} // 구 진단/실험 게이트(재설계 2026-07-26에 제거)는 무시
        }
    }
}

// ───────────────────────── 세르펜 detour ─────────────────────────
type SerpenFn = extern "win64" fn(u64, u64, u64, u64, u64, u64);
extern "win64" fn serpen_detour(rcx: u64, rdx: u64, r8: u64, r9: u64, a5: u64, a6: u64) {
    // ★ 속성 시스템: 세르펜 리스폰 감지 → 속성 배정 → +0x108 템플릿 덮기 → 처치 시 자동 전파
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        serpen_apply_attr(rcx, rdx, a5);
    }));
    // 원본 실행 (게임 로직 정상 진행)
    let t = SERPEN_TRAMP.load(Ordering::Relaxed);
    if t != 0 {
        let orig: SerpenFn = unsafe { core::mem::transmute(t) };
        orig(rcx, rdx, r8, r9, a5, a6);
    }
}

// ───────────────────────── i18n (텍스트 분리) ─────────────────────────
//   사용자 표시 문자열은 전부 text/<lang>.txt(key = value, BOM 없는 UTF-8)에서 로드.
//   언어 선택 = config/serpen.cfg 의 `language`(기본 en). 키 미존재 시 en 폴백 → 그래도 없으면 key 자체.
//   ⚠게임 툴팁 감지 키워드("세르펜"/"누적 효과" 등, update_tooltip/parse_stacks)는 **게임 UI 언어** 의존이라
//     이 i18n 범위 밖(모드 출력이 아니라 게임 텍스트 매칭). 그쪽은 별도.
static I18N: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);
static I18N_LANG: Mutex<Option<String>> = Mutex::new(None);
fn tr(key: &str) -> String {
    let g = I18N.lock().unwrap_or_else(|e| e.into_inner());
    match g.as_ref().and_then(|m| m.get(key)) {
        Some(v) => v.clone(),
        None => key.to_string(), // 최종 폴백: 키 노출(누락 즉시 눈에 띔)
    }
}
// ── 미니 JSON 파서 (tfm2_scrim 이식) — text/serpen.i18n 파싱용 ──
#[allow(dead_code)] // 파서 완전성: Bool/Num/Arr variant는 serpen.i18n에 안 나와도 유지
enum JsonValue { Null, Bool(bool), Num(f64), Str(String), Arr(Vec<JsonValue>), Obj(Vec<(String, JsonValue)>) }
impl JsonValue {
    fn as_obj(&self) -> Option<&Vec<(String, JsonValue)>> { if let JsonValue::Obj(o) = self { Some(o) } else { None } }
    fn get<'b>(&'b self, key: &str) -> Option<&'b JsonValue> { self.as_obj()?.iter().find(|(k, _)| k == key).map(|(_, v)| v) }
    fn as_str(&self) -> Option<&str> { if let JsonValue::Str(s) = self { Some(s.as_str()) } else { None } }
}
struct JsonParser<'a> { b: &'a [u8], i: usize }
impl<'a> JsonParser<'a> {
    fn new(s: &'a str) -> Self { JsonParser { b: s.as_bytes(), i: 0 } }
    fn skip_ws(&mut self) {
        while self.i < self.b.len() {
            match self.b[self.i] { b' ' | b'\t' | b'\r' | b'\n' | b',' => self.i += 1, _ => break }
        }
    }
    fn parse_value(&mut self) -> Option<JsonValue> {
        self.skip_ws();
        if self.i >= self.b.len() { return None; }
        match self.b[self.i] {
            b'{' => self.parse_object(),
            b'[' => self.parse_array(),
            b'"' => self.parse_string().map(JsonValue::Str),
            b't' => { self.i += 4; Some(JsonValue::Bool(true)) }
            b'f' => { self.i += 5; Some(JsonValue::Bool(false)) }
            b'n' => { self.i += 4; Some(JsonValue::Null) }
            _ => self.parse_number(),
        }
    }
    fn parse_string(&mut self) -> Option<String> {
        if self.b.get(self.i) != Some(&b'"') { return None; }
        self.i += 1;
        let mut out: Vec<u8> = Vec::new();
        while self.i < self.b.len() {
            let c = self.b[self.i]; self.i += 1;
            match c {
                b'"' => return Some(String::from_utf8_lossy(&out).into_owned()),
                b'\\' => {
                    let e = *self.b.get(self.i)?; self.i += 1;
                    match e {
                        b'n' => out.push(b'\n'), b't' => out.push(b'\t'), b'r' => out.push(b'\r'),
                        b'b' => out.push(0x08), b'f' => out.push(0x0c),
                        b'"' => out.push(b'"'), b'\\' => out.push(b'\\'), b'/' => out.push(b'/'),
                        b'u' => {
                            if self.i + 4 <= self.b.len() {
                                if let Ok(hex) = std::str::from_utf8(&self.b[self.i..self.i + 4]) {
                                    if let Ok(cp) = u32::from_str_radix(hex, 16) {
                                        if let Some(ch) = char::from_u32(cp) {
                                            let mut buf = [0u8; 4];
                                            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                                        }
                                    }
                                }
                                self.i += 4;
                            }
                        }
                        other => out.push(other),
                    }
                }
                _ => out.push(c),
            }
        }
        None
    }
    fn parse_number(&mut self) -> Option<JsonValue> {
        let start = self.i;
        while self.i < self.b.len() {
            match self.b[self.i] { b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E' => self.i += 1, _ => break }
        }
        std::str::from_utf8(&self.b[start..self.i]).ok()?.parse::<f64>().ok().map(JsonValue::Num)
    }
    fn parse_array(&mut self) -> Option<JsonValue> {
        self.i += 1;
        let mut arr = Vec::new();
        loop {
            self.skip_ws();
            if self.i >= self.b.len() { return None; }
            if self.b[self.i] == b']' { self.i += 1; break; }
            arr.push(self.parse_value()?);
        }
        Some(JsonValue::Arr(arr))
    }
    fn parse_object(&mut self) -> Option<JsonValue> {
        self.i += 1;
        let mut pairs = Vec::new();
        loop {
            self.skip_ws();
            if self.i >= self.b.len() { return None; }
            if self.b[self.i] == b'}' { self.i += 1; break; }
            let k = self.parse_string()?;
            self.skip_ws();
            if self.b.get(self.i) != Some(&b':') { return None; }
            self.i += 1;
            let v = self.parse_value()?;
            pairs.push((k, v));
        }
        Some(JsonValue::Obj(pairs))
    }
}
// serpen.i18n 의 한 언어 블록(lang > namespace > key)을 "namespace.key" flat map에 병합.
fn merge_lang(map: &mut HashMap<String, String>, root: &JsonValue, lang: &str) {
    let Some(langobj) = root.get(lang) else { return; };
    let Some(nss) = langobj.as_obj() else { return; };
    for (ns, nsobj) in nss {
        if let Some(keys) = nsobj.as_obj() {
            for (k, val) in keys {
                if let Some(s) = val.as_str() { map.insert(format!("{}.{}", ns, k), s.to_string()); }
            }
        }
    }
}
// 게임 UI 언어 = <게임>/config/game/base.json 의 "lang" (예: "en", "ko").
//   ⚠우리 언어를 게임과 맞춰야 함 — 안 그러면 폰트 불일치로 글자 깨짐(영어폰트+한글=□, 07-25 실측).
fn game_lang() -> Option<String> {
    let p = mod_dir()?.parent()?.parent()?.join("config").join("game").join("base.json");
    let txt = fs::read_to_string(p).ok()?;
    JsonParser::new(&txt).parse_value()?.get("lang")?.as_str().map(|s| s.to_lowercase())
}
fn load_i18n() {
    // 언어 = ①게임 언어 자동(base.json) ②serpen.cfg `language`가 auto/빈값 아니면 override.
    //   ⚠참조(#asset)가 인라인서 안 풀려(07-25 실측) 우리가 serpen.i18n을 직접 파싱.
    let mut lang = game_lang().unwrap_or_else(|| "en".to_string());
    if let Some(txt) = read_text("config/serpen.cfg") {
        for line in txt.lines() {
            if let Some((k, v)) = line.trim().split_once('=') {
                if k.trim() == "language" {
                    let val = v.split('#').next().unwrap_or("").trim().to_lowercase();
                    if !val.is_empty() && val != "auto" { lang = val; } // 명시 override
                }
            }
        }
    }
    let mut map = HashMap::new();
    if let Some(txt) = read_text("text/serpen.i18n") {
        if let Some(root) = JsonParser::new(&txt).parse_value() {
            merge_lang(&mut map, &root, "en");         // en 베이스(폴백)
            if lang != "en" { merge_lang(&mut map, &root, &lang); } // 선택 언어로 덮어쓰기
        }
    }
    log_push(format!("[{}ms] i18n 로드: lang={} 키={}개", now_ms(), lang, map.len()));
    *I18N.lock().unwrap_or_else(|e| e.into_inner()) = Some(map);
    *I18N_LANG.lock().unwrap_or_else(|e| e.into_inner()) = Some(lang);
}
fn parse_attr(name: &str, txt: &str) -> Attr {
    // 표시명 = i18n `attr.<파일명>`(예: attr.fire). 미존재면 파일명 그대로.
    //   ⚠게임 i18n 에셋 참조(#asset?..)는 인라인(조합 문자열 중간)서 안 풀림(2026-07-25 실측) → 우리가 조회.
    let dn_key = format!("attr.{}", name);
    let dn = tr(&dn_key);
    let dn = if dn == dn_key { name.to_string() } else { dn };
    let mut a = Attr { name: name.to_string(), display_name: dn, weight: 1, ..Default::default() };
    for line in txt.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let Some((k, v)) = line.split_once('=') else { continue; };
        let (k, v) = (k.trim(), v.split('#').next().unwrap_or("").trim()); // 인라인 주석(#) 제거
        match k {
            // display_name은 i18n(attr.<name>)에서 옴 — cfg엔 없음. 하위호환: cfg에 있으면 우선 적용.
            "display_name" => a.display_name = v.to_string(),
            "sprite" => a.sprite = v.to_string(),
            "weight" => a.weight = v.parse().unwrap_or(1),
            "execute_hp_threshold_percent" => a.execute_hp_pct = v.parse().unwrap_or(0),
            // 처형 능력 지속시간(sim tick, 30틱=1초). 장로 처치 tick부터 이만큼 동안만 처형이 발동.
            "execute_duration_tick" => a.execute_duration = v.parse().unwrap_or(0),
            _ => {
                if let Some(&(_, idx)) = STAT_KEYS.iter().find(|(key, _)| *key == k) {
                    a.stats[idx] = v.parse().unwrap_or(0);
                }
            }
        }
    }
    // 스프라이트 이름 교체 버퍼 + modid 네임스페이스 키 (예: serpen_fire)
    if !a.sprite.is_empty() {
        a.name_buf = format!("{}_monster", a.sprite);
        a.anim_key = format!("asset/{}/aseprite_resources/ingame/{}#anim", MOD_ID, a.sprite);
        a.sheet_key = format!("asset/{}/aseprite_resources/ingame/{}#sheet", MOD_ID, a.sprite);
        // 단축 별칭: sprite "serpen_fire" → 'f'. elder만 'x'(earth와 첫 글자 충돌 회피).
        let sh = match a.sprite.rsplit('_').next().unwrap_or("") {
            "fire" => "f", "wind" => "w", "earth" => "e", "hextech" => "h",
            "chemtech" => "c", "elder" => "x", "ocean" => "o", _ => "",
        };
        if !sh.is_empty() {
            let k = format!("asset/{}/s/{}", MOD_ID, sh);
            if k.len() <= VAN_BASE_KEY.len() { a.short_key = k; }
        }
    }
    a
}
fn load_attrs() {
    if ATTRS_LOADED.swap(true, Ordering::Relaxed) { return; }
    let Some(dir) = mod_dir().map(|d| d.join("config")) else { return; };
    let Ok(rd) = fs::read_dir(&dir) else {
        log_push(format!("[{}ms] config 폴더 없음: {}", now_ms(), dir.display())); return;
    };
    let mut pool: Vec<Attr> = Vec::new();
    let mut elder: Option<Attr> = None;
    for e in rd.flatten() {
        let path = e.path();
        if path.extension().and_then(|s| s.to_str()) != Some("cfg") { continue; }
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
        let Ok(txt) = fs::read_to_string(&path) else { continue; };
        if stem == "serpen" { // 전역 설정
            for line in txt.lines() {
                if let Some((k, v)) = line.trim().split_once('=') {
                    let (k, v) = (k.trim(), v.split('#').next().unwrap_or("").trim()); // 인라인 주석(#) 제거
                    let on = v == "1" || v.eq_ignore_ascii_case("true");
                    // ★elder_after는 config에서 1-based("N번째부터 장로")로 입력받아 내부 0-based로 변환.
                    //   예: config 4 → 내부 3 → wave_idx>=3(=4번째 세르펜)부터 장로. 끄기=elder_enabled=0.
                    match k {
                        "elder_after" => ELDER_AFTER.store(v.parse::<u32>().unwrap_or(4).max(1) - 1, Ordering::Relaxed),
                        "elemental_enabled" => CFG_ELEMENTAL.store(on, Ordering::Relaxed), // 원소 온오프(serpen.cfg)
                        "elder_enabled" => CFG_ELDER.store(on, Ordering::Relaxed),         // 장로 온오프(serpen.cfg)
                        // ★스프라이트 위치 보정 — 재빌드 없이 게임 재시작만으로 조정 가능.
                        "sprite_center_fix" => SPR_CENTER_FIX.store(on, Ordering::Relaxed),
                        "sprite_offset_x" => SPR_OFF_X.store(v.parse::<i32>().unwrap_or(0), Ordering::Relaxed),
                        "sprite_offset_y" => SPR_OFF_Y.store(v.parse::<i32>().unwrap_or(0), Ordering::Relaxed),
                        _ => {}
                    }
                }
            }
            continue;
        }
        let attr = parse_attr(&stem, &txt);
        if stem == "elder" || attr.execute_hp_pct > 0 {
            // 처형 파라미터를 static으로 — detour(sim 스레드)가 lock 없이 읽어야 하므로
            EXEC_THR_PCT.store(attr.execute_hp_pct.max(0) as u64, Ordering::Relaxed);
            EXEC_DURATION.store(attr.execute_duration, Ordering::Relaxed);
            elder = Some(attr);
        }
        else { pool.push(attr); }
    }
    pool.sort_by(|a, b| a.name.cmp(&b.name)); // 결정론 순서(파일시스템 순서 비의존)
    let names: Vec<&str> = pool.iter().map(|a| a.name.as_str()).collect();
    log_push(format!("[{}ms] config 로드: 풀 {}종={:?} 장로={} elder_after={}",
        now_ms(), pool.len(), names, elder.is_some(), ELDER_AFTER.load(Ordering::Relaxed)));
    publish_attrs(pool, elder);   // ★락 없는 스냅샷 발행(구본은 leak — ATTR_SNAP 주석 참조)
    *WORLDS.lock().unwrap_or_else(|e| e.into_inner()) = Some(HashMap::new());
}


// ═══════════ 장로 처형 (MobaMode::tick post-hook) ═══════════
// 의미: "장로 버프를 가진 팀이 적을 **때렸을 때**, 그 적 HP가 임계 이하면 처형" (LoL 장로와 동일).
//   버프 = 장로 처치 tick부터 execute_duration 동안만 유효(시간제한).
//   "때렸을 때" = entity+0x670+team*8 (데미지 적용 0x1f147e0이 남기는 180틱 윈도우) != 0.
//   ⇒ 이 윈도우를 조건에 넣어야 킬 크레딧/골드가 그 팀에 정상 귀속된다(게임 계약: 처형은 가해자의
//     데미지 기록 뒤에 온다). 처형 자체는 게임과 동일하게 entity+0x658 = 0 raw write.
// ⚠데미지 함수(0x1f147e0) 훅이 아니라 tick 훅인 이유: 그쪽엔 provider가 안 들어와 **어느 경기인지
//   구분 불가**(배경 30~40경기가 같은 team 0/1을 씀). tick은 rcx=World라 경기가 명확하다.
unsafe fn team_champions(w: usize, team: u64, out: &mut Vec<usize>) {
    out.clear();
    let (Some(p_ptr), Some(p_len)) = (safe_read_u64(w + W_PLAYER_DENSE), safe_read_u64(w + W_PLAYER_DENSE + 8)) else { return };
    let (Some(cs), Some(cs_len)) = (safe_read_u64(w + W_CHAMP_SLOTS), safe_read_u64(w + W_CHAMP_SLOTS + 8)) else { return };
    let (Some(cd), Some(cd_len)) = (safe_read_u64(w + W_CHAMP_DENSE), safe_read_u64(w + W_CHAMP_DENSE + 8)) else { return };
    if p_ptr < 0x10000 || cs < 0x10000 || cd < 0x10000 { return; }
    for i in 0..(p_len as usize).min(32) {
        let p = p_ptr as usize + i * PLAYER_STRIDE;
        if safe_read_u64(p + P_TEAM) != Some(team) { continue; }
        if safe_read_u64(p + P_CHAMP_TAG).unwrap_or(0) == 0 { continue; } // 챔피언 없음
        let key = safe_read_u64(p + P_CHAMP_KEY).unwrap_or(u64::MAX) as usize;
        if key >= cs_len as usize { continue; }
        let slot = cs as usize + key * 0x10;
        if safe_read_i32(slot).unwrap_or(0) != 1 { continue; } // 빈 슬롯
        let d = safe_read_u64(slot + 8).unwrap_or(u64::MAX) as usize;
        if d >= cd_len as usize { continue; }
        let e = cd as usize + d * CHAMP_STRIDE;
        if safe_read_u64(e + ENTITY_KIND_OFF) != Some(CHAMP_KIND) { continue; }
        out.push(e);
    }
}
unsafe fn execute_pass(w: usize) {
    let thr = EXEC_THR_PCT.load(Ordering::Relaxed);
    if thr == 0 { return; } // 장로 config에 처형 임계 없음
    // ★★화면 경기 게이트 (2026-07-17 "관전 경기가 멈춤" 실측 원인).
    //   MobaMode::tick은 배경 리그 경기 30~40개가 전부 부른다. 게이트가 없으면:
    //   ① 배경 경기 챔피언까지 즉사시켜 리그 결과가 오염되고(로그에 tick=17721/19643/24923 등
    //      서로 다른 시간축의 처형이 섞여 나온 것이 증거)
    //   ② 매 틱 × 전 경기 × 챔피언 전수 스캔(safe_read=VEH 경유 수백회)으로 sim이 스로틀돼
    //      재생 커서(db+0x1598)가 sim을 따라잡고 화면이 정지한다. 크래시가 아니라 프리즈.
    //   → 배경 경기는 seed read 1회로 즉시 탈출. 화면 경기 식별 = CURRENT_MATCH_DETECT.md.
    let rs = RENDER_SEED.load(Ordering::Relaxed);
    if rs == 0 || safe_read_u64(w + SEED_OFF) != Some(rs) { return; }
    let ea = ELDER_AFTER.load(Ordering::Relaxed) as u64;
    // 빠른 탈출: 아직 장로 웨이브를 아무도 처치 안 했으면 끝(대부분의 경기·틱이 여기서 리턴)
    let klen = safe_read_u64(w + KILLS_LEN_OFF).unwrap_or(0);
    if klen == 0 || klen > 128 || (ea > 0 && klen <= ea) { return; }
    let Some(kptr) = safe_read_u64(w + KILLS_PTR_OFF) else { return };
    if kptr < 0x10000 { return; }
    let cur = safe_read_u64(w + SIM_TICK_OFF).unwrap_or(0);
    let dur = EXEC_DURATION.load(Ordering::Relaxed);
    // 장로 웨이브(인덱스 >= elder_after) 처치 팀 중, 버프가 아직 유효한 팀
    let mut et = [false; 2];
    for i in (ea as usize)..(klen as usize) {
        let e = kptr as usize + i * 16;
        let t = safe_read_u64(e).unwrap_or(u64::MAX);
        let kt = safe_read_u64(e + 8).unwrap_or(0);
        if t > 1 { continue; }
        if dur == 0 || cur <= kt.saturating_add(dur) { et[t as usize] = true; } // 시간제한 버프
    }
    if !et[0] && !et[1] { return; }
    let mut foes: Vec<usize> = Vec::new();
    for team in 0..2u64 {
        if !et[team as usize] { continue; }
        team_champions(w, 1 - team, &mut foes); // 상대팀 챔피언
        for &e in foes.iter() {
            let hp = safe_read_u64(e + O_CUR_HP).unwrap_or(0);
            if hp == 0 { continue; }
            // ★"때렸을 때"만 — 이 팀의 피해 윈도우가 살아있어야 킬 크레딧이 정상 귀속된다
            if safe_read_u64(e + O_DMG_WINDOW + (team as usize) * 8).unwrap_or(0) == 0 { continue; }
            let mx = safe_read_u64(e + O_EXEC_MAXHP).unwrap_or(0);
            if mx == 0 { continue; }
            // 게임 Inquisitor ult와 동일 판정식: curHP <= maxHP * thr / 100 (u64, 내림)
            if hp <= mx.saturating_mul(thr) / 100 {
                // ⚠★HP 필드 write 전면 폐기(2026-07-18 확정, 3회 실측 실패):
                //   ① curHP=0 대입 → 죽음 부기 누락, sim 모순 → 무한 재시뮬(프리즈)
                //   ② baseHP 동반 감소 → maxHP 재계산 오염 → 상대 최대체력 폭증
                //   ③ curHP를 음수(-99999)로 → u64 랩어라운드 = 1800경 HP 표시 + 재시뮬 재발
                //   공통 원인 = sim 바깥에서 HP를 조작해 "죽음을 날조"하는 접근 자체가 불가.
                //   → 실동작 = 증폭 훅(install_dmg_hook, FUN_1422474a0 pre-hook)이 담당.
                //     여기(mobatick post-hook)는 교차검증 진단만: "임계권 대상 존재" 표시.
                let n = EXEC_CAND_N.fetch_add(1, Ordering::Relaxed);
                if n < 6 {
                    log_push(format!("[{}ms] ☠임계권(진단) team={} 적={:#x} hp={}/{} (임계 {}%, tick={})",
                        now_ms(), team, e, hp, mx, thr, cur));
                }
            }
        }
    }
}
type MobaTickFn = extern "win64" fn(u64, u64, u64, u64);
// ★처치 팀 귀속 정본 = 팀별 카운터(+0xed50/58) 델타 감시 (2026-07-18 RE §6 b안).
//   serpen_logs Vec 직접 판독은 폐기: len==cap이면 grow로 ptr 재할당+구버퍼 free → 메인 스레드가
//   죽은 버퍼를 읽으면 팀 필드가 전부 0(블루)으로 보임(= "레드가 잡았는데 블루 버프" 실사고 원인).
//   카운터는 World 인라인 필드(포인터 체이스 없음) + 게임 킬메시지와 같은 basic block에서
//   록스텝 증가(exe 전체 유일 사이트 0x2202ac8) → 정의상 항상 일치. 같은 sim 스레드 post-tick
//   에서 읽으므로 레이스도 없음. 뒤로감기(World 재생성→카운터 0 리셋) 시 (team,tick) 중복
//   체크로 결정론 재시뮬 재적립을 흡수.
// ⚠카운터 이전값은 반드시 **World 인스턴스(포인터)별**로 추적 — 게임이 World를 딥클론
//   (0x142210900)하므로 같은 seed 인스턴스가 진행도 다르게 동시에 존재한다. seed 키로 공유하면
//   두 인스턴스(카운터 0↔2)가 번갈아 "델타"를 만들어 매 틱 가짜 킬 적립(실측 9,073건 폭발).
static LIVE_SIM_TICK: AtomicU64 = AtomicU64::new(0); // 화면 경기 최신 sim tick(재생 커서 미준비 시 폴백)
unsafe fn track_kills(w: usize) {
    let seed = safe_read_u64(w + SEED_OFF).unwrap_or(0);
    if seed == 0 { return; }
    let cb = safe_read_u64(w + KILLS_BLUE_OFF).unwrap_or(0);
    let cr = safe_read_u64(w + KILLS_RED_OFF).unwrap_or(0);
    if cb > 512 || cr > 512 { return; }
    let tick = safe_read_u64(w + SIM_TICK_OFF).unwrap_or(0);
    // ★화면 경기의 최신 sim tick을 매 틱 노출(mobatick는 세르펜 사망 후에도 매 틱 호출) — 일시정지/
    //   라이브로 재생 커서가 미준비일 때 played 폴백에 사용(방금 죽인 장로 버프 표시용, 2026-07-18).
    if tick != 0 && seed == LIVE_SEED.load(Ordering::Relaxed) { LIVE_SIM_TICK.store(tick, Ordering::Relaxed); }
    // ★★처치 이력 정본 = 게임 serpen_logs 배열 **통째 읽기**(2026-07-18 재설계).
    //   ⚠카운터 델타 추론 폐기: 같은 seed의 World 인스턴스(재시뮬 클론)가 여러 개 서로 다른 진행도로
    //     존재해 같은 킬이 인스턴스마다 다른 인덱스로 중복 기록되고 팀이 밀렸다
    //     (실측: 게임 blue=2/red=1인데 우리 집계는 4건 전부 블루).
    //   → serpen_logs 원본은 팀·틱이 게임이 직접 쓴 값이라 오귀속이 없다. sim 스레드 post-tick이라
    //     writer가 이미 끝난 안정 상태(메인 스레드 읽기의 stale/free 문제와 무관).
    let klen = safe_read_u64(w + KILLS_LEN_OFF).unwrap_or(0);
    if klen > 128 { return; }                       // 비정상 → 무시
    let Some(kptr) = safe_read_u64(w + KILLS_PTR_OFF) else { return };
    if klen > 0 && (kptr < 0x10000 || kptr >= (1u64 << 47)) { return; }
    // ★성능(2026-07-19): 처치 이력은 세르펜이 죽을 때만 바뀌는데, 예전엔 **매 틱·매 World**마다
    //   배열 전체를 다시 읽고(최대 128×2 VEH read) Vec을 할당한 뒤 전역 WORLDS 락까지 잡았다.
    //   sim 스레드 30~40개 × 30틱/초 = 초당 수십만 VEH read + 락 경합 → 메인 렌더 스레드가 밀림.
    //   → (길이 + 마지막 원소)로 서명을 만들어 안 바뀌었으면 즉시 반환. 서명 계산은 VEH read 2회.
    //   마지막 원소를 포함시키는 이유: 뒤로감기로 길이는 같은데 내용이 달라지는 경우를 놓치지 않기 위함.
    let sig = if klen == 0 { 1 } else {
        let e = kptr as usize + (klen as usize - 1) * 16;
        let lt = safe_read_u64(e).unwrap_or(u64::MAX);
        let lk = safe_read_u64(e + 8).unwrap_or(u64::MAX);
        klen ^ (lt << 40) ^ (lk << 8) ^ 0x5eed
    };
    let slot = ((w >> 4) & (KC_SLOTS - 1)) as usize;
    if KC_ADDR[slot].load(Ordering::Relaxed) == w as u64
        && KC_SIG[slot].load(Ordering::Relaxed) == sig { return; }
    let mut v: Vec<(u64, u64, u64)> = Vec::with_capacity(klen as usize); // (team, tick, index)
    for i in 0..klen as usize {
        let e = kptr as usize + i * 16;
        let Some(t) = safe_read_u64(e) else { return };      // 읽기 실패 = 통째로 버림
        let Some(kt) = safe_read_u64(e + 8) else { return };
        if t > 1 { return; }                                  // 오염된 팀값 → 부분반영 금지, 통째 버림
        v.push((t, kt, i as u64));
    }
    let Some(fp) = fp_for_world(w) else { return }; // (seed,fp) 파티션 — 미확정이면 다음 틱 재시도
    let mut wg = WORLDS.lock().unwrap_or_else(|e| e.into_inner());
    let Some(map) = wg.as_mut() else { return };
    let Some(ws) = map.get_mut(&(seed, fp)) else { return };
    // ◆진단(07-24 제보): 킬 적립 순간의 (로그idx ki) vs (캠프 웨이브idx) 대조 — 1:1 가정이 깨지는
    //   지점을 잡는 핵심 로그. ⚠ws.wave_idx는 같은 seed 클론들이 공유해 근사치(마지막 detour 기준).
    if CFG_PROBE_LOG.load(Ordering::Relaxed) {
        let is_live = seed == LIVE_SEED.load(Ordering::Relaxed);
        if v.len() > ws.kills.len() {
            let n = KILLGROW_LOG_N.fetch_add(1, Ordering::Relaxed);
            // 07-24 2차: 배경경기가 캡 100을 다 먹음 → 화면경기 무제한 + 배경 30건만
            if is_live || n < 30 {
                let new: Vec<String> = v[ws.kills.len()..].iter()
                    .map(|(t, kt, ki)| format!("{}#{}@{}", if *t == 0 { "B" } else { "R" }, ki, kt)).collect();
                log_push(format!("[{}ms] ◆킬적립{} seed={:#x} fp={:04x} +[{}] (캠프 웨이브#{} spawn_tick={} sim_tick={})",
                    now_ms(), if is_live { "★" } else { "" }, seed, fp & 0xffff, new.join(" "), ws.wave_idx, ws.spawn_tick, tick));
            }
        }
        // ◆진단(07-24 2차): 같은 킬인덱스의 (팀|틱) 재기록 = 같은 seed에 다른 타임라인이 섞이는 순간의
        //   직격 증거(팀 오귀속 → 툴팁/장로UI 뒤죽박죽의 유력 기전). 겹치는 prefix를 대조한다.
        // ★07-24 FP조사: 재기록을 만든 현재 스레드 tid와 이 파티션 직전 write의 tid를 대조.
        //   같은 tid = 같은 스레드가 시간차로 재계산(되감기/재시뮬) / 다른 tid = 다른 스레드 동시 실행.
        //   전자면 FP 병렬 비결정 가설 성립 불가, 후자면 성립 가능 = 다음 조사 방향을 가른다.
        let cur_tid = GetCurrentThreadId();
        let overlap = v.len().min(ws.kills.len());
        for i in 0..overlap {
            if v[i] != ws.kills[i] {
                let n = KILLDIFF_LOG_N.fetch_add(1, Ordering::Relaxed);
                // ★07-24 실증: 재기록은 대부분 "같은 팀·같은 idx인데 tick만 다름"(두 인스턴스가 세르펜을
                //   다른 tick에 죽임)이었다. 구 게이트(team/idx 변화만)는 그걸 안 세 카운터가 0 = 오진단.
                //   → v[i]!=ws.kills[i](이미 참)면 무조건 tid 대조. 실측 = 전부 다른스레드(동시).
                if cur_tid == ws.last_kill_tid { KILLDIFF_SAME_TID.fetch_add(1, Ordering::Relaxed); }
                else { KILLDIFF_DIFF_TID.fetch_add(1, Ordering::Relaxed); }
                if n < 40 {
                    let f = |e: &(u64, u64, u64)| format!("{}#{}@{}", if e.0 == 0 { "B" } else { "R" }, e.2, e.1);
                    log_push(format!("[{}ms] ⚠킬재기록{} seed={:#x} fp={:04x} idx{}: 기존 {} → 신규 {} (sim_tick={} tid={} prev_tid={} {})",
                        now_ms(), if is_live { "★" } else { "" }, seed, fp & 0xffff, i, f(&ws.kills[i]), f(&v[i]), tick,
                        cur_tid, ws.last_kill_tid,
                        if cur_tid == ws.last_kill_tid { "◆같은스레드(재계산)" } else { "◆다른스레드(동시)" }));
                }
                break; // 첫 불일치만 (연쇄는 어차피 전부 밀림)
            }
        }
    }
    // 뒤처진 클론이 앞선 이력을 덮지 않게 단조증가만 반영(같은 길이면 최신값으로 갱신).
    if v.len() >= ws.kills.len() { ws.kills = v; ws.last_kill_tid = GetCurrentThreadId(); }
    // ★캐시 갱신은 "완주한 뒤"에만 — 중간에 return한 실패 경로는 캐시를 안 남겨 다음 틱에 재시도된다.
    KC_ADDR[slot].store(w as u64, Ordering::Relaxed);
    KC_SIG[slot].store(sig, Ordering::Relaxed);
}
extern "win64" fn mobatick_detour(a: u64, b: u64, c: u64, d: u64) {
    let t = MOBATICK_TRAMP.load(Ordering::Relaxed);
    if t == 0 { return; }
    unsafe { core::mem::transmute::<usize, MobaTickFn>(t)(a, b, c, d) }; // 원본 tick(데미지 처리 포함) 먼저
    // 처치 팀귀속(track_kills) + 장로 처형(execute_pass) — tick 후 순서 보장
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { track_kills(a as usize) }));
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { execute_pass(a as usize) }));
}
fn install_mobatick_hook() {
    if MOBATICK_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    unsafe {
        let base = GetModuleHandleW(core::ptr::null());
        if base == 0 { return; }
        let fn_addr = base + MOBATICK_RVA;
        for i in 0..12 {
            if *((fn_addr + i) as *const u8) != MOBATICK_PROLOGUE[i] {
                log_push(format!("[{}ms] mobatick prologue mismatch @+{}: {:#x}", now_ms(), i, *((fn_addr + i) as *const u8)));
                return;
            }
        }
        let stub = VirtualAlloc(0, 64, 0x3000, 0x40);
        if stub == 0 { return; }
        let mut s: Vec<u8> = Vec::new();
        s.extend_from_slice(&MOBATICK_PROLOGUE);
        s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&((fn_addr + 12) as u64).to_le_bytes());
        s.extend_from_slice(&[0xff, 0xe0]);
        core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
        MOBATICK_TRAMP.store(stub, Ordering::Relaxed);
        let mut patch = [0u8; 12];
        patch[0] = 0x48; patch[1] = 0xb8;
        patch[2..10].copy_from_slice(&(mobatick_detour as usize).to_le_bytes());
        patch[10] = 0xff; patch[11] = 0xe0;
        let mut old = 0u32;
        if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 { return; }
        core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
        VirtualProtect(fn_addr, 12, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
        log_push(format!("[{}ms] ☠장로처형 훅 installed fn={:#x} (RVA {:#x})", now_ms(), fn_addr, MOBATICK_RVA));
    }
}
static MOBATICK_TRAMP: AtomicUsize = AtomicUsize::new(0);
static MOBATICK_INSTALLED: AtomicBool = AtomicBool::new(false);
static EXEC_THR_PCT: AtomicU64 = AtomicU64::new(0);   // 처형 임계 %(config)
static EXEC_DURATION: AtomicU64 = AtomicU64::new(0);  // 처형 지속 tick(config, 0=무제한)
static EXEC_N: AtomicU64 = AtomicU64::new(0);         // 처형 발생 수
static EXEC_CAND_N: AtomicU64 = AtomicU64::new(0);    // 진단: mobatick 임계권 후보 관측 수
static CFG_EXECUTE: AtomicBool = AtomicBool::new(true);

// ── ☠장로 처형 v3 = 최종 데미지 어플라이어 증폭 훅 (2026-07-18 상향식 RE 확정) ──
//   v2(0x22474a0)는 오판: 그 함수는 "처치 시 영구 HP 스택" 이펙트 전용(실측 음수델타 0회).
//   +0x658 write 전수 스캔으로 확정한 진짜 경로:
//     스킬·평타 핸들러 → 딜 파이프라인 FUN_1421e2400(0x21e2400: 증폭→방어→크리→피해감소→트리거)
//       → ★FUN_141f147e0(0x1f147e0: 실드풀 흡수 → curHP 감산, 언다잉 1 유지, 킬크레딧 기록)
//   설계: A(0x1f147e0) pre-hook에서 "이 타격 후 HP <= 임계% && 공격팀=장로버프팀"이면
//   데미지 인자(p6)를 curHP+실드로 키워 curHP가 정확히 0 착지 → 게임이 정상 사망 처리.
//   World는 A 인자에 없음 → B(0x21e2400, r8=World) pre-hook이 TLS로 전달. stale TLS 방어 =
//   대상 엔티티가 그 World의 챔피언 dense 배열 원소인지 검증(불일치 시 무동작).
//   ★생사 판정 = `curHP != 0`(is_alive 0x9a2ce0) — 정확히 0 착지가 절대 조건(음수=불사 버그).
//   A ABI(9인자): rcx=대상엔티티, rdx=tick, r8=공격자정보{id@+0, disc@+8(MAX=없음/홀수=팀무효),
//     team:u32@+0x10}, r9d=타입(0물리/1마법/3실드무시/4DoT/5존), p5(2=평타), ★p6=최종데미지(u64),
//     p7=실드무시, p8=크리, p9=이벤트싱크. B ABI(12인자): rcx=PRNG, rdx=ctx, r8=World, r9=싱글턴, ...
const DMGA_RVA: usize = 0x10670a0;   // 0.5.4 (구0.5.3=0xfdbbb0) 최종 HP 감산 어플라이어 (모든 실드-경유 피해 수렴점). 프롤로그 12B 바이트 동일·크기 811→811 무변화.
                                     //   판정: L1=0.9945·L2/L3 UNIQUE·크기 0x32b 동일·프롤로그 동일(imm 극소변경만).
const DMGA_PROLOGUE: [u8; 12] = [0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53];
const DMGB_RVA: usize = 0x14eaef0;   // 0.5.4 (구0.5.3=0x12c3bb0) 딜 파이프라인 (r8=World) — TLS world 캡처 전용. 프롤로그 12B 동일.
const DMGB_PROLOGUE: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
// 실드 = Vec (2026-07-18 완전해부 RE 확정): +0x268=요소버퍼 ptr(빈 Vec=dangling 8),
//   +0x270=len, 요소 stride 0x28, 실드량=요소+0x18(u64). A가 HP보다 먼저 흡수(제자리 차감).
//   지난 "전 대상 상수 8" = 빈 Vec의 (ptr=8, len=0)을 실드값으로 오독한 것.
// 공격자 챔피언 판정용 엔티티 id = O_ENTITY_ID(0x5a8, 상단 정의). r8+0x0(공격자 id)과 대조.
static AMP_LAND_N: AtomicU64 = AtomicU64::new(0);  // 진단: 착지 검증 로그 수
static AMP_FIRE_LIVE_N: AtomicU64 = AtomicU64::new(0); // 진단: 화면 경기 처형 발화 수
static AMP_LIVE_LOG_N: AtomicU64 = AtomicU64::new(0);  // 진단: 화면 경기 발화 로그 캡
static DMGA_TRAMP: AtomicUsize = AtomicUsize::new(0);
static DMGB_TRAMP: AtomicUsize = AtomicUsize::new(0);
static DMG_INSTALLED: AtomicBool = AtomicBool::new(false);
static AMP_SEEN_N: AtomicU64 = AtomicU64::new(0);  // 진단: 챔피언 대상 데미지 히트 수
static AMP_FIRE_N: AtomicU64 = AtomicU64::new(0);  // 진단: 증폭(처형) 발화 수
static AMP_CALL_N: AtomicU64 = AtomicU64::new(0);  // 진단: A훅 호출 총수
static AMP_SAMPLE_N: AtomicU64 = AtomicU64::new(0);// 진단: 히트 샘플 로그 수
type DmgAFn = extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64, u64) -> u64;
type DmgBFn = extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) -> u64;
thread_local! {
    // B가 캡처한 World — A는 같은 스레드에서 B 내부에서 호출된다(정적 콜그래프 확증).
    //   DoT/존 등 B를 안 거치는 A 진입은 TLS가 직전 경기로 stale일 수 있음 → dense 검증이 방어.
    static TL_WORLD: core::cell::Cell<u64> = core::cell::Cell::new(0);
}

// A pre-hook 판정: 증폭 대상이면 Some(새 p6) 반환, 아니면 None(원본 그대로).
// 탈락 사유 카운터 (probe [☠처형탈락] — "발화가 너무 드묾" 원인 특정용)
//   0=비챔피언대상 1=disc무효 2=팀무효 3=TLS월드무효 4=dense불일치 5=장로창없음 6=임계초과 7=(예비) 8=공격자비챔피언
static AMP_REJ: [AtomicU64; 9] = [const { AtomicU64::new(0) }; 9];
unsafe fn exec_amp(tgt: u64, ainfo: u64, dmg: u64) -> Option<u64> {
    if !CFG_ELDER.load(Ordering::Relaxed) { return None; } // 장로 off → 처형 없음
    let thr = EXEC_THR_PCT.load(Ordering::Relaxed) as i64;
    if thr == 0 { return None; }
    AMP_CALL_N.fetch_add(1, Ordering::Relaxed);
    if dmg == 0 || dmg > (1u64 << 40) { return None; }
    let tgt = tgt as usize;
    if tgt < 0x10000 { return None; }
    if safe_read_u64(tgt + ENTITY_KIND_OFF) != Some(CHAMP_KIND) {
        AMP_REJ[0].fetch_add(1, Ordering::Relaxed); return None; } // 챔피언 대상만
    AMP_SEEN_N.fetch_add(1, Ordering::Relaxed);
    // 공격자 팀 (r8 구조체 — disc MAX=공격자없음/홀수=팀무효)
    let ai = ainfo as usize;
    if ai < 0x10000 { return None; }
    let disc = safe_read_u64(ai + 8)?;
    if disc == u64::MAX || (disc & 1) == 1 { AMP_REJ[1].fetch_add(1, Ordering::Relaxed); return None; }
    let team = safe_read_i32(ai + 0x10)? as i64;
    if !(0..=1).contains(&team) { AMP_REJ[2].fetch_add(1, Ordering::Relaxed); return None; }
    // 진단 샘플: 챔피언 대상 히트의 실 파라미터 (경로 검증용)
    let sn = AMP_SAMPLE_N.fetch_add(1, Ordering::Relaxed);
    if sn < 6 {
        log_push(format!("[{}ms] ◇히트샘플#{} dmg={} team={} tgt_hp={:?}", now_ms(), sn, dmg, team,
            safe_read_u64(tgt + O_CUR_HP)));
    }
    // World = TLS(B 캡처). stale 방어: 대상이 이 World의 엔티티 dense 원소여야 함
    let w = TL_WORLD.with(|c| c.get()) as usize;
    if w < 0x10000 { AMP_REJ[3].fetch_add(1, Ordering::Relaxed); return None; }
    let dp = safe_read_u64(w + W_CHAMP_DENSE)? as usize;
    let dl = safe_read_u64(w + W_CHAMP_DENSE + 8)? as usize;
    // ⚠len 상한 4096: +0x720 배열은 챔피언 전용이 아니라 미니언 포함 **전체 엔티티**라 실전에서
    //   수십~수백. 구 상한 64가 대부분의 히트를 기각시켜 "발화가 너무 드묾"의 원인이었다(2026-07-18).
    //   범위 검사는 O(1)이라 상한은 sanity 용도일 뿐.
    if dp < 0x10000 || dl == 0 || dl > 4096 { AMP_REJ[3].fetch_add(1, Ordering::Relaxed); return None; }
    if tgt < dp || tgt >= dp + dl * CHAMP_STRIDE || (tgt - dp) % CHAMP_STRIDE != 0 {
        AMP_REJ[4].fetch_add(1, Ordering::Relaxed); return None; }
    // 공격팀의 장로 버프 활성? (이 World의 serpen_logs[i>=elder_after] + duration 창)
    let ea = ELDER_AFTER.load(Ordering::Relaxed) as u64;
    let klen = safe_read_u64(w + KILLS_LEN_OFF).unwrap_or(0);
    if klen == 0 || klen > 128 || (ea > 0 && klen <= ea) { return None; }
    let kptr = safe_read_u64(w + KILLS_PTR_OFF)?;
    if kptr < 0x10000 { return None; }
    let cur_tick = safe_read_u64(w + SIM_TICK_OFF).unwrap_or(0);
    let dur = EXEC_DURATION.load(Ordering::Relaxed);
    // ★성능(2026-07-19): 이 판정은 **피해 타격마다** 돌아서, 한타 중엔 처치이력 순회(최대 128×2 VEH
    //   read)가 초당 수만 번 반복됐다. 결과가 바뀌는 계기는 (처치 발생 | 지속시간 만료) 둘뿐이므로
    //   (klen, tick/30초버킷)을 키로 팀별 활성여부를 캐시한다. 버프 길이가 분 단위라 1초 해상도면 충분.
    let bucket = cur_tick / 30;
    let eslot = ((w >> 4) & (KC_SLOTS - 1)) as usize;
    let ekey = klen ^ (bucket << 20) ^ 0xe1de3;
    let active = if EA_KEY[eslot].load(Ordering::Relaxed) == ekey
        && EA_ADDR[eslot].load(Ordering::Relaxed) == w as u64 {
        EA_MASK[eslot].load(Ordering::Relaxed) & (1 << team) != 0
    } else {
        let mut mask = 0u64;
        for i in (ea as usize)..(klen as usize) {
            let e = kptr as usize + i * 16;
            let Some(t) = safe_read_u64(e) else { continue };
            if t > 1 { continue; }
            let kt = safe_read_u64(e + 8).unwrap_or(0);
            if dur == 0 || cur_tick <= kt.saturating_add(dur) { mask |= 1 << t; }
        }
        EA_ADDR[eslot].store(w as u64, Ordering::Relaxed);
        EA_MASK[eslot].store(mask, Ordering::Relaxed);
        EA_KEY[eslot].store(ekey, Ordering::Relaxed);
        mask & (1 << team) != 0
    };
    if !active { AMP_REJ[5].fetch_add(1, Ordering::Relaxed); return None; }
    // ★공격자 = 장로버프팀의 **챔피언**이어야 함 (2026-07-18 유저: 미니언/포탑 타격은 처형 금지).
    //   A 함수엔 공격자 kind 게이트가 없다(대상 기준). r8+0x0 = 공격자 id(=엔티티+0x5a8).
    //   그 팀 챔피언 목록의 id와 대조 → 미니언/포탑/중립은 챔피언 목록에 없어 자동 배제.
    let atk_id = safe_read_u64(ai + 0)?;
    let mut champs: Vec<usize> = Vec::new();
    team_champions(w, team as u64, &mut champs);
    if !champs.iter().any(|&c| safe_read_u64(c + O_ENTITY_ID) == Some(atk_id)) {
        AMP_REJ[8].fetch_add(1, Ordering::Relaxed); return None; // 공격자가 챔피언 아님
    }
    // 언다잉(O_UNDYING +0x470!=0)도 증폭 수행 — A 감산이 min(cur-1,dmg)로 1을 남김(완전해부 RE).
    // 임계 판정 = **현재 HP**(pre-hit). "저체력 적을 때리면 처형" — 실드 계산에 의존 안 함(견고).
    let cur = safe_read_u64(tgt + O_CUR_HP).unwrap_or(0) as i64;
    if cur <= 0 { return None; }
    let mx = safe_read_u64(tgt + O_EXEC_MAXHP).unwrap_or(0) as i64;
    if mx <= 0 { return None; }
    if cur.saturating_mul(100) > thr.saturating_mul(mx) {
        AMP_REJ[6].fetch_add(1, Ordering::Relaxed); return None; } // 아직 임계 초과 = 처형 안 함
    // ★증폭: p6 = cur + maxHP → A가 실드를 먼저 흡수해도(실드 ≤ maxHP면) curHP가 0 착지.
    //   RE 확정: A 감산은 saturating_sub이라 초과분도 정확히 0에 멈춤(음수 불사 버그 없음).
    //   실드 Vec 정밀 합산은 실측에서 0이 나와 신뢰 불가 → maxHP 여유분으로 확실히 관통(2026-07-18).
    let amp = (cur + mx) as u64;
    let n = AMP_FIRE_N.fetch_add(1, Ordering::Relaxed);
    EXEC_N.fetch_add(1, Ordering::Relaxed);
    // 화면 경기 발화는 별도 집계 + 별도 로그 캡 ("화면에선 처형 안 보임" 검증용)
    let is_live = safe_read_u64(w + SEED_OFF) == Some(RENDER_SEED.load(Ordering::Relaxed));
    if is_live { AMP_FIRE_LIVE_N.fetch_add(1, Ordering::Relaxed); }
    let ln = if is_live { AMP_LIVE_LOG_N.fetch_add(1, Ordering::Relaxed) } else { u64::MAX };
    if n < 12 || ln < 10 {
        log_push(format!("[{}ms] ☠처형(증폭){} team={} 대상={:#x} hp={}/{} dmg {}→{} (임계 {}%, tick={})",
            now_ms(), if is_live { "★화면" } else { "" }, team, tgt, cur, mx, dmg, amp, thr, cur_tick));
    }
    Some(amp)
}
extern "win64" fn dmga_detour(rcx: u64, rdx: u64, r8: u64, r9: u64, p5: u64, p6: u64, p7: u64, p8: u64, p9: u64) -> u64 {
    let t = DMGA_TRAMP.load(Ordering::Relaxed);
    if t == 0 { return 0; }
    let mut dmg = p6;
    if CFG_EXECUTE.load(Ordering::Relaxed) {
        if let Ok(Some(nd)) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { exec_amp(rcx, r8, p6) })) {
            dmg = nd;
        }
    }
    let r = unsafe { core::mem::transmute::<usize, DmgAFn>(t)(rcx, rdx, r8, r9, p5, dmg, p7, p8, p9) };
    // 착지 검증: 증폭했으면 감산 직후 curHP가 정확히 0이어야 사망 성립 (음수/양수 잔존 = 버그)
    if dmg != p6 {
        let n = AMP_LAND_N.fetch_add(1, Ordering::Relaxed);
        if n < 6 {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let hp = unsafe { safe_read_u64(rcx as usize + O_CUR_HP) };
                log_push(format!("[{}ms] ◇착지검증#{} 감산후 curHP={:?} (0=정상사망)", now_ms(), n, hp));
            }));
        }
    }
    r
}
extern "win64" fn dmgb_detour(a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64, a8: u64, a9: u64, a10: u64, a11: u64, a12: u64) -> u64 {
    let t = DMGB_TRAMP.load(Ordering::Relaxed);
    if t == 0 { return 0; }
    TL_WORLD.with(|c| c.set(a3)); // r8 = World (딜 파이프라인 진입마다 갱신, 패닉 여지 없음)
    unsafe { core::mem::transmute::<usize, DmgBFn>(t)(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12) }
}
// 공용 12B 트램폴린 설치 (프롤로그 검증 → 스텁 → movabs jmp 패치)

// ══ 리소스 조회 프로브 (2026-07-19) — hot path 이식 가능 여부 판정용 ══
//   목표 seam = FUN_1413c0e90(0x13c0e90, 엔티티이름→에셋 베이스키). 여기서 반환 String을 제자리
//   치환하면 게임이 {키}#sheet·{키}#anim을 조립해 우리 시트+우리 프레임표를 쓰고, dst/flip/중심정렬은
//   게임 원래 로직이 처리한다(= UV 역산·보정 코드 전부 소멸, 발화 50~100배 감소).
//   ⚠단 **키가 에셋 맵에 없으면 조용한 무시가 아니라 즉시 패닉/크래시**(디스패처 경로와 정반대).
//   우리 모드의 #anim(프레임표)이 등록되는지가 미확인이라, 치환 전에 조회만 해보고 판정한다.
//   ABI: FUN_1413c4e90(rcx=sret, rdx=뷰엔티티, r8=에셋스토어, ...) → r8만 캡처.
//        FUN_140eb0880(store, key_ptr, key_len) -> *anim 리소스 (없으면 null)
//        FUN_140eb0420(store, key_ptr, key_len) -> *텍스처   (없으면 null)
// ⬜0.5.2 미확정(값은 구0.5.1 그대로 = stale). 이 둘은 제네릭 리소스 게터의 모노모픽 copy라
//   0.5.2에서 바이트동일 후보가 26개 나와 정적으로 가릴 수 없었다(스켈레톤·마스크시그 모두 MULTI).
//   ★현재 무해: 호출부는 anim_probe 게이트 안에만 있고 cfg `anim_probe = 0`(shadow-call이라 기본 OFF)이며,
//     이 프로브는 2026-07-19에 판정을 끝내고 producer_seam 채택으로 역할이 끝난 1회용 개발 코드다.
//   ⚠되살리려면 먼저 ghidra-re로 재핀할 것 — 지금 값으로 shadow-call 하면 엉뚱한 함수를 호출한다.
// ══ 생산자 seam: 에셋 베이스 키 리졸버 후킹 (2026-07-19 RE + 런타임 확인 완료) ══
//   FUN_1413c0e90(rcx=out:String, rdx=엔티티) — 엔티티 이름으로 "asset/base/.../serpen" 같은
//   베이스 키를 만들어 반환한다. 게임은 이걸로 {키}#sheet·{키}#anim을 조립해 텍스처와 **프레임표**를
//   가져온다. 여기서 반환 String만 우리 것으로 바꾸면 시트·프레임 배치·크기·flip·중심정렬이 전부
//   게임 원래 로직으로 처리된다 ⇒ 디스패처에서 하던 UV 역산·dst 보정·슬롯 매핑이 통째로 불필요.
//   비용도 커맨드당(수천/프레임) → 엔티티당(수십/프레임)으로 떨어지고, 디스패처를 안 건드리니
//   map_skin_selector와의 체인 후킹도 불필요해진다.
//   ★제자리 치환만 허용: 호출자가 (ptr,cap)으로 String을 drop하므로 ptr/cap은 절대 손대지 않고
//     내용과 len만 바꾼다 → 우리 키가 원본(43자)보다 짧아야 한다(단축키 31자).
//   ⚠키가 에셋 맵에 없으면 게임이 즉시 패닉(디스패처와 정반대) → 등록 확인된 키만 쓴다(프로브 완료).
// ★★0.5.2 마이그 주의 — 이 함수는 **프롤로그 바이트가 바뀐 유일한 훅**이다.
//   0.5.1: push rbp/rsi/rdi; sub rsp,0x70; lea rbp,[rsp+0x70]  = 55 56 57 48 83 EC 70 48 8D 6C 24 70
//   0.5.2: 동일하되 프레임이 0x70→0x60  = 55 56 57 48 83 EC 60 48 8D 6C 24 60
//   ⇒ RVA만 갈고 PROLOGUE를 안 고치면 install_tramp12 의 프롤로그 검증이 실패해 **훅이 조용히 미설치**
//     되고(로그 "실패(프롤로그 mismatch)") 스프라이트 교체가 통째로 죽는다. 둘을 항상 같이 고칠 것.
//   변경 성격(디스어셈 대조): 명령어열 L2=0.9884, 차이는 딱 한 군데 — 패닉/포맷 인자 셋업 4명령어
//   (`mov [rsp+0x30],0` / `[rsp+0x28],1` / `[rsp+0x20],8` / `lea r9,[rip+..]`)가 삭제됨. 크기 0x33c→0x31a.
//   **ABI·시맨틱 불변**: rcx=out String(→rsi), rdx=엔티티, 반환 rax=rsi=out. 진입/복귀열 동일.
//   바닐라 베이스키 "asset/base/aseprite_resources/ingame/serpen" = 0.5.2에서도 **43자·3회 등장**
//   ⇒ 제자리 치환(키 ≤43자) 제약 그대로 유효.
const KEYRES_RVA: usize = 0x218be90; // 0.5.4 (구0.5.3=0x1b0aba0). 프롤로그 12B 바이트 완전동일(아래 KEYRES_PROLOGUE 무수정)·크기 776=776·aseprite 문자열 동일.
const KEYRES_PROLOGUE: [u8; 12] = [0x55, 0x56, 0x57, 0x48, 0x83, 0xEC, 0x60, 0x48, 0x8D, 0x6C, 0x24, 0x60];
const VAN_BASE_KEY: &[u8] = b"asset/base/aseprite_resources/ingame/serpen";
static KEYRES_TRAMP: AtomicUsize = AtomicUsize::new(0);
static KEYRES_INSTALLED: AtomicBool = AtomicBool::new(false);
static CFG_PRODUCER_SEAM: AtomicBool = AtomicBool::new(true); // 색 교체 = 생산자 seam(키 리졸버) — 항상 on(유일 경로)
static KEYRES_SWAP_N: AtomicU64 = AtomicU64::new(0);
type KeyResFn = extern "win64" fn(usize, usize) -> usize;

extern "win64" fn keyres_detour(out: usize, ent: usize) -> usize {
    let t = KEYRES_TRAMP.load(Ordering::Relaxed);
    if t == 0 { return 0; }
    let r = unsafe { core::mem::transmute::<usize, KeyResFn>(t)(out, ent) };
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { keyres_swap(out, ent) }));
    r
}

// ★엔티티별 속성 고정 (2026-07-19) — 죽는 모션 대책의 정공법.
//   처치 순간 화면 카운터가 +1되어 CURRENT_ATTR은 곧바로 **다음 웨이브**를 가리킨다. 그런데 죽는
//   애니는 그 뒤로도 재생되므로, 매 프레임 CURRENT_ATTR을 그대로 쓰면 죽는 모션만 다음 세르펜
//   모습이 된다. 디스패처 경로에선 "죽는 프레임이면 직전 속성" 같은 프레임 역산으로 우회했지만,
//   리졸버는 **엔티티 포인터**를 받으므로 그 세르펜이 처음 보일 때의 속성을 못 박아두면 된다
//   (= 그 개체가 사라질 때까지 일관). 프레임 추측이 아예 필요 없다.
//   직접 매핑 캐시(엔티티 주소 → 속성). 포인터 재사용 대비: 경기(seed)가 바뀌면 통째로 무효화.
const EPIN_SLOTS: usize = 16;
static EPIN_ENT: [AtomicU64; EPIN_SLOTS] = [const { AtomicU64::new(0) }; EPIN_SLOTS];
static EPIN_ATTR: [AtomicI32; EPIN_SLOTS] = [const { AtomicI32::new(-2) }; EPIN_SLOTS];
static EPIN_SEED: AtomicU64 = AtomicU64::new(0);

fn entity_pinned_attr(ent: usize, cur: i32) -> i32 {
    let seed = RENDER_SEED.load(Ordering::Relaxed);
    if EPIN_SEED.swap(seed, Ordering::Relaxed) != seed {          // 경기 전환 → 전부 무효화
        for i in 0..EPIN_SLOTS {
            EPIN_ENT[i].store(0, Ordering::Relaxed);
            EPIN_ATTR[i].store(-2, Ordering::Relaxed);
        }
    }
    let key = ent as u64;
    let slot = ((key >> 4) as usize) & (EPIN_SLOTS - 1);
    if EPIN_ENT[slot].load(Ordering::Relaxed) == key {
        let a = EPIN_ATTR[slot].load(Ordering::Relaxed);
        if a != -2 { return a; }                                   // 이 개체에 이미 못 박힌 속성
    }
    EPIN_ENT[slot].store(key, Ordering::Relaxed);                  // 처음 본 개체 → 현재 속성으로 고정
    EPIN_ATTR[slot].store(cur, Ordering::Relaxed);
    cur
}

unsafe fn keyres_swap(out: usize, ent: usize) {
    if !CFG_PRODUCER_SEAM.load(Ordering::Relaxed) { return; }
    if out < 0x10000 || out >= (1usize << 48) { return; }
    let cap = *(out as *const usize);
    let ptr = *((out + 8) as *const usize);
    let len = *((out + 0x10) as *const usize);
    // 세르펜 베이스 키일 때만. 길이가 다르면 즉시 탈락(대다수 엔티티가 여기서 끝)
    if len != VAN_BASE_KEY.len() || ptr < 0x10000 || cap < len { return; }
    if core::slice::from_raw_parts(ptr as *const u8, len) != VAN_BASE_KEY { return; }
    let cur = CURRENT_ATTR.load(Ordering::Relaxed);
    if cur == -2 { return; }                    // 바닐라 유지
    let idx = entity_pinned_attr(ent, cur);     // 그 개체가 처음 보일 때의 속성으로 고정
    if idx == -2 { return; }
    // ★2026-08-02: 렌더 스레드가 매 프레임 POOL/ELDER Mutex를 잡던 자리 — 무락 스냅샷으로 교체
    //   (sim 워커 8개와 같은 락을 두고 경합해 렌더 히치까지 유발하던 경로).
    let Some(sn) = attrs() else { return };
    let attr: &Attr = if idx == ELDER_IDX {
        match sn.elder.as_ref() { Some(a) => a, None => return }
    } else {
        match sn.pool.get(idx as usize) { Some(a) => a, None => return }
    };
    let k = attr.short_key.as_bytes();
    if k.is_empty() || k.len() > cap { return; }
    core::ptr::copy_nonoverlapping(k.as_ptr(), ptr as *mut u8, k.len());
    *((out + 0x10) as *mut usize) = k.len();     // ptr/cap은 건드리지 않는다
    let n = KEYRES_SWAP_N.fetch_add(1, Ordering::Relaxed);
    if n < 4 {
        log_push(format!("[{}ms] ★생산자seam 키교체#{} → '{}' (cap={} 원본{}자)",
            now_ms(), n, attr.short_key, cap, len));
    }
}

unsafe fn install_tramp12(rva: usize, prologue: &[u8; 12], detour: usize, tramp: &AtomicUsize, name: &str) {
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { return; }
    let fn_addr = base + rva;
    for i in 0..12 {
        if *((fn_addr + i) as *const u8) != prologue[i] {
            log_push(format!("[{}ms] {} prologue mismatch @+{}: {:#x} — 미설치", now_ms(), name, i, *((fn_addr + i) as *const u8)));
            return;
        }
    }
    let stub = VirtualAlloc(0, 64, 0x3000, 0x40);
    if stub == 0 { return; }
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(prologue);
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&((fn_addr + 12) as u64).to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    tramp.store(stub, Ordering::Relaxed);
    let mut patch = [0u8; 12];
    patch[0] = 0x48; patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&detour.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    let mut old = 0u32;
    if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 { return; }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    log_push(format!("[{}ms] {} installed fn={:#x} (RVA {:#x})", now_ms(), name, fn_addr, rva));
}
fn install_dmg_hook() {
    if DMG_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    unsafe {
        install_tramp12(DMGB_RVA, &DMGB_PROLOGUE, dmgb_detour as usize, &DMGB_TRAMP, "☠파이프라인훅(B)");
        install_tramp12(DMGA_RVA, &DMGA_PROLOGUE, dmga_detour as usize, &DMGA_TRAMP, "☠증폭훅(A)");
    }
}
// splitmix64: 결정론 해시 (seed+순번 → 속성 선택용)
fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}
// 가중 선택: 풀에서 hash로 인덱스 (weight 비례)
fn weighted_pick(pool: &[Attr], h: u64) -> i32 {
    let total: u64 = pool.iter().map(|a| a.weight.max(1) as u64).sum();
    if total == 0 { return 0; }
    let mut r = h % total;
    for (i, a) in pool.iter().enumerate() {
        let w = a.weight.max(1) as u64;
        if r < w { return i as i32; }
        r -= w;
    }
    (pool.len() - 1) as i32
}

// 세르펜 detour 본체: 엔티티 획득 → 리스폰 감지 → 속성 배정(결정론) → +0x108 템플릿 덮기.
unsafe fn serpen_apply_attr(rcx: u64, rdx: u64, a5: u64) {
    let Some(acc) = safe_read_u64(rdx as usize + O_ENTITY_ACCESSOR) else { return; };
    if acc < 0x10000 { return; }
    let resolver: extern "win64" fn(u64, u64) -> u64 = core::mem::transmute(acc as usize);
    let ent = resolver(rcx, a5) as usize;
    if ent < 0x10000 { return; }
    if safe_read_i32(ent + ENTITY_KIND_OFF) != Some(SERPEN_KIND) { return; }
    // 템플릿 이름이 serpen_permanent_buff(len 21)인지 = 안전 게이트
    if safe_read_i32(ent + O_SERPEN_TEMPLATE) != Some(21) { return; }

    let elder_after = ELDER_AFTER.load(Ordering::Relaxed);
    // ★2026-08-02: 여기서 POOL/ELDER Mutex 2개를 잡고 **함수 끝까지 들고 있었다**(가드 스코프=함수 전체).
    //   8 sim 워커 + 렌더 스레드가 매 틱 같은 락을 두고 경합 → 락 슬로우패스 park 가 이 모드 최대 비용원.
    //   ⇒ 무락 스냅샷(&'static)으로 교체. attr 참조가 'static 이라 가드 수명 문제도 사라진다.
    let Some(sn) = attrs() else { GATE_N[0].fetch_add(1, Ordering::Relaxed); return; };
    let (pool, elder) = (&sn.pool, &sn.elder);
    // ⚠구: `pool.is_empty()`만 보고 return → **원소 cfg가 없거나 비면 장로 버프까지 같이 죽었다**
    //   (장로만 쓰는 사용자에게 "스택은 쌓이는데 능력치가 안 붙는" 증상으로 나타남).
    //   둘 다 없을 때만 바닐라 유지한다.
    if pool.is_empty() && elder.is_none() {
        GATE_N[0].fetch_add(1, Ordering::Relaxed);
        return;
    }
    let seed = safe_read_u64(rcx as usize + SEED_OFF).unwrap_or(0);
    // ★(seed, fp) 파티션 (07-24): 같은 seed를 쓰는 다른 세트/타임라인과 상태 분리. fp 미확정이면
    //   이 틱은 배정 보류(세르펜 등장 시점엔 드래프트 완료 = 사실상 항상 확정).
    let Some(fp) = (unsafe { fp_for_world(rcx as usize) }) else { GATE_N[1].fetch_add(1, Ordering::Relaxed); return; };
    let mut wg = WORLDS.lock().unwrap_or_else(|e| e.into_inner());
    let worlds = wg.get_or_insert_with(HashMap::new);
    let ws = worlds.entry((seed, fp)).or_default(); // 키 = (경기 시드, 세트 지문)
    ws.rcx = rcx as u64;
    ws.last_ms = now_ms();
    ws.hits += 1;
    ws.sim_tick = safe_read_u64(rcx as usize + SIM_TICK_OFF).unwrap_or(0); // 이 sim 계산이 일어난 프레임
    // ⚠★세르펜 처치 이력 = **track_kills(mobatick 카운터 델타)가 유일 소스**(2026-07-18).
    //   과거 여기서 serpen_logs를 직접 읽어 ws.kills를 clear+rebuild했으나, 그 e+0(team) 직독이
    //   grow 재할당/뒤로감기 시 stale·team=0 오염 + track_kills와 충돌해 웨이브/버프가 흔들렸다
    //   ("장로 살아있는데 버프 뜸"·스폰tick churn 16023↔20386의 근원). → 직접 읽기 전면 제거.

    // ★★웨이브 = 게임 캠프 필드에서 직접 (2026-07-17 RE). heuristic 전면 폐기:
    //   구: wall-clock 500ms 갭으로 전멸 추정 + a5 추적 + spawn_count 자체 집계 + tick 순서 재현
    //   신: respawn_count(웨이브 인덱스) / next_respawn_tick(이 웨이브 스폰 tick, 웨이브 내내 불변)
    //   → 스폰 tick이 정확해지고(구: 감지 지연으로 수천 틱 늦음), 재시뮬해도 같은 값이 재생산된다.
    let wave_idx = safe_read_u64(rcx as usize + CAMP_WAVE_IDX).unwrap_or(u64::MAX);
    let spawn_tick = safe_read_u64(rcx as usize + CAMP_SPAWN_TICK).unwrap_or(0);
    if wave_idx == u64::MAX { GATE_N[1].fetch_add(1, Ordering::Relaxed); return; } // 캠프 필드 무효
    ws.wave_idx = wave_idx;
    ws.spawn_tick = spawn_tick;
    // 속성 = f(seed, spawn_tick). 장로 = respawn_count가 elder_after 이상(0-based → N번째부터 장로).
    ws.current = match ws.waves.get(&wave_idx) {
        Some(&(_, a)) => a, // 이미 정한 웨이브(재시뮬 포함) → 그 색 그대로
        None => {
            // ★온오프 스위치(serpen.cfg): 장로/원소 독립. 둘다 off면 이 웨이브=바닐라(VANILLA_ATTR=-2).
            let use_elder = CFG_ELDER.load(Ordering::Relaxed);
            let use_ele = CFG_ELEMENTAL.load(Ordering::Relaxed);
            // elder_after는 내부 0-based(config 1-based−1). wave_idx>=elder_after면 장로. 끄기=use_elder(elder_enabled).
            let a = if use_elder && wave_idx >= elder_after as u64 && elder.is_some() {
                ELDER_IDX
            } else if use_ele && !pool.is_empty() {
                weighted_pick(pool, splitmix64(seed ^ spawn_tick.wrapping_mul(0x9E3779B97F4A7C15)))
            } else {
                VANILLA_ATTR // 원소 off & (장로 off 또는 비장로 웨이브) → 색/버프 없음(바닐라)
            };
            ws.waves.insert(wave_idx, (spawn_tick, a));
            let picked = if a == ELDER_IDX {
                elder.as_ref().map(|x| x.display_name.clone()).unwrap_or_default()
            } else if a == VANILLA_ATTR { "바닐라".to_string() }
            else { pool.get(a as usize).map(|x| x.display_name.clone()).unwrap_or_default() };
            let c = TEMPLATE_WRITES.fetch_add(1, Ordering::Relaxed);
            // 07-24 2차: 배경경기 전수(80캡)가 링버퍼를 삼킴 → 화면경기 전수 + 배경 초반 20만
            if seed == LIVE_SEED.load(Ordering::Relaxed) || c < 20 {
                log_push(format!("[{}ms] 세르펜 웨이브 seed={:#x} fp={:04x} 웨이브#{} spawn_tick={} a5={:#x} → '{}'",
                    now_ms(), seed, fp & 0xffff, wave_idx, spawn_tick, a5, picked));
            }
            // ◆진단(07-24 2차): 타임라인 모순 즉석 검출 — 웨이브N 스폰은 킬N-1 + 7200이어야 한다
            //   (1차 실측: 배경경기 전부 +7200~7201). 어긋나면 이 배정이 "다른 타임라인"의 캠프값으로
            //   이뤄진 것 = 화면경기 0x2fea… #2@24768(킬2@23952보다 뒤) 모순의 실시간 포착.
            if wave_idx >= 1 {
                if let Some(&(_, ktick, _)) = ws.kills.iter().find(|(_, _, ki)| *ki == wave_idx - 1) {
                    let expect = ktick.wrapping_add(7200);
                    if spawn_tick.abs_diff(expect) > 90 {
                        let n = TL_ANOMALY_N.fetch_add(1, Ordering::Relaxed);
                        if n < 40 {
                            log_push(format!("[{}ms] ⚠타임라인모순 seed={:#x} 웨이브#{} spawn_tick={} 기대={}(킬#{}@{}+7200) 차이={}",
                                now_ms(), seed, wave_idx, spawn_tick, expect, wave_idx - 1, ktick,
                                spawn_tick as i64 - expect as i64));
                        }
                    }
                }
            }
            a
        }
    };
    // ◆진단(07-24 제보): 화면 경기(sim측)의 웨이브/속성 전이 = "실제 부여될 버프" 스트림 실측.
    //   dedup = 웨이브 비트마스크(진행도 다른 클론 핑퐁 방지 — Option 페어는 초당 수백 줄 실사고).
    if CFG_PROBE_LOG.load(Ordering::Relaxed) && seed == LIVE_SEED.load(Ordering::Relaxed) {
        let bit = 1u64 << wave_idx.min(63);
        if ws.logged_mask & bit == 0 {
            ws.logged_mask |= bit;
            log_push(format!("[{}ms] ◆sim웨이브전이 seed={:#x} fp={:04x} 웨이브#{} spawn_tick={} 속성={} sim_tick={}",
                now_ms(), seed, fp & 0xffff, wave_idx, spawn_tick, ws.current, ws.sim_tick));
        }
    }

    // 배정된 속성으로 템플릿 덮기 (매 틱). VANILLA_ATTR/무효면 스탯 안 씀(바닐라 유지).
    let cur_attr_idx = ws.current;
    // ★2026-08-02: 아래 write 검증 로그의 "이번에 찍을 차례인가" 판정과 마스크 갱신만 락 안에서 끝낸다
    //   (실제 로그 생성은 stat_read 32회 + format! 이라 락 밖으로 뺀다 = 아래 rb_log).
    let rb_log = ws.rb_mask & (1u64 << wave_idx.min(63)) == 0 && seed == LIVE_SEED.load(Ordering::Relaxed);
    if rb_log { ws.rb_mask |= 1u64 << wave_idx.min(63); }
    // ★★WORLDS 가드 해제 지점 — 여기서부터는 공유 상태를 안 만진다(템플릿 write는 이 경기 엔티티 전용).
    //   구조: 가드가 함수 끝까지 살아 있어서 32회 write + 검증 로그까지 전역 락 안에서 돌았다.
    drop(wg);
    let attr: &Attr = if cur_attr_idx == ELDER_IDX {
        match elder.as_ref() { Some(a) => a, None => return }
    } else if cur_attr_idx == VANILLA_ATTR {
        GATE_N[2].fetch_add(1, Ordering::Relaxed);
        return; // 바닐라 웨이브 — 색/버프 없음
    } else {
        match pool.get(cur_attr_idx as usize) { Some(a) => a, None => return }
    };
    // ★스탯블록 물리 배치 (2026-07-18 RE 확정, ghidra 합산함수 FUN_141f097b0/09500):
    //   블록 = 이펙트엔트리+0x58(=ent+0x108=ts). idx0~14 = i32(연속 4B). entry+0x94 = 4B 패딩.
    //   idx15~31 = **i64(8B)**, entry+0x98부터. (범용 i32[32] 가정이 idx15+를 오정렬시켜, 특히
    //   crit_chance(idx29)를 range(idx21) i64의 상위 4B(entry+0xcc)에 써 사거리 폭증 = 실측 버그.)
    //   확증 앵커: range(idx21)=entry+0xc8=block+0x70, crit(idx29)=entry+0x108=block+0xb0.
    let ts = ent + TMPL_STAT_OFF;
    if !CFG_STAT_WRITE.load(Ordering::Relaxed) { return; } // 대조실험: sim 개입 억제(배정·기록은 위에서 완료)
    // ★2026-08-02: **값이 다를 때만 쓴다**(read-compare-skip). 최종 메모리 상태는 구현과 동일하고,
    //   게임이 템플릿을 되돌리면 다음 틱에 차이가 감지돼 다시 쓰이므로 "매 틱 무조건"의 의도도 유지된다.
    //   정상 상태(이미 우리 값)에서 write 32회가 0회가 된다.
    for i in 0..NUM_STATS {
        let v = attr.stats[i];
        let a = ts + stat_off(i);
        if stat_is_i32(i) {                                                     // idx0~14, 27~29
            if safe_read_i32(a) != Some(v) { let _ = safe_write_i32(a, v); }
        } else {                                                                // idx15~26, 30~31
            let w = v as i64 as u64;
            if safe_read_u64(a) != Some(w) { let _ = safe_write_u64(a, w); }
        }
        // ⚠0xa0(cc_immune)·0xc0(undying)·0xc1(ignore_wall)은 절대 건드리지 않는다 — 불리언이라
        //   숫자를 흘려넣으면 CC면역·불사가 몰래 켜진다(구 공식의 실제 부작용).
    }
    // ★write 검증(2026-07-19): 쓴 값이 템플릿에 실제로 남는지 되읽는다. 게임은 세르펜 사망 시
    //   이 템플릿(ent+0xb0, 0x120B)을 통째 memcpy해 처치팀 전원에게 뿌리므로(RE 확정),
    //   여기서 우리 값이 보이면 적용 경로는 보장된다. 웨이브가 바뀔 때만 1회 기록.
    //   ⚠구 게이트 = 전역 RB_LAST_WAVE 하나 → 배경경기 30~40개가 교차 진입하면 swap이 매 호출
    //   달라져 매 틱 로그 = 링버퍼 즉시 만석(07-24 발견, 진단 로그 전멸의 원인) → per-경기 필드로 교체.
    //   07-24 2차: 배경경기 write검증(경기당 웨이브수×3줄)도 버퍼를 삼킴 → 화면경기만 + 웨이브 비트마스크.
    //   ★2026-08-02: 판정·마스크 갱신은 위(락 안)에서 끝냈고 여기선 rb_log 만 본다 = 락 밖 실행.
    if rb_log {
        let nz: Vec<String> = (0..NUM_STATS)
            .filter_map(|i| stat_read(ts, i).filter(|v| *v != 0).map(|v| format!("{}={}", STAT_KEYS[i].0, v)))
            .collect();
        let want: Vec<String> = (0..NUM_STATS)
            .filter(|&i| attr.stats[i] != 0)
            .map(|i| format!("{}={}", STAT_KEYS[i].0, attr.stats[i])).collect();
        log_push(format!("[{}ms] ★버프write검증 웨이브#{} '{}'\n     의도: {}\n     실제: {}{}",
            now_ms(), wave_idx, attr.display_name, want.join(", "), nz.join(", "),
            if nz == want { "  ← 일치" } else { "  ← ★불일치" }));
    }

    // ★관전 경기 판별: 세르펜 detour rcx(=provider) == LIVE_PROVIDER(스폰훅이 캡처한 관전경기 provider)
    //   → 관전 세르펜 속성을 전역화 → skia 키 교체가 사용. 배경 리그 경기는 provider 불일치 → 자연 배제.
    //   ★관전/화면 경기 판별 (item_tactics 검증 기법):
    //   ① provider 일치(LIVE_PROVIDER 캡처된 경우) 또는
    //   ② tid 일치(스폰 클로저가 캡처한 화면 sim 스레드 == 현재 세르펜 detour 스레드).
    //   세르펜 detour=sim 틱이라 스폰 클로저와 동일 sim 스레드 → tid 안정. 배경(rayon 워커)은
    //   둘 다 불일치 → CURRENT_ATTR 오염 없음(=스폰 타이밍 아닌데 색 깜빡이는 현상 방지).
    //   ★게이트 = seed 일치. capture_live_from_db(메인스레드)가 db 스캔으로 "화면 경기"를 확정하고
    //   그 seed를 LIVE_SEED에 싣는다. ⚠주소(rcx==lp) 비교는 실패한다 — detour rcx와 Game+0x1dc0은
    //   같은 경기라도 서로 다른 객체(실측: 매칭슬롯 1개인데 rcx 불일치). seed만이 경기 고유 키.
    //   폐기된 것들: RENDER_TID(렌더 tid≠sim tid), cap_spawn provider/tid(★LIVE 0건),
    //   runner_ctor(rctor_n=0 미발화), fallback(모든 배경경기 통과 → 깜빡임).
    //   미확보 시엔 갱신 안 함 → CURRENT_ATTR=-2 → 교체 없이 원본(깜빡임보다 안전).
    // ⚠여기서 CURRENT_ATTR(화면 색)을 정하면 안 된다 — sim은 이미 앞서 계산 중이고 화면은 과거
    //   프레임을 재생하므로 색이 어긋난다(실측: 뒤로감기 시 불일치). 화면 색은 post_update가
    //   played_tick으로 waves 타임라인을 조회해 결정한다(resolve_color_from_played).
    // detour 로그 총량 제한(전역 카운터) — SKIA key/SWAP·스폰 로그가 링버퍼서 밀리지 않게. cur_tid vs rt 대조 포함.
    // ★2026-08-02: 구현은 **매 호출 fetch_add**(8스레드가 같은 캐시라인을 튕기는 RMW)였다. 상한 12줄을
    //   넘긴 뒤에는 읽기만으로 걸러 RMW 자체를 없앤다(로그 개수 의미는 동일).
    if LOG_N.load(Ordering::Relaxed) < 12 {
        let ln = LOG_N.fetch_add(1, Ordering::Relaxed);
        if ln < 12 { // 07-24 2차: 60줄이 버퍼 낭비 → 12
            let lp = LIVE_PROVIDER.load(Ordering::Relaxed);
            let lt = LIVE_TID.load(Ordering::Relaxed);
            let ls = LIVE_SEED.load(Ordering::Relaxed);
            let cur_tid = unsafe { GetCurrentThreadId() } as u64;
            let is_live = ls != 0 && seed == ls;
            log_push(format!("[{}ms] serpen rcx={:#x} cur_tid={} lp={:#x} lt={} is_live={} spawn_n={} → CUR={}",
                now_ms(), rcx, cur_tid, lp, lt, is_live, SPAWN_CAP_N.load(Ordering::Relaxed),
                CURRENT_ATTR.load(Ordering::Relaxed)));
        }
    }

    // 스프라이트 이름 교체: 세르펜 엔티티+0x250 (ptr,len)을 "serpen_<attr>_monster"로 → 속성별 이미지.
    //   name_buf는 POOL/ELDER의 Attr에 상주(프로그램 수명) → ptr 안정. 매 틱 무조건(게임 리셋 대비).
    //   ★2026-08-02: 여기도 read-compare-skip — 이미 우리 ptr/len이면 write 안 한다(게임이 되돌리면 재기록).
    if CFG_NAME_SWAP.load(Ordering::Relaxed) && !attr.name_buf.is_empty() {
        let (p, l) = (attr.name_buf.as_ptr() as u64, attr.name_buf.len() as u64);
        if safe_read_u64(ent + O_SPRITE_NAME_PTR) != Some(p) { let _ = safe_write_u64(ent + O_SPRITE_NAME_PTR, p); }
        if safe_read_u64(ent + O_SPRITE_NAME_LEN) != Some(l) { let _ = safe_write_u64(ent + O_SPRITE_NAME_LEN, l); }
    }
}

// ───────────────────────── 트램폴린 설치 ─────────────────────────
fn install_serpen_hook() {
    if SERPEN_HOOK_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    unsafe {
        let base = GetModuleHandleW(core::ptr::null());
        if base == 0 { SERPEN_HOOK_INSTALLED.store(false, Ordering::Relaxed); return; }
        let fn_addr = base + SERPEN_RVA;
        // 프롤로그 검증 (패치 전 필수)
        for i in 0..12 {
            if *((fn_addr + i) as *const u8) != SERPEN_PROLOGUE[i] {
                log_push(format!("[{}ms] prologue mismatch @+{}: {:#x} (RVA {:#x}) — 훅 미설치",
                    now_ms(), i, *((fn_addr + i) as *const u8), SERPEN_RVA));
                return;
            }
        }
        // 트램폴린 스텁 = 원본 프롤로그 12B + jmp (fn_addr+0xc)
        let stub = VirtualAlloc(0, 64, 0x3000, 0x40); // COMMIT|RESERVE, EXECUTE_READWRITE
        if stub == 0 { return; }
        let mut sb: Vec<u8> = Vec::new();
        sb.extend_from_slice(&SERPEN_PROLOGUE);
        sb.extend_from_slice(&[0x48, 0xb8]); sb.extend_from_slice(&((fn_addr + 0xc) as u64).to_le_bytes());
        sb.extend_from_slice(&[0xff, 0xe0]); // jmp rax
        core::ptr::copy_nonoverlapping(sb.as_ptr(), stub as *mut u8, sb.len());
        SERPEN_TRAMP.store(stub, Ordering::Relaxed);
        // 원본 = mov rax,detour; jmp rax (12B)
        let d = serpen_detour as usize;
        let mut patch = [0u8; 12];
        patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&d.to_le_bytes());
        patch[10] = 0xff; patch[11] = 0xe0;
        let mut old = 0u32;
        if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 { return; }
        core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
        VirtualProtect(fn_addr, 12, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
        log_push(format!("[{}ms] serpen hook installed fn={:#x} stub={:#x} (RVA {:#x})",
            now_ms(), fn_addr, stub, SERPEN_RVA));
    }
}

// ───────────────────────── 모드 진입 ─────────────────────────
static SETUP_DONE: AtomicBool = AtomicBool::new(false);
// 훅/SEH/cfg 1회 설치 (on_init 미발화 대비 post_update에서도 호출 — 가드로 실제 실행은 1회)
//   ★실기능 훅만 무조건 설치한다(진단/실험 훅은 재설계 2026-07-26에 전면 제거).
fn ensure_setup() {
    if SETUP_DONE.swap(true, Ordering::Relaxed) { return; }
    MAIN_TID.store(unsafe { GetCurrentThreadId() }, Ordering::Relaxed); // on_init/post_update = 메인 스레드
    seh_install();
    load_cfg();
    load_i18n();   // ★attr 표시명이 i18n에서 오므로 load_attrs 전에 로드
    load_attrs();
    install_serpen_hook();       // ① 속성 배정 + 팀버프(템플릿 write)
    install_launcher_hook();     // ② 화면(LIVE) 경기 Game→seed 확정
    install_render_step_hook();  // ③ 활성 뷰 played_tick 캡처
    install_mobatick_hook();     // ④ 처치 팀귀속(track_kills) + 장로 처형 발동
    install_dmg_hook();          // ⑤ 장로 처형(데미지 증폭)
    // ⑥ 색 교체 = 생산자 seam(키 리졸버). 스프라이트 렌더 시 CURRENT_ATTR 기반으로 에셋 키를 교체.
    if !KEYRES_INSTALLED.swap(true, Ordering::Relaxed) {
        unsafe { install_tramp12(KEYRES_RVA, &KEYRES_PROLOGUE, keyres_detour as usize,
                                 &KEYRES_TRAMP, "생산자seam(키리졸버)"); }
    }
    install_uiloader_hook();     // ⑦ 툴팁/장로버프 패널 가산주입
    install_arg_str_hook();      // ⑧ 세르펜 카운터 툴팁에 속성별 처치 스택 주입(arg_str seam)
}
static DB_PROBE_LOGGED: AtomicBool = AtomicBool::new(false);
// db 캡처 상태(head에 항상 최신 출력 — 1회성 로그는 메뉴에서 소진돼 InGame 진단을 못 봄)
static DB_INGAME: AtomicBool = AtomicBool::new(false);
static DB_PTR: AtomicUsize = AtomicUsize::new(0);
// ★런타임 자동 오프셋 탐색: db 영역을 훑어 Game→provider→seed가 세르펜 detour가 관측한 seed와
//   일치하는 슬롯을 찾는다. 하드코딩 오프셋(0.5.0 기준 A/B)이 패치마다 죽는 문제를 근본 해결.
// ★화면 경기 seed — 게이트의 진짜 기준. detour rcx와 Game+0x1dc0(provider)은 같은 경기라도
//   서로 다른 객체라 주소 비교가 실패한다(실측: 매칭슬롯 1개인데 rcx 불일치). seed는 경기 고유.
static LIVE_SEED: AtomicU64 = AtomicU64::new(0);
// 직접 플레이/관전 경기 Game → provider를 클라 db에서 캡처 (post_update=메인 스레드).
//   후보 A: *(db+0x1340)+0x1950 / 후보 B: *(db+0x1af08). Game+0x1660=provider, +0xeab8=seed.
//   seed가 우리 WORLDS(사이드테이블)에 있으면 그 provider = 화면 경기 → LIVE_PROVIDER.
// db+off의 u64를 Game 후보로 보고 → +0x1dc0=provider → +SEED_OFF=seed 가 살아있는 경기 seed면 그 provider.
unsafe fn capture_live_from_db(scene: &Scene, in_match: bool) {
    let Scene::InGame { data } = scene else { DB_INGAME.store(false, Ordering::Relaxed); return; };
    DB_INGAME.store(true, Ordering::Relaxed);
    let db = data.db();
    let dbp = &*db as *const ClientDatabase as usize;
    DB_PTR.store(dbp, Ordering::Relaxed);
    // ★재생 커서 읽기 (Spectator_Chat 방식): 리플레이/다시보기는 SDK game_view, 라이브는 raw.
    //   ⚠라이브 raw는 frames(events)가 준비되기 전엔 played가 쓰레기값(포인터 등)이라 반드시 검증.
    // ⚠PLAYED_TICK은 여기서 세팅하지 않는다 — db+0x1598은 실측상 sim_tick보다 앞서 나와(비율 1.03)
    //   재생 커서가 아니었고, 그걸로 필터하니 미래 처치가 안 걸러졌다(일시정지 중 2→3).
    //   재생 커서 정본 = post_update의 game_time 라벨("06:42" → 초×30). 여기 값은 진단용만.
    if let Some(gv) = db.game_view.as_ref() {
        PLAYED_TICK.store(gv.client.view.played_tick as u64, Ordering::Relaxed); // 리플레이 경로(SDK)
        PLAYED_SRC.store(1, Ordering::Relaxed);
    } else {
        let tag = safe_read_i32(dbp + SCENE_TAG_OFF).unwrap_or(-1) as u32;
        SCENE_TAG.store(tag as u64, Ordering::Relaxed);
        // ★tag 전이 기록: variant 표가 rmeta 선언순 추정이라, 실제 화면 전환과 대조해 InGame tag를 확정한다.
        //   (경기 진입/이탈 시 어떤 값으로 바뀌는지 보면 됨)
        let prev = LAST_SCENE_TAG.swap(tag as u64, Ordering::Relaxed);
        if prev != tag as u64 {
            let n = TAG_LOG_N.fetch_add(1, Ordering::Relaxed);
            if n < 30 {
                let pl = safe_read_u64(dbp + LIVE_PLAYED_OFF).unwrap_or(u64::MAX);
                let el = safe_read_u64(dbp + EV_LEN_OFF).unwrap_or(0);
                let ep = safe_read_u64(dbp + EV_PTR_OFF).unwrap_or(0);
                let ok = ep > 0x10000 && (ep as usize) < 0x7ff0_0000_0000 && el >= 1 && el <= 10_000_000 && pl <= el;
                log_push(format!("[{}ms] ◆scene tag 전이: {} → {} | played={} events.len={} ptr={:#x} 유효={}",
                    now_ms(), prev as i64, tag as i64, pl as i64, el, ep, ok));
            }
        }
        // ★경기 화면 판정 = UI에 "game_time" 노드 존재 (Spectator_Chat 검증 방식 — 라이브/리플레이 모두 통과).
        //   scene tag 해석은 폐기: rmeta 추정표가 실측 전이(3→4→5→6→9)와 안 맞고, 패턴 스캔은 오탐 21개.
        //   game_time이 있으면 scene payload가 InGame이므로 Spectator_Chat 고정 오프셋이 유효하다.
        if in_match {
            let evlen = safe_read_u64(dbp + EV_LEN_OFF).unwrap_or(0);
            EV_PTR.store(safe_read_u64(dbp + EV_PTR_OFF).unwrap_or(0), Ordering::Relaxed);
            EV_LEN.store(evlen, Ordering::Relaxed);
            DB_PLAYED_RAW.store(safe_read_u64(dbp + LIVE_PLAYED_OFF).unwrap_or(u64::MAX), Ordering::Relaxed);
            // ★db+0x1630(라이브 활성뷰 view#2 커서) 후보. **game_time 라벨(정답)과 대조**해 근접(±90틱=3초)
            //   & events.len 이하면 정밀 tick으로 채택(초 단위 라벨보다 정밀). 불일치=유휴뷰 → 라벨 유지.
            let t1630 = safe_read_u64(dbp + VIEW2_TICK_OFF).unwrap_or(u64::MAX);
            VIEW_TICK_DIAG.store(t1630, Ordering::Relaxed);
            let gt = PLAYED_TICK.load(Ordering::Relaxed); // post_update가 이미 세팅한 game_time 값
            if gt > 0 && t1630 < 10_000_000 && (evlen == 0 || t1630 <= evlen + 600)
                && (t1630 as i64 - gt as i64).abs() < 90 {
                PLAYED_TICK.store(t1630, Ordering::Relaxed);
                PLAYED_SRC.store(7, Ordering::Relaxed); // 7 = db+0x1630(라이브 정밀커서, 라벨 검증됨)
                if !DB_PROBE_LOGGED.swap(true, Ordering::Relaxed) {
                    log_push(format!("[{}ms] ★★라이브 정밀커서 확정: db+0x1630={} (game_time≈{} events.len={})",
                        now_ms(), t1630, gt, evlen));
                }
            }
        }
    }
    // ★★화면 경기 = db 3-deref 정석 (2026-07-17 RE). db 128KB 스캔은 폐기 —
    //   VEH 폴트 25만의 주범이었고 매칭이 0~3개로 요동쳤다. 구 "후보 A"는 베이스(db+0x1340)가
    //   정답이었고 내부 오프셋만 틀렸다(+0x1950 → 실제 +0x1dc0).
    //   db+0x1340 = ClientScene payload = *mut Game → +0x1dc0 = provider(= 세르펜 detour rcx) → +0xeab8 = seed.
    //   ⚠payload는 scene 태그가 경기 화면일 때만 유효 → in_match(game_time 노드) 게이트 병행 필수.
    //   ⚠캐시 금지(경기 전환 시 stale) → 매 프레임 새로 읽는다. 3-deref라 비용 무시 가능.
    // ⚠db → provider 링크는 **존재하지 않음**이 확정됐다(2026-07-17 RE): GameView(scene payload)는
    //   순수 이벤트-리플레이 렌더 상태이고 World/Game 핸들을 갖지 않는다(Default derive 전수 열거로 확인).
    //   db+0x1340이 0인 것도 정상 — 그건 Game 포인터가 아니라 **인라인 payload의 첫 8바이트**다
    //   (Spectator_Chat 산식 scene+8+0x258 = db+0x1598 이 실측 일치하는 것이 근거).
    //   ⇒ 화면 경기 식별은 **런처 훅(cap_launcher)** 이 담당한다. 여기선 재생 커서만 읽는다.
}
// ★처치 이력을 메인 스레드에서도 동기화 — 세르펜이 죽으면 세르펜 detour가 더 이상 오지 않아
//   마지막 처치가 영영 누락된다(실측: 화염 처치가 집계에 안 잡힘). 화면 경기 provider에서 직접 읽는다.
fn build_tooltip_text(team: u64) -> String {
    let played = PLAYED_TICK.load(Ordering::Relaxed);
    let ls = LIVE_SEED.load(Ordering::Relaxed);
    if ls == 0 { TIP_FAIL.store(1, Ordering::Relaxed); return String::new(); } // 화면 경기 미확보
    // ★2026-08-02: POOL/ELDER 락 → 무락 스냅샷(ATTR_SNAP).
    let Some(sn) = attrs() else { TIP_FAIL.store(1, Ordering::Relaxed); return String::new(); };
    let (pool, elder) = (&sn.pool, &sn.elder);
    let wg = WORLDS.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ws) = wg.as_ref().and_then(|w| pick_live(w, ls)) else {
        TIP_FAIL.store(2, Ordering::Relaxed); return String::new(); // WORLDS에 그 경기 없음/파티션 모호
    };
    TIP_KILLS.store(ws.kills.len() as u64, Ordering::Relaxed);
    TIP_PLAYED.store(played, Ordering::Relaxed);
    TIP_WAVES.store(ws.waves.len() as u64, Ordering::Relaxed);
    // ★표시는 **재생 기준**(played_tick) — sim이 재생보다 앞서 달리며 미래의 처치를 미리 쌓는다.
    //   (실측: 일시정지 중인데 2마리 → 3마리로 혼자 늘어남 = 뒷부분 처치가 미리 계산된 것)
    //   화면에서 아직 죽지도 않은 세르펜을 세면 스포일러이자 오표시다.
    //   ⚠kills는 시간순이므로 played를 넘는 순간 이후도 전부 미래 → break.
    //   ⚠단 팀 판정용 kill_counts()는 게임 "(N스택)"(sim 기준)과 대조해야 하므로 필터를 걸지 않는다.
    // ★★serpen_logs[i] = **웨이브 i의 죽음**(세르펜은 경기당 1마리 → 1:1, 2026-07-17 RE 확증).
    //   → 순서 매칭/정렬 로직 불필요. i를 그대로 웨이브 인덱스로 쓴다.
    //   ⚠표시는 재생 기준: 처치 tick(= World.tick 축)이 played_tick 이하인 것만. kills는 시간순 → break.
    // ★★2026-07-24 재전환: 팀 귀속 소스 = **SCREEN_KILLS(화면 카운터 델타 이력)**.
    //   구 ws.kills 방식은 같은 (seed,fp) 클론들의 마지막 기록자 복불복로 팀이 뒤집혀
    //   "종류가 이상하다/바닐라" 제보의 잔여 원인이었다. 화면 이력은 정의상 화면과 동기.
    //   원소 = waves[ki] (파티션된 배정표 — sim/렌더/툴팁 공용이라 스프라이트와 항상 일치).
    let want = SERPEN_CNT_ONSCREEN[(team as usize) & 1].load(Ordering::Relaxed);
    TIP_TEAM_AT.store(team as u64, Ordering::Relaxed);   // 호버 시점의 판정팀·기대치(진단)
    TIP_WANT.store(want, Ordering::Relaxed);
    let mut cnt: HashMap<i32, u32> = HashMap::new();
    {
        let skg = SCREEN_KILLS.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(sk) = skg.as_ref() {
            for (t, ki, _gt) in sk.iter() {
                if *t != team { continue; }
                if let Some(&(_, a)) = ws.waves.get(ki) { *cnt.entry(a).or_insert(0) += 1; } // 원소 = 웨이브 배정표
            }
        }
    }
    if cnt.is_empty() { TIP_FAIL.store(3, Ordering::Relaxed); return String::new(); } // 재생된 처치 없음/웨이브 매칭 실패
    TIP_FAIL.store(0, Ordering::Relaxed);
    let get = |i: i32| -> Option<&Attr> {
        if i == ELDER_IDX { elder.as_ref() } else { pool.get(i as usize) }
    };
    // 목록 + 스탯 합산
    let mut lines: Vec<String> = Vec::new();
    let mut total = [0i32; NUM_STATS];
    let mut idx: Vec<(&i32, &u32)> = cnt.iter().collect();
    idx.sort_by_key(|(a, _)| **a);
    for (a, c) in idx {
        let Some(at) = get(*a) else { continue };
        lines.push(format!("{} x{}", at.display_name, c));
        for i in 0..NUM_STATS { total[i] = total[i].saturating_add(at.stats[i] * (*c as i32)); }
    }
    let sum: Vec<String> = (0..NUM_STATS)
        .filter(|&i| total[i] != 0)
        .map(|i| format!("{} +{}{}", tr(&format!("stat.{}", STAT_KEYS[i].0)), total[i], if STAT_PCT[i] { "%" } else { "" }))
        .collect();
    // 구분선 = ASCII(-----). 유니코드 ─(U+2500)은 영어 폰트에 글리프 없어 □로 깨짐(07-25 실측).
    format!("\n{}\n-----\n{}", lines.join("\n"), sum.join(", "))
}
// ── arg_str 툴팁 표시 훅 (복구 2026-07-26): 세르펜 카운터 호버 시 게임 툴팁 "Stats" 값을
//   update_tooltip이 만든 본문(속성별 처치 스택+총 버프)으로 잠깐 교체. 현행 0.5.2 RVA에서 크래시 없이
//   동작 — 구 "재시도금지"(0.5.1 RVA 0xb4fda0)는 현행 무효, 인게임 검증됨(2026-07-26).
// ★0.5.3 확정(2026-07-29, 독립 2방법 일치): ①콜러 사상 투표 61표(2위 23)·크기 359→359 무변화
//   ②"Stats" 를 LEA한 직후 호출하는 지점이 두 exe 각각 1곳뿐인데 그 콜 타깃이 0x1228a90.
//   진입 15B가 0.5.2와 바이트 동일(push×6=8B + sub rsp,0x88=7B) ⇒ 아래 15B 재배치 로직 무수정.
//   ⚠쌍둥이 후보 0x1e7610(engine_ui)·0x1a2ed40(effect view)은 오답 — 훅해도 조용히 미발화.
const ARG_STR_RVA: usize = 0x16a31e0; // 0.5.4 (구0.5.3=0x1228a90)
static ARG_STR_TRAMP: AtomicUsize = AtomicUsize::new(0);
static ARG_STR_HOOKED: AtomicBool = AtomicBool::new(false);
static CFG_TOOLTIP: AtomicBool = AtomicBool::new(true); // 툴팁 스택 표시 = 항상 on(복구)
type ArgStrFn = extern "win64" fn(u64, u64, u64, u64, u64) -> u64;
extern "win64" fn arg_str_detour(rcx: u64, rdx: u64, r8: u64, r9: u64, a5: u64) -> u64 {
    let t = ARG_STR_TRAMP.load(Ordering::Relaxed);
    if t == 0 { return 0; }
    let orig: ArgStrFn = unsafe { core::mem::transmute(t) };
    // 값싼 게이트: 호버 중 + key_len==5 + 우리 텍스트 있음
    if !(CFG_TOOLTIP.load(Ordering::Relaxed) && HOVER_TEAM.load(Ordering::Relaxed) >= 0 && r9 == 5) {
        return orig(rcx, rdx, r8, r9, a5);
    }
    let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        let mut kb = [0u8; 5];
        if !safe_copy(kb.as_mut_ptr(), r8 as *const u8, 5) || &kb != b"Stats" { return None; }
        let sp = a5 as usize; // &String { cap@+0, ptr@+8, len@+0x10 }
        let (c, p, l) = (safe_read_u64(sp)?, safe_read_u64(sp + 8)?, safe_read_u64(sp + 0x10)?);
        let txt = TOOLTIP_TEXT.lock().unwrap_or_else(|e| e.into_inner());
        if txt.is_empty() { return None; }
        if !safe_write_u64(sp, txt.len() as u64) { return None; }
        safe_write_u64(sp + 8, txt.as_ptr() as u64);
        safe_write_u64(sp + 0x10, txt.len() as u64);
        Some((sp, c, p, l))
    }));
    match ok {
        Ok(Some((sp, c, p, l))) => {
            let r = orig(rcx, rdx, r8, r9, a5); // 게임이 우리 문자열을 clone
            unsafe { safe_write_u64(sp, c); safe_write_u64(sp + 8, p); safe_write_u64(sp + 0x10, l); } // ★원복 필수
            TIP_SWAPS.fetch_add(1, Ordering::Relaxed);
            r
        }
        _ => orig(rcx, rdx, r8, r9, a5),
    }
}
fn install_arg_str_hook() {
    if ARG_STR_HOOKED.swap(true, Ordering::Relaxed) { return; }
    unsafe {
        let base = GetModuleHandleW(core::ptr::null());
        if base == 0 { return; }
        let fn_addr = base + ARG_STR_RVA;
        // 진입 15B 재배치(12B는 명령 중간을 자름 — ghidra-re: push×6(8B)+sub rsp,0x88(7B))
        let stub = VirtualAlloc(0, 64, 0x3000, 0x40);
        if stub == 0 { return; }
        let mut s: Vec<u8> = Vec::new();
        s.extend_from_slice(core::slice::from_raw_parts(fn_addr as *const u8, 15));
        s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&(fn_addr + 15).to_le_bytes());
        s.extend_from_slice(&[0xff, 0xe0]);
        core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
        ARG_STR_TRAMP.store(stub, Ordering::Relaxed);
        let mut old = 0u32;
        if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 { return; }
        let mut patch: Vec<u8> = vec![0x48, 0xb8];
        patch.extend_from_slice(&(arg_str_detour as usize).to_le_bytes());
        patch.extend_from_slice(&[0xff, 0xe0]);
        core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, patch.len());
        VirtualProtect(fn_addr, 12, old, &mut old);
        log_push(format!("[{}ms] tooltip arg_str hook installed fn={:#x} (RVA {:#x})", now_ms(), fn_addr, ARG_STR_RVA));
    }
}
// 세르펜 카운터 호버 판정 + 툴팁 본문 갱신 (post_update = 메인 스레드)
// UI 트리에서 세르펜/스탯 관련 노드 id를 훑는다 — ghidra가 준 "header.blue_stat.serpen"이
//   실제 트리에 없어서(노드=없음) 진짜 id를 찾아야 함. 1회만.
fn dump_node_ids(n: &Node, out: &mut Vec<String>, hit: &mut Vec<String>) {
    let id = n.id.as_str();
    if !id.is_empty() {
        out.push(id.to_string()); // 전체(개수 파악용)
        if id.contains("serpen") || id.contains("stat") || id.contains("header")
            || id.contains("morgard") || id.contains("tooltip") || id.contains("game_time") {
            hit.push(id.to_string()); // 관심 노드
        }
    }
    for c in n.child.iter() { dump_node_ids(c, out, hit); }
}
// ★툴팁 v3 본체 — 게임 툴팁은 read-only 관찰만(호버 감지용), 표시는 우리 소유 serpen_tip 라벨에.
//   양팀을 함께 표시하므로 "(N 스택)" 팀 대조의 모호성(양팀 동수)도 없다.
fn kill_counts() -> (u32, u32) {
    let ls = LIVE_SEED.load(Ordering::Relaxed);
    if ls == 0 { return (0, 0); }
    let wg = WORLDS.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ws) = wg.as_ref().and_then(|w| pick_live(w, ls)) else { return (0, 0) };
    // ⚠played 필터 없음 — 게임 툴팁의 "(N스택)"은 sim 시점 카운트라 그것과 대조하려면 기준이 같아야 한다.
    let b = ws.kills.iter().filter(|(t, _, _)| *t == 0).count() as u32;
    let r = ws.kills.iter().filter(|(t, _, _)| *t == 1).count() as u32;
    (b, r)
}
// ★"06:42" → 재생 tick. 화면 game_time 라벨 = 지금 재생 중인 경기 시각이라 가장 확실한 재생 지표다.
//   tick = 초 × 30 (sim 30틱/초 — 세르펜 첫 스폰 7200tick = 240초 = 4분으로 검증됨).
//   ⚠db+0x1598(구 played_tick)은 실측상 sim_tick보다 앞서 나와(비율 1.03) 재생 커서가 아니었다
//   → 그걸로 필터하니 미래 처치가 안 걸러졌다(일시정지 중 2→3 증가).
fn parse_game_time(s: &str) -> Option<u64> {
    let t = s.trim();
    let (m, sec) = t.rsplit_once(':')?;
    let m: u64 = m.trim().trim_start_matches(|c: char| !c.is_ascii_digit()).parse().ok()?;
    let sec: u64 = sec.trim().chars().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().ok()?;
    if sec >= 60 { return None; }
    Some((m * 60 + sec) * 30)
}
// "현재 누적 효과(2스택): " → 2. 게임 툴팁이 보여주는 스택수 = 그 팀의 세르펜 처치수.
fn parse_stacks(s: &str) -> Option<u32> {
    let i = s.find("누적 효과(")?;
    let rest = &s[i + "누적 효과(".len()..];
    let j = rest.find("스택")?;
    rest[..j].trim().parse::<u32>().ok()
}
fn update_tooltip(ui: &GameUI) {
    if !NODE_DUMP_DONE.swap(true, Ordering::Relaxed) {
        let (mut all, mut hit) = (Vec::new(), Vec::new());
        dump_node_ids(&ui.root, &mut all, &mut hit);
        hit.sort(); hit.dedup();
        log_push(format!("[{}ms] ◆UI 노드 총 {}개 | 관심노드 {}개: {}", now_ms(), all.len(), hit.len(),
            if hit.is_empty() { "없음".into() } else { hit.join(" | ") }));
    }
    // ★호버 감지 = 커서가 세르펜 카운터 rect 안인지 (게임 툴팁 유무 무관 = 리플레이 호환).
    //   rect: blue=[633,11,36,28] red=[1251,11,36,28]. cursor_to_game()=레터박스 보정 1920x1080.
    let hit_rect = |id: &str| -> bool {
        let Some(n) = ui_kit::find(&ui.root, id).and_then(|t| ui_kit::find(t, "serpen")) else { return false };
        let p = n as *const _ as usize;
        let rd = |o: usize| unsafe { safe_read_i32(p + o) }.map(|v| f32::from_bits(v as u32)).unwrap_or(0.0);
        let (x, y, w, h) = (rd(0x240), rd(0x244), rd(0x248), rd(0x24c));
        if w <= 0.0 || h <= 0.0 { return false; }
        let (mx, my) = ui_kit::cursor_to_game();
        mx >= x && mx <= x + w && my >= y && my <= y + h
    };
    let by_rect: i32 = if hit_rect("blue_stat") { 0 } else if hit_rect("red_stat") { 1 } else { -1 };
    // 게임 툴팁 텍스트(팀 판정 폴백·라이브 호환용, read-only). 리플레이선 보통 빈 문자열.
    let game_tip = ui_kit::find(&ui.root, "tooltip").and_then(|t| ui_kit::find(t, "text"))
        .and_then(|n| ui_kit::label_get(n)).unwrap_or_default();
    if CFG_PROBE_LOG.load(Ordering::Relaxed) { *GAME_TIP_TEXT.lock().unwrap_or_else(|e| e.into_inner()) = game_tip.clone(); }
    // 호버 = 커서 rect(리플레이/라이브 공통) 또는 게임 툴팁 세르펜 키워드(폴백)
    let hovering = by_rect >= 0 || game_tip.contains("세르펜") || game_tip.to_lowercase().contains("serpen")
        || game_tip.contains("누적 효과");
    if !hovering { HOVER_TEAM.store(-1, Ordering::Relaxed); return; }
    TIP_SEEN.fetch_add(1, Ordering::Relaxed);
    let (kb, kr) = kill_counts();
    // 팀: 1순위=커서 rect / 폴백=게임 "(N스택)"↔집계 대조 / 동수·모호=양팀 표시
    let team: i32 = if by_rect >= 0 { by_rect } else {
        match parse_stacks(&game_tip) {
            Some(n) if n == kb && n != kr => 0,
            Some(n) if n == kr && n != kb => 1,
            _ => 2,
        }
    };
    TEAM_BY_RECT.store(by_rect as i64 as u64, Ordering::Relaxed);
    HOVER_TEAM.store(team, Ordering::Relaxed);
    let text = if team == 2 {
        let mut parts: Vec<String> = Vec::new();
        let b = build_tooltip_text(0);
        let r = build_tooltip_text(1);
        if !b.is_empty() { parts.push(format!("[{}]{}", tr("ui.blue_team"), b)); }
        if !r.is_empty() { parts.push(format!("[{}]{}", tr("ui.red_team"), r)); }
        if parts.is_empty() { format!("\n{}", tr("ui.no_kills_paren")) }
        else { format!("\n{}", parts.join("\n")) }
    } else {
        let t = build_tooltip_text(team as u64);
        if t.is_empty() { format!("\n{}", tr("ui.no_kills_paren")) } else { t }
    };
    *TOOLTIP_TEXT.lock().unwrap_or_else(|e| e.into_inner()) = text;
}
// ═══════════ 색↔버프 경계 불변식 (재설계 2026-07-26) ═══════════
//   화면 처치수 total=N  ⟺  지금 화면 세르펜 = 웨이브 N (waves[N])  ⟺  N번째 처치 = 화면킬 ki=N-1.
//   장로 판정 기준은 세 표시축 모두 **내부 elder_after**(= config 1-based − 1 = 0-based, L1404):
//     · 색(스프라이트) : waves[total] == ELDER_IDX   (배정 = serpen_apply_attr L2074: wave_idx >= elder_after)
//     · 버프창          : SCREEN_KILLS latest_ki >= ea (= ELDER_AFTER, L2607)
//     · 처형            : execute_pass (장로 웨이브 처치 후 execute_duration 창)
//   ⟹ 색·버프·처형이 **화면 카운터 축에서 동일 N**을 참조 → 화면 안에서 서로 어긋나지 않는다.
//   ⚠ "재생 화면 ≠ 확정 결과"(되감기 시 색↔확정 divergence)는 게임 thread_rng(sim-ahead) 한계라
//     이 불변식 밖 = 모드로 제거 불가. 별개 과제(MEM\DONE.md 관전≠확정, 재조사금지).
// ─────────────────────────────────────────────────────────────
// ★장로 버프 표시 갱신 (모르가드 버프와 같은 형식). 잔여시간은 **played_tick 기준** = 화면과 동기.
//   sim은 앞서 달리므로 sim tick으로 계산하면 화면보다 빨리 닳는다.
fn update_elder_buff_ui(ui: &mut GameUI) {
    let played = PLAYED_TICK.load(Ordering::Relaxed);
    let ls = LIVE_SEED.load(Ordering::Relaxed);
    let dur = EXEC_DURATION.load(Ordering::Relaxed);
    // ★★버프 트리거 = **화면 세르펜 카운터**(우상단 blue_stat/red_stat.serpen). 2026-07-18 재설계:
    //   game_time 시계가 화면 엔티티보다 1-2분 앞서(desync) 시계 기반은 버프가 일찍 떴다. 카운터는
    //   재생과 동기라 "오르는 순간 = 화면에서 세르펜 죽는 순간". 그게 장로면 버프 시작을 game_time으로
    //   기록하고, 지속은 game_time 델타(양끝 지연 상쇄=화면 경과와 일치)로 카운트다운.
    let mut left = [0u64; 2];
    // 화면 세르펜 카운트 읽기(자식 value 라벨 우선, 없으면 노드 라벨). 숫자만 추출.
    let read_cnt = |team: usize| -> Option<u64> {
        let id = if team == 0 { "blue_stat" } else { "red_stat" };
        let n = ui_kit::find(&ui.root, id).and_then(|s| ui_kit::find(s, "serpen"))?;
        let txt = ui_kit::find(n, "value").and_then(ui_kit::label_get).or_else(|| ui_kit::label_get(n))?;
        let d: String = txt.chars().filter(|c| c.is_ascii_digit()).collect();
        d.parse::<u64>().ok()
    };
    if ls != 0 {
        let use_elder = CFG_ELDER.load(Ordering::Relaxed); // 장로 off면 버프 계산 skip(색은 계속)
        let ea = ELDER_AFTER.load(Ordering::Relaxed) as u64;
        let wg = WORLDS.lock().unwrap_or_else(|e| e.into_inner());
        let ws = wg.as_ref().and_then(|w| pick_live(w, ls));
        // ★처치 ki별 "화면에 뜬 game_time"을 영구 저장(경기 바뀌면 초기화). 만료 후 되감기해도
        //   창 안이면 다시 뜨게 하려면 상태를 edge-trigger로 지우면 안 됨 → ki→game_time 맵에 누적.
        let mut gtkg = ELDER_GT_BY_KI.lock().unwrap_or_else(|e| e.into_inner());
        let gtk = gtkg.get_or_insert_with(HashMap::new);
        let gt_key = ls ^ RENDER_FP.load(Ordering::Relaxed); // 세트(fp)까지 포함해 전환 감지 (07-24)
        if ELDER_GT_SEED.swap(gt_key, Ordering::Relaxed) != gt_key { gtk.clear(); }
        // ── 1) 카운터 판독 + 화면 자체 처치 이력(SCREEN_KILLS) 축적 ──
        let mut cnts: [Option<(u64, u64)>; 2] = [None, None]; // (prev, cnt)
        for team in 0..2usize {
            let Some(cnt) = read_cnt(team) else {
                // ◆진단(07-24 제보): 읽기 실패 = 이전 값(stale, 세트 경계면 전 세트 값) 유지 증거
                if CFG_PROBE_LOG.load(Ordering::Relaxed) {
                    let n = CNT_READFAIL_N.fetch_add(1, Ordering::Relaxed);
                    if n < 12 { log_push(format!("[{}ms] ◆카운터읽기실패 팀{} (stale={} 유지) ls={:#x}",
                        now_ms(), team, SERPEN_CNT_ONSCREEN[team].load(Ordering::Relaxed), ls)); }
                }
                continue
            };
            let prevc = SERPEN_CNT_ONSCREEN[team].swap(cnt, Ordering::Relaxed); // 색이 쓰므로 항상 갱신
            cnts[team] = Some((prevc, cnt));
            // ◆진단(07-24 제보): 화면 카운터 전이 = "외형" 스트림의 인덱스 소스 실측
            if CFG_PROBE_LOG.load(Ordering::Relaxed) && prevc != cnt {
                log_push(format!("[{}ms] ◆화면카운터 팀{} {}→{} played={} ls={:#x}",
                    now_ms(), team, prevc, cnt, played, ls));
            }
        }
        let mut skg = SCREEN_KILLS.lock().unwrap_or_else(|e| e.into_inner());
        let sk = skg.get_or_insert_with(Vec::new);
        let sk_key = ls ^ RENDER_FP.load(Ordering::Relaxed);
        if SK_KEY.swap(sk_key, Ordering::Relaxed) != sk_key { sk.clear(); } // 세트 전환 → 이력 리셋
        for team in 0..2usize {
            if let Some((prevc, cnt)) = cnts[team] {
                if cnt > prevc && cnt.saturating_sub(prevc) <= 8 { // 카운터 델타 = 화면에서 그 팀이 처치
                    for _ in prevc..cnt {
                        let ki = sk.len() as u64;
                        sk.push((team as u64, ki, played.max(1)));
                        if CFG_PROBE_LOG.load(Ordering::Relaxed) {
                            log_push(format!("[{}ms] ◆화면킬 팀{} ki={} played={}", now_ms(), team, ki, played));
                        }
                    }
                }
            }
        }
        // 되감기: 화면 총합보다 이력이 길면 뒤(최신)부터 삭제 = 시간 되돌림과 일치
        let sk_total = SERPEN_CNT_ONSCREEN[0].load(Ordering::Relaxed) + SERPEN_CNT_ONSCREEN[1].load(Ordering::Relaxed);
        if (sk.len() as u64) > sk_total { sk.truncate(sk_total as usize); }
        // ── 2) 장로 버프 창 판정 — 팀 귀속 소스 = SCREEN_KILLS (ws.kills 의존 제거: 클론 복불복 절연) ──
        if use_elder {
            for team in 0..2usize {
                let Some(&(_, latest_ki, _)) = sk.iter().rev().find(|(t, _, _)| *t == team as u64) else { continue };
                if latest_ki < ea { continue; }        // 장로 아님
                // 이 처치가 화면에 처음 뜬 game_time을 기록(있으면 유지). 되감기해도 이 값이 창 기준.
                let gt = *gtk.entry(latest_ki).or_insert_with(|| {
                    let c = ELDER_ANOMALY_N.fetch_add(1, Ordering::Relaxed);
                    if c < 16 { log_push(format!("[{}ms] ◇장로버프ON(화면킬)#{} 팀{} 처치ki={} game_time={}",
                        now_ms(), c, team, latest_ki, played)); }
                    played.max(1)
                });
                // 활성 판정: 현재 game_time이 [gt, gt+dur) 창 안이면 표시(만료 후 되감기해도 재표시).
                if played >= gt {
                    let elapsed = played - gt;
                    if dur == 0 { left[team] = u64::MAX; }
                    else if elapsed < dur { left[team] = dur - elapsed; }
                }
            }
        }
        drop(skg);
        // ★★화면 색도 카운터 기반 (2026-07-18): 현재 화면 세르펜의 웨이브 = 화면 처치수(양팀 합).
        //   N마리 죽었으면 지금 살아있는(또는 다음) 세르펜 = 웨이브 N. game_time 시계 desync 무관.
        //   시계 기반 resolve_color_from_played(앞섬)는 색을 미리 바꿔 장로가 일찍 회색이 됐다.
        let total = SERPEN_CNT_ONSCREEN[0].load(Ordering::Relaxed) + SERPEN_CNT_ONSCREEN[1].load(Ordering::Relaxed);
        if let Some(ws) = ws {
            if let Some(&(_, attr)) = ws.waves.get(&total) {
                let prev = CURRENT_ATTR.swap(attr, Ordering::Relaxed);
                PLAYED_RESOLVED.store(1, Ordering::Relaxed);
                // ◆진단(07-24 제보): 렌더색 전이 = "외형" 스트림 실측 (sim웨이브전이·킬적립과 시간축 대조)
                if CFG_PROBE_LOG.load(Ordering::Relaxed) && prev != attr {
                    log_push(format!("[{}ms] ◆렌더색 total={} → 속성{} (prev {}) ls={:#x} played={}",
                        now_ms(), total, attr, prev, ls, played));
                }
            } else if CFG_PROBE_LOG.load(Ordering::Relaxed) {
                // ◆진단(07-24 제보): total에 해당하는 웨이브가 없음 = 카운터/ws 어긋남(stale seed 의심) 증거
                let n = RENDER_MISS_N.fetch_add(1, Ordering::Relaxed);
                if n < 12 { log_push(format!("[{}ms] ◆렌더색 웨이브없음 total={} ls={:#x} waves={}개 played={}",
                    now_ms(), total, ls, ws.waves.len(), played)); }
            }
            // ★죽는 모션용 직전 웨이브 (2026-07-19): 처치 순간 카운터가 즉시 올라가 CURRENT_ATTR이
            //   다음 세르펜으로 바뀌는데, 그때 죽는 애니가 아직 재생 중이라 "다음 세르펜의 죽는 모션"이
            //   나왔다. 죽는 프레임을 그릴 땐 이 값(방금 죽은 웨이브 = total-1)을 쓴다.
            let prev = if total == 0 { None } else { ws.waves.get(&(total - 1)).map(|&(_, a)| a) };
            PREV_ATTR.store(prev.unwrap_or_else(|| CURRENT_ATTR.load(Ordering::Relaxed)), Ordering::Relaxed);
        }
    }
    ELDER_LEFT_B.store(left[0], Ordering::Relaxed); // 진단: probe_flush가 표시
    ELDER_LEFT_R.store(left[1], Ordering::Relaxed);
    // 폭 90px + 아이콘 → "장로 1:30" (모르가드 버프와 같은 형식)
    for (team, id) in [(0usize, "blue_elder_buff"), (1usize, "red_elder_buff")] {
        let Some(n) = ui_kit::find_mut(&mut ui.root, id) else {
            ELDER_NODE_MISS.fetch_add(1, Ordering::Relaxed); continue };
        if left[team] == 0 { ui_kit::set_visible(n, false); continue; }
        let secs = left[team] / 30; // sim 30틱 = 1초
        let txt = if dur == 0 { tr("ui.elder") } else { format!("{} {}:{:02}", tr("ui.elder"), secs / 60, secs % 60) };
        if let Some(t) = ui_kit::find_mut(n, "text") { ui_kit::label_set(t, &txt); }
        if let Some(n2) = ui_kit::find_mut(&mut ui.root, id) { ui_kit::set_visible(n2, true); }
        ELDER_UI_N.fetch_add(1, Ordering::Relaxed);
    }
}
static ELDER_UI_N: AtomicU64 = AtomicU64::new(0);
static ELDER_LEFT_B: AtomicU64 = AtomicU64::new(0); // 진단: 마지막 산출 잔여틱(블루)
static ELDER_LEFT_R: AtomicU64 = AtomicU64::new(0); // 진단: 마지막 산출 잔여틱(레드)
static ELDER_NODE_MISS: AtomicU64 = AtomicU64::new(0); // 진단: elder_buff 노드 미발견 횟수
// 진단: 현재 표시 중인 버프의 출처 [팀] = (웨이브idx<<56 | spawn_tick<<32 | kill_tick). 스폰tick vs kill_tick 대조용.
static LAST_BUFF_SRC: [AtomicU64; 2] = [const { AtomicU64::new(0) }; 2];
static ELDER_ANOMALY_N: AtomicU64 = AtomicU64::new(0); // 진단: "스폰 전 버프" 이상 포착 수
static BUFF_SHOW_DELAY: AtomicU64 = AtomicU64::new(0); // 버프 표시 지연 tick(cfg buff_show_delay_tick). 0=처치 즉시.
static GT_DUMPED: AtomicBool = AtomicBool::new(false); // game_time 서브트리 1회 덤프 게이트
// ★버프 카운터 트리거 상태: 화면 세르펜 카운트 이전값 / 장로버프 시작 game_time / 진단용 현재카운트
static ELDER_BUFF_START: [AtomicU64; 2] = [const { AtomicU64::new(0) }; 2]; // (진단 유지)
static SERPEN_CNT_ONSCREEN: [AtomicU64; 2] = [const { AtomicU64::new(0) }; 2];
// ◆07-24 제보 진단 카운터 (외형↔버프 어긋남 4-스트림 대조용)
static KILLGROW_LOG_N: AtomicU64 = AtomicU64::new(0);  // 킬적립 로그 수(배경경기분)
static CNT_READFAIL_N: AtomicU64 = AtomicU64::new(0);  // 화면카운터 읽기실패 로그 수
static RENDER_MISS_N: AtomicU64 = AtomicU64::new(0);   // 렌더색 웨이브없음 로그 수
static TL_ANOMALY_N: AtomicU64 = AtomicU64::new(0);    // ⚠타임라인모순(스폰tick≠킬+7200) 로그 수
static KILLDIFF_LOG_N: AtomicU64 = AtomicU64::new(0);  // ⚠킬재기록(같은 idx 팀/틱 변경) 로그 수
static KILLDIFF_SAME_TID: AtomicU64 = AtomicU64::new(0); // 07-24 FP조사: 재기록이 같은 스레드(재계산)
static KILLDIFF_DIFF_TID: AtomicU64 = AtomicU64::new(0); // 07-24 FP조사: 재기록이 다른 스레드(동시)
// 버프 write 검증·게이트 사유 진단 (RB_LAST_WAVE 전역 게이트는 07-24에 per-경기 ws.rb_logged로 교체)
static GATE_N: [AtomicU64; 3] = [const { AtomicU64::new(0) }; 3]; // 0=속성없음 1=캠프무효 2=바닐라웨이브
static TIP_TEAM_AT: AtomicU64 = AtomicU64::new(9);  // 툴팁 생성 시점의 팀(진단)
static TIP_WANT: AtomicU64 = AtomicU64::new(0);     // 그때의 화면 처치수(진단)
// 처치 ki → 화면에 처음 뜬 game_time (만료 후 되감기 재표시용, 영구). 경기 전환 시 clear.
static ELDER_GT_BY_KI: Mutex<Option<HashMap<u64, u64>>> = Mutex::new(None);
static ELDER_GT_SEED: AtomicU64 = AtomicU64::new(0);
// ★★화면 자체 처치 이력 (07-24 클론분기 대응): (team, ki, 표시된 game_time). 화면 카운터가 오르는
//   순간에만 append — sim 클론들의 ws.kills(마지막 기록자 복불복·팀 뒤집힘 오염)와 완전 절연.
//   ki = 전역 처치순번(= append 시점 len) = 웨이브idx (화면 기준 1:1). 되감기로 카운터가 줄면 truncate,
//   세트 전환(ls^fp 변화)이면 clear. 툴팁·장로버프 팀 귀속의 유일 소스.
//   ⚠락 순서: WORLDS→ELDER_GT_BY_KI→SCREEN_KILLS→PROBE_LOG (역전 금지).
//     ★2026-08-02: POOL/ELDER는 무락 스냅샷(ATTR_SNAP)으로 바뀌어 락 순서에서 제외됐다.
static SCREEN_KILLS: Mutex<Option<Vec<(u64, u64, u64)>>> = Mutex::new(None);
static SK_KEY: AtomicU64 = AtomicU64::new(0);
// 레드팀 모르가드 버프(게임 원본 노드)를 오른쪽 5px 이동 — item_tactics 검증 패턴
//   (force_blue_slot_spacing): .ui 원본값 기준 **고정 목표값을 매 프레임 4상태 전부 강제**.
//   게임/러너가 매 프레임 바닐라로 재설정해도 post_update가 다시 이김. 캐시 없음 = 견고.
//   ⚠하드코딩(-395/-405) 폐기: 실제 원본 x를 몰라 방향/크기가 계속 어긋났다. → **런타임 원본을
//   1회 캐시하고 원본+DX로 매 프레임 세팅**(상대 오프셋 = 크기 정확, 방향만 부호로 조정).
//   유저 실측: 지금까지 적용분이 전부 화면상 왼쪽 → 반대(오른쪽)로. DX 음수를 시도(이전과 반대 부호).
const MORGARD_DX: f32 = 12.0;   // 레드 원본 x 오프셋(양수=오른쪽 확정). 유저: +20에서 왼쪽 8 = +12.
const MORGARD_DW: f32 = -20.0;  // 레드 원본 width 오프셋(음수=축소). 유저: 너비 20px 줄임.
// 블루 모르가드는 왼쪽 기준(anchor 없음) → x 감소=왼쪽. 유저: 너비 20 줄이고 줄인만큼(20) 왼쪽으로.
const BLUE_MORGARD_DX: f32 = 0.0;   // 위치 유지(유저: 안 옮겨도 됐음). 왼쪽 기준이라 너비만 줄면 오른쪽 끝만 당겨짐.
const BLUE_MORGARD_DW: f32 = -20.0; // 너비 20px 축소
static MORGARD_NUDGE_N: AtomicU64 = AtomicU64::new(0); // 진단: 세팅 성공 프레임 수
// [0]=red, [1]=blue 원본 x/width 캐시
static MORGARD_SET: [AtomicBool; 2] = [const { AtomicBool::new(false) }; 2];
static MORGARD_X0: [AtomicU32; 2] = [const { AtomicU32::new(0) }; 2];
static MORGARD_W0: [AtomicU32; 2] = [const { AtomicU32::new(0) }; 2];
fn nudge_morgard(ui: &mut GameUI) {
    for (i, id, dx, dw) in [(0usize, "red_morgard_buff", MORGARD_DX, MORGARD_DW),
                            (1usize, "blue_morgard_buff", BLUE_MORGARD_DX, BLUE_MORGARD_DW)] {
        let Some(n) = ui_kit::find_mut(&mut ui.root, id) else { continue };
        let (x0, w0) = if !MORGARD_SET[i].swap(true, Ordering::Relaxed) {
            let cx = ui_kit::node_layout_read(n, ui_kit::NODE_LF_X);
            let cw = ui_kit::node_layout_read(n, ui_kit::NODE_LF_W);
            MORGARD_X0[i].store(cx.to_bits(), Ordering::Relaxed);
            MORGARD_W0[i].store(cw.to_bits(), Ordering::Relaxed);
            log_push(format!("[{}ms] {} 원본 x={} w={} → x{} w{}", now_ms(), id, cx, cw, cx + dx, cw + dw));
            (cx, cw)
        } else {
            (f32::from_bits(MORGARD_X0[i].load(Ordering::Relaxed)), f32::from_bits(MORGARD_W0[i].load(Ordering::Relaxed)))
        };
        ui_kit::node_layout_write_all(n, ui_kit::NODE_LF_X, x0 + dx);
        ui_kit::node_layout_write_all(n, ui_kit::NODE_LF_W, w0 + dw);
        MORGARD_NUDGE_N.fetch_add(1, Ordering::Relaxed);
    }
}
static PLAYED_RESOLVED: AtomicU64 = AtomicU64::new(0); // 0=미시도 1=조회성공 2=구간없음
struct ElementalSerpenExt;
impl ModExtension for ElementalSerpenExt {
    fn on_init(&self, _scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets) {
        ensure_setup();
        probe_flush();
    }
    fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        ensure_setup();
        // ★경기 화면 판정(Spectator_Chat 검증): game_time 노드가 있어야 scene payload가 InGame이다.
        //   ★같은 노드의 텍스트("06:42")가 곧 재생 시각 → 재생 커서(tick)로 쓴다. db+0x1598보다 확실.
        let gt_node = ui_kit::find(&ui.root, "game_time");
        let in_match = gt_node.is_some();
        let was_in = IN_MATCH.swap(in_match, Ordering::Relaxed);
        // ◆진단(07-24 제보): 세트 경계 타임스탬프 — 이 시점에 ls/카운터가 갱신됐는지(stale인지) 대조.
        //   세트 진입인데 "★★재생경기 선택 포착" 로그가 안 따라오면 = 런처 미커버 경로 확정.
        if was_in != in_match && CFG_PROBE_LOG.load(Ordering::Relaxed) {
            log_push(format!("[{}ms] ◆경기화면 {} ls={:#x} 카운터B/R={}/{} played={}",
                now_ms(), if in_match { "진입" } else { "이탈" }, LIVE_SEED.load(Ordering::Relaxed),
                SERPEN_CNT_ONSCREEN[0].load(Ordering::Relaxed), SERPEN_CNT_ONSCREEN[1].load(Ordering::Relaxed),
                PLAYED_TICK.load(Ordering::Relaxed)));
        }
        // ★★재생 커서 정본 = game_time 라벨 (2026-07-18 서브트리 덤프로 확정):
        //   시각 문자열은 game_time **노드 자체가 아니라 자식 `value`**에 있다("12:34"). 그래서 그동안
        //   game_time.label이 ""로 읽혀 실패했다. 이게 화면에 실제 보이는 시계 = 진짜 재생 커서.
        //   db+0x1598/db+0xBA0 논쟁을 우회 — 화면 라벨이 곧 유저가 보는 시각(초×30=tick).
        // 1순위 재생 커서 = 렌더 스텝 훅이 캡처한 활성 뷰 tick(정밀). game_time 라벨 = 폴백(초 단위).
        let gt_tick = gt_node.and_then(|gt| ui_kit::find(gt, "value").and_then(ui_kit::label_get)
            .or_else(|| ui_kit::label_get(gt))).as_deref().and_then(parse_game_time);
        if let Some(t) = gt_tick {
            if let Ok(mut g) = GAME_TIME_TEXT.lock() {
                *g = gt_node.and_then(|gt| ui_kit::find(gt, "value").and_then(ui_kit::label_get)).unwrap_or_default();
            }
            PLAYED_TICK.store(t, Ordering::Relaxed);   // 폴백 먼저 세팅
            PLAYED_SRC.store(6, Ordering::Relaxed);
        }
        let rt = RENDER_TICK.swap(0, Ordering::Relaxed); // 이 프레임에 렌더된 활성 뷰 최대 tick
        let evl = EV_LEN.load(Ordering::Relaxed);
        if rt > 0 && rt < 10_000_000 && (evl == 0 || rt <= evl + 600) {
            PLAYED_TICK.store(rt, Ordering::Relaxed);   // ★정밀 커서(화면과 100% 일치)
            PLAYED_SRC.store(8, Ordering::Relaxed);
            if !GT_DUMPED.swap(true, Ordering::Relaxed) {
                log_push(format!("[{}ms] ★★재생커서=렌더스텝 tick={} (game_time≈{:?} events.len={} 훅발화={})",
                    now_ms(), rt, gt_tick, evl, RENDER_HOOK_N.load(Ordering::Relaxed)));
            }
        }
        // 화면(LIVE) 경기 provider/seed 폴백 캡처 (주 경로 = launcher 훅, 이건 db 스캔 보완)
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { capture_live_from_db(scene, in_match); }));
        // 처치 이력 정본 = track_kills(mobatick post-tick, 카운터 델타) — 킬메시지와 록스텝 보장.
        let mod_active = CFG_ELEMENTAL.load(Ordering::Relaxed) || CFG_ELDER.load(Ordering::Relaxed);
        resolve_render_fp(); // 화면 세트 fp — 미확정이면 프레임마다 재시도(확정 후 no-op)
        if in_match && mod_active { update_tooltip(ui); } else { HOVER_TEAM.store(-1, Ordering::Relaxed); }
        if in_match { update_elder_buff_ui(ui); } // 색 결정(화면 카운터) + 장로 버프 표시
        if in_match { nudge_morgard(ui); }        // 레드 모르가드 버프 오른쪽 5px
        // ★진단 flush는 배포본에서 제거(2026-07-29). probe_flush 호출부 = on_init 한 곳뿐이라
        //   **프로세스당 1회**만 기록된다(실측). 경기 중 상태를 관측해야 할 땐 아래를 되살릴 것:
        //     let npf = now_ms();
        //     if npf.saturating_sub(POST_FLUSH_MS.load(Ordering::Relaxed)) >= 1000 {
        //         POST_FLUSH_MS.store(npf, Ordering::Relaxed); probe_flush();
        //     }
        //   (0.5.3 마이그 인게임 검증은 이 방식으로 완료했다 — 훅 12/12·+0x40 오프셋 정합·런처 게이트 적중)
    }
}

// 서버(경기 sim) 확장: 스폰 클로저가 서버 컨텍스트에서 발화하므로 여기서도 스폰훅 설치.
struct SerpenServerExt;
impl ModServerExtension for SerpenServerExt {
    fn on_server_start(&self, _ctx: &mut ServerModContext) {
        seh_install();
        load_cfg();
        install_spawn_hooks(); // 서버 컨텍스트에서도 provider 캡처 스폰훅 설치
    }
}

fn init(ctx: &GameCtx) -> ModRegistration {
    SAVED_GAMECTX.store(ctx as *const GameCtx as usize, Ordering::Relaxed);
    // ★모드 로드 시점(관전 진입 전)에 훅 설치 — 스폰 클로저를 놓치지 않게. (post_update는 늦음)
    ensure_setup();
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(ElementalSerpenExt);
    reg.set_server_extension(SerpenServerExt);
    reg
}

declare_mod!(init);
