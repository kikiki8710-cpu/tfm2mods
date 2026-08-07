# -*- coding: utf-8 -*-
"""경매(eacf10) 진입부 프로브 훅 — TeamPlan.version 을 3번째 인자에서 직접 읽는다.

앞선 2번의 시도가 실패한 이유:
  ①`subplan_dispatch_capture` 의 p3 → 그 디스패처가 **한 번도 안 불렸다**(넥서스 국면 전용)
  ②`champ+0xF8+0x2888` / `*(champ+0xF8)+0x2888` → **둘 다 오답**.
    값이 0~7에 골고루 퍼지고 범위밖도 240만 = 그냥 임의 메모리였다.
    (version 이라면 한 값에만 몰려야 한다.)
⟹ 추측을 그만두고 **값이 확실히 있는 자리**(경매의 3번째 인자)에서 읽는다.

안전성: `eacf10` 선두 12B = `55 41 57 41 56 41 55 41 54 56 57 53`(push8) 로
  **이미 안전하게 후킹 중인 disc18(`da1850`)과 바이트 완전 동일**하고,
  orig_len 12 가 명령경계에 정확히 떨어진다(실측). rip-rel/분기 없음.
  본문은 읽기만 하고 원본을 그대로 호출한다(passthrough) = 기능 무영향.
"""
import io

SRC = r'C:\tfm2mods\tfm2_ai_adjust\src'

# ── ① rva_054.rs 에 상수 추가 ──────────────────────────────
p = SRC + r'\rva_054.rs'
t = io.open(p, encoding='utf-8').read()
if 'RVA_AUCTION' not in t:
    t += ('\n// ★[0.5.4 프로브] 경매(전술 입찰) 진입 — `TeamPlan.version` 이 **3번째 인자(r8)** 로 들어온다.\n'
          '//   version 은 0.5.4 신규 필드이고 `>=2` 게이트가 exe 전역 8곳(경매 강제귀환·점수식 넥서스 게이트 등)을\n'
          '//   여닫는데 **정적으로는 값을 못 밝혔다**(팩토리 e8c020 이 정적 호출 0건).\n'
          '//   선두 12B = disc18(da1850)과 바이트 완전 동일한 push8, orig_len 12 경계정확, rip-rel/분기 無.\n'
          '//   ⚠passthrough 프로브 전용 — 원본을 항상 호출한다(기능 무영향).\n'
          'const RVA_AUCTION: usize = 0xeacf10;\n')
    io.open(p, 'w', encoding='utf-8', newline='\n').write(t)
    print('rva_054.rs: RVA_AUCTION 추가')

# ── ② 실패한 v2 프로브 제거 + 경매 캡처 함수 추가 ───────────
p = SRC + r'\tfm2_ai_adjust.rs'
t = io.open(p, encoding='utf-8').read()

s = t.index('    // ★[0.5.4 프로브 v2] TeamPlan.version 을 **새 훅 없이** 잡는다.')
e = t.index('    if JUDGE_DUMP.load(Ordering::Relaxed) != 0 {', s)
t = t[:s] + t[e:]
print('실패한 v2 프로브 제거(champ+0xF8 경로 = 오답 확인됨)')

FN = '''
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

'''
anchor = 'unsafe extern "C" fn mp_capture('
i = t.index(anchor)
t = t[:i] + FN.lstrip('\n') + t[i:]

STAT = ['/// ★0.5.4 경매 진입 시 관측한 TeamPlan.version 분포(0~7 개별, 그 밖은 BIG).',
        'static AUC_VER_HIST: [AtomicU64; 8] = [',
        '    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),',
        '    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];',
        'static AUC_VER_BIG: AtomicU64 = AtomicU64::new(0);',
        'static ORIG_AUCTION: AtomicUsize = AtomicUsize::new(0);',
        'static ANIMM_SIG: AtomicU64']
t = t.replace('static ANIMM_SIG: AtomicU64', '\n'.join(STAT), 1)

# ── ③ 출력부 교체 ─────────────────────────────────────────
os_ = t.index('            if let Some(p) = pth("teamplan_version.txt") {')
oe = t.index('            if let Some(p) = pth("itemnet_guard.txt") {', os_)
NEW = [
    '            if let Some(p) = pth("teamplan_version.txt") {',
    '                let mut v: Vec<String> = Vec::new();',
    '                for i in 0..AUC_VER_HIST.len() {',
    '                    let c = AUC_VER_HIST[i].load(Ordering::Relaxed);',
    '                    if c != 0 { v.push(format!("{}:{}", i, c)); }',
    '                }',
    '                let b = AUC_VER_BIG.load(Ordering::Relaxed);',
    '                if b != 0 { v.push(format!("(8이상):{}", b)); }',
    '                let _ = fs::write(p, format!(',
    '                    "TeamPlan.version = {}   (경매 진입 3번째 인자 실측)\\n\\',
    '                     훅 설치 = {}\\n\\',
    '                     ※ 2 이상이면 0.5.4 신규 판단(경매 강제귀환·점수식 넥서스 게이트)이 켜져 있다.\\n",',
    '                    if v.is_empty() { "(관측 0 — 훅 미설치 또는 경매 미도달)".to_string() } else { v.join(" ") },',
    '                    if ORIG_AUCTION.load(Ordering::Relaxed) != 0 { "OK" } else { "실패" }));',
    '            }',
    '',
]
t = t[:os_] + '\n'.join(NEW) + t[oe:]

# ── ④ 설치 ────────────────────────────────────────────────
anc = '                match install_wrap(RVA_SUBPLAN_DISPATCH, 12, subplan_dispatch_capture as *const () as usize) {'
j = t.index(anc)
k = t.index('\n', t.index('}', t.index('{', j)))   # 대충 블록 끝 — 앞에 삽입하는 게 안전
INS = ('                // ★[0.5.4 프로브] 경매 진입 래퍼 — version 관측 전용(passthrough).\n'
       '                if let Ok(o) = install_wrap(RVA_AUCTION, 12, auction_probe_capture as *const () as usize) {\n'
       '                    ORIG_AUCTION.store(o, Ordering::Relaxed);\n'
       '                }\n')
t = t[:j] + INS + t[j:]

io.open(p, 'w', encoding='utf-8', newline='\n').write(t)
print('경매 프로브 훅 삽입 완료')
