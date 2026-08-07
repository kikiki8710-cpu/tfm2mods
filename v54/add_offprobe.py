# -*- coding: utf-8 -*-
"""athlete_id 오프셋을 **런타임에 직접 판별**하는 진단을 team_gate 앞에 넣는다.
   기존 champ_verify 카운터는 선수별 오버라이드가 있을 때만 돌아서(=idx<0 조기반환) 오프셋 검증에 못 쓴다.
   여기서는 오버라이드 유무와 무관하게, 후보 두 오프셋(0x800/0x810)에서 읽은 값이
   ALL_ATHLETES(전체 athlete_id 집합)에 들어맞는지를 세어 **어느 쪽이 진짜 athlete_id 인지** 가린다."""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/tfm2_ai_adjust/src/tfm2_ai_adjust.rs'
t = io.open(P, encoding='utf-8').read()
n = 0


def sub1(old, new, tag):
    global t, n
    if old not in t:
        print('  [건너뜀] %s' % tag); return
    t = t.replace(old, new, 1); n += 1
    print('  [ok] %s' % tag)


# ── 카운터 ─────────────────────────────────────────────────
sub1('static GATE_PASS: AtomicU64 = AtomicU64::new(0);',
'''// ★[08-06 오프셋 판별] 후보 두 자리에서 읽은 값이 실제 athlete_id 집합에 맞는 횟수.
//   0x800 쪽이 크게 이기면 정정이 옳고, 0x810 쪽이 이기면 되돌려야 한다.
static OFF_HIT_800: AtomicU64 = AtomicU64::new(0);
static OFF_HIT_810: AtomicU64 = AtomicU64::new(0);
static OFF_SEEN: AtomicU64 = AtomicU64::new(0);
static OFF_SAMPLE: Mutex<Vec<(u64, u64)>> = Mutex::new(Vec::new());   // (at0x800, at0x810) 표본
static GATE_PASS: AtomicU64 = AtomicU64::new(0);''', '카운터 추가')

# ── team_gate 진입부에 프로브 (조기반환보다 앞) ──────────────
sub1('''#[inline] unsafe fn team_gate(idx: i16, p5ath: usize) -> i16 {
    if idx < 0 || !SELF_TEAM_ONLY.load(Ordering::Relaxed) { return idx; }''',
'''#[inline] unsafe fn team_gate(idx: i16, p5ath: usize) -> i16 {
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
    if idx < 0 || !SELF_TEAM_ONLY.load(Ordering::Relaxed) { return idx; }''', 'team_gate 프로브')

# ── champ_verify.txt 출력에 결과 추가 ──────────────────────
sub1('''[우리팀 게이트 (self_team_only=1)]''',
'''[★athlete_id 오프셋 판별 (08-06)]
  관측 표본        = {off_seen}
  +0x800 이 실제 id = {off800}
  +0x810 이 실제 id = {off810}
  표본(0x800 / 0x810) = {off_sample}
  ※ 0x800 쪽이 크게 많으면 08-06 정정(0x810→0x800)이 옳다. 0x810 쪽이면 되돌려야 한다.
  ※ 둘 다 0이면 로스터(ALL_ATHLETES) 미확보 — 관리화면을 한 번 들른 뒤 다시 볼 것.

[우리팀 게이트 (self_team_only=1)]''', '출력 템플릿')

io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('\n적용 %d건' % n)
