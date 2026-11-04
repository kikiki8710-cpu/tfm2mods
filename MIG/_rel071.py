# 0.7.1 릴리스 zip 생성 (2026-11-04) — 원천 = release\0.7.0 의 같은 이름 zip(_rel070.py·_rel070_1104.py 산출).
#   엔트리 구성·순서·나머지 바이트는 그대로 두고 바뀐 엔트리만 교체한다:
#     재빌드 모드의 dll·mod.mod_info = 게임 배포본(deploy-verify 활성 경로 불일치 0 · 1호기 인게임 확인판)
#     tfm2_transfer_fix·tfm2_level_cap 은 README.txt 도 소스로 교체(v0.2.2 ② 제거 안내 · level_cap 은 09-19 부터 「0.6.0 전용」 문구가 낡아 있었다)
#     banpick_view_plus ui/·text/·asset/ 은 소스와 대조만 한다(0.7.1 번들 .ui 동일 · 이번 회차 소스 변경 없음)
#   deps 상한 없는 3종(Spectator_Chat·community_reaction_mod·tfm2_mod_order)은 바이트 그대로 복사.
#   제외 = tfm2_detail_tactics(별도 세션) · tfm2_lane_tactics(영구) · pos_lock_rework(0.7.0 미마이그).
#   ★serpen.cfg·serpen_probe.cfg 는 원천 zip 의 배포 기본값을 잇되 값을 단언한다([[tfm2-release-zip-location]] §공통 규칙).
#   사용: python MIG\_rel071.py [--dry]
import zipfile, sys, os, hashlib, json, shutil, time

G = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods'
REPO = r'F:\claude\tfm2mods'
SRC = r'F:\claude\tfm2mods\release\0.7.0'
DST = r'F:\claude\tfm2mods\release\0.7.1'
DRY = '--dry' in sys.argv
DEPS = '>=0.7.1, <0.7.2'
UI_DIRS = ('ui', 'text', 'asset')
PLAN = {
    'tfm2_level_cap': {'tfm2_level_cap'},
    'tfm2_champion_exclude': {'tfm2_champion_exclude'},
    'banpick_view_plus': {'banpick_view_plus'},
    'tfm2_comptest_unlock': {'tfm2_comptest_unlock'},
    'tfm2_elemental_serpen': {'tfm2_elemental_serpen'},
    'tfm2_champ_pos_lock': {'tfm2_champ_pos_lock'},
    'daram2_viewplus': {'training_view_plus', 'facility_view_plus', 'custom_tier_assignment'},
    'tfm2_transfer_fix': {'tfm2_transfer_fix'},
    'Spectator_Chat': None,
    'community_reaction_mod': None,
    'tfm2_mod_order': None,
}
SRC_FILE = {m + '/README.txt': os.path.join(REPO, m, 'README.txt') for m in ('tfm2_transfer_fix', 'tfm2_level_cap')}
PII = [b'C:\\Users', b'jungs', b'kikiki8710', 'C:\\Users'.encode('utf-16le'), 'jungs'.encode('utf-16le')]


def h(b):
    return hashlib.sha256(b).hexdigest()[:12]


if not DRY:
    os.makedirs(DST, exist_ok=True)
now = time.localtime()[:6]
bad = 0
for name, repl in PLAN.items():
    src = os.path.join(SRC, name + '.zip')
    dst = os.path.join(DST, name + '.zip')
    if repl is None:
        print('%-24s COPY (0.7.0 바이트 그대로)' % name)
        if not DRY:
            shutil.copy2(src, dst)
        continue
    zin = zipfile.ZipFile(src)
    out = None if DRY else zipfile.ZipFile(dst + '.tmp', 'w', zipfile.ZIP_DEFLATED)
    n = 0
    for zi in zin.infolist():
        parts = zi.filename.split('/')
        data = zin.read(zi)
        new = None
        if parts[0] in repl and not zi.is_dir():
            if len(parts) == 2 and (parts[1].endswith('.dll') or parts[1] == 'mod.mod_info'):
                new = open(os.path.join(G, *parts), 'rb').read()
            elif zi.filename in SRC_FILE:
                new = open(SRC_FILE[zi.filename], 'rb').read()
            elif parts[0] == 'banpick_view_plus' and parts[1] in UI_DIRS:
                sp = os.path.join(REPO, *parts)
                if os.path.isfile(sp) and open(sp, 'rb').read() != data:
                    print('%-24s %s ⚠소스와 다름(원천 유지)' % (name, zi.filename))
        t = 'keep'
        if new is not None and new != data:
            t = 'REPLACED %s→%s (%d→%d B)' % (h(data), h(new), len(data), len(new))
            data = new
        if t != 'keep':
            print('%-24s %-52s %s' % (name, zi.filename, t))
        low = zi.filename.lower()
        if t == 'keep' and low.endswith(('.txt', '.md', '.mod_info')) and b'0.7.0' in data:
            ln = [l.strip()[:90] for l in data.decode('utf-8', 'replace').splitlines() if '0.7.0' in l][:3]
            print('%-24s %s ⚠「0.7.0」 문구: %s' % (name, zi.filename, ln))
        if out:
            ni = zipfile.ZipInfo(zi.filename, date_time=zi.date_time if t == 'keep' else now)
            ni.compress_type = zipfile.ZIP_DEFLATED
            ni.external_attr = zi.external_attr
            out.writestr(ni, data)
        n += 1
    if out:
        out.close()
        os.replace(dst + '.tmp', dst)
        print('%-24s -> %d B %d entries' % (name, os.path.getsize(dst), n))

if DRY:
    sys.exit(0)

# ── 검증 ──
for name, repl in PLAN.items():
    z = zipfile.ZipFile(os.path.join(DST, name + '.zip'))
    zs = zipfile.ZipFile(os.path.join(SRC, name + '.zip'))
    if len(z.infolist()) != len(zs.infolist()):
        print('!! 엔트리 수', name, len(zs.infolist()), len(z.infolist()))
        bad += 1
    for zi in z.infolist():
        if zi.is_dir():
            continue
        b = z.read(zi)
        for p in PII:
            if p in b:
                print('!! PII', name, zi.filename, p)
                bad += 1
        parts = zi.filename.split('/')
        if repl and parts[0] in repl and len(parts) == 2 and parts[1].endswith('.dll'):
            if b != open(os.path.join(G, *parts), 'rb').read():
                print('!! dll 불일치', name, zi.filename)
                bad += 1
        if zi.filename.endswith('mod.mod_info'):
            if b[:1] != b'{':
                print('!! BOM/형식', name, zi.filename, b[:3])
                bad += 1
            j = json.loads(b.decode('utf-8'))
            dep = [d['version'] for d in j['dependencies'] if d.get('mod_id') == 'base'][0]
            print('   %-22s %-46s v%-6s author=%-9s deps=%s' % (name, zi.filename, j.get('version'), j.get('author'), dep))
            if repl and parts[0] in repl and dep != DEPS:
                print('!! deps 불일치', name, zi.filename, dep)
                bad += 1
            if repl is None and '<' in dep:
                print('!! 상한 있는 deps 를 복사', name, zi.filename, dep)
                bad += 1
z = zipfile.ZipFile(os.path.join(DST, 'tfm2_elemental_serpen.zip'))
sc = z.read('tfm2_elemental_serpen/config/serpen.cfg')
pc = z.read('tfm2_elemental_serpen/serpen_probe.cfg')
print('   serpen.cfg elder_after=5:', b'\nelder_after = 5\n' in sc, ' BOM:', sc[:3] == b'\xef\xbb\xbf', ' 0x3f:', sc.count(b'?'))
print('   serpen_probe.cfg', len(pc), 'B')
if b'\nelder_after = 5\n' not in sc or len(pc) != 280:
    bad += 1
z = zipfile.ZipFile(os.path.join(DST, 'tfm2_transfer_fix.zip'))
names = [x.filename for x in z.infolist()]
print('   transfer_fix 엔트리', names)
if any(x.endswith('.cfg') or x.endswith('.txt') and 'README' not in x for x in names):
    print('!! transfer_fix cfg·로그 동봉')
    bad += 1
print('검증 이상', bad, '건')
for name in PLAN:
    p = os.path.join(DST, name + '.zip')
    print('%-24s %9d B  %s' % (name, os.path.getsize(p), hashlib.sha256(open(p, 'rb').read()).hexdigest()[:12]))
