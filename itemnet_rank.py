# itemnet_rank.py — 세이브만으로 챔피언별 아이템 빌드 추천 순위를 오프라인 계산
#
# 게임을 켤 필요가 없다. 세이브의 신경망 weight 를 뽑아 게임의 forward 를 그대로 재현해
# 각 챔피언 × 포지션의 상위 N개 빌드를 뽑는다. 출력 CSV 는 tfm2_item_tactics 인게임 덤프와
# 같은 스키마라 `compare_builds.py A.csv B.csv` 로 그대로 비교된다.
#
# 사용:
#   python itemnet_rank.py <save.data>                       # 바닐라 최종템 6개만
#   python itemnet_rank.py <save.data> --auto-cands          # 모드템 포함(아래 두 파일에서 자동 수집)
#   python itemnet_rank.py <save.data> --auto-cands --exhaustive   # 완전탐색(느림)
#   python itemnet_rank.py A.data B.data --auto-cands        # 두 세이브 동시
#   python itemnet_rank.py <save.data> --champ knight --top 10 --print
#
# --auto-cands 소스 (둘 다 있으면 모드템 id+이름+티어가 붙는다):
#   <게임>/mods/tfm2_item_tactics/item_tactics_active.txt   ← id ↔ key   (모드 DB 스캔 산출물)
#   item_tree.json (meta_item_delegate → save_probe 산출물)  ← key ↔ tier
#   경로는 --active / --tree 로 덮어쓸 수 있다.
#
# ── 재현 근거 (RE 2026-07-31, Rust DefaultHasher ground-truth 와 비트동일 검증) ──
#   해시   = SipHash-1-3, key=(0,0)  ( = Rust `DefaultHasher` )
#   메시지 = name || 0xff || a_le8 || b_le8 || c_le8   ( = (&str,usize,usize,usize).hash() )
#   idx    = h % cap        (`& 0x3fff` 아님. cap = len(w))
#   부호   = (h>>32)&1 == 0 → +1.0, 아니면 −1.0       (최하위 비트가 아니다)
#   idx==0 폐기(0번 버킷 = bias 전용) → 같은 idx 는 누적 병합 → 전체 1회 L2 정규화
#   acc = w[0] + Σ val*w[idx] ;  p = sigmoid(acc) ;  노이즈는 끈 상태(결정론)
#
# ⚠ ctx = 중립 고정(본인 챔프만 배치, 아군·적군 나머지 전부 9999).
#   적이 전부 9999라 `lane_counter`·`global_counter` 두 피처는 **구조적으로 빠진다**.
#   A/B 비교에는 문제없지만(양쪽 동일 조건), "실제 경기 점수"와는 다르다.

import argparse, csv, gzip, io, json, math, os, re, struct
from itertools import combinations, permutations

WCOUNT = 16384
NONE_CHAMP = 9999

CHAMP_SHEET = [
    'swordman', 'monk', 'mod_champions', 'fighter', 'knight', 'archer', 'soldier', 'priest',
    'pythoness', 'pyromancer', 'ice_mage', 'ninja', 'magic_knight', 'berserker', 'executioner',
    'lancer', 'ogre', 'dual_blader', 'cavalry_knight', 'gunner', 'pole_warrior', 'jiangshi',
    'gambler', 'hammerer', 'demon', 'vampire', 'spirit_caller', 'boomerang_hunter', 'inquisitor',
    'shield_bearer', 'whip_master', 'werewolf', 'dokkaebi', 'necromancer', 'bard',
    'barrier_magician', 'chef', 'clown', 'dancer', 'dark_mage', 'exorcist', 'ghost', 'illusionist',
    'lightning_mage', 'plague_doctor', 'poison_dart_hunter', 'shadowmancer', 'taoist',
    'siege_breaker', 'android', 'druid', 'prisoner', 'bomber', 'voodoo_shaman', 'white_mage',
    'wind_mage', 'enchanter', 'hitman', 'guardian_spirit', 'hunter', 'circus_blade',
]
VANILLA_FINAL = [4, 9, 14, 19, 24, 29]
# 바닐라 최종템 id ↔ 이름 (item_tree.json 의 t5_* 아이콘 = 카테고리 0~5 의 티어4)
VANILLA_FINAL_NAME = {
    4: 'warlords_final_judgement', 9: 'storm_sovereign', 14: 'impregnable_fortress',
    19: 'veil_of_annihilation', 24: 'prophet_of_the_abyss', 29: 'giants_horn_shard',
}
DEF_CHAMPS = (r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2'
              r'\mods\tfm2_itemnet_tune\champ_ids.txt')
DEF_ACTIVE = (r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2'
              r'\mods\tfm2_item_tactics\item_tactics_active.txt')
DEF_TREE = (r'C:\Users\dev\Desktop\claude\tfm2\_backup\3738242091\DashboardApp\resources\app'
            r'\tfm2_meta_dashboard\data\save_probe_snapshot\mod_save\tfm2_meta_item_delegate'
            r'\item_tree.json')

# ─────────────────────────── SipHash-1-3 (key = 0,0) ───────────────────────────
M = (1 << 64) - 1


def _rotl(x, b):
    return ((x << b) | (x >> (64 - b))) & M


def _round(v0, v1, v2, v3):
    v0 = (v0 + v1) & M; v1 = _rotl(v1, 13); v1 ^= v0; v0 = _rotl(v0, 32)
    v2 = (v2 + v3) & M; v3 = _rotl(v3, 16); v3 ^= v2
    v0 = (v0 + v3) & M; v3 = _rotl(v3, 21); v3 ^= v0
    v2 = (v2 + v1) & M; v1 = _rotl(v1, 17); v1 ^= v2; v2 = _rotl(v2, 32)
    return v0, v1, v2, v3


def siphash13(data):
    v0, v1 = 0x736f6d6570736575, 0x646f72616e646f6d
    v2, v3 = 0x6c7967656e657261, 0x7465646279746573
    n = len(data)
    nb = n // 8
    for i in range(nb):
        m = int.from_bytes(data[i * 8:i * 8 + 8], 'little')
        v3 ^= m
        v0, v1, v2, v3 = _round(v0, v1, v2, v3)
        v0 ^= m
    tail = data[nb * 8:]
    b = ((n & 0xff) << 56) | int.from_bytes(tail + b'\x00' * (8 - len(tail)), 'little')
    v3 ^= b
    v0, v1, v2, v3 = _round(v0, v1, v2, v3)
    v0 ^= b
    v2 ^= 0xff
    for _ in range(3):
        v0, v1, v2, v3 = _round(v0, v1, v2, v3)
    return v0 ^ v1 ^ v2 ^ v3


def feat(name, a, b, c):
    return siphash13(name.encode() + b'\xff'
                     + a.to_bytes(8, 'little') + b.to_bytes(8, 'little') + c.to_bytes(8, 'little'))


def bucket(name, a, b, c, cap):
    """(idx, sign). idx==0 이면 None (0번 버킷 폐기)."""
    h = feat(name, a, b, c)
    i = h % cap
    if i == 0:
        return None
    return i, (1.0 if ((h >> 32) & 1) == 0 else -1.0)


# ─────────────────────────── 세이브 weight ───────────────────────────
def _looks_like_net(v):
    """학습된 가중치 배열인지 판정.
    ⚠유한·|x|<5 만으로는 부족하다 — 110MB 스트림에는 작은 float 처럼 보이는 영역이 널려 있어
      확장(65536=256KB 창)에서 가짜 후보가 수십 개 잡힌다. 실제 망은 거의 전 버킷이 0이 아니고
      값이 흩어져 있으므로 '0 아닌 비율'과 '분산'을 함께 본다."""
    nz = 0
    s = 0.0
    ss = 0.0
    n = len(v)
    for x in v:
        if x != x or x <= -5.0 or x >= 5.0:
            return False
        if x != 0.0:
            nz += 1
        s += x
        ss += x * x
    if nz < n * 0.5:
        return False
    var = ss / n - (s / n) ** 2
    return var > 1e-12


def read_save_weights(path):
    """버킷 수는 세이브에서 읽는다 — 확장된 세이브(65536 등)도 그대로 처리한다.
    `idx = h % cap` 의 cap 이 곧 이 길이이므로, 채점도 자동으로 맞는 모듈러를 쓴다."""
    raw = open(path, 'rb').read()
    if len(raw) < 21 or raw[:4] != b'TFM2':
        raise SystemExit(f'{path}: TFM2 세이브가 아닙니다')
    gz = raw.find(b'\x1f\x8b\x08')
    if gz < 0:
        raise SystemExit(f'{path}: gzip 블록 없음')
    dec = gzip.GzipFile(fileobj=io.BytesIO(raw[gz:])).read()
    for wc in (16384, 32768, 65536, 131072, 262144, 524288):
        wb = wc * 4
        target = struct.pack('<Q', wc)
        hits, i, end = [], dec.find(target), len(dec) - (8 + wb + 4)
        while i != -1 and i <= end:
            ws = i + 8
            # 꼬리 version==1 을 먼저 본다(싸다) — 가짜 후보를 즉시 걸러낸다
            if struct.unpack_from('<I', dec, ws + wb)[0] == 1:
                v = struct.unpack_from('<%df' % wc, dec, ws)
                if _looks_like_net(v):
                    hits.append(v)
            i = dec.find(target, i + 1)
        if len(hits) == 1:
            return list(hits[0])
        if len(hits) > 1:
            raise SystemExit(f'{path}: 버킷 {wc} 후보가 {len(hits)}개 — 중단')
    raise SystemExit(f'{path}: 아이템 신경망을 찾지 못했습니다')


# ─────────────────────────── 후보 아이템 수집 ───────────────────────────
def load_champs(path):
    """`champ_ids.txt`(모드가 실측 수집) → [(id, name)]. 없으면 내장 CHAMP_SHEET 로 폴백.
    반환 = (목록, 메모, 경고목록)."""
    warn = []
    if not os.path.exists(path):
        ids = [(i, CHAMP_SHEET[i]) for i in range(len(CHAMP_SHEET))
               if CHAMP_SHEET[i] != 'mod_champions']
        return ids, f'내장 시트 {len(ids)}종(모드 챔피언 제외)', [
            'champ_ids.txt 가 없어 모드 챔피언이 빠집니다. '
            'tfm2_itemnet_tune 을 켜고 경기를 진행하면 자동 생성됩니다.']
    m = {}
    for ln in open(path, encoding='utf-8'):
        ln = ln.strip()
        if not ln or ln.startswith('#'):
            continue
        parts = ln.split('\t')
        if len(parts) != 2:
            continue
        try:
            m[parts[1]] = int(parts[0])
        except ValueError:
            continue
    conflicts = [k for k in m if '#CONFLICT#' in k]
    if conflicts:
        warn.append(f'⚠ 같은 이름에 다른 id 가 관측됨 {len(conflicts)}건 — '
                    f'매핑이 커리어마다 다를 수 있어 다른 세이브에 소급 적용하면 안 됩니다: '
                    f'{", ".join(conflicts[:3])}')
        for k in conflicts:
            del m[k]
    # ★자체 검증: 바닐라 챔프의 실측 id 가 내장 시트 인덱스와 맞는가
    #   맞으면 = 레지스트리 순서가 시트 순서 = 설치 고정 속성 = 다른 세이브에 써도 됨
    bad = [(n, m[n], CHAMP_SHEET.index(n)) for n in m
           if n in CHAMP_SHEET and m[n] != CHAMP_SHEET.index(n)]
    if bad:
        warn.append(f'⚠ 내장 시트와 어긋나는 바닐라 챔프 {len(bad)}종 '
                    f'(내장 시트가 낡았을 수 있음): '
                    f'{", ".join(f"{n} 실측{a}≠시트{b}" for n, a, b in bad[:5])}')
    known = sum(1 for n in m if n in CHAMP_SHEET)
    ids = sorted(((i, n) for n, i in m.items()), key=lambda x: x[0])
    return ids, f'실측 {len(ids)}종 (내장 시트에 있는 것 {known}, 모드/신규 {len(ids)-known})', warn


def collect_cands(active_path, tree_path):
    """(id 리스트, {id: 이름}). 두 파일이 없으면 바닐라 6개만."""
    names = dict(VANILLA_FINAL_NAME)
    ids = list(VANILLA_FINAL)
    if not (os.path.exists(active_path) and os.path.exists(tree_path)):
        return ids, names, '모드템 소스 파일이 없어 바닐라 최종템 6개만 사용'
    key2id = {}
    for ln in open(active_path, encoding='utf-8', errors='replace'):
        m = re.match(r'\s*(\d+)\s*\|\s*(\S+)\s*\|\s*\S+\s*\|\s*(\S+)', ln)
        if m and m.group(2) in ('O', 'o'):          # 활성 아이템만
            key2id[m.group(3)] = int(m.group(1))
    items = json.load(open(tree_path, encoding='utf-8'))['items']
    nmod = 0
    for k, v in items.items():
        if v.get('tier') == 4 and k in key2id:      # 최종템 = tier 4
            i = key2id[k]
            ids.append(i)
            names[i] = k
            nmod += 1
    ids.sort()
    return ids, names, f'바닐라 최종템 6 + 모드 최종템 {nmod} = {len(ids)}개'


# ─────────────────────────── 사전계산 + 점수 ───────────────────────────
class Scorer:
    """중립 ctx 전용 고속 채점기.
    적이 전부 9999라 lane_counter·global_counter 는 발생하지 않으므로
    self_item / champ_pos_build / synergy 세 피처만 챔프별로 미리 해싱해 둔다."""

    def __init__(self, w, cands, slots, flag=True):
        self.w, self.cap, self.slots, self.flag = w, len(w), slots, flag
        self.cands = cands
        self.self_b = {}   # (champ, item, i) -> (idx,sign)
        self.cpb_b = {}    # (champ, lane, item)
        self.syn_b = {}    # (champ, a, b)  a<b

    def prep(self, me):
        cap = self.cap
        for it in self.cands:
            for i in range(self.slots):
                self.self_b[(me, it, i)] = bucket('self_item', me, it, i, cap)
            if self.flag:
                for lane in range(5):
                    self.cpb_b[(me, lane, it)] = bucket('champ_pos_build', me, lane, it, cap)
        for a, b in combinations(self.cands, 2):
            lo, hi = (a, b) if a < b else (b, a)
            self.syn_b[(me, lo, hi)] = bucket('synergy', me, lo, hi, cap)

    def score(self, me, lane, build):
        ent = {}
        for i, it in enumerate(build):
            e = self.self_b[(me, it, i)]
            if e:
                ent[e[0]] = ent.get(e[0], 0.0) + e[1]
            if self.flag:
                e = self.cpb_b[(me, lane, it)]
                if e:
                    ent[e[0]] = ent.get(e[0], 0.0) + e[1]
            for j in range(i + 1, len(build)):
                o = build[j]
                lo, hi = (it, o) if it < o else (o, it)
                e = self.syn_b[(me, lo, hi)]
                if e:
                    ent[e[0]] = ent.get(e[0], 0.0) + e[1]
        ss = 0.0
        for v in ent.values():
            ss += v * v
        w = self.w
        acc = w[0]
        if ss > 0.0:
            nrm = math.sqrt(ss)
            for i, v in ent.items():
                acc += (v / nrm) * w[i]
        return 1.0 / (1.0 + math.exp(-acc))


def _dedupe(pool, top):
    """상위 N을 **서로 다른 아이템 조합** 기준으로 고른다.
    같은 4개의 다른 배열이 1~3위를 차지하면 정보가 없으므로, 조합당 최선 배열 하나만 남긴다."""
    out, seen = [], set()
    for s, b in pool:
        k = frozenset(b)
        if k in seen:
            continue
        seen.add(k)
        out.append((s, b))
        if len(out) >= top:
            break
    return out


def top_builds(sc, me, lane, cands, slots, top, exhaustive, beam_w):
    """★빌드는 **순서 있는 수열**이다 — 피처 `self_item(챔프, 아이템, 순서)` 가 위치를 보기 때문에
    같은 4개라도 배열에 따라 점수가 달라진다(실측: 순서만 바꿔도 평균 0.011, 최대 0.049 차이.
    셀 간 1위↔3위 격차 0.004보다 크다). 그래서 오름차순 제약 없이 순열까지 탐색한다.
    ⚠구버전은 id 오름차순만 평가해 60회 중 58회 최선이 아니었다 — 그 결과는 폐기."""
    if exhaustive:
        pool = []
        for comb in combinations(cands, slots):
            for b in permutations(comb):
                pool.append((sc.score(me, lane, b), b))
        pool.sort(key=lambda x: -x[0])
        return _dedupe(pool, top)
    beam = [(0.0, ())]
    last_pool = []
    for d in range(slots):
        nxt = []
        for _, e in beam:
            for c in cands:
                if c in e:
                    continue          # 같은 아이템 중복만 금지(순서는 자유)
                b = e + (c,)
                nxt.append((sc.score(me, lane, b), b))
        nxt.sort(key=lambda x: -x[0])
        last_pool = nxt               # 마지막 단계의 **전체 풀**에서 dedupe 해야 서로 다른 조합이 모인다
        beam = nxt[:beam_w]
        if not beam:
            break
    return _dedupe(last_pool, top)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('saves', nargs='+')
    ap.add_argument('--slots', type=int, default=4)
    ap.add_argument('--top', type=int, default=3)
    ap.add_argument('--cands', help='후보 id 콤마 목록(직접 지정)')
    ap.add_argument('--auto-cands', action='store_true', help='모드템까지 자동 수집')
    ap.add_argument('--active', default=DEF_ACTIVE)
    ap.add_argument('--tree', default=DEF_TREE)
    ap.add_argument('--champs', default=DEF_CHAMPS, help='champ_ids.txt 경로(모드가 생성)')
    ap.add_argument('--champ')
    ap.add_argument('--exhaustive', action='store_true', help='완전탐색(느림). 기본은 beam')
    ap.add_argument('--beam', type=int, default=32)
    ap.add_argument('--flag0', action='store_true')
    ap.add_argument('--print', dest='show', action='store_true')
    a = ap.parse_args()

    names = dict(VANILLA_FINAL_NAME)
    if a.cands:
        cands = sorted(int(x) for x in a.cands.split(','))
        note = f'직접 지정 {len(cands)}개'
    elif a.auto_cands:
        cands, names, note = collect_cands(a.active, a.tree)
    else:
        cands, note = list(VANILLA_FINAL), '바닐라 최종템 6개(--auto-cands 로 모드템 포함)'
    if len(cands) < a.slots:
        raise SystemExit(f'후보 {len(cands)}개 < 칸 수 {a.slots}')

    champs, cnote, cwarn = load_champs(a.champs)
    for w in cwarn:
        print(f'  {w}')
    if a.champ:
        sel = [(i, n) for i, n in champs if n == a.champ]
        if not sel:
            raise SystemExit(f'알 수 없는 챔피언: {a.champ} (수집된 {len(champs)}종에 없음)')
        champs = sel
    cname = dict(champs)

    def label(i):
        return names.get(i, f'v{i//5}_t{i%5}' if i < 30 else f'id{i}')

    ncombo = math.comb(len(cands), a.slots)
    print(f'후보 {len(cands)}개 ({note}) / {a.slots}칸 조합 {ncombo:,} / '
          f'{"완전탐색" if a.exhaustive else f"beam 폭 {a.beam}"} / 챔프 {len(champs)} ({cnote})')

    for sp in a.saves:
        w = read_save_weights(sp)
        sc = Scorer(w, cands, a.slots, not a.flag0)
        rows = []
        for ci, cn in champs:
            sc.prep(ci)
            for pos in range(5):
                for rank, (s, b) in enumerate(
                        top_builds(sc, ci, pos, cands, a.slots, a.top, a.exhaustive, a.beam), 1):
                    rows.append((cn, pos, rank, s, list(b)))
        base = os.path.splitext(os.path.basename(sp))[0]
        out = f'ranked_{base}.csv'
        meta = [
            f'세이브={os.path.basename(sp)} / weight {len(w)}개',
            f'후보 {len(cands)}개 = {note}',
            f'{a.slots}칸 조합 {ncombo:,} / 탐색={"완전(순열포함)" if a.exhaustive else f"beam{a.beam} 순열탐색"} / 상위 {a.top}',
            'id 순서 = 실제 배열 순서(구매 순서에 대응). self_item 피처가 위치를 본다.',
            f'챔피언 {len(champs)}종 = {cnote}',
            f'flag={"0" if a.flag0 else "1(바닐라)"} / 노이즈 OFF(결정론)',
            'ctx = 중립 고정(본인 챔프만, 나머지 9999) ⟹ lane_counter·global_counter 피처는 발생하지 않음',
        ]
        with open(out, 'w', encoding='utf-8', newline='') as f:
            for m in meta:
                f.write('# ' + m + '\n')
            wr = csv.writer(f)
            wr.writerow(['champion', 'position', 'rank', 'score']
                        + [f'id{i}' for i in range(a.slots)]
                        + [f'item{i}' for i in range(a.slots)])
            for champ, pos, rank, s, b in rows:
                wr.writerow([champ, pos, rank, f'{s:.6f}'] + b + [label(x) for x in b])
        print(f'{os.path.basename(sp)} → {out}  ({len(rows)}행)')
        if a.show:
            cur = None
            for champ, pos, rank, s, b in rows:
                if (champ, pos) != cur:
                    cur = (champ, pos)
                    print(f'\n{champ}  pos{pos}')
                print(f'  {rank}. {s:.6f}  ' + ' + '.join(label(x) for x in b))


if __name__ == '__main__':
    main()
