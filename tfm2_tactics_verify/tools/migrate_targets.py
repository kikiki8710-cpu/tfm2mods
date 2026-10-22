"""게임 패치 대응: probe_targets_<구>.json → probe_targets_<신>.json (2026-10-22).

이 게임은 거의 매주 패치된다. 지점 주소를 손으로 옮기지 않도록 MIG 재핀 엔진(`MIG\\repin.py resolve`)으로
구 exe 의 각 지점을 신 exe 에서 다시 찾는다.
  - 자동 확정: 재핀 결과가 BYTES_UNIQUE 이고 바이트가 그대로인 지점만.
  - 그 밖(바이트 다름·실패·사본 분기)은 overrides 파일의 사람 판정만 받는다. 판정이 없으면 **생성 중단**(종료코드 1).
  - 판정 값: "0x…"(옮김) · ["0x…", "0x…"](사본으로 갈라짐 → key 에 #2, #3 …) · null(이 버전에서 그 엔트리 비활성, reason 필수).
  - 옮긴 지점은 신 exe 에서 바이트를 다시 읽고 명령 꼴(니모닉·피연산자 종류·즉치값)이 구와 같은지 대조한다.
    즉치값이 바뀌었으면 중단(문턱 자체가 바뀐 것 = 기획 재검토 대상).
사용:
  python -I tools/migrate_targets.py <구 targets.json> <구 exe> <신 exe> <신 버전> <overrides.json>
    [--oldpkl P --newpkl P --oldcg P --newcg P]
출력: tools/targets/probe_targets_<신 버전>.json · tools/targets/migrate_<구>_<신>.log.md
"""
import argparse, hashlib, json, os, re, struct, subprocess, sys

import capstone

HERE = os.path.dirname(os.path.abspath(__file__))
MIG = r'C:\tfm2mods\MIG'
LINE = re.compile(r'^(0x[0-9a-f]+)\s+->\s+(0x[0-9a-f]+)\s+(\S+)\s+(bytes동일|bytes다름)')
FAIL = re.compile(r'^(0x[0-9a-f]+)\s+실패')


def load_exe(path):
    data = open(path, 'rb').read()
    pe = struct.unpack_from('<I', data, 0x3c)[0]
    nsec = struct.unpack_from('<H', data, pe + 6)[0]
    optsz = struct.unpack_from('<H', data, pe + 20)[0]
    secs, off = [], pe + 24 + optsz
    for _ in range(nsec):
        vsize, va, rawsz, rawptr = struct.unpack_from('<IIII', data, off + 8)
        secs.append((va, max(vsize, rawsz), rawptr))
        off += 40

    def read(rva, n):
        for va, sz, raw in secs:
            if va <= rva < va + sz:
                return data[raw + rva - va: raw + rva - va + n]
        raise KeyError(hex(rva))
    return hashlib.sha256(data).hexdigest().upper(), read


def shape(md, read, rva, n):
    """명령 꼴: 니모닉 + 피연산자 종류(레지스터 이름은 무시) + 즉치값."""
    ins = next(md.disasm(read(rva, 16), rva))
    kinds = []
    imms = []
    for op in ins.operands:
        kinds.append({1: 'r', 2: 'i', 3: 'm'}.get(op.type, '?'))
        if op.type == capstone.x86.X86_OP_IMM:
            imms.append(op.imm)
    return ins, f"{ins.mnemonic}/{''.join(kinds)}", imms


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('old_targets'); ap.add_argument('old_exe'); ap.add_argument('new_exe')
    ap.add_argument('new_ver'); ap.add_argument('overrides')
    ap.add_argument('--oldpkl'); ap.add_argument('--newpkl'); ap.add_argument('--oldcg'); ap.add_argument('--newcg')
    a = ap.parse_args()

    t = json.load(open(a.old_targets, encoding='utf-8'))
    ov_all = json.load(open(a.overrides, encoding='utf-8')) if os.path.exists(a.overrides) else {}
    ov = ov_all.get('sites', {})
    osha, oread = load_exe(a.old_exe)
    nsha, nread = load_exe(a.new_exe)
    if osha != t['exe_sha256'].upper():
        sys.exit(f'구 exe SHA {osha[:16]} ≠ targets {t["exe_sha256"][:16]}')

    rvas = sorted({s['rva'] for e in t['entries'] for s in e['sites']}, key=lambda x: int(x, 16))
    cmd = [sys.executable, '-X', 'utf8', os.path.join(MIG, 'repin.py'), 'resolve', 'x', '--addrs', *rvas,
           '--old', a.old_exe, '--new', a.new_exe]
    for k in ('oldpkl', 'newpkl', 'oldcg', 'newcg'):
        if getattr(a, k):
            cmd += [f'--{k}', getattr(a, k)]
    out = subprocess.run(cmd, capture_output=True, text=True, encoding='utf-8', errors='replace', cwd=r'C:\tfm2mods').stdout
    auto, note = {}, {}
    for line in out.splitlines():
        m = LINE.match(line.strip())
        if m:
            old, new, how, same = m.groups()
            note[old] = f'{how} {same} → {new}'
            if how.startswith('BYTES_UNIQUE') and same == 'bytes동일':
                auto[old] = new
            continue
        m = FAIL.match(line.strip())
        if m:
            note[m.group(1)] = '재핀 실패'

    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64); md.detail = True
    log = [f'# 지점 이전 {t["game_version"]} → {a.new_ver}', '', f'- 구 exe {osha[:16]} · 신 exe {nsha[:16]}', '',
           '| key | 구 RVA | 신 RVA | 근거 | 명령(구 → 신) |', '|---|---|---|---|---|']
    missing, bad = [], []
    new_entries = []
    for e in t['entries']:
        ne = dict(e); ne['sites'] = []
        disabled = None
        for s in e['sites']:
            old = s['rva']
            if old in ov:
                dest, why = ov[old], 'override: ' + ov_all.get('why', {}).get(old, '')
            elif old in auto:
                dest, why = auto[old], 'auto: ' + note[old]
            else:
                missing.append((e['id'], old, note.get(old, '재핀 결과 없음')))
                continue
            if dest is None:
                disabled = ov_all.get('why', {}).get(old, '사유 미기재')
                log.append(f"| {s['key']} | {old} | (비활성) | {why} | — |")
                continue
            dests = dest if isinstance(dest, list) else [dest]
            oins, oshape, oimm = shape(md, oread, int(old, 16), 16)
            for i, d in enumerate(dests):
                ln = len(bytes.fromhex(s['bytes_hex'].replace(' ', '')))
                nb = nread(int(d, 16), ln)
                nins, nshape, nimm = shape(md, nread, int(d, 16), 16)
                if s.get('role') != 'toggle' and (nshape != oshape or nimm != oimm):
                    bad.append((e['id'], old, d, f'{oins.mnemonic} {oins.op_str} → {nins.mnemonic} {nins.op_str}'))
                ns = dict(s)
                ns['rva'] = d
                ns['key'] = s['key'] if i == 0 else f"{s['key']}#{i + 1}"
                ns['bytes_hex'] = ' '.join(f'{b:02x}' for b in nb)
                ns['asm'] = f'{nins.mnemonic} {nins.op_str}'
                if s.get('imm_offset') is not None:
                    ns['patch_byte_rva'] = hex(int(d, 16) + s['imm_offset'])
                if s.get('branch'):
                    ns['branch'] = dict(s['branch']); ns['branch']['rva'] = None  # gen_sites 가 다음 jcc 를 다시 찾는다
                ns['migrated_from'] = old
                ne['sites'].append(ns)
                log.append(f"| {ns['key']} | {old} | {d} | {why} | `{oins.mnemonic} {oins.op_str}` → `{nins.mnemonic} {nins.op_str}` |")
        if disabled:
            ne['disabled'] = f'{a.new_ver}: {disabled}'
        new_entries.append(ne)

    if missing or bad:
        print('== 사람 판정 필요(overrides 에 추가) ==')
        for x in missing:
            print('  미해결', *x)
        for x in bad:
            print('  명령 꼴/즉치 불일치', *x)
        sys.exit(1)
    nt = dict(t)
    nt.update(game_version=a.new_ver, exe_sha256=nsha, exe_sha256_prefix16=nsha[:16], entries=new_entries,
              migrated_from=f"{t['game_version']} ({t['exe_sha256'][:16]})")
    dst = os.path.join(HERE, 'targets', f'probe_targets_{a.new_ver}.json')
    json.dump(nt, open(dst, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
    open(os.path.join(HERE, 'targets', f"migrate_{t['game_version']}_{a.new_ver}.log.md"), 'w', encoding='utf-8').write('\n'.join(log) + '\n')
    print(f'OK {dst} · 지점 {sum(len(e["sites"]) for e in new_entries)} · 비활성 엔트리 {sum(1 for e in new_entries if e.get("disabled"))}')


if __name__ == '__main__':
    main()
