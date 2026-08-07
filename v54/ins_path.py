# -*- coding: utf-8 -*-
import io
SRC = r'C:\tfm2mods\tfm2_ai_adjust\src'
tables = io.open('path_tables.rs', encoding='utf-8').read()

p = SRC + r'\detour.rs'
t = io.open(p, encoding='utf-8').read()
assert 'fn apply_path_imm' not in t

FN = r'''
// ★[0.5.4 신설] 경로/거리 시스템 노브 — 0.5.4 게임-ai 증가분(+464KB) 중 ~447KB가 이 신규 서브시스템이다.
//   ⚠⚠**표+루프로 유지할 것.** 208개 사이트를 p! 인라인으로 펼치면 opt-level=1 에서 호출부마다 스택 슬롯이
//     생겨 rayon 워커 스택이 터진다(2026-08-05 STATUS_STACK_OVERFLOW 실사고 = 크래시2).
//   전 사이트 exe 대조 완료(prefix/imm 실제위치/원본값/명령경계) = v54\wire_path.py, 208/208 통과.
''' + tables + r'''
/// ★[0.5.4 신설] 경로탐색 비용·위험 회피 노브. 전 키 기본 -1 = 원본값 = 무변화.
///   ⚠**A\* 허용성(admissibility)**: 휴리스틱이 `2^greedy × free_dist` 라, 간선비용을 휴리스틱보다
///     **낮추면** 최단경로 보장이 깨진다(크래시는 아니고 경로가 나빠질 뿐).
///     안전선 = 직교 >= 640, 대각 >= 896. **올리는 방향은 언제나 안전**하다.
unsafe fn apply_path_imm() {
    let orth  = tune("path_orth_cost", -1);     // 직교 1칸 비용 (원본 640) — 위험 페널티와의 상대 크기를 정한다
    let diag  = tune("path_diag_cost", -1);     // 대각 1칸 비용 (원본 896) — 올리면 계단식(맨해튼) 이동
    let dang  = tune("path_danger_cost", -1);   // 미니언 웨이브가 죽는 자리 회피 비용 (원본 1281 = 2칸우회×640+1)
    let greedy= tune("path_greedy", -1);        // A* 탐욕도 shl 자릿수 (원본 7 = ×128). 0=완전탐색(CPU↑)
    let tfloor= tune("path_threat_floor", -1);  // 위험지대 최소 우회 칸 (원본 2)
    let tcap  = tune("path_threat_cap", -1);    // 위험지대 최대 우회 칸 (원본 60)
    let tscale= tune("path_threat_scale", -1);  // 체력대비 피해 민감도 (원본 30)
    let tdef  = tune("path_threat_default", -1);// 위험원 못 찾았을 때 기본 우회 칸 (원본 2)
    let wave  = tune("path_wave_risk_ret", -1); // 위협원 안에 있을 때 위험등급 (원본 3)

    let mut sig = 0u64;
    for v in [orth, diag, dang, greedy, tfloor, tcap, tscale, tdef, wave] {
        sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64);
    }
    if sig == PATHIMM_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let b1 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0).min(0x7f)) as u64 };
    let b4 = |v: i64, orig: u64| if v < 0 { orig } else { (v.max(0) as u64) & 0xffff_ffff };
    let mut ok = 0u32; let mut tot = 0u32;

    // ── 다중 사이트 4종: 표 1개당 루프 1개 (인라인 전개 금지) ──
    let v = b4(orth, 640);
    for &(a, pre, off) in PATH_STEP640.iter()  { tot += 1; ok += patch_imm_bytes(base + a, pre, off, 4, v) as u32; }
    let v = b4(diag, 896);
    for &(a, pre, off) in PATH_STEP896.iter()  { tot += 1; ok += patch_imm_bytes(base + a, pre, off, 4, v) as u32; }
    let v = b4(dang, 1281);
    for &(a, pre, off) in PATH_RISK1281.iter() { tot += 1; ok += patch_imm_bytes(base + a, pre, off, 4, v) as u32; }
    // ⚠`shl r32, imm8` 의 자릿수 — 9를 넘기면 휴리스틱이 폭주해 경로가 직선으로 뭉개진다. 0~9 로 조인다.
    let v = if greedy < 0 { 7 } else { greedy.max(0).min(9) as u64 };
    for &(a, pre, off) in PATH_HEUR.iter()     { tot += 1; ok += patch_imm_bytes(base + a, pre, off, 1, v) as u32; }

    // ── 위협원 산식: clamp(floor + scale*(1칸에 받을피해)/HP, floor, cap) 칸 ──
    macro_rules! p { ($a:expr, $pre:expr, $off:expr, $w:expr, $v:expr) => {{
        tot += 1; ok += patch_imm_bytes($a, $pre, $off, $w, $v) as u32;
    }}; }
    p!(base + 0xdb07cb, &[0x83,0xc1], 2, 1, b1(tfloor, 2));
    // ⚠상한은 **2곳 동시**(cmp 는 imm8, 뒤이은 mov 는 imm32) — 한쪽만 바꾸면 반쪽 노브가 된다.
    p!(base + 0xdb07ce, &[0x83,0xf9], 2, 1, b1(tcap, 60));
    p!(base + 0xdb07d1, &[0xb8], 1, 4, b4(tcap, 60));
    p!(base + 0xdb077e, &[0xb9], 1, 4, b4(tscale, 30));
    p!(base + 0xdb07b2, &[0xb9], 1, 4, b4(tscale, 30));
    p!(base + 0xdb05fc, &[0xb8], 1, 4, b4(tdef, 2));
    p!(base + 0xdb0745, &[0xb8], 1, 4, b4(tdef, 2));
    p!(base + 0xd3101c, &[0xb8], 1, 4, b4(wave, 3));

    PATHIMM_SIG.store(sig, Ordering::Relaxed);
    if let Some(pp) = pth("path_imm.txt") {
        let _ = fs::write(pp, format!(
            "applied={}/{} orth={} diag={} danger={} greedy={} | threat[floor={} cap={} scale={} default={}] wave={}\n\
             (-1=원본: 640/896/1281/7 | 2/60/30/2 | 3) @base{:#x}\n",
            ok, tot, orth, diag, dang, greedy, tfloor, tcap, tscale, tdef, wave, base));
    }
}

'''
anc = 'unsafe fn apply_plan_imm() {'
i = t.index(anc)
t = t[:i] + FN.lstrip('\n') + t[i:]
io.open(p, 'w', encoding='utf-8', newline='\n').write(t)
print('detour.rs: apply_path_imm 추가 (208사이트, 표4+개별8)')

p = SRC + r'\tfm2_ai_adjust.rs'
t = io.open(p, encoding='utf-8').read()
t = t.replace('static AUCIMM_SIG: AtomicU64',
              'static PATHIMM_SIG: AtomicU64 = AtomicU64::new(u64::MAX);\nstatic AUCIMM_SIG: AtomicU64', 1)
anc = '        apply_auc_imm();      '
i = t.index(anc)
t = t[:i] + '        apply_path_imm();     // ★[0.5.4 신설] 경로/거리 시스템 208사이트 (전 키 기본 -1=무변화)\n' + t[i:]
io.open(p, 'w', encoding='utf-8', newline='\n').write(t)
print('tfm2_ai_adjust.rs: SIG + 호출 추가')
