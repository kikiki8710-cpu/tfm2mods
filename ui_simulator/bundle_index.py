# -*- coding: utf-8 -*-
"""bundle.game_data 레코드 인덱서.
레코드 포맷 (ANA\tfm2-ui-dsl-reference.md §1):
    [u32 extlen][ext][u32 pathlen][path][u32 bodylen][body]
"""
import struct
import sys
from collections import Counter
from pathlib import Path

BUNDLE = Path(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\bundle.game_data")


def scan(data):
    """(ext, path, body_off, body_len) 제너레이터. 포맷 어긋나면 중단.
    파일 선두 u32 = 레코드 수(헤더) → 오프셋 4부터 레코드."""
    pos = 4
    n = len(data)
    while pos + 4 <= n:
        (extlen,) = struct.unpack_from("<I", data, pos)
        if extlen == 0 or extlen > 64:
            break
        pos += 4
        ext = data[pos:pos + extlen].decode("ascii", "replace")
        pos += extlen
        (pathlen,) = struct.unpack_from("<I", data, pos)
        if pathlen == 0 or pathlen > 512:
            break
        pos += 4
        path = data[pos:pos + pathlen].decode("utf-8", "replace")
        pos += pathlen
        (bodylen,) = struct.unpack_from("<I", data, pos)
        pos += 4
        yield ext, path, pos, bodylen
        pos += bodylen


def main():
    data = BUNDLE.read_bytes()
    print(f"bundle {len(data):,}B")
    exts = Counter()
    ext_bytes = Counter()
    samples = {}
    total = 0
    last_end = 0
    for ext, path, off, blen in scan(data):
        total += 1
        exts[ext] += 1
        ext_bytes[ext] += blen
        if ext not in samples:
            samples[ext] = path
        last_end = off + blen
    print(f"records={total}, 커버리지 끝={last_end:,}B / {len(data):,}B")
    for ext, cnt in exts.most_common():
        print(f"  {ext:14s} {cnt:6d}개 {ext_bytes[ext]:>13,}B  예: {samples[ext]}")


if __name__ == "__main__":
    main()
