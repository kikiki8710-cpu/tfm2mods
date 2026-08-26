# -*- coding: utf-8 -*-
"""rel_pw.py — 임의의 릴리스 zip을 암호(ZipCrypto) 걸린 `_pw(<비번>)` 변형으로 재포장.

용도: 백신이 zip 안의 dll/exe를 스캔해 격리·다운로드 차단하는 환경 대응
      (암호 zip은 내용 스캔이 불가 ⇒ 오탐 격리 회피). 내용물은 정규본과 100% 동일.

사용: python rel_pw.py <원본zip경로> [비번]        (비번 기본 "tfm2")
출력: 같은 폴더에 `<원본이름>_pw(<비번>).zip`

구현 노트(rel_pw_056.py 일반화):
  - ZipCrypto(전통 PKWARE 암호) 순수 파이썬 구현. 7-Zip·반디집·WinRAR·Windows 탐색기
    모두 해제 가능(AES-256은 탐색기가 못 여니 호환성상 ZipCrypto 유지).
  - 비-ASCII 엔트리명은 UTF-8 + 범용플래그 0x800 로 기록(예: ai_adjust 의 한글 exe).
  - 생성 후 자체검증: 비번 해제 추출 → 정규본과 SHA256 전수 대조 + 오답 비번 거부 확인.
  - 게임 폴더 샌드박스 회피용으로 TEMP 스테이징 후 copy2.
"""
import os, io, sys, struct, shutil, zipfile, zlib, hashlib

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

SRC_ZIP = os.path.abspath(sys.argv[1])
PWD = (sys.argv[2] if len(sys.argv) > 2 else "tfm2").encode("utf-8")
stem = os.path.splitext(os.path.basename(SRC_ZIP))[0]
DST_ZIP = os.path.join(os.path.dirname(SRC_ZIP), f"{stem}_pw({PWD.decode()}).zip")

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
stage = os.path.join(os.environ["TEMP"], f"pwzip_{stem}.zip")
out = io.BytesIO()
central = []

for i in zin.infolist():
    try:
        name, utf8 = i.filename.encode("ascii"), 0
    except UnicodeEncodeError:
        name, utf8 = i.filename.encode("utf-8"), 0x800
    dt = i.date_time
    dostime = (dt[3] << 11) | (dt[4] << 5) | (dt[5] // 2)
    dosdate = ((dt[0] - 1980) << 9) | (dt[1] << 5) | dt[2]
    off = out.tell()
    if i.filename.endswith("/"):  # 디렉토리 엔트리 = 비암호·stored
        flags, method, crc, csize, usize, payload, extattr = utf8, 0, 0, 0, 0, b"", 0x10
    else:
        data = zin.read(i.filename)
        crc = zlib.crc32(data) & 0xFFFFFFFF
        co = zlib.compressobj(9, zlib.DEFLATED, -15)
        comp = co.compress(data) + co.flush()
        keys = Keys(PWD)
        hdr = os.urandom(11) + bytes([(crc >> 24) & 0xFF])  # 검사바이트 = CRC 상위
        payload = keys.encrypt(hdr + comp)
        flags, method, csize, usize, extattr = 0x1 | utf8, 8, len(payload), len(data), 0
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

# ── 자체검증: 비번 해제 추출 → 정규본과 SHA256 전수 대조 + 오답 비번 거부 ──
zv = zipfile.ZipFile(stage)
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
    zv.read([i for i in zv.infolist() if not i.filename.endswith("/")][0].filename, pwd=b"__wrong__")
    print("  !! 잘못된 비번이 통과됨(암호 미적용?)")
    ok = False
except RuntimeError:
    pass

if not ok:
    print("검증 실패 — 배포하지 않음")
    sys.exit(1)

shutil.copy2(stage, DST_ZIP)
zz = zipfile.ZipFile(DST_ZIP)
print(f"OK: {os.path.basename(DST_ZIP)}  {os.path.getsize(DST_ZIP):,}B  엔트리 {len(zz.namelist())}  "
      f"(원본 {os.path.getsize(SRC_ZIP):,}B · 비번 해제 SHA256 전수일치 · ZipCrypto · pwd='{PWD.decode()}')")
