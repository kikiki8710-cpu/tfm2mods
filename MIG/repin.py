#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""repin.py — 매니페스트 STALE 엔트리를 새 exe 에서 재핀(버전 무관 범용 엔진).

  python MIG/repin.py plan  [MOD ...] --old <구exe> --new <신exe>
                            --oldpkl <구.pkl> --newpkl <신.pkl> --out <map.json>
  python MIG/repin.py apply [MOD ...] --map <map.json> [--write]

재핀 우선순위 (MIG/README.md ③):
  1) 매니페스트 채록 바이트(12B, 부족하면 구exe 에서 연장)를 신 exe 전 섹션에서 유일검색
  2) 구 exe 함수시작이면 skeleton 지문 매칭(match_fn)
  3) 함수 중간 사이트면 owner 함수 매칭 후 오프셋 승계(match_mid)
  실패 = ghidra-re 로 넘길 목록으로 보고
"""
import sys, os, re, json, struct, bisect, argparse

sys.stdout.reconfigure(encoding='utf-8', errors='replace')
ROOT = r'C:\tfm2mods'
sys.path.insert(0, os.path.join(ROOT, 'MIG'))
import mig_verify as MV  # noqa: E402

IMAGE_BASE = 0x140000000


class Img:
    """PE + fnindex.pkl (skel/head 지문)"""

    def __init__(s, exe, pkl):
        d = open(exe, 'rb').read()
        s.raw = d
        pe = struct.unpack_from('<I', d, 0x3c)[0]
        nsec = struct.unpack_from('<H', d, pe + 6)[0]
        opt = pe + 24
        sectab = opt + struct.unpack_from('<H', d, pe + 20)[0]
        s.secs = []
        for i in range(nsec):
            o = sectab + i * 40
            nm = d[o:o + 8].rstrip(b'\0').decode(errors='replace')
            vsz, va, rsz, rraw = struct.unpack_from('<IIII', d, o + 8)
            s.secs.append((nm, va, max(vsz, rsz), rraw, rsz))
        P = __import__('pickle').load(open(pkl, 'rb'))
        s.fn = {(int(k, 16) if isinstance(k, str) else k): v
                for k, v in P['idx'].items()}
        s.by_skel = P['by_skel']
        s.by_head = P.get('by_head', {})
        s.starts = sorted(s.fn)

    def roff(s, rva):
        for nm, va, vsz, rraw, rsz in s.secs:
            if va <= rva < va + vsz:
                o = rva - va
                return rraw + o if o < rsz else None
        return None

    def read(s, rva, n):
        o = s.roff(rva)
        return None if o is None else s.raw[o:o + n]

    def owner(s, rva):
        i = bisect.bisect_right(s.starts, rva) - 1
        if i < 0:
            return None
        st = s.starts[i]
        return st if rva < st + s.fn[st]['size'] else None

    def sect_of(s, rva):
        for nm, va, vsz, rraw, rsz in s.secs:
            if va <= rva < va + vsz:
                return nm
        return '?'

    def find_all(s, pat, limit=8, sects=None):
        """전 섹션 raw 검색 -> [rva,...] (sects 지정 시 그 섹션만)"""
        hits = []
        for nm, va, vsz, rraw, rsz in s.secs:
            if sects and nm not in sects:
                continue
            blob = s.raw[rraw:rraw + rsz]
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


def _cos(a, b):
    ks = set(a) | set(b)
    num = sum(a.get(k, 0) * b.get(k, 0) for k in ks)
    da = sum(v * v for v in a.values()) ** .5
    db = sum(v * v for v in b.values()) ** .5
    return num / (da * db) if da and db else 0.0


def build_match(O, N, cgO=None, cgN=None):
    """cgO/cgN = callgraph.py 산출 {'callee':…,'caller':…}. 주면 MULTI/NONE 을 호출관계로 판별."""

    def skel_match(rva):
        f = O.fn.get(rva)
        if not f:
            return ('NOT_FN_START', None, '구 exe 함수시작 아님')
        cands = N.by_skel.get(f['skel'], [])
        if len(cands) == 1:
            return ('UNIQUE', cands[0], 'size %d->%d' % (f['size'], N.fn[cands[0]]['size']))
        if len(cands) == 0:
            hc = N.by_head.get(f['head'], [])
            if len(hc) == 1:
                return ('HEAD_UNIQUE', hc[0], 'skel0/head유일 size %d->%d' % (f['size'], N.fn[hc[0]]['size']))
            return ('NONE', None, 'skel 후보0 / head후보 %d' % len(hc))
        same = [c for c in cands if N.fn[c]['size'] == f['size']]
        if len(same) == 1:
            return ('UNIQUE_bySize', same[0], 'skel %d후보 size로 유일' % len(cands))
        return ('MULTI', cands, '%d후보' % len(cands))

    # ── 전역 대응맵(UNIQUE 계열만) — callee/caller 사영의 기준
    GM = {}
    if cgO and cgN:
        for rva in O.fn:
            r, nr, _ = skel_match(rva)
            if r in ('UNIQUE', 'UNIQUE_bySize', 'HEAD_UNIQUE') and isinstance(nr, int):
                GM[rva] = nr
        print('[cg] 전역 대응맵 %d/%d (%.1f%%)' % (len(GM), len(O.fn), 100.0 * len(GM) / len(O.fn)))
    GMV = set(GM.values())

    def _score(old, cand):
        """callee/caller 집합의 사영 일치 수"""
        oc = [GM[c] for c in cgO['callee'].get(old, []) if c in GM]
        nc = set(cgN['callee'].get(cand, []))
        s = len(set(oc) & nc)
        op = [GM[c] for c in cgO['caller'].get(old, []) if c in GM]
        npr = set(cgN['caller'].get(cand, []))
        s += len(set(op) & npr)
        return s, len(oc) + len(op)

    def _pick(old, cands, tag):
        """후보 축소 3단: ①사영된 caller 의 callee 집합으로 제한 ②cg 중첩 점수
        ③size 일치 + mnem 코사인. 1·2위 차가 확실할 때만 채택한다."""
        f = O.fn[old]
        cands = list(cands)
        # ① 사영 caller 의 callee 로 제한 (강한 제약)
        mc = {GM[c] for c in cgO['caller'].get(old, []) if c in GM}
        allowed = set()
        for m in mc:
            allowed |= set(cgN['callee'].get(m, []))
        restricted = [c for c in cands if c in allowed]
        via = ''
        if len(restricted) == 1:
            return (tag + '_caller', restricted[0],
                    '사영 caller %d개의 callee 로 후보 %d->1' % (len(mc), len(cands)))
        if restricted:
            cands, via = restricted, ' (caller제한 %d->%d)' % (len(cands), len(restricted))
        # ②③ 복합 점수
        def total(c):
            s, _ = _score(old, c)
            sz = 2 if N.fn[c]['size'] == f['size'] else 0
            return 3 * s + sz + _cos(f['mnem'], N.fn[c]['mnem'])
        rank = sorted(cands, key=lambda c: -total(c))
        if not rank:
            return (None, None, 'cg 후보 0' + via)
        t1 = total(rank[0])
        t2 = total(rank[1]) if len(rank) > 1 else -9
        if t1 >= 1.0 and t1 - t2 >= 0.5:
            return (tag, rank[0], 'cg복합 %.2f>%.2f (후보 %d)%s' % (t1, t2, len(rank), via))
        return (None, None, 'cg 판별실패 (1위 %.2f 2위 %.2f, 후보 %d)%s' % (t1, t2, len(rank), via))

    def match_fn(rva):
        res, nr, note = skel_match(rva)
        if res in ('MULTI', 'NONE') and cgO and cgN and rva in O.fn:
            if res == 'MULTI':
                cands = nr
            else:
                # skel 0후보 = 본문 변경. 사영된 caller 들이 새로 부르는 함수들을 후보로
                cands = set()
                for c in cgO['caller'].get(rva, []):
                    if c in GM:
                        cands |= set(cgN['callee'].get(GM[c], []))
                cands = [c for c in cands if c not in GMV] or list(cands)
                if not cands:
                    return (res, None, note)
            t, pick, why = _pick(rva, cands, 'CG_' + res)
            if pick is not None:
                return (t, pick, note + ' | ' + why)
            return (res, None, note + ' | ' + why)
        return (res, nr, note)

    def locate_in_fn(site, nown):
        """새 owner 함수 안에서 옛 사이트의 로컬 바이트로 정확 위치를 찾는다."""
        own = O.owner(site)
        f = N.fn.get(nown)
        if own is None or not f:
            return None
        body = N.read(nown, f['size']) or b''
        for n in (16, 12, 10, 8):
            pat = O.read(site, n)
            if not pat or len(pat) < n:
                continue
            hits, i = [], 0
            while True:
                i = body.find(pat, i)
                if i < 0:
                    break
                hits.append(nown + i)
                i += 1
                if len(hits) > 1:
                    break
            if len(hits) == 1:
                return hits[0]
        return None

    def match_mid(site):
        own = O.owner(site)
        if own is None:
            return ('NO_OWNER', None, '구 exe owner 없음')
        off = site - own
        res, nown, note = match_fn(own)
        if isinstance(nown, int):
            exact = locate_in_fn(site, nown)
            if exact is not None:
                return ('OWNER_' + res, exact,
                        'owner %s->%s 본문검색 off %s->%s'
                        % (hex(own), hex(nown), hex(off), hex(exact - nown)))
            return ('OWNER_' + res, nown + off,
                    'owner %s->%s off %s(동일오프셋 가정)' % (hex(own), hex(nown), hex(off)))
        return ('OWNER_' + res, None, 'owner=%s off=%s %s' % (hex(own), hex(off), note))

    return match_fn, match_mid


def repin_entry(e, O, N, match_fn, match_mid):
    """-> (new_rva|None, kind, note)"""
    v = int(e['value'], 16)
    ob = e.get('bytes')
    # ── 1) 바이트 유일검색 (부족하면 구 exe 에서 패턴 연장)
    if ob:
        pat = bytes.fromhex(ob)
        for n in (12, 20, 32, 48, 64):
            blob = O.read(v, n)
            if blob is None or len(blob) < n:
                break
            if not blob.startswith(pat):
                break              # 매니페스트가 이 exe 채록이 아님 -> 연장 불가
            hits = N.find_all(blob)
            if len(hits) == 1:
                return (hits[0], 'BYTES_UNIQUE(%dB)' % n, '')
            if len(hits) == 0:
                break              # 더 늘려도 0
        else:
            pass
        hits = N.find_all(pat)
        if len(hits) == 1:
            return (hits[0], 'BYTES_UNIQUE(12B)', '')
        if len(hits) > 1:
            note12 = '12B %d후보' % len(hits)
        else:
            note12 = '12B 0후보'
    else:
        note12 = 'bytes 없음'
    # ── 2) 함수시작 skeleton
    res, nr, note = match_fn(v)
    if res.startswith(('UNIQUE', 'HEAD_UNIQUE')) and isinstance(nr, int):
        return (nr, 'FN_' + res, note12 + ' / ' + note)
    # ── 3) 함수 중간 사이트
    res2, nr2, note2 = match_mid(v)
    if isinstance(nr2, int):
        return (nr2, 'MID_' + res2, note12 + ' / ' + note2)
    return (None, 'FAIL', '%s / fn:%s %s / mid:%s %s' % (note12, res, note, res2, note2))


def cmd_plan(mods, a):
    O = Img(a.old, a.oldpkl)
    N = Img(a.new, a.newpkl)
    import pickle
    cgO = pickle.load(open(a.oldcg, 'rb')) if a.oldcg else None
    cgN = pickle.load(open(a.newcg, 'rb')) if a.newcg else None
    match_fn, match_mid = build_match(O, N, cgO, cgN)
    at_new, _ = MV.read_exe(a.new)
    out = {}
    for mod in mods:
        man = MV.load_man(mod)
        if not man:
            print('%-26s !매니페스트 없음' % mod)
            continue
        rows, stats = {}, {}
        for e in man['entries']:
            if e.get('ignore'):
                continue
            v = int(e['value'], 16)
            cur = at_new(v)
            if e.get('bytes') and cur and cur.hex() == e['bytes']:
                continue                       # PASS = 안 움직임
            nr, kind, note = repin_entry(e, O, N, match_fn, match_mid)
            stats[kind.split('(')[0]] = stats.get(kind.split('(')[0], 0) + 1
            rec = {'name': e['name'], 'kind': kind, 'note': note,
                   'locs': e.get('locs', []), 'sect_old': e.get('sect')}
            if nr is not None:
                rec['new'] = hex(nr)
                rec['sect_new'] = N.sect_of(nr)
                nb = N.read(nr, 12)
                rec['new_bytes'] = nb.hex() if nb else None
                rec['bytes_same'] = (nb.hex() == e['bytes']) if (nb and e.get('bytes')) else False
                rec['fn_start_new'] = nr in N.fn
                rec['fn_start_old'] = v in O.fn
            rows[e['value']] = rec
        out[mod] = rows
        nfail = sum(1 for r in rows.values() if 'new' not in r)
        print('%-26s 재핀대상 %4d  해결 %4d  실패 %3d   %s'
              % (mod, len(rows), len(rows) - nfail, nfail,
                 ' '.join('%s=%d' % kv for kv in sorted(stats.items()))))
    json.dump(out, open(a.out, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
    print('-> %s' % a.out)


mask_code = MV.mask_code   # 정본 = mig_verify (extract/coverage 와 같은 규칙을 쓰기 위해)


HEXP = re.compile(r'0x([0-9a-fA-F]{5,8})\b')


def cmd_apply(mods, a):
    mp = json.load(open(a.map, encoding='utf-8'))
    for mod in mods:
        rows = mp.get(mod)
        if not rows:
            continue
        # 값 -> 새 값 (해결된 것만). 같은 값이 다른 새 값으로 갈리면 사고 -> 거부
        sub = {}
        for old, r in rows.items():
            if 'new' in r:
                sub[int(old, 16)] = int(r['new'], 16)
        man = MV.load_man(mod)
        exclude = man.get('exclude', []) if man else []
        nfile = nrep = 0
        for path in MV.sources(mod):
            rel = os.path.relpath(path, os.path.join(ROOT, mod)).replace('\\', '/')
            if any(rel.startswith(x) for x in exclude):
                continue
            raw = open(path, 'rb').read().decode('utf-8', 'replace')
            stripped = mask_code(raw)   # ★길이보존 마스킹(주석+문자열)
            # 주석 밖 위치만 치환 대상 (offset 동일하므로 stripped 로 판정)
            edits = []
            for m in HEXP.finditer(stripped):
                v = int(m.group(1), 16)
                if v in sub:
                    edits.append((m.start(), m.end(), '0x%x' % sub[v]))
            if not edits:
                continue
            buf = raw
            for s0, e0, new in reversed(edits):
                buf = buf[:s0] + new + buf[e0:]
            nfile += 1
            nrep += len(edits)
            if a.write:
                open(path, 'w', encoding='utf-8', newline='').write(buf)
        print('%-26s %s 파일 %d개 / 치환 %d곳'
              % (mod, '적용' if a.write else '드라이런', nfile, nrep))
        if a.write and man:
            for e in man['entries']:
                r = rows.get(e['value'])
                if r and 'new' in r:
                    e['value'] = r['new']
                    e['ver'] = a.ver
            MV.save_man(mod, man)
            print('%-26s 매니페스트 value 갱신 (ver=%s)' % (mod, a.ver))




# ─────────────────────────────────────────────────────────────
def cmd_rdata(a):
    """.rdata 포인터 표(vtable 등) 재핀 — 표 안의 함수 포인터를 전역 대응맵으로 사영해
    새 exe .rdata 에서 같은 배열을 투표로 찾는다. (바이트 대조·skel 매칭이 안 닿는 축)"""
    import pickle, struct, collections
    O = Img(a.old, a.oldpkl); N = Img(a.new, a.newpkl)
    cgO = pickle.load(open(a.oldcg,'rb')) if a.oldcg else None
    cgN = pickle.load(open(a.newcg,'rb')) if a.newcg else None
    build_match(O, N, cgO, cgN)          # GM 구축 로그용
    # GM 재구축(반환 안 하므로 여기서 다시)
    GM = {}
    for rva in O.fn:
        f = O.fn[rva]; c = N.by_skel.get(f['skel'], [])
        if len(c) == 1: GM[rva] = c[0]; continue
        if len(c) == 0:
            h = N.by_head.get(f['head'], [])
            if len(h) == 1: GM[rva] = h[0]
            continue
        same = [x for x in c if N.fn[x]['size'] == f['size']]
        if len(same) == 1: GM[rva] = same[0]
    print('[rdata] GM %d' % len(GM))
    IB = IMAGE_BASE
    nr_off = None
    for nm, va, vsz, rraw, rsz in N.secs:
        if nm == '.rdata': nr_off = (va, rraw, rsz)
    nva, nraw, nsz = nr_off
    nblob = N.raw[nraw:nraw+nsz]
    for spec in a.addrs:
        addr = int(spec, 16)
        span = a.span
        votes = collections.Counter(); nmap = 0
        for i in range(-span, span+1):
            q = O.read(addr + 8*i, 8)
            if not q or len(q) < 8: continue
            v = struct.unpack('<Q', q)[0]
            if not (IB <= v < IB + 0x5000000): continue
            orva = v - IB
            if orva not in GM: continue
            nmap += 1
            pat = struct.pack('<Q', IB + GM[orva])
            j = 0
            while True:
                j = nblob.find(pat, j)
                if j < 0: break
                votes[(nva + j) - 8*i] += 1
                j += 8
        if not votes:
            print('%s  판별불가 (사영가능 포인터 %d)' % (spec, nmap)); continue
        (base, n), = votes.most_common(1)
        sec = votes.most_common(2)[1][1] if len(votes) > 1 else 0
        print('%s -> %s   표 %d/%d (2위 %d)  delta=%+#x'
              % (spec, hex(base), n, nmap, sec, base - addr))



def cmd_resolve(a):
    """구 RVA 목록을 직접 재핀(소스·매니페스트 무관) — 잔여 실패 개별 처리용."""
    import pickle
    O = Img(a.old, a.oldpkl); N = Img(a.new, a.newpkl)
    cgO = pickle.load(open(a.oldcg, 'rb')) if a.oldcg else None
    cgN = pickle.load(open(a.newcg, 'rb')) if a.newcg else None
    match_fn, match_mid = build_match(O, N, cgO, cgN)
    for spec in a.addrs:
        v = int(spec, 16)
        e = {'value': hex(v), 'bytes': (O.read(v, 12) or b'').hex() or None}
        nr, kind, note = repin_entry(e, O, N, match_fn, match_mid)
        if nr is None:
            print('%-11s  실패   %s' % (spec, note))
        else:
            ob = O.read(v, 12); nb = N.read(nr, 12)
            print('%-11s -> %-11s %-26s bytes%s  fn?%s->%s  %s'
                  % (spec, hex(nr), kind,
                     '동일' if (ob and nb and ob == nb) else '다름',
                     v in O.fn, nr in N.fn, note[:110]))


if __name__ == '__main__':
    ap = argparse.ArgumentParser()
    ap.add_argument('cmd', choices=['plan', 'apply', 'rdata', 'resolve'])
    ap.add_argument('--addrs', nargs='*', default=[])
    ap.add_argument('--span', type=int, default=24)
    ap.add_argument('mods', nargs='*')
    ap.add_argument('--old'), ap.add_argument('--new')
    ap.add_argument('--oldpkl'), ap.add_argument('--newpkl')
    ap.add_argument('--oldcg'), ap.add_argument('--newcg')
    ap.add_argument('--out', default=os.path.join(ROOT, 'MIG', 'repin_map.json'))
    ap.add_argument('--map', default=os.path.join(ROOT, 'MIG', 'repin_map.json'))
    ap.add_argument('--ver', default='0.5.8')
    ap.add_argument('--write', action='store_true')
    a = ap.parse_args()
    mods = a.mods or list(MV.MODS)
    if a.cmd == 'plan':
        cmd_plan(mods, a)
    elif a.cmd == 'rdata':
        cmd_rdata(a)
    elif a.cmd == 'resolve':
        cmd_resolve(a)
    else:
        cmd_apply(mods, a)
