#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""deploy_stable.py — stable ABI 껍데기 모드(0.6.0+) 빌드 산출물을 게임 mods\ 에 배포한다.
  python deploy_stable.py <MOD_ID> [--dll <cargo 산출 dll>] [--ver <mod_info version>] [--base ">=0.7.0, <0.7.1"(생략=기존 유지)]
하는 일: ① export 검사(tfm2_mod_entry_stable 필수 · 클래식 export 있으면 거부)
        ② <게임설치>\mods\<MOD_ID>\<MOD_ID>.dll 로 복사(크레이트 이름과 무관 · 파일명 = MOD_ID)
        ③ 소스·게임측 mod.mod_info 의 base 대역/version/last_updated 갱신(BOM 없는 UTF-8 · 첫 바이트 7b 검증)
        ④ 배포 dll Length+mtime 출력(= CLAUDE.md §10 증거)
게임 설치 경로는 하드코딩하지 않고 mig_verify.GAME_EXE 기준으로 도출.
"""
import sys, os, json, struct, shutil, time, argparse, glob
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), 'MIG'))
import mig_verify as MV
ROOT = os.path.dirname(os.path.abspath(__file__))
GAMEDIR = os.path.dirname(MV.GAME_EXE)


def exports(p):
    d = open(p, 'rb').read(); pe = struct.unpack_from('<I', d, 0x3c)[0]
    n = struct.unpack_from('<H', d, pe + 6)[0]; opt = struct.unpack_from('<H', d, pe + 20)[0]
    secs = []
    for i in range(n):
        o = pe + 24 + opt + 40 * i; vs, va, rs, rp = struct.unpack_from('<IIII', d, o + 8); secs.append((va, vs, rp, rs))

    def r2o(rva):
        for va, vs, rp, rs in secs:
            if va <= rva < va + max(vs, rs):
                return rp + rva - va
    erva = struct.unpack_from('<I', d, pe + 24 + 112)[0]
    if not erva:
        return []
    eo = r2o(erva); nn = struct.unpack_from('<I', d, eo + 24)[0]; names = struct.unpack_from('<I', d, eo + 32)[0]
    return [d[r2o(struct.unpack_from('<I', d, r2o(names) + 4 * i)[0]):].split(b'\0')[0].decode() for i in range(nn)]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('mod')
    ap.add_argument('--dll', default=None)
    ap.add_argument('--ver', default=None)
    # ~~default='>=0.6.5, <0.6.6'~~ → None = 기존 base 유지(2026-10-29: 0.7.0 에서 기본값이 0.7.0 대역을 0.6.5 로 되돌려 버렸다)
    ap.add_argument('--base', default=None)
    a = ap.parse_args()
    dll = a.dll or (glob.glob(os.path.join(ROOT, a.mod, 'target', 'release', '*.dll')) or [None])[0]
    if not dll or not os.path.isfile(dll):
        sys.exit('산출 dll 없음: %s' % dll)
    ex = exports(dll)
    if 'tfm2_mod_entry_stable' not in ex:
        sys.exit('★stable 진입점 없음 exports=%s' % ex)
    if 'tfm2_mod_entry' in ex:
        sys.exit('★클래식 export 잔존(0.6.0 로더 거부) exports=%s' % ex)
    dst_dir = os.path.join(GAMEDIR, 'mods', a.mod); os.makedirs(dst_dir, exist_ok=True)
    dst = os.path.join(dst_dir, a.mod + '.dll')
    shutil.copy2(dll, dst)
    for p in [os.path.join(ROOT, a.mod, 'mod.mod_info'), os.path.join(dst_dir, 'mod.mod_info')]:
        if not os.path.isfile(p):
            print('⚠mod_info 없음', p); continue
        raw = open(p, 'rb').read()
        if raw[:3] == b'\xef\xbb\xbf':
            raw = raw[3:]
        d = json.loads(raw.decode('utf-8'))
        for dep in d.get('dependencies', []):
            if dep.get('mod_id') == 'base' and a.base:
                dep['version'] = a.base
        if a.ver:
            d['version'] = a.ver
        d['last_updated'] = time.strftime('%Y-%m-%d')
        out = json.dumps(d, ensure_ascii=False).encode('utf-8')
        assert out[:1] == b'{'
        open(p, 'wb').write(out)
        bv = next((x.get('version') for x in d.get('dependencies', []) if x.get('mod_id') == 'base'), None)
        print('mod_info OK  %s  base=%s ver=%s' % (p, bv, d.get('version')))
    st = os.stat(dst)
    print('OK: deployed %s  Length=%d  LastWriteTime=%s  exports=%s'
          % (dst, st.st_size, time.strftime('%Y-%m-%d %H:%M:%S', time.localtime(st.st_mtime)), ex))
    # ★2026-10-14: 이 스크립트는 dll·mod_info 만 옮긴다. ui/·text/ 를 고친 모드는 게임 쪽이 옛판으로 남는다
    #   (banpick_view_plus 풀다운·확대 토글이 인게임에서 안 보인 실사고) ⟹ 다르면 경고만(자동 덮어쓰기 안 함 — 게임 쪽 수동 수정본 보호).
    stale = []
    for sub in ('ui', 'text'):
        sroot = os.path.join(ROOT, a.mod, sub)
        if not os.path.isdir(sroot):
            continue
        for dp, _, fns in os.walk(sroot):
            for fn in fns:
                if '.bak' in fn:
                    continue
                sp = os.path.join(dp, fn); rel = os.path.relpath(sp, os.path.join(ROOT, a.mod))
                gp = os.path.join(dst_dir, rel)
                if not os.path.isfile(gp) or open(sp, 'rb').read() != open(gp, 'rb').read():
                    stale.append(rel)
    if stale:
        print('⚠ui/text 가 게임 쪽과 다름(직접 복사 필요): %s' % ', '.join(stale))


if __name__ == '__main__':
    main()
