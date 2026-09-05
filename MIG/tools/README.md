> ⚠ **STALE — 정본은 `MIG\aifill.py`** (2026-09-05)

이 폴더의 `fill_decomp.py` / `verify_fill.py` 는 0.5.8 디컴 배치 B 작업 중 임시로 만든 것이다.
같은 일(스캐폴딩 본문 자동 채움)을 하는 **정본은 상위 폴더의 `aifill.py`** 이고,
배치 A·C 가 쓴 capstone 폴백·jumptable 보강까지 전부 흡수했다.

```
python MIG\aifill.py MIG\decomp\<버전> --port 8081
```

`aifill.py` 가 추가로 갖는 것(여기 스크립트엔 없다):
- **서버↔exe 프롤로그 3명령 대조** — 포트를 잘못 잡아 **구버전을 디컴해 놓고 성공으로 보고**하는
  조용한 오염을 시작 시 막는다(실측으로 `mov mov cmp` vs `push push push` 불일치를 잡아 중단).
- **capstone 대체**(Ghidra 함수 미정의) + **jumptable 아암 복구**를 한 번에.
- **멱등** — 다시 돌리면 이전 보강을 걷어내고 재생성한다.

로그(`fill_b1.log`·`fill_b2.log`)는 배치 B 이력이라 남겨 둔다.
