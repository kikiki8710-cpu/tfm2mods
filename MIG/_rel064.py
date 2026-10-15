# 0.6.4 릴리스 zip 생성 (2026-10-15)
#   _rel063.py 와 같은 원칙: 0.6.3 zip 의 엔트리 구성을 따르되 재빌드 모드는 **dll·mod.mod_info 만** 게임 배포본으로 교체.
#   차이:
#     ① tfm2_lane_tactics 편입(0.6.3 회차 막바지에 만든 zip 을 원천으로).
#     ② UI_SYNC 모드는 ui/·text/ 엔트리도 **소스(C:\tfm2mods\<top>\)** 바이트로 교체·추가
#        (banpick_view_plus v0.6.2 풀다운·확대 토글·#instant 누락 수정 — 인게임 확인분).
#        그 밖의 모드는 ui/text 를 바꾸지 않고 소스↔zip 차이만 보고(의도치 않은 개발분 유출 방지).
#   순수 stable 3종(Spectator_Chat·community_reaction_mod·tfm2_mod_order)은 0.6.3 zip 바이트 그대로 복사.
#   사용: python MIG\_rel064.py [--dry] [zip이름...]
import zipfile, sys, os, hashlib, json, shutil, time

G = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods'
REPO = r'C:\tfm2mods'
SRC = r'C:\tfm2mods\release\0.6.3'
DST = r'C:\tfm2mods\release\0.6.4'
DRY = '--dry' in sys.argv
ONLY = [x for x in sys.argv[1:] if not x.startswith('--')]
REWORK_DLL = r'C:\tfm2mods\tfm2_champ_pos_lock_rework\target\release\tfm2_champ_pos_lock.dll'
REWORK_INFO = r'C:\tfm2mods\tfm2_champ_pos_lock_rework\mod.mod_info'
DEPS = '>=0.6.4, <0.6.5'
UI_SYNC = {'banpick_view_plus'}
UI_DIRS = ('ui', 'text')
# ★배포 기본값 정규화(유저 지시 07-22 · [[tfm2-release-zip-location]] §공통 규칙) — 0.5.8~0.6.3 zip 이 어겨 왔다:
#   serpen.cfg elder_after 3(라이브값) → 5 · serpen_probe.cfg 1,895B 개발판 → 0.5.7 zip 의 280B 릴리스판
#   (개발판엔 buff_show_delay_tick 키가 없어 장로 버프 표시 지연이 의도한 90 이 아니라 0 으로 동작했다).
TEXT_FIX = {
    ('tfm2_elemental_serpen', 'tfm2_elemental_serpen/config/serpen.cfg'):
        [(b'\nelder_after = 3\n', b'\nelder_after = 5\n')],
}
ENTRY_FROM = {
    ('tfm2_elemental_serpen', 'tfm2_elemental_serpen/serpen_probe.cfg'):
        (r'C:\tfm2mods\release\0.5.7\tfm2_elemental_serpen.zip', 'tfm2_elemental_serpen/serpen_probe.cfg'),
}

PLAN = {
    'tfm2_level_cap': {'tfm2_level_cap'},
    'tfm2_champion_exclude': {'tfm2_champion_exclude'},
    'banpick_view_plus': {'banpick_view_plus'},
    'tfm2_comptest_unlock': {'tfm2_comptest_unlock'},
    'tfm2_elemental_serpen': {'tfm2_elemental_serpen'},
    'tfm2_champ_pos_lock': {'tfm2_champ_pos_lock'},
    'tfm2_champ_pos_lock_rework': {'tfm2_champ_pos_lock'},
    'daram2_viewplus': {'training_view_plus', 'facility_view_plus', 'custom_tier_assignment'},
    'tfm2_lane_tactics': {'tfm2_lane_tactics'},
    'Spectator_Chat': None,
    'community_reaction_mod': None,
    'tfm2_mod_order': None,
}


def h(b):
    return hashlib.sha256(b).hexdigest()[:16]


def src_ui_files(top):
    """소스 폴더의 ui/·text/ 파일 → {zip 상대경로: 절대경로} (.bak 류 제외)."""
    out = {}
    for sub in UI_DIRS:
        root = os.path.join(REPO, top, sub)
        for dp, _, fns in os.walk(root):
            for fn in fns:
                if '.bak' in fn:
                    continue
                ap = os.path.join(dp, fn)
                out[top + '/' + os.path.relpath(ap, os.path.join(REPO, top)).replace('\\', '/')] = ap
    return out


def new_bytes(zipname, top, rel_parts):
    fname = rel_parts[-1]
    if len(rel_parts) == 1:   # 모드 폴더 바로 아래 파일
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
    if ONLY and name not in ONLY:
        continue
    src = os.path.join(SRC, name + '.zip')
    dst = os.path.join(DST, name + '.zip')
    if repl is None:
        print('%-28s COPY (0.6.3 바이트 그대로)' % name)
        if not DRY:
            shutil.copy2(src, dst)
        continue
    zin = zipfile.ZipFile(src)
    names = {zi.filename for zi in zin.infolist()}
    ui_src = {}
    for top in repl:
        ui_src.update(src_ui_files(top))
    out = None if DRY else zipfile.ZipFile(dst, 'w', zipfile.ZIP_DEFLATED)
    n = 0
    for zi in zin.infolist():
        parts = zi.filename.split('/')
        top = parts[0]
        data = zin.read(zi)
        tag = 'keep'
        if top in repl and not zi.is_dir():
            nb = new_bytes(name, top, parts[1:])
            if nb is not None and nb != data:
                tag = 'REPLACED %s→%s' % (h(data), h(nb))
                data = nb
            elif parts[1] in UI_DIRS and zi.filename in ui_src:
                sb = open(ui_src[zi.filename], 'rb').read()
                if sb != data:
                    if top in UI_SYNC:
                        tag = 'UI-SYNC %s→%s' % (h(data), h(sb))
                        data = sb
                    else:
                        print('%-28s %s ⚠소스와 다름(유지)' % (name, zi.filename))
        key = (name, zi.filename)
        if key in ENTRY_FROM:
            zp, en = ENTRY_FROM[key]
            nb = zipfile.ZipFile(zp).read(en)
            if nb != data:
                tag = 'RELEASE-DEFAULT %dB→%dB' % (len(data), len(nb))
                data = nb
        for old, new in TEXT_FIX.get(key, []):
            if old in data:
                data = data.replace(old, new)
                tag = 'TEXTFIX'
        if tag != 'keep':
            print('%-28s %s %s' % (name, zi.filename, tag))
        if out:
            ni = zipfile.ZipInfo(zi.filename, date_time=zi.date_time if tag == 'keep' else now)
            ni.compress_type = zipfile.ZIP_DEFLATED
            ni.external_attr = zi.external_attr
            out.writestr(ni, data)
        n += 1
    # 소스에만 있는 ui/text 파일
    for rel, ap in sorted(ui_src.items()):
        if rel in names:
            continue
        if rel.split('/')[0] in UI_SYNC:
            print('%-28s %s UI-ADD' % (name, rel))
            if out:
                ni = zipfile.ZipInfo(rel, date_time=now)
                ni.compress_type = zipfile.ZIP_DEFLATED
                ni.external_attr = 0o644 << 16
                out.writestr(ni, open(ap, 'rb').read())
                n += 1
        else:
            print('%-28s %s ⚠소스에만 있음(미포함)' % (name, rel))
    if out:
        out.close()
        print('%-28s -> %d B %d entries' % (name, os.path.getsize(dst), n))

if DRY:
    sys.exit(0)

# ── 검증: mod_info BOM·deps, PII, zip dll == 원천, UI_SYNC == 소스 ──
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
        parts = zi.filename.split('/')
        if repl and parts[0] in repl and len(parts) == 2 and parts[1].endswith('.dll'):
            want = open(REWORK_DLL if name == 'tfm2_champ_pos_lock_rework' else os.path.join(G, parts[0], parts[1]), 'rb').read()
            if b != want:
                print('!! dll 불일치', name, zi.filename)
                bad += 1
        if repl and parts[0] in UI_SYNC and parts[1] in UI_DIRS:
            sp = os.path.join(REPO, *parts)
            if os.path.isfile(sp) and open(sp, 'rb').read() != b:
                print('!! UI 불일치', name, zi.filename)
                bad += 1
        if zi.filename.endswith('mod.mod_info'):
            if b[:1] != b'{':
                print('!! BOM/형식', name, zi.filename, b[:3])
                bad += 1
            j = json.loads(b.decode('utf-8'))
            dep = j['dependencies'][0]['version']
            want = DEPS if (repl and parts[0] in repl) else None
            print('   %-28s %-48s v%s deps=%s' % (name, zi.filename, j.get('version'), dep))
            if want and dep != want:
                print('!! deps 불일치', name, zi.filename, dep)
                bad += 1
z = zipfile.ZipFile(os.path.join(DST, 'tfm2_elemental_serpen.zip'))
sc = z.read('tfm2_elemental_serpen/config/serpen.cfg')
pc = z.read('tfm2_elemental_serpen/serpen_probe.cfg')
print('   serpen.cfg elder_after=5:', b'\nelder_after = 5\n' in sc, ' BOM:', sc[:3] == b'\xef\xbb\xbf', ' 0x3f:', sc.count(b'?'))
print('   serpen_probe.cfg', len(pc), 'B  buff_show_delay_tick=90:', b'buff_show_delay_tick = 90' in pc)
if b'\nelder_after = 5\n' not in sc or len(pc) != 280:
    bad += 1
print('검증 이상', bad, '건')
