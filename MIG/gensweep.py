#!/usr/bin/env python3
"""gensweep.py — 판정 계층이 이미 RVA 를 확정해 둔 함수들을 **전수** 링크사본 대조하는 코드를 생성한다.

왜: 지금까지는 seq_trace 가 가리키는 함수만 한둘씩 붙여 대조했다(=추측). 그러다
    `interaction_score` 가 주범이 아님이 드러나 추측이 틀렸음이 확인됐다.
    `gen_fns.rs` 에 RVA 가 확정된 함수가 50개 있으니 **전부 한 번에** 걸어 어디서 갈라지는지 본다.

방법: gen_fns.rs 에서 (name, rva, sym) 을 읽고, sym 이 실제 Rust 경로인 것만 골라
      IR 에서 mangled 심볼과 시그니처를 찾아 래퍼를 만든다.
      - 인자는 ptr / i64 / i32 / i8 만 지원(그 외는 건너뜀 — ABI 틀리면 게임이 즉사한다)
      - 반환은 i64 / void / i1 만
      - 320B 로 역참조되는 인자는 **StdRng** 로 보고 호출 전후로 떠서 되돌린다
      - 프롤로그는 exe 에서 capstone 으로 12바이트 이상 되는 **명령 경계**까지 읽는다

출력: `tfm2_ai_adjust\\src\\judge\\sweep.rs`
사용: python MIG\\gensweep.py
"""
import glob
import io
import re
import struct

import capstone

EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
BASE = 0x140000000
GEN = r'C:\tfm2mods\tfm2_ai_adjust\src\judge\gen_fns.rs'
OUT = r'C:\tfm2mods\tfm2_ai_adjust\src\judge\sweep.rs'
SKIP_RVA = {0xc87fe0, 0xc87850, 0x12857f0, 0x1323a00, 0x132b310, 0x12a5770}   # 접근자/공용헬퍼·census 밖


def sections(d):
    pe = struct.unpack_from('<I', d, 0x3c)[0]
    n = struct.unpack_from('<H', d, pe + 6)[0]
    opt = struct.unpack_from('<H', d, pe + 20)[0]
    return [struct.unpack_from('<IIII', d, pe + 24 + opt + i * 40 + 8) for i in range(n)]


def main():
    d = open(EXE, 'rb').read()
    secs = sections(d)

    def off(r):
        for vsz, va, rsz, ra in secs:
            if va <= r < va + max(vsz, rsz):
                return ra + r - va

    cs = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)

    # 1) gen_fns.rs
    specs = []
    for m in re.finditer(r'pub const (\w+): FnSpec = FnSpec \{([^}]*)\}', io.open(GEN, encoding='utf-8').read()):
        body = m.group(2)
        nm = re.search(r'name: "([^"]+)"', body)
        rv = re.search(r'rva: (0x[0-9a-f]+)', body)
        sy = re.search(r'sym: r"([^"]*)"', body)
        if not (nm and rv and sy):
            continue
        specs.append((m.group(1), nm.group(1), int(rv.group(1), 16), sy.group(1)))

    # 2) IR 심볼 표 — define 줄 전부
    defs = []
    for f in sorted(glob.glob(r'C:\tfm2mods\_gaibc\m*.ll')):
        t = io.open(f, encoding='utf-8', errors='ignore').read()
        for m in re.finditer(r'^define ([^@\n]*)@([A-Za-z0-9_.$]+)\(([^\n]*?)\) unnamed_addr', t, re.M):
            defs.append((m.group(2), m.group(1).strip(), m.group(3)))
    by_sym = {s: (r, a) for s, r, a in defs}

    # --- 소스 파일 경로 명세용: IR DISubprogram 구간 표 ---
    import json
    import os
    aimap = json.load(io.open(r'C:\tfm2mods\MIG\aimap.json', encoding='utf-8'))['info']
    spans = []          # (basename, lo, hi, linkageName)
    for f in sorted(glob.glob(r'C:\tfm2mods\_gaibc\m*.ll')):
        tt = io.open(f, encoding='utf-8', errors='ignore').read()
        files = {m.group(1): m.group(2) for m in re.finditer(r'^!(\d+) = !DIFile\(filename: "([^"]+)"', tt, re.M)}
        md = dict(re.findall(r'^!(\d+) = (.*)$', tt, re.M))
        subs = {}
        for m in re.finditer(r'^!(\d+) = (?:distinct )?!DISubprogram\(([^\n]*)', tt, re.M):
            b = m.group(2)
            lk = re.search(r'linkageName: "([^"]+)"', b)
            fi = re.search(r'file: !(\d+)', b)
            ln = re.search(r'line: (\d+)', b)
            if lk and fi and ln and fi.group(1) in files:
                subs[m.group(1)] = [lk.group(1), re.split(r'[\\/]', files[fi.group(1)])[-1], int(ln.group(1)), int(ln.group(1))]
        cache = {}

        def owner(nn, dep=0):
            if nn in cache:
                return cache[nn]
            if nn in subs:
                cache[nn] = nn
                return nn
            v = md.get(nn, '')
            sm = re.search(r'scope: !(\d+)', v)
            r2 = owner(sm.group(1), dep + 1) if (sm and dep < 24) else None
            cache[nn] = r2
            return r2
        for m in re.finditer(r'^!\d+ = !DILocation\(line: (\d+)(?:, column: \d+)?, scope: !(\d+)', tt, re.M):
            ow = owner(m.group(2))
            if ow:
                v = int(m.group(1))
                if v > subs[ow][3]:
                    subs[ow][3] = v
        for v in subs.values():
            spans.append(tuple(v))

    def find_by_file(rva):
        info = aimap.get(hex(rva))
        if not info or not info.get('lines'):
            return None
        base = re.split(r'[\\/]', info.get('mod', ''))[-1] + '.rs'
        lines = info['lines']
        cands = []
        for lk, bs, lo, hi in spans:
            if bs != base or lk not in by_sym:
                continue
            # 클로저·제네릭 인스턴스는 제외 — 진짜 함수만
            if lk.startswith('_RNC') or 'NC' in lk[:12] or 'closure' in lk:
                continue
            if all(lo <= x <= hi for x in lines):
                cands.append((lo, hi, lk))
        if not cands:
            return None
        top_lo = max(c[0] for c in cands)                  # 선언줄이 가장 아래 = 가장 안쪽
        cands = [c for c in cands if c[0] == top_lo]
        if len(cands) != 1:                                 # 애매하면 버린다(즉사 방지)
            return None
        lk = cands[0][2]
        return (lk, by_sym[lk][0], by_sym[lk][1])

    def find_sym(path):
        """`game_ai::buff_value::v55_mark_value` → mangled 심볼(길이접두 매칭)."""
        parts = [p for p in path.split('::') if p]
        if len(parts) < 2:
            return None
        needles = ['%d%s' % (len(p), p) for p in parts[1:]]
        best = None
        for s, r, a in defs:
            if all(nd in s for nd in needles) and 'internal' not in r:
                if best is None or len(s) < len(best[0]):
                    best = (s, r, a)
        return best

    def parse_args(argstr):
        """괄호 깊이 0 의 쉼표로만 자른다 — `range(i64 a, b)` 같은 속성 안 쉼표에 안 속는다."""
        parts, depth, cur = [], 0, ''
        for ch in argstr:
            if ch in '([<':
                depth += 1
            elif ch in ')]>':
                depth -= 1
            if ch == ',' and depth == 0:
                parts.append(cur)
                cur = ''
            else:
                cur += ch
        if cur.strip():
            parts.append(cur)
        out = []
        for i, p in enumerate(parts):
            mm = re.match(r'\s*(ptr|i64|i32|i16|i8|i1)\b', p)
            if not mm:
                return []                      # 모르는 타입 → 통째로 포기(즉사 방지)
            if '%' not in p:
                return []
            deref = re.search(r'dereferenceable\((\d+)\)', p)
            out.append((i, mm.group(1), int(deref.group(1)) if deref else 0))
        return out

    rows = []
    for const, name, rva, sym in specs:
        if rva in SKIP_RVA:
            continue
        # ⚠파일경로 역추적(find_by_file)은 클로저를 잡아 ABI 가 틀린다(base_score 를 3인자로 오인 — 실제 7인자).
        #   즉사 위험이 커서 Rust 경로 명세만 쓴다. 파일경로 명세는 손으로 심볼을 확정해야 한다.
        if not sym.startswith('game_ai::'):
            continue
        hit = find_sym(sym)
        if not hit:
            continue
        msym, ret, argstr = hit
        args = parse_args(argstr)
        if not args or len(args) > 9:
            continue
        if 'sret' in argstr:
            continue
        rty = 'i64' if ' i64' in ret else ('bool' if 'i1' in ret else ('()' if ret.startswith('void') or ret == '' else None))
        if rty is None:
            continue
        if any(t not in ('ptr', 'i64', 'i32', 'i8', 'i1') for _, t, _ in args):
            continue
        o = off(rva)
        if o is None:
            continue
        tot, pro = 0, []
        for ins in cs.disasm(d[o:o + 64], BASE + rva):
            if any(x in ins.op_str for x in ('rip',)) or ins.mnemonic.startswith('j') or ins.mnemonic == 'call':
                break
            pro += list(ins.bytes)
            tot += ins.size
            if tot >= 12:
                break
        if tot < 12:
            continue
        rows.append((name, rva, msym, args, rty, pro))

    # 3) 코드 생성
    L = []
    L.append('//! sweep.rs — **자동 생성**(`MIG/gensweep.py`). 판정 계층이 RVA 를 확정해 둔 함수를 전수로')
    L.append('//! 게임 vs 내 링크 사본 대조한다. cfg `fn_sweep` 비트마스크로 하나씩/묶어서 켠다.')
    L.append('//! ⚠ABI 가 틀리면 게임이 즉사한다 — 크래시나면 그 비트를 빼고 그 함수는 RVA/시그니처를 다시 본다.')
    L.append('use std::sync::atomic::{AtomicUsize, Ordering};')
    L.append('use std::sync::Mutex;')
    L.append('use super::tune;')
    L.append('')
    L.append('extern "Rust" {')
    for name, rva, msym, args, rty, pro in rows:
        sig = ', '.join('a%d: %s' % (i, {'ptr': '*const u8', 'i64': 'i64', 'i32': 'i32', 'i8': 'u8', 'i1': 'bool'}[t]) for i, (_, t, _) in enumerate(args))
        r = '' if rty == '()' else ' -> %s' % rty
        L.append('    #[link_name = "%s"]' % msym)
        L.append('    fn my_%s(%s)%s;' % (name, sig, r))
    L.append('}')
    L.append('')
    L.append('pub struct Slot { pub name: &\'static str, pub rva: usize, pub orig: AtomicUsize, pub n: AtomicUsize, pub diff: AtomicUsize }')
    L.append('macro_rules! sl { ($n:expr, $r:expr) => { Slot { name: $n, rva: $r, orig: AtomicUsize::new(0), n: AtomicUsize::new(0), diff: AtomicUsize::new(0) } } }')
    L.append('pub static S: [Slot; %d] = [' % len(rows))
    for name, rva, msym, args, rty, pro in rows:
        L.append('    sl!("%s", %#x),' % (name, rva))
    L.append('];')
    L.append('static LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());')
    L.append('thread_local! { static D: [std::cell::Cell<u32>; %d] = [const { std::cell::Cell::new(0) }; %d]; }' % (len(rows), len(rows)))
    L.append('#[inline] fn top(i: usize) -> bool { D.with(|d| { let v = d[i].get(); d[i].set(v + 1); v == 0 }) }')
    L.append('#[inline] fn pop(i: usize) { D.with(|d| d[i].set(d[i].get().saturating_sub(1))); }')
    L.append('fn note(i: usize, s: String) { S[i].diff.fetch_add(1, Ordering::Relaxed); let mut g = LOG.lock().unwrap_or_else(|e| e.into_inner()); if g.len() < 30 { g.push(s); } }')
    L.append('')
    for idx, (name, rva, msym, args, rty, pro) in enumerate(rows):
        rmap = {'ptr': '*const u8', 'i64': 'i64', 'i32': 'i32', 'i8': 'u8', 'i1': 'bool'}
        sig = ', '.join('a%d: %s' % (i, rmap[t]) for i, (_, t, _) in enumerate(args))
        call = ', '.join('a%d' % i for i in range(len(args)))
        r = '' if rty == '()' else ' -> %s' % rty
        rngi = next((i for i, (_, t, dr) in enumerate(args) if t == 'ptr' and dr == 320), None)
        L.append('unsafe fn w_%s(%s)%s {' % (name, sig, r))
        L.append('    let f: unsafe fn(%s)%s = core::mem::transmute(S[%d].orig.load(Ordering::Relaxed));' % (', '.join(rmap[t] for _, t, _ in args), r, idx))
        L.append('    let t = top(%d);' % idx)
        if rngi is not None:
            L.append('    let mut s0 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a%d, s0.as_mut_ptr(), 320); }' % rngi)
        L.append('    let g = f(%s);' % call)
        L.append('    if !t { pop(%d); return g; }' % idx)
        L.append('    S[%d].n.fetch_add(1, Ordering::Relaxed);' % idx)
        if rngi is not None:
            L.append('    let mut af = [0u8; 320]; core::ptr::copy_nonoverlapping(a%d, af.as_mut_ptr(), 320);' % rngi)
            L.append('    core::ptr::copy_nonoverlapping(s0.as_ptr(), a%d as *mut u8, 320);' % rngi)
        L.append('    let m = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_%s(%s)));' % (name, call))
        if rngi is not None:
            L.append('    core::ptr::copy_nonoverlapping(af.as_ptr(), a%d as *mut u8, 320);' % rngi)
        if rty == '()':
            L.append('    let _ = m;')
        else:
            L.append('    if let Ok(m) = m { if m != g { note(%d, format!("%s g={:?} m={:?}", g, m)); } }' % (idx, name))
        L.append('    pop(%d);' % idx)
        L.append('    g')
        L.append('}')
    L.append('')
    L.append('pub unsafe fn install(log: &mut String) {')
    L.append('    let mask = tune("fn_sweep", 0);')
    L.append('    if mask == 0 { return; }')
    L.append('    let w: [usize; %d] = [%s];' % (len(rows), ', '.join('w_%s as usize' % r[0] for r in rows)))
    L.append('    let pro: [&[u8]; %d] = [%s];' % (len(rows), ', '.join('&[%s]' % ', '.join('%#04x' % b for b in r[5]) for r in rows)))
    L.append('    for i in 0..S.len() {')
    L.append('        if mask & (1i64 << i) == 0 { continue; }')
    L.append('        match super::hook::install_wrap_bytes(S[i].rva, pro[i], w[i]) {')
    L.append('            Ok(o) => { S[i].orig.store(o, Ordering::Relaxed); log.push_str(&format!("[sweep] {} OK @{:#x}\\n", S[i].name, S[i].rva)); }')
    L.append('            Err(e) => log.push_str(&format!("[sweep] {} 실패: {} @{:#x}\\n", S[i].name, e, S[i].rva)),')
    L.append('        }')
    L.append('    }')
    L.append('}')
    L.append('pub fn report() -> String {')
    L.append('    let mut s = String::from("[sweep] 판정계층 RVA 확정 함수 전수 대조 (게임 vs 링크 사본)\\n");')
    L.append('    for (i, x) in S.iter().enumerate() {')
    L.append('        let (n, dd) = (x.n.load(Ordering::Relaxed), x.diff.load(Ordering::Relaxed));')
    L.append('        if n == 0 && dd == 0 { continue; }')
    L.append('        s.push_str(&format!("  bit{:<2} {:<22} n={:>10} DIFF={:>7}\\n", i, x.name, n, dd));')
    L.append('    }')
    L.append('    let g = LOG.lock().unwrap_or_else(|e| e.into_inner());')
    L.append('    for l in g.iter() { s.push_str("   "); s.push_str(l); s.push(\'\\n\'); }')
    L.append('    s')
    L.append('}')

    io.open(OUT, 'w', encoding='utf-8').write('\n'.join(L) + '\n')
    print('생성 %d개 함수 → %s' % (len(rows), OUT))
    for i, (name, rva, msym, args, rty, pro) in enumerate(rows):
        print('  bit%-2d %-22s %#-9x 인자%d %s' % (i, name, rva, len(args), rty))


if __name__ == '__main__':
    main()
