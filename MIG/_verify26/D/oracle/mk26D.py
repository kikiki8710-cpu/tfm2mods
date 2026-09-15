# -*- coding: utf-8 -*-
"""26차 배치 D patch.json 생성 (mkpatch 참조구현 경유). 실행: python -X utf8 mk26D.py → _verify26/D/patch.json"""
import sys, io
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch
sys.stdout.reconfigure(encoding='utf-8')

p = mkpatch.Patch(round=26, batch="D")
ORC = u"오라클 실행 확인(26차 D · o207.exe 85/85 MATCH · resolve_fight(hidden) 진입 → resolve_fight_full(len>8 우회/캐시 미스) → uncached · 독립 재구현 대조)"
ORC8 = u"오라클 실행 확인(26차 D · o208.exe · AgentVerHamster::new(pub)+get_input 직접 호출)"

# ── 207 resolve_fight_uncached ──────────────────────────────────────────
# G15: 부정문 「임계 아님」이 kind=임계 와 자기모순 — 낱말을 고친다(값·동작 불변)
p.fix("/specs[207]/consts[21]/meaning",
      old=u"임계 아님 · stride 아님 · 인덱스 상한이라 등록",
      new=u"판정 노브가 아니라 로컬 배열 경계검사의 비교 상한(icmp ult · 5칸)이라 등록 · stride 아님",
      evidence=u"m10.ll:46753 `icmp ult i64 %idx, 5` → panic_bounds_check(…,5) — CMP_ORD 소비라 파생 kind=임계 가 맞고, 「임계 아님」 문면만 자기모순(G15 NEG)",
      behavior_change=False, found_by="reused", kind=u"실오류")
# G16 P1: v3 role(=v2 note) 에 IR 레지스터 대응이 없어 자리 매핑 보류 — define m10.ll:43974 원문대로 `%k` 를 적는다
regs = [
 (0, u"레이아웃 = signature.return", u"`%0`(sret out-ptr · dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(64)) · 레이아웃 = signature.return"),
 (1, u"reach version=2 접기에서", u"`%1`(i64 noundef) · reach version=2 접기에서"),
 (3, u"(배치 L) (배치 L) 미사용", u"`%4`(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728)) · (배치 L) 미사용"),
 (4, u"len==0 이면 L381 조기 반환(Hold) | (배치 L) (배치 L) 496·501", u"`%5`(data_ptr · noalias noundef nonnull readonly align 8 captures(address, read_provenance)) / `%6`(len · noundef range(i64 0, 1152921504606846976)) · len==0 이면 L381 조기 반환(Hold) | (배치 L) 496·501"),
 (5, u"len==0 이면 L381 조기 반환(Hold) | (배치 L) (배치 L) 미사용(their_*", u"`%7`(data_ptr · noalias noundef nonnull readonly align 8 captures(address, read_provenance)) / `%8`(len · noundef range(i64 0, 1152921504606846976)) · len==0 이면 L381 조기 반환(Hold) | (배치 L) 미사용(their_*"),
 (6, u"(배치 L) 소비처 서술 필요 | (배치 L) (배치 L) 515 switch", u"`%9`(i8 noundef) · (배치 L) 515 switch"),
 (7, u"(배치 L) (배치 L) 미사용 — 타워 효과는", u"`%10`(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable_or_null(1728) · null=None) · (배치 L) 미사용 — 타워 효과는"),
 (8, u"999 비교는 dbg 없는 호이스트 명령(%93)", u"`%11`(i64 noundef) · 999 비교는 dbg 없는 호이스트 명령(%93)"),
 (9, u"arrivals[ai] 값은 L463 arrived 클로저", u"`%12`(data_ptr · noalias noundef nonnull readonly align 8 captures(none)) / `%13`(len · noundef range(i64 0, 1152921504606846976)) · arrivals[ai] 값은 L463 arrived 클로저"),
 (10, u"(배치 L) 소비처 서술 필요 | (배치 L) (배치 L) 508", u"`%14`(i64 noundef) · (배치 L) 508"),
]
for j, old, new in regs:
    p.fix("/specs[207]/sig/params[%d]/note" % j, old=old, new=new,
          evidence=u"m10.ll:43974 define 원문(인자 15 = sret %0 · version %1 · data.cache %2 · data.context %3 · champ %4 · allies %5/%6 · enemies %7/%8 · committed_dir %9 · tower %10 · judge_accuracy %11 · arrivals %12/%13 · baseline %14) · 속성은 그 줄에 실제로 있는 것만. ★v3 role = v2 note 이고 v2 role(레지스터 대응이 이미 적혀 있음)은 mkspec3 L565 가 버린다 — 그래서 note 쪽에 적는다(경로 키 note · v3 에 없어 force)",
          behavior_change=False, found_by="reused", kind=u"보강", force=True)

# ── 208 get_input ──────────────────────────────────────────────────────
# G12: 132 는 ProfTimer drop(L906) 의 PHASE_NANOS/PHASE_CALLS bounds — L855 가 아니다
p.fix("/specs[208]/consts[2]/src_line", old=855, new=906,
      evidence=u"m14.ll:39007 `%112 = icmp ult i64 %111, 132` ;L185<825<825<1004<906 · 39011 panic_bounds_check(%111, 132) — 루트 줄 906(drop(_t_sai)). L855 에 있는 것은 `store i64 26`(39,965)뿐",
      behavior_change=False, found_by="reused", kind=u"실오류")
# mem[38] 오프셋 오기: small_action@tag 는 0x2858+0xb1 = 0x2909 (gep 10505) — 0x28b1 은 존재하지 않는 자리
p.fix("/specs[208]/mem[38]/offset", old=u"0x28b1", new=u"0x2909",
      evidence=u"m14.ll:39196 `%210 = getelementptr inbounds nuw i8, ptr %1, i64 10505` ;L909 (10505 = 0x2909) · 이 행의 note 자신이 %210 재로드라 적음 · tcxdict AgentVerHamster --deep: 0x2909 small_action@tag(Niche 1B) · 0x28b1 은 어떤 필드도 아님. 같은 명세 mem[3]/[85] 는 0x2909 로 맞게 적음(G20 R2 자기모순)",
      behavior_change=True, found_by="new", kind=u"실오류")
p.fix("/specs[208]/notes[0]", old=u"C3 경고 5건(0x28b1·", new=u"C3 경고 5건(0x2909·",
      evidence=u"위 mem[38] 정정과 같은 근거(gep 10505 = 0x2909)",
      behavior_change=False, found_by="new", kind=u"실오류")
# mem[79] 보강: L1089 는 %58 이 아니라 재로드 vtable %1138 경유 · 함수 전체 tick() 호출 수
p.fix("/specs[208]/mem[79]/note", old=u"%58 로 L1005(%634) · L1025(%1058) · L1049(%1132) · L1089(%1139) 4회 호출",
      new=u"%58 로 L1005(%634) · L1025(%1058) · L1049(%1132) 3회 + L1089(%1139 · 재로드 vtable %1138(%1135 data 재로드) 경유) 1회 — L1005 는 mem[18] 과 중복 계상. 함수 전체 tick() 호출 = 8회(L844·852·911·950·1005·1025·1049·1089)",
      evidence=u"m14.ll:38894·38934·39205·39729·40127·41351·41543 `invoke i64 %58(ptr %54)` 7회 + 41556 `%1139 = invoke i64 %1138(ptr %1135)` 1회 (grep '%58(' = 7)",
      behavior_change=False, found_by="new", kind=u"보강")
# sig.tls: 함수 전체 기준으로 확정
p.fix("/specs[208]/sig/tls/name", old=u"없음(배치 M 범위)", new=u"없음(함수 전체 m14.ll:38800~41939 — `llvm.threadlocal.address`/`LocalKey`/`call_once` 참조 0건 · 26차 D grep)",
      evidence=u"grep -c 'threadlocal|LocalKey|call_once' _next/reach/e900b0.ll = 0 · 접촉 전역은 atomic 3종뿐(38943 ENABLED · 39037/39041 PHASE_NANOS/CALLS · 39749~39753 CNT_DM_IDLE_INPUT)",
      behavior_change=False, found_by="new", kind=u"보강")

# ── ev 상향(오라클 실행) ────────────────────────────────────────────────
# 207 consts
for j, why in [
 (0, u"version=2 vs 1/0 케이스(V1~V1e·V0) 에서 시드 분기가 갈리고 각각 재구현과 일치"),
 (1, u"조기 반환 케이스 E0a/E0b: line=line_absolute=3(Hold) · net 0 · 태그 3개 0"),
 (2, u"na=9/ne=9 케이스(B9a~c): 앞 5명만 모델링한 재구현과 일치(6번째 이후 무시)"),
 (3, u"시드 집합 해시 상수 0x9E3779B97F4A7C15 로 NoiseRng 시드를 재현해 misjudge 결과(judge≤999 케이스 40여 건)가 비트 일치"),
 (4, u"적 해시 상수 0x517CC1B727220A95 — 위와 동일 근거"),
 (5, u"rotate_left(set_h,17) — 위와 동일 근거(seed = game.seed() ^ rotl17 ^ champ.id 로 NoiseRng::new 에 그대로)"),
 (6, u"version≤1 케이스 6건: bucket = tick / max(tps*2,1) 로 재현한 시드가 일치(tick 0/59/120/3000/7777)"),
 (7, u"위와 동일(umax 하한 1 포함 식으로 재현)"),
 (8, u"위와 동일(champ.id ^ (bucket << 40))"),
 (9, u"horizon = tps*6 = 360 으로 재현한 틱 전개(dt 누적·horizon-t 클램프)가 85/85 일치"),
 (10, u"J999(노이즈 적용)·J1000(무노이즈) 케이스가 각각 재구현과 일치"),
 (11, u"v * error_ratio_noise / 100 (sdiv) 로 재현"),
 (12, u"ehp*1000 스케일로 재현"),
 (15, u"t_te/t_ta/t_soak 의 INF 센티널을 2305843009213693951 로 재현(분모 0·소커 없음 경로 포함)"),
 (16, u"dt = max(1, min(horizon-t, min(t_soak, min(t_ta, t_te)))) 로 재현"),
 (17, u"dir=-1 케이스(D-1 외 무작위 다수) · 516 net>-1 · ceil 나눗셈 -1 로 재현"),
 (18, u"Commit 태그 0 · rescue None 0 · first_focus None 0 재현(sret 0x38/0x20/0x0)"),
 (19, u"Disengage 태그 2 재현(dir 3종 전부에서 관측)"),
 (20, u"Hold 태그 3 재현"),
]:
    p.ev("/specs[207]/consts[%d]" % j, evidence=ORC + u" — " + why, to=2, frm=(3 if j in (1, 18) else 4), found_by="new")
# 207 knobs
for j, why in [
 (0, u"horizon 6초"), (1, u"999 경계(J999/J1000)"), (4, u"our_unit 밴드(dir 0 대칭)"), (5, u"dir=1 net>=0 Commit · dir=-1 net<=0 Disengage"),
 (6, u"사망 가치 = dps 합"), (7, u"min_by_key(hp) 동률 앞 인덱스"), (8, u"소커 재선정 max_by_key(stat_cached.hp) 동률 뒤 인덱스 — 재선정 발생 케이스(R1 등) 포함"),
 (9, u"dt 하한 1"), (10, u"종료 조건(t>=horizon · 양측 dps 0 · 생존 적 없음)"),
]:
    p.ev("/specs[207]/knobs[%d]" % j, evidence=ORC + u" — " + why, to=2, found_by="new")
# 207 mem(상한 3): 읽기 6~13 · 쓰기 16~29
for j in list(range(6, 14)) + list(range(16, 30)):
    p.ev("/specs[207]/mem[%d]" % j, evidence=ORC + u" — 이 오프셋을 그대로 읽/쓰는 재구현이 sret live 바이트(0x0/0x8/0x10/0x18/0x20/0x30/0x38/0x39) 비트 일치", to=3, found_by="new")
# 208
for path, why in [
 ("/specs[208]/consts[0]", u"delay = rnd.gen_range(input_delay_min..=input_delay_max)/100 예측이 12/12 호출에서 next_input_tick 실측과 일치(700..=1200 → 7~12틱)"),
 ("/specs[208]/consts[4]", u"early=1(next_input_tick>tick) 케이스: sret +0 = -1(i64) · self diff 0 · rnd 미소비"),
 ("/specs[208]/knobs[0]", u"위 delay/100 실측"),
]:
    p.ev(path, evidence=ORC8 + u" — " + why, to=2, frm=(5 if path.endswith("consts[0]") else 4), found_by="new")
for j, why in [
 (0, u"L844 게이트: next_input_tick 을 tick+100 으로 쓰면 조기반환(self diff 0)"), (22, u"조기반환 sret +40 = ctx.pool 포인터 실측 일치"),
 (27, u"player+0x180 AthleteParameter 로 input_delay_min/max 호출값(700/1200)이 실측 delay 와 정합"),
 (81, u"next_input_tick = tick + gen_range(min..=max)/100 실측 12/12"), (90, u"input_chances +1 매 호출 실측"),
 (99, u"freeze_since=tick 리셋 경로(freeze_since==0 첫 호출 · input Some 후속 호출) 실측"), (100, u"freeze_anchor.0 = champ.x 실측"), (101, u"freeze_anchor.1 = champ.y 실측"),
 (114, u"sret +0..+32 = Option<Input>(Move 태그 0 · x/y) 실측"), (115, u"sret +32..+64 = bumpalo Vec(ptr 8 · bump=pool · cap 0 · len 0) 실측"),
]:
    p.ev("/specs[208]/mem[%d]" % j, evidence=ORC8 + u" — " + why, to=3, found_by="new")

p.brief_error(u"§4 G16 [207] 은 명세 오류가 아니라 mkspec3 L565 의 결함이다: v3 `sig.params[].role` 에 v2 `note` 만 싣고 v2 `role`(레지스터 %k·define 속성이 이미 정확히 적혀 있음)을 버린다. 그래서 G16 P1 이 「승격 미기재」로 오탐한다. 정정 경로도 꼬인다 — `/sig/params[j]/role` 은 v2 row 에 role 키가 있어 v2 role 을 고치고(v3 무영향), v3 role 을 고치려면 `/note` 로 써야 하며 mkpatch.locate 가 v3 에서 못 찾아 force 가 필요하다.")
p.brief_error(u"§4 G20 R1 [208] 0x2860 의 이름 충돌은 [212] 쪽 오기다: tcxdict AgentVerHamster --deep 기준 0x2860 = small_action@AroundBush.0.change_tick 이고 AroundBush.target_x 는 0x2870, target_y 는 0x2878 — [208] mem[41]/[42] 가 맞고 [212] 의 「AroundBush target_x @0x2860」 이 틀렸다(배치 D 담당 밖이라 patch 미제출).")
p.brief_error(u"지시문 §1 의 「get_input … gen_range 1회」는 이 함수 직접 사이트 기준으로만 참이다 — 콜리(update_state·SmallActionPlay::get_input)가 추가로 소비해 rnd 상태는 대개 1회 이상 전진한다(실측: RunAway 경로에서만 정확히 1회).")
p.brief_error(u"지시문 §1 의 resolve_fight_uncached 「exe 15」·호출자 없음 표기와 달리 IR 호출자는 resolve_fight_full 2곳(m10.ll:39986 캐시 우회 len>8 · 40290 캐시 미스)이고 resolve_fight(hidden 심볼)가 그 래퍼라 오라클 진입이 가능하다 — 「internal 이라 오라클 불가」로 읽히지 않게 도시에에 진입 경로를 적어 달라.")
p.save()
print("errors", len(p.errors), "ev_up", len(p.ev_up) if hasattr(p, 'ev_up') else '?', "warn", p.warn)
