# -*- coding: utf-8 -*-
r"""addrbook.py — 전 주소록(매니페스트 전 지점 + KB 수확 행)을 한 장으로 만들고, 새 exe 로 한 번에 대응(SAME/MOVED/FN_EDITED/AMBIGUOUS/MISSING)한다.

왜 (2026-10-27 · 0.6.5 → 다음 패치 대비):
    패치마다 "우리가 아는 주소"가 매니페스트 13모드 · 상세전술 322 · KB 문서 곳곳에 흩어져 있어
    모드별로 repin/sitealign/수동판정을 따로 돌렸다. 이 도구는 그 전부를 **한 장(addrbook)** 으로 모으고,
    새 exe 가 오면 행마다 신 주소와 **바뀐 방식**을 기계로 판정한다.

사용 (CLI 계약 — 바꾸지 말 것):
  python -X utf8 MIG\patchday\addrbook.py build --ver 0.6.5 --exe <exe> --pkl <fnidx.pkl> --cg <cg.pkl>
         [--extra <rows.json> ...] --out MIG\patchday\addrbook_0.6.5.json
         [--no-manifests] [--from-repin-map <map.json> --side old|new]      # 리허설용(구버전 주소록 임시 생성)
         [--from-pairs <probe_targets.json|overrides.json|{구:신}.json> --side old|new]   # 리허설용(tactics_verify 정답)
         (매니페스트 = MIG\manifest\*.json 중 ver == --ver · ignore/NOT_RVA 제외 · --extra = kb_harvest.py 산출 · exe_sha256 일치 필수)
  python -X utf8 MIG\patchday\addrbook.py map --book <addrbook.json> --old-exe <구exe> --new-exe <신exe>
         --new-ver <라벨> --old-pkl P --new-pkl P --old-cg P --new-cg P --out <출력폴더>
         → addrmap.json · addrmap.md · review.json
         종료코드: 0 = 전 행 자동 대응(SAME/MOVED_BYTES_CHANGED) · 1 = 검토 필요 행 있음(정상) · 2 = 실행 오류
  python -X utf8 MIG\patchday\addrbook.py grade --addrmap <addrmap.json> --truth <json> [--truth-format repinmap|pairs] [--out g.json]
         → 정답지(사람 확정 재핀표)와 대조: 자동 행 오답 수 · 자동 대응률 · 검토 행 수 (형식 자동 판별:
           repin 맵 / tactics_verify probe_targets(sites[].migrated_from) / overrides({"sites":{구:신|[신…]|null}}) / {구:신})
           종료코드 0 = 자동 오답 0 · 1 = 자동 오답 있음 · 2 = 실행 오류

리허설 (2026-10-27 실측 = MIG\patchday\rehearsal_addrbook.md):
  build --ver 0.6.4 --exe <0.6.4> … --from-repin-map MIG\repin_map_065.final.json --side old --out <tmp>\book_064.json
  map --book <tmp>\book_064.json --old-exe <0.6.4> --new-exe <0.6.5> … → grade --truth MIG\repin_map_065.final.json

등급 (행 단위):
  SAME                 신 주소 확정 + 12B 동일 (이동만)
  MOVED_BYTES_CHANGED  신 주소 확정 + 12B 다름 → change 필드 = reloc/reg/stack/disp/imm/insn (imm = 게임 기본값 변경 후보)
  FN_EDITED            owner 함수의 패닉 Location 집합이 바뀜(aidiff EDITED) — 후보 주소는 주지만 사람 검토
  AMBIGUOUS            후보가 여럿(사본 분기 포함)이거나 증거가 한 갈래뿐/충돌 — 사람 검토
  MISSING              후보 없음(소실·로직 변경 의심)

판별 엔진 (전부 기존 엔진 재사용 — 새 지문 발명 없음):
  함수(owner/함수시작) : R = repin.build_match 의 match_fn(skel→head→콜그래프) · L = 패닉 Location 집합(aidiff 판정 규칙:
                         정확 일치 / 모듈별 균일 행시프트 / 부분 겹침) · X = 사영 caller 들의 공통 callee · W = 주소순 이웃 창
                         · P = 전단사 caller 의 같은 서수 call 타깃 · V = .rdata 포인터 슬롯(vtable) 사영(repin rdata 표 투표)
                         · Z = 전단사 앵커 사이 블록이 같은 간격·같은 skel 배치로 통째 이동(+K = 자기 skel 일치)
                         · K = 채택 후보가 skel/Location/공통 callee 후보 집합 안(약)
                         + README 0.6.0 채택 규칙(콜러 수 · callee 사영 · size 비 3종 중 2종) 게이트
  .pdata 없는 leaf     : XC = 호출 명령을 owner 정렬로 옮겨 신 call 타깃 투표 · G · V · Z(앞뒤 앵커 블록 · leaf 바이트 동일)
  함수 중간(MID/RET)   : A = owner 안 바이트 유일검색(repin.locate_in_fn 방식) · U = 형태키 n-gram 유일(레지스터·즉치 무시)
                         · B = owner 명령열 정렬(sitealign 방식 · difflib) · C = 콜타깃 대조(직전/현 명령이 call 이면 피호출 함수 재핀값과 일치)
                         · G = 전역 바이트 유일검색(12→64B 연장, 20B 미만 유일은 WEAK) · O = 동일오프셋+같은 명령(약)
                         · 사본 분기 = 구 owner 에서 유일한 바이트/긴 꼴/「즉치 보존·레지스터 지움」 서명이 신 owner 에 여럿 → AMBIGUOUS
                         · 명령 안 지점(즉치·변위 바이트)은 신 명령의 니모닉·길이가 같을 때만 채택
  .pdata 끝 상수(FN_END): owner 대응 + 신 크기 (함수 시작이자 앞 함수 끝인 값은 이름이 HI/END/끝 이면 두 해석이 갈릴 때 검토)
  .rdata/.data         : G · T = repin rdata 표 투표(함수 포인터 사영) · X = 코드 xref 사영(참조 명령을 정렬로 옮겨 rip 타깃을 읽음 ·
                         정확 참조가 있으면 그것만) · Y = 앞뒤 4KB 안 코드 참조 앵커를 X 로 옮기고 앵커~지점 내용이 같으면 같은 상대 위치
  채택 = 충돌 0 + 강한 증거 2갈래(또는 강1+약1). 바이트 유일(G)만으로는 채택하지 않는다(README 0.6.0: BYTES_UNIQUE 는 후보).
"""
import argparse
import bisect
import collections
import difflib
import glob
import hashlib
import io
import json
import os
import pickle
import re
import struct
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
MIGD = os.path.dirname(HERE)
sys.path.insert(0, MIGD)

import mig_verify as MV      # noqa: E402  매니페스트 위치·로더
import repin as RP           # noqa: E402  match_fn / locate_in_fn 엔진
import midpin as MP          # noqa: E402  Img(fn 인덱스·콜그래프)
import sitealign as SA       # noqa: E402  레지스터 지운 키(_REG)
import aidiff as AD          # noqa: E402  Location 판정(verdict/mod_overlap)

from capstone import Cs, CS_ARCH_X86, CS_MODE_64              # noqa: E402
from capstone.x86 import X86_OP_IMM, X86_OP_MEM, X86_OP_REG, X86_REG_RIP  # noqa: E402

try:
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')
except Exception:
    pass

BASE = 0x140000000
CACHE = os.path.join(MIGD, '_patchday', '_tmp', '_cache')
TOOL_VER = 1
MAND = MV.MAND

_MD = Cs(CS_ARCH_X86, CS_MODE_64)
_MD.detail = False
_MDD = Cs(CS_ARCH_X86, CS_MODE_64)
_MDD.detail = True

RE_NUM = re.compile(r'0x[0-9a-f]+|\b\d+\b')
KIND_RET = {'RETURN_SITE', 'INLINE_RETADDR'}
YSPAN = 4096
END_NAME = re.compile(r'(?:_HI|_END|END_|_LIMIT|_UPPER)|끝|end|_hi', re.I)


def log(*a):
    print(*a, flush=True)


def H(v):
    return None if v is None else '0x%x' % v


def P(s):
    return int(s, 16) if isinstance(s, str) else int(s)


# ═══════════════════════════════════════════════════════════════ exe 문맥

class Exe:
    """midpin.Img(fn 인덱스·콜그래프) + 패닉 Location 인덱스 + 명령 캐시."""

    def __init__(s, exe, pkl, cg, tag):
        t0 = time.time()
        s.path, s.tag = exe, tag
        s.I = MP.Img(exe, pkl, cg)
        s.raw = s.I.raw
        s.sha = hashlib.sha256(s.raw).hexdigest().upper()
        s.sha16 = s.sha[:16]
        s.fn = s.I.fn
        s.starts = s.I.starts
        s.cal = s.I.cal            # callee
        s.calr = s.I.calr          # caller
        s.secs = s.I.secs          # (nm, va, max(vsz,rsz), rr, rsz)
        pe = struct.unpack_from('<I', s.raw, 0x3C)[0]
        nsec = struct.unpack_from('<H', s.raw, pe + 6)[0]
        st = pe + 24 + struct.unpack_from('<H', s.raw, pe + 20)[0]
        s.I_vsz = {}               # 섹션 이름 → 진짜 VirtualSize (secs 의 3번째 값은 max(vsz,rsz))
        for i in range(nsec):
            o = st + i * 40
            s.I_vsz[s.raw[o:o + 8].rstrip(b'\x00').decode(errors='replace')] = struct.unpack_from('<I', s.raw, o + 8)[0]
        s._ins = {}
        s._gaps = {}
        s.leaves = {}
        s._ng = {}
        s.fnloc = None
        log('[%s] %s  sha %s  함수 %d  (%.0fs)' % (tag, os.path.basename(os.path.dirname(exe)) or exe,
                                              s.sha16, len(s.fn), time.time() - t0))

    # ── 기본
    def read(s, rva, n):
        return s.I.read(rva, n) or b''

    def owner(s, rva):
        return s.I.owner(rva)

    def sect(s, rva):
        for nm, va, vsz, rr, rsz in s.secs:
            if va <= rva < va + vsz:
                return nm
        return '?'

    def sec(s, name):
        for x in s.secs:
            if x[0] == name:
                return x
        return None

    def end(s, f):
        return f + s.fn[f]['size']

    def find_all(s, pat, sects=None, limit=8):
        hits = []
        for nm, va, vsz, rr, rsz in s.secs:
            if sects and nm not in sects:
                continue
            blob = s.raw[rr:rr + rsz]
            i = 0
            while True:
                i = blob.find(pat, i)
                if i < 0:
                    break
                hits.append(va + i)
                i += 1
                if len(hits) > limit:
                    return hits
        return hits

    # ── 명령열 (addr, mnem, op_str, size) + 키 2종
    def fsize(s, f):
        if f in s.fn:
            return s.fn[f]['size']
        return s.leaves.get(f, 0)

    def is_fn(s, f):
        return f in s.fn or f in s.leaves

    def prev_fn(s, rva):
        """rva 바로 앞(시작 < rva)의 fnidx 함수."""
        i = bisect.bisect_left(s.starts, rva) - 1
        return s.starts[i] if i >= 0 else None

    def fn_end_of(s, rva):
        """rva == (앞 함수 시작 + 크기) 이면 그 함수(.pdata 끝 상수 — comptest CT_REGION_HI 류)."""
        p = s.prev_fn(rva)
        if p is not None and p + s.fn[p]['size'] == rva:
            return p
        return None

    def leaf_of(s, rva):
        """fnidx(.pdata) 밖 .text 의 leaf 함수 (시작, 끝). 틈 [앞 끝, 다음 시작) 을 CC 패딩·종결(ret/jmp) 기준으로 쪼갠다
        (kb_harvest.Exe.classify 와 같은 규칙 · ui_kit RVA_DROP 류). 패딩 안이면 None."""
        if s.sect(rva) != '.text' or s.owner(rva) is not None:
            return None
        i = bisect.bisect_right(s.starts, rva) - 1
        g0 = s.starts[i] + s.fn[s.starts[i]]['size'] if i >= 0 else s.sec('.text')[1]
        g1 = s.starts[i + 1] if i + 1 < len(s.starts) else s.sec('.text')[1] + s.sec('.text')[2]
        key = (g0, g1)
        L = s._gaps.get(key)
        if L is None:
            L = []
            code = s.read(g0, g1 - g0)
            p = 0
            while p < len(code):
                while p < len(code) and code[p] in (0xCC, 0x90, 0x00):
                    p += 1
                if p >= len(code):
                    break
                st = p
                end = None
                for a, sz, mn, op in _MD.disasm_lite(code[p:], g0 + p):
                    q = a + sz - g0
                    if mn in ('ret', 'retf', 'jmp') and q < len(code) and code[q] == 0xCC:
                        end = q
                        break
                    if q >= len(code):
                        end = q
                        break
                if end is None:          # 해독 실패 — 남은 틈 전체를 한 덩어리로
                    end = len(code)
                L.append((g0 + st, g0 + end))
                p = end
            s._gaps[key] = L
            for a, b in L:
                s.leaves[a] = b - a
        for a, b in L:
            if a <= rva < b:
                return a, b
        return None

    def owner2(s, rva):
        """fnidx owner, 없으면 leaf 시작."""
        o = s.owner(rva)
        if o is not None:
            return o
        lf = s.leaf_of(rva)
        return lf[0] if lf else None

    def insns(s, f):
        r = s._ins.get(f)
        if r is None:
            sz = s.fsize(f)
            code = s.read(f, sz)
            L = [(i.address, i.mnemonic, i.op_str, i.size) for i in _MD.disasm(code, f)]
            tok = ['%s|%s' % (m, RE_NUM.sub('I', o)) for _, m, o, _ in L]
            shp = ['%s|%s' % (m, SA._REG.sub('R', RE_NUM.sub('I', o))) for _, m, o, _ in L]
            addrs = [a for a, _, _, _ in L]
            r = (L, tok, shp, addrs)
            s._ins[f] = r
        return r

    def insn_at(s, f, rva):
        """함수 f 안에서 rva 를 덮는 명령 index (없으면 None)."""
        L, _, _, addrs = s.insns(f)
        i = bisect.bisect_right(addrs, rva) - 1
        if i >= 0 and L[i][0] <= rva < L[i][0] + L[i][3]:
            return i
        return None

    def dis1(s, rva, n=1):
        out = []
        for i in _MDD.disasm(s.read(rva, 16 * n), rva):
            out.append(i)
            if len(out) >= n:
                break
        return out

    # ── 패닉 Location 인덱스 (mig060 의 lea 스캔 + aidiff 의 24B 구조체·포인터 추종 규칙)
    def try_loc(s, t):
        if s.sect(t) not in ('.rdata', '.data'):
            return None
        b = s.read(t, 24)
        if len(b) < 24:
            return None
        p, ln, line, col = struct.unpack('<QQII', b)
        if not (4 < ln <= 200 and 0 < line < 100000 and 0 < col < 1000):
            return None
        st = s.read(p - BASE, ln) if BASE <= p < BASE + 0x10000000 else b''
        if len(st) != ln or not st.endswith(b'.rs') or any(c < 0x20 or c > 0x7e for c in st):
            return None
        return (st.decode('latin1'), line, col)

    def load_locs(s):
        if s.fnloc is not None:
            return
        os.makedirs(CACHE, exist_ok=True)
        cp = os.path.join(CACHE, 'locidx_%s_v%d.pkl' % (s.sha16, TOOL_VER))
        if os.path.isfile(cp):
            s.fnloc = pickle.load(open(cp, 'rb'))
        else:
            t0 = time.time()
            nm, va, vsz, rr, rsz = s.sec('.text')
            buf = s.raw[rr:rr + rsz]
            locs, fnloc = {}, collections.defaultdict(list)
            for m in re.finditer(rb'[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]', buf):
                i = m.start()
                if i + 7 > len(buf):
                    break
                tgt = va + i + 7 + struct.unpack_from('<i', buf, i + 3)[0]
                if tgt in locs:
                    L = locs[tgt]
                else:
                    L = s.try_loc(tgt)
                    locs[tgt] = L
                if L is None:
                    continue
                o = s.owner(va + i)
                if o is not None:
                    fnloc[o].append(L)
            s.fnloc = {k: tuple(sorted(v)) for k, v in fnloc.items()}
            pickle.dump(s.fnloc, open(cp, 'wb'), 4)
            log('[%s] Location 인덱스 %d함수 (%.0fs) -> %s' % (s.tag, len(s.fnloc), time.time() - t0, cp))
        s.ex_idx = collections.defaultdict(list)
        s.fc_idx = collections.defaultdict(list)
        s.it_idx = collections.defaultdict(set)
        for f, L in s.fnloc.items():
            s.ex_idx[L].append(f)
            fc = tuple(sorted((a, c) for a, _, c in L))
            s.fc_idx[fc].append(f)
            for a, _, c in set(L):
                s.it_idx[(a, c)].add(f)

    def locview(s, f):
        """aidiff.verdict 가 받는 꼴 {'lmods':{모듈:{(행,열)}}, 'top':모듈}."""
        L = s.fnloc.get(f, ())
        lm = collections.defaultdict(set)
        cnt = collections.Counter()
        for a, l, c in L:
            lm[a].add((l, c))
            cnt[a] += 1
        return {'lmods': dict(lm), 'top': cnt.most_common(1)[0][0] if cnt else None}


# ═══════════════════════════════════════════════════════════════ xref 색인 (rel32 필드 → 대상)

def xref_scan(X, code_targets, data_ranges):
    """X 의 .text 전수에서 rel32/disp32 필드가 가리키는 곳을 numpy 로 계산해
    code_targets(정확 일치) · data_ranges(구간) 에 걸리는 것만 명령 해독으로 확인한다.
    반환 {대상rva: [(명령rva, 명령크기, 니모닉), ...]}  (data 는 실제 타깃 rva 키)"""
    import numpy as np
    nm, va, vsz, rr, rsz = X.sec('.text')
    buf = X.raw[rr:rr + rsz]
    qc = np.array(sorted(code_targets), dtype=np.int64)
    rng = sorted(data_ranges)
    merged = []
    for lo, hi in rng:
        if merged and lo <= merged[-1][1]:
            merged[-1][1] = max(merged[-1][1], hi)
        else:
            merged.append([lo, hi])
    rs = np.array([a for a, _ in merged], dtype=np.int64)
    re_ = np.array([b for _, b in merged], dtype=np.int64)
    hits = []          # (disp 필드 위치 rva, t = 필드끝 + disp)
    for r in range(4):
        cnt = (len(buf) - r) // 4
        v = np.frombuffer(buf, dtype='<i4', count=cnt, offset=r).astype(np.int64)
        pos = va + r + 4 * np.arange(cnt, dtype=np.int64)
        t = pos + 4 + v
        m = np.zeros(cnt, dtype=bool)
        if len(qc):
            m |= np.isin(t, qc)
        if len(rs):
            # 데이터 참조는 뒤에 즉치(1/2/4B)가 붙을 수 있어 실제 타깃 = t + k (k∈{0,1,2,4}) → 구간을 4B 넓혀 거른다
            j = np.searchsorted(rs, t + 4, side='right') - 1
            ok = j >= 0
            jj = np.where(ok, j, 0)
            m |= ok & (t >= rs[jj] - 4) & (t <= re_[jj])
        idx = np.nonzero(m)[0]
        hits += list(zip((pos[idx]).tolist(), (t[idx]).tolist()))
        del v, pos, t, m
    out = collections.defaultdict(list)
    seen = set()
    cset = set(code_targets)
    for fpos, t in hits:
        o = X.owner2(fpos)
        if o is None:
            continue
        k = X.insn_at(o, fpos)
        if k is None:
            continue
        a, mn, ops, sz = X.insns(o)[0][k]
        if (a, ) in seen:
            continue
        ins = X.dis1(a)
        if not ins or ins[0].address != a:
            continue
        ins = ins[0]
        tgt = None
        for op in ins.operands:
            if op.type == X86_OP_MEM and op.mem.base == X86_REG_RIP:
                tgt = a + ins.size + op.mem.disp
            elif op.type == X86_OP_IMM and ins.mnemonic in ('call', 'jmp') or (
                    op.type == X86_OP_IMM and ins.mnemonic.startswith('j')):
                tgt = op.imm
        if tgt is None:
            continue
        good = tgt in cset or any(lo <= tgt < hi for lo, hi in merged if lo - 8 <= tgt <= hi + 8)
        if not good:
            continue
        seen.add((a, ))
        out[tgt].append((a, ins.size, ins.mnemonic))
    return out


# ═══════════════════════════════════════════════════════════════ 대응 엔진

class Mapper:
    def __init__(s, O, N):
        s.O, s.N = O, N
        O.load_locs()
        N.load_locs()
        cgO = {'callee': O.cal, 'caller': O.calr}
        cgN = {'callee': N.cal, 'caller': N.calr}
        s.match_fn, s.match_mid = RP.build_match(O.I, N.I, cgO, cgN)
        # 전역 대응맵 — repin.cmd_rdata 와 같은 규칙(skel 유일 / head 유일 / size 로 유일)
        s.GM, s.GMb = {}, {}
        byss = collections.defaultdict(list)          # (skel, size) → 신 함수 (skel 다수 후보의 size 유일 판정을 O(1) 로)
        for x, g in N.fn.items():
            byss[(g['skel'], g['size'])].append(x)
        for rva, f in O.fn.items():
            c = N.I.by_skel.get(f['skel'], [])
            if len(c) == 1:
                s.GM[rva] = c[0]
                if len(O.I.by_skel.get(f['skel'], [])) == 1:
                    s.GMb[rva] = c[0]          # 양쪽 다 유일 = 전단사
                continue
            if not c:
                h = N.I.by_head.get(f['head'], [])
                if len(h) == 1:
                    s.GM[rva] = h[0]
                continue
            same = byss.get((f['skel'], f['size']), ())
            if len(same) == 1:
                s.GM[rva] = same[0]
        s.GMb_keys = sorted(s.GMb)
        log('[map] 전역 대응맵 %d (전단사 %d) / 구 함수 %d' % (len(s.GM), len(s.GMb), len(O.fn)))
        s._fn = {}
        s._sm = {}
        s._rslot = None
        s.XRk = []
        s._zblk = {}
        s.XR = {}

    # ── 3종 게이트 (README 0.6.0: 콜러 수 · callee 사영 · size 비 — 3종 중 2종)
    def votes(s, o, c):
        O, N = s.O, s.N
        co, cn = len(O.calr.get(o, [])), len(N.calr.get(c, []))
        if co == 0 and cn == 0:
            v1 = None
        else:
            v1 = (co == cn) or (co > 0 and cn > 0 and abs(co - cn) <= max(1, round(0.15 * max(co, cn))))
        pj = {s.GM[x] for x in O.cal.get(o, []) if x in s.GM}
        if pj:
            hit = len(pj & set(N.cal.get(c, [])))
            v2 = hit / len(pj) >= 0.5
        else:
            hit, v2 = 0, None
        so, sn = O.fsize(o), N.fsize(c)
        v3 = (min(so, sn) / max(so, sn, 1)) >= 0.85
        vs = [v1, v2, v3]
        return {'ok': sum(1 for v in vs if v is True), 'bad': sum(1 for v in vs if v is False),
                'callers': [co, cn], 'callee_proj': [hit, len(pj)], 'size': [so, sn]}

    # ── Location 엔진
    def loc_match(s, o):
        O, N = s.O, s.N
        L = O.fnloc.get(o)
        if not L:
            return {'kind': 'NOLOC', 'cands': []}
        c = N.ex_idx.get(L, [])
        if c:
            return {'kind': 'EXACT', 'cands': list(c), 'uniq_old': len(O.ex_idx.get(L, [])) == 1}
        fc = tuple(sorted((a, cc) for a, _, cc in L))
        vo = O.locview(o)
        good = []
        for f2 in N.fc_idx.get(fc, []):
            v, _ = AD.verdict(vo, N.locview(f2))
            if v != 'EDITED':
                good.append(f2)
        if good:
            return {'kind': 'SHIFT', 'cands': good, 'uniq_old': len(O.fc_idx.get(fc, [])) == 1}
        # 부분 겹침(소스 편집) — aidiff.mod_overlap 점수
        pool = collections.Counter()
        for key in set((a, cc) for a, _, cc in L):
            for f2 in N.it_idx.get(key, ()):
                pool[f2] += 1
        nset = len(set(L))
        sc = []
        for f2, _ in pool.most_common(60):
            _, ov = AD.mod_overlap(vo, N.locview(f2))
            sc.append((ov, f2))
        sc.sort(reverse=True)
        if sc and sc[0][0] >= max(2, 0.5 * nset) and (len(sc) == 1 or sc[0][0] - sc[1][0] >= max(1, 0.2 * sc[0][0])):
            return {'kind': 'PARTIAL', 'cands': [sc[0][1]], 'ov': [sc[0][0], nset, sc[1][0] if len(sc) > 1 else 0],
                    'strong': sc[0][0] >= 0.6 * nset and (len(sc) == 1 or sc[0][0] - sc[1][0] >= 2)}
        return {'kind': 'NONE', 'cands': [], 'top': sc[:3]}

    def loc_verdict(s, o, c):
        lo, ln = s.O.fnloc.get(o), s.N.fnloc.get(c)
        if not lo and not ln:
            return 'NOLOC', None
        if not lo or not ln:
            return 'EDITED', {'note': 'Location %d→%d' % (len(lo or ()), len(ln or ()))}
        v, extra = AD.verdict(s.O.locview(o), s.N.locview(c))
        if v == 'EDITED':
            lost, born = extra
            return v, {'lost': ['%s:%d:%d' % (m.split('\\')[-1], x[0], x[1]) for m, x in lost[:12]],
                       'born': ['%s:%d:%d' % (m.split('\\')[-1], x[0], x[1]) for m, x in born[:12]],
                       'n_lost': len(lost), 'n_born': len(born)}
        return v, None

    # ── 함수 대응 보조 증거
    @staticmethod
    def _direct_calls(X, f):
        return [int(op, 16) for a, m, op, z in X.insns(f)[0] if m == 'call' and op.startswith('0x')]

    def p_votes(s, o, limit=8):
        votes, fns = collections.Counter(), collections.defaultdict(set)
        for fc in list(s.O.calr.get(o, []))[:limit]:
            nc = s.GMb.get(fc)
            if nc is None:
                continue
            oc, ncl = s._direct_calls(s.O, fc), s._direct_calls(s.N, nc)
            if len(oc) != len(ncl):
                continue
            for i, t in enumerate(oc):
                if t == o:
                    votes[ncl[i]] += 1
                    fns[ncl[i]].add(fc)
        # 한 caller 가 다른 후보에도 표를 줬다면(서수 어긋남) 그 표는 버린다
        return votes, fns

    def _rdata_slots(s, o, limit=4):
        if s._rslot is None:
            import numpy as np
            nm, va, vsz, rr, rsz = s.O.sec('.rdata')
            n = rsz // 8
            arr = np.frombuffer(s.O.raw, dtype='<u8', count=n, offset=rr)
            m = (arr >= BASE) & (arr < BASE + 0x6000000)
            idx = np.nonzero(m)[0]
            d = collections.defaultdict(list)
            for i, q in zip(idx.tolist(), arr[idx].tolist()):
                L = d[q - BASE]
                if len(L) < 6:
                    L.append(va + 8 * i)
            s._rslot = d
        return s._rslot.get(o, [])[:limit]

    def v_votes(s, o):
        out = []
        for slot in s._rdata_slots(o):
            b, n, nmap = s.ptr_table_vote(slot)
            if b is None:
                continue
            q = s.N.read(b, 8)
            if len(q) < 8:
                continue
            t = struct.unpack('<Q', q)[0] - BASE
            if t in s.N.fn or (s.N.leaf_of(t) or (None,))[0] == t:
                out.append((t, 'rdata 슬롯 %s→%s 표 %d/%d' % (H(slot), H(b), n, nmap), 'S' if n >= 3 else 'W'))
        # 같은 후보에 여러 슬롯이면 하나로
        seen, res = set(), []
        for t, how, st in out:
            if t not in seen:
                seen.add(t)
                res.append((t, how, st))
        return res

    def _blk_ok(s, alo, ahi, blo):
        """[alo, ahi) 안의 구 함수가 전부 blo+(x-alo) 에 같은 skel 로 있는가 (블록 단위 캐시)."""
        key = (alo, ahi, blo)
        r = s._zblk.get(key)
        if r is None:
            O, N = s.O, s.N
            j0 = bisect.bisect_right(O.starts, alo)
            j1 = bisect.bisect_left(O.starts, ahi)
            ok = all((blo + (x - alo)) in N.fn and N.fn[blo + (x - alo)]['skel'] == O.fn[x]['skel']
                     for x in O.starts[j0:j1])
            r = (ok, j1 - j0)
            s._zblk[key] = r
        return r

    def z_block(s, o, maxfn=8192):
        """o 앞뒤의 전단사 앵커 a_lo<o<a_hi 가 신 exe 에서 같은 간격이면 블록이 통째로 옮겨졌는지 본다."""
        O, N = s.O, s.N
        ks = s.GMb_keys
        i = bisect.bisect_left(ks, o)
        j = i + 1 if (i < len(ks) and ks[i] == o) else i      # o 자신이 앵커면 그다음 앵커를 쓴다(독립 증거)
        if i == 0 or j >= len(ks):
            return None
        alo, ahi = ks[i - 1], ks[j]
        blo, bhi = s.GMb[alo], s.GMb[ahi]
        if bhi - blo != ahi - alo:
            return None
        ok, nf = s._blk_ok(alo, ahi, blo)
        if not ok or nf > maxfn:
            return None
        cand = blo + (o - alo)
        return (cand, 'S', '블록 이동 %s..%s→%s (함수 %d · 간격 동일 · skel 전부 일치)' % (H(alo), H(ahi), H(blo), nf))

    def leaf_z(s, leaf):
        """leaf 를 감싸는 전단사 앵커 블록(z_block 과 같은 규칙)이 같은 간격으로 옮겨졌으면 같은 상대 위치.
        앵커 블록이 안 맞으면 바로 앞뒤 fnidx 함수가 채택 대응되고 간격이 같은지 본다."""
        O, N = s.O, s.N
        cand, how = None, None
        ks = s.GMb_keys
        i = bisect.bisect_left(ks, leaf)
        if 0 < i < len(ks):
            alo, ahi = ks[i - 1], ks[i]
            blo, bhi = s.GMb[alo], s.GMb[ahi]
            if bhi - blo == ahi - alo:
                if s._blk_ok(alo, ahi, blo)[0]:
                    cand = blo + (leaf - alo)
                    how = '앵커 블록 %s..%s→%s 간격 동일' % (H(alo), H(ahi), H(blo))
        if cand is None:
            j = bisect.bisect_right(O.starts, leaf) - 1
            if j < 0 or j + 1 >= len(O.starts):
                return None
            fp, fn_ = O.starts[j], O.starts[j + 1]
            rp, rn = s.fnmap(fp), s.fnmap(fn_)
            if not (rp.get('accepted') and rn.get('accepted')) or rn['new'] - rp['new'] != fn_ - fp:
                return None
            cand = rp['new'] + (leaf - fp)
            how = '앞뒤 함수 %s·%s 채택 · 간격 동일' % (H(fp), H(fn_))
        lf = N.leaf_of(cand)
        if not lf or lf[0] != cand:
            return None
        same = O.read(leaf, O.fsize(leaf)) == N.read(cand, N.fsize(cand))
        return (cand, 'S' if same else 'W', how + (' · 바이트 동일' if same else ''), same)

    # ── 함수 대응
    def fnmap(s, o):
        if o in s._fn:
            return s._fn[o]
        O, N = s.O, s.N
        f = O.fn[o]
        ev, cons = [], {}
        res, nr, note = s.match_fn(o)
        if isinstance(nr, int):
            strong = res == 'UNIQUE' and len(O.I.by_skel.get(f['skel'], [])) == 1
            ev.append(('R', nr, 'S' if strong else 'W', res + ' ' + note))
        else:
            sk = N.I.by_skel.get(f['skel'], [])
            if 1 < len(sk) <= 4096:          # skel 다수 후보(repin 이 CG 판별 실패 시 후보 목록을 버리므로 여기서 다시)
                cons['R'] = set(sk)
        # P = 사영 caller 의 같은 서수 call 타깃(전단사 caller · 직접 call 수 동일)
        pv, pf = s.p_votes(o)
        for t, n in pv.most_common(3):
            ev.append(('P', t, 'S' if len(pf[t]) >= 2 else 'W', 'caller 콜서수 사영 %d곳(caller %d)' % (n, len(pf[t]))))
        # V = .rdata 포인터 슬롯(vtable 등) 사영
        for t, how, st in s.v_votes(o):
            ev.append(('V', t, st, how))
        # Z = 전단사 이웃 사이 블록이 통째로 같은 배치로 옮겨졌다(간격 동일 + 블록 안 전 함수 skel 일치)
        z = s.z_block(o)
        if z:
            ev.append(('Z', z[0], z[1], z[2]))
        lm = s.loc_match(o)
        if lm['kind'] in ('EXACT', 'SHIFT'):
            if len(lm['cands']) == 1:
                ev.append(('L', lm['cands'][0], 'S' if lm['uniq_old'] else 'W', 'Location ' + lm['kind']))
            else:
                cons['L'] = set(lm['cands'])
        elif lm['kind'] == 'PARTIAL':
            ev.append(('L', lm['cands'][0], 'S' if lm['strong'] else 'W', 'Location 부분 %d/%d (2위 %d)' % tuple(lm['ov'])))
        # X = 사영 caller 들의 공통 callee
        mc = {s.GM[c] for c in O.calr.get(o, []) if c in s.GM}
        if mc:
            inter = None
            for m in mc:
                st = set(N.cal.get(m, []))
                inter = st if inter is None else inter & st
            inter = {x for x in (inter or ()) if x in N.fn and
                     min(N.fn[x]['size'], f['size']) / max(N.fn[x]['size'], f['size'], 1) >= 0.5}
            if len(inter) == 1:
                ev.append(('X', next(iter(inter)), 'S' if len(mc) >= 2 else 'W', '사영 caller %d 공통 callee' % len(mc)))
            elif 1 < len(inter) <= 64:
                cons['X'] = inter
        # 제약 교집합 / 주소순 이웃 창
        cs = [v for v in cons.values() if v]
        if cs and not any(e[2] == 'S' for e in ev):
            inter = set.intersection(*cs) if len(cs) > 1 else set(cs[0])
            if len(inter) == 1:
                ev.append(('K', next(iter(inter)), 'W', '제약 교집합(%s)' % '+'.join(sorted(cons))))
            pool = inter if inter else set.union(*cs)
            i = bisect.bisect_left(s.GMb_keys, o)
            lo = s.GMb[s.GMb_keys[i - 1]] if i > 0 else None
            hi = s.GMb[s.GMb_keys[i]] if i < len(s.GMb_keys) and s.GMb_keys[i] != o else (
                s.GMb[s.GMb_keys[i + 1]] if i + 1 < len(s.GMb_keys) else None)
            if lo is not None and hi is not None and lo < hi:
                win = [c for c in pool if lo < c < hi]
                if len(win) == 1:
                    ev.append(('W', win[0], 'W', '이웃창 %s..%s' % (H(lo), H(hi))))
        r = s._decide(ev, cons)
        best = r['new']
        if best is not None and z and z[0] == best and z[1] == 'S' and not any(e[0] == 'K' for e in ev):
            # 블록 이동(Z) 은 o 자신의 skel 일치까지 확인한 것 — 앵커 배치 + 자기 꼴 = 두 갈래
            ev.append(('K', best, 'W', 'skel 일치(블록 검사)'))
            r = s._decide(ev, cons)
            best = r['new']
        if best is not None and cons and all(best in v for v in cons.values() if v) and                 not any(e[0] in ('K', 'W') for e in ev) and min(len(v) for v in cons.values() if v) <= 4096:
            # 채택 후보가 skel/Location/공통 callee 후보 집합 안에 있다 = 약한 정합 증거(K)
            ev.append(('K', best, 'W', '후보 집합(%s) 정합' % '+'.join(sorted(k for k, v in cons.items() if v))))
            r = s._decide(ev, cons)
            best = r['new']
        if best is not None:
            r['votes'] = s.votes(o, best)
            r['loc'], r['loc_detail'] = s.loc_verdict(o, best)
            r['skel_same'] = O.fn[o]['skel'] == N.fn.get(best, {}).get('skel')
            nS, nW = r['nS'], r['nW']
            v = r['votes']
            r['accepted'] = (not r['conflicts']) and v['bad'] <= 1 and (
                nS >= 2 or (nS >= 1 and (nW >= 1 or v['ok'] >= 2)))
        else:
            r['accepted'] = False
        s._fn[o] = r
        return r

    @staticmethod
    def _decide(ev, cons=None):
        by = collections.defaultdict(list)
        for e in ev:
            by[e[1]].append(e)
        if not by:
            return {'new': None, 'cands': [], 'ev': ev, 'conflicts': [], 'nS': 0, 'nW': 0}
        def sc(c):
            es = by[c]
            return (sum(1 for e in es if e[2] == 'S'), sum(1 for e in es if e[2] == 'W'))
        order = sorted(by, key=lambda c: sc(c), reverse=True)
        best = order[0]
        nS, nW = sc(best)
        conflicts = [e for e in ev if e[1] != best and e[2] == 'S']
        if cons and 'L' in cons and cons['L'] and best not in cons['L']:
            conflicts.append(('L', None, 'S', 'Location 정확일치 후보 밖'))
        if len(order) > 1 and sc(order[1]) == (nS, nW):
            conflicts.append(('TIE', order[1], 'S', '동점 후보'))
        return {'new': best, 'cands': order, 'ev': ev, 'conflicts': conflicts, 'nS': nS, 'nW': nW}

    # ── 정렬 (sitealign 방식: tok 키 → 형태 키 → 틈 사영)
    def _blocks(s, ok, nk, key):
        b = s._sm.get(key)
        if b is None:
            b = difflib.SequenceMatcher(a=ok, b=nk, autojunk=False).get_matching_blocks()
            s._sm[key] = b
        return b

    def align(s, o, c, k, win=1800):
        O, N = s.O, s.N
        oL, otok, oshp, _ = O.insns(o)
        nL, ntok, nshp, _ = N.insns(c)
        if len(oL) > 2 * win or len(nL) > 2 * win:
            # 창을 900 명령 단위로 양자화 — 같은 거대 함수 안의 행 수천 개가 정렬 블록을 공유한다(캐시 키 = 창 시작)
            q = (k // 900) * 900
            o0, o1 = max(0, q - win), min(len(oL), q + 900 + win)
            cc = int(q * len(nL) / max(1, len(oL)))
            n0, n1 = max(0, cc - int(win * 1.3)), min(len(nL), cc + 900 + int(win * 1.3))
        else:
            o0, o1, n0, n1 = 0, len(oL), 0, len(nL)
        kk = k - o0
        for tag, ok_, nk_ in (('TOK', otok, ntok), ('SHAPE', oshp, nshp)):
            bl = s._blocks(ok_[o0:o1], nk_[n0:n1], (o, c, tag, o0, n0))
            for a0, b0, ln in bl:
                if a0 <= kk < a0 + ln and ln >= 3:
                    j = n0 + b0 + (kk - a0)
                    margin = min(kk - a0, a0 + ln - 1 - kk)
                    strong = (tag == 'TOK' and ln >= 8 and (margin >= 2 or ln >= 24)) or \
                             (tag == 'SHAPE' and ln >= 10 and (margin >= 3 or ln >= 30))
                    return j, '%s blk%d m%d' % (tag, ln, margin), strong
            prev = max((b for b in bl if b[0] + b[2] <= kk and b[2] >= 3), key=lambda b: b[0], default=None)
            nxt = min((b for b in bl if b[0] > kk and b[2] >= 3), key=lambda b: b[0], default=None)
            if prev and nxt:
                og = (prev[0] + prev[2], nxt[0])
                ng = (prev[1] + prev[2], nxt[1])
                if og[1] - og[0] == ng[1] - ng[0]:
                    j = n0 + ng[0] + (kk - og[0])
                    if oL[k][1] == nL[j][1] and oL[k][3] == nL[j][3]:
                        return j, '%s_GAP' % tag, False
        return None, 'NO_ALIGN', False

    # ── n-gram 색인 (형태키 · 함수별 캐시)
    @staticmethod
    def _ngram(X, f, w):
        k = (f, w)
        d = X._ng.get(k)
        if d is None:
            shp = X.insns(f)[2]
            d = {}
            for i in range(0, max(0, len(shp) - w + 1)):
                g = tuple(shp[i:i + w])
                v = d.get(g)
                if v is None:
                    d[g] = [i]
                elif len(v) < 2:
                    v.append(i)
            X._ng[k] = d
        return d

    @staticmethod
    def _sigidx(X, f):
        k = ('sig', f)
        d = X._ng.get(k)
        if d is None:
            d = collections.defaultdict(list)
            for i, it in enumerate(X.insns(f)[0]):
                g = _sig(it)
                if g:
                    d[g].append(i)
            X._ng[k] = d
        return d

    def calls_to(s, X, f, tgt):
        """함수 f 안의 직접 call tgt 명령 index 목록(주소순)."""
        L = X.insns(f)[0]
        h = '0x%x' % tgt
        return [i for i, (a, m, o, z) in enumerate(L) if m == 'call' and o == h]

    def map_fn_quick(s, f):
        """callee 사영용 — 전역 대응맵 우선, 없으면 fnmap 채택분."""
        if f in s.GM:
            return s.GM[f]
        if f in s.O.fn:
            r = s.fnmap(f)
            if r.get('accepted'):
                return r['new']
        return None

    def sitemap(s, site, o, c):
        """owner o(구) ↔ c(신) 안에서 site 대응.
        반환 dict(new, ev, conflicts, copies, accepted, nS, nW, why)."""
        O, N = s.O, s.N
        k = O.insn_at(o, site)
        if k is None:
            return {'new': None, 'ev': [], 'conflicts': [], 'copies': [], 'accepted': False,
                    'why': 'site 가 구 owner 명령 경계 밖'}
        oL, otok, oshp, _ = O.insns(o)
        nL, ntok, nshp, naddrs = N.insns(c)
        if not nL:
            return {'new': None, 'ev': [], 'conflicts': [], 'copies': [], 'accepted': False, 'why': '신 owner 명령 0'}
        delta = site - oL[k][0]
        nset = set(naddrs)
        ev, copies = [], []
        obody = O.read(o, O.fsize(o))
        nbody = N.read(c, N.fsize(c))
        # A — 바이트 유일 (16/12/10/8B · 구 owner 안에서 유일한 가장 긴 패턴)
        for n in (16, 12, 10, 8):
            pat = O.read(site, n)
            if len(pat) < n or site + n > o + O.fsize(o) + 8:
                continue
            if _count(obody, pat) != 1:
                continue
            pos = _positions(nbody, pat, 8)
            if len(pos) == 1:
                p = c + pos[0]
                if (p - delta) in nset:
                    ev.append(('A', p, 'S', 'owner 안 %dB 유일' % n))
            elif len(pos) > 1:
                copies = [c + x for x in pos if (c + x - delta) in nset]
            break
        # U — 형태키 n-gram 유일(레지스터·즉치 무시 · site 명령을 가운데 둔 창)
        for w in (12, 8, 6, 4, 3):
            a0 = k - w // 2
            if a0 < 0 or a0 + w > len(oshp):
                continue
            g = tuple(oshp[a0:a0 + w])
            if len(s._ngram(O, o, w).get(g, ())) != 1:
                continue
            hit = s._ngram(N, c, w).get(g, ())
            if len(hit) == 1:
                j = hit[0] + w // 2
                ev.append(('U', nL[j][0] + delta, 'S' if w >= 6 else 'W', '형태 %d-gram 유일' % w))
                break
            if len(hit) > 1:
                if w >= 8 and not copies:
                    # 구에선 유일한 긴 꼴이 신 owner 에 여럿 = 사본 분기 의심(0.6.5 tactics_verify 0xff95b2 1→3 — 바이트는 달라 A 가 못 봄)
                    allh = [i for i, x in enumerate(nshp[:len(nshp) - w + 1]) if tuple(nshp[i:i + w]) == g]
                    copies = [nL[i + w // 2][0] + delta for i in allh[:8]]
                break
        # 사본 분기 — site 명령의 「즉치·변위값 보존 + 레지스터 지움」 서명이 구 owner 에선 하나, 신 owner 에선 여럿
        # (0.6.5 tactics_verify 0xff95b2 `cmp r14,0x1e` → 신 `cmp r15,0x1e` 3곳 — 바이트·n-gram 은 못 본다)
        if not copies:
            sig = _sig(oL[k])
            if sig:
                so = s._sigidx(O, o).get(sig, [])
                sn = s._sigidx(N, c).get(sig, [])
                if len(so) == 1 and len(sn) >= 2:
                    copies = [nL[i][0] + delta for i in sn[:8]]
        # B — 명령열 정렬(sitealign 방식)
        j, how, strong = s.align(o, c, k)
        if j is not None:
            ev.append(('B', nL[j][0] + delta, 'S' if strong else 'W', '정렬 ' + how))
        # C — 콜타깃 대조: site 명령(또는 반환주소면 직전 명령)이 call T 이면 T' 를 부르는 같은 서수의 call
        for kk, extra in ((k, 0), (k - 1, 1)):
            if kk < 0 or oL[kk][1] != 'call' or not oL[kk][2].startswith('0x'):
                continue
            if extra and delta != 0:
                continue
            T = int(oL[kk][2], 16)
            T2 = s.map_fn_quick(T) if T in O.fn else None
            if T2 is None:
                continue
            oc = s.calls_to(O, o, T)
            nc = s.calls_to(N, c, T2)
            if len(oc) == len(nc) and kk in oc:
                jj = nc[oc.index(kk)] + extra
                if jj < len(nL):
                    ev.append(('C', nL[jj][0] + delta, 'S', 'call %s→%s 서수 %d/%d' % (H(T), H(T2), oc.index(kk) + 1, len(oc))))
            break
        # O — 동일 오프셋 + 같은 니모닉·길이 (약)
        p = c + (oL[k][0] - o)
        if p in nset:
            jn = bisect.bisect_left(naddrs, p)
            if nL[jn][1] == oL[k][1] and nL[jn][3] == oL[k][3]:
                ev.append(('O', p + delta, 'W', '동일 오프셋'))
        r = s._decide(ev)
        r['copies'] = copies
        r['accepted'] = bool(r['new'] is not None and not r['conflicts'] and
                             (r['nS'] >= 2 or (r['nS'] >= 1 and r['nW'] >= 1)) and not
                             (len(copies) > 1))
        # 같은 꼴 사본이 신 owner 에 여럿(구는 하나) = 사본 분기 후보(tactics_verify 0.6.5 1→3)
        if len(copies) > 1:
            r['conflicts'] = r['conflicts'] + [('COPIES', None, 'S', '신 owner 안 같은 바이트 %d곳' % len(copies))]
        # 명령 안 오프셋(즉치·변위 바이트 지점)은 신 명령의 길이·니모닉이 같아야 의미가 같다
        if r['new'] is not None and delta:
            ns = r['new'] - delta
            jn = bisect.bisect_left(naddrs, ns)
            if not (jn < len(naddrs) and naddrs[jn] == ns and nL[jn][1] == oL[k][1] and nL[jn][3] == oL[k][3]):
                r['accepted'] = False
                r['conflicts'] = r['conflicts'] + [('INSN', None, 'S', '명령 안 +%d 지점인데 신 명령 꼴이 다름' % delta)]
        r['why'] = None
        return r

    def global_unique(s, site, sects, cap=64):
        """전역 바이트 유일 — 12B 부터 64B 까지 늘려 유일해지는 첫 길이. (rva, n) 또는 (None, 후보수)."""
        last = 0
        for n in (12, 16, 20, 24, 32, 48, 64):
            if n > cap:
                break
            pat = s.O.read(site, n)
            if len(pat) < n:
                break
            if len(set(pat)) <= 2:          # 0 채움·반복 바이트 = 지문 아님
                return None, 0
            hits = s.N.find_all(pat, sects=sects, limit=8)
            if len(hits) == 1:
                return (hits[0], n)
            if not hits:
                return (None, 0 if not last else last)
            last = len(hits)
        return (None, last)

    # ── xref 사영 (데이터 참조 · leaf 호출) — 참조 명령을 owner 정렬로 옮겨 신 명령의 타깃을 읽는다
    def prepare_xrefs(s, code_targets, data_rvas):
        t0 = time.time()
        rng = [(v - YSPAN, v + YSPAN) for v in data_rvas]      # 앵커(Y) 탐색용으로 앞뒤 4KB 까지
        s.XR = xref_scan(s.O, set(code_targets), rng) if (code_targets or rng) else {}
        s.XRk = sorted(s.XR)
        log('[xref] 구 exe 참조 색인 대상 %d+%d → 참조 타깃 %d (%.0fs)' % (len(code_targets), len(data_rvas), len(s.XR), time.time() - t0))

    def _ref_target_new(s, a_new):
        ins = s.N.dis1(a_new)
        if not ins or ins[0].address != a_new:
            return None
        ins = ins[0]
        for op in ins.operands:
            if op.type == X86_OP_MEM and op.mem.base == X86_REG_RIP:
                return a_new + ins.size + op.mem.disp
            if op.type == X86_OP_IMM and (ins.mnemonic in ('call', 'jmp') or ins.mnemonic.startswith('j')):
                return op.imm
        return None

    def xref_votes(s, rva, exact_only=False, limit=16):
        """rva 를 (또는 rva 앞 64B 안의 표 머리를) 참조하는 구 명령 → 신 명령 → 신 타깃 + (rva − 구 타깃) 투표."""
        votes = collections.Counter()
        fns = collections.defaultdict(set)
        nref = 0
        items = []
        for t in range(rva - (0 if exact_only else 64), rva + 1):
            for a, sz, mn in s.XR.get(t, ()):
                items.append((rva - t, a))
        items.sort()
        if any(d == 0 for d, _ in items):
            # 정확히 이 주소를 집는 참조가 있으면 그것만 — 표 머리+오프셋 참조는 상수 재배치 때 다른 상수를 따라간다
            items = [x for x in items if x[0] == 0]
        for d, a in items[:limit]:
            f = s.O.owner(a)
            if f is None:
                continue
            fr = s.fnmap(f)
            if not fr.get('accepted') or fr.get('new') is None:
                continue
            sm = s.sitemap(a, f, fr['new'])
            if not sm['accepted']:
                continue
            nt = s._ref_target_new(sm['new'])
            if nt is None:
                continue
            nref += 1
            votes[nt + d] += 1
            fns[nt + d].add(f)
        return votes, fns, nref

    def y_anchor(s, v, tries=4):
        """데이터 앵커: v 앞뒤 4KB 안에서 코드가 정확히 집는 주소 a 를 X 로 옮긴 b 가 유일하고,
        a..v 구간 내용이 구/신 같으면 v ↦ b+(v−a). (표 머리만 코드가 집는 표 안쪽 지점 · 상수 블록)"""
        O, N = s.O, s.N
        ks = s.XRk
        i = bisect.bisect_left(ks, v)
        cands = []
        for j in range(i - 1, max(-1, i - 1 - tries * 3), -1):
            if ks[j] < v - YSPAN:
                break
            cands.append(ks[j])
        for j in range(i + (1 if i < len(ks) and ks[i] == v else 0), min(len(ks), i + 1 + tries * 3)):
            if ks[j] > v + YSPAN:
                break
            cands.append(ks[j])
        cands = sorted(set(cands) - {v}, key=lambda a: abs(a - v))[:tries]
        for a in cands:
            votes, fns, nref = s.xref_votes(a, exact_only=True, limit=6)
            if len(votes) != 1:
                continue
            b = next(iter(votes))
            lo, hi = min(a, v), max(a, v) + 16
            if O.read(lo, hi - lo) == N.read(b + (lo - a), hi - lo):
                return (b + (v - a), 'S', '앵커 %s→%s(코드 참조 %d) · 구간 %dB 내용 동일' % (H(a), H(b), nref, hi - lo))
        return None

    def sec_bound(s, v):
        """섹션 경계(시작 · 가상 끝 · 16B 정렬 끝 · 원시 끝) 이면 신 exe 의 같은 경계 — 정의상 유일(.rdata 끝 상수 류)."""
        for i, (nm, va, vsz, rr, rsz) in enumerate(s.O.secs):
            nsec = s.N.sec(nm)
            if not nsec:
                continue
            _, nva, nvsz, nrr, nrsz = nsec
            for what, o_, n_ in (('시작', va, nva), ('끝', va + _vsz(s.O, nm), nva + _vsz(s.N, nm)),
                                 ('16B 정렬 끝', _al(va + _vsz(s.O, nm)), _al(nva + _vsz(s.N, nm))),
                                 ('원시 끝', va + rsz, nva + nrsz)):
                if v == o_:
                    return (n_, 'S', '섹션 %s %s' % (nm, what))
        return None

    def ptr_table_vote(s, rva, span=8, deep=False):
        """repin.cmd_rdata 와 같은 규칙: rva 주변 8B 포인터들을 전역 대응맵으로 사영해 신 .rdata 에서 같은 배열을 찾는다."""
        O, N = s.O, s.N
        sec = N.sec('.rdata')
        if not sec:
            return None, 0, 0
        nm, va, vsz, rr, rsz = sec
        blob = N.raw[rr:rr + rsz]
        votes = collections.Counter()
        nmap = 0
        for i in range(-span, span + 1):
            q = O.read(rva + 8 * i, 8)
            if len(q) < 8:
                continue
            v = struct.unpack('<Q', q)[0]
            if not (BASE <= v < BASE + 0x5000000):
                continue
            orva = v - BASE
            tgt = s.GMb.get(orva) or s.GM.get(orva)
            if tgt is None and deep and orva in s.O.fn:
                fr = s.fnmap(orva)          # vtable 항목은 skel 이 흔해 GM 밖인 경우가 많다 → 채택 대응으로 사영
                tgt = fr['new'] if fr.get('accepted') else None
            if tgt is None:
                continue
            nmap += 1
            pat = struct.pack('<Q', BASE + tgt)
            j = 0
            while True:
                j = blob.find(pat, j)
                if j < 0:
                    break
                votes[(va + j) - 8 * i] += 1
                j += 8
        if not votes:
            return None, 0, nmap
        (b, n), = votes.most_common(1)
        n2 = votes.most_common(2)[1][1] if len(votes) > 1 else 0
        return (b if n >= 2 and n - n2 >= 2 else None), n, nmap

    def caller_votes(s, leaf, limit=12):
        """leaf(.pdata 없음) 를 부르는/점프하는 구 명령을 옮겨 신 타깃 투표."""
        votes = collections.Counter()
        fns = collections.defaultdict(set)
        for a, sz, mn in s.XR.get(leaf, ())[:limit]:
            f = s.O.owner(a)
            if f is None:
                continue
            fr = s.fnmap(f)
            if not fr.get('accepted') or fr.get('new') is None:
                continue
            sm = s.sitemap(a, f, fr['new'])
            if not sm['accepted']:
                continue
            nt = s._ref_target_new(sm['new'])
            if nt is not None:
                votes[nt] += 1
                fns[nt].add(f)
        return votes, fns

    # ── 바뀐 방식 분류 (12B 가 다를 때)
    def change_of(s, old, new, nbytes=12):
        O, N = s.O, s.N
        ob, nb = O.read(old, nbytes), N.read(new, nbytes)
        if ob == nb:
            return [], None
        tags = []
        oi, ni = O.dis1(old, 4), N.dis1(new, 4)
        desc = None
        span = 0
        for idx, (a, b) in enumerate(zip(oi, ni)):
            if span >= nbytes:
                break
            span += a.size
            if a.bytes == b.bytes:
                continue
            t = _insn_change(a, b)
            if idx > 0:
                t = ['next:' + x for x in t]
            tags += t
            if desc is None:
                desc = '%s %s -> %s %s' % (a.mnemonic, a.op_str, b.mnemonic, b.op_str)
        if not tags:
            tags = ['bytes']
        return sorted(set(tags)), desc


def _insn_change(a, b):
    """두 명령(capstone detail) 의 차이 종류: insn/reg/imm/reloc/stack/disp/encoding."""
    if a.mnemonic != b.mnemonic or len(a.operands) != len(b.operands):
        return ['insn']
    out = set()
    br = a.mnemonic == 'call' or a.mnemonic.startswith('j')
    for x, y in zip(a.operands, b.operands):
        if x.type != y.type:
            out.add('insn')
            continue
        if x.type == X86_OP_REG:
            if x.reg != y.reg:
                out.add('reg')
        elif x.type == X86_OP_IMM:
            if x.imm != y.imm:
                out.add('reloc' if br else 'imm')
        elif x.type == X86_OP_MEM:
            m, n = x.mem, y.mem
            if m.base == X86_REG_RIP and n.base == X86_REG_RIP:
                if m.disp != n.disp:
                    out.add('reloc')
                continue
            if m.base != n.base or m.index != n.index or m.scale != n.scale:
                out.add('reg')
            if m.disp != n.disp:
                bn = a.reg_name(m.base) if m.base else ''
                out.add('stack' if bn in ('rbp', 'rsp', 'ebp', 'esp') else 'disp')
    if not out and a.size != b.size:
        out.add('encoding')
    return sorted(out) or ['encoding']


_RE_BIGNUM = re.compile(r'0x(?:[2-9a-f]|[0-9a-f]{2,})')


def _sig(it):
    """(addr, mnem, op_str, size) → 레지스터를 지우고 수치는 남긴 서명. 수치(2 이상·rip 상대 제외)가 없으면 None."""
    a, m, o, z = it
    if m == 'call' or m.startswith('j') or 'rip' in o or not _RE_BIGNUM.search(o):
        return None
    return m + '|' + SA._REG.sub('R', o)


def _vsz(X, nm):
    sec = X.sec(nm)
    return X.I_vsz.get(nm, sec[2]) if sec else 0


def _al(x, a=16):
    return (x + a - 1) // a * a


def _count(buf, pat):
    n, i = 0, 0
    while True:
        i = buf.find(pat, i)
        if i < 0:
            return n
        n += 1
        if n > 1:
            return n
        i += 1


def _positions(buf, pat, limit):
    out, i = [], 0
    while True:
        i = buf.find(pat, i)
        if i < 0:
            return out
        out.append(i)
        if len(out) > limit:
            return out
        i += 1


AUTO = ('SAME', 'MOVED_BYTES_CHANGED')


def _ev_json(ev):
    return [[e[0], H(e[1]), e[2], e[3]] for e in ev]


def map_row(M, row):
    """주소록 한 행 → 대응 결과 dict."""
    O, N = M.O, M.N
    v = P(row['rva'])
    kind = row['kind']
    res = {'id': row['id'], 'old': row['rva'], 'kind': kind, 'names': row.get('names', [])[:4],
           'mods': row.get('mods', []), 'src': row.get('src', [])[:3]}
    new, grade, ev, conflicts, cands, loc, notes = None, 'MISSING', [], [], [], None, []
    owner_new = None
    if kind in ('FN_START', 'FN_END') and not row.get('leaf'):
        f = v if kind == 'FN_START' else P(row['owner'])
        fr = M.fnmap(f)
        ev = fr['ev']
        conflicts = list(fr['conflicts'])
        cands = fr['cands'][:6]
        c = fr['new']
        loc = fr.get('loc')
        if c is not None:
            new = c if kind == 'FN_START' else c + N.fsize(c)
            owner_new = c
        if kind == 'FN_START' and row.get('also_fn_end_of') and c is not None:
            # 같은 값이 「앞 함수 .pdata 끝」 이기도 하다 → 두 해석이 신 exe 에서 갈리면 사람 판정(CT_REGION_HI 류)
            pf = P(row['also_fn_end_of'])
            pr = M.fnmap(pf)
            if pr['new'] is not None:
                alt = pr['new'] + N.fsize(pr['new'])
                if alt != c:
                    endish = any(END_NAME.search(x or '') for x in row.get('names') or [])
                    if endish:      # 이름이 「끝/HI/END」 = .pdata 끝 상수일 수 있다 → 두 해석이 갈리면 사람 판정
                        conflicts.append(('FN_END', alt, 'S', '앞 함수 %s 의 끝으로 보면 %s' % (H(pf), H(alt))))
                        cands = [c, alt]
                    else:
                        notes.append('앞 함수 %s 의 끝이기도 했음(신: 끝 %s ≠ 시작)' % (H(pf), H(alt)))
        if c is None and kind == 'FN_START' and row.get('also_fn_end_of'):
            pf = P(row['also_fn_end_of'])
            pr = M.fnmap(pf)
            if pr['new'] is not None:
                new = pr['new'] + N.fsize(pr['new'])
                owner_new = pr['new']
                conflicts.append(('FN_END', new, 'S', '함수 시작 대응 실패 · 앞 함수 %s 의 끝으로 보면 %s(사람 판정)' % (H(pf), H(new))))
                cands = [new]
                ev = ev + [('E', new, 'W', '앞 함수 끝 해석')]
        if new is None:
            grade = 'MISSING'
        elif c is None or not fr['accepted'] or conflicts:
            grade = 'AMBIGUOUS'
        elif loc == 'EDITED':
            grade = 'FN_EDITED'
        else:
            grade = 'SAME'
        if kind == 'FN_END' and c is not None and fr.get('votes'):
            so, sn = fr['votes']['size']
            if so != sn:
                notes.append('size %d->%d' % (so, sn))
    elif kind in ('FN_START', 'MID', 'FN_END') and row.get('leaf'):
        leaf = P(row['owner']) if kind != 'FN_START' else v
        lsz = O.fsize(leaf) or (O.leaf_of(leaf) or (leaf, leaf + 12))[1] - leaf
        votes, fns = M.caller_votes(leaf)
        g, gn = M.global_unique(leaf, ('.text',), cap=max(12, min(64, lsz)))
        for t, n in votes.most_common(4):
            ev.append(('XC', t, 'S' if len(fns[t]) >= 2 else 'W', '호출자 사영 %d표(함수 %d)' % (n, len(fns[t]))))
        if g is not None:
            ev.append(('G', g, 'S' if gn >= 20 else 'W', 'leaf 바이트 전역 유일 %dB' % gn))
        for t, how, st in M.v_votes(leaf):
            ev.append(('V', t, st, how))
        z = M.leaf_z(leaf)
        if z:
            ev.append(('Z', z[0], z[1], z[2]))
        r = Mapper._decide(ev)
        conflicts = r['conflicts']
        cands = r['cands'][:6]
        nl = r['new']
        kinds = {e[0] for e in ev if e[1] == nl}
        # 채택: 서로 다른 증거 2갈래 · 또는 호출자 2곳 이상이 각자 채택 정렬로 같은 타깃 · 또는 앵커 블록 같은 배치 + leaf 바이트 동일
        ok = nl is not None and not conflicts and (
            len(kinds) >= 2 or any(e[0] == 'XC' and e[1] == nl and e[2] == 'S' for e in ev)
            or bool(z and z[0] == nl and z[3]))
        if nl is not None:
            owner_new = nl
            new = nl + (v - leaf)
            if kind == 'FN_END':
                lf = N.leaf_of(nl)
                new = lf[1] if lf else None
        grade = 'MISSING' if new is None else ('SAME' if ok else 'AMBIGUOUS')
        if ok and kind == 'MID':
            # leaf 는 작다 — 같은 오프셋의 명령이 같은 니모닉인지 확인
            oi, ni = O.dis1(v), N.dis1(new)
            if not oi or not ni or oi[0].mnemonic != ni[0].mnemonic:
                grade = 'AMBIGUOUS'
                conflicts = conflicts + [('INSN', None, 'S', 'leaf 안 같은 오프셋 명령이 다름')]
    elif kind == 'MID':
        o = P(row['owner'])
        fr = M.fnmap(o)
        c = fr['new']
        loc = fr.get('loc')
        owner_new = c
        if c is not None:
            sm = M.sitemap(v, o, c)
            ev = sm['ev']
            conflicts = list(sm['conflicts'])
            cands = sm.get('cands', [])[:6] + [x for x in sm.get('copies', []) if x not in sm.get('cands', [])]
            new = sm['new']
            if not fr['accepted']:
                conflicts.append(('OWNER', c, 'S', 'owner 대응 미채택(%s)' % ', '.join('%s:%s' % (e[0], H(e[1])) for e in fr['ev'][:4])))
                conflicts += list(fr.get('conflicts') or [])[:3]
            if new is None:
                grade = 'MISSING'
            elif not fr['accepted'] or not sm['accepted']:
                grade = 'AMBIGUOUS'
            elif loc == 'EDITED':
                grade = 'FN_EDITED'
            else:
                grade = 'SAME'
            if sm.get('copies') and len(sm['copies']) > 1:
                notes.append('사본 %d곳' % len(sm['copies']))
        else:
            # owner 를 못 찾음 — 전역 바이트 유일만 후보로(채택 안 함)
            g, gn = M.global_unique(v, ('.text',))
            if g is not None:
                new = g
                ev = [('G', g, 'W', '전역 바이트 유일 %dB (owner 미대응)' % gn)]
                grade = 'AMBIGUOUS'
            conflicts = [('OWNER', None, 'S', 'owner %s 대응 실패' % H(o))]
            cands = fr['cands'][:6]
    elif kind in ('RDATA', 'DATA'):
        sect = '.rdata' if kind == 'RDATA' else '.data'
        g, gn = M.global_unique(v, (sect,)) if kind == 'RDATA' else (None, 0)
        if g is not None:
            ev.append(('G', g, 'S' if gn >= 20 else 'W', '%s 바이트 유일 %dB' % (sect, gn)))
        votes, fns, nref = M.xref_votes(v)
        for t, n in votes.most_common(4):
            ev.append(('X', t, 'S' if (len(fns[t]) >= 2 or n >= 2) else 'W', '코드 참조 사영 %d표(함수 %d)' % (n, len(fns[t]))))
        y = M.y_anchor(v)
        if y:
            ev.append(('Y', y[0], y[1], y[2]))
        if kind == 'RDATA':
            tb, tn, tm = M.ptr_table_vote(v, span=24)
            if tb is None:
                # 전역 대응맵만으로 표가 안 잡히면(vtable 항목은 skel 이 흔해 GM 밖) 채택 대응까지 써서 ±8 슬롯 재투표
                tb, tn, tm = M.ptr_table_vote(v, span=8, deep=True)
            if tb is not None:
                ev.append(('T', tb, 'S' if tn >= 3 else 'W', '포인터 표 %d/%d' % (tn, tm)))
        sb = M.sec_bound(v)
        if sb:
            ev.append(('SEC', sb[0], sb[1], sb[2]))
        r = Mapper._decide(ev)
        new = r['new']
        conflicts = r['conflicts']
        cands = r['cands'][:6]
        kinds = {e[0] for e in ev if e[1] == new}
        ok = new is not None and not conflicts and (
            len(kinds) >= 2 or (kinds == {'X'} and len(votes) == 1 and sum(votes.values()) >= 2)
            or 'SEC' in kinds)              # 섹션 경계는 정의상 유일
        grade = 'MISSING' if new is None else ('SAME' if ok else 'AMBIGUOUS')
    else:
        notes.append('알 수 없는 kind')
    change, cdesc = [], None
    if new is not None:
        if kind == 'FN_END':
            change = ['size'] if any(x.startswith('size') for x in notes) else []
        elif kind in ('RDATA', 'DATA'):
            # 데이터는 명령으로 해독하지 않는다 — 내용 비교만(공통 앞부분 길이)
            ob, nb = O.read(v, 12), N.read(new, 12)
            if ob != nb:
                k = next((i for i in range(min(len(ob), len(nb))) if ob[i] != nb[i]), min(len(ob), len(nb)))
                change, cdesc = ['content'], '앞 %dB 동일 · 구 %s → 신 %s' % (k, ob.hex(), nb.hex())
        else:
            change, cdesc = M.change_of(v, new)
        if grade == 'SAME' and change:
            grade = 'MOVED_BYTES_CHANGED'
    res.update(new=H(new), grade=grade, change=change, change_desc=cdesc,
               moved=(new is not None and new != v), owner_old=row.get('owner'), owner_new=H(owner_new),
               loc=loc, ev=_ev_json(ev), conflicts=_ev_json(conflicts),
               cands=[H(x) for x in cands if x is not None][:8], notes=notes)
    if row.get('mkind'):
        res['mkind'] = row['mkind']
    return res


# ═══════════════════════════════════════════════════════════════ build — 주소록 만들기

def _classify(X, v, hint_kind=None, hint_note=''):
    """주소 v 의 꼴: (kind, owner, extra dict) — X = Exe."""
    sec = X.sect(v)
    ex = {'sect': sec}
    if sec == '.rdata':
        return 'RDATA', None, ex
    if sec == '.data':
        return 'DATA', None, ex
    if sec != '.text':
        return None, None, dict(ex, why='섹션 %s' % sec)
    if hint_note.startswith('fn_end'):
        p = X.fn_end_of(v)
        if p is not None:
            return 'FN_END', p, ex
    o = X.owner(v)
    if o == v:
        pe = X.fn_end_of(v)
        if pe is not None:
            ex['also_fn_end_of'] = H(pe)
        return 'FN_START', v, ex
    if o is not None:
        k = X.insn_at(o, v)
        if k is None:
            return None, None, dict(ex, why='owner %s 명령 해독 밖' % H(o))
        a = X.insns(o)[0][k][0]
        if a != v:
            ex['insn_start'] = H(a)
            ex['insn_off'] = v - a
        return 'MID', o, ex
    p = X.fn_end_of(v)
    if p is not None:
        return 'FN_END', p, ex
    lf = X.leaf_of(v)
    if lf is None:
        return None, None, dict(ex, why='패딩/해독 불가')
    ex['leaf'] = True
    if lf[0] == v:
        return 'FN_START', v, ex
    k = X.insn_at(lf[0], v)
    if k is None:
        return None, None, dict(ex, why='leaf %s 명령 해독 밖' % H(lf[0]))
    a = X.insns(lf[0])[0][k][0]
    if a != v:
        ex['insn_start'] = H(a)
        ex['insn_off'] = v - a
    return 'MID', lf[0], ex


def _iter_manifest_rows(ver):
    for f in sorted(glob.glob(os.path.join(MAND, '*.json'))):
        mod = os.path.splitext(os.path.basename(f))[0]
        try:
            man = json.load(io.open(f, encoding='utf-8'))
        except Exception:
            continue
        for e in man.get('entries') or []:
            if e.get('ver') != ver or e.get('ignore') or e.get('kind') == 'NOT_RVA':
                continue
            try:
                v = P(e['value'])
            except (TypeError, ValueError, KeyError):
                continue
            yield dict(rva=v, names=[e.get('name', '')], mods=[mod], src=['manifest:%s:%s' % (mod, e.get('name', ''))],
                       mkind=e.get('kind'), bytes=e.get('bytes'), conf='manifest',
                       roles=e.get('roles'))


def _iter_extra_rows(path, exe_sha):
    d = json.load(io.open(path, encoding='utf-8'))
    sha = (d.get('exe_sha256') or '').upper()
    if sha and sha != exe_sha:
        raise SystemExit('--extra %s 의 exe_sha256 %s ≠ --exe %s' % (path, sha[:16], exe_sha[:16]))
    for r in d.get('rows') or []:
        yield dict(rva=P(r['rva']), names=list(r.get('names') or [])[:6], mods=[],
                   src=['kb:' + s for s in (r.get('sources') or [])[:4]], conf=r.get('confidence', 'kb'),
                   kb_kind=r.get('kind'), kb_note=r.get('note') or '', listing_only=r.get('listing_only'),
                   kb_md=('md' in (r.get('src_types') or [])))


def _iter_repinmap_rows(path, side):
    m = json.load(io.open(path, encoding='utf-8'))
    for mod, ents in m.items():
        if not isinstance(ents, dict):
            continue
        for old, e in ents.items():
            vv = old if side == 'old' else e.get('new')
            if not vv:
                continue
            yield dict(rva=P(vv), names=[e.get('name', '')], mods=[mod], src=['repinmap:%s:%s' % (mod, e.get('name', ''))],
                       conf='repinmap', mkind=e.get('kind'))


def _iter_pairs_rows(path, side):
    """probe_targets(entries[].sites[].migrated_from/rva) 또는 {구:신} / [[구,신],…]."""
    for o, n, nm in _truth_pairs(path):
        vv = o if side == 'old' else n
        if vv is None:
            continue
        for x in (vv if isinstance(vv, list) else [vv]):
            yield dict(rva=P(x), names=[nm], mods=[], src=['pairs:' + nm], conf='pairs')


def cmd_build(a):
    t0 = time.time()
    X = Exe(a.exe, a.pkl, a.cg, 'book')
    src_rows = []
    nman = 0
    if not a.no_manifests and not a.from_repin_map and not a.from_pairs:
        for r in _iter_manifest_rows(a.ver):
            src_rows.append(r)
            nman += 1
    for p in a.from_repin_map or []:
        src_rows += list(_iter_repinmap_rows(p, a.side))
    for p in a.from_pairs or []:
        src_rows += list(_iter_pairs_rows(p, a.side))
    nkb = 0
    for p in a.extra or []:
        for r in _iter_extra_rows(p, X.sha):
            src_rows.append(r)
            nkb += 1
    log('[build] 입력 행: 매니페스트 %d · extra %d · 그 밖 %d' % (nman, nkb, len(src_rows) - nman - nkb))
    by = collections.OrderedDict()
    for r in src_rows:
        b = by.get(r['rva'])
        if b is None:
            by[r['rva']] = dict(r, names=list(r['names']), mods=list(r['mods']), src=list(r['src']))
            continue
        for k in ('names', 'mods', 'src'):
            for x in r.get(k) or []:
                if x and x not in b[k]:
                    b[k].append(x)
        for k in ('mkind', 'bytes', 'roles', 'kb_kind', 'kb_note'):
            if not b.get(k) and r.get(k):
                b[k] = r[k]
        if r.get('conf') == 'manifest':
            b['conf'] = 'manifest'
    rows, rejected = [], []
    bad_bytes = 0
    for v, r in sorted(by.items()):
        kind, own, ex = _classify(X, v, r.get('kb_kind'), r.get('kb_note') or '')
        if kind is None:
            rejected.append({'rva': H(v), 'why': ex.get('why'), 'src': r['src'][:3], 'names': r['names'][:3]})
            continue
        b16 = X.read(v, 16)
        if r.get('bytes') and b16[:12].hex() != r['bytes']:
            bad_bytes += 1
            ex['bytes_mismatch_manifest'] = r['bytes']
        mk = r.get('mkind') or ''
        row = dict(id=(r['src'][0] if r['src'] else H(v)), rva=H(v), kind=kind, owner=H(own),
                   off=(v - own) if own is not None else None, bytes=b16.hex(),
                   names=[x for x in r['names'] if x][:8], mods=r['mods'], src=r['src'][:6],
                   conf=r.get('conf'))
        row.update({k: ex[k] for k in ('sect', 'insn_start', 'insn_off', 'leaf', 'also_fn_end_of', 'bytes_mismatch_manifest') if k in ex})
        if mk:
            row['mkind'] = mk
        if mk in KIND_RET:
            row['ret'] = True
        if r.get('roles'):
            row['roles'] = r['roles']
        if kind in ('FN_START', 'MID') and X.sect(v) == '.text':
            ins = X.dis1(P(row.get('insn_start') or row['rva']))
            if ins:
                row['insn'] = '%s %s' % (ins[0].mnemonic, ins[0].op_str)
        if r.get('listing_only') is not None:
            row['listing_only'] = r['listing_only']
        rows.append(row)
    cnt = collections.Counter(r['kind'] for r in rows)
    srcc = collections.Counter()
    for r in rows:
        ks = {s.split(':', 1)[0] for s in r['src']}
        srcc['+'.join(sorted(ks))] += 1
    doc = {'tool': 'MIG\\patchday\\addrbook.py', 'tool_ver': TOOL_VER, 'game_ver': a.ver, 'exe': a.exe,
           'exe_sha256': X.sha, 'generated': time.strftime('%Y-%m-%dT%H:%M:%S'),
           'args': {'pkl': a.pkl, 'cg': a.cg, 'extra': a.extra, 'no_manifests': a.no_manifests,
                    'from_repin_map': a.from_repin_map, 'from_pairs': a.from_pairs, 'side': a.side},
           'summary': {'rows': len(rows), 'rejected': len(rejected), 'kind': dict(cnt), 'source': dict(srcc),
                       'manifest_bytes_mismatch': bad_bytes,
                       'mods': dict(collections.Counter(m for r in rows for m in r['mods']))},
           'schema_note': 'kind=FN_START|MID|FN_END(.pdata 끝 상수)|RDATA|DATA · owner=fnidx 함수(leaf=True 면 .pdata 없는 leaf 시작) · '
                          'insn_start/insn_off=명령 중간 지점(즉치·변위 바이트) · also_fn_end_of=함수 시작이자 앞 함수 끝 · '
                          'ret=반환주소(매니페스트 RETURN_SITE/INLINE_RETADDR) · bytes=16B · src=manifest:/kb:/repinmap:/pairs:',
           'rows': rows, 'rejected': rejected}
    os.makedirs(os.path.dirname(os.path.abspath(a.out)), exist_ok=True)
    json.dump(doc, io.open(a.out, 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
    log('[build] 행 %d · 기각 %d · kind %s · 매니페스트 바이트 불일치 %d · %.0fs -> %s' % (
        len(rows), len(rejected), dict(cnt), bad_bytes, time.time() - t0, a.out))
    return 0


# ═══════════════════════════════════════════════════════════════ map — 구 주소록 → 신 exe

def cmd_map(a):
    t0 = time.time()
    book = json.load(io.open(a.book, encoding='utf-8'))
    O = Exe(a.old_exe, a.old_pkl, a.old_cg, 'old')
    if book.get('exe_sha256') and book['exe_sha256'] != O.sha:
        log('★주소록 exe sha %s ≠ --old-exe %s' % (book['exe_sha256'][:16], O.sha16))
        return 2
    N = Exe(a.new_exe, a.new_pkl, a.new_cg, 'new')
    M = Mapper(O, N)
    rows = book['rows']
    if a.limit:
        rows = rows[:a.limit]
    code_t = [P(r['rva']) for r in rows if r.get('leaf') and r['kind'] == 'FN_START']
    code_t += [P(r['owner']) for r in rows if r.get('leaf') and r['kind'] != 'FN_START' and r.get('owner')]
    data_v = [P(r['rva']) for r in rows if r['kind'] in ('RDATA', 'DATA')]
    M.prepare_xrefs(sorted(set(code_t)), sorted(set(data_v)))
    out = []
    errs = 0
    for i, r in enumerate(rows):
        try:
            out.append(map_row(M, r))
        except Exception as ex:          # 한 행의 실패가 전체를 막지 않게 — 오류 행은 MISSING + error
            errs += 1
            out.append({'id': r['id'], 'old': r['rva'], 'kind': r['kind'], 'grade': 'MISSING', 'new': None,
                        'error': '%s: %s' % (type(ex).__name__, ex), 'names': r.get('names', [])[:4],
                        'mods': r.get('mods', []), 'src': r.get('src', [])[:3]})
        if (i + 1) % 2000 == 0:
            log('  ... %d/%d  %.0fs' % (i + 1, len(rows), time.time() - t0))
    cnt = collections.Counter(x['grade'] for x in out)
    chg = collections.Counter(c for x in out if x['grade'] == 'MOVED_BYTES_CHANGED' for c in x.get('change') or [])
    nauto = sum(cnt[g] for g in AUTO)
    summ = {'rows': len(out), 'grade': dict(cnt), 'auto': nauto, 'review': len(out) - nauto,
            'auto_rate': round(nauto / max(1, len(out)), 4), 'change': dict(chg), 'row_errors': errs,
            'by_kind': {k: dict(collections.Counter(x['grade'] for x in out if x['kind'] == k))
                        for k in sorted({x['kind'] for x in out})},
            'moved': sum(1 for x in out if x.get('moved')),
            'old_sha16': O.sha16, 'new_sha16': N.sha16, 'new_ver': a.new_ver, 'sec': round(time.time() - t0, 1)}
    os.makedirs(a.out, exist_ok=True)
    doc = {'tool': 'MIG\\patchday\\addrbook.py map', 'book': a.book, 'old_exe': a.old_exe, 'new_exe': a.new_exe,
           'new_ver': a.new_ver, 'generated': time.strftime('%Y-%m-%dT%H:%M:%S'), 'summary': summ,
           'grade_note': __doc__.split('등급 (행 단위):')[1].split('판별 엔진')[0].strip(), 'rows': out}
    json.dump(doc, io.open(os.path.join(a.out, 'addrmap.json'), 'w', encoding='utf-8', newline='\n'),
              ensure_ascii=False, indent=1)
    rv = [x for x in out if x['grade'] not in AUTO]
    json.dump({'summary': summ, 'rows': rv}, io.open(os.path.join(a.out, 'review.json'), 'w', encoding='utf-8', newline='\n'),
              ensure_ascii=False, indent=1)
    write_md(os.path.join(a.out, 'addrmap.md'), doc)
    log('[map] %d행 · %s · 자동 %d (%.1f%%) · 행 오류 %d · %.0fs -> %s' % (
        len(out), ' '.join('%s=%d' % kv for kv in cnt.most_common()), nauto, 100.0 * nauto / max(1, len(out)), errs,
        time.time() - t0, a.out))
    if errs:
        return 2
    return 0 if nauto == len(out) else 1


def _md(s):
    return str(s if s is not None else '').replace('|', '\\|').replace('\n', ' ')


def write_md(path, doc):
    s = doc['summary']
    L = ['# 주소록 대응표 (addrbook map)', '',
         '- 구 exe `%s` → 신 exe `%s` (라벨 %s) · 생성 %s' % (s['old_sha16'], s['new_sha16'], s['new_ver'], doc['generated']),
         '- 행 %d · 등급 %s · 자동(SAME/MOVED_BYTES_CHANGED) %d = %.1f%% · 검토 %d' % (
             s['rows'], ', '.join('**%s** %d' % kv for kv in sorted(s['grade'].items(), key=lambda kv: -kv[1])),
             s['auto'], 100.0 * s['auto_rate'], s['review']),
         '- 바뀐 방식(MOVED_BYTES_CHANGED): %s' % (', '.join('%s %d' % kv for kv in sorted(s['change'].items(), key=lambda kv: -kv[1])) or '없음'),
         '- 등급 뜻: SAME=이동만(12B 동일) · MOVED_BYTES_CHANGED=신 주소 확정·바이트 다름(change: reloc/reg/stack/disp/imm/insn — **imm = 게임 기본값 변경 후보**) · '
         'FN_EDITED=owner 소스 변경(패닉 Location 집합) · AMBIGUOUS=후보 여럿/증거 부족/충돌 · MISSING=후보 없음',
         '- KB 반영은 record-keeper 경유(CLAUDE.md §9). SAME·MOVED_BYTES_CHANGED 는 그대로, 나머지는 `review.json` 판정 뒤.', '']
    order = ['MISSING', 'AMBIGUOUS', 'FN_EDITED', 'MOVED_BYTES_CHANGED', 'SAME']
    rows = doc['rows']
    for g in order:
        rs = [x for x in rows if x['grade'] == g]
        if not rs:
            continue
        L.append('## %s (%d)' % (g, len(rs)))
        L.append('')
        L.append('| 구 | 신 | kind | 바뀐 방식 | 이름 | 모드 | 근거/충돌 |')
        L.append('|---|---|---|---|---|---|---|')
        lim = 400 if g != 'SAME' else 200
        for x in rs[:lim]:
            why = '; '.join('%s %s' % (e[0], e[3]) for e in (x.get('conflicts') or [])[:2]) or \
                  ', '.join('%s:%s' % (e[0], e[2]) for e in (x.get('ev') or [])[:4])
            if x.get('error'):
                why = 'ERROR ' + x['error']
            L.append('| `%s` | `%s` | %s | %s | %s | %s | %s |' % (
                x['old'], x.get('new') or '-', x['kind'], _md(','.join(x.get('change') or [])) + (
                    ' `' + _md(x['change_desc'])[:60] + '`' if x.get('change_desc') else ''),
                _md(' · '.join(x.get('names') or [])[:70]), _md(','.join(x.get('mods') or [])), _md(why)[:160]))
        if len(rs) > lim:
            L.append('')
            L.append('… %d행 더 — `addrmap.json`' % (len(rs) - lim))
        L.append('')
    io.open(path, 'w', encoding='utf-8', newline='\n').write('\n'.join(L) + '\n')


# ═══════════════════════════════════════════════════════════════ grade — 정답지 대조

def _truth_pairs(path):
    """→ [(구, 신 또는 [신…] 또는 None, 이름)] (repin 맵 / probe_targets / {구:신} / [[구,신],…])."""
    d = json.load(io.open(path, encoding='utf-8'))
    out = []
    if isinstance(d, dict) and isinstance(d.get('entries'), list) and d['entries'] and 'sites' in d['entries'][0]:
        # tactics_verify probe_targets: 같은 migrated_from 이 여러 사이트(사본 분기)면 신 = 목록
        g = collections.OrderedDict()
        nm = {}
        for e in d['entries']:
            for st in e.get('sites') or []:
                mf = st.get('migrated_from')
                if not mf:
                    continue
                g.setdefault(mf, [])
                if st.get('rva') and st['rva'] not in g[mf]:
                    g[mf].append(st['rva'])
                nm.setdefault(mf, '%s:%s' % (e.get('id'), st.get('key') or ''))
        for mf, ns in g.items():
            out.append((mf, (ns[0] if len(ns) == 1 else ns) if ns else None, nm[mf]))
        return out
    if isinstance(d, dict) and all(isinstance(v, dict) for v in d.values()) and any(
            isinstance(x, dict) and ('new' in x or 'name' in x) for v in d.values() for x in v.values()):
        for mod, ents in d.items():
            for old, e in ents.items():
                out.append((old, e.get('new'), '%s:%s%s' % (mod, e.get('name', ''), ' [수동]' if e.get('manual') else '')))
        return out
    if isinstance(d, dict) and isinstance(d.get('sites'), dict):
        # tactics_verify overrides_<구>_<신>.json: {"sites": {구: 신 | [신…] | null}, "why": {구: 사유}}
        why = d.get('why') or {}
        return [(k, v, '%s %s' % (k, (why.get(k) or '')[:40])) for k, v in d['sites'].items()]
    if isinstance(d, dict):
        return [(k, v, k) for k, v in d.items()]
    return [(x[0], x[1], str(x[0])) for x in d]


def cmd_grade(a):
    am = json.load(io.open(a.addrmap, encoding='utf-8'))
    rows = {P(x['old']): x for x in am['rows']}
    tp = _truth_pairs(a.truth)
    res = collections.Counter()
    wrong, rv_bad, rv_ok, miss = [], [], [], []
    seen = set()
    for o, n, nm in tp:
        ov = P(o)
        if ov in seen:
            continue
        seen.add(ov)
        x = rows.get(ov)
        tset = {P(t) for t in (n if isinstance(n, list) else [n])} if n else set()
        if x is None:
            res['not_in_book'] += 1
            miss.append({'old': H(ov), 'truth': [H(t) for t in tset], 'name': nm})
            continue
        got = P(x['new']) if x.get('new') else None
        auto = x['grade'] in AUTO
        rec = {'old': H(ov), 'truth': sorted(H(t) for t in tset), 'got': x.get('new'), 'grade': x['grade'],
               'kind': x['kind'], 'name': nm, 'change': x.get('change'),
               'conflicts': [c[3] for c in (x.get('conflicts') or [])][:3],
               'ev': ['%s:%s:%s' % (e[0], e[1], e[2]) for e in (x.get('ev') or [])][:5]}
        if not tset:
            res['truth_none'] += 1
            if auto:
                res['auto_wrong'] += 1
                wrong.append(dict(rec, why='정답 = 대응 없음(비활성)인데 자동 확정'))
            else:
                res['review_ok'] += 1
            continue
        if auto:
            if len(tset) == 1 and got in tset:
                res['auto_correct'] += 1
            else:
                res['auto_wrong'] += 1
                wrong.append(dict(rec, why='사본 분기 정답 %d곳을 단일 확정' % len(tset) if len(tset) > 1 else '자동 확정값 ≠ 정답'))
        else:
            res['review'] += 1
            if got in tset:
                res['review_cand_correct'] += 1
                rv_ok.append(rec)
            else:
                rv_bad.append(rec)
    n = len(seen)
    summ = {'truth': a.truth, 'addrmap': a.addrmap, 'n': n, 'counts': dict(res),
            'auto_wrong': res['auto_wrong'], 'auto_rate': round(res['auto_correct'] / max(1, n), 4),
            'review_rows': res['review'] + res['review_ok'],
            'review_cand_correct': res['review_cand_correct']}
    log('[grade] 정답 %d · 자동정답 %d · ★자동오답 %d · 검토 %d(후보 일치 %d) · 정답없음 %d · 주소록 밖 %d · 자동 대응률 %.1f%%' % (
        n, res['auto_correct'], res['auto_wrong'], res['review'], res['review_cand_correct'], res['truth_none'],
        res['not_in_book'], 100.0 * summ['auto_rate']))
    for w in wrong:
        log('  ★자동오답 %s → got %s / 정답 %s  [%s] %s · %s' % (w['old'], w['got'], w['truth'], w['grade'], w['name'], w['why']))
    out = {'summary': summ, 'auto_wrong': wrong, 'review_cand_wrong': rv_bad, 'review_cand_right': rv_ok, 'not_in_book': miss}
    if a.out:
        json.dump(out, io.open(a.out, 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
        log('-> %s' % a.out)
    return 0 if not wrong else 1


# ═══════════════════════════════════════════════════════════════ main

def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0], formatter_class=argparse.RawDescriptionHelpFormatter,
                                 epilog='상세 = 모듈 docstring')
    sp = ap.add_subparsers(dest='cmd', required=True)
    b = sp.add_parser('build', help='주소록 만들기')
    b.add_argument('--ver', required=True)
    b.add_argument('--exe', required=True)
    b.add_argument('--pkl', required=True)
    b.add_argument('--cg', required=True)
    b.add_argument('--extra', nargs='*', default=[], help='kb_harvest 산출 rows JSON (exe_sha256 일치 필수)')
    b.add_argument('--out', required=True)
    b.add_argument('--no-manifests', action='store_true')
    b.add_argument('--from-repin-map', nargs='*', default=[], help='리허설용 — repin 맵의 구/신 값으로 주소록')
    b.add_argument('--from-pairs', nargs='*', default=[], help='리허설용 — probe_targets/{구:신} 쌍의 구/신 값으로 주소록')
    b.add_argument('--side', choices=['old', 'new'], default='old')
    m = sp.add_parser('map', help='구 주소록 → 신 exe 대응')
    m.add_argument('--book', required=True)
    m.add_argument('--old-exe', required=True)
    m.add_argument('--new-exe', required=True)
    m.add_argument('--new-ver', required=True)
    m.add_argument('--old-pkl', required=True)
    m.add_argument('--new-pkl', required=True)
    m.add_argument('--old-cg', required=True)
    m.add_argument('--new-cg', required=True)
    m.add_argument('--out', required=True)
    m.add_argument('--limit', type=int, default=0, help='디버그: 앞 N행만')
    g = sp.add_parser('grade', help='정답지 대조')
    g.add_argument('--addrmap', required=True)
    g.add_argument('--truth', required=True)
    g.add_argument('--truth-format', choices=['repinmap', 'pairs'], default=None, help='(자동 판별 — 호환용)')
    g.add_argument('--out', default=None)
    a = ap.parse_args(argv)
    try:
        if a.cmd == 'build':
            return cmd_build(a)
        if a.cmd == 'map':
            return cmd_map(a)
        return cmd_grade(a)
    except SystemExit:
        raise
    except Exception:
        import traceback
        traceback.print_exc()
        return 2


if __name__ == '__main__':
    sys.exit(main())
