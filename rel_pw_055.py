# -*- coding: utf-8 -*-
"""rel_pw_055.py — `tfm2_banpick_order_pw(tfm2).zip` 0.5.5 판 생성.

정규본 `release\\0.5.5\\tfm2_banpick_order.zip`(먼저 rel_055.py 로 생성)의 엔트리를
그대로 암호 "tfm2" 로 재포장한다. 소스·빌드·mod_id 차이 0 = 순수 배포 포장 변형.

구현 노트:
  - 0.5.4 판은 AES(compress_type 99) 였으나 이 머신엔 그 도구(pyzipper 류)가 현재 없다.
    여기서는 **ZipCrypto(전통 PKWARE 암호)** 를 순수 파이썬으로 구현해 쓴다.
    호환성은 ZipCrypto 쪽이 오히려 넓다(7-Zip·반디집·WinRAR·Windows 탐색기 모두 해제 가능,
    AES 는 탐색기 불가). 비번은 동일하게 "tfm2".
  - 생성 후 자체검증: zipfile 로 비번 해제 추출 → 정규본과 SHA256 전수 대조.
  - 게임 폴더는 샌드박스가 삭제를 막으므로 로컬 스테이징 후 copy.
"""
import os, io, sys, struct, shutil, zipfile, zlib, hashlib

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

GAME = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods"
SRC_ZIP = os.path.join(GAME, "release", "0.5.5", "tfm2_banpick_order.zip")
DST_ZIP = os.path.join(GAME, "release", "0.5.5", "tfm2_banpick_order_pw(tfm2).zip")
PWD = b"tfm2"

CRCTAB = []
for _i in range(256):
    _c = _i
    for _ in range(8):
        _c = (_c >> 1) ^ (0xEDB88320 if _c & 1 else 0)
    CRCTAB.append(_c)


def crc32u(v, b):
    return ((v >> 8) ^ CRCTAB[(v ^ b) & 0xFF]) & 0xFFFFFFFF


class Keys:
    def __init__(self, pwd):
        self.k0, self.k1, self.k2 = 0x12345678, 0x23456789, 0x34567890
        for b in pwd:
            self.update(b)

    def update(self, b):
        self.k0 = crc32u(self.k0, b)
        self.k1 = (self.k1 + (self.k0 & 0xFF)) & 0xFFFFFFFF
        self.k1 = (self.k1 * 134775813 + 1) & 0xFFFFFFFF
        self.k2 = crc32u(self.k2, self.k1 >> 24)

    def encrypt(self, data):
        out = bytearray()
        for b in data:
            t = (self.k2 | 2) & 0xFFFF
            out.append(b ^ (((t * (t ^ 1)) >> 8) & 0xFF))
            self.update(b)
        return bytes(out)


zin = zipfile.ZipFile(SRC_ZIP)
stage = os.path.join(os.environ["TEMP"], "bo_pw_055.zip")
out = io.BytesIO()
central = []

for i in zin.infolist():
    name = i.filename.encode("ascii")  # 이 zip 의 엔트리명은 전부 ASCII
    dt, dd = i.date_time, None
    dostime = (dt[3] << 11) | (dt[4] << 5) | (dt[5] // 2)
    dosdate = ((dt[0] - 1980) << 9) | (dt[1] << 5) | dt[2]
    off = out.tell()
    if i.filename.endswith("/"):  # 디렉토리 엔트리(비암호·stored) — 0.5.4 판과 동일 구성
        flags, method, crc, csize, usize, payload, extattr = 0, 0, 0, 0, 0, b"", 0x10
    else:
        data = zin.read(i.filename)
        crc = zlib.crc32(data) & 0xFFFFFFFF
        co = zlib.compressobj(9, zlib.DEFLATED, -15)
        comp = co.compress(data) + co.flush()
        keys = Keys(PWD)
        hdr = os.urandom(11) + bytes([(crc >> 24) & 0xFF])
        payload = keys.encrypt(hdr + comp)
        flags, method, csize, usize, extattr = 0x1, 8, len(payload), len(data), 0
    out.write(struct.pack("<IHHHHHIIIHH", 0x04034B50, 20, flags, method,
                          dostime, dosdate, crc, csize, usize, len(name), 0))
    out.write(name)
    out.write(payload)
    central.append((name, flags, method, dostime, dosdate, crc, csize, usize, extattr, off))

cd_off = out.tell()
for name, flags, method, dostime, dosdate, crc, csize, usize, extattr, off in central:
    out.write(struct.pack("<IHHHHHHIIIHHHHHII", 0x02014B50, (3 << 8) | 20, 20, flags, method,
                          dostime, dosdate, crc, csize, usize, len(name), 0, 0, 0, 0, extattr, off))
    out.write(name)
cd_size = out.tell() - cd_off
out.write(struct.pack("<IHHHHIIH", 0x06054B50, 0, 0, len(central), len(central), cd_size, cd_off, 0))

with open(stage, "wb") as f:
    f.write(out.getvalue())

# ── 자체검증: 비번 해제 추출 → 정규본과 SHA256 전수 대조 ──
zv = zipfile.ZipFile(stage)
bad = zv.testzip() if False else None  # testzip 은 pwd 없이는 못 씀 — 아래에서 직접 검증
ok = True
for i in zin.infolist():
    if i.filename.endswith("/"):
        continue
    a = hashlib.sha256(zin.read(i.filename)).hexdigest()
    b = hashlib.sha256(zv.read(i.filename, pwd=PWD)).hexdigest()
    if a != b:
        ok = False
        print(f"  !! 대조 불일치: {i.filename}")
try:
    zv.read([i for i in zv.infolist() if not i.filename.endswith("/")][0].filename, pwd=b"wrong")
    print("  !! 잘못된 비번이 통과됨(암호 미적용?)")
    ok = False
except RuntimeError:
    pass

if not ok:
    print("검증 실패 — 배포하지 않음")
    sys.exit(1)

shutil.copy2(stage, DST_ZIP)
zz = zipfile.ZipFile(DST_ZIP)
print(f"tfm2_banpick_order_pw(tfm2): {os.path.getsize(DST_ZIP):,}B  엔트리 {len(zz.namelist())}  "
      f"(비번 해제 SHA256 전수일치 · ZipCrypto · pwd='tfm2')")
