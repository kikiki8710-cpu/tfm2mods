# -*- coding: utf-8 -*-
r"""aidiff — AI 판단함수 버전간 자동 대조기

    python MIG\aidiff.py <구exe> <신exe> [-o 리포트.md] [--all]

## 왜 만들었나
게임이 주 1회 업데이트되는데, 매번 "AI 판단에서 뭐가 바뀌었는지" 찾는 데 일주일이 걸렸다.
`MIG\` 에 `_sp1~36`·`_ai_*` 같은 일회용 스크립트가 60개 넘게 쌓인 게 그 증거다.
이 도구는 2026-09-05 세션에서 **사람이 손으로 5번 반복한 절차**를 그대로 자동화한 것이다.

## 핵심 원리 — 유사도가 아니라 **Rust 패닉 Location**
게임은 Rust 라 `core::panic::Location{ file:&str(ptr,len), line:u32, col:u32 }`(24B)가 `.rdata` 에 남는다.
함수가 참조하는 Location 집합 = **그 함수의 신원 지문**이고, 리링크·인라이닝·레지스터 재할당에 **불변**이다.

실측으로 확인된 것(2026-09-05):
  · 유사도는 **자매쌍에서 원리적으로 못 가른다** — `serpen_hunt` 의 1위가 `epic_hunt`(0.9327)이고
    정답은 0.5163 이었다. `epic_check` 도 자매(0.72)가 정답(0.53)보다 높았다.
  · 반대로 Location 은 `serpen_hunt.rs` vs `epic_hunt.rs` 로 **파일명이 직접 나와** 100% 갈린다.
  · 유사도 0.388~0.763 로 "대폭 변경"으로 보이던 함수 6개가 Location 행·열까지 완전 동일 =
    **리링크 코드젠 노이즈**였다. 반대로 `line_defense` 는 Location 이 실제로 줄어 **소스 삭제**였다.

## 판정 어휘
  IDENTICAL  소스 무변경(코드젠만 바뀜). 재현코드·문서 그대로 유효.
  SHIFTED    같은 파일의 **다른 곳**이 바뀌어 행번호만 밀림. 이 함수 자체는 무변경.
  EDITED     ★이 함수의 소스가 바뀜(행이 사라지거나 생김). **여기만 보면 된다.**
  DELETED    함수가 사라짐.   NEW  새로 생김.

## 출력
함수별 판정 + 사라진/생긴 소스 행 + 상수·필드오프셋·vtable 슬롯 변화 + 콜리 매핑.
⚠vtable 델타는 **한 종류가 아니다**(0.5.8 실측: WorldOps Δ+0x10, 엔티티 dyn Δ+0x8).
   그래서 이 도구는 델타를 가정하지 않고 **관측된 이동을 그대로 보고**한다.
"""
import argparse
import bisect
import collections
import hashlib
import io
import os
import pickle
import re
import struct
import sys

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')

try:
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64
    from capstone.x86 import X86_OP_MEM, X86_OP_IMM, X86_REG_RIP
except ImportError:
    sys.exit("capstone 이 필요하다:  pip install capstone")

BASE = 0x140000000
CACHE_VER = 6          # 캐시 포맷이 바뀌면 올린다


# ────────────────────────────────────────────────────────────── PE

class Img:
    def __init__(self, path):
        self.path = path
        d = self.data = open(path, 'rb').read()
        self.sha = hashlib.sha256(d).hexdigest()[:16]
        e = struct.unpack_from('<I', d, 0x3c)[0]
        n = struct.unpack_from('<H', d, e + 6)[0]
        ss = e + 24 + struct.unpack_from('<H', d, e + 20)[0]
        self.secs = []
        for i in range(n):
            o = ss + i * 40
            nm = d[o:o + 8].rstrip(b'\0').decode('latin1')
            va = struct.unpack_from('<I', d, o + 12)[0]
            vsz = struct.unpack_from('<I', d, o + 8)[0]
            rsz = struct.unpack_from('<I', d, o + 16)[0]
            pr = struct.unpack_from('<I', d, o + 20)[0]
            self.secs.append((nm, va, max(vsz, rsz), pr))
        opt = e + 24
        magic = struct.unpack_from('<H', d, opt)[0]
        dd = opt + (112 if magic == 0x20b else 96)
        self.pdata_rva, self.pdata_sz = struct.unpack_from('<II', d, dd + 3 * 8)
        # 함수 경계
        o = self.r2o(self.pdata_rva)
        fs = set()
        for k in range(self.pdata_sz // 12):
            b, en, _ = struct.unpack_from('<III', d, o + 12 * k)
            if en > b:
                fs.add((b, en))
        self.funcs = sorted(fs)
        self.starts = [x[0] for x in self.funcs]

    def r2o(self, r):
        for nm, va, sz, pr in self.secs:
            if va <= r < va + sz:
                return pr + (r - va)
        return None

    def code(self, r, n):
        o = self.r2o(r)
        return self.data[o:o + n] if o is not None else b''

    def owner(self, rva):
        i = bisect.bisect_right(self.starts, rva) - 1
        if i >= 0 and self.funcs[i][0] <= rva < self.funcs[i][1]:
            return self.funcs[i]
        return None


# ────────────────────────────────────────────── Rust panic Location

RS = re.compile(rb'[A-Za-z0-9_\\\-./]{3,120}\.rs')


def find_locations(img):
    """`.rdata` 의 Location{&str,len,line,col}(24B) 를 전수로 찾는다.

    반환 loc[rva] = (모듈경로, line, col)
    """
    d = img.data
    # 1) `.rs` 로 끝나는 문자열의 RVA 수집
    str_rva = {}
    for nm, va, sz, pr in img.secs:
        if nm not in ('.rdata', '.data', '.text'):
            continue
        blob = d[pr:pr + sz]
        for m in RS.finditer(blob):
            s = m.group(0)
            # 앞이 경로문자면 잘린 것 — 건너뛴다
            if m.start() > 0 and blob[m.start() - 1:m.start()] not in (b'\0', b'', b'"'):
                pass
            str_rva[va + m.start()] = s.decode('latin1')
    if not str_rva:
        return {}
    want = {BASE + r: (r, t) for r, t in str_rva.items()}
    # 2) 그 문자열을 가리키는 24B 구조체
    loc = {}
    for nm, va, sz, pr in img.secs:
        if nm not in ('.rdata', '.data'):
            continue
        blob = d[pr:pr + sz]
        for i in range(0, max(0, len(blob) - 24), 8):
            line, col = struct.unpack_from('<II', blob, i + 16)
            if line == 0 or line > 100000 or col == 0 or col > 1000:
                continue                      # line/col 부터 걸러야 아래가 싸다
            v = struct.unpack_from('<Q', blob, i)[0]
            ln = struct.unpack_from('<Q', blob, i + 8)[0]
            hit = want.get(v)
            if hit is not None and ln == len(hit[1]):
                loc[va + i] = (hit[1], line, col)
                continue
            # ★정규식이 시작점을 놓친 사본 — 포인터를 직접 따라가 확인한다.
            #   문자열이 NUL 없이 앞 문자열과 붙어 있으면(0.6.1 실측:
            #   `DM_NO_ACTIVE_DODGE` + `game-ai\src\…\passive_line.rs`)
            #   정규식이 앞 문자열부터 매치해 str_rva 의 키가 18바이트 앞이 되고,
            #   진짜 포인터는 want 에 없어 Location 이 통째로 누락된다.
            #   그 결과 그 사본만 참조하는 함수가 AI 계층에서 빠져 **삭제로 오판**된다.
            if not 4 < ln <= 120:
                continue
            s = img.code(v - BASE, ln)
            if len(s) != ln or not s.endswith(b'.rs'):
                continue
            if any(c < 0x20 or c > 0x7e for c in s):
                continue
            loc[va + i] = (s.decode('latin1'), line, col)
    return loc


# ─────────────────────────────────────────────────────────── census

def census(img, loc, only_prefix, verbose=False):
    """함수별 지문을 한 번의 디스어셈 패스로 수집.

    반환 fn[(b,e)] = dict(mods=..., lines=set, imms=Counter, offs=Counter,
                          vslots=Counter, callees=Counter, n=명령수)
    """
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = True
    out = {}
    tot = len(img.funcs)
    for idx, (b, e) in enumerate(img.funcs):
        if verbose and idx % 20000 == 0:
            print("   …%d/%d" % (idx, tot), file=sys.stderr)
        blob = img.code(b, e - b)
        if not blob:
            continue
        lines = set()
        lmods = collections.defaultdict(set)   # 모듈별 (line,col) — 모듈마다 시프트가 다를 수 있다
        mods = collections.Counter()
        imms = collections.Counter()
        offs = collections.Counter()
        vslots = collections.Counter()
        callees = collections.Counter()
        skel = []                              # 명령어 니모닉 열 = 코드 골격(짝짓기 잠금용)
        n = 0
        for ins in md.disasm(blob, BASE + b):
            n += 1
            mn = ins.mnemonic
            skel.append(mn)
            if mn == 'call' and ins.operands and ins.operands[0].type == X86_OP_IMM:
                callees[ins.operands[0].imm - BASE] += 1
            for op in ins.operands:
                if op.type == X86_OP_IMM:
                    v = op.imm
                    if abs(v) >= 3:                     # 0/1/2 는 잡음
                        imms[v] += 1
                elif op.type == X86_OP_MEM:
                    m = op.mem
                    if m.base == X86_REG_RIP:
                        t = ins.address + ins.size + m.disp - BASE
                        L = loc.get(t)
                        if L:
                            mods[L[0]] += 1
                            lines.add((L[1], L[2]))
                            lmods[L[0]].add((L[1], L[2]))
                    elif m.base and m.disp:
                        offs[m.disp] += 1
                        if mn == 'call':
                            vslots[m.disp] += 1
        if not mods:
            continue                                    # AI 계층 아님
        top = mods.most_common(1)[0][0]
        if only_prefix and only_prefix not in top:
            continue
        out[(b, e)] = dict(mods=mods, top=top, lines=lines, lmods=dict(lmods),
                           imms=imms, offs=offs, vslots=vslots, callees=callees, n=n,
                           skel=hashlib.md5(' '.join(skel).encode()).hexdigest())
    return out


def load(path, only_prefix, nocache=False):
    img = Img(path)
    cp = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                      '_aidiff_%s_%s_v%d.pkl' % (img.sha, hashlib.md5(
                          only_prefix.encode()).hexdigest()[:6], CACHE_VER))
    if not nocache and os.path.isfile(cp):
        with open(cp, 'rb') as f:
            return img, pickle.load(f)
    print("[*] %s 분석 중(최초 1회, 수분 소요)…" % os.path.basename(path), file=sys.stderr)
    loc = find_locations(img)
    print("    Location %d개" % len(loc), file=sys.stderr)
    fn = census(img, loc, only_prefix, verbose=True)
    print("    AI 계층 함수 %d개" % len(fn), file=sys.stderr)
    with open(cp, 'wb') as f:
        pickle.dump(fn, f)
    return img, fn


# ─────────────────────────────────────────────────────────── pairing

def shift_of(a, b):
    """두 (line,col) 집합이 균일 시프트로 겹치는지. (시프트, 겹친수) 최적값.

    ⚠**한 함수가 여러 모듈의 Location 을 참조**하는 일이 흔하다(본체 모듈 + utils.rs 등).
      모듈마다 시프트가 **다를 수 있으므로**(예: 본체 0, utils.rs −4) 전체를 한 덩어리로
      보면 멀쩡한 함수가 EDITED 로 오판된다. 그래서 호출측이 **모듈별로** 이 함수를 쓴다.
    """
    if not a or not b:
        return None, 0
    best, bn = None, 0
    cand = collections.Counter()
    for la, _ in a:
        for lb, _ in b:
            cand[lb - la] += 1
    for d, _ in cand.most_common(16):
        n = sum(1 for (l, c) in a if (l + d, c) in b)
        if n > bn:
            best, bn = d, n
    return best, bn


def by_mod(lines_by_mod):
    """{모듈: {(line,col)}} 로 재그룹."""
    return lines_by_mod


def mod_shifts(vo, vn):
    """모듈별 (시프트, 겹친수, 잃은행, 생긴행) 을 계산."""
    res = {}
    mods = set(vo['lmods']) | set(vn['lmods'])
    for m in sorted(mods):
        a = vo['lmods'].get(m, set())
        b = vn['lmods'].get(m, set())
        if not a:
            res[m] = (None, 0, [], sorted(b))
            continue
        if not b:
            res[m] = (None, 0, sorted(a), [])
            continue
        d, n = shift_of(a, b)
        sh = d or 0
        lost = sorted(x for x in a if (x[0] + sh, x[1]) not in b)
        born = sorted(x for x in b if (x[0] - sh, x[1]) not in a)
        res[m] = (d, n, lost, born)
    return res


def mod_overlap(vo, vn):
    """**모듈별** 시프트로 맞춘 Location 겹침 총합. (대표시프트, 겹침수)

    짝짓기 점수는 이 값을 쓴다. 전체를 한 덩어리로 시프트하면 본체 모듈과 utils 모듈의
    시프트가 달라 겹침이 과소평가되고, 엉뚱한 함수와 짝지어진다.
    """
    tot, rep = 0, None
    for m in set(vo['lmods']) | set(vn['lmods']):
        a = vo['lmods'].get(m, set())
        b = vn['lmods'].get(m, set())
        if not a or not b:
            continue
        d, n = shift_of(a, b)
        tot += n
        if m == vo['top']:
            rep = d
    return rep, tot


def pair(old, new):
    """모듈명 우선 + line 집합 시프트 정합으로 짝짓기."""
    by_mod_new = collections.defaultdict(list)
    for k, v in new.items():
        by_mod_new[v['top']].append(k)
    pairs, gone = {}, []
    used = set()
    # ★0순위 — **골격이 완전히 같은 쌍을 먼저 잠근다**(2026-09-23 신설).
    #   자매 함수들은 Location 집합이 서로 같아 겹침 점수가 동률이 되고, 크기 근접
    #   2순위도 자매끼리 뒤섞인다. 그 상태로 구 주소 순 탐욕 짝짓기를 돌리면
    #   **앞 함수가 뒤 함수의 쌍둥이를 먼저 채가** 한 줄로 밀리는 사슬이 생긴다
    #   (0.6.0→0.6.1 실측: free_dist 2522→2440·2440→1725 식으로 EDITED 9건 허위).
    #   골격 일치는 유사도가 아니라 **완전일치 지문**이라 자매쌍에서도 안전하다.
    lock = collections.defaultdict(list)
    for kn, vn in new.items():
        lock[(vn['top'], vn.get('skel'), kn[1] - kn[0])].append(kn)
    for ko, vo in sorted(old.items()):
        key = (vo['top'], vo.get('skel'), ko[1] - ko[0])
        c = [k for k in lock.get(key, []) if k not in used]
        if len(c) == 1:
            used.add(c[0])
            d, n = mod_overlap(vo, new[c[0]])
            pairs[ko] = (c[0], d, n)
    # ★나머지는 **전역 최선순**으로 붙인다(2026-09-23).
    #   구 주소순 탐욕이면 **먼저 온 구 함수가 뒤 함수의 더 좋은 짝을 채간다**
    #   (0.6.0→0.6.1 실측: around 533B 가 1778B 를 먼저 물어, 진짜 짝인 1780B 가
    #   「삭제」로 밀렸다). 후보쌍을 전부 점수화해 **높은 점수부터** 확정한다.
    cand = []
    for ko, vo in sorted(old.items()):
        if ko in pairs:
            continue
        for kn in by_mod_new.get(vo['top'], []):
            if kn in used:
                continue
            d, n = mod_overlap(vo, new[kn])
            # 겹친 Location 수가 1순위, 크기 근접이 2순위
            sc = n * 1000 - min(999, abs((ko[1] - ko[0]) - (kn[1] - kn[0])) // 16)
            cand.append((sc, ko, kn, d, n))
    cand.sort(key=lambda x: (-x[0], x[1], x[2]))
    for sc, ko, kn, d, n in cand:
        if ko in pairs or kn in used:
            continue
        used.add(kn)
        pairs[ko] = (kn, d, n)
    for ko in sorted(old):
        if ko not in pairs:
            gone.append(ko)
    added = [k for k in new if k not in used]
    return pairs, gone, added


def verdict(vo, vn):
    """소스 변경 판정 — ★**모듈별** 시프트로 본다.

    한 함수가 본체 모듈과 `utils.rs` 등 **여러 모듈**의 Location 을 참조하는 일이 흔하고,
    모듈마다 행 시프트가 다르다(본체 +0, utils −4 처럼). 전체를 한 덩어리로 시프트하면
    한쪽이 안 맞아 **소스가 안 바뀐 함수가 EDITED 로 오판**된다.

    반환 lost/born 은 `(모듈, (행, 열))` 목록이다.
    """
    ms = mod_shifts(vo, vn)
    lost_all, born_all, shifts = [], [], {}
    for m, (d, n, lost, born) in ms.items():
        lost_all += [(m, x) for x in lost]
        born_all += [(m, x) for x in born]
        shifts[m] = d
    if lost_all or born_all:
        return 'EDITED', (sorted(lost_all), sorted(born_all))
    ds = set(v for v in shifts.values() if v is not None)
    if not ds or ds == {0}:
        return 'IDENTICAL', []
    if len(ds) == 1:
        return 'SHIFTED(%+d)' % ds.pop(), []
    # 모듈마다 시프트가 다르지만 잃은 행도 생긴 행도 없다 = 소스는 그대로, 위치만 밀림
    return 'SHIFTED(%s)' % ', '.join(
        '%s%+d' % (m.split(chr(92))[-1], shifts[m])
        for m in sorted(shifts) if shifts[m] is not None), []


def dcount(a, b):
    """Counter 차이 — (값, 구, 신) 목록."""
    out = []
    for k in sorted(set(a) | set(b)):
        if a.get(k, 0) != b.get(k, 0):
            out.append((k, a.get(k, 0), b.get(k, 0)))
    return out


# ─────────────────────────────────────────────────────────── report

def main():
    ap = argparse.ArgumentParser(description='AI 판단함수 버전간 자동 대조')
    ap.add_argument('old')
    ap.add_argument('new')
    ap.add_argument('-o', '--out', default=None)
    ap.add_argument('-p', '--prefix', default='game-ai',
                    help='이 문자열을 포함한 모듈만 (기본 game-ai)')
    ap.add_argument('--all', action='store_true', help='IDENTICAL 도 전부 출력')
    ap.add_argument('--nocache', action='store_true')
    a = ap.parse_args()

    io_, fo = load(a.old, a.prefix, a.nocache)
    inn, fn = load(a.new, a.prefix, a.nocache)
    pairs, gone, added = pair(fo, fn)

    L = []
    w = L.append
    w('# AI 판단함수 버전 대조')
    w('')
    w('| | 구 | 신 |')
    w('|---|---|---|')
    w('| exe | `%s` | `%s` |' % (os.path.basename(a.old), os.path.basename(a.new)))
    w('| sha256[:16] | `%s` | `%s` |' % (io_.sha, inn.sha))
    w('| AI 계층 함수 | %d | %d |' % (len(fo), len(fn)))
    w('')

    rows = []
    for ko, (kn, d, n) in pairs.items():
        v, extra = verdict(fo[ko], fn[kn])
        rows.append((v, ko, kn, d, extra))

    # ★MERGED — **인라인 병합은 1:1 짝짓기로 표현이 안 된다**(2026-09-23 신설).
    #   구 함수 둘이 신 함수 하나로 인라인되면 한쪽만 짝이 붙고 나머지는 「사라짐」으로,
    #   짝이 붙은 쪽은 상대의 Location 을 통째로 얻어 **EDITED 로 오판**된다
    #   (0.6.0→0.6.1 실측: around 636B+533B → 1030B 가 Location 2+3=5 로 정확히 합산).
    #   그래서 「생긴 행」이 사라진 함수의 Location 과 정확히 맞아떨어지면 병합으로 본다.
    absorbed = {}
    for i, (v, ko, kn, d, extra) in enumerate(rows):
        if v != 'EDITED' or not extra:
            continue
        lost, born = extra
        if lost or not born:
            continue
        rest, eaten = set(born), []
        for kg in gone:
            g = set()
            for m, ls in fo[kg]['lmods'].items():
                g |= {(m, x) for x in ls}
            if g and g <= rest:
                rest -= g
                eaten.append(kg)
        if eaten and not rest:
            rows[i] = ('MERGED', ko, kn, d, ([], born))
            for kg in eaten:
                absorbed[kg] = kn
    if absorbed:
        gone = [k for k in gone if k not in absorbed]
    order = {'EDITED': 0, 'MERGED': 1, 'SHIFTED': 2, 'IDENTICAL': 3}
    rows.sort(key=lambda r: (order.get(r[0].split('(')[0], 9), fo[r[1]]['top']))

    ned = sum(1 for r in rows if r[0] == 'EDITED')
    w('## 요약')
    w('')
    w('- ★**소스가 바뀐 함수(EDITED) = %d개** ← 이번 버전에서 실제로 볼 곳' % ned)
    w('- 행만 밀림(SHIFTED) = %d개 · 소스 무변경(IDENTICAL) = %d개'
      % (sum(1 for r in rows if r[0].startswith('SHIFTED')),
         sum(1 for r in rows if r[0] == 'IDENTICAL')))
    w('- 인라인 병합(MERGED) = %d개 · 사라짐 = %d개 · 새로 생김 = %d개'
      % (sum(1 for r in rows if r[0] == 'MERGED'), len(gone), len(added)))
    if absorbed:
        w('')
        w('> MERGED = 구 함수가 **신 함수 안으로 인라인**된 것이다(Location 이 정확히 합산).')
        w('> 소스 변경이 아니므로 EDITED 와 섞지 마라. 흡수된 구 함수: %s'
          % ', '.join('`%#x`→`%#x`' % (k[0], v[0]) for k, v in sorted(absorbed.items())))
    w('')
    w('> 판정은 **Rust 패닉 Location**(모듈·행·열) 기준이다. 유사도는 자매쌍에서 원리적으로')
    w('> 오답을 내므로 쓰지 않는다(2026-09-05 실측: serpen_hunt 의 유사도 1위가 epic_hunt).')
    w('')

    if gone:
        w('## 사라진 함수')
        w('')
        for k in gone:
            w('- `0x%x` %s (%d B) — Location %d개' % (k[0], fo[k]['top'], k[1] - k[0], len(fo[k]['lines'])))
        w('')
    if added:
        w('## 새로 생긴 함수')
        w('')
        for k in added:
            w('- `0x%x` %s (%d B)' % (k[0], fn[k]['top'], k[1] - k[0]))
        w('')

    w('## 함수별')
    w('')
    for v, ko, kn, d, extra in rows:
        if v == 'IDENTICAL' and not a.all:
            continue
        vo, vn = fo[ko], fn[kn]
        w('### %s — `0x%x` → `0x%x`   **%s**' % (vo['top'], ko[0], kn[0], v))
        w('')
        w('- 크기 %d → %d B · 명령 %d → %d' % (ko[1] - ko[0], kn[1] - kn[0], vo['n'], vn['n']))
        if v == 'EDITED':
            lost, born = extra
            if lost:
                w('- ★**사라진 소스 행**: %s' % ', '.join(
                    '%s:%d:%d' % (m.split(chr(92))[-1], x[0], x[1]) for m, x in lost[:24]))
            if born:
                w('- ★**생긴 소스 행**: %s' % ', '.join(
                    '%s:%d:%d' % (m.split(chr(92))[-1], x[0], x[1]) for m, x in born[:24]))
        di = dcount(vo['imms'], vn['imms'])
        big = [x for x in di if abs(x[0]) >= 16]
        if big:
            w('- 상수 변화 %d종:' % len(big))
            for k_, o_, n_ in big[:18]:
                w('    - `0x%x`(%d) %d → %d회' % (k_ & 0xffffffffffffffff, k_, o_, n_))
        dv = dcount(vo['vslots'], vn['vslots'])
        if dv:
            w('- ★vtable 간접호출 슬롯 변화(델타를 가정하지 않고 관측 그대로):')
            for k_, o_, n_ in dv:
                w('    - `+0x%x` %d → %d회' % (k_, o_, n_))
        w('')

    txt = '\n'.join(L)
    if a.out:
        io.open(a.out, 'w', encoding='utf-8', newline='\n').write(txt)
        print("리포트: %s  (EDITED %d개)" % (a.out, ned))
    else:
        print(txt)


if __name__ == '__main__':
    main()
