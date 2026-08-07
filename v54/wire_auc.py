# -*- coding: utf-8 -*-
"""경매 강제귀환 12노브 배선 + 경매 프로브를 SPDISP_PROBE 밖 자체 게이트로 분리."""
import io
SRC = r'C:\tfm2mods\tfm2_ai_adjust\src'

# ── ① detour.rs: apply_auc_imm 추가 ────────────────────────────────
p = SRC + r'\detour.rs'
t = io.open(p, encoding='utf-8').read()
assert 'fn apply_auc_imm' not in t

FN = r'''
/// ★[0.5.4 신설] 경매(전술 입찰) 중 **강제 귀환** 오버라이드 — 12노브.
///   0.5.4에서 새로 생긴 판단으로, 경매가 진행되는 동안 "지금 도망칠 피해를 견딜 수 없다"고
///   보이면 다른 모든 플랜을 제치고(점수 99999) 기지 코너로 물러나게 만든다.
///   ⚠전체가 `team_plan.version >= 2` 게이트(`auc_flee_version_gate`) 아래에 있다.
///     런타임 version 값은 **아직 미확정**(정적 호출 0건) — 게이트를 0으로 낮추면 항상 켤 수 있다.
///   전 키 기본 -1 = 원본값 그대로 = 무변화. 검증 = v54\aucknobs.py (12/12 exe 대조 통과).
unsafe fn apply_auc_imm() {
    let gate  = tune("auc_flee_version_gate", -1); // AI 사양 버전 게이트 (원본 1 → version>=2 에서 발동)
    let undy  = tune("auc_flee_undying_gate", -1); // 불사 특례 판정값 (원본 0) — 발동여부엔 무영향
    let hpf   = tune("auc_flee_hp_field", -1);     // 피해와 비교할 대상 (원본 0x658=현재HP, 0x610=최대HP)
    let nmask = tune("auc_flee_nexus_mask", -1);   // "넥서스 피격 중이면 취소" 비트마스크 (원본 0x100)
    let gfar  = tune("auc_flee_goal_far", -1);     // 도망 목적지 먼쪽 축 (원본 928000)
    let gna   = tune("auc_flee_goal_near_a", -1);  // 〃 가까운쪽 축 (원본 32000)
    let gnb   = tune("auc_flee_goal_near_b", -1);  // 〃 반대 사이드 사본 (원본 32000) — a와 같이 바꿀 것
    let dly   = tune("auc_flee_end_delay", -1);    // 도착 후 붙잡는 틱 (원본 5)
    let pf    = tune("auc_flee_pathfinder", -1);   // 경로탐색 사용 (원본 2=None) ⚠2 외 금지
    let wsk   = tune("auc_flee_with_skill", -1);   // 도망 중 허용 행동 비트 (원본 1=스킬만)
    let sc    = tune("auc_flee_score", -1);        // 이 플랜의 점수 (원본 99999 = 사실상 무조건 1위)
    let tag   = tune("auc_flee_action_tag", -1);   // 실제 행동 태그 (원본 3=RunAway) ⚠3 외 권장 안 함

    let mut sig = 0u64;
    for v in [gate, undy, hpf, nmask, gfar, gna, gnb, dly, pf, wsk, sc, tag] {
        sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64);
    }
    if sig == AUCIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let mut ok = 0u32; let mut tot = 0u32;
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }

    // ⚠`cmp rax, imm8`(부호확장) — 0~127 범위. 0 으로 낮추면 version 과 무관하게 항상 켜진다.
    p!(base + 0xead271, &[0x48,0x83,0xf8], 3, 1, b1(gate, 1));
    p!(base + 0xead285, &[0x41,0x80,0xb9,0x70,0x04,0x00,0x00], 7, 1, b1(undy, 0));
    // disp32 교체 — 값이 아니라 **비교할 필드**를 바꾼다(0x658 현재HP / 0x610 최대HP).
    p!(base + 0xead5e1, &[0x48,0x3b,0x81], 3, 4, b4(hpf, 0x658));
    p!(base + 0xead68d, &[0xa9], 1, 4, b4(nmask, 0x100));
    p!(base + 0xead6c8, &[0xb9], 1, 4, b4(gfar, 928_000));
    p!(base + 0xead6cd, &[0xba], 1, 4, b4(gna, 32_000));
    p!(base + 0xead6d6, &[0x41,0xb8], 2, 4, b4(gnb, 32_000));
    p!(base + 0xead6ff, &[0x49,0xc7,0x84,0x24,0x28,0x15,0x00,0x00], 8, 4, b4(dly, 5));
    p!(base + 0xead72f, &[0x41,0xc6,0x84,0x24,0x8d,0x15,0x00,0x00], 8, 1, b1(pf, 2));
    p!(base + 0xead738, &[0x41,0xc7,0x84,0x24,0x90,0x15,0x00,0x00], 8, 4, b4(wsk, 1));
    p!(base + 0xead759, &[0x49,0xc7,0x84,0x24,0x08,0x15,0x00,0x00], 8, 4, b4(sc, 99_999));
    p!(base + 0xead765, &[0x41,0xc6,0x84,0x24,0xc1,0x15,0x00,0x00], 8, 1, b1(tag, 3));

    AUCIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("auc_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} gate={} undying={} hp_field={} nexus_mask={} goal={}/{}/{} delay={} pf={} skill={} score={} tag={}\n\
             (-1=원본: 1/0/0x658/0x100/928000/32000/32000/5/2/1/99999/3) @base{:#x}\n",
            ok, tot, gate, undy, hpf, nmask, gfar, gna, gnb, dly, pf, wsk, sc, tag, base));
    }
}

'''
anc = 'unsafe fn apply_plan_imm() {'
i = t.index(anc)
t = t[:i] + FN.lstrip('\n') + t[i:]
io.open(p, 'w', encoding='utf-8', newline='\n').write(t)
print('detour.rs: apply_auc_imm 추가 (12노브)')

# ── ② tfm2_ai_adjust.rs: SIG static + 호출 + 프로브 게이트 분리 ─────
p = SRC + r'\tfm2_ai_adjust.rs'
t = io.open(p, encoding='utf-8').read()

t = t.replace('static ANIMM_SIG: AtomicU64',
              'static AUCIMM_SIG: AtomicU64 = AtomicU64::new(u64::MAX);\nstatic ANIMM_SIG: AtomicU64', 1)

anc = '        apply_an_imm();       // ★[0.5.4 신설] 판단14 넥서스공격'
i = t.index(anc)
t = (t[:i] + '        apply_auc_imm();      '
     '// ★[0.5.4 신설] 경매 중 강제귀환 12노브 (전 키 기본 -1=무변화)\n' + t[i:])

# 프로브를 SPDISP_PROBE 밖으로
OLD = ('            if SPDISP_PROBE {\n'
       '                // ★[0.5.4 프로브] 경매 진입 래퍼 — version 관측 전용(passthrough).\n'
       '                if let Ok(o) = install_wrap(RVA_AUCTION, 12, auction_probe_capture as *const () as usize) {\n'
       '                    ORIG_AUCTION.store(o, Ordering::Relaxed);\n'
       '                }\n')
NEW = ('            // ★[0.5.4 프로브] 경매 진입 래퍼 — `TeamPlan.version` 관측 전용(passthrough).\n'
       '            //   위 SPDISP_PROBE 블록 **밖**에 둔다: 07-31 크래시는 `d98740` 한정이고,\n'
       '            //   경매(`eacf10`)는 안전 실증된 disc18(`da1850`)과 측정 가능한 전 항목이 동일하다 —\n'
       '            //   선두 12B 바이트 완전동일(push8) · 12인자 extern "C" 동형 · 호출부 1곳 ·\n'
       '            //   **테일콜 진입 0 · 선두 12B 내부 진입 0**(v54\jmpin2.py 전역 스캔).\n'
       '            //   ⚠크래시가 나면 이 상수 하나만 false 로 되돌리면 된다.\n'
       '            if AUC_PROBE {\n'
       '                if let Ok(o) = install_wrap(RVA_AUCTION, 12, auction_probe_capture as *const () as usize) {\n'
       '                    ORIG_AUCTION.store(o, Ordering::Relaxed);\n'
       '                }\n'
       '            }\n'
       '            if SPDISP_PROBE {\n')
assert OLD in t
t = t.replace(OLD, NEW, 1)
t = t.replace('const SPDISP_PROBE: bool = false;',
              'const SPDISP_PROBE: bool = false;\n'
              '/// ★[0.5.4] 경매 진입 passthrough 프로브(`TeamPlan.version` 관측). 크래시 시 여기만 false.\n'
              'const AUC_PROBE: bool = true;', 1)

io.open(p, 'w', encoding='utf-8', newline='\n').write(t)
print('tfm2_ai_adjust.rs: SIG·호출·AUC_PROBE 게이트 분리 완료')
