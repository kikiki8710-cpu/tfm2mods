#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""MIG 매니페스트 도구 — 마이그 = "매니페스트의 각 엔트리를 새 버전에서 찾는 것"

사용법:
  python mig_verify.py gen [MOD ...]        매니페스트 스켈레톤 생성(소스 스캔 + 현행 exe 바이트 채록)
  python mig_verify.py check [MOD ...] [--exe PATH]
                                            전 엔트리를 exe 와 대조 -> PASS/STALE/INVALID
                                            (패치 직후 새 exe 로 실행 = 깨진 곳 전수 목록)
  python mig_verify.py coverage [MOD ...]   소스의 RVA 대역 리터럴 중 매니페스트 미등록 검출
  python mig_verify.py dups                 같은 값이 여러 모드에 -> 연동 그룹 보고(로컬 복사본 방어)
  python mig_verify.py rebase MOD [--exe PATH]
                                            재핀 후 검증 바이트를 exe 에서 다시 채록

종료코드: 0=클린 / 1=문제 있음(STALE·미등록·INVALID)
"""
import sys, os, re, json, glob
sys.stdout.reconfigure(encoding='utf-8', errors='replace')

ROOT = r'C:\tfm2mods'
MIGD = os.path.join(ROOT, 'MIG')
MAND = os.path.join(MIGD, 'manifest')
GAME_EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
RVA_LO, RVA_HI = 0x80000, 0x4800000   # .text + RVA vtable(.rdata ~0x34e6a20) 포함
NBYTES = 12                            # 엔트리당 채록 바이트

# 마이그 대상 모드 (설치 여부 아님 — ui_kit 같은 공유모듈 포함이 이 목록의 존재 이유)
MODS = [
  'tfm2_ai_adjust', 'tfm2_item_tactics', 'tfm2_champ_pos_lock',
  'tfm2_comptest_unlock', 'tfm2_banpick_order', 'tfm2_banpick_illust',
  'tfm2_elemental_serpen', 'tfm2_draft_overlay', 'tfm2_flow_capture',
  'tfm2_stat_exp', 'tfm2_level_cap', 'tfm2_champion_exclude',
  'tfm2_bancard_keep', 'tfm2_mod_order', 'tfm2_html_overlay',
  'sylas', 'Spectator_Chat', 'community_reaction_mod',
  'tfm2_meta_item_delegate',
  'ui_kit',   # 공유모듈(dll 없음) — 0.5.7 사고 1·2 의 근원
]
SKIP_DIRS = {'target', '_archive', 'backup_0.2.26', 'release'}


def sources(mod):
    base = os.path.join(ROOT, mod)
    out = []
    for r, ds, fs in os.walk(base):
        ds[:] = [d for d in ds if d not in SKIP_DIRS]
        out += [os.path.join(r, f) for f in fs if f.endswith('.rs')]
    return sorted(out)


_BLK = re.compile(r'/\*.*?\*/', re.S)


def strip_code(text):
    """주석 제거하되 줄 구조 유지(라인번호 보존)."""
    def keep_nl(m):
        return '\n' * m.group(0).count('\n')
    text = _BLK.sub(keep_nl, text)
    return '\n'.join(l.split('//')[0] for l in text.split('\n'))


HEXP = re.compile(r'0x([0-9a-fA-F]{5,8})\b')
CONST = re.compile(r'const\s+([A-Z_][A-Z0-9_]*)\s*:')


def extract(mod):
    """소스에서 RVA 대역 리터럴 전수 추출 -> {value: {'locs': [...], 'name': 최선 이름}}"""
    found = {}
    for path in sources(mod):
        rel = os.path.relpath(path, os.path.join(ROOT, mod)).replace('\\', '/')
        raw = open(path, 'rb').read().decode('utf-8', 'replace')
        lines = strip_code(raw).split('\n')
        cur_const, const_line = None, -10
        for i, line in enumerate(lines):
            m = CONST.search(line)
            if m:
                cur_const, const_line = m.group(1), i
            for h in HEXP.finditer(line):
                v = int(h.group(1), 16)
                if not (RVA_LO <= v < RVA_HI):
                    continue
                e = found.setdefault(v, {'locs': [], 'name': None})
                e['locs'].append(rel + ':' + str(i + 1))
                # const 선언 8줄 이내면 그 이름 귀속(여러 줄 배열 커버), 아니면 INLINE
                if e['name'] is None:
                    if i - const_line <= 8 and cur_const:
                        e['name'] = cur_const
                    else:
                        e['name'] = 'INLINE@' + rel + ':' + str(i + 1)
    return found


def read_exe(exe):
    import pefile
    pe = pefile.PE(exe, fast_load=True)
    sects = [(s.VirtualAddress, s.VirtualAddress + s.Misc_VirtualSize,
              s.Name.rstrip(b'\x00').decode()) for s in pe.sections]

    def at(rva, n=NBYTES):
        try:
            return pe.get_data(rva, n)
        except Exception:
            return None

    def sect(rva):
        for lo, hi, nm in sects:
            if lo <= rva < hi:
                return nm
        return '?'
    return at, sect


def man_path(mod):
    return os.path.join(MAND, mod + '.json')


def load_man(mod):
    p = man_path(mod)
    return json.load(open(p, encoding='utf-8')) if os.path.isfile(p) else None


def save_man(mod, man):
    json.dump(man, open(man_path(mod), 'w', encoding='utf-8'),
              ensure_ascii=False, indent=1)


def cmd_gen(mods, exe):
    at, sect = read_exe(exe)
    for mod in mods:
        old = load_man(mod)
        keep = {e['value']: e for e in old['entries']} if old else {}
        found = extract(mod)
        entries = []
        for v in sorted(found):
            hexv = hex(v)
            if hexv in keep and keep[hexv].get('curated'):
                entries.append(keep[hexv])
                continue
            b = at(v)
            e = {'name': found[v]['name'], 'value': hexv, 'ver': '0.5.7',
                 'kind': 'UNCLASSIFIED', 'locs': found[v]['locs'][:6],
                 'sect': sect(v), 'bytes': b.hex() if b else None,
                 'method': 'bytes 12B find_unique -> 실패시 match_fn/match_mid(_mig 엔진)'}
            if hexv in keep:   # 비큐레이션 기존 엔트리의 수동 필드 승계
                for k in ('kind', 'method', 'note', 'ignore', 'ver'):
                    if k in keep[hexv]:
                        e[k] = keep[hexv][k]
            if b is None:
                e['note'] = (e.get('note', '') + ' *exe 범위 밖=INVALID').strip()
            entries.append(e)
        man = old or {'mod': mod, 'game_ver': '0.5.7', 'build': '',
                      'notes': [], 'offsets': []}
        man['entries'] = entries
        save_man(mod, man)
        print('%-26s 엔트리 %d개 -> manifest/%s.json' % (mod, len(entries), mod))


def cmd_check(mods, exe):
    at, _ = read_exe(exe)
    bad = 0
    for mod in mods:
        man = load_man(mod)
        if not man:
            print('%-26s !매니페스트 없음' % mod)
            bad += 1
            continue
        rows = []
        for e in man['entries']:
            if e.get('ignore'):
                continue
            cur = at(int(e['value'], 16))
            if e.get('bytes') is None or cur is None:
                rows.append(('INVALID', e))
                continue
            rows.append(('PASS' if cur.hex() == e['bytes'] else 'STALE', e))
        st = [r for r in rows if r[0] != 'PASS']
        suffix = '' if rows else ' (엔트리 0 = SDK 전용, 재빌드만)'
        print('%-26s PASS %3d / STALE·INVALID %3d%s'
              % (mod, len(rows) - len(st), len(st), suffix))
        for tag, e in st:
            print('    %-7s %-32s %10s [%s] %s'
                  % (tag, e['name'], e['value'], e.get('ver', '?'), e['locs'][0]))
        bad += len(st)
    return bad


def cmd_coverage(mods):
    bad = 0
    for mod in mods:
        man = load_man(mod)
        known = {int(e['value'], 16) for e in man['entries']} if man else set()
        missing = {v: d for v, d in extract(mod).items() if v not in known}
        if missing:
            print('%-26s *미등록 %d건' % (mod, len(missing)))
            for v in sorted(missing):
                d = missing[v]
                print('    %-32s %10s %s' % (d['name'] or '?', hex(v), d['locs'][0]))
            bad += len(missing)
        else:
            print('%-26s 커버리지 클린' % mod)
    return bad


def cmd_dups():
    byval = {}
    for f in glob.glob(os.path.join(MAND, '*.json')):
        man = json.load(open(f, encoding='utf-8'))
        for e in man['entries']:
            if not e.get('ignore'):
                byval.setdefault(e['value'], []).append((man['mod'], e['name']))
    n = 0
    for v, users in sorted(byval.items()):
        if len(users) > 1:
            n += 1
            print('%10s  %s' % (v, ' | '.join(m + ':' + nm for m, nm in users)))
    print('-- 연동 그룹 %d개 (한쪽만 고치면 사고 — 0x1788 참조)' % n)
    return 0


def cmd_rebase(mod, exe):
    at, sect = read_exe(exe)
    man = load_man(mod)
    assert man, '매니페스트 없음'
    for e in man['entries']:
        b = at(int(e['value'], 16))
        e['bytes'] = b.hex() if b else None
        e['sect'] = sect(int(e['value'], 16))
    save_man(mod, man)
    print('%s: %d개 엔트리 바이트 재채록 완료' % (mod, len(man['entries'])))


if __name__ == '__main__':
    args = sys.argv[1:]
    cmd = args[0] if args else 'check'
    exe = GAME_EXE
    if '--exe' in args:
        i = args.index('--exe')
        exe = args[i + 1]
        del args[i:i + 2]
    sel = [a for a in args[1:] if not a.startswith('-')] or list(MODS)
    if cmd == 'gen':
        cmd_gen(sel, exe)
        sys.exit(0)
    elif cmd == 'check':
        sys.exit(1 if cmd_check(sel, exe) else 0)
    elif cmd == 'coverage':
        sys.exit(1 if cmd_coverage(sel) else 0)
    elif cmd == 'dups':
        sys.exit(cmd_dups())
    elif cmd == 'rebase':
        cmd_rebase(sel[0], exe)
        sys.exit(0)
    else:
        print(__doc__)
        sys.exit(2)
