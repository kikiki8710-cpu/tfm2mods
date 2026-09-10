# -*- coding: utf-8 -*-
"""rmeta 의 SourceMap(SourceFile 테이블)을 디코드한다.

rustc 1.98.0-nightly(23a3312d9 2026-05-23) 의 rmeta 포맷을 바이트에서 역산한 것
(공식 명세 없음 — 2026-09-11 세션에서 확정, 자기검증 = diff 합 <= source_len +
다음 배열이 단조증가 (pos,bytes) 쌍으로 파싱될 것 + 다음 엔트리 시작에 정확히 착지).

엔트리 레이아웃:
  <len>  rel_path                 (예: game-ai\\src\\lib.rs)
  0xc1 <len> remap_base           (D:\\a\\TeamfightManager2\\TeamfightManager2)
  0xc1 <len> local_abs_path
  0xc1 0x00                       (종료)
  <u8 alg> <len> <hash bytes>     (alg=2 => 32B)
  <u8 checksum_opt = 0>
  <leb source_len>                normalize 후 바이트 길이(CRLF->LF)
  <leb raw_len>                   원본(정규화 전) 바이트 길이
  <leb nlines>
  <nlines * width> 줄 시작 델타     width 는 1/2/4 (자동 판정)
  <leb n> (pos, nbytes)*n         multibyte_chars
  <leb n> (pos, diff)*n           normalized_pos (CRLF 위치)
  ... stable_id, cnum

사용:
  python rmeta_srcmap.py game_ai                     # 파일 목록
  python rmeta_srcmap.py game_ai <경로부분> [a] [b]  # a~b 줄의 바이트길이/문자길이
"""
import io
import os
import re
import sys
import bisect

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

SDK = r'C:\tfm2mods\sdk_058\mod-sdk\deps'
BS = bytes([92])


def rmeta_path(c):
    for x in os.listdir(SDK):
        if x.startswith('lib' + c + '-') and x.endswith('.rmeta'):
            return os.path.join(SDK, x)
    raise SystemExit('no rmeta for ' + c)


class R(object):
    def __init__(self, d, p=0):
        self.d = d
        self.p = p

    def u8(self):
        b = self.d[self.p]
        self.p += 1
        return b

    def leb(self):
        r = 0
        s = 0
        while True:
            b = self.d[self.p]
            self.p += 1
            r |= (b & 0x7f) << s
            if not b & 0x80:
                return r
            s += 7

    def st(self):
        n = self.leb()
        s = self.d[self.p:self.p + n]
        self.p += n
        return s.decode('utf-8', 'replace')


def find_entries(d):
    sep = re.escape(BS)
    pat = (b'(game-ai|game-core|game-view|common|engine-[a-z-]+|mod-api)' + sep
           + b'src' + sep + b'[0-9A-Za-z_' + sep + b']+[.]rs')
    out = []
    for m in re.finditer(pat, d):
        s, e = m.start(), m.end()
        ln = e - s
        if ln < 128 and d[s - 1] == ln:
            out.append((s - 1, e, d[s:e].decode('ascii')))
    return out


def parse_entry(d, start, limit):
    r = R(d, start)
    paths = [r.st()]
    while r.d[r.p] == 0xc1:
        r.p += 1
        n = r.leb()
        if n == 0:
            break
        s = r.d[r.p:r.p + n]
        r.p += n
        paths.append(s.decode('utf-8', 'replace'))
    alg = r.u8()
    hl = r.leb()
    r.p += hl
    ck = r.u8()
    if ck:
        # checksum_hash = Some(..) -> 길이는 알고리즘에 따름; 관측 범위(0.5.8)에선 항상 0
        raise ValueError('checksum present tag=%d' % ck)
    src_len = r.leb()
    raw_len = r.leb()
    nlines = r.leb()
    bpd = r.u8()                     # bytes_per_diff (1/2/4)
    diffs_at = r.p
    ncrlf = raw_len - src_len
    ndiff = nlines - 1
    after = diffs_at + ndiff * bpd
    rr = R(d, after)
    nmb = rr.leb()
    mb = [(rr.leb(), rr.leb()) for _ in range(nmb)]
    rr.p += 16                       # StableSourceFileId (u128, 고정 16B)
    nnp = rr.leb()
    if nnp > src_len:
        raise ValueError('normalized_pos %d 비정상 (bpd=%d nlines=%d)' % (nnp, bpd, nlines))
    npos = [(rr.leb(), rr.leb()) for _ in range(nnp)]
    if bpd == 1:
        diffs = list(d[diffs_at:after])
    else:
        diffs = [int.from_bytes(d[diffs_at + i * bpd:diffs_at + (i + 1) * bpd], 'little')
                 for i in range(ndiff)]
    return dict(rel=paths[0], paths=paths, alg=alg, src_len=src_len, raw_len=raw_len,
                nlines=nlines, width=bpd, diffs=diffs, diffsum=sum(diffs), mb=mb,
                nnp=len(npos), npos=npos, end=rr.p, entry=start, limit=limit)


def load(crate):
    d = open(rmeta_path(crate), 'rb').read()
    ents = find_entries(d)
    out = []
    for i, (s, e, rel) in enumerate(ents):
        lim = ents[i + 1][0] if i + 1 < len(ents) else len(d)
        try:
            out.append(parse_entry(d, s, lim))
        except Exception as ex:
            out.append(dict(rel=rel, entry=s, limit=lim, error=str(ex)))
    return d, out


def line_table(e):
    """1-based 줄번호 -> (byte_start, byte_len, char_len, mb_count)"""
    starts = [0]
    for x in e['diffs']:
        starts.append(starts[-1] + x)
    mbd = dict(e['mb'])
    mbs = sorted(mbd)
    rows = []
    for i in range(len(starts)):
        s0 = starts[i]
        s1 = starts[i + 1] if i + 1 < len(starts) else e['src_len']
        i0 = bisect.bisect_left(mbs, s0)
        i1 = bisect.bisect_left(mbs, s1)
        extra = sum(mbd[mbs[k]] - 1 for k in range(i0, i1))
        rows.append((s0, s1 - s0, (s1 - s0) - extra, i1 - i0))
    return rows


def main():
    crate = sys.argv[1]
    d, ents = load(crate)
    if len(sys.argv) < 3:
        for e in ents:
            if 'error' in e:
                print('ERR %-62s @%08x %s' % (e['rel'], e['entry'], e['error']))
            else:
                print('%-62s @%08x len=%-7d raw=%-7d lines=%-5d w=%d sum=%-7d mb=%-5d np=%-5d slack=%d'
                      % (e['rel'], e['entry'], e['src_len'], e['raw_len'], e['nlines'],
                         e['width'], e['diffsum'], len(e['mb']), e['nnp'],
                         e['src_len'] - e['diffsum']))
        return
    pat = sys.argv[2]
    a = int(sys.argv[3]) if len(sys.argv) > 3 else 1
    b = int(sys.argv[4]) if len(sys.argv) > 4 else a + 40
    for e in ents:
        if pat not in e.get('rel', ''):
            continue
        if 'error' in e:
            print('ERR', e)
            continue
        rows = line_table(e)
        print('== %s  src_len=%d raw_len=%d lines=%d w=%d mb=%d crlf=%d'
              % (e['rel'], e['src_len'], e['raw_len'], len(rows), e['width'],
                 len(e['mb']), e['nnp']))
        for ln in range(a, min(b, len(rows)) + 1):
            s0, bl, cl, nm = rows[ln - 1]
            print('L%-5d off=%-7d bytes=%-4d chars=%-4d mb=%d' % (ln, s0, bl, cl, nm))


if __name__ == '__main__':
    main()
