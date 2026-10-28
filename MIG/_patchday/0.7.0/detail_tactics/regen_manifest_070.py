"""상세전술 매니페스트 재생성(0.7.0) — sites.rs 생성물 → MIG/manifest/tfm2_detail_tactics.json (2026-10-28)

`mig_verify.py gen` 은 값이 같은 큐레이션 엔트리만 보존해 값이 바뀐 창의 roles·contracts 가 사라진다(README ③-6).
0.6.5 매니페스트의 큐레이션(PATCH_SITE/RETURN_SITE/BRANCH_TARGET 의 roles·contracts)은 전부 sites.rs 의
Patch 행에서 기계적으로 나온 값이므로, 같은 스키마로 신 sites.rs 에서 다시 만든다.
- NOT_RVA(사람이 위치를 확인한 제외 항목)는 값이 여전히 소스에 있으면 그대로 승계(locs 만 갱신).
- 출력 후 `mig_verify.py check/coverage tfm2_detail_tactics --exe <0.7.0>` 로 검증한다.
사용: python -X utf8 regen_manifest_070.py
"""
import argparse, hashlib, json, re, sys
from pathlib import Path

MODS = Path(r'F:\claude\tfm2mods')
MOD = MODS / 'tfm2_detail_tactics'
MAN = MODS / 'MIG' / 'manifest' / 'tfm2_detail_tactics.json'
ap = argparse.ArgumentParser()
ap.add_argument('--exe', default=r'F:\claude\tfm2\tfm2_0.7.0\TeamfightManager2.exe')
ap.add_argument('--sites', default=str(MOD / 'src' / 'sites.rs'))
ap.add_argument('--ver', default='0.7.0')
ap.add_argument('--old', default=str(MAN), help='큐레이션(NOT_RVA·notes) 승계 원천')
ap.add_argument('--out', default=str(MAN), help='자기검증 때는 다른 경로로')
A = ap.parse_args()
EXE = Path(A.exe)
V = A.ver
sys.path.insert(0, str(MOD / 'tools'))
sys.path.insert(0, str(MODS / 'MIG'))
from gen_sites import PE, find_func  # noqa: E402
import mig_verify  # noqa: E402

METHOD = '현행 EXE 12B 및 .pdata/호출자/원천 JSON 대조. 패치 시 tools/targets 원천을 먼저 재탐색하고 gen_sites.py로 다시 생성.'
OWNER = '해당 sites 행의 Src::Caller 중첩 계약을 그대로 참조'

old = json.loads(Path(A.old).read_text(encoding='utf-8'))
pe = PE(EXE)
sha = hashlib.sha256(pe.data).hexdigest().upper()
S, E = pe.functions()
lines = Path(A.sites).read_text(encoding='utf-8').split('\n')
assert f'pub const EXE_SHA256: &str = "{sha}";' in lines, 'sites.rs 가 이 exe 의 생성물이 아니다'


def b12(v):
    o = pe.off(v)
    return pe.data[o:o + 12].hex() if o is not None else None


def sect(v):
    for name, va, sz, _raw, _rsz in pe.secs:
        if va <= v < va + sz:
            return name
    return None


pat = re.compile(r'Patch \{ name: "([^"]+)", tactic: (\d+), rva: (0x[0-9a-f]+), win: &\[([^\]]*)\].*?'
                 r'branches: &\[(.*?)\], src: (.*), team_chk: (.*), first_len: (\d+), dynamic:')
entries, ret_use, br_use = {}, {}, {}
tactics, n_caller_uses, n_branch_uses = set(), 0, 0
for i, ln in enumerate(lines, 1):
    m = pat.search(ln)
    if not m:
        continue
    name, tac, rva, win, br, src, chk, fl = m.groups()
    tactics.add(int(tac))
    v = int(rva, 16)
    wb = [int(x, 16) for x in win.split(',') if x.strip()]
    fn = find_func(S, E, v)
    assert fn, name
    loc = f'src/sites.rs:{i}'
    entries[v] = {'name': name, 'value': hex(v), 'ver': V, 'kind': 'PATCH_SITE', 'locs': [loc], 'sect': sect(v), 'bytes': b12(v),
                  'method': METHOD, 'curated': True, 'roles': ['PATCH_SITE'],
                  'contracts': [{'name': name, 'rva': hex(v), 'length': len(wb), 'bytes': ' '.join(f'{x:02x}' for x in wb),
                                 'first_len': int(fl), 'function': [hex(fn[0]), hex(fn[1])], 'src': src, 'team_chk': chk}]}
    rets = list(dict.fromkeys(re.findall(r'ret_rva: (0x[0-9a-f]+)', src + ' ' + chk)))
    n_caller_uses += len(rets)
    for r in rets:
        ret_use.setdefault(int(r, 16), []).append((name, loc))
    tg = list(dict.fromkeys(re.findall(r'target_rva: (0x[0-9a-f]+)', br)))
    n_branch_uses += len(tg)
    for t in tg:
        br_use.setdefault(int(t, 16), []).append((name, loc))

for kind, uses, nm, key in (('RETURN_SITE', ret_use, 'CALLER_RETURN_', 'return_rva'), ('BRANCH_TARGET', br_use, 'RELOCATED_BRANCH_', 'target_rva')):
    for v, us in uses.items():
        contracts = [{'site': s, key: hex(v), **({'owner_filter': OWNER} if kind == 'RETURN_SITE' else {})} for s, _ in us]
        if v in entries:  # 다른 역할과 겹치면 역할·계약만 합친다(0.6.5 엔 없었음)
            e = entries[v]
            e['roles'].append(kind)
            e['contracts'] += contracts
            e['locs'] = list(dict.fromkeys(e['locs'] + [l for _, l in us]))
            continue
        entries[v] = {'name': f'{nm}{hex(v)}', 'value': hex(v), 'ver': V, 'kind': kind, 'locs': list(dict.fromkeys(l for _, l in us)),
                      'sect': sect(v), 'bytes': b12(v), 'method': METHOD, 'curated': True, 'roles': [kind], 'contracts': contracts}

# NOT_RVA 승계 — 소스에 아직 있는 값만
found = mig_verify.extract('tfm2_detail_tactics')
kept_not_rva, dropped_not_rva = [], []
for e in old['entries']:
    if e.get('kind') != 'NOT_RVA':
        continue
    v = int(e['value'], 16)
    if v in found and v not in entries:
        e = dict(e)
        e['locs'] = found[v]['locs'][:6]
        entries[v] = e
        kept_not_rva.append(e['value'])
    else:
        dropped_not_rva.append(e['value'])

man = dict(old)
man['game_ver'] = V
man['build'] = 'powershell -NoProfile -ExecutionPolicy Bypass -File F:/claude/tfm2mods/tfm2_detail_tactics/build_local.ps1 -RunTests'
n_patch = sum(1 for e in entries.values() if 'PATCH_SITE' in e.get('roles', []))
man['notes'] = [f'2026-10-28 0.7.0 재생성(MIG/_patchday/0.7.0/detail_tactics/regen_manifest_070.py — 0.6.5 큐레이션과 같은 스키마로 sites.rs 에서 기계 생성). '
                f'실제 소스 생성물 {len(tactics)}전술(미지원 1 = lane_opportunity_enemy_limit){n_patch}창 + Caller 반환주소 + 재배치 분기 목적지. game EXE SHA256 고정.'] + old['notes'][1:]
inputs = [MOD / 'src' / 'sites.rs', MOD / 'tools' / 'gen_sites.py', MOD / 'tools' / 'gen_extras.py', MOD / 'tools' / 'promote_verified.py',
          MOD / 'src' / 'verified_candidates.rs'] + [MOD / 'tools' / 'targets' / f'{n}_{V}.json' for n in
                                                      ('knobs', 'subjects', 'extra_lane', 'extra_combat', 'extra_ops', 'extra_verified')]
man['source_contracts'] = {'exe_sha256': sha, 'tactics': 100, 'supported_tactics': len(tactics), 'patch_sites': n_patch,
                           'unique_caller_returns': len(ret_use), 'caller_uses': n_caller_uses,
                           'unique_branch_targets': len(br_use), 'branch_uses': n_branch_uses,
                           'input_sha256': {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs}}
man['entries'] = [entries[v] for v in sorted(entries)]
Path(A.out).write_text(json.dumps(man, ensure_ascii=False, indent=2) + '\n', encoding='utf-8', newline='\n')
print(f'엔트리 {len(man["entries"])} (PATCH {n_patch} · RETURN {len(ret_use)} · BRANCH {len(br_use)} · NOT_RVA 승계 {kept_not_rva} 제외 {dropped_not_rva}) '
      f'caller_uses {n_caller_uses} branch_uses {n_branch_uses} → {A.out}')
