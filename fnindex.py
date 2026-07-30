# -*- coding: utf-8 -*-
# fnindex.py — exe 전 함수의 구조 지문 인덱스 빌더.
#   왜: 0.5.2→0.5.3 은 핫픽스가 아니라 전면 재컴파일(.text +10.5%, 함수 +11,965개)이라
#       migrate_rva.py 의 연속바이트 마스크시그가 전멸한다. 바이트 대신 "명령 구조"로 맞춘다.
#   지문 3종:
#     skel      = 명령 시퀀스에서 immediate/변위/분기타겟만 I 로 치환한 정규화 문자열의 md5 (완전일치용)
#     skel_head = 앞 24개 명령만으로 같은 계산 (본체 일부 변경 내성)
#     mnem      = 니모닉 빈도 벡터 (코사인 유사도용)
# 사용: python fnindex.py <exe> <out.pkl>
import struct, sys, re, io, hashlib, pickle, collections, time
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = False                      # detail 불필요 — op_str 정규화로 충분하고 5배 이상 빠르다
RE_NUM = re.compile(r'0x[0-9a-f]+')

def load(p):
    d = open(p, "rb").read()
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    nsec = struct.unpack_from("<H", d, pe + 6)[0]
    opt = pe + 24
    ib = struct.unpack_from("<Q", d, opt + 24)[0]
    sectab = opt + struct.unpack_from("<H", d, pe + 20)[0]
    secs = []
    for i in range(nsec):
        o = sectab + i * 40
        nm = d[o:o + 8].rstrip(b"\0").decode(errors="replace")
        vsz, va, rsz, rraw = struct.unpack_from("<IIII", d, o + 8)
        secs.append((nm, va, vsz, rraw, rsz))
    magic = struct.unpack_from("<H", d, opt)[0]
    ddir = opt + (112 if magic == 0x20b else 96)
    ex_rva, ex_sz = struct.unpack_from("<II", d, ddir + 3 * 8)
    return d, ib, secs, ex_rva, ex_sz

def roff(secs, rva):
    for nm, va, vsz, rraw, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            return rraw + (rva - va)
    return None

def build(path):
    d, ib, secs, ex_rva, ex_sz = load(path)
    po = roff(secs, ex_rva)
    n = ex_sz // 12
    # .pdata 는 (begin, end, unwind) 정렬 배열. 같은 함수가 여러 조각으로 쪼개진 경우(chained unwind)
    # begin 이 중복 등장하므로 begin 기준으로 최대 end 를 취한다.
    ranges = {}
    for i in range(n):
        b, e, u = struct.unpack_from("<III", d, po + i * 12)
        if e <= b or e - b > 1 << 20:
            continue
        if b not in ranges or e > ranges[b]:
            ranges[b] = e
    print(f"  RUNTIME_FUNCTION {n} → 고유 함수 {len(ranges)}")

    idx = {}                       # rva -> dict(skel, head, size, mnem)
    by_skel = collections.defaultdict(list)
    by_head = collections.defaultdict(list)
    t0 = time.time()
    for k, (b, e) in enumerate(sorted(ranges.items())):
        off = roff(secs, b)
        if off is None:
            continue
        code = d[off:off + (e - b)]
        toks = []
        mn = collections.Counter()
        for ins in md.disasm(code, ib + b):
            op = RE_NUM.sub("I", ins.op_str)
            toks.append(ins.mnemonic + "|" + op)
            mn[ins.mnemonic] += 1
        if not toks:
            continue
        skel = hashlib.md5("\n".join(toks).encode()).hexdigest()
        head = hashlib.md5("\n".join(toks[:24]).encode()).hexdigest()
        idx[b] = dict(skel=skel, head=head, size=e - b, ninsn=len(toks), mnem=dict(mn))
        by_skel[skel].append(b)
        by_head[head].append(b)
        if k % 20000 == 0 and k:
            print(f"  ... {k}/{len(ranges)}  {time.time()-t0:.0f}s", flush=True)
    print(f"  완료 {len(idx)}함수  {time.time()-t0:.0f}s")
    return dict(path=path, idx=idx, by_skel=dict(by_skel), by_head=dict(by_head))

if __name__ == "__main__":
    src, out = sys.argv[1], sys.argv[2]
    print(f"[build] {src}")
    r = build(src)
    pickle.dump(r, open(out, "wb"), protocol=4)
    print(f"[saved] {out}")
