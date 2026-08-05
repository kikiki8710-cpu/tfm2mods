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
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

#[path = "ui_inject.rs"]
mod uinj;   // comp_test 아이템칸 모드 소유 드롭다운 오버레이(로더 훅, 체인 설치)
use std::time::{SystemTime, UNIX_EPOCH, Duration};

const MOD_ID: &str = "tfm2_comptest_unlock";

// ── 패치 테이블(**0.5.4**, image_base 0x140000000) ──
// ★0.5.4 재핀(2026-08-05): 마스크 시그(rip-rel/분기 rel 와일드카드) 전역 유일 + 컨테이너 대응 투표 2방법 교차.
//   ⚠orig 바이트가 바뀐 사이트 6건: server_dedup_real(d4→d3)·roster_count_gate(5e→4d)·collected_gate(10→17)
//   ·collect_err_gate(50→57)·run_push_gate(15→12)·★server_roster_min(명령 자체가 lea 4B→add/mov 3+3B 로 교체).
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
    Patch { name: "no_stamina_cost", rva: 0x20ecf0c,   // ★0.5.4(구0.5.3=0x17f6f44 / 0.5.2=0xe93b2d) `sub rax,5` imm·문맥 8/11
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
    Patch { name: "dr_inline_a", rva: 0x2306164,   // ★0.5.4 재핀(구0.5.3=0x18d9436)   // 0.5.3 신규(구 daily_remaining 0x1f14090 인라인 분산)
            orig: &[0x4c, 0x0f, 0x44, 0xe2],       // cmove r12,rdx
            fixed: &[0x0f, 0x1f, 0x40, 0x00] },    // 4B nop → used=0(xor r12d 선행)
    // 사이트 B: ★진짜 클라 게이트(2차 스윕 디컴 확정) — `if(4<count && rec_id==outer_id) ok=0` 후
    //   `[node+0x261]=!ok`(run 버튼)·`[open_tactics+0x261]`에 같은 r13b 공유 → 한 방에 둘 다 해제.
    Patch { name: "dr_inline_b", rva: 0x2310c86,   // ★0.5.4 재핀(구0.5.3=0x18e3fd6)
            orig: &[0x41, 0x20, 0xc5],             // and r13b,al (exhausted 플래그 합성)
            fixed: &[0x45, 0x30, 0xed] },          // xor r13b,r13b → exhausted=0, 직후 je 항상 taken
    // ~~사이트 C: RUN 핸들러 0x18f18c7 cmove~~ → ★제거(2026-07-30 2차 스윕): 게이트가 아니라
    //   클라가 요청 페이로드에 넣는 **시드 성분**(seed = (used|X<<32) ^ epoch_ms)이었음.
    //   서버는 자기 레코드로 판정하므로 무의미 + 시드 변화 부작용 회피 위해 원본 유지.
    // 사이트 D: 버튼 빌더A 컨테이너 0x19866f0(앵커) 내부, cmove @0x1987a3d — 버튼 회색화의 실체.
    Patch { name: "dr_inline_d", rva: 0x23ce6bc,   // ★0.5.4 재핀(구0.5.3=0x1987a3d)
            orig: &[0x4c, 0x0f, 0x44, 0xf8],       // cmove r15,rax
            fixed: &[0x0f, 0x1f, 0x40, 0x00] },
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
    Patch { name: "daily_inc_gate", rva: 0x20e8246,   // ★0.5.4(구0.5.3=0x17f239c / 0.5.2=0xe8cb20) `cmp rax,4` imm·문맥 10/11
            orig: &[0x04], fixed: &[0xff] },
    // ★★(2b-2) 서버 **사전거부 게이트** [game_core] — 0.5.3 일일제한 잔존의 진범(2026-07-30 2차 스윕).
    //   서버 핸들러엔 daily 게이트가 **2개**: ①위 inc_gate(카운터 증가 지점) ②이 pre-gate(수락 판정).
    //   0x17ef5d2 call map.get(base+0x16a28, 오늘날짜) → 0x17ef5e3 `cmp [rax+0x1dc],esi`(오늘 레코드?)
    //   → 0x17ef5ef `cmp qword [rax+0x1d0],4`·0x17ef5f7 jbe→허용 / fall-through 0x17ef5fd
    //   `mov byte [rsp+0x20],1` = **거부코드 1(no_attempts) 생산** → 0x17ef616 call 거부 디스패처
    //   = 유저가 본 "오늘은 더 이상…" 안내문구의 실체. code 1 생산지는 exe 전체 이 2곳뿐(둘 다 daily 직후).
    //   전체 명령 실측: `48 83 b8 d0 01 00 00 04 | 0f 86 05 2d 00 00`(전역 1히트·클론 없음).
    //   imm8 ff(-1, sign-extend) → jbe 항상 taken = 무제한. inc_gate와 같은 라이브 함수 내.
    Patch { name: "server_pregate", rva: 0x20e5471,   // ★0.5.4 재핀(구0.5.3=0x17ef5f6)   // 0.5.3 신규 — `cmp qword [rax+0x1d0],4` imm8
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
    Patch { name: "server_dedup_real", rva: 0x2126f73,   // ★0.5.4(구0.5.3=0x1830df0 / 0.5.2=0xec7758)
            orig: &[0x0f, 0x85, 0xd3, 0x00, 0x00, 0x00],
            fixed: &[0x66, 0x0f, 0x1f, 0x44, 0x00, 0x00] },
    // 0.5.2: ~~0.5.1 0x1615495~~ → 0xd00ee5. 컨테이너 0x1615030→0xd00a80 = **L1-UNIQUE(스켈레톤 바이트동일
    //   ·357→357 instr·align=1.0000)** ⇒ 함수 내부 오프셋 전부 보존, instr#291→#291·orig 75 76 실측 MATCH.
    Patch { name: "allow_dup_players", rva: 0x2311131,   // ★0.5.4(구0.5.3=0x18e4481 / 0.5.2=0xd00ee5) ⚠점프 거리 76→47
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
    Patch { name: "server_dedup", rva: 0x20e42d1,   // ★0.5.4(구0.5.3=0x17ee49c / 0.5.2=0xe8b5fa) 문맥 11/11·no-op 유지
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
    Patch { name: "btn5v5_roster_min_a", rva: 0x23ceae4,     // ★0.5.4(구0.5.3=0x1987e64 / 0.5.2=0xd967cf) cmp rbx,0xa → 5
            orig:  &[0x48, 0x83, 0xfb, 0x0a, 0x0f, 0xb6, 0xf9, 0xb8],
            fixed: &[0x48, 0x83, 0xfb, 0x05, 0x0f, 0xb6, 0xf9, 0xb8] },
    // ⛔0.5.3 미해결: `cmp r13,0xa; setb r13b` 사이트가 0.5.3 에 없다(레지스터·형태 모두 변경 추정).
    //   컨테이너(0.5.2 0xcf7970)도 앵커 없음 → 투표 컨테이너 안에서 후보 0건. rva 0 = 스킵.
    //   영향: 빌더B 경로의 버튼 disabled 조건이 10명 유지(빌더A·경고문구는 완화 적용됨).
    Patch { name: "btn5v5_roster_min_b", rva: 0,             // ⬜0.5.3 미해결(구0.5.2=0xcf7b68)
            orig:  &[0x49, 0x83, 0xfd, 0x0a, 0x41, 0x0f, 0x92, 0xc5],
            fixed: &[0x49, 0x83, 0xfd, 0x05, 0x41, 0x0f, 0x92, 0xc5] },
    Patch { name: "btn5v5_warn_text",    rva: 0x23ce6fc,     // ★0.5.4(구0.5.3=0x1987a7d / 0.5.2=0xd9662c) cmp rdi,0xa → 5
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
    Patch { name: "server_roster_min", rva: 0x2126ed0,  // ★0.5.4(구0.5.3=0x1830d2e / 0.5.2=0xec768e) 필요치 2×N → 1×N
            orig:  &[0x48, 0x01, 0xdb],                 // 0.5.4 `add rbx,rbx` (0.5.3=`lea rax,[rsi+rsi]` 4B)
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
    Patch { name: "roster_count_gate", rva: 0x231e155,   // ★0.5.4(구0.5.3=0x18f14ab / 0.5.2=0xd0a74c)
            orig: &[0x0f, 0x83, 0x4d, 0x01, 0x00, 0x00],
            fixed: &[0xe9, 0x4e, 0x01, 0x00, 0x00, 0x90] },
    // (8) collected != required 게이트 [클라]: 0x101c330 `jne →abort`. collect 반환 len != required면 abort.
    //     collect 무-dedup(중복10개 push)이면 ==여야 하나, 실측 막힘 → 이 게이트가 실제 abort일 가능성. NOP.
    Patch { name: "collected_gate", rva: 0x231e142,  // ★0.5.4(구0.5.3=0x18f149f / 0.5.2=0xd0a740) 문맥 7/11·orig 75 10 불변
            orig: &[0x75, 0x17], fixed: &[0x90, 0x90] },
    // (9) ⚠collect==-1 게이트 [클라, 위험]: 0x101c318 `je →abort`. collect가 슬롯서 -1(미선택) 반환시 abort.
    //     NOP=무효슬롯도 진행→garbage build→서버 크래시 위험. 판정용: 크래시=drop커밋 -1 확정(근본).
    Patch { name: "collect_err_gate", rva: 0x231e127,  // ★0.5.4(구0.5.3=0x18f1484 / 0.5.2=0xd0a728) ⚠점프 거리 6a→50
            orig: &[0x74, 0x57], fixed: &[0x90, 0x90] },
    // (6) run 핸들러 r15 게이트 [클라]: 0x101c9e1 `cmp r15,-1; je 0x14101c453` (r15=빌드산출물).
    //     — r15(빌드산출물 [rbp+0x1a50])==-1이면 메일박스 push 전 조용히 abort(서버 미전송).
    //     je NOP → r15==-1여도 push 강행(서버 전송). ⚠무효 빌드면 서버가 깨진 0x19c0 메시지 받아
    //     크래시 위험(세이브 백업됨). 실험: sim진입=성공 / 크래시=빌드 진짜무효.
    Patch { name: "run_push_gate", rva: 0x231e838,  // ★0.5.4(구0.5.3=0x18f1b95 / 0.5.2=0xd0adf1) ⚠변위 6cfaffff→15faffff
            orig: &[0x0f, 0x84, 0x12, 0xfa, 0xff, 0xff],
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
const LOG_ENABLED: bool = false;
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
const CT_REGION_LO: usize = 0x20d5bf0;   // ★0.5.4(구0.5.3=0x17e0240) .pdata 함수 시작       // 0.5.2(구0.5.1환산 0xf1d2c0, 컨테이너 매칭 HIGH)
const CT_REGION_HI: usize = 0x20ff156;   // ★0.5.4(구0.5.3=0x180920f) = .pdata 함수 끝       // 0.5.2(= LO + pdata 크기 0x25675)
const CT_CLIENT_LO: usize = 0x2300000;   // ★0.5.4(구0.5.3=0x18c0000) 클라 사이트 0x2306164~0x23ceae4 포괄       // 0.5.2(~~0xf50000~~ = 0.5.1서 이미 무효였던 범위)
const CT_CLIENT_HI: usize = 0x23e0000;   // ★0.5.4(구0.5.3=0x19a0000)       // 0.5.2(~~0xf80000~~)
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
const RUN_RVA: usize = 0x231de30;   // ★0.5.4(구0.5.3=0x18f1180) HOOK_PROLOGUE12 실측 MATCH // 0.5.2(구0.5.1=0x161eab0, **L1-UNIQUE 스켈레톤 바이트동일**·628 instr·PROL-OK push8 12B)
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
const FN_DD_SETOPT_RVA: usize = 0x1bfc80;
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
const ITEMCONV_RVA: usize = 0x18429d0;
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

const COLLECT_RVA: usize = 0x18f2b50;   // 0.5.2(구0.5.1=0x16203f0, **L1-UNIQUE**·145 instr·PROL-OK push8 12B)
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
const ATH_GET_SC_RVA: usize = 0x1794280;           // shadow-call: rcx=game_ctx+0x16b90, rdx=&id → rax(0=miss), athlete*=[rax]
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
const ORACLE_RVA: usize = 0xeb6590;   // 1틱 오케(run one tick), 프롤로그=HOOK_PROLOGUE12
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
const SLOT_RVA: usize = 0x1904640;
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
const RUST_ALLOC_RVA: usize = 0x28f7df0;
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
