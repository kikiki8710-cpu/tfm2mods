# -*- coding: utf-8 -*-
"""모드 수정 ①② — CLASS_ANY 정밀화 + 효과 없는 _class_ 무시·로그.

문제(08-06 재생 멈춤, 이분탐색 7판으로 확정):
  `CLASS_ANY = 키에 "_class_" 문자열이 있으면 참` 이라 **아무 효과도 없는 클래스 키 하나**가
  skip_untuned 최적화를 통째로 껐다. 손대지 않은 판단까지 전부 Rust 재구현으로 흘러 멈춘다.

수정:
  ① base 노브가 CLASS_CAPABLE(판단 본문에서 읽히는 115개)에 없으면 = 원래 안 먹던 값 →
     **skip 게이트에 영향을 주지 않는다.** class_override.txt 에 "무시됨" 으로 남긴다.
  ② 효과가 있는 오버라이드는 g() 가 "그 노브 튜닝됨" 으로 보게 해서 **그 판단만** 재구현을 유지한다.
     ⚠어느 판단 그룹에도 속하지 않는 base 는 어느 판단이 읽는지 알 수 없으므로 **전체 skip 해제로 폴백**
       (구 동작)하고 그 사실을 로그에 남긴다 — 조용히 무시하면 값이 안 먹는 잠복버그가 된다."""
import sys, io, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/tfm2_ai_adjust/src/tfm2_ai_adjust.rs'
t = io.open(P, encoding='utf-8').read()

# ── 0) skip 판정 그룹에 등장하는 키 전부 수집 → SKIP_GROUP_KEYS 생성 ──
grp = set()
for m in re.finditer(r'g\(&\[(.*?)\]\)', t, re.S):
    grp.update(re.findall(r'"([a-zA-Z0-9_]+)"', m.group(1)))
keys = sorted(grp)
rs = ['// ★[08-07 자동생성 — v54/fix_class_gate.py] skip_untuned 판정 그룹(g(&[...]))에 등장하는 노브 전부.',
      '//   클래스 오버라이드가 이 목록 안의 노브에 걸리면 "그 판단만" 재구현을 유지하면 된다.',
      '//   목록 밖이면 어느 판단이 그 값을 읽는지 알 수 없으므로 보수적으로 전체 skip 을 해제한다.',
      'pub static SKIP_GROUP_KEYS: [&str; %d] = [' % len(keys)]
for i in range(0, len(keys), 6):
    rs.append('    ' + ' '.join('"%s",' % x for x in keys[i:i + 6]))
rs.append('];')
io.open('C:/tfm2mods/tfm2_ai_adjust/src/skip_groups.rs', 'w', encoding='utf-8', newline='\n').write('\n'.join(rs) + '\n')
print('  [ok] skip_groups.rs 생성 — 그룹 키 %d개' % len(keys))

n = 0
# ── 1) 모듈 포함 ──────────────────────────────────────────
A0 = 'static CLASS_ANY: AtomicBool = AtomicBool::new(false);'
if '#[path = "class_capable.rs"]' not in t:
    t = t.replace(A0,
        '#[path = "class_capable.rs"] mod class_capable;\n'
        '#[path = "skip_groups.rs"] mod skip_groups;\n'
        'use class_capable::CLASS_CAPABLE;\n'
        'use skip_groups::SKIP_GROUP_KEYS;\n' + A0, 1)
    n += 1
    print('  [ok] 모듈 포함')

# ── 2) CLASS_ANY 정밀화 + 진단 로그 ────────────────────────
OLD1 = ('    CLASS_ANY.store(new_tune.keys().any(|k| k.contains("_class_")), Ordering::Relaxed);'
        '   // ★클래스 오버라이드 존재 → 맵빌드 + skip 우회')
NEW1 = '''    // ★[08-07] 클래스 오버라이드 정밀화. 구현은 "_class_ 문자열이 있으면 CLASS_ANY=참" 이었고,
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
    // 그룹 목록 밖의 유효 오버라이드 = 어느 판단이 읽는지 미상 → 보수적으로 전체 skip 해제
    let ov_unknown: Vec<String> = ov_live.iter()
        .filter(|b| !SKIP_GROUP_KEYS.contains(&b.as_str())).cloned().collect();
    CLASS_ANY.store(!ov_live.is_empty(), Ordering::Relaxed);   // 맵빌드는 유효 오버라이드가 있을 때만
    {   // 진단 로그 — 무엇이 먹고 무엇이 무시됐는지 남긴다(조용한 무시 금지)
        let mut s = String::from("=== 클래스별 값(_class_) 적용 결과 ===\\n");
        s.push_str(&format!("적용됨({}) : {}\\n", ov_live.len(), ov_live.join(", ")));
        s.push_str(&format!("무시됨({}) : {}\\n", ov_dead.len(), ov_dead.join(", ")));
        s.push_str("  ↑ 무시 사유 = 바이트패치 전용 노브. exe 기계어 상수를 고치는 방식이라 선수별로 다를 수 없다.\\n");
        if !ov_unknown.is_empty() {
            s.push_str(&format!("판단 미상({}) : {}\\n  ↑ 어느 판단이 읽는지 몰라 최적화(skip_untuned)를 전부 해제했다(느려짐).\\n",
                ov_unknown.len(), ov_unknown.join(", ")));
        }
        if let Some(p) = pth("class_override.txt") { let _ = fs::write(&p, &s); }
    }'''
assert OLD1 in t, 'CLASS_ANY 원문 불일치'
t = t.replace(OLD1, NEW1, 1); n += 1
print('  [ok] CLASS_ANY 정밀화 + class_override.txt 로그')

# ── 3) skip 게이트 ─────────────────────────────────────────
OLD2 = ('    if SKIP_UNTUNED.load(Ordering::Relaxed) && !CLASS_ANY.load(Ordering::Relaxed) '
        '&& !CHAMP_ANY.load(Ordering::Relaxed) {')
NEW2 = ('    // ★[08-07] CLASS_ANY 로 전체 skip 을 끄지 않는다 — 아래 g() 가 유효 오버라이드를 "튜닝됨" 으로\n'
        '    //   취급해 **그 판단만** 재구현을 유지한다. 판단 미상 오버라이드가 있을 때만 구 동작(전체 해제)로 폴백.\n'
        '    if SKIP_UNTUNED.load(Ordering::Relaxed) && ov_unknown.is_empty() && !CHAMP_ANY.load(Ordering::Relaxed) {')
assert OLD2 in t, 'skip 게이트 원문 불일치'
t = t.replace(OLD2, NEW2, 1); n += 1
print('  [ok] skip 게이트 정밀화')

# ── 4) g() 가 클래스 오버라이드도 튜닝으로 ──────────────────
OLD3 = ('            let g = |keys: &[&str]| keys.iter().any(|&k| match base.get(k) '
        '{ Some(&b) => tune(k, b) != b, None => false });')
NEW3 = ('            // ★[08-07] 클래스 오버라이드가 걸린 노브도 "튜닝됨" 으로 본다 → 그 판단만 재구현 유지.\n'
        '            let g = |keys: &[&str]| keys.iter().any(|&k|\n'
        '                ov_live.iter().any(|b| b == k)\n'
        '                || match base.get(k) { Some(&b) => tune(k, b) != b, None => false });')
assert OLD3 in t, 'g() 원문 불일치'
t = t.replace(OLD3, NEW3, 1); n += 1
print('  [ok] g() 에 클래스 오버라이드 반영')

io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('\n적용 %d건' % n)
