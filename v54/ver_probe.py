# -*- coding: utf-8 -*-
"""TeamPlan.version 프로브 v2 삽입 — plan 디스패처(매 판단 발화)에서 2후보를 동시에 관측."""
import io

P = r'C:\tfm2mods\tfm2_ai_adjust\src\tfm2_ai_adjust.rs'
t = io.open(P, encoding='utf-8').read()

ANCHOR = ('    if JUDGE_DUMP.load(Ordering::Relaxed) != 0 { let _ = '
          'std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| '
          'judge_dump_capture(saved, entry_rsp))); }   // ★판단 풀덤프(관리팀 경기 1개)')

PROBE = [
    '    // ★[0.5.4 프로브 v2] TeamPlan.version 을 **새 훅 없이** 잡는다.',
    '    //   1차 시도(subplan_dispatch_capture 의 p3)는 실패했다 — 그 디스패처가 한 번도 안 불렸다',
    '    //   (`subplan_dispatch total=0`). 넥서스 국면 전용 경로라 일반 경기에선 안 탄다.',
    '    //   여기(plan 디스패처)는 **매 판단마다** 불리므로 확실히 표본이 잡힌다.',
    '    //   TeamPlan 은 champion 안에 인라인이고(경매가 `lea r12,[rcx+0xf8]`), version 은 그 0번 필드다.',
    '    //   ⚠경로가 틀리면 큰 쓰레기 값이 나온다 — 그것도 정보다(0~7 버킷 밖은 BIG 카운터).',
    '    //   ⚠rd_u64 는 VEH 경유라 잘못된 포인터여도 안전하다. 읽기만 하므로 게임 동작 무영향.',
    '    {',
    '        let r14 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;   // p5 = champion/athlete',
    '        if ptr_ok(r14) {',
    '            let v = rd_u64(r14 + 0xF8 + 0x2888).unwrap_or(u64::MAX) as usize;',
    '            if v < SP_VER_HIST.len() { SP_VER_HIST[v].fetch_add(1, Ordering::Relaxed); }',
    '            else { SP_VER_BIG.fetch_add(1, Ordering::Relaxed); }',
    '            let tp = rd_u64(r14 + 0xF8).unwrap_or(0) as usize;   // 후보B: 포인터였을 경우',
    '            if ptr_ok(tp) {',
    '                let v2 = rd_u64(tp + 0x2888).unwrap_or(u64::MAX) as usize;',
    '                if v2 < SP_VER2_HIST.len() { SP_VER2_HIST[v2].fetch_add(1, Ordering::Relaxed); }',
    '                else { SP_VER2_BIG.fetch_add(1, Ordering::Relaxed); }',
    '            }',
    '        }',
    '        MP_VER_SEEN.fetch_add(1, Ordering::Relaxed);',
    '    }',
]
assert ANCHOR in t
t = t.replace(ANCHOR, '\n'.join(PROBE) + '\n' + ANCHOR, 1)

STAT = ['static SP_VER2_HIST: [AtomicU64; 8] = [',
        '    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),',
        '    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];',
        'static SP_VER2_BIG: AtomicU64 = AtomicU64::new(0);',
        'static MP_VER_SEEN: AtomicU64 = AtomicU64::new(0);',
        'static ANIMM_SIG: AtomicU64']
assert 'static ANIMM_SIG: AtomicU64' in t
t = t.replace('static ANIMM_SIG: AtomicU64', '\n'.join(STAT), 1)

# 출력부: 기존 fs::write 블록을 통째로 교체
old_start = t.index('            if let Some(p) = pth("teamplan_version.txt") {')
old_end = t.index('            if let Some(p) = pth("itemnet_guard.txt") {', old_start)
NEW = [
    '            if let Some(p) = pth("teamplan_version.txt") {',
    '                let dump = |h: &[AtomicU64; 8], big: &AtomicU64| -> String {',
    '                    let mut v: Vec<String> = Vec::new();',
    '                    for i in 0..h.len() {',
    '                        let c = h[i].load(Ordering::Relaxed);',
    '                        if c != 0 { v.push(format!("{}:{}", i, c)); }',
    '                    }',
    '                    let b = big.load(Ordering::Relaxed);',
    '                    if b != 0 { v.push(format!("(범위밖):{}", b)); }',
    '                    if v.is_empty() { "(관측 0)".to_string() } else { v.join(" ") }',
    '                };',
    '                let _ = fs::write(p, format!(',
    '                    "후보A champ+0xF8+0x2888 (인라인 가정) = {}\\n\\',
    '                     후보B *(champ+0xF8)+0x2888 (포인터 가정) = {}\\n\\',
    '                     mp_capture 표본 = {}\\n\\',
    '                     subplan_dispatch total = {}  (0 = 그 경로 미발화)\\n\\',
    '                     ※ 0~7 중 한 값에 표본이 몰리면 그게 version. 전부 (범위밖)이면 두 경로 다 틀린 것.\\n",',
    '                    dump(&SP_VER_HIST, &SP_VER_BIG),',
    '                    dump(&SP_VER2_HIST, &SP_VER2_BIG),',
    '                    MP_VER_SEEN.load(Ordering::Relaxed),',
    '                    SP_TOTAL.load(Ordering::Relaxed)));',
    '            }',
    '',
]
t = t[:old_start] + '\n'.join(NEW) + t[old_end:]

io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('version 프로브 v2 삽입 (2후보 동시 관측)')
