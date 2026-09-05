# -*- coding: utf-8 -*-
r"""aiport — AI 판단함수 **버전별 재구현(judge 계층) 드라이버**

    python MIG\aiport.py add <name> --rva 0x… [--role scorer|builder|entry|resolver|helper] [--exe PATH]
    python MIG\aiport.py gen        [--exe PATH]                 src\judge\gen_fns.rs 재생성
    python MIG\aiport.py skeleton <name> [--decomp DIR]           src\judge\port\<name>.rs 스켈레톤(디컴 C 동봉)
    python MIG\aiport.py status [--old EXE] [--new EXE] [--apply] 버전 대조(IDENTICAL/SHIFTED/EDITED) + 새 RVA. --apply = 매니페스트 갱신+gen
    python MIG\aiport.py mark <name> todo|ported|verified|stale [--note …]   status 수동 갱신(+gen)
    python MIG\aiport.py list                                     등록 함수 표

## 왜 만들었나 (2026-09-06)
유저 방침: "AI 판단함수는 마이그(구 재현코드 재핀)하지 말고 **매 버전 디컴 소스를 따와 새로 구현해 대체**한다."
그러려면 버전마다 반복되는 기계적 일이 도구여야 한다:
  ① 이 함수가 새 버전에서 **어디 있고(RVA)**, **바뀌었나(EDITED)**       → aidiff 의 Location 지문 재사용
  ② 훅을 안전하게 걸 수 있나 — 프롤로그 몇 바이트를 옮겨야 명령 경계인가, 옮겨도 되는 명령인가(rip-상대/분기 금지),
     그 구간 안으로 뛰어드는 분기가 exe 어딘가에 있나(jmpin)                → 여기서 계산해 매니페스트에 박는다
  ③ Rust 쪽 상수 파일(gen_fns.rs)은 **손으로 쓰지 않는다**                → gen 이 만든다
  ④ 포팅 시작점(스켈레톤 + 디컴 C 동봉)                                  → skeleton

⚠ 매니페스트 = MIG\judge\manifest.json 이 단일 출처. Rust 소스의 RVA/프롤로그 리터럴은 전부 gen 산출물이다.
⚠ 기존 MIG\manifest\tfm2_ai_adjust.json(마이그 축)과 별개다 — 그쪽은 바이트패치·구 훅, 이쪽은 judge 계층.
   단 gen_fns.rs 의 RVA 리터럴은 mig_verify coverage 가 잡으므로, 그 파일은 coverage 의 exclude 에 넣어 둔다(중복 관리 방지).
"""
import argparse
import datetime
import io
import json
import os
import pickle
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import aidiff                                                    # noqa: E402  (stdout 재바인딩 포함)
from capstone import Cs, CS_ARCH_X86, CS_MODE_64                 # noqa: E402
from capstone.x86 import X86_OP_MEM, X86_OP_IMM, X86_REG_RIP    # noqa: E402

BASE = 0x140000000
GAME_EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
MAN = os.path.join(HERE, 'judge', 'manifest.json')
GEN_RS = r'C:\tfm2mods\tfm2_ai_adjust\src\judge\gen_fns.rs'
PORT_DIR = r'C:\tfm2mods\tfm2_ai_adjust\src\judge\port'
DECOMP_DEFAULT = os.path.join(HERE, 'decomp')
MIN_PATCH = 12          # movabs rax,imm64; jmp rax
BRANCH_MN = {'jmp', 'call', 'ret', 'loop', 'loope', 'loopne', 'jrcxz', 'jecxz'}


def load_man():
    if os.path.isfile(MAN):
        return json.load(io.open(MAN, encoding='utf-8'))
    return {'ver': '', 'exe_sha': '', 'fns': {}}


def save_man(m):
    os.makedirs(os.path.dirname(MAN), exist_ok=True)
    io.open(MAN, 'w', encoding='utf-8').write(json.dumps(m, ensure_ascii=False, indent=1))


def game_ver_of(exe):
    """게임 버전 라벨 — 매니페스트가 알면 그 값, 아니면 mig_verify.GAME_VER(패치마다 갱신되는 상수)."""
    try:
        import mig_verify
        return mig_verify.GAME_VER
    except Exception:
        return '?'


# ──────────────────────────────────────────────── 프롤로그 분석

def prologue_plan(img, rva):
    """훅 패치에 옮길 선두 바이트 계획.

    반환 dict(orig_len, prolog(bytes), reloc('ok'|사유), insns[(addr, bytes, text)])
    orig_len = 12B 이상이 되는 첫 명령 경계. 옮기는 명령은 rip-상대 메모리·분기·리턴이면 안 된다
    (rsp-상대는 스텁에서도 같은 rsp 라 안전).
    """
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = True
    blob = img.code(rva, 64)
    n = 0
    insns = []
    reloc = 'ok'
    for ins in md.disasm(blob, BASE + rva):
        insns.append((ins.address, bytes(ins.bytes), '%s %s' % (ins.mnemonic, ins.op_str)))
        mn = ins.mnemonic
        if mn in BRANCH_MN or mn.startswith('j'):
            reloc = '분기/리턴 명령 %s @+%#x' % (mn, ins.address - BASE - rva)
        for op in ins.operands:
            if op.type == X86_OP_MEM and op.mem.base == X86_REG_RIP:
                reloc = 'rip-상대 메모리 %s @+%#x' % (ins.op_str, ins.address - BASE - rva)
        n += ins.size
        if n >= MIN_PATCH:
            break
    if n < MIN_PATCH:
        reloc = '디스어셈 실패(%dB)' % n
    return dict(orig_len=n, prolog=blob[:n], reloc=reloc, insns=insns)


def branch_targets(img):
    """exe .text 전체의 직접 분기/호출 타깃 집합(RVA). 첫 실행 수분, 이후 캐시."""
    cp = os.path.join(HERE, '_jmpin_%s.pkl' % img.sha)
    if os.path.isfile(cp):
        with open(cp, 'rb') as f:
            return pickle.load(f)
    print('[*] 분기 타깃 전수 수집(최초 1회, 수분)…', file=sys.stderr)
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = True
    tg = set()
    for i, (b, e) in enumerate(img.funcs):
        if i % 20000 == 0:
            print('   …%d/%d' % (i, len(img.funcs)), file=sys.stderr)
        blob = img.code(b, e - b)
        if not blob:
            continue
        for ins in md.disasm(blob, BASE + b):
            mn = ins.mnemonic
            if (mn.startswith('j') or mn == 'call') and ins.operands and ins.operands[0].type == X86_OP_IMM:
                tg.add(ins.operands[0].imm - BASE)
    with open(cp, 'wb') as f:
        pickle.dump(tg, f)
    return tg


def jmpin_count(img, rva, orig_len, targets):
    """[rva+1, rva+orig_len) 안으로 들어오는 분기 수(=트램폴린이 깨뜨릴 경로). 0 이어야 안전."""
    return sum(1 for t in targets if rva < t < rva + orig_len)


def fn_record(img, cen, rva, targets):
    """census 항목 + 프롤로그 계획 + jmpin 을 한 레코드로."""
    own = img.owner(rva)
    if not own or own[0] != rva:
        raise SystemExit('RVA %#x 는 .pdata 함수 시작이 아니다(owner=%s)' % (rva, own and '%#x~%#x' % own))
    v = cen.get(own)
    pp = prologue_plan(img, rva)
    rec = dict(
        rva='0x%x' % rva, end='0x%x' % own[1], size=own[1] - own[0],
        orig_len=pp['orig_len'], prolog=' '.join('%02x' % b for b in pp['prolog']),
        prolog_insns=[t for _, _, t in pp['insns']], reloc=pp['reloc'],
        jmpin=jmpin_count(img, rva, pp['orig_len'], targets),
    )
    if v:
        rec.update(
            n=v['n'], sym=v['top'], lines=sorted(set(l for l, _ in v['lines'])),
            callees=['0x%x' % k for k in sorted(v['callees'])],
            offs={'0x%x' % k: c for k, c in sorted(v['offs'].items()) if k >= 0},
            vslots={'0x%x' % k: c for k, c in sorted(v['vslots'].items())},
            imms={('0x%x' % k if k >= 0 else '-0x%x' % -k): c for k, c in sorted(v['imms'].items()) if abs(k) >= 16},
        )
    else:
        rec.update(n=None, sym='(AI 계층 census 밖 — Location 없음)', lines=[], callees=[], offs={}, vslots={}, imms={})
    return rec


# ──────────────────────────────────────────────── 명령

def cmd_add(a):
    img, cen = aidiff.load(a.exe, 'game-ai')
    m = load_man()
    rva = int(a.rva, 16)
    targets = branch_targets(img)
    rec = fn_record(img, cen, rva, targets)
    ver = game_ver_of(a.exe)
    e = m['fns'].setdefault(a.name, {'role': a.role, 'status': 'todo', 'history': []})
    e['role'] = a.role or e.get('role', 'helper')
    e['sym'] = rec['sym']
    e['cur'] = rec
    e['history'].append({'ver': ver, 'rva': rec['rva'], 'size': rec['size'], 'date': datetime.date.today().isoformat(), 'event': 'add'})
    m['ver'] = ver
    m['exe_sha'] = img.sha
    save_man(m)
    print('등록: %s  %s  %s (%dB/%s명령)  orig_len=%d  reloc=%s  jmpin=%d' % (
        a.name, rec['rva'], rec['sym'], rec['size'], rec['n'], rec['orig_len'], rec['reloc'], rec['jmpin']))
    for t in rec['prolog_insns']:
        print('    ', t)
    if rec['reloc'] != 'ok' or rec['jmpin']:
        print('  ⚠ 훅 부적합 — 이 함수는 진입부 트램폴린을 걸면 안 된다(대안: 호출부 훅 / 리턴 주소 훅)')


def cmd_gen(a):
    m = load_man()
    if not m['fns']:
        raise SystemExit('등록된 함수 없음')
    L = []
    w = L.append
    w('// gen_fns.rs — ★자동생성(python MIG\\aiport.py gen). 손으로 고치지 말 것 — 정본 = MIG\\judge\\manifest.json')
    w('//   게임 %s · exe sha %s · %s' % (m['ver'], m['exe_sha'], datetime.datetime.now().strftime('%Y-%m-%d %H:%M')))
    w('//   FnSpec.prolog = 훅이 옮기는 선두 바이트(명령 경계 ≥12B). 설치기는 exe 바이트가 이와 **완전 일치**할 때만 패치한다(패치판/스테일 방어).')
    w('#![allow(dead_code)]')
    w('pub const GAME_VER: &str = "%s";' % m['ver'])
    w('pub struct FnSpec { pub name: &\'static str, pub sym: &\'static str, pub role: &\'static str, pub rva: usize, pub size: usize, pub prolog: &\'static [u8], pub status: &\'static str }')
    names = []
    for name, e in sorted(m['fns'].items()):
        c = e['cur']
        if c['reloc'] != 'ok' or c['jmpin']:
            w('// ⛔ %s: 훅 부적합(reloc=%s jmpin=%d) — 상수만 남기고 ALL 에서 제외' % (name, c['reloc'], c['jmpin']))
        pro = ', '.join('0x%s' % b for b in c['prolog'].split())
        const = name.upper()
        w('pub const %s: FnSpec = FnSpec { name: "%s", sym: r"%s", role: "%s", rva: 0x%x, size: %d, prolog: &[%s], status: "%s" };   // 원본 행 %s · %s명령 · vslots %s' % (
            const, name, c['sym'], e.get('role', 'helper'), int(c['rva'], 16), c['size'], pro, e.get('status', 'todo'),
            ','.join(str(x) for x in c.get('lines', [])) or '-', c.get('n'), ','.join(c.get('vslots', {}).keys()) or '-'))
        if c['reloc'] == 'ok' and not c['jmpin']:
            names.append('&' + const)
    w('pub static ALL: &[&FnSpec] = &[%s];' % ', '.join(names))
    os.makedirs(os.path.dirname(GEN_RS), exist_ok=True)
    io.open(GEN_RS, 'w', encoding='utf-8', newline='\n').write('\n'.join(L) + '\n')
    print('gen: %s (%d함수, 훅 가능 %d)' % (GEN_RS, len(m['fns']), len(names)))


def find_decomp_block(decomp_dir, rva):
    """decomp 트리에서 `## \`0x…\`` 블록을 찾아 (파일, 블록텍스트) 반환."""
    tag = '## `0x%x`' % rva
    for root, _, fs in os.walk(decomp_dir):
        for f in fs:
            if not f.endswith('.md'):
                continue
            p = os.path.join(root, f)
            t = io.open(p, encoding='utf-8', errors='replace').read()
            i = t.find(tag)
            if i < 0:
                continue
            j = t.find('\n## ', i + 5)
            return p, t[i:j if j > 0 else None]
    return None, None


def cmd_skeleton(a):
    m = load_man()
    e = m['fns'].get(a.name)
    if not e:
        raise SystemExit('미등록: %s (먼저 add)' % a.name)
    c = e['cur']
    out = os.path.join(PORT_DIR, a.name + '.rs')
    if os.path.isfile(out) and not a.force:
        raise SystemExit('이미 있음: %s (--force 로 덮어씀 — 손으로 쓴 본문이 날아간다)' % out)
    dec_dir = a.decomp or os.path.join(DECOMP_DEFAULT, m['ver'])
    src, blk = find_decomp_block(dec_dir, int(c['rva'], 16))
    L = []
    w = L.append
    w('//! %s — %s' % (a.name, c['sym']))
    w('//!   게임 %s · RVA %s (%dB / %s명령) · 원본 행 %s · 역할 %s' % (m['ver'], c['rva'], c['size'], c['n'], ','.join(str(x) for x in c.get('lines', [])), e.get('role')))
    w('//!   콜리 %s · vtable 슬롯 %s' % (', '.join(c.get('callees', [])) or '-', ', '.join(c.get('vslots', {}).keys()) or '-'))
    w('//!   필드 오프셋 %s' % (' '.join('%s×%d' % kv for kv in c.get('offs', {}).items()) or '-'))
    w('//! 포팅 규약(CLAUDE.md §3): 게임 함수 호출 0 · 메모리 읽기는 judge::world(safe read) 경유 · 반환 None = 판단 불가(가드) → 호출부가 게임 원본으로 passthrough.')
    w('//! 스켈레톤 생성 = python MIG\\aiport.py skeleton %s (디컴 원문은 아래 주석). 이 파일은 사람이 채운다 — 재생성은 --force 뿐.' % a.name)
    w('use crate::*;')
    w('use super::super::world::*;')
    w('use super::super::ScorerArgs;')
    w('')
    w('/// 반환: Some(점수) / None = 재현 불가 경로(가드) → passthrough')
    w('pub unsafe fn %s(a: &ScorerArgs) -> Option<i64> {' % a.name)
    w('    let _ = a;')
    w('    None   // TODO: 포팅')
    w('}')
    w('')
    w('// ═══ 원본 디컴(자동 동봉, 참고용 — %s) ═══' % (os.path.relpath(src, HERE) if src else '디컴 블록 못 찾음: ' + dec_dir))
    if blk:
        for line in blk.split('\n'):
            w('// ' + line if line.strip() else '//')
    os.makedirs(PORT_DIR, exist_ok=True)
    io.open(out, 'w', encoding='utf-8', newline='\n').write('\n'.join(L) + '\n')
    print('skeleton: %s (%d줄, 디컴 %s)' % (out, len(L), '동봉' if blk else '없음'))


def cmd_status(a):
    m = load_man()
    if not m['fns']:
        raise SystemExit('등록된 함수 없음')
    io_, co = aidiff.load(a.old, 'game-ai')
    in_, cn = aidiff.load(a.new, 'game-ai')
    pairs, gone, added = aidiff.pair(co, cn)
    targets = branch_targets(in_) if a.apply else None
    newver = game_ver_of(a.new)
    rows = []
    for name, e in sorted(m['fns'].items()):
        rva = int(e['cur']['rva'], 16)
        ko = io_.owner(rva)
        if not ko or ko not in co:
            rows.append((name, e['cur']['rva'], '-', '구 census 에 없음', e.get('status')))
            continue
        if ko in pairs:
            kn, _, _ = pairs[ko]
            vd, detail = aidiff.verdict(co[ko], cn[kn])
            rows.append((name, e['cur']['rva'], '0x%x' % kn[0], vd, e.get('status')))
            if a.apply:
                rec = fn_record(in_, cn, kn[0], targets)
                e['history'].append({'ver': newver, 'rva': rec['rva'], 'size': rec['size'], 'date': datetime.date.today().isoformat(),
                                     'event': 'status:%s' % vd, 'from': e['cur']['rva']})
                e['cur'] = rec
                if vd.startswith('EDITED') and e.get('status') in ('ported', 'verified'):
                    e['status'] = 'stale'      # ★소스가 바뀐 함수 = 포팅 다시. 검증 기록도 무효.
        else:
            rows.append((name, e['cur']['rva'], '(삭제)', 'DELETED', e.get('status')))
    print('%-22s %-10s %-10s %-14s %s' % ('함수', '구 RVA', '신 RVA', '판정', 'status'))
    for r in rows:
        print('%-22s %-10s %-10s %-14s %s' % r)
    if a.apply:
        m['ver'] = newver
        m['exe_sha'] = in_.sha
        save_man(m)
        cmd_gen(a)
        print('★ EDITED 였던 ported/verified 함수는 status=stale 로 내렸다. 재포팅 후 status 를 손으로 올릴 것.')


def cmd_mark(a):
    """status 를 손으로 올린다(todo → ported → verified). stale 강등은 status --apply 가 자동으로 한다."""
    m = load_man()
    e = m['fns'].get(a.name)
    if not e:
        raise SystemExit('미등록: %s' % a.name)
    if a.status not in ('todo', 'ported', 'verified', 'stale'):
        raise SystemExit('status 는 todo|ported|verified|stale')
    e['history'].append({'ver': m['ver'], 'rva': e['cur']['rva'], 'date': datetime.date.today().isoformat(),
                         'event': 'mark:%s→%s' % (e.get('status'), a.status), 'note': a.note or ''})
    e['status'] = a.status
    save_man(m)
    cmd_gen(a)
    print('mark: %s → %s' % (a.name, a.status))


def cmd_list(a):
    m = load_man()
    print('게임 %s · exe %s' % (m['ver'], m['exe_sha']))
    for name, e in sorted(m['fns'].items()):
        c = e['cur']
        print('%-22s %-9s %-8s %s  %dB  orig_len=%d reloc=%s jmpin=%d  %s' % (
            name, e.get('role'), e.get('status'), c['rva'], c['size'], c['orig_len'], c['reloc'], c['jmpin'], c['sym']))


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sp = ap.add_subparsers(dest='cmd', required=True)
    p = sp.add_parser('add'); p.add_argument('name'); p.add_argument('--rva', required=True); p.add_argument('--role', default='helper'); p.add_argument('--exe', default=GAME_EXE); p.set_defaults(f=cmd_add)
    p = sp.add_parser('gen'); p.add_argument('--exe', default=GAME_EXE); p.set_defaults(f=cmd_gen)
    p = sp.add_parser('skeleton'); p.add_argument('name'); p.add_argument('--decomp', default=None); p.add_argument('--force', action='store_true'); p.set_defaults(f=cmd_skeleton)
    p = sp.add_parser('status'); p.add_argument('--old', required=True); p.add_argument('--new', default=GAME_EXE); p.add_argument('--apply', action='store_true'); p.set_defaults(f=cmd_status)
    p = sp.add_parser('mark'); p.add_argument('name'); p.add_argument('status'); p.add_argument('--note', default=''); p.set_defaults(f=cmd_mark)
    p = sp.add_parser('list'); p.set_defaults(f=cmd_list)
    a = ap.parse_args()
    a.f(a)


if __name__ == '__main__':
    main()
