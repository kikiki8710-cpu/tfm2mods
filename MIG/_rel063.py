# 0.6.3 릴리스 zip 생성 (2026-10-07)
#   0.6.2 zip 의 엔트리 구성을 그대로 따르되, 재핀 모드는 **dll·mod.mod_info 만** 새 산출물로 교체한다.
#   (0.6.2 의 _rel062.py 는 모드 폴더 파일 전체를 게임 배포본으로 교체 → serpen 개발 cfg 등 라이브 테스트값이
#    섞일 위험이 있어 교체 대상을 dll·mod_info 로 한정. cfg 는 0.6.2 zip 의 배포 기본값을 그대로 유지.)
#   순수 stable 3종(Spectator_Chat·community_reaction_mod·tfm2_mod_order)은 0.6.2 zip 바이트 그대로 복사.
#   사용: python MIG\_rel063.py [--dry]
import zipfile, sys, os, hashlib, json, shutil, time

G = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods'
SRC = r'C:\tfm2mods\release\0.6.2'
DST = r'C:\tfm2mods\release\0.6.3'
DRY = '--dry' in sys.argv
REWORK_DLL = r'C:\tfm2mods\tfm2_champ_pos_lock_rework\target\release\tfm2_champ_pos_lock.dll'
REWORK_INFO = r'C:\tfm2mods\tfm2_champ_pos_lock_rework\mod.mod_info'
DEPS = '>=0.6.3, <0.6.4'

# zip 이름 -> 교체할 모드 폴더(top) 집합. None = 바이트 그대로 복사
PLAN = {
    'tfm2_level_cap': {'tfm2_level_cap'},
    'tfm2_champion_exclude': {'tfm2_champion_exclude'},
    'banpick_view_plus': {'banpick_view_plus'},
    'tfm2_comptest_unlock': {'tfm2_comptest_unlock'},
    'tfm2_elemental_serpen': {'tfm2_elemental_serpen'},
    'tfm2_champ_pos_lock': {'tfm2_champ_pos_lock'},
    'tfm2_champ_pos_lock_rework': {'tfm2_champ_pos_lock'},
    'daram2_viewplus': {'training_view_plus', 'facility_view_plus', 'custom_tier_assignment'},
    'Spectator_Chat': None,
    'community_reaction_mod': None,
    'tfm2_mod_order': None,
}


def h(b):
    return hashlib.sha256(b).hexdigest()[:16]


def new_bytes(zipname, top, fname):
    """교체 원천. dll·mod_info 외에는 None(=0.6.2 zip 엔트리 유지)."""
    if zipname == 'tfm2_champ_pos_lock_rework':
        if fname.endswith('.dll'):
            return open(REWORK_DLL, 'rb').read()
        if fname == 'mod.mod_info':
            return open(REWORK_INFO, 'rb').read()
        return None
    if fname.endswith('.dll') or fname == 'mod.mod_info':
        p = os.path.join(G, top, fname)
        return open(p, 'rb').read() if os.path.isfile(p) else None
    return None


os.makedirs(DST, exist_ok=True)
now = time.localtime()[:6]
for name, repl in PLAN.items():
    src = os.path.join(SRC, name + '.zip')
    dst = os.path.join(DST, name + '.zip')
    if repl is None:
        print('%-28s COPY (0.6.2 바이트 그대로)' % name)
        if not DRY:
            shutil.copy2(src, dst)
        continue
    zin = zipfile.ZipFile(src)
    out = None if DRY else zipfile.ZipFile(dst, 'w', zipfile.ZIP_DEFLATED)
    for zi in zin.infolist():
        parts = zi.filename.split('/')
        top, fname = parts[0], parts[-1]
        data = zin.read(zi)
        tag = 'keep'
        if top in repl and not zi.is_dir() and len(parts) == 2:
            nb = new_bytes(name, top, fname)
            if nb is not None and nb != data:
                tag = 'REPLACED %s→%s' % (h(data), h(nb))
                data = nb
        if tag != 'keep':
            print('%-28s %s %s' % (name, zi.filename, tag))
        if out:
            ni = zipfile.ZipInfo(zi.filename, date_time=zi.date_time if tag == 'keep' else now)
            ni.compress_type = zipfile.ZIP_DEFLATED
            ni.external_attr = zi.external_attr
            out.writestr(ni, data)
    if out:
        out.close()
        print('%-28s -> %d B %d entries' % (name, os.path.getsize(dst), len(zin.infolist())))

if DRY:
    sys.exit(0)

# ── 검증: mod_info BOM·deps, PII, zip dll == 원천 ──
PII = [b'C:\\Users', b'jungs', b'kikiki8710', 'C:\\Users'.encode('utf-16le'), 'jungs'.encode('utf-16le')]
bad = 0
for name, repl in PLAN.items():
    z = zipfile.ZipFile(os.path.join(DST, name + '.zip'))
    for zi in z.infolist():
        if zi.is_dir():
            continue
        b = z.read(zi)
        for p in PII:
            if p in b:
                print('!! PII', name, zi.filename, p)
                bad += 1
        if zi.filename.endswith('mod.mod_info'):
            if b[:1] != b'{':
                print('!! BOM/형식', name, zi.filename, b[:3])
                bad += 1
            dep = json.loads(b.decode('utf-8'))['dependencies'][0]['version']
            want = DEPS if (repl and zi.filename.split('/')[0] in repl) else None
            print('   %-28s %-48s deps=%s' % (name, zi.filename, dep))
            if want and dep != want:
                print('!! deps 불일치', name, zi.filename, dep)
                bad += 1
print('검증 이상', bad, '건')
