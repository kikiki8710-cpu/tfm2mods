#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""dllmatch.py - **exe ↔ 내 DLL 의 기계어끼리** 맞춰 exe 함수에 이름을 붙인다(2안).

왜 이 방식인가:
  1안(rlib DWARF 의 패닉 줄로 조인)은 원리적 한계가 있다 - **패닉 사이트가 없는 함수 964개**는
  영원히 못 잡는다(실측 분류). 커버리지가 52.5% 에서 더 안 올라간다.
  2안은 패닉 줄에 전혀 의존하지 않는다:
    - 내 DLL 에는 **같은 rlib 이 컴파일돼 있고**, 링커 MAP 이 `망글심볼 -> DLL RVA` 를 정확히 준다.
      (이름은 망글 심볼에서 바로 나오므로 **조인할 필요조차 없다**.)
    - exe 와 내 DLL 을 **같은 방식으로 디스어셈**해 지문을 뽑아 맞춘다.
      IR 파싱의 불완전성(인라인·모듈 분산·DILocation 누락)이 개입하지 않는다.

지문 = **메모리 오퍼랜드의 변위(구조체 필드 오프셋) 집합**.
  구조체 레이아웃은 최적화 수준과 무관하게 보존되므로 opt-level 이 달라도 살아남는다.
  (상수는 접히고 인라인돼 못 쓴다 - irskew.py 가 그래서 폐기됐다.)

MAP 만들기(한 번):
  rustc ... -C link-arg=/MAP:m.map ...   → `0001:002e9d50  _RNv...  00000001802ead50  ...obj`

검증: `--selftest` - 이미 확인된 exe 주소들이 제 이름으로 붙는지. **통과 전에는 쓰지 말 것.**
"""
import argparse
import glob
import io
import json
import os
import re
import struct
import sys

import capstone

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

HERE = os.path.dirname(os.path.abspath(__file__))
EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
# ★분석용 DLL 은 **opt-level=3** 으로 빌드한다 - exe 도 고최적화라 인라인 양상을 맞춰야
#   오프셋 지문이 겹친다(opt-level=1 로 빌드했을 때 유사도가 눈에 띄게 낮았다).
#   이 DLL 은 실행하지 않으므로 CLAUDE.md 의 "opt-level 2/3 은 스택오버플로" 경고와 무관하다.
_MAPDIR = os.path.join(os.environ.get('TEMP', r'C:\Temp'), 'tfm2_map3')
DLL = os.path.join(_MAPDIR, 'm.dll')
MAP = os.path.join(_MAPDIR, 'm.map')
AIMAP = os.path.join(HERE, 'aimap.json')
OUT = os.path.join(HERE, 'dllmatch.json')
IRDIRS = [r'C:\tfm2mods\_gaibc', r'C:\tfm2mods\_gcbc']

MIN_OFF = 0x20
MAX_OFF = 0x4000
MIN_N = 5           # 지문 원소가 이보다 적으면 판정하지 않는다
MIN_J = 0.60        # 자카드 유사도 하한

MAP_RE = re.compile(r'^\s*[0-9a-f]{4}:[0-9a-f]{8}\s+(\S+)\s+([0-9a-fA-F]{16})\s', re.M)
SUB_RE = re.compile(r'!DISubprogram\(name: "([^"]+)", linkageName: "([^"]+)"[^)]*?file: (![0-9]+)')
FILE_RE = re.compile(r'(![0-9]+) = !DIFile\(filename: "([^"]+)"')


def leaf(mangled):
    """Rust v0 망글 심볼에서 **마지막 이름 성분**만 뽑는다.
    v0 는 `<길이><이름>` 길이접두라 `\d+(\w+)` 같은 정규식으로는 못 자른다
    (숫자가 다음 성분의 길이인데 식별자 문자로 먹혀 통째로 붙어 나온다)."""
    out, i, n = [], 0, len(mangled)
    while i < n:
        if mangled[i].isdigit():
            j = i
            while j < n and mangled[j].isdigit():
                j += 1
            ln = int(mangled[i:j])
            if j + ln <= n and ln > 0:
                out.append(mangled[j:j + ln].lstrip('_'))
                i = j + ln
                continue
        i += 1
    if not out:
        return mangled
    # ★클로저·백레퍼런스 성분(`0`, `0E0`, `s_0B5_`)은 이름이 아니다.
    #   그대로 쓰면 Ghidra 에 `path_finder__0E0` 같은 쓸모없는 이름이 박힌다
    #   (안 붙이느니만 못하다). **알파벳 3자 이상이 이어지는 성분**만 진짜 이름으로 본다.
    real = [p for p in out if re.search(r'[A-Za-z]{3,}', p)]
    if not real:
        return out[-1]
    # 마지막 성분이 클로저 표식이면 부모 함수 이름에 `_cl` 을 붙인다.
    return real[-1] if out[-1] == real[-1] else real[-1] + '_cl'


def pe(path):
    d = open(path, 'rb').read()
    off = struct.unpack_from('<I', d, 0x3c)[0]
    nsec = struct.unpack_from('<H', d, off + 6)[0]
    optsz = struct.unpack_from('<H', d, off + 20)[0]
    base = struct.unpack_from('<Q', d, off + 24 + 24)[0]
    secs = [struct.unpack_from('<IIII', d, off + 24 + optsz + i * 40 + 8) for i in range(nsec)]
    # ★예외 디렉토리(.pdata) = 데이터디렉토리 3번. PE32+ 는 옵션헤더 +112 부터 디렉토리.
    exc = struct.unpack_from('<II', d, off + 24 + 112 + 3 * 8)
    return d, base, secs, exc


def func_ranges(d, secs, exc):
    """★x64 PE 의 **정확한 함수 경계**를 .pdata 에서 읽는다 - 추정하지 않는다.

    왜 필요한가(2026-09-10 실사고): 처음엔 링커 MAP 의 "다음 심볼 VA 까지" 로 크기를 잡았는데,
    MAP 에는 내부 함수가 다 실리지 않아 **한 심볼 뒤의 남의 함수 수십 개가 통째로 지문에 섞였다**
    (`position_eval_at_uncached` 가 27,632B 로 잡혔다 - 함수 하나가 그럴 리 없다).
    그 결과 지문이 0x20/0x28/0x30 같은 **어디에나 있는 오프셋 뭉텅이**가 되어 매칭이 무의미해졌다.
    RUNTIME_FUNCTION{Begin, End, UnwindData} 은 시작·끝을 정확히 준다.
    """
    va, size = exc
    if not va or not size:
        return {}
    o = None
    for vsz, sva, rsz, ra in secs:
        if sva <= va < sva + max(vsz, rsz):
            o = ra + va - sva
            break
    if o is None:
        return {}
    out = {}
    for i in range(size // 12):
        b, e, _u = struct.unpack_from('<III', d, o + i * 12)
        if e > b:
            # 청크가 쪼개진 함수(같은 Begin 이 여러 번)는 가장 넓은 범위를 쓴다
            out[b] = max(out.get(b, 0), e - b)
    return out


def make_off(secs):
    def f(rva):
        for vsz, va, rsz, ra in secs:
            if va <= rva < va + max(vsz, rsz):
                return ra + rva - va
        return None
    return f


def fingerprint(cs, data, foff, rva, size):
    o = foff(rva)
    if o is None or size <= 0:
        return set()
    out = set()
    for ins in cs.disasm(data[o:o + size], rva):
        for op in ins.operands:
            if op.type == capstone.x86.X86_OP_MEM:
                v = op.mem.disp
                # rsp/rbp 기준은 지역변수라 구조체 지문이 아니다
                b = ins.reg_name(op.mem.base) if op.mem.base else ''
                if b in ('rsp', 'rbp', 'rip'):
                    continue
                if MIN_OFF <= v < MAX_OFF:
                    out.add(int(v))
    return out


DEF_RE = re.compile(r'^define[^@\n]*@([A-Za-z0-9_.$]+)\(', re.M)
DBG_RE = re.compile(r'!dbg (![0-9]+)')
LOC_RE = re.compile(r'^(![0-9]+) = !DILocation\(line: (\d+), column: \d+, scope: (![0-9]+)', re.M)
# ★scope 가 **어느 소스 파일**인지 알아야 한다. 이걸 안 보면 인라인된 남의 파일 줄이 섞인다.
SCOPE_FILE_RE = re.compile(
    r'^(![0-9]+) = (?:distinct )?!DI(?:Subprogram|LexicalBlock|LexicalBlockFile)\([^\n]*?file: (![0-9]+)',
    re.M)


def ir_index():
    """망글 심볼 -> (소스 모듈명, 그 함수가 참조하는 소스 줄 집합).

    ★줄 집합이 왜 필요한가: 지문(기계어)만으로는 **형제 함수를 구분할 수 없다**.
      실측 - `collect_unseen_enemy_estimates` 와 `collect_known_enemy_positions` 가
      서로 이름이 뒤바뀌어 배정됐다. 코드 모양이 거의 같기 때문이다.
      그런데 **패닉 줄은 둘을 정확히 가른다**(각자 제 소스 줄에서만 패닉한다).
      => 지문(내용) · 호출그래프(관계) · 소스줄(출처) 세 축을 함께 쓴다.
    인라인된 콜리의 줄도 포함한다 - exe 함수는 인라인된 콜리의 패닉도 품기 때문이다.
    """
    # ★디스크 캐시 - .ll 전량 재파싱은 수 분 걸린다. 원본이 바뀌면 자동 무효화한다.
    files = sorted(sum([glob.glob(os.path.join(p, '*.ll')) for p in IRDIRS], []))
    stamp = 'v2:' + str([(os.path.basename(f), os.path.getmtime(f), os.path.getsize(f)) for f in files])
    cache = os.path.join(HERE, 'ir_index.json')
    if os.path.exists(cache):
        try:
            c = json.load(io.open(cache, encoding='utf-8'))
            if c.get('stamp') == stamp:
                return c['src'], {k: set(v) for k, v in c['lines'].items()}
        except Exception:
            pass
    src, lines = {}, {}
    for f in files:
        t = io.open(f, encoding='utf-8', errors='ignore').read()
        fmap = dict(FILE_RE.findall(t))
        for m in SUB_RE.finditer(t):
            s = fmap.get(m.group(3), '')
            if s:
                src[m.group(2)] = s.replace(chr(92), '/').split('/')[-1].replace('.rs', '')
        # scope -> 파일. DILexicalBlock 은 대개 자체 `file:` 을 들고 있어 한 단계면 충분하다.
        sfile = dict(SCOPE_FILE_RE.findall(t))
        loc = {k: (int(l), sfile.get(s)) for k, l, s in LOC_RE.findall(t)}
        own_file = {m.group(2): m.group(3) for m in SUB_RE.finditer(t)}
        for m in DEF_RE.finditer(t):
            sym = m.group(1)
            j = t.find('\n}\n', m.start())
            body = t[m.start():j if j > 0 else m.start() + 40000]
            of = own_file.get(sym)
            # ★**그 함수 자신의 파일에 속한 줄만** 남긴다.
            #   전엔 본문의 `!dbg` 를 전부 긁어 **인라인된 콜리(다른 파일)의 줄까지 섞였다**.
            #   실측: `position_eval_at_uncached` 의 줄집합이 [1, 8, 9, 12, ...] 로 나왔는데
            #   exe 쪽 패닉 줄은 [374, 412, 418, ...] 이라 하나도 안 겹쳤다 —
            #   크기·지문·호출관계 3중 근거로 맞다고 확인한 짝인데도. 신호가 오염돼 있었다.
            ls = set()
            for x in DBG_RE.findall(body):
                v = loc.get(x)
                if v and v[0] and (of is None or v[1] == of):
                    ls.add(v[0])
            if ls:
                lines[sym] = ls
        del t
    json.dump({'stamp': stamp, 'src': src, 'lines': {k: sorted(v) for k, v in lines.items()}},
              io.open(cache, 'w', encoding='utf-8'), separators=(',', ':'))
    return src, lines


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--selftest', action='store_true')
    ap.add_argument('--minj', type=float, default=MIN_J)
    a = ap.parse_args()

    if not os.path.exists(MAP):
        print('MAP 이 없다: %s\n  rustc 에 `-C link-arg=/MAP:<경로>` 를 붙여 한 번 빌드할 것.' % MAP)
        return

    # ── 내 DLL: 망글심볼 -> RVA (MAP). ★크기는 **.pdata 의 실제 함수 경계**로 잡는다.
    mt = io.open(MAP, encoding='utf-8', errors='ignore').read()
    syms = []
    for m in MAP_RE.finditer(mt):
        name, va = m.group(1), int(m.group(2), 16)
        syms.append((va, name))
    syms.sort()
    dll_d, dll_base, dll_secs, dll_exc = pe(DLL)
    dll_off = make_off(dll_secs)
    dll_fr = func_ranges(dll_d, dll_secs, dll_exc)
    dsym, off_pdata = {}, 0
    for va, name in syms:
        rva = va - dll_base
        sz = dll_fr.get(rva)
        if sz is None:
            # .pdata 에 시작이 없다 = 리프/청크 함수거나 심볼이 함수 시작이 아니다. 버린다.
            off_pdata += 1
            continue
        dsym[name] = (rva, sz)

    # ── exe: RVA -> (모듈, 크기). ★크기는 aimap 값 대신 **.pdata** 를 우선한다.
    info = json.load(open(AIMAP)).get('info', {})
    exe_d, exe_base, exe_secs, exe_exc = pe(EXE)
    exe_off = make_off(exe_secs)
    exe_fr = func_ranges(exe_d, exe_secs, exe_exc)
    print('.pdata 함수경계: 내 DLL %d개(MAP 심볼 중 경계없어 버린 것 %d) · exe %d개'
          % (len(dll_fr), off_pdata, len(exe_fr)))

    src_of, lines_of = ir_index()
    cs = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    cs.detail = True

    # 모듈별로 내 DLL 후보를 모은다
    by_mod = {}
    for name, (rva, sz) in dsym.items():
        s = src_of.get(name)
        if not s or sz < 32:
            continue
        by_mod.setdefault(s, []).append((name, rva, sz))

    # ── ★내 DLL 지문은 **한 번만** 뽑아 캐시한다(exe 함수마다 다시 뜨면 수백 배 낭비).
    dfp = {}
    for mod, lst in by_mod.items():
        for name, drva, dsz in lst:
            fd = fingerprint(cs, dll_d, dll_off, drva, dsz)
            if len(fd) >= MIN_N:
                dfp[name] = fd

    # ── ★오프셋마다 **희소도 가중(IDF)**. 이게 없으면 지문 비교가 거의 무의미하다:
    #   `0x20`·`0x28` 은 사실상 모든 함수에 나오고, `0x5c0`·`0x930` 은 그 함수에만 나온다.
    #   전부 같은 무게로 세면 **흔한 오프셋만 몇 개 가진 작은 심볼이 아무 함수와도 1.00** 이 되어
    #   일대일 배정에서 엉뚱한 주소에 먼저 채인다(실측: position_eval_at 이 그렇게 증발했다).
    import math
    df = {}
    for fd in dfp.values():
        for o in fd:
            df[o] = df.get(o, 0) + 1
    ND = max(len(dfp), 1)
    def w(o):
        return math.log(1.0 + ND / (1.0 + df.get(o, 0)))
    def mass(sset):
        return sum(w(o) for o in sset) or 1e-9

    dmass = {k: mass(v) for k, v in dfp.items()}

    # ── exe 함수마다 후보 점수를 전부 계산해 둔다(아직 확정하지 않는다)
    scores, nocand, weak = {}, 0, 0
    emeta = {}
    for rva_s, e in info.items():
        if not isinstance(e, dict):
            continue
        mod = str(e.get('mod', '')).replace(chr(92), '/').split('/')[-1]
        rva = int(rva_s, 16)
        sz = exe_fr.get(rva) or int(e.get('bytes', 0))
        cands = by_mod.get(mod, [])
        if not cands or sz <= 0:
            nocand += 1
            continue
        fe = fingerprint(cs, exe_d, exe_off, rva, sz)
        if len(fe) < MIN_N:
            weak += 1
            continue
        emeta[rva] = (mod, sz)
        me = mass(fe)
        els = {x for x in e.get('lines', []) if isinstance(x, int)}
        row = {}
        for name, _drva, _dsz in cands:
            fd = dfp.get(name)
            if fd is None:
                continue
            inter = mass(fe & fd)
            # 가중 포함도(내 함수가 exe 안에 있나) × 가중 정밀도(exe 의 그 부분을 내가 설명하나).
            # 기하평균이라 **한쪽만 높은 우연 일치**는 걸러진다.
            r, p = inter / dmass[name], inter / me
            fp = math.sqrt(r * p)
            # ★소스줄 증거를 곱한다. 지문이 같아 보이는 형제 함수는 **줄로만 갈린다**.
            #   양쪽 다 줄 정보가 있을 때만 적용하고, 가중은 완만하게 둔다 -
            #   rlib 과 exe 가 실제로 다른 소스인 함수가 있어(nexus_final_stand)
            #   줄을 과신하면 그런 곳에서 오히려 틀린다.
            dls = lines_of.get(name)
            if els and dls:
                fp *= 0.35 + 0.65 * (len(els & dls) / len(els))
            if r > 0:
                row[name] = (fp, r)
        if row:
            scores[rva] = row

    # ── ★상호 최선(mutual best) 배정. 그리디 전역 정렬은 "먼저 집은 쪽이 임자" 라
    #   점수 척도가 함수마다 조금만 어긋나도 오배정이 연쇄한다. 상호 최선은
    #   "서로가 서로의 1순위" 일 때만 확정하므로 그 연쇄가 없다. 남은 것은 라운드를 돌린다.
    rows = []
    freeE = set(scores)
    freeD = set(dfp)
    for _round in range(6):
        bestE = {}
        for rva in freeE:
            c = [(v[0], n) for n, v in scores[rva].items() if n in freeD]
            if c:
                bestE[rva] = max(c)
        bestD = {}
        for rva, (sc, n) in bestE.items():
            if n not in bestD or sc > bestD[n][0]:
                bestD[n] = (sc, rva)
        fixed = 0
        for n, (sc, rva) in bestD.items():
            mod, sz = emeta[rva]
            rows.append({'addr': '0x%x' % (0x140000000 + rva), 'rva': rva, 'mangled': n,
                         'name': leaf(n), 'mod': mod, 'jaccard': round(sc, 3),
                         'contain': round(scores[rva][n][1], 3), 'bytes': sz})
            freeE.discard(rva); freeD.discard(n); fixed += 1
        if not fixed:
            break
    # ── ★교환 복구(2-opt). 상호 최선은 **국소 최적**이라, 두 함수가 서로의 짝을
    #   차지한 채 굳는 경우가 남는다(실측: 여러 주소가 `deadly_band_cell` 하나를 두고 엇갈렸다).
    #   두 배정을 맞바꿔 총점이 오르면 바꾼다. 짝수는 수백이라 비용은 무시할 만하다.
    asg = [(r['rva'], r['mangled']) for r in rows]

    def sc(e, d):
        v = scores.get(e, {}).get(d)
        return v[0] if v else 0.0
    swaps = 0
    for _ in range(4):
        moved = 0
        for i in range(len(asg)):
            for j in range(i + 1, len(asg)):
                (e1, d1), (e2, d2) = asg[i], asg[j]
                if sc(e1, d2) + sc(e2, d1) > sc(e1, d1) + sc(e2, d2) + 1e-9:
                    asg[i], asg[j] = (e1, d2), (e2, d1)
                    moved += 1
        swaps += moved
        if not moved:
            break
    if swaps:
        pos = {r['rva']: r for r in rows}
        for e, d in asg:
            r = pos[e]
            if r['mangled'] != d:
                v = scores.get(e, {}).get(d)
                r['mangled'], r['name'] = d, leaf(d)
                if v:
                    r['jaccard'], r['contain'] = round(v[0], 3), round(v[1], 3)
    print('교환 복구: %d건 맞바꿈' % swaps)

    nocand += len(freeE)

    # ══ ★2단계: **호출그래프 전파**. 지문은 큰 함수에만 통한다.
    #   841B 짜리 캐시 래퍼는 지문 원소가 4개뿐이라 원리적으로 판별 불가다
    #   (`position_eval_at` 이 그래서 계속 미매칭이었다).
    #   그런데 호출 관계로는 즉시 드러난다 - 그 래퍼가 부르는 것은 uncached 본체 하나뿐이고,
    #   본체의 호출자도 그 래퍼 하나뿐이었다. 지문으로 큰 것을 고정하고 그래프로 작은 것을 잡는다.
    cg_e = {int(k): v for k, v in json.load(open(os.path.join(HERE, 'cg_exe.json'))).items()}
    cg_d = {int(k): v for k, v in json.load(open(os.path.join(HERE, 'cg_dll.json'))).items()}
    # ★후보는 **rlib(game_ai/game_core) 심볼로 한정**한다.
    #   내 분석용 DLL 에는 rlib 말고 **내 모드 코드도 함께 링크**돼 있는데,
    #   그 코드는 game_ai 함수를 재구현하며 호출하므로 그래프상 바로 이웃이 된다.
    #   한정하지 않으면 게임 exe 함수에 `tune`·`chain40_dump` 같은
    #   **내 모드 함수 이름이 붙는다**(실측: 주입 목록에 그대로 올라왔다).
    rva_of = {n: r for n, (r, _s) in dsym.items() if n in src_of}
    sym_at = {r: n for n, r in rva_of.items()}

    def rev(g):
        out = {}
        for a, bs in g.items():
            for b in bs:
                out.setdefault(b, []).append(a)
        return out
    rcg_e, rcg_d = rev(cg_e), rev(cg_d)

    # 앵커 = 지문이 확실히 맞은 것만(포함도 1.0). 불확실한 것을 앵커로 쓰면 오염이 번진다.
    emap = {r['rva']: r['mangled'] for r in rows if r['contain'] >= 0.99}
    # ⚠**이미 지문으로 배정된 주소는 전파가 건드리면 안 된다.**
    #   앵커(contain>=0.99)만 emap 에 넣었더니, 그보다 약한 지문 배정 주소가
    #   전파에서 다시 배정돼 **한 주소에 행이 두 개** 생겼다
    #   (실측: 0x140c872f0 에 `slot_cc_time_cached`(전파)와
    #    `available_cc_in_window`(지문)가 동시에 붙었다).
    assigned_e = {r['rva'] for r in rows}
    # ⚠**심볼 쪽도 마찬가지다.** `taken` 을 앵커만으로 채웠더니 약한 지문 배정이 쓴 심볼을
    #   전파가 다시 써서 **한 심볼이 주소 여러 개에 붙었다**(실측 63건).
    #   주소 중복만 막고 심볼 중복을 안 막은 것은 같은 실수의 반쪽짜리 수정이었다.
    already_d = {r['mangled'] for r in rows}
    anchors0 = len(emap)
    taken = set(emap.values()) | already_d
    e_mod = {int(k, 16): str(v.get('mod', '')).replace(chr(92), '/').split('/')[-1]
             for k, v in info.items() if isinstance(v, dict)}
    e_lines = {int(k, 16): {x for x in v.get('lines', []) if isinstance(x, int)}
               for k, v in info.items() if isinstance(v, dict)}

    def dsyms(rvas):
        return {sym_at[x] for x in rvas if x in sym_at}

    prop = 0
    for _round in range(8):
        added = 0
        # 앵커에 인접한 미매칭 exe 함수만 본다
        frontier = set()
        for r in list(emap):
            frontier.update(cg_e.get(r, ()))
            frontier.update(rcg_e.get(r, ()))
        frontier -= assigned_e
        frontier -= set(emap)
        # ★제안을 모아 **점수 순으로** 확정한다. 라운드 안에서 먼저 온 것부터 확정하면
        #   근거가 약한 쪽이 심볼을 선점한다(실측: `position_eval_at` 이 419B 짜리 엉뚱한
        #   함수에 붙고, 진짜 주인 0xd84db0 은 후보를 잃어 미매칭이 됐다).
        #   지문 단계에는 상호 최선 규율을 걸어 놓고 전파에는 안 걸었던 것이 실수다.
        props = []
        for E in sorted(frontier):
            CE = {emap[x] for x in cg_e.get(E, ()) if x in emap}
            RE = {emap[y] for y in rcg_e.get(E, ()) if y in emap}
            if not CE and not RE:
                continue
            mod = e_mod.get(E)
            # 후보 = 이웃 심볼의 호출자/피호출자
            cand = set()
            for s in CE:
                cand |= dsyms(rcg_d.get(rva_of.get(s, -1), ()))
            for s in RE:
                cand |= dsyms(cg_d.get(rva_of.get(s, -1), ()))
            cand -= taken
            scored = []
            for D in cand:
                if mod and src_of.get(D) and src_of[D] != mod:
                    continue
                dr = rva_of[D]
                CD = dsyms(cg_d.get(dr, ()))
                RD = dsyms(rcg_d.get(dr, ()))
                hit = len(CE & CD) + len(RE & RD)
                sc = hit / max(1, len(CE) + len(RE))
                # ★전파에도 **줄 증거**를 건다. 그래프 구조만 쓰면 같은 앵커에 붙은
                #   형제·단일화본을 못 가른다(실측: 전파분의 1위 일치율이 66% 로
                #   지문분보다 뚜렷이 낮았다 - 구조만으로는 갈리지 않는 짝들이다).
                dls = lines_of.get(D)
                els2 = e_lines.get(E)
                if els2 and dls:
                    ov = len(els2 & dls) / len(els2)
                    if ov == 0.0:
                        continue          # 줄이 하나도 안 겹치면 다른 함수다
                    sc *= 0.35 + 0.65 * ov
                if hit:
                    scored.append((sc, hit, D))
            if not scored:
                continue
            scored.sort(reverse=True)
            # ★유일할 때만 확정 - 동점이면 버린다(오염보다 미적용이 낫다).
            if len(scored) > 1 and scored[0][:2] == scored[1][:2]:
                continue
            sc, hit, D = scored[0]
            if sc < 0.5:
                continue
            props.append((sc, hit, E, D, mod))
        props.sort(key=lambda x: (-x[0], -x[1], x[2]))
        for sc, hit, E, D, mod in props:
            if E in emap or D in taken:
                continue
            emap[E] = D
            taken.add(D)
            assigned_e.add(E)
            rows.append({'addr': '0x%x' % (0x140000000 + E), 'rva': E, 'mangled': D,
                         'name': leaf(D), 'mod': mod or src_of.get(D, '?'),
                         'jaccard': round(sc, 3), 'contain': round(sc, 3),
                         'bytes': exe_fr.get(E, 0), 'via': 'callgraph'})
            added += 1
        prop += added
        if not added:
            break
    print('그래프 전파: 앵커 %d개 -> %d개 추가 확정' % (anchors0, prop))

    strong = [r for r in rows if r['jaccard'] >= a.minj]
    rows.sort(key=lambda r: -r['jaccard'])

    if a.selftest:
        # ⚠기대값은 **정확 일치**로 본다. 전엔 `exp in mangled` 였는데
        #   `position_eval_at` 이 `position_eval_at_uncached` 에 걸려 오답이 통과했다.
        # ★2026-09-10 정정: ~~0xd84db0 = position_eval_at_uncached~~ 는 **틀렸다**.
        #   .pdata 실측 크기 841B(래퍼) vs 30,055B(본체) + 호출관계로 확정 —
        #   0xd84db0(래퍼) 가 0xd851d0 을 부르고, 0xd851d0 의 호출자는 그 하나뿐이다.
        want = {0xd3fe50: 'nexus_final_stand_uncached', 0xd3e660: 'nexus_last_stand_uncached',
                0xd405d0: 'base_attacking_minion_uncached', 0xd57540: 'interaction_score',
                0xd851d0: 'position_eval_at_uncached', 0xd84db0: 'position_eval_at',
                0xd8ca70: 'position_risk_all_zero_near'}
        got = {r['rva']: r for r in rows}
        ok = True
        print('=== 자체 검증 ===')
        for rva, exp in sorted(want.items()):
            r = got.get(rva)
            if r is None:
                print('  FAIL  0x%-8x 미매칭 (기대 %s)' % (rva, exp))
                ok = False
            else:
                hit = r['name'] == exp
                ok &= hit
                print('  %s  0x%-8x j=%.2f 포함%.2f  %-34s %s' %
                      ('PASS' if hit else 'FAIL', rva, r['jaccard'], r['contain'],
                       r['name'][:34], '' if hit else '(기대 %s)' % exp))
        print('  => %s' % ('검증 통과' if ok else '검증 실패 - 쓰지 말 것'))
        print()

    json.dump(rows, open(OUT, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
    # ⚠집계는 **aimap 안/밖을 갈라서** 낸다. 그래프 전파는 aimap 640개 바깥의 함수에도
    #   이름을 붙이므로, 한 분모로 뭉치면 커버리지가 138% 같은 헛수치가 된다.
    inmap = {int(k, 16) for k in info}
    s_in = [r for r in strong if r['rva'] in inmap]
    s_out = [r for r in strong if r['rva'] not in inmap]
    byfp = sum(1 for r in s_in if r.get('via') != 'callgraph')
    bycg = len(s_in) - byfp
    print('aimap %d개 중 이름 확정 %d (%.1f%%)  [지문 %d + 그래프전파 %d]'
          % (len(inmap), len(s_in), 100.0 * len(s_in) / max(len(inmap), 1), byfp, bycg))
    print('  aimap 밖 덤으로 이름 붙은 함수 %d개 · 미확정: 후보없음 %d · 지문빈약 %d'
          % (len(s_out), nocand, weak))


main()
