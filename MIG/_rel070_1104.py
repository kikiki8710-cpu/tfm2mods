# 0.7.0 릴리스 zip 갱신 (2026-11-04) — 신규 UI 대응 5모드(코치진·선수단·훈련 v0.6.1 · 시설 v0.6.2 · 밴픽 v0.6.4)
#   원천 = 현재 release\0.7.0\ 의 같은 이름 zip(10-29 `_rel070.py` 산출). 엔트리 구성·순서·나머지 바이트는 그대로 두고
#   바뀐 엔트리만 교체한다: 대상 모드의 dll·mod.mod_info = 게임 배포본(deploy-verify PASS 판) · 밴픽 ui/·text/·asset/ = 소스.
#   구판은 덮어쓰기 전에 `<이름>-<꼬리표>.zip` 으로 보존(선례 = tfm2_transfer_fix-0.1.0.zip).
#   recruitment_view_plus·custom_tier_assignment 는 이번 변경 대상이 아니라 원천 바이트 그대로(10-29 판단 유지).
#   사용: python MIG\_rel070_1104.py [--dry]
import zipfile, sys, os, hashlib, json, shutil, time

G = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods'
REPO = r'F:\claude\tfm2mods'
DST = r'F:\claude\tfm2mods\release\0.7.0'
DRY = '--dry' in sys.argv
UI_DIRS = ('ui', 'text', 'asset')
PLAN = {
    # zip 이름: (교체 대상 모드, UI 를 소스와 동기할 모드, 구판 꼬리표)
    'daram2_viewplus': ({'roster_view_plus', 'coaching_staff_view_plus', 'training_view_plus', 'facility_view_plus'}, set(), '20261029'),
    'banpick_view_plus': ({'banpick_view_plus'}, {'banpick_view_plus'}, '0.6.3'),
}
WANT_VER = {'roster_view_plus': '0.6.1', 'coaching_staff_view_plus': '0.6.1', 'training_view_plus': '0.6.1',
            'facility_view_plus': '0.6.2', 'banpick_view_plus': '0.6.4'}
WANT_DEPS = {'roster_view_plus': '>=0.6.0', 'coaching_staff_view_plus': '>=0.6.0'}  # 나머지 = '>=0.7.0, <0.7.1'
PII = [b'C:\\Users', b'jungs', b'kikiki8710', 'C:\\Users'.encode('utf-16le'), 'jungs'.encode('utf-16le')]


def h(b):
    return hashlib.sha256(b).hexdigest()[:12]


now = time.localtime()[:6]
bad = 0
for name, (repl, ui_sync, tag) in PLAN.items():
    cur = os.path.join(DST, name + '.zip')
    old = os.path.join(DST, '%s-%s.zip' % (name, tag))
    if not DRY and not os.path.isfile(old):
        shutil.copy2(cur, old)
        print('%-20s 구판 보존 → %s (%d B)' % (name, os.path.basename(old), os.path.getsize(old)))
    src = old if os.path.isfile(old) else cur           # 재실행해도 원천은 늘 구판(교체할 cur 를 열어 두면 Windows 에서 덮어쓰기 실패)
    zin = zipfile.ZipFile(src)
    out = None if DRY else zipfile.ZipFile(cur + '.tmp', 'w', zipfile.ZIP_DEFLATED)
    n = 0
    for zi in zin.infolist():
        parts = zi.filename.split('/')
        data = zin.read(zi)
        new = None
        if parts[0] in repl and not zi.is_dir():
            if len(parts) == 2 and (parts[1].endswith('.dll') or parts[1] == 'mod.mod_info'):
                new = open(os.path.join(G, *parts), 'rb').read()
            elif parts[0] in ui_sync and parts[1] in UI_DIRS:
                sp = os.path.join(REPO, *parts)
                if os.path.isfile(sp):
                    new = open(sp, 'rb').read()
        t = 'keep'
        if new is not None and new != data:
            t = 'REPLACED %s→%s (%d→%d B)' % (h(data), h(new), len(data), len(new))
            data = new
        if t != 'keep':
            print('%-20s %-58s %s' % (name, zi.filename, t))
        if out:
            ni = zipfile.ZipInfo(zi.filename, date_time=zi.date_time if t == 'keep' else now)
            ni.compress_type = zipfile.ZIP_DEFLATED
            ni.external_attr = zi.external_attr
            out.writestr(ni, data)
        n += 1
    n_src = len(zin.infolist())
    zin.close()
    if out:
        out.close()
        os.replace(cur + '.tmp', cur)
        print('%-20s -> %d B %d entries (원천 %d entries)' % (name, os.path.getsize(cur), n, n_src))

if DRY:
    sys.exit(0)

# ── 검증: 엔트리 수 동일 · 대상 dll/mod_info == 게임 배포본 · 밴픽 UI == 소스 · 비대상 == 구판 바이트 · BOM·version·deps · PII ──
for name, (repl, ui_sync, tag) in PLAN.items():
    z = zipfile.ZipFile(os.path.join(DST, name + '.zip'))
    zo = zipfile.ZipFile(os.path.join(DST, '%s-%s.zip' % (name, tag)))
    if [i.filename for i in z.infolist()] != [i.filename for i in zo.infolist()]:
        print('!! 엔트리 목록 다름', name); bad += 1
    for zi in z.infolist():
        if zi.is_dir():
            continue
        b = z.read(zi)
        parts = zi.filename.split('/')
        for p in PII:
            if p in b:
                print('!! PII', name, zi.filename, p); bad += 1
        target = parts[0] in repl and len(parts) == 2 and (parts[1].endswith('.dll') or parts[1] == 'mod.mod_info')
        ui = parts[0] in ui_sync and len(parts) > 1 and parts[1] in UI_DIRS
        if target:
            if b != open(os.path.join(G, *parts), 'rb').read():
                print('!! 배포본과 다름', name, zi.filename); bad += 1
        elif ui:
            sp = os.path.join(REPO, *parts)
            if os.path.isfile(sp) and open(sp, 'rb').read() != b:
                print('!! 소스와 다름', name, zi.filename); bad += 1
        elif b != zo.read(zi.filename):
            print('!! 비대상 엔트리가 바뀜', name, zi.filename); bad += 1
        if zi.filename.endswith('mod.mod_info'):
            if b[:1] != b'{':
                print('!! BOM/형식', name, zi.filename, b[:3]); bad += 1
            j = json.loads(b.decode('utf-8'))
            dep = j['dependencies'][0]['version']
            print('   %-20s %-44s v%-6s deps=%s' % (name, zi.filename, j.get('version'), dep))
            if parts[0] in repl:
                if j.get('version') != WANT_VER[parts[0]]:
                    print('!! version', parts[0], j.get('version')); bad += 1
                if dep != WANT_DEPS.get(parts[0], '>=0.7.0, <0.7.1'):
                    print('!! deps', parts[0], dep); bad += 1
        if zi.filename.endswith('mod.override_info') and parts[0] != 'training_view_plus' and parts[0] != 'banpick_view_plus':
            if b != b'{}':
                print('!! 빈 override_info 아님', name, zi.filename); bad += 1
    print('%-20s sha256=%s' % (name, hashlib.sha256(open(os.path.join(DST, name + '.zip'), 'rb').read()).hexdigest()))
print('검증 이상', bad, '건')
sys.exit(1 if bad else 0)
