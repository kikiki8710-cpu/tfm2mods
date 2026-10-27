#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""run_patchday.py — 패치날 비파괴 실행기: 새 exe 하나로 지문·콜그래프·AI diff·STALE·재핀 계획·오프셋·상세전술·주소록을 돌리고 SUMMARY.md 대시보드를 만든다.

사용 (F:\\claude\\tfm2mods 에서):
  python -X utf8 MIG\\patchday\\run_patchday.py [--new-exe <경로, 기본=설치본>] [--label <버전라벨>]
         [--rehearse] [--skip <단계,...>] [--only <단계,...>]

  --label   기본 = <게임폴더>\\mod-sdk-stable\\base_version.txt. 핫픽스(버전 문자열은 기준과 같은데 sha 가 다름,
            또는 F:\\claude\\tfm2\\tfm2_<버전>\\ 에 sha 가 다른 exe 가 이미 있음)면 '<버전>_<sha앞8>'.
  --rehearse 백업 단계를 건너뛰고 출력을 MIG\\_patchday\\_rehearsal_<label>\\ 로(예: 0.6.4 exe 로 거꾸로 리허설).
  --skip/--only 단계 이름 또는 번호(쉼표). detect 는 항상 돈다(식별 필수). summary 는 --skip summary 가 아니면 항상 돈다.

단계 (각 단계 종료코드·소요·로그 = <out>\\logs\\NN_<단계>.log · 실패해도 다음 단계 계속):
  0 detect         새 exe sha256 vs 기준(0.6.5) sha. 같으면 "아직 업데이트 안 됨" → 종료코드 0, 출력 폴더도 안 만든다.
  1 backup         F:\\claude\\tfm2\\tfm2_<label>\\ 에 TeamfightManager2.exe·bundle.game_data 파일 단위 복사(.part→이름바꿈·sha 대조).
                   이미 있으면 sha 비교 후 건너뜀, 다르면 덮어쓰지 않고 오류. 백업이 성공하면 이후 단계는 백업본을 신 exe 로 쓴다.
  2 fnindex        fnindex.py → F:\\claude\\tfm2mods\\_fnidx_<tok>.pkl (있으면 함수 경계 전수·skel 표본 대조로 검증 후 재사용)
  3 callgraph      MIG\\callgraph.py → _cg_<tok>.pkl (있으면 callee 표본 대조로 검증 후 재사용)
  4 aidiff         MIG\\aidiff.py 기준exe 신exe -o <out>\\aidiff.md
  5 check          MIG\\mig_verify.py check <MODS + tfm2_detail_tactics> --exe 신exe
  6 repin          MIG\\repin.py plan (같은 모드 목록) → <out>\\repin_map.json → MIG\\sitealign.py → repin_map.aligned.json
                   → 함수시작 교차검증(콜러 수·callee 사영·size 비 2/3 + 동형 클론) repin_fncheck.json · 대기열 repin_review.md
                   ★apply/--write 는 절대 부르지 않는다.
  7 offsets        offsets.py 의 fingerprint/shift_pairs 를 import 해 「기준선 지문 ↔ 재핀 후보값의 신 exe 지문」 대조
                   → <out>\\offsets_shifts.json. (offsets.py check 를 직접 부르지 않는 이유는 아래)
  8 detail_tactics tfm2_detail_tactics\\tools\\track_update.py track … --out <out>\\detail_tactics  (0/1 정상 · 2 오류)
  9 addrbook       MIG\\patchday\\addrbook.py map … --out <out>\\addrmap  (0/1 정상 · 2 오류 · 2.4만 행 ≈28분·메모리 약 9GB —
                   2026-10-27 리허설 실측. 패치날엔 --skip addrbook 으로 먼저 돌리고 --only addrbook 을 따로)
  10 summary       <out>\\SUMMARY.md (한국어 대시보드) + <out>\\run.json (기계용)

★offsets.py check 를 직접 부르지 않는 이유 (코드 확인 2026-10-27, MIG\\offsets.py):
  · cmd_snap 은 기준선 MIG\\offsets\\<MOD>.json 을 덮어쓴다(L89) — 절대 호출 금지.
  · cmd_check 도 이동이 있으면 git 추적 파일 MIG\\offsets\\_shifts.json 을 덮어쓴다(L145-148).
  · cmd_check 는 매니페스트의 **현재 value**(=apply 전이면 구버전 RVA)로 신 exe 를 지문한다(L118-128)
    ⟹ apply 전에 돌리면 엉뚱한 함수를 본다. 그래서 여기서는 repin 후보값(new_aligned > new, PASS 는 그대로)으로 대조한다.
  · apply 뒤의 정식 ④ offsets check/sources 는 MIG\\README.md 표준 절차 그대로 사람이 돌린다.

비파괴 보장: 매니페스트·모드 소스·targets·기준선을 쓰는 명령은 부르지 않는다. 시작·끝에
  git -C F:\\claude\\tfm2mods status --porcelain 스냅샷 + 보호 파일(매니페스트·offsets 기준선·모드 src·targets·기준선 json)
  크기/mtime 스냅샷을 떠서, 출력 폴더(MIG\\_patchday\\…)·*.pkl·__pycache__ 밖의 변화가 있으면 SUMMARY 맨 위에 경고한다.

종료코드: 0 = 업데이트 없음, 또는 모든 단계가 정상 종료(검토 항목이 있어도 0 — 내용은 SUMMARY)
          1 = 하나 이상 단계 오류, 또는 비파괴 경고 발생
          2 = 실행기 자체 오류(인자·기준 exe 없음 등)
런북 = MIG\\patchday\\README.md
"""
import argparse
import collections
import datetime
import glob
import hashlib
import io
import json
import os
import pickle
import queue
import re
import shutil
import struct
import subprocess
import sys
import threading
import time

try:
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')
    sys.stderr.reconfigure(encoding='utf-8', errors='replace')
except Exception:
    pass

# ───────────────────────────────────────────────────────────── 기준(현행) — 회차가 끝나면 여기만 갱신
BASE_VER = '0.6.5'
BASE_SHA = 'DAF040DB8A3A3AE136F3EF8CE704E2718F049656B6B5E4872EACB599811D30B5'
BASE_SIZE = 87009792

ROOT = r'F:\claude\tfm2mods'
MIGD = os.path.join(ROOT, 'MIG')
HERE = os.path.dirname(os.path.abspath(__file__))
PDROOT = os.path.join(MIGD, '_patchday')            # 출력 루트(비파괴 허용 구역)
TFM2 = r'F:\claude\tfm2'                             # 백업 tfm2_<label>\ 의 부모
BASE_TOK = BASE_VER.replace('.', '')
BASE_EXE = os.path.join(TFM2, 'tfm2_' + BASE_VER, 'TeamfightManager2.exe')
BASE_PKL = os.path.join(ROOT, '_fnidx_%s.pkl' % BASE_TOK)
BASE_CG = os.path.join(ROOT, '_cg_%s.pkl' % BASE_TOK)

FNINDEX = os.path.join(ROOT, 'fnindex.py')
CALLGRAPH = os.path.join(MIGD, 'callgraph.py')
AIDIFF = os.path.join(MIGD, 'aidiff.py')
MIGVERIFY = os.path.join(MIGD, 'mig_verify.py')
REPIN = os.path.join(MIGD, 'repin.py')
SITEALIGN = os.path.join(MIGD, 'sitealign.py')
DT = os.path.join(ROOT, 'tfm2_detail_tactics')
TRACK = os.path.join(DT, 'tools', 'track_update.py')
TRACK_BASE = os.path.join(DT, 'tools', 'patchday', 'baseline_%s.json' % BASE_VER)
ADDRBOOK = os.path.join(HERE, 'addrbook.py')
ADDRBOOK_JSON = os.path.join(HERE, 'addrbook_%s.json' % BASE_VER)
HOTFIX_JSON = os.path.join(HERE, 'hotfix_exposure_%s.json' % BASE_VER)

EXTRA_MODS = ['tfm2_detail_tactics']   # ★mig_verify.MODS 에 없는 모드(알려진 빈틈) — 여기서 명시 추가해 검사한다
# sites.rs 가 생성물인 모드 — repin apply 대상이 아니다(README ③ track_update·④ migrate_targets 가 따로 추적).
# 재핀 「사람 판정 필요」에서 따로 센다(2026-10-27 리허설: 132건 중 116건이 이 두 모드라 apply 대상 16건이 묻혔다).
GEN_MODS = ('tfm2_detail_tactics', 'tfm2_tactics_verify')

STAGES = ['detect', 'backup', 'fnindex', 'callgraph', 'aidiff', 'check', 'repin', 'offsets',
          'detail_tactics', 'addrbook', 'summary']
ST_KO = {'OK': '정상', 'REVIEW': '정상·검토 필요', 'CACHED': '캐시 재사용(검증 통과)', 'SKIP': '건너뜀',
         'NOTOOL': '도구 없음', 'NOINPUT': '선행 산출물 없음', 'FAIL': '★오류'}

PY = sys.executable


def now():
    return datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')


def sha256_file(p, chunk=1 << 22):
    h = hashlib.sha256()
    with open(p, 'rb') as f:
        while True:
            b = f.read(chunk)
            if not b:
                break
            h.update(b)
    return h.hexdigest().upper()


def mig_verify_mods():
    """mig_verify.MODS·GAME_EXE 를 정본에서 읽는다(목록을 여기 복제하지 않는다)."""
    sys.path.insert(0, MIGD)
    import mig_verify as MV  # noqa: E402  (모듈 수준 부작용 = stdout reconfigure 뿐)
    return list(MV.MODS), MV.GAME_EXE, MV


def rel(p, base):
    try:
        return os.path.relpath(p, base).replace('\\', '/')
    except ValueError:
        return p


# ───────────────────────────────────────────────────────────── PE·pkl 검증(캐시 재사용 판단)
class PE:
    def __init__(self, path):
        d = self.d = open(path, 'rb').read()
        pe = struct.unpack_from('<I', d, 0x3c)[0]
        nsec = struct.unpack_from('<H', d, pe + 6)[0]
        opt = pe + 24
        magic = struct.unpack_from('<H', d, opt)[0]
        sectab = opt + struct.unpack_from('<H', d, pe + 20)[0]
        self.secs = []
        for i in range(nsec):
            o = sectab + i * 40
            vsz, va, rsz, raw = struct.unpack_from('<IIII', d, o + 8)
            self.secs.append((va, max(vsz, rsz), raw, rsz))
        ddir = opt + (112 if magic == 0x20b else 96)
        self.ex_rva, self.ex_sz = struct.unpack_from('<II', d, ddir + 3 * 8)

    def roff(self, rva):
        for va, vsz, raw, rsz in self.secs:
            if va <= rva < va + vsz:
                o = rva - va
                return raw + o if o < rsz else None
        return None

    def ranges(self):
        """fnindex.build 와 같은 규칙으로 .pdata 고유 함수 경계."""
        po = self.roff(self.ex_rva)
        r = {}
        for i in range(self.ex_sz // 12):
            b, e, _ = struct.unpack_from('<III', self.d, po + i * 12)
            if e <= b or e - b > 1 << 20:
                continue
            if b not in r or e > r[b]:
                r[b] = e
        return r


_RE_NUM = re.compile(r'0x[0-9a-f]+')


def _md():
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64
    m = Cs(CS_ARCH_X86, CS_MODE_64)
    m.detail = False
    return m


def _samples(keys, n=48):
    ks = sorted(keys)
    if len(ks) <= n:
        return ks
    step = len(ks) / float(n)
    return [ks[int(i * step)] for i in range(n)]


def verify_fnidx(pkl, exe_path, exe_sha):
    """pkl 이 이 exe 의 fnindex 산출물인가 — ①경로 메타 sha ②함수 경계 전수 ③skel 표본 재계산."""
    notes = []
    try:
        P = pickle.load(open(pkl, 'rb'))
    except Exception as e:
        return False, 'pkl 읽기 실패: %s' % e, None
    idx = {(int(k, 16) if isinstance(k, str) else k): v for k, v in P.get('idx', {}).items()}
    mp = P.get('path')
    if mp and os.path.isfile(mp):
        try:
            same = sha256_file(mp) == exe_sha
            notes.append('메타 경로 sha %s' % ('일치' if same else '다름(경로의 파일이 그 뒤 바뀌었을 수 있음 — 경계·표본으로 판정)'))
        except Exception:
            notes.append('메타 경로 sha 확인 불가')
    else:
        notes.append('메타 경로 없음/접근 불가 — 경계·표본으로 판정')
    pe = PE(exe_path)
    rg = pe.ranges()
    bad_b = [b for b, v in idx.items() if b not in rg or rg[b] - b != v.get('size')]
    if bad_b:
        return False, '함수 경계 불일치 %d개 (예 %s) — 다른 exe 의 pkl' % (len(bad_b), hex(bad_b[0])), None
    miss = len(rg) - len(idx)
    if miss > max(50, len(rg) // 1000):
        return False, '.pdata 함수 %d 중 pkl 에 %d 개 없음' % (len(rg), miss), None
    md = _md()
    for b in _samples(idx):
        v = idx[b]
        o = pe.roff(b)
        if o is None:
            continue
        toks = [ins.mnemonic + '|' + _RE_NUM.sub('I', ins.op_str)
                for ins in md.disasm(pe.d[o:o + v['size']], 0x140000000 + b)]
        if hashlib.md5('\n'.join(toks).encode()).hexdigest() != v.get('skel'):
            return False, 'skel 표본 불일치 @%s' % hex(b), None
    notes.append('경계 %d/%d 일치 · skel 표본 %d 일치' % (len(idx), len(rg), min(48, len(idx))))
    return True, ' · '.join(notes), idx


def verify_cg(cg, exe_path, idx):
    """cg pkl 이 이 exe+fnidx 의 callgraph 산출물인가 — 키 포함관계 + callee 표본 재계산."""
    try:
        G = pickle.load(open(cg, 'rb'))
    except Exception as e:
        return False, 'cg 읽기 실패: %s' % e
    callee = G.get('callee', {})
    starts = set(idx)
    if not set(callee) <= starts:
        return False, 'callee 키가 fnidx 함수시작 밖 %d개' % len(set(callee) - starts)
    pe = PE(exe_path)
    md = _md()
    for st in _samples(idx, 40):
        o = pe.roff(st)
        if o is None:
            continue
        tg = set()
        for ins in md.disasm(pe.d[o:o + idx[st]['size']], st):
            if ins.mnemonic in ('call', 'jmp') and ins.op_str.startswith('0x'):
                t = int(ins.op_str, 16)
                if t in starts and t != st:
                    tg.add(t)
        if sorted(tg) != sorted(callee.get(st, [])):
            return False, 'callee 표본 불일치 @%s' % hex(st)
    return True, 'callee %d개 · 표본 40 일치' % len(callee)


# ───────────────────────────────────────────────────────────── 비파괴 스냅샷
ALLOW_PREFIX = ('MIG/_patchday/',)


def _allowed(path):
    p = path.replace('\\', '/').strip('"')
    return p.startswith(ALLOW_PREFIX) or p.endswith('.pkl') or '__pycache__' in p


def git_status():
    try:
        r = subprocess.run(['git', '-C', ROOT, '-c', 'core.quotepath=false', 'status', '--porcelain'],
                           capture_output=True, timeout=180)
        return [l for l in r.stdout.decode('utf-8', 'replace').splitlines() if l.strip()], r.returncode
    except Exception as e:
        return ['<git status 실패: %s>' % e], -1


def protected_files(mods):
    pats = ['MIG/manifest/*.json', 'MIG/offsets/*.json', 'MIG/repin_map_*.json', 'MIG/mig_verify.py',
            'MIG/patchday/*.json',
            'tfm2_detail_tactics/src/*.rs', 'tfm2_detail_tactics/tools/targets/*.json',
            'tfm2_detail_tactics/tools/patchday/*.json', 'tfm2_detail_tactics/tools/visible_extras.json',
            'tfm2_tactics_verify/src/*.rs', 'tfm2_tactics_verify/tools/targets/*.json',
            'tfm2_tactics_verify/tools/sites.json']
    for m in mods:
        pats += ['%s/src/**/*.rs' % m, '%s/mod.mod_info' % m]
    out = {}
    for pat in pats:
        for f in glob.glob(os.path.join(ROOT, pat), recursive=True):
            try:
                st = os.stat(f)
                out[rel(f, ROOT)] = (st.st_size, st.st_mtime_ns)
            except OSError:
                pass
    return out


def snapshot(mods):
    lines, rc = git_status()
    return {'git': lines, 'git_rc': rc, 'files': protected_files(mods), 'at': now()}


def snap_diff(a, b):
    """시작↔끝 스냅샷 차이 중 허용 구역 밖 = 경고."""
    warn = []
    ga, gb = set(a['git']), set(b['git'])
    for l in sorted(gb - ga):
        if not _allowed(l[3:]):
            warn.append('git status 새 줄: `%s`' % l)
    for l in sorted(ga - gb):
        if not _allowed(l[3:]):
            warn.append('git status 사라진 줄: `%s`' % l)
    fa, fb = a['files'], b['files']
    for k in sorted(set(fa) | set(fb)):
        if fa.get(k) != fb.get(k):
            what = '생김' if k not in fa else ('사라짐' if k not in fb else '바뀜(크기/시각)')
            warn.append('보호 파일 %s: `%s`' % (what, k))
    return warn


# ───────────────────────────────────────────────────────────── 하위 명령 실행(tee + 진행 표시)
PROGRESS = re.compile(r'(^\s*\.\.\.|^\s*…|\[saved\]|\[build\]|^\[\*\]|^\s*->|^집계|^리포트|완료 \d)')


def run_cmd(args, log_path, title, echo_lines=25, heartbeat=30):
    env = dict(os.environ, PYTHONUTF8='1', PYTHONIOENCODING='utf-8', PYTHONUNBUFFERED='1')  # 파이프여도 진행 줄이 바로 보이게
    shown = [os.path.basename(x) if (x.endswith('.py') and os.path.isabs(x)) else x for x in args[1:]]
    cmdline = ' '.join('"%s"' % x if ' ' in x else x for x in args)
    print('  $ python ' + ' '.join(shown))
    t0 = time.time()
    with io.open(log_path, 'w', encoding='utf-8', newline='\n') as lf:
        lf.write('# %s  %s\n$ %s\n\n' % (now(), title, cmdline))
        lf.flush()
        try:
            p = subprocess.Popen(args, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, cwd=ROOT, env=env)
        except Exception as e:
            lf.write('실행 실패: %s\n' % e)
            print('    실행 실패: %s' % e)
            return -1, time.time() - t0
        q = queue.Queue()

        def rd():
            for raw in iter(p.stdout.readline, b''):
                q.put(raw)
            q.put(None)
        threading.Thread(target=rd, daemon=True).start()
        n, last = 0, time.time()
        while True:
            try:
                raw = q.get(timeout=5)
            except queue.Empty:
                if time.time() - last >= heartbeat:
                    print('    … %s 진행 중 (%ds 경과)' % (title, time.time() - t0), flush=True)
                    last = time.time()
                continue
            if raw is None:
                break
            for line in raw.decode('utf-8', 'replace').replace('\r', '\n').split('\n'):
                if not line.strip():
                    continue
                lf.write(line + '\n')
                n += 1
                if n <= echo_lines or PROGRESS.search(line):
                    print('    | ' + line[:220], flush=True)
                elif n == echo_lines + 1:
                    print('    | (이하 생략 — 진행 줄만 표시 · 전문 = %s)' % log_path, flush=True)
            lf.flush()
            last = time.time()
        rc = p.wait()
        dt = time.time() - t0
        lf.write('\n# 종료코드 %d · %.1fs\n' % (rc, dt))
    print('  -> 종료코드 %d · %.1fs' % (rc, dt))
    return rc, dt


# ───────────────────────────────────────────────────────────── 오프셋 워커(하위 프로세스)
def offsets_worker(argv):
    """repin 후보값 기준으로 offsets.py 의 지문을 대조한다(파일 쓰기 = --out 하나뿐)."""
    ap = argparse.ArgumentParser(prog='run_patchday.py _offsets_worker')
    ap.add_argument('--exe', required=True)
    ap.add_argument('--pkl', required=True)
    ap.add_argument('--map', required=True)
    ap.add_argument('--out', required=True)
    ap.add_argument('--base-ver', default=BASE_VER)
    ap.add_argument('--fncheck', default=None, help='repin_fncheck.json — 후보가 WEAK/CLONE 이면 행에 표시')
    ap.add_argument('mods', nargs='*')
    a = ap.parse_args(argv)
    sys.path.insert(0, MIGD)
    import mig_verify as MV  # noqa: E402
    import offsets as OF     # noqa: E402  fingerprint / shift_pairs / fp_path (snap·check 는 부르지 않는다)
    from repin import Img    # noqa: E402
    img = Img(a.exe, a.pkl)
    mp = json.load(open(a.map, encoding='utf-8'))
    fcj = json.load(open(a.fncheck, encoding='utf-8')) if (a.fncheck and os.path.isfile(a.fncheck)) else {}
    res, nshift = {}, 0
    for mod in (a.mods or list(MV.MODS)):
        p = OF.fp_path(mod)
        if not os.path.isfile(p):
            res[mod] = {'status': 'nobaseline', 'rows': []}
            print('%-26s !기준선 지문 없음(offsets\\%s.json)' % (mod, mod))
            continue
        doc = json.load(open(p, encoding='utf-8'))
        man = MV.load_man(mod) or {'entries': []}
        cur = {e['name']: e for e in man['entries'] if not e.get('ignore')}
        # apply 전이면 매니페스트 value == 기준선 키(구 RVA) — 값으로 먼저 잇는다. 이름만으로 이으면 같은 이름이
        # 둘인 엔트리(serpen SPAWN_HOOKS 0xb073c0·0xb080f0)가 한쪽으로 겹쳐 엉뚱한 지문을 대조한다(2026-10-27 리허설).
        byval = {e['value'].lower(): e for e in man['entries'] if not e.get('ignore')}
        mrows = mp.get(mod, {})
        rows = []
        for oldkey, rec in doc.get('fns', {}).items():
            e = byval.get(oldkey.lower()) or cur.get(rec['name'])
            if not e:
                rows.append({'fn': rec['name'], 'old': oldkey, 'note': '매니페스트에서 이름이 사라짐'})
                continue
            r = mrows.get(e['value'])
            if r is None:
                nv, how = e['value'], 'PASS(바이트 그대로)'
            else:
                nv = r.get('new_aligned') or r.get('new')
                how = r.get('kind', '?') + ('/aligned' if r.get('new_aligned') else '')
                if nv is None:
                    rows.append({'fn': rec['name'], 'old': e['value'], 'note': '재핀 실패 — 지문 대조 불가'})
                    continue
            v = int(nv, 16)
            if v not in img.fn:
                rows.append({'fn': rec['name'], 'old': e['value'], 'new': nv, 'how': how,
                             'note': '재핀 후보값이 함수시작 아님'})
                continue
            now_fp = OF.fingerprint(img, v)
            o, n, best = OF.shift_pairs(rec['offs'], now_fp)
            if not o and not n:
                continue
            delta, pairs = (best[0], best[1]) if best else (None, [])
            fv = (fcj.get(mod, {}).get(e['value']) or {}).get('verdict')
            note = None
            if fv in ('WEAK', 'CLONE'):
                note = '⚠재핀 후보가 %s — 이 행은 엉뚱한 함수를 본 것일 수 있다(재핀 판정 후 다시)' % fv
            rows.append({'fn': rec['name'], 'old': e['value'], 'new': nv, 'how': how,
                         'delta': delta, 'npairs': len(pairs),
                         'pairs': [[hex(x), hex(y)] for x, y in pairs][:16],
                         'lost': [hex(x) for x in o][:12], 'born': [hex(x) for x in n][:12],
                         'cand': fv, 'note': note})
        st = 'shift' if rows else 'match'
        res[mod] = {'status': st, 'baseline_ver': doc.get('game_ver'), 'rows': rows}
        bv = doc.get('game_ver')
        tag = '' if bv == a.base_ver else '  ⚠기준선 버전 %s(≠%s)' % (bv, a.base_ver)
        if rows:
            nshift += len(rows)
            print('%-26s ★오프셋 변화 %d개 함수%s' % (mod, len(rows), tag))
            for r in rows:
                d = ('Δ%+#x x%d' % (r['delta'], r['npairs'])) if r.get('delta') is not None else '-'
                det = ('사라짐 %s / 생김 %s' % (r['lost'][:8], r['born'][:8])) if 'lost' in r else ''
                print('    %-26s %-12s %s' % (r['fn'], d, ' '.join(x for x in (r.get('note'), det) if x)))
        else:
            print('%-26s 오프셋 지문 일치%s' % (mod, tag))
    io.open(a.out, 'w', encoding='utf-8', newline='\n').write(json.dumps(res, ensure_ascii=False, indent=1))
    print('-> %s (재핀 **후보값** 기준 — apply 뒤 정식 offsets.py check/sources 는 별도)' % a.out)
    return 1 if nshift else 0


# ───────────────────────────────────────────────────────────── 컨텍스트
class Ctx:
    pass


def read_base_version(new_exe, game_exe):
    cands = [os.path.join(os.path.dirname(new_exe), 'mod-sdk-stable', 'base_version.txt')]
    if os.path.normcase(os.path.abspath(new_exe)) == os.path.normcase(os.path.abspath(game_exe)):
        cands.append(os.path.join(os.path.dirname(game_exe), 'mod-sdk-stable', 'base_version.txt'))
    for c in cands:
        if os.path.isfile(c):
            return io.open(c, encoding='utf-8-sig').read().strip(), c
    return None, None


def decide_label(given, bv, new_sha, backup_sha=None):
    """(라벨, 핫픽스 여부, 사유). 라벨 None = 결정 불가(종료코드 2).
    핫픽스 = 버전 문자열이 기준과 같은데 sha 가 다름, 또는 tfm2_<버전>\\ 백업이 이미 있는데 sha 가 다름."""
    if given:
        lab = given
        hot = False
    else:
        if not bv:
            return None, False, 'base_version.txt 를 찾지 못했다(설치본이 아닌 exe) — --label 을 줘야 한다'
        lab, hot = bv, False
        if backup_sha is None:
            prev = os.path.join(TFM2, 'tfm2_' + bv, 'TeamfightManager2.exe')
            backup_sha = sha256_file(prev) if os.path.isfile(prev) else ''
        if bv == BASE_VER or (backup_sha and backup_sha != new_sha):
            lab, hot = '%s_%s' % (bv, new_sha[:8].lower()), True
    if not re.match(r'^[0-9A-Za-z._-]+$', lab):
        return None, False, '라벨에 쓸 수 없는 문자: %r' % lab
    return lab, hot, ''


def safe_tok(label):
    return re.sub(r'[^0-9A-Za-z_]', '_', label.replace('.', ''))


def stage_rec(name, status, rc=None, dt=0.0, note='', log=None, outputs=None):
    return {'stage': name, 'no': STAGES.index(name), 'status': status, 'rc': rc, 'sec': round(dt, 1),
            'note': note, 'log': log, 'outputs': outputs or [], 'at': now()}


def logp(C, name):
    return os.path.join(C.logs, '%02d_%s.log' % (STAGES.index(name), name))


# ───────────────────────────────────────────────────────────── 단계 1~9
def st_backup(C):
    t0 = time.time()
    if C.rehearse:
        return stage_rec('backup', 'SKIP', note='--rehearse')
    dst = os.path.join(TFM2, 'tfm2_' + C.label)
    src_dir = os.path.dirname(C.new_exe)
    lines, ok, outs = [], True, []
    os.makedirs(dst, exist_ok=True)
    for fn in ('TeamfightManager2.exe', 'bundle.game_data'):
        s = os.path.join(src_dir, fn)
        d = os.path.join(dst, fn)
        if not os.path.isfile(s):
            lines.append('%s: 원본 없음(%s)' % (fn, s))
            ok = ok and fn != 'TeamfightManager2.exe'
            continue
        ssha = C.new_sha if fn == 'TeamfightManager2.exe' else sha256_file(s)
        if os.path.isfile(d):
            dsha = sha256_file(d)
            if dsha == ssha:
                lines.append('%s: 이미 있음·sha 일치 → 건너뜀 (%s)' % (fn, ssha[:16]))
                outs.append(d)
            else:
                lines.append('%s: ★이미 있는데 sha 다름(%s ≠ %s) → 덮어쓰지 않음. 사람이 확인할 것' % (fn, dsha[:16], ssha[:16]))
                ok = False
            continue
        print('    복사 %s (%.1f MB)…' % (fn, os.path.getsize(s) / 1e6), flush=True)
        part = d + '.part'
        shutil.copyfile(s, part)
        psha = sha256_file(part)
        if psha != ssha:
            lines.append('%s: ★복사본 sha 불일치(%s ≠ %s) — 원본이 복사 중 바뀌었을 수 있음. .part 남김' % (fn, psha[:16], ssha[:16]))
            ok = False
            continue
        os.replace(part, d)
        lines.append('%s: 복사 완료 %d B · sha %s' % (fn, os.path.getsize(d), psha[:16]))
        outs.append(d)
    io.open(logp(C, 'backup'), 'w', encoding='utf-8').write('\n'.join(['# %s backup → %s' % (now(), dst)] + lines) + '\n')
    for l in lines:
        print('    ' + l)
    bexe = os.path.join(dst, 'TeamfightManager2.exe')
    if ok and os.path.isfile(bexe) and sha256_file(bexe) == C.new_sha:
        C.new_exe_used = bexe            # 이후 단계는 백업본을 신 exe 로(스팀이 설치본을 또 바꿔도 안전)
    return stage_rec('backup', 'OK' if ok else 'FAIL', 0 if ok else 1, time.time() - t0,
                     ' / '.join(lines), logp(C, 'backup'), outs)


def st_fnindex(C):
    t0 = time.time()
    lg = logp(C, 'fnindex')
    notes = []
    okb, why, _ = verify_fnidx(BASE_PKL, BASE_EXE, BASE_SHA) if os.path.isfile(BASE_PKL) else (False, '없음', None)
    notes.append('기준 %s: %s' % (os.path.basename(BASE_PKL), ('검증 통과 — ' + why) if okb else ('★' + why)))
    if not okb:
        C.base_pkl_bad = True
    target = os.path.join(ROOT, '_fnidx_%s.pkl' % C.tok)
    if os.path.isfile(target):
        ok, why, _ = verify_fnidx(target, C.new_exe_used, C.new_sha)
        if ok:
            C.new_pkl = target
            notes.append('신 %s 이미 있음 → 검증 통과(%s) · 건너뜀' % (os.path.basename(target), why))
            io.open(lg, 'w', encoding='utf-8').write('# %s fnindex\n%s\n' % (now(), '\n'.join(notes)))
            for nn in notes:
                print('    ' + nn)
            return stage_rec('fnindex', 'CACHED' if okb else 'FAIL', 0, time.time() - t0, ' / '.join(notes), lg, [target])
        notes.append('★기존 %s 가 이 exe 와 불일치(%s) → 덮어쓰지 않고 출력 폴더에 새로 만든다' % (os.path.basename(target), why))
        target = os.path.join(C.out, '_fnidx_%s.rebuilt.pkl' % C.tok)
    for nn in notes:
        print('    ' + nn)
    rc, dt = run_cmd([PY, '-X', 'utf8', FNINDEX, C.new_exe_used, target], lg, 'fnindex')
    with io.open(lg, 'a', encoding='utf-8') as f:
        f.write('\n'.join(notes) + '\n')
    if rc == 0 and os.path.isfile(target):
        ok, why, _ = verify_fnidx(target, C.new_exe_used, C.new_sha)
        C.new_pkl = target if ok else None
        notes.append('생성 후 검증: %s' % (why if ok else '★' + why))
        st = 'OK' if (ok and okb) else 'FAIL'
        if ok and okb and 'rebuilt' in target:
            st = 'REVIEW'
        return stage_rec('fnindex', st, rc, time.time() - t0, ' / '.join(notes), lg, [target])
    return stage_rec('fnindex', 'FAIL', rc, time.time() - t0, ' / '.join(notes), lg)


def st_callgraph(C):
    t0 = time.time()
    lg = logp(C, 'callgraph')
    if not C.new_pkl:
        return stage_rec('callgraph', 'NOINPUT', note='신 fnidx pkl 없음')
    notes = []
    base_idx = None
    if os.path.isfile(BASE_PKL):
        P = pickle.load(open(BASE_PKL, 'rb'))
        base_idx = {(int(k, 16) if isinstance(k, str) else k): v for k, v in P['idx'].items()}
    okb, why = verify_cg(BASE_CG, BASE_EXE, base_idx) if (base_idx and os.path.isfile(BASE_CG)) else (False, '없음')
    notes.append('기준 %s: %s' % (os.path.basename(BASE_CG), ('검증 통과 — ' + why) if okb else ('★' + why)))
    P = pickle.load(open(C.new_pkl, 'rb'))
    idx = {(int(k, 16) if isinstance(k, str) else k): v for k, v in P['idx'].items()}
    in_out = os.path.dirname(os.path.abspath(C.new_pkl)) == os.path.abspath(C.out)
    target = os.path.join(C.out if in_out else ROOT, '_cg_%s%s.pkl' % (C.tok, '.rebuilt' if in_out else ''))
    if os.path.isfile(target):
        ok, why = verify_cg(target, C.new_exe_used, idx)
        if ok:
            C.new_cg = target
            notes.append('신 %s 이미 있음 → 검증 통과(%s) · 건너뜀' % (os.path.basename(target), why))
            io.open(lg, 'w', encoding='utf-8').write('# %s callgraph\n%s\n' % (now(), '\n'.join(notes)))
            for nn in notes:
                print('    ' + nn)
            return stage_rec('callgraph', 'CACHED' if okb else 'FAIL', 0, time.time() - t0, ' / '.join(notes), lg, [target])
        notes.append('★기존 %s 불일치(%s) → 출력 폴더에 새로 만든다' % (os.path.basename(target), why))
        target = os.path.join(C.out, '_cg_%s.rebuilt.pkl' % C.tok)
    for nn in notes:
        print('    ' + nn)
    rc, dt = run_cmd([PY, '-X', 'utf8', CALLGRAPH, C.new_exe_used, C.new_pkl, target], lg, 'callgraph')
    with io.open(lg, 'a', encoding='utf-8') as f:
        f.write('\n'.join(notes) + '\n')
    if rc == 0 and os.path.isfile(target):
        ok, why = verify_cg(target, C.new_exe_used, idx)
        C.new_cg = target if ok else None
        notes.append('생성 후 검증: %s' % (why if ok else '★' + why))
        return stage_rec('callgraph', 'OK' if (ok and okb) else 'FAIL', rc, time.time() - t0, ' / '.join(notes), lg, [target])
    return stage_rec('callgraph', 'FAIL', rc, time.time() - t0, ' / '.join(notes), lg)


def st_aidiff(C):
    lg = logp(C, 'aidiff')
    out = os.path.join(C.out, 'aidiff.md')
    rc, dt = run_cmd([PY, '-X', 'utf8', AIDIFF, BASE_EXE, C.new_exe_used, '-o', out], lg, 'aidiff')
    if rc != 0 or not os.path.isfile(out):
        return stage_rec('aidiff', 'FAIL', rc, dt, 'aidiff 실패', lg)
    a = parse_aidiff(out)
    return stage_rec('aidiff', 'REVIEW' if a['edited'] else 'OK', rc, dt,
                     'EDITED %d · MERGED %d · 사라짐 %s · 새로 생김 %s' % (len(a['edited']), a['merged'], a['gone'], a['added']),
                     lg, [out])


def st_check(C):
    lg = logp(C, 'check')
    rc, dt = run_cmd([PY, '-X', 'utf8', MIGVERIFY, 'check'] + C.mods + ['--exe', C.new_exe_used], lg, 'check', echo_lines=15)
    if rc not in (0, 1):
        return stage_rec('check', 'FAIL', rc, dt, 'mig_verify check 비정상 종료', lg)
    p = parse_check(lg)
    tot = sum(v['stale'] for v in p.values())
    return stage_rec('check', 'REVIEW' if rc == 1 else 'OK', rc, dt,
                     'STALE·INVALID %d (모드 %d개 중 %d개)' % (tot, len(p), sum(1 for v in p.values() if v['stale'])),
                     lg)


def st_repin(C):
    if not (C.new_pkl and os.path.isfile(BASE_PKL)):
        return stage_rec('repin', 'NOINPUT', note='fnidx pkl 없음')
    lg = logp(C, 'repin')
    mp = os.path.join(C.out, 'repin_map.json')
    al = os.path.join(C.out, 'repin_map.aligned.json')
    args = [PY, '-X', 'utf8', REPIN, 'plan'] + C.mods + ['--old', BASE_EXE, '--new', C.new_exe_used,
                                                         '--oldpkl', BASE_PKL, '--newpkl', C.new_pkl, '--out', mp]
    cgargs = []
    if os.path.isfile(BASE_CG) and C.new_cg:
        cgargs = ['--oldcg', BASE_CG, '--newcg', C.new_cg]
    rc, dt = run_cmd(args + cgargs, lg, 'repin plan', echo_lines=20)
    if rc != 0 or not os.path.isfile(mp):
        return stage_rec('repin', 'FAIL', rc, dt, 'repin plan 실패', lg)
    lg2 = os.path.join(C.logs, '%02d_%s_sitealign.log' % (STAGES.index('repin'), 'repin'))
    rc2, dt2 = run_cmd([PY, '-X', 'utf8', SITEALIGN, '--map', mp, '--old', BASE_EXE, '--new', C.new_exe_used,
                        '--oldpkl', BASE_PKL, '--newpkl', C.new_pkl] + cgargs + ['--write', al], lg2, 'sitealign',
                       echo_lines=12)
    use = al if (rc2 == 0 and os.path.isfile(al)) else mp
    fc = os.path.join(C.out, 'repin_fncheck.json')
    lg3 = os.path.join(C.logs, '%02d_%s_fncheck.log' % (STAGES.index('repin'), 'repin'))
    t3 = time.time()
    try:
        print('  함수시작 교차검증(콜러 수·callee 사영·size 비 2/3 + 동형 클론)…', flush=True)
        fcs = repin_fncheck(use, BASE_PKL, C.new_pkl, BASE_CG if cgargs else None, C.new_cg if cgargs else None, fc)
        msg = '함수시작 %d · 약함(WEAK) %d · 동형 클론 %d · %.1fs' % (fcs['n'], fcs['weak'], fcs['clone'], time.time() - t3)
    except Exception as e:
        import traceback
        msg = '★교차검증 실패: %s\n%s' % (e, traceback.format_exc())
    io.open(lg3, 'w', encoding='utf-8').write('# %s fncheck\n%s\n' % (now(), msg))
    print('    ' + msg.split('\n')[0])
    s = summarize_repin(use, fc if os.path.isfile(fc) else None)
    ra, rg = split_gen(s['residual'])
    note = '대상 %d · 엔진 해결 %d · 실패 %d · 사람 판정 필요 %d(apply 대상 %d + 생성물 모드 %d) · 동형 클론 주의 %d' % (
        s['n'], s['resolved'], s['failed'], len(s['residual']), len(ra), len(rg), len(s['clones']))
    if rc2 != 0 or not os.path.isfile(al):
        return stage_rec('repin', 'FAIL', rc2, dt + dt2, note + ' · ★sitealign 실패', lg, [mp])
    return stage_rec('repin', 'REVIEW' if s['residual'] else 'OK', rc2, time.time() - t3 + dt + dt2, note, lg,
                     [mp, al, fc])


def repin_fncheck(map_path, opkl, npkl, ocg, ncg, out):
    """함수시작 엔트리의 재핀 후보를 독립 증거 3종으로 채점한다(MIG\\README 0.6.0 절 「함수시작 채택 규칙」).
    ① 콜러 수 일치(±10%) ② callee 사영(skel 유일쌍 기준맵으로 구 callee 를 옮겨 신 callee 와 겹침 ≥0.6) ③ size 비 0.85~1.18.
    skel 완전일치면 강함. 아니면 2/3 미만 = WEAK(사람 판정). 신 함수와 같은 skel 이 신 exe 에 여럿 = 동형 클론(콜러 문맥 확인).
    근거(2026-10-27 리허설 0.6.5→0.6.4): 사람 확정 0.6.5 final 맵 역대응 137건 중 엔진 오답 2건 —
    ORACLE_RVA(BYTES_UNIQUE 32B) = 표 0/3 → WEAK 로 잡힘 · ui_kit RVA_DROP_PLAN = skel 같은 클론 2개 → CLONE 으로 잡힘."""
    m = json.load(open(map_path, encoding='utf-8'))
    O = pickle.load(open(opkl, 'rb'))
    N = pickle.load(open(npkl, 'rb'))
    gO = pickle.load(open(ocg, 'rb')) if ocg else {'caller': {}, 'callee': {}}
    gN = pickle.load(open(ncg, 'rb')) if ncg else {'caller': {}, 'callee': {}}
    uo = {s: v[0] for s, v in O['by_skel'].items() if len(v) == 1}
    un = {s: v[0] for s, v in N['by_skel'].items() if len(v) == 1}
    gm = {uo[s]: un[s] for s in uo if s in un}
    res, n, weak, clone = {}, 0, 0, 0
    for mod, ents in m.items():
        for old, e in ents.items():
            nv = e.get('new_aligned') or e.get('new')
            o = int(old, 16)
            if not nv or o not in O['idx']:
                continue
            v = int(nv, 16)
            fo, fn = O['idx'][o], N['idx'].get(v)
            r = {'name': e.get('name'), 'old': old, 'new': nv, 'kind': e.get('kind')}
            n += 1
            if fn is None:
                r.update(verdict='WEAK', why='후보가 신 함수시작 아님')
                weak += 1
                res.setdefault(mod, {})[old] = r
                continue
            co, cn = len(gO['caller'].get(o, [])), len(gN['caller'].get(v, []))
            cc = co == cn or (max(co, cn) >= 4 and abs(co - cn) <= max(1, round(0.1 * max(co, cn))))
            po = {gm[c] for c in gO['callee'].get(o, []) if c in gm}
            pr = (len(po & set(gN['callee'].get(v, []))) / float(len(po))) if po else None
            sr = fn['size'] / float(fo['size'])
            votes = int(cc) + int(pr is not None and pr >= 0.6) + int(0.85 <= sr <= 1.18)
            sk = fo['skel'] == fn['skel']
            ncl = len(N['by_skel'].get(fn['skel'], []))
            r.update(skel_same=sk, callers=[co, cn], callee_proj=None if pr is None else round(pr, 2),
                     size_ratio=round(sr, 3), votes=votes, clones=ncl)
            if not sk and votes < 2:
                r['verdict'] = 'WEAK'
                weak += 1
            elif ncl > 1:
                r['verdict'] = 'CLONE'
                clone += 1
            else:
                r['verdict'] = 'STRONG'
            res.setdefault(mod, {})[old] = r
    io.open(out, 'w', encoding='utf-8', newline='\n').write(json.dumps(res, ensure_ascii=False, indent=1))
    return {'n': n, 'weak': weak, 'clone': clone}


def st_offsets(C):
    al = os.path.join(C.out, 'repin_map.aligned.json')
    mp = al if os.path.isfile(al) else os.path.join(C.out, 'repin_map.json')
    if not (C.new_pkl and os.path.isfile(mp)):
        return stage_rec('offsets', 'NOINPUT', note='신 pkl 또는 repin_map 없음(repin 단계 필요)')
    lg = logp(C, 'offsets')
    out = os.path.join(C.out, 'offsets_shifts.json')
    rc, dt = run_cmd([PY, '-X', 'utf8', os.path.abspath(__file__), '_offsets_worker', '--exe', C.new_exe_used,
                      '--pkl', C.new_pkl, '--map', mp, '--out', out,
                      '--fncheck', os.path.join(C.out, 'repin_fncheck.json')] + C.mods, lg, 'offsets', echo_lines=60)
    if rc not in (0, 1) or not os.path.isfile(out):
        return stage_rec('offsets', 'FAIL', rc, dt, '오프셋 워커 실패', lg)
    d = json.load(open(out, encoding='utf-8'))
    nmod = sum(1 for v in d.values() if v['status'] == 'shift')
    nfn = sum(len(v['rows']) for v in d.values())
    return stage_rec('offsets', 'REVIEW' if rc == 1 else 'OK', rc, dt,
                     '변화 %d개 모드 · %d개 함수 (기준 = %s)' % (nmod, nfn, os.path.basename(mp)), lg, [out])


def _contract_stage(C, name, tool, extra_need, args_fn, outdir, count_fn=None):
    for p, what in [(tool, '도구')] + extra_need:
        if not os.path.isfile(p):
            return stage_rec(name, 'NOTOOL', note='%s 없음: %s' % (what, p))
    if not (C.new_pkl and C.new_cg and os.path.isfile(BASE_PKL) and os.path.isfile(BASE_CG)):
        return stage_rec(name, 'NOINPUT', note='pkl/cg 없음')
    os.makedirs(outdir, exist_ok=True)
    lg = logp(C, name)
    rc, dt = run_cmd(args_fn(), lg, name, echo_lines=30)
    st = {0: 'OK', 1: 'REVIEW'}.get(rc, 'FAIL')
    note = {0: '전부 자동 확정', 1: '검토 필요 항목 있음(정상)'}.get(rc, '★실행 오류(종료코드 %d)' % rc)
    if rc in (0, 1) and count_fn:
        try:
            note += ' · ' + count_fn()
        except Exception as e:   # 집계 실패는 단계 결과를 바꾸지 않는다(원문은 산출물)
            note += ' · (산출물 집계 실패: %s)' % e
    return stage_rec(name, st, rc, dt, note, lg, [outdir])


def _count_track(od):
    d = json.load(open(os.path.join(od, 'track.json'), encoding='utf-8'))
    c = d.get('counts', {})
    w = c.get('windows') or {}
    return '전술 AUTO %s · REVIEW %s · 창 %s(자동 %s) · 기본값 변경 전술 %d' % (
        c.get('auto', '?'), c.get('review', '?'), sum(w.values()) if isinstance(w, dict) else '?',
        c.get('windows_auto', '?'), sum(1 for t in d.get('tactics', []) if t.get('value_changes')))


def _count_addrmap(od):
    s = json.load(open(os.path.join(od, 'addrmap.json'), encoding='utf-8')).get('summary', {})
    return '%s행 · 자동 %s(%.1f%%) · 검토 %s · 행 오류 %s' % (
        s.get('rows', '?'), s.get('auto', '?'), 100.0 * float(s.get('auto_rate') or 0), s.get('review', '?'),
        s.get('row_errors', '?') if not isinstance(s.get('row_errors'), list) else len(s['row_errors']))


def common_args(C):
    return ['--old-exe', BASE_EXE, '--new-exe', C.new_exe_used, '--new-ver', C.label,
            '--old-pkl', BASE_PKL, '--new-pkl', C.new_pkl, '--old-cg', BASE_CG, '--new-cg', C.new_cg]


def st_detail(C):
    od = os.path.join(C.out, 'detail_tactics')
    return _contract_stage(C, 'detail_tactics', TRACK, [(TRACK_BASE, '기준선')],
                           lambda: [PY, '-X', 'utf8', TRACK, 'track', '--base', TRACK_BASE] + common_args(C) + ['--out', od],
                           od, lambda: _count_track(od))


def st_addrbook(C):
    od = os.path.join(C.out, 'addrmap')
    if os.path.isfile(ADDRBOOK) and os.path.isfile(ADDRBOOK_JSON):
        print('    (주소록 2.4만 행 — 2026-10-27 리허설 실측 1673s · 메모리 약 9GB. 먼저 결과를 보려면 --skip addrbook 뒤 --only addrbook)',
              flush=True)
    return _contract_stage(C, 'addrbook', ADDRBOOK, [(ADDRBOOK_JSON, '주소록')],
                           lambda: [PY, '-X', 'utf8', ADDRBOOK, 'map', '--book', ADDRBOOK_JSON] + common_args(C) + ['--out', od],
                           od, lambda: _count_addrmap(od))


# ───────────────────────────────────────────────────────────── 결과 파싱(요약용 — 전부 파일에서 다시 읽는다)
def parse_aidiff(md):
    t = io.open(md, encoding='utf-8', errors='replace').read()
    ed = re.findall(r'^### (.+?) — `(0x[0-9a-f]+)` → `(0x[0-9a-f]+)`\s+\*\*EDITED\*\*', t, re.M)
    mg = re.findall(r'^### .+?\*\*MERGED\*\*', t, re.M)
    gone = re.search(r'사라짐 = (\d+)개', t)
    added = re.search(r'새로 생김 = (\d+)개', t)
    sh = re.search(r'SHIFTED\) = (\d+)개 · 소스 무변경\(IDENTICAL\) = (\d+)개', t)
    return {'edited': ed, 'merged': len(mg), 'gone': gone.group(1) if gone else '?',
            'added': added.group(1) if added else '?',
            'shifted': sh.group(1) if sh else '?', 'identical': sh.group(2) if sh else '?'}


def parse_check(log):
    res = collections.OrderedDict()
    cur = None
    for l in io.open(log, encoding='utf-8', errors='replace'):
        m = re.match(r'^(\S+)\s+PASS\s+(\d+) / STALE·INVALID\s+(\d+)', l)
        if m:
            cur = m.group(1)
            res[cur] = {'pass': int(m.group(2)), 'stale': int(m.group(3)), 'invalid': 0}
            continue
        m = re.match(r'^(\S+)\s+!매니페스트 없음', l)
        if m:
            res[m.group(1)] = {'pass': 0, 'stale': 0, 'invalid': 0, 'noman': True}
            continue
        if cur and l.startswith('    INVALID'):
            res[cur]['invalid'] += 1
    return res


def summarize_repin(path, fncheck=None):
    m = json.load(open(path, encoding='utf-8'))
    fc = json.load(open(fncheck, encoding='utf-8')) if fncheck else {}
    s = {'n': 0, 'resolved': 0, 'failed': 0, 'residual': [], 'clones': [], 'per': collections.OrderedDict(),
         'kinds': collections.Counter(), 'align': collections.Counter(), 'fn': collections.Counter(),
         'path': path, 'fncheck': bool(fncheck)}
    for mod, ents in m.items():
        pm = {'n': 0, 'resolved': 0, 'failed': 0, 'residual': 0, 'agree': 0, 'weak': 0, 'clone': 0}
        for old, e in ents.items():
            pm['n'] += 1
            kind = (e.get('kind') or '?').split('(')[0]
            s['kinds'][kind] += 1
            al = (e.get('align') or {}).get('verdict')
            if al:
                s['align'][al] += 1
            if e.get('new'):
                pm['resolved'] += 1
            else:
                pm['failed'] += 1
            fr = fc.get(mod, {}).get(old)
            fv = fr.get('verdict') if fr else None
            if fv:
                s['fn'][fv] += 1
            need = (not e.get('new')) or (al and not al.startswith('AGREE')) or fv == 'WEAK'
            if al == 'AGREE':
                pm['agree'] += 1
            if fv == 'WEAK':
                pm['weak'] += 1
            if need:
                pm['residual'] += 1
                why = al or kind
                if fv == 'WEAK':
                    why = 'FN_WEAK(%s · 콜러 %s · 사영 %s · size %s)' % (kind, fr.get('callers'), fr.get('callee_proj'),
                                                                     fr.get('size_ratio')) if 'votes' in fr else 'FN_WEAK(%s)' % fr.get('why')
                s['residual'].append((mod, e.get('name'), old, e.get('new'), e.get('new_aligned'), why))
            elif fv == 'CLONE':
                pm['clone'] += 1
                s['clones'].append((mod, e.get('name'), old, e.get('new_aligned') or e.get('new'),
                                    '%s · 같은 skel %d개' % (kind, fr.get('clones', 0))))
        s['per'][mod] = pm
        s['n'] += pm['n']
        s['resolved'] += pm['resolved']
        s['failed'] += pm['failed']
    return s


def split_gen(rows):
    """(apply 대상, 생성물 모드) — rows 의 첫 칸 = 모드."""
    return [x for x in rows if x[0] not in GEN_MODS], [x for x in rows if x[0] in GEN_MODS]


def _pick(d, keys, default=None):
    for k in keys:
        if isinstance(d, dict) and k in d and d[k] not in (None, ''):
            return d[k]
    return default


def _items(doc, keys=('tactics', 'items', 'rows', 'results', 'entries', 'sites', 'book', 'map')):
    if isinstance(doc, list):
        return doc
    if not isinstance(doc, dict):
        return []
    for k in keys:
        v = doc.get(k)
        if isinstance(v, list):
            return v
        if isinstance(v, dict) and v and all(isinstance(x, dict) for x in v.values()):
            return [dict(x, _key=kk) for kk, x in v.items()]
    return []


VERDICT_KEYS = ('verdict', 'grade', 'status', 'class', 'category', 'kind', '판정', '등급')
NAME_KEYS = ('id', 'tactic', 'name', 'key', '_key', 'label', 'title')


CODE_KO = collections.OrderedDict([
    ('SAME', '이동만(자동확정)'), ('REG_RENAME', '레지스터·스택 재배치'), ('VALUE_CHANGED', '게임 기본값 변경'),
    ('COPIES', '사본 분기'), ('LOGIC_CHANGED', '로직 변경'), ('SITE_MISSING', '창 소실'), ('OWNER_MISSING', '함수 소실')])
CODE_RANK = {c: i for i, c in enumerate(['OWNER_MISSING', 'SITE_MISSING', 'LOGIC_CHANGED', 'COPIES',
                                         'VALUE_CHANGED', 'REG_RENAME', 'SAME'])}


def summarize_track(path):
    """track.json(track_update.py v1 스키마: counts·tactics[verdict/worst/value_changes/reasons]) 요약.
    스키마가 다르면 느슨한 일반 해석으로 떨어진다."""
    doc = json.load(open(path, encoding='utf-8'))
    tac = doc.get('tactics') if isinstance(doc, dict) else None
    if isinstance(tac, list) and tac and isinstance(tac[0], dict) and 'worst' in tac[0]:
        worst = collections.Counter(t.get('worst', '?') for t in tac)
        verd = collections.Counter(t.get('verdict', '?') for t in tac)
        dflt = [t for t in tac if t.get('value_changes')]
        rev = sorted([t for t in tac if t.get('verdict') != 'AUTO'],
                     key=lambda t: (CODE_RANK.get(t.get('worst'), 9), t.get('index', 0)))
        return {'schema': 'v1', 'n': len(tac), 'verdict': verd, 'worst': worst, 'default': dflt, 'review': rev,
                'counts': doc.get('counts', {}), 'old_ver': doc.get('old_ver'), 'new_ver': doc.get('new_ver'),
                'new_sha': doc.get('new_sha256')}
    items = _items(doc)
    cnt = collections.Counter()
    dflt, review = [], []
    for it in items:
        if not isinstance(it, dict):
            continue
        v = str(_pick(it, VERDICT_KEYS, '?'))
        nm = str(_pick(it, NAME_KEYS, '?'))
        cnt[v] += 1
        txt = v.upper()
        dc = _pick(it, ('default_changed', 'defaults_changed', 'game_default_changed'))
        if dc or 'DEFAULT' in txt or '기본값' in v:
            dflt.append((nm, v, it))
        rv = _pick(it, ('review', 'needs_review', 'need_review', 'manual'))
        if rv or any(w in txt for w in ('REVIEW', 'LOGIC', 'LOST', 'GONE', 'FORK', 'SPLIT', 'MANUAL', 'FAIL',
                                         '검토', '로직', '소실', '사본', '분기')):
            review.append((nm, v, it))
    summ = doc.get('summary') or doc.get('counts') if isinstance(doc, dict) else None
    return {'n': len(items), 'cnt': cnt, 'default': dflt, 'review': review, 'raw_summary': summ,
            'meta': {k: doc[k] for k in ('old_ver', 'new_ver', 'old_sha', 'new_sha', 'version')
                     if isinstance(doc, dict) and k in doc}}


def _brief(it, skip=NAME_KEYS + VERDICT_KEYS):
    parts = []
    for k, v in it.items():
        if k in skip or k.startswith('_') or isinstance(v, (dict, list)):
            continue
        parts.append('%s=%s' % (k, str(v)[:40]))
        if len(parts) >= 4:
            break
    return ' · '.join(parts)


def summarize_addrmap(d):
    files = sorted(glob.glob(os.path.join(d, '*.json')))
    for f in files:
        try:
            doc = json.load(open(f, encoding='utf-8'))
        except Exception:
            continue
        items = _items(doc)
        if not items:
            continue
        cnt = collections.Counter(str(_pick(it, VERDICT_KEYS, '?')) for it in items if isinstance(it, dict))
        return {'file': f, 'n': len(items), 'cnt': cnt,
                'raw_summary': (doc.get('summary') or doc.get('counts')) if isinstance(doc, dict) else None}
    return None


def render_generic_table(doc, limit=40):
    items = _items(doc, keys=('rows', 'items', 'mods', 'entries', 'exposure', 'table', 'results'))
    if not items:
        if isinstance(doc, dict):
            L = ['| 키 | 값 |', '|---|---|']
            for k, v in list(doc.items())[:limit]:
                L.append('| %s | %s |' % (k, json.dumps(v, ensure_ascii=False)[:160].replace('|', '\\|')))
            return L
        return ['(형식 해석 불가 — 원문 참조)']
    keys = []
    for it in items[:limit]:
        if isinstance(it, dict):
            for k, v in it.items():
                if k not in keys and not isinstance(v, (dict, list)):
                    keys.append(k)
    keys = keys[:7]
    L = ['| ' + ' | '.join(keys) + ' |', '|' + '---|' * len(keys)]
    for it in items[:limit]:
        if isinstance(it, dict):
            L.append('| ' + ' | '.join(str(it.get(k, '')).replace('|', '\\|')[:60] for k in keys) + ' |')
    if len(items) > limit:
        L.append('')
        L.append('… 외 %d행 (원문 참조)' % (len(items) - limit))
    return L


# ───────────────────────────────────────────────────────────── SUMMARY.md
def write_summary(C, recs, warns, total):
    o = []
    w = o.append
    title = '패치날 요약 — %s%s' % (C.label, ' (리허설)' if C.rehearse else '')
    w('# ' + title)
    w('')
    w('> 생성 %s · `MIG\\patchday\\run_patchday.py` · 전체 소요 %.0fs · 출력 = `%s`' % (now(), total, C.out))
    w('')
    if warns:
        w('## ⚠⚠ 비파괴 경고 — 출력 폴더·pkl 밖에서 바뀐 것이 있다')
        w('')
        w('이 실행기가 부른 명령 때문인지, **같은 시간에 돈 다른 세션** 때문인지 확인할 것(이 실행기는 그런 쓰기를 하지 않게 짜여 있다).')
        w('')
        for x in warns[:60]:
            w('- ' + x)
        if len(warns) > 60:
            w('- … 외 %d건 (run.json 참조)' % (len(warns) - 60))
        w('')
    w('## 0. 버전')
    w('')
    w('| | 기준(구) | 신 |')
    w('|---|---|---|')
    w('| 라벨 | %s | **%s**%s |' % (BASE_VER, C.label, ' (핫픽스 라벨)' if C.hotfix else ''))
    w('| sha256 | `%s` | `%s` |' % (BASE_SHA, C.new_sha))
    w('| 크기 | %d B | %d B |' % (BASE_SIZE, C.new_size))
    w('| exe | `%s` | `%s` |' % (BASE_EXE, C.new_exe_used))
    w('| base_version.txt | — | %s |' % (('`%s` (%s)' % (C.bv, C.bv_path)) if C.bv else
                                          ('없음 — 라벨은 --label 지정' if C.label_given else '읽지 못함')))
    w('')
    w('## 1. 단계 결과')
    w('')
    w('| # | 단계 | 결과 | 종료코드 | 소요 | 메모 | 로그 |')
    w('|---|---|---|---|---|---|---|')
    for r in recs:
        lg = ('[log](%s)' % rel(r['log'], C.out)) if r.get('log') else ''
        prev = ' (이전 실행 %s)' % r['at'] if r.get('_prev') else ''
        w('| %d | %s | %s%s | %s | %ss | %s | %s |' % (r['no'], r['stage'], ST_KO.get(r['status'], r['status']), prev,
                                                     '' if r['rc'] is None else r['rc'], r['sec'],
                                                     str(r.get('note', '')).replace('|', '\\|')[:300], lg))
    w('')

    # AI 판단계층
    w('## 2. AI 판단계층 (aidiff)')
    w('')
    ad = os.path.join(C.out, 'aidiff.md')
    if os.path.isfile(ad):
        a = parse_aidiff(ad)
        w('- ★**EDITED %d개** · MERGED %d · SHIFTED %s · IDENTICAL %s · 사라짐 %s · 새로 생김 %s → 전문 [aidiff.md](aidiff.md)'
          % (len(a['edited']), a['merged'], a['shifted'], a['identical'], a['gone'], a['added']))
        for top, o_, n_ in a['edited'][:40]:
            w('  - `%s` `%s`→`%s`' % (top, o_, n_))
        if len(a['edited']) > 40:
            w('  - … 외 %d개' % (len(a['edited']) - 40))
    else:
        w('- (aidiff 산출물 없음)')
    w('')

    # check
    w('## 3. 매니페스트 STALE (mig_verify check)')
    w('')
    cl = logp(C, 'check')
    if os.path.isfile(cl):
        p = parse_check(cl)
        w('| 모드 | PASS | STALE·INVALID | (INVALID) |')
        w('|---|---|---|---|')
        for mod, v in p.items():
            tag = ' (★MODS 밖 — 명시 추가)' if mod in getattr(C, 'extra', []) else ''
            w('| %s%s | %s | %s | %s |' % (mod, tag, v['pass'], ('**%d**' % v['stale']) if v['stale'] else 0,
                                          v['invalid'] or '' if not v.get('noman') else '매니페스트 없음'))
        w('')
        w('합계 STALE·INVALID = **%d** · 전문 [log](%s)' % (sum(v['stale'] for v in p.values()), rel(cl, C.out)))
    else:
        w('- (check 로그 없음)')
    w('')

    # repin
    w('## 4. 재핀 계획 (repin plan + sitealign) — ★apply 안 함')
    w('')
    al = os.path.join(C.out, 'repin_map.aligned.json')
    mp = al if os.path.isfile(al) else os.path.join(C.out, 'repin_map.json')
    fcp = os.path.join(C.out, 'repin_fncheck.json')
    n_apply_res = '?'
    if os.path.isfile(mp):
        s = summarize_repin(mp, fcp if os.path.isfile(fcp) else None)
        ra, rg = split_gen(s['residual'])
        n_apply_res = len(ra)
        gen_here = [m for m in GEN_MODS if m in s['per']]
        w('- 대상 %d · 엔진 해결 %d · 실패 %d · 사람 판정 필요 %d (실패 + sitealign AGREE 아님 + 함수시작 WEAK) · 동형 클론 주의 %d · 파일 [%s](%s)'
          % (s['n'], s['resolved'], s['failed'], len(s['residual']), len(s['clones']), os.path.basename(mp), os.path.basename(mp)))
        w('  - ★**repin apply 대상 사람 판정 = %d건** (아래 표 전부) · 생성물 모드 %d건(%s)은 **repin 판정 불필요** — sites.rs 가 생성물이라 '
          '6절 상세전술(③ track_update)·④ migrate_targets 가 따로 추적한다(참고용으로만 repin_review.md 뒤쪽에 둔다)'
          % (len(ra), len(rg), '·'.join(gen_here) or '없음'))
        w('- 엔진 판별 종류: %s' % ', '.join('%s=%d' % kv for kv in s['kinds'].most_common()))
        if s['align']:
            w('- sitealign(함수 중간 사이트): %s' % ', '.join('%s=%d' % kv for kv in sorted(s['align'].items())))
        if s['fn']:
            w('- 함수시작 교차검증(콜러 수·callee 사영·size 비 2/3, [repin_fncheck.json](repin_fncheck.json)): %s'
              % ', '.join('%s=%d' % kv for kv in sorted(s['fn'].items())))
        else:
            w('- 함수시작 교차검증 결과 없음 — ⚠`BYTES_UNIQUE` 함수시작도 오답 사례가 있다(MIG\\README 0.6.0 절)')
        w('- ⚠「사람 판정 필요」 밖(AGREE·STRONG)도 **후보**다 — apply 전 MIG\\README 0.6.0 절 규칙으로 최종 확정.')
        w('')
        w('| 모드 | 대상 | 해결 | 실패 | 정렬 AGREE | 함수 WEAK | 클론 | 사람 판정 |')
        w('|---|---|---|---|---|---|---|---|')
        for mod, v in s['per'].items():
            gen = mod in GEN_MODS
            w('| %s%s | %d | %d | %d | %d | %d | %d | %s |' % (mod, ' (생성물 — ③④, apply 안 함)' if gen else '',
                                                             v['n'], v['resolved'], v['failed'], v['agree'], v['weak'], v['clone'],
                                                             (('%d(참고)' if gen else '**%d**') % v['residual']) if v['residual'] else 0))
        if s['clones']:
            w('')
            w('<details><summary>동형 클론 주의 %d건 (같은 skel 의 함수가 신 exe 에 여럿 — 콜러 문맥으로 확인)</summary>' % len(s['clones']))
            w('')
            w('| 모드 | 이름 | 구 | 후보 | 비고 |')
            w('|---|---|---|---|---|')
            for mod, nm, old, new, why in s['clones'][:60]:
                w('| %s | %s | `%s` | %s | %s |' % (mod, nm, old, new, why))
            w('')
            w('</details>')
        row_ = lambda x: '| %s | %s | `%s` | %s | %s | %s |' % (x[0], x[1], x[2], x[3] or '실패', x[4] or '', x[5])
        rv = ['# 재핀 사람 판정 대기열 — %s (구 %s → 신 %s)' % (C.label, BASE_VER, C.label), '',
              '> run_patchday 생성. 판정 결과는 `MIG\\_mkfinal%s.py`(= `_mkfinal065.py` 복제) 의 FIX 표로 → `MIG\\repin_map_%s.final.json`.' % (C.tok, C.tok),
              '> 근거 원문은 REPORT `<MOD>\\RE\\YYYY-MM-DD_%s-재핀판정.md` 로 저장(§11).' % C.label, '',
              '## ★repin apply 대상 — 사람 판정 필요 %d' % len(ra), '', '| 모드 | 이름 | 구 | repin | aligned | 판정 |', '|---|---|---|---|---|---|']
        rv += [row_(x) for x in ra]
        rv += ['', '## 동형 클론 주의 %d' % len(s['clones']), '', '| 모드 | 이름 | 구 | 후보 | 비고 |', '|---|---|---|---|---|']
        rv += ['| %s | %s | `%s` | %s | %s |' % x for x in s['clones']]
        rv += ['', '## (참고) 생성물 모드 %d — repin 판정 불필요' % len(rg), '',
               '> %s 는 sites.rs 생성물이라 `repin apply` 대상이 아니다. 상세전술 = `detail_tactics\\track.md`(README ③), '
               'tactics_verify = `tools\\migrate_targets.py`(README ④) 가 따로 판정한다. 이 표는 교차 확인용.' % '·'.join(GEN_MODS), '',
               '| 모드 | 이름 | 구 | repin | aligned | 판정 |', '|---|---|---|---|---|---|']
        rv += [row_(x) for x in rg]
        io.open(os.path.join(C.out, 'repin_review.md'), 'w', encoding='utf-8', newline='\n').write('\n'.join(rv) + '\n')
        w('- 전 목록 = [repin_review.md](repin_review.md)')
        if ra:
            w('')
            w('<details open><summary>★repin apply 대상 사람 판정 %d건%s</summary>' % (len(ra), '' if len(ra) <= 100 else ' (앞 100)'))
            w('')
            w('| 모드 | 이름 | 구 | repin | aligned | 판정 |')
            w('|---|---|---|---|---|---|')
            for x in ra[:100]:
                w(row_(x))
            w('')
            w('</details>')
        if rg:
            w('')
            w('<details><summary>(참고) 생성물 모드 %d건 — repin 판정 불필요(③④ 도구가 따로 판정) · 앞 30</summary>' % len(rg))
            w('')
            w('| 모드 | 이름 | 구 | repin | aligned | 판정 |')
            w('|---|---|---|---|---|---|')
            for x in rg[:30]:
                w(row_(x))
            w('')
            w('</details>')
    else:
        w('- (repin 산출물 없음)')
    w('')

    # offsets
    w('## 5. 구조체 오프셋 (재핀 후보값 기준 지문 대조)')
    w('')
    of = os.path.join(C.out, 'offsets_shifts.json')
    if os.path.isfile(of):
        d = json.load(open(of, encoding='utf-8'))
        for mod, v in d.items():
            if v['status'] == 'nobaseline':
                w('- %s — 기준선 지문 없음' % mod)
            elif v['status'] == 'match':
                w('- %s — 일치%s' % (mod, '' if v.get('baseline_ver') == BASE_VER else ' (⚠기준선 %s)' % v.get('baseline_ver')))
            else:
                w('- ★**%s — 변화 %d개 함수**' % (mod, len(v['rows'])))
                for r in v['rows'][:12]:
                    d_ = ('Δ%+#x ×%d' % (r['delta'], r['npairs'])) if r.get('delta') is not None else '-'
                    lb = ''
                    if r.get('delta') is None and (r.get('lost') or r.get('born')):   # 짝 없는 변화 — 사라짐/생김을 그대로 보인다
                        lb = '사라짐 %s / 생김 %s' % (','.join(r.get('lost') or []) or '-', ','.join(r.get('born') or []) or '-')
                    w('  - `%s` %s→%s %s %s' % (r['fn'], r.get('old', ''), r.get('new', ''), d_,
                                               ' '.join(x for x in (lb, r.get('note') or '') if x)))
        w('')
        w('> apply 뒤 정식 `offsets.py check` → `sources` 는 따로 돈다(이 표는 apply 전 예보). [json](offsets_shifts.json)')
    else:
        w('- (offsets 산출물 없음)')
    w('')

    # detail tactics
    w('## 6. ★상세전술 (tfm2_detail_tactics — 전술 100 · 패치창 218)')
    w('')
    tj = os.path.join(C.out, 'detail_tactics', 'track.json')
    if os.path.isfile(tj):
        try:
            t = summarize_track(tj)
        except Exception as e:
            t = {'schema': 'error', 'err': e}
        if t.get('schema') == 'v1':
            c = t['counts']
            if t.get('new_sha') and not C.new_sha.startswith(str(t['new_sha']).upper()):
                w('- ⚠track.json 의 신 sha `%s` ≠ 이번 exe — 이전 실행 산출물일 수 있다' % str(t['new_sha'])[:16])
            w('- 전술 %d개 → **자동확정(AUTO) %d · 검토 필요(REVIEW) %d** · 창 %s (자동 %s)'
              % (t['n'], t['verdict'].get('AUTO', 0), t['n'] - t['verdict'].get('AUTO', 0),
                 sum((c.get('windows') or {}).values()) if isinstance(c.get('windows'), dict) else '?', c.get('windows_auto', '?')))
            w('')
            w('| 전술의 가장 나쁜 창 판정 | 뜻 | 전술 수 | 창 수 |')
            w('|---|---|---|---|')
            wc = c.get('windows') if isinstance(c.get('windows'), dict) else {}
            for code, ko in CODE_KO.items():
                w('| `%s` | %s | %s | %s |' % (code, ko, ('**%d**' % t['worst'][code]) if (t['worst'].get(code) and code != 'SAME') else t['worst'].get(code, 0),
                                               wc.get(code, 0)))
            for code in t['worst']:
                if code not in CODE_KO:
                    w('| `%s` | (새 판정 어휘) | %d | %s |' % (code, t['worst'][code], wc.get(code, 0)))
            w('')
            w('- 전문: [track.md](detail_tactics/track.md) · [review_queue.json](detail_tactics/review_queue.json) · 초안 `detail_tactics\\draft_targets\\`')
            w('')
            w('### ★게임이 기본값을 바꾼 전술 (%d)' % len(t['default']))
            w('')
            if not t['default']:
                w('- 없음')
            else:
                w('| # | 탭 | 전술 | 구 기본값 → 신 | 창 |')
                w('|---|---|---|---|---|')
                for x in t['default'][:40]:
                    for v in x['value_changes'][:4]:
                        w('| %s | %s | %s | %s → **%s** (즉치 %s→%s) | `%s` |' % (
                            x.get('index'), x.get('tab') or '(숨김)', x.get('name'), v.get('old_default'), v.get('new_default'),
                            v.get('old_imm'), v.get('new_imm'), v.get('window')))
            w('')
            w('### 검토 필요 전술 (%d · 나쁜 판정 순 앞 30)' % len(t['review']))
            w('')
            if not t['review']:
                w('- 없음 — 전 전술 자동확정')
            for x in t['review'][:30]:
                rs = [str(r_) for r_ in (x.get('reasons') or [])]
                # 가장 나쁜 판정을 말하는 근거를 고른다(reasons 첫 줄은 다른 창의 「참고:」일 수 있다)
                wk = x.get('worst') or ''
                r0 = next((r_ for r_ in rs if (': %s' % wk) in r_), None) or next(
                    (r_ for r_ in rs if wk and wk in r_), None) or (rs[0] if rs else '')
                w('- [%s] **%s** (`%s`, %s) — `%s` %s%s' % (
                    x.get('index'), x.get('name'), x.get('id'), x.get('tab') or '숨김', x.get('worst'),
                    CODE_KO.get(x.get('worst'), ''), (' · ' + r0[:160]) if r0 else ''))
            if len(t['review']) > 30:
                w('- … 외 %d (track.md)' % (len(t['review']) - 30))
        elif t.get('schema') == 'error':
            w('- track.json 해석 실패: %s — 원문 확인' % t['err'])
        else:
            w('- 전술 %d개 · 판정별: %s' % (t['n'], ', '.join('**%s** %d' % kv for kv in t['cnt'].most_common())))
            if t['raw_summary']:
                w('- track.json 요약: `%s`' % json.dumps(t['raw_summary'], ensure_ascii=False)[:400])
            w('- 전문: [track.md](detail_tactics/track.md) · 검토 대기열 = `detail_tactics\\` 안 review_queue*')
            w('')
            w('### 기본값이 바뀐 전술 (%d)' % len(t['default']))
            w('')
            for nm, v, it in t['default'][:30]:
                w('- `%s` — %s %s' % (nm, v, _brief(it)))
            w('')
            w('### 검토 필요 전술 (%d, 앞 30)' % len(t['review']))
            w('')
            for nm, v, it in t['review'][:30]:
                w('- `%s` — %s %s' % (nm, v, _brief(it)))
    else:
        r = next((x for x in recs if x['stage'] == 'detail_tactics'), None)
        w('- (track.json 없음 — 단계 상태: %s %s)' % (ST_KO.get(r['status'], r['status']) if r else '?', r.get('note', '') if r else ''))
    rq = sorted(glob.glob(os.path.join(C.out, 'detail_tactics', 'review_queue*')))
    if rq:
        w('- 검토 대기열 파일: ' + ', '.join('[%s](detail_tactics/%s)' % (os.path.basename(x), os.path.basename(x)) for x in rq))
    w('')

    # addrbook
    w('## 7. 주소록 (addrbook — KB 에 적힌 함수·지점)')
    w('')
    am = summarize_addrmap(os.path.join(C.out, 'addrmap')) if os.path.isdir(os.path.join(C.out, 'addrmap')) else None
    if am:
        w('- %d건 · 등급 분포: %s · 파일 `%s`' % (am['n'], ', '.join('**%s** %d' % kv for kv in am['cnt'].most_common()),
                                           rel(am['file'], C.out)))
        rs_ = am['raw_summary'] if isinstance(am['raw_summary'], dict) else None
        if rs_ and isinstance(rs_.get('grade'), dict):     # addrbook.py map 스키마(summary.grade/auto/by_kind/change)
            re_ = rs_.get('row_errors')
            w('- ★자동 반영 가능(SAME·MOVED_BYTES_CHANGED) **%s** (%.1f%%) · 검토(FN_EDITED·AMBIGUOUS·MISSING) **%s** · 행 오류 %s'
              % (rs_.get('auto', '?'), 100.0 * float(rs_.get('auto_rate') or 0), rs_.get('review', '?'),
                 len(re_) if isinstance(re_, list) else re_))
            bk = rs_.get('by_kind') or {}
            if bk:
                gs = [g for g in ('SAME', 'MOVED_BYTES_CHANGED', 'FN_EDITED', 'AMBIGUOUS', 'MISSING')
                      if any(g in v for v in bk.values())]
                gs += sorted({g for v in bk.values() for g in v} - set(gs))
                w('')
                w('| 종류 | ' + ' | '.join(gs) + ' |')
                w('|---|' + '---|' * len(gs))
                for k, v in sorted(bk.items(), key=lambda kv: -sum(kv[1].values())):
                    w('| %s | %s |' % (k, ' | '.join(str(v.get(g, 0)) for g in gs)))
                w('')
            ch = rs_.get('change') or {}
            if ch:
                w('- MOVED_BYTES_CHANGED 의 바뀐 방식(상위): %s' % ', '.join(
                    '%s %d' % kv for kv in sorted(ch.items(), key=lambda kv: -kv[1])[:10]))
        elif am['raw_summary']:
            w('- 요약: `%s`' % json.dumps(am['raw_summary'], ensure_ascii=False)[:400])
        if os.path.isfile(os.path.join(C.out, 'addrmap', 'addrmap.md')):
            w('- 구→신 표: [addrmap.md](addrmap/addrmap.md) (KB 일괄 갱신 = record-keeper 경유)')
    else:
        r = next((x for x in recs if x['stage'] == 'addrbook'), None)
        w('- (주소록 산출물 없음 — 단계 상태: %s %s)' % (ST_KO.get(r['status'], r['status']) if r else '?', r.get('note', '') if r else ''))
    w('')

    # hotfix exposure
    w('## 8. 핫픽스 노출 표 (게임을 켜기 전에 볼 것)')
    w('')
    if os.path.isfile(HOTFIX_JSON):
        try:
            hj = json.load(open(HOTFIX_JSON, encoding='utf-8'))
            w('- 이번 판정: **%s** — %s' % ('핫픽스 라벨(버전 문자열 그대로 · sha 만 다름)' if C.hotfix else '정식 버전 변경(또는 --label 지정)',
                                         '아래 「핫픽스」 열을 볼 것' if C.hotfix else '「정식패치」 열을 볼 것'))
            if isinstance(hj, dict) and isinstance(hj.get('conclusion'), list):
                for x in hj['conclusion'][:8]:
                    w('- ' + (x if isinstance(x, str) else json.dumps(x, ensure_ascii=False))[:300])
            w('')
            for l in render_generic_table(hj):
                w(l)
            w('')
            md_ = os.path.splitext(HOTFIX_JSON)[0] + '.md'
            w('원문 `%s`%s' % (HOTFIX_JSON, (' · 사람용 `%s`' % md_) if os.path.isfile(md_) else ''))
        except Exception as e:
            w('- 해석 실패: %s — 원문 `%s`' % (e, HOTFIX_JSON))
    else:
        w('- (`%s` 없음)' % HOTFIX_JSON)
    w('')

    # next
    w('## 9. 다음에 사람이 할 일 (순서대로 — 정확한 명령 = MIG\\patchday\\README.md)')
    w('')
    nxt = []
    if warns:
        nxt.append('⚠ 위 비파괴 경고부터 확인(다른 세션 작업인지)')
    if C.hotfix:
        nxt.append('★**핫픽스 라벨** — 8절 「핫픽스」 열 권고대로 위험 모드를 끈 뒤에만 게임을 켤 것(deps 대역을 통과해 낡은 주소로 로드된다)')
    nxt += ['**게임은 아직 켜지 말 것** — 8절 노출 표에서 낡은 주소로 패치하는 모드가 있는지 확인',
            '6절 상세전술: `detail_tactics\\track.md`·review_queue → 사람 판정 → draft_targets 승격 → `gen_sites.py --ver %s` → `build_local.ps1 -RunTests` → 인게임 검증' % C.label,
            'tfm2_tactics_verify: `tools\\migrate_targets.py` (sites.rs 생성물 — repin apply 대상 아님)',
            '4절 apply 대상 잔여 판정 %s건(+동형 클론) → `MIG\\_mkfinal%s.py` 로 final 맵 → `repin.py apply <MOD…> --write`(생성물 모드 %s 제외) → `rebase` → `coverage` → `offsets.py check`·`sources` → `bump_deps.py` → `env.py` → `run.py` 0'
            % (n_apply_res, C.tok, '·'.join(GEN_MODS)),
            '2절 EDITED 함수 = 재현코드(ai_adjust·judge 계층)에 닿는지 확인',
            '7절 `addrmap.md` 구→신 표로 KB 갱신(record-keeper 경유)',
            'REPORT `_%s_마이그.md`·모드별 03_시행착오·DONE.md 기록' % C.label]
    for i, x in enumerate(nxt, 1):
        w('%d. %s' % (i, x))
    w('')
    io.open(os.path.join(C.out, 'SUMMARY.md'), 'w', encoding='utf-8', newline='\n').write('\n'.join(o))


# ───────────────────────────────────────────────────────────── main
def parse_sel(s):
    out = []
    for x in (s or '').split(','):
        x = x.strip()
        if not x:
            continue
        if x.isdigit() and int(x) < len(STAGES):
            out.append(STAGES[int(x)])
        elif x in STAGES:
            out.append(x)
        else:
            raise ValueError('알 수 없는 단계: %s (가능: %s)' % (x, ', '.join(STAGES)))
    return out


def main():
    if len(sys.argv) > 1 and sys.argv[1] == '_offsets_worker':
        return offsets_worker(sys.argv[2:])
    ap = argparse.ArgumentParser(
        prog='run_patchday.py',
        description='패치날 비파괴 실행기 — 새 exe 하나로 fnindex·callgraph·aidiff·check·repin plan·offsets·상세전술·주소록을 돌리고 SUMMARY.md 를 만든다(apply/rebase/snap/gen 은 부르지 않는다).',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog='단계: ' + ', '.join('%d %s' % (i, s) for i, s in enumerate(STAGES)) +
               '\n종료코드: 0=업데이트 없음 또는 전 단계 정상 종료 / 1=단계 오류 또는 비파괴 경고 / 2=실행기 오류'
               '\n권장(패치날): --skip addrbook 으로 먼저(캐시 있을 때 5~10분) → --only addrbook (≈28분·메모리 약 9GB, 앞 결과 유지)'
               '\n예) 리허설: python -X utf8 MIG\\patchday\\run_patchday.py --rehearse --new-exe F:\\claude\\tfm2\\tfm2_0.6.4\\TeamfightManager2.exe --label 0.6.4 --skip addrbook')
    ap.add_argument('--new-exe', default=None, help='새 exe (기본 = 설치본 mig_verify.GAME_EXE)')
    ap.add_argument('--label', default=None, help="버전 라벨 (기본 = base_version.txt · 핫픽스면 '<버전>_<sha앞8>')")
    ap.add_argument('--rehearse', action='store_true', help='백업 건너뜀 · 출력 = MIG\\_patchday\\_rehearsal_<label>\\')
    ap.add_argument('--skip', default='', help='건너뛸 단계(이름/번호, 쉼표)')
    ap.add_argument('--only', default='', help='이 단계만(+detect·summary)')
    a = ap.parse_args()
    t_all = time.time()

    mods_base, game_exe, _MV = mig_verify_mods()
    C = Ctx()
    C.rehearse = a.rehearse
    C.new_exe = os.path.abspath(a.new_exe or game_exe)
    try:
        skip, only = parse_sel(a.skip), parse_sel(a.only)
    except ValueError as e:
        print(str(e))
        return 2

    # ── 0 detect
    t0 = time.time()
    print('■ 0 detect')
    if not os.path.isfile(C.new_exe):
        print('  신 exe 없음: %s' % C.new_exe)
        return 2
    if not os.path.isfile(BASE_EXE):
        print('  기준 exe 없음: %s' % BASE_EXE)
        return 2
    C.new_sha = sha256_file(C.new_exe)
    C.new_size = os.path.getsize(C.new_exe)
    print('  신 %s  %d B  sha %s' % (C.new_exe, C.new_size, C.new_sha[:16]))
    if C.new_sha == BASE_SHA:
        print('  아직 업데이트 안 됨 — 신 exe sha 가 기준 %s 와 같다 (출력 없음)' % BASE_VER)
        return 0
    bsha = sha256_file(BASE_EXE)
    if bsha != BASE_SHA:
        print('  ★기준 exe 의 sha 가 상수와 다르다: %s ≠ %s — 기준 백업이 손상/교체됨. 중단' % (bsha[:16], BASE_SHA[:16]))
        return 2
    C.bv, C.bv_path = read_base_version(C.new_exe, game_exe)
    C.label_given = bool(a.label)
    C.label, C.hotfix, why = decide_label(a.label, C.bv, C.new_sha)
    if C.label is None:
        print('  ' + why)
        return 2
    C.tok = safe_tok(C.label)
    C.out = os.path.join(PDROOT, ('_rehearsal_' if C.rehearse else '') + C.label)
    C.logs = os.path.join(C.out, 'logs')
    C.new_exe_used = C.new_exe
    C.new_pkl = C.new_cg = None
    C.base_pkl_bad = False
    print('  ★업데이트 감지: %s → %s%s (tok %s) · 출력 %s' % (BASE_VER, C.label, ' [핫픽스 라벨]' if C.hotfix else '',
                                                        C.tok, C.out))
    os.makedirs(C.logs, exist_ok=True)
    C.mods = list(mods_base) + [m for m in EXTRA_MODS if m not in mods_base
                                and os.path.isfile(os.path.join(MIGD, 'manifest', m + '.json'))]
    C.extra = [m for m in C.mods if m not in mods_base]
    print('  모드 %d개 (MODS %d + 명시 추가 %s)' % (len(C.mods), len(mods_base), C.extra))

    snap0 = snapshot(C.mods)
    print('  비파괴 스냅샷: git status %d줄 · 보호 파일 %d개' % (len(snap0['git']), len(snap0['files'])))

    prevrun = {}
    rj = os.path.join(C.out, 'run.json')
    if os.path.isfile(rj):
        try:
            pj = json.load(open(rj, encoding='utf-8'))
            if pj.get('new_sha') == C.new_sha:
                prevrun = {r['stage']: r for r in pj.get('stages', [])}
                for k in ('new_exe_used', 'new_pkl', 'new_cg'):
                    if pj.get(k) and os.path.isfile(pj[k]):
                        setattr(C, k, pj[k])
        except Exception:
            prevrun = {}
    recs = [stage_rec('detect', 'OK', 0, time.time() - t0,
                      '%s → %s · sha %s · %d B' % (BASE_VER, C.label, C.new_sha[:16], C.new_size))]
    io.open(logp(C, 'detect'), 'w', encoding='utf-8').write(
        '# %s detect\n신 exe %s\nsha %s\n크기 %d\nbase_version %s (%s)\n라벨 %s (핫픽스 %s)\n기준 %s %s\n' %
        (now(), C.new_exe, C.new_sha, C.new_size, C.bv, C.bv_path, C.label, C.hotfix, BASE_VER, BASE_SHA))
    recs[0]['log'] = logp(C, 'detect')

    # 단계를 건너뛰어도 뒤 단계가 쓸 수 있게 기존 산출물 경로를 미리 잡는다(검증은 그 단계가 돌 때)
    for attr, cands in (('new_pkl', [os.path.join(ROOT, '_fnidx_%s.pkl' % C.tok), os.path.join(C.out, '_fnidx_%s.rebuilt.pkl' % C.tok)]),
                        ('new_cg', [os.path.join(ROOT, '_cg_%s.pkl' % C.tok), os.path.join(C.out, '_cg_%s.rebuilt.pkl' % C.tok)])):
        if not getattr(C, attr):
            for c in cands:
                if os.path.isfile(c):
                    setattr(C, attr, c)
                    break
    if C.rehearse is False:
        bexe = os.path.join(TFM2, 'tfm2_' + C.label, 'TeamfightManager2.exe')
        if os.path.isfile(bexe) and C.new_exe_used == C.new_exe and sha256_file(bexe) == C.new_sha:
            C.new_exe_used = bexe

    fns = {'backup': st_backup, 'fnindex': st_fnindex, 'callgraph': st_callgraph, 'aidiff': st_aidiff,
           'check': st_check, 'repin': st_repin, 'offsets': st_offsets, 'detail_tactics': st_detail,
           'addrbook': st_addrbook}
    for name in STAGES[1:-1]:
        if (only and name not in only) or name in skip:
            if name in prevrun and prevrun[name]['status'] != 'SKIP':
                r = dict(prevrun[name], _prev=True)
            else:
                r = stage_rec(name, 'SKIP', note='--only' if only else '--skip')
            recs.append(r)
            print('■ %d %s — 건너뜀%s' % (STAGES.index(name), name, ' (이전 실행 결과 유지)' if r.get('_prev') else ''))
            continue
        print('■ %d %s' % (STAGES.index(name), name), flush=True)
        try:
            r = fns[name](C)
        except KeyboardInterrupt:
            r = stage_rec(name, 'FAIL', None, 0, '사용자 중단')
            recs.append(r)
            print('  중단됨')
            break
        except Exception as e:
            import traceback
            tb = traceback.format_exc()
            io.open(logp(C, name), 'a', encoding='utf-8').write('\n# 실행기 예외\n' + tb)
            r = stage_rec(name, 'FAIL', None, 0, '실행기 예외: %s' % e, logp(C, name))
            print('  ★실행기 예외: %s' % e)
        recs.append(r)
        print('  = %s · %s' % (ST_KO.get(r['status'], r['status']), str(r.get('note', ''))[:200]), flush=True)
        json.dump({'label': C.label, 'new_sha': C.new_sha, 'stages': recs, 'partial': True},
                  io.open(rj, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)

    snap1 = snapshot(C.mods)
    warns = snap_diff(snap0, snap1)
    if snap0['git_rc'] != 0 or snap1['git_rc'] != 0:
        warns.append('git status 실행 실패(rc %s/%s) — git 쪽 비파괴 확인 불가' % (snap0['git_rc'], snap1['git_rc']))
    total = time.time() - t_all
    srec = None
    if 'summary' not in skip:
        t1 = time.time()
        srec = stage_rec('summary', 'OK', 0, 0, 'SUMMARY.md', None, [os.path.join(C.out, 'SUMMARY.md')])
        recs.append(srec)
        try:
            write_summary(C, recs, warns, total)
            srec['sec'] = round(time.time() - t1, 1)
        except Exception as e:
            import traceback
            srec.update(status='FAIL', note='요약 실패: %s' % e)
            io.open(logp(C, 'summary'), 'w', encoding='utf-8').write(traceback.format_exc())
            srec['log'] = logp(C, 'summary')
    json.dump({'label': C.label, 'tok': C.tok, 'rehearse': C.rehearse, 'hotfix': C.hotfix,
               'base_ver': BASE_VER, 'base_sha': BASE_SHA, 'new_sha': C.new_sha, 'new_size': C.new_size,
               'new_exe': C.new_exe, 'new_exe_used': C.new_exe_used, 'new_pkl': C.new_pkl, 'new_cg': C.new_cg,
               'mods': C.mods, 'mods_added_outside_MODS': C.extra, 'stages': recs, 'nondestructive_warnings': warns,
               'total_sec': round(total, 1), 'finished': now()},
              io.open(rj, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
    bad = [r['stage'] for r in recs if r['status'] == 'FAIL' and not r.get('_prev')]
    print('')
    print('■ 끝 — 전체 %.0fs · 오류 단계 %s · 비파괴 경고 %d건' % (total, bad or '없음', len(warns)))
    for x in warns[:20]:
        print('  ⚠ ' + x)
    print('  요약 = %s' % os.path.join(C.out, 'SUMMARY.md'))
    return 1 if (bad or warns) else 0


if __name__ == '__main__':
    sys.exit(main())
