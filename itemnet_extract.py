# itemnet_extract.py — TFM2 세이브에서 아이템 추천 신경망(LogisticSGD) weight 추출·비교
#
# 세이브 포맷(itemnet_reset_tool 실측본과 동일):
#   [비압축 헤더 "TFM2"…] + [gzip 멤버 = Database Spitz 직렬화]
#   해제 스트림 안: [u64 len==16384][f32 × 16384][u64 fdim==1]   ← 세이브당 유일 매치
#
# 사용:
#   python itemnet_extract.py <save.data>                  # 요약 통계
#   python itemnet_extract.py <save.data> -o w.bin         # weight 16384개를 f32 raw 로 저장
#   python itemnet_extract.py --cmp <A.data> <B.data>      # A/B 학습량 비교
#   python itemnet_extract.py --cmp <A.data> <B.data> --factory <base_item_network.item_network>
#   python itemnet_extract.py --list                       # 기본 세이브 폴더 목록
#
# ⚠이 도구가 하는 것 = weight 추출과 "얼마나 학습했나" 비교.
#   "어떤 빌드를 추천하나"는 피처 해싱 재현이 있어야 계산 가능하다(별도 RE 진행 중).
#   그때까지 빌드 순위는 tfm2_item_tactics 의 인게임 덤프(dump_builds.trigger)를 쓸 것.

import argparse, gzip, io, os, struct, sys, math

WCOUNT = 16384
WBYTES = WCOUNT * 4

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



def default_save_dir():
    ad = os.environ.get('APPDATA')
    if not ad:
        return None
    p = os.path.join(ad, 'TeamSamoyed', 'TeamfightManager2', 'data')
    return p if os.path.isdir(p) else None


def find_gzip(raw):
    i = raw.find(b'\x1f\x8b\x08')
    return i if i >= 0 else None


SIZES = (16384, 32768, 65536, 131072, 262144, 524288)


def locate_itemnet(dec, wc):
    """[u64 wc][f32×wc][u32 version==1] 를 만족하는 시작 오프셋 전부."""
    hits = []
    wb = wc * 4
    end = len(dec) - (8 + wb + 4)
    if end < 0:
        return hits
    target = struct.pack('<Q', wc)
    i = dec.find(target)
    while i != -1 and i <= end:
        ws = i + 8
        # ★꼬리 version==1 을 먼저 본다 — float 전수검사보다 훨씬 싸서
        #   가짜 후보(수십 MB 스트림에 같은 u64 패턴이 수천 번 나온다)를 즉시 걸러낸다.
        if struct.unpack_from('<I', dec, ws + wb)[0] == 1:
            vals = struct.unpack_from('<%df' % wc, dec, ws)
            if _looks_like_net(vals):
                hits.append((i, vals))
        i = dec.find(target, i + 1)
    return hits


def read_save(path):
    """★버킷 수는 세이브에서 읽는다 — 확장본(65536·131072 등)도 그대로 처리한다."""
    raw = open(path, 'rb').read()
    if len(raw) < 21 or raw[:4] != b'TFM2':
        raise SystemExit(f'{path}: TFM2 세이브가 아닙니다(매직 불일치)')
    gz = find_gzip(raw)
    if gz is None:
        raise SystemExit(f'{path}: gzip 블록을 찾지 못했습니다(포맷 변경?)')
    dec = gzip.GzipFile(fileobj=io.BytesIO(raw[gz:])).read()
    for wc in SIZES:
        hits = locate_itemnet(dec, wc)
        if len(hits) == 1:
            return list(hits[0][1])
        if len(hits) > 1:
            raise SystemExit(f'{path}: 버킷 {wc} 후보가 {len(hits)}개 — 안전을 위해 중단합니다')
    raise SystemExit(f'{path}: 아이템 신경망을 찾지 못했습니다')


def read_factory(path):
    """공장 기본망 파일: [u64 count][f32×count][u32 fdim]"""
    b = open(path, 'rb').read()
    n = struct.unpack_from('<Q', b, 0)[0]
    if n != WCOUNT:
        raise SystemExit(f'{path}: count={n} (기대 {WCOUNT})')
    return struct.unpack_from('<%df' % WCOUNT, b, 8)


def stats(w):
    nz = [v for v in w if v != 0.0]
    l2 = math.sqrt(sum(v * v for v in w))
    return {
        'l2': l2,
        'nonzero': len(nz),
        'min': min(w),
        'max': max(w),
        'mean': sum(w) / len(w),
        'absmean': sum(abs(v) for v in w) / len(w),
    }


def show(tag, w):
    s = stats(w)
    n = len(w)
    print(f'[{tag}]')
    print(f'  버킷 수      : {n:,}')
    print(f'  L2 norm      : {s["l2"]:.6f}')
    print(f'  0 아닌 버킷  : {s["nonzero"]:,} / {n:,}  ({s["nonzero"]/n*100:.1f}%)')
    print(f'  범위         : {s["min"]:+.6f} ~ {s["max"]:+.6f}')
    print(f'  평균 / |평균|: {s["mean"]:+.6e} / {s["absmean"]:.6e}')


def compare(wa, wb, la='A', lb='B'):
    if len(wa) != len(wb):
        print(f'\n⚠ 버킷 수가 다릅니다 ({la} {len(wa):,} vs {lb} {len(wb):,}) — 직접 비교 불가.')
        print('   확장은 무손실이지만 배열 길이가 달라 버킷 단위 대응이 성립하지 않습니다.')
        print('   같은 버킷 수끼리 비교하거나, 짧은 쪽을 같은 크기로 확장한 뒤 비교하세요.')
        return
    n = len(wa)
    d = [b - a for a, b in zip(wa, wb)]
    l2d = math.sqrt(sum(v * v for v in d))
    l2a = math.sqrt(sum(v * v for v in wa))
    changed = sum(1 for v in d if abs(v) > 1e-9)
    # 코사인 유사도 = 두 망이 얼마나 같은 방향인가
    dot = sum(a * b for a, b in zip(wa, wb))
    nb = math.sqrt(sum(v * v for v in wb))
    cos = dot / (l2a * nb) if l2a > 0 and nb > 0 else float('nan')

    print(f'\n=== {la} → {lb} 변화 ===')
    print(f'  바뀐 버킷    : {changed:,} / {n:,}  ({changed/n*100:.1f}%)')
    print(f'  ||Δ||        : {l2d:.6f}   (기준 ||{la}|| = {l2a:.6f})')
    print(f'  상대 변화량  : {l2d/l2a*100:.2f}%' if l2a > 0 else '  상대 변화량  : n/a')
    print(f'  코사인 유사도: {cos:.6f}   (1.0 = 완전히 같은 방향)')

    idx = sorted(range(n), key=lambda i: -abs(d[i]))[:15]
    print(f'  가장 많이 움직인 버킷 15개:')
    print(f'    {"bucket":>7} {la:>12} {lb:>12} {"Δ":>12}')
    for i in idx:
        if abs(d[i]) < 1e-9:
            break
        print(f'    {i:>7} {wa[i]:>+12.6f} {wb[i]:>+12.6f} {d[i]:>+12.6f}')
    print('  ※ 버킷은 SipHash 해시값이라 "어떤 아이템"인지는 피처 해싱 재현 후에 알 수 있습니다.')


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('save', nargs='?')
    ap.add_argument('-o', '--out', help='weight 16384개를 f32 raw 로 저장')
    ap.add_argument('--cmp', nargs=2, metavar=('A', 'B'), help='두 세이브 비교')
    ap.add_argument('--factory', help='공장 기본망 파일(base_item_network.item_network) 경로')
    ap.add_argument('--list', action='store_true', help='기본 세이브 폴더 목록')
    a = ap.parse_args()

    if a.list:
        d = default_save_dir()
        if not d:
            print('기본 세이브 폴더를 찾지 못했습니다.')
            return
        print(d)
        for f in sorted(os.listdir(d)):
            if f.endswith('.data'):
                p = os.path.join(d, f)
                print(f'  {f:<40} {os.path.getsize(p):>12,} B')
        return

    fac = read_factory(a.factory) if a.factory else None

    if a.cmp:
        wa, wb = read_save(a.cmp[0]), read_save(a.cmp[1])
        show(os.path.basename(a.cmp[0]), wa)
        print()
        show(os.path.basename(a.cmp[1]), wb)
        if fac:
            print()
            show('공장 기본망', fac)
            compare(fac, wa, '공장', os.path.basename(a.cmp[0]))
            compare(fac, wb, '공장', os.path.basename(a.cmp[1]))
        compare(wa, wb, os.path.basename(a.cmp[0]), os.path.basename(a.cmp[1]))
        return

    if not a.save:
        ap.error('세이브 파일 경로 또는 --cmp / --list 가 필요합니다')
    w = read_save(a.save)
    show(os.path.basename(a.save), w)
    if fac:
        print()
        compare(fac, w, '공장', os.path.basename(a.save))
    if a.out:
        with open(a.out, 'wb') as f:
            f.write(struct.pack('<%df' % len(w), *w))
        print(f'\n저장: {a.out}  ({len(w) * 4:,} B)')


if __name__ == '__main__':
    main()
