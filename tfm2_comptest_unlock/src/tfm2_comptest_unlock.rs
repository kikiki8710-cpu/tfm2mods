// tfm2_comptest_unlock.rs — 조합 테스트(comp_test) 제약 해제
// =====================================================================================
// 0.5.0 정식판에 새로 내장된 "조합 테스트(comp_test)"의 제약을 무력화한다.
//   ★핵심: 일일횟수·체력차감의 실제 시행은 클라(game_view UI)가 아니라 서버(game_core)
//   권위다. 클라 UI 게이트(FUN_141014a70)만 패치하면 무효였다(인게임 확인). run 핸들러
//   (FUN_14101e3e0)는 StartCompTest(tag 0x1c) 패킷을 메일박스에 push만 하고, 실제
//   카운터증가·체력−5·최종검증은 game_core의 tag-0x1c 핸들러 FUN_1413d2110 내
//   comp_test 아암에서 일어난다. → 서버측을 직접 패치해야 진짜 언락.
//
// 넣는 기능 (game==exe 정적 RE로 6차까지 확정):
//   1) 체력 −5 안 함 — 서버 시작 핸들러 0x13e95cf `sub rax,5`(stamina@Athlete+0x710)의
//      imm8을 0으로. sub rax,0 → CF=0 → cmovae가 원본 stamina 유지 = 불변.
//   2) 일일 무제한 — (a)잔여계산 FUN_141d413d0을 `mov eax,5; ret`로 대체(클라 표시 + 서버
//      재검사#1 동시통과) (b)서버 증가게이트 0x13e2324 `cmp rax,4`의 한도 4→127.
//   3) 선수(athlete) 중복 — 서버는 중복 무검증(6차 확정). 클라 UI 게이트(0x1014ed5 dedup
//      JNE)만 NOP하면 성립. 다운스트림(같은 선수 10명 매치스폰) 안전 판정됨.
//   4) 챔프(champion) 중복 — 서버·클라 둘 다 중복검사 없음(존재성만 검사) = 이미 허용. 패치 X.
//
// 안전: 각 패치는 apply 전 orig 바이트 확인(미스매치=RVA stale → 조용히 스킵, 로그만).
//   전부 함수 중간부(프롤로그·RIP-rel 무관) 또는 함수 전체 대체(호출규약 eax 반환 준수).
//   순수 코드 바이트 write(VirtualProtect RWX→복원) — detour/shadow-call 없음 = AV 위험 없음.
//
// ⚠ 대상 = ~~0.5.0 핫픽스(24109342)~~ ~~0.5.1 정식(24215274)~~ → **0.5.2**(buildid 24310934,
//   exe 69,209,088B, sha256[:16] 40b55c1b819dff50). byte mismatch면 RVA stale → migrate.
//   0.5.2 마이그(2026-07-22, version-migrator): 성격=**버전업급 전면 재정렬**(전역 델타 없음·함수별 제각각)
//   이지만 comptest 대상 함수는 **로직 거의 불변** — 컨테이너 6개 중 4개가 L1-UNIQUE(스켈레톤 바이트동일).
//   byte-patch 13/13 전부 신주소 orig 실측 MATCH. ★단 server_dedup_real은 **jne rel32 변위가 d4→cd로 변경**
//   (아래 해당 항목 주석 참조) — "로직 동일 ≠ 인코딩 동일"의 실례.
//   로그(tfm2_comptest_unlock.txt) 확인. 서버 stamina 오프셋 = Athlete+0x710(0.5.0 PDB TPI 확정,
//   0.4.x +0x6d0에서 shift).
//
// 빌드: powershell -ExecutionPolicy Bypass -File C:\tfm2mods\build_inj_050.ps1 -Src C:\tfm2mods\tfm2_comptest_unlock\src\tfm2_comptest_unlock.rs -ModId tfm2_comptest_unlock
// =====================================================================================

use mod_api::*;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::os::windows::ffi::OsStrExt;   // (26) 크래시 로그 경로 UTF-16 변환
use std::sync::Mutex;

#[path = "ui_inject.rs"]
mod uinj;   // comp_test 아이템칸 모드 소유 드롭다운 오버레이(로더 훅, 체인 설치)
use uinj as ui_inject;                       // (27) 킬스코어 오버레이에서 쓰는 별칭
#[path = "../../ui_kit/ui_kit.rs"]
mod ui_kit;  // 런타임 Node 조작(라벨 텍스트·visible) — 복사 금지, 공용 모듈 import(CLAUDE.md §1)
use std::time::{SystemTime, UNIX_EPOCH, Duration};

const MOD_ID: &str = "tfm2_comptest_unlock";

// ── 패치 테이블(**0.5.5**, image_base 0x140000000) ──
// ★0.5.5 재핀(2026-08-12): 마스크시그(method A, rip-rel/분기 rel 와일드카드) 전역유일 + 컨테이너 difflib(method B) 2방법 교차.
//   ⚠0.5.5 orig 바이트 바뀐 사이트 4건: roster_count_gate(jae 4d→61, fixed jmp 4e→62 재계산)·
//   collected_gate(jne 17→10)·collect_err_gate(je 57→52)·run_push_gate(je 12→05). 나머지 orig 불변.
//   (도구 = ct_055_repin.py=method A / ct_055b.py=method B / 검증 = ct_055_verify.py)
// ★0.5.4 재핀(2026-08-05): 마스크 시그 전역 유일 + 컨테이너 대응 투표 2방법 교차.
// ★0.5.3 재핀(2026-07-30): 컨테이너(앵커맵 25,862쌍) → 컨테이너 안에서 **명령 형태 + 문맥 ±5 유사도**로
//   사이트 재도출. ⚠오프셋 이식 금지(0.5.3 함수는 2~10% 커짐) · ⚠orig 바이트도 바뀐다
//   (점프 거리·레지스터 할당 변경) ⟹ orig/fixed 를 실측 바이트로 **재생성**했다.
//   ⛔미해결 2건(rva: 0)은 apply_one 이 스킵 = 기능만 미적용(크래시 없음).
struct Patch { name: &'static str, rva: usize, orig: &'static [u8], fixed: &'static [u8] }
// ★0.5.2 주소. 시프트 비균일(함수별 제각각)이라 **컨테이너 함수 매칭 → 명령어 difflib 정렬**로
//   사이트별 개별 재도출(단순 델타 더하기 금지). 구 exe 주소는 각 항목 주석에 병기.
const PATCHES: &[Patch] = &[
    // (1) 체력 −5 안 함 [서버]: comp_test arm 참가선수 순회. `sub rax,5`(rax=stamina@Athlete+0x710)
    //     의 imm8을 0으로. sub rax,0 → CF=0 → cmovae rcx,rax 가 원본 stamina 유지 = 불변.
    //     구 0x13e95d2 → 신 0x13ebfb2 (sub rax,5 @0x13ebfaf, imm8 +3).
    // 0.5.1: 서버핸들러 재정렬(구0.5.0_2 0x13d4af0→0.5.1 0xf1d2c0, 음의 드리프트). ghidra-re HIGH,
    //   sub rax,5 @0xf3411a(imm8 +3)·+0x710 stamina 되쓰기 유일확정·orig 05 실측MATCH.
    // 0.5.2: ~~0.5.1 0xf3411d~~ → 0xe93b2d. 컨테이너(서버핸들러) 0xf1d2c0→0xe7ccd0(cos=1.0000·align=0.9994·
    //   28056→28038 instr·big-fn 유일후보) 후 명령어 difflib 정렬 instr#17396→#17396, `sub rax,5` 동일·orig 05 실측 MATCH.
    Patch { name: "no_stamina_cost", rva: 0x2185b96,   // ★0.5.5(구0.5.4=0x20ecf0c) 마스크시그 k=1 유일·orig 05 실측·cont5=0x216e870 // 0.5.4(구0.5.3=0x17f6f44) `sub rax,5` imm
            orig: &[0x05], fixed: &[0x00] },
    // (2a) 일일 무제한 [서버+클라 공유]: 잔여계산 함수(remaining=max(0,5-count))를 통째
    //     `mov eax,5; ret`로 대체 → 항상 5회 남음. 클라 UI게이트 + 서버 재검사#1 동시통과.
    //     구 0x1d413d0 → 신 0x1a6e0b0 (⚠형제 count_today 함수 0x1a6dc10와 혼동주의).
    // 0.5.0_3: daily_remaining 함수 UNIQUE PROL-OK 재링크(구0.5.0_2=0x1a6e0b0).
    // 0.5.2: ~~0.5.1 0x1c0b480~~ → 0x1f14090. 이 함수는 pdata(unwind) 없는 leaf 33B이고 rip-rel/rel32가
    //   전혀 없어 **위치독립** ⇒ 본문 33B 전체를 시그니처로 스캔: OLD/NEW 각각 .text 전역 **정확히 1건**
    //   (OLD 히트가 0.5.1 선언값과 일치 = 방법 자체 검증됨). 신뢰도 HIGH.
    // ~~⛔0.5.3 미해결~~ → ★해결(2026-07-30 ghidra-re): 시그 0건의 진짜 이유 = 재작성이 아니라
    //   **완전 인라인화 + 의미 반전**. 0.5.3은 remaining(=max(0,5−count))이 아니라
    //   `used = (rec_id==outer_id) ? min(count,5) : 0`을 계산하고 `used>=5`로 차단한다.
    //   레코드 필드도 컨테이너-상대 +0x18 시프트(rec_id=base+0xdc1c·count=base+0xdc10·outer_id=base+0xe434).
    //   ⟹ 통짜 `mov eax,5; ret` 대체 대상이 없음 → **클라 인라인 4사이트 개별 패치**(아래 dr_inline_a~d).
    //   패치 원리: A/C/D는 선행 `xor rXX,rXX` 후 `cmove`로 used를 싣는 구조라 cmove를 4B NOP하면
    //   used=0 고정 = 게임 자신의 "id 불일치=fresh day" 분기와 동일한 정상 상태(잔여 만땅).
    //   B는 `and r13b,al`(r13b=setae count>=5 · al=sete id일치) → `xor r13b,r13b`로 exhausted=0 고정.
    //   레코드 write(reset/increment) 경로는 0x9d67a3/0x9d6c9b/0x9d6cb4 별도 주소 = 카운터 기록 무영향.
    //   서버 권위 게이트는 daily_inc_gate(아래)가 담당 — 0x17f239c 실측 재검증 PASS(cmp rax,4·+0x1d0/+0x1dc 문맥).
    // 사이트 A: 클러스터 0x18d9411~(추정: comp_test 팝업/툴팁), cmove @0x18d9436.
    Patch { name: "dr_inline_a", rva: 0x1a8ab76,   // ★0.5.5(구0.5.4=0x2306164) 마스크시그 k=1·orig 4c0f44e2·cont5=0x1a8aa10 // 0.5.4 재핀(구0.5.3=0x18d9436)
            orig: &[0x4c, 0x0f, 0x44, 0xe2],       // cmove r12,rdx
            fixed: &[0x0f, 0x1f, 0x40, 0x00] },    // 4B nop → used=0(xor r12d 선행)
    // 사이트 B: ★진짜 클라 게이트(2차 스윕 디컴 확정) — `if(4<count && rec_id==outer_id) ok=0` 후
    //   `[node+0x261]=!ok`(run 버튼)·`[open_tactics+0x261]`에 같은 r13b 공유 → 한 방에 둘 다 해제.
    Patch { name: "dr_inline_b", rva: 0x1a95776,   // ★0.5.5(구0.5.4=0x2310c86) 마스크시그 k=1·orig 4120c5·cont5=0x1a95570(=CGATE) // 0.5.4 재핀(구0.5.3=0x18e3fd6)
            orig: &[0x41, 0x20, 0xc5],             // and r13b,al (exhausted 플래그 합성)
            fixed: &[0x45, 0x30, 0xed] },          // xor r13b,r13b → exhausted=0, 직후 je 항상 taken
    // ~~사이트 C: RUN 핸들러 0x18f18c7 cmove~~ → ★제거(2026-07-30 2차 스윕): 게이트가 아니라
    //   클라가 요청 페이로드에 넣는 **시드 성분**(seed = (used|X<<32) ^ epoch_ms)이었음.
    //   서버는 자기 레코드로 판정하므로 무의미 + 시드 변화 부작용 회피 위해 원본 유지.
    // 사이트 D: 버튼 빌더A 컨테이너 0x19866f0(앵커) 내부, cmove @0x1987a3d — 버튼 회색화의 실체.
    Patch { name: "dr_inline_d", rva: 0x1afdf6c,   // ★0.5.5(구0.5.4=0x23ce6bc) 마스크시그 k=1·orig 4c0f44f8·cont5=0x1afcc20 // 0.5.4 재핀(구0.5.3=0x1987a3d)
            orig: &[0x4c, 0x0f, 0x44, 0xf8],       // cmove r15,rax
            fixed: &[0x0f, 0x1f, 0x40, 0x00] },
    // ★★2026-08-08 신규 — **훈련 패널의 조합테스트 진입 버튼(5v5 / 라인전) 일일게이트**.
    //   증상: 테스트를 돌리고 나면 그 버튼이 회색이 되어 **팝업 자체를 못 연다**(툴팁 "오늘 …
    //   횟수를 모두 사용했습니다"). 클릭 핸들러가 `node+0x261`(disabled)를 보고 팝업 오픈을 건너뛴다.
    //   ⚠**왜 여태 안 잡혔나**: `dr_inline_d`(0x23ce6bc)가 NOP한 `cmove r15,rax`는 **"N / 5" 라벨 표시용**
    //   값에만 쓰이고, 진짜 게이트는 원본 필드(r13=used, r12/r14=날짜)에서 **여기서 다시 도출**한다.
    //   즉 표시만 고치고 게이트는 살아 있었다(패널 draw는 경고문구 세터도 안 타서 훅 로그에도 안 남음).
    //   식: `record_day==today && used>=5` → cl. 이 곱을 0으로 만들면 로스터 조건만 남는다.
    //   부작용 없음: `and`가 세운 플래그는 바로 다음 `cmp rbx,0xa`가 덮고, `al`은 직후 `mov eax,1`로 사망.
    //   실행 자체는 팝업 사전게이트(0x2310a90)와 서버 게이트가 독립 재검증하므로 OOB 위험도 없다.
    //   마이그 시그(.text 전역 1히트) = `45 39 E6 0F 94 C0 49 83 FD 05 0F 93 C1 20 C1` @0x23cead5(+0xD 지점).
    Patch { name: "panel_btn_daily_gate", rva: 0x1afe392,   // ★0.5.5(구0.5.4=0x23ceae2) 마스크시그 k=1·orig 20c1·cont5=0x1afcc20
            orig: &[0x20, 0xc1],                   // and cl, al   (cl = exhausted)
            fixed: &[0x30, 0xc9] },                // xor cl, cl
    // (2b) 일일 무제한 [서버 증가게이트]: `cmp rax,4`(count) 의 imm8을 4→127 → 카운터가 5 이상이어도
    //     증가·수락 허용(사실상 무제한). 구 0x13e2327 → 신 0x13e4d07 (cmp rax,4 @0x13e4d04, imm8 +3).
    // 0.5.1: ghidra-re HIGH. cmp rax,4 @0xf2d10d(imm8 +3), +0x1d0 count·+0x1dc 마커=daily_remaining
    //   동일 필드셋(서버측)·orig 04 실측MATCH. 4→127로 사실상 무제한.
    // 0.5.2: ~~0.5.1 0xf2d110~~ → 0xe8cb20. 같은 서버핸들러 컨테이너 정렬 instr#12127→#12127,
    //   `cmp rax,4` 동일·orig 04 실측 MATCH.
    // ★0.5.3 2차 스윕(2026-07-30): imm 7f→**ff**로 상향. inc_gate는 카운터를 계속 증가시키므로
    //   (0x17f6df7 `inc rax; mov [rbx+0x1d0],rax`) 127 도달 시 재차단되는 헛점 제거.
    //   cmp rax,imm8은 sign-extend라 ff=-1=unsigned max ⟹ jbe(unsigned) 항상 taken = 진짜 무제한.
    //   사이트 검증(2차): 명령 시작 0x17f2399 `48 83 f8 04`, imm8=+3=0x17f239c·전역 시퀀스 1히트(클론 없음)·
    //   no_stamina_cost와 같은 pdata 함수 0x17e0240..0x180924f = 라이브 확정.
    Patch { name: "daily_inc_gate", rva: 0x2180f84,   // ★0.5.5(구0.5.4=0x20e8246) 마스크시그 k=1·orig 04·cont5=0x216e870 // 0.5.4(구0.5.3=0x17f239c) `cmp rax,4` imm
            orig: &[0x04], fixed: &[0xff] },
    // ★★(2b-2) 서버 **사전거부 게이트** [game_core] — 0.5.3 일일제한 잔존의 진범(2026-07-30 2차 스윕).
    //   서버 핸들러엔 daily 게이트가 **2개**: ①위 inc_gate(카운터 증가 지점) ②이 pre-gate(수락 판정).
    //   0x17ef5d2 call map.get(base+0x16a28, 오늘날짜) → 0x17ef5e3 `cmp [rax+0x1dc],esi`(오늘 레코드?)
    //   → 0x17ef5ef `cmp qword [rax+0x1d0],4`·0x17ef5f7 jbe→허용 / fall-through 0x17ef5fd
    //   `mov byte [rsp+0x20],1` = **거부코드 1(no_attempts) 생산** → 0x17ef616 call 거부 디스패처
    //   = 유저가 본 "오늘은 더 이상…" 안내문구의 실체. code 1 생산지는 exe 전체 이 2곳뿐(둘 다 daily 직후).
    //   전체 명령 실측: `48 83 b8 d0 01 00 00 04 | 0f 86 05 2d 00 00`(전역 1히트·클론 없음).
    //   imm8 ff(-1, sign-extend) → jbe 항상 taken = 무제한. inc_gate와 같은 라이브 함수 내.
    Patch { name: "server_pregate", rva: 0x217e1ac,   // ★0.5.5(구0.5.4=0x20e5471) 마스크시그 k=1·orig 04·cont5=0x216e870 // 0.5.4 재핀(구0.5.3=0x17ef5f6) `cmp qword [rax+0x1d0],4`
            orig: &[0x04], fixed: &[0xff] },
    // (3) 선수 중복 허용 [클라]: setup 게이트 내 athlete_id HashSet dedup(len!=count) →
    //     duplicate_players 거부 jne를 NOP. 서버는 중복 무검증(6차). 이게 되면 로스터"10명 이상"
    //     조건도 자동소멸(1명으로 10슬롯 채워 collected==required 통과, 서버 로스터 재검증 없음-8차).
    //     구 0x1014ed5 → 신 0x1012b25.
    // 0.5.0_3: FORGE_CALLER 컨테이너-델타 검증(orig 75 76 확인, 구0.5.0_2=0x1012b25).
    // ★(3-b) 진짜 서버 dedup [서버] (re_T 2026-07-20): 0xf675f0 등록루프의 SwissSet insert(0xf67b89) 결과
    //   `test al,al; jne 0xf67c6b(mov dil,3 = 중복거부 return)`. 이 6B jne만 NOP하면 fall-through(0xf67b97)로
    //   레지스트리 find→유효성 4조건→루프 계속→0xff 반환(run 진입) = **중복 athlete_id 그대로 등록**.
    //   ⚠구 `server_dedup` 0xf2bbea는 등록 dedup이 아니라 **로스터 join 브로드캐스트** dedup이었음(오진).
    //   과거 "중복 등록=크래시" 판정은 그 틀린 주소를 NOP한 결과 → 본 패치로 무효화.
    //   안전근거(re_T): ef1ea0의 챔프 Arc=iteration내 획득/해제 닫힘·String/Vec=각자 deep clone·
    //   athlete* = 읽기전용(소유 저장 없음) ⟹ 중복 10명 이중해제 위험 없음. 부작용 = 스태미나 −5×10.
    // ★0.5.2 마이그: ~~0.5.1 0xf67b91~~ → **0xec7758**. 컨테이너 0xf675f0→0xec71b0(cos=0.99984·2nd후보
    //   0.99174로 갭 충분·434→442 instr) 명령어 difflib 정렬 instr#363→#367.
    // ⚠★**orig 바이트가 바뀐다**(로직 동일 ≠ 인코딩 동일): jne rel32 변위 `d4`→**`cd`**
    //   (= 0f 85 cd 00 00 00). 분기 거리가 7바이트 줄었을 뿐 의미 동일. 주소만 갈고 orig를 그대로 두면
    //   byte mismatch로 **조용히 skip = 선수중복 기능 통째 사망**하므로 반드시 함께 수정.
    // 시맨틱 재확인(0.5.2 실측): `call <SwissSet insert> 0x1177a10; nop; test al,al; jne 0xec782b` 이고
    //   점프 타깃 0xec782b = `mov dil,3; jmp`(중복거부 return) **바이트동일**, fall-through도 구조동일
    //   (`mov [rbp-0x60],r15; cmp qword[rbp+0x50],0; je ...` — 스택슬롯 -0x58→-0x60 프레임 시프트만).
    //   ⇒ 0.5.1에서 인게임 검증(07-20)된 그 게이트가 맞음. fixed(6B nop)는 무변경.
    // 0.5.3(2026-07-30): 컨테이너 0xec71b0→0x1830900, 문맥점수 11/11 만점. ⚠점프 거리 cd→d4 변경.
    Patch { name: "server_dedup_real", rva: 0x21bf2d3,   // ★0.5.5(구0.5.4=0x2126f73) 2방법(마스크시그 k=1 + 컨테이너델타 off=0x473)·orig 0f85d3000000 실측 // 0.5.4(구0.5.3=0x1830df0)
            orig: &[0x0f, 0x85, 0xd3, 0x00, 0x00, 0x00],
            fixed: &[0x66, 0x0f, 0x1f, 0x44, 0x00, 0x00] },
    // 0.5.2: ~~0.5.1 0x1615495~~ → 0xd00ee5. 컨테이너 0x1615030→0xd00a80 = **L1-UNIQUE(스켈레톤 바이트동일
    //   ·357→357 instr·align=1.0000)** ⇒ 함수 내부 오프셋 전부 보존, instr#291→#291·orig 75 76 실측 MATCH.
    Patch { name: "allow_dup_players", rva: 0x1a95c21,   // ★0.5.5(구0.5.4=0x2311131) 컨테이너 difflib(cont 0x2310a90→0x1a95570)·orig 7547 실측 // 0.5.4(구0.5.3=0x18e4481)
            orig: &[0x75, 0x47], fixed: &[0x90, 0x90] },
    // (5) ★서버 dedup [game_core] — 선수중복 최종 열쇠: 서버 comp_test 핸들러(0x13d4af0) 로스터
    //     빌드 루프의 참가자 유일-필터. 0x13e376b `HashSet.insert(id)` → 0x13e3773 `jne`(75 10)가
    //     중복(al!=0) athlete의 로스터-add(0x13e377f)를 스킵 → 10 dupes→1 참가자 → [rbx+0x3a8]≤1
    //     → 0x13d4bc6 매치 abort(조용한 실패=sim 미생성). NOP화 → 중복도 항상 로스터 등록 → 매치 형성.
    //     안전=로스터가 athlete를 0x740바이트 독립 복사본으로 저장(aliasing 없음, 미러전 동형).
    //     (클라 제출게이트 0x101c077은 A가 al=1이라 무효였음 → 제거. 실차단은 여기 서버였음.)
    // ⚠0.5.1 주소 재핀 완료(ghidra-re HIGH: jne @0xf2bbea, HashSet.insert CALL@0xf2bbe2 직후·orig 75 10 MATCH)
    //   ★비활성 유지(orig==fixed=no-op) — 단 사유 정정(2026-07-20): 여기는 **등록 dedup이 아니라 로스터 join
    //   브로드캐스트 dedup(lobby+0x600)**. NOP하면 중복 join 방송으로 수신측 리스트가 깨짐 = 과거 크래시의 진범.
    //   ~~"athlete_id HashMap 하드리밋·재시도금지"~~는 **오진, 무효**(참가자는 해시가 아니라 로스터 Vec).
    //   ~~크래시 근거 0x140402840~~도 오류(0x402730 EH funclet 내부·명령경계도 아님).
    //   진짜 등록 dedup = 위 server_dedup_real 0xf67b91 (패치 완료·인게임 검증). 이 주소는 기록용으로만 유지.
    // 0.5.2: ~~0.5.1 0xf2bbea~~ → 0xe8b5fa(서버핸들러 컨테이너 정렬 instr#11112→#11112·orig 75 10 MATCH).
    //   ★기록용 no-op(orig==fixed)이라 동작 무관하지만, 주소를 맞춰 둬야 "이 사이트는 여기"라는 기록이 유효.
    Patch { name: "server_dedup", rva: 0x217d01f,   // ★0.5.5(구0.5.4=0x20e42d1) 컨테이너 difflib·orig 7510 실측·no-op 유지 // 0.5.4(구0.5.3=0x17ee49c)
            orig: &[0x75, 0x10], fixed: &[0x75, 0x10] },
    // ★(10) 훈련탭 "조합 테스트 5v5" 버튼 활성 조건 = 로스터 보유인원 ≥10 → **≥5로 완화** (2026-07-20).
    //   disabled = (로스터수 < 10) OR (일일잔여 == 0). 로스터수 = FUN_1415fc8e0 반환 Vec<u64>.len
    //   (드롭다운/collect가 쓰는 것과 같은 목록). 라인업 10칸 채움 판정(0x1615030)과는 **별개 게이트**.
    //   ⟹ 로스터 5명이어도 중복 선택으로 10칸을 채울 수 있으므로(중복 패치 완료) 임계만 낮추면 성립.
    //   빌더가 A(0x167eb50, vtable)·B(0x160c040, 직접호출) **2개**라 둘 다 패치해야 함(어느 쪽이 라이브인지 무관).
    //   ⚠동일 패턴 `49 83 FD 0A`인 0x1683e43은 **라인업 10슬롯 루프 상한**이므로 절대 건드리지 말 것(5=슬롯5개만 그림).
    //   1v1(lane) 버튼은 별도 상수 2(0x167fedd/0x160c1d0) → 무영향. imm8 교체라 분기구조 불변 = 부작용 최소.
    // 0.5.2 마이그: 세 사이트 모두 **컨테이너 L1-UNIQUE(바이트동일)** → 명령어 인덱스 1:1 보존, orig 8B 실측 MATCH.
    //   빌더A 0x167eb50→**0xd95450**(6162→6162 instr·align=1.0000) / 빌더B 0x160c040→**0xcf7970**(254→254·align=1.0000).
    // ⚠★금지 사이트 재확인: 0.5.1 `0x1683e43`(라인업 10슬롯 루프 상한)은 **빌더A와 같은 컨테이너 안**에 있고
    //   패턴도 `49 83 fd 0a`로 유사 → **바이트 패턴 검색으로 재핀했다면 오매칭 위험이 실재**했다. 본 마이그는
    //   패턴검색이 아니라 **컨테이너 명령어 인덱스 정렬**(min_a=instr#968, warn=instr#885, min_b=instr#111)로
    //   도출했으므로 그 사이트를 잡을 수 없음. 변별: min_a=`49 83 fc 0a b8 01`(cmp **r12**;mov eax) vs
    //   금지=`49 83 fd 0a 0f 83`(cmp r13;**jae**).
    // 0.5.3(2026-07-30): ⚠**레지스터가 바뀌었다** — min_a `cmp r12,0xa`→`cmp rbx,0xa`,
    //   warn `cmp rbx,0xa`→`cmp rdi,0xa`. 후속 명령열(mov eax,1 / mov eax,0x38; mov ecx,0x32; cmovb)로
    //   동일 사이트임을 확인(컨테이너 0xd95450→0x19866f0 안에서 유일).
    //   ⛔주의: imm 을 정규화한 자동매칭은 `cmp r12,0x30` 을 오답으로 집었다 — imm 0xa 고정이 필수.
    Patch { name: "btn5v5_roster_min_a", rva: 0x1afe394,     // ★0.5.5(구0.5.4=0x23ceae4) 마스크시그 k=1·orig 4883fb0a0fb6f9b8·cont5=0x1afcc20 // 0.5.4(구0.5.3=0x1987e64) cmp rbx,0xa → 5
            orig:  &[0x48, 0x83, 0xfb, 0x0a, 0x0f, 0xb6, 0xf9, 0xb8],
            fixed: &[0x48, 0x83, 0xfb, 0x05, 0x0f, 0xb6, 0xf9, 0xb8] },
    // ⛔0.5.3 미해결: `cmp r13,0xa; setb r13b` 사이트가 0.5.3 에 없다(레지스터·형태 모두 변경 추정).
    //   컨테이너(0.5.2 0xcf7970)도 앵커 없음 → 투표 컨테이너 안에서 후보 0건. rva 0 = 스킵.
    //   영향: 빌더B 경로의 버튼 disabled 조건이 10명 유지(빌더A·경고문구는 완화 적용됨).
    Patch { name: "btn5v5_roster_min_b", rva: 0,             // ⬜0.5.3 미해결(구0.5.2=0xcf7b68)
            orig:  &[0x49, 0x83, 0xfd, 0x0a, 0x41, 0x0f, 0x92, 0xc5],
            fixed: &[0x49, 0x83, 0xfd, 0x05, 0x41, 0x0f, 0x92, 0xc5] },
    Patch { name: "btn5v5_warn_text",    rva: 0x1afdfac,     // ★0.5.5(구0.5.4=0x23ce6fc) 마스크시그 k=1·orig 4883ff0ab8380000·cont5=0x1afcc20 // 0.5.4(구0.5.3=0x1987a7d) cmp rdi,0xa → 5
            orig:  &[0x48, 0x83, 0xff, 0x0a, 0xb8, 0x38, 0x00, 0x00],
            fixed: &[0x48, 0x83, 0xff, 0x05, 0xb8, 0x38, 0x00, 0x00] },
    // ★(11) ★★진짜 벽(2026-07-23 규명) = **서버측 로스터 인원 게이트** [game_core].
    //   증상: 위 btn5v5_* 3건이 전부 patched+VERIFIED인데도 "선수단 수가 부족합니다"가 계속 뜸.
    //   ⟹ 그 3건은 **클라 패널 버튼/툴팁**만 담당했고, 실제 거부는 서버가 하고 있었음.
    //   경로: 메시지 텍스트키 `training.comp_test.not_enough_roster`(에셋키 문자열 @0x1437042c8, len 0x38).
    //     이 키는 LEA xref가 아니라 **오프셋 테이블 0x14370ebb0의 index 2**로 참조돼 문자열 xref에 안 잡혔음
    //     (테이블 정본 = DISP_RVA 0xd3f780: 0=lane_roster 1=no_attempts **2=not_enough_roster** 3=dup 4=champion).
    //     디스패처 호출점은 exe 전체 단 1곳(0x74d8bc), r8d = 게임코어가 돌려준 거부코드.
    //     모드 disp_detour는 idx 3(dup)만 억제 → idx 2는 그대로 통과 = 유저가 본 메시지.
    //   게이트 본체: 함수 0xec71b0..0xec786f = server_dedup_real(0xec7758)과 **같은 등록루프 함수**.
    //     0xec7641~ 팀 가용선수 카운트 루프(레지스트리 순회·[a+0x568]==내 팀id 필터) → rdx
    //     0xec768e `lea rax,[r15+r15]`  = 필요치 2×팀당인원(5v5=10 / lane=2)
    //     0xec7692 `mov dil,2`(reason)  0xec7695 `cmp rdx,rax`  0xec7698 `jb →거부`
    //   ★핵심 판정(ghidra-re HIGH): 이 게이트는 **선택 배열을 전혀 안 본다**(레지스트리만 순회) ⟹ 세는 대상은
    //     (a)로스터 보유 선수 수. distinct도 선택수도 아님 ⟹ **중복 선택으로 통과 가능 = 임계만 낮추면 성립**.
    //     (중복 검사는 완전 별개 지점 0xec7758 = 이미 NOP 완료.)
    //   ⚠0.5.2 신규 아님 — 0.5.1 0xf67ace `lea rax,[rsi+rsi]`로 동일 존재(레지스터만 rsi→r15).
    //     0.5.1에서 안 걸린 이유 = 당시 세이브의 팀 가용인원이 10 이상이었기 때문(athlete_pool 20개 id가 그 흔적).
    //     ⟹ **버전 문제가 아니라 세이브 인원 문제**. 재조사 금지.
    //   패치: 필요치를 2×r15 → **1×r15**(5v5=5·lane=1)로. rax는 직후 call에서 즉시 덮어써지는 dead 값이라 부작용 없음.
    //     완전해제(`31 c0 90 90`=필요치 0)도 가능하나, 상위 제출검증(0xe81787)에 의존하게 되므로 보수적으로 1×r15 선택.
    //   ⚠금지 사이트 오염 위험 없음: 별도 함수(서버 등록루프)이고 imm 0xa가 아니라 r15*2 계산값이라
    //     UI 10슬롯 루프 상한 패턴과 겹치지 않음(이 함수 내 `cmp r64,0xa`는 0건).
    //   마이그 시그: `4b 8d 04 3f 40 b7 02 48 39 c2`(10B) = .text 전역 UNIQUE.
    // ★0.5.3 재핀(2026-07-30): 문서화된 10B 시그(`4b 8d 04 3f 40 b7 02 48 39 c2`)는 **0건** —
    //   레지스터가 r15→rsi, reason 이 dil→bl 로 바뀌었다. 컨테이너 0xec71b0→0x1830900 안에서
    //   `lea r?,[r?+r?]` 직후 `cmp rdx,rax; jb` 인 사이트가 **유일**(0x1830d2e) ⟹ 확정.
    //   패치도 그에 맞춰 재작성: `lea rax,[rsi+rsi]`(4B) → `mov rax,rsi`(3B) + nop.
    Patch { name: "server_roster_min", rva: 0x21bf230,  // ★0.5.5(구0.5.4=0x2126ed0) 2방법(마스크시그 k=1 + 컨테이너델타 off=0x3d0)·orig 4801db 실측·필요치 2×N → 1×N // 0.5.4(구0.5.3=0x1830d2e)
            orig:  &[0x48, 0x01, 0xdb],                 // `add rbx,rbx` (0.5.3=`lea rax,[rsi+rsi]` 4B)
            fixed: &[0x0f, 0x1f, 0x00] },               // nop3 → 직후 `mov rax,rbx` 가 rax=1×N 로 남김
    // (7) ★★진짜 벽 = "5v5 인원부족" 게이트 [클라]: run 핸들러 제출빌드 진입 직전 0x101c33c
    //     `jae 0x14101c48c`(slot_count>=required면 build). slot_count(+0x1bf0 리스트 len)<10이면
    //     fall-through→roster abort(문구는 디스패처훅으로 억제=조용한 실패). 이게 "5v5 하려면 10명"의 실체.
    //     → 무조건 build jmp. collect가 반환한 중복10개 그대로 빌드→push→서버. 빈슬롯은 collect -1(0x101c318)이
    //     별도 차단하므로 "중복으로 10칸 채운 것"만 통과(안전). orig 6B jae → E9 rel32 jmp + nop.
    // 0.5.0_3: RUN 핸들러(0xf687e0) 컨테이너-델타 검증(orig 0f834a010000 확인, 구0.5.0_2=0x101c33c).
    // ★0.5.2: 아래 4개 게이트는 전부 RUN 핸들러 컨테이너 0x161eab0→**0xd0a440 = L1-UNIQUE(바이트동일·628→628
    //   instr·align=1.0000)** 안에 있다 ⇒ 함수 내 상대오프셋 완전보존, 네 사이트 모두 orig 실측 MATCH.
    //   (jae→jmp 변환 rel32 재계산도 불필요: orig 변위가 그대로 0x14a라 fixed `e9 4b 01 00 00 90` 유효.)
    // 0.5.3(2026-07-30): RUN 컨테이너 0xd0a440→0x18f1180. ⚠jae 변위가 0x14a→0x15e 로 바뀌어
    //   fixed 의 jmp rel32 도 재계산했다: 타겟 0x18f160f - 명령끝 0x18f14b0 = 0x15f.
    Patch { name: "roster_count_gate", rva: 0x1aa2c28,   // ★0.5.5(구0.5.4=0x231e155) 마스크시그 k=1·cont5=0x1aa2930(=RUN) // 0.5.4(구0.5.3=0x18f14ab)
            // ⚠orig 바뀜: jae 변위 0x14d→0x161(0f8361010000). fixed jmp rel32 = disp+1 = 0x162(e962010000 90)로 재계산(타깃 0x1aa2d8f 동일).
            orig: &[0x0f, 0x83, 0x61, 0x01, 0x00, 0x00],
            fixed: &[0xe9, 0x62, 0x01, 0x00, 0x00, 0x90] },
    // (8) collected != required 게이트 [클라]: 0x101c330 `jne →abort`. collect 반환 len != required면 abort.
    //     collect 무-dedup(중복10개 push)이면 ==여야 하나, 실측 막힘 → 이 게이트가 실제 abort일 가능성. NOP.
    Patch { name: "collected_gate", rva: 0x1aa2c1c,  // ★0.5.5(구0.5.4=0x231e142) 마스크시그 k=1·cont5=0x1aa2930 // 0.5.4(구0.5.3=0x18f149f)
            // ⚠orig 바뀜: jne 변위 0x17→0x10(7510).
            orig: &[0x75, 0x10], fixed: &[0x90, 0x90] },
    // (9) ⚠collect==-1 게이트 [클라, 위험]: 0x101c318 `je →abort`. collect가 슬롯서 -1(미선택) 반환시 abort.
    //     NOP=무효슬롯도 진행→garbage build→서버 크래시 위험. 판정용: 크래시=drop커밋 -1 확정(근본).
    Patch { name: "collect_err_gate", rva: 0x1aa2bff,  // ★0.5.5(구0.5.4=0x231e127) 마스크시그 k=1·cont5=0x1aa2930 // 0.5.4(구0.5.3=0x18f1484)
            // ⚠orig 바뀜: je 변위 0x57→0x52(7452).
            orig: &[0x74, 0x52], fixed: &[0x90, 0x90] },
    // (6) run 핸들러 r15 게이트 [클라]: 0x101c9e1 `cmp r15,-1; je 0x14101c453` (r15=빌드산출물).
    //     — r15(빌드산출물 [rbp+0x1a50])==-1이면 메일박스 push 전 조용히 abort(서버 미전송).
    //     je NOP → r15==-1여도 push 강행(서버 전송). ⚠무효 빌드면 서버가 깨진 0x19c0 메시지 받아
    //     크래시 위험(세이브 백업됨). 실험: sim진입=성공 / 크래시=빌드 진짜무효.
    Patch { name: "run_push_gate", rva: 0x1aa3325,  // ★0.5.5(구0.5.4=0x231e838) 컨테이너 difflib(cont 0x231de30→0x1aa2930) // 0.5.4(구0.5.3=0x18f1b95)
            // ⚠orig 바뀜: je 변위 fa12→fa05(0f8405faffff). fixed 6×NOP 불변.
            orig: &[0x0f, 0x84, 0x05, 0xfa, 0xff, 0xff],
            fixed: &[0x90, 0x90, 0x90, 0x90, 0x90, 0x90] },
    // (4) 챔프 중복: 서버·클라 둘 다 존재성만 검사, 중복 reject 없음 = 이미 허용. 패치 없음(기록용).
    // 로스터 인원조건: 별도 패치 불필요(위 (3) dedup으로 자동해소, 서버 재검증 없음).
];

type BOOL = i32; type DWORD = u32; type HMODULE = usize;
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(flags: u32, addr: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, sz: DWORD) -> DWORD;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old: *mut u32) -> BOOL;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, sz: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
    fn GetCurrentThreadId() -> DWORD;                  // (13) 동시실행 프로브: sim 스레드 식별용
    fn GetProcessHeap() -> usize;                      // 0.5.3 game_dealloc 대체용
    fn HeapFree(heap: usize, flags: u32, mem: usize) -> BOOL;
}
#[repr(C)] #[derive(Default)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32,
    region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32 }

static EXE_BASE: AtomicU64 = AtomicU64::new(0);
#[inline] unsafe fn exe_base() -> usize {
    let v = EXE_BASE.load(Ordering::Relaxed) as usize;
    if v != 0 { return v; }
    let b = GetModuleHandleW(core::ptr::null());
    EXE_BASE.store(b as u64, Ordering::Relaxed); b
}
#[inline] unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const RD: u32 = 0x02|0x04|0x20|0x40; const GUARD: u32 = 0x01|0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
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
// ★배포 게이트(2026-07-23 신설): 릴리스본은 로그를 남기지 않는다.
//   종전엔 게이트 자체가 없어 `tfm2_comptest_unlock.txt`가 무조건 append됐다(0.5.1까지 633KB까지 자람).
//   ⚠유저 지원/디버깅 시엔 **여기만 true로 바꿔 재빌드**하면 전량 복구된다(호출부는 손댈 필요 없음).
//   ※기능영속 write(`athlete_pool.txt` pool_save / `comptest_items.cfg` 읽기)는 이 경로와 **분리**돼 있어
//     로그를 꺼도 기능 손실 없음(배포체크리스트 §2 함정 점검 완료).
const LOG_ENABLED: bool = false;  // 릴리스. (조사 시 true — 로그는 mods\<MOD_ID>\<MOD_ID>.txt)
fn log(s: &str) {
    if !LOG_ENABLED { return; }
    if let Some(mut p) = dir() {
        p.push("tfm2_comptest_unlock.txt");
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = write!(f, "{}", s); let _ = f.flush(); }
    }
}

unsafe fn apply_one(p: &Patch) -> Result<String, String> {
    // rva 0 = 현재 게임버전에서 사이트 미해결(재핀 실패) → 패치 스킵. base+0 를 건드리지 않는다.
    if p.rva == 0 { return Ok("skip (0.5.3 미해결 사이트)".into()); }
    let base = exe_base(); if base == 0 { return Err("module 0".into()); }
    let n = p.orig.len();
    let addr = base + p.rva;
    if !readable(addr, n) { return Err(format!("addr unreadable @abs=0x{:x} base=0x{:x}", addr, base)); }
    let mut buf = [0u8; 8];
    core::ptr::copy_nonoverlapping(addr as *const u8, buf.as_mut_ptr(), n);
    let cur = &buf[..n];
    if cur == p.fixed { return Ok(format!("already @abs=0x{:x}", addr)); }   // 멱등
    if cur != p.orig { return Err(format!("byte mismatch @abs=0x{:x} cur={:02x?} want_orig={:02x?}", addr, cur, p.orig)); }
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(addr, n, RWX, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::copy_nonoverlapping(p.fixed.as_ptr(), addr as *mut u8, n);
    VirtualProtect(addr, n, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, n);
    // ★ write 후 재read 검증 — 실제 메모리에 박힌 바이트 확인(로그 patched인데 효과없음 진단용)
    let mut vbuf = [0u8; 8];
    core::ptr::copy_nonoverlapping(addr as *const u8, vbuf.as_mut_ptr(), n);
    let landed = &vbuf[..n];
    if landed == p.fixed {
        Ok(format!("patched+VERIFIED @abs=0x{:x} landed={:02x?}", addr, landed))
    } else {
        Err(format!("★write안박힘! @abs=0x{:x} landed={:02x?} want={:02x?}", addr, landed, p.fixed))
    }
}

// ── (5) dup 경고 디스패처 트램폴린 훅 ──────────────────────────────────────────
// 진짜 dup 게이트 = 검증기 B(0x1406057d0, 233KB 매프레임 갱신기)가 인덱스 디스패처
//   FUN_140cd1e60(rva 0xcd1e60)에 "메시지 인덱스 3(=duplicate_players)"을 넘겨 문구+버튼차단.
//   dup 문자열은 코드 LEA가 아니라 오프셋테이블 인덱스로 참조돼 바이트패치 대상이 안 됨.
//   → 디스패처를 훅해 r8d(인덱스)==3이면 즉시 return(원본 미실행) = dup 경고만 억제, 다른 경고(roster
//   /champion_required 등 idx 0,1,2,4)는 원본대로. 계약: rcx, rdx=컨텍스트, r8d=메시지 인덱스, r9.
//   프롤로그 12B(56 53 48 83 EC 28 44 89 C3 48 89 D1) = 온전한 명령경계 → 12B abs-jmp detour 안전.
const DISP_RVA: usize = 0; // ⬜0.5.3 미해결(구0.5.2=0xd3f780) — 0 이면 훅 설치 스킵.
// 콜러가 1개뿐이라 투표가 성립 안 하고, 문자열 오프셋 테이블 경로도 0.5.3 에서 재현 실패.
// ⚠12B push8 프롤로그 검증은 0.5.3 에서 66,635곳이 통과하므로 **오답을 못 거른다** ⟹ 추정값 금지. // 0.5.2(구0.5.1=0xc82370, L1-UNIQUE 스켈레톤·DISP_PROLOGUE 12B 실측 MATCH)
const DISP_PROLOGUE: [u8; 12] = [0x56, 0x53, 0x48, 0x83, 0xec, 0x28, 0x44, 0x89, 0xc3, 0x48, 0x89, 0xd1];
const DUP_MSG_INDEX: u32 = 3;
static DISP_TRAMP: AtomicUsize = AtomicUsize::new(0);
static DISP_SUPPRESS: AtomicU64 = AtomicU64::new(0);   // dup 억제 횟수(진단)

extern "win64" fn disp_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    // dup(idx 3) → 원본 디스패처 미실행(경고 세팅 스킵). detour 본문 패닉이 게임 콜스택으로
    //   unwind하면 UB → catch_unwind로 격리(여기선 순수 분기라 사실상 무해하나 방어).
    let out = std::panic::catch_unwind(|| {
        if (r8 as u32) == DUP_MSG_INDEX {
            DISP_SUPPRESS.fetch_add(1, Ordering::Relaxed);
            return 0usize;                       // dup 경고 억제
        }
        let stub = DISP_TRAMP.load(Ordering::Relaxed);
        if stub == 0 { return 0usize; }
        let f: extern "win64" fn(usize, usize, usize, usize) -> usize =
            unsafe { core::mem::transmute(stub) };
        f(rcx, rdx, r8, r9)                       // 다른 인덱스 = 원본 그대로
    });
    out.unwrap_or(0)
}

unsafe fn install_disp_hook() -> Result<String, String> {
    if DISP_RVA == 0 { return Ok("skip (0.5.3 RVA 미해결)".into()); }
    let base = exe_base(); if base == 0 { return Err("module 0".into()); }
    let fn_addr = base + DISP_RVA;
    if !readable(fn_addr, 12) { return Err(format!("disp unreadable @abs=0x{:x}", fn_addr)); }
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    if cur != DISP_PROLOGUE { return Err(format!("disp prologue mismatch @abs=0x{:x} cur={:02x?}", fn_addr, cur)); }
    // 트램폴린 stub: 훔친 12B + jmp fn+12
    let stub = VirtualAlloc(0, 64, 0x1000 | 0x2000, 0x40);
    if stub == 0 { return Err("VirtualAlloc".into()); }
    let mut s: Vec<u8> = Vec::with_capacity(24);
    s.extend_from_slice(&DISP_PROLOGUE);
    s.extend_from_slice(&[0x48, 0xb8]);                       // mov rax,
    s.extend_from_slice(&(fn_addr + 12).to_le_bytes());       //   fn+12
    s.extend_from_slice(&[0xff, 0xe0]);                       // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    DISP_TRAMP.store(stub, Ordering::Relaxed);
    // 원본에 abs-jmp detour (12B): mov rax,detour; jmp rax
    let d = disp_detour as usize;
    let mut patch = [0u8; 12];
    patch[0] = 0x48; patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&d.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok(format!("disp hook installed @abs=0x{:x} stub=0x{:x}", fn_addr, stub))
}

// ── (6) [진단] insert 훅 프로브 — HashSet 병합 실측 ──────────────────────────────
// set-insert(FUN_140cabac0, rva 0xcabac0)를 훅해 comp_test 서버핸들러 region
//   (0x13d4af0..0x1412575)에서 호출될 때 caller/rdx(id)/al(반환=중복여부) 로깅.
//   같은 athlete_id 반복 삽입에 al=1(이미 존재=병합)이 찍히면 HashSet<id> 하드리밋 확정.
//   프롤로그 14B(41 57 41 56 56 57 53 48 83 EC 30 48 89 D6) 온전 → 12B abs-jmp+2nop detour.
// ⛔INSERT_RVA = **죽은 상수(0.5.0_3부터 STALE, 0.5.2 미마이그)**. 0.5.1 exe 실측에서도 이 주소는 함수시작이
//   아니라 0xca6b20 함수 **중간**이고 INSERT_PROLOGUE와 불일치 ⇒ install_insert_probe가 프롤로그 검증에서
//   Err 반환 = **훅이 설치된 적 없음**(진단 로깅훅이라 무증상). 0.5.2에서도 해당 주소 바이트가 프롤로그와
//   불일치함을 확인(=오후킹 위험 없는 inert). ⇒ 재핀 대상이 아니라 **정리 대상**(필요해지면 ghidra-re).
const INSERT_RVA: usize = 0xcabac0; // ⬜미확정(죽은 상수·inert 확인필·0.5.1값 유지)
const INSERT_PROLOGUE: [u8; 14] = [0x41, 0x57, 0x41, 0x56, 0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x30, 0x48, 0x89, 0xd6];
// 진단 로깅 필터 range(동작 무영향). 0.5.2 재도출:
//   서버 = comp_test 서버핸들러 함수 범위. ~~0.5.0_3 0x13d4af0..0x1412575(2세대 STALE)~~ →
//     0.5.2 **0xe7ccd0..0xea2345**(= 0.5.1 0xf1d2c0 핸들러의 신주소 + pdata 크기 0x25675).
//   클라 = comp_test 클라 함수들이 모인 대략 영역. ~~0.5.0_3 0xf50000..0xf80000(0.5.1엔 이미 어긋나 있었음
//     — 0.5.1 RUN=0x161eab0은 이 범위 **밖**이라 in_client가 상시 false였다)~~ →
//     0.5.2 **0xcf0000..0xda0000**(RUN 0xd0a440·COLLECT 0xd0bd80·SLOT 0xd1acf0·LOADING 0xd186f0·
//     btn5v5 빌더 0xcf7970/0xd95450 전부 포함).
const CT_REGION_LO: usize = 0x216e870;   // ★0.5.5(구0.5.4=0x20d5bf0) 서버핸들러 .pdata 함수 시작(실측)       // 0.5.4(구0.5.3=0x17e0240)
const CT_REGION_HI: usize = 0x219775c;   // ★0.5.5(구0.5.4=0x20ff156) = .pdata 함수 끝(실측)       // 0.5.4(구0.5.3=0x180920f)
const CT_CLIENT_LO: usize = 0x1a80000;   // ★0.5.5(구0.5.4=0x2300000) 클라 사이트 0x1a8ab76~0x1afe394 포괄(소비처 insert_detour inert)       // 0.5.4(구0.5.3=0x18c0000)
const CT_CLIENT_HI: usize = 0x1b00000;   // ★0.5.5(구0.5.4=0x23e0000)       // 0.5.4(구0.5.3=0x19a0000)
static INSERT_TRAMP: AtomicUsize = AtomicUsize::new(0);
static INSERT_CALLER: AtomicUsize = AtomicUsize::new(0);   // shim이 [rsp](리턴주소) 저장
static PROBE_LOGS: AtomicU64 = AtomicU64::new(0);          // region 로깅 횟수(상한)
// ★id 위조(우회 실험): comp_test dedup insert의 athlete_id를 상위비트에 단조카운터 섞어 유니크화
//   → HashSet 병합 회피(len 증가) → 참가자수 게이트 통과 기대. 하위비트=원본 유지.
const FORGE_ENABLED: bool = true;
const ATH_ID_LO: usize = 0x100;       // athlete_id 범위(슬롯인덱스 0-4 제외)
const ATH_ID_HI: usize = 0x100000;
// ★위조 대상 caller 화이트리스트(RVA) — comp_test dedup insert 콜사이트만. 광역range 대신 정밀타겟
//   → comp_test 외 UI insert는 안 건드려 시작 크래시 회피. 프로브로 확인된 검증기A 콜사이트=0x1012b10.
//   (START 제출 경로 caller는 로그로 파악 후 추가 예정.)
// 0.5.2: ~~0.5.1 0x1615480~~ → 0xd00ed0. 이건 함수 시작이 아니라 **콜사이트 리턴주소**(insert_detour가
//   [rsp]로 잡는 caller). 컨테이너 0x1615030→0xd00a80이 L1-UNIQUE(바이트동일)라 델타 −0x9145b0이 정확하고,
//   실측 교차확인: OLD `call 0x140c5d910`@0x161547b(5B) → retaddr 0x1615480 / NEW `call 0x141177a10`@0xd00ecb
//   → retaddr 0xd00ed0. 그 callee(0xc5d910→0x1177a10)는 server_dedup_real이 호출하는 SwissSet insert와 동일 함수.
// ⚠단 이 값은 현재 **inert**: 소비처 insert_detour는 INSERT_RVA(죽은 상수)로 설치되므로 발화하지 않는다.
const FORGE_CALLERS: &[usize] = &[0xd00ed0]; // 0.5.2(구0.5.1=0x1615480, 컨테이너 L1-UNIQUE 델타·retaddr 실측확인)
static FORGE_CTR: AtomicU64 = AtomicU64::new(1);
static FORGE_HITS: AtomicU64 = AtomicU64::new(0);

#[unsafe(naked)]
unsafe extern "win64" fn insert_shim() {
    // [rsp]=caller 리턴주소. 전역 저장 후 detour_rust로 tail-jmp(인자 rcx/rdx/r8/r9 보존).
    core::arch::naked_asm!(
        "mov rax, [rsp]",
        "lea r10, [rip + {caller}]",
        "mov [r10], rax",
        "jmp {detour}",
        caller = sym INSERT_CALLER,
        detour = sym insert_detour_rust,
    );
}

extern "win64" fn insert_detour_rust(rcx: usize, rdx: usize, r8: usize, r9: usize) -> u8 {
    let out = std::panic::catch_unwind(|| {
        let caller = INSERT_CALLER.load(Ordering::Relaxed);
        let stub = INSERT_TRAMP.load(Ordering::Relaxed);
        if stub == 0 { return 0u8; }
        let f: extern "win64" fn(usize, usize, usize, usize) -> u8 = unsafe { core::mem::transmute(stub) };
        let base = unsafe { exe_base() };
        let rva = caller.wrapping_sub(base);
        let in_server = base != 0 && rva >= CT_REGION_LO && rva < CT_REGION_HI;
        let in_client = base != 0 && rva >= CT_CLIENT_LO && rva < CT_CLIENT_HI;
        let in_ct = in_server || in_client;
        // ★위조: comp_test dedup + athlete_id 범위면 상위비트에 유니크 카운터 → HashSet 병합 회피
        let mut id = rdx;
        let mut forged = false;
        if FORGE_ENABLED && in_ct && rdx >= ATH_ID_LO && rdx < ATH_ID_HI && FORGE_CALLERS.contains(&rva) {
            let ctr = FORGE_CTR.fetch_add(1, Ordering::Relaxed) & 0xffff;
            id = rdx | ((ctr as usize) << 40);
            forged = true;
            FORGE_HITS.fetch_add(1, Ordering::Relaxed);
        }
        let al = f(rcx, id, r8, r9);                     // 원본 insert (위조 id)
        if in_ct && PROBE_LOGS.fetch_add(1, Ordering::Relaxed) < 120 {
            log(&format!("[probe] caller=0x{:x} ({}) rdx=0x{:x}{} set=0x{:x} al={}\n",
                rva, if in_server { "srv" } else { "cli" }, rdx,
                if forged { format!("→0x{:x}", id) } else { String::new() }, rcx, al));
        }
        al
    });
    out.unwrap_or(0)
}

unsafe fn install_insert_probe() -> Result<String, String> {
    let base = exe_base(); if base == 0 { return Err("module 0".into()); }
    let fn_addr = base + INSERT_RVA;
    if !readable(fn_addr, 14) { return Err(format!("insert unreadable @abs=0x{:x}", fn_addr)); }
    let mut cur = [0u8; 14];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 14);
    if cur != INSERT_PROLOGUE { return Err(format!("insert prologue mismatch cur={:02x?}", cur)); }
    let stub = VirtualAlloc(0, 64, 0x1000 | 0x2000, 0x40);
    if stub == 0 { return Err("VirtualAlloc".into()); }
    let mut s: Vec<u8> = Vec::with_capacity(28);
    s.extend_from_slice(&INSERT_PROLOGUE);
    s.extend_from_slice(&[0x48, 0xb8]);
    s.extend_from_slice(&(fn_addr + 14).to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    INSERT_TRAMP.store(stub, Ordering::Relaxed);
    // 원본에 12B abs-jmp(mov rax,shim; jmp rax) + 2 nop = 14B
    let d = insert_shim as usize;
    let mut patch = [0u8; 14];
    patch[0] = 0x48; patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&d.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    patch[12] = 0x90; patch[13] = 0x90;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 14, 0x40, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 14);
    VirtualProtect(fn_addr, 14, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 14);
    Ok(format!("insert probe installed @abs=0x{:x} stub=0x{:x}", fn_addr, stub))
}

// ── (7) [진단] enqueue 훅 — 서버 진입 command 캡처 ────────────────────────────────
// START 제출 종단 enqueue(FUN_140cb9c80, rva 0xcb9c80)를 훅해 command 타입/포인터 로깅.
//   클라→서버(game_core) 경계. 여기서 command disc를 잡으면 서버 comp_test 핸들러(dedup) 특정 가능.
//   프롤로그 14B: 55 48 83 EC 30 48 8D 6C 24 30 48 C7 45 F8 (온전, rip-rel 없음).
// ⛔ENQ_RVA = **죽은 상수**(0.5.0_3부터 STALE·0.5.2 미마이그). 0.5.1 실측 = 0xca6b20 함수 중간이고
//   ENQ_PROLOGUE 불일치 ⇒ 설치된 적 없음. 0.5.2에서도 프롤로그 불일치 = inert(오후킹 위험 없음). 정리 대상.
const ENQ_RVA: usize = 0xcb9c80; // ⬜미확정(죽은 상수·inert 확인필·0.5.1값 유지)
// ★18B 온전 경계: 마지막 `48 C7 45 F8 FE FF FF FF`(mov [rbp-8],-2)는 8B 명령 → 14B로 자르면 imm32 잘려 크래시.
const ENQ_PROLOGUE: [u8; 18] = [0x55, 0x48, 0x83, 0xec, 0x30, 0x48, 0x8d, 0x6c, 0x24, 0x30, 0x48, 0xc7, 0x45, 0xf8, 0xfe, 0xff, 0xff, 0xff];
static ENQ_TRAMP: AtomicUsize = AtomicUsize::new(0);
static ENQ_HITS: AtomicU64 = AtomicU64::new(0);

extern "win64" fn enq_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    let out = std::panic::catch_unwind(|| {
        let stub = ENQ_TRAMP.load(Ordering::Relaxed);
        if stub == 0 { return 0usize; }
        // command 앞부분(disc/tag) 안전읽기: rcx,rdx가 command 포인터 후보
        let peek = |p: usize| -> u64 {
            unsafe { if p >= 0x10000 && readable(p, 8) { core::ptr::read_unaligned(p as *const u64) } else { 0 } }
        };
        if ENQ_HITS.fetch_add(1, Ordering::Relaxed) < 40 {
            log(&format!("[enq] rcx=0x{:x} rdx=0x{:x} r8=0x{:x} r9=0x{:x} [rcx]=0x{:x} [rdx]=0x{:x} [r8]=0x{:x}\n",
                rcx, rdx, r8, r9, peek(rcx), peek(rdx), peek(r8)));
        }
        let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
        f(rcx, rdx, r8, r9)
    });
    out.unwrap_or(0)
}

unsafe fn install_enq_hook() -> Result<String, String> {
    let base = exe_base(); if base == 0 { return Err("module 0".into()); }
    let fn_addr = base + ENQ_RVA;
    if !readable(fn_addr, 18) { return Err(format!("enq unreadable @abs=0x{:x}", fn_addr)); }
    let mut cur = [0u8; 18];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 18);
    if cur != ENQ_PROLOGUE { return Err(format!("enq prologue mismatch cur={:02x?}", cur)); }
    let stub = VirtualAlloc(0, 64, 0x1000 | 0x2000, 0x40);
    if stub == 0 { return Err("VirtualAlloc".into()); }
    let mut s: Vec<u8> = Vec::with_capacity(32);
    s.extend_from_slice(&ENQ_PROLOGUE);
    s.extend_from_slice(&[0x48, 0xb8]);
    s.extend_from_slice(&(fn_addr + 18).to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    ENQ_TRAMP.store(stub, Ordering::Relaxed);
    let d = enq_detour as usize;
    let mut patch = [0u8; 18];
    patch[0] = 0x48; patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&d.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    for b in &mut patch[12..18] { *b = 0x90; }              // 12B jmp + 6 nop = 18B
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 18, 0x40, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 18);
    VirtualProtect(fn_addr, 18, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 18);
    Ok(format!("enq hook installed @abs=0x{:x}", fn_addr))
}

// ── (8) [진단] run/서버 핸들러 진입 훅 — 제출이 서버까지 가는지 판정 ────────────────
// run 핸들러(제출) 0x101c030, 서버 핸들러(comp_test arm) 0x13d4af0. 둘 다 push8개 12B 프롤로그.
//   START시 서버 핸들러 발화 O = 제출 서버도달(서버 arm abort 범인) / X = 클라 멈춤(클라 게이트 범인).
const RUN_RVA: usize = 0x1aa2930;   // ★0.5.5(구0.5.4=0x231de30) 3방법(마스크시그 k=8 + 내부 3사이트 cont5 일치 + 콜그래프투표)·HOOK_PROLOGUE12 실측 MATCH // 0.5.4(구0.5.3=0x18f1180)
// ⛔SRV_RVA = **죽은 상수**(0.5.0_3부터 STALE). 어차피 `let _ = (SRV_RVA,...)`로 훅 비활성(크래시 방지).
//   0.5.1·0.5.2 모두 이 주소는 함수시작 아님 = inert. 참고: 0.5.2 서버핸들러 실주소는 CT_REGION_LO(0xe7ccd0).
const SRV_RVA: usize = 0x13d4af0; // ⬜미확정(죽은 상수·훅 비활성·0.5.1값 유지)
const HOOK_PROLOGUE12: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
// push 순서 변형(rbp가 뒤쪽) — 0xf794c0(카테고리→아이템id)이 이 패턴. 길이·경계는 동일해 12B 스틸 가능.
const HOOK_PROLOGUE12_ALT: [u8; 12] = [0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53];
static RUN_TRAMP: AtomicUsize = AtomicUsize::new(0);
static SRV_TRAMP: AtomicUsize = AtomicUsize::new(0);
static RUN_HITS: AtomicU64 = AtomicU64::new(0);
static SRV_HITS: AtomicU64 = AtomicU64::new(0);

// run 핸들러(START 클릭, UI 단일스레드): 발화 로깅 + 이 시점 SRV 누적카운트 스냅샷.
//   START 여러 번 눌러 로그의 srv_total 증가분을 보면 = 제출 후 서버 arm 발화 여부.
extern "win64" fn run_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    std::panic::catch_unwind(|| {
        let n = RUN_HITS.fetch_add(1, Ordering::Relaxed);
        let before = PUSH_HITS.load(Ordering::Relaxed);
        let stub = RUN_TRAMP.load(Ordering::Relaxed);
        if stub == 0 { return 0usize; }
        let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
        RUN_ACTIVE.store(true, Ordering::Relaxed);        // ★이 구간의 collect(0x161ed83 제출)만 치환 대상
        let ret = f(rcx, rdx, r8, r9);                   // 원본 run 핸들러 실행(내부서 push 도달시 PUSH_HITS++)
        RUN_ACTIVE.store(false, Ordering::Relaxed);
        let after = PUSH_HITS.load(Ordering::Relaxed);
        let hyb = HYBRID_HITS.load(Ordering::Relaxed);
        // ★comp_test 제출 = 하이브리드 활성화(이 시점부터 sim 조회 미러) + 슬롯0 기억 리셋
        FIRST_ATH.store(0, Ordering::Relaxed);
        HYBRID_ACTIVE.store(true, Ordering::Relaxed);
        // (15) v2 멀티발사 — 이 프레임 안에서 추가 N-1회 동기 제출(+회차별 시드 분기).
        //   ⚠원 클릭의 첫 제출은 위 f() 에서 이미 끝났고, 그 패킷 시드는 원본 그대로 둔다
        //     (원 경기 = 평소와 동일한 결과). 시드를 흔드는 건 추가분뿐.
        unsafe { watch_capture(rcx, rdx) };   // (20) 러너 폴링용 node 캡처
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            cd_probe("RUN ", rdx);            // (30) 경기 시작 시점의 ClientData 지문
        }));
        let runs = CONC_RUNS.load(Ordering::Relaxed);
        if CONC_ON && runs > 1 {
            if CONC_PROBE_ON { tickmap_reset(); }   // (v3) 이후 생기는 sim 스레드만 세기 위해
            // ★v8: 여기서 몰아 쏘지 않는다 — 예약만 하고, 기록 1건이 끝날 때마다 csend에서 1발씩.
            CONC_PENDING.store(runs - 1, Ordering::Relaxed);
            CONC_SHOT.store(0, Ordering::Relaxed);
            // (24) 새 배치 — 시드 추적 슬롯 초기화 후, **원본 클릭이 방금 제출한 패킷의 시드**를 등록.
            //   (원본 1발은 시드를 변조하지 않으므로 여기서 큐 꼬리를 읽어 그대로 잡는다.)
            match_reset();
            BATCH_REC.store(0, Ordering::Relaxed);     // ★v61 새 회차 = 기록 카운터 리셋
            BATCH_T0.store(now_ms(), Ordering::Relaxed);
            for i in 0..MAXR {
                M_FIN[i].store(0, Ordering::Relaxed);
                M_STARTLOG[i].store(0, Ordering::Relaxed);
                M_WIN[i].store(0, Ordering::Relaxed);
                M_LASTB[i].store(0, Ordering::Relaxed);
                M_LASTR[i].store(0, Ordering::Relaxed);
                M_LASTT[i].store(0, Ordering::Relaxed);
                M_IDLE[i].store(0, Ordering::Relaxed);
            }
            unsafe {
                if let (Some(len), Some(ptr)) = (rd_u64(rdx + 0x10), rd_u64(rdx + 0x08)) {
                    if len > 0 {
                        let elem = (ptr as usize).wrapping_add((len as usize - 1) * CMD_STRIDE);
                        if let Some(tag) = rd_u64(elem) {
                            if tag as usize == CMD_TAG {
                                if let Some(pkt) = rd_u64(elem + 0x10) {
                                    if let Some(s) = rd_u64(pkt as usize + PKT_SEED_OFF) {
                                        match_register(s);
                                        log(&format!("[score] 경기1 시드 0x{:x}(원본) 등록\n", s));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            RESULT_HITS.store(0, Ordering::Relaxed);
            MFORGE_HITS.store(0, Ordering::Relaxed);
            // ★PARALLEL_ON이면 **지금 이 프레임에 N−1발을 몰아서** 발사한다(진짜 병렬).
            //   결과가 몰려 도착해도 (25) 지연·재주입이 순서를 세워 전량 기록한다.
            //   OFF면 종전대로 "기록 완료마다 1발"(순차·검증됨).
            if par_on() {
                log(&format!("[conc] 클릭 [{}ms]: 총 {}경기 **병렬** 동시 발사\n", now_ms(), runs));
                unsafe {
                    for i in 1..runs {
                        if !conc_fire_one(rcx, rdx, i) { break; }
                    }
                }
                CONC_PENDING.store(0, Ordering::Relaxed);
            } else {
                log(&format!("[conc] 클릭 [{}ms]: 총 {}경기 예약(기록 완료마다 1발)\n", now_ms(), runs));
            }
        }
        if n < 30 { log(&format!("[run] hit#{} | push {}→{} ({}) | hybrid_prev={}\n",
            n, before, after, if after > before { "도달" } else { "미도달" }, hyb)); }
        ret
    }).unwrap_or(0)
}
// 서버 핸들러: ★파일IO 금지(병렬 sim 스레드서도 호출 → 멀티스레드 로그충돌 크래시). 원자카운트만.
extern "win64" fn srv_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    SRV_HITS.fetch_add(1, Ordering::Relaxed);
    let stub = SRV_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(rcx, rdx, r8, r9))).unwrap_or(0)
}

unsafe fn install_hook12(rva: usize, tramp: &AtomicUsize, detour: usize) -> Result<String, String> {
    // rva 0 = 현재 버전에서 미해결 → 미설치. ⚠0.5.3 에선 12B push8 프롤로그가 66,635곳에 있어
    //   프롤로그 검증이 오답을 못 거른다 ⟹ 확정 못 한 주소는 반드시 0 으로 두고 스킵할 것.
    if rva == 0 { return Ok("skip (RVA 미해결)".into()); }
    let base = exe_base(); if base == 0 { return Err("module 0".into()); }
    let fn_addr = base + rva;
    if !readable(fn_addr, 12) { return Err(format!("unreadable @abs=0x{:x}", fn_addr)); }
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    // push×8 = 12B 프롤로그. 순서 변형이 있어 허용 목록으로 검증(스틸 길이·명령경계는 동일).
    if cur != HOOK_PROLOGUE12 && cur != HOOK_PROLOGUE12_ALT {
        return Err(format!("prologue mismatch @0x{:x} cur={:02x?}", rva, cur));
    }
    let stub = VirtualAlloc(0, 32, 0x1000 | 0x2000, 0x40);
    if stub == 0 { return Err("VirtualAlloc".into()); }
    let mut s: Vec<u8> = Vec::with_capacity(24);
    s.extend_from_slice(&cur);      // ★상수가 아니라 실제 원본 12B(프롤로그 변형 대응)
    s.extend_from_slice(&[0x48, 0xb8]);
    s.extend_from_slice(&(fn_addr + 12).to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    tramp.store(stub, Ordering::Relaxed);
    let mut patch = [0u8; 12];
    patch[0] = 0x48; patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&detour.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok(format!("hook @abs=0x{:x} stub=0x{:x}", fn_addr, stub))
}

// ── (11) [진단] 로딩빌더 훅 = state 4(로딩)↑ 도달 판정 = 서버 accept 여부 ──────────
// 0x14102a2c0(로딩/결과 빌더)은 유일호출 0x14100b796에서 state==4/5일 때만 발화 → 발화=서버 accept(sim함).
//   단일 UI 스레드·read-only·setup 무영향. 발화 로그 있으면 accept, 없으면 서버 거부 확정.
//   프롤로그 15B(push8 + sub rsp,0x88) 온지 경계 → 12B abs-jmp + 3nop.
const LOADING_RVA: usize = 0; // ⬜0.5.3 확정도 미달(후보 0x18f6000, 역순도 20%) — 진단 훅이라 0=스킵 // 0.5.2(구0.5.1=0x162cf10, **L1-UNIQUE**·199 instr·LOADING_PROLOGUE 15B 실측 MATCH)
const LOADING_PROLOGUE: [u8; 15] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x56, 0x57, 0x53, 0x48, 0x81, 0xec, 0x88, 0x00, 0x00, 0x00];
static LOADING_TRAMP: AtomicUsize = AtomicUsize::new(0);
static LOADING_HITS: AtomicU64 = AtomicU64::new(0);

extern "win64" fn loading_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    ORACLE_GATE.store(true, Ordering::Relaxed);   // comp_test ACCEPT = oracle 캡처 스코프 ON
    let n = LOADING_HITS.fetch_add(1, Ordering::Relaxed);
    if n < 8 { log(&format!("[loading] ★ACCEPT #{} | hybrid_hits={} first_ath=0x{:x}\n",
        n, HYBRID_HITS.load(Ordering::Relaxed), FIRST_ATH.load(Ordering::Relaxed))); }
    let stub = LOADING_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(rcx, rdx, r8, r9))).unwrap_or(0)
}

// ── (13) [진단] sim 실행 본체 동시실행 프로브 (2026-08-08, 0.5.4) ─────────────────
// 목적: "조합테스트 N개 동시 실행" 선행 확증 3건을 훅 1개로 — ①comp_test RUN이 이 함수(매치
//   워커 스레드의 sim 실행 본체 0x237c030, 구 0x1a511a0 계열)를 타는지 ②호출 스레드가 매치마다
//   다른지(= 매치당 detached 스레드 구조 실증) ③인자 중 매치 리스트 len(RE상 6번째 인자 부근).
// 근거 = REPORT\tfm2_comptest_unlock\RE\2026-08-08_동시실행_구조규명.md (ghidra-re 0.5.4 실측).
// 안전: 발화 빈도 = 매치 단위(틱 아님)·로그는 앞 32회만. 프롤로그 = HOOK_PROLOGUE12 실측 MATCH
//   (exe 파일 덤프 55 41 57 41 56 41 55 41 54 56 57 53). 본문 catch_unwind + passthrough.
// ⚠프로브 빌드 전용 — 릴리스 전 CONC_PROBE_ON=false + LOG_ENABLED=false 복귀 (배포체크리스트).
const CONC_PROBE_ON: bool = false;   // 릴리스(진단 훅 8종 미설치). ⚠기능 훅은 CONC_ON 게이트로 분리됨
const SIMBODY_RVA: usize = 0x237c030;   // ⬜0.5.5 미확정(skeleton NONE=본문변경, CONC_PROBE_ON=false로 inert·install_hook12 자체검증 fail-safe·0.5.4값 유지) // 0.5.4 sim 실행 본체
static SIMBODY_TRAMP: AtomicUsize = AtomicUsize::new(0);
static SIMBODY_HITS: AtomicU64 = AtomicU64::new(0);

extern "win64" fn simbody_detour(a1: usize, a2: usize, a3: usize, a4: usize,
                                 a5: usize, a6: usize, a7: usize, a8: usize) -> usize {
    let n = SIMBODY_HITS.fetch_add(1, Ordering::Relaxed);
    if n < 32 {
        let tid = unsafe { GetCurrentThreadId() };
        log(&format!("[simbody] hit#{} [{}ms] tid={} run_hits={} a1=0x{:x} a2=0x{:x} a3=0x{:x} a4=0x{:x} a5=0x{:x} a6=0x{:x} a7=0x{:x} a8=0x{:x}\n",
            n, now_ms(), tid, RUN_HITS.load(Ordering::Relaxed), a1, a2, a3, a4, a5, a6, a7, a8));
    }
    let stub = SIMBODY_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        unsafe { core::mem::transmute(stub) };
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(a1, a2, a3, a4, a5, a6, a7, a8)))
        .unwrap_or(0);
    if n < 32 {
        let tid = unsafe { GetCurrentThreadId() };
        log(&format!("[simbody] done#{} [{}ms] tid={} ret=0x{:x}\n", n, now_ms(), tid, r));
    }
    r
}

// ── (14) [진단] 완주 폴러 + run_tick 스레드맵 (2026-08-08 2차 프로브, 0.5.4) ─────────
// 1차 프로브 결과: 0x237c030 = 부팅 시 8-워커 상주 풀(동시 8발화·tid 8개·a1=0..7·done 0건 =
//   워커 루프 미종료)로 판명 — 매치별 스폰이 아니라 큐 소비 구조. comp_test RUN 후 신규 발화 없음
//   ⟹ comp_test 경기가 "어느 스레드에서" 도는지 직접 관측 필요.
// 이번 훅 2개: ①완주 폴러 0x148a7c0(feedback.rs, 본경기 1회 완주 루프) 진입/이탈 로깅(cap 32)
//   ②run_tick 오라클 0x13b3150 = 스레드별 틱 카운터만(핫패스 — 로깅·락·할당 절대 금지,
//   16슬롯 정적 atomic 테이블). 덤프는 UI 스레드(post_update)에서 주기 출력.
const POLLER_RVA: usize = 0x148a7c0;    // ⬜0.5.5 미확정(skeleton NONE, CONC_PROBE_ON=false로 inert·fail-safe·0.5.4값 유지) // 완주 폴러
static POLLER_TRAMP: AtomicUsize = AtomicUsize::new(0);
static POLLER_HITS: AtomicU64 = AtomicU64::new(0);

extern "win64" fn poller_detour(a1: usize, a2: usize, a3: usize, a4: usize,
                                a5: usize, a6: usize) -> usize {
    let n = POLLER_HITS.fetch_add(1, Ordering::Relaxed);
    if n < 32 {
        let tid = unsafe { GetCurrentThreadId() };
        log(&format!("[poller] hit#{} [{}ms] tid={} run_hits={} a1=0x{:x} a2=0x{:x} a3=0x{:x} a4=0x{:x}\n",
            n, now_ms(), tid, RUN_HITS.load(Ordering::Relaxed), a1, a2, a3, a4));
    }
    let stub = POLLER_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize, usize, usize) -> usize =
        unsafe { core::mem::transmute(stub) };
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(a1, a2, a3, a4, a5, a6)))
        .unwrap_or(0);
    if n < 32 {
        log(&format!("[poller] done#{} [{}ms] tid={} ret=0x{:x}\n",
            n, now_ms(), unsafe { GetCurrentThreadId() }, r));
    }
    r
}

// run_tick(오라클) 스레드별 틱 카운터 — 16슬롯 고정 테이블(tid=0 빈칸). 핫패스라 로깅 없음.
const TZERO32: AtomicU32 = AtomicU32::new(0);
const TZERO64: AtomicU64 = AtomicU64::new(0);
// ★2026-08-08 v3: 16슬롯이 부팅 presim 8 + UI + 잡스레드로 **만석**이 되어 정작 comp_test
//   스레드가 테이블에 못 들어갔다(v2 회차에서 신규 sim 스레드 수를 세지 못한 원인). 64로 확대 +
//   RUN 클릭 시 리셋 ⟹ "클릭 이후 생긴 sim 스레드 수" = 실제로 돈 경기 수.
const TICK_SLOTS: usize = 64;
static TICK_TIDS: [AtomicU32; TICK_SLOTS] = [TZERO32; TICK_SLOTS];
static TICK_CNTS: [AtomicU64; TICK_SLOTS] = [TZERO64; TICK_SLOTS];
static TICK_TRAMP: AtomicUsize = AtomicUsize::new(0);

// RUN 클릭 시 호출(UI 스레드) — 테이블을 비워 이후 생성되는 sim 스레드만 관측한다.
//   동시에 다른 스레드가 tick_count 중일 수 있으나 전부 원자 store라 UB 없음(카운트만 잠깐 흔들림).
fn tickmap_reset() {
    for i in 0..TICK_SLOTS {
        TICK_TIDS[i].store(0, Ordering::Relaxed);
        TICK_CNTS[i].store(0, Ordering::Relaxed);
    }
    TICKDUMP_LAST.store(0, Ordering::Relaxed);
    TICKDUMP_N.store(0, Ordering::Relaxed);
}

fn tick_count(tid: u32) {
    for i in 0..TICK_SLOTS {
        let t = TICK_TIDS[i].load(Ordering::Relaxed);
        if t == tid { TICK_CNTS[i].fetch_add(1, Ordering::Relaxed); return; }
        if t == 0 {
            if TICK_TIDS[i].compare_exchange(0, tid, Ordering::Relaxed, Ordering::Relaxed).is_ok()
                || TICK_TIDS[i].load(Ordering::Relaxed) == tid {
                TICK_CNTS[i].fetch_add(1, Ordering::Relaxed); return;
            }
        }
    }
    // 테이블 만석 = 관측 포기(드롭) — 핫패스라 어떤 fallback 작업도 하지 않음
}

// ── (24) 실시간 킬스코어 + sim 소요시간 계측 (2026-08-08 v13) ────────────────────────
// 근거 = RE\2026-08-08_실시간_킬스코어_판정.md. ★comp_test는 **진짜 틱 시뮬**을 전용 스레드에서
//   돌린다(오전의 "클라 동기 생성" 판정은 오독 — `0x235bf20`은 리플레이용 Game 셋업이었다).
//   run_tick(`0x13b3150`)의 인자 rdx = ctrl ⟹ `game = *(ctrl+0x1dc0)`,
//   **킬 스코어 = game+0xeb38(팀0/blue) / game+0xeb40(팀1/red)**, 진행 틱 = game+0xeb30, 시드 = +0xeb28.
// ⚠이 detour는 **sim 스레드**에서 3만+회 돈다 — alloc·lock·format!·파일IO 절대 금지, 원시 read +
//   원자 store만. 로깅은 UI 스레드(post_update)에서 값이 변할 때만.
// 목적 2가지: ①실시간 표시가 실효 있는지 가를 **sim 벽시계 소요시간** 실측 ②스코어 배선 검증.
const G_OFF: usize = 0x1dc0;        // ctrl → game (0.5.5 불변)
// ★0.5.5(2026-08-12): provider(=game) 대역 +0x168 균일 시프트(serpen 세션 실측 + ghidra-re 명령확정).
//   킬증가 @0.5.4 0x140982d↔0.5.5 0x14fe3aa `inc [rdx+0xeca0]/[rdx+0xeca8]` + 동시read @0x14ed5f3 등 5쌍 일관.
//   game(ctrl+0x1dc0)=serpen provider 동일 구조체. SEED+0x10=팀0·SEED+0x18=팀1 상대배치 유지.
const G_SCORE0: usize = 0xeca0;     // ★0.5.5(구0.5.4=0xeb38) 킬스코어 팀0/blue
const G_SCORE1: usize = 0xeca8;     // ★0.5.5(구0.5.4=0xeb40) 킬스코어 팀1/red
const G_TICK: usize = 0xec98;       // ★0.5.5(구0.5.4=0xeb30) 진행 틱
const G_SEED: usize = 0xec90;       // ★0.5.5(구0.5.4=0xeb28) 시드(경기 식별키)
// ★v14 정정 — **"RUN 직후 처음 도는 sim = 내 경기"는 틀렸다.** 실측(2026-08-08): 게임은 배경에서
//   리그 경기 등 **다른 sim을 상시 돌린다**(한 번의 조합테스트 실행에 경기 종료 로그가 수십 건).
//   게다가 경기가 끝난 game을 계속 읽어 `최종 3 : 1871880210880 · 1875663913472틱` 같은
//   **stale 쓰레기 값**까지 나왔다.
// ⟹ 정확한 식별키 = **시드**. 우리는 발사한 요청의 시드를 **알고 있다**(원본 1발 + 변조 4발).
//   `game+0xeb28`이 그 목록에 있을 때만 내 경기로 인정하면 오염이 원천 차단되고,
//   **병렬 N경기도 슬롯별로 동시 추적**할 수 있다(실시간 스코어 N개 = 유저 원안).
const MAXR: usize = 10;   // 시드 추적 슬롯 = 최대 경기 수와 일치해야 함
const AZ64: AtomicU64 = AtomicU64::new(0);
static M_SEED: [AtomicU64; MAXR] = [AZ64; MAXR];   // 0 = 빈 슬롯
static M_BLUE: [AtomicU64; MAXR] = [AZ64; MAXR];
static M_RED: [AtomicU64; MAXR] = [AZ64; MAXR];
static M_TICK: [AtomicU64; MAXR] = [AZ64; MAXR];
static M_T0: [AtomicU64; MAXR] = [AZ64; MAXR];
static M_T1: [AtomicU64; MAXR] = [AZ64; MAXR];
static M_N: AtomicU64 = AtomicU64::new(0);

// 발사한 요청의 시드를 등록(원본 클릭분 + 변조분). UI 스레드에서만 호출.
fn match_register(seed: u64) {
    if seed == 0 { return; }
    let n = M_N.load(Ordering::Relaxed) as usize;
    if n >= MAXR { return; }
    M_SEED[n].store(seed, Ordering::Relaxed);
    M_BLUE[n].store(0, Ordering::Relaxed);
    M_RED[n].store(0, Ordering::Relaxed);
    M_TICK[n].store(0, Ordering::Relaxed);
    M_T0[n].store(0, Ordering::Relaxed);
    M_T1[n].store(0, Ordering::Relaxed);
    M_N.store(n as u64 + 1, Ordering::Relaxed);
}
fn match_reset() {
    for i in 0..MAXR { M_SEED[i].store(0, Ordering::Relaxed); }
    M_N.store(0, Ordering::Relaxed);
}

extern "win64" fn tick_detour(a1: usize, a2: usize, a3: usize, a4: usize,
                              a5: usize, a6: usize) -> usize {
    let tid = unsafe { GetCurrentThreadId() };
    tick_count(tid);
    // (24) 이 sim이 **내가 발사한 경기**인지 시드로 판별하고, 맞으면 그 슬롯을 갱신한다.
    let n = M_N.load(Ordering::Relaxed) as usize;
    if n > 0 {
        unsafe {
            if readable(a2 + G_OFF, 8) {
                let g = core::ptr::read_unaligned((a2 + G_OFF) as *const u64) as usize;
                if g > 0x10000 && readable(g + G_SCORE1, 8) {
                    let seed = core::ptr::read_unaligned((g + G_SEED) as *const u64);
                    for i in 0..n.min(MAXR) {
                        if M_SEED[i].load(Ordering::Relaxed) == seed {
                            M_BLUE[i].store(core::ptr::read_unaligned((g + G_SCORE0) as *const u64), Ordering::Relaxed);
                            M_RED[i].store(core::ptr::read_unaligned((g + G_SCORE1) as *const u64), Ordering::Relaxed);
                            M_TICK[i].store(core::ptr::read_unaligned((g + G_TICK) as *const u64), Ordering::Relaxed);
                            let t = now_ms();
                            let _ = M_T0[i].compare_exchange(0, t, Ordering::Relaxed, Ordering::Relaxed);
                            M_T1[i].store(t, Ordering::Relaxed);
                            break;
                        }
                    }
                }
            }
        }
    }
    let stub = TICK_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize, usize, usize) -> usize =
        unsafe { core::mem::transmute(stub) };
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(a1, a2, a3, a4, a5, a6)))
        .unwrap_or(0)
}

// (24) UI 스레드 전용 — 내 경기들의 스코어 변화·완주를 로깅(sim 스레드에서는 절대 로깅 금지).
static M_LASTB: [AtomicU64; MAXR] = [AZ64; MAXR];
static M_LASTR: [AtomicU64; MAXR] = [AZ64; MAXR];
static M_LASTT: [AtomicU64; MAXR] = [AZ64; MAXR];   // ★완주 판정은 **틱** 기준(v15 수정)
static M_IDLE: [AtomicU64; MAXR] = [AZ64; MAXR];
static M_FIN: [AtomicU64; MAXR] = [AZ64; MAXR];     // 1 = 종료 로그 이미 출력
static M_STARTLOG: [AtomicU64; MAXR] = [AZ64; MAXR];   // 1 = sim 시작 시각 로그 완료
// ★승패(확정) — 결과 엔트리 `+0xD9` bool: **1=블루 승 / 0=레드 승**(무승부 표현 없음).
//   근거: 기록탭 카드 렌더 `0x2312a09`가 이 바이트로 `blue_win`/`red_win` 문자열과 색을 고른다
//   (`or rbx,0x2e`로 문자열 길이를 46↔47로 만드는 트릭 = 값이 엄격히 0/1인 증거).
//   ⚠킬 스코어(`+0x78`/`+0x80`)로 승패를 판정하면 안 된다 — 유저 실측으로 불일치 확인됨.
//   슬롯 값: 0=미확정 / 1=레드 승 / 2=블루 승
const E_SEED_OFF: usize = 0xa8;
const E_BLUEWIN_OFF: usize = 0xd9;
static M_WIN: [AtomicU64; MAXR] = [AZ64; MAXR];
static BATCH_T0: AtomicU64 = AtomicU64::new(0);        // 클릭 시각(상대 시각 계산용)
static KS_LOGS: AtomicU64 = AtomicU64::new(0);

fn killscore_tick() {
    let n = M_N.load(Ordering::Relaxed) as usize;
    for i in 0..n.min(MAXR) {
        let seed = M_SEED[i].load(Ordering::Relaxed);
        if seed == 0 || M_FIN[i].load(Ordering::Relaxed) != 0 { continue; }
        let t0 = M_T0[i].load(Ordering::Relaxed);
        if t0 == 0 { continue; }                     // 아직 sim이 시작되지 않은 슬롯
        // ★경기별 **sim 시작 절대시각**을 1회 기록 — 이게 병렬/순차의 결정적 증거다.
        //   5경기의 시작 시각이 서로 몇 ms 이내면 병렬, 각각 5~6초씩 벌어지면 순차.
        if M_STARTLOG[i].swap(1, Ordering::Relaxed) == 0 {
            log(&format!("[score] ★경기{} sim 시작 [{}ms] (클릭 후 +{}ms)\n",
                i + 1, t0, t0.saturating_sub(BATCH_T0.load(Ordering::Relaxed))));
        }
        let (b, r) = (M_BLUE[i].load(Ordering::Relaxed), M_RED[i].load(Ordering::Relaxed));
        let tick = M_TICK[i].load(Ordering::Relaxed);
        let t1 = M_T1[i].load(Ordering::Relaxed);
        // 스코어가 바뀌면 한 줄 (표시용)
        if b != M_LASTB[i].swap(b, Ordering::Relaxed) || r != M_LASTR[i].swap(r, Ordering::Relaxed) {
            if KS_LOGS.fetch_add(1, Ordering::Relaxed) < 120 {
                log(&format!("[score] 경기{} {} : {} (틱 {} · +{}ms)\n",
                    i + 1, b, r, tick, t1.saturating_sub(t0)));
            }
        }
        // ★완주 판정 = **틱이 멈췄는가**. v14는 이걸 "스코어 변화"로 했다가, 초반 1초간 킬이 없는
        //   경기(5건 중 4건)를 `0:0`으로 조기 종료 처리해버렸다 — 킬은 몇 초씩 안 날 수 있지만
        //   틱은 sim이 도는 한 매번 증가하므로 이쪽이 올바른 신호다.
        if tick != M_LASTT[i].swap(tick, Ordering::Relaxed) {
            M_IDLE[i].store(0, Ordering::Relaxed);
            continue;
        }
        if M_IDLE[i].fetch_add(1, Ordering::Relaxed) + 1 > 60 {
            M_FIN[i].store(1, Ordering::Relaxed);
            log(&format!("[score] ★경기{} 종료 — 최종 {} : {} · 총 {}틱 · **{}ms** (시드 0x{:x})\n",
                i + 1, b, r, tick, t1.saturating_sub(t0), seed));
        }
    }
}

// 틱맵 주기 덤프(UI 스레드 전용 — post_update에서 호출)
static TICKDUMP_T: AtomicU64 = AtomicU64::new(0);
static TICKDUMP_N: AtomicU64 = AtomicU64::new(0);
static TICKDUMP_LAST: AtomicU64 = AtomicU64::new(0);

fn tickmap_dump_periodic() {
    let t = TICKDUMP_T.fetch_add(1, Ordering::Relaxed);
    if t % 600 != 599 { return; }                      // ~10초마다
    if TICKDUMP_N.load(Ordering::Relaxed) >= 20 { return; }
    let total: u64 = (0..TICK_SLOTS).map(|i| TICK_CNTS[i].load(Ordering::Relaxed)).sum();
    if total == TICKDUMP_LAST.swap(total, Ordering::Relaxed) { return; }  // 변화 없으면 침묵
    TICKDUMP_N.fetch_add(1, Ordering::Relaxed);
    let live = (0..TICK_SLOTS).filter(|&i| TICK_TIDS[i].load(Ordering::Relaxed) != 0).count();
    let mut s = format!("[tickmap] [{}ms] run_hits={} forge={} 스레드={}개 |", now_ms(),
        RUN_HITS.load(Ordering::Relaxed), MFORGE_HITS.load(Ordering::Relaxed), live);
    for i in 0..TICK_SLOTS {
        let tid = TICK_TIDS[i].load(Ordering::Relaxed);
        if tid == 0 { continue; }
        s.push_str(&format!(" tid{}={}", tid, TICK_CNTS[i].load(Ordering::Relaxed)));
    }
    s.push('\n');
    log(&s);
}

// ── (15) RUN 멀티발사 = 조합테스트 동시 N경기 (2026-08-08, 0.5.4) ─────────────────
// 근거(프로브 실측 = RE\2026-08-08 부록 A): comp_test 경기는 요청당 전용 스레드에서 인라인
//   실행(tid 32796·33,808틱 실측) ⟹ 요청 N개 = 스레드 N개 = 자연 동시 실행. 서버측 "실행 중
//   거부" 게이트 없음(정적). 남은 개입 = 클라 제출을 N회로 늘리는 것뿐.
// 방식: RUN 클릭 1회(원본 1발) 후 post_update(UI 스레드)에서 같은 인자로 원본 핸들러를
//   N-1회 재호출. ★재발사 간격 = 2프레임(~33ms) — 클라 시드 성분에 epoch_ms가 들어가므로
//   (0.5.3 dr_inline_c 규명: seed=(used|X<<32)^epoch_ms) 간격만 벌리면 매 경기 시드 자동 상이.
// 안전: 재발사는 트램폴린 직접 호출(run_detour 재진입 없음 = RUN_HITS 오염 없음), UI 스레드
//   전용(클릭 핸들러와 동일 스레드), catch_unwind + RUN_ACTIVE 스코프는 원본 클릭과 동일하게 재현.
// ★v2(2026-08-08 오후): v1(post_update 재발사) 폐기 — 4발 전부 ret=1인데 경기 1건(기록 탭 확증).
//   RE(RE\2026-08-08_RUN재제출_차단원인.md)로 원인 확정:
//   ①`ret=1`은 판별 불가(함수 출구가 `0x231e265 mov al,1` 단 하나 — 성공·실패 전부 합류).
//   ②제출 = 소켓이 아니라 **호출자가 준 커맨드 Vec(param_2=RDX)에 push**(원소 0x2120,
//     `Vec{cap@0x0, ptr@0x8, len@0x10}`). post_update 시점의 rdx는 이미 drain된 1회용 Vec일
//     가능성이 높아 push해도 아무도 안 읽음(조용한 소실) = v1 실패의 유력 원인.
//   ③★시드 = `(used_today | game[0xe3b8]<<32) ^ game_time_ms`, **패킷 +0x68**. 같은 프레임에서
//     N회 호출하면 **시드가 전부 동일** ⟹ 그냥 N번 부르면 똑같은 경기 5판이 나온다. 시드 변조 필수.
//   ⟹ v2 = **detour 본문에서 동기 N회 루프**(원 호출자 프레임 = Vec 100% 유효) + 회차별 시드 XOR.
//     계측도 내장: orig 전후 `*(rdx+0x10)` 델타가 곧 "이 회차가 제출됐는가"의 정답(새 훅 불요).
const CONC_ON: bool = true;
// RUN 1클릭당 실행할 총 경기 수. **`comptest_items.cfg`의 `runs = N`로 지정**(1~10, 기본 5).
//   1이면 기능 OFF와 동일(바닐라 동작). cfg는 게임 시작 시 1회 로드 = 변경하면 재시작 필요.
static CONC_RUNS: AtomicU64 = AtomicU64::new(5);
// ★상한 이력: 10 → 20(요청) → **10으로 복귀**(2026-08-08 유저 실측 "20경기는 겁나 느려진다").
//   20 동시 sim은 CPU를 다 먹어 경기당 시간이 크게 늘어난다. 10이 실용 상한.
const CONC_RUNS_MAX: u64 = 10;
// ★v62 — 병렬/순차 **런타임 전환**(`comptest_items.cfg`의 `parallel = 0|1`, 기본 1).
//   왜: "리플레이 불일치가 동시 실행 때문인가"를 가르려면 같은 빌드로 두 모드를 비교해야 한다.
//   순차(0) = 기록 1건 끝날 때마다 1발(동시 sim 없음). 병렬(1) = 클릭 시 N발 동시.
static PAR_RT: AtomicU64 = AtomicU64::new(1);
#[inline] fn par_on() -> bool { PARALLEL_ON && PAR_RT.load(Ordering::Relaxed) != 0 }
const CMD_STRIDE: usize = 0x2120;      // 커맨드 Vec 원소 크기
const CMD_TAG: usize = 0x16;           // 커맨드 tag(검증용)
const PKT_DISC: usize = 0x1c;          // 패킷 discriminant = comp_test RUN(검증용)
const PKT_SEED_OFF: usize = 0x68;      // 패킷 내 seed(u64) 위치
static CONC_SALT: AtomicU64 = AtomicU64::new(0);

#[inline] unsafe fn rd_u64(addr: usize) -> Option<u64> {
    if !readable(addr, 8) { return None; }
    Some(core::ptr::read_unaligned(addr as *const u64))
}

// 회차별 시드 분기값 — 세션마다 달라지도록 now_ms를 1회 섞고, 회차는 홀수 곱으로 흩는다.
fn conc_salt(i: u64) -> u64 {
    let base = CONC_SALT.load(Ordering::Relaxed);
    let base = if base != 0 { base } else {
        let b = (now_ms() as u64).wrapping_mul(0x9E3779B97F4A7C15) | 1;
        CONC_SALT.store(b, Ordering::Relaxed); b
    };
    base.wrapping_mul(i.wrapping_mul(2).wrapping_add(1)).rotate_left((i % 61) as u32 + 1)
}

// ★v8 전환(2026-08-08): "클릭 1회에 5발 동시 발사" → **"기록 1건이 끝날 때마다 1발씩 순차 발사"**.
//   이유 = 5발을 몰아 보내면 서버 응답 5건이 **같은 프레임에 몰려 도착**하고, 클라 드레인 루프가 그걸
//   한 프레임에 전부 소비하는 사이 팝업 draw가 끼지 못해 결과 슬롯이 계속 덮어써진다(4건 소실).
//   순차로 보내면 매 왕복이 stock 동작 그대로 완결된다(도착→draw→기록→히스토리 저장) = 소실 원천 차단.
//   ⟹ 신규 훅 0개, 깊은 스택 함수 미접촉(v6 크래시 원인 회피).
static CONC_PENDING: AtomicU64 = AtomicU64::new(0);
static CONC_SHOT: AtomicU64 = AtomicU64::new(0);
// ★v61 — **이번 회차에 클라가 기록을 몇 번 냈는가**(배치마다 0으로 리셋).
//   v60까지 결과화면 진입 조건이 `HPUSH_HITS`(=서버 저장 수)에 걸려 있었는데, 서버 저장은 **비동기**라
//   마지막 csend가 그 직전에 들어오면 조건이 1 모자라 영영 성립하지 않는다(v60 실측: rview 0회 발화
//   → 화면이 넘어가지 않음). 클라 기록 수는 동기·정확하므로 이쪽을 판정에 쓴다.
static BATCH_REC: AtomicU64 = AtomicU64::new(0);

// RUN 1발 발사 + 그 회차 시드 변조. 성공(제출됨) 시 true.
//   node/cmdVec은 **살아있는 프레임의 인자**만 넘길 것(캐시한 포인터는 v1에서 조용한 소실로 실패).
unsafe fn conc_fire_one(node: usize, cmdvec: usize, i: u64) -> bool {
    let stub = RUN_TRAMP.load(Ordering::Relaxed);
    if stub == 0 || !readable(cmdvec, 0x18) { return false; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = core::mem::transmute(stub);
    let before = match rd_u64(cmdvec + 0x10) { Some(v) => v, None => return false };
    RUN_ACTIVE.store(true, Ordering::Relaxed);
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(node, cmdvec, 0, 0)));
    RUN_ACTIVE.store(false, Ordering::Relaxed);
    FIRST_ATH.store(0, Ordering::Relaxed);
    HYBRID_ACTIVE.store(true, Ordering::Relaxed);
    let after = match rd_u64(cmdvec + 0x10) { Some(v) => v, None => return false };
    if after <= before {
        log(&format!("[conc] 발사 #{} 미제출(len {}→{}) — 클라 사전게이트 컷\n", i, before, after));
        return false;
    }
    // 방금 push된 커맨드의 패킷 시드를 회차별로 분기
    let ptr = match rd_u64(cmdvec + 0x08) { Some(v) => v as usize, None => return true };
    let elem = ptr.wrapping_add((after as usize - 1).wrapping_mul(CMD_STRIDE));
    let mut seed_note = String::from("seed=skip");
    if let Some(tag) = rd_u64(elem) {
        if tag as usize == CMD_TAG {
            if let Some(pkt) = rd_u64(elem + 0x10) {
                let pkt = pkt as usize;
                if let Some(disc) = rd_u64(pkt) {
                    if disc as usize == PKT_DISC {
                        let sa = pkt + PKT_SEED_OFF;
                        if let Some(old) = rd_u64(sa) {
                            let new = old ^ conc_salt(i);
                            let mut prot: u32 = 0;
                            if VirtualProtect(sa, 8, 0x04, &mut prot) != 0 {
                                core::ptr::write_unaligned(sa as *mut u64, new);
                                VirtualProtect(sa, 8, prot, &mut prot);
                                seed_note = format!("seed 0x{:x}→0x{:x}", old, new);
                                match_register(new);   // (24) 이 시드로 도는 sim = 내 경기
                            }
                        }
                    }
                }
            }
        }
    }
    log(&format!("[conc] 발사 #{} 제출 OK (len {}→{}) {}\n", i, before, after, seed_note));
    true
}

// 기록 1건이 끝난 직후(state==5) 다음 1발 — csend_detour 말미에서 호출.
//   csend 계약: rcx=팝업ctx, rdx=node, r8=&cmdVec ⟹ RUN에 넘길 (node, cmdVec)이 그대로 살아있다.
unsafe fn conc_next_shot(node: usize, cmdvec: usize) {
    if !CONC_ON || CONC_RUNS.load(Ordering::Relaxed) <= 1 { return; }
    if CONC_PENDING.load(Ordering::Relaxed) == 0 { return; }
    let runner = match runner_of(node) { Some(r) => r, None => return };
    if runner_state(runner) != 5 { return; }            // 아직 기록 완료 전이면 대기
    let i = CONC_SHOT.fetch_add(1, Ordering::Relaxed) + 1;
    if conc_fire_one(node, cmdvec, i) {
        let left = CONC_PENDING.fetch_sub(1, Ordering::Relaxed) - 1;
        log(&format!("[conc] 남은 발사 예정={}\n", left));
    } else {
        CONC_PENDING.store(0, Ordering::Relaxed);       // 게이트 컷이면 더 시도하지 않음
        log("[conc] 순차 발사 중단(게이트 컷)\n");
    }
}

// ── (16) [진단] 클라 사전게이트 + 서버 등록루프 반환코드 (2026-08-08 v2 동반) ─────────
// 왜: v1 실패가 (A)큐 소실인지 (B)클라 게이트 컷인지, 서버까지 갔다면 왜 거부인지를 한 판에 가른다.
//   근거·반환코드표 = RE\2026-08-08_RUN재제출_차단원인.md.
// ①CGATE 0x2310a90 (RUN 진입 첫 분기): 1=통과 / 2=state downcast 실패 / 0=4사유(일일한도·
//   인원부족·중복·챔피언미지정). UI 스레드 전용. 프롤로그 12B push8 실측 MATCH.
// ②SREG 0x2126b00 (서버 등록루프, `server_roster_min`·`server_dedup_real`의 소속 함수):
//   반환 AL 0xff=성공 / 4=챔피언 / 3=요청 내 중복 / 2=로스터부족 / 0=조회실패.
//   ⚠서버 스레드에서 호출 = 멀티스레드 — 로그는 앞 32회만, 본문은 catch_unwind.
//   **호출 횟수 = 서버 도달 건수** ⟹ 제출 5건 중 몇 건이 서버에 닿았는지 직접 카운트.
const CGATE_RVA: usize = 0x1a95570;   // ★0.5.5(구0.5.4=0x2310a90) 마스크시그 k=12 + dr_inline_b/allow_dup 컨테이너·push8 확인
const SREG_RVA: usize = 0x21bee60;    // ★0.5.5(구0.5.4=0x2126b00) skeleton UNIQUE size1400(§7.5 일치)
static CGATE_TRAMP: AtomicUsize = AtomicUsize::new(0);
static SREG_TRAMP: AtomicUsize = AtomicUsize::new(0);
static CGATE_HITS: AtomicU64 = AtomicU64::new(0);
static SREG_HITS: AtomicU64 = AtomicU64::new(0);

extern "win64" fn cgate_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    // ★팝업 노드를 여기서 캡처한다. `0x2310a90`은 팝업이 열려 있는 동안 자주 호출되므로
    //   **경기를 돌리기 전에도** node를 확보할 수 있다(csend는 기록 시에만 발화해서, v37은
    //   경기를 한 번 돌리기 전엔 화면 판정이 안 돼 −/+ 박스가 아예 안 떴다).
    unsafe { if runner_of(rcx).is_some() { RV_NODE.store(rcx, Ordering::Relaxed); } }
    let stub = CGATE_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(rcx, rdx, r8, r9))).unwrap_or(0);
    let n = CGATE_HITS.fetch_add(1, Ordering::Relaxed);
    if n < 32 {
        let al = r & 0xff;
        let why = match al { 1 => "통과", 2 => "state downcast 실패", 0 => "거부(일일/인원/중복/챔피언)", _ => "?" };
        log(&format!("[cgate] #{} al={} ({})\n", n, al, why));
    }
    r
}

extern "win64" fn sreg_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    let stub = SREG_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(rcx, rdx, r8, r9))).unwrap_or(0);
    let n = SREG_HITS.fetch_add(1, Ordering::Relaxed);
    if n < 32 {
        let al = r & 0xff;
        let why = match al { 0xff => "성공(run 진입)", 4 => "챔피언 불일치", 3 => "요청 내 중복",
                             2 => "로스터 부족", 0 => "선수 조회 실패", _ => "?" };
        log(&format!("[sreg] #{} al=0x{:x} ({}) team={}\n", n, al, why, rdx as i64));
    }
    r
}

// ── (17) [진단] 경기 형성 함수 = 5건 중 어디서 죽는지 다음 관문 (2026-08-08 v3) ─────────
// v2 실측: 제출 5/5 성공(len 1→5·시드 4종 변조 확인) + **서버 등록루프 5/5 al=0xff(성공)**.
//   그런데 기록 탭 경기 1건 ⟹ 병목은 등록 **이후**. 서버 성공 파이프라인 =
//   일일게이트(0x20e5428) → **경기형성 `0x2123590`**(실패 시 -1 → 거부응답 0x20eafbc) →
//   inc_gate(0x20e8247) → 응답 0x56. ⟹ 경기형성 호출 횟수·반환값이 다음 판별점.
// 프롤로그 12B push8 실측 MATCH. 서버 스레드 호출 = 로그 cap + catch_unwind.
// ⚠이름 주의: 기존 `FORGE_*`(0xd00ed0 계열)는 **비활성 insert 프로브**의 것 — 무관하다.
//   이쪽은 경기 형성(match forge) = `MFORGE_*`.
const MFORGE_RVA: usize = 0x21bb930;   // ★0.5.5(구0.5.4=0x2123590) skeleton UNIQUE size3101 (CONC_PROBE inert)
static MFORGE_TRAMP: AtomicUsize = AtomicUsize::new(0);
static MFORGE_HITS: AtomicU64 = AtomicU64::new(0);

extern "win64" fn mforge_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    let stub = MFORGE_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(rcx, rdx, r8, r9))).unwrap_or(0);
    let n = MFORGE_HITS.fetch_add(1, Ordering::Relaxed);
    if n < 32 {
        // ★정정(2026-08-08): 이 함수는 **void** 다 — 성공/실패는 반환값이 아니라 **out 파라미터**
        //   `rcx`(0x30 구조체)의 첫 qword로 온다(`out[0]==-1` = 형성 실패 = 응답 미송신).
        //   구 로그의 `ret=1`은 rax 쓰레기값이었고 "형성 OK"의 근거가 아니었다(무효 계측).
        let out0 = unsafe {
            if readable(rcx, 8) { core::ptr::read_unaligned(rcx as *const i64) } else { 0 }
        };
        let ok = if out0 == -1 { "❌형성 실패(None)" } else { "✅형성 OK" };
        log(&format!("[forge] #{} out[0]=0x{:x} ({}) tid={}\n", n, out0, ok, unsafe { GetCurrentThreadId() }));
    }
    r
}

// ── (21) [진단] 클라 결과 생성 카운터 = "결과가 실제로 몇 건 만들어졌나" ────────────────
// `0x235b270` = 클라 수신 핸들러 `0xa15e20`이 결과 1건당 정확히 1회 부르는 엔트리 조립 함수.
//   ⟹ **hits = 클라가 실제로 만든 결과 개수**(폴링과 달리 같은 프레임 다중 도착도 전부 센다).
// ★v7 폴링의 "도착 1회"는 **착시일 수 있다**: 클라 드레인 루프가 한 프레임에 응답을 전부 소비하면
//   결과 생성이 한 프레임에 N회 일어나는데, 프레임당 1회 폴링은 그것을 전이 1회로 접어버린다.
//   이 카운터가 그 모호함을 없앤다. 프롤로그 12B push8 + 프레임 0xb8(**__chkstk 없음**) = 훅 안전.
const RESULT_RVA: usize = 0x1aec180;   // ★0.5.5(구0.5.4=0x235b270) skeleton UNIQUE size540 (CONC_PROBE inert)
static RESULT_TRAMP: AtomicUsize = AtomicUsize::new(0);
static RESULT_HITS: AtomicU64 = AtomicU64::new(0);

extern "win64" fn result_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    let stub = RESULT_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(rcx, rdx, r8, r9))).unwrap_or(0);
    let n = RESULT_HITS.fetch_add(1, Ordering::Relaxed);
    if n < 40 { log(&format!("[result] #{} 결과 엔트리 생성 (누적={})\n", n, n + 1)); }
    r
}

// ── (18) [진단] 결과 기록 경로 = 5경기 결과가 어디서 1건으로 줄어드는가 (2026-08-08 v3.1) ──
// 근거 = RE\2026-08-08_리플레이-히스토리-저장구조.md. comp_test는 MatchReplayData를 안 쓰고
//   **`CompTestHistoryEntry`(stride 0xe0)의 무제한 append Vec**(`TeamTrainingPlan.comp_test_history`)에
//   저장된다. 저장소·세이브·서버 핸들러 **어디에도 상한이 없다**. ★진짜 병목 후보 =
//   **클라 러너의 결과 슬롯 `runner+0x21a0` = 단일 Option** — 결과가 연달아 도착하면 앞 것을
//   drop+overwrite ⟹ 최종 1건만 기록. 이번 v2 관측(서버 등록 5/5 성공인데 기록 1건)과 정합.
// 카운터 2개로 어느 계층에서 줄어드는지 확정한다:
//   ①CSEND `0x230c910` = 클라 record&send 진입(state 4→5, 1건 = 기록패킷 1발) → **클라가 보낸 건수**
//   ②HPUSH `0x13006b0` = `Vec<CompTestHistoryEntry>::push`(유일 쓰기 사이트) → **실제 저장된 건수**
// forge(경기 형성) 카운터와 합치면: forge N / CSEND M / HPUSH K 로 계층별 손실이 한 판에 드러난다.
const CSEND_RVA: usize = 0x1a913a0;   // ★0.5.5(구0.5.4=0x230c910) PAGE_IMM owner·push8 12B 실측 MATCH
const HPUSH_RVA: usize = 0x16e3890;   // ★0.5.5(구0.5.4=0x13006b0) skeleton UNIQUE size153·HPUSH_PROLOGUE 실측 MATCH // 커스텀 프롤로그(push rbp,rsi,rdi; sub rsp,0x60; lea rbp,[rsp+0x60])
const HPUSH_PROLOGUE: [u8; 12] = [0x55, 0x56, 0x57, 0x48, 0x83, 0xec, 0x60, 0x48, 0x8d, 0x6c, 0x24, 0x60];
static CSEND_TRAMP: AtomicUsize = AtomicUsize::new(0);
static HPUSH_TRAMP: AtomicUsize = AtomicUsize::new(0);
static CSEND_HITS: AtomicU64 = AtomicU64::new(0);
static HPUSH_HITS: AtomicU64 = AtomicU64::new(0);

// ★v5: rcx는 러너가 아니었다(로그 실측 `rcx=0x76c3ffeda0` = 스택 주소·state 읽기 실패).
//   러너(self)가 어느 인자인지 확정하려고 4개 인자를 전수 프로빙한다 — 각 인자 p에 대해
//   `[p+0x240c]`(팝업 상태머신 0~5)와 `[p+0x21a0]`(결과 슬롯)을 읽어 그럴듯한 조합을 찾는다.
//   러너 포인터를 확보해야 v6의 "결과 큐잉"(슬롯을 매 프레임 비워 덮어쓰기 방지)이 가능하다.
fn runner_probe(p: usize) -> String {
    unsafe {
        if p < 0x10000 || !readable(p + 0x240c, 1) || !readable(p + 0x21a0, 8) {
            return "-".into();
        }
        let st = core::ptr::read_unaligned((p + 0x240c) as *const u8);
        let slot = core::ptr::read_unaligned((p + 0x21a0) as *const u64);
        // 상태머신은 0..=5, 슬롯은 -1(빈칸) 또는 유효 데이터
        let plausible = st <= 5;
        format!("st={}{} slot=0x{:x}", st, if plausible { "★" } else { "" }, slot)
    }
}

// ── (30) [진단] 다시보기 재시뮬이 읽는 "현재 ClientData" 6덩어리 지문 (2026-08-08 v63) ──
// 왜: RE(2026-08-08 `RE\...ClientData누출.md`) 확정 — 다시보기 `0x2323aa0`은 **시드·선수·전략은
//   기록 엔트리에서** 읽지만, 맵·챔피언DB 등 **6덩어리는 `runner+0x23c8`의 현재 ClientData**에서
//   읽는다. comp_test를 한 판 더 돌리면 그 중 하나가 바뀌어 앞 경기 재시뮬이 달라진다(= 관측 증상).
//   ⟹ **어느 덩어리가 바뀌는지**만 확정하면 수정 범위가 그 하나로 좁혀진다. 이 프로브는 **읽기 전용**.
const CD_PROBE_ON: bool = false;     // 릴리스(지문 계산이 매 경기 도는 부하 제거). 재조사 시 true
const CD_RC_OFF: usize = 0x23c8;     // runner → Rc<RefCell<ClientData>>

fn fnv1a(addr: usize, len: usize) -> u64 {
    unsafe {
        if len == 0 || len > 0x200000 || addr < 0x10000 || !readable(addr, len) { return 0; }
        let mut h: u64 = 0xcbf29ce484222325;
        let p = addr as *const u8;
        for i in 0..len {
            h ^= core::ptr::read_unaligned(p.add(i)) as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        h
    }
}
#[inline] unsafe fn qw(a: usize) -> u64 {
    if readable(a, 8) { core::ptr::read_unaligned(a as *const u64) } else { 0 }
}
/// (ptr,len) 쌍으로 보이는 3워드에서 버퍼 지문. 레이아웃이 (cap,ptr,len)인지 (ptr,len,cap)인지
/// 정적으로 확정 못 했으므로 **두 해석 모두** 찍어 로그로 가른다.
unsafe fn buf_fp(base: usize) -> (u64, u64) {
    let (a, b, c) = (qw(base), qw(base + 8), qw(base + 0x10));
    let g = |p: u64, l: u64| -> u64 {
        if p > 0x10000 && l > 0 && l < 0x100000 { fnv1a(p as usize, l as usize) } else { 0 }
    };
    (g(b, c), g(a, b))     // (cap,ptr,len) 해석 / (ptr,len,cap) 해석
}
static CD_LOGS: AtomicU64 = AtomicU64::new(0);

// ★★v69 — **한 겹 깊이 지문**. 다시보기가 읽는 설정들(champion_info_sheet 등)은 **껍데기(inline)만
//   ClientData 안에 있고 실제 내용은 힙**에 있다. v63~66은 inline만 해싱해서 "불변"으로 나왔지만,
//   내용이 제자리에서 바뀌면(in-place) 그건 못 잡는다 — RE가 지목한 유일한 사각지대다.
//   ⟹ inline 영역의 8바이트 워드 중 **유효한 힙 주소로 보이는 것**을 따라가 앞 256B를 함께 해싱한다.
//   원소 크기를 몰라도 "내용이 변했는지"는 이 방식으로 충분히 잡힌다.
unsafe fn deep_fp(base: usize, len: usize) -> (u64, u32) {
    let mut h = fnv1a(base, len);
    let mut chased = 0u32;
    let mut i = 0usize;
    while i + 8 <= len {
        let p = qw(base + i) as usize;
        if p > 0x10000 && p < (1usize << 48) && (p & 7) == 0 && readable(p, 256) {
            h ^= fnv1a(p, 256).rotate_left((i % 61) as u32 + 1);
            chased += 1;
        }
        i += 8;
    }
    (h, chased)
}

// ★★v65 — **전수 페이지 스캔**. v63/64는 RE가 지목한 오프셋만 찍었는데, 그 값들이 전부 불변이고
//   "안 읽는다"는 구간만 변했다 ⟹ 지목 목록이 불완전하거나 base가 어긋났다는 뜻.
//   ⟹ 추측을 늘리지 말고 **ClientData 앞 128KB를 1KB 페이지로 전수 해싱**해 경기 1회로 바뀌는 곳을
//   **전부** 뽑는다. 그 목록과 "다시보기가 읽는 곳"을 교집합하면 범인이 남는다.
// ⚠v66 스캔 폭 0x20000은 **객체 밖까지 훑었다** — Rc 할당은 `rc+0xE460`에서 끝난다(RE 확정).
//   그래서 잡힌 255B×5 구간은 comp_test와 무관한 **옆 힙 객체**였다. 폭을 실제 크기로 정정.
const CD_SCAN_SPAN: usize = 0xe460;
const CD_PAGE: usize = 0x400;          // 1KB 단위로 국소화
static CD_PAGES: std::sync::Mutex<Vec<u8>> = std::sync::Mutex::new(Vec::new());

// ★v66 — 페이지 단위로는 "어느 값"인지 못 가린다(1KB 안에 필드가 수십 개). **바이트 단위 diff**로
//   바뀐 구간을 정확히 뽑고, 8바이트 이하 구간은 **옛값→새값**까지 찍는다.
unsafe fn cd_scan(tag: &str, rc: usize) {
    let n = CD_SCAN_SPAN;
    if !readable(rc, n) { log(&format!("[cdscan] {} 읽기 불가\n", tag)); return; }
    let cur: Vec<u8> = core::slice::from_raw_parts(rc as *const u8, n).to_vec();
    let mut prev = CD_PAGES.lock().unwrap_or_else(|e| e.into_inner());
    if prev.len() != n {
        *prev = cur;
        log(&format!("[cdscan] {} 기준 스냅샷 저장 (0x{:x}B)\n", tag, n));
        return;
    }
    // 달라진 바이트를 **구간으로 병합**(간격 16B 이내면 한 구간으로 본다)
    let mut runs: Vec<(usize, usize)> = Vec::new();
    let mut i = 0usize;
    while i < n {
        if prev[i] != cur[i] {
            let s = i;
            let mut e = i + 1;
            let mut gap = 0;
            while e < n && gap < 16 {
                if prev[e] != cur[e] { gap = 0; } else { gap += 1; }
                e += 1;
            }
            runs.push((s, e - gap));
            i = e;
        } else { i += 1; }
    }
    let mut out = String::new();
    for (k, &(s, e)) in runs.iter().enumerate() {
        if k >= 24 { out.push_str(" …"); break; }
        let len = e - s;
        if len <= 8 {
            let (mut a, mut b) = (0u64, 0u64);
            for j in 0..len {
                a |= (prev[s + j] as u64) << (8 * j);
                b |= (cur[s + j] as u64) << (8 * j);
            }
            out.push_str(&format!("\n    +0x{:<6x} {}B  {:x} -> {:x}", s, len, a, b));
        } else {
            out.push_str(&format!("\n    +0x{:<6x} {}B  (블록)", s, len));
        }
    }
    *prev = cur;
    log(&format!("[cdscan] {} 변경 구간 {}개:{}\n", tag, runs.len(), out));
}

// ── (31) ★다시보기 재현 = "경기가 바꾸는 상태를 되돌려 고정" (2026-08-08 v67) ──────────────
// 왜: comp_test 다시보기는 **저장 재생이 아니라 재시뮬**이고(RE 확정), 재시뮬 입력 일부를
//   **현재 ClientData**에서 읽는다. 경기를 한 판 더 돌리면 그 상태가 바뀌어 앞 경기가 재현 불가.
//   ⟹ 정공법은 "다시보기 동안만 옛 값으로 스왑"이지만 그 함수(`0x2323aa0`)는 대형 프레임이라
//   진입부 훅이 위험하다(v6 STATUS_STACK_OVERFLOW 전례). **대안 = 애초에 안 바뀌게 고정**한다.
//   경기마다 기록 직후 원래 값으로 되돌리면 모든 경기가 같은 입력에서 돌고, 다시보기도 그 입력을 읽는다.
//   시드는 시간 기반이라 경기별로 여전히 달라진다(= 경기 다양성 유지).
// 대상 = v66 바이트 diff 실측으로 확정된 **경기 1회가 바꾸는 13구간**(rc 기준 오프셋).
//   레벨로 나눠 켠다 — 큰 블록은 내부에 포인터가 있을 수 있어 마지막에 붙인다.
const FREEZE_L1: &[(usize, usize)] = &[(0x7d0, 4), (0x7f0, 3), (0x8c1, 3), (0x8e0, 3)];
const FREEZE_L2: &[(usize, usize)] = &[(0xda40, 17), (0xdc10, 1), (0xdc31, 7), (0xde60, 12)];
const FREEZE_L3: &[(usize, usize)] = &[(0x18070, 255), (0x18180, 254), (0x18290, 253),
                                       (0x184b0, 254), (0x186d0, 255)];
// ⛔★★v67 실측 폐기(2026-08-08) — **이 방식(옛 값 write-back)은 원리적으로 불가능하다.**
//   lv=1(가장 작은 4개 구간)만으로도 즉시 크래시: `c0000005 read @0x20bd2e3a620`, exe+0x24a0593.
//   원인 = `+0x7d0`(4B, `d005a7a0`→`dafe7900`) 등이 **힙 포인터의 하위 절반**이었다. 경기마다
//   comp_test 셋업 객체가 **새로 할당되고 옛 것은 해제**되므로, 옛 포인터를 되살리면 dangling이다.
//   ⟹ "상태를 되돌린다"가 아니라 **옛 객체를 살려 둔 채 재시뮬에 물려주는** 방식이어야 한다
//     (= 딥카피 보관 + 다시보기 동안 스왑). 그건 객체 타입·크기·안전한 훅 지점이 먼저 필요하다.
//   ※ 이 코드는 근거 보존용으로 남기되 **기본 비활성**. cfg로도 켜지지 않는다.
const FREEZE_ALLOWED: bool = false;
static FREEZE_LV: AtomicU64 = AtomicU64::new(0);     // cfg `freeze = 0|1|2|3`
static FREEZE_BASE: std::sync::Mutex<Vec<(usize, Vec<u8>)>> = std::sync::Mutex::new(Vec::new());

fn freeze_regions(lv: u64) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = Vec::new();
    if lv >= 1 { v.extend_from_slice(FREEZE_L1); }
    if lv >= 2 { v.extend_from_slice(FREEZE_L2); }
    if lv >= 3 { v.extend_from_slice(FREEZE_L3); }
    v
}

/// 기록 직후 호출. 처음이면 기준값을 뜨고, 이후엔 기준값으로 되돌린다.
unsafe fn freeze_apply(node: usize) {
    if !FREEZE_ALLOWED { return; }        // ⛔포인터 write-back = 확정 크래시(위 주석)
    let lv = FREEZE_LV.load(Ordering::Relaxed);
    if lv == 0 { return; }
    let Some(runner) = runner_of(node) else { return };
    let rc = qw(runner + CD_RC_OFF) as usize;
    if rc < 0x10000 { return; }
    let regs = freeze_regions(lv);
    let mut base = FREEZE_BASE.lock().unwrap_or_else(|e| e.into_inner());
    if base.is_empty() {
        for &(off, len) in &regs {
            if !readable(rc + off, len) { continue; }
            base.push((off, core::slice::from_raw_parts((rc + off) as *const u8, len).to_vec()));
        }
        log(&format!("[freeze] 기준값 확보 lv={} 구간={}개\n", lv, base.len()));
        return;
    }
    let mut n = 0;
    for (off, old) in base.iter() {
        let a = rc + *off;
        if !readable(a, old.len()) { continue; }
        if core::slice::from_raw_parts(a as *const u8, old.len()) == old.as_slice() { continue; }
        core::ptr::copy_nonoverlapping(old.as_ptr(), a as *mut u8, old.len());
        n += 1;
    }
    if n > 0 && FZ_LOGS.fetch_add(1, Ordering::Relaxed) < 12 {
        log(&format!("[freeze] 되돌림 {}구간 (lv={})\n", n, lv));
    }
}
static FZ_LOGS: AtomicU64 = AtomicU64::new(0);

// ── (32) ★★다시보기 재현 = "챔피언 데이터 사본을 다시보기에만 물려주기" (2026-08-08 v70) ────
// 확정 경위: ①다시보기는 재시뮬이고 시드·선수·전략은 기록에서 읽는다(문제없음) ②경기 1회가 바꾸는
//   13구간 중 다시보기가 읽는 건 0개 ③같은 기록을 두 번 다시보기 하면 결과가 같다(=계산은 확정적)
//   ④**한 겹 깊이 지문**에서 `champion_info_sheet`만 변했다(`03a2…`→`b810…`, 추적 163개).
//   ⟹ 껍데기는 그대로인데 **힙 내용이 제자리에서 바뀐다**. 정식 경기가 버전키 스냅샷으로 봉인하는
//   바로 그 데이터이고, comp_test만 라이브를 읽는 게 재현 실패의 원인이다.
// 방식(⚠지난번 크래시와 정반대): **라이브에 write 0회**. 경기 시점에 게임 clone으로 딥카피를 떠 두고,
//   다시보기 동안만 `R8`(ClientData 포인터)을 우리 버퍼로 돌린다. 재시뮬이 읽는 6덩어리가 전부
//   `[rbp+0x190e0]` 한 슬롯을 거치므로 **레지스터 하나만 바꾸면 전부 사본에서 읽힌다**(RE 확정).
// ⛔v70/71 실측(2026-08-08): 다시보기를 누르면 **게임이 즉시 종료**. ★크래시 로거(VEH)에 기록이
//   **남지 않았다** — 이게 결정적 단서다. AV였다면 VEH가 잡아 기록했을 텐데 못 남겼다는 건
//   **핸들러를 돌릴 스택조차 없었다** = STATUS_STACK_OVERFLOW의 전형이다.
//   원인: `0x2323aa0`의 프레임이 **0x19218(≈103KB)**인데, 그 위에서 우리 shim이 다시
//   `catch_unwind` + `Mutex` + `format!/log`(할당·깊은 호출)를 태웠다. 남은 스택을 넘긴 것.
//   ⟹ 훅 지점 문제가 아니라 **detour 본문의 스택 사용량 문제**. 아래 v72에서 shim이 부르는 Rust를
//   **패닉·락·할당·로그 전부 제거한 순수 포인터 연산**으로 축소했다(스택 ~100B).
//   재시험 전까지 기본 OFF. (사본 확보(`snap_take`)는 일반 스택에서 도므로 계속 켜 둔다.)
const RPLY_ON: bool = false;                // ← 스왑 훅 설치 여부(재시험 시에만 true)
const SNAP_ON: bool = true;                 // 사본 확보만(안전) — 스왑과 독립
const RPLY_RVA: usize = 0x1aa8489;          // ★0.5.5(구0.5.4=0x2323bb2) 컨테이너 difflib. ⚠RPLY_ON=false로 inert — RPLY_ORIG 18B는 0.5.5 본문변경으로 불일치(재활성 시 install_mid가 byte mismatch로 fail-safe, 재작성 필요) // 스왑 훅(프레임 확보 후·첫 clone 직전)
const RPLY_ORIG: [u8; 18] = [0x49,0x8d,0x90,0x80,0x19,0x00,0x00,   // lea rdx,[r8+0x1980]
                             0x48,0x8d,0x4d,0xe0,                   // lea rcx,[rbp-0x20]
                             0x4c,0x89,0x85,0xe0,0x90,0x01,0x00];   // mov [rbp+0x190e0],r8
const RPLY_RESUME_RVA: usize = 0x1aa8494;   // ★0.5.5(구0.5.4=0x2323bc4) (RPLY_ON=false inert)
const CLONE_CHAMP_RVA: usize = 0x1b9c660;   // ★0.5.5(구0.5.4=0x193d560) RPLY_RESUME 사이트의 call 타깃으로 확정 // ChampionInfoSheet::clone(rcx=dst 0x7A90, rdx=&src)
const DROP_CHAMP_RVA: usize = 0x182bf30;    // ⬜0.5.5 미확정(5후보 byte동일 monomorphized drop, RPLY_ON=false로 inert·0.5.4값 유지) // ChampionInfoSheet::drop_in_place(rcx=&val)
const CHAMP_OFF: usize = 0x1980;
const CHAMP_SZ: usize = 0x7a90;
const CD_SZ: usize = 0xe460;                // Rc 할당 전체(RcBox 헤더 포함)
const SNAP_MAX: usize = 24;                 // 시드별 사본 상한(초과분은 게임 drop으로 정리)

static RPLY_RESUME: AtomicUsize = AtomicUsize::new(0);
static FAKE_RC: AtomicUsize = AtomicUsize::new(0);          // 다시보기에 물려줄 가짜 ClientData
static RPLY_LOGS: AtomicU64 = AtomicU64::new(0);
// ★v72 — 사본 보관을 **Mutex+Vec → 고정 배열 + 원자변수**로 교체.
//   이유: 스왑 detour는 103KB 프레임 위에서 도므로 **락·할당·패닉 경로를 쓸 수 없다**(스택 초과로
//   VEH조차 못 돌고 프로세스가 즉사했다). 조회는 원자적 읽기 몇 번이면 끝나야 한다.
static SNAP_SEED: [AtomicU64; SNAP_MAX] = [const { AtomicU64::new(0) }; SNAP_MAX];
static SNAP_PTR: [AtomicUsize; SNAP_MAX] = [const { AtomicUsize::new(0) }; SNAP_MAX];
static SNAP_NEXT: AtomicU64 = AtomicU64::new(0);
// ★★v79 — **선수 배열 스냅샷**. 실측으로 확정된 진짜 원인:
//   같은 시드(=같은 경기)인데 **저장되는 엔트리의 선수 배열**과 **다시보기가 읽는 엔트리의 선수 배열**이
//   다르다(`b68aa329…` vs `0a925057…`). 다시보기는 이 배열에서 능력치·아이템·챔피언을 읽으므로
//   다른 선수로 다시 돌리는 셈이 된다 ⟹ 결과 불일치.
//   ⟹ 저장 순간(`0x13006b0`, rdx=완성 엔트리)의 배열을 통째로 떠 두었다가, 다시보기 동안만 끼워 넣는다.
//   배열은 평범한 값 버퍼(stride 0x1A8)이고 길이가 같을 때만 교환하며 끝나면 되돌리므로 소유권 문제가 없다.
const PLAYER_STRIDE: usize = 0x1a8;
static PSNAP_SEED: [AtomicU64; SNAP_MAX] = [const { AtomicU64::new(0) }; SNAP_MAX];
static PSNAP_PTR: [AtomicUsize; SNAP_MAX] = [const { AtomicUsize::new(0) }; SNAP_MAX];
static PSNAP_LEN: [AtomicU64; SNAP_MAX] = [const { AtomicU64::new(0) }; SNAP_MAX];
static PSNAP_NEXT: AtomicU64 = AtomicU64::new(0);

/// 저장되는 완성 엔트리에서 선수 배열을 통째로 복사해 둔다(시드 키).
unsafe fn psnap_take(entry: usize) {
    if entry < 0x10000 || !readable(entry, ENTRY_SZ2) { return; }
    let seed = core::ptr::read_unaligned((entry + E_SEED_OFF) as *const u64);
    if seed == 0 { return; }
    let (p, n) = (qw(entry + 0x38) as usize, qw(entry + 0x40) as usize);
    if p < 0x10000 || n == 0 || n > 64 { return; }
    let sz = n * PLAYER_STRIDE;
    if !readable(p, sz) { return; }
    for i in 0..SNAP_MAX { if PSNAP_SEED[i].load(Ordering::Acquire) == seed { return; } }
    let buf = alloc16(sz);
    if buf == 0 { return; }
    core::ptr::copy_nonoverlapping(p as *const u8, buf as *mut u8, sz);
    let slot = (PSNAP_NEXT.fetch_add(1, Ordering::Relaxed) as usize) % SNAP_MAX;
    PSNAP_SEED[slot].store(0, Ordering::Release);
    let old = PSNAP_PTR[slot].swap(buf, Ordering::AcqRel);
    let oldn = PSNAP_LEN[slot].swap(n as u64, Ordering::AcqRel) as usize;
    if old != 0 {
        if let Ok(l) = std::alloc::Layout::from_size_align(oldn * PLAYER_STRIDE, 16) {
            std::alloc::dealloc(old as *mut u8, l);
        }
    }
    PSNAP_SEED[slot].store(seed, Ordering::Release);
}

#[inline] unsafe fn alloc16(n: usize) -> usize {
    match std::alloc::Layout::from_size_align(n, 16) {
        Ok(l) => std::alloc::alloc(l) as usize,
        Err(_) => 0,
    }
}

/// 경기 기록 **직전**에 호출 — 그 경기가 쓴 챔피언 데이터를 딥카피로 확보한다.
/// (기록 전이므로, 변화가 경기 시작에서 나든 기록에서 나든 이 시점 값이 그 경기의 것이다.)
unsafe fn snap_take(runner: usize, seed: u64) {
    if !SNAP_ON || seed == 0 { return; }
    let rc = qw(runner + CD_RC_OFF) as usize;
    if rc < 0x10000 || !readable(rc + CHAMP_OFF, CHAMP_SZ) { return; }
    let base = exe_base(); if base == 0 { return; }
    for i in 0..SNAP_MAX {
        if SNAP_SEED[i].load(Ordering::Acquire) == seed { return; }   // 이미 있음
    }
    let buf = alloc16(CHAMP_SZ);
    if buf == 0 { return; }
    core::ptr::write_bytes(buf as *mut u8, 0, CHAMP_SZ);
    let clone: extern "win64" fn(usize, usize) -> usize = core::mem::transmute(base + CLONE_CHAMP_RVA);
    clone(buf, rc + CHAMP_OFF);                 // ★반드시 게임 clone — memcpy면 내부 힙이 dangling
    let slot = (SNAP_NEXT.fetch_add(1, Ordering::Relaxed) as usize) % SNAP_MAX;
    SNAP_SEED[slot].store(0, Ordering::Release);            // 먼저 무효화(조회가 반쯤 본 상태 방지)
    let old = SNAP_PTR[slot].swap(buf, Ordering::AcqRel);
    if old != 0 {
        let dropf: extern "win64" fn(usize) = core::mem::transmute(base + DROP_CHAMP_RVA);
        dropf(old);
        if let Ok(l) = std::alloc::Layout::from_size_align(CHAMP_SZ, 16) {
            std::alloc::dealloc(old as *mut u8, l);
        }
    }
    SNAP_SEED[slot].store(seed, Ordering::Release);
    if RPLY_LOGS.fetch_add(1, Ordering::Relaxed) < 12 {
        // ★기록 시점의 엔트리 지문도 함께 남긴다 — 다시보기 시점과 대조하기 위해.
        let (ei, ep, en) = entry_fp(runner + 0x21a0);
        log(&format!("[rply] 사본 확보 seed=0x{:x} slot={} | 엔트리 inline={:016x} 선수{}명={:016x}\n",
            seed, slot, ei, en, ep));
    }
}

/// 다시보기 스왑. 반환 = 물려줄 가짜 ClientData 포인터(0 = 스왑 안 함).
/// ⚠**이 함수는 103KB 프레임 위에서 호출된다** — 패닉 훅·락·할당·로그·포맷 **전부 금지**.
///   순수 포인터 연산만 한다(스택 사용 ~100B). v70의 즉사 원인이 정확히 이 규칙 위반이었다.
extern "win64" fn rply_swap(rc: usize, entry: usize) -> usize {
    unsafe {
        if !RPLY_ON || rc < 0x10000 || entry < 0x10000 { return 0; }
        let fake = FAKE_RC.load(Ordering::Relaxed);
        if fake == 0 { return 0; }
        if !readable(entry + E_SEED_OFF, 8) || !readable(rc, CD_SZ) { return 0; }
        let seed = core::ptr::read_unaligned((entry + E_SEED_OFF) as *const u64);
        if seed == 0 { return 0; }
        let mut sp = 0usize;
        for i in 0..SNAP_MAX {
            if SNAP_SEED[i].load(Ordering::Acquire) == seed {
                sp = SNAP_PTR[i].load(Ordering::Acquire); break;
            }
        }
        if sp == 0 { return 0; }                       // 사본 없는 기록 = 바닐라 동작
        // ① 라이브를 통째로 복사(사본 안 뜬 필드는 최신·유효 상태여야 한다)
        core::ptr::copy_nonoverlapping(rc as *const u8, fake as *mut u8, CD_SZ);
        // ② 챔피언 데이터만 경기 시점 사본으로 교체
        core::ptr::copy_nonoverlapping(sp as *const u8, (fake + CHAMP_OFF) as *mut u8, CHAMP_SZ);
        // ③ RcBox/RefCell 헤더 — 게임의 borrow--가 이쪽으로 오므로 1로 맞춰 둔다
        core::ptr::write_unaligned(fake as *mut u64, 1);
        core::ptr::write_unaligned((fake + 8) as *mut u64, 1);
        core::ptr::write_unaligned((fake + 0x10) as *mut isize, 1);
        // ④ ★진짜 rc의 borrow 보정(borrow++는 진짜에, borrow--는 사본에 걸린다)
        let b = core::ptr::read_unaligned((rc + 0x10) as *const isize);
        core::ptr::write_unaligned((rc + 0x10) as *mut isize, b - 1);
        RPLY_LOGS.fetch_add(1, Ordering::Relaxed);      // 카운터만(로그 금지)
        fake
    }
}

#[unsafe(naked)]
unsafe extern "win64" fn rply_shim() {
    // 진입(0x2323bb2): R8 = rc(RcBox ptr), RDI = &CompTestHistoryEntry, RBP = 프레임.
    //   `FF 25` 무클로버 점프로 들어오므로 모든 레지스터가 원본 그대로다.
    //   ⚠wrapper 호출 금지(원본 프레임 0x19218) — 일 처리 후 원래 흐름으로 jmp back 한다.
    core::arch::naked_asm!(
        "push rcx", "push rdx", "push r9", "push r10", "push r11", "push r8", "push rax",
        "mov r10, rsp",            // 진입 rsp 정렬을 모르므로 강제 정렬(r10은 위에서 보존됨)
        "and rsp, -16",
        "sub rsp, 0x20",
        "mov rcx, r8",             // rc
        "mov rdx, rdi",            // entry
        "call {sw}",
        "mov rsp, r10",
        "test rax, rax",
        "mov r11, rax",            // 반환값을 pop 이후까지 나른다(r11도 보존됨)
        "pop rax",                 // 원래 rax
        "pop r8",                  // 원래 r8
        "cmovne r8, r11",          // 사본이 있으면 그것으로 교체(pop/mov는 플래그 불변)
        "pop r11", "pop r10", "pop r9", "pop rdx", "pop rcx",
        // 스틸한 원본 3명령 재현
        "lea rdx, [r8+0x1980]",
        "lea rcx, [rbp-0x20]",
        "mov qword ptr [rbp+0x190e0], r8",
        "jmp qword ptr [rip + {res}]",
        sw = sym rply_swap,
        res = sym RPLY_RESUME,
    );
}

// ── (33) ★★v73 — **호출자에서 갈아끼우기** (RE 2026-08-08 권고) ─────────────────────────
// 왜 갈아탔나: `0x2323aa0`(다시보기 빌더)은 프레임 103KB + unwind 랜딩패드가 있는 함수라
//   중간 훅에서 Rust를 부르는 것 자체가 위험했다(실측: 무로그 즉사).
//   ★그런데 **그 함수를 부르는 곳은 딱 하나**(`0x2326820`, 프레임 17KB, push 8개짜리 평범한 프롤로그)이고
//   거기서 인자가 그대로 보인다(rcx=ctx, rdx=node, **r8=&CompTestHistoryEntry**).
//   ⟹ **평범한 진입 detour**로 안전하게 개입할 수 있다. 103KB 문제를 통째로 회피.
// 무엇을 하나: 원본을 부르기 **전에** 살아있는 챔피언 데이터와 우리 사본을 **맞바꾸고**, 원본이
//   끝나면 **도로 맞바꾼다**. 둘 다 정상적인 소유값이라 바이트 교환은 그대로 소유권 교환이 된다
//   (복사·해제가 없으므로 이중 해제도 dangling도 원천적으로 없다 — 지난 두 실패와 결정적으로 다른 점).
//   원본 호출 전후로만 감싸므로 **중간에 빠져나가는 경로가 없다** = 원복 누락 불가.
// ⛔v80에서 OFF — 챔피언 데이터·선수 배열 교체는 **원인이 아니었다**(둘 다 실행됐으나 결과 불일치).
//   기록 자체가 sim 입력이 아니므로 기록을 손보는 방식은 전부 무효. (35) 컨텍스트 스냅샷으로 대체.
const RPLY2_ON: bool = false;
const RPLY2_RVA: usize = 0x1aab0f0;         // ★0.5.5(구0.5.4=0x2326820) skeleton UNIQUE size1344 (RPLY2_ON=false inert) // 상세(.detail) 버튼 경로 — 대조군으로만 유지
static RPLY2_TRAMP: AtomicUsize = AtomicUsize::new(0);
static RPLY2_HITS: AtomicU64 = AtomicU64::new(0);

extern "win64" fn rply2_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    let stub = RPLY2_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
    // 스왑 대상 결정 — 실패해도 원본은 반드시 부른다.
    // ★v74 진단: 호출 자체가 오는지 / 어느 검사에서 걸리는지를 한 줄로 가른다.
    //   (v73은 실패 시 조용히 빠져 "안 불린 것"과 "인자 해석이 틀린 것"을 구분할 수 없었다.)
    let n = RPLY2_HITS.fetch_add(1, Ordering::Relaxed);
    let target = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        if !RPLY2_ON { return (0usize, 0usize); }
        // r8이 엔트리가 아닐 수 있으므로, 후보를 넓게 본다: r8 / rdx / rcx 각각을 엔트리로 가정해 시드 매칭.
        let mut seed = 0u64; let mut sp = 0usize; let mut via = "-";
        for (name, cand) in [("r8", r8), ("rdx", rdx), ("rcx", rcx), ("r9", r9)] {
            if cand < 0x10000 || !readable(cand + E_SEED_OFF, 8) { continue; }
            let s = core::ptr::read_unaligned((cand + E_SEED_OFF) as *const u64);
            if s == 0 { continue; }
            for i in 0..SNAP_MAX {
                if SNAP_SEED[i].load(Ordering::Acquire) == s {
                    sp = SNAP_PTR[i].load(Ordering::Acquire); seed = s; via = name; break;
                }
            }
            if sp != 0 { break; }
        }
        // 러너도 인자 후보 전체에서 찾는다(rdx가 node라는 보장이 없다).
        let mut rc = 0usize;
        for cand in [rdx, rcx, r8, r9, RV_NODE.load(Ordering::Relaxed), WATCH_NODE.load(Ordering::Relaxed)] {
            if cand == 0 { continue; }
            if let Some(rn) = runner_of(cand) {
                let v = qw(rn + CD_RC_OFF) as usize;
                if v >= 0x10000 && readable(v + CHAMP_OFF, CHAMP_SZ) { rc = v; break; }
            }
        }
        if n < 8 {
            log(&format!("[rply] 호출자 진입 #{} rcx=0x{:x} rdx=0x{:x} r8=0x{:x} | 시드매칭={}(0x{:x}) rc=0x{:x}\n",
                n, rcx, rdx, r8, via, seed, rc));
        }
        if sp == 0 || rc == 0 { return (0, 0); }
        if n < 12 { log(&format!("[rply] 다시보기 seed=0x{:x} → 경기 시점 챔피언 데이터로 교체\n", seed)); }
        (rc + CHAMP_OFF, sp)
    })).unwrap_or((0, 0));
    // ① 살아있는 값 ↔ 사본 맞바꾸기
    if target.0 != 0 { unsafe { swap_bytes(target.0, target.1, CHAMP_SZ) }; }
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(rcx, rdx, r8, r9))).unwrap_or(0);
    // ② 도로 맞바꾸기 (원본 호출을 감싸므로 빠져나갈 경로가 없다)
    if target.0 != 0 { unsafe { swap_bytes(target.0, target.1, CHAMP_SZ) }; }
    r
}

#[inline] unsafe fn swap_bytes(a: usize, b: usize, n: usize) {
    let (pa, pb) = (a as *mut u8, b as *mut u8);
    for i in 0..n { core::ptr::swap(pa.add(i), pb.add(i)); }
}

// ── (34) ★★v75 — **진짜 경로에 훅** (RE 2026-08-09) ────────────────────────────────────
// 앞선 조사가 "`0x2323aa0`의 유일한 호출자 = `0x2326820`"이라 했지만 **호출자를 하나 놓쳤다**:
//   · **다시보기(.replay)** = `0x1ccb260`(매처) → **`0x1ccb2c0`** → `call 0x2323aa0` **직행**
//   · **상세(.detail)**   = `0x1ccb690` → `0x1ccb6f0` → `0x2326820` → `call 0x2323aa0`
//   ⟹ 내가 건 훅은 **상세 버튼 전용**이었다. 그래서 다시보기를 눌러도 영영 발화하지 않았다.
// ⟹ 재시뮬 진입점 `0x2323aa0` 자체에 건다. 프롤로그 12B가 `0x2326820`과 **완전히 동일**해서
//   설치 로직은 그대로 쓰고 RVA만 바꾸면 된다. 참조 전수 스캔(직접호출 2건 / 절대QWORD 0 / rip-rel 0)으로
//   **간접 호출이 없음**도 확인됐다.
// ★자체 증명: 복귀주소가 `0x1ccb596`이면 다시보기, `0x2326891`이면 상세. 그 외 값은 존재할 수 없다.
const RPLY3_RVA: usize = 0x1aa8370;         // ★0.5.5(구0.5.4=0x2323aa0) 마스크시그 k=8·push8 확인 // 재시뮬 진입점(= 다시보기·상세 공통)
const LIVEB_RVA: usize = 0x1aece30;         // ★0.5.5(구0.5.4=0x235bf20) skeleton UNIQUE size3930·push8 확인 // 원본 경기 빌더(대조군, 읽기 전용)
static RPLY3_TRAMP: AtomicUsize = AtomicUsize::new(0);
static LIVEB_TRAMP: AtomicUsize = AtomicUsize::new(0);
static RPLY3_HITS: AtomicU64 = AtomicU64::new(0);
static LIVEB_HITS: AtomicU64 = AtomicU64::new(0);

// ── (35) ★★★v80 — **경기 컨텍스트 통째 스냅샷** (RE 2026-08-09, 최종 설계) ─────────────
// 왜 여기까지 왔나: 지금까지 "기록을 맞추면 재현된다"고 보고 여러 방식을 시도했는데 전부 실패했다.
//   RE가 그 이유를 확정했다 — **기록 엔트리(0x1A8/명)는 sim의 "입력"이 아니라 "결과 기록"**이고,
//   다시보기는 그 결과에서 입력을 **역으로 짜맞춘다**(챔피언은 이름 문자열로 재조회, 일부 필드는 기본값).
//   ⟹ **손실이 구조적으로 내장**돼 있어 기록을 아무리 정확히 맞춰도 원 경기는 복원되지 않는다.
// 새 설계: 경기가 실제로 돌린 **완성된 컨텍스트(0x20A0)** 를 그 자리에서 딥클론해 보관했다가,
//   다시보기 때 **그 사본을 다시 딥클론해 넘겨주고 원본 빌더는 아예 건너뛴다**.
//   · 컨텍스트는 만들어지는 순간 게임 DB와 분리된다(설정·챔피언 표를 전부 복사해 들고 감) ⟹ 자기완결
//   · sim은 이 컨텍스트 하나만 보고 돈다(전역 읽기 없음, PRNG 상태도 그 안에 굳어 있음) ⟹ 같은 입력 = 같은 결과
//   · 원본 빌더는 외부 상태를 건드리지 않으므로(참조 카운터 증감이 짝으로 상쇄) **건너뛰어도 뒤탈이 없다**
// ⚠소유권: 클론 함수는 공유 표를 참조 카운트로 공유하는 "반쯤 얕은" 복사다. 따라서
//   **보관할 때도 넘겨줄 때도 반드시 클론을 거쳐야** 하고(단순 memcpy 금지 = 이중 해제),
//   보관본은 오직 우리만 게임 drop 함수로 해제한다.
const CTXSNAP_ON: bool = true;
const CTX_SZ: usize = 0x20a0;
const CTX_CLONE_RVA: usize = 0x1cb5390;     // ★0.5.5(구0.5.4=0x23e11e0) skeleton UNIQUE size5570·push8 확인 // (rcx=dst 0x20A0, rdx=src) -> rax=dst, 논리적 딥클론
const CTX_DROP_RVA: usize = 0x1a458d0;      // ★0.5.5(구0.5.4=0x22df620) ghidra-re 2방법: ARRIVE(0x1aab950) 실호출 대상 + 러너 vtable drop 타입짝(원소drop FUN_141851730 공유)·최고접근오프셋 0x2080<0x20A0 // (rcx=&ctx) 정식 drop
const LIVE_SEED_OFF: usize = 0x258;         // request+0x258 = 시드
// ★v81 — 상한 12 → **40**. 유저 실측: 1경기씩은 일치하는데 **병렬 10경기는 전부 불일치**였다.
//   12칸이면 10경기 한 번에 거의 가득 차고, 두 번째 회차에 앞엣것이 밀려난다. 보관본이 없으면
//   조용히 원래(재구성) 경로로 폴백하므로 **티도 안 난다** ⟹ 상한을 넉넉히 잡고 미스를 로그로 남긴다.
//   1건 ≈ 150KB 추정 × 40 ≈ 6MB — 감당 가능.
const CTX_MAX: usize = 40;
static CTX_SEED: [AtomicU64; CTX_MAX] = [const { AtomicU64::new(0) }; CTX_MAX];
static CTX_PTR: [AtomicUsize; CTX_MAX] = [const { AtomicUsize::new(0) }; CTX_MAX];
static CTX_NEXT: AtomicU64 = AtomicU64::new(0);
static CTX_LOGS: AtomicU64 = AtomicU64::new(0);

#[inline] unsafe fn ctx_valid(p: usize) -> bool {
    p >= 0x10000 && readable(p, 4)
        && core::ptr::read_unaligned(p as *const u32) != u32::MAX     // 실패 표식 = dword -1
}

/// 경기 컨텍스트를 딥클론해 시드 키로 보관.
unsafe fn ctx_take(seed: u64, src: usize) {
    if !CTXSNAP_ON || seed == 0 || !ctx_valid(src) { return; }
    let base = exe_base(); if base == 0 { return; }
    for i in 0..CTX_MAX { if CTX_SEED[i].load(Ordering::Acquire) == seed { return; } }
    let buf = alloc16(CTX_SZ);
    if buf == 0 { return; }
    core::ptr::write_bytes(buf as *mut u8, 0, CTX_SZ);
    let clone: extern "win64" fn(usize, usize) -> usize = core::mem::transmute(base + CTX_CLONE_RVA);
    clone(buf, src);
    let slot = (CTX_NEXT.fetch_add(1, Ordering::Relaxed) as usize) % CTX_MAX;
    CTX_SEED[slot].store(0, Ordering::Release);
    let old = CTX_PTR[slot].swap(buf, Ordering::AcqRel);
    if old != 0 {
        let dropf: extern "win64" fn(usize) = core::mem::transmute(base + CTX_DROP_RVA);
        dropf(old);                                   // ★반드시 게임 drop 먼저
        if let Ok(l) = std::alloc::Layout::from_size_align(CTX_SZ, 16) {
            std::alloc::dealloc(old as *mut u8, l);
        }
    }
    CTX_SEED[slot].store(seed, Ordering::Release);
    if CTX_LOGS.fetch_add(1, Ordering::Relaxed) < 12 {
        log(&format!("[ctx] 경기 컨텍스트 보관 seed=0x{:x} slot={}\n", seed, slot));
    }
}

/// 보관본을 다시보기 출력 버퍼에 딥클론해 넣는다. 성공하면 true(원본 빌더를 건너뛴다).
unsafe fn ctx_give(seed: u64, dst: usize) -> bool {
    if !CTXSNAP_ON || seed == 0 || dst < 0x10000 { return false; }
    let base = exe_base(); if base == 0 { return false; }
    let mut src = 0usize;
    for i in 0..CTX_MAX {
        if CTX_SEED[i].load(Ordering::Acquire) == seed { src = CTX_PTR[i].load(Ordering::Acquire); break; }
    }
    if !ctx_valid(src) {
        // ★보관본 미스는 **반드시 남긴다** — 없으면 조용히 원래 경로로 폴백해 원인이 안 보인다.
        if CTX_LOGS.fetch_add(1, Ordering::Relaxed) < 40 {
            let mut held = 0;
            for i in 0..CTX_MAX { if CTX_SEED[i].load(Ordering::Acquire) != 0 { held += 1; } }
            log(&format!("[ctx] ⚠보관본 없음 seed=0x{:x} (보관 {}건) → 원래 방식으로 폴백\n", seed, held));
        }
        return false;
    }
    let clone: extern "win64" fn(usize, usize) -> usize = core::mem::transmute(base + CTX_CLONE_RVA);
    clone(dst, src);                                  // ★memcpy 금지 — 반드시 클론
    if CTX_LOGS.fetch_add(1, Ordering::Relaxed) < 24 {
        log(&format!("[ctx] ★다시보기에 경기 컨텍스트 주입 seed=0x{:x}\n", seed));
    }
    true
}

/// 재시뮬이 읽는 6덩어리의 지문(inline). 두 db가 **같은 객체인지**를 가르는 게 1차 목적이라
/// 포인터가 섞인 inline 해시로 충분하다(같은 객체면 전부 일치).
// ★v76 — 기록 엔트리 지문. 다시보기는 엔트리의 **선수 배열**(ptr `+0x38` / len `+0x40`, stride 0x1A8)에서
//   능력치·아이템·챔피언을 읽는다. 저장 시점과 다시보기 시점이 같은지 대조하기 위한 것.
//   (ClientData 6덩어리가 동일함이 확정됐으므로, 남은 입력은 엔트리뿐이다.)
unsafe fn entry_fp(e: usize) -> (u64, u64, u64) {
    if e < 0x10000 || !readable(e, ENTRY_SZ2) { return (0, 0, 0); }
    let inline = fnv1a(e, ENTRY_SZ2);
    let (p, n) = (qw(e + 0x38) as usize, qw(e + 0x40) as usize);
    let players = if p > 0x10000 && n > 0 && n < 64 { fnv1a(p, n * 0x1a8) } else { 0 };
    (inline, players, n as u64)
}

unsafe fn cd6(rc: usize) -> (u8, u64, u64, u64, u64, u64) {
    let m = if readable(rc + 0x750, 1) { core::ptr::read_unaligned((rc + 0x750) as *const u8) } else { 0xff };
    (m, fnv1a(rc + 0x1980, 0x7a90), fnv1a(rc + 0x9410, 0x1538),
        fnv1a(rc + 0xa948, 0x58), fnv1a(rc + 0xa9a0, 0x3040), fnv1a(rc + 0xd9f8, 0x18))
}

extern "win64" fn rply3_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    let stub = RPLY3_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
    // ★★v80 — 보관해 둔 경기 컨텍스트가 있으면 **그걸 넣고 원본 빌더는 건너뛴다**.
    //   원본은 기록(결과)에서 입력을 역구성하므로 구조적으로 손실이 있다. 우리는 원본 그대로를 준다.
    let hit = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        if !CTXSNAP_ON || !readable(r8 + E_SEED_OFF, 8) { return false; }
        let seed = core::ptr::read_unaligned((r8 + E_SEED_OFF) as *const u64);
        ctx_give(seed, rcx)
    })).unwrap_or(false);
    if hit { return rcx; }              // 반환 규약: rax = out 버퍼
    let target = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        let n = RPLY3_HITS.fetch_add(1, Ordering::Relaxed);
        let seed = if readable(r8 + E_SEED_OFF, 8) {
            core::ptr::read_unaligned((r8 + E_SEED_OFF) as *const u64) } else { 0 };
        let rc = match runner_of(rdx) { Some(rn) => qw(rn + CD_RC_OFF) as usize, None => 0 };
        if n < 8 && rc >= 0x10000 {
            let (m, a, b, c, d, e) = cd6(rc);
            let (ei, ep, en) = entry_fp(r8);
            log(&format!("[rply3] 재시뮬 진입 #{} seed=0x{:x} rc=0x{:x} mode={} \
                champ={:016x} gset={:016x} map={:016x} item={:016x} ai={:016x} \
                | 엔트리 inline={:016x} 선수{}명={:016x}\n",
                n, seed, rc, m, a, b, c, d, e, ei, en, ep));
        } else if n < 8 {
            log(&format!("[rply3] 재시뮬 진입 #{} seed=0x{:x} rc=미확보 (rdx=0x{:x})\n", n, seed, rdx));
        }
        if !RPLY2_ON || seed == 0 || rc < 0x10000 { return (0usize, 0usize); }
        let mut sp = 0usize;
        for i in 0..SNAP_MAX {
            if SNAP_SEED[i].load(Ordering::Acquire) == seed {
                sp = SNAP_PTR[i].load(Ordering::Acquire); break;
            }
        }
        if sp == 0 || !readable(rc + CHAMP_OFF, CHAMP_SZ) { return (0, 0); }
        if n < 8 { log("[rply3] → 경기 시점 챔피언 데이터로 교체\n"); }
        (rc + CHAMP_OFF, sp)
    })).unwrap_or((0, 0));
    // ★v79 — 선수 배열도 저장 시점 것으로 교체(길이가 같을 때만).
    let pswap = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        if !readable(r8 + E_SEED_OFF, 8) { return (0usize, 0usize, 0usize); }
        let seed = core::ptr::read_unaligned((r8 + E_SEED_OFF) as *const u64);
        let (p, n) = (qw(r8 + 0x38) as usize, qw(r8 + 0x40) as usize);
        if seed == 0 || p < 0x10000 || n == 0 || n > 64 { return (0, 0, 0); }
        for i in 0..SNAP_MAX {
            if PSNAP_SEED[i].load(Ordering::Acquire) == seed
                && PSNAP_LEN[i].load(Ordering::Acquire) as usize == n {
                let q = PSNAP_PTR[i].load(Ordering::Acquire);
                let sz = n * PLAYER_STRIDE;
                if q >= 0x10000 && readable(p, sz) {
                    log(&format!("[rply3] → 저장 시점 선수 {}명으로 교체\n", n));
                    return (p, q, sz);
                }
            }
        }
        (0, 0, 0)
    })).unwrap_or((0, 0, 0));
    if target.0 != 0 { unsafe { swap_bytes(target.0, target.1, CHAMP_SZ) }; }
    if pswap.0 != 0 { unsafe { swap_bytes(pswap.0, pswap.1, pswap.2) }; }
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(rcx, rdx, r8, r9))).unwrap_or(0);
    if pswap.0 != 0 { unsafe { swap_bytes(pswap.0, pswap.1, pswap.2) }; }
    if target.0 != 0 { unsafe { swap_bytes(target.0, target.1, CHAMP_SZ) }; }
    r
}

/// 원본 경기 빌더 — **읽기 전용 대조군**. `rc=*(rdx+0x18)`, `seed=*(r8+0x258)`.
extern "win64" fn liveb_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    let stub = LIVEB_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        let n = LIVEB_HITS.fetch_add(1, Ordering::Relaxed);
        if n >= 8 { return; }
        let rc = if readable(rdx + 0x18, 8) { qw(rdx + 0x18) as usize } else { 0 };
        let seed = if readable(r8 + 0x258, 8) { qw(r8 + 0x258) } else { 0 };
        if rc >= 0x10000 {
            let (m, a, b, c, d, e) = cd6(rc);
            log(&format!("[liveb] 원본 빌더 #{} seed=0x{:x} rc=0x{:x} mode={} \
                champ={:016x} gset={:016x} map={:016x} item={:016x} ai={:016x}\n",
                n, seed, rc, m, a, b, c, d, e));
        } else {
            log(&format!("[liveb] 원본 빌더 #{} seed=0x{:x} rc=미확보\n", n, seed));
        }
    }));
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(rcx, rdx, r8, r9))).unwrap_or(0);
    // ★★v80 — 원본이 완성한 컨텍스트를 **그 자리에서 딥클론해 보관**(rcx = out 0x20A0).
    //   여기가 "경기가 실제로 돌린 입력"이 온전한 유일한 지점이다.
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        if !readable(r8 + LIVE_SEED_OFF, 8) { return; }
        let seed = core::ptr::read_unaligned((r8 + LIVE_SEED_OFF) as *const u64);
        ctx_take(seed, rcx);
    }));
    r
}

unsafe fn install_replay_real() {
    match install_hook12(RPLY3_RVA, &RPLY3_TRAMP, rply3_detour as usize) {
        Ok(s) => log(&format!("[rply3] 재시뮬 진입점 0x2323aa0 {}\n", s)),
        Err(e) => log(&format!("[rply3] 실패: {}\n", e)),
    }
    match install_hook12(LIVEB_RVA, &LIVEB_TRAMP, liveb_detour as usize) {
        Ok(s) => log(&format!("[liveb] 원본 빌더 0x235bf20 {}\n", s)),
        Err(e) => log(&format!("[liveb] 실패: {}\n", e)),
    }
}

unsafe fn install_replay_caller() {
    if !RPLY2_ON { log("[rply] 호출자 훅 OFF\n"); return; }
    match install_hook12(RPLY2_RVA, &RPLY2_TRAMP, rply2_detour as usize) {
        Ok(s) => log(&format!("[rply] 호출자 훅 0x2326820 {}\n", s)),
        Err(e) => log(&format!("[rply] 호출자 훅 실패: {}\n", e)),
    }
}

unsafe fn install_replay_swap() {
    if !RPLY_ON { log("[rply] 스왑 훅 OFF (사본 확보만 동작)\n"); return; }
    let base = exe_base(); if base == 0 { return; }
    RPLY_RESUME.store(base + RPLY_RESUME_RVA, Ordering::Relaxed);
    let fake = alloc16(CD_SZ);
    if fake == 0 { log("[rply] 버퍼 할당 실패\n"); return; }
    core::ptr::write_bytes(fake as *mut u8, 0, CD_SZ);
    FAKE_RC.store(fake, Ordering::Relaxed);
    match install_mid(RPLY_RVA, &RPLY_ORIG, rply_shim as usize) {
        Ok(s) => log(&format!("[rply] {}\n", s)),
        Err(e) => log(&format!("[rply] 실패: {}\n", e)),
    }
}

unsafe fn cd_probe(tag: &str, node: usize) {
    if !CD_PROBE_ON { return; }
    if CD_LOGS.fetch_add(1, Ordering::Relaxed) >= 24 { return; }   // 로그 폭주 방지
    let Some(runner) = runner_of(node) else {
        log(&format!("[cd] {} node=0x{:x} 러너 미확인(스킵)\n", tag, node)); return };
    let rc = qw(runner + CD_RC_OFF) as usize;
    if rc < 0x10000 { log(&format!("[cd] {} rc=0 (미획득)\n", tag)); return; }
    let b0 = if readable(rc + 0x750, 1) { core::ptr::read_unaligned((rc + 0x750) as *const u8) } else { 0xff };
    let h1 = fnv1a(rc + 0x1980, 0x7a90);      // 챔피언 DB
    let h2 = fnv1a(rc + 0x9410, 0x1538);      // 맵 원본
    let h3 = fnv1a(rc + 0xa9a0, 0x3040);
    let (p1a, p1b) = buf_fp(rc + 0xa968);
    let (p2a, p2b) = buf_fp(rc + 0xa988);
    let h4 = fnv1a(rc + 0xa948, 0x18);
    // ★1순위 후보 = comp_test 상태 블록.
    //   ⚠v63 결함: `buf_fp`가 len을 **바이트 수**로 해석해 원소 1개(0x10B)를 **1바이트만** 해싱했다
    //   ⟹ Vec 내용 변화를 놓칠 수 있었다. v64에서 `len * 0x10`으로 정정.
    let (v0, v1, v2) = (qw(rc + 0xd9f8), qw(rc + 0xda00), qw(rc + 0xda08));
    let e_full = if v1 > 0x10000 && v2 > 0 && v2 < 0x10000 {
        fnv1a(v1 as usize, (v2 as usize) * 0x10)
    } else { 0 };
    // 블록을 3구간으로 쪼개 **어디가 변하는지** 국소화한다.
    //   s1 = Vec 헤더 주변(0xD9F8~0xDA40) / s2 = `0xa15e20`이 매 실행 memcpy 하는 0x1F8 구간 / s3 = 꼬리
    let s1 = fnv1a(rc + 0xd9f8, 0x48);
    let s2 = fnv1a(rc + 0xda40, 0x1f8);
    let s3 = fnv1a(rc + 0xdc38, 0x8);
    let h5 = fnv1a(rc + 0xd9f8, 0x248);
    log(&format!(
        "[cd] {} rc=0x{:x} b0={} h1={:016x} h2={:016x} h3={:016x} h4={:016x} \
         p1={:016x}/{:016x} p2={:016x}/{:016x} vec=[{:x},{:x},{:x}] eFULL={:016x} \
         s1={:016x} s2={:016x} s3={:016x} blk={:016x}\n",
        tag, rc, b0, h1, h2, h3, h4, p1a, p1b, p2a, p2b, v0, v1, v2, e_full, s1, s2, s3, h5));
    // ★v69 — 다시보기가 실제로 읽는 6덩어리를 **한 겹 깊이**로 다시 잰다(이게 유일한 사각지대).
    let (d_champ, c1) = deep_fp(rc + 0x1980, 0x7a90);   // champion_info_sheet
    let (d_gset, c2) = deep_fp(rc + 0x9410, 0x1538);    // game_setting
    let (d_item, c3) = deep_fp(rc + 0xa9a0, 0x3040);    // item_setting
    let (d_map, c4) = deep_fp(rc + 0xa948, 0x60);       // map_setting(visible_view + path)
    let (d_ai, c5) = deep_fp(rc + 0xd9f8, 0x18);        // mod_ai_registry.player_input_ai
    log(&format!(
        "[cddeep] {} champ={:016x}({}) gset={:016x}({}) item={:016x}({}) map={:016x}({}) ai={:016x}({})\n",
        tag, d_champ, c1, d_gset, c2, d_item, c3, d_map, c4, d_ai, c5));
    cd_scan(tag, rc);
}

extern "win64" fn csend_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    // 진입 시 러너 상태(+0x240c)·미소비 슬롯(+0x21a0) 스냅샷 — 4가 아니면 원본도 즉시 exit.
    let (st, slot) = unsafe {
        if readable(rcx + 0x240c, 1) && readable(rcx + 0x21a0, 8) {
            (core::ptr::read_unaligned((rcx + 0x240c) as *const u8) as i64,
             core::ptr::read_unaligned((rcx + 0x21a0) as *const u64) as i64)
        } else { (-1, 0) }
    };
    let stub = CSEND_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
    // ★(23) 마지막 경기라면 **원본 호출 전에** 무장 — 원본 말미가 페이지를 history로 쓰고
    //   곧바로 refresh를 부르므로, 그 한 번의 흐름으로 결과화면 진입까지 끝난다.
    // ★v61 — 이번 회차의 몇 번째 기록인지는 **여기서** 센다(서버 저장 수는 비동기라 못 쓴다).
    let k = BATCH_REC.fetch_add(1, Ordering::Relaxed) + 1;
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        // 결과화면 원복 판정에 쓸 node = 여기서 본 것(러너와 확실히 연결됨)
        if runner_of(rdx).is_some() { RV_NODE.store(rdx, Ordering::Relaxed); }
        // (32) ★기록 **직전**에 이 경기의 챔피언 데이터 사본 확보.
        //   ⚠v70 결함: 러너를 `rcx`(인자)에서 찾았는데 **러너는 인자가 아니라 `node(rdx)+0x230`**이다
        //   (이 함수 아래쪽 주석에 이미 적혀 있던 사실 — 그래서 사본이 0건이었다).
        if let Some(rn) = runner_of(rdx) {
            if readable(rn + 0x21a0 + E_SEED_OFF, 8)
                && core::ptr::read_unaligned((rn + 0x21a0) as *const u64) != u64::MAX {
                let seed = core::ptr::read_unaligned((rn + 0x21a0 + E_SEED_OFF) as *const u64);
                snap_take(rn, seed);
            }
        }
        resultview_arm_if_last(k)
    }));
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(rcx, rdx, r8, r9))).unwrap_or(0);
    // ★v4 정정: st==4 조건부 로깅은 **0건**이었다(rcx가 러너가 아니거나 오프셋 불일치 추정) —
    //   그 탓에 "클라가 결과를 몇 건 보냈는가"를 못 셌다. 조건을 빼고 전수 카운트하되,
    //   상태 스냅샷은 참고값으로만 남긴다. (조건부 계측이 0을 내면 그 0은 정보가 아니라 계측 실패다.)
    let n = CSEND_HITS.fetch_add(1, Ordering::Relaxed);
    // ★러너는 인자가 아니라 **arg2(node)의 +0x230**에서 온다(v5 프로빙으로 인자 4개 전부 배제됨).
    let st2 = unsafe { runner_of(rdx).map(|r| runner_state(r) as i64).unwrap_or(-1) };
    if n < 40 {
        log(&format!("[csend] #{} 기록 완료 (state {}→{}) forge={} result={} hpush={} | par seen={} 큐={} 재주입={}\n",
            n, st, st2, MFORGE_HITS.load(Ordering::Relaxed),
            RESULT_HITS.load(Ordering::Relaxed), HPUSH_HITS.load(Ordering::Relaxed),
            PAR_SEEN.load(Ordering::Relaxed), PAR_QUEUED.load(Ordering::Relaxed),
            PAR_REPLAYED.load(Ordering::Relaxed)));
    }
    let _ = slot;
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        cd_probe("REC ", rdx);          // (30) 기록 직후 지문 — RUN과 비교해 바뀐 덩어리를 특정
        freeze_apply(rdx);              // (31) 경기가 바꾼 상태를 되돌려 다시보기 입력을 고정
    }));
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        queue_rearm(rcx, rdx);          // (19) QUEUE_ON=false면 no-op
        conc_next_shot(rdx, r8);        // ★(v8) 기록이 끝났으니 다음 경기 1발 발사
    }));
    r
}

extern "win64" fn hpush_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    // ★승패 채집 지점 = **여기**(서버가 히스토리에 저장하는 순간, rdx = 최종 엔트리).
    //   도착 훅(`0x2327094`)에서 읽었더니 `+0xD9`가 항상 0으로 나와 "0 : 5"가 됐다(유저 실측:
    //   실제는 블루 4승) — 그 시점엔 아직 채워지지 않은 것으로 보인다. 저장 직전 값이 최종이다.
    let _ = std::panic::catch_unwind(|| unsafe {
        if !readable(rdx, ENTRY_SZ2) { return; }
        let seed = core::ptr::read_unaligned((rdx + E_SEED_OFF) as *const u64);
        let bw = core::ptr::read_unaligned((rdx + E_BLUEWIN_OFF) as *const u8);
        // ★v78 — 저장 엔트리 지문은 **시드 추적 성공 여부와 무관하게** 찍는다.
        //   v77은 이걸 추적 성공 분기 안에 넣어, `runs=1`(추적 미가동)에선 한 줄도 안 나왔다.
        psnap_take(rdx);        // ★v79 저장 시점 선수 배열 확보
        if HPUSH_HITS.load(Ordering::Relaxed) < 12 {
            let (ei, ep, en) = entry_fp(rdx);
            log(&format!("[save] ★저장 엔트리 seed=0x{:x} 승={} inline={:016x} 선수{}명={:016x}\n",
                seed, bw, ei, en, ep));
        }
        let cnt = M_N.load(Ordering::Relaxed) as usize;
        for i in 0..cnt.min(MAXR) {
            if M_SEED[i].load(Ordering::Relaxed) == seed {
                M_WIN[i].store(if bw != 0 { 2 } else { 1 }, Ordering::Relaxed);
                if HPUSH_HITS.load(Ordering::Relaxed) < 12 {
                    // ★v77 — **완성된 기록이 저장되는 바로 이 순간**의 엔트리 지문.
                    //   v76은 "합쳐지기 전"(러너 슬롯)을 찍어 비교가 성립하지 않았다.
                    //   기록은 `+0x21a0`(엔트리)과 `+0x2280`(결과상세)을 합쳐 만들어지고,
                    //   **선수 배열은 후자에서** 온다 ⟹ 합치기 전과 다른 게 당연했다.
                    let (ei, ep, en) = entry_fp(rdx);
                    log(&format!("[win] 경기{} {} (seed 0x{:x}, +0xD9={}) | ★저장 엔트리 inline={:016x} 선수{}명={:016x}\n",
                        i + 1, if bw != 0 { "블루 승" } else { "레드 승" }, seed, bw, ei, en, ep));
                }
                return;
            }
        }
        // 시드가 안 맞으면 어떤 값들이었는지 남긴다(오프셋 재확인용)
        if HPUSH_HITS.load(Ordering::Relaxed) < 6 {
            log(&format!("[win] ⚠시드 불일치 entry_seed=0x{:x} +0xD9={} +0xD8={}\n", seed, bw,
                core::ptr::read_unaligned((rdx + 0xd8) as *const u8)));
        }
    });
    let before = unsafe { if readable(rcx + 0x10, 8) {
        core::ptr::read_unaligned((rcx + 0x10) as *const u64) } else { u64::MAX } };
    let stub = HPUSH_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(rcx, rdx, r8, r9))).unwrap_or(0);
    let n = HPUSH_HITS.fetch_add(1, Ordering::Relaxed);
    if n < 32 {
        let after = unsafe { if readable(rcx + 0x10, 8) {
            core::ptr::read_unaligned((rcx + 0x10) as *const u64) } else { u64::MAX } };
        log(&format!("[hpush] #{} 히스토리 저장 (len {}→{})\n", n, before as i64, after as i64));
    }
    // (28) 이번 배치의 **마지막 저장**이면 목록을 다시 그리게 예약(2프레임 뒤)
    if RESULTVIEW_ON && RV_ACTIVE.load(Ordering::Relaxed) {
        let runs = CONC_RUNS.load(Ordering::Relaxed);
        if n + 1 >= runs { REFRESH_REQ.store(24, Ordering::Relaxed); }
    }
    r
}

// ── (19) ★결과 큐잉 = 동시 N경기 결과를 전부 기록 (2026-08-08 v6) ────────────────────
// 문제(3회 재현): 경기 5개가 완주해도 **기록은 1건**. 원인 = 결과 도착 함수가 상태와 무관하게
//   러너의 단일 슬롯 `+0x21a0`을 drop+overwrite ⟹ 앞 결과가 소멸(정본 = RE\2026-08-08_결과기록_훅지점_확정.md).
// 해법(D안 "도착 보류 + 재생"): 도착 함수 `0x2327080`을 훅해 **러너가 바쁘면(state==4) 도착을
//   통째로 보류**하고 인자 2개(0x20a0 결과상세 / 0xe0 히스토리 엔트리)를 모드 FIFO로 옮긴다.
//   기록 함수 `0x230c910`이 1건을 끝내면(state==5) 그 자리에서 FIFO의 다음 건을 **원본 도착함수
//   트램폴린으로 재생** ⟹ 게임이 스스로 다음 기록을 수행. 체인: 기록1 → 재무장 → draw → 기록2 → …
// ★게임함수 shadow-call 없음: 호출하는 건 "훅한 함수 자신의 트램폴린"뿐이고, 인자는 살아있는
//   프레임의 rcx/rdx + 모드 소유 버퍼(conc_extra_runs에서 검증된 패턴).
// ★소유권: 인자 3·4는 **move**(호출자가 성공 경로에서 drop하지 않음) ⟹ 바이트를 복사해 오면
//   내부 Vec(participants/champions/players)의 소유권도 함께 넘어온다. 재생으로 게임에 넘긴 뒤엔
//   **바이트 버퍼만** free(내부 Vec은 이미 게임 소유) — 게임 drop 함수는 절대 호출하지 않는다.
// ⛔**2026-08-08 03:3x 인게임 크래시로 OFF 복귀.** 증상 = RUN 1클릭 후 `sreg #1`(두 번째 경기
//   서버 등록 성공) 직후, `forge #1`(두 번째 경기 형성)이 찍히기 전에 게임 사망. `[arrive]` 발화
//   로그는 0건(보류 시에만 찍는 조건이라 무발화인지 미발화인지는 불명).
//   ⚠**이 크래시는 결과 도착 단계가 아니라 "경기 형성 단계"에서 났다** — 즉 arrive 훅의 *설치* 자체가
//   경기 형성 경로를 깨뜨렸을 가능성이 크다(0x2327080은 `mov eax,0x42c0` + `call __chkstk`로
//   17KB 스택을 잡는 함수이고, 그 진입점을 가로채면 __chkstk **프로브 이전에** Rust detour가
//   스택을 쓰게 된다 ⟹ 스택 가드 페이지를 건너뛰어 STATUS_STACK_OVERFLOW 가능. 또한 이 함수는
//   거대 클라 이벤트 디스패처(0x9d5f20)에서 오는 **공용 경로**라 comp_test 외 호출도 받는다).
//   ⟹ 재시도 전 필수 = ①크래시 덤프로 실제 예외코드·스레드 확인 ②detour를 naked-shim 형태로
//   바꿔 __chkstk 프로브를 먼저 태우거나, 훅 지점을 **호출자 쪽(0xa15e20)**으로 옮기는 대안 검토.
//   상세 = 03_시행착오.md 2026-08-08 "결과 큐잉 v6 크래시".
const QUEUE_ON: bool = false;
const ARRIVE_RVA: usize = 0x1aab950;   // ★0.5.5(구0.5.4=0x2327080) 다중앵커 투표 37/37·ARRIVE_PROLOGUE 15B 실측 MATCH (QUEUE_ON=false inert)
const ARRIVE_PROLOGUE: [u8; 15] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x54, 0x56, 0x57, 0x53,
                                   0xb8, 0xc0, 0x42, 0x00, 0x00];
const RUNNER_VT_RVA: usize = 0x348db28;   // ★0.5.5(구0.5.4=0x33b91f8) ghidra-re 2방법: vtable 구조지문(size 0x2410·중복슬롯) .rdata 전수유일 + ctor lea rdx 참조 1곳(@0x21121e5)·R_STATE +0x240c 유지 // node+0x238 == base+이 값 이어야 comp_test 러너
const NODE_RUNNER_OFF: usize = 0x230;     // runner = *(node + 0x230)
const R_STATE: usize = 0x240c;            // 0 idle/1 setup/2 tactics/3 running/4 도착/5 기록완료
const DETAIL_SZ: usize = 0x20a0;          // arg3
const ENTRY_SZ: usize = 0xe0;             // arg4
const QUEUE_CAP: usize = 16;              // 폭주 방지 상한(초과분은 원본 동작 = 덮어쓰기)

static ARRIVE_TRAMP: AtomicUsize = AtomicUsize::new(0);
static ARRIVE_HITS: AtomicU64 = AtomicU64::new(0);
static QUEUED: AtomicU64 = AtomicU64::new(0);
static REPLAYED: AtomicU64 = AtomicU64::new(0);

// 16B 정렬 원시 버퍼 — 게임에 넘긴 뒤엔 바이트만 해제한다(내부 Vec은 게임 소유).
struct RawBuf { ptr: usize, size: usize }
impl RawBuf {
    unsafe fn copy_from(src: usize, size: usize) -> Option<RawBuf> {
        if !readable(src, size) { return None; }
        let layout = std::alloc::Layout::from_size_align(size, 16).ok()?;
        let p = std::alloc::alloc(layout);
        if p.is_null() { return None; }
        core::ptr::copy_nonoverlapping(src as *const u8, p, size);
        Some(RawBuf { ptr: p as usize, size })
    }
}
impl Drop for RawBuf {
    fn drop(&mut self) {
        unsafe {
            if let Ok(l) = std::alloc::Layout::from_size_align(self.size, 16) {
                std::alloc::dealloc(self.ptr as *mut u8, l);
            }
        }
    }
}

static RESULT_FIFO: std::sync::Mutex<Vec<(RawBuf, RawBuf)>> = std::sync::Mutex::new(Vec::new());

// node로부터 러너 포인터 획득 + vtable 일치 검증(게임함수 호출 없이 identity 슬롯을 대체).
unsafe fn runner_of(node: usize) -> Option<usize> {
    if !readable(node + NODE_RUNNER_OFF, 8) || !readable(node + 0x238, 8) { return None; }
    let vt = core::ptr::read_unaligned((node + 0x238) as *const u64) as usize;
    if vt != exe_base().wrapping_add(RUNNER_VT_RVA) { return None; }
    let r = core::ptr::read_unaligned((node + NODE_RUNNER_OFF) as *const u64) as usize;
    if !readable(r + R_STATE, 1) { return None; }
    Some(r)
}
#[inline] unsafe fn runner_state(runner: usize) -> u8 {
    if readable(runner + R_STATE, 1) { core::ptr::read_unaligned((runner + R_STATE) as *const u8) }
    else { 0xff }
}
#[inline] fn fifo_lock() -> std::sync::MutexGuard<'static, Vec<(RawBuf, RawBuf)>> {
    RESULT_FIFO.lock().unwrap_or_else(|e| e.into_inner())
}
#[inline] unsafe fn arrive_orig(rcx: usize, rdx: usize, detail: usize, entry: usize) -> usize {
    let stub = ARRIVE_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = core::mem::transmute(stub);
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(rcx, rdx, detail, entry))).unwrap_or(0)
}

extern "win64" fn arrive_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    let n = ARRIVE_HITS.fetch_add(1, Ordering::Relaxed);
    let handled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        if !QUEUE_ON { return None; }
        let runner = match runner_of(rdx) { Some(r) => r, None => return None };
        let st = runner_state(runner);
        let mut q = fifo_lock();
        // 계류 중인 결과가 있거나(state==4) 이미 줄이 서 있으면 → 이번 도착을 줄 끝에 넣는다
        if st == 4 || !q.is_empty() {
            if q.len() >= QUEUE_CAP { return None; }          // 상한 초과 = 원본 동작(덮어쓰기)
            let d = match RawBuf::copy_from(r8, DETAIL_SZ) { Some(b) => b, None => return None };
            let e = match RawBuf::copy_from(r9, ENTRY_SZ) { Some(b) => b, None => return None };
            q.push((d, e));
            QUEUED.fetch_add(1, Ordering::Relaxed);
            if st != 4 {
                // 파이프라인이 비어 있다 → 줄 맨 앞 1건을 지금 즉시 게임에 넘긴다
                let (d0, e0) = q.remove(0);
                drop(q);
                let r = arrive_orig(rcx, rdx, d0.ptr(), e0.ptr());
                REPLAYED.fetch_add(1, Ordering::Relaxed);
                return Some(r);
            }
            if n < 40 { log(&format!("[arrive] #{} 보류 (state={} 대기열={})\n", n, st, q.len())); }
            return Some(0);                                    // 보류 = 원본 스킵(호출자는 반환값 무시)
        }
        None
    })).unwrap_or(None);
    match handled {
        Some(r) => r,
        None => unsafe { arrive_orig(rcx, rdx, r8, r9) },
    }
}

impl RawBuf { #[inline] fn ptr(&self) -> usize { self.ptr } }

// 기록 1건이 끝난 직후(state==5) 다음 결과를 재무장 — csend_detour 말미에서 호출.
unsafe fn queue_rearm(rcx: usize, rdx: usize) {
    if !QUEUE_ON { return; }
    let runner = match runner_of(rdx) { Some(r) => r, None => return };
    if runner_state(runner) != 5 { return; }
    let mut q = fifo_lock();
    if q.is_empty() { return; }
    let (d, e) = q.remove(0);
    let left = q.len();
    drop(q);
    arrive_orig(rcx, rdx, d.ptr(), e.ptr());
    let k = REPLAYED.fetch_add(1, Ordering::Relaxed);
    log(&format!("[arrive] 재무장 #{} (남은 대기열={})\n", k, left));
}

// ── (20) [진단·무훅] 러너 상태 폴링 = "결과가 몇 번 도착하는가" 확정 (2026-08-08 v7) ────
// v6가 크래시한 뒤 남은 결정적 미지수: **도착이 5회인데 기록이 1회**인가, **도착 자체가 1회**인가.
//   전자면 큐잉이 필요하고, 후자면 문제는 상류(서버 응답)라 접근이 완전히 달라진다.
// v6처럼 도착 함수를 훅하면 또 스택 오버플로 ⟹ **훅 없이 UI 스레드 폴링(읽기 전용)** 으로 센다:
//   러너의 `+0x240c`(상태) / `+0x21a0`(결과 슬롯 첫 qword, -1=빈칸)을 매 프레임 읽어 **변화만** 기록.
//   슬롯이 (빈칸→값) 또는 (값→다른 값)으로 바뀐 횟수 = 도착 횟수. 쓰기·훅·게임함수 호출 전무 = 무해.
// 러너 포인터는 RUN 클릭 시 `run_detour`의 인자에서 node를 캡처해 얻는다(러너 = *(node+0x230),
//   검증 = *(node+0x238)==base+0x33b91f8). node는 매번 재검증하므로 stale이어도 안전하게 탈락.
static WATCH_NODE: AtomicUsize = AtomicUsize::new(0);
static WATCH_LAST_ST: AtomicU64 = AtomicU64::new(u64::MAX);
static WATCH_LAST_SLOT: AtomicU64 = AtomicU64::new(0);
static WATCH_ARRIVALS: AtomicU64 = AtomicU64::new(0);
static WATCH_LOGS: AtomicU64 = AtomicU64::new(0);

// RUN 클릭 시 호출 — 인자 중 comp_test 러너로 이어지는 node를 찾아 저장
unsafe fn watch_capture(a: usize, b: usize) {
    if WATCH_NODE.load(Ordering::Relaxed) != 0 { return; }
    for cand in [a, b] {
        if let Some(r) = runner_of(cand) {
            WATCH_NODE.store(cand, Ordering::Relaxed);
            log(&format!("[watch] node=0x{:x} → runner=0x{:x} 캡처\n", cand, r));
            return;
        }
    }
    log("[watch] ⚠러너 캡처 실패(인자 어느 쪽도 comp_test 노드 아님)\n");
}

// post_update(UI 스레드)에서 매 프레임 호출 — 읽기 전용
fn watch_tick() {
    let node = WATCH_NODE.load(Ordering::Relaxed);
    if node == 0 { return; }
    unsafe {
        let runner = match runner_of(node) { Some(r) => r, None => return };
        if !readable(runner + 0x21a0, 8) { return; }
        let st = runner_state(runner) as u64;
        let slot = core::ptr::read_unaligned((runner + 0x21a0) as *const u64);
        let prev_st = WATCH_LAST_ST.swap(st, Ordering::Relaxed);
        let prev_slot = WATCH_LAST_SLOT.swap(slot, Ordering::Relaxed);
        if st == prev_st && slot == prev_slot { return; }          // 변화 없으면 침묵
        // 슬롯이 새로 채워졌다 = 결과 도착 1건
        let arrived = slot != u64::MAX && slot != prev_slot;
        if arrived { WATCH_ARRIVALS.fetch_add(1, Ordering::Relaxed); }
        if WATCH_LOGS.fetch_add(1, Ordering::Relaxed) < 120 {
            log(&format!("[watch] state {}→{} slot 0x{:x}→0x{:x}{} | 도착누적={} csend={} hpush={}\n",
                prev_st as i64, st, prev_slot, slot, if arrived { " ★도착" } else { "" },
                WATCH_ARRIVALS.load(Ordering::Relaxed),
                CSEND_HITS.load(Ordering::Relaxed), HPUSH_HITS.load(Ordering::Relaxed)));
        }
    }
}

// ── (22) [진단] 클라 경고문구 사유 로깅 = "RUN이 막힌 진짜 이유" (2026-08-08 v10) ─────
// 배경: 5경기 실행 후 RUN이 막히는데(사전게이트 `0x2310a90` → al=0), RE 결과 **일일 5회 제한은
//   원인이 아니다**(그 분기 ①은 `dr_inline_b 0x2310c86`으로 이미 사망, 온디스크 바이트 대조 확인).
//   남은 사유는 ②`not_enough_lane_roster`(가용 로스터 목록이 빔 = 선수가 busy로 마킹돼 필터에서 탈락)
//   또는 ④`champion_required`(픽 Vec이 비었거나 len<required). 둘은 대응 패치가 전혀 다르다.
// 이 훅이 **문자열 길이로 사유를 즉시 특정**한다(문구 자체는 번역키라 len이 지문):
//   0x32=no_attempts / 0x3d=not_enough_lane_roster / 0x38=champion_required 또는 duplicate_players
//   (0x38 두 종은 ptr로 구분: champion=0x33ddf8a, duplicate=0x33ddfc2)
// 프롤로그 13B(`push rbp,r14,rsi,rdi,rbx` + `sub rsp,0xb0`) — `__chkstk` 없음 = 훅 안전.
const WARN_RVA: usize = 0x1a7c000;   // ★0.5.5(구0.5.4=0x22f6d00) skeleton UNIQUE size458·WARN_PROLOGUE 13B 실측 MATCH (CONC_PROBE inert)
const WARN_PROLOGUE: [u8; 13] = [0x55, 0x41, 0x56, 0x56, 0x57, 0x53,
                                 0x48, 0x81, 0xec, 0xb0, 0x00, 0x00, 0x00];
static WARN_TRAMP: AtomicUsize = AtomicUsize::new(0);
static WARN_HITS: AtomicU64 = AtomicU64::new(0);

extern "win64" fn warn_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    let n = WARN_HITS.fetch_add(1, Ordering::Relaxed);
    if n < 40 {
        let base = unsafe { exe_base() };
        let rva = if rdx > base { rdx - base } else { 0 };
        let why = match (r8, rva) {
            (0x32, _) => "일일 5회 초과(no_attempts)",
            (0x3d, _) => "★인원부족(not_enough_lane_roster) = 가용 로스터 목록이 비었음",
            (0x38, 0x33ddf8a) => "★챔피언 미지정(champion_required)",
            (0x38, 0x33ddfc2) => "선수 중복(duplicate_players)",
            (0x38, _) => "len 0x38(챔피언/중복 중 하나 — ptr 미상)",
            (0, _) => "통과(문구 없음)",
            _ => "기타",
        };
        log(&format!("[warn] #{} len=0x{:x} strRVA=0x{:x} → {}\n", n, r8, rva, why));
    }
    let stub = WARN_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(rcx, rdx, r8, r9))).unwrap_or(0)
}

// ── (23) ★다회차 결과 화면 = 기록탭 재활용 (2026-08-08 v11) ─────────────────────────
// 근거 = RE\2026-08-08_다회차결과화면_경로확정.md. 유저 요구("이번에 돌린 경기만 모은 결과 화면,
//   승패 집계, 정보/리플레이 버튼 동작, 뒤로가기")를 **신규 UI 구현 없이** 달성한다.
// 원리: 기록탭 목록 빌더 `0x2311c20`은 게임상태 원본 Vec을 `filter.rev().take(20)`으로 잘라
//   로컬 Vec을 만들고 그것을 렌더한다. 그 **take 한도가 정적 상수(imm32 @0x2311d43, 기본 0x14)**다.
//   ⟹ 이 값을 이번 회차 N으로 바꾸면 "최신 N건 = 이번에 돌린 N경기"만 남는다.
//   목록·스크롤·카드·정보/리플레이 버튼은 **게임 것 그대로** 동작한다(클릭 핸들러가 쓰는 캐시도
//   같은 Vec의 클론이라 인덱스가 자동 일치). 포인터를 건드리지 않으므로 소유권 위험 0.
// 자동 진입: 기록·송신 `0x230c910` 말미가 `runner+0x240c`(UI **페이지** enum)에 `5`(summary)를
//   쓰고 곧바로 refresh를 부른다. 그 **imm8(@0x230d0ec)을 `1`(history)로** 바꾸면 마지막 경기
//   기록 직후 팝업이 기록탭으로 점프하며 목록을 그 자리에서 재생성한다.
// ⚠`runner+0x240c` = **UI 페이지**(0 setup/1 history/2 champion/3 tactics/4·5 summary).
//   기존 코드 주석의 "상태머신 0 idle/1 setup/…"은 오독 — 동작에는 영향 없었으나 의미가 다르다.
const RESULTVIEW_ON: bool = true;
const TAKE_RVA: usize = 0x1a9683a;      // ★0.5.5(구0.5.4=0x2311d43) 마스크시그 k=1·imm32 14000000 실측·cont5=0x1a96710 // imm32 = 기록탭 목록 take 한도(기본 0x14)
const TAKE_DEFAULT: u32 = 0x14;
const PAGE_IMM_RVA: usize = 0x1a91bcc;  // ★0.5.5(구0.5.4=0x230d0ec) 고정패턴 41c6860c24000005 전역유일·imm off+7 // imm8 = 기록 완료 후 이동할 페이지(기본 5=summary)
const PAGE_SUMMARY: u8 = 5;
const PAGE_HISTORY: u8 = 1;
static RV_ACTIVE: AtomicBool = AtomicBool::new(false);   // 배치 결과화면 모드 진행 중
// ★v12: 원복 판정에 쓰는 node는 **csend에서 본 것**(= 팝업 러너와 확실히 연결된 노드)을 쓴다.
//   run_detour의 rcx로 잡은 `WATCH_NODE`를 쓰면 다른 러너를 보고 즉시 원복해버린다(v11 실측:
//   기록탭 진입 직후 `page==0`으로 읽혀 take가 바로 20으로 되돌아갔다).
static RV_NODE: AtomicUsize = AtomicUsize::new(0);
static RV_SETUP_FRAMES: AtomicU64 = AtomicU64::new(0);   // 준비화면 연속 관측 프레임(디바운스)

unsafe fn poke_u32(rva: usize, val: u32) -> bool {
    let base = exe_base(); if base == 0 { return false; }
    let a = base + rva;
    if !readable(a, 4) { return false; }
    if core::ptr::read_unaligned(a as *const u32) == val { return true; }   // 멱등
    let mut old: u32 = 0;
    if VirtualProtect(a, 4, 0x04, &mut old) == 0 { return false; }
    core::ptr::write_unaligned(a as *mut u32, val);
    VirtualProtect(a, 4, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), a, 4);
    true
}
unsafe fn poke_u8(rva: usize, val: u8) -> bool {
    let base = exe_base(); if base == 0 { return false; }
    let a = base + rva;
    if !readable(a, 1) { return false; }
    if core::ptr::read_unaligned(a as *const u8) == val { return true; }
    let mut old: u32 = 0;
    if VirtualProtect(a, 1, 0x04, &mut old) == 0 { return false; }
    core::ptr::write_unaligned(a as *mut u8, val);
    VirtualProtect(a, 1, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), a, 1);
    true
}

// 마지막 경기 기록 **직전**에 호출 — 원본이 페이지를 history로 쓰게 만든다.
unsafe fn resultview_arm_if_last(k: u64) -> bool {
    let runs = CONC_RUNS.load(Ordering::Relaxed);
    if !RESULTVIEW_ON || runs <= 1 { return false; }
    // ★★v61 정정 — 조건에서 **서버 저장 수(HPUSH_HITS)를 뺀다.**
    //   서버 저장은 비동기라 마지막 클라 기록(csend)이 그 직전에 들어오면 카운터가 1 모자라
    //   조건이 영영 성립하지 않는다 ⟹ v60 실측: 5경기 전량 기록됐는데 **rview 0회 발화**,
    //   화면이 결과창에서 멈춤(유저 보고 "5경기 끝났는데 화면이 안 넘어가").
    //   `k` = 이번 회차의 몇 번째 클라 기록인가(csend에서 동기 증가) = 경합 없는 정확한 신호.
    //   ※마지막 1건이 목록에 늦게 뜨는 문제는 원래대로 `REFRESH_REQ`(지연 재생성)가 담당한다.
    let is_last = if par_on() {
        k >= runs
    } else {
        CONC_PENDING.load(Ordering::Relaxed) == 0 && CONC_SHOT.load(Ordering::Relaxed) > 0
    };
    if !is_last {
        if par_on() && k <= 12 {
            log(&format!("[rview] 대기 {}/{} (큐={})\n", k, runs, par_lock().len()));
        }
        return false;
    }
    let n = runs.min(0xffff) as u32;
    poke_u32(TAKE_RVA, n);                 // 목록을 이번 회차 N건으로 한정
    poke_u8(PAGE_IMM_RVA, PAGE_HISTORY);   // 기록 완료 → 기록탭으로 점프
    RV_ACTIVE.store(true, Ordering::Relaxed);
    HIST_SCOPE.store(0, Ordering::Relaxed);   // 새 배치 = "테스트 결과"로 시작
    log(&format!("[rview] 마지막 경기 — 결과화면 진입 준비(take={} page=history)\n", n));
    true
}

// ★(28) 목록 재생성 유발 — "마지막 1건 누락" 해결 (2026-08-08 v25)
// 증상: 기록탭 자동 진입 시 **N−1건만** 보이고, 다른 탭 갔다 오면 N건이 된다.
// 원인: 페이지 전환·목록 생성이 **마지막 경기의 서버 저장(hpush)보다 먼저** 일어난다(클라 기록 요청 →
//   서버 저장이 비동기라 한 박자 늦다). ⟹ 마지막 저장이 끝난 뒤 **목록만 다시 그리면** 된다.
// 방법(RE\2026-08-08 토글조사): 팝업 refresh `0x2306000(rcx=assets, rdx=node)`를 직접 호출.
//   assets = 게임 에셋 매니저(모드 `Assets`가 아님) → 로더 체인 훅에서 캐시한 값 사용.
//   ⚠shadow-call이므로 catch_unwind + 3중 가드(assets/node/page 검증), UI 스레드에서만.
const REFRESH_RVA: usize = 0x1a8aa10;   // ★0.5.5(구0.5.4=0x2306000) dr_inline_a 컨테이너·push8 확인(직접 CALL)
static REFRESH_REQ: AtomicU64 = AtomicU64::new(0);   // >0 = N프레임 뒤 refresh
static REFRESH_DONE: AtomicU64 = AtomicU64::new(0);

// ★(29) 기록탭 범위 토글 — "테스트 결과(이번 회차 N건) / 모든 결과(전체)" (2026-08-08 v35)
//   목록 개수를 정하는 take 상수(@0x2311d43)를 토글값에 맞춰 바꾸고 목록을 다시 그린다.
//   버튼·스크롤·카드는 전부 게임 것 그대로이므로, 우리가 할 일은 상수 1개 + refresh 1회뿐.
static HIST_SCOPE: AtomicU64 = AtomicU64::new(0);      // 0 = 이번 회차 N건 / 1 = 전체
static HS_LAST_FH: AtomicUsize = AtomicUsize::new(usize::MAX);
// 토글로 목록이 바뀌면 스크롤을 맨 위로. 목록 재생성(refresh)이 1~2프레임 뒤에 끝나므로
//   몇 프레임에 걸쳐 반복 적용해야 확실히 먹는다.
static SCROLL_RESET: AtomicU64 = AtomicU64::new(0);
static RUNS_LAST_FH: AtomicUsize = AtomicUsize::new(usize::MAX);
// 경기 수 증감(1~10). cfg `runs`는 기본값이고, 인게임 조작이 우선한다(세션 내 유지).
fn runs_dec() {
    let v = CONC_RUNS.load(Ordering::Relaxed);
    CONC_RUNS.store(if v > 1 { v - 1 } else { 1 }, Ordering::Relaxed);
}
fn runs_inc() {
    let v = CONC_RUNS.load(Ordering::Relaxed);
    CONC_RUNS.store(if v < CONC_RUNS_MAX { v + 1 } else { CONC_RUNS_MAX }, Ordering::Relaxed);
}

fn scope_tick(ui: &mut GameUI) {
    if !RESULTVIEW_ON { return; }
    // 클릭 라우팅(게임이 filter_handler를 갈아끼우면 자동 재등록)
    ui_kit::ensure_clicks(ui, &HS_LAST_FH, vec![
        ui_kit::route(ui_inject::HS_LAST_ID, std::rc::Rc::new(|| {
            HIST_SCOPE.store(0, Ordering::Relaxed); REFRESH_REQ.store(1, Ordering::Relaxed);
            SCROLL_RESET.store(6, Ordering::Relaxed);
        })),
        ui_kit::route(ui_inject::HS_ALL_ID, std::rc::Rc::new(|| {
            HIST_SCOPE.store(1, Ordering::Relaxed); REFRESH_REQ.store(1, Ordering::Relaxed);
            SCROLL_RESET.store(6, Ordering::Relaxed);
        })),
    ]);
    // 목록이 갈린 직후 몇 프레임 동안 스크롤을 맨 위로 고정
    if SCROLL_RESET.load(Ordering::Relaxed) > 0 {
        SCROLL_RESET.fetch_sub(1, Ordering::Relaxed);
        ui_kit::scroll_set_by_id(&mut ui.root, "scroll", 0.0);
    }
    // ── (30) 경기 수 선택(−/+) — 준비 화면[0] / 전술 화면[1] 두 벌 ──
    let page = unsafe {
        let node = RV_NODE.load(Ordering::Relaxed);
        if node == 0 { 0xff } else { runner_of(node).map(|r| runner_state(r)).unwrap_or(0xff) }
    };
    ui_kit::ensure_clicks(ui, &RUNS_LAST_FH, vec![
        ui_kit::route(ui_inject::RUNS_DEC_ID[0], std::rc::Rc::new(runs_dec)),
        ui_kit::route(ui_inject::RUNS_INC_ID[0], std::rc::Rc::new(runs_inc)),
        ui_kit::route(ui_inject::RUNS_DEC_ID[1], std::rc::Rc::new(runs_dec)),
        ui_kit::route(ui_inject::RUNS_INC_ID[1], std::rc::Rc::new(runs_inc)),
    ]);
    let runs = CONC_RUNS.load(Ordering::Relaxed);
    for i in 0..2 {
        let want = (i == 0 && page == 0) || (i == 1 && page == 3);
        ui_kit::set_visible_by_id(&mut ui.root, ui_inject::RUNS_BOX_ID[i], want);
        if want {
            let txt = format!("한 번에 {}경기", runs);
            if let Some(n) = ui_kit::find_mut(&mut ui.root, ui_inject::RUNS_VAL_ID[i]) {
                if ui_kit::label_get(n).as_deref() != Some(txt.as_str()) { ui_kit::label_set(n, &txt); }
            }
        }
    }
    // 기록탭(page 1)일 때만 노출 + 선택 상태 반영
    let on_history = unsafe {
        let node = RV_NODE.load(Ordering::Relaxed);
        node != 0 && runner_of(node).map(|r| runner_state(r) == 1).unwrap_or(false)
    };
    ui_kit::set_visible_by_id(&mut ui.root, ui_inject::HS_BOX_ID, on_history);
    // 이번 회차 승패 집계("블루 N : M 레드") — 범위가 "테스트 결과"일 때만 의미가 있다.
    let sc = HIST_SCOPE.load(Ordering::Relaxed);
    let show_wl = on_history && sc == 0 && M_N.load(Ordering::Relaxed) > 0;
    for id in [ui_inject::WL_BL_ID, ui_inject::WL_BN_ID, ui_inject::WL_C_ID,
               ui_inject::WL_RN_ID, ui_inject::WL_RL_ID] {
        ui_kit::set_visible_by_id(&mut ui.root, id, show_wl);
    }
    if on_history {
        ui_kit::toggle_set_by_id(&mut ui.root, ui_inject::HS_LAST_ID, sc == 0);
        ui_kit::toggle_set_by_id(&mut ui.root, ui_inject::HS_ALL_ID, sc == 1);
    }
    if show_wl {
        // ⚠**임시 집계** — 킬 스코어 비교. 유저 실측으로 **실제 승패와 어긋남이 확인**됐다
        //   (승부는 넥서스 파괴로 갈린다). 결과 엔트리의 `blue_win` 오프셋을 확정하는 대로
        //   그 값으로 교체할 것(⬜RE 진행 중). 그때까지는 참고용 수치다.
        let n = M_N.load(Ordering::Relaxed) as usize;
        let (mut b, mut r) = (0u32, 0u32);
        for i in 0..n.min(MAXR) {
            match M_WIN[i].load(Ordering::Relaxed) { 2 => b += 1, 1 => r += 1, _ => {} }
        }
        // 팀 이름 = 팀 색, 숫자 = 흰색 (라벨 하나에 색이 하나뿐이라 5조각으로 나눔)
        for (id, txt, col) in [
            (ui_inject::WL_BL_ID, "블루".to_string(),  ui_inject::COL_BLUE),
            (ui_inject::WL_BN_ID, format!("{}", b),    ui_inject::COL_WHITE),
            (ui_inject::WL_C_ID,  ":".to_string(),     ui_inject::COL_DIM),
            (ui_inject::WL_RN_ID, format!("{}", r),    ui_inject::COL_WHITE),
            (ui_inject::WL_RL_ID, "레드".to_string(),  ui_inject::COL_RED),
        ] {
            if let Some(node) = ui_kit::find_mut(&mut ui.root, id) {
                if ui_kit::label_get(node).as_deref() != Some(txt.as_str()) {
                    ui_kit::label_set(node, &txt);
                    ui_kit::label_set_color(node, ui_kit::Rgba::hex(col));
                }
            }
        }
    }
}

fn refresh_tick() {
    let pend = REFRESH_REQ.load(Ordering::Relaxed);
    if pend == 0 { return; }
    // ★2프레임 뒤 1회만 그렸더니 **가끔 마지막 1건이 빠졌다**(유저 실측 — 탭을 바꾸면 나옴).
    //   서버 저장이 끝나는 시점에 편차가 있다는 뜻 ⟹ **여유를 두고 3번에 걸쳐** 다시 그린다
    //   (약 0.1초 / 0.3초 / 0.4초 지점). 목록 재생성은 부작용이 없어 여러 번 해도 무해하다.
    let now = REFRESH_REQ.fetch_sub(1, Ordering::Relaxed).saturating_sub(1);
    if now != 18 && now != 6 && now != 0 { return; }
    let _ = std::panic::catch_unwind(|| unsafe {
        let node = RV_NODE.load(Ordering::Relaxed);
        let am = ui_inject::ASSETS.load(Ordering::Relaxed);
        if node == 0 || am <= 0x10000 { return; }
        let runner = match runner_of(node) { Some(r) => r, None => return };
        if runner_state(runner) != 1 { return; }        // 기록탭일 때만 의미가 있다
        // 토글 상태에 맞춰 목록 개수를 정한 뒤 다시 그린다
        let take = if HIST_SCOPE.load(Ordering::Relaxed) == 1 { TAKE_DEFAULT }
                   else { CONC_RUNS.load(Ordering::Relaxed).min(0xffff) as u32 };
        poke_u32(TAKE_RVA, take);
        let base = exe_base(); if base == 0 { return; }
        let f: extern "win64" fn(usize, usize) -> usize = core::mem::transmute(base + REFRESH_RVA);
        f(am, node);
        let k = REFRESH_DONE.fetch_add(1, Ordering::Relaxed);
        if k < 8 { log(&format!("[rview] 목록 재생성 #{} (기록 {}건)\n", k, HPUSH_HITS.load(Ordering::Relaxed))); }
    });
}

// UI 스레드(post_update) — 유저가 결과화면을 벗어나면(=setup 페이지로 복귀) 원복한다.
fn resultview_tick() {
    if !RESULTVIEW_ON || !RV_ACTIVE.load(Ordering::Relaxed) { return; }
    let node = RV_NODE.load(Ordering::Relaxed);
    if node == 0 { return; }
    unsafe {
        let runner = match runner_of(node) { Some(r) => r, None => return };
        let page = runner_state(runner);
        // 0 = setup(준비 화면). ⚠전환 도중 한 프레임 0으로 스치는 경우가 있어(v11 실측 = 즉시 원복
        //   사고) **연속 30프레임(≈0.5초) 유지될 때만** 진짜 이탈로 본다.
        if page == 0 {
            if RV_SETUP_FRAMES.fetch_add(1, Ordering::Relaxed) + 1 >= 30 {
                poke_u32(TAKE_RVA, TAKE_DEFAULT);
                poke_u8(PAGE_IMM_RVA, PAGE_SUMMARY);
                RV_ACTIVE.store(false, Ordering::Relaxed);
                RV_SETUP_FRAMES.store(0, Ordering::Relaxed);
                log("[rview] 준비 화면 복귀 — 기록탭 원복(take=20, page=summary)\n");
            }
        } else {
            RV_SETUP_FRAMES.store(0, Ordering::Relaxed);
        }
    }
}

// ── (27) ★킬스코어 현황 오버레이 (2026-08-08 v23) ──────────────────────────────────
// 유저 요구: "처음 진행바 대신 경기 N개의 킬스코어 현황을 다 보여주고, 끝나면 결과 정리 중,
//   그 다음 기록탭에 이번 N개".
// 근거: 게임 로딩바는 **결과 도착 후 1.4초 시간 램프 연출**이라 병렬에선 의미가 없다(그래서 결과
//   정리 5회 = 약 7초를 그냥 까먹는다 — 유저 실측 총 20초 중). 대신 sim 중 실제 스코어를 띄운다.
// 데이터 = (24)의 시드 기반 슬롯(M_BLUE/M_RED/M_TICK/M_T0). 예상 총 틱은 관측 상한 근사치.
const KS_ON: bool = true;
const KS_TICK_FULL: u64 = 40_000;      // 진행률 분모(실측 29.6k~42.7k의 상단 근사)
static KS_LAST_MSG: Mutex<String> = Mutex::new(String::new());
static KS_LOGGED: AtomicU64 = AtomicU64::new(0);

fn killscore_overlay(ui: &mut GameUI) {
    if !KS_ON || !CONC_ON { return; }
    let n = M_N.load(Ordering::Relaxed) as usize;
    if n == 0 { return; }
    // 주입 결과를 1회 로깅(안 보일 때 원인 파악용)
    if KS_LOGGED.fetch_add(1, Ordering::Relaxed) == 0 {
        let found = (0..ui_inject::KS_MAX)
            .filter(|&i| ui_kit::find_mut(&mut ui.root, &ui_inject::ks_id(i)).is_some()).count();
        let msg_ok = ui_kit::find_mut(&mut ui.root, ui_inject::KS_MSG_ID).is_some();
        log(&format!("[ks] 주입={} 런타임노드 발견={}개 msg={} \n",
            ui_inject::KS_INJECTED.load(Ordering::Relaxed), found, msg_ok));
    }
    let queued = par_lock().len();
    let done = (0..n.min(MAXR)).filter(|&i| M_FIN[i].load(Ordering::Relaxed) != 0).count();
    let started = (0..n.min(MAXR)).filter(|&i| M_T0[i].load(Ordering::Relaxed) != 0).count();
    // 배치가 완전히 끝났으면(전부 완주 + 큐 비었고 기록도 끝) 오버레이를 걷는다.
    let finished = done >= n && queued == 0 && HPUSH_HITS.load(Ordering::Relaxed) >= n as u64;
    // ★기록탭(page 1)으로 넘어갔으면 현황판은 **즉시** 걷는다. v24는 기록 완료 카운트만 봐서
    //   전환 후에도 약 1초간 텍스트가 기록탭 위에 남았다(유저 보고).
    let on_history = unsafe {
        let node = RV_NODE.load(Ordering::Relaxed);
        node != 0 && runner_of(node).map(|r| runner_state(r) == 1).unwrap_or(false)
    };
    let show = !finished && !on_history && started > 0;
    // ⛔**v23 버그**: `set_visible_by_id("loading", !show)`로 썼더니, 우리가 안 띄우는 동안
    //   loading을 **true로 강제**해 결과창 위에 진행바가 겹쳐 보였다(유저 스크린샷).
    //   게임이 스스로 관리하는 노드는 **끄고 싶을 때만 끄고, 켜는 건 게임에 맡긴다.**
    // 현황판이 떠 있는 동안은 게임의 로딩바와 **결과창(summary)** 을 함께 가린다.
    //   병렬에선 결과를 하나씩 되돌리는 사이사이 게임이 결과창을 띄워 현황판과 겹쳐 보였다(유저 보고).
    //   ⚠끄기만 하고 켜지는 않는다 — 켜는 건 게임(refresh)에 맡겨야 다른 화면이 망가지지 않는다.
    if show {
        ui_kit::set_visible_by_id(&mut ui.root, "loading", false);
        ui_kit::set_visible_by_id(&mut ui.root, "summary", false);
    }
    ui_kit::set_visible_by_id(&mut ui.root, ui_inject::KS_MSG_ID, show);
    for i in 0..ui_inject::KS_MAX {
        ui_kit::set_visible_by_id(&mut ui.root, &ui_inject::ks_id(i), show && i < n);
    }
    if !show { return; }
    // 상태 줄
    let msg = if queued > 0 || done < n {
        if done >= n { format!("결과 정리 중  {} / {}", HPUSH_HITS.load(Ordering::Relaxed).min(n as u64), n) }
        else { format!("경기 진행 중  {} / {} 완료", done, n) }
    } else { format!("결과 정리 중  {} / {}", HPUSH_HITS.load(Ordering::Relaxed).min(n as u64), n) };
    {
        let mut last = KS_LAST_MSG.lock().unwrap_or_else(|e| e.into_inner());
        if *last != msg {
            if let Some(node) = ui_kit::find_mut(&mut ui.root, ui_inject::KS_MSG_ID) {
                ui_kit::label_set(node, &msg);
            }
            *last = msg;
        }
    }
    // 경기별 줄
    for i in 0..n.min(ui_inject::KS_MAX) {
        let (b, r) = (M_BLUE[i].load(Ordering::Relaxed), M_RED[i].load(Ordering::Relaxed));
        let t0 = M_T0[i].load(Ordering::Relaxed);
        let tick = M_TICK[i].load(Ordering::Relaxed);
        let fin = M_FIN[i].load(Ordering::Relaxed) != 0;
        let line = if t0 == 0 {
            format!("경기 {}   대기 중", i + 1)
        } else if fin {
            format!("경기 {}   {} : {}   완료", i + 1, b, r)
        } else {
            let pct = ((tick.min(KS_TICK_FULL) * 100) / KS_TICK_FULL).min(99);
            format!("경기 {}   {} : {}   {}%", i + 1, b, r, pct)
        };
        // 끝난 경기는 **승자 팀 색**으로 칠한다(진행 중·대기는 기본 회색).
        //   한 라벨엔 색이 하나뿐이라 점수별 색 분리는 못 하고, 줄 전체를 승패로 물들인다.
        let col = match M_WIN[i].load(Ordering::Relaxed) {
            2 => ui_inject::COL_BLUE,
            1 => ui_inject::COL_RED,
            _ => ui_inject::COL_DIM,
        };
        if let Some(node) = ui_kit::find_mut(&mut ui.root, &ui_inject::ks_id(i)) {
            if ui_kit::label_get(node).as_deref() != Some(line.as_str()) {
                ui_kit::label_set(node, &line);
            }
            ui_kit::label_set_color(node, ui_kit::Rgba::hex(col));
        }
    }
}

// ── (26) ★자체 크래시 로거 (2026-08-08 v17) ────────────────────────────────────────
// v16 병렬 크래시가 **아무 진단도 남기지 않았다**(타 모드 VEH가 못 잡는 형태로 종료) — 원인 후보를
//   4개나 남긴 채 롤백해야 했다. ⟹ 위험한 훅을 켜기 전에 **내 VEH를 먼저 붙인다**.
// ⚠VEH 핸들러 규약(CLAUDE.md §3): 패닉 유발 코드 금지 — **alloc·format!·lock·Vec 전부 금지**.
//   경로는 init에서 미리 UTF-16으로 만들어 두고, 본문은 고정 배열 + 수동 hex 변환 + WriteFile만 쓴다.
const CRASHLOG_ON: bool = true;
static CRASH_PATH: Mutex<Option<Vec<u16>>> = Mutex::new(None);   // init에서만 접근(VEH는 아래 정적 사본)
static mut CRASH_PATH_W: [u16; 520] = [0u16; 520];
static CRASH_PATH_LEN: AtomicUsize = AtomicUsize::new(0);
static CRASH_WROTE: AtomicU64 = AtomicU64::new(0);
static MOD_BASE: AtomicUsize = AtomicUsize::new(0);   // 이 dll의 로드 주소(사고 지점 판별용)

extern "system" {
    fn CreateFileW(name: *const u16, access: u32, share: u32, sa: usize,
                   disp: u32, flags: u32, tmpl: usize) -> usize;
    fn WriteFile(h: usize, buf: *const u8, n: u32, written: *mut u32, ovl: usize) -> i32;
    fn CloseHandle(h: usize) -> i32;
    fn SetFilePointer(h: usize, lo: i32, hi: *mut i32, method: u32) -> u32;
    fn RtlCaptureStackBackTrace(skip: u32, count: u32, frames: *mut usize, hash: *mut u32) -> u16;
}

#[inline] fn hexb(buf: &mut [u8], pos: &mut usize, mut v: u64) {
    const HD: &[u8; 16] = b"0123456789abcdef";
    if *pos + 18 >= buf.len() { return; }
    buf[*pos] = b'0'; buf[*pos + 1] = b'x'; *pos += 2;
    let mut started = false;
    for i in (0..16).rev() {
        let d = ((v >> (i * 4)) & 0xf) as usize;
        if d != 0 { started = true; }
        if started || i == 0 { buf[*pos] = HD[d]; *pos += 1; }
    }
    v = 0; let _ = v;
}
#[inline] fn puts(buf: &mut [u8], pos: &mut usize, s: &[u8]) {
    for &c in s { if *pos < buf.len() { buf[*pos] = c; *pos += 1; } }
}

extern "system" fn crash_veh(p: *mut ExcPointers) -> i32 {
    const SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return SEARCH; }
        let rec = (*p).rec;
        if rec.is_null() { return SEARCH; }
        let code = (*rec).code;
        // 1st-chance 소음 제외: BP(0x80000003)·C++ EH(0xe06d7363)·기타 계속가능 예외는 무시.
        //   진짜 죽는 것들만: AV(0xc0000005)·스택오버플로(0xc00000fd)·불법명령(0xc000001d)·
        //   정렬(0xc000001e/0xc0000096)·0나눗셈 등
        let fatal = matches!(code, 0xc0000005 | 0xc00000fd | 0xc000001d | 0xc000001e
                                 | 0xc0000096 | 0xc0000094 | 0xc000008e | 0xc0000409 | 0xc000041d);
        if !fatal { return SEARCH; }
        // ★v19 오탐 수정: **다른 모드가 의도적으로 발생시키는 예외**까지 잡아 "크래시"로 기록했다
        //   (ai_adjust의 VEH 가드가 그 예 — 실측 addr이 그 모듈 내부였고 게임은 멀쩡했다).
        //   ⟹ 사고 지점이 **게임 exe 또는 이 모드 dll** 안일 때만 기록한다.
        let base0 = GetModuleHandleW(core::ptr::null());
        let mb0 = MOD_BASE.load(Ordering::Relaxed);
        let a0 = (*rec).addr;
        let in_exe = base0 != 0 && a0 > base0 && a0 - base0 < 0x8000000;
        let in_mod = mb0 != 0 && a0 > mb0 && a0 - mb0 < 0x1000000;
        if !in_exe && !in_mod { return SEARCH; }
        if CRASH_WROTE.fetch_add(1, Ordering::Relaxed) >= 3 { return SEARCH; }   // 폭주 방지
        let plen = CRASH_PATH_LEN.load(Ordering::Relaxed);
        if plen == 0 { return SEARCH; }

        let mut buf = [0u8; 1400];
        let mut n = 0usize;
        puts(&mut buf, &mut n, b"\r\n=== CRASH (tfm2_comptest_unlock) ===\r\ncode=");
        hexb(&mut buf, &mut n, code as u64);
        puts(&mut buf, &mut n, b" addr=");
        hexb(&mut buf, &mut n, (*rec).addr as u64);
        let base = GetModuleHandleW(core::ptr::null());
        let mbase = MOD_BASE.load(Ordering::Relaxed);
        puts(&mut buf, &mut n, b"\r\nexe_base=");
        hexb(&mut buf, &mut n, base as u64);
        puts(&mut buf, &mut n, b" mod_base=");
        hexb(&mut buf, &mut n, mbase as u64);
        // ★사고 지점이 exe인지 **이 모드 dll인지**를 반드시 가른다(v17은 mod_base가 없어 미특정이었다)
        if base != 0 && (*rec).addr > base && (*rec).addr - base < 0x8000000 {
            puts(&mut buf, &mut n, b" @exe+");
            hexb(&mut buf, &mut n, ((*rec).addr - base) as u64);
        } else if mbase != 0 && (*rec).addr > mbase && (*rec).addr - mbase < 0x1000000 {
            puts(&mut buf, &mut n, b" @MOD+");
            hexb(&mut buf, &mut n, ((*rec).addr - mbase) as u64);
        }
        if (*rec).nparams >= 2 {
            puts(&mut buf, &mut n, b"\r\naccess=");
            hexb(&mut buf, &mut n, (*rec).params[0] as u64);
            puts(&mut buf, &mut n, b" fault=");
            hexb(&mut buf, &mut n, (*rec).params[1] as u64);
        }
        // 스택 되추적 — 어느 훅에서 왔는지 가른다(모드 shim 주소도 그대로 보인다)
        let mut frames = [0usize; 24];
        let cnt = RtlCaptureStackBackTrace(0, 24, frames.as_mut_ptr(), core::ptr::null_mut());
        puts(&mut buf, &mut n, b"\r\nstack:");
        for i in 0..(cnt as usize).min(24) {
            let f = frames[i];
            puts(&mut buf, &mut n, b"\r\n  ");
            if base != 0 && f > base && f - base < 0x8000000 {
                puts(&mut buf, &mut n, b"exe+"); hexb(&mut buf, &mut n, (f - base) as u64);
            } else if mbase != 0 && f > mbase && f - mbase < 0x1000000 {
                puts(&mut buf, &mut n, b"MOD+"); hexb(&mut buf, &mut n, (f - mbase) as u64);
            } else {
                hexb(&mut buf, &mut n, f as u64);
            }
        }
        puts(&mut buf, &mut n, b"\r\n=== end ===\r\n");

        // append 오픈(GENERIC_WRITE | OPEN_ALWAYS) 후 끝으로 이동
        let h = CreateFileW(core::ptr::addr_of!(CRASH_PATH_W) as *const u16,
                            0x40000000, 0x1 | 0x2, 0, 4, 0x80, 0);
        if h != usize::MAX && h != 0 {
            SetFilePointer(h, 0, core::ptr::null_mut(), 2);   // FILE_END
            let mut w: u32 = 0;
            WriteFile(h, buf.as_ptr(), n as u32, &mut w, 0);
            CloseHandle(h);
        }
        SEARCH
    }
}

unsafe fn install_crash_logger() {
    if !CRASHLOG_ON { return; }
    // 이 dll의 base 확보(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | UNCHANGED_REFCOUNT)
    let mut h: HMODULE = 0;
    if GetModuleHandleExW(0x4 | 0x2, install_crash_logger as *const () as *const u16, &mut h) != 0 {
        MOD_BASE.store(h, Ordering::Relaxed);
    }
    let Some(mut p) = dir() else { return };
    p.push("comptest_crash.txt");
    let s: Vec<u16> = p.as_os_str().encode_wide().chain(core::iter::once(0)).collect();
    let n = s.len().min(519);
    for i in 0..n { CRASH_PATH_W[i] = s[i]; }
    CRASH_PATH_W[n] = 0;
    CRASH_PATH_LEN.store(n, Ordering::Relaxed);
    *CRASH_PATH.lock().unwrap_or_else(|e| e.into_inner()) = Some(s);
    AddVectoredExceptionHandler(1, crash_veh);              // first=1 = 최우선
    log("[crash] VEH 크래시 로거 설치(comptest_crash.txt)\n");
}

// ── (25) ★병렬 발사 + 전량 기록 (2026-08-08 v16) ──────────────────────────────────
// 정본 = RE\2026-08-08_병렬전량기록_경로A_확정.md.
// 구조: N발을 **동시** 발사 → 결과가 몰려 도착 → 앞 결과가 아직 처리 중이면 **그 도착을 통째로
//   보류**(payload를 모드 큐로 move + 조기 리턴) → 매 프레임 슬롯이 비면 큐에서 1건씩 **정상 경로로
//   재주입**. 게임 입장에선 결과가 하나씩 순서대로 온 것과 같아 **히스토리에 N건 전부** 남는다.
// ★소유권: payload(0x268)는 `0xA15E20`이 모든 종료 경로에서 drop한다(호출자는 Box만 free) ⟹
//   **0x268 얕은 복사 = 완전한 move**. 보류 시 원본을 통과시키지 않으므로 이중 drop이 없고,
//   재주입 때 게임이 정상 drop한다. ⛔보류분에 `0xA46880`(payload drop)을 부르면 안 된다.
// ★훅 지점은 둘 다 **프레임 확보 이후의 함수 중간**(v6 크래시 = 프레임 확보 *전* 진입점 훅).
//   패치는 `FF 25`(rip-relative 간접 점프) = **레지스터 무클로버**(movabs rax 방식 금지 — rax 생존 필요).
// ⛔**2026-08-08 05:2x 인게임 크래시로 OFF 복귀.** 증상 = 5발 병렬 발사까지는 정상
//   (`[conc] 발사 #1~#4 제출 OK`, len 1→5), **그 직후 결과 도착 시점에 게임 사망**. ai_adjust VEH
//   crash_log에 신규 항목 없음(= 핸들러가 못 잡는 형태로 종료).
//   ⬜미규명 후보: ①naked shim의 `jmp qword ptr [rip+{sym}]` 실효성 ②조기리턴 시 에필로그
//   `0xa161c0` 진입 rsp 전제(프롤로그에 `and rsp` 유무 미확인 — `0x2327080`과 다를 수 있음)
//   ③드라이버가 넘기는 (p2,p3,p4)가 실제로 `0xA15E20` 인자와 일치하는지 ④재주입 시 재진입.
//   ⟹ 다음 시도 전 **지연 훅만 단독으로**(드라이버 없이) 켜서 어느 쪽이 원인인지 가를 것.
// ★v17: 게이트를 **3개로 분리**한다. v16은 지연·드라이버를 한 번에 켜서 크래시 원인을 못 갈랐다
//   (RE 문서 자체가 "지연 훅만 먼저" 권고했는데 어긴 것 = 03_시행착오 교훈).
// ⛔**2026-08-08 06:1x — 훅 지점을 `0x2327094`로 옮긴 뒤 "세이브 로드만 해도" 크래시**(VEH 미기록
//   = 스택 파괴 수준). 이전 지점(`0xa15e48`)에서는 지연·재주입까지 정상 동작했으므로 **지점 이동분에
//   결함이 있다.** 원인 미규명 상태로 유저가 게임을 못 쓰게 되어 즉시 OFF.
//   ⬜다음 시도 전 확인: ①`0x2327080`이 comp_test 외 경로에서도 호출되는지(세이브 로드 중 발화?)
//   ②`jmp qword ptr [rip+{sym}]`가 의도대로 어셈블되는지(디스어셈으로 실물 확인)
//   ③스틸 15B가 정말 `0x2327094`에 정확히 얹혔는지(설치 후 재읽기 검증 추가)
//   ④통과 경로의 `and rsp,-0x80` 이후 rsp가 원본과 동일한지.
//   ⟹ **순차 방식(v25까지)은 정상 동작**하므로 그것이 안전 기준선이다.
// ★★2026-08-08 최종 판정 — **병렬과 기록 정확도는 현재 구조상 양자택일**이다.
//   `0x2327080`(결과 도착)의 **완주가 서버의 "다음 경기 sim 시작" 트리거**다(v31·v56 실측 일치).
//     · 도착을 **통과**시키면 → sim은 병렬로 돌지만, 다음 도착이 이전 결과를 덮어쓴다.
//       이때 슬롯만 훔치면 `+0x2280`(점수·승패)이 아직 없어 **키메라 엔트리**(A 시드 + B 점수)가 된다.
//     · 도착을 **보류**하면 → 키메라는 없지만 트리거가 끊겨 **sim이 순차**가 된다.
//   ⟹ 지금은 **정확한 쪽(순차)** 을 기본으로 둔다. 순차는 기록·리플레이 모두 검증 완료.
//   ⬜병렬+정확을 동시에 얻으려면: 도착을 통과시키되 **`+0x2280` 생성기(`0x141a18460`)를 모드가
//     직접 호출해 3조각을 즉시 완성**시킨 뒤 훔치는 방식이 필요(미구현·미검증).
// ⛔v58 결과(2026-08-08 실측): 병렬은 유지됐으나 **결과 정리가 중간에 멈추고 리플레이도 여전히 불일치**.
//   원인 추정 = `+0x20f0`(Rc 핸들) **소비 시맨틱**. 생성기 `0x1a18460`은 그 핸들을 take 하는 것으로
//   보이는데, 우리가 A의 핸들을 미리 소비해버리면 이어 도착한 B의 결과상세를 게임이 만들지 못한다
//   ⟹ B의 기록이 bail(정리 중단) + 짝 어긋남 지속. ⬜정확한 소비 규칙 미확인 → RE 위임.
const PARALLEL_ON: bool = true;      // 병렬 발사(클릭 시 N발 동시)
const PAR_DELAY_ON: bool = true;     // 빼내기 훅 0x2327094
// ✅**v19 실측 = 보류 경로도 안전**(`seen=5 큐=4`, 크래시 0, 기록 1건 = 설계대로).
//   ⟹ v16 크래시의 범인은 **재주입(드라이버)** 하나로 확정. 이번엔 훅을 켜되 **호출은 하지 않고
//   인자만 비교**해서, RE가 주장한 `(p2,p3,p4)` 매핑이 실제로 맞는지 먼저 검증한다.
const PAR_DRIVE_ON: bool = true;     // 되돌리기(러너가 비면 보관분 1건을 원형 복원)
// ✅v21 실측: 정렬 수정 후 **크래시 0** + 인자 3개 전부 일치(`p2✓ p3✓ p4✓`) ⟹ 재주입을 켠다.
const PAR_DRIVE_DRYRUN: bool = false;
// ★재진입 가드: 재주입으로 부른 `0xA15E20` 안에서 지연 훅이 **다시** 발화하는데, 그때 큐가 비어있지
//   않으면 그 결과까지 보류해버려 **영원히 소진되지 않는다**(자기 자신을 다시 큐에 넣는 꼴).
//   ⟹ 재주입 중에는 지연 판단을 무조건 통과시킨다.
static PAR_REPLAYING: AtomicBool = AtomicBool::new(false);
// ★v18: 지연 훅 단독으로도 크래시(v17 실측)했으므로 **한 단계 더 쪼갠다**.
//   true = 판단 함수가 **항상 0(통과)** 을 반환 = shim의 "통과 경로"만 태운다
//     ⟹ 이게 죽으면 원인은 **shim/스틸 재현/점프**(보류 로직 무관).
//     ⟹ 이게 살면 원인은 **보류 경로**(payload 복사 or 조기리턴 점프).
//   ✅**v18 실측 = shim 뼈대 정상**(`par seen=5`, 크래시 0, forge 5/result 5). 통과 경로는 안전하다.
//   ⟹ v19에서 **보류 경로**를 켠다(드라이버는 계속 OFF). 여기서 죽으면 원인 = payload 복사 or 조기리턴 점프.
// ✅**v28 실측 = 지점 이동 성공**. `and rsp,-0x80`을 원본 바이트로 박으니 크래시가 사라졌고,
//   무엇보다 **5경기가 진짜 병렬로 돌았다**(현황판에 경기1~5가 모두 "완료"). 통과 경로 검증 완료.
//   ⟹ v29에서 보류를 켠다. 이제 sim은 병렬로 끝난 뒤 결과만 줄 세워 기록된다.
const PAR_OBSERVE_ONLY: bool = false;
// ⚠PAR_DRIVE_ON=false면 보류된 결과가 **재주입되지 않는다** ⟹ 기록은 1건만 남는 게 정상이다.
//   이번 목적은 "지연 훅만으로 크래시가 나는가"를 가르는 것뿐.
// ★★v26 — 가로채기 지점을 **뒤로 옮긴다**(진짜 병렬의 핵심).
//   구: `0xa15e48`(= `0x235bf20` Game 생성 **이전**) ⟹ 보류하면 **sim이 시작조차 안 해서** 경기가
//       한 개씩 돌았다(유저 관측 "경기 진행은 한 경기씩").
//   신: `0x2327094`(= 결과 도착 함수가 프레임을 확보한 직후) ⟹ **sim은 이미 5개 다 돌고 있고**,
//       우리는 러너 슬롯 덮어쓰기만 막는다.
//   조기 리턴 = `lea rbp,[rsp-0x4240]` 후 **`0x232790a`**(함수의 유일한 에필로그: `xor eax,eax` →
//       `lea rsp,[rbp+0x4240]` → pop×7 → ret)로 점프. 훅 지점이 **모든 소유권 등록 이전**이라
//       이 경로는 detail/entry를 drop하지 않는다 = 우리가 소유권을 가져간다.
//   ⚠호출자가 detail/entry를 **자기 스택 버퍼**에 담아 넘기므로 리턴 즉시 사라진다 ⟹ 반드시 복사.
const DELAY_RVA: usize = 0x1aab964;   // ★0.5.5(구0.5.4=0x2327094) 컨테이너 difflib(cont 0x2327080→0x1aab950)·DELAY_ORIG 15B 실측 MATCH
const DELAY_ORIG: [u8; 15] = [0x48,0x29,0xc4, 0x48,0x8d,0xac,0x24,0x80,0x00,0x00,0x00,
                              0x48,0x83,0xe4,0x80];
const DELAY_RESUME_RVA: usize = 0x1aab973;   // ★0.5.5(구0.5.4=0x23270a3) 컨테이너 difflib(mov rbx,rsp)
const EPILOG_RVA: usize = 0x1aac1ca;     // ★0.5.5(구0.5.4=0x232790a) 컨테이너 difflib(xor eax,eax) // → lea rsp,[rbp+..] → pop×7 → ret
const ARRIVE_FN_RVA: usize = 0x1aab950;  // ★0.5.5(구0.5.4=0x2327080) 투표 37/37 // 재주입 시 직접 호출할 결과 도착 함수
const DETAIL_SZ2: usize = 0x20a0;
const ENTRY_SZ2: usize = 0xe0;
const DRIVE_RVA: usize = 0x7a9da0;   // ★0.5.5(구0.5.4=0xa289c0) 컨테이너델타 UNIQUE(off=0x60)·DRIVE_ORIG 22B 실측 MATCH
const DRIVE_ORIG: [u8; 22] = [0x48,0xc7,0x85,0xa8,0x6b,0x01,0x00,0xff,0xff,0xff,0xff,
                              0x48,0xc7,0x85,0x70,0x20,0x00,0x00,0xff,0xff,0xff,0xff];
const DRIVE_RESUME_RVA: usize = 0x7a9db6;   // ★0.5.5(구0.5.4=0xa289d6) 컨테이너델타(off=0x76)
const A15E20_RVA: usize = 0x7970f0;   // ★0.5.5(구0.5.4=0xa15e20) 다중앵커 투표 50/50 size974→988
const PAYLOAD_SZ: usize = 0x268;
const OUT_SZ: usize = 0x740;

static PAR_RESUME: AtomicUsize = AtomicUsize::new(0);   // = base + DELAY_RESUME_RVA
static PAR_EPILOG: AtomicUsize = AtomicUsize::new(0);   // = base + EPILOG_RVA
static PAR_DRESUME: AtomicUsize = AtomicUsize::new(0);  // = base + DRIVE_RESUME_RVA
static PAR_QUEUED: AtomicU64 = AtomicU64::new(0);
static PAR_REPLAYED: AtomicU64 = AtomicU64::new(0);
static PAR_OUTBUF: AtomicUsize = AtomicUsize::new(0);   // 재주입용 0x740 out 슬롯(모드 소유)

// 보류된 결과 큐. 원소 = (detail 0x20a0, entry 0xe0) 바이트 사본 = **완전한 move**.
//   ⚠호출자 스택 버퍼라 리턴 즉시 사라지므로 반드시 복사해 나가야 한다.
// ★v33 — 결과 1건이 기록되려면 러너의 **세 조각이 세트**로 있어야 한다:
//   `+0x21a0`(엔트리 0xE0) · `+0x2280`(결과상세 0xA0) · `+0x20f0/+0x20f8`(Rc 핸들 16B).
//   v32는 엔트리만 되돌려서, 두 번째부터 `+0x2280`을 만들 수 없었다(팝업 draw가 그걸
//   `+0x20f0`에서 만드는데 그 Rc는 **첫 결과 때 이미 소비**됨) ⟹ `0x230c910`이 즉시 bail = 기록 1건.
//   ⟹ 세 조각을 함께 빼돌리고 함께 되돌린다.
const DETAIL_B_OFF: usize = 0x2280;
const DETAIL_B_SZ: usize = 0xa0;
const RCH_OFF: usize = 0x20f0;
// ⛔`0x1a18460`은 **결과상세 생성기가 아니다** — RE 확정(2026-08-08): crossbeam bounded(1) 채널의
//   **`try_recv`**(= 메시지를 꺼내 비우는 함수, 실체 `0x20d4190`)다. v58이 이걸 "만드는 함수"로
//   오해하고 미리 부르는 바람에 **게임이 열어볼 편지를 모드가 먼저 꺼내가** 이후 영원히 None →
//   `+0x2280`을 못 만들어 기록 정지. ⟹ **모드는 이 함수를 부르지 않는다.**
// ★★v56 전면 수정 — 리플레이 불일치의 진범은 **키메라 엔트리**였다.
//   기록 엔트리(0xE0)는 러너의 **두 슬롯을 합성**해서 만들어진다:
//     · `+0x21a0` 계보 → **seed(+0xA8)**, participants, champions
//     · `+0x2280` 계보 → **blue/red_score, players, blue_win, 전략, 타워**
//   그런데 `+0x2280`은 결과 도착 시점엔 **비어 있고**(도착 함수가 -1로 리셋), 다음 프레임의 팝업
//   draw가 `+0x20f0`에서 만들어 낸다. v33~v55는 도착 시점에 슬롯을 훔쳤으므로 `+0x2280`을
//   챙기지 못했고, 되돌릴 때 **A의 시드 + B의 점수**가 한 엔트리에 섞였다 ⟹ 리플레이(A 시드로 재시뮬)와
//   화면 점수(B)가 달랐다. 병렬일 때만 결과가 2건 이상 겹치므로 `runs=1`에선 멀쩡했던 것도 정확히 설명된다.
//   ⟹ **슬롯을 훔치지 않는다.** 대신 **도착 자체를 보류**했다가(payload 통째로) 나중에 도착 함수를
//   재구동해 **게임이 3조각을 스스로 만들게** 한다. 그러면 짝이 구조적으로 어긋날 수 없다.
//   ⚠sim은 이 지점 이전(`0x235bf20` Game 생성)에 이미 시작되므로, 보류해도 **병렬은 유지**된다.
// ★★v60 확정 설계 (RE 2026-08-08 근거) — "우편함째로 보관했다가 통째로 되돌린다"
//   · `0x2327080`(도착)은 경기 **종료**가 아니라 **시작** 함수다: bounded(1) 채널을 만들고
//     `std::thread::spawn`으로 sim 스레드를 띄운 뒤, 러너에 (entry, Receiver)를 싣는다.
//   · 결과는 나중에 팝업 draw(`0x23cd370`)가 `try_recv`로 꺼내 `+0x2280`을 만들고, 기록
//     `0x230c910`이 `+0x2280`을 **take**(1-shot)해서 엔트리와 병합·송신한다.
//   ⟹ 병렬의 진짜 병목: 러너 슬롯이 **1인용**이라, 연달아 시작하면 앞 경기의 Receiver가
//      덮여 drop되고 **그 경기 결과가 통째로 소멸**했다(스레드는 돌지만 수신구가 사라짐).
//   ⟹ 해법: 덮이기 직전에 **(entry + Receiver [+ 이미 만들어진 detail])을 한 원소로** 빼내
//      보관하고, 러너가 비면 한 원소씩 원형 그대로 되돌린다. **모드는 결과를 만들거나 꺼내지 않는다**
//      — 게임이 자기 함수로 꺼내게 둔다. 채널 cap=1 & 도착마다 새 채널이므로 **짝이 어긋날 경로가 없다**.
//   소유권: Receiver variant0의 16B 복사 + 원본 `-1` = 순수 move(clone 아님), entry 0xE0도 동일.
//      원본을 `-1`로 만들면 게임의 drop이 스킵되므로(`0x23276e6`/`0x230d0a9`) 이중 해제 없음.
pub struct ParItem {
    e: Box<[u8; ENTRY_SZ2]>,      // 엔트리 0xE0 (시드·참가자·챔피언)
    has_e: bool,
    rc: [u64; 2],                 // Receiver 16B (태그 + 페이로드 ptr) — 이 경기 전용 우편함
    d2: Box<[u8; DETAIL_B_SZ]>,   // 이미 꺼내져 있던 결과상세 0xA0 (있을 때만)
    has_d2: bool,
    node: usize,
}
static PAR_QUEUE: std::sync::Mutex<Vec<ParItem>> = std::sync::Mutex::new(Vec::new());

#[inline] fn par_lock() -> std::sync::MutexGuard<'static, Vec<ParItem>> {
    PAR_QUEUE.lock().unwrap_or_else(|e| e.into_inner())
}

// 현재 comp_test 러너를 얻는다(캡처해둔 node 후보 2개를 매번 재검증 — stale이면 자동 탈락).
unsafe fn par_runner() -> usize {
    for n in [RV_NODE.load(Ordering::Relaxed), WATCH_NODE.load(Ordering::Relaxed)] {
        if n != 0 { if let Some(r) = runner_of(n) { return r; } }
    }
    0
}
// 러너가 아직 결과를 물고 있는가(= 지금 도착하면 덮어써진다)
#[inline] unsafe fn par_busy(runner: usize) -> bool {
    if runner == 0 { return false; }
    if runner_state(runner) == 4 { return true; }
    if readable(runner + 0x21a0, 8) {
        return core::ptr::read_unaligned((runner + 0x21a0) as *const u64) != u64::MAX;
    }
    false
}

/// 지연 판단. 1 = 이번 도착을 보류(원본 스킵), 0 = 통과.
static PAR_SEEN: AtomicU64 = AtomicU64::new(0);

const PAR_AZ: AtomicUsize = AtomicUsize::new(0);
static PAR_A: [AtomicUsize; 3] = [PAR_AZ; 3];       // 자연 호출 시점의 (p2,p3,p4) 기준값

/// 지연 판단. 1 = 이번 도착을 보류(조기 리턴), 0 = 통과.
/// 인자 = `0x2327094` 시점의 (rcx=a1, rdx=node, r8=&detail(0x20a0), r9=&entry(0xe0)).
extern "win64" fn par_delay_check(a1: usize, node: usize, pd: usize, pe: usize) -> u8 {
    let n = PAR_SEEN.fetch_add(1, Ordering::Relaxed);   // shim이 여기까지 왔다는 증거
    if n < 4 { log(&format!("[par] 도착#{} a1=0x{:x} node=0x{:x}\n", n, a1, node)); }
    // (승패 채집은 `hpush_detour`에서 한다 — 이 시점의 `+0xD9`는 아직 최종값이 아니다.)
    if PAR_OBSERVE_ONLY { return 0; }               // ★관찰 전용: 통과 경로만 태운다
    if PAR_REPLAYING.load(Ordering::Relaxed) { return 0; }   // ★재주입 중 = 무조건 통과
    // ★★★v32 — **조기 리턴을 폐기한다.** 실측(v31)이 결정적이었다:
    //   경기2 sim이 **경기1의 결과가 기록된 뒤에야** 시작됐다(+86ms / +5894ms / +11800ms / +17864ms
    //   / +25444ms — 정확히 재주입·hpush 직후마다 다음 sim 시작). 즉 **결과 처리(`0x2327080` 완주)가
    //   다음 경기 sim의 트리거**다. 우리가 조기 리턴하면 그 트리거가 사라져 sim이 직렬화된다.
    //   ⟹ 유저 관측 "저장 안 하는 설정(관찰 전용)일 땐 5경기 동시에 돌았다"와 완전히 일치.
    // 새 방식: **원본은 항상 통과시키고**(= sim 병렬 유지), 덮어쓰기 **직전에 이전 결과를 빼돌린다**.
    //   러너 슬롯 `+0x21a0`이 차 있으면 그 0xE0을 큐로 복사한 뒤 슬롯을 `-1`로 만든다 ⟹
    //   원본의 `0x23276f7`(기존 값 drop)이 **-1이라 스킵**되므로 이중 해제도 없고, 새 결과는 정상 기록된다.
    // ★★v60 — **통과시키되(병렬 유지) 덮이기 직전에 "엔트리 + 우편함"을 한 쌍으로 빼낸다.**
    //   v33~55: 엔트리만 챙김 → 우편함이 죽어 결과 소멸/키메라.   v56: 조기 리턴 → 순차 퇴행.
    //   v58: 우편함을 모드가 먼저 열어버림(try_recv) → 게임이 영원히 None → 기록 정지.
    //   v60: **아무것도 열지 않는다.** 우편함을 봉인된 채로 보관했다가 되돌려주면 게임이 연다.
    let _ = (a1, pd, pe);
    let _ = std::panic::catch_unwind(|| unsafe {
        if !par_on() { return; }
        // ★comp_test 러너인지 확인 = 타입태그 검사(`*(node+0x238)==base+0x33b91f8`).
        //   이 훅 지점은 도착 함수의 다운캐스트 게이트보다 **앞**이라, 이 검사를 우리가 대신 한다.
        let runner = match runner_of(node) { Some(r) => r, None => return };
        if !readable(runner + 0x21a0, ENTRY_SZ2) || !readable(runner + RCH_OFF, 16) { return; }
        let e_head = core::ptr::read_unaligned((runner + 0x21a0) as *const u64);
        let rc_tag = core::ptr::read_unaligned((runner + RCH_OFF) as *const u32);   // ★dword 니치
        if e_head == u64::MAX && rc_tag == u32::MAX { return; }   // 실을 게 없음(첫 도착)
        let mut q = par_lock();
        if q.len() >= 32 { return; }
        // ① 엔트리(0xE0) move-out
        let mut e: Box<[u8; ENTRY_SZ2]> = Box::new([0u8; ENTRY_SZ2]);
        let has_e = e_head != u64::MAX;
        if has_e {
            core::ptr::copy_nonoverlapping((runner + 0x21a0) as *const u8, e.as_mut_ptr(), ENTRY_SZ2);
            core::ptr::write_unaligned((runner + 0x21a0) as *mut u64, u64::MAX);
        }
        // ② Receiver(16B) move-out — **열지 않고 그대로** 보관한다
        let mut rc = [u64::MAX, 0u64];
        if rc_tag != u32::MAX {
            rc[0] = core::ptr::read_unaligned((runner + RCH_OFF) as *const u64);
            rc[1] = core::ptr::read_unaligned((runner + RCH_OFF + 8) as *const u64);
            core::ptr::write_unaligned((runner + RCH_OFF) as *mut u64, u64::MAX);
        }
        // ③ 이미 꺼내져 있던 결과상세(0xA0)가 있으면 그것도 함께(같은 경기 것이 확실)
        let mut d2: Box<[u8; DETAIL_B_SZ]> = Box::new([0u8; DETAIL_B_SZ]);
        let mut has_d2 = false;
        if readable(runner + DETAIL_B_OFF, DETAIL_B_SZ)
            && core::ptr::read_unaligned((runner + DETAIL_B_OFF) as *const u64) != u64::MAX {
            core::ptr::copy_nonoverlapping((runner + DETAIL_B_OFF) as *const u8,
                                           d2.as_mut_ptr(), DETAIL_B_SZ);
            core::ptr::write_unaligned((runner + DETAIL_B_OFF) as *mut u64, u64::MAX);
            has_d2 = true;
        }
        q.push(ParItem { e, has_e, rc, d2, has_d2, node });
        PAR_QUEUED.fetch_add(1, Ordering::Relaxed);
    });
    0   // ★항상 통과 — 원본이 완주해야 다음 경기 sim이 시작된다
}

/// 매 프레임 호출 — 슬롯이 비었으면 큐에서 1건을 정상 경로로 재주입.
static PAR_DRYLOG: AtomicU64 = AtomicU64::new(0);

extern "win64" fn par_drive_rust(p2: usize, p3: usize, p4: usize) {
    let _ = std::panic::catch_unwind(|| unsafe {
        if !PAR_DRIVE_ON { return; }
        if par_lock().is_empty() { return; }
        // ★DRYRUN: 호출 대신 **인자 일치 여부만** 검증한다(v16 크래시의 유일한 남은 후보).
        if PAR_DRIVE_DRYRUN {
            if PAR_DRYLOG.fetch_add(1, Ordering::Relaxed) < 3 {
                let (a2, a3, a4) = (PAR_A[0].load(Ordering::Relaxed),
                                    PAR_A[1].load(Ordering::Relaxed),
                                    PAR_A[2].load(Ordering::Relaxed));
                log(&format!("[par] DRY 드라이버 p2=0x{:x}{} p3=0x{:x}{} p4=0x{:x}{} (큐={})\n",
                    p2, if p2 == a2 { "✓" } else { "✗" },
                    p3, if p3 == a3 { "✓" } else { "✗" },
                    p4, if p4 == a4 { "✓" } else { "✗" },
                    par_lock().len()));
            }
            return;
        }
        let _ = (p2, p3, p4);
        // ★v56 재주입 = **도착 함수를 그 payload로 다시 부른다.** 슬롯을 직접 만지지 않으므로
        //   게임이 3조각(엔트리·결과상세·Rc)을 스스로 만들어 **짝이 절대 어긋나지 않는다**.
        //   ⚠`0x2327080`은 17KB 프레임이라 깊은 스택에서 부르면 터진다 — 이 드라이버(`0xa289c0`)는
        //   프레임 루프 초입이라 여유가 충분하다(RE 확인).
        // ★★v60 되돌리기 = 보관한 원소를 **원형 그대로** 러너에 되돌린다(엔트리 + 우편함 [+ 결과상세]).
        //   그 다음은 게임이 평소대로 한다: draw가 우편함을 열어 `+0x2280`을 만들고, 기록이 take해서 송신.
        //   ⟹ 모드는 결과를 만들지도 꺼내지도 않으므로 **짝이 어긋날 경로가 원천적으로 없다**.
        let runner = par_runner();
        if runner == 0 { return; }
        if !readable(runner + 0x21a0, ENTRY_SZ2) || !readable(runner + RCH_OFF, 16)
            || !readable(runner + DETAIL_B_OFF, DETAIL_B_SZ) || !readable(runner + 0x240c, 1) { return; }
        // 러너가 완전히 비었을 때만 싣는다(진행 중인 결과를 밀어내면 그게 곧 소멸이다).
        //   기록이 끝나면 `0x230d0c7`이 `+0x21a0`을 -1로, draw가 `+0x20f0`을 -1로 되돌려 게이트가 열린다.
        if core::ptr::read_unaligned((runner + 0x21a0) as *const u64) != u64::MAX { return; }
        if core::ptr::read_unaligned((runner + RCH_OFF) as *const u32) != u32::MAX { return; }
        if core::ptr::read_unaligned((runner + DETAIL_B_OFF) as *const u64) != u64::MAX { return; }
        let item = { let mut q = par_lock(); if q.is_empty() { return; } q.remove(0) };
        if item.has_e {
            core::ptr::copy_nonoverlapping(item.e.as_ptr(), (runner + 0x21a0) as *mut u8, ENTRY_SZ2);
        }
        if item.rc[0] != u64::MAX {
            core::ptr::write_unaligned((runner + RCH_OFF) as *mut u64, item.rc[0]);
            core::ptr::write_unaligned((runner + RCH_OFF + 8) as *mut u64, item.rc[1]);
        }
        if item.has_d2 {
            core::ptr::copy_nonoverlapping(item.d2.as_ptr(), (runner + DETAIL_B_OFF) as *mut u8, DETAIL_B_SZ);
        }
        core::ptr::write_unaligned((runner + 0x240a) as *mut u8, 1);      // 신규결과 플래그
        core::ptr::write_unaligned((runner + 0x2404) as *mut f32, 0.0);   // 진행도 리셋
        core::ptr::write_unaligned((runner + 0x240c) as *mut u8, 4);      // 상태 = 결과 대기/도착
        core::mem::forget(item);                              // 내부 힙 소유권이 게임으로
        let k = PAR_REPLAYED.fetch_add(1, Ordering::Relaxed);
        if k < 12 { log(&format!("[par] 되돌림 #{} (남은 큐={})\n", k, par_lock().len())); }
    });
}

#[unsafe(naked)]
unsafe extern "win64" fn par_delay_shim() {
    // 진입 시점(0xA15E48): rcx=p1, rdx=p2, r8=p3, r9=p4, rbp 유효, [rbp+0x4320]=p5, 프레임 확보 완료
    // 진입(0x2327094): rcx=a1, rdx=node, r8=&detail, r9=&entry, **rax=0x42C0**(스틸 명령이 씀),
    //   rsp ≡ 0(mod 16) — 0x2327080이 push 7개(56B) + chkstk 뒤이므로.
    //   push 4개(32B) + sub 0x20 → call 시점 rsp ≡ 8 = Win64 ABI 충족.
    // ★`FF 25` 무클로버 점프로 진입하므로 **rax(0x42C0)가 살아 있다** → 통과 경로에서 `sub rsp,rax` 그대로 사용 가능.
    core::arch::naked_asm!(
        "push rcx", "push rdx", "push r8", "push r9",
        "sub rsp, 0x20",
        "call {chk}",
        "add rsp, 0x20",
        "pop r9", "pop r8", "pop rdx", "pop rcx",
        "test al, al",
        "jne 2f",
        // 통과 = 스틸한 원본 3명령 재현 후 복귀.
        // ⛔v26 크래시 원인①: `sub rsp, rax`를 그대로 썼다가 위 `call {chk}`가 rax를 반환값으로
        //   덮어써 스택이 안 깎였다 ⟹ 프레임 크기를 **상수로**.
        // ⛔v27 크래시 원인 후보②(이번 수정): **`and rsp, -0x80`의 인코딩**. 원본은
        //   `48 83 E4 80`(imm8 부호확장 = -128)인데, 어셈블러가 `-0x80`을 imm32로 잡으면
        //   `48 81 E4 80 FF FF FF` = `and rsp, 0xFFFFFF80` **상위 32비트가 0으로 잘려** rsp가
        //   4GB 아래로 날아간다 ⟹ 즉사(VEH도 못 돎, 세이브 로드만으로도 죽는 증상과 정합).
        //   ⟹ **원본과 동일한 바이트를 직접 박는다.**
        "sub rsp, 0x42c0",
        "lea rbp, [rsp+0x80]",
        ".byte 0x48, 0x83, 0xe4, 0x80",   // and rsp, -0x80  (원본 바이트 그대로)
        "jmp qword ptr [rip + {res}]",
        // 보류 = 에필로그가 요구하는 rbp만 세팅하고 함수의 유일한 리턴 경로로 점프.
        //   이 시점은 detail/entry가 러너에 등록되기 **전**이라 게임이 drop하지 않는다 = 소유권이 우리 것.
        "2:",
        "lea rbp, [rsp-0x4240]",
        "jmp qword ptr [rip + {epi}]",
        chk = sym par_delay_check,
        res = sym PAR_RESUME,
        epi = sym PAR_EPILOG,
    );
}

#[unsafe(naked)]
unsafe extern "win64" fn par_drive_shim() {
    // 진입 시점(0xA289C0): rcx=p2, rdx=p3, [rbp+0x17470]=p4
    core::arch::naked_asm!(
        // ★스택 정렬(v20 크래시의 원인): `0xa28960`은 push **8개** + `sub rsp,0x17488`(%16=8)이라
        //   `0xa289c0` 시점 rsp ≡ 0(mod 16)이다. 여기서 push 6개(≡0) 후 `sub 0x28`(40B, %16=8)을 하면
        //   call 시점에 rsp ≡ 0 → **Win64 ABI 위반**(callee 진입 시 8이어야 함) → Rust 함수 내부에서 AV.
        //   ⟹ `sub 0x20`(shadow space만, %16=0)이 정답. 지연 shim은 push 4 + sub 0x30으로 이미 맞았고,
        //   그래서 v19(지연만)는 멀쩡했는데 v20(드라이버 켬)만 죽었다.
        "push rcx", "push rdx", "push r8", "push r9", "push r10", "push r11",
        "mov r8, [rbp+0x17470]",
        "sub rsp, 0x20",
        "call {drv}",
        "add rsp, 0x20",
        "pop r11", "pop r10", "pop r9", "pop r8", "pop rdx", "pop rcx",
        // 스틸한 원본 2명령 재현
        "mov qword ptr [rbp+0x16ba8], -1",
        "mov qword ptr [rbp+0x2070], -1",
        "jmp qword ptr [rip + {res}]",
        drv = sym par_drive_rust,
        res = sym PAR_DRESUME,
    );
}

/// 함수 **중간**에 14B `FF 25` 간접 점프를 심는다(레지스터 무클로버). 나머지는 NOP.
unsafe fn install_mid(rva: usize, orig: &[u8], target: usize) -> Result<String, String> {
    let base = exe_base(); if base == 0 { return Err("module 0".into()); }
    let n = orig.len();
    if n < 14 || n > 32 { return Err("steal len".into()); }
    let a = base + rva;
    if !readable(a, n) { return Err(format!("unreadable 0x{:x}", rva)); }
    let mut cur = [0u8; 32];
    core::ptr::copy_nonoverlapping(a as *const u8, cur.as_mut_ptr(), n);
    if &cur[..n] != orig {
        return Err(format!("byte mismatch @0x{:x} cur={:02x?}", rva, &cur[..n]));
    }
    let mut patch = [0x90u8; 32];
    patch[0] = 0xff; patch[1] = 0x25;                 // jmp qword ptr [rip+0]
    patch[2] = 0; patch[3] = 0; patch[4] = 0; patch[5] = 0;
    patch[6..14].copy_from_slice(&target.to_le_bytes());
    let mut old: u32 = 0;
    if VirtualProtect(a, n, 0x40, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), a as *mut u8, n);
    VirtualProtect(a, n, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), a, n);
    // ★설치 후 실물 재읽기 검증(§10 증거 규칙) — 엉뚱한 곳에 얹혔는지 로그로 남긴다
    let mut back = [0u8; 32];
    core::ptr::copy_nonoverlapping(a as *const u8, back.as_mut_ptr(), n.min(16));
    Ok(format!("mid-hook 0x{:x} ({}B) landed={:02x?} tgt=0x{:x}", rva, n, &back[..14], target))
}

unsafe fn install_parallel() {
    if !PAR_DELAY_ON && !PAR_DRIVE_ON { log("[par] 병렬 훅 OFF\n"); return; }
    let base = exe_base(); if base == 0 { return; }
    PAR_RESUME.store(base + DELAY_RESUME_RVA, Ordering::Relaxed);
    PAR_EPILOG.store(base + EPILOG_RVA, Ordering::Relaxed);
    PAR_DRESUME.store(base + DRIVE_RESUME_RVA, Ordering::Relaxed);
    // 재주입용 out 슬롯(게임은 여기에 -1만 쓴다)
    let layout = std::alloc::Layout::from_size_align(OUT_SZ, 16).unwrap();
    let p = std::alloc::alloc_zeroed(layout);
    if !p.is_null() { PAR_OUTBUF.store(p as usize, Ordering::Relaxed); }
    if PAR_DELAY_ON {
        match install_mid(DELAY_RVA, &DELAY_ORIG, par_delay_shim as usize) {
            Ok(s) => log(&format!("[par] delay {}\n", s)),
            Err(e) => log(&format!("[par] delay 실패: {}\n", e)),
        }
    }
    if PAR_DRIVE_ON {
        match install_mid(DRIVE_RVA, &DRIVE_ORIG, par_drive_shim as usize) {
            Ok(s) => log(&format!("[par] drive {}\n", s)),
            Err(e) => log(&format!("[par] drive 실패: {}\n", e)),
        }
    } else {
        log("[par] drive OFF — 보류분 재주입 없음(기록 1건이 정상)\n");
    }
}

// ── (10) [진단] dedup insert / spawn copy 카운터 (스폰이 원본순회(b) vs dedup순회(a) 판정) ──
//   dedup_insert 0x140ca75f0(14B) / spawn_copy 0x1413c71b0(19B, 0x768 deep-copy). 무-IO 원자카운트.
//   중복10명: dedup +10(매삽입), spawn +10=(b)원본순회=GO / spawn +1=(a)dedup순회=NO-GO.
// ⛔★두 상수 모두 **죽은 상수 — 0.5.1 exe 실측으로 신규 발견(2026-07-22 마이그)**. 종전 주석엔 STALE 표기가
//   없었으나, 0.5.1에서 DEDUP_INS_RVA는 0xca6b20 함수 중간(바이트 `84 3e 3f 01 ...`), SPAWN_CP_RVA는
//   0x13c4e90 함수 중간(`66 0f 6e c1 ...`)이라 **선언 프롤로그와 불일치 = install_hook_n이 Err = 미설치**.
//   ⇒ 이 두 진단 카운터는 0.5.1 내내 0이었다(로그를 그렇게 읽지 말 것). 0.5.2에서도 프롤로그 불일치 = inert.
//   재핀하려면 ghidra-re 필요(SwissTable insert 모노모픽 copy 다수 = 정적 변별 난이도 높음).
const DEDUP_INS_RVA: usize = 0xca75f0;   // ⬜미확정(죽은 상수·inert 확인필·0.5.1값 유지)
const SPAWN_CP_RVA: usize = 0x13c71b0;   // ⬜미확정(죽은 상수·inert 확인필·0.5.1값 유지)
const DEDUP_INS_PROLOGUE: [u8; 14] = [0x41, 0x57, 0x41, 0x56, 0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x30, 0x48, 0x89, 0xd6];
const SPAWN_CP_PROLOGUE: [u8; 19] = [0x55, 0x56, 0x57, 0x53, 0x48, 0x81, 0xec, 0x18, 0x0f, 0x00, 0x00, 0x48, 0x8d, 0xac, 0x24, 0x80, 0x00, 0x00, 0x00];
static DEDUP_INS_TRAMP: AtomicUsize = AtomicUsize::new(0);
static SPAWN_CP_TRAMP: AtomicUsize = AtomicUsize::new(0);
static DEDUP_INS_HITS: AtomicU64 = AtomicU64::new(0);
static SPAWN_CP_HITS: AtomicU64 = AtomicU64::new(0);

extern "win64" fn dedup_ins_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> u8 {
    DEDUP_INS_HITS.fetch_add(1, Ordering::Relaxed);
    let stub = DEDUP_INS_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> u8 = unsafe { core::mem::transmute(stub) };
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(rcx, rdx, r8, r9))).unwrap_or(0)
}
extern "win64" fn spawn_cp_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    SPAWN_CP_HITS.fetch_add(1, Ordering::Relaxed);
    let stub = SPAWN_CP_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(rcx, rdx, r8, r9))).unwrap_or(0)
}

unsafe fn install_hook_n(rva: usize, prologue: &[u8], tramp: &AtomicUsize, detour: usize) -> Result<String, String> {
    if rva == 0 { return Ok("skip (RVA 미해결)".into()); }
    let base = exe_base(); if base == 0 { return Err("module 0".into()); }
    let n = prologue.len();
    let fn_addr = base + rva;
    if !readable(fn_addr, n) { return Err(format!("unreadable 0x{:x}", fn_addr)); }
    let mut cur = [0u8; 24];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), n);
    if &cur[..n] != prologue { return Err(format!("prologue mismatch 0x{:x} cur={:02x?}", rva, &cur[..n])); }
    let stub = VirtualAlloc(0, 64, 0x1000 | 0x2000, 0x40);
    if stub == 0 { return Err("VirtualAlloc".into()); }
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(prologue);
    s.extend_from_slice(&[0x48, 0xb8]);
    s.extend_from_slice(&(fn_addr + n).to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    tramp.store(stub, Ordering::Relaxed);
    let mut patch = [0u8; 24];
    patch[0] = 0x48; patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&detour.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    for i in 12..n { patch[i] = 0x90; }
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, n, 0x40, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, n);
    VirtualProtect(fn_addr, n, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, n);
    Ok(format!("hook 0x{:x} @abs=0x{:x}", rva, fn_addr))
}

// ── (9) [진단] push 도달 프로브 (INT3+VEH, UI스레드 안전) ──────────────────────────
// push 콜사이트 0x101cc08(mov [rax],0x1c)에 INT3 심어, run 핸들러가 여기 도달=서버 전송 확정.
//   도달 1회 잡으면 원본 복원(재무장 안 함). run_detour가 PUSH_HITS 로깅.
// ⛔죽은 상수(0.5.0_2 기준). 0.5.2에서도 미마이그 — **하지만 이건 그대로 두는 게 안전**: install_push_probe는
//   프롤로그 검증 없이 INT3를 blind write하는 유일한 경로라 주소가 틀리면 명령 중간을 파괴한다.
//   현재 init()에서 `let _ = install_push_probe;`로 **호출 자체를 비활성**했으므로 무해. 재활성 금지(ghidra-re 선행).
const PUSH_RVA: usize = 0x101cc08;   // ⬜미확정(죽은 상수·프로브 호출 비활성 상태·0.5.1값 유지)
static PUSH_HITS: AtomicU64 = AtomicU64::new(0);
static PUSH_ORIG: AtomicU64 = AtomicU64::new(0);
static VEH_INSTALLED: AtomicBool = AtomicBool::new(false);

#[repr(C)]
struct ExcRecord { code: u32, flags: u32, rec: usize, addr: usize, nparams: u32, _p: u32, params: [usize; 15] }
#[repr(C)]
struct ExcPointers { rec: *mut ExcRecord, ctx: *mut core::ffi::c_void }
extern "system" {
    fn AddVectoredExceptionHandler(first: u32, handler: extern "system" fn(*mut ExcPointers) -> i32) -> usize;
}

extern "system" fn push_veh(p: *mut ExcPointers) -> i32 {
    const BP: u32 = 0x80000003;
    const CONTINUE: i32 = -1;
    const SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return SEARCH; }
        let rec = (*p).rec;
        if rec.is_null() || (*rec).code != BP { return SEARCH; }
        let base = EXE_BASE.load(Ordering::Relaxed) as usize;
        if base == 0 { return SEARCH; }
        let push_addr = base + PUSH_RVA;
        if (*rec).addr != push_addr { return SEARCH; }
        // 원본 바이트 복원(재무장 안 함 = 1회만 잡음)
        let orig = PUSH_ORIG.load(Ordering::Relaxed) as u8;
        let mut old: u32 = 0;
        if VirtualProtect(push_addr, 1, 0x40, &mut old) != 0 {
            *(push_addr as *mut u8) = orig;
            VirtualProtect(push_addr, 1, old, &mut old);
            FlushInstructionCache(GetCurrentProcess(), push_addr, 1);
        }
        // Rip을 push_addr로 되돌려 원본 명령 재실행 (CONTEXT.Rip @ +0xF8)
        let ctx = (*p).ctx as usize;
        if ctx != 0 { *((ctx + 0xF8) as *mut u64) = push_addr as u64; }
        PUSH_HITS.fetch_add(1, Ordering::Relaxed);
        CONTINUE
    }
}

unsafe fn install_push_probe() -> Result<String, String> {
    let base = exe_base(); if base == 0 { return Err("module 0".into()); }
    let addr = base + PUSH_RVA;
    if !readable(addr, 1) { return Err("push addr unreadable".into()); }
    let orig = *(addr as *const u8);
    PUSH_ORIG.store(orig as u64, Ordering::Relaxed);
    if !VEH_INSTALLED.swap(true, Ordering::Relaxed) {
        AddVectoredExceptionHandler(1, push_veh);
    }
    let mut old: u32 = 0;
    if VirtualProtect(addr, 1, 0x40, &mut old) == 0 { return Err("VirtualProtect".into()); }
    *(addr as *mut u8) = 0xCC;                            // INT3
    VirtualProtect(addr, 1, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, 1);
    Ok(format!("push probe (INT3) @abs=0x{:x} orig=0x{:02x}", addr, orig))
}

// ── (13) collect 후킹 = comp_test UI 선택 라인업을 그대로 주입 (관측만, 치환 없음) ──
// collect(0.5.1 0x16203f0, 구0.5.0_2=0x101d970)가 반환하는 out 구조 = {cap@+0, data ptr@+8, len@+0x10},
//   data = 8바이트 athlete_id "정수" 배열(UI에서 고른 선수 id 목록, len=슬롯수).
// ★서버 소속검증 없음(ghidra-re 0.5.1: 핸들러 0xf1d2c0 등록루프에 team 게이트 부재) → UI 선택 id는
//   전역 레지스트리(game_ctx+0x16b90) 실존이므로 그대로 서버 sim에 등록됨(distinct면 봉쇄 없음).
// 동작: UI 선택 id 배열을 관측 로깅만 하고 **원본 그대로 통과 = UI에서 고른 선수로 주입**.
//   (구 inject_ids.txt 파일 오버라이드는 폐기 — 유저 요청: UI 선택 선수로 주입.)
// ★실존 athlete_id 풀 = collect가 관측한 id 누적(파일 영속). 중복 선택 시 distinct 보충용.
//   위조 id는 서버 등록서 크래시하므로 "실제로 UI에 떴던 id"만 사용(관측분만 누적).
static ID_POOL: Mutex<Vec<u64>> = Mutex::new(Vec::new());
static REG_ORDER: Mutex<Vec<u64>> = Mutex::new(Vec::new());   // 등록(치환)된 id 순서 = 슬롯 매핑 진단용
static ENT_SCANNED: AtomicBool = AtomicBool::new(false);      // 엔티티 필드 1회 스캔 플래그
static A2_STATS: Mutex<Vec<[u64; 8]>> = Mutex::new(Vec::new());  // SELECTED 전원 8스탯(positional 주입용)
// ★server_dedup_real 패치로 중복 athlete_id가 그대로 등록되므로 치환 자체가 불필요 → OFF.
//   (치환이 켜져 있으면 등록이 distinct로 바뀌어 진짜 중복 등록을 검증할 수 없음.)
const SUBST_ON: bool = false;
// ★RUN 핸들러 FUN_14161eab0(0x161eab0, 2-arg RCX=view_ctx/RDX=param2, 프롤로그 push8 = 12B).
//   collect 호출자 3곳 중 0x161ed83(RUN 제출)만 치환 대상. 0x162c85c(선택상태 영속저장)를 치환하면
//   가짜 distinct가 "현재 선택"으로 굳어 드롭다운 중복선택이 막힘(2026-07-20 실측) ⟹ 이 플래그로 격리.
//   RUN 구간엔 0x162c750(영속저장) 호출이 없음이 확인됨(re_Q).
//   훅 자체는 기존 run_detour(L418, 0x161eab0에 이미 설치)를 재사용 — 거기서 이 플래그를 on/off.
static RUN_ACTIVE: AtomicBool = AtomicBool::new(false);

fn pool_path() -> Option<PathBuf> { dir().map(|mut p| { p.push("athlete_pool.txt"); p }) }

fn pool_load() {
    let Some(p) = pool_path() else { return };
    let Ok(s) = fs::read_to_string(&p) else { return };
    let mut v = ID_POOL.lock().unwrap_or_else(|e| e.into_inner());
    for line in s.lines() {
        let t = line.trim().trim_start_matches("0x");
        if let Ok(id) = u64::from_str_radix(t, 16) {
            if id != 0 && !v.contains(&id) { v.push(id); }
        }
    }
}

fn pool_add(ids: &[u64]) {
    let mut changed = false;
    let snapshot = {
        let mut v = ID_POOL.lock().unwrap_or_else(|e| e.into_inner());
        for &id in ids { if id != 0 && !v.contains(&id) { v.push(id); changed = true; } }
        v.clone()
    };
    if !changed { return; }
    if let Some(p) = pool_path() {
        let s: String = snapshot.iter().map(|id| format!("0x{:x}\n", id)).collect();
        let _ = fs::write(&p, s);
    }
}

// ── (17) ★comp_test 아이템 주입 = 카테고리→아이템id 변환 `FUN_140f794c0` POST 훅 (re_U 2026-07-20) ──
// 계약: RCX=roster_begin(stride 0x28), RDX=count, R8=out_base(슬롯당 Vec<u64> = {cap@+0, ptr@+8, len@+0x10},
//   stride 0x18), R9=out_count. 슬롯당 u64 3개를 채움. ef1ea0(참가자struct 복사) **직전** 실행이라
//   여기서 덮어쓰면 UI revert와 무관하게 확실 주입 → 결과창·리플레이까지 자연 반영.
// ★UI 채널(러너+0x2388 Vec<[u8;3]>)은 카테고리 0~6만 표현(점프테이블 1→4,2→24,3→9,4→14,5→19,6→29)이라
//   모드템 id≥30은 구조적으로 표현 불가 ⟹ 모드 자체 저장소(cfg)에서 읽어 여기서 주입.
// 1단계 = cfg 지정 + 관측(모드템 id가 sim 구매 resolver를 통과하는지 검증). 2단계 = 오버레이 드롭다운 UI.
// ★모드 아이템 목록 덤프 (item_tactics/scrim의 mod_items 스캔 이식, 판정법은 [활성템] 정본 그대로)
//   mod_items Vec @ db+0x15d78 = {cap@+0x00, ptr@+0x08, len@+0x10}. element stride 0x1a8(모드)/0x198(바닐라).
//   element: key String @+0x08(ptr)/+0x10(len), next_tier Vec @+0x30, price @+0x180, tier @+0x188.
//   ★게임 ID = 30 + Vec인덱스 (바닐라 0~29). "Vec에 존재 = 활성" — 비활성 모드템은 애초에 병합 안 됨.
static MODITEMS_DONE: AtomicBool = AtomicBool::new(false);
const UI_INJECT_ON: bool = true;                              // comp_test 아이템칸 오버레이 드롭다운
static UINJ_LOGGED: AtomicBool = AtomicBool::new(false);
// 모드 "최종템"만 드롭다운 옵션으로 (next_tier 없음 = 조합 결과물). (id, key)
static MOD_FINALS: Mutex<Vec<(u64, String)>> = Mutex::new(Vec::new());
// ★드롭다운 옵션 = 게임 원래 카테고리 7개 + 모드 최종템. (item_tactics 와 동일 체계)
//   인덱스 0 = 미지정(게임이 정한 목표빌드 유지), 1~6 = 바닐라 카테고리, 7+ = 모드템.
const VANILLA_OPTS: [&str; 7] = ["선수에게 맡김", "공격력", "주문력", "공격속도", "방어력", "마저", "체력"];
// 카테고리 1~6 → 바닐라 최종템 게임 id (게임 c6 점프테이블 상수와 동일: cat1=AD .. cat6=Hp)
const VANILLA_FINAL: [u64; 6] = [4, 24, 9, 14, 19, 29];

/// 선택 인덱스 → 주입할 아이템 id (0 = 미지정)
fn sel_to_item_id(sel: usize, finals: &[(u64, String)]) -> u64 {
    if sel == 0 { return 0; }
    if sel <= 6 { return VANILLA_FINAL[sel - 1]; }
    finals.get(sel - 7).map(|(id, _)| *id).unwrap_or(0)
}

// ── 네이티브 드롭다운 조작 (item_tactics/scrim 공용 방식) ──
// ⚠0.5.1 = 0x2450f40 (구 0.5.0_3 = 0x2416070). 프롤로그 검증으로 오식별 방지.
// 0.5.2: ~~0.5.1 0x2450f40~~ → 0x242f250 (**L1-UNIQUE**·53 instr·12B 프롤로그 실측 MATCH).
//   ⚠0.5.1 때의 "구 0x2416070 금지" 경고와 동류 — 반드시 이 값만 쓸 것.
const FN_DD_SETOPT_RVA: usize = 0x1bfb50;   // ★0.5.5(구0.5.4=0x1bfc80) 다중앵커 투표 14/14 size2428 동일(자체 프롤로그 검증 있음)
static DD_VALID: AtomicUsize = AtomicUsize::new(0);   // 0=미판정 1=유효 2=무효

#[repr(C)]
struct DdOpt { color: u64, color2: u32, alpha: f32, s_len: usize, s_ptr: usize, s_cap: usize }

unsafe fn dd_addr_valid() -> bool {
    match DD_VALID.load(Ordering::Relaxed) { 1 => return true, 2 => return false, _ => {} }
    let fa = exe_base() + FN_DD_SETOPT_RVA;
    // 0.5.3(2026-07-29 item_tactics 세션 실측): 프롤로그가 7B→12B 로 바뀌었다.
    //   0.5.2 `55 56 57 48 83 ec 70` → 0.5.3 `55 41 57 41 56 56 57 53 48 81 ec 88`
    //   (드롭다운 runner+0x1150/+0x1154 오프셋은 불변)
    let expect = [0x55u8, 0x41, 0x57, 0x41, 0x56, 0x56, 0x57, 0x53, 0x48, 0x81, 0xec, 0x88];
    let mut ok = readable(fa, expect.len());
    if ok { for i in 0..expect.len() { if *((fa + i) as *const u8) != expect[i] { ok = false; break; } } }
    DD_VALID.store(if ok { 1 } else { 2 }, Ordering::Relaxed);
    if !ok { log(&format!("[dd] ⚠프롤로그 불일치 @0x{:x} — RVA stale 의심\n", FN_DD_SETOPT_RVA)); }
    ok
}

unsafe fn runner_base(n: &Node) -> usize {
    let any: &dyn std::any::Any = n.runner.as_any();
    let parts: [usize; 2] = core::mem::transmute::<*const dyn std::any::Any, [usize; 2]>(any as *const dyn std::any::Any);
    parts[0]
}

fn find_rb(n: &Node, t: &str) -> Option<usize> {
    if n.id.as_str() == t { return Some(unsafe { runner_base(n) }); }
    for c in n.child.iter() { if let Some(b) = find_rb(c, t) { return Some(b); } }
    None
}

fn find_node<'a>(n: &'a Node, t: &str) -> Option<&'a Node> {
    if n.id.as_str() == t { return Some(n); }
    for c in n.child.iter() { if let Some(x) = find_node(c, t) { return Some(x); } }
    None
}

fn find_mut<'a>(n: &'a mut Node, t: &str) -> Option<&'a mut Node> {
    if n.id.as_str() == t { return Some(n); }
    for c in n.child.iter_mut() { if let Some(x) = find_mut(c, t) { return Some(x); } }
    None
}

// ── 노드 레이아웃 (ui_kit 규약) ──
// 4상태 블록 [0x70, 0xf0, 0x170, 0x1f0], 블록 내 W+0x00 / H+0x08 / X+0x10 / Y+0x18, 값은 각 +4.
// ★역할 분리(2026-07-21): 1~3칸의 크기·위치는 **item_tactics 단독 관리**. 우리는 네이티브 칸의 좌표를
//   "읽어서" 오버레이에 복사만 한다(쓰기 대상은 우리 노드뿐). 3칸/4칸 어느 배치든 자동 추종되고,
//   상대가 좌표 정책을 바꿔도 우리가 따라가므로 충돌이 구조적으로 없다.
unsafe fn node_box(n: &Node) -> Option<(f32, f32, f32, f32)> {
    let na = n as *const Node as usize;
    if na <= 0x10000 { return None; }
    let rd = |off: usize| -> f32 {
        let a = na + 0x70 + off + 4;
        if readable(a, 4) { *(a as *const f32) } else { 0.0 }
    };
    let (w, h, x, y) = (rd(0x00), rd(0x08), rd(0x10), rd(0x18));
    if w <= 1.0 || h <= 1.0 { return None; }
    Some((x, y, w, h))
}

/// 우리 소유 노드에만 사용. 드롭다운은 인터랙티브라 4상태 전부 기입해야 hover 때 튀지 않는다.
unsafe fn set_node_box_all_states(n: &Node, x: f32, y: f32, w: f32, h: f32) {
    let na = n as *const Node as usize;
    if na <= 0x10000 { return; }
    for blk in [0x70usize, 0xf0, 0x170, 0x1f0] {
        for (fo, v) in [(0x00usize, w), (0x08, h), (0x10, x), (0x18, y)] {
            let a = na + blk + fo + 4;
            if readable(a, 4) { *(a as *mut f32) = v; }
        }
    }
}

/// target 드롭다운에 옵션 세팅(문자열 소유권은 게임에 넘김 = forget).
unsafe fn dd_set_options(root: &Node, target: &str, items: &[String], sel: u64) -> bool {
    if !dd_addr_valid() { return false; }
    let Some(rb) = find_rb(root, target) else { return false };
    let mut opts: Vec<DdOpt> = Vec::with_capacity(items.len());
    for it in items {
        let s = it.clone();
        opts.push(DdOpt { color: 0x3f800000_3f800000, color2: 0x3f800000, alpha: 1.0,
                          s_len: s.len(), s_ptr: s.as_ptr() as usize, s_cap: s.capacity() });
        core::mem::forget(s);
    }
    let param3: [usize; 3] = [0, opts.as_ptr() as usize, opts.len()];
    let f: unsafe extern "system" fn(usize, u64, *const [usize; 3]) =
        core::mem::transmute(exe_base() + FN_DD_SETOPT_RVA);
    f(rb, sel, &param3);
    core::mem::forget(opts);
    true
}

/// 현재 선택 인덱스(runner+0x1788). u64::MAX = 미선택.
unsafe fn dd_selected(root: &Node, target: &str) -> Option<usize> {
    let rb = find_rb(root, target)?;
    if !readable(rb + 0x1788, 8) { return None; }
    let v = *((rb + 0x1788) as *const u64);
    if v == u64::MAX { None } else { Some(v as usize) }
}
static UINJ_TICK: AtomicU64 = AtomicU64::new(0);
static UINJ_LOGS: AtomicU64 = AtomicU64::new(0);
static DD_LAST_RB: AtomicUsize = AtomicUsize::new(0);   // 옵션 재세팅 판정용(노드 재생성 감지)
static DD_DIAG: AtomicU64 = AtomicU64::new(0);

unsafe fn dump_mod_items(db: usize) {
    if MODITEMS_DONE.swap(true, Ordering::Relaxed) { return; }
    let vec = db + 0x15d78;
    if !readable(vec, 0x18) { log(&format!("[moditem] vec unreadable @0x{:x}\n", vec)); return; }
    let ptr = *((vec + 8) as *const usize);
    let len = *((vec + 0x10) as *const usize);
    if ptr < 0x10000 || len == 0 || len > 500 {
        log(&format!("[moditem] vec 비정상 ptr=0x{:x} len={} (모드 아이템 없음?)\n", ptr, len));
        return;
    }
    // stride 감지: 연속 4개 element의 key가 전부 유효 문자열이고 서로 달라야 함
    let mut stride = 0usize;
    for &st in &[0x1a8usize, 0x198, 0x1b0] {
        let n = len.min(4);
        let k: Vec<String> = (0..n).map(|i| read_str(ptr + i * st, 0x8)).collect();
        if k.iter().all(|s| s.len() >= 3) && (n < 2 || k[0] != k[1]) { stride = st; break; }
    }
    if stride == 0 { log(&format!("[moditem] stride 감지 실패 (ptr=0x{:x} len={})\n", ptr, len)); return; }
    log(&format!("[moditem] === 모드 아이템 {}개 (stride 0x{:x}) — 게임 id = 30+인덱스 ===\n", len, stride));
    let mut finals: Vec<(u64, String)> = Vec::new();
    for i in 0..len {
        let e = ptr + i * stride;
        if !readable(e, 0x190) { continue; }
        let key = read_str(e, 0x8);
        let price = *((e + 0x180) as *const u64);
        let tier = *((e + 0x188) as *const u64);
        // next_tier Vec(@+0x30)의 len — 0이면 상위 조합이 없음 = 최종템 후보
        let nt_len = if readable(e + 0x40, 8) { *((e + 0x40) as *const usize) } else { usize::MAX };
        let fin = if nt_len == 0 { " ★최종템" } else { "" };
        if nt_len == 0 && !key.is_empty() { finals.push(((30 + i) as u64, key.clone())); }
        log(&format!("[moditem] id={:<4} tier={} price={:<6} next={:<3} {}{}\n",
            30 + i, tier, price, nt_len, key, fin));
    }
    log(&format!("[moditem] 최종템 {}개 = 드롭다운 옵션\n", finals.len()));
    *MOD_FINALS.lock().unwrap_or_else(|e| e.into_inner()) = finals;
}

// 0.5.2: ~~0.5.1 0xf794c0~~ → 0xed8770 (**L1-UNIQUE**·198 instr·HOOK_PROLOGUE12_ALT 실측 MATCH).
//   07-21 인게임 검증된 comp_test 아이템칸 모드템 주입 POST 훅.
const ITEMCONV_RVA: usize = 0x21d1cd0;   // ★0.5.5 ghidra-re 2방법: movzx 앵커 0fb64325.. 전역유일 + HOOK_PROLOGUE12_ALT 실측·.pdata 엔트리. ⚠구값 ~~0x18429d0~~은 실은 0.5.3값(0.5.4 재핀 누락으로 0.5.4 내내 프롤로그검증 실패=죽어있었음)
static ITEMCONV_TRAMP: AtomicUsize = AtomicUsize::new(0);
static ITEMCONV_HITS: AtomicU64 = AtomicU64::new(0);
static ITEM_OVERRIDE: Mutex<Vec<[u64; 3]>> = Mutex::new(Vec::new());   // 슬롯(0~9) → 아이템 id 3개(0=미지정)

// cfg: `mods\tfm2_comptest_unlock\comptest_items.cfg`, 형식 `슬롯=id,id,id` (바닐라 0~29 / 모드 30+)
fn load_item_cfg() {
    let Some(mut p) = dir() else { return };
    p.push("comptest_items.cfg");
    let Ok(s) = fs::read_to_string(&p) else { return };
    let mut v = vec![[0u64; 3]; 16];
    let mut n = 0;
    for line in s.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') { continue; }
        let Some((k, val)) = t.split_once('=') else { continue };
        // `layout = 3|4` : 오버레이 좌표 세트 선택(4=item_tactics 4칸 배치, 3=바닐라). 재시작 시 반영.
        // `runs = N` : RUN 1클릭당 실행할 총 경기 수(1~10). 1 = 바닐라 동작.
        if k.trim().eq_ignore_ascii_case("runs") {
            if let Ok(n) = val.trim().parse::<u64>() {
                let n = n.clamp(1, CONC_RUNS_MAX);
                CONC_RUNS.store(n, Ordering::Relaxed);
                log(&format!("[conc] cfg runs = {}경기/클릭\n", n));
            }
            continue;
        }
        // `freeze = 0|1|2|3` : 다시보기 재현용 상태 고정 단계(0=끔). 재시작 시 반영.
        if k.trim().eq_ignore_ascii_case("freeze") {
            if let Ok(n) = val.trim().parse::<u64>() {
                let n = n.min(3);
                FREEZE_LV.store(n, Ordering::Relaxed);
                log(&format!("[freeze] cfg freeze = {}\n", n));
            }
            continue;
        }
        // `parallel = 0|1` : 0 = 순차(한 경기씩), 1 = 병렬(동시). 재시작 시 반영.
        if k.trim().eq_ignore_ascii_case("parallel") {
            let on = !val.trim().starts_with('0');
            PAR_RT.store(if on { 1 } else { 0 }, Ordering::Relaxed);
            log(&format!("[conc] cfg parallel = {}\n", if on { "병렬" } else { "순차" }));
            continue;
        }
        if k.trim().eq_ignore_ascii_case("layout") {
            let four = !val.contains('3');
            uinj::LAYOUT4.store(four, Ordering::Relaxed);
            log(&format!("[item] layout = {}칸 배치\n", if four { 4 } else { 3 }));
            continue;
        }
        let Ok(slot) = k.trim().parse::<usize>() else { continue };
        if slot >= 16 { continue; }
        for (j, part) in val.split(',').take(3).enumerate() {
            if let Ok(id) = part.trim().parse::<u64>() { v[slot][j] = id; }
        }
        n += 1;
    }
    if n > 0 {
        log(&format!("[item] cfg 로드: {}슬롯 지정\n", n));
        *ITEM_OVERRIDE.lock().unwrap_or_else(|e| e.into_inner()) = v;
    }
}

extern "win64" fn itemconv_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    let stub = ITEMCONV_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
    let ret = f(rcx, rdx, r8, r9);          // 원본이 카테고리→id로 out Vec을 채움
    let n = ITEMCONV_HITS.fetch_add(1, Ordering::Relaxed);
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        if r8 < 0x10000 || rdx < 1 || rdx > 16 { return; }
        let ov = ITEM_OVERRIDE.lock().unwrap_or_else(|e| e.into_inner()).clone();
        // ★이 함수는 팀당 1회(경기당 2회) 호출되고 슬롯은 팀 내 0~4다. UI 슬롯(blue0..4=0~4,
        //   red0..4=5~9)과 맞추려면 호출 순번의 홀짝으로 팀을 판정해 +5 오프셋을 준다.
        let team = (n % 2) as usize;
        for i in 0..rdx {
            let v = r8 + i * 0x18;
            if !readable(v, 0x18) { continue; }
            let ptr = *((v + 8) as *const usize);
            let len = *((v + 0x10) as *const usize);
            if ptr < 0x10000 || len == 0 || len > 8 || !readable(ptr, len * 8) { continue; }
            let before: Vec<u64> = (0..len).map(|j| *((ptr + j * 8) as *const u64)).collect();
            let mut after = before.clone();
            if let Some(ids) = ov.get(team * 5 + i) {
                for j in 0..len.min(3) {
                    if ids[j] != 0 { *((ptr + j * 8) as *mut u64) = ids[j]; after[j] = ids[j]; }
                }
            }
            if n < 4 { log(&format!("[item] #{} slot{} len={} {:?} -> {:?}\n", n, i, len, before, after)); }
        }
    }));
    ret
}

unsafe fn install_itemconv_hook() -> Result<String, String> {
    install_hook12(ITEMCONV_RVA, &ITEMCONV_TRAMP, itemconv_detour as usize)
        .map(|s| format!("itemconv 0xf794c0 {}", s))
}

const COLLECT_RVA: usize = 0x1aa4290;   // ★0.5.5 ghidra-re 2방법: TypeId+cmovb+mov rsi,[rsi+0x1788] 결합시그 유일 + push8 12B 실측·.pdata 엔트리. ⚠구값 ~~0x18f2b50~~은 실은 0.5.3값(0.5.4선 죽어있었음) // (구0.5.1=0x16203f0, 145 instr)
static COLLECT_TRAMP: AtomicUsize = AtomicUsize::new(0);
static COLLECT_HITS: AtomicU64 = AtomicU64::new(0);

extern "win64" fn collect_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    let stub = COLLECT_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
    let ret = f(rcx, rdx, r8, r9);                       // 원본 collect: *rcx = {cap@0, data@8, len@0x10}
    let n = COLLECT_HITS.fetch_add(1, Ordering::Relaxed);
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        if rcx < 0x10000 || !readable(rcx, 0x18) { return; }
        let data = *((rcx + 8) as *const usize);         // data @ rcx+8 (rcx+0=cap)
        let len = *((rcx + 0x10) as *const usize);
        if data < 0x10000 || len < 1 || len > 32 || !readable(data, len * 8) { return; }
        // ① 관측: UI 선택 id 배열 로깅(처음 4회)
        if n < 4 {
            let mut s = String::new();
            for i in 0..len.min(16) {
                let id = if readable(data + i * 8, 8) { *((data + i * 8) as *const usize) } else { 0 };
                s.push_str(&format!("0x{:x} ", id));
            }
            log(&format!("[collect] #{} len={} UI선택ids= {} (그대로 주입)\n", n, len, s));
        }
        // ★UI 선택 id(중복 포함) 기억 = A2 엔티티 주입 소스. 매 collect 갱신(최신 선택 반영).
        let m = len.min(16);
        for i in 0..m {
            let id = if readable(data + i * 8, 8) { *((data + i * 8) as *const u64) } else { 0 };
            SELECTED_IDS[i].store(id, Ordering::Relaxed);
        }
        SELECTED_N.store(m, Ordering::Relaxed);
        // ★② 치환: UI 선택에 중복이 있으면 서버로는 distinct 실존 10명을 넘긴다(게이트·서버 HashMap 통과).
        //    등록만 distinct일 뿐, 경기 시작 후 스탯·이름은 SELECTED(UI 원본)로 덮어써 "같은 선수 N명"이 성립.
        let ui: Vec<u64> = (0..len)
            .map(|i| if readable(data + i * 8, 8) { *((data + i * 8) as *const u64) } else { 0 })
            .collect();
        pool_add(&ui);                       // 관측된 실존 id 누적(파일 영속)
        let mut uniq: Vec<u64> = Vec::new();
        for &id in &ui { if id != 0 && !uniq.contains(&id) { uniq.push(id); } }
        // ⚠RUN 제출 구간에서만 치환(UI 조회·영속저장 경로는 원본 유지 = 드롭다운 중복선택 보존).
        if SUBST_ON && RUN_ACTIVE.load(Ordering::Relaxed) && uniq.len() < len {
            let pool = ID_POOL.lock().unwrap_or_else(|e| e.into_inner()).clone();
            for c in pool {
                if uniq.len() >= len { break; }
                if !uniq.contains(&c) { uniq.push(c); }
            }
            if uniq.len() == len {
                for i in 0..len { *((data + i * 8) as *mut u64) = uniq[i]; }
                // positional 매핑 진단: 등록 순서 기록 → 결과창 슬롯의 원본 이름과 대조하면
                //   "collect 인덱스 ↔ 결과창 슬롯" 대응이 확정됨(현재 어긋나 있음: SEL[0]≠슬롯#0).
                {
                    let mut reg = REG_ORDER.lock().unwrap_or_else(|e| e.into_inner());
                    *reg = uniq.clone();
                }
                log(&format!("[collect]   등록순서= {}\n",
                    uniq.iter().map(|id| format!("0x{:x}", id)).collect::<Vec<_>>().join(" ")));
                log(&format!("[collect] ★치환: UI중복 {}칸 → 등록용 distinct {}명 (경기는 UI 선택으로 주입)\n",
                    len, uniq.len()));
            } else {
                log(&format!("[collect] ⚠풀 부족 distinct {}/{} → 치환 생략(시작 실패 예상). \
                    서로 다른 선수를 한 번 골라 풀을 채우면 이후 중복 선택 가능\n", uniq.len(), len));
            }
        }
    }));
    ret
}

unsafe fn install_collect_hook() -> Result<String, String> {
    install_hook12(COLLECT_RVA, &COLLECT_TRAMP, collect_detour as usize)
        .map(|s| format!("collect(중복→등록용 distinct 치환, 풀 {}개) {}",
            ID_POOL.lock().unwrap_or_else(|e| e.into_inner()).len(), s))
}

// ── (15) [실증] ef1ea0 로스터 재매핑 프로브 = "등록 distinct / sim만 재매핑" 유효성 테스트 ──
// FUN_140ef1ea0(0xef1ea0, comp_test 경기형성 한 팀 참가자벡터 빌드, push8 프롤로그): 계약
//   rcx=out, rdx=game_ctx, r8=roster_begin(stride 0x28), r9=roster_count. 엔트리+0x18=u64 athlete id.
// ef1ea0는 게이트(distinct dedup)·등록 통과 후 호출 → 여기서 엔트리+0x18(id)만 바꾸면 sim은 그 선수로,
//   등록/HashMap은 이미 distinct라 무충돌(re_M). ⚠ef1ea0 출력이 live sim 도달하는지 정적 미확정 → 실증용.
// 최소 테스트: 전 슬롯 id를 slot0 id로 통일 → 결과 경기가 "같은 선수 N명"이면 ①live 도달 ②중복 스폰 실증.
// ef1ea0(4-arg 확정: rcx=out·rdx=game_ctx·r8=roster_begin·r9=count). ★형성 크리티컬경로(팀당1회 2회)라
//   detour는 최소(game_ctx 원자 store만·로깅/락/alloc 금지) — 로깅 detour가 이전 형성방해 원인.
// 0.5.2: ~~0.5.1 0xef1ea0~~ → 0xe58c30 (**L1-UNIQUE**·302 instr·PROL-OK push8 12B).
//   (이름의 EF1EA0은 0.4.x RVA 유래 — 주소가 아니라 식별자로만 유지)
const EF1EA0_RVA: usize = 0; // ⬜0.5.3 미해결(구0.5.2=0xe58c30) — SIM_PROBE_ON=false 라 미설치
static EF1EA0_TRAMP: AtomicUsize = AtomicUsize::new(0);
static GAME_CTX: AtomicU64 = AtomicU64::new(0);   // ef1ea0 rdx=game_ctx(athlete 레지스트리 base = +0x16b90)
// A2 엔티티 스탯 주입 배선
// 0.5.2: ~~0.5.1 0xeaad40~~ → **0xe3b200**. 스켈레톤 L1~L3는 NO-MATCH(모노모픽 copy가 여럿, cos=1.0000
//   동점 후보 2개)였으나 **콜그래프 앵커링으로 확정**: 콜사이트 수가 OLD 199 ↔ 0xe3b200 **199로 정확히 일치**
//   (경합 후보 0x82bff0은 93개 = 탈락), 콜러 컨테이너 3개도 1:1 대응(그 중 서버핸들러 0xf1d2c0→0xe7ccd0은
//   독립 확정분과 자기일치). ATH_GET_PROLOGUE 17B 실측 MATCH. 신뢰도 HIGH.
const ATH_GET_SC_RVA: usize = 0x1794280;           // ⬜0.5.5 미확정(SIM_PROBE_ON=false로 inert·0.5.4값 유지) // shadow-call: rcx=game_ctx+0x16b90, rdx=&id → rax(0=miss), athlete*=[rax]
static SELECTED_IDS: [AtomicU64; 16] = [const { AtomicU64::new(0) }; 16];   // UI 선택 athlete_id(중복 포함)
static SELECTED_N: AtomicUsize = AtomicUsize::new(0);
// ★server_dedup_real(0xf67b91)로 중복 등록이 진짜로 되므로 스탯/이름 주입은 불필요 → OFF.
//   ⚠주입을 켜두면 sim 입력이 매 틱 바뀌는데 그 **시작 시점이 원경기(캐시 채운 뒤)와 리플레이(캐시 기보유 →
//   첫 틱부터)에서 달라** 같은 시드라도 궤적이 갈림 = 다시보기 결과 불일치의 원인(2026-07-20 유저 관측).
static A2_WRITE_ON: AtomicBool = AtomicBool::new(false);
const COSMETIC_ON: bool = false;   // 결과창/엔티티 이름 교체(진짜 등록이면 불필요)
static A2_STAT: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];   // SELECTED[0] athlete 8스탯 캐시([1,100] 클램프)
static A2_STAT_READY: AtomicBool = AtomicBool::new(false);

extern "win64" fn ef1ea0_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    GAME_CTX.store(rdx as u64, Ordering::Relaxed);   // ★최소: game_ctx 캡처만(무로깅=형성 무방해)
    let stub = EF1EA0_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
    f(rcx, rdx, r8, r9)
}

unsafe fn install_ef1ea0_probe() -> Result<String, String> {
    install_hook12(EF1EA0_RVA, &EF1EA0_TRAMP, ef1ea0_detour as usize)
        .map(|s| format!("ef1ea0 game_ctx캡처(최소) {}", s))
}

// SELECTED athlete_id(u64 handle) → athlete 구조 ptr (ATH_GET 0xeaad40 shadow-call). 0=miss/실패.
//   순수 hashbrown find(뮤테이션·alloc 없음)=shadow 안전. 반환 deref만 readable 가드.
unsafe fn shadow_ath_get(gctx: u64, id: u64) -> usize {
    if gctx == 0 { return 0; }
    let reg = gctx as usize + 0x16b90;
    if !readable(reg, 0x28) { return 0; }
    let f: extern "win64" fn(usize, *const u64) -> usize =
        core::mem::transmute(exe_base() + ATH_GET_SC_RVA);
    let h = id;
    let rax = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(reg, &h as *const u64))) {
        Ok(v) => v, Err(_) => return 0,
    };
    if rax < 0x10000 || !readable(rax, 8) { return 0; }
    let ath = *(rax as *const usize);
    if ath < 0x10000 || !readable(ath, 0xe0) { return 0; }
    ath
}

// ── (14) [진단] sim 오라클 0x20566c0(1틱 오케) 직접 훅 = comp_test sim 배선·Game·mode vtable 캡처 ──
// ★comp_test sim = rayon 배치(seam1) 아님(결과창까지 갔는데 seam1 미발화 확증) = 동기폴러 FUN_14206dc10가
//   오라클 0x20566c0을 매틱 직접호출. 계약: rcx=out(0x100·[rcx]==-1 진행중), rdx=Game(self), r8=seed컨텍스트.
//   [Game+0x1dd8]=mode vtable(comp_test 식별키)·[Game+0x1dc8]=engine vtable. 프롤로그=HOOK_PROLOGUE12(push8).
//   loading(ACCEPT) 게이트 ON일 때만 첫 히트 캡처(공용 초핫패스라 comp_test 스코프한정)→이후 분기1개+원본.
//   ⚠조사용(배포 전 SIM_PROBE_ON=false).
// ★(2026-07-23) 릴리스 배포로 **false 전환**: oracle·ef1ea0·slot 진단 훅 3개가 설치되지 않는다.
//   근거=load-bearing 아님 점검 완료 — 이 게이트가 채우는 GAME_CTX/ORACLE_*/SELECTED_* 는 전부
//   프로브 경로 안에서만 읽히고, 유일한 소비처였던 A2 스탯주입은 `A2_WRITE_ON=false`(server_dedup_real로
//   진짜 중복등록이 되므로 스탯/이름 주입 자체가 불요). ⟹ 기능 무영향.
//   ★부수이득: 모드에 남아있던 **유일한 게임함수 shadow-CALL(`shadow_ath_get`) 경로가 도달불가**가 된다
//   (oracle_detour 안에서만 호출) = AV 위험 제거. 재조사 시에만 true.
const SIM_PROBE_ON: bool = false;
// 0.5.2: ~~0.5.1 0x20566c0~~ → **0x1d94720**. L1~L3 NO-MATCH(본문 리팩터: align=0.65)지만
//   니모닉 코사인 0.99862로 2nd후보(0.98802) 대비 갭 충분 + **콜그래프 교차확증**: 콜사이트 3개 ↔ 3개,
//   콜러 컨테이너 3개(0x2060be0/0x2061140/0x206dc10)를 독립 매칭하니 0x1d9e7e0/0x1d9ef10/0x1da5650 —
//   0x1d94720을 호출하는 컨테이너 집합과 **완전일치**(앞 둘은 L1-UNIQUE). PROL-OK push8 12B. 신뢰도 HIGH.
// ★0.5.4(2026-08-08 ghidra-re 재핀): ~~0xeb6590(오답 — 재조사 시 켰다면 엉뚱한 함수 후킹)~~ → **0x13b3150**.
//   근거=Game+0x1dc0/0x1dc8(engine)·+0x1dd0/0x1dd8(mode)·init플래그 +0x208a·SEED +0x40c 전 항목 0.5.1 채록과 일치(HIGH).
//   완주 폴러(feedback.rs, 구 0x206dc10)=0x148a7c0 · sim 실행 본체(구 0x1a511a0 계열)=0x237c030.
const ORACLE_RVA: usize = 0x14aa160;  // ★0.5.5(구0.5.4=0x13b3150) skeleton UNIQUE size5417·push8 확인 // 1틱 오케(run one tick), 프롤로그=HOOK_PROLOGUE12
static ORACLE_TRAMP: AtomicUsize = AtomicUsize::new(0);
static ORACLE_GATE: AtomicBool = AtomicBool::new(false);      // loading ACCEPT시 ON = comp_test 스코프
static ORACLE_CAPTURED: AtomicBool = AtomicBool::new(false);
static ORACLE_GAME: AtomicU64 = AtomicU64::new(0);
static ORACLE_MODEVT: AtomicU64 = AtomicU64::new(0);
static ORACLE_ENGVT: AtomicU64 = AtomicU64::new(0);
static ORACLE_SEED: AtomicU64 = AtomicU64::new(0);
static ORACLE_HITS: AtomicU64 = AtomicU64::new(0);
static ORACLE_ENT_LOGGED: AtomicBool = AtomicBool::new(false);   // 엔티티 dense 배열 관측 1회 게이트

// ★0x20566c0 = 5-arg(out, Game, seed, bool, bool5) — a5(5번째 스택인자) 반드시 보존해 재호출.
//   (4-arg로 호출하면 5번째 유실→sim이 쓰레기 플래그로 돌아 전투0 빈경기 = 실증된 버그.)
extern "win64" fn oracle_detour(rcx: usize, rdx: usize, r8: usize, r9: usize, a5: usize) -> usize {
    let stub = ORACLE_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
    if ORACLE_GATE.load(Ordering::Relaxed) && !ORACLE_CAPTURED.swap(true, Ordering::SeqCst) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            let game = rdx;
            let rd = |p: usize| -> u64 { if readable(p, 8) { *(p as *const u64) } else { 0 } };
            ORACLE_GAME.store(game as u64, Ordering::Relaxed);
            ORACLE_MODEVT.store(rd(game + 0x1dd8), Ordering::Relaxed);
            ORACLE_ENGVT.store(rd(game + 0x1dc8), Ordering::Relaxed);
            ORACLE_SEED.store(r8 as u64, Ordering::Relaxed);
        }));
    }
    ORACLE_HITS.fetch_add(1, Ordering::Relaxed);
    let ret = f(rcx, rdx, r8, r9, a5);                   // ★5-arg 보존(init 엔티티 스폰 포함)
    // [진단] 엔티티 dense 배열 관측(init 완료 후 1회·write없음) = 선수 판별 데이터 확보
    //   engine=*(Game+0x1dc0), base=*(engine+0x840), count=*(engine+0x848), e=base+i*0x8d0.
    //   선수(athlete) vs 구조물(타워/미니언) 판별용: team+0x820·id+0x818·스탯+0x208/+0x210·레벨+0x5e0·판별자+0x8b8.
    if ORACLE_GATE.load(Ordering::Relaxed) && !ORACLE_ENT_LOGGED.load(Ordering::Relaxed) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            let game = rdx;
            if !readable(game + 0x208a, 1) || *((game + 0x208a) as *const u8) != 1 { return; }   // init 완료
            if !readable(game + 0x1dc0, 8) { return; }
            let engine = *((game + 0x1dc0) as *const usize);
            if engine < 0x10000 || !readable(engine + 0x848, 8) { return; }
            let base = *((engine + 0x840) as *const usize);
            let count = *((engine + 0x848) as *const usize);
            if base < 0x10000 || count < 1 || count > 500 || !readable(base, 0x8d0) { return; }
            ORACLE_ENT_LOGGED.store(true, Ordering::Relaxed);
            let rd = |e: usize, off: usize| -> u64 { if readable(e + off, 8) { *((e + off) as *const u64) } else { 0 } };
            log(&format!("[ent] dense count={} (전투8스탯 cb=+0x1e0..+0x218, 처음 24개)\n", count));
            for i in 0..count.min(24) {
                let e = base + i * 0x8d0;
                if !readable(e, 0x8c0) { break; }
                log(&format!("[ent] #{} team=0x{:x} id=0x{:x} cb= {:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x} d3e0={:x} d3e8={:x} f8b8={:x}\n",
                    i, rd(e,0x820), rd(e,0x818),
                    rd(e,0x1e0),rd(e,0x1e8),rd(e,0x1f0),rd(e,0x1f8),rd(e,0x200),rd(e,0x208),rd(e,0x210),rd(e,0x218),
                    rd(e,0x3e0),rd(e,0x3e8), rd(e,0x8b8)));
            }
            // ★SELECTED athlete 스탯 shadow-call 덤프 (스케일·매핑 확인 = A2 write 전 필수 검증)
            let gc = GAME_CTX.load(Ordering::Relaxed);
            let sn = SELECTED_N.load(Ordering::Relaxed);
            log(&format!("[ath] game_ctx=0x{:x} sel_n={}\n", gc, sn));
            for k in 0..sn.min(10) {
                let id = SELECTED_IDS[k].load(Ordering::Relaxed);
                let ath = shadow_ath_get(gc, id);
                if ath != 0 {
                    log(&format!("[ath] sel[{}] id=0x{:x} ath=0x{:x} s98..d0= {:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x}\n",
                        k, id, ath, rd(ath,0x98),rd(ath,0xa0),rd(ath,0xa8),rd(ath,0xb0),rd(ath,0xb8),rd(ath,0xc0),rd(ath,0xc8),rd(ath,0xd0)));
                } else {
                    log(&format!("[ath] sel[{}] id=0x{:x} shadow MISS\n", k, id));
                }
            }
        }));
    }
    // ★A2 엔티티 스탯 주입 (매틱 idempotent, A2_WRITE_ON=true일 때만 — 진단서 스케일/매핑 확인 후 활성)
    if ORACLE_GATE.load(Ordering::Relaxed) && A2_WRITE_ON.load(Ordering::Relaxed) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            let game = rdx;
            if !readable(game + 0x208a, 1) || *((game + 0x208a) as *const u8) != 1 { return; }
            if !readable(game + 0x1dc0, 8) { return; }
            let engine = *((game + 0x1dc0) as *const usize);
            if engine < 0x10000 || !readable(engine + 0x848, 8) { return; }
            let base = *((engine + 0x840) as *const usize);
            let count = *((engine + 0x848) as *const usize);
            if base < 0x10000 || count < 1 || count > 500 { return; }
            let gc = GAME_CTX.load(Ordering::Relaxed);
            if gc == 0 || SELECTED_N.load(Ordering::Relaxed) == 0 { return; }
            // 첫 틱 1회: SELECTED[0] athlete 8스탯([1,100] 클램프) 캐시 → 이후 매틱 write에 재사용.
            if !A2_STAT_READY.load(Ordering::Relaxed) {
                let ath = shadow_ath_get(gc, SELECTED_IDS[0].load(Ordering::Relaxed));
                if ath == 0 { return; }
                for j in 0..8 {
                    let v = if readable(ath + 0x98 + j * 8, 8) { *((ath + 0x98 + j * 8) as *const u64) } else { 0 };
                    A2_STAT[j].store(v.max(1).min(100), Ordering::Relaxed);
                }
                A2_STAT_READY.store(true, Ordering::Relaxed);
                // SELECTED 전원의 이름 + 8스탯을 sim 틱에서 캐시(렌더 스레드 shadow-call 회피, positional 주입 소스).
                let sn = SELECTED_N.load(Ordering::Relaxed).min(16);
                let mut names: Vec<String> = Vec::new();
                let mut stats: Vec<[u64; 8]> = Vec::new();
                for i in 0..sn {
                    let a = shadow_ath_get(gc, SELECTED_IDS[i].load(Ordering::Relaxed));
                    names.push(if a >= 0x10000 { athlete_name(a).unwrap_or_default() } else { String::new() });
                    let mut st = [0u64; 8];
                    if a >= 0x10000 {
                        for j in 0..8 {
                            let v = if readable(a + 0x98 + j * 8, 8) { *((a + 0x98 + j * 8) as *const u64) } else { 0 };
                            st[j] = v.max(1).min(100);
                        }
                    }
                    stats.push(st);
                }
                *A2_STATS.lock().unwrap_or_else(|e| e.into_inner()) = stats;
                // 1회: athlete 문자열 오프셋 스캔(이름 필드 확정용)
                if ath >= 0x10000 {
                    for off in (0..0x100).step_by(8) {
                        let s = read_str(ath, off);
                        if !s.is_empty() && s.chars().all(|c| c.is_ascii_graphic() || c == ' ') {
                            log(&format!("[ath] scan +0x{:x} = '{}'\n", off, s));
                        }
                    }
                }
                log(&format!("[name] 캐시 {}명: {:?}\n", names.len(), names));
                *SELECTED_NAMES.lock().unwrap_or_else(|e| e.into_inner()) = names;
            }
            // [진단] 엔티티 dense 순서 ↔ 슬롯 대응 규명용 1회 스캔(엔티티에 이름/팀/포지션 필드가 있으면 노출).
            if !ENT_SCANNED.swap(true, Ordering::Relaxed) {
                for i in 0..count.min(3) {
                    let e = base + i * 0x8d0;
                    for off in (0..0x120).step_by(8) {
                        let s = read_str(e, off);
                        if s.len() >= 2 && s.len() <= 24 && s.chars().all(|c| c.is_ascii_graphic() || c == ' ') {
                            log(&format!("[ent] #{} +0x{:x} = '{}'\n", i, off, s));
                        }
                    }
                }
            }
            // 전 엔티티 전투 8스탯(+0x1e0..+0x218) = 캐시 스탯 + 파생(+0x3e0/+0x3e8) 재계산. 불가침 +0x818/+0x820/+0x8b0.
            // ★positional 주입: team(i64 @+0x820)·position(i32 @+0x8b0)으로 SELECTED[pos + 5*team] 매핑.
            //   (re_R 정정: +0x8b0은 phase가 아니라 position. +0x818/+0x820/+0x8b0은 읽기 전용 불가침.)
            //   경기·리플레이 화면 이름은 엔티티 String {cap@+0x80, ptr@+0x88, len@+0x90}에서 렌더되므로 같이 교체.
            //   리플레이 = 시드 재시뮬이라 이 훅이 재발화 → 재생 때도 자동 적용. ⚠멱등 가드(이미 내 이름이면 skip)로 leak 방지.
            let stats = A2_STATS.lock().unwrap_or_else(|e| e.into_inner()).clone();
            let names = SELECTED_NAMES.lock().unwrap_or_else(|e| e.into_inner()).clone();
            for i in 0..count {
                let e = base + i * 0x8d0;
                if !readable(e + 0x218, 8) || !readable(e + 0x8b0, 4) { continue; }
                let team = *((e + 0x820) as *const i64);
                let pos = *((e + 0x8b0) as *const i32);
                if !(0..=1).contains(&team) || !(0..=4).contains(&pos) { continue; }
                let idx = pos as usize + 5 * team as usize;
                if idx >= stats.len() { continue; }
                let st = stats[idx];
                for j in 0..8 { *((e + 0x1e0 + j * 8) as *mut u64) = st[j]; }
                *((e + 0x3e0) as *mut u64) = st[6] * 0x20 + 700;
                *((e + 0x3e8) as *mut u64) = st[6] * 0x32 + 1000;
                if COSMETIC_ON {
                    if let Some(nm) = names.get(idx) {
                        if !nm.is_empty() && read_str(e, 0x88) != *nm { replace_str(e, 0x80, nm.as_bytes()); }
                    }
                }
            }
        }));
    }
    ret
}

unsafe fn install_oracle_probe() -> Result<String, String> {
    let r = install_hook12(ORACLE_RVA, &ORACLE_TRAMP, oracle_detour as usize)
        .map(|s| format!("oracle 0x20566c0 {}", s))?;
    // 폴링 로거(최대 20분): 캡처되면 Game·mode/engine vtable·seed 로깅 후 종료
    std::thread::spawn(|| {
        for _ in 0..2400 {
            std::thread::sleep(Duration::from_millis(500));
            if ORACLE_CAPTURED.load(Ordering::Relaxed) {
                let b = exe_base() as u64;
                let g = ORACLE_GAME.load(Ordering::Relaxed);
                let mvt = ORACLE_MODEVT.load(Ordering::Relaxed);
                let evt = ORACLE_ENGVT.load(Ordering::Relaxed);
                let sd = ORACLE_SEED.load(Ordering::Relaxed);
                let rva = |x: u64| if x > b { x - b } else { x };
                log(&format!("[oracle] ★sim 발화! Game=0x{:x} mode_vt=0x{:x}(RVA 0x{:x}) engine_vt=0x{:x}(RVA 0x{:x}) seed=0x{:x} hits={}\n",
                    g, mvt, rva(mvt), evt, rva(evt), sd, ORACLE_HITS.load(Ordering::Relaxed)));
                break;
            }
        }
    });
    Ok(r)
}

// ── (16) [진단] 결과창 슬롯 렌더러 0x162f500(9-arg, R8=statblock stride 0x1a8) ──
// statblock 필드: 이름 String @+0x10/+0x18·챔프키 String @+0x20/+0x28(champion_icon)·포지션 @+0x1a0·통계 @+0x168/+0x170/+0x178.
//   ★9-arg라 detour도 9-arg(스택인자 유실 방지). 진단=String 내용 덤프(교체값 포맷·이름/얼굴 실체 확인).
// 0.5.2: ~~0.5.1 0x162f500~~ → **0xd1acf0**. L1~L3 NO-MATCH(264→261 instr·align=0.819)지만 코사인
//   0.99988 vs 2nd 0.99501로 갭 충분 + 콜사이트 2↔2, 콜러 컨테이너 0x1617e60→**0xd038a0이 L1-UNIQUE**로
//   독립 확정되어 자기일치. PROL-OK push8 12B. 신뢰도 HIGH.
const SLOT_RVA: usize = 0x1904640;   // ⬜0.5.5 미확정(SIM_PROBE_ON=false로 inert·install_hook12 fail-safe·0.5.4값 유지)
static SLOT_TRAMP: AtomicUsize = AtomicUsize::new(0);
static SLOT_HITS: AtomicU64 = AtomicU64::new(0);

// 게임(Rust) 할당자 — __rust_alloc(size, align)->ptr / __rust_dealloc(ptr, size=cap, align).
// statblock String 레이아웃 = {cap@+0x00, ptr@+0x08, len@+0x10}, drop이 cap 크기로 free ⟹ 세 필드 동시 갱신 필수.
// 0.5.2: ~~0.5.1 alloc 0x8b8210 / dealloc 0x8b8220~~ → **0x8b7f80 / 0x8b7f90**.
//   이 둘은 pdata 없는 5B `jmp rel32` 썽크다. 실체 함수를 먼저 매칭(alloc 0x25c5a40→0x25c4d30,
//   dealloc 0x25c5aa0→0x25c4d90 — 둘 다 **L1-UNIQUE**)한 뒤, 신 exe에서 그 실체를 가리키는 썽크를 역탐색하니
//   각각 **정확히 1개**. (참고: alloc 실체 0x25c4d30 = uinj ALLOC_RVA와 동일 함수 — 0.5.1에서도 그랬다.)
// 0.5.3(2026-07-30): 0.5.2 의 5B jmp 썽크(0x8b7f80/0x8b7f90)는 사라졌다.
//   alloc = HeapAlloc 래퍼 **0x28f7df0**(exe 전체에서 HeapAlloc 참조 함수가 이것 하나뿐 = 신원확정),
//           계약 3인자 (rcx 무시, rdx=flags, r8=size) -> rax, 실패 시 0.
//   dealloc = 범용 함수가 **인라인화로 소멸**(0.5.2 형태의 함수가 0.5.3 .text 에 없음)
//           ⟹ 그 본문과 동일한 `HeapFree(GetProcessHeap(), 0, ptr)` 를 직접 호출한다.
//           (0.5.2 본문의 `align>16 → ptr-8` 보정은 align=1 호출이라 해당 없음)
const RUST_ALLOC_RVA: usize = 0x29d7f20;   // ★0.5.5(구0.5.4=0x28f7df0) 다중앵커 투표 62/62 size927 동일 (game_alloc 소비처=slot/cosmetic OFF = inert)
static SELECTED_NAMES: Mutex<Vec<String>> = Mutex::new(Vec::new());

unsafe fn game_alloc(size: usize, _align: usize) -> usize {
    let b = exe_base(); if b == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize) -> usize = core::mem::transmute(b + RUST_ALLOC_RVA);
    f(0, 0, size)   // 0.5.3 impl 계약: rcx 무시·rdx=flags·r8=size
}

unsafe fn game_dealloc(ptr: usize, _size: usize, _align: usize) {
    if ptr <= 0x10000 { return; }
    let h = GetProcessHeap();
    if h != 0 { HeapFree(h, 0, ptr); }
}

// statblock의 String 필드(cap_off 기준 cap/ptr/len 3워드)를 새 문자열로 교체. 게임 힙 할당 → drop이 정상 free.
unsafe fn replace_str(sb: usize, cap_off: usize, s: &[u8]) -> bool {
    if s.is_empty() || s.len() > 128 || !readable(sb + cap_off, 24) { return false; }
    let old_cap = *((sb + cap_off) as *const usize);
    let old_ptr = *((sb + cap_off + 8) as *const usize);
    let np = game_alloc(s.len(), 1);
    if np < 0x10000 { return false; }
    core::ptr::copy_nonoverlapping(s.as_ptr(), np as *mut u8, s.len());
    *((sb + cap_off) as *mut usize) = s.len();          // cap
    *((sb + cap_off + 8) as *mut usize) = np;           // ptr
    *((sb + cap_off + 0x10) as *mut usize) = s.len();   // len
    if old_ptr >= 0x10000 && old_cap > 0 && old_cap < (1 << 20) { game_dealloc(old_ptr, old_cap, 1); }
    true
}

// athlete 구조에서 이름 String 추출(오프셋 미확정 → ASCII 문자열 첫 후보 채택, [ath] scan 로그로 검증).
unsafe fn athlete_name(ath: usize) -> Option<String> {
    for off in (0..0x100).step_by(8) {
        let s = read_str(ath, off);
        if s.len() >= 2 && s.len() <= 24 && s.chars().all(|c| c.is_ascii_graphic() || c == ' ') {
            return Some(s);
        }
    }
    None
}

unsafe fn read_str(base: usize, off: usize) -> String {
    if !readable(base + off, 16) { return String::new(); }
    let ptr = *((base + off) as *const usize);
    let len = *((base + off + 8) as *const usize);
    if ptr < 0x10000 || len == 0 || len > 256 || !readable(ptr, len) { return String::new(); }
    let bytes = core::slice::from_raw_parts(ptr as *const u8, len);
    String::from_utf8_lossy(bytes).into_owned()
}

extern "win64" fn slot_detour(rcx: usize, rdx: usize, r8: usize, r9: usize,
                              a5: usize, a6: usize, a7: usize, a8: usize, a9: usize) -> usize {
    let stub = SLOT_TRAMP.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        unsafe { core::mem::transmute(stub) };
    let n = SLOT_HITS.fetch_add(1, Ordering::Relaxed);
    if n < 12 && r8 >= 0x10000 {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            let sb = r8;
            let rd32 = |o: usize| -> u32 { if readable(sb + o, 4) { *((sb + o) as *const u32) } else { 0 } };
            let rd = |o: usize| -> u64 { if readable(sb + o, 8) { *((sb + o) as *const u64) } else { 0 } };
            log(&format!("[slot] #{} sb=0x{:x} champ='{}' pos={} stat= {} {} {}\n",
                n, sb, read_str(sb, 0x20), rd32(0x1a0), rd(0x168), rd(0x170), rd(0x178)));
        }));
    }
    // ★결과창 이름 교체: 슬롯 순번(포지션별 blue/red 쌍) → SELECTED[i] 이름. 캐시만 읽어 렌더 스레드 안전.
    if COSMETIC_ON && r8 >= 0x10000 {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            let names = SELECTED_NAMES.lock().unwrap_or_else(|e| e.into_inner());
            if names.is_empty() { return; }
            // ★positional 매핑(2026-07-20 실측 확정): SELECTED[0..5]=블루 pos0~4, SELECTED[5..10]=레드 pos0~4.
            //   결과창 슬롯은 두 팀 교대(#0 blue-p0, #1 red-p0, #2 blue-p1 …) ⟹ idx = (slot>>1) + 5*(slot&1).
            let sn = names.len();
            let k = (n as usize) % sn.max(1);          // 결과창 재진입 대비(슬롯 10칸 주기)
            let idx = if sn == 10 { (k >> 1) + 5 * (k & 1) } else { k };
            let nm = &names[idx.min(sn - 1)];
            if nm.is_empty() { return; }
            let orig = read_str(r8, 0x08);   // 교체 전 원본 = 등록된 선수 → 등록순서와 대조해 슬롯 매핑 도출
            let ok = replace_str(r8, 0x00, nm.as_bytes());
            if n < 12 {
                log(&format!("[slot] #{} pos={} 원본='{}' -> '{}' ({})\n",
                    n, if readable(r8 + 0x1a0, 4) { *((r8 + 0x1a0) as *const u32) } else { 0 },
                    orig, nm, if ok { "OK" } else { "실패" }));
            }
        }));
    }
    f(rcx, rdx, r8, r9, a5, a6, a7, a8, a9)
}

unsafe fn install_slot_probe() -> Result<String, String> {
    install_hook12(SLOT_RVA, &SLOT_TRAMP, slot_detour as usize)
        .map(|s| format!("slot 0x{:x}(9-arg 관측) {}", SLOT_RVA, s))
}

// ── (12) ★athlete 조회 하이브리드 주입 (프로토타입: comp_test sim을 슬롯0 선수 10명 미러) ──
// 0x140402840(id→athlete 조회, comp_test arm서 37회). caller가 comp_test arm이면 조회결과를
//   "슬롯0 선수(FIRST_ATH)"로 바꿔치기 → sim 딥카피가 그 선수 독립 엔티티 10명 스폰.
//   유니크 10명 accept 유지(조회는 정상 id로 됨) + 실전투는 미러. ⚠병렬 초핫패스 → caller 필터
//   + read-only(게임 ptr 반환) + catch_unwind. clone/id조작 불요(sim 딥카피가 독립화).
// ⛔★**죽은 상수 — 2026-07-22 0.5.2 마이그에서 신규 규명**. ATH_GET_RVA 0x402840은 **0.4.x 잔재**다:
//   0.5.1 exe 실측 시 이 주소는 함수시작이 아니라 0x402830 함수 중간(`8d 6a 40 48 8b 4d f0 ...`)이고
//   ATH_GET_PROLOGUE와 불일치 ⇒ install_athlete_hook은 애초에 Err.
//   (CURRENT.md 07-20 "0x140402840은 함수 아님 = EH funclet 내부" 기록과 일치.)
// ★그리고 **진짜 그 함수는 이미 이 파일에 있다**: 선언된 ATH_GET_PROLOGUE 17B가 0.5.1 `0xeaad40`
//   (= ATH_GET_SC_RVA)와 **정확히 일치**하고, je 타깃 산술도 자기일치한다
//   (0x402840+0x11+0xaa = 0x4028fb = 선언값 / 0xeaad40+0x11+0xaa = 0xeaadfb = 실제 je 타깃).
//   ⇒ ATH_GET_RVA와 ATH_GET_SC_RVA는 **같은 함수를 가리키는 두 상수인데 SC 쪽만 마이그돼 왔던 것**.
//   0.5.2 실주소가 필요하면 ATH_GET_SC_RVA(0xe3b200) 및 je타깃 0xe3b2bb를 쓰면 된다.
// ⚠**그럼에도 지금은 일부러 되살리지 않는다**: ①init()에서 `let _ = install_athlete_hook;`로 호출 비활성
//   ②HYBRID는 "선수중복"의 **폐기된 프로토타입**이고 진짜 해법은 server_dedup_real(07-20 인게임 검증완)
//   ③CT_ARM_LO/HI(comp_test arm 범위)가 0.5.0_2 기준이라 재도출 안 됨 = 되살리면 필터가 상시 false.
//   되살리려면 ghidra-re로 CT_ARM 범위부터 재확정할 것. 현 상태 = 프롤로그 검증 fail-safe로 inert(0.5.2 확인필).
const ATH_GET_RVA: usize = 0x402840;             // ⬜죽은 상수(0.4.x 잔재)·훅 호출 비활성·실체=ATH_GET_SC_RVA
const ATH_GET_JE_TARGET_RVA: usize = 0x4028fb;   // ⬜위와 동일(실체 기준 0.5.2값은 0xe3b2bb)
const ATH_GET_PROLOGUE: [u8; 17] = [0x56, 0x57, 0x48, 0x83, 0xec, 0x28, 0x48, 0x83, 0x79, 0x18, 0x00, 0x0f, 0x84, 0xaa, 0x00, 0x00, 0x00];
const CT_ARM_LO: usize = 0x13e1c00;              // ⬜미확정(0.5.0_2 기준·HYBRID 비활성이라 inert)
const CT_ARM_HI: usize = 0x13ea200;              // ⬜미확정(위와 동일)
const HYBRID_ENABLED: bool = true;
static HYBRID_ACTIVE: AtomicBool = AtomicBool::new(false);  // ★comp_test 실행 중에만 true(세이브 로드 레이스 회피)
static ATH_GET_TRAMP: AtomicUsize = AtomicUsize::new(0);
static ATH_GET_CALLER: AtomicUsize = AtomicUsize::new(0);
static FIRST_ATH: AtomicUsize = AtomicUsize::new(0);      // 슬롯0 선수 ptr(comp_test arm 첫 조회 기억)
static HYBRID_HITS: AtomicU64 = AtomicU64::new(0);

#[unsafe(naked)]
unsafe extern "win64" fn ath_get_shim() {
    core::arch::naked_asm!(
        "mov r10, [rsp]",
        "lea r11, [rip + {caller}]",
        "mov [r11], r10",
        "jmp {detour}",
        caller = sym ATH_GET_CALLER,
        detour = sym ath_get_detour,
    );
}

extern "win64" fn ath_get_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    let out = std::panic::catch_unwind(|| {
        let caller = ATH_GET_CALLER.load(Ordering::Relaxed);
        let stub = ATH_GET_TRAMP.load(Ordering::Relaxed);
        if stub == 0 { return 0usize; }
        let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(stub) };
        let orig = f(rcx, rdx, r8, r9);                  // 원본 조회 (athlete ptr or 0)
        let base = unsafe { exe_base() };
        if HYBRID_ENABLED && HYBRID_ACTIVE.load(Ordering::Relaxed) && base != 0 && orig != 0 {
            let rva = caller.wrapping_sub(base);
            if rva >= CT_ARM_LO && rva < CT_ARM_HI {
                // comp_test arm 조회: 슬롯0 선수로 미러
                let first = FIRST_ATH.load(Ordering::Relaxed);
                if first == 0 {
                    FIRST_ATH.store(orig, Ordering::Relaxed);   // 첫 조회 = 슬롯0 기억
                    return orig;
                }
                HYBRID_HITS.fetch_add(1, Ordering::Relaxed);
                return first;                            // 이후 조회 = 슬롯0 선수 반환(미러)
            }
        }
        orig
    });
    out.unwrap_or(0)
}

unsafe fn install_athlete_hook() -> Result<String, String> {
    let base = exe_base(); if base == 0 { return Err("module 0".into()); }
    let fn_addr = base + ATH_GET_RVA;
    if !readable(fn_addr, 17) { return Err("ath_get unreadable".into()); }
    let mut cur = [0u8; 17];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 17);
    if cur != ATH_GET_PROLOGUE { return Err(format!("ath_get prologue mismatch cur={:02x?}", cur)); }
    let stub = VirtualAlloc(0, 96, 0x1000 | 0x2000, 0x40);
    if stub == 0 { return Err("VirtualAlloc".into()); }
    // stub: push2+sub+cmp(11B) + je(재계산 6B) + jmp fn+17(12B abs)
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&ATH_GET_PROLOGUE[0..11]);       // 56 57 48 83 ec 28 48 83 79 18 00
    let je_target = base + ATH_GET_JE_TARGET_RVA;
    let je_end = stub + 11 + 6;                           // stub 내 je 명령 끝
    let rel = (je_target as i64 - je_end as i64) as i32;
    s.push(0x0f); s.push(0x84); s.extend_from_slice(&rel.to_le_bytes());
    s.extend_from_slice(&[0x48, 0xb8]);
    s.extend_from_slice(&(fn_addr + 17).to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    ATH_GET_TRAMP.store(stub, Ordering::Relaxed);
    // 원본: 12B abs jmp(shim) + 5 nop = 17B
    let d = ath_get_shim as usize;
    let mut patch = [0u8; 17];
    patch[0] = 0x48; patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&d.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    for i in 12..17 { patch[i] = 0x90; }
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 17, 0x40, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 17);
    VirtualProtect(fn_addr, 17, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 17);
    Ok(format!("ath_get hybrid hook @abs=0x{:x} stub=0x{:x}", fn_addr, stub))
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    log(&format!("[{}ms] === tfm2_comptest_unlock INIT ({} patches) ===\n", now_ms(), PATCHES.len()));
    unsafe {
        install_crash_logger();   // (26) ★어떤 훅보다 먼저 — 다음 크래시는 반드시 흔적을 남긴다
        for p in PATCHES {
            match apply_one(p) {
                Ok(st) => log(&format!("[patch] {} @0x{:x} {}\n", p.name, p.rva, st)),
                Err(e) => log(&format!("[patch] {} @0x{:x} 실패: {}\n", p.name, p.rva, e)),
            }
        }
        // dup 경고 디스패처 트램폴린 훅 (진짜 dup 게이트=검증기 B 경유)
        match install_disp_hook() {
            Ok(st) => log(&format!("[hook] dup_dispatcher {}\n", st)),
            Err(e) => log(&format!("[hook] dup_dispatcher 실패: {}\n", e)),
        }
        // [진단] insert 훅 프로브 (HashSet 병합 실측 + 검증기A 위조)
        match install_insert_probe() {
            Ok(st) => log(&format!("[probe] {}\n", st)),
            Err(e) => log(&format!("[probe] 실패: {}\n", e)),
        }
        // [진단] run 핸들러 진입 훅만 (UI 단일스레드 안전). ⚠서버훅(0x13d4af0)은 병렬 sim서
        //   호출되는 범용 대형함수라 진입 detour가 크래시 → 제거. 서버도달은 run 내부 push 콜사이트로.
        match install_hook12(RUN_RVA, &RUN_TRAMP, run_detour as usize) {
            Ok(st) => log(&format!("[run] hook {}\n", st)),
            Err(e) => log(&format!("[run] 실패: {}\n", e)),
        }
        // ★★v82 릴리스 정리 — **기능 훅은 진단 게이트 밖으로**.
        //   v81까지 병렬 발사·다시보기 재현·서버저장(승패 채집) 훅이 `CONC_PROBE_ON` 안에 있어서,
        //   진단을 끄면 **기능 자체가 죽는** 구조였다. 배포 전 반드시 분리해야 하는 지점.
        if CONC_ON {
            install_parallel();         // (25) 병렬 발사 + 전량 기록
            install_replay_real();      // (35) 다시보기 재현 — 컨텍스트 보관/주입 (★핵심 기능)
            // (24) ★실시간 킬스코어 현황판의 **데이터 공급원**. 이름이 tickmap이라 진단으로 보이지만,
            //   경기별 스코어·틱을 시드로 매칭해 채우는 건 이 훅이다(v82 실측: 빼면 현황판이 안 뜬다).
            match install_hook12(ORACLE_RVA, &TICK_TRAMP, tick_detour as usize) {
                Ok(st) => log(&format!("[tickmap] hook {}\n", st)),
                Err(e) => log(&format!("[tickmap] 실패: {}\n", e)),
            }
            // (18) 서버 저장 = 승패 채집 + 다시보기용 보조 스냅샷 지점
            match install_hook_n(HPUSH_RVA, &HPUSH_PROLOGUE, &HPUSH_TRAMP, hpush_detour as usize) {
                Ok(st) => log(&format!("[hpush] hook {}\n", st)),
                Err(e) => log(&format!("[hpush] 실패: {}\n", e)),
            }
        }
        // (13) [진단] sim 실행 본체 동시실행 프로브 — comp_test 귀속·스레드 구조 확증
        if CONC_PROBE_ON {
            log(&format!("[simbody] init_tid={} (UI/메인 스레드 기준값)\n", GetCurrentThreadId()));
            match install_hook12(SIMBODY_RVA, &SIMBODY_TRAMP, simbody_detour as usize) {
                Ok(st) => log(&format!("[simbody] hook {}\n", st)),
                Err(e) => log(&format!("[simbody] 실패: {}\n", e)),
            }
            // (14) 2차: 완주 폴러 + run_tick 스레드맵
            match install_hook12(POLLER_RVA, &POLLER_TRAMP, poller_detour as usize) {
                Ok(st) => log(&format!("[poller] hook {}\n", st)),
                Err(e) => log(&format!("[poller] 실패: {}\n", e)),
            }
            // (16) 서버 등록루프 반환코드(진단)
            match install_hook12(SREG_RVA, &SREG_TRAMP, sreg_detour as usize) {
                Ok(st) => log(&format!("[sreg] hook {}\n", st)),
                Err(e) => log(&format!("[sreg] 실패: {}\n", e)),
            }
            // (17) 경기 형성 = 등록 성공 이후의 다음 관문
            match install_hook12(MFORGE_RVA, &MFORGE_TRAMP, mforge_detour as usize) {
                Ok(st) => log(&format!("[forge] hook {}\n", st)),
                Err(e) => log(&format!("[forge] 실패: {}\n", e)),
            }
            // (21) 클라 결과 생성 카운터 — 프레임당 폴링으로는 못 세는 "같은 프레임 다중 도착"을 셈
            match install_hook12(RESULT_RVA, &RESULT_TRAMP, result_detour as usize) {
                Ok(st) => log(&format!("[result] hook {}\n", st)),
                Err(e) => log(&format!("[result] 실패: {}\n", e)),
            }
            // (22) 경고문구 사유 로깅 — RUN이 막힌 진짜 이유를 문자열 len으로 특정
            match install_hook_n(WARN_RVA, &WARN_PROLOGUE, &WARN_TRAMP, warn_detour as usize) {
                Ok(st) => log(&format!("[warn] hook {}\n", st)),
                Err(e) => log(&format!("[warn] 실패: {}\n", e)),
            }
        }
        // ★CGATE 훅 = **기능 게이트**(진단 아님) — 팝업 node를 여기서 캡처해야 −/+ 박스·범위 토글의
        //   화면 판정이 경기 전에도 동작한다.
        if CONC_ON || CONC_PROBE_ON {
            match install_hook12(CGATE_RVA, &CGATE_TRAMP, cgate_detour as usize) {
                Ok(st) => log(&format!("[cgate] hook {}\n", st)),
                Err(e) => log(&format!("[cgate] 실패: {}\n", e)),
            }
        }
        // ★CSEND 훅 = **기능 게이트**(진단 아님) — (v8) 순차 발사의 트리거이자 (19) 큐 재무장의
        //   트리거다. 프로브를 꺼도 CONC_ON이면 반드시 설치돼야 한다(안 하면 2번째 경기가 안 나감).
        if CONC_ON || QUEUE_ON || CONC_PROBE_ON {
            match install_hook12(CSEND_RVA, &CSEND_TRAMP, csend_detour as usize) {
                Ok(st) => log(&format!("[csend] hook {}\n", st)),
                Err(e) => log(&format!("[csend] 실패: {}\n", e)),
            }
        }
        // (19) 결과 큐잉(병렬 수집) — ⛔v6 크래시로 기본 OFF
        if QUEUE_ON {
            match install_hook_n(ARRIVE_RVA, &ARRIVE_PROLOGUE, &ARRIVE_TRAMP, arrive_detour as usize) {
                Ok(st) => log(&format!("[arrive] hook {}\n", st)),
                Err(e) => log(&format!("[arrive] 실패: {}\n", e)),
            }
        }
        let _ = (SRV_RVA, &SRV_TRAMP, srv_detour as usize);   // srv 훅 비활성(크래시 방지)
        // [진단] push 도달 프로브 (INT3) — ⚠0.5.1 비활성: PUSH_RVA=0.5.0_2 STALE인데
        //   install_push_probe는 프롤로그 검증 없이 INT3 blind write → 명령 중간 파괴 위험.
        //   재활성 조건 = ghidra-re로 0.5.1 push 콜사이트 재핀 후.
        let _ = install_push_probe;
        log("[push] 프로브 비활성(0.5.1 STALE RVA, blind write 방지)\n");
        // [진단] dedup insert / spawn copy 카운터 ((a)dedup순회 vs (b)원본순회 판정)
        match install_hook_n(DEDUP_INS_RVA, &DEDUP_INS_PROLOGUE, &DEDUP_INS_TRAMP, dedup_ins_detour as usize) {
            Ok(st) => log(&format!("[dedup_ins] {}\n", st)),
            Err(e) => log(&format!("[dedup_ins] 실패: {}\n", e)),
        }
        match install_hook_n(SPAWN_CP_RVA, &SPAWN_CP_PROLOGUE, &SPAWN_CP_TRAMP, spawn_cp_detour as usize) {
            Ok(st) => log(&format!("[spawn_cp] {}\n", st)),
            Err(e) => log(&format!("[spawn_cp] 실패: {}\n", e)),
        }
        // [진단] 로딩빌더 훅 = state4↑ 도달(서버 accept) 판정
        match install_hook_n(LOADING_RVA, &LOADING_PROLOGUE, &LOADING_TRAMP, loading_detour as usize) {
            Ok(st) => log(&format!("[loading] {}\n", st)),
            Err(e) => log(&format!("[loading] 실패: {}\n", e)),
        }
        // ★collect(0.5.1 0x16203f0 재핀) = UI 선택 관측 + 중복 시 등록용 distinct 치환. 풀은 파일서 복원.
        // ★comp_test 아이템 주입(모드템 id≥30 지정) = cfg + 카테고리→id 변환 POST 훅
        load_item_cfg();
        match install_itemconv_hook() {
            Ok(st) => log(&format!("[item] {}\n", st)),
            Err(e) => log(&format!("[item] 실패: {}\n", e)),
        }
        pool_load();
        match install_collect_hook() {
            Ok(st) => log(&format!("[collect] {}\n", st)),
            Err(e) => log(&format!("[collect] 실패: {}\n", e)),
        }
        let _ = install_athlete_hook;   // ath_get 훅 비활성(전투 미반영 = 무효, collect로 대체)
        // [진단] sim 오라클 0x20566c0 직접 훅 = comp_test sim 배선·Game·mode vtable 캡처(런타임)
        if SIM_PROBE_ON {
            match install_oracle_probe() {
                Ok(st) => log(&format!("[oracle] {}\n", st)),
                Err(e) => log(&format!("[oracle] 실패: {}\n", e)),
            }
            // ef1ea0 최소 훅(game_ctx 캡처만·무로깅) = ATH_GET shadow-call 레지스트리 확보. 4-arg 확정=인자유실無.
            match install_ef1ea0_probe() {
                Ok(st) => log(&format!("[ef1ea0] {}\n", st)),
                Err(e) => log(&format!("[ef1ea0] 실패: {}\n", e)),
            }
            // [진단] 결과창 슬롯 렌더러 = statblock 내용 덤프(이름/챔프키/포지션 실체 확인)
            match install_slot_probe() {
                Ok(st) => log(&format!("[slot] {}\n", st)),
                Err(e) => log(&format!("[slot] 실패: {}\n", e)),
            }
        }
    }
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(CompTestExt);
    reg.set_server_extension(CompTestServerExt);
    reg
}

// 클라 확장 = UI 주입 훅 설치(매프레임 멱등). ★로더 훅은 "늦게" 설치해야 다른 모드 훅 위에 체인된다.
struct CompTestExt;
impl ModExtension for CompTestExt {
    fn on_init(&self, _s: &mut Scene, _u: &mut GameUI, _a: &mut Assets) {}
    fn post_update(&self, _scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        if CONC_PROBE_ON { tickmap_dump_periodic(); }   // (14) 틱맵 덤프 = UI 스레드 전용
        watch_tick();                                   // (20) 러너 상태 폴링(읽기 전용)
        resultview_tick();                              // (23) 결과화면 이탈 시 기록탭 원복
        refresh_tick();                                 // (28) 마지막 저장 후 목록 재생성
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| scope_tick(_ui))); // (29) 범위 토글
        killscore_tick();                               // (24) 실시간 킬스코어 현황판 = **기능**(진단 아님)
        // (27) 킬스코어 현황 오버레이 — 게임 로딩바 대신 경기 N개 상태를 한 화면에
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| killscore_overlay(_ui)));
        if !UI_INJECT_ON { return; }
        // 설치 순서 무관 — item_tactics 가 training 행을 append 방식으로 전환(07-21)해서 서로의 노드를
        //   지우지 않는다. (구 replace_children 시절엔 늦게 설치해 그 뒤에 체인해야 했음.)
        unsafe { let _ = uinj::install(); }
        // [진단] 어디서 막히는지 단계별 계측을 주기적으로(초반 5회) 남긴다.
        //   install(1=copy1/2=copy2/3=둘다) → loader 발화 → training 경로 목격 → PATH 일치 → 행 발견 → 주입
        let t = UINJ_TICK.fetch_add(1, Ordering::Relaxed);
        if t % 300 == 0 && UINJ_LOGS.fetch_add(1, Ordering::Relaxed) < 5 {
            let paths = uinj::SEEN_PATHS.lock().unwrap_or_else(|e| e.into_inner()).clone();
            log(&format!("[uinj] install={} loader={} train_seen={} path_hit={} rows={} injected={} paths={:?}\n",
                uinj::INSTALL_OK.load(Ordering::Relaxed),
                uinj::LOADER_CALLS.load(Ordering::Relaxed),
                uinj::TRAIN_SEEN.load(Ordering::Relaxed),
                uinj::PATH_HIT.load(Ordering::Relaxed),
                uinj::ROWS_FOUND.load(Ordering::Relaxed),
                uinj::INJECTED.load(Ordering::Relaxed), paths));
        }
        let n = uinj::INJECTED.load(Ordering::Relaxed);
        if n > 0 && !UINJ_LOGGED.swap(true, Ordering::Relaxed) {
            log(&format!("[uinj] ★comp_test 드롭다운 주입 {}개(10행×3칸)\n", n));
        }
        if n == 0 { return; }
        // ── 옵션 세팅 + 선택 폴링 ──
        // 옵션 = ["(게임 기본)"] + 모드 최종템. 인덱스 0 = 미지정(게임이 정한 목표빌드 유지).
        let finals = MOD_FINALS.lock().unwrap_or_else(|e| e.into_inner()).clone();
        if finals.is_empty() { return; }
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            // 화면이 새로 열리면 노드가 재생성되므로 대표 노드의 runner 주소로 재세팅 여부 판정.
            // [진단] 런타임 트리에 내 노드가 실제로 생겼는지 — 안 생겼으면 blue0 자식 id를 그대로 덤프.
            if t % 120 == 0 && DD_DIAG.fetch_add(1, Ordering::Relaxed) < 6 {
                match find_node(&_ui.root, "blue0") {
                    Some(row) => {
                        let ids: Vec<String> = row.child.iter().map(|c| c.id.as_str().to_string()).collect();
                        log(&format!("[dd] blue0 자식({}) = {:?}\n", ids.len(), ids));
                    }
                    None => log("[dd] blue0 런타임 노드 없음(팝업 미오픈이거나 트리 다름)\n"),
                }
            }
            let cur_rb = find_rb(&_ui.root, &uinj::dd_id(0, 0)).unwrap_or(0);
            if cur_rb == 0 { return; }
            if cur_rb != DD_LAST_RB.load(Ordering::Relaxed) {
                // 게임 원래 카테고리 7개 + 모드 최종템. 모드템 라벨은 i18n 참조라 게임이 번역명으로 표시.
                let mut opts: Vec<String> = Vec::with_capacity(finals.len() + 7);
                for v in VANILLA_OPTS.iter() { opts.push(v.to_string()); }
                for (_, k) in finals.iter() { opts.push(format!("#asset/base/text/item?{}.name", k)); }
                let mut ok = 0;
                for s in 0..10 { for j in 0..3 {
                    if dd_set_options(&_ui.root, &uinj::dd_id(s, j), &opts, 0) { ok += 1; }
                } }
                DD_LAST_RB.store(cur_rb, Ordering::Relaxed);
                log(&format!("[dd] 옵션 세팅 {}칸 × {}개(최종템 {})\n", ok, opts.len(), finals.len()));
            }
            // ★오버레이 좌표를 네이티브 item0/1/2 에 맞춤(읽기 전용 추종 — 좌표는 item_tactics 소관).
            //   상대가 3칸(146/296/446 w140)이든 4칸(146/258/370 w104)이든 자동으로 정확히 겹친다.
            // ⛔런타임 좌표 추종 + 네이티브 visible 강제 = 폐기(2026-07-21 실측).
            //   ① 4상태 박스에 직접 write 하니 오버레이 히트박스가 깨져 **클릭이 관통**(그 전엔 정상 동작).
            //   ② visible 강제는 게임이 매 프레임 되돌려 화면이 떨림.
            //   ⟹ 좌표는 템플릿 선언값(ct_dd)만 쓰고 런타임에 손대지 않는다. 3칸/4칸 대응은 별도 방식으로.
            // 선택 폴링(10프레임마다) → ITEM_OVERRIDE. sel 0 = 미지정.
            if t % 10 != 0 { return; }
            let mut ov = ITEM_OVERRIDE.lock().unwrap_or_else(|e| e.into_inner());
            if ov.len() < 16 { *ov = vec![[0u64; 3]; 16]; }
            for s in 0..10 { for j in 0..3 {
                if let Some(sel) = dd_selected(&_ui.root, &uinj::dd_id(s, j)) {
                    ov[s][j] = sel_to_item_id(sel, &finals);
                }
            } }
        }));
    }
}

// 서버 확장 = Database 접근 경로(모드 아이템 목록 덤프용).
//   db 베이스 = champion_patch_statistics 절대주소 − 0x16698 (item_tactics probe_db와 동일 방식).
struct CompTestServerExt;
impl ModServerExtension for CompTestServerExt {
    fn on_server_start(&self, ctx: &mut ServerModContext) {
        let cps = &ctx.database.champion_patch_statistics as *const _ as usize;
        let db = cps.wrapping_sub(0x16698);
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { dump_mod_items(db); }));
    }
}
declare_mod!(init);
