# -*- coding: utf-8 -*-
"""스테이징 폴더를 일반 zip + 비번 zip 두 벌로 굽는다.

비번 zip 은 기존 릴리스와 같은 **ZipCrypto**(전통 PKWARE)로 맞춘다.
  · 실측: 0.5.6\tfm2_ai_adjust_pw(tfm2).zip = flag 0x1 + method 8(deflate), AES(99) 아님.
  · 이 머신엔 7-Zip 도 pyzipper 도 없고 파이썬 zipfile 은 암호화 쓰기를 못 하므로 직접 구현한다.
    (pyzipper 를 깔면 AES 가 되는데, 그러면 기존 zip 과 형식이 달라져 옛 도구에서 안 열릴 수 있다.)
비번은 파일명 규약 그대로 `_pw(tfm2)` = tfm2.
"""
import io
import os
import struct
import sys
import time
import zlib

sys.stdout.reconfigure(encoding='utf-8', errors='replace')

# ── ZipCrypto ──────────────────────────────────────────────────────────────
TBL = []
for _i in range(256):
    _c = _i
    for _ in range(8):
        _c = (_c >> 1) ^ (0xEDB88320 if _c & 1 else 0)
    TBL.append(_c)


class Crypt:
    def __init__(self, pw):
        self.k = [0x12345678, 0x23456789, 0x34567890]
        for b in pw:
            self.upd(b)

    def upd(self, b):
        k = self.k
        k[0] = (k[0] >> 8) ^ TBL[(k[0] ^ b) & 0xFF]
        k[1] = (k[1] + (k[0] & 0xFF)) & 0xFFFFFFFF
        k[1] = (k[1] * 134775813 + 1) & 0xFFFFFFFF
        k[2] = (k[2] >> 8) ^ TBL[(k[2] ^ (k[1] >> 24)) & 0xFF]

    def enc(self, data):
        out = bytearray(len(data))
        for i, p in enumerate(data):
            t = (self.k[2] | 2) & 0xFFFF
            out[i] = p ^ (((t * (t ^ 1)) >> 8) & 0xFF)
            self.upd(p)
        return bytes(out)


def dostime(ts):
    t = time.localtime(ts)
    return (((t.tm_hour << 11) | (t.tm_min << 5) | (t.tm_sec // 2)),
            (((t.tm_year - 1980) << 9) | (t.tm_mon << 5) | t.tm_mday))


def build(files, out, pw=None):
    """files = [(디스크경로, zip내경로)]  pw=None 이면 평문 zip."""
    body, cd = bytearray(), bytearray()
    for disk, name in files:
        raw = open(disk, 'rb').read()
        crc = zlib.crc32(raw) & 0xFFFFFFFF
        co = zlib.compressobj(9, zlib.DEFLATED, -15)
        comp = co.compress(raw) + co.flush()
        flag = 0x0800                                   # 파일명 UTF-8
        if pw:
            flag |= 0x0001
            c = Crypt(pw.encode())
            hdr = bytearray(os.urandom(11)) + bytes([(crc >> 24) & 0xFF])
            comp = c.enc(bytes(hdr)) + c.enc(comp)
        tm, dt = dostime(os.path.getmtime(disk))
        nb = name.encode('utf-8')
        off = len(body)
        body += struct.pack('<IHHHHHIIIHH', 0x04034B50, 20, flag, 8, tm, dt,
                            crc, len(comp), len(raw), len(nb), 0) + nb + comp
        cd += struct.pack('<IHHHHHHIIIHHHHHII', 0x02014B50, 0x031E, 20, flag, 8, tm, dt,
                          crc, len(comp), len(raw), len(nb), 0, 0, 0, 0, 0x20, off) + nb
    eocd = struct.pack('<IHHHHIIH', 0x06054B50, 0, 0, len(files), len(files),
                       len(cd), len(body), 0)
    with open(out, 'wb') as f:
        f.write(bytes(body) + bytes(cd) + eocd)
    return os.path.getsize(out)


USAGE = r"""사용법:
  python mkzip_pw.py <스테이징폴더> <MOD_ID> <릴리스폴더> [비번]

  · 스테이징폴더 안에 <MOD_ID>\ 가 한 겹 있어야 한다(zip 루트 구조 = /deploy §4).
  · 비번을 주면 <MOD_ID>_pw(<비번>).zip 을 **추가로** 굽는다(평문 zip 은 항상 굽는다).
  예) python mkzip_pw.py C:\...\rel163\stage tfm2_ai_adjust C:\tfm2mods\release\0.5.8 tfm2
"""
if len(sys.argv) < 4:
    sys.exit(USAGE)
STG, MOD, DST = sys.argv[1], sys.argv[2], sys.argv[3]
PW = sys.argv[4] if len(sys.argv) > 4 else None
if not os.path.isdir(os.path.join(STG, MOD)):
    sys.exit("★스테이징에 %s\\ 폴더가 없다: %s" % (MOD, STG))
files = []
for root, _, fs in os.walk(os.path.join(STG, MOD)):
    for f in sorted(fs):
        p = os.path.join(root, f)
        files.append((p, os.path.relpath(p, STG).replace('\\', '/')))
files.sort(key=lambda x: x[1])
# 기존 zip 과 같은 순서(루트 파일 먼저, 그다음 하위폴더)로 맞춘다
files.sort(key=lambda x: (x[1].count('/') > 1, x[1]))
print("엔트리 %d개" % len(files))

n1 = '%s.zip' % MOD
print("  %-28s %9d B" % (n1, build(files, os.path.join(DST, n1))))
if PW:
    n2 = '%s_pw(%s).zip' % (MOD, PW)
    print("  %-28s %9d B" % (n2, build(files, os.path.join(DST, n2), pw=PW)))
print("\n★굽고 나면 반드시 풀어서 검증할 것 — 전 엔트리 해제 CRC · 두 zip 내용 동일 ·")
print("  비번 없이/틀린 비번으로 거부되는지 · zip 안 dll == 배포 dll(SHA) · mod_info 첫 바이트 0x7b.")
