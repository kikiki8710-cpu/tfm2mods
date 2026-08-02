# itemnet_expand.py — 아이템 추천망 해시 버킷 확장 (무손실)
#
# 왜: 피처 공간이 버킷 수(16384)를 크게 초과해 충돌이 ~100:1 이다. 그래서 챔프별 아이템 선호가
#     충돌 배경에 묻혀 점수 폭이 노이즈의 1/40 수준까지 떨어진다. 버킷을 늘리면 충돌만 쪼개진다.
#
# ★무손실인 이유 (수학적 확정):
#     게임은 idx = hash % cap 을 쓴다. 새 cap 이 옛 cap 의 **배수**이면
#         h % (k·cap) ≡ h (mod cap)
#     이므로  w_new[j] = w_old[j % cap]  (옛 배열을 k번 타일링) 하면
#     **모든 피처의 가중치가 정확히 보존**되고 충돌만 분리된다. bias w[0] 도 보존된다.
#
# ★안전한 이유 (RE 2026-08-01 확정):
#     · forward/update/dense/hash/beam 전 범위에 16384 하드코딩이 **없다**(전부 w.len() 구동)
#     · 세이브 역직렬화가 길이 접두를 읽고 **길이 검증이 없다**
#       ⟹ 확장된 세이브는 **모드 없는 바닐라 게임에서도 그대로 로드**된다(원복 자유)
#     · 추론(빌드 생성) 비용은 O(피처 수)라 **cap 과 무관** — 느려지지 않는다
#       늘어나는 건 학습 비용만: 65536(4배)은 경기당 ~18MB zeroing = 무시 가능
#
# 사용:
#   python itemnet_expand.py --inspect <save.data 또는 *.item_network>
#   python itemnet_expand.py --save <save.data> --size 65536          # 새 파일로 출력
#   python itemnet_expand.py --save <save.data> --size 65536 --in-place   # 백업 후 덮어쓰기
#   python itemnet_expand.py --asset <in.item_network> --size 65536 -o <out.item_network>
#
# 되돌리기 = 백업 파일을 복원하면 된다(축소는 학습이 갈라져 무손실이 불가능하므로 지원하지 않는다).

import argparse, gzip, io, os, shutil, struct, sys

COMP_LEN_OFF = 13          # 세이브 헤더의 gzip 길이 필드(로더가 검증하는 유일한 값)
MAX_REASONABLE = 1 << 24   # sanity 상한

def _looks_like_net(vals):
    """학습된 가중치 배열인지 판정.
    ⚠유한·|x|<5 만으로는 부족하다 — 큰 스트림에는 전부 0인 영역이나 작은 float 처럼 보이는
      구간이 널려 있어 가짜 후보가 잡힌다(실제로 L2=0.000000 오탐 발생). 실제 망은 거의 전
      버킷이 0이 아니고 값이 흩어져 있으므로 '0 아닌 비율'과 '분산'을 함께 본다."""
    nz = 0; s = 0.0; ss = 0.0; n = len(vals)
    for x in vals:
        if x != x or x <= -5.0 or x >= 5.0:
            return False
        if x != 0.0:
            nz += 1
        s += x; ss += x * x
    if nz < n * 0.5:
        return False
    return ss / n - (s / n) ** 2 > 1e-12



# ─────────────────────────── 공통 ───────────────────────────
def find_block(buf, expect_len=None):
    """[u64 len][f32 × len][u32 version==1] 블록을 찾는다.
    반환 = [(start, len, version_off)] — start 는 u64 길이 접두의 위치."""
    hits = []
    i = 0
    n = len(buf)
    while True:
        # 길이 후보를 하나씩 훑는 대신, 이미 아는 길이가 있으면 그 패턴만 찾는다(훨씬 빠름)
        if expect_len is not None:
            i = buf.find(struct.pack('<Q', expect_len), i)
            if i < 0:
                break
            cand = [expect_len]
        else:
            break
        for L in cand:
            wb = L * 4
            if i + 8 + wb + 4 > n:
                continue
            ws = i + 8
            ver_off = ws + wb
            if struct.unpack_from('<I', buf, ver_off)[0] != 1:
                continue
            vals = struct.unpack_from('<%df' % L, buf, ws)
            if _looks_like_net(vals):
                hits.append((i, L, ver_off))
        i += 1
    return hits


def tile(vals, factor):
    """w_new[j] = w_old[j % old]  — 옛 배열을 factor 번 이어붙인다."""
    return list(vals) * factor


def make_block(new_vals):
    return (struct.pack('<Q', len(new_vals))
            + struct.pack('<%df' % len(new_vals), *new_vals)
            + struct.pack('<I', 1))


def check_size(old, new):
    if new <= old:
        raise SystemExit(f'새 크기({new})가 현재({old})보다 크지 않습니다. 축소는 무손실이 불가능합니다.')
    if new % old != 0:
        raise SystemExit(
            f'새 크기({new})가 현재({old})의 배수가 아닙니다.\n'
            f'  배수여야만 h %% new ≡ h (mod old) 가 성립해 무손실이 됩니다.\n'
            f'  가능한 값: {", ".join(str(old*k) for k in (2,4,8,16,32))}')
    if new > MAX_REASONABLE:
        raise SystemExit(f'새 크기({new})가 과도합니다.')
    return new // old


# ─────────────────────────── 에셋 ───────────────────────────
def read_asset(path):
    b = open(path, 'rb').read()
    if len(b) < 12:
        raise SystemExit(f'{path}: 너무 작습니다')
    cnt = struct.unpack_from('<Q', b, 0)[0]
    if len(b) != 8 + cnt * 4 + 4:
        raise SystemExit(f'{path}: 크기 불일치 (count={cnt} → 기대 {8+cnt*4+4}, 실제 {len(b)})')
    ver = struct.unpack_from('<I', b, 8 + cnt * 4)[0]
    return cnt, list(struct.unpack_from('<%df' % cnt, b, 8)), ver


def do_asset(src, dst, size):
    cnt, vals, ver = read_asset(src)
    k = check_size(cnt, size)
    print(f'  원본 : {os.path.basename(src)}  count={cnt:,}  version={ver}  ({8+cnt*4+4:,} B)')
    new = tile(vals, k)
    out = struct.pack('<Q', len(new)) + struct.pack('<%df' % len(new), *new) + struct.pack('<I', ver)
    open(dst, 'wb').write(out)
    print(f'  출력 : {os.path.basename(dst)}  count={len(new):,}  ({len(out):,} B)  타일링 {k}회')
    verify = read_asset(dst)
    assert verify[0] == size and all(verify[1][j] == vals[j % cnt] for j in range(0, size, 997))
    print('  검증 : w_new[j] == w_old[j % old]  OK')


# ─────────────────────────── 세이브 ───────────────────────────
def do_save(path, size, out_path, in_place):
    raw = open(path, 'rb').read()
    if len(raw) < 21 or raw[:4] != b'TFM2':
        raise SystemExit(f'{path}: TFM2 세이브가 아닙니다')
    gz = raw.find(b'\x1f\x8b\x08')
    if gz < 0:
        raise SystemExit(f'{path}: gzip 블록을 찾지 못했습니다')
    header, comp = raw[:gz], raw[gz:]
    orig_comp_len = len(comp)
    dec = gzip.GzipFile(fileobj=io.BytesIO(comp)).read()

    hits = find_block(dec, expect_len=16384)
    if not hits:
        # 이미 확장된 세이브인지 확인
        for L in (size, 32768, 65536, 131072, 262144):
            if find_block(dec, expect_len=L):
                raise SystemExit(f'{path}: 이미 {L:,} 버킷입니다(16384 블록 없음)')
        raise SystemExit(f'{path}: 아이템 신경망 블록을 찾지 못했습니다')
    if len(hits) > 1:
        raise SystemExit(f'{path}: 블록 후보가 {len(hits)}개 — 안전을 위해 중단합니다')

    start, L, ver_off = hits[0]
    k = check_size(L, size)
    vals = list(struct.unpack_from('<%df' % L, dec, start + 8))
    end = ver_off + 4
    print(f'  블록 : 오프셋 {start:,}  버킷 {L:,}  →  {size:,}  (타일링 {k}회)')

    new_dec = dec[:start] + make_block(tile(vals, k)) + dec[end:]
    print(f'  스트림: {len(dec):,} B → {len(new_dec):,} B  (+{len(new_dec)-len(dec):,})')

    newcomp = gzip.compress(new_dec, 6)
    out = bytearray(header + newcomp)
    # 헤더의 comp_len 갱신 — 로더가 검증하는 유일한 필드
    nl = struct.pack('<Q', len(newcomp))
    if len(header) >= COMP_LEN_OFF + 8 and struct.unpack_from('<Q', out, COMP_LEN_OFF)[0] == orig_comp_len:
        out[COMP_LEN_OFF:COMP_LEN_OFF + 8] = nl
    else:
        want = struct.pack('<Q', orig_comp_len)
        p = out.find(want, 0, min(len(header), 64))
        if p < 0:
            raise SystemExit('헤더의 comp_len 필드를 찾지 못했습니다(포맷 변경?)')
        out[p:p + 8] = nl
    print(f'  gzip  : {orig_comp_len:,} B → {len(newcomp):,} B  (comp_len 갱신됨)')

    if in_place:
        bak = path + '.pre_expand'
        n = 1
        while os.path.exists(bak):
            bak = f'{path}.pre_expand.{n}'; n += 1
        shutil.copy2(path, bak)
        open(path, 'wb').write(out)
        print(f'  백업 : {bak}')
        print(f'  저장 : {path}  ({len(out):,} B)')
        final = path
    else:
        open(out_path, 'wb').write(out)
        print(f'  저장 : {out_path}  ({len(out):,} B)   ※원본은 그대로입니다')
        final = out_path

    # 되읽어 검증
    r2 = open(final, 'rb').read()
    d2 = gzip.GzipFile(fileobj=io.BytesIO(r2[r2.find(b'\x1f\x8b\x08'):])).read()
    h2 = find_block(d2, expect_len=size)
    if not h2:
        raise SystemExit('⚠ 검증 실패: 저장된 파일에서 확장 블록을 다시 찾지 못했습니다')
    v2 = struct.unpack_from('<%df' % size, d2, h2[0][0] + 8)
    bad = [j for j in range(0, size, 991) if v2[j] != vals[j % L]]
    if bad:
        raise SystemExit(f'⚠ 검증 실패: 타일링 불일치 {len(bad)}건')
    print(f'  검증 : 버킷 {size:,} · w_new[j] == w_old[j % {L}]  OK')


def do_inspect(path):
    if path.lower().endswith('.item_network'):
        cnt, vals, ver = read_asset(path)
        import math
        print(f'  에셋  count={cnt:,}  version={ver}  L2={math.sqrt(sum(v*v for v in vals)):.6f}')
        return
    raw = open(path, 'rb').read()
    gz = raw.find(b'\x1f\x8b\x08')
    dec = gzip.GzipFile(fileobj=io.BytesIO(raw[gz:])).read()
    for L in (16384, 32768, 65536, 131072, 262144, 524288):
        h = find_block(dec, expect_len=L)
        if h:
            import math
            v = struct.unpack_from('<%df' % L, dec, h[0][0] + 8)
            print(f'  세이브  버킷={L:,}  오프셋={h[0][0]:,}  L2={math.sqrt(sum(x*x for x in v)):.6f}')
            return
    print('  아이템 신경망 블록을 찾지 못했습니다')


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--save')
    ap.add_argument('--asset')
    ap.add_argument('--inspect')
    ap.add_argument('--size', type=int, default=65536)
    ap.add_argument('-o', '--out')
    ap.add_argument('--in-place', action='store_true', help='세이브를 백업 후 덮어쓰기')
    a = ap.parse_args()

    if a.inspect:
        do_inspect(a.inspect); return
    if a.asset:
        out = a.out or (os.path.splitext(a.asset)[0] + f'_{a.size}.item_network')
        print(f'[에셋 확장 → {a.size:,} 버킷]')
        do_asset(a.asset, out, a.size); return
    if a.save:
        base, ext = os.path.splitext(a.save)
        out = a.out or f'{base}_bk{a.size}{ext}'
        print(f'[세이브 확장 → {a.size:,} 버킷]')
        do_save(a.save, a.size, out, a.in_place); return
    ap.error('--save / --asset / --inspect 중 하나가 필요합니다')


if __name__ == '__main__':
    main()
