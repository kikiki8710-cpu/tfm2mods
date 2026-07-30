# RVA 구조매칭 결과 0.5.2 → 0.5.3

> 방식: `.pdata` 함수경계 + 명령 스켈레톤 해시 + 니모닉 코사인. 연속바이트 마스크시그(migrate_rva.py)는 이번 패치에서 전멸해 사용 불가.

등급: **L1_EXACT**=구조 완전일치·유일(교체 가능) / **L2_HEAD**=앞 24명령 일치 / **L3_SIM**=니모닉 유사 상위(확정 필요) / **L1_MULTI·L2_MULTI·L3_WEAK**=후보 다중 / **NONE**=미발견(ghidra-re 필요)

전체 집계: L3_WEAK 126 / L3_SIM 101 / NOT_IN_TEXT 39 / L2_HEAD 32 / L1_EXACT 26 / L1_MULTI 13 / NONE 4 / L2_MULTI 3

## tfm2_ai_adjust — 29/264 해결 · L3_WEAK 104 / L3_SIM 84 / NOT_IN_TEXT 30 / L1_EXACT 16 / L1_MULTI 15 / L2_HEAD 11 / L2_MULTI 2 / NONE 2

| 상수 | 0.5.2 | → 0.5.3 | 등급 | 종류 | 위치 |
|---|---|---|---|---|---|
| `(inline)` | `0x20def90` | `0xc8db70, 0xca22c0` | L1_MULTI | inline | src/detour.rs:311 |
| `(inline)` | `0x1b934a4` | `0xdec6b0, 0xd06810` | L3_SIM | inline | src/detour.rs:684 |
| `(inline)` | `0x1b934b0` | `0xdec6b0, 0xd06810` | L3_SIM | inline | src/detour.rs:685 |
| `(inline)` | `0x1b934ec` | `0xdec6b0, 0xd06810` | L3_SIM | inline | src/detour.rs:686 |
| `(inline)` | `0x1b9351c` | `0xdec6b0, 0xd06810` | L3_SIM | inline | src/detour.rs:687 |
| `(inline)` | `0x1b9302c` | `0xdec6b0, 0xd06810` | L3_SIM | inline | src/detour.rs:688 |
| `(inline)` | `0x1b93152` | `0xdec6b0, 0xd06810` | L3_SIM | inline | src/detour.rs:689 |
| `(inline)` | `0x1b933d8` | `0xdec6b0, 0xd06810` | L3_SIM | inline | src/detour.rs:690 |
| `(inline)` | `0x1bdac25` | `0xdf9320, 0x16887f0` | L3_SIM | inline | src/detour.rs:692 |
| `(inline)` | `0x1bdac95` | `0xdf9320, 0x16887f0` | L3_SIM | inline | src/detour.rs:693 |
| `(inline)` | `0x2376e86` | `0xcb02a0, 0xd94d00` | L3_WEAK | inline | src/detour.rs:695 |
| `(inline)` | `0x23777fe` | `0xcb02a0, 0xd94d00` | L3_WEAK | inline | src/detour.rs:696 |
| `(inline)` | `0x237780a` | `0xcb02a0, 0xd94d00` | L3_WEAK | inline | src/detour.rs:697 |
| `(inline)` | `0x2126ae3` | `0xc42db0, 0x2195ef0` | L3_SIM | inline | src/detour.rs:719 |
| `(inline)` | `0x2126ae3` | `0xc42db0, 0x2195ef0` | L3_SIM | inline | src/detour.rs:722 |
| `(inline)` | `0x22b2555` | `0xe06c10, 0xd48ec0` | L3_SIM | inline | src/detour.rs:778 |
| `(inline)` | `0x22b2ca5` | `0xe06c10, 0xd48ec0` | L3_SIM | inline | src/detour.rs:779 |
| `(inline)` | `0x22b2bb1` | `0xe06c10, 0xd48ec0` | L3_SIM | inline | src/detour.rs:780 |
| `(inline)` | `0x22b58ad` | `0xe06c10, 0xd48ec0` | L3_SIM | inline | src/detour.rs:781 |
| `(inline)` | `0x2398342` | `0xcc3960, 0xd6f720` | L3_WEAK | inline | src/detour.rs:783 |
| `(inline)` | `0x2398ef3` | `0xcc3960, 0xd6f720` | L3_WEAK | inline | src/detour.rs:784 |
| `(inline)` | `0x2398f3c` | `0xcc3960, 0xd6f720` | L3_WEAK | inline | src/detour.rs:785 |
| `(inline)` | `0x23ad9d7` | `—` | L2_HEAD | inline | src/detour.rs:788 |
| `(inline)` | `0x23ba8f3` | `0xdc2850, 0xce8520` | L3_WEAK | inline | src/detour.rs:789 |
| `(inline)` | `0x22b43ae` | `0xe06c10, 0xd48ec0` | L3_SIM | inline | src/detour.rs:790 |
| `(inline)` | `0x22e3cdf` | `0xcc9d70, 0xdaf780` | L3_SIM | inline | src/detour.rs:833 |
| `(inline)` | `0x22e3cf0` | `0xcc9d70, 0xdaf780` | L3_SIM | inline | src/detour.rs:834 |
| `(inline)` | `0x22e3cf6` | `0xcc9d70, 0xdaf780` | L3_SIM | inline | src/detour.rs:835 |
| `(inline)` | `0x22e3d00` | `0xcc9d70, 0xdaf780` | L3_SIM | inline | src/detour.rs:836 |
| `(inline)` | `0x22e3d06` | `0xcc9d70, 0xdaf780` | L3_SIM | inline | src/detour.rs:837 |
| `(inline)` | `0x22e3d10` | `0xcc9d70, 0xdaf780` | L3_SIM | inline | src/detour.rs:838 |
| `(inline)` | `0x22e3d16` | `0xcc9d70, 0xdaf780` | L3_SIM | inline | src/detour.rs:839 |
| `(inline)` | `0x22e3d2b` | `0xcc9d70, 0xdaf780` | L3_SIM | inline | src/detour.rs:840 |
| `(inline)` | `0x22e3d2f` | `0xcc9d70, 0xdaf780` | L3_SIM | inline | src/detour.rs:841 |
| `(inline)` | `0x22e3d33` | `0xcc9d70, 0xdaf780` | L3_SIM | inline | src/detour.rs:842 |
| `(inline)` | `0x22edb5f` | `0xd159f0, 0x24625c0` | L3_WEAK | inline | src/detour.rs:844 |
| `(inline)` | `0x22edb65` | `0xd159f0, 0x24625c0` | L3_WEAK | inline | src/detour.rs:845 |
| `(inline)` | `0x22edb6b` | `0xd159f0, 0x24625c0` | L3_WEAK | inline | src/detour.rs:846 |
| `(inline)` | `0x22edb71` | `0xd159f0, 0x24625c0` | L3_WEAK | inline | src/detour.rs:847 |
| `(inline)` | `0x22edb7b` | `0xd159f0, 0x24625c0` | L3_WEAK | inline | src/detour.rs:848 |
| `(inline)` | `0x22effff` | `0xcd4c6f` | L1_EXACT | inline | src/detour.rs:850 |
| `(inline)` | `0x22f0005` | `0xcd4c75` | L1_EXACT | inline | src/detour.rs:851 |
| `(inline)` | `0x22f000b` | `0xcd4c7b` | L1_EXACT | inline | src/detour.rs:852 |
| `(inline)` | `0x22f0011` | `0xcd4c81` | L1_EXACT | inline | src/detour.rs:853 |
| `(inline)` | `0x22f0017` | `0xcd4c87` | L1_EXACT | inline | src/detour.rs:854 |
| `(inline)` | `0x22f001d` | `0xcd4c8d` | L1_EXACT | inline | src/detour.rs:855 |
| `(inline)` | `0x22f0023` | `0xcd4c93` | L1_EXACT | inline | src/detour.rs:856 |
| `(inline)` | `0x23a0c21` | `0xc7f640, 0xd82200` | L3_SIM | inline | src/detour.rs:858 |
| `(inline)` | `0x23a0c27` | `0xc7f640, 0xd82200` | L3_SIM | inline | src/detour.rs:859 |
| `(inline)` | `0x23a0c2d` | `0xc7f640, 0xd82200` | L3_SIM | inline | src/detour.rs:860 |
| `(inline)` | `0x23a0c33` | `0xc7f640, 0xd82200` | L3_SIM | inline | src/detour.rs:861 |
| `(inline)` | `0x23a0c39` | `0xc7f640, 0xd82200` | L3_SIM | inline | src/detour.rs:862 |
| `(inline)` | `0x23a0c41` | `0xc7f640, 0xd82200` | L3_SIM | inline | src/detour.rs:863 |
| `(inline)` | `0x23a0c47` | `0xc7f640, 0xd82200` | L3_SIM | inline | src/detour.rs:864 |
| `SIMUNCHUNK_RVA` | `0x19b40c3` | `0x25b12e0, 0x2998780` | L3_SIM | const | src/detour.rs:880 |
| `(inline)` | `0x2380e16` | `0xdece30, 0xd94d00` | L3_WEAK | inline | src/disc19_repro.rs:52 |
| `(inline)` | `0x2380e22` | `0xdece30, 0xd94d00` | L3_WEAK | inline | src/disc19_repro.rs:53 |
| `(inline)` | `0x2380e2e` | `0xdece30, 0xd94d00` | L3_WEAK | inline | src/disc19_repro.rs:54 |
| `(inline)` | `0x2380e3c` | `0xdece30, 0xd94d00` | L3_WEAK | inline | src/disc19_repro.rs:55 |
| `(inline)` | `0x2380e1c` | `0xdece30, 0xd94d00` | L3_WEAK | inline | src/disc19_repro.rs:57 |
| `(inline)` | `0x2380e28` | `0xdece30, 0xd94d00` | L3_WEAK | inline | src/disc19_repro.rs:58 |
| `(inline)` | `0x2380e36` | `0xdece30, 0xd94d00` | L3_WEAK | inline | src/disc19_repro.rs:59 |
| `(inline)` | `0x2380e92` | `0xdece30, 0xd94d00` | L3_WEAK | inline | src/disc19_repro.rs:61 |
| `(inline)` | `0x2380ec0` | `0xdece30, 0xd94d00` | L3_WEAK | inline | src/disc19_repro.rs:62 |
| `(inline)` | `0x2380ecd` | `0xdece30, 0xd94d00` | L3_WEAK | inline | src/disc19_repro.rs:64 |
| `(inline)` | `0x1d204c0` | `0x14b6b50, 0x14bc230` | L3_WEAK | inline | src/disc19_repro.rs:259 |
| `(inline)` | `0x1f23a60` | `0xf18320, 0x12d7b60` | L3_WEAK | inline | src/disc19_repro.rs:259 |
| `(inline)` | `0x1a5ee60` | `—` | L2_HEAD | inline | src/disc19_repro.rs:260 |
| `(inline)` | `0x1d1f630` | `0x14b67c0` | L1_EXACT | inline | src/disc19_repro.rs:261 |
| `(inline)` | `0x1dce1d0` | `0x29a5b90, 0x2825ec0` | L3_WEAK | inline | src/disc19_repro.rs:266 |
| `(inline)` | `0x1d328e0` | `0x302f0a0, 0x3027660` | L3_WEAK | inline | src/disc19_repro.rs:270 |
| `(inline)` | `0x23a4d90` | `0xcc6180, 0x2166cc0` | L3_WEAK | inline | src/disc19_repro.rs:285 |
| `(inline)` | `0x20a3fd0` | `0x24b4ab0, 0x22c3a00` | L3_WEAK | inline | src/disc19_repro.rs:465 |
| `(inline)` | `0x20a3fd0` | `0x24b4ab0, 0x22c3a00` | L3_WEAK | inline | src/disc19_repro.rs:466 |
| `(inline)` | `0x1c974a0` | `0x120afa0, 0x10b4ca0` | L3_WEAK | inline | src/disc19_repro.rs:629 |
| `(inline)` | `0x1fce700` | `0x2904520, 0x2997310` | L3_WEAK | inline | src/disc19_repro.rs:726 |
| `(inline)` | `0x1fce700` | `0x2904520, 0x2997310` | L3_WEAK | inline | src/disc19_repro.rs:727 |
| `(inline)` | `0x1fbe950` | `0x1ec54d0, 0xa0cf30` | L3_WEAK | inline | src/disc19_repro.rs:736 |
| `(inline)` | `0x1fbe950` | `0x1ec54d0, 0xa0cf30` | L3_WEAK | inline | src/disc19_repro.rs:737 |
| `(inline)` | `0x19ed260` | `0x160f120, 0x1016dd0` | L3_SIM | inline | src/disc19_repro.rs:762 |
| `(inline)` | `0x19f2f60` | `0x10dd360, 0x10ebc40` | L3_SIM | inline | src/disc19_repro.rs:762 |
| `(inline)` | `0x1a13cb0` | `0x11020a0, 0x1a96470` | L3_WEAK | inline | src/disc19_repro.rs:762 |
| `(inline)` | `0xb024b0` | `—` | L2_HEAD | inline | src/disc19_repro.rs:763 |
| `(inline)` | `0x19ed250` | `0x160f120, 0x1016dd0` | L3_SIM | inline | src/disc19_repro.rs:763 |
| `(inline)` | `0x1a3a240` | `0x144c530, 0x16621d0` | L3_WEAK | inline | src/disc19_repro.rs:763 |
| `(inline)` | `0x1e85540` | `—` | L2_HEAD | inline | src/disc19_repro.rs:764 |
| `(inline)` | `0x9a1230` | `0x16270, 0x16c10` | L1_MULTI | inline | src/disc19_repro.rs:795 |
| `(inline)` | `0x1bbe3c0` | `0x2c0ba40, 0xee9100` | L3_WEAK | inline | src/disc19_repro.rs:796 |
| `(inline)` | `0x1a13cb0` | `0x11020a0, 0x1a96470` | L3_WEAK | inline | src/disc19_repro.rs:797 |
| `(inline)` | `0x50fc80` | `0xa8ed10, 0x1aa89c0` | L3_SIM | inline | src/disc19_repro.rs:1402 |
| `(inline)` | `0x5418a0` | `0x23c9490, 0x23c8230` | L3_WEAK | inline | src/disc19_repro.rs:1402 |
| `(inline)` | `0x19ec2c0` | `0x160f120, 0x1016dd0` | L3_SIM | inline | src/disc19_repro.rs:1402 |
| `(inline)` | `0x50fc80` | `0xa8ed10, 0x1aa89c0` | L3_SIM | inline | src/disc19_repro.rs:1405 |
| `(inline)` | `0x5418a0` | `0x23c9490, 0x23c8230` | L3_WEAK | inline | src/disc19_repro.rs:1406 |
| `(inline)` | `0x19ec2c0` | `0x160f120, 0x1016dd0` | L3_SIM | inline | src/disc19_repro.rs:1407 |
| `(inline)` | `0x1e66f40` | `0x28ddab0, 0x274ce50` | L3_WEAK | inline | src/disc19_repro.rs:1408 |
| `(inline)` | `0x1eacc00` | `0x2995d90, 0x29554e0` | L3_WEAK | inline | src/disc19_repro.rs:1409 |
| `(inline)` | `0x1e65a80` | `0x2208870, 0x2433e90` | L3_WEAK | inline | src/disc19_repro.rs:1414 |
| `(inline)` | `0x1f23eb0` | `0xf26280, 0x160b020` | L3_SIM | inline | src/disc19_repro.rs:1420 |
| `(inline)` | `0x1d1edd0` | `—` | L2_HEAD | inline | src/disc19_repro.rs:1421 |
| `(inline)` | `0x2291570` | `0x288f8d0, 0x288f890` | L3_WEAK | inline | src/disc19_repro.rs:1422 |
| `(inline)` | `0x50fc80` | `0xa8ed10, 0x1aa89c0` | L3_SIM | inline | src/disc19_repro.rs:1468 |
| `(inline)` | `0x5418a0` | `0x23c9490, 0x23c8230` | L3_WEAK | inline | src/disc19_repro.rs:1469 |
| `(inline)` | `0x1f23dd0` | `0xf26280, 0x160b020` | L3_SIM | inline | src/disc19_repro.rs:1470 |
| `(inline)` | `0x1ce1070` | `0x20a3ca0, 0x10f0160` | L3_WEAK | inline | src/disc19_repro.rs:1471 |
| `(inline)` | `0x23a4f80` | `0xcc6180, 0x2166cc0` | L3_WEAK | inline | src/disc19_repro.rs:1472 |
| `(inline)` | `0x23b5790` | `0xe29f30, 0x2b0e990` | L3_WEAK | inline | src/disc19_repro.rs:1473 |
| `(inline)` | `0x50fc80` | `0xa8ed10, 0x1aa89c0` | L3_SIM | inline | src/disc19_repro.rs:1481 |
| `(inline)` | `0x5418a0` | `0x23c9490, 0x23c8230` | L3_WEAK | inline | src/disc19_repro.rs:1482 |
| `(inline)` | `0x1f23d70` | `0xf26250` | L1_EXACT | inline | src/disc19_repro.rs:1483 |
| `(inline)` | `0x1a671e0` | `0x1155ad0, 0x1bd46d0` | L3_SIM | inline | src/disc19_repro.rs:1484 |
| `(inline)` | `0x1d1ed70` | `0x14b5f30` | L1_EXACT | inline | src/disc19_repro.rs:1486 |
| `(inline)` | `0x1faac80` | `0x3020af0, 0x3073740` | L3_WEAK | inline | src/disc19_repro.rs:1487 |
| `(inline)` | `0x23a4f60` | `0xcc6180, 0x2166cc0` | L3_WEAK | inline | src/disc19_repro.rs:1488 |
| `(inline)` | `0x23b5770` | `0xe29f30, 0x2b0e990` | L3_WEAK | inline | src/disc19_repro.rs:1489 |
| `(inline)` | `0x9c8850` | `0x3dbf60, 0x3dff60` | L3_WEAK | inline | src/disc19_repro.rs:1498 |
| `(inline)` | `0x5418a0` | `0x23c9490, 0x23c8230` | L3_WEAK | inline | src/disc19_repro.rs:1499 |
| `(inline)` | `0x1f23f90` | `0xf26280, 0x160b020` | L3_SIM | inline | src/disc19_repro.rs:1500 |
| `(inline)` | `0x1ce1090` | `0x20a3ca0, 0x10f0160` | L3_WEAK | inline | src/disc19_repro.rs:1501 |
| `(inline)` | `0x1ce10f0` | `0x2118c90, 0x2116d80` | L3_WEAK | inline | src/disc19_repro.rs:1501 |
| `(inline)` | `0x1fabac0` | `0x15f4ce0, 0x25c2910` | L3_WEAK | inline | src/disc19_repro.rs:1502 |
| `(inline)` | `0x1ff1970` | `0x23c09f0, 0x24d9660` | L3_WEAK | inline | src/disc19_repro.rs:1503 |
| `(inline)` | `0x23a5080` | `0xcc6180, 0x2166cc0` | L3_WEAK | inline | src/disc19_repro.rs:1504 |
| `(inline)` | `0x23b5890` | `0xe29f30, 0x2b0e990` | L3_WEAK | inline | src/disc19_repro.rs:1505 |
| `(inline)` | `0x50fc80` | `0xa8ed10, 0x1aa89c0` | L3_SIM | inline | src/disc19_repro.rs:1518 |
| `(inline)` | `0x1a671e0` | `0x1155ad0, 0x1bd46d0` | L3_SIM | inline | src/disc19_repro.rs:1519 |
| `(inline)` | `0x1d1ed70` | `0x14b5f30` | L1_EXACT | inline | src/disc19_repro.rs:1519 |
| `(inline)` | `0x1f77e30` | `0x124d280, 0x2b37330` | L3_SIM | inline | src/disc19_repro.rs:1519 |
| `(inline)` | `0x23bd430` | `0xe2caa0, 0x8bd0a0` | L3_SIM | inline | src/disc19_repro.rs:1519 |
| `(inline)` | `0x5418a0` | `0x23c9490, 0x23c8230` | L3_WEAK | inline | src/disc19_repro.rs:1520 |
| `(inline)` | `0x1faac80` | `0x3020af0, 0x3073740` | L3_WEAK | inline | src/disc19_repro.rs:1520 |
| `(inline)` | `0x23bd370` | `0xe2c710, 0xe2ec30` | L1_MULTI | inline | src/disc19_repro.rs:1520 |
| `(inline)` | `0x23bd3d0` | `0xe2caa0, 0x8bd0a0` | L3_SIM | inline | src/disc19_repro.rs:1520 |
| `(inline)` | `0x50fc80` | `0xa8ed10, 0x1aa89c0` | L3_SIM | inline | src/disc19_repro.rs:1525 |
| `(inline)` | `0x5418a0` | `0x23c9490, 0x23c8230` | L3_WEAK | inline | src/disc19_repro.rs:1526 |
| `(inline)` | `0x1faac80` | `0x3020af0, 0x3073740` | L3_WEAK | inline | src/disc19_repro.rs:1527 |
| `(inline)` | `0x1f77e30` | `0x124d280, 0x2b37330` | L3_SIM | inline | src/disc19_repro.rs:1528 |
| `(inline)` | `0x23bd430` | `0xe2caa0, 0x8bd0a0` | L3_SIM | inline | src/disc19_repro.rs:1539 |
| `(inline)` | `0x1a671e0` | `0x1155ad0, 0x1bd46d0` | L3_SIM | inline | src/disc19_repro.rs:1541 |
| `(inline)` | `0x1d1ed70` | `0x14b5f30` | L1_EXACT | inline | src/disc19_repro.rs:1543 |
| `(inline)` | `0x23bd370` | `0xe2c710, 0xe2ec30` | L1_MULTI | inline | src/disc19_repro.rs:1544 |
| `(inline)` | `0x23bd3d0` | `0xe2caa0, 0x8bd0a0` | L3_SIM | inline | src/disc19_repro.rs:1545 |
| `(inline)` | `0x1f23680` | `0x1756f0, 0x183ce0` | L1_MULTI | inline | src/disc19_repro.rs:1546 |
| `(inline)` | `0x50fc80` | `0xa8ed10, 0x1aa89c0` | L3_SIM | inline | src/disc19_repro.rs:1566 |
| `(inline)` | `0x1f236f0` | `0x2d420, 0x35240` | L1_MULTI | inline | src/disc19_repro.rs:1566 |
| `(inline)` | `0x20958d0` | `0x2904520, 0x2997310` | L3_WEAK | inline | src/disc19_repro.rs:1566 |
| `(inline)` | `0x50fc80` | `0xa8ed10, 0x1aa89c0` | L3_SIM | inline | src/disc19_repro.rs:1570 |
| `(inline)` | `0x1f236f0` | `0x2d420, 0x35240` | L1_MULTI | inline | src/disc19_repro.rs:1571 |
| `(inline)` | `0x20958d0` | `0x2904520, 0x2997310` | L3_WEAK | inline | src/disc19_repro.rs:1585 |
| `(inline)` | `0x1f23d30` | `0xf26210` | L1_EXACT | inline | src/disc19_repro.rs:1599 |
| `(inline)` | `0x23a49f0` | `0xcc6180, 0x2166cc0` | L3_WEAK | inline | src/disc19_repro.rs:1599 |
| `(inline)` | `0x50fc80` | `0xa8ed10, 0x1aa89c0` | L3_SIM | inline | src/disc19_repro.rs:2220 |
| `(inline)` | `0x1f236f0` | `0x2d420, 0x35240` | L1_MULTI | inline | src/disc19_repro.rs:2220 |
| `(inline)` | `0x20958d0` | `0x2904520, 0x2997310` | L3_WEAK | inline | src/disc19_repro.rs:2220 |
| `(inline)` | `0x50fc80` | `0xa8ed10, 0x1aa89c0` | L3_SIM | inline | src/disc19_repro.rs:2221 |
| `(inline)` | `0x5418a0` | `0x23c9490, 0x23c8230` | L3_WEAK | inline | src/disc19_repro.rs:2221 |
| `(inline)` | `0x19ec2c0` | `0x160f120, 0x1016dd0` | L3_SIM | inline | src/disc19_repro.rs:2221 |
| `(inline)` | `0x50fc80` | `0xa8ed10, 0x1aa89c0` | L3_SIM | inline | src/disc19_repro.rs:2223 |
| `(inline)` | `0x3886538` | `—` | NOT_IN_TEXT | inline | src/disc19_repro.rs:2268 |
| `(inline)` | `0x237d910` | `0xc5e160, 0xda0750` | L3_WEAK | inline | src/disc19_repro.rs:2386 |
| `(inline)` | `0x237d910` | `0xc5e160, 0xda0750` | L3_WEAK | inline | src/disc19_repro.rs:2387 |
| `(inline)` | `0x1a36e3` | `0x9bc50, 0x9bd20` | L1_MULTI | inline | src/disc19_repro.rs:2423 |
| `(inline)` | `0x9502f9` | `—` | L2_HEAD | inline | src/disc19_repro.rs:2728 |
| `(inline)` | `0x236b6b0` | `0x1264890` | L1_EXACT | inline | src/disc19_repro.rs:2814 |
| `(inline)` | `0x236b6b0` | `0x1264890` | L1_EXACT | inline | src/disc19_repro.rs:2817 |
| `(inline)` | `0x18c3090` | `0x2bb3070, 0x2dca690` | NONE | inline | src/genbuild_repro.rs:59 |
| `(inline)` | `0x1bc6f10` | `0x1080f20, 0x107d780` | L2_HEAD | inline | src/genbuild_repro.rs:73 |
| `(inline)` | `0x1db1eb0` | `0xcc9d70, 0xc9ce50` | L3_WEAK | inline | src/genbuild_repro.rs:157 |
| `(inline)` | `0x1db2c30` | `0xcc9d70, 0xc9ce50` | L3_WEAK | inline | src/genbuild_repro.rs:161 |
| `(inline)` | `0x1a45ba0` | `0x1c925c0, 0x1149160` | L3_WEAK | inline | src/genbuild_repro.rs:162 |
| `(inline)` | `0x35f5f28` | `—` | NOT_IN_TEXT | inline | src/genbuild_repro.rs:587 |
| `RVA_GB_ATKCTX_CB30` | `0x35d8018` | `—` | NOT_IN_TEXT | const | src/genbuild_repro.rs:698 |
| `RVA_GB_ATKCTX_C0690` | `0x35efd48` | `—` | NOT_IN_TEXT | const | src/genbuild_repro.rs:699 |
| `DESCS` | `0x1c7d5f9` | `0x1468ce0, 0x10f07b0` | L3_WEAK | array[3] | src/knobs.rs:25 |
| `DESCS` | `0x1c7df47` | `0x1468ce0, 0x10f07b0` | L3_WEAK | array[3] | src/knobs.rs:25 |
| `DESCS` | `0x1caedd3` | `0x4c510, 0x2903e20` | L3_WEAK | array[3] | src/knobs.rs:25 |
| `(inline)` | `0x2000000` | `0x29bbbf0` | L3_SIM | inline | src/mem_safety.rs:130 |
| `(inline)` | `0x2000000` | `0x29bbbf0` | L3_SIM | inline | src/mem_safety.rs:142 |
| `TEXT_END_RVA` | `0x2c087ff` | `—` | NOT_IN_TEXT | const | src/mem_safety.rs:310 |
| `(inline)` | `0x2000000` | `0x29bbbf0` | L3_SIM | inline | src/mem_safety.rs:520 |
| `(inline)` | `0x2000000` | `0x29bbbf0` | L3_SIM | inline | src/mem_safety.rs:721 |
| `(inline)` | `0x2000000` | `0x29bbbf0` | L3_SIM | inline | src/mem_safety.rs:733 |
| `(inline)` | `0x2000000` | `0x29bbbf0` | L3_SIM | inline | src/mem_safety.rs:738 |
| `RVA_RETREAT` | `0x1b94670` | `0xe00350` | L3_SIM | const | src/rva_052.rs:15 |
| `RVA_TG_CALL` | `0x1feca43` | `0x73e780, 0x7a0f00` | L1_MULTI | const | src/rva_052.rs:17 |
| `RVA_THREATGATE_FN` | `0x20a8680` | `0x1865730, 0x15d4110` | L3_WEAK | const | src/rva_052.rs:19 |
| `RVA_F2_BUILD_CALL` | `0x22dd4fe` | `—` | NOT_IN_TEXT | const | src/rva_052.rs:23 |
| `RVA_GENERIC_BUILD` | `0x22b2280` | `0xe06c10` | L3_SIM | const | src/rva_052.rs:25 |
| `RVA_FC59A0` | `0x1bdb3e0` | `0xe168d0` | L3_SIM | const | src/rva_052.rs:29 |
| `RVA_TABLE_A` | `0x3828818` | `—` | NOT_IN_TEXT | const | src/rva_052.rs:32 |
| `RVA_GB_REGIOND_HOOK` | `0x22dafea` | `0x5aa40, 0x96f70` | L1_MULTI | const | src/rva_052.rs:36 |
| `RVA_GB_FUNNEL` | `0x22dbc4e` | `0x15513b0, 0x1551e80` | L3_WEAK | const | src/rva_052.rs:40 |
| `RVA_CONDGATE` | `0x21338d0` | `0xc550b0` | L3_SIM | const | src/rva_052.rs:43 |
| `RVA_MOVEPRI` | `0x2134240` | `0xc559e0` | L3_SIM | const | src/rva_052.rs:45 |
| `RVA_COMMIT_CALL` | `0x1e3dfd2` | `0x2904520, 0x2997310` | L3_WEAK | const | src/rva_052.rs:51 |
| `RVA_COMMIT_FN` | `0x235ffa0` | `0x15f8690` | L1_EXACT | const | src/rva_052.rs:53 |
| `RVA_ENGAGE_GATE` | `0x1c9b33d` | `0x1ec54d0, 0xa0cf30` | L3_WEAK | const | src/rva_052.rs:57 |
| `RVA_DISC18_HANDLER` | `0x2376320` | `0xcb02a0, 0xd94d00` | L3_WEAK | const | src/rva_052.rs:64 |
| `RVA_DISC19_HANDLER` | `0x2380820` | `0xdece30, 0xd94d00` | L3_WEAK | const | src/rva_052.rs:66 |
| `RVA_ITEMNET_SCORER` | `0x1b9cce0` | `0x10587e0` | L3_SIM | const | src/rva_052.rs:76 |
| `RVA_C8C_DMG_SHEET` | `0x381e1e0` | `—` | NOT_IN_TEXT | const | src/rva_052.rs:86 |
| `RVA_DISC7_DMG_SHEET` | `0x38d1918` | `—` | NOT_IN_TEXT | const | src/rva_052.rs:97 |
| `(inline)` | `0x19ed660` | `0x160f120, 0x1016dd0` | L3_SIM | inline | src/serpen.rs:626 |
| `(inline)` | `0x19f2f60` | `0x10dd360, 0x10ebc40` | L3_SIM | inline | src/serpen.rs:627 |
| `(inline)` | `0x19ed250` | `0x160f120, 0x1016dd0` | L3_SIM | inline | src/serpen.rs:628 |
| `(inline)` | `0x1a3a240` | `0x144c530, 0x16621d0` | L3_WEAK | inline | src/serpen.rs:629 |
| `(inline)` | `0xb024b0` | `—` | L2_HEAD | inline | src/serpen.rs:630 |
| `(inline)` | `0x50fc80` | `0xa8ed10, 0x1aa89c0` | L3_SIM | inline | src/serpen.rs:631 |
| `(inline)` | `0x9a1230` | `0x16270, 0x16c10` | L1_MULTI | inline | src/serpen.rs:632 |
| `(inline)` | `0x1a13cb0` | `0x11020a0, 0x1a96470` | L3_WEAK | inline | src/serpen.rs:633 |
| `(inline)` | `0x5418a0` | `0x23c9490, 0x23c8230` | L3_WEAK | inline | src/serpen.rs:634 |
| `ROLE_THR` | `0x1d3602b` | `—` | NOT_IN_TEXT | array[4] | src/tfm2_ai_adjust.rs:967 |
| `(inline)` | `0x1d3602b` | `—` | NOT_IN_TEXT | inline | src/tfm2_ai_adjust.rs:967 |
| `ROLE_THR` | `0x1d36043` | `0x2904520, 0x2997310` | L3_WEAK | array[4] | src/tfm2_ai_adjust.rs:967 |
| `(inline)` | `0x1d36043` | `0x2904520, 0x2997310` | L3_WEAK | inline | src/tfm2_ai_adjust.rs:967 |
| `ROLE_THR` | `0x1d36058` | `0x2904520, 0x2997310` | L3_WEAK | array[4] | src/tfm2_ai_adjust.rs:967 |
| `(inline)` | `0x1d36058` | `0x2904520, 0x2997310` | L3_WEAK | inline | src/tfm2_ai_adjust.rs:967 |
| `ROLE_THR` | `0x1d3605d` | `0x2904520, 0x2997310` | L3_WEAK | array[4] | src/tfm2_ai_adjust.rs:967 |
| `(inline)` | `0x1d3605d` | `0x2904520, 0x2997310` | L3_WEAK | inline | src/tfm2_ai_adjust.rs:967 |
| `OK_DESC_052` | `0x381e1e0` | `—` | NOT_IN_TEXT | array[2] | src/tfm2_ai_adjust.rs:1618 |
| `(inline)` | `0x381e1e0` | `—` | NOT_IN_TEXT | inline | src/tfm2_ai_adjust.rs:1618 |
| `OK_DESC_052` | `0x38d1918` | `—` | NOT_IN_TEXT | array[2] | src/tfm2_ai_adjust.rs:1618 |
| `(inline)` | `0x38d1918` | `—` | NOT_IN_TEXT | inline | src/tfm2_ai_adjust.rs:1618 |
| `LANE_GATE_RVA` | `0x20d9bf9` | `0xd48ec0, 0xec0ff0` | L3_WEAK | const | src/tfm2_ai_adjust.rs:2524 |
| `T3_GATE_A_RVA` | `0x1e9d318` | `0xb5f640, 0x1163230` | L3_WEAK | const | src/tfm2_ai_adjust.rs:2534 |
| `T3_GATE_B_RVA` | `0x1e9d59b` | `0x4364c0, 0x3b42a0` | L3_WEAK | const | src/tfm2_ai_adjust.rs:2535 |
| `CALL_PUSH_A_RVA` | `0x2070ce9` | `0x159caf0, 0x10a52e0` | L3_SIM | const | src/tfm2_ai_adjust.rs:2545 |
| `CALL_PUSH_B_RVA` | `0x2071752` | `0x159caf0, 0x10a52e0` | L3_SIM | const | src/tfm2_ai_adjust.rs:2546 |
| `CALL_JOIN_A_RVA` | `0x2070d01` | `0x159caf0, 0x10a52e0` | L3_SIM | const | src/tfm2_ai_adjust.rs:2547 |
| `CALL_JOIN_B_RVA` | `0x207176c` | `0x159caf0, 0x10a52e0` | L3_SIM | const | src/tfm2_ai_adjust.rs:2548 |
| `D19_SLOT2_EMPTY_RVA` | `0x38d1af0` | `—` | NOT_IN_TEXT | const | src/tfm2_ai_adjust.rs:2728 |
| `D19_STATIC_TEMPLATE_RVA` | `0x38d1af0` | `—` | NOT_IN_TEXT | const | src/tfm2_ai_adjust.rs:2734 |
| `D19_STATIC2_TEMPLATE_RVA` | `0x38d17b8` | `—` | NOT_IN_TEXT | const | src/tfm2_ai_adjust.rs:2745 |
| `D19_TV7_RVA` | `0x3863a28` | `—` | NOT_IN_TEXT | const | src/tfm2_ai_adjust.rs:2751 |
| `(inline)` | `0x83126f` | `0x7a0f80, 0x7a13e0` | L2_MULTI | inline | src/tfm2_ai_adjust.rs:3031 |
| `(inline)` | `0xffffff` | `0x1cc5000, 0x1723920` | L3_WEAK | inline | src/tfm2_ai_adjust.rs:4450 |
| `(inline)` | `0x383cd68` | `—` | NOT_IN_TEXT | inline | src/tfm2_ai_adjust.rs:5494 |
| `(inline)` | `0x38c5d78` | `—` | NOT_IN_TEXT | inline | src/tfm2_ai_adjust.rs:5494 |
| `(inline)` | `0x383d080` | `—` | NOT_IN_TEXT | inline | src/tfm2_ai_adjust.rs:5495 |
| `(inline)` | `0x38c5aa0` | `—` | NOT_IN_TEXT | inline | src/tfm2_ai_adjust.rs:5495 |
| `(inline)` | `0x383d358` | `—` | NOT_IN_TEXT | inline | src/tfm2_ai_adjust.rs:5496 |
| `(inline)` | `0x38c57c8` | `—` | NOT_IN_TEXT | inline | src/tfm2_ai_adjust.rs:5496 |
| `(inline)` | `0x381e1e0` | `—` | NOT_IN_TEXT | inline | src/tfm2_ai_adjust.rs:5702 |
| `(inline)` | `0x35eeff0` | `—` | NOT_IN_TEXT | inline | src/tfm2_ai_adjust.rs:5894 |
| `(inline)` | `0x35ef020` | `—` | NOT_IN_TEXT | inline | src/tfm2_ai_adjust.rs:5894 |
| `(inline)` | `0x381e1e0` | `—` | NOT_IN_TEXT | inline | src/tfm2_ai_adjust.rs:5941 |
| `(inline)` | `0x35e5730` | `—` | NOT_IN_TEXT | inline | src/tfm2_ai_adjust.rs:5967 |
| `(inline)` | `0x1234567` | `—` | L2_HEAD | inline | src/tfm2_ai_adjust.rs:7415 |
| `(inline)` | `0x1234567` | `—` | L2_HEAD | inline | src/tfm2_ai_adjust.rs:7452 |
| `(inline)` | `0x1fcfda0` | `0x10afd90, 0x1288b40` | L3_WEAK | inline | src/tfm2_ai_adjust.rs:7575 |
| `(inline)` | `0x1b6ec93` | `0x2904520, 0x2997310` | L3_WEAK | inline | src/tfm2_ai_adjust.rs:7580 |
| `(inline)` | `0x1b6e806` | `0x109ae40, 0x121fc60` | L3_WEAK | inline | src/tfm2_ai_adjust.rs:7585 |
| `(inline)` | `0x1feca43` | `0x73e780, 0x7a0f00` | L1_MULTI | inline | src/tfm2_ai_adjust.rs:7590 |
| `(inline)` | `0x2080e20` | `0x109ae40, 0x121fc60` | L3_WEAK | inline | src/tfm2_ai_adjust.rs:7597 |
| `(inline)` | `0x20def90` | `0xc8db70, 0xca22c0` | L1_MULTI | inline | src/tfm2_ai_adjust.rs:7603 |
| `(inline)` | `0x22dafea` | `0x5aa40, 0x96f70` | L1_MULTI | inline | src/tfm2_ai_adjust.rs:7610 |
| `(inline)` | `0x1c383f0` | `0x1ec54d0, 0xa0cf30` | L3_WEAK | inline | src/tfm2_ai_adjust.rs:7617 |
| `(inline)` | `0x22df630` | `0xcc9d70, 0xdaf780` | L3_SIM | inline | src/tfm2_ai_adjust.rs:7623 |
| `(inline)` | `0x1b78420` | `0xc98b40, 0x16d02b0` | L2_MULTI | inline | src/tfm2_ai_adjust.rs:7630 |
| `(inline)` | `0x1c7ca20` | `0x1467ea0, 0x230460` | L3_WEAK | inline | src/tfm2_ai_adjust.rs:7638 |
| `(inline)` | `0x2380820` | `0xdece30, 0xd94d00` | L3_WEAK | inline | src/tfm2_ai_adjust.rs:7642 |
| `LOADER_RVA` | `0x5ac950` | `0x91ab0` | L2_HEAD | const | src/ui_inject_embed.rs:24 |
| `PARSER_RVA` | `0x24b5a00` | `0x28a47d0, 0x1a9050` | L3_WEAK | const | src/ui_inject_embed.rs:25 |
| `ALLOC_RVA` | `0x25c4d30` | `0x2c26e30, 0x2b2f140` | NONE | const | src/ui_inject_embed.rs:26 |

## tfm2_banpick_illust — 8/30 해결 · NOT_IN_TEXT 9 / L3_SIM 8 / L3_WEAK 7 / L2_HEAD 4 / L1_EXACT 2

| 상수 | 0.5.2 | → 0.5.3 | 등급 | 종류 | 위치 |
|---|---|---|---|---|---|
| `RVA_FX_SET` | `0x11e2370` | `0x1bd8e50, 0x228bdd0` | L3_WEAK | const | src/showcase.rs:19 |
| `RVA_CARD_DRAW` | `0x11f9030` | `0x1bee8e0` | L2_HEAD | const | src/showcase.rs:20 |
| `RVA_ILLUST_GET` | `0xfdabe0` | `0x1e91400` | L3_SIM | const | src/showcase.rs:21 |
| `RVA_SUBMIT` | `0x248b1c0` | `0x1859f0, 0x185f40` | L3_WEAK | const | src/showcase.rs:22 |
| `RVA_SUBMIT_TEXT` | `0x248b400` | `0x1859f0, 0x185f40` | L3_WEAK | const | src/showcase.rs:23 |
| `RVA_IMG_BUILD` | `0x248c130` | `0x2a2c9b0, 0x2d85250` | L3_WEAK | const | src/showcase.rs:24 |
| `RVA_IMG_UV` | `0x248c7c0` | `0x186f70` | L1_EXACT | const | src/showcase.rs:25 |
| `RVA_IMG_FLAG` | `0x248cd40` | `0x187420` | L1_EXACT | const | src/showcase.rs:26 |
| `RVA_IMG_COLOR` | `0xff0c20` | `0x1875b0` | L3_SIM | const | src/showcase.rs:27 |
| `RVA_IMG_SHADER` | `0x248e850` | `0xeae7d0, 0xeb32d0` | L3_WEAK | const | src/showcase.rs:28 |
| `RVA_TEXT_BUILD` | `0x248c1e0` | `0x1165380` | L2_HEAD | const | src/showcase.rs:29 |
| `RVA_NAME_GET` | `0x1217630` | `0x2bc46a0, 0x12550f0` | L3_WEAK | const | src/showcase.rs:30 |
| `RVA_ASSET_GET` | `0x99c860` | `0x91ab0` | L2_HEAD | const | src/showcase.rs:31 |
| `RVA_ANIM_GET` | `0x5ab7d0` | `0x91ab0` | L2_HEAD | const | src/showcase.rs:32 |
| `RVA_SPRITE_CALC` | `0x121aca0` | `0x1c1e4e0, 0x1b57090` | L3_WEAK | const | src/showcase.rs:33 |
| `RVA_GAME_ALLOC` | `0x8b7f80` | `—` | NOT_IN_TEXT | const | src/showcase.rs:34 |
| `RVA_GAME_FREE` | `0x8b7f90` | `—` | NOT_IN_TEXT | const | src/showcase.rs:35 |
| `RVA_C_CARD_RECT` | `0x3731380` | `—` | NOT_IN_TEXT | const | src/showcase.rs:57 |
| `RVA_C_SNAP_RECT` | `0x37313b0` | `—` | NOT_IN_TEXT | const | src/showcase.rs:58 |
| `RVA_C_LINE_DIR` | `0x37313e0` | `—` | NOT_IN_TEXT | const | src/showcase.rs:59 |
| `RVA_C_LINE_START` | `0x37313f0` | `—` | NOT_IN_TEXT | const | src/showcase.rs:60 |
| `RVA_C_LINE_ANCHOR` | `0x3731400` | `—` | NOT_IN_TEXT | const | src/showcase.rs:61 |
| `RVA_C_NORMAL` | `0x37313c0` | `—` | NOT_IN_TEXT | const | src/showcase.rs:62 |
| `RVA_I_SNAP_H` | `0x124e2ba` | `0x1c52950, 0x184f120` | L3_SIM | const | src/showcase.rs:63 |
| `RVA_D_SNAP_W` | `0x124e2c2` | `0x1c52950, 0x184f120` | L3_SIM | const | src/showcase.rs:64 |
| `RVA_D_CUT_LO` | `0x1201e19` | `0x1bf89a0, 0x1f0800` | L3_SIM | const | src/showcase.rs:65 |
| `RVA_D_CUT_HI` | `0x1201e27` | `0x1bf89a0, 0x1f0800` | L3_SIM | const | src/showcase.rs:66 |
| `RVA_D_ZIG_X1` | `0x124e8cf` | `0x1c52950, 0x184f120` | L3_SIM | const | src/showcase.rs:67 |
| `RVA_D_ZIG_X2` | `0x124efa1` | `0x1c52950, 0x184f120` | L3_SIM | const | src/showcase.rs:68 |
| `RVA_SLOTS` | `0x3fd2b00` | `—` | NOT_IN_TEXT | const | src/showcase.rs:69 |

## tfm2_banpick_order — 4/27 해결 · L3_SIM 9 / L3_WEAK 8 / NOT_IN_TEXT 5 / L2_HEAD 4 / L1_EXACT 1

| 상수 | 0.5.2 | → 0.5.3 | 등급 | 종류 | 위치 |
|---|---|---|---|---|---|
| `PANIC_SITES` | `0x11da680` | `0x1bcf010, 0x23a6ec0` | L3_SIM | array[6] | src/diag.rs:64 |
| `PANIC_SITES` | `0x11da6a0` | `0x1bcf010, 0x23a6ec0` | L3_SIM | array[6] | src/diag.rs:64 |
| `PANIC_SITES` | `0x11db418` | `0x1bcf010, 0x23a6ec0` | L3_SIM | array[6] | src/diag.rs:64 |
| `PANIC_SITES` | `0x11db438` | `0x1bcf010, 0x23a6ec0` | L3_SIM | array[6] | src/diag.rs:64 |
| `PANIC_SITES` | `0x11dbed9` | `0x1bcf010, 0x23a6ec0` | L3_SIM | array[6] | src/diag.rs:64 |
| `PANIC_SITES` | `0x11dc023` | `0x1bcf010, 0x23a6ec0` | L3_SIM | array[6] | src/diag.rs:64 |
| `RVA_PANIC_HOOK` | `0x25d4764` | `0x28f2f34` | L2_HEAD | const | src/diag.rs:636 |
| `RVA_PHASE_INFO` | `0x1cd9380` | `—` | NOT_IN_TEXT | const | src/hooks.rs:23 |
| `RVA_PHASE_SCALAR` | `0x1d04120` | `—` | NOT_IN_TEXT | const | src/hooks.rs:24 |
| `RVA_APPLIER` | `0x11e2140` | `0x1bd8c20` | L1_EXACT | const | src/hooks.rs:25 |
| `RVA_APP_PICK_T1` | `0x11ce240` | `0x1bc4980, 0x1bc47f0` | L3_WEAK | const | src/hooks.rs:275 |
| `RVA_APP_PICK_T2` | `0x11ce400` | `0x1bc4980, 0x1bc47f0` | L3_WEAK | const | src/hooks.rs:276 |
| `RVA_APP_BAN_T1` | `0x120c020` | `0x1bc4980, 0x1bc47f0` | L3_WEAK | const | src/hooks.rs:277 |
| `RVA_APP_BAN_T2` | `0x120c1d0` | `0x1bc4980, 0x1bc47f0` | L3_WEAK | const | src/hooks.rs:278 |
| `RVA_TRANSITION` | `0x11d8ef0` | `0x1bcf010` | L3_SIM | const | src/hooks.rs:279 |
| `RVA_AI_SITE1` | `0x1c04389` | `0x10a0320, 0x10a1430` | L3_SIM | const | src/hooks.rs:299 |
| `RVA_AI_JOIN1` | `0x1c04475` | `0x10a0320, 0x10a1430` | L3_SIM | const | src/hooks.rs:300 |
| `RVA_AI_SITE2` | `0x1c07938` | `0x10a3c40, 0x285ecb0` | L3_WEAK | const | src/hooks.rs:301 |
| `RVA_AI_JOIN2` | `0x1c07a09` | `0x10a3c40, 0x285ecb0` | L3_WEAK | const | src/hooks.rs:302 |
| `RVA_SFX_SITE` | `0x1251303` | `0x1c55300, 0xb8b2f0` | L2_HEAD | const | src/hooks.rs:315 |
| `RVA_SFX_END` | `0x1251352` | `0x1c55300, 0xb8b2f0` | L2_HEAD | const | src/hooks.rs:316 |
| `RVA_STR_BAN` | `0x373d596` | `—` | NOT_IN_TEXT | const | src/hooks.rs:317 |
| `RVA_STR_PICK` | `0x373d5b2` | `—` | NOT_IN_TEXT | const | src/hooks.rs:318 |
| `RVA_BANNER` | `0x11df9f0` | `0x1bd63a0, 0x2b0c000` | L3_WEAK | const | src/hooks.rs:357 |
| `RVA_LINEUP` | `0x11cedb0` | `0x1bc52b0, 0x243b9f0` | L3_WEAK | const | src/hooks.rs:384 |
| `RVA_COMMIT` | `0x1d075d0` | `0x167fdd0` | L2_HEAD | const | src/hooks.rs:402 |
| `RVA_TURN` | `0x1d07cf0` | `—` | NOT_IN_TEXT | const | src/hooks.rs:416 |

## tfm2_comptest_unlock — 8/47 해결 · L3_SIM 17 / L3_WEAK 14 / L2_HEAD 6 / NOT_IN_TEXT 4 / L1_MULTI 2 / L1_EXACT 2 / L2_MULTI 1 / NONE 1

| 상수 | 0.5.2 | → 0.5.3 | 등급 | 종류 | 위치 |
|---|---|---|---|---|---|
| `no_stamina_cost` | `0xe93b2d` | `0x17e0240, 0xa5c1e0` | L3_SIM | patch_site | src/tfm2_comptest_unlock.rs:60 |
| `daily_remaining` | `0x1f14090` | `—` | NOT_IN_TEXT | patch_site | src/tfm2_comptest_unlock.rs:69 |
| `daily_inc_gate` | `0xe8cb20` | `0x17e0240, 0xa5c1e0` | L3_SIM | patch_site | src/tfm2_comptest_unlock.rs:78 |
| `server_dedup_real` | `0xec7758` | `0x1830900, 0x196f1b0` | L3_SIM | patch_site | src/tfm2_comptest_unlock.rs:101 |
| `allow_dup_players` | `0xd00ee5` | `0x238eb60, 0x2a59d00` | L3_WEAK | patch_site | src/tfm2_comptest_unlock.rs:106 |
| `server_dedup` | `0xe8b5fa` | `0x17e0240, 0xa5c1e0` | L3_SIM | patch_site | src/tfm2_comptest_unlock.rs:122 |
| `btn5v5_roster_min_a` | `0xd967cf` | `—` | L2_HEAD | patch_site | src/tfm2_comptest_unlock.rs:138 |
| `btn5v5_roster_min_b` | `0xcf7b68` | `0x1c133f0, 0x2ab1410` | L3_WEAK | patch_site | src/tfm2_comptest_unlock.rs:141 |
| `btn5v5_warn_text` | `0xd9662c` | `—` | L2_HEAD | patch_site | src/tfm2_comptest_unlock.rs:144 |
| `server_roster_min` | `0xec768e` | `0x1830900, 0x196f1b0` | L3_SIM | patch_site | src/tfm2_comptest_unlock.rs:170 |
| `roster_count_gate` | `0xd0a74c` | `0x18f1180, 0x1890fd0` | L3_SIM | patch_site | src/tfm2_comptest_unlock.rs:182 |
| `collected_gate` | `0xd0a740` | `0x18f1180, 0x1890fd0` | L3_SIM | patch_site | src/tfm2_comptest_unlock.rs:187 |
| `collect_err_gate` | `0xd0a728` | `0x18f1180, 0x1890fd0` | L3_SIM | patch_site | src/tfm2_comptest_unlock.rs:191 |
| `run_push_gate` | `0xd0adf1` | `0x18f1180, 0x1890fd0` | L3_SIM | patch_site | src/tfm2_comptest_unlock.rs:197 |
| `DISP_RVA` | `0xd3f780` | `0x2cb0290, 0x2ca8d80` | L3_WEAK | const | src/tfm2_comptest_unlock.rs:292 |
| `INSERT_RVA` | `0xcabac0` | `0x9bc50, 0x9bd20` | L1_MULTI | const | src/tfm2_comptest_unlock.rs:355 |
| `CT_REGION_LO` | `0xe7ccd0` | `0x17e0240` | L3_SIM | const | src/tfm2_comptest_unlock.rs:364 |
| `CT_REGION_HI` | `0xea2345` | `—` | NOT_IN_TEXT | const | src/tfm2_comptest_unlock.rs:365 |
| `CT_CLIENT_LO` | `0xcf0000` | `0x22ea820, 0x18cf8a0` | L3_WEAK | const | src/tfm2_comptest_unlock.rs:366 |
| `CT_CLIENT_HI` | `0xda0000` | `0x9c1830, 0x9c16b0` | L3_WEAK | const | src/tfm2_comptest_unlock.rs:367 |
| `ATH_ID_HI` | `0x100000` | `0x3f4760, 0x53e0e0` | L2_HEAD | const | src/tfm2_comptest_unlock.rs:375 |
| `(inline)` | `0xd00ed0` | `0x238eb60, 0x2a59d00` | L3_WEAK | inline | src/tfm2_comptest_unlock.rs:384 |
| `ENQ_RVA` | `0xcb9c80` | `0x1b8a180, 0x12feb70` | L3_WEAK | const | src/tfm2_comptest_unlock.rs:469 |
| `RUN_RVA` | `0xd0a440` | `0x18f1180` | L3_SIM | const | src/tfm2_comptest_unlock.rs:526 |
| `SRV_RVA` | `0x13d4af0` | `0x240b8f0` | L1_EXACT | const | src/tfm2_comptest_unlock.rs:529 |
| `LOADING_RVA` | `0xd186f0` | `0x80e9d0, 0x2219ea0` | L3_WEAK | const | src/tfm2_comptest_unlock.rs:604 |
| `DEDUP_INS_RVA` | `0xca75f0` | `0x28c590, 0x1a13a80` | L2_MULTI | const | src/tfm2_comptest_unlock.rs:628 |
| `SPAWN_CP_RVA` | `0x13c71b0` | `0x23fd0f0, 0xf0840` | L3_WEAK | const | src/tfm2_comptest_unlock.rs:629 |
| `PUSH_RVA` | `0x101cc08` | `0x1d072e0, 0x1f74680` | L3_SIM | const | src/tfm2_comptest_unlock.rs:688 |
| `FN_DD_SETOPT_RVA` | `0x242f250` | `0x12550f0, 0x1ed4380` | L3_WEAK | const | src/tfm2_comptest_unlock.rs:830 |
| `ITEMCONV_RVA` | `0xed8770` | `0x18429d0` | L1_EXACT | const | src/tfm2_comptest_unlock.rs:970 |
| `(inline)` | `0xf794c0` | `0x2564e50, 0x2474240` | L3_WEAK | inline | src/tfm2_comptest_unlock.rs:1039 |
| `COLLECT_RVA` | `0xd0bd80` | `0xcde820` | L3_SIM | const | src/tfm2_comptest_unlock.rs:1042 |
| `EF1EA0_RVA` | `0xe58c30` | `0x1927fa0, 0x2847b30` | L3_WEAK | const | src/tfm2_comptest_unlock.rs:1125 |
| `ATH_GET_SC_RVA` | `0xe3b200` | `0x7a07f0, 0x1794280` | L1_MULTI | const | src/tfm2_comptest_unlock.rs:1133 |
| `ORACLE_RVA` | `0x1d94720` | `0xeb6590` | L3_SIM | const | src/tfm2_comptest_unlock.rs:1192 |
| `(inline)` | `0x20566c0` | `0x1ec54d0, 0xa0cf30` | L3_WEAK | inline | src/tfm2_comptest_unlock.rs:1356 |
| `SLOT_RVA` | `0xd1acf0` | `0x1904640` | L2_HEAD | const | src/tfm2_comptest_unlock.rs:1383 |
| `RUST_ALLOC_RVA` | `0x8b7f80` | `—` | NOT_IN_TEXT | const | src/tfm2_comptest_unlock.rs:1393 |
| `RUST_DEALLOC_RVA` | `0x8b7f90` | `—` | NOT_IN_TEXT | const | src/tfm2_comptest_unlock.rs:1394 |
| `ATH_GET_RVA` | `0x402840` | `0xb89b20, 0x240e050` | L3_SIM | const | src/tfm2_comptest_unlock.rs:1507 |
| `ATH_GET_JE_TARGET_RVA` | `0x4028fb` | `0xb89b20, 0x240e050` | L3_SIM | const | src/tfm2_comptest_unlock.rs:1508 |
| `CT_ARM_LO` | `0x13e1c00` | `0x2417ea0, 0x22f8240` | L3_SIM | const | src/tfm2_comptest_unlock.rs:1510 |
| `CT_ARM_HI` | `0x13ea200` | `—` | L2_HEAD | const | src/tfm2_comptest_unlock.rs:1511 |
| `LOADER_RVA` | `0x5ac950` | `0x91ab0` | L2_HEAD | const | src/ui_inject.rs:32 |
| `PARSER_RVA` | `0x24b5a00` | `0x28a47d0, 0x1a9050` | L3_WEAK | const | src/ui_inject.rs:33 |
| `ALLOC_RVA` | `0x25c4d30` | `0x2c26e30, 0x2b2f140` | NONE | const | src/ui_inject.rs:34 |

## tfm2_draft_overlay — 0/5 해결 · L3_WEAK 3 / L1_MULTI 2

| 상수 | 0.5.2 | → 0.5.3 | 등급 | 종류 | 위치 |
|---|---|---|---|---|---|
| `ANIM_GET_RVA` | `0x40e250` | `0x1f03860, 0xb2c210` | L3_WEAK | const | src/lib.rs:142 |
| `LOADER_RVA` | `0x40f3d0` | `0x1f03860, 0xb2c210` | L3_WEAK | const | src/lib.rs:359 |
| `BANPICK_LOADER_RVA` | `0xeb17d0` | `0x1819df0, 0x181a350` | L1_MULTI | const | src/lib.rs:365 |
| `PARSER_RVA` | `0x24b4590` | `0x1a4f00, 0xfadeb0` | L1_MULTI | const | src/lib.rs:366 |
| `ALLOC_RVA` | `0x25c5a40` | `0x2ac9930, 0x2bcfc30` | L3_WEAK | const | src/lib.rs:367 |

## tfm2_elemental_serpen — 6/19 해결 · L3_WEAK 9 / L2_HEAD 6 / L3_SIM 2 / L1_EXACT 1 / NONE 1

| 상수 | 0.5.2 | → 0.5.3 | 등급 | 종류 | 위치 |
|---|---|---|---|---|---|
| `SERPEN_RVA` | `0x21f8ca0` | `0x1535810` | L2_HEAD | const | src/lib.rs:34 |
| `MOBATICK_RVA` | `0x230c290` | `0xeeeac0, 0x2328370` | L3_WEAK | const | src/lib.rs:350 |
| `SPAWN_HOOKS` | `0x539f40` | `0xabdf60, 0xabd340` | L3_WEAK | array[2] | src/lib.rs:405 |
| `(inline)` | `0x539f40` | `0xabdf60, 0xabd340` | L3_WEAK | inline | src/lib.rs:405 |
| `SPAWN_HOOKS` | `0x53aae0` | `0xabdf60, 0xabd340` | L3_WEAK | array[2] | src/lib.rs:405 |
| `(inline)` | `0x53aae0` | `0xabdf60, 0xabd340` | L3_WEAK | inline | src/lib.rs:405 |
| `LAUNCHER_RVA` | `0x1d96870` | `0x2413d10, 0x9d7180` | L3_WEAK | const | src/lib.rs:414 |
| `LAUNCHER_RET_A` | `0x759c36` | `—` | L2_HEAD | const | src/lib.rs:420 |
| `LAUNCHER_RET_B` | `0x75e5cf` | `—` | L2_HEAD | const | src/lib.rs:421 |
| `LAUNCHER_RET_C` | `0x1555215` | `—` | L2_HEAD | const | src/lib.rs:425 |
| `UILOADER_RVA` | `0x5ac950` | `0x91ab0` | L2_HEAD | const | src/lib.rs:513 |
| `UIPARSER_RVA` | `0x24b5a00` | `0x28a47d0, 0x1a9050` | L3_WEAK | const | src/lib.rs:514 |
| `UIALLOC_RVA` | `0x25c4d30` | `0x2c26e30, 0x2b2f140` | NONE | const | src/lib.rs:515 |
| `RENDER_STEP_RVA` | `0x811500` | `0x960df0` | L3_SIM | const | src/lib.rs:717 |
| `RUNNER_CTOR_RVA` | `0x1d981e0` | `0x2413d10, 0x1925ab0` | L3_WEAK | const | src/lib.rs:744 |
| `DMGA_RVA` | `0x22164a0` | `0xfdbbb0` | L1_EXACT | const | src/lib.rs:1707 |
| `DMGB_RVA` | `0x22d2b20` | `0x12c3bb0` | L3_SIM | const | src/lib.rs:1710 |
| `KEYRES_RVA` | `0xc2f990` | `0x1b0aba0` | L2_HEAD | const | src/lib.rs:1902 |
| `ARG_STR_RVA` | `0xfef190` | `0x1a2ed40, 0x1228a90` | L3_WEAK | const | src/lib.rs:2427 |

## tfm2_item_tactics — 5/31 해결 · L3_WEAK 11 / L2_HEAD 9 / L3_SIM 6 / NONE 3 / L1_MULTI 1 / L1_EXACT 1

| 상수 | 0.5.2 | → 0.5.3 | 등급 | 종류 | 위치 |
|---|---|---|---|---|---|
| `FN_DD_SETOPT_RVA` | `0x242f250` | `0x12550f0, 0x1ed4380` | L3_WEAK | const | src/lib.rs:32 |
| `SETTER_NOP_RVA` | `0xda42ee` | `0x22c17b0, 0x2495560` | L3_WEAK | const | src/lib.rs:1179 |
| `RVA_REALLOC` | `0x25c4dd0` | `0x28e3b10` | L1_EXACT | const | src/lib.rs:1772 |
| `CL_LAUNCHER_RVA` | `0x1d96870` | `0x2413d10, 0x9d7180` | L3_WEAK | const | src/lib.rs:1813 |
| `(inline)` | `0xd40a63` | `0x1925ab0, 0x18f6c30` | L3_WEAK | inline | src/lib.rs:1848 |
| `(inline)` | `0x759c36` | `—` | L2_HEAD | inline | src/lib.rs:1849 |
| `(inline)` | `0x75e5cf` | `—` | L2_HEAD | inline | src/lib.rs:1849 |
| `SEEDCTOR_RVA` | `0x22c1da0` | `0x12b9ab0, 0x1e73bc0` | L3_WEAK | const | src/lib.rs:1928 |
| `SPAWN_RVA` | `0x1d9e0e0` | `0xebfe50, 0x20f3220` | L3_WEAK | const | src/lib.rs:1976 |
| `SIM_RVA` | `0x223d1b0` | `0x302f0a0, 0x3027660` | L3_WEAK | const | src/lib.rs:2102 |
| `VIEW_RVA` | `0x20ae1ac` | `0x16d34d0, 0x16d2590` | L3_WEAK | const | src/lib.rs:2143 |
| `(inline)` | `0x722ca0` | `0xb03ee0, 0x122ac20` | L1_MULTI | inline | src/lib.rs:2349 |
| `(inline)` | `0x740000` | `0x983040, 0x211e850` | L3_WEAK | inline | src/lib.rs:2349 |
| `(inline)` | `0x2060280` | `0x2e48ea0, 0x2825ec0` | L3_WEAK | inline | src/lib.rs:2377 |
| `RVA_BUY_ITEM` | `0x211e070` | `0xd0c680` | L3_SIM | const | src/lib.rs:2658 |
| `ITEMNET_FORWARD_RVA` | `0x1b9cce0` | `0x10587e0` | L3_SIM | const | src/lib.rs:2706 |
| `(inline)` | `0x2341440` | `0xf21fe0, 0xf19670` | L3_SIM | inline | src/lib.rs:3881 |
| `(inline)` | `0x2341447` | `0xf21fe0, 0xf19670` | L3_SIM | inline | src/lib.rs:3882 |
| `(inline)` | `0x211e428` | `0xd0c770, 0xd05af0` | L3_SIM | inline | src/lib.rs:3904 |
| `(inline)` | `0x211e42e` | `0xd0c770, 0xd05af0` | L3_SIM | inline | src/lib.rs:3905 |
| `CAND_GATE_RVA` | `0x1a3b280` | `—` | L2_HEAD | const | src/lib.rs:3953 |
| `RVA_SLOT_HELPER` | `0xc5cd80` | `0x25c4030, 0x2c5f8f0` | NONE | const | src/lib.rs:3975 |
| `(inline)` | `0x4e46c0` | `—` | L2_HEAD | inline | src/lib.rs:3986 |
| `(inline)` | `0x4e4a30` | `—` | L2_HEAD | inline | src/lib.rs:3987 |
| `(inline)` | `0x4e5110` | `—` | L2_HEAD | inline | src/lib.rs:3988 |
| `(inline)` | `0x4e5480` | `—` | L2_HEAD | inline | src/lib.rs:3989 |
| `LOADER_RVA` | `0x5ac950` | `0x91ab0` | L2_HEAD | const | src/ui_inject.rs:20 |
| `STRAT_LOADER_RVA` | `0x5ac950` | `0x91ab0` | L2_HEAD | const | src/ui_inject.rs:21 |
| `PARSER_RVA` | `0x24b5a00` | `0x28a47d0, 0x1a9050` | L3_WEAK | const | src/ui_inject.rs:22 |
| `ALLOC_RVA` | `0x25c4d30` | `0x2c26e30, 0x2b2f140` | NONE | const | src/ui_inject.rs:23 |
| `DEALLOC_RVA` | `0x25c4d90` | `0x1000, 0x2b0baf0` | NONE | const | src/ui_inject.rs:24 |

## tfm2_level_cap — 0/2 해결 · L3_SIM 2

| 상수 | 0.5.2 | → 0.5.3 | 등급 | 종류 | 위치 |
|---|---|---|---|---|---|
| `RVA_LEN_LOAD` | `0x22d3fea` | `0x12c56d0, 0x149e010` | L3_SIM | const | src/lib.rs:82 |
| `RVA_UI_CMP` | `0x80ae73` | `0x952170, 0x20b70a0` | L3_SIM | const | src/lib.rs:88 |

## tfm2_transfer_tweak — 6/7 해결 · L1_EXACT 6 / NOT_IN_TEXT 1

| 상수 | 0.5.2 | → 0.5.3 | 등급 | 종류 | 위치 |
|---|---|---|---|---|---|
| `RVA_GATE` | `0x1d15e90` | `0x14a6f50` | L1_EXACT | const | src/lib.rs:43 |
| `RVA_TBL` | `0x3835560` | `—` | NOT_IN_TEXT | const | src/lib.rs:44 |
| `thr_1_20` | `0x1d1626b` | `0x14a732b` | L1_EXACT | patch_site | src/lib.rs:52 |
| `thr_1_45` | `0x1d162db` | `0x14a739b` | L1_EXACT | patch_site | src/lib.rs:53 |
| `thr_1_35` | `0x1d162e9` | `0x14a73a9` | L1_EXACT | patch_site | src/lib.rs:54 |
| `pen_0_25` | `0x1d16340` | `0x14a7400` | L1_EXACT | patch_site | src/lib.rs:55 |
| `gate_0_30` | `0x1d162ab` | `0x14a736b` | L1_EXACT | patch_site | src/lib.rs:56 |
