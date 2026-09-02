#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""env.py — ★RVA·오프셋이 아닌 "환경 축"을 기계 검사한다 (MIG 의 세 번째 축).

왜 (2026-09-02 0.5.8 실사고 목록 — 전부 이 축이었다):
  · `mod.mod_info` 의 base 대역이 `<0.5.8` 이라 **재핀·재빌드를 다 해도 전 모드가 자동 비활성**.
  · `tfm2_item_tactics` 가 소스에 **exe 크기를 하드코딩한 버전 게이트**를 갖고 있어 스스로 비활성.
  · 빌드 스크립트 6종이 아직 `sdk_057` 을 가리키고 있었다.
  · `flow_capture`·`stat_exp` 는 apply 목록에서 빠졌는데 `rebase` 가 덮어써서 **check 는 PASS** 였다.
  · 배포 dll 이 소스보다 오래된 채로 남아 있었다(level_cap — 재핀 전에 빌드한 파일).
  이것들은 "RVA 가 맞는가"로는 하나도 안 잡힌다. 그래서 별도 축으로 만든다.

사용: python MIG\env.py [--ver 0.5.8] [--exe <exe>] [--sdk sdk_058]
종료코드: 0=클린 / 1=문제
"""
import sys, os, re, json, argparse, datetime

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
import mig_verify as MV  # noqa: E402

GAMEDIR = os.path.dirname(MV.GAME_EXE)
MODSDIR = os.path.join(GAMEDIR, 'mods')
ALT = os.path.join('C:', os.sep, 'Users', 'jungs', 'Desktop', 'claude', 'tfm2', 'tfm2-mods-main')
BUILD_SCRIPTS = ['build_inj.ps1', 'build_extra.ps1', 'build_full.ps1', 'build_full_remap.ps1',
                 os.path.join('tfm2_banpick_illust', 'build.ps1'),
                 os.path.join('tfm2_dashboard_probe', 'build.ps1'),
                 os.path.join('tfm2_meta_item_delegate', 'build_delegate.ps1')]


def vtuple(s):
    return tuple(int(x) for x in s.split('.'))


def band_ok(band, ver):
    """'>=0.5.8, <0.5.9' 가 ver 를 포함하는가."""
    v = vtuple(ver)
    ok = True
    for part in band.split(','):
        m = re.match(r'\s*(>=|<=|<|>|==)\s*(\d+\.\d+\.\d+)', part)
        if not m:
            continue
        op, t = m.group(1), vtuple(m.group(2))
        ok &= {'>=': v >= t, '<=': v <= t, '<': v < t, '>': v > t, '==': v == t}[op]
    return ok


def check_modinfo(ver):
    """①mod_info 무결성(BOM/빈파일/파싱) ②base 대역이 현행 버전을 포함하는가."""
    bad = []
    rows = []
    for d in sorted(os.listdir(MODSDIR)):
        p = os.path.join(MODSDIR, d, 'mod.mod_info')
        if not os.path.isfile(p):
            continue
        raw = open(p, 'rb').read()
        if not raw:
            bad.append((d, '빈 파일(0바이트)'))
            continue
        if raw[:3] == b'\xef\xbb\xbf':
            bad.append((d, 'BOM — 게임 파서가 못 읽어 강제 비활성'))
            continue
        try:
            doc = json.loads(raw.decode('utf-8'))
        except Exception as e:
            bad.append((d, '파싱 실패 %s' % str(e)[:40]))
            continue
        band = None
        for x in doc.get('dependencies') or []:
            if x.get('mod_id') == 'base':
                band = x.get('version')
        rows.append((d, band, band is None or band_ok(band, ver)))
    off = [r for r in rows if not r[2]]
    print('[mod_info] %d개 검사 · 무결성 문제 %d · 현행버전 대역 밖 %d'
          % (len(rows), len(bad), len(off)))
    for d, why in bad:
        print('   !! %-40s %s' % (d, why))
    for d, band, _ in off:
        print('   ·  %-40s deps=%s  → 이 모드는 %s 에서 자동 비활성(의도면 OK)' % (d, band, ver))
    return len(bad)


def check_gates(exe):
    """소스에 exe 크기(십진)를 박아 둔 버전 게이트가 현행 exe 와 맞는가."""
    size = os.path.getsize(exe)
    # ⚠Rust 숫자 구분자 `_` 를 반드시 포함할 것 — `77_111_808` 을 놓쳐서
    #   item_tactics 가 0.5.8 에서 스스로 비활성된 걸 이 검사가 못 잡았다(2026-09-02).
    pat = re.compile(r'\b(\d[\d_]{7,})\b')
    # ★판별자 = "구버전 exe 의 실제 크기와 같은 수". 범위 필터만으론 5^n 같은 상수가 잔뜩 걸린다.
    #   백업 exe(tfm2_0.5.*\TeamfightManager2.exe)의 크기를 사전으로 쓴다.
    old_sizes = {}
    bdir = os.path.join('C:', os.sep, 'Users', 'jungs', 'Desktop', 'claude', 'tfm2')
    for d in sorted(os.listdir(bdir)) if os.path.isdir(bdir) else []:
        p = os.path.join(bdir, d, 'TeamfightManager2.exe')
        if d.startswith('tfm2_0.') and os.path.isfile(p):
            old_sizes.setdefault(os.path.getsize(p), d)
    hits = 0
    for mod in MV.MODS:
        for path in MV.sources(mod):
            if re.search(r'(_backup|\.bak|_old|_release_backup)', path, re.I):
                continue                       # 빌드에 안 들어가는 백업본은 대상 아님
            raw = open(path, 'rb').read().decode('utf-8', 'replace')
            for i, line in enumerate(MV.mask_code(raw).split('\n')):
                for m in pat.finditer(line):
                    v = int(m.group(1).replace('_', ''))
                    if v == size or v not in old_sizes:
                        continue
                    rel = os.path.relpath(path, MV.ROOT).replace('\\', '/')
                    print('   !! %-22s %s:%d  %d = %s 의 exe 크기 (현행 %d) — 버전 게이트 갱신 필요'
                          % (mod, rel, i + 1, v, old_sizes[v], size))
                    hits += 1
    print('[버전 게이트] 현행 exe %d B · 불일치 리터럴 %d곳' % (size, hits))
    return hits


def check_sdk(sdk):
    bad = 0
    for rel in BUILD_SCRIPTS:
        p = os.path.join(MV.ROOT, rel)
        if not os.path.isfile(p):
            continue
        for i, line in enumerate(open(p, encoding='utf-8-sig', errors='replace')):
            code = line.split('#')[0]
            m = re.search(r'sdk_\d+', code)
            if m and m.group(0) != sdk:
                print('   !! %-40s:%d  %s (현행 %s)' % (rel, i + 1, m.group(0), sdk))
                bad += 1
    print('[빌드 스크립트 SDK] 현행 %s · 불일치 %d곳' % (sdk, bad))
    return bad


def check_applied():
    """매니페스트 value 가 소스에 실제로 있는가 = apply 를 빼먹은 모드 탐지.
       ⚠rebase 가 덮어쓰면 check 는 PASS 라 이 검사가 유일한 단서다."""
    bad = 0
    for mod in MV.MODS:
        man = MV.load_man(mod)
        if not man:
            continue
        live = {int(e['value'], 16) for e in man['entries'] if not e.get('ignore')}
        if not live:
            continue
        insrc = set(MV.extract(mod))
        missing = live - insrc
        if missing:
            print('   !! %-22s 매니페스트 값 %d개가 소스에 없음(apply 누락 의심) 예: %s'
                  % (mod, len(missing), [hex(x) for x in sorted(missing)][:5]))
            bad += len(missing)
    print('[apply 반영] 매니페스트↔소스 불일치 %d건' % bad)
    return bad


def check_stale():
    """배포 dll 이 소스보다 오래됐는가(= 재핀 전에 빌드한 파일이 남아 있는가)."""
    uikit = os.path.join(MV.ROOT, 'ui_kit', 'ui_kit.rs')
    ukm = os.path.getmtime(uikit) if os.path.isfile(uikit) else 0
    bad = 0
    for mod in MV.MODS:
        dll = os.path.join(MODSDIR, mod, mod + '.dll')
        srcdir = os.path.join(MV.ROOT, mod)
        if not os.path.isfile(dll) or not os.path.isdir(srcdir):
            continue
        newest = ukm
        for r, ds, fs in os.walk(srcdir):
            ds[:] = [x for x in ds if x not in MV.SKIP_DIRS]
            for f in fs:
                if f.endswith('.rs'):
                    newest = max(newest, os.path.getmtime(os.path.join(r, f)))
        if os.path.getmtime(dll) < newest:
            print('   !! %-22s dll %s < 소스 %s — 재빌드 필요' % (
                mod,
                datetime.datetime.fromtimestamp(os.path.getmtime(dll)).strftime('%m-%d %H:%M'),
                datetime.datetime.fromtimestamp(newest).strftime('%m-%d %H:%M')))
            bad += 1
    print('[배포 신선도] stale dll %d개' % bad)
    return bad


if __name__ == '__main__':
    ap = argparse.ArgumentParser()
    ap.add_argument('--ver', default=MV.GAME_VER)
    ap.add_argument('--exe', default=MV.GAME_EXE)
    ap.add_argument('--sdk', default='sdk_' + MV.GAME_VER.replace('.', '')[-3:])
    a = ap.parse_args()
    n = 0
    n += check_modinfo(a.ver)
    n += check_gates(a.exe)
    n += check_sdk(a.sdk)
    n += check_applied()
    n += check_stale()
    print('\n환경 축 문제 %d건' % n)
    sys.exit(1 if n else 0)
